#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_99(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let assign31250_e41363: f64 = (p.p205 * 1.9e-9);
        let assign31250_e41365: f64 = (assign31250_e41363 / locals.var_t1);
        locals.var_xdcinv = assign31250_e41365;
        locals.var_xdcinv_dn0 = (-((assign31250_e41363 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn2 = (-((assign31250_e41363 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn3 = (-((assign31250_e41363 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn4 = (-((assign31250_e41363 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn5 = (-((assign31250_e41363 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn6 = (-((assign31250_e41363 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn7 = (-((assign31250_e41363 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn8 = (-((assign31250_e41363 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn9 = (-((assign31250_e41363 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn10 = (-((assign31250_e41363 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn11 = (-((assign31250_e41363 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn12 = (-((assign31250_e41363 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn13 = (-((assign31250_e41363 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_dn14 = (-((assign31250_e41363 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1)));
        locals.var_xdcinv_rv = 0.0;

        let assign31260_e41368: f64 = (3.9 * 8.85418e-12);
        let assign31260_e41371: f64 = (locals.var_bsimbulktoxp * 3.9);
        let assign31260_e41373: f64 = (assign31260_e41371 / p.p111);
        let assign31260_e41376: f64 = (locals.var_xdcinv / locals.var_epsratio);
        let assign31260_e41377: f64 = (assign31260_e41373 + assign31260_e41376);
        let assign31260_e41378: f64 = (assign31260_e41368 / assign31260_e41377);
        locals.var_coxeffinv = assign31260_e41378;
        locals.var_coxeffinv_dn0 = (-((assign31260_e41368 * (locals.var_xdcinv_dn0 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn2 = (-((assign31260_e41368 * (locals.var_xdcinv_dn2 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn3 = (-((assign31260_e41368 * (locals.var_xdcinv_dn3 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn4 = (-((assign31260_e41368 * (locals.var_xdcinv_dn4 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn5 = (-((assign31260_e41368 * (locals.var_xdcinv_dn5 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn6 = (-((assign31260_e41368 * (locals.var_xdcinv_dn6 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn7 = (-((assign31260_e41368 * (locals.var_xdcinv_dn7 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn8 = (-((assign31260_e41368 * (locals.var_xdcinv_dn8 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn9 = (-((assign31260_e41368 * (locals.var_xdcinv_dn9 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn10 = (-((assign31260_e41368 * (locals.var_xdcinv_dn10 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn11 = (-((assign31260_e41368 * (locals.var_xdcinv_dn11 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn12 = (-((assign31260_e41368 * (locals.var_xdcinv_dn12 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn13 = (-((assign31260_e41368 * (locals.var_xdcinv_dn13 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_dn14 = (-((assign31260_e41368 * (locals.var_xdcinv_dn14 / locals.var_epsratio)) / (assign31260_e41377 * assign31260_e41377)));
        locals.var_coxeffinv_rv = 0.0;

        let assign31270_e41380: f64 = (-p.p2);
        let assign31270_e41382: f64 = (assign31270_e41380 * locals.var_wact);
        let assign31270_e41384: f64 = (assign31270_e41382 * locals.var_lact);
        let assign31270_e41387: f64 = (8.85418e-12 * p.p111);
        let assign31270_e41389: f64 = (assign31270_e41387 / locals.var_bsimbulktoxp);
        let assign31270_e41390: f64 = (assign31270_e41384 * assign31270_e41389);
        let assign31270_e41392: f64 = (assign31270_e41390 * locals.var_vt);
        let assign31270_e41394: f64 = (assign31270_e41392 * locals.var_qb_1);
        locals.var_qbi = assign31270_e41394;
        locals.var_qbi_dn0 = (assign31270_e41392 * locals.var_qb_1_dn0);
        locals.var_qbi_dn2 = (assign31270_e41392 * locals.var_qb_1_dn2);
        locals.var_qbi_dn3 = (assign31270_e41392 * locals.var_qb_1_dn3);
        locals.var_qbi_dn4 = (((assign31270_e41390 * locals.var_vt_dn4) * locals.var_qb_1) + (assign31270_e41392 * locals.var_qb_1_dn4));
        locals.var_qbi_dn5 = (assign31270_e41392 * locals.var_qb_1_dn5);
        locals.var_qbi_dn6 = (assign31270_e41392 * locals.var_qb_1_dn6);
        locals.var_qbi_dn7 = (assign31270_e41392 * locals.var_qb_1_dn7);
        locals.var_qbi_dn8 = (assign31270_e41392 * locals.var_qb_1_dn8);
        locals.var_qbi_dn9 = (assign31270_e41392 * locals.var_qb_1_dn9);
        locals.var_qbi_dn10 = (assign31270_e41392 * locals.var_qb_1_dn10);
        locals.var_qbi_dn11 = (assign31270_e41392 * locals.var_qb_1_dn11);
        locals.var_qbi_dn12 = (assign31270_e41392 * locals.var_qb_1_dn12);
        locals.var_qbi_dn13 = (assign31270_e41392 * locals.var_qb_1_dn13);
        locals.var_qbi_dn14 = (assign31270_e41392 * locals.var_qb_1_dn14);
        locals.var_qbi_rv = 0.0;

        let assign31280_e41397: f64 = (p.p2 * locals.var_wact);
        let assign31280_e41399: f64 = (assign31280_e41397 * locals.var_lact);
        let assign31280_e41401: f64 = (assign31280_e41399 * locals.var_coxeffinv);
        let assign31280_e41403: f64 = (assign31280_e41401 * locals.var_vt);
        locals.var_wlcoxvtinv = assign31280_e41403;
        locals.var_wlcoxvtinv_dn0 = ((assign31280_e41399 * locals.var_coxeffinv_dn0) * locals.var_vt);
        locals.var_wlcoxvtinv_dn2 = ((assign31280_e41399 * locals.var_coxeffinv_dn2) * locals.var_vt);
        locals.var_wlcoxvtinv_dn3 = ((assign31280_e41399 * locals.var_coxeffinv_dn3) * locals.var_vt);
        locals.var_wlcoxvtinv_dn4 = (((assign31280_e41399 * locals.var_coxeffinv_dn4) * locals.var_vt) + (assign31280_e41401 * locals.var_vt_dn4));
        locals.var_wlcoxvtinv_dn5 = ((assign31280_e41399 * locals.var_coxeffinv_dn5) * locals.var_vt);
        locals.var_wlcoxvtinv_dn6 = ((assign31280_e41399 * locals.var_coxeffinv_dn6) * locals.var_vt);
        locals.var_wlcoxvtinv_dn7 = ((assign31280_e41399 * locals.var_coxeffinv_dn7) * locals.var_vt);
        locals.var_wlcoxvtinv_dn8 = ((assign31280_e41399 * locals.var_coxeffinv_dn8) * locals.var_vt);
        locals.var_wlcoxvtinv_dn9 = ((assign31280_e41399 * locals.var_coxeffinv_dn9) * locals.var_vt);
        locals.var_wlcoxvtinv_dn10 = ((assign31280_e41399 * locals.var_coxeffinv_dn10) * locals.var_vt);
        locals.var_wlcoxvtinv_dn11 = ((assign31280_e41399 * locals.var_coxeffinv_dn11) * locals.var_vt);
        locals.var_wlcoxvtinv_dn12 = ((assign31280_e41399 * locals.var_coxeffinv_dn12) * locals.var_vt);
        locals.var_wlcoxvtinv_dn13 = ((assign31280_e41399 * locals.var_coxeffinv_dn13) * locals.var_vt);
        locals.var_wlcoxvtinv_dn14 = ((assign31280_e41399 * locals.var_coxeffinv_dn14) * locals.var_vt);
        locals.var_wlcoxvtinv_rv = 0.0;

        let assign31290_e41405: f64 = (-locals.var_wlcoxvtinv);
        let assign31290_e41407: f64 = (assign31290_e41405 * locals.var_qs);
        locals.var_qsi = assign31290_e41407;
        locals.var_qsi_dn0 = (((-locals.var_wlcoxvtinv_dn0) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn0));
        locals.var_qsi_dn2 = (((-locals.var_wlcoxvtinv_dn2) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn2));
        locals.var_qsi_dn3 = (((-locals.var_wlcoxvtinv_dn3) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn3));
        locals.var_qsi_dn4 = (((-locals.var_wlcoxvtinv_dn4) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn4));
        locals.var_qsi_dn5 = (((-locals.var_wlcoxvtinv_dn5) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn5));
        locals.var_qsi_dn6 = (((-locals.var_wlcoxvtinv_dn6) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn6));
        locals.var_qsi_dn7 = (((-locals.var_wlcoxvtinv_dn7) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn7));
        locals.var_qsi_dn8 = (((-locals.var_wlcoxvtinv_dn8) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn8));
        locals.var_qsi_dn9 = (((-locals.var_wlcoxvtinv_dn9) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn9));
        locals.var_qsi_dn10 = (((-locals.var_wlcoxvtinv_dn10) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn10));
        locals.var_qsi_dn11 = (((-locals.var_wlcoxvtinv_dn11) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn11));
        locals.var_qsi_dn12 = (((-locals.var_wlcoxvtinv_dn12) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn12));
        locals.var_qsi_dn13 = (((-locals.var_wlcoxvtinv_dn13) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn13));
        locals.var_qsi_dn14 = (((-locals.var_wlcoxvtinv_dn14) * locals.var_qs) + (assign31290_e41405 * locals.var_qs_dn14));
        locals.var_qsi_rv = 0.0;

        let assign31300_e41409: f64 = (-locals.var_wlcoxvtinv);
        let assign31300_e41411: f64 = (assign31300_e41409 * locals.var_qd);
        locals.var_qdi = assign31300_e41411;
        locals.var_qdi_dn0 = (((-locals.var_wlcoxvtinv_dn0) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn0));
        locals.var_qdi_dn2 = (((-locals.var_wlcoxvtinv_dn2) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn2));
        locals.var_qdi_dn3 = (((-locals.var_wlcoxvtinv_dn3) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn3));
        locals.var_qdi_dn4 = (((-locals.var_wlcoxvtinv_dn4) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn4));
        locals.var_qdi_dn5 = (((-locals.var_wlcoxvtinv_dn5) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn5));
        locals.var_qdi_dn6 = (((-locals.var_wlcoxvtinv_dn6) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn6));
        locals.var_qdi_dn7 = (((-locals.var_wlcoxvtinv_dn7) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn7));
        locals.var_qdi_dn8 = (((-locals.var_wlcoxvtinv_dn8) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn8));
        locals.var_qdi_dn9 = (((-locals.var_wlcoxvtinv_dn9) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn9));
        locals.var_qdi_dn10 = (((-locals.var_wlcoxvtinv_dn10) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn10));
        locals.var_qdi_dn11 = (((-locals.var_wlcoxvtinv_dn11) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn11));
        locals.var_qdi_dn12 = (((-locals.var_wlcoxvtinv_dn12) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn12));
        locals.var_qdi_dn13 = (((-locals.var_wlcoxvtinv_dn13) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn13));
        locals.var_qdi_dn14 = (((-locals.var_wlcoxvtinv_dn14) * locals.var_qd) + (assign31300_e41409 * locals.var_qd_dn14));
        locals.var_qdi_rv = 0.0;

        let assign31310_e41414: f64 = (locals.var_qbi + locals.var_qsi);
        let assign31310_e41416: f64 = (assign31310_e41414 + locals.var_qdi);
        let assign31310_e41417: f64 = (-assign31310_e41416);
        locals.var_qgi = assign31310_e41417;
        locals.var_qgi_dn0 = (-((locals.var_qbi_dn0 + locals.var_qsi_dn0) + locals.var_qdi_dn0));
        locals.var_qgi_dn2 = (-((locals.var_qbi_dn2 + locals.var_qsi_dn2) + locals.var_qdi_dn2));
        locals.var_qgi_dn3 = (-((locals.var_qbi_dn3 + locals.var_qsi_dn3) + locals.var_qdi_dn3));
        locals.var_qgi_dn4 = (-((locals.var_qbi_dn4 + locals.var_qsi_dn4) + locals.var_qdi_dn4));
        locals.var_qgi_dn5 = (-((locals.var_qbi_dn5 + locals.var_qsi_dn5) + locals.var_qdi_dn5));
        locals.var_qgi_dn6 = (-((locals.var_qbi_dn6 + locals.var_qsi_dn6) + locals.var_qdi_dn6));
        locals.var_qgi_dn7 = (-((locals.var_qbi_dn7 + locals.var_qsi_dn7) + locals.var_qdi_dn7));
        locals.var_qgi_dn8 = (-((locals.var_qbi_dn8 + locals.var_qsi_dn8) + locals.var_qdi_dn8));
        locals.var_qgi_dn9 = (-((locals.var_qbi_dn9 + locals.var_qsi_dn9) + locals.var_qdi_dn9));
        locals.var_qgi_dn10 = (-((locals.var_qbi_dn10 + locals.var_qsi_dn10) + locals.var_qdi_dn10));
        locals.var_qgi_dn11 = (-((locals.var_qbi_dn11 + locals.var_qsi_dn11) + locals.var_qdi_dn11));
        locals.var_qgi_dn12 = (-((locals.var_qbi_dn12 + locals.var_qsi_dn12) + locals.var_qdi_dn12));
        locals.var_qgi_dn13 = (-((locals.var_qbi_dn13 + locals.var_qsi_dn13) + locals.var_qdi_dn13));
        locals.var_qgi_dn14 = (-((locals.var_qbi_dn14 + locals.var_qsi_dn14) + locals.var_qdi_dn14));
        locals.var_qgi_rv = 0.0;

        let assign31320_e41420: f64 = if (!param_given[666]) { 1.0 } else { 0.0 };
        locals.var_guard728 = assign31320_e41420;
        locals.var_guard728_rv = 0.0;

        let (assign31330_e41441,) = {
    if (locals.var_guard728 != 0.0) {
        let assign31330_e41424: f64 = (2.0 * p.p111);
        let assign31330_e41426: f64 = (assign31330_e41424 * 8.85418e-12);
        let assign31330_e41428: f64 = (assign31330_e41426 / 3.141592653589793);
        let assign31330_e41433: f64 = (4e-7 / p.p77);
        let assign31330_e41434: f64 = (1.0 + assign31330_e41433);
        let assign31330_e41435: f64 = (p.p670 * assign31330_e41434);
        let assign31330_e41437: f64 = (assign31330_e41435).max(1e-38);
        let assign31330_e41438: f64 = (assign31330_e41437).ln();
        let assign31330_e41439: f64 = (assign31330_e41428 * assign31330_e41438);
        (assign31330_e41439,)
    } else {
        (locals.var_cf_i,)
    }
};
        locals.var_cf_i = assign31330_e41441;
        locals.var_cf_i_rv = 0.0;

        let assign31340_e41444: f64 = (p.p671 + locals.var_cf_i);
        locals.var_cgsof = assign31340_e41444;
        locals.var_cgsof_rv = 0.0;

        let assign31350_e41447: f64 = (p.p672 + locals.var_cf_i);
        locals.var_cgdof = assign31350_e41447;
        locals.var_cgdof_rv = 0.0;

        let assign31360_e41450: f64 = if p.p41 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard729 = assign31360_e41450;
        locals.var_guard729_rv = 0.0;

        let (assign31370_e41461, assign31370_e41461_d_n0, assign31370_e41461_d_n2, assign31370_e41461_d_n3, assign31370_e41461_d_n4, assign31370_e41461_d_n5, assign31370_e41461_d_n6, assign31370_e41461_d_n7, assign31370_e41461_d_n8, assign31370_e41461_d_n9, assign31370_e41461_d_n10, assign31370_e41461_d_n11, assign31370_e41461_d_n12, assign31370_e41461_d_n13, assign31370_e41461_d_n14,) = {
    if (locals.var_guard729 != 0.0) {
        let assign31370_e41453: f64 = (-locals.var_wact);
        let assign31370_e41455: f64 = (assign31370_e41453 * p.p2);
        let assign31370_e41457: f64 = (assign31370_e41455 * locals.var_cgsof);
        let assign31370_e41459: f64 = (assign31370_e41457 * locals.var_vgs_ov_noswap);
        (assign31370_e41459, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (assign31370_e41457 * locals.var_vgs_ov_noswap_dn7), 0.0, 0.0, (assign31370_e41457 * locals.var_vgs_ov_noswap_dn10), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn3, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn12, locals.var_qovs_dn13, locals.var_qovs_dn14,)
    }
};
        locals.var_qovs = assign31370_e41461;
        locals.var_qovs_dn0 = assign31370_e41461_d_n0;
        locals.var_qovs_dn2 = assign31370_e41461_d_n2;
        locals.var_qovs_dn3 = assign31370_e41461_d_n3;
        locals.var_qovs_dn4 = assign31370_e41461_d_n4;
        locals.var_qovs_dn5 = assign31370_e41461_d_n5;
        locals.var_qovs_dn6 = assign31370_e41461_d_n6;
        locals.var_qovs_dn7 = assign31370_e41461_d_n7;
        locals.var_qovs_dn8 = assign31370_e41461_d_n8;
        locals.var_qovs_dn9 = assign31370_e41461_d_n9;
        locals.var_qovs_dn10 = assign31370_e41461_d_n10;
        locals.var_qovs_dn11 = assign31370_e41461_d_n11;
        locals.var_qovs_dn12 = assign31370_e41461_d_n12;
        locals.var_qovs_dn13 = assign31370_e41461_d_n13;
        locals.var_qovs_dn14 = assign31370_e41461_d_n14;
        locals.var_qovs_rv = 0.0;

        let (assign31380_e41472, assign31380_e41472_d_n0, assign31380_e41472_d_n2, assign31380_e41472_d_n3, assign31380_e41472_d_n4, assign31380_e41472_d_n5, assign31380_e41472_d_n6, assign31380_e41472_d_n7, assign31380_e41472_d_n8, assign31380_e41472_d_n9, assign31380_e41472_d_n10, assign31380_e41472_d_n11, assign31380_e41472_d_n12, assign31380_e41472_d_n13, assign31380_e41472_d_n14,) = {
    if (locals.var_guard729 != 0.0) {
        let assign31380_e41464: f64 = (-locals.var_wact);
        let assign31380_e41466: f64 = (assign31380_e41464 * p.p2);
        let assign31380_e41468: f64 = (assign31380_e41466 * locals.var_cgdof);
        let assign31380_e41470: f64 = (assign31380_e41468 * locals.var_vgd_ov_noswapcv);
        (assign31380_e41470, 0.0, 0.0, 0.0, 0.0, (assign31380_e41468 * locals.var_vgd_ov_noswapcv_dn5), (assign31380_e41468 * locals.var_vgd_ov_noswapcv_dn6), (assign31380_e41468 * locals.var_vgd_ov_noswapcv_dn7), 0.0, 0.0, (assign31380_e41468 * locals.var_vgd_ov_noswapcv_dn10), (assign31380_e41468 * locals.var_vgd_ov_noswapcv_dn11), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn3, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn12, locals.var_qovd_dn13, locals.var_qovd_dn14,)
    }
};
        locals.var_qovd = assign31380_e41472;
        locals.var_qovd_dn0 = assign31380_e41472_d_n0;
        locals.var_qovd_dn2 = assign31380_e41472_d_n2;
        locals.var_qovd_dn3 = assign31380_e41472_d_n3;
        locals.var_qovd_dn4 = assign31380_e41472_d_n4;
        locals.var_qovd_dn5 = assign31380_e41472_d_n5;
        locals.var_qovd_dn6 = assign31380_e41472_d_n6;
        locals.var_qovd_dn7 = assign31380_e41472_d_n7;
        locals.var_qovd_dn8 = assign31380_e41472_d_n8;
        locals.var_qovd_dn9 = assign31380_e41472_d_n9;
        locals.var_qovd_dn10 = assign31380_e41472_d_n10;
        locals.var_qovd_dn11 = assign31380_e41472_d_n11;
        locals.var_qovd_dn12 = assign31380_e41472_d_n12;
        locals.var_qovd_dn13 = assign31380_e41472_d_n13;
        locals.var_qovd_dn14 = assign31380_e41472_d_n14;
        locals.var_qovd_rv = 0.0;

        let (assign31390_e41492, assign31390_e41492_d_n0, assign31390_e41492_d_n2, assign31390_e41492_d_n3, assign31390_e41492_d_n4, assign31390_e41492_d_n5, assign31390_e41492_d_n6, assign31390_e41492_d_n7, assign31390_e41492_d_n8, assign31390_e41492_d_n9, assign31390_e41492_d_n10, assign31390_e41492_d_n11, assign31390_e41492_d_n12, assign31390_e41492_d_n13, assign31390_e41492_d_n14,) = {
    if (locals.var_guard729 == 0.0) {
        let assign31390_e41477: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsdr);
        let assign31390_e41479: f64 = (assign31390_e41477 + 0.02);
        let assign31390_e41482: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsdr);
        let assign31390_e41484: f64 = (assign31390_e41482 + 0.02);
        let assign31390_e41485: f64 = (assign31390_e41479 * assign31390_e41484);
        let assign31390_e41488: f64 = (4.0 * 0.02);
        let assign31390_e41489: f64 = (assign31390_e41485 + assign31390_e41488);
        let assign31390_e41490: f64 = (assign31390_e41489).sqrt();
        (assign31390_e41490, 0.0, 0.0, 0.0, ((((-locals.var_vfbsdr_dn4) * assign31390_e41484) + (assign31390_e41479 * (-locals.var_vfbsdr_dn4))) / (2.0 * assign31390_e41490)), 0.0, 0.0, (((locals.var_vgs_ov_noswap_dn7 * assign31390_e41484) + (assign31390_e41479 * locals.var_vgs_ov_noswap_dn7)) / (2.0 * assign31390_e41490)), 0.0, 0.0, (((locals.var_vgs_ov_noswap_dn10 * assign31390_e41484) + (assign31390_e41479 * locals.var_vgs_ov_noswap_dn10)) / (2.0 * assign31390_e41490)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign31390_e41492;
        locals.var_t0_dn0 = assign31390_e41492_d_n0;
        locals.var_t0_dn2 = assign31390_e41492_d_n2;
        locals.var_t0_dn3 = assign31390_e41492_d_n3;
        locals.var_t0_dn4 = assign31390_e41492_d_n4;
        locals.var_t0_dn5 = assign31390_e41492_d_n5;
        locals.var_t0_dn6 = assign31390_e41492_d_n6;
        locals.var_t0_dn7 = assign31390_e41492_d_n7;
        locals.var_t0_dn8 = assign31390_e41492_d_n8;
        locals.var_t0_dn9 = assign31390_e41492_d_n9;
        locals.var_t0_dn10 = assign31390_e41492_d_n10;
        locals.var_t0_dn11 = assign31390_e41492_d_n11;
        locals.var_t0_dn12 = assign31390_e41492_d_n12;
        locals.var_t0_dn13 = assign31390_e41492_d_n13;
        locals.var_t0_dn14 = assign31390_e41492_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign31400_e41505, assign31400_e41505_d_n0, assign31400_e41505_d_n2, assign31400_e41505_d_n3, assign31400_e41505_d_n4, assign31400_e41505_d_n5, assign31400_e41505_d_n6, assign31400_e41505_d_n7, assign31400_e41505_d_n8, assign31400_e41505_d_n9, assign31400_e41505_d_n10, assign31400_e41505_d_n11, assign31400_e41505_d_n12, assign31400_e41505_d_n13, assign31400_e41505_d_n14,) = {
    if (locals.var_guard729 == 0.0) {
        let assign31400_e41498: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsdr);
        let assign31400_e41500: f64 = (assign31400_e41498 + 0.02);
        let assign31400_e41502: f64 = (assign31400_e41500 - locals.var_t0);
        let assign31400_e41503: f64 = (0.5 * assign31400_e41502);
        (assign31400_e41503, (0.5 * (-locals.var_t0_dn0)), (0.5 * (-locals.var_t0_dn2)), (0.5 * (-locals.var_t0_dn3)), (0.5 * ((-locals.var_vfbsdr_dn4) - locals.var_t0_dn4)), (0.5 * (-locals.var_t0_dn5)), (0.5 * (-locals.var_t0_dn6)), (0.5 * (locals.var_vgs_ov_noswap_dn7 - locals.var_t0_dn7)), (0.5 * (-locals.var_t0_dn8)), (0.5 * (-locals.var_t0_dn9)), (0.5 * (locals.var_vgs_ov_noswap_dn10 - locals.var_t0_dn10)), (0.5 * (-locals.var_t0_dn11)), (0.5 * (-locals.var_t0_dn12)), (0.5 * (-locals.var_t0_dn13)), (0.5 * (-locals.var_t0_dn14)),)
    } else {
        (locals.var_vgsov, locals.var_vgsov_dn0, locals.var_vgsov_dn2, locals.var_vgsov_dn3, locals.var_vgsov_dn4, locals.var_vgsov_dn5, locals.var_vgsov_dn6, locals.var_vgsov_dn7, locals.var_vgsov_dn8, locals.var_vgsov_dn9, locals.var_vgsov_dn10, locals.var_vgsov_dn11, locals.var_vgsov_dn12, locals.var_vgsov_dn13, locals.var_vgsov_dn14,)
    }
};
        locals.var_vgsov = assign31400_e41505;
        locals.var_vgsov_dn0 = assign31400_e41505_d_n0;
        locals.var_vgsov_dn2 = assign31400_e41505_d_n2;
        locals.var_vgsov_dn3 = assign31400_e41505_d_n3;
        locals.var_vgsov_dn4 = assign31400_e41505_d_n4;
        locals.var_vgsov_dn5 = assign31400_e41505_d_n5;
        locals.var_vgsov_dn6 = assign31400_e41505_d_n6;
        locals.var_vgsov_dn7 = assign31400_e41505_d_n7;
        locals.var_vgsov_dn8 = assign31400_e41505_d_n8;
        locals.var_vgsov_dn9 = assign31400_e41505_d_n9;
        locals.var_vgsov_dn10 = assign31400_e41505_d_n10;
        locals.var_vgsov_dn11 = assign31400_e41505_d_n11;
        locals.var_vgsov_dn12 = assign31400_e41505_d_n12;
        locals.var_vgsov_dn13 = assign31400_e41505_d_n13;
        locals.var_vgsov_dn14 = assign31400_e41505_d_n14;
        locals.var_vgsov_rv = 0.0;

        let (assign31410_e41523, assign31410_e41523_d_n0, assign31410_e41523_d_n2, assign31410_e41523_d_n3, assign31410_e41523_d_n4, assign31410_e41523_d_n5, assign31410_e41523_d_n6, assign31410_e41523_d_n7, assign31410_e41523_d_n8, assign31410_e41523_d_n9, assign31410_e41523_d_n10, assign31410_e41523_d_n11, assign31410_e41523_d_n12, assign31410_e41523_d_n13, assign31410_e41523_d_n14,) = {
    if (locals.var_guard729 == 0.0) {
        let assign31410_e41511: f64 = (-locals.var_vgsov);
        let assign31410_e41513: f64 = (assign31410_e41511 / p.p692);
        let assign31410_e41515: f64 = (assign31410_e41513).powf(p.p693);
        let assign31410_e41516: f64 = (1.0 + assign31410_e41515);
        let assign31410_e41519: f64 = (1.0 / p.p693);
        let assign31410_e41520: f64 = (assign31410_e41516).powf(assign31410_e41519);
        let assign31410_e41521: f64 = (locals.var_vgsov / assign31410_e41520);
        (assign31410_e41521, (((locals.var_vgsov_dn0 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn0) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn0) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn0) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn0) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn2 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn2) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn2) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn2) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn2) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn3 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn3) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn3) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn3) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn3) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn4 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn4) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn4) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn4) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn4) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn5 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn5) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn5) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn5) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn5) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn6 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn6) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn6) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn6) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn6) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn7 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn7) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn7) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn7) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn7) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn8 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn8) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn8) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn8) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn8) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn9 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn9) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn9) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn9) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn9) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn10 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn10) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn10) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn10) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn10) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn11 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn11) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn11) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn11) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn11) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn12 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn12) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn12) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn12) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn12) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn13 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn13) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn13) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn13) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn13) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)), (((locals.var_vgsov_dn14 * assign31410_e41520) - (locals.var_vgsov * if 0.0 == 0.0 && ((assign31410_e41519) as f64).is_finite() && ((assign31410_e41519) as f64).fract() == 0.0 { if assign31410_e41519 == 0.0 { 0.0 } else { (assign31410_e41519 * ((assign31410_e41516).powf(assign31410_e41519 - 1.0) * if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn14) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn14) / p.p692) / assign31410_e41513))) })) } } else { (assign31410_e41520 * (assign31410_e41519 * (if 0.0 == 0.0 && ((p.p693) as f64).is_finite() && ((p.p693) as f64).fract() == 0.0 { if p.p693 == 0.0 { 0.0 } else { (p.p693 * ((assign31410_e41513).powf(p.p693 - 1.0) * ((-locals.var_vgsov_dn14) / p.p692))) } } else { (assign31410_e41515 * (p.p693 * (((-locals.var_vgsov_dn14) / p.p692) / assign31410_e41513))) } / assign31410_e41516))) })) / (assign31410_e41520 * assign31410_e41520)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign31410_e41523;
        locals.var_t6_dn0 = assign31410_e41523_d_n0;
        locals.var_t6_dn2 = assign31410_e41523_d_n2;
        locals.var_t6_dn3 = assign31410_e41523_d_n3;
        locals.var_t6_dn4 = assign31410_e41523_d_n4;
        locals.var_t6_dn5 = assign31410_e41523_d_n5;
        locals.var_t6_dn6 = assign31410_e41523_d_n6;
        locals.var_t6_dn7 = assign31410_e41523_d_n7;
        locals.var_t6_dn8 = assign31410_e41523_d_n8;
        locals.var_t6_dn9 = assign31410_e41523_d_n9;
        locals.var_t6_dn10 = assign31410_e41523_d_n10;
        locals.var_t6_dn11 = assign31410_e41523_d_n11;
        locals.var_t6_dn12 = assign31410_e41523_d_n12;
        locals.var_t6_dn13 = assign31410_e41523_d_n13;
        locals.var_t6_dn14 = assign31410_e41523_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign31420_e41535, assign31420_e41535_d_n0, assign31420_e41535_d_n2, assign31420_e41535_d_n3, assign31420_e41535_d_n4, assign31420_e41535_d_n5, assign31420_e41535_d_n6, assign31420_e41535_d_n7, assign31420_e41535_d_n8, assign31420_e41535_d_n9, assign31420_e41535_d_n10, assign31420_e41535_d_n11, assign31420_e41535_d_n12, assign31420_e41535_d_n13, assign31420_e41535_d_n14,) = {
    if (locals.var_guard729 == 0.0) {
        let assign31420_e41529: f64 = (4.0 * locals.var_t6);
        let assign31420_e41531: f64 = (assign31420_e41529 / locals.var_ckappas_i);
        let assign31420_e41532: f64 = (1.0 - assign31420_e41531);
        let assign31420_e41533: f64 = (assign31420_e41532).sqrt();
        (assign31420_e41533, ((-((4.0 * locals.var_t6_dn0) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn2) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn3) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn4) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn5) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn6) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn7) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn8) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn9) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn10) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn11) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn12) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn13) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)), ((-((4.0 * locals.var_t6_dn14) / locals.var_ckappas_i)) / (2.0 * assign31420_e41533)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign31420_e41535;
        locals.var_t1_dn0 = assign31420_e41535_d_n0;
        locals.var_t1_dn2 = assign31420_e41535_d_n2;
        locals.var_t1_dn3 = assign31420_e41535_d_n3;
        locals.var_t1_dn4 = assign31420_e41535_d_n4;
        locals.var_t1_dn5 = assign31420_e41535_d_n5;
        locals.var_t1_dn6 = assign31420_e41535_d_n6;
        locals.var_t1_dn7 = assign31420_e41535_d_n7;
        locals.var_t1_dn8 = assign31420_e41535_d_n8;
        locals.var_t1_dn9 = assign31420_e41535_d_n9;
        locals.var_t1_dn10 = assign31420_e41535_d_n10;
        locals.var_t1_dn11 = assign31420_e41535_d_n11;
        locals.var_t1_dn12 = assign31420_e41535_d_n12;
        locals.var_t1_dn13 = assign31420_e41535_d_n13;
        locals.var_t1_dn14 = assign31420_e41535_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign31430_e41564, assign31430_e41564_d_n0, assign31430_e41564_d_n2, assign31430_e41564_d_n3, assign31430_e41564_d_n4, assign31430_e41564_d_n5, assign31430_e41564_d_n6, assign31430_e41564_d_n7, assign31430_e41564_d_n8, assign31430_e41564_d_n9, assign31430_e41564_d_n10, assign31430_e41564_d_n11, assign31430_e41564_d_n12, assign31430_e41564_d_n13, assign31430_e41564_d_n14,) = {
    if (locals.var_guard729 == 0.0) {
        let assign31430_e41539: f64 = (-locals.var_wact);
        let assign31430_e41541: f64 = (assign31430_e41539 * p.p2);
        let assign31430_e41544: f64 = (locals.var_cgsof * locals.var_vgs_ov_noswap);
        let assign31430_e41548: f64 = (locals.var_vgs_ov_noswap - locals.var_vfbsdr);
        let assign31430_e41550: f64 = (assign31430_e41548 - locals.var_vgsov);
        let assign31430_e41553: f64 = (0.5 * locals.var_ckappas_i);
        let assign31430_e41555: f64 = (-1.0);
        let assign31430_e41557: f64 = (assign31430_e41555 + locals.var_t1);
        let assign31430_e41558: f64 = (assign31430_e41553 * assign31430_e41557);
        let assign31430_e41559: f64 = (assign31430_e41550 - assign31430_e41558);
        let assign31430_e41560: f64 = (locals.var_cgsl_i * assign31430_e41559);
        let assign31430_e41561: f64 = (assign31430_e41544 + assign31430_e41560);
        let assign31430_e41562: f64 = (assign31430_e41541 * assign31430_e41561);
        (assign31430_e41562, (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn0) - (assign31430_e41553 * locals.var_t1_dn0)))), (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn2) - (assign31430_e41553 * locals.var_t1_dn2)))), (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn3) - (assign31430_e41553 * locals.var_t1_dn3)))), (assign31430_e41541 * (locals.var_cgsl_i * (((-locals.var_vfbsdr_dn4) - locals.var_vgsov_dn4) - (assign31430_e41553 * locals.var_t1_dn4)))), (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn5) - (assign31430_e41553 * locals.var_t1_dn5)))), (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn6) - (assign31430_e41553 * locals.var_t1_dn6)))), (assign31430_e41541 * ((locals.var_cgsof * locals.var_vgs_ov_noswap_dn7) + (locals.var_cgsl_i * ((locals.var_vgs_ov_noswap_dn7 - locals.var_vgsov_dn7) - (assign31430_e41553 * locals.var_t1_dn7))))), (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn8) - (assign31430_e41553 * locals.var_t1_dn8)))), (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn9) - (assign31430_e41553 * locals.var_t1_dn9)))), (assign31430_e41541 * ((locals.var_cgsof * locals.var_vgs_ov_noswap_dn10) + (locals.var_cgsl_i * ((locals.var_vgs_ov_noswap_dn10 - locals.var_vgsov_dn10) - (assign31430_e41553 * locals.var_t1_dn10))))), (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn11) - (assign31430_e41553 * locals.var_t1_dn11)))), (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn12) - (assign31430_e41553 * locals.var_t1_dn12)))), (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn13) - (assign31430_e41553 * locals.var_t1_dn13)))), (assign31430_e41541 * (locals.var_cgsl_i * ((-locals.var_vgsov_dn14) - (assign31430_e41553 * locals.var_t1_dn14)))),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn3, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn12, locals.var_qovs_dn13, locals.var_qovs_dn14,)
    }
};
        locals.var_qovs = assign31430_e41564;
        locals.var_qovs_dn0 = assign31430_e41564_d_n0;
        locals.var_qovs_dn2 = assign31430_e41564_d_n2;
        locals.var_qovs_dn3 = assign31430_e41564_d_n3;
        locals.var_qovs_dn4 = assign31430_e41564_d_n4;
        locals.var_qovs_dn5 = assign31430_e41564_d_n5;
        locals.var_qovs_dn6 = assign31430_e41564_d_n6;
        locals.var_qovs_dn7 = assign31430_e41564_d_n7;
        locals.var_qovs_dn8 = assign31430_e41564_d_n8;
        locals.var_qovs_dn9 = assign31430_e41564_d_n9;
        locals.var_qovs_dn10 = assign31430_e41564_d_n10;
        locals.var_qovs_dn11 = assign31430_e41564_d_n11;
        locals.var_qovs_dn12 = assign31430_e41564_d_n12;
        locals.var_qovs_dn13 = assign31430_e41564_d_n13;
        locals.var_qovs_dn14 = assign31430_e41564_d_n14;
        locals.var_qovs_rv = 0.0;

        let (assign31440_e41584, assign31440_e41584_d_n0, assign31440_e41584_d_n2, assign31440_e41584_d_n3, assign31440_e41584_d_n4, assign31440_e41584_d_n5, assign31440_e41584_d_n6, assign31440_e41584_d_n7, assign31440_e41584_d_n8, assign31440_e41584_d_n9, assign31440_e41584_d_n10, assign31440_e41584_d_n11, assign31440_e41584_d_n12, assign31440_e41584_d_n13, assign31440_e41584_d_n14,) = {
    if (locals.var_guard729 == 0.0) {
        let assign31440_e41569: f64 = (locals.var_vgd_ov_noswapcv - locals.var_vfbsdr);
        let assign31440_e41571: f64 = (assign31440_e41569 + 0.02);
        let assign31440_e41574: f64 = (locals.var_vgd_ov_noswapcv - locals.var_vfbsdr);
        let assign31440_e41576: f64 = (assign31440_e41574 + 0.02);
        let assign31440_e41577: f64 = (assign31440_e41571 * assign31440_e41576);
        let assign31440_e41580: f64 = (4.0 * 0.02);
        let assign31440_e41581: f64 = (assign31440_e41577 + assign31440_e41580);
        let assign31440_e41582: f64 = (assign31440_e41581).sqrt();
        (assign31440_e41582, 0.0, 0.0, 0.0, ((((-locals.var_vfbsdr_dn4) * assign31440_e41576) + (assign31440_e41571 * (-locals.var_vfbsdr_dn4))) / (2.0 * assign31440_e41582)), (((locals.var_vgd_ov_noswapcv_dn5 * assign31440_e41576) + (assign31440_e41571 * locals.var_vgd_ov_noswapcv_dn5)) / (2.0 * assign31440_e41582)), (((locals.var_vgd_ov_noswapcv_dn6 * assign31440_e41576) + (assign31440_e41571 * locals.var_vgd_ov_noswapcv_dn6)) / (2.0 * assign31440_e41582)), (((locals.var_vgd_ov_noswapcv_dn7 * assign31440_e41576) + (assign31440_e41571 * locals.var_vgd_ov_noswapcv_dn7)) / (2.0 * assign31440_e41582)), 0.0, 0.0, (((locals.var_vgd_ov_noswapcv_dn10 * assign31440_e41576) + (assign31440_e41571 * locals.var_vgd_ov_noswapcv_dn10)) / (2.0 * assign31440_e41582)), (((locals.var_vgd_ov_noswapcv_dn11 * assign31440_e41576) + (assign31440_e41571 * locals.var_vgd_ov_noswapcv_dn11)) / (2.0 * assign31440_e41582)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign31440_e41584;
        locals.var_t0_dn0 = assign31440_e41584_d_n0;
        locals.var_t0_dn2 = assign31440_e41584_d_n2;
        locals.var_t0_dn3 = assign31440_e41584_d_n3;
        locals.var_t0_dn4 = assign31440_e41584_d_n4;
        locals.var_t0_dn5 = assign31440_e41584_d_n5;
        locals.var_t0_dn6 = assign31440_e41584_d_n6;
        locals.var_t0_dn7 = assign31440_e41584_d_n7;
        locals.var_t0_dn8 = assign31440_e41584_d_n8;
        locals.var_t0_dn9 = assign31440_e41584_d_n9;
        locals.var_t0_dn10 = assign31440_e41584_d_n10;
        locals.var_t0_dn11 = assign31440_e41584_d_n11;
        locals.var_t0_dn12 = assign31440_e41584_d_n12;
        locals.var_t0_dn13 = assign31440_e41584_d_n13;
        locals.var_t0_dn14 = assign31440_e41584_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign31450_e41597, assign31450_e41597_d_n0, assign31450_e41597_d_n2, assign31450_e41597_d_n3, assign31450_e41597_d_n4, assign31450_e41597_d_n5, assign31450_e41597_d_n6, assign31450_e41597_d_n7, assign31450_e41597_d_n8, assign31450_e41597_d_n9, assign31450_e41597_d_n10, assign31450_e41597_d_n11, assign31450_e41597_d_n12, assign31450_e41597_d_n13, assign31450_e41597_d_n14,) = {
    if (locals.var_guard729 == 0.0) {
        let assign31450_e41590: f64 = (locals.var_vgd_ov_noswapcv - locals.var_vfbsdr);
        let assign31450_e41592: f64 = (assign31450_e41590 + 0.02);
        let assign31450_e41594: f64 = (assign31450_e41592 - locals.var_t0);
        let assign31450_e41595: f64 = (0.5 * assign31450_e41594);
        (assign31450_e41595, (0.5 * (-locals.var_t0_dn0)), (0.5 * (-locals.var_t0_dn2)), (0.5 * (-locals.var_t0_dn3)), (0.5 * ((-locals.var_vfbsdr_dn4) - locals.var_t0_dn4)), (0.5 * (locals.var_vgd_ov_noswapcv_dn5 - locals.var_t0_dn5)), (0.5 * (locals.var_vgd_ov_noswapcv_dn6 - locals.var_t0_dn6)), (0.5 * (locals.var_vgd_ov_noswapcv_dn7 - locals.var_t0_dn7)), (0.5 * (-locals.var_t0_dn8)), (0.5 * (-locals.var_t0_dn9)), (0.5 * (locals.var_vgd_ov_noswapcv_dn10 - locals.var_t0_dn10)), (0.5 * (locals.var_vgd_ov_noswapcv_dn11 - locals.var_t0_dn11)), (0.5 * (-locals.var_t0_dn12)), (0.5 * (-locals.var_t0_dn13)), (0.5 * (-locals.var_t0_dn14)),)
    } else {
        (locals.var_vgdov, locals.var_vgdov_dn0, locals.var_vgdov_dn2, locals.var_vgdov_dn3, locals.var_vgdov_dn4, locals.var_vgdov_dn5, locals.var_vgdov_dn6, locals.var_vgdov_dn7, locals.var_vgdov_dn8, locals.var_vgdov_dn9, locals.var_vgdov_dn10, locals.var_vgdov_dn11, locals.var_vgdov_dn12, locals.var_vgdov_dn13, locals.var_vgdov_dn14,)
    }
};
        locals.var_vgdov = assign31450_e41597;
        locals.var_vgdov_dn0 = assign31450_e41597_d_n0;
        locals.var_vgdov_dn2 = assign31450_e41597_d_n2;
        locals.var_vgdov_dn3 = assign31450_e41597_d_n3;
        locals.var_vgdov_dn4 = assign31450_e41597_d_n4;
        locals.var_vgdov_dn5 = assign31450_e41597_d_n5;
        locals.var_vgdov_dn6 = assign31450_e41597_d_n6;
        locals.var_vgdov_dn7 = assign31450_e41597_d_n7;
        locals.var_vgdov_dn8 = assign31450_e41597_d_n8;
        locals.var_vgdov_dn9 = assign31450_e41597_d_n9;
        locals.var_vgdov_dn10 = assign31450_e41597_d_n10;
        locals.var_vgdov_dn11 = assign31450_e41597_d_n11;
        locals.var_vgdov_dn12 = assign31450_e41597_d_n12;
        locals.var_vgdov_dn13 = assign31450_e41597_d_n13;
        locals.var_vgdov_dn14 = assign31450_e41597_d_n14;
        locals.var_vgdov_rv = 0.0;

        let (assign31460_e41615, assign31460_e41615_d_n0, assign31460_e41615_d_n2, assign31460_e41615_d_n3, assign31460_e41615_d_n4, assign31460_e41615_d_n5, assign31460_e41615_d_n6, assign31460_e41615_d_n7, assign31460_e41615_d_n8, assign31460_e41615_d_n9, assign31460_e41615_d_n10, assign31460_e41615_d_n11, assign31460_e41615_d_n12, assign31460_e41615_d_n13, assign31460_e41615_d_n14,) = {
    if (locals.var_guard729 == 0.0) {
        let assign31460_e41603: f64 = (-locals.var_vgdov);
        let assign31460_e41605: f64 = (assign31460_e41603 / p.p690);
        let assign31460_e41607: f64 = (assign31460_e41605).powf(p.p691);
        let assign31460_e41608: f64 = (1.0 + assign31460_e41607);
        let assign31460_e41611: f64 = (1.0 / p.p691);
        let assign31460_e41612: f64 = (assign31460_e41608).powf(assign31460_e41611);
        let assign31460_e41613: f64 = (locals.var_vgdov / assign31460_e41612);
        (assign31460_e41613, (((locals.var_vgdov_dn0 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn0) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn0) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn0) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn0) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn2 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn2) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn2) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn2) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn2) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn3 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn3) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn3) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn3) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn3) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn4 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn4) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn4) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn4) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn4) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn5 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn5) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn5) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn5) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn5) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn6 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn6) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn6) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn6) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn6) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn7 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn7) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn7) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn7) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn7) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn8 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn8) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn8) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn8) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn8) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn9 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn9) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn9) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn9) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn9) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn10 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn10) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn10) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn10) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn10) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn11 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn11) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn11) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn11) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn11) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn12 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn12) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn12) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn12) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn12) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn13 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn13) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn13) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn13) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn13) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)), (((locals.var_vgdov_dn14 * assign31460_e41612) - (locals.var_vgdov * if 0.0 == 0.0 && ((assign31460_e41611) as f64).is_finite() && ((assign31460_e41611) as f64).fract() == 0.0 { if assign31460_e41611 == 0.0 { 0.0 } else { (assign31460_e41611 * ((assign31460_e41608).powf(assign31460_e41611 - 1.0) * if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn14) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn14) / p.p690) / assign31460_e41605))) })) } } else { (assign31460_e41612 * (assign31460_e41611 * (if 0.0 == 0.0 && ((p.p691) as f64).is_finite() && ((p.p691) as f64).fract() == 0.0 { if p.p691 == 0.0 { 0.0 } else { (p.p691 * ((assign31460_e41605).powf(p.p691 - 1.0) * ((-locals.var_vgdov_dn14) / p.p690))) } } else { (assign31460_e41607 * (p.p691 * (((-locals.var_vgdov_dn14) / p.p690) / assign31460_e41605))) } / assign31460_e41608))) })) / (assign31460_e41612 * assign31460_e41612)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign31460_e41615;
        locals.var_t6_dn0 = assign31460_e41615_d_n0;
        locals.var_t6_dn2 = assign31460_e41615_d_n2;
        locals.var_t6_dn3 = assign31460_e41615_d_n3;
        locals.var_t6_dn4 = assign31460_e41615_d_n4;
        locals.var_t6_dn5 = assign31460_e41615_d_n5;
        locals.var_t6_dn6 = assign31460_e41615_d_n6;
        locals.var_t6_dn7 = assign31460_e41615_d_n7;
        locals.var_t6_dn8 = assign31460_e41615_d_n8;
        locals.var_t6_dn9 = assign31460_e41615_d_n9;
        locals.var_t6_dn10 = assign31460_e41615_d_n10;
        locals.var_t6_dn11 = assign31460_e41615_d_n11;
        locals.var_t6_dn12 = assign31460_e41615_d_n12;
        locals.var_t6_dn13 = assign31460_e41615_d_n13;
        locals.var_t6_dn14 = assign31460_e41615_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign31470_e41627, assign31470_e41627_d_n0, assign31470_e41627_d_n2, assign31470_e41627_d_n3, assign31470_e41627_d_n4, assign31470_e41627_d_n5, assign31470_e41627_d_n6, assign31470_e41627_d_n7, assign31470_e41627_d_n8, assign31470_e41627_d_n9, assign31470_e41627_d_n10, assign31470_e41627_d_n11, assign31470_e41627_d_n12, assign31470_e41627_d_n13, assign31470_e41627_d_n14,) = {
    if (locals.var_guard729 == 0.0) {
        let assign31470_e41621: f64 = (4.0 * locals.var_t6);
        let assign31470_e41623: f64 = (assign31470_e41621 / locals.var_ckappad_i);
        let assign31470_e41624: f64 = (1.0 - assign31470_e41623);
        let assign31470_e41625: f64 = (assign31470_e41624).sqrt();
        (assign31470_e41625, ((-((4.0 * locals.var_t6_dn0) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn2) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn3) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn4) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn5) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn6) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn7) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn8) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn9) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn10) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn11) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn12) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn13) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)), ((-((4.0 * locals.var_t6_dn14) / locals.var_ckappad_i)) / (2.0 * assign31470_e41625)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign31470_e41627;
        locals.var_t2_dn0 = assign31470_e41627_d_n0;
        locals.var_t2_dn2 = assign31470_e41627_d_n2;
        locals.var_t2_dn3 = assign31470_e41627_d_n3;
        locals.var_t2_dn4 = assign31470_e41627_d_n4;
        locals.var_t2_dn5 = assign31470_e41627_d_n5;
        locals.var_t2_dn6 = assign31470_e41627_d_n6;
        locals.var_t2_dn7 = assign31470_e41627_d_n7;
        locals.var_t2_dn8 = assign31470_e41627_d_n8;
        locals.var_t2_dn9 = assign31470_e41627_d_n9;
        locals.var_t2_dn10 = assign31470_e41627_d_n10;
        locals.var_t2_dn11 = assign31470_e41627_d_n11;
        locals.var_t2_dn12 = assign31470_e41627_d_n12;
        locals.var_t2_dn13 = assign31470_e41627_d_n13;
        locals.var_t2_dn14 = assign31470_e41627_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_100(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (assign31480_e41656, assign31480_e41656_d_n0, assign31480_e41656_d_n2, assign31480_e41656_d_n3, assign31480_e41656_d_n4, assign31480_e41656_d_n5, assign31480_e41656_d_n6, assign31480_e41656_d_n7, assign31480_e41656_d_n8, assign31480_e41656_d_n9, assign31480_e41656_d_n10, assign31480_e41656_d_n11, assign31480_e41656_d_n12, assign31480_e41656_d_n13, assign31480_e41656_d_n14,) = {
    if (locals.var_guard729 == 0.0) {
        let assign31480_e41631: f64 = (-locals.var_wact);
        let assign31480_e41633: f64 = (assign31480_e41631 * p.p2);
        let assign31480_e41636: f64 = (locals.var_cgdof * locals.var_vgd_ov_noswapcv);
        let assign31480_e41640: f64 = (locals.var_vgd_ov_noswapcv - locals.var_vfbsdr);
        let assign31480_e41642: f64 = (assign31480_e41640 - locals.var_vgdov);
        let assign31480_e41645: f64 = (0.5 * locals.var_ckappad_i);
        let assign31480_e41647: f64 = (-1.0);
        let assign31480_e41649: f64 = (assign31480_e41647 + locals.var_t2);
        let assign31480_e41650: f64 = (assign31480_e41645 * assign31480_e41649);
        let assign31480_e41651: f64 = (assign31480_e41642 - assign31480_e41650);
        let assign31480_e41652: f64 = (locals.var_cgdl_i * assign31480_e41651);
        let assign31480_e41653: f64 = (assign31480_e41636 + assign31480_e41652);
        let assign31480_e41654: f64 = (assign31480_e41633 * assign31480_e41653);
        (assign31480_e41654, (assign31480_e41633 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn0) - (assign31480_e41645 * locals.var_t2_dn0)))), (assign31480_e41633 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn2) - (assign31480_e41645 * locals.var_t2_dn2)))), (assign31480_e41633 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn3) - (assign31480_e41645 * locals.var_t2_dn3)))), (assign31480_e41633 * (locals.var_cgdl_i * (((-locals.var_vfbsdr_dn4) - locals.var_vgdov_dn4) - (assign31480_e41645 * locals.var_t2_dn4)))), (assign31480_e41633 * ((locals.var_cgdof * locals.var_vgd_ov_noswapcv_dn5) + (locals.var_cgdl_i * ((locals.var_vgd_ov_noswapcv_dn5 - locals.var_vgdov_dn5) - (assign31480_e41645 * locals.var_t2_dn5))))), (assign31480_e41633 * ((locals.var_cgdof * locals.var_vgd_ov_noswapcv_dn6) + (locals.var_cgdl_i * ((locals.var_vgd_ov_noswapcv_dn6 - locals.var_vgdov_dn6) - (assign31480_e41645 * locals.var_t2_dn6))))), (assign31480_e41633 * ((locals.var_cgdof * locals.var_vgd_ov_noswapcv_dn7) + (locals.var_cgdl_i * ((locals.var_vgd_ov_noswapcv_dn7 - locals.var_vgdov_dn7) - (assign31480_e41645 * locals.var_t2_dn7))))), (assign31480_e41633 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn8) - (assign31480_e41645 * locals.var_t2_dn8)))), (assign31480_e41633 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn9) - (assign31480_e41645 * locals.var_t2_dn9)))), (assign31480_e41633 * ((locals.var_cgdof * locals.var_vgd_ov_noswapcv_dn10) + (locals.var_cgdl_i * ((locals.var_vgd_ov_noswapcv_dn10 - locals.var_vgdov_dn10) - (assign31480_e41645 * locals.var_t2_dn10))))), (assign31480_e41633 * ((locals.var_cgdof * locals.var_vgd_ov_noswapcv_dn11) + (locals.var_cgdl_i * ((locals.var_vgd_ov_noswapcv_dn11 - locals.var_vgdov_dn11) - (assign31480_e41645 * locals.var_t2_dn11))))), (assign31480_e41633 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn12) - (assign31480_e41645 * locals.var_t2_dn12)))), (assign31480_e41633 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn13) - (assign31480_e41645 * locals.var_t2_dn13)))), (assign31480_e41633 * (locals.var_cgdl_i * ((-locals.var_vgdov_dn14) - (assign31480_e41645 * locals.var_t2_dn14)))),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn3, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn12, locals.var_qovd_dn13, locals.var_qovd_dn14,)
    }
};
        locals.var_qovd = assign31480_e41656;
        locals.var_qovd_dn0 = assign31480_e41656_d_n0;
        locals.var_qovd_dn2 = assign31480_e41656_d_n2;
        locals.var_qovd_dn3 = assign31480_e41656_d_n3;
        locals.var_qovd_dn4 = assign31480_e41656_d_n4;
        locals.var_qovd_dn5 = assign31480_e41656_d_n5;
        locals.var_qovd_dn6 = assign31480_e41656_d_n6;
        locals.var_qovd_dn7 = assign31480_e41656_d_n7;
        locals.var_qovd_dn8 = assign31480_e41656_d_n8;
        locals.var_qovd_dn9 = assign31480_e41656_d_n9;
        locals.var_qovd_dn10 = assign31480_e41656_d_n10;
        locals.var_qovd_dn11 = assign31480_e41656_d_n11;
        locals.var_qovd_dn12 = assign31480_e41656_d_n12;
        locals.var_qovd_dn13 = assign31480_e41656_d_n13;
        locals.var_qovd_dn14 = assign31480_e41656_d_n14;
        locals.var_qovd_rv = 0.0;

        let assign31490_e41658: f64 = (-locals.var_devsign);
        let assign31490_e41660: f64 = (assign31490_e41658 * p.p2);
        let assign31490_e41662: f64 = (assign31490_e41660 * locals.var_lact);
        let assign31490_e41664: f64 = (assign31490_e41662 * p.p673);
        let assign31490_e41666: f64 = (assign31490_e41664 * (nv10 - nv11));
        locals.var_qovb = assign31490_e41666;
        locals.var_qovb_dn0 = 0.0;
        locals.var_qovb_dn2 = 0.0;
        locals.var_qovb_dn3 = 0.0;
        locals.var_qovb_dn4 = 0.0;
        locals.var_qovb_dn5 = 0.0;
        locals.var_qovb_dn6 = 0.0;
        locals.var_qovb_dn7 = 0.0;
        locals.var_qovb_dn8 = 0.0;
        locals.var_qovb_dn9 = 0.0;
        locals.var_qovb_dn10 = assign31490_e41664;
        locals.var_qovb_dn11 = (-assign31490_e41664);
        locals.var_qovb_dn12 = 0.0;
        locals.var_qovb_dn13 = 0.0;
        locals.var_qovb_dn14 = 0.0;
        locals.var_qovb_rv = 0.0;

        let assign31510_e41675: f64 = if p.p37 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard730 = assign31510_e41675;
        locals.var_guard730_rv = 0.0;

        let (assign31520_e41684, assign31520_e41684_d_n0, assign31520_e41684_d_n2, assign31520_e41684_d_n3, assign31520_e41684_d_n4, assign31520_e41684_d_n5, assign31520_e41684_d_n6, assign31520_e41684_d_n7, assign31520_e41684_d_n8, assign31520_e41684_d_n9, assign31520_e41684_d_n10, assign31520_e41684_d_n11, assign31520_e41684_d_n12, assign31520_e41684_d_n13, assign31520_e41684_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31520_e41679: f64 = (locals.var_ndepedge_i / locals.var_ni);
        let assign31520_e41681: f64 = (assign31520_e41679).max(1e-38);
        let assign31520_e41682: f64 = (assign31520_e41681).ln();
        (assign31520_e41682, (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn12) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681), (if assign31520_e41679 >= 1e-38 { (-((locals.var_ndepedge_i * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) } else { 0.0 } / assign31520_e41681),)
    } else {
        (locals.var_phib_edge, locals.var_phib_edge_dn0, locals.var_phib_edge_dn2, locals.var_phib_edge_dn3, locals.var_phib_edge_dn4, locals.var_phib_edge_dn5, locals.var_phib_edge_dn6, locals.var_phib_edge_dn7, locals.var_phib_edge_dn8, locals.var_phib_edge_dn9, locals.var_phib_edge_dn10, locals.var_phib_edge_dn11, locals.var_phib_edge_dn12, locals.var_phib_edge_dn13, locals.var_phib_edge_dn14,)
    }
};
        locals.var_phib_edge = assign31520_e41684;
        locals.var_phib_edge_dn0 = assign31520_e41684_d_n0;
        locals.var_phib_edge_dn2 = assign31520_e41684_d_n2;
        locals.var_phib_edge_dn3 = assign31520_e41684_d_n3;
        locals.var_phib_edge_dn4 = assign31520_e41684_d_n4;
        locals.var_phib_edge_dn5 = assign31520_e41684_d_n5;
        locals.var_phib_edge_dn6 = assign31520_e41684_d_n6;
        locals.var_phib_edge_dn7 = assign31520_e41684_d_n7;
        locals.var_phib_edge_dn8 = assign31520_e41684_d_n8;
        locals.var_phib_edge_dn9 = assign31520_e41684_d_n9;
        locals.var_phib_edge_dn10 = assign31520_e41684_d_n10;
        locals.var_phib_edge_dn11 = assign31520_e41684_d_n11;
        locals.var_phib_edge_dn12 = assign31520_e41684_d_n12;
        locals.var_phib_edge_dn13 = assign31520_e41684_d_n13;
        locals.var_phib_edge_dn14 = assign31520_e41684_d_n14;
        locals.var_phib_edge_rv = 0.0;

        let (assign31530_e41696, assign31530_e41696_d_n0, assign31530_e41696_d_n2, assign31530_e41696_d_n3, assign31530_e41696_d_n4, assign31530_e41696_d_n5, assign31530_e41696_d_n6, assign31530_e41696_d_n7, assign31530_e41696_d_n8, assign31530_e41696_d_n9, assign31530_e41696_d_n10, assign31530_e41696_d_n11, assign31530_e41696_d_n12, assign31530_e41696_d_n13, assign31530_e41696_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31530_e41689: f64 = (locals.var_vt * locals.var_phib_edge);
        let assign31530_e41690: f64 = (0.4 + assign31530_e41689);
        let assign31530_e41692: f64 = (assign31530_e41690 + locals.var_phin_i);
        let assign31530_e41694: f64 = (assign31530_e41692).max(0.4);
        (assign31530_e41694, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn0) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn2) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn3) } else { 0.0 }, if assign31530_e41692 >= 0.4 { ((locals.var_vt_dn4 * locals.var_phib_edge) + (locals.var_vt * locals.var_phib_edge_dn4)) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn5) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn6) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn7) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn8) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn9) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn10) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn11) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn12) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn13) } else { 0.0 }, if assign31530_e41692 >= 0.4 { (locals.var_vt * locals.var_phib_edge_dn14) } else { 0.0 },)
    } else {
        (locals.var_phist, locals.var_phist_dn0, locals.var_phist_dn2, locals.var_phist_dn3, locals.var_phist_dn4, locals.var_phist_dn5, locals.var_phist_dn6, locals.var_phist_dn7, locals.var_phist_dn8, locals.var_phist_dn9, locals.var_phist_dn10, locals.var_phist_dn11, locals.var_phist_dn12, locals.var_phist_dn13, locals.var_phist_dn14,)
    }
};
        locals.var_phist = assign31530_e41696;
        locals.var_phist_dn0 = assign31530_e41696_d_n0;
        locals.var_phist_dn2 = assign31530_e41696_d_n2;
        locals.var_phist_dn3 = assign31530_e41696_d_n3;
        locals.var_phist_dn4 = assign31530_e41696_d_n4;
        locals.var_phist_dn5 = assign31530_e41696_d_n5;
        locals.var_phist_dn6 = assign31530_e41696_d_n6;
        locals.var_phist_dn7 = assign31530_e41696_d_n7;
        locals.var_phist_dn8 = assign31530_e41696_d_n8;
        locals.var_phist_dn9 = assign31530_e41696_d_n9;
        locals.var_phist_dn10 = assign31530_e41696_d_n10;
        locals.var_phist_dn11 = assign31530_e41696_d_n11;
        locals.var_phist_dn12 = assign31530_e41696_d_n12;
        locals.var_phist_dn13 = assign31530_e41696_d_n13;
        locals.var_phist_dn14 = assign31530_e41696_d_n14;
        locals.var_phist_rv = 0.0;

        let (assign31540_e41707, assign31540_e41707_d_n0, assign31540_e41707_d_n2, assign31540_e41707_d_n3, assign31540_e41707_d_n4, assign31540_e41707_d_n5, assign31540_e41707_d_n6, assign31540_e41707_d_n7, assign31540_e41707_d_n8, assign31540_e41707_d_n9, assign31540_e41707_d_n10, assign31540_e41707_d_n11, assign31540_e41707_d_n12, assign31540_e41707_d_n13, assign31540_e41707_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31540_e41700: f64 = (2.0 * locals.var_epssi);
        let assign31540_e41703: f64 = (1.60219e-19 * locals.var_ndepedge_i);
        let assign31540_e41704: f64 = (assign31540_e41700 / assign31540_e41703);
        let assign31540_e41705: f64 = (assign31540_e41704).sqrt();
        (assign31540_e41705, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1dep, locals.var_t1dep_dn0, locals.var_t1dep_dn2, locals.var_t1dep_dn3, locals.var_t1dep_dn4, locals.var_t1dep_dn5, locals.var_t1dep_dn6, locals.var_t1dep_dn7, locals.var_t1dep_dn8, locals.var_t1dep_dn9, locals.var_t1dep_dn10, locals.var_t1dep_dn11, locals.var_t1dep_dn12, locals.var_t1dep_dn13, locals.var_t1dep_dn14,)
    }
};
        locals.var_t1dep = assign31540_e41707;
        locals.var_t1dep_dn0 = assign31540_e41707_d_n0;
        locals.var_t1dep_dn2 = assign31540_e41707_d_n2;
        locals.var_t1dep_dn3 = assign31540_e41707_d_n3;
        locals.var_t1dep_dn4 = assign31540_e41707_d_n4;
        locals.var_t1dep_dn5 = assign31540_e41707_d_n5;
        locals.var_t1dep_dn6 = assign31540_e41707_d_n6;
        locals.var_t1dep_dn7 = assign31540_e41707_d_n7;
        locals.var_t1dep_dn8 = assign31540_e41707_d_n8;
        locals.var_t1dep_dn9 = assign31540_e41707_d_n9;
        locals.var_t1dep_dn10 = assign31540_e41707_d_n10;
        locals.var_t1dep_dn11 = assign31540_e41707_d_n11;
        locals.var_t1dep_dn12 = assign31540_e41707_d_n12;
        locals.var_t1dep_dn13 = assign31540_e41707_d_n13;
        locals.var_t1dep_dn14 = assign31540_e41707_d_n14;
        locals.var_t1dep_rv = 0.0;

        let (assign31550_e41784, assign31550_e41784_d_n4,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31550_e41714: f64 = (locals.var_tratio - 1.0);
        let assign31550_e41715: f64 = (locals.var_tnfactoredge_i * assign31550_e41714);
        let assign31550_e41716: f64 = (1.0 + assign31550_e41715);
        let assign31550_e41718: f64 = (-10000.0);
        let assign31550_e41720: f64 = (assign31550_e41718 * 0.001);
        let (assign31550_e41781, assign31550_e41781_d_n4,) = {
            if (!(assign31550_e41716 < assign31550_e41720)) {
                let assign31550_e41728: f64 = (locals.var_tratio - 1.0);
                let assign31550_e41729: f64 = (locals.var_tnfactoredge_i * assign31550_e41728);
                let assign31550_e41730: f64 = (1.0 + assign31550_e41729);
                let assign31550_e41735: f64 = (locals.var_tratio - 1.0);
                let assign31550_e41736: f64 = (locals.var_tnfactoredge_i * assign31550_e41735);
                let assign31550_e41737: f64 = (1.0 + assign31550_e41736);
                let assign31550_e41742: f64 = (locals.var_tratio - 1.0);
                let assign31550_e41743: f64 = (locals.var_tnfactoredge_i * assign31550_e41742);
                let assign31550_e41744: f64 = (1.0 + assign31550_e41743);
                let assign31550_e41745: f64 = (assign31550_e41737 * assign31550_e41744);
                let assign31550_e41748: f64 = (4.0 * 0.001);
                let assign31550_e41750: f64 = (assign31550_e41748 * 0.001);
                let assign31550_e41751: f64 = (assign31550_e41745 + assign31550_e41750);
                let assign31550_e41752: f64 = (assign31550_e41751).sqrt();
                let assign31550_e41753: f64 = (assign31550_e41730 + assign31550_e41752);
                let assign31550_e41754: f64 = (0.5 * assign31550_e41753);
                (assign31550_e41754, (0.5 * ((locals.var_tnfactoredge_i * locals.var_tratio_dn4) + ((((locals.var_tnfactoredge_i * locals.var_tratio_dn4) * assign31550_e41744) + (assign31550_e41737 * (locals.var_tnfactoredge_i * locals.var_tratio_dn4))) / (2.0 * assign31550_e41752)))),)
            } else {
                let assign31550_e41759: f64 = (locals.var_tratio - 1.0);
                let assign31550_e41760: f64 = (locals.var_tnfactoredge_i * assign31550_e41759);
                let assign31550_e41761: f64 = (1.0 + assign31550_e41760);
                let assign31550_e41763: f64 = (-10000.0);
                let assign31550_e41765: f64 = (assign31550_e41763 * 0.001);
                let (assign31550_e41780, assign31550_e41780_d_n4,) = {
                    if (assign31550_e41761 < assign31550_e41765) {
                        let assign31550_e41768: f64 = (-0.001);
                        let assign31550_e41770: f64 = (assign31550_e41768 * 0.001);
                        let assign31550_e41775: f64 = (locals.var_tratio - 1.0);
                        let assign31550_e41776: f64 = (locals.var_tnfactoredge_i * assign31550_e41775);
                        let assign31550_e41777: f64 = (1.0 + assign31550_e41776);
                        let assign31550_e41778: f64 = (assign31550_e41770 / assign31550_e41777);
                        (assign31550_e41778, (-((assign31550_e41770 * (locals.var_tnfactoredge_i * locals.var_tratio_dn4)) / (assign31550_e41777 * assign31550_e41777))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign31550_e41780, assign31550_e41780_d_n4,)
            }
        };
        let assign31550_e41782: f64 = (locals.var_nfactoredge_i * assign31550_e41781);
        (assign31550_e41782, (locals.var_nfactoredge_i * assign31550_e41781_d_n4),)
    } else {
        (locals.var_nfactoredge_t, locals.var_nfactoredge_t_dn4,)
    }
};
        locals.var_nfactoredge_t = assign31550_e41784;
        locals.var_nfactoredge_t_dn4 = assign31550_e41784_d_n4;
        locals.var_nfactoredge_t_rv = 0.0;

        let (assign31560_e41796, assign31560_e41796_d_n0, assign31560_e41796_d_n2, assign31560_e41796_d_n3, assign31560_e41796_d_n4, assign31560_e41796_d_n5, assign31560_e41796_d_n6, assign31560_e41796_d_n7, assign31560_e41796_d_n8, assign31560_e41796_d_n9, assign31560_e41796_d_n10, assign31560_e41796_d_n11, assign31560_e41796_d_n12, assign31560_e41796_d_n13, assign31560_e41796_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31560_e41791: f64 = (locals.var_tratio - 1.0);
        let assign31560_e41792: f64 = (locals.var_teta0edge_i * assign31560_e41791);
        let assign31560_e41793: f64 = (1.0 + assign31560_e41792);
        let assign31560_e41794: f64 = (locals.var_eta0edge_i * assign31560_e41793);
        (assign31560_e41794, (locals.var_eta0edge_i_dn0 * assign31560_e41793), (locals.var_eta0edge_i_dn2 * assign31560_e41793), (locals.var_eta0edge_i_dn3 * assign31560_e41793), ((locals.var_eta0edge_i_dn4 * assign31560_e41793) + (locals.var_eta0edge_i * (locals.var_teta0edge_i * locals.var_tratio_dn4))), (locals.var_eta0edge_i_dn5 * assign31560_e41793), (locals.var_eta0edge_i_dn6 * assign31560_e41793), (locals.var_eta0edge_i_dn7 * assign31560_e41793), (locals.var_eta0edge_i_dn8 * assign31560_e41793), (locals.var_eta0edge_i_dn9 * assign31560_e41793), (locals.var_eta0edge_i_dn10 * assign31560_e41793), (locals.var_eta0edge_i_dn11 * assign31560_e41793), (locals.var_eta0edge_i_dn12 * assign31560_e41793), (locals.var_eta0edge_i_dn13 * assign31560_e41793), (locals.var_eta0edge_i_dn14 * assign31560_e41793),)
    } else {
        (locals.var_eta0edge_t, locals.var_eta0edge_t_dn0, locals.var_eta0edge_t_dn2, locals.var_eta0edge_t_dn3, locals.var_eta0edge_t_dn4, locals.var_eta0edge_t_dn5, locals.var_eta0edge_t_dn6, locals.var_eta0edge_t_dn7, locals.var_eta0edge_t_dn8, locals.var_eta0edge_t_dn9, locals.var_eta0edge_t_dn10, locals.var_eta0edge_t_dn11, locals.var_eta0edge_t_dn12, locals.var_eta0edge_t_dn13, locals.var_eta0edge_t_dn14,)
    }
};
        locals.var_eta0edge_t = assign31560_e41796;
        locals.var_eta0edge_t_dn0 = assign31560_e41796_d_n0;
        locals.var_eta0edge_t_dn2 = assign31560_e41796_d_n2;
        locals.var_eta0edge_t_dn3 = assign31560_e41796_d_n3;
        locals.var_eta0edge_t_dn4 = assign31560_e41796_d_n4;
        locals.var_eta0edge_t_dn5 = assign31560_e41796_d_n5;
        locals.var_eta0edge_t_dn6 = assign31560_e41796_d_n6;
        locals.var_eta0edge_t_dn7 = assign31560_e41796_d_n7;
        locals.var_eta0edge_t_dn8 = assign31560_e41796_d_n8;
        locals.var_eta0edge_t_dn9 = assign31560_e41796_d_n9;
        locals.var_eta0edge_t_dn10 = assign31560_e41796_d_n10;
        locals.var_eta0edge_t_dn11 = assign31560_e41796_d_n11;
        locals.var_eta0edge_t_dn12 = assign31560_e41796_d_n12;
        locals.var_eta0edge_t_dn13 = assign31560_e41796_d_n13;
        locals.var_eta0edge_t_dn14 = assign31560_e41796_d_n14;
        locals.var_eta0edge_t_rv = 0.0;

        let assign31570_e41802: f64 = (locals.var_phist - locals.var_vbsx);
        let assign31570_e41804: f64 = (-2500.0);
        let assign31570_e41806: f64 = (assign31570_e41804 * 0.1);
        let assign31570_e41808: f64 = if ((0.05 == 0.0) && (assign31570_e41802 < assign31570_e41806)) { 1.0 } else { 0.0 };
        locals.var_guard731 = assign31570_e41808;
        locals.var_guard731_rv = 0.0;

        let (assign31580_e41823, assign31580_e41823_d_n0, assign31580_e41823_d_n2, assign31580_e41823_d_n3, assign31580_e41823_d_n4, assign31580_e41823_d_n5, assign31580_e41823_d_n6, assign31580_e41823_d_n7, assign31580_e41823_d_n8, assign31580_e41823_d_n9, assign31580_e41823_d_n10, assign31580_e41823_d_n11, assign31580_e41823_d_n12, assign31580_e41823_d_n13, assign31580_e41823_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard731 != 0.0)) {
        let assign31580_e41813: f64 = (-0.1);
        let assign31580_e41815: f64 = (assign31580_e41813 * 0.1);
        let assign31580_e41819: f64 = (locals.var_phist - locals.var_vbsx);
        let assign31580_e41820: f64 = (16.0 * assign31580_e41819);
        let assign31580_e41821: f64 = (assign31580_e41815 / assign31580_e41820);
        (assign31580_e41821, (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn0 - locals.var_vbsx_dn0))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn2 - locals.var_vbsx_dn2))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn3 - locals.var_vbsx_dn3))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn4 - locals.var_vbsx_dn4))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn5 - locals.var_vbsx_dn5))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn6 - locals.var_vbsx_dn6))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn7 - locals.var_vbsx_dn7))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn8 - locals.var_vbsx_dn8))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn9 - locals.var_vbsx_dn9))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn10 - locals.var_vbsx_dn10))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn11 - locals.var_vbsx_dn11))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn12 - locals.var_vbsx_dn12))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn13 - locals.var_vbsx_dn13))) / (assign31580_e41820 * assign31580_e41820))), (-((assign31580_e41815 * (16.0 * (locals.var_phist_dn14 - locals.var_vbsx_dn14))) / (assign31580_e41820 * assign31580_e41820))),)
    } else {
        (locals.var_phistvbs, locals.var_phistvbs_dn0, locals.var_phistvbs_dn2, locals.var_phistvbs_dn3, locals.var_phistvbs_dn4, locals.var_phistvbs_dn5, locals.var_phistvbs_dn6, locals.var_phistvbs_dn7, locals.var_phistvbs_dn8, locals.var_phistvbs_dn9, locals.var_phistvbs_dn10, locals.var_phistvbs_dn11, locals.var_phistvbs_dn12, locals.var_phistvbs_dn13, locals.var_phistvbs_dn14,)
    }
};
        locals.var_phistvbs = assign31580_e41823;
        locals.var_phistvbs_dn0 = assign31580_e41823_d_n0;
        locals.var_phistvbs_dn2 = assign31580_e41823_d_n2;
        locals.var_phistvbs_dn3 = assign31580_e41823_d_n3;
        locals.var_phistvbs_dn4 = assign31580_e41823_d_n4;
        locals.var_phistvbs_dn5 = assign31580_e41823_d_n5;
        locals.var_phistvbs_dn6 = assign31580_e41823_d_n6;
        locals.var_phistvbs_dn7 = assign31580_e41823_d_n7;
        locals.var_phistvbs_dn8 = assign31580_e41823_d_n8;
        locals.var_phistvbs_dn9 = assign31580_e41823_d_n9;
        locals.var_phistvbs_dn10 = assign31580_e41823_d_n10;
        locals.var_phistvbs_dn11 = assign31580_e41823_d_n11;
        locals.var_phistvbs_dn12 = assign31580_e41823_d_n12;
        locals.var_phistvbs_dn13 = assign31580_e41823_d_n13;
        locals.var_phistvbs_dn14 = assign31580_e41823_d_n14;
        locals.var_phistvbs_rv = 0.0;

        let (assign31590_e41855, assign31590_e41855_d_n0, assign31590_e41855_d_n2, assign31590_e41855_d_n3, assign31590_e41855_d_n4, assign31590_e41855_d_n5, assign31590_e41855_d_n6, assign31590_e41855_d_n7, assign31590_e41855_d_n8, assign31590_e41855_d_n9, assign31590_e41855_d_n10, assign31590_e41855_d_n11, assign31590_e41855_d_n12, assign31590_e41855_d_n13, assign31590_e41855_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard731 == 0.0)) {
        let assign31590_e41831: f64 = (locals.var_phist - locals.var_vbsx);
        let assign31590_e41833: f64 = (assign31590_e41831 + 0.05);
        let assign31590_e41836: f64 = (locals.var_phist - locals.var_vbsx);
        let assign31590_e41838: f64 = (assign31590_e41836 - 0.05);
        let assign31590_e41841: f64 = (locals.var_phist - locals.var_vbsx);
        let assign31590_e41843: f64 = (assign31590_e41841 - 0.05);
        let assign31590_e41844: f64 = (assign31590_e41838 * assign31590_e41843);
        let assign31590_e41847: f64 = (0.25 * 0.1);
        let assign31590_e41849: f64 = (assign31590_e41847 * 0.1);
        let assign31590_e41850: f64 = (assign31590_e41844 + assign31590_e41849);
        let assign31590_e41851: f64 = (assign31590_e41850).sqrt();
        let assign31590_e41852: f64 = (assign31590_e41833 + assign31590_e41851);
        let assign31590_e41853: f64 = (0.5 * assign31590_e41852);
        (assign31590_e41853, (0.5 * ((locals.var_phist_dn0 - locals.var_vbsx_dn0) + ((((locals.var_phist_dn0 - locals.var_vbsx_dn0) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn0 - locals.var_vbsx_dn0))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn2 - locals.var_vbsx_dn2) + ((((locals.var_phist_dn2 - locals.var_vbsx_dn2) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn2 - locals.var_vbsx_dn2))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn3 - locals.var_vbsx_dn3) + ((((locals.var_phist_dn3 - locals.var_vbsx_dn3) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn3 - locals.var_vbsx_dn3))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn4 - locals.var_vbsx_dn4) + ((((locals.var_phist_dn4 - locals.var_vbsx_dn4) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn4 - locals.var_vbsx_dn4))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn5 - locals.var_vbsx_dn5) + ((((locals.var_phist_dn5 - locals.var_vbsx_dn5) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn5 - locals.var_vbsx_dn5))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn6 - locals.var_vbsx_dn6) + ((((locals.var_phist_dn6 - locals.var_vbsx_dn6) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn6 - locals.var_vbsx_dn6))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn7 - locals.var_vbsx_dn7) + ((((locals.var_phist_dn7 - locals.var_vbsx_dn7) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn7 - locals.var_vbsx_dn7))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn8 - locals.var_vbsx_dn8) + ((((locals.var_phist_dn8 - locals.var_vbsx_dn8) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn8 - locals.var_vbsx_dn8))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn9 - locals.var_vbsx_dn9) + ((((locals.var_phist_dn9 - locals.var_vbsx_dn9) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn9 - locals.var_vbsx_dn9))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn10 - locals.var_vbsx_dn10) + ((((locals.var_phist_dn10 - locals.var_vbsx_dn10) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn10 - locals.var_vbsx_dn10))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn11 - locals.var_vbsx_dn11) + ((((locals.var_phist_dn11 - locals.var_vbsx_dn11) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn11 - locals.var_vbsx_dn11))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn12 - locals.var_vbsx_dn12) + ((((locals.var_phist_dn12 - locals.var_vbsx_dn12) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn12 - locals.var_vbsx_dn12))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn13 - locals.var_vbsx_dn13) + ((((locals.var_phist_dn13 - locals.var_vbsx_dn13) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn13 - locals.var_vbsx_dn13))) / (2.0 * assign31590_e41851)))), (0.5 * ((locals.var_phist_dn14 - locals.var_vbsx_dn14) + ((((locals.var_phist_dn14 - locals.var_vbsx_dn14) * assign31590_e41843) + (assign31590_e41838 * (locals.var_phist_dn14 - locals.var_vbsx_dn14))) / (2.0 * assign31590_e41851)))),)
    } else {
        (locals.var_phistvbs, locals.var_phistvbs_dn0, locals.var_phistvbs_dn2, locals.var_phistvbs_dn3, locals.var_phistvbs_dn4, locals.var_phistvbs_dn5, locals.var_phistvbs_dn6, locals.var_phistvbs_dn7, locals.var_phistvbs_dn8, locals.var_phistvbs_dn9, locals.var_phistvbs_dn10, locals.var_phistvbs_dn11, locals.var_phistvbs_dn12, locals.var_phistvbs_dn13, locals.var_phistvbs_dn14,)
    }
};
        locals.var_phistvbs = assign31590_e41855;
        locals.var_phistvbs_dn0 = assign31590_e41855_d_n0;
        locals.var_phistvbs_dn2 = assign31590_e41855_d_n2;
        locals.var_phistvbs_dn3 = assign31590_e41855_d_n3;
        locals.var_phistvbs_dn4 = assign31590_e41855_d_n4;
        locals.var_phistvbs_dn5 = assign31590_e41855_d_n5;
        locals.var_phistvbs_dn6 = assign31590_e41855_d_n6;
        locals.var_phistvbs_dn7 = assign31590_e41855_d_n7;
        locals.var_phistvbs_dn8 = assign31590_e41855_d_n8;
        locals.var_phistvbs_dn9 = assign31590_e41855_d_n9;
        locals.var_phistvbs_dn10 = assign31590_e41855_d_n10;
        locals.var_phistvbs_dn11 = assign31590_e41855_d_n11;
        locals.var_phistvbs_dn12 = assign31590_e41855_d_n12;
        locals.var_phistvbs_dn13 = assign31590_e41855_d_n13;
        locals.var_phistvbs_dn14 = assign31590_e41855_d_n14;
        locals.var_phistvbs_rv = 0.0;

        let (assign31600_e41860, assign31600_e41860_d_n0, assign31600_e41860_d_n2, assign31600_e41860_d_n3, assign31600_e41860_d_n4, assign31600_e41860_d_n5, assign31600_e41860_d_n6, assign31600_e41860_d_n7, assign31600_e41860_d_n8, assign31600_e41860_d_n9, assign31600_e41860_d_n10, assign31600_e41860_d_n11, assign31600_e41860_d_n12, assign31600_e41860_d_n13, assign31600_e41860_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31600_e41858: f64 = (locals.var_phistvbs).sqrt();
        (assign31600_e41858, (locals.var_phistvbs_dn0 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn2 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn3 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn4 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn5 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn6 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn7 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn8 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn9 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn10 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn11 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn12 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn13 / (2.0 * assign31600_e41858)), (locals.var_phistvbs_dn14 / (2.0 * assign31600_e41858)),)
    } else {
        (locals.var_sqrtphistvbs, locals.var_sqrtphistvbs_dn0, locals.var_sqrtphistvbs_dn2, locals.var_sqrtphistvbs_dn3, locals.var_sqrtphistvbs_dn4, locals.var_sqrtphistvbs_dn5, locals.var_sqrtphistvbs_dn6, locals.var_sqrtphistvbs_dn7, locals.var_sqrtphistvbs_dn8, locals.var_sqrtphistvbs_dn9, locals.var_sqrtphistvbs_dn10, locals.var_sqrtphistvbs_dn11, locals.var_sqrtphistvbs_dn12, locals.var_sqrtphistvbs_dn13, locals.var_sqrtphistvbs_dn14,)
    }
};
        locals.var_sqrtphistvbs = assign31600_e41860;
        locals.var_sqrtphistvbs_dn0 = assign31600_e41860_d_n0;
        locals.var_sqrtphistvbs_dn2 = assign31600_e41860_d_n2;
        locals.var_sqrtphistvbs_dn3 = assign31600_e41860_d_n3;
        locals.var_sqrtphistvbs_dn4 = assign31600_e41860_d_n4;
        locals.var_sqrtphistvbs_dn5 = assign31600_e41860_d_n5;
        locals.var_sqrtphistvbs_dn6 = assign31600_e41860_d_n6;
        locals.var_sqrtphistvbs_dn7 = assign31600_e41860_d_n7;
        locals.var_sqrtphistvbs_dn8 = assign31600_e41860_d_n8;
        locals.var_sqrtphistvbs_dn9 = assign31600_e41860_d_n9;
        locals.var_sqrtphistvbs_dn10 = assign31600_e41860_d_n10;
        locals.var_sqrtphistvbs_dn11 = assign31600_e41860_d_n11;
        locals.var_sqrtphistvbs_dn12 = assign31600_e41860_d_n12;
        locals.var_sqrtphistvbs_dn13 = assign31600_e41860_d_n13;
        locals.var_sqrtphistvbs_dn14 = assign31600_e41860_d_n14;
        locals.var_sqrtphistvbs_rv = 0.0;

        let (assign31610_e41866, assign31610_e41866_d_n0, assign31610_e41866_d_n2, assign31610_e41866_d_n3, assign31610_e41866_d_n4, assign31610_e41866_d_n5, assign31610_e41866_d_n6, assign31610_e41866_d_n7, assign31610_e41866_d_n8, assign31610_e41866_d_n9, assign31610_e41866_d_n10, assign31610_e41866_d_n11, assign31610_e41866_d_n12, assign31610_e41866_d_n13, assign31610_e41866_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31610_e41864: f64 = (locals.var_t1dep * locals.var_sqrtphistvbs);
        (assign31610_e41864, ((locals.var_t1dep_dn0 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn0)), ((locals.var_t1dep_dn2 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn2)), ((locals.var_t1dep_dn3 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn3)), ((locals.var_t1dep_dn4 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn4)), ((locals.var_t1dep_dn5 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn5)), ((locals.var_t1dep_dn6 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn6)), ((locals.var_t1dep_dn7 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn7)), ((locals.var_t1dep_dn8 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn8)), ((locals.var_t1dep_dn9 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn9)), ((locals.var_t1dep_dn10 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn10)), ((locals.var_t1dep_dn11 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn11)), ((locals.var_t1dep_dn12 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn12)), ((locals.var_t1dep_dn13 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn13)), ((locals.var_t1dep_dn14 * locals.var_sqrtphistvbs) + (locals.var_t1dep * locals.var_sqrtphistvbs_dn14)),)
    } else {
        (locals.var_xdep, locals.var_xdep_dn0, locals.var_xdep_dn2, locals.var_xdep_dn3, locals.var_xdep_dn4, locals.var_xdep_dn5, locals.var_xdep_dn6, locals.var_xdep_dn7, locals.var_xdep_dn8, locals.var_xdep_dn9, locals.var_xdep_dn10, locals.var_xdep_dn11, locals.var_xdep_dn12, locals.var_xdep_dn13, locals.var_xdep_dn14,)
    }
};
        locals.var_xdep = assign31610_e41866;
        locals.var_xdep_dn0 = assign31610_e41866_d_n0;
        locals.var_xdep_dn2 = assign31610_e41866_d_n2;
        locals.var_xdep_dn3 = assign31610_e41866_d_n3;
        locals.var_xdep_dn4 = assign31610_e41866_d_n4;
        locals.var_xdep_dn5 = assign31610_e41866_d_n5;
        locals.var_xdep_dn6 = assign31610_e41866_d_n6;
        locals.var_xdep_dn7 = assign31610_e41866_d_n7;
        locals.var_xdep_dn8 = assign31610_e41866_d_n8;
        locals.var_xdep_dn9 = assign31610_e41866_d_n9;
        locals.var_xdep_dn10 = assign31610_e41866_d_n10;
        locals.var_xdep_dn11 = assign31610_e41866_d_n11;
        locals.var_xdep_dn12 = assign31610_e41866_d_n12;
        locals.var_xdep_dn13 = assign31610_e41866_d_n13;
        locals.var_xdep_dn14 = assign31610_e41866_d_n14;
        locals.var_xdep_rv = 0.0;

        let (assign31620_e41872, assign31620_e41872_d_n0, assign31620_e41872_d_n2, assign31620_e41872_d_n3, assign31620_e41872_d_n4, assign31620_e41872_d_n5, assign31620_e41872_d_n6, assign31620_e41872_d_n7, assign31620_e41872_d_n8, assign31620_e41872_d_n9, assign31620_e41872_d_n10, assign31620_e41872_d_n11, assign31620_e41872_d_n12, assign31620_e41872_d_n13, assign31620_e41872_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31620_e41870: f64 = (locals.var_epssi / locals.var_xdep);
        (assign31620_e41870, (-((locals.var_epssi * locals.var_xdep_dn0) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn2) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn3) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn4) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn5) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn6) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn7) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn8) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn9) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn10) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn11) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn12) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn13) / (locals.var_xdep * locals.var_xdep))), (-((locals.var_epssi * locals.var_xdep_dn14) / (locals.var_xdep * locals.var_xdep))),)
    } else {
        (locals.var_cdep, locals.var_cdep_dn0, locals.var_cdep_dn2, locals.var_cdep_dn3, locals.var_cdep_dn4, locals.var_cdep_dn5, locals.var_cdep_dn6, locals.var_cdep_dn7, locals.var_cdep_dn8, locals.var_cdep_dn9, locals.var_cdep_dn10, locals.var_cdep_dn11, locals.var_cdep_dn12, locals.var_cdep_dn13, locals.var_cdep_dn14,)
    }
};
        locals.var_cdep = assign31620_e41872;
        locals.var_cdep_dn0 = assign31620_e41872_d_n0;
        locals.var_cdep_dn2 = assign31620_e41872_d_n2;
        locals.var_cdep_dn3 = assign31620_e41872_d_n3;
        locals.var_cdep_dn4 = assign31620_e41872_d_n4;
        locals.var_cdep_dn5 = assign31620_e41872_d_n5;
        locals.var_cdep_dn6 = assign31620_e41872_d_n6;
        locals.var_cdep_dn7 = assign31620_e41872_d_n7;
        locals.var_cdep_dn8 = assign31620_e41872_d_n8;
        locals.var_cdep_dn9 = assign31620_e41872_d_n9;
        locals.var_cdep_dn10 = assign31620_e41872_d_n10;
        locals.var_cdep_dn11 = assign31620_e41872_d_n11;
        locals.var_cdep_dn12 = assign31620_e41872_d_n12;
        locals.var_cdep_dn13 = assign31620_e41872_d_n13;
        locals.var_cdep_dn14 = assign31620_e41872_d_n14;
        locals.var_cdep_rv = 0.0;

        let (assign31630_e41886, assign31630_e41886_d_n0, assign31630_e41886_d_n2, assign31630_e41886_d_n3, assign31630_e41886_d_n4, assign31630_e41886_d_n5, assign31630_e41886_d_n6, assign31630_e41886_d_n7, assign31630_e41886_d_n8, assign31630_e41886_d_n9, assign31630_e41886_d_n10, assign31630_e41886_d_n11, assign31630_e41886_d_n12, assign31630_e41886_d_n13, assign31630_e41886_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31630_e41876: f64 = (locals.var_citedge_i + locals.var_nfactoredge_t);
        let assign31630_e41879: f64 = (locals.var_cdscdedge_i * locals.var_vdsx);
        let assign31630_e41880: f64 = (assign31630_e41876 + assign31630_e41879);
        let assign31630_e41883: f64 = (locals.var_cdscbedge_i * locals.var_vbsx);
        let assign31630_e41884: f64 = (assign31630_e41880 - assign31630_e41883);
        (assign31630_e41884, ((locals.var_cdscdedge_i * locals.var_vdsx_dn0) - (locals.var_cdscbedge_i * locals.var_vbsx_dn0)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn2) - (locals.var_cdscbedge_i * locals.var_vbsx_dn2)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn3) - (locals.var_cdscbedge_i * locals.var_vbsx_dn3)), ((locals.var_nfactoredge_t_dn4 + (locals.var_cdscdedge_i * locals.var_vdsx_dn4)) - (locals.var_cdscbedge_i * locals.var_vbsx_dn4)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn5) - (locals.var_cdscbedge_i * locals.var_vbsx_dn5)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn6) - (locals.var_cdscbedge_i * locals.var_vbsx_dn6)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn7) - (locals.var_cdscbedge_i * locals.var_vbsx_dn7)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn8) - (locals.var_cdscbedge_i * locals.var_vbsx_dn8)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn9) - (locals.var_cdscbedge_i * locals.var_vbsx_dn9)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn10) - (locals.var_cdscbedge_i * locals.var_vbsx_dn10)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn11) - (locals.var_cdscbedge_i * locals.var_vbsx_dn11)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn12) - (locals.var_cdscbedge_i * locals.var_vbsx_dn12)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn13) - (locals.var_cdscbedge_i * locals.var_vbsx_dn13)), ((locals.var_cdscdedge_i * locals.var_vdsx_dn14) - (locals.var_cdscbedge_i * locals.var_vbsx_dn14)),)
    } else {
        (locals.var_cdsc, locals.var_cdsc_dn0, locals.var_cdsc_dn2, locals.var_cdsc_dn3, locals.var_cdsc_dn4, locals.var_cdsc_dn5, locals.var_cdsc_dn6, locals.var_cdsc_dn7, locals.var_cdsc_dn8, locals.var_cdsc_dn9, locals.var_cdsc_dn10, locals.var_cdsc_dn11, locals.var_cdsc_dn12, locals.var_cdsc_dn13, locals.var_cdsc_dn14,)
    }
};
        locals.var_cdsc = assign31630_e41886;
        locals.var_cdsc_dn0 = assign31630_e41886_d_n0;
        locals.var_cdsc_dn2 = assign31630_e41886_d_n2;
        locals.var_cdsc_dn3 = assign31630_e41886_d_n3;
        locals.var_cdsc_dn4 = assign31630_e41886_d_n4;
        locals.var_cdsc_dn5 = assign31630_e41886_d_n5;
        locals.var_cdsc_dn6 = assign31630_e41886_d_n6;
        locals.var_cdsc_dn7 = assign31630_e41886_d_n7;
        locals.var_cdsc_dn8 = assign31630_e41886_d_n8;
        locals.var_cdsc_dn9 = assign31630_e41886_d_n9;
        locals.var_cdsc_dn10 = assign31630_e41886_d_n10;
        locals.var_cdsc_dn11 = assign31630_e41886_d_n11;
        locals.var_cdsc_dn12 = assign31630_e41886_d_n12;
        locals.var_cdsc_dn13 = assign31630_e41886_d_n13;
        locals.var_cdsc_dn14 = assign31630_e41886_d_n14;
        locals.var_cdsc_rv = 0.0;

        let (assign31640_e41894, assign31640_e41894_d_n0, assign31640_e41894_d_n2, assign31640_e41894_d_n3, assign31640_e41894_d_n4, assign31640_e41894_d_n5, assign31640_e41894_d_n6, assign31640_e41894_d_n7, assign31640_e41894_d_n8, assign31640_e41894_d_n9, assign31640_e41894_d_n10, assign31640_e41894_d_n11, assign31640_e41894_d_n12, assign31640_e41894_d_n13, assign31640_e41894_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31640_e41891: f64 = (locals.var_cdsc / locals.var_cox);
        let assign31640_e41892: f64 = (1.0 + assign31640_e41891);
        (assign31640_e41892, (locals.var_cdsc_dn0 / locals.var_cox), (locals.var_cdsc_dn2 / locals.var_cox), (locals.var_cdsc_dn3 / locals.var_cox), (locals.var_cdsc_dn4 / locals.var_cox), (locals.var_cdsc_dn5 / locals.var_cox), (locals.var_cdsc_dn6 / locals.var_cox), (locals.var_cdsc_dn7 / locals.var_cox), (locals.var_cdsc_dn8 / locals.var_cox), (locals.var_cdsc_dn9 / locals.var_cox), (locals.var_cdsc_dn10 / locals.var_cox), (locals.var_cdsc_dn11 / locals.var_cox), (locals.var_cdsc_dn12 / locals.var_cox), (locals.var_cdsc_dn13 / locals.var_cox), (locals.var_cdsc_dn14 / locals.var_cox),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign31640_e41894;
        locals.var_t1_dn0 = assign31640_e41894_d_n0;
        locals.var_t1_dn2 = assign31640_e41894_d_n2;
        locals.var_t1_dn3 = assign31640_e41894_d_n3;
        locals.var_t1_dn4 = assign31640_e41894_d_n4;
        locals.var_t1_dn5 = assign31640_e41894_d_n5;
        locals.var_t1_dn6 = assign31640_e41894_d_n6;
        locals.var_t1_dn7 = assign31640_e41894_d_n7;
        locals.var_t1_dn8 = assign31640_e41894_d_n8;
        locals.var_t1_dn9 = assign31640_e41894_d_n9;
        locals.var_t1_dn10 = assign31640_e41894_d_n10;
        locals.var_t1_dn11 = assign31640_e41894_d_n11;
        locals.var_t1_dn12 = assign31640_e41894_d_n12;
        locals.var_t1_dn13 = assign31640_e41894_d_n13;
        locals.var_t1_dn14 = assign31640_e41894_d_n14;
        locals.var_t1_rv = 0.0;

        let assign31650_e41900: f64 = (-2500.0);
        let assign31650_e41902: f64 = (assign31650_e41900 * 0.05);
        let assign31650_e41904: f64 = if ((1.0 == 0.0) && (locals.var_t1 < assign31650_e41902)) { 1.0 } else { 0.0 };
        locals.var_guard732 = assign31650_e41904;
        locals.var_guard732_rv = 0.0;

        let (assign31660_e41917, assign31660_e41917_d_n0, assign31660_e41917_d_n2, assign31660_e41917_d_n3, assign31660_e41917_d_n4, assign31660_e41917_d_n5, assign31660_e41917_d_n6, assign31660_e41917_d_n7, assign31660_e41917_d_n8, assign31660_e41917_d_n9, assign31660_e41917_d_n10, assign31660_e41917_d_n11, assign31660_e41917_d_n12, assign31660_e41917_d_n13, assign31660_e41917_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard732 != 0.0)) {
        let assign31660_e41909: f64 = (-0.05);
        let assign31660_e41911: f64 = (assign31660_e41909 * 0.05);
        let assign31660_e41914: f64 = (16.0 * locals.var_t1);
        let assign31660_e41915: f64 = (assign31660_e41911 / assign31660_e41914);
        (assign31660_e41915, (-((assign31660_e41911 * (16.0 * locals.var_t1_dn0)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn2)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn3)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn4)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn5)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn6)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn7)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn8)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn9)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn10)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn11)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn12)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn13)) / (assign31660_e41914 * assign31660_e41914))), (-((assign31660_e41911 * (16.0 * locals.var_t1_dn14)) / (assign31660_e41914 * assign31660_e41914))),)
    } else {
        (locals.var_n, locals.var_n_dn0, locals.var_n_dn2, locals.var_n_dn3, locals.var_n_dn4, locals.var_n_dn5, locals.var_n_dn6, locals.var_n_dn7, locals.var_n_dn8, locals.var_n_dn9, locals.var_n_dn10, locals.var_n_dn11, locals.var_n_dn12, locals.var_n_dn13, locals.var_n_dn14,)
    }
};
        locals.var_n = assign31660_e41917;
        locals.var_n_dn0 = assign31660_e41917_d_n0;
        locals.var_n_dn2 = assign31660_e41917_d_n2;
        locals.var_n_dn3 = assign31660_e41917_d_n3;
        locals.var_n_dn4 = assign31660_e41917_d_n4;
        locals.var_n_dn5 = assign31660_e41917_d_n5;
        locals.var_n_dn6 = assign31660_e41917_d_n6;
        locals.var_n_dn7 = assign31660_e41917_d_n7;
        locals.var_n_dn8 = assign31660_e41917_d_n8;
        locals.var_n_dn9 = assign31660_e41917_d_n9;
        locals.var_n_dn10 = assign31660_e41917_d_n10;
        locals.var_n_dn11 = assign31660_e41917_d_n11;
        locals.var_n_dn12 = assign31660_e41917_d_n12;
        locals.var_n_dn13 = assign31660_e41917_d_n13;
        locals.var_n_dn14 = assign31660_e41917_d_n14;
        locals.var_n_rv = 0.0;

        let (assign31670_e41943, assign31670_e41943_d_n0, assign31670_e41943_d_n2, assign31670_e41943_d_n3, assign31670_e41943_d_n4, assign31670_e41943_d_n5, assign31670_e41943_d_n6, assign31670_e41943_d_n7, assign31670_e41943_d_n8, assign31670_e41943_d_n9, assign31670_e41943_d_n10, assign31670_e41943_d_n11, assign31670_e41943_d_n12, assign31670_e41943_d_n13, assign31670_e41943_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard732 == 0.0)) {
        let assign31670_e41925: f64 = (locals.var_t1 + 1.0);
        let assign31670_e41928: f64 = (locals.var_t1 - 1.0);
        let assign31670_e41931: f64 = (locals.var_t1 - 1.0);
        let assign31670_e41932: f64 = (assign31670_e41928 * assign31670_e41931);
        let assign31670_e41935: f64 = (0.25 * 0.05);
        let assign31670_e41937: f64 = (assign31670_e41935 * 0.05);
        let assign31670_e41938: f64 = (assign31670_e41932 + assign31670_e41937);
        let assign31670_e41939: f64 = (assign31670_e41938).sqrt();
        let assign31670_e41940: f64 = (assign31670_e41925 + assign31670_e41939);
        let assign31670_e41941: f64 = (0.5 * assign31670_e41940);
        (assign31670_e41941, (0.5 * (locals.var_t1_dn0 + (((locals.var_t1_dn0 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn0)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn2 + (((locals.var_t1_dn2 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn2)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn3)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn4)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn5)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn6)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn7)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn8)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn9)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn10)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn11)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn12 + (((locals.var_t1_dn12 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn12)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn13 + (((locals.var_t1_dn13 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn13)) / (2.0 * assign31670_e41939)))), (0.5 * (locals.var_t1_dn14 + (((locals.var_t1_dn14 * assign31670_e41931) + (assign31670_e41928 * locals.var_t1_dn14)) / (2.0 * assign31670_e41939)))),)
    } else {
        (locals.var_n, locals.var_n_dn0, locals.var_n_dn2, locals.var_n_dn3, locals.var_n_dn4, locals.var_n_dn5, locals.var_n_dn6, locals.var_n_dn7, locals.var_n_dn8, locals.var_n_dn9, locals.var_n_dn10, locals.var_n_dn11, locals.var_n_dn12, locals.var_n_dn13, locals.var_n_dn14,)
    }
};
        locals.var_n = assign31670_e41943;
        locals.var_n_dn0 = assign31670_e41943_d_n0;
        locals.var_n_dn2 = assign31670_e41943_d_n2;
        locals.var_n_dn3 = assign31670_e41943_d_n3;
        locals.var_n_dn4 = assign31670_e41943_d_n4;
        locals.var_n_dn5 = assign31670_e41943_d_n5;
        locals.var_n_dn6 = assign31670_e41943_d_n6;
        locals.var_n_dn7 = assign31670_e41943_d_n7;
        locals.var_n_dn8 = assign31670_e41943_d_n8;
        locals.var_n_dn9 = assign31670_e41943_d_n9;
        locals.var_n_dn10 = assign31670_e41943_d_n10;
        locals.var_n_dn11 = assign31670_e41943_d_n11;
        locals.var_n_dn12 = assign31670_e41943_d_n12;
        locals.var_n_dn13 = assign31670_e41943_d_n13;
        locals.var_n_dn14 = assign31670_e41943_d_n14;
        locals.var_n_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_101(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign31680_e41949, assign31680_e41949_d_n0, assign31680_e41949_d_n2, assign31680_e41949_d_n3, assign31680_e41949_d_n4, assign31680_e41949_d_n5, assign31680_e41949_d_n6, assign31680_e41949_d_n7, assign31680_e41949_d_n8, assign31680_e41949_d_n9, assign31680_e41949_d_n10, assign31680_e41949_d_n11, assign31680_e41949_d_n12, assign31680_e41949_d_n13, assign31680_e41949_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31680_e41947: f64 = (locals.var_n * locals.var_vt);
        (assign31680_e41947, (locals.var_n_dn0 * locals.var_vt), (locals.var_n_dn2 * locals.var_vt), (locals.var_n_dn3 * locals.var_vt), ((locals.var_n_dn4 * locals.var_vt) + (locals.var_n * locals.var_vt_dn4)), (locals.var_n_dn5 * locals.var_vt), (locals.var_n_dn6 * locals.var_vt), (locals.var_n_dn7 * locals.var_vt), (locals.var_n_dn8 * locals.var_vt), (locals.var_n_dn9 * locals.var_vt), (locals.var_n_dn10 * locals.var_vt), (locals.var_n_dn11 * locals.var_vt), (locals.var_n_dn12 * locals.var_vt), (locals.var_n_dn13 * locals.var_vt), (locals.var_n_dn14 * locals.var_vt),)
    } else {
        (locals.var_nvt, locals.var_nvt_dn0, locals.var_nvt_dn2, locals.var_nvt_dn3, locals.var_nvt_dn4, locals.var_nvt_dn5, locals.var_nvt_dn6, locals.var_nvt_dn7, locals.var_nvt_dn8, locals.var_nvt_dn9, locals.var_nvt_dn10, locals.var_nvt_dn11, locals.var_nvt_dn12, locals.var_nvt_dn13, locals.var_nvt_dn14,)
    }
};
        locals.var_nvt = assign31680_e41949;
        locals.var_nvt_dn0 = assign31680_e41949_d_n0;
        locals.var_nvt_dn2 = assign31680_e41949_d_n2;
        locals.var_nvt_dn3 = assign31680_e41949_d_n3;
        locals.var_nvt_dn4 = assign31680_e41949_d_n4;
        locals.var_nvt_dn5 = assign31680_e41949_d_n5;
        locals.var_nvt_dn6 = assign31680_e41949_d_n6;
        locals.var_nvt_dn7 = assign31680_e41949_d_n7;
        locals.var_nvt_dn8 = assign31680_e41949_d_n8;
        locals.var_nvt_dn9 = assign31680_e41949_d_n9;
        locals.var_nvt_dn10 = assign31680_e41949_d_n10;
        locals.var_nvt_dn11 = assign31680_e41949_d_n11;
        locals.var_nvt_dn12 = assign31680_e41949_d_n12;
        locals.var_nvt_dn13 = assign31680_e41949_d_n13;
        locals.var_nvt_dn14 = assign31680_e41949_d_n14;
        locals.var_nvt_rv = 0.0;

        let (assign31690_e41955, assign31690_e41955_d_n0, assign31690_e41955_d_n2, assign31690_e41955_d_n3, assign31690_e41955_d_n4, assign31690_e41955_d_n5, assign31690_e41955_d_n6, assign31690_e41955_d_n7, assign31690_e41955_d_n8, assign31690_e41955_d_n9, assign31690_e41955_d_n10, assign31690_e41955_d_n11, assign31690_e41955_d_n12, assign31690_e41955_d_n13, assign31690_e41955_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31690_e41953: f64 = (1.0 / locals.var_nvt);
        (assign31690_e41953, (-(locals.var_nvt_dn0 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn2 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn3 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn4 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn5 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn6 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn7 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn8 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn9 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn10 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn11 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn12 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn13 / (locals.var_nvt * locals.var_nvt))), (-(locals.var_nvt_dn14 / (locals.var_nvt * locals.var_nvt))),)
    } else {
        (locals.var_inv_nvt, locals.var_inv_nvt_dn0, locals.var_inv_nvt_dn2, locals.var_inv_nvt_dn3, locals.var_inv_nvt_dn4, locals.var_inv_nvt_dn5, locals.var_inv_nvt_dn6, locals.var_inv_nvt_dn7, locals.var_inv_nvt_dn8, locals.var_inv_nvt_dn9, locals.var_inv_nvt_dn10, locals.var_inv_nvt_dn11, locals.var_inv_nvt_dn12, locals.var_inv_nvt_dn13, locals.var_inv_nvt_dn14,)
    }
};
        locals.var_inv_nvt = assign31690_e41955;
        locals.var_inv_nvt_dn0 = assign31690_e41955_d_n0;
        locals.var_inv_nvt_dn2 = assign31690_e41955_d_n2;
        locals.var_inv_nvt_dn3 = assign31690_e41955_d_n3;
        locals.var_inv_nvt_dn4 = assign31690_e41955_d_n4;
        locals.var_inv_nvt_dn5 = assign31690_e41955_d_n5;
        locals.var_inv_nvt_dn6 = assign31690_e41955_d_n6;
        locals.var_inv_nvt_dn7 = assign31690_e41955_d_n7;
        locals.var_inv_nvt_dn8 = assign31690_e41955_d_n8;
        locals.var_inv_nvt_dn9 = assign31690_e41955_d_n9;
        locals.var_inv_nvt_dn10 = assign31690_e41955_d_n10;
        locals.var_inv_nvt_dn11 = assign31690_e41955_d_n11;
        locals.var_inv_nvt_dn12 = assign31690_e41955_d_n12;
        locals.var_inv_nvt_dn13 = assign31690_e41955_d_n13;
        locals.var_inv_nvt_dn14 = assign31690_e41955_d_n14;
        locals.var_inv_nvt_rv = 0.0;

        let (assign31700_e41961, assign31700_e41961_d_n0, assign31700_e41961_d_n2, assign31700_e41961_d_n3, assign31700_e41961_d_n4, assign31700_e41961_d_n5, assign31700_e41961_d_n6, assign31700_e41961_d_n7, assign31700_e41961_d_n8, assign31700_e41961_d_n9, assign31700_e41961_d_n10, assign31700_e41961_d_n11, assign31700_e41961_d_n12, assign31700_e41961_d_n13, assign31700_e41961_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31700_e41959: f64 = (locals.var_vg * locals.var_inv_nvt);
        (assign31700_e41959, (locals.var_vg * locals.var_inv_nvt_dn0), (locals.var_vg * locals.var_inv_nvt_dn2), (locals.var_vg * locals.var_inv_nvt_dn3), (locals.var_vg * locals.var_inv_nvt_dn4), (locals.var_vg * locals.var_inv_nvt_dn5), (locals.var_vg * locals.var_inv_nvt_dn6), (locals.var_vg * locals.var_inv_nvt_dn7), (locals.var_vg * locals.var_inv_nvt_dn8), ((locals.var_vg_dn9 * locals.var_inv_nvt) + (locals.var_vg * locals.var_inv_nvt_dn9)), (locals.var_vg * locals.var_inv_nvt_dn10), ((locals.var_vg_dn11 * locals.var_inv_nvt) + (locals.var_vg * locals.var_inv_nvt_dn11)), (locals.var_vg * locals.var_inv_nvt_dn12), (locals.var_vg * locals.var_inv_nvt_dn13), (locals.var_vg * locals.var_inv_nvt_dn14),)
    } else {
        (locals.var_vg_1, locals.var_vg_1_dn0, locals.var_vg_1_dn2, locals.var_vg_1_dn3, locals.var_vg_1_dn4, locals.var_vg_1_dn5, locals.var_vg_1_dn6, locals.var_vg_1_dn7, locals.var_vg_1_dn8, locals.var_vg_1_dn9, locals.var_vg_1_dn10, locals.var_vg_1_dn11, locals.var_vg_1_dn12, locals.var_vg_1_dn13, locals.var_vg_1_dn14,)
    }
};
        locals.var_vg_1 = assign31700_e41961;
        locals.var_vg_1_dn0 = assign31700_e41961_d_n0;
        locals.var_vg_1_dn2 = assign31700_e41961_d_n2;
        locals.var_vg_1_dn3 = assign31700_e41961_d_n3;
        locals.var_vg_1_dn4 = assign31700_e41961_d_n4;
        locals.var_vg_1_dn5 = assign31700_e41961_d_n5;
        locals.var_vg_1_dn6 = assign31700_e41961_d_n6;
        locals.var_vg_1_dn7 = assign31700_e41961_d_n7;
        locals.var_vg_1_dn8 = assign31700_e41961_d_n8;
        locals.var_vg_1_dn9 = assign31700_e41961_d_n9;
        locals.var_vg_1_dn10 = assign31700_e41961_d_n10;
        locals.var_vg_1_dn11 = assign31700_e41961_d_n11;
        locals.var_vg_1_dn12 = assign31700_e41961_d_n12;
        locals.var_vg_1_dn13 = assign31700_e41961_d_n13;
        locals.var_vg_1_dn14 = assign31700_e41961_d_n14;
        locals.var_vg_1_rv = 0.0;

        let (assign31710_e41967, assign31710_e41967_d_n0, assign31710_e41967_d_n2, assign31710_e41967_d_n3, assign31710_e41967_d_n4, assign31710_e41967_d_n5, assign31710_e41967_d_n6, assign31710_e41967_d_n7, assign31710_e41967_d_n8, assign31710_e41967_d_n9, assign31710_e41967_d_n10, assign31710_e41967_d_n11, assign31710_e41967_d_n12, assign31710_e41967_d_n13, assign31710_e41967_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31710_e41965: f64 = (locals.var_vs * locals.var_inv_nvt);
        (assign31710_e41965, (locals.var_vs * locals.var_inv_nvt_dn0), (locals.var_vs * locals.var_inv_nvt_dn2), (locals.var_vs * locals.var_inv_nvt_dn3), (locals.var_vs * locals.var_inv_nvt_dn4), ((locals.var_vs_dn5 * locals.var_inv_nvt) + (locals.var_vs * locals.var_inv_nvt_dn5)), (locals.var_vs * locals.var_inv_nvt_dn6), ((locals.var_vs_dn7 * locals.var_inv_nvt) + (locals.var_vs * locals.var_inv_nvt_dn7)), (locals.var_vs * locals.var_inv_nvt_dn8), (locals.var_vs * locals.var_inv_nvt_dn9), (locals.var_vs * locals.var_inv_nvt_dn10), ((locals.var_vs_dn11 * locals.var_inv_nvt) + (locals.var_vs * locals.var_inv_nvt_dn11)), (locals.var_vs * locals.var_inv_nvt_dn12), (locals.var_vs * locals.var_inv_nvt_dn13), (locals.var_vs * locals.var_inv_nvt_dn14),)
    } else {
        (locals.var_vs_1, locals.var_vs_1_dn0, locals.var_vs_1_dn2, locals.var_vs_1_dn3, locals.var_vs_1_dn4, locals.var_vs_1_dn5, locals.var_vs_1_dn6, locals.var_vs_1_dn7, locals.var_vs_1_dn8, locals.var_vs_1_dn9, locals.var_vs_1_dn10, locals.var_vs_1_dn11, locals.var_vs_1_dn12, locals.var_vs_1_dn13, locals.var_vs_1_dn14,)
    }
};
        locals.var_vs_1 = assign31710_e41967;
        locals.var_vs_1_dn0 = assign31710_e41967_d_n0;
        locals.var_vs_1_dn2 = assign31710_e41967_d_n2;
        locals.var_vs_1_dn3 = assign31710_e41967_d_n3;
        locals.var_vs_1_dn4 = assign31710_e41967_d_n4;
        locals.var_vs_1_dn5 = assign31710_e41967_d_n5;
        locals.var_vs_1_dn6 = assign31710_e41967_d_n6;
        locals.var_vs_1_dn7 = assign31710_e41967_d_n7;
        locals.var_vs_1_dn8 = assign31710_e41967_d_n8;
        locals.var_vs_1_dn9 = assign31710_e41967_d_n9;
        locals.var_vs_1_dn10 = assign31710_e41967_d_n10;
        locals.var_vs_1_dn11 = assign31710_e41967_d_n11;
        locals.var_vs_1_dn12 = assign31710_e41967_d_n12;
        locals.var_vs_1_dn13 = assign31710_e41967_d_n13;
        locals.var_vs_1_dn14 = assign31710_e41967_d_n14;
        locals.var_vs_1_rv = 0.0;

        let (assign31720_e41973, assign31720_e41973_d_n0, assign31720_e41973_d_n2, assign31720_e41973_d_n3, assign31720_e41973_d_n4, assign31720_e41973_d_n5, assign31720_e41973_d_n6, assign31720_e41973_d_n7, assign31720_e41973_d_n8, assign31720_e41973_d_n9, assign31720_e41973_d_n10, assign31720_e41973_d_n11, assign31720_e41973_d_n12, assign31720_e41973_d_n13, assign31720_e41973_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31720_e41971: f64 = (locals.var_vfb_i * locals.var_inv_nvt);
        (assign31720_e41971, ((locals.var_vfb_i_dn0 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn0)), ((locals.var_vfb_i_dn2 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn2)), ((locals.var_vfb_i_dn3 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn3)), ((locals.var_vfb_i_dn4 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn4)), ((locals.var_vfb_i_dn5 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn5)), ((locals.var_vfb_i_dn6 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn6)), ((locals.var_vfb_i_dn7 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn7)), ((locals.var_vfb_i_dn8 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn8)), ((locals.var_vfb_i_dn9 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn9)), ((locals.var_vfb_i_dn10 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn10)), ((locals.var_vfb_i_dn11 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn11)), ((locals.var_vfb_i_dn12 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn12)), ((locals.var_vfb_i_dn13 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn13)), ((locals.var_vfb_i_dn14 * locals.var_inv_nvt) + (locals.var_vfb_i * locals.var_inv_nvt_dn14)),)
    } else {
        (locals.var_vfb, locals.var_vfb_dn0, locals.var_vfb_dn2, locals.var_vfb_dn3, locals.var_vfb_dn4, locals.var_vfb_dn5, locals.var_vfb_dn6, locals.var_vfb_dn7, locals.var_vfb_dn8, locals.var_vfb_dn9, locals.var_vfb_dn10, locals.var_vfb_dn11, locals.var_vfb_dn12, locals.var_vfb_dn13, locals.var_vfb_dn14,)
    }
};
        locals.var_vfb = assign31720_e41973;
        locals.var_vfb_dn0 = assign31720_e41973_d_n0;
        locals.var_vfb_dn2 = assign31720_e41973_d_n2;
        locals.var_vfb_dn3 = assign31720_e41973_d_n3;
        locals.var_vfb_dn4 = assign31720_e41973_d_n4;
        locals.var_vfb_dn5 = assign31720_e41973_d_n5;
        locals.var_vfb_dn6 = assign31720_e41973_d_n6;
        locals.var_vfb_dn7 = assign31720_e41973_d_n7;
        locals.var_vfb_dn8 = assign31720_e41973_d_n8;
        locals.var_vfb_dn9 = assign31720_e41973_d_n9;
        locals.var_vfb_dn10 = assign31720_e41973_d_n10;
        locals.var_vfb_dn11 = assign31720_e41973_d_n11;
        locals.var_vfb_dn12 = assign31720_e41973_d_n12;
        locals.var_vfb_dn13 = assign31720_e41973_d_n13;
        locals.var_vfb_dn14 = assign31720_e41973_d_n14;
        locals.var_vfb_rv = 0.0;

        let (assign31730_e41984, assign31730_e41984_d_n0, assign31730_e41984_d_n2, assign31730_e41984_d_n3, assign31730_e41984_d_n4, assign31730_e41984_d_n5, assign31730_e41984_d_n6, assign31730_e41984_d_n7, assign31730_e41984_d_n8, assign31730_e41984_d_n9, assign31730_e41984_d_n10, assign31730_e41984_d_n11, assign31730_e41984_d_n12, assign31730_e41984_d_n13, assign31730_e41984_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31730_e41978: f64 = (locals.var_etabedge_i * locals.var_vbsx);
        let assign31730_e41979: f64 = (locals.var_eta0edge_t + assign31730_e41978);
        let assign31730_e41980: f64 = (-assign31730_e41979);
        let assign31730_e41982: f64 = (assign31730_e41980 * locals.var_vdsx);
        (assign31730_e41982, (((-(locals.var_eta0edge_t_dn0 + (locals.var_etabedge_i * locals.var_vbsx_dn0))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn0)), (((-(locals.var_eta0edge_t_dn2 + (locals.var_etabedge_i * locals.var_vbsx_dn2))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn2)), (((-(locals.var_eta0edge_t_dn3 + (locals.var_etabedge_i * locals.var_vbsx_dn3))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn3)), (((-(locals.var_eta0edge_t_dn4 + (locals.var_etabedge_i * locals.var_vbsx_dn4))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn4)), (((-(locals.var_eta0edge_t_dn5 + (locals.var_etabedge_i * locals.var_vbsx_dn5))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn5)), (((-(locals.var_eta0edge_t_dn6 + (locals.var_etabedge_i * locals.var_vbsx_dn6))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn6)), (((-(locals.var_eta0edge_t_dn7 + (locals.var_etabedge_i * locals.var_vbsx_dn7))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn7)), (((-(locals.var_eta0edge_t_dn8 + (locals.var_etabedge_i * locals.var_vbsx_dn8))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn8)), (((-(locals.var_eta0edge_t_dn9 + (locals.var_etabedge_i * locals.var_vbsx_dn9))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn9)), (((-(locals.var_eta0edge_t_dn10 + (locals.var_etabedge_i * locals.var_vbsx_dn10))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn10)), (((-(locals.var_eta0edge_t_dn11 + (locals.var_etabedge_i * locals.var_vbsx_dn11))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn11)), (((-(locals.var_eta0edge_t_dn12 + (locals.var_etabedge_i * locals.var_vbsx_dn12))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn12)), (((-(locals.var_eta0edge_t_dn13 + (locals.var_etabedge_i * locals.var_vbsx_dn13))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn13)), (((-(locals.var_eta0edge_t_dn14 + (locals.var_etabedge_i * locals.var_vbsx_dn14))) * locals.var_vdsx) + (assign31730_e41980 * locals.var_vdsx_dn14)),)
    } else {
        (locals.var_dvth_dibl_1, locals.var_dvth_dibl_1_dn0, locals.var_dvth_dibl_1_dn2, locals.var_dvth_dibl_1_dn3, locals.var_dvth_dibl_1_dn4, locals.var_dvth_dibl_1_dn5, locals.var_dvth_dibl_1_dn6, locals.var_dvth_dibl_1_dn7, locals.var_dvth_dibl_1_dn8, locals.var_dvth_dibl_1_dn9, locals.var_dvth_dibl_1_dn10, locals.var_dvth_dibl_1_dn11, locals.var_dvth_dibl_1_dn12, locals.var_dvth_dibl_1_dn13, locals.var_dvth_dibl_1_dn14,)
    }
};
        locals.var_dvth_dibl_1 = assign31730_e41984;
        locals.var_dvth_dibl_1_dn0 = assign31730_e41984_d_n0;
        locals.var_dvth_dibl_1_dn2 = assign31730_e41984_d_n2;
        locals.var_dvth_dibl_1_dn3 = assign31730_e41984_d_n3;
        locals.var_dvth_dibl_1_dn4 = assign31730_e41984_d_n4;
        locals.var_dvth_dibl_1_dn5 = assign31730_e41984_d_n5;
        locals.var_dvth_dibl_1_dn6 = assign31730_e41984_d_n6;
        locals.var_dvth_dibl_1_dn7 = assign31730_e41984_d_n7;
        locals.var_dvth_dibl_1_dn8 = assign31730_e41984_d_n8;
        locals.var_dvth_dibl_1_dn9 = assign31730_e41984_d_n9;
        locals.var_dvth_dibl_1_dn10 = assign31730_e41984_d_n10;
        locals.var_dvth_dibl_1_dn11 = assign31730_e41984_d_n11;
        locals.var_dvth_dibl_1_dn12 = assign31730_e41984_d_n12;
        locals.var_dvth_dibl_1_dn13 = assign31730_e41984_d_n13;
        locals.var_dvth_dibl_1_dn14 = assign31730_e41984_d_n14;
        locals.var_dvth_dibl_1_rv = 0.0;

        let (assign31740_e42002, assign31740_e42002_d_n0, assign31740_e42002_d_n2, assign31740_e42002_d_n3, assign31740_e42002_d_n4, assign31740_e42002_d_n5, assign31740_e42002_d_n6, assign31740_e42002_d_n7, assign31740_e42002_d_n8, assign31740_e42002_d_n9, assign31740_e42002_d_n10, assign31740_e42002_d_n11, assign31740_e42002_d_n12, assign31740_e42002_d_n13, assign31740_e42002_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31740_e41989: f64 = (locals.var_kt1ledge_i / locals.var_leff);
        let assign31740_e41990: f64 = (locals.var_kt1edge_i + assign31740_e41989);
        let assign31740_e41993: f64 = (locals.var_kt2edge_i * locals.var_vbsx);
        let assign31740_e41994: f64 = (assign31740_e41990 + assign31740_e41993);
        let assign31740_e41997: f64 = (locals.var_tratio).powf(locals.var_kt1expedge_i);
        let assign31740_e41999: f64 = (assign31740_e41997 - 1.0);
        let assign31740_e42000: f64 = (assign31740_e41994 * assign31740_e41999);
        (assign31740_e42000, ((locals.var_kt2edge_i * locals.var_vbsx_dn0) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn2) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn3) * assign31740_e41999), (((locals.var_kt2edge_i * locals.var_vbsx_dn4) * assign31740_e41999) + (assign31740_e41994 * if 0.0 == 0.0 && ((locals.var_kt1expedge_i) as f64).is_finite() && ((locals.var_kt1expedge_i) as f64).fract() == 0.0 { if locals.var_kt1expedge_i == 0.0 { 0.0 } else { (locals.var_kt1expedge_i * ((locals.var_tratio).powf(locals.var_kt1expedge_i - 1.0) * locals.var_tratio_dn4)) } } else { (assign31740_e41997 * (locals.var_kt1expedge_i * (locals.var_tratio_dn4 / locals.var_tratio))) })), ((locals.var_kt2edge_i * locals.var_vbsx_dn5) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn6) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn7) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn8) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn9) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn10) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn11) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn12) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn13) * assign31740_e41999), ((locals.var_kt2edge_i * locals.var_vbsx_dn14) * assign31740_e41999),)
    } else {
        (locals.var_dvth_temp, locals.var_dvth_temp_dn0, locals.var_dvth_temp_dn2, locals.var_dvth_temp_dn3, locals.var_dvth_temp_dn4, locals.var_dvth_temp_dn5, locals.var_dvth_temp_dn6, locals.var_dvth_temp_dn7, locals.var_dvth_temp_dn8, locals.var_dvth_temp_dn9, locals.var_dvth_temp_dn10, locals.var_dvth_temp_dn11, locals.var_dvth_temp_dn12, locals.var_dvth_temp_dn13, locals.var_dvth_temp_dn14,)
    }
};
        locals.var_dvth_temp = assign31740_e42002;
        locals.var_dvth_temp_dn0 = assign31740_e42002_d_n0;
        locals.var_dvth_temp_dn2 = assign31740_e42002_d_n2;
        locals.var_dvth_temp_dn3 = assign31740_e42002_d_n3;
        locals.var_dvth_temp_dn4 = assign31740_e42002_d_n4;
        locals.var_dvth_temp_dn5 = assign31740_e42002_d_n5;
        locals.var_dvth_temp_dn6 = assign31740_e42002_d_n6;
        locals.var_dvth_temp_dn7 = assign31740_e42002_d_n7;
        locals.var_dvth_temp_dn8 = assign31740_e42002_d_n8;
        locals.var_dvth_temp_dn9 = assign31740_e42002_d_n9;
        locals.var_dvth_temp_dn10 = assign31740_e42002_d_n10;
        locals.var_dvth_temp_dn11 = assign31740_e42002_d_n11;
        locals.var_dvth_temp_dn12 = assign31740_e42002_d_n12;
        locals.var_dvth_temp_dn13 = assign31740_e42002_d_n13;
        locals.var_dvth_temp_dn14 = assign31740_e42002_d_n14;
        locals.var_dvth_temp_rv = 0.0;

        let (assign31750_e42012, assign31750_e42012_d_n0, assign31750_e42012_d_n2, assign31750_e42012_d_n3, assign31750_e42012_d_n4, assign31750_e42012_d_n5, assign31750_e42012_d_n6, assign31750_e42012_d_n7, assign31750_e42012_d_n8, assign31750_e42012_d_n9, assign31750_e42012_d_n10, assign31750_e42012_d_n11, assign31750_e42012_d_n12, assign31750_e42012_d_n13, assign31750_e42012_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31750_e42008: f64 = (p.p1016 * locals.var_vbsx);
        let assign31750_e42009: f64 = (1.0 + assign31750_e42008);
        let assign31750_e42010: f64 = (locals.var_litl * assign31750_e42009);
        (assign31750_e42010, (locals.var_litl * (p.p1016 * locals.var_vbsx_dn0)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn2)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn3)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn4)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn5)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn6)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn7)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn8)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn9)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn10)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn11)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn12)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn13)), (locals.var_litl * (p.p1016 * locals.var_vbsx_dn14)),)
    } else {
        (locals.var_litl_edge, locals.var_litl_edge_dn0, locals.var_litl_edge_dn2, locals.var_litl_edge_dn3, locals.var_litl_edge_dn4, locals.var_litl_edge_dn5, locals.var_litl_edge_dn6, locals.var_litl_edge_dn7, locals.var_litl_edge_dn8, locals.var_litl_edge_dn9, locals.var_litl_edge_dn10, locals.var_litl_edge_dn11, locals.var_litl_edge_dn12, locals.var_litl_edge_dn13, locals.var_litl_edge_dn14,)
    }
};
        locals.var_litl_edge = assign31750_e42012;
        locals.var_litl_edge_dn0 = assign31750_e42012_d_n0;
        locals.var_litl_edge_dn2 = assign31750_e42012_d_n2;
        locals.var_litl_edge_dn3 = assign31750_e42012_d_n3;
        locals.var_litl_edge_dn4 = assign31750_e42012_d_n4;
        locals.var_litl_edge_dn5 = assign31750_e42012_d_n5;
        locals.var_litl_edge_dn6 = assign31750_e42012_d_n6;
        locals.var_litl_edge_dn7 = assign31750_e42012_d_n7;
        locals.var_litl_edge_dn8 = assign31750_e42012_d_n8;
        locals.var_litl_edge_dn9 = assign31750_e42012_d_n9;
        locals.var_litl_edge_dn10 = assign31750_e42012_d_n10;
        locals.var_litl_edge_dn11 = assign31750_e42012_d_n11;
        locals.var_litl_edge_dn12 = assign31750_e42012_d_n12;
        locals.var_litl_edge_dn13 = assign31750_e42012_d_n13;
        locals.var_litl_edge_dn14 = assign31750_e42012_d_n14;
        locals.var_litl_edge_rv = 0.0;

        let assign31760_e42015: f64 = if locals.var_litl_edge > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard733 = assign31760_e42015;
        locals.var_guard733_rv = 0.0;

        let (assign31770_e42025, assign31770_e42025_d_n0, assign31770_e42025_d_n2, assign31770_e42025_d_n3, assign31770_e42025_d_n4, assign31770_e42025_d_n5, assign31770_e42025_d_n6, assign31770_e42025_d_n7, assign31770_e42025_d_n8, assign31770_e42025_d_n9, assign31770_e42025_d_n10, assign31770_e42025_d_n11, assign31770_e42025_d_n12, assign31770_e42025_d_n13, assign31770_e42025_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard733 != 0.0)) {
        let assign31770_e42021: f64 = (p.p1015 * locals.var_leff);
        let assign31770_e42023: f64 = (assign31770_e42021 / locals.var_litl_edge);
        (assign31770_e42023, (-((assign31770_e42021 * locals.var_litl_edge_dn0) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn2) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn3) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn4) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn5) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn6) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn7) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn8) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn9) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn10) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn11) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn12) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn13) / (locals.var_litl_edge * locals.var_litl_edge))), (-((assign31770_e42021 * locals.var_litl_edge_dn14) / (locals.var_litl_edge * locals.var_litl_edge))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign31770_e42025;
        locals.var_t0_dn0 = assign31770_e42025_d_n0;
        locals.var_t0_dn2 = assign31770_e42025_d_n2;
        locals.var_t0_dn3 = assign31770_e42025_d_n3;
        locals.var_t0_dn4 = assign31770_e42025_d_n4;
        locals.var_t0_dn5 = assign31770_e42025_d_n5;
        locals.var_t0_dn6 = assign31770_e42025_d_n6;
        locals.var_t0_dn7 = assign31770_e42025_d_n7;
        locals.var_t0_dn8 = assign31770_e42025_d_n8;
        locals.var_t0_dn9 = assign31770_e42025_d_n9;
        locals.var_t0_dn10 = assign31770_e42025_d_n10;
        locals.var_t0_dn11 = assign31770_e42025_d_n11;
        locals.var_t0_dn12 = assign31770_e42025_d_n12;
        locals.var_t0_dn13 = assign31770_e42025_d_n13;
        locals.var_t0_dn14 = assign31770_e42025_d_n14;
        locals.var_t0_rv = 0.0;

        let assign31780_e42028: f64 = if locals.var_t0 < 40.0 { 1.0 } else { 0.0 };
        locals.var_guard734 = assign31780_e42028;
        locals.var_guard734_rv = 0.0;

        let (assign31790_e42043, assign31790_e42043_d_n0, assign31790_e42043_d_n2, assign31790_e42043_d_n3, assign31790_e42043_d_n4, assign31790_e42043_d_n5, assign31790_e42043_d_n6, assign31790_e42043_d_n7, assign31790_e42043_d_n8, assign31790_e42043_d_n9, assign31790_e42043_d_n10, assign31790_e42043_d_n11, assign31790_e42043_d_n12, assign31790_e42043_d_n13, assign31790_e42043_d_n14,) = {
    if (((locals.var_guard730 != 0.0) && (locals.var_guard733 != 0.0)) && (locals.var_guard734 != 0.0)) {
        let assign31790_e42036: f64 = (0.5 * p.p1014);
        let assign31790_e42038: f64 = (locals.var_t0).cosh();
        let assign31790_e42040: f64 = (assign31790_e42038 - 1.0);
        let assign31790_e42041: f64 = (assign31790_e42036 / assign31790_e42040);
        (assign31790_e42041, (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn0)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn2)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn3)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn4)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn5)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn6)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn7)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn8)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn9)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn10)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn11)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn12)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn13)) / (assign31790_e42040 * assign31790_e42040))), (-((assign31790_e42036 * ((locals.var_t0).sinh() * locals.var_t0_dn14)) / (assign31790_e42040 * assign31790_e42040))),)
    } else {
        (locals.var_theta_sce_edge, locals.var_theta_sce_edge_dn0, locals.var_theta_sce_edge_dn2, locals.var_theta_sce_edge_dn3, locals.var_theta_sce_edge_dn4, locals.var_theta_sce_edge_dn5, locals.var_theta_sce_edge_dn6, locals.var_theta_sce_edge_dn7, locals.var_theta_sce_edge_dn8, locals.var_theta_sce_edge_dn9, locals.var_theta_sce_edge_dn10, locals.var_theta_sce_edge_dn11, locals.var_theta_sce_edge_dn12, locals.var_theta_sce_edge_dn13, locals.var_theta_sce_edge_dn14,)
    }
};
        locals.var_theta_sce_edge = assign31790_e42043;
        locals.var_theta_sce_edge_dn0 = assign31790_e42043_d_n0;
        locals.var_theta_sce_edge_dn2 = assign31790_e42043_d_n2;
        locals.var_theta_sce_edge_dn3 = assign31790_e42043_d_n3;
        locals.var_theta_sce_edge_dn4 = assign31790_e42043_d_n4;
        locals.var_theta_sce_edge_dn5 = assign31790_e42043_d_n5;
        locals.var_theta_sce_edge_dn6 = assign31790_e42043_d_n6;
        locals.var_theta_sce_edge_dn7 = assign31790_e42043_d_n7;
        locals.var_theta_sce_edge_dn8 = assign31790_e42043_d_n8;
        locals.var_theta_sce_edge_dn9 = assign31790_e42043_d_n9;
        locals.var_theta_sce_edge_dn10 = assign31790_e42043_d_n10;
        locals.var_theta_sce_edge_dn11 = assign31790_e42043_d_n11;
        locals.var_theta_sce_edge_dn12 = assign31790_e42043_d_n12;
        locals.var_theta_sce_edge_dn13 = assign31790_e42043_d_n13;
        locals.var_theta_sce_edge_dn14 = assign31790_e42043_d_n14;
        locals.var_theta_sce_edge_rv = 0.0;

        let (assign31800_e42056, assign31800_e42056_d_n0, assign31800_e42056_d_n2, assign31800_e42056_d_n3, assign31800_e42056_d_n4, assign31800_e42056_d_n5, assign31800_e42056_d_n6, assign31800_e42056_d_n7, assign31800_e42056_d_n8, assign31800_e42056_d_n9, assign31800_e42056_d_n10, assign31800_e42056_d_n11, assign31800_e42056_d_n12, assign31800_e42056_d_n13, assign31800_e42056_d_n14,) = {
    if (((locals.var_guard730 != 0.0) && (locals.var_guard733 != 0.0)) && (locals.var_guard734 == 0.0)) {
        let assign31800_e42052: f64 = (-locals.var_t0);
        let assign31800_e42053: f64 = { let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign31800_e42054: f64 = (p.p1014 * assign31800_e42053);
        (assign31800_e42054, (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn0))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn2))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn3))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn4))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn5))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn6))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn7))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn8))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn9))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn10))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn11))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn12))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn13))), (p.p1014 * ({ let limited_exp_arg = assign31800_e42052; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t0_dn14))),)
    } else {
        (locals.var_theta_sce_edge, locals.var_theta_sce_edge_dn0, locals.var_theta_sce_edge_dn2, locals.var_theta_sce_edge_dn3, locals.var_theta_sce_edge_dn4, locals.var_theta_sce_edge_dn5, locals.var_theta_sce_edge_dn6, locals.var_theta_sce_edge_dn7, locals.var_theta_sce_edge_dn8, locals.var_theta_sce_edge_dn9, locals.var_theta_sce_edge_dn10, locals.var_theta_sce_edge_dn11, locals.var_theta_sce_edge_dn12, locals.var_theta_sce_edge_dn13, locals.var_theta_sce_edge_dn14,)
    }
};
        locals.var_theta_sce_edge = assign31800_e42056;
        locals.var_theta_sce_edge_dn0 = assign31800_e42056_d_n0;
        locals.var_theta_sce_edge_dn2 = assign31800_e42056_d_n2;
        locals.var_theta_sce_edge_dn3 = assign31800_e42056_d_n3;
        locals.var_theta_sce_edge_dn4 = assign31800_e42056_d_n4;
        locals.var_theta_sce_edge_dn5 = assign31800_e42056_d_n5;
        locals.var_theta_sce_edge_dn6 = assign31800_e42056_d_n6;
        locals.var_theta_sce_edge_dn7 = assign31800_e42056_d_n7;
        locals.var_theta_sce_edge_dn8 = assign31800_e42056_d_n8;
        locals.var_theta_sce_edge_dn9 = assign31800_e42056_d_n9;
        locals.var_theta_sce_edge_dn10 = assign31800_e42056_d_n10;
        locals.var_theta_sce_edge_dn11 = assign31800_e42056_d_n11;
        locals.var_theta_sce_edge_dn12 = assign31800_e42056_d_n12;
        locals.var_theta_sce_edge_dn13 = assign31800_e42056_d_n13;
        locals.var_theta_sce_edge_dn14 = assign31800_e42056_d_n14;
        locals.var_theta_sce_edge_rv = 0.0;

        let (assign31810_e42063, assign31810_e42063_d_n0, assign31810_e42063_d_n2, assign31810_e42063_d_n3, assign31810_e42063_d_n4, assign31810_e42063_d_n5, assign31810_e42063_d_n6, assign31810_e42063_d_n7, assign31810_e42063_d_n8, assign31810_e42063_d_n9, assign31810_e42063_d_n10, assign31810_e42063_d_n11, assign31810_e42063_d_n12, assign31810_e42063_d_n13, assign31810_e42063_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard733 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_theta_sce_edge, locals.var_theta_sce_edge_dn0, locals.var_theta_sce_edge_dn2, locals.var_theta_sce_edge_dn3, locals.var_theta_sce_edge_dn4, locals.var_theta_sce_edge_dn5, locals.var_theta_sce_edge_dn6, locals.var_theta_sce_edge_dn7, locals.var_theta_sce_edge_dn8, locals.var_theta_sce_edge_dn9, locals.var_theta_sce_edge_dn10, locals.var_theta_sce_edge_dn11, locals.var_theta_sce_edge_dn12, locals.var_theta_sce_edge_dn13, locals.var_theta_sce_edge_dn14,)
    }
};
        locals.var_theta_sce_edge = assign31810_e42063;
        locals.var_theta_sce_edge_dn0 = assign31810_e42063_d_n0;
        locals.var_theta_sce_edge_dn2 = assign31810_e42063_d_n2;
        locals.var_theta_sce_edge_dn3 = assign31810_e42063_d_n3;
        locals.var_theta_sce_edge_dn4 = assign31810_e42063_d_n4;
        locals.var_theta_sce_edge_dn5 = assign31810_e42063_d_n5;
        locals.var_theta_sce_edge_dn6 = assign31810_e42063_d_n6;
        locals.var_theta_sce_edge_dn7 = assign31810_e42063_d_n7;
        locals.var_theta_sce_edge_dn8 = assign31810_e42063_d_n8;
        locals.var_theta_sce_edge_dn9 = assign31810_e42063_d_n9;
        locals.var_theta_sce_edge_dn10 = assign31810_e42063_d_n10;
        locals.var_theta_sce_edge_dn11 = assign31810_e42063_d_n11;
        locals.var_theta_sce_edge_dn12 = assign31810_e42063_d_n12;
        locals.var_theta_sce_edge_dn13 = assign31810_e42063_d_n13;
        locals.var_theta_sce_edge_dn14 = assign31810_e42063_d_n14;
        locals.var_theta_sce_edge_rv = 0.0;

        let (assign31820_e42071, assign31820_e42071_d_n0, assign31820_e42071_d_n2, assign31820_e42071_d_n3, assign31820_e42071_d_n4, assign31820_e42071_d_n5, assign31820_e42071_d_n6, assign31820_e42071_d_n7, assign31820_e42071_d_n8, assign31820_e42071_d_n9, assign31820_e42071_d_n10, assign31820_e42071_d_n11, assign31820_e42071_d_n12, assign31820_e42071_d_n13, assign31820_e42071_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31820_e42068: f64 = (locals.var_vbi_edge - locals.var_phist);
        let assign31820_e42069: f64 = (locals.var_theta_sce_edge * assign31820_e42068);
        (assign31820_e42069, ((locals.var_theta_sce_edge_dn0 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn0 - locals.var_phist_dn0))), ((locals.var_theta_sce_edge_dn2 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn2 - locals.var_phist_dn2))), ((locals.var_theta_sce_edge_dn3 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn3 - locals.var_phist_dn3))), ((locals.var_theta_sce_edge_dn4 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn4 - locals.var_phist_dn4))), ((locals.var_theta_sce_edge_dn5 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn5 - locals.var_phist_dn5))), ((locals.var_theta_sce_edge_dn6 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn6 - locals.var_phist_dn6))), ((locals.var_theta_sce_edge_dn7 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn7 - locals.var_phist_dn7))), ((locals.var_theta_sce_edge_dn8 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn8 - locals.var_phist_dn8))), ((locals.var_theta_sce_edge_dn9 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn9 - locals.var_phist_dn9))), ((locals.var_theta_sce_edge_dn10 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn10 - locals.var_phist_dn10))), ((locals.var_theta_sce_edge_dn11 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn11 - locals.var_phist_dn11))), ((locals.var_theta_sce_edge_dn12 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn12 - locals.var_phist_dn12))), ((locals.var_theta_sce_edge_dn13 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn13 - locals.var_phist_dn13))), ((locals.var_theta_sce_edge_dn14 * assign31820_e42068) + (locals.var_theta_sce_edge * (locals.var_vbi_edge_dn14 - locals.var_phist_dn14))),)
    } else {
        (locals.var_dvth_sce, locals.var_dvth_sce_dn0, locals.var_dvth_sce_dn2, locals.var_dvth_sce_dn3, locals.var_dvth_sce_dn4, locals.var_dvth_sce_dn5, locals.var_dvth_sce_dn6, locals.var_dvth_sce_dn7, locals.var_dvth_sce_dn8, locals.var_dvth_sce_dn9, locals.var_dvth_sce_dn10, locals.var_dvth_sce_dn11, locals.var_dvth_sce_dn12, locals.var_dvth_sce_dn13, locals.var_dvth_sce_dn14,)
    }
};
        locals.var_dvth_sce = assign31820_e42071;
        locals.var_dvth_sce_dn0 = assign31820_e42071_d_n0;
        locals.var_dvth_sce_dn2 = assign31820_e42071_d_n2;
        locals.var_dvth_sce_dn3 = assign31820_e42071_d_n3;
        locals.var_dvth_sce_dn4 = assign31820_e42071_d_n4;
        locals.var_dvth_sce_dn5 = assign31820_e42071_d_n5;
        locals.var_dvth_sce_dn6 = assign31820_e42071_d_n6;
        locals.var_dvth_sce_dn7 = assign31820_e42071_d_n7;
        locals.var_dvth_sce_dn8 = assign31820_e42071_d_n8;
        locals.var_dvth_sce_dn9 = assign31820_e42071_d_n9;
        locals.var_dvth_sce_dn10 = assign31820_e42071_d_n10;
        locals.var_dvth_sce_dn11 = assign31820_e42071_d_n11;
        locals.var_dvth_sce_dn12 = assign31820_e42071_d_n12;
        locals.var_dvth_sce_dn13 = assign31820_e42071_d_n13;
        locals.var_dvth_sce_dn14 = assign31820_e42071_d_n14;
        locals.var_dvth_sce_rv = 0.0;

        let (assign31830_e42091, assign31830_e42091_d_n0, assign31830_e42091_d_n2, assign31830_e42091_d_n3, assign31830_e42091_d_n4, assign31830_e42091_d_n5, assign31830_e42091_d_n6, assign31830_e42091_d_n7, assign31830_e42091_d_n8, assign31830_e42091_d_n9, assign31830_e42091_d_n10, assign31830_e42091_d_n11, assign31830_e42091_d_n12, assign31830_e42091_d_n13, assign31830_e42091_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31830_e42075: f64 = (locals.var_dvth_dibl_1 - locals.var_dvth_temp);
        let assign31830_e42077: f64 = (assign31830_e42075 + locals.var_dvth_sce);
        let assign31830_e42079: f64 = (assign31830_e42077 + p.p961);
        let assign31830_e42081: f64 = (assign31830_e42079 + locals.var_vth0_stress_edge);
        let assign31830_e42084: f64 = (locals.var_k2edge_i + locals.var_k2_well_edge);
        let assign31830_e42086: f64 = (assign31830_e42084 * locals.var_vbsx);
        let assign31830_e42087: f64 = (assign31830_e42081 - assign31830_e42086);
        let assign31830_e42089: f64 = (assign31830_e42087 + locals.var_vth0_well_edge);
        (assign31830_e42089, (((((locals.var_dvth_dibl_1_dn0 - locals.var_dvth_temp_dn0) + locals.var_dvth_sce_dn0) + locals.var_vth0_stress_edge_dn0) - (((locals.var_k2edge_i_dn0 + locals.var_k2_well_edge_dn0) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn0))) + locals.var_vth0_well_edge_dn0), (((((locals.var_dvth_dibl_1_dn2 - locals.var_dvth_temp_dn2) + locals.var_dvth_sce_dn2) + locals.var_vth0_stress_edge_dn2) - (((locals.var_k2edge_i_dn2 + locals.var_k2_well_edge_dn2) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn2))) + locals.var_vth0_well_edge_dn2), (((((locals.var_dvth_dibl_1_dn3 - locals.var_dvth_temp_dn3) + locals.var_dvth_sce_dn3) + locals.var_vth0_stress_edge_dn3) - (((locals.var_k2edge_i_dn3 + locals.var_k2_well_edge_dn3) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn3))) + locals.var_vth0_well_edge_dn3), (((((locals.var_dvth_dibl_1_dn4 - locals.var_dvth_temp_dn4) + locals.var_dvth_sce_dn4) + locals.var_vth0_stress_edge_dn4) - (((locals.var_k2edge_i_dn4 + locals.var_k2_well_edge_dn4) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn4))) + locals.var_vth0_well_edge_dn4), (((((locals.var_dvth_dibl_1_dn5 - locals.var_dvth_temp_dn5) + locals.var_dvth_sce_dn5) + locals.var_vth0_stress_edge_dn5) - (((locals.var_k2edge_i_dn5 + locals.var_k2_well_edge_dn5) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn5))) + locals.var_vth0_well_edge_dn5), (((((locals.var_dvth_dibl_1_dn6 - locals.var_dvth_temp_dn6) + locals.var_dvth_sce_dn6) + locals.var_vth0_stress_edge_dn6) - (((locals.var_k2edge_i_dn6 + locals.var_k2_well_edge_dn6) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn6))) + locals.var_vth0_well_edge_dn6), (((((locals.var_dvth_dibl_1_dn7 - locals.var_dvth_temp_dn7) + locals.var_dvth_sce_dn7) + locals.var_vth0_stress_edge_dn7) - (((locals.var_k2edge_i_dn7 + locals.var_k2_well_edge_dn7) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn7))) + locals.var_vth0_well_edge_dn7), (((((locals.var_dvth_dibl_1_dn8 - locals.var_dvth_temp_dn8) + locals.var_dvth_sce_dn8) + locals.var_vth0_stress_edge_dn8) - (((locals.var_k2edge_i_dn8 + locals.var_k2_well_edge_dn8) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn8))) + locals.var_vth0_well_edge_dn8), (((((locals.var_dvth_dibl_1_dn9 - locals.var_dvth_temp_dn9) + locals.var_dvth_sce_dn9) + locals.var_vth0_stress_edge_dn9) - (((locals.var_k2edge_i_dn9 + locals.var_k2_well_edge_dn9) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn9))) + locals.var_vth0_well_edge_dn9), (((((locals.var_dvth_dibl_1_dn10 - locals.var_dvth_temp_dn10) + locals.var_dvth_sce_dn10) + locals.var_vth0_stress_edge_dn10) - (((locals.var_k2edge_i_dn10 + locals.var_k2_well_edge_dn10) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn10))) + locals.var_vth0_well_edge_dn10), (((((locals.var_dvth_dibl_1_dn11 - locals.var_dvth_temp_dn11) + locals.var_dvth_sce_dn11) + locals.var_vth0_stress_edge_dn11) - (((locals.var_k2edge_i_dn11 + locals.var_k2_well_edge_dn11) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn11))) + locals.var_vth0_well_edge_dn11), (((((locals.var_dvth_dibl_1_dn12 - locals.var_dvth_temp_dn12) + locals.var_dvth_sce_dn12) + locals.var_vth0_stress_edge_dn12) - (((locals.var_k2edge_i_dn12 + locals.var_k2_well_edge_dn12) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn12))) + locals.var_vth0_well_edge_dn12), (((((locals.var_dvth_dibl_1_dn13 - locals.var_dvth_temp_dn13) + locals.var_dvth_sce_dn13) + locals.var_vth0_stress_edge_dn13) - (((locals.var_k2edge_i_dn13 + locals.var_k2_well_edge_dn13) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn13))) + locals.var_vth0_well_edge_dn13), (((((locals.var_dvth_dibl_1_dn14 - locals.var_dvth_temp_dn14) + locals.var_dvth_sce_dn14) + locals.var_vth0_stress_edge_dn14) - (((locals.var_k2edge_i_dn14 + locals.var_k2_well_edge_dn14) * locals.var_vbsx) + (assign31830_e42084 * locals.var_vbsx_dn14))) + locals.var_vth0_well_edge_dn14),)
    } else {
        (locals.var_vth_shift, locals.var_vth_shift_dn0, locals.var_vth_shift_dn2, locals.var_vth_shift_dn3, locals.var_vth_shift_dn4, locals.var_vth_shift_dn5, locals.var_vth_shift_dn6, locals.var_vth_shift_dn7, locals.var_vth_shift_dn8, locals.var_vth_shift_dn9, locals.var_vth_shift_dn10, locals.var_vth_shift_dn11, locals.var_vth_shift_dn12, locals.var_vth_shift_dn13, locals.var_vth_shift_dn14,)
    }
};
        locals.var_vth_shift = assign31830_e42091;
        locals.var_vth_shift_dn0 = assign31830_e42091_d_n0;
        locals.var_vth_shift_dn2 = assign31830_e42091_d_n2;
        locals.var_vth_shift_dn3 = assign31830_e42091_d_n3;
        locals.var_vth_shift_dn4 = assign31830_e42091_d_n4;
        locals.var_vth_shift_dn5 = assign31830_e42091_d_n5;
        locals.var_vth_shift_dn6 = assign31830_e42091_d_n6;
        locals.var_vth_shift_dn7 = assign31830_e42091_d_n7;
        locals.var_vth_shift_dn8 = assign31830_e42091_d_n8;
        locals.var_vth_shift_dn9 = assign31830_e42091_d_n9;
        locals.var_vth_shift_dn10 = assign31830_e42091_d_n10;
        locals.var_vth_shift_dn11 = assign31830_e42091_d_n11;
        locals.var_vth_shift_dn12 = assign31830_e42091_d_n12;
        locals.var_vth_shift_dn13 = assign31830_e42091_d_n13;
        locals.var_vth_shift_dn14 = assign31830_e42091_d_n14;
        locals.var_vth_shift_rv = 0.0;

        let (assign31840_e42101, assign31840_e42101_d_n0, assign31840_e42101_d_n2, assign31840_e42101_d_n3, assign31840_e42101_d_n4, assign31840_e42101_d_n5, assign31840_e42101_d_n6, assign31840_e42101_d_n7, assign31840_e42101_d_n8, assign31840_e42101_d_n9, assign31840_e42101_d_n10, assign31840_e42101_d_n11, assign31840_e42101_d_n12, assign31840_e42101_d_n13, assign31840_e42101_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31840_e42095: f64 = (locals.var_vg_1 - locals.var_vfb);
        let assign31840_e42098: f64 = (locals.var_vth_shift * locals.var_inv_nvt);
        let assign31840_e42099: f64 = (assign31840_e42095 - assign31840_e42098);
        (assign31840_e42099, ((locals.var_vg_1_dn0 - locals.var_vfb_dn0) - ((locals.var_vth_shift_dn0 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn0))), ((locals.var_vg_1_dn2 - locals.var_vfb_dn2) - ((locals.var_vth_shift_dn2 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn2))), ((locals.var_vg_1_dn3 - locals.var_vfb_dn3) - ((locals.var_vth_shift_dn3 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn3))), ((locals.var_vg_1_dn4 - locals.var_vfb_dn4) - ((locals.var_vth_shift_dn4 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn4))), ((locals.var_vg_1_dn5 - locals.var_vfb_dn5) - ((locals.var_vth_shift_dn5 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn5))), ((locals.var_vg_1_dn6 - locals.var_vfb_dn6) - ((locals.var_vth_shift_dn6 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn6))), ((locals.var_vg_1_dn7 - locals.var_vfb_dn7) - ((locals.var_vth_shift_dn7 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn7))), ((locals.var_vg_1_dn8 - locals.var_vfb_dn8) - ((locals.var_vth_shift_dn8 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn8))), ((locals.var_vg_1_dn9 - locals.var_vfb_dn9) - ((locals.var_vth_shift_dn9 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn9))), ((locals.var_vg_1_dn10 - locals.var_vfb_dn10) - ((locals.var_vth_shift_dn10 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn10))), ((locals.var_vg_1_dn11 - locals.var_vfb_dn11) - ((locals.var_vth_shift_dn11 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn11))), ((locals.var_vg_1_dn12 - locals.var_vfb_dn12) - ((locals.var_vth_shift_dn12 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn12))), ((locals.var_vg_1_dn13 - locals.var_vfb_dn13) - ((locals.var_vth_shift_dn13 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn13))), ((locals.var_vg_1_dn14 - locals.var_vfb_dn14) - ((locals.var_vth_shift_dn14 * locals.var_inv_nvt) + (locals.var_vth_shift * locals.var_inv_nvt_dn14))),)
    } else {
        (locals.var_vgfb, locals.var_vgfb_dn0, locals.var_vgfb_dn2, locals.var_vgfb_dn3, locals.var_vgfb_dn4, locals.var_vgfb_dn5, locals.var_vgfb_dn6, locals.var_vgfb_dn7, locals.var_vgfb_dn8, locals.var_vgfb_dn9, locals.var_vgfb_dn10, locals.var_vgfb_dn11, locals.var_vgfb_dn12, locals.var_vgfb_dn13, locals.var_vgfb_dn14,)
    }
};
        locals.var_vgfb = assign31840_e42101;
        locals.var_vgfb_dn0 = assign31840_e42101_d_n0;
        locals.var_vgfb_dn2 = assign31840_e42101_d_n2;
        locals.var_vgfb_dn3 = assign31840_e42101_d_n3;
        locals.var_vgfb_dn4 = assign31840_e42101_d_n4;
        locals.var_vgfb_dn5 = assign31840_e42101_d_n5;
        locals.var_vgfb_dn6 = assign31840_e42101_d_n6;
        locals.var_vgfb_dn7 = assign31840_e42101_d_n7;
        locals.var_vgfb_dn8 = assign31840_e42101_d_n8;
        locals.var_vgfb_dn9 = assign31840_e42101_d_n9;
        locals.var_vgfb_dn10 = assign31840_e42101_d_n10;
        locals.var_vgfb_dn11 = assign31840_e42101_d_n11;
        locals.var_vgfb_dn12 = assign31840_e42101_d_n12;
        locals.var_vgfb_dn13 = assign31840_e42101_d_n13;
        locals.var_vgfb_dn14 = assign31840_e42101_d_n14;
        locals.var_vgfb_rv = 0.0;

        let (assign31850_e42114,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31850_e42108: f64 = (-p.p960);
        let assign31850_e42109: f64 = (locals.var_leff).powf(assign31850_e42108);
        let assign31850_e42110: f64 = (p.p959 * assign31850_e42109);
        let assign31850_e42111: f64 = (1.0 + assign31850_e42110);
        let assign31850_e42112: f64 = (p.p958 * assign31850_e42111);
        (assign31850_e42112,)
    } else {
        (locals.var_dgammaedge_i,)
    }
};
        locals.var_dgammaedge_i = assign31850_e42114;
        locals.var_dgammaedge_i_rv = 0.0;

        let (assign31860_e42129, assign31860_e42129_d_n0, assign31860_e42129_d_n2, assign31860_e42129_d_n3, assign31860_e42129_d_n4, assign31860_e42129_d_n5, assign31860_e42129_d_n6, assign31860_e42129_d_n7, assign31860_e42129_d_n8, assign31860_e42129_d_n9, assign31860_e42129_d_n10, assign31860_e42129_d_n11, assign31860_e42129_d_n12, assign31860_e42129_d_n13, assign31860_e42129_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31860_e42118: f64 = (2.0 * 1.60219e-19);
        let assign31860_e42120: f64 = (assign31860_e42118 * locals.var_epssi);
        let assign31860_e42122: f64 = (assign31860_e42120 * locals.var_ndepedge_i);
        let assign31860_e42124: f64 = (assign31860_e42122 * locals.var_inv_nvt);
        let assign31860_e42125: f64 = (assign31860_e42124).sqrt();
        let assign31860_e42127: f64 = (assign31860_e42125 / locals.var_cox);
        (assign31860_e42127, (((assign31860_e42122 * locals.var_inv_nvt_dn0) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn2) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn3) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn4) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn5) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn6) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn7) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn8) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn9) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn10) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn11) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn12) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn13) / (2.0 * assign31860_e42125)) / locals.var_cox), (((assign31860_e42122 * locals.var_inv_nvt_dn14) / (2.0 * assign31860_e42125)) / locals.var_cox),)
    } else {
        (locals.var_gam_edge, locals.var_gam_edge_dn0, locals.var_gam_edge_dn2, locals.var_gam_edge_dn3, locals.var_gam_edge_dn4, locals.var_gam_edge_dn5, locals.var_gam_edge_dn6, locals.var_gam_edge_dn7, locals.var_gam_edge_dn8, locals.var_gam_edge_dn9, locals.var_gam_edge_dn10, locals.var_gam_edge_dn11, locals.var_gam_edge_dn12, locals.var_gam_edge_dn13, locals.var_gam_edge_dn14,)
    }
};
        locals.var_gam_edge = assign31860_e42129;
        locals.var_gam_edge_dn0 = assign31860_e42129_d_n0;
        locals.var_gam_edge_dn2 = assign31860_e42129_d_n2;
        locals.var_gam_edge_dn3 = assign31860_e42129_d_n3;
        locals.var_gam_edge_dn4 = assign31860_e42129_d_n4;
        locals.var_gam_edge_dn5 = assign31860_e42129_d_n5;
        locals.var_gam_edge_dn6 = assign31860_e42129_d_n6;
        locals.var_gam_edge_dn7 = assign31860_e42129_d_n7;
        locals.var_gam_edge_dn8 = assign31860_e42129_d_n8;
        locals.var_gam_edge_dn9 = assign31860_e42129_d_n9;
        locals.var_gam_edge_dn10 = assign31860_e42129_d_n10;
        locals.var_gam_edge_dn11 = assign31860_e42129_d_n11;
        locals.var_gam_edge_dn12 = assign31860_e42129_d_n12;
        locals.var_gam_edge_dn13 = assign31860_e42129_d_n13;
        locals.var_gam_edge_dn14 = assign31860_e42129_d_n14;
        locals.var_gam_edge_rv = 0.0;

        let (assign31870_e42137, assign31870_e42137_d_n0, assign31870_e42137_d_n2, assign31870_e42137_d_n3, assign31870_e42137_d_n4, assign31870_e42137_d_n5, assign31870_e42137_d_n6, assign31870_e42137_d_n7, assign31870_e42137_d_n8, assign31870_e42137_d_n9, assign31870_e42137_d_n10, assign31870_e42137_d_n11, assign31870_e42137_d_n12, assign31870_e42137_d_n13, assign31870_e42137_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31870_e42134: f64 = (1.0 + locals.var_dgammaedge_i);
        let assign31870_e42135: f64 = (locals.var_gam_edge * assign31870_e42134);
        (assign31870_e42135, (locals.var_gam_edge_dn0 * assign31870_e42134), (locals.var_gam_edge_dn2 * assign31870_e42134), (locals.var_gam_edge_dn3 * assign31870_e42134), (locals.var_gam_edge_dn4 * assign31870_e42134), (locals.var_gam_edge_dn5 * assign31870_e42134), (locals.var_gam_edge_dn6 * assign31870_e42134), (locals.var_gam_edge_dn7 * assign31870_e42134), (locals.var_gam_edge_dn8 * assign31870_e42134), (locals.var_gam_edge_dn9 * assign31870_e42134), (locals.var_gam_edge_dn10 * assign31870_e42134), (locals.var_gam_edge_dn11 * assign31870_e42134), (locals.var_gam_edge_dn12 * assign31870_e42134), (locals.var_gam_edge_dn13 * assign31870_e42134), (locals.var_gam_edge_dn14 * assign31870_e42134),)
    } else {
        (locals.var_gam_edge, locals.var_gam_edge_dn0, locals.var_gam_edge_dn2, locals.var_gam_edge_dn3, locals.var_gam_edge_dn4, locals.var_gam_edge_dn5, locals.var_gam_edge_dn6, locals.var_gam_edge_dn7, locals.var_gam_edge_dn8, locals.var_gam_edge_dn9, locals.var_gam_edge_dn10, locals.var_gam_edge_dn11, locals.var_gam_edge_dn12, locals.var_gam_edge_dn13, locals.var_gam_edge_dn14,)
    }
};
        locals.var_gam_edge = assign31870_e42137;
        locals.var_gam_edge_dn0 = assign31870_e42137_d_n0;
        locals.var_gam_edge_dn2 = assign31870_e42137_d_n2;
        locals.var_gam_edge_dn3 = assign31870_e42137_d_n3;
        locals.var_gam_edge_dn4 = assign31870_e42137_d_n4;
        locals.var_gam_edge_dn5 = assign31870_e42137_d_n5;
        locals.var_gam_edge_dn6 = assign31870_e42137_d_n6;
        locals.var_gam_edge_dn7 = assign31870_e42137_d_n7;
        locals.var_gam_edge_dn8 = assign31870_e42137_d_n8;
        locals.var_gam_edge_dn9 = assign31870_e42137_d_n9;
        locals.var_gam_edge_dn10 = assign31870_e42137_d_n10;
        locals.var_gam_edge_dn11 = assign31870_e42137_d_n11;
        locals.var_gam_edge_dn12 = assign31870_e42137_d_n12;
        locals.var_gam_edge_dn13 = assign31870_e42137_d_n13;
        locals.var_gam_edge_dn14 = assign31870_e42137_d_n14;
        locals.var_gam_edge_rv = 0.0;

        let (assign31880_e42143, assign31880_e42143_d_n0, assign31880_e42143_d_n2, assign31880_e42143_d_n3, assign31880_e42143_d_n4, assign31880_e42143_d_n5, assign31880_e42143_d_n6, assign31880_e42143_d_n7, assign31880_e42143_d_n8, assign31880_e42143_d_n9, assign31880_e42143_d_n10, assign31880_e42143_d_n11, assign31880_e42143_d_n12, assign31880_e42143_d_n13, assign31880_e42143_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31880_e42141: f64 = (locals.var_phib_edge / locals.var_n);
        (assign31880_e42141, (((locals.var_phib_edge_dn0 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn0)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn2 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn2)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn3 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn3)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn4 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn4)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn5 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn5)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn6 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn6)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn7 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn7)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn8 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn8)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn9 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn9)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn10 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn10)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn11 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn11)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn12 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn12)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn13 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn13)) / (locals.var_n * locals.var_n)), (((locals.var_phib_edge_dn14 * locals.var_n) - (locals.var_phib_edge * locals.var_n_dn14)) / (locals.var_n * locals.var_n)),)
    } else {
        (locals.var_phib_n_edge, locals.var_phib_n_edge_dn0, locals.var_phib_n_edge_dn2, locals.var_phib_n_edge_dn3, locals.var_phib_n_edge_dn4, locals.var_phib_n_edge_dn5, locals.var_phib_n_edge_dn6, locals.var_phib_n_edge_dn7, locals.var_phib_n_edge_dn8, locals.var_phib_n_edge_dn9, locals.var_phib_n_edge_dn10, locals.var_phib_n_edge_dn11, locals.var_phib_n_edge_dn12, locals.var_phib_n_edge_dn13, locals.var_phib_n_edge_dn14,)
    }
};
        locals.var_phib_n_edge = assign31880_e42143;
        locals.var_phib_n_edge_dn0 = assign31880_e42143_d_n0;
        locals.var_phib_n_edge_dn2 = assign31880_e42143_d_n2;
        locals.var_phib_n_edge_dn3 = assign31880_e42143_d_n3;
        locals.var_phib_n_edge_dn4 = assign31880_e42143_d_n4;
        locals.var_phib_n_edge_dn5 = assign31880_e42143_d_n5;
        locals.var_phib_n_edge_dn6 = assign31880_e42143_d_n6;
        locals.var_phib_n_edge_dn7 = assign31880_e42143_d_n7;
        locals.var_phib_n_edge_dn8 = assign31880_e42143_d_n8;
        locals.var_phib_n_edge_dn9 = assign31880_e42143_d_n9;
        locals.var_phib_n_edge_dn10 = assign31880_e42143_d_n10;
        locals.var_phib_n_edge_dn11 = assign31880_e42143_d_n11;
        locals.var_phib_n_edge_dn12 = assign31880_e42143_d_n12;
        locals.var_phib_n_edge_dn13 = assign31880_e42143_d_n13;
        locals.var_phib_n_edge_dn14 = assign31880_e42143_d_n14;
        locals.var_phib_n_edge_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_102(
        locals: &mut StampLocals,
    ) {
        let (assign31890_e42149, assign31890_e42149_d_n0, assign31890_e42149_d_n2, assign31890_e42149_d_n3, assign31890_e42149_d_n4, assign31890_e42149_d_n5, assign31890_e42149_d_n6, assign31890_e42149_d_n7, assign31890_e42149_d_n8, assign31890_e42149_d_n9, assign31890_e42149_d_n10, assign31890_e42149_d_n11, assign31890_e42149_d_n12, assign31890_e42149_d_n13, assign31890_e42149_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31890_e42147: f64 = 1.0;
        (assign31890_e42147, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign31890_e42149;
        locals.var_t1_dn0 = assign31890_e42149_d_n0;
        locals.var_t1_dn2 = assign31890_e42149_d_n2;
        locals.var_t1_dn3 = assign31890_e42149_d_n3;
        locals.var_t1_dn4 = assign31890_e42149_d_n4;
        locals.var_t1_dn5 = assign31890_e42149_d_n5;
        locals.var_t1_dn6 = assign31890_e42149_d_n6;
        locals.var_t1_dn7 = assign31890_e42149_d_n7;
        locals.var_t1_dn8 = assign31890_e42149_d_n8;
        locals.var_t1_dn9 = assign31890_e42149_d_n9;
        locals.var_t1_dn10 = assign31890_e42149_d_n10;
        locals.var_t1_dn11 = assign31890_e42149_d_n11;
        locals.var_t1_dn12 = assign31890_e42149_d_n12;
        locals.var_t1_dn13 = assign31890_e42149_d_n13;
        locals.var_t1_dn14 = assign31890_e42149_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign31900_e42155, assign31900_e42155_d_n0, assign31900_e42155_d_n2, assign31900_e42155_d_n3, assign31900_e42155_d_n4, assign31900_e42155_d_n5, assign31900_e42155_d_n6, assign31900_e42155_d_n7, assign31900_e42155_d_n8, assign31900_e42155_d_n9, assign31900_e42155_d_n10, assign31900_e42155_d_n11, assign31900_e42155_d_n12, assign31900_e42155_d_n13, assign31900_e42155_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31900_e42153: f64 = (locals.var_vgfb / locals.var_t1);
        (assign31900_e42153, (((locals.var_vgfb_dn0 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn2 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn3 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn4 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn5 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn6 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn7 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn8 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn9 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn10 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn11 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn12 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn13 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn13)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfb_dn14 * locals.var_t1) - (locals.var_vgfb * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_vgfbpd, locals.var_vgfbpd_dn0, locals.var_vgfbpd_dn2, locals.var_vgfbpd_dn3, locals.var_vgfbpd_dn4, locals.var_vgfbpd_dn5, locals.var_vgfbpd_dn6, locals.var_vgfbpd_dn7, locals.var_vgfbpd_dn8, locals.var_vgfbpd_dn9, locals.var_vgfbpd_dn10, locals.var_vgfbpd_dn11, locals.var_vgfbpd_dn12, locals.var_vgfbpd_dn13, locals.var_vgfbpd_dn14,)
    }
};
        locals.var_vgfbpd = assign31900_e42155;
        locals.var_vgfbpd_dn0 = assign31900_e42155_d_n0;
        locals.var_vgfbpd_dn2 = assign31900_e42155_d_n2;
        locals.var_vgfbpd_dn3 = assign31900_e42155_d_n3;
        locals.var_vgfbpd_dn4 = assign31900_e42155_d_n4;
        locals.var_vgfbpd_dn5 = assign31900_e42155_d_n5;
        locals.var_vgfbpd_dn6 = assign31900_e42155_d_n6;
        locals.var_vgfbpd_dn7 = assign31900_e42155_d_n7;
        locals.var_vgfbpd_dn8 = assign31900_e42155_d_n8;
        locals.var_vgfbpd_dn9 = assign31900_e42155_d_n9;
        locals.var_vgfbpd_dn10 = assign31900_e42155_d_n10;
        locals.var_vgfbpd_dn11 = assign31900_e42155_d_n11;
        locals.var_vgfbpd_dn12 = assign31900_e42155_d_n12;
        locals.var_vgfbpd_dn13 = assign31900_e42155_d_n13;
        locals.var_vgfbpd_dn14 = assign31900_e42155_d_n14;
        locals.var_vgfbpd_rv = 0.0;

        let (assign31910_e42161, assign31910_e42161_d_n0, assign31910_e42161_d_n2, assign31910_e42161_d_n3, assign31910_e42161_d_n4, assign31910_e42161_d_n5, assign31910_e42161_d_n6, assign31910_e42161_d_n7, assign31910_e42161_d_n8, assign31910_e42161_d_n9, assign31910_e42161_d_n10, assign31910_e42161_d_n11, assign31910_e42161_d_n12, assign31910_e42161_d_n13, assign31910_e42161_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31910_e42159: f64 = (locals.var_gam_edge / locals.var_t1);
        (assign31910_e42159, (((locals.var_gam_edge_dn0 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn2 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn3 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn4 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn5 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn6 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn7 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn8 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn9 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn10 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn11 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn12 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn13 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn13)) / (locals.var_t1 * locals.var_t1)), (((locals.var_gam_edge_dn14 * locals.var_t1) - (locals.var_gam_edge * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_gammapd, locals.var_gammapd_dn0, locals.var_gammapd_dn2, locals.var_gammapd_dn3, locals.var_gammapd_dn4, locals.var_gammapd_dn5, locals.var_gammapd_dn6, locals.var_gammapd_dn7, locals.var_gammapd_dn8, locals.var_gammapd_dn9, locals.var_gammapd_dn10, locals.var_gammapd_dn11, locals.var_gammapd_dn12, locals.var_gammapd_dn13, locals.var_gammapd_dn14,)
    }
};
        locals.var_gammapd = assign31910_e42161;
        locals.var_gammapd_dn0 = assign31910_e42161_d_n0;
        locals.var_gammapd_dn2 = assign31910_e42161_d_n2;
        locals.var_gammapd_dn3 = assign31910_e42161_d_n3;
        locals.var_gammapd_dn4 = assign31910_e42161_d_n4;
        locals.var_gammapd_dn5 = assign31910_e42161_d_n5;
        locals.var_gammapd_dn6 = assign31910_e42161_d_n6;
        locals.var_gammapd_dn7 = assign31910_e42161_d_n7;
        locals.var_gammapd_dn8 = assign31910_e42161_d_n8;
        locals.var_gammapd_dn9 = assign31910_e42161_d_n9;
        locals.var_gammapd_dn10 = assign31910_e42161_d_n10;
        locals.var_gammapd_dn11 = assign31910_e42161_d_n11;
        locals.var_gammapd_dn12 = assign31910_e42161_d_n12;
        locals.var_gammapd_dn13 = assign31910_e42161_d_n13;
        locals.var_gammapd_dn14 = assign31910_e42161_d_n14;
        locals.var_gammapd_rv = 0.0;

        let (assign31920_e42175, assign31920_e42175_d_n0, assign31920_e42175_d_n2, assign31920_e42175_d_n3, assign31920_e42175_d_n4, assign31920_e42175_d_n5, assign31920_e42175_d_n6, assign31920_e42175_d_n7, assign31920_e42175_d_n8, assign31920_e42175_d_n9, assign31920_e42175_d_n10, assign31920_e42175_d_n11, assign31920_e42175_d_n12, assign31920_e42175_d_n13, assign31920_e42175_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31920_e42165: f64 = (0.5 * locals.var_vgfbpd);
        let assign31920_e42170: f64 = (locals.var_gammapd / 1.4142135623730951);
        let assign31920_e42171: f64 = (1.0 + assign31920_e42170);
        let assign31920_e42172: f64 = (3.0 * assign31920_e42171);
        let assign31920_e42173: f64 = (assign31920_e42165 - assign31920_e42172);
        (assign31920_e42173, ((0.5 * locals.var_vgfbpd_dn0) - (3.0 * (locals.var_gammapd_dn0 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn2) - (3.0 * (locals.var_gammapd_dn2 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn3) - (3.0 * (locals.var_gammapd_dn3 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn4) - (3.0 * (locals.var_gammapd_dn4 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn5) - (3.0 * (locals.var_gammapd_dn5 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn6) - (3.0 * (locals.var_gammapd_dn6 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn7) - (3.0 * (locals.var_gammapd_dn7 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn8) - (3.0 * (locals.var_gammapd_dn8 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn9) - (3.0 * (locals.var_gammapd_dn9 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn10) - (3.0 * (locals.var_gammapd_dn10 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn11) - (3.0 * (locals.var_gammapd_dn11 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn12) - (3.0 * (locals.var_gammapd_dn12 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn13) - (3.0 * (locals.var_gammapd_dn13 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn14) - (3.0 * (locals.var_gammapd_dn14 / 1.4142135623730951))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign31920_e42175;
        locals.var_t1_dn0 = assign31920_e42175_d_n0;
        locals.var_t1_dn2 = assign31920_e42175_d_n2;
        locals.var_t1_dn3 = assign31920_e42175_d_n3;
        locals.var_t1_dn4 = assign31920_e42175_d_n4;
        locals.var_t1_dn5 = assign31920_e42175_d_n5;
        locals.var_t1_dn6 = assign31920_e42175_d_n6;
        locals.var_t1_dn7 = assign31920_e42175_d_n7;
        locals.var_t1_dn8 = assign31920_e42175_d_n8;
        locals.var_t1_dn9 = assign31920_e42175_d_n9;
        locals.var_t1_dn10 = assign31920_e42175_d_n10;
        locals.var_t1_dn11 = assign31920_e42175_d_n11;
        locals.var_t1_dn12 = assign31920_e42175_d_n12;
        locals.var_t1_dn13 = assign31920_e42175_d_n13;
        locals.var_t1_dn14 = assign31920_e42175_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign31930_e42188, assign31930_e42188_d_n0, assign31930_e42188_d_n2, assign31930_e42188_d_n3, assign31930_e42188_d_n4, assign31930_e42188_d_n5, assign31930_e42188_d_n6, assign31930_e42188_d_n7, assign31930_e42188_d_n8, assign31930_e42188_d_n9, assign31930_e42188_d_n10, assign31930_e42188_d_n11, assign31930_e42188_d_n12, assign31930_e42188_d_n13, assign31930_e42188_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign31930_e42180: f64 = (locals.var_t1 * locals.var_t1);
        let assign31930_e42183: f64 = (6.0 * locals.var_vgfbpd);
        let assign31930_e42184: f64 = (assign31930_e42180 + assign31930_e42183);
        let assign31930_e42185: f64 = (assign31930_e42184).sqrt();
        let assign31930_e42186: f64 = (locals.var_t1 + assign31930_e42185);
        (assign31930_e42186, (locals.var_t1_dn0 + ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + (6.0 * locals.var_vgfbpd_dn0)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn2 + ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + (6.0 * locals.var_vgfbpd_dn2)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn3 + ((((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) + (6.0 * locals.var_vgfbpd_dn3)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn4 + ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + (6.0 * locals.var_vgfbpd_dn4)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn5 + ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + (6.0 * locals.var_vgfbpd_dn5)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn6 + ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + (6.0 * locals.var_vgfbpd_dn6)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn7 + ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + (6.0 * locals.var_vgfbpd_dn7)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn8 + ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + (6.0 * locals.var_vgfbpd_dn8)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn9 + ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + (6.0 * locals.var_vgfbpd_dn9)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn10 + ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + (6.0 * locals.var_vgfbpd_dn10)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn11 + ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + (6.0 * locals.var_vgfbpd_dn11)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn12 + ((((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) + (6.0 * locals.var_vgfbpd_dn12)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn13 + ((((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) + (6.0 * locals.var_vgfbpd_dn13)) / (2.0 * assign31930_e42185))), (locals.var_t1_dn14 + ((((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) + (6.0 * locals.var_vgfbpd_dn14)) / (2.0 * assign31930_e42185))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign31930_e42188;
        locals.var_t2_dn0 = assign31930_e42188_d_n0;
        locals.var_t2_dn2 = assign31930_e42188_d_n2;
        locals.var_t2_dn3 = assign31930_e42188_d_n3;
        locals.var_t2_dn4 = assign31930_e42188_d_n4;
        locals.var_t2_dn5 = assign31930_e42188_d_n5;
        locals.var_t2_dn6 = assign31930_e42188_d_n6;
        locals.var_t2_dn7 = assign31930_e42188_d_n7;
        locals.var_t2_dn8 = assign31930_e42188_d_n8;
        locals.var_t2_dn9 = assign31930_e42188_d_n9;
        locals.var_t2_dn10 = assign31930_e42188_d_n10;
        locals.var_t2_dn11 = assign31930_e42188_d_n11;
        locals.var_t2_dn12 = assign31930_e42188_d_n12;
        locals.var_t2_dn13 = assign31930_e42188_d_n13;
        locals.var_t2_dn14 = assign31930_e42188_d_n14;
        locals.var_t2_rv = 0.0;

        let assign31940_e42191: f64 = if locals.var_vgfbpd < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard735 = assign31940_e42191;
        locals.var_guard735_rv = 0.0;

        let (assign31950_e42201, assign31950_e42201_d_n0, assign31950_e42201_d_n2, assign31950_e42201_d_n3, assign31950_e42201_d_n4, assign31950_e42201_d_n5, assign31950_e42201_d_n6, assign31950_e42201_d_n7, assign31950_e42201_d_n8, assign31950_e42201_d_n9, assign31950_e42201_d_n10, assign31950_e42201_d_n11, assign31950_e42201_d_n12, assign31950_e42201_d_n13, assign31950_e42201_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard735 != 0.0)) {
        let assign31950_e42197: f64 = (locals.var_vgfbpd - locals.var_t2);
        let assign31950_e42199: f64 = (assign31950_e42197 / locals.var_gammapd);
        (assign31950_e42199, ((((locals.var_vgfbpd_dn0 - locals.var_t2_dn0) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn0)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn2 - locals.var_t2_dn2) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn2)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn3 - locals.var_t2_dn3) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn3)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn4 - locals.var_t2_dn4) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn4)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn5 - locals.var_t2_dn5) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn5)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn6 - locals.var_t2_dn6) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn6)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn7 - locals.var_t2_dn7) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn7)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn8 - locals.var_t2_dn8) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn8)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn9 - locals.var_t2_dn9) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn9)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn10 - locals.var_t2_dn10) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn10)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn11 - locals.var_t2_dn11) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn11)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn12 - locals.var_t2_dn12) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn12)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn13 - locals.var_t2_dn13) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn13)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn14 - locals.var_t2_dn14) * locals.var_gammapd) - (assign31950_e42197 * locals.var_gammapd_dn14)) / (locals.var_gammapd * locals.var_gammapd)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign31950_e42201;
        locals.var_t3_dn0 = assign31950_e42201_d_n0;
        locals.var_t3_dn2 = assign31950_e42201_d_n2;
        locals.var_t3_dn3 = assign31950_e42201_d_n3;
        locals.var_t3_dn4 = assign31950_e42201_d_n4;
        locals.var_t3_dn5 = assign31950_e42201_d_n5;
        locals.var_t3_dn6 = assign31950_e42201_d_n6;
        locals.var_t3_dn7 = assign31950_e42201_d_n7;
        locals.var_t3_dn8 = assign31950_e42201_d_n8;
        locals.var_t3_dn9 = assign31950_e42201_d_n9;
        locals.var_t3_dn10 = assign31950_e42201_d_n10;
        locals.var_t3_dn11 = assign31950_e42201_d_n11;
        locals.var_t3_dn12 = assign31950_e42201_d_n12;
        locals.var_t3_dn13 = assign31950_e42201_d_n13;
        locals.var_t3_dn14 = assign31950_e42201_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign31960_e42217, assign31960_e42217_d_n0, assign31960_e42217_d_n2, assign31960_e42217_d_n3, assign31960_e42217_d_n4, assign31960_e42217_d_n5, assign31960_e42217_d_n6, assign31960_e42217_d_n7, assign31960_e42217_d_n8, assign31960_e42217_d_n9, assign31960_e42217_d_n10, assign31960_e42217_d_n11, assign31960_e42217_d_n12, assign31960_e42217_d_n13, assign31960_e42217_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard735 != 0.0)) {
        let assign31960_e42207: f64 = (1.0 - locals.var_t2);
        let assign31960_e42210: f64 = (locals.var_t3 * locals.var_t3);
        let assign31960_e42211: f64 = (assign31960_e42207 + assign31960_e42210);
        let assign31960_e42213: f64 = (assign31960_e42211).max(1e-38);
        let assign31960_e42214: f64 = (assign31960_e42213).ln();
        let assign31960_e42215: f64 = (-assign31960_e42214);
        (assign31960_e42215, (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn0) + ((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn2) + ((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn3) + ((locals.var_t3_dn3 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn3))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn4) + ((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn5) + ((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn6) + ((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn7) + ((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn8) + ((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn9) + ((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn10) + ((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn11) + ((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn12) + ((locals.var_t3_dn12 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn12))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn13) + ((locals.var_t3_dn13 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn13))) } else { 0.0 } / assign31960_e42213)), (-(if assign31960_e42211 >= 1e-38 { ((-locals.var_t2_dn14) + ((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14))) } else { 0.0 } / assign31960_e42213)),)
    } else {
        (locals.var_psip, locals.var_psip_dn0, locals.var_psip_dn2, locals.var_psip_dn3, locals.var_psip_dn4, locals.var_psip_dn5, locals.var_psip_dn6, locals.var_psip_dn7, locals.var_psip_dn8, locals.var_psip_dn9, locals.var_psip_dn10, locals.var_psip_dn11, locals.var_psip_dn12, locals.var_psip_dn13, locals.var_psip_dn14,)
    }
};
        locals.var_psip = assign31960_e42217;
        locals.var_psip_dn0 = assign31960_e42217_d_n0;
        locals.var_psip_dn2 = assign31960_e42217_d_n2;
        locals.var_psip_dn3 = assign31960_e42217_d_n3;
        locals.var_psip_dn4 = assign31960_e42217_d_n4;
        locals.var_psip_dn5 = assign31960_e42217_d_n5;
        locals.var_psip_dn6 = assign31960_e42217_d_n6;
        locals.var_psip_dn7 = assign31960_e42217_d_n7;
        locals.var_psip_dn8 = assign31960_e42217_d_n8;
        locals.var_psip_dn9 = assign31960_e42217_d_n9;
        locals.var_psip_dn10 = assign31960_e42217_d_n10;
        locals.var_psip_dn11 = assign31960_e42217_d_n11;
        locals.var_psip_dn12 = assign31960_e42217_d_n12;
        locals.var_psip_dn13 = assign31960_e42217_d_n13;
        locals.var_psip_dn14 = assign31960_e42217_d_n14;
        locals.var_psip_rv = 0.0;

        let (assign31970_e42226, assign31970_e42226_d_n0, assign31970_e42226_d_n2, assign31970_e42226_d_n3, assign31970_e42226_d_n4, assign31970_e42226_d_n5, assign31970_e42226_d_n6, assign31970_e42226_d_n7, assign31970_e42226_d_n8, assign31970_e42226_d_n9, assign31970_e42226_d_n10, assign31970_e42226_d_n11, assign31970_e42226_d_n12, assign31970_e42226_d_n13, assign31970_e42226_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard735 == 0.0)) {
        let assign31970_e42223: f64 = (-locals.var_t2);
        let assign31970_e42224: f64 = { let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign31970_e42224, ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn0)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn2)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn12)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn13)), ({ let limited_exp_arg = assign31970_e42223; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign31970_e42226;
        locals.var_t3_dn0 = assign31970_e42226_d_n0;
        locals.var_t3_dn2 = assign31970_e42226_d_n2;
        locals.var_t3_dn3 = assign31970_e42226_d_n3;
        locals.var_t3_dn4 = assign31970_e42226_d_n4;
        locals.var_t3_dn5 = assign31970_e42226_d_n5;
        locals.var_t3_dn6 = assign31970_e42226_d_n6;
        locals.var_t3_dn7 = assign31970_e42226_d_n7;
        locals.var_t3_dn8 = assign31970_e42226_d_n8;
        locals.var_t3_dn9 = assign31970_e42226_d_n9;
        locals.var_t3_dn10 = assign31970_e42226_d_n10;
        locals.var_t3_dn11 = assign31970_e42226_d_n11;
        locals.var_t3_dn12 = assign31970_e42226_d_n12;
        locals.var_t3_dn13 = assign31970_e42226_d_n13;
        locals.var_t3_dn14 = assign31970_e42226_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign31980_e42235, assign31980_e42235_d_n0, assign31980_e42235_d_n2, assign31980_e42235_d_n3, assign31980_e42235_d_n4, assign31980_e42235_d_n5, assign31980_e42235_d_n6, assign31980_e42235_d_n7, assign31980_e42235_d_n8, assign31980_e42235_d_n9, assign31980_e42235_d_n10, assign31980_e42235_d_n11, assign31980_e42235_d_n12, assign31980_e42235_d_n13, assign31980_e42235_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard735 == 0.0)) {
        let assign31980_e42233: f64 = (0.5 * locals.var_gammapd);
        (assign31980_e42233, (0.5 * locals.var_gammapd_dn0), (0.5 * locals.var_gammapd_dn2), (0.5 * locals.var_gammapd_dn3), (0.5 * locals.var_gammapd_dn4), (0.5 * locals.var_gammapd_dn5), (0.5 * locals.var_gammapd_dn6), (0.5 * locals.var_gammapd_dn7), (0.5 * locals.var_gammapd_dn8), (0.5 * locals.var_gammapd_dn9), (0.5 * locals.var_gammapd_dn10), (0.5 * locals.var_gammapd_dn11), (0.5 * locals.var_gammapd_dn12), (0.5 * locals.var_gammapd_dn13), (0.5 * locals.var_gammapd_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign31980_e42235;
        locals.var_t1_dn0 = assign31980_e42235_d_n0;
        locals.var_t1_dn2 = assign31980_e42235_d_n2;
        locals.var_t1_dn3 = assign31980_e42235_d_n3;
        locals.var_t1_dn4 = assign31980_e42235_d_n4;
        locals.var_t1_dn5 = assign31980_e42235_d_n5;
        locals.var_t1_dn6 = assign31980_e42235_d_n6;
        locals.var_t1_dn7 = assign31980_e42235_d_n7;
        locals.var_t1_dn8 = assign31980_e42235_d_n8;
        locals.var_t1_dn9 = assign31980_e42235_d_n9;
        locals.var_t1_dn10 = assign31980_e42235_d_n10;
        locals.var_t1_dn11 = assign31980_e42235_d_n11;
        locals.var_t1_dn12 = assign31980_e42235_d_n12;
        locals.var_t1_dn13 = assign31980_e42235_d_n13;
        locals.var_t1_dn14 = assign31980_e42235_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign31990_e42253, assign31990_e42253_d_n0, assign31990_e42253_d_n2, assign31990_e42253_d_n3, assign31990_e42253_d_n4, assign31990_e42253_d_n5, assign31990_e42253_d_n6, assign31990_e42253_d_n7, assign31990_e42253_d_n8, assign31990_e42253_d_n9, assign31990_e42253_d_n10, assign31990_e42253_d_n11, assign31990_e42253_d_n12, assign31990_e42253_d_n13, assign31990_e42253_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard735 == 0.0)) {
        let assign31990_e42242: f64 = (locals.var_vgfbpd - 1.0);
        let assign31990_e42244: f64 = (assign31990_e42242 + locals.var_t3);
        let assign31990_e42247: f64 = (locals.var_t1 * locals.var_t1);
        let assign31990_e42248: f64 = (assign31990_e42244 + assign31990_e42247);
        let assign31990_e42249: f64 = (assign31990_e42248).sqrt();
        let assign31990_e42251: f64 = (assign31990_e42249 - locals.var_t1);
        (assign31990_e42251, ((((locals.var_vgfbpd_dn0 + locals.var_t3_dn0) + ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn0), ((((locals.var_vgfbpd_dn2 + locals.var_t3_dn2) + ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn2), ((((locals.var_vgfbpd_dn3 + locals.var_t3_dn3) + ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn3), ((((locals.var_vgfbpd_dn4 + locals.var_t3_dn4) + ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn4), ((((locals.var_vgfbpd_dn5 + locals.var_t3_dn5) + ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn5), ((((locals.var_vgfbpd_dn6 + locals.var_t3_dn6) + ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn6), ((((locals.var_vgfbpd_dn7 + locals.var_t3_dn7) + ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn7), ((((locals.var_vgfbpd_dn8 + locals.var_t3_dn8) + ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn8), ((((locals.var_vgfbpd_dn9 + locals.var_t3_dn9) + ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn9), ((((locals.var_vgfbpd_dn10 + locals.var_t3_dn10) + ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn10), ((((locals.var_vgfbpd_dn11 + locals.var_t3_dn11) + ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn11), ((((locals.var_vgfbpd_dn12 + locals.var_t3_dn12) + ((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn12), ((((locals.var_vgfbpd_dn13 + locals.var_t3_dn13) + ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn13), ((((locals.var_vgfbpd_dn14 + locals.var_t3_dn14) + ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14))) / (2.0 * assign31990_e42249)) - locals.var_t1_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign31990_e42253;
        locals.var_t2_dn0 = assign31990_e42253_d_n0;
        locals.var_t2_dn2 = assign31990_e42253_d_n2;
        locals.var_t2_dn3 = assign31990_e42253_d_n3;
        locals.var_t2_dn4 = assign31990_e42253_d_n4;
        locals.var_t2_dn5 = assign31990_e42253_d_n5;
        locals.var_t2_dn6 = assign31990_e42253_d_n6;
        locals.var_t2_dn7 = assign31990_e42253_d_n7;
        locals.var_t2_dn8 = assign31990_e42253_d_n8;
        locals.var_t2_dn9 = assign31990_e42253_d_n9;
        locals.var_t2_dn10 = assign31990_e42253_d_n10;
        locals.var_t2_dn11 = assign31990_e42253_d_n11;
        locals.var_t2_dn12 = assign31990_e42253_d_n12;
        locals.var_t2_dn13 = assign31990_e42253_d_n13;
        locals.var_t2_dn14 = assign31990_e42253_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign32000_e42266, assign32000_e42266_d_n0, assign32000_e42266_d_n2, assign32000_e42266_d_n3, assign32000_e42266_d_n4, assign32000_e42266_d_n5, assign32000_e42266_d_n6, assign32000_e42266_d_n7, assign32000_e42266_d_n8, assign32000_e42266_d_n9, assign32000_e42266_d_n10, assign32000_e42266_d_n11, assign32000_e42266_d_n12, assign32000_e42266_d_n13, assign32000_e42266_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard735 == 0.0)) {
        let assign32000_e42260: f64 = (locals.var_t2 * locals.var_t2);
        let assign32000_e42262: f64 = (assign32000_e42260 + 1.0);
        let assign32000_e42264: f64 = (assign32000_e42262 - locals.var_t3);
        (assign32000_e42264, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) - locals.var_t3_dn0), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) - locals.var_t3_dn2), (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) - locals.var_t3_dn3), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) - locals.var_t3_dn4), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) - locals.var_t3_dn5), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) - locals.var_t3_dn6), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) - locals.var_t3_dn7), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) - locals.var_t3_dn8), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) - locals.var_t3_dn9), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) - locals.var_t3_dn10), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) - locals.var_t3_dn11), (((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)) - locals.var_t3_dn12), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) - locals.var_t3_dn13), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) - locals.var_t3_dn14),)
    } else {
        (locals.var_psip, locals.var_psip_dn0, locals.var_psip_dn2, locals.var_psip_dn3, locals.var_psip_dn4, locals.var_psip_dn5, locals.var_psip_dn6, locals.var_psip_dn7, locals.var_psip_dn8, locals.var_psip_dn9, locals.var_psip_dn10, locals.var_psip_dn11, locals.var_psip_dn12, locals.var_psip_dn13, locals.var_psip_dn14,)
    }
};
        locals.var_psip = assign32000_e42266;
        locals.var_psip_dn0 = assign32000_e42266_d_n0;
        locals.var_psip_dn2 = assign32000_e42266_d_n2;
        locals.var_psip_dn3 = assign32000_e42266_d_n3;
        locals.var_psip_dn4 = assign32000_e42266_d_n4;
        locals.var_psip_dn5 = assign32000_e42266_d_n5;
        locals.var_psip_dn6 = assign32000_e42266_d_n6;
        locals.var_psip_dn7 = assign32000_e42266_d_n7;
        locals.var_psip_dn8 = assign32000_e42266_d_n8;
        locals.var_psip_dn9 = assign32000_e42266_d_n9;
        locals.var_psip_dn10 = assign32000_e42266_d_n10;
        locals.var_psip_dn11 = assign32000_e42266_d_n11;
        locals.var_psip_dn12 = assign32000_e42266_d_n12;
        locals.var_psip_dn13 = assign32000_e42266_d_n13;
        locals.var_psip_dn14 = assign32000_e42266_d_n14;
        locals.var_psip_rv = 0.0;

        let (assign32010_e42289, assign32010_e42289_d_n0, assign32010_e42289_d_n2, assign32010_e42289_d_n3, assign32010_e42289_d_n4, assign32010_e42289_d_n5, assign32010_e42289_d_n6, assign32010_e42289_d_n7, assign32010_e42289_d_n8, assign32010_e42289_d_n9, assign32010_e42289_d_n10, assign32010_e42289_d_n11, assign32010_e42289_d_n12, assign32010_e42289_d_n13, assign32010_e42289_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32010_e42271: f64 = (locals.var_psip + 1.0);
        let assign32010_e42274: f64 = (locals.var_psip - 1.0);
        let assign32010_e42277: f64 = (locals.var_psip - 1.0);
        let assign32010_e42278: f64 = (assign32010_e42274 * assign32010_e42277);
        let assign32010_e42281: f64 = (0.25 * 2.0);
        let assign32010_e42283: f64 = (assign32010_e42281 * 2.0);
        let assign32010_e42284: f64 = (assign32010_e42278 + assign32010_e42283);
        let assign32010_e42285: f64 = (assign32010_e42284).sqrt();
        let assign32010_e42286: f64 = (assign32010_e42271 + assign32010_e42285);
        let assign32010_e42287: f64 = (0.5 * assign32010_e42286);
        (assign32010_e42287, (0.5 * (locals.var_psip_dn0 + (((locals.var_psip_dn0 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn0)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn2 + (((locals.var_psip_dn2 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn2)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn3)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn4)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn5)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn6)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn7)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn8)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn9)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn10)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn11)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn12 + (((locals.var_psip_dn12 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn12)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn13 + (((locals.var_psip_dn13 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn13)) / (2.0 * assign32010_e42285)))), (0.5 * (locals.var_psip_dn14 + (((locals.var_psip_dn14 * assign32010_e42277) + (assign32010_e42274 * locals.var_psip_dn14)) / (2.0 * assign32010_e42285)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign32010_e42289;
        locals.var_t8_dn0 = assign32010_e42289_d_n0;
        locals.var_t8_dn2 = assign32010_e42289_d_n2;
        locals.var_t8_dn3 = assign32010_e42289_d_n3;
        locals.var_t8_dn4 = assign32010_e42289_d_n4;
        locals.var_t8_dn5 = assign32010_e42289_d_n5;
        locals.var_t8_dn6 = assign32010_e42289_d_n6;
        locals.var_t8_dn7 = assign32010_e42289_d_n7;
        locals.var_t8_dn8 = assign32010_e42289_d_n8;
        locals.var_t8_dn9 = assign32010_e42289_d_n9;
        locals.var_t8_dn10 = assign32010_e42289_d_n10;
        locals.var_t8_dn11 = assign32010_e42289_d_n11;
        locals.var_t8_dn12 = assign32010_e42289_d_n12;
        locals.var_t8_dn13 = assign32010_e42289_d_n13;
        locals.var_t8_dn14 = assign32010_e42289_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign32020_e42294, assign32020_e42294_d_n0, assign32020_e42294_d_n2, assign32020_e42294_d_n3, assign32020_e42294_d_n4, assign32020_e42294_d_n5, assign32020_e42294_d_n6, assign32020_e42294_d_n7, assign32020_e42294_d_n8, assign32020_e42294_d_n9, assign32020_e42294_d_n10, assign32020_e42294_d_n11, assign32020_e42294_d_n12, assign32020_e42294_d_n13, assign32020_e42294_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32020_e42292: f64 = (locals.var_t8).sqrt();
        (assign32020_e42292, (locals.var_t8_dn0 / (2.0 * assign32020_e42292)), (locals.var_t8_dn2 / (2.0 * assign32020_e42292)), (locals.var_t8_dn3 / (2.0 * assign32020_e42292)), (locals.var_t8_dn4 / (2.0 * assign32020_e42292)), (locals.var_t8_dn5 / (2.0 * assign32020_e42292)), (locals.var_t8_dn6 / (2.0 * assign32020_e42292)), (locals.var_t8_dn7 / (2.0 * assign32020_e42292)), (locals.var_t8_dn8 / (2.0 * assign32020_e42292)), (locals.var_t8_dn9 / (2.0 * assign32020_e42292)), (locals.var_t8_dn10 / (2.0 * assign32020_e42292)), (locals.var_t8_dn11 / (2.0 * assign32020_e42292)), (locals.var_t8_dn12 / (2.0 * assign32020_e42292)), (locals.var_t8_dn13 / (2.0 * assign32020_e42292)), (locals.var_t8_dn14 / (2.0 * assign32020_e42292)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn0, locals.var_sqrtpsip_dn2, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11, locals.var_sqrtpsip_dn12, locals.var_sqrtpsip_dn13, locals.var_sqrtpsip_dn14,)
    }
};
        locals.var_sqrtpsip = assign32020_e42294;
        locals.var_sqrtpsip_dn0 = assign32020_e42294_d_n0;
        locals.var_sqrtpsip_dn2 = assign32020_e42294_d_n2;
        locals.var_sqrtpsip_dn3 = assign32020_e42294_d_n3;
        locals.var_sqrtpsip_dn4 = assign32020_e42294_d_n4;
        locals.var_sqrtpsip_dn5 = assign32020_e42294_d_n5;
        locals.var_sqrtpsip_dn6 = assign32020_e42294_d_n6;
        locals.var_sqrtpsip_dn7 = assign32020_e42294_d_n7;
        locals.var_sqrtpsip_dn8 = assign32020_e42294_d_n8;
        locals.var_sqrtpsip_dn9 = assign32020_e42294_d_n9;
        locals.var_sqrtpsip_dn10 = assign32020_e42294_d_n10;
        locals.var_sqrtpsip_dn11 = assign32020_e42294_d_n11;
        locals.var_sqrtpsip_dn12 = assign32020_e42294_d_n12;
        locals.var_sqrtpsip_dn13 = assign32020_e42294_d_n13;
        locals.var_sqrtpsip_dn14 = assign32020_e42294_d_n14;
        locals.var_sqrtpsip_rv = 0.0;

        let (assign32030_e42306, assign32030_e42306_d_n0, assign32030_e42306_d_n2, assign32030_e42306_d_n3, assign32030_e42306_d_n4, assign32030_e42306_d_n5, assign32030_e42306_d_n6, assign32030_e42306_d_n7, assign32030_e42306_d_n8, assign32030_e42306_d_n9, assign32030_e42306_d_n10, assign32030_e42306_d_n11, assign32030_e42306_d_n12, assign32030_e42306_d_n13, assign32030_e42306_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32030_e42300: f64 = (2.0 * locals.var_sqrtpsip);
        let assign32030_e42301: f64 = (locals.var_gam_edge / assign32030_e42300);
        let assign32030_e42302: f64 = (1.0 + assign32030_e42301);
        let assign32030_e42304: f64 = (assign32030_e42302 / locals.var_gam_edge);
        (assign32030_e42304, ((((((locals.var_gam_edge_dn0 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn0))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn0)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn2 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn2))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn2)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn3 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn3))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn3)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn4 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn4))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn4)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn5 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn5))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn5)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn6 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn6))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn6)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn7 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn7))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn7)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn8 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn8))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn8)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn9 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn9))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn9)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn10 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn10))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn10)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn11 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn11))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn11)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn12 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn12))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn12)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn13 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn13))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn13)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn14 * assign32030_e42300) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn14))) / (assign32030_e42300 * assign32030_e42300)) * locals.var_gam_edge) - (assign32030_e42302 * locals.var_gam_edge_dn14)) / (locals.var_gam_edge * locals.var_gam_edge)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign32030_e42306;
        locals.var_t0_dn0 = assign32030_e42306_d_n0;
        locals.var_t0_dn2 = assign32030_e42306_d_n2;
        locals.var_t0_dn3 = assign32030_e42306_d_n3;
        locals.var_t0_dn4 = assign32030_e42306_d_n4;
        locals.var_t0_dn5 = assign32030_e42306_d_n5;
        locals.var_t0_dn6 = assign32030_e42306_d_n6;
        locals.var_t0_dn7 = assign32030_e42306_d_n7;
        locals.var_t0_dn8 = assign32030_e42306_d_n8;
        locals.var_t0_dn9 = assign32030_e42306_d_n9;
        locals.var_t0_dn10 = assign32030_e42306_d_n10;
        locals.var_t0_dn11 = assign32030_e42306_d_n11;
        locals.var_t0_dn12 = assign32030_e42306_d_n12;
        locals.var_t0_dn13 = assign32030_e42306_d_n13;
        locals.var_t0_dn14 = assign32030_e42306_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign32040_e42316, assign32040_e42316_d_n0, assign32040_e42316_d_n2, assign32040_e42316_d_n3, assign32040_e42316_d_n4, assign32040_e42316_d_n5, assign32040_e42316_d_n6, assign32040_e42316_d_n7, assign32040_e42316_d_n8, assign32040_e42316_d_n9, assign32040_e42316_d_n10, assign32040_e42316_d_n11, assign32040_e42316_d_n12, assign32040_e42316_d_n13, assign32040_e42316_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32040_e42311: f64 = (2.0 * locals.var_phib_n_edge);
        let assign32040_e42312: f64 = (locals.var_psip - assign32040_e42311);
        let assign32040_e42314: f64 = (assign32040_e42312 - locals.var_vs_1);
        (assign32040_e42314, ((locals.var_psip_dn0 - (2.0 * locals.var_phib_n_edge_dn0)) - locals.var_vs_1_dn0), ((locals.var_psip_dn2 - (2.0 * locals.var_phib_n_edge_dn2)) - locals.var_vs_1_dn2), ((locals.var_psip_dn3 - (2.0 * locals.var_phib_n_edge_dn3)) - locals.var_vs_1_dn3), ((locals.var_psip_dn4 - (2.0 * locals.var_phib_n_edge_dn4)) - locals.var_vs_1_dn4), ((locals.var_psip_dn5 - (2.0 * locals.var_phib_n_edge_dn5)) - locals.var_vs_1_dn5), ((locals.var_psip_dn6 - (2.0 * locals.var_phib_n_edge_dn6)) - locals.var_vs_1_dn6), ((locals.var_psip_dn7 - (2.0 * locals.var_phib_n_edge_dn7)) - locals.var_vs_1_dn7), ((locals.var_psip_dn8 - (2.0 * locals.var_phib_n_edge_dn8)) - locals.var_vs_1_dn8), ((locals.var_psip_dn9 - (2.0 * locals.var_phib_n_edge_dn9)) - locals.var_vs_1_dn9), ((locals.var_psip_dn10 - (2.0 * locals.var_phib_n_edge_dn10)) - locals.var_vs_1_dn10), ((locals.var_psip_dn11 - (2.0 * locals.var_phib_n_edge_dn11)) - locals.var_vs_1_dn11), ((locals.var_psip_dn12 - (2.0 * locals.var_phib_n_edge_dn12)) - locals.var_vs_1_dn12), ((locals.var_psip_dn13 - (2.0 * locals.var_phib_n_edge_dn13)) - locals.var_vs_1_dn13), ((locals.var_psip_dn14 - (2.0 * locals.var_phib_n_edge_dn14)) - locals.var_vs_1_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32040_e42316;
        locals.var_t1_dn0 = assign32040_e42316_d_n0;
        locals.var_t1_dn2 = assign32040_e42316_d_n2;
        locals.var_t1_dn3 = assign32040_e42316_d_n3;
        locals.var_t1_dn4 = assign32040_e42316_d_n4;
        locals.var_t1_dn5 = assign32040_e42316_d_n5;
        locals.var_t1_dn6 = assign32040_e42316_d_n6;
        locals.var_t1_dn7 = assign32040_e42316_d_n7;
        locals.var_t1_dn8 = assign32040_e42316_d_n8;
        locals.var_t1_dn9 = assign32040_e42316_d_n9;
        locals.var_t1_dn10 = assign32040_e42316_d_n10;
        locals.var_t1_dn11 = assign32040_e42316_d_n11;
        locals.var_t1_dn12 = assign32040_e42316_d_n12;
        locals.var_t1_dn13 = assign32040_e42316_d_n13;
        locals.var_t1_dn14 = assign32040_e42316_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign32050_e42331, assign32050_e42331_d_n0, assign32050_e42331_d_n2, assign32050_e42331_d_n3, assign32050_e42331_d_n4, assign32050_e42331_d_n5, assign32050_e42331_d_n6, assign32050_e42331_d_n7, assign32050_e42331_d_n8, assign32050_e42331_d_n9, assign32050_e42331_d_n10, assign32050_e42331_d_n11, assign32050_e42331_d_n12, assign32050_e42331_d_n13, assign32050_e42331_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32050_e42320: f64 = locals.var_t1;
        let assign32050_e42323: f64 = (4.0 * locals.var_t0);
        let assign32050_e42325: f64 = (assign32050_e42323 * locals.var_sqrtpsip);
        let assign32050_e42327: f64 = (assign32050_e42325).max(1e-38);
        let assign32050_e42328: f64 = (assign32050_e42327).ln();
        let assign32050_e42329: f64 = (assign32050_e42320 - assign32050_e42328);
        (assign32050_e42329, (locals.var_t1_dn0 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn0) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn0)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn2 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn2) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn2)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn3 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn3) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn3)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn4 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn4) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn4)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn5 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn5) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn5)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn6 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn6) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn6)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn7 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn7) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn7)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn8 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn8) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn8)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn9 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn9) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn9)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn10 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn10) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn10)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn11 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn11) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn11)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn12 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn12) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn12)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn13 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn13) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn13)) } else { 0.0 } / assign32050_e42327)), (locals.var_t1_dn14 - (if assign32050_e42325 >= 1e-38 { (((4.0 * locals.var_t0_dn14) * locals.var_sqrtpsip) + (assign32050_e42323 * locals.var_sqrtpsip_dn14)) } else { 0.0 } / assign32050_e42327)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32050_e42331;
        locals.var_t2_dn0 = assign32050_e42331_d_n0;
        locals.var_t2_dn2 = assign32050_e42331_d_n2;
        locals.var_t2_dn3 = assign32050_e42331_d_n3;
        locals.var_t2_dn4 = assign32050_e42331_d_n4;
        locals.var_t2_dn5 = assign32050_e42331_d_n5;
        locals.var_t2_dn6 = assign32050_e42331_d_n6;
        locals.var_t2_dn7 = assign32050_e42331_d_n7;
        locals.var_t2_dn8 = assign32050_e42331_d_n8;
        locals.var_t2_dn9 = assign32050_e42331_d_n9;
        locals.var_t2_dn10 = assign32050_e42331_d_n10;
        locals.var_t2_dn11 = assign32050_e42331_d_n11;
        locals.var_t2_dn12 = assign32050_e42331_d_n12;
        locals.var_t2_dn13 = assign32050_e42331_d_n13;
        locals.var_t2_dn14 = assign32050_e42331_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign32060_e42348, assign32060_e42348_d_n0, assign32060_e42348_d_n2, assign32060_e42348_d_n3, assign32060_e42348_d_n4, assign32060_e42348_d_n5, assign32060_e42348_d_n6, assign32060_e42348_d_n7, assign32060_e42348_d_n8, assign32060_e42348_d_n9, assign32060_e42348_d_n10, assign32060_e42348_d_n11, assign32060_e42348_d_n12, assign32060_e42348_d_n13, assign32060_e42348_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32060_e42336: f64 = (locals.var_t2 - 0.201491);
        let assign32060_e42340: f64 = (locals.var_t2 + 0.402982);
        let assign32060_e42341: f64 = (locals.var_t2 * assign32060_e42340);
        let assign32060_e42343: f64 = (assign32060_e42341 + 2.446562);
        let assign32060_e42344: f64 = (assign32060_e42343).sqrt();
        let assign32060_e42345: f64 = (assign32060_e42336 - assign32060_e42344);
        let assign32060_e42346: f64 = (0.5 * assign32060_e42345);
        (assign32060_e42346, (0.5 * (locals.var_t2_dn0 - (((locals.var_t2_dn0 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn2 - (((locals.var_t2_dn2 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn3 - (((locals.var_t2_dn3 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn4 - (((locals.var_t2_dn4 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn5 - (((locals.var_t2_dn5 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn6 - (((locals.var_t2_dn6 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn7 - (((locals.var_t2_dn7 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn8 - (((locals.var_t2_dn8 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn9 - (((locals.var_t2_dn9 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn10 - (((locals.var_t2_dn10 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn11 - (((locals.var_t2_dn11 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn12 - (((locals.var_t2_dn12 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn12)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn13 - (((locals.var_t2_dn13 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign32060_e42344)))), (0.5 * (locals.var_t2_dn14 - (((locals.var_t2_dn14 * assign32060_e42340) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign32060_e42344)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign32060_e42348;
        locals.var_t8_dn0 = assign32060_e42348_d_n0;
        locals.var_t8_dn2 = assign32060_e42348_d_n2;
        locals.var_t8_dn3 = assign32060_e42348_d_n3;
        locals.var_t8_dn4 = assign32060_e42348_d_n4;
        locals.var_t8_dn5 = assign32060_e42348_d_n5;
        locals.var_t8_dn6 = assign32060_e42348_d_n6;
        locals.var_t8_dn7 = assign32060_e42348_d_n7;
        locals.var_t8_dn8 = assign32060_e42348_d_n8;
        locals.var_t8_dn9 = assign32060_e42348_d_n9;
        locals.var_t8_dn10 = assign32060_e42348_d_n10;
        locals.var_t8_dn11 = assign32060_e42348_d_n11;
        locals.var_t8_dn12 = assign32060_e42348_d_n12;
        locals.var_t8_dn13 = assign32060_e42348_d_n13;
        locals.var_t8_dn14 = assign32060_e42348_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign32070_e42352, assign32070_e42352_d_n0, assign32070_e42352_d_n2, assign32070_e42352_d_n3, assign32070_e42352_d_n4, assign32070_e42352_d_n5, assign32070_e42352_d_n6, assign32070_e42352_d_n7, assign32070_e42352_d_n8, assign32070_e42352_d_n9, assign32070_e42352_d_n10, assign32070_e42352_d_n11, assign32070_e42352_d_n12, assign32070_e42352_d_n13, assign32070_e42352_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn0, locals.var_sqrtpsip_dn2, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11, locals.var_sqrtpsip_dn12, locals.var_sqrtpsip_dn13, locals.var_sqrtpsip_dn14,)
    } else {
        (locals.var_sqrtpsisa, locals.var_sqrtpsisa_dn0, locals.var_sqrtpsisa_dn2, locals.var_sqrtpsisa_dn3, locals.var_sqrtpsisa_dn4, locals.var_sqrtpsisa_dn5, locals.var_sqrtpsisa_dn6, locals.var_sqrtpsisa_dn7, locals.var_sqrtpsisa_dn8, locals.var_sqrtpsisa_dn9, locals.var_sqrtpsisa_dn10, locals.var_sqrtpsisa_dn11, locals.var_sqrtpsisa_dn12, locals.var_sqrtpsisa_dn13, locals.var_sqrtpsisa_dn14,)
    }
};
        locals.var_sqrtpsisa = assign32070_e42352;
        locals.var_sqrtpsisa_dn0 = assign32070_e42352_d_n0;
        locals.var_sqrtpsisa_dn2 = assign32070_e42352_d_n2;
        locals.var_sqrtpsisa_dn3 = assign32070_e42352_d_n3;
        locals.var_sqrtpsisa_dn4 = assign32070_e42352_d_n4;
        locals.var_sqrtpsisa_dn5 = assign32070_e42352_d_n5;
        locals.var_sqrtpsisa_dn6 = assign32070_e42352_d_n6;
        locals.var_sqrtpsisa_dn7 = assign32070_e42352_d_n7;
        locals.var_sqrtpsisa_dn8 = assign32070_e42352_d_n8;
        locals.var_sqrtpsisa_dn9 = assign32070_e42352_d_n9;
        locals.var_sqrtpsisa_dn10 = assign32070_e42352_d_n10;
        locals.var_sqrtpsisa_dn11 = assign32070_e42352_d_n11;
        locals.var_sqrtpsisa_dn12 = assign32070_e42352_d_n12;
        locals.var_sqrtpsisa_dn13 = assign32070_e42352_d_n13;
        locals.var_sqrtpsisa_dn14 = assign32070_e42352_d_n14;
        locals.var_sqrtpsisa_rv = 0.0;

        let assign32080_e42355: f64 = (-68.0);
        let assign32080_e42356: f64 = if locals.var_t8 <= assign32080_e42355 { 1.0 } else { 0.0 };
        locals.var_guard736 = assign32080_e42356;
        locals.var_guard736_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_103(
        locals: &mut StampLocals,
    ) {
        let (assign32090_e42363, assign32090_e42363_d_n0, assign32090_e42363_d_n2, assign32090_e42363_d_n3, assign32090_e42363_d_n4, assign32090_e42363_d_n5, assign32090_e42363_d_n6, assign32090_e42363_d_n7, assign32090_e42363_d_n8, assign32090_e42363_d_n9, assign32090_e42363_d_n10, assign32090_e42363_d_n11, assign32090_e42363_d_n12, assign32090_e42363_d_n13, assign32090_e42363_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) {
        let assign32090_e42361: f64 = (-100.0);
        (assign32090_e42361, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32090_e42363;
        locals.var_t4_dn0 = assign32090_e42363_d_n0;
        locals.var_t4_dn2 = assign32090_e42363_d_n2;
        locals.var_t4_dn3 = assign32090_e42363_d_n3;
        locals.var_t4_dn4 = assign32090_e42363_d_n4;
        locals.var_t4_dn5 = assign32090_e42363_d_n5;
        locals.var_t4_dn6 = assign32090_e42363_d_n6;
        locals.var_t4_dn7 = assign32090_e42363_d_n7;
        locals.var_t4_dn8 = assign32090_e42363_d_n8;
        locals.var_t4_dn9 = assign32090_e42363_d_n9;
        locals.var_t4_dn10 = assign32090_e42363_d_n10;
        locals.var_t4_dn11 = assign32090_e42363_d_n11;
        locals.var_t4_dn12 = assign32090_e42363_d_n12;
        locals.var_t4_dn13 = assign32090_e42363_d_n13;
        locals.var_t4_dn14 = assign32090_e42363_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign32100_e42369, assign32100_e42369_d_n0, assign32100_e42369_d_n2, assign32100_e42369_d_n3, assign32100_e42369_d_n4, assign32100_e42369_d_n5, assign32100_e42369_d_n6, assign32100_e42369_d_n7, assign32100_e42369_d_n8, assign32100_e42369_d_n9, assign32100_e42369_d_n10, assign32100_e42369_d_n11, assign32100_e42369_d_n12, assign32100_e42369_d_n13, assign32100_e42369_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32100_e42369;
        locals.var_t5_dn0 = assign32100_e42369_d_n0;
        locals.var_t5_dn2 = assign32100_e42369_d_n2;
        locals.var_t5_dn3 = assign32100_e42369_d_n3;
        locals.var_t5_dn4 = assign32100_e42369_d_n4;
        locals.var_t5_dn5 = assign32100_e42369_d_n5;
        locals.var_t5_dn6 = assign32100_e42369_d_n6;
        locals.var_t5_dn7 = assign32100_e42369_d_n7;
        locals.var_t5_dn8 = assign32100_e42369_d_n8;
        locals.var_t5_dn9 = assign32100_e42369_d_n9;
        locals.var_t5_dn10 = assign32100_e42369_d_n10;
        locals.var_t5_dn11 = assign32100_e42369_d_n11;
        locals.var_t5_dn12 = assign32100_e42369_d_n12;
        locals.var_t5_dn13 = assign32100_e42369_d_n13;
        locals.var_t5_dn14 = assign32100_e42369_d_n14;
        locals.var_t5_rv = 0.0;

        let assign32110_e42374: f64 = (0.5 * locals.var_t5);
        let assign32110_e42375: f64 = (locals.var_t4 - assign32110_e42374);
        let assign32110_e42376: f64 = if locals.var_t8 < assign32110_e42375 { 1.0 } else { 0.0 };
        locals.var_guard737 = assign32110_e42376;
        locals.var_guard737_rv = 0.0;

        let (assign32120_e42385, assign32120_e42385_d_n0, assign32120_e42385_d_n2, assign32120_e42385_d_n3, assign32120_e42385_d_n4, assign32120_e42385_d_n5, assign32120_e42385_d_n6, assign32120_e42385_d_n7, assign32120_e42385_d_n8, assign32120_e42385_d_n9, assign32120_e42385_d_n10, assign32120_e42385_d_n11, assign32120_e42385_d_n12, assign32120_e42385_d_n13, assign32120_e42385_d_n14,) = {
    if (((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) && (locals.var_guard737 != 0.0)) {
        let assign32120_e42383: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32120_e42383, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn0), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn2), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn12), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn13), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32120_e42385;
        locals.var_t3_dn0 = assign32120_e42385_d_n0;
        locals.var_t3_dn2 = assign32120_e42385_d_n2;
        locals.var_t3_dn3 = assign32120_e42385_d_n3;
        locals.var_t3_dn4 = assign32120_e42385_d_n4;
        locals.var_t3_dn5 = assign32120_e42385_d_n5;
        locals.var_t3_dn6 = assign32120_e42385_d_n6;
        locals.var_t3_dn7 = assign32120_e42385_d_n7;
        locals.var_t3_dn8 = assign32120_e42385_d_n8;
        locals.var_t3_dn9 = assign32120_e42385_d_n9;
        locals.var_t3_dn10 = assign32120_e42385_d_n10;
        locals.var_t3_dn11 = assign32120_e42385_d_n11;
        locals.var_t3_dn12 = assign32120_e42385_d_n12;
        locals.var_t3_dn13 = assign32120_e42385_d_n13;
        locals.var_t3_dn14 = assign32120_e42385_d_n14;
        locals.var_t3_rv = 0.0;

        let assign32130_e42390: f64 = (0.5 * locals.var_t5);
        let assign32130_e42391: f64 = (locals.var_t4 + assign32130_e42390);
        let assign32130_e42392: f64 = if locals.var_t8 > assign32130_e42391 { 1.0 } else { 0.0 };
        locals.var_guard738 = assign32130_e42392;
        locals.var_guard738_rv = 0.0;

        let (assign32140_e42404, assign32140_e42404_d_n0, assign32140_e42404_d_n2, assign32140_e42404_d_n3, assign32140_e42404_d_n4, assign32140_e42404_d_n5, assign32140_e42404_d_n6, assign32140_e42404_d_n7, assign32140_e42404_d_n8, assign32140_e42404_d_n9, assign32140_e42404_d_n10, assign32140_e42404_d_n11, assign32140_e42404_d_n12, assign32140_e42404_d_n13, assign32140_e42404_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 != 0.0)) {
        let assign32140_e42402: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32140_e42402, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn0), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn2), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn12), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn13), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32140_e42404;
        locals.var_t3_dn0 = assign32140_e42404_d_n0;
        locals.var_t3_dn2 = assign32140_e42404_d_n2;
        locals.var_t3_dn3 = assign32140_e42404_d_n3;
        locals.var_t3_dn4 = assign32140_e42404_d_n4;
        locals.var_t3_dn5 = assign32140_e42404_d_n5;
        locals.var_t3_dn6 = assign32140_e42404_d_n6;
        locals.var_t3_dn7 = assign32140_e42404_d_n7;
        locals.var_t3_dn8 = assign32140_e42404_d_n8;
        locals.var_t3_dn9 = assign32140_e42404_d_n9;
        locals.var_t3_dn10 = assign32140_e42404_d_n10;
        locals.var_t3_dn11 = assign32140_e42404_d_n11;
        locals.var_t3_dn12 = assign32140_e42404_d_n12;
        locals.var_t3_dn13 = assign32140_e42404_d_n13;
        locals.var_t3_dn14 = assign32140_e42404_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32150_e42420, assign32150_e42420_d_n0, assign32150_e42420_d_n2, assign32150_e42420_d_n3, assign32150_e42420_d_n4, assign32150_e42420_d_n5, assign32150_e42420_d_n6, assign32150_e42420_d_n7, assign32150_e42420_d_n8, assign32150_e42420_d_n9, assign32150_e42420_d_n10, assign32150_e42420_d_n11, assign32150_e42420_d_n12, assign32150_e42420_d_n13, assign32150_e42420_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 == 0.0)) {
        let assign32150_e42416: f64 = (locals.var_t8 - locals.var_t4);
        let assign32150_e42418: f64 = (assign32150_e42416 / locals.var_t5);
        (assign32150_e42418, ((((locals.var_t8_dn0 - locals.var_t4_dn0) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn2 - locals.var_t4_dn2) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn3 - locals.var_t4_dn3) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn4 - locals.var_t4_dn4) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn5 - locals.var_t4_dn5) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn6 - locals.var_t4_dn6) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn7 - locals.var_t4_dn7) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn8 - locals.var_t4_dn8) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn9 - locals.var_t4_dn9) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn10 - locals.var_t4_dn10) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn11 - locals.var_t4_dn11) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn12 - locals.var_t4_dn12) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn13 - locals.var_t4_dn13) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn14 - locals.var_t4_dn14) * locals.var_t5) - (assign32150_e42416 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32150_e42420;
        locals.var_t2_dn0 = assign32150_e42420_d_n0;
        locals.var_t2_dn2 = assign32150_e42420_d_n2;
        locals.var_t2_dn3 = assign32150_e42420_d_n3;
        locals.var_t2_dn4 = assign32150_e42420_d_n4;
        locals.var_t2_dn5 = assign32150_e42420_d_n5;
        locals.var_t2_dn6 = assign32150_e42420_d_n6;
        locals.var_t2_dn7 = assign32150_e42420_d_n7;
        locals.var_t2_dn8 = assign32150_e42420_d_n8;
        locals.var_t2_dn9 = assign32150_e42420_d_n9;
        locals.var_t2_dn10 = assign32150_e42420_d_n10;
        locals.var_t2_dn11 = assign32150_e42420_d_n11;
        locals.var_t2_dn12 = assign32150_e42420_d_n12;
        locals.var_t2_dn13 = assign32150_e42420_d_n13;
        locals.var_t2_dn14 = assign32150_e42420_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign32160_e42434, assign32160_e42434_d_n0, assign32160_e42434_d_n2, assign32160_e42434_d_n3, assign32160_e42434_d_n4, assign32160_e42434_d_n5, assign32160_e42434_d_n6, assign32160_e42434_d_n7, assign32160_e42434_d_n8, assign32160_e42434_d_n9, assign32160_e42434_d_n10, assign32160_e42434_d_n11, assign32160_e42434_d_n12, assign32160_e42434_d_n13, assign32160_e42434_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 == 0.0)) {
        let assign32160_e42432: f64 = (locals.var_t2 * locals.var_t2);
        (assign32160_e42432, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)), ((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)), ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign32160_e42434;
        locals.var_t6_dn0 = assign32160_e42434_d_n0;
        locals.var_t6_dn2 = assign32160_e42434_d_n2;
        locals.var_t6_dn3 = assign32160_e42434_d_n3;
        locals.var_t6_dn4 = assign32160_e42434_d_n4;
        locals.var_t6_dn5 = assign32160_e42434_d_n5;
        locals.var_t6_dn6 = assign32160_e42434_d_n6;
        locals.var_t6_dn7 = assign32160_e42434_d_n7;
        locals.var_t6_dn8 = assign32160_e42434_d_n8;
        locals.var_t6_dn9 = assign32160_e42434_d_n9;
        locals.var_t6_dn10 = assign32160_e42434_d_n10;
        locals.var_t6_dn11 = assign32160_e42434_d_n11;
        locals.var_t6_dn12 = assign32160_e42434_d_n12;
        locals.var_t6_dn13 = assign32160_e42434_d_n13;
        locals.var_t6_dn14 = assign32160_e42434_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign32170_e42469, assign32170_e42469_d_n0, assign32170_e42469_d_n2, assign32170_e42469_d_n3, assign32170_e42469_d_n4, assign32170_e42469_d_n5, assign32170_e42469_d_n6, assign32170_e42469_d_n7, assign32170_e42469_d_n8, assign32170_e42469_d_n9, assign32170_e42469_d_n10, assign32170_e42469_d_n11, assign32170_e42469_d_n12, assign32170_e42469_d_n13, assign32170_e42469_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) && (locals.var_guard737 == 0.0)) && (locals.var_guard738 == 0.0)) {
        let assign32170_e42448: f64 = (5.0 / 64.0);
        let assign32170_e42451: f64 = (0.5 * locals.var_t2);
        let assign32170_e42452: f64 = (assign32170_e42448 + assign32170_e42451);
        let assign32170_e42456: f64 = (15.0 / 16.0);
        let assign32170_e42460: f64 = (1.25 - locals.var_t6);
        let assign32170_e42461: f64 = (locals.var_t6 * assign32170_e42460);
        let assign32170_e42462: f64 = (assign32170_e42456 - assign32170_e42461);
        let assign32170_e42463: f64 = (locals.var_t6 * assign32170_e42462);
        let assign32170_e42464: f64 = (assign32170_e42452 + assign32170_e42463);
        let assign32170_e42465: f64 = (locals.var_t5 * assign32170_e42464);
        let assign32170_e42466: f64 = (locals.var_t4 + assign32170_e42465);
        let assign32170_e42467: f64 = { let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32170_e42467, ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn0 + ((locals.var_t5_dn0 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn0) + ((locals.var_t6_dn0 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn0 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn0))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn2 + ((locals.var_t5_dn2 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn2) + ((locals.var_t6_dn2 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn2 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn2))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn3 + ((locals.var_t5_dn3 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn3) + ((locals.var_t6_dn3 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn3 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn3))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn4) + ((locals.var_t6_dn4 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn4 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn4))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn5) + ((locals.var_t6_dn5 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn5 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn5))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn6) + ((locals.var_t6_dn6 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn6 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn6))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn7) + ((locals.var_t6_dn7 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn7 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn7))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn8) + ((locals.var_t6_dn8 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn8 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn8))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn9) + ((locals.var_t6_dn9 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn9 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn9))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn10) + ((locals.var_t6_dn10 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn10 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn10))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn11) + ((locals.var_t6_dn11 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn11 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn11))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn12 + ((locals.var_t5_dn12 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn12) + ((locals.var_t6_dn12 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn12 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn12))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn13 + ((locals.var_t5_dn13 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn13) + ((locals.var_t6_dn13 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn13 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn13))))))))))), ({ let limited_exp_arg = assign32170_e42466; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn14 + ((locals.var_t5_dn14 * assign32170_e42464) + (locals.var_t5 * ((0.5 * locals.var_t2_dn14) + ((locals.var_t6_dn14 * assign32170_e42462) + (locals.var_t6 * (-((locals.var_t6_dn14 * assign32170_e42460) + (locals.var_t6 * (-locals.var_t6_dn14))))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32170_e42469;
        locals.var_t3_dn0 = assign32170_e42469_d_n0;
        locals.var_t3_dn2 = assign32170_e42469_d_n2;
        locals.var_t3_dn3 = assign32170_e42469_d_n3;
        locals.var_t3_dn4 = assign32170_e42469_d_n4;
        locals.var_t3_dn5 = assign32170_e42469_d_n5;
        locals.var_t3_dn6 = assign32170_e42469_d_n6;
        locals.var_t3_dn7 = assign32170_e42469_d_n7;
        locals.var_t3_dn8 = assign32170_e42469_d_n8;
        locals.var_t3_dn9 = assign32170_e42469_d_n9;
        locals.var_t3_dn10 = assign32170_e42469_d_n10;
        locals.var_t3_dn11 = assign32170_e42469_d_n11;
        locals.var_t3_dn12 = assign32170_e42469_d_n12;
        locals.var_t3_dn13 = assign32170_e42469_d_n13;
        locals.var_t3_dn14 = assign32170_e42469_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32180_e42502, assign32180_e42502_d_n0, assign32180_e42502_d_n2, assign32180_e42502_d_n3, assign32180_e42502_d_n4, assign32180_e42502_d_n5, assign32180_e42502_d_n6, assign32180_e42502_d_n7, assign32180_e42502_d_n8, assign32180_e42502_d_n9, assign32180_e42502_d_n10, assign32180_e42502_d_n11, assign32180_e42502_d_n12, assign32180_e42502_d_n13, assign32180_e42502_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 != 0.0)) {
        let assign32180_e42476: f64 = (1.0 + locals.var_t1);
        let assign32180_e42479: f64 = locals.var_t8;
        let assign32180_e42480: f64 = (assign32180_e42476 - assign32180_e42479);
        let assign32180_e42484: f64 = (2.0 * locals.var_t0);
        let assign32180_e42487: f64 = (locals.var_t3 * 2.0);
        let assign32180_e42489: f64 = (assign32180_e42487 * locals.var_t0);
        let assign32180_e42492: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign32180_e42493: f64 = (assign32180_e42489 + assign32180_e42492);
        let assign32180_e42494: f64 = (assign32180_e42484 * assign32180_e42493);
        let assign32180_e42496: f64 = (assign32180_e42494).max(1e-38);
        let assign32180_e42497: f64 = (assign32180_e42496).ln();
        let assign32180_e42498: f64 = assign32180_e42497;
        let assign32180_e42499: f64 = (assign32180_e42480 - assign32180_e42498);
        let assign32180_e42500: f64 = (locals.var_t3 * assign32180_e42499);
        (assign32180_e42500, ((locals.var_t3_dn0 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn0 - locals.var_t8_dn0) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn0) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn0)) + (2.0 * locals.var_sqrtpsisa_dn0)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn2 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn2 - locals.var_t8_dn2) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn2) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn2)) + (2.0 * locals.var_sqrtpsisa_dn2)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn3 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn3 - locals.var_t8_dn3) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn3) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn4 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn4 - locals.var_t8_dn4) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn4) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn5 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn5 - locals.var_t8_dn5) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn5) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn6 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn6 - locals.var_t8_dn6) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn6) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn7 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn7 - locals.var_t8_dn7) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn7) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn8 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn8 - locals.var_t8_dn8) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn8) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn9 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn9 - locals.var_t8_dn9) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn9) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn10 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn10 - locals.var_t8_dn10) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn10) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn11 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn11 - locals.var_t8_dn11) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn11) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn12 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn12 - locals.var_t8_dn12) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn12) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn12)) + (2.0 * locals.var_sqrtpsisa_dn12)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn13 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn13 - locals.var_t8_dn13) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn13) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn13)) + (2.0 * locals.var_sqrtpsisa_dn13)))) } else { 0.0 } / assign32180_e42496)))), ((locals.var_t3_dn14 * assign32180_e42499) + (locals.var_t3 * ((locals.var_t1_dn14 - locals.var_t8_dn14) - (if assign32180_e42494 >= 1e-38 { (((2.0 * locals.var_t0_dn14) * assign32180_e42493) + (assign32180_e42484 * ((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32180_e42487 * locals.var_t0_dn14)) + (2.0 * locals.var_sqrtpsisa_dn14)))) } else { 0.0 } / assign32180_e42496)))),)
    } else {
        (locals.var_qs_edge, locals.var_qs_edge_dn0, locals.var_qs_edge_dn2, locals.var_qs_edge_dn3, locals.var_qs_edge_dn4, locals.var_qs_edge_dn5, locals.var_qs_edge_dn6, locals.var_qs_edge_dn7, locals.var_qs_edge_dn8, locals.var_qs_edge_dn9, locals.var_qs_edge_dn10, locals.var_qs_edge_dn11, locals.var_qs_edge_dn12, locals.var_qs_edge_dn13, locals.var_qs_edge_dn14,)
    }
};
        locals.var_qs_edge = assign32180_e42502;
        locals.var_qs_edge_dn0 = assign32180_e42502_d_n0;
        locals.var_qs_edge_dn2 = assign32180_e42502_d_n2;
        locals.var_qs_edge_dn3 = assign32180_e42502_d_n3;
        locals.var_qs_edge_dn4 = assign32180_e42502_d_n4;
        locals.var_qs_edge_dn5 = assign32180_e42502_d_n5;
        locals.var_qs_edge_dn6 = assign32180_e42502_d_n6;
        locals.var_qs_edge_dn7 = assign32180_e42502_d_n7;
        locals.var_qs_edge_dn8 = assign32180_e42502_d_n8;
        locals.var_qs_edge_dn9 = assign32180_e42502_d_n9;
        locals.var_qs_edge_dn10 = assign32180_e42502_d_n10;
        locals.var_qs_edge_dn11 = assign32180_e42502_d_n11;
        locals.var_qs_edge_dn12 = assign32180_e42502_d_n12;
        locals.var_qs_edge_dn13 = assign32180_e42502_d_n13;
        locals.var_qs_edge_dn14 = assign32180_e42502_d_n14;
        locals.var_qs_edge_rv = 0.0;

        let (assign32190_e42510, assign32190_e42510_d_n0, assign32190_e42510_d_n2, assign32190_e42510_d_n3, assign32190_e42510_d_n4, assign32190_e42510_d_n5, assign32190_e42510_d_n6, assign32190_e42510_d_n7, assign32190_e42510_d_n8, assign32190_e42510_d_n9, assign32190_e42510_d_n10, assign32190_e42510_d_n11, assign32190_e42510_d_n12, assign32190_e42510_d_n13, assign32190_e42510_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32190_e42508: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32190_e42508, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn0), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn2), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn12), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn13), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32190_e42510;
        locals.var_t3_dn0 = assign32190_e42510_d_n0;
        locals.var_t3_dn2 = assign32190_e42510_d_n2;
        locals.var_t3_dn3 = assign32190_e42510_d_n3;
        locals.var_t3_dn4 = assign32190_e42510_d_n4;
        locals.var_t3_dn5 = assign32190_e42510_d_n5;
        locals.var_t3_dn6 = assign32190_e42510_d_n6;
        locals.var_t3_dn7 = assign32190_e42510_d_n7;
        locals.var_t3_dn8 = assign32190_e42510_d_n8;
        locals.var_t3_dn9 = assign32190_e42510_d_n9;
        locals.var_t3_dn10 = assign32190_e42510_d_n10;
        locals.var_t3_dn11 = assign32190_e42510_d_n11;
        locals.var_t3_dn12 = assign32190_e42510_d_n12;
        locals.var_t3_dn13 = assign32190_e42510_d_n13;
        locals.var_t3_dn14 = assign32190_e42510_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32200_e42519, assign32200_e42519_d_n0, assign32200_e42519_d_n2, assign32200_e42519_d_n3, assign32200_e42519_d_n4, assign32200_e42519_d_n5, assign32200_e42519_d_n6, assign32200_e42519_d_n7, assign32200_e42519_d_n8, assign32200_e42519_d_n9, assign32200_e42519_d_n10, assign32200_e42519_d_n11, assign32200_e42519_d_n12, assign32200_e42519_d_n13, assign32200_e42519_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32200_e42517: f64 = (1.0 / locals.var_sqrtpsisa);
        (assign32200_e42517, (-(locals.var_sqrtpsisa_dn0 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn2 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn3 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn4 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn5 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn6 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn7 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn8 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn9 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn10 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn11 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn12 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn13 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn14 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))),)
    } else {
        (locals.var_sqrtpsisainv, locals.var_sqrtpsisainv_dn0, locals.var_sqrtpsisainv_dn2, locals.var_sqrtpsisainv_dn3, locals.var_sqrtpsisainv_dn4, locals.var_sqrtpsisainv_dn5, locals.var_sqrtpsisainv_dn6, locals.var_sqrtpsisainv_dn7, locals.var_sqrtpsisainv_dn8, locals.var_sqrtpsisainv_dn9, locals.var_sqrtpsisainv_dn10, locals.var_sqrtpsisainv_dn11, locals.var_sqrtpsisainv_dn12, locals.var_sqrtpsisainv_dn13, locals.var_sqrtpsisainv_dn14,)
    }
};
        locals.var_sqrtpsisainv = assign32200_e42519;
        locals.var_sqrtpsisainv_dn0 = assign32200_e42519_d_n0;
        locals.var_sqrtpsisainv_dn2 = assign32200_e42519_d_n2;
        locals.var_sqrtpsisainv_dn3 = assign32200_e42519_d_n3;
        locals.var_sqrtpsisainv_dn4 = assign32200_e42519_d_n4;
        locals.var_sqrtpsisainv_dn5 = assign32200_e42519_d_n5;
        locals.var_sqrtpsisainv_dn6 = assign32200_e42519_d_n6;
        locals.var_sqrtpsisainv_dn7 = assign32200_e42519_d_n7;
        locals.var_sqrtpsisainv_dn8 = assign32200_e42519_d_n8;
        locals.var_sqrtpsisainv_dn9 = assign32200_e42519_d_n9;
        locals.var_sqrtpsisainv_dn10 = assign32200_e42519_d_n10;
        locals.var_sqrtpsisainv_dn11 = assign32200_e42519_d_n11;
        locals.var_sqrtpsisainv_dn12 = assign32200_e42519_d_n12;
        locals.var_sqrtpsisainv_dn13 = assign32200_e42519_d_n13;
        locals.var_sqrtpsisainv_dn14 = assign32200_e42519_d_n14;
        locals.var_sqrtpsisainv_rv = 0.0;

        let (assign32210_e42551, assign32210_e42551_d_n0, assign32210_e42551_d_n2, assign32210_e42551_d_n3, assign32210_e42551_d_n4, assign32210_e42551_d_n5, assign32210_e42551_d_n6, assign32210_e42551_d_n7, assign32210_e42551_d_n8, assign32210_e42551_d_n9, assign32210_e42551_d_n10, assign32210_e42551_d_n11, assign32210_e42551_d_n12, assign32210_e42551_d_n13, assign32210_e42551_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32210_e42526: f64 = (2.0 * locals.var_t3);
        let assign32210_e42530: f64 = (locals.var_t3 * 2.0);
        let assign32210_e42532: f64 = (assign32210_e42530 * locals.var_t0);
        let assign32210_e42535: f64 = (locals.var_t3 * 2.0);
        let assign32210_e42537: f64 = (assign32210_e42535 * locals.var_t0);
        let assign32210_e42540: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign32210_e42541: f64 = (assign32210_e42537 + assign32210_e42540);
        let assign32210_e42542: f64 = (assign32210_e42532 * assign32210_e42541);
        let assign32210_e42544: f64 = (assign32210_e42542).max(1e-38);
        let assign32210_e42545: f64 = (assign32210_e42544).ln();
        let assign32210_e42546: f64 = assign32210_e42545;
        let assign32210_e42547: f64 = (assign32210_e42526 + assign32210_e42546);
        let assign32210_e42549: f64 = (assign32210_e42547 - locals.var_t1);
        (assign32210_e42549, (((2.0 * locals.var_t3_dn0) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn0)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn0)) + (2.0 * locals.var_sqrtpsisa_dn0)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn0), (((2.0 * locals.var_t3_dn2) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn2)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn2)) + (2.0 * locals.var_sqrtpsisa_dn2)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn2), (((2.0 * locals.var_t3_dn3) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn3)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn4)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn5)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn6)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn7)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn8)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn9)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn10)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn11)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn11), (((2.0 * locals.var_t3_dn12) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn12)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn12)) + (2.0 * locals.var_sqrtpsisa_dn12)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn12), (((2.0 * locals.var_t3_dn13) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn13)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn13)) + (2.0 * locals.var_sqrtpsisa_dn13)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn13), (((2.0 * locals.var_t3_dn14) + (if assign32210_e42542 >= 1e-38 { (((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32210_e42530 * locals.var_t0_dn14)) * assign32210_e42541) + (assign32210_e42532 * ((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32210_e42535 * locals.var_t0_dn14)) + (2.0 * locals.var_sqrtpsisa_dn14)))) } else { 0.0 } / assign32210_e42544)) - locals.var_t1_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32210_e42551;
        locals.var_t4_dn0 = assign32210_e42551_d_n0;
        locals.var_t4_dn2 = assign32210_e42551_d_n2;
        locals.var_t4_dn3 = assign32210_e42551_d_n3;
        locals.var_t4_dn4 = assign32210_e42551_d_n4;
        locals.var_t4_dn5 = assign32210_e42551_d_n5;
        locals.var_t4_dn6 = assign32210_e42551_d_n6;
        locals.var_t4_dn7 = assign32210_e42551_d_n7;
        locals.var_t4_dn8 = assign32210_e42551_d_n8;
        locals.var_t4_dn9 = assign32210_e42551_d_n9;
        locals.var_t4_dn10 = assign32210_e42551_d_n10;
        locals.var_t4_dn11 = assign32210_e42551_d_n11;
        locals.var_t4_dn12 = assign32210_e42551_d_n12;
        locals.var_t4_dn13 = assign32210_e42551_d_n13;
        locals.var_t4_dn14 = assign32210_e42551_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign32220_e42576, assign32220_e42576_d_n0, assign32220_e42576_d_n2, assign32220_e42576_d_n3, assign32220_e42576_d_n4, assign32220_e42576_d_n5, assign32220_e42576_d_n6, assign32220_e42576_d_n7, assign32220_e42576_d_n8, assign32220_e42576_d_n9, assign32220_e42576_d_n10, assign32220_e42576_d_n11, assign32220_e42576_d_n12, assign32220_e42576_d_n13, assign32220_e42576_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32220_e42559: f64 = 1.0;
        let assign32220_e42561: f64 = (assign32220_e42559 / locals.var_t3);
        let assign32220_e42562: f64 = (2.0 + assign32220_e42561);
        let assign32220_e42566: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32220_e42567: f64 = assign32220_e42566;
        let assign32220_e42570: f64 = (locals.var_t0 * locals.var_t3);
        let assign32220_e42572: f64 = (assign32220_e42570 + locals.var_sqrtpsisa);
        let assign32220_e42573: f64 = (assign32220_e42567 / assign32220_e42572);
        let assign32220_e42574: f64 = (assign32220_e42562 + assign32220_e42573);
        (assign32220_e42574, ((-((assign32220_e42559 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn3) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn12) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32220_e42572 * assign32220_e42572))), ((-((assign32220_e42559 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32220_e42572) - (assign32220_e42567 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32220_e42572 * assign32220_e42572))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32220_e42576;
        locals.var_t5_dn0 = assign32220_e42576_d_n0;
        locals.var_t5_dn2 = assign32220_e42576_d_n2;
        locals.var_t5_dn3 = assign32220_e42576_d_n3;
        locals.var_t5_dn4 = assign32220_e42576_d_n4;
        locals.var_t5_dn5 = assign32220_e42576_d_n5;
        locals.var_t5_dn6 = assign32220_e42576_d_n6;
        locals.var_t5_dn7 = assign32220_e42576_d_n7;
        locals.var_t5_dn8 = assign32220_e42576_d_n8;
        locals.var_t5_dn9 = assign32220_e42576_d_n9;
        locals.var_t5_dn10 = assign32220_e42576_d_n10;
        locals.var_t5_dn11 = assign32220_e42576_d_n11;
        locals.var_t5_dn12 = assign32220_e42576_d_n12;
        locals.var_t5_dn13 = assign32220_e42576_d_n13;
        locals.var_t5_dn14 = assign32220_e42576_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign32230_e42587, assign32230_e42587_d_n0, assign32230_e42587_d_n2, assign32230_e42587_d_n3, assign32230_e42587_d_n4, assign32230_e42587_d_n5, assign32230_e42587_d_n6, assign32230_e42587_d_n7, assign32230_e42587_d_n8, assign32230_e42587_d_n9, assign32230_e42587_d_n10, assign32230_e42587_d_n11, assign32230_e42587_d_n12, assign32230_e42587_d_n13, assign32230_e42587_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32230_e42584: f64 = (locals.var_t4 / locals.var_t5);
        let assign32230_e42585: f64 = (locals.var_t3 - assign32230_e42584);
        (assign32230_e42585, (locals.var_t3_dn0 - (((locals.var_t4_dn0 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn2 - (((locals.var_t4_dn2 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn3 - (((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn4 - (((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn5 - (((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn6 - (((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn7 - (((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn8 - (((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn9 - (((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn10 - (((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn11 - (((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn12 - (((locals.var_t4_dn12 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn13 - (((locals.var_t4_dn13 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn14 - (((locals.var_t4_dn14 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32230_e42587;
        locals.var_t3_dn0 = assign32230_e42587_d_n0;
        locals.var_t3_dn2 = assign32230_e42587_d_n2;
        locals.var_t3_dn3 = assign32230_e42587_d_n3;
        locals.var_t3_dn4 = assign32230_e42587_d_n4;
        locals.var_t3_dn5 = assign32230_e42587_d_n5;
        locals.var_t3_dn6 = assign32230_e42587_d_n6;
        locals.var_t3_dn7 = assign32230_e42587_d_n7;
        locals.var_t3_dn8 = assign32230_e42587_d_n8;
        locals.var_t3_dn9 = assign32230_e42587_d_n9;
        locals.var_t3_dn10 = assign32230_e42587_d_n10;
        locals.var_t3_dn11 = assign32230_e42587_d_n11;
        locals.var_t3_dn12 = assign32230_e42587_d_n12;
        locals.var_t3_dn13 = assign32230_e42587_d_n13;
        locals.var_t3_dn14 = assign32230_e42587_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32240_e42619, assign32240_e42619_d_n0, assign32240_e42619_d_n2, assign32240_e42619_d_n3, assign32240_e42619_d_n4, assign32240_e42619_d_n5, assign32240_e42619_d_n6, assign32240_e42619_d_n7, assign32240_e42619_d_n8, assign32240_e42619_d_n9, assign32240_e42619_d_n10, assign32240_e42619_d_n11, assign32240_e42619_d_n12, assign32240_e42619_d_n13, assign32240_e42619_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32240_e42594: f64 = (2.0 * locals.var_t3);
        let assign32240_e42598: f64 = (locals.var_t3 * 2.0);
        let assign32240_e42600: f64 = (assign32240_e42598 * locals.var_t0);
        let assign32240_e42603: f64 = (locals.var_t3 * 2.0);
        let assign32240_e42605: f64 = (assign32240_e42603 * locals.var_t0);
        let assign32240_e42608: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign32240_e42609: f64 = (assign32240_e42605 + assign32240_e42608);
        let assign32240_e42610: f64 = (assign32240_e42600 * assign32240_e42609);
        let assign32240_e42612: f64 = (assign32240_e42610).max(1e-38);
        let assign32240_e42613: f64 = (assign32240_e42612).ln();
        let assign32240_e42614: f64 = assign32240_e42613;
        let assign32240_e42615: f64 = (assign32240_e42594 + assign32240_e42614);
        let assign32240_e42617: f64 = (assign32240_e42615 - locals.var_t1);
        (assign32240_e42617, (((2.0 * locals.var_t3_dn0) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn0)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn0)) + (2.0 * locals.var_sqrtpsisa_dn0)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn0), (((2.0 * locals.var_t3_dn2) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn2)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn2)) + (2.0 * locals.var_sqrtpsisa_dn2)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn2), (((2.0 * locals.var_t3_dn3) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn3)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn4)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn5)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn6)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn7)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn8)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn9)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn10)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn11)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn11), (((2.0 * locals.var_t3_dn12) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn12)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn12)) + (2.0 * locals.var_sqrtpsisa_dn12)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn12), (((2.0 * locals.var_t3_dn13) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn13)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn13)) + (2.0 * locals.var_sqrtpsisa_dn13)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn13), (((2.0 * locals.var_t3_dn14) + (if assign32240_e42610 >= 1e-38 { (((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32240_e42598 * locals.var_t0_dn14)) * assign32240_e42609) + (assign32240_e42600 * ((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32240_e42603 * locals.var_t0_dn14)) + (2.0 * locals.var_sqrtpsisa_dn14)))) } else { 0.0 } / assign32240_e42612)) - locals.var_t1_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32240_e42619;
        locals.var_t4_dn0 = assign32240_e42619_d_n0;
        locals.var_t4_dn2 = assign32240_e42619_d_n2;
        locals.var_t4_dn3 = assign32240_e42619_d_n3;
        locals.var_t4_dn4 = assign32240_e42619_d_n4;
        locals.var_t4_dn5 = assign32240_e42619_d_n5;
        locals.var_t4_dn6 = assign32240_e42619_d_n6;
        locals.var_t4_dn7 = assign32240_e42619_d_n7;
        locals.var_t4_dn8 = assign32240_e42619_d_n8;
        locals.var_t4_dn9 = assign32240_e42619_d_n9;
        locals.var_t4_dn10 = assign32240_e42619_d_n10;
        locals.var_t4_dn11 = assign32240_e42619_d_n11;
        locals.var_t4_dn12 = assign32240_e42619_d_n12;
        locals.var_t4_dn13 = assign32240_e42619_d_n13;
        locals.var_t4_dn14 = assign32240_e42619_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign32250_e42644, assign32250_e42644_d_n0, assign32250_e42644_d_n2, assign32250_e42644_d_n3, assign32250_e42644_d_n4, assign32250_e42644_d_n5, assign32250_e42644_d_n6, assign32250_e42644_d_n7, assign32250_e42644_d_n8, assign32250_e42644_d_n9, assign32250_e42644_d_n10, assign32250_e42644_d_n11, assign32250_e42644_d_n12, assign32250_e42644_d_n13, assign32250_e42644_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32250_e42627: f64 = 1.0;
        let assign32250_e42629: f64 = (assign32250_e42627 / locals.var_t3);
        let assign32250_e42630: f64 = (2.0 + assign32250_e42629);
        let assign32250_e42634: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32250_e42635: f64 = assign32250_e42634;
        let assign32250_e42638: f64 = (locals.var_t0 * locals.var_t3);
        let assign32250_e42640: f64 = (assign32250_e42638 + locals.var_sqrtpsisa);
        let assign32250_e42641: f64 = (assign32250_e42635 / assign32250_e42640);
        let assign32250_e42642: f64 = (assign32250_e42630 + assign32250_e42641);
        (assign32250_e42642, ((-((assign32250_e42627 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn3) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn12) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32250_e42640 * assign32250_e42640))), ((-((assign32250_e42627 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32250_e42640) - (assign32250_e42635 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32250_e42640 * assign32250_e42640))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32250_e42644;
        locals.var_t5_dn0 = assign32250_e42644_d_n0;
        locals.var_t5_dn2 = assign32250_e42644_d_n2;
        locals.var_t5_dn3 = assign32250_e42644_d_n3;
        locals.var_t5_dn4 = assign32250_e42644_d_n4;
        locals.var_t5_dn5 = assign32250_e42644_d_n5;
        locals.var_t5_dn6 = assign32250_e42644_d_n6;
        locals.var_t5_dn7 = assign32250_e42644_d_n7;
        locals.var_t5_dn8 = assign32250_e42644_d_n8;
        locals.var_t5_dn9 = assign32250_e42644_d_n9;
        locals.var_t5_dn10 = assign32250_e42644_d_n10;
        locals.var_t5_dn11 = assign32250_e42644_d_n11;
        locals.var_t5_dn12 = assign32250_e42644_d_n12;
        locals.var_t5_dn13 = assign32250_e42644_d_n13;
        locals.var_t5_dn14 = assign32250_e42644_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign32260_e42671, assign32260_e42671_d_n0, assign32260_e42671_d_n2, assign32260_e42671_d_n3, assign32260_e42671_d_n4, assign32260_e42671_d_n5, assign32260_e42671_d_n6, assign32260_e42671_d_n7, assign32260_e42671_d_n8, assign32260_e42671_d_n9, assign32260_e42671_d_n10, assign32260_e42671_d_n11, assign32260_e42671_d_n12, assign32260_e42671_d_n13, assign32260_e42671_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32260_e42652: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32260_e42655: f64 = (locals.var_t0 * locals.var_t3);
        let assign32260_e42657: f64 = (assign32260_e42655 + locals.var_sqrtpsisa);
        let assign32260_e42658: f64 = (assign32260_e42652 / assign32260_e42657);
        let assign32260_e42659: f64 = assign32260_e42658;
        let assign32260_e42662: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32260_e42665: f64 = (locals.var_t0 * locals.var_t3);
        let assign32260_e42667: f64 = (assign32260_e42665 + locals.var_sqrtpsisa);
        let assign32260_e42668: f64 = (assign32260_e42662 / assign32260_e42667);
        let assign32260_e42669: f64 = (assign32260_e42659 * assign32260_e42668);
        (assign32260_e42669, ((((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32260_e42667 * assign32260_e42667)))), ((((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32260_e42657) - (assign32260_e42652 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32260_e42657 * assign32260_e42657)) * assign32260_e42668) + (assign32260_e42659 * ((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32260_e42667) - (assign32260_e42662 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32260_e42667 * assign32260_e42667)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign32260_e42671;
        locals.var_t6_dn0 = assign32260_e42671_d_n0;
        locals.var_t6_dn2 = assign32260_e42671_d_n2;
        locals.var_t6_dn3 = assign32260_e42671_d_n3;
        locals.var_t6_dn4 = assign32260_e42671_d_n4;
        locals.var_t6_dn5 = assign32260_e42671_d_n5;
        locals.var_t6_dn6 = assign32260_e42671_d_n6;
        locals.var_t6_dn7 = assign32260_e42671_d_n7;
        locals.var_t6_dn8 = assign32260_e42671_d_n8;
        locals.var_t6_dn9 = assign32260_e42671_d_n9;
        locals.var_t6_dn10 = assign32260_e42671_d_n10;
        locals.var_t6_dn11 = assign32260_e42671_d_n11;
        locals.var_t6_dn12 = assign32260_e42671_d_n12;
        locals.var_t6_dn13 = assign32260_e42671_d_n13;
        locals.var_t6_dn14 = assign32260_e42671_d_n14;
        locals.var_t6_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_104(
        locals: &mut StampLocals,
    ) {
        let (assign32270_e42705, assign32270_e42705_d_n0, assign32270_e42705_d_n2, assign32270_e42705_d_n3, assign32270_e42705_d_n4, assign32270_e42705_d_n5, assign32270_e42705_d_n6, assign32270_e42705_d_n7, assign32270_e42705_d_n8, assign32270_e42705_d_n9, assign32270_e42705_d_n10, assign32270_e42705_d_n11, assign32270_e42705_d_n12, assign32270_e42705_d_n13, assign32270_e42705_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32270_e42677: f64 = (-1.0);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t3;
        let assign32270_e42680: f64 = (1.0 * __rspice_inv_cse_0);
        let assign32270_e42683: f64 = (1.0 * __rspice_inv_cse_0);
        let assign32270_e42684: f64 = (assign32270_e42680 * assign32270_e42683);
        let assign32270_e42685: f64 = (assign32270_e42677 * assign32270_e42684);
        let assign32270_e42688: f64 = 1.0;
        let assign32270_e42691: f64 = (locals.var_sqrtpsisa * locals.var_sqrtpsisa);
        let assign32270_e42693: f64 = (assign32270_e42691 * locals.var_sqrtpsisa);
        let assign32270_e42696: f64 = (locals.var_t0 * locals.var_t3);
        let assign32270_e42698: f64 = (assign32270_e42696 + locals.var_sqrtpsisa);
        let assign32270_e42699: f64 = (assign32270_e42693 * assign32270_e42698);
        let assign32270_e42700: f64 = (assign32270_e42688 / assign32270_e42699);
        let assign32270_e42701: f64 = (assign32270_e42685 - assign32270_e42700);
        let assign32270_e42703: f64 = (assign32270_e42701 - locals.var_t6);
        (assign32270_e42703, (((assign32270_e42677 * (((-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn0 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn0)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn0)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn0), (((assign32270_e42677 * (((-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn2 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn2)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn2)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn2), (((assign32270_e42677 * (((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn3 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn3)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn3)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn3), (((assign32270_e42677 * (((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn4 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn4)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn4)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn4), (((assign32270_e42677 * (((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn5 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn5)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn5)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn5), (((assign32270_e42677 * (((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn6 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn6)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn6)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn6), (((assign32270_e42677 * (((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn7 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn7)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn7)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn7), (((assign32270_e42677 * (((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn8 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn8)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn8)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn8), (((assign32270_e42677 * (((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn9 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn9)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn9)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn9), (((assign32270_e42677 * (((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn10 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn10)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn10)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn10), (((assign32270_e42677 * (((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn11 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn11)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn11)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn11), (((assign32270_e42677 * (((-(locals.var_t3_dn12 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn12 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn12 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn12)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn12)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn12), (((assign32270_e42677 * (((-(locals.var_t3_dn13 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn13 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn13 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn13)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn13)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn13), (((assign32270_e42677 * (((-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3))) * assign32270_e42683) + (assign32270_e42680 * (-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32270_e42688 * ((((((locals.var_sqrtpsisa_dn14 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn14)) * locals.var_sqrtpsisa) + (assign32270_e42691 * locals.var_sqrtpsisa_dn14)) * assign32270_e42698) + (assign32270_e42693 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14)))) / (assign32270_e42699 * assign32270_e42699)))) - locals.var_t6_dn14),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign32270_e42705;
        locals.var_t7_dn0 = assign32270_e42705_d_n0;
        locals.var_t7_dn2 = assign32270_e42705_d_n2;
        locals.var_t7_dn3 = assign32270_e42705_d_n3;
        locals.var_t7_dn4 = assign32270_e42705_d_n4;
        locals.var_t7_dn5 = assign32270_e42705_d_n5;
        locals.var_t7_dn6 = assign32270_e42705_d_n6;
        locals.var_t7_dn7 = assign32270_e42705_d_n7;
        locals.var_t7_dn8 = assign32270_e42705_d_n8;
        locals.var_t7_dn9 = assign32270_e42705_d_n9;
        locals.var_t7_dn10 = assign32270_e42705_d_n10;
        locals.var_t7_dn11 = assign32270_e42705_d_n11;
        locals.var_t7_dn12 = assign32270_e42705_d_n12;
        locals.var_t7_dn13 = assign32270_e42705_d_n13;
        locals.var_t7_dn14 = assign32270_e42705_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign32280_e42728, assign32280_e42728_d_n0, assign32280_e42728_d_n2, assign32280_e42728_d_n3, assign32280_e42728_d_n4, assign32280_e42728_d_n5, assign32280_e42728_d_n6, assign32280_e42728_d_n7, assign32280_e42728_d_n8, assign32280_e42728_d_n9, assign32280_e42728_d_n10, assign32280_e42728_d_n11, assign32280_e42728_d_n12, assign32280_e42728_d_n13, assign32280_e42728_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard736 == 0.0)) {
        let assign32280_e42713: f64 = (locals.var_t4 / locals.var_t5);
        let assign32280_e42717: f64 = (locals.var_t4 * locals.var_t7);
        let assign32280_e42720: f64 = (2.0 * locals.var_t5);
        let assign32280_e42722: f64 = (assign32280_e42720 * locals.var_t5);
        let assign32280_e42723: f64 = (assign32280_e42717 / assign32280_e42722);
        let assign32280_e42724: f64 = (1.0 + assign32280_e42723);
        let assign32280_e42725: f64 = (assign32280_e42713 * assign32280_e42724);
        let assign32280_e42726: f64 = (locals.var_t3 - assign32280_e42725);
        (assign32280_e42726, (locals.var_t3_dn0 - (((((locals.var_t4_dn0 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn0 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn0)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn0) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn0)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn2 - (((((locals.var_t4_dn2 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn2 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn2)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn2) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn2)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn3 - (((((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn3) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn3)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn4 - (((((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn4) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn4)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn5 - (((((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn5) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn5)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn6 - (((((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn6) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn6)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn7 - (((((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn7) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn7)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn8 - (((((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn8) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn8)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn9 - (((((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn9) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn9)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn10 - (((((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn10) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn10)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn11 - (((((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn11) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn11)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn12 - (((((locals.var_t4_dn12 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn12 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn12)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn12) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn12)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn13 - (((((locals.var_t4_dn13 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn13 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn13)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn13) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn13)))) / (assign32280_e42722 * assign32280_e42722))))), (locals.var_t3_dn14 - (((((locals.var_t4_dn14 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)) * assign32280_e42724) + (assign32280_e42713 * (((((locals.var_t4_dn14 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn14)) * assign32280_e42722) - (assign32280_e42717 * (((2.0 * locals.var_t5_dn14) * locals.var_t5) + (assign32280_e42720 * locals.var_t5_dn14)))) / (assign32280_e42722 * assign32280_e42722))))),)
    } else {
        (locals.var_qs_edge, locals.var_qs_edge_dn0, locals.var_qs_edge_dn2, locals.var_qs_edge_dn3, locals.var_qs_edge_dn4, locals.var_qs_edge_dn5, locals.var_qs_edge_dn6, locals.var_qs_edge_dn7, locals.var_qs_edge_dn8, locals.var_qs_edge_dn9, locals.var_qs_edge_dn10, locals.var_qs_edge_dn11, locals.var_qs_edge_dn12, locals.var_qs_edge_dn13, locals.var_qs_edge_dn14,)
    }
};
        locals.var_qs_edge = assign32280_e42728;
        locals.var_qs_edge_dn0 = assign32280_e42728_d_n0;
        locals.var_qs_edge_dn2 = assign32280_e42728_d_n2;
        locals.var_qs_edge_dn3 = assign32280_e42728_d_n3;
        locals.var_qs_edge_dn4 = assign32280_e42728_d_n4;
        locals.var_qs_edge_dn5 = assign32280_e42728_d_n5;
        locals.var_qs_edge_dn6 = assign32280_e42728_d_n6;
        locals.var_qs_edge_dn7 = assign32280_e42728_d_n7;
        locals.var_qs_edge_dn8 = assign32280_e42728_d_n8;
        locals.var_qs_edge_dn9 = assign32280_e42728_d_n9;
        locals.var_qs_edge_dn10 = assign32280_e42728_d_n10;
        locals.var_qs_edge_dn11 = assign32280_e42728_d_n11;
        locals.var_qs_edge_dn12 = assign32280_e42728_d_n12;
        locals.var_qs_edge_dn13 = assign32280_e42728_d_n13;
        locals.var_qs_edge_dn14 = assign32280_e42728_d_n14;
        locals.var_qs_edge_rv = 0.0;

        let (assign32290_e42740, assign32290_e42740_d_n0, assign32290_e42740_d_n2, assign32290_e42740_d_n3, assign32290_e42740_d_n4, assign32290_e42740_d_n5, assign32290_e42740_d_n6, assign32290_e42740_d_n7, assign32290_e42740_d_n8, assign32290_e42740_d_n9, assign32290_e42740_d_n10, assign32290_e42740_d_n11, assign32290_e42740_d_n12, assign32290_e42740_d_n13, assign32290_e42740_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32290_e42732: f64 = (2.0 * locals.var_nvt);
        let assign32290_e42734: f64 = (assign32290_e42732 * locals.var_qs_edge);
        let assign32290_e42737: f64 = (2.0 * locals.var_nvt);
        let assign32290_e42738: f64 = (assign32290_e42734 + assign32290_e42737);
        (assign32290_e42738, ((((2.0 * locals.var_nvt_dn0) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn0)) + (2.0 * locals.var_nvt_dn0)), ((((2.0 * locals.var_nvt_dn2) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn2)) + (2.0 * locals.var_nvt_dn2)), ((((2.0 * locals.var_nvt_dn3) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn3)) + (2.0 * locals.var_nvt_dn3)), ((((2.0 * locals.var_nvt_dn4) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn4)) + (2.0 * locals.var_nvt_dn4)), ((((2.0 * locals.var_nvt_dn5) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn5)) + (2.0 * locals.var_nvt_dn5)), ((((2.0 * locals.var_nvt_dn6) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn6)) + (2.0 * locals.var_nvt_dn6)), ((((2.0 * locals.var_nvt_dn7) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn7)) + (2.0 * locals.var_nvt_dn7)), ((((2.0 * locals.var_nvt_dn8) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn8)) + (2.0 * locals.var_nvt_dn8)), ((((2.0 * locals.var_nvt_dn9) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn9)) + (2.0 * locals.var_nvt_dn9)), ((((2.0 * locals.var_nvt_dn10) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn10)) + (2.0 * locals.var_nvt_dn10)), ((((2.0 * locals.var_nvt_dn11) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn11)) + (2.0 * locals.var_nvt_dn11)), ((((2.0 * locals.var_nvt_dn12) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn12)) + (2.0 * locals.var_nvt_dn12)), ((((2.0 * locals.var_nvt_dn13) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn13)) + (2.0 * locals.var_nvt_dn13)), ((((2.0 * locals.var_nvt_dn14) * locals.var_qs_edge) + (assign32290_e42732 * locals.var_qs_edge_dn14)) + (2.0 * locals.var_nvt_dn14)),)
    } else {
        (locals.var_vdsatedge, locals.var_vdsatedge_dn0, locals.var_vdsatedge_dn2, locals.var_vdsatedge_dn3, locals.var_vdsatedge_dn4, locals.var_vdsatedge_dn5, locals.var_vdsatedge_dn6, locals.var_vdsatedge_dn7, locals.var_vdsatedge_dn8, locals.var_vdsatedge_dn9, locals.var_vdsatedge_dn10, locals.var_vdsatedge_dn11, locals.var_vdsatedge_dn12, locals.var_vdsatedge_dn13, locals.var_vdsatedge_dn14,)
    }
};
        locals.var_vdsatedge = assign32290_e42740;
        locals.var_vdsatedge_dn0 = assign32290_e42740_d_n0;
        locals.var_vdsatedge_dn2 = assign32290_e42740_d_n2;
        locals.var_vdsatedge_dn3 = assign32290_e42740_d_n3;
        locals.var_vdsatedge_dn4 = assign32290_e42740_d_n4;
        locals.var_vdsatedge_dn5 = assign32290_e42740_d_n5;
        locals.var_vdsatedge_dn6 = assign32290_e42740_d_n6;
        locals.var_vdsatedge_dn7 = assign32290_e42740_d_n7;
        locals.var_vdsatedge_dn8 = assign32290_e42740_d_n8;
        locals.var_vdsatedge_dn9 = assign32290_e42740_d_n9;
        locals.var_vdsatedge_dn10 = assign32290_e42740_d_n10;
        locals.var_vdsatedge_dn11 = assign32290_e42740_d_n11;
        locals.var_vdsatedge_dn12 = assign32290_e42740_d_n12;
        locals.var_vdsatedge_dn13 = assign32290_e42740_d_n13;
        locals.var_vdsatedge_dn14 = assign32290_e42740_d_n14;
        locals.var_vdsatedge_rv = 0.0;

        let (assign32300_e42744, assign32300_e42744_d_n0, assign32300_e42744_d_n2, assign32300_e42744_d_n3, assign32300_e42744_d_n4, assign32300_e42744_d_n5, assign32300_e42744_d_n6, assign32300_e42744_d_n7, assign32300_e42744_d_n8, assign32300_e42744_d_n9, assign32300_e42744_d_n10, assign32300_e42744_d_n11, assign32300_e42744_d_n12, assign32300_e42744_d_n13, assign32300_e42744_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        (locals.var_vdsatedge, locals.var_vdsatedge_dn0, locals.var_vdsatedge_dn2, locals.var_vdsatedge_dn3, locals.var_vdsatedge_dn4, locals.var_vdsatedge_dn5, locals.var_vdsatedge_dn6, locals.var_vdsatedge_dn7, locals.var_vdsatedge_dn8, locals.var_vdsatedge_dn9, locals.var_vdsatedge_dn10, locals.var_vdsatedge_dn11, locals.var_vdsatedge_dn12, locals.var_vdsatedge_dn13, locals.var_vdsatedge_dn14,)
    } else {
        (locals.var_vdsatedge_1, locals.var_vdsatedge_1_dn0, locals.var_vdsatedge_1_dn2, locals.var_vdsatedge_1_dn3, locals.var_vdsatedge_1_dn4, locals.var_vdsatedge_1_dn5, locals.var_vdsatedge_1_dn6, locals.var_vdsatedge_1_dn7, locals.var_vdsatedge_1_dn8, locals.var_vdsatedge_1_dn9, locals.var_vdsatedge_1_dn10, locals.var_vdsatedge_1_dn11, locals.var_vdsatedge_1_dn12, locals.var_vdsatedge_1_dn13, locals.var_vdsatedge_1_dn14,)
    }
};
        locals.var_vdsatedge_1 = assign32300_e42744;
        locals.var_vdsatedge_1_dn0 = assign32300_e42744_d_n0;
        locals.var_vdsatedge_1_dn2 = assign32300_e42744_d_n2;
        locals.var_vdsatedge_1_dn3 = assign32300_e42744_d_n3;
        locals.var_vdsatedge_1_dn4 = assign32300_e42744_d_n4;
        locals.var_vdsatedge_1_dn5 = assign32300_e42744_d_n5;
        locals.var_vdsatedge_1_dn6 = assign32300_e42744_d_n6;
        locals.var_vdsatedge_1_dn7 = assign32300_e42744_d_n7;
        locals.var_vdsatedge_1_dn8 = assign32300_e42744_d_n8;
        locals.var_vdsatedge_1_dn9 = assign32300_e42744_d_n9;
        locals.var_vdsatedge_1_dn10 = assign32300_e42744_d_n10;
        locals.var_vdsatedge_1_dn11 = assign32300_e42744_d_n11;
        locals.var_vdsatedge_1_dn12 = assign32300_e42744_d_n12;
        locals.var_vdsatedge_1_dn13 = assign32300_e42744_d_n13;
        locals.var_vdsatedge_1_dn14 = assign32300_e42744_d_n14;
        locals.var_vdsatedge_1_rv = 0.0;

        let (assign32310_e42750, assign32310_e42750_d_n0, assign32310_e42750_d_n2, assign32310_e42750_d_n3, assign32310_e42750_d_n4, assign32310_e42750_d_n5, assign32310_e42750_d_n6, assign32310_e42750_d_n7, assign32310_e42750_d_n8, assign32310_e42750_d_n9, assign32310_e42750_d_n10, assign32310_e42750_d_n11, assign32310_e42750_d_n12, assign32310_e42750_d_n13, assign32310_e42750_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32310_e42748: f64 = (locals.var_vdsatedge_1 + locals.var_vs);
        (assign32310_e42748, locals.var_vdsatedge_1_dn0, locals.var_vdsatedge_1_dn2, locals.var_vdsatedge_1_dn3, locals.var_vdsatedge_1_dn4, (locals.var_vdsatedge_1_dn5 + locals.var_vs_dn5), locals.var_vdsatedge_1_dn6, (locals.var_vdsatedge_1_dn7 + locals.var_vs_dn7), locals.var_vdsatedge_1_dn8, locals.var_vdsatedge_1_dn9, locals.var_vdsatedge_1_dn10, (locals.var_vdsatedge_1_dn11 + locals.var_vs_dn11), locals.var_vdsatedge_1_dn12, locals.var_vdsatedge_1_dn13, locals.var_vdsatedge_1_dn14,)
    } else {
        (locals.var_vdsatedge_1, locals.var_vdsatedge_1_dn0, locals.var_vdsatedge_1_dn2, locals.var_vdsatedge_1_dn3, locals.var_vdsatedge_1_dn4, locals.var_vdsatedge_1_dn5, locals.var_vdsatedge_1_dn6, locals.var_vdsatedge_1_dn7, locals.var_vdsatedge_1_dn8, locals.var_vdsatedge_1_dn9, locals.var_vdsatedge_1_dn10, locals.var_vdsatedge_1_dn11, locals.var_vdsatedge_1_dn12, locals.var_vdsatedge_1_dn13, locals.var_vdsatedge_1_dn14,)
    }
};
        locals.var_vdsatedge_1 = assign32310_e42750;
        locals.var_vdsatedge_1_dn0 = assign32310_e42750_d_n0;
        locals.var_vdsatedge_1_dn2 = assign32310_e42750_d_n2;
        locals.var_vdsatedge_1_dn3 = assign32310_e42750_d_n3;
        locals.var_vdsatedge_1_dn4 = assign32310_e42750_d_n4;
        locals.var_vdsatedge_1_dn5 = assign32310_e42750_d_n5;
        locals.var_vdsatedge_1_dn6 = assign32310_e42750_d_n6;
        locals.var_vdsatedge_1_dn7 = assign32310_e42750_d_n7;
        locals.var_vdsatedge_1_dn8 = assign32310_e42750_d_n8;
        locals.var_vdsatedge_1_dn9 = assign32310_e42750_d_n9;
        locals.var_vdsatedge_1_dn10 = assign32310_e42750_d_n10;
        locals.var_vdsatedge_1_dn11 = assign32310_e42750_d_n11;
        locals.var_vdsatedge_1_dn12 = assign32310_e42750_d_n12;
        locals.var_vdsatedge_1_dn13 = assign32310_e42750_d_n13;
        locals.var_vdsatedge_1_dn14 = assign32310_e42750_d_n14;
        locals.var_vdsatedge_1_rv = 0.0;

        let assign32320_e42756: f64 = (locals.var_vdsatedge_1 - locals.var_vs);
        let assign32320_e42758: f64 = (-2500.0);
        let assign32320_e42760: f64 = (assign32320_e42758 * 0.001);
        let assign32320_e42762: f64 = if ((0.0 == 0.0) && (assign32320_e42756 < assign32320_e42760)) { 1.0 } else { 0.0 };
        locals.var_guard739 = assign32320_e42762;
        locals.var_guard739_rv = 0.0;

        let (assign32330_e42777, assign32330_e42777_d_n0, assign32330_e42777_d_n2, assign32330_e42777_d_n3, assign32330_e42777_d_n4, assign32330_e42777_d_n5, assign32330_e42777_d_n6, assign32330_e42777_d_n7, assign32330_e42777_d_n8, assign32330_e42777_d_n9, assign32330_e42777_d_n10, assign32330_e42777_d_n11, assign32330_e42777_d_n12, assign32330_e42777_d_n13, assign32330_e42777_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard739 != 0.0)) {
        let assign32330_e42767: f64 = (-0.001);
        let assign32330_e42769: f64 = (assign32330_e42767 * 0.001);
        let assign32330_e42773: f64 = (locals.var_vdsatedge_1 - locals.var_vs);
        let assign32330_e42774: f64 = (16.0 * assign32330_e42773);
        let assign32330_e42775: f64 = (assign32330_e42769 / assign32330_e42774);
        (assign32330_e42775, (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn0)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn2)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn3)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn4)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * (locals.var_vdsatedge_1_dn5 - locals.var_vs_dn5))) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn6)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * (locals.var_vdsatedge_1_dn7 - locals.var_vs_dn7))) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn8)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn9)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn10)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * (locals.var_vdsatedge_1_dn11 - locals.var_vs_dn11))) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn12)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn13)) / (assign32330_e42774 * assign32330_e42774))), (-((assign32330_e42769 * (16.0 * locals.var_vdsatedge_1_dn14)) / (assign32330_e42774 * assign32330_e42774))),)
    } else {
        (locals.var_vdssate, locals.var_vdssate_dn0, locals.var_vdssate_dn2, locals.var_vdssate_dn3, locals.var_vdssate_dn4, locals.var_vdssate_dn5, locals.var_vdssate_dn6, locals.var_vdssate_dn7, locals.var_vdssate_dn8, locals.var_vdssate_dn9, locals.var_vdssate_dn10, locals.var_vdssate_dn11, locals.var_vdssate_dn12, locals.var_vdssate_dn13, locals.var_vdssate_dn14,)
    }
};
        locals.var_vdssate = assign32330_e42777;
        locals.var_vdssate_dn0 = assign32330_e42777_d_n0;
        locals.var_vdssate_dn2 = assign32330_e42777_d_n2;
        locals.var_vdssate_dn3 = assign32330_e42777_d_n3;
        locals.var_vdssate_dn4 = assign32330_e42777_d_n4;
        locals.var_vdssate_dn5 = assign32330_e42777_d_n5;
        locals.var_vdssate_dn6 = assign32330_e42777_d_n6;
        locals.var_vdssate_dn7 = assign32330_e42777_d_n7;
        locals.var_vdssate_dn8 = assign32330_e42777_d_n8;
        locals.var_vdssate_dn9 = assign32330_e42777_d_n9;
        locals.var_vdssate_dn10 = assign32330_e42777_d_n10;
        locals.var_vdssate_dn11 = assign32330_e42777_d_n11;
        locals.var_vdssate_dn12 = assign32330_e42777_d_n12;
        locals.var_vdssate_dn13 = assign32330_e42777_d_n13;
        locals.var_vdssate_dn14 = assign32330_e42777_d_n14;
        locals.var_vdssate_rv = 0.0;

        let (assign32340_e42809, assign32340_e42809_d_n0, assign32340_e42809_d_n2, assign32340_e42809_d_n3, assign32340_e42809_d_n4, assign32340_e42809_d_n5, assign32340_e42809_d_n6, assign32340_e42809_d_n7, assign32340_e42809_d_n8, assign32340_e42809_d_n9, assign32340_e42809_d_n10, assign32340_e42809_d_n11, assign32340_e42809_d_n12, assign32340_e42809_d_n13, assign32340_e42809_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard739 == 0.0)) {
        let assign32340_e42785: f64 = (locals.var_vdsatedge_1 - locals.var_vs);
        let assign32340_e42787: f64 = assign32340_e42785;
        let assign32340_e42790: f64 = (locals.var_vdsatedge_1 - locals.var_vs);
        let assign32340_e42792: f64 = assign32340_e42790;
        let assign32340_e42795: f64 = (locals.var_vdsatedge_1 - locals.var_vs);
        let assign32340_e42797: f64 = assign32340_e42795;
        let assign32340_e42798: f64 = (assign32340_e42792 * assign32340_e42797);
        let assign32340_e42801: f64 = (0.25 * 0.001);
        let assign32340_e42803: f64 = (assign32340_e42801 * 0.001);
        let assign32340_e42804: f64 = (assign32340_e42798 + assign32340_e42803);
        let assign32340_e42805: f64 = (assign32340_e42804).sqrt();
        let assign32340_e42806: f64 = (assign32340_e42787 + assign32340_e42805);
        let assign32340_e42807: f64 = (0.5 * assign32340_e42806);
        (assign32340_e42807, (0.5 * (locals.var_vdsatedge_1_dn0 + (((locals.var_vdsatedge_1_dn0 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn0)) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn2 + (((locals.var_vdsatedge_1_dn2 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn2)) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn3 + (((locals.var_vdsatedge_1_dn3 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn3)) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn4 + (((locals.var_vdsatedge_1_dn4 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn4)) / (2.0 * assign32340_e42805)))), (0.5 * ((locals.var_vdsatedge_1_dn5 - locals.var_vs_dn5) + ((((locals.var_vdsatedge_1_dn5 - locals.var_vs_dn5) * assign32340_e42797) + (assign32340_e42792 * (locals.var_vdsatedge_1_dn5 - locals.var_vs_dn5))) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn6 + (((locals.var_vdsatedge_1_dn6 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn6)) / (2.0 * assign32340_e42805)))), (0.5 * ((locals.var_vdsatedge_1_dn7 - locals.var_vs_dn7) + ((((locals.var_vdsatedge_1_dn7 - locals.var_vs_dn7) * assign32340_e42797) + (assign32340_e42792 * (locals.var_vdsatedge_1_dn7 - locals.var_vs_dn7))) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn8 + (((locals.var_vdsatedge_1_dn8 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn8)) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn9 + (((locals.var_vdsatedge_1_dn9 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn9)) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn10 + (((locals.var_vdsatedge_1_dn10 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn10)) / (2.0 * assign32340_e42805)))), (0.5 * ((locals.var_vdsatedge_1_dn11 - locals.var_vs_dn11) + ((((locals.var_vdsatedge_1_dn11 - locals.var_vs_dn11) * assign32340_e42797) + (assign32340_e42792 * (locals.var_vdsatedge_1_dn11 - locals.var_vs_dn11))) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn12 + (((locals.var_vdsatedge_1_dn12 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn12)) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn13 + (((locals.var_vdsatedge_1_dn13 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn13)) / (2.0 * assign32340_e42805)))), (0.5 * (locals.var_vdsatedge_1_dn14 + (((locals.var_vdsatedge_1_dn14 * assign32340_e42797) + (assign32340_e42792 * locals.var_vdsatedge_1_dn14)) / (2.0 * assign32340_e42805)))),)
    } else {
        (locals.var_vdssate, locals.var_vdssate_dn0, locals.var_vdssate_dn2, locals.var_vdssate_dn3, locals.var_vdssate_dn4, locals.var_vdssate_dn5, locals.var_vdssate_dn6, locals.var_vdssate_dn7, locals.var_vdssate_dn8, locals.var_vdssate_dn9, locals.var_vdssate_dn10, locals.var_vdssate_dn11, locals.var_vdssate_dn12, locals.var_vdssate_dn13, locals.var_vdssate_dn14,)
    }
};
        locals.var_vdssate = assign32340_e42809;
        locals.var_vdssate_dn0 = assign32340_e42809_d_n0;
        locals.var_vdssate_dn2 = assign32340_e42809_d_n2;
        locals.var_vdssate_dn3 = assign32340_e42809_d_n3;
        locals.var_vdssate_dn4 = assign32340_e42809_d_n4;
        locals.var_vdssate_dn5 = assign32340_e42809_d_n5;
        locals.var_vdssate_dn6 = assign32340_e42809_d_n6;
        locals.var_vdssate_dn7 = assign32340_e42809_d_n7;
        locals.var_vdssate_dn8 = assign32340_e42809_d_n8;
        locals.var_vdssate_dn9 = assign32340_e42809_d_n9;
        locals.var_vdssate_dn10 = assign32340_e42809_d_n10;
        locals.var_vdssate_dn11 = assign32340_e42809_d_n11;
        locals.var_vdssate_dn12 = assign32340_e42809_d_n12;
        locals.var_vdssate_dn13 = assign32340_e42809_d_n13;
        locals.var_vdssate_dn14 = assign32340_e42809_d_n14;
        locals.var_vdssate_rv = 0.0;

        let (assign32350_e42819, assign32350_e42819_d_n0, assign32350_e42819_d_n2, assign32350_e42819_d_n3, assign32350_e42819_d_n4, assign32350_e42819_d_n5, assign32350_e42819_d_n6, assign32350_e42819_d_n7, assign32350_e42819_d_n8, assign32350_e42819_d_n9, assign32350_e42819_d_n10, assign32350_e42819_d_n11, assign32350_e42819_d_n12, assign32350_e42819_d_n13, assign32350_e42819_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32350_e42813: f64 = (locals.var_vds / locals.var_vdssate);
        let assign32350_e42816: f64 = (1.0 / locals.var_delta_t);
        let assign32350_e42817: f64 = (assign32350_e42813).powf(assign32350_e42816);
        (assign32350_e42817, if (-(locals.var_delta_t_dn0 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn0) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn0 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn0) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn2 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn2) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn2 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn2) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn3 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn3) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn3 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn3) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn4 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn4) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn4 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn4) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn5 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (((locals.var_vds_dn5 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn5)) / (locals.var_vdssate * locals.var_vdssate)))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn5 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((((locals.var_vds_dn5 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn5)) / (locals.var_vdssate * locals.var_vdssate)) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn6 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn6) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn6 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn6) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn7 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (((locals.var_vds_dn7 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn7)) / (locals.var_vdssate * locals.var_vdssate)))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn7 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((((locals.var_vds_dn7 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn7)) / (locals.var_vdssate * locals.var_vdssate)) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn8 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn8) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn8 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn8) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn9 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn9) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn9 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn9) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn10 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn10) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn10 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn10) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn11 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (((locals.var_vds_dn11 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn11)) / (locals.var_vdssate * locals.var_vdssate)))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn11 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((((locals.var_vds_dn11 * locals.var_vdssate) - (locals.var_vds * locals.var_vdssate_dn11)) / (locals.var_vdssate * locals.var_vdssate)) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn12 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn12) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn12 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn12) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn13 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn13) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn13 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn13) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) }, if (-(locals.var_delta_t_dn14 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign32350_e42816) as f64).is_finite() && ((assign32350_e42816) as f64).fract() == 0.0 { if assign32350_e42816 == 0.0 { 0.0 } else { (assign32350_e42816 * ((assign32350_e42813).powf(assign32350_e42816 - 1.0) * (-((locals.var_vds * locals.var_vdssate_dn14) / (locals.var_vdssate * locals.var_vdssate))))) } } else { (assign32350_e42817 * (((-(locals.var_delta_t_dn14 / (locals.var_delta_t * locals.var_delta_t))) * (assign32350_e42813).ln()) + (assign32350_e42816 * ((-((locals.var_vds * locals.var_vdssate_dn14) / (locals.var_vdssate * locals.var_vdssate))) / assign32350_e42813)))) },)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign32350_e42819;
        locals.var_t7_dn0 = assign32350_e42819_d_n0;
        locals.var_t7_dn2 = assign32350_e42819_d_n2;
        locals.var_t7_dn3 = assign32350_e42819_d_n3;
        locals.var_t7_dn4 = assign32350_e42819_d_n4;
        locals.var_t7_dn5 = assign32350_e42819_d_n5;
        locals.var_t7_dn6 = assign32350_e42819_d_n6;
        locals.var_t7_dn7 = assign32350_e42819_d_n7;
        locals.var_t7_dn8 = assign32350_e42819_d_n8;
        locals.var_t7_dn9 = assign32350_e42819_d_n9;
        locals.var_t7_dn10 = assign32350_e42819_d_n10;
        locals.var_t7_dn11 = assign32350_e42819_d_n11;
        locals.var_t7_dn12 = assign32350_e42819_d_n12;
        locals.var_t7_dn13 = assign32350_e42819_d_n13;
        locals.var_t7_dn14 = assign32350_e42819_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign32360_e42828, assign32360_e42828_d_n0, assign32360_e42828_d_n2, assign32360_e42828_d_n3, assign32360_e42828_d_n4, assign32360_e42828_d_n5, assign32360_e42828_d_n6, assign32360_e42828_d_n7, assign32360_e42828_d_n8, assign32360_e42828_d_n9, assign32360_e42828_d_n10, assign32360_e42828_d_n11, assign32360_e42828_d_n12, assign32360_e42828_d_n13, assign32360_e42828_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32360_e42823: f64 = (1.0 + locals.var_t7);
        let assign32360_e42825: f64 = (-locals.var_delta_t);
        let assign32360_e42826: f64 = (assign32360_e42823).powf(assign32360_e42825);
        (assign32360_e42826, if (-locals.var_delta_t_dn0) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn0)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn0) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn0 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn2) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn2)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn2) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn2 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn3) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn3)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn3) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn3 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn4) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn4)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn4) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn4 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn5) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn5)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn5) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn5 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn6) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn6)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn6) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn6 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn7) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn7)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn7) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn7 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn8) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn8)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn8) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn8 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn9) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn9)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn9) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn9 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn10) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn10)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn10) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn10 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn11) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn11)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn11) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn11 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn12) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn12)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn12) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn12 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn13) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn13)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn13) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn13 / assign32360_e42823)))) }, if (-locals.var_delta_t_dn14) == 0.0 && ((assign32360_e42825) as f64).is_finite() && ((assign32360_e42825) as f64).fract() == 0.0 { if assign32360_e42825 == 0.0 { 0.0 } else { (assign32360_e42825 * ((assign32360_e42823).powf(assign32360_e42825 - 1.0) * locals.var_t7_dn14)) } } else { (assign32360_e42826 * (((-locals.var_delta_t_dn14) * (assign32360_e42823).ln()) + (assign32360_e42825 * (locals.var_t7_dn14 / assign32360_e42823)))) },)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign32360_e42828;
        locals.var_t8_dn0 = assign32360_e42828_d_n0;
        locals.var_t8_dn2 = assign32360_e42828_d_n2;
        locals.var_t8_dn3 = assign32360_e42828_d_n3;
        locals.var_t8_dn4 = assign32360_e42828_d_n4;
        locals.var_t8_dn5 = assign32360_e42828_d_n5;
        locals.var_t8_dn6 = assign32360_e42828_d_n6;
        locals.var_t8_dn7 = assign32360_e42828_d_n7;
        locals.var_t8_dn8 = assign32360_e42828_d_n8;
        locals.var_t8_dn9 = assign32360_e42828_d_n9;
        locals.var_t8_dn10 = assign32360_e42828_d_n10;
        locals.var_t8_dn11 = assign32360_e42828_d_n11;
        locals.var_t8_dn12 = assign32360_e42828_d_n12;
        locals.var_t8_dn13 = assign32360_e42828_d_n13;
        locals.var_t8_dn14 = assign32360_e42828_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign32370_e42834, assign32370_e42834_d_n0, assign32370_e42834_d_n2, assign32370_e42834_d_n3, assign32370_e42834_d_n4, assign32370_e42834_d_n5, assign32370_e42834_d_n6, assign32370_e42834_d_n7, assign32370_e42834_d_n8, assign32370_e42834_d_n9, assign32370_e42834_d_n10, assign32370_e42834_d_n11, assign32370_e42834_d_n12, assign32370_e42834_d_n13, assign32370_e42834_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32370_e42832: f64 = (locals.var_vds * locals.var_t8);
        (assign32370_e42832, (locals.var_vds * locals.var_t8_dn0), (locals.var_vds * locals.var_t8_dn2), (locals.var_vds * locals.var_t8_dn3), (locals.var_vds * locals.var_t8_dn4), ((locals.var_vds_dn5 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn5)), (locals.var_vds * locals.var_t8_dn6), ((locals.var_vds_dn7 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn7)), (locals.var_vds * locals.var_t8_dn8), (locals.var_vds * locals.var_t8_dn9), (locals.var_vds * locals.var_t8_dn10), ((locals.var_vds_dn11 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn11)), (locals.var_vds * locals.var_t8_dn12), (locals.var_vds * locals.var_t8_dn13), (locals.var_vds * locals.var_t8_dn14),)
    } else {
        (locals.var_vdseff, locals.var_vdseff_dn0, locals.var_vdseff_dn2, locals.var_vdseff_dn3, locals.var_vdseff_dn4, locals.var_vdseff_dn5, locals.var_vdseff_dn6, locals.var_vdseff_dn7, locals.var_vdseff_dn8, locals.var_vdseff_dn9, locals.var_vdseff_dn10, locals.var_vdseff_dn11, locals.var_vdseff_dn12, locals.var_vdseff_dn13, locals.var_vdseff_dn14,)
    }
};
        locals.var_vdseff = assign32370_e42834;
        locals.var_vdseff_dn0 = assign32370_e42834_d_n0;
        locals.var_vdseff_dn2 = assign32370_e42834_d_n2;
        locals.var_vdseff_dn3 = assign32370_e42834_d_n3;
        locals.var_vdseff_dn4 = assign32370_e42834_d_n4;
        locals.var_vdseff_dn5 = assign32370_e42834_d_n5;
        locals.var_vdseff_dn6 = assign32370_e42834_d_n6;
        locals.var_vdseff_dn7 = assign32370_e42834_d_n7;
        locals.var_vdseff_dn8 = assign32370_e42834_d_n8;
        locals.var_vdseff_dn9 = assign32370_e42834_d_n9;
        locals.var_vdseff_dn10 = assign32370_e42834_d_n10;
        locals.var_vdseff_dn11 = assign32370_e42834_d_n11;
        locals.var_vdseff_dn12 = assign32370_e42834_d_n12;
        locals.var_vdseff_dn13 = assign32370_e42834_d_n13;
        locals.var_vdseff_dn14 = assign32370_e42834_d_n14;
        locals.var_vdseff_rv = 0.0;

        let (assign32380_e42842, assign32380_e42842_d_n0, assign32380_e42842_d_n2, assign32380_e42842_d_n3, assign32380_e42842_d_n4, assign32380_e42842_d_n5, assign32380_e42842_d_n6, assign32380_e42842_d_n7, assign32380_e42842_d_n8, assign32380_e42842_d_n9, assign32380_e42842_d_n10, assign32380_e42842_d_n11, assign32380_e42842_d_n12, assign32380_e42842_d_n13, assign32380_e42842_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32380_e42838: f64 = (locals.var_vdseff + locals.var_vs);
        let assign32380_e42840: f64 = (assign32380_e42838 * locals.var_inv_nvt);
        (assign32380_e42840, ((locals.var_vdseff_dn0 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn0)), ((locals.var_vdseff_dn2 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn2)), ((locals.var_vdseff_dn3 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn3)), ((locals.var_vdseff_dn4 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn4)), (((locals.var_vdseff_dn5 + locals.var_vs_dn5) * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn5)), ((locals.var_vdseff_dn6 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn6)), (((locals.var_vdseff_dn7 + locals.var_vs_dn7) * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn7)), ((locals.var_vdseff_dn8 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn8)), ((locals.var_vdseff_dn9 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn9)), ((locals.var_vdseff_dn10 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn10)), (((locals.var_vdseff_dn11 + locals.var_vs_dn11) * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn11)), ((locals.var_vdseff_dn12 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn12)), ((locals.var_vdseff_dn13 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn13)), ((locals.var_vdseff_dn14 * locals.var_inv_nvt) + (assign32380_e42838 * locals.var_inv_nvt_dn14)),)
    } else {
        (locals.var_vdeff, locals.var_vdeff_dn0, locals.var_vdeff_dn2, locals.var_vdeff_dn3, locals.var_vdeff_dn4, locals.var_vdeff_dn5, locals.var_vdeff_dn6, locals.var_vdeff_dn7, locals.var_vdeff_dn8, locals.var_vdeff_dn9, locals.var_vdeff_dn10, locals.var_vdeff_dn11, locals.var_vdeff_dn12, locals.var_vdeff_dn13, locals.var_vdeff_dn14,)
    }
};
        locals.var_vdeff = assign32380_e42842;
        locals.var_vdeff_dn0 = assign32380_e42842_d_n0;
        locals.var_vdeff_dn2 = assign32380_e42842_d_n2;
        locals.var_vdeff_dn3 = assign32380_e42842_d_n3;
        locals.var_vdeff_dn4 = assign32380_e42842_d_n4;
        locals.var_vdeff_dn5 = assign32380_e42842_d_n5;
        locals.var_vdeff_dn6 = assign32380_e42842_d_n6;
        locals.var_vdeff_dn7 = assign32380_e42842_d_n7;
        locals.var_vdeff_dn8 = assign32380_e42842_d_n8;
        locals.var_vdeff_dn9 = assign32380_e42842_d_n9;
        locals.var_vdeff_dn10 = assign32380_e42842_d_n10;
        locals.var_vdeff_dn11 = assign32380_e42842_d_n11;
        locals.var_vdeff_dn12 = assign32380_e42842_d_n12;
        locals.var_vdeff_dn13 = assign32380_e42842_d_n13;
        locals.var_vdeff_dn14 = assign32380_e42842_d_n14;
        locals.var_vdeff_rv = 0.0;

        let (assign32390_e42865, assign32390_e42865_d_n0, assign32390_e42865_d_n2, assign32390_e42865_d_n3, assign32390_e42865_d_n4, assign32390_e42865_d_n5, assign32390_e42865_d_n6, assign32390_e42865_d_n7, assign32390_e42865_d_n8, assign32390_e42865_d_n9, assign32390_e42865_d_n10, assign32390_e42865_d_n11, assign32390_e42865_d_n12, assign32390_e42865_d_n13, assign32390_e42865_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32390_e42847: f64 = (locals.var_psip + 1.0);
        let assign32390_e42850: f64 = (locals.var_psip - 1.0);
        let assign32390_e42853: f64 = (locals.var_psip - 1.0);
        let assign32390_e42854: f64 = (assign32390_e42850 * assign32390_e42853);
        let assign32390_e42857: f64 = (0.25 * 2.0);
        let assign32390_e42859: f64 = (assign32390_e42857 * 2.0);
        let assign32390_e42860: f64 = (assign32390_e42854 + assign32390_e42859);
        let assign32390_e42861: f64 = (assign32390_e42860).sqrt();
        let assign32390_e42862: f64 = (assign32390_e42847 + assign32390_e42861);
        let assign32390_e42863: f64 = (0.5 * assign32390_e42862);
        (assign32390_e42863, (0.5 * (locals.var_psip_dn0 + (((locals.var_psip_dn0 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn0)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn2 + (((locals.var_psip_dn2 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn2)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn3)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn4)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn5)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn6)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn7)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn8)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn9)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn10)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn11)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn12 + (((locals.var_psip_dn12 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn12)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn13 + (((locals.var_psip_dn13 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn13)) / (2.0 * assign32390_e42861)))), (0.5 * (locals.var_psip_dn14 + (((locals.var_psip_dn14 * assign32390_e42853) + (assign32390_e42850 * locals.var_psip_dn14)) / (2.0 * assign32390_e42861)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign32390_e42865;
        locals.var_t8_dn0 = assign32390_e42865_d_n0;
        locals.var_t8_dn2 = assign32390_e42865_d_n2;
        locals.var_t8_dn3 = assign32390_e42865_d_n3;
        locals.var_t8_dn4 = assign32390_e42865_d_n4;
        locals.var_t8_dn5 = assign32390_e42865_d_n5;
        locals.var_t8_dn6 = assign32390_e42865_d_n6;
        locals.var_t8_dn7 = assign32390_e42865_d_n7;
        locals.var_t8_dn8 = assign32390_e42865_d_n8;
        locals.var_t8_dn9 = assign32390_e42865_d_n9;
        locals.var_t8_dn10 = assign32390_e42865_d_n10;
        locals.var_t8_dn11 = assign32390_e42865_d_n11;
        locals.var_t8_dn12 = assign32390_e42865_d_n12;
        locals.var_t8_dn13 = assign32390_e42865_d_n13;
        locals.var_t8_dn14 = assign32390_e42865_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign32400_e42870, assign32400_e42870_d_n0, assign32400_e42870_d_n2, assign32400_e42870_d_n3, assign32400_e42870_d_n4, assign32400_e42870_d_n5, assign32400_e42870_d_n6, assign32400_e42870_d_n7, assign32400_e42870_d_n8, assign32400_e42870_d_n9, assign32400_e42870_d_n10, assign32400_e42870_d_n11, assign32400_e42870_d_n12, assign32400_e42870_d_n13, assign32400_e42870_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32400_e42868: f64 = (locals.var_t8).sqrt();
        (assign32400_e42868, (locals.var_t8_dn0 / (2.0 * assign32400_e42868)), (locals.var_t8_dn2 / (2.0 * assign32400_e42868)), (locals.var_t8_dn3 / (2.0 * assign32400_e42868)), (locals.var_t8_dn4 / (2.0 * assign32400_e42868)), (locals.var_t8_dn5 / (2.0 * assign32400_e42868)), (locals.var_t8_dn6 / (2.0 * assign32400_e42868)), (locals.var_t8_dn7 / (2.0 * assign32400_e42868)), (locals.var_t8_dn8 / (2.0 * assign32400_e42868)), (locals.var_t8_dn9 / (2.0 * assign32400_e42868)), (locals.var_t8_dn10 / (2.0 * assign32400_e42868)), (locals.var_t8_dn11 / (2.0 * assign32400_e42868)), (locals.var_t8_dn12 / (2.0 * assign32400_e42868)), (locals.var_t8_dn13 / (2.0 * assign32400_e42868)), (locals.var_t8_dn14 / (2.0 * assign32400_e42868)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn0, locals.var_sqrtpsip_dn2, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11, locals.var_sqrtpsip_dn12, locals.var_sqrtpsip_dn13, locals.var_sqrtpsip_dn14,)
    }
};
        locals.var_sqrtpsip = assign32400_e42870;
        locals.var_sqrtpsip_dn0 = assign32400_e42870_d_n0;
        locals.var_sqrtpsip_dn2 = assign32400_e42870_d_n2;
        locals.var_sqrtpsip_dn3 = assign32400_e42870_d_n3;
        locals.var_sqrtpsip_dn4 = assign32400_e42870_d_n4;
        locals.var_sqrtpsip_dn5 = assign32400_e42870_d_n5;
        locals.var_sqrtpsip_dn6 = assign32400_e42870_d_n6;
        locals.var_sqrtpsip_dn7 = assign32400_e42870_d_n7;
        locals.var_sqrtpsip_dn8 = assign32400_e42870_d_n8;
        locals.var_sqrtpsip_dn9 = assign32400_e42870_d_n9;
        locals.var_sqrtpsip_dn10 = assign32400_e42870_d_n10;
        locals.var_sqrtpsip_dn11 = assign32400_e42870_d_n11;
        locals.var_sqrtpsip_dn12 = assign32400_e42870_d_n12;
        locals.var_sqrtpsip_dn13 = assign32400_e42870_d_n13;
        locals.var_sqrtpsip_dn14 = assign32400_e42870_d_n14;
        locals.var_sqrtpsip_rv = 0.0;

        let (assign32410_e42882, assign32410_e42882_d_n0, assign32410_e42882_d_n2, assign32410_e42882_d_n3, assign32410_e42882_d_n4, assign32410_e42882_d_n5, assign32410_e42882_d_n6, assign32410_e42882_d_n7, assign32410_e42882_d_n8, assign32410_e42882_d_n9, assign32410_e42882_d_n10, assign32410_e42882_d_n11, assign32410_e42882_d_n12, assign32410_e42882_d_n13, assign32410_e42882_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32410_e42876: f64 = (2.0 * locals.var_sqrtpsip);
        let assign32410_e42877: f64 = (locals.var_gam_edge / assign32410_e42876);
        let assign32410_e42878: f64 = (1.0 + assign32410_e42877);
        let assign32410_e42880: f64 = (assign32410_e42878 / locals.var_gam_edge);
        (assign32410_e42880, ((((((locals.var_gam_edge_dn0 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn0))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn0)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn2 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn2))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn2)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn3 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn3))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn3)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn4 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn4))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn4)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn5 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn5))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn5)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn6 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn6))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn6)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn7 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn7))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn7)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn8 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn8))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn8)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn9 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn9))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn9)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn10 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn10))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn10)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn11 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn11))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn11)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn12 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn12))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn12)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn13 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn13))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn13)) / (locals.var_gam_edge * locals.var_gam_edge)), ((((((locals.var_gam_edge_dn14 * assign32410_e42876) - (locals.var_gam_edge * (2.0 * locals.var_sqrtpsip_dn14))) / (assign32410_e42876 * assign32410_e42876)) * locals.var_gam_edge) - (assign32410_e42878 * locals.var_gam_edge_dn14)) / (locals.var_gam_edge * locals.var_gam_edge)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign32410_e42882;
        locals.var_t0_dn0 = assign32410_e42882_d_n0;
        locals.var_t0_dn2 = assign32410_e42882_d_n2;
        locals.var_t0_dn3 = assign32410_e42882_d_n3;
        locals.var_t0_dn4 = assign32410_e42882_d_n4;
        locals.var_t0_dn5 = assign32410_e42882_d_n5;
        locals.var_t0_dn6 = assign32410_e42882_d_n6;
        locals.var_t0_dn7 = assign32410_e42882_d_n7;
        locals.var_t0_dn8 = assign32410_e42882_d_n8;
        locals.var_t0_dn9 = assign32410_e42882_d_n9;
        locals.var_t0_dn10 = assign32410_e42882_d_n10;
        locals.var_t0_dn11 = assign32410_e42882_d_n11;
        locals.var_t0_dn12 = assign32410_e42882_d_n12;
        locals.var_t0_dn13 = assign32410_e42882_d_n13;
        locals.var_t0_dn14 = assign32410_e42882_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign32420_e42892, assign32420_e42892_d_n0, assign32420_e42892_d_n2, assign32420_e42892_d_n3, assign32420_e42892_d_n4, assign32420_e42892_d_n5, assign32420_e42892_d_n6, assign32420_e42892_d_n7, assign32420_e42892_d_n8, assign32420_e42892_d_n9, assign32420_e42892_d_n10, assign32420_e42892_d_n11, assign32420_e42892_d_n12, assign32420_e42892_d_n13, assign32420_e42892_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32420_e42887: f64 = (2.0 * locals.var_phib_n_edge);
        let assign32420_e42888: f64 = (locals.var_psip - assign32420_e42887);
        let assign32420_e42890: f64 = (assign32420_e42888 - locals.var_vdeff);
        (assign32420_e42890, ((locals.var_psip_dn0 - (2.0 * locals.var_phib_n_edge_dn0)) - locals.var_vdeff_dn0), ((locals.var_psip_dn2 - (2.0 * locals.var_phib_n_edge_dn2)) - locals.var_vdeff_dn2), ((locals.var_psip_dn3 - (2.0 * locals.var_phib_n_edge_dn3)) - locals.var_vdeff_dn3), ((locals.var_psip_dn4 - (2.0 * locals.var_phib_n_edge_dn4)) - locals.var_vdeff_dn4), ((locals.var_psip_dn5 - (2.0 * locals.var_phib_n_edge_dn5)) - locals.var_vdeff_dn5), ((locals.var_psip_dn6 - (2.0 * locals.var_phib_n_edge_dn6)) - locals.var_vdeff_dn6), ((locals.var_psip_dn7 - (2.0 * locals.var_phib_n_edge_dn7)) - locals.var_vdeff_dn7), ((locals.var_psip_dn8 - (2.0 * locals.var_phib_n_edge_dn8)) - locals.var_vdeff_dn8), ((locals.var_psip_dn9 - (2.0 * locals.var_phib_n_edge_dn9)) - locals.var_vdeff_dn9), ((locals.var_psip_dn10 - (2.0 * locals.var_phib_n_edge_dn10)) - locals.var_vdeff_dn10), ((locals.var_psip_dn11 - (2.0 * locals.var_phib_n_edge_dn11)) - locals.var_vdeff_dn11), ((locals.var_psip_dn12 - (2.0 * locals.var_phib_n_edge_dn12)) - locals.var_vdeff_dn12), ((locals.var_psip_dn13 - (2.0 * locals.var_phib_n_edge_dn13)) - locals.var_vdeff_dn13), ((locals.var_psip_dn14 - (2.0 * locals.var_phib_n_edge_dn14)) - locals.var_vdeff_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32420_e42892;
        locals.var_t1_dn0 = assign32420_e42892_d_n0;
        locals.var_t1_dn2 = assign32420_e42892_d_n2;
        locals.var_t1_dn3 = assign32420_e42892_d_n3;
        locals.var_t1_dn4 = assign32420_e42892_d_n4;
        locals.var_t1_dn5 = assign32420_e42892_d_n5;
        locals.var_t1_dn6 = assign32420_e42892_d_n6;
        locals.var_t1_dn7 = assign32420_e42892_d_n7;
        locals.var_t1_dn8 = assign32420_e42892_d_n8;
        locals.var_t1_dn9 = assign32420_e42892_d_n9;
        locals.var_t1_dn10 = assign32420_e42892_d_n10;
        locals.var_t1_dn11 = assign32420_e42892_d_n11;
        locals.var_t1_dn12 = assign32420_e42892_d_n12;
        locals.var_t1_dn13 = assign32420_e42892_d_n13;
        locals.var_t1_dn14 = assign32420_e42892_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign32430_e42907, assign32430_e42907_d_n0, assign32430_e42907_d_n2, assign32430_e42907_d_n3, assign32430_e42907_d_n4, assign32430_e42907_d_n5, assign32430_e42907_d_n6, assign32430_e42907_d_n7, assign32430_e42907_d_n8, assign32430_e42907_d_n9, assign32430_e42907_d_n10, assign32430_e42907_d_n11, assign32430_e42907_d_n12, assign32430_e42907_d_n13, assign32430_e42907_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32430_e42896: f64 = locals.var_t1;
        let assign32430_e42899: f64 = (4.0 * locals.var_t0);
        let assign32430_e42901: f64 = (assign32430_e42899 * locals.var_sqrtpsip);
        let assign32430_e42903: f64 = (assign32430_e42901).max(1e-38);
        let assign32430_e42904: f64 = (assign32430_e42903).ln();
        let assign32430_e42905: f64 = (assign32430_e42896 - assign32430_e42904);
        (assign32430_e42905, (locals.var_t1_dn0 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn0) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn0)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn2 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn2) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn2)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn3 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn3) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn3)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn4 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn4) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn4)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn5 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn5) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn5)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn6 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn6) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn6)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn7 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn7) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn7)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn8 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn8) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn8)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn9 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn9) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn9)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn10 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn10) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn10)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn11 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn11) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn11)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn12 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn12) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn12)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn13 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn13) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn13)) } else { 0.0 } / assign32430_e42903)), (locals.var_t1_dn14 - (if assign32430_e42901 >= 1e-38 { (((4.0 * locals.var_t0_dn14) * locals.var_sqrtpsip) + (assign32430_e42899 * locals.var_sqrtpsip_dn14)) } else { 0.0 } / assign32430_e42903)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32430_e42907;
        locals.var_t2_dn0 = assign32430_e42907_d_n0;
        locals.var_t2_dn2 = assign32430_e42907_d_n2;
        locals.var_t2_dn3 = assign32430_e42907_d_n3;
        locals.var_t2_dn4 = assign32430_e42907_d_n4;
        locals.var_t2_dn5 = assign32430_e42907_d_n5;
        locals.var_t2_dn6 = assign32430_e42907_d_n6;
        locals.var_t2_dn7 = assign32430_e42907_d_n7;
        locals.var_t2_dn8 = assign32430_e42907_d_n8;
        locals.var_t2_dn9 = assign32430_e42907_d_n9;
        locals.var_t2_dn10 = assign32430_e42907_d_n10;
        locals.var_t2_dn11 = assign32430_e42907_d_n11;
        locals.var_t2_dn12 = assign32430_e42907_d_n12;
        locals.var_t2_dn13 = assign32430_e42907_d_n13;
        locals.var_t2_dn14 = assign32430_e42907_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign32440_e42924, assign32440_e42924_d_n0, assign32440_e42924_d_n2, assign32440_e42924_d_n3, assign32440_e42924_d_n4, assign32440_e42924_d_n5, assign32440_e42924_d_n6, assign32440_e42924_d_n7, assign32440_e42924_d_n8, assign32440_e42924_d_n9, assign32440_e42924_d_n10, assign32440_e42924_d_n11, assign32440_e42924_d_n12, assign32440_e42924_d_n13, assign32440_e42924_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32440_e42912: f64 = (locals.var_t2 - 0.201491);
        let assign32440_e42916: f64 = (locals.var_t2 + 0.402982);
        let assign32440_e42917: f64 = (locals.var_t2 * assign32440_e42916);
        let assign32440_e42919: f64 = (assign32440_e42917 + 2.446562);
        let assign32440_e42920: f64 = (assign32440_e42919).sqrt();
        let assign32440_e42921: f64 = (assign32440_e42912 - assign32440_e42920);
        let assign32440_e42922: f64 = (0.5 * assign32440_e42921);
        (assign32440_e42922, (0.5 * (locals.var_t2_dn0 - (((locals.var_t2_dn0 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn2 - (((locals.var_t2_dn2 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn3 - (((locals.var_t2_dn3 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn4 - (((locals.var_t2_dn4 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn5 - (((locals.var_t2_dn5 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn6 - (((locals.var_t2_dn6 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn7 - (((locals.var_t2_dn7 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn8 - (((locals.var_t2_dn8 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn9 - (((locals.var_t2_dn9 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn10 - (((locals.var_t2_dn10 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn11 - (((locals.var_t2_dn11 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn12 - (((locals.var_t2_dn12 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn12)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn13 - (((locals.var_t2_dn13 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign32440_e42920)))), (0.5 * (locals.var_t2_dn14 - (((locals.var_t2_dn14 * assign32440_e42916) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign32440_e42920)))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign32440_e42924;
        locals.var_t8_dn0 = assign32440_e42924_d_n0;
        locals.var_t8_dn2 = assign32440_e42924_d_n2;
        locals.var_t8_dn3 = assign32440_e42924_d_n3;
        locals.var_t8_dn4 = assign32440_e42924_d_n4;
        locals.var_t8_dn5 = assign32440_e42924_d_n5;
        locals.var_t8_dn6 = assign32440_e42924_d_n6;
        locals.var_t8_dn7 = assign32440_e42924_d_n7;
        locals.var_t8_dn8 = assign32440_e42924_d_n8;
        locals.var_t8_dn9 = assign32440_e42924_d_n9;
        locals.var_t8_dn10 = assign32440_e42924_d_n10;
        locals.var_t8_dn11 = assign32440_e42924_d_n11;
        locals.var_t8_dn12 = assign32440_e42924_d_n12;
        locals.var_t8_dn13 = assign32440_e42924_d_n13;
        locals.var_t8_dn14 = assign32440_e42924_d_n14;
        locals.var_t8_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_105(
        locals: &mut StampLocals,
    ) {
        let (assign32450_e42928, assign32450_e42928_d_n0, assign32450_e42928_d_n2, assign32450_e42928_d_n3, assign32450_e42928_d_n4, assign32450_e42928_d_n5, assign32450_e42928_d_n6, assign32450_e42928_d_n7, assign32450_e42928_d_n8, assign32450_e42928_d_n9, assign32450_e42928_d_n10, assign32450_e42928_d_n11, assign32450_e42928_d_n12, assign32450_e42928_d_n13, assign32450_e42928_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn0, locals.var_sqrtpsip_dn2, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11, locals.var_sqrtpsip_dn12, locals.var_sqrtpsip_dn13, locals.var_sqrtpsip_dn14,)
    } else {
        (locals.var_sqrtpsisa, locals.var_sqrtpsisa_dn0, locals.var_sqrtpsisa_dn2, locals.var_sqrtpsisa_dn3, locals.var_sqrtpsisa_dn4, locals.var_sqrtpsisa_dn5, locals.var_sqrtpsisa_dn6, locals.var_sqrtpsisa_dn7, locals.var_sqrtpsisa_dn8, locals.var_sqrtpsisa_dn9, locals.var_sqrtpsisa_dn10, locals.var_sqrtpsisa_dn11, locals.var_sqrtpsisa_dn12, locals.var_sqrtpsisa_dn13, locals.var_sqrtpsisa_dn14,)
    }
};
        locals.var_sqrtpsisa = assign32450_e42928;
        locals.var_sqrtpsisa_dn0 = assign32450_e42928_d_n0;
        locals.var_sqrtpsisa_dn2 = assign32450_e42928_d_n2;
        locals.var_sqrtpsisa_dn3 = assign32450_e42928_d_n3;
        locals.var_sqrtpsisa_dn4 = assign32450_e42928_d_n4;
        locals.var_sqrtpsisa_dn5 = assign32450_e42928_d_n5;
        locals.var_sqrtpsisa_dn6 = assign32450_e42928_d_n6;
        locals.var_sqrtpsisa_dn7 = assign32450_e42928_d_n7;
        locals.var_sqrtpsisa_dn8 = assign32450_e42928_d_n8;
        locals.var_sqrtpsisa_dn9 = assign32450_e42928_d_n9;
        locals.var_sqrtpsisa_dn10 = assign32450_e42928_d_n10;
        locals.var_sqrtpsisa_dn11 = assign32450_e42928_d_n11;
        locals.var_sqrtpsisa_dn12 = assign32450_e42928_d_n12;
        locals.var_sqrtpsisa_dn13 = assign32450_e42928_d_n13;
        locals.var_sqrtpsisa_dn14 = assign32450_e42928_d_n14;
        locals.var_sqrtpsisa_rv = 0.0;

        let assign32460_e42931: f64 = (-68.0);
        let assign32460_e42932: f64 = if locals.var_t8 <= assign32460_e42931 { 1.0 } else { 0.0 };
        locals.var_guard740 = assign32460_e42932;
        locals.var_guard740_rv = 0.0;

        let (assign32470_e42939, assign32470_e42939_d_n0, assign32470_e42939_d_n2, assign32470_e42939_d_n3, assign32470_e42939_d_n4, assign32470_e42939_d_n5, assign32470_e42939_d_n6, assign32470_e42939_d_n7, assign32470_e42939_d_n8, assign32470_e42939_d_n9, assign32470_e42939_d_n10, assign32470_e42939_d_n11, assign32470_e42939_d_n12, assign32470_e42939_d_n13, assign32470_e42939_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) {
        let assign32470_e42937: f64 = (-100.0);
        (assign32470_e42937, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32470_e42939;
        locals.var_t4_dn0 = assign32470_e42939_d_n0;
        locals.var_t4_dn2 = assign32470_e42939_d_n2;
        locals.var_t4_dn3 = assign32470_e42939_d_n3;
        locals.var_t4_dn4 = assign32470_e42939_d_n4;
        locals.var_t4_dn5 = assign32470_e42939_d_n5;
        locals.var_t4_dn6 = assign32470_e42939_d_n6;
        locals.var_t4_dn7 = assign32470_e42939_d_n7;
        locals.var_t4_dn8 = assign32470_e42939_d_n8;
        locals.var_t4_dn9 = assign32470_e42939_d_n9;
        locals.var_t4_dn10 = assign32470_e42939_d_n10;
        locals.var_t4_dn11 = assign32470_e42939_d_n11;
        locals.var_t4_dn12 = assign32470_e42939_d_n12;
        locals.var_t4_dn13 = assign32470_e42939_d_n13;
        locals.var_t4_dn14 = assign32470_e42939_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign32480_e42945, assign32480_e42945_d_n0, assign32480_e42945_d_n2, assign32480_e42945_d_n3, assign32480_e42945_d_n4, assign32480_e42945_d_n5, assign32480_e42945_d_n6, assign32480_e42945_d_n7, assign32480_e42945_d_n8, assign32480_e42945_d_n9, assign32480_e42945_d_n10, assign32480_e42945_d_n11, assign32480_e42945_d_n12, assign32480_e42945_d_n13, assign32480_e42945_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) {
        (20.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32480_e42945;
        locals.var_t5_dn0 = assign32480_e42945_d_n0;
        locals.var_t5_dn2 = assign32480_e42945_d_n2;
        locals.var_t5_dn3 = assign32480_e42945_d_n3;
        locals.var_t5_dn4 = assign32480_e42945_d_n4;
        locals.var_t5_dn5 = assign32480_e42945_d_n5;
        locals.var_t5_dn6 = assign32480_e42945_d_n6;
        locals.var_t5_dn7 = assign32480_e42945_d_n7;
        locals.var_t5_dn8 = assign32480_e42945_d_n8;
        locals.var_t5_dn9 = assign32480_e42945_d_n9;
        locals.var_t5_dn10 = assign32480_e42945_d_n10;
        locals.var_t5_dn11 = assign32480_e42945_d_n11;
        locals.var_t5_dn12 = assign32480_e42945_d_n12;
        locals.var_t5_dn13 = assign32480_e42945_d_n13;
        locals.var_t5_dn14 = assign32480_e42945_d_n14;
        locals.var_t5_rv = 0.0;

        let assign32490_e42950: f64 = (0.5 * locals.var_t5);
        let assign32490_e42951: f64 = (locals.var_t4 - assign32490_e42950);
        let assign32490_e42952: f64 = if locals.var_t8 < assign32490_e42951 { 1.0 } else { 0.0 };
        locals.var_guard741 = assign32490_e42952;
        locals.var_guard741_rv = 0.0;

        let (assign32500_e42961, assign32500_e42961_d_n0, assign32500_e42961_d_n2, assign32500_e42961_d_n3, assign32500_e42961_d_n4, assign32500_e42961_d_n5, assign32500_e42961_d_n6, assign32500_e42961_d_n7, assign32500_e42961_d_n8, assign32500_e42961_d_n9, assign32500_e42961_d_n10, assign32500_e42961_d_n11, assign32500_e42961_d_n12, assign32500_e42961_d_n13, assign32500_e42961_d_n14,) = {
    if (((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 != 0.0)) {
        let assign32500_e42959: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32500_e42959, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn0), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn2), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn12), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn13), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32500_e42961;
        locals.var_t3_dn0 = assign32500_e42961_d_n0;
        locals.var_t3_dn2 = assign32500_e42961_d_n2;
        locals.var_t3_dn3 = assign32500_e42961_d_n3;
        locals.var_t3_dn4 = assign32500_e42961_d_n4;
        locals.var_t3_dn5 = assign32500_e42961_d_n5;
        locals.var_t3_dn6 = assign32500_e42961_d_n6;
        locals.var_t3_dn7 = assign32500_e42961_d_n7;
        locals.var_t3_dn8 = assign32500_e42961_d_n8;
        locals.var_t3_dn9 = assign32500_e42961_d_n9;
        locals.var_t3_dn10 = assign32500_e42961_d_n10;
        locals.var_t3_dn11 = assign32500_e42961_d_n11;
        locals.var_t3_dn12 = assign32500_e42961_d_n12;
        locals.var_t3_dn13 = assign32500_e42961_d_n13;
        locals.var_t3_dn14 = assign32500_e42961_d_n14;
        locals.var_t3_rv = 0.0;

        let assign32510_e42966: f64 = (0.5 * locals.var_t5);
        let assign32510_e42967: f64 = (locals.var_t4 + assign32510_e42966);
        let assign32510_e42968: f64 = if locals.var_t8 > assign32510_e42967 { 1.0 } else { 0.0 };
        locals.var_guard742 = assign32510_e42968;
        locals.var_guard742_rv = 0.0;

        let (assign32520_e42980, assign32520_e42980_d_n0, assign32520_e42980_d_n2, assign32520_e42980_d_n3, assign32520_e42980_d_n4, assign32520_e42980_d_n5, assign32520_e42980_d_n6, assign32520_e42980_d_n7, assign32520_e42980_d_n8, assign32520_e42980_d_n9, assign32520_e42980_d_n10, assign32520_e42980_d_n11, assign32520_e42980_d_n12, assign32520_e42980_d_n13, assign32520_e42980_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 == 0.0)) && (locals.var_guard742 != 0.0)) {
        let assign32520_e42978: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32520_e42978, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn0), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn2), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn12), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn13), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32520_e42980;
        locals.var_t3_dn0 = assign32520_e42980_d_n0;
        locals.var_t3_dn2 = assign32520_e42980_d_n2;
        locals.var_t3_dn3 = assign32520_e42980_d_n3;
        locals.var_t3_dn4 = assign32520_e42980_d_n4;
        locals.var_t3_dn5 = assign32520_e42980_d_n5;
        locals.var_t3_dn6 = assign32520_e42980_d_n6;
        locals.var_t3_dn7 = assign32520_e42980_d_n7;
        locals.var_t3_dn8 = assign32520_e42980_d_n8;
        locals.var_t3_dn9 = assign32520_e42980_d_n9;
        locals.var_t3_dn10 = assign32520_e42980_d_n10;
        locals.var_t3_dn11 = assign32520_e42980_d_n11;
        locals.var_t3_dn12 = assign32520_e42980_d_n12;
        locals.var_t3_dn13 = assign32520_e42980_d_n13;
        locals.var_t3_dn14 = assign32520_e42980_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32530_e42996, assign32530_e42996_d_n0, assign32530_e42996_d_n2, assign32530_e42996_d_n3, assign32530_e42996_d_n4, assign32530_e42996_d_n5, assign32530_e42996_d_n6, assign32530_e42996_d_n7, assign32530_e42996_d_n8, assign32530_e42996_d_n9, assign32530_e42996_d_n10, assign32530_e42996_d_n11, assign32530_e42996_d_n12, assign32530_e42996_d_n13, assign32530_e42996_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 == 0.0)) && (locals.var_guard742 == 0.0)) {
        let assign32530_e42992: f64 = (locals.var_t8 - locals.var_t4);
        let assign32530_e42994: f64 = (assign32530_e42992 / locals.var_t5);
        (assign32530_e42994, ((((locals.var_t8_dn0 - locals.var_t4_dn0) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn2 - locals.var_t4_dn2) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn3 - locals.var_t4_dn3) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn4 - locals.var_t4_dn4) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn5 - locals.var_t4_dn5) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn6 - locals.var_t4_dn6) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn7 - locals.var_t4_dn7) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn8 - locals.var_t4_dn8) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn9 - locals.var_t4_dn9) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn10 - locals.var_t4_dn10) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn11 - locals.var_t4_dn11) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn12 - locals.var_t4_dn12) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn13 - locals.var_t4_dn13) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5)), ((((locals.var_t8_dn14 - locals.var_t4_dn14) * locals.var_t5) - (assign32530_e42992 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32530_e42996;
        locals.var_t2_dn0 = assign32530_e42996_d_n0;
        locals.var_t2_dn2 = assign32530_e42996_d_n2;
        locals.var_t2_dn3 = assign32530_e42996_d_n3;
        locals.var_t2_dn4 = assign32530_e42996_d_n4;
        locals.var_t2_dn5 = assign32530_e42996_d_n5;
        locals.var_t2_dn6 = assign32530_e42996_d_n6;
        locals.var_t2_dn7 = assign32530_e42996_d_n7;
        locals.var_t2_dn8 = assign32530_e42996_d_n8;
        locals.var_t2_dn9 = assign32530_e42996_d_n9;
        locals.var_t2_dn10 = assign32530_e42996_d_n10;
        locals.var_t2_dn11 = assign32530_e42996_d_n11;
        locals.var_t2_dn12 = assign32530_e42996_d_n12;
        locals.var_t2_dn13 = assign32530_e42996_d_n13;
        locals.var_t2_dn14 = assign32530_e42996_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign32540_e43010, assign32540_e43010_d_n0, assign32540_e43010_d_n2, assign32540_e43010_d_n3, assign32540_e43010_d_n4, assign32540_e43010_d_n5, assign32540_e43010_d_n6, assign32540_e43010_d_n7, assign32540_e43010_d_n8, assign32540_e43010_d_n9, assign32540_e43010_d_n10, assign32540_e43010_d_n11, assign32540_e43010_d_n12, assign32540_e43010_d_n13, assign32540_e43010_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 == 0.0)) && (locals.var_guard742 == 0.0)) {
        let assign32540_e43008: f64 = (locals.var_t2 * locals.var_t2);
        (assign32540_e43008, ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)), ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)), ((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)), ((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)), ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign32540_e43010;
        locals.var_t6_dn0 = assign32540_e43010_d_n0;
        locals.var_t6_dn2 = assign32540_e43010_d_n2;
        locals.var_t6_dn3 = assign32540_e43010_d_n3;
        locals.var_t6_dn4 = assign32540_e43010_d_n4;
        locals.var_t6_dn5 = assign32540_e43010_d_n5;
        locals.var_t6_dn6 = assign32540_e43010_d_n6;
        locals.var_t6_dn7 = assign32540_e43010_d_n7;
        locals.var_t6_dn8 = assign32540_e43010_d_n8;
        locals.var_t6_dn9 = assign32540_e43010_d_n9;
        locals.var_t6_dn10 = assign32540_e43010_d_n10;
        locals.var_t6_dn11 = assign32540_e43010_d_n11;
        locals.var_t6_dn12 = assign32540_e43010_d_n12;
        locals.var_t6_dn13 = assign32540_e43010_d_n13;
        locals.var_t6_dn14 = assign32540_e43010_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign32550_e43045, assign32550_e43045_d_n0, assign32550_e43045_d_n2, assign32550_e43045_d_n3, assign32550_e43045_d_n4, assign32550_e43045_d_n5, assign32550_e43045_d_n6, assign32550_e43045_d_n7, assign32550_e43045_d_n8, assign32550_e43045_d_n9, assign32550_e43045_d_n10, assign32550_e43045_d_n11, assign32550_e43045_d_n12, assign32550_e43045_d_n13, assign32550_e43045_d_n14,) = {
    if ((((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) && (locals.var_guard741 == 0.0)) && (locals.var_guard742 == 0.0)) {
        let assign32550_e43024: f64 = (5.0 / 64.0);
        let assign32550_e43027: f64 = (0.5 * locals.var_t2);
        let assign32550_e43028: f64 = (assign32550_e43024 + assign32550_e43027);
        let assign32550_e43032: f64 = (15.0 / 16.0);
        let assign32550_e43036: f64 = (1.25 - locals.var_t6);
        let assign32550_e43037: f64 = (locals.var_t6 * assign32550_e43036);
        let assign32550_e43038: f64 = (assign32550_e43032 - assign32550_e43037);
        let assign32550_e43039: f64 = (locals.var_t6 * assign32550_e43038);
        let assign32550_e43040: f64 = (assign32550_e43028 + assign32550_e43039);
        let assign32550_e43041: f64 = (locals.var_t5 * assign32550_e43040);
        let assign32550_e43042: f64 = (locals.var_t4 + assign32550_e43041);
        let assign32550_e43043: f64 = { let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32550_e43043, ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn0 + ((locals.var_t5_dn0 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn0) + ((locals.var_t6_dn0 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn0 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn0))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn2 + ((locals.var_t5_dn2 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn2) + ((locals.var_t6_dn2 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn2 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn2))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn3 + ((locals.var_t5_dn3 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn3) + ((locals.var_t6_dn3 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn3 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn3))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn4) + ((locals.var_t6_dn4 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn4 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn4))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn5) + ((locals.var_t6_dn5 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn5 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn5))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn6) + ((locals.var_t6_dn6 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn6 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn6))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn7) + ((locals.var_t6_dn7 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn7 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn7))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn8) + ((locals.var_t6_dn8 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn8 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn8))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn9) + ((locals.var_t6_dn9 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn9 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn9))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn10) + ((locals.var_t6_dn10 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn10 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn10))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn11) + ((locals.var_t6_dn11 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn11 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn11))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn12 + ((locals.var_t5_dn12 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn12) + ((locals.var_t6_dn12 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn12 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn12))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn13 + ((locals.var_t5_dn13 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn13) + ((locals.var_t6_dn13 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn13 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn13))))))))))), ({ let limited_exp_arg = assign32550_e43042; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t4_dn14 + ((locals.var_t5_dn14 * assign32550_e43040) + (locals.var_t5 * ((0.5 * locals.var_t2_dn14) + ((locals.var_t6_dn14 * assign32550_e43038) + (locals.var_t6 * (-((locals.var_t6_dn14 * assign32550_e43036) + (locals.var_t6 * (-locals.var_t6_dn14))))))))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32550_e43045;
        locals.var_t3_dn0 = assign32550_e43045_d_n0;
        locals.var_t3_dn2 = assign32550_e43045_d_n2;
        locals.var_t3_dn3 = assign32550_e43045_d_n3;
        locals.var_t3_dn4 = assign32550_e43045_d_n4;
        locals.var_t3_dn5 = assign32550_e43045_d_n5;
        locals.var_t3_dn6 = assign32550_e43045_d_n6;
        locals.var_t3_dn7 = assign32550_e43045_d_n7;
        locals.var_t3_dn8 = assign32550_e43045_d_n8;
        locals.var_t3_dn9 = assign32550_e43045_d_n9;
        locals.var_t3_dn10 = assign32550_e43045_d_n10;
        locals.var_t3_dn11 = assign32550_e43045_d_n11;
        locals.var_t3_dn12 = assign32550_e43045_d_n12;
        locals.var_t3_dn13 = assign32550_e43045_d_n13;
        locals.var_t3_dn14 = assign32550_e43045_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32560_e43078, assign32560_e43078_d_n0, assign32560_e43078_d_n2, assign32560_e43078_d_n3, assign32560_e43078_d_n4, assign32560_e43078_d_n5, assign32560_e43078_d_n6, assign32560_e43078_d_n7, assign32560_e43078_d_n8, assign32560_e43078_d_n9, assign32560_e43078_d_n10, assign32560_e43078_d_n11, assign32560_e43078_d_n12, assign32560_e43078_d_n13, assign32560_e43078_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 != 0.0)) {
        let assign32560_e43052: f64 = (1.0 + locals.var_t1);
        let assign32560_e43055: f64 = locals.var_t8;
        let assign32560_e43056: f64 = (assign32560_e43052 - assign32560_e43055);
        let assign32560_e43060: f64 = (2.0 * locals.var_t0);
        let assign32560_e43063: f64 = (locals.var_t3 * 2.0);
        let assign32560_e43065: f64 = (assign32560_e43063 * locals.var_t0);
        let assign32560_e43068: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign32560_e43069: f64 = (assign32560_e43065 + assign32560_e43068);
        let assign32560_e43070: f64 = (assign32560_e43060 * assign32560_e43069);
        let assign32560_e43072: f64 = (assign32560_e43070).max(1e-38);
        let assign32560_e43073: f64 = (assign32560_e43072).ln();
        let assign32560_e43074: f64 = assign32560_e43073;
        let assign32560_e43075: f64 = (assign32560_e43056 - assign32560_e43074);
        let assign32560_e43076: f64 = (locals.var_t3 * assign32560_e43075);
        (assign32560_e43076, ((locals.var_t3_dn0 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn0 - locals.var_t8_dn0) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn0) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn0)) + (2.0 * locals.var_sqrtpsisa_dn0)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn2 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn2 - locals.var_t8_dn2) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn2) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn2)) + (2.0 * locals.var_sqrtpsisa_dn2)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn3 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn3 - locals.var_t8_dn3) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn3) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn4 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn4 - locals.var_t8_dn4) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn4) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn5 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn5 - locals.var_t8_dn5) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn5) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn6 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn6 - locals.var_t8_dn6) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn6) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn7 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn7 - locals.var_t8_dn7) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn7) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn8 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn8 - locals.var_t8_dn8) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn8) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn9 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn9 - locals.var_t8_dn9) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn9) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn10 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn10 - locals.var_t8_dn10) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn10) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn11 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn11 - locals.var_t8_dn11) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn11) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn12 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn12 - locals.var_t8_dn12) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn12) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn12)) + (2.0 * locals.var_sqrtpsisa_dn12)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn13 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn13 - locals.var_t8_dn13) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn13) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn13)) + (2.0 * locals.var_sqrtpsisa_dn13)))) } else { 0.0 } / assign32560_e43072)))), ((locals.var_t3_dn14 * assign32560_e43075) + (locals.var_t3 * ((locals.var_t1_dn14 - locals.var_t8_dn14) - (if assign32560_e43070 >= 1e-38 { (((2.0 * locals.var_t0_dn14) * assign32560_e43069) + (assign32560_e43060 * ((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32560_e43063 * locals.var_t0_dn14)) + (2.0 * locals.var_sqrtpsisa_dn14)))) } else { 0.0 } / assign32560_e43072)))),)
    } else {
        (locals.var_qdeff_edge, locals.var_qdeff_edge_dn0, locals.var_qdeff_edge_dn2, locals.var_qdeff_edge_dn3, locals.var_qdeff_edge_dn4, locals.var_qdeff_edge_dn5, locals.var_qdeff_edge_dn6, locals.var_qdeff_edge_dn7, locals.var_qdeff_edge_dn8, locals.var_qdeff_edge_dn9, locals.var_qdeff_edge_dn10, locals.var_qdeff_edge_dn11, locals.var_qdeff_edge_dn12, locals.var_qdeff_edge_dn13, locals.var_qdeff_edge_dn14,)
    }
};
        locals.var_qdeff_edge = assign32560_e43078;
        locals.var_qdeff_edge_dn0 = assign32560_e43078_d_n0;
        locals.var_qdeff_edge_dn2 = assign32560_e43078_d_n2;
        locals.var_qdeff_edge_dn3 = assign32560_e43078_d_n3;
        locals.var_qdeff_edge_dn4 = assign32560_e43078_d_n4;
        locals.var_qdeff_edge_dn5 = assign32560_e43078_d_n5;
        locals.var_qdeff_edge_dn6 = assign32560_e43078_d_n6;
        locals.var_qdeff_edge_dn7 = assign32560_e43078_d_n7;
        locals.var_qdeff_edge_dn8 = assign32560_e43078_d_n8;
        locals.var_qdeff_edge_dn9 = assign32560_e43078_d_n9;
        locals.var_qdeff_edge_dn10 = assign32560_e43078_d_n10;
        locals.var_qdeff_edge_dn11 = assign32560_e43078_d_n11;
        locals.var_qdeff_edge_dn12 = assign32560_e43078_d_n12;
        locals.var_qdeff_edge_dn13 = assign32560_e43078_d_n13;
        locals.var_qdeff_edge_dn14 = assign32560_e43078_d_n14;
        locals.var_qdeff_edge_rv = 0.0;

        let (assign32570_e43086, assign32570_e43086_d_n0, assign32570_e43086_d_n2, assign32570_e43086_d_n3, assign32570_e43086_d_n4, assign32570_e43086_d_n5, assign32570_e43086_d_n6, assign32570_e43086_d_n7, assign32570_e43086_d_n8, assign32570_e43086_d_n9, assign32570_e43086_d_n10, assign32570_e43086_d_n11, assign32570_e43086_d_n12, assign32570_e43086_d_n13, assign32570_e43086_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32570_e43084: f64 = { let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign32570_e43084, ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn0), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn2), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn3), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn4), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn5), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn6), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn7), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn8), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn9), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn10), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn11), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn12), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn13), ({ let limited_exp_arg = locals.var_t8; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t8_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32570_e43086;
        locals.var_t3_dn0 = assign32570_e43086_d_n0;
        locals.var_t3_dn2 = assign32570_e43086_d_n2;
        locals.var_t3_dn3 = assign32570_e43086_d_n3;
        locals.var_t3_dn4 = assign32570_e43086_d_n4;
        locals.var_t3_dn5 = assign32570_e43086_d_n5;
        locals.var_t3_dn6 = assign32570_e43086_d_n6;
        locals.var_t3_dn7 = assign32570_e43086_d_n7;
        locals.var_t3_dn8 = assign32570_e43086_d_n8;
        locals.var_t3_dn9 = assign32570_e43086_d_n9;
        locals.var_t3_dn10 = assign32570_e43086_d_n10;
        locals.var_t3_dn11 = assign32570_e43086_d_n11;
        locals.var_t3_dn12 = assign32570_e43086_d_n12;
        locals.var_t3_dn13 = assign32570_e43086_d_n13;
        locals.var_t3_dn14 = assign32570_e43086_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32580_e43095, assign32580_e43095_d_n0, assign32580_e43095_d_n2, assign32580_e43095_d_n3, assign32580_e43095_d_n4, assign32580_e43095_d_n5, assign32580_e43095_d_n6, assign32580_e43095_d_n7, assign32580_e43095_d_n8, assign32580_e43095_d_n9, assign32580_e43095_d_n10, assign32580_e43095_d_n11, assign32580_e43095_d_n12, assign32580_e43095_d_n13, assign32580_e43095_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32580_e43093: f64 = (1.0 / locals.var_sqrtpsisa);
        (assign32580_e43093, (-(locals.var_sqrtpsisa_dn0 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn2 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn3 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn4 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn5 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn6 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn7 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn8 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn9 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn10 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn11 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn12 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn13 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))), (-(locals.var_sqrtpsisa_dn14 / (locals.var_sqrtpsisa * locals.var_sqrtpsisa))),)
    } else {
        (locals.var_sqrtpsisainv, locals.var_sqrtpsisainv_dn0, locals.var_sqrtpsisainv_dn2, locals.var_sqrtpsisainv_dn3, locals.var_sqrtpsisainv_dn4, locals.var_sqrtpsisainv_dn5, locals.var_sqrtpsisainv_dn6, locals.var_sqrtpsisainv_dn7, locals.var_sqrtpsisainv_dn8, locals.var_sqrtpsisainv_dn9, locals.var_sqrtpsisainv_dn10, locals.var_sqrtpsisainv_dn11, locals.var_sqrtpsisainv_dn12, locals.var_sqrtpsisainv_dn13, locals.var_sqrtpsisainv_dn14,)
    }
};
        locals.var_sqrtpsisainv = assign32580_e43095;
        locals.var_sqrtpsisainv_dn0 = assign32580_e43095_d_n0;
        locals.var_sqrtpsisainv_dn2 = assign32580_e43095_d_n2;
        locals.var_sqrtpsisainv_dn3 = assign32580_e43095_d_n3;
        locals.var_sqrtpsisainv_dn4 = assign32580_e43095_d_n4;
        locals.var_sqrtpsisainv_dn5 = assign32580_e43095_d_n5;
        locals.var_sqrtpsisainv_dn6 = assign32580_e43095_d_n6;
        locals.var_sqrtpsisainv_dn7 = assign32580_e43095_d_n7;
        locals.var_sqrtpsisainv_dn8 = assign32580_e43095_d_n8;
        locals.var_sqrtpsisainv_dn9 = assign32580_e43095_d_n9;
        locals.var_sqrtpsisainv_dn10 = assign32580_e43095_d_n10;
        locals.var_sqrtpsisainv_dn11 = assign32580_e43095_d_n11;
        locals.var_sqrtpsisainv_dn12 = assign32580_e43095_d_n12;
        locals.var_sqrtpsisainv_dn13 = assign32580_e43095_d_n13;
        locals.var_sqrtpsisainv_dn14 = assign32580_e43095_d_n14;
        locals.var_sqrtpsisainv_rv = 0.0;

        let (assign32590_e43127, assign32590_e43127_d_n0, assign32590_e43127_d_n2, assign32590_e43127_d_n3, assign32590_e43127_d_n4, assign32590_e43127_d_n5, assign32590_e43127_d_n6, assign32590_e43127_d_n7, assign32590_e43127_d_n8, assign32590_e43127_d_n9, assign32590_e43127_d_n10, assign32590_e43127_d_n11, assign32590_e43127_d_n12, assign32590_e43127_d_n13, assign32590_e43127_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32590_e43102: f64 = (2.0 * locals.var_t3);
        let assign32590_e43106: f64 = (locals.var_t3 * 2.0);
        let assign32590_e43108: f64 = (assign32590_e43106 * locals.var_t0);
        let assign32590_e43111: f64 = (locals.var_t3 * 2.0);
        let assign32590_e43113: f64 = (assign32590_e43111 * locals.var_t0);
        let assign32590_e43116: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign32590_e43117: f64 = (assign32590_e43113 + assign32590_e43116);
        let assign32590_e43118: f64 = (assign32590_e43108 * assign32590_e43117);
        let assign32590_e43120: f64 = (assign32590_e43118).max(1e-38);
        let assign32590_e43121: f64 = (assign32590_e43120).ln();
        let assign32590_e43122: f64 = assign32590_e43121;
        let assign32590_e43123: f64 = (assign32590_e43102 + assign32590_e43122);
        let assign32590_e43125: f64 = (assign32590_e43123 - locals.var_t1);
        (assign32590_e43125, (((2.0 * locals.var_t3_dn0) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn0)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn0)) + (2.0 * locals.var_sqrtpsisa_dn0)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn0), (((2.0 * locals.var_t3_dn2) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn2)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn2)) + (2.0 * locals.var_sqrtpsisa_dn2)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn2), (((2.0 * locals.var_t3_dn3) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn3)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn4)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn5)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn6)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn7)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn8)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn9)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn10)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn11)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn11), (((2.0 * locals.var_t3_dn12) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn12)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn12)) + (2.0 * locals.var_sqrtpsisa_dn12)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn12), (((2.0 * locals.var_t3_dn13) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn13)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn13)) + (2.0 * locals.var_sqrtpsisa_dn13)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn13), (((2.0 * locals.var_t3_dn14) + (if assign32590_e43118 >= 1e-38 { (((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32590_e43106 * locals.var_t0_dn14)) * assign32590_e43117) + (assign32590_e43108 * ((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32590_e43111 * locals.var_t0_dn14)) + (2.0 * locals.var_sqrtpsisa_dn14)))) } else { 0.0 } / assign32590_e43120)) - locals.var_t1_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32590_e43127;
        locals.var_t4_dn0 = assign32590_e43127_d_n0;
        locals.var_t4_dn2 = assign32590_e43127_d_n2;
        locals.var_t4_dn3 = assign32590_e43127_d_n3;
        locals.var_t4_dn4 = assign32590_e43127_d_n4;
        locals.var_t4_dn5 = assign32590_e43127_d_n5;
        locals.var_t4_dn6 = assign32590_e43127_d_n6;
        locals.var_t4_dn7 = assign32590_e43127_d_n7;
        locals.var_t4_dn8 = assign32590_e43127_d_n8;
        locals.var_t4_dn9 = assign32590_e43127_d_n9;
        locals.var_t4_dn10 = assign32590_e43127_d_n10;
        locals.var_t4_dn11 = assign32590_e43127_d_n11;
        locals.var_t4_dn12 = assign32590_e43127_d_n12;
        locals.var_t4_dn13 = assign32590_e43127_d_n13;
        locals.var_t4_dn14 = assign32590_e43127_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign32600_e43152, assign32600_e43152_d_n0, assign32600_e43152_d_n2, assign32600_e43152_d_n3, assign32600_e43152_d_n4, assign32600_e43152_d_n5, assign32600_e43152_d_n6, assign32600_e43152_d_n7, assign32600_e43152_d_n8, assign32600_e43152_d_n9, assign32600_e43152_d_n10, assign32600_e43152_d_n11, assign32600_e43152_d_n12, assign32600_e43152_d_n13, assign32600_e43152_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32600_e43135: f64 = 1.0;
        let assign32600_e43137: f64 = (assign32600_e43135 / locals.var_t3);
        let assign32600_e43138: f64 = (2.0 + assign32600_e43137);
        let assign32600_e43142: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32600_e43143: f64 = assign32600_e43142;
        let assign32600_e43146: f64 = (locals.var_t0 * locals.var_t3);
        let assign32600_e43148: f64 = (assign32600_e43146 + locals.var_sqrtpsisa);
        let assign32600_e43149: f64 = (assign32600_e43143 / assign32600_e43148);
        let assign32600_e43150: f64 = (assign32600_e43138 + assign32600_e43149);
        (assign32600_e43150, ((-((assign32600_e43135 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn3) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn12) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32600_e43148 * assign32600_e43148))), ((-((assign32600_e43135 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32600_e43148) - (assign32600_e43143 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32600_e43148 * assign32600_e43148))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32600_e43152;
        locals.var_t5_dn0 = assign32600_e43152_d_n0;
        locals.var_t5_dn2 = assign32600_e43152_d_n2;
        locals.var_t5_dn3 = assign32600_e43152_d_n3;
        locals.var_t5_dn4 = assign32600_e43152_d_n4;
        locals.var_t5_dn5 = assign32600_e43152_d_n5;
        locals.var_t5_dn6 = assign32600_e43152_d_n6;
        locals.var_t5_dn7 = assign32600_e43152_d_n7;
        locals.var_t5_dn8 = assign32600_e43152_d_n8;
        locals.var_t5_dn9 = assign32600_e43152_d_n9;
        locals.var_t5_dn10 = assign32600_e43152_d_n10;
        locals.var_t5_dn11 = assign32600_e43152_d_n11;
        locals.var_t5_dn12 = assign32600_e43152_d_n12;
        locals.var_t5_dn13 = assign32600_e43152_d_n13;
        locals.var_t5_dn14 = assign32600_e43152_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign32610_e43163, assign32610_e43163_d_n0, assign32610_e43163_d_n2, assign32610_e43163_d_n3, assign32610_e43163_d_n4, assign32610_e43163_d_n5, assign32610_e43163_d_n6, assign32610_e43163_d_n7, assign32610_e43163_d_n8, assign32610_e43163_d_n9, assign32610_e43163_d_n10, assign32610_e43163_d_n11, assign32610_e43163_d_n12, assign32610_e43163_d_n13, assign32610_e43163_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32610_e43160: f64 = (locals.var_t4 / locals.var_t5);
        let assign32610_e43161: f64 = (locals.var_t3 - assign32610_e43160);
        (assign32610_e43161, (locals.var_t3_dn0 - (((locals.var_t4_dn0 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn2 - (((locals.var_t4_dn2 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn3 - (((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn4 - (((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn5 - (((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn6 - (((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn7 - (((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn8 - (((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn9 - (((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn10 - (((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn11 - (((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn12 - (((locals.var_t4_dn12 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn13 - (((locals.var_t4_dn13 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5))), (locals.var_t3_dn14 - (((locals.var_t4_dn14 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32610_e43163;
        locals.var_t3_dn0 = assign32610_e43163_d_n0;
        locals.var_t3_dn2 = assign32610_e43163_d_n2;
        locals.var_t3_dn3 = assign32610_e43163_d_n3;
        locals.var_t3_dn4 = assign32610_e43163_d_n4;
        locals.var_t3_dn5 = assign32610_e43163_d_n5;
        locals.var_t3_dn6 = assign32610_e43163_d_n6;
        locals.var_t3_dn7 = assign32610_e43163_d_n7;
        locals.var_t3_dn8 = assign32610_e43163_d_n8;
        locals.var_t3_dn9 = assign32610_e43163_d_n9;
        locals.var_t3_dn10 = assign32610_e43163_d_n10;
        locals.var_t3_dn11 = assign32610_e43163_d_n11;
        locals.var_t3_dn12 = assign32610_e43163_d_n12;
        locals.var_t3_dn13 = assign32610_e43163_d_n13;
        locals.var_t3_dn14 = assign32610_e43163_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32620_e43195, assign32620_e43195_d_n0, assign32620_e43195_d_n2, assign32620_e43195_d_n3, assign32620_e43195_d_n4, assign32620_e43195_d_n5, assign32620_e43195_d_n6, assign32620_e43195_d_n7, assign32620_e43195_d_n8, assign32620_e43195_d_n9, assign32620_e43195_d_n10, assign32620_e43195_d_n11, assign32620_e43195_d_n12, assign32620_e43195_d_n13, assign32620_e43195_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32620_e43170: f64 = (2.0 * locals.var_t3);
        let assign32620_e43174: f64 = (locals.var_t3 * 2.0);
        let assign32620_e43176: f64 = (assign32620_e43174 * locals.var_t0);
        let assign32620_e43179: f64 = (locals.var_t3 * 2.0);
        let assign32620_e43181: f64 = (assign32620_e43179 * locals.var_t0);
        let assign32620_e43184: f64 = (2.0 * locals.var_sqrtpsisa);
        let assign32620_e43185: f64 = (assign32620_e43181 + assign32620_e43184);
        let assign32620_e43186: f64 = (assign32620_e43176 * assign32620_e43185);
        let assign32620_e43188: f64 = (assign32620_e43186).max(1e-38);
        let assign32620_e43189: f64 = (assign32620_e43188).ln();
        let assign32620_e43190: f64 = assign32620_e43189;
        let assign32620_e43191: f64 = (assign32620_e43170 + assign32620_e43190);
        let assign32620_e43193: f64 = (assign32620_e43191 - locals.var_t1);
        (assign32620_e43193, (((2.0 * locals.var_t3_dn0) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn0)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn0 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn0)) + (2.0 * locals.var_sqrtpsisa_dn0)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn0), (((2.0 * locals.var_t3_dn2) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn2)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn2 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn2)) + (2.0 * locals.var_sqrtpsisa_dn2)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn2), (((2.0 * locals.var_t3_dn3) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn3)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn3 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn3)) + (2.0 * locals.var_sqrtpsisa_dn3)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn3), (((2.0 * locals.var_t3_dn4) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn4)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn4 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn4)) + (2.0 * locals.var_sqrtpsisa_dn4)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn4), (((2.0 * locals.var_t3_dn5) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn5)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn5 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn5)) + (2.0 * locals.var_sqrtpsisa_dn5)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn5), (((2.0 * locals.var_t3_dn6) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn6)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn6 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn6)) + (2.0 * locals.var_sqrtpsisa_dn6)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn6), (((2.0 * locals.var_t3_dn7) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn7)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn7 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn7)) + (2.0 * locals.var_sqrtpsisa_dn7)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn7), (((2.0 * locals.var_t3_dn8) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn8)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn8 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn8)) + (2.0 * locals.var_sqrtpsisa_dn8)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn8), (((2.0 * locals.var_t3_dn9) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn9)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn9 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn9)) + (2.0 * locals.var_sqrtpsisa_dn9)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn9), (((2.0 * locals.var_t3_dn10) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn10)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn10 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn10)) + (2.0 * locals.var_sqrtpsisa_dn10)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn10), (((2.0 * locals.var_t3_dn11) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn11)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn11 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn11)) + (2.0 * locals.var_sqrtpsisa_dn11)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn11), (((2.0 * locals.var_t3_dn12) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn12)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn12 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn12)) + (2.0 * locals.var_sqrtpsisa_dn12)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn12), (((2.0 * locals.var_t3_dn13) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn13)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn13 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn13)) + (2.0 * locals.var_sqrtpsisa_dn13)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn13), (((2.0 * locals.var_t3_dn14) + (if assign32620_e43186 >= 1e-38 { (((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32620_e43174 * locals.var_t0_dn14)) * assign32620_e43185) + (assign32620_e43176 * ((((locals.var_t3_dn14 * 2.0) * locals.var_t0) + (assign32620_e43179 * locals.var_t0_dn14)) + (2.0 * locals.var_sqrtpsisa_dn14)))) } else { 0.0 } / assign32620_e43188)) - locals.var_t1_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32620_e43195;
        locals.var_t4_dn0 = assign32620_e43195_d_n0;
        locals.var_t4_dn2 = assign32620_e43195_d_n2;
        locals.var_t4_dn3 = assign32620_e43195_d_n3;
        locals.var_t4_dn4 = assign32620_e43195_d_n4;
        locals.var_t4_dn5 = assign32620_e43195_d_n5;
        locals.var_t4_dn6 = assign32620_e43195_d_n6;
        locals.var_t4_dn7 = assign32620_e43195_d_n7;
        locals.var_t4_dn8 = assign32620_e43195_d_n8;
        locals.var_t4_dn9 = assign32620_e43195_d_n9;
        locals.var_t4_dn10 = assign32620_e43195_d_n10;
        locals.var_t4_dn11 = assign32620_e43195_d_n11;
        locals.var_t4_dn12 = assign32620_e43195_d_n12;
        locals.var_t4_dn13 = assign32620_e43195_d_n13;
        locals.var_t4_dn14 = assign32620_e43195_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign32630_e43220, assign32630_e43220_d_n0, assign32630_e43220_d_n2, assign32630_e43220_d_n3, assign32630_e43220_d_n4, assign32630_e43220_d_n5, assign32630_e43220_d_n6, assign32630_e43220_d_n7, assign32630_e43220_d_n8, assign32630_e43220_d_n9, assign32630_e43220_d_n10, assign32630_e43220_d_n11, assign32630_e43220_d_n12, assign32630_e43220_d_n13, assign32630_e43220_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32630_e43203: f64 = 1.0;
        let assign32630_e43205: f64 = (assign32630_e43203 / locals.var_t3);
        let assign32630_e43206: f64 = (2.0 + assign32630_e43205);
        let assign32630_e43210: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32630_e43211: f64 = assign32630_e43210;
        let assign32630_e43214: f64 = (locals.var_t0 * locals.var_t3);
        let assign32630_e43216: f64 = (assign32630_e43214 + locals.var_sqrtpsisa);
        let assign32630_e43217: f64 = (assign32630_e43211 / assign32630_e43216);
        let assign32630_e43218: f64 = (assign32630_e43206 + assign32630_e43217);
        (assign32630_e43218, ((-((assign32630_e43203 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn3) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn12) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32630_e43216 * assign32630_e43216))), ((-((assign32630_e43203 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))) + ((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32630_e43216) - (assign32630_e43211 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32630_e43216 * assign32630_e43216))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32630_e43220;
        locals.var_t5_dn0 = assign32630_e43220_d_n0;
        locals.var_t5_dn2 = assign32630_e43220_d_n2;
        locals.var_t5_dn3 = assign32630_e43220_d_n3;
        locals.var_t5_dn4 = assign32630_e43220_d_n4;
        locals.var_t5_dn5 = assign32630_e43220_d_n5;
        locals.var_t5_dn6 = assign32630_e43220_d_n6;
        locals.var_t5_dn7 = assign32630_e43220_d_n7;
        locals.var_t5_dn8 = assign32630_e43220_d_n8;
        locals.var_t5_dn9 = assign32630_e43220_d_n9;
        locals.var_t5_dn10 = assign32630_e43220_d_n10;
        locals.var_t5_dn11 = assign32630_e43220_d_n11;
        locals.var_t5_dn12 = assign32630_e43220_d_n12;
        locals.var_t5_dn13 = assign32630_e43220_d_n13;
        locals.var_t5_dn14 = assign32630_e43220_d_n14;
        locals.var_t5_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_106(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32640_e43247, assign32640_e43247_d_n0, assign32640_e43247_d_n2, assign32640_e43247_d_n3, assign32640_e43247_d_n4, assign32640_e43247_d_n5, assign32640_e43247_d_n6, assign32640_e43247_d_n7, assign32640_e43247_d_n8, assign32640_e43247_d_n9, assign32640_e43247_d_n10, assign32640_e43247_d_n11, assign32640_e43247_d_n12, assign32640_e43247_d_n13, assign32640_e43247_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32640_e43228: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32640_e43231: f64 = (locals.var_t0 * locals.var_t3);
        let assign32640_e43233: f64 = (assign32640_e43231 + locals.var_sqrtpsisa);
        let assign32640_e43234: f64 = (assign32640_e43228 / assign32640_e43233);
        let assign32640_e43235: f64 = assign32640_e43234;
        let assign32640_e43238: f64 = (locals.var_t0 + locals.var_sqrtpsisainv);
        let assign32640_e43241: f64 = (locals.var_t0 * locals.var_t3);
        let assign32640_e43243: f64 = (assign32640_e43241 + locals.var_sqrtpsisa);
        let assign32640_e43244: f64 = (assign32640_e43238 / assign32640_e43243);
        let assign32640_e43245: f64 = (assign32640_e43235 * assign32640_e43244);
        (assign32640_e43245, ((((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn0 + locals.var_sqrtpsisainv_dn0) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn2 + locals.var_sqrtpsisainv_dn2) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn3 + locals.var_sqrtpsisainv_dn3) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn4 + locals.var_sqrtpsisainv_dn4) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn5 + locals.var_sqrtpsisainv_dn5) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn6 + locals.var_sqrtpsisainv_dn6) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn7 + locals.var_sqrtpsisainv_dn7) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn8 + locals.var_sqrtpsisainv_dn8) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn9 + locals.var_sqrtpsisainv_dn9) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn10 + locals.var_sqrtpsisainv_dn10) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn11 + locals.var_sqrtpsisainv_dn11) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn12 + locals.var_sqrtpsisainv_dn12) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn13 + locals.var_sqrtpsisainv_dn13) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13))) / (assign32640_e43243 * assign32640_e43243)))), ((((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32640_e43233) - (assign32640_e43228 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32640_e43233 * assign32640_e43233)) * assign32640_e43244) + (assign32640_e43235 * ((((locals.var_t0_dn14 + locals.var_sqrtpsisainv_dn14) * assign32640_e43243) - (assign32640_e43238 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14))) / (assign32640_e43243 * assign32640_e43243)))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign32640_e43247;
        locals.var_t6_dn0 = assign32640_e43247_d_n0;
        locals.var_t6_dn2 = assign32640_e43247_d_n2;
        locals.var_t6_dn3 = assign32640_e43247_d_n3;
        locals.var_t6_dn4 = assign32640_e43247_d_n4;
        locals.var_t6_dn5 = assign32640_e43247_d_n5;
        locals.var_t6_dn6 = assign32640_e43247_d_n6;
        locals.var_t6_dn7 = assign32640_e43247_d_n7;
        locals.var_t6_dn8 = assign32640_e43247_d_n8;
        locals.var_t6_dn9 = assign32640_e43247_d_n9;
        locals.var_t6_dn10 = assign32640_e43247_d_n10;
        locals.var_t6_dn11 = assign32640_e43247_d_n11;
        locals.var_t6_dn12 = assign32640_e43247_d_n12;
        locals.var_t6_dn13 = assign32640_e43247_d_n13;
        locals.var_t6_dn14 = assign32640_e43247_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign32650_e43281, assign32650_e43281_d_n0, assign32650_e43281_d_n2, assign32650_e43281_d_n3, assign32650_e43281_d_n4, assign32650_e43281_d_n5, assign32650_e43281_d_n6, assign32650_e43281_d_n7, assign32650_e43281_d_n8, assign32650_e43281_d_n9, assign32650_e43281_d_n10, assign32650_e43281_d_n11, assign32650_e43281_d_n12, assign32650_e43281_d_n13, assign32650_e43281_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32650_e43253: f64 = (-1.0);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t3;
        let assign32650_e43256: f64 = (1.0 * __rspice_inv_cse_0);
        let assign32650_e43259: f64 = (1.0 * __rspice_inv_cse_0);
        let assign32650_e43260: f64 = (assign32650_e43256 * assign32650_e43259);
        let assign32650_e43261: f64 = (assign32650_e43253 * assign32650_e43260);
        let assign32650_e43264: f64 = 1.0;
        let assign32650_e43267: f64 = (locals.var_sqrtpsisa * locals.var_sqrtpsisa);
        let assign32650_e43269: f64 = (assign32650_e43267 * locals.var_sqrtpsisa);
        let assign32650_e43272: f64 = (locals.var_t0 * locals.var_t3);
        let assign32650_e43274: f64 = (assign32650_e43272 + locals.var_sqrtpsisa);
        let assign32650_e43275: f64 = (assign32650_e43269 * assign32650_e43274);
        let assign32650_e43276: f64 = (assign32650_e43264 / assign32650_e43275);
        let assign32650_e43277: f64 = (assign32650_e43261 - assign32650_e43276);
        let assign32650_e43279: f64 = (assign32650_e43277 - locals.var_t6);
        (assign32650_e43279, (((assign32650_e43253 * (((-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn0 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn0)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn0)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn0 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn0)) + locals.var_sqrtpsisa_dn0)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn0), (((assign32650_e43253 * (((-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn2 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn2)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn2)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn2 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn2)) + locals.var_sqrtpsisa_dn2)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn2), (((assign32650_e43253 * (((-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn3 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn3 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn3)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn3)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn3 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn3)) + locals.var_sqrtpsisa_dn3)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn3), (((assign32650_e43253 * (((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn4 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn4)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn4)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn4 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn4)) + locals.var_sqrtpsisa_dn4)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn4), (((assign32650_e43253 * (((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn5 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn5)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn5)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn5 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn5)) + locals.var_sqrtpsisa_dn5)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn5), (((assign32650_e43253 * (((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn6 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn6)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn6)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn6 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn6)) + locals.var_sqrtpsisa_dn6)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn6), (((assign32650_e43253 * (((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn7 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn7)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn7)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn7 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn7)) + locals.var_sqrtpsisa_dn7)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn7), (((assign32650_e43253 * (((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn8 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn8)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn8)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn8 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn8)) + locals.var_sqrtpsisa_dn8)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn8), (((assign32650_e43253 * (((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn9 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn9)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn9)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn9 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn9)) + locals.var_sqrtpsisa_dn9)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn9), (((assign32650_e43253 * (((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn10 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn10)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn10)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn10 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn10)) + locals.var_sqrtpsisa_dn10)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn10), (((assign32650_e43253 * (((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn11 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn11)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn11)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn11 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn11)) + locals.var_sqrtpsisa_dn11)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn11), (((assign32650_e43253 * (((-(locals.var_t3_dn12 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn12 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn12 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn12)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn12)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn12 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn12)) + locals.var_sqrtpsisa_dn12)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn12), (((assign32650_e43253 * (((-(locals.var_t3_dn13 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn13 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn13 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn13)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn13)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn13 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn13)) + locals.var_sqrtpsisa_dn13)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn13), (((assign32650_e43253 * (((-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3))) * assign32650_e43259) + (assign32650_e43256 * (-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3)))))) - (-((assign32650_e43264 * ((((((locals.var_sqrtpsisa_dn14 * locals.var_sqrtpsisa) + (locals.var_sqrtpsisa * locals.var_sqrtpsisa_dn14)) * locals.var_sqrtpsisa) + (assign32650_e43267 * locals.var_sqrtpsisa_dn14)) * assign32650_e43274) + (assign32650_e43269 * (((locals.var_t0_dn14 * locals.var_t3) + (locals.var_t0 * locals.var_t3_dn14)) + locals.var_sqrtpsisa_dn14)))) / (assign32650_e43275 * assign32650_e43275)))) - locals.var_t6_dn14),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign32650_e43281;
        locals.var_t7_dn0 = assign32650_e43281_d_n0;
        locals.var_t7_dn2 = assign32650_e43281_d_n2;
        locals.var_t7_dn3 = assign32650_e43281_d_n3;
        locals.var_t7_dn4 = assign32650_e43281_d_n4;
        locals.var_t7_dn5 = assign32650_e43281_d_n5;
        locals.var_t7_dn6 = assign32650_e43281_d_n6;
        locals.var_t7_dn7 = assign32650_e43281_d_n7;
        locals.var_t7_dn8 = assign32650_e43281_d_n8;
        locals.var_t7_dn9 = assign32650_e43281_d_n9;
        locals.var_t7_dn10 = assign32650_e43281_d_n10;
        locals.var_t7_dn11 = assign32650_e43281_d_n11;
        locals.var_t7_dn12 = assign32650_e43281_d_n12;
        locals.var_t7_dn13 = assign32650_e43281_d_n13;
        locals.var_t7_dn14 = assign32650_e43281_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign32660_e43304, assign32660_e43304_d_n0, assign32660_e43304_d_n2, assign32660_e43304_d_n3, assign32660_e43304_d_n4, assign32660_e43304_d_n5, assign32660_e43304_d_n6, assign32660_e43304_d_n7, assign32660_e43304_d_n8, assign32660_e43304_d_n9, assign32660_e43304_d_n10, assign32660_e43304_d_n11, assign32660_e43304_d_n12, assign32660_e43304_d_n13, assign32660_e43304_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard740 == 0.0)) {
        let assign32660_e43289: f64 = (locals.var_t4 / locals.var_t5);
        let assign32660_e43293: f64 = (locals.var_t4 * locals.var_t7);
        let assign32660_e43296: f64 = (2.0 * locals.var_t5);
        let assign32660_e43298: f64 = (assign32660_e43296 * locals.var_t5);
        let assign32660_e43299: f64 = (assign32660_e43293 / assign32660_e43298);
        let assign32660_e43300: f64 = (1.0 + assign32660_e43299);
        let assign32660_e43301: f64 = (assign32660_e43289 * assign32660_e43300);
        let assign32660_e43302: f64 = (locals.var_t3 - assign32660_e43301);
        (assign32660_e43302, (locals.var_t3_dn0 - (((((locals.var_t4_dn0 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn0 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn0)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn0) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn0)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn2 - (((((locals.var_t4_dn2 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn2 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn2)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn2) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn2)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn3 - (((((locals.var_t4_dn3 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn3 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn3)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn3) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn3)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn4 - (((((locals.var_t4_dn4 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn4 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn4)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn4) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn4)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn5 - (((((locals.var_t4_dn5 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn5 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn5)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn5) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn5)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn6 - (((((locals.var_t4_dn6 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn6 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn6)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn6) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn6)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn7 - (((((locals.var_t4_dn7 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn7 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn7)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn7) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn7)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn8 - (((((locals.var_t4_dn8 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn8 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn8)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn8) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn8)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn9 - (((((locals.var_t4_dn9 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn9 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn9)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn9) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn9)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn10 - (((((locals.var_t4_dn10 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn10 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn10)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn10) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn10)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn11 - (((((locals.var_t4_dn11 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn11 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn11)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn11) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn11)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn12 - (((((locals.var_t4_dn12 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn12 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn12)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn12) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn12)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn13 - (((((locals.var_t4_dn13 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn13 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn13)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn13) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn13)))) / (assign32660_e43298 * assign32660_e43298))))), (locals.var_t3_dn14 - (((((locals.var_t4_dn14 * locals.var_t5) - (locals.var_t4 * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)) * assign32660_e43300) + (assign32660_e43289 * (((((locals.var_t4_dn14 * locals.var_t7) + (locals.var_t4 * locals.var_t7_dn14)) * assign32660_e43298) - (assign32660_e43293 * (((2.0 * locals.var_t5_dn14) * locals.var_t5) + (assign32660_e43296 * locals.var_t5_dn14)))) / (assign32660_e43298 * assign32660_e43298))))),)
    } else {
        (locals.var_qdeff_edge, locals.var_qdeff_edge_dn0, locals.var_qdeff_edge_dn2, locals.var_qdeff_edge_dn3, locals.var_qdeff_edge_dn4, locals.var_qdeff_edge_dn5, locals.var_qdeff_edge_dn6, locals.var_qdeff_edge_dn7, locals.var_qdeff_edge_dn8, locals.var_qdeff_edge_dn9, locals.var_qdeff_edge_dn10, locals.var_qdeff_edge_dn11, locals.var_qdeff_edge_dn12, locals.var_qdeff_edge_dn13, locals.var_qdeff_edge_dn14,)
    }
};
        locals.var_qdeff_edge = assign32660_e43304;
        locals.var_qdeff_edge_dn0 = assign32660_e43304_d_n0;
        locals.var_qdeff_edge_dn2 = assign32660_e43304_d_n2;
        locals.var_qdeff_edge_dn3 = assign32660_e43304_d_n3;
        locals.var_qdeff_edge_dn4 = assign32660_e43304_d_n4;
        locals.var_qdeff_edge_dn5 = assign32660_e43304_d_n5;
        locals.var_qdeff_edge_dn6 = assign32660_e43304_d_n6;
        locals.var_qdeff_edge_dn7 = assign32660_e43304_d_n7;
        locals.var_qdeff_edge_dn8 = assign32660_e43304_d_n8;
        locals.var_qdeff_edge_dn9 = assign32660_e43304_d_n9;
        locals.var_qdeff_edge_dn10 = assign32660_e43304_d_n10;
        locals.var_qdeff_edge_dn11 = assign32660_e43304_d_n11;
        locals.var_qdeff_edge_dn12 = assign32660_e43304_d_n12;
        locals.var_qdeff_edge_dn13 = assign32660_e43304_d_n13;
        locals.var_qdeff_edge_dn14 = assign32660_e43304_d_n14;
        locals.var_qdeff_edge_rv = 0.0;

        let assign32670_e43310: f64 = (-2500.0);
        let assign32670_e43312: f64 = (assign32670_e43310 * 2.0);
        let assign32670_e43314: f64 = if ((1.0 == 0.0) && (locals.var_psip < assign32670_e43312)) { 1.0 } else { 0.0 };
        locals.var_guard743 = assign32670_e43314;
        locals.var_guard743_rv = 0.0;

        let (assign32680_e43327, assign32680_e43327_d_n0, assign32680_e43327_d_n2, assign32680_e43327_d_n3, assign32680_e43327_d_n4, assign32680_e43327_d_n5, assign32680_e43327_d_n6, assign32680_e43327_d_n7, assign32680_e43327_d_n8, assign32680_e43327_d_n9, assign32680_e43327_d_n10, assign32680_e43327_d_n11, assign32680_e43327_d_n12, assign32680_e43327_d_n13, assign32680_e43327_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard743 != 0.0)) {
        let assign32680_e43319: f64 = (-2.0);
        let assign32680_e43321: f64 = (assign32680_e43319 * 2.0);
        let assign32680_e43324: f64 = (16.0 * locals.var_psip);
        let assign32680_e43325: f64 = (assign32680_e43321 / assign32680_e43324);
        (assign32680_e43325, (-((assign32680_e43321 * (16.0 * locals.var_psip_dn0)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn2)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn3)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn4)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn5)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn6)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn7)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn8)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn9)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn10)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn11)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn12)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn13)) / (assign32680_e43324 * assign32680_e43324))), (-((assign32680_e43321 * (16.0 * locals.var_psip_dn14)) / (assign32680_e43324 * assign32680_e43324))),)
    } else {
        (locals.var_psipclamp, locals.var_psipclamp_dn0, locals.var_psipclamp_dn2, locals.var_psipclamp_dn3, locals.var_psipclamp_dn4, locals.var_psipclamp_dn5, locals.var_psipclamp_dn6, locals.var_psipclamp_dn7, locals.var_psipclamp_dn8, locals.var_psipclamp_dn9, locals.var_psipclamp_dn10, locals.var_psipclamp_dn11, locals.var_psipclamp_dn12, locals.var_psipclamp_dn13, locals.var_psipclamp_dn14,)
    }
};
        locals.var_psipclamp = assign32680_e43327;
        locals.var_psipclamp_dn0 = assign32680_e43327_d_n0;
        locals.var_psipclamp_dn2 = assign32680_e43327_d_n2;
        locals.var_psipclamp_dn3 = assign32680_e43327_d_n3;
        locals.var_psipclamp_dn4 = assign32680_e43327_d_n4;
        locals.var_psipclamp_dn5 = assign32680_e43327_d_n5;
        locals.var_psipclamp_dn6 = assign32680_e43327_d_n6;
        locals.var_psipclamp_dn7 = assign32680_e43327_d_n7;
        locals.var_psipclamp_dn8 = assign32680_e43327_d_n8;
        locals.var_psipclamp_dn9 = assign32680_e43327_d_n9;
        locals.var_psipclamp_dn10 = assign32680_e43327_d_n10;
        locals.var_psipclamp_dn11 = assign32680_e43327_d_n11;
        locals.var_psipclamp_dn12 = assign32680_e43327_d_n12;
        locals.var_psipclamp_dn13 = assign32680_e43327_d_n13;
        locals.var_psipclamp_dn14 = assign32680_e43327_d_n14;
        locals.var_psipclamp_rv = 0.0;

        let (assign32690_e43353, assign32690_e43353_d_n0, assign32690_e43353_d_n2, assign32690_e43353_d_n3, assign32690_e43353_d_n4, assign32690_e43353_d_n5, assign32690_e43353_d_n6, assign32690_e43353_d_n7, assign32690_e43353_d_n8, assign32690_e43353_d_n9, assign32690_e43353_d_n10, assign32690_e43353_d_n11, assign32690_e43353_d_n12, assign32690_e43353_d_n13, assign32690_e43353_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard743 == 0.0)) {
        let assign32690_e43335: f64 = (locals.var_psip + 1.0);
        let assign32690_e43338: f64 = (locals.var_psip - 1.0);
        let assign32690_e43341: f64 = (locals.var_psip - 1.0);
        let assign32690_e43342: f64 = (assign32690_e43338 * assign32690_e43341);
        let assign32690_e43345: f64 = (0.25 * 2.0);
        let assign32690_e43347: f64 = (assign32690_e43345 * 2.0);
        let assign32690_e43348: f64 = (assign32690_e43342 + assign32690_e43347);
        let assign32690_e43349: f64 = (assign32690_e43348).sqrt();
        let assign32690_e43350: f64 = (assign32690_e43335 + assign32690_e43349);
        let assign32690_e43351: f64 = (0.5 * assign32690_e43350);
        (assign32690_e43351, (0.5 * (locals.var_psip_dn0 + (((locals.var_psip_dn0 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn0)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn2 + (((locals.var_psip_dn2 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn2)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn3 + (((locals.var_psip_dn3 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn3)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn4 + (((locals.var_psip_dn4 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn4)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn5 + (((locals.var_psip_dn5 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn5)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn6 + (((locals.var_psip_dn6 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn6)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn7 + (((locals.var_psip_dn7 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn7)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn8 + (((locals.var_psip_dn8 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn8)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn9 + (((locals.var_psip_dn9 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn9)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn10 + (((locals.var_psip_dn10 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn10)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn11 + (((locals.var_psip_dn11 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn11)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn12 + (((locals.var_psip_dn12 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn12)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn13 + (((locals.var_psip_dn13 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn13)) / (2.0 * assign32690_e43349)))), (0.5 * (locals.var_psip_dn14 + (((locals.var_psip_dn14 * assign32690_e43341) + (assign32690_e43338 * locals.var_psip_dn14)) / (2.0 * assign32690_e43349)))),)
    } else {
        (locals.var_psipclamp, locals.var_psipclamp_dn0, locals.var_psipclamp_dn2, locals.var_psipclamp_dn3, locals.var_psipclamp_dn4, locals.var_psipclamp_dn5, locals.var_psipclamp_dn6, locals.var_psipclamp_dn7, locals.var_psipclamp_dn8, locals.var_psipclamp_dn9, locals.var_psipclamp_dn10, locals.var_psipclamp_dn11, locals.var_psipclamp_dn12, locals.var_psipclamp_dn13, locals.var_psipclamp_dn14,)
    }
};
        locals.var_psipclamp = assign32690_e43353;
        locals.var_psipclamp_dn0 = assign32690_e43353_d_n0;
        locals.var_psipclamp_dn2 = assign32690_e43353_d_n2;
        locals.var_psipclamp_dn3 = assign32690_e43353_d_n3;
        locals.var_psipclamp_dn4 = assign32690_e43353_d_n4;
        locals.var_psipclamp_dn5 = assign32690_e43353_d_n5;
        locals.var_psipclamp_dn6 = assign32690_e43353_d_n6;
        locals.var_psipclamp_dn7 = assign32690_e43353_d_n7;
        locals.var_psipclamp_dn8 = assign32690_e43353_d_n8;
        locals.var_psipclamp_dn9 = assign32690_e43353_d_n9;
        locals.var_psipclamp_dn10 = assign32690_e43353_d_n10;
        locals.var_psipclamp_dn11 = assign32690_e43353_d_n11;
        locals.var_psipclamp_dn12 = assign32690_e43353_d_n12;
        locals.var_psipclamp_dn13 = assign32690_e43353_d_n13;
        locals.var_psipclamp_dn14 = assign32690_e43353_d_n14;
        locals.var_psipclamp_rv = 0.0;

        let (assign32700_e43358, assign32700_e43358_d_n0, assign32700_e43358_d_n2, assign32700_e43358_d_n3, assign32700_e43358_d_n4, assign32700_e43358_d_n5, assign32700_e43358_d_n6, assign32700_e43358_d_n7, assign32700_e43358_d_n8, assign32700_e43358_d_n9, assign32700_e43358_d_n10, assign32700_e43358_d_n11, assign32700_e43358_d_n12, assign32700_e43358_d_n13, assign32700_e43358_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32700_e43356: f64 = (locals.var_psipclamp).sqrt();
        (assign32700_e43356, (locals.var_psipclamp_dn0 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn2 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn3 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn4 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn5 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn6 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn7 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn8 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn9 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn10 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn11 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn12 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn13 / (2.0 * assign32700_e43356)), (locals.var_psipclamp_dn14 / (2.0 * assign32700_e43356)),)
    } else {
        (locals.var_sqrtpsip, locals.var_sqrtpsip_dn0, locals.var_sqrtpsip_dn2, locals.var_sqrtpsip_dn3, locals.var_sqrtpsip_dn4, locals.var_sqrtpsip_dn5, locals.var_sqrtpsip_dn6, locals.var_sqrtpsip_dn7, locals.var_sqrtpsip_dn8, locals.var_sqrtpsip_dn9, locals.var_sqrtpsip_dn10, locals.var_sqrtpsip_dn11, locals.var_sqrtpsip_dn12, locals.var_sqrtpsip_dn13, locals.var_sqrtpsip_dn14,)
    }
};
        locals.var_sqrtpsip = assign32700_e43358;
        locals.var_sqrtpsip_dn0 = assign32700_e43358_d_n0;
        locals.var_sqrtpsip_dn2 = assign32700_e43358_d_n2;
        locals.var_sqrtpsip_dn3 = assign32700_e43358_d_n3;
        locals.var_sqrtpsip_dn4 = assign32700_e43358_d_n4;
        locals.var_sqrtpsip_dn5 = assign32700_e43358_d_n5;
        locals.var_sqrtpsip_dn6 = assign32700_e43358_d_n6;
        locals.var_sqrtpsip_dn7 = assign32700_e43358_d_n7;
        locals.var_sqrtpsip_dn8 = assign32700_e43358_d_n8;
        locals.var_sqrtpsip_dn9 = assign32700_e43358_d_n9;
        locals.var_sqrtpsip_dn10 = assign32700_e43358_d_n10;
        locals.var_sqrtpsip_dn11 = assign32700_e43358_d_n11;
        locals.var_sqrtpsip_dn12 = assign32700_e43358_d_n12;
        locals.var_sqrtpsip_dn13 = assign32700_e43358_d_n13;
        locals.var_sqrtpsip_dn14 = assign32700_e43358_d_n14;
        locals.var_sqrtpsip_rv = 0.0;

        let (assign32710_e43368, assign32710_e43368_d_n0, assign32710_e43368_d_n2, assign32710_e43368_d_n3, assign32710_e43368_d_n4, assign32710_e43368_d_n5, assign32710_e43368_d_n6, assign32710_e43368_d_n7, assign32710_e43368_d_n8, assign32710_e43368_d_n9, assign32710_e43368_d_n10, assign32710_e43368_d_n11, assign32710_e43368_d_n12, assign32710_e43368_d_n13, assign32710_e43368_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32710_e43362: f64 = (locals.var_psip - locals.var_qs_edge);
        let assign32710_e43364: f64 = (assign32710_e43362 - locals.var_qdeff_edge);
        let assign32710_e43366: f64 = (assign32710_e43364 - 1.0);
        (assign32710_e43366, ((locals.var_psip_dn0 - locals.var_qs_edge_dn0) - locals.var_qdeff_edge_dn0), ((locals.var_psip_dn2 - locals.var_qs_edge_dn2) - locals.var_qdeff_edge_dn2), ((locals.var_psip_dn3 - locals.var_qs_edge_dn3) - locals.var_qdeff_edge_dn3), ((locals.var_psip_dn4 - locals.var_qs_edge_dn4) - locals.var_qdeff_edge_dn4), ((locals.var_psip_dn5 - locals.var_qs_edge_dn5) - locals.var_qdeff_edge_dn5), ((locals.var_psip_dn6 - locals.var_qs_edge_dn6) - locals.var_qdeff_edge_dn6), ((locals.var_psip_dn7 - locals.var_qs_edge_dn7) - locals.var_qdeff_edge_dn7), ((locals.var_psip_dn8 - locals.var_qs_edge_dn8) - locals.var_qdeff_edge_dn8), ((locals.var_psip_dn9 - locals.var_qs_edge_dn9) - locals.var_qdeff_edge_dn9), ((locals.var_psip_dn10 - locals.var_qs_edge_dn10) - locals.var_qdeff_edge_dn10), ((locals.var_psip_dn11 - locals.var_qs_edge_dn11) - locals.var_qdeff_edge_dn11), ((locals.var_psip_dn12 - locals.var_qs_edge_dn12) - locals.var_qdeff_edge_dn12), ((locals.var_psip_dn13 - locals.var_qs_edge_dn13) - locals.var_qdeff_edge_dn13), ((locals.var_psip_dn14 - locals.var_qs_edge_dn14) - locals.var_qdeff_edge_dn14),)
    } else {
        (locals.var_psiavg, locals.var_psiavg_dn0, locals.var_psiavg_dn2, locals.var_psiavg_dn3, locals.var_psiavg_dn4, locals.var_psiavg_dn5, locals.var_psiavg_dn6, locals.var_psiavg_dn7, locals.var_psiavg_dn8, locals.var_psiavg_dn9, locals.var_psiavg_dn10, locals.var_psiavg_dn11, locals.var_psiavg_dn12, locals.var_psiavg_dn13, locals.var_psiavg_dn14,)
    }
};
        locals.var_psiavg = assign32710_e43368;
        locals.var_psiavg_dn0 = assign32710_e43368_d_n0;
        locals.var_psiavg_dn2 = assign32710_e43368_d_n2;
        locals.var_psiavg_dn3 = assign32710_e43368_d_n3;
        locals.var_psiavg_dn4 = assign32710_e43368_d_n4;
        locals.var_psiavg_dn5 = assign32710_e43368_d_n5;
        locals.var_psiavg_dn6 = assign32710_e43368_d_n6;
        locals.var_psiavg_dn7 = assign32710_e43368_d_n7;
        locals.var_psiavg_dn8 = assign32710_e43368_d_n8;
        locals.var_psiavg_dn9 = assign32710_e43368_d_n9;
        locals.var_psiavg_dn10 = assign32710_e43368_d_n10;
        locals.var_psiavg_dn11 = assign32710_e43368_d_n11;
        locals.var_psiavg_dn12 = assign32710_e43368_d_n12;
        locals.var_psiavg_dn13 = assign32710_e43368_d_n13;
        locals.var_psiavg_dn14 = assign32710_e43368_d_n14;
        locals.var_psiavg_rv = 0.0;

        let assign32720_e43374: f64 = (-2500.0);
        let assign32720_e43376: f64 = (assign32720_e43374 * 2.0);
        let assign32720_e43378: f64 = if ((1.0 == 0.0) && (locals.var_psiavg < assign32720_e43376)) { 1.0 } else { 0.0 };
        locals.var_guard744 = assign32720_e43378;
        locals.var_guard744_rv = 0.0;

        let (assign32730_e43391, assign32730_e43391_d_n0, assign32730_e43391_d_n2, assign32730_e43391_d_n3, assign32730_e43391_d_n4, assign32730_e43391_d_n5, assign32730_e43391_d_n6, assign32730_e43391_d_n7, assign32730_e43391_d_n8, assign32730_e43391_d_n9, assign32730_e43391_d_n10, assign32730_e43391_d_n11, assign32730_e43391_d_n12, assign32730_e43391_d_n13, assign32730_e43391_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard744 != 0.0)) {
        let assign32730_e43383: f64 = (-2.0);
        let assign32730_e43385: f64 = (assign32730_e43383 * 2.0);
        let assign32730_e43388: f64 = (16.0 * locals.var_psiavg);
        let assign32730_e43389: f64 = (assign32730_e43385 / assign32730_e43388);
        (assign32730_e43389, (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn0)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn2)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn3)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn4)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn5)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn6)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn7)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn8)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn9)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn10)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn11)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn12)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn13)) / (assign32730_e43388 * assign32730_e43388))), (-((assign32730_e43385 * (16.0 * locals.var_psiavg_dn14)) / (assign32730_e43388 * assign32730_e43388))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign32730_e43391;
        locals.var_t0_dn0 = assign32730_e43391_d_n0;
        locals.var_t0_dn2 = assign32730_e43391_d_n2;
        locals.var_t0_dn3 = assign32730_e43391_d_n3;
        locals.var_t0_dn4 = assign32730_e43391_d_n4;
        locals.var_t0_dn5 = assign32730_e43391_d_n5;
        locals.var_t0_dn6 = assign32730_e43391_d_n6;
        locals.var_t0_dn7 = assign32730_e43391_d_n7;
        locals.var_t0_dn8 = assign32730_e43391_d_n8;
        locals.var_t0_dn9 = assign32730_e43391_d_n9;
        locals.var_t0_dn10 = assign32730_e43391_d_n10;
        locals.var_t0_dn11 = assign32730_e43391_d_n11;
        locals.var_t0_dn12 = assign32730_e43391_d_n12;
        locals.var_t0_dn13 = assign32730_e43391_d_n13;
        locals.var_t0_dn14 = assign32730_e43391_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign32740_e43417, assign32740_e43417_d_n0, assign32740_e43417_d_n2, assign32740_e43417_d_n3, assign32740_e43417_d_n4, assign32740_e43417_d_n5, assign32740_e43417_d_n6, assign32740_e43417_d_n7, assign32740_e43417_d_n8, assign32740_e43417_d_n9, assign32740_e43417_d_n10, assign32740_e43417_d_n11, assign32740_e43417_d_n12, assign32740_e43417_d_n13, assign32740_e43417_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard744 == 0.0)) {
        let assign32740_e43399: f64 = (locals.var_psiavg + 1.0);
        let assign32740_e43402: f64 = (locals.var_psiavg - 1.0);
        let assign32740_e43405: f64 = (locals.var_psiavg - 1.0);
        let assign32740_e43406: f64 = (assign32740_e43402 * assign32740_e43405);
        let assign32740_e43409: f64 = (0.25 * 2.0);
        let assign32740_e43411: f64 = (assign32740_e43409 * 2.0);
        let assign32740_e43412: f64 = (assign32740_e43406 + assign32740_e43411);
        let assign32740_e43413: f64 = (assign32740_e43412).sqrt();
        let assign32740_e43414: f64 = (assign32740_e43399 + assign32740_e43413);
        let assign32740_e43415: f64 = (0.5 * assign32740_e43414);
        (assign32740_e43415, (0.5 * (locals.var_psiavg_dn0 + (((locals.var_psiavg_dn0 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn0)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn2 + (((locals.var_psiavg_dn2 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn2)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn3 + (((locals.var_psiavg_dn3 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn3)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn4 + (((locals.var_psiavg_dn4 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn4)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn5 + (((locals.var_psiavg_dn5 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn5)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn6 + (((locals.var_psiavg_dn6 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn6)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn7 + (((locals.var_psiavg_dn7 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn7)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn8 + (((locals.var_psiavg_dn8 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn8)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn9 + (((locals.var_psiavg_dn9 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn9)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn10 + (((locals.var_psiavg_dn10 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn10)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn11 + (((locals.var_psiavg_dn11 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn11)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn12 + (((locals.var_psiavg_dn12 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn12)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn13 + (((locals.var_psiavg_dn13 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn13)) / (2.0 * assign32740_e43413)))), (0.5 * (locals.var_psiavg_dn14 + (((locals.var_psiavg_dn14 * assign32740_e43405) + (assign32740_e43402 * locals.var_psiavg_dn14)) / (2.0 * assign32740_e43413)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign32740_e43417;
        locals.var_t0_dn0 = assign32740_e43417_d_n0;
        locals.var_t0_dn2 = assign32740_e43417_d_n2;
        locals.var_t0_dn3 = assign32740_e43417_d_n3;
        locals.var_t0_dn4 = assign32740_e43417_d_n4;
        locals.var_t0_dn5 = assign32740_e43417_d_n5;
        locals.var_t0_dn6 = assign32740_e43417_d_n6;
        locals.var_t0_dn7 = assign32740_e43417_d_n7;
        locals.var_t0_dn8 = assign32740_e43417_d_n8;
        locals.var_t0_dn9 = assign32740_e43417_d_n9;
        locals.var_t0_dn10 = assign32740_e43417_d_n10;
        locals.var_t0_dn11 = assign32740_e43417_d_n11;
        locals.var_t0_dn12 = assign32740_e43417_d_n12;
        locals.var_t0_dn13 = assign32740_e43417_d_n13;
        locals.var_t0_dn14 = assign32740_e43417_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign32750_e43422, assign32750_e43422_d_n0, assign32750_e43422_d_n2, assign32750_e43422_d_n3, assign32750_e43422_d_n4, assign32750_e43422_d_n5, assign32750_e43422_d_n6, assign32750_e43422_d_n7, assign32750_e43422_d_n8, assign32750_e43422_d_n9, assign32750_e43422_d_n10, assign32750_e43422_d_n11, assign32750_e43422_d_n12, assign32750_e43422_d_n13, assign32750_e43422_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32750_e43420: f64 = (locals.var_t0).sqrt();
        (assign32750_e43420, (locals.var_t0_dn0 / (2.0 * assign32750_e43420)), (locals.var_t0_dn2 / (2.0 * assign32750_e43420)), (locals.var_t0_dn3 / (2.0 * assign32750_e43420)), (locals.var_t0_dn4 / (2.0 * assign32750_e43420)), (locals.var_t0_dn5 / (2.0 * assign32750_e43420)), (locals.var_t0_dn6 / (2.0 * assign32750_e43420)), (locals.var_t0_dn7 / (2.0 * assign32750_e43420)), (locals.var_t0_dn8 / (2.0 * assign32750_e43420)), (locals.var_t0_dn9 / (2.0 * assign32750_e43420)), (locals.var_t0_dn10 / (2.0 * assign32750_e43420)), (locals.var_t0_dn11 / (2.0 * assign32750_e43420)), (locals.var_t0_dn12 / (2.0 * assign32750_e43420)), (locals.var_t0_dn13 / (2.0 * assign32750_e43420)), (locals.var_t0_dn14 / (2.0 * assign32750_e43420)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32750_e43422;
        locals.var_t2_dn0 = assign32750_e43422_d_n0;
        locals.var_t2_dn2 = assign32750_e43422_d_n2;
        locals.var_t2_dn3 = assign32750_e43422_d_n3;
        locals.var_t2_dn4 = assign32750_e43422_d_n4;
        locals.var_t2_dn5 = assign32750_e43422_d_n5;
        locals.var_t2_dn6 = assign32750_e43422_d_n6;
        locals.var_t2_dn7 = assign32750_e43422_d_n7;
        locals.var_t2_dn8 = assign32750_e43422_d_n8;
        locals.var_t2_dn9 = assign32750_e43422_d_n9;
        locals.var_t2_dn10 = assign32750_e43422_d_n10;
        locals.var_t2_dn11 = assign32750_e43422_d_n11;
        locals.var_t2_dn12 = assign32750_e43422_d_n12;
        locals.var_t2_dn13 = assign32750_e43422_d_n13;
        locals.var_t2_dn14 = assign32750_e43422_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign32760_e43432, assign32760_e43432_d_n0, assign32760_e43432_d_n2, assign32760_e43432_d_n3, assign32760_e43432_d_n4, assign32760_e43432_d_n5, assign32760_e43432_d_n6, assign32760_e43432_d_n7, assign32760_e43432_d_n8, assign32760_e43432_d_n9, assign32760_e43432_d_n10, assign32760_e43432_d_n11, assign32760_e43432_d_n12, assign32760_e43432_d_n13, assign32760_e43432_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32760_e43428: f64 = (locals.var_sqrtpsip + locals.var_t2);
        let assign32760_e43429: f64 = (locals.var_gam_edge / assign32760_e43428);
        let assign32760_e43430: f64 = (1.0 + assign32760_e43429);
        (assign32760_e43430, (((locals.var_gam_edge_dn0 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn0 + locals.var_t2_dn0))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn2 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn2 + locals.var_t2_dn2))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn3 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn3 + locals.var_t2_dn3))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn4 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn4 + locals.var_t2_dn4))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn5 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn5 + locals.var_t2_dn5))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn6 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn6 + locals.var_t2_dn6))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn7 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn7 + locals.var_t2_dn7))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn8 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn8 + locals.var_t2_dn8))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn9 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn9 + locals.var_t2_dn9))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn10 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn10 + locals.var_t2_dn10))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn11 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn11 + locals.var_t2_dn11))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn12 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn12 + locals.var_t2_dn12))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn13 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn13 + locals.var_t2_dn13))) / (assign32760_e43428 * assign32760_e43428)), (((locals.var_gam_edge_dn14 * assign32760_e43428) - (locals.var_gam_edge * (locals.var_sqrtpsip_dn14 + locals.var_t2_dn14))) / (assign32760_e43428 * assign32760_e43428)),)
    } else {
        (locals.var_nq_edge, locals.var_nq_edge_dn0, locals.var_nq_edge_dn2, locals.var_nq_edge_dn3, locals.var_nq_edge_dn4, locals.var_nq_edge_dn5, locals.var_nq_edge_dn6, locals.var_nq_edge_dn7, locals.var_nq_edge_dn8, locals.var_nq_edge_dn9, locals.var_nq_edge_dn10, locals.var_nq_edge_dn11, locals.var_nq_edge_dn12, locals.var_nq_edge_dn13, locals.var_nq_edge_dn14,)
    }
};
        locals.var_nq_edge = assign32760_e43432;
        locals.var_nq_edge_dn0 = assign32760_e43432_d_n0;
        locals.var_nq_edge_dn2 = assign32760_e43432_d_n2;
        locals.var_nq_edge_dn3 = assign32760_e43432_d_n3;
        locals.var_nq_edge_dn4 = assign32760_e43432_d_n4;
        locals.var_nq_edge_dn5 = assign32760_e43432_d_n5;
        locals.var_nq_edge_dn6 = assign32760_e43432_d_n6;
        locals.var_nq_edge_dn7 = assign32760_e43432_d_n7;
        locals.var_nq_edge_dn8 = assign32760_e43432_d_n8;
        locals.var_nq_edge_dn9 = assign32760_e43432_d_n9;
        locals.var_nq_edge_dn10 = assign32760_e43432_d_n10;
        locals.var_nq_edge_dn11 = assign32760_e43432_d_n11;
        locals.var_nq_edge_dn12 = assign32760_e43432_d_n12;
        locals.var_nq_edge_dn13 = assign32760_e43432_d_n13;
        locals.var_nq_edge_dn14 = assign32760_e43432_d_n14;
        locals.var_nq_edge_rv = 0.0;

        let (assign32770_e43464, assign32770_e43464_d_n0, assign32770_e43464_d_n2, assign32770_e43464_d_n3, assign32770_e43464_d_n4, assign32770_e43464_d_n5, assign32770_e43464_d_n6, assign32770_e43464_d_n7, assign32770_e43464_d_n8, assign32770_e43464_d_n9, assign32770_e43464_d_n10, assign32770_e43464_d_n11, assign32770_e43464_d_n12, assign32770_e43464_d_n13, assign32770_e43464_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32770_e43436: f64 = (2.0 * p.p2);
        let assign32770_e43438: f64 = (assign32770_e43436 * locals.var_nq_edge);
        let assign32770_e43440: f64 = (assign32770_e43438 * locals.var_ueff);
        let assign32770_e43442: f64 = (assign32770_e43440 * p.p957);
        let assign32770_e43444: f64 = (assign32770_e43442 / locals.var_leff);
        let assign32770_e43446: f64 = (assign32770_e43444 * locals.var_cox);
        let assign32770_e43448: f64 = (assign32770_e43446 * locals.var_nvt);
        let assign32770_e43450: f64 = (assign32770_e43448 * locals.var_nvt);
        let assign32770_e43453: f64 = (locals.var_qs_edge - locals.var_qdeff_edge);
        let assign32770_e43456: f64 = (1.0 + locals.var_qs_edge);
        let assign32770_e43458: f64 = (assign32770_e43456 + locals.var_qdeff_edge);
        let assign32770_e43459: f64 = (assign32770_e43453 * assign32770_e43458);
        let assign32770_e43460: f64 = (assign32770_e43450 * assign32770_e43459);
        let assign32770_e43462: f64 = (assign32770_e43460 * locals.var_moc);
        (assign32770_e43462, ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn0) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn0)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn0)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn0)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn0 - locals.var_qdeff_edge_dn0) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn0 + locals.var_qdeff_edge_dn0))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn0)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn2) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn2)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn2)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn2)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn2 - locals.var_qdeff_edge_dn2) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn2 + locals.var_qdeff_edge_dn2))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn2)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn3) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn3)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn3)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn3)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn3 - locals.var_qdeff_edge_dn3) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn3 + locals.var_qdeff_edge_dn3))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn3)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn4) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn4)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn4)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn4)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn4 - locals.var_qdeff_edge_dn4) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn4 + locals.var_qdeff_edge_dn4))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn4)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn5) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn5)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn5)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn5)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn5 - locals.var_qdeff_edge_dn5) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn5 + locals.var_qdeff_edge_dn5))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn5)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn6) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn6)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn6)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn6)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn6 - locals.var_qdeff_edge_dn6) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn6 + locals.var_qdeff_edge_dn6))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn6)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn7) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn7)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn7)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn7)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn7 - locals.var_qdeff_edge_dn7) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn7 + locals.var_qdeff_edge_dn7))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn7)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn8) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn8)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn8)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn8)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn8 - locals.var_qdeff_edge_dn8) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn8 + locals.var_qdeff_edge_dn8))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn8)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn9) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn9)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn9)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn9)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn9 - locals.var_qdeff_edge_dn9) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn9 + locals.var_qdeff_edge_dn9))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn9)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn10) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn10)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn10)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn10)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn10 - locals.var_qdeff_edge_dn10) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn10 + locals.var_qdeff_edge_dn10))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn10)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn11) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn11)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn11)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn11)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn11 - locals.var_qdeff_edge_dn11) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn11 + locals.var_qdeff_edge_dn11))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn11)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn12) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn12)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn12)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn12)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn12 - locals.var_qdeff_edge_dn12) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn12 + locals.var_qdeff_edge_dn12))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn12)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn13) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn13)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn13)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn13)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn13 - locals.var_qdeff_edge_dn13) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn13 + locals.var_qdeff_edge_dn13))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn13)), ((((((((((((((assign32770_e43436 * locals.var_nq_edge_dn14) * locals.var_ueff) + (assign32770_e43438 * locals.var_ueff_dn14)) * p.p957) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign32770_e43446 * locals.var_nvt_dn14)) * locals.var_nvt) + (assign32770_e43448 * locals.var_nvt_dn14)) * assign32770_e43459) + (assign32770_e43450 * (((locals.var_qs_edge_dn14 - locals.var_qdeff_edge_dn14) * assign32770_e43458) + (assign32770_e43453 * (locals.var_qs_edge_dn14 + locals.var_qdeff_edge_dn14))))) * locals.var_moc) + (assign32770_e43460 * locals.var_moc_dn14)),)
    } else {
        (locals.var_ids_edge, locals.var_ids_edge_dn0, locals.var_ids_edge_dn2, locals.var_ids_edge_dn3, locals.var_ids_edge_dn4, locals.var_ids_edge_dn5, locals.var_ids_edge_dn6, locals.var_ids_edge_dn7, locals.var_ids_edge_dn8, locals.var_ids_edge_dn9, locals.var_ids_edge_dn10, locals.var_ids_edge_dn11, locals.var_ids_edge_dn12, locals.var_ids_edge_dn13, locals.var_ids_edge_dn14,)
    }
};
        locals.var_ids_edge = assign32770_e43464;
        locals.var_ids_edge_dn0 = assign32770_e43464_d_n0;
        locals.var_ids_edge_dn2 = assign32770_e43464_d_n2;
        locals.var_ids_edge_dn3 = assign32770_e43464_d_n3;
        locals.var_ids_edge_dn4 = assign32770_e43464_d_n4;
        locals.var_ids_edge_dn5 = assign32770_e43464_d_n5;
        locals.var_ids_edge_dn6 = assign32770_e43464_d_n6;
        locals.var_ids_edge_dn7 = assign32770_e43464_d_n7;
        locals.var_ids_edge_dn8 = assign32770_e43464_d_n8;
        locals.var_ids_edge_dn9 = assign32770_e43464_d_n9;
        locals.var_ids_edge_dn10 = assign32770_e43464_d_n10;
        locals.var_ids_edge_dn11 = assign32770_e43464_d_n11;
        locals.var_ids_edge_dn12 = assign32770_e43464_d_n12;
        locals.var_ids_edge_dn13 = assign32770_e43464_d_n13;
        locals.var_ids_edge_dn14 = assign32770_e43464_d_n14;
        locals.var_ids_edge_rv = 0.0;

        let (assign32780_e43470, assign32780_e43470_d_n0, assign32780_e43470_d_n2, assign32780_e43470_d_n3, assign32780_e43470_d_n4, assign32780_e43470_d_n5, assign32780_e43470_d_n6, assign32780_e43470_d_n7, assign32780_e43470_d_n8, assign32780_e43470_d_n9, assign32780_e43470_d_n10, assign32780_e43470_d_n11, assign32780_e43470_d_n12, assign32780_e43470_d_n13, assign32780_e43470_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32780_e43468: f64 = (locals.var_ids_edge + locals.var_ids);
        (assign32780_e43468, (locals.var_ids_edge_dn0 + locals.var_ids_dn0), (locals.var_ids_edge_dn2 + locals.var_ids_dn2), (locals.var_ids_edge_dn3 + locals.var_ids_dn3), (locals.var_ids_edge_dn4 + locals.var_ids_dn4), (locals.var_ids_edge_dn5 + locals.var_ids_dn5), (locals.var_ids_edge_dn6 + locals.var_ids_dn6), (locals.var_ids_edge_dn7 + locals.var_ids_dn7), (locals.var_ids_edge_dn8 + locals.var_ids_dn8), (locals.var_ids_edge_dn9 + locals.var_ids_dn9), (locals.var_ids_edge_dn10 + locals.var_ids_dn10), (locals.var_ids_edge_dn11 + locals.var_ids_dn11), (locals.var_ids_edge_dn12 + locals.var_ids_dn12), (locals.var_ids_edge_dn13 + locals.var_ids_dn13), (locals.var_ids_edge_dn14 + locals.var_ids_dn14),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn3, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn13, locals.var_ids_dn14,)
    }
};
        locals.var_ids = assign32780_e43470;
        locals.var_ids_dn0 = assign32780_e43470_d_n0;
        locals.var_ids_dn2 = assign32780_e43470_d_n2;
        locals.var_ids_dn3 = assign32780_e43470_d_n3;
        locals.var_ids_dn4 = assign32780_e43470_d_n4;
        locals.var_ids_dn5 = assign32780_e43470_d_n5;
        locals.var_ids_dn6 = assign32780_e43470_d_n6;
        locals.var_ids_dn7 = assign32780_e43470_d_n7;
        locals.var_ids_dn8 = assign32780_e43470_d_n8;
        locals.var_ids_dn9 = assign32780_e43470_d_n9;
        locals.var_ids_dn10 = assign32780_e43470_d_n10;
        locals.var_ids_dn11 = assign32780_e43470_d_n11;
        locals.var_ids_dn12 = assign32780_e43470_d_n12;
        locals.var_ids_dn13 = assign32780_e43470_d_n13;
        locals.var_ids_dn14 = assign32780_e43470_d_n14;
        locals.var_ids_rv = 0.0;

        let (assign32790_e43476,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32790_e43474: f64 = (p.p785 * p.p1062);
        (assign32790_e43474,)
    } else {
        (locals.var_noia_edge,)
    }
};
        locals.var_noia_edge = assign32790_e43476;
        locals.var_noia_edge_rv = 0.0;

        let (assign32800_e43482,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32800_e43480: f64 = (p.p799 * p.p1062);
        (assign32800_e43480,)
    } else {
        (locals.var_noib_edge,)
    }
};
        locals.var_noib_edge = assign32800_e43482;
        locals.var_noib_edge_rv = 0.0;

        let (assign32810_e43488,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32810_e43486: f64 = (p.p800 * p.p1062);
        (assign32810_e43486,)
    } else {
        (locals.var_noic_edge,)
    }
};
        locals.var_noic_edge = assign32810_e43488;
        locals.var_noic_edge_rv = 0.0;

        let (assign32820_e43496,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32820_e43493: f64 = (2.0 * locals.var_lintnoi_i);
        let assign32820_e43494: f64 = (locals.var_leff - assign32820_e43493);
        (assign32820_e43494,)
    } else {
        (locals.var_leffnoi_edge,)
    }
};
        locals.var_leffnoi_edge = assign32820_e43496;
        locals.var_leffnoi_edge_rv = 0.0;

        let (assign32830_e43502,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32830_e43500: f64 = (locals.var_leffnoi_edge * locals.var_leffnoi_edge);
        (assign32830_e43500,)
    } else {
        (locals.var_leffnoisq_edge,)
    }
};
        locals.var_leffnoisq_edge = assign32830_e43502;
        locals.var_leffnoisq_edge_rv = 0.0;

        let (assign32840_e43514, assign32840_e43514_d_n0, assign32840_e43514_d_n2, assign32840_e43514_d_n3, assign32840_e43514_d_n4, assign32840_e43514_d_n5, assign32840_e43514_d_n6, assign32840_e43514_d_n7, assign32840_e43514_d_n8, assign32840_e43514_d_n9, assign32840_e43514_d_n10, assign32840_e43514_d_n11, assign32840_e43514_d_n12, assign32840_e43514_d_n13, assign32840_e43514_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32840_e43506: f64 = (locals.var_vt / 1.60219e-19);
        let assign32840_e43509: f64 = (locals.var_cox + locals.var_cdep);
        let assign32840_e43511: f64 = (assign32840_e43509 + locals.var_citedge_i);
        let assign32840_e43512: f64 = (assign32840_e43506 * assign32840_e43511);
        (assign32840_e43512, (assign32840_e43506 * locals.var_cdep_dn0), (assign32840_e43506 * locals.var_cdep_dn2), (assign32840_e43506 * locals.var_cdep_dn3), (((locals.var_vt_dn4 / 1.60219e-19) * assign32840_e43511) + (assign32840_e43506 * locals.var_cdep_dn4)), (assign32840_e43506 * locals.var_cdep_dn5), (assign32840_e43506 * locals.var_cdep_dn6), (assign32840_e43506 * locals.var_cdep_dn7), (assign32840_e43506 * locals.var_cdep_dn8), (assign32840_e43506 * locals.var_cdep_dn9), (assign32840_e43506 * locals.var_cdep_dn10), (assign32840_e43506 * locals.var_cdep_dn11), (assign32840_e43506 * locals.var_cdep_dn12), (assign32840_e43506 * locals.var_cdep_dn13), (assign32840_e43506 * locals.var_cdep_dn14),)
    } else {
        (locals.var_nstar, locals.var_nstar_dn0, locals.var_nstar_dn2, locals.var_nstar_dn3, locals.var_nstar_dn4, locals.var_nstar_dn5, locals.var_nstar_dn6, locals.var_nstar_dn7, locals.var_nstar_dn8, locals.var_nstar_dn9, locals.var_nstar_dn10, locals.var_nstar_dn11, locals.var_nstar_dn12, locals.var_nstar_dn13, locals.var_nstar_dn14,)
    }
};
        locals.var_nstar = assign32840_e43514;
        locals.var_nstar_dn0 = assign32840_e43514_d_n0;
        locals.var_nstar_dn2 = assign32840_e43514_d_n2;
        locals.var_nstar_dn3 = assign32840_e43514_d_n3;
        locals.var_nstar_dn4 = assign32840_e43514_d_n4;
        locals.var_nstar_dn5 = assign32840_e43514_d_n5;
        locals.var_nstar_dn6 = assign32840_e43514_d_n6;
        locals.var_nstar_dn7 = assign32840_e43514_d_n7;
        locals.var_nstar_dn8 = assign32840_e43514_d_n8;
        locals.var_nstar_dn9 = assign32840_e43514_d_n9;
        locals.var_nstar_dn10 = assign32840_e43514_d_n10;
        locals.var_nstar_dn11 = assign32840_e43514_d_n11;
        locals.var_nstar_dn12 = assign32840_e43514_d_n12;
        locals.var_nstar_dn13 = assign32840_e43514_d_n13;
        locals.var_nstar_dn14 = assign32840_e43514_d_n14;
        locals.var_nstar_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_107(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32850_e43528, assign32850_e43528_d_n0, assign32850_e43528_d_n2, assign32850_e43528_d_n3, assign32850_e43528_d_n4, assign32850_e43528_d_n5, assign32850_e43528_d_n6, assign32850_e43528_d_n7, assign32850_e43528_d_n8, assign32850_e43528_d_n9, assign32850_e43528_d_n10, assign32850_e43528_d_n11, assign32850_e43528_d_n12, assign32850_e43528_d_n13, assign32850_e43528_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32850_e43518: f64 = (2.0 * locals.var_nq_edge);
        let assign32850_e43520: f64 = (assign32850_e43518 * locals.var_cox);
        let assign32850_e43522: f64 = (assign32850_e43520 * locals.var_vt);
        let assign32850_e43524: f64 = (assign32850_e43522 * locals.var_qdeff_edge);
        let assign32850_e43526: f64 = (assign32850_e43524 / 1.60219e-19);
        (assign32850_e43526, ((((((2.0 * locals.var_nq_edge_dn0) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn0)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn2) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn2)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn3) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn3)) / 1.60219e-19), (((((((2.0 * locals.var_nq_edge_dn4) * locals.var_cox) * locals.var_vt) + (assign32850_e43520 * locals.var_vt_dn4)) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn4)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn5) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn5)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn6) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn6)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn7) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn7)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn8) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn8)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn9) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn9)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn10) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn10)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn11) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn11)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn12) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn12)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn13) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn13)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn14) * locals.var_cox) * locals.var_vt) * locals.var_qdeff_edge) + (assign32850_e43522 * locals.var_qdeff_edge_dn14)) / 1.60219e-19),)
    } else {
        (locals.var_nl, locals.var_nl_dn0, locals.var_nl_dn2, locals.var_nl_dn3, locals.var_nl_dn4, locals.var_nl_dn5, locals.var_nl_dn6, locals.var_nl_dn7, locals.var_nl_dn8, locals.var_nl_dn9, locals.var_nl_dn10, locals.var_nl_dn11, locals.var_nl_dn12, locals.var_nl_dn13, locals.var_nl_dn14,)
    }
};
        locals.var_nl = assign32850_e43528;
        locals.var_nl_dn0 = assign32850_e43528_d_n0;
        locals.var_nl_dn2 = assign32850_e43528_d_n2;
        locals.var_nl_dn3 = assign32850_e43528_d_n3;
        locals.var_nl_dn4 = assign32850_e43528_d_n4;
        locals.var_nl_dn5 = assign32850_e43528_d_n5;
        locals.var_nl_dn6 = assign32850_e43528_d_n6;
        locals.var_nl_dn7 = assign32850_e43528_d_n7;
        locals.var_nl_dn8 = assign32850_e43528_d_n8;
        locals.var_nl_dn9 = assign32850_e43528_d_n9;
        locals.var_nl_dn10 = assign32850_e43528_d_n10;
        locals.var_nl_dn11 = assign32850_e43528_d_n11;
        locals.var_nl_dn12 = assign32850_e43528_d_n12;
        locals.var_nl_dn13 = assign32850_e43528_d_n13;
        locals.var_nl_dn14 = assign32850_e43528_d_n14;
        locals.var_nl_rv = 0.0;

        let (assign32860_e43543, assign32860_e43543_d_n0, assign32860_e43543_d_n2, assign32860_e43543_d_n3, assign32860_e43543_d_n4, assign32860_e43543_d_n5, assign32860_e43543_d_n6, assign32860_e43543_d_n7, assign32860_e43543_d_n8, assign32860_e43543_d_n9, assign32860_e43543_d_n10, assign32860_e43543_d_n11, assign32860_e43543_d_n12, assign32860_e43543_d_n13, assign32860_e43543_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32860_e43532: f64 = (1.60219e-19 * 1.60219e-19);
        let assign32860_e43534: f64 = (assign32860_e43532 * 1.60219e-19);
        let assign32860_e43536: f64 = (assign32860_e43534 * locals.var_vt);
        let assign32860_e43538: f64 = (locals.var_ids_edge).abs();
        let assign32860_e43539: f64 = (assign32860_e43536 * assign32860_e43538);
        let assign32860_e43541: f64 = (assign32860_e43539 * locals.var_ueff);
        (assign32860_e43541, (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn0 } else { (-locals.var_ids_edge_dn0) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn0)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn2 } else { (-locals.var_ids_edge_dn2) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn2)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn3 } else { (-locals.var_ids_edge_dn3) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn3)), (((((assign32860_e43534 * locals.var_vt_dn4) * assign32860_e43538) + (assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn4 } else { (-locals.var_ids_edge_dn4) })) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn4)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn5 } else { (-locals.var_ids_edge_dn5) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn5)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn6 } else { (-locals.var_ids_edge_dn6) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn6)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn7 } else { (-locals.var_ids_edge_dn7) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn7)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn8 } else { (-locals.var_ids_edge_dn8) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn8)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn9 } else { (-locals.var_ids_edge_dn9) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn9)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn10 } else { (-locals.var_ids_edge_dn10) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn10)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn11 } else { (-locals.var_ids_edge_dn11) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn11)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn12 } else { (-locals.var_ids_edge_dn12) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn12)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn13 } else { (-locals.var_ids_edge_dn13) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn13)), (((assign32860_e43536 * if locals.var_ids_edge >= 0.0 { locals.var_ids_edge_dn14 } else { (-locals.var_ids_edge_dn14) }) * locals.var_ueff) + (assign32860_e43539 * locals.var_ueff_dn14)),)
    } else {
        (locals.var_t0a, locals.var_t0a_dn0, locals.var_t0a_dn2, locals.var_t0a_dn3, locals.var_t0a_dn4, locals.var_t0a_dn5, locals.var_t0a_dn6, locals.var_t0a_dn7, locals.var_t0a_dn8, locals.var_t0a_dn9, locals.var_t0a_dn10, locals.var_t0a_dn11, locals.var_t0a_dn12, locals.var_t0a_dn13, locals.var_t0a_dn14,)
    }
};
        locals.var_t0a = assign32860_e43543;
        locals.var_t0a_dn0 = assign32860_e43543_d_n0;
        locals.var_t0a_dn2 = assign32860_e43543_d_n2;
        locals.var_t0a_dn3 = assign32860_e43543_d_n3;
        locals.var_t0a_dn4 = assign32860_e43543_d_n4;
        locals.var_t0a_dn5 = assign32860_e43543_d_n5;
        locals.var_t0a_dn6 = assign32860_e43543_d_n6;
        locals.var_t0a_dn7 = assign32860_e43543_d_n7;
        locals.var_t0a_dn8 = assign32860_e43543_d_n8;
        locals.var_t0a_dn9 = assign32860_e43543_d_n9;
        locals.var_t0a_dn10 = assign32860_e43543_d_n10;
        locals.var_t0a_dn11 = assign32860_e43543_d_n11;
        locals.var_t0a_dn12 = assign32860_e43543_d_n12;
        locals.var_t0a_dn13 = assign32860_e43543_d_n13;
        locals.var_t0a_dn14 = assign32860_e43543_d_n14;
        locals.var_t0a_rv = 0.0;

        let (assign32870_e43553, assign32870_e43553_d_n0, assign32870_e43553_d_n2, assign32870_e43553_d_n3, assign32870_e43553_d_n4, assign32870_e43553_d_n5, assign32870_e43553_d_n6, assign32870_e43553_d_n7, assign32870_e43553_d_n8, assign32870_e43553_d_n9, assign32870_e43553_d_n10, assign32870_e43553_d_n11, assign32870_e43553_d_n12, assign32870_e43553_d_n13, assign32870_e43553_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32870_e43547: f64 = (1.60219e-19 * locals.var_vt);
        let assign32870_e43549: f64 = (assign32870_e43547 * locals.var_ids_edge);
        let assign32870_e43551: f64 = (assign32870_e43549 * locals.var_ids_edge);
        (assign32870_e43551, (((assign32870_e43547 * locals.var_ids_edge_dn0) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn0)), (((assign32870_e43547 * locals.var_ids_edge_dn2) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn2)), (((assign32870_e43547 * locals.var_ids_edge_dn3) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn3)), (((((1.60219e-19 * locals.var_vt_dn4) * locals.var_ids_edge) + (assign32870_e43547 * locals.var_ids_edge_dn4)) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn4)), (((assign32870_e43547 * locals.var_ids_edge_dn5) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn5)), (((assign32870_e43547 * locals.var_ids_edge_dn6) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn6)), (((assign32870_e43547 * locals.var_ids_edge_dn7) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn7)), (((assign32870_e43547 * locals.var_ids_edge_dn8) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn8)), (((assign32870_e43547 * locals.var_ids_edge_dn9) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn9)), (((assign32870_e43547 * locals.var_ids_edge_dn10) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn10)), (((assign32870_e43547 * locals.var_ids_edge_dn11) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn11)), (((assign32870_e43547 * locals.var_ids_edge_dn12) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn12)), (((assign32870_e43547 * locals.var_ids_edge_dn13) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn13)), (((assign32870_e43547 * locals.var_ids_edge_dn14) * locals.var_ids_edge) + (assign32870_e43549 * locals.var_ids_edge_dn14)),)
    } else {
        (locals.var_t0b, locals.var_t0b_dn0, locals.var_t0b_dn2, locals.var_t0b_dn3, locals.var_t0b_dn4, locals.var_t0b_dn5, locals.var_t0b_dn6, locals.var_t0b_dn7, locals.var_t0b_dn8, locals.var_t0b_dn9, locals.var_t0b_dn10, locals.var_t0b_dn11, locals.var_t0b_dn12, locals.var_t0b_dn13, locals.var_t0b_dn14,)
    }
};
        locals.var_t0b = assign32870_e43553;
        locals.var_t0b_dn0 = assign32870_e43553_d_n0;
        locals.var_t0b_dn2 = assign32870_e43553_d_n2;
        locals.var_t0b_dn3 = assign32870_e43553_d_n3;
        locals.var_t0b_dn4 = assign32870_e43553_d_n4;
        locals.var_t0b_dn5 = assign32870_e43553_d_n5;
        locals.var_t0b_dn6 = assign32870_e43553_d_n6;
        locals.var_t0b_dn7 = assign32870_e43553_d_n7;
        locals.var_t0b_dn8 = assign32870_e43553_d_n8;
        locals.var_t0b_dn9 = assign32870_e43553_d_n9;
        locals.var_t0b_dn10 = assign32870_e43553_d_n10;
        locals.var_t0b_dn11 = assign32870_e43553_d_n11;
        locals.var_t0b_dn12 = assign32870_e43553_d_n12;
        locals.var_t0b_dn13 = assign32870_e43553_d_n13;
        locals.var_t0b_dn14 = assign32870_e43553_d_n14;
        locals.var_t0b_rv = 0.0;

        let (assign32880_e43567, assign32880_e43567_d_n0, assign32880_e43567_d_n2, assign32880_e43567_d_n3, assign32880_e43567_d_n4, assign32880_e43567_d_n5, assign32880_e43567_d_n6, assign32880_e43567_d_n7, assign32880_e43567_d_n8, assign32880_e43567_d_n9, assign32880_e43567_d_n10, assign32880_e43567_d_n11, assign32880_e43567_d_n12, assign32880_e43567_d_n13, assign32880_e43567_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32880_e43558: f64 = (locals.var_noib_edge * locals.var_nl);
        let assign32880_e43559: f64 = (locals.var_noia_edge + assign32880_e43558);
        let assign32880_e43562: f64 = (locals.var_noic_edge * locals.var_nl);
        let assign32880_e43564: f64 = (assign32880_e43562 * locals.var_nl);
        let assign32880_e43565: f64 = (assign32880_e43559 + assign32880_e43564);
        (assign32880_e43565, ((locals.var_noib_edge * locals.var_nl_dn0) + (((locals.var_noic_edge * locals.var_nl_dn0) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn0))), ((locals.var_noib_edge * locals.var_nl_dn2) + (((locals.var_noic_edge * locals.var_nl_dn2) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn2))), ((locals.var_noib_edge * locals.var_nl_dn3) + (((locals.var_noic_edge * locals.var_nl_dn3) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn3))), ((locals.var_noib_edge * locals.var_nl_dn4) + (((locals.var_noic_edge * locals.var_nl_dn4) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn4))), ((locals.var_noib_edge * locals.var_nl_dn5) + (((locals.var_noic_edge * locals.var_nl_dn5) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn5))), ((locals.var_noib_edge * locals.var_nl_dn6) + (((locals.var_noic_edge * locals.var_nl_dn6) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn6))), ((locals.var_noib_edge * locals.var_nl_dn7) + (((locals.var_noic_edge * locals.var_nl_dn7) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn7))), ((locals.var_noib_edge * locals.var_nl_dn8) + (((locals.var_noic_edge * locals.var_nl_dn8) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn8))), ((locals.var_noib_edge * locals.var_nl_dn9) + (((locals.var_noic_edge * locals.var_nl_dn9) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn9))), ((locals.var_noib_edge * locals.var_nl_dn10) + (((locals.var_noic_edge * locals.var_nl_dn10) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn10))), ((locals.var_noib_edge * locals.var_nl_dn11) + (((locals.var_noic_edge * locals.var_nl_dn11) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn11))), ((locals.var_noib_edge * locals.var_nl_dn12) + (((locals.var_noic_edge * locals.var_nl_dn12) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn12))), ((locals.var_noib_edge * locals.var_nl_dn13) + (((locals.var_noic_edge * locals.var_nl_dn13) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn13))), ((locals.var_noib_edge * locals.var_nl_dn14) + (((locals.var_noic_edge * locals.var_nl_dn14) * locals.var_nl) + (assign32880_e43562 * locals.var_nl_dn14))),)
    } else {
        (locals.var_t0c, locals.var_t0c_dn0, locals.var_t0c_dn2, locals.var_t0c_dn3, locals.var_t0c_dn4, locals.var_t0c_dn5, locals.var_t0c_dn6, locals.var_t0c_dn7, locals.var_t0c_dn8, locals.var_t0c_dn9, locals.var_t0c_dn10, locals.var_t0c_dn11, locals.var_t0c_dn12, locals.var_t0c_dn13, locals.var_t0c_dn14,)
    }
};
        locals.var_t0c = assign32880_e43567;
        locals.var_t0c_dn0 = assign32880_e43567_d_n0;
        locals.var_t0c_dn2 = assign32880_e43567_d_n2;
        locals.var_t0c_dn3 = assign32880_e43567_d_n3;
        locals.var_t0c_dn4 = assign32880_e43567_d_n4;
        locals.var_t0c_dn5 = assign32880_e43567_d_n5;
        locals.var_t0c_dn6 = assign32880_e43567_d_n6;
        locals.var_t0c_dn7 = assign32880_e43567_d_n7;
        locals.var_t0c_dn8 = assign32880_e43567_d_n8;
        locals.var_t0c_dn9 = assign32880_e43567_d_n9;
        locals.var_t0c_dn10 = assign32880_e43567_d_n10;
        locals.var_t0c_dn11 = assign32880_e43567_d_n11;
        locals.var_t0c_dn12 = assign32880_e43567_d_n12;
        locals.var_t0c_dn13 = assign32880_e43567_d_n13;
        locals.var_t0c_dn14 = assign32880_e43567_d_n14;
        locals.var_t0c_rv = 0.0;

        let (assign32890_e43577, assign32890_e43577_d_n0, assign32890_e43577_d_n2, assign32890_e43577_d_n3, assign32890_e43577_d_n4, assign32890_e43577_d_n5, assign32890_e43577_d_n6, assign32890_e43577_d_n7, assign32890_e43577_d_n8, assign32890_e43577_d_n9, assign32890_e43577_d_n10, assign32890_e43577_d_n11, assign32890_e43577_d_n12, assign32890_e43577_d_n13, assign32890_e43577_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32890_e43571: f64 = (locals.var_nl + locals.var_nstar);
        let assign32890_e43574: f64 = (locals.var_nl + locals.var_nstar);
        let assign32890_e43575: f64 = (assign32890_e43571 * assign32890_e43574);
        (assign32890_e43575, (((locals.var_nl_dn0 + locals.var_nstar_dn0) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn0 + locals.var_nstar_dn0))), (((locals.var_nl_dn2 + locals.var_nstar_dn2) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn2 + locals.var_nstar_dn2))), (((locals.var_nl_dn3 + locals.var_nstar_dn3) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn3 + locals.var_nstar_dn3))), (((locals.var_nl_dn4 + locals.var_nstar_dn4) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn4 + locals.var_nstar_dn4))), (((locals.var_nl_dn5 + locals.var_nstar_dn5) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn5 + locals.var_nstar_dn5))), (((locals.var_nl_dn6 + locals.var_nstar_dn6) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn6 + locals.var_nstar_dn6))), (((locals.var_nl_dn7 + locals.var_nstar_dn7) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn7 + locals.var_nstar_dn7))), (((locals.var_nl_dn8 + locals.var_nstar_dn8) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn8 + locals.var_nstar_dn8))), (((locals.var_nl_dn9 + locals.var_nstar_dn9) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn9 + locals.var_nstar_dn9))), (((locals.var_nl_dn10 + locals.var_nstar_dn10) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn10 + locals.var_nstar_dn10))), (((locals.var_nl_dn11 + locals.var_nstar_dn11) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn11 + locals.var_nstar_dn11))), (((locals.var_nl_dn12 + locals.var_nstar_dn12) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn12 + locals.var_nstar_dn12))), (((locals.var_nl_dn13 + locals.var_nstar_dn13) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn13 + locals.var_nstar_dn13))), (((locals.var_nl_dn14 + locals.var_nstar_dn14) * assign32890_e43574) + (assign32890_e43571 * (locals.var_nl_dn14 + locals.var_nstar_dn14))),)
    } else {
        (locals.var_t0d, locals.var_t0d_dn0, locals.var_t0d_dn2, locals.var_t0d_dn3, locals.var_t0d_dn4, locals.var_t0d_dn5, locals.var_t0d_dn6, locals.var_t0d_dn7, locals.var_t0d_dn8, locals.var_t0d_dn9, locals.var_t0d_dn10, locals.var_t0d_dn11, locals.var_t0d_dn12, locals.var_t0d_dn13, locals.var_t0d_dn14,)
    }
};
        locals.var_t0d = assign32890_e43577;
        locals.var_t0d_dn0 = assign32890_e43577_d_n0;
        locals.var_t0d_dn2 = assign32890_e43577_d_n2;
        locals.var_t0d_dn3 = assign32890_e43577_d_n3;
        locals.var_t0d_dn4 = assign32890_e43577_d_n4;
        locals.var_t0d_dn5 = assign32890_e43577_d_n5;
        locals.var_t0d_dn6 = assign32890_e43577_d_n6;
        locals.var_t0d_dn7 = assign32890_e43577_d_n7;
        locals.var_t0d_dn8 = assign32890_e43577_d_n8;
        locals.var_t0d_dn9 = assign32890_e43577_d_n9;
        locals.var_t0d_dn10 = assign32890_e43577_d_n10;
        locals.var_t0d_dn11 = assign32890_e43577_d_n11;
        locals.var_t0d_dn12 = assign32890_e43577_d_n12;
        locals.var_t0d_dn13 = assign32890_e43577_d_n13;
        locals.var_t0d_dn14 = assign32890_e43577_d_n14;
        locals.var_t0d_rv = 0.0;

        let (assign32900_e43585, assign32900_e43585_d_n0, assign32900_e43585_d_n2, assign32900_e43585_d_n3, assign32900_e43585_d_n4, assign32900_e43585_d_n5, assign32900_e43585_d_n6, assign32900_e43585_d_n7, assign32900_e43585_d_n8, assign32900_e43585_d_n9, assign32900_e43585_d_n10, assign32900_e43585_d_n11, assign32900_e43585_d_n12, assign32900_e43585_d_n13, assign32900_e43585_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32900_e43581: f64 = (locals.var_noia_edge * 1.60219e-19);
        let assign32900_e43583: f64 = (assign32900_e43581 * locals.var_vt);
        (assign32900_e43583, 0.0, 0.0, 0.0, (assign32900_e43581 * locals.var_vt_dn4), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0e, locals.var_t0e_dn0, locals.var_t0e_dn2, locals.var_t0e_dn3, locals.var_t0e_dn4, locals.var_t0e_dn5, locals.var_t0e_dn6, locals.var_t0e_dn7, locals.var_t0e_dn8, locals.var_t0e_dn9, locals.var_t0e_dn10, locals.var_t0e_dn11, locals.var_t0e_dn12, locals.var_t0e_dn13, locals.var_t0e_dn14,)
    }
};
        locals.var_t0e = assign32900_e43585;
        locals.var_t0e_dn0 = assign32900_e43585_d_n0;
        locals.var_t0e_dn2 = assign32900_e43585_d_n2;
        locals.var_t0e_dn3 = assign32900_e43585_d_n3;
        locals.var_t0e_dn4 = assign32900_e43585_d_n4;
        locals.var_t0e_dn5 = assign32900_e43585_d_n5;
        locals.var_t0e_dn6 = assign32900_e43585_d_n6;
        locals.var_t0e_dn7 = assign32900_e43585_d_n7;
        locals.var_t0e_dn8 = assign32900_e43585_d_n8;
        locals.var_t0e_dn9 = assign32900_e43585_d_n9;
        locals.var_t0e_dn10 = assign32900_e43585_d_n10;
        locals.var_t0e_dn11 = assign32900_e43585_d_n11;
        locals.var_t0e_dn12 = assign32900_e43585_d_n12;
        locals.var_t0e_dn13 = assign32900_e43585_d_n13;
        locals.var_t0e_dn14 = assign32900_e43585_d_n14;
        locals.var_t0e_rv = 0.0;

        let (assign32910_e43599, assign32910_e43599_d_n0, assign32910_e43599_d_n2, assign32910_e43599_d_n3, assign32910_e43599_d_n4, assign32910_e43599_d_n5, assign32910_e43599_d_n6, assign32910_e43599_d_n7, assign32910_e43599_d_n8, assign32910_e43599_d_n9, assign32910_e43599_d_n10, assign32910_e43599_d_n11, assign32910_e43599_d_n12, assign32910_e43599_d_n13, assign32910_e43599_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32910_e43589: f64 = (2.0 * locals.var_nq_edge);
        let assign32910_e43591: f64 = (assign32910_e43589 * locals.var_cox);
        let assign32910_e43593: f64 = (assign32910_e43591 * locals.var_vt);
        let assign32910_e43595: f64 = (assign32910_e43593 * locals.var_qs_edge);
        let assign32910_e43597: f64 = (assign32910_e43595 / 1.60219e-19);
        (assign32910_e43597, ((((((2.0 * locals.var_nq_edge_dn0) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn0)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn2) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn2)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn3) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn3)) / 1.60219e-19), (((((((2.0 * locals.var_nq_edge_dn4) * locals.var_cox) * locals.var_vt) + (assign32910_e43591 * locals.var_vt_dn4)) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn4)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn5) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn5)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn6) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn6)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn7) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn7)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn8) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn8)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn9) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn9)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn10) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn10)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn11) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn11)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn12) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn12)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn13) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn13)) / 1.60219e-19), ((((((2.0 * locals.var_nq_edge_dn14) * locals.var_cox) * locals.var_vt) * locals.var_qs_edge) + (assign32910_e43593 * locals.var_qs_edge_dn14)) / 1.60219e-19),)
    } else {
        (locals.var_n0, locals.var_n0_dn0, locals.var_n0_dn2, locals.var_n0_dn3, locals.var_n0_dn4, locals.var_n0_dn5, locals.var_n0_dn6, locals.var_n0_dn7, locals.var_n0_dn8, locals.var_n0_dn9, locals.var_n0_dn10, locals.var_n0_dn11, locals.var_n0_dn12, locals.var_n0_dn13, locals.var_n0_dn14,)
    }
};
        locals.var_n0 = assign32910_e43599;
        locals.var_n0_dn0 = assign32910_e43599_d_n0;
        locals.var_n0_dn2 = assign32910_e43599_d_n2;
        locals.var_n0_dn3 = assign32910_e43599_d_n3;
        locals.var_n0_dn4 = assign32910_e43599_d_n4;
        locals.var_n0_dn5 = assign32910_e43599_d_n5;
        locals.var_n0_dn6 = assign32910_e43599_d_n6;
        locals.var_n0_dn7 = assign32910_e43599_d_n7;
        locals.var_n0_dn8 = assign32910_e43599_d_n8;
        locals.var_n0_dn9 = assign32910_e43599_d_n9;
        locals.var_n0_dn10 = assign32910_e43599_d_n10;
        locals.var_n0_dn11 = assign32910_e43599_d_n11;
        locals.var_n0_dn12 = assign32910_e43599_d_n12;
        locals.var_n0_dn13 = assign32910_e43599_d_n13;
        locals.var_n0_dn14 = assign32910_e43599_d_n14;
        locals.var_n0_rv = 0.0;

        let (assign32920_e43614, assign32920_e43614_d_n0, assign32920_e43614_d_n2, assign32920_e43614_d_n3, assign32920_e43614_d_n4, assign32920_e43614_d_n5, assign32920_e43614_d_n6, assign32920_e43614_d_n7, assign32920_e43614_d_n8, assign32920_e43614_d_n9, assign32920_e43614_d_n10, assign32920_e43614_d_n11, assign32920_e43614_d_n12, assign32920_e43614_d_n13, assign32920_e43614_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32920_e43604: f64 = (locals.var_n0 + locals.var_nstar);
        let assign32920_e43607: f64 = (locals.var_nl + locals.var_nstar);
        let assign32920_e43608: f64 = (assign32920_e43604 / assign32920_e43607);
        let assign32920_e43610: f64 = (assign32920_e43608).max(1e-38);
        let assign32920_e43611: f64 = (assign32920_e43610).ln();
        let assign32920_e43612: f64 = (locals.var_noia_edge * assign32920_e43611);
        (assign32920_e43612, (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn0 + locals.var_nstar_dn0) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn0 + locals.var_nstar_dn0))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn2 + locals.var_nstar_dn2) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn2 + locals.var_nstar_dn2))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn3 + locals.var_nstar_dn3) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn3 + locals.var_nstar_dn3))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn4 + locals.var_nstar_dn4) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn4 + locals.var_nstar_dn4))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn5 + locals.var_nstar_dn5) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn5 + locals.var_nstar_dn5))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn6 + locals.var_nstar_dn6) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn6 + locals.var_nstar_dn6))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn7 + locals.var_nstar_dn7) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn7 + locals.var_nstar_dn7))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn8 + locals.var_nstar_dn8) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn8 + locals.var_nstar_dn8))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn9 + locals.var_nstar_dn9) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn9 + locals.var_nstar_dn9))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn10 + locals.var_nstar_dn10) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn10 + locals.var_nstar_dn10))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn11 + locals.var_nstar_dn11) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn11 + locals.var_nstar_dn11))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn12 + locals.var_nstar_dn12) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn12 + locals.var_nstar_dn12))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn13 + locals.var_nstar_dn13) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn13 + locals.var_nstar_dn13))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)), (locals.var_noia_edge * (if assign32920_e43608 >= 1e-38 { ((((locals.var_n0_dn14 + locals.var_nstar_dn14) * assign32920_e43607) - (assign32920_e43604 * (locals.var_nl_dn14 + locals.var_nstar_dn14))) / (assign32920_e43607 * assign32920_e43607)) } else { 0.0 } / assign32920_e43610)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign32920_e43614;
        locals.var_t1_dn0 = assign32920_e43614_d_n0;
        locals.var_t1_dn2 = assign32920_e43614_d_n2;
        locals.var_t1_dn3 = assign32920_e43614_d_n3;
        locals.var_t1_dn4 = assign32920_e43614_d_n4;
        locals.var_t1_dn5 = assign32920_e43614_d_n5;
        locals.var_t1_dn6 = assign32920_e43614_d_n6;
        locals.var_t1_dn7 = assign32920_e43614_d_n7;
        locals.var_t1_dn8 = assign32920_e43614_d_n8;
        locals.var_t1_dn9 = assign32920_e43614_d_n9;
        locals.var_t1_dn10 = assign32920_e43614_d_n10;
        locals.var_t1_dn11 = assign32920_e43614_d_n11;
        locals.var_t1_dn12 = assign32920_e43614_d_n12;
        locals.var_t1_dn13 = assign32920_e43614_d_n13;
        locals.var_t1_dn14 = assign32920_e43614_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign32930_e43622, assign32930_e43622_d_n0, assign32930_e43622_d_n2, assign32930_e43622_d_n3, assign32930_e43622_d_n4, assign32930_e43622_d_n5, assign32930_e43622_d_n6, assign32930_e43622_d_n7, assign32930_e43622_d_n8, assign32930_e43622_d_n9, assign32930_e43622_d_n10, assign32930_e43622_d_n11, assign32930_e43622_d_n12, assign32930_e43622_d_n13, assign32930_e43622_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32930_e43619: f64 = (locals.var_n0 - locals.var_nl);
        let assign32930_e43620: f64 = (locals.var_noib_edge * assign32930_e43619);
        (assign32930_e43620, (locals.var_noib_edge * (locals.var_n0_dn0 - locals.var_nl_dn0)), (locals.var_noib_edge * (locals.var_n0_dn2 - locals.var_nl_dn2)), (locals.var_noib_edge * (locals.var_n0_dn3 - locals.var_nl_dn3)), (locals.var_noib_edge * (locals.var_n0_dn4 - locals.var_nl_dn4)), (locals.var_noib_edge * (locals.var_n0_dn5 - locals.var_nl_dn5)), (locals.var_noib_edge * (locals.var_n0_dn6 - locals.var_nl_dn6)), (locals.var_noib_edge * (locals.var_n0_dn7 - locals.var_nl_dn7)), (locals.var_noib_edge * (locals.var_n0_dn8 - locals.var_nl_dn8)), (locals.var_noib_edge * (locals.var_n0_dn9 - locals.var_nl_dn9)), (locals.var_noib_edge * (locals.var_n0_dn10 - locals.var_nl_dn10)), (locals.var_noib_edge * (locals.var_n0_dn11 - locals.var_nl_dn11)), (locals.var_noib_edge * (locals.var_n0_dn12 - locals.var_nl_dn12)), (locals.var_noib_edge * (locals.var_n0_dn13 - locals.var_nl_dn13)), (locals.var_noib_edge * (locals.var_n0_dn14 - locals.var_nl_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign32930_e43622;
        locals.var_t2_dn0 = assign32930_e43622_d_n0;
        locals.var_t2_dn2 = assign32930_e43622_d_n2;
        locals.var_t2_dn3 = assign32930_e43622_d_n3;
        locals.var_t2_dn4 = assign32930_e43622_d_n4;
        locals.var_t2_dn5 = assign32930_e43622_d_n5;
        locals.var_t2_dn6 = assign32930_e43622_d_n6;
        locals.var_t2_dn7 = assign32930_e43622_d_n7;
        locals.var_t2_dn8 = assign32930_e43622_d_n8;
        locals.var_t2_dn9 = assign32930_e43622_d_n9;
        locals.var_t2_dn10 = assign32930_e43622_d_n10;
        locals.var_t2_dn11 = assign32930_e43622_d_n11;
        locals.var_t2_dn12 = assign32930_e43622_d_n12;
        locals.var_t2_dn13 = assign32930_e43622_d_n13;
        locals.var_t2_dn14 = assign32930_e43622_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign32940_e43636, assign32940_e43636_d_n0, assign32940_e43636_d_n2, assign32940_e43636_d_n3, assign32940_e43636_d_n4, assign32940_e43636_d_n5, assign32940_e43636_d_n6, assign32940_e43636_d_n7, assign32940_e43636_d_n8, assign32940_e43636_d_n9, assign32940_e43636_d_n10, assign32940_e43636_d_n11, assign32940_e43636_d_n12, assign32940_e43636_d_n13, assign32940_e43636_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32940_e43626: f64 = (0.5 * locals.var_noic_edge);
        let assign32940_e43629: f64 = (locals.var_n0 * locals.var_n0);
        let assign32940_e43632: f64 = (locals.var_nl * locals.var_nl);
        let assign32940_e43633: f64 = (assign32940_e43629 - assign32940_e43632);
        let assign32940_e43634: f64 = (assign32940_e43626 * assign32940_e43633);
        (assign32940_e43634, (assign32940_e43626 * (((locals.var_n0_dn0 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn0)) - ((locals.var_nl_dn0 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn0)))), (assign32940_e43626 * (((locals.var_n0_dn2 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn2)) - ((locals.var_nl_dn2 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn2)))), (assign32940_e43626 * (((locals.var_n0_dn3 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn3)) - ((locals.var_nl_dn3 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn3)))), (assign32940_e43626 * (((locals.var_n0_dn4 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn4)) - ((locals.var_nl_dn4 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn4)))), (assign32940_e43626 * (((locals.var_n0_dn5 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn5)) - ((locals.var_nl_dn5 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn5)))), (assign32940_e43626 * (((locals.var_n0_dn6 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn6)) - ((locals.var_nl_dn6 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn6)))), (assign32940_e43626 * (((locals.var_n0_dn7 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn7)) - ((locals.var_nl_dn7 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn7)))), (assign32940_e43626 * (((locals.var_n0_dn8 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn8)) - ((locals.var_nl_dn8 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn8)))), (assign32940_e43626 * (((locals.var_n0_dn9 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn9)) - ((locals.var_nl_dn9 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn9)))), (assign32940_e43626 * (((locals.var_n0_dn10 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn10)) - ((locals.var_nl_dn10 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn10)))), (assign32940_e43626 * (((locals.var_n0_dn11 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn11)) - ((locals.var_nl_dn11 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn11)))), (assign32940_e43626 * (((locals.var_n0_dn12 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn12)) - ((locals.var_nl_dn12 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn12)))), (assign32940_e43626 * (((locals.var_n0_dn13 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn13)) - ((locals.var_nl_dn13 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn13)))), (assign32940_e43626 * (((locals.var_n0_dn14 * locals.var_n0) + (locals.var_n0 * locals.var_n0_dn14)) - ((locals.var_nl_dn14 * locals.var_nl) + (locals.var_nl * locals.var_nl_dn14)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign32940_e43636;
        locals.var_t3_dn0 = assign32940_e43636_d_n0;
        locals.var_t3_dn2 = assign32940_e43636_d_n2;
        locals.var_t3_dn3 = assign32940_e43636_d_n3;
        locals.var_t3_dn4 = assign32940_e43636_d_n4;
        locals.var_t3_dn5 = assign32940_e43636_d_n5;
        locals.var_t3_dn6 = assign32940_e43636_d_n6;
        locals.var_t3_dn7 = assign32940_e43636_d_n7;
        locals.var_t3_dn8 = assign32940_e43636_d_n8;
        locals.var_t3_dn9 = assign32940_e43636_d_n9;
        locals.var_t3_dn10 = assign32940_e43636_d_n10;
        locals.var_t3_dn11 = assign32940_e43636_d_n11;
        locals.var_t3_dn12 = assign32940_e43636_d_n12;
        locals.var_t3_dn13 = assign32940_e43636_d_n13;
        locals.var_t3_dn14 = assign32940_e43636_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign32950_e43646, assign32950_e43646_d_n0, assign32950_e43646_d_n2, assign32950_e43646_d_n3, assign32950_e43646_d_n4, assign32950_e43646_d_n5, assign32950_e43646_d_n6, assign32950_e43646_d_n7, assign32950_e43646_d_n8, assign32950_e43646_d_n9, assign32950_e43646_d_n10, assign32950_e43646_d_n11, assign32950_e43646_d_n12, assign32950_e43646_d_n13, assign32950_e43646_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32950_e43640: f64 = (10000000000.0 * locals.var_leffnoisq_edge);
        let assign32950_e43642: f64 = (assign32950_e43640 * p.p957);
        let assign32950_e43644: f64 = (assign32950_e43642 * p.p2);
        (assign32950_e43644, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign32950_e43646;
        locals.var_t4_dn0 = assign32950_e43646_d_n0;
        locals.var_t4_dn2 = assign32950_e43646_d_n2;
        locals.var_t4_dn3 = assign32950_e43646_d_n3;
        locals.var_t4_dn4 = assign32950_e43646_d_n4;
        locals.var_t4_dn5 = assign32950_e43646_d_n5;
        locals.var_t4_dn6 = assign32950_e43646_d_n6;
        locals.var_t4_dn7 = assign32950_e43646_d_n7;
        locals.var_t4_dn8 = assign32950_e43646_d_n8;
        locals.var_t4_dn9 = assign32950_e43646_d_n9;
        locals.var_t4_dn10 = assign32950_e43646_d_n10;
        locals.var_t4_dn11 = assign32950_e43646_d_n11;
        locals.var_t4_dn12 = assign32950_e43646_d_n12;
        locals.var_t4_dn13 = assign32950_e43646_d_n13;
        locals.var_t4_dn14 = assign32950_e43646_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign32960_e43668, assign32960_e43668_d_n0, assign32960_e43668_d_n2, assign32960_e43668_d_n3, assign32960_e43668_d_n4, assign32960_e43668_d_n5, assign32960_e43668_d_n6, assign32960_e43668_d_n7, assign32960_e43668_d_n8, assign32960_e43668_d_n9, assign32960_e43668_d_n10, assign32960_e43668_d_n11, assign32960_e43668_d_n12, assign32960_e43668_d_n13, assign32960_e43668_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32960_e43650: f64 = (locals.var_t0a / locals.var_t0);
        let assign32960_e43653: f64 = (locals.var_t1 + locals.var_t2);
        let assign32960_e43655: f64 = (assign32960_e43653 + locals.var_t3);
        let assign32960_e43656: f64 = (assign32960_e43650 * assign32960_e43655);
        let assign32960_e43659: f64 = (locals.var_t0b / locals.var_t4);
        let assign32960_e43661: f64 = (assign32960_e43659 * locals.var_delclm);
        let assign32960_e43663: f64 = (assign32960_e43661 * locals.var_t0c);
        let assign32960_e43665: f64 = (assign32960_e43663 / locals.var_t0d);
        let assign32960_e43666: f64 = (assign32960_e43656 + assign32960_e43665);
        (assign32960_e43666, ((((((locals.var_t0a_dn0 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn0 + locals.var_t2_dn0) + locals.var_t3_dn0))) + ((((((((((locals.var_t0b_dn0 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn0)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn0)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn0)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn2 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn2 + locals.var_t2_dn2) + locals.var_t3_dn2))) + ((((((((((locals.var_t0b_dn2 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn2)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn2)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn2)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn3 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn3 + locals.var_t2_dn3) + locals.var_t3_dn3))) + ((((((((((locals.var_t0b_dn3 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn3)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn3)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn3)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn4 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn4 + locals.var_t2_dn4) + locals.var_t3_dn4))) + ((((((((((locals.var_t0b_dn4 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn4)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn4)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn4)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn5 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn5 + locals.var_t2_dn5) + locals.var_t3_dn5))) + ((((((((((locals.var_t0b_dn5 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn5)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn5)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn5)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn6 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn6 + locals.var_t2_dn6) + locals.var_t3_dn6))) + ((((((((((locals.var_t0b_dn6 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn6)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn6)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn6)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn7 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn7 + locals.var_t2_dn7) + locals.var_t3_dn7))) + ((((((((((locals.var_t0b_dn7 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn7)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn7)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn7)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn8 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn8 + locals.var_t2_dn8) + locals.var_t3_dn8))) + ((((((((((locals.var_t0b_dn8 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn8)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn8)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn8)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn9 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn9 + locals.var_t2_dn9) + locals.var_t3_dn9))) + ((((((((((locals.var_t0b_dn9 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn9)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn9)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn9)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn10 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn10 + locals.var_t2_dn10) + locals.var_t3_dn10))) + ((((((((((locals.var_t0b_dn10 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn10)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn10)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn10)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn11 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn11 + locals.var_t2_dn11) + locals.var_t3_dn11))) + ((((((((((locals.var_t0b_dn11 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn11)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn11)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn11)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn12 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn12 + locals.var_t2_dn12) + locals.var_t3_dn12))) + ((((((((((locals.var_t0b_dn12 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn12)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn12)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn12)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn12)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn13 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn13)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn13 + locals.var_t2_dn13) + locals.var_t3_dn13))) + ((((((((((locals.var_t0b_dn13 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn13)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn13)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn13)) / (locals.var_t0d * locals.var_t0d))), ((((((locals.var_t0a_dn14 * locals.var_t0) - (locals.var_t0a * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)) * assign32960_e43655) + (assign32960_e43650 * ((locals.var_t1_dn14 + locals.var_t2_dn14) + locals.var_t3_dn14))) + ((((((((((locals.var_t0b_dn14 * locals.var_t4) - (locals.var_t0b * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)) * locals.var_delclm) + (assign32960_e43659 * locals.var_delclm_dn14)) * locals.var_t0c) + (assign32960_e43661 * locals.var_t0c_dn14)) * locals.var_t0d) - (assign32960_e43663 * locals.var_t0d_dn14)) / (locals.var_t0d * locals.var_t0d))),)
    } else {
        (locals.var_ssi, locals.var_ssi_dn0, locals.var_ssi_dn2, locals.var_ssi_dn3, locals.var_ssi_dn4, locals.var_ssi_dn5, locals.var_ssi_dn6, locals.var_ssi_dn7, locals.var_ssi_dn8, locals.var_ssi_dn9, locals.var_ssi_dn10, locals.var_ssi_dn11, locals.var_ssi_dn12, locals.var_ssi_dn13, locals.var_ssi_dn14,)
    }
};
        locals.var_ssi = assign32960_e43668;
        locals.var_ssi_dn0 = assign32960_e43668_d_n0;
        locals.var_ssi_dn2 = assign32960_e43668_d_n2;
        locals.var_ssi_dn3 = assign32960_e43668_d_n3;
        locals.var_ssi_dn4 = assign32960_e43668_d_n4;
        locals.var_ssi_dn5 = assign32960_e43668_d_n5;
        locals.var_ssi_dn6 = assign32960_e43668_d_n6;
        locals.var_ssi_dn7 = assign32960_e43668_d_n7;
        locals.var_ssi_dn8 = assign32960_e43668_d_n8;
        locals.var_ssi_dn9 = assign32960_e43668_d_n9;
        locals.var_ssi_dn10 = assign32960_e43668_d_n10;
        locals.var_ssi_dn11 = assign32960_e43668_d_n11;
        locals.var_ssi_dn12 = assign32960_e43668_d_n12;
        locals.var_ssi_dn13 = assign32960_e43668_d_n13;
        locals.var_ssi_dn14 = assign32960_e43668_d_n14;
        locals.var_ssi_rv = 0.0;

        let (assign32970_e43682, assign32970_e43682_d_n0, assign32970_e43682_d_n2, assign32970_e43682_d_n3, assign32970_e43682_d_n4, assign32970_e43682_d_n5, assign32970_e43682_d_n6, assign32970_e43682_d_n7, assign32970_e43682_d_n8, assign32970_e43682_d_n9, assign32970_e43682_d_n10, assign32970_e43682_d_n11, assign32970_e43682_d_n12, assign32970_e43682_d_n13, assign32970_e43682_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32970_e43672: f64 = (p.p957 * p.p2);
        let assign32970_e43674: f64 = (assign32970_e43672 * locals.var_leffnoi_edge);
        let assign32970_e43676: f64 = (assign32970_e43674 * 10000000000.0);
        let assign32970_e43678: f64 = (assign32970_e43676 * locals.var_nstar);
        let assign32970_e43680: f64 = (assign32970_e43678 * locals.var_nstar);
        (assign32970_e43680, (((assign32970_e43676 * locals.var_nstar_dn0) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn0)), (((assign32970_e43676 * locals.var_nstar_dn2) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn2)), (((assign32970_e43676 * locals.var_nstar_dn3) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn3)), (((assign32970_e43676 * locals.var_nstar_dn4) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn4)), (((assign32970_e43676 * locals.var_nstar_dn5) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn5)), (((assign32970_e43676 * locals.var_nstar_dn6) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn6)), (((assign32970_e43676 * locals.var_nstar_dn7) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn7)), (((assign32970_e43676 * locals.var_nstar_dn8) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn8)), (((assign32970_e43676 * locals.var_nstar_dn9) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn9)), (((assign32970_e43676 * locals.var_nstar_dn10) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn10)), (((assign32970_e43676 * locals.var_nstar_dn11) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn11)), (((assign32970_e43676 * locals.var_nstar_dn12) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn12)), (((assign32970_e43676 * locals.var_nstar_dn13) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn13)), (((assign32970_e43676 * locals.var_nstar_dn14) * locals.var_nstar) + (assign32970_e43678 * locals.var_nstar_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign32970_e43682;
        locals.var_t5_dn0 = assign32970_e43682_d_n0;
        locals.var_t5_dn2 = assign32970_e43682_d_n2;
        locals.var_t5_dn3 = assign32970_e43682_d_n3;
        locals.var_t5_dn4 = assign32970_e43682_d_n4;
        locals.var_t5_dn5 = assign32970_e43682_d_n5;
        locals.var_t5_dn6 = assign32970_e43682_d_n6;
        locals.var_t5_dn7 = assign32970_e43682_d_n7;
        locals.var_t5_dn8 = assign32970_e43682_d_n8;
        locals.var_t5_dn9 = assign32970_e43682_d_n9;
        locals.var_t5_dn10 = assign32970_e43682_d_n10;
        locals.var_t5_dn11 = assign32970_e43682_d_n11;
        locals.var_t5_dn12 = assign32970_e43682_d_n12;
        locals.var_t5_dn13 = assign32970_e43682_d_n13;
        locals.var_t5_dn14 = assign32970_e43682_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign32980_e43692, assign32980_e43692_d_n0, assign32980_e43692_d_n2, assign32980_e43692_d_n3, assign32980_e43692_d_n4, assign32980_e43692_d_n5, assign32980_e43692_d_n6, assign32980_e43692_d_n7, assign32980_e43692_d_n8, assign32980_e43692_d_n9, assign32980_e43692_d_n10, assign32980_e43692_d_n11, assign32980_e43692_d_n12, assign32980_e43692_d_n13, assign32980_e43692_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32980_e43686: f64 = (locals.var_t0e / locals.var_t5);
        let assign32980_e43688: f64 = (assign32980_e43686 * locals.var_ids_edge);
        let assign32980_e43690: f64 = (assign32980_e43688 * locals.var_ids_edge);
        (assign32980_e43690, (((((((locals.var_t0e_dn0 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn0)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn0)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn0)), (((((((locals.var_t0e_dn2 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn2)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn2)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn2)), (((((((locals.var_t0e_dn3 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn3)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn3)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn3)), (((((((locals.var_t0e_dn4 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn4)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn4)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn4)), (((((((locals.var_t0e_dn5 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn5)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn5)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn5)), (((((((locals.var_t0e_dn6 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn6)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn6)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn6)), (((((((locals.var_t0e_dn7 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn7)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn7)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn7)), (((((((locals.var_t0e_dn8 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn8)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn8)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn8)), (((((((locals.var_t0e_dn9 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn9)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn9)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn9)), (((((((locals.var_t0e_dn10 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn10)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn10)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn10)), (((((((locals.var_t0e_dn11 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn11)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn11)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn11)), (((((((locals.var_t0e_dn12 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn12)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn12)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn12)), (((((((locals.var_t0e_dn13 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn13)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn13)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn13)), (((((((locals.var_t0e_dn14 * locals.var_t5) - (locals.var_t0e * locals.var_t5_dn14)) / (locals.var_t5 * locals.var_t5)) * locals.var_ids_edge) + (assign32980_e43686 * locals.var_ids_edge_dn14)) * locals.var_ids_edge) + (assign32980_e43688 * locals.var_ids_edge_dn14)),)
    } else {
        (locals.var_swi, locals.var_swi_dn0, locals.var_swi_dn2, locals.var_swi_dn3, locals.var_swi_dn4, locals.var_swi_dn5, locals.var_swi_dn6, locals.var_swi_dn7, locals.var_swi_dn8, locals.var_swi_dn9, locals.var_swi_dn10, locals.var_swi_dn11, locals.var_swi_dn12, locals.var_swi_dn13, locals.var_swi_dn14,)
    }
};
        locals.var_swi = assign32980_e43692;
        locals.var_swi_dn0 = assign32980_e43692_d_n0;
        locals.var_swi_dn2 = assign32980_e43692_d_n2;
        locals.var_swi_dn3 = assign32980_e43692_d_n3;
        locals.var_swi_dn4 = assign32980_e43692_d_n4;
        locals.var_swi_dn5 = assign32980_e43692_d_n5;
        locals.var_swi_dn6 = assign32980_e43692_d_n6;
        locals.var_swi_dn7 = assign32980_e43692_d_n7;
        locals.var_swi_dn8 = assign32980_e43692_d_n8;
        locals.var_swi_dn9 = assign32980_e43692_d_n9;
        locals.var_swi_dn10 = assign32980_e43692_d_n10;
        locals.var_swi_dn11 = assign32980_e43692_d_n11;
        locals.var_swi_dn12 = assign32980_e43692_d_n12;
        locals.var_swi_dn13 = assign32980_e43692_d_n13;
        locals.var_swi_dn14 = assign32980_e43692_d_n14;
        locals.var_swi_rv = 0.0;

        let (assign32990_e43698, assign32990_e43698_d_n0, assign32990_e43698_d_n2, assign32990_e43698_d_n3, assign32990_e43698_d_n4, assign32990_e43698_d_n5, assign32990_e43698_d_n6, assign32990_e43698_d_n7, assign32990_e43698_d_n8, assign32990_e43698_d_n9, assign32990_e43698_d_n10, assign32990_e43698_d_n11, assign32990_e43698_d_n12, assign32990_e43698_d_n13, assign32990_e43698_d_n14,) = {
    if (locals.var_guard730 != 0.0) {
        let assign32990_e43696: f64 = (locals.var_swi + locals.var_ssi);
        (assign32990_e43696, (locals.var_swi_dn0 + locals.var_ssi_dn0), (locals.var_swi_dn2 + locals.var_ssi_dn2), (locals.var_swi_dn3 + locals.var_ssi_dn3), (locals.var_swi_dn4 + locals.var_ssi_dn4), (locals.var_swi_dn5 + locals.var_ssi_dn5), (locals.var_swi_dn6 + locals.var_ssi_dn6), (locals.var_swi_dn7 + locals.var_ssi_dn7), (locals.var_swi_dn8 + locals.var_ssi_dn8), (locals.var_swi_dn9 + locals.var_ssi_dn9), (locals.var_swi_dn10 + locals.var_ssi_dn10), (locals.var_swi_dn11 + locals.var_ssi_dn11), (locals.var_swi_dn12 + locals.var_ssi_dn12), (locals.var_swi_dn13 + locals.var_ssi_dn13), (locals.var_swi_dn14 + locals.var_ssi_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign32990_e43698;
        locals.var_t6_dn0 = assign32990_e43698_d_n0;
        locals.var_t6_dn2 = assign32990_e43698_d_n2;
        locals.var_t6_dn3 = assign32990_e43698_d_n3;
        locals.var_t6_dn4 = assign32990_e43698_d_n4;
        locals.var_t6_dn5 = assign32990_e43698_d_n5;
        locals.var_t6_dn6 = assign32990_e43698_d_n6;
        locals.var_t6_dn7 = assign32990_e43698_d_n7;
        locals.var_t6_dn8 = assign32990_e43698_d_n8;
        locals.var_t6_dn9 = assign32990_e43698_d_n9;
        locals.var_t6_dn10 = assign32990_e43698_d_n10;
        locals.var_t6_dn11 = assign32990_e43698_d_n11;
        locals.var_t6_dn12 = assign32990_e43698_d_n12;
        locals.var_t6_dn13 = assign32990_e43698_d_n13;
        locals.var_t6_dn14 = assign32990_e43698_d_n14;
        locals.var_t6_rv = 0.0;

        let assign33000_e43701: f64 = if locals.var_t6 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard745 = assign33000_e43701;
        locals.var_guard745_rv = 0.0;

        let (assign33010_e43711, assign33010_e43711_d_n0, assign33010_e43711_d_n2, assign33010_e43711_d_n3, assign33010_e43711_d_n4, assign33010_e43711_d_n5, assign33010_e43711_d_n6, assign33010_e43711_d_n7, assign33010_e43711_d_n8, assign33010_e43711_d_n9, assign33010_e43711_d_n10, assign33010_e43711_d_n11, assign33010_e43711_d_n12, assign33010_e43711_d_n13, assign33010_e43711_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard745 != 0.0)) {
        let assign33010_e43707: f64 = (locals.var_ssi * locals.var_swi);
        let assign33010_e43709: f64 = (assign33010_e43707 / locals.var_t6);
        (assign33010_e43709, (((((locals.var_ssi_dn0 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn0)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn0)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn2 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn2)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn2)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn3 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn3)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn3)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn4 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn4)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn4)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn5 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn5)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn5)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn6 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn6)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn6)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn7 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn7)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn7)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn8 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn8)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn8)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn9 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn9)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn9)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn10 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn10)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn10)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn11 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn11)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn11)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn12 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn12)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn12)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn13 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn13)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn13)) / (locals.var_t6 * locals.var_t6)), (((((locals.var_ssi_dn14 * locals.var_swi) + (locals.var_ssi * locals.var_swi_dn14)) * locals.var_t6) - (assign33010_e43707 * locals.var_t6_dn14)) / (locals.var_t6 * locals.var_t6)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign33010_e43711;
        locals.var_t7_dn0 = assign33010_e43711_d_n0;
        locals.var_t7_dn2 = assign33010_e43711_d_n2;
        locals.var_t7_dn3 = assign33010_e43711_d_n3;
        locals.var_t7_dn4 = assign33010_e43711_d_n4;
        locals.var_t7_dn5 = assign33010_e43711_d_n5;
        locals.var_t7_dn6 = assign33010_e43711_d_n6;
        locals.var_t7_dn7 = assign33010_e43711_d_n7;
        locals.var_t7_dn8 = assign33010_e43711_d_n8;
        locals.var_t7_dn9 = assign33010_e43711_d_n9;
        locals.var_t7_dn10 = assign33010_e43711_d_n10;
        locals.var_t7_dn11 = assign33010_e43711_d_n11;
        locals.var_t7_dn12 = assign33010_e43711_d_n12;
        locals.var_t7_dn13 = assign33010_e43711_d_n13;
        locals.var_t7_dn14 = assign33010_e43711_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign33020_e43725, assign33020_e43725_d_n0, assign33020_e43725_d_n2, assign33020_e43725_d_n3, assign33020_e43725_d_n4, assign33020_e43725_d_n5, assign33020_e43725_d_n6, assign33020_e43725_d_n7, assign33020_e43725_d_n8, assign33020_e43725_d_n9, assign33020_e43725_d_n10, assign33020_e43725_d_n11, assign33020_e43725_d_n12, assign33020_e43725_d_n13, assign33020_e43725_d_n14,) = {
    if ((locals.var_guard730 != 0.0) && (locals.var_guard745 != 0.0)) {
        let assign33020_e43719: f64 = (locals.var_qs_edge - locals.var_qdeff_edge);
        let assign33020_e43721: f64 = (assign33020_e43719).powf(p.p1064);
        let assign33020_e43722: f64 = (p.p1063 * assign33020_e43721);
        let assign33020_e43723: f64 = (1.0 + assign33020_e43722);
        (assign33020_e43723, (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn0 - locals.var_qdeff_edge_dn0))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn0 - locals.var_qdeff_edge_dn0) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn2 - locals.var_qdeff_edge_dn2))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn2 - locals.var_qdeff_edge_dn2) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn3 - locals.var_qdeff_edge_dn3))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn3 - locals.var_qdeff_edge_dn3) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn4 - locals.var_qdeff_edge_dn4))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn4 - locals.var_qdeff_edge_dn4) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn5 - locals.var_qdeff_edge_dn5))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn5 - locals.var_qdeff_edge_dn5) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn6 - locals.var_qdeff_edge_dn6))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn6 - locals.var_qdeff_edge_dn6) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn7 - locals.var_qdeff_edge_dn7))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn7 - locals.var_qdeff_edge_dn7) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn8 - locals.var_qdeff_edge_dn8))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn8 - locals.var_qdeff_edge_dn8) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn9 - locals.var_qdeff_edge_dn9))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn9 - locals.var_qdeff_edge_dn9) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn10 - locals.var_qdeff_edge_dn10))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn10 - locals.var_qdeff_edge_dn10) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn11 - locals.var_qdeff_edge_dn11))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn11 - locals.var_qdeff_edge_dn11) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn12 - locals.var_qdeff_edge_dn12))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn12 - locals.var_qdeff_edge_dn12) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn13 - locals.var_qdeff_edge_dn13))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn13 - locals.var_qdeff_edge_dn13) / assign33020_e43719))) }), (p.p1063 * if 0.0 == 0.0 && ((p.p1064) as f64).is_finite() && ((p.p1064) as f64).fract() == 0.0 { if p.p1064 == 0.0 { 0.0 } else { (p.p1064 * ((assign33020_e43719).powf(p.p1064 - 1.0) * (locals.var_qs_edge_dn14 - locals.var_qdeff_edge_dn14))) } } else { (assign33020_e43721 * (p.p1064 * ((locals.var_qs_edge_dn14 - locals.var_qdeff_edge_dn14) / assign33020_e43719))) }),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign33020_e43725;
        locals.var_t8_dn0 = assign33020_e43725_d_n0;
        locals.var_t8_dn2 = assign33020_e43725_d_n2;
        locals.var_t8_dn3 = assign33020_e43725_d_n3;
        locals.var_t8_dn4 = assign33020_e43725_d_n4;
        locals.var_t8_dn5 = assign33020_e43725_d_n5;
        locals.var_t8_dn6 = assign33020_e43725_d_n6;
        locals.var_t8_dn7 = assign33020_e43725_d_n7;
        locals.var_t8_dn8 = assign33020_e43725_d_n8;
        locals.var_t8_dn9 = assign33020_e43725_d_n9;
        locals.var_t8_dn10 = assign33020_e43725_d_n10;
        locals.var_t8_dn11 = assign33020_e43725_d_n11;
        locals.var_t8_dn12 = assign33020_e43725_d_n12;
        locals.var_t8_dn13 = assign33020_e43725_d_n13;
        locals.var_t8_dn14 = assign33020_e43725_d_n14;
        locals.var_t8_rv = 0.0;

        let assign33060_e43756: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard746 = assign33060_e43756;
        locals.var_guard746_rv = 0.0;

        let (assign33070_e43764, assign33070_e43764_d_n0, assign33070_e43764_d_n2, assign33070_e43764_d_n3, assign33070_e43764_d_n4, assign33070_e43764_d_n5, assign33070_e43764_d_n6, assign33070_e43764_d_n7, assign33070_e43764_d_n8, assign33070_e43764_d_n9, assign33070_e43764_d_n10, assign33070_e43764_d_n11, assign33070_e43764_d_n12, assign33070_e43764_d_n13, assign33070_e43764_d_n14,) = {
    if (locals.var_guard746 != 0.0) {
        let assign33070_e43760: f64 = (locals.var_devsign * p.p29);
        let assign33070_e43762: f64 = (assign33070_e43760 * locals.var_qsi);
        (assign33070_e43762, (assign33070_e43760 * locals.var_qsi_dn0), (assign33070_e43760 * locals.var_qsi_dn2), (assign33070_e43760 * locals.var_qsi_dn3), (assign33070_e43760 * locals.var_qsi_dn4), (assign33070_e43760 * locals.var_qsi_dn5), (assign33070_e43760 * locals.var_qsi_dn6), (assign33070_e43760 * locals.var_qsi_dn7), (assign33070_e43760 * locals.var_qsi_dn8), (assign33070_e43760 * locals.var_qsi_dn9), (assign33070_e43760 * locals.var_qsi_dn10), (assign33070_e43760 * locals.var_qsi_dn11), (assign33070_e43760 * locals.var_qsi_dn12), (assign33070_e43760 * locals.var_qsi_dn13), (assign33070_e43760 * locals.var_qsi_dn14),)
    } else {
        (locals.var_qsi_1, locals.var_qsi_1_dn0, locals.var_qsi_1_dn2, locals.var_qsi_1_dn3, locals.var_qsi_1_dn4, locals.var_qsi_1_dn5, locals.var_qsi_1_dn6, locals.var_qsi_1_dn7, locals.var_qsi_1_dn8, locals.var_qsi_1_dn9, locals.var_qsi_1_dn10, locals.var_qsi_1_dn11, locals.var_qsi_1_dn12, locals.var_qsi_1_dn13, locals.var_qsi_1_dn14,)
    }
};
        locals.var_qsi_1 = assign33070_e43764;
        locals.var_qsi_1_dn0 = assign33070_e43764_d_n0;
        locals.var_qsi_1_dn2 = assign33070_e43764_d_n2;
        locals.var_qsi_1_dn3 = assign33070_e43764_d_n3;
        locals.var_qsi_1_dn4 = assign33070_e43764_d_n4;
        locals.var_qsi_1_dn5 = assign33070_e43764_d_n5;
        locals.var_qsi_1_dn6 = assign33070_e43764_d_n6;
        locals.var_qsi_1_dn7 = assign33070_e43764_d_n7;
        locals.var_qsi_1_dn8 = assign33070_e43764_d_n8;
        locals.var_qsi_1_dn9 = assign33070_e43764_d_n9;
        locals.var_qsi_1_dn10 = assign33070_e43764_d_n10;
        locals.var_qsi_1_dn11 = assign33070_e43764_d_n11;
        locals.var_qsi_1_dn12 = assign33070_e43764_d_n12;
        locals.var_qsi_1_dn13 = assign33070_e43764_d_n13;
        locals.var_qsi_1_dn14 = assign33070_e43764_d_n14;
        locals.var_qsi_1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_108(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (assign33080_e43772, assign33080_e43772_d_n0, assign33080_e43772_d_n2, assign33080_e43772_d_n3, assign33080_e43772_d_n4, assign33080_e43772_d_n5, assign33080_e43772_d_n6, assign33080_e43772_d_n7, assign33080_e43772_d_n8, assign33080_e43772_d_n9, assign33080_e43772_d_n10, assign33080_e43772_d_n11, assign33080_e43772_d_n12, assign33080_e43772_d_n13, assign33080_e43772_d_n14,) = {
    if (locals.var_guard746 != 0.0) {
        let assign33080_e43768: f64 = (locals.var_devsign * p.p29);
        let assign33080_e43770: f64 = (assign33080_e43768 * locals.var_qdi);
        (assign33080_e43770, (assign33080_e43768 * locals.var_qdi_dn0), (assign33080_e43768 * locals.var_qdi_dn2), (assign33080_e43768 * locals.var_qdi_dn3), (assign33080_e43768 * locals.var_qdi_dn4), (assign33080_e43768 * locals.var_qdi_dn5), (assign33080_e43768 * locals.var_qdi_dn6), (assign33080_e43768 * locals.var_qdi_dn7), (assign33080_e43768 * locals.var_qdi_dn8), (assign33080_e43768 * locals.var_qdi_dn9), (assign33080_e43768 * locals.var_qdi_dn10), (assign33080_e43768 * locals.var_qdi_dn11), (assign33080_e43768 * locals.var_qdi_dn12), (assign33080_e43768 * locals.var_qdi_dn13), (assign33080_e43768 * locals.var_qdi_dn14),)
    } else {
        (locals.var_qdi_1, locals.var_qdi_1_dn0, locals.var_qdi_1_dn2, locals.var_qdi_1_dn3, locals.var_qdi_1_dn4, locals.var_qdi_1_dn5, locals.var_qdi_1_dn6, locals.var_qdi_1_dn7, locals.var_qdi_1_dn8, locals.var_qdi_1_dn9, locals.var_qdi_1_dn10, locals.var_qdi_1_dn11, locals.var_qdi_1_dn12, locals.var_qdi_1_dn13, locals.var_qdi_1_dn14,)
    }
};
        locals.var_qdi_1 = assign33080_e43772;
        locals.var_qdi_1_dn0 = assign33080_e43772_d_n0;
        locals.var_qdi_1_dn2 = assign33080_e43772_d_n2;
        locals.var_qdi_1_dn3 = assign33080_e43772_d_n3;
        locals.var_qdi_1_dn4 = assign33080_e43772_d_n4;
        locals.var_qdi_1_dn5 = assign33080_e43772_d_n5;
        locals.var_qdi_1_dn6 = assign33080_e43772_d_n6;
        locals.var_qdi_1_dn7 = assign33080_e43772_d_n7;
        locals.var_qdi_1_dn8 = assign33080_e43772_d_n8;
        locals.var_qdi_1_dn9 = assign33080_e43772_d_n9;
        locals.var_qdi_1_dn10 = assign33080_e43772_d_n10;
        locals.var_qdi_1_dn11 = assign33080_e43772_d_n11;
        locals.var_qdi_1_dn12 = assign33080_e43772_d_n12;
        locals.var_qdi_1_dn13 = assign33080_e43772_d_n13;
        locals.var_qdi_1_dn14 = assign33080_e43772_d_n14;
        locals.var_qdi_1_rv = 0.0;

        let (assign33110_e43807, assign33110_e43807_d_n0, assign33110_e43807_d_n2, assign33110_e43807_d_n3, assign33110_e43807_d_n4, assign33110_e43807_d_n5, assign33110_e43807_d_n6, assign33110_e43807_d_n7, assign33110_e43807_d_n8, assign33110_e43807_d_n9, assign33110_e43807_d_n10, assign33110_e43807_d_n11, assign33110_e43807_d_n12, assign33110_e43807_d_n13, assign33110_e43807_d_n14,) = {
    if (locals.var_guard746 == 0.0) {
        let assign33110_e43803: f64 = (locals.var_devsign * p.p29);
        let assign33110_e43805: f64 = (assign33110_e43803 * locals.var_qdi);
        (assign33110_e43805, (assign33110_e43803 * locals.var_qdi_dn0), (assign33110_e43803 * locals.var_qdi_dn2), (assign33110_e43803 * locals.var_qdi_dn3), (assign33110_e43803 * locals.var_qdi_dn4), (assign33110_e43803 * locals.var_qdi_dn5), (assign33110_e43803 * locals.var_qdi_dn6), (assign33110_e43803 * locals.var_qdi_dn7), (assign33110_e43803 * locals.var_qdi_dn8), (assign33110_e43803 * locals.var_qdi_dn9), (assign33110_e43803 * locals.var_qdi_dn10), (assign33110_e43803 * locals.var_qdi_dn11), (assign33110_e43803 * locals.var_qdi_dn12), (assign33110_e43803 * locals.var_qdi_dn13), (assign33110_e43803 * locals.var_qdi_dn14),)
    } else {
        (locals.var_qsi_1, locals.var_qsi_1_dn0, locals.var_qsi_1_dn2, locals.var_qsi_1_dn3, locals.var_qsi_1_dn4, locals.var_qsi_1_dn5, locals.var_qsi_1_dn6, locals.var_qsi_1_dn7, locals.var_qsi_1_dn8, locals.var_qsi_1_dn9, locals.var_qsi_1_dn10, locals.var_qsi_1_dn11, locals.var_qsi_1_dn12, locals.var_qsi_1_dn13, locals.var_qsi_1_dn14,)
    }
};
        locals.var_qsi_1 = assign33110_e43807;
        locals.var_qsi_1_dn0 = assign33110_e43807_d_n0;
        locals.var_qsi_1_dn2 = assign33110_e43807_d_n2;
        locals.var_qsi_1_dn3 = assign33110_e43807_d_n3;
        locals.var_qsi_1_dn4 = assign33110_e43807_d_n4;
        locals.var_qsi_1_dn5 = assign33110_e43807_d_n5;
        locals.var_qsi_1_dn6 = assign33110_e43807_d_n6;
        locals.var_qsi_1_dn7 = assign33110_e43807_d_n7;
        locals.var_qsi_1_dn8 = assign33110_e43807_d_n8;
        locals.var_qsi_1_dn9 = assign33110_e43807_d_n9;
        locals.var_qsi_1_dn10 = assign33110_e43807_d_n10;
        locals.var_qsi_1_dn11 = assign33110_e43807_d_n11;
        locals.var_qsi_1_dn12 = assign33110_e43807_d_n12;
        locals.var_qsi_1_dn13 = assign33110_e43807_d_n13;
        locals.var_qsi_1_dn14 = assign33110_e43807_d_n14;
        locals.var_qsi_1_rv = 0.0;

        let (assign33120_e43816, assign33120_e43816_d_n0, assign33120_e43816_d_n2, assign33120_e43816_d_n3, assign33120_e43816_d_n4, assign33120_e43816_d_n5, assign33120_e43816_d_n6, assign33120_e43816_d_n7, assign33120_e43816_d_n8, assign33120_e43816_d_n9, assign33120_e43816_d_n10, assign33120_e43816_d_n11, assign33120_e43816_d_n12, assign33120_e43816_d_n13, assign33120_e43816_d_n14,) = {
    if (locals.var_guard746 == 0.0) {
        let assign33120_e43812: f64 = (locals.var_devsign * p.p29);
        let assign33120_e43814: f64 = (assign33120_e43812 * locals.var_qsi);
        (assign33120_e43814, (assign33120_e43812 * locals.var_qsi_dn0), (assign33120_e43812 * locals.var_qsi_dn2), (assign33120_e43812 * locals.var_qsi_dn3), (assign33120_e43812 * locals.var_qsi_dn4), (assign33120_e43812 * locals.var_qsi_dn5), (assign33120_e43812 * locals.var_qsi_dn6), (assign33120_e43812 * locals.var_qsi_dn7), (assign33120_e43812 * locals.var_qsi_dn8), (assign33120_e43812 * locals.var_qsi_dn9), (assign33120_e43812 * locals.var_qsi_dn10), (assign33120_e43812 * locals.var_qsi_dn11), (assign33120_e43812 * locals.var_qsi_dn12), (assign33120_e43812 * locals.var_qsi_dn13), (assign33120_e43812 * locals.var_qsi_dn14),)
    } else {
        (locals.var_qdi_1, locals.var_qdi_1_dn0, locals.var_qdi_1_dn2, locals.var_qdi_1_dn3, locals.var_qdi_1_dn4, locals.var_qdi_1_dn5, locals.var_qdi_1_dn6, locals.var_qdi_1_dn7, locals.var_qdi_1_dn8, locals.var_qdi_1_dn9, locals.var_qdi_1_dn10, locals.var_qdi_1_dn11, locals.var_qdi_1_dn12, locals.var_qdi_1_dn13, locals.var_qdi_1_dn14,)
    }
};
        locals.var_qdi_1 = assign33120_e43816;
        locals.var_qdi_1_dn0 = assign33120_e43816_d_n0;
        locals.var_qdi_1_dn2 = assign33120_e43816_d_n2;
        locals.var_qdi_1_dn3 = assign33120_e43816_d_n3;
        locals.var_qdi_1_dn4 = assign33120_e43816_d_n4;
        locals.var_qdi_1_dn5 = assign33120_e43816_d_n5;
        locals.var_qdi_1_dn6 = assign33120_e43816_d_n6;
        locals.var_qdi_1_dn7 = assign33120_e43816_d_n7;
        locals.var_qdi_1_dn8 = assign33120_e43816_d_n8;
        locals.var_qdi_1_dn9 = assign33120_e43816_d_n9;
        locals.var_qdi_1_dn10 = assign33120_e43816_d_n10;
        locals.var_qdi_1_dn11 = assign33120_e43816_d_n11;
        locals.var_qdi_1_dn12 = assign33120_e43816_d_n12;
        locals.var_qdi_1_dn13 = assign33120_e43816_d_n13;
        locals.var_qdi_1_dn14 = assign33120_e43816_d_n14;
        locals.var_qdi_1_rv = 0.0;

        let assign33160_e43858: f64 = if ((p.p1094 == 1.0) && (p.p1095 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard747 = assign33160_e43858;
        locals.var_guard747_rv = 0.0;

        let (assign33170_e43864, assign33170_e43864_d_n0, assign33170_e43864_d_n2, assign33170_e43864_d_n3, assign33170_e43864_d_n4, assign33170_e43864_d_n5, assign33170_e43864_d_n6, assign33170_e43864_d_n7, assign33170_e43864_d_n8, assign33170_e43864_d_n9, assign33170_e43864_d_n10, assign33170_e43864_d_n11, assign33170_e43864_d_n12, assign33170_e43864_d_n13, assign33170_e43864_d_n14,) = {
    if (locals.var_guard747 != 0.0) {
        let assign33170_e43862: f64 = (locals.var_qovb + locals.var_qiov);
        (assign33170_e43862, (locals.var_qovb_dn0 + locals.var_qiov_dn0), (locals.var_qovb_dn2 + locals.var_qiov_dn2), (locals.var_qovb_dn3 + locals.var_qiov_dn3), (locals.var_qovb_dn4 + locals.var_qiov_dn4), (locals.var_qovb_dn5 + locals.var_qiov_dn5), (locals.var_qovb_dn6 + locals.var_qiov_dn6), (locals.var_qovb_dn7 + locals.var_qiov_dn7), (locals.var_qovb_dn8 + locals.var_qiov_dn8), (locals.var_qovb_dn9 + locals.var_qiov_dn9), (locals.var_qovb_dn10 + locals.var_qiov_dn10), (locals.var_qovb_dn11 + locals.var_qiov_dn11), (locals.var_qovb_dn12 + locals.var_qiov_dn12), (locals.var_qovb_dn13 + locals.var_qiov_dn13), (locals.var_qovb_dn14 + locals.var_qiov_dn14),)
    } else {
        (locals.var_qovb, locals.var_qovb_dn0, locals.var_qovb_dn2, locals.var_qovb_dn3, locals.var_qovb_dn4, locals.var_qovb_dn5, locals.var_qovb_dn6, locals.var_qovb_dn7, locals.var_qovb_dn8, locals.var_qovb_dn9, locals.var_qovb_dn10, locals.var_qovb_dn11, locals.var_qovb_dn12, locals.var_qovb_dn13, locals.var_qovb_dn14,)
    }
};
        locals.var_qovb = assign33170_e43864;
        locals.var_qovb_dn0 = assign33170_e43864_d_n0;
        locals.var_qovb_dn2 = assign33170_e43864_d_n2;
        locals.var_qovb_dn3 = assign33170_e43864_d_n3;
        locals.var_qovb_dn4 = assign33170_e43864_d_n4;
        locals.var_qovb_dn5 = assign33170_e43864_d_n5;
        locals.var_qovb_dn6 = assign33170_e43864_d_n6;
        locals.var_qovb_dn7 = assign33170_e43864_d_n7;
        locals.var_qovb_dn8 = assign33170_e43864_d_n8;
        locals.var_qovb_dn9 = assign33170_e43864_d_n9;
        locals.var_qovb_dn10 = assign33170_e43864_d_n10;
        locals.var_qovb_dn11 = assign33170_e43864_d_n11;
        locals.var_qovb_dn12 = assign33170_e43864_d_n12;
        locals.var_qovb_dn13 = assign33170_e43864_d_n13;
        locals.var_qovb_dn14 = assign33170_e43864_d_n14;
        locals.var_qovb_rv = 0.0;

        let (assign33180_e43870, assign33180_e43870_d_n0, assign33180_e43870_d_n2, assign33180_e43870_d_n3, assign33180_e43870_d_n4, assign33180_e43870_d_n5, assign33180_e43870_d_n6, assign33180_e43870_d_n7, assign33180_e43870_d_n8, assign33180_e43870_d_n9, assign33180_e43870_d_n10, assign33180_e43870_d_n11, assign33180_e43870_d_n12, assign33180_e43870_d_n13, assign33180_e43870_d_n14,) = {
    if (locals.var_guard747 != 0.0) {
        let assign33180_e43868: f64 = (locals.var_qovd + locals.var_qbov);
        (assign33180_e43868, (locals.var_qovd_dn0 + locals.var_qbov_dn0), (locals.var_qovd_dn2 + locals.var_qbov_dn2), (locals.var_qovd_dn3 + locals.var_qbov_dn3), (locals.var_qovd_dn4 + locals.var_qbov_dn4), (locals.var_qovd_dn5 + locals.var_qbov_dn5), (locals.var_qovd_dn6 + locals.var_qbov_dn6), (locals.var_qovd_dn7 + locals.var_qbov_dn7), (locals.var_qovd_dn8 + locals.var_qbov_dn8), (locals.var_qovd_dn9 + locals.var_qbov_dn9), (locals.var_qovd_dn10 + locals.var_qbov_dn10), (locals.var_qovd_dn11 + locals.var_qbov_dn11), (locals.var_qovd_dn12 + locals.var_qbov_dn12), (locals.var_qovd_dn13 + locals.var_qbov_dn13), (locals.var_qovd_dn14 + locals.var_qbov_dn14),)
    } else {
        (locals.var_qovd, locals.var_qovd_dn0, locals.var_qovd_dn2, locals.var_qovd_dn3, locals.var_qovd_dn4, locals.var_qovd_dn5, locals.var_qovd_dn6, locals.var_qovd_dn7, locals.var_qovd_dn8, locals.var_qovd_dn9, locals.var_qovd_dn10, locals.var_qovd_dn11, locals.var_qovd_dn12, locals.var_qovd_dn13, locals.var_qovd_dn14,)
    }
};
        locals.var_qovd = assign33180_e43870;
        locals.var_qovd_dn0 = assign33180_e43870_d_n0;
        locals.var_qovd_dn2 = assign33180_e43870_d_n2;
        locals.var_qovd_dn3 = assign33180_e43870_d_n3;
        locals.var_qovd_dn4 = assign33180_e43870_d_n4;
        locals.var_qovd_dn5 = assign33180_e43870_d_n5;
        locals.var_qovd_dn6 = assign33180_e43870_d_n6;
        locals.var_qovd_dn7 = assign33180_e43870_d_n7;
        locals.var_qovd_dn8 = assign33180_e43870_d_n8;
        locals.var_qovd_dn9 = assign33180_e43870_d_n9;
        locals.var_qovd_dn10 = assign33180_e43870_d_n10;
        locals.var_qovd_dn11 = assign33180_e43870_d_n11;
        locals.var_qovd_dn12 = assign33180_e43870_d_n12;
        locals.var_qovd_dn13 = assign33180_e43870_d_n13;
        locals.var_qovd_dn14 = assign33180_e43870_d_n14;
        locals.var_qovd_rv = 0.0;

        let assign33190_e43873: f64 = if p.p1096 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard748 = assign33190_e43873;
        locals.var_guard748_rv = 0.0;

        let (assign33200_e43881, assign33200_e43881_d_n0, assign33200_e43881_d_n2, assign33200_e43881_d_n3, assign33200_e43881_d_n4, assign33200_e43881_d_n5, assign33200_e43881_d_n6, assign33200_e43881_d_n7, assign33200_e43881_d_n8, assign33200_e43881_d_n9, assign33200_e43881_d_n10, assign33200_e43881_d_n11, assign33200_e43881_d_n12, assign33200_e43881_d_n13, assign33200_e43881_d_n14,) = {
    if ((locals.var_guard747 != 0.0) && (locals.var_guard748 != 0.0)) {
        let assign33200_e43879: f64 = (locals.var_qovb + locals.var_qiovs);
        (assign33200_e43879, (locals.var_qovb_dn0 + locals.var_qiovs_dn0), (locals.var_qovb_dn2 + locals.var_qiovs_dn2), (locals.var_qovb_dn3 + locals.var_qiovs_dn3), (locals.var_qovb_dn4 + locals.var_qiovs_dn4), (locals.var_qovb_dn5 + locals.var_qiovs_dn5), (locals.var_qovb_dn6 + locals.var_qiovs_dn6), (locals.var_qovb_dn7 + locals.var_qiovs_dn7), (locals.var_qovb_dn8 + locals.var_qiovs_dn8), (locals.var_qovb_dn9 + locals.var_qiovs_dn9), (locals.var_qovb_dn10 + locals.var_qiovs_dn10), (locals.var_qovb_dn11 + locals.var_qiovs_dn11), (locals.var_qovb_dn12 + locals.var_qiovs_dn12), (locals.var_qovb_dn13 + locals.var_qiovs_dn13), (locals.var_qovb_dn14 + locals.var_qiovs_dn14),)
    } else {
        (locals.var_qovb, locals.var_qovb_dn0, locals.var_qovb_dn2, locals.var_qovb_dn3, locals.var_qovb_dn4, locals.var_qovb_dn5, locals.var_qovb_dn6, locals.var_qovb_dn7, locals.var_qovb_dn8, locals.var_qovb_dn9, locals.var_qovb_dn10, locals.var_qovb_dn11, locals.var_qovb_dn12, locals.var_qovb_dn13, locals.var_qovb_dn14,)
    }
};
        locals.var_qovb = assign33200_e43881;
        locals.var_qovb_dn0 = assign33200_e43881_d_n0;
        locals.var_qovb_dn2 = assign33200_e43881_d_n2;
        locals.var_qovb_dn3 = assign33200_e43881_d_n3;
        locals.var_qovb_dn4 = assign33200_e43881_d_n4;
        locals.var_qovb_dn5 = assign33200_e43881_d_n5;
        locals.var_qovb_dn6 = assign33200_e43881_d_n6;
        locals.var_qovb_dn7 = assign33200_e43881_d_n7;
        locals.var_qovb_dn8 = assign33200_e43881_d_n8;
        locals.var_qovb_dn9 = assign33200_e43881_d_n9;
        locals.var_qovb_dn10 = assign33200_e43881_d_n10;
        locals.var_qovb_dn11 = assign33200_e43881_d_n11;
        locals.var_qovb_dn12 = assign33200_e43881_d_n12;
        locals.var_qovb_dn13 = assign33200_e43881_d_n13;
        locals.var_qovb_dn14 = assign33200_e43881_d_n14;
        locals.var_qovb_rv = 0.0;

        let (assign33210_e43889, assign33210_e43889_d_n0, assign33210_e43889_d_n2, assign33210_e43889_d_n3, assign33210_e43889_d_n4, assign33210_e43889_d_n5, assign33210_e43889_d_n6, assign33210_e43889_d_n7, assign33210_e43889_d_n8, assign33210_e43889_d_n9, assign33210_e43889_d_n10, assign33210_e43889_d_n11, assign33210_e43889_d_n12, assign33210_e43889_d_n13, assign33210_e43889_d_n14,) = {
    if ((locals.var_guard747 != 0.0) && (locals.var_guard748 != 0.0)) {
        let assign33210_e43887: f64 = (locals.var_qovs + locals.var_qbovs);
        (assign33210_e43887, (locals.var_qovs_dn0 + locals.var_qbovs_dn0), (locals.var_qovs_dn2 + locals.var_qbovs_dn2), (locals.var_qovs_dn3 + locals.var_qbovs_dn3), (locals.var_qovs_dn4 + locals.var_qbovs_dn4), (locals.var_qovs_dn5 + locals.var_qbovs_dn5), (locals.var_qovs_dn6 + locals.var_qbovs_dn6), (locals.var_qovs_dn7 + locals.var_qbovs_dn7), (locals.var_qovs_dn8 + locals.var_qbovs_dn8), (locals.var_qovs_dn9 + locals.var_qbovs_dn9), (locals.var_qovs_dn10 + locals.var_qbovs_dn10), (locals.var_qovs_dn11 + locals.var_qbovs_dn11), (locals.var_qovs_dn12 + locals.var_qbovs_dn12), (locals.var_qovs_dn13 + locals.var_qbovs_dn13), (locals.var_qovs_dn14 + locals.var_qbovs_dn14),)
    } else {
        (locals.var_qovs, locals.var_qovs_dn0, locals.var_qovs_dn2, locals.var_qovs_dn3, locals.var_qovs_dn4, locals.var_qovs_dn5, locals.var_qovs_dn6, locals.var_qovs_dn7, locals.var_qovs_dn8, locals.var_qovs_dn9, locals.var_qovs_dn10, locals.var_qovs_dn11, locals.var_qovs_dn12, locals.var_qovs_dn13, locals.var_qovs_dn14,)
    }
};
        locals.var_qovs = assign33210_e43889;
        locals.var_qovs_dn0 = assign33210_e43889_d_n0;
        locals.var_qovs_dn2 = assign33210_e43889_d_n2;
        locals.var_qovs_dn3 = assign33210_e43889_d_n3;
        locals.var_qovs_dn4 = assign33210_e43889_d_n4;
        locals.var_qovs_dn5 = assign33210_e43889_d_n5;
        locals.var_qovs_dn6 = assign33210_e43889_d_n6;
        locals.var_qovs_dn7 = assign33210_e43889_d_n7;
        locals.var_qovs_dn8 = assign33210_e43889_d_n8;
        locals.var_qovs_dn9 = assign33210_e43889_d_n9;
        locals.var_qovs_dn10 = assign33210_e43889_d_n10;
        locals.var_qovs_dn11 = assign33210_e43889_d_n11;
        locals.var_qovs_dn12 = assign33210_e43889_d_n12;
        locals.var_qovs_dn13 = assign33210_e43889_d_n13;
        locals.var_qovs_dn14 = assign33210_e43889_d_n14;
        locals.var_qovs_rv = 0.0;

        let assign33230_e43897: f64 = (locals.var_devsign * p.p29);
        let assign33230_e43899: f64 = (assign33230_e43897 * locals.var_qgi);
        locals.var_qgi_1 = assign33230_e43899;
        locals.var_qgi_1_dn0 = (assign33230_e43897 * locals.var_qgi_dn0);
        locals.var_qgi_1_dn2 = (assign33230_e43897 * locals.var_qgi_dn2);
        locals.var_qgi_1_dn3 = (assign33230_e43897 * locals.var_qgi_dn3);
        locals.var_qgi_1_dn4 = (assign33230_e43897 * locals.var_qgi_dn4);
        locals.var_qgi_1_dn5 = (assign33230_e43897 * locals.var_qgi_dn5);
        locals.var_qgi_1_dn6 = (assign33230_e43897 * locals.var_qgi_dn6);
        locals.var_qgi_1_dn7 = (assign33230_e43897 * locals.var_qgi_dn7);
        locals.var_qgi_1_dn8 = (assign33230_e43897 * locals.var_qgi_dn8);
        locals.var_qgi_1_dn9 = (assign33230_e43897 * locals.var_qgi_dn9);
        locals.var_qgi_1_dn10 = (assign33230_e43897 * locals.var_qgi_dn10);
        locals.var_qgi_1_dn11 = (assign33230_e43897 * locals.var_qgi_dn11);
        locals.var_qgi_1_dn12 = (assign33230_e43897 * locals.var_qgi_dn12);
        locals.var_qgi_1_dn13 = (assign33230_e43897 * locals.var_qgi_dn13);
        locals.var_qgi_1_dn14 = (assign33230_e43897 * locals.var_qgi_dn14);
        locals.var_qgi_1_rv = 0.0;

        let assign33870_e44253: f64 = if ((p.p42 != 2.0) && (locals.var_rdraingeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard754 = assign33870_e44253;
        locals.var_guard754_rv = 0.0;

        let (assign33880_e44259, assign33880_e44259_d_n0, assign33880_e44259_d_n2, assign33880_e44259_d_n3, assign33880_e44259_d_n4, assign33880_e44259_d_n5, assign33880_e44259_d_n6, assign33880_e44259_d_n7, assign33880_e44259_d_n8, assign33880_e44259_d_n9, assign33880_e44259_d_n10, assign33880_e44259_d_n11, assign33880_e44259_d_n12, assign33880_e44259_d_n13, assign33880_e44259_d_n14,) = {
    if (locals.var_guard754 != 0.0) {
        let assign33880_e44257: f64 = (1.0 / locals.var_rdrain);
        (assign33880_e44257, (-(locals.var_rdrain_dn0 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn2 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn3 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn4 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn5 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn6 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn7 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn8 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn9 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn10 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn11 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn12 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn13 / (locals.var_rdrain * locals.var_rdrain))), (-(locals.var_rdrain_dn14 / (locals.var_rdrain * locals.var_rdrain))),)
    } else {
        (locals.var_gdpr, locals.var_gdpr_dn0, locals.var_gdpr_dn2, locals.var_gdpr_dn3, locals.var_gdpr_dn4, locals.var_gdpr_dn5, locals.var_gdpr_dn6, locals.var_gdpr_dn7, locals.var_gdpr_dn8, locals.var_gdpr_dn9, locals.var_gdpr_dn10, locals.var_gdpr_dn11, locals.var_gdpr_dn12, locals.var_gdpr_dn13, locals.var_gdpr_dn14,)
    }
};
        locals.var_gdpr = assign33880_e44259;
        locals.var_gdpr_dn0 = assign33880_e44259_d_n0;
        locals.var_gdpr_dn2 = assign33880_e44259_d_n2;
        locals.var_gdpr_dn3 = assign33880_e44259_d_n3;
        locals.var_gdpr_dn4 = assign33880_e44259_d_n4;
        locals.var_gdpr_dn5 = assign33880_e44259_d_n5;
        locals.var_gdpr_dn6 = assign33880_e44259_d_n6;
        locals.var_gdpr_dn7 = assign33880_e44259_d_n7;
        locals.var_gdpr_dn8 = assign33880_e44259_d_n8;
        locals.var_gdpr_dn9 = assign33880_e44259_d_n9;
        locals.var_gdpr_dn10 = assign33880_e44259_d_n10;
        locals.var_gdpr_dn11 = assign33880_e44259_d_n11;
        locals.var_gdpr_dn12 = assign33880_e44259_d_n12;
        locals.var_gdpr_dn13 = assign33880_e44259_d_n13;
        locals.var_gdpr_dn14 = assign33880_e44259_d_n14;
        locals.var_gdpr_rv = 0.0;

        let assign33890_e44270: f64 = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1110 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard755 = assign33890_e44270;
        locals.var_guard755_rv = 0.0;

        let (assign33900_e44278, assign33900_e44278_d_n0, assign33900_e44278_d_n2, assign33900_e44278_d_n3, assign33900_e44278_d_n4, assign33900_e44278_d_n5, assign33900_e44278_d_n6, assign33900_e44278_d_n7, assign33900_e44278_d_n8, assign33900_e44278_d_n9, assign33900_e44278_d_n10, assign33900_e44278_d_n11, assign33900_e44278_d_n12, assign33900_e44278_d_n13, assign33900_e44278_d_n14,) = {
    if ((locals.var_guard754 != 0.0) && (locals.var_guard755 != 0.0)) {
        let assign33900_e44276: f64 = (1.0 / locals.var_rdrift_d);
        (assign33900_e44276, (-(locals.var_rdrift_d_dn0 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn2 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn3 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn4 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn5 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn6 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn7 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn8 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn9 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn10 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn11 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn12 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn13 / (locals.var_rdrift_d * locals.var_rdrift_d))), (-(locals.var_rdrift_d_dn14 / (locals.var_rdrift_d * locals.var_rdrift_d))),)
    } else {
        (locals.var_gdrift_d, locals.var_gdrift_d_dn0, locals.var_gdrift_d_dn2, locals.var_gdrift_d_dn3, locals.var_gdrift_d_dn4, locals.var_gdrift_d_dn5, locals.var_gdrift_d_dn6, locals.var_gdrift_d_dn7, locals.var_gdrift_d_dn8, locals.var_gdrift_d_dn9, locals.var_gdrift_d_dn10, locals.var_gdrift_d_dn11, locals.var_gdrift_d_dn12, locals.var_gdrift_d_dn13, locals.var_gdrift_d_dn14,)
    }
};
        locals.var_gdrift_d = assign33900_e44278;
        locals.var_gdrift_d_dn0 = assign33900_e44278_d_n0;
        locals.var_gdrift_d_dn2 = assign33900_e44278_d_n2;
        locals.var_gdrift_d_dn3 = assign33900_e44278_d_n3;
        locals.var_gdrift_d_dn4 = assign33900_e44278_d_n4;
        locals.var_gdrift_d_dn5 = assign33900_e44278_d_n5;
        locals.var_gdrift_d_dn6 = assign33900_e44278_d_n6;
        locals.var_gdrift_d_dn7 = assign33900_e44278_d_n7;
        locals.var_gdrift_d_dn8 = assign33900_e44278_d_n8;
        locals.var_gdrift_d_dn9 = assign33900_e44278_d_n9;
        locals.var_gdrift_d_dn10 = assign33900_e44278_d_n10;
        locals.var_gdrift_d_dn11 = assign33900_e44278_d_n11;
        locals.var_gdrift_d_dn12 = assign33900_e44278_d_n12;
        locals.var_gdrift_d_dn13 = assign33900_e44278_d_n13;
        locals.var_gdrift_d_dn14 = assign33900_e44278_d_n14;
        locals.var_gdrift_d_rv = 0.0;

        let assign33910_e44285: f64 = if ((p.p42 != 2.0) && (locals.var_rsourcegeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard756 = assign33910_e44285;
        locals.var_guard756_rv = 0.0;

        let (assign33920_e44291, assign33920_e44291_d_n0, assign33920_e44291_d_n2, assign33920_e44291_d_n3, assign33920_e44291_d_n4, assign33920_e44291_d_n5, assign33920_e44291_d_n6, assign33920_e44291_d_n7, assign33920_e44291_d_n8, assign33920_e44291_d_n9, assign33920_e44291_d_n10, assign33920_e44291_d_n11, assign33920_e44291_d_n12, assign33920_e44291_d_n13, assign33920_e44291_d_n14,) = {
    if (locals.var_guard756 != 0.0) {
        let assign33920_e44289: f64 = (1.0 / locals.var_rsource);
        (assign33920_e44289, (-(locals.var_rsource_dn0 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn2 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn3 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn4 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn5 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn6 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn7 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn8 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn9 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn10 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn11 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn12 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn13 / (locals.var_rsource * locals.var_rsource))), (-(locals.var_rsource_dn14 / (locals.var_rsource * locals.var_rsource))),)
    } else {
        (locals.var_gspr, locals.var_gspr_dn0, locals.var_gspr_dn2, locals.var_gspr_dn3, locals.var_gspr_dn4, locals.var_gspr_dn5, locals.var_gspr_dn6, locals.var_gspr_dn7, locals.var_gspr_dn8, locals.var_gspr_dn9, locals.var_gspr_dn10, locals.var_gspr_dn11, locals.var_gspr_dn12, locals.var_gspr_dn13, locals.var_gspr_dn14,)
    }
};
        locals.var_gspr = assign33920_e44291;
        locals.var_gspr_dn0 = assign33920_e44291_d_n0;
        locals.var_gspr_dn2 = assign33920_e44291_d_n2;
        locals.var_gspr_dn3 = assign33920_e44291_d_n3;
        locals.var_gspr_dn4 = assign33920_e44291_d_n4;
        locals.var_gspr_dn5 = assign33920_e44291_d_n5;
        locals.var_gspr_dn6 = assign33920_e44291_d_n6;
        locals.var_gspr_dn7 = assign33920_e44291_d_n7;
        locals.var_gspr_dn8 = assign33920_e44291_d_n8;
        locals.var_gspr_dn9 = assign33920_e44291_d_n9;
        locals.var_gspr_dn10 = assign33920_e44291_d_n10;
        locals.var_gspr_dn11 = assign33920_e44291_d_n11;
        locals.var_gspr_dn12 = assign33920_e44291_d_n12;
        locals.var_gspr_dn13 = assign33920_e44291_d_n13;
        locals.var_gspr_dn14 = assign33920_e44291_d_n14;
        locals.var_gspr_rv = 0.0;

        let assign33930_e44302: f64 = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1112 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard757 = assign33930_e44302;
        locals.var_guard757_rv = 0.0;

        let (assign33940_e44310, assign33940_e44310_d_n0, assign33940_e44310_d_n2, assign33940_e44310_d_n3, assign33940_e44310_d_n4, assign33940_e44310_d_n5, assign33940_e44310_d_n6, assign33940_e44310_d_n7, assign33940_e44310_d_n8, assign33940_e44310_d_n9, assign33940_e44310_d_n10, assign33940_e44310_d_n11, assign33940_e44310_d_n12, assign33940_e44310_d_n13, assign33940_e44310_d_n14,) = {
    if ((locals.var_guard756 != 0.0) && (locals.var_guard757 != 0.0)) {
        let assign33940_e44308: f64 = (1.0 / locals.var_rdrift_s);
        (assign33940_e44308, (-(locals.var_rdrift_s_dn0 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn2 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn3 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn4 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn5 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn6 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn7 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn8 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn9 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn10 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn11 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn12 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn13 / (locals.var_rdrift_s * locals.var_rdrift_s))), (-(locals.var_rdrift_s_dn14 / (locals.var_rdrift_s * locals.var_rdrift_s))),)
    } else {
        (locals.var_gdrift_s, locals.var_gdrift_s_dn0, locals.var_gdrift_s_dn2, locals.var_gdrift_s_dn3, locals.var_gdrift_s_dn4, locals.var_gdrift_s_dn5, locals.var_gdrift_s_dn6, locals.var_gdrift_s_dn7, locals.var_gdrift_s_dn8, locals.var_gdrift_s_dn9, locals.var_gdrift_s_dn10, locals.var_gdrift_s_dn11, locals.var_gdrift_s_dn12, locals.var_gdrift_s_dn13, locals.var_gdrift_s_dn14,)
    }
};
        locals.var_gdrift_s = assign33940_e44310;
        locals.var_gdrift_s_dn0 = assign33940_e44310_d_n0;
        locals.var_gdrift_s_dn2 = assign33940_e44310_d_n2;
        locals.var_gdrift_s_dn3 = assign33940_e44310_d_n3;
        locals.var_gdrift_s_dn4 = assign33940_e44310_d_n4;
        locals.var_gdrift_s_dn5 = assign33940_e44310_d_n5;
        locals.var_gdrift_s_dn6 = assign33940_e44310_d_n6;
        locals.var_gdrift_s_dn7 = assign33940_e44310_d_n7;
        locals.var_gdrift_s_dn8 = assign33940_e44310_d_n8;
        locals.var_gdrift_s_dn9 = assign33940_e44310_d_n9;
        locals.var_gdrift_s_dn10 = assign33940_e44310_d_n10;
        locals.var_gdrift_s_dn11 = assign33940_e44310_d_n11;
        locals.var_gdrift_s_dn12 = assign33940_e44310_d_n12;
        locals.var_gdrift_s_dn13 = assign33940_e44310_d_n13;
        locals.var_gdrift_s_dn14 = assign33940_e44310_d_n14;
        locals.var_gdrift_s_rv = 0.0;

        let assign34020_e44360: f64 = if ((p.p49 != 0.0) && (p.p909 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard763 = assign34020_e44360;
        locals.var_guard763_rv = 0.0;

        let (assign34030_e44370, assign34030_e44370_d_n0, assign34030_e44370_d_n2, assign34030_e44370_d_n3, assign34030_e44370_d_n4, assign34030_e44370_d_n5, assign34030_e44370_d_n6, assign34030_e44370_d_n7, assign34030_e44370_d_n8, assign34030_e44370_d_n9, assign34030_e44370_d_n10, assign34030_e44370_d_n11, assign34030_e44370_d_n12, assign34030_e44370_d_n13, assign34030_e44370_d_n14,) = {
    if (locals.var_guard763 != 0.0) {
        let assign34030_e44364: f64 = (locals.var_devsign * locals.var_sigvds);
        let assign34030_e44366: f64 = (assign34030_e44364 * locals.var_ids);
        let assign34030_e44368: f64 = (assign34030_e44366 * (nv5 - nv7));
        (assign34030_e44368, ((assign34030_e44364 * locals.var_ids_dn0) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn2) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn3) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn4) * (nv5 - nv7)), (((assign34030_e44364 * locals.var_ids_dn5) * (nv5 - nv7)) + assign34030_e44366), ((assign34030_e44364 * locals.var_ids_dn6) * (nv5 - nv7)), (((assign34030_e44364 * locals.var_ids_dn7) * (nv5 - nv7)) + (-assign34030_e44366)), ((assign34030_e44364 * locals.var_ids_dn8) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn9) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn10) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn11) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn12) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn13) * (nv5 - nv7)), ((assign34030_e44364 * locals.var_ids_dn14) * (nv5 - nv7)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11, locals.var_pdiss_dn12, locals.var_pdiss_dn13, locals.var_pdiss_dn14,)
    }
};
        locals.var_pdiss = assign34030_e44370;
        locals.var_pdiss_dn0 = assign34030_e44370_d_n0;
        locals.var_pdiss_dn2 = assign34030_e44370_d_n2;
        locals.var_pdiss_dn3 = assign34030_e44370_d_n3;
        locals.var_pdiss_dn4 = assign34030_e44370_d_n4;
        locals.var_pdiss_dn5 = assign34030_e44370_d_n5;
        locals.var_pdiss_dn6 = assign34030_e44370_d_n6;
        locals.var_pdiss_dn7 = assign34030_e44370_d_n7;
        locals.var_pdiss_dn8 = assign34030_e44370_d_n8;
        locals.var_pdiss_dn9 = assign34030_e44370_d_n9;
        locals.var_pdiss_dn10 = assign34030_e44370_d_n10;
        locals.var_pdiss_dn11 = assign34030_e44370_d_n11;
        locals.var_pdiss_dn12 = assign34030_e44370_d_n12;
        locals.var_pdiss_dn13 = assign34030_e44370_d_n13;
        locals.var_pdiss_dn14 = assign34030_e44370_d_n14;
        locals.var_pdiss_rv = 0.0;

        let assign34040_e44377: f64 = if ((p.p42 != 2.0) && (locals.var_rdraingeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard764 = assign34040_e44377;
        locals.var_guard764_rv = 0.0;

        let assign34050_e44388: f64 = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1110 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard765 = assign34050_e44388;
        locals.var_guard765_rv = 0.0;

        let (assign34060_e44408, assign34060_e44408_d_n0, assign34060_e44408_d_n2, assign34060_e44408_d_n3, assign34060_e44408_d_n4, assign34060_e44408_d_n5, assign34060_e44408_d_n6, assign34060_e44408_d_n7, assign34060_e44408_d_n8, assign34060_e44408_d_n9, assign34060_e44408_d_n10, assign34060_e44408_d_n11, assign34060_e44408_d_n12, assign34060_e44408_d_n13, assign34060_e44408_d_n14,) = {
    if (((locals.var_guard763 != 0.0) && (locals.var_guard764 != 0.0)) && (locals.var_guard765 != 0.0)) {
        let assign34060_e44397: f64 = ((nv0 - nv6) * (nv0 - nv6));
        let assign34060_e44399: f64 = (assign34060_e44397 * locals.var_gdpr);
        let assign34060_e44400: f64 = (locals.var_pdiss + assign34060_e44399);
        let assign34060_e44403: f64 = ((nv6 - nv5) * (nv6 - nv5));
        let assign34060_e44405: f64 = (assign34060_e44403 * locals.var_gdrift_d);
        let assign34060_e44406: f64 = (assign34060_e44400 + assign34060_e44405);
        (assign34060_e44406, ((locals.var_pdiss_dn0 + ((((nv0 - nv6) + (nv0 - nv6)) * locals.var_gdpr) + (assign34060_e44397 * locals.var_gdpr_dn0))) + (assign34060_e44403 * locals.var_gdrift_d_dn0)), ((locals.var_pdiss_dn2 + (assign34060_e44397 * locals.var_gdpr_dn2)) + (assign34060_e44403 * locals.var_gdrift_d_dn2)), ((locals.var_pdiss_dn3 + (assign34060_e44397 * locals.var_gdpr_dn3)) + (assign34060_e44403 * locals.var_gdrift_d_dn3)), ((locals.var_pdiss_dn4 + (assign34060_e44397 * locals.var_gdpr_dn4)) + (assign34060_e44403 * locals.var_gdrift_d_dn4)), ((locals.var_pdiss_dn5 + (assign34060_e44397 * locals.var_gdpr_dn5)) + ((((-(nv6 - nv5)) + (-(nv6 - nv5))) * locals.var_gdrift_d) + (assign34060_e44403 * locals.var_gdrift_d_dn5))), ((locals.var_pdiss_dn6 + ((((-(nv0 - nv6)) + (-(nv0 - nv6))) * locals.var_gdpr) + (assign34060_e44397 * locals.var_gdpr_dn6))) + ((((nv6 - nv5) + (nv6 - nv5)) * locals.var_gdrift_d) + (assign34060_e44403 * locals.var_gdrift_d_dn6))), ((locals.var_pdiss_dn7 + (assign34060_e44397 * locals.var_gdpr_dn7)) + (assign34060_e44403 * locals.var_gdrift_d_dn7)), ((locals.var_pdiss_dn8 + (assign34060_e44397 * locals.var_gdpr_dn8)) + (assign34060_e44403 * locals.var_gdrift_d_dn8)), ((locals.var_pdiss_dn9 + (assign34060_e44397 * locals.var_gdpr_dn9)) + (assign34060_e44403 * locals.var_gdrift_d_dn9)), ((locals.var_pdiss_dn10 + (assign34060_e44397 * locals.var_gdpr_dn10)) + (assign34060_e44403 * locals.var_gdrift_d_dn10)), ((locals.var_pdiss_dn11 + (assign34060_e44397 * locals.var_gdpr_dn11)) + (assign34060_e44403 * locals.var_gdrift_d_dn11)), ((locals.var_pdiss_dn12 + (assign34060_e44397 * locals.var_gdpr_dn12)) + (assign34060_e44403 * locals.var_gdrift_d_dn12)), ((locals.var_pdiss_dn13 + (assign34060_e44397 * locals.var_gdpr_dn13)) + (assign34060_e44403 * locals.var_gdrift_d_dn13)), ((locals.var_pdiss_dn14 + (assign34060_e44397 * locals.var_gdpr_dn14)) + (assign34060_e44403 * locals.var_gdrift_d_dn14)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11, locals.var_pdiss_dn12, locals.var_pdiss_dn13, locals.var_pdiss_dn14,)
    }
};
        locals.var_pdiss = assign34060_e44408;
        locals.var_pdiss_dn0 = assign34060_e44408_d_n0;
        locals.var_pdiss_dn2 = assign34060_e44408_d_n2;
        locals.var_pdiss_dn3 = assign34060_e44408_d_n3;
        locals.var_pdiss_dn4 = assign34060_e44408_d_n4;
        locals.var_pdiss_dn5 = assign34060_e44408_d_n5;
        locals.var_pdiss_dn6 = assign34060_e44408_d_n6;
        locals.var_pdiss_dn7 = assign34060_e44408_d_n7;
        locals.var_pdiss_dn8 = assign34060_e44408_d_n8;
        locals.var_pdiss_dn9 = assign34060_e44408_d_n9;
        locals.var_pdiss_dn10 = assign34060_e44408_d_n10;
        locals.var_pdiss_dn11 = assign34060_e44408_d_n11;
        locals.var_pdiss_dn12 = assign34060_e44408_d_n12;
        locals.var_pdiss_dn13 = assign34060_e44408_d_n13;
        locals.var_pdiss_dn14 = assign34060_e44408_d_n14;
        locals.var_pdiss_rv = 0.0;

        let (assign34070_e44423, assign34070_e44423_d_n0, assign34070_e44423_d_n2, assign34070_e44423_d_n3, assign34070_e44423_d_n4, assign34070_e44423_d_n5, assign34070_e44423_d_n6, assign34070_e44423_d_n7, assign34070_e44423_d_n8, assign34070_e44423_d_n9, assign34070_e44423_d_n10, assign34070_e44423_d_n11, assign34070_e44423_d_n12, assign34070_e44423_d_n13, assign34070_e44423_d_n14,) = {
    if (((locals.var_guard763 != 0.0) && (locals.var_guard764 != 0.0)) && (locals.var_guard765 == 0.0)) {
        let assign34070_e44418: f64 = ((nv0 - nv6) * (nv0 - nv6));
        let assign34070_e44420: f64 = (assign34070_e44418 * locals.var_gdpr);
        let assign34070_e44421: f64 = (locals.var_pdiss + assign34070_e44420);
        (assign34070_e44421, (locals.var_pdiss_dn0 + ((((nv0 - nv6) + (nv0 - nv6)) * locals.var_gdpr) + (assign34070_e44418 * locals.var_gdpr_dn0))), (locals.var_pdiss_dn2 + (assign34070_e44418 * locals.var_gdpr_dn2)), (locals.var_pdiss_dn3 + (assign34070_e44418 * locals.var_gdpr_dn3)), (locals.var_pdiss_dn4 + (assign34070_e44418 * locals.var_gdpr_dn4)), (locals.var_pdiss_dn5 + (assign34070_e44418 * locals.var_gdpr_dn5)), (locals.var_pdiss_dn6 + ((((-(nv0 - nv6)) + (-(nv0 - nv6))) * locals.var_gdpr) + (assign34070_e44418 * locals.var_gdpr_dn6))), (locals.var_pdiss_dn7 + (assign34070_e44418 * locals.var_gdpr_dn7)), (locals.var_pdiss_dn8 + (assign34070_e44418 * locals.var_gdpr_dn8)), (locals.var_pdiss_dn9 + (assign34070_e44418 * locals.var_gdpr_dn9)), (locals.var_pdiss_dn10 + (assign34070_e44418 * locals.var_gdpr_dn10)), (locals.var_pdiss_dn11 + (assign34070_e44418 * locals.var_gdpr_dn11)), (locals.var_pdiss_dn12 + (assign34070_e44418 * locals.var_gdpr_dn12)), (locals.var_pdiss_dn13 + (assign34070_e44418 * locals.var_gdpr_dn13)), (locals.var_pdiss_dn14 + (assign34070_e44418 * locals.var_gdpr_dn14)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11, locals.var_pdiss_dn12, locals.var_pdiss_dn13, locals.var_pdiss_dn14,)
    }
};
        locals.var_pdiss = assign34070_e44423;
        locals.var_pdiss_dn0 = assign34070_e44423_d_n0;
        locals.var_pdiss_dn2 = assign34070_e44423_d_n2;
        locals.var_pdiss_dn3 = assign34070_e44423_d_n3;
        locals.var_pdiss_dn4 = assign34070_e44423_d_n4;
        locals.var_pdiss_dn5 = assign34070_e44423_d_n5;
        locals.var_pdiss_dn6 = assign34070_e44423_d_n6;
        locals.var_pdiss_dn7 = assign34070_e44423_d_n7;
        locals.var_pdiss_dn8 = assign34070_e44423_d_n8;
        locals.var_pdiss_dn9 = assign34070_e44423_d_n9;
        locals.var_pdiss_dn10 = assign34070_e44423_d_n10;
        locals.var_pdiss_dn11 = assign34070_e44423_d_n11;
        locals.var_pdiss_dn12 = assign34070_e44423_d_n12;
        locals.var_pdiss_dn13 = assign34070_e44423_d_n13;
        locals.var_pdiss_dn14 = assign34070_e44423_d_n14;
        locals.var_pdiss_rv = 0.0;

        let assign34080_e44430: f64 = if ((p.p42 != 2.0) && (locals.var_rsourcegeo > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard766 = assign34080_e44430;
        locals.var_guard766_rv = 0.0;

        let assign34090_e44441: f64 = if (((p.p42 == 1.0) && (p.p1094 == 1.0)) && (p.p1112 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard767 = assign34090_e44441;
        locals.var_guard767_rv = 0.0;

        let (assign34100_e44461, assign34100_e44461_d_n0, assign34100_e44461_d_n2, assign34100_e44461_d_n3, assign34100_e44461_d_n4, assign34100_e44461_d_n5, assign34100_e44461_d_n6, assign34100_e44461_d_n7, assign34100_e44461_d_n8, assign34100_e44461_d_n9, assign34100_e44461_d_n10, assign34100_e44461_d_n11, assign34100_e44461_d_n12, assign34100_e44461_d_n13, assign34100_e44461_d_n14,) = {
    if (((locals.var_guard763 != 0.0) && (locals.var_guard766 != 0.0)) && (locals.var_guard767 != 0.0)) {
        let assign34100_e44450: f64 = ((nv2 - nv8) * (nv2 - nv8));
        let assign34100_e44452: f64 = (assign34100_e44450 * locals.var_gspr);
        let assign34100_e44453: f64 = (locals.var_pdiss + assign34100_e44452);
        let assign34100_e44456: f64 = ((nv8 - nv7) * (nv8 - nv7));
        let assign34100_e44458: f64 = (assign34100_e44456 * locals.var_gdrift_s);
        let assign34100_e44459: f64 = (assign34100_e44453 + assign34100_e44458);
        (assign34100_e44459, ((locals.var_pdiss_dn0 + (assign34100_e44450 * locals.var_gspr_dn0)) + (assign34100_e44456 * locals.var_gdrift_s_dn0)), ((locals.var_pdiss_dn2 + ((((nv2 - nv8) + (nv2 - nv8)) * locals.var_gspr) + (assign34100_e44450 * locals.var_gspr_dn2))) + (assign34100_e44456 * locals.var_gdrift_s_dn2)), ((locals.var_pdiss_dn3 + (assign34100_e44450 * locals.var_gspr_dn3)) + (assign34100_e44456 * locals.var_gdrift_s_dn3)), ((locals.var_pdiss_dn4 + (assign34100_e44450 * locals.var_gspr_dn4)) + (assign34100_e44456 * locals.var_gdrift_s_dn4)), ((locals.var_pdiss_dn5 + (assign34100_e44450 * locals.var_gspr_dn5)) + (assign34100_e44456 * locals.var_gdrift_s_dn5)), ((locals.var_pdiss_dn6 + (assign34100_e44450 * locals.var_gspr_dn6)) + (assign34100_e44456 * locals.var_gdrift_s_dn6)), ((locals.var_pdiss_dn7 + (assign34100_e44450 * locals.var_gspr_dn7)) + ((((-(nv8 - nv7)) + (-(nv8 - nv7))) * locals.var_gdrift_s) + (assign34100_e44456 * locals.var_gdrift_s_dn7))), ((locals.var_pdiss_dn8 + ((((-(nv2 - nv8)) + (-(nv2 - nv8))) * locals.var_gspr) + (assign34100_e44450 * locals.var_gspr_dn8))) + ((((nv8 - nv7) + (nv8 - nv7)) * locals.var_gdrift_s) + (assign34100_e44456 * locals.var_gdrift_s_dn8))), ((locals.var_pdiss_dn9 + (assign34100_e44450 * locals.var_gspr_dn9)) + (assign34100_e44456 * locals.var_gdrift_s_dn9)), ((locals.var_pdiss_dn10 + (assign34100_e44450 * locals.var_gspr_dn10)) + (assign34100_e44456 * locals.var_gdrift_s_dn10)), ((locals.var_pdiss_dn11 + (assign34100_e44450 * locals.var_gspr_dn11)) + (assign34100_e44456 * locals.var_gdrift_s_dn11)), ((locals.var_pdiss_dn12 + (assign34100_e44450 * locals.var_gspr_dn12)) + (assign34100_e44456 * locals.var_gdrift_s_dn12)), ((locals.var_pdiss_dn13 + (assign34100_e44450 * locals.var_gspr_dn13)) + (assign34100_e44456 * locals.var_gdrift_s_dn13)), ((locals.var_pdiss_dn14 + (assign34100_e44450 * locals.var_gspr_dn14)) + (assign34100_e44456 * locals.var_gdrift_s_dn14)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11, locals.var_pdiss_dn12, locals.var_pdiss_dn13, locals.var_pdiss_dn14,)
    }
};
        locals.var_pdiss = assign34100_e44461;
        locals.var_pdiss_dn0 = assign34100_e44461_d_n0;
        locals.var_pdiss_dn2 = assign34100_e44461_d_n2;
        locals.var_pdiss_dn3 = assign34100_e44461_d_n3;
        locals.var_pdiss_dn4 = assign34100_e44461_d_n4;
        locals.var_pdiss_dn5 = assign34100_e44461_d_n5;
        locals.var_pdiss_dn6 = assign34100_e44461_d_n6;
        locals.var_pdiss_dn7 = assign34100_e44461_d_n7;
        locals.var_pdiss_dn8 = assign34100_e44461_d_n8;
        locals.var_pdiss_dn9 = assign34100_e44461_d_n9;
        locals.var_pdiss_dn10 = assign34100_e44461_d_n10;
        locals.var_pdiss_dn11 = assign34100_e44461_d_n11;
        locals.var_pdiss_dn12 = assign34100_e44461_d_n12;
        locals.var_pdiss_dn13 = assign34100_e44461_d_n13;
        locals.var_pdiss_dn14 = assign34100_e44461_d_n14;
        locals.var_pdiss_rv = 0.0;

        let (assign34110_e44476, assign34110_e44476_d_n0, assign34110_e44476_d_n2, assign34110_e44476_d_n3, assign34110_e44476_d_n4, assign34110_e44476_d_n5, assign34110_e44476_d_n6, assign34110_e44476_d_n7, assign34110_e44476_d_n8, assign34110_e44476_d_n9, assign34110_e44476_d_n10, assign34110_e44476_d_n11, assign34110_e44476_d_n12, assign34110_e44476_d_n13, assign34110_e44476_d_n14,) = {
    if (((locals.var_guard763 != 0.0) && (locals.var_guard766 != 0.0)) && (locals.var_guard767 == 0.0)) {
        let assign34110_e44471: f64 = ((nv2 - nv8) * (nv2 - nv8));
        let assign34110_e44473: f64 = (assign34110_e44471 * locals.var_gspr);
        let assign34110_e44474: f64 = (locals.var_pdiss + assign34110_e44473);
        (assign34110_e44474, (locals.var_pdiss_dn0 + (assign34110_e44471 * locals.var_gspr_dn0)), (locals.var_pdiss_dn2 + ((((nv2 - nv8) + (nv2 - nv8)) * locals.var_gspr) + (assign34110_e44471 * locals.var_gspr_dn2))), (locals.var_pdiss_dn3 + (assign34110_e44471 * locals.var_gspr_dn3)), (locals.var_pdiss_dn4 + (assign34110_e44471 * locals.var_gspr_dn4)), (locals.var_pdiss_dn5 + (assign34110_e44471 * locals.var_gspr_dn5)), (locals.var_pdiss_dn6 + (assign34110_e44471 * locals.var_gspr_dn6)), (locals.var_pdiss_dn7 + (assign34110_e44471 * locals.var_gspr_dn7)), (locals.var_pdiss_dn8 + ((((-(nv2 - nv8)) + (-(nv2 - nv8))) * locals.var_gspr) + (assign34110_e44471 * locals.var_gspr_dn8))), (locals.var_pdiss_dn9 + (assign34110_e44471 * locals.var_gspr_dn9)), (locals.var_pdiss_dn10 + (assign34110_e44471 * locals.var_gspr_dn10)), (locals.var_pdiss_dn11 + (assign34110_e44471 * locals.var_gspr_dn11)), (locals.var_pdiss_dn12 + (assign34110_e44471 * locals.var_gspr_dn12)), (locals.var_pdiss_dn13 + (assign34110_e44471 * locals.var_gspr_dn13)), (locals.var_pdiss_dn14 + (assign34110_e44471 * locals.var_gspr_dn14)),)
    } else {
        (locals.var_pdiss, locals.var_pdiss_dn0, locals.var_pdiss_dn2, locals.var_pdiss_dn3, locals.var_pdiss_dn4, locals.var_pdiss_dn5, locals.var_pdiss_dn6, locals.var_pdiss_dn7, locals.var_pdiss_dn8, locals.var_pdiss_dn9, locals.var_pdiss_dn10, locals.var_pdiss_dn11, locals.var_pdiss_dn12, locals.var_pdiss_dn13, locals.var_pdiss_dn14,)
    }
};
        locals.var_pdiss = assign34110_e44476;
        locals.var_pdiss_dn0 = assign34110_e44476_d_n0;
        locals.var_pdiss_dn2 = assign34110_e44476_d_n2;
        locals.var_pdiss_dn3 = assign34110_e44476_d_n3;
        locals.var_pdiss_dn4 = assign34110_e44476_d_n4;
        locals.var_pdiss_dn5 = assign34110_e44476_d_n5;
        locals.var_pdiss_dn6 = assign34110_e44476_d_n6;
        locals.var_pdiss_dn7 = assign34110_e44476_d_n7;
        locals.var_pdiss_dn8 = assign34110_e44476_d_n8;
        locals.var_pdiss_dn9 = assign34110_e44476_d_n9;
        locals.var_pdiss_dn10 = assign34110_e44476_d_n10;
        locals.var_pdiss_dn11 = assign34110_e44476_d_n11;
        locals.var_pdiss_dn12 = assign34110_e44476_d_n12;
        locals.var_pdiss_dn13 = assign34110_e44476_d_n13;
        locals.var_pdiss_dn14 = assign34110_e44476_d_n14;
        locals.var_pdiss_rv = 0.0;

        let assign34130_e44482: f64 = if p.p8 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard769 = assign34130_e44482;
        locals.var_guard769_rv = 0.0;

        let assign34140_e44485: f64 = if p.p1097 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard770 = assign34140_e44485;
        locals.var_guard770_rv = 0.0;

        let assign34160_e44499: f64 = if ((p.p8 != 0.0) && (p.p1097 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard772 = assign34160_e44499;
        locals.var_guard772_rv = 0.0;

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
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let (eq1_e1207, eq1_e1207_d_n0, eq1_e1207_d_n2, eq1_e1207_d_n3, eq1_e1207_d_n4, eq1_e1207_d_n5, eq1_e1207_d_n6, eq1_e1207_d_n7, eq1_e1207_d_n8, eq1_e1207_d_n9, eq1_e1207_d_n10, eq1_e1207_d_n11, eq1_e1207_d_n12, eq1_e1207_d_n13, eq1_e1207_d_n14,) = {
    if (locals.var_guard680 != 0.0) {
        let eq1_e1203: f64 = (locals.var_devsign * p.p28);
        let eq1_e1205: f64 = (eq1_e1203 * locals.var_issl);
        let eq1_e1205_d_n0: f64 = (eq1_e1203 * locals.var_issl_dn0);
        let eq1_e1205_d_n2: f64 = (eq1_e1203 * locals.var_issl_dn2);
        let eq1_e1205_d_n3: f64 = (eq1_e1203 * locals.var_issl_dn3);
        let eq1_e1205_d_n4: f64 = (eq1_e1203 * locals.var_issl_dn4);
        let eq1_e1205_d_n5: f64 = (eq1_e1203 * locals.var_issl_dn5);
        let eq1_e1205_d_n6: f64 = (eq1_e1203 * locals.var_issl_dn6);
        let eq1_e1205_d_n7: f64 = (eq1_e1203 * locals.var_issl_dn7);
        let eq1_e1205_d_n8: f64 = (eq1_e1203 * locals.var_issl_dn8);
        let eq1_e1205_d_n9: f64 = (eq1_e1203 * locals.var_issl_dn9);
        let eq1_e1205_d_n10: f64 = (eq1_e1203 * locals.var_issl_dn10);
        let eq1_e1205_d_n11: f64 = (eq1_e1203 * locals.var_issl_dn11);
        let eq1_e1205_d_n12: f64 = (eq1_e1203 * locals.var_issl_dn12);
        let eq1_e1205_d_n13: f64 = (eq1_e1203 * locals.var_issl_dn13);
        let eq1_e1205_d_n14: f64 = (eq1_e1203 * locals.var_issl_dn14);
        (eq1_e1205, eq1_e1205_d_n0, eq1_e1205_d_n2, eq1_e1205_d_n3, eq1_e1205_d_n4, eq1_e1205_d_n5, eq1_e1205_d_n6, eq1_e1205_d_n7, eq1_e1205_d_n8, eq1_e1205_d_n9, eq1_e1205_d_n10, eq1_e1205_d_n11, eq1_e1205_d_n12, eq1_e1205_d_n13, eq1_e1205_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e1207;
        let eq1_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq1_node_derivatives: [f64; 14] = [eq1_e1207_d_n0, eq1_e1207_d_n2, eq1_e1207_d_n3, eq1_e1207_d_n4, eq1_e1207_d_n5, eq1_e1207_d_n6, eq1_e1207_d_n7, eq1_e1207_d_n8, eq1_e1207_d_n9, eq1_e1207_d_n10, eq1_e1207_d_n11, eq1_e1207_d_n12, eq1_e1207_d_n13, eq1_e1207_d_n14];
        let eq1_branch_derivative_indices: [usize; 0] = [];
        let eq1_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq1_value),
            &eq1_node_derivative_indices,
            &eq1_node_derivatives,
            &eq1_branch_derivative_indices,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq7_e1272, eq7_e1272_d_n0, eq7_e1272_d_n2, eq7_e1272_d_n3, eq7_e1272_d_n4, eq7_e1272_d_n5, eq7_e1272_d_n6, eq7_e1272_d_n7, eq7_e1272_d_n8, eq7_e1272_d_n9, eq7_e1272_d_n10, eq7_e1272_d_n11, eq7_e1272_d_n12, eq7_e1272_d_n13, eq7_e1272_d_n14, eq7_e1272_d_n16,) = {
    if ((locals.var_guard698 != 0.0) && (locals.var_guard697 == 0.0)) {
        let eq7_e1268: f64 = (-locals.var_sqig);
        let eq7_e1270: f64 = (eq7_e1268 * (nv16 - 0.0));
        let eq7_e1270_d_n0: f64 = ((-locals.var_sqig_dn0) * (nv16 - 0.0));
        let eq7_e1270_d_n2: f64 = ((-locals.var_sqig_dn2) * (nv16 - 0.0));
        let eq7_e1270_d_n3: f64 = ((-locals.var_sqig_dn3) * (nv16 - 0.0));
        let eq7_e1270_d_n4: f64 = ((-locals.var_sqig_dn4) * (nv16 - 0.0));
        let eq7_e1270_d_n5: f64 = ((-locals.var_sqig_dn5) * (nv16 - 0.0));
        let eq7_e1270_d_n6: f64 = ((-locals.var_sqig_dn6) * (nv16 - 0.0));
        let eq7_e1270_d_n7: f64 = ((-locals.var_sqig_dn7) * (nv16 - 0.0));
        let eq7_e1270_d_n8: f64 = ((-locals.var_sqig_dn8) * (nv16 - 0.0));
        let eq7_e1270_d_n9: f64 = ((-locals.var_sqig_dn9) * (nv16 - 0.0));
        let eq7_e1270_d_n10: f64 = ((-locals.var_sqig_dn10) * (nv16 - 0.0));
        let eq7_e1270_d_n11: f64 = ((-locals.var_sqig_dn11) * (nv16 - 0.0));
        let eq7_e1270_d_n12: f64 = ((-locals.var_sqig_dn12) * (nv16 - 0.0));
        let eq7_e1270_d_n13: f64 = ((-locals.var_sqig_dn13) * (nv16 - 0.0));
        let eq7_e1270_d_n14: f64 = ((-locals.var_sqig_dn14) * (nv16 - 0.0));
        (eq7_e1270, eq7_e1270_d_n0, eq7_e1270_d_n2, eq7_e1270_d_n3, eq7_e1270_d_n4, eq7_e1270_d_n5, eq7_e1270_d_n6, eq7_e1270_d_n7, eq7_e1270_d_n8, eq7_e1270_d_n9, eq7_e1270_d_n10, eq7_e1270_d_n11, eq7_e1270_d_n12, eq7_e1270_d_n13, eq7_e1270_d_n14, eq7_e1268,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1272;
        let eq7_node_derivative_indices: [usize; 15] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16];
        let eq7_node_derivatives: [f64; 15] = [eq7_e1272_d_n0, eq7_e1272_d_n2, eq7_e1272_d_n3, eq7_e1272_d_n4, eq7_e1272_d_n5, eq7_e1272_d_n6, eq7_e1272_d_n7, eq7_e1272_d_n8, eq7_e1272_d_n9, eq7_e1272_d_n10, eq7_e1272_d_n11, eq7_e1272_d_n12, eq7_e1272_d_n13, eq7_e1272_d_n14, eq7_e1272_d_n16];
        let eq7_branch_derivative_indices: [usize; 0] = [];
        let eq7_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(15),
            None,
            multiplicity * (eq7_value),
            &eq7_node_derivative_indices,
            &eq7_node_derivatives,
            &eq7_branch_derivative_indices,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let (eq8_e1290, eq8_e1290_d_n0, eq8_e1290_d_n2, eq8_e1290_d_n3, eq8_e1290_d_n4, eq8_e1290_d_n5, eq8_e1290_d_n6, eq8_e1290_d_n7, eq8_e1290_d_n8, eq8_e1290_d_n9, eq8_e1290_d_n10, eq8_e1290_d_n11, eq8_e1290_d_n12, eq8_e1290_d_n13, eq8_e1290_d_n14, eq8_e1290_d_n15,) = {
    if ((locals.var_guard698 != 0.0) && (locals.var_guard697 == 0.0)) {
        let eq8_e1279: f64 = (locals.var_mig * locals.var_cox);
        let eq8_e1279_d_n0: f64 = (locals.var_mig_dn0 * locals.var_cox);
        let eq8_e1279_d_n2: f64 = (locals.var_mig_dn2 * locals.var_cox);
        let eq8_e1279_d_n3: f64 = (locals.var_mig_dn3 * locals.var_cox);
        let eq8_e1279_d_n4: f64 = (locals.var_mig_dn4 * locals.var_cox);
        let eq8_e1279_d_n5: f64 = (locals.var_mig_dn5 * locals.var_cox);
        let eq8_e1279_d_n6: f64 = (locals.var_mig_dn6 * locals.var_cox);
        let eq8_e1279_d_n7: f64 = (locals.var_mig_dn7 * locals.var_cox);
        let eq8_e1279_d_n8: f64 = (locals.var_mig_dn8 * locals.var_cox);
        let eq8_e1279_d_n9: f64 = (locals.var_mig_dn9 * locals.var_cox);
        let eq8_e1279_d_n10: f64 = (locals.var_mig_dn10 * locals.var_cox);
        let eq8_e1279_d_n11: f64 = (locals.var_mig_dn11 * locals.var_cox);
        let eq8_e1279_d_n12: f64 = (locals.var_mig_dn12 * locals.var_cox);
        let eq8_e1279_d_n13: f64 = (locals.var_mig_dn13 * locals.var_cox);
        let eq8_e1279_d_n14: f64 = (locals.var_mig_dn14 * locals.var_cox);
        let eq8_e1281: f64 = (eq8_e1279 * locals.var_weff);
        let eq8_e1281_d_n0: f64 = (eq8_e1279_d_n0 * locals.var_weff);
        let eq8_e1281_d_n2: f64 = (eq8_e1279_d_n2 * locals.var_weff);
        let eq8_e1281_d_n3: f64 = (eq8_e1279_d_n3 * locals.var_weff);
        let eq8_e1281_d_n4: f64 = (eq8_e1279_d_n4 * locals.var_weff);
        let eq8_e1281_d_n5: f64 = (eq8_e1279_d_n5 * locals.var_weff);
        let eq8_e1281_d_n6: f64 = (eq8_e1279_d_n6 * locals.var_weff);
        let eq8_e1281_d_n7: f64 = (eq8_e1279_d_n7 * locals.var_weff);
        let eq8_e1281_d_n8: f64 = (eq8_e1279_d_n8 * locals.var_weff);
        let eq8_e1281_d_n9: f64 = (eq8_e1279_d_n9 * locals.var_weff);
        let eq8_e1281_d_n10: f64 = (eq8_e1279_d_n10 * locals.var_weff);
        let eq8_e1281_d_n11: f64 = (eq8_e1279_d_n11 * locals.var_weff);
        let eq8_e1281_d_n12: f64 = (eq8_e1279_d_n12 * locals.var_weff);
        let eq8_e1281_d_n13: f64 = (eq8_e1279_d_n13 * locals.var_weff);
        let eq8_e1281_d_n14: f64 = (eq8_e1279_d_n14 * locals.var_weff);
        let eq8_e1283: f64 = (eq8_e1281 * p.p2);
        let eq8_e1283_d_n0: f64 = (eq8_e1281_d_n0 * p.p2);
        let eq8_e1283_d_n2: f64 = (eq8_e1281_d_n2 * p.p2);
        let eq8_e1283_d_n3: f64 = (eq8_e1281_d_n3 * p.p2);
        let eq8_e1283_d_n4: f64 = (eq8_e1281_d_n4 * p.p2);
        let eq8_e1283_d_n5: f64 = (eq8_e1281_d_n5 * p.p2);
        let eq8_e1283_d_n6: f64 = (eq8_e1281_d_n6 * p.p2);
        let eq8_e1283_d_n7: f64 = (eq8_e1281_d_n7 * p.p2);
        let eq8_e1283_d_n8: f64 = (eq8_e1281_d_n8 * p.p2);
        let eq8_e1283_d_n9: f64 = (eq8_e1281_d_n9 * p.p2);
        let eq8_e1283_d_n10: f64 = (eq8_e1281_d_n10 * p.p2);
        let eq8_e1283_d_n11: f64 = (eq8_e1281_d_n11 * p.p2);
        let eq8_e1283_d_n12: f64 = (eq8_e1281_d_n12 * p.p2);
        let eq8_e1283_d_n13: f64 = (eq8_e1281_d_n13 * p.p2);
        let eq8_e1283_d_n14: f64 = (eq8_e1281_d_n14 * p.p2);
        let eq8_e1285: f64 = (eq8_e1283 * locals.var_leff);
        let eq8_e1285_d_n0: f64 = (eq8_e1283_d_n0 * locals.var_leff);
        let eq8_e1285_d_n2: f64 = (eq8_e1283_d_n2 * locals.var_leff);
        let eq8_e1285_d_n3: f64 = (eq8_e1283_d_n3 * locals.var_leff);
        let eq8_e1285_d_n4: f64 = (eq8_e1283_d_n4 * locals.var_leff);
        let eq8_e1285_d_n5: f64 = (eq8_e1283_d_n5 * locals.var_leff);
        let eq8_e1285_d_n6: f64 = (eq8_e1283_d_n6 * locals.var_leff);
        let eq8_e1285_d_n7: f64 = (eq8_e1283_d_n7 * locals.var_leff);
        let eq8_e1285_d_n8: f64 = (eq8_e1283_d_n8 * locals.var_leff);
        let eq8_e1285_d_n9: f64 = (eq8_e1283_d_n9 * locals.var_leff);
        let eq8_e1285_d_n10: f64 = (eq8_e1283_d_n10 * locals.var_leff);
        let eq8_e1285_d_n11: f64 = (eq8_e1283_d_n11 * locals.var_leff);
        let eq8_e1285_d_n12: f64 = (eq8_e1283_d_n12 * locals.var_leff);
        let eq8_e1285_d_n13: f64 = (eq8_e1283_d_n13 * locals.var_leff);
        let eq8_e1285_d_n14: f64 = (eq8_e1283_d_n14 * locals.var_leff);
        let eq8_e1287: f64 = (eq8_e1285 * (nv15 - 0.0));
        let eq8_e1287_d_n0: f64 = (eq8_e1285_d_n0 * (nv15 - 0.0));
        let eq8_e1287_d_n2: f64 = (eq8_e1285_d_n2 * (nv15 - 0.0));
        let eq8_e1287_d_n3: f64 = (eq8_e1285_d_n3 * (nv15 - 0.0));
        let eq8_e1287_d_n4: f64 = (eq8_e1285_d_n4 * (nv15 - 0.0));
        let eq8_e1287_d_n5: f64 = (eq8_e1285_d_n5 * (nv15 - 0.0));
        let eq8_e1287_d_n6: f64 = (eq8_e1285_d_n6 * (nv15 - 0.0));
        let eq8_e1287_d_n7: f64 = (eq8_e1285_d_n7 * (nv15 - 0.0));
        let eq8_e1287_d_n8: f64 = (eq8_e1285_d_n8 * (nv15 - 0.0));
        let eq8_e1287_d_n9: f64 = (eq8_e1285_d_n9 * (nv15 - 0.0));
        let eq8_e1287_d_n10: f64 = (eq8_e1285_d_n10 * (nv15 - 0.0));
        let eq8_e1287_d_n11: f64 = (eq8_e1285_d_n11 * (nv15 - 0.0));
        let eq8_e1287_d_n12: f64 = (eq8_e1285_d_n12 * (nv15 - 0.0));
        let eq8_e1287_d_n13: f64 = (eq8_e1285_d_n13 * (nv15 - 0.0));
        let eq8_e1287_d_n14: f64 = (eq8_e1285_d_n14 * (nv15 - 0.0));
        let eq8_e1288: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq8_e1287);
        (eq8_e1288, (eq8_e1287_d_n0 * ddt_scale), (eq8_e1287_d_n2 * ddt_scale), (eq8_e1287_d_n3 * ddt_scale), (eq8_e1287_d_n4 * ddt_scale), (eq8_e1287_d_n5 * ddt_scale), (eq8_e1287_d_n6 * ddt_scale), (eq8_e1287_d_n7 * ddt_scale), (eq8_e1287_d_n8 * ddt_scale), (eq8_e1287_d_n9 * ddt_scale), (eq8_e1287_d_n10 * ddt_scale), (eq8_e1287_d_n11 * ddt_scale), (eq8_e1287_d_n12 * ddt_scale), (eq8_e1287_d_n13 * ddt_scale), (eq8_e1287_d_n14 * ddt_scale), (eq8_e1285 * ddt_scale),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_value: f64 = eq8_e1290;
        let eq8_node_derivative_indices: [usize; 15] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let eq8_node_derivatives: [f64; 15] = [eq8_e1290_d_n0, eq8_e1290_d_n2, eq8_e1290_d_n3, eq8_e1290_d_n4, eq8_e1290_d_n5, eq8_e1290_d_n6, eq8_e1290_d_n7, eq8_e1290_d_n8, eq8_e1290_d_n9, eq8_e1290_d_n10, eq8_e1290_d_n11, eq8_e1290_d_n12, eq8_e1290_d_n13, eq8_e1290_d_n14, eq8_e1290_d_n15];
        let eq8_branch_derivative_indices: [usize; 0] = [];
        let eq8_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(15),
            None,
            multiplicity * (eq8_value),
            &eq8_node_derivative_indices,
            &eq8_node_derivatives,
            &eq8_branch_derivative_indices,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let (eq10_e1318, eq10_e1318_d_n0, eq10_e1318_d_n2, eq10_e1318_d_n3, eq10_e1318_d_n4, eq10_e1318_d_n5, eq10_e1318_d_n6, eq10_e1318_d_n7, eq10_e1318_d_n8, eq10_e1318_d_n9, eq10_e1318_d_n10, eq10_e1318_d_n11, eq10_e1318_d_n12, eq10_e1318_d_n13, eq10_e1318_d_n14, eq10_e1318_d_n16,) = {
    if ((locals.var_guard698 != 0.0) && (locals.var_guard697 == 0.0)) {
        let eq10_e1314: f64 = (locals.var_sqid * p.p28);
        let eq10_e1314_d_n0: f64 = (locals.var_sqid_dn0 * p.p28);
        let eq10_e1314_d_n2: f64 = (locals.var_sqid_dn2 * p.p28);
        let eq10_e1314_d_n3: f64 = (locals.var_sqid_dn3 * p.p28);
        let eq10_e1314_d_n4: f64 = (locals.var_sqid_dn4 * p.p28);
        let eq10_e1314_d_n5: f64 = (locals.var_sqid_dn5 * p.p28);
        let eq10_e1314_d_n6: f64 = (locals.var_sqid_dn6 * p.p28);
        let eq10_e1314_d_n7: f64 = (locals.var_sqid_dn7 * p.p28);
        let eq10_e1314_d_n8: f64 = (locals.var_sqid_dn8 * p.p28);
        let eq10_e1314_d_n9: f64 = (locals.var_sqid_dn9 * p.p28);
        let eq10_e1314_d_n10: f64 = (locals.var_sqid_dn10 * p.p28);
        let eq10_e1314_d_n11: f64 = (locals.var_sqid_dn11 * p.p28);
        let eq10_e1314_d_n12: f64 = (locals.var_sqid_dn12 * p.p28);
        let eq10_e1314_d_n13: f64 = (locals.var_sqid_dn13 * p.p28);
        let eq10_e1314_d_n14: f64 = (locals.var_sqid_dn14 * p.p28);
        let eq10_e1316: f64 = (eq10_e1314 * (nv16 - 0.0));
        let eq10_e1316_d_n0: f64 = (eq10_e1314_d_n0 * (nv16 - 0.0));
        let eq10_e1316_d_n2: f64 = (eq10_e1314_d_n2 * (nv16 - 0.0));
        let eq10_e1316_d_n3: f64 = (eq10_e1314_d_n3 * (nv16 - 0.0));
        let eq10_e1316_d_n4: f64 = (eq10_e1314_d_n4 * (nv16 - 0.0));
        let eq10_e1316_d_n5: f64 = (eq10_e1314_d_n5 * (nv16 - 0.0));
        let eq10_e1316_d_n6: f64 = (eq10_e1314_d_n6 * (nv16 - 0.0));
        let eq10_e1316_d_n7: f64 = (eq10_e1314_d_n7 * (nv16 - 0.0));
        let eq10_e1316_d_n8: f64 = (eq10_e1314_d_n8 * (nv16 - 0.0));
        let eq10_e1316_d_n9: f64 = (eq10_e1314_d_n9 * (nv16 - 0.0));
        let eq10_e1316_d_n10: f64 = (eq10_e1314_d_n10 * (nv16 - 0.0));
        let eq10_e1316_d_n11: f64 = (eq10_e1314_d_n11 * (nv16 - 0.0));
        let eq10_e1316_d_n12: f64 = (eq10_e1314_d_n12 * (nv16 - 0.0));
        let eq10_e1316_d_n13: f64 = (eq10_e1314_d_n13 * (nv16 - 0.0));
        let eq10_e1316_d_n14: f64 = (eq10_e1314_d_n14 * (nv16 - 0.0));
        (eq10_e1316, eq10_e1316_d_n0, eq10_e1316_d_n2, eq10_e1316_d_n3, eq10_e1316_d_n4, eq10_e1316_d_n5, eq10_e1316_d_n6, eq10_e1316_d_n7, eq10_e1316_d_n8, eq10_e1316_d_n9, eq10_e1316_d_n10, eq10_e1316_d_n11, eq10_e1316_d_n12, eq10_e1316_d_n13, eq10_e1316_d_n14, eq10_e1314,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq10_value: f64 = eq10_e1318;
        let eq10_node_derivative_indices: [usize; 15] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16];
        let eq10_node_derivatives: [f64; 15] = [eq10_e1318_d_n0, eq10_e1318_d_n2, eq10_e1318_d_n3, eq10_e1318_d_n4, eq10_e1318_d_n5, eq10_e1318_d_n6, eq10_e1318_d_n7, eq10_e1318_d_n8, eq10_e1318_d_n9, eq10_e1318_d_n10, eq10_e1318_d_n11, eq10_e1318_d_n12, eq10_e1318_d_n13, eq10_e1318_d_n14, eq10_e1318_d_n16];
        let eq10_branch_derivative_indices: [usize; 0] = [];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivative_indices,
            &eq10_node_derivatives,
            &eq10_branch_derivative_indices,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let (eq11_e1344, eq11_e1344_d_n0, eq11_e1344_d_n2, eq11_e1344_d_n3, eq11_e1344_d_n4, eq11_e1344_d_n5, eq11_e1344_d_n6, eq11_e1344_d_n7, eq11_e1344_d_n8, eq11_e1344_d_n9, eq11_e1344_d_n10, eq11_e1344_d_n11, eq11_e1344_d_n12, eq11_e1344_d_n13, eq11_e1344_d_n14, eq11_e1344_d_n15,) = {
    if ((locals.var_guard698 != 0.0) && (locals.var_guard697 == 0.0)) {
        let eq11_e1327: f64 = (1.0 + locals.var_sigvds);
        let eq11_e1329: f64 = (eq11_e1327 * locals.var_mig);
        let eq11_e1329_d_n0: f64 = (eq11_e1327 * locals.var_mig_dn0);
        let eq11_e1329_d_n2: f64 = (eq11_e1327 * locals.var_mig_dn2);
        let eq11_e1329_d_n3: f64 = (eq11_e1327 * locals.var_mig_dn3);
        let eq11_e1329_d_n4: f64 = (eq11_e1327 * locals.var_mig_dn4);
        let eq11_e1329_d_n5: f64 = (eq11_e1327 * locals.var_mig_dn5);
        let eq11_e1329_d_n6: f64 = (eq11_e1327 * locals.var_mig_dn6);
        let eq11_e1329_d_n7: f64 = (eq11_e1327 * locals.var_mig_dn7);
        let eq11_e1329_d_n8: f64 = (eq11_e1327 * locals.var_mig_dn8);
        let eq11_e1329_d_n9: f64 = (eq11_e1327 * locals.var_mig_dn9);
        let eq11_e1329_d_n10: f64 = (eq11_e1327 * locals.var_mig_dn10);
        let eq11_e1329_d_n11: f64 = (eq11_e1327 * locals.var_mig_dn11);
        let eq11_e1329_d_n12: f64 = (eq11_e1327 * locals.var_mig_dn12);
        let eq11_e1329_d_n13: f64 = (eq11_e1327 * locals.var_mig_dn13);
        let eq11_e1329_d_n14: f64 = (eq11_e1327 * locals.var_mig_dn14);
        let eq11_e1331: f64 = (eq11_e1329 * locals.var_cox);
        let eq11_e1331_d_n0: f64 = (eq11_e1329_d_n0 * locals.var_cox);
        let eq11_e1331_d_n2: f64 = (eq11_e1329_d_n2 * locals.var_cox);
        let eq11_e1331_d_n3: f64 = (eq11_e1329_d_n3 * locals.var_cox);
        let eq11_e1331_d_n4: f64 = (eq11_e1329_d_n4 * locals.var_cox);
        let eq11_e1331_d_n5: f64 = (eq11_e1329_d_n5 * locals.var_cox);
        let eq11_e1331_d_n6: f64 = (eq11_e1329_d_n6 * locals.var_cox);
        let eq11_e1331_d_n7: f64 = (eq11_e1329_d_n7 * locals.var_cox);
        let eq11_e1331_d_n8: f64 = (eq11_e1329_d_n8 * locals.var_cox);
        let eq11_e1331_d_n9: f64 = (eq11_e1329_d_n9 * locals.var_cox);
        let eq11_e1331_d_n10: f64 = (eq11_e1329_d_n10 * locals.var_cox);
        let eq11_e1331_d_n11: f64 = (eq11_e1329_d_n11 * locals.var_cox);
        let eq11_e1331_d_n12: f64 = (eq11_e1329_d_n12 * locals.var_cox);
        let eq11_e1331_d_n13: f64 = (eq11_e1329_d_n13 * locals.var_cox);
        let eq11_e1331_d_n14: f64 = (eq11_e1329_d_n14 * locals.var_cox);
        let eq11_e1333: f64 = (eq11_e1331 * locals.var_weff);
        let eq11_e1333_d_n0: f64 = (eq11_e1331_d_n0 * locals.var_weff);
        let eq11_e1333_d_n2: f64 = (eq11_e1331_d_n2 * locals.var_weff);
        let eq11_e1333_d_n3: f64 = (eq11_e1331_d_n3 * locals.var_weff);
        let eq11_e1333_d_n4: f64 = (eq11_e1331_d_n4 * locals.var_weff);
        let eq11_e1333_d_n5: f64 = (eq11_e1331_d_n5 * locals.var_weff);
        let eq11_e1333_d_n6: f64 = (eq11_e1331_d_n6 * locals.var_weff);
        let eq11_e1333_d_n7: f64 = (eq11_e1331_d_n7 * locals.var_weff);
        let eq11_e1333_d_n8: f64 = (eq11_e1331_d_n8 * locals.var_weff);
        let eq11_e1333_d_n9: f64 = (eq11_e1331_d_n9 * locals.var_weff);
        let eq11_e1333_d_n10: f64 = (eq11_e1331_d_n10 * locals.var_weff);
        let eq11_e1333_d_n11: f64 = (eq11_e1331_d_n11 * locals.var_weff);
        let eq11_e1333_d_n12: f64 = (eq11_e1331_d_n12 * locals.var_weff);
        let eq11_e1333_d_n13: f64 = (eq11_e1331_d_n13 * locals.var_weff);
        let eq11_e1333_d_n14: f64 = (eq11_e1331_d_n14 * locals.var_weff);
        let eq11_e1335: f64 = (eq11_e1333 * p.p2);
        let eq11_e1335_d_n0: f64 = (eq11_e1333_d_n0 * p.p2);
        let eq11_e1335_d_n2: f64 = (eq11_e1333_d_n2 * p.p2);
        let eq11_e1335_d_n3: f64 = (eq11_e1333_d_n3 * p.p2);
        let eq11_e1335_d_n4: f64 = (eq11_e1333_d_n4 * p.p2);
        let eq11_e1335_d_n5: f64 = (eq11_e1333_d_n5 * p.p2);
        let eq11_e1335_d_n6: f64 = (eq11_e1333_d_n6 * p.p2);
        let eq11_e1335_d_n7: f64 = (eq11_e1333_d_n7 * p.p2);
        let eq11_e1335_d_n8: f64 = (eq11_e1333_d_n8 * p.p2);
        let eq11_e1335_d_n9: f64 = (eq11_e1333_d_n9 * p.p2);
        let eq11_e1335_d_n10: f64 = (eq11_e1333_d_n10 * p.p2);
        let eq11_e1335_d_n11: f64 = (eq11_e1333_d_n11 * p.p2);
        let eq11_e1335_d_n12: f64 = (eq11_e1333_d_n12 * p.p2);
        let eq11_e1335_d_n13: f64 = (eq11_e1333_d_n13 * p.p2);
        let eq11_e1335_d_n14: f64 = (eq11_e1333_d_n14 * p.p2);
        let eq11_e1337: f64 = (eq11_e1335 * locals.var_leff);
        let eq11_e1337_d_n0: f64 = (eq11_e1335_d_n0 * locals.var_leff);
        let eq11_e1337_d_n2: f64 = (eq11_e1335_d_n2 * locals.var_leff);
        let eq11_e1337_d_n3: f64 = (eq11_e1335_d_n3 * locals.var_leff);
        let eq11_e1337_d_n4: f64 = (eq11_e1335_d_n4 * locals.var_leff);
        let eq11_e1337_d_n5: f64 = (eq11_e1335_d_n5 * locals.var_leff);
        let eq11_e1337_d_n6: f64 = (eq11_e1335_d_n6 * locals.var_leff);
        let eq11_e1337_d_n7: f64 = (eq11_e1335_d_n7 * locals.var_leff);
        let eq11_e1337_d_n8: f64 = (eq11_e1335_d_n8 * locals.var_leff);
        let eq11_e1337_d_n9: f64 = (eq11_e1335_d_n9 * locals.var_leff);
        let eq11_e1337_d_n10: f64 = (eq11_e1335_d_n10 * locals.var_leff);
        let eq11_e1337_d_n11: f64 = (eq11_e1335_d_n11 * locals.var_leff);
        let eq11_e1337_d_n12: f64 = (eq11_e1335_d_n12 * locals.var_leff);
        let eq11_e1337_d_n13: f64 = (eq11_e1335_d_n13 * locals.var_leff);
        let eq11_e1337_d_n14: f64 = (eq11_e1335_d_n14 * locals.var_leff);
        let eq11_e1339: f64 = (eq11_e1337 * (nv15 - 0.0));
        let eq11_e1339_d_n0: f64 = (eq11_e1337_d_n0 * (nv15 - 0.0));
        let eq11_e1339_d_n2: f64 = (eq11_e1337_d_n2 * (nv15 - 0.0));
        let eq11_e1339_d_n3: f64 = (eq11_e1337_d_n3 * (nv15 - 0.0));
        let eq11_e1339_d_n4: f64 = (eq11_e1337_d_n4 * (nv15 - 0.0));
        let eq11_e1339_d_n5: f64 = (eq11_e1337_d_n5 * (nv15 - 0.0));
        let eq11_e1339_d_n6: f64 = (eq11_e1337_d_n6 * (nv15 - 0.0));
        let eq11_e1339_d_n7: f64 = (eq11_e1337_d_n7 * (nv15 - 0.0));
        let eq11_e1339_d_n8: f64 = (eq11_e1337_d_n8 * (nv15 - 0.0));
        let eq11_e1339_d_n9: f64 = (eq11_e1337_d_n9 * (nv15 - 0.0));
        let eq11_e1339_d_n10: f64 = (eq11_e1337_d_n10 * (nv15 - 0.0));
        let eq11_e1339_d_n11: f64 = (eq11_e1337_d_n11 * (nv15 - 0.0));
        let eq11_e1339_d_n12: f64 = (eq11_e1337_d_n12 * (nv15 - 0.0));
        let eq11_e1339_d_n13: f64 = (eq11_e1337_d_n13 * (nv15 - 0.0));
        let eq11_e1339_d_n14: f64 = (eq11_e1337_d_n14 * (nv15 - 0.0));
        let eq11_e1340: f64 = (0.5 * eq11_e1339);
        let eq11_e1340_d_n0: f64 = (0.5 * eq11_e1339_d_n0);
        let eq11_e1340_d_n2: f64 = (0.5 * eq11_e1339_d_n2);
        let eq11_e1340_d_n3: f64 = (0.5 * eq11_e1339_d_n3);
        let eq11_e1340_d_n4: f64 = (0.5 * eq11_e1339_d_n4);
        let eq11_e1340_d_n5: f64 = (0.5 * eq11_e1339_d_n5);
        let eq11_e1340_d_n6: f64 = (0.5 * eq11_e1339_d_n6);
        let eq11_e1340_d_n7: f64 = (0.5 * eq11_e1339_d_n7);
        let eq11_e1340_d_n8: f64 = (0.5 * eq11_e1339_d_n8);
        let eq11_e1340_d_n9: f64 = (0.5 * eq11_e1339_d_n9);
        let eq11_e1340_d_n10: f64 = (0.5 * eq11_e1339_d_n10);
        let eq11_e1340_d_n11: f64 = (0.5 * eq11_e1339_d_n11);
        let eq11_e1340_d_n12: f64 = (0.5 * eq11_e1339_d_n12);
        let eq11_e1340_d_n13: f64 = (0.5 * eq11_e1339_d_n13);
        let eq11_e1340_d_n14: f64 = (0.5 * eq11_e1339_d_n14);
        let eq11_e1340_d_n15: f64 = (0.5 * eq11_e1337);
        let eq11_e1341: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq11_e1340);
        let eq11_e1342: f64 = (p.p29 * eq11_e1341);
        let eq11_e1342_d_n0: f64 = (p.p29 * (eq11_e1340_d_n0 * ddt_scale));
        let eq11_e1342_d_n2: f64 = (p.p29 * (eq11_e1340_d_n2 * ddt_scale));
        let eq11_e1342_d_n3: f64 = (p.p29 * (eq11_e1340_d_n3 * ddt_scale));
        let eq11_e1342_d_n4: f64 = (p.p29 * (eq11_e1340_d_n4 * ddt_scale));
        let eq11_e1342_d_n5: f64 = (p.p29 * (eq11_e1340_d_n5 * ddt_scale));
        let eq11_e1342_d_n6: f64 = (p.p29 * (eq11_e1340_d_n6 * ddt_scale));
        let eq11_e1342_d_n7: f64 = (p.p29 * (eq11_e1340_d_n7 * ddt_scale));
        let eq11_e1342_d_n8: f64 = (p.p29 * (eq11_e1340_d_n8 * ddt_scale));
        let eq11_e1342_d_n9: f64 = (p.p29 * (eq11_e1340_d_n9 * ddt_scale));
        let eq11_e1342_d_n10: f64 = (p.p29 * (eq11_e1340_d_n10 * ddt_scale));
        let eq11_e1342_d_n11: f64 = (p.p29 * (eq11_e1340_d_n11 * ddt_scale));
        let eq11_e1342_d_n12: f64 = (p.p29 * (eq11_e1340_d_n12 * ddt_scale));
        let eq11_e1342_d_n13: f64 = (p.p29 * (eq11_e1340_d_n13 * ddt_scale));
        let eq11_e1342_d_n14: f64 = (p.p29 * (eq11_e1340_d_n14 * ddt_scale));
        let eq11_e1342_d_n15: f64 = (p.p29 * (eq11_e1340_d_n15 * ddt_scale));
        (eq11_e1342, eq11_e1342_d_n0, eq11_e1342_d_n2, eq11_e1342_d_n3, eq11_e1342_d_n4, eq11_e1342_d_n5, eq11_e1342_d_n6, eq11_e1342_d_n7, eq11_e1342_d_n8, eq11_e1342_d_n9, eq11_e1342_d_n10, eq11_e1342_d_n11, eq11_e1342_d_n12, eq11_e1342_d_n13, eq11_e1342_d_n14, eq11_e1342_d_n15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_value: f64 = eq11_e1344;
        let eq11_node_derivative_indices: [usize; 15] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let eq11_node_derivatives: [f64; 15] = [eq11_e1344_d_n0, eq11_e1344_d_n2, eq11_e1344_d_n3, eq11_e1344_d_n4, eq11_e1344_d_n5, eq11_e1344_d_n6, eq11_e1344_d_n7, eq11_e1344_d_n8, eq11_e1344_d_n9, eq11_e1344_d_n10, eq11_e1344_d_n11, eq11_e1344_d_n12, eq11_e1344_d_n13, eq11_e1344_d_n14, eq11_e1344_d_n15];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq12_e1370, eq12_e1370_d_n0, eq12_e1370_d_n2, eq12_e1370_d_n3, eq12_e1370_d_n4, eq12_e1370_d_n5, eq12_e1370_d_n6, eq12_e1370_d_n7, eq12_e1370_d_n8, eq12_e1370_d_n9, eq12_e1370_d_n10, eq12_e1370_d_n11, eq12_e1370_d_n12, eq12_e1370_d_n13, eq12_e1370_d_n14, eq12_e1370_d_n15,) = {
    if ((locals.var_guard698 != 0.0) && (locals.var_guard697 == 0.0)) {
        let eq12_e1353: f64 = (1.0 - locals.var_sigvds);
        let eq12_e1355: f64 = (eq12_e1353 * locals.var_mig);
        let eq12_e1355_d_n0: f64 = (eq12_e1353 * locals.var_mig_dn0);
        let eq12_e1355_d_n2: f64 = (eq12_e1353 * locals.var_mig_dn2);
        let eq12_e1355_d_n3: f64 = (eq12_e1353 * locals.var_mig_dn3);
        let eq12_e1355_d_n4: f64 = (eq12_e1353 * locals.var_mig_dn4);
        let eq12_e1355_d_n5: f64 = (eq12_e1353 * locals.var_mig_dn5);
        let eq12_e1355_d_n6: f64 = (eq12_e1353 * locals.var_mig_dn6);
        let eq12_e1355_d_n7: f64 = (eq12_e1353 * locals.var_mig_dn7);
        let eq12_e1355_d_n8: f64 = (eq12_e1353 * locals.var_mig_dn8);
        let eq12_e1355_d_n9: f64 = (eq12_e1353 * locals.var_mig_dn9);
        let eq12_e1355_d_n10: f64 = (eq12_e1353 * locals.var_mig_dn10);
        let eq12_e1355_d_n11: f64 = (eq12_e1353 * locals.var_mig_dn11);
        let eq12_e1355_d_n12: f64 = (eq12_e1353 * locals.var_mig_dn12);
        let eq12_e1355_d_n13: f64 = (eq12_e1353 * locals.var_mig_dn13);
        let eq12_e1355_d_n14: f64 = (eq12_e1353 * locals.var_mig_dn14);
        let eq12_e1357: f64 = (eq12_e1355 * locals.var_cox);
        let eq12_e1357_d_n0: f64 = (eq12_e1355_d_n0 * locals.var_cox);
        let eq12_e1357_d_n2: f64 = (eq12_e1355_d_n2 * locals.var_cox);
        let eq12_e1357_d_n3: f64 = (eq12_e1355_d_n3 * locals.var_cox);
        let eq12_e1357_d_n4: f64 = (eq12_e1355_d_n4 * locals.var_cox);
        let eq12_e1357_d_n5: f64 = (eq12_e1355_d_n5 * locals.var_cox);
        let eq12_e1357_d_n6: f64 = (eq12_e1355_d_n6 * locals.var_cox);
        let eq12_e1357_d_n7: f64 = (eq12_e1355_d_n7 * locals.var_cox);
        let eq12_e1357_d_n8: f64 = (eq12_e1355_d_n8 * locals.var_cox);
        let eq12_e1357_d_n9: f64 = (eq12_e1355_d_n9 * locals.var_cox);
        let eq12_e1357_d_n10: f64 = (eq12_e1355_d_n10 * locals.var_cox);
        let eq12_e1357_d_n11: f64 = (eq12_e1355_d_n11 * locals.var_cox);
        let eq12_e1357_d_n12: f64 = (eq12_e1355_d_n12 * locals.var_cox);
        let eq12_e1357_d_n13: f64 = (eq12_e1355_d_n13 * locals.var_cox);
        let eq12_e1357_d_n14: f64 = (eq12_e1355_d_n14 * locals.var_cox);
        let eq12_e1359: f64 = (eq12_e1357 * locals.var_weff);
        let eq12_e1359_d_n0: f64 = (eq12_e1357_d_n0 * locals.var_weff);
        let eq12_e1359_d_n2: f64 = (eq12_e1357_d_n2 * locals.var_weff);
        let eq12_e1359_d_n3: f64 = (eq12_e1357_d_n3 * locals.var_weff);
        let eq12_e1359_d_n4: f64 = (eq12_e1357_d_n4 * locals.var_weff);
        let eq12_e1359_d_n5: f64 = (eq12_e1357_d_n5 * locals.var_weff);
        let eq12_e1359_d_n6: f64 = (eq12_e1357_d_n6 * locals.var_weff);
        let eq12_e1359_d_n7: f64 = (eq12_e1357_d_n7 * locals.var_weff);
        let eq12_e1359_d_n8: f64 = (eq12_e1357_d_n8 * locals.var_weff);
        let eq12_e1359_d_n9: f64 = (eq12_e1357_d_n9 * locals.var_weff);
        let eq12_e1359_d_n10: f64 = (eq12_e1357_d_n10 * locals.var_weff);
        let eq12_e1359_d_n11: f64 = (eq12_e1357_d_n11 * locals.var_weff);
        let eq12_e1359_d_n12: f64 = (eq12_e1357_d_n12 * locals.var_weff);
        let eq12_e1359_d_n13: f64 = (eq12_e1357_d_n13 * locals.var_weff);
        let eq12_e1359_d_n14: f64 = (eq12_e1357_d_n14 * locals.var_weff);
        let eq12_e1361: f64 = (eq12_e1359 * p.p2);
        let eq12_e1361_d_n0: f64 = (eq12_e1359_d_n0 * p.p2);
        let eq12_e1361_d_n2: f64 = (eq12_e1359_d_n2 * p.p2);
        let eq12_e1361_d_n3: f64 = (eq12_e1359_d_n3 * p.p2);
        let eq12_e1361_d_n4: f64 = (eq12_e1359_d_n4 * p.p2);
        let eq12_e1361_d_n5: f64 = (eq12_e1359_d_n5 * p.p2);
        let eq12_e1361_d_n6: f64 = (eq12_e1359_d_n6 * p.p2);
        let eq12_e1361_d_n7: f64 = (eq12_e1359_d_n7 * p.p2);
        let eq12_e1361_d_n8: f64 = (eq12_e1359_d_n8 * p.p2);
        let eq12_e1361_d_n9: f64 = (eq12_e1359_d_n9 * p.p2);
        let eq12_e1361_d_n10: f64 = (eq12_e1359_d_n10 * p.p2);
        let eq12_e1361_d_n11: f64 = (eq12_e1359_d_n11 * p.p2);
        let eq12_e1361_d_n12: f64 = (eq12_e1359_d_n12 * p.p2);
        let eq12_e1361_d_n13: f64 = (eq12_e1359_d_n13 * p.p2);
        let eq12_e1361_d_n14: f64 = (eq12_e1359_d_n14 * p.p2);
        let eq12_e1363: f64 = (eq12_e1361 * locals.var_leff);
        let eq12_e1363_d_n0: f64 = (eq12_e1361_d_n0 * locals.var_leff);
        let eq12_e1363_d_n2: f64 = (eq12_e1361_d_n2 * locals.var_leff);
        let eq12_e1363_d_n3: f64 = (eq12_e1361_d_n3 * locals.var_leff);
        let eq12_e1363_d_n4: f64 = (eq12_e1361_d_n4 * locals.var_leff);
        let eq12_e1363_d_n5: f64 = (eq12_e1361_d_n5 * locals.var_leff);
        let eq12_e1363_d_n6: f64 = (eq12_e1361_d_n6 * locals.var_leff);
        let eq12_e1363_d_n7: f64 = (eq12_e1361_d_n7 * locals.var_leff);
        let eq12_e1363_d_n8: f64 = (eq12_e1361_d_n8 * locals.var_leff);
        let eq12_e1363_d_n9: f64 = (eq12_e1361_d_n9 * locals.var_leff);
        let eq12_e1363_d_n10: f64 = (eq12_e1361_d_n10 * locals.var_leff);
        let eq12_e1363_d_n11: f64 = (eq12_e1361_d_n11 * locals.var_leff);
        let eq12_e1363_d_n12: f64 = (eq12_e1361_d_n12 * locals.var_leff);
        let eq12_e1363_d_n13: f64 = (eq12_e1361_d_n13 * locals.var_leff);
        let eq12_e1363_d_n14: f64 = (eq12_e1361_d_n14 * locals.var_leff);
        let eq12_e1365: f64 = (eq12_e1363 * (nv15 - 0.0));
        let eq12_e1365_d_n0: f64 = (eq12_e1363_d_n0 * (nv15 - 0.0));
        let eq12_e1365_d_n2: f64 = (eq12_e1363_d_n2 * (nv15 - 0.0));
        let eq12_e1365_d_n3: f64 = (eq12_e1363_d_n3 * (nv15 - 0.0));
        let eq12_e1365_d_n4: f64 = (eq12_e1363_d_n4 * (nv15 - 0.0));
        let eq12_e1365_d_n5: f64 = (eq12_e1363_d_n5 * (nv15 - 0.0));
        let eq12_e1365_d_n6: f64 = (eq12_e1363_d_n6 * (nv15 - 0.0));
        let eq12_e1365_d_n7: f64 = (eq12_e1363_d_n7 * (nv15 - 0.0));
        let eq12_e1365_d_n8: f64 = (eq12_e1363_d_n8 * (nv15 - 0.0));
        let eq12_e1365_d_n9: f64 = (eq12_e1363_d_n9 * (nv15 - 0.0));
        let eq12_e1365_d_n10: f64 = (eq12_e1363_d_n10 * (nv15 - 0.0));
        let eq12_e1365_d_n11: f64 = (eq12_e1363_d_n11 * (nv15 - 0.0));
        let eq12_e1365_d_n12: f64 = (eq12_e1363_d_n12 * (nv15 - 0.0));
        let eq12_e1365_d_n13: f64 = (eq12_e1363_d_n13 * (nv15 - 0.0));
        let eq12_e1365_d_n14: f64 = (eq12_e1363_d_n14 * (nv15 - 0.0));
        let eq12_e1366: f64 = (0.5 * eq12_e1365);
        let eq12_e1366_d_n0: f64 = (0.5 * eq12_e1365_d_n0);
        let eq12_e1366_d_n2: f64 = (0.5 * eq12_e1365_d_n2);
        let eq12_e1366_d_n3: f64 = (0.5 * eq12_e1365_d_n3);
        let eq12_e1366_d_n4: f64 = (0.5 * eq12_e1365_d_n4);
        let eq12_e1366_d_n5: f64 = (0.5 * eq12_e1365_d_n5);
        let eq12_e1366_d_n6: f64 = (0.5 * eq12_e1365_d_n6);
        let eq12_e1366_d_n7: f64 = (0.5 * eq12_e1365_d_n7);
        let eq12_e1366_d_n8: f64 = (0.5 * eq12_e1365_d_n8);
        let eq12_e1366_d_n9: f64 = (0.5 * eq12_e1365_d_n9);
        let eq12_e1366_d_n10: f64 = (0.5 * eq12_e1365_d_n10);
        let eq12_e1366_d_n11: f64 = (0.5 * eq12_e1365_d_n11);
        let eq12_e1366_d_n12: f64 = (0.5 * eq12_e1365_d_n12);
        let eq12_e1366_d_n13: f64 = (0.5 * eq12_e1365_d_n13);
        let eq12_e1366_d_n14: f64 = (0.5 * eq12_e1365_d_n14);
        let eq12_e1366_d_n15: f64 = (0.5 * eq12_e1363);
        let eq12_e1367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq12_e1366);
        let eq12_e1368: f64 = (p.p29 * eq12_e1367);
        let eq12_e1368_d_n0: f64 = (p.p29 * (eq12_e1366_d_n0 * ddt_scale));
        let eq12_e1368_d_n2: f64 = (p.p29 * (eq12_e1366_d_n2 * ddt_scale));
        let eq12_e1368_d_n3: f64 = (p.p29 * (eq12_e1366_d_n3 * ddt_scale));
        let eq12_e1368_d_n4: f64 = (p.p29 * (eq12_e1366_d_n4 * ddt_scale));
        let eq12_e1368_d_n5: f64 = (p.p29 * (eq12_e1366_d_n5 * ddt_scale));
        let eq12_e1368_d_n6: f64 = (p.p29 * (eq12_e1366_d_n6 * ddt_scale));
        let eq12_e1368_d_n7: f64 = (p.p29 * (eq12_e1366_d_n7 * ddt_scale));
        let eq12_e1368_d_n8: f64 = (p.p29 * (eq12_e1366_d_n8 * ddt_scale));
        let eq12_e1368_d_n9: f64 = (p.p29 * (eq12_e1366_d_n9 * ddt_scale));
        let eq12_e1368_d_n10: f64 = (p.p29 * (eq12_e1366_d_n10 * ddt_scale));
        let eq12_e1368_d_n11: f64 = (p.p29 * (eq12_e1366_d_n11 * ddt_scale));
        let eq12_e1368_d_n12: f64 = (p.p29 * (eq12_e1366_d_n12 * ddt_scale));
        let eq12_e1368_d_n13: f64 = (p.p29 * (eq12_e1366_d_n13 * ddt_scale));
        let eq12_e1368_d_n14: f64 = (p.p29 * (eq12_e1366_d_n14 * ddt_scale));
        let eq12_e1368_d_n15: f64 = (p.p29 * (eq12_e1366_d_n15 * ddt_scale));
        (eq12_e1368, eq12_e1368_d_n0, eq12_e1368_d_n2, eq12_e1368_d_n3, eq12_e1368_d_n4, eq12_e1368_d_n5, eq12_e1368_d_n6, eq12_e1368_d_n7, eq12_e1368_d_n8, eq12_e1368_d_n9, eq12_e1368_d_n10, eq12_e1368_d_n11, eq12_e1368_d_n12, eq12_e1368_d_n13, eq12_e1368_d_n14, eq12_e1368_d_n15,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_value: f64 = eq12_e1370;
        let eq12_node_derivative_indices: [usize; 15] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let eq12_node_derivatives: [f64; 15] = [eq12_e1370_d_n0, eq12_e1370_d_n2, eq12_e1370_d_n3, eq12_e1370_d_n4, eq12_e1370_d_n5, eq12_e1370_d_n6, eq12_e1370_d_n7, eq12_e1370_d_n8, eq12_e1370_d_n9, eq12_e1370_d_n10, eq12_e1370_d_n11, eq12_e1370_d_n12, eq12_e1370_d_n13, eq12_e1370_d_n14, eq12_e1370_d_n15];
        let eq12_branch_derivative_indices: [usize; 0] = [];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq12_value),
            &eq12_node_derivative_indices,
            &eq12_node_derivatives,
            &eq12_branch_derivative_indices,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq19_e1428: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, locals.var_qgi_1);
        let eq19_value: f64 = eq19_e1428;
        let eq19_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq19_node_derivatives: [f64; 14] = [(locals.var_qgi_1_dn0 * ddt_scale), (locals.var_qgi_1_dn2 * ddt_scale), (locals.var_qgi_1_dn3 * ddt_scale), (locals.var_qgi_1_dn4 * ddt_scale), (locals.var_qgi_1_dn5 * ddt_scale), (locals.var_qgi_1_dn6 * ddt_scale), (locals.var_qgi_1_dn7 * ddt_scale), (locals.var_qgi_1_dn8 * ddt_scale), (locals.var_qgi_1_dn9 * ddt_scale), (locals.var_qgi_1_dn10 * ddt_scale), (locals.var_qgi_1_dn11 * ddt_scale), (locals.var_qgi_1_dn12 * ddt_scale), (locals.var_qgi_1_dn13 * ddt_scale), (locals.var_qgi_1_dn14 * ddt_scale)];
        let eq19_branch_derivative_indices: [usize; 0] = [];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(11),
            multiplicity * (eq19_value),
            &eq19_node_derivative_indices,
            &eq19_node_derivatives,
            &eq19_branch_derivative_indices,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_e1430: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, locals.var_qsi_1);
        let eq20_value: f64 = eq20_e1430;
        let eq20_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq20_node_derivatives: [f64; 14] = [(locals.var_qsi_1_dn0 * ddt_scale), (locals.var_qsi_1_dn2 * ddt_scale), (locals.var_qsi_1_dn3 * ddt_scale), (locals.var_qsi_1_dn4 * ddt_scale), (locals.var_qsi_1_dn5 * ddt_scale), (locals.var_qsi_1_dn6 * ddt_scale), (locals.var_qsi_1_dn7 * ddt_scale), (locals.var_qsi_1_dn8 * ddt_scale), (locals.var_qsi_1_dn9 * ddt_scale), (locals.var_qsi_1_dn10 * ddt_scale), (locals.var_qsi_1_dn11 * ddt_scale), (locals.var_qsi_1_dn12 * ddt_scale), (locals.var_qsi_1_dn13 * ddt_scale), (locals.var_qsi_1_dn14 * ddt_scale)];
        let eq20_branch_derivative_indices: [usize; 0] = [];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(11),
            multiplicity * (eq20_value),
            &eq20_node_derivative_indices,
            &eq20_node_derivatives,
            &eq20_branch_derivative_indices,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let eq21_e1432: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, locals.var_qdi_1);
        let eq21_value: f64 = eq21_e1432;
        let eq21_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq21_node_derivatives: [f64; 14] = [(locals.var_qdi_1_dn0 * ddt_scale), (locals.var_qdi_1_dn2 * ddt_scale), (locals.var_qdi_1_dn3 * ddt_scale), (locals.var_qdi_1_dn4 * ddt_scale), (locals.var_qdi_1_dn5 * ddt_scale), (locals.var_qdi_1_dn6 * ddt_scale), (locals.var_qdi_1_dn7 * ddt_scale), (locals.var_qdi_1_dn8 * ddt_scale), (locals.var_qdi_1_dn9 * ddt_scale), (locals.var_qdi_1_dn10 * ddt_scale), (locals.var_qdi_1_dn11 * ddt_scale), (locals.var_qdi_1_dn12 * ddt_scale), (locals.var_qdi_1_dn13 * ddt_scale), (locals.var_qdi_1_dn14 * ddt_scale)];
        let eq21_branch_derivative_indices: [usize; 0] = [];
        let eq21_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(11),
            multiplicity * (eq21_value),
            &eq21_node_derivative_indices,
            &eq21_node_derivatives,
            &eq21_branch_derivative_indices,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let eq22_e1435: f64 = (-locals.var_devsign);
        let eq22_e1437: f64 = (eq22_e1435 * locals.var_qovs);
        let eq22_e1437_d_n0: f64 = (eq22_e1435 * locals.var_qovs_dn0);
        let eq22_e1437_d_n2: f64 = (eq22_e1435 * locals.var_qovs_dn2);
        let eq22_e1437_d_n3: f64 = (eq22_e1435 * locals.var_qovs_dn3);
        let eq22_e1437_d_n4: f64 = (eq22_e1435 * locals.var_qovs_dn4);
        let eq22_e1437_d_n5: f64 = (eq22_e1435 * locals.var_qovs_dn5);
        let eq22_e1437_d_n6: f64 = (eq22_e1435 * locals.var_qovs_dn6);
        let eq22_e1437_d_n7: f64 = (eq22_e1435 * locals.var_qovs_dn7);
        let eq22_e1437_d_n8: f64 = (eq22_e1435 * locals.var_qovs_dn8);
        let eq22_e1437_d_n9: f64 = (eq22_e1435 * locals.var_qovs_dn9);
        let eq22_e1437_d_n10: f64 = (eq22_e1435 * locals.var_qovs_dn10);
        let eq22_e1437_d_n11: f64 = (eq22_e1435 * locals.var_qovs_dn11);
        let eq22_e1437_d_n12: f64 = (eq22_e1435 * locals.var_qovs_dn12);
        let eq22_e1437_d_n13: f64 = (eq22_e1435 * locals.var_qovs_dn13);
        let eq22_e1437_d_n14: f64 = (eq22_e1435 * locals.var_qovs_dn14);
        let eq22_e1438: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq22_e1437);
        let eq22_e1439: f64 = (p.p29 * eq22_e1438);
        let eq22_e1439_d_n0: f64 = (p.p29 * (eq22_e1437_d_n0 * ddt_scale));
        let eq22_e1439_d_n2: f64 = (p.p29 * (eq22_e1437_d_n2 * ddt_scale));
        let eq22_e1439_d_n3: f64 = (p.p29 * (eq22_e1437_d_n3 * ddt_scale));
        let eq22_e1439_d_n4: f64 = (p.p29 * (eq22_e1437_d_n4 * ddt_scale));
        let eq22_e1439_d_n5: f64 = (p.p29 * (eq22_e1437_d_n5 * ddt_scale));
        let eq22_e1439_d_n6: f64 = (p.p29 * (eq22_e1437_d_n6 * ddt_scale));
        let eq22_e1439_d_n7: f64 = (p.p29 * (eq22_e1437_d_n7 * ddt_scale));
        let eq22_e1439_d_n8: f64 = (p.p29 * (eq22_e1437_d_n8 * ddt_scale));
        let eq22_e1439_d_n9: f64 = (p.p29 * (eq22_e1437_d_n9 * ddt_scale));
        let eq22_e1439_d_n10: f64 = (p.p29 * (eq22_e1437_d_n10 * ddt_scale));
        let eq22_e1439_d_n11: f64 = (p.p29 * (eq22_e1437_d_n11 * ddt_scale));
        let eq22_e1439_d_n12: f64 = (p.p29 * (eq22_e1437_d_n12 * ddt_scale));
        let eq22_e1439_d_n13: f64 = (p.p29 * (eq22_e1437_d_n13 * ddt_scale));
        let eq22_e1439_d_n14: f64 = (p.p29 * (eq22_e1437_d_n14 * ddt_scale));
        let eq22_value: f64 = eq22_e1439;
        let eq22_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq22_node_derivatives: [f64; 14] = [eq22_e1439_d_n0, eq22_e1439_d_n2, eq22_e1439_d_n3, eq22_e1439_d_n4, eq22_e1439_d_n5, eq22_e1439_d_n6, eq22_e1439_d_n7, eq22_e1439_d_n8, eq22_e1439_d_n9, eq22_e1439_d_n10, eq22_e1439_d_n11, eq22_e1439_d_n12, eq22_e1439_d_n13, eq22_e1439_d_n14];
        let eq22_branch_derivative_indices: [usize; 0] = [];
        let eq22_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(7),
            multiplicity * (eq22_value),
            &eq22_node_derivative_indices,
            &eq22_node_derivatives,
            &eq22_branch_derivative_indices,
            &eq22_branch_derivatives,
            multiplicity,
        );
        let eq23_e1442: f64 = (-locals.var_devsign);
        let eq23_e1444: f64 = (eq23_e1442 * locals.var_qovd);
        let eq23_e1444_d_n0: f64 = (eq23_e1442 * locals.var_qovd_dn0);
        let eq23_e1444_d_n2: f64 = (eq23_e1442 * locals.var_qovd_dn2);
        let eq23_e1444_d_n3: f64 = (eq23_e1442 * locals.var_qovd_dn3);
        let eq23_e1444_d_n4: f64 = (eq23_e1442 * locals.var_qovd_dn4);
        let eq23_e1444_d_n5: f64 = (eq23_e1442 * locals.var_qovd_dn5);
        let eq23_e1444_d_n6: f64 = (eq23_e1442 * locals.var_qovd_dn6);
        let eq23_e1444_d_n7: f64 = (eq23_e1442 * locals.var_qovd_dn7);
        let eq23_e1444_d_n8: f64 = (eq23_e1442 * locals.var_qovd_dn8);
        let eq23_e1444_d_n9: f64 = (eq23_e1442 * locals.var_qovd_dn9);
        let eq23_e1444_d_n10: f64 = (eq23_e1442 * locals.var_qovd_dn10);
        let eq23_e1444_d_n11: f64 = (eq23_e1442 * locals.var_qovd_dn11);
        let eq23_e1444_d_n12: f64 = (eq23_e1442 * locals.var_qovd_dn12);
        let eq23_e1444_d_n13: f64 = (eq23_e1442 * locals.var_qovd_dn13);
        let eq23_e1444_d_n14: f64 = (eq23_e1442 * locals.var_qovd_dn14);
        let eq23_e1445: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq23_e1444);
        let eq23_e1446: f64 = (p.p29 * eq23_e1445);
        let eq23_e1446_d_n0: f64 = (p.p29 * (eq23_e1444_d_n0 * ddt_scale));
        let eq23_e1446_d_n2: f64 = (p.p29 * (eq23_e1444_d_n2 * ddt_scale));
        let eq23_e1446_d_n3: f64 = (p.p29 * (eq23_e1444_d_n3 * ddt_scale));
        let eq23_e1446_d_n4: f64 = (p.p29 * (eq23_e1444_d_n4 * ddt_scale));
        let eq23_e1446_d_n5: f64 = (p.p29 * (eq23_e1444_d_n5 * ddt_scale));
        let eq23_e1446_d_n6: f64 = (p.p29 * (eq23_e1444_d_n6 * ddt_scale));
        let eq23_e1446_d_n7: f64 = (p.p29 * (eq23_e1444_d_n7 * ddt_scale));
        let eq23_e1446_d_n8: f64 = (p.p29 * (eq23_e1444_d_n8 * ddt_scale));
        let eq23_e1446_d_n9: f64 = (p.p29 * (eq23_e1444_d_n9 * ddt_scale));
        let eq23_e1446_d_n10: f64 = (p.p29 * (eq23_e1444_d_n10 * ddt_scale));
        let eq23_e1446_d_n11: f64 = (p.p29 * (eq23_e1444_d_n11 * ddt_scale));
        let eq23_e1446_d_n12: f64 = (p.p29 * (eq23_e1444_d_n12 * ddt_scale));
        let eq23_e1446_d_n13: f64 = (p.p29 * (eq23_e1444_d_n13 * ddt_scale));
        let eq23_e1446_d_n14: f64 = (p.p29 * (eq23_e1444_d_n14 * ddt_scale));
        let eq23_value: f64 = eq23_e1446;
        let eq23_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq23_node_derivatives: [f64; 14] = [eq23_e1446_d_n0, eq23_e1446_d_n2, eq23_e1446_d_n3, eq23_e1446_d_n4, eq23_e1446_d_n5, eq23_e1446_d_n6, eq23_e1446_d_n7, eq23_e1446_d_n8, eq23_e1446_d_n9, eq23_e1446_d_n10, eq23_e1446_d_n11, eq23_e1446_d_n12, eq23_e1446_d_n13, eq23_e1446_d_n14];
        let eq23_branch_derivative_indices: [usize; 0] = [];
        let eq23_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(5),
            multiplicity * (eq23_value),
            &eq23_node_derivative_indices,
            &eq23_node_derivatives,
            &eq23_branch_derivative_indices,
            &eq23_branch_derivatives,
            multiplicity,
        );
        let eq24_e1449: f64 = (-locals.var_devsign);
        let eq24_e1451: f64 = (eq24_e1449 * locals.var_qovb);
        let eq24_e1451_d_n0: f64 = (eq24_e1449 * locals.var_qovb_dn0);
        let eq24_e1451_d_n2: f64 = (eq24_e1449 * locals.var_qovb_dn2);
        let eq24_e1451_d_n3: f64 = (eq24_e1449 * locals.var_qovb_dn3);
        let eq24_e1451_d_n4: f64 = (eq24_e1449 * locals.var_qovb_dn4);
        let eq24_e1451_d_n5: f64 = (eq24_e1449 * locals.var_qovb_dn5);
        let eq24_e1451_d_n6: f64 = (eq24_e1449 * locals.var_qovb_dn6);
        let eq24_e1451_d_n7: f64 = (eq24_e1449 * locals.var_qovb_dn7);
        let eq24_e1451_d_n8: f64 = (eq24_e1449 * locals.var_qovb_dn8);
        let eq24_e1451_d_n9: f64 = (eq24_e1449 * locals.var_qovb_dn9);
        let eq24_e1451_d_n10: f64 = (eq24_e1449 * locals.var_qovb_dn10);
        let eq24_e1451_d_n11: f64 = (eq24_e1449 * locals.var_qovb_dn11);
        let eq24_e1451_d_n12: f64 = (eq24_e1449 * locals.var_qovb_dn12);
        let eq24_e1451_d_n13: f64 = (eq24_e1449 * locals.var_qovb_dn13);
        let eq24_e1451_d_n14: f64 = (eq24_e1449 * locals.var_qovb_dn14);
        let eq24_e1452: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq24_e1451);
        let eq24_e1453: f64 = (p.p29 * eq24_e1452);
        let eq24_e1453_d_n0: f64 = (p.p29 * (eq24_e1451_d_n0 * ddt_scale));
        let eq24_e1453_d_n2: f64 = (p.p29 * (eq24_e1451_d_n2 * ddt_scale));
        let eq24_e1453_d_n3: f64 = (p.p29 * (eq24_e1451_d_n3 * ddt_scale));
        let eq24_e1453_d_n4: f64 = (p.p29 * (eq24_e1451_d_n4 * ddt_scale));
        let eq24_e1453_d_n5: f64 = (p.p29 * (eq24_e1451_d_n5 * ddt_scale));
        let eq24_e1453_d_n6: f64 = (p.p29 * (eq24_e1451_d_n6 * ddt_scale));
        let eq24_e1453_d_n7: f64 = (p.p29 * (eq24_e1451_d_n7 * ddt_scale));
        let eq24_e1453_d_n8: f64 = (p.p29 * (eq24_e1451_d_n8 * ddt_scale));
        let eq24_e1453_d_n9: f64 = (p.p29 * (eq24_e1451_d_n9 * ddt_scale));
        let eq24_e1453_d_n10: f64 = (p.p29 * (eq24_e1451_d_n10 * ddt_scale));
        let eq24_e1453_d_n11: f64 = (p.p29 * (eq24_e1451_d_n11 * ddt_scale));
        let eq24_e1453_d_n12: f64 = (p.p29 * (eq24_e1451_d_n12 * ddt_scale));
        let eq24_e1453_d_n13: f64 = (p.p29 * (eq24_e1451_d_n13 * ddt_scale));
        let eq24_e1453_d_n14: f64 = (p.p29 * (eq24_e1451_d_n14 * ddt_scale));
        let eq24_value: f64 = eq24_e1453;
        let eq24_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq24_node_derivatives: [f64; 14] = [eq24_e1453_d_n0, eq24_e1453_d_n2, eq24_e1453_d_n3, eq24_e1453_d_n4, eq24_e1453_d_n5, eq24_e1453_d_n6, eq24_e1453_d_n7, eq24_e1453_d_n8, eq24_e1453_d_n9, eq24_e1453_d_n10, eq24_e1453_d_n11, eq24_e1453_d_n12, eq24_e1453_d_n13, eq24_e1453_d_n14];
        let eq24_branch_derivative_indices: [usize; 0] = [];
        let eq24_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(11),
            multiplicity * (eq24_value),
            &eq24_node_derivative_indices,
            &eq24_node_derivatives,
            &eq24_branch_derivative_indices,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let eq25_e1456: f64 = (locals.var_devsign * p.p28);
        let eq25_e1458: f64 = (eq25_e1456 * locals.var_sigvds);
        let eq25_e1460: f64 = (eq25_e1458 * locals.var_ids);
        let eq25_e1460_d_n0: f64 = (eq25_e1458 * locals.var_ids_dn0);
        let eq25_e1460_d_n2: f64 = (eq25_e1458 * locals.var_ids_dn2);
        let eq25_e1460_d_n3: f64 = (eq25_e1458 * locals.var_ids_dn3);
        let eq25_e1460_d_n4: f64 = (eq25_e1458 * locals.var_ids_dn4);
        let eq25_e1460_d_n5: f64 = (eq25_e1458 * locals.var_ids_dn5);
        let eq25_e1460_d_n6: f64 = (eq25_e1458 * locals.var_ids_dn6);
        let eq25_e1460_d_n7: f64 = (eq25_e1458 * locals.var_ids_dn7);
        let eq25_e1460_d_n8: f64 = (eq25_e1458 * locals.var_ids_dn8);
        let eq25_e1460_d_n9: f64 = (eq25_e1458 * locals.var_ids_dn9);
        let eq25_e1460_d_n10: f64 = (eq25_e1458 * locals.var_ids_dn10);
        let eq25_e1460_d_n11: f64 = (eq25_e1458 * locals.var_ids_dn11);
        let eq25_e1460_d_n12: f64 = (eq25_e1458 * locals.var_ids_dn12);
        let eq25_e1460_d_n13: f64 = (eq25_e1458 * locals.var_ids_dn13);
        let eq25_e1460_d_n14: f64 = (eq25_e1458 * locals.var_ids_dn14);
        let eq25_value: f64 = eq25_e1460;
        let eq25_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq25_node_derivatives: [f64; 14] = [eq25_e1460_d_n0, eq25_e1460_d_n2, eq25_e1460_d_n3, eq25_e1460_d_n4, eq25_e1460_d_n5, eq25_e1460_d_n6, eq25_e1460_d_n7, eq25_e1460_d_n8, eq25_e1460_d_n9, eq25_e1460_d_n10, eq25_e1460_d_n11, eq25_e1460_d_n12, eq25_e1460_d_n13, eq25_e1460_d_n14];
        let eq25_branch_derivative_indices: [usize; 0] = [];
        let eq25_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq25_value),
            &eq25_node_derivative_indices,
            &eq25_node_derivatives,
            &eq25_branch_derivative_indices,
            &eq25_branch_derivatives,
            multiplicity,
        );
        let (eq26_e1464, eq26_e1464_d_n0, eq26_e1464_d_n2, eq26_e1464_d_n3, eq26_e1464_d_n4, eq26_e1464_d_n5, eq26_e1464_d_n6, eq26_e1464_d_n7, eq26_e1464_d_n8, eq26_e1464_d_n9, eq26_e1464_d_n10, eq26_e1464_d_n11, eq26_e1464_d_n12, eq26_e1464_d_n13, eq26_e1464_d_n14,) = {
    if (locals.var_guard751 != 0.0) {
        (locals.var_igb_1, locals.var_igb_1_dn0, locals.var_igb_1_dn2, locals.var_igb_1_dn3, locals.var_igb_1_dn4, locals.var_igb_1_dn5, locals.var_igb_1_dn6, locals.var_igb_1_dn7, locals.var_igb_1_dn8, locals.var_igb_1_dn9, locals.var_igb_1_dn10, locals.var_igb_1_dn11, locals.var_igb_1_dn12, locals.var_igb_1_dn13, locals.var_igb_1_dn14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e1464;
        let eq26_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq26_node_derivatives: [f64; 14] = [eq26_e1464_d_n0, eq26_e1464_d_n2, eq26_e1464_d_n3, eq26_e1464_d_n4, eq26_e1464_d_n5, eq26_e1464_d_n6, eq26_e1464_d_n7, eq26_e1464_d_n8, eq26_e1464_d_n9, eq26_e1464_d_n10, eq26_e1464_d_n11, eq26_e1464_d_n12, eq26_e1464_d_n13, eq26_e1464_d_n14];
        let eq26_branch_derivative_indices: [usize; 0] = [];
        let eq26_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(11),
            multiplicity * (eq26_value),
            &eq26_node_derivative_indices,
            &eq26_node_derivatives,
            &eq26_branch_derivative_indices,
            &eq26_branch_derivatives,
            multiplicity,
        );
        let (eq27_e1470, eq27_e1470_d_n0, eq27_e1470_d_n2, eq27_e1470_d_n3, eq27_e1470_d_n4, eq27_e1470_d_n5, eq27_e1470_d_n6, eq27_e1470_d_n7, eq27_e1470_d_n8, eq27_e1470_d_n9, eq27_e1470_d_n10, eq27_e1470_d_n11, eq27_e1470_d_n12, eq27_e1470_d_n13, eq27_e1470_d_n14,) = {
    if (locals.var_guard752 != 0.0) {
        let eq27_e1468: f64 = (locals.var_igs_1 + locals.var_igcs_1);
        let eq27_e1468_d_n0: f64 = (locals.var_igs_1_dn0 + locals.var_igcs_1_dn0);
        let eq27_e1468_d_n2: f64 = (locals.var_igs_1_dn2 + locals.var_igcs_1_dn2);
        let eq27_e1468_d_n3: f64 = (locals.var_igs_1_dn3 + locals.var_igcs_1_dn3);
        let eq27_e1468_d_n4: f64 = (locals.var_igs_1_dn4 + locals.var_igcs_1_dn4);
        let eq27_e1468_d_n5: f64 = (locals.var_igs_1_dn5 + locals.var_igcs_1_dn5);
        let eq27_e1468_d_n6: f64 = (locals.var_igs_1_dn6 + locals.var_igcs_1_dn6);
        let eq27_e1468_d_n7: f64 = (locals.var_igs_1_dn7 + locals.var_igcs_1_dn7);
        let eq27_e1468_d_n8: f64 = (locals.var_igs_1_dn8 + locals.var_igcs_1_dn8);
        let eq27_e1468_d_n9: f64 = (locals.var_igs_1_dn9 + locals.var_igcs_1_dn9);
        let eq27_e1468_d_n10: f64 = (locals.var_igs_1_dn10 + locals.var_igcs_1_dn10);
        let eq27_e1468_d_n11: f64 = (locals.var_igs_1_dn11 + locals.var_igcs_1_dn11);
        let eq27_e1468_d_n12: f64 = (locals.var_igs_1_dn12 + locals.var_igcs_1_dn12);
        let eq27_e1468_d_n13: f64 = (locals.var_igs_1_dn13 + locals.var_igcs_1_dn13);
        let eq27_e1468_d_n14: f64 = (locals.var_igs_1_dn14 + locals.var_igcs_1_dn14);
        (eq27_e1468, eq27_e1468_d_n0, eq27_e1468_d_n2, eq27_e1468_d_n3, eq27_e1468_d_n4, eq27_e1468_d_n5, eq27_e1468_d_n6, eq27_e1468_d_n7, eq27_e1468_d_n8, eq27_e1468_d_n9, eq27_e1468_d_n10, eq27_e1468_d_n11, eq27_e1468_d_n12, eq27_e1468_d_n13, eq27_e1468_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1470;
        let eq27_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq27_node_derivatives: [f64; 14] = [eq27_e1470_d_n0, eq27_e1470_d_n2, eq27_e1470_d_n3, eq27_e1470_d_n4, eq27_e1470_d_n5, eq27_e1470_d_n6, eq27_e1470_d_n7, eq27_e1470_d_n8, eq27_e1470_d_n9, eq27_e1470_d_n10, eq27_e1470_d_n11, eq27_e1470_d_n12, eq27_e1470_d_n13, eq27_e1470_d_n14];
        let eq27_branch_derivative_indices: [usize; 0] = [];
        let eq27_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq27_value),
            &eq27_node_derivative_indices,
            &eq27_node_derivatives,
            &eq27_branch_derivative_indices,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let (eq28_e1476, eq28_e1476_d_n0, eq28_e1476_d_n2, eq28_e1476_d_n3, eq28_e1476_d_n4, eq28_e1476_d_n5, eq28_e1476_d_n6, eq28_e1476_d_n7, eq28_e1476_d_n8, eq28_e1476_d_n9, eq28_e1476_d_n10, eq28_e1476_d_n11, eq28_e1476_d_n12, eq28_e1476_d_n13, eq28_e1476_d_n14,) = {
    if (locals.var_guard752 != 0.0) {
        let eq28_e1474: f64 = (locals.var_igd_1 + locals.var_igcd_1);
        let eq28_e1474_d_n0: f64 = (locals.var_igd_1_dn0 + locals.var_igcd_1_dn0);
        let eq28_e1474_d_n2: f64 = (locals.var_igd_1_dn2 + locals.var_igcd_1_dn2);
        let eq28_e1474_d_n3: f64 = (locals.var_igd_1_dn3 + locals.var_igcd_1_dn3);
        let eq28_e1474_d_n4: f64 = (locals.var_igd_1_dn4 + locals.var_igcd_1_dn4);
        let eq28_e1474_d_n5: f64 = (locals.var_igd_1_dn5 + locals.var_igcd_1_dn5);
        let eq28_e1474_d_n6: f64 = (locals.var_igd_1_dn6 + locals.var_igcd_1_dn6);
        let eq28_e1474_d_n7: f64 = (locals.var_igd_1_dn7 + locals.var_igcd_1_dn7);
        let eq28_e1474_d_n8: f64 = (locals.var_igd_1_dn8 + locals.var_igcd_1_dn8);
        let eq28_e1474_d_n9: f64 = (locals.var_igd_1_dn9 + locals.var_igcd_1_dn9);
        let eq28_e1474_d_n10: f64 = (locals.var_igd_1_dn10 + locals.var_igcd_1_dn10);
        let eq28_e1474_d_n11: f64 = (locals.var_igd_1_dn11 + locals.var_igcd_1_dn11);
        let eq28_e1474_d_n12: f64 = (locals.var_igd_1_dn12 + locals.var_igcd_1_dn12);
        let eq28_e1474_d_n13: f64 = (locals.var_igd_1_dn13 + locals.var_igcd_1_dn13);
        let eq28_e1474_d_n14: f64 = (locals.var_igd_1_dn14 + locals.var_igcd_1_dn14);
        (eq28_e1474, eq28_e1474_d_n0, eq28_e1474_d_n2, eq28_e1474_d_n3, eq28_e1474_d_n4, eq28_e1474_d_n5, eq28_e1474_d_n6, eq28_e1474_d_n7, eq28_e1474_d_n8, eq28_e1474_d_n9, eq28_e1474_d_n10, eq28_e1474_d_n11, eq28_e1474_d_n12, eq28_e1474_d_n13, eq28_e1474_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e1476;
        let eq28_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq28_node_derivatives: [f64; 14] = [eq28_e1476_d_n0, eq28_e1476_d_n2, eq28_e1476_d_n3, eq28_e1476_d_n4, eq28_e1476_d_n5, eq28_e1476_d_n6, eq28_e1476_d_n7, eq28_e1476_d_n8, eq28_e1476_d_n9, eq28_e1476_d_n10, eq28_e1476_d_n11, eq28_e1476_d_n12, eq28_e1476_d_n13, eq28_e1476_d_n14];
        let eq28_branch_derivative_indices: [usize; 0] = [];
        let eq28_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(9),
            Some(5),
            multiplicity * (eq28_value),
            &eq28_node_derivative_indices,
            &eq28_node_derivatives,
            &eq28_branch_derivative_indices,
            &eq28_branch_derivatives,
            multiplicity,
        );
        let (eq29_e1482, eq29_e1482_d_n0, eq29_e1482_d_n2, eq29_e1482_d_n3, eq29_e1482_d_n4, eq29_e1482_d_n5, eq29_e1482_d_n6, eq29_e1482_d_n7, eq29_e1482_d_n8, eq29_e1482_d_n9, eq29_e1482_d_n10, eq29_e1482_d_n11, eq29_e1482_d_n12, eq29_e1482_d_n13, eq29_e1482_d_n14,) = {
    if (locals.var_guard753 != 0.0) {
        let eq29_e1480: f64 = (locals.var_isub + locals.var_igidl_1);
        let eq29_e1480_d_n0: f64 = (locals.var_isub_dn0 + locals.var_igidl_1_dn0);
        let eq29_e1480_d_n2: f64 = (locals.var_isub_dn2 + locals.var_igidl_1_dn2);
        let eq29_e1480_d_n3: f64 = (locals.var_isub_dn3 + locals.var_igidl_1_dn3);
        let eq29_e1480_d_n4: f64 = (locals.var_isub_dn4 + locals.var_igidl_1_dn4);
        let eq29_e1480_d_n5: f64 = (locals.var_isub_dn5 + locals.var_igidl_1_dn5);
        let eq29_e1480_d_n6: f64 = (locals.var_isub_dn6 + locals.var_igidl_1_dn6);
        let eq29_e1480_d_n7: f64 = (locals.var_isub_dn7 + locals.var_igidl_1_dn7);
        let eq29_e1480_d_n8: f64 = (locals.var_isub_dn8 + locals.var_igidl_1_dn8);
        let eq29_e1480_d_n9: f64 = (locals.var_isub_dn9 + locals.var_igidl_1_dn9);
        let eq29_e1480_d_n10: f64 = (locals.var_isub_dn10 + locals.var_igidl_1_dn10);
        let eq29_e1480_d_n11: f64 = (locals.var_isub_dn11 + locals.var_igidl_1_dn11);
        let eq29_e1480_d_n12: f64 = (locals.var_isub_dn12 + locals.var_igidl_1_dn12);
        let eq29_e1480_d_n13: f64 = (locals.var_isub_dn13 + locals.var_igidl_1_dn13);
        let eq29_e1480_d_n14: f64 = (locals.var_isub_dn14 + locals.var_igidl_1_dn14);
        (eq29_e1480, eq29_e1480_d_n0, eq29_e1480_d_n2, eq29_e1480_d_n3, eq29_e1480_d_n4, eq29_e1480_d_n5, eq29_e1480_d_n6, eq29_e1480_d_n7, eq29_e1480_d_n8, eq29_e1480_d_n9, eq29_e1480_d_n10, eq29_e1480_d_n11, eq29_e1480_d_n12, eq29_e1480_d_n13, eq29_e1480_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq29_value: f64 = eq29_e1482;
        let eq29_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq29_node_derivatives: [f64; 14] = [eq29_e1482_d_n0, eq29_e1482_d_n2, eq29_e1482_d_n3, eq29_e1482_d_n4, eq29_e1482_d_n5, eq29_e1482_d_n6, eq29_e1482_d_n7, eq29_e1482_d_n8, eq29_e1482_d_n9, eq29_e1482_d_n10, eq29_e1482_d_n11, eq29_e1482_d_n12, eq29_e1482_d_n13, eq29_e1482_d_n14];
        let eq29_branch_derivative_indices: [usize; 0] = [];
        let eq29_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(11),
            multiplicity * (eq29_value),
            &eq29_node_derivative_indices,
            &eq29_node_derivatives,
            &eq29_branch_derivative_indices,
            &eq29_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq30_e1490, eq30_e1490_d_n0, eq30_e1490_d_n2, eq30_e1490_d_n3, eq30_e1490_d_n4, eq30_e1490_d_n5, eq30_e1490_d_n6, eq30_e1490_d_n7, eq30_e1490_d_n8, eq30_e1490_d_n9, eq30_e1490_d_n10, eq30_e1490_d_n11, eq30_e1490_d_n12, eq30_e1490_d_n13, eq30_e1490_d_n14,) = {
    if (locals.var_guard753 != 0.0) {
        let eq30_e1486: f64 = (p.p28 * locals.var_devsign);
        let eq30_e1488: f64 = (eq30_e1486 * locals.var_isubdr);
        let eq30_e1488_d_n0: f64 = (eq30_e1486 * locals.var_isubdr_dn0);
        let eq30_e1488_d_n2: f64 = (eq30_e1486 * locals.var_isubdr_dn2);
        let eq30_e1488_d_n3: f64 = (eq30_e1486 * locals.var_isubdr_dn3);
        let eq30_e1488_d_n4: f64 = (eq30_e1486 * locals.var_isubdr_dn4);
        let eq30_e1488_d_n5: f64 = (eq30_e1486 * locals.var_isubdr_dn5);
        let eq30_e1488_d_n6: f64 = (eq30_e1486 * locals.var_isubdr_dn6);
        let eq30_e1488_d_n7: f64 = (eq30_e1486 * locals.var_isubdr_dn7);
        let eq30_e1488_d_n8: f64 = (eq30_e1486 * locals.var_isubdr_dn8);
        let eq30_e1488_d_n9: f64 = (eq30_e1486 * locals.var_isubdr_dn9);
        let eq30_e1488_d_n10: f64 = (eq30_e1486 * locals.var_isubdr_dn10);
        let eq30_e1488_d_n11: f64 = (eq30_e1486 * locals.var_isubdr_dn11);
        let eq30_e1488_d_n12: f64 = (eq30_e1486 * locals.var_isubdr_dn12);
        let eq30_e1488_d_n13: f64 = (eq30_e1486 * locals.var_isubdr_dn13);
        let eq30_e1488_d_n14: f64 = (eq30_e1486 * locals.var_isubdr_dn14);
        (eq30_e1488, eq30_e1488_d_n0, eq30_e1488_d_n2, eq30_e1488_d_n3, eq30_e1488_d_n4, eq30_e1488_d_n5, eq30_e1488_d_n6, eq30_e1488_d_n7, eq30_e1488_d_n8, eq30_e1488_d_n9, eq30_e1488_d_n10, eq30_e1488_d_n11, eq30_e1488_d_n12, eq30_e1488_d_n13, eq30_e1488_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1490;
        let eq30_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq30_node_derivatives: [f64; 14] = [eq30_e1490_d_n0, eq30_e1490_d_n2, eq30_e1490_d_n3, eq30_e1490_d_n4, eq30_e1490_d_n5, eq30_e1490_d_n6, eq30_e1490_d_n7, eq30_e1490_d_n8, eq30_e1490_d_n9, eq30_e1490_d_n10, eq30_e1490_d_n11, eq30_e1490_d_n12, eq30_e1490_d_n13, eq30_e1490_d_n14];
        let eq30_branch_derivative_indices: [usize; 0] = [];
        let eq30_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(11),
            multiplicity * (eq30_value),
            &eq30_node_derivative_indices,
            &eq30_node_derivatives,
            &eq30_branch_derivative_indices,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let (eq31_e1494, eq31_e1494_d_n0, eq31_e1494_d_n2, eq31_e1494_d_n3, eq31_e1494_d_n4, eq31_e1494_d_n5, eq31_e1494_d_n6, eq31_e1494_d_n7, eq31_e1494_d_n8, eq31_e1494_d_n9, eq31_e1494_d_n10, eq31_e1494_d_n11, eq31_e1494_d_n12, eq31_e1494_d_n13, eq31_e1494_d_n14,) = {
    if (locals.var_guard753 != 0.0) {
        (locals.var_igisl_1, locals.var_igisl_1_dn0, locals.var_igisl_1_dn2, locals.var_igisl_1_dn3, locals.var_igisl_1_dn4, locals.var_igisl_1_dn5, locals.var_igisl_1_dn6, locals.var_igisl_1_dn7, locals.var_igisl_1_dn8, locals.var_igisl_1_dn9, locals.var_igisl_1_dn10, locals.var_igisl_1_dn11, locals.var_igisl_1_dn12, locals.var_igisl_1_dn13, locals.var_igisl_1_dn14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e1494;
        let eq31_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq31_node_derivatives: [f64; 14] = [eq31_e1494_d_n0, eq31_e1494_d_n2, eq31_e1494_d_n3, eq31_e1494_d_n4, eq31_e1494_d_n5, eq31_e1494_d_n6, eq31_e1494_d_n7, eq31_e1494_d_n8, eq31_e1494_d_n9, eq31_e1494_d_n10, eq31_e1494_d_n11, eq31_e1494_d_n12, eq31_e1494_d_n13, eq31_e1494_d_n14];
        let eq31_branch_derivative_indices: [usize; 0] = [];
        let eq31_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(11),
            multiplicity * (eq31_value),
            &eq31_node_derivative_indices,
            &eq31_node_derivatives,
            &eq31_branch_derivative_indices,
            &eq31_branch_derivatives,
            multiplicity,
        );
        let (eq32_e1499, eq32_e1499_d_n0, eq32_e1499_d_n2, eq32_e1499_d_n3, eq32_e1499_d_n4, eq32_e1499_d_n5, eq32_e1499_d_n6, eq32_e1499_d_n7, eq32_e1499_d_n8, eq32_e1499_d_n9, eq32_e1499_d_n10, eq32_e1499_d_n11, eq32_e1499_d_n12, eq32_e1499_d_n13, eq32_e1499_d_n14,) = {
    if (locals.var_guard753 == 0.0) {
        (locals.var_igidl_1, locals.var_igidl_1_dn0, locals.var_igidl_1_dn2, locals.var_igidl_1_dn3, locals.var_igidl_1_dn4, locals.var_igidl_1_dn5, locals.var_igidl_1_dn6, locals.var_igidl_1_dn7, locals.var_igidl_1_dn8, locals.var_igidl_1_dn9, locals.var_igidl_1_dn10, locals.var_igidl_1_dn11, locals.var_igidl_1_dn12, locals.var_igidl_1_dn13, locals.var_igidl_1_dn14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e1499;
        let eq32_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq32_node_derivatives: [f64; 14] = [eq32_e1499_d_n0, eq32_e1499_d_n2, eq32_e1499_d_n3, eq32_e1499_d_n4, eq32_e1499_d_n5, eq32_e1499_d_n6, eq32_e1499_d_n7, eq32_e1499_d_n8, eq32_e1499_d_n9, eq32_e1499_d_n10, eq32_e1499_d_n11, eq32_e1499_d_n12, eq32_e1499_d_n13, eq32_e1499_d_n14];
        let eq32_branch_derivative_indices: [usize; 0] = [];
        let eq32_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(5),
            Some(11),
            multiplicity * (eq32_value),
            &eq32_node_derivative_indices,
            &eq32_node_derivatives,
            &eq32_branch_derivative_indices,
            &eq32_branch_derivatives,
            multiplicity,
        );
        let (eq33_e1506, eq33_e1506_d_n0, eq33_e1506_d_n2, eq33_e1506_d_n3, eq33_e1506_d_n4, eq33_e1506_d_n5, eq33_e1506_d_n6, eq33_e1506_d_n7, eq33_e1506_d_n8, eq33_e1506_d_n9, eq33_e1506_d_n10, eq33_e1506_d_n11, eq33_e1506_d_n12, eq33_e1506_d_n13, eq33_e1506_d_n14,) = {
    if (locals.var_guard753 == 0.0) {
        let eq33_e1504: f64 = (locals.var_isub + locals.var_igisl_1);
        let eq33_e1504_d_n0: f64 = (locals.var_isub_dn0 + locals.var_igisl_1_dn0);
        let eq33_e1504_d_n2: f64 = (locals.var_isub_dn2 + locals.var_igisl_1_dn2);
        let eq33_e1504_d_n3: f64 = (locals.var_isub_dn3 + locals.var_igisl_1_dn3);
        let eq33_e1504_d_n4: f64 = (locals.var_isub_dn4 + locals.var_igisl_1_dn4);
        let eq33_e1504_d_n5: f64 = (locals.var_isub_dn5 + locals.var_igisl_1_dn5);
        let eq33_e1504_d_n6: f64 = (locals.var_isub_dn6 + locals.var_igisl_1_dn6);
        let eq33_e1504_d_n7: f64 = (locals.var_isub_dn7 + locals.var_igisl_1_dn7);
        let eq33_e1504_d_n8: f64 = (locals.var_isub_dn8 + locals.var_igisl_1_dn8);
        let eq33_e1504_d_n9: f64 = (locals.var_isub_dn9 + locals.var_igisl_1_dn9);
        let eq33_e1504_d_n10: f64 = (locals.var_isub_dn10 + locals.var_igisl_1_dn10);
        let eq33_e1504_d_n11: f64 = (locals.var_isub_dn11 + locals.var_igisl_1_dn11);
        let eq33_e1504_d_n12: f64 = (locals.var_isub_dn12 + locals.var_igisl_1_dn12);
        let eq33_e1504_d_n13: f64 = (locals.var_isub_dn13 + locals.var_igisl_1_dn13);
        let eq33_e1504_d_n14: f64 = (locals.var_isub_dn14 + locals.var_igisl_1_dn14);
        (eq33_e1504, eq33_e1504_d_n0, eq33_e1504_d_n2, eq33_e1504_d_n3, eq33_e1504_d_n4, eq33_e1504_d_n5, eq33_e1504_d_n6, eq33_e1504_d_n7, eq33_e1504_d_n8, eq33_e1504_d_n9, eq33_e1504_d_n10, eq33_e1504_d_n11, eq33_e1504_d_n12, eq33_e1504_d_n13, eq33_e1504_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1506;
        let eq33_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq33_node_derivatives: [f64; 14] = [eq33_e1506_d_n0, eq33_e1506_d_n2, eq33_e1506_d_n3, eq33_e1506_d_n4, eq33_e1506_d_n5, eq33_e1506_d_n6, eq33_e1506_d_n7, eq33_e1506_d_n8, eq33_e1506_d_n9, eq33_e1506_d_n10, eq33_e1506_d_n11, eq33_e1506_d_n12, eq33_e1506_d_n13, eq33_e1506_d_n14];
        let eq33_branch_derivative_indices: [usize; 0] = [];
        let eq33_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(7),
            Some(11),
            multiplicity * (eq33_value),
            &eq33_node_derivative_indices,
            &eq33_node_derivatives,
            &eq33_branch_derivative_indices,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let (eq34_e1514, eq34_e1514_d_n0, eq34_e1514_d_n2, eq34_e1514_d_n3, eq34_e1514_d_n4, eq34_e1514_d_n5, eq34_e1514_d_n6, eq34_e1514_d_n7, eq34_e1514_d_n8, eq34_e1514_d_n9, eq34_e1514_d_n10, eq34_e1514_d_n11, eq34_e1514_d_n12, eq34_e1514_d_n13, eq34_e1514_d_n14,) = {
    if (locals.var_guard754 != 0.0) {
        let eq34_e1510: f64 = (p.p28 * (nv0 - nv6));
        let eq34_e1512: f64 = (eq34_e1510 * locals.var_gdpr);
        let eq34_e1512_d_n0: f64 = ((p.p28 * locals.var_gdpr) + (eq34_e1510 * locals.var_gdpr_dn0));
        let eq34_e1512_d_n2: f64 = (eq34_e1510 * locals.var_gdpr_dn2);
        let eq34_e1512_d_n3: f64 = (eq34_e1510 * locals.var_gdpr_dn3);
        let eq34_e1512_d_n4: f64 = (eq34_e1510 * locals.var_gdpr_dn4);
        let eq34_e1512_d_n5: f64 = (eq34_e1510 * locals.var_gdpr_dn5);
        let eq34_e1512_d_n6: f64 = (((-p.p28) * locals.var_gdpr) + (eq34_e1510 * locals.var_gdpr_dn6));
        let eq34_e1512_d_n7: f64 = (eq34_e1510 * locals.var_gdpr_dn7);
        let eq34_e1512_d_n8: f64 = (eq34_e1510 * locals.var_gdpr_dn8);
        let eq34_e1512_d_n9: f64 = (eq34_e1510 * locals.var_gdpr_dn9);
        let eq34_e1512_d_n10: f64 = (eq34_e1510 * locals.var_gdpr_dn10);
        let eq34_e1512_d_n11: f64 = (eq34_e1510 * locals.var_gdpr_dn11);
        let eq34_e1512_d_n12: f64 = (eq34_e1510 * locals.var_gdpr_dn12);
        let eq34_e1512_d_n13: f64 = (eq34_e1510 * locals.var_gdpr_dn13);
        let eq34_e1512_d_n14: f64 = (eq34_e1510 * locals.var_gdpr_dn14);
        (eq34_e1512, eq34_e1512_d_n0, eq34_e1512_d_n2, eq34_e1512_d_n3, eq34_e1512_d_n4, eq34_e1512_d_n5, eq34_e1512_d_n6, eq34_e1512_d_n7, eq34_e1512_d_n8, eq34_e1512_d_n9, eq34_e1512_d_n10, eq34_e1512_d_n11, eq34_e1512_d_n12, eq34_e1512_d_n13, eq34_e1512_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e1514;
        let eq34_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq34_node_derivatives: [f64; 14] = [eq34_e1514_d_n0, eq34_e1514_d_n2, eq34_e1514_d_n3, eq34_e1514_d_n4, eq34_e1514_d_n5, eq34_e1514_d_n6, eq34_e1514_d_n7, eq34_e1514_d_n8, eq34_e1514_d_n9, eq34_e1514_d_n10, eq34_e1514_d_n11, eq34_e1514_d_n12, eq34_e1514_d_n13, eq34_e1514_d_n14];
        let eq34_branch_derivative_indices: [usize; 0] = [];
        let eq34_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(0),
            Some(6),
            multiplicity * (eq34_value),
            &eq34_node_derivative_indices,
            &eq34_node_derivatives,
            &eq34_branch_derivative_indices,
            &eq34_branch_derivatives,
            multiplicity,
        );
        let (eq36_e1534, eq36_e1534_d_n0, eq36_e1534_d_n2, eq36_e1534_d_n3, eq36_e1534_d_n4, eq36_e1534_d_n5, eq36_e1534_d_n6, eq36_e1534_d_n7, eq36_e1534_d_n8, eq36_e1534_d_n9, eq36_e1534_d_n10, eq36_e1534_d_n11, eq36_e1534_d_n12, eq36_e1534_d_n13, eq36_e1534_d_n14,) = {
    if ((locals.var_guard754 != 0.0) && (locals.var_guard755 != 0.0)) {
        let eq36_e1530: f64 = (p.p28 * (nv6 - nv5));
        let eq36_e1532: f64 = (eq36_e1530 * locals.var_gdrift_d);
        let eq36_e1532_d_n0: f64 = (eq36_e1530 * locals.var_gdrift_d_dn0);
        let eq36_e1532_d_n2: f64 = (eq36_e1530 * locals.var_gdrift_d_dn2);
        let eq36_e1532_d_n3: f64 = (eq36_e1530 * locals.var_gdrift_d_dn3);
        let eq36_e1532_d_n4: f64 = (eq36_e1530 * locals.var_gdrift_d_dn4);
        let eq36_e1532_d_n5: f64 = (((-p.p28) * locals.var_gdrift_d) + (eq36_e1530 * locals.var_gdrift_d_dn5));
        let eq36_e1532_d_n6: f64 = ((p.p28 * locals.var_gdrift_d) + (eq36_e1530 * locals.var_gdrift_d_dn6));
        let eq36_e1532_d_n7: f64 = (eq36_e1530 * locals.var_gdrift_d_dn7);
        let eq36_e1532_d_n8: f64 = (eq36_e1530 * locals.var_gdrift_d_dn8);
        let eq36_e1532_d_n9: f64 = (eq36_e1530 * locals.var_gdrift_d_dn9);
        let eq36_e1532_d_n10: f64 = (eq36_e1530 * locals.var_gdrift_d_dn10);
        let eq36_e1532_d_n11: f64 = (eq36_e1530 * locals.var_gdrift_d_dn11);
        let eq36_e1532_d_n12: f64 = (eq36_e1530 * locals.var_gdrift_d_dn12);
        let eq36_e1532_d_n13: f64 = (eq36_e1530 * locals.var_gdrift_d_dn13);
        let eq36_e1532_d_n14: f64 = (eq36_e1530 * locals.var_gdrift_d_dn14);
        (eq36_e1532, eq36_e1532_d_n0, eq36_e1532_d_n2, eq36_e1532_d_n3, eq36_e1532_d_n4, eq36_e1532_d_n5, eq36_e1532_d_n6, eq36_e1532_d_n7, eq36_e1532_d_n8, eq36_e1532_d_n9, eq36_e1532_d_n10, eq36_e1532_d_n11, eq36_e1532_d_n12, eq36_e1532_d_n13, eq36_e1532_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e1534;
        let eq36_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq36_node_derivatives: [f64; 14] = [eq36_e1534_d_n0, eq36_e1534_d_n2, eq36_e1534_d_n3, eq36_e1534_d_n4, eq36_e1534_d_n5, eq36_e1534_d_n6, eq36_e1534_d_n7, eq36_e1534_d_n8, eq36_e1534_d_n9, eq36_e1534_d_n10, eq36_e1534_d_n11, eq36_e1534_d_n12, eq36_e1534_d_n13, eq36_e1534_d_n14];
        let eq36_branch_derivative_indices: [usize; 0] = [];
        let eq36_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(5),
            multiplicity * (eq36_value),
            &eq36_node_derivative_indices,
            &eq36_node_derivatives,
            &eq36_branch_derivative_indices,
            &eq36_branch_derivatives,
            multiplicity,
        );
        let (eq39_e1572,) = {
    if ((locals.var_guard754 != 0.0) && (locals.var_guard755 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq39_value: f64 = eq39_e1572;
        stamper.stamp_potential_const_local(
            1,
            eq39_value,
        );
        let (eq40_e1577,) = {
    if (locals.var_guard754 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq40_value: f64 = eq40_e1577;
        stamper.stamp_potential_const_local(
            2,
            eq40_value,
        );
        let (eq41_e1582,) = {
    if (locals.var_guard754 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq41_value: f64 = eq41_e1582;
        stamper.stamp_potential_const_local(
            3,
            eq41_value,
        );
        let (eq42_e1590, eq42_e1590_d_n0, eq42_e1590_d_n2, eq42_e1590_d_n3, eq42_e1590_d_n4, eq42_e1590_d_n5, eq42_e1590_d_n6, eq42_e1590_d_n7, eq42_e1590_d_n8, eq42_e1590_d_n9, eq42_e1590_d_n10, eq42_e1590_d_n11, eq42_e1590_d_n12, eq42_e1590_d_n13, eq42_e1590_d_n14,) = {
    if (locals.var_guard756 != 0.0) {
        let eq42_e1586: f64 = (p.p28 * (nv2 - nv8));
        let eq42_e1588: f64 = (eq42_e1586 * locals.var_gspr);
        let eq42_e1588_d_n0: f64 = (eq42_e1586 * locals.var_gspr_dn0);
        let eq42_e1588_d_n2: f64 = ((p.p28 * locals.var_gspr) + (eq42_e1586 * locals.var_gspr_dn2));
        let eq42_e1588_d_n3: f64 = (eq42_e1586 * locals.var_gspr_dn3);
        let eq42_e1588_d_n4: f64 = (eq42_e1586 * locals.var_gspr_dn4);
        let eq42_e1588_d_n5: f64 = (eq42_e1586 * locals.var_gspr_dn5);
        let eq42_e1588_d_n6: f64 = (eq42_e1586 * locals.var_gspr_dn6);
        let eq42_e1588_d_n7: f64 = (eq42_e1586 * locals.var_gspr_dn7);
        let eq42_e1588_d_n8: f64 = (((-p.p28) * locals.var_gspr) + (eq42_e1586 * locals.var_gspr_dn8));
        let eq42_e1588_d_n9: f64 = (eq42_e1586 * locals.var_gspr_dn9);
        let eq42_e1588_d_n10: f64 = (eq42_e1586 * locals.var_gspr_dn10);
        let eq42_e1588_d_n11: f64 = (eq42_e1586 * locals.var_gspr_dn11);
        let eq42_e1588_d_n12: f64 = (eq42_e1586 * locals.var_gspr_dn12);
        let eq42_e1588_d_n13: f64 = (eq42_e1586 * locals.var_gspr_dn13);
        let eq42_e1588_d_n14: f64 = (eq42_e1586 * locals.var_gspr_dn14);
        (eq42_e1588, eq42_e1588_d_n0, eq42_e1588_d_n2, eq42_e1588_d_n3, eq42_e1588_d_n4, eq42_e1588_d_n5, eq42_e1588_d_n6, eq42_e1588_d_n7, eq42_e1588_d_n8, eq42_e1588_d_n9, eq42_e1588_d_n10, eq42_e1588_d_n11, eq42_e1588_d_n12, eq42_e1588_d_n13, eq42_e1588_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq42_value: f64 = eq42_e1590;
        let eq42_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq42_node_derivatives: [f64; 14] = [eq42_e1590_d_n0, eq42_e1590_d_n2, eq42_e1590_d_n3, eq42_e1590_d_n4, eq42_e1590_d_n5, eq42_e1590_d_n6, eq42_e1590_d_n7, eq42_e1590_d_n8, eq42_e1590_d_n9, eq42_e1590_d_n10, eq42_e1590_d_n11, eq42_e1590_d_n12, eq42_e1590_d_n13, eq42_e1590_d_n14];
        let eq42_branch_derivative_indices: [usize; 0] = [];
        let eq42_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(2),
            Some(8),
            multiplicity * (eq42_value),
            &eq42_node_derivative_indices,
            &eq42_node_derivatives,
            &eq42_branch_derivative_indices,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let (eq44_e1610, eq44_e1610_d_n0, eq44_e1610_d_n2, eq44_e1610_d_n3, eq44_e1610_d_n4, eq44_e1610_d_n5, eq44_e1610_d_n6, eq44_e1610_d_n7, eq44_e1610_d_n8, eq44_e1610_d_n9, eq44_e1610_d_n10, eq44_e1610_d_n11, eq44_e1610_d_n12, eq44_e1610_d_n13, eq44_e1610_d_n14,) = {
    if ((locals.var_guard756 != 0.0) && (locals.var_guard757 != 0.0)) {
        let eq44_e1606: f64 = (p.p28 * (nv8 - nv7));
        let eq44_e1608: f64 = (eq44_e1606 * locals.var_gdrift_s);
        let eq44_e1608_d_n0: f64 = (eq44_e1606 * locals.var_gdrift_s_dn0);
        let eq44_e1608_d_n2: f64 = (eq44_e1606 * locals.var_gdrift_s_dn2);
        let eq44_e1608_d_n3: f64 = (eq44_e1606 * locals.var_gdrift_s_dn3);
        let eq44_e1608_d_n4: f64 = (eq44_e1606 * locals.var_gdrift_s_dn4);
        let eq44_e1608_d_n5: f64 = (eq44_e1606 * locals.var_gdrift_s_dn5);
        let eq44_e1608_d_n6: f64 = (eq44_e1606 * locals.var_gdrift_s_dn6);
        let eq44_e1608_d_n7: f64 = (((-p.p28) * locals.var_gdrift_s) + (eq44_e1606 * locals.var_gdrift_s_dn7));
        let eq44_e1608_d_n8: f64 = ((p.p28 * locals.var_gdrift_s) + (eq44_e1606 * locals.var_gdrift_s_dn8));
        let eq44_e1608_d_n9: f64 = (eq44_e1606 * locals.var_gdrift_s_dn9);
        let eq44_e1608_d_n10: f64 = (eq44_e1606 * locals.var_gdrift_s_dn10);
        let eq44_e1608_d_n11: f64 = (eq44_e1606 * locals.var_gdrift_s_dn11);
        let eq44_e1608_d_n12: f64 = (eq44_e1606 * locals.var_gdrift_s_dn12);
        let eq44_e1608_d_n13: f64 = (eq44_e1606 * locals.var_gdrift_s_dn13);
        let eq44_e1608_d_n14: f64 = (eq44_e1606 * locals.var_gdrift_s_dn14);
        (eq44_e1608, eq44_e1608_d_n0, eq44_e1608_d_n2, eq44_e1608_d_n3, eq44_e1608_d_n4, eq44_e1608_d_n5, eq44_e1608_d_n6, eq44_e1608_d_n7, eq44_e1608_d_n8, eq44_e1608_d_n9, eq44_e1608_d_n10, eq44_e1608_d_n11, eq44_e1608_d_n12, eq44_e1608_d_n13, eq44_e1608_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq44_value: f64 = eq44_e1610;
        let eq44_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq44_node_derivatives: [f64; 14] = [eq44_e1610_d_n0, eq44_e1610_d_n2, eq44_e1610_d_n3, eq44_e1610_d_n4, eq44_e1610_d_n5, eq44_e1610_d_n6, eq44_e1610_d_n7, eq44_e1610_d_n8, eq44_e1610_d_n9, eq44_e1610_d_n10, eq44_e1610_d_n11, eq44_e1610_d_n12, eq44_e1610_d_n13, eq44_e1610_d_n14];
        let eq44_branch_derivative_indices: [usize; 0] = [];
        let eq44_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq44_value),
            &eq44_node_derivative_indices,
            &eq44_node_derivatives,
            &eq44_branch_derivative_indices,
            &eq44_branch_derivatives,
            multiplicity,
        );
        let (eq47_e1648,) = {
    if ((locals.var_guard756 != 0.0) && (locals.var_guard757 == 0.0)) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq47_value: f64 = eq47_e1648;
        stamper.stamp_potential_const_local(
            4,
            eq47_value,
        );
        let (eq48_e1653,) = {
    if (locals.var_guard756 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq48_value: f64 = eq48_e1653;
        stamper.stamp_potential_const_local(
            5,
            eq48_value,
        );
        let (eq49_e1658,) = {
    if (locals.var_guard756 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq49_value: f64 = eq49_e1658;
        stamper.stamp_potential_const_local(
            6,
            eq49_value,
        );
        let (eq51_e1671, eq51_e1671_d_n0, eq51_e1671_d_n1, eq51_e1671_d_n2, eq51_e1671_d_n3, eq51_e1671_d_n4, eq51_e1671_d_n5, eq51_e1671_d_n6, eq51_e1671_d_n7, eq51_e1671_d_n8, eq51_e1671_d_n9, eq51_e1671_d_n10, eq51_e1671_d_n11, eq51_e1671_d_n12, eq51_e1671_d_n13, eq51_e1671_d_n14,) = {
    if (locals.var_guard758 == 0.0) {
        let eq51_e1667: f64 = (p.p28 * (nv1 - nv10));
        let eq51_e1669: f64 = (eq51_e1667 * locals.var_ggate);
        let eq51_e1669_d_n0: f64 = (eq51_e1667 * locals.var_ggate_dn0);
        let eq51_e1669_d_n1: f64 = (p.p28 * locals.var_ggate);
        let eq51_e1669_d_n2: f64 = (eq51_e1667 * locals.var_ggate_dn2);
        let eq51_e1669_d_n3: f64 = (eq51_e1667 * locals.var_ggate_dn3);
        let eq51_e1669_d_n4: f64 = (eq51_e1667 * locals.var_ggate_dn4);
        let eq51_e1669_d_n5: f64 = (eq51_e1667 * locals.var_ggate_dn5);
        let eq51_e1669_d_n6: f64 = (eq51_e1667 * locals.var_ggate_dn6);
        let eq51_e1669_d_n7: f64 = (eq51_e1667 * locals.var_ggate_dn7);
        let eq51_e1669_d_n8: f64 = (eq51_e1667 * locals.var_ggate_dn8);
        let eq51_e1669_d_n9: f64 = (eq51_e1667 * locals.var_ggate_dn9);
        let eq51_e1669_d_n10: f64 = (((-p.p28) * locals.var_ggate) + (eq51_e1667 * locals.var_ggate_dn10));
        let eq51_e1669_d_n11: f64 = (eq51_e1667 * locals.var_ggate_dn11);
        let eq51_e1669_d_n12: f64 = (eq51_e1667 * locals.var_ggate_dn12);
        let eq51_e1669_d_n13: f64 = (eq51_e1667 * locals.var_ggate_dn13);
        let eq51_e1669_d_n14: f64 = (eq51_e1667 * locals.var_ggate_dn14);
        (eq51_e1669, eq51_e1669_d_n0, eq51_e1669_d_n1, eq51_e1669_d_n2, eq51_e1669_d_n3, eq51_e1669_d_n4, eq51_e1669_d_n5, eq51_e1669_d_n6, eq51_e1669_d_n7, eq51_e1669_d_n8, eq51_e1669_d_n9, eq51_e1669_d_n10, eq51_e1669_d_n11, eq51_e1669_d_n12, eq51_e1669_d_n13, eq51_e1669_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq51_value: f64 = eq51_e1671;
        let eq51_node_derivative_indices: [usize; 15] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq51_node_derivatives: [f64; 15] = [eq51_e1671_d_n0, eq51_e1671_d_n1, eq51_e1671_d_n2, eq51_e1671_d_n3, eq51_e1671_d_n4, eq51_e1671_d_n5, eq51_e1671_d_n6, eq51_e1671_d_n7, eq51_e1671_d_n8, eq51_e1671_d_n9, eq51_e1671_d_n10, eq51_e1671_d_n11, eq51_e1671_d_n12, eq51_e1671_d_n13, eq51_e1671_d_n14];
        let eq51_branch_derivative_indices: [usize; 0] = [];
        let eq51_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(1),
            Some(10),
            multiplicity * (eq51_value),
            &eq51_node_derivative_indices,
            &eq51_node_derivatives,
            &eq51_branch_derivative_indices,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let (eq53_e1690, eq53_e1690_d_n0, eq53_e1690_d_n2, eq53_e1690_d_n3, eq53_e1690_d_n4, eq53_e1690_d_n5, eq53_e1690_d_n6, eq53_e1690_d_n7, eq53_e1690_d_n8, eq53_e1690_d_n9, eq53_e1690_d_n10, eq53_e1690_d_n11, eq53_e1690_d_n12, eq53_e1690_d_n13, eq53_e1690_d_n14,) = {
    if (locals.var_guard762 != 0.0) {
        let eq53_e1686: f64 = ((nv10 - nv9) * p.p28);
        let eq53_e1688: f64 = (eq53_e1686 * locals.var_gcrg);
        let eq53_e1688_d_n0: f64 = (eq53_e1686 * locals.var_gcrg_dn0);
        let eq53_e1688_d_n2: f64 = (eq53_e1686 * locals.var_gcrg_dn2);
        let eq53_e1688_d_n3: f64 = (eq53_e1686 * locals.var_gcrg_dn3);
        let eq53_e1688_d_n4: f64 = (eq53_e1686 * locals.var_gcrg_dn4);
        let eq53_e1688_d_n5: f64 = (eq53_e1686 * locals.var_gcrg_dn5);
        let eq53_e1688_d_n6: f64 = (eq53_e1686 * locals.var_gcrg_dn6);
        let eq53_e1688_d_n7: f64 = (eq53_e1686 * locals.var_gcrg_dn7);
        let eq53_e1688_d_n8: f64 = (eq53_e1686 * locals.var_gcrg_dn8);
        let eq53_e1688_d_n9: f64 = (((-p.p28) * locals.var_gcrg) + (eq53_e1686 * locals.var_gcrg_dn9));
        let eq53_e1688_d_n10: f64 = ((p.p28 * locals.var_gcrg) + (eq53_e1686 * locals.var_gcrg_dn10));
        let eq53_e1688_d_n11: f64 = (eq53_e1686 * locals.var_gcrg_dn11);
        let eq53_e1688_d_n12: f64 = (eq53_e1686 * locals.var_gcrg_dn12);
        let eq53_e1688_d_n13: f64 = (eq53_e1686 * locals.var_gcrg_dn13);
        let eq53_e1688_d_n14: f64 = (eq53_e1686 * locals.var_gcrg_dn14);
        (eq53_e1688, eq53_e1688_d_n0, eq53_e1688_d_n2, eq53_e1688_d_n3, eq53_e1688_d_n4, eq53_e1688_d_n5, eq53_e1688_d_n6, eq53_e1688_d_n7, eq53_e1688_d_n8, eq53_e1688_d_n9, eq53_e1688_d_n10, eq53_e1688_d_n11, eq53_e1688_d_n12, eq53_e1688_d_n13, eq53_e1688_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e1690;
        let eq53_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq53_node_derivatives: [f64; 14] = [eq53_e1690_d_n0, eq53_e1690_d_n2, eq53_e1690_d_n3, eq53_e1690_d_n4, eq53_e1690_d_n5, eq53_e1690_d_n6, eq53_e1690_d_n7, eq53_e1690_d_n8, eq53_e1690_d_n9, eq53_e1690_d_n10, eq53_e1690_d_n11, eq53_e1690_d_n12, eq53_e1690_d_n13, eq53_e1690_d_n14];
        let eq53_branch_derivative_indices: [usize; 0] = [];
        let eq53_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(10),
            Some(9),
            multiplicity * (eq53_value),
            &eq53_node_derivative_indices,
            &eq53_node_derivatives,
            &eq53_branch_derivative_indices,
            &eq53_branch_derivatives,
            multiplicity,
        );
        let (eq55_e1708, eq55_e1708_d_n0, eq55_e1708_d_n2, eq55_e1708_d_n3, eq55_e1708_d_n4, eq55_e1708_d_n5, eq55_e1708_d_n6, eq55_e1708_d_n7, eq55_e1708_d_n8, eq55_e1708_d_n9, eq55_e1708_d_n10, eq55_e1708_d_n11, eq55_e1708_d_n12, eq55_e1708_d_n13, eq55_e1708_d_n14,) = {
    if (locals.var_guard763 != 0.0) {
        let eq55_e1699: f64 = (locals.var_deltemp1 * locals.var_gth);
        let eq55_e1699_d_n4: f64 = (locals.var_deltemp1_dn4 * locals.var_gth);
        let eq55_e1702: f64 = (locals.var_deltemp1 * locals.var_cth);
        let eq55_e1702_d_n4: f64 = (locals.var_deltemp1_dn4 * locals.var_cth);
        let eq55_e1703: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq55_e1702);
        let eq55_e1704: f64 = (eq55_e1699 + eq55_e1703);
        let eq55_e1704_d_n4: f64 = (eq55_e1699_d_n4 + (eq55_e1702_d_n4 * ddt_scale));
        let eq55_e1706: f64 = (eq55_e1704 - locals.var_pdiss);
        let eq55_e1706_d_n4: f64 = (eq55_e1704_d_n4 - locals.var_pdiss_dn4);
        (eq55_e1706, (-locals.var_pdiss_dn0), (-locals.var_pdiss_dn2), (-locals.var_pdiss_dn3), eq55_e1706_d_n4, (-locals.var_pdiss_dn5), (-locals.var_pdiss_dn6), (-locals.var_pdiss_dn7), (-locals.var_pdiss_dn8), (-locals.var_pdiss_dn9), (-locals.var_pdiss_dn10), (-locals.var_pdiss_dn11), (-locals.var_pdiss_dn12), (-locals.var_pdiss_dn13), (-locals.var_pdiss_dn14),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e1708;
        let eq55_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq55_node_derivatives: [f64; 14] = [eq55_e1708_d_n0, eq55_e1708_d_n2, eq55_e1708_d_n3, eq55_e1708_d_n4, eq55_e1708_d_n5, eq55_e1708_d_n6, eq55_e1708_d_n7, eq55_e1708_d_n8, eq55_e1708_d_n9, eq55_e1708_d_n10, eq55_e1708_d_n11, eq55_e1708_d_n12, eq55_e1708_d_n13, eq55_e1708_d_n14];
        let eq55_branch_derivative_indices: [usize; 0] = [];
        let eq55_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(4),
            None,
            multiplicity * (eq55_value),
            &eq55_node_derivative_indices,
            &eq55_node_derivatives,
            &eq55_branch_derivative_indices,
            &eq55_branch_derivatives,
            multiplicity,
        );
        let (eq70_e1832, eq70_e1832_d_n0, eq70_e1832_d_n2, eq70_e1832_d_n3, eq70_e1832_d_n4, eq70_e1832_d_n5, eq70_e1832_d_n6, eq70_e1832_d_n7, eq70_e1832_d_n8, eq70_e1832_d_n9, eq70_e1832_d_n10, eq70_e1832_d_n11, eq70_e1832_d_n12, eq70_e1832_d_n13, eq70_e1832_d_n14,) = {
    if (locals.var_guard769 != 0.0) {
        let eq70_e1822: f64 = (locals.var_devsign * p.p28);
        let eq70_e1824: f64 = (eq70_e1822 * locals.var_ibs);
        let eq70_e1824_d_n0: f64 = (eq70_e1822 * locals.var_ibs_dn0);
        let eq70_e1824_d_n2: f64 = (eq70_e1822 * locals.var_ibs_dn2);
        let eq70_e1824_d_n3: f64 = (eq70_e1822 * locals.var_ibs_dn3);
        let eq70_e1824_d_n4: f64 = (eq70_e1822 * locals.var_ibs_dn4);
        let eq70_e1824_d_n5: f64 = (eq70_e1822 * locals.var_ibs_dn5);
        let eq70_e1824_d_n6: f64 = (eq70_e1822 * locals.var_ibs_dn6);
        let eq70_e1824_d_n7: f64 = (eq70_e1822 * locals.var_ibs_dn7);
        let eq70_e1824_d_n8: f64 = (eq70_e1822 * locals.var_ibs_dn8);
        let eq70_e1824_d_n9: f64 = (eq70_e1822 * locals.var_ibs_dn9);
        let eq70_e1824_d_n10: f64 = (eq70_e1822 * locals.var_ibs_dn10);
        let eq70_e1824_d_n11: f64 = (eq70_e1822 * locals.var_ibs_dn11);
        let eq70_e1824_d_n12: f64 = (eq70_e1822 * locals.var_ibs_dn12);
        let eq70_e1824_d_n13: f64 = (eq70_e1822 * locals.var_ibs_dn13);
        let eq70_e1824_d_n14: f64 = (eq70_e1822 * locals.var_ibs_dn14);
        let eq70_e1827: f64 = ((nv12 - nv7) * p.p28);
        let eq70_e1829: f64 = (eq70_e1827 * locals.var_gmin);
        let eq70_e1829_d_n7: f64 = ((-p.p28) * locals.var_gmin);
        let eq70_e1829_d_n12: f64 = (p.p28 * locals.var_gmin);
        let eq70_e1830: f64 = (eq70_e1824 + eq70_e1829);
        let eq70_e1830_d_n7: f64 = (eq70_e1824_d_n7 + eq70_e1829_d_n7);
        let eq70_e1830_d_n12: f64 = (eq70_e1824_d_n12 + eq70_e1829_d_n12);
        (eq70_e1830, eq70_e1824_d_n0, eq70_e1824_d_n2, eq70_e1824_d_n3, eq70_e1824_d_n4, eq70_e1824_d_n5, eq70_e1824_d_n6, eq70_e1830_d_n7, eq70_e1824_d_n8, eq70_e1824_d_n9, eq70_e1824_d_n10, eq70_e1824_d_n11, eq70_e1830_d_n12, eq70_e1824_d_n13, eq70_e1824_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq70_value: f64 = eq70_e1832;
        let eq70_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq70_node_derivatives: [f64; 14] = [eq70_e1832_d_n0, eq70_e1832_d_n2, eq70_e1832_d_n3, eq70_e1832_d_n4, eq70_e1832_d_n5, eq70_e1832_d_n6, eq70_e1832_d_n7, eq70_e1832_d_n8, eq70_e1832_d_n9, eq70_e1832_d_n10, eq70_e1832_d_n11, eq70_e1832_d_n12, eq70_e1832_d_n13, eq70_e1832_d_n14];
        let eq70_branch_derivative_indices: [usize; 0] = [];
        let eq70_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq70_value),
            &eq70_node_derivative_indices,
            &eq70_node_derivatives,
            &eq70_branch_derivative_indices,
            &eq70_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
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
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let (eq71_e1841, eq71_e1841_d_n0, eq71_e1841_d_n2, eq71_e1841_d_n3, eq71_e1841_d_n4, eq71_e1841_d_n5, eq71_e1841_d_n6, eq71_e1841_d_n7, eq71_e1841_d_n8, eq71_e1841_d_n9, eq71_e1841_d_n10, eq71_e1841_d_n11, eq71_e1841_d_n12, eq71_e1841_d_n13, eq71_e1841_d_n14,) = {
    if (locals.var_guard769 != 0.0) {
        let eq71_e1837: f64 = (p.p29 * locals.var_qbsj);
        let eq71_e1837_d_n0: f64 = (p.p29 * locals.var_qbsj_dn0);
        let eq71_e1837_d_n2: f64 = (p.p29 * locals.var_qbsj_dn2);
        let eq71_e1837_d_n3: f64 = (p.p29 * locals.var_qbsj_dn3);
        let eq71_e1837_d_n4: f64 = (p.p29 * locals.var_qbsj_dn4);
        let eq71_e1837_d_n5: f64 = (p.p29 * locals.var_qbsj_dn5);
        let eq71_e1837_d_n6: f64 = (p.p29 * locals.var_qbsj_dn6);
        let eq71_e1837_d_n7: f64 = (p.p29 * locals.var_qbsj_dn7);
        let eq71_e1837_d_n8: f64 = (p.p29 * locals.var_qbsj_dn8);
        let eq71_e1837_d_n9: f64 = (p.p29 * locals.var_qbsj_dn9);
        let eq71_e1837_d_n10: f64 = (p.p29 * locals.var_qbsj_dn10);
        let eq71_e1837_d_n11: f64 = (p.p29 * locals.var_qbsj_dn11);
        let eq71_e1837_d_n12: f64 = (p.p29 * locals.var_qbsj_dn12);
        let eq71_e1837_d_n13: f64 = (p.p29 * locals.var_qbsj_dn13);
        let eq71_e1837_d_n14: f64 = (p.p29 * locals.var_qbsj_dn14);
        let eq71_e1838: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq71_e1837);
        let eq71_e1839: f64 = (locals.var_devsign * eq71_e1838);
        let eq71_e1839_d_n0: f64 = (locals.var_devsign * (eq71_e1837_d_n0 * ddt_scale));
        let eq71_e1839_d_n2: f64 = (locals.var_devsign * (eq71_e1837_d_n2 * ddt_scale));
        let eq71_e1839_d_n3: f64 = (locals.var_devsign * (eq71_e1837_d_n3 * ddt_scale));
        let eq71_e1839_d_n4: f64 = (locals.var_devsign * (eq71_e1837_d_n4 * ddt_scale));
        let eq71_e1839_d_n5: f64 = (locals.var_devsign * (eq71_e1837_d_n5 * ddt_scale));
        let eq71_e1839_d_n6: f64 = (locals.var_devsign * (eq71_e1837_d_n6 * ddt_scale));
        let eq71_e1839_d_n7: f64 = (locals.var_devsign * (eq71_e1837_d_n7 * ddt_scale));
        let eq71_e1839_d_n8: f64 = (locals.var_devsign * (eq71_e1837_d_n8 * ddt_scale));
        let eq71_e1839_d_n9: f64 = (locals.var_devsign * (eq71_e1837_d_n9 * ddt_scale));
        let eq71_e1839_d_n10: f64 = (locals.var_devsign * (eq71_e1837_d_n10 * ddt_scale));
        let eq71_e1839_d_n11: f64 = (locals.var_devsign * (eq71_e1837_d_n11 * ddt_scale));
        let eq71_e1839_d_n12: f64 = (locals.var_devsign * (eq71_e1837_d_n12 * ddt_scale));
        let eq71_e1839_d_n13: f64 = (locals.var_devsign * (eq71_e1837_d_n13 * ddt_scale));
        let eq71_e1839_d_n14: f64 = (locals.var_devsign * (eq71_e1837_d_n14 * ddt_scale));
        (eq71_e1839, eq71_e1839_d_n0, eq71_e1839_d_n2, eq71_e1839_d_n3, eq71_e1839_d_n4, eq71_e1839_d_n5, eq71_e1839_d_n6, eq71_e1839_d_n7, eq71_e1839_d_n8, eq71_e1839_d_n9, eq71_e1839_d_n10, eq71_e1839_d_n11, eq71_e1839_d_n12, eq71_e1839_d_n13, eq71_e1839_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq71_value: f64 = eq71_e1841;
        let eq71_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq71_node_derivatives: [f64; 14] = [eq71_e1841_d_n0, eq71_e1841_d_n2, eq71_e1841_d_n3, eq71_e1841_d_n4, eq71_e1841_d_n5, eq71_e1841_d_n6, eq71_e1841_d_n7, eq71_e1841_d_n8, eq71_e1841_d_n9, eq71_e1841_d_n10, eq71_e1841_d_n11, eq71_e1841_d_n12, eq71_e1841_d_n13, eq71_e1841_d_n14];
        let eq71_branch_derivative_indices: [usize; 0] = [];
        let eq71_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq71_value),
            &eq71_node_derivative_indices,
            &eq71_node_derivatives,
            &eq71_branch_derivative_indices,
            &eq71_branch_derivatives,
            multiplicity,
        );
        let (eq72_e1857, eq72_e1857_d_n0, eq72_e1857_d_n2, eq72_e1857_d_n3, eq72_e1857_d_n4, eq72_e1857_d_n5, eq72_e1857_d_n6, eq72_e1857_d_n7, eq72_e1857_d_n8, eq72_e1857_d_n9, eq72_e1857_d_n10, eq72_e1857_d_n11, eq72_e1857_d_n12, eq72_e1857_d_n13, eq72_e1857_d_n14,) = {
    if ((locals.var_guard769 != 0.0) && (locals.var_guard770 != 0.0)) {
        let eq72_e1847: f64 = (locals.var_devsign * p.p28);
        let eq72_e1849: f64 = (eq72_e1847 * locals.var_ibd);
        let eq72_e1849_d_n0: f64 = (eq72_e1847 * locals.var_ibd_dn0);
        let eq72_e1849_d_n2: f64 = (eq72_e1847 * locals.var_ibd_dn2);
        let eq72_e1849_d_n3: f64 = (eq72_e1847 * locals.var_ibd_dn3);
        let eq72_e1849_d_n4: f64 = (eq72_e1847 * locals.var_ibd_dn4);
        let eq72_e1849_d_n5: f64 = (eq72_e1847 * locals.var_ibd_dn5);
        let eq72_e1849_d_n6: f64 = (eq72_e1847 * locals.var_ibd_dn6);
        let eq72_e1849_d_n7: f64 = (eq72_e1847 * locals.var_ibd_dn7);
        let eq72_e1849_d_n8: f64 = (eq72_e1847 * locals.var_ibd_dn8);
        let eq72_e1849_d_n9: f64 = (eq72_e1847 * locals.var_ibd_dn9);
        let eq72_e1849_d_n10: f64 = (eq72_e1847 * locals.var_ibd_dn10);
        let eq72_e1849_d_n11: f64 = (eq72_e1847 * locals.var_ibd_dn11);
        let eq72_e1849_d_n12: f64 = (eq72_e1847 * locals.var_ibd_dn12);
        let eq72_e1849_d_n13: f64 = (eq72_e1847 * locals.var_ibd_dn13);
        let eq72_e1849_d_n14: f64 = (eq72_e1847 * locals.var_ibd_dn14);
        let eq72_e1852: f64 = ((nv13 - nv5) * p.p28);
        let eq72_e1854: f64 = (eq72_e1852 * locals.var_gmin);
        let eq72_e1854_d_n5: f64 = ((-p.p28) * locals.var_gmin);
        let eq72_e1854_d_n13: f64 = (p.p28 * locals.var_gmin);
        let eq72_e1855: f64 = (eq72_e1849 + eq72_e1854);
        let eq72_e1855_d_n5: f64 = (eq72_e1849_d_n5 + eq72_e1854_d_n5);
        let eq72_e1855_d_n13: f64 = (eq72_e1849_d_n13 + eq72_e1854_d_n13);
        (eq72_e1855, eq72_e1849_d_n0, eq72_e1849_d_n2, eq72_e1849_d_n3, eq72_e1849_d_n4, eq72_e1855_d_n5, eq72_e1849_d_n6, eq72_e1849_d_n7, eq72_e1849_d_n8, eq72_e1849_d_n9, eq72_e1849_d_n10, eq72_e1849_d_n11, eq72_e1849_d_n12, eq72_e1855_d_n13, eq72_e1849_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq72_value: f64 = eq72_e1857;
        let eq72_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq72_node_derivatives: [f64; 14] = [eq72_e1857_d_n0, eq72_e1857_d_n2, eq72_e1857_d_n3, eq72_e1857_d_n4, eq72_e1857_d_n5, eq72_e1857_d_n6, eq72_e1857_d_n7, eq72_e1857_d_n8, eq72_e1857_d_n9, eq72_e1857_d_n10, eq72_e1857_d_n11, eq72_e1857_d_n12, eq72_e1857_d_n13, eq72_e1857_d_n14];
        let eq72_branch_derivative_indices: [usize; 0] = [];
        let eq72_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            Some(5),
            multiplicity * (eq72_value),
            &eq72_node_derivative_indices,
            &eq72_node_derivatives,
            &eq72_branch_derivative_indices,
            &eq72_branch_derivatives,
            multiplicity,
        );
        let (eq73_e1868, eq73_e1868_d_n0, eq73_e1868_d_n2, eq73_e1868_d_n3, eq73_e1868_d_n4, eq73_e1868_d_n5, eq73_e1868_d_n6, eq73_e1868_d_n7, eq73_e1868_d_n8, eq73_e1868_d_n9, eq73_e1868_d_n10, eq73_e1868_d_n11, eq73_e1868_d_n12, eq73_e1868_d_n13, eq73_e1868_d_n14,) = {
    if ((locals.var_guard769 != 0.0) && (locals.var_guard770 != 0.0)) {
        let eq73_e1864: f64 = (p.p29 * locals.var_qbdj);
        let eq73_e1864_d_n0: f64 = (p.p29 * locals.var_qbdj_dn0);
        let eq73_e1864_d_n2: f64 = (p.p29 * locals.var_qbdj_dn2);
        let eq73_e1864_d_n3: f64 = (p.p29 * locals.var_qbdj_dn3);
        let eq73_e1864_d_n4: f64 = (p.p29 * locals.var_qbdj_dn4);
        let eq73_e1864_d_n5: f64 = (p.p29 * locals.var_qbdj_dn5);
        let eq73_e1864_d_n6: f64 = (p.p29 * locals.var_qbdj_dn6);
        let eq73_e1864_d_n7: f64 = (p.p29 * locals.var_qbdj_dn7);
        let eq73_e1864_d_n8: f64 = (p.p29 * locals.var_qbdj_dn8);
        let eq73_e1864_d_n9: f64 = (p.p29 * locals.var_qbdj_dn9);
        let eq73_e1864_d_n10: f64 = (p.p29 * locals.var_qbdj_dn10);
        let eq73_e1864_d_n11: f64 = (p.p29 * locals.var_qbdj_dn11);
        let eq73_e1864_d_n12: f64 = (p.p29 * locals.var_qbdj_dn12);
        let eq73_e1864_d_n13: f64 = (p.p29 * locals.var_qbdj_dn13);
        let eq73_e1864_d_n14: f64 = (p.p29 * locals.var_qbdj_dn14);
        let eq73_e1865: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq73_e1864);
        let eq73_e1866: f64 = (locals.var_devsign * eq73_e1865);
        let eq73_e1866_d_n0: f64 = (locals.var_devsign * (eq73_e1864_d_n0 * ddt_scale));
        let eq73_e1866_d_n2: f64 = (locals.var_devsign * (eq73_e1864_d_n2 * ddt_scale));
        let eq73_e1866_d_n3: f64 = (locals.var_devsign * (eq73_e1864_d_n3 * ddt_scale));
        let eq73_e1866_d_n4: f64 = (locals.var_devsign * (eq73_e1864_d_n4 * ddt_scale));
        let eq73_e1866_d_n5: f64 = (locals.var_devsign * (eq73_e1864_d_n5 * ddt_scale));
        let eq73_e1866_d_n6: f64 = (locals.var_devsign * (eq73_e1864_d_n6 * ddt_scale));
        let eq73_e1866_d_n7: f64 = (locals.var_devsign * (eq73_e1864_d_n7 * ddt_scale));
        let eq73_e1866_d_n8: f64 = (locals.var_devsign * (eq73_e1864_d_n8 * ddt_scale));
        let eq73_e1866_d_n9: f64 = (locals.var_devsign * (eq73_e1864_d_n9 * ddt_scale));
        let eq73_e1866_d_n10: f64 = (locals.var_devsign * (eq73_e1864_d_n10 * ddt_scale));
        let eq73_e1866_d_n11: f64 = (locals.var_devsign * (eq73_e1864_d_n11 * ddt_scale));
        let eq73_e1866_d_n12: f64 = (locals.var_devsign * (eq73_e1864_d_n12 * ddt_scale));
        let eq73_e1866_d_n13: f64 = (locals.var_devsign * (eq73_e1864_d_n13 * ddt_scale));
        let eq73_e1866_d_n14: f64 = (locals.var_devsign * (eq73_e1864_d_n14 * ddt_scale));
        (eq73_e1866, eq73_e1866_d_n0, eq73_e1866_d_n2, eq73_e1866_d_n3, eq73_e1866_d_n4, eq73_e1866_d_n5, eq73_e1866_d_n6, eq73_e1866_d_n7, eq73_e1866_d_n8, eq73_e1866_d_n9, eq73_e1866_d_n10, eq73_e1866_d_n11, eq73_e1866_d_n12, eq73_e1866_d_n13, eq73_e1866_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq73_value: f64 = eq73_e1868;
        let eq73_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq73_node_derivatives: [f64; 14] = [eq73_e1868_d_n0, eq73_e1868_d_n2, eq73_e1868_d_n3, eq73_e1868_d_n4, eq73_e1868_d_n5, eq73_e1868_d_n6, eq73_e1868_d_n7, eq73_e1868_d_n8, eq73_e1868_d_n9, eq73_e1868_d_n10, eq73_e1868_d_n11, eq73_e1868_d_n12, eq73_e1868_d_n13, eq73_e1868_d_n14];
        let eq73_branch_derivative_indices: [usize; 0] = [];
        let eq73_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            Some(5),
            multiplicity * (eq73_value),
            &eq73_node_derivative_indices,
            &eq73_node_derivatives,
            &eq73_branch_derivative_indices,
            &eq73_branch_derivatives,
            multiplicity,
        );
        let (eq74_e1883, eq74_e1883_d_n0, eq74_e1883_d_n2, eq74_e1883_d_n3, eq74_e1883_d_n4, eq74_e1883_d_n5, eq74_e1883_d_n6, eq74_e1883_d_n7, eq74_e1883_d_n8, eq74_e1883_d_n9, eq74_e1883_d_n10, eq74_e1883_d_n11, eq74_e1883_d_n12, eq74_e1883_d_n13, eq74_e1883_d_n14,) = {
    if (locals.var_guard769 == 0.0) {
        let eq74_e1873: f64 = (locals.var_devsign * p.p28);
        let eq74_e1875: f64 = (eq74_e1873 * locals.var_ibs);
        let eq74_e1875_d_n0: f64 = (eq74_e1873 * locals.var_ibs_dn0);
        let eq74_e1875_d_n2: f64 = (eq74_e1873 * locals.var_ibs_dn2);
        let eq74_e1875_d_n3: f64 = (eq74_e1873 * locals.var_ibs_dn3);
        let eq74_e1875_d_n4: f64 = (eq74_e1873 * locals.var_ibs_dn4);
        let eq74_e1875_d_n5: f64 = (eq74_e1873 * locals.var_ibs_dn5);
        let eq74_e1875_d_n6: f64 = (eq74_e1873 * locals.var_ibs_dn6);
        let eq74_e1875_d_n7: f64 = (eq74_e1873 * locals.var_ibs_dn7);
        let eq74_e1875_d_n8: f64 = (eq74_e1873 * locals.var_ibs_dn8);
        let eq74_e1875_d_n9: f64 = (eq74_e1873 * locals.var_ibs_dn9);
        let eq74_e1875_d_n10: f64 = (eq74_e1873 * locals.var_ibs_dn10);
        let eq74_e1875_d_n11: f64 = (eq74_e1873 * locals.var_ibs_dn11);
        let eq74_e1875_d_n12: f64 = (eq74_e1873 * locals.var_ibs_dn12);
        let eq74_e1875_d_n13: f64 = (eq74_e1873 * locals.var_ibs_dn13);
        let eq74_e1875_d_n14: f64 = (eq74_e1873 * locals.var_ibs_dn14);
        let eq74_e1878: f64 = ((nv11 - nv7) * p.p28);
        let eq74_e1880: f64 = (eq74_e1878 * locals.var_gmin);
        let eq74_e1880_d_n7: f64 = ((-p.p28) * locals.var_gmin);
        let eq74_e1880_d_n11: f64 = (p.p28 * locals.var_gmin);
        let eq74_e1881: f64 = (eq74_e1875 + eq74_e1880);
        let eq74_e1881_d_n7: f64 = (eq74_e1875_d_n7 + eq74_e1880_d_n7);
        let eq74_e1881_d_n11: f64 = (eq74_e1875_d_n11 + eq74_e1880_d_n11);
        (eq74_e1881, eq74_e1875_d_n0, eq74_e1875_d_n2, eq74_e1875_d_n3, eq74_e1875_d_n4, eq74_e1875_d_n5, eq74_e1875_d_n6, eq74_e1881_d_n7, eq74_e1875_d_n8, eq74_e1875_d_n9, eq74_e1875_d_n10, eq74_e1881_d_n11, eq74_e1875_d_n12, eq74_e1875_d_n13, eq74_e1875_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq74_value: f64 = eq74_e1883;
        let eq74_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq74_node_derivatives: [f64; 14] = [eq74_e1883_d_n0, eq74_e1883_d_n2, eq74_e1883_d_n3, eq74_e1883_d_n4, eq74_e1883_d_n5, eq74_e1883_d_n6, eq74_e1883_d_n7, eq74_e1883_d_n8, eq74_e1883_d_n9, eq74_e1883_d_n10, eq74_e1883_d_n11, eq74_e1883_d_n12, eq74_e1883_d_n13, eq74_e1883_d_n14];
        let eq74_branch_derivative_indices: [usize; 0] = [];
        let eq74_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq74_value),
            &eq74_node_derivative_indices,
            &eq74_node_derivatives,
            &eq74_branch_derivative_indices,
            &eq74_branch_derivatives,
            multiplicity,
        );
        let (eq75_e1898, eq75_e1898_d_n0, eq75_e1898_d_n2, eq75_e1898_d_n3, eq75_e1898_d_n4, eq75_e1898_d_n5, eq75_e1898_d_n6, eq75_e1898_d_n7, eq75_e1898_d_n8, eq75_e1898_d_n9, eq75_e1898_d_n10, eq75_e1898_d_n11, eq75_e1898_d_n12, eq75_e1898_d_n13, eq75_e1898_d_n14,) = {
    if (locals.var_guard769 == 0.0) {
        let eq75_e1888: f64 = (locals.var_devsign * p.p28);
        let eq75_e1890: f64 = (eq75_e1888 * locals.var_ibd);
        let eq75_e1890_d_n0: f64 = (eq75_e1888 * locals.var_ibd_dn0);
        let eq75_e1890_d_n2: f64 = (eq75_e1888 * locals.var_ibd_dn2);
        let eq75_e1890_d_n3: f64 = (eq75_e1888 * locals.var_ibd_dn3);
        let eq75_e1890_d_n4: f64 = (eq75_e1888 * locals.var_ibd_dn4);
        let eq75_e1890_d_n5: f64 = (eq75_e1888 * locals.var_ibd_dn5);
        let eq75_e1890_d_n6: f64 = (eq75_e1888 * locals.var_ibd_dn6);
        let eq75_e1890_d_n7: f64 = (eq75_e1888 * locals.var_ibd_dn7);
        let eq75_e1890_d_n8: f64 = (eq75_e1888 * locals.var_ibd_dn8);
        let eq75_e1890_d_n9: f64 = (eq75_e1888 * locals.var_ibd_dn9);
        let eq75_e1890_d_n10: f64 = (eq75_e1888 * locals.var_ibd_dn10);
        let eq75_e1890_d_n11: f64 = (eq75_e1888 * locals.var_ibd_dn11);
        let eq75_e1890_d_n12: f64 = (eq75_e1888 * locals.var_ibd_dn12);
        let eq75_e1890_d_n13: f64 = (eq75_e1888 * locals.var_ibd_dn13);
        let eq75_e1890_d_n14: f64 = (eq75_e1888 * locals.var_ibd_dn14);
        let eq75_e1893: f64 = ((nv11 - nv5) * p.p28);
        let eq75_e1895: f64 = (eq75_e1893 * locals.var_gmin);
        let eq75_e1895_d_n5: f64 = ((-p.p28) * locals.var_gmin);
        let eq75_e1895_d_n11: f64 = (p.p28 * locals.var_gmin);
        let eq75_e1896: f64 = (eq75_e1890 + eq75_e1895);
        let eq75_e1896_d_n5: f64 = (eq75_e1890_d_n5 + eq75_e1895_d_n5);
        let eq75_e1896_d_n11: f64 = (eq75_e1890_d_n11 + eq75_e1895_d_n11);
        (eq75_e1896, eq75_e1890_d_n0, eq75_e1890_d_n2, eq75_e1890_d_n3, eq75_e1890_d_n4, eq75_e1896_d_n5, eq75_e1890_d_n6, eq75_e1890_d_n7, eq75_e1890_d_n8, eq75_e1890_d_n9, eq75_e1890_d_n10, eq75_e1896_d_n11, eq75_e1890_d_n12, eq75_e1890_d_n13, eq75_e1890_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq75_value: f64 = eq75_e1898;
        let eq75_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq75_node_derivatives: [f64; 14] = [eq75_e1898_d_n0, eq75_e1898_d_n2, eq75_e1898_d_n3, eq75_e1898_d_n4, eq75_e1898_d_n5, eq75_e1898_d_n6, eq75_e1898_d_n7, eq75_e1898_d_n8, eq75_e1898_d_n9, eq75_e1898_d_n10, eq75_e1898_d_n11, eq75_e1898_d_n12, eq75_e1898_d_n13, eq75_e1898_d_n14];
        let eq75_branch_derivative_indices: [usize; 0] = [];
        let eq75_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(5),
            multiplicity * (eq75_value),
            &eq75_node_derivative_indices,
            &eq75_node_derivatives,
            &eq75_branch_derivative_indices,
            &eq75_branch_derivatives,
            multiplicity,
        );
        let (eq76_e1908, eq76_e1908_d_n0, eq76_e1908_d_n2, eq76_e1908_d_n3, eq76_e1908_d_n4, eq76_e1908_d_n5, eq76_e1908_d_n6, eq76_e1908_d_n7, eq76_e1908_d_n8, eq76_e1908_d_n9, eq76_e1908_d_n10, eq76_e1908_d_n11, eq76_e1908_d_n12, eq76_e1908_d_n13, eq76_e1908_d_n14,) = {
    if (locals.var_guard769 == 0.0) {
        let eq76_e1904: f64 = (p.p29 * locals.var_qbsj);
        let eq76_e1904_d_n0: f64 = (p.p29 * locals.var_qbsj_dn0);
        let eq76_e1904_d_n2: f64 = (p.p29 * locals.var_qbsj_dn2);
        let eq76_e1904_d_n3: f64 = (p.p29 * locals.var_qbsj_dn3);
        let eq76_e1904_d_n4: f64 = (p.p29 * locals.var_qbsj_dn4);
        let eq76_e1904_d_n5: f64 = (p.p29 * locals.var_qbsj_dn5);
        let eq76_e1904_d_n6: f64 = (p.p29 * locals.var_qbsj_dn6);
        let eq76_e1904_d_n7: f64 = (p.p29 * locals.var_qbsj_dn7);
        let eq76_e1904_d_n8: f64 = (p.p29 * locals.var_qbsj_dn8);
        let eq76_e1904_d_n9: f64 = (p.p29 * locals.var_qbsj_dn9);
        let eq76_e1904_d_n10: f64 = (p.p29 * locals.var_qbsj_dn10);
        let eq76_e1904_d_n11: f64 = (p.p29 * locals.var_qbsj_dn11);
        let eq76_e1904_d_n12: f64 = (p.p29 * locals.var_qbsj_dn12);
        let eq76_e1904_d_n13: f64 = (p.p29 * locals.var_qbsj_dn13);
        let eq76_e1904_d_n14: f64 = (p.p29 * locals.var_qbsj_dn14);
        let eq76_e1905: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq76_e1904);
        let eq76_e1906: f64 = (locals.var_devsign * eq76_e1905);
        let eq76_e1906_d_n0: f64 = (locals.var_devsign * (eq76_e1904_d_n0 * ddt_scale));
        let eq76_e1906_d_n2: f64 = (locals.var_devsign * (eq76_e1904_d_n2 * ddt_scale));
        let eq76_e1906_d_n3: f64 = (locals.var_devsign * (eq76_e1904_d_n3 * ddt_scale));
        let eq76_e1906_d_n4: f64 = (locals.var_devsign * (eq76_e1904_d_n4 * ddt_scale));
        let eq76_e1906_d_n5: f64 = (locals.var_devsign * (eq76_e1904_d_n5 * ddt_scale));
        let eq76_e1906_d_n6: f64 = (locals.var_devsign * (eq76_e1904_d_n6 * ddt_scale));
        let eq76_e1906_d_n7: f64 = (locals.var_devsign * (eq76_e1904_d_n7 * ddt_scale));
        let eq76_e1906_d_n8: f64 = (locals.var_devsign * (eq76_e1904_d_n8 * ddt_scale));
        let eq76_e1906_d_n9: f64 = (locals.var_devsign * (eq76_e1904_d_n9 * ddt_scale));
        let eq76_e1906_d_n10: f64 = (locals.var_devsign * (eq76_e1904_d_n10 * ddt_scale));
        let eq76_e1906_d_n11: f64 = (locals.var_devsign * (eq76_e1904_d_n11 * ddt_scale));
        let eq76_e1906_d_n12: f64 = (locals.var_devsign * (eq76_e1904_d_n12 * ddt_scale));
        let eq76_e1906_d_n13: f64 = (locals.var_devsign * (eq76_e1904_d_n13 * ddt_scale));
        let eq76_e1906_d_n14: f64 = (locals.var_devsign * (eq76_e1904_d_n14 * ddt_scale));
        (eq76_e1906, eq76_e1906_d_n0, eq76_e1906_d_n2, eq76_e1906_d_n3, eq76_e1906_d_n4, eq76_e1906_d_n5, eq76_e1906_d_n6, eq76_e1906_d_n7, eq76_e1906_d_n8, eq76_e1906_d_n9, eq76_e1906_d_n10, eq76_e1906_d_n11, eq76_e1906_d_n12, eq76_e1906_d_n13, eq76_e1906_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq76_value: f64 = eq76_e1908;
        let eq76_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq76_node_derivatives: [f64; 14] = [eq76_e1908_d_n0, eq76_e1908_d_n2, eq76_e1908_d_n3, eq76_e1908_d_n4, eq76_e1908_d_n5, eq76_e1908_d_n6, eq76_e1908_d_n7, eq76_e1908_d_n8, eq76_e1908_d_n9, eq76_e1908_d_n10, eq76_e1908_d_n11, eq76_e1908_d_n12, eq76_e1908_d_n13, eq76_e1908_d_n14];
        let eq76_branch_derivative_indices: [usize; 0] = [];
        let eq76_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq76_value),
            &eq76_node_derivative_indices,
            &eq76_node_derivatives,
            &eq76_branch_derivative_indices,
            &eq76_branch_derivatives,
            multiplicity,
        );
        let (eq77_e1918, eq77_e1918_d_n0, eq77_e1918_d_n2, eq77_e1918_d_n3, eq77_e1918_d_n4, eq77_e1918_d_n5, eq77_e1918_d_n6, eq77_e1918_d_n7, eq77_e1918_d_n8, eq77_e1918_d_n9, eq77_e1918_d_n10, eq77_e1918_d_n11, eq77_e1918_d_n12, eq77_e1918_d_n13, eq77_e1918_d_n14,) = {
    if (locals.var_guard769 == 0.0) {
        let eq77_e1914: f64 = (p.p29 * locals.var_qbdj);
        let eq77_e1914_d_n0: f64 = (p.p29 * locals.var_qbdj_dn0);
        let eq77_e1914_d_n2: f64 = (p.p29 * locals.var_qbdj_dn2);
        let eq77_e1914_d_n3: f64 = (p.p29 * locals.var_qbdj_dn3);
        let eq77_e1914_d_n4: f64 = (p.p29 * locals.var_qbdj_dn4);
        let eq77_e1914_d_n5: f64 = (p.p29 * locals.var_qbdj_dn5);
        let eq77_e1914_d_n6: f64 = (p.p29 * locals.var_qbdj_dn6);
        let eq77_e1914_d_n7: f64 = (p.p29 * locals.var_qbdj_dn7);
        let eq77_e1914_d_n8: f64 = (p.p29 * locals.var_qbdj_dn8);
        let eq77_e1914_d_n9: f64 = (p.p29 * locals.var_qbdj_dn9);
        let eq77_e1914_d_n10: f64 = (p.p29 * locals.var_qbdj_dn10);
        let eq77_e1914_d_n11: f64 = (p.p29 * locals.var_qbdj_dn11);
        let eq77_e1914_d_n12: f64 = (p.p29 * locals.var_qbdj_dn12);
        let eq77_e1914_d_n13: f64 = (p.p29 * locals.var_qbdj_dn13);
        let eq77_e1914_d_n14: f64 = (p.p29 * locals.var_qbdj_dn14);
        let eq77_e1915: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq77_e1914);
        let eq77_e1916: f64 = (locals.var_devsign * eq77_e1915);
        let eq77_e1916_d_n0: f64 = (locals.var_devsign * (eq77_e1914_d_n0 * ddt_scale));
        let eq77_e1916_d_n2: f64 = (locals.var_devsign * (eq77_e1914_d_n2 * ddt_scale));
        let eq77_e1916_d_n3: f64 = (locals.var_devsign * (eq77_e1914_d_n3 * ddt_scale));
        let eq77_e1916_d_n4: f64 = (locals.var_devsign * (eq77_e1914_d_n4 * ddt_scale));
        let eq77_e1916_d_n5: f64 = (locals.var_devsign * (eq77_e1914_d_n5 * ddt_scale));
        let eq77_e1916_d_n6: f64 = (locals.var_devsign * (eq77_e1914_d_n6 * ddt_scale));
        let eq77_e1916_d_n7: f64 = (locals.var_devsign * (eq77_e1914_d_n7 * ddt_scale));
        let eq77_e1916_d_n8: f64 = (locals.var_devsign * (eq77_e1914_d_n8 * ddt_scale));
        let eq77_e1916_d_n9: f64 = (locals.var_devsign * (eq77_e1914_d_n9 * ddt_scale));
        let eq77_e1916_d_n10: f64 = (locals.var_devsign * (eq77_e1914_d_n10 * ddt_scale));
        let eq77_e1916_d_n11: f64 = (locals.var_devsign * (eq77_e1914_d_n11 * ddt_scale));
        let eq77_e1916_d_n12: f64 = (locals.var_devsign * (eq77_e1914_d_n12 * ddt_scale));
        let eq77_e1916_d_n13: f64 = (locals.var_devsign * (eq77_e1914_d_n13 * ddt_scale));
        let eq77_e1916_d_n14: f64 = (locals.var_devsign * (eq77_e1914_d_n14 * ddt_scale));
        (eq77_e1916, eq77_e1916_d_n0, eq77_e1916_d_n2, eq77_e1916_d_n3, eq77_e1916_d_n4, eq77_e1916_d_n5, eq77_e1916_d_n6, eq77_e1916_d_n7, eq77_e1916_d_n8, eq77_e1916_d_n9, eq77_e1916_d_n10, eq77_e1916_d_n11, eq77_e1916_d_n12, eq77_e1916_d_n13, eq77_e1916_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq77_value: f64 = eq77_e1918;
        let eq77_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq77_node_derivatives: [f64; 14] = [eq77_e1918_d_n0, eq77_e1918_d_n2, eq77_e1918_d_n3, eq77_e1918_d_n4, eq77_e1918_d_n5, eq77_e1918_d_n6, eq77_e1918_d_n7, eq77_e1918_d_n8, eq77_e1918_d_n9, eq77_e1918_d_n10, eq77_e1918_d_n11, eq77_e1918_d_n12, eq77_e1918_d_n13, eq77_e1918_d_n14];
        let eq77_branch_derivative_indices: [usize; 0] = [];
        let eq77_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(5),
            multiplicity * (eq77_value),
            &eq77_node_derivative_indices,
            &eq77_node_derivatives,
            &eq77_branch_derivative_indices,
            &eq77_branch_derivatives,
            multiplicity,
        );
        let (eq81_e1959, eq81_e1959_d_n0, eq81_e1959_d_n2, eq81_e1959_d_n3, eq81_e1959_d_n4, eq81_e1959_d_n5, eq81_e1959_d_n6, eq81_e1959_d_n7, eq81_e1959_d_n8, eq81_e1959_d_n9, eq81_e1959_d_n10, eq81_e1959_d_n11, eq81_e1959_d_n12, eq81_e1959_d_n13, eq81_e1959_d_n14,) = {
    if (locals.var_guard772 != 0.0) {
        let eq81_e1945: f64 = (locals.var_devsign * p.p28);
        let eq81_e1947: f64 = (eq81_e1945 * locals.var_ibd);
        let eq81_e1947_d_n0: f64 = (eq81_e1945 * locals.var_ibd_dn0);
        let eq81_e1947_d_n2: f64 = (eq81_e1945 * locals.var_ibd_dn2);
        let eq81_e1947_d_n3: f64 = (eq81_e1945 * locals.var_ibd_dn3);
        let eq81_e1947_d_n4: f64 = (eq81_e1945 * locals.var_ibd_dn4);
        let eq81_e1947_d_n5: f64 = (eq81_e1945 * locals.var_ibd_dn5);
        let eq81_e1947_d_n6: f64 = (eq81_e1945 * locals.var_ibd_dn6);
        let eq81_e1947_d_n7: f64 = (eq81_e1945 * locals.var_ibd_dn7);
        let eq81_e1947_d_n8: f64 = (eq81_e1945 * locals.var_ibd_dn8);
        let eq81_e1947_d_n9: f64 = (eq81_e1945 * locals.var_ibd_dn9);
        let eq81_e1947_d_n10: f64 = (eq81_e1945 * locals.var_ibd_dn10);
        let eq81_e1947_d_n11: f64 = (eq81_e1945 * locals.var_ibd_dn11);
        let eq81_e1947_d_n12: f64 = (eq81_e1945 * locals.var_ibd_dn12);
        let eq81_e1947_d_n13: f64 = (eq81_e1945 * locals.var_ibd_dn13);
        let eq81_e1947_d_n14: f64 = (eq81_e1945 * locals.var_ibd_dn14);
        let eq81_e1950: f64 = (1.0 - p.p1128);
        let eq81_e1952: f64 = (eq81_e1950 * p.p28);
        let eq81_e1954: f64 = (eq81_e1952 * (nv13 - nv5));
        let eq81_e1956: f64 = (eq81_e1954 * locals.var_gmin);
        let eq81_e1956_d_n5: f64 = ((-eq81_e1952) * locals.var_gmin);
        let eq81_e1956_d_n13: f64 = (eq81_e1952 * locals.var_gmin);
        let eq81_e1957: f64 = (eq81_e1947 + eq81_e1956);
        let eq81_e1957_d_n5: f64 = (eq81_e1947_d_n5 + eq81_e1956_d_n5);
        let eq81_e1957_d_n13: f64 = (eq81_e1947_d_n13 + eq81_e1956_d_n13);
        (eq81_e1957, eq81_e1947_d_n0, eq81_e1947_d_n2, eq81_e1947_d_n3, eq81_e1947_d_n4, eq81_e1957_d_n5, eq81_e1947_d_n6, eq81_e1947_d_n7, eq81_e1947_d_n8, eq81_e1947_d_n9, eq81_e1947_d_n10, eq81_e1947_d_n11, eq81_e1947_d_n12, eq81_e1957_d_n13, eq81_e1947_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq81_value: f64 = eq81_e1959;
        let eq81_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq81_node_derivatives: [f64; 14] = [eq81_e1959_d_n0, eq81_e1959_d_n2, eq81_e1959_d_n3, eq81_e1959_d_n4, eq81_e1959_d_n5, eq81_e1959_d_n6, eq81_e1959_d_n7, eq81_e1959_d_n8, eq81_e1959_d_n9, eq81_e1959_d_n10, eq81_e1959_d_n11, eq81_e1959_d_n12, eq81_e1959_d_n13, eq81_e1959_d_n14];
        let eq81_branch_derivative_indices: [usize; 0] = [];
        let eq81_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            Some(5),
            multiplicity * (eq81_value),
            &eq81_node_derivative_indices,
            &eq81_node_derivatives,
            &eq81_branch_derivative_indices,
            &eq81_branch_derivatives,
            multiplicity,
        );
        let (eq82_e1975, eq82_e1975_d_n0, eq82_e1975_d_n2, eq82_e1975_d_n3, eq82_e1975_d_n4, eq82_e1975_d_n5, eq82_e1975_d_n6, eq82_e1975_d_n7, eq82_e1975_d_n8, eq82_e1975_d_n9, eq82_e1975_d_n10, eq82_e1975_d_n11, eq82_e1975_d_n12, eq82_e1975_d_n13, eq82_e1975_d_n14,) = {
    if (locals.var_guard772 != 0.0) {
        let eq82_e1963: f64 = (locals.var_devsign * p.p28);
        let eq82_e1965: f64 = (eq82_e1963 * locals.var_ibd_ext);
        let eq82_e1965_d_n0: f64 = (eq82_e1963 * locals.var_ibd_ext_dn0);
        let eq82_e1965_d_n2: f64 = (eq82_e1963 * locals.var_ibd_ext_dn2);
        let eq82_e1965_d_n3: f64 = (eq82_e1963 * locals.var_ibd_ext_dn3);
        let eq82_e1965_d_n4: f64 = (eq82_e1963 * locals.var_ibd_ext_dn4);
        let eq82_e1965_d_n5: f64 = (eq82_e1963 * locals.var_ibd_ext_dn5);
        let eq82_e1965_d_n6: f64 = (eq82_e1963 * locals.var_ibd_ext_dn6);
        let eq82_e1965_d_n7: f64 = (eq82_e1963 * locals.var_ibd_ext_dn7);
        let eq82_e1965_d_n8: f64 = (eq82_e1963 * locals.var_ibd_ext_dn8);
        let eq82_e1965_d_n9: f64 = (eq82_e1963 * locals.var_ibd_ext_dn9);
        let eq82_e1965_d_n10: f64 = (eq82_e1963 * locals.var_ibd_ext_dn10);
        let eq82_e1965_d_n11: f64 = (eq82_e1963 * locals.var_ibd_ext_dn11);
        let eq82_e1965_d_n12: f64 = (eq82_e1963 * locals.var_ibd_ext_dn12);
        let eq82_e1965_d_n13: f64 = (eq82_e1963 * locals.var_ibd_ext_dn13);
        let eq82_e1965_d_n14: f64 = (eq82_e1963 * locals.var_ibd_ext_dn14);
        let eq82_e1968: f64 = (p.p1128 * p.p28);
        let eq82_e1970: f64 = (eq82_e1968 * (nv13 - nv14));
        let eq82_e1972: f64 = (eq82_e1970 * locals.var_gmin);
        let eq82_e1972_d_n13: f64 = (eq82_e1968 * locals.var_gmin);
        let eq82_e1972_d_n14: f64 = ((-eq82_e1968) * locals.var_gmin);
        let eq82_e1973: f64 = (eq82_e1965 + eq82_e1972);
        let eq82_e1973_d_n13: f64 = (eq82_e1965_d_n13 + eq82_e1972_d_n13);
        let eq82_e1973_d_n14: f64 = (eq82_e1965_d_n14 + eq82_e1972_d_n14);
        (eq82_e1973, eq82_e1965_d_n0, eq82_e1965_d_n2, eq82_e1965_d_n3, eq82_e1965_d_n4, eq82_e1965_d_n5, eq82_e1965_d_n6, eq82_e1965_d_n7, eq82_e1965_d_n8, eq82_e1965_d_n9, eq82_e1965_d_n10, eq82_e1965_d_n11, eq82_e1965_d_n12, eq82_e1973_d_n13, eq82_e1973_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq82_value: f64 = eq82_e1975;
        let eq82_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq82_node_derivatives: [f64; 14] = [eq82_e1975_d_n0, eq82_e1975_d_n2, eq82_e1975_d_n3, eq82_e1975_d_n4, eq82_e1975_d_n5, eq82_e1975_d_n6, eq82_e1975_d_n7, eq82_e1975_d_n8, eq82_e1975_d_n9, eq82_e1975_d_n10, eq82_e1975_d_n11, eq82_e1975_d_n12, eq82_e1975_d_n13, eq82_e1975_d_n14];
        let eq82_branch_derivative_indices: [usize; 0] = [];
        let eq82_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            Some(14),
            multiplicity * (eq82_value),
            &eq82_node_derivative_indices,
            &eq82_node_derivatives,
            &eq82_branch_derivative_indices,
            &eq82_branch_derivatives,
            multiplicity,
        );
        let (eq83_e1984, eq83_e1984_d_n0, eq83_e1984_d_n2, eq83_e1984_d_n3, eq83_e1984_d_n4, eq83_e1984_d_n5, eq83_e1984_d_n6, eq83_e1984_d_n7, eq83_e1984_d_n8, eq83_e1984_d_n9, eq83_e1984_d_n10, eq83_e1984_d_n11, eq83_e1984_d_n12, eq83_e1984_d_n13, eq83_e1984_d_n14,) = {
    if (locals.var_guard772 != 0.0) {
        let eq83_e1980: f64 = (p.p29 * locals.var_qbdj);
        let eq83_e1980_d_n0: f64 = (p.p29 * locals.var_qbdj_dn0);
        let eq83_e1980_d_n2: f64 = (p.p29 * locals.var_qbdj_dn2);
        let eq83_e1980_d_n3: f64 = (p.p29 * locals.var_qbdj_dn3);
        let eq83_e1980_d_n4: f64 = (p.p29 * locals.var_qbdj_dn4);
        let eq83_e1980_d_n5: f64 = (p.p29 * locals.var_qbdj_dn5);
        let eq83_e1980_d_n6: f64 = (p.p29 * locals.var_qbdj_dn6);
        let eq83_e1980_d_n7: f64 = (p.p29 * locals.var_qbdj_dn7);
        let eq83_e1980_d_n8: f64 = (p.p29 * locals.var_qbdj_dn8);
        let eq83_e1980_d_n9: f64 = (p.p29 * locals.var_qbdj_dn9);
        let eq83_e1980_d_n10: f64 = (p.p29 * locals.var_qbdj_dn10);
        let eq83_e1980_d_n11: f64 = (p.p29 * locals.var_qbdj_dn11);
        let eq83_e1980_d_n12: f64 = (p.p29 * locals.var_qbdj_dn12);
        let eq83_e1980_d_n13: f64 = (p.p29 * locals.var_qbdj_dn13);
        let eq83_e1980_d_n14: f64 = (p.p29 * locals.var_qbdj_dn14);
        let eq83_e1981: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq83_e1980);
        let eq83_e1982: f64 = (locals.var_devsign * eq83_e1981);
        let eq83_e1982_d_n0: f64 = (locals.var_devsign * (eq83_e1980_d_n0 * ddt_scale));
        let eq83_e1982_d_n2: f64 = (locals.var_devsign * (eq83_e1980_d_n2 * ddt_scale));
        let eq83_e1982_d_n3: f64 = (locals.var_devsign * (eq83_e1980_d_n3 * ddt_scale));
        let eq83_e1982_d_n4: f64 = (locals.var_devsign * (eq83_e1980_d_n4 * ddt_scale));
        let eq83_e1982_d_n5: f64 = (locals.var_devsign * (eq83_e1980_d_n5 * ddt_scale));
        let eq83_e1982_d_n6: f64 = (locals.var_devsign * (eq83_e1980_d_n6 * ddt_scale));
        let eq83_e1982_d_n7: f64 = (locals.var_devsign * (eq83_e1980_d_n7 * ddt_scale));
        let eq83_e1982_d_n8: f64 = (locals.var_devsign * (eq83_e1980_d_n8 * ddt_scale));
        let eq83_e1982_d_n9: f64 = (locals.var_devsign * (eq83_e1980_d_n9 * ddt_scale));
        let eq83_e1982_d_n10: f64 = (locals.var_devsign * (eq83_e1980_d_n10 * ddt_scale));
        let eq83_e1982_d_n11: f64 = (locals.var_devsign * (eq83_e1980_d_n11 * ddt_scale));
        let eq83_e1982_d_n12: f64 = (locals.var_devsign * (eq83_e1980_d_n12 * ddt_scale));
        let eq83_e1982_d_n13: f64 = (locals.var_devsign * (eq83_e1980_d_n13 * ddt_scale));
        let eq83_e1982_d_n14: f64 = (locals.var_devsign * (eq83_e1980_d_n14 * ddt_scale));
        (eq83_e1982, eq83_e1982_d_n0, eq83_e1982_d_n2, eq83_e1982_d_n3, eq83_e1982_d_n4, eq83_e1982_d_n5, eq83_e1982_d_n6, eq83_e1982_d_n7, eq83_e1982_d_n8, eq83_e1982_d_n9, eq83_e1982_d_n10, eq83_e1982_d_n11, eq83_e1982_d_n12, eq83_e1982_d_n13, eq83_e1982_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq83_value: f64 = eq83_e1984;
        let eq83_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq83_node_derivatives: [f64; 14] = [eq83_e1984_d_n0, eq83_e1984_d_n2, eq83_e1984_d_n3, eq83_e1984_d_n4, eq83_e1984_d_n5, eq83_e1984_d_n6, eq83_e1984_d_n7, eq83_e1984_d_n8, eq83_e1984_d_n9, eq83_e1984_d_n10, eq83_e1984_d_n11, eq83_e1984_d_n12, eq83_e1984_d_n13, eq83_e1984_d_n14];
        let eq83_branch_derivative_indices: [usize; 0] = [];
        let eq83_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            Some(5),
            multiplicity * (eq83_value),
            &eq83_node_derivative_indices,
            &eq83_node_derivatives,
            &eq83_branch_derivative_indices,
            &eq83_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
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
        let (eq84_e1993, eq84_e1993_d_n0, eq84_e1993_d_n2, eq84_e1993_d_n3, eq84_e1993_d_n4, eq84_e1993_d_n5, eq84_e1993_d_n6, eq84_e1993_d_n7, eq84_e1993_d_n8, eq84_e1993_d_n9, eq84_e1993_d_n10, eq84_e1993_d_n11, eq84_e1993_d_n12, eq84_e1993_d_n13, eq84_e1993_d_n14,) = {
    if (locals.var_guard772 != 0.0) {
        let eq84_e1989: f64 = (p.p29 * locals.var_qbdj_ext);
        let eq84_e1989_d_n0: f64 = (p.p29 * locals.var_qbdj_ext_dn0);
        let eq84_e1989_d_n2: f64 = (p.p29 * locals.var_qbdj_ext_dn2);
        let eq84_e1989_d_n3: f64 = (p.p29 * locals.var_qbdj_ext_dn3);
        let eq84_e1989_d_n4: f64 = (p.p29 * locals.var_qbdj_ext_dn4);
        let eq84_e1989_d_n5: f64 = (p.p29 * locals.var_qbdj_ext_dn5);
        let eq84_e1989_d_n6: f64 = (p.p29 * locals.var_qbdj_ext_dn6);
        let eq84_e1989_d_n7: f64 = (p.p29 * locals.var_qbdj_ext_dn7);
        let eq84_e1989_d_n8: f64 = (p.p29 * locals.var_qbdj_ext_dn8);
        let eq84_e1989_d_n9: f64 = (p.p29 * locals.var_qbdj_ext_dn9);
        let eq84_e1989_d_n10: f64 = (p.p29 * locals.var_qbdj_ext_dn10);
        let eq84_e1989_d_n11: f64 = (p.p29 * locals.var_qbdj_ext_dn11);
        let eq84_e1989_d_n12: f64 = (p.p29 * locals.var_qbdj_ext_dn12);
        let eq84_e1989_d_n13: f64 = (p.p29 * locals.var_qbdj_ext_dn13);
        let eq84_e1989_d_n14: f64 = (p.p29 * locals.var_qbdj_ext_dn14);
        let eq84_e1990: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 15, eq84_e1989);
        let eq84_e1991: f64 = (locals.var_devsign * eq84_e1990);
        let eq84_e1991_d_n0: f64 = (locals.var_devsign * (eq84_e1989_d_n0 * ddt_scale));
        let eq84_e1991_d_n2: f64 = (locals.var_devsign * (eq84_e1989_d_n2 * ddt_scale));
        let eq84_e1991_d_n3: f64 = (locals.var_devsign * (eq84_e1989_d_n3 * ddt_scale));
        let eq84_e1991_d_n4: f64 = (locals.var_devsign * (eq84_e1989_d_n4 * ddt_scale));
        let eq84_e1991_d_n5: f64 = (locals.var_devsign * (eq84_e1989_d_n5 * ddt_scale));
        let eq84_e1991_d_n6: f64 = (locals.var_devsign * (eq84_e1989_d_n6 * ddt_scale));
        let eq84_e1991_d_n7: f64 = (locals.var_devsign * (eq84_e1989_d_n7 * ddt_scale));
        let eq84_e1991_d_n8: f64 = (locals.var_devsign * (eq84_e1989_d_n8 * ddt_scale));
        let eq84_e1991_d_n9: f64 = (locals.var_devsign * (eq84_e1989_d_n9 * ddt_scale));
        let eq84_e1991_d_n10: f64 = (locals.var_devsign * (eq84_e1989_d_n10 * ddt_scale));
        let eq84_e1991_d_n11: f64 = (locals.var_devsign * (eq84_e1989_d_n11 * ddt_scale));
        let eq84_e1991_d_n12: f64 = (locals.var_devsign * (eq84_e1989_d_n12 * ddt_scale));
        let eq84_e1991_d_n13: f64 = (locals.var_devsign * (eq84_e1989_d_n13 * ddt_scale));
        let eq84_e1991_d_n14: f64 = (locals.var_devsign * (eq84_e1989_d_n14 * ddt_scale));
        (eq84_e1991, eq84_e1991_d_n0, eq84_e1991_d_n2, eq84_e1991_d_n3, eq84_e1991_d_n4, eq84_e1991_d_n5, eq84_e1991_d_n6, eq84_e1991_d_n7, eq84_e1991_d_n8, eq84_e1991_d_n9, eq84_e1991_d_n10, eq84_e1991_d_n11, eq84_e1991_d_n12, eq84_e1991_d_n13, eq84_e1991_d_n14,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq84_value: f64 = eq84_e1993;
        let eq84_node_derivative_indices: [usize; 14] = [0, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let eq84_node_derivatives: [f64; 14] = [eq84_e1993_d_n0, eq84_e1993_d_n2, eq84_e1993_d_n3, eq84_e1993_d_n4, eq84_e1993_d_n5, eq84_e1993_d_n6, eq84_e1993_d_n7, eq84_e1993_d_n8, eq84_e1993_d_n9, eq84_e1993_d_n10, eq84_e1993_d_n11, eq84_e1993_d_n12, eq84_e1993_d_n13, eq84_e1993_d_n14];
        let eq84_branch_derivative_indices: [usize; 0] = [];
        let eq84_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(13),
            Some(14),
            multiplicity * (eq84_value),
            &eq84_node_derivative_indices,
            &eq84_node_derivatives,
            &eq84_branch_derivative_indices,
            &eq84_branch_derivatives,
            multiplicity,
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
        let nv15 = ctx.node_voltage(nodes[15]);
        let (eq8_e1290, eq8_e1290_d_n0, eq8_e1290_d_n2, eq8_e1290_d_n3, eq8_e1290_d_n4, eq8_e1290_d_n5, eq8_e1290_d_n6, eq8_e1290_d_n7, eq8_e1290_d_n8, eq8_e1290_d_n9, eq8_e1290_d_n10, eq8_e1290_d_n11, eq8_e1290_d_n12, eq8_e1290_d_n13, eq8_e1290_d_n14, eq8_e1290_d_n15, eq8_e1290_q,) = {
    if ((locals.var_guard698 != 0.0) && (locals.var_guard697 == 0.0)) {
        let eq8_e1279: f64 = (locals.var_mig * locals.var_cox);
        let eq8_e1279_d_n0: f64 = (locals.var_mig_dn0 * locals.var_cox);
        let eq8_e1279_d_n2: f64 = (locals.var_mig_dn2 * locals.var_cox);
        let eq8_e1279_d_n3: f64 = (locals.var_mig_dn3 * locals.var_cox);
        let eq8_e1279_d_n4: f64 = (locals.var_mig_dn4 * locals.var_cox);
        let eq8_e1279_d_n5: f64 = (locals.var_mig_dn5 * locals.var_cox);
        let eq8_e1279_d_n6: f64 = (locals.var_mig_dn6 * locals.var_cox);
        let eq8_e1279_d_n7: f64 = (locals.var_mig_dn7 * locals.var_cox);
        let eq8_e1279_d_n8: f64 = (locals.var_mig_dn8 * locals.var_cox);
        let eq8_e1279_d_n9: f64 = (locals.var_mig_dn9 * locals.var_cox);
        let eq8_e1279_d_n10: f64 = (locals.var_mig_dn10 * locals.var_cox);
        let eq8_e1279_d_n11: f64 = (locals.var_mig_dn11 * locals.var_cox);
        let eq8_e1279_d_n12: f64 = (locals.var_mig_dn12 * locals.var_cox);
        let eq8_e1279_d_n13: f64 = (locals.var_mig_dn13 * locals.var_cox);
        let eq8_e1279_d_n14: f64 = (locals.var_mig_dn14 * locals.var_cox);
        let eq8_e1281: f64 = (eq8_e1279 * locals.var_weff);
        let eq8_e1281_d_n0: f64 = (eq8_e1279_d_n0 * locals.var_weff);
        let eq8_e1281_d_n2: f64 = (eq8_e1279_d_n2 * locals.var_weff);
        let eq8_e1281_d_n3: f64 = (eq8_e1279_d_n3 * locals.var_weff);
        let eq8_e1281_d_n4: f64 = (eq8_e1279_d_n4 * locals.var_weff);
        let eq8_e1281_d_n5: f64 = (eq8_e1279_d_n5 * locals.var_weff);
        let eq8_e1281_d_n6: f64 = (eq8_e1279_d_n6 * locals.var_weff);
        let eq8_e1281_d_n7: f64 = (eq8_e1279_d_n7 * locals.var_weff);
        let eq8_e1281_d_n8: f64 = (eq8_e1279_d_n8 * locals.var_weff);
        let eq8_e1281_d_n9: f64 = (eq8_e1279_d_n9 * locals.var_weff);
        let eq8_e1281_d_n10: f64 = (eq8_e1279_d_n10 * locals.var_weff);
        let eq8_e1281_d_n11: f64 = (eq8_e1279_d_n11 * locals.var_weff);
        let eq8_e1281_d_n12: f64 = (eq8_e1279_d_n12 * locals.var_weff);
        let eq8_e1281_d_n13: f64 = (eq8_e1279_d_n13 * locals.var_weff);
        let eq8_e1281_d_n14: f64 = (eq8_e1279_d_n14 * locals.var_weff);
        let eq8_e1283: f64 = (eq8_e1281 * p.p2);
        let eq8_e1283_d_n0: f64 = (eq8_e1281_d_n0 * p.p2);
        let eq8_e1283_d_n2: f64 = (eq8_e1281_d_n2 * p.p2);
        let eq8_e1283_d_n3: f64 = (eq8_e1281_d_n3 * p.p2);
        let eq8_e1283_d_n4: f64 = (eq8_e1281_d_n4 * p.p2);
        let eq8_e1283_d_n5: f64 = (eq8_e1281_d_n5 * p.p2);
        let eq8_e1283_d_n6: f64 = (eq8_e1281_d_n6 * p.p2);
        let eq8_e1283_d_n7: f64 = (eq8_e1281_d_n7 * p.p2);
        let eq8_e1283_d_n8: f64 = (eq8_e1281_d_n8 * p.p2);
        let eq8_e1283_d_n9: f64 = (eq8_e1281_d_n9 * p.p2);
        let eq8_e1283_d_n10: f64 = (eq8_e1281_d_n10 * p.p2);
        let eq8_e1283_d_n11: f64 = (eq8_e1281_d_n11 * p.p2);
        let eq8_e1283_d_n12: f64 = (eq8_e1281_d_n12 * p.p2);
        let eq8_e1283_d_n13: f64 = (eq8_e1281_d_n13 * p.p2);
        let eq8_e1283_d_n14: f64 = (eq8_e1281_d_n14 * p.p2);
        let eq8_e1285: f64 = (eq8_e1283 * locals.var_leff);
        let eq8_e1285_d_n0: f64 = (eq8_e1283_d_n0 * locals.var_leff);
        let eq8_e1285_d_n2: f64 = (eq8_e1283_d_n2 * locals.var_leff);
        let eq8_e1285_d_n3: f64 = (eq8_e1283_d_n3 * locals.var_leff);
        let eq8_e1285_d_n4: f64 = (eq8_e1283_d_n4 * locals.var_leff);
        let eq8_e1285_d_n5: f64 = (eq8_e1283_d_n5 * locals.var_leff);
        let eq8_e1285_d_n6: f64 = (eq8_e1283_d_n6 * locals.var_leff);
        let eq8_e1285_d_n7: f64 = (eq8_e1283_d_n7 * locals.var_leff);
        let eq8_e1285_d_n8: f64 = (eq8_e1283_d_n8 * locals.var_leff);
        let eq8_e1285_d_n9: f64 = (eq8_e1283_d_n9 * locals.var_leff);
        let eq8_e1285_d_n10: f64 = (eq8_e1283_d_n10 * locals.var_leff);
        let eq8_e1285_d_n11: f64 = (eq8_e1283_d_n11 * locals.var_leff);
        let eq8_e1285_d_n12: f64 = (eq8_e1283_d_n12 * locals.var_leff);
        let eq8_e1285_d_n13: f64 = (eq8_e1283_d_n13 * locals.var_leff);
        let eq8_e1285_d_n14: f64 = (eq8_e1283_d_n14 * locals.var_leff);
        let eq8_e1287: f64 = (eq8_e1285 * (nv15 - 0.0));
        let eq8_e1287_d_n0: f64 = (eq8_e1285_d_n0 * (nv15 - 0.0));
        let eq8_e1287_d_n2: f64 = (eq8_e1285_d_n2 * (nv15 - 0.0));
        let eq8_e1287_d_n3: f64 = (eq8_e1285_d_n3 * (nv15 - 0.0));
        let eq8_e1287_d_n4: f64 = (eq8_e1285_d_n4 * (nv15 - 0.0));
        let eq8_e1287_d_n5: f64 = (eq8_e1285_d_n5 * (nv15 - 0.0));
        let eq8_e1287_d_n6: f64 = (eq8_e1285_d_n6 * (nv15 - 0.0));
        let eq8_e1287_d_n7: f64 = (eq8_e1285_d_n7 * (nv15 - 0.0));
        let eq8_e1287_d_n8: f64 = (eq8_e1285_d_n8 * (nv15 - 0.0));
        let eq8_e1287_d_n9: f64 = (eq8_e1285_d_n9 * (nv15 - 0.0));
        let eq8_e1287_d_n10: f64 = (eq8_e1285_d_n10 * (nv15 - 0.0));
        let eq8_e1287_d_n11: f64 = (eq8_e1285_d_n11 * (nv15 - 0.0));
        let eq8_e1287_d_n12: f64 = (eq8_e1285_d_n12 * (nv15 - 0.0));
        let eq8_e1287_d_n13: f64 = (eq8_e1285_d_n13 * (nv15 - 0.0));
        let eq8_e1287_d_n14: f64 = (eq8_e1285_d_n14 * (nv15 - 0.0));
        let eq8_e1288_q: f64 = eq8_e1287;
        (eq8_e1287, eq8_e1287_d_n0, eq8_e1287_d_n2, eq8_e1287_d_n3, eq8_e1287_d_n4, eq8_e1287_d_n5, eq8_e1287_d_n6, eq8_e1287_d_n7, eq8_e1287_d_n8, eq8_e1287_d_n9, eq8_e1287_d_n10, eq8_e1287_d_n11, eq8_e1287_d_n12, eq8_e1287_d_n13, eq8_e1287_d_n14, eq8_e1285, eq8_e1288_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq8_reactive_node_derivatives: [f64; 17] = [eq8_e1290_d_n0, 0.0, eq8_e1290_d_n2, eq8_e1290_d_n3, eq8_e1290_d_n4, eq8_e1290_d_n5, eq8_e1290_d_n6, eq8_e1290_d_n7, eq8_e1290_d_n8, eq8_e1290_d_n9, eq8_e1290_d_n10, eq8_e1290_d_n11, eq8_e1290_d_n12, eq8_e1290_d_n13, eq8_e1290_d_n14, eq8_e1290_d_n15, 0.0];
        let eq8_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[15]),
            None,
            nodes,
            &eq8_reactive_node_derivatives,
            branches,
            &eq8_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq11_e1344, eq11_e1344_d_n0, eq11_e1344_d_n2, eq11_e1344_d_n3, eq11_e1344_d_n4, eq11_e1344_d_n5, eq11_e1344_d_n6, eq11_e1344_d_n7, eq11_e1344_d_n8, eq11_e1344_d_n9, eq11_e1344_d_n10, eq11_e1344_d_n11, eq11_e1344_d_n12, eq11_e1344_d_n13, eq11_e1344_d_n14, eq11_e1344_d_n15, eq11_e1344_q,) = {
    if ((locals.var_guard698 != 0.0) && (locals.var_guard697 == 0.0)) {
        let eq11_e1327: f64 = (1.0 + locals.var_sigvds);
        let eq11_e1329: f64 = (eq11_e1327 * locals.var_mig);
        let eq11_e1329_d_n0: f64 = (eq11_e1327 * locals.var_mig_dn0);
        let eq11_e1329_d_n2: f64 = (eq11_e1327 * locals.var_mig_dn2);
        let eq11_e1329_d_n3: f64 = (eq11_e1327 * locals.var_mig_dn3);
        let eq11_e1329_d_n4: f64 = (eq11_e1327 * locals.var_mig_dn4);
        let eq11_e1329_d_n5: f64 = (eq11_e1327 * locals.var_mig_dn5);
        let eq11_e1329_d_n6: f64 = (eq11_e1327 * locals.var_mig_dn6);
        let eq11_e1329_d_n7: f64 = (eq11_e1327 * locals.var_mig_dn7);
        let eq11_e1329_d_n8: f64 = (eq11_e1327 * locals.var_mig_dn8);
        let eq11_e1329_d_n9: f64 = (eq11_e1327 * locals.var_mig_dn9);
        let eq11_e1329_d_n10: f64 = (eq11_e1327 * locals.var_mig_dn10);
        let eq11_e1329_d_n11: f64 = (eq11_e1327 * locals.var_mig_dn11);
        let eq11_e1329_d_n12: f64 = (eq11_e1327 * locals.var_mig_dn12);
        let eq11_e1329_d_n13: f64 = (eq11_e1327 * locals.var_mig_dn13);
        let eq11_e1329_d_n14: f64 = (eq11_e1327 * locals.var_mig_dn14);
        let eq11_e1331: f64 = (eq11_e1329 * locals.var_cox);
        let eq11_e1331_d_n0: f64 = (eq11_e1329_d_n0 * locals.var_cox);
        let eq11_e1331_d_n2: f64 = (eq11_e1329_d_n2 * locals.var_cox);
        let eq11_e1331_d_n3: f64 = (eq11_e1329_d_n3 * locals.var_cox);
        let eq11_e1331_d_n4: f64 = (eq11_e1329_d_n4 * locals.var_cox);
        let eq11_e1331_d_n5: f64 = (eq11_e1329_d_n5 * locals.var_cox);
        let eq11_e1331_d_n6: f64 = (eq11_e1329_d_n6 * locals.var_cox);
        let eq11_e1331_d_n7: f64 = (eq11_e1329_d_n7 * locals.var_cox);
        let eq11_e1331_d_n8: f64 = (eq11_e1329_d_n8 * locals.var_cox);
        let eq11_e1331_d_n9: f64 = (eq11_e1329_d_n9 * locals.var_cox);
        let eq11_e1331_d_n10: f64 = (eq11_e1329_d_n10 * locals.var_cox);
        let eq11_e1331_d_n11: f64 = (eq11_e1329_d_n11 * locals.var_cox);
        let eq11_e1331_d_n12: f64 = (eq11_e1329_d_n12 * locals.var_cox);
        let eq11_e1331_d_n13: f64 = (eq11_e1329_d_n13 * locals.var_cox);
        let eq11_e1331_d_n14: f64 = (eq11_e1329_d_n14 * locals.var_cox);
        let eq11_e1333: f64 = (eq11_e1331 * locals.var_weff);
        let eq11_e1333_d_n0: f64 = (eq11_e1331_d_n0 * locals.var_weff);
        let eq11_e1333_d_n2: f64 = (eq11_e1331_d_n2 * locals.var_weff);
        let eq11_e1333_d_n3: f64 = (eq11_e1331_d_n3 * locals.var_weff);
        let eq11_e1333_d_n4: f64 = (eq11_e1331_d_n4 * locals.var_weff);
        let eq11_e1333_d_n5: f64 = (eq11_e1331_d_n5 * locals.var_weff);
        let eq11_e1333_d_n6: f64 = (eq11_e1331_d_n6 * locals.var_weff);
        let eq11_e1333_d_n7: f64 = (eq11_e1331_d_n7 * locals.var_weff);
        let eq11_e1333_d_n8: f64 = (eq11_e1331_d_n8 * locals.var_weff);
        let eq11_e1333_d_n9: f64 = (eq11_e1331_d_n9 * locals.var_weff);
        let eq11_e1333_d_n10: f64 = (eq11_e1331_d_n10 * locals.var_weff);
        let eq11_e1333_d_n11: f64 = (eq11_e1331_d_n11 * locals.var_weff);
        let eq11_e1333_d_n12: f64 = (eq11_e1331_d_n12 * locals.var_weff);
        let eq11_e1333_d_n13: f64 = (eq11_e1331_d_n13 * locals.var_weff);
        let eq11_e1333_d_n14: f64 = (eq11_e1331_d_n14 * locals.var_weff);
        let eq11_e1335: f64 = (eq11_e1333 * p.p2);
        let eq11_e1335_d_n0: f64 = (eq11_e1333_d_n0 * p.p2);
        let eq11_e1335_d_n2: f64 = (eq11_e1333_d_n2 * p.p2);
        let eq11_e1335_d_n3: f64 = (eq11_e1333_d_n3 * p.p2);
        let eq11_e1335_d_n4: f64 = (eq11_e1333_d_n4 * p.p2);
        let eq11_e1335_d_n5: f64 = (eq11_e1333_d_n5 * p.p2);
        let eq11_e1335_d_n6: f64 = (eq11_e1333_d_n6 * p.p2);
        let eq11_e1335_d_n7: f64 = (eq11_e1333_d_n7 * p.p2);
        let eq11_e1335_d_n8: f64 = (eq11_e1333_d_n8 * p.p2);
        let eq11_e1335_d_n9: f64 = (eq11_e1333_d_n9 * p.p2);
        let eq11_e1335_d_n10: f64 = (eq11_e1333_d_n10 * p.p2);
        let eq11_e1335_d_n11: f64 = (eq11_e1333_d_n11 * p.p2);
        let eq11_e1335_d_n12: f64 = (eq11_e1333_d_n12 * p.p2);
        let eq11_e1335_d_n13: f64 = (eq11_e1333_d_n13 * p.p2);
        let eq11_e1335_d_n14: f64 = (eq11_e1333_d_n14 * p.p2);
        let eq11_e1337: f64 = (eq11_e1335 * locals.var_leff);
        let eq11_e1337_d_n0: f64 = (eq11_e1335_d_n0 * locals.var_leff);
        let eq11_e1337_d_n2: f64 = (eq11_e1335_d_n2 * locals.var_leff);
        let eq11_e1337_d_n3: f64 = (eq11_e1335_d_n3 * locals.var_leff);
        let eq11_e1337_d_n4: f64 = (eq11_e1335_d_n4 * locals.var_leff);
        let eq11_e1337_d_n5: f64 = (eq11_e1335_d_n5 * locals.var_leff);
        let eq11_e1337_d_n6: f64 = (eq11_e1335_d_n6 * locals.var_leff);
        let eq11_e1337_d_n7: f64 = (eq11_e1335_d_n7 * locals.var_leff);
        let eq11_e1337_d_n8: f64 = (eq11_e1335_d_n8 * locals.var_leff);
        let eq11_e1337_d_n9: f64 = (eq11_e1335_d_n9 * locals.var_leff);
        let eq11_e1337_d_n10: f64 = (eq11_e1335_d_n10 * locals.var_leff);
        let eq11_e1337_d_n11: f64 = (eq11_e1335_d_n11 * locals.var_leff);
        let eq11_e1337_d_n12: f64 = (eq11_e1335_d_n12 * locals.var_leff);
        let eq11_e1337_d_n13: f64 = (eq11_e1335_d_n13 * locals.var_leff);
        let eq11_e1337_d_n14: f64 = (eq11_e1335_d_n14 * locals.var_leff);
        let eq11_e1339: f64 = (eq11_e1337 * (nv15 - 0.0));
        let eq11_e1339_d_n0: f64 = (eq11_e1337_d_n0 * (nv15 - 0.0));
        let eq11_e1339_d_n2: f64 = (eq11_e1337_d_n2 * (nv15 - 0.0));
        let eq11_e1339_d_n3: f64 = (eq11_e1337_d_n3 * (nv15 - 0.0));
        let eq11_e1339_d_n4: f64 = (eq11_e1337_d_n4 * (nv15 - 0.0));
        let eq11_e1339_d_n5: f64 = (eq11_e1337_d_n5 * (nv15 - 0.0));
        let eq11_e1339_d_n6: f64 = (eq11_e1337_d_n6 * (nv15 - 0.0));
        let eq11_e1339_d_n7: f64 = (eq11_e1337_d_n7 * (nv15 - 0.0));
        let eq11_e1339_d_n8: f64 = (eq11_e1337_d_n8 * (nv15 - 0.0));
        let eq11_e1339_d_n9: f64 = (eq11_e1337_d_n9 * (nv15 - 0.0));
        let eq11_e1339_d_n10: f64 = (eq11_e1337_d_n10 * (nv15 - 0.0));
        let eq11_e1339_d_n11: f64 = (eq11_e1337_d_n11 * (nv15 - 0.0));
        let eq11_e1339_d_n12: f64 = (eq11_e1337_d_n12 * (nv15 - 0.0));
        let eq11_e1339_d_n13: f64 = (eq11_e1337_d_n13 * (nv15 - 0.0));
        let eq11_e1339_d_n14: f64 = (eq11_e1337_d_n14 * (nv15 - 0.0));
        let eq11_e1340: f64 = (0.5 * eq11_e1339);
        let eq11_e1340_d_n0: f64 = (0.5 * eq11_e1339_d_n0);
        let eq11_e1340_d_n2: f64 = (0.5 * eq11_e1339_d_n2);
        let eq11_e1340_d_n3: f64 = (0.5 * eq11_e1339_d_n3);
        let eq11_e1340_d_n4: f64 = (0.5 * eq11_e1339_d_n4);
        let eq11_e1340_d_n5: f64 = (0.5 * eq11_e1339_d_n5);
        let eq11_e1340_d_n6: f64 = (0.5 * eq11_e1339_d_n6);
        let eq11_e1340_d_n7: f64 = (0.5 * eq11_e1339_d_n7);
        let eq11_e1340_d_n8: f64 = (0.5 * eq11_e1339_d_n8);
        let eq11_e1340_d_n9: f64 = (0.5 * eq11_e1339_d_n9);
        let eq11_e1340_d_n10: f64 = (0.5 * eq11_e1339_d_n10);
        let eq11_e1340_d_n11: f64 = (0.5 * eq11_e1339_d_n11);
        let eq11_e1340_d_n12: f64 = (0.5 * eq11_e1339_d_n12);
        let eq11_e1340_d_n13: f64 = (0.5 * eq11_e1339_d_n13);
        let eq11_e1340_d_n14: f64 = (0.5 * eq11_e1339_d_n14);
        let eq11_e1340_d_n15: f64 = (0.5 * eq11_e1337);
        let eq11_e1341_q: f64 = eq11_e1340;
        let eq11_e1342: f64 = (p.p29 * eq11_e1340);
        let eq11_e1342_d_n0: f64 = (p.p29 * eq11_e1340_d_n0);
        let eq11_e1342_d_n2: f64 = (p.p29 * eq11_e1340_d_n2);
        let eq11_e1342_d_n3: f64 = (p.p29 * eq11_e1340_d_n3);
        let eq11_e1342_d_n4: f64 = (p.p29 * eq11_e1340_d_n4);
        let eq11_e1342_d_n5: f64 = (p.p29 * eq11_e1340_d_n5);
        let eq11_e1342_d_n6: f64 = (p.p29 * eq11_e1340_d_n6);
        let eq11_e1342_d_n7: f64 = (p.p29 * eq11_e1340_d_n7);
        let eq11_e1342_d_n8: f64 = (p.p29 * eq11_e1340_d_n8);
        let eq11_e1342_d_n9: f64 = (p.p29 * eq11_e1340_d_n9);
        let eq11_e1342_d_n10: f64 = (p.p29 * eq11_e1340_d_n10);
        let eq11_e1342_d_n11: f64 = (p.p29 * eq11_e1340_d_n11);
        let eq11_e1342_d_n12: f64 = (p.p29 * eq11_e1340_d_n12);
        let eq11_e1342_d_n13: f64 = (p.p29 * eq11_e1340_d_n13);
        let eq11_e1342_d_n14: f64 = (p.p29 * eq11_e1340_d_n14);
        let eq11_e1342_d_n15: f64 = (p.p29 * eq11_e1340_d_n15);
        let eq11_e1342_q: f64 = (p.p29 * eq11_e1341_q);
        (eq11_e1342, eq11_e1342_d_n0, eq11_e1342_d_n2, eq11_e1342_d_n3, eq11_e1342_d_n4, eq11_e1342_d_n5, eq11_e1342_d_n6, eq11_e1342_d_n7, eq11_e1342_d_n8, eq11_e1342_d_n9, eq11_e1342_d_n10, eq11_e1342_d_n11, eq11_e1342_d_n12, eq11_e1342_d_n13, eq11_e1342_d_n14, eq11_e1342_d_n15, eq11_e1342_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq11_reactive_node_derivatives: [f64; 17] = [eq11_e1344_d_n0, 0.0, eq11_e1344_d_n2, eq11_e1344_d_n3, eq11_e1344_d_n4, eq11_e1344_d_n5, eq11_e1344_d_n6, eq11_e1344_d_n7, eq11_e1344_d_n8, eq11_e1344_d_n9, eq11_e1344_d_n10, eq11_e1344_d_n11, eq11_e1344_d_n12, eq11_e1344_d_n13, eq11_e1344_d_n14, eq11_e1344_d_n15, 0.0];
        let eq11_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq12_e1370, eq12_e1370_d_n0, eq12_e1370_d_n2, eq12_e1370_d_n3, eq12_e1370_d_n4, eq12_e1370_d_n5, eq12_e1370_d_n6, eq12_e1370_d_n7, eq12_e1370_d_n8, eq12_e1370_d_n9, eq12_e1370_d_n10, eq12_e1370_d_n11, eq12_e1370_d_n12, eq12_e1370_d_n13, eq12_e1370_d_n14, eq12_e1370_d_n15, eq12_e1370_q,) = {
    if ((locals.var_guard698 != 0.0) && (locals.var_guard697 == 0.0)) {
        let eq12_e1353: f64 = (1.0 - locals.var_sigvds);
        let eq12_e1355: f64 = (eq12_e1353 * locals.var_mig);
        let eq12_e1355_d_n0: f64 = (eq12_e1353 * locals.var_mig_dn0);
        let eq12_e1355_d_n2: f64 = (eq12_e1353 * locals.var_mig_dn2);
        let eq12_e1355_d_n3: f64 = (eq12_e1353 * locals.var_mig_dn3);
        let eq12_e1355_d_n4: f64 = (eq12_e1353 * locals.var_mig_dn4);
        let eq12_e1355_d_n5: f64 = (eq12_e1353 * locals.var_mig_dn5);
        let eq12_e1355_d_n6: f64 = (eq12_e1353 * locals.var_mig_dn6);
        let eq12_e1355_d_n7: f64 = (eq12_e1353 * locals.var_mig_dn7);
        let eq12_e1355_d_n8: f64 = (eq12_e1353 * locals.var_mig_dn8);
        let eq12_e1355_d_n9: f64 = (eq12_e1353 * locals.var_mig_dn9);
        let eq12_e1355_d_n10: f64 = (eq12_e1353 * locals.var_mig_dn10);
        let eq12_e1355_d_n11: f64 = (eq12_e1353 * locals.var_mig_dn11);
        let eq12_e1355_d_n12: f64 = (eq12_e1353 * locals.var_mig_dn12);
        let eq12_e1355_d_n13: f64 = (eq12_e1353 * locals.var_mig_dn13);
        let eq12_e1355_d_n14: f64 = (eq12_e1353 * locals.var_mig_dn14);
        let eq12_e1357: f64 = (eq12_e1355 * locals.var_cox);
        let eq12_e1357_d_n0: f64 = (eq12_e1355_d_n0 * locals.var_cox);
        let eq12_e1357_d_n2: f64 = (eq12_e1355_d_n2 * locals.var_cox);
        let eq12_e1357_d_n3: f64 = (eq12_e1355_d_n3 * locals.var_cox);
        let eq12_e1357_d_n4: f64 = (eq12_e1355_d_n4 * locals.var_cox);
        let eq12_e1357_d_n5: f64 = (eq12_e1355_d_n5 * locals.var_cox);
        let eq12_e1357_d_n6: f64 = (eq12_e1355_d_n6 * locals.var_cox);
        let eq12_e1357_d_n7: f64 = (eq12_e1355_d_n7 * locals.var_cox);
        let eq12_e1357_d_n8: f64 = (eq12_e1355_d_n8 * locals.var_cox);
        let eq12_e1357_d_n9: f64 = (eq12_e1355_d_n9 * locals.var_cox);
        let eq12_e1357_d_n10: f64 = (eq12_e1355_d_n10 * locals.var_cox);
        let eq12_e1357_d_n11: f64 = (eq12_e1355_d_n11 * locals.var_cox);
        let eq12_e1357_d_n12: f64 = (eq12_e1355_d_n12 * locals.var_cox);
        let eq12_e1357_d_n13: f64 = (eq12_e1355_d_n13 * locals.var_cox);
        let eq12_e1357_d_n14: f64 = (eq12_e1355_d_n14 * locals.var_cox);
        let eq12_e1359: f64 = (eq12_e1357 * locals.var_weff);
        let eq12_e1359_d_n0: f64 = (eq12_e1357_d_n0 * locals.var_weff);
        let eq12_e1359_d_n2: f64 = (eq12_e1357_d_n2 * locals.var_weff);
        let eq12_e1359_d_n3: f64 = (eq12_e1357_d_n3 * locals.var_weff);
        let eq12_e1359_d_n4: f64 = (eq12_e1357_d_n4 * locals.var_weff);
        let eq12_e1359_d_n5: f64 = (eq12_e1357_d_n5 * locals.var_weff);
        let eq12_e1359_d_n6: f64 = (eq12_e1357_d_n6 * locals.var_weff);
        let eq12_e1359_d_n7: f64 = (eq12_e1357_d_n7 * locals.var_weff);
        let eq12_e1359_d_n8: f64 = (eq12_e1357_d_n8 * locals.var_weff);
        let eq12_e1359_d_n9: f64 = (eq12_e1357_d_n9 * locals.var_weff);
        let eq12_e1359_d_n10: f64 = (eq12_e1357_d_n10 * locals.var_weff);
        let eq12_e1359_d_n11: f64 = (eq12_e1357_d_n11 * locals.var_weff);
        let eq12_e1359_d_n12: f64 = (eq12_e1357_d_n12 * locals.var_weff);
        let eq12_e1359_d_n13: f64 = (eq12_e1357_d_n13 * locals.var_weff);
        let eq12_e1359_d_n14: f64 = (eq12_e1357_d_n14 * locals.var_weff);
        let eq12_e1361: f64 = (eq12_e1359 * p.p2);
        let eq12_e1361_d_n0: f64 = (eq12_e1359_d_n0 * p.p2);
        let eq12_e1361_d_n2: f64 = (eq12_e1359_d_n2 * p.p2);
        let eq12_e1361_d_n3: f64 = (eq12_e1359_d_n3 * p.p2);
        let eq12_e1361_d_n4: f64 = (eq12_e1359_d_n4 * p.p2);
        let eq12_e1361_d_n5: f64 = (eq12_e1359_d_n5 * p.p2);
        let eq12_e1361_d_n6: f64 = (eq12_e1359_d_n6 * p.p2);
        let eq12_e1361_d_n7: f64 = (eq12_e1359_d_n7 * p.p2);
        let eq12_e1361_d_n8: f64 = (eq12_e1359_d_n8 * p.p2);
        let eq12_e1361_d_n9: f64 = (eq12_e1359_d_n9 * p.p2);
        let eq12_e1361_d_n10: f64 = (eq12_e1359_d_n10 * p.p2);
        let eq12_e1361_d_n11: f64 = (eq12_e1359_d_n11 * p.p2);
        let eq12_e1361_d_n12: f64 = (eq12_e1359_d_n12 * p.p2);
        let eq12_e1361_d_n13: f64 = (eq12_e1359_d_n13 * p.p2);
        let eq12_e1361_d_n14: f64 = (eq12_e1359_d_n14 * p.p2);
        let eq12_e1363: f64 = (eq12_e1361 * locals.var_leff);
        let eq12_e1363_d_n0: f64 = (eq12_e1361_d_n0 * locals.var_leff);
        let eq12_e1363_d_n2: f64 = (eq12_e1361_d_n2 * locals.var_leff);
        let eq12_e1363_d_n3: f64 = (eq12_e1361_d_n3 * locals.var_leff);
        let eq12_e1363_d_n4: f64 = (eq12_e1361_d_n4 * locals.var_leff);
        let eq12_e1363_d_n5: f64 = (eq12_e1361_d_n5 * locals.var_leff);
        let eq12_e1363_d_n6: f64 = (eq12_e1361_d_n6 * locals.var_leff);
        let eq12_e1363_d_n7: f64 = (eq12_e1361_d_n7 * locals.var_leff);
        let eq12_e1363_d_n8: f64 = (eq12_e1361_d_n8 * locals.var_leff);
        let eq12_e1363_d_n9: f64 = (eq12_e1361_d_n9 * locals.var_leff);
        let eq12_e1363_d_n10: f64 = (eq12_e1361_d_n10 * locals.var_leff);
        let eq12_e1363_d_n11: f64 = (eq12_e1361_d_n11 * locals.var_leff);
        let eq12_e1363_d_n12: f64 = (eq12_e1361_d_n12 * locals.var_leff);
        let eq12_e1363_d_n13: f64 = (eq12_e1361_d_n13 * locals.var_leff);
        let eq12_e1363_d_n14: f64 = (eq12_e1361_d_n14 * locals.var_leff);
        let eq12_e1365: f64 = (eq12_e1363 * (nv15 - 0.0));
        let eq12_e1365_d_n0: f64 = (eq12_e1363_d_n0 * (nv15 - 0.0));
        let eq12_e1365_d_n2: f64 = (eq12_e1363_d_n2 * (nv15 - 0.0));
        let eq12_e1365_d_n3: f64 = (eq12_e1363_d_n3 * (nv15 - 0.0));
        let eq12_e1365_d_n4: f64 = (eq12_e1363_d_n4 * (nv15 - 0.0));
        let eq12_e1365_d_n5: f64 = (eq12_e1363_d_n5 * (nv15 - 0.0));
        let eq12_e1365_d_n6: f64 = (eq12_e1363_d_n6 * (nv15 - 0.0));
        let eq12_e1365_d_n7: f64 = (eq12_e1363_d_n7 * (nv15 - 0.0));
        let eq12_e1365_d_n8: f64 = (eq12_e1363_d_n8 * (nv15 - 0.0));
        let eq12_e1365_d_n9: f64 = (eq12_e1363_d_n9 * (nv15 - 0.0));
        let eq12_e1365_d_n10: f64 = (eq12_e1363_d_n10 * (nv15 - 0.0));
        let eq12_e1365_d_n11: f64 = (eq12_e1363_d_n11 * (nv15 - 0.0));
        let eq12_e1365_d_n12: f64 = (eq12_e1363_d_n12 * (nv15 - 0.0));
        let eq12_e1365_d_n13: f64 = (eq12_e1363_d_n13 * (nv15 - 0.0));
        let eq12_e1365_d_n14: f64 = (eq12_e1363_d_n14 * (nv15 - 0.0));
        let eq12_e1366: f64 = (0.5 * eq12_e1365);
        let eq12_e1366_d_n0: f64 = (0.5 * eq12_e1365_d_n0);
        let eq12_e1366_d_n2: f64 = (0.5 * eq12_e1365_d_n2);
        let eq12_e1366_d_n3: f64 = (0.5 * eq12_e1365_d_n3);
        let eq12_e1366_d_n4: f64 = (0.5 * eq12_e1365_d_n4);
        let eq12_e1366_d_n5: f64 = (0.5 * eq12_e1365_d_n5);
        let eq12_e1366_d_n6: f64 = (0.5 * eq12_e1365_d_n6);
        let eq12_e1366_d_n7: f64 = (0.5 * eq12_e1365_d_n7);
        let eq12_e1366_d_n8: f64 = (0.5 * eq12_e1365_d_n8);
        let eq12_e1366_d_n9: f64 = (0.5 * eq12_e1365_d_n9);
        let eq12_e1366_d_n10: f64 = (0.5 * eq12_e1365_d_n10);
        let eq12_e1366_d_n11: f64 = (0.5 * eq12_e1365_d_n11);
        let eq12_e1366_d_n12: f64 = (0.5 * eq12_e1365_d_n12);
        let eq12_e1366_d_n13: f64 = (0.5 * eq12_e1365_d_n13);
        let eq12_e1366_d_n14: f64 = (0.5 * eq12_e1365_d_n14);
        let eq12_e1366_d_n15: f64 = (0.5 * eq12_e1363);
        let eq12_e1367_q: f64 = eq12_e1366;
        let eq12_e1368: f64 = (p.p29 * eq12_e1366);
        let eq12_e1368_d_n0: f64 = (p.p29 * eq12_e1366_d_n0);
        let eq12_e1368_d_n2: f64 = (p.p29 * eq12_e1366_d_n2);
        let eq12_e1368_d_n3: f64 = (p.p29 * eq12_e1366_d_n3);
        let eq12_e1368_d_n4: f64 = (p.p29 * eq12_e1366_d_n4);
        let eq12_e1368_d_n5: f64 = (p.p29 * eq12_e1366_d_n5);
        let eq12_e1368_d_n6: f64 = (p.p29 * eq12_e1366_d_n6);
        let eq12_e1368_d_n7: f64 = (p.p29 * eq12_e1366_d_n7);
        let eq12_e1368_d_n8: f64 = (p.p29 * eq12_e1366_d_n8);
        let eq12_e1368_d_n9: f64 = (p.p29 * eq12_e1366_d_n9);
        let eq12_e1368_d_n10: f64 = (p.p29 * eq12_e1366_d_n10);
        let eq12_e1368_d_n11: f64 = (p.p29 * eq12_e1366_d_n11);
        let eq12_e1368_d_n12: f64 = (p.p29 * eq12_e1366_d_n12);
        let eq12_e1368_d_n13: f64 = (p.p29 * eq12_e1366_d_n13);
        let eq12_e1368_d_n14: f64 = (p.p29 * eq12_e1366_d_n14);
        let eq12_e1368_d_n15: f64 = (p.p29 * eq12_e1366_d_n15);
        let eq12_e1368_q: f64 = (p.p29 * eq12_e1367_q);
        (eq12_e1368, eq12_e1368_d_n0, eq12_e1368_d_n2, eq12_e1368_d_n3, eq12_e1368_d_n4, eq12_e1368_d_n5, eq12_e1368_d_n6, eq12_e1368_d_n7, eq12_e1368_d_n8, eq12_e1368_d_n9, eq12_e1368_d_n10, eq12_e1368_d_n11, eq12_e1368_d_n12, eq12_e1368_d_n13, eq12_e1368_d_n14, eq12_e1368_d_n15, eq12_e1368_q,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq12_reactive_node_derivatives: [f64; 17] = [eq12_e1370_d_n0, 0.0, eq12_e1370_d_n2, eq12_e1370_d_n3, eq12_e1370_d_n4, eq12_e1370_d_n5, eq12_e1370_d_n6, eq12_e1370_d_n7, eq12_e1370_d_n8, eq12_e1370_d_n9, eq12_e1370_d_n10, eq12_e1370_d_n11, eq12_e1370_d_n12, eq12_e1370_d_n13, eq12_e1370_d_n14, eq12_e1370_d_n15, 0.0];
        let eq12_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[5]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e1428_q: f64 = locals.var_qgi_1;
        let eq19_reactive_node_derivatives: [f64; 17] = [locals.var_qgi_1_dn0, 0.0, locals.var_qgi_1_dn2, locals.var_qgi_1_dn3, locals.var_qgi_1_dn4, locals.var_qgi_1_dn5, locals.var_qgi_1_dn6, locals.var_qgi_1_dn7, locals.var_qgi_1_dn8, locals.var_qgi_1_dn9, locals.var_qgi_1_dn10, locals.var_qgi_1_dn11, locals.var_qgi_1_dn12, locals.var_qgi_1_dn13, locals.var_qgi_1_dn14, 0.0, 0.0];
        let eq19_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[11]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let eq20_e1430_q: f64 = locals.var_qsi_1;
        let eq20_reactive_node_derivatives: [f64; 17] = [locals.var_qsi_1_dn0, 0.0, locals.var_qsi_1_dn2, locals.var_qsi_1_dn3, locals.var_qsi_1_dn4, locals.var_qsi_1_dn5, locals.var_qsi_1_dn6, locals.var_qsi_1_dn7, locals.var_qsi_1_dn8, locals.var_qsi_1_dn9, locals.var_qsi_1_dn10, locals.var_qsi_1_dn11, locals.var_qsi_1_dn12, locals.var_qsi_1_dn13, locals.var_qsi_1_dn14, 0.0, 0.0];
        let eq20_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[11]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
        let eq21_e1432_q: f64 = locals.var_qdi_1;
        let eq21_reactive_node_derivatives: [f64; 17] = [locals.var_qdi_1_dn0, 0.0, locals.var_qdi_1_dn2, locals.var_qdi_1_dn3, locals.var_qdi_1_dn4, locals.var_qdi_1_dn5, locals.var_qdi_1_dn6, locals.var_qdi_1_dn7, locals.var_qdi_1_dn8, locals.var_qdi_1_dn9, locals.var_qdi_1_dn10, locals.var_qdi_1_dn11, locals.var_qdi_1_dn12, locals.var_qdi_1_dn13, locals.var_qdi_1_dn14, 0.0, 0.0];
        let eq21_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[11]),
            nodes,
            &eq21_reactive_node_derivatives,
            branches,
            &eq21_reactive_branch_derivatives,
            multiplicity,
        );
        let eq22_e1435: f64 = (-locals.var_devsign);
        let eq22_e1437: f64 = (eq22_e1435 * locals.var_qovs);
        let eq22_e1437_d_n0: f64 = (eq22_e1435 * locals.var_qovs_dn0);
        let eq22_e1437_d_n2: f64 = (eq22_e1435 * locals.var_qovs_dn2);
        let eq22_e1437_d_n3: f64 = (eq22_e1435 * locals.var_qovs_dn3);
        let eq22_e1437_d_n4: f64 = (eq22_e1435 * locals.var_qovs_dn4);
        let eq22_e1437_d_n5: f64 = (eq22_e1435 * locals.var_qovs_dn5);
        let eq22_e1437_d_n6: f64 = (eq22_e1435 * locals.var_qovs_dn6);
        let eq22_e1437_d_n7: f64 = (eq22_e1435 * locals.var_qovs_dn7);
        let eq22_e1437_d_n8: f64 = (eq22_e1435 * locals.var_qovs_dn8);
        let eq22_e1437_d_n9: f64 = (eq22_e1435 * locals.var_qovs_dn9);
        let eq22_e1437_d_n10: f64 = (eq22_e1435 * locals.var_qovs_dn10);
        let eq22_e1437_d_n11: f64 = (eq22_e1435 * locals.var_qovs_dn11);
        let eq22_e1437_d_n12: f64 = (eq22_e1435 * locals.var_qovs_dn12);
        let eq22_e1437_d_n13: f64 = (eq22_e1435 * locals.var_qovs_dn13);
        let eq22_e1437_d_n14: f64 = (eq22_e1435 * locals.var_qovs_dn14);
        let eq22_e1438_q: f64 = eq22_e1437;
        let eq22_e1439: f64 = (p.p29 * eq22_e1437);
        let eq22_e1439_d_n0: f64 = (p.p29 * eq22_e1437_d_n0);
        let eq22_e1439_d_n2: f64 = (p.p29 * eq22_e1437_d_n2);
        let eq22_e1439_d_n3: f64 = (p.p29 * eq22_e1437_d_n3);
        let eq22_e1439_d_n4: f64 = (p.p29 * eq22_e1437_d_n4);
        let eq22_e1439_d_n5: f64 = (p.p29 * eq22_e1437_d_n5);
        let eq22_e1439_d_n6: f64 = (p.p29 * eq22_e1437_d_n6);
        let eq22_e1439_d_n7: f64 = (p.p29 * eq22_e1437_d_n7);
        let eq22_e1439_d_n8: f64 = (p.p29 * eq22_e1437_d_n8);
        let eq22_e1439_d_n9: f64 = (p.p29 * eq22_e1437_d_n9);
        let eq22_e1439_d_n10: f64 = (p.p29 * eq22_e1437_d_n10);
        let eq22_e1439_d_n11: f64 = (p.p29 * eq22_e1437_d_n11);
        let eq22_e1439_d_n12: f64 = (p.p29 * eq22_e1437_d_n12);
        let eq22_e1439_d_n13: f64 = (p.p29 * eq22_e1437_d_n13);
        let eq22_e1439_d_n14: f64 = (p.p29 * eq22_e1437_d_n14);
        let eq22_e1439_q: f64 = (p.p29 * eq22_e1438_q);
        let eq22_reactive_node_derivatives: [f64; 17] = [eq22_e1439_d_n0, 0.0, eq22_e1439_d_n2, eq22_e1439_d_n3, eq22_e1439_d_n4, eq22_e1439_d_n5, eq22_e1439_d_n6, eq22_e1439_d_n7, eq22_e1439_d_n8, eq22_e1439_d_n9, eq22_e1439_d_n10, eq22_e1439_d_n11, eq22_e1439_d_n12, eq22_e1439_d_n13, eq22_e1439_d_n14, 0.0, 0.0];
        let eq22_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[7]),
            nodes,
            &eq22_reactive_node_derivatives,
            branches,
            &eq22_reactive_branch_derivatives,
            multiplicity,
        );
        let eq23_e1442: f64 = (-locals.var_devsign);
        let eq23_e1444: f64 = (eq23_e1442 * locals.var_qovd);
        let eq23_e1444_d_n0: f64 = (eq23_e1442 * locals.var_qovd_dn0);
        let eq23_e1444_d_n2: f64 = (eq23_e1442 * locals.var_qovd_dn2);
        let eq23_e1444_d_n3: f64 = (eq23_e1442 * locals.var_qovd_dn3);
        let eq23_e1444_d_n4: f64 = (eq23_e1442 * locals.var_qovd_dn4);
        let eq23_e1444_d_n5: f64 = (eq23_e1442 * locals.var_qovd_dn5);
        let eq23_e1444_d_n6: f64 = (eq23_e1442 * locals.var_qovd_dn6);
        let eq23_e1444_d_n7: f64 = (eq23_e1442 * locals.var_qovd_dn7);
        let eq23_e1444_d_n8: f64 = (eq23_e1442 * locals.var_qovd_dn8);
        let eq23_e1444_d_n9: f64 = (eq23_e1442 * locals.var_qovd_dn9);
        let eq23_e1444_d_n10: f64 = (eq23_e1442 * locals.var_qovd_dn10);
        let eq23_e1444_d_n11: f64 = (eq23_e1442 * locals.var_qovd_dn11);
        let eq23_e1444_d_n12: f64 = (eq23_e1442 * locals.var_qovd_dn12);
        let eq23_e1444_d_n13: f64 = (eq23_e1442 * locals.var_qovd_dn13);
        let eq23_e1444_d_n14: f64 = (eq23_e1442 * locals.var_qovd_dn14);
        let eq23_e1445_q: f64 = eq23_e1444;
        let eq23_e1446: f64 = (p.p29 * eq23_e1444);
        let eq23_e1446_d_n0: f64 = (p.p29 * eq23_e1444_d_n0);
        let eq23_e1446_d_n2: f64 = (p.p29 * eq23_e1444_d_n2);
        let eq23_e1446_d_n3: f64 = (p.p29 * eq23_e1444_d_n3);
        let eq23_e1446_d_n4: f64 = (p.p29 * eq23_e1444_d_n4);
        let eq23_e1446_d_n5: f64 = (p.p29 * eq23_e1444_d_n5);
        let eq23_e1446_d_n6: f64 = (p.p29 * eq23_e1444_d_n6);
        let eq23_e1446_d_n7: f64 = (p.p29 * eq23_e1444_d_n7);
        let eq23_e1446_d_n8: f64 = (p.p29 * eq23_e1444_d_n8);
        let eq23_e1446_d_n9: f64 = (p.p29 * eq23_e1444_d_n9);
        let eq23_e1446_d_n10: f64 = (p.p29 * eq23_e1444_d_n10);
        let eq23_e1446_d_n11: f64 = (p.p29 * eq23_e1444_d_n11);
        let eq23_e1446_d_n12: f64 = (p.p29 * eq23_e1444_d_n12);
        let eq23_e1446_d_n13: f64 = (p.p29 * eq23_e1444_d_n13);
        let eq23_e1446_d_n14: f64 = (p.p29 * eq23_e1444_d_n14);
        let eq23_e1446_q: f64 = (p.p29 * eq23_e1445_q);
        let eq23_reactive_node_derivatives: [f64; 17] = [eq23_e1446_d_n0, 0.0, eq23_e1446_d_n2, eq23_e1446_d_n3, eq23_e1446_d_n4, eq23_e1446_d_n5, eq23_e1446_d_n6, eq23_e1446_d_n7, eq23_e1446_d_n8, eq23_e1446_d_n9, eq23_e1446_d_n10, eq23_e1446_d_n11, eq23_e1446_d_n12, eq23_e1446_d_n13, eq23_e1446_d_n14, 0.0, 0.0];
        let eq23_reactive_branch_derivatives: [f64; 14] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[5]),
            nodes,
            &eq23_reactive_node_derivatives,
            branches,
            &eq23_reactive_branch_derivatives,
            multiplicity,
        );
    }
}

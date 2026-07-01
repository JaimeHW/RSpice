#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_43(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18090_e12594, assign18090_e12594_d_n0, assign18090_e12594_d_n2, assign18090_e12594_d_n4, assign18090_e12594_d_n5, assign18090_e12594_d_n6, assign18090_e12594_d_n7, assign18090_e12594_d_n8, assign18090_e12594_d_n9, assign18090_e12594_d_n10, assign18090_e12594_d_n11, assign18090_e12594_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign18090_e12590: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18090_e12591: f64 = (1.0 + assign18090_e12590);
        let assign18090_e12592: f64 = (0.5 * assign18090_e12591);
        (assign18090_e12592, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18090_e12594;
        locals.var_t0_dn0 = assign18090_e12594_d_n0;
        locals.var_t0_dn2 = assign18090_e12594_d_n2;
        locals.var_t0_dn4 = assign18090_e12594_d_n4;
        locals.var_t0_dn5 = assign18090_e12594_d_n5;
        locals.var_t0_dn6 = assign18090_e12594_d_n6;
        locals.var_t0_dn7 = assign18090_e12594_d_n7;
        locals.var_t0_dn8 = assign18090_e12594_d_n8;
        locals.var_t0_dn9 = assign18090_e12594_d_n9;
        locals.var_t0_dn10 = assign18090_e12594_d_n10;
        locals.var_t0_dn11 = assign18090_e12594_d_n11;
        locals.var_t0_dn14 = assign18090_e12594_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign18100_e12604, assign18100_e12604_d_n0, assign18100_e12604_d_n2, assign18100_e12604_d_n4, assign18100_e12604_d_n5, assign18100_e12604_d_n6, assign18100_e12604_d_n7, assign18100_e12604_d_n8, assign18100_e12604_d_n9, assign18100_e12604_d_n10, assign18100_e12604_d_n11, assign18100_e12604_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign18100_e12600: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18100_e12601: f64 = (0.5 * assign18100_e12600);
        let assign18100_e12602: f64 = (1.0 - assign18100_e12601);
        (assign18100_e12602, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_powratio, locals.var_powratio_dn0, locals.var_powratio_dn2, locals.var_powratio_dn4, locals.var_powratio_dn5, locals.var_powratio_dn6, locals.var_powratio_dn7, locals.var_powratio_dn8, locals.var_powratio_dn9, locals.var_powratio_dn10, locals.var_powratio_dn11, locals.var_powratio_dn14,)
    }
};
        locals.var_powratio = assign18100_e12604;
        locals.var_powratio_dn0 = assign18100_e12604_d_n0;
        locals.var_powratio_dn2 = assign18100_e12604_d_n2;
        locals.var_powratio_dn4 = assign18100_e12604_d_n4;
        locals.var_powratio_dn5 = assign18100_e12604_d_n5;
        locals.var_powratio_dn6 = assign18100_e12604_d_n6;
        locals.var_powratio_dn7 = assign18100_e12604_d_n7;
        locals.var_powratio_dn8 = assign18100_e12604_d_n8;
        locals.var_powratio_dn9 = assign18100_e12604_d_n9;
        locals.var_powratio_dn10 = assign18100_e12604_d_n10;
        locals.var_powratio_dn11 = assign18100_e12604_d_n11;
        locals.var_powratio_dn14 = assign18100_e12604_d_n14;
        locals.var_powratio_rv = 0.0;

        let (assign18110_e12615, assign18110_e12615_d_n0, assign18110_e12615_d_n2, assign18110_e12615_d_n4, assign18110_e12615_d_n5, assign18110_e12615_d_n6, assign18110_e12615_d_n7, assign18110_e12615_d_n8, assign18110_e12615_d_n9, assign18110_e12615_d_n10, assign18110_e12615_d_n11, assign18110_e12615_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign18110_e12608: f64 = (2.0 * locals.var_beta_inv);
        let assign18110_e12611: f64 = (locals.var_nsub / locals.var_nin);
        let assign18110_e12612: f64 = (assign18110_e12611).ln();
        let assign18110_e12613: f64 = (assign18110_e12608 * assign18110_e12612);
        (assign18110_e12613, (((2.0 * locals.var_beta_inv_dn0) * assign18110_e12612) + (assign18110_e12608 * ((((locals.var_nsub_dn0 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign18110_e12611))), (((2.0 * locals.var_beta_inv_dn2) * assign18110_e12612) + (assign18110_e12608 * ((((locals.var_nsub_dn2 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign18110_e12611))), (((2.0 * locals.var_beta_inv_dn4) * assign18110_e12612) + (assign18110_e12608 * ((((locals.var_nsub_dn4 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign18110_e12611))), (((2.0 * locals.var_beta_inv_dn5) * assign18110_e12612) + (assign18110_e12608 * ((((locals.var_nsub_dn5 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign18110_e12611))), (((2.0 * locals.var_beta_inv_dn6) * assign18110_e12612) + (assign18110_e12608 * ((((locals.var_nsub_dn6 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign18110_e12611))), (((2.0 * locals.var_beta_inv_dn7) * assign18110_e12612) + (assign18110_e12608 * ((((locals.var_nsub_dn7 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign18110_e12611))), (((2.0 * locals.var_beta_inv_dn8) * assign18110_e12612) + (assign18110_e12608 * ((((locals.var_nsub_dn8 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign18110_e12611))), (((2.0 * locals.var_beta_inv_dn9) * assign18110_e12612) + (assign18110_e12608 * ((((locals.var_nsub_dn9 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign18110_e12611))), (((2.0 * locals.var_beta_inv_dn10) * assign18110_e12612) + (assign18110_e12608 * ((((locals.var_nsub_dn10 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign18110_e12611))), (((2.0 * locals.var_beta_inv_dn11) * assign18110_e12612) + (assign18110_e12608 * ((((locals.var_nsub_dn11 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign18110_e12611))), (((2.0 * locals.var_beta_inv_dn14) * assign18110_e12612) + (assign18110_e12608 * ((((locals.var_nsub_dn14 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign18110_e12611))),)
    } else {
        (locals.var_pb2, locals.var_pb2_dn0, locals.var_pb2_dn2, locals.var_pb2_dn4, locals.var_pb2_dn5, locals.var_pb2_dn6, locals.var_pb2_dn7, locals.var_pb2_dn8, locals.var_pb2_dn9, locals.var_pb2_dn10, locals.var_pb2_dn11, locals.var_pb2_dn14,)
    }
};
        locals.var_pb2 = assign18110_e12615;
        locals.var_pb2_dn0 = assign18110_e12615_d_n0;
        locals.var_pb2_dn2 = assign18110_e12615_d_n2;
        locals.var_pb2_dn4 = assign18110_e12615_d_n4;
        locals.var_pb2_dn5 = assign18110_e12615_d_n5;
        locals.var_pb2_dn6 = assign18110_e12615_d_n6;
        locals.var_pb2_dn7 = assign18110_e12615_d_n7;
        locals.var_pb2_dn8 = assign18110_e12615_d_n8;
        locals.var_pb2_dn9 = assign18110_e12615_d_n9;
        locals.var_pb2_dn10 = assign18110_e12615_d_n10;
        locals.var_pb2_dn11 = assign18110_e12615_d_n11;
        locals.var_pb2_dn14 = assign18110_e12615_d_n14;
        locals.var_pb2_rv = 0.0;

        let (assign18120_e12623, assign18120_e12623_d_n0, assign18120_e12623_d_n2, assign18120_e12623_d_n4, assign18120_e12623_d_n5, assign18120_e12623_d_n6, assign18120_e12623_d_n7, assign18120_e12623_d_n8, assign18120_e12623_d_n9, assign18120_e12623_d_n10, assign18120_e12623_d_n11, assign18120_e12623_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign18120_e12619: f64 = (2.0 * 1.034943e-10);
        let assign18120_e12621: f64 = (assign18120_e12619 / 1.6021918e-19);
        (assign18120_e12621, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign18120_e12623;
        locals.var_t1_dn0 = assign18120_e12623_d_n0;
        locals.var_t1_dn2 = assign18120_e12623_d_n2;
        locals.var_t1_dn4 = assign18120_e12623_d_n4;
        locals.var_t1_dn5 = assign18120_e12623_d_n5;
        locals.var_t1_dn6 = assign18120_e12623_d_n6;
        locals.var_t1_dn7 = assign18120_e12623_d_n7;
        locals.var_t1_dn8 = assign18120_e12623_d_n8;
        locals.var_t1_dn9 = assign18120_e12623_d_n9;
        locals.var_t1_dn10 = assign18120_e12623_d_n10;
        locals.var_t1_dn11 = assign18120_e12623_d_n11;
        locals.var_t1_dn14 = assign18120_e12623_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign18130_e12630, assign18130_e12630_d_n0, assign18130_e12630_d_n2, assign18130_e12630_d_n4, assign18130_e12630_d_n5, assign18130_e12630_d_n6, assign18130_e12630_d_n7, assign18130_e12630_d_n8, assign18130_e12630_d_n9, assign18130_e12630_d_n10, assign18130_e12630_d_n11, assign18130_e12630_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign18130_e12627: f64 = (locals.var_t1 / locals.var_nsub);
        let assign18130_e12628: f64 = (assign18130_e12627).sqrt();
        (assign18130_e12628, ((((locals.var_t1_dn0 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18130_e12628)), ((((locals.var_t1_dn2 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18130_e12628)), ((((locals.var_t1_dn4 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn4)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18130_e12628)), ((((locals.var_t1_dn5 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn5)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18130_e12628)), ((((locals.var_t1_dn6 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18130_e12628)), ((((locals.var_t1_dn7 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18130_e12628)), ((((locals.var_t1_dn8 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn8)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18130_e12628)), ((((locals.var_t1_dn9 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn9)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18130_e12628)), ((((locals.var_t1_dn10 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18130_e12628)), ((((locals.var_t1_dn11 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn11)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18130_e12628)), ((((locals.var_t1_dn14 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn14)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign18130_e12628)),)
    } else {
        (locals.var_wdpl, locals.var_wdpl_dn0, locals.var_wdpl_dn2, locals.var_wdpl_dn4, locals.var_wdpl_dn5, locals.var_wdpl_dn6, locals.var_wdpl_dn7, locals.var_wdpl_dn8, locals.var_wdpl_dn9, locals.var_wdpl_dn10, locals.var_wdpl_dn11, locals.var_wdpl_dn14,)
    }
};
        locals.var_wdpl = assign18130_e12630;
        locals.var_wdpl_dn0 = assign18130_e12630_d_n0;
        locals.var_wdpl_dn2 = assign18130_e12630_d_n2;
        locals.var_wdpl_dn4 = assign18130_e12630_d_n4;
        locals.var_wdpl_dn5 = assign18130_e12630_d_n5;
        locals.var_wdpl_dn6 = assign18130_e12630_d_n6;
        locals.var_wdpl_dn7 = assign18130_e12630_d_n7;
        locals.var_wdpl_dn8 = assign18130_e12630_d_n8;
        locals.var_wdpl_dn9 = assign18130_e12630_d_n9;
        locals.var_wdpl_dn10 = assign18130_e12630_d_n10;
        locals.var_wdpl_dn11 = assign18130_e12630_d_n11;
        locals.var_wdpl_dn14 = assign18130_e12630_d_n14;
        locals.var_wdpl_rv = 0.0;

        let (assign18140_e12637, assign18140_e12637_d_n0, assign18140_e12637_d_n2, assign18140_e12637_d_n4, assign18140_e12637_d_n5, assign18140_e12637_d_n6, assign18140_e12637_d_n7, assign18140_e12637_d_n8, assign18140_e12637_d_n9, assign18140_e12637_d_n10, assign18140_e12637_d_n11, assign18140_e12637_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign18140_e12634: f64 = (locals.var_t1 / locals.var_ef_nsubp);
        let assign18140_e12635: f64 = (assign18140_e12634).sqrt();
        (assign18140_e12635, ((((locals.var_t1_dn0 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn0)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18140_e12635)), ((((locals.var_t1_dn2 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn2)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18140_e12635)), ((((locals.var_t1_dn4 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn4)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18140_e12635)), ((((locals.var_t1_dn5 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn5)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18140_e12635)), ((((locals.var_t1_dn6 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn6)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18140_e12635)), ((((locals.var_t1_dn7 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn7)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18140_e12635)), ((((locals.var_t1_dn8 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn8)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18140_e12635)), ((((locals.var_t1_dn9 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn9)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18140_e12635)), ((((locals.var_t1_dn10 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn10)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18140_e12635)), ((((locals.var_t1_dn11 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn11)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18140_e12635)), ((((locals.var_t1_dn14 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn14)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign18140_e12635)),)
    } else {
        (locals.var_wdplp, locals.var_wdplp_dn0, locals.var_wdplp_dn2, locals.var_wdplp_dn4, locals.var_wdplp_dn5, locals.var_wdplp_dn6, locals.var_wdplp_dn7, locals.var_wdplp_dn8, locals.var_wdplp_dn9, locals.var_wdplp_dn10, locals.var_wdplp_dn11, locals.var_wdplp_dn14,)
    }
};
        locals.var_wdplp = assign18140_e12637;
        locals.var_wdplp_dn0 = assign18140_e12637_d_n0;
        locals.var_wdplp_dn2 = assign18140_e12637_d_n2;
        locals.var_wdplp_dn4 = assign18140_e12637_d_n4;
        locals.var_wdplp_dn5 = assign18140_e12637_d_n5;
        locals.var_wdplp_dn6 = assign18140_e12637_d_n6;
        locals.var_wdplp_dn7 = assign18140_e12637_d_n7;
        locals.var_wdplp_dn8 = assign18140_e12637_d_n8;
        locals.var_wdplp_dn9 = assign18140_e12637_d_n9;
        locals.var_wdplp_dn10 = assign18140_e12637_d_n10;
        locals.var_wdplp_dn11 = assign18140_e12637_d_n11;
        locals.var_wdplp_dn14 = assign18140_e12637_d_n14;
        locals.var_wdplp_rv = 0.0;

        let assign18150_e12640: f64 = if locals.var_uc_codep == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard374 = assign18150_e12640;
        locals.var_guard374_rv = 0.0;

        let (assign18160_e12655, assign18160_e12655_d_n0, assign18160_e12655_d_n2, assign18160_e12655_d_n4, assign18160_e12655_d_n5, assign18160_e12655_d_n6, assign18160_e12655_d_n7, assign18160_e12655_d_n8, assign18160_e12655_d_n9, assign18160_e12655_d_n10, assign18160_e12655_d_n11, assign18160_e12655_d_n14,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard374 != 0.0)) {
        let assign18160_e12646: f64 = (2.0 * 1.034943e-10);
        let assign18160_e12648: f64 = (assign18160_e12646 * 1.6021918e-19);
        let assign18160_e12650: f64 = (assign18160_e12648 * locals.var_nsub);
        let assign18160_e12652: f64 = (assign18160_e12650 * locals.var_beta_inv);
        let assign18160_e12653: f64 = (assign18160_e12652).sqrt();
        (assign18160_e12653, ((((assign18160_e12648 * locals.var_nsub_dn0) * locals.var_beta_inv) + (assign18160_e12650 * locals.var_beta_inv_dn0)) / (2.0 * assign18160_e12653)), ((((assign18160_e12648 * locals.var_nsub_dn2) * locals.var_beta_inv) + (assign18160_e12650 * locals.var_beta_inv_dn2)) / (2.0 * assign18160_e12653)), ((((assign18160_e12648 * locals.var_nsub_dn4) * locals.var_beta_inv) + (assign18160_e12650 * locals.var_beta_inv_dn4)) / (2.0 * assign18160_e12653)), ((((assign18160_e12648 * locals.var_nsub_dn5) * locals.var_beta_inv) + (assign18160_e12650 * locals.var_beta_inv_dn5)) / (2.0 * assign18160_e12653)), ((((assign18160_e12648 * locals.var_nsub_dn6) * locals.var_beta_inv) + (assign18160_e12650 * locals.var_beta_inv_dn6)) / (2.0 * assign18160_e12653)), ((((assign18160_e12648 * locals.var_nsub_dn7) * locals.var_beta_inv) + (assign18160_e12650 * locals.var_beta_inv_dn7)) / (2.0 * assign18160_e12653)), ((((assign18160_e12648 * locals.var_nsub_dn8) * locals.var_beta_inv) + (assign18160_e12650 * locals.var_beta_inv_dn8)) / (2.0 * assign18160_e12653)), ((((assign18160_e12648 * locals.var_nsub_dn9) * locals.var_beta_inv) + (assign18160_e12650 * locals.var_beta_inv_dn9)) / (2.0 * assign18160_e12653)), ((((assign18160_e12648 * locals.var_nsub_dn10) * locals.var_beta_inv) + (assign18160_e12650 * locals.var_beta_inv_dn10)) / (2.0 * assign18160_e12653)), ((((assign18160_e12648 * locals.var_nsub_dn11) * locals.var_beta_inv) + (assign18160_e12650 * locals.var_beta_inv_dn11)) / (2.0 * assign18160_e12653)), ((((assign18160_e12648 * locals.var_nsub_dn14) * locals.var_beta_inv) + (assign18160_e12650 * locals.var_beta_inv_dn14)) / (2.0 * assign18160_e12653)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn11, locals.var_cnst0_dn14,)
    }
};
        locals.var_cnst0 = assign18160_e12655;
        locals.var_cnst0_dn0 = assign18160_e12655_d_n0;
        locals.var_cnst0_dn2 = assign18160_e12655_d_n2;
        locals.var_cnst0_dn4 = assign18160_e12655_d_n4;
        locals.var_cnst0_dn5 = assign18160_e12655_d_n5;
        locals.var_cnst0_dn6 = assign18160_e12655_d_n6;
        locals.var_cnst0_dn7 = assign18160_e12655_d_n7;
        locals.var_cnst0_dn8 = assign18160_e12655_d_n8;
        locals.var_cnst0_dn9 = assign18160_e12655_d_n9;
        locals.var_cnst0_dn10 = assign18160_e12655_d_n10;
        locals.var_cnst0_dn11 = assign18160_e12655_d_n11;
        locals.var_cnst0_dn14 = assign18160_e12655_d_n14;
        locals.var_cnst0_rv = 0.0;

        let (assign18170_e12663, assign18170_e12663_d_n0, assign18170_e12663_d_n2, assign18170_e12663_d_n4, assign18170_e12663_d_n5, assign18170_e12663_d_n6, assign18170_e12663_d_n7, assign18170_e12663_d_n8, assign18170_e12663_d_n9, assign18170_e12663_d_n10, assign18170_e12663_d_n11, assign18170_e12663_d_n14,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard374 != 0.0)) {
        let assign18170_e12661: f64 = (locals.var_nin / locals.var_nsub);
        (assign18170_e12661, (((locals.var_nin_dn0 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn2 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn4 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn4)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn5 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn5)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn6 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn7 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn8 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn8)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn9 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn9)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn10 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn11 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn11)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn14 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn14)) / (locals.var_nsub * locals.var_nsub)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign18170_e12663;
        locals.var_t1_dn0 = assign18170_e12663_d_n0;
        locals.var_t1_dn2 = assign18170_e12663_d_n2;
        locals.var_t1_dn4 = assign18170_e12663_d_n4;
        locals.var_t1_dn5 = assign18170_e12663_d_n5;
        locals.var_t1_dn6 = assign18170_e12663_d_n6;
        locals.var_t1_dn7 = assign18170_e12663_d_n7;
        locals.var_t1_dn8 = assign18170_e12663_d_n8;
        locals.var_t1_dn9 = assign18170_e12663_d_n9;
        locals.var_t1_dn10 = assign18170_e12663_d_n10;
        locals.var_t1_dn11 = assign18170_e12663_d_n11;
        locals.var_t1_dn14 = assign18170_e12663_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign18180_e12671, assign18180_e12671_d_n0, assign18180_e12671_d_n2, assign18180_e12671_d_n4, assign18180_e12671_d_n5, assign18180_e12671_d_n6, assign18180_e12671_d_n7, assign18180_e12671_d_n8, assign18180_e12671_d_n9, assign18180_e12671_d_n10, assign18180_e12671_d_n11, assign18180_e12671_d_n14,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard374 != 0.0)) {
        let assign18180_e12669: f64 = (locals.var_t1 * locals.var_t1);
        (assign18180_e12669, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn11, locals.var_cnst1_dn14,)
    }
};
        locals.var_cnst1 = assign18180_e12671;
        locals.var_cnst1_dn0 = assign18180_e12671_d_n0;
        locals.var_cnst1_dn2 = assign18180_e12671_d_n2;
        locals.var_cnst1_dn4 = assign18180_e12671_d_n4;
        locals.var_cnst1_dn5 = assign18180_e12671_d_n5;
        locals.var_cnst1_dn6 = assign18180_e12671_d_n6;
        locals.var_cnst1_dn7 = assign18180_e12671_d_n7;
        locals.var_cnst1_dn8 = assign18180_e12671_d_n8;
        locals.var_cnst1_dn9 = assign18180_e12671_d_n9;
        locals.var_cnst1_dn10 = assign18180_e12671_d_n10;
        locals.var_cnst1_dn11 = assign18180_e12671_d_n11;
        locals.var_cnst1_dn14 = assign18180_e12671_d_n14;
        locals.var_cnst1_rv = 0.0;

        let assign18190_e12674: f64 = if locals.var_uc_codep == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard375 = assign18190_e12674;
        locals.var_guard375_rv = 0.0;

        let assign18200_e12677: f64 = if locals.var_uc_nover != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard376 = assign18200_e12677;
        locals.var_guard376_rv = 0.0;

        let (assign18210_e12690, assign18210_e12690_d_n0, assign18210_e12690_d_n2, assign18210_e12690_d_n4, assign18210_e12690_d_n5, assign18210_e12690_d_n6, assign18210_e12690_d_n7, assign18210_e12690_d_n8, assign18210_e12690_d_n9, assign18210_e12690_d_n10, assign18210_e12690_d_n11, assign18210_e12690_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard375 != 0.0)) && (locals.var_guard376 != 0.0)) {
        let assign18210_e12686: f64 = (locals.var_uc_nover / locals.var_nsub);
        let assign18210_e12687: f64 = (assign18210_e12686).sqrt();
        let assign18210_e12688: f64 = (locals.var_cnst0 * assign18210_e12687);
        (assign18210_e12688, ((locals.var_cnst0_dn0 * assign18210_e12687) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12687)))), ((locals.var_cnst0_dn2 * assign18210_e12687) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12687)))), ((locals.var_cnst0_dn4 * assign18210_e12687) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn4) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12687)))), ((locals.var_cnst0_dn5 * assign18210_e12687) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn5) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12687)))), ((locals.var_cnst0_dn6 * assign18210_e12687) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12687)))), ((locals.var_cnst0_dn7 * assign18210_e12687) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12687)))), ((locals.var_cnst0_dn8 * assign18210_e12687) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn8) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12687)))), ((locals.var_cnst0_dn9 * assign18210_e12687) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn9) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12687)))), ((locals.var_cnst0_dn10 * assign18210_e12687) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12687)))), ((locals.var_cnst0_dn11 * assign18210_e12687) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn11) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12687)))), ((locals.var_cnst0_dn14 * assign18210_e12687) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn14) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18210_e12687)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn14,)
    }
};
        locals.var_cnst0over = assign18210_e12690;
        locals.var_cnst0over_dn0 = assign18210_e12690_d_n0;
        locals.var_cnst0over_dn2 = assign18210_e12690_d_n2;
        locals.var_cnst0over_dn4 = assign18210_e12690_d_n4;
        locals.var_cnst0over_dn5 = assign18210_e12690_d_n5;
        locals.var_cnst0over_dn6 = assign18210_e12690_d_n6;
        locals.var_cnst0over_dn7 = assign18210_e12690_d_n7;
        locals.var_cnst0over_dn8 = assign18210_e12690_d_n8;
        locals.var_cnst0over_dn9 = assign18210_e12690_d_n9;
        locals.var_cnst0over_dn10 = assign18210_e12690_d_n10;
        locals.var_cnst0over_dn11 = assign18210_e12690_d_n11;
        locals.var_cnst0over_dn14 = assign18210_e12690_d_n14;
        locals.var_cnst0over_rv = 0.0;

        let assign18220_e12693: f64 = if locals.var_uc_novers != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard377 = assign18220_e12693;
        locals.var_guard377_rv = 0.0;

        let (assign18230_e12706, assign18230_e12706_d_n0, assign18230_e12706_d_n2, assign18230_e12706_d_n4, assign18230_e12706_d_n5, assign18230_e12706_d_n6, assign18230_e12706_d_n7, assign18230_e12706_d_n8, assign18230_e12706_d_n9, assign18230_e12706_d_n10, assign18230_e12706_d_n11, assign18230_e12706_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard375 != 0.0)) && (locals.var_guard377 != 0.0)) {
        let assign18230_e12702: f64 = (locals.var_uc_novers / locals.var_nsub);
        let assign18230_e12703: f64 = (assign18230_e12702).sqrt();
        let assign18230_e12704: f64 = (locals.var_cnst0 * assign18230_e12703);
        (assign18230_e12704, ((locals.var_cnst0_dn0 * assign18230_e12703) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12703)))), ((locals.var_cnst0_dn2 * assign18230_e12703) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12703)))), ((locals.var_cnst0_dn4 * assign18230_e12703) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn4) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12703)))), ((locals.var_cnst0_dn5 * assign18230_e12703) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn5) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12703)))), ((locals.var_cnst0_dn6 * assign18230_e12703) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12703)))), ((locals.var_cnst0_dn7 * assign18230_e12703) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12703)))), ((locals.var_cnst0_dn8 * assign18230_e12703) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn8) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12703)))), ((locals.var_cnst0_dn9 * assign18230_e12703) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn9) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12703)))), ((locals.var_cnst0_dn10 * assign18230_e12703) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12703)))), ((locals.var_cnst0_dn11 * assign18230_e12703) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn11) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12703)))), ((locals.var_cnst0_dn14 * assign18230_e12703) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn14) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign18230_e12703)))),)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn11, locals.var_cnst0overs_dn14,)
    }
};
        locals.var_cnst0overs = assign18230_e12706;
        locals.var_cnst0overs_dn0 = assign18230_e12706_d_n0;
        locals.var_cnst0overs_dn2 = assign18230_e12706_d_n2;
        locals.var_cnst0overs_dn4 = assign18230_e12706_d_n4;
        locals.var_cnst0overs_dn5 = assign18230_e12706_d_n5;
        locals.var_cnst0overs_dn6 = assign18230_e12706_d_n6;
        locals.var_cnst0overs_dn7 = assign18230_e12706_d_n7;
        locals.var_cnst0overs_dn8 = assign18230_e12706_d_n8;
        locals.var_cnst0overs_dn9 = assign18230_e12706_d_n9;
        locals.var_cnst0overs_dn10 = assign18230_e12706_d_n10;
        locals.var_cnst0overs_dn11 = assign18230_e12706_d_n11;
        locals.var_cnst0overs_dn14 = assign18230_e12706_d_n14;
        locals.var_cnst0overs_rv = 0.0;

        let assign18240_e12709: f64 = if locals.var_uc_nover != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard378 = assign18240_e12709;
        locals.var_guard378_rv = 0.0;

        let (assign18250_e12723, assign18250_e12723_d_n0, assign18250_e12723_d_n2, assign18250_e12723_d_n4, assign18250_e12723_d_n5, assign18250_e12723_d_n6, assign18250_e12723_d_n7, assign18250_e12723_d_n8, assign18250_e12723_d_n9, assign18250_e12723_d_n10, assign18250_e12723_d_n11, assign18250_e12723_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard375 == 0.0)) && (locals.var_guard378 != 0.0)) {
        let assign18250_e12719: f64 = (locals.var_uc_nover / locals.var_uc_ndepm);
        let assign18250_e12720: f64 = (assign18250_e12719).sqrt();
        let assign18250_e12721: f64 = (locals.var_cnst0 * assign18250_e12720);
        (assign18250_e12721, ((locals.var_cnst0_dn0 * assign18250_e12720) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn0) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12720)))), ((locals.var_cnst0_dn2 * assign18250_e12720) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn2) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12720)))), ((locals.var_cnst0_dn4 * assign18250_e12720) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn4) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12720)))), ((locals.var_cnst0_dn5 * assign18250_e12720) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn5) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12720)))), ((locals.var_cnst0_dn6 * assign18250_e12720) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn6) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12720)))), ((locals.var_cnst0_dn7 * assign18250_e12720) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn7) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12720)))), ((locals.var_cnst0_dn8 * assign18250_e12720) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn8) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12720)))), ((locals.var_cnst0_dn9 * assign18250_e12720) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn9) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12720)))), ((locals.var_cnst0_dn10 * assign18250_e12720) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn10) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12720)))), ((locals.var_cnst0_dn11 * assign18250_e12720) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn11) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12720)))), ((locals.var_cnst0_dn14 * assign18250_e12720) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn14) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18250_e12720)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn14,)
    }
};
        locals.var_cnst0over = assign18250_e12723;
        locals.var_cnst0over_dn0 = assign18250_e12723_d_n0;
        locals.var_cnst0over_dn2 = assign18250_e12723_d_n2;
        locals.var_cnst0over_dn4 = assign18250_e12723_d_n4;
        locals.var_cnst0over_dn5 = assign18250_e12723_d_n5;
        locals.var_cnst0over_dn6 = assign18250_e12723_d_n6;
        locals.var_cnst0over_dn7 = assign18250_e12723_d_n7;
        locals.var_cnst0over_dn8 = assign18250_e12723_d_n8;
        locals.var_cnst0over_dn9 = assign18250_e12723_d_n9;
        locals.var_cnst0over_dn10 = assign18250_e12723_d_n10;
        locals.var_cnst0over_dn11 = assign18250_e12723_d_n11;
        locals.var_cnst0over_dn14 = assign18250_e12723_d_n14;
        locals.var_cnst0over_rv = 0.0;

        let assign18260_e12726: f64 = if locals.var_uc_novers != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard379 = assign18260_e12726;
        locals.var_guard379_rv = 0.0;

        let (assign18270_e12740, assign18270_e12740_d_n0, assign18270_e12740_d_n2, assign18270_e12740_d_n4, assign18270_e12740_d_n5, assign18270_e12740_d_n6, assign18270_e12740_d_n7, assign18270_e12740_d_n8, assign18270_e12740_d_n9, assign18270_e12740_d_n10, assign18270_e12740_d_n11, assign18270_e12740_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard375 == 0.0)) && (locals.var_guard379 != 0.0)) {
        let assign18270_e12736: f64 = (locals.var_uc_novers / locals.var_uc_ndepm);
        let assign18270_e12737: f64 = (assign18270_e12736).sqrt();
        let assign18270_e12738: f64 = (locals.var_cnst0 * assign18270_e12737);
        (assign18270_e12738, ((locals.var_cnst0_dn0 * assign18270_e12737) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn0) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12737)))), ((locals.var_cnst0_dn2 * assign18270_e12737) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn2) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12737)))), ((locals.var_cnst0_dn4 * assign18270_e12737) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn4) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12737)))), ((locals.var_cnst0_dn5 * assign18270_e12737) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn5) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12737)))), ((locals.var_cnst0_dn6 * assign18270_e12737) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn6) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12737)))), ((locals.var_cnst0_dn7 * assign18270_e12737) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn7) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12737)))), ((locals.var_cnst0_dn8 * assign18270_e12737) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn8) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12737)))), ((locals.var_cnst0_dn9 * assign18270_e12737) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn9) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12737)))), ((locals.var_cnst0_dn10 * assign18270_e12737) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn10) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12737)))), ((locals.var_cnst0_dn11 * assign18270_e12737) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn11) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12737)))), ((locals.var_cnst0_dn14 * assign18270_e12737) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn14) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign18270_e12737)))),)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn11, locals.var_cnst0overs_dn14,)
    }
};
        locals.var_cnst0overs = assign18270_e12740;
        locals.var_cnst0overs_dn0 = assign18270_e12740_d_n0;
        locals.var_cnst0overs_dn2 = assign18270_e12740_d_n2;
        locals.var_cnst0overs_dn4 = assign18270_e12740_d_n4;
        locals.var_cnst0overs_dn5 = assign18270_e12740_d_n5;
        locals.var_cnst0overs_dn6 = assign18270_e12740_d_n6;
        locals.var_cnst0overs_dn7 = assign18270_e12740_d_n7;
        locals.var_cnst0overs_dn8 = assign18270_e12740_d_n8;
        locals.var_cnst0overs_dn9 = assign18270_e12740_d_n9;
        locals.var_cnst0overs_dn10 = assign18270_e12740_d_n10;
        locals.var_cnst0overs_dn11 = assign18270_e12740_d_n11;
        locals.var_cnst0overs_dn14 = assign18270_e12740_d_n14;
        locals.var_cnst0overs_rv = 0.0;

        let assign18280_e12743: f64 = if locals.var_uc_cordrift == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard380 = assign18280_e12743;
        locals.var_guard380_rv = 0.0;

        let assign18290_e12746: f64 = if locals.var_uc_rd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard381 = assign18290_e12746;
        locals.var_guard381_rv = 0.0;

        let (assign18300_e12770, assign18300_e12770_d_n0, assign18300_e12770_d_n2, assign18300_e12770_d_n4, assign18300_e12770_d_n5, assign18300_e12770_d_n6, assign18300_e12770_d_n7, assign18300_e12770_d_n8, assign18300_e12770_d_n9, assign18300_e12770_d_n10, assign18300_e12770_d_n11, assign18300_e12770_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard381 != 0.0)) {
        let assign18300_e12755: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign18300_e12757: f64 = (assign18300_e12755 * 1000000.0);
        let assign18300_e12759: f64 = (assign18300_e12757 + locals.var_uc_rdict1);
        let assign18300_e12760: f64 = (locals.var_rdtemp0 * assign18300_e12759);
        let assign18300_e12763: f64 = (p.p68 * p.p100);
        let assign18300_e12765: f64 = (assign18300_e12763 * 1000000.0);
        let assign18300_e12767: f64 = (assign18300_e12765 + p.p101);
        let assign18300_e12768: f64 = (assign18300_e12760 * assign18300_e12767);
        (assign18300_e12768, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign18300_e12770;
        locals.var_t2_dn0 = assign18300_e12770_d_n0;
        locals.var_t2_dn2 = assign18300_e12770_d_n2;
        locals.var_t2_dn4 = assign18300_e12770_d_n4;
        locals.var_t2_dn5 = assign18300_e12770_d_n5;
        locals.var_t2_dn6 = assign18300_e12770_d_n6;
        locals.var_t2_dn7 = assign18300_e12770_d_n7;
        locals.var_t2_dn8 = assign18300_e12770_d_n8;
        locals.var_t2_dn9 = assign18300_e12770_d_n9;
        locals.var_t2_dn10 = assign18300_e12770_d_n10;
        locals.var_t2_dn11 = assign18300_e12770_d_n11;
        locals.var_t2_dn14 = assign18300_e12770_d_n14;
        locals.var_t2_rv = 0.0;

        let assign18310_e12773: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard382 = assign18310_e12773;
        locals.var_guard382_rv = 0.0;

        let (assign18320_e12793, assign18320_e12793_d_n0, assign18320_e12793_d_n2, assign18320_e12793_d_n4, assign18320_e12793_d_n5, assign18320_e12793_d_n6, assign18320_e12793_d_n7, assign18320_e12793_d_n8, assign18320_e12793_d_n9, assign18320_e12793_d_n10, assign18320_e12793_d_n11, assign18320_e12793_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 != 0.0)) {
        let assign18320_e12784: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff0);
        let assign18320_e12785: f64 = (locals.var_uc_rd + assign18320_e12784);
        let assign18320_e12788: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff0_2);
        let assign18320_e12789: f64 = (assign18320_e12785 + assign18320_e12788);
        let assign18320_e12791: f64 = (assign18320_e12789 * locals.var_t2);
        (assign18320_e12791, ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign18320_e12789 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign18320_e12789 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign18320_e12789 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign18320_e12789 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign18320_e12789 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign18320_e12789 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign18320_e12789 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign18320_e12789 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign18320_e12789 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn11) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn11)) * locals.var_t2) + (assign18320_e12789 * locals.var_t2_dn11)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn14) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn14)) * locals.var_t2) + (assign18320_e12789 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign18320_e12793;
        locals.var_rde_dn0 = assign18320_e12793_d_n0;
        locals.var_rde_dn2 = assign18320_e12793_d_n2;
        locals.var_rde_dn4 = assign18320_e12793_d_n4;
        locals.var_rde_dn5 = assign18320_e12793_d_n5;
        locals.var_rde_dn6 = assign18320_e12793_d_n6;
        locals.var_rde_dn7 = assign18320_e12793_d_n7;
        locals.var_rde_dn8 = assign18320_e12793_d_n8;
        locals.var_rde_dn9 = assign18320_e12793_d_n9;
        locals.var_rde_dn10 = assign18320_e12793_d_n10;
        locals.var_rde_dn11 = assign18320_e12793_d_n11;
        locals.var_rde_dn14 = assign18320_e12793_d_n14;
        locals.var_rde_rv = 0.0;

        let (assign18330_e12811, assign18330_e12811_d_n0, assign18330_e12811_d_n2, assign18330_e12811_d_n4, assign18330_e12811_d_n5, assign18330_e12811_d_n6, assign18330_e12811_d_n7, assign18330_e12811_d_n8, assign18330_e12811_d_n9, assign18330_e12811_d_n10, assign18330_e12811_d_n11, assign18330_e12811_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 != 0.0)) {
        let assign18330_e12804: f64 = (0.005 * locals.var_uc_rd);
        let assign18330_e12805: f64 = (locals.var_rde - assign18330_e12804);
        let assign18330_e12808: f64 = (0.01 * locals.var_uc_rd);
        let assign18330_e12809: f64 = (assign18330_e12805 - assign18330_e12808);
        (assign18330_e12809, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18330_e12811;
        locals.var_tmf1_dn0 = assign18330_e12811_d_n0;
        locals.var_tmf1_dn2 = assign18330_e12811_d_n2;
        locals.var_tmf1_dn4 = assign18330_e12811_d_n4;
        locals.var_tmf1_dn5 = assign18330_e12811_d_n5;
        locals.var_tmf1_dn6 = assign18330_e12811_d_n6;
        locals.var_tmf1_dn7 = assign18330_e12811_d_n7;
        locals.var_tmf1_dn8 = assign18330_e12811_d_n8;
        locals.var_tmf1_dn9 = assign18330_e12811_d_n9;
        locals.var_tmf1_dn10 = assign18330_e12811_d_n10;
        locals.var_tmf1_dn11 = assign18330_e12811_d_n11;
        locals.var_tmf1_dn14 = assign18330_e12811_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign18340_e12829, assign18340_e12829_d_n0, assign18340_e12829_d_n2, assign18340_e12829_d_n4, assign18340_e12829_d_n5, assign18340_e12829_d_n6, assign18340_e12829_d_n7, assign18340_e12829_d_n8, assign18340_e12829_d_n9, assign18340_e12829_d_n10, assign18340_e12829_d_n11, assign18340_e12829_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 != 0.0)) {
        let assign18340_e12822: f64 = (0.005 * locals.var_uc_rd);
        let assign18340_e12823: f64 = (4.0 * assign18340_e12822);
        let assign18340_e12826: f64 = (0.01 * locals.var_uc_rd);
        let assign18340_e12827: f64 = (assign18340_e12823 * assign18340_e12826);
        (assign18340_e12827, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18340_e12829;
        locals.var_tmf2_dn0 = assign18340_e12829_d_n0;
        locals.var_tmf2_dn2 = assign18340_e12829_d_n2;
        locals.var_tmf2_dn4 = assign18340_e12829_d_n4;
        locals.var_tmf2_dn5 = assign18340_e12829_d_n5;
        locals.var_tmf2_dn6 = assign18340_e12829_d_n6;
        locals.var_tmf2_dn7 = assign18340_e12829_d_n7;
        locals.var_tmf2_dn8 = assign18340_e12829_d_n8;
        locals.var_tmf2_dn9 = assign18340_e12829_d_n9;
        locals.var_tmf2_dn10 = assign18340_e12829_d_n10;
        locals.var_tmf2_dn11 = assign18340_e12829_d_n11;
        locals.var_tmf2_dn14 = assign18340_e12829_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18350_e12845, assign18350_e12845_d_n0, assign18350_e12845_d_n2, assign18350_e12845_d_n4, assign18350_e12845_d_n5, assign18350_e12845_d_n6, assign18350_e12845_d_n7, assign18350_e12845_d_n8, assign18350_e12845_d_n9, assign18350_e12845_d_n10, assign18350_e12845_d_n11, assign18350_e12845_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 != 0.0)) {
        let (assign18350_e12843, assign18350_e12843_d_n0, assign18350_e12843_d_n2, assign18350_e12843_d_n4, assign18350_e12843_d_n5, assign18350_e12843_d_n6, assign18350_e12843_d_n7, assign18350_e12843_d_n8, assign18350_e12843_d_n9, assign18350_e12843_d_n10, assign18350_e12843_d_n11, assign18350_e12843_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18350_e12842: f64 = (-locals.var_tmf2);
                (assign18350_e12842, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18350_e12843, assign18350_e12843_d_n0, assign18350_e12843_d_n2, assign18350_e12843_d_n4, assign18350_e12843_d_n5, assign18350_e12843_d_n6, assign18350_e12843_d_n7, assign18350_e12843_d_n8, assign18350_e12843_d_n9, assign18350_e12843_d_n10, assign18350_e12843_d_n11, assign18350_e12843_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18350_e12845;
        locals.var_tmf2_dn0 = assign18350_e12845_d_n0;
        locals.var_tmf2_dn2 = assign18350_e12845_d_n2;
        locals.var_tmf2_dn4 = assign18350_e12845_d_n4;
        locals.var_tmf2_dn5 = assign18350_e12845_d_n5;
        locals.var_tmf2_dn6 = assign18350_e12845_d_n6;
        locals.var_tmf2_dn7 = assign18350_e12845_d_n7;
        locals.var_tmf2_dn8 = assign18350_e12845_d_n8;
        locals.var_tmf2_dn9 = assign18350_e12845_d_n9;
        locals.var_tmf2_dn10 = assign18350_e12845_d_n10;
        locals.var_tmf2_dn11 = assign18350_e12845_d_n11;
        locals.var_tmf2_dn14 = assign18350_e12845_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18360_e12860, assign18360_e12860_d_n0, assign18360_e12860_d_n2, assign18360_e12860_d_n4, assign18360_e12860_d_n5, assign18360_e12860_d_n6, assign18360_e12860_d_n7, assign18360_e12860_d_n8, assign18360_e12860_d_n9, assign18360_e12860_d_n10, assign18360_e12860_d_n11, assign18360_e12860_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 != 0.0)) {
        let assign18360_e12855: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18360_e12857: f64 = (assign18360_e12855 + locals.var_tmf2);
        let assign18360_e12858: f64 = (assign18360_e12857).sqrt();
        (assign18360_e12858, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18360_e12858)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18360_e12858)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18360_e12858)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18360_e12858)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18360_e12858)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18360_e12858)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18360_e12858)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18360_e12858)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18360_e12858)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18360_e12858)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18360_e12858)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18360_e12860;
        locals.var_tmf2_dn0 = assign18360_e12860_d_n0;
        locals.var_tmf2_dn2 = assign18360_e12860_d_n2;
        locals.var_tmf2_dn4 = assign18360_e12860_d_n4;
        locals.var_tmf2_dn5 = assign18360_e12860_d_n5;
        locals.var_tmf2_dn6 = assign18360_e12860_d_n6;
        locals.var_tmf2_dn7 = assign18360_e12860_d_n7;
        locals.var_tmf2_dn8 = assign18360_e12860_d_n8;
        locals.var_tmf2_dn9 = assign18360_e12860_d_n9;
        locals.var_tmf2_dn10 = assign18360_e12860_d_n10;
        locals.var_tmf2_dn11 = assign18360_e12860_d_n11;
        locals.var_tmf2_dn14 = assign18360_e12860_d_n14;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_44(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18370_e12876, assign18370_e12876_d_n0, assign18370_e12876_d_n2, assign18370_e12876_d_n4, assign18370_e12876_d_n5, assign18370_e12876_d_n6, assign18370_e12876_d_n7, assign18370_e12876_d_n8, assign18370_e12876_d_n9, assign18370_e12876_d_n10, assign18370_e12876_d_n11, assign18370_e12876_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 != 0.0)) {
        let assign18370_e12872: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18370_e12873: f64 = (1.0 + assign18370_e12872);
        let assign18370_e12874: f64 = (0.5 * assign18370_e12873);
        (assign18370_e12874, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18370_e12876;
        locals.var_t0_dn0 = assign18370_e12876_d_n0;
        locals.var_t0_dn2 = assign18370_e12876_d_n2;
        locals.var_t0_dn4 = assign18370_e12876_d_n4;
        locals.var_t0_dn5 = assign18370_e12876_d_n5;
        locals.var_t0_dn6 = assign18370_e12876_d_n6;
        locals.var_t0_dn7 = assign18370_e12876_d_n7;
        locals.var_t0_dn8 = assign18370_e12876_d_n8;
        locals.var_t0_dn9 = assign18370_e12876_d_n9;
        locals.var_t0_dn10 = assign18370_e12876_d_n10;
        locals.var_t0_dn11 = assign18370_e12876_d_n11;
        locals.var_t0_dn14 = assign18370_e12876_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign18380_e12894, assign18380_e12894_d_n0, assign18380_e12894_d_n2, assign18380_e12894_d_n4, assign18380_e12894_d_n5, assign18380_e12894_d_n6, assign18380_e12894_d_n7, assign18380_e12894_d_n8, assign18380_e12894_d_n9, assign18380_e12894_d_n10, assign18380_e12894_d_n11, assign18380_e12894_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 != 0.0)) {
        let assign18380_e12886: f64 = (0.005 * locals.var_uc_rd);
        let assign18380_e12890: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18380_e12891: f64 = (0.5 * assign18380_e12890);
        let assign18380_e12892: f64 = (assign18380_e12886 + assign18380_e12891);
        (assign18380_e12892, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign18380_e12894;
        locals.var_rde_dn0 = assign18380_e12894_d_n0;
        locals.var_rde_dn2 = assign18380_e12894_d_n2;
        locals.var_rde_dn4 = assign18380_e12894_d_n4;
        locals.var_rde_dn5 = assign18380_e12894_d_n5;
        locals.var_rde_dn6 = assign18380_e12894_d_n6;
        locals.var_rde_dn7 = assign18380_e12894_d_n7;
        locals.var_rde_dn8 = assign18380_e12894_d_n8;
        locals.var_rde_dn9 = assign18380_e12894_d_n9;
        locals.var_rde_dn10 = assign18380_e12894_d_n10;
        locals.var_rde_dn11 = assign18380_e12894_d_n11;
        locals.var_rde_dn14 = assign18380_e12894_d_n14;
        locals.var_rde_rv = 0.0;

        let (assign18390_e12915, assign18390_e12915_d_n0, assign18390_e12915_d_n2, assign18390_e12915_d_n4, assign18390_e12915_d_n5, assign18390_e12915_d_n6, assign18390_e12915_d_n7, assign18390_e12915_d_n8, assign18390_e12915_d_n9, assign18390_e12915_d_n10, assign18390_e12915_d_n11, assign18390_e12915_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 == 0.0)) {
        let assign18390_e12906: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff);
        let assign18390_e12907: f64 = (locals.var_uc_rd + assign18390_e12906);
        let assign18390_e12910: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff_2);
        let assign18390_e12911: f64 = (assign18390_e12907 + assign18390_e12910);
        let assign18390_e12913: f64 = (assign18390_e12911 * locals.var_t2);
        (assign18390_e12913, ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign18390_e12911 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign18390_e12911 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign18390_e12911 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign18390_e12911 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign18390_e12911 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign18390_e12911 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign18390_e12911 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign18390_e12911 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign18390_e12911 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn11) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn11)) * locals.var_t2) + (assign18390_e12911 * locals.var_t2_dn11)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn14) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn14)) * locals.var_t2) + (assign18390_e12911 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign18390_e12915;
        locals.var_rde_dn0 = assign18390_e12915_d_n0;
        locals.var_rde_dn2 = assign18390_e12915_d_n2;
        locals.var_rde_dn4 = assign18390_e12915_d_n4;
        locals.var_rde_dn5 = assign18390_e12915_d_n5;
        locals.var_rde_dn6 = assign18390_e12915_d_n6;
        locals.var_rde_dn7 = assign18390_e12915_d_n7;
        locals.var_rde_dn8 = assign18390_e12915_d_n8;
        locals.var_rde_dn9 = assign18390_e12915_d_n9;
        locals.var_rde_dn10 = assign18390_e12915_d_n10;
        locals.var_rde_dn11 = assign18390_e12915_d_n11;
        locals.var_rde_dn14 = assign18390_e12915_d_n14;
        locals.var_rde_rv = 0.0;

        let (assign18400_e12934, assign18400_e12934_d_n0, assign18400_e12934_d_n2, assign18400_e12934_d_n4, assign18400_e12934_d_n5, assign18400_e12934_d_n6, assign18400_e12934_d_n7, assign18400_e12934_d_n8, assign18400_e12934_d_n9, assign18400_e12934_d_n10, assign18400_e12934_d_n11, assign18400_e12934_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 == 0.0)) {
        let assign18400_e12927: f64 = (0.005 * locals.var_uc_rd);
        let assign18400_e12928: f64 = (locals.var_rde - assign18400_e12927);
        let assign18400_e12931: f64 = (0.01 * locals.var_uc_rd);
        let assign18400_e12932: f64 = (assign18400_e12928 - assign18400_e12931);
        (assign18400_e12932, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18400_e12934;
        locals.var_tmf1_dn0 = assign18400_e12934_d_n0;
        locals.var_tmf1_dn2 = assign18400_e12934_d_n2;
        locals.var_tmf1_dn4 = assign18400_e12934_d_n4;
        locals.var_tmf1_dn5 = assign18400_e12934_d_n5;
        locals.var_tmf1_dn6 = assign18400_e12934_d_n6;
        locals.var_tmf1_dn7 = assign18400_e12934_d_n7;
        locals.var_tmf1_dn8 = assign18400_e12934_d_n8;
        locals.var_tmf1_dn9 = assign18400_e12934_d_n9;
        locals.var_tmf1_dn10 = assign18400_e12934_d_n10;
        locals.var_tmf1_dn11 = assign18400_e12934_d_n11;
        locals.var_tmf1_dn14 = assign18400_e12934_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign18410_e12953, assign18410_e12953_d_n0, assign18410_e12953_d_n2, assign18410_e12953_d_n4, assign18410_e12953_d_n5, assign18410_e12953_d_n6, assign18410_e12953_d_n7, assign18410_e12953_d_n8, assign18410_e12953_d_n9, assign18410_e12953_d_n10, assign18410_e12953_d_n11, assign18410_e12953_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 == 0.0)) {
        let assign18410_e12946: f64 = (0.005 * locals.var_uc_rd);
        let assign18410_e12947: f64 = (4.0 * assign18410_e12946);
        let assign18410_e12950: f64 = (0.01 * locals.var_uc_rd);
        let assign18410_e12951: f64 = (assign18410_e12947 * assign18410_e12950);
        (assign18410_e12951, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18410_e12953;
        locals.var_tmf2_dn0 = assign18410_e12953_d_n0;
        locals.var_tmf2_dn2 = assign18410_e12953_d_n2;
        locals.var_tmf2_dn4 = assign18410_e12953_d_n4;
        locals.var_tmf2_dn5 = assign18410_e12953_d_n5;
        locals.var_tmf2_dn6 = assign18410_e12953_d_n6;
        locals.var_tmf2_dn7 = assign18410_e12953_d_n7;
        locals.var_tmf2_dn8 = assign18410_e12953_d_n8;
        locals.var_tmf2_dn9 = assign18410_e12953_d_n9;
        locals.var_tmf2_dn10 = assign18410_e12953_d_n10;
        locals.var_tmf2_dn11 = assign18410_e12953_d_n11;
        locals.var_tmf2_dn14 = assign18410_e12953_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18420_e12970, assign18420_e12970_d_n0, assign18420_e12970_d_n2, assign18420_e12970_d_n4, assign18420_e12970_d_n5, assign18420_e12970_d_n6, assign18420_e12970_d_n7, assign18420_e12970_d_n8, assign18420_e12970_d_n9, assign18420_e12970_d_n10, assign18420_e12970_d_n11, assign18420_e12970_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 == 0.0)) {
        let (assign18420_e12968, assign18420_e12968_d_n0, assign18420_e12968_d_n2, assign18420_e12968_d_n4, assign18420_e12968_d_n5, assign18420_e12968_d_n6, assign18420_e12968_d_n7, assign18420_e12968_d_n8, assign18420_e12968_d_n9, assign18420_e12968_d_n10, assign18420_e12968_d_n11, assign18420_e12968_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18420_e12967: f64 = (-locals.var_tmf2);
                (assign18420_e12967, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18420_e12968, assign18420_e12968_d_n0, assign18420_e12968_d_n2, assign18420_e12968_d_n4, assign18420_e12968_d_n5, assign18420_e12968_d_n6, assign18420_e12968_d_n7, assign18420_e12968_d_n8, assign18420_e12968_d_n9, assign18420_e12968_d_n10, assign18420_e12968_d_n11, assign18420_e12968_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18420_e12970;
        locals.var_tmf2_dn0 = assign18420_e12970_d_n0;
        locals.var_tmf2_dn2 = assign18420_e12970_d_n2;
        locals.var_tmf2_dn4 = assign18420_e12970_d_n4;
        locals.var_tmf2_dn5 = assign18420_e12970_d_n5;
        locals.var_tmf2_dn6 = assign18420_e12970_d_n6;
        locals.var_tmf2_dn7 = assign18420_e12970_d_n7;
        locals.var_tmf2_dn8 = assign18420_e12970_d_n8;
        locals.var_tmf2_dn9 = assign18420_e12970_d_n9;
        locals.var_tmf2_dn10 = assign18420_e12970_d_n10;
        locals.var_tmf2_dn11 = assign18420_e12970_d_n11;
        locals.var_tmf2_dn14 = assign18420_e12970_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18430_e12986, assign18430_e12986_d_n0, assign18430_e12986_d_n2, assign18430_e12986_d_n4, assign18430_e12986_d_n5, assign18430_e12986_d_n6, assign18430_e12986_d_n7, assign18430_e12986_d_n8, assign18430_e12986_d_n9, assign18430_e12986_d_n10, assign18430_e12986_d_n11, assign18430_e12986_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 == 0.0)) {
        let assign18430_e12981: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18430_e12983: f64 = (assign18430_e12981 + locals.var_tmf2);
        let assign18430_e12984: f64 = (assign18430_e12983).sqrt();
        (assign18430_e12984, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18430_e12984)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18430_e12984)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18430_e12984)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18430_e12984)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18430_e12984)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18430_e12984)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18430_e12984)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18430_e12984)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18430_e12984)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18430_e12984)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18430_e12984)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18430_e12986;
        locals.var_tmf2_dn0 = assign18430_e12986_d_n0;
        locals.var_tmf2_dn2 = assign18430_e12986_d_n2;
        locals.var_tmf2_dn4 = assign18430_e12986_d_n4;
        locals.var_tmf2_dn5 = assign18430_e12986_d_n5;
        locals.var_tmf2_dn6 = assign18430_e12986_d_n6;
        locals.var_tmf2_dn7 = assign18430_e12986_d_n7;
        locals.var_tmf2_dn8 = assign18430_e12986_d_n8;
        locals.var_tmf2_dn9 = assign18430_e12986_d_n9;
        locals.var_tmf2_dn10 = assign18430_e12986_d_n10;
        locals.var_tmf2_dn11 = assign18430_e12986_d_n11;
        locals.var_tmf2_dn14 = assign18430_e12986_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18440_e13003, assign18440_e13003_d_n0, assign18440_e13003_d_n2, assign18440_e13003_d_n4, assign18440_e13003_d_n5, assign18440_e13003_d_n6, assign18440_e13003_d_n7, assign18440_e13003_d_n8, assign18440_e13003_d_n9, assign18440_e13003_d_n10, assign18440_e13003_d_n11, assign18440_e13003_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 == 0.0)) {
        let assign18440_e12999: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18440_e13000: f64 = (1.0 + assign18440_e12999);
        let assign18440_e13001: f64 = (0.5 * assign18440_e13000);
        (assign18440_e13001, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18440_e13003;
        locals.var_t0_dn0 = assign18440_e13003_d_n0;
        locals.var_t0_dn2 = assign18440_e13003_d_n2;
        locals.var_t0_dn4 = assign18440_e13003_d_n4;
        locals.var_t0_dn5 = assign18440_e13003_d_n5;
        locals.var_t0_dn6 = assign18440_e13003_d_n6;
        locals.var_t0_dn7 = assign18440_e13003_d_n7;
        locals.var_t0_dn8 = assign18440_e13003_d_n8;
        locals.var_t0_dn9 = assign18440_e13003_d_n9;
        locals.var_t0_dn10 = assign18440_e13003_d_n10;
        locals.var_t0_dn11 = assign18440_e13003_d_n11;
        locals.var_t0_dn14 = assign18440_e13003_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign18450_e13022, assign18450_e13022_d_n0, assign18450_e13022_d_n2, assign18450_e13022_d_n4, assign18450_e13022_d_n5, assign18450_e13022_d_n6, assign18450_e13022_d_n7, assign18450_e13022_d_n8, assign18450_e13022_d_n9, assign18450_e13022_d_n10, assign18450_e13022_d_n11, assign18450_e13022_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard381 != 0.0)) && (locals.var_guard382 == 0.0)) {
        let assign18450_e13014: f64 = (0.005 * locals.var_uc_rd);
        let assign18450_e13018: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18450_e13019: f64 = (0.5 * assign18450_e13018);
        let assign18450_e13020: f64 = (assign18450_e13014 + assign18450_e13019);
        (assign18450_e13020, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign18450_e13022;
        locals.var_rde_dn0 = assign18450_e13022_d_n0;
        locals.var_rde_dn2 = assign18450_e13022_d_n2;
        locals.var_rde_dn4 = assign18450_e13022_d_n4;
        locals.var_rde_dn5 = assign18450_e13022_d_n5;
        locals.var_rde_dn6 = assign18450_e13022_d_n6;
        locals.var_rde_dn7 = assign18450_e13022_d_n7;
        locals.var_rde_dn8 = assign18450_e13022_d_n8;
        locals.var_rde_dn9 = assign18450_e13022_d_n9;
        locals.var_rde_dn10 = assign18450_e13022_d_n10;
        locals.var_rde_dn11 = assign18450_e13022_d_n11;
        locals.var_rde_dn14 = assign18450_e13022_d_n14;
        locals.var_rde_rv = 0.0;

        let (assign18460_e13031, assign18460_e13031_d_n0, assign18460_e13031_d_n2, assign18460_e13031_d_n4, assign18460_e13031_d_n5, assign18460_e13031_d_n6, assign18460_e13031_d_n7, assign18460_e13031_d_n8, assign18460_e13031_d_n9, assign18460_e13031_d_n10, assign18460_e13031_d_n11, assign18460_e13031_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard381 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign18460_e13031;
        locals.var_rde_dn0 = assign18460_e13031_d_n0;
        locals.var_rde_dn2 = assign18460_e13031_d_n2;
        locals.var_rde_dn4 = assign18460_e13031_d_n4;
        locals.var_rde_dn5 = assign18460_e13031_d_n5;
        locals.var_rde_dn6 = assign18460_e13031_d_n6;
        locals.var_rde_dn7 = assign18460_e13031_d_n7;
        locals.var_rde_dn8 = assign18460_e13031_d_n8;
        locals.var_rde_dn9 = assign18460_e13031_d_n9;
        locals.var_rde_dn10 = assign18460_e13031_d_n10;
        locals.var_rde_dn11 = assign18460_e13031_d_n11;
        locals.var_rde_dn14 = assign18460_e13031_d_n14;
        locals.var_rde_rv = 0.0;

        let assign18470_e13034: f64 = if locals.var_uc_rs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard383 = assign18470_e13034;
        locals.var_guard383_rv = 0.0;

        let (assign18480_e13058, assign18480_e13058_d_n0, assign18480_e13058_d_n2, assign18480_e13058_d_n4, assign18480_e13058_d_n5, assign18480_e13058_d_n6, assign18480_e13058_d_n7, assign18480_e13058_d_n8, assign18480_e13058_d_n9, assign18480_e13058_d_n10, assign18480_e13058_d_n11, assign18480_e13058_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard383 != 0.0)) {
        let assign18480_e13043: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign18480_e13045: f64 = (assign18480_e13043 * 1000000.0);
        let assign18480_e13047: f64 = (assign18480_e13045 + locals.var_uc_rdict1);
        let assign18480_e13048: f64 = (locals.var_rdtemp0 * assign18480_e13047);
        let assign18480_e13051: f64 = (p.p70 * p.p100);
        let assign18480_e13053: f64 = (assign18480_e13051 * 1000000.0);
        let assign18480_e13055: f64 = (assign18480_e13053 + p.p101);
        let assign18480_e13056: f64 = (assign18480_e13048 * assign18480_e13055);
        (assign18480_e13056, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign18480_e13058;
        locals.var_t2_dn0 = assign18480_e13058_d_n0;
        locals.var_t2_dn2 = assign18480_e13058_d_n2;
        locals.var_t2_dn4 = assign18480_e13058_d_n4;
        locals.var_t2_dn5 = assign18480_e13058_d_n5;
        locals.var_t2_dn6 = assign18480_e13058_d_n6;
        locals.var_t2_dn7 = assign18480_e13058_d_n7;
        locals.var_t2_dn8 = assign18480_e13058_d_n8;
        locals.var_t2_dn9 = assign18480_e13058_d_n9;
        locals.var_t2_dn10 = assign18480_e13058_d_n10;
        locals.var_t2_dn11 = assign18480_e13058_d_n11;
        locals.var_t2_dn14 = assign18480_e13058_d_n14;
        locals.var_t2_rv = 0.0;

        let assign18490_e13061: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard384 = assign18490_e13061;
        locals.var_guard384_rv = 0.0;

        let (assign18500_e13081, assign18500_e13081_d_n0, assign18500_e13081_d_n2, assign18500_e13081_d_n4, assign18500_e13081_d_n5, assign18500_e13081_d_n6, assign18500_e13081_d_n7, assign18500_e13081_d_n8, assign18500_e13081_d_n9, assign18500_e13081_d_n10, assign18500_e13081_d_n11, assign18500_e13081_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        let assign18500_e13072: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff0);
        let assign18500_e13073: f64 = (locals.var_uc_rs + assign18500_e13072);
        let assign18500_e13076: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff0_2);
        let assign18500_e13077: f64 = (assign18500_e13073 + assign18500_e13076);
        let assign18500_e13079: f64 = (assign18500_e13077 * locals.var_t2);
        (assign18500_e13079, ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign18500_e13077 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign18500_e13077 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign18500_e13077 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign18500_e13077 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign18500_e13077 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign18500_e13077 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign18500_e13077 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign18500_e13077 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign18500_e13077 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn11) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn11)) * locals.var_t2) + (assign18500_e13077 * locals.var_t2_dn11)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn14) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn14)) * locals.var_t2) + (assign18500_e13077 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign18500_e13081;
        locals.var_rse_dn0 = assign18500_e13081_d_n0;
        locals.var_rse_dn2 = assign18500_e13081_d_n2;
        locals.var_rse_dn4 = assign18500_e13081_d_n4;
        locals.var_rse_dn5 = assign18500_e13081_d_n5;
        locals.var_rse_dn6 = assign18500_e13081_d_n6;
        locals.var_rse_dn7 = assign18500_e13081_d_n7;
        locals.var_rse_dn8 = assign18500_e13081_d_n8;
        locals.var_rse_dn9 = assign18500_e13081_d_n9;
        locals.var_rse_dn10 = assign18500_e13081_d_n10;
        locals.var_rse_dn11 = assign18500_e13081_d_n11;
        locals.var_rse_dn14 = assign18500_e13081_d_n14;
        locals.var_rse_rv = 0.0;

        let (assign18510_e13099, assign18510_e13099_d_n0, assign18510_e13099_d_n2, assign18510_e13099_d_n4, assign18510_e13099_d_n5, assign18510_e13099_d_n6, assign18510_e13099_d_n7, assign18510_e13099_d_n8, assign18510_e13099_d_n9, assign18510_e13099_d_n10, assign18510_e13099_d_n11, assign18510_e13099_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        let assign18510_e13092: f64 = (0.005 * locals.var_uc_rs);
        let assign18510_e13093: f64 = (locals.var_rse - assign18510_e13092);
        let assign18510_e13096: f64 = (0.01 * locals.var_uc_rs);
        let assign18510_e13097: f64 = (assign18510_e13093 - assign18510_e13096);
        (assign18510_e13097, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18510_e13099;
        locals.var_tmf1_dn0 = assign18510_e13099_d_n0;
        locals.var_tmf1_dn2 = assign18510_e13099_d_n2;
        locals.var_tmf1_dn4 = assign18510_e13099_d_n4;
        locals.var_tmf1_dn5 = assign18510_e13099_d_n5;
        locals.var_tmf1_dn6 = assign18510_e13099_d_n6;
        locals.var_tmf1_dn7 = assign18510_e13099_d_n7;
        locals.var_tmf1_dn8 = assign18510_e13099_d_n8;
        locals.var_tmf1_dn9 = assign18510_e13099_d_n9;
        locals.var_tmf1_dn10 = assign18510_e13099_d_n10;
        locals.var_tmf1_dn11 = assign18510_e13099_d_n11;
        locals.var_tmf1_dn14 = assign18510_e13099_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign18520_e13117, assign18520_e13117_d_n0, assign18520_e13117_d_n2, assign18520_e13117_d_n4, assign18520_e13117_d_n5, assign18520_e13117_d_n6, assign18520_e13117_d_n7, assign18520_e13117_d_n8, assign18520_e13117_d_n9, assign18520_e13117_d_n10, assign18520_e13117_d_n11, assign18520_e13117_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        let assign18520_e13110: f64 = (0.005 * locals.var_uc_rs);
        let assign18520_e13111: f64 = (4.0 * assign18520_e13110);
        let assign18520_e13114: f64 = (0.01 * locals.var_uc_rs);
        let assign18520_e13115: f64 = (assign18520_e13111 * assign18520_e13114);
        (assign18520_e13115, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18520_e13117;
        locals.var_tmf2_dn0 = assign18520_e13117_d_n0;
        locals.var_tmf2_dn2 = assign18520_e13117_d_n2;
        locals.var_tmf2_dn4 = assign18520_e13117_d_n4;
        locals.var_tmf2_dn5 = assign18520_e13117_d_n5;
        locals.var_tmf2_dn6 = assign18520_e13117_d_n6;
        locals.var_tmf2_dn7 = assign18520_e13117_d_n7;
        locals.var_tmf2_dn8 = assign18520_e13117_d_n8;
        locals.var_tmf2_dn9 = assign18520_e13117_d_n9;
        locals.var_tmf2_dn10 = assign18520_e13117_d_n10;
        locals.var_tmf2_dn11 = assign18520_e13117_d_n11;
        locals.var_tmf2_dn14 = assign18520_e13117_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18530_e13133, assign18530_e13133_d_n0, assign18530_e13133_d_n2, assign18530_e13133_d_n4, assign18530_e13133_d_n5, assign18530_e13133_d_n6, assign18530_e13133_d_n7, assign18530_e13133_d_n8, assign18530_e13133_d_n9, assign18530_e13133_d_n10, assign18530_e13133_d_n11, assign18530_e13133_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        let (assign18530_e13131, assign18530_e13131_d_n0, assign18530_e13131_d_n2, assign18530_e13131_d_n4, assign18530_e13131_d_n5, assign18530_e13131_d_n6, assign18530_e13131_d_n7, assign18530_e13131_d_n8, assign18530_e13131_d_n9, assign18530_e13131_d_n10, assign18530_e13131_d_n11, assign18530_e13131_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18530_e13130: f64 = (-locals.var_tmf2);
                (assign18530_e13130, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18530_e13131, assign18530_e13131_d_n0, assign18530_e13131_d_n2, assign18530_e13131_d_n4, assign18530_e13131_d_n5, assign18530_e13131_d_n6, assign18530_e13131_d_n7, assign18530_e13131_d_n8, assign18530_e13131_d_n9, assign18530_e13131_d_n10, assign18530_e13131_d_n11, assign18530_e13131_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18530_e13133;
        locals.var_tmf2_dn0 = assign18530_e13133_d_n0;
        locals.var_tmf2_dn2 = assign18530_e13133_d_n2;
        locals.var_tmf2_dn4 = assign18530_e13133_d_n4;
        locals.var_tmf2_dn5 = assign18530_e13133_d_n5;
        locals.var_tmf2_dn6 = assign18530_e13133_d_n6;
        locals.var_tmf2_dn7 = assign18530_e13133_d_n7;
        locals.var_tmf2_dn8 = assign18530_e13133_d_n8;
        locals.var_tmf2_dn9 = assign18530_e13133_d_n9;
        locals.var_tmf2_dn10 = assign18530_e13133_d_n10;
        locals.var_tmf2_dn11 = assign18530_e13133_d_n11;
        locals.var_tmf2_dn14 = assign18530_e13133_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18540_e13148, assign18540_e13148_d_n0, assign18540_e13148_d_n2, assign18540_e13148_d_n4, assign18540_e13148_d_n5, assign18540_e13148_d_n6, assign18540_e13148_d_n7, assign18540_e13148_d_n8, assign18540_e13148_d_n9, assign18540_e13148_d_n10, assign18540_e13148_d_n11, assign18540_e13148_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        let assign18540_e13143: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18540_e13145: f64 = (assign18540_e13143 + locals.var_tmf2);
        let assign18540_e13146: f64 = (assign18540_e13145).sqrt();
        (assign18540_e13146, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18540_e13146)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18540_e13146)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18540_e13146)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18540_e13146)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18540_e13146)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18540_e13146)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18540_e13146)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18540_e13146)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18540_e13146)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18540_e13146)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18540_e13146)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18540_e13148;
        locals.var_tmf2_dn0 = assign18540_e13148_d_n0;
        locals.var_tmf2_dn2 = assign18540_e13148_d_n2;
        locals.var_tmf2_dn4 = assign18540_e13148_d_n4;
        locals.var_tmf2_dn5 = assign18540_e13148_d_n5;
        locals.var_tmf2_dn6 = assign18540_e13148_d_n6;
        locals.var_tmf2_dn7 = assign18540_e13148_d_n7;
        locals.var_tmf2_dn8 = assign18540_e13148_d_n8;
        locals.var_tmf2_dn9 = assign18540_e13148_d_n9;
        locals.var_tmf2_dn10 = assign18540_e13148_d_n10;
        locals.var_tmf2_dn11 = assign18540_e13148_d_n11;
        locals.var_tmf2_dn14 = assign18540_e13148_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18550_e13164, assign18550_e13164_d_n0, assign18550_e13164_d_n2, assign18550_e13164_d_n4, assign18550_e13164_d_n5, assign18550_e13164_d_n6, assign18550_e13164_d_n7, assign18550_e13164_d_n8, assign18550_e13164_d_n9, assign18550_e13164_d_n10, assign18550_e13164_d_n11, assign18550_e13164_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        let assign18550_e13160: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18550_e13161: f64 = (1.0 + assign18550_e13160);
        let assign18550_e13162: f64 = (0.5 * assign18550_e13161);
        (assign18550_e13162, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18550_e13164;
        locals.var_t0_dn0 = assign18550_e13164_d_n0;
        locals.var_t0_dn2 = assign18550_e13164_d_n2;
        locals.var_t0_dn4 = assign18550_e13164_d_n4;
        locals.var_t0_dn5 = assign18550_e13164_d_n5;
        locals.var_t0_dn6 = assign18550_e13164_d_n6;
        locals.var_t0_dn7 = assign18550_e13164_d_n7;
        locals.var_t0_dn8 = assign18550_e13164_d_n8;
        locals.var_t0_dn9 = assign18550_e13164_d_n9;
        locals.var_t0_dn10 = assign18550_e13164_d_n10;
        locals.var_t0_dn11 = assign18550_e13164_d_n11;
        locals.var_t0_dn14 = assign18550_e13164_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign18560_e13182, assign18560_e13182_d_n0, assign18560_e13182_d_n2, assign18560_e13182_d_n4, assign18560_e13182_d_n5, assign18560_e13182_d_n6, assign18560_e13182_d_n7, assign18560_e13182_d_n8, assign18560_e13182_d_n9, assign18560_e13182_d_n10, assign18560_e13182_d_n11, assign18560_e13182_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 != 0.0)) {
        let assign18560_e13174: f64 = (0.005 * locals.var_uc_rs);
        let assign18560_e13178: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18560_e13179: f64 = (0.5 * assign18560_e13178);
        let assign18560_e13180: f64 = (assign18560_e13174 + assign18560_e13179);
        (assign18560_e13180, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign18560_e13182;
        locals.var_rse_dn0 = assign18560_e13182_d_n0;
        locals.var_rse_dn2 = assign18560_e13182_d_n2;
        locals.var_rse_dn4 = assign18560_e13182_d_n4;
        locals.var_rse_dn5 = assign18560_e13182_d_n5;
        locals.var_rse_dn6 = assign18560_e13182_d_n6;
        locals.var_rse_dn7 = assign18560_e13182_d_n7;
        locals.var_rse_dn8 = assign18560_e13182_d_n8;
        locals.var_rse_dn9 = assign18560_e13182_d_n9;
        locals.var_rse_dn10 = assign18560_e13182_d_n10;
        locals.var_rse_dn11 = assign18560_e13182_d_n11;
        locals.var_rse_dn14 = assign18560_e13182_d_n14;
        locals.var_rse_rv = 0.0;

        let (assign18570_e13203, assign18570_e13203_d_n0, assign18570_e13203_d_n2, assign18570_e13203_d_n4, assign18570_e13203_d_n5, assign18570_e13203_d_n6, assign18570_e13203_d_n7, assign18570_e13203_d_n8, assign18570_e13203_d_n9, assign18570_e13203_d_n10, assign18570_e13203_d_n11, assign18570_e13203_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 == 0.0)) {
        let assign18570_e13194: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff);
        let assign18570_e13195: f64 = (locals.var_uc_rs + assign18570_e13194);
        let assign18570_e13198: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff_2);
        let assign18570_e13199: f64 = (assign18570_e13195 + assign18570_e13198);
        let assign18570_e13201: f64 = (assign18570_e13199 * locals.var_t2);
        (assign18570_e13201, ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign18570_e13199 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign18570_e13199 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign18570_e13199 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign18570_e13199 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign18570_e13199 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign18570_e13199 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign18570_e13199 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign18570_e13199 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign18570_e13199 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn11) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn11)) * locals.var_t2) + (assign18570_e13199 * locals.var_t2_dn11)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn14) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn14)) * locals.var_t2) + (assign18570_e13199 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign18570_e13203;
        locals.var_rse_dn0 = assign18570_e13203_d_n0;
        locals.var_rse_dn2 = assign18570_e13203_d_n2;
        locals.var_rse_dn4 = assign18570_e13203_d_n4;
        locals.var_rse_dn5 = assign18570_e13203_d_n5;
        locals.var_rse_dn6 = assign18570_e13203_d_n6;
        locals.var_rse_dn7 = assign18570_e13203_d_n7;
        locals.var_rse_dn8 = assign18570_e13203_d_n8;
        locals.var_rse_dn9 = assign18570_e13203_d_n9;
        locals.var_rse_dn10 = assign18570_e13203_d_n10;
        locals.var_rse_dn11 = assign18570_e13203_d_n11;
        locals.var_rse_dn14 = assign18570_e13203_d_n14;
        locals.var_rse_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_45(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18580_e13222, assign18580_e13222_d_n0, assign18580_e13222_d_n2, assign18580_e13222_d_n4, assign18580_e13222_d_n5, assign18580_e13222_d_n6, assign18580_e13222_d_n7, assign18580_e13222_d_n8, assign18580_e13222_d_n9, assign18580_e13222_d_n10, assign18580_e13222_d_n11, assign18580_e13222_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 == 0.0)) {
        let assign18580_e13215: f64 = (0.005 * locals.var_uc_rs);
        let assign18580_e13216: f64 = (locals.var_rse - assign18580_e13215);
        let assign18580_e13219: f64 = (0.01 * locals.var_uc_rs);
        let assign18580_e13220: f64 = (assign18580_e13216 - assign18580_e13219);
        (assign18580_e13220, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18580_e13222;
        locals.var_tmf1_dn0 = assign18580_e13222_d_n0;
        locals.var_tmf1_dn2 = assign18580_e13222_d_n2;
        locals.var_tmf1_dn4 = assign18580_e13222_d_n4;
        locals.var_tmf1_dn5 = assign18580_e13222_d_n5;
        locals.var_tmf1_dn6 = assign18580_e13222_d_n6;
        locals.var_tmf1_dn7 = assign18580_e13222_d_n7;
        locals.var_tmf1_dn8 = assign18580_e13222_d_n8;
        locals.var_tmf1_dn9 = assign18580_e13222_d_n9;
        locals.var_tmf1_dn10 = assign18580_e13222_d_n10;
        locals.var_tmf1_dn11 = assign18580_e13222_d_n11;
        locals.var_tmf1_dn14 = assign18580_e13222_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign18590_e13241, assign18590_e13241_d_n0, assign18590_e13241_d_n2, assign18590_e13241_d_n4, assign18590_e13241_d_n5, assign18590_e13241_d_n6, assign18590_e13241_d_n7, assign18590_e13241_d_n8, assign18590_e13241_d_n9, assign18590_e13241_d_n10, assign18590_e13241_d_n11, assign18590_e13241_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 == 0.0)) {
        let assign18590_e13234: f64 = (0.005 * locals.var_uc_rs);
        let assign18590_e13235: f64 = (4.0 * assign18590_e13234);
        let assign18590_e13238: f64 = (0.01 * locals.var_uc_rs);
        let assign18590_e13239: f64 = (assign18590_e13235 * assign18590_e13238);
        (assign18590_e13239, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18590_e13241;
        locals.var_tmf2_dn0 = assign18590_e13241_d_n0;
        locals.var_tmf2_dn2 = assign18590_e13241_d_n2;
        locals.var_tmf2_dn4 = assign18590_e13241_d_n4;
        locals.var_tmf2_dn5 = assign18590_e13241_d_n5;
        locals.var_tmf2_dn6 = assign18590_e13241_d_n6;
        locals.var_tmf2_dn7 = assign18590_e13241_d_n7;
        locals.var_tmf2_dn8 = assign18590_e13241_d_n8;
        locals.var_tmf2_dn9 = assign18590_e13241_d_n9;
        locals.var_tmf2_dn10 = assign18590_e13241_d_n10;
        locals.var_tmf2_dn11 = assign18590_e13241_d_n11;
        locals.var_tmf2_dn14 = assign18590_e13241_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18600_e13258, assign18600_e13258_d_n0, assign18600_e13258_d_n2, assign18600_e13258_d_n4, assign18600_e13258_d_n5, assign18600_e13258_d_n6, assign18600_e13258_d_n7, assign18600_e13258_d_n8, assign18600_e13258_d_n9, assign18600_e13258_d_n10, assign18600_e13258_d_n11, assign18600_e13258_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 == 0.0)) {
        let (assign18600_e13256, assign18600_e13256_d_n0, assign18600_e13256_d_n2, assign18600_e13256_d_n4, assign18600_e13256_d_n5, assign18600_e13256_d_n6, assign18600_e13256_d_n7, assign18600_e13256_d_n8, assign18600_e13256_d_n9, assign18600_e13256_d_n10, assign18600_e13256_d_n11, assign18600_e13256_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18600_e13255: f64 = (-locals.var_tmf2);
                (assign18600_e13255, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18600_e13256, assign18600_e13256_d_n0, assign18600_e13256_d_n2, assign18600_e13256_d_n4, assign18600_e13256_d_n5, assign18600_e13256_d_n6, assign18600_e13256_d_n7, assign18600_e13256_d_n8, assign18600_e13256_d_n9, assign18600_e13256_d_n10, assign18600_e13256_d_n11, assign18600_e13256_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18600_e13258;
        locals.var_tmf2_dn0 = assign18600_e13258_d_n0;
        locals.var_tmf2_dn2 = assign18600_e13258_d_n2;
        locals.var_tmf2_dn4 = assign18600_e13258_d_n4;
        locals.var_tmf2_dn5 = assign18600_e13258_d_n5;
        locals.var_tmf2_dn6 = assign18600_e13258_d_n6;
        locals.var_tmf2_dn7 = assign18600_e13258_d_n7;
        locals.var_tmf2_dn8 = assign18600_e13258_d_n8;
        locals.var_tmf2_dn9 = assign18600_e13258_d_n9;
        locals.var_tmf2_dn10 = assign18600_e13258_d_n10;
        locals.var_tmf2_dn11 = assign18600_e13258_d_n11;
        locals.var_tmf2_dn14 = assign18600_e13258_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18610_e13274, assign18610_e13274_d_n0, assign18610_e13274_d_n2, assign18610_e13274_d_n4, assign18610_e13274_d_n5, assign18610_e13274_d_n6, assign18610_e13274_d_n7, assign18610_e13274_d_n8, assign18610_e13274_d_n9, assign18610_e13274_d_n10, assign18610_e13274_d_n11, assign18610_e13274_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 == 0.0)) {
        let assign18610_e13269: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18610_e13271: f64 = (assign18610_e13269 + locals.var_tmf2);
        let assign18610_e13272: f64 = (assign18610_e13271).sqrt();
        (assign18610_e13272, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18610_e13272)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18610_e13272)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18610_e13272)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18610_e13272)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18610_e13272)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18610_e13272)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18610_e13272)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18610_e13272)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18610_e13272)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18610_e13272)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18610_e13272)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18610_e13274;
        locals.var_tmf2_dn0 = assign18610_e13274_d_n0;
        locals.var_tmf2_dn2 = assign18610_e13274_d_n2;
        locals.var_tmf2_dn4 = assign18610_e13274_d_n4;
        locals.var_tmf2_dn5 = assign18610_e13274_d_n5;
        locals.var_tmf2_dn6 = assign18610_e13274_d_n6;
        locals.var_tmf2_dn7 = assign18610_e13274_d_n7;
        locals.var_tmf2_dn8 = assign18610_e13274_d_n8;
        locals.var_tmf2_dn9 = assign18610_e13274_d_n9;
        locals.var_tmf2_dn10 = assign18610_e13274_d_n10;
        locals.var_tmf2_dn11 = assign18610_e13274_d_n11;
        locals.var_tmf2_dn14 = assign18610_e13274_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18620_e13291, assign18620_e13291_d_n0, assign18620_e13291_d_n2, assign18620_e13291_d_n4, assign18620_e13291_d_n5, assign18620_e13291_d_n6, assign18620_e13291_d_n7, assign18620_e13291_d_n8, assign18620_e13291_d_n9, assign18620_e13291_d_n10, assign18620_e13291_d_n11, assign18620_e13291_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 == 0.0)) {
        let assign18620_e13287: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18620_e13288: f64 = (1.0 + assign18620_e13287);
        let assign18620_e13289: f64 = (0.5 * assign18620_e13288);
        (assign18620_e13289, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18620_e13291;
        locals.var_t0_dn0 = assign18620_e13291_d_n0;
        locals.var_t0_dn2 = assign18620_e13291_d_n2;
        locals.var_t0_dn4 = assign18620_e13291_d_n4;
        locals.var_t0_dn5 = assign18620_e13291_d_n5;
        locals.var_t0_dn6 = assign18620_e13291_d_n6;
        locals.var_t0_dn7 = assign18620_e13291_d_n7;
        locals.var_t0_dn8 = assign18620_e13291_d_n8;
        locals.var_t0_dn9 = assign18620_e13291_d_n9;
        locals.var_t0_dn10 = assign18620_e13291_d_n10;
        locals.var_t0_dn11 = assign18620_e13291_d_n11;
        locals.var_t0_dn14 = assign18620_e13291_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign18630_e13310, assign18630_e13310_d_n0, assign18630_e13310_d_n2, assign18630_e13310_d_n4, assign18630_e13310_d_n5, assign18630_e13310_d_n6, assign18630_e13310_d_n7, assign18630_e13310_d_n8, assign18630_e13310_d_n9, assign18630_e13310_d_n10, assign18630_e13310_d_n11, assign18630_e13310_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard383 != 0.0)) && (locals.var_guard384 == 0.0)) {
        let assign18630_e13302: f64 = (0.005 * locals.var_uc_rs);
        let assign18630_e13306: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18630_e13307: f64 = (0.5 * assign18630_e13306);
        let assign18630_e13308: f64 = (assign18630_e13302 + assign18630_e13307);
        (assign18630_e13308, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign18630_e13310;
        locals.var_rse_dn0 = assign18630_e13310_d_n0;
        locals.var_rse_dn2 = assign18630_e13310_d_n2;
        locals.var_rse_dn4 = assign18630_e13310_d_n4;
        locals.var_rse_dn5 = assign18630_e13310_d_n5;
        locals.var_rse_dn6 = assign18630_e13310_d_n6;
        locals.var_rse_dn7 = assign18630_e13310_d_n7;
        locals.var_rse_dn8 = assign18630_e13310_d_n8;
        locals.var_rse_dn9 = assign18630_e13310_d_n9;
        locals.var_rse_dn10 = assign18630_e13310_d_n10;
        locals.var_rse_dn11 = assign18630_e13310_d_n11;
        locals.var_rse_dn14 = assign18630_e13310_d_n14;
        locals.var_rse_rv = 0.0;

        let (assign18640_e13319, assign18640_e13319_d_n0, assign18640_e13319_d_n2, assign18640_e13319_d_n4, assign18640_e13319_d_n5, assign18640_e13319_d_n6, assign18640_e13319_d_n7, assign18640_e13319_d_n8, assign18640_e13319_d_n9, assign18640_e13319_d_n10, assign18640_e13319_d_n11, assign18640_e13319_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard383 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign18640_e13319;
        locals.var_rse_dn0 = assign18640_e13319_d_n0;
        locals.var_rse_dn2 = assign18640_e13319_d_n2;
        locals.var_rse_dn4 = assign18640_e13319_d_n4;
        locals.var_rse_dn5 = assign18640_e13319_d_n5;
        locals.var_rse_dn6 = assign18640_e13319_d_n6;
        locals.var_rse_dn7 = assign18640_e13319_d_n7;
        locals.var_rse_dn8 = assign18640_e13319_d_n8;
        locals.var_rse_dn9 = assign18640_e13319_d_n9;
        locals.var_rse_dn10 = assign18640_e13319_d_n10;
        locals.var_rse_dn11 = assign18640_e13319_d_n11;
        locals.var_rse_dn14 = assign18640_e13319_d_n14;
        locals.var_rse_rv = 0.0;

        let assign18650_e13322: f64 = if locals.var_uc_rdvd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard385 = assign18650_e13322;
        locals.var_guard385_rv = 0.0;

        let (assign18660_e13346, assign18660_e13346_d_n0, assign18660_e13346_d_n2, assign18660_e13346_d_n4, assign18660_e13346_d_n5, assign18660_e13346_d_n6, assign18660_e13346_d_n7, assign18660_e13346_d_n8, assign18660_e13346_d_n9, assign18660_e13346_d_n10, assign18660_e13346_d_n11, assign18660_e13346_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18660_e13331: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign18660_e13333: f64 = (assign18660_e13331 * 1000000.0);
        let assign18660_e13335: f64 = (assign18660_e13333 + locals.var_uc_rdict1);
        let assign18660_e13336: f64 = (locals.var_rdvdtemp0 * assign18660_e13335);
        let assign18660_e13339: f64 = (p.p68 * p.p100);
        let assign18660_e13341: f64 = (assign18660_e13339 * 1000000.0);
        let assign18660_e13343: f64 = (assign18660_e13341 + p.p101);
        let assign18660_e13344: f64 = (assign18660_e13336 * assign18660_e13343);
        (assign18660_e13344, ((locals.var_rdvdtemp0_dn0 * assign18660_e13335) * assign18660_e13343), ((locals.var_rdvdtemp0_dn2 * assign18660_e13335) * assign18660_e13343), ((locals.var_rdvdtemp0_dn4 * assign18660_e13335) * assign18660_e13343), ((locals.var_rdvdtemp0_dn5 * assign18660_e13335) * assign18660_e13343), ((locals.var_rdvdtemp0_dn6 * assign18660_e13335) * assign18660_e13343), ((locals.var_rdvdtemp0_dn7 * assign18660_e13335) * assign18660_e13343), ((locals.var_rdvdtemp0_dn8 * assign18660_e13335) * assign18660_e13343), ((locals.var_rdvdtemp0_dn9 * assign18660_e13335) * assign18660_e13343), ((locals.var_rdvdtemp0_dn10 * assign18660_e13335) * assign18660_e13343), ((locals.var_rdvdtemp0_dn11 * assign18660_e13335) * assign18660_e13343), ((locals.var_rdvdtemp0_dn14 * assign18660_e13335) * assign18660_e13343),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign18660_e13346;
        locals.var_t4_dn0 = assign18660_e13346_d_n0;
        locals.var_t4_dn2 = assign18660_e13346_d_n2;
        locals.var_t4_dn4 = assign18660_e13346_d_n4;
        locals.var_t4_dn5 = assign18660_e13346_d_n5;
        locals.var_t4_dn6 = assign18660_e13346_d_n6;
        locals.var_t4_dn7 = assign18660_e13346_d_n7;
        locals.var_t4_dn8 = assign18660_e13346_d_n8;
        locals.var_t4_dn9 = assign18660_e13346_d_n9;
        locals.var_t4_dn10 = assign18660_e13346_d_n10;
        locals.var_t4_dn11 = assign18660_e13346_d_n11;
        locals.var_t4_dn14 = assign18660_e13346_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign18670_e13360, assign18670_e13360_d_n0, assign18670_e13360_d_n2, assign18670_e13360_d_n4, assign18670_e13360_d_n5, assign18670_e13360_d_n6, assign18670_e13360_d_n7, assign18670_e13360_d_n8, assign18670_e13360_d_n9, assign18670_e13360_d_n10, assign18670_e13360_d_n11, assign18670_e13360_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18670_e13354: f64 = (1.0 - locals.var_uc_rdov13);
        let assign18670_e13356: f64 = (assign18670_e13354 * p.p63);
        let assign18670_e13358: f64 = (assign18670_e13356 * 1000000.0);
        (assign18670_e13358, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign18670_e13360;
        locals.var_t1_dn0 = assign18670_e13360_d_n0;
        locals.var_t1_dn2 = assign18670_e13360_d_n2;
        locals.var_t1_dn4 = assign18670_e13360_d_n4;
        locals.var_t1_dn5 = assign18670_e13360_d_n5;
        locals.var_t1_dn6 = assign18670_e13360_d_n6;
        locals.var_t1_dn7 = assign18670_e13360_d_n7;
        locals.var_t1_dn8 = assign18670_e13360_d_n8;
        locals.var_t1_dn9 = assign18670_e13360_d_n9;
        locals.var_t1_dn10 = assign18670_e13360_d_n10;
        locals.var_t1_dn11 = assign18670_e13360_d_n11;
        locals.var_t1_dn14 = assign18670_e13360_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign18680_e13381, assign18680_e13381_d_n0, assign18680_e13381_d_n2, assign18680_e13381_d_n4, assign18680_e13381_d_n5, assign18680_e13381_d_n6, assign18680_e13381_d_n7, assign18680_e13381_d_n8, assign18680_e13381_d_n9, assign18680_e13381_d_n10, assign18680_e13381_d_n11, assign18680_e13381_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18680_e13368: f64 = (p.p99 * p.p99);
        let assign18680_e13372: f64 = (0.0001 * 0.01);
        let assign18680_e13373: f64 = (4.0 * assign18680_e13372);
        let assign18680_e13376: f64 = (0.0001 * 0.01);
        let assign18680_e13377: f64 = (assign18680_e13373 * assign18680_e13376);
        let assign18680_e13378: f64 = (assign18680_e13368 + assign18680_e13377);
        let assign18680_e13379: f64 = (assign18680_e13378).sqrt();
        (assign18680_e13379, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18680_e13381;
        locals.var_tmf2_dn0 = assign18680_e13381_d_n0;
        locals.var_tmf2_dn2 = assign18680_e13381_d_n2;
        locals.var_tmf2_dn4 = assign18680_e13381_d_n4;
        locals.var_tmf2_dn5 = assign18680_e13381_d_n5;
        locals.var_tmf2_dn6 = assign18680_e13381_d_n6;
        locals.var_tmf2_dn7 = assign18680_e13381_d_n7;
        locals.var_tmf2_dn8 = assign18680_e13381_d_n8;
        locals.var_tmf2_dn9 = assign18680_e13381_d_n9;
        locals.var_tmf2_dn10 = assign18680_e13381_d_n10;
        locals.var_tmf2_dn11 = assign18680_e13381_d_n11;
        locals.var_tmf2_dn14 = assign18680_e13381_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18690_e13395, assign18690_e13395_d_n0, assign18690_e13395_d_n2, assign18690_e13395_d_n4, assign18690_e13395_d_n5, assign18690_e13395_d_n6, assign18690_e13395_d_n7, assign18690_e13395_d_n8, assign18690_e13395_d_n9, assign18690_e13395_d_n10, assign18690_e13395_d_n11, assign18690_e13395_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18690_e13391: f64 = (p.p99 / locals.var_tmf2);
        let assign18690_e13392: f64 = (1.0 + assign18690_e13391);
        let assign18690_e13393: f64 = (0.5 * assign18690_e13392);
        (assign18690_e13393, (0.5 * (-((p.p99 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18690_e13395;
        locals.var_t0_dn0 = assign18690_e13395_d_n0;
        locals.var_t0_dn2 = assign18690_e13395_d_n2;
        locals.var_t0_dn4 = assign18690_e13395_d_n4;
        locals.var_t0_dn5 = assign18690_e13395_d_n5;
        locals.var_t0_dn6 = assign18690_e13395_d_n6;
        locals.var_t0_dn7 = assign18690_e13395_d_n7;
        locals.var_t0_dn8 = assign18690_e13395_d_n8;
        locals.var_t0_dn9 = assign18690_e13395_d_n9;
        locals.var_t0_dn10 = assign18690_e13395_d_n10;
        locals.var_t0_dn11 = assign18690_e13395_d_n11;
        locals.var_t0_dn14 = assign18690_e13395_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign18700_e13407, assign18700_e13407_d_n0, assign18700_e13407_d_n2, assign18700_e13407_d_n4, assign18700_e13407_d_n5, assign18700_e13407_d_n6, assign18700_e13407_d_n7, assign18700_e13407_d_n8, assign18700_e13407_d_n9, assign18700_e13407_d_n10, assign18700_e13407_d_n11, assign18700_e13407_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18700_e13404: f64 = (p.p99 + locals.var_tmf2);
        let assign18700_e13405: f64 = (0.5 * assign18700_e13404);
        (assign18700_e13405, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * locals.var_tmf2_dn6), (0.5 * locals.var_tmf2_dn7), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign18700_e13407;
        locals.var_t2_dn0 = assign18700_e13407_d_n0;
        locals.var_t2_dn2 = assign18700_e13407_d_n2;
        locals.var_t2_dn4 = assign18700_e13407_d_n4;
        locals.var_t2_dn5 = assign18700_e13407_d_n5;
        locals.var_t2_dn6 = assign18700_e13407_d_n6;
        locals.var_t2_dn7 = assign18700_e13407_d_n7;
        locals.var_t2_dn8 = assign18700_e13407_d_n8;
        locals.var_t2_dn9 = assign18700_e13407_d_n9;
        locals.var_t2_dn10 = assign18700_e13407_d_n10;
        locals.var_t2_dn11 = assign18700_e13407_d_n11;
        locals.var_t2_dn14 = assign18700_e13407_d_n14;
        locals.var_t2_rv = 0.0;

        let assign18710_e13410: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard386 = assign18710_e13410;
        locals.var_guard386_rv = 0.0;

        let (assign18720_e13420, assign18720_e13420_d_n0, assign18720_e13420_d_n2, assign18720_e13420_d_n4, assign18720_e13420_d_n5, assign18720_e13420_d_n6, assign18720_e13420_d_n7, assign18720_e13420_d_n8, assign18720_e13420_d_n9, assign18720_e13420_d_n10, assign18720_e13420_d_n11, assign18720_e13420_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign18720_e13420;
        locals.var_t2_dn0 = assign18720_e13420_d_n0;
        locals.var_t2_dn2 = assign18720_e13420_d_n2;
        locals.var_t2_dn4 = assign18720_e13420_d_n4;
        locals.var_t2_dn5 = assign18720_e13420_d_n5;
        locals.var_t2_dn6 = assign18720_e13420_d_n6;
        locals.var_t2_dn7 = assign18720_e13420_d_n7;
        locals.var_t2_dn8 = assign18720_e13420_d_n8;
        locals.var_t2_dn9 = assign18720_e13420_d_n9;
        locals.var_t2_dn10 = assign18720_e13420_d_n10;
        locals.var_t2_dn11 = assign18720_e13420_d_n11;
        locals.var_t2_dn14 = assign18720_e13420_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign18730_e13430, assign18730_e13430_d_n0, assign18730_e13430_d_n2, assign18730_e13430_d_n4, assign18730_e13430_d_n5, assign18730_e13430_d_n6, assign18730_e13430_d_n7, assign18730_e13430_d_n8, assign18730_e13430_d_n9, assign18730_e13430_d_n10, assign18730_e13430_d_n11, assign18730_e13430_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard386 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign18730_e13430;
        locals.var_t0_dn0 = assign18730_e13430_d_n0;
        locals.var_t0_dn2 = assign18730_e13430_d_n2;
        locals.var_t0_dn4 = assign18730_e13430_d_n4;
        locals.var_t0_dn5 = assign18730_e13430_d_n5;
        locals.var_t0_dn6 = assign18730_e13430_d_n6;
        locals.var_t0_dn7 = assign18730_e13430_d_n7;
        locals.var_t0_dn8 = assign18730_e13430_d_n8;
        locals.var_t0_dn9 = assign18730_e13430_d_n9;
        locals.var_t0_dn10 = assign18730_e13430_d_n10;
        locals.var_t0_dn11 = assign18730_e13430_d_n11;
        locals.var_t0_dn14 = assign18730_e13430_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign18740_e13441, assign18740_e13441_d_n0, assign18740_e13441_d_n2, assign18740_e13441_d_n4, assign18740_e13441_d_n5, assign18740_e13441_d_n6, assign18740_e13441_d_n7, assign18740_e13441_d_n8, assign18740_e13441_d_n9, assign18740_e13441_d_n10, assign18740_e13441_d_n11, assign18740_e13441_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18740_e13437: f64 = (-p.p98);
        let assign18740_e13439: f64 = (assign18740_e13437 / locals.var_t2);
        (assign18740_e13439, (-((assign18740_e13437 * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))), (-((assign18740_e13437 * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))), (-((assign18740_e13437 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))), (-((assign18740_e13437 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))), (-((assign18740_e13437 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))), (-((assign18740_e13437 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))), (-((assign18740_e13437 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))), (-((assign18740_e13437 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))), (-((assign18740_e13437 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))), (-((assign18740_e13437 * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))), (-((assign18740_e13437 * locals.var_t2_dn14) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign18740_e13441;
        locals.var_t8_dn0 = assign18740_e13441_d_n0;
        locals.var_t8_dn2 = assign18740_e13441_d_n2;
        locals.var_t8_dn4 = assign18740_e13441_d_n4;
        locals.var_t8_dn5 = assign18740_e13441_d_n5;
        locals.var_t8_dn6 = assign18740_e13441_d_n6;
        locals.var_t8_dn7 = assign18740_e13441_d_n7;
        locals.var_t8_dn8 = assign18740_e13441_d_n8;
        locals.var_t8_dn9 = assign18740_e13441_d_n9;
        locals.var_t8_dn10 = assign18740_e13441_d_n10;
        locals.var_t8_dn11 = assign18740_e13441_d_n11;
        locals.var_t8_dn14 = assign18740_e13441_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign18750_e13457, assign18750_e13457_d_n0, assign18750_e13457_d_n2, assign18750_e13457_d_n4, assign18750_e13457_d_n5, assign18750_e13457_d_n6, assign18750_e13457_d_n7, assign18750_e13457_d_n8, assign18750_e13457_d_n9, assign18750_e13457_d_n10, assign18750_e13457_d_n11, assign18750_e13457_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18750_e13449: f64 = (locals.var_t8 * p.p63);
        let assign18750_e13451: f64 = (assign18750_e13449 * 1000000.0);
        let assign18750_e13453: f64 = (assign18750_e13451 + 1.0);
        let assign18750_e13455: f64 = (assign18750_e13453 + p.p98);
        (assign18750_e13455, ((locals.var_t8_dn0 * p.p63) * 1000000.0), ((locals.var_t8_dn2 * p.p63) * 1000000.0), ((locals.var_t8_dn4 * p.p63) * 1000000.0), ((locals.var_t8_dn5 * p.p63) * 1000000.0), ((locals.var_t8_dn6 * p.p63) * 1000000.0), ((locals.var_t8_dn7 * p.p63) * 1000000.0), ((locals.var_t8_dn8 * p.p63) * 1000000.0), ((locals.var_t8_dn9 * p.p63) * 1000000.0), ((locals.var_t8_dn10 * p.p63) * 1000000.0), ((locals.var_t8_dn11 * p.p63) * 1000000.0), ((locals.var_t8_dn14 * p.p63) * 1000000.0),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign18750_e13457;
        locals.var_t3_dn0 = assign18750_e13457_d_n0;
        locals.var_t3_dn2 = assign18750_e13457_d_n2;
        locals.var_t3_dn4 = assign18750_e13457_d_n4;
        locals.var_t3_dn5 = assign18750_e13457_d_n5;
        locals.var_t3_dn6 = assign18750_e13457_d_n6;
        locals.var_t3_dn7 = assign18750_e13457_d_n7;
        locals.var_t3_dn8 = assign18750_e13457_d_n8;
        locals.var_t3_dn9 = assign18750_e13457_d_n9;
        locals.var_t3_dn10 = assign18750_e13457_d_n10;
        locals.var_t3_dn11 = assign18750_e13457_d_n11;
        locals.var_t3_dn14 = assign18750_e13457_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign18760_e13471, assign18760_e13471_d_n0, assign18760_e13471_d_n2, assign18760_e13471_d_n4, assign18760_e13471_d_n5, assign18760_e13471_d_n6, assign18760_e13471_d_n7, assign18760_e13471_d_n8, assign18760_e13471_d_n9, assign18760_e13471_d_n10, assign18760_e13471_d_n11, assign18760_e13471_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18760_e13465: f64 = (locals.var_t3 * locals.var_t4);
        let assign18760_e13467: f64 = (assign18760_e13465 - locals.var_t4);
        let assign18760_e13469: f64 = (assign18760_e13467 - 0.01);
        (assign18760_e13469, (((locals.var_t3_dn0 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn0)) - locals.var_t4_dn0), (((locals.var_t3_dn2 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn2)) - locals.var_t4_dn2), (((locals.var_t3_dn4 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn4)) - locals.var_t4_dn4), (((locals.var_t3_dn5 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn5)) - locals.var_t4_dn5), (((locals.var_t3_dn6 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn6)) - locals.var_t4_dn6), (((locals.var_t3_dn7 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn7)) - locals.var_t4_dn7), (((locals.var_t3_dn8 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn8)) - locals.var_t4_dn8), (((locals.var_t3_dn9 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn9)) - locals.var_t4_dn9), (((locals.var_t3_dn10 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn10)) - locals.var_t4_dn10), (((locals.var_t3_dn11 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn11)) - locals.var_t4_dn11), (((locals.var_t3_dn14 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn14)) - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18760_e13471;
        locals.var_tmf1_dn0 = assign18760_e13471_d_n0;
        locals.var_tmf1_dn2 = assign18760_e13471_d_n2;
        locals.var_tmf1_dn4 = assign18760_e13471_d_n4;
        locals.var_tmf1_dn5 = assign18760_e13471_d_n5;
        locals.var_tmf1_dn6 = assign18760_e13471_d_n6;
        locals.var_tmf1_dn7 = assign18760_e13471_d_n7;
        locals.var_tmf1_dn8 = assign18760_e13471_d_n8;
        locals.var_tmf1_dn9 = assign18760_e13471_d_n9;
        locals.var_tmf1_dn10 = assign18760_e13471_d_n10;
        locals.var_tmf1_dn11 = assign18760_e13471_d_n11;
        locals.var_tmf1_dn14 = assign18760_e13471_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign18770_e13483, assign18770_e13483_d_n0, assign18770_e13483_d_n2, assign18770_e13483_d_n4, assign18770_e13483_d_n5, assign18770_e13483_d_n6, assign18770_e13483_d_n7, assign18770_e13483_d_n8, assign18770_e13483_d_n9, assign18770_e13483_d_n10, assign18770_e13483_d_n11, assign18770_e13483_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18770_e13479: f64 = (4.0 * locals.var_t4);
        let assign18770_e13481: f64 = (assign18770_e13479 * 0.01);
        (assign18770_e13481, ((4.0 * locals.var_t4_dn0) * 0.01), ((4.0 * locals.var_t4_dn2) * 0.01), ((4.0 * locals.var_t4_dn4) * 0.01), ((4.0 * locals.var_t4_dn5) * 0.01), ((4.0 * locals.var_t4_dn6) * 0.01), ((4.0 * locals.var_t4_dn7) * 0.01), ((4.0 * locals.var_t4_dn8) * 0.01), ((4.0 * locals.var_t4_dn9) * 0.01), ((4.0 * locals.var_t4_dn10) * 0.01), ((4.0 * locals.var_t4_dn11) * 0.01), ((4.0 * locals.var_t4_dn14) * 0.01),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18770_e13483;
        locals.var_tmf2_dn0 = assign18770_e13483_d_n0;
        locals.var_tmf2_dn2 = assign18770_e13483_d_n2;
        locals.var_tmf2_dn4 = assign18770_e13483_d_n4;
        locals.var_tmf2_dn5 = assign18770_e13483_d_n5;
        locals.var_tmf2_dn6 = assign18770_e13483_d_n6;
        locals.var_tmf2_dn7 = assign18770_e13483_d_n7;
        locals.var_tmf2_dn8 = assign18770_e13483_d_n8;
        locals.var_tmf2_dn9 = assign18770_e13483_d_n9;
        locals.var_tmf2_dn10 = assign18770_e13483_d_n10;
        locals.var_tmf2_dn11 = assign18770_e13483_d_n11;
        locals.var_tmf2_dn14 = assign18770_e13483_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18780_e13497, assign18780_e13497_d_n0, assign18780_e13497_d_n2, assign18780_e13497_d_n4, assign18780_e13497_d_n5, assign18780_e13497_d_n6, assign18780_e13497_d_n7, assign18780_e13497_d_n8, assign18780_e13497_d_n9, assign18780_e13497_d_n10, assign18780_e13497_d_n11, assign18780_e13497_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let (assign18780_e13495, assign18780_e13495_d_n0, assign18780_e13495_d_n2, assign18780_e13495_d_n4, assign18780_e13495_d_n5, assign18780_e13495_d_n6, assign18780_e13495_d_n7, assign18780_e13495_d_n8, assign18780_e13495_d_n9, assign18780_e13495_d_n10, assign18780_e13495_d_n11, assign18780_e13495_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18780_e13494: f64 = (-locals.var_tmf2);
                (assign18780_e13494, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18780_e13495, assign18780_e13495_d_n0, assign18780_e13495_d_n2, assign18780_e13495_d_n4, assign18780_e13495_d_n5, assign18780_e13495_d_n6, assign18780_e13495_d_n7, assign18780_e13495_d_n8, assign18780_e13495_d_n9, assign18780_e13495_d_n10, assign18780_e13495_d_n11, assign18780_e13495_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18780_e13497;
        locals.var_tmf2_dn0 = assign18780_e13497_d_n0;
        locals.var_tmf2_dn2 = assign18780_e13497_d_n2;
        locals.var_tmf2_dn4 = assign18780_e13497_d_n4;
        locals.var_tmf2_dn5 = assign18780_e13497_d_n5;
        locals.var_tmf2_dn6 = assign18780_e13497_d_n6;
        locals.var_tmf2_dn7 = assign18780_e13497_d_n7;
        locals.var_tmf2_dn8 = assign18780_e13497_d_n8;
        locals.var_tmf2_dn9 = assign18780_e13497_d_n9;
        locals.var_tmf2_dn10 = assign18780_e13497_d_n10;
        locals.var_tmf2_dn11 = assign18780_e13497_d_n11;
        locals.var_tmf2_dn14 = assign18780_e13497_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18790_e13510, assign18790_e13510_d_n0, assign18790_e13510_d_n2, assign18790_e13510_d_n4, assign18790_e13510_d_n5, assign18790_e13510_d_n6, assign18790_e13510_d_n7, assign18790_e13510_d_n8, assign18790_e13510_d_n9, assign18790_e13510_d_n10, assign18790_e13510_d_n11, assign18790_e13510_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18790_e13505: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18790_e13507: f64 = (assign18790_e13505 + locals.var_tmf2);
        let assign18790_e13508: f64 = (assign18790_e13507).sqrt();
        (assign18790_e13508, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18790_e13508)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18790_e13508)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18790_e13508)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18790_e13508)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18790_e13508)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18790_e13508)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18790_e13508)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18790_e13508)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18790_e13508)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18790_e13508)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18790_e13508)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18790_e13510;
        locals.var_tmf2_dn0 = assign18790_e13510_d_n0;
        locals.var_tmf2_dn2 = assign18790_e13510_d_n2;
        locals.var_tmf2_dn4 = assign18790_e13510_d_n4;
        locals.var_tmf2_dn5 = assign18790_e13510_d_n5;
        locals.var_tmf2_dn6 = assign18790_e13510_d_n6;
        locals.var_tmf2_dn7 = assign18790_e13510_d_n7;
        locals.var_tmf2_dn8 = assign18790_e13510_d_n8;
        locals.var_tmf2_dn9 = assign18790_e13510_d_n9;
        locals.var_tmf2_dn10 = assign18790_e13510_d_n10;
        locals.var_tmf2_dn11 = assign18790_e13510_d_n11;
        locals.var_tmf2_dn14 = assign18790_e13510_d_n14;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_46(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign18800_e13524, assign18800_e13524_d_n0, assign18800_e13524_d_n2, assign18800_e13524_d_n4, assign18800_e13524_d_n5, assign18800_e13524_d_n6, assign18800_e13524_d_n7, assign18800_e13524_d_n8, assign18800_e13524_d_n9, assign18800_e13524_d_n10, assign18800_e13524_d_n11, assign18800_e13524_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18800_e13520: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18800_e13521: f64 = (1.0 + assign18800_e13520);
        let assign18800_e13522: f64 = (0.5 * assign18800_e13521);
        (assign18800_e13522, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign18800_e13524;
        locals.var_t6_dn0 = assign18800_e13524_d_n0;
        locals.var_t6_dn2 = assign18800_e13524_d_n2;
        locals.var_t6_dn4 = assign18800_e13524_d_n4;
        locals.var_t6_dn5 = assign18800_e13524_d_n5;
        locals.var_t6_dn6 = assign18800_e13524_d_n6;
        locals.var_t6_dn7 = assign18800_e13524_d_n7;
        locals.var_t6_dn8 = assign18800_e13524_d_n8;
        locals.var_t6_dn9 = assign18800_e13524_d_n9;
        locals.var_t6_dn10 = assign18800_e13524_d_n10;
        locals.var_t6_dn11 = assign18800_e13524_d_n11;
        locals.var_t6_dn14 = assign18800_e13524_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign18810_e13538, assign18810_e13538_d_n0, assign18810_e13538_d_n2, assign18810_e13538_d_n4, assign18810_e13538_d_n5, assign18810_e13538_d_n6, assign18810_e13538_d_n7, assign18810_e13538_d_n8, assign18810_e13538_d_n9, assign18810_e13538_d_n10, assign18810_e13538_d_n11, assign18810_e13538_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18810_e13534: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18810_e13535: f64 = (0.5 * assign18810_e13534);
        let assign18810_e13536: f64 = (locals.var_t4 + assign18810_e13535);
        (assign18810_e13536, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t4_dn14 + (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign18810_e13538;
        locals.var_t5_dn0 = assign18810_e13538_d_n0;
        locals.var_t5_dn2 = assign18810_e13538_d_n2;
        locals.var_t5_dn4 = assign18810_e13538_d_n4;
        locals.var_t5_dn5 = assign18810_e13538_d_n5;
        locals.var_t5_dn6 = assign18810_e13538_d_n6;
        locals.var_t5_dn7 = assign18810_e13538_d_n7;
        locals.var_t5_dn8 = assign18810_e13538_d_n8;
        locals.var_t5_dn9 = assign18810_e13538_d_n9;
        locals.var_t5_dn10 = assign18810_e13538_d_n10;
        locals.var_t5_dn11 = assign18810_e13538_d_n11;
        locals.var_t5_dn14 = assign18810_e13538_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign18820_e13554, assign18820_e13554_d_n0, assign18820_e13554_d_n2, assign18820_e13554_d_n4, assign18820_e13554_d_n5, assign18820_e13554_d_n6, assign18820_e13554_d_n7, assign18820_e13554_d_n8, assign18820_e13554_d_n9, assign18820_e13554_d_n10, assign18820_e13554_d_n11, assign18820_e13554_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18820_e13547: f64 = (p.p98 + 1.0);
        let assign18820_e13548: f64 = (locals.var_t4 * assign18820_e13547);
        let assign18820_e13550: f64 = (assign18820_e13548 - locals.var_t5);
        let assign18820_e13552: f64 = (assign18820_e13550 - 5e-5);
        (assign18820_e13552, ((locals.var_t4_dn0 * assign18820_e13547) - locals.var_t5_dn0), ((locals.var_t4_dn2 * assign18820_e13547) - locals.var_t5_dn2), ((locals.var_t4_dn4 * assign18820_e13547) - locals.var_t5_dn4), ((locals.var_t4_dn5 * assign18820_e13547) - locals.var_t5_dn5), ((locals.var_t4_dn6 * assign18820_e13547) - locals.var_t5_dn6), ((locals.var_t4_dn7 * assign18820_e13547) - locals.var_t5_dn7), ((locals.var_t4_dn8 * assign18820_e13547) - locals.var_t5_dn8), ((locals.var_t4_dn9 * assign18820_e13547) - locals.var_t5_dn9), ((locals.var_t4_dn10 * assign18820_e13547) - locals.var_t5_dn10), ((locals.var_t4_dn11 * assign18820_e13547) - locals.var_t5_dn11), ((locals.var_t4_dn14 * assign18820_e13547) - locals.var_t5_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18820_e13554;
        locals.var_tmf1_dn0 = assign18820_e13554_d_n0;
        locals.var_tmf1_dn2 = assign18820_e13554_d_n2;
        locals.var_tmf1_dn4 = assign18820_e13554_d_n4;
        locals.var_tmf1_dn5 = assign18820_e13554_d_n5;
        locals.var_tmf1_dn6 = assign18820_e13554_d_n6;
        locals.var_tmf1_dn7 = assign18820_e13554_d_n7;
        locals.var_tmf1_dn8 = assign18820_e13554_d_n8;
        locals.var_tmf1_dn9 = assign18820_e13554_d_n9;
        locals.var_tmf1_dn10 = assign18820_e13554_d_n10;
        locals.var_tmf1_dn11 = assign18820_e13554_d_n11;
        locals.var_tmf1_dn14 = assign18820_e13554_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign18830_e13570, assign18830_e13570_d_n0, assign18830_e13570_d_n2, assign18830_e13570_d_n4, assign18830_e13570_d_n5, assign18830_e13570_d_n6, assign18830_e13570_d_n7, assign18830_e13570_d_n8, assign18830_e13570_d_n9, assign18830_e13570_d_n10, assign18830_e13570_d_n11, assign18830_e13570_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18830_e13564: f64 = (p.p98 + 1.0);
        let assign18830_e13565: f64 = (locals.var_t4 * assign18830_e13564);
        let assign18830_e13566: f64 = (4.0 * assign18830_e13565);
        let assign18830_e13568: f64 = (assign18830_e13566 * 5e-5);
        (assign18830_e13568, ((4.0 * (locals.var_t4_dn0 * assign18830_e13564)) * 5e-5), ((4.0 * (locals.var_t4_dn2 * assign18830_e13564)) * 5e-5), ((4.0 * (locals.var_t4_dn4 * assign18830_e13564)) * 5e-5), ((4.0 * (locals.var_t4_dn5 * assign18830_e13564)) * 5e-5), ((4.0 * (locals.var_t4_dn6 * assign18830_e13564)) * 5e-5), ((4.0 * (locals.var_t4_dn7 * assign18830_e13564)) * 5e-5), ((4.0 * (locals.var_t4_dn8 * assign18830_e13564)) * 5e-5), ((4.0 * (locals.var_t4_dn9 * assign18830_e13564)) * 5e-5), ((4.0 * (locals.var_t4_dn10 * assign18830_e13564)) * 5e-5), ((4.0 * (locals.var_t4_dn11 * assign18830_e13564)) * 5e-5), ((4.0 * (locals.var_t4_dn14 * assign18830_e13564)) * 5e-5),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18830_e13570;
        locals.var_tmf2_dn0 = assign18830_e13570_d_n0;
        locals.var_tmf2_dn2 = assign18830_e13570_d_n2;
        locals.var_tmf2_dn4 = assign18830_e13570_d_n4;
        locals.var_tmf2_dn5 = assign18830_e13570_d_n5;
        locals.var_tmf2_dn6 = assign18830_e13570_d_n6;
        locals.var_tmf2_dn7 = assign18830_e13570_d_n7;
        locals.var_tmf2_dn8 = assign18830_e13570_d_n8;
        locals.var_tmf2_dn9 = assign18830_e13570_d_n9;
        locals.var_tmf2_dn10 = assign18830_e13570_d_n10;
        locals.var_tmf2_dn11 = assign18830_e13570_d_n11;
        locals.var_tmf2_dn14 = assign18830_e13570_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18840_e13584, assign18840_e13584_d_n0, assign18840_e13584_d_n2, assign18840_e13584_d_n4, assign18840_e13584_d_n5, assign18840_e13584_d_n6, assign18840_e13584_d_n7, assign18840_e13584_d_n8, assign18840_e13584_d_n9, assign18840_e13584_d_n10, assign18840_e13584_d_n11, assign18840_e13584_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let (assign18840_e13582, assign18840_e13582_d_n0, assign18840_e13582_d_n2, assign18840_e13582_d_n4, assign18840_e13582_d_n5, assign18840_e13582_d_n6, assign18840_e13582_d_n7, assign18840_e13582_d_n8, assign18840_e13582_d_n9, assign18840_e13582_d_n10, assign18840_e13582_d_n11, assign18840_e13582_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18840_e13581: f64 = (-locals.var_tmf2);
                (assign18840_e13581, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18840_e13582, assign18840_e13582_d_n0, assign18840_e13582_d_n2, assign18840_e13582_d_n4, assign18840_e13582_d_n5, assign18840_e13582_d_n6, assign18840_e13582_d_n7, assign18840_e13582_d_n8, assign18840_e13582_d_n9, assign18840_e13582_d_n10, assign18840_e13582_d_n11, assign18840_e13582_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18840_e13584;
        locals.var_tmf2_dn0 = assign18840_e13584_d_n0;
        locals.var_tmf2_dn2 = assign18840_e13584_d_n2;
        locals.var_tmf2_dn4 = assign18840_e13584_d_n4;
        locals.var_tmf2_dn5 = assign18840_e13584_d_n5;
        locals.var_tmf2_dn6 = assign18840_e13584_d_n6;
        locals.var_tmf2_dn7 = assign18840_e13584_d_n7;
        locals.var_tmf2_dn8 = assign18840_e13584_d_n8;
        locals.var_tmf2_dn9 = assign18840_e13584_d_n9;
        locals.var_tmf2_dn10 = assign18840_e13584_d_n10;
        locals.var_tmf2_dn11 = assign18840_e13584_d_n11;
        locals.var_tmf2_dn14 = assign18840_e13584_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18850_e13597, assign18850_e13597_d_n0, assign18850_e13597_d_n2, assign18850_e13597_d_n4, assign18850_e13597_d_n5, assign18850_e13597_d_n6, assign18850_e13597_d_n7, assign18850_e13597_d_n8, assign18850_e13597_d_n9, assign18850_e13597_d_n10, assign18850_e13597_d_n11, assign18850_e13597_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18850_e13592: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18850_e13594: f64 = (assign18850_e13592 + locals.var_tmf2);
        let assign18850_e13595: f64 = (assign18850_e13594).sqrt();
        (assign18850_e13595, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18850_e13595)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18850_e13595)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18850_e13595)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18850_e13595)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18850_e13595)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18850_e13595)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18850_e13595)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18850_e13595)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18850_e13595)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18850_e13595)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18850_e13595)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18850_e13597;
        locals.var_tmf2_dn0 = assign18850_e13597_d_n0;
        locals.var_tmf2_dn2 = assign18850_e13597_d_n2;
        locals.var_tmf2_dn4 = assign18850_e13597_d_n4;
        locals.var_tmf2_dn5 = assign18850_e13597_d_n5;
        locals.var_tmf2_dn6 = assign18850_e13597_d_n6;
        locals.var_tmf2_dn7 = assign18850_e13597_d_n7;
        locals.var_tmf2_dn8 = assign18850_e13597_d_n8;
        locals.var_tmf2_dn9 = assign18850_e13597_d_n9;
        locals.var_tmf2_dn10 = assign18850_e13597_d_n10;
        locals.var_tmf2_dn11 = assign18850_e13597_d_n11;
        locals.var_tmf2_dn14 = assign18850_e13597_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18860_e13611, assign18860_e13611_d_n0, assign18860_e13611_d_n2, assign18860_e13611_d_n4, assign18860_e13611_d_n5, assign18860_e13611_d_n6, assign18860_e13611_d_n7, assign18860_e13611_d_n8, assign18860_e13611_d_n9, assign18860_e13611_d_n10, assign18860_e13611_d_n11, assign18860_e13611_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18860_e13607: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18860_e13608: f64 = (1.0 + assign18860_e13607);
        let assign18860_e13609: f64 = (0.5 * assign18860_e13608);
        (assign18860_e13609, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign18860_e13611;
        locals.var_t6_dn0 = assign18860_e13611_d_n0;
        locals.var_t6_dn2 = assign18860_e13611_d_n2;
        locals.var_t6_dn4 = assign18860_e13611_d_n4;
        locals.var_t6_dn5 = assign18860_e13611_d_n5;
        locals.var_t6_dn6 = assign18860_e13611_d_n6;
        locals.var_t6_dn7 = assign18860_e13611_d_n7;
        locals.var_t6_dn8 = assign18860_e13611_d_n8;
        locals.var_t6_dn9 = assign18860_e13611_d_n9;
        locals.var_t6_dn10 = assign18860_e13611_d_n10;
        locals.var_t6_dn11 = assign18860_e13611_d_n11;
        locals.var_t6_dn14 = assign18860_e13611_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign18870_e13629, assign18870_e13629_d_n0, assign18870_e13629_d_n2, assign18870_e13629_d_n4, assign18870_e13629_d_n5, assign18870_e13629_d_n6, assign18870_e13629_d_n7, assign18870_e13629_d_n8, assign18870_e13629_d_n9, assign18870_e13629_d_n10, assign18870_e13629_d_n11, assign18870_e13629_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18870_e13620: f64 = (p.p98 + 1.0);
        let assign18870_e13621: f64 = (locals.var_t4 * assign18870_e13620);
        let assign18870_e13625: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18870_e13626: f64 = (0.5 * assign18870_e13625);
        let assign18870_e13627: f64 = (assign18870_e13621 - assign18870_e13626);
        (assign18870_e13627, ((locals.var_t4_dn0 * assign18870_e13620) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((locals.var_t4_dn2 * assign18870_e13620) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((locals.var_t4_dn4 * assign18870_e13620) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((locals.var_t4_dn5 * assign18870_e13620) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((locals.var_t4_dn6 * assign18870_e13620) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((locals.var_t4_dn7 * assign18870_e13620) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((locals.var_t4_dn8 * assign18870_e13620) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((locals.var_t4_dn9 * assign18870_e13620) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((locals.var_t4_dn10 * assign18870_e13620) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((locals.var_t4_dn11 * assign18870_e13620) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((locals.var_t4_dn14 * assign18870_e13620) - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign18870_e13629;
        locals.var_t7_dn0 = assign18870_e13629_d_n0;
        locals.var_t7_dn2 = assign18870_e13629_d_n2;
        locals.var_t7_dn4 = assign18870_e13629_d_n4;
        locals.var_t7_dn5 = assign18870_e13629_d_n5;
        locals.var_t7_dn6 = assign18870_e13629_d_n6;
        locals.var_t7_dn7 = assign18870_e13629_d_n7;
        locals.var_t7_dn8 = assign18870_e13629_d_n8;
        locals.var_t7_dn9 = assign18870_e13629_d_n9;
        locals.var_t7_dn10 = assign18870_e13629_d_n10;
        locals.var_t7_dn11 = assign18870_e13629_d_n11;
        locals.var_t7_dn14 = assign18870_e13629_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign18880_e13645, assign18880_e13645_d_n0, assign18880_e13645_d_n2, assign18880_e13645_d_n4, assign18880_e13645_d_n5, assign18880_e13645_d_n6, assign18880_e13645_d_n7, assign18880_e13645_d_n8, assign18880_e13645_d_n9, assign18880_e13645_d_n10, assign18880_e13645_d_n11, assign18880_e13645_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18880_e13638: f64 = (locals.var_t1 * locals.var_t4);
        let assign18880_e13639: f64 = (locals.var_t7 + assign18880_e13638);
        let assign18880_e13641: f64 = assign18880_e13639;
        let assign18880_e13643: f64 = (assign18880_e13641 - 5e-5);
        (assign18880_e13643, (locals.var_t7_dn0 + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))), (locals.var_t7_dn2 + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))), (locals.var_t7_dn4 + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))), (locals.var_t7_dn5 + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))), (locals.var_t7_dn6 + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))), (locals.var_t7_dn7 + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))), (locals.var_t7_dn8 + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))), (locals.var_t7_dn9 + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))), (locals.var_t7_dn10 + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))), (locals.var_t7_dn11 + ((locals.var_t1_dn11 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn11))), (locals.var_t7_dn14 + ((locals.var_t1_dn14 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn14))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18880_e13645;
        locals.var_tmf1_dn0 = assign18880_e13645_d_n0;
        locals.var_tmf1_dn2 = assign18880_e13645_d_n2;
        locals.var_tmf1_dn4 = assign18880_e13645_d_n4;
        locals.var_tmf1_dn5 = assign18880_e13645_d_n5;
        locals.var_tmf1_dn6 = assign18880_e13645_d_n6;
        locals.var_tmf1_dn7 = assign18880_e13645_d_n7;
        locals.var_tmf1_dn8 = assign18880_e13645_d_n8;
        locals.var_tmf1_dn9 = assign18880_e13645_d_n9;
        locals.var_tmf1_dn10 = assign18880_e13645_d_n10;
        locals.var_tmf1_dn11 = assign18880_e13645_d_n11;
        locals.var_tmf1_dn14 = assign18880_e13645_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign18890_e13657, assign18890_e13657_d_n0, assign18890_e13657_d_n2, assign18890_e13657_d_n4, assign18890_e13657_d_n5, assign18890_e13657_d_n6, assign18890_e13657_d_n7, assign18890_e13657_d_n8, assign18890_e13657_d_n9, assign18890_e13657_d_n10, assign18890_e13657_d_n11, assign18890_e13657_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18890_e13657;
        locals.var_tmf2_dn0 = assign18890_e13657_d_n0;
        locals.var_tmf2_dn2 = assign18890_e13657_d_n2;
        locals.var_tmf2_dn4 = assign18890_e13657_d_n4;
        locals.var_tmf2_dn5 = assign18890_e13657_d_n5;
        locals.var_tmf2_dn6 = assign18890_e13657_d_n6;
        locals.var_tmf2_dn7 = assign18890_e13657_d_n7;
        locals.var_tmf2_dn8 = assign18890_e13657_d_n8;
        locals.var_tmf2_dn9 = assign18890_e13657_d_n9;
        locals.var_tmf2_dn10 = assign18890_e13657_d_n10;
        locals.var_tmf2_dn11 = assign18890_e13657_d_n11;
        locals.var_tmf2_dn14 = assign18890_e13657_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18900_e13671, assign18900_e13671_d_n0, assign18900_e13671_d_n2, assign18900_e13671_d_n4, assign18900_e13671_d_n5, assign18900_e13671_d_n6, assign18900_e13671_d_n7, assign18900_e13671_d_n8, assign18900_e13671_d_n9, assign18900_e13671_d_n10, assign18900_e13671_d_n11, assign18900_e13671_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let (assign18900_e13669, assign18900_e13669_d_n0, assign18900_e13669_d_n2, assign18900_e13669_d_n4, assign18900_e13669_d_n5, assign18900_e13669_d_n6, assign18900_e13669_d_n7, assign18900_e13669_d_n8, assign18900_e13669_d_n9, assign18900_e13669_d_n10, assign18900_e13669_d_n11, assign18900_e13669_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18900_e13668: f64 = (-locals.var_tmf2);
                (assign18900_e13668, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18900_e13669, assign18900_e13669_d_n0, assign18900_e13669_d_n2, assign18900_e13669_d_n4, assign18900_e13669_d_n5, assign18900_e13669_d_n6, assign18900_e13669_d_n7, assign18900_e13669_d_n8, assign18900_e13669_d_n9, assign18900_e13669_d_n10, assign18900_e13669_d_n11, assign18900_e13669_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18900_e13671;
        locals.var_tmf2_dn0 = assign18900_e13671_d_n0;
        locals.var_tmf2_dn2 = assign18900_e13671_d_n2;
        locals.var_tmf2_dn4 = assign18900_e13671_d_n4;
        locals.var_tmf2_dn5 = assign18900_e13671_d_n5;
        locals.var_tmf2_dn6 = assign18900_e13671_d_n6;
        locals.var_tmf2_dn7 = assign18900_e13671_d_n7;
        locals.var_tmf2_dn8 = assign18900_e13671_d_n8;
        locals.var_tmf2_dn9 = assign18900_e13671_d_n9;
        locals.var_tmf2_dn10 = assign18900_e13671_d_n10;
        locals.var_tmf2_dn11 = assign18900_e13671_d_n11;
        locals.var_tmf2_dn14 = assign18900_e13671_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18910_e13684, assign18910_e13684_d_n0, assign18910_e13684_d_n2, assign18910_e13684_d_n4, assign18910_e13684_d_n5, assign18910_e13684_d_n6, assign18910_e13684_d_n7, assign18910_e13684_d_n8, assign18910_e13684_d_n9, assign18910_e13684_d_n10, assign18910_e13684_d_n11, assign18910_e13684_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18910_e13679: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18910_e13681: f64 = (assign18910_e13679 + locals.var_tmf2);
        let assign18910_e13682: f64 = (assign18910_e13681).sqrt();
        (assign18910_e13682, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18910_e13682)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18910_e13682)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18910_e13682)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18910_e13682)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18910_e13682)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18910_e13682)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18910_e13682)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18910_e13682)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18910_e13682)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18910_e13682)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18910_e13682)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18910_e13684;
        locals.var_tmf2_dn0 = assign18910_e13684_d_n0;
        locals.var_tmf2_dn2 = assign18910_e13684_d_n2;
        locals.var_tmf2_dn4 = assign18910_e13684_d_n4;
        locals.var_tmf2_dn5 = assign18910_e13684_d_n5;
        locals.var_tmf2_dn6 = assign18910_e13684_d_n6;
        locals.var_tmf2_dn7 = assign18910_e13684_d_n7;
        locals.var_tmf2_dn8 = assign18910_e13684_d_n8;
        locals.var_tmf2_dn9 = assign18910_e13684_d_n9;
        locals.var_tmf2_dn10 = assign18910_e13684_d_n10;
        locals.var_tmf2_dn11 = assign18910_e13684_d_n11;
        locals.var_tmf2_dn14 = assign18910_e13684_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18920_e13698, assign18920_e13698_d_n0, assign18920_e13698_d_n2, assign18920_e13698_d_n4, assign18920_e13698_d_n5, assign18920_e13698_d_n6, assign18920_e13698_d_n7, assign18920_e13698_d_n8, assign18920_e13698_d_n9, assign18920_e13698_d_n10, assign18920_e13698_d_n11, assign18920_e13698_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18920_e13694: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign18920_e13695: f64 = (1.0 + assign18920_e13694);
        let assign18920_e13696: f64 = (0.5 * assign18920_e13695);
        (assign18920_e13696, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign18920_e13698;
        locals.var_t6_dn0 = assign18920_e13698_d_n0;
        locals.var_t6_dn2 = assign18920_e13698_d_n2;
        locals.var_t6_dn4 = assign18920_e13698_d_n4;
        locals.var_t6_dn5 = assign18920_e13698_d_n5;
        locals.var_t6_dn6 = assign18920_e13698_d_n6;
        locals.var_t6_dn7 = assign18920_e13698_d_n7;
        locals.var_t6_dn8 = assign18920_e13698_d_n8;
        locals.var_t6_dn9 = assign18920_e13698_d_n9;
        locals.var_t6_dn10 = assign18920_e13698_d_n10;
        locals.var_t6_dn11 = assign18920_e13698_d_n11;
        locals.var_t6_dn14 = assign18920_e13698_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign18930_e13712, assign18930_e13712_d_n0, assign18930_e13712_d_n2, assign18930_e13712_d_n4, assign18930_e13712_d_n5, assign18930_e13712_d_n6, assign18930_e13712_d_n7, assign18930_e13712_d_n8, assign18930_e13712_d_n9, assign18930_e13712_d_n10, assign18930_e13712_d_n11, assign18930_e13712_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign18930_e13708: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign18930_e13709: f64 = (0.5 * assign18930_e13708);
        let assign18930_e13710: f64 = assign18930_e13709;
        (assign18930_e13710, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign18930_e13712;
        locals.var_t2_dn0 = assign18930_e13712_d_n0;
        locals.var_t2_dn2 = assign18930_e13712_d_n2;
        locals.var_t2_dn4 = assign18930_e13712_d_n4;
        locals.var_t2_dn5 = assign18930_e13712_d_n5;
        locals.var_t2_dn6 = assign18930_e13712_d_n6;
        locals.var_t2_dn7 = assign18930_e13712_d_n7;
        locals.var_t2_dn8 = assign18930_e13712_d_n8;
        locals.var_t2_dn9 = assign18930_e13712_d_n9;
        locals.var_t2_dn10 = assign18930_e13712_d_n10;
        locals.var_t2_dn11 = assign18930_e13712_d_n11;
        locals.var_t2_dn14 = assign18930_e13712_d_n14;
        locals.var_t2_rv = 0.0;

        let assign18940_e13719: f64 = if ((p.p39 == 0.0) || (p.p39 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard387 = assign18940_e13719;
        locals.var_guard387_rv = 0.0;

        let (assign18950_e13739, assign18950_e13739_d_n0, assign18950_e13739_d_n2, assign18950_e13739_d_n4, assign18950_e13739_d_n5, assign18950_e13739_d_n6, assign18950_e13739_d_n7, assign18950_e13739_d_n8, assign18950_e13739_d_n9, assign18950_e13739_d_n10, assign18950_e13739_d_n11, assign18950_e13739_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18950_e13730: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff0);
        let assign18950_e13731: f64 = (locals.var_uc_rdvd + assign18950_e13730);
        let assign18950_e13734: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2);
        let assign18950_e13735: f64 = (assign18950_e13731 + assign18950_e13734);
        let assign18950_e13737: f64 = (assign18950_e13735 * locals.var_t2);
        (assign18950_e13737, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign18950_e13735 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign18950_e13735 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign18950_e13735 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign18950_e13735 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign18950_e13735 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign18950_e13735 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign18950_e13735 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign18950_e13735 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign18950_e13735 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn11) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn11)) * locals.var_t2) + (assign18950_e13735 * locals.var_t2_dn11)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn14) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn14)) * locals.var_t2) + (assign18950_e13735 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign18950_e13739;
        locals.var_rdvde_dn0 = assign18950_e13739_d_n0;
        locals.var_rdvde_dn2 = assign18950_e13739_d_n2;
        locals.var_rdvde_dn4 = assign18950_e13739_d_n4;
        locals.var_rdvde_dn5 = assign18950_e13739_d_n5;
        locals.var_rdvde_dn6 = assign18950_e13739_d_n6;
        locals.var_rdvde_dn7 = assign18950_e13739_d_n7;
        locals.var_rdvde_dn8 = assign18950_e13739_d_n8;
        locals.var_rdvde_dn9 = assign18950_e13739_d_n9;
        locals.var_rdvde_dn10 = assign18950_e13739_d_n10;
        locals.var_rdvde_dn11 = assign18950_e13739_d_n11;
        locals.var_rdvde_dn14 = assign18950_e13739_d_n14;
        locals.var_rdvde_rv = 0.0;

        let (assign18960_e13757, assign18960_e13757_d_n0, assign18960_e13757_d_n2, assign18960_e13757_d_n4, assign18960_e13757_d_n5, assign18960_e13757_d_n6, assign18960_e13757_d_n7, assign18960_e13757_d_n8, assign18960_e13757_d_n9, assign18960_e13757_d_n10, assign18960_e13757_d_n11, assign18960_e13757_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18960_e13750: f64 = (0.005 * locals.var_uc_rdvd);
        let assign18960_e13751: f64 = (locals.var_rdvde - assign18960_e13750);
        let assign18960_e13754: f64 = (0.01 * locals.var_uc_rdvd);
        let assign18960_e13755: f64 = (assign18960_e13751 - assign18960_e13754);
        (assign18960_e13755, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign18960_e13757;
        locals.var_tmf1_dn0 = assign18960_e13757_d_n0;
        locals.var_tmf1_dn2 = assign18960_e13757_d_n2;
        locals.var_tmf1_dn4 = assign18960_e13757_d_n4;
        locals.var_tmf1_dn5 = assign18960_e13757_d_n5;
        locals.var_tmf1_dn6 = assign18960_e13757_d_n6;
        locals.var_tmf1_dn7 = assign18960_e13757_d_n7;
        locals.var_tmf1_dn8 = assign18960_e13757_d_n8;
        locals.var_tmf1_dn9 = assign18960_e13757_d_n9;
        locals.var_tmf1_dn10 = assign18960_e13757_d_n10;
        locals.var_tmf1_dn11 = assign18960_e13757_d_n11;
        locals.var_tmf1_dn14 = assign18960_e13757_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign18970_e13775, assign18970_e13775_d_n0, assign18970_e13775_d_n2, assign18970_e13775_d_n4, assign18970_e13775_d_n5, assign18970_e13775_d_n6, assign18970_e13775_d_n7, assign18970_e13775_d_n8, assign18970_e13775_d_n9, assign18970_e13775_d_n10, assign18970_e13775_d_n11, assign18970_e13775_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18970_e13768: f64 = (0.005 * locals.var_uc_rdvd);
        let assign18970_e13769: f64 = (4.0 * assign18970_e13768);
        let assign18970_e13772: f64 = (0.01 * locals.var_uc_rdvd);
        let assign18970_e13773: f64 = (assign18970_e13769 * assign18970_e13772);
        (assign18970_e13773, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18970_e13775;
        locals.var_tmf2_dn0 = assign18970_e13775_d_n0;
        locals.var_tmf2_dn2 = assign18970_e13775_d_n2;
        locals.var_tmf2_dn4 = assign18970_e13775_d_n4;
        locals.var_tmf2_dn5 = assign18970_e13775_d_n5;
        locals.var_tmf2_dn6 = assign18970_e13775_d_n6;
        locals.var_tmf2_dn7 = assign18970_e13775_d_n7;
        locals.var_tmf2_dn8 = assign18970_e13775_d_n8;
        locals.var_tmf2_dn9 = assign18970_e13775_d_n9;
        locals.var_tmf2_dn10 = assign18970_e13775_d_n10;
        locals.var_tmf2_dn11 = assign18970_e13775_d_n11;
        locals.var_tmf2_dn14 = assign18970_e13775_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18980_e13791, assign18980_e13791_d_n0, assign18980_e13791_d_n2, assign18980_e13791_d_n4, assign18980_e13791_d_n5, assign18980_e13791_d_n6, assign18980_e13791_d_n7, assign18980_e13791_d_n8, assign18980_e13791_d_n9, assign18980_e13791_d_n10, assign18980_e13791_d_n11, assign18980_e13791_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let (assign18980_e13789, assign18980_e13789_d_n0, assign18980_e13789_d_n2, assign18980_e13789_d_n4, assign18980_e13789_d_n5, assign18980_e13789_d_n6, assign18980_e13789_d_n7, assign18980_e13789_d_n8, assign18980_e13789_d_n9, assign18980_e13789_d_n10, assign18980_e13789_d_n11, assign18980_e13789_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign18980_e13788: f64 = (-locals.var_tmf2);
                (assign18980_e13788, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign18980_e13789, assign18980_e13789_d_n0, assign18980_e13789_d_n2, assign18980_e13789_d_n4, assign18980_e13789_d_n5, assign18980_e13789_d_n6, assign18980_e13789_d_n7, assign18980_e13789_d_n8, assign18980_e13789_d_n9, assign18980_e13789_d_n10, assign18980_e13789_d_n11, assign18980_e13789_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18980_e13791;
        locals.var_tmf2_dn0 = assign18980_e13791_d_n0;
        locals.var_tmf2_dn2 = assign18980_e13791_d_n2;
        locals.var_tmf2_dn4 = assign18980_e13791_d_n4;
        locals.var_tmf2_dn5 = assign18980_e13791_d_n5;
        locals.var_tmf2_dn6 = assign18980_e13791_d_n6;
        locals.var_tmf2_dn7 = assign18980_e13791_d_n7;
        locals.var_tmf2_dn8 = assign18980_e13791_d_n8;
        locals.var_tmf2_dn9 = assign18980_e13791_d_n9;
        locals.var_tmf2_dn10 = assign18980_e13791_d_n10;
        locals.var_tmf2_dn11 = assign18980_e13791_d_n11;
        locals.var_tmf2_dn14 = assign18980_e13791_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign18990_e13806, assign18990_e13806_d_n0, assign18990_e13806_d_n2, assign18990_e13806_d_n4, assign18990_e13806_d_n5, assign18990_e13806_d_n6, assign18990_e13806_d_n7, assign18990_e13806_d_n8, assign18990_e13806_d_n9, assign18990_e13806_d_n10, assign18990_e13806_d_n11, assign18990_e13806_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign18990_e13801: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign18990_e13803: f64 = (assign18990_e13801 + locals.var_tmf2);
        let assign18990_e13804: f64 = (assign18990_e13803).sqrt();
        (assign18990_e13804, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign18990_e13804)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign18990_e13804)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign18990_e13804)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign18990_e13804)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign18990_e13804)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign18990_e13804)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign18990_e13804)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign18990_e13804)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign18990_e13804)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign18990_e13804)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign18990_e13804)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign18990_e13806;
        locals.var_tmf2_dn0 = assign18990_e13806_d_n0;
        locals.var_tmf2_dn2 = assign18990_e13806_d_n2;
        locals.var_tmf2_dn4 = assign18990_e13806_d_n4;
        locals.var_tmf2_dn5 = assign18990_e13806_d_n5;
        locals.var_tmf2_dn6 = assign18990_e13806_d_n6;
        locals.var_tmf2_dn7 = assign18990_e13806_d_n7;
        locals.var_tmf2_dn8 = assign18990_e13806_d_n8;
        locals.var_tmf2_dn9 = assign18990_e13806_d_n9;
        locals.var_tmf2_dn10 = assign18990_e13806_d_n10;
        locals.var_tmf2_dn11 = assign18990_e13806_d_n11;
        locals.var_tmf2_dn14 = assign18990_e13806_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19000_e13822, assign19000_e13822_d_n0, assign19000_e13822_d_n2, assign19000_e13822_d_n4, assign19000_e13822_d_n5, assign19000_e13822_d_n6, assign19000_e13822_d_n7, assign19000_e13822_d_n8, assign19000_e13822_d_n9, assign19000_e13822_d_n10, assign19000_e13822_d_n11, assign19000_e13822_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19000_e13818: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19000_e13819: f64 = (1.0 + assign19000_e13818);
        let assign19000_e13820: f64 = (0.5 * assign19000_e13819);
        (assign19000_e13820, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign19000_e13822;
        locals.var_t0_dn0 = assign19000_e13822_d_n0;
        locals.var_t0_dn2 = assign19000_e13822_d_n2;
        locals.var_t0_dn4 = assign19000_e13822_d_n4;
        locals.var_t0_dn5 = assign19000_e13822_d_n5;
        locals.var_t0_dn6 = assign19000_e13822_d_n6;
        locals.var_t0_dn7 = assign19000_e13822_d_n7;
        locals.var_t0_dn8 = assign19000_e13822_d_n8;
        locals.var_t0_dn9 = assign19000_e13822_d_n9;
        locals.var_t0_dn10 = assign19000_e13822_d_n10;
        locals.var_t0_dn11 = assign19000_e13822_d_n11;
        locals.var_t0_dn14 = assign19000_e13822_d_n14;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_47(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19010_e13840, assign19010_e13840_d_n0, assign19010_e13840_d_n2, assign19010_e13840_d_n4, assign19010_e13840_d_n5, assign19010_e13840_d_n6, assign19010_e13840_d_n7, assign19010_e13840_d_n8, assign19010_e13840_d_n9, assign19010_e13840_d_n10, assign19010_e13840_d_n11, assign19010_e13840_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard387 != 0.0)) {
        let assign19010_e13832: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19010_e13836: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19010_e13837: f64 = (0.5 * assign19010_e13836);
        let assign19010_e13838: f64 = (assign19010_e13832 + assign19010_e13837);
        (assign19010_e13838, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign19010_e13840;
        locals.var_rdvde_dn0 = assign19010_e13840_d_n0;
        locals.var_rdvde_dn2 = assign19010_e13840_d_n2;
        locals.var_rdvde_dn4 = assign19010_e13840_d_n4;
        locals.var_rdvde_dn5 = assign19010_e13840_d_n5;
        locals.var_rdvde_dn6 = assign19010_e13840_d_n6;
        locals.var_rdvde_dn7 = assign19010_e13840_d_n7;
        locals.var_rdvde_dn8 = assign19010_e13840_d_n8;
        locals.var_rdvde_dn9 = assign19010_e13840_d_n9;
        locals.var_rdvde_dn10 = assign19010_e13840_d_n10;
        locals.var_rdvde_dn11 = assign19010_e13840_d_n11;
        locals.var_rdvde_dn14 = assign19010_e13840_d_n14;
        locals.var_rdvde_rv = 0.0;

        let (assign19020_e13861, assign19020_e13861_d_n0, assign19020_e13861_d_n2, assign19020_e13861_d_n4, assign19020_e13861_d_n5, assign19020_e13861_d_n6, assign19020_e13861_d_n7, assign19020_e13861_d_n8, assign19020_e13861_d_n9, assign19020_e13861_d_n10, assign19020_e13861_d_n11, assign19020_e13861_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard387 == 0.0)) {
        let assign19020_e13852: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff);
        let assign19020_e13853: f64 = (locals.var_uc_rdvd + assign19020_e13852);
        let assign19020_e13856: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2);
        let assign19020_e13857: f64 = (assign19020_e13853 + assign19020_e13856);
        let assign19020_e13859: f64 = (assign19020_e13857 * locals.var_t2);
        (assign19020_e13859, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign19020_e13857 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign19020_e13857 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign19020_e13857 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign19020_e13857 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign19020_e13857 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign19020_e13857 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign19020_e13857 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign19020_e13857 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign19020_e13857 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn11) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn11)) * locals.var_t2) + (assign19020_e13857 * locals.var_t2_dn11)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn14) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn14)) * locals.var_t2) + (assign19020_e13857 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign19020_e13861;
        locals.var_rdvde_dn0 = assign19020_e13861_d_n0;
        locals.var_rdvde_dn2 = assign19020_e13861_d_n2;
        locals.var_rdvde_dn4 = assign19020_e13861_d_n4;
        locals.var_rdvde_dn5 = assign19020_e13861_d_n5;
        locals.var_rdvde_dn6 = assign19020_e13861_d_n6;
        locals.var_rdvde_dn7 = assign19020_e13861_d_n7;
        locals.var_rdvde_dn8 = assign19020_e13861_d_n8;
        locals.var_rdvde_dn9 = assign19020_e13861_d_n9;
        locals.var_rdvde_dn10 = assign19020_e13861_d_n10;
        locals.var_rdvde_dn11 = assign19020_e13861_d_n11;
        locals.var_rdvde_dn14 = assign19020_e13861_d_n14;
        locals.var_rdvde_rv = 0.0;

        let (assign19030_e13880, assign19030_e13880_d_n0, assign19030_e13880_d_n2, assign19030_e13880_d_n4, assign19030_e13880_d_n5, assign19030_e13880_d_n6, assign19030_e13880_d_n7, assign19030_e13880_d_n8, assign19030_e13880_d_n9, assign19030_e13880_d_n10, assign19030_e13880_d_n11, assign19030_e13880_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard387 == 0.0)) {
        let assign19030_e13873: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19030_e13874: f64 = (locals.var_rdvde - assign19030_e13873);
        let assign19030_e13877: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19030_e13878: f64 = (assign19030_e13874 - assign19030_e13877);
        (assign19030_e13878, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign19030_e13880;
        locals.var_tmf1_dn0 = assign19030_e13880_d_n0;
        locals.var_tmf1_dn2 = assign19030_e13880_d_n2;
        locals.var_tmf1_dn4 = assign19030_e13880_d_n4;
        locals.var_tmf1_dn5 = assign19030_e13880_d_n5;
        locals.var_tmf1_dn6 = assign19030_e13880_d_n6;
        locals.var_tmf1_dn7 = assign19030_e13880_d_n7;
        locals.var_tmf1_dn8 = assign19030_e13880_d_n8;
        locals.var_tmf1_dn9 = assign19030_e13880_d_n9;
        locals.var_tmf1_dn10 = assign19030_e13880_d_n10;
        locals.var_tmf1_dn11 = assign19030_e13880_d_n11;
        locals.var_tmf1_dn14 = assign19030_e13880_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign19040_e13899, assign19040_e13899_d_n0, assign19040_e13899_d_n2, assign19040_e13899_d_n4, assign19040_e13899_d_n5, assign19040_e13899_d_n6, assign19040_e13899_d_n7, assign19040_e13899_d_n8, assign19040_e13899_d_n9, assign19040_e13899_d_n10, assign19040_e13899_d_n11, assign19040_e13899_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard387 == 0.0)) {
        let assign19040_e13892: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19040_e13893: f64 = (4.0 * assign19040_e13892);
        let assign19040_e13896: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19040_e13897: f64 = (assign19040_e13893 * assign19040_e13896);
        (assign19040_e13897, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19040_e13899;
        locals.var_tmf2_dn0 = assign19040_e13899_d_n0;
        locals.var_tmf2_dn2 = assign19040_e13899_d_n2;
        locals.var_tmf2_dn4 = assign19040_e13899_d_n4;
        locals.var_tmf2_dn5 = assign19040_e13899_d_n5;
        locals.var_tmf2_dn6 = assign19040_e13899_d_n6;
        locals.var_tmf2_dn7 = assign19040_e13899_d_n7;
        locals.var_tmf2_dn8 = assign19040_e13899_d_n8;
        locals.var_tmf2_dn9 = assign19040_e13899_d_n9;
        locals.var_tmf2_dn10 = assign19040_e13899_d_n10;
        locals.var_tmf2_dn11 = assign19040_e13899_d_n11;
        locals.var_tmf2_dn14 = assign19040_e13899_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19050_e13916, assign19050_e13916_d_n0, assign19050_e13916_d_n2, assign19050_e13916_d_n4, assign19050_e13916_d_n5, assign19050_e13916_d_n6, assign19050_e13916_d_n7, assign19050_e13916_d_n8, assign19050_e13916_d_n9, assign19050_e13916_d_n10, assign19050_e13916_d_n11, assign19050_e13916_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard387 == 0.0)) {
        let (assign19050_e13914, assign19050_e13914_d_n0, assign19050_e13914_d_n2, assign19050_e13914_d_n4, assign19050_e13914_d_n5, assign19050_e13914_d_n6, assign19050_e13914_d_n7, assign19050_e13914_d_n8, assign19050_e13914_d_n9, assign19050_e13914_d_n10, assign19050_e13914_d_n11, assign19050_e13914_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign19050_e13913: f64 = (-locals.var_tmf2);
                (assign19050_e13913, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign19050_e13914, assign19050_e13914_d_n0, assign19050_e13914_d_n2, assign19050_e13914_d_n4, assign19050_e13914_d_n5, assign19050_e13914_d_n6, assign19050_e13914_d_n7, assign19050_e13914_d_n8, assign19050_e13914_d_n9, assign19050_e13914_d_n10, assign19050_e13914_d_n11, assign19050_e13914_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19050_e13916;
        locals.var_tmf2_dn0 = assign19050_e13916_d_n0;
        locals.var_tmf2_dn2 = assign19050_e13916_d_n2;
        locals.var_tmf2_dn4 = assign19050_e13916_d_n4;
        locals.var_tmf2_dn5 = assign19050_e13916_d_n5;
        locals.var_tmf2_dn6 = assign19050_e13916_d_n6;
        locals.var_tmf2_dn7 = assign19050_e13916_d_n7;
        locals.var_tmf2_dn8 = assign19050_e13916_d_n8;
        locals.var_tmf2_dn9 = assign19050_e13916_d_n9;
        locals.var_tmf2_dn10 = assign19050_e13916_d_n10;
        locals.var_tmf2_dn11 = assign19050_e13916_d_n11;
        locals.var_tmf2_dn14 = assign19050_e13916_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19060_e13932, assign19060_e13932_d_n0, assign19060_e13932_d_n2, assign19060_e13932_d_n4, assign19060_e13932_d_n5, assign19060_e13932_d_n6, assign19060_e13932_d_n7, assign19060_e13932_d_n8, assign19060_e13932_d_n9, assign19060_e13932_d_n10, assign19060_e13932_d_n11, assign19060_e13932_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard387 == 0.0)) {
        let assign19060_e13927: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19060_e13929: f64 = (assign19060_e13927 + locals.var_tmf2);
        let assign19060_e13930: f64 = (assign19060_e13929).sqrt();
        (assign19060_e13930, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19060_e13930)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19060_e13930)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19060_e13930)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19060_e13930)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19060_e13930)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19060_e13930)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19060_e13930)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19060_e13930)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19060_e13930)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19060_e13930)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19060_e13930)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19060_e13932;
        locals.var_tmf2_dn0 = assign19060_e13932_d_n0;
        locals.var_tmf2_dn2 = assign19060_e13932_d_n2;
        locals.var_tmf2_dn4 = assign19060_e13932_d_n4;
        locals.var_tmf2_dn5 = assign19060_e13932_d_n5;
        locals.var_tmf2_dn6 = assign19060_e13932_d_n6;
        locals.var_tmf2_dn7 = assign19060_e13932_d_n7;
        locals.var_tmf2_dn8 = assign19060_e13932_d_n8;
        locals.var_tmf2_dn9 = assign19060_e13932_d_n9;
        locals.var_tmf2_dn10 = assign19060_e13932_d_n10;
        locals.var_tmf2_dn11 = assign19060_e13932_d_n11;
        locals.var_tmf2_dn14 = assign19060_e13932_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19070_e13949, assign19070_e13949_d_n0, assign19070_e13949_d_n2, assign19070_e13949_d_n4, assign19070_e13949_d_n5, assign19070_e13949_d_n6, assign19070_e13949_d_n7, assign19070_e13949_d_n8, assign19070_e13949_d_n9, assign19070_e13949_d_n10, assign19070_e13949_d_n11, assign19070_e13949_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard387 == 0.0)) {
        let assign19070_e13945: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19070_e13946: f64 = (1.0 + assign19070_e13945);
        let assign19070_e13947: f64 = (0.5 * assign19070_e13946);
        (assign19070_e13947, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign19070_e13949;
        locals.var_t0_dn0 = assign19070_e13949_d_n0;
        locals.var_t0_dn2 = assign19070_e13949_d_n2;
        locals.var_t0_dn4 = assign19070_e13949_d_n4;
        locals.var_t0_dn5 = assign19070_e13949_d_n5;
        locals.var_t0_dn6 = assign19070_e13949_d_n6;
        locals.var_t0_dn7 = assign19070_e13949_d_n7;
        locals.var_t0_dn8 = assign19070_e13949_d_n8;
        locals.var_t0_dn9 = assign19070_e13949_d_n9;
        locals.var_t0_dn10 = assign19070_e13949_d_n10;
        locals.var_t0_dn11 = assign19070_e13949_d_n11;
        locals.var_t0_dn14 = assign19070_e13949_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign19080_e13968, assign19080_e13968_d_n0, assign19080_e13968_d_n2, assign19080_e13968_d_n4, assign19080_e13968_d_n5, assign19080_e13968_d_n6, assign19080_e13968_d_n7, assign19080_e13968_d_n8, assign19080_e13968_d_n9, assign19080_e13968_d_n10, assign19080_e13968_d_n11, assign19080_e13968_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard387 == 0.0)) {
        let assign19080_e13960: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19080_e13964: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19080_e13965: f64 = (0.5 * assign19080_e13964);
        let assign19080_e13966: f64 = (assign19080_e13960 + assign19080_e13965);
        (assign19080_e13966, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign19080_e13968;
        locals.var_rdvde_dn0 = assign19080_e13968_d_n0;
        locals.var_rdvde_dn2 = assign19080_e13968_d_n2;
        locals.var_rdvde_dn4 = assign19080_e13968_d_n4;
        locals.var_rdvde_dn5 = assign19080_e13968_d_n5;
        locals.var_rdvde_dn6 = assign19080_e13968_d_n6;
        locals.var_rdvde_dn7 = assign19080_e13968_d_n7;
        locals.var_rdvde_dn8 = assign19080_e13968_d_n8;
        locals.var_rdvde_dn9 = assign19080_e13968_d_n9;
        locals.var_rdvde_dn10 = assign19080_e13968_d_n10;
        locals.var_rdvde_dn11 = assign19080_e13968_d_n11;
        locals.var_rdvde_dn14 = assign19080_e13968_d_n14;
        locals.var_rdvde_rv = 0.0;

        let (assign19090_e13992, assign19090_e13992_d_n0, assign19090_e13992_d_n2, assign19090_e13992_d_n4, assign19090_e13992_d_n5, assign19090_e13992_d_n6, assign19090_e13992_d_n7, assign19090_e13992_d_n8, assign19090_e13992_d_n9, assign19090_e13992_d_n10, assign19090_e13992_d_n11, assign19090_e13992_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign19090_e13977: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign19090_e13979: f64 = (assign19090_e13977 * 1000000.0);
        let assign19090_e13981: f64 = (assign19090_e13979 + locals.var_uc_rdict1);
        let assign19090_e13982: f64 = (locals.var_rdvdtemp0 * assign19090_e13981);
        let assign19090_e13985: f64 = (p.p70 * p.p100);
        let assign19090_e13987: f64 = (assign19090_e13985 * 1000000.0);
        let assign19090_e13989: f64 = (assign19090_e13987 + p.p101);
        let assign19090_e13990: f64 = (assign19090_e13982 * assign19090_e13989);
        (assign19090_e13990, ((locals.var_rdvdtemp0_dn0 * assign19090_e13981) * assign19090_e13989), ((locals.var_rdvdtemp0_dn2 * assign19090_e13981) * assign19090_e13989), ((locals.var_rdvdtemp0_dn4 * assign19090_e13981) * assign19090_e13989), ((locals.var_rdvdtemp0_dn5 * assign19090_e13981) * assign19090_e13989), ((locals.var_rdvdtemp0_dn6 * assign19090_e13981) * assign19090_e13989), ((locals.var_rdvdtemp0_dn7 * assign19090_e13981) * assign19090_e13989), ((locals.var_rdvdtemp0_dn8 * assign19090_e13981) * assign19090_e13989), ((locals.var_rdvdtemp0_dn9 * assign19090_e13981) * assign19090_e13989), ((locals.var_rdvdtemp0_dn10 * assign19090_e13981) * assign19090_e13989), ((locals.var_rdvdtemp0_dn11 * assign19090_e13981) * assign19090_e13989), ((locals.var_rdvdtemp0_dn14 * assign19090_e13981) * assign19090_e13989),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign19090_e13992;
        locals.var_t4_dn0 = assign19090_e13992_d_n0;
        locals.var_t4_dn2 = assign19090_e13992_d_n2;
        locals.var_t4_dn4 = assign19090_e13992_d_n4;
        locals.var_t4_dn5 = assign19090_e13992_d_n5;
        locals.var_t4_dn6 = assign19090_e13992_d_n6;
        locals.var_t4_dn7 = assign19090_e13992_d_n7;
        locals.var_t4_dn8 = assign19090_e13992_d_n8;
        locals.var_t4_dn9 = assign19090_e13992_d_n9;
        locals.var_t4_dn10 = assign19090_e13992_d_n10;
        locals.var_t4_dn11 = assign19090_e13992_d_n11;
        locals.var_t4_dn14 = assign19090_e13992_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign19100_e14006, assign19100_e14006_d_n0, assign19100_e14006_d_n2, assign19100_e14006_d_n4, assign19100_e14006_d_n5, assign19100_e14006_d_n6, assign19100_e14006_d_n7, assign19100_e14006_d_n8, assign19100_e14006_d_n9, assign19100_e14006_d_n10, assign19100_e14006_d_n11, assign19100_e14006_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign19100_e14000: f64 = (1.0 - locals.var_uc_rdov13);
        let assign19100_e14002: f64 = (assign19100_e14000 * p.p66);
        let assign19100_e14004: f64 = (assign19100_e14002 * 1000000.0);
        (assign19100_e14004, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign19100_e14006;
        locals.var_t1_dn0 = assign19100_e14006_d_n0;
        locals.var_t1_dn2 = assign19100_e14006_d_n2;
        locals.var_t1_dn4 = assign19100_e14006_d_n4;
        locals.var_t1_dn5 = assign19100_e14006_d_n5;
        locals.var_t1_dn6 = assign19100_e14006_d_n6;
        locals.var_t1_dn7 = assign19100_e14006_d_n7;
        locals.var_t1_dn8 = assign19100_e14006_d_n8;
        locals.var_t1_dn9 = assign19100_e14006_d_n9;
        locals.var_t1_dn10 = assign19100_e14006_d_n10;
        locals.var_t1_dn11 = assign19100_e14006_d_n11;
        locals.var_t1_dn14 = assign19100_e14006_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign19110_e14022, assign19110_e14022_d_n0, assign19110_e14022_d_n2, assign19110_e14022_d_n4, assign19110_e14022_d_n5, assign19110_e14022_d_n6, assign19110_e14022_d_n7, assign19110_e14022_d_n8, assign19110_e14022_d_n9, assign19110_e14022_d_n10, assign19110_e14022_d_n11, assign19110_e14022_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign19110_e14014: f64 = (locals.var_t8 * p.p66);
        let assign19110_e14016: f64 = (assign19110_e14014 * 1000000.0);
        let assign19110_e14018: f64 = (assign19110_e14016 + 1.0);
        let assign19110_e14020: f64 = (assign19110_e14018 + p.p98);
        (assign19110_e14020, ((locals.var_t8_dn0 * p.p66) * 1000000.0), ((locals.var_t8_dn2 * p.p66) * 1000000.0), ((locals.var_t8_dn4 * p.p66) * 1000000.0), ((locals.var_t8_dn5 * p.p66) * 1000000.0), ((locals.var_t8_dn6 * p.p66) * 1000000.0), ((locals.var_t8_dn7 * p.p66) * 1000000.0), ((locals.var_t8_dn8 * p.p66) * 1000000.0), ((locals.var_t8_dn9 * p.p66) * 1000000.0), ((locals.var_t8_dn10 * p.p66) * 1000000.0), ((locals.var_t8_dn11 * p.p66) * 1000000.0), ((locals.var_t8_dn14 * p.p66) * 1000000.0),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign19110_e14022;
        locals.var_t3_dn0 = assign19110_e14022_d_n0;
        locals.var_t3_dn2 = assign19110_e14022_d_n2;
        locals.var_t3_dn4 = assign19110_e14022_d_n4;
        locals.var_t3_dn5 = assign19110_e14022_d_n5;
        locals.var_t3_dn6 = assign19110_e14022_d_n6;
        locals.var_t3_dn7 = assign19110_e14022_d_n7;
        locals.var_t3_dn8 = assign19110_e14022_d_n8;
        locals.var_t3_dn9 = assign19110_e14022_d_n9;
        locals.var_t3_dn10 = assign19110_e14022_d_n10;
        locals.var_t3_dn11 = assign19110_e14022_d_n11;
        locals.var_t3_dn14 = assign19110_e14022_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign19120_e14036, assign19120_e14036_d_n0, assign19120_e14036_d_n2, assign19120_e14036_d_n4, assign19120_e14036_d_n5, assign19120_e14036_d_n6, assign19120_e14036_d_n7, assign19120_e14036_d_n8, assign19120_e14036_d_n9, assign19120_e14036_d_n10, assign19120_e14036_d_n11, assign19120_e14036_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign19120_e14030: f64 = (locals.var_t3 * locals.var_t4);
        let assign19120_e14032: f64 = (assign19120_e14030 - locals.var_t4);
        let assign19120_e14034: f64 = (assign19120_e14032 - 0.01);
        (assign19120_e14034, (((locals.var_t3_dn0 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn0)) - locals.var_t4_dn0), (((locals.var_t3_dn2 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn2)) - locals.var_t4_dn2), (((locals.var_t3_dn4 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn4)) - locals.var_t4_dn4), (((locals.var_t3_dn5 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn5)) - locals.var_t4_dn5), (((locals.var_t3_dn6 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn6)) - locals.var_t4_dn6), (((locals.var_t3_dn7 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn7)) - locals.var_t4_dn7), (((locals.var_t3_dn8 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn8)) - locals.var_t4_dn8), (((locals.var_t3_dn9 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn9)) - locals.var_t4_dn9), (((locals.var_t3_dn10 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn10)) - locals.var_t4_dn10), (((locals.var_t3_dn11 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn11)) - locals.var_t4_dn11), (((locals.var_t3_dn14 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn14)) - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign19120_e14036;
        locals.var_tmf1_dn0 = assign19120_e14036_d_n0;
        locals.var_tmf1_dn2 = assign19120_e14036_d_n2;
        locals.var_tmf1_dn4 = assign19120_e14036_d_n4;
        locals.var_tmf1_dn5 = assign19120_e14036_d_n5;
        locals.var_tmf1_dn6 = assign19120_e14036_d_n6;
        locals.var_tmf1_dn7 = assign19120_e14036_d_n7;
        locals.var_tmf1_dn8 = assign19120_e14036_d_n8;
        locals.var_tmf1_dn9 = assign19120_e14036_d_n9;
        locals.var_tmf1_dn10 = assign19120_e14036_d_n10;
        locals.var_tmf1_dn11 = assign19120_e14036_d_n11;
        locals.var_tmf1_dn14 = assign19120_e14036_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign19130_e14048, assign19130_e14048_d_n0, assign19130_e14048_d_n2, assign19130_e14048_d_n4, assign19130_e14048_d_n5, assign19130_e14048_d_n6, assign19130_e14048_d_n7, assign19130_e14048_d_n8, assign19130_e14048_d_n9, assign19130_e14048_d_n10, assign19130_e14048_d_n11, assign19130_e14048_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign19130_e14044: f64 = (4.0 * locals.var_t4);
        let assign19130_e14046: f64 = (assign19130_e14044 * 0.01);
        (assign19130_e14046, ((4.0 * locals.var_t4_dn0) * 0.01), ((4.0 * locals.var_t4_dn2) * 0.01), ((4.0 * locals.var_t4_dn4) * 0.01), ((4.0 * locals.var_t4_dn5) * 0.01), ((4.0 * locals.var_t4_dn6) * 0.01), ((4.0 * locals.var_t4_dn7) * 0.01), ((4.0 * locals.var_t4_dn8) * 0.01), ((4.0 * locals.var_t4_dn9) * 0.01), ((4.0 * locals.var_t4_dn10) * 0.01), ((4.0 * locals.var_t4_dn11) * 0.01), ((4.0 * locals.var_t4_dn14) * 0.01),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19130_e14048;
        locals.var_tmf2_dn0 = assign19130_e14048_d_n0;
        locals.var_tmf2_dn2 = assign19130_e14048_d_n2;
        locals.var_tmf2_dn4 = assign19130_e14048_d_n4;
        locals.var_tmf2_dn5 = assign19130_e14048_d_n5;
        locals.var_tmf2_dn6 = assign19130_e14048_d_n6;
        locals.var_tmf2_dn7 = assign19130_e14048_d_n7;
        locals.var_tmf2_dn8 = assign19130_e14048_d_n8;
        locals.var_tmf2_dn9 = assign19130_e14048_d_n9;
        locals.var_tmf2_dn10 = assign19130_e14048_d_n10;
        locals.var_tmf2_dn11 = assign19130_e14048_d_n11;
        locals.var_tmf2_dn14 = assign19130_e14048_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19140_e14062, assign19140_e14062_d_n0, assign19140_e14062_d_n2, assign19140_e14062_d_n4, assign19140_e14062_d_n5, assign19140_e14062_d_n6, assign19140_e14062_d_n7, assign19140_e14062_d_n8, assign19140_e14062_d_n9, assign19140_e14062_d_n10, assign19140_e14062_d_n11, assign19140_e14062_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let (assign19140_e14060, assign19140_e14060_d_n0, assign19140_e14060_d_n2, assign19140_e14060_d_n4, assign19140_e14060_d_n5, assign19140_e14060_d_n6, assign19140_e14060_d_n7, assign19140_e14060_d_n8, assign19140_e14060_d_n9, assign19140_e14060_d_n10, assign19140_e14060_d_n11, assign19140_e14060_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign19140_e14059: f64 = (-locals.var_tmf2);
                (assign19140_e14059, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign19140_e14060, assign19140_e14060_d_n0, assign19140_e14060_d_n2, assign19140_e14060_d_n4, assign19140_e14060_d_n5, assign19140_e14060_d_n6, assign19140_e14060_d_n7, assign19140_e14060_d_n8, assign19140_e14060_d_n9, assign19140_e14060_d_n10, assign19140_e14060_d_n11, assign19140_e14060_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19140_e14062;
        locals.var_tmf2_dn0 = assign19140_e14062_d_n0;
        locals.var_tmf2_dn2 = assign19140_e14062_d_n2;
        locals.var_tmf2_dn4 = assign19140_e14062_d_n4;
        locals.var_tmf2_dn5 = assign19140_e14062_d_n5;
        locals.var_tmf2_dn6 = assign19140_e14062_d_n6;
        locals.var_tmf2_dn7 = assign19140_e14062_d_n7;
        locals.var_tmf2_dn8 = assign19140_e14062_d_n8;
        locals.var_tmf2_dn9 = assign19140_e14062_d_n9;
        locals.var_tmf2_dn10 = assign19140_e14062_d_n10;
        locals.var_tmf2_dn11 = assign19140_e14062_d_n11;
        locals.var_tmf2_dn14 = assign19140_e14062_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19150_e14075, assign19150_e14075_d_n0, assign19150_e14075_d_n2, assign19150_e14075_d_n4, assign19150_e14075_d_n5, assign19150_e14075_d_n6, assign19150_e14075_d_n7, assign19150_e14075_d_n8, assign19150_e14075_d_n9, assign19150_e14075_d_n10, assign19150_e14075_d_n11, assign19150_e14075_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign19150_e14070: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19150_e14072: f64 = (assign19150_e14070 + locals.var_tmf2);
        let assign19150_e14073: f64 = (assign19150_e14072).sqrt();
        (assign19150_e14073, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19150_e14073)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19150_e14073)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19150_e14073)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19150_e14073)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19150_e14073)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19150_e14073)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19150_e14073)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19150_e14073)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19150_e14073)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19150_e14073)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19150_e14073)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19150_e14075;
        locals.var_tmf2_dn0 = assign19150_e14075_d_n0;
        locals.var_tmf2_dn2 = assign19150_e14075_d_n2;
        locals.var_tmf2_dn4 = assign19150_e14075_d_n4;
        locals.var_tmf2_dn5 = assign19150_e14075_d_n5;
        locals.var_tmf2_dn6 = assign19150_e14075_d_n6;
        locals.var_tmf2_dn7 = assign19150_e14075_d_n7;
        locals.var_tmf2_dn8 = assign19150_e14075_d_n8;
        locals.var_tmf2_dn9 = assign19150_e14075_d_n9;
        locals.var_tmf2_dn10 = assign19150_e14075_d_n10;
        locals.var_tmf2_dn11 = assign19150_e14075_d_n11;
        locals.var_tmf2_dn14 = assign19150_e14075_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19160_e14089, assign19160_e14089_d_n0, assign19160_e14089_d_n2, assign19160_e14089_d_n4, assign19160_e14089_d_n5, assign19160_e14089_d_n6, assign19160_e14089_d_n7, assign19160_e14089_d_n8, assign19160_e14089_d_n9, assign19160_e14089_d_n10, assign19160_e14089_d_n11, assign19160_e14089_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign19160_e14085: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19160_e14086: f64 = (1.0 + assign19160_e14085);
        let assign19160_e14087: f64 = (0.5 * assign19160_e14086);
        (assign19160_e14087, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign19160_e14089;
        locals.var_t6_dn0 = assign19160_e14089_d_n0;
        locals.var_t6_dn2 = assign19160_e14089_d_n2;
        locals.var_t6_dn4 = assign19160_e14089_d_n4;
        locals.var_t6_dn5 = assign19160_e14089_d_n5;
        locals.var_t6_dn6 = assign19160_e14089_d_n6;
        locals.var_t6_dn7 = assign19160_e14089_d_n7;
        locals.var_t6_dn8 = assign19160_e14089_d_n8;
        locals.var_t6_dn9 = assign19160_e14089_d_n9;
        locals.var_t6_dn10 = assign19160_e14089_d_n10;
        locals.var_t6_dn11 = assign19160_e14089_d_n11;
        locals.var_t6_dn14 = assign19160_e14089_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign19170_e14103, assign19170_e14103_d_n0, assign19170_e14103_d_n2, assign19170_e14103_d_n4, assign19170_e14103_d_n5, assign19170_e14103_d_n6, assign19170_e14103_d_n7, assign19170_e14103_d_n8, assign19170_e14103_d_n9, assign19170_e14103_d_n10, assign19170_e14103_d_n11, assign19170_e14103_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign19170_e14099: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19170_e14100: f64 = (0.5 * assign19170_e14099);
        let assign19170_e14101: f64 = (locals.var_t4 + assign19170_e14100);
        (assign19170_e14101, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t4_dn14 + (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign19170_e14103;
        locals.var_t5_dn0 = assign19170_e14103_d_n0;
        locals.var_t5_dn2 = assign19170_e14103_d_n2;
        locals.var_t5_dn4 = assign19170_e14103_d_n4;
        locals.var_t5_dn5 = assign19170_e14103_d_n5;
        locals.var_t5_dn6 = assign19170_e14103_d_n6;
        locals.var_t5_dn7 = assign19170_e14103_d_n7;
        locals.var_t5_dn8 = assign19170_e14103_d_n8;
        locals.var_t5_dn9 = assign19170_e14103_d_n9;
        locals.var_t5_dn10 = assign19170_e14103_d_n10;
        locals.var_t5_dn11 = assign19170_e14103_d_n11;
        locals.var_t5_dn14 = assign19170_e14103_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign19180_e14119, assign19180_e14119_d_n0, assign19180_e14119_d_n2, assign19180_e14119_d_n4, assign19180_e14119_d_n5, assign19180_e14119_d_n6, assign19180_e14119_d_n7, assign19180_e14119_d_n8, assign19180_e14119_d_n9, assign19180_e14119_d_n10, assign19180_e14119_d_n11, assign19180_e14119_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign19180_e14112: f64 = (p.p98 + 1.0);
        let assign19180_e14113: f64 = (locals.var_t4 * assign19180_e14112);
        let assign19180_e14115: f64 = (assign19180_e14113 - locals.var_t5);
        let assign19180_e14117: f64 = (assign19180_e14115 - 5e-5);
        (assign19180_e14117, ((locals.var_t4_dn0 * assign19180_e14112) - locals.var_t5_dn0), ((locals.var_t4_dn2 * assign19180_e14112) - locals.var_t5_dn2), ((locals.var_t4_dn4 * assign19180_e14112) - locals.var_t5_dn4), ((locals.var_t4_dn5 * assign19180_e14112) - locals.var_t5_dn5), ((locals.var_t4_dn6 * assign19180_e14112) - locals.var_t5_dn6), ((locals.var_t4_dn7 * assign19180_e14112) - locals.var_t5_dn7), ((locals.var_t4_dn8 * assign19180_e14112) - locals.var_t5_dn8), ((locals.var_t4_dn9 * assign19180_e14112) - locals.var_t5_dn9), ((locals.var_t4_dn10 * assign19180_e14112) - locals.var_t5_dn10), ((locals.var_t4_dn11 * assign19180_e14112) - locals.var_t5_dn11), ((locals.var_t4_dn14 * assign19180_e14112) - locals.var_t5_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign19180_e14119;
        locals.var_tmf1_dn0 = assign19180_e14119_d_n0;
        locals.var_tmf1_dn2 = assign19180_e14119_d_n2;
        locals.var_tmf1_dn4 = assign19180_e14119_d_n4;
        locals.var_tmf1_dn5 = assign19180_e14119_d_n5;
        locals.var_tmf1_dn6 = assign19180_e14119_d_n6;
        locals.var_tmf1_dn7 = assign19180_e14119_d_n7;
        locals.var_tmf1_dn8 = assign19180_e14119_d_n8;
        locals.var_tmf1_dn9 = assign19180_e14119_d_n9;
        locals.var_tmf1_dn10 = assign19180_e14119_d_n10;
        locals.var_tmf1_dn11 = assign19180_e14119_d_n11;
        locals.var_tmf1_dn14 = assign19180_e14119_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign19190_e14135, assign19190_e14135_d_n0, assign19190_e14135_d_n2, assign19190_e14135_d_n4, assign19190_e14135_d_n5, assign19190_e14135_d_n6, assign19190_e14135_d_n7, assign19190_e14135_d_n8, assign19190_e14135_d_n9, assign19190_e14135_d_n10, assign19190_e14135_d_n11, assign19190_e14135_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign19190_e14129: f64 = (p.p98 + 1.0);
        let assign19190_e14130: f64 = (locals.var_t4 * assign19190_e14129);
        let assign19190_e14131: f64 = (4.0 * assign19190_e14130);
        let assign19190_e14133: f64 = (assign19190_e14131 * 5e-5);
        (assign19190_e14133, ((4.0 * (locals.var_t4_dn0 * assign19190_e14129)) * 5e-5), ((4.0 * (locals.var_t4_dn2 * assign19190_e14129)) * 5e-5), ((4.0 * (locals.var_t4_dn4 * assign19190_e14129)) * 5e-5), ((4.0 * (locals.var_t4_dn5 * assign19190_e14129)) * 5e-5), ((4.0 * (locals.var_t4_dn6 * assign19190_e14129)) * 5e-5), ((4.0 * (locals.var_t4_dn7 * assign19190_e14129)) * 5e-5), ((4.0 * (locals.var_t4_dn8 * assign19190_e14129)) * 5e-5), ((4.0 * (locals.var_t4_dn9 * assign19190_e14129)) * 5e-5), ((4.0 * (locals.var_t4_dn10 * assign19190_e14129)) * 5e-5), ((4.0 * (locals.var_t4_dn11 * assign19190_e14129)) * 5e-5), ((4.0 * (locals.var_t4_dn14 * assign19190_e14129)) * 5e-5),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19190_e14135;
        locals.var_tmf2_dn0 = assign19190_e14135_d_n0;
        locals.var_tmf2_dn2 = assign19190_e14135_d_n2;
        locals.var_tmf2_dn4 = assign19190_e14135_d_n4;
        locals.var_tmf2_dn5 = assign19190_e14135_d_n5;
        locals.var_tmf2_dn6 = assign19190_e14135_d_n6;
        locals.var_tmf2_dn7 = assign19190_e14135_d_n7;
        locals.var_tmf2_dn8 = assign19190_e14135_d_n8;
        locals.var_tmf2_dn9 = assign19190_e14135_d_n9;
        locals.var_tmf2_dn10 = assign19190_e14135_d_n10;
        locals.var_tmf2_dn11 = assign19190_e14135_d_n11;
        locals.var_tmf2_dn14 = assign19190_e14135_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19200_e14149, assign19200_e14149_d_n0, assign19200_e14149_d_n2, assign19200_e14149_d_n4, assign19200_e14149_d_n5, assign19200_e14149_d_n6, assign19200_e14149_d_n7, assign19200_e14149_d_n8, assign19200_e14149_d_n9, assign19200_e14149_d_n10, assign19200_e14149_d_n11, assign19200_e14149_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let (assign19200_e14147, assign19200_e14147_d_n0, assign19200_e14147_d_n2, assign19200_e14147_d_n4, assign19200_e14147_d_n5, assign19200_e14147_d_n6, assign19200_e14147_d_n7, assign19200_e14147_d_n8, assign19200_e14147_d_n9, assign19200_e14147_d_n10, assign19200_e14147_d_n11, assign19200_e14147_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign19200_e14146: f64 = (-locals.var_tmf2);
                (assign19200_e14146, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign19200_e14147, assign19200_e14147_d_n0, assign19200_e14147_d_n2, assign19200_e14147_d_n4, assign19200_e14147_d_n5, assign19200_e14147_d_n6, assign19200_e14147_d_n7, assign19200_e14147_d_n8, assign19200_e14147_d_n9, assign19200_e14147_d_n10, assign19200_e14147_d_n11, assign19200_e14147_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19200_e14149;
        locals.var_tmf2_dn0 = assign19200_e14149_d_n0;
        locals.var_tmf2_dn2 = assign19200_e14149_d_n2;
        locals.var_tmf2_dn4 = assign19200_e14149_d_n4;
        locals.var_tmf2_dn5 = assign19200_e14149_d_n5;
        locals.var_tmf2_dn6 = assign19200_e14149_d_n6;
        locals.var_tmf2_dn7 = assign19200_e14149_d_n7;
        locals.var_tmf2_dn8 = assign19200_e14149_d_n8;
        locals.var_tmf2_dn9 = assign19200_e14149_d_n9;
        locals.var_tmf2_dn10 = assign19200_e14149_d_n10;
        locals.var_tmf2_dn11 = assign19200_e14149_d_n11;
        locals.var_tmf2_dn14 = assign19200_e14149_d_n14;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_48(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign19210_e14162, assign19210_e14162_d_n0, assign19210_e14162_d_n2, assign19210_e14162_d_n4, assign19210_e14162_d_n5, assign19210_e14162_d_n6, assign19210_e14162_d_n7, assign19210_e14162_d_n8, assign19210_e14162_d_n9, assign19210_e14162_d_n10, assign19210_e14162_d_n11, assign19210_e14162_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign19210_e14157: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19210_e14159: f64 = (assign19210_e14157 + locals.var_tmf2);
        let assign19210_e14160: f64 = (assign19210_e14159).sqrt();
        (assign19210_e14160, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19210_e14160)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19210_e14160)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19210_e14160)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19210_e14160)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19210_e14160)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19210_e14160)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19210_e14160)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19210_e14160)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19210_e14160)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19210_e14160)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19210_e14160)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19210_e14162;
        locals.var_tmf2_dn0 = assign19210_e14162_d_n0;
        locals.var_tmf2_dn2 = assign19210_e14162_d_n2;
        locals.var_tmf2_dn4 = assign19210_e14162_d_n4;
        locals.var_tmf2_dn5 = assign19210_e14162_d_n5;
        locals.var_tmf2_dn6 = assign19210_e14162_d_n6;
        locals.var_tmf2_dn7 = assign19210_e14162_d_n7;
        locals.var_tmf2_dn8 = assign19210_e14162_d_n8;
        locals.var_tmf2_dn9 = assign19210_e14162_d_n9;
        locals.var_tmf2_dn10 = assign19210_e14162_d_n10;
        locals.var_tmf2_dn11 = assign19210_e14162_d_n11;
        locals.var_tmf2_dn14 = assign19210_e14162_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19220_e14176, assign19220_e14176_d_n0, assign19220_e14176_d_n2, assign19220_e14176_d_n4, assign19220_e14176_d_n5, assign19220_e14176_d_n6, assign19220_e14176_d_n7, assign19220_e14176_d_n8, assign19220_e14176_d_n9, assign19220_e14176_d_n10, assign19220_e14176_d_n11, assign19220_e14176_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign19220_e14172: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19220_e14173: f64 = (1.0 + assign19220_e14172);
        let assign19220_e14174: f64 = (0.5 * assign19220_e14173);
        (assign19220_e14174, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign19220_e14176;
        locals.var_t6_dn0 = assign19220_e14176_d_n0;
        locals.var_t6_dn2 = assign19220_e14176_d_n2;
        locals.var_t6_dn4 = assign19220_e14176_d_n4;
        locals.var_t6_dn5 = assign19220_e14176_d_n5;
        locals.var_t6_dn6 = assign19220_e14176_d_n6;
        locals.var_t6_dn7 = assign19220_e14176_d_n7;
        locals.var_t6_dn8 = assign19220_e14176_d_n8;
        locals.var_t6_dn9 = assign19220_e14176_d_n9;
        locals.var_t6_dn10 = assign19220_e14176_d_n10;
        locals.var_t6_dn11 = assign19220_e14176_d_n11;
        locals.var_t6_dn14 = assign19220_e14176_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign19230_e14194, assign19230_e14194_d_n0, assign19230_e14194_d_n2, assign19230_e14194_d_n4, assign19230_e14194_d_n5, assign19230_e14194_d_n6, assign19230_e14194_d_n7, assign19230_e14194_d_n8, assign19230_e14194_d_n9, assign19230_e14194_d_n10, assign19230_e14194_d_n11, assign19230_e14194_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign19230_e14185: f64 = (p.p98 + 1.0);
        let assign19230_e14186: f64 = (locals.var_t4 * assign19230_e14185);
        let assign19230_e14190: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19230_e14191: f64 = (0.5 * assign19230_e14190);
        let assign19230_e14192: f64 = (assign19230_e14186 - assign19230_e14191);
        (assign19230_e14192, ((locals.var_t4_dn0 * assign19230_e14185) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((locals.var_t4_dn2 * assign19230_e14185) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((locals.var_t4_dn4 * assign19230_e14185) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((locals.var_t4_dn5 * assign19230_e14185) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((locals.var_t4_dn6 * assign19230_e14185) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((locals.var_t4_dn7 * assign19230_e14185) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((locals.var_t4_dn8 * assign19230_e14185) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((locals.var_t4_dn9 * assign19230_e14185) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((locals.var_t4_dn10 * assign19230_e14185) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((locals.var_t4_dn11 * assign19230_e14185) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((locals.var_t4_dn14 * assign19230_e14185) - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign19230_e14194;
        locals.var_t7_dn0 = assign19230_e14194_d_n0;
        locals.var_t7_dn2 = assign19230_e14194_d_n2;
        locals.var_t7_dn4 = assign19230_e14194_d_n4;
        locals.var_t7_dn5 = assign19230_e14194_d_n5;
        locals.var_t7_dn6 = assign19230_e14194_d_n6;
        locals.var_t7_dn7 = assign19230_e14194_d_n7;
        locals.var_t7_dn8 = assign19230_e14194_d_n8;
        locals.var_t7_dn9 = assign19230_e14194_d_n9;
        locals.var_t7_dn10 = assign19230_e14194_d_n10;
        locals.var_t7_dn11 = assign19230_e14194_d_n11;
        locals.var_t7_dn14 = assign19230_e14194_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign19240_e14210, assign19240_e14210_d_n0, assign19240_e14210_d_n2, assign19240_e14210_d_n4, assign19240_e14210_d_n5, assign19240_e14210_d_n6, assign19240_e14210_d_n7, assign19240_e14210_d_n8, assign19240_e14210_d_n9, assign19240_e14210_d_n10, assign19240_e14210_d_n11, assign19240_e14210_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign19240_e14203: f64 = (locals.var_t1 * locals.var_t4);
        let assign19240_e14204: f64 = (locals.var_t7 + assign19240_e14203);
        let assign19240_e14206: f64 = assign19240_e14204;
        let assign19240_e14208: f64 = (assign19240_e14206 - 5e-5);
        (assign19240_e14208, (locals.var_t7_dn0 + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))), (locals.var_t7_dn2 + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))), (locals.var_t7_dn4 + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))), (locals.var_t7_dn5 + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))), (locals.var_t7_dn6 + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))), (locals.var_t7_dn7 + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))), (locals.var_t7_dn8 + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))), (locals.var_t7_dn9 + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))), (locals.var_t7_dn10 + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))), (locals.var_t7_dn11 + ((locals.var_t1_dn11 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn11))), (locals.var_t7_dn14 + ((locals.var_t1_dn14 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn14))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign19240_e14210;
        locals.var_tmf1_dn0 = assign19240_e14210_d_n0;
        locals.var_tmf1_dn2 = assign19240_e14210_d_n2;
        locals.var_tmf1_dn4 = assign19240_e14210_d_n4;
        locals.var_tmf1_dn5 = assign19240_e14210_d_n5;
        locals.var_tmf1_dn6 = assign19240_e14210_d_n6;
        locals.var_tmf1_dn7 = assign19240_e14210_d_n7;
        locals.var_tmf1_dn8 = assign19240_e14210_d_n8;
        locals.var_tmf1_dn9 = assign19240_e14210_d_n9;
        locals.var_tmf1_dn10 = assign19240_e14210_d_n10;
        locals.var_tmf1_dn11 = assign19240_e14210_d_n11;
        locals.var_tmf1_dn14 = assign19240_e14210_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign19250_e14222, assign19250_e14222_d_n0, assign19250_e14222_d_n2, assign19250_e14222_d_n4, assign19250_e14222_d_n5, assign19250_e14222_d_n6, assign19250_e14222_d_n7, assign19250_e14222_d_n8, assign19250_e14222_d_n9, assign19250_e14222_d_n10, assign19250_e14222_d_n11, assign19250_e14222_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19250_e14222;
        locals.var_tmf2_dn0 = assign19250_e14222_d_n0;
        locals.var_tmf2_dn2 = assign19250_e14222_d_n2;
        locals.var_tmf2_dn4 = assign19250_e14222_d_n4;
        locals.var_tmf2_dn5 = assign19250_e14222_d_n5;
        locals.var_tmf2_dn6 = assign19250_e14222_d_n6;
        locals.var_tmf2_dn7 = assign19250_e14222_d_n7;
        locals.var_tmf2_dn8 = assign19250_e14222_d_n8;
        locals.var_tmf2_dn9 = assign19250_e14222_d_n9;
        locals.var_tmf2_dn10 = assign19250_e14222_d_n10;
        locals.var_tmf2_dn11 = assign19250_e14222_d_n11;
        locals.var_tmf2_dn14 = assign19250_e14222_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19260_e14236, assign19260_e14236_d_n0, assign19260_e14236_d_n2, assign19260_e14236_d_n4, assign19260_e14236_d_n5, assign19260_e14236_d_n6, assign19260_e14236_d_n7, assign19260_e14236_d_n8, assign19260_e14236_d_n9, assign19260_e14236_d_n10, assign19260_e14236_d_n11, assign19260_e14236_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let (assign19260_e14234, assign19260_e14234_d_n0, assign19260_e14234_d_n2, assign19260_e14234_d_n4, assign19260_e14234_d_n5, assign19260_e14234_d_n6, assign19260_e14234_d_n7, assign19260_e14234_d_n8, assign19260_e14234_d_n9, assign19260_e14234_d_n10, assign19260_e14234_d_n11, assign19260_e14234_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign19260_e14233: f64 = (-locals.var_tmf2);
                (assign19260_e14233, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign19260_e14234, assign19260_e14234_d_n0, assign19260_e14234_d_n2, assign19260_e14234_d_n4, assign19260_e14234_d_n5, assign19260_e14234_d_n6, assign19260_e14234_d_n7, assign19260_e14234_d_n8, assign19260_e14234_d_n9, assign19260_e14234_d_n10, assign19260_e14234_d_n11, assign19260_e14234_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19260_e14236;
        locals.var_tmf2_dn0 = assign19260_e14236_d_n0;
        locals.var_tmf2_dn2 = assign19260_e14236_d_n2;
        locals.var_tmf2_dn4 = assign19260_e14236_d_n4;
        locals.var_tmf2_dn5 = assign19260_e14236_d_n5;
        locals.var_tmf2_dn6 = assign19260_e14236_d_n6;
        locals.var_tmf2_dn7 = assign19260_e14236_d_n7;
        locals.var_tmf2_dn8 = assign19260_e14236_d_n8;
        locals.var_tmf2_dn9 = assign19260_e14236_d_n9;
        locals.var_tmf2_dn10 = assign19260_e14236_d_n10;
        locals.var_tmf2_dn11 = assign19260_e14236_d_n11;
        locals.var_tmf2_dn14 = assign19260_e14236_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19270_e14249, assign19270_e14249_d_n0, assign19270_e14249_d_n2, assign19270_e14249_d_n4, assign19270_e14249_d_n5, assign19270_e14249_d_n6, assign19270_e14249_d_n7, assign19270_e14249_d_n8, assign19270_e14249_d_n9, assign19270_e14249_d_n10, assign19270_e14249_d_n11, assign19270_e14249_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign19270_e14244: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19270_e14246: f64 = (assign19270_e14244 + locals.var_tmf2);
        let assign19270_e14247: f64 = (assign19270_e14246).sqrt();
        (assign19270_e14247, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19270_e14247)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19270_e14247)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19270_e14247)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19270_e14247)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19270_e14247)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19270_e14247)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19270_e14247)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19270_e14247)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19270_e14247)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19270_e14247)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19270_e14247)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19270_e14249;
        locals.var_tmf2_dn0 = assign19270_e14249_d_n0;
        locals.var_tmf2_dn2 = assign19270_e14249_d_n2;
        locals.var_tmf2_dn4 = assign19270_e14249_d_n4;
        locals.var_tmf2_dn5 = assign19270_e14249_d_n5;
        locals.var_tmf2_dn6 = assign19270_e14249_d_n6;
        locals.var_tmf2_dn7 = assign19270_e14249_d_n7;
        locals.var_tmf2_dn8 = assign19270_e14249_d_n8;
        locals.var_tmf2_dn9 = assign19270_e14249_d_n9;
        locals.var_tmf2_dn10 = assign19270_e14249_d_n10;
        locals.var_tmf2_dn11 = assign19270_e14249_d_n11;
        locals.var_tmf2_dn14 = assign19270_e14249_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19280_e14263, assign19280_e14263_d_n0, assign19280_e14263_d_n2, assign19280_e14263_d_n4, assign19280_e14263_d_n5, assign19280_e14263_d_n6, assign19280_e14263_d_n7, assign19280_e14263_d_n8, assign19280_e14263_d_n9, assign19280_e14263_d_n10, assign19280_e14263_d_n11, assign19280_e14263_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign19280_e14259: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19280_e14260: f64 = (1.0 + assign19280_e14259);
        let assign19280_e14261: f64 = (0.5 * assign19280_e14260);
        (assign19280_e14261, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign19280_e14263;
        locals.var_t6_dn0 = assign19280_e14263_d_n0;
        locals.var_t6_dn2 = assign19280_e14263_d_n2;
        locals.var_t6_dn4 = assign19280_e14263_d_n4;
        locals.var_t6_dn5 = assign19280_e14263_d_n5;
        locals.var_t6_dn6 = assign19280_e14263_d_n6;
        locals.var_t6_dn7 = assign19280_e14263_d_n7;
        locals.var_t6_dn8 = assign19280_e14263_d_n8;
        locals.var_t6_dn9 = assign19280_e14263_d_n9;
        locals.var_t6_dn10 = assign19280_e14263_d_n10;
        locals.var_t6_dn11 = assign19280_e14263_d_n11;
        locals.var_t6_dn14 = assign19280_e14263_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign19290_e14277, assign19290_e14277_d_n0, assign19290_e14277_d_n2, assign19290_e14277_d_n4, assign19290_e14277_d_n5, assign19290_e14277_d_n6, assign19290_e14277_d_n7, assign19290_e14277_d_n8, assign19290_e14277_d_n9, assign19290_e14277_d_n10, assign19290_e14277_d_n11, assign19290_e14277_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) {
        let assign19290_e14273: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19290_e14274: f64 = (0.5 * assign19290_e14273);
        let assign19290_e14275: f64 = assign19290_e14274;
        (assign19290_e14275, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign19290_e14277;
        locals.var_t2_dn0 = assign19290_e14277_d_n0;
        locals.var_t2_dn2 = assign19290_e14277_d_n2;
        locals.var_t2_dn4 = assign19290_e14277_d_n4;
        locals.var_t2_dn5 = assign19290_e14277_d_n5;
        locals.var_t2_dn6 = assign19290_e14277_d_n6;
        locals.var_t2_dn7 = assign19290_e14277_d_n7;
        locals.var_t2_dn8 = assign19290_e14277_d_n8;
        locals.var_t2_dn9 = assign19290_e14277_d_n9;
        locals.var_t2_dn10 = assign19290_e14277_d_n10;
        locals.var_t2_dn11 = assign19290_e14277_d_n11;
        locals.var_t2_dn14 = assign19290_e14277_d_n14;
        locals.var_t2_rv = 0.0;

        let assign19300_e14284: f64 = if ((p.p39 == 0.0) || (p.p39 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard388 = assign19300_e14284;
        locals.var_guard388_rv = 0.0;

        let (assign19310_e14304, assign19310_e14304_d_n0, assign19310_e14304_d_n2, assign19310_e14304_d_n4, assign19310_e14304_d_n5, assign19310_e14304_d_n6, assign19310_e14304_d_n7, assign19310_e14304_d_n8, assign19310_e14304_d_n9, assign19310_e14304_d_n10, assign19310_e14304_d_n11, assign19310_e14304_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign19310_e14295: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff0);
        let assign19310_e14296: f64 = (locals.var_uc_rdvd + assign19310_e14295);
        let assign19310_e14299: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2);
        let assign19310_e14300: f64 = (assign19310_e14296 + assign19310_e14299);
        let assign19310_e14302: f64 = (assign19310_e14300 * locals.var_t2);
        (assign19310_e14302, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign19310_e14300 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign19310_e14300 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign19310_e14300 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign19310_e14300 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign19310_e14300 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign19310_e14300 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign19310_e14300 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign19310_e14300 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign19310_e14300 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn11) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn11)) * locals.var_t2) + (assign19310_e14300 * locals.var_t2_dn11)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn14) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn14)) * locals.var_t2) + (assign19310_e14300 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign19310_e14304;
        locals.var_rsvde_dn0 = assign19310_e14304_d_n0;
        locals.var_rsvde_dn2 = assign19310_e14304_d_n2;
        locals.var_rsvde_dn4 = assign19310_e14304_d_n4;
        locals.var_rsvde_dn5 = assign19310_e14304_d_n5;
        locals.var_rsvde_dn6 = assign19310_e14304_d_n6;
        locals.var_rsvde_dn7 = assign19310_e14304_d_n7;
        locals.var_rsvde_dn8 = assign19310_e14304_d_n8;
        locals.var_rsvde_dn9 = assign19310_e14304_d_n9;
        locals.var_rsvde_dn10 = assign19310_e14304_d_n10;
        locals.var_rsvde_dn11 = assign19310_e14304_d_n11;
        locals.var_rsvde_dn14 = assign19310_e14304_d_n14;
        locals.var_rsvde_rv = 0.0;

        let (assign19320_e14322, assign19320_e14322_d_n0, assign19320_e14322_d_n2, assign19320_e14322_d_n4, assign19320_e14322_d_n5, assign19320_e14322_d_n6, assign19320_e14322_d_n7, assign19320_e14322_d_n8, assign19320_e14322_d_n9, assign19320_e14322_d_n10, assign19320_e14322_d_n11, assign19320_e14322_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign19320_e14315: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19320_e14316: f64 = (locals.var_rsvde - assign19320_e14315);
        let assign19320_e14319: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19320_e14320: f64 = (assign19320_e14316 - assign19320_e14319);
        (assign19320_e14320, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign19320_e14322;
        locals.var_tmf1_dn0 = assign19320_e14322_d_n0;
        locals.var_tmf1_dn2 = assign19320_e14322_d_n2;
        locals.var_tmf1_dn4 = assign19320_e14322_d_n4;
        locals.var_tmf1_dn5 = assign19320_e14322_d_n5;
        locals.var_tmf1_dn6 = assign19320_e14322_d_n6;
        locals.var_tmf1_dn7 = assign19320_e14322_d_n7;
        locals.var_tmf1_dn8 = assign19320_e14322_d_n8;
        locals.var_tmf1_dn9 = assign19320_e14322_d_n9;
        locals.var_tmf1_dn10 = assign19320_e14322_d_n10;
        locals.var_tmf1_dn11 = assign19320_e14322_d_n11;
        locals.var_tmf1_dn14 = assign19320_e14322_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign19330_e14340, assign19330_e14340_d_n0, assign19330_e14340_d_n2, assign19330_e14340_d_n4, assign19330_e14340_d_n5, assign19330_e14340_d_n6, assign19330_e14340_d_n7, assign19330_e14340_d_n8, assign19330_e14340_d_n9, assign19330_e14340_d_n10, assign19330_e14340_d_n11, assign19330_e14340_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign19330_e14333: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19330_e14334: f64 = (4.0 * assign19330_e14333);
        let assign19330_e14337: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19330_e14338: f64 = (assign19330_e14334 * assign19330_e14337);
        (assign19330_e14338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19330_e14340;
        locals.var_tmf2_dn0 = assign19330_e14340_d_n0;
        locals.var_tmf2_dn2 = assign19330_e14340_d_n2;
        locals.var_tmf2_dn4 = assign19330_e14340_d_n4;
        locals.var_tmf2_dn5 = assign19330_e14340_d_n5;
        locals.var_tmf2_dn6 = assign19330_e14340_d_n6;
        locals.var_tmf2_dn7 = assign19330_e14340_d_n7;
        locals.var_tmf2_dn8 = assign19330_e14340_d_n8;
        locals.var_tmf2_dn9 = assign19330_e14340_d_n9;
        locals.var_tmf2_dn10 = assign19330_e14340_d_n10;
        locals.var_tmf2_dn11 = assign19330_e14340_d_n11;
        locals.var_tmf2_dn14 = assign19330_e14340_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19340_e14356, assign19340_e14356_d_n0, assign19340_e14356_d_n2, assign19340_e14356_d_n4, assign19340_e14356_d_n5, assign19340_e14356_d_n6, assign19340_e14356_d_n7, assign19340_e14356_d_n8, assign19340_e14356_d_n9, assign19340_e14356_d_n10, assign19340_e14356_d_n11, assign19340_e14356_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let (assign19340_e14354, assign19340_e14354_d_n0, assign19340_e14354_d_n2, assign19340_e14354_d_n4, assign19340_e14354_d_n5, assign19340_e14354_d_n6, assign19340_e14354_d_n7, assign19340_e14354_d_n8, assign19340_e14354_d_n9, assign19340_e14354_d_n10, assign19340_e14354_d_n11, assign19340_e14354_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign19340_e14353: f64 = (-locals.var_tmf2);
                (assign19340_e14353, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign19340_e14354, assign19340_e14354_d_n0, assign19340_e14354_d_n2, assign19340_e14354_d_n4, assign19340_e14354_d_n5, assign19340_e14354_d_n6, assign19340_e14354_d_n7, assign19340_e14354_d_n8, assign19340_e14354_d_n9, assign19340_e14354_d_n10, assign19340_e14354_d_n11, assign19340_e14354_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19340_e14356;
        locals.var_tmf2_dn0 = assign19340_e14356_d_n0;
        locals.var_tmf2_dn2 = assign19340_e14356_d_n2;
        locals.var_tmf2_dn4 = assign19340_e14356_d_n4;
        locals.var_tmf2_dn5 = assign19340_e14356_d_n5;
        locals.var_tmf2_dn6 = assign19340_e14356_d_n6;
        locals.var_tmf2_dn7 = assign19340_e14356_d_n7;
        locals.var_tmf2_dn8 = assign19340_e14356_d_n8;
        locals.var_tmf2_dn9 = assign19340_e14356_d_n9;
        locals.var_tmf2_dn10 = assign19340_e14356_d_n10;
        locals.var_tmf2_dn11 = assign19340_e14356_d_n11;
        locals.var_tmf2_dn14 = assign19340_e14356_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19350_e14371, assign19350_e14371_d_n0, assign19350_e14371_d_n2, assign19350_e14371_d_n4, assign19350_e14371_d_n5, assign19350_e14371_d_n6, assign19350_e14371_d_n7, assign19350_e14371_d_n8, assign19350_e14371_d_n9, assign19350_e14371_d_n10, assign19350_e14371_d_n11, assign19350_e14371_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign19350_e14366: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19350_e14368: f64 = (assign19350_e14366 + locals.var_tmf2);
        let assign19350_e14369: f64 = (assign19350_e14368).sqrt();
        (assign19350_e14369, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19350_e14369)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19350_e14369)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19350_e14369)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19350_e14369)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19350_e14369)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19350_e14369)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19350_e14369)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19350_e14369)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19350_e14369)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19350_e14369)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19350_e14369)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19350_e14371;
        locals.var_tmf2_dn0 = assign19350_e14371_d_n0;
        locals.var_tmf2_dn2 = assign19350_e14371_d_n2;
        locals.var_tmf2_dn4 = assign19350_e14371_d_n4;
        locals.var_tmf2_dn5 = assign19350_e14371_d_n5;
        locals.var_tmf2_dn6 = assign19350_e14371_d_n6;
        locals.var_tmf2_dn7 = assign19350_e14371_d_n7;
        locals.var_tmf2_dn8 = assign19350_e14371_d_n8;
        locals.var_tmf2_dn9 = assign19350_e14371_d_n9;
        locals.var_tmf2_dn10 = assign19350_e14371_d_n10;
        locals.var_tmf2_dn11 = assign19350_e14371_d_n11;
        locals.var_tmf2_dn14 = assign19350_e14371_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19360_e14387, assign19360_e14387_d_n0, assign19360_e14387_d_n2, assign19360_e14387_d_n4, assign19360_e14387_d_n5, assign19360_e14387_d_n6, assign19360_e14387_d_n7, assign19360_e14387_d_n8, assign19360_e14387_d_n9, assign19360_e14387_d_n10, assign19360_e14387_d_n11, assign19360_e14387_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign19360_e14383: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19360_e14384: f64 = (1.0 + assign19360_e14383);
        let assign19360_e14385: f64 = (0.5 * assign19360_e14384);
        (assign19360_e14385, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign19360_e14387;
        locals.var_t0_dn0 = assign19360_e14387_d_n0;
        locals.var_t0_dn2 = assign19360_e14387_d_n2;
        locals.var_t0_dn4 = assign19360_e14387_d_n4;
        locals.var_t0_dn5 = assign19360_e14387_d_n5;
        locals.var_t0_dn6 = assign19360_e14387_d_n6;
        locals.var_t0_dn7 = assign19360_e14387_d_n7;
        locals.var_t0_dn8 = assign19360_e14387_d_n8;
        locals.var_t0_dn9 = assign19360_e14387_d_n9;
        locals.var_t0_dn10 = assign19360_e14387_d_n10;
        locals.var_t0_dn11 = assign19360_e14387_d_n11;
        locals.var_t0_dn14 = assign19360_e14387_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign19370_e14405, assign19370_e14405_d_n0, assign19370_e14405_d_n2, assign19370_e14405_d_n4, assign19370_e14405_d_n5, assign19370_e14405_d_n6, assign19370_e14405_d_n7, assign19370_e14405_d_n8, assign19370_e14405_d_n9, assign19370_e14405_d_n10, assign19370_e14405_d_n11, assign19370_e14405_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 != 0.0)) {
        let assign19370_e14397: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19370_e14401: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19370_e14402: f64 = (0.5 * assign19370_e14401);
        let assign19370_e14403: f64 = (assign19370_e14397 + assign19370_e14402);
        (assign19370_e14403, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign19370_e14405;
        locals.var_rsvde_dn0 = assign19370_e14405_d_n0;
        locals.var_rsvde_dn2 = assign19370_e14405_d_n2;
        locals.var_rsvde_dn4 = assign19370_e14405_d_n4;
        locals.var_rsvde_dn5 = assign19370_e14405_d_n5;
        locals.var_rsvde_dn6 = assign19370_e14405_d_n6;
        locals.var_rsvde_dn7 = assign19370_e14405_d_n7;
        locals.var_rsvde_dn8 = assign19370_e14405_d_n8;
        locals.var_rsvde_dn9 = assign19370_e14405_d_n9;
        locals.var_rsvde_dn10 = assign19370_e14405_d_n10;
        locals.var_rsvde_dn11 = assign19370_e14405_d_n11;
        locals.var_rsvde_dn14 = assign19370_e14405_d_n14;
        locals.var_rsvde_rv = 0.0;

        let (assign19380_e14426, assign19380_e14426_d_n0, assign19380_e14426_d_n2, assign19380_e14426_d_n4, assign19380_e14426_d_n5, assign19380_e14426_d_n6, assign19380_e14426_d_n7, assign19380_e14426_d_n8, assign19380_e14426_d_n9, assign19380_e14426_d_n10, assign19380_e14426_d_n11, assign19380_e14426_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 == 0.0)) {
        let assign19380_e14417: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff);
        let assign19380_e14418: f64 = (locals.var_uc_rdvd + assign19380_e14417);
        let assign19380_e14421: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2);
        let assign19380_e14422: f64 = (assign19380_e14418 + assign19380_e14421);
        let assign19380_e14424: f64 = (assign19380_e14422 * locals.var_t2);
        (assign19380_e14424, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign19380_e14422 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign19380_e14422 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign19380_e14422 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign19380_e14422 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign19380_e14422 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign19380_e14422 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign19380_e14422 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign19380_e14422 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign19380_e14422 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn11) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn11)) * locals.var_t2) + (assign19380_e14422 * locals.var_t2_dn11)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn14) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn14)) * locals.var_t2) + (assign19380_e14422 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign19380_e14426;
        locals.var_rsvde_dn0 = assign19380_e14426_d_n0;
        locals.var_rsvde_dn2 = assign19380_e14426_d_n2;
        locals.var_rsvde_dn4 = assign19380_e14426_d_n4;
        locals.var_rsvde_dn5 = assign19380_e14426_d_n5;
        locals.var_rsvde_dn6 = assign19380_e14426_d_n6;
        locals.var_rsvde_dn7 = assign19380_e14426_d_n7;
        locals.var_rsvde_dn8 = assign19380_e14426_d_n8;
        locals.var_rsvde_dn9 = assign19380_e14426_d_n9;
        locals.var_rsvde_dn10 = assign19380_e14426_d_n10;
        locals.var_rsvde_dn11 = assign19380_e14426_d_n11;
        locals.var_rsvde_dn14 = assign19380_e14426_d_n14;
        locals.var_rsvde_rv = 0.0;

        let (assign19390_e14445, assign19390_e14445_d_n0, assign19390_e14445_d_n2, assign19390_e14445_d_n4, assign19390_e14445_d_n5, assign19390_e14445_d_n6, assign19390_e14445_d_n7, assign19390_e14445_d_n8, assign19390_e14445_d_n9, assign19390_e14445_d_n10, assign19390_e14445_d_n11, assign19390_e14445_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 == 0.0)) {
        let assign19390_e14438: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19390_e14439: f64 = (locals.var_rsvde - assign19390_e14438);
        let assign19390_e14442: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19390_e14443: f64 = (assign19390_e14439 - assign19390_e14442);
        (assign19390_e14443, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign19390_e14445;
        locals.var_tmf1_dn0 = assign19390_e14445_d_n0;
        locals.var_tmf1_dn2 = assign19390_e14445_d_n2;
        locals.var_tmf1_dn4 = assign19390_e14445_d_n4;
        locals.var_tmf1_dn5 = assign19390_e14445_d_n5;
        locals.var_tmf1_dn6 = assign19390_e14445_d_n6;
        locals.var_tmf1_dn7 = assign19390_e14445_d_n7;
        locals.var_tmf1_dn8 = assign19390_e14445_d_n8;
        locals.var_tmf1_dn9 = assign19390_e14445_d_n9;
        locals.var_tmf1_dn10 = assign19390_e14445_d_n10;
        locals.var_tmf1_dn11 = assign19390_e14445_d_n11;
        locals.var_tmf1_dn14 = assign19390_e14445_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign19400_e14464, assign19400_e14464_d_n0, assign19400_e14464_d_n2, assign19400_e14464_d_n4, assign19400_e14464_d_n5, assign19400_e14464_d_n6, assign19400_e14464_d_n7, assign19400_e14464_d_n8, assign19400_e14464_d_n9, assign19400_e14464_d_n10, assign19400_e14464_d_n11, assign19400_e14464_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 == 0.0)) {
        let assign19400_e14457: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19400_e14458: f64 = (4.0 * assign19400_e14457);
        let assign19400_e14461: f64 = (0.01 * locals.var_uc_rdvd);
        let assign19400_e14462: f64 = (assign19400_e14458 * assign19400_e14461);
        (assign19400_e14462, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19400_e14464;
        locals.var_tmf2_dn0 = assign19400_e14464_d_n0;
        locals.var_tmf2_dn2 = assign19400_e14464_d_n2;
        locals.var_tmf2_dn4 = assign19400_e14464_d_n4;
        locals.var_tmf2_dn5 = assign19400_e14464_d_n5;
        locals.var_tmf2_dn6 = assign19400_e14464_d_n6;
        locals.var_tmf2_dn7 = assign19400_e14464_d_n7;
        locals.var_tmf2_dn8 = assign19400_e14464_d_n8;
        locals.var_tmf2_dn9 = assign19400_e14464_d_n9;
        locals.var_tmf2_dn10 = assign19400_e14464_d_n10;
        locals.var_tmf2_dn11 = assign19400_e14464_d_n11;
        locals.var_tmf2_dn14 = assign19400_e14464_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19410_e14481, assign19410_e14481_d_n0, assign19410_e14481_d_n2, assign19410_e14481_d_n4, assign19410_e14481_d_n5, assign19410_e14481_d_n6, assign19410_e14481_d_n7, assign19410_e14481_d_n8, assign19410_e14481_d_n9, assign19410_e14481_d_n10, assign19410_e14481_d_n11, assign19410_e14481_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 == 0.0)) {
        let (assign19410_e14479, assign19410_e14479_d_n0, assign19410_e14479_d_n2, assign19410_e14479_d_n4, assign19410_e14479_d_n5, assign19410_e14479_d_n6, assign19410_e14479_d_n7, assign19410_e14479_d_n8, assign19410_e14479_d_n9, assign19410_e14479_d_n10, assign19410_e14479_d_n11, assign19410_e14479_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign19410_e14478: f64 = (-locals.var_tmf2);
                (assign19410_e14478, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign19410_e14479, assign19410_e14479_d_n0, assign19410_e14479_d_n2, assign19410_e14479_d_n4, assign19410_e14479_d_n5, assign19410_e14479_d_n6, assign19410_e14479_d_n7, assign19410_e14479_d_n8, assign19410_e14479_d_n9, assign19410_e14479_d_n10, assign19410_e14479_d_n11, assign19410_e14479_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19410_e14481;
        locals.var_tmf2_dn0 = assign19410_e14481_d_n0;
        locals.var_tmf2_dn2 = assign19410_e14481_d_n2;
        locals.var_tmf2_dn4 = assign19410_e14481_d_n4;
        locals.var_tmf2_dn5 = assign19410_e14481_d_n5;
        locals.var_tmf2_dn6 = assign19410_e14481_d_n6;
        locals.var_tmf2_dn7 = assign19410_e14481_d_n7;
        locals.var_tmf2_dn8 = assign19410_e14481_d_n8;
        locals.var_tmf2_dn9 = assign19410_e14481_d_n9;
        locals.var_tmf2_dn10 = assign19410_e14481_d_n10;
        locals.var_tmf2_dn11 = assign19410_e14481_d_n11;
        locals.var_tmf2_dn14 = assign19410_e14481_d_n14;
        locals.var_tmf2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_49(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign19420_e14497, assign19420_e14497_d_n0, assign19420_e14497_d_n2, assign19420_e14497_d_n4, assign19420_e14497_d_n5, assign19420_e14497_d_n6, assign19420_e14497_d_n7, assign19420_e14497_d_n8, assign19420_e14497_d_n9, assign19420_e14497_d_n10, assign19420_e14497_d_n11, assign19420_e14497_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 == 0.0)) {
        let assign19420_e14492: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19420_e14494: f64 = (assign19420_e14492 + locals.var_tmf2);
        let assign19420_e14495: f64 = (assign19420_e14494).sqrt();
        (assign19420_e14495, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19420_e14495)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19420_e14495)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19420_e14495)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19420_e14495)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19420_e14495)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19420_e14495)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19420_e14495)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19420_e14495)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19420_e14495)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19420_e14495)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19420_e14495)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign19420_e14497;
        locals.var_tmf2_dn0 = assign19420_e14497_d_n0;
        locals.var_tmf2_dn2 = assign19420_e14497_d_n2;
        locals.var_tmf2_dn4 = assign19420_e14497_d_n4;
        locals.var_tmf2_dn5 = assign19420_e14497_d_n5;
        locals.var_tmf2_dn6 = assign19420_e14497_d_n6;
        locals.var_tmf2_dn7 = assign19420_e14497_d_n7;
        locals.var_tmf2_dn8 = assign19420_e14497_d_n8;
        locals.var_tmf2_dn9 = assign19420_e14497_d_n9;
        locals.var_tmf2_dn10 = assign19420_e14497_d_n10;
        locals.var_tmf2_dn11 = assign19420_e14497_d_n11;
        locals.var_tmf2_dn14 = assign19420_e14497_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign19430_e14514, assign19430_e14514_d_n0, assign19430_e14514_d_n2, assign19430_e14514_d_n4, assign19430_e14514_d_n5, assign19430_e14514_d_n6, assign19430_e14514_d_n7, assign19430_e14514_d_n8, assign19430_e14514_d_n9, assign19430_e14514_d_n10, assign19430_e14514_d_n11, assign19430_e14514_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 == 0.0)) {
        let assign19430_e14510: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19430_e14511: f64 = (1.0 + assign19430_e14510);
        let assign19430_e14512: f64 = (0.5 * assign19430_e14511);
        (assign19430_e14512, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign19430_e14514;
        locals.var_t0_dn0 = assign19430_e14514_d_n0;
        locals.var_t0_dn2 = assign19430_e14514_d_n2;
        locals.var_t0_dn4 = assign19430_e14514_d_n4;
        locals.var_t0_dn5 = assign19430_e14514_d_n5;
        locals.var_t0_dn6 = assign19430_e14514_d_n6;
        locals.var_t0_dn7 = assign19430_e14514_d_n7;
        locals.var_t0_dn8 = assign19430_e14514_d_n8;
        locals.var_t0_dn9 = assign19430_e14514_d_n9;
        locals.var_t0_dn10 = assign19430_e14514_d_n10;
        locals.var_t0_dn11 = assign19430_e14514_d_n11;
        locals.var_t0_dn14 = assign19430_e14514_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign19440_e14533, assign19440_e14533_d_n0, assign19440_e14533_d_n2, assign19440_e14533_d_n4, assign19440_e14533_d_n5, assign19440_e14533_d_n6, assign19440_e14533_d_n7, assign19440_e14533_d_n8, assign19440_e14533_d_n9, assign19440_e14533_d_n10, assign19440_e14533_d_n11, assign19440_e14533_d_n14,) = {
    if ((((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 != 0.0)) && (locals.var_guard388 == 0.0)) {
        let assign19440_e14525: f64 = (0.005 * locals.var_uc_rdvd);
        let assign19440_e14529: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19440_e14530: f64 = (0.5 * assign19440_e14529);
        let assign19440_e14531: f64 = (assign19440_e14525 + assign19440_e14530);
        (assign19440_e14531, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign19440_e14533;
        locals.var_rsvde_dn0 = assign19440_e14533_d_n0;
        locals.var_rsvde_dn2 = assign19440_e14533_d_n2;
        locals.var_rsvde_dn4 = assign19440_e14533_d_n4;
        locals.var_rsvde_dn5 = assign19440_e14533_d_n5;
        locals.var_rsvde_dn6 = assign19440_e14533_d_n6;
        locals.var_rsvde_dn7 = assign19440_e14533_d_n7;
        locals.var_rsvde_dn8 = assign19440_e14533_d_n8;
        locals.var_rsvde_dn9 = assign19440_e14533_d_n9;
        locals.var_rsvde_dn10 = assign19440_e14533_d_n10;
        locals.var_rsvde_dn11 = assign19440_e14533_d_n11;
        locals.var_rsvde_dn14 = assign19440_e14533_d_n14;
        locals.var_rsvde_rv = 0.0;

        let (assign19450_e14542, assign19450_e14542_d_n0, assign19450_e14542_d_n2, assign19450_e14542_d_n4, assign19450_e14542_d_n5, assign19450_e14542_d_n6, assign19450_e14542_d_n7, assign19450_e14542_d_n8, assign19450_e14542_d_n9, assign19450_e14542_d_n10, assign19450_e14542_d_n11, assign19450_e14542_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign19450_e14542;
        locals.var_rdvde_dn0 = assign19450_e14542_d_n0;
        locals.var_rdvde_dn2 = assign19450_e14542_d_n2;
        locals.var_rdvde_dn4 = assign19450_e14542_d_n4;
        locals.var_rdvde_dn5 = assign19450_e14542_d_n5;
        locals.var_rdvde_dn6 = assign19450_e14542_d_n6;
        locals.var_rdvde_dn7 = assign19450_e14542_d_n7;
        locals.var_rdvde_dn8 = assign19450_e14542_d_n8;
        locals.var_rdvde_dn9 = assign19450_e14542_d_n9;
        locals.var_rdvde_dn10 = assign19450_e14542_d_n10;
        locals.var_rdvde_dn11 = assign19450_e14542_d_n11;
        locals.var_rdvde_dn14 = assign19450_e14542_d_n14;
        locals.var_rdvde_rv = 0.0;

        let (assign19460_e14551, assign19460_e14551_d_n0, assign19460_e14551_d_n2, assign19460_e14551_d_n4, assign19460_e14551_d_n5, assign19460_e14551_d_n6, assign19460_e14551_d_n7, assign19460_e14551_d_n8, assign19460_e14551_d_n9, assign19460_e14551_d_n10, assign19460_e14551_d_n11, assign19460_e14551_d_n14,) = {
    if (((locals.var_guard354 != 0.0) && (locals.var_guard380 != 0.0)) && (locals.var_guard385 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign19460_e14551;
        locals.var_rsvde_dn0 = assign19460_e14551_d_n0;
        locals.var_rsvde_dn2 = assign19460_e14551_d_n2;
        locals.var_rsvde_dn4 = assign19460_e14551_d_n4;
        locals.var_rsvde_dn5 = assign19460_e14551_d_n5;
        locals.var_rsvde_dn6 = assign19460_e14551_d_n6;
        locals.var_rsvde_dn7 = assign19460_e14551_d_n7;
        locals.var_rsvde_dn8 = assign19460_e14551_d_n8;
        locals.var_rsvde_dn9 = assign19460_e14551_d_n9;
        locals.var_rsvde_dn10 = assign19460_e14551_d_n10;
        locals.var_rsvde_dn11 = assign19460_e14551_d_n11;
        locals.var_rsvde_dn14 = assign19460_e14551_d_n14;
        locals.var_rsvde_rv = 0.0;

        let (assign19470_e14558, assign19470_e14558_d_n0, assign19470_e14558_d_n2, assign19470_e14558_d_n4, assign19470_e14558_d_n5, assign19470_e14558_d_n6, assign19470_e14558_d_n7, assign19470_e14558_d_n8, assign19470_e14558_d_n9, assign19470_e14558_d_n10, assign19470_e14558_d_n11, assign19470_e14558_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign19470_e14555: f64 = (locals.var_beta_inv).sqrt();
        let assign19470_e14556: f64 = (locals.var_costi00 * assign19470_e14555);
        (assign19470_e14556, (locals.var_costi00 * (locals.var_beta_inv_dn0 / (2.0 * assign19470_e14555))), (locals.var_costi00 * (locals.var_beta_inv_dn2 / (2.0 * assign19470_e14555))), (locals.var_costi00 * (locals.var_beta_inv_dn4 / (2.0 * assign19470_e14555))), (locals.var_costi00 * (locals.var_beta_inv_dn5 / (2.0 * assign19470_e14555))), (locals.var_costi00 * (locals.var_beta_inv_dn6 / (2.0 * assign19470_e14555))), (locals.var_costi00 * (locals.var_beta_inv_dn7 / (2.0 * assign19470_e14555))), (locals.var_costi00 * (locals.var_beta_inv_dn8 / (2.0 * assign19470_e14555))), (locals.var_costi00 * (locals.var_beta_inv_dn9 / (2.0 * assign19470_e14555))), (locals.var_costi00 * (locals.var_beta_inv_dn10 / (2.0 * assign19470_e14555))), (locals.var_costi00 * (locals.var_beta_inv_dn11 / (2.0 * assign19470_e14555))), (locals.var_costi00 * (locals.var_beta_inv_dn14 / (2.0 * assign19470_e14555))),)
    } else {
        (locals.var_costi0, locals.var_costi0_dn0, locals.var_costi0_dn2, locals.var_costi0_dn4, locals.var_costi0_dn5, locals.var_costi0_dn6, locals.var_costi0_dn7, locals.var_costi0_dn8, locals.var_costi0_dn9, locals.var_costi0_dn10, locals.var_costi0_dn11, locals.var_costi0_dn14,)
    }
};
        locals.var_costi0 = assign19470_e14558;
        locals.var_costi0_dn0 = assign19470_e14558_d_n0;
        locals.var_costi0_dn2 = assign19470_e14558_d_n2;
        locals.var_costi0_dn4 = assign19470_e14558_d_n4;
        locals.var_costi0_dn5 = assign19470_e14558_d_n5;
        locals.var_costi0_dn6 = assign19470_e14558_d_n6;
        locals.var_costi0_dn7 = assign19470_e14558_d_n7;
        locals.var_costi0_dn8 = assign19470_e14558_d_n8;
        locals.var_costi0_dn9 = assign19470_e14558_d_n9;
        locals.var_costi0_dn10 = assign19470_e14558_d_n10;
        locals.var_costi0_dn11 = assign19470_e14558_d_n11;
        locals.var_costi0_dn14 = assign19470_e14558_d_n14;
        locals.var_costi0_rv = 0.0;

        let (assign19480_e14564, assign19480_e14564_d_n0, assign19480_e14564_d_n2, assign19480_e14564_d_n4, assign19480_e14564_d_n5, assign19480_e14564_d_n6, assign19480_e14564_d_n7, assign19480_e14564_d_n8, assign19480_e14564_d_n9, assign19480_e14564_d_n10, assign19480_e14564_d_n11, assign19480_e14564_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign19480_e14562: f64 = (locals.var_costi0 * locals.var_costi0);
        (assign19480_e14562, ((locals.var_costi0_dn0 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn0)), ((locals.var_costi0_dn2 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn2)), ((locals.var_costi0_dn4 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn4)), ((locals.var_costi0_dn5 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn5)), ((locals.var_costi0_dn6 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn6)), ((locals.var_costi0_dn7 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn7)), ((locals.var_costi0_dn8 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn8)), ((locals.var_costi0_dn9 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn9)), ((locals.var_costi0_dn10 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn10)), ((locals.var_costi0_dn11 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn11)), ((locals.var_costi0_dn14 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn14)),)
    } else {
        (locals.var_costi0_p2, locals.var_costi0_p2_dn0, locals.var_costi0_p2_dn2, locals.var_costi0_p2_dn4, locals.var_costi0_p2_dn5, locals.var_costi0_p2_dn6, locals.var_costi0_p2_dn7, locals.var_costi0_p2_dn8, locals.var_costi0_p2_dn9, locals.var_costi0_p2_dn10, locals.var_costi0_p2_dn11, locals.var_costi0_p2_dn14,)
    }
};
        locals.var_costi0_p2 = assign19480_e14564;
        locals.var_costi0_p2_dn0 = assign19480_e14564_d_n0;
        locals.var_costi0_p2_dn2 = assign19480_e14564_d_n2;
        locals.var_costi0_p2_dn4 = assign19480_e14564_d_n4;
        locals.var_costi0_p2_dn5 = assign19480_e14564_d_n5;
        locals.var_costi0_p2_dn6 = assign19480_e14564_d_n6;
        locals.var_costi0_p2_dn7 = assign19480_e14564_d_n7;
        locals.var_costi0_p2_dn8 = assign19480_e14564_d_n8;
        locals.var_costi0_p2_dn9 = assign19480_e14564_d_n9;
        locals.var_costi0_p2_dn10 = assign19480_e14564_d_n10;
        locals.var_costi0_p2_dn11 = assign19480_e14564_d_n11;
        locals.var_costi0_p2_dn14 = assign19480_e14564_d_n14;
        locals.var_costi0_p2_rv = 0.0;

        let (assign19490_e14572, assign19490_e14572_d_n0, assign19490_e14572_d_n2, assign19490_e14572_d_n4, assign19490_e14572_d_n5, assign19490_e14572_d_n6, assign19490_e14572_d_n7, assign19490_e14572_d_n8, assign19490_e14572_d_n9, assign19490_e14572_d_n10, assign19490_e14572_d_n11, assign19490_e14572_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign19490_e14568: f64 = (locals.var_nin * locals.var_nin);
        let assign19490_e14570: f64 = (assign19490_e14568 * locals.var_nsti_p2);
        (assign19490_e14570, (((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_nsti_p2), (((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_nsti_p2), (((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_nsti_p2), (((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_nsti_p2), (((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_nsti_p2), (((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_nsti_p2), (((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_nsti_p2), (((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_nsti_p2), (((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_nsti_p2), (((locals.var_nin_dn11 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn11)) * locals.var_nsti_p2), (((locals.var_nin_dn14 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn14)) * locals.var_nsti_p2),)
    } else {
        (locals.var_costi1, locals.var_costi1_dn0, locals.var_costi1_dn2, locals.var_costi1_dn4, locals.var_costi1_dn5, locals.var_costi1_dn6, locals.var_costi1_dn7, locals.var_costi1_dn8, locals.var_costi1_dn9, locals.var_costi1_dn10, locals.var_costi1_dn11, locals.var_costi1_dn14,)
    }
};
        locals.var_costi1 = assign19490_e14572;
        locals.var_costi1_dn0 = assign19490_e14572_d_n0;
        locals.var_costi1_dn2 = assign19490_e14572_d_n2;
        locals.var_costi1_dn4 = assign19490_e14572_d_n4;
        locals.var_costi1_dn5 = assign19490_e14572_d_n5;
        locals.var_costi1_dn6 = assign19490_e14572_d_n6;
        locals.var_costi1_dn7 = assign19490_e14572_d_n7;
        locals.var_costi1_dn8 = assign19490_e14572_d_n8;
        locals.var_costi1_dn9 = assign19490_e14572_d_n9;
        locals.var_costi1_dn10 = assign19490_e14572_d_n10;
        locals.var_costi1_dn11 = assign19490_e14572_d_n11;
        locals.var_costi1_dn14 = assign19490_e14572_d_n14;
        locals.var_costi1_rv = 0.0;

        let (assign19500_e14580, assign19500_e14580_d_n0, assign19500_e14580_d_n2, assign19500_e14580_d_n4, assign19500_e14580_d_n5, assign19500_e14580_d_n6, assign19500_e14580_d_n7, assign19500_e14580_d_n8, assign19500_e14580_d_n9, assign19500_e14580_d_n10, assign19500_e14580_d_n11, assign19500_e14580_d_n14,) = {
    if (locals.var_guard354 != 0.0) {
        let assign19500_e14577: f64 = (p.p448 * locals.var_tdiff);
        let assign19500_e14578: f64 = (p.p447 + assign19500_e14577);
        (assign19500_e14578, (p.p448 * locals.var_tdiff_dn0), (p.p448 * locals.var_tdiff_dn2), (p.p448 * locals.var_tdiff_dn4), (p.p448 * locals.var_tdiff_dn5), (p.p448 * locals.var_tdiff_dn6), (p.p448 * locals.var_tdiff_dn7), (p.p448 * locals.var_tdiff_dn8), (p.p448 * locals.var_tdiff_dn9), (p.p448 * locals.var_tdiff_dn10), (p.p448 * locals.var_tdiff_dn11), (p.p448 * locals.var_tdiff_dn14),)
    } else {
        (locals.var_hbdceff, locals.var_hbdceff_dn0, locals.var_hbdceff_dn2, locals.var_hbdceff_dn4, locals.var_hbdceff_dn5, locals.var_hbdceff_dn6, locals.var_hbdceff_dn7, locals.var_hbdceff_dn8, locals.var_hbdceff_dn9, locals.var_hbdceff_dn10, locals.var_hbdceff_dn11, locals.var_hbdceff_dn14,)
    }
};
        locals.var_hbdceff = assign19500_e14580;
        locals.var_hbdceff_dn0 = assign19500_e14580_d_n0;
        locals.var_hbdceff_dn2 = assign19500_e14580_d_n2;
        locals.var_hbdceff_dn4 = assign19500_e14580_d_n4;
        locals.var_hbdceff_dn5 = assign19500_e14580_d_n5;
        locals.var_hbdceff_dn6 = assign19500_e14580_d_n6;
        locals.var_hbdceff_dn7 = assign19500_e14580_d_n7;
        locals.var_hbdceff_dn8 = assign19500_e14580_d_n8;
        locals.var_hbdceff_dn9 = assign19500_e14580_d_n9;
        locals.var_hbdceff_dn10 = assign19500_e14580_d_n10;
        locals.var_hbdceff_dn11 = assign19500_e14580_d_n11;
        locals.var_hbdceff_dn14 = assign19500_e14580_d_n14;
        locals.var_hbdceff_rv = 0.0;

        let (assign19510_e14584,) = {
    if (locals.var_guard354 != 0.0) {
        (p.p193,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign19510_e14584;
        locals.var_uc_subtmp_rv = 0.0;

        let assign19540_e14597: f64 = if locals.var_uc_subtmp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard391 = assign19540_e14597;
        locals.var_guard391_rv = 0.0;

        let (assign19550_e14603,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard391 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign19550_e14603;
        locals.var_uc_subtmp_rv = 0.0;

        let assign19560_e14606: f64 = if locals.var_uc_subtmp > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard392 = assign19560_e14606;
        locals.var_guard392_rv = 0.0;

        let (assign19570_e14612,) = {
    if ((locals.var_guard354 != 0.0) && (locals.var_guard392 != 0.0)) {
        (0.005,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign19570_e14612;
        locals.var_uc_subtmp_rv = 0.0;

        let (assign19580_e14619, assign19580_e14619_d_n0, assign19580_e14619_d_n2, assign19580_e14619_d_n4, assign19580_e14619_d_n5, assign19580_e14619_d_n6, assign19580_e14619_d_n7, assign19580_e14619_d_n8, assign19580_e14619_d_n9, assign19580_e14619_d_n10, assign19580_e14619_d_n11, assign19580_e14619_d_n14,) = {
    if (locals.var_guard354 == 0.0) {
        let assign19580_e14615: f64 = ctx_temp;
        let assign19580_e14617: f64 = (assign19580_e14615 + p.p11);
        (assign19580_e14617, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign19580_e14619;
        locals.var_ttemp_dn0 = assign19580_e14619_d_n0;
        locals.var_ttemp_dn2 = assign19580_e14619_d_n2;
        locals.var_ttemp_dn4 = assign19580_e14619_d_n4;
        locals.var_ttemp_dn5 = assign19580_e14619_d_n5;
        locals.var_ttemp_dn6 = assign19580_e14619_d_n6;
        locals.var_ttemp_dn7 = assign19580_e14619_d_n7;
        locals.var_ttemp_dn8 = assign19580_e14619_d_n8;
        locals.var_ttemp_dn9 = assign19580_e14619_d_n9;
        locals.var_ttemp_dn10 = assign19580_e14619_d_n10;
        locals.var_ttemp_dn11 = assign19580_e14619_d_n11;
        locals.var_ttemp_dn14 = assign19580_e14619_d_n14;
        locals.var_ttemp_rv = 0.0;

        let assign19590_e14622: f64 = (locals.var_weff_ld * p.p7);
        locals.var_weffld_nf = assign19590_e14622;
        locals.var_weffld_nf_rv = 0.0;

        let assign19600_e14625: f64 = (p.p67 + p.p68);
        locals.var_ldrift0 = assign19600_e14625;
        locals.var_ldrift0_rv = 0.0;

        locals.var_vfb = locals.var_uc_vfbc;
        locals.var_vfb_rv = 0.0;

        locals.var_vmaxe = locals.var_vmaxeff;
        locals.var_vmaxe_dn0 = locals.var_vmaxeff_dn0;
        locals.var_vmaxe_dn2 = locals.var_vmaxeff_dn2;
        locals.var_vmaxe_dn4 = locals.var_vmaxeff_dn4;
        locals.var_vmaxe_dn5 = locals.var_vmaxeff_dn5;
        locals.var_vmaxe_dn6 = locals.var_vmaxeff_dn6;
        locals.var_vmaxe_dn7 = locals.var_vmaxeff_dn7;
        locals.var_vmaxe_dn8 = locals.var_vmaxeff_dn8;
        locals.var_vmaxe_dn9 = locals.var_vmaxeff_dn9;
        locals.var_vmaxe_dn10 = locals.var_vmaxeff_dn10;
        locals.var_vmaxe_dn11 = locals.var_vmaxeff_dn11;
        locals.var_vmaxe_dn14 = locals.var_vmaxeff_dn14;
        locals.var_vmaxe_rv = 0.0;

        locals.var_c_eox = locals.var_cecox;
        locals.var_c_eox_rv = 0.0;

        locals.var_tox0 = p.p95;
        locals.var_tox0_rv = 0.0;

        let assign19650_e14632: f64 = (locals.var_c_eox / locals.var_tox0);
        locals.var_cox0 = assign19650_e14632;
        locals.var_cox0_rv = 0.0;

        let assign19660_e14635: f64 = (1.0 / locals.var_cox0);
        locals.var_cox0_inv = assign19660_e14635;
        locals.var_cox0_inv_rv = 0.0;

        let assign19670_e14638: f64 = (locals.var_c_eox / locals.var_uc_toxb);
        locals.var_coxb0 = assign19670_e14638;
        locals.var_coxb0_rv = 0.0;

        let assign19680_e14641: f64 = (p.p87 * p.p434);
        locals.var_vgs_min = assign19680_e14641;
        locals.var_vgs_min_rv = 0.0;

        let assign19690_e14645: f64 = (locals.var_pb2 - p.p262);
        let assign19690_e14646: f64 = (0.8 - assign19690_e14645);
        let assign19690_e14648: f64 = (assign19690_e14646 - 0.1);
        locals.var_tmf1 = assign19690_e14648;
        locals.var_tmf1_dn0 = (-locals.var_pb2_dn0);
        locals.var_tmf1_dn2 = (-locals.var_pb2_dn2);
        locals.var_tmf1_dn4 = (-locals.var_pb2_dn4);
        locals.var_tmf1_dn5 = (-locals.var_pb2_dn5);
        locals.var_tmf1_dn6 = (-locals.var_pb2_dn6);
        locals.var_tmf1_dn7 = (-locals.var_pb2_dn7);
        locals.var_tmf1_dn8 = (-locals.var_pb2_dn8);
        locals.var_tmf1_dn9 = (-locals.var_pb2_dn9);
        locals.var_tmf1_dn10 = (-locals.var_pb2_dn10);
        locals.var_tmf1_dn11 = (-locals.var_pb2_dn11);
        locals.var_tmf1_dn14 = (-locals.var_pb2_dn14);
        locals.var_tmf1_rv = 0.0;

        let assign19700_e14651: f64 = (4.0 * 0.8);
        let assign19700_e14653: f64 = (assign19700_e14651 * 0.1);
        locals.var_tmf2 = assign19700_e14653;
        locals.var_tmf2_dn0 = 0.0;
        locals.var_tmf2_dn2 = 0.0;
        locals.var_tmf2_dn4 = 0.0;
        locals.var_tmf2_dn5 = 0.0;
        locals.var_tmf2_dn6 = 0.0;
        locals.var_tmf2_dn7 = 0.0;
        locals.var_tmf2_dn8 = 0.0;
        locals.var_tmf2_dn9 = 0.0;
        locals.var_tmf2_dn10 = 0.0;
        locals.var_tmf2_dn11 = 0.0;
        locals.var_tmf2_dn14 = 0.0;
        locals.var_tmf2_rv = 0.0;

        let (assign19710_e14660, assign19710_e14660_d_n0, assign19710_e14660_d_n2, assign19710_e14660_d_n4, assign19710_e14660_d_n5, assign19710_e14660_d_n6, assign19710_e14660_d_n7, assign19710_e14660_d_n8, assign19710_e14660_d_n9, assign19710_e14660_d_n10, assign19710_e14660_d_n11, assign19710_e14660_d_n14,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    } else {
        let assign19710_e14659: f64 = (-locals.var_tmf2);
        (assign19710_e14659, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
    }
};
        locals.var_tmf2 = assign19710_e14660;
        locals.var_tmf2_dn0 = assign19710_e14660_d_n0;
        locals.var_tmf2_dn2 = assign19710_e14660_d_n2;
        locals.var_tmf2_dn4 = assign19710_e14660_d_n4;
        locals.var_tmf2_dn5 = assign19710_e14660_d_n5;
        locals.var_tmf2_dn6 = assign19710_e14660_d_n6;
        locals.var_tmf2_dn7 = assign19710_e14660_d_n7;
        locals.var_tmf2_dn8 = assign19710_e14660_d_n8;
        locals.var_tmf2_dn9 = assign19710_e14660_d_n9;
        locals.var_tmf2_dn10 = assign19710_e14660_d_n10;
        locals.var_tmf2_dn11 = assign19710_e14660_d_n11;
        locals.var_tmf2_dn14 = assign19710_e14660_d_n14;
        locals.var_tmf2_rv = 0.0;

        let assign19720_e14663: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign19720_e14665: f64 = (assign19720_e14663 + locals.var_tmf2);
        let assign19720_e14666: f64 = (assign19720_e14665).sqrt();
        locals.var_tmf2 = assign19720_e14666;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign19720_e14666));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign19720_e14666));
        locals.var_tmf2_dn4 = ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign19720_e14666));
        locals.var_tmf2_dn5 = ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign19720_e14666));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign19720_e14666));
        locals.var_tmf2_dn7 = ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign19720_e14666));
        locals.var_tmf2_dn8 = ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign19720_e14666));
        locals.var_tmf2_dn9 = ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign19720_e14666));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign19720_e14666));
        locals.var_tmf2_dn11 = ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign19720_e14666));
        locals.var_tmf2_dn14 = ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign19720_e14666));
        locals.var_tmf2_rv = 0.0;

        let assign19730_e14671: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign19730_e14672: f64 = (1.0 + assign19730_e14671);
        let assign19730_e14673: f64 = (0.5 * assign19730_e14672);
        locals.var_t0 = assign19730_e14673;
        locals.var_t0_dn0 = (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn2 = (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn4 = (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn5 = (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn6 = (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn7 = (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn8 = (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn9 = (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn10 = (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn11 = (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn14 = (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_rv = 0.0;

        let assign19740_e14678: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign19740_e14679: f64 = (0.5 * assign19740_e14678);
        let assign19740_e14680: f64 = (0.8 - assign19740_e14679);
        locals.var_t1 = assign19740_e14680;
        locals.var_t1_dn0 = (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)));
        locals.var_t1_dn2 = (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)));
        locals.var_t1_dn4 = (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)));
        locals.var_t1_dn5 = (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)));
        locals.var_t1_dn6 = (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)));
        locals.var_t1_dn7 = (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)));
        locals.var_t1_dn8 = (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)));
        locals.var_t1_dn9 = (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)));
        locals.var_t1_dn10 = (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)));
        locals.var_t1_dn11 = (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)));
        locals.var_t1_dn14 = (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)));
        locals.var_t1_rv = 0.0;

        locals.var_vbs_max = locals.var_t1;
        locals.var_vbs_max_dn0 = locals.var_t1_dn0;
        locals.var_vbs_max_dn2 = locals.var_t1_dn2;
        locals.var_vbs_max_dn4 = locals.var_t1_dn4;
        locals.var_vbs_max_dn5 = locals.var_t1_dn5;
        locals.var_vbs_max_dn6 = locals.var_t1_dn6;
        locals.var_vbs_max_dn7 = locals.var_t1_dn7;
        locals.var_vbs_max_dn8 = locals.var_t1_dn8;
        locals.var_vbs_max_dn9 = locals.var_t1_dn9;
        locals.var_vbs_max_dn10 = locals.var_t1_dn10;
        locals.var_vbs_max_dn11 = locals.var_t1_dn11;
        locals.var_vbs_max_dn14 = locals.var_t1_dn14;
        locals.var_vbs_max_rv = 0.0;

        let assign19760_e14684: f64 = (locals.var_pb20 - p.p262);
        let assign19760_e14686: f64 = if assign19760_e14684 < locals.var_vbs_max { 1.0 } else { 0.0 };
        locals.var_guard393 = assign19760_e14686;
        locals.var_guard393_rv = 0.0;

        let (assign19770_e14692, assign19770_e14692_d_n0, assign19770_e14692_d_n2, assign19770_e14692_d_n4, assign19770_e14692_d_n5, assign19770_e14692_d_n6, assign19770_e14692_d_n7, assign19770_e14692_d_n8, assign19770_e14692_d_n9, assign19770_e14692_d_n10, assign19770_e14692_d_n11, assign19770_e14692_d_n14,) = {
    if (locals.var_guard393 != 0.0) {
        let assign19770_e14690: f64 = (locals.var_pb20 - p.p262);
        (assign19770_e14690, locals.var_pb20_dn0, locals.var_pb20_dn2, locals.var_pb20_dn4, locals.var_pb20_dn5, locals.var_pb20_dn6, locals.var_pb20_dn7, locals.var_pb20_dn8, locals.var_pb20_dn9, locals.var_pb20_dn10, locals.var_pb20_dn11, locals.var_pb20_dn14,)
    } else {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn11, locals.var_vbs_max_dn14,)
    }
};
        locals.var_vbs_max = assign19770_e14692;
        locals.var_vbs_max_dn0 = assign19770_e14692_d_n0;
        locals.var_vbs_max_dn2 = assign19770_e14692_d_n2;
        locals.var_vbs_max_dn4 = assign19770_e14692_d_n4;
        locals.var_vbs_max_dn5 = assign19770_e14692_d_n5;
        locals.var_vbs_max_dn6 = assign19770_e14692_d_n6;
        locals.var_vbs_max_dn7 = assign19770_e14692_d_n7;
        locals.var_vbs_max_dn8 = assign19770_e14692_d_n8;
        locals.var_vbs_max_dn9 = assign19770_e14692_d_n9;
        locals.var_vbs_max_dn10 = assign19770_e14692_d_n10;
        locals.var_vbs_max_dn11 = assign19770_e14692_d_n11;
        locals.var_vbs_max_dn14 = assign19770_e14692_d_n14;
        locals.var_vbs_max_rv = 0.0;

        let assign19780_e14695: f64 = (locals.var_pb2c - p.p262);
        let assign19780_e14697: f64 = if assign19780_e14695 < locals.var_vbs_max { 1.0 } else { 0.0 };
        locals.var_guard394 = assign19780_e14697;
        locals.var_guard394_rv = 0.0;

        let (assign19790_e14703, assign19790_e14703_d_n0, assign19790_e14703_d_n2, assign19790_e14703_d_n4, assign19790_e14703_d_n5, assign19790_e14703_d_n6, assign19790_e14703_d_n7, assign19790_e14703_d_n8, assign19790_e14703_d_n9, assign19790_e14703_d_n10, assign19790_e14703_d_n11, assign19790_e14703_d_n14,) = {
    if (locals.var_guard394 != 0.0) {
        let assign19790_e14701: f64 = (locals.var_pb2c - p.p262);
        (assign19790_e14701, locals.var_pb2c_dn0, locals.var_pb2c_dn2, locals.var_pb2c_dn4, locals.var_pb2c_dn5, locals.var_pb2c_dn6, locals.var_pb2c_dn7, locals.var_pb2c_dn8, locals.var_pb2c_dn9, locals.var_pb2c_dn10, locals.var_pb2c_dn11, locals.var_pb2c_dn14,)
    } else {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn11, locals.var_vbs_max_dn14,)
    }
};
        locals.var_vbs_max = assign19790_e14703;
        locals.var_vbs_max_dn0 = assign19790_e14703_d_n0;
        locals.var_vbs_max_dn2 = assign19790_e14703_d_n2;
        locals.var_vbs_max_dn4 = assign19790_e14703_d_n4;
        locals.var_vbs_max_dn5 = assign19790_e14703_d_n5;
        locals.var_vbs_max_dn6 = assign19790_e14703_d_n6;
        locals.var_vbs_max_dn7 = assign19790_e14703_d_n7;
        locals.var_vbs_max_dn8 = assign19790_e14703_d_n8;
        locals.var_vbs_max_dn9 = assign19790_e14703_d_n9;
        locals.var_vbs_max_dn10 = assign19790_e14703_d_n10;
        locals.var_vbs_max_dn11 = assign19790_e14703_d_n11;
        locals.var_vbs_max_dn14 = assign19790_e14703_d_n14;
        locals.var_vbs_max_rv = 0.0;

        let assign19800_e14710: f64 = if ((locals.var_uc_codep > 0.0) && (locals.var_uc_codep <= 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard395 = assign19800_e14710;
        locals.var_guard395_rv = 0.0;

        let assign19810_e14713: f64 = (locals.var_pb2n - p.p262);
        let assign19810_e14715: f64 = if assign19810_e14713 < locals.var_vbs_max { 1.0 } else { 0.0 };
        locals.var_guard396 = assign19810_e14715;
        locals.var_guard396_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_50(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign19820_e14723, assign19820_e14723_d_n0, assign19820_e14723_d_n2, assign19820_e14723_d_n4, assign19820_e14723_d_n5, assign19820_e14723_d_n6, assign19820_e14723_d_n7, assign19820_e14723_d_n8, assign19820_e14723_d_n9, assign19820_e14723_d_n10, assign19820_e14723_d_n11, assign19820_e14723_d_n14,) = {
    if ((locals.var_guard395 != 0.0) && (locals.var_guard396 != 0.0)) {
        let assign19820_e14721: f64 = (locals.var_pb2n - p.p262);
        (assign19820_e14721, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn11, locals.var_pb2n_dn14,)
    } else {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn11, locals.var_vbs_max_dn14,)
    }
};
        locals.var_vbs_max = assign19820_e14723;
        locals.var_vbs_max_dn0 = assign19820_e14723_d_n0;
        locals.var_vbs_max_dn2 = assign19820_e14723_d_n2;
        locals.var_vbs_max_dn4 = assign19820_e14723_d_n4;
        locals.var_vbs_max_dn5 = assign19820_e14723_d_n5;
        locals.var_vbs_max_dn6 = assign19820_e14723_d_n6;
        locals.var_vbs_max_dn7 = assign19820_e14723_d_n7;
        locals.var_vbs_max_dn8 = assign19820_e14723_d_n8;
        locals.var_vbs_max_dn9 = assign19820_e14723_d_n9;
        locals.var_vbs_max_dn10 = assign19820_e14723_d_n10;
        locals.var_vbs_max_dn11 = assign19820_e14723_d_n11;
        locals.var_vbs_max_dn14 = assign19820_e14723_d_n14;
        locals.var_vbs_max_rv = 0.0;

        let assign19830_e14726: f64 = (locals.var_vbipn - p.p262);
        let assign19830_e14728: f64 = if assign19830_e14726 < locals.var_vbs_max { 1.0 } else { 0.0 };
        locals.var_guard397 = assign19830_e14728;
        locals.var_guard397_rv = 0.0;

        let (assign19840_e14736, assign19840_e14736_d_n0, assign19840_e14736_d_n2, assign19840_e14736_d_n4, assign19840_e14736_d_n5, assign19840_e14736_d_n6, assign19840_e14736_d_n7, assign19840_e14736_d_n8, assign19840_e14736_d_n9, assign19840_e14736_d_n10, assign19840_e14736_d_n11, assign19840_e14736_d_n14,) = {
    if ((locals.var_guard395 != 0.0) && (locals.var_guard397 != 0.0)) {
        let assign19840_e14734: f64 = (locals.var_vbipn - p.p262);
        (assign19840_e14734, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn14,)
    } else {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn11, locals.var_vbs_max_dn14,)
    }
};
        locals.var_vbs_max = assign19840_e14736;
        locals.var_vbs_max_dn0 = assign19840_e14736_d_n0;
        locals.var_vbs_max_dn2 = assign19840_e14736_d_n2;
        locals.var_vbs_max_dn4 = assign19840_e14736_d_n4;
        locals.var_vbs_max_dn5 = assign19840_e14736_d_n5;
        locals.var_vbs_max_dn6 = assign19840_e14736_d_n6;
        locals.var_vbs_max_dn7 = assign19840_e14736_d_n7;
        locals.var_vbs_max_dn8 = assign19840_e14736_d_n8;
        locals.var_vbs_max_dn9 = assign19840_e14736_d_n9;
        locals.var_vbs_max_dn10 = assign19840_e14736_d_n10;
        locals.var_vbs_max_dn11 = assign19840_e14736_d_n11;
        locals.var_vbs_max_dn14 = assign19840_e14736_d_n14;
        locals.var_vbs_max_rv = 0.0;

        let assign19850_e14740: f64 = (locals.var_vbs_max * 0.5);
        let assign19850_e14741: f64 = if locals.var_vbs_bnd > assign19850_e14740 { 1.0 } else { 0.0 };
        locals.var_guard398 = assign19850_e14741;
        locals.var_guard398_rv = 0.0;

        let (assign19860_e14747, assign19860_e14747_d_n0, assign19860_e14747_d_n2, assign19860_e14747_d_n4, assign19860_e14747_d_n5, assign19860_e14747_d_n6, assign19860_e14747_d_n7, assign19860_e14747_d_n8, assign19860_e14747_d_n9, assign19860_e14747_d_n10, assign19860_e14747_d_n11, assign19860_e14747_d_n14,) = {
    if (locals.var_guard398 != 0.0) {
        let assign19860_e14745: f64 = (0.5 * locals.var_vbs_max);
        (assign19860_e14745, (0.5 * locals.var_vbs_max_dn0), (0.5 * locals.var_vbs_max_dn2), (0.5 * locals.var_vbs_max_dn4), (0.5 * locals.var_vbs_max_dn5), (0.5 * locals.var_vbs_max_dn6), (0.5 * locals.var_vbs_max_dn7), (0.5 * locals.var_vbs_max_dn8), (0.5 * locals.var_vbs_max_dn9), (0.5 * locals.var_vbs_max_dn10), (0.5 * locals.var_vbs_max_dn11), (0.5 * locals.var_vbs_max_dn14),)
    } else {
        (locals.var_vbs_bnd, locals.var_vbs_bnd_dn0, locals.var_vbs_bnd_dn2, locals.var_vbs_bnd_dn4, locals.var_vbs_bnd_dn5, locals.var_vbs_bnd_dn6, locals.var_vbs_bnd_dn7, locals.var_vbs_bnd_dn8, locals.var_vbs_bnd_dn9, locals.var_vbs_bnd_dn10, locals.var_vbs_bnd_dn11, locals.var_vbs_bnd_dn14,)
    }
};
        locals.var_vbs_bnd = assign19860_e14747;
        locals.var_vbs_bnd_dn0 = assign19860_e14747_d_n0;
        locals.var_vbs_bnd_dn2 = assign19860_e14747_d_n2;
        locals.var_vbs_bnd_dn4 = assign19860_e14747_d_n4;
        locals.var_vbs_bnd_dn5 = assign19860_e14747_d_n5;
        locals.var_vbs_bnd_dn6 = assign19860_e14747_d_n6;
        locals.var_vbs_bnd_dn7 = assign19860_e14747_d_n7;
        locals.var_vbs_bnd_dn8 = assign19860_e14747_d_n8;
        locals.var_vbs_bnd_dn9 = assign19860_e14747_d_n9;
        locals.var_vbs_bnd_dn10 = assign19860_e14747_d_n10;
        locals.var_vbs_bnd_dn11 = assign19860_e14747_d_n11;
        locals.var_vbs_bnd_dn14 = assign19860_e14747_d_n14;
        locals.var_vbs_bnd_rv = 0.0;

        let assign19870_e14749: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard399 = assign19870_e14749;
        locals.var_guard399_rv = 0.0;

        let (assign19880_e14753, assign19880_e14753_d_n0, assign19880_e14753_d_n2, assign19880_e14753_d_n4, assign19880_e14753_d_n5, assign19880_e14753_d_n6, assign19880_e14753_d_n7, assign19880_e14753_d_n8, assign19880_e14753_d_n9, assign19880_e14753_d_n10, assign19880_e14753_d_n11, assign19880_e14753_d_n14,) = {
    if (locals.var_guard399 != 0.0) {
        (p.p338, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_max_local, locals.var_vbs_max_local_dn0, locals.var_vbs_max_local_dn2, locals.var_vbs_max_local_dn4, locals.var_vbs_max_local_dn5, locals.var_vbs_max_local_dn6, locals.var_vbs_max_local_dn7, locals.var_vbs_max_local_dn8, locals.var_vbs_max_local_dn9, locals.var_vbs_max_local_dn10, locals.var_vbs_max_local_dn11, locals.var_vbs_max_local_dn14,)
    }
};
        locals.var_vbs_max_local = assign19880_e14753;
        locals.var_vbs_max_local_dn0 = assign19880_e14753_d_n0;
        locals.var_vbs_max_local_dn2 = assign19880_e14753_d_n2;
        locals.var_vbs_max_local_dn4 = assign19880_e14753_d_n4;
        locals.var_vbs_max_local_dn5 = assign19880_e14753_d_n5;
        locals.var_vbs_max_local_dn6 = assign19880_e14753_d_n6;
        locals.var_vbs_max_local_dn7 = assign19880_e14753_d_n7;
        locals.var_vbs_max_local_dn8 = assign19880_e14753_d_n8;
        locals.var_vbs_max_local_dn9 = assign19880_e14753_d_n9;
        locals.var_vbs_max_local_dn10 = assign19880_e14753_d_n10;
        locals.var_vbs_max_local_dn11 = assign19880_e14753_d_n11;
        locals.var_vbs_max_local_dn14 = assign19880_e14753_d_n14;
        locals.var_vbs_max_local_rv = 0.0;

        let (assign19890_e14758, assign19890_e14758_d_n0, assign19890_e14758_d_n2, assign19890_e14758_d_n4, assign19890_e14758_d_n5, assign19890_e14758_d_n6, assign19890_e14758_d_n7, assign19890_e14758_d_n8, assign19890_e14758_d_n9, assign19890_e14758_d_n10, assign19890_e14758_d_n11, assign19890_e14758_d_n14,) = {
    if (locals.var_guard399 == 0.0) {
        (locals.var_vbs_max, locals.var_vbs_max_dn0, locals.var_vbs_max_dn2, locals.var_vbs_max_dn4, locals.var_vbs_max_dn5, locals.var_vbs_max_dn6, locals.var_vbs_max_dn7, locals.var_vbs_max_dn8, locals.var_vbs_max_dn9, locals.var_vbs_max_dn10, locals.var_vbs_max_dn11, locals.var_vbs_max_dn14,)
    } else {
        (locals.var_vbs_max_local, locals.var_vbs_max_local_dn0, locals.var_vbs_max_local_dn2, locals.var_vbs_max_local_dn4, locals.var_vbs_max_local_dn5, locals.var_vbs_max_local_dn6, locals.var_vbs_max_local_dn7, locals.var_vbs_max_local_dn8, locals.var_vbs_max_local_dn9, locals.var_vbs_max_local_dn10, locals.var_vbs_max_local_dn11, locals.var_vbs_max_local_dn14,)
    }
};
        locals.var_vbs_max_local = assign19890_e14758;
        locals.var_vbs_max_local_dn0 = assign19890_e14758_d_n0;
        locals.var_vbs_max_local_dn2 = assign19890_e14758_d_n2;
        locals.var_vbs_max_local_dn4 = assign19890_e14758_d_n4;
        locals.var_vbs_max_local_dn5 = assign19890_e14758_d_n5;
        locals.var_vbs_max_local_dn6 = assign19890_e14758_d_n6;
        locals.var_vbs_max_local_dn7 = assign19890_e14758_d_n7;
        locals.var_vbs_max_local_dn8 = assign19890_e14758_d_n8;
        locals.var_vbs_max_local_dn9 = assign19890_e14758_d_n9;
        locals.var_vbs_max_local_dn10 = assign19890_e14758_d_n10;
        locals.var_vbs_max_local_dn11 = assign19890_e14758_d_n11;
        locals.var_vbs_max_local_dn14 = assign19890_e14758_d_n14;
        locals.var_vbs_max_local_rv = 0.0;

        let assign19900_e14760: f64 = if param_given[339] { 1.0 } else { 0.0 };
        locals.var_guard400 = assign19900_e14760;
        locals.var_guard400_rv = 0.0;

        let (assign19910_e14764, assign19910_e14764_d_n0, assign19910_e14764_d_n2, assign19910_e14764_d_n4, assign19910_e14764_d_n5, assign19910_e14764_d_n6, assign19910_e14764_d_n7, assign19910_e14764_d_n8, assign19910_e14764_d_n9, assign19910_e14764_d_n10, assign19910_e14764_d_n11, assign19910_e14764_d_n14,) = {
    if (locals.var_guard400 != 0.0) {
        (p.p339, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbs_bnd_local, locals.var_vbs_bnd_local_dn0, locals.var_vbs_bnd_local_dn2, locals.var_vbs_bnd_local_dn4, locals.var_vbs_bnd_local_dn5, locals.var_vbs_bnd_local_dn6, locals.var_vbs_bnd_local_dn7, locals.var_vbs_bnd_local_dn8, locals.var_vbs_bnd_local_dn9, locals.var_vbs_bnd_local_dn10, locals.var_vbs_bnd_local_dn11, locals.var_vbs_bnd_local_dn14,)
    }
};
        locals.var_vbs_bnd_local = assign19910_e14764;
        locals.var_vbs_bnd_local_dn0 = assign19910_e14764_d_n0;
        locals.var_vbs_bnd_local_dn2 = assign19910_e14764_d_n2;
        locals.var_vbs_bnd_local_dn4 = assign19910_e14764_d_n4;
        locals.var_vbs_bnd_local_dn5 = assign19910_e14764_d_n5;
        locals.var_vbs_bnd_local_dn6 = assign19910_e14764_d_n6;
        locals.var_vbs_bnd_local_dn7 = assign19910_e14764_d_n7;
        locals.var_vbs_bnd_local_dn8 = assign19910_e14764_d_n8;
        locals.var_vbs_bnd_local_dn9 = assign19910_e14764_d_n9;
        locals.var_vbs_bnd_local_dn10 = assign19910_e14764_d_n10;
        locals.var_vbs_bnd_local_dn11 = assign19910_e14764_d_n11;
        locals.var_vbs_bnd_local_dn14 = assign19910_e14764_d_n14;
        locals.var_vbs_bnd_local_rv = 0.0;

        let assign19920_e14766: f64 = if param_given[338] { 1.0 } else { 0.0 };
        locals.var_guard401 = assign19920_e14766;
        locals.var_guard401_rv = 0.0;

        let (assign19930_e14775, assign19930_e14775_d_n0, assign19930_e14775_d_n2, assign19930_e14775_d_n4, assign19930_e14775_d_n5, assign19930_e14775_d_n6, assign19930_e14775_d_n7, assign19930_e14775_d_n8, assign19930_e14775_d_n9, assign19930_e14775_d_n10, assign19930_e14775_d_n11, assign19930_e14775_d_n14,) = {
    if ((locals.var_guard400 == 0.0) && (locals.var_guard401 != 0.0)) {
        let assign19930_e14773: f64 = (0.5 * locals.var_vbs_max_local);
        (assign19930_e14773, (0.5 * locals.var_vbs_max_local_dn0), (0.5 * locals.var_vbs_max_local_dn2), (0.5 * locals.var_vbs_max_local_dn4), (0.5 * locals.var_vbs_max_local_dn5), (0.5 * locals.var_vbs_max_local_dn6), (0.5 * locals.var_vbs_max_local_dn7), (0.5 * locals.var_vbs_max_local_dn8), (0.5 * locals.var_vbs_max_local_dn9), (0.5 * locals.var_vbs_max_local_dn10), (0.5 * locals.var_vbs_max_local_dn11), (0.5 * locals.var_vbs_max_local_dn14),)
    } else {
        (locals.var_vbs_bnd_local, locals.var_vbs_bnd_local_dn0, locals.var_vbs_bnd_local_dn2, locals.var_vbs_bnd_local_dn4, locals.var_vbs_bnd_local_dn5, locals.var_vbs_bnd_local_dn6, locals.var_vbs_bnd_local_dn7, locals.var_vbs_bnd_local_dn8, locals.var_vbs_bnd_local_dn9, locals.var_vbs_bnd_local_dn10, locals.var_vbs_bnd_local_dn11, locals.var_vbs_bnd_local_dn14,)
    }
};
        locals.var_vbs_bnd_local = assign19930_e14775;
        locals.var_vbs_bnd_local_dn0 = assign19930_e14775_d_n0;
        locals.var_vbs_bnd_local_dn2 = assign19930_e14775_d_n2;
        locals.var_vbs_bnd_local_dn4 = assign19930_e14775_d_n4;
        locals.var_vbs_bnd_local_dn5 = assign19930_e14775_d_n5;
        locals.var_vbs_bnd_local_dn6 = assign19930_e14775_d_n6;
        locals.var_vbs_bnd_local_dn7 = assign19930_e14775_d_n7;
        locals.var_vbs_bnd_local_dn8 = assign19930_e14775_d_n8;
        locals.var_vbs_bnd_local_dn9 = assign19930_e14775_d_n9;
        locals.var_vbs_bnd_local_dn10 = assign19930_e14775_d_n10;
        locals.var_vbs_bnd_local_dn11 = assign19930_e14775_d_n11;
        locals.var_vbs_bnd_local_dn14 = assign19930_e14775_d_n14;
        locals.var_vbs_bnd_local_rv = 0.0;

        let (assign19940_e14783, assign19940_e14783_d_n0, assign19940_e14783_d_n2, assign19940_e14783_d_n4, assign19940_e14783_d_n5, assign19940_e14783_d_n6, assign19940_e14783_d_n7, assign19940_e14783_d_n8, assign19940_e14783_d_n9, assign19940_e14783_d_n10, assign19940_e14783_d_n11, assign19940_e14783_d_n14,) = {
    if ((locals.var_guard400 == 0.0) && (locals.var_guard401 == 0.0)) {
        (locals.var_vbs_bnd, locals.var_vbs_bnd_dn0, locals.var_vbs_bnd_dn2, locals.var_vbs_bnd_dn4, locals.var_vbs_bnd_dn5, locals.var_vbs_bnd_dn6, locals.var_vbs_bnd_dn7, locals.var_vbs_bnd_dn8, locals.var_vbs_bnd_dn9, locals.var_vbs_bnd_dn10, locals.var_vbs_bnd_dn11, locals.var_vbs_bnd_dn14,)
    } else {
        (locals.var_vbs_bnd_local, locals.var_vbs_bnd_local_dn0, locals.var_vbs_bnd_local_dn2, locals.var_vbs_bnd_local_dn4, locals.var_vbs_bnd_local_dn5, locals.var_vbs_bnd_local_dn6, locals.var_vbs_bnd_local_dn7, locals.var_vbs_bnd_local_dn8, locals.var_vbs_bnd_local_dn9, locals.var_vbs_bnd_local_dn10, locals.var_vbs_bnd_local_dn11, locals.var_vbs_bnd_local_dn14,)
    }
};
        locals.var_vbs_bnd_local = assign19940_e14783;
        locals.var_vbs_bnd_local_dn0 = assign19940_e14783_d_n0;
        locals.var_vbs_bnd_local_dn2 = assign19940_e14783_d_n2;
        locals.var_vbs_bnd_local_dn4 = assign19940_e14783_d_n4;
        locals.var_vbs_bnd_local_dn5 = assign19940_e14783_d_n5;
        locals.var_vbs_bnd_local_dn6 = assign19940_e14783_d_n6;
        locals.var_vbs_bnd_local_dn7 = assign19940_e14783_d_n7;
        locals.var_vbs_bnd_local_dn8 = assign19940_e14783_d_n8;
        locals.var_vbs_bnd_local_dn9 = assign19940_e14783_d_n9;
        locals.var_vbs_bnd_local_dn10 = assign19940_e14783_d_n10;
        locals.var_vbs_bnd_local_dn11 = assign19940_e14783_d_n11;
        locals.var_vbs_bnd_local_dn14 = assign19940_e14783_d_n14;
        locals.var_vbs_bnd_local_rv = 0.0;

        let assign19950_e14787: f64 = (locals.var_vbs_max_local * 0.5);
        let assign19950_e14788: f64 = if locals.var_vbs_bnd_local > assign19950_e14787 { 1.0 } else { 0.0 };
        locals.var_guard402 = assign19950_e14788;
        locals.var_guard402_rv = 0.0;

        let (assign19960_e14794, assign19960_e14794_d_n0, assign19960_e14794_d_n2, assign19960_e14794_d_n4, assign19960_e14794_d_n5, assign19960_e14794_d_n6, assign19960_e14794_d_n7, assign19960_e14794_d_n8, assign19960_e14794_d_n9, assign19960_e14794_d_n10, assign19960_e14794_d_n11, assign19960_e14794_d_n14,) = {
    if (locals.var_guard402 != 0.0) {
        let assign19960_e14792: f64 = (0.5 * locals.var_vbs_max_local);
        (assign19960_e14792, (0.5 * locals.var_vbs_max_local_dn0), (0.5 * locals.var_vbs_max_local_dn2), (0.5 * locals.var_vbs_max_local_dn4), (0.5 * locals.var_vbs_max_local_dn5), (0.5 * locals.var_vbs_max_local_dn6), (0.5 * locals.var_vbs_max_local_dn7), (0.5 * locals.var_vbs_max_local_dn8), (0.5 * locals.var_vbs_max_local_dn9), (0.5 * locals.var_vbs_max_local_dn10), (0.5 * locals.var_vbs_max_local_dn11), (0.5 * locals.var_vbs_max_local_dn14),)
    } else {
        (locals.var_vbs_bnd_local, locals.var_vbs_bnd_local_dn0, locals.var_vbs_bnd_local_dn2, locals.var_vbs_bnd_local_dn4, locals.var_vbs_bnd_local_dn5, locals.var_vbs_bnd_local_dn6, locals.var_vbs_bnd_local_dn7, locals.var_vbs_bnd_local_dn8, locals.var_vbs_bnd_local_dn9, locals.var_vbs_bnd_local_dn10, locals.var_vbs_bnd_local_dn11, locals.var_vbs_bnd_local_dn14,)
    }
};
        locals.var_vbs_bnd_local = assign19960_e14794;
        locals.var_vbs_bnd_local_dn0 = assign19960_e14794_d_n0;
        locals.var_vbs_bnd_local_dn2 = assign19960_e14794_d_n2;
        locals.var_vbs_bnd_local_dn4 = assign19960_e14794_d_n4;
        locals.var_vbs_bnd_local_dn5 = assign19960_e14794_d_n5;
        locals.var_vbs_bnd_local_dn6 = assign19960_e14794_d_n6;
        locals.var_vbs_bnd_local_dn7 = assign19960_e14794_d_n7;
        locals.var_vbs_bnd_local_dn8 = assign19960_e14794_d_n8;
        locals.var_vbs_bnd_local_dn9 = assign19960_e14794_d_n9;
        locals.var_vbs_bnd_local_dn10 = assign19960_e14794_d_n10;
        locals.var_vbs_bnd_local_dn11 = assign19960_e14794_d_n11;
        locals.var_vbs_bnd_local_dn14 = assign19960_e14794_d_n14;
        locals.var_vbs_bnd_local_rv = 0.0;

        let assign19970_e14801: f64 = if ((locals.var_rse > 0.0) || (locals.var_rde > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard403 = assign19970_e14801;
        locals.var_guard403_rv = 0.0;

        let assign19980_e14804: f64 = if locals.var_uc_corsrd == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard404 = assign19980_e14804;
        locals.var_guard404_rv = 0.0;

        let (assign19990_e14810,) = {
    if ((locals.var_guard403 != 0.0) && (locals.var_guard404 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_flg_rsrd,)
    }
};
        locals.var_flg_rsrd = assign19990_e14810;
        locals.var_flg_rsrd_rv = 0.0;

        let assign20000_e14813: f64 = if locals.var_uc_corsrd == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard405 = assign20000_e14813;
        locals.var_guard405_rv = 0.0;

        let (assign20010_e14819,) = {
    if ((locals.var_guard403 != 0.0) && (locals.var_guard405 != 0.0)) {
        (2.0,)
    } else {
        (locals.var_flg_rsrd,)
    }
};
        locals.var_flg_rsrd = assign20010_e14819;
        locals.var_flg_rsrd_rv = 0.0;

        let assign20020_e14822: f64 = if locals.var_uc_corsrd == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard406 = assign20020_e14822;
        locals.var_guard406_rv = 0.0;

        let (assign20030_e14828,) = {
    if ((locals.var_guard403 != 0.0) && (locals.var_guard406 != 0.0)) {
        (3.0,)
    } else {
        (locals.var_flg_rsrd,)
    }
};
        locals.var_flg_rsrd = assign20030_e14828;
        locals.var_flg_rsrd_rv = 0.0;

        locals.var_flg_pprv = 0.0;
        locals.var_flg_pprv_rv = 0.0;

        let assign20050_e14840: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign20050_e14841: f64 = (locals.var_uc_nover * assign20050_e14840);
        let assign20050_e14844: f64 = if (((locals.var_uc_cordrift == 1.0) && (p.p54 == 1.0)) && (assign20050_e14841 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard407 = assign20050_e14844;
        locals.var_guard407_rv = 0.0;

        let (assign20060_e14848, assign20060_e14848_d_n0, assign20060_e14848_d_n2,) = {
    if (locals.var_guard407 != 0.0) {
        (locals.var_vdsei, locals.var_vdsei_dn0, locals.var_vdsei_dn2,)
    } else {
        (locals.var_vdsegmt, locals.var_vdsegmt_dn0, locals.var_vdsegmt_dn2,)
    }
};
        locals.var_vdsegmt = assign20060_e14848;
        locals.var_vdsegmt_dn0 = assign20060_e14848_d_n0;
        locals.var_vdsegmt_dn2 = assign20060_e14848_d_n2;
        locals.var_vdsegmt_rv = 0.0;

        let assign20070_e14851: f64 = if locals.var_vdsegmt >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard408 = assign20070_e14851;
        locals.var_guard408_rv = 0.0;

        let (assign20080_e14857, assign20080_e14857_d_n0, assign20080_e14857_d_n2,) = {
    if ((locals.var_guard407 != 0.0) && (locals.var_guard408 != 0.0)) {
        (locals.var_vdsegmt, locals.var_vdsegmt_dn0, locals.var_vdsegmt_dn2,)
    } else {
        (locals.var_vdserev, locals.var_vdserev_dn0, locals.var_vdserev_dn2,)
    }
};
        locals.var_vdserev = assign20080_e14857;
        locals.var_vdserev_dn0 = assign20080_e14857_d_n0;
        locals.var_vdserev_dn2 = assign20080_e14857_d_n2;
        locals.var_vdserev_rv = 0.0;

        let (assign20090_e14863, assign20090_e14863_d_n0, assign20090_e14863_d_n2, assign20090_e14863_d_n4,) = {
    if ((locals.var_guard407 != 0.0) && (locals.var_guard408 != 0.0)) {
        (locals.var_vsubs, 0.0, locals.var_vsubs_dn2, locals.var_vsubs_dn4,)
    } else {
        (locals.var_vsubsrev, locals.var_vsubsrev_dn0, locals.var_vsubsrev_dn2, locals.var_vsubsrev_dn4,)
    }
};
        locals.var_vsubsrev = assign20090_e14863;
        locals.var_vsubsrev_dn0 = assign20090_e14863_d_n0;
        locals.var_vsubsrev_dn2 = assign20090_e14863_d_n2;
        locals.var_vsubsrev_dn4 = assign20090_e14863_d_n4;
        locals.var_vsubsrev_rv = 0.0;

        let (assign20100_e14871, assign20100_e14871_d_n0, assign20100_e14871_d_n2,) = {
    if ((locals.var_guard407 != 0.0) && (locals.var_guard408 == 0.0)) {
        let assign20100_e14869: f64 = (-locals.var_vdsegmt);
        (assign20100_e14869, (-locals.var_vdsegmt_dn0), (-locals.var_vdsegmt_dn2),)
    } else {
        (locals.var_vdserev, locals.var_vdserev_dn0, locals.var_vdserev_dn2,)
    }
};
        locals.var_vdserev = assign20100_e14871;
        locals.var_vdserev_dn0 = assign20100_e14871_d_n0;
        locals.var_vdserev_dn2 = assign20100_e14871_d_n2;
        locals.var_vdserev_rv = 0.0;

        let (assign20110_e14880, assign20110_e14880_d_n0, assign20110_e14880_d_n2, assign20110_e14880_d_n4,) = {
    if ((locals.var_guard407 != 0.0) && (locals.var_guard408 == 0.0)) {
        let assign20110_e14878: f64 = (locals.var_vsubs - locals.var_vdsegmt);
        (assign20110_e14878, (-locals.var_vdsegmt_dn0), (locals.var_vsubs_dn2 - locals.var_vdsegmt_dn2), locals.var_vsubs_dn4,)
    } else {
        (locals.var_vsubsrev, locals.var_vsubsrev_dn0, locals.var_vsubsrev_dn2, locals.var_vsubsrev_dn4,)
    }
};
        locals.var_vsubsrev = assign20110_e14880;
        locals.var_vsubsrev_dn0 = assign20110_e14880_d_n0;
        locals.var_vsubsrev_dn2 = assign20110_e14880_d_n2;
        locals.var_vsubsrev_dn4 = assign20110_e14880_d_n4;
        locals.var_vsubsrev_rv = 0.0;

        let (assign20120_e14890, assign20120_e14890_d_n0, assign20120_e14890_d_n2, assign20120_e14890_d_n4, assign20120_e14890_d_n5, assign20120_e14890_d_n6, assign20120_e14890_d_n7, assign20120_e14890_d_n8, assign20120_e14890_d_n9, assign20120_e14890_d_n10, assign20120_e14890_d_n11, assign20120_e14890_d_n14,) = {
    if (locals.var_guard407 != 0.0) {
        let assign20120_e14885: f64 = (locals.var_vdserev / 2.0);
        let assign20120_e14886: f64 = (2.0 * assign20120_e14885);
        let assign20120_e14888: f64 = (assign20120_e14886 / p.p262);
        (assign20120_e14888, ((2.0 * (locals.var_vdserev_dn0 / 2.0)) / p.p262), ((2.0 * (locals.var_vdserev_dn2 / 2.0)) / p.p262), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign20120_e14890;
        locals.var_tmf1_dn0 = assign20120_e14890_d_n0;
        locals.var_tmf1_dn2 = assign20120_e14890_d_n2;
        locals.var_tmf1_dn4 = assign20120_e14890_d_n4;
        locals.var_tmf1_dn5 = assign20120_e14890_d_n5;
        locals.var_tmf1_dn6 = assign20120_e14890_d_n6;
        locals.var_tmf1_dn7 = assign20120_e14890_d_n7;
        locals.var_tmf1_dn8 = assign20120_e14890_d_n8;
        locals.var_tmf1_dn9 = assign20120_e14890_d_n9;
        locals.var_tmf1_dn10 = assign20120_e14890_d_n10;
        locals.var_tmf1_dn11 = assign20120_e14890_d_n11;
        locals.var_tmf1_dn14 = assign20120_e14890_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign20130_e14930, assign20130_e14930_d_n0, assign20130_e14930_d_n2, assign20130_e14930_d_n4, assign20130_e14930_d_n5, assign20130_e14930_d_n6, assign20130_e14930_d_n7, assign20130_e14930_d_n8, assign20130_e14930_d_n9, assign20130_e14930_d_n10, assign20130_e14930_d_n11, assign20130_e14930_d_n14,) = {
    if (locals.var_guard407 != 0.0) {
        let assign20130_e14896: f64 = (1.0 / 2.0);
        let assign20130_e14900: f64 = (1.0 / 6.0);
        let assign20130_e14904: f64 = (1.0 / 24.0);
        let assign20130_e14908: f64 = (1.0 / 120.0);
        let assign20130_e14912: f64 = (1.0 / 720.0);
        let assign20130_e14916: f64 = (1.0 / 5040.0);
        let assign20130_e14917: f64 = (locals.var_tmf1 * assign20130_e14916);
        let assign20130_e14918: f64 = (assign20130_e14912 + assign20130_e14917);
        let assign20130_e14919: f64 = (locals.var_tmf1 * assign20130_e14918);
        let assign20130_e14920: f64 = (assign20130_e14908 + assign20130_e14919);
        let assign20130_e14921: f64 = (locals.var_tmf1 * assign20130_e14920);
        let assign20130_e14922: f64 = (assign20130_e14904 + assign20130_e14921);
        let assign20130_e14923: f64 = (locals.var_tmf1 * assign20130_e14922);
        let assign20130_e14924: f64 = (assign20130_e14900 + assign20130_e14923);
        let assign20130_e14925: f64 = (locals.var_tmf1 * assign20130_e14924);
        let assign20130_e14926: f64 = (assign20130_e14896 + assign20130_e14925);
        let assign20130_e14927: f64 = (locals.var_tmf1 * assign20130_e14926);
        let assign20130_e14928: f64 = (1.0 + assign20130_e14927);
        (assign20130_e14928, ((locals.var_tmf1_dn0 * assign20130_e14926) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20130_e14924) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20130_e14922) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20130_e14920) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20130_e14918) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign20130_e14916))))))))))), ((locals.var_tmf1_dn2 * assign20130_e14926) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20130_e14924) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20130_e14922) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20130_e14920) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20130_e14918) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign20130_e14916))))))))))), ((locals.var_tmf1_dn4 * assign20130_e14926) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20130_e14924) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20130_e14922) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20130_e14920) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20130_e14918) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign20130_e14916))))))))))), ((locals.var_tmf1_dn5 * assign20130_e14926) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20130_e14924) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20130_e14922) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20130_e14920) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20130_e14918) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign20130_e14916))))))))))), ((locals.var_tmf1_dn6 * assign20130_e14926) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20130_e14924) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20130_e14922) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20130_e14920) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20130_e14918) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign20130_e14916))))))))))), ((locals.var_tmf1_dn7 * assign20130_e14926) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20130_e14924) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20130_e14922) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20130_e14920) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20130_e14918) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign20130_e14916))))))))))), ((locals.var_tmf1_dn8 * assign20130_e14926) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20130_e14924) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20130_e14922) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20130_e14920) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20130_e14918) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign20130_e14916))))))))))), ((locals.var_tmf1_dn9 * assign20130_e14926) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20130_e14924) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20130_e14922) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20130_e14920) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20130_e14918) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign20130_e14916))))))))))), ((locals.var_tmf1_dn10 * assign20130_e14926) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20130_e14924) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20130_e14922) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20130_e14920) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20130_e14918) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign20130_e14916))))))))))), ((locals.var_tmf1_dn11 * assign20130_e14926) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20130_e14924) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20130_e14922) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20130_e14920) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20130_e14918) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign20130_e14916))))))))))), ((locals.var_tmf1_dn14 * assign20130_e14926) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20130_e14924) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20130_e14922) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20130_e14920) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20130_e14918) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign20130_e14916))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20130_e14930;
        locals.var_tmf2_dn0 = assign20130_e14930_d_n0;
        locals.var_tmf2_dn2 = assign20130_e14930_d_n2;
        locals.var_tmf2_dn4 = assign20130_e14930_d_n4;
        locals.var_tmf2_dn5 = assign20130_e14930_d_n5;
        locals.var_tmf2_dn6 = assign20130_e14930_d_n6;
        locals.var_tmf2_dn7 = assign20130_e14930_d_n7;
        locals.var_tmf2_dn8 = assign20130_e14930_d_n8;
        locals.var_tmf2_dn9 = assign20130_e14930_d_n9;
        locals.var_tmf2_dn10 = assign20130_e14930_d_n10;
        locals.var_tmf2_dn11 = assign20130_e14930_d_n11;
        locals.var_tmf2_dn14 = assign20130_e14930_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20140_e14966, assign20140_e14966_d_n0, assign20140_e14966_d_n2, assign20140_e14966_d_n4, assign20140_e14966_d_n5, assign20140_e14966_d_n6, assign20140_e14966_d_n7, assign20140_e14966_d_n8, assign20140_e14966_d_n9, assign20140_e14966_d_n10, assign20140_e14966_d_n11, assign20140_e14966_d_n14,) = {
    if (locals.var_guard407 != 0.0) {
        let assign20140_e14934: f64 = (1.0 / 2.0);
        let assign20140_e14938: f64 = (1.0 / 3.0);
        let assign20140_e14942: f64 = (1.0 / 8.0);
        let assign20140_e14946: f64 = (1.0 / 30.0);
        let assign20140_e14950: f64 = (1.0 / 144.0);
        let assign20140_e14954: f64 = (1.0 / 840.0);
        let assign20140_e14955: f64 = (locals.var_tmf1 * assign20140_e14954);
        let assign20140_e14956: f64 = (assign20140_e14950 + assign20140_e14955);
        let assign20140_e14957: f64 = (locals.var_tmf1 * assign20140_e14956);
        let assign20140_e14958: f64 = (assign20140_e14946 + assign20140_e14957);
        let assign20140_e14959: f64 = (locals.var_tmf1 * assign20140_e14958);
        let assign20140_e14960: f64 = (assign20140_e14942 + assign20140_e14959);
        let assign20140_e14961: f64 = (locals.var_tmf1 * assign20140_e14960);
        let assign20140_e14962: f64 = (assign20140_e14938 + assign20140_e14961);
        let assign20140_e14963: f64 = (locals.var_tmf1 * assign20140_e14962);
        let assign20140_e14964: f64 = (assign20140_e14934 + assign20140_e14963);
        (assign20140_e14964, ((locals.var_tmf1_dn0 * assign20140_e14962) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20140_e14960) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20140_e14958) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20140_e14956) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign20140_e14954))))))))), ((locals.var_tmf1_dn2 * assign20140_e14962) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20140_e14960) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20140_e14958) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20140_e14956) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign20140_e14954))))))))), ((locals.var_tmf1_dn4 * assign20140_e14962) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20140_e14960) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20140_e14958) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20140_e14956) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign20140_e14954))))))))), ((locals.var_tmf1_dn5 * assign20140_e14962) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20140_e14960) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20140_e14958) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20140_e14956) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign20140_e14954))))))))), ((locals.var_tmf1_dn6 * assign20140_e14962) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20140_e14960) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20140_e14958) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20140_e14956) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign20140_e14954))))))))), ((locals.var_tmf1_dn7 * assign20140_e14962) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20140_e14960) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20140_e14958) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20140_e14956) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign20140_e14954))))))))), ((locals.var_tmf1_dn8 * assign20140_e14962) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20140_e14960) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20140_e14958) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20140_e14956) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign20140_e14954))))))))), ((locals.var_tmf1_dn9 * assign20140_e14962) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20140_e14960) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20140_e14958) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20140_e14956) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign20140_e14954))))))))), ((locals.var_tmf1_dn10 * assign20140_e14962) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20140_e14960) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20140_e14958) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20140_e14956) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign20140_e14954))))))))), ((locals.var_tmf1_dn11 * assign20140_e14962) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20140_e14960) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20140_e14958) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20140_e14956) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign20140_e14954))))))))), ((locals.var_tmf1_dn14 * assign20140_e14962) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20140_e14960) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20140_e14958) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20140_e14956) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign20140_e14954))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign20140_e14966;
        locals.var_tmf3_dn0 = assign20140_e14966_d_n0;
        locals.var_tmf3_dn2 = assign20140_e14966_d_n2;
        locals.var_tmf3_dn4 = assign20140_e14966_d_n4;
        locals.var_tmf3_dn5 = assign20140_e14966_d_n5;
        locals.var_tmf3_dn6 = assign20140_e14966_d_n6;
        locals.var_tmf3_dn7 = assign20140_e14966_d_n7;
        locals.var_tmf3_dn8 = assign20140_e14966_d_n8;
        locals.var_tmf3_dn9 = assign20140_e14966_d_n9;
        locals.var_tmf3_dn10 = assign20140_e14966_d_n10;
        locals.var_tmf3_dn11 = assign20140_e14966_d_n11;
        locals.var_tmf3_dn14 = assign20140_e14966_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign20150_e14972, assign20150_e14972_d_n0, assign20150_e14972_d_n2, assign20150_e14972_d_n4, assign20150_e14972_d_n5, assign20150_e14972_d_n6, assign20150_e14972_d_n7, assign20150_e14972_d_n8, assign20150_e14972_d_n9, assign20150_e14972_d_n10, assign20150_e14972_d_n11, assign20150_e14972_d_n14,) = {
    if (locals.var_guard407 != 0.0) {
        let assign20150_e14970: f64 = (p.p262 / locals.var_tmf2);
        (assign20150_e14970, (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign20150_e14972;
        locals.var_vzadd_dn0 = assign20150_e14972_d_n0;
        locals.var_vzadd_dn2 = assign20150_e14972_d_n2;
        locals.var_vzadd_dn4 = assign20150_e14972_d_n4;
        locals.var_vzadd_dn5 = assign20150_e14972_d_n5;
        locals.var_vzadd_dn6 = assign20150_e14972_d_n6;
        locals.var_vzadd_dn7 = assign20150_e14972_d_n7;
        locals.var_vzadd_dn8 = assign20150_e14972_d_n8;
        locals.var_vzadd_dn9 = assign20150_e14972_d_n9;
        locals.var_vzadd_dn10 = assign20150_e14972_d_n10;
        locals.var_vzadd_dn11 = assign20150_e14972_d_n11;
        locals.var_vzadd_dn14 = assign20150_e14972_d_n14;
        locals.var_vzadd_rv = 0.0;

        let (assign20160_e14983, assign20160_e14983_d_n0, assign20160_e14983_d_n2, assign20160_e14983_d_n4, assign20160_e14983_d_n5, assign20160_e14983_d_n6, assign20160_e14983_d_n7, assign20160_e14983_d_n8, assign20160_e14983_d_n9, assign20160_e14983_d_n10, assign20160_e14983_d_n11, assign20160_e14983_d_n14,) = {
    if (locals.var_guard407 != 0.0) {
        let assign20160_e14975: f64 = (-2.0);
        let assign20160_e14977: f64 = (assign20160_e14975 * locals.var_tmf3);
        let assign20160_e14980: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign20160_e14981: f64 = (assign20160_e14977 / assign20160_e14980);
        (assign20160_e14981, ((((assign20160_e14975 * locals.var_tmf3_dn0) * assign20160_e14980) - (assign20160_e14977 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign20160_e14980 * assign20160_e14980)), ((((assign20160_e14975 * locals.var_tmf3_dn2) * assign20160_e14980) - (assign20160_e14977 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign20160_e14980 * assign20160_e14980)), ((((assign20160_e14975 * locals.var_tmf3_dn4) * assign20160_e14980) - (assign20160_e14977 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign20160_e14980 * assign20160_e14980)), ((((assign20160_e14975 * locals.var_tmf3_dn5) * assign20160_e14980) - (assign20160_e14977 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign20160_e14980 * assign20160_e14980)), ((((assign20160_e14975 * locals.var_tmf3_dn6) * assign20160_e14980) - (assign20160_e14977 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign20160_e14980 * assign20160_e14980)), ((((assign20160_e14975 * locals.var_tmf3_dn7) * assign20160_e14980) - (assign20160_e14977 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign20160_e14980 * assign20160_e14980)), ((((assign20160_e14975 * locals.var_tmf3_dn8) * assign20160_e14980) - (assign20160_e14977 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign20160_e14980 * assign20160_e14980)), ((((assign20160_e14975 * locals.var_tmf3_dn9) * assign20160_e14980) - (assign20160_e14977 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign20160_e14980 * assign20160_e14980)), ((((assign20160_e14975 * locals.var_tmf3_dn10) * assign20160_e14980) - (assign20160_e14977 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign20160_e14980 * assign20160_e14980)), ((((assign20160_e14975 * locals.var_tmf3_dn11) * assign20160_e14980) - (assign20160_e14977 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign20160_e14980 * assign20160_e14980)), ((((assign20160_e14975 * locals.var_tmf3_dn14) * assign20160_e14980) - (assign20160_e14977 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign20160_e14980 * assign20160_e14980)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign20160_e14983;
        locals.var_t2_dn0 = assign20160_e14983_d_n0;
        locals.var_t2_dn2 = assign20160_e14983_d_n2;
        locals.var_t2_dn4 = assign20160_e14983_d_n4;
        locals.var_t2_dn5 = assign20160_e14983_d_n5;
        locals.var_t2_dn6 = assign20160_e14983_d_n6;
        locals.var_t2_dn7 = assign20160_e14983_d_n7;
        locals.var_t2_dn8 = assign20160_e14983_d_n8;
        locals.var_t2_dn9 = assign20160_e14983_d_n9;
        locals.var_t2_dn10 = assign20160_e14983_d_n10;
        locals.var_t2_dn11 = assign20160_e14983_d_n11;
        locals.var_t2_dn14 = assign20160_e14983_d_n14;
        locals.var_t2_rv = 0.0;

        let assign20170_e14986: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard409 = assign20170_e14986;
        locals.var_guard409_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_51(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20180_e14992, assign20180_e14992_d_n0, assign20180_e14992_d_n2, assign20180_e14992_d_n4, assign20180_e14992_d_n5, assign20180_e14992_d_n6, assign20180_e14992_d_n7, assign20180_e14992_d_n8, assign20180_e14992_d_n9, assign20180_e14992_d_n10, assign20180_e14992_d_n11, assign20180_e14992_d_n14,) = {
    if ((locals.var_guard407 != 0.0) && (locals.var_guard409 != 0.0)) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign20180_e14992;
        locals.var_vzadd_dn0 = assign20180_e14992_d_n0;
        locals.var_vzadd_dn2 = assign20180_e14992_d_n2;
        locals.var_vzadd_dn4 = assign20180_e14992_d_n4;
        locals.var_vzadd_dn5 = assign20180_e14992_d_n5;
        locals.var_vzadd_dn6 = assign20180_e14992_d_n6;
        locals.var_vzadd_dn7 = assign20180_e14992_d_n7;
        locals.var_vzadd_dn8 = assign20180_e14992_d_n8;
        locals.var_vzadd_dn9 = assign20180_e14992_d_n9;
        locals.var_vzadd_dn10 = assign20180_e14992_d_n10;
        locals.var_vzadd_dn11 = assign20180_e14992_d_n11;
        locals.var_vzadd_dn14 = assign20180_e14992_d_n14;
        locals.var_vzadd_rv = 0.0;

        let (assign20190_e15000, assign20190_e15000_d_n0, assign20190_e15000_d_n2, assign20190_e15000_d_n4, assign20190_e15000_d_n5, assign20190_e15000_d_n6, assign20190_e15000_d_n7, assign20190_e15000_d_n8, assign20190_e15000_d_n9, assign20190_e15000_d_n10, assign20190_e15000_d_n11, assign20190_e15000_d_n14,) = {
    if (locals.var_guard407 != 0.0) {
        let assign20190_e14997: f64 = (2.0 * locals.var_vzadd);
        let assign20190_e14998: f64 = (locals.var_vdserev + assign20190_e14997);
        (assign20190_e14998, (locals.var_vdserev_dn0 + (2.0 * locals.var_vzadd_dn0)), (locals.var_vdserev_dn2 + (2.0 * locals.var_vzadd_dn2)), (2.0 * locals.var_vzadd_dn4), (2.0 * locals.var_vzadd_dn5), (2.0 * locals.var_vzadd_dn6), (2.0 * locals.var_vzadd_dn7), (2.0 * locals.var_vzadd_dn8), (2.0 * locals.var_vzadd_dn9), (2.0 * locals.var_vzadd_dn10), (2.0 * locals.var_vzadd_dn11), (2.0 * locals.var_vzadd_dn14),)
    } else {
        (locals.var_vdserevz, locals.var_vdserevz_dn0, locals.var_vdserevz_dn2, locals.var_vdserevz_dn4, locals.var_vdserevz_dn5, locals.var_vdserevz_dn6, locals.var_vdserevz_dn7, locals.var_vdserevz_dn8, locals.var_vdserevz_dn9, locals.var_vdserevz_dn10, locals.var_vdserevz_dn11, locals.var_vdserevz_dn14,)
    }
};
        locals.var_vdserevz = assign20190_e15000;
        locals.var_vdserevz_dn0 = assign20190_e15000_d_n0;
        locals.var_vdserevz_dn2 = assign20190_e15000_d_n2;
        locals.var_vdserevz_dn4 = assign20190_e15000_d_n4;
        locals.var_vdserevz_dn5 = assign20190_e15000_d_n5;
        locals.var_vdserevz_dn6 = assign20190_e15000_d_n6;
        locals.var_vdserevz_dn7 = assign20190_e15000_d_n7;
        locals.var_vdserevz_dn8 = assign20190_e15000_d_n8;
        locals.var_vdserevz_dn9 = assign20190_e15000_d_n9;
        locals.var_vdserevz_dn10 = assign20190_e15000_d_n10;
        locals.var_vdserevz_dn11 = assign20190_e15000_d_n11;
        locals.var_vdserevz_dn14 = assign20190_e15000_d_n14;
        locals.var_vdserevz_rv = 0.0;

        let (assign20200_e15012, assign20200_e15012_d_n0, assign20200_e15012_d_n2, assign20200_e15012_d_n4, assign20200_e15012_d_n5, assign20200_e15012_d_n6, assign20200_e15012_d_n7, assign20200_e15012_d_n8, assign20200_e15012_d_n9, assign20200_e15012_d_n10, assign20200_e15012_d_n11, assign20200_e15012_d_n14,) = {
    if (locals.var_guard407 != 0.0) {
        let assign20200_e15005: f64 = (p.p333 * locals.var_vdserevz);
        let assign20200_e15006: f64 = (p.p335 - assign20200_e15005);
        let assign20200_e15009: f64 = (p.p332 * locals.var_vsubsrev);
        let assign20200_e15010: f64 = (assign20200_e15006 - assign20200_e15009);
        (assign20200_e15010, ((-(p.p333 * locals.var_vdserevz_dn0)) - (p.p332 * locals.var_vsubsrev_dn0)), ((-(p.p333 * locals.var_vdserevz_dn2)) - (p.p332 * locals.var_vsubsrev_dn2)), ((-(p.p333 * locals.var_vdserevz_dn4)) - (p.p332 * locals.var_vsubsrev_dn4)), (-(p.p333 * locals.var_vdserevz_dn5)), (-(p.p333 * locals.var_vdserevz_dn6)), (-(p.p333 * locals.var_vdserevz_dn7)), (-(p.p333 * locals.var_vdserevz_dn8)), (-(p.p333 * locals.var_vdserevz_dn9)), (-(p.p333 * locals.var_vdserevz_dn10)), (-(p.p333 * locals.var_vdserevz_dn11)), (-(p.p333 * locals.var_vdserevz_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20200_e15012;
        locals.var_t0_dn0 = assign20200_e15012_d_n0;
        locals.var_t0_dn2 = assign20200_e15012_d_n2;
        locals.var_t0_dn4 = assign20200_e15012_d_n4;
        locals.var_t0_dn5 = assign20200_e15012_d_n5;
        locals.var_t0_dn6 = assign20200_e15012_d_n6;
        locals.var_t0_dn7 = assign20200_e15012_d_n7;
        locals.var_t0_dn8 = assign20200_e15012_d_n8;
        locals.var_t0_dn9 = assign20200_e15012_d_n9;
        locals.var_t0_dn10 = assign20200_e15012_d_n10;
        locals.var_t0_dn11 = assign20200_e15012_d_n11;
        locals.var_t0_dn14 = assign20200_e15012_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign20210_e15025, assign20210_e15025_d_n0, assign20210_e15025_d_n2, assign20210_e15025_d_n4, assign20210_e15025_d_n5, assign20210_e15025_d_n6, assign20210_e15025_d_n7, assign20210_e15025_d_n8, assign20210_e15025_d_n9, assign20210_e15025_d_n10, assign20210_e15025_d_n11, assign20210_e15025_d_n14,) = {
    if (locals.var_guard407 != 0.0) {
        let assign20210_e15016: f64 = (locals.var_t0 * locals.var_t0);
        let assign20210_e15019: f64 = (4.0 * 10.0);
        let assign20210_e15021: f64 = (assign20210_e15019 * 10.0);
        let assign20210_e15022: f64 = (assign20210_e15016 + assign20210_e15021);
        let assign20210_e15023: f64 = (assign20210_e15022).sqrt();
        (assign20210_e15023, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign20210_e15023)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign20210_e15023)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign20210_e15023)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign20210_e15023)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign20210_e15023)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign20210_e15023)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign20210_e15023)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign20210_e15023)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign20210_e15023)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign20210_e15023)), (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign20210_e15023)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20210_e15025;
        locals.var_tmf2_dn0 = assign20210_e15025_d_n0;
        locals.var_tmf2_dn2 = assign20210_e15025_d_n2;
        locals.var_tmf2_dn4 = assign20210_e15025_d_n4;
        locals.var_tmf2_dn5 = assign20210_e15025_d_n5;
        locals.var_tmf2_dn6 = assign20210_e15025_d_n6;
        locals.var_tmf2_dn7 = assign20210_e15025_d_n7;
        locals.var_tmf2_dn8 = assign20210_e15025_d_n8;
        locals.var_tmf2_dn9 = assign20210_e15025_d_n9;
        locals.var_tmf2_dn10 = assign20210_e15025_d_n10;
        locals.var_tmf2_dn11 = assign20210_e15025_d_n11;
        locals.var_tmf2_dn14 = assign20210_e15025_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20220_e15035, assign20220_e15035_d_n0, assign20220_e15035_d_n2, assign20220_e15035_d_n4, assign20220_e15035_d_n5, assign20220_e15035_d_n6, assign20220_e15035_d_n7, assign20220_e15035_d_n8, assign20220_e15035_d_n9, assign20220_e15035_d_n10, assign20220_e15035_d_n11, assign20220_e15035_d_n14,) = {
    if (locals.var_guard407 != 0.0) {
        let assign20220_e15031: f64 = (locals.var_t0 / locals.var_tmf2);
        let assign20220_e15032: f64 = (1.0 + assign20220_e15031);
        let assign20220_e15033: f64 = (0.5 * assign20220_e15032);
        (assign20220_e15033, (0.5 * (((locals.var_t0_dn0 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn2 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn4 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn5 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn6 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn7 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn8 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn9 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn10 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn11 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn14 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign20220_e15035;
        locals.var_t2_dn0 = assign20220_e15035_d_n0;
        locals.var_t2_dn2 = assign20220_e15035_d_n2;
        locals.var_t2_dn4 = assign20220_e15035_d_n4;
        locals.var_t2_dn5 = assign20220_e15035_d_n5;
        locals.var_t2_dn6 = assign20220_e15035_d_n6;
        locals.var_t2_dn7 = assign20220_e15035_d_n7;
        locals.var_t2_dn8 = assign20220_e15035_d_n8;
        locals.var_t2_dn9 = assign20220_e15035_d_n9;
        locals.var_t2_dn10 = assign20220_e15035_d_n10;
        locals.var_t2_dn11 = assign20220_e15035_d_n11;
        locals.var_t2_dn14 = assign20220_e15035_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign20230_e15043, assign20230_e15043_d_n0, assign20230_e15043_d_n2, assign20230_e15043_d_n4, assign20230_e15043_d_n5, assign20230_e15043_d_n6, assign20230_e15043_d_n7, assign20230_e15043_d_n8, assign20230_e15043_d_n9, assign20230_e15043_d_n10, assign20230_e15043_d_n11, assign20230_e15043_d_n14,) = {
    if (locals.var_guard407 != 0.0) {
        let assign20230_e15040: f64 = (locals.var_t0 + locals.var_tmf2);
        let assign20230_e15041: f64 = (0.5 * assign20230_e15040);
        (assign20230_e15041, (0.5 * (locals.var_t0_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t0_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t0_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t0_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t0_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t0_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t0_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t0_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t0_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t0_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t0_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign20230_e15043;
        locals.var_t1_dn0 = assign20230_e15043_d_n0;
        locals.var_t1_dn2 = assign20230_e15043_d_n2;
        locals.var_t1_dn4 = assign20230_e15043_d_n4;
        locals.var_t1_dn5 = assign20230_e15043_d_n5;
        locals.var_t1_dn6 = assign20230_e15043_d_n6;
        locals.var_t1_dn7 = assign20230_e15043_d_n7;
        locals.var_t1_dn8 = assign20230_e15043_d_n8;
        locals.var_t1_dn9 = assign20230_e15043_d_n9;
        locals.var_t1_dn10 = assign20230_e15043_d_n10;
        locals.var_t1_dn11 = assign20230_e15043_d_n11;
        locals.var_t1_dn14 = assign20230_e15043_d_n14;
        locals.var_t1_rv = 0.0;

        let assign20240_e15046: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard410 = assign20240_e15046;
        locals.var_guard410_rv = 0.0;

        let (assign20250_e15052, assign20250_e15052_d_n0, assign20250_e15052_d_n2, assign20250_e15052_d_n4, assign20250_e15052_d_n5, assign20250_e15052_d_n6, assign20250_e15052_d_n7, assign20250_e15052_d_n8, assign20250_e15052_d_n9, assign20250_e15052_d_n10, assign20250_e15052_d_n11, assign20250_e15052_d_n14,) = {
    if ((locals.var_guard407 != 0.0) && (locals.var_guard410 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign20250_e15052;
        locals.var_t1_dn0 = assign20250_e15052_d_n0;
        locals.var_t1_dn2 = assign20250_e15052_d_n2;
        locals.var_t1_dn4 = assign20250_e15052_d_n4;
        locals.var_t1_dn5 = assign20250_e15052_d_n5;
        locals.var_t1_dn6 = assign20250_e15052_d_n6;
        locals.var_t1_dn7 = assign20250_e15052_d_n7;
        locals.var_t1_dn8 = assign20250_e15052_d_n8;
        locals.var_t1_dn9 = assign20250_e15052_d_n9;
        locals.var_t1_dn10 = assign20250_e15052_d_n10;
        locals.var_t1_dn11 = assign20250_e15052_d_n11;
        locals.var_t1_dn14 = assign20250_e15052_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign20260_e15058, assign20260_e15058_d_n0, assign20260_e15058_d_n2, assign20260_e15058_d_n4, assign20260_e15058_d_n5, assign20260_e15058_d_n6, assign20260_e15058_d_n7, assign20260_e15058_d_n8, assign20260_e15058_d_n9, assign20260_e15058_d_n10, assign20260_e15058_d_n11, assign20260_e15058_d_n14,) = {
    if ((locals.var_guard407 != 0.0) && (locals.var_guard410 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign20260_e15058;
        locals.var_t2_dn0 = assign20260_e15058_d_n0;
        locals.var_t2_dn2 = assign20260_e15058_d_n2;
        locals.var_t2_dn4 = assign20260_e15058_d_n4;
        locals.var_t2_dn5 = assign20260_e15058_d_n5;
        locals.var_t2_dn6 = assign20260_e15058_d_n6;
        locals.var_t2_dn7 = assign20260_e15058_d_n7;
        locals.var_t2_dn8 = assign20260_e15058_d_n8;
        locals.var_t2_dn9 = assign20260_e15058_d_n9;
        locals.var_t2_dn10 = assign20260_e15058_d_n10;
        locals.var_t2_dn11 = assign20260_e15058_d_n11;
        locals.var_t2_dn14 = assign20260_e15058_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign20270_e15066, assign20270_e15066_d_n0, assign20270_e15066_d_n2, assign20270_e15066_d_n4, assign20270_e15066_d_n5, assign20270_e15066_d_n6, assign20270_e15066_d_n7, assign20270_e15066_d_n8, assign20270_e15066_d_n9, assign20270_e15066_d_n10, assign20270_e15066_d_n11, assign20270_e15066_d_n14,) = {
    if (locals.var_guard407 != 0.0) {
        let assign20270_e15063: f64 = (10.0 * 2.220446049250313e-16);
        let assign20270_e15064: f64 = (locals.var_t1 + assign20270_e15063);
        (assign20270_e15064, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign20270_e15066;
        locals.var_t1_dn0 = assign20270_e15066_d_n0;
        locals.var_t1_dn2 = assign20270_e15066_d_n2;
        locals.var_t1_dn4 = assign20270_e15066_d_n4;
        locals.var_t1_dn5 = assign20270_e15066_d_n5;
        locals.var_t1_dn6 = assign20270_e15066_d_n6;
        locals.var_t1_dn7 = assign20270_e15066_d_n7;
        locals.var_t1_dn8 = assign20270_e15066_d_n8;
        locals.var_t1_dn9 = assign20270_e15066_d_n9;
        locals.var_t1_dn10 = assign20270_e15066_d_n10;
        locals.var_t1_dn11 = assign20270_e15066_d_n11;
        locals.var_t1_dn14 = assign20270_e15066_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign20280_e15076, assign20280_e15076_d_n0, assign20280_e15076_d_n2, assign20280_e15076_d_n4, assign20280_e15076_d_n5, assign20280_e15076_d_n6, assign20280_e15076_d_n7, assign20280_e15076_d_n8, assign20280_e15076_d_n9, assign20280_e15076_d_n10, assign20280_e15076_d_n11, assign20280_e15076_d_n14,) = {
    if (locals.var_guard407 != 0.0) {
        let assign20280_e15072: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign20280_e15073: f64 = (locals.var_uc_nover * assign20280_e15072);
        let assign20280_e15074: f64 = (locals.var_mks_nsubsub / assign20280_e15073);
        (assign20280_e15074, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20280_e15076;
        locals.var_t0_dn0 = assign20280_e15076_d_n0;
        locals.var_t0_dn2 = assign20280_e15076_d_n2;
        locals.var_t0_dn4 = assign20280_e15076_d_n4;
        locals.var_t0_dn5 = assign20280_e15076_d_n5;
        locals.var_t0_dn6 = assign20280_e15076_d_n6;
        locals.var_t0_dn7 = assign20280_e15076_d_n7;
        locals.var_t0_dn8 = assign20280_e15076_d_n8;
        locals.var_t0_dn9 = assign20280_e15076_d_n9;
        locals.var_t0_dn10 = assign20280_e15076_d_n10;
        locals.var_t0_dn11 = assign20280_e15076_d_n11;
        locals.var_t0_dn14 = assign20280_e15076_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign20290_e15086, assign20290_e15086_d_n0, assign20290_e15086_d_n2, assign20290_e15086_d_n4, assign20290_e15086_d_n5, assign20290_e15086_d_n6, assign20290_e15086_d_n7, assign20290_e15086_d_n8, assign20290_e15086_d_n9, assign20290_e15086_d_n10, assign20290_e15086_d_n11, assign20290_e15086_d_n14,) = {
    if (locals.var_guard407 != 0.0) {
        let assign20290_e15080: f64 = (2.0 * 1.034943e-10);
        let assign20290_e15082: f64 = (assign20290_e15080 / 1.6021918e-19);
        let assign20290_e15084: f64 = (assign20290_e15082 * locals.var_t0);
        (assign20290_e15084, (assign20290_e15082 * locals.var_t0_dn0), (assign20290_e15082 * locals.var_t0_dn2), (assign20290_e15082 * locals.var_t0_dn4), (assign20290_e15082 * locals.var_t0_dn5), (assign20290_e15082 * locals.var_t0_dn6), (assign20290_e15082 * locals.var_t0_dn7), (assign20290_e15082 * locals.var_t0_dn8), (assign20290_e15082 * locals.var_t0_dn9), (assign20290_e15082 * locals.var_t0_dn10), (assign20290_e15082 * locals.var_t0_dn11), (assign20290_e15082 * locals.var_t0_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign20290_e15086;
        locals.var_t4_dn0 = assign20290_e15086_d_n0;
        locals.var_t4_dn2 = assign20290_e15086_d_n2;
        locals.var_t4_dn4 = assign20290_e15086_d_n4;
        locals.var_t4_dn5 = assign20290_e15086_d_n5;
        locals.var_t4_dn6 = assign20290_e15086_d_n6;
        locals.var_t4_dn7 = assign20290_e15086_d_n7;
        locals.var_t4_dn8 = assign20290_e15086_d_n8;
        locals.var_t4_dn9 = assign20290_e15086_d_n9;
        locals.var_t4_dn10 = assign20290_e15086_d_n10;
        locals.var_t4_dn11 = assign20290_e15086_d_n11;
        locals.var_t4_dn14 = assign20290_e15086_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign20300_e15095, assign20300_e15095_d_n0, assign20300_e15095_d_n2, assign20300_e15095_d_n4, assign20300_e15095_d_n5, assign20300_e15095_d_n6, assign20300_e15095_d_n7, assign20300_e15095_d_n8, assign20300_e15095_d_n9, assign20300_e15095_d_n10, assign20300_e15095_d_n11, assign20300_e15095_d_n14,) = {
    if (locals.var_guard407 != 0.0) {
        let assign20300_e15090: f64 = (locals.var_t4 * locals.var_t1);
        let assign20300_e15091: f64 = (assign20300_e15090).sqrt();
        let assign20300_e15093: f64 = (assign20300_e15091 + 1e-25);
        (assign20300_e15093, (((locals.var_t4_dn0 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn0)) / (2.0 * assign20300_e15091)), (((locals.var_t4_dn2 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn2)) / (2.0 * assign20300_e15091)), (((locals.var_t4_dn4 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn4)) / (2.0 * assign20300_e15091)), (((locals.var_t4_dn5 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn5)) / (2.0 * assign20300_e15091)), (((locals.var_t4_dn6 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn6)) / (2.0 * assign20300_e15091)), (((locals.var_t4_dn7 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn7)) / (2.0 * assign20300_e15091)), (((locals.var_t4_dn8 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn8)) / (2.0 * assign20300_e15091)), (((locals.var_t4_dn9 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn9)) / (2.0 * assign20300_e15091)), (((locals.var_t4_dn10 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn10)) / (2.0 * assign20300_e15091)), (((locals.var_t4_dn11 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn11)) / (2.0 * assign20300_e15091)), (((locals.var_t4_dn14 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn14)) / (2.0 * assign20300_e15091)),)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    }
};
        locals.var_wdep = assign20300_e15095;
        locals.var_wdep_dn0 = assign20300_e15095_d_n0;
        locals.var_wdep_dn2 = assign20300_e15095_d_n2;
        locals.var_wdep_dn4 = assign20300_e15095_d_n4;
        locals.var_wdep_dn5 = assign20300_e15095_d_n5;
        locals.var_wdep_dn6 = assign20300_e15095_d_n6;
        locals.var_wdep_dn7 = assign20300_e15095_d_n7;
        locals.var_wdep_dn8 = assign20300_e15095_d_n8;
        locals.var_wdep_dn9 = assign20300_e15095_d_n9;
        locals.var_wdep_dn10 = assign20300_e15095_d_n10;
        locals.var_wdep_dn11 = assign20300_e15095_d_n11;
        locals.var_wdep_dn14 = assign20300_e15095_d_n14;
        locals.var_wdep_rv = 0.0;

        let (assign20310_e15105, assign20310_e15105_d_n0, assign20310_e15105_d_n2, assign20310_e15105_d_n4, assign20310_e15105_d_n5, assign20310_e15105_d_n6, assign20310_e15105_d_n7, assign20310_e15105_d_n8, assign20310_e15105_d_n9, assign20310_e15105_d_n10, assign20310_e15105_d_n11, assign20310_e15105_d_n14,) = {
    if (locals.var_guard407 != 0.0) {
        let assign20310_e15099: f64 = (p.p334 - locals.var_wdep);
        let assign20310_e15102: f64 = (0.1 * p.p334);
        let assign20310_e15103: f64 = (assign20310_e15099 - assign20310_e15102);
        (assign20310_e15103, (-locals.var_wdep_dn0), (-locals.var_wdep_dn2), (-locals.var_wdep_dn4), (-locals.var_wdep_dn5), (-locals.var_wdep_dn6), (-locals.var_wdep_dn7), (-locals.var_wdep_dn8), (-locals.var_wdep_dn9), (-locals.var_wdep_dn10), (-locals.var_wdep_dn11), (-locals.var_wdep_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign20310_e15105;
        locals.var_tmf1_dn0 = assign20310_e15105_d_n0;
        locals.var_tmf1_dn2 = assign20310_e15105_d_n2;
        locals.var_tmf1_dn4 = assign20310_e15105_d_n4;
        locals.var_tmf1_dn5 = assign20310_e15105_d_n5;
        locals.var_tmf1_dn6 = assign20310_e15105_d_n6;
        locals.var_tmf1_dn7 = assign20310_e15105_d_n7;
        locals.var_tmf1_dn8 = assign20310_e15105_d_n8;
        locals.var_tmf1_dn9 = assign20310_e15105_d_n9;
        locals.var_tmf1_dn10 = assign20310_e15105_d_n10;
        locals.var_tmf1_dn11 = assign20310_e15105_d_n11;
        locals.var_tmf1_dn14 = assign20310_e15105_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign20320_e15115, assign20320_e15115_d_n0, assign20320_e15115_d_n2, assign20320_e15115_d_n4, assign20320_e15115_d_n5, assign20320_e15115_d_n6, assign20320_e15115_d_n7, assign20320_e15115_d_n8, assign20320_e15115_d_n9, assign20320_e15115_d_n10, assign20320_e15115_d_n11, assign20320_e15115_d_n14,) = {
    if (locals.var_guard407 != 0.0) {
        let assign20320_e15109: f64 = (4.0 * p.p334);
        let assign20320_e15112: f64 = (0.1 * p.p334);
        let assign20320_e15113: f64 = (assign20320_e15109 * assign20320_e15112);
        (assign20320_e15113, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20320_e15115;
        locals.var_tmf2_dn0 = assign20320_e15115_d_n0;
        locals.var_tmf2_dn2 = assign20320_e15115_d_n2;
        locals.var_tmf2_dn4 = assign20320_e15115_d_n4;
        locals.var_tmf2_dn5 = assign20320_e15115_d_n5;
        locals.var_tmf2_dn6 = assign20320_e15115_d_n6;
        locals.var_tmf2_dn7 = assign20320_e15115_d_n7;
        locals.var_tmf2_dn8 = assign20320_e15115_d_n8;
        locals.var_tmf2_dn9 = assign20320_e15115_d_n9;
        locals.var_tmf2_dn10 = assign20320_e15115_d_n10;
        locals.var_tmf2_dn11 = assign20320_e15115_d_n11;
        locals.var_tmf2_dn14 = assign20320_e15115_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20330_e15125, assign20330_e15125_d_n0, assign20330_e15125_d_n2, assign20330_e15125_d_n4, assign20330_e15125_d_n5, assign20330_e15125_d_n6, assign20330_e15125_d_n7, assign20330_e15125_d_n8, assign20330_e15125_d_n9, assign20330_e15125_d_n10, assign20330_e15125_d_n11, assign20330_e15125_d_n14,) = {
    if (locals.var_guard407 != 0.0) {
        let (assign20330_e15123, assign20330_e15123_d_n0, assign20330_e15123_d_n2, assign20330_e15123_d_n4, assign20330_e15123_d_n5, assign20330_e15123_d_n6, assign20330_e15123_d_n7, assign20330_e15123_d_n8, assign20330_e15123_d_n9, assign20330_e15123_d_n10, assign20330_e15123_d_n11, assign20330_e15123_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign20330_e15122: f64 = (-locals.var_tmf2);
                (assign20330_e15122, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign20330_e15123, assign20330_e15123_d_n0, assign20330_e15123_d_n2, assign20330_e15123_d_n4, assign20330_e15123_d_n5, assign20330_e15123_d_n6, assign20330_e15123_d_n7, assign20330_e15123_d_n8, assign20330_e15123_d_n9, assign20330_e15123_d_n10, assign20330_e15123_d_n11, assign20330_e15123_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20330_e15125;
        locals.var_tmf2_dn0 = assign20330_e15125_d_n0;
        locals.var_tmf2_dn2 = assign20330_e15125_d_n2;
        locals.var_tmf2_dn4 = assign20330_e15125_d_n4;
        locals.var_tmf2_dn5 = assign20330_e15125_d_n5;
        locals.var_tmf2_dn6 = assign20330_e15125_d_n6;
        locals.var_tmf2_dn7 = assign20330_e15125_d_n7;
        locals.var_tmf2_dn8 = assign20330_e15125_d_n8;
        locals.var_tmf2_dn9 = assign20330_e15125_d_n9;
        locals.var_tmf2_dn10 = assign20330_e15125_d_n10;
        locals.var_tmf2_dn11 = assign20330_e15125_d_n11;
        locals.var_tmf2_dn14 = assign20330_e15125_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20340_e15134, assign20340_e15134_d_n0, assign20340_e15134_d_n2, assign20340_e15134_d_n4, assign20340_e15134_d_n5, assign20340_e15134_d_n6, assign20340_e15134_d_n7, assign20340_e15134_d_n8, assign20340_e15134_d_n9, assign20340_e15134_d_n10, assign20340_e15134_d_n11, assign20340_e15134_d_n14,) = {
    if (locals.var_guard407 != 0.0) {
        let assign20340_e15129: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20340_e15131: f64 = (assign20340_e15129 + locals.var_tmf2);
        let assign20340_e15132: f64 = (assign20340_e15131).sqrt();
        (assign20340_e15132, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20340_e15132)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20340_e15132)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign20340_e15132)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign20340_e15132)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20340_e15132)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign20340_e15132)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign20340_e15132)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign20340_e15132)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20340_e15132)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign20340_e15132)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign20340_e15132)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20340_e15134;
        locals.var_tmf2_dn0 = assign20340_e15134_d_n0;
        locals.var_tmf2_dn2 = assign20340_e15134_d_n2;
        locals.var_tmf2_dn4 = assign20340_e15134_d_n4;
        locals.var_tmf2_dn5 = assign20340_e15134_d_n5;
        locals.var_tmf2_dn6 = assign20340_e15134_d_n6;
        locals.var_tmf2_dn7 = assign20340_e15134_d_n7;
        locals.var_tmf2_dn8 = assign20340_e15134_d_n8;
        locals.var_tmf2_dn9 = assign20340_e15134_d_n9;
        locals.var_tmf2_dn10 = assign20340_e15134_d_n10;
        locals.var_tmf2_dn11 = assign20340_e15134_d_n11;
        locals.var_tmf2_dn14 = assign20340_e15134_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20350_e15144, assign20350_e15144_d_n0, assign20350_e15144_d_n2, assign20350_e15144_d_n4, assign20350_e15144_d_n5, assign20350_e15144_d_n6, assign20350_e15144_d_n7, assign20350_e15144_d_n8, assign20350_e15144_d_n9, assign20350_e15144_d_n10, assign20350_e15144_d_n11, assign20350_e15144_d_n14,) = {
    if (locals.var_guard407 != 0.0) {
        let assign20350_e15140: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign20350_e15141: f64 = (1.0 + assign20350_e15140);
        let assign20350_e15142: f64 = (0.5 * assign20350_e15141);
        (assign20350_e15142, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20350_e15144;
        locals.var_t0_dn0 = assign20350_e15144_d_n0;
        locals.var_t0_dn2 = assign20350_e15144_d_n2;
        locals.var_t0_dn4 = assign20350_e15144_d_n4;
        locals.var_t0_dn5 = assign20350_e15144_d_n5;
        locals.var_t0_dn6 = assign20350_e15144_d_n6;
        locals.var_t0_dn7 = assign20350_e15144_d_n7;
        locals.var_t0_dn8 = assign20350_e15144_d_n8;
        locals.var_t0_dn9 = assign20350_e15144_d_n9;
        locals.var_t0_dn10 = assign20350_e15144_d_n10;
        locals.var_t0_dn11 = assign20350_e15144_d_n11;
        locals.var_t0_dn14 = assign20350_e15144_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign20360_e15154, assign20360_e15154_d_n0, assign20360_e15154_d_n2, assign20360_e15154_d_n4, assign20360_e15154_d_n5, assign20360_e15154_d_n6, assign20360_e15154_d_n7, assign20360_e15154_d_n8, assign20360_e15154_d_n9, assign20360_e15154_d_n10, assign20360_e15154_d_n11, assign20360_e15154_d_n14,) = {
    if (locals.var_guard407 != 0.0) {
        let assign20360_e15150: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20360_e15151: f64 = (0.5 * assign20360_e15150);
        let assign20360_e15152: f64 = (p.p334 - assign20360_e15151);
        (assign20360_e15152, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    }
};
        locals.var_wdep = assign20360_e15154;
        locals.var_wdep_dn0 = assign20360_e15154_d_n0;
        locals.var_wdep_dn2 = assign20360_e15154_d_n2;
        locals.var_wdep_dn4 = assign20360_e15154_d_n4;
        locals.var_wdep_dn5 = assign20360_e15154_d_n5;
        locals.var_wdep_dn6 = assign20360_e15154_d_n6;
        locals.var_wdep_dn7 = assign20360_e15154_d_n7;
        locals.var_wdep_dn8 = assign20360_e15154_d_n8;
        locals.var_wdep_dn9 = assign20360_e15154_d_n9;
        locals.var_wdep_dn10 = assign20360_e15154_d_n10;
        locals.var_wdep_dn11 = assign20360_e15154_d_n11;
        locals.var_wdep_dn14 = assign20360_e15154_d_n14;
        locals.var_wdep_rv = 0.0;

        let (assign20370_e15159, assign20370_e15159_d_n0, assign20370_e15159_d_n2, assign20370_e15159_d_n4, assign20370_e15159_d_n5, assign20370_e15159_d_n6, assign20370_e15159_d_n7, assign20370_e15159_d_n8, assign20370_e15159_d_n9, assign20370_e15159_d_n10, assign20370_e15159_d_n11, assign20370_e15159_d_n14,) = {
    if (locals.var_guard407 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    }
};
        locals.var_wdep = assign20370_e15159;
        locals.var_wdep_dn0 = assign20370_e15159_d_n0;
        locals.var_wdep_dn2 = assign20370_e15159_d_n2;
        locals.var_wdep_dn4 = assign20370_e15159_d_n4;
        locals.var_wdep_dn5 = assign20370_e15159_d_n5;
        locals.var_wdep_dn6 = assign20370_e15159_d_n6;
        locals.var_wdep_dn7 = assign20370_e15159_d_n7;
        locals.var_wdep_dn8 = assign20370_e15159_d_n8;
        locals.var_wdep_dn9 = assign20370_e15159_d_n9;
        locals.var_wdep_dn10 = assign20370_e15159_d_n10;
        locals.var_wdep_dn11 = assign20370_e15159_d_n11;
        locals.var_wdep_dn14 = assign20370_e15159_d_n14;
        locals.var_wdep_rv = 0.0;

        let assign20380_e15166: f64 = if ((locals.var_flg_rsrd == 1.0) || (locals.var_flg_rsrd == 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard411 = assign20380_e15166;
        locals.var_guard411_rv = 0.0;

        let (assign20390_e15170, assign20390_e15170_d_n0, assign20390_e15170_d_n2,) = {
    if (locals.var_guard411 != 0.0) {
        (locals.var_vdsei, locals.var_vdsei_dn0, locals.var_vdsei_dn2,)
    } else {
        (locals.var_vdsegmt, locals.var_vdsegmt_dn0, locals.var_vdsegmt_dn2,)
    }
};
        locals.var_vdsegmt = assign20390_e15170;
        locals.var_vdsegmt_dn0 = assign20390_e15170_d_n0;
        locals.var_vdsegmt_dn2 = assign20390_e15170_d_n2;
        locals.var_vdsegmt_rv = 0.0;

        let (assign20400_e15174, assign20400_e15174_d_n2, assign20400_e15174_d_n7,) = {
    if (locals.var_guard411 != 0.0) {
        (locals.var_vgsei, locals.var_vgsei_dn2, locals.var_vgsei_dn7,)
    } else {
        (locals.var_vgsegmt, locals.var_vgsegmt_dn2, locals.var_vgsegmt_dn7,)
    }
};
        locals.var_vgsegmt = assign20400_e15174;
        locals.var_vgsegmt_dn2 = assign20400_e15174_d_n2;
        locals.var_vgsegmt_dn7 = assign20400_e15174_d_n7;
        locals.var_vgsegmt_rv = 0.0;

        let (assign20410_e15178, assign20410_e15178_d_n2, assign20410_e15178_d_n9,) = {
    if (locals.var_guard411 != 0.0) {
        (locals.var_vbsei, locals.var_vbsei_dn2, locals.var_vbsei_dn9,)
    } else {
        (locals.var_vbsegmt, locals.var_vbsegmt_dn2, locals.var_vbsegmt_dn9,)
    }
};
        locals.var_vbsegmt = assign20410_e15178;
        locals.var_vbsegmt_dn2 = assign20410_e15178_d_n2;
        locals.var_vbsegmt_dn9 = assign20410_e15178_d_n9;
        locals.var_vbsegmt_rv = 0.0;

        let assign20420_e15181: f64 = if locals.var_vdsegmt >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard412 = assign20420_e15181;
        locals.var_guard412_rv = 0.0;

        let (assign20430_e15187,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard412 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_vdsemodenml,)
    }
};
        locals.var_vdsemodenml = assign20430_e15187;
        locals.var_vdsemodenml_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_52(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20440_e15193,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard412 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_vdsemodervs,)
    }
};
        locals.var_vdsemodervs = assign20440_e15193;
        locals.var_vdsemodervs_rv = 0.0;

        let (assign20450_e15199, assign20450_e15199_d_n0, assign20450_e15199_d_n2,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard412 != 0.0)) {
        (locals.var_vdsegmt, locals.var_vdsegmt_dn0, locals.var_vdsegmt_dn2,)
    } else {
        (locals.var_vdserev, locals.var_vdserev_dn0, locals.var_vdserev_dn2,)
    }
};
        locals.var_vdserev = assign20450_e15199;
        locals.var_vdserev_dn0 = assign20450_e15199_d_n0;
        locals.var_vdserev_dn2 = assign20450_e15199_d_n2;
        locals.var_vdserev_rv = 0.0;

        let (assign20460_e15205, assign20460_e15205_d_n0, assign20460_e15205_d_n2, assign20460_e15205_d_n7,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard412 != 0.0)) {
        (locals.var_vgsegmt, 0.0, locals.var_vgsegmt_dn2, locals.var_vgsegmt_dn7,)
    } else {
        (locals.var_vgserev, locals.var_vgserev_dn0, locals.var_vgserev_dn2, locals.var_vgserev_dn7,)
    }
};
        locals.var_vgserev = assign20460_e15205;
        locals.var_vgserev_dn0 = assign20460_e15205_d_n0;
        locals.var_vgserev_dn2 = assign20460_e15205_d_n2;
        locals.var_vgserev_dn7 = assign20460_e15205_d_n7;
        locals.var_vgserev_rv = 0.0;

        let (assign20470_e15211, assign20470_e15211_d_n0, assign20470_e15211_d_n2, assign20470_e15211_d_n9,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard412 != 0.0)) {
        (locals.var_vbsegmt, 0.0, locals.var_vbsegmt_dn2, locals.var_vbsegmt_dn9,)
    } else {
        (locals.var_vbserev, locals.var_vbserev_dn0, locals.var_vbserev_dn2, locals.var_vbserev_dn9,)
    }
};
        locals.var_vbserev = assign20470_e15211;
        locals.var_vbserev_dn0 = assign20470_e15211_d_n0;
        locals.var_vbserev_dn2 = assign20470_e15211_d_n2;
        locals.var_vbserev_dn9 = assign20470_e15211_d_n9;
        locals.var_vbserev_rv = 0.0;

        let (assign20480_e15217, assign20480_e15217_d_n0, assign20480_e15217_d_n2, assign20480_e15217_d_n4,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard412 != 0.0)) {
        (locals.var_vsubs, 0.0, locals.var_vsubs_dn2, locals.var_vsubs_dn4,)
    } else {
        (locals.var_vsubsrev, locals.var_vsubsrev_dn0, locals.var_vsubsrev_dn2, locals.var_vsubsrev_dn4,)
    }
};
        locals.var_vsubsrev = assign20480_e15217;
        locals.var_vsubsrev_dn0 = assign20480_e15217_d_n0;
        locals.var_vsubsrev_dn2 = assign20480_e15217_d_n2;
        locals.var_vsubsrev_dn4 = assign20480_e15217_d_n4;
        locals.var_vsubsrev_rv = 0.0;

        let (assign20490_e15224,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard412 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_vdsemodenml,)
    }
};
        locals.var_vdsemodenml = assign20490_e15224;
        locals.var_vdsemodenml_rv = 0.0;

        let (assign20500_e15231,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard412 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_vdsemodervs,)
    }
};
        locals.var_vdsemodervs = assign20500_e15231;
        locals.var_vdsemodervs_rv = 0.0;

        let (assign20510_e15239, assign20510_e15239_d_n0, assign20510_e15239_d_n2,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard412 == 0.0)) {
        let assign20510_e15237: f64 = (-locals.var_vdsegmt);
        (assign20510_e15237, (-locals.var_vdsegmt_dn0), (-locals.var_vdsegmt_dn2),)
    } else {
        (locals.var_vdserev, locals.var_vdserev_dn0, locals.var_vdserev_dn2,)
    }
};
        locals.var_vdserev = assign20510_e15239;
        locals.var_vdserev_dn0 = assign20510_e15239_d_n0;
        locals.var_vdserev_dn2 = assign20510_e15239_d_n2;
        locals.var_vdserev_rv = 0.0;

        let (assign20520_e15248, assign20520_e15248_d_n0, assign20520_e15248_d_n2, assign20520_e15248_d_n7,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard412 == 0.0)) {
        let assign20520_e15246: f64 = (locals.var_vgsegmt - locals.var_vdsegmt);
        (assign20520_e15246, (-locals.var_vdsegmt_dn0), (locals.var_vgsegmt_dn2 - locals.var_vdsegmt_dn2), locals.var_vgsegmt_dn7,)
    } else {
        (locals.var_vgserev, locals.var_vgserev_dn0, locals.var_vgserev_dn2, locals.var_vgserev_dn7,)
    }
};
        locals.var_vgserev = assign20520_e15248;
        locals.var_vgserev_dn0 = assign20520_e15248_d_n0;
        locals.var_vgserev_dn2 = assign20520_e15248_d_n2;
        locals.var_vgserev_dn7 = assign20520_e15248_d_n7;
        locals.var_vgserev_rv = 0.0;

        let (assign20530_e15257, assign20530_e15257_d_n0, assign20530_e15257_d_n2, assign20530_e15257_d_n9,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard412 == 0.0)) {
        let assign20530_e15255: f64 = (locals.var_vbsegmt - locals.var_vdsegmt);
        (assign20530_e15255, (-locals.var_vdsegmt_dn0), (locals.var_vbsegmt_dn2 - locals.var_vdsegmt_dn2), locals.var_vbsegmt_dn9,)
    } else {
        (locals.var_vbserev, locals.var_vbserev_dn0, locals.var_vbserev_dn2, locals.var_vbserev_dn9,)
    }
};
        locals.var_vbserev = assign20530_e15257;
        locals.var_vbserev_dn0 = assign20530_e15257_d_n0;
        locals.var_vbserev_dn2 = assign20530_e15257_d_n2;
        locals.var_vbserev_dn9 = assign20530_e15257_d_n9;
        locals.var_vbserev_rv = 0.0;

        let (assign20540_e15266, assign20540_e15266_d_n0, assign20540_e15266_d_n2, assign20540_e15266_d_n4,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard412 == 0.0)) {
        let assign20540_e15264: f64 = (locals.var_vsubs - locals.var_vdsegmt);
        (assign20540_e15264, (-locals.var_vdsegmt_dn0), (locals.var_vsubs_dn2 - locals.var_vdsegmt_dn2), locals.var_vsubs_dn4,)
    } else {
        (locals.var_vsubsrev, locals.var_vsubsrev_dn0, locals.var_vsubsrev_dn2, locals.var_vsubsrev_dn4,)
    }
};
        locals.var_vsubsrev = assign20540_e15266;
        locals.var_vsubsrev_dn0 = assign20540_e15266_d_n0;
        locals.var_vsubsrev_dn2 = assign20540_e15266_d_n2;
        locals.var_vsubsrev_dn4 = assign20540_e15266_d_n4;
        locals.var_vsubsrev_rv = 0.0;

        let assign20550_e15285: f64 = if (((((locals.var_rdvde > 0.0) || (locals.var_rsvde > 0.0)) || (locals.var_uc_rdvg11 > 0.0)) || (locals.var_uc_rdvb > 0.0)) || (p.p54 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard413 = assign20550_e15285;
        locals.var_guard413_rv = 0.0;

        let (assign20560_e15297, assign20560_e15297_d_n0, assign20560_e15297_d_n2, assign20560_e15297_d_n4, assign20560_e15297_d_n5, assign20560_e15297_d_n6, assign20560_e15297_d_n7, assign20560_e15297_d_n8, assign20560_e15297_d_n9, assign20560_e15297_d_n10, assign20560_e15297_d_n11, assign20560_e15297_d_n14,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) {
        let assign20560_e15292: f64 = (locals.var_vdserev / 2.0);
        let assign20560_e15293: f64 = (2.0 * assign20560_e15292);
        let assign20560_e15295: f64 = (assign20560_e15293 / p.p262);
        (assign20560_e15295, ((2.0 * (locals.var_vdserev_dn0 / 2.0)) / p.p262), ((2.0 * (locals.var_vdserev_dn2 / 2.0)) / p.p262), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign20560_e15297;
        locals.var_tmf1_dn0 = assign20560_e15297_d_n0;
        locals.var_tmf1_dn2 = assign20560_e15297_d_n2;
        locals.var_tmf1_dn4 = assign20560_e15297_d_n4;
        locals.var_tmf1_dn5 = assign20560_e15297_d_n5;
        locals.var_tmf1_dn6 = assign20560_e15297_d_n6;
        locals.var_tmf1_dn7 = assign20560_e15297_d_n7;
        locals.var_tmf1_dn8 = assign20560_e15297_d_n8;
        locals.var_tmf1_dn9 = assign20560_e15297_d_n9;
        locals.var_tmf1_dn10 = assign20560_e15297_d_n10;
        locals.var_tmf1_dn11 = assign20560_e15297_d_n11;
        locals.var_tmf1_dn14 = assign20560_e15297_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign20570_e15339, assign20570_e15339_d_n0, assign20570_e15339_d_n2, assign20570_e15339_d_n4, assign20570_e15339_d_n5, assign20570_e15339_d_n6, assign20570_e15339_d_n7, assign20570_e15339_d_n8, assign20570_e15339_d_n9, assign20570_e15339_d_n10, assign20570_e15339_d_n11, assign20570_e15339_d_n14,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) {
        let assign20570_e15305: f64 = (1.0 / 2.0);
        let assign20570_e15309: f64 = (1.0 / 6.0);
        let assign20570_e15313: f64 = (1.0 / 24.0);
        let assign20570_e15317: f64 = (1.0 / 120.0);
        let assign20570_e15321: f64 = (1.0 / 720.0);
        let assign20570_e15325: f64 = (1.0 / 5040.0);
        let assign20570_e15326: f64 = (locals.var_tmf1 * assign20570_e15325);
        let assign20570_e15327: f64 = (assign20570_e15321 + assign20570_e15326);
        let assign20570_e15328: f64 = (locals.var_tmf1 * assign20570_e15327);
        let assign20570_e15329: f64 = (assign20570_e15317 + assign20570_e15328);
        let assign20570_e15330: f64 = (locals.var_tmf1 * assign20570_e15329);
        let assign20570_e15331: f64 = (assign20570_e15313 + assign20570_e15330);
        let assign20570_e15332: f64 = (locals.var_tmf1 * assign20570_e15331);
        let assign20570_e15333: f64 = (assign20570_e15309 + assign20570_e15332);
        let assign20570_e15334: f64 = (locals.var_tmf1 * assign20570_e15333);
        let assign20570_e15335: f64 = (assign20570_e15305 + assign20570_e15334);
        let assign20570_e15336: f64 = (locals.var_tmf1 * assign20570_e15335);
        let assign20570_e15337: f64 = (1.0 + assign20570_e15336);
        (assign20570_e15337, ((locals.var_tmf1_dn0 * assign20570_e15335) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20570_e15333) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20570_e15331) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20570_e15329) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20570_e15327) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign20570_e15325))))))))))), ((locals.var_tmf1_dn2 * assign20570_e15335) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20570_e15333) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20570_e15331) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20570_e15329) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20570_e15327) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign20570_e15325))))))))))), ((locals.var_tmf1_dn4 * assign20570_e15335) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20570_e15333) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20570_e15331) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20570_e15329) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20570_e15327) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign20570_e15325))))))))))), ((locals.var_tmf1_dn5 * assign20570_e15335) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20570_e15333) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20570_e15331) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20570_e15329) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20570_e15327) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign20570_e15325))))))))))), ((locals.var_tmf1_dn6 * assign20570_e15335) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20570_e15333) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20570_e15331) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20570_e15329) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20570_e15327) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign20570_e15325))))))))))), ((locals.var_tmf1_dn7 * assign20570_e15335) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20570_e15333) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20570_e15331) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20570_e15329) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20570_e15327) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign20570_e15325))))))))))), ((locals.var_tmf1_dn8 * assign20570_e15335) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20570_e15333) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20570_e15331) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20570_e15329) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20570_e15327) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign20570_e15325))))))))))), ((locals.var_tmf1_dn9 * assign20570_e15335) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20570_e15333) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20570_e15331) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20570_e15329) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20570_e15327) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign20570_e15325))))))))))), ((locals.var_tmf1_dn10 * assign20570_e15335) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20570_e15333) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20570_e15331) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20570_e15329) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20570_e15327) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign20570_e15325))))))))))), ((locals.var_tmf1_dn11 * assign20570_e15335) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20570_e15333) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20570_e15331) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20570_e15329) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20570_e15327) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign20570_e15325))))))))))), ((locals.var_tmf1_dn14 * assign20570_e15335) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20570_e15333) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20570_e15331) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20570_e15329) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20570_e15327) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign20570_e15325))))))))))),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20570_e15339;
        locals.var_tmf2_dn0 = assign20570_e15339_d_n0;
        locals.var_tmf2_dn2 = assign20570_e15339_d_n2;
        locals.var_tmf2_dn4 = assign20570_e15339_d_n4;
        locals.var_tmf2_dn5 = assign20570_e15339_d_n5;
        locals.var_tmf2_dn6 = assign20570_e15339_d_n6;
        locals.var_tmf2_dn7 = assign20570_e15339_d_n7;
        locals.var_tmf2_dn8 = assign20570_e15339_d_n8;
        locals.var_tmf2_dn9 = assign20570_e15339_d_n9;
        locals.var_tmf2_dn10 = assign20570_e15339_d_n10;
        locals.var_tmf2_dn11 = assign20570_e15339_d_n11;
        locals.var_tmf2_dn14 = assign20570_e15339_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20580_e15377, assign20580_e15377_d_n0, assign20580_e15377_d_n2, assign20580_e15377_d_n4, assign20580_e15377_d_n5, assign20580_e15377_d_n6, assign20580_e15377_d_n7, assign20580_e15377_d_n8, assign20580_e15377_d_n9, assign20580_e15377_d_n10, assign20580_e15377_d_n11, assign20580_e15377_d_n14,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) {
        let assign20580_e15345: f64 = (1.0 / 2.0);
        let assign20580_e15349: f64 = (1.0 / 3.0);
        let assign20580_e15353: f64 = (1.0 / 8.0);
        let assign20580_e15357: f64 = (1.0 / 30.0);
        let assign20580_e15361: f64 = (1.0 / 144.0);
        let assign20580_e15365: f64 = (1.0 / 840.0);
        let assign20580_e15366: f64 = (locals.var_tmf1 * assign20580_e15365);
        let assign20580_e15367: f64 = (assign20580_e15361 + assign20580_e15366);
        let assign20580_e15368: f64 = (locals.var_tmf1 * assign20580_e15367);
        let assign20580_e15369: f64 = (assign20580_e15357 + assign20580_e15368);
        let assign20580_e15370: f64 = (locals.var_tmf1 * assign20580_e15369);
        let assign20580_e15371: f64 = (assign20580_e15353 + assign20580_e15370);
        let assign20580_e15372: f64 = (locals.var_tmf1 * assign20580_e15371);
        let assign20580_e15373: f64 = (assign20580_e15349 + assign20580_e15372);
        let assign20580_e15374: f64 = (locals.var_tmf1 * assign20580_e15373);
        let assign20580_e15375: f64 = (assign20580_e15345 + assign20580_e15374);
        (assign20580_e15375, ((locals.var_tmf1_dn0 * assign20580_e15373) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20580_e15371) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20580_e15369) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign20580_e15367) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign20580_e15365))))))))), ((locals.var_tmf1_dn2 * assign20580_e15373) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20580_e15371) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20580_e15369) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign20580_e15367) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign20580_e15365))))))))), ((locals.var_tmf1_dn4 * assign20580_e15373) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20580_e15371) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20580_e15369) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign20580_e15367) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign20580_e15365))))))))), ((locals.var_tmf1_dn5 * assign20580_e15373) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20580_e15371) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20580_e15369) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign20580_e15367) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign20580_e15365))))))))), ((locals.var_tmf1_dn6 * assign20580_e15373) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20580_e15371) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20580_e15369) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign20580_e15367) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign20580_e15365))))))))), ((locals.var_tmf1_dn7 * assign20580_e15373) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20580_e15371) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20580_e15369) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign20580_e15367) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign20580_e15365))))))))), ((locals.var_tmf1_dn8 * assign20580_e15373) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20580_e15371) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20580_e15369) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign20580_e15367) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign20580_e15365))))))))), ((locals.var_tmf1_dn9 * assign20580_e15373) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20580_e15371) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20580_e15369) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign20580_e15367) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign20580_e15365))))))))), ((locals.var_tmf1_dn10 * assign20580_e15373) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20580_e15371) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20580_e15369) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign20580_e15367) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign20580_e15365))))))))), ((locals.var_tmf1_dn11 * assign20580_e15373) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20580_e15371) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20580_e15369) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign20580_e15367) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign20580_e15365))))))))), ((locals.var_tmf1_dn14 * assign20580_e15373) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20580_e15371) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20580_e15369) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign20580_e15367) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign20580_e15365))))))))),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign20580_e15377;
        locals.var_tmf3_dn0 = assign20580_e15377_d_n0;
        locals.var_tmf3_dn2 = assign20580_e15377_d_n2;
        locals.var_tmf3_dn4 = assign20580_e15377_d_n4;
        locals.var_tmf3_dn5 = assign20580_e15377_d_n5;
        locals.var_tmf3_dn6 = assign20580_e15377_d_n6;
        locals.var_tmf3_dn7 = assign20580_e15377_d_n7;
        locals.var_tmf3_dn8 = assign20580_e15377_d_n8;
        locals.var_tmf3_dn9 = assign20580_e15377_d_n9;
        locals.var_tmf3_dn10 = assign20580_e15377_d_n10;
        locals.var_tmf3_dn11 = assign20580_e15377_d_n11;
        locals.var_tmf3_dn14 = assign20580_e15377_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign20590_e15385, assign20590_e15385_d_n0, assign20590_e15385_d_n2, assign20590_e15385_d_n4, assign20590_e15385_d_n5, assign20590_e15385_d_n6, assign20590_e15385_d_n7, assign20590_e15385_d_n8, assign20590_e15385_d_n9, assign20590_e15385_d_n10, assign20590_e15385_d_n11, assign20590_e15385_d_n14,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) {
        let assign20590_e15383: f64 = (p.p262 / locals.var_tmf2);
        (assign20590_e15383, (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2))), (-((p.p262 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign20590_e15385;
        locals.var_vzadd_dn0 = assign20590_e15385_d_n0;
        locals.var_vzadd_dn2 = assign20590_e15385_d_n2;
        locals.var_vzadd_dn4 = assign20590_e15385_d_n4;
        locals.var_vzadd_dn5 = assign20590_e15385_d_n5;
        locals.var_vzadd_dn6 = assign20590_e15385_d_n6;
        locals.var_vzadd_dn7 = assign20590_e15385_d_n7;
        locals.var_vzadd_dn8 = assign20590_e15385_d_n8;
        locals.var_vzadd_dn9 = assign20590_e15385_d_n9;
        locals.var_vzadd_dn10 = assign20590_e15385_d_n10;
        locals.var_vzadd_dn11 = assign20590_e15385_d_n11;
        locals.var_vzadd_dn14 = assign20590_e15385_d_n14;
        locals.var_vzadd_rv = 0.0;

        let (assign20600_e15398, assign20600_e15398_d_n0, assign20600_e15398_d_n2, assign20600_e15398_d_n4, assign20600_e15398_d_n5, assign20600_e15398_d_n6, assign20600_e15398_d_n7, assign20600_e15398_d_n8, assign20600_e15398_d_n9, assign20600_e15398_d_n10, assign20600_e15398_d_n11, assign20600_e15398_d_n14,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) {
        let assign20600_e15390: f64 = (-2.0);
        let assign20600_e15392: f64 = (assign20600_e15390 * locals.var_tmf3);
        let assign20600_e15395: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign20600_e15396: f64 = (assign20600_e15392 / assign20600_e15395);
        (assign20600_e15396, ((((assign20600_e15390 * locals.var_tmf3_dn0) * assign20600_e15395) - (assign20600_e15392 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign20600_e15395 * assign20600_e15395)), ((((assign20600_e15390 * locals.var_tmf3_dn2) * assign20600_e15395) - (assign20600_e15392 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign20600_e15395 * assign20600_e15395)), ((((assign20600_e15390 * locals.var_tmf3_dn4) * assign20600_e15395) - (assign20600_e15392 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign20600_e15395 * assign20600_e15395)), ((((assign20600_e15390 * locals.var_tmf3_dn5) * assign20600_e15395) - (assign20600_e15392 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign20600_e15395 * assign20600_e15395)), ((((assign20600_e15390 * locals.var_tmf3_dn6) * assign20600_e15395) - (assign20600_e15392 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign20600_e15395 * assign20600_e15395)), ((((assign20600_e15390 * locals.var_tmf3_dn7) * assign20600_e15395) - (assign20600_e15392 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign20600_e15395 * assign20600_e15395)), ((((assign20600_e15390 * locals.var_tmf3_dn8) * assign20600_e15395) - (assign20600_e15392 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign20600_e15395 * assign20600_e15395)), ((((assign20600_e15390 * locals.var_tmf3_dn9) * assign20600_e15395) - (assign20600_e15392 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign20600_e15395 * assign20600_e15395)), ((((assign20600_e15390 * locals.var_tmf3_dn10) * assign20600_e15395) - (assign20600_e15392 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign20600_e15395 * assign20600_e15395)), ((((assign20600_e15390 * locals.var_tmf3_dn11) * assign20600_e15395) - (assign20600_e15392 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign20600_e15395 * assign20600_e15395)), ((((assign20600_e15390 * locals.var_tmf3_dn14) * assign20600_e15395) - (assign20600_e15392 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign20600_e15395 * assign20600_e15395)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign20600_e15398;
        locals.var_t2_dn0 = assign20600_e15398_d_n0;
        locals.var_t2_dn2 = assign20600_e15398_d_n2;
        locals.var_t2_dn4 = assign20600_e15398_d_n4;
        locals.var_t2_dn5 = assign20600_e15398_d_n5;
        locals.var_t2_dn6 = assign20600_e15398_d_n6;
        locals.var_t2_dn7 = assign20600_e15398_d_n7;
        locals.var_t2_dn8 = assign20600_e15398_d_n8;
        locals.var_t2_dn9 = assign20600_e15398_d_n9;
        locals.var_t2_dn10 = assign20600_e15398_d_n10;
        locals.var_t2_dn11 = assign20600_e15398_d_n11;
        locals.var_t2_dn14 = assign20600_e15398_d_n14;
        locals.var_t2_rv = 0.0;

        let assign20610_e15401: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard414 = assign20610_e15401;
        locals.var_guard414_rv = 0.0;

        let (assign20620_e15409, assign20620_e15409_d_n0, assign20620_e15409_d_n2, assign20620_e15409_d_n4, assign20620_e15409_d_n5, assign20620_e15409_d_n6, assign20620_e15409_d_n7, assign20620_e15409_d_n8, assign20620_e15409_d_n9, assign20620_e15409_d_n10, assign20620_e15409_d_n11, assign20620_e15409_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard414 != 0.0)) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign20620_e15409;
        locals.var_vzadd_dn0 = assign20620_e15409_d_n0;
        locals.var_vzadd_dn2 = assign20620_e15409_d_n2;
        locals.var_vzadd_dn4 = assign20620_e15409_d_n4;
        locals.var_vzadd_dn5 = assign20620_e15409_d_n5;
        locals.var_vzadd_dn6 = assign20620_e15409_d_n6;
        locals.var_vzadd_dn7 = assign20620_e15409_d_n7;
        locals.var_vzadd_dn8 = assign20620_e15409_d_n8;
        locals.var_vzadd_dn9 = assign20620_e15409_d_n9;
        locals.var_vzadd_dn10 = assign20620_e15409_d_n10;
        locals.var_vzadd_dn11 = assign20620_e15409_d_n11;
        locals.var_vzadd_dn14 = assign20620_e15409_d_n14;
        locals.var_vzadd_rv = 0.0;

        let (assign20630_e15419, assign20630_e15419_d_n0, assign20630_e15419_d_n2, assign20630_e15419_d_n4, assign20630_e15419_d_n5, assign20630_e15419_d_n6, assign20630_e15419_d_n7, assign20630_e15419_d_n8, assign20630_e15419_d_n9, assign20630_e15419_d_n10, assign20630_e15419_d_n11, assign20630_e15419_d_n14,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) {
        let assign20630_e15416: f64 = (2.0 * locals.var_vzadd);
        let assign20630_e15417: f64 = (locals.var_vdserev + assign20630_e15416);
        (assign20630_e15417, (locals.var_vdserev_dn0 + (2.0 * locals.var_vzadd_dn0)), (locals.var_vdserev_dn2 + (2.0 * locals.var_vzadd_dn2)), (2.0 * locals.var_vzadd_dn4), (2.0 * locals.var_vzadd_dn5), (2.0 * locals.var_vzadd_dn6), (2.0 * locals.var_vzadd_dn7), (2.0 * locals.var_vzadd_dn8), (2.0 * locals.var_vzadd_dn9), (2.0 * locals.var_vzadd_dn10), (2.0 * locals.var_vzadd_dn11), (2.0 * locals.var_vzadd_dn14),)
    } else {
        (locals.var_vdserevz, locals.var_vdserevz_dn0, locals.var_vdserevz_dn2, locals.var_vdserevz_dn4, locals.var_vdserevz_dn5, locals.var_vdserevz_dn6, locals.var_vdserevz_dn7, locals.var_vdserevz_dn8, locals.var_vdserevz_dn9, locals.var_vdserevz_dn10, locals.var_vdserevz_dn11, locals.var_vdserevz_dn14,)
    }
};
        locals.var_vdserevz = assign20630_e15419;
        locals.var_vdserevz_dn0 = assign20630_e15419_d_n0;
        locals.var_vdserevz_dn2 = assign20630_e15419_d_n2;
        locals.var_vdserevz_dn4 = assign20630_e15419_d_n4;
        locals.var_vdserevz_dn5 = assign20630_e15419_d_n5;
        locals.var_vdserevz_dn6 = assign20630_e15419_d_n6;
        locals.var_vdserevz_dn7 = assign20630_e15419_d_n7;
        locals.var_vdserevz_dn8 = assign20630_e15419_d_n8;
        locals.var_vdserevz_dn9 = assign20630_e15419_d_n9;
        locals.var_vdserevz_dn10 = assign20630_e15419_d_n10;
        locals.var_vdserevz_dn11 = assign20630_e15419_d_n11;
        locals.var_vdserevz_dn14 = assign20630_e15419_d_n14;
        locals.var_vdserevz_rv = 0.0;

        let (assign20640_e15427, assign20640_e15427_d_n0, assign20640_e15427_d_n2, assign20640_e15427_d_n4, assign20640_e15427_d_n5, assign20640_e15427_d_n6, assign20640_e15427_d_n7, assign20640_e15427_d_n8, assign20640_e15427_d_n9, assign20640_e15427_d_n10, assign20640_e15427_d_n11, assign20640_e15427_d_n14,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) {
        let assign20640_e15425: f64 = (locals.var_vgserev + locals.var_vzadd);
        (assign20640_e15425, (locals.var_vgserev_dn0 + locals.var_vzadd_dn0), (locals.var_vgserev_dn2 + locals.var_vzadd_dn2), locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, (locals.var_vgserev_dn7 + locals.var_vzadd_dn7), locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    } else {
        (locals.var_vgserevz, locals.var_vgserevz_dn0, locals.var_vgserevz_dn2, locals.var_vgserevz_dn4, locals.var_vgserevz_dn5, locals.var_vgserevz_dn6, locals.var_vgserevz_dn7, locals.var_vgserevz_dn8, locals.var_vgserevz_dn9, locals.var_vgserevz_dn10, locals.var_vgserevz_dn11, locals.var_vgserevz_dn14,)
    }
};
        locals.var_vgserevz = assign20640_e15427;
        locals.var_vgserevz_dn0 = assign20640_e15427_d_n0;
        locals.var_vgserevz_dn2 = assign20640_e15427_d_n2;
        locals.var_vgserevz_dn4 = assign20640_e15427_d_n4;
        locals.var_vgserevz_dn5 = assign20640_e15427_d_n5;
        locals.var_vgserevz_dn6 = assign20640_e15427_d_n6;
        locals.var_vgserevz_dn7 = assign20640_e15427_d_n7;
        locals.var_vgserevz_dn8 = assign20640_e15427_d_n8;
        locals.var_vgserevz_dn9 = assign20640_e15427_d_n9;
        locals.var_vgserevz_dn10 = assign20640_e15427_d_n10;
        locals.var_vgserevz_dn11 = assign20640_e15427_d_n11;
        locals.var_vgserevz_dn14 = assign20640_e15427_d_n14;
        locals.var_vgserevz_rv = 0.0;

        let (assign20650_e15435, assign20650_e15435_d_n0, assign20650_e15435_d_n2, assign20650_e15435_d_n4, assign20650_e15435_d_n5, assign20650_e15435_d_n6, assign20650_e15435_d_n7, assign20650_e15435_d_n8, assign20650_e15435_d_n9, assign20650_e15435_d_n10, assign20650_e15435_d_n11, assign20650_e15435_d_n14,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) {
        let assign20650_e15433: f64 = (locals.var_vbserev + locals.var_vzadd);
        (assign20650_e15433, (locals.var_vbserev_dn0 + locals.var_vzadd_dn0), (locals.var_vbserev_dn2 + locals.var_vzadd_dn2), locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, (locals.var_vbserev_dn9 + locals.var_vzadd_dn9), locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    } else {
        (locals.var_vbserevz, locals.var_vbserevz_dn0, locals.var_vbserevz_dn2, locals.var_vbserevz_dn4, locals.var_vbserevz_dn5, locals.var_vbserevz_dn6, locals.var_vbserevz_dn7, locals.var_vbserevz_dn8, locals.var_vbserevz_dn9, locals.var_vbserevz_dn10, locals.var_vbserevz_dn11, locals.var_vbserevz_dn14,)
    }
};
        locals.var_vbserevz = assign20650_e15435;
        locals.var_vbserevz_dn0 = assign20650_e15435_d_n0;
        locals.var_vbserevz_dn2 = assign20650_e15435_d_n2;
        locals.var_vbserevz_dn4 = assign20650_e15435_d_n4;
        locals.var_vbserevz_dn5 = assign20650_e15435_d_n5;
        locals.var_vbserevz_dn6 = assign20650_e15435_d_n6;
        locals.var_vbserevz_dn7 = assign20650_e15435_d_n7;
        locals.var_vbserevz_dn8 = assign20650_e15435_d_n8;
        locals.var_vbserevz_dn9 = assign20650_e15435_d_n9;
        locals.var_vbserevz_dn10 = assign20650_e15435_d_n10;
        locals.var_vbserevz_dn11 = assign20650_e15435_d_n11;
        locals.var_vbserevz_dn14 = assign20650_e15435_d_n14;
        locals.var_vbserevz_rv = 0.0;

        let assign20660_e15442: f64 = if ((p.p34 == 1.0) || (locals.var_vdsemodenml == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard415 = assign20660_e15442;
        locals.var_guard415_rv = 0.0;

        let (assign20670_e15456, assign20670_e15456_d_n0, assign20670_e15456_d_n2, assign20670_e15456_d_n4, assign20670_e15456_d_n5, assign20670_e15456_d_n6, assign20670_e15456_d_n7, assign20670_e15456_d_n8, assign20670_e15456_d_n9, assign20670_e15456_d_n10, assign20670_e15456_d_n11, assign20670_e15456_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20670_e15450: f64 = (locals.var_vdsemodenml * locals.var_rde);
        let assign20670_e15453: f64 = (locals.var_vdsemodervs * locals.var_rse);
        let assign20670_e15454: f64 = (assign20670_e15450 + assign20670_e15453);
        (assign20670_e15454, ((locals.var_vdsemodenml * locals.var_rde_dn0) + (locals.var_vdsemodervs * locals.var_rse_dn0)), ((locals.var_vdsemodenml * locals.var_rde_dn2) + (locals.var_vdsemodervs * locals.var_rse_dn2)), ((locals.var_vdsemodenml * locals.var_rde_dn4) + (locals.var_vdsemodervs * locals.var_rse_dn4)), ((locals.var_vdsemodenml * locals.var_rde_dn5) + (locals.var_vdsemodervs * locals.var_rse_dn5)), ((locals.var_vdsemodenml * locals.var_rde_dn6) + (locals.var_vdsemodervs * locals.var_rse_dn6)), ((locals.var_vdsemodenml * locals.var_rde_dn7) + (locals.var_vdsemodervs * locals.var_rse_dn7)), ((locals.var_vdsemodenml * locals.var_rde_dn8) + (locals.var_vdsemodervs * locals.var_rse_dn8)), ((locals.var_vdsemodenml * locals.var_rde_dn9) + (locals.var_vdsemodervs * locals.var_rse_dn9)), ((locals.var_vdsemodenml * locals.var_rde_dn10) + (locals.var_vdsemodervs * locals.var_rse_dn10)), ((locals.var_vdsemodenml * locals.var_rde_dn11) + (locals.var_vdsemodervs * locals.var_rse_dn11)), ((locals.var_vdsemodenml * locals.var_rde_dn14) + (locals.var_vdsemodervs * locals.var_rse_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign20670_e15456;
        locals.var_t1_dn0 = assign20670_e15456_d_n0;
        locals.var_t1_dn2 = assign20670_e15456_d_n2;
        locals.var_t1_dn4 = assign20670_e15456_d_n4;
        locals.var_t1_dn5 = assign20670_e15456_d_n5;
        locals.var_t1_dn6 = assign20670_e15456_d_n6;
        locals.var_t1_dn7 = assign20670_e15456_d_n7;
        locals.var_t1_dn8 = assign20670_e15456_d_n8;
        locals.var_t1_dn9 = assign20670_e15456_d_n9;
        locals.var_t1_dn10 = assign20670_e15456_d_n10;
        locals.var_t1_dn11 = assign20670_e15456_d_n11;
        locals.var_t1_dn14 = assign20670_e15456_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign20680_e15470, assign20680_e15470_d_n0, assign20680_e15470_d_n2, assign20680_e15470_d_n4, assign20680_e15470_d_n5, assign20680_e15470_d_n6, assign20680_e15470_d_n7, assign20680_e15470_d_n8, assign20680_e15470_d_n9, assign20680_e15470_d_n10, assign20680_e15470_d_n11, assign20680_e15470_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20680_e15464: f64 = (locals.var_vdsemodenml * locals.var_rdvde);
        let assign20680_e15467: f64 = (locals.var_vdsemodervs * locals.var_rsvde);
        let assign20680_e15468: f64 = (assign20680_e15464 + assign20680_e15467);
        (assign20680_e15468, ((locals.var_vdsemodenml * locals.var_rdvde_dn0) + (locals.var_vdsemodervs * locals.var_rsvde_dn0)), ((locals.var_vdsemodenml * locals.var_rdvde_dn2) + (locals.var_vdsemodervs * locals.var_rsvde_dn2)), ((locals.var_vdsemodenml * locals.var_rdvde_dn4) + (locals.var_vdsemodervs * locals.var_rsvde_dn4)), ((locals.var_vdsemodenml * locals.var_rdvde_dn5) + (locals.var_vdsemodervs * locals.var_rsvde_dn5)), ((locals.var_vdsemodenml * locals.var_rdvde_dn6) + (locals.var_vdsemodervs * locals.var_rsvde_dn6)), ((locals.var_vdsemodenml * locals.var_rdvde_dn7) + (locals.var_vdsemodervs * locals.var_rsvde_dn7)), ((locals.var_vdsemodenml * locals.var_rdvde_dn8) + (locals.var_vdsemodervs * locals.var_rsvde_dn8)), ((locals.var_vdsemodenml * locals.var_rdvde_dn9) + (locals.var_vdsemodervs * locals.var_rsvde_dn9)), ((locals.var_vdsemodenml * locals.var_rdvde_dn10) + (locals.var_vdsemodervs * locals.var_rsvde_dn10)), ((locals.var_vdsemodenml * locals.var_rdvde_dn11) + (locals.var_vdsemodervs * locals.var_rsvde_dn11)), ((locals.var_vdsemodenml * locals.var_rdvde_dn14) + (locals.var_vdsemodervs * locals.var_rsvde_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20680_e15470;
        locals.var_t0_dn0 = assign20680_e15470_d_n0;
        locals.var_t0_dn2 = assign20680_e15470_d_n2;
        locals.var_t0_dn4 = assign20680_e15470_d_n4;
        locals.var_t0_dn5 = assign20680_e15470_d_n5;
        locals.var_t0_dn6 = assign20680_e15470_d_n6;
        locals.var_t0_dn7 = assign20680_e15470_d_n7;
        locals.var_t0_dn8 = assign20680_e15470_d_n8;
        locals.var_t0_dn9 = assign20680_e15470_d_n9;
        locals.var_t0_dn10 = assign20680_e15470_d_n10;
        locals.var_t0_dn11 = assign20680_e15470_d_n11;
        locals.var_t0_dn14 = assign20680_e15470_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign20690_e15482, assign20690_e15482_d_n0, assign20690_e15482_d_n2, assign20690_e15482_d_n4, assign20690_e15482_d_n5, assign20690_e15482_d_n6, assign20690_e15482_d_n7, assign20690_e15482_d_n8, assign20690_e15482_d_n9, assign20690_e15482_d_n10, assign20690_e15482_d_n11, assign20690_e15482_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20690_e15479: f64 = (locals.var_t0 * locals.var_vdserevz);
        let assign20690_e15480: f64 = (locals.var_t1 + assign20690_e15479);
        (assign20690_e15480, (locals.var_t1_dn0 + ((locals.var_t0_dn0 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn0))), (locals.var_t1_dn2 + ((locals.var_t0_dn2 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn2))), (locals.var_t1_dn4 + ((locals.var_t0_dn4 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn4))), (locals.var_t1_dn5 + ((locals.var_t0_dn5 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn5))), (locals.var_t1_dn6 + ((locals.var_t0_dn6 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn6))), (locals.var_t1_dn7 + ((locals.var_t0_dn7 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn7))), (locals.var_t1_dn8 + ((locals.var_t0_dn8 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn8))), (locals.var_t1_dn9 + ((locals.var_t0_dn9 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn9))), (locals.var_t1_dn10 + ((locals.var_t0_dn10 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn10))), (locals.var_t1_dn11 + ((locals.var_t0_dn11 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn11))), (locals.var_t1_dn14 + ((locals.var_t0_dn14 * locals.var_vdserevz) + (locals.var_t0 * locals.var_vdserevz_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign20690_e15482;
        locals.var_t4_dn0 = assign20690_e15482_d_n0;
        locals.var_t4_dn2 = assign20690_e15482_d_n2;
        locals.var_t4_dn4 = assign20690_e15482_d_n4;
        locals.var_t4_dn5 = assign20690_e15482_d_n5;
        locals.var_t4_dn6 = assign20690_e15482_d_n6;
        locals.var_t4_dn7 = assign20690_e15482_d_n7;
        locals.var_t4_dn8 = assign20690_e15482_d_n8;
        locals.var_t4_dn9 = assign20690_e15482_d_n9;
        locals.var_t4_dn10 = assign20690_e15482_d_n10;
        locals.var_t4_dn11 = assign20690_e15482_d_n11;
        locals.var_t4_dn14 = assign20690_e15482_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign20700_e15503, assign20700_e15503_d_n0, assign20700_e15503_d_n2, assign20700_e15503_d_n4, assign20700_e15503_d_n5, assign20700_e15503_d_n6, assign20700_e15503_d_n7, assign20700_e15503_d_n8, assign20700_e15503_d_n9, assign20700_e15503_d_n10, assign20700_e15503_d_n11, assign20700_e15503_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20700_e15490: f64 = (p.p292 * p.p292);
        let assign20700_e15494: f64 = (0.0001 * 0.01);
        let assign20700_e15495: f64 = (4.0 * assign20700_e15494);
        let assign20700_e15498: f64 = (0.0001 * 0.01);
        let assign20700_e15499: f64 = (assign20700_e15495 * assign20700_e15498);
        let assign20700_e15500: f64 = (assign20700_e15490 + assign20700_e15499);
        let assign20700_e15501: f64 = (assign20700_e15500).sqrt();
        (assign20700_e15501, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20700_e15503;
        locals.var_tmf2_dn0 = assign20700_e15503_d_n0;
        locals.var_tmf2_dn2 = assign20700_e15503_d_n2;
        locals.var_tmf2_dn4 = assign20700_e15503_d_n4;
        locals.var_tmf2_dn5 = assign20700_e15503_d_n5;
        locals.var_tmf2_dn6 = assign20700_e15503_d_n6;
        locals.var_tmf2_dn7 = assign20700_e15503_d_n7;
        locals.var_tmf2_dn8 = assign20700_e15503_d_n8;
        locals.var_tmf2_dn9 = assign20700_e15503_d_n9;
        locals.var_tmf2_dn10 = assign20700_e15503_d_n10;
        locals.var_tmf2_dn11 = assign20700_e15503_d_n11;
        locals.var_tmf2_dn14 = assign20700_e15503_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20710_e15517, assign20710_e15517_d_n0, assign20710_e15517_d_n2, assign20710_e15517_d_n4, assign20710_e15517_d_n5, assign20710_e15517_d_n6, assign20710_e15517_d_n7, assign20710_e15517_d_n8, assign20710_e15517_d_n9, assign20710_e15517_d_n10, assign20710_e15517_d_n11, assign20710_e15517_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20710_e15513: f64 = (p.p292 / locals.var_tmf2);
        let assign20710_e15514: f64 = (1.0 + assign20710_e15513);
        let assign20710_e15515: f64 = (0.5 * assign20710_e15514);
        (assign20710_e15515, (0.5 * (-((p.p292 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p292 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20710_e15517;
        locals.var_t0_dn0 = assign20710_e15517_d_n0;
        locals.var_t0_dn2 = assign20710_e15517_d_n2;
        locals.var_t0_dn4 = assign20710_e15517_d_n4;
        locals.var_t0_dn5 = assign20710_e15517_d_n5;
        locals.var_t0_dn6 = assign20710_e15517_d_n6;
        locals.var_t0_dn7 = assign20710_e15517_d_n7;
        locals.var_t0_dn8 = assign20710_e15517_d_n8;
        locals.var_t0_dn9 = assign20710_e15517_d_n9;
        locals.var_t0_dn10 = assign20710_e15517_d_n10;
        locals.var_t0_dn11 = assign20710_e15517_d_n11;
        locals.var_t0_dn14 = assign20710_e15517_d_n14;
        locals.var_t0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_53(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20720_e15529, assign20720_e15529_d_n0, assign20720_e15529_d_n2, assign20720_e15529_d_n4, assign20720_e15529_d_n5, assign20720_e15529_d_n6, assign20720_e15529_d_n7, assign20720_e15529_d_n8, assign20720_e15529_d_n9, assign20720_e15529_d_n10, assign20720_e15529_d_n11, assign20720_e15529_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20720_e15526: f64 = (p.p292 + locals.var_tmf2);
        let assign20720_e15527: f64 = (0.5 * assign20720_e15526);
        (assign20720_e15527, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * locals.var_tmf2_dn6), (0.5 * locals.var_tmf2_dn7), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign20720_e15529;
        locals.var_t10_dn0 = assign20720_e15529_d_n0;
        locals.var_t10_dn2 = assign20720_e15529_d_n2;
        locals.var_t10_dn4 = assign20720_e15529_d_n4;
        locals.var_t10_dn5 = assign20720_e15529_d_n5;
        locals.var_t10_dn6 = assign20720_e15529_d_n6;
        locals.var_t10_dn7 = assign20720_e15529_d_n7;
        locals.var_t10_dn8 = assign20720_e15529_d_n8;
        locals.var_t10_dn9 = assign20720_e15529_d_n9;
        locals.var_t10_dn10 = assign20720_e15529_d_n10;
        locals.var_t10_dn11 = assign20720_e15529_d_n11;
        locals.var_t10_dn14 = assign20720_e15529_d_n14;
        locals.var_t10_rv = 0.0;

        let assign20730_e15532: f64 = if locals.var_t10 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard416 = assign20730_e15532;
        locals.var_guard416_rv = 0.0;

        let (assign20740_e15542, assign20740_e15542_d_n0, assign20740_e15542_d_n2, assign20740_e15542_d_n4, assign20740_e15542_d_n5, assign20740_e15542_d_n6, assign20740_e15542_d_n7, assign20740_e15542_d_n8, assign20740_e15542_d_n9, assign20740_e15542_d_n10, assign20740_e15542_d_n11, assign20740_e15542_d_n14,) = {
    if ((((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) && (locals.var_guard416 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign20740_e15542;
        locals.var_t10_dn0 = assign20740_e15542_d_n0;
        locals.var_t10_dn2 = assign20740_e15542_d_n2;
        locals.var_t10_dn4 = assign20740_e15542_d_n4;
        locals.var_t10_dn5 = assign20740_e15542_d_n5;
        locals.var_t10_dn6 = assign20740_e15542_d_n6;
        locals.var_t10_dn7 = assign20740_e15542_d_n7;
        locals.var_t10_dn8 = assign20740_e15542_d_n8;
        locals.var_t10_dn9 = assign20740_e15542_d_n9;
        locals.var_t10_dn10 = assign20740_e15542_d_n10;
        locals.var_t10_dn11 = assign20740_e15542_d_n11;
        locals.var_t10_dn14 = assign20740_e15542_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign20750_e15552, assign20750_e15552_d_n0, assign20750_e15552_d_n2, assign20750_e15552_d_n4, assign20750_e15552_d_n5, assign20750_e15552_d_n6, assign20750_e15552_d_n7, assign20750_e15552_d_n8, assign20750_e15552_d_n9, assign20750_e15552_d_n10, assign20750_e15552_d_n11, assign20750_e15552_d_n14,) = {
    if ((((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) && (locals.var_guard416 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20750_e15552;
        locals.var_t0_dn0 = assign20750_e15552_d_n0;
        locals.var_t0_dn2 = assign20750_e15552_d_n2;
        locals.var_t0_dn4 = assign20750_e15552_d_n4;
        locals.var_t0_dn5 = assign20750_e15552_d_n5;
        locals.var_t0_dn6 = assign20750_e15552_d_n6;
        locals.var_t0_dn7 = assign20750_e15552_d_n7;
        locals.var_t0_dn8 = assign20750_e15552_d_n8;
        locals.var_t0_dn9 = assign20750_e15552_d_n9;
        locals.var_t0_dn10 = assign20750_e15552_d_n10;
        locals.var_t0_dn11 = assign20750_e15552_d_n11;
        locals.var_t0_dn14 = assign20750_e15552_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign20760_e15570, assign20760_e15570_d_n0, assign20760_e15570_d_n2, assign20760_e15570_d_n4, assign20760_e15570_d_n5, assign20760_e15570_d_n6, assign20760_e15570_d_n7, assign20760_e15570_d_n8, assign20760_e15570_d_n9, assign20760_e15570_d_n10, assign20760_e15570_d_n11, assign20760_e15570_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20760_e15564: f64 = (locals.var_vgserevz / locals.var_t10);
        let assign20760_e15565: f64 = (1.0 - assign20760_e15564);
        let assign20760_e15566: f64 = (locals.var_uc_rdvg11 * assign20760_e15565);
        let assign20760_e15567: f64 = (1.0 + assign20760_e15566);
        let assign20760_e15568: f64 = (locals.var_t4 * assign20760_e15567);
        (assign20760_e15568, ((locals.var_t4_dn0 * assign20760_e15567) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn0 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn2 * assign20760_e15567) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn2 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn4 * assign20760_e15567) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn4 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn5 * assign20760_e15567) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn5 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn6 * assign20760_e15567) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn6 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn7 * assign20760_e15567) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn7 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn8 * assign20760_e15567) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn8 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn9 * assign20760_e15567) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn9 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn10 * assign20760_e15567) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn10 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn11 * assign20760_e15567) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn11 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn11)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn14 * assign20760_e15567) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn14 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn14)) / (locals.var_t10 * locals.var_t10)))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign20760_e15570;
        locals.var_t1_dn0 = assign20760_e15570_d_n0;
        locals.var_t1_dn2 = assign20760_e15570_d_n2;
        locals.var_t1_dn4 = assign20760_e15570_d_n4;
        locals.var_t1_dn5 = assign20760_e15570_d_n5;
        locals.var_t1_dn6 = assign20760_e15570_d_n6;
        locals.var_t1_dn7 = assign20760_e15570_d_n7;
        locals.var_t1_dn8 = assign20760_e15570_d_n8;
        locals.var_t1_dn9 = assign20760_e15570_d_n9;
        locals.var_t1_dn10 = assign20760_e15570_d_n10;
        locals.var_t1_dn11 = assign20760_e15570_d_n11;
        locals.var_t1_dn14 = assign20760_e15570_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign20770_e15584, assign20770_e15584_d_n0, assign20770_e15584_d_n2, assign20770_e15584_d_n4, assign20770_e15584_d_n5, assign20770_e15584_d_n6, assign20770_e15584_d_n7, assign20770_e15584_d_n8, assign20770_e15584_d_n9, assign20770_e15584_d_n10, assign20770_e15584_d_n11, assign20770_e15584_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20770_e15578: f64 = (locals.var_t1 - locals.var_t4);
        let assign20770_e15581: f64 = (0.01 * 0.01);
        let assign20770_e15582: f64 = (assign20770_e15578 - assign20770_e15581);
        (assign20770_e15582, (locals.var_t1_dn0 - locals.var_t4_dn0), (locals.var_t1_dn2 - locals.var_t4_dn2), (locals.var_t1_dn4 - locals.var_t4_dn4), (locals.var_t1_dn5 - locals.var_t4_dn5), (locals.var_t1_dn6 - locals.var_t4_dn6), (locals.var_t1_dn7 - locals.var_t4_dn7), (locals.var_t1_dn8 - locals.var_t4_dn8), (locals.var_t1_dn9 - locals.var_t4_dn9), (locals.var_t1_dn10 - locals.var_t4_dn10), (locals.var_t1_dn11 - locals.var_t4_dn11), (locals.var_t1_dn14 - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign20770_e15584;
        locals.var_tmf1_dn0 = assign20770_e15584_d_n0;
        locals.var_tmf1_dn2 = assign20770_e15584_d_n2;
        locals.var_tmf1_dn4 = assign20770_e15584_d_n4;
        locals.var_tmf1_dn5 = assign20770_e15584_d_n5;
        locals.var_tmf1_dn6 = assign20770_e15584_d_n6;
        locals.var_tmf1_dn7 = assign20770_e15584_d_n7;
        locals.var_tmf1_dn8 = assign20770_e15584_d_n8;
        locals.var_tmf1_dn9 = assign20770_e15584_d_n9;
        locals.var_tmf1_dn10 = assign20770_e15584_d_n10;
        locals.var_tmf1_dn11 = assign20770_e15584_d_n11;
        locals.var_tmf1_dn14 = assign20770_e15584_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign20780_e15598, assign20780_e15598_d_n0, assign20780_e15598_d_n2, assign20780_e15598_d_n4, assign20780_e15598_d_n5, assign20780_e15598_d_n6, assign20780_e15598_d_n7, assign20780_e15598_d_n8, assign20780_e15598_d_n9, assign20780_e15598_d_n10, assign20780_e15598_d_n11, assign20780_e15598_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20780_e15592: f64 = (4.0 * locals.var_t4);
        let assign20780_e15595: f64 = (0.01 * 0.01);
        let assign20780_e15596: f64 = (assign20780_e15592 * assign20780_e15595);
        (assign20780_e15596, ((4.0 * locals.var_t4_dn0) * assign20780_e15595), ((4.0 * locals.var_t4_dn2) * assign20780_e15595), ((4.0 * locals.var_t4_dn4) * assign20780_e15595), ((4.0 * locals.var_t4_dn5) * assign20780_e15595), ((4.0 * locals.var_t4_dn6) * assign20780_e15595), ((4.0 * locals.var_t4_dn7) * assign20780_e15595), ((4.0 * locals.var_t4_dn8) * assign20780_e15595), ((4.0 * locals.var_t4_dn9) * assign20780_e15595), ((4.0 * locals.var_t4_dn10) * assign20780_e15595), ((4.0 * locals.var_t4_dn11) * assign20780_e15595), ((4.0 * locals.var_t4_dn14) * assign20780_e15595),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20780_e15598;
        locals.var_tmf2_dn0 = assign20780_e15598_d_n0;
        locals.var_tmf2_dn2 = assign20780_e15598_d_n2;
        locals.var_tmf2_dn4 = assign20780_e15598_d_n4;
        locals.var_tmf2_dn5 = assign20780_e15598_d_n5;
        locals.var_tmf2_dn6 = assign20780_e15598_d_n6;
        locals.var_tmf2_dn7 = assign20780_e15598_d_n7;
        locals.var_tmf2_dn8 = assign20780_e15598_d_n8;
        locals.var_tmf2_dn9 = assign20780_e15598_d_n9;
        locals.var_tmf2_dn10 = assign20780_e15598_d_n10;
        locals.var_tmf2_dn11 = assign20780_e15598_d_n11;
        locals.var_tmf2_dn14 = assign20780_e15598_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20790_e15612, assign20790_e15612_d_n0, assign20790_e15612_d_n2, assign20790_e15612_d_n4, assign20790_e15612_d_n5, assign20790_e15612_d_n6, assign20790_e15612_d_n7, assign20790_e15612_d_n8, assign20790_e15612_d_n9, assign20790_e15612_d_n10, assign20790_e15612_d_n11, assign20790_e15612_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let (assign20790_e15610, assign20790_e15610_d_n0, assign20790_e15610_d_n2, assign20790_e15610_d_n4, assign20790_e15610_d_n5, assign20790_e15610_d_n6, assign20790_e15610_d_n7, assign20790_e15610_d_n8, assign20790_e15610_d_n9, assign20790_e15610_d_n10, assign20790_e15610_d_n11, assign20790_e15610_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign20790_e15609: f64 = (-locals.var_tmf2);
                (assign20790_e15609, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign20790_e15610, assign20790_e15610_d_n0, assign20790_e15610_d_n2, assign20790_e15610_d_n4, assign20790_e15610_d_n5, assign20790_e15610_d_n6, assign20790_e15610_d_n7, assign20790_e15610_d_n8, assign20790_e15610_d_n9, assign20790_e15610_d_n10, assign20790_e15610_d_n11, assign20790_e15610_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20790_e15612;
        locals.var_tmf2_dn0 = assign20790_e15612_d_n0;
        locals.var_tmf2_dn2 = assign20790_e15612_d_n2;
        locals.var_tmf2_dn4 = assign20790_e15612_d_n4;
        locals.var_tmf2_dn5 = assign20790_e15612_d_n5;
        locals.var_tmf2_dn6 = assign20790_e15612_d_n6;
        locals.var_tmf2_dn7 = assign20790_e15612_d_n7;
        locals.var_tmf2_dn8 = assign20790_e15612_d_n8;
        locals.var_tmf2_dn9 = assign20790_e15612_d_n9;
        locals.var_tmf2_dn10 = assign20790_e15612_d_n10;
        locals.var_tmf2_dn11 = assign20790_e15612_d_n11;
        locals.var_tmf2_dn14 = assign20790_e15612_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20800_e15625, assign20800_e15625_d_n0, assign20800_e15625_d_n2, assign20800_e15625_d_n4, assign20800_e15625_d_n5, assign20800_e15625_d_n6, assign20800_e15625_d_n7, assign20800_e15625_d_n8, assign20800_e15625_d_n9, assign20800_e15625_d_n10, assign20800_e15625_d_n11, assign20800_e15625_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20800_e15620: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20800_e15622: f64 = (assign20800_e15620 + locals.var_tmf2);
        let assign20800_e15623: f64 = (assign20800_e15622).sqrt();
        (assign20800_e15623, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20800_e15623)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20800_e15623)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign20800_e15623)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign20800_e15623)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20800_e15623)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign20800_e15623)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign20800_e15623)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign20800_e15623)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20800_e15623)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign20800_e15623)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign20800_e15623)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20800_e15625;
        locals.var_tmf2_dn0 = assign20800_e15625_d_n0;
        locals.var_tmf2_dn2 = assign20800_e15625_d_n2;
        locals.var_tmf2_dn4 = assign20800_e15625_d_n4;
        locals.var_tmf2_dn5 = assign20800_e15625_d_n5;
        locals.var_tmf2_dn6 = assign20800_e15625_d_n6;
        locals.var_tmf2_dn7 = assign20800_e15625_d_n7;
        locals.var_tmf2_dn8 = assign20800_e15625_d_n8;
        locals.var_tmf2_dn9 = assign20800_e15625_d_n9;
        locals.var_tmf2_dn10 = assign20800_e15625_d_n10;
        locals.var_tmf2_dn11 = assign20800_e15625_d_n11;
        locals.var_tmf2_dn14 = assign20800_e15625_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20810_e15639, assign20810_e15639_d_n0, assign20810_e15639_d_n2, assign20810_e15639_d_n4, assign20810_e15639_d_n5, assign20810_e15639_d_n6, assign20810_e15639_d_n7, assign20810_e15639_d_n8, assign20810_e15639_d_n9, assign20810_e15639_d_n10, assign20810_e15639_d_n11, assign20810_e15639_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20810_e15635: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign20810_e15636: f64 = (1.0 + assign20810_e15635);
        let assign20810_e15637: f64 = (0.5 * assign20810_e15636);
        (assign20810_e15637, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20810_e15639;
        locals.var_t0_dn0 = assign20810_e15639_d_n0;
        locals.var_t0_dn2 = assign20810_e15639_d_n2;
        locals.var_t0_dn4 = assign20810_e15639_d_n4;
        locals.var_t0_dn5 = assign20810_e15639_d_n5;
        locals.var_t0_dn6 = assign20810_e15639_d_n6;
        locals.var_t0_dn7 = assign20810_e15639_d_n7;
        locals.var_t0_dn8 = assign20810_e15639_d_n8;
        locals.var_t0_dn9 = assign20810_e15639_d_n9;
        locals.var_t0_dn10 = assign20810_e15639_d_n10;
        locals.var_t0_dn11 = assign20810_e15639_d_n11;
        locals.var_t0_dn14 = assign20810_e15639_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign20820_e15659, assign20820_e15659_d_n0, assign20820_e15659_d_n2, assign20820_e15659_d_n4, assign20820_e15659_d_n5, assign20820_e15659_d_n6, assign20820_e15659_d_n7, assign20820_e15659_d_n8, assign20820_e15659_d_n9, assign20820_e15659_d_n10, assign20820_e15659_d_n11, assign20820_e15659_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20820_e15650: f64 = (2.0 * 0.01);
        let assign20820_e15652: f64 = (assign20820_e15650 * 0.01);
        let assign20820_e15653: f64 = (locals.var_tmf1 - assign20820_e15652);
        let assign20820_e15655: f64 = (assign20820_e15653 / locals.var_tmf2);
        let assign20820_e15656: f64 = (1.0 - assign20820_e15655);
        let assign20820_e15657: f64 = (0.5 * assign20820_e15656);
        (assign20820_e15657, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign20820_e15653 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign20820_e15653 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign20820_e15653 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign20820_e15653 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign20820_e15653 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign20820_e15653 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign20820_e15653 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn9 * locals.var_tmf2) - (assign20820_e15653 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign20820_e15653 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign20820_e15653 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn14 * locals.var_tmf2) - (assign20820_e15653 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign20820_e15659;
        locals.var_t5_dn0 = assign20820_e15659_d_n0;
        locals.var_t5_dn2 = assign20820_e15659_d_n2;
        locals.var_t5_dn4 = assign20820_e15659_d_n4;
        locals.var_t5_dn5 = assign20820_e15659_d_n5;
        locals.var_t5_dn6 = assign20820_e15659_d_n6;
        locals.var_t5_dn7 = assign20820_e15659_d_n7;
        locals.var_t5_dn8 = assign20820_e15659_d_n8;
        locals.var_t5_dn9 = assign20820_e15659_d_n9;
        locals.var_t5_dn10 = assign20820_e15659_d_n10;
        locals.var_t5_dn11 = assign20820_e15659_d_n11;
        locals.var_t5_dn14 = assign20820_e15659_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign20830_e15673, assign20830_e15673_d_n0, assign20830_e15673_d_n2, assign20830_e15673_d_n4, assign20830_e15673_d_n5, assign20830_e15673_d_n6, assign20830_e15673_d_n7, assign20830_e15673_d_n8, assign20830_e15673_d_n9, assign20830_e15673_d_n10, assign20830_e15673_d_n11, assign20830_e15673_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20830_e15669: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20830_e15670: f64 = (0.5 * assign20830_e15669);
        let assign20830_e15671: f64 = (locals.var_t4 + assign20830_e15670);
        (assign20830_e15671, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t4_dn14 + (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign20830_e15673;
        locals.var_t2_dn0 = assign20830_e15673_d_n0;
        locals.var_t2_dn2 = assign20830_e15673_d_n2;
        locals.var_t2_dn4 = assign20830_e15673_d_n4;
        locals.var_t2_dn5 = assign20830_e15673_d_n5;
        locals.var_t2_dn6 = assign20830_e15673_d_n6;
        locals.var_t2_dn7 = assign20830_e15673_d_n7;
        locals.var_t2_dn8 = assign20830_e15673_d_n8;
        locals.var_t2_dn9 = assign20830_e15673_d_n9;
        locals.var_t2_dn10 = assign20830_e15673_d_n10;
        locals.var_t2_dn11 = assign20830_e15673_d_n11;
        locals.var_t2_dn14 = assign20830_e15673_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign20840_e15685, assign20840_e15685_d_n0, assign20840_e15685_d_n2, assign20840_e15685_d_n4, assign20840_e15685_d_n5, assign20840_e15685_d_n6, assign20840_e15685_d_n7, assign20840_e15685_d_n8, assign20840_e15685_d_n9, assign20840_e15685_d_n10, assign20840_e15685_d_n11, assign20840_e15685_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20840_e15682: f64 = (1.0 + locals.var_uc_rdvg11);
        let assign20840_e15683: f64 = (locals.var_t4 * assign20840_e15682);
        (assign20840_e15683, (locals.var_t4_dn0 * assign20840_e15682), (locals.var_t4_dn2 * assign20840_e15682), (locals.var_t4_dn4 * assign20840_e15682), (locals.var_t4_dn5 * assign20840_e15682), (locals.var_t4_dn6 * assign20840_e15682), (locals.var_t4_dn7 * assign20840_e15682), (locals.var_t4_dn8 * assign20840_e15682), (locals.var_t4_dn9 * assign20840_e15682), (locals.var_t4_dn10 * assign20840_e15682), (locals.var_t4_dn11 * assign20840_e15682), (locals.var_t4_dn14 * assign20840_e15682),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign20840_e15685;
        locals.var_t3_dn0 = assign20840_e15685_d_n0;
        locals.var_t3_dn2 = assign20840_e15685_d_n2;
        locals.var_t3_dn4 = assign20840_e15685_d_n4;
        locals.var_t3_dn5 = assign20840_e15685_d_n5;
        locals.var_t3_dn6 = assign20840_e15685_d_n6;
        locals.var_t3_dn7 = assign20840_e15685_d_n7;
        locals.var_t3_dn8 = assign20840_e15685_d_n8;
        locals.var_t3_dn9 = assign20840_e15685_d_n9;
        locals.var_t3_dn10 = assign20840_e15685_d_n10;
        locals.var_t3_dn11 = assign20840_e15685_d_n11;
        locals.var_t3_dn14 = assign20840_e15685_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign20850_e15699, assign20850_e15699_d_n0, assign20850_e15699_d_n2, assign20850_e15699_d_n4, assign20850_e15699_d_n5, assign20850_e15699_d_n6, assign20850_e15699_d_n7, assign20850_e15699_d_n8, assign20850_e15699_d_n9, assign20850_e15699_d_n10, assign20850_e15699_d_n11, assign20850_e15699_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20850_e15693: f64 = (locals.var_t3 - locals.var_t2);
        let assign20850_e15696: f64 = (5e-5 * 0.01);
        let assign20850_e15697: f64 = (assign20850_e15693 - assign20850_e15696);
        (assign20850_e15697, (locals.var_t3_dn0 - locals.var_t2_dn0), (locals.var_t3_dn2 - locals.var_t2_dn2), (locals.var_t3_dn4 - locals.var_t2_dn4), (locals.var_t3_dn5 - locals.var_t2_dn5), (locals.var_t3_dn6 - locals.var_t2_dn6), (locals.var_t3_dn7 - locals.var_t2_dn7), (locals.var_t3_dn8 - locals.var_t2_dn8), (locals.var_t3_dn9 - locals.var_t2_dn9), (locals.var_t3_dn10 - locals.var_t2_dn10), (locals.var_t3_dn11 - locals.var_t2_dn11), (locals.var_t3_dn14 - locals.var_t2_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign20850_e15699;
        locals.var_tmf1_dn0 = assign20850_e15699_d_n0;
        locals.var_tmf1_dn2 = assign20850_e15699_d_n2;
        locals.var_tmf1_dn4 = assign20850_e15699_d_n4;
        locals.var_tmf1_dn5 = assign20850_e15699_d_n5;
        locals.var_tmf1_dn6 = assign20850_e15699_d_n6;
        locals.var_tmf1_dn7 = assign20850_e15699_d_n7;
        locals.var_tmf1_dn8 = assign20850_e15699_d_n8;
        locals.var_tmf1_dn9 = assign20850_e15699_d_n9;
        locals.var_tmf1_dn10 = assign20850_e15699_d_n10;
        locals.var_tmf1_dn11 = assign20850_e15699_d_n11;
        locals.var_tmf1_dn14 = assign20850_e15699_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign20860_e15713, assign20860_e15713_d_n0, assign20860_e15713_d_n2, assign20860_e15713_d_n4, assign20860_e15713_d_n5, assign20860_e15713_d_n6, assign20860_e15713_d_n7, assign20860_e15713_d_n8, assign20860_e15713_d_n9, assign20860_e15713_d_n10, assign20860_e15713_d_n11, assign20860_e15713_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20860_e15707: f64 = (4.0 * locals.var_t3);
        let assign20860_e15710: f64 = (5e-5 * 0.01);
        let assign20860_e15711: f64 = (assign20860_e15707 * assign20860_e15710);
        (assign20860_e15711, ((4.0 * locals.var_t3_dn0) * assign20860_e15710), ((4.0 * locals.var_t3_dn2) * assign20860_e15710), ((4.0 * locals.var_t3_dn4) * assign20860_e15710), ((4.0 * locals.var_t3_dn5) * assign20860_e15710), ((4.0 * locals.var_t3_dn6) * assign20860_e15710), ((4.0 * locals.var_t3_dn7) * assign20860_e15710), ((4.0 * locals.var_t3_dn8) * assign20860_e15710), ((4.0 * locals.var_t3_dn9) * assign20860_e15710), ((4.0 * locals.var_t3_dn10) * assign20860_e15710), ((4.0 * locals.var_t3_dn11) * assign20860_e15710), ((4.0 * locals.var_t3_dn14) * assign20860_e15710),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20860_e15713;
        locals.var_tmf2_dn0 = assign20860_e15713_d_n0;
        locals.var_tmf2_dn2 = assign20860_e15713_d_n2;
        locals.var_tmf2_dn4 = assign20860_e15713_d_n4;
        locals.var_tmf2_dn5 = assign20860_e15713_d_n5;
        locals.var_tmf2_dn6 = assign20860_e15713_d_n6;
        locals.var_tmf2_dn7 = assign20860_e15713_d_n7;
        locals.var_tmf2_dn8 = assign20860_e15713_d_n8;
        locals.var_tmf2_dn9 = assign20860_e15713_d_n9;
        locals.var_tmf2_dn10 = assign20860_e15713_d_n10;
        locals.var_tmf2_dn11 = assign20860_e15713_d_n11;
        locals.var_tmf2_dn14 = assign20860_e15713_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20870_e15727, assign20870_e15727_d_n0, assign20870_e15727_d_n2, assign20870_e15727_d_n4, assign20870_e15727_d_n5, assign20870_e15727_d_n6, assign20870_e15727_d_n7, assign20870_e15727_d_n8, assign20870_e15727_d_n9, assign20870_e15727_d_n10, assign20870_e15727_d_n11, assign20870_e15727_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let (assign20870_e15725, assign20870_e15725_d_n0, assign20870_e15725_d_n2, assign20870_e15725_d_n4, assign20870_e15725_d_n5, assign20870_e15725_d_n6, assign20870_e15725_d_n7, assign20870_e15725_d_n8, assign20870_e15725_d_n9, assign20870_e15725_d_n10, assign20870_e15725_d_n11, assign20870_e15725_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign20870_e15724: f64 = (-locals.var_tmf2);
                (assign20870_e15724, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign20870_e15725, assign20870_e15725_d_n0, assign20870_e15725_d_n2, assign20870_e15725_d_n4, assign20870_e15725_d_n5, assign20870_e15725_d_n6, assign20870_e15725_d_n7, assign20870_e15725_d_n8, assign20870_e15725_d_n9, assign20870_e15725_d_n10, assign20870_e15725_d_n11, assign20870_e15725_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20870_e15727;
        locals.var_tmf2_dn0 = assign20870_e15727_d_n0;
        locals.var_tmf2_dn2 = assign20870_e15727_d_n2;
        locals.var_tmf2_dn4 = assign20870_e15727_d_n4;
        locals.var_tmf2_dn5 = assign20870_e15727_d_n5;
        locals.var_tmf2_dn6 = assign20870_e15727_d_n6;
        locals.var_tmf2_dn7 = assign20870_e15727_d_n7;
        locals.var_tmf2_dn8 = assign20870_e15727_d_n8;
        locals.var_tmf2_dn9 = assign20870_e15727_d_n9;
        locals.var_tmf2_dn10 = assign20870_e15727_d_n10;
        locals.var_tmf2_dn11 = assign20870_e15727_d_n11;
        locals.var_tmf2_dn14 = assign20870_e15727_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20880_e15740, assign20880_e15740_d_n0, assign20880_e15740_d_n2, assign20880_e15740_d_n4, assign20880_e15740_d_n5, assign20880_e15740_d_n6, assign20880_e15740_d_n7, assign20880_e15740_d_n8, assign20880_e15740_d_n9, assign20880_e15740_d_n10, assign20880_e15740_d_n11, assign20880_e15740_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20880_e15735: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign20880_e15737: f64 = (assign20880_e15735 + locals.var_tmf2);
        let assign20880_e15738: f64 = (assign20880_e15737).sqrt();
        (assign20880_e15738, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign20880_e15738)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign20880_e15738)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign20880_e15738)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign20880_e15738)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign20880_e15738)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign20880_e15738)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign20880_e15738)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign20880_e15738)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign20880_e15738)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign20880_e15738)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign20880_e15738)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20880_e15740;
        locals.var_tmf2_dn0 = assign20880_e15740_d_n0;
        locals.var_tmf2_dn2 = assign20880_e15740_d_n2;
        locals.var_tmf2_dn4 = assign20880_e15740_d_n4;
        locals.var_tmf2_dn5 = assign20880_e15740_d_n5;
        locals.var_tmf2_dn6 = assign20880_e15740_d_n6;
        locals.var_tmf2_dn7 = assign20880_e15740_d_n7;
        locals.var_tmf2_dn8 = assign20880_e15740_d_n8;
        locals.var_tmf2_dn9 = assign20880_e15740_d_n9;
        locals.var_tmf2_dn10 = assign20880_e15740_d_n10;
        locals.var_tmf2_dn11 = assign20880_e15740_d_n11;
        locals.var_tmf2_dn14 = assign20880_e15740_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20890_e15754, assign20890_e15754_d_n0, assign20890_e15754_d_n2, assign20890_e15754_d_n4, assign20890_e15754_d_n5, assign20890_e15754_d_n6, assign20890_e15754_d_n7, assign20890_e15754_d_n8, assign20890_e15754_d_n9, assign20890_e15754_d_n10, assign20890_e15754_d_n11, assign20890_e15754_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20890_e15750: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign20890_e15751: f64 = (1.0 + assign20890_e15750);
        let assign20890_e15752: f64 = (0.5 * assign20890_e15751);
        (assign20890_e15752, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign20890_e15754;
        locals.var_t0_dn0 = assign20890_e15754_d_n0;
        locals.var_t0_dn2 = assign20890_e15754_d_n2;
        locals.var_t0_dn4 = assign20890_e15754_d_n4;
        locals.var_t0_dn5 = assign20890_e15754_d_n5;
        locals.var_t0_dn6 = assign20890_e15754_d_n6;
        locals.var_t0_dn7 = assign20890_e15754_d_n7;
        locals.var_t0_dn8 = assign20890_e15754_d_n8;
        locals.var_t0_dn9 = assign20890_e15754_d_n9;
        locals.var_t0_dn10 = assign20890_e15754_d_n10;
        locals.var_t0_dn11 = assign20890_e15754_d_n11;
        locals.var_t0_dn14 = assign20890_e15754_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign20900_e15774, assign20900_e15774_d_n0, assign20900_e15774_d_n2, assign20900_e15774_d_n4, assign20900_e15774_d_n5, assign20900_e15774_d_n6, assign20900_e15774_d_n7, assign20900_e15774_d_n8, assign20900_e15774_d_n9, assign20900_e15774_d_n10, assign20900_e15774_d_n11, assign20900_e15774_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20900_e15765: f64 = (2.0 * 5e-5);
        let assign20900_e15767: f64 = (assign20900_e15765 * 0.01);
        let assign20900_e15768: f64 = (locals.var_tmf1 + assign20900_e15767);
        let assign20900_e15770: f64 = (assign20900_e15768 / locals.var_tmf2);
        let assign20900_e15771: f64 = (1.0 - assign20900_e15770);
        let assign20900_e15772: f64 = (0.5 * assign20900_e15771);
        (assign20900_e15772, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign20900_e15768 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign20900_e15768 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign20900_e15768 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign20900_e15768 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign20900_e15768 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign20900_e15768 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign20900_e15768 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn9 * locals.var_tmf2) - (assign20900_e15768 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign20900_e15768 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign20900_e15768 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn14 * locals.var_tmf2) - (assign20900_e15768 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign20900_e15774;
        locals.var_t5_dn0 = assign20900_e15774_d_n0;
        locals.var_t5_dn2 = assign20900_e15774_d_n2;
        locals.var_t5_dn4 = assign20900_e15774_d_n4;
        locals.var_t5_dn5 = assign20900_e15774_d_n5;
        locals.var_t5_dn6 = assign20900_e15774_d_n6;
        locals.var_t5_dn7 = assign20900_e15774_d_n7;
        locals.var_t5_dn8 = assign20900_e15774_d_n8;
        locals.var_t5_dn9 = assign20900_e15774_d_n9;
        locals.var_t5_dn10 = assign20900_e15774_d_n10;
        locals.var_t5_dn11 = assign20900_e15774_d_n11;
        locals.var_t5_dn14 = assign20900_e15774_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign20910_e15788, assign20910_e15788_d_n0, assign20910_e15788_d_n2, assign20910_e15788_d_n4, assign20910_e15788_d_n5, assign20910_e15788_d_n6, assign20910_e15788_d_n7, assign20910_e15788_d_n8, assign20910_e15788_d_n9, assign20910_e15788_d_n10, assign20910_e15788_d_n11, assign20910_e15788_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20910_e15784: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign20910_e15785: f64 = (0.5 * assign20910_e15784);
        let assign20910_e15786: f64 = (locals.var_t3 - assign20910_e15785);
        (assign20910_e15786, (locals.var_t3_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t3_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t3_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t3_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t3_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t3_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t3_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t3_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t3_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t3_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t3_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn11, locals.var_rdrift_dn14,)
    }
};
        locals.var_rdrift = assign20910_e15788;
        locals.var_rdrift_dn0 = assign20910_e15788_d_n0;
        locals.var_rdrift_dn2 = assign20910_e15788_d_n2;
        locals.var_rdrift_dn4 = assign20910_e15788_d_n4;
        locals.var_rdrift_dn5 = assign20910_e15788_d_n5;
        locals.var_rdrift_dn6 = assign20910_e15788_d_n6;
        locals.var_rdrift_dn7 = assign20910_e15788_d_n7;
        locals.var_rdrift_dn8 = assign20910_e15788_d_n8;
        locals.var_rdrift_dn9 = assign20910_e15788_d_n9;
        locals.var_rdrift_dn10 = assign20910_e15788_d_n10;
        locals.var_rdrift_dn11 = assign20910_e15788_d_n11;
        locals.var_rdrift_dn14 = assign20910_e15788_d_n14;
        locals.var_rdrift_rv = 0.0;

        let (assign20920_e15800, assign20920_e15800_d_n0, assign20920_e15800_d_n2, assign20920_e15800_d_n4, assign20920_e15800_d_n5, assign20920_e15800_d_n6, assign20920_e15800_d_n7, assign20920_e15800_d_n8, assign20920_e15800_d_n9, assign20920_e15800_d_n10, assign20920_e15800_d_n11, assign20920_e15800_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20920_e15797: f64 = (locals.var_uc_rdvb * locals.var_vbserevz);
        let assign20920_e15798: f64 = (1.0 - assign20920_e15797);
        (assign20920_e15798, (-(locals.var_uc_rdvb * locals.var_vbserevz_dn0)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn2)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn4)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn5)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn6)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn7)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn8)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn9)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn10)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn11)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign20920_e15800;
        locals.var_t1_dn0 = assign20920_e15800_d_n0;
        locals.var_t1_dn2 = assign20920_e15800_d_n2;
        locals.var_t1_dn4 = assign20920_e15800_d_n4;
        locals.var_t1_dn5 = assign20920_e15800_d_n5;
        locals.var_t1_dn6 = assign20920_e15800_d_n6;
        locals.var_t1_dn7 = assign20920_e15800_d_n7;
        locals.var_t1_dn8 = assign20920_e15800_d_n8;
        locals.var_t1_dn9 = assign20920_e15800_d_n9;
        locals.var_t1_dn10 = assign20920_e15800_d_n10;
        locals.var_t1_dn11 = assign20920_e15800_d_n11;
        locals.var_t1_dn14 = assign20920_e15800_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_54(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign20930_e15821, assign20930_e15821_d_n0, assign20930_e15821_d_n2, assign20930_e15821_d_n4, assign20930_e15821_d_n5, assign20930_e15821_d_n6, assign20930_e15821_d_n7, assign20930_e15821_d_n8, assign20930_e15821_d_n9, assign20930_e15821_d_n10, assign20930_e15821_d_n11, assign20930_e15821_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20930_e15808: f64 = (locals.var_t1 * locals.var_t1);
        let assign20930_e15812: f64 = (0.0001 * 0.01);
        let assign20930_e15813: f64 = (4.0 * assign20930_e15812);
        let assign20930_e15816: f64 = (0.0001 * 0.01);
        let assign20930_e15817: f64 = (assign20930_e15813 * assign20930_e15816);
        let assign20930_e15818: f64 = (assign20930_e15808 + assign20930_e15817);
        let assign20930_e15819: f64 = (assign20930_e15818).sqrt();
        (assign20930_e15819, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign20930_e15819)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign20930_e15819)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign20930_e15819)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign20930_e15819)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign20930_e15819)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign20930_e15819)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign20930_e15819)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign20930_e15819)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign20930_e15819)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign20930_e15819)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign20930_e15819)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign20930_e15821;
        locals.var_tmf2_dn0 = assign20930_e15821_d_n0;
        locals.var_tmf2_dn2 = assign20930_e15821_d_n2;
        locals.var_tmf2_dn4 = assign20930_e15821_d_n4;
        locals.var_tmf2_dn5 = assign20930_e15821_d_n5;
        locals.var_tmf2_dn6 = assign20930_e15821_d_n6;
        locals.var_tmf2_dn7 = assign20930_e15821_d_n7;
        locals.var_tmf2_dn8 = assign20930_e15821_d_n8;
        locals.var_tmf2_dn9 = assign20930_e15821_d_n9;
        locals.var_tmf2_dn10 = assign20930_e15821_d_n10;
        locals.var_tmf2_dn11 = assign20930_e15821_d_n11;
        locals.var_tmf2_dn14 = assign20930_e15821_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign20940_e15835, assign20940_e15835_d_n0, assign20940_e15835_d_n2, assign20940_e15835_d_n4, assign20940_e15835_d_n5, assign20940_e15835_d_n6, assign20940_e15835_d_n7, assign20940_e15835_d_n8, assign20940_e15835_d_n9, assign20940_e15835_d_n10, assign20940_e15835_d_n11, assign20940_e15835_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20940_e15831: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign20940_e15832: f64 = (1.0 + assign20940_e15831);
        let assign20940_e15833: f64 = (0.5 * assign20940_e15832);
        (assign20940_e15833, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign20940_e15835;
        locals.var_t4_dn0 = assign20940_e15835_d_n0;
        locals.var_t4_dn2 = assign20940_e15835_d_n2;
        locals.var_t4_dn4 = assign20940_e15835_d_n4;
        locals.var_t4_dn5 = assign20940_e15835_d_n5;
        locals.var_t4_dn6 = assign20940_e15835_d_n6;
        locals.var_t4_dn7 = assign20940_e15835_d_n7;
        locals.var_t4_dn8 = assign20940_e15835_d_n8;
        locals.var_t4_dn9 = assign20940_e15835_d_n9;
        locals.var_t4_dn10 = assign20940_e15835_d_n10;
        locals.var_t4_dn11 = assign20940_e15835_d_n11;
        locals.var_t4_dn14 = assign20940_e15835_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign20950_e15847, assign20950_e15847_d_n0, assign20950_e15847_d_n2, assign20950_e15847_d_n4, assign20950_e15847_d_n5, assign20950_e15847_d_n6, assign20950_e15847_d_n7, assign20950_e15847_d_n8, assign20950_e15847_d_n9, assign20950_e15847_d_n10, assign20950_e15847_d_n11, assign20950_e15847_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20950_e15844: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign20950_e15845: f64 = (0.5 * assign20950_e15844);
        (assign20950_e15845, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign20950_e15847;
        locals.var_t3_dn0 = assign20950_e15847_d_n0;
        locals.var_t3_dn2 = assign20950_e15847_d_n2;
        locals.var_t3_dn4 = assign20950_e15847_d_n4;
        locals.var_t3_dn5 = assign20950_e15847_d_n5;
        locals.var_t3_dn6 = assign20950_e15847_d_n6;
        locals.var_t3_dn7 = assign20950_e15847_d_n7;
        locals.var_t3_dn8 = assign20950_e15847_d_n8;
        locals.var_t3_dn9 = assign20950_e15847_d_n9;
        locals.var_t3_dn10 = assign20950_e15847_d_n10;
        locals.var_t3_dn11 = assign20950_e15847_d_n11;
        locals.var_t3_dn14 = assign20950_e15847_d_n14;
        locals.var_t3_rv = 0.0;

        let assign20960_e15850: f64 = if locals.var_t3 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard417 = assign20960_e15850;
        locals.var_guard417_rv = 0.0;

        let (assign20970_e15860, assign20970_e15860_d_n0, assign20970_e15860_d_n2, assign20970_e15860_d_n4, assign20970_e15860_d_n5, assign20970_e15860_d_n6, assign20970_e15860_d_n7, assign20970_e15860_d_n8, assign20970_e15860_d_n9, assign20970_e15860_d_n10, assign20970_e15860_d_n11, assign20970_e15860_d_n14,) = {
    if ((((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign20970_e15860;
        locals.var_t3_dn0 = assign20970_e15860_d_n0;
        locals.var_t3_dn2 = assign20970_e15860_d_n2;
        locals.var_t3_dn4 = assign20970_e15860_d_n4;
        locals.var_t3_dn5 = assign20970_e15860_d_n5;
        locals.var_t3_dn6 = assign20970_e15860_d_n6;
        locals.var_t3_dn7 = assign20970_e15860_d_n7;
        locals.var_t3_dn8 = assign20970_e15860_d_n8;
        locals.var_t3_dn9 = assign20970_e15860_d_n9;
        locals.var_t3_dn10 = assign20970_e15860_d_n10;
        locals.var_t3_dn11 = assign20970_e15860_d_n11;
        locals.var_t3_dn14 = assign20970_e15860_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign20980_e15870, assign20980_e15870_d_n0, assign20980_e15870_d_n2, assign20980_e15870_d_n4, assign20980_e15870_d_n5, assign20980_e15870_d_n6, assign20980_e15870_d_n7, assign20980_e15870_d_n8, assign20980_e15870_d_n9, assign20980_e15870_d_n10, assign20980_e15870_d_n11, assign20980_e15870_d_n14,) = {
    if ((((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) && (locals.var_guard417 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign20980_e15870;
        locals.var_t4_dn0 = assign20980_e15870_d_n0;
        locals.var_t4_dn2 = assign20980_e15870_d_n2;
        locals.var_t4_dn4 = assign20980_e15870_d_n4;
        locals.var_t4_dn5 = assign20980_e15870_d_n5;
        locals.var_t4_dn6 = assign20980_e15870_d_n6;
        locals.var_t4_dn7 = assign20980_e15870_d_n7;
        locals.var_t4_dn8 = assign20980_e15870_d_n8;
        locals.var_t4_dn9 = assign20980_e15870_d_n9;
        locals.var_t4_dn10 = assign20980_e15870_d_n10;
        locals.var_t4_dn11 = assign20980_e15870_d_n11;
        locals.var_t4_dn14 = assign20980_e15870_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign20990_e15880, assign20990_e15880_d_n0, assign20990_e15880_d_n2, assign20990_e15880_d_n4, assign20990_e15880_d_n5, assign20990_e15880_d_n6, assign20990_e15880_d_n7, assign20990_e15880_d_n8, assign20990_e15880_d_n9, assign20990_e15880_d_n10, assign20990_e15880_d_n11, assign20990_e15880_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign20990_e15878: f64 = (locals.var_t3 + 1e-25);
        (assign20990_e15878, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign20990_e15880;
        locals.var_t3_dn0 = assign20990_e15880_d_n0;
        locals.var_t3_dn2 = assign20990_e15880_d_n2;
        locals.var_t3_dn4 = assign20990_e15880_d_n4;
        locals.var_t3_dn5 = assign20990_e15880_d_n5;
        locals.var_t3_dn6 = assign20990_e15880_d_n6;
        locals.var_t3_dn7 = assign20990_e15880_d_n7;
        locals.var_t3_dn8 = assign20990_e15880_d_n8;
        locals.var_t3_dn9 = assign20990_e15880_d_n9;
        locals.var_t3_dn10 = assign20990_e15880_d_n10;
        locals.var_t3_dn11 = assign20990_e15880_d_n11;
        locals.var_t3_dn14 = assign20990_e15880_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign21000_e15888, assign21000_e15888_d_n0, assign21000_e15888_d_n2, assign21000_e15888_d_n4, assign21000_e15888_d_n5, assign21000_e15888_d_n6, assign21000_e15888_d_n7, assign21000_e15888_d_n8, assign21000_e15888_d_n9, assign21000_e15888_d_n10, assign21000_e15888_d_n11, assign21000_e15888_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn11, locals.var_rdrift_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21000_e15888;
        locals.var_t0_dn0 = assign21000_e15888_d_n0;
        locals.var_t0_dn2 = assign21000_e15888_d_n2;
        locals.var_t0_dn4 = assign21000_e15888_d_n4;
        locals.var_t0_dn5 = assign21000_e15888_d_n5;
        locals.var_t0_dn6 = assign21000_e15888_d_n6;
        locals.var_t0_dn7 = assign21000_e15888_d_n7;
        locals.var_t0_dn8 = assign21000_e15888_d_n8;
        locals.var_t0_dn9 = assign21000_e15888_d_n9;
        locals.var_t0_dn10 = assign21000_e15888_d_n10;
        locals.var_t0_dn11 = assign21000_e15888_d_n11;
        locals.var_t0_dn14 = assign21000_e15888_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21010_e15898, assign21010_e15898_d_n0, assign21010_e15898_d_n2, assign21010_e15898_d_n4, assign21010_e15898_d_n5, assign21010_e15898_d_n6, assign21010_e15898_d_n7, assign21010_e15898_d_n8, assign21010_e15898_d_n9, assign21010_e15898_d_n10, assign21010_e15898_d_n11, assign21010_e15898_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign21010_e15896: f64 = (locals.var_rdrift * locals.var_t3);
        (assign21010_e15896, ((locals.var_rdrift_dn0 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn0)), ((locals.var_rdrift_dn2 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn2)), ((locals.var_rdrift_dn4 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn4)), ((locals.var_rdrift_dn5 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn5)), ((locals.var_rdrift_dn6 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn6)), ((locals.var_rdrift_dn7 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn7)), ((locals.var_rdrift_dn8 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn8)), ((locals.var_rdrift_dn9 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn9)), ((locals.var_rdrift_dn10 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn10)), ((locals.var_rdrift_dn11 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn11)), ((locals.var_rdrift_dn14 * locals.var_t3) + (locals.var_rdrift * locals.var_t3_dn14)),)
    } else {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn11, locals.var_rdrift_dn14,)
    }
};
        locals.var_rdrift = assign21010_e15898;
        locals.var_rdrift_dn0 = assign21010_e15898_d_n0;
        locals.var_rdrift_dn2 = assign21010_e15898_d_n2;
        locals.var_rdrift_dn4 = assign21010_e15898_d_n4;
        locals.var_rdrift_dn5 = assign21010_e15898_d_n5;
        locals.var_rdrift_dn6 = assign21010_e15898_d_n6;
        locals.var_rdrift_dn7 = assign21010_e15898_d_n7;
        locals.var_rdrift_dn8 = assign21010_e15898_d_n8;
        locals.var_rdrift_dn9 = assign21010_e15898_d_n9;
        locals.var_rdrift_dn10 = assign21010_e15898_d_n10;
        locals.var_rdrift_dn11 = assign21010_e15898_d_n11;
        locals.var_rdrift_dn14 = assign21010_e15898_d_n14;
        locals.var_rdrift_rv = 0.0;

        let (assign21020_e15907, assign21020_e15907_d_n0, assign21020_e15907_d_n2, assign21020_e15907_d_n4, assign21020_e15907_d_n5, assign21020_e15907_d_n6, assign21020_e15907_d_n7, assign21020_e15907_d_n8, assign21020_e15907_d_n9, assign21020_e15907_d_n10, assign21020_e15907_d_n11, assign21020_e15907_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard415 == 0.0)) {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    } else {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn11, locals.var_rdrift_dn14,)
    }
};
        locals.var_rdrift = assign21020_e15907;
        locals.var_rdrift_dn0 = assign21020_e15907_d_n0;
        locals.var_rdrift_dn2 = assign21020_e15907_d_n2;
        locals.var_rdrift_dn4 = assign21020_e15907_d_n4;
        locals.var_rdrift_dn5 = assign21020_e15907_d_n5;
        locals.var_rdrift_dn6 = assign21020_e15907_d_n6;
        locals.var_rdrift_dn7 = assign21020_e15907_d_n7;
        locals.var_rdrift_dn8 = assign21020_e15907_d_n8;
        locals.var_rdrift_dn9 = assign21020_e15907_d_n9;
        locals.var_rdrift_dn10 = assign21020_e15907_d_n10;
        locals.var_rdrift_dn11 = assign21020_e15907_d_n11;
        locals.var_rdrift_dn14 = assign21020_e15907_d_n14;
        locals.var_rdrift_rv = 0.0;

        let (assign21030_e15919, assign21030_e15919_d_n0, assign21030_e15919_d_n2, assign21030_e15919_d_n4, assign21030_e15919_d_n5, assign21030_e15919_d_n6, assign21030_e15919_d_n7, assign21030_e15919_d_n8, assign21030_e15919_d_n9, assign21030_e15919_d_n10, assign21030_e15919_d_n11, assign21030_e15919_d_n14,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) {
        let assign21030_e15913: f64 = (locals.var_vdsemodenml * locals.var_rse);
        let assign21030_e15916: f64 = (locals.var_vdsemodervs * locals.var_rde);
        let assign21030_e15917: f64 = (assign21030_e15913 + assign21030_e15916);
        (assign21030_e15917, ((locals.var_vdsemodenml * locals.var_rse_dn0) + (locals.var_vdsemodervs * locals.var_rde_dn0)), ((locals.var_vdsemodenml * locals.var_rse_dn2) + (locals.var_vdsemodervs * locals.var_rde_dn2)), ((locals.var_vdsemodenml * locals.var_rse_dn4) + (locals.var_vdsemodervs * locals.var_rde_dn4)), ((locals.var_vdsemodenml * locals.var_rse_dn5) + (locals.var_vdsemodervs * locals.var_rde_dn5)), ((locals.var_vdsemodenml * locals.var_rse_dn6) + (locals.var_vdsemodervs * locals.var_rde_dn6)), ((locals.var_vdsemodenml * locals.var_rse_dn7) + (locals.var_vdsemodervs * locals.var_rde_dn7)), ((locals.var_vdsemodenml * locals.var_rse_dn8) + (locals.var_vdsemodervs * locals.var_rde_dn8)), ((locals.var_vdsemodenml * locals.var_rse_dn9) + (locals.var_vdsemodervs * locals.var_rde_dn9)), ((locals.var_vdsemodenml * locals.var_rse_dn10) + (locals.var_vdsemodervs * locals.var_rde_dn10)), ((locals.var_vdsemodenml * locals.var_rse_dn11) + (locals.var_vdsemodervs * locals.var_rde_dn11)), ((locals.var_vdsemodenml * locals.var_rse_dn14) + (locals.var_vdsemodervs * locals.var_rde_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21030_e15919;
        locals.var_t4_dn0 = assign21030_e15919_d_n0;
        locals.var_t4_dn2 = assign21030_e15919_d_n2;
        locals.var_t4_dn4 = assign21030_e15919_d_n4;
        locals.var_t4_dn5 = assign21030_e15919_d_n5;
        locals.var_t4_dn6 = assign21030_e15919_d_n6;
        locals.var_t4_dn7 = assign21030_e15919_d_n7;
        locals.var_t4_dn8 = assign21030_e15919_d_n8;
        locals.var_t4_dn9 = assign21030_e15919_d_n9;
        locals.var_t4_dn10 = assign21030_e15919_d_n10;
        locals.var_t4_dn11 = assign21030_e15919_d_n11;
        locals.var_t4_dn14 = assign21030_e15919_d_n14;
        locals.var_t4_rv = 0.0;

        let assign21040_e15926: f64 = if ((p.p34 == 1.0) || (locals.var_vdsemodervs == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard418 = assign21040_e15926;
        locals.var_guard418_rv = 0.0;

        let (assign21050_e15940, assign21050_e15940_d_n0, assign21050_e15940_d_n2, assign21050_e15940_d_n4, assign21050_e15940_d_n5, assign21050_e15940_d_n6, assign21050_e15940_d_n7, assign21050_e15940_d_n8, assign21050_e15940_d_n9, assign21050_e15940_d_n10, assign21050_e15940_d_n11, assign21050_e15940_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21050_e15934: f64 = (locals.var_vdsemodenml * locals.var_rsvde);
        let assign21050_e15937: f64 = (locals.var_vdsemodervs * locals.var_rdvde);
        let assign21050_e15938: f64 = (assign21050_e15934 + assign21050_e15937);
        (assign21050_e15938, ((locals.var_vdsemodenml * locals.var_rsvde_dn0) + (locals.var_vdsemodervs * locals.var_rdvde_dn0)), ((locals.var_vdsemodenml * locals.var_rsvde_dn2) + (locals.var_vdsemodervs * locals.var_rdvde_dn2)), ((locals.var_vdsemodenml * locals.var_rsvde_dn4) + (locals.var_vdsemodervs * locals.var_rdvde_dn4)), ((locals.var_vdsemodenml * locals.var_rsvde_dn5) + (locals.var_vdsemodervs * locals.var_rdvde_dn5)), ((locals.var_vdsemodenml * locals.var_rsvde_dn6) + (locals.var_vdsemodervs * locals.var_rdvde_dn6)), ((locals.var_vdsemodenml * locals.var_rsvde_dn7) + (locals.var_vdsemodervs * locals.var_rdvde_dn7)), ((locals.var_vdsemodenml * locals.var_rsvde_dn8) + (locals.var_vdsemodervs * locals.var_rdvde_dn8)), ((locals.var_vdsemodenml * locals.var_rsvde_dn9) + (locals.var_vdsemodervs * locals.var_rdvde_dn9)), ((locals.var_vdsemodenml * locals.var_rsvde_dn10) + (locals.var_vdsemodervs * locals.var_rdvde_dn10)), ((locals.var_vdsemodenml * locals.var_rsvde_dn11) + (locals.var_vdsemodervs * locals.var_rdvde_dn11)), ((locals.var_vdsemodenml * locals.var_rsvde_dn14) + (locals.var_vdsemodervs * locals.var_rdvde_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21050_e15940;
        locals.var_t0_dn0 = assign21050_e15940_d_n0;
        locals.var_t0_dn2 = assign21050_e15940_d_n2;
        locals.var_t0_dn4 = assign21050_e15940_d_n4;
        locals.var_t0_dn5 = assign21050_e15940_d_n5;
        locals.var_t0_dn6 = assign21050_e15940_d_n6;
        locals.var_t0_dn7 = assign21050_e15940_d_n7;
        locals.var_t0_dn8 = assign21050_e15940_d_n8;
        locals.var_t0_dn9 = assign21050_e15940_d_n9;
        locals.var_t0_dn10 = assign21050_e15940_d_n10;
        locals.var_t0_dn11 = assign21050_e15940_d_n11;
        locals.var_t0_dn14 = assign21050_e15940_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21060_e15954, assign21060_e15954_d_n0, assign21060_e15954_d_n2, assign21060_e15954_d_n4, assign21060_e15954_d_n5, assign21060_e15954_d_n6, assign21060_e15954_d_n7, assign21060_e15954_d_n8, assign21060_e15954_d_n9, assign21060_e15954_d_n10, assign21060_e15954_d_n11, assign21060_e15954_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21060_e15950: f64 = (2.0 * p.p262);
        let assign21060_e15951: f64 = (locals.var_t0 * assign21060_e15950);
        let assign21060_e15952: f64 = (locals.var_t4 + assign21060_e15951);
        (assign21060_e15952, (locals.var_t4_dn0 + (locals.var_t0_dn0 * assign21060_e15950)), (locals.var_t4_dn2 + (locals.var_t0_dn2 * assign21060_e15950)), (locals.var_t4_dn4 + (locals.var_t0_dn4 * assign21060_e15950)), (locals.var_t4_dn5 + (locals.var_t0_dn5 * assign21060_e15950)), (locals.var_t4_dn6 + (locals.var_t0_dn6 * assign21060_e15950)), (locals.var_t4_dn7 + (locals.var_t0_dn7 * assign21060_e15950)), (locals.var_t4_dn8 + (locals.var_t0_dn8 * assign21060_e15950)), (locals.var_t4_dn9 + (locals.var_t0_dn9 * assign21060_e15950)), (locals.var_t4_dn10 + (locals.var_t0_dn10 * assign21060_e15950)), (locals.var_t4_dn11 + (locals.var_t0_dn11 * assign21060_e15950)), (locals.var_t4_dn14 + (locals.var_t0_dn14 * assign21060_e15950)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21060_e15954;
        locals.var_t4_dn0 = assign21060_e15954_d_n0;
        locals.var_t4_dn2 = assign21060_e15954_d_n2;
        locals.var_t4_dn4 = assign21060_e15954_d_n4;
        locals.var_t4_dn5 = assign21060_e15954_d_n5;
        locals.var_t4_dn6 = assign21060_e15954_d_n6;
        locals.var_t4_dn7 = assign21060_e15954_d_n7;
        locals.var_t4_dn8 = assign21060_e15954_d_n8;
        locals.var_t4_dn9 = assign21060_e15954_d_n9;
        locals.var_t4_dn10 = assign21060_e15954_d_n10;
        locals.var_t4_dn11 = assign21060_e15954_d_n11;
        locals.var_t4_dn14 = assign21060_e15954_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign21070_e15964, assign21070_e15964_d_n0, assign21070_e15964_d_n2, assign21070_e15964_d_n4, assign21070_e15964_d_n5, assign21070_e15964_d_n6, assign21070_e15964_d_n7, assign21070_e15964_d_n8, assign21070_e15964_d_n9, assign21070_e15964_d_n10, assign21070_e15964_d_n11, assign21070_e15964_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21070_e15962: f64 = (p.p292 + 1e-25);
        (assign21070_e15962, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign21070_e15964;
        locals.var_t10_dn0 = assign21070_e15964_d_n0;
        locals.var_t10_dn2 = assign21070_e15964_d_n2;
        locals.var_t10_dn4 = assign21070_e15964_d_n4;
        locals.var_t10_dn5 = assign21070_e15964_d_n5;
        locals.var_t10_dn6 = assign21070_e15964_d_n6;
        locals.var_t10_dn7 = assign21070_e15964_d_n7;
        locals.var_t10_dn8 = assign21070_e15964_d_n8;
        locals.var_t10_dn9 = assign21070_e15964_d_n9;
        locals.var_t10_dn10 = assign21070_e15964_d_n10;
        locals.var_t10_dn11 = assign21070_e15964_d_n11;
        locals.var_t10_dn14 = assign21070_e15964_d_n14;
        locals.var_t10_rv = 0.0;

        let (assign21080_e15982, assign21080_e15982_d_n0, assign21080_e15982_d_n2, assign21080_e15982_d_n4, assign21080_e15982_d_n5, assign21080_e15982_d_n6, assign21080_e15982_d_n7, assign21080_e15982_d_n8, assign21080_e15982_d_n9, assign21080_e15982_d_n10, assign21080_e15982_d_n11, assign21080_e15982_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21080_e15976: f64 = (locals.var_vgserevz / locals.var_t10);
        let assign21080_e15977: f64 = (1.0 - assign21080_e15976);
        let assign21080_e15978: f64 = (locals.var_uc_rdvg11 * assign21080_e15977);
        let assign21080_e15979: f64 = (1.0 + assign21080_e15978);
        let assign21080_e15980: f64 = (locals.var_t4 * assign21080_e15979);
        (assign21080_e15980, ((locals.var_t4_dn0 * assign21080_e15979) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn0 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn0)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn2 * assign21080_e15979) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn2 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn2)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn4 * assign21080_e15979) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn4 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn5 * assign21080_e15979) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn5 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn6 * assign21080_e15979) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn6 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn7 * assign21080_e15979) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn7 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn8 * assign21080_e15979) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn8 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn9 * assign21080_e15979) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn9 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn10 * assign21080_e15979) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn10 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn11 * assign21080_e15979) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn11 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn11)) / (locals.var_t10 * locals.var_t10)))))), ((locals.var_t4_dn14 * assign21080_e15979) + (locals.var_t4 * (locals.var_uc_rdvg11 * (-(((locals.var_vgserevz_dn14 * locals.var_t10) - (locals.var_vgserevz * locals.var_t10_dn14)) / (locals.var_t10 * locals.var_t10)))))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21080_e15982;
        locals.var_t1_dn0 = assign21080_e15982_d_n0;
        locals.var_t1_dn2 = assign21080_e15982_d_n2;
        locals.var_t1_dn4 = assign21080_e15982_d_n4;
        locals.var_t1_dn5 = assign21080_e15982_d_n5;
        locals.var_t1_dn6 = assign21080_e15982_d_n6;
        locals.var_t1_dn7 = assign21080_e15982_d_n7;
        locals.var_t1_dn8 = assign21080_e15982_d_n8;
        locals.var_t1_dn9 = assign21080_e15982_d_n9;
        locals.var_t1_dn10 = assign21080_e15982_d_n10;
        locals.var_t1_dn11 = assign21080_e15982_d_n11;
        locals.var_t1_dn14 = assign21080_e15982_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign21090_e15996, assign21090_e15996_d_n0, assign21090_e15996_d_n2, assign21090_e15996_d_n4, assign21090_e15996_d_n5, assign21090_e15996_d_n6, assign21090_e15996_d_n7, assign21090_e15996_d_n8, assign21090_e15996_d_n9, assign21090_e15996_d_n10, assign21090_e15996_d_n11, assign21090_e15996_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21090_e15990: f64 = (locals.var_t1 - locals.var_t4);
        let assign21090_e15993: f64 = (0.01 * 0.01);
        let assign21090_e15994: f64 = (assign21090_e15990 - assign21090_e15993);
        (assign21090_e15994, (locals.var_t1_dn0 - locals.var_t4_dn0), (locals.var_t1_dn2 - locals.var_t4_dn2), (locals.var_t1_dn4 - locals.var_t4_dn4), (locals.var_t1_dn5 - locals.var_t4_dn5), (locals.var_t1_dn6 - locals.var_t4_dn6), (locals.var_t1_dn7 - locals.var_t4_dn7), (locals.var_t1_dn8 - locals.var_t4_dn8), (locals.var_t1_dn9 - locals.var_t4_dn9), (locals.var_t1_dn10 - locals.var_t4_dn10), (locals.var_t1_dn11 - locals.var_t4_dn11), (locals.var_t1_dn14 - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign21090_e15996;
        locals.var_tmf1_dn0 = assign21090_e15996_d_n0;
        locals.var_tmf1_dn2 = assign21090_e15996_d_n2;
        locals.var_tmf1_dn4 = assign21090_e15996_d_n4;
        locals.var_tmf1_dn5 = assign21090_e15996_d_n5;
        locals.var_tmf1_dn6 = assign21090_e15996_d_n6;
        locals.var_tmf1_dn7 = assign21090_e15996_d_n7;
        locals.var_tmf1_dn8 = assign21090_e15996_d_n8;
        locals.var_tmf1_dn9 = assign21090_e15996_d_n9;
        locals.var_tmf1_dn10 = assign21090_e15996_d_n10;
        locals.var_tmf1_dn11 = assign21090_e15996_d_n11;
        locals.var_tmf1_dn14 = assign21090_e15996_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign21100_e16010, assign21100_e16010_d_n0, assign21100_e16010_d_n2, assign21100_e16010_d_n4, assign21100_e16010_d_n5, assign21100_e16010_d_n6, assign21100_e16010_d_n7, assign21100_e16010_d_n8, assign21100_e16010_d_n9, assign21100_e16010_d_n10, assign21100_e16010_d_n11, assign21100_e16010_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21100_e16004: f64 = (4.0 * locals.var_t4);
        let assign21100_e16007: f64 = (0.01 * 0.01);
        let assign21100_e16008: f64 = (assign21100_e16004 * assign21100_e16007);
        (assign21100_e16008, ((4.0 * locals.var_t4_dn0) * assign21100_e16007), ((4.0 * locals.var_t4_dn2) * assign21100_e16007), ((4.0 * locals.var_t4_dn4) * assign21100_e16007), ((4.0 * locals.var_t4_dn5) * assign21100_e16007), ((4.0 * locals.var_t4_dn6) * assign21100_e16007), ((4.0 * locals.var_t4_dn7) * assign21100_e16007), ((4.0 * locals.var_t4_dn8) * assign21100_e16007), ((4.0 * locals.var_t4_dn9) * assign21100_e16007), ((4.0 * locals.var_t4_dn10) * assign21100_e16007), ((4.0 * locals.var_t4_dn11) * assign21100_e16007), ((4.0 * locals.var_t4_dn14) * assign21100_e16007),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21100_e16010;
        locals.var_tmf2_dn0 = assign21100_e16010_d_n0;
        locals.var_tmf2_dn2 = assign21100_e16010_d_n2;
        locals.var_tmf2_dn4 = assign21100_e16010_d_n4;
        locals.var_tmf2_dn5 = assign21100_e16010_d_n5;
        locals.var_tmf2_dn6 = assign21100_e16010_d_n6;
        locals.var_tmf2_dn7 = assign21100_e16010_d_n7;
        locals.var_tmf2_dn8 = assign21100_e16010_d_n8;
        locals.var_tmf2_dn9 = assign21100_e16010_d_n9;
        locals.var_tmf2_dn10 = assign21100_e16010_d_n10;
        locals.var_tmf2_dn11 = assign21100_e16010_d_n11;
        locals.var_tmf2_dn14 = assign21100_e16010_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21110_e16024, assign21110_e16024_d_n0, assign21110_e16024_d_n2, assign21110_e16024_d_n4, assign21110_e16024_d_n5, assign21110_e16024_d_n6, assign21110_e16024_d_n7, assign21110_e16024_d_n8, assign21110_e16024_d_n9, assign21110_e16024_d_n10, assign21110_e16024_d_n11, assign21110_e16024_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let (assign21110_e16022, assign21110_e16022_d_n0, assign21110_e16022_d_n2, assign21110_e16022_d_n4, assign21110_e16022_d_n5, assign21110_e16022_d_n6, assign21110_e16022_d_n7, assign21110_e16022_d_n8, assign21110_e16022_d_n9, assign21110_e16022_d_n10, assign21110_e16022_d_n11, assign21110_e16022_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign21110_e16021: f64 = (-locals.var_tmf2);
                (assign21110_e16021, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign21110_e16022, assign21110_e16022_d_n0, assign21110_e16022_d_n2, assign21110_e16022_d_n4, assign21110_e16022_d_n5, assign21110_e16022_d_n6, assign21110_e16022_d_n7, assign21110_e16022_d_n8, assign21110_e16022_d_n9, assign21110_e16022_d_n10, assign21110_e16022_d_n11, assign21110_e16022_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21110_e16024;
        locals.var_tmf2_dn0 = assign21110_e16024_d_n0;
        locals.var_tmf2_dn2 = assign21110_e16024_d_n2;
        locals.var_tmf2_dn4 = assign21110_e16024_d_n4;
        locals.var_tmf2_dn5 = assign21110_e16024_d_n5;
        locals.var_tmf2_dn6 = assign21110_e16024_d_n6;
        locals.var_tmf2_dn7 = assign21110_e16024_d_n7;
        locals.var_tmf2_dn8 = assign21110_e16024_d_n8;
        locals.var_tmf2_dn9 = assign21110_e16024_d_n9;
        locals.var_tmf2_dn10 = assign21110_e16024_d_n10;
        locals.var_tmf2_dn11 = assign21110_e16024_d_n11;
        locals.var_tmf2_dn14 = assign21110_e16024_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21120_e16037, assign21120_e16037_d_n0, assign21120_e16037_d_n2, assign21120_e16037_d_n4, assign21120_e16037_d_n5, assign21120_e16037_d_n6, assign21120_e16037_d_n7, assign21120_e16037_d_n8, assign21120_e16037_d_n9, assign21120_e16037_d_n10, assign21120_e16037_d_n11, assign21120_e16037_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21120_e16032: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign21120_e16034: f64 = (assign21120_e16032 + locals.var_tmf2);
        let assign21120_e16035: f64 = (assign21120_e16034).sqrt();
        (assign21120_e16035, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign21120_e16035)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign21120_e16035)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign21120_e16035)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign21120_e16035)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign21120_e16035)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign21120_e16035)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign21120_e16035)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign21120_e16035)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign21120_e16035)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign21120_e16035)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign21120_e16035)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21120_e16037;
        locals.var_tmf2_dn0 = assign21120_e16037_d_n0;
        locals.var_tmf2_dn2 = assign21120_e16037_d_n2;
        locals.var_tmf2_dn4 = assign21120_e16037_d_n4;
        locals.var_tmf2_dn5 = assign21120_e16037_d_n5;
        locals.var_tmf2_dn6 = assign21120_e16037_d_n6;
        locals.var_tmf2_dn7 = assign21120_e16037_d_n7;
        locals.var_tmf2_dn8 = assign21120_e16037_d_n8;
        locals.var_tmf2_dn9 = assign21120_e16037_d_n9;
        locals.var_tmf2_dn10 = assign21120_e16037_d_n10;
        locals.var_tmf2_dn11 = assign21120_e16037_d_n11;
        locals.var_tmf2_dn14 = assign21120_e16037_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21130_e16051, assign21130_e16051_d_n0, assign21130_e16051_d_n2, assign21130_e16051_d_n4, assign21130_e16051_d_n5, assign21130_e16051_d_n6, assign21130_e16051_d_n7, assign21130_e16051_d_n8, assign21130_e16051_d_n9, assign21130_e16051_d_n10, assign21130_e16051_d_n11, assign21130_e16051_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21130_e16047: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign21130_e16048: f64 = (1.0 + assign21130_e16047);
        let assign21130_e16049: f64 = (0.5 * assign21130_e16048);
        (assign21130_e16049, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21130_e16051;
        locals.var_t0_dn0 = assign21130_e16051_d_n0;
        locals.var_t0_dn2 = assign21130_e16051_d_n2;
        locals.var_t0_dn4 = assign21130_e16051_d_n4;
        locals.var_t0_dn5 = assign21130_e16051_d_n5;
        locals.var_t0_dn6 = assign21130_e16051_d_n6;
        locals.var_t0_dn7 = assign21130_e16051_d_n7;
        locals.var_t0_dn8 = assign21130_e16051_d_n8;
        locals.var_t0_dn9 = assign21130_e16051_d_n9;
        locals.var_t0_dn10 = assign21130_e16051_d_n10;
        locals.var_t0_dn11 = assign21130_e16051_d_n11;
        locals.var_t0_dn14 = assign21130_e16051_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21140_e16071, assign21140_e16071_d_n0, assign21140_e16071_d_n2, assign21140_e16071_d_n4, assign21140_e16071_d_n5, assign21140_e16071_d_n6, assign21140_e16071_d_n7, assign21140_e16071_d_n8, assign21140_e16071_d_n9, assign21140_e16071_d_n10, assign21140_e16071_d_n11, assign21140_e16071_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21140_e16062: f64 = (2.0 * 0.01);
        let assign21140_e16064: f64 = (assign21140_e16062 * 0.01);
        let assign21140_e16065: f64 = (locals.var_tmf1 - assign21140_e16064);
        let assign21140_e16067: f64 = (assign21140_e16065 / locals.var_tmf2);
        let assign21140_e16068: f64 = (1.0 - assign21140_e16067);
        let assign21140_e16069: f64 = (0.5 * assign21140_e16068);
        (assign21140_e16069, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign21140_e16065 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign21140_e16065 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign21140_e16065 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign21140_e16065 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign21140_e16065 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign21140_e16065 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign21140_e16065 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn9 * locals.var_tmf2) - (assign21140_e16065 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign21140_e16065 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign21140_e16065 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn14 * locals.var_tmf2) - (assign21140_e16065 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign21140_e16071;
        locals.var_t5_dn0 = assign21140_e16071_d_n0;
        locals.var_t5_dn2 = assign21140_e16071_d_n2;
        locals.var_t5_dn4 = assign21140_e16071_d_n4;
        locals.var_t5_dn5 = assign21140_e16071_d_n5;
        locals.var_t5_dn6 = assign21140_e16071_d_n6;
        locals.var_t5_dn7 = assign21140_e16071_d_n7;
        locals.var_t5_dn8 = assign21140_e16071_d_n8;
        locals.var_t5_dn9 = assign21140_e16071_d_n9;
        locals.var_t5_dn10 = assign21140_e16071_d_n10;
        locals.var_t5_dn11 = assign21140_e16071_d_n11;
        locals.var_t5_dn14 = assign21140_e16071_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign21150_e16085, assign21150_e16085_d_n0, assign21150_e16085_d_n2, assign21150_e16085_d_n4, assign21150_e16085_d_n5, assign21150_e16085_d_n6, assign21150_e16085_d_n7, assign21150_e16085_d_n8, assign21150_e16085_d_n9, assign21150_e16085_d_n10, assign21150_e16085_d_n11, assign21150_e16085_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21150_e16081: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign21150_e16082: f64 = (0.5 * assign21150_e16081);
        let assign21150_e16083: f64 = (locals.var_t4 + assign21150_e16082);
        (assign21150_e16083, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t4_dn14 + (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign21150_e16085;
        locals.var_t2_dn0 = assign21150_e16085_d_n0;
        locals.var_t2_dn2 = assign21150_e16085_d_n2;
        locals.var_t2_dn4 = assign21150_e16085_d_n4;
        locals.var_t2_dn5 = assign21150_e16085_d_n5;
        locals.var_t2_dn6 = assign21150_e16085_d_n6;
        locals.var_t2_dn7 = assign21150_e16085_d_n7;
        locals.var_t2_dn8 = assign21150_e16085_d_n8;
        locals.var_t2_dn9 = assign21150_e16085_d_n9;
        locals.var_t2_dn10 = assign21150_e16085_d_n10;
        locals.var_t2_dn11 = assign21150_e16085_d_n11;
        locals.var_t2_dn14 = assign21150_e16085_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_55(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21160_e16097, assign21160_e16097_d_n0, assign21160_e16097_d_n2, assign21160_e16097_d_n4, assign21160_e16097_d_n5, assign21160_e16097_d_n6, assign21160_e16097_d_n7, assign21160_e16097_d_n8, assign21160_e16097_d_n9, assign21160_e16097_d_n10, assign21160_e16097_d_n11, assign21160_e16097_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21160_e16094: f64 = (1.0 + locals.var_uc_rdvg11);
        let assign21160_e16095: f64 = (locals.var_t4 * assign21160_e16094);
        (assign21160_e16095, (locals.var_t4_dn0 * assign21160_e16094), (locals.var_t4_dn2 * assign21160_e16094), (locals.var_t4_dn4 * assign21160_e16094), (locals.var_t4_dn5 * assign21160_e16094), (locals.var_t4_dn6 * assign21160_e16094), (locals.var_t4_dn7 * assign21160_e16094), (locals.var_t4_dn8 * assign21160_e16094), (locals.var_t4_dn9 * assign21160_e16094), (locals.var_t4_dn10 * assign21160_e16094), (locals.var_t4_dn11 * assign21160_e16094), (locals.var_t4_dn14 * assign21160_e16094),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign21160_e16097;
        locals.var_t3_dn0 = assign21160_e16097_d_n0;
        locals.var_t3_dn2 = assign21160_e16097_d_n2;
        locals.var_t3_dn4 = assign21160_e16097_d_n4;
        locals.var_t3_dn5 = assign21160_e16097_d_n5;
        locals.var_t3_dn6 = assign21160_e16097_d_n6;
        locals.var_t3_dn7 = assign21160_e16097_d_n7;
        locals.var_t3_dn8 = assign21160_e16097_d_n8;
        locals.var_t3_dn9 = assign21160_e16097_d_n9;
        locals.var_t3_dn10 = assign21160_e16097_d_n10;
        locals.var_t3_dn11 = assign21160_e16097_d_n11;
        locals.var_t3_dn14 = assign21160_e16097_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign21170_e16111, assign21170_e16111_d_n0, assign21170_e16111_d_n2, assign21170_e16111_d_n4, assign21170_e16111_d_n5, assign21170_e16111_d_n6, assign21170_e16111_d_n7, assign21170_e16111_d_n8, assign21170_e16111_d_n9, assign21170_e16111_d_n10, assign21170_e16111_d_n11, assign21170_e16111_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21170_e16105: f64 = (locals.var_t3 - locals.var_t2);
        let assign21170_e16108: f64 = (5e-5 * 0.01);
        let assign21170_e16109: f64 = (assign21170_e16105 - assign21170_e16108);
        (assign21170_e16109, (locals.var_t3_dn0 - locals.var_t2_dn0), (locals.var_t3_dn2 - locals.var_t2_dn2), (locals.var_t3_dn4 - locals.var_t2_dn4), (locals.var_t3_dn5 - locals.var_t2_dn5), (locals.var_t3_dn6 - locals.var_t2_dn6), (locals.var_t3_dn7 - locals.var_t2_dn7), (locals.var_t3_dn8 - locals.var_t2_dn8), (locals.var_t3_dn9 - locals.var_t2_dn9), (locals.var_t3_dn10 - locals.var_t2_dn10), (locals.var_t3_dn11 - locals.var_t2_dn11), (locals.var_t3_dn14 - locals.var_t2_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign21170_e16111;
        locals.var_tmf1_dn0 = assign21170_e16111_d_n0;
        locals.var_tmf1_dn2 = assign21170_e16111_d_n2;
        locals.var_tmf1_dn4 = assign21170_e16111_d_n4;
        locals.var_tmf1_dn5 = assign21170_e16111_d_n5;
        locals.var_tmf1_dn6 = assign21170_e16111_d_n6;
        locals.var_tmf1_dn7 = assign21170_e16111_d_n7;
        locals.var_tmf1_dn8 = assign21170_e16111_d_n8;
        locals.var_tmf1_dn9 = assign21170_e16111_d_n9;
        locals.var_tmf1_dn10 = assign21170_e16111_d_n10;
        locals.var_tmf1_dn11 = assign21170_e16111_d_n11;
        locals.var_tmf1_dn14 = assign21170_e16111_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign21180_e16125, assign21180_e16125_d_n0, assign21180_e16125_d_n2, assign21180_e16125_d_n4, assign21180_e16125_d_n5, assign21180_e16125_d_n6, assign21180_e16125_d_n7, assign21180_e16125_d_n8, assign21180_e16125_d_n9, assign21180_e16125_d_n10, assign21180_e16125_d_n11, assign21180_e16125_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21180_e16119: f64 = (4.0 * locals.var_t3);
        let assign21180_e16122: f64 = (5e-5 * 0.01);
        let assign21180_e16123: f64 = (assign21180_e16119 * assign21180_e16122);
        (assign21180_e16123, ((4.0 * locals.var_t3_dn0) * assign21180_e16122), ((4.0 * locals.var_t3_dn2) * assign21180_e16122), ((4.0 * locals.var_t3_dn4) * assign21180_e16122), ((4.0 * locals.var_t3_dn5) * assign21180_e16122), ((4.0 * locals.var_t3_dn6) * assign21180_e16122), ((4.0 * locals.var_t3_dn7) * assign21180_e16122), ((4.0 * locals.var_t3_dn8) * assign21180_e16122), ((4.0 * locals.var_t3_dn9) * assign21180_e16122), ((4.0 * locals.var_t3_dn10) * assign21180_e16122), ((4.0 * locals.var_t3_dn11) * assign21180_e16122), ((4.0 * locals.var_t3_dn14) * assign21180_e16122),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21180_e16125;
        locals.var_tmf2_dn0 = assign21180_e16125_d_n0;
        locals.var_tmf2_dn2 = assign21180_e16125_d_n2;
        locals.var_tmf2_dn4 = assign21180_e16125_d_n4;
        locals.var_tmf2_dn5 = assign21180_e16125_d_n5;
        locals.var_tmf2_dn6 = assign21180_e16125_d_n6;
        locals.var_tmf2_dn7 = assign21180_e16125_d_n7;
        locals.var_tmf2_dn8 = assign21180_e16125_d_n8;
        locals.var_tmf2_dn9 = assign21180_e16125_d_n9;
        locals.var_tmf2_dn10 = assign21180_e16125_d_n10;
        locals.var_tmf2_dn11 = assign21180_e16125_d_n11;
        locals.var_tmf2_dn14 = assign21180_e16125_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21190_e16139, assign21190_e16139_d_n0, assign21190_e16139_d_n2, assign21190_e16139_d_n4, assign21190_e16139_d_n5, assign21190_e16139_d_n6, assign21190_e16139_d_n7, assign21190_e16139_d_n8, assign21190_e16139_d_n9, assign21190_e16139_d_n10, assign21190_e16139_d_n11, assign21190_e16139_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let (assign21190_e16137, assign21190_e16137_d_n0, assign21190_e16137_d_n2, assign21190_e16137_d_n4, assign21190_e16137_d_n5, assign21190_e16137_d_n6, assign21190_e16137_d_n7, assign21190_e16137_d_n8, assign21190_e16137_d_n9, assign21190_e16137_d_n10, assign21190_e16137_d_n11, assign21190_e16137_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign21190_e16136: f64 = (-locals.var_tmf2);
                (assign21190_e16136, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign21190_e16137, assign21190_e16137_d_n0, assign21190_e16137_d_n2, assign21190_e16137_d_n4, assign21190_e16137_d_n5, assign21190_e16137_d_n6, assign21190_e16137_d_n7, assign21190_e16137_d_n8, assign21190_e16137_d_n9, assign21190_e16137_d_n10, assign21190_e16137_d_n11, assign21190_e16137_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21190_e16139;
        locals.var_tmf2_dn0 = assign21190_e16139_d_n0;
        locals.var_tmf2_dn2 = assign21190_e16139_d_n2;
        locals.var_tmf2_dn4 = assign21190_e16139_d_n4;
        locals.var_tmf2_dn5 = assign21190_e16139_d_n5;
        locals.var_tmf2_dn6 = assign21190_e16139_d_n6;
        locals.var_tmf2_dn7 = assign21190_e16139_d_n7;
        locals.var_tmf2_dn8 = assign21190_e16139_d_n8;
        locals.var_tmf2_dn9 = assign21190_e16139_d_n9;
        locals.var_tmf2_dn10 = assign21190_e16139_d_n10;
        locals.var_tmf2_dn11 = assign21190_e16139_d_n11;
        locals.var_tmf2_dn14 = assign21190_e16139_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21200_e16152, assign21200_e16152_d_n0, assign21200_e16152_d_n2, assign21200_e16152_d_n4, assign21200_e16152_d_n5, assign21200_e16152_d_n6, assign21200_e16152_d_n7, assign21200_e16152_d_n8, assign21200_e16152_d_n9, assign21200_e16152_d_n10, assign21200_e16152_d_n11, assign21200_e16152_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21200_e16147: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign21200_e16149: f64 = (assign21200_e16147 + locals.var_tmf2);
        let assign21200_e16150: f64 = (assign21200_e16149).sqrt();
        (assign21200_e16150, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign21200_e16150)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign21200_e16150)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign21200_e16150)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign21200_e16150)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign21200_e16150)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign21200_e16150)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign21200_e16150)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign21200_e16150)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign21200_e16150)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign21200_e16150)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign21200_e16150)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21200_e16152;
        locals.var_tmf2_dn0 = assign21200_e16152_d_n0;
        locals.var_tmf2_dn2 = assign21200_e16152_d_n2;
        locals.var_tmf2_dn4 = assign21200_e16152_d_n4;
        locals.var_tmf2_dn5 = assign21200_e16152_d_n5;
        locals.var_tmf2_dn6 = assign21200_e16152_d_n6;
        locals.var_tmf2_dn7 = assign21200_e16152_d_n7;
        locals.var_tmf2_dn8 = assign21200_e16152_d_n8;
        locals.var_tmf2_dn9 = assign21200_e16152_d_n9;
        locals.var_tmf2_dn10 = assign21200_e16152_d_n10;
        locals.var_tmf2_dn11 = assign21200_e16152_d_n11;
        locals.var_tmf2_dn14 = assign21200_e16152_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21210_e16166, assign21210_e16166_d_n0, assign21210_e16166_d_n2, assign21210_e16166_d_n4, assign21210_e16166_d_n5, assign21210_e16166_d_n6, assign21210_e16166_d_n7, assign21210_e16166_d_n8, assign21210_e16166_d_n9, assign21210_e16166_d_n10, assign21210_e16166_d_n11, assign21210_e16166_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21210_e16162: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign21210_e16163: f64 = (1.0 + assign21210_e16162);
        let assign21210_e16164: f64 = (0.5 * assign21210_e16163);
        (assign21210_e16164, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21210_e16166;
        locals.var_t0_dn0 = assign21210_e16166_d_n0;
        locals.var_t0_dn2 = assign21210_e16166_d_n2;
        locals.var_t0_dn4 = assign21210_e16166_d_n4;
        locals.var_t0_dn5 = assign21210_e16166_d_n5;
        locals.var_t0_dn6 = assign21210_e16166_d_n6;
        locals.var_t0_dn7 = assign21210_e16166_d_n7;
        locals.var_t0_dn8 = assign21210_e16166_d_n8;
        locals.var_t0_dn9 = assign21210_e16166_d_n9;
        locals.var_t0_dn10 = assign21210_e16166_d_n10;
        locals.var_t0_dn11 = assign21210_e16166_d_n11;
        locals.var_t0_dn14 = assign21210_e16166_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21220_e16186, assign21220_e16186_d_n0, assign21220_e16186_d_n2, assign21220_e16186_d_n4, assign21220_e16186_d_n5, assign21220_e16186_d_n6, assign21220_e16186_d_n7, assign21220_e16186_d_n8, assign21220_e16186_d_n9, assign21220_e16186_d_n10, assign21220_e16186_d_n11, assign21220_e16186_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21220_e16177: f64 = (2.0 * 5e-5);
        let assign21220_e16179: f64 = (assign21220_e16177 * 0.01);
        let assign21220_e16180: f64 = (locals.var_tmf1 + assign21220_e16179);
        let assign21220_e16182: f64 = (assign21220_e16180 / locals.var_tmf2);
        let assign21220_e16183: f64 = (1.0 - assign21220_e16182);
        let assign21220_e16184: f64 = (0.5 * assign21220_e16183);
        (assign21220_e16184, (0.5 * (-(((locals.var_tmf1_dn0 * locals.var_tmf2) - (assign21220_e16180 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn2 * locals.var_tmf2) - (assign21220_e16180 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn4 * locals.var_tmf2) - (assign21220_e16180 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn5 * locals.var_tmf2) - (assign21220_e16180 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn6 * locals.var_tmf2) - (assign21220_e16180 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn7 * locals.var_tmf2) - (assign21220_e16180 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn8 * locals.var_tmf2) - (assign21220_e16180 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn9 * locals.var_tmf2) - (assign21220_e16180 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn10 * locals.var_tmf2) - (assign21220_e16180 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn11 * locals.var_tmf2) - (assign21220_e16180 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-(((locals.var_tmf1_dn14 * locals.var_tmf2) - (assign21220_e16180 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign21220_e16186;
        locals.var_t5_dn0 = assign21220_e16186_d_n0;
        locals.var_t5_dn2 = assign21220_e16186_d_n2;
        locals.var_t5_dn4 = assign21220_e16186_d_n4;
        locals.var_t5_dn5 = assign21220_e16186_d_n5;
        locals.var_t5_dn6 = assign21220_e16186_d_n6;
        locals.var_t5_dn7 = assign21220_e16186_d_n7;
        locals.var_t5_dn8 = assign21220_e16186_d_n8;
        locals.var_t5_dn9 = assign21220_e16186_d_n9;
        locals.var_t5_dn10 = assign21220_e16186_d_n10;
        locals.var_t5_dn11 = assign21220_e16186_d_n11;
        locals.var_t5_dn14 = assign21220_e16186_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign21230_e16200, assign21230_e16200_d_n0, assign21230_e16200_d_n2, assign21230_e16200_d_n4, assign21230_e16200_d_n5, assign21230_e16200_d_n6, assign21230_e16200_d_n7, assign21230_e16200_d_n8, assign21230_e16200_d_n9, assign21230_e16200_d_n10, assign21230_e16200_d_n11, assign21230_e16200_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21230_e16196: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign21230_e16197: f64 = (0.5 * assign21230_e16196);
        let assign21230_e16198: f64 = (locals.var_t3 - assign21230_e16197);
        (assign21230_e16198, (locals.var_t3_dn0 - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t3_dn2 - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t3_dn4 - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t3_dn5 - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t3_dn6 - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t3_dn7 - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t3_dn8 - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t3_dn9 - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t3_dn10 - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t3_dn11 - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t3_dn14 - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn11, locals.var_rsdrift_dn14,)
    }
};
        locals.var_rsdrift = assign21230_e16200;
        locals.var_rsdrift_dn0 = assign21230_e16200_d_n0;
        locals.var_rsdrift_dn2 = assign21230_e16200_d_n2;
        locals.var_rsdrift_dn4 = assign21230_e16200_d_n4;
        locals.var_rsdrift_dn5 = assign21230_e16200_d_n5;
        locals.var_rsdrift_dn6 = assign21230_e16200_d_n6;
        locals.var_rsdrift_dn7 = assign21230_e16200_d_n7;
        locals.var_rsdrift_dn8 = assign21230_e16200_d_n8;
        locals.var_rsdrift_dn9 = assign21230_e16200_d_n9;
        locals.var_rsdrift_dn10 = assign21230_e16200_d_n10;
        locals.var_rsdrift_dn11 = assign21230_e16200_d_n11;
        locals.var_rsdrift_dn14 = assign21230_e16200_d_n14;
        locals.var_rsdrift_rv = 0.0;

        let (assign21240_e16212, assign21240_e16212_d_n0, assign21240_e16212_d_n2, assign21240_e16212_d_n4, assign21240_e16212_d_n5, assign21240_e16212_d_n6, assign21240_e16212_d_n7, assign21240_e16212_d_n8, assign21240_e16212_d_n9, assign21240_e16212_d_n10, assign21240_e16212_d_n11, assign21240_e16212_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21240_e16209: f64 = (locals.var_uc_rdvb * locals.var_vbserevz);
        let assign21240_e16210: f64 = (1.0 - assign21240_e16209);
        (assign21240_e16210, (-(locals.var_uc_rdvb * locals.var_vbserevz_dn0)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn2)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn4)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn5)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn6)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn7)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn8)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn9)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn10)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn11)), (-(locals.var_uc_rdvb * locals.var_vbserevz_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21240_e16212;
        locals.var_t1_dn0 = assign21240_e16212_d_n0;
        locals.var_t1_dn2 = assign21240_e16212_d_n2;
        locals.var_t1_dn4 = assign21240_e16212_d_n4;
        locals.var_t1_dn5 = assign21240_e16212_d_n5;
        locals.var_t1_dn6 = assign21240_e16212_d_n6;
        locals.var_t1_dn7 = assign21240_e16212_d_n7;
        locals.var_t1_dn8 = assign21240_e16212_d_n8;
        locals.var_t1_dn9 = assign21240_e16212_d_n9;
        locals.var_t1_dn10 = assign21240_e16212_d_n10;
        locals.var_t1_dn11 = assign21240_e16212_d_n11;
        locals.var_t1_dn14 = assign21240_e16212_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign21250_e16233, assign21250_e16233_d_n0, assign21250_e16233_d_n2, assign21250_e16233_d_n4, assign21250_e16233_d_n5, assign21250_e16233_d_n6, assign21250_e16233_d_n7, assign21250_e16233_d_n8, assign21250_e16233_d_n9, assign21250_e16233_d_n10, assign21250_e16233_d_n11, assign21250_e16233_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21250_e16220: f64 = (locals.var_t1 * locals.var_t1);
        let assign21250_e16224: f64 = (0.0001 * 0.01);
        let assign21250_e16225: f64 = (4.0 * assign21250_e16224);
        let assign21250_e16228: f64 = (0.0001 * 0.01);
        let assign21250_e16229: f64 = (assign21250_e16225 * assign21250_e16228);
        let assign21250_e16230: f64 = (assign21250_e16220 + assign21250_e16229);
        let assign21250_e16231: f64 = (assign21250_e16230).sqrt();
        (assign21250_e16231, (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign21250_e16231)), (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign21250_e16231)), (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign21250_e16231)), (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign21250_e16231)), (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign21250_e16231)), (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign21250_e16231)), (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign21250_e16231)), (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign21250_e16231)), (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign21250_e16231)), (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign21250_e16231)), (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign21250_e16231)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21250_e16233;
        locals.var_tmf2_dn0 = assign21250_e16233_d_n0;
        locals.var_tmf2_dn2 = assign21250_e16233_d_n2;
        locals.var_tmf2_dn4 = assign21250_e16233_d_n4;
        locals.var_tmf2_dn5 = assign21250_e16233_d_n5;
        locals.var_tmf2_dn6 = assign21250_e16233_d_n6;
        locals.var_tmf2_dn7 = assign21250_e16233_d_n7;
        locals.var_tmf2_dn8 = assign21250_e16233_d_n8;
        locals.var_tmf2_dn9 = assign21250_e16233_d_n9;
        locals.var_tmf2_dn10 = assign21250_e16233_d_n10;
        locals.var_tmf2_dn11 = assign21250_e16233_d_n11;
        locals.var_tmf2_dn14 = assign21250_e16233_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21260_e16247, assign21260_e16247_d_n0, assign21260_e16247_d_n2, assign21260_e16247_d_n4, assign21260_e16247_d_n5, assign21260_e16247_d_n6, assign21260_e16247_d_n7, assign21260_e16247_d_n8, assign21260_e16247_d_n9, assign21260_e16247_d_n10, assign21260_e16247_d_n11, assign21260_e16247_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21260_e16243: f64 = (locals.var_t1 / locals.var_tmf2);
        let assign21260_e16244: f64 = (1.0 + assign21260_e16243);
        let assign21260_e16245: f64 = (0.5 * assign21260_e16244);
        (assign21260_e16245, (0.5 * (((locals.var_t1_dn0 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn2 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn4 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn5 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn6 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn7 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn8 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn9 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn10 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn11 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t1_dn14 * locals.var_tmf2) - (locals.var_t1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21260_e16247;
        locals.var_t4_dn0 = assign21260_e16247_d_n0;
        locals.var_t4_dn2 = assign21260_e16247_d_n2;
        locals.var_t4_dn4 = assign21260_e16247_d_n4;
        locals.var_t4_dn5 = assign21260_e16247_d_n5;
        locals.var_t4_dn6 = assign21260_e16247_d_n6;
        locals.var_t4_dn7 = assign21260_e16247_d_n7;
        locals.var_t4_dn8 = assign21260_e16247_d_n8;
        locals.var_t4_dn9 = assign21260_e16247_d_n9;
        locals.var_t4_dn10 = assign21260_e16247_d_n10;
        locals.var_t4_dn11 = assign21260_e16247_d_n11;
        locals.var_t4_dn14 = assign21260_e16247_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign21270_e16259, assign21270_e16259_d_n0, assign21270_e16259_d_n2, assign21270_e16259_d_n4, assign21270_e16259_d_n5, assign21270_e16259_d_n6, assign21270_e16259_d_n7, assign21270_e16259_d_n8, assign21270_e16259_d_n9, assign21270_e16259_d_n10, assign21270_e16259_d_n11, assign21270_e16259_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21270_e16256: f64 = (locals.var_t1 + locals.var_tmf2);
        let assign21270_e16257: f64 = (0.5 * assign21270_e16256);
        (assign21270_e16257, (0.5 * (locals.var_t1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign21270_e16259;
        locals.var_t3_dn0 = assign21270_e16259_d_n0;
        locals.var_t3_dn2 = assign21270_e16259_d_n2;
        locals.var_t3_dn4 = assign21270_e16259_d_n4;
        locals.var_t3_dn5 = assign21270_e16259_d_n5;
        locals.var_t3_dn6 = assign21270_e16259_d_n6;
        locals.var_t3_dn7 = assign21270_e16259_d_n7;
        locals.var_t3_dn8 = assign21270_e16259_d_n8;
        locals.var_t3_dn9 = assign21270_e16259_d_n9;
        locals.var_t3_dn10 = assign21270_e16259_d_n10;
        locals.var_t3_dn11 = assign21270_e16259_d_n11;
        locals.var_t3_dn14 = assign21270_e16259_d_n14;
        locals.var_t3_rv = 0.0;

        let assign21280_e16262: f64 = if locals.var_t3 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard419 = assign21280_e16262;
        locals.var_guard419_rv = 0.0;

        let (assign21290_e16272, assign21290_e16272_d_n0, assign21290_e16272_d_n2, assign21290_e16272_d_n4, assign21290_e16272_d_n5, assign21290_e16272_d_n6, assign21290_e16272_d_n7, assign21290_e16272_d_n8, assign21290_e16272_d_n9, assign21290_e16272_d_n10, assign21290_e16272_d_n11, assign21290_e16272_d_n14,) = {
    if ((((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) && (locals.var_guard419 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign21290_e16272;
        locals.var_t3_dn0 = assign21290_e16272_d_n0;
        locals.var_t3_dn2 = assign21290_e16272_d_n2;
        locals.var_t3_dn4 = assign21290_e16272_d_n4;
        locals.var_t3_dn5 = assign21290_e16272_d_n5;
        locals.var_t3_dn6 = assign21290_e16272_d_n6;
        locals.var_t3_dn7 = assign21290_e16272_d_n7;
        locals.var_t3_dn8 = assign21290_e16272_d_n8;
        locals.var_t3_dn9 = assign21290_e16272_d_n9;
        locals.var_t3_dn10 = assign21290_e16272_d_n10;
        locals.var_t3_dn11 = assign21290_e16272_d_n11;
        locals.var_t3_dn14 = assign21290_e16272_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign21300_e16282, assign21300_e16282_d_n0, assign21300_e16282_d_n2, assign21300_e16282_d_n4, assign21300_e16282_d_n5, assign21300_e16282_d_n6, assign21300_e16282_d_n7, assign21300_e16282_d_n8, assign21300_e16282_d_n9, assign21300_e16282_d_n10, assign21300_e16282_d_n11, assign21300_e16282_d_n14,) = {
    if ((((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) && (locals.var_guard419 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21300_e16282;
        locals.var_t4_dn0 = assign21300_e16282_d_n0;
        locals.var_t4_dn2 = assign21300_e16282_d_n2;
        locals.var_t4_dn4 = assign21300_e16282_d_n4;
        locals.var_t4_dn5 = assign21300_e16282_d_n5;
        locals.var_t4_dn6 = assign21300_e16282_d_n6;
        locals.var_t4_dn7 = assign21300_e16282_d_n7;
        locals.var_t4_dn8 = assign21300_e16282_d_n8;
        locals.var_t4_dn9 = assign21300_e16282_d_n9;
        locals.var_t4_dn10 = assign21300_e16282_d_n10;
        locals.var_t4_dn11 = assign21300_e16282_d_n11;
        locals.var_t4_dn14 = assign21300_e16282_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign21310_e16292, assign21310_e16292_d_n0, assign21310_e16292_d_n2, assign21310_e16292_d_n4, assign21310_e16292_d_n5, assign21310_e16292_d_n6, assign21310_e16292_d_n7, assign21310_e16292_d_n8, assign21310_e16292_d_n9, assign21310_e16292_d_n10, assign21310_e16292_d_n11, assign21310_e16292_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21310_e16290: f64 = (locals.var_t3 + 1e-25);
        (assign21310_e16290, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign21310_e16292;
        locals.var_t3_dn0 = assign21310_e16292_d_n0;
        locals.var_t3_dn2 = assign21310_e16292_d_n2;
        locals.var_t3_dn4 = assign21310_e16292_d_n4;
        locals.var_t3_dn5 = assign21310_e16292_d_n5;
        locals.var_t3_dn6 = assign21310_e16292_d_n6;
        locals.var_t3_dn7 = assign21310_e16292_d_n7;
        locals.var_t3_dn8 = assign21310_e16292_d_n8;
        locals.var_t3_dn9 = assign21310_e16292_d_n9;
        locals.var_t3_dn10 = assign21310_e16292_d_n10;
        locals.var_t3_dn11 = assign21310_e16292_d_n11;
        locals.var_t3_dn14 = assign21310_e16292_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign21320_e16300, assign21320_e16300_d_n0, assign21320_e16300_d_n2, assign21320_e16300_d_n4, assign21320_e16300_d_n5, assign21320_e16300_d_n6, assign21320_e16300_d_n7, assign21320_e16300_d_n8, assign21320_e16300_d_n9, assign21320_e16300_d_n10, assign21320_e16300_d_n11, assign21320_e16300_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn11, locals.var_rsdrift_dn14,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21320_e16300;
        locals.var_t0_dn0 = assign21320_e16300_d_n0;
        locals.var_t0_dn2 = assign21320_e16300_d_n2;
        locals.var_t0_dn4 = assign21320_e16300_d_n4;
        locals.var_t0_dn5 = assign21320_e16300_d_n5;
        locals.var_t0_dn6 = assign21320_e16300_d_n6;
        locals.var_t0_dn7 = assign21320_e16300_d_n7;
        locals.var_t0_dn8 = assign21320_e16300_d_n8;
        locals.var_t0_dn9 = assign21320_e16300_d_n9;
        locals.var_t0_dn10 = assign21320_e16300_d_n10;
        locals.var_t0_dn11 = assign21320_e16300_d_n11;
        locals.var_t0_dn14 = assign21320_e16300_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21330_e16310, assign21330_e16310_d_n0, assign21330_e16310_d_n2, assign21330_e16310_d_n4, assign21330_e16310_d_n5, assign21330_e16310_d_n6, assign21330_e16310_d_n7, assign21330_e16310_d_n8, assign21330_e16310_d_n9, assign21330_e16310_d_n10, assign21330_e16310_d_n11, assign21330_e16310_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 != 0.0)) {
        let assign21330_e16308: f64 = (locals.var_rsdrift * locals.var_t3);
        (assign21330_e16308, ((locals.var_rsdrift_dn0 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn0)), ((locals.var_rsdrift_dn2 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn2)), ((locals.var_rsdrift_dn4 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn4)), ((locals.var_rsdrift_dn5 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn5)), ((locals.var_rsdrift_dn6 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn6)), ((locals.var_rsdrift_dn7 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn7)), ((locals.var_rsdrift_dn8 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn8)), ((locals.var_rsdrift_dn9 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn9)), ((locals.var_rsdrift_dn10 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn10)), ((locals.var_rsdrift_dn11 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn11)), ((locals.var_rsdrift_dn14 * locals.var_t3) + (locals.var_rsdrift * locals.var_t3_dn14)),)
    } else {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn11, locals.var_rsdrift_dn14,)
    }
};
        locals.var_rsdrift = assign21330_e16310;
        locals.var_rsdrift_dn0 = assign21330_e16310_d_n0;
        locals.var_rsdrift_dn2 = assign21330_e16310_d_n2;
        locals.var_rsdrift_dn4 = assign21330_e16310_d_n4;
        locals.var_rsdrift_dn5 = assign21330_e16310_d_n5;
        locals.var_rsdrift_dn6 = assign21330_e16310_d_n6;
        locals.var_rsdrift_dn7 = assign21330_e16310_d_n7;
        locals.var_rsdrift_dn8 = assign21330_e16310_d_n8;
        locals.var_rsdrift_dn9 = assign21330_e16310_d_n9;
        locals.var_rsdrift_dn10 = assign21330_e16310_d_n10;
        locals.var_rsdrift_dn11 = assign21330_e16310_d_n11;
        locals.var_rsdrift_dn14 = assign21330_e16310_d_n14;
        locals.var_rsdrift_rv = 0.0;

        let (assign21340_e16319, assign21340_e16319_d_n0, assign21340_e16319_d_n2, assign21340_e16319_d_n4, assign21340_e16319_d_n5, assign21340_e16319_d_n6, assign21340_e16319_d_n7, assign21340_e16319_d_n8, assign21340_e16319_d_n9, assign21340_e16319_d_n10, assign21340_e16319_d_n11, assign21340_e16319_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard418 == 0.0)) {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    } else {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn11, locals.var_rsdrift_dn14,)
    }
};
        locals.var_rsdrift = assign21340_e16319;
        locals.var_rsdrift_dn0 = assign21340_e16319_d_n0;
        locals.var_rsdrift_dn2 = assign21340_e16319_d_n2;
        locals.var_rsdrift_dn4 = assign21340_e16319_d_n4;
        locals.var_rsdrift_dn5 = assign21340_e16319_d_n5;
        locals.var_rsdrift_dn6 = assign21340_e16319_d_n6;
        locals.var_rsdrift_dn7 = assign21340_e16319_d_n7;
        locals.var_rsdrift_dn8 = assign21340_e16319_d_n8;
        locals.var_rsdrift_dn9 = assign21340_e16319_d_n9;
        locals.var_rsdrift_dn10 = assign21340_e16319_d_n10;
        locals.var_rsdrift_dn11 = assign21340_e16319_d_n11;
        locals.var_rsdrift_dn14 = assign21340_e16319_d_n14;
        locals.var_rsdrift_rv = 0.0;

        let assign21350_e16330: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign21350_e16331: f64 = (locals.var_uc_nover * assign21350_e16330);
        let assign21350_e16334: f64 = if (((p.p54 == 1.0) && (p.p34 == 0.0)) && (assign21350_e16331 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard420 = assign21350_e16334;
        locals.var_guard420_rv = 0.0;

        let (assign21360_e16350, assign21360_e16350_d_n0, assign21360_e16350_d_n2, assign21360_e16350_d_n4, assign21360_e16350_d_n5, assign21360_e16350_d_n6, assign21360_e16350_d_n7, assign21360_e16350_d_n8, assign21360_e16350_d_n9, assign21360_e16350_d_n10, assign21360_e16350_d_n11, assign21360_e16350_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21360_e16343: f64 = (p.p333 * locals.var_vdserevz);
        let assign21360_e16344: f64 = (p.p335 - assign21360_e16343);
        let assign21360_e16347: f64 = (p.p332 * locals.var_vsubsrev);
        let assign21360_e16348: f64 = (assign21360_e16344 - assign21360_e16347);
        (assign21360_e16348, ((-(p.p333 * locals.var_vdserevz_dn0)) - (p.p332 * locals.var_vsubsrev_dn0)), ((-(p.p333 * locals.var_vdserevz_dn2)) - (p.p332 * locals.var_vsubsrev_dn2)), ((-(p.p333 * locals.var_vdserevz_dn4)) - (p.p332 * locals.var_vsubsrev_dn4)), (-(p.p333 * locals.var_vdserevz_dn5)), (-(p.p333 * locals.var_vdserevz_dn6)), (-(p.p333 * locals.var_vdserevz_dn7)), (-(p.p333 * locals.var_vdserevz_dn8)), (-(p.p333 * locals.var_vdserevz_dn9)), (-(p.p333 * locals.var_vdserevz_dn10)), (-(p.p333 * locals.var_vdserevz_dn11)), (-(p.p333 * locals.var_vdserevz_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21360_e16350;
        locals.var_t0_dn0 = assign21360_e16350_d_n0;
        locals.var_t0_dn2 = assign21360_e16350_d_n2;
        locals.var_t0_dn4 = assign21360_e16350_d_n4;
        locals.var_t0_dn5 = assign21360_e16350_d_n5;
        locals.var_t0_dn6 = assign21360_e16350_d_n6;
        locals.var_t0_dn7 = assign21360_e16350_d_n7;
        locals.var_t0_dn8 = assign21360_e16350_d_n8;
        locals.var_t0_dn9 = assign21360_e16350_d_n9;
        locals.var_t0_dn10 = assign21360_e16350_d_n10;
        locals.var_t0_dn11 = assign21360_e16350_d_n11;
        locals.var_t0_dn14 = assign21360_e16350_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21370_e16367, assign21370_e16367_d_n0, assign21370_e16367_d_n2, assign21370_e16367_d_n4, assign21370_e16367_d_n5, assign21370_e16367_d_n6, assign21370_e16367_d_n7, assign21370_e16367_d_n8, assign21370_e16367_d_n9, assign21370_e16367_d_n10, assign21370_e16367_d_n11, assign21370_e16367_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21370_e16358: f64 = (locals.var_t0 * locals.var_t0);
        let assign21370_e16361: f64 = (4.0 * 10.0);
        let assign21370_e16363: f64 = (assign21370_e16361 * 10.0);
        let assign21370_e16364: f64 = (assign21370_e16358 + assign21370_e16363);
        let assign21370_e16365: f64 = (assign21370_e16364).sqrt();
        (assign21370_e16365, (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / (2.0 * assign21370_e16365)), (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / (2.0 * assign21370_e16365)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign21370_e16365)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign21370_e16365)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign21370_e16365)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign21370_e16365)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign21370_e16365)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign21370_e16365)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign21370_e16365)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign21370_e16365)), (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / (2.0 * assign21370_e16365)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21370_e16367;
        locals.var_tmf2_dn0 = assign21370_e16367_d_n0;
        locals.var_tmf2_dn2 = assign21370_e16367_d_n2;
        locals.var_tmf2_dn4 = assign21370_e16367_d_n4;
        locals.var_tmf2_dn5 = assign21370_e16367_d_n5;
        locals.var_tmf2_dn6 = assign21370_e16367_d_n6;
        locals.var_tmf2_dn7 = assign21370_e16367_d_n7;
        locals.var_tmf2_dn8 = assign21370_e16367_d_n8;
        locals.var_tmf2_dn9 = assign21370_e16367_d_n9;
        locals.var_tmf2_dn10 = assign21370_e16367_d_n10;
        locals.var_tmf2_dn11 = assign21370_e16367_d_n11;
        locals.var_tmf2_dn14 = assign21370_e16367_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21380_e16381, assign21380_e16381_d_n0, assign21380_e16381_d_n2, assign21380_e16381_d_n4, assign21380_e16381_d_n5, assign21380_e16381_d_n6, assign21380_e16381_d_n7, assign21380_e16381_d_n8, assign21380_e16381_d_n9, assign21380_e16381_d_n10, assign21380_e16381_d_n11, assign21380_e16381_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21380_e16377: f64 = (locals.var_t0 / locals.var_tmf2);
        let assign21380_e16378: f64 = (1.0 + assign21380_e16377);
        let assign21380_e16379: f64 = (0.5 * assign21380_e16378);
        (assign21380_e16379, (0.5 * (((locals.var_t0_dn0 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn2 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn4 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn5 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn6 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn7 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn8 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn9 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn10 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn11 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t0_dn14 * locals.var_tmf2) - (locals.var_t0 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign21380_e16381;
        locals.var_t2_dn0 = assign21380_e16381_d_n0;
        locals.var_t2_dn2 = assign21380_e16381_d_n2;
        locals.var_t2_dn4 = assign21380_e16381_d_n4;
        locals.var_t2_dn5 = assign21380_e16381_d_n5;
        locals.var_t2_dn6 = assign21380_e16381_d_n6;
        locals.var_t2_dn7 = assign21380_e16381_d_n7;
        locals.var_t2_dn8 = assign21380_e16381_d_n8;
        locals.var_t2_dn9 = assign21380_e16381_d_n9;
        locals.var_t2_dn10 = assign21380_e16381_d_n10;
        locals.var_t2_dn11 = assign21380_e16381_d_n11;
        locals.var_t2_dn14 = assign21380_e16381_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_56(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21390_e16393, assign21390_e16393_d_n0, assign21390_e16393_d_n2, assign21390_e16393_d_n4, assign21390_e16393_d_n5, assign21390_e16393_d_n6, assign21390_e16393_d_n7, assign21390_e16393_d_n8, assign21390_e16393_d_n9, assign21390_e16393_d_n10, assign21390_e16393_d_n11, assign21390_e16393_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21390_e16390: f64 = (locals.var_t0 + locals.var_tmf2);
        let assign21390_e16391: f64 = (0.5 * assign21390_e16390);
        (assign21390_e16391, (0.5 * (locals.var_t0_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t0_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t0_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t0_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t0_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t0_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t0_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t0_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t0_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t0_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t0_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21390_e16393;
        locals.var_t1_dn0 = assign21390_e16393_d_n0;
        locals.var_t1_dn2 = assign21390_e16393_d_n2;
        locals.var_t1_dn4 = assign21390_e16393_d_n4;
        locals.var_t1_dn5 = assign21390_e16393_d_n5;
        locals.var_t1_dn6 = assign21390_e16393_d_n6;
        locals.var_t1_dn7 = assign21390_e16393_d_n7;
        locals.var_t1_dn8 = assign21390_e16393_d_n8;
        locals.var_t1_dn9 = assign21390_e16393_d_n9;
        locals.var_t1_dn10 = assign21390_e16393_d_n10;
        locals.var_t1_dn11 = assign21390_e16393_d_n11;
        locals.var_t1_dn14 = assign21390_e16393_d_n14;
        locals.var_t1_rv = 0.0;

        let assign21400_e16396: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard421 = assign21400_e16396;
        locals.var_guard421_rv = 0.0;

        let (assign21410_e16406, assign21410_e16406_d_n0, assign21410_e16406_d_n2, assign21410_e16406_d_n4, assign21410_e16406_d_n5, assign21410_e16406_d_n6, assign21410_e16406_d_n7, assign21410_e16406_d_n8, assign21410_e16406_d_n9, assign21410_e16406_d_n10, assign21410_e16406_d_n11, assign21410_e16406_d_n14,) = {
    if ((((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 != 0.0)) && (locals.var_guard421 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21410_e16406;
        locals.var_t1_dn0 = assign21410_e16406_d_n0;
        locals.var_t1_dn2 = assign21410_e16406_d_n2;
        locals.var_t1_dn4 = assign21410_e16406_d_n4;
        locals.var_t1_dn5 = assign21410_e16406_d_n5;
        locals.var_t1_dn6 = assign21410_e16406_d_n6;
        locals.var_t1_dn7 = assign21410_e16406_d_n7;
        locals.var_t1_dn8 = assign21410_e16406_d_n8;
        locals.var_t1_dn9 = assign21410_e16406_d_n9;
        locals.var_t1_dn10 = assign21410_e16406_d_n10;
        locals.var_t1_dn11 = assign21410_e16406_d_n11;
        locals.var_t1_dn14 = assign21410_e16406_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign21420_e16416, assign21420_e16416_d_n0, assign21420_e16416_d_n2, assign21420_e16416_d_n4, assign21420_e16416_d_n5, assign21420_e16416_d_n6, assign21420_e16416_d_n7, assign21420_e16416_d_n8, assign21420_e16416_d_n9, assign21420_e16416_d_n10, assign21420_e16416_d_n11, assign21420_e16416_d_n14,) = {
    if ((((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 != 0.0)) && (locals.var_guard421 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign21420_e16416;
        locals.var_t2_dn0 = assign21420_e16416_d_n0;
        locals.var_t2_dn2 = assign21420_e16416_d_n2;
        locals.var_t2_dn4 = assign21420_e16416_d_n4;
        locals.var_t2_dn5 = assign21420_e16416_d_n5;
        locals.var_t2_dn6 = assign21420_e16416_d_n6;
        locals.var_t2_dn7 = assign21420_e16416_d_n7;
        locals.var_t2_dn8 = assign21420_e16416_d_n8;
        locals.var_t2_dn9 = assign21420_e16416_d_n9;
        locals.var_t2_dn10 = assign21420_e16416_d_n10;
        locals.var_t2_dn11 = assign21420_e16416_d_n11;
        locals.var_t2_dn14 = assign21420_e16416_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign21430_e16428, assign21430_e16428_d_n0, assign21430_e16428_d_n2, assign21430_e16428_d_n4, assign21430_e16428_d_n5, assign21430_e16428_d_n6, assign21430_e16428_d_n7, assign21430_e16428_d_n8, assign21430_e16428_d_n9, assign21430_e16428_d_n10, assign21430_e16428_d_n11, assign21430_e16428_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21430_e16425: f64 = (10.0 * 2.220446049250313e-16);
        let assign21430_e16426: f64 = (locals.var_t1 + assign21430_e16425);
        (assign21430_e16426, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21430_e16428;
        locals.var_t1_dn0 = assign21430_e16428_d_n0;
        locals.var_t1_dn2 = assign21430_e16428_d_n2;
        locals.var_t1_dn4 = assign21430_e16428_d_n4;
        locals.var_t1_dn5 = assign21430_e16428_d_n5;
        locals.var_t1_dn6 = assign21430_e16428_d_n6;
        locals.var_t1_dn7 = assign21430_e16428_d_n7;
        locals.var_t1_dn8 = assign21430_e16428_d_n8;
        locals.var_t1_dn9 = assign21430_e16428_d_n9;
        locals.var_t1_dn10 = assign21430_e16428_d_n10;
        locals.var_t1_dn11 = assign21430_e16428_d_n11;
        locals.var_t1_dn14 = assign21430_e16428_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign21440_e16442, assign21440_e16442_d_n0, assign21440_e16442_d_n2, assign21440_e16442_d_n4, assign21440_e16442_d_n5, assign21440_e16442_d_n6, assign21440_e16442_d_n7, assign21440_e16442_d_n8, assign21440_e16442_d_n9, assign21440_e16442_d_n10, assign21440_e16442_d_n11, assign21440_e16442_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21440_e16438: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign21440_e16439: f64 = (locals.var_uc_nover * assign21440_e16438);
        let assign21440_e16440: f64 = (locals.var_mks_nsubsub / assign21440_e16439);
        (assign21440_e16440, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21440_e16442;
        locals.var_t0_dn0 = assign21440_e16442_d_n0;
        locals.var_t0_dn2 = assign21440_e16442_d_n2;
        locals.var_t0_dn4 = assign21440_e16442_d_n4;
        locals.var_t0_dn5 = assign21440_e16442_d_n5;
        locals.var_t0_dn6 = assign21440_e16442_d_n6;
        locals.var_t0_dn7 = assign21440_e16442_d_n7;
        locals.var_t0_dn8 = assign21440_e16442_d_n8;
        locals.var_t0_dn9 = assign21440_e16442_d_n9;
        locals.var_t0_dn10 = assign21440_e16442_d_n10;
        locals.var_t0_dn11 = assign21440_e16442_d_n11;
        locals.var_t0_dn14 = assign21440_e16442_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21450_e16456, assign21450_e16456_d_n0, assign21450_e16456_d_n2, assign21450_e16456_d_n4, assign21450_e16456_d_n5, assign21450_e16456_d_n6, assign21450_e16456_d_n7, assign21450_e16456_d_n8, assign21450_e16456_d_n9, assign21450_e16456_d_n10, assign21450_e16456_d_n11, assign21450_e16456_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21450_e16450: f64 = (2.0 * 1.034943e-10);
        let assign21450_e16452: f64 = (assign21450_e16450 / 1.6021918e-19);
        let assign21450_e16454: f64 = (assign21450_e16452 * locals.var_t0);
        (assign21450_e16454, (assign21450_e16452 * locals.var_t0_dn0), (assign21450_e16452 * locals.var_t0_dn2), (assign21450_e16452 * locals.var_t0_dn4), (assign21450_e16452 * locals.var_t0_dn5), (assign21450_e16452 * locals.var_t0_dn6), (assign21450_e16452 * locals.var_t0_dn7), (assign21450_e16452 * locals.var_t0_dn8), (assign21450_e16452 * locals.var_t0_dn9), (assign21450_e16452 * locals.var_t0_dn10), (assign21450_e16452 * locals.var_t0_dn11), (assign21450_e16452 * locals.var_t0_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21450_e16456;
        locals.var_t4_dn0 = assign21450_e16456_d_n0;
        locals.var_t4_dn2 = assign21450_e16456_d_n2;
        locals.var_t4_dn4 = assign21450_e16456_d_n4;
        locals.var_t4_dn5 = assign21450_e16456_d_n5;
        locals.var_t4_dn6 = assign21450_e16456_d_n6;
        locals.var_t4_dn7 = assign21450_e16456_d_n7;
        locals.var_t4_dn8 = assign21450_e16456_d_n8;
        locals.var_t4_dn9 = assign21450_e16456_d_n9;
        locals.var_t4_dn10 = assign21450_e16456_d_n10;
        locals.var_t4_dn11 = assign21450_e16456_d_n11;
        locals.var_t4_dn14 = assign21450_e16456_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign21460_e16469, assign21460_e16469_d_n0, assign21460_e16469_d_n2, assign21460_e16469_d_n4, assign21460_e16469_d_n5, assign21460_e16469_d_n6, assign21460_e16469_d_n7, assign21460_e16469_d_n8, assign21460_e16469_d_n9, assign21460_e16469_d_n10, assign21460_e16469_d_n11, assign21460_e16469_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21460_e16464: f64 = (locals.var_t4 * locals.var_t1);
        let assign21460_e16465: f64 = (assign21460_e16464).sqrt();
        let assign21460_e16467: f64 = (assign21460_e16465 + 1e-25);
        (assign21460_e16467, (((locals.var_t4_dn0 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn0)) / (2.0 * assign21460_e16465)), (((locals.var_t4_dn2 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn2)) / (2.0 * assign21460_e16465)), (((locals.var_t4_dn4 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn4)) / (2.0 * assign21460_e16465)), (((locals.var_t4_dn5 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn5)) / (2.0 * assign21460_e16465)), (((locals.var_t4_dn6 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn6)) / (2.0 * assign21460_e16465)), (((locals.var_t4_dn7 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn7)) / (2.0 * assign21460_e16465)), (((locals.var_t4_dn8 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn8)) / (2.0 * assign21460_e16465)), (((locals.var_t4_dn9 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn9)) / (2.0 * assign21460_e16465)), (((locals.var_t4_dn10 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn10)) / (2.0 * assign21460_e16465)), (((locals.var_t4_dn11 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn11)) / (2.0 * assign21460_e16465)), (((locals.var_t4_dn14 * locals.var_t1) + (locals.var_t4 * locals.var_t1_dn14)) / (2.0 * assign21460_e16465)),)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    }
};
        locals.var_wdep = assign21460_e16469;
        locals.var_wdep_dn0 = assign21460_e16469_d_n0;
        locals.var_wdep_dn2 = assign21460_e16469_d_n2;
        locals.var_wdep_dn4 = assign21460_e16469_d_n4;
        locals.var_wdep_dn5 = assign21460_e16469_d_n5;
        locals.var_wdep_dn6 = assign21460_e16469_d_n6;
        locals.var_wdep_dn7 = assign21460_e16469_d_n7;
        locals.var_wdep_dn8 = assign21460_e16469_d_n8;
        locals.var_wdep_dn9 = assign21460_e16469_d_n9;
        locals.var_wdep_dn10 = assign21460_e16469_d_n10;
        locals.var_wdep_dn11 = assign21460_e16469_d_n11;
        locals.var_wdep_dn14 = assign21460_e16469_d_n14;
        locals.var_wdep_rv = 0.0;

        let (assign21470_e16483, assign21470_e16483_d_n0, assign21470_e16483_d_n2, assign21470_e16483_d_n4, assign21470_e16483_d_n5, assign21470_e16483_d_n6, assign21470_e16483_d_n7, assign21470_e16483_d_n8, assign21470_e16483_d_n9, assign21470_e16483_d_n10, assign21470_e16483_d_n11, assign21470_e16483_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21470_e16477: f64 = (p.p334 - locals.var_wdep);
        let assign21470_e16480: f64 = (0.1 * p.p334);
        let assign21470_e16481: f64 = (assign21470_e16477 - assign21470_e16480);
        (assign21470_e16481, (-locals.var_wdep_dn0), (-locals.var_wdep_dn2), (-locals.var_wdep_dn4), (-locals.var_wdep_dn5), (-locals.var_wdep_dn6), (-locals.var_wdep_dn7), (-locals.var_wdep_dn8), (-locals.var_wdep_dn9), (-locals.var_wdep_dn10), (-locals.var_wdep_dn11), (-locals.var_wdep_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign21470_e16483;
        locals.var_tmf1_dn0 = assign21470_e16483_d_n0;
        locals.var_tmf1_dn2 = assign21470_e16483_d_n2;
        locals.var_tmf1_dn4 = assign21470_e16483_d_n4;
        locals.var_tmf1_dn5 = assign21470_e16483_d_n5;
        locals.var_tmf1_dn6 = assign21470_e16483_d_n6;
        locals.var_tmf1_dn7 = assign21470_e16483_d_n7;
        locals.var_tmf1_dn8 = assign21470_e16483_d_n8;
        locals.var_tmf1_dn9 = assign21470_e16483_d_n9;
        locals.var_tmf1_dn10 = assign21470_e16483_d_n10;
        locals.var_tmf1_dn11 = assign21470_e16483_d_n11;
        locals.var_tmf1_dn14 = assign21470_e16483_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign21480_e16497, assign21480_e16497_d_n0, assign21480_e16497_d_n2, assign21480_e16497_d_n4, assign21480_e16497_d_n5, assign21480_e16497_d_n6, assign21480_e16497_d_n7, assign21480_e16497_d_n8, assign21480_e16497_d_n9, assign21480_e16497_d_n10, assign21480_e16497_d_n11, assign21480_e16497_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21480_e16491: f64 = (4.0 * p.p334);
        let assign21480_e16494: f64 = (0.1 * p.p334);
        let assign21480_e16495: f64 = (assign21480_e16491 * assign21480_e16494);
        (assign21480_e16495, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21480_e16497;
        locals.var_tmf2_dn0 = assign21480_e16497_d_n0;
        locals.var_tmf2_dn2 = assign21480_e16497_d_n2;
        locals.var_tmf2_dn4 = assign21480_e16497_d_n4;
        locals.var_tmf2_dn5 = assign21480_e16497_d_n5;
        locals.var_tmf2_dn6 = assign21480_e16497_d_n6;
        locals.var_tmf2_dn7 = assign21480_e16497_d_n7;
        locals.var_tmf2_dn8 = assign21480_e16497_d_n8;
        locals.var_tmf2_dn9 = assign21480_e16497_d_n9;
        locals.var_tmf2_dn10 = assign21480_e16497_d_n10;
        locals.var_tmf2_dn11 = assign21480_e16497_d_n11;
        locals.var_tmf2_dn14 = assign21480_e16497_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21490_e16511, assign21490_e16511_d_n0, assign21490_e16511_d_n2, assign21490_e16511_d_n4, assign21490_e16511_d_n5, assign21490_e16511_d_n6, assign21490_e16511_d_n7, assign21490_e16511_d_n8, assign21490_e16511_d_n9, assign21490_e16511_d_n10, assign21490_e16511_d_n11, assign21490_e16511_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let (assign21490_e16509, assign21490_e16509_d_n0, assign21490_e16509_d_n2, assign21490_e16509_d_n4, assign21490_e16509_d_n5, assign21490_e16509_d_n6, assign21490_e16509_d_n7, assign21490_e16509_d_n8, assign21490_e16509_d_n9, assign21490_e16509_d_n10, assign21490_e16509_d_n11, assign21490_e16509_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign21490_e16508: f64 = (-locals.var_tmf2);
                (assign21490_e16508, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign21490_e16509, assign21490_e16509_d_n0, assign21490_e16509_d_n2, assign21490_e16509_d_n4, assign21490_e16509_d_n5, assign21490_e16509_d_n6, assign21490_e16509_d_n7, assign21490_e16509_d_n8, assign21490_e16509_d_n9, assign21490_e16509_d_n10, assign21490_e16509_d_n11, assign21490_e16509_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21490_e16511;
        locals.var_tmf2_dn0 = assign21490_e16511_d_n0;
        locals.var_tmf2_dn2 = assign21490_e16511_d_n2;
        locals.var_tmf2_dn4 = assign21490_e16511_d_n4;
        locals.var_tmf2_dn5 = assign21490_e16511_d_n5;
        locals.var_tmf2_dn6 = assign21490_e16511_d_n6;
        locals.var_tmf2_dn7 = assign21490_e16511_d_n7;
        locals.var_tmf2_dn8 = assign21490_e16511_d_n8;
        locals.var_tmf2_dn9 = assign21490_e16511_d_n9;
        locals.var_tmf2_dn10 = assign21490_e16511_d_n10;
        locals.var_tmf2_dn11 = assign21490_e16511_d_n11;
        locals.var_tmf2_dn14 = assign21490_e16511_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21500_e16524, assign21500_e16524_d_n0, assign21500_e16524_d_n2, assign21500_e16524_d_n4, assign21500_e16524_d_n5, assign21500_e16524_d_n6, assign21500_e16524_d_n7, assign21500_e16524_d_n8, assign21500_e16524_d_n9, assign21500_e16524_d_n10, assign21500_e16524_d_n11, assign21500_e16524_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21500_e16519: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign21500_e16521: f64 = (assign21500_e16519 + locals.var_tmf2);
        let assign21500_e16522: f64 = (assign21500_e16521).sqrt();
        (assign21500_e16522, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign21500_e16522)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign21500_e16522)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign21500_e16522)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign21500_e16522)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign21500_e16522)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign21500_e16522)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign21500_e16522)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign21500_e16522)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign21500_e16522)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign21500_e16522)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign21500_e16522)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21500_e16524;
        locals.var_tmf2_dn0 = assign21500_e16524_d_n0;
        locals.var_tmf2_dn2 = assign21500_e16524_d_n2;
        locals.var_tmf2_dn4 = assign21500_e16524_d_n4;
        locals.var_tmf2_dn5 = assign21500_e16524_d_n5;
        locals.var_tmf2_dn6 = assign21500_e16524_d_n6;
        locals.var_tmf2_dn7 = assign21500_e16524_d_n7;
        locals.var_tmf2_dn8 = assign21500_e16524_d_n8;
        locals.var_tmf2_dn9 = assign21500_e16524_d_n9;
        locals.var_tmf2_dn10 = assign21500_e16524_d_n10;
        locals.var_tmf2_dn11 = assign21500_e16524_d_n11;
        locals.var_tmf2_dn14 = assign21500_e16524_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21510_e16538, assign21510_e16538_d_n0, assign21510_e16538_d_n2, assign21510_e16538_d_n4, assign21510_e16538_d_n5, assign21510_e16538_d_n6, assign21510_e16538_d_n7, assign21510_e16538_d_n8, assign21510_e16538_d_n9, assign21510_e16538_d_n10, assign21510_e16538_d_n11, assign21510_e16538_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21510_e16534: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign21510_e16535: f64 = (1.0 + assign21510_e16534);
        let assign21510_e16536: f64 = (0.5 * assign21510_e16535);
        (assign21510_e16536, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21510_e16538;
        locals.var_t0_dn0 = assign21510_e16538_d_n0;
        locals.var_t0_dn2 = assign21510_e16538_d_n2;
        locals.var_t0_dn4 = assign21510_e16538_d_n4;
        locals.var_t0_dn5 = assign21510_e16538_d_n5;
        locals.var_t0_dn6 = assign21510_e16538_d_n6;
        locals.var_t0_dn7 = assign21510_e16538_d_n7;
        locals.var_t0_dn8 = assign21510_e16538_d_n8;
        locals.var_t0_dn9 = assign21510_e16538_d_n9;
        locals.var_t0_dn10 = assign21510_e16538_d_n10;
        locals.var_t0_dn11 = assign21510_e16538_d_n11;
        locals.var_t0_dn14 = assign21510_e16538_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21520_e16552, assign21520_e16552_d_n0, assign21520_e16552_d_n2, assign21520_e16552_d_n4, assign21520_e16552_d_n5, assign21520_e16552_d_n6, assign21520_e16552_d_n7, assign21520_e16552_d_n8, assign21520_e16552_d_n9, assign21520_e16552_d_n10, assign21520_e16552_d_n11, assign21520_e16552_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21520_e16548: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign21520_e16549: f64 = (0.5 * assign21520_e16548);
        let assign21520_e16550: f64 = (p.p334 - assign21520_e16549);
        (assign21520_e16550, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    }
};
        locals.var_wdep = assign21520_e16552;
        locals.var_wdep_dn0 = assign21520_e16552_d_n0;
        locals.var_wdep_dn2 = assign21520_e16552_d_n2;
        locals.var_wdep_dn4 = assign21520_e16552_d_n4;
        locals.var_wdep_dn5 = assign21520_e16552_d_n5;
        locals.var_wdep_dn6 = assign21520_e16552_d_n6;
        locals.var_wdep_dn7 = assign21520_e16552_d_n7;
        locals.var_wdep_dn8 = assign21520_e16552_d_n8;
        locals.var_wdep_dn9 = assign21520_e16552_d_n9;
        locals.var_wdep_dn10 = assign21520_e16552_d_n10;
        locals.var_wdep_dn11 = assign21520_e16552_d_n11;
        locals.var_wdep_dn14 = assign21520_e16552_d_n14;
        locals.var_wdep_rv = 0.0;

        let (assign21530_e16564, assign21530_e16564_d_n0, assign21530_e16564_d_n2, assign21530_e16564_d_n4, assign21530_e16564_d_n5, assign21530_e16564_d_n6, assign21530_e16564_d_n7, assign21530_e16564_d_n8, assign21530_e16564_d_n9, assign21530_e16564_d_n10, assign21530_e16564_d_n11, assign21530_e16564_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21530_e16561: f64 = (p.p334 - locals.var_wdep);
        let assign21530_e16562: f64 = (locals.var_ldrift0 / assign21530_e16561);
        (assign21530_e16562, (-((locals.var_ldrift0 * (-locals.var_wdep_dn0)) / (assign21530_e16561 * assign21530_e16561))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn2)) / (assign21530_e16561 * assign21530_e16561))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn4)) / (assign21530_e16561 * assign21530_e16561))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn5)) / (assign21530_e16561 * assign21530_e16561))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn6)) / (assign21530_e16561 * assign21530_e16561))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn7)) / (assign21530_e16561 * assign21530_e16561))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn8)) / (assign21530_e16561 * assign21530_e16561))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn9)) / (assign21530_e16561 * assign21530_e16561))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn10)) / (assign21530_e16561 * assign21530_e16561))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn11)) / (assign21530_e16561 * assign21530_e16561))), (-((locals.var_ldrift0 * (-locals.var_wdep_dn14)) / (assign21530_e16561 * assign21530_e16561))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign21530_e16564;
        locals.var_t6_dn0 = assign21530_e16564_d_n0;
        locals.var_t6_dn2 = assign21530_e16564_d_n2;
        locals.var_t6_dn4 = assign21530_e16564_d_n4;
        locals.var_t6_dn5 = assign21530_e16564_d_n5;
        locals.var_t6_dn6 = assign21530_e16564_d_n6;
        locals.var_t6_dn7 = assign21530_e16564_d_n7;
        locals.var_t6_dn8 = assign21530_e16564_d_n8;
        locals.var_t6_dn9 = assign21530_e16564_d_n9;
        locals.var_t6_dn10 = assign21530_e16564_d_n10;
        locals.var_t6_dn11 = assign21530_e16564_d_n11;
        locals.var_t6_dn14 = assign21530_e16564_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign21540_e16574, assign21540_e16574_d_n0, assign21540_e16574_d_n2, assign21540_e16574_d_n4, assign21540_e16574_d_n5, assign21540_e16574_d_n6, assign21540_e16574_d_n7, assign21540_e16574_d_n8, assign21540_e16574_d_n9, assign21540_e16574_d_n10, assign21540_e16574_d_n11, assign21540_e16574_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21540_e16572: f64 = (locals.var_rdrift * locals.var_t6);
        (assign21540_e16572, ((locals.var_rdrift_dn0 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn0)), ((locals.var_rdrift_dn2 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn2)), ((locals.var_rdrift_dn4 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn4)), ((locals.var_rdrift_dn5 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn5)), ((locals.var_rdrift_dn6 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn6)), ((locals.var_rdrift_dn7 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn7)), ((locals.var_rdrift_dn8 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn8)), ((locals.var_rdrift_dn9 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn9)), ((locals.var_rdrift_dn10 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn10)), ((locals.var_rdrift_dn11 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn11)), ((locals.var_rdrift_dn14 * locals.var_t6) + (locals.var_rdrift * locals.var_t6_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21540_e16574;
        locals.var_t0_dn0 = assign21540_e16574_d_n0;
        locals.var_t0_dn2 = assign21540_e16574_d_n2;
        locals.var_t0_dn4 = assign21540_e16574_d_n4;
        locals.var_t0_dn5 = assign21540_e16574_d_n5;
        locals.var_t0_dn6 = assign21540_e16574_d_n6;
        locals.var_t0_dn7 = assign21540_e16574_d_n7;
        locals.var_t0_dn8 = assign21540_e16574_d_n8;
        locals.var_t0_dn9 = assign21540_e16574_d_n9;
        locals.var_t0_dn10 = assign21540_e16574_d_n10;
        locals.var_t0_dn11 = assign21540_e16574_d_n11;
        locals.var_t0_dn14 = assign21540_e16574_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21550_e16584, assign21550_e16584_d_n0, assign21550_e16584_d_n2, assign21550_e16584_d_n4, assign21550_e16584_d_n5, assign21550_e16584_d_n6, assign21550_e16584_d_n7, assign21550_e16584_d_n8, assign21550_e16584_d_n9, assign21550_e16584_d_n10, assign21550_e16584_d_n11, assign21550_e16584_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21550_e16582: f64 = (locals.var_rsdrift * locals.var_t6);
        (assign21550_e16582, ((locals.var_rsdrift_dn0 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn0)), ((locals.var_rsdrift_dn2 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn2)), ((locals.var_rsdrift_dn4 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn4)), ((locals.var_rsdrift_dn5 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn5)), ((locals.var_rsdrift_dn6 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn6)), ((locals.var_rsdrift_dn7 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn7)), ((locals.var_rsdrift_dn8 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn8)), ((locals.var_rsdrift_dn9 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn9)), ((locals.var_rsdrift_dn10 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn10)), ((locals.var_rsdrift_dn11 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn11)), ((locals.var_rsdrift_dn14 * locals.var_t6) + (locals.var_rsdrift * locals.var_t6_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21550_e16584;
        locals.var_t1_dn0 = assign21550_e16584_d_n0;
        locals.var_t1_dn2 = assign21550_e16584_d_n2;
        locals.var_t1_dn4 = assign21550_e16584_d_n4;
        locals.var_t1_dn5 = assign21550_e16584_d_n5;
        locals.var_t1_dn6 = assign21550_e16584_d_n6;
        locals.var_t1_dn7 = assign21550_e16584_d_n7;
        locals.var_t1_dn8 = assign21550_e16584_d_n8;
        locals.var_t1_dn9 = assign21550_e16584_d_n9;
        locals.var_t1_dn10 = assign21550_e16584_d_n10;
        locals.var_t1_dn11 = assign21550_e16584_d_n11;
        locals.var_t1_dn14 = assign21550_e16584_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign21560_e16598, assign21560_e16598_d_n0, assign21560_e16598_d_n2, assign21560_e16598_d_n4, assign21560_e16598_d_n5, assign21560_e16598_d_n6, assign21560_e16598_d_n7, assign21560_e16598_d_n8, assign21560_e16598_d_n9, assign21560_e16598_d_n10, assign21560_e16598_d_n11, assign21560_e16598_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21560_e16592: f64 = (locals.var_t0 * locals.var_vdsemodenml);
        let assign21560_e16595: f64 = (locals.var_rdrift * locals.var_vdsemodervs);
        let assign21560_e16596: f64 = (assign21560_e16592 + assign21560_e16595);
        (assign21560_e16596, ((locals.var_t0_dn0 * locals.var_vdsemodenml) + (locals.var_rdrift_dn0 * locals.var_vdsemodervs)), ((locals.var_t0_dn2 * locals.var_vdsemodenml) + (locals.var_rdrift_dn2 * locals.var_vdsemodervs)), ((locals.var_t0_dn4 * locals.var_vdsemodenml) + (locals.var_rdrift_dn4 * locals.var_vdsemodervs)), ((locals.var_t0_dn5 * locals.var_vdsemodenml) + (locals.var_rdrift_dn5 * locals.var_vdsemodervs)), ((locals.var_t0_dn6 * locals.var_vdsemodenml) + (locals.var_rdrift_dn6 * locals.var_vdsemodervs)), ((locals.var_t0_dn7 * locals.var_vdsemodenml) + (locals.var_rdrift_dn7 * locals.var_vdsemodervs)), ((locals.var_t0_dn8 * locals.var_vdsemodenml) + (locals.var_rdrift_dn8 * locals.var_vdsemodervs)), ((locals.var_t0_dn9 * locals.var_vdsemodenml) + (locals.var_rdrift_dn9 * locals.var_vdsemodervs)), ((locals.var_t0_dn10 * locals.var_vdsemodenml) + (locals.var_rdrift_dn10 * locals.var_vdsemodervs)), ((locals.var_t0_dn11 * locals.var_vdsemodenml) + (locals.var_rdrift_dn11 * locals.var_vdsemodervs)), ((locals.var_t0_dn14 * locals.var_vdsemodenml) + (locals.var_rdrift_dn14 * locals.var_vdsemodervs)),)
    } else {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn11, locals.var_rdrift_dn14,)
    }
};
        locals.var_rdrift = assign21560_e16598;
        locals.var_rdrift_dn0 = assign21560_e16598_d_n0;
        locals.var_rdrift_dn2 = assign21560_e16598_d_n2;
        locals.var_rdrift_dn4 = assign21560_e16598_d_n4;
        locals.var_rdrift_dn5 = assign21560_e16598_d_n5;
        locals.var_rdrift_dn6 = assign21560_e16598_d_n6;
        locals.var_rdrift_dn7 = assign21560_e16598_d_n7;
        locals.var_rdrift_dn8 = assign21560_e16598_d_n8;
        locals.var_rdrift_dn9 = assign21560_e16598_d_n9;
        locals.var_rdrift_dn10 = assign21560_e16598_d_n10;
        locals.var_rdrift_dn11 = assign21560_e16598_d_n11;
        locals.var_rdrift_dn14 = assign21560_e16598_d_n14;
        locals.var_rdrift_rv = 0.0;

        let (assign21570_e16612, assign21570_e16612_d_n0, assign21570_e16612_d_n2, assign21570_e16612_d_n4, assign21570_e16612_d_n5, assign21570_e16612_d_n6, assign21570_e16612_d_n7, assign21570_e16612_d_n8, assign21570_e16612_d_n9, assign21570_e16612_d_n10, assign21570_e16612_d_n11, assign21570_e16612_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 != 0.0)) {
        let assign21570_e16606: f64 = (locals.var_t1 * locals.var_vdsemodervs);
        let assign21570_e16609: f64 = (locals.var_rsdrift * locals.var_vdsemodenml);
        let assign21570_e16610: f64 = (assign21570_e16606 + assign21570_e16609);
        (assign21570_e16610, ((locals.var_t1_dn0 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn0 * locals.var_vdsemodenml)), ((locals.var_t1_dn2 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn2 * locals.var_vdsemodenml)), ((locals.var_t1_dn4 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn4 * locals.var_vdsemodenml)), ((locals.var_t1_dn5 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn5 * locals.var_vdsemodenml)), ((locals.var_t1_dn6 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn6 * locals.var_vdsemodenml)), ((locals.var_t1_dn7 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn7 * locals.var_vdsemodenml)), ((locals.var_t1_dn8 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn8 * locals.var_vdsemodenml)), ((locals.var_t1_dn9 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn9 * locals.var_vdsemodenml)), ((locals.var_t1_dn10 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn10 * locals.var_vdsemodenml)), ((locals.var_t1_dn11 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn11 * locals.var_vdsemodenml)), ((locals.var_t1_dn14 * locals.var_vdsemodervs) + (locals.var_rsdrift_dn14 * locals.var_vdsemodenml)),)
    } else {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn11, locals.var_rsdrift_dn14,)
    }
};
        locals.var_rsdrift = assign21570_e16612;
        locals.var_rsdrift_dn0 = assign21570_e16612_d_n0;
        locals.var_rsdrift_dn2 = assign21570_e16612_d_n2;
        locals.var_rsdrift_dn4 = assign21570_e16612_d_n4;
        locals.var_rsdrift_dn5 = assign21570_e16612_d_n5;
        locals.var_rsdrift_dn6 = assign21570_e16612_d_n6;
        locals.var_rsdrift_dn7 = assign21570_e16612_d_n7;
        locals.var_rsdrift_dn8 = assign21570_e16612_d_n8;
        locals.var_rsdrift_dn9 = assign21570_e16612_d_n9;
        locals.var_rsdrift_dn10 = assign21570_e16612_d_n10;
        locals.var_rsdrift_dn11 = assign21570_e16612_d_n11;
        locals.var_rsdrift_dn14 = assign21570_e16612_d_n14;
        locals.var_rsdrift_rv = 0.0;

        let (assign21580_e16621, assign21580_e16621_d_n0, assign21580_e16621_d_n2, assign21580_e16621_d_n4, assign21580_e16621_d_n5, assign21580_e16621_d_n6, assign21580_e16621_d_n7, assign21580_e16621_d_n8, assign21580_e16621_d_n9, assign21580_e16621_d_n10, assign21580_e16621_d_n11, assign21580_e16621_d_n14,) = {
    if (((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) && (locals.var_guard420 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_wdep, locals.var_wdep_dn0, locals.var_wdep_dn2, locals.var_wdep_dn4, locals.var_wdep_dn5, locals.var_wdep_dn6, locals.var_wdep_dn7, locals.var_wdep_dn8, locals.var_wdep_dn9, locals.var_wdep_dn10, locals.var_wdep_dn11, locals.var_wdep_dn14,)
    }
};
        locals.var_wdep = assign21580_e16621;
        locals.var_wdep_dn0 = assign21580_e16621_d_n0;
        locals.var_wdep_dn2 = assign21580_e16621_d_n2;
        locals.var_wdep_dn4 = assign21580_e16621_d_n4;
        locals.var_wdep_dn5 = assign21580_e16621_d_n5;
        locals.var_wdep_dn6 = assign21580_e16621_d_n6;
        locals.var_wdep_dn7 = assign21580_e16621_d_n7;
        locals.var_wdep_dn8 = assign21580_e16621_d_n8;
        locals.var_wdep_dn9 = assign21580_e16621_d_n9;
        locals.var_wdep_dn10 = assign21580_e16621_d_n10;
        locals.var_wdep_dn11 = assign21580_e16621_d_n11;
        locals.var_wdep_dn14 = assign21580_e16621_d_n14;
        locals.var_wdep_rv = 0.0;

        let (assign21590_e16627, assign21590_e16627_d_n0, assign21590_e16627_d_n2, assign21590_e16627_d_n4, assign21590_e16627_d_n5, assign21590_e16627_d_n6, assign21590_e16627_d_n7, assign21590_e16627_d_n8, assign21590_e16627_d_n9, assign21590_e16627_d_n10, assign21590_e16627_d_n11, assign21590_e16627_d_n14,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) {
        (locals.var_rdrift, locals.var_rdrift_dn0, locals.var_rdrift_dn2, locals.var_rdrift_dn4, locals.var_rdrift_dn5, locals.var_rdrift_dn6, locals.var_rdrift_dn7, locals.var_rdrift_dn8, locals.var_rdrift_dn9, locals.var_rdrift_dn10, locals.var_rdrift_dn11, locals.var_rdrift_dn14,)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign21590_e16627;
        locals.var_rdd_dn0 = assign21590_e16627_d_n0;
        locals.var_rdd_dn2 = assign21590_e16627_d_n2;
        locals.var_rdd_dn4 = assign21590_e16627_d_n4;
        locals.var_rdd_dn5 = assign21590_e16627_d_n5;
        locals.var_rdd_dn6 = assign21590_e16627_d_n6;
        locals.var_rdd_dn7 = assign21590_e16627_d_n7;
        locals.var_rdd_dn8 = assign21590_e16627_d_n8;
        locals.var_rdd_dn9 = assign21590_e16627_d_n9;
        locals.var_rdd_dn10 = assign21590_e16627_d_n10;
        locals.var_rdd_dn11 = assign21590_e16627_d_n11;
        locals.var_rdd_dn14 = assign21590_e16627_d_n14;
        locals.var_rdd_rv = 0.0;

        let (assign21600_e16633, assign21600_e16633_d_n0, assign21600_e16633_d_n2, assign21600_e16633_d_n4, assign21600_e16633_d_n5, assign21600_e16633_d_n6, assign21600_e16633_d_n7, assign21600_e16633_d_n8, assign21600_e16633_d_n9, assign21600_e16633_d_n10, assign21600_e16633_d_n11, assign21600_e16633_d_n14,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard413 != 0.0)) {
        (locals.var_rsdrift, locals.var_rsdrift_dn0, locals.var_rsdrift_dn2, locals.var_rsdrift_dn4, locals.var_rsdrift_dn5, locals.var_rsdrift_dn6, locals.var_rsdrift_dn7, locals.var_rsdrift_dn8, locals.var_rsdrift_dn9, locals.var_rsdrift_dn10, locals.var_rsdrift_dn11, locals.var_rsdrift_dn14,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign21600_e16633;
        locals.var_rsd_dn0 = assign21600_e16633_d_n0;
        locals.var_rsd_dn2 = assign21600_e16633_d_n2;
        locals.var_rsd_dn4 = assign21600_e16633_d_n4;
        locals.var_rsd_dn5 = assign21600_e16633_d_n5;
        locals.var_rsd_dn6 = assign21600_e16633_d_n6;
        locals.var_rsd_dn7 = assign21600_e16633_d_n7;
        locals.var_rsd_dn8 = assign21600_e16633_d_n8;
        locals.var_rsd_dn9 = assign21600_e16633_d_n9;
        locals.var_rsd_dn10 = assign21600_e16633_d_n10;
        locals.var_rsd_dn11 = assign21600_e16633_d_n11;
        locals.var_rsd_dn14 = assign21600_e16633_d_n14;
        locals.var_rsd_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_57(
        locals: &mut StampLocals,
    ) {
        let (assign21610_e16646, assign21610_e16646_d_n0, assign21610_e16646_d_n2, assign21610_e16646_d_n4, assign21610_e16646_d_n5, assign21610_e16646_d_n6, assign21610_e16646_d_n7, assign21610_e16646_d_n8, assign21610_e16646_d_n9, assign21610_e16646_d_n10, assign21610_e16646_d_n11, assign21610_e16646_d_n14,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard413 == 0.0)) {
        let assign21610_e16640: f64 = (locals.var_vdsemodenml * locals.var_rde);
        let assign21610_e16643: f64 = (locals.var_vdsemodervs * locals.var_rse);
        let assign21610_e16644: f64 = (assign21610_e16640 + assign21610_e16643);
        (assign21610_e16644, ((locals.var_vdsemodenml * locals.var_rde_dn0) + (locals.var_vdsemodervs * locals.var_rse_dn0)), ((locals.var_vdsemodenml * locals.var_rde_dn2) + (locals.var_vdsemodervs * locals.var_rse_dn2)), ((locals.var_vdsemodenml * locals.var_rde_dn4) + (locals.var_vdsemodervs * locals.var_rse_dn4)), ((locals.var_vdsemodenml * locals.var_rde_dn5) + (locals.var_vdsemodervs * locals.var_rse_dn5)), ((locals.var_vdsemodenml * locals.var_rde_dn6) + (locals.var_vdsemodervs * locals.var_rse_dn6)), ((locals.var_vdsemodenml * locals.var_rde_dn7) + (locals.var_vdsemodervs * locals.var_rse_dn7)), ((locals.var_vdsemodenml * locals.var_rde_dn8) + (locals.var_vdsemodervs * locals.var_rse_dn8)), ((locals.var_vdsemodenml * locals.var_rde_dn9) + (locals.var_vdsemodervs * locals.var_rse_dn9)), ((locals.var_vdsemodenml * locals.var_rde_dn10) + (locals.var_vdsemodervs * locals.var_rse_dn10)), ((locals.var_vdsemodenml * locals.var_rde_dn11) + (locals.var_vdsemodervs * locals.var_rse_dn11)), ((locals.var_vdsemodenml * locals.var_rde_dn14) + (locals.var_vdsemodervs * locals.var_rse_dn14)),)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign21610_e16646;
        locals.var_rdd_dn0 = assign21610_e16646_d_n0;
        locals.var_rdd_dn2 = assign21610_e16646_d_n2;
        locals.var_rdd_dn4 = assign21610_e16646_d_n4;
        locals.var_rdd_dn5 = assign21610_e16646_d_n5;
        locals.var_rdd_dn6 = assign21610_e16646_d_n6;
        locals.var_rdd_dn7 = assign21610_e16646_d_n7;
        locals.var_rdd_dn8 = assign21610_e16646_d_n8;
        locals.var_rdd_dn9 = assign21610_e16646_d_n9;
        locals.var_rdd_dn10 = assign21610_e16646_d_n10;
        locals.var_rdd_dn11 = assign21610_e16646_d_n11;
        locals.var_rdd_dn14 = assign21610_e16646_d_n14;
        locals.var_rdd_rv = 0.0;

        let (assign21620_e16659, assign21620_e16659_d_n0, assign21620_e16659_d_n2, assign21620_e16659_d_n4, assign21620_e16659_d_n5, assign21620_e16659_d_n6, assign21620_e16659_d_n7, assign21620_e16659_d_n8, assign21620_e16659_d_n9, assign21620_e16659_d_n10, assign21620_e16659_d_n11, assign21620_e16659_d_n14,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard413 == 0.0)) {
        let assign21620_e16653: f64 = (locals.var_vdsemodenml * locals.var_rse);
        let assign21620_e16656: f64 = (locals.var_vdsemodervs * locals.var_rde);
        let assign21620_e16657: f64 = (assign21620_e16653 + assign21620_e16656);
        (assign21620_e16657, ((locals.var_vdsemodenml * locals.var_rse_dn0) + (locals.var_vdsemodervs * locals.var_rde_dn0)), ((locals.var_vdsemodenml * locals.var_rse_dn2) + (locals.var_vdsemodervs * locals.var_rde_dn2)), ((locals.var_vdsemodenml * locals.var_rse_dn4) + (locals.var_vdsemodervs * locals.var_rde_dn4)), ((locals.var_vdsemodenml * locals.var_rse_dn5) + (locals.var_vdsemodervs * locals.var_rde_dn5)), ((locals.var_vdsemodenml * locals.var_rse_dn6) + (locals.var_vdsemodervs * locals.var_rde_dn6)), ((locals.var_vdsemodenml * locals.var_rse_dn7) + (locals.var_vdsemodervs * locals.var_rde_dn7)), ((locals.var_vdsemodenml * locals.var_rse_dn8) + (locals.var_vdsemodervs * locals.var_rde_dn8)), ((locals.var_vdsemodenml * locals.var_rse_dn9) + (locals.var_vdsemodervs * locals.var_rde_dn9)), ((locals.var_vdsemodenml * locals.var_rse_dn10) + (locals.var_vdsemodervs * locals.var_rde_dn10)), ((locals.var_vdsemodenml * locals.var_rse_dn11) + (locals.var_vdsemodervs * locals.var_rde_dn11)), ((locals.var_vdsemodenml * locals.var_rse_dn14) + (locals.var_vdsemodervs * locals.var_rde_dn14)),)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign21620_e16659;
        locals.var_rsd_dn0 = assign21620_e16659_d_n0;
        locals.var_rsd_dn2 = assign21620_e16659_d_n2;
        locals.var_rsd_dn4 = assign21620_e16659_d_n4;
        locals.var_rsd_dn5 = assign21620_e16659_d_n5;
        locals.var_rsd_dn6 = assign21620_e16659_d_n6;
        locals.var_rsd_dn7 = assign21620_e16659_d_n7;
        locals.var_rsd_dn8 = assign21620_e16659_d_n8;
        locals.var_rsd_dn9 = assign21620_e16659_d_n9;
        locals.var_rsd_dn10 = assign21620_e16659_d_n10;
        locals.var_rsd_dn11 = assign21620_e16659_d_n11;
        locals.var_rsd_dn14 = assign21620_e16659_d_n14;
        locals.var_rsd_rv = 0.0;

        let (assign21630_e16665, assign21630_e16665_d_n0, assign21630_e16665_d_n2, assign21630_e16665_d_n4, assign21630_e16665_d_n5, assign21630_e16665_d_n6, assign21630_e16665_d_n7, assign21630_e16665_d_n8, assign21630_e16665_d_n9, assign21630_e16665_d_n10, assign21630_e16665_d_n11, assign21630_e16665_d_n14,) = {
    if (locals.var_guard411 != 0.0) {
        let assign21630_e16663: f64 = (locals.var_rdd / locals.var_weffld_nf);
        (assign21630_e16663, (locals.var_rdd_dn0 / locals.var_weffld_nf), (locals.var_rdd_dn2 / locals.var_weffld_nf), (locals.var_rdd_dn4 / locals.var_weffld_nf), (locals.var_rdd_dn5 / locals.var_weffld_nf), (locals.var_rdd_dn6 / locals.var_weffld_nf), (locals.var_rdd_dn7 / locals.var_weffld_nf), (locals.var_rdd_dn8 / locals.var_weffld_nf), (locals.var_rdd_dn9 / locals.var_weffld_nf), (locals.var_rdd_dn10 / locals.var_weffld_nf), (locals.var_rdd_dn11 / locals.var_weffld_nf), (locals.var_rdd_dn14 / locals.var_weffld_nf),)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign21630_e16665;
        locals.var_rdd_dn0 = assign21630_e16665_d_n0;
        locals.var_rdd_dn2 = assign21630_e16665_d_n2;
        locals.var_rdd_dn4 = assign21630_e16665_d_n4;
        locals.var_rdd_dn5 = assign21630_e16665_d_n5;
        locals.var_rdd_dn6 = assign21630_e16665_d_n6;
        locals.var_rdd_dn7 = assign21630_e16665_d_n7;
        locals.var_rdd_dn8 = assign21630_e16665_d_n8;
        locals.var_rdd_dn9 = assign21630_e16665_d_n9;
        locals.var_rdd_dn10 = assign21630_e16665_d_n10;
        locals.var_rdd_dn11 = assign21630_e16665_d_n11;
        locals.var_rdd_dn14 = assign21630_e16665_d_n14;
        locals.var_rdd_rv = 0.0;

        let (assign21640_e16671, assign21640_e16671_d_n0, assign21640_e16671_d_n2, assign21640_e16671_d_n4, assign21640_e16671_d_n5, assign21640_e16671_d_n6, assign21640_e16671_d_n7, assign21640_e16671_d_n8, assign21640_e16671_d_n9, assign21640_e16671_d_n10, assign21640_e16671_d_n11, assign21640_e16671_d_n14,) = {
    if (locals.var_guard411 != 0.0) {
        let assign21640_e16669: f64 = (locals.var_rsd / locals.var_weffld_nf);
        (assign21640_e16669, (locals.var_rsd_dn0 / locals.var_weffld_nf), (locals.var_rsd_dn2 / locals.var_weffld_nf), (locals.var_rsd_dn4 / locals.var_weffld_nf), (locals.var_rsd_dn5 / locals.var_weffld_nf), (locals.var_rsd_dn6 / locals.var_weffld_nf), (locals.var_rsd_dn7 / locals.var_weffld_nf), (locals.var_rsd_dn8 / locals.var_weffld_nf), (locals.var_rsd_dn9 / locals.var_weffld_nf), (locals.var_rsd_dn10 / locals.var_weffld_nf), (locals.var_rsd_dn11 / locals.var_weffld_nf), (locals.var_rsd_dn14 / locals.var_weffld_nf),)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign21640_e16671;
        locals.var_rsd_dn0 = assign21640_e16671_d_n0;
        locals.var_rsd_dn2 = assign21640_e16671_d_n2;
        locals.var_rsd_dn4 = assign21640_e16671_d_n4;
        locals.var_rsd_dn5 = assign21640_e16671_d_n5;
        locals.var_rsd_dn6 = assign21640_e16671_d_n6;
        locals.var_rsd_dn7 = assign21640_e16671_d_n7;
        locals.var_rsd_dn8 = assign21640_e16671_d_n8;
        locals.var_rsd_dn9 = assign21640_e16671_d_n9;
        locals.var_rsd_dn10 = assign21640_e16671_d_n10;
        locals.var_rsd_dn11 = assign21640_e16671_d_n11;
        locals.var_rsd_dn14 = assign21640_e16671_d_n14;
        locals.var_rsd_rv = 0.0;

        let (assign21650_e16683, assign21650_e16683_d_n0, assign21650_e16683_d_n2, assign21650_e16683_d_n4, assign21650_e16683_d_n5, assign21650_e16683_d_n6, assign21650_e16683_d_n7, assign21650_e16683_d_n8, assign21650_e16683_d_n9, assign21650_e16683_d_n10, assign21650_e16683_d_n11, assign21650_e16683_d_n14,) = {
    if (locals.var_guard411 != 0.0) {
        let assign21650_e16676: f64 = (locals.var_vdsemodenml * locals.var_rd0);
        let assign21650_e16677: f64 = (locals.var_rdd + assign21650_e16676);
        let assign21650_e16680: f64 = (locals.var_vdsemodervs * locals.var_rs0);
        let assign21650_e16681: f64 = (assign21650_e16677 + assign21650_e16680);
        (assign21650_e16681, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    } else {
        (locals.var_rdd, locals.var_rdd_dn0, locals.var_rdd_dn2, locals.var_rdd_dn4, locals.var_rdd_dn5, locals.var_rdd_dn6, locals.var_rdd_dn7, locals.var_rdd_dn8, locals.var_rdd_dn9, locals.var_rdd_dn10, locals.var_rdd_dn11, locals.var_rdd_dn14,)
    }
};
        locals.var_rdd = assign21650_e16683;
        locals.var_rdd_dn0 = assign21650_e16683_d_n0;
        locals.var_rdd_dn2 = assign21650_e16683_d_n2;
        locals.var_rdd_dn4 = assign21650_e16683_d_n4;
        locals.var_rdd_dn5 = assign21650_e16683_d_n5;
        locals.var_rdd_dn6 = assign21650_e16683_d_n6;
        locals.var_rdd_dn7 = assign21650_e16683_d_n7;
        locals.var_rdd_dn8 = assign21650_e16683_d_n8;
        locals.var_rdd_dn9 = assign21650_e16683_d_n9;
        locals.var_rdd_dn10 = assign21650_e16683_d_n10;
        locals.var_rdd_dn11 = assign21650_e16683_d_n11;
        locals.var_rdd_dn14 = assign21650_e16683_d_n14;
        locals.var_rdd_rv = 0.0;

        let (assign21660_e16695, assign21660_e16695_d_n0, assign21660_e16695_d_n2, assign21660_e16695_d_n4, assign21660_e16695_d_n5, assign21660_e16695_d_n6, assign21660_e16695_d_n7, assign21660_e16695_d_n8, assign21660_e16695_d_n9, assign21660_e16695_d_n10, assign21660_e16695_d_n11, assign21660_e16695_d_n14,) = {
    if (locals.var_guard411 != 0.0) {
        let assign21660_e16688: f64 = (locals.var_vdsemodenml * locals.var_rs0);
        let assign21660_e16689: f64 = (locals.var_rsd + assign21660_e16688);
        let assign21660_e16692: f64 = (locals.var_vdsemodervs * locals.var_rd0);
        let assign21660_e16693: f64 = (assign21660_e16689 + assign21660_e16692);
        (assign21660_e16693, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    } else {
        (locals.var_rsd, locals.var_rsd_dn0, locals.var_rsd_dn2, locals.var_rsd_dn4, locals.var_rsd_dn5, locals.var_rsd_dn6, locals.var_rsd_dn7, locals.var_rsd_dn8, locals.var_rsd_dn9, locals.var_rsd_dn10, locals.var_rsd_dn11, locals.var_rsd_dn14,)
    }
};
        locals.var_rsd = assign21660_e16695;
        locals.var_rsd_dn0 = assign21660_e16695_d_n0;
        locals.var_rsd_dn2 = assign21660_e16695_d_n2;
        locals.var_rsd_dn4 = assign21660_e16695_d_n4;
        locals.var_rsd_dn5 = assign21660_e16695_d_n5;
        locals.var_rsd_dn6 = assign21660_e16695_d_n6;
        locals.var_rsd_dn7 = assign21660_e16695_d_n7;
        locals.var_rsd_dn8 = assign21660_e16695_d_n8;
        locals.var_rsd_dn9 = assign21660_e16695_d_n9;
        locals.var_rsd_dn10 = assign21660_e16695_d_n10;
        locals.var_rsd_dn11 = assign21660_e16695_d_n11;
        locals.var_rsd_dn14 = assign21660_e16695_d_n14;
        locals.var_rsd_rv = 0.0;

        let (assign21670_e16705, assign21670_e16705_d_n0, assign21670_e16705_d_n2, assign21670_e16705_d_n4, assign21670_e16705_d_n5, assign21670_e16705_d_n6, assign21670_e16705_d_n7, assign21670_e16705_d_n8, assign21670_e16705_d_n9, assign21670_e16705_d_n10, assign21670_e16705_d_n11, assign21670_e16705_d_n14,) = {
    if (locals.var_guard411 != 0.0) {
        let assign21670_e16699: f64 = (locals.var_vdsemodenml * locals.var_rdd);
        let assign21670_e16702: f64 = (locals.var_vdsemodervs * locals.var_rsd);
        let assign21670_e16703: f64 = (assign21670_e16699 + assign21670_e16702);
        (assign21670_e16703, ((locals.var_vdsemodenml * locals.var_rdd_dn0) + (locals.var_vdsemodervs * locals.var_rsd_dn0)), ((locals.var_vdsemodenml * locals.var_rdd_dn2) + (locals.var_vdsemodervs * locals.var_rsd_dn2)), ((locals.var_vdsemodenml * locals.var_rdd_dn4) + (locals.var_vdsemodervs * locals.var_rsd_dn4)), ((locals.var_vdsemodenml * locals.var_rdd_dn5) + (locals.var_vdsemodervs * locals.var_rsd_dn5)), ((locals.var_vdsemodenml * locals.var_rdd_dn6) + (locals.var_vdsemodervs * locals.var_rsd_dn6)), ((locals.var_vdsemodenml * locals.var_rdd_dn7) + (locals.var_vdsemodervs * locals.var_rsd_dn7)), ((locals.var_vdsemodenml * locals.var_rdd_dn8) + (locals.var_vdsemodervs * locals.var_rsd_dn8)), ((locals.var_vdsemodenml * locals.var_rdd_dn9) + (locals.var_vdsemodervs * locals.var_rsd_dn9)), ((locals.var_vdsemodenml * locals.var_rdd_dn10) + (locals.var_vdsemodervs * locals.var_rsd_dn10)), ((locals.var_vdsemodenml * locals.var_rdd_dn11) + (locals.var_vdsemodervs * locals.var_rsd_dn11)), ((locals.var_vdsemodenml * locals.var_rdd_dn14) + (locals.var_vdsemodervs * locals.var_rsd_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21670_e16705;
        locals.var_t0_dn0 = assign21670_e16705_d_n0;
        locals.var_t0_dn2 = assign21670_e16705_d_n2;
        locals.var_t0_dn4 = assign21670_e16705_d_n4;
        locals.var_t0_dn5 = assign21670_e16705_d_n5;
        locals.var_t0_dn6 = assign21670_e16705_d_n6;
        locals.var_t0_dn7 = assign21670_e16705_d_n7;
        locals.var_t0_dn8 = assign21670_e16705_d_n8;
        locals.var_t0_dn9 = assign21670_e16705_d_n9;
        locals.var_t0_dn10 = assign21670_e16705_d_n10;
        locals.var_t0_dn11 = assign21670_e16705_d_n11;
        locals.var_t0_dn14 = assign21670_e16705_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21710_e16737, assign21710_e16737_d_n0, assign21710_e16737_d_n2, assign21710_e16737_d_n4, assign21710_e16737_d_n5, assign21710_e16737_d_n6, assign21710_e16737_d_n7, assign21710_e16737_d_n8, assign21710_e16737_d_n9, assign21710_e16737_d_n10, assign21710_e16737_d_n11, assign21710_e16737_d_n14,) = {
    if (locals.var_guard411 != 0.0) {
        let assign21710_e16731: f64 = (locals.var_vdsemodenml * locals.var_rsd);
        let assign21710_e16734: f64 = (locals.var_vdsemodervs * locals.var_rdd);
        let assign21710_e16735: f64 = (assign21710_e16731 + assign21710_e16734);
        (assign21710_e16735, ((locals.var_vdsemodenml * locals.var_rsd_dn0) + (locals.var_vdsemodervs * locals.var_rdd_dn0)), ((locals.var_vdsemodenml * locals.var_rsd_dn2) + (locals.var_vdsemodervs * locals.var_rdd_dn2)), ((locals.var_vdsemodenml * locals.var_rsd_dn4) + (locals.var_vdsemodervs * locals.var_rdd_dn4)), ((locals.var_vdsemodenml * locals.var_rsd_dn5) + (locals.var_vdsemodervs * locals.var_rdd_dn5)), ((locals.var_vdsemodenml * locals.var_rsd_dn6) + (locals.var_vdsemodervs * locals.var_rdd_dn6)), ((locals.var_vdsemodenml * locals.var_rsd_dn7) + (locals.var_vdsemodervs * locals.var_rdd_dn7)), ((locals.var_vdsemodenml * locals.var_rsd_dn8) + (locals.var_vdsemodervs * locals.var_rdd_dn8)), ((locals.var_vdsemodenml * locals.var_rsd_dn9) + (locals.var_vdsemodervs * locals.var_rdd_dn9)), ((locals.var_vdsemodenml * locals.var_rsd_dn10) + (locals.var_vdsemodervs * locals.var_rdd_dn10)), ((locals.var_vdsemodenml * locals.var_rsd_dn11) + (locals.var_vdsemodervs * locals.var_rdd_dn11)), ((locals.var_vdsemodenml * locals.var_rsd_dn14) + (locals.var_vdsemodervs * locals.var_rdd_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21710_e16737;
        locals.var_t0_dn0 = assign21710_e16737_d_n0;
        locals.var_t0_dn2 = assign21710_e16737_d_n2;
        locals.var_t0_dn4 = assign21710_e16737_d_n4;
        locals.var_t0_dn5 = assign21710_e16737_d_n5;
        locals.var_t0_dn6 = assign21710_e16737_d_n6;
        locals.var_t0_dn7 = assign21710_e16737_d_n7;
        locals.var_t0_dn8 = assign21710_e16737_d_n8;
        locals.var_t0_dn9 = assign21710_e16737_d_n9;
        locals.var_t0_dn10 = assign21710_e16737_d_n10;
        locals.var_t0_dn11 = assign21710_e16737_d_n11;
        locals.var_t0_dn14 = assign21710_e16737_d_n14;
        locals.var_t0_rv = 0.0;

        let assign21750_e16762: f64 = if locals.var_vbs > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard424 = assign21750_e16762;
        locals.var_guard424_rv = 0.0;

        let (assign21760_e16768, assign21760_e16768_d_n0, assign21760_e16768_d_n2, assign21760_e16768_d_n4, assign21760_e16768_d_n5, assign21760_e16768_d_n6, assign21760_e16768_d_n7, assign21760_e16768_d_n8, assign21760_e16768_d_n9, assign21760_e16768_d_n10, assign21760_e16768_d_n11, assign21760_e16768_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21760_e16766: f64 = (locals.var_vbs - locals.var_vbs_bnd);
        (assign21760_e16766, (-locals.var_vbs_bnd_dn0), (-locals.var_vbs_bnd_dn2), (-locals.var_vbs_bnd_dn4), (-locals.var_vbs_bnd_dn5), (locals.var_vbs_dn6 - locals.var_vbs_bnd_dn6), (-locals.var_vbs_bnd_dn7), (locals.var_vbs_dn8 - locals.var_vbs_bnd_dn8), (locals.var_vbs_dn9 - locals.var_vbs_bnd_dn9), (-locals.var_vbs_bnd_dn10), (-locals.var_vbs_bnd_dn11), (-locals.var_vbs_bnd_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign21760_e16768;
        locals.var_t1_dn0 = assign21760_e16768_d_n0;
        locals.var_t1_dn2 = assign21760_e16768_d_n2;
        locals.var_t1_dn4 = assign21760_e16768_d_n4;
        locals.var_t1_dn5 = assign21760_e16768_d_n5;
        locals.var_t1_dn6 = assign21760_e16768_d_n6;
        locals.var_t1_dn7 = assign21760_e16768_d_n7;
        locals.var_t1_dn8 = assign21760_e16768_d_n8;
        locals.var_t1_dn9 = assign21760_e16768_d_n9;
        locals.var_t1_dn10 = assign21760_e16768_d_n10;
        locals.var_t1_dn11 = assign21760_e16768_d_n11;
        locals.var_t1_dn14 = assign21760_e16768_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign21770_e16774, assign21770_e16774_d_n0, assign21770_e16774_d_n2, assign21770_e16774_d_n4, assign21770_e16774_d_n5, assign21770_e16774_d_n6, assign21770_e16774_d_n7, assign21770_e16774_d_n8, assign21770_e16774_d_n9, assign21770_e16774_d_n10, assign21770_e16774_d_n11, assign21770_e16774_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21770_e16772: f64 = (locals.var_vbs_max - locals.var_vbs_bnd);
        (assign21770_e16772, (locals.var_vbs_max_dn0 - locals.var_vbs_bnd_dn0), (locals.var_vbs_max_dn2 - locals.var_vbs_bnd_dn2), (locals.var_vbs_max_dn4 - locals.var_vbs_bnd_dn4), (locals.var_vbs_max_dn5 - locals.var_vbs_bnd_dn5), (locals.var_vbs_max_dn6 - locals.var_vbs_bnd_dn6), (locals.var_vbs_max_dn7 - locals.var_vbs_bnd_dn7), (locals.var_vbs_max_dn8 - locals.var_vbs_bnd_dn8), (locals.var_vbs_max_dn9 - locals.var_vbs_bnd_dn9), (locals.var_vbs_max_dn10 - locals.var_vbs_bnd_dn10), (locals.var_vbs_max_dn11 - locals.var_vbs_bnd_dn11), (locals.var_vbs_max_dn14 - locals.var_vbs_bnd_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign21770_e16774;
        locals.var_t2_dn0 = assign21770_e16774_d_n0;
        locals.var_t2_dn2 = assign21770_e16774_d_n2;
        locals.var_t2_dn4 = assign21770_e16774_d_n4;
        locals.var_t2_dn5 = assign21770_e16774_d_n5;
        locals.var_t2_dn6 = assign21770_e16774_d_n6;
        locals.var_t2_dn7 = assign21770_e16774_d_n7;
        locals.var_t2_dn8 = assign21770_e16774_d_n8;
        locals.var_t2_dn9 = assign21770_e16774_d_n9;
        locals.var_t2_dn10 = assign21770_e16774_d_n10;
        locals.var_t2_dn11 = assign21770_e16774_d_n11;
        locals.var_t2_dn14 = assign21770_e16774_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign21780_e16780, assign21780_e16780_d_n0, assign21780_e16780_d_n2, assign21780_e16780_d_n4, assign21780_e16780_d_n5, assign21780_e16780_d_n6, assign21780_e16780_d_n7, assign21780_e16780_d_n8, assign21780_e16780_d_n9, assign21780_e16780_d_n10, assign21780_e16780_d_n11, assign21780_e16780_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21780_e16778: f64 = (locals.var_t1 / locals.var_t2);
        (assign21780_e16778, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn14 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign21780_e16780;
        locals.var_tmf1_dn0 = assign21780_e16780_d_n0;
        locals.var_tmf1_dn2 = assign21780_e16780_d_n2;
        locals.var_tmf1_dn4 = assign21780_e16780_d_n4;
        locals.var_tmf1_dn5 = assign21780_e16780_d_n5;
        locals.var_tmf1_dn6 = assign21780_e16780_d_n6;
        locals.var_tmf1_dn7 = assign21780_e16780_d_n7;
        locals.var_tmf1_dn8 = assign21780_e16780_d_n8;
        locals.var_tmf1_dn9 = assign21780_e16780_d_n9;
        locals.var_tmf1_dn10 = assign21780_e16780_d_n10;
        locals.var_tmf1_dn11 = assign21780_e16780_d_n11;
        locals.var_tmf1_dn14 = assign21780_e16780_d_n14;
        locals.var_tmf1_rv = 0.0;

        let (assign21790_e16786, assign21790_e16786_d_n0, assign21790_e16786_d_n2, assign21790_e16786_d_n4, assign21790_e16786_d_n5, assign21790_e16786_d_n6, assign21790_e16786_d_n7, assign21790_e16786_d_n8, assign21790_e16786_d_n9, assign21790_e16786_d_n10, assign21790_e16786_d_n11, assign21790_e16786_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21790_e16784: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign21790_e16784, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign21790_e16786;
        locals.var_tmf2_dn0 = assign21790_e16786_d_n0;
        locals.var_tmf2_dn2 = assign21790_e16786_d_n2;
        locals.var_tmf2_dn4 = assign21790_e16786_d_n4;
        locals.var_tmf2_dn5 = assign21790_e16786_d_n5;
        locals.var_tmf2_dn6 = assign21790_e16786_d_n6;
        locals.var_tmf2_dn7 = assign21790_e16786_d_n7;
        locals.var_tmf2_dn8 = assign21790_e16786_d_n8;
        locals.var_tmf2_dn9 = assign21790_e16786_d_n9;
        locals.var_tmf2_dn10 = assign21790_e16786_d_n10;
        locals.var_tmf2_dn11 = assign21790_e16786_d_n11;
        locals.var_tmf2_dn14 = assign21790_e16786_d_n14;
        locals.var_tmf2_rv = 0.0;

        let (assign21800_e16792, assign21800_e16792_d_n0, assign21800_e16792_d_n2, assign21800_e16792_d_n4, assign21800_e16792_d_n5, assign21800_e16792_d_n6, assign21800_e16792_d_n7, assign21800_e16792_d_n8, assign21800_e16792_d_n9, assign21800_e16792_d_n10, assign21800_e16792_d_n11, assign21800_e16792_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21800_e16790: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign21800_e16790, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign21800_e16792;
        locals.var_tmf3_dn0 = assign21800_e16792_d_n0;
        locals.var_tmf3_dn2 = assign21800_e16792_d_n2;
        locals.var_tmf3_dn4 = assign21800_e16792_d_n4;
        locals.var_tmf3_dn5 = assign21800_e16792_d_n5;
        locals.var_tmf3_dn6 = assign21800_e16792_d_n6;
        locals.var_tmf3_dn7 = assign21800_e16792_d_n7;
        locals.var_tmf3_dn8 = assign21800_e16792_d_n8;
        locals.var_tmf3_dn9 = assign21800_e16792_d_n9;
        locals.var_tmf3_dn10 = assign21800_e16792_d_n10;
        locals.var_tmf3_dn11 = assign21800_e16792_d_n11;
        locals.var_tmf3_dn14 = assign21800_e16792_d_n14;
        locals.var_tmf3_rv = 0.0;

        let (assign21810_e16798, assign21810_e16798_d_n0, assign21810_e16798_d_n2, assign21810_e16798_d_n4, assign21810_e16798_d_n5, assign21810_e16798_d_n6, assign21810_e16798_d_n7, assign21810_e16798_d_n8, assign21810_e16798_d_n9, assign21810_e16798_d_n10, assign21810_e16798_d_n11, assign21810_e16798_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21810_e16796: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign21810_e16796, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign21810_e16798;
        locals.var_tmf4_dn0 = assign21810_e16798_d_n0;
        locals.var_tmf4_dn2 = assign21810_e16798_d_n2;
        locals.var_tmf4_dn4 = assign21810_e16798_d_n4;
        locals.var_tmf4_dn5 = assign21810_e16798_d_n5;
        locals.var_tmf4_dn6 = assign21810_e16798_d_n6;
        locals.var_tmf4_dn7 = assign21810_e16798_d_n7;
        locals.var_tmf4_dn8 = assign21810_e16798_d_n8;
        locals.var_tmf4_dn9 = assign21810_e16798_d_n9;
        locals.var_tmf4_dn10 = assign21810_e16798_d_n10;
        locals.var_tmf4_dn11 = assign21810_e16798_d_n11;
        locals.var_tmf4_dn14 = assign21810_e16798_d_n14;
        locals.var_tmf4_rv = 0.0;

        let (assign21820_e16812, assign21820_e16812_d_n0, assign21820_e16812_d_n2, assign21820_e16812_d_n4, assign21820_e16812_d_n5, assign21820_e16812_d_n6, assign21820_e16812_d_n7, assign21820_e16812_d_n8, assign21820_e16812_d_n9, assign21820_e16812_d_n10, assign21820_e16812_d_n11, assign21820_e16812_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21820_e16803: f64 = (1.0 + locals.var_tmf1);
        let assign21820_e16805: f64 = (assign21820_e16803 + locals.var_tmf2);
        let assign21820_e16807: f64 = (assign21820_e16805 + locals.var_tmf3);
        let assign21820_e16809: f64 = (assign21820_e16807 + locals.var_tmf4);
        let assign21820_e16810: f64 = (1.0 / assign21820_e16809);
        (assign21820_e16810, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign21820_e16809 * assign21820_e16809))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign21820_e16809 * assign21820_e16809))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign21820_e16809 * assign21820_e16809))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign21820_e16809 * assign21820_e16809))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign21820_e16809 * assign21820_e16809))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign21820_e16809 * assign21820_e16809))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign21820_e16809 * assign21820_e16809))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign21820_e16809 * assign21820_e16809))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign21820_e16809 * assign21820_e16809))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign21820_e16809 * assign21820_e16809))), (-((((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) + locals.var_tmf3_dn14) + locals.var_tmf4_dn14) / (assign21820_e16809 * assign21820_e16809))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign21820_e16812;
        locals.var_tmf0_dn0 = assign21820_e16812_d_n0;
        locals.var_tmf0_dn2 = assign21820_e16812_d_n2;
        locals.var_tmf0_dn4 = assign21820_e16812_d_n4;
        locals.var_tmf0_dn5 = assign21820_e16812_d_n5;
        locals.var_tmf0_dn6 = assign21820_e16812_d_n6;
        locals.var_tmf0_dn7 = assign21820_e16812_d_n7;
        locals.var_tmf0_dn8 = assign21820_e16812_d_n8;
        locals.var_tmf0_dn9 = assign21820_e16812_d_n9;
        locals.var_tmf0_dn10 = assign21820_e16812_d_n10;
        locals.var_tmf0_dn11 = assign21820_e16812_d_n11;
        locals.var_tmf0_dn14 = assign21820_e16812_d_n14;
        locals.var_tmf0_rv = 0.0;

        let (assign21830_e16833, assign21830_e16833_d_n0, assign21830_e16833_d_n2, assign21830_e16833_d_n4, assign21830_e16833_d_n5, assign21830_e16833_d_n6, assign21830_e16833_d_n7, assign21830_e16833_d_n8, assign21830_e16833_d_n9, assign21830_e16833_d_n10, assign21830_e16833_d_n11, assign21830_e16833_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21830_e16817: f64 = (2.0 * locals.var_tmf1);
        let assign21830_e16818: f64 = (1.0 + assign21830_e16817);
        let assign21830_e16821: f64 = (3.0 * locals.var_tmf2);
        let assign21830_e16822: f64 = (assign21830_e16818 + assign21830_e16821);
        let assign21830_e16825: f64 = (4.0 * locals.var_tmf3);
        let assign21830_e16826: f64 = (assign21830_e16822 + assign21830_e16825);
        let assign21830_e16827: f64 = (-assign21830_e16826);
        let assign21830_e16829: f64 = (assign21830_e16827 * locals.var_tmf0);
        let assign21830_e16831: f64 = (assign21830_e16829 * locals.var_tmf0);
        (assign21830_e16831, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn11)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn11)), (((((-(((2.0 * locals.var_tmf1_dn14) + (3.0 * locals.var_tmf2_dn14)) + (4.0 * locals.var_tmf3_dn14))) * locals.var_tmf0) + (assign21830_e16827 * locals.var_tmf0_dn14)) * locals.var_tmf0) + (assign21830_e16829 * locals.var_tmf0_dn14)),)
    } else {
        (locals.var_vbscldvbs, locals.var_vbscldvbs_dn0, locals.var_vbscldvbs_dn2, locals.var_vbscldvbs_dn4, locals.var_vbscldvbs_dn5, locals.var_vbscldvbs_dn6, locals.var_vbscldvbs_dn7, locals.var_vbscldvbs_dn8, locals.var_vbscldvbs_dn9, locals.var_vbscldvbs_dn10, locals.var_vbscldvbs_dn11, locals.var_vbscldvbs_dn14,)
    }
};
        locals.var_vbscldvbs = assign21830_e16833;
        locals.var_vbscldvbs_dn0 = assign21830_e16833_d_n0;
        locals.var_vbscldvbs_dn2 = assign21830_e16833_d_n2;
        locals.var_vbscldvbs_dn4 = assign21830_e16833_d_n4;
        locals.var_vbscldvbs_dn5 = assign21830_e16833_d_n5;
        locals.var_vbscldvbs_dn6 = assign21830_e16833_d_n6;
        locals.var_vbscldvbs_dn7 = assign21830_e16833_d_n7;
        locals.var_vbscldvbs_dn8 = assign21830_e16833_d_n8;
        locals.var_vbscldvbs_dn9 = assign21830_e16833_d_n9;
        locals.var_vbscldvbs_dn10 = assign21830_e16833_d_n10;
        locals.var_vbscldvbs_dn11 = assign21830_e16833_d_n11;
        locals.var_vbscldvbs_dn14 = assign21830_e16833_d_n14;
        locals.var_vbscldvbs_rv = 0.0;

        let (assign21840_e16841, assign21840_e16841_d_n0, assign21840_e16841_d_n2, assign21840_e16841_d_n4, assign21840_e16841_d_n5, assign21840_e16841_d_n6, assign21840_e16841_d_n7, assign21840_e16841_d_n8, assign21840_e16841_d_n9, assign21840_e16841_d_n10, assign21840_e16841_d_n11, assign21840_e16841_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21840_e16838: f64 = (1.0 - locals.var_tmf0);
        let assign21840_e16839: f64 = (locals.var_t2 * assign21840_e16838);
        (assign21840_e16839, ((locals.var_t2_dn0 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn11 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn11))), ((locals.var_t2_dn14 * assign21840_e16838) + (locals.var_t2 * (-locals.var_tmf0_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign21840_e16841;
        locals.var_ty_dn0 = assign21840_e16841_d_n0;
        locals.var_ty_dn2 = assign21840_e16841_d_n2;
        locals.var_ty_dn4 = assign21840_e16841_d_n4;
        locals.var_ty_dn5 = assign21840_e16841_d_n5;
        locals.var_ty_dn6 = assign21840_e16841_d_n6;
        locals.var_ty_dn7 = assign21840_e16841_d_n7;
        locals.var_ty_dn8 = assign21840_e16841_d_n8;
        locals.var_ty_dn9 = assign21840_e16841_d_n9;
        locals.var_ty_dn10 = assign21840_e16841_d_n10;
        locals.var_ty_dn11 = assign21840_e16841_d_n11;
        locals.var_ty_dn14 = assign21840_e16841_d_n14;
        locals.var_ty_rv = 0.0;

        let (assign21850_e16851, assign21850_e16851_d_n0, assign21850_e16851_d_n2, assign21850_e16851_d_n4, assign21850_e16851_d_n5, assign21850_e16851_d_n6, assign21850_e16851_d_n7, assign21850_e16851_d_n8, assign21850_e16851_d_n9, assign21850_e16851_d_n10, assign21850_e16851_d_n11, assign21850_e16851_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21850_e16845: f64 = (1.0 - locals.var_tmf0);
        let assign21850_e16848: f64 = (locals.var_tmf1 * locals.var_vbscldvbs);
        let assign21850_e16849: f64 = (assign21850_e16845 + assign21850_e16848);
        (assign21850_e16849, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn10))), ((-locals.var_tmf0_dn11) + ((locals.var_tmf1_dn11 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn11))), ((-locals.var_tmf0_dn14) + ((locals.var_tmf1_dn14 * locals.var_vbscldvbs) + (locals.var_tmf1 * locals.var_vbscldvbs_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign21850_e16851;
        locals.var_t0_dn0 = assign21850_e16851_d_n0;
        locals.var_t0_dn2 = assign21850_e16851_d_n2;
        locals.var_t0_dn4 = assign21850_e16851_d_n4;
        locals.var_t0_dn5 = assign21850_e16851_d_n5;
        locals.var_t0_dn6 = assign21850_e16851_d_n6;
        locals.var_t0_dn7 = assign21850_e16851_d_n7;
        locals.var_t0_dn8 = assign21850_e16851_d_n8;
        locals.var_t0_dn9 = assign21850_e16851_d_n9;
        locals.var_t0_dn10 = assign21850_e16851_d_n10;
        locals.var_t0_dn11 = assign21850_e16851_d_n11;
        locals.var_t0_dn14 = assign21850_e16851_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign21860_e16856, assign21860_e16856_d_n0, assign21860_e16856_d_n2, assign21860_e16856_d_n4, assign21860_e16856_d_n5, assign21860_e16856_d_n6, assign21860_e16856_d_n7, assign21860_e16856_d_n8, assign21860_e16856_d_n9, assign21860_e16856_d_n10, assign21860_e16856_d_n11, assign21860_e16856_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21860_e16854: f64 = (-locals.var_vbscldvbs);
        (assign21860_e16854, (-locals.var_vbscldvbs_dn0), (-locals.var_vbscldvbs_dn2), (-locals.var_vbscldvbs_dn4), (-locals.var_vbscldvbs_dn5), (-locals.var_vbscldvbs_dn6), (-locals.var_vbscldvbs_dn7), (-locals.var_vbscldvbs_dn8), (-locals.var_vbscldvbs_dn9), (-locals.var_vbscldvbs_dn10), (-locals.var_vbscldvbs_dn11), (-locals.var_vbscldvbs_dn14),)
    } else {
        (locals.var_vbscldvbs, locals.var_vbscldvbs_dn0, locals.var_vbscldvbs_dn2, locals.var_vbscldvbs_dn4, locals.var_vbscldvbs_dn5, locals.var_vbscldvbs_dn6, locals.var_vbscldvbs_dn7, locals.var_vbscldvbs_dn8, locals.var_vbscldvbs_dn9, locals.var_vbscldvbs_dn10, locals.var_vbscldvbs_dn11, locals.var_vbscldvbs_dn14,)
    }
};
        locals.var_vbscldvbs = assign21860_e16856;
        locals.var_vbscldvbs_dn0 = assign21860_e16856_d_n0;
        locals.var_vbscldvbs_dn2 = assign21860_e16856_d_n2;
        locals.var_vbscldvbs_dn4 = assign21860_e16856_d_n4;
        locals.var_vbscldvbs_dn5 = assign21860_e16856_d_n5;
        locals.var_vbscldvbs_dn6 = assign21860_e16856_d_n6;
        locals.var_vbscldvbs_dn7 = assign21860_e16856_d_n7;
        locals.var_vbscldvbs_dn8 = assign21860_e16856_d_n8;
        locals.var_vbscldvbs_dn9 = assign21860_e16856_d_n9;
        locals.var_vbscldvbs_dn10 = assign21860_e16856_d_n10;
        locals.var_vbscldvbs_dn11 = assign21860_e16856_d_n11;
        locals.var_vbscldvbs_dn14 = assign21860_e16856_d_n14;
        locals.var_vbscldvbs_rv = 0.0;

        let (assign21870_e16862, assign21870_e16862_d_n0, assign21870_e16862_d_n2, assign21870_e16862_d_n4, assign21870_e16862_d_n5, assign21870_e16862_d_n6, assign21870_e16862_d_n7, assign21870_e16862_d_n8, assign21870_e16862_d_n9, assign21870_e16862_d_n10, assign21870_e16862_d_n11, assign21870_e16862_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21870_e16860: f64 = (locals.var_vbs_bnd + locals.var_ty);
        (assign21870_e16860, (locals.var_vbs_bnd_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_dn11 + locals.var_ty_dn11), (locals.var_vbs_bnd_dn14 + locals.var_ty_dn14),)
    } else {
        (locals.var_vbscl, locals.var_vbscl_dn0, locals.var_vbscl_dn2, locals.var_vbscl_dn4, locals.var_vbscl_dn5, locals.var_vbscl_dn6, locals.var_vbscl_dn7, locals.var_vbscl_dn8, locals.var_vbscl_dn9, locals.var_vbscl_dn10, locals.var_vbscl_dn11, locals.var_vbscl_dn14,)
    }
};
        locals.var_vbscl = assign21870_e16862;
        locals.var_vbscl_dn0 = assign21870_e16862_d_n0;
        locals.var_vbscl_dn2 = assign21870_e16862_d_n2;
        locals.var_vbscl_dn4 = assign21870_e16862_d_n4;
        locals.var_vbscl_dn5 = assign21870_e16862_d_n5;
        locals.var_vbscl_dn6 = assign21870_e16862_d_n6;
        locals.var_vbscl_dn7 = assign21870_e16862_d_n7;
        locals.var_vbscl_dn8 = assign21870_e16862_d_n8;
        locals.var_vbscl_dn9 = assign21870_e16862_d_n9;
        locals.var_vbscl_dn10 = assign21870_e16862_d_n10;
        locals.var_vbscl_dn11 = assign21870_e16862_d_n11;
        locals.var_vbscl_dn14 = assign21870_e16862_d_n14;
        locals.var_vbscl_rv = 0.0;

        let (assign21880_e16868, assign21880_e16868_d_n0, assign21880_e16868_d_n2, assign21880_e16868_d_n4, assign21880_e16868_d_n5, assign21880_e16868_d_n6, assign21880_e16868_d_n7, assign21880_e16868_d_n8, assign21880_e16868_d_n9, assign21880_e16868_d_n10, assign21880_e16868_d_n11, assign21880_e16868_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21880_e16866: f64 = (1.0 / locals.var_t2);
        (assign21880_e16866, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn14 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign21880_e16868;
        locals.var_t3_dn0 = assign21880_e16868_d_n0;
        locals.var_t3_dn2 = assign21880_e16868_d_n2;
        locals.var_t3_dn4 = assign21880_e16868_d_n4;
        locals.var_t3_dn5 = assign21880_e16868_d_n5;
        locals.var_t3_dn6 = assign21880_e16868_d_n6;
        locals.var_t3_dn7 = assign21880_e16868_d_n7;
        locals.var_t3_dn8 = assign21880_e16868_d_n8;
        locals.var_t3_dn9 = assign21880_e16868_d_n9;
        locals.var_t3_dn10 = assign21880_e16868_d_n10;
        locals.var_t3_dn11 = assign21880_e16868_d_n11;
        locals.var_t3_dn14 = assign21880_e16868_d_n14;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_58(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21890_e16874, assign21890_e16874_d_n0, assign21890_e16874_d_n2, assign21890_e16874_d_n4, assign21890_e16874_d_n5, assign21890_e16874_d_n6, assign21890_e16874_d_n7, assign21890_e16874_d_n8, assign21890_e16874_d_n9, assign21890_e16874_d_n10, assign21890_e16874_d_n11, assign21890_e16874_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21890_e16872: f64 = (locals.var_t1 * locals.var_t3);
        (assign21890_e16872, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign21890_e16874;
        locals.var_t4_dn0 = assign21890_e16874_d_n0;
        locals.var_t4_dn2 = assign21890_e16874_d_n2;
        locals.var_t4_dn4 = assign21890_e16874_d_n4;
        locals.var_t4_dn5 = assign21890_e16874_d_n5;
        locals.var_t4_dn6 = assign21890_e16874_d_n6;
        locals.var_t4_dn7 = assign21890_e16874_d_n7;
        locals.var_t4_dn8 = assign21890_e16874_d_n8;
        locals.var_t4_dn9 = assign21890_e16874_d_n9;
        locals.var_t4_dn10 = assign21890_e16874_d_n10;
        locals.var_t4_dn11 = assign21890_e16874_d_n11;
        locals.var_t4_dn14 = assign21890_e16874_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign21900_e16880, assign21900_e16880_d_n0, assign21900_e16880_d_n2, assign21900_e16880_d_n4, assign21900_e16880_d_n5, assign21900_e16880_d_n6, assign21900_e16880_d_n7, assign21900_e16880_d_n8, assign21900_e16880_d_n9, assign21900_e16880_d_n10, assign21900_e16880_d_n11, assign21900_e16880_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21900_e16878: f64 = (locals.var_t4 * locals.var_t4);
        (assign21900_e16878, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)), ((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)), ((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)), ((locals.var_t4_dn14 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign21900_e16880;
        locals.var_t5_dn0 = assign21900_e16880_d_n0;
        locals.var_t5_dn2 = assign21900_e16880_d_n2;
        locals.var_t5_dn4 = assign21900_e16880_d_n4;
        locals.var_t5_dn5 = assign21900_e16880_d_n5;
        locals.var_t5_dn6 = assign21900_e16880_d_n6;
        locals.var_t5_dn7 = assign21900_e16880_d_n7;
        locals.var_t5_dn8 = assign21900_e16880_d_n8;
        locals.var_t5_dn9 = assign21900_e16880_d_n9;
        locals.var_t5_dn10 = assign21900_e16880_d_n10;
        locals.var_t5_dn11 = assign21900_e16880_d_n11;
        locals.var_t5_dn14 = assign21900_e16880_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign21910_e16894, assign21910_e16894_d_n0, assign21910_e16894_d_n2, assign21910_e16894_d_n4, assign21910_e16894_d_n5, assign21910_e16894_d_n6, assign21910_e16894_d_n7, assign21910_e16894_d_n8, assign21910_e16894_d_n9, assign21910_e16894_d_n10, assign21910_e16894_d_n11, assign21910_e16894_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21910_e16884: f64 = (1.0 + locals.var_t4);
        let assign21910_e16888: f64 = (1.0 + locals.var_t4);
        let assign21910_e16890: f64 = (assign21910_e16888 + locals.var_t5);
        let assign21910_e16891: f64 = (locals.var_t5 * assign21910_e16890);
        let assign21910_e16892: f64 = (assign21910_e16884 + assign21910_e16891);
        (assign21910_e16892, (locals.var_t4_dn0 + ((locals.var_t5_dn0 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn0 + locals.var_t5_dn0)))), (locals.var_t4_dn2 + ((locals.var_t5_dn2 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn2 + locals.var_t5_dn2)))), (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn4 + locals.var_t5_dn4)))), (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn5 + locals.var_t5_dn5)))), (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn6 + locals.var_t5_dn6)))), (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn7 + locals.var_t5_dn7)))), (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn8 + locals.var_t5_dn8)))), (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn9 + locals.var_t5_dn9)))), (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn10 + locals.var_t5_dn10)))), (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn11 + locals.var_t5_dn11)))), (locals.var_t4_dn14 + ((locals.var_t5_dn14 * assign21910_e16890) + (locals.var_t5 * (locals.var_t4_dn14 + locals.var_t5_dn14)))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign21910_e16894;
        locals.var_t7_dn0 = assign21910_e16894_d_n0;
        locals.var_t7_dn2 = assign21910_e16894_d_n2;
        locals.var_t7_dn4 = assign21910_e16894_d_n4;
        locals.var_t7_dn5 = assign21910_e16894_d_n5;
        locals.var_t7_dn6 = assign21910_e16894_d_n6;
        locals.var_t7_dn7 = assign21910_e16894_d_n7;
        locals.var_t7_dn8 = assign21910_e16894_d_n8;
        locals.var_t7_dn9 = assign21910_e16894_d_n9;
        locals.var_t7_dn10 = assign21910_e16894_d_n10;
        locals.var_t7_dn11 = assign21910_e16894_d_n11;
        locals.var_t7_dn14 = assign21910_e16894_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign21920_e16916, assign21920_e16916_d_n0, assign21920_e16916_d_n2, assign21920_e16916_d_n4, assign21920_e16916_d_n5, assign21920_e16916_d_n6, assign21920_e16916_d_n7, assign21920_e16916_d_n8, assign21920_e16916_d_n9, assign21920_e16916_d_n10, assign21920_e16916_d_n11, assign21920_e16916_d_n14,) = {
    if (locals.var_guard424 != 0.0) {
        let assign21920_e16899: f64 = (2.0 * locals.var_t4);
        let assign21920_e16900: f64 = (1.0 + assign21920_e16899);
        let assign21920_e16903: f64 = (3.0 * locals.var_t5);
        let assign21920_e16904: f64 = (assign21920_e16900 + assign21920_e16903);
        let assign21920_e16907: f64 = (4.0 * locals.var_t4);
        let assign21920_e16909: f64 = (assign21920_e16907 * locals.var_t5);
        let assign21920_e16910: f64 = (assign21920_e16904 + assign21920_e16909);
        let assign21920_e16913: f64 = (locals.var_t7 * locals.var_t7);
        let assign21920_e16914: f64 = (assign21920_e16910 / assign21920_e16913);
        (assign21920_e16914, ((((((2.0 * locals.var_t4_dn0) + (3.0 * locals.var_t5_dn0)) + (((4.0 * locals.var_t4_dn0) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn0))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)))) / (assign21920_e16913 * assign21920_e16913)), ((((((2.0 * locals.var_t4_dn2) + (3.0 * locals.var_t5_dn2)) + (((4.0 * locals.var_t4_dn2) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn2))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)))) / (assign21920_e16913 * assign21920_e16913)), ((((((2.0 * locals.var_t4_dn4) + (3.0 * locals.var_t5_dn4)) + (((4.0 * locals.var_t4_dn4) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn4))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)))) / (assign21920_e16913 * assign21920_e16913)), ((((((2.0 * locals.var_t4_dn5) + (3.0 * locals.var_t5_dn5)) + (((4.0 * locals.var_t4_dn5) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn5))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)))) / (assign21920_e16913 * assign21920_e16913)), ((((((2.0 * locals.var_t4_dn6) + (3.0 * locals.var_t5_dn6)) + (((4.0 * locals.var_t4_dn6) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn6))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)))) / (assign21920_e16913 * assign21920_e16913)), ((((((2.0 * locals.var_t4_dn7) + (3.0 * locals.var_t5_dn7)) + (((4.0 * locals.var_t4_dn7) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn7))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)))) / (assign21920_e16913 * assign21920_e16913)), ((((((2.0 * locals.var_t4_dn8) + (3.0 * locals.var_t5_dn8)) + (((4.0 * locals.var_t4_dn8) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn8))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)))) / (assign21920_e16913 * assign21920_e16913)), ((((((2.0 * locals.var_t4_dn9) + (3.0 * locals.var_t5_dn9)) + (((4.0 * locals.var_t4_dn9) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn9))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)))) / (assign21920_e16913 * assign21920_e16913)), ((((((2.0 * locals.var_t4_dn10) + (3.0 * locals.var_t5_dn10)) + (((4.0 * locals.var_t4_dn10) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn10))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)))) / (assign21920_e16913 * assign21920_e16913)), ((((((2.0 * locals.var_t4_dn11) + (3.0 * locals.var_t5_dn11)) + (((4.0 * locals.var_t4_dn11) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn11))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)))) / (assign21920_e16913 * assign21920_e16913)), ((((((2.0 * locals.var_t4_dn14) + (3.0 * locals.var_t5_dn14)) + (((4.0 * locals.var_t4_dn14) * locals.var_t5) + (assign21920_e16907 * locals.var_t5_dn14))) * assign21920_e16913) - (assign21920_e16910 * ((locals.var_t7_dn14 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn14)))) / (assign21920_e16913 * assign21920_e16913)),)
    } else {
        (locals.var_vbscldvbs, locals.var_vbscldvbs_dn0, locals.var_vbscldvbs_dn2, locals.var_vbscldvbs_dn4, locals.var_vbscldvbs_dn5, locals.var_vbscldvbs_dn6, locals.var_vbscldvbs_dn7, locals.var_vbscldvbs_dn8, locals.var_vbscldvbs_dn9, locals.var_vbscldvbs_dn10, locals.var_vbscldvbs_dn11, locals.var_vbscldvbs_dn14,)
    }
};
        locals.var_vbscldvbs = assign21920_e16916;
        locals.var_vbscldvbs_dn0 = assign21920_e16916_d_n0;
        locals.var_vbscldvbs_dn2 = assign21920_e16916_d_n2;
        locals.var_vbscldvbs_dn4 = assign21920_e16916_d_n4;
        locals.var_vbscldvbs_dn5 = assign21920_e16916_d_n5;
        locals.var_vbscldvbs_dn6 = assign21920_e16916_d_n6;
        locals.var_vbscldvbs_dn7 = assign21920_e16916_d_n7;
        locals.var_vbscldvbs_dn8 = assign21920_e16916_d_n8;
        locals.var_vbscldvbs_dn9 = assign21920_e16916_d_n9;
        locals.var_vbscldvbs_dn10 = assign21920_e16916_d_n10;
        locals.var_vbscldvbs_dn11 = assign21920_e16916_d_n11;
        locals.var_vbscldvbs_dn14 = assign21920_e16916_d_n14;
        locals.var_vbscldvbs_rv = 0.0;

        let (assign21930_e16921, assign21930_e16921_d_n0, assign21930_e16921_d_n2, assign21930_e16921_d_n4, assign21930_e16921_d_n5, assign21930_e16921_d_n6, assign21930_e16921_d_n7, assign21930_e16921_d_n8, assign21930_e16921_d_n9, assign21930_e16921_d_n10, assign21930_e16921_d_n11, assign21930_e16921_d_n14,) = {
    if (locals.var_guard424 == 0.0) {
        (locals.var_vbs, 0.0, 0.0, 0.0, 0.0, locals.var_vbs_dn6, 0.0, locals.var_vbs_dn8, locals.var_vbs_dn9, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscl, locals.var_vbscl_dn0, locals.var_vbscl_dn2, locals.var_vbscl_dn4, locals.var_vbscl_dn5, locals.var_vbscl_dn6, locals.var_vbscl_dn7, locals.var_vbscl_dn8, locals.var_vbscl_dn9, locals.var_vbscl_dn10, locals.var_vbscl_dn11, locals.var_vbscl_dn14,)
    }
};
        locals.var_vbscl = assign21930_e16921;
        locals.var_vbscl_dn0 = assign21930_e16921_d_n0;
        locals.var_vbscl_dn2 = assign21930_e16921_d_n2;
        locals.var_vbscl_dn4 = assign21930_e16921_d_n4;
        locals.var_vbscl_dn5 = assign21930_e16921_d_n5;
        locals.var_vbscl_dn6 = assign21930_e16921_d_n6;
        locals.var_vbscl_dn7 = assign21930_e16921_d_n7;
        locals.var_vbscl_dn8 = assign21930_e16921_d_n8;
        locals.var_vbscl_dn9 = assign21930_e16921_d_n9;
        locals.var_vbscl_dn10 = assign21930_e16921_d_n10;
        locals.var_vbscl_dn11 = assign21930_e16921_d_n11;
        locals.var_vbscl_dn14 = assign21930_e16921_d_n14;
        locals.var_vbscl_rv = 0.0;

        let (assign21940_e16926, assign21940_e16926_d_n0, assign21940_e16926_d_n2, assign21940_e16926_d_n4, assign21940_e16926_d_n5, assign21940_e16926_d_n6, assign21940_e16926_d_n7, assign21940_e16926_d_n8, assign21940_e16926_d_n9, assign21940_e16926_d_n10, assign21940_e16926_d_n11, assign21940_e16926_d_n14,) = {
    if (locals.var_guard424 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscldvbs, locals.var_vbscldvbs_dn0, locals.var_vbscldvbs_dn2, locals.var_vbscldvbs_dn4, locals.var_vbscldvbs_dn5, locals.var_vbscldvbs_dn6, locals.var_vbscldvbs_dn7, locals.var_vbscldvbs_dn8, locals.var_vbscldvbs_dn9, locals.var_vbscldvbs_dn10, locals.var_vbscldvbs_dn11, locals.var_vbscldvbs_dn14,)
    }
};
        locals.var_vbscldvbs = assign21940_e16926;
        locals.var_vbscldvbs_dn0 = assign21940_e16926_d_n0;
        locals.var_vbscldvbs_dn2 = assign21940_e16926_d_n2;
        locals.var_vbscldvbs_dn4 = assign21940_e16926_d_n4;
        locals.var_vbscldvbs_dn5 = assign21940_e16926_d_n5;
        locals.var_vbscldvbs_dn6 = assign21940_e16926_d_n6;
        locals.var_vbscldvbs_dn7 = assign21940_e16926_d_n7;
        locals.var_vbscldvbs_dn8 = assign21940_e16926_d_n8;
        locals.var_vbscldvbs_dn9 = assign21940_e16926_d_n9;
        locals.var_vbscldvbs_dn10 = assign21940_e16926_d_n10;
        locals.var_vbscldvbs_dn11 = assign21940_e16926_d_n11;
        locals.var_vbscldvbs_dn14 = assign21940_e16926_d_n14;
        locals.var_vbscldvbs_rv = 0.0;

        let assign21950_e16929: f64 = (locals.var_vbscldvbs * locals.var_vds);
        let assign21950_e16931: f64 = (assign21950_e16929 / 2.0);
        locals.var_t1 = assign21950_e16931;
        locals.var_t1_dn0 = (((locals.var_vbscldvbs_dn0 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn0)) / 2.0);
        locals.var_t1_dn2 = (((locals.var_vbscldvbs_dn2 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn2)) / 2.0);
        locals.var_t1_dn4 = (((locals.var_vbscldvbs_dn4 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn4)) / 2.0);
        locals.var_t1_dn5 = (((locals.var_vbscldvbs_dn5 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn5)) / 2.0);
        locals.var_t1_dn6 = (((locals.var_vbscldvbs_dn6 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn6)) / 2.0);
        locals.var_t1_dn7 = (((locals.var_vbscldvbs_dn7 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn7)) / 2.0);
        locals.var_t1_dn8 = (((locals.var_vbscldvbs_dn8 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn8)) / 2.0);
        locals.var_t1_dn9 = (((locals.var_vbscldvbs_dn9 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn9)) / 2.0);
        locals.var_t1_dn10 = (((locals.var_vbscldvbs_dn10 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn10)) / 2.0);
        locals.var_t1_dn11 = (((locals.var_vbscldvbs_dn11 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn11)) / 2.0);
        locals.var_t1_dn14 = (((locals.var_vbscldvbs_dn14 * locals.var_vds) + (locals.var_vbscldvbs * locals.var_vds_dn14)) / 2.0);
        locals.var_t1_rv = 0.0;

        let assign21960_e16934: f64 = (2.0 * locals.var_t1);
        let assign21960_e16936: f64 = (assign21960_e16934 / p.p262);
        locals.var_tmf1 = assign21960_e16936;
        locals.var_tmf1_dn0 = ((2.0 * locals.var_t1_dn0) / p.p262);
        locals.var_tmf1_dn2 = ((2.0 * locals.var_t1_dn2) / p.p262);
        locals.var_tmf1_dn4 = ((2.0 * locals.var_t1_dn4) / p.p262);
        locals.var_tmf1_dn5 = ((2.0 * locals.var_t1_dn5) / p.p262);
        locals.var_tmf1_dn6 = ((2.0 * locals.var_t1_dn6) / p.p262);
        locals.var_tmf1_dn7 = ((2.0 * locals.var_t1_dn7) / p.p262);
        locals.var_tmf1_dn8 = ((2.0 * locals.var_t1_dn8) / p.p262);
        locals.var_tmf1_dn9 = ((2.0 * locals.var_t1_dn9) / p.p262);
        locals.var_tmf1_dn10 = ((2.0 * locals.var_t1_dn10) / p.p262);
        locals.var_tmf1_dn11 = ((2.0 * locals.var_t1_dn11) / p.p262);
        locals.var_tmf1_dn14 = ((2.0 * locals.var_t1_dn14) / p.p262);
        locals.var_tmf1_rv = 0.0;

        let assign21970_e16941: f64 = (1.0 / 2.0);
        let assign21970_e16945: f64 = (1.0 / 6.0);
        let assign21970_e16949: f64 = (1.0 / 24.0);
        let assign21970_e16953: f64 = (1.0 / 120.0);
        let assign21970_e16957: f64 = (1.0 / 720.0);
        let assign21970_e16961: f64 = (1.0 / 5040.0);
        let assign21970_e16962: f64 = (locals.var_tmf1 * assign21970_e16961);
        let assign21970_e16963: f64 = (assign21970_e16957 + assign21970_e16962);
        let assign21970_e16964: f64 = (locals.var_tmf1 * assign21970_e16963);
        let assign21970_e16965: f64 = (assign21970_e16953 + assign21970_e16964);
        let assign21970_e16966: f64 = (locals.var_tmf1 * assign21970_e16965);
        let assign21970_e16967: f64 = (assign21970_e16949 + assign21970_e16966);
        let assign21970_e16968: f64 = (locals.var_tmf1 * assign21970_e16967);
        let assign21970_e16969: f64 = (assign21970_e16945 + assign21970_e16968);
        let assign21970_e16970: f64 = (locals.var_tmf1 * assign21970_e16969);
        let assign21970_e16971: f64 = (assign21970_e16941 + assign21970_e16970);
        let assign21970_e16972: f64 = (locals.var_tmf1 * assign21970_e16971);
        let assign21970_e16973: f64 = (1.0 + assign21970_e16972);
        locals.var_tmf2 = assign21970_e16973;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign21970_e16961)))))))))));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign21970_e16961)))))))))));
        locals.var_tmf2_dn4 = ((locals.var_tmf1_dn4 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign21970_e16961)))))))))));
        locals.var_tmf2_dn5 = ((locals.var_tmf1_dn5 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign21970_e16961)))))))))));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign21970_e16961)))))))))));
        locals.var_tmf2_dn7 = ((locals.var_tmf1_dn7 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign21970_e16961)))))))))));
        locals.var_tmf2_dn8 = ((locals.var_tmf1_dn8 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign21970_e16961)))))))))));
        locals.var_tmf2_dn9 = ((locals.var_tmf1_dn9 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign21970_e16961)))))))))));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign21970_e16961)))))))))));
        locals.var_tmf2_dn11 = ((locals.var_tmf1_dn11 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign21970_e16961)))))))))));
        locals.var_tmf2_dn14 = ((locals.var_tmf1_dn14 * assign21970_e16971) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign21970_e16969) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign21970_e16967) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign21970_e16965) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign21970_e16963) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign21970_e16961)))))))))));
        locals.var_tmf2_rv = 0.0;

        let assign21980_e16976: f64 = (1.0 / 2.0);
        let assign21980_e16980: f64 = (1.0 / 3.0);
        let assign21980_e16984: f64 = (1.0 / 8.0);
        let assign21980_e16988: f64 = (1.0 / 30.0);
        let assign21980_e16992: f64 = (1.0 / 144.0);
        let assign21980_e16996: f64 = (1.0 / 840.0);
        let assign21980_e16997: f64 = (locals.var_tmf1 * assign21980_e16996);
        let assign21980_e16998: f64 = (assign21980_e16992 + assign21980_e16997);
        let assign21980_e16999: f64 = (locals.var_tmf1 * assign21980_e16998);
        let assign21980_e17000: f64 = (assign21980_e16988 + assign21980_e16999);
        let assign21980_e17001: f64 = (locals.var_tmf1 * assign21980_e17000);
        let assign21980_e17002: f64 = (assign21980_e16984 + assign21980_e17001);
        let assign21980_e17003: f64 = (locals.var_tmf1 * assign21980_e17002);
        let assign21980_e17004: f64 = (assign21980_e16980 + assign21980_e17003);
        let assign21980_e17005: f64 = (locals.var_tmf1 * assign21980_e17004);
        let assign21980_e17006: f64 = (assign21980_e16976 + assign21980_e17005);
        locals.var_tmf3 = assign21980_e17006;
        locals.var_tmf3_dn0 = ((locals.var_tmf1_dn0 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign21980_e16996)))))))));
        locals.var_tmf3_dn2 = ((locals.var_tmf1_dn2 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign21980_e16996)))))))));
        locals.var_tmf3_dn4 = ((locals.var_tmf1_dn4 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign21980_e16996)))))))));
        locals.var_tmf3_dn5 = ((locals.var_tmf1_dn5 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign21980_e16996)))))))));
        locals.var_tmf3_dn6 = ((locals.var_tmf1_dn6 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign21980_e16996)))))))));
        locals.var_tmf3_dn7 = ((locals.var_tmf1_dn7 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign21980_e16996)))))))));
        locals.var_tmf3_dn8 = ((locals.var_tmf1_dn8 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign21980_e16996)))))))));
        locals.var_tmf3_dn9 = ((locals.var_tmf1_dn9 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign21980_e16996)))))))));
        locals.var_tmf3_dn10 = ((locals.var_tmf1_dn10 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign21980_e16996)))))))));
        locals.var_tmf3_dn11 = ((locals.var_tmf1_dn11 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign21980_e16996)))))))));
        locals.var_tmf3_dn14 = ((locals.var_tmf1_dn14 * assign21980_e17004) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign21980_e17002) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign21980_e17000) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign21980_e16998) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign21980_e16996)))))))));
        locals.var_tmf3_rv = 0.0;

        let assign21990_e17009: f64 = (p.p262 / locals.var_tmf2);
        locals.var_vzadd = assign21990_e17009;
        locals.var_vzadd_dn0 = (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn2 = (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn4 = (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn5 = (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn6 = (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn7 = (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn8 = (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn9 = (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn10 = (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn11 = (-((p.p262 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_dn14 = (-((p.p262 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd_rv = 0.0;

        let assign22000_e17011: f64 = (-2.0);
        let assign22000_e17013: f64 = (assign22000_e17011 * locals.var_tmf3);
        let assign22000_e17016: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign22000_e17017: f64 = (assign22000_e17013 / assign22000_e17016);
        locals.var_t2 = assign22000_e17017;
        locals.var_t2_dn0 = ((((assign22000_e17011 * locals.var_tmf3_dn0) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_dn2 = ((((assign22000_e17011 * locals.var_tmf3_dn2) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_dn4 = ((((assign22000_e17011 * locals.var_tmf3_dn4) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_dn5 = ((((assign22000_e17011 * locals.var_tmf3_dn5) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_dn6 = ((((assign22000_e17011 * locals.var_tmf3_dn6) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_dn7 = ((((assign22000_e17011 * locals.var_tmf3_dn7) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_dn8 = ((((assign22000_e17011 * locals.var_tmf3_dn8) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_dn9 = ((((assign22000_e17011 * locals.var_tmf3_dn9) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_dn10 = ((((assign22000_e17011 * locals.var_tmf3_dn10) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_dn11 = ((((assign22000_e17011 * locals.var_tmf3_dn11) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_dn14 = ((((assign22000_e17011 * locals.var_tmf3_dn14) * assign22000_e17016) - (assign22000_e17013 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign22000_e17016 * assign22000_e17016));
        locals.var_t2_rv = 0.0;

        let assign22010_e17020: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard425 = assign22010_e17020;
        locals.var_guard425_rv = 0.0;

        let (assign22020_e17024, assign22020_e17024_d_n0, assign22020_e17024_d_n2, assign22020_e17024_d_n4, assign22020_e17024_d_n5, assign22020_e17024_d_n6, assign22020_e17024_d_n7, assign22020_e17024_d_n8, assign22020_e17024_d_n9, assign22020_e17024_d_n10, assign22020_e17024_d_n11, assign22020_e17024_d_n14,) = {
    if (locals.var_guard425 != 0.0) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd, locals.var_vzadd_dn0, locals.var_vzadd_dn2, locals.var_vzadd_dn4, locals.var_vzadd_dn5, locals.var_vzadd_dn6, locals.var_vzadd_dn7, locals.var_vzadd_dn8, locals.var_vzadd_dn9, locals.var_vzadd_dn10, locals.var_vzadd_dn11, locals.var_vzadd_dn14,)
    }
};
        locals.var_vzadd = assign22020_e17024;
        locals.var_vzadd_dn0 = assign22020_e17024_d_n0;
        locals.var_vzadd_dn2 = assign22020_e17024_d_n2;
        locals.var_vzadd_dn4 = assign22020_e17024_d_n4;
        locals.var_vzadd_dn5 = assign22020_e17024_d_n5;
        locals.var_vzadd_dn6 = assign22020_e17024_d_n6;
        locals.var_vzadd_dn7 = assign22020_e17024_d_n7;
        locals.var_vzadd_dn8 = assign22020_e17024_d_n8;
        locals.var_vzadd_dn9 = assign22020_e17024_d_n9;
        locals.var_vzadd_dn10 = assign22020_e17024_d_n10;
        locals.var_vzadd_dn11 = assign22020_e17024_d_n11;
        locals.var_vzadd_dn14 = assign22020_e17024_d_n14;
        locals.var_vzadd_rv = 0.0;

        let assign22030_e17027: f64 = (locals.var_vbscl + locals.var_vzadd);
        locals.var_vbsz = assign22030_e17027;
        locals.var_vbsz_dn0 = (locals.var_vbscl_dn0 + locals.var_vzadd_dn0);
        locals.var_vbsz_dn2 = (locals.var_vbscl_dn2 + locals.var_vzadd_dn2);
        locals.var_vbsz_dn4 = (locals.var_vbscl_dn4 + locals.var_vzadd_dn4);
        locals.var_vbsz_dn5 = (locals.var_vbscl_dn5 + locals.var_vzadd_dn5);
        locals.var_vbsz_dn6 = (locals.var_vbscl_dn6 + locals.var_vzadd_dn6);
        locals.var_vbsz_dn7 = (locals.var_vbscl_dn7 + locals.var_vzadd_dn7);
        locals.var_vbsz_dn8 = (locals.var_vbscl_dn8 + locals.var_vzadd_dn8);
        locals.var_vbsz_dn9 = (locals.var_vbscl_dn9 + locals.var_vzadd_dn9);
        locals.var_vbsz_dn10 = (locals.var_vbscl_dn10 + locals.var_vzadd_dn10);
        locals.var_vbsz_dn11 = (locals.var_vbscl_dn11 + locals.var_vzadd_dn11);
        locals.var_vbsz_dn14 = (locals.var_vbscl_dn14 + locals.var_vzadd_dn14);
        locals.var_vbsz_rv = 0.0;

        let assign22040_e17031: f64 = (2.0 * locals.var_vzadd);
        let assign22040_e17032: f64 = (locals.var_vds + assign22040_e17031);
        locals.var_vdsz = assign22040_e17032;
        locals.var_vdsz_dn0 = (locals.var_vds_dn0 + (2.0 * locals.var_vzadd_dn0));
        locals.var_vdsz_dn2 = (locals.var_vds_dn2 + (2.0 * locals.var_vzadd_dn2));
        locals.var_vdsz_dn4 = (locals.var_vds_dn4 + (2.0 * locals.var_vzadd_dn4));
        locals.var_vdsz_dn5 = (locals.var_vds_dn5 + (2.0 * locals.var_vzadd_dn5));
        locals.var_vdsz_dn6 = (locals.var_vds_dn6 + (2.0 * locals.var_vzadd_dn6));
        locals.var_vdsz_dn7 = (locals.var_vds_dn7 + (2.0 * locals.var_vzadd_dn7));
        locals.var_vdsz_dn8 = (locals.var_vds_dn8 + (2.0 * locals.var_vzadd_dn8));
        locals.var_vdsz_dn9 = (locals.var_vds_dn9 + (2.0 * locals.var_vzadd_dn9));
        locals.var_vdsz_dn10 = (locals.var_vds_dn10 + (2.0 * locals.var_vzadd_dn10));
        locals.var_vdsz_dn11 = (locals.var_vds_dn11 + (2.0 * locals.var_vzadd_dn11));
        locals.var_vdsz_dn14 = (locals.var_vds_dn14 + (2.0 * locals.var_vzadd_dn14));
        locals.var_vdsz_rv = 0.0;

        let assign22050_e17035: f64 = (locals.var_vgs + locals.var_vzadd);
        locals.var_vgsz = assign22050_e17035;
        locals.var_vgsz_dn0 = locals.var_vzadd_dn0;
        locals.var_vgsz_dn2 = locals.var_vzadd_dn2;
        locals.var_vgsz_dn4 = locals.var_vzadd_dn4;
        locals.var_vgsz_dn5 = locals.var_vzadd_dn5;
        locals.var_vgsz_dn6 = (locals.var_vgs_dn6 + locals.var_vzadd_dn6);
        locals.var_vgsz_dn7 = (locals.var_vgs_dn7 + locals.var_vzadd_dn7);
        locals.var_vgsz_dn8 = (locals.var_vgs_dn8 + locals.var_vzadd_dn8);
        locals.var_vgsz_dn9 = locals.var_vzadd_dn9;
        locals.var_vgsz_dn10 = locals.var_vzadd_dn10;
        locals.var_vgsz_dn11 = locals.var_vzadd_dn11;
        locals.var_vgsz_dn14 = locals.var_vzadd_dn14;
        locals.var_vgsz_rv = 0.0;

        let assign22060_e17038: f64 = (locals.var_qnsub_esi * locals.var_cox0_inv);
        let assign22060_e17040: f64 = (assign22060_e17038 * locals.var_cox0_inv);
        locals.var_t1 = assign22060_e17040;
        locals.var_t1_dn0 = ((locals.var_qnsub_esi_dn0 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn2 = ((locals.var_qnsub_esi_dn2 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn4 = ((locals.var_qnsub_esi_dn4 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn5 = ((locals.var_qnsub_esi_dn5 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn6 = ((locals.var_qnsub_esi_dn6 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn7 = ((locals.var_qnsub_esi_dn7 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn8 = ((locals.var_qnsub_esi_dn8 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn9 = ((locals.var_qnsub_esi_dn9 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn10 = ((locals.var_qnsub_esi_dn10 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn11 = ((locals.var_qnsub_esi_dn11 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_dn14 = ((locals.var_qnsub_esi_dn14 * locals.var_cox0_inv) * locals.var_cox0_inv);
        locals.var_t1_rv = 0.0;

        let assign22070_e17043: f64 = (locals.var_vgs - locals.var_vfb);
        locals.var_t2 = assign22070_e17043;
        locals.var_t2_dn0 = 0.0;
        locals.var_t2_dn2 = 0.0;
        locals.var_t2_dn4 = 0.0;
        locals.var_t2_dn5 = 0.0;
        locals.var_t2_dn6 = locals.var_vgs_dn6;
        locals.var_t2_dn7 = locals.var_vgs_dn7;
        locals.var_t2_dn8 = locals.var_vgs_dn8;
        locals.var_t2_dn9 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn11 = 0.0;
        locals.var_t2_dn14 = 0.0;
        locals.var_t2_rv = 0.0;

        let assign22080_e17047: f64 = (2.0 / locals.var_t1);
        let assign22080_e17051: f64 = (1.0 / locals.var_betatnom);
        let assign22080_e17052: f64 = (locals.var_t2 - assign22080_e17051);
        let assign22080_e17054: f64 = (assign22080_e17052 - locals.var_vbscl);
        let assign22080_e17055: f64 = (assign22080_e17047 * assign22080_e17054);
        let assign22080_e17056: f64 = (1.0 + assign22080_e17055);
        locals.var_t3 = assign22080_e17056;
        locals.var_t3_dn0 = (((-((2.0 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn0 - locals.var_vbscl_dn0)));
        locals.var_t3_dn2 = (((-((2.0 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn2 - locals.var_vbscl_dn2)));
        locals.var_t3_dn4 = (((-((2.0 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn4 - locals.var_vbscl_dn4)));
        locals.var_t3_dn5 = (((-((2.0 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn5 - locals.var_vbscl_dn5)));
        locals.var_t3_dn6 = (((-((2.0 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn6 - locals.var_vbscl_dn6)));
        locals.var_t3_dn7 = (((-((2.0 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn7 - locals.var_vbscl_dn7)));
        locals.var_t3_dn8 = (((-((2.0 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn8 - locals.var_vbscl_dn8)));
        locals.var_t3_dn9 = (((-((2.0 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn9 - locals.var_vbscl_dn9)));
        locals.var_t3_dn10 = (((-((2.0 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn10 - locals.var_vbscl_dn10)));
        locals.var_t3_dn11 = (((-((2.0 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn11 - locals.var_vbscl_dn11)));
        locals.var_t3_dn14 = (((-((2.0 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))) * assign22080_e17054) + (assign22080_e17047 * (locals.var_t2_dn14 - locals.var_vbscl_dn14)));
        locals.var_t3_rv = 0.0;

        let assign22090_e17059: f64 = (locals.var_t3 * locals.var_t3);
        let assign22090_e17062: f64 = (4.0 * 0.001);
        let assign22090_e17064: f64 = (assign22090_e17062 * 0.001);
        let assign22090_e17065: f64 = (assign22090_e17059 + assign22090_e17064);
        let assign22090_e17066: f64 = (assign22090_e17065).sqrt();
        locals.var_tmf2 = assign22090_e17066;
        locals.var_tmf2_dn0 = (((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_dn2 = (((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_dn4 = (((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_dn5 = (((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_dn6 = (((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_dn7 = (((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_dn8 = (((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_dn9 = (((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_dn10 = (((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_dn11 = (((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_dn14 = (((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14)) / (2.0 * assign22090_e17066));
        locals.var_tmf2_rv = 0.0;

        let assign22100_e17071: f64 = (locals.var_t3 / locals.var_tmf2);
        let assign22100_e17072: f64 = (1.0 + assign22100_e17071);
        let assign22100_e17073: f64 = (0.5 * assign22100_e17072);
        locals.var_t5 = assign22100_e17073;
        locals.var_t5_dn0 = (0.5 * (((locals.var_t3_dn0 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn2 = (0.5 * (((locals.var_t3_dn2 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn4 = (0.5 * (((locals.var_t3_dn4 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn5 = (0.5 * (((locals.var_t3_dn5 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn6 = (0.5 * (((locals.var_t3_dn6 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn7 = (0.5 * (((locals.var_t3_dn7 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn8 = (0.5 * (((locals.var_t3_dn8 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn9 = (0.5 * (((locals.var_t3_dn9 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn10 = (0.5 * (((locals.var_t3_dn10 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn11 = (0.5 * (((locals.var_t3_dn11 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_dn14 = (0.5 * (((locals.var_t3_dn14 * locals.var_tmf2) - (locals.var_t3 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t5_rv = 0.0;

        let assign22110_e17077: f64 = (locals.var_t3 + locals.var_tmf2);
        let assign22110_e17078: f64 = (0.5 * assign22110_e17077);
        locals.var_t4 = assign22110_e17078;
        locals.var_t4_dn0 = (0.5 * (locals.var_t3_dn0 + locals.var_tmf2_dn0));
        locals.var_t4_dn2 = (0.5 * (locals.var_t3_dn2 + locals.var_tmf2_dn2));
        locals.var_t4_dn4 = (0.5 * (locals.var_t3_dn4 + locals.var_tmf2_dn4));
        locals.var_t4_dn5 = (0.5 * (locals.var_t3_dn5 + locals.var_tmf2_dn5));
        locals.var_t4_dn6 = (0.5 * (locals.var_t3_dn6 + locals.var_tmf2_dn6));
        locals.var_t4_dn7 = (0.5 * (locals.var_t3_dn7 + locals.var_tmf2_dn7));
        locals.var_t4_dn8 = (0.5 * (locals.var_t3_dn8 + locals.var_tmf2_dn8));
        locals.var_t4_dn9 = (0.5 * (locals.var_t3_dn9 + locals.var_tmf2_dn9));
        locals.var_t4_dn10 = (0.5 * (locals.var_t3_dn10 + locals.var_tmf2_dn10));
        locals.var_t4_dn11 = (0.5 * (locals.var_t3_dn11 + locals.var_tmf2_dn11));
        locals.var_t4_dn14 = (0.5 * (locals.var_t3_dn14 + locals.var_tmf2_dn14));
        locals.var_t4_rv = 0.0;

        let assign22120_e17081: f64 = if locals.var_t4 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard426 = assign22120_e17081;
        locals.var_guard426_rv = 0.0;

        let (assign22130_e17085, assign22130_e17085_d_n0, assign22130_e17085_d_n2, assign22130_e17085_d_n4, assign22130_e17085_d_n5, assign22130_e17085_d_n6, assign22130_e17085_d_n7, assign22130_e17085_d_n8, assign22130_e17085_d_n9, assign22130_e17085_d_n10, assign22130_e17085_d_n11, assign22130_e17085_d_n14,) = {
    if (locals.var_guard426 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign22130_e17085;
        locals.var_t4_dn0 = assign22130_e17085_d_n0;
        locals.var_t4_dn2 = assign22130_e17085_d_n2;
        locals.var_t4_dn4 = assign22130_e17085_d_n4;
        locals.var_t4_dn5 = assign22130_e17085_d_n5;
        locals.var_t4_dn6 = assign22130_e17085_d_n6;
        locals.var_t4_dn7 = assign22130_e17085_d_n7;
        locals.var_t4_dn8 = assign22130_e17085_d_n8;
        locals.var_t4_dn9 = assign22130_e17085_d_n9;
        locals.var_t4_dn10 = assign22130_e17085_d_n10;
        locals.var_t4_dn11 = assign22130_e17085_d_n11;
        locals.var_t4_dn14 = assign22130_e17085_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign22140_e17089, assign22140_e17089_d_n0, assign22140_e17089_d_n2, assign22140_e17089_d_n4, assign22140_e17089_d_n5, assign22140_e17089_d_n6, assign22140_e17089_d_n7, assign22140_e17089_d_n8, assign22140_e17089_d_n9, assign22140_e17089_d_n10, assign22140_e17089_d_n11, assign22140_e17089_d_n14,) = {
    if (locals.var_guard426 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign22140_e17089;
        locals.var_t5_dn0 = assign22140_e17089_d_n0;
        locals.var_t5_dn2 = assign22140_e17089_d_n2;
        locals.var_t5_dn4 = assign22140_e17089_d_n4;
        locals.var_t5_dn5 = assign22140_e17089_d_n5;
        locals.var_t5_dn6 = assign22140_e17089_d_n6;
        locals.var_t5_dn7 = assign22140_e17089_d_n7;
        locals.var_t5_dn8 = assign22140_e17089_d_n8;
        locals.var_t5_dn9 = assign22140_e17089_d_n9;
        locals.var_t5_dn10 = assign22140_e17089_d_n10;
        locals.var_t5_dn11 = assign22140_e17089_d_n11;
        locals.var_t5_dn14 = assign22140_e17089_d_n14;
        locals.var_t5_rv = 0.0;

        let assign22150_e17092: f64 = (locals.var_t4 + 1e-25);
        locals.var_t4 = assign22150_e17092;
        locals.var_t4_dn0 = locals.var_t4_dn0;
        locals.var_t4_dn2 = locals.var_t4_dn2;
        locals.var_t4_dn4 = locals.var_t4_dn4;
        locals.var_t4_dn5 = locals.var_t4_dn5;
        locals.var_t4_dn6 = locals.var_t4_dn6;
        locals.var_t4_dn7 = locals.var_t4_dn7;
        locals.var_t4_dn8 = locals.var_t4_dn8;
        locals.var_t4_dn9 = locals.var_t4_dn9;
        locals.var_t4_dn10 = locals.var_t4_dn10;
        locals.var_t4_dn11 = locals.var_t4_dn11;
        locals.var_t4_dn14 = locals.var_t4_dn14;
        locals.var_t4_rv = 0.0;

    }
}

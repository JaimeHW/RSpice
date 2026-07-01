#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign5810_e7165: f64 = (locals.var_pparam_b4soipdibl1 * locals.var_t2);
        let assign5810_e7167: f64 = (assign5810_e7165 + locals.var_pparam_b4soipdibl2);
        locals.var_pparam_b4soithetarout = assign5810_e7167;
        locals.var_pparam_b4soithetarout_dn3 = (((locals.var_pparam_b4soipdibl1_dn3 * locals.var_t2) + (locals.var_pparam_b4soipdibl1 * locals.var_t2_dn3)) + locals.var_pparam_b4soipdibl2_dn3);
        locals.var_pparam_b4soithetarout_dn4 = (((locals.var_pparam_b4soipdibl1_dn4 * locals.var_t2) + (locals.var_pparam_b4soipdibl1 * locals.var_t2_dn4)) + locals.var_pparam_b4soipdibl2_dn4);
        locals.var_pparam_b4soithetarout_dn5 = (((locals.var_pparam_b4soipdibl1_dn5 * locals.var_t2) + (locals.var_pparam_b4soipdibl1 * locals.var_t2_dn5)) + locals.var_pparam_b4soipdibl2_dn5);
        locals.var_pparam_b4soithetarout_dn6 = (((locals.var_pparam_b4soipdibl1_dn6 * locals.var_t2) + (locals.var_pparam_b4soipdibl1 * locals.var_t2_dn6)) + locals.var_pparam_b4soipdibl2_dn6);
        locals.var_pparam_b4soithetarout_dn7 = (((locals.var_pparam_b4soipdibl1_dn7 * locals.var_t2) + (locals.var_pparam_b4soipdibl1 * locals.var_t2_dn7)) + locals.var_pparam_b4soipdibl2_dn7);
        locals.var_pparam_b4soithetarout_dn8 = (((locals.var_pparam_b4soipdibl1_dn8 * locals.var_t2) + (locals.var_pparam_b4soipdibl1 * locals.var_t2_dn8)) + locals.var_pparam_b4soipdibl2_dn8);
        locals.var_pparam_b4soithetarout_dn9 = (((locals.var_pparam_b4soipdibl1_dn9 * locals.var_t2) + (locals.var_pparam_b4soipdibl1 * locals.var_t2_dn9)) + locals.var_pparam_b4soipdibl2_dn9);
        locals.var_pparam_b4soithetarout_dn10 = (((locals.var_pparam_b4soipdibl1_dn10 * locals.var_t2) + (locals.var_pparam_b4soipdibl1 * locals.var_t2_dn10)) + locals.var_pparam_b4soipdibl2_dn10);
        locals.var_pparam_b4soithetarout_dn11 = (((locals.var_pparam_b4soipdibl1_dn11 * locals.var_t2) + (locals.var_pparam_b4soipdibl1 * locals.var_t2_dn11)) + locals.var_pparam_b4soipdibl2_dn11);
        locals.var_pparam_b4soithetarout_dn12 = (((locals.var_pparam_b4soipdibl1_dn12 * locals.var_t2) + (locals.var_pparam_b4soipdibl1 * locals.var_t2_dn12)) + locals.var_pparam_b4soipdibl2_dn12);

        let (assign5820_e7177, assign5820_e7177_d_n3, assign5820_e7177_d_n4, assign5820_e7177_d_n5, assign5820_e7177_d_n6, assign5820_e7177_d_n7, assign5820_e7177_d_n8, assign5820_e7177_d_n9, assign5820_e7177_d_n10, assign5820_e7177_d_n11, assign5820_e7177_d_n12,) = {
    if (locals.var_pparam_b4soileff > 1e-38) {
        let assign5820_e7174: f64 = (locals.var_pparam_b4soileff).ln();
        (assign5820_e7174, (locals.var_pparam_b4soileff_dn3 / locals.var_pparam_b4soileff), (locals.var_pparam_b4soileff_dn4 / locals.var_pparam_b4soileff), (locals.var_pparam_b4soileff_dn5 / locals.var_pparam_b4soileff), (locals.var_pparam_b4soileff_dn6 / locals.var_pparam_b4soileff), (locals.var_pparam_b4soileff_dn7 / locals.var_pparam_b4soileff), (locals.var_pparam_b4soileff_dn8 / locals.var_pparam_b4soileff), (locals.var_pparam_b4soileff_dn9 / locals.var_pparam_b4soileff), (locals.var_pparam_b4soileff_dn10 / locals.var_pparam_b4soileff), (locals.var_pparam_b4soileff_dn11 / locals.var_pparam_b4soileff), (locals.var_pparam_b4soileff_dn12 / locals.var_pparam_b4soileff),)
    } else {
        let assign5820_e7176: f64 = (-87.49823353377374);
        (assign5820_e7176, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let assign5820_e7178: f64 = (locals.var_pparam_b4soidvtp3 * assign5820_e7177);
        let assign5820_e7179: f64 = (assign5820_e7178).exp();
        let assign5820_e7180: f64 = (locals.var_pparam_b4soidvtp2 / assign5820_e7179);
        locals.var_pparam_b4soidvtp2factor = assign5820_e7180;
        locals.var_pparam_b4soidvtp2factor_dn3 = (((locals.var_pparam_b4soidvtp2_dn3 * assign5820_e7179) - (locals.var_pparam_b4soidvtp2 * (assign5820_e7179 * ((locals.var_pparam_b4soidvtp3_dn3 * assign5820_e7177) + (locals.var_pparam_b4soidvtp3 * assign5820_e7177_d_n3))))) / (assign5820_e7179 * assign5820_e7179));
        locals.var_pparam_b4soidvtp2factor_dn4 = (((locals.var_pparam_b4soidvtp2_dn4 * assign5820_e7179) - (locals.var_pparam_b4soidvtp2 * (assign5820_e7179 * ((locals.var_pparam_b4soidvtp3_dn4 * assign5820_e7177) + (locals.var_pparam_b4soidvtp3 * assign5820_e7177_d_n4))))) / (assign5820_e7179 * assign5820_e7179));
        locals.var_pparam_b4soidvtp2factor_dn5 = (((locals.var_pparam_b4soidvtp2_dn5 * assign5820_e7179) - (locals.var_pparam_b4soidvtp2 * (assign5820_e7179 * ((locals.var_pparam_b4soidvtp3_dn5 * assign5820_e7177) + (locals.var_pparam_b4soidvtp3 * assign5820_e7177_d_n5))))) / (assign5820_e7179 * assign5820_e7179));
        locals.var_pparam_b4soidvtp2factor_dn6 = (((locals.var_pparam_b4soidvtp2_dn6 * assign5820_e7179) - (locals.var_pparam_b4soidvtp2 * (assign5820_e7179 * ((locals.var_pparam_b4soidvtp3_dn6 * assign5820_e7177) + (locals.var_pparam_b4soidvtp3 * assign5820_e7177_d_n6))))) / (assign5820_e7179 * assign5820_e7179));
        locals.var_pparam_b4soidvtp2factor_dn7 = (((locals.var_pparam_b4soidvtp2_dn7 * assign5820_e7179) - (locals.var_pparam_b4soidvtp2 * (assign5820_e7179 * ((locals.var_pparam_b4soidvtp3_dn7 * assign5820_e7177) + (locals.var_pparam_b4soidvtp3 * assign5820_e7177_d_n7))))) / (assign5820_e7179 * assign5820_e7179));
        locals.var_pparam_b4soidvtp2factor_dn8 = (((locals.var_pparam_b4soidvtp2_dn8 * assign5820_e7179) - (locals.var_pparam_b4soidvtp2 * (assign5820_e7179 * ((locals.var_pparam_b4soidvtp3_dn8 * assign5820_e7177) + (locals.var_pparam_b4soidvtp3 * assign5820_e7177_d_n8))))) / (assign5820_e7179 * assign5820_e7179));
        locals.var_pparam_b4soidvtp2factor_dn9 = (((locals.var_pparam_b4soidvtp2_dn9 * assign5820_e7179) - (locals.var_pparam_b4soidvtp2 * (assign5820_e7179 * ((locals.var_pparam_b4soidvtp3_dn9 * assign5820_e7177) + (locals.var_pparam_b4soidvtp3 * assign5820_e7177_d_n9))))) / (assign5820_e7179 * assign5820_e7179));
        locals.var_pparam_b4soidvtp2factor_dn10 = (((locals.var_pparam_b4soidvtp2_dn10 * assign5820_e7179) - (locals.var_pparam_b4soidvtp2 * (assign5820_e7179 * ((locals.var_pparam_b4soidvtp3_dn10 * assign5820_e7177) + (locals.var_pparam_b4soidvtp3 * assign5820_e7177_d_n10))))) / (assign5820_e7179 * assign5820_e7179));
        locals.var_pparam_b4soidvtp2factor_dn11 = (((locals.var_pparam_b4soidvtp2_dn11 * assign5820_e7179) - (locals.var_pparam_b4soidvtp2 * (assign5820_e7179 * ((locals.var_pparam_b4soidvtp3_dn11 * assign5820_e7177) + (locals.var_pparam_b4soidvtp3 * assign5820_e7177_d_n11))))) / (assign5820_e7179 * assign5820_e7179));
        locals.var_pparam_b4soidvtp2factor_dn12 = (((locals.var_pparam_b4soidvtp2_dn12 * assign5820_e7179) - (locals.var_pparam_b4soidvtp2 * (assign5820_e7179 * ((locals.var_pparam_b4soidvtp3_dn12 * assign5820_e7177) + (locals.var_pparam_b4soidvtp3 * assign5820_e7177_d_n12))))) / (assign5820_e7179 * assign5820_e7179));

        let assign5830_e7183: f64 = if locals.var_b4soiwlod < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard577 = assign5830_e7183;

        let (assign5840_e7187,) = {
    if (locals.var_guard577 != 0.0) {
        (0.0,)
    } else {
        (locals.var_b4soiwlod,)
    }
};
        locals.var_b4soiwlod = assign5840_e7187;

        let assign5850_e7190: f64 = (locals.var_ldrn).powf(p.p239);
        locals.var_t0 = assign5850_e7190;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;

        let assign5860_e7193: f64 = (locals.var_wdrn + locals.var_b4soiwlod);
        locals.var_w_tmp = assign5860_e7193;

        let assign5870_e7196: f64 = (locals.var_w_tmp).powf(p.p240);
        locals.var_t1 = assign5870_e7196;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;

        let assign5880_e7199: f64 = (p.p243 / locals.var_t0);
        let assign5880_e7202: f64 = (p.p244 / locals.var_t1);
        let assign5880_e7203: f64 = (assign5880_e7199 + assign5880_e7202);
        let assign5880_e7207: f64 = (locals.var_t0 * locals.var_t1);
        let assign5880_e7208: f64 = (p.p245 / assign5880_e7207);
        let assign5880_e7209: f64 = (assign5880_e7203 + assign5880_e7208);
        locals.var_tmp1 = assign5880_e7209;
        locals.var_tmp1_dn3 = (((-((p.p243 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))) + (-((p.p244 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1)))) + (-((p.p245 * ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3))) / (assign5880_e7207 * assign5880_e7207))));
        locals.var_tmp1_dn4 = (((-((p.p243 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) + (-((p.p244 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)))) + (-((p.p245 * ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4))) / (assign5880_e7207 * assign5880_e7207))));
        locals.var_tmp1_dn5 = (((-((p.p243 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) + (-((p.p244 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)))) + (-((p.p245 * ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5))) / (assign5880_e7207 * assign5880_e7207))));
        locals.var_tmp1_dn6 = (((-((p.p243 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) + (-((p.p244 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)))) + (-((p.p245 * ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6))) / (assign5880_e7207 * assign5880_e7207))));
        locals.var_tmp1_dn7 = (((-((p.p243 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) + (-((p.p244 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)))) + (-((p.p245 * ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7))) / (assign5880_e7207 * assign5880_e7207))));
        locals.var_tmp1_dn8 = (((-((p.p243 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) + (-((p.p244 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)))) + (-((p.p245 * ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8))) / (assign5880_e7207 * assign5880_e7207))));
        locals.var_tmp1_dn9 = (((-((p.p243 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) + (-((p.p244 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)))) + (-((p.p245 * ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9))) / (assign5880_e7207 * assign5880_e7207))));
        locals.var_tmp1_dn10 = (((-((p.p243 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) + (-((p.p244 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)))) + (-((p.p245 * ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10))) / (assign5880_e7207 * assign5880_e7207))));
        locals.var_tmp1_dn11 = (((-((p.p243 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) + (-((p.p244 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1)))) + (-((p.p245 * ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11))) / (assign5880_e7207 * assign5880_e7207))));
        locals.var_tmp1_dn12 = (((-((p.p243 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))) + (-((p.p244 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1)))) + (-((p.p245 * ((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12))) / (assign5880_e7207 * assign5880_e7207))));

        let assign5890_e7212: f64 = (1.0 + locals.var_tmp1);
        locals.var_pparam_b4soiku0 = assign5890_e7212;
        locals.var_pparam_b4soiku0_dn3 = locals.var_tmp1_dn3;
        locals.var_pparam_b4soiku0_dn4 = locals.var_tmp1_dn4;
        locals.var_pparam_b4soiku0_dn5 = locals.var_tmp1_dn5;
        locals.var_pparam_b4soiku0_dn6 = locals.var_tmp1_dn6;
        locals.var_pparam_b4soiku0_dn7 = locals.var_tmp1_dn7;
        locals.var_pparam_b4soiku0_dn8 = locals.var_tmp1_dn8;
        locals.var_pparam_b4soiku0_dn9 = locals.var_tmp1_dn9;
        locals.var_pparam_b4soiku0_dn10 = locals.var_tmp1_dn10;
        locals.var_pparam_b4soiku0_dn11 = locals.var_tmp1_dn11;
        locals.var_pparam_b4soiku0_dn12 = locals.var_tmp1_dn12;

        let assign5900_e7215: f64 = (locals.var_ldrn).powf(p.p241);
        locals.var_t0 = assign5900_e7215;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;

        let assign5910_e7218: f64 = (locals.var_w_tmp).powf(p.p242);
        locals.var_t1 = assign5910_e7218;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;

        let assign5920_e7221: f64 = (p.p246 / locals.var_t0);
        let assign5920_e7224: f64 = (p.p247 / locals.var_t1);
        let assign5920_e7225: f64 = (assign5920_e7221 + assign5920_e7224);
        let assign5920_e7229: f64 = (locals.var_t0 * locals.var_t1);
        let assign5920_e7230: f64 = (p.p248 / assign5920_e7229);
        let assign5920_e7231: f64 = (assign5920_e7225 + assign5920_e7230);
        locals.var_tmp1 = assign5920_e7231;
        locals.var_tmp1_dn3 = (((-((p.p246 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))) + (-((p.p247 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1)))) + (-((p.p248 * ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3))) / (assign5920_e7229 * assign5920_e7229))));
        locals.var_tmp1_dn4 = (((-((p.p246 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) + (-((p.p247 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)))) + (-((p.p248 * ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4))) / (assign5920_e7229 * assign5920_e7229))));
        locals.var_tmp1_dn5 = (((-((p.p246 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) + (-((p.p247 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)))) + (-((p.p248 * ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5))) / (assign5920_e7229 * assign5920_e7229))));
        locals.var_tmp1_dn6 = (((-((p.p246 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) + (-((p.p247 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)))) + (-((p.p248 * ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6))) / (assign5920_e7229 * assign5920_e7229))));
        locals.var_tmp1_dn7 = (((-((p.p246 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) + (-((p.p247 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)))) + (-((p.p248 * ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7))) / (assign5920_e7229 * assign5920_e7229))));
        locals.var_tmp1_dn8 = (((-((p.p246 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) + (-((p.p247 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)))) + (-((p.p248 * ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8))) / (assign5920_e7229 * assign5920_e7229))));
        locals.var_tmp1_dn9 = (((-((p.p246 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) + (-((p.p247 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)))) + (-((p.p248 * ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9))) / (assign5920_e7229 * assign5920_e7229))));
        locals.var_tmp1_dn10 = (((-((p.p246 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) + (-((p.p247 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)))) + (-((p.p248 * ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10))) / (assign5920_e7229 * assign5920_e7229))));
        locals.var_tmp1_dn11 = (((-((p.p246 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) + (-((p.p247 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1)))) + (-((p.p248 * ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11))) / (assign5920_e7229 * assign5920_e7229))));
        locals.var_tmp1_dn12 = (((-((p.p246 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))) + (-((p.p247 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1)))) + (-((p.p248 * ((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12))) / (assign5920_e7229 * assign5920_e7229))));

        let assign5930_e7234: f64 = (1.0 + locals.var_tmp1);
        locals.var_pparam_b4soikvth0 = assign5930_e7234;
        locals.var_pparam_b4soikvth0_dn3 = locals.var_tmp1_dn3;
        locals.var_pparam_b4soikvth0_dn4 = locals.var_tmp1_dn4;
        locals.var_pparam_b4soikvth0_dn5 = locals.var_tmp1_dn5;
        locals.var_pparam_b4soikvth0_dn6 = locals.var_tmp1_dn6;
        locals.var_pparam_b4soikvth0_dn7 = locals.var_tmp1_dn7;
        locals.var_pparam_b4soikvth0_dn8 = locals.var_tmp1_dn8;
        locals.var_pparam_b4soikvth0_dn9 = locals.var_tmp1_dn9;
        locals.var_pparam_b4soikvth0_dn10 = locals.var_tmp1_dn10;
        locals.var_pparam_b4soikvth0_dn11 = locals.var_tmp1_dn11;
        locals.var_pparam_b4soikvth0_dn12 = locals.var_tmp1_dn12;

        let assign5940_e7237: f64 = (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0);
        let assign5940_e7239: f64 = (assign5940_e7237 + 1e-9);
        let assign5940_e7240: f64 = (assign5940_e7239).sqrt();
        locals.var_pparam_b4soikvth0 = assign5940_e7240;
        locals.var_pparam_b4soikvth0_dn3 = (((locals.var_pparam_b4soikvth0_dn3 * locals.var_pparam_b4soikvth0) + (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0_dn3)) / (2.0 * assign5940_e7240));
        locals.var_pparam_b4soikvth0_dn4 = (((locals.var_pparam_b4soikvth0_dn4 * locals.var_pparam_b4soikvth0) + (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0_dn4)) / (2.0 * assign5940_e7240));
        locals.var_pparam_b4soikvth0_dn5 = (((locals.var_pparam_b4soikvth0_dn5 * locals.var_pparam_b4soikvth0) + (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0_dn5)) / (2.0 * assign5940_e7240));
        locals.var_pparam_b4soikvth0_dn6 = (((locals.var_pparam_b4soikvth0_dn6 * locals.var_pparam_b4soikvth0) + (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0_dn6)) / (2.0 * assign5940_e7240));
        locals.var_pparam_b4soikvth0_dn7 = (((locals.var_pparam_b4soikvth0_dn7 * locals.var_pparam_b4soikvth0) + (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0_dn7)) / (2.0 * assign5940_e7240));
        locals.var_pparam_b4soikvth0_dn8 = (((locals.var_pparam_b4soikvth0_dn8 * locals.var_pparam_b4soikvth0) + (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0_dn8)) / (2.0 * assign5940_e7240));
        locals.var_pparam_b4soikvth0_dn9 = (((locals.var_pparam_b4soikvth0_dn9 * locals.var_pparam_b4soikvth0) + (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0_dn9)) / (2.0 * assign5940_e7240));
        locals.var_pparam_b4soikvth0_dn10 = (((locals.var_pparam_b4soikvth0_dn10 * locals.var_pparam_b4soikvth0) + (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0_dn10)) / (2.0 * assign5940_e7240));
        locals.var_pparam_b4soikvth0_dn11 = (((locals.var_pparam_b4soikvth0_dn11 * locals.var_pparam_b4soikvth0) + (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0_dn11)) / (2.0 * assign5940_e7240));
        locals.var_pparam_b4soikvth0_dn12 = (((locals.var_pparam_b4soikvth0_dn12 * locals.var_pparam_b4soikvth0) + (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0_dn12)) / (2.0 * assign5940_e7240));

        let assign5950_e7245: f64 = (p.p238 * locals.var_trm1);
        let assign5950_e7246: f64 = (1.0 + assign5950_e7245);
        let assign5950_e7247: f64 = (locals.var_pparam_b4soiku0 * assign5950_e7246);
        let assign5950_e7249: f64 = (assign5950_e7247 + 1e-9);
        locals.var_pparam_b4soiku0temp = assign5950_e7249;
        locals.var_pparam_b4soiku0temp_dn3 = (locals.var_pparam_b4soiku0_dn3 * assign5950_e7246);
        locals.var_pparam_b4soiku0temp_dn4 = ((locals.var_pparam_b4soiku0_dn4 * assign5950_e7246) + (locals.var_pparam_b4soiku0 * (p.p238 * locals.var_trm1_dn4)));
        locals.var_pparam_b4soiku0temp_dn5 = ((locals.var_pparam_b4soiku0_dn5 * assign5950_e7246) + (locals.var_pparam_b4soiku0 * (p.p238 * locals.var_trm1_dn5)));
        locals.var_pparam_b4soiku0temp_dn6 = ((locals.var_pparam_b4soiku0_dn6 * assign5950_e7246) + (locals.var_pparam_b4soiku0 * (p.p238 * locals.var_trm1_dn6)));
        locals.var_pparam_b4soiku0temp_dn7 = (locals.var_pparam_b4soiku0_dn7 * assign5950_e7246);
        locals.var_pparam_b4soiku0temp_dn8 = (locals.var_pparam_b4soiku0_dn8 * assign5950_e7246);
        locals.var_pparam_b4soiku0temp_dn9 = (locals.var_pparam_b4soiku0_dn9 * assign5950_e7246);
        locals.var_pparam_b4soiku0temp_dn10 = (locals.var_pparam_b4soiku0_dn10 * assign5950_e7246);
        locals.var_pparam_b4soiku0temp_dn11 = (locals.var_pparam_b4soiku0_dn11 * assign5950_e7246);
        locals.var_pparam_b4soiku0temp_dn12 = (locals.var_pparam_b4soiku0_dn12 * assign5950_e7246);

        let assign5960_e7254: f64 = (0.5 * locals.var_ldrn);
        let assign5960_e7255: f64 = (p.p232 + assign5960_e7254);
        let assign5960_e7256: f64 = (1.0 / assign5960_e7255);
        locals.var_inv_saref = assign5960_e7256;

        let assign5970_e7261: f64 = (0.5 * locals.var_ldrn);
        let assign5970_e7262: f64 = (p.p233 + assign5970_e7261);
        let assign5970_e7263: f64 = (1.0 / assign5970_e7262);
        locals.var_inv_sbref = assign5970_e7263;

        let assign5980_e7266: f64 = (locals.var_inv_saref + locals.var_inv_sbref);
        locals.var_pparam_b4soiinv_od_ref = assign5980_e7266;

        let assign5990_e7269: f64 = (p.p235 / locals.var_pparam_b4soiku0temp);
        let assign5990_e7271: f64 = (assign5990_e7269 * locals.var_pparam_b4soiinv_od_ref);
        locals.var_pparam_b4soirho_ref = assign5990_e7271;
        locals.var_pparam_b4soirho_ref_dn3 = ((-((p.p235 * locals.var_pparam_b4soiku0temp_dn3) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_pparam_b4soiinv_od_ref);
        locals.var_pparam_b4soirho_ref_dn4 = ((-((p.p235 * locals.var_pparam_b4soiku0temp_dn4) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_pparam_b4soiinv_od_ref);
        locals.var_pparam_b4soirho_ref_dn5 = ((-((p.p235 * locals.var_pparam_b4soiku0temp_dn5) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_pparam_b4soiinv_od_ref);
        locals.var_pparam_b4soirho_ref_dn6 = ((-((p.p235 * locals.var_pparam_b4soiku0temp_dn6) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_pparam_b4soiinv_od_ref);
        locals.var_pparam_b4soirho_ref_dn7 = ((-((p.p235 * locals.var_pparam_b4soiku0temp_dn7) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_pparam_b4soiinv_od_ref);
        locals.var_pparam_b4soirho_ref_dn8 = ((-((p.p235 * locals.var_pparam_b4soiku0temp_dn8) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_pparam_b4soiinv_od_ref);
        locals.var_pparam_b4soirho_ref_dn9 = ((-((p.p235 * locals.var_pparam_b4soiku0temp_dn9) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_pparam_b4soiinv_od_ref);
        locals.var_pparam_b4soirho_ref_dn10 = ((-((p.p235 * locals.var_pparam_b4soiku0temp_dn10) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_pparam_b4soiinv_od_ref);
        locals.var_pparam_b4soirho_ref_dn11 = ((-((p.p235 * locals.var_pparam_b4soiku0temp_dn11) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_pparam_b4soiinv_od_ref);
        locals.var_pparam_b4soirho_ref_dn12 = ((-((p.p235 * locals.var_pparam_b4soiku0temp_dn12) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_pparam_b4soiinv_od_ref);

        let assign6000_e7290: f64 = if (((p.p4 > 0.0) && (p.p5 > 0.0)) && ((p.p3 == 1.0) || ((p.p3 > 1.0) && (p.p6 > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard578 = assign6000_e7290;

        let (assign6010_e7294,) = {
    if (locals.var_guard578 != 0.0) {
        (0.0,)
    } else {
        (locals.var_inv_sa,)
    }
};
        locals.var_inv_sa = assign6010_e7294;

        let (assign6020_e7298,) = {
    if (locals.var_guard578 != 0.0) {
        (0.0,)
    } else {
        (locals.var_inv_sb,)
    }
};
        locals.var_inv_sb = assign6020_e7298;

        let assign6030_e7301: f64 = (-1.0);
        let assign6030_e7302: f64 = if locals.var_b4soikvsat < assign6030_e7301 { 1.0 } else { 0.0 };
        locals.var_guard579 = assign6030_e7302;

        let (assign6040_e7309,) = {
    if ((locals.var_guard578 != 0.0) && (locals.var_guard579 != 0.0)) {
        let assign6040_e7307: f64 = (-1.0);
        (assign6040_e7307,)
    } else {
        (locals.var_b4soikvsat,)
    }
};
        locals.var_b4soikvsat = assign6040_e7309;

        let assign6050_e7312: f64 = if locals.var_b4soikvsat > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard580 = assign6050_e7312;

        let (assign6060_e7321,) = {
    if (((locals.var_guard578 != 0.0) && (locals.var_guard579 == 0.0)) && (locals.var_guard580 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_b4soikvsat,)
    }
};
        locals.var_b4soikvsat = assign6060_e7321;

        let (assign6070_e7331,) = {
    if (((locals.var_guard578 != 0.0) && (locals.var_guard579 == 0.0)) && (locals.var_guard580 == 0.0)) {
        (locals.var_b4soikvsat,)
    } else {
        (locals.var_b4soikvsat,)
    }
};
        locals.var_b4soikvsat = assign6070_e7331;

        let (assign6080_e7335,) = {
    if (locals.var_guard578 != 0.0) {
        (0.0,)
    } else {
        (locals.var_i,)
    }
};
        locals.var_i = assign6080_e7335;

        let mut assign6090_loop_guard: usize = 0;
        while {
            let assign6090_cond_e7340: f64 = if ((locals.var_guard578 != 0.0) && (locals.var_i < p.p3)) { 1.0 } else { 0.0 };
            assign6090_cond_e7340 != 0.0
        } {
            assign6090_loop_guard += 1;
            assert!(assign6090_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign6090_body0_e7358,) = {
    if (locals.var_guard578 != 0.0) {
        let assign6090_body0_e7344: f64 = (1.0 / p.p3);
        let assign6090_body0_e7348: f64 = (0.5 * locals.var_ldrn);
        let assign6090_body0_e7349: f64 = (p.p4 + assign6090_body0_e7348);
        let assign6090_body0_e7353: f64 = (p.p6 + locals.var_ldrn);
        let assign6090_body0_e7354: f64 = (locals.var_i * assign6090_body0_e7353);
        let assign6090_body0_e7355: f64 = (assign6090_body0_e7349 + assign6090_body0_e7354);
        let assign6090_body0_e7356: f64 = (assign6090_body0_e7344 / assign6090_body0_e7355);
        (assign6090_body0_e7356,)
    } else {
        (locals.var_t0__blk581,)
    }
};
            locals.var_t0__blk581 = assign6090_body0_e7358;
            let (assign6090_body1_e7376,) = {
    if (locals.var_guard578 != 0.0) {
        let assign6090_body1_e7362: f64 = (1.0 / p.p3);
        let assign6090_body1_e7366: f64 = (0.5 * locals.var_ldrn);
        let assign6090_body1_e7367: f64 = (p.p5 + assign6090_body1_e7366);
        let assign6090_body1_e7371: f64 = (p.p6 + locals.var_ldrn);
        let assign6090_body1_e7372: f64 = (locals.var_i * assign6090_body1_e7371);
        let assign6090_body1_e7373: f64 = (assign6090_body1_e7367 + assign6090_body1_e7372);
        let assign6090_body1_e7374: f64 = (assign6090_body1_e7362 / assign6090_body1_e7373);
        (assign6090_body1_e7374,)
    } else {
        (locals.var_t1__blk582,)
    }
};
            locals.var_t1__blk582 = assign6090_body1_e7376;
            let (assign6090_body2_e7382,) = {
    if (locals.var_guard578 != 0.0) {
        let assign6090_body2_e7380: f64 = (locals.var_inv_sa + locals.var_t0__blk581);
        (assign6090_body2_e7380,)
    } else {
        (locals.var_inv_sa,)
    }
};
            locals.var_inv_sa = assign6090_body2_e7382;
            let (assign6090_body3_e7388,) = {
    if (locals.var_guard578 != 0.0) {
        let assign6090_body3_e7386: f64 = (locals.var_inv_sb + locals.var_t1__blk582);
        (assign6090_body3_e7386,)
    } else {
        (locals.var_inv_sb,)
    }
};
            locals.var_inv_sb = assign6090_body3_e7388;
            let (assign6090_body4_e7394,) = {
    if (locals.var_guard578 != 0.0) {
        let assign6090_body4_e7392: f64 = (locals.var_i + 1.0);
        (assign6090_body4_e7392,)
    } else {
        (locals.var_i,)
    }
};
            locals.var_i = assign6090_body4_e7394;
        }

        let (assign6100_e7400,) = {
    if (locals.var_guard578 != 0.0) {
        let assign6100_e7398: f64 = (locals.var_inv_sa + locals.var_inv_sb);
        (assign6100_e7398,)
    } else {
        (locals.var_inv_odeff,)
    }
};
        locals.var_inv_odeff = assign6100_e7400;

        let (assign6110_e7404,) = {
    if (locals.var_guard578 != 0.0) {
        (locals.var_inv_odeff,)
    } else {
        (locals.var_b4soiinv_odeff,)
    }
};
        locals.var_b4soiinv_odeff = assign6110_e7404;

        let (assign6120_e7412, assign6120_e7412_d_n3, assign6120_e7412_d_n4, assign6120_e7412_d_n5, assign6120_e7412_d_n6, assign6120_e7412_d_n7, assign6120_e7412_d_n8, assign6120_e7412_d_n9, assign6120_e7412_d_n10, assign6120_e7412_d_n11, assign6120_e7412_d_n12,) = {
    if (locals.var_guard578 != 0.0) {
        let assign6120_e7408: f64 = (p.p235 / locals.var_pparam_b4soiku0temp);
        let assign6120_e7410: f64 = (assign6120_e7408 * locals.var_inv_odeff);
        (assign6120_e7410, ((-((p.p235 * locals.var_pparam_b4soiku0temp_dn3) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_inv_odeff), ((-((p.p235 * locals.var_pparam_b4soiku0temp_dn4) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_inv_odeff), ((-((p.p235 * locals.var_pparam_b4soiku0temp_dn5) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_inv_odeff), ((-((p.p235 * locals.var_pparam_b4soiku0temp_dn6) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_inv_odeff), ((-((p.p235 * locals.var_pparam_b4soiku0temp_dn7) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_inv_odeff), ((-((p.p235 * locals.var_pparam_b4soiku0temp_dn8) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_inv_odeff), ((-((p.p235 * locals.var_pparam_b4soiku0temp_dn9) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_inv_odeff), ((-((p.p235 * locals.var_pparam_b4soiku0temp_dn10) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_inv_odeff), ((-((p.p235 * locals.var_pparam_b4soiku0temp_dn11) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_inv_odeff), ((-((p.p235 * locals.var_pparam_b4soiku0temp_dn12) / (locals.var_pparam_b4soiku0temp * locals.var_pparam_b4soiku0temp))) * locals.var_inv_odeff),)
    } else {
        (locals.var_rho, locals.var_rho_dn3, locals.var_rho_dn4, locals.var_rho_dn5, locals.var_rho_dn6, locals.var_rho_dn7, locals.var_rho_dn8, locals.var_rho_dn9, locals.var_rho_dn10, locals.var_rho_dn11, locals.var_rho_dn12,)
    }
};
        locals.var_rho = assign6120_e7412;
        locals.var_rho_dn3 = assign6120_e7412_d_n3;
        locals.var_rho_dn4 = assign6120_e7412_d_n4;
        locals.var_rho_dn5 = assign6120_e7412_d_n5;
        locals.var_rho_dn6 = assign6120_e7412_d_n6;
        locals.var_rho_dn7 = assign6120_e7412_d_n7;
        locals.var_rho_dn8 = assign6120_e7412_d_n8;
        locals.var_rho_dn9 = assign6120_e7412_d_n9;
        locals.var_rho_dn10 = assign6120_e7412_d_n10;
        locals.var_rho_dn11 = assign6120_e7412_d_n11;
        locals.var_rho_dn12 = assign6120_e7412_d_n12;

        let (assign6130_e7422, assign6130_e7422_d_n3, assign6130_e7422_d_n4, assign6130_e7422_d_n5, assign6130_e7422_d_n6, assign6130_e7422_d_n7, assign6130_e7422_d_n8, assign6130_e7422_d_n9, assign6130_e7422_d_n10, assign6130_e7422_d_n11, assign6130_e7422_d_n12,) = {
    if (locals.var_guard578 != 0.0) {
        let assign6130_e7416: f64 = (1.0 + locals.var_rho);
        let assign6130_e7419: f64 = (1.0 + locals.var_pparam_b4soirho_ref);
        let assign6130_e7420: f64 = (assign6130_e7416 / assign6130_e7419);
        (assign6130_e7420, (((locals.var_rho_dn3 * assign6130_e7419) - (assign6130_e7416 * locals.var_pparam_b4soirho_ref_dn3)) / (assign6130_e7419 * assign6130_e7419)), (((locals.var_rho_dn4 * assign6130_e7419) - (assign6130_e7416 * locals.var_pparam_b4soirho_ref_dn4)) / (assign6130_e7419 * assign6130_e7419)), (((locals.var_rho_dn5 * assign6130_e7419) - (assign6130_e7416 * locals.var_pparam_b4soirho_ref_dn5)) / (assign6130_e7419 * assign6130_e7419)), (((locals.var_rho_dn6 * assign6130_e7419) - (assign6130_e7416 * locals.var_pparam_b4soirho_ref_dn6)) / (assign6130_e7419 * assign6130_e7419)), (((locals.var_rho_dn7 * assign6130_e7419) - (assign6130_e7416 * locals.var_pparam_b4soirho_ref_dn7)) / (assign6130_e7419 * assign6130_e7419)), (((locals.var_rho_dn8 * assign6130_e7419) - (assign6130_e7416 * locals.var_pparam_b4soirho_ref_dn8)) / (assign6130_e7419 * assign6130_e7419)), (((locals.var_rho_dn9 * assign6130_e7419) - (assign6130_e7416 * locals.var_pparam_b4soirho_ref_dn9)) / (assign6130_e7419 * assign6130_e7419)), (((locals.var_rho_dn10 * assign6130_e7419) - (assign6130_e7416 * locals.var_pparam_b4soirho_ref_dn10)) / (assign6130_e7419 * assign6130_e7419)), (((locals.var_rho_dn11 * assign6130_e7419) - (assign6130_e7416 * locals.var_pparam_b4soirho_ref_dn11)) / (assign6130_e7419 * assign6130_e7419)), (((locals.var_rho_dn12 * assign6130_e7419) - (assign6130_e7416 * locals.var_pparam_b4soirho_ref_dn12)) / (assign6130_e7419 * assign6130_e7419)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign6130_e7422;
        locals.var_t0_dn3 = assign6130_e7422_d_n3;
        locals.var_t0_dn4 = assign6130_e7422_d_n4;
        locals.var_t0_dn5 = assign6130_e7422_d_n5;
        locals.var_t0_dn6 = assign6130_e7422_d_n6;
        locals.var_t0_dn7 = assign6130_e7422_d_n7;
        locals.var_t0_dn8 = assign6130_e7422_d_n8;
        locals.var_t0_dn9 = assign6130_e7422_d_n9;
        locals.var_t0_dn10 = assign6130_e7422_d_n10;
        locals.var_t0_dn11 = assign6130_e7422_d_n11;
        locals.var_t0_dn12 = assign6130_e7422_d_n12;

        let (assign6140_e7428, assign6140_e7428_d_n3, assign6140_e7428_d_n4, assign6140_e7428_d_n5, assign6140_e7428_d_n6, assign6140_e7428_d_n7, assign6140_e7428_d_n8, assign6140_e7428_d_n9, assign6140_e7428_d_n10, assign6140_e7428_d_n11, assign6140_e7428_d_n12,) = {
    if (locals.var_guard578 != 0.0) {
        let assign6140_e7426: f64 = (locals.var_pparam_b4soiu0temp * locals.var_t0);
        (assign6140_e7426, ((locals.var_pparam_b4soiu0temp_dn3 * locals.var_t0) + (locals.var_pparam_b4soiu0temp * locals.var_t0_dn3)), ((locals.var_pparam_b4soiu0temp_dn4 * locals.var_t0) + (locals.var_pparam_b4soiu0temp * locals.var_t0_dn4)), ((locals.var_pparam_b4soiu0temp_dn5 * locals.var_t0) + (locals.var_pparam_b4soiu0temp * locals.var_t0_dn5)), ((locals.var_pparam_b4soiu0temp_dn6 * locals.var_t0) + (locals.var_pparam_b4soiu0temp * locals.var_t0_dn6)), ((locals.var_pparam_b4soiu0temp_dn7 * locals.var_t0) + (locals.var_pparam_b4soiu0temp * locals.var_t0_dn7)), ((locals.var_pparam_b4soiu0temp_dn8 * locals.var_t0) + (locals.var_pparam_b4soiu0temp * locals.var_t0_dn8)), ((locals.var_pparam_b4soiu0temp_dn9 * locals.var_t0) + (locals.var_pparam_b4soiu0temp * locals.var_t0_dn9)), ((locals.var_pparam_b4soiu0temp_dn10 * locals.var_t0) + (locals.var_pparam_b4soiu0temp * locals.var_t0_dn10)), ((locals.var_pparam_b4soiu0temp_dn11 * locals.var_t0) + (locals.var_pparam_b4soiu0temp * locals.var_t0_dn11)), ((locals.var_pparam_b4soiu0temp_dn12 * locals.var_t0) + (locals.var_pparam_b4soiu0temp * locals.var_t0_dn12)),)
    } else {
        (locals.var_here_b4soiu0temp, locals.var_here_b4soiu0temp_dn3, locals.var_here_b4soiu0temp_dn4, locals.var_here_b4soiu0temp_dn5, locals.var_here_b4soiu0temp_dn6, locals.var_here_b4soiu0temp_dn7, locals.var_here_b4soiu0temp_dn8, locals.var_here_b4soiu0temp_dn9, locals.var_here_b4soiu0temp_dn10, locals.var_here_b4soiu0temp_dn11, locals.var_here_b4soiu0temp_dn12,)
    }
};
        locals.var_here_b4soiu0temp = assign6140_e7428;
        locals.var_here_b4soiu0temp_dn3 = assign6140_e7428_d_n3;
        locals.var_here_b4soiu0temp_dn4 = assign6140_e7428_d_n4;
        locals.var_here_b4soiu0temp_dn5 = assign6140_e7428_d_n5;
        locals.var_here_b4soiu0temp_dn6 = assign6140_e7428_d_n6;
        locals.var_here_b4soiu0temp_dn7 = assign6140_e7428_d_n7;
        locals.var_here_b4soiu0temp_dn8 = assign6140_e7428_d_n8;
        locals.var_here_b4soiu0temp_dn9 = assign6140_e7428_d_n9;
        locals.var_here_b4soiu0temp_dn10 = assign6140_e7428_d_n10;
        locals.var_here_b4soiu0temp_dn11 = assign6140_e7428_d_n11;
        locals.var_here_b4soiu0temp_dn12 = assign6140_e7428_d_n12;

        let (assign6150_e7442, assign6150_e7442_d_n3, assign6150_e7442_d_n4, assign6150_e7442_d_n5, assign6150_e7442_d_n6, assign6150_e7442_d_n7, assign6150_e7442_d_n8, assign6150_e7442_d_n9, assign6150_e7442_d_n10, assign6150_e7442_d_n11, assign6150_e7442_d_n12,) = {
    if (locals.var_guard578 != 0.0) {
        let assign6150_e7433: f64 = (locals.var_b4soikvsat * locals.var_rho);
        let assign6150_e7434: f64 = (1.0 + assign6150_e7433);
        let assign6150_e7438: f64 = (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref);
        let assign6150_e7439: f64 = (1.0 + assign6150_e7438);
        let assign6150_e7440: f64 = (assign6150_e7434 / assign6150_e7439);
        (assign6150_e7440, ((((locals.var_b4soikvsat * locals.var_rho_dn3) * assign6150_e7439) - (assign6150_e7434 * (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref_dn3))) / (assign6150_e7439 * assign6150_e7439)), ((((locals.var_b4soikvsat * locals.var_rho_dn4) * assign6150_e7439) - (assign6150_e7434 * (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref_dn4))) / (assign6150_e7439 * assign6150_e7439)), ((((locals.var_b4soikvsat * locals.var_rho_dn5) * assign6150_e7439) - (assign6150_e7434 * (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref_dn5))) / (assign6150_e7439 * assign6150_e7439)), ((((locals.var_b4soikvsat * locals.var_rho_dn6) * assign6150_e7439) - (assign6150_e7434 * (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref_dn6))) / (assign6150_e7439 * assign6150_e7439)), ((((locals.var_b4soikvsat * locals.var_rho_dn7) * assign6150_e7439) - (assign6150_e7434 * (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref_dn7))) / (assign6150_e7439 * assign6150_e7439)), ((((locals.var_b4soikvsat * locals.var_rho_dn8) * assign6150_e7439) - (assign6150_e7434 * (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref_dn8))) / (assign6150_e7439 * assign6150_e7439)), ((((locals.var_b4soikvsat * locals.var_rho_dn9) * assign6150_e7439) - (assign6150_e7434 * (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref_dn9))) / (assign6150_e7439 * assign6150_e7439)), ((((locals.var_b4soikvsat * locals.var_rho_dn10) * assign6150_e7439) - (assign6150_e7434 * (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref_dn10))) / (assign6150_e7439 * assign6150_e7439)), ((((locals.var_b4soikvsat * locals.var_rho_dn11) * assign6150_e7439) - (assign6150_e7434 * (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref_dn11))) / (assign6150_e7439 * assign6150_e7439)), ((((locals.var_b4soikvsat * locals.var_rho_dn12) * assign6150_e7439) - (assign6150_e7434 * (locals.var_b4soikvsat * locals.var_pparam_b4soirho_ref_dn12))) / (assign6150_e7439 * assign6150_e7439)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign6150_e7442;
        locals.var_t1_dn3 = assign6150_e7442_d_n3;
        locals.var_t1_dn4 = assign6150_e7442_d_n4;
        locals.var_t1_dn5 = assign6150_e7442_d_n5;
        locals.var_t1_dn6 = assign6150_e7442_d_n6;
        locals.var_t1_dn7 = assign6150_e7442_d_n7;
        locals.var_t1_dn8 = assign6150_e7442_d_n8;
        locals.var_t1_dn9 = assign6150_e7442_d_n9;
        locals.var_t1_dn10 = assign6150_e7442_d_n10;
        locals.var_t1_dn11 = assign6150_e7442_d_n11;
        locals.var_t1_dn12 = assign6150_e7442_d_n12;

        let (assign6160_e7448, assign6160_e7448_d_n3, assign6160_e7448_d_n4, assign6160_e7448_d_n5, assign6160_e7448_d_n6, assign6160_e7448_d_n7, assign6160_e7448_d_n8, assign6160_e7448_d_n9, assign6160_e7448_d_n10, assign6160_e7448_d_n11, assign6160_e7448_d_n12,) = {
    if (locals.var_guard578 != 0.0) {
        let assign6160_e7446: f64 = (locals.var_pparam_b4soivsattemp * locals.var_t1);
        (assign6160_e7446, ((locals.var_pparam_b4soivsattemp_dn3 * locals.var_t1) + (locals.var_pparam_b4soivsattemp * locals.var_t1_dn3)), ((locals.var_pparam_b4soivsattemp_dn4 * locals.var_t1) + (locals.var_pparam_b4soivsattemp * locals.var_t1_dn4)), ((locals.var_pparam_b4soivsattemp_dn5 * locals.var_t1) + (locals.var_pparam_b4soivsattemp * locals.var_t1_dn5)), ((locals.var_pparam_b4soivsattemp_dn6 * locals.var_t1) + (locals.var_pparam_b4soivsattemp * locals.var_t1_dn6)), ((locals.var_pparam_b4soivsattemp_dn7 * locals.var_t1) + (locals.var_pparam_b4soivsattemp * locals.var_t1_dn7)), ((locals.var_pparam_b4soivsattemp_dn8 * locals.var_t1) + (locals.var_pparam_b4soivsattemp * locals.var_t1_dn8)), ((locals.var_pparam_b4soivsattemp_dn9 * locals.var_t1) + (locals.var_pparam_b4soivsattemp * locals.var_t1_dn9)), ((locals.var_pparam_b4soivsattemp_dn10 * locals.var_t1) + (locals.var_pparam_b4soivsattemp * locals.var_t1_dn10)), ((locals.var_pparam_b4soivsattemp_dn11 * locals.var_t1) + (locals.var_pparam_b4soivsattemp * locals.var_t1_dn11)), ((locals.var_pparam_b4soivsattemp_dn12 * locals.var_t1) + (locals.var_pparam_b4soivsattemp * locals.var_t1_dn12)),)
    } else {
        (locals.var_here_b4soivsattemp, locals.var_here_b4soivsattemp_dn3, locals.var_here_b4soivsattemp_dn4, locals.var_here_b4soivsattemp_dn5, locals.var_here_b4soivsattemp_dn6, locals.var_here_b4soivsattemp_dn7, locals.var_here_b4soivsattemp_dn8, locals.var_here_b4soivsattemp_dn9, locals.var_here_b4soivsattemp_dn10, locals.var_here_b4soivsattemp_dn11, locals.var_here_b4soivsattemp_dn12,)
    }
};
        locals.var_here_b4soivsattemp = assign6160_e7448;
        locals.var_here_b4soivsattemp_dn3 = assign6160_e7448_d_n3;
        locals.var_here_b4soivsattemp_dn4 = assign6160_e7448_d_n4;
        locals.var_here_b4soivsattemp_dn5 = assign6160_e7448_d_n5;
        locals.var_here_b4soivsattemp_dn6 = assign6160_e7448_d_n6;
        locals.var_here_b4soivsattemp_dn7 = assign6160_e7448_d_n7;
        locals.var_here_b4soivsattemp_dn8 = assign6160_e7448_d_n8;
        locals.var_here_b4soivsattemp_dn9 = assign6160_e7448_d_n9;
        locals.var_here_b4soivsattemp_dn10 = assign6160_e7448_d_n10;
        locals.var_here_b4soivsattemp_dn11 = assign6160_e7448_d_n11;
        locals.var_here_b4soivsattemp_dn12 = assign6160_e7448_d_n12;

        let (assign6170_e7454,) = {
    if (locals.var_guard578 != 0.0) {
        let assign6170_e7452: f64 = (locals.var_inv_odeff - locals.var_pparam_b4soiinv_od_ref);
        (assign6170_e7452,)
    } else {
        (locals.var_od_offset,)
    }
};
        locals.var_od_offset = assign6170_e7454;

    }

    pub(super) fn stamp_transient_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6180_e7462, assign6180_e7462_d_n3, assign6180_e7462_d_n4, assign6180_e7462_d_n5, assign6180_e7462_d_n6, assign6180_e7462_d_n7, assign6180_e7462_d_n8, assign6180_e7462_d_n9, assign6180_e7462_d_n10, assign6180_e7462_d_n11, assign6180_e7462_d_n12,) = {
    if (locals.var_guard578 != 0.0) {
        let assign6180_e7458: f64 = (p.p237 / locals.var_pparam_b4soikvth0);
        let assign6180_e7460: f64 = (assign6180_e7458 * locals.var_od_offset);
        (assign6180_e7460, ((-((p.p237 * locals.var_pparam_b4soikvth0_dn3) / (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0))) * locals.var_od_offset), ((-((p.p237 * locals.var_pparam_b4soikvth0_dn4) / (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0))) * locals.var_od_offset), ((-((p.p237 * locals.var_pparam_b4soikvth0_dn5) / (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0))) * locals.var_od_offset), ((-((p.p237 * locals.var_pparam_b4soikvth0_dn6) / (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0))) * locals.var_od_offset), ((-((p.p237 * locals.var_pparam_b4soikvth0_dn7) / (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0))) * locals.var_od_offset), ((-((p.p237 * locals.var_pparam_b4soikvth0_dn8) / (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0))) * locals.var_od_offset), ((-((p.p237 * locals.var_pparam_b4soikvth0_dn9) / (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0))) * locals.var_od_offset), ((-((p.p237 * locals.var_pparam_b4soikvth0_dn10) / (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0))) * locals.var_od_offset), ((-((p.p237 * locals.var_pparam_b4soikvth0_dn11) / (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0))) * locals.var_od_offset), ((-((p.p237 * locals.var_pparam_b4soikvth0_dn12) / (locals.var_pparam_b4soikvth0 * locals.var_pparam_b4soikvth0))) * locals.var_od_offset),)
    } else {
        (locals.var_dvth0_lod, locals.var_dvth0_lod_dn3, locals.var_dvth0_lod_dn4, locals.var_dvth0_lod_dn5, locals.var_dvth0_lod_dn6, locals.var_dvth0_lod_dn7, locals.var_dvth0_lod_dn8, locals.var_dvth0_lod_dn9, locals.var_dvth0_lod_dn10, locals.var_dvth0_lod_dn11, locals.var_dvth0_lod_dn12,)
    }
};
        locals.var_dvth0_lod = assign6180_e7462;
        locals.var_dvth0_lod_dn3 = assign6180_e7462_d_n3;
        locals.var_dvth0_lod_dn4 = assign6180_e7462_d_n4;
        locals.var_dvth0_lod_dn5 = assign6180_e7462_d_n5;
        locals.var_dvth0_lod_dn6 = assign6180_e7462_d_n6;
        locals.var_dvth0_lod_dn7 = assign6180_e7462_d_n7;
        locals.var_dvth0_lod_dn8 = assign6180_e7462_d_n8;
        locals.var_dvth0_lod_dn9 = assign6180_e7462_d_n9;
        locals.var_dvth0_lod_dn10 = assign6180_e7462_d_n10;
        locals.var_dvth0_lod_dn11 = assign6180_e7462_d_n11;
        locals.var_dvth0_lod_dn12 = assign6180_e7462_d_n12;

        let (assign6190_e7472, assign6190_e7472_d_n3, assign6190_e7472_d_n4, assign6190_e7472_d_n5, assign6190_e7472_d_n6, assign6190_e7472_d_n7, assign6190_e7472_d_n8, assign6190_e7472_d_n9, assign6190_e7472_d_n10, assign6190_e7472_d_n11, assign6190_e7472_d_n12,) = {
    if (locals.var_guard578 != 0.0) {
        let assign6190_e7467: f64 = (locals.var_pparam_b4soikvth0).powf(p.p250);
        let assign6190_e7468: f64 = (p.p249 / assign6190_e7467);
        let assign6190_e7470: f64 = (assign6190_e7468 * locals.var_od_offset);
        (assign6190_e7470, ((-((p.p249 * if 0.0 == 0.0 && ((p.p250) as f64).is_finite() && ((p.p250) as f64).fract() == 0.0 { if p.p250 == 0.0 { 0.0 } else { (p.p250 * ((locals.var_pparam_b4soikvth0).powf(p.p250 - 1.0) * locals.var_pparam_b4soikvth0_dn3)) } } else { (assign6190_e7467 * (p.p250 * (locals.var_pparam_b4soikvth0_dn3 / locals.var_pparam_b4soikvth0))) }) / (assign6190_e7467 * assign6190_e7467))) * locals.var_od_offset), ((-((p.p249 * if 0.0 == 0.0 && ((p.p250) as f64).is_finite() && ((p.p250) as f64).fract() == 0.0 { if p.p250 == 0.0 { 0.0 } else { (p.p250 * ((locals.var_pparam_b4soikvth0).powf(p.p250 - 1.0) * locals.var_pparam_b4soikvth0_dn4)) } } else { (assign6190_e7467 * (p.p250 * (locals.var_pparam_b4soikvth0_dn4 / locals.var_pparam_b4soikvth0))) }) / (assign6190_e7467 * assign6190_e7467))) * locals.var_od_offset), ((-((p.p249 * if 0.0 == 0.0 && ((p.p250) as f64).is_finite() && ((p.p250) as f64).fract() == 0.0 { if p.p250 == 0.0 { 0.0 } else { (p.p250 * ((locals.var_pparam_b4soikvth0).powf(p.p250 - 1.0) * locals.var_pparam_b4soikvth0_dn5)) } } else { (assign6190_e7467 * (p.p250 * (locals.var_pparam_b4soikvth0_dn5 / locals.var_pparam_b4soikvth0))) }) / (assign6190_e7467 * assign6190_e7467))) * locals.var_od_offset), ((-((p.p249 * if 0.0 == 0.0 && ((p.p250) as f64).is_finite() && ((p.p250) as f64).fract() == 0.0 { if p.p250 == 0.0 { 0.0 } else { (p.p250 * ((locals.var_pparam_b4soikvth0).powf(p.p250 - 1.0) * locals.var_pparam_b4soikvth0_dn6)) } } else { (assign6190_e7467 * (p.p250 * (locals.var_pparam_b4soikvth0_dn6 / locals.var_pparam_b4soikvth0))) }) / (assign6190_e7467 * assign6190_e7467))) * locals.var_od_offset), ((-((p.p249 * if 0.0 == 0.0 && ((p.p250) as f64).is_finite() && ((p.p250) as f64).fract() == 0.0 { if p.p250 == 0.0 { 0.0 } else { (p.p250 * ((locals.var_pparam_b4soikvth0).powf(p.p250 - 1.0) * locals.var_pparam_b4soikvth0_dn7)) } } else { (assign6190_e7467 * (p.p250 * (locals.var_pparam_b4soikvth0_dn7 / locals.var_pparam_b4soikvth0))) }) / (assign6190_e7467 * assign6190_e7467))) * locals.var_od_offset), ((-((p.p249 * if 0.0 == 0.0 && ((p.p250) as f64).is_finite() && ((p.p250) as f64).fract() == 0.0 { if p.p250 == 0.0 { 0.0 } else { (p.p250 * ((locals.var_pparam_b4soikvth0).powf(p.p250 - 1.0) * locals.var_pparam_b4soikvth0_dn8)) } } else { (assign6190_e7467 * (p.p250 * (locals.var_pparam_b4soikvth0_dn8 / locals.var_pparam_b4soikvth0))) }) / (assign6190_e7467 * assign6190_e7467))) * locals.var_od_offset), ((-((p.p249 * if 0.0 == 0.0 && ((p.p250) as f64).is_finite() && ((p.p250) as f64).fract() == 0.0 { if p.p250 == 0.0 { 0.0 } else { (p.p250 * ((locals.var_pparam_b4soikvth0).powf(p.p250 - 1.0) * locals.var_pparam_b4soikvth0_dn9)) } } else { (assign6190_e7467 * (p.p250 * (locals.var_pparam_b4soikvth0_dn9 / locals.var_pparam_b4soikvth0))) }) / (assign6190_e7467 * assign6190_e7467))) * locals.var_od_offset), ((-((p.p249 * if 0.0 == 0.0 && ((p.p250) as f64).is_finite() && ((p.p250) as f64).fract() == 0.0 { if p.p250 == 0.0 { 0.0 } else { (p.p250 * ((locals.var_pparam_b4soikvth0).powf(p.p250 - 1.0) * locals.var_pparam_b4soikvth0_dn10)) } } else { (assign6190_e7467 * (p.p250 * (locals.var_pparam_b4soikvth0_dn10 / locals.var_pparam_b4soikvth0))) }) / (assign6190_e7467 * assign6190_e7467))) * locals.var_od_offset), ((-((p.p249 * if 0.0 == 0.0 && ((p.p250) as f64).is_finite() && ((p.p250) as f64).fract() == 0.0 { if p.p250 == 0.0 { 0.0 } else { (p.p250 * ((locals.var_pparam_b4soikvth0).powf(p.p250 - 1.0) * locals.var_pparam_b4soikvth0_dn11)) } } else { (assign6190_e7467 * (p.p250 * (locals.var_pparam_b4soikvth0_dn11 / locals.var_pparam_b4soikvth0))) }) / (assign6190_e7467 * assign6190_e7467))) * locals.var_od_offset), ((-((p.p249 * if 0.0 == 0.0 && ((p.p250) as f64).is_finite() && ((p.p250) as f64).fract() == 0.0 { if p.p250 == 0.0 { 0.0 } else { (p.p250 * ((locals.var_pparam_b4soikvth0).powf(p.p250 - 1.0) * locals.var_pparam_b4soikvth0_dn12)) } } else { (assign6190_e7467 * (p.p250 * (locals.var_pparam_b4soikvth0_dn12 / locals.var_pparam_b4soikvth0))) }) / (assign6190_e7467 * assign6190_e7467))) * locals.var_od_offset),)
    } else {
        (locals.var_dk2_lod, locals.var_dk2_lod_dn3, locals.var_dk2_lod_dn4, locals.var_dk2_lod_dn5, locals.var_dk2_lod_dn6, locals.var_dk2_lod_dn7, locals.var_dk2_lod_dn8, locals.var_dk2_lod_dn9, locals.var_dk2_lod_dn10, locals.var_dk2_lod_dn11, locals.var_dk2_lod_dn12,)
    }
};
        locals.var_dk2_lod = assign6190_e7472;
        locals.var_dk2_lod_dn3 = assign6190_e7472_d_n3;
        locals.var_dk2_lod_dn4 = assign6190_e7472_d_n4;
        locals.var_dk2_lod_dn5 = assign6190_e7472_d_n5;
        locals.var_dk2_lod_dn6 = assign6190_e7472_d_n6;
        locals.var_dk2_lod_dn7 = assign6190_e7472_d_n7;
        locals.var_dk2_lod_dn8 = assign6190_e7472_d_n8;
        locals.var_dk2_lod_dn9 = assign6190_e7472_d_n9;
        locals.var_dk2_lod_dn10 = assign6190_e7472_d_n10;
        locals.var_dk2_lod_dn11 = assign6190_e7472_d_n11;
        locals.var_dk2_lod_dn12 = assign6190_e7472_d_n12;

        let (assign6200_e7482, assign6200_e7482_d_n3, assign6200_e7482_d_n4, assign6200_e7482_d_n5, assign6200_e7482_d_n6, assign6200_e7482_d_n7, assign6200_e7482_d_n8, assign6200_e7482_d_n9, assign6200_e7482_d_n10, assign6200_e7482_d_n11, assign6200_e7482_d_n12,) = {
    if (locals.var_guard578 != 0.0) {
        let assign6200_e7477: f64 = (locals.var_pparam_b4soikvth0).powf(p.p252);
        let assign6200_e7478: f64 = (p.p251 / assign6200_e7477);
        let assign6200_e7480: f64 = (assign6200_e7478 * locals.var_od_offset);
        (assign6200_e7480, ((-((p.p251 * if 0.0 == 0.0 && ((p.p252) as f64).is_finite() && ((p.p252) as f64).fract() == 0.0 { if p.p252 == 0.0 { 0.0 } else { (p.p252 * ((locals.var_pparam_b4soikvth0).powf(p.p252 - 1.0) * locals.var_pparam_b4soikvth0_dn3)) } } else { (assign6200_e7477 * (p.p252 * (locals.var_pparam_b4soikvth0_dn3 / locals.var_pparam_b4soikvth0))) }) / (assign6200_e7477 * assign6200_e7477))) * locals.var_od_offset), ((-((p.p251 * if 0.0 == 0.0 && ((p.p252) as f64).is_finite() && ((p.p252) as f64).fract() == 0.0 { if p.p252 == 0.0 { 0.0 } else { (p.p252 * ((locals.var_pparam_b4soikvth0).powf(p.p252 - 1.0) * locals.var_pparam_b4soikvth0_dn4)) } } else { (assign6200_e7477 * (p.p252 * (locals.var_pparam_b4soikvth0_dn4 / locals.var_pparam_b4soikvth0))) }) / (assign6200_e7477 * assign6200_e7477))) * locals.var_od_offset), ((-((p.p251 * if 0.0 == 0.0 && ((p.p252) as f64).is_finite() && ((p.p252) as f64).fract() == 0.0 { if p.p252 == 0.0 { 0.0 } else { (p.p252 * ((locals.var_pparam_b4soikvth0).powf(p.p252 - 1.0) * locals.var_pparam_b4soikvth0_dn5)) } } else { (assign6200_e7477 * (p.p252 * (locals.var_pparam_b4soikvth0_dn5 / locals.var_pparam_b4soikvth0))) }) / (assign6200_e7477 * assign6200_e7477))) * locals.var_od_offset), ((-((p.p251 * if 0.0 == 0.0 && ((p.p252) as f64).is_finite() && ((p.p252) as f64).fract() == 0.0 { if p.p252 == 0.0 { 0.0 } else { (p.p252 * ((locals.var_pparam_b4soikvth0).powf(p.p252 - 1.0) * locals.var_pparam_b4soikvth0_dn6)) } } else { (assign6200_e7477 * (p.p252 * (locals.var_pparam_b4soikvth0_dn6 / locals.var_pparam_b4soikvth0))) }) / (assign6200_e7477 * assign6200_e7477))) * locals.var_od_offset), ((-((p.p251 * if 0.0 == 0.0 && ((p.p252) as f64).is_finite() && ((p.p252) as f64).fract() == 0.0 { if p.p252 == 0.0 { 0.0 } else { (p.p252 * ((locals.var_pparam_b4soikvth0).powf(p.p252 - 1.0) * locals.var_pparam_b4soikvth0_dn7)) } } else { (assign6200_e7477 * (p.p252 * (locals.var_pparam_b4soikvth0_dn7 / locals.var_pparam_b4soikvth0))) }) / (assign6200_e7477 * assign6200_e7477))) * locals.var_od_offset), ((-((p.p251 * if 0.0 == 0.0 && ((p.p252) as f64).is_finite() && ((p.p252) as f64).fract() == 0.0 { if p.p252 == 0.0 { 0.0 } else { (p.p252 * ((locals.var_pparam_b4soikvth0).powf(p.p252 - 1.0) * locals.var_pparam_b4soikvth0_dn8)) } } else { (assign6200_e7477 * (p.p252 * (locals.var_pparam_b4soikvth0_dn8 / locals.var_pparam_b4soikvth0))) }) / (assign6200_e7477 * assign6200_e7477))) * locals.var_od_offset), ((-((p.p251 * if 0.0 == 0.0 && ((p.p252) as f64).is_finite() && ((p.p252) as f64).fract() == 0.0 { if p.p252 == 0.0 { 0.0 } else { (p.p252 * ((locals.var_pparam_b4soikvth0).powf(p.p252 - 1.0) * locals.var_pparam_b4soikvth0_dn9)) } } else { (assign6200_e7477 * (p.p252 * (locals.var_pparam_b4soikvth0_dn9 / locals.var_pparam_b4soikvth0))) }) / (assign6200_e7477 * assign6200_e7477))) * locals.var_od_offset), ((-((p.p251 * if 0.0 == 0.0 && ((p.p252) as f64).is_finite() && ((p.p252) as f64).fract() == 0.0 { if p.p252 == 0.0 { 0.0 } else { (p.p252 * ((locals.var_pparam_b4soikvth0).powf(p.p252 - 1.0) * locals.var_pparam_b4soikvth0_dn10)) } } else { (assign6200_e7477 * (p.p252 * (locals.var_pparam_b4soikvth0_dn10 / locals.var_pparam_b4soikvth0))) }) / (assign6200_e7477 * assign6200_e7477))) * locals.var_od_offset), ((-((p.p251 * if 0.0 == 0.0 && ((p.p252) as f64).is_finite() && ((p.p252) as f64).fract() == 0.0 { if p.p252 == 0.0 { 0.0 } else { (p.p252 * ((locals.var_pparam_b4soikvth0).powf(p.p252 - 1.0) * locals.var_pparam_b4soikvth0_dn11)) } } else { (assign6200_e7477 * (p.p252 * (locals.var_pparam_b4soikvth0_dn11 / locals.var_pparam_b4soikvth0))) }) / (assign6200_e7477 * assign6200_e7477))) * locals.var_od_offset), ((-((p.p251 * if 0.0 == 0.0 && ((p.p252) as f64).is_finite() && ((p.p252) as f64).fract() == 0.0 { if p.p252 == 0.0 { 0.0 } else { (p.p252 * ((locals.var_pparam_b4soikvth0).powf(p.p252 - 1.0) * locals.var_pparam_b4soikvth0_dn12)) } } else { (assign6200_e7477 * (p.p252 * (locals.var_pparam_b4soikvth0_dn12 / locals.var_pparam_b4soikvth0))) }) / (assign6200_e7477 * assign6200_e7477))) * locals.var_od_offset),)
    } else {
        (locals.var_deta0_lod, locals.var_deta0_lod_dn3, locals.var_deta0_lod_dn4, locals.var_deta0_lod_dn5, locals.var_deta0_lod_dn6, locals.var_deta0_lod_dn7, locals.var_deta0_lod_dn8, locals.var_deta0_lod_dn9, locals.var_deta0_lod_dn10, locals.var_deta0_lod_dn11, locals.var_deta0_lod_dn12,)
    }
};
        locals.var_deta0_lod = assign6200_e7482;
        locals.var_deta0_lod_dn3 = assign6200_e7482_d_n3;
        locals.var_deta0_lod_dn4 = assign6200_e7482_d_n4;
        locals.var_deta0_lod_dn5 = assign6200_e7482_d_n5;
        locals.var_deta0_lod_dn6 = assign6200_e7482_d_n6;
        locals.var_deta0_lod_dn7 = assign6200_e7482_d_n7;
        locals.var_deta0_lod_dn8 = assign6200_e7482_d_n8;
        locals.var_deta0_lod_dn9 = assign6200_e7482_d_n9;
        locals.var_deta0_lod_dn10 = assign6200_e7482_d_n10;
        locals.var_deta0_lod_dn11 = assign6200_e7482_d_n11;
        locals.var_deta0_lod_dn12 = assign6200_e7482_d_n12;

        let (assign6210_e7492, assign6210_e7492_d_n3, assign6210_e7492_d_n4, assign6210_e7492_d_n5, assign6210_e7492_d_n6, assign6210_e7492_d_n7, assign6210_e7492_d_n8, assign6210_e7492_d_n9, assign6210_e7492_d_n10, assign6210_e7492_d_n11, assign6210_e7492_d_n12,) = {
    if (locals.var_guard578 != 0.0) {
        let assign6210_e7487: f64 = (locals.var_pparam_b4soikvth0).powf(p.p254);
        let assign6210_e7488: f64 = (p.p253 / assign6210_e7487);
        let assign6210_e7490: f64 = (assign6210_e7488 * locals.var_od_offset);
        (assign6210_e7490, ((-((p.p253 * if 0.0 == 0.0 && ((p.p254) as f64).is_finite() && ((p.p254) as f64).fract() == 0.0 { if p.p254 == 0.0 { 0.0 } else { (p.p254 * ((locals.var_pparam_b4soikvth0).powf(p.p254 - 1.0) * locals.var_pparam_b4soikvth0_dn3)) } } else { (assign6210_e7487 * (p.p254 * (locals.var_pparam_b4soikvth0_dn3 / locals.var_pparam_b4soikvth0))) }) / (assign6210_e7487 * assign6210_e7487))) * locals.var_od_offset), ((-((p.p253 * if 0.0 == 0.0 && ((p.p254) as f64).is_finite() && ((p.p254) as f64).fract() == 0.0 { if p.p254 == 0.0 { 0.0 } else { (p.p254 * ((locals.var_pparam_b4soikvth0).powf(p.p254 - 1.0) * locals.var_pparam_b4soikvth0_dn4)) } } else { (assign6210_e7487 * (p.p254 * (locals.var_pparam_b4soikvth0_dn4 / locals.var_pparam_b4soikvth0))) }) / (assign6210_e7487 * assign6210_e7487))) * locals.var_od_offset), ((-((p.p253 * if 0.0 == 0.0 && ((p.p254) as f64).is_finite() && ((p.p254) as f64).fract() == 0.0 { if p.p254 == 0.0 { 0.0 } else { (p.p254 * ((locals.var_pparam_b4soikvth0).powf(p.p254 - 1.0) * locals.var_pparam_b4soikvth0_dn5)) } } else { (assign6210_e7487 * (p.p254 * (locals.var_pparam_b4soikvth0_dn5 / locals.var_pparam_b4soikvth0))) }) / (assign6210_e7487 * assign6210_e7487))) * locals.var_od_offset), ((-((p.p253 * if 0.0 == 0.0 && ((p.p254) as f64).is_finite() && ((p.p254) as f64).fract() == 0.0 { if p.p254 == 0.0 { 0.0 } else { (p.p254 * ((locals.var_pparam_b4soikvth0).powf(p.p254 - 1.0) * locals.var_pparam_b4soikvth0_dn6)) } } else { (assign6210_e7487 * (p.p254 * (locals.var_pparam_b4soikvth0_dn6 / locals.var_pparam_b4soikvth0))) }) / (assign6210_e7487 * assign6210_e7487))) * locals.var_od_offset), ((-((p.p253 * if 0.0 == 0.0 && ((p.p254) as f64).is_finite() && ((p.p254) as f64).fract() == 0.0 { if p.p254 == 0.0 { 0.0 } else { (p.p254 * ((locals.var_pparam_b4soikvth0).powf(p.p254 - 1.0) * locals.var_pparam_b4soikvth0_dn7)) } } else { (assign6210_e7487 * (p.p254 * (locals.var_pparam_b4soikvth0_dn7 / locals.var_pparam_b4soikvth0))) }) / (assign6210_e7487 * assign6210_e7487))) * locals.var_od_offset), ((-((p.p253 * if 0.0 == 0.0 && ((p.p254) as f64).is_finite() && ((p.p254) as f64).fract() == 0.0 { if p.p254 == 0.0 { 0.0 } else { (p.p254 * ((locals.var_pparam_b4soikvth0).powf(p.p254 - 1.0) * locals.var_pparam_b4soikvth0_dn8)) } } else { (assign6210_e7487 * (p.p254 * (locals.var_pparam_b4soikvth0_dn8 / locals.var_pparam_b4soikvth0))) }) / (assign6210_e7487 * assign6210_e7487))) * locals.var_od_offset), ((-((p.p253 * if 0.0 == 0.0 && ((p.p254) as f64).is_finite() && ((p.p254) as f64).fract() == 0.0 { if p.p254 == 0.0 { 0.0 } else { (p.p254 * ((locals.var_pparam_b4soikvth0).powf(p.p254 - 1.0) * locals.var_pparam_b4soikvth0_dn9)) } } else { (assign6210_e7487 * (p.p254 * (locals.var_pparam_b4soikvth0_dn9 / locals.var_pparam_b4soikvth0))) }) / (assign6210_e7487 * assign6210_e7487))) * locals.var_od_offset), ((-((p.p253 * if 0.0 == 0.0 && ((p.p254) as f64).is_finite() && ((p.p254) as f64).fract() == 0.0 { if p.p254 == 0.0 { 0.0 } else { (p.p254 * ((locals.var_pparam_b4soikvth0).powf(p.p254 - 1.0) * locals.var_pparam_b4soikvth0_dn10)) } } else { (assign6210_e7487 * (p.p254 * (locals.var_pparam_b4soikvth0_dn10 / locals.var_pparam_b4soikvth0))) }) / (assign6210_e7487 * assign6210_e7487))) * locals.var_od_offset), ((-((p.p253 * if 0.0 == 0.0 && ((p.p254) as f64).is_finite() && ((p.p254) as f64).fract() == 0.0 { if p.p254 == 0.0 { 0.0 } else { (p.p254 * ((locals.var_pparam_b4soikvth0).powf(p.p254 - 1.0) * locals.var_pparam_b4soikvth0_dn11)) } } else { (assign6210_e7487 * (p.p254 * (locals.var_pparam_b4soikvth0_dn11 / locals.var_pparam_b4soikvth0))) }) / (assign6210_e7487 * assign6210_e7487))) * locals.var_od_offset), ((-((p.p253 * if 0.0 == 0.0 && ((p.p254) as f64).is_finite() && ((p.p254) as f64).fract() == 0.0 { if p.p254 == 0.0 { 0.0 } else { (p.p254 * ((locals.var_pparam_b4soikvth0).powf(p.p254 - 1.0) * locals.var_pparam_b4soikvth0_dn12)) } } else { (assign6210_e7487 * (p.p254 * (locals.var_pparam_b4soikvth0_dn12 / locals.var_pparam_b4soikvth0))) }) / (assign6210_e7487 * assign6210_e7487))) * locals.var_od_offset),)
    } else {
        (locals.var_deta0cv_lod, locals.var_deta0cv_lod_dn3, locals.var_deta0cv_lod_dn4, locals.var_deta0cv_lod_dn5, locals.var_deta0cv_lod_dn6, locals.var_deta0cv_lod_dn7, locals.var_deta0cv_lod_dn8, locals.var_deta0cv_lod_dn9, locals.var_deta0cv_lod_dn10, locals.var_deta0cv_lod_dn11, locals.var_deta0cv_lod_dn12,)
    }
};
        locals.var_deta0cv_lod = assign6210_e7492;
        locals.var_deta0cv_lod_dn3 = assign6210_e7492_d_n3;
        locals.var_deta0cv_lod_dn4 = assign6210_e7492_d_n4;
        locals.var_deta0cv_lod_dn5 = assign6210_e7492_d_n5;
        locals.var_deta0cv_lod_dn6 = assign6210_e7492_d_n6;
        locals.var_deta0cv_lod_dn7 = assign6210_e7492_d_n7;
        locals.var_deta0cv_lod_dn8 = assign6210_e7492_d_n8;
        locals.var_deta0cv_lod_dn9 = assign6210_e7492_d_n9;
        locals.var_deta0cv_lod_dn10 = assign6210_e7492_d_n10;
        locals.var_deta0cv_lod_dn11 = assign6210_e7492_d_n11;
        locals.var_deta0cv_lod_dn12 = assign6210_e7492_d_n12;

        let (assign6220_e7498, assign6220_e7498_d_n3, assign6220_e7498_d_n4, assign6220_e7498_d_n5, assign6220_e7498_d_n6, assign6220_e7498_d_n7, assign6220_e7498_d_n8, assign6220_e7498_d_n9, assign6220_e7498_d_n10, assign6220_e7498_d_n11, assign6220_e7498_d_n12,) = {
    if (locals.var_guard578 != 0.0) {
        let assign6220_e7496: f64 = (locals.var_pparam_b4soivth0 + locals.var_dvth0_lod);
        (assign6220_e7496, (locals.var_pparam_b4soivth0_dn3 + locals.var_dvth0_lod_dn3), (locals.var_pparam_b4soivth0_dn4 + locals.var_dvth0_lod_dn4), (locals.var_pparam_b4soivth0_dn5 + locals.var_dvth0_lod_dn5), (locals.var_pparam_b4soivth0_dn6 + locals.var_dvth0_lod_dn6), (locals.var_pparam_b4soivth0_dn7 + locals.var_dvth0_lod_dn7), (locals.var_pparam_b4soivth0_dn8 + locals.var_dvth0_lod_dn8), (locals.var_pparam_b4soivth0_dn9 + locals.var_dvth0_lod_dn9), (locals.var_pparam_b4soivth0_dn10 + locals.var_dvth0_lod_dn10), (locals.var_pparam_b4soivth0_dn11 + locals.var_dvth0_lod_dn11), (locals.var_pparam_b4soivth0_dn12 + locals.var_dvth0_lod_dn12),)
    } else {
        (locals.var_here_b4soivth0, locals.var_here_b4soivth0_dn3, locals.var_here_b4soivth0_dn4, locals.var_here_b4soivth0_dn5, locals.var_here_b4soivth0_dn6, locals.var_here_b4soivth0_dn7, locals.var_here_b4soivth0_dn8, locals.var_here_b4soivth0_dn9, locals.var_here_b4soivth0_dn10, locals.var_here_b4soivth0_dn11, locals.var_here_b4soivth0_dn12,)
    }
};
        locals.var_here_b4soivth0 = assign6220_e7498;
        locals.var_here_b4soivth0_dn3 = assign6220_e7498_d_n3;
        locals.var_here_b4soivth0_dn4 = assign6220_e7498_d_n4;
        locals.var_here_b4soivth0_dn5 = assign6220_e7498_d_n5;
        locals.var_here_b4soivth0_dn6 = assign6220_e7498_d_n6;
        locals.var_here_b4soivth0_dn7 = assign6220_e7498_d_n7;
        locals.var_here_b4soivth0_dn8 = assign6220_e7498_d_n8;
        locals.var_here_b4soivth0_dn9 = assign6220_e7498_d_n9;
        locals.var_here_b4soivth0_dn10 = assign6220_e7498_d_n10;
        locals.var_here_b4soivth0_dn11 = assign6220_e7498_d_n11;
        locals.var_here_b4soivth0_dn12 = assign6220_e7498_d_n12;

        let (assign6230_e7504, assign6230_e7504_d_n3, assign6230_e7504_d_n4, assign6230_e7504_d_n5, assign6230_e7504_d_n6, assign6230_e7504_d_n7, assign6230_e7504_d_n8, assign6230_e7504_d_n9, assign6230_e7504_d_n10, assign6230_e7504_d_n11, assign6230_e7504_d_n12,) = {
    if (locals.var_guard578 != 0.0) {
        let assign6230_e7502: f64 = (locals.var_pparam_b4soik2 + locals.var_dk2_lod);
        (assign6230_e7502, (locals.var_pparam_b4soik2_dn3 + locals.var_dk2_lod_dn3), (locals.var_pparam_b4soik2_dn4 + locals.var_dk2_lod_dn4), (locals.var_pparam_b4soik2_dn5 + locals.var_dk2_lod_dn5), (locals.var_pparam_b4soik2_dn6 + locals.var_dk2_lod_dn6), (locals.var_pparam_b4soik2_dn7 + locals.var_dk2_lod_dn7), (locals.var_pparam_b4soik2_dn8 + locals.var_dk2_lod_dn8), (locals.var_pparam_b4soik2_dn9 + locals.var_dk2_lod_dn9), (locals.var_pparam_b4soik2_dn10 + locals.var_dk2_lod_dn10), (locals.var_pparam_b4soik2_dn11 + locals.var_dk2_lod_dn11), (locals.var_pparam_b4soik2_dn12 + locals.var_dk2_lod_dn12),)
    } else {
        (locals.var_here_b4soik2, locals.var_here_b4soik2_dn3, locals.var_here_b4soik2_dn4, locals.var_here_b4soik2_dn5, locals.var_here_b4soik2_dn6, locals.var_here_b4soik2_dn7, locals.var_here_b4soik2_dn8, locals.var_here_b4soik2_dn9, locals.var_here_b4soik2_dn10, locals.var_here_b4soik2_dn11, locals.var_here_b4soik2_dn12,)
    }
};
        locals.var_here_b4soik2 = assign6230_e7504;
        locals.var_here_b4soik2_dn3 = assign6230_e7504_d_n3;
        locals.var_here_b4soik2_dn4 = assign6230_e7504_d_n4;
        locals.var_here_b4soik2_dn5 = assign6230_e7504_d_n5;
        locals.var_here_b4soik2_dn6 = assign6230_e7504_d_n6;
        locals.var_here_b4soik2_dn7 = assign6230_e7504_d_n7;
        locals.var_here_b4soik2_dn8 = assign6230_e7504_d_n8;
        locals.var_here_b4soik2_dn9 = assign6230_e7504_d_n9;
        locals.var_here_b4soik2_dn10 = assign6230_e7504_d_n10;
        locals.var_here_b4soik2_dn11 = assign6230_e7504_d_n11;
        locals.var_here_b4soik2_dn12 = assign6230_e7504_d_n12;

        let (assign6240_e7510, assign6240_e7510_d_n3, assign6240_e7510_d_n4, assign6240_e7510_d_n5, assign6240_e7510_d_n6, assign6240_e7510_d_n7, assign6240_e7510_d_n8, assign6240_e7510_d_n9, assign6240_e7510_d_n10, assign6240_e7510_d_n11, assign6240_e7510_d_n12,) = {
    if (locals.var_guard578 != 0.0) {
        let assign6240_e7508: f64 = (locals.var_pparam_b4soieta0 + locals.var_deta0_lod);
        (assign6240_e7508, (locals.var_pparam_b4soieta0_dn3 + locals.var_deta0_lod_dn3), (locals.var_pparam_b4soieta0_dn4 + locals.var_deta0_lod_dn4), (locals.var_pparam_b4soieta0_dn5 + locals.var_deta0_lod_dn5), (locals.var_pparam_b4soieta0_dn6 + locals.var_deta0_lod_dn6), (locals.var_pparam_b4soieta0_dn7 + locals.var_deta0_lod_dn7), (locals.var_pparam_b4soieta0_dn8 + locals.var_deta0_lod_dn8), (locals.var_pparam_b4soieta0_dn9 + locals.var_deta0_lod_dn9), (locals.var_pparam_b4soieta0_dn10 + locals.var_deta0_lod_dn10), (locals.var_pparam_b4soieta0_dn11 + locals.var_deta0_lod_dn11), (locals.var_pparam_b4soieta0_dn12 + locals.var_deta0_lod_dn12),)
    } else {
        (locals.var_here_b4soieta0, locals.var_here_b4soieta0_dn3, locals.var_here_b4soieta0_dn4, locals.var_here_b4soieta0_dn5, locals.var_here_b4soieta0_dn6, locals.var_here_b4soieta0_dn7, locals.var_here_b4soieta0_dn8, locals.var_here_b4soieta0_dn9, locals.var_here_b4soieta0_dn10, locals.var_here_b4soieta0_dn11, locals.var_here_b4soieta0_dn12,)
    }
};
        locals.var_here_b4soieta0 = assign6240_e7510;
        locals.var_here_b4soieta0_dn3 = assign6240_e7510_d_n3;
        locals.var_here_b4soieta0_dn4 = assign6240_e7510_d_n4;
        locals.var_here_b4soieta0_dn5 = assign6240_e7510_d_n5;
        locals.var_here_b4soieta0_dn6 = assign6240_e7510_d_n6;
        locals.var_here_b4soieta0_dn7 = assign6240_e7510_d_n7;
        locals.var_here_b4soieta0_dn8 = assign6240_e7510_d_n8;
        locals.var_here_b4soieta0_dn9 = assign6240_e7510_d_n9;
        locals.var_here_b4soieta0_dn10 = assign6240_e7510_d_n10;
        locals.var_here_b4soieta0_dn11 = assign6240_e7510_d_n11;
        locals.var_here_b4soieta0_dn12 = assign6240_e7510_d_n12;

        let (assign6250_e7516, assign6250_e7516_d_n3, assign6250_e7516_d_n4, assign6250_e7516_d_n5, assign6250_e7516_d_n6, assign6250_e7516_d_n7, assign6250_e7516_d_n8, assign6250_e7516_d_n9, assign6250_e7516_d_n10, assign6250_e7516_d_n11, assign6250_e7516_d_n12,) = {
    if (locals.var_guard578 != 0.0) {
        let assign6250_e7514: f64 = (locals.var_pparam_b4soieta0cv + locals.var_deta0cv_lod);
        (assign6250_e7514, (locals.var_pparam_b4soieta0cv_dn3 + locals.var_deta0cv_lod_dn3), (locals.var_pparam_b4soieta0cv_dn4 + locals.var_deta0cv_lod_dn4), (locals.var_pparam_b4soieta0cv_dn5 + locals.var_deta0cv_lod_dn5), (locals.var_pparam_b4soieta0cv_dn6 + locals.var_deta0cv_lod_dn6), (locals.var_pparam_b4soieta0cv_dn7 + locals.var_deta0cv_lod_dn7), (locals.var_pparam_b4soieta0cv_dn8 + locals.var_deta0cv_lod_dn8), (locals.var_pparam_b4soieta0cv_dn9 + locals.var_deta0cv_lod_dn9), (locals.var_pparam_b4soieta0cv_dn10 + locals.var_deta0cv_lod_dn10), (locals.var_pparam_b4soieta0cv_dn11 + locals.var_deta0cv_lod_dn11), (locals.var_pparam_b4soieta0cv_dn12 + locals.var_deta0cv_lod_dn12),)
    } else {
        (locals.var_here_b4soieta0cv, locals.var_here_b4soieta0cv_dn3, locals.var_here_b4soieta0cv_dn4, locals.var_here_b4soieta0cv_dn5, locals.var_here_b4soieta0cv_dn6, locals.var_here_b4soieta0cv_dn7, locals.var_here_b4soieta0cv_dn8, locals.var_here_b4soieta0cv_dn9, locals.var_here_b4soieta0cv_dn10, locals.var_here_b4soieta0cv_dn11, locals.var_here_b4soieta0cv_dn12,)
    }
};
        locals.var_here_b4soieta0cv = assign6250_e7516;
        locals.var_here_b4soieta0cv_dn3 = assign6250_e7516_d_n3;
        locals.var_here_b4soieta0cv_dn4 = assign6250_e7516_d_n4;
        locals.var_here_b4soieta0cv_dn5 = assign6250_e7516_d_n5;
        locals.var_here_b4soieta0cv_dn6 = assign6250_e7516_d_n6;
        locals.var_here_b4soieta0cv_dn7 = assign6250_e7516_d_n7;
        locals.var_here_b4soieta0cv_dn8 = assign6250_e7516_d_n8;
        locals.var_here_b4soieta0cv_dn9 = assign6250_e7516_d_n9;
        locals.var_here_b4soieta0cv_dn10 = assign6250_e7516_d_n10;
        locals.var_here_b4soieta0cv_dn11 = assign6250_e7516_d_n11;
        locals.var_here_b4soieta0cv_dn12 = assign6250_e7516_d_n12;

        let (assign6260_e7521, assign6260_e7521_d_n3, assign6260_e7521_d_n4, assign6260_e7521_d_n5, assign6260_e7521_d_n6, assign6260_e7521_d_n7, assign6260_e7521_d_n8, assign6260_e7521_d_n9, assign6260_e7521_d_n10, assign6260_e7521_d_n11, assign6260_e7521_d_n12,) = {
    if (locals.var_guard578 == 0.0) {
        (locals.var_pparam_b4soiu0temp, locals.var_pparam_b4soiu0temp_dn3, locals.var_pparam_b4soiu0temp_dn4, locals.var_pparam_b4soiu0temp_dn5, locals.var_pparam_b4soiu0temp_dn6, locals.var_pparam_b4soiu0temp_dn7, locals.var_pparam_b4soiu0temp_dn8, locals.var_pparam_b4soiu0temp_dn9, locals.var_pparam_b4soiu0temp_dn10, locals.var_pparam_b4soiu0temp_dn11, locals.var_pparam_b4soiu0temp_dn12,)
    } else {
        (locals.var_here_b4soiu0temp, locals.var_here_b4soiu0temp_dn3, locals.var_here_b4soiu0temp_dn4, locals.var_here_b4soiu0temp_dn5, locals.var_here_b4soiu0temp_dn6, locals.var_here_b4soiu0temp_dn7, locals.var_here_b4soiu0temp_dn8, locals.var_here_b4soiu0temp_dn9, locals.var_here_b4soiu0temp_dn10, locals.var_here_b4soiu0temp_dn11, locals.var_here_b4soiu0temp_dn12,)
    }
};
        locals.var_here_b4soiu0temp = assign6260_e7521;
        locals.var_here_b4soiu0temp_dn3 = assign6260_e7521_d_n3;
        locals.var_here_b4soiu0temp_dn4 = assign6260_e7521_d_n4;
        locals.var_here_b4soiu0temp_dn5 = assign6260_e7521_d_n5;
        locals.var_here_b4soiu0temp_dn6 = assign6260_e7521_d_n6;
        locals.var_here_b4soiu0temp_dn7 = assign6260_e7521_d_n7;
        locals.var_here_b4soiu0temp_dn8 = assign6260_e7521_d_n8;
        locals.var_here_b4soiu0temp_dn9 = assign6260_e7521_d_n9;
        locals.var_here_b4soiu0temp_dn10 = assign6260_e7521_d_n10;
        locals.var_here_b4soiu0temp_dn11 = assign6260_e7521_d_n11;
        locals.var_here_b4soiu0temp_dn12 = assign6260_e7521_d_n12;

        let (assign6270_e7526, assign6270_e7526_d_n3, assign6270_e7526_d_n4, assign6270_e7526_d_n5, assign6270_e7526_d_n6, assign6270_e7526_d_n7, assign6270_e7526_d_n8, assign6270_e7526_d_n9, assign6270_e7526_d_n10, assign6270_e7526_d_n11, assign6270_e7526_d_n12,) = {
    if (locals.var_guard578 == 0.0) {
        (locals.var_pparam_b4soivth0, locals.var_pparam_b4soivth0_dn3, locals.var_pparam_b4soivth0_dn4, locals.var_pparam_b4soivth0_dn5, locals.var_pparam_b4soivth0_dn6, locals.var_pparam_b4soivth0_dn7, locals.var_pparam_b4soivth0_dn8, locals.var_pparam_b4soivth0_dn9, locals.var_pparam_b4soivth0_dn10, locals.var_pparam_b4soivth0_dn11, locals.var_pparam_b4soivth0_dn12,)
    } else {
        (locals.var_here_b4soivth0, locals.var_here_b4soivth0_dn3, locals.var_here_b4soivth0_dn4, locals.var_here_b4soivth0_dn5, locals.var_here_b4soivth0_dn6, locals.var_here_b4soivth0_dn7, locals.var_here_b4soivth0_dn8, locals.var_here_b4soivth0_dn9, locals.var_here_b4soivth0_dn10, locals.var_here_b4soivth0_dn11, locals.var_here_b4soivth0_dn12,)
    }
};
        locals.var_here_b4soivth0 = assign6270_e7526;
        locals.var_here_b4soivth0_dn3 = assign6270_e7526_d_n3;
        locals.var_here_b4soivth0_dn4 = assign6270_e7526_d_n4;
        locals.var_here_b4soivth0_dn5 = assign6270_e7526_d_n5;
        locals.var_here_b4soivth0_dn6 = assign6270_e7526_d_n6;
        locals.var_here_b4soivth0_dn7 = assign6270_e7526_d_n7;
        locals.var_here_b4soivth0_dn8 = assign6270_e7526_d_n8;
        locals.var_here_b4soivth0_dn9 = assign6270_e7526_d_n9;
        locals.var_here_b4soivth0_dn10 = assign6270_e7526_d_n10;
        locals.var_here_b4soivth0_dn11 = assign6270_e7526_d_n11;
        locals.var_here_b4soivth0_dn12 = assign6270_e7526_d_n12;

        let (assign6280_e7531, assign6280_e7531_d_n3, assign6280_e7531_d_n4, assign6280_e7531_d_n5, assign6280_e7531_d_n6, assign6280_e7531_d_n7, assign6280_e7531_d_n8, assign6280_e7531_d_n9, assign6280_e7531_d_n10, assign6280_e7531_d_n11, assign6280_e7531_d_n12,) = {
    if (locals.var_guard578 == 0.0) {
        (locals.var_pparam_b4soivsattemp, locals.var_pparam_b4soivsattemp_dn3, locals.var_pparam_b4soivsattemp_dn4, locals.var_pparam_b4soivsattemp_dn5, locals.var_pparam_b4soivsattemp_dn6, locals.var_pparam_b4soivsattemp_dn7, locals.var_pparam_b4soivsattemp_dn8, locals.var_pparam_b4soivsattemp_dn9, locals.var_pparam_b4soivsattemp_dn10, locals.var_pparam_b4soivsattemp_dn11, locals.var_pparam_b4soivsattemp_dn12,)
    } else {
        (locals.var_here_b4soivsattemp, locals.var_here_b4soivsattemp_dn3, locals.var_here_b4soivsattemp_dn4, locals.var_here_b4soivsattemp_dn5, locals.var_here_b4soivsattemp_dn6, locals.var_here_b4soivsattemp_dn7, locals.var_here_b4soivsattemp_dn8, locals.var_here_b4soivsattemp_dn9, locals.var_here_b4soivsattemp_dn10, locals.var_here_b4soivsattemp_dn11, locals.var_here_b4soivsattemp_dn12,)
    }
};
        locals.var_here_b4soivsattemp = assign6280_e7531;
        locals.var_here_b4soivsattemp_dn3 = assign6280_e7531_d_n3;
        locals.var_here_b4soivsattemp_dn4 = assign6280_e7531_d_n4;
        locals.var_here_b4soivsattemp_dn5 = assign6280_e7531_d_n5;
        locals.var_here_b4soivsattemp_dn6 = assign6280_e7531_d_n6;
        locals.var_here_b4soivsattemp_dn7 = assign6280_e7531_d_n7;
        locals.var_here_b4soivsattemp_dn8 = assign6280_e7531_d_n8;
        locals.var_here_b4soivsattemp_dn9 = assign6280_e7531_d_n9;
        locals.var_here_b4soivsattemp_dn10 = assign6280_e7531_d_n10;
        locals.var_here_b4soivsattemp_dn11 = assign6280_e7531_d_n11;
        locals.var_here_b4soivsattemp_dn12 = assign6280_e7531_d_n12;

        let (assign6290_e7536, assign6290_e7536_d_n3, assign6290_e7536_d_n4, assign6290_e7536_d_n5, assign6290_e7536_d_n6, assign6290_e7536_d_n7, assign6290_e7536_d_n8, assign6290_e7536_d_n9, assign6290_e7536_d_n10, assign6290_e7536_d_n11, assign6290_e7536_d_n12,) = {
    if (locals.var_guard578 == 0.0) {
        (locals.var_pparam_b4soik2, locals.var_pparam_b4soik2_dn3, locals.var_pparam_b4soik2_dn4, locals.var_pparam_b4soik2_dn5, locals.var_pparam_b4soik2_dn6, locals.var_pparam_b4soik2_dn7, locals.var_pparam_b4soik2_dn8, locals.var_pparam_b4soik2_dn9, locals.var_pparam_b4soik2_dn10, locals.var_pparam_b4soik2_dn11, locals.var_pparam_b4soik2_dn12,)
    } else {
        (locals.var_here_b4soik2, locals.var_here_b4soik2_dn3, locals.var_here_b4soik2_dn4, locals.var_here_b4soik2_dn5, locals.var_here_b4soik2_dn6, locals.var_here_b4soik2_dn7, locals.var_here_b4soik2_dn8, locals.var_here_b4soik2_dn9, locals.var_here_b4soik2_dn10, locals.var_here_b4soik2_dn11, locals.var_here_b4soik2_dn12,)
    }
};
        locals.var_here_b4soik2 = assign6290_e7536;
        locals.var_here_b4soik2_dn3 = assign6290_e7536_d_n3;
        locals.var_here_b4soik2_dn4 = assign6290_e7536_d_n4;
        locals.var_here_b4soik2_dn5 = assign6290_e7536_d_n5;
        locals.var_here_b4soik2_dn6 = assign6290_e7536_d_n6;
        locals.var_here_b4soik2_dn7 = assign6290_e7536_d_n7;
        locals.var_here_b4soik2_dn8 = assign6290_e7536_d_n8;
        locals.var_here_b4soik2_dn9 = assign6290_e7536_d_n9;
        locals.var_here_b4soik2_dn10 = assign6290_e7536_d_n10;
        locals.var_here_b4soik2_dn11 = assign6290_e7536_d_n11;
        locals.var_here_b4soik2_dn12 = assign6290_e7536_d_n12;

        let (assign6300_e7541, assign6300_e7541_d_n3, assign6300_e7541_d_n4, assign6300_e7541_d_n5, assign6300_e7541_d_n6, assign6300_e7541_d_n7, assign6300_e7541_d_n8, assign6300_e7541_d_n9, assign6300_e7541_d_n10, assign6300_e7541_d_n11, assign6300_e7541_d_n12,) = {
    if (locals.var_guard578 == 0.0) {
        (locals.var_pparam_b4soieta0, locals.var_pparam_b4soieta0_dn3, locals.var_pparam_b4soieta0_dn4, locals.var_pparam_b4soieta0_dn5, locals.var_pparam_b4soieta0_dn6, locals.var_pparam_b4soieta0_dn7, locals.var_pparam_b4soieta0_dn8, locals.var_pparam_b4soieta0_dn9, locals.var_pparam_b4soieta0_dn10, locals.var_pparam_b4soieta0_dn11, locals.var_pparam_b4soieta0_dn12,)
    } else {
        (locals.var_here_b4soieta0, locals.var_here_b4soieta0_dn3, locals.var_here_b4soieta0_dn4, locals.var_here_b4soieta0_dn5, locals.var_here_b4soieta0_dn6, locals.var_here_b4soieta0_dn7, locals.var_here_b4soieta0_dn8, locals.var_here_b4soieta0_dn9, locals.var_here_b4soieta0_dn10, locals.var_here_b4soieta0_dn11, locals.var_here_b4soieta0_dn12,)
    }
};
        locals.var_here_b4soieta0 = assign6300_e7541;
        locals.var_here_b4soieta0_dn3 = assign6300_e7541_d_n3;
        locals.var_here_b4soieta0_dn4 = assign6300_e7541_d_n4;
        locals.var_here_b4soieta0_dn5 = assign6300_e7541_d_n5;
        locals.var_here_b4soieta0_dn6 = assign6300_e7541_d_n6;
        locals.var_here_b4soieta0_dn7 = assign6300_e7541_d_n7;
        locals.var_here_b4soieta0_dn8 = assign6300_e7541_d_n8;
        locals.var_here_b4soieta0_dn9 = assign6300_e7541_d_n9;
        locals.var_here_b4soieta0_dn10 = assign6300_e7541_d_n10;
        locals.var_here_b4soieta0_dn11 = assign6300_e7541_d_n11;
        locals.var_here_b4soieta0_dn12 = assign6300_e7541_d_n12;

        let (assign6310_e7546, assign6310_e7546_d_n3, assign6310_e7546_d_n4, assign6310_e7546_d_n5, assign6310_e7546_d_n6, assign6310_e7546_d_n7, assign6310_e7546_d_n8, assign6310_e7546_d_n9, assign6310_e7546_d_n10, assign6310_e7546_d_n11, assign6310_e7546_d_n12,) = {
    if (locals.var_guard578 == 0.0) {
        (locals.var_pparam_b4soieta0cv, locals.var_pparam_b4soieta0cv_dn3, locals.var_pparam_b4soieta0cv_dn4, locals.var_pparam_b4soieta0cv_dn5, locals.var_pparam_b4soieta0cv_dn6, locals.var_pparam_b4soieta0cv_dn7, locals.var_pparam_b4soieta0cv_dn8, locals.var_pparam_b4soieta0cv_dn9, locals.var_pparam_b4soieta0cv_dn10, locals.var_pparam_b4soieta0cv_dn11, locals.var_pparam_b4soieta0cv_dn12,)
    } else {
        (locals.var_here_b4soieta0cv, locals.var_here_b4soieta0cv_dn3, locals.var_here_b4soieta0cv_dn4, locals.var_here_b4soieta0cv_dn5, locals.var_here_b4soieta0cv_dn6, locals.var_here_b4soieta0cv_dn7, locals.var_here_b4soieta0cv_dn8, locals.var_here_b4soieta0cv_dn9, locals.var_here_b4soieta0cv_dn10, locals.var_here_b4soieta0cv_dn11, locals.var_here_b4soieta0cv_dn12,)
    }
};
        locals.var_here_b4soieta0cv = assign6310_e7546;
        locals.var_here_b4soieta0cv_dn3 = assign6310_e7546_d_n3;
        locals.var_here_b4soieta0cv_dn4 = assign6310_e7546_d_n4;
        locals.var_here_b4soieta0cv_dn5 = assign6310_e7546_d_n5;
        locals.var_here_b4soieta0cv_dn6 = assign6310_e7546_d_n6;
        locals.var_here_b4soieta0cv_dn7 = assign6310_e7546_d_n7;
        locals.var_here_b4soieta0cv_dn8 = assign6310_e7546_d_n8;
        locals.var_here_b4soieta0cv_dn9 = assign6310_e7546_d_n9;
        locals.var_here_b4soieta0cv_dn10 = assign6310_e7546_d_n10;
        locals.var_here_b4soieta0cv_dn11 = assign6310_e7546_d_n11;
        locals.var_here_b4soieta0cv_dn12 = assign6310_e7546_d_n12;

        let (assign6320_e7551,) = {
    if (locals.var_guard578 == 0.0) {
        (0.0,)
    } else {
        (locals.var_b4soiinv_odeff,)
    }
};
        locals.var_b4soiinv_odeff = assign6320_e7551;

        let (assign6330_e7556,) = {
    if (locals.var_guard578 == 0.0) {
        (0.0,)
    } else {
        (locals.var_pparam_b4soiinv_od_ref,)
    }
};
        locals.var_pparam_b4soiinv_od_ref = assign6330_e7556;

        let (assign6340_e7561,) = {
    if (locals.var_guard578 == 0.0) {
        (0.0,)
    } else {
        (locals.var_b4soikvsat,)
    }
};
        locals.var_b4soikvsat = assign6340_e7561;

        let assign6350_e7564: f64 = (locals.var_here_b4soik2 * p.p66);
        let assign6350_e7566: f64 = (assign6350_e7564 / p.p67);
        locals.var_here_b4soik2ox = assign6350_e7566;
        locals.var_here_b4soik2ox_dn3 = ((locals.var_here_b4soik2_dn3 * p.p66) / p.p67);
        locals.var_here_b4soik2ox_dn4 = ((locals.var_here_b4soik2_dn4 * p.p66) / p.p67);
        locals.var_here_b4soik2ox_dn5 = ((locals.var_here_b4soik2_dn5 * p.p66) / p.p67);
        locals.var_here_b4soik2ox_dn6 = ((locals.var_here_b4soik2_dn6 * p.p66) / p.p67);
        locals.var_here_b4soik2ox_dn7 = ((locals.var_here_b4soik2_dn7 * p.p66) / p.p67);
        locals.var_here_b4soik2ox_dn8 = ((locals.var_here_b4soik2_dn8 * p.p66) / p.p67);
        locals.var_here_b4soik2ox_dn9 = ((locals.var_here_b4soik2_dn9 * p.p66) / p.p67);
        locals.var_here_b4soik2ox_dn10 = ((locals.var_here_b4soik2_dn10 * p.p66) / p.p67);
        locals.var_here_b4soik2ox_dn11 = ((locals.var_here_b4soik2_dn11 * p.p66) / p.p67);
        locals.var_here_b4soik2ox_dn12 = ((locals.var_here_b4soik2_dn12 * p.p66) / p.p67);

        let assign6360_e7569: f64 = (locals.var_here_b4soivth0 + p.p20);
        locals.var_here_b4soivth0 = assign6360_e7569;
        locals.var_here_b4soivth0_dn3 = locals.var_here_b4soivth0_dn3;
        locals.var_here_b4soivth0_dn4 = locals.var_here_b4soivth0_dn4;
        locals.var_here_b4soivth0_dn5 = locals.var_here_b4soivth0_dn5;
        locals.var_here_b4soivth0_dn6 = locals.var_here_b4soivth0_dn6;
        locals.var_here_b4soivth0_dn7 = locals.var_here_b4soivth0_dn7;
        locals.var_here_b4soivth0_dn8 = locals.var_here_b4soivth0_dn8;
        locals.var_here_b4soivth0_dn9 = locals.var_here_b4soivth0_dn9;
        locals.var_here_b4soivth0_dn10 = locals.var_here_b4soivth0_dn10;
        locals.var_here_b4soivth0_dn11 = locals.var_here_b4soivth0_dn11;
        locals.var_here_b4soivth0_dn12 = locals.var_here_b4soivth0_dn12;

        let assign6370_e7573: f64 = (p.p37 * p.p20);
        let assign6370_e7574: f64 = (locals.var_pparam_b4soivfb + assign6370_e7573);
        locals.var_here_b4soivfb = assign6370_e7574;
        locals.var_here_b4soivfb_dn3 = locals.var_pparam_b4soivfb_dn3;
        locals.var_here_b4soivfb_dn4 = locals.var_pparam_b4soivfb_dn4;
        locals.var_here_b4soivfb_dn5 = locals.var_pparam_b4soivfb_dn5;
        locals.var_here_b4soivfb_dn6 = locals.var_pparam_b4soivfb_dn6;
        locals.var_here_b4soivfb_dn7 = locals.var_pparam_b4soivfb_dn7;
        locals.var_here_b4soivfb_dn8 = locals.var_pparam_b4soivfb_dn8;
        locals.var_here_b4soivfb_dn9 = locals.var_pparam_b4soivfb_dn9;
        locals.var_here_b4soivfb_dn10 = locals.var_pparam_b4soivfb_dn10;
        locals.var_here_b4soivfb_dn11 = locals.var_pparam_b4soivfb_dn11;
        locals.var_here_b4soivfb_dn12 = locals.var_pparam_b4soivfb_dn12;

        let assign6380_e7577: f64 = (locals.var_b4soicbox * p.p8);
        locals.var_b4soicsbox = assign6380_e7577;

        let assign6390_e7580: f64 = (locals.var_b4soicsdmin * p.p8);
        locals.var_b4soicsmin = assign6390_e7580;
        locals.var_b4soicsmin_dn3 = (locals.var_b4soicsdmin_dn3 * p.p8);
        locals.var_b4soicsmin_dn4 = (locals.var_b4soicsdmin_dn4 * p.p8);
        locals.var_b4soicsmin_dn5 = (locals.var_b4soicsdmin_dn5 * p.p8);
        locals.var_b4soicsmin_dn6 = (locals.var_b4soicsdmin_dn6 * p.p8);
        locals.var_b4soicsmin_dn7 = (locals.var_b4soicsdmin_dn7 * p.p8);
        locals.var_b4soicsmin_dn8 = (locals.var_b4soicsdmin_dn8 * p.p8);
        locals.var_b4soicsmin_dn9 = (locals.var_b4soicsdmin_dn9 * p.p8);
        locals.var_b4soicsmin_dn10 = (locals.var_b4soicsdmin_dn10 * p.p8);
        locals.var_b4soicsmin_dn11 = (locals.var_b4soicsdmin_dn11 * p.p8);
        locals.var_b4soicsmin_dn12 = (locals.var_b4soicsdmin_dn12 * p.p8);

        let assign6400_e7583: f64 = (locals.var_b4soicbox * p.p7);
        locals.var_b4soicdbox = assign6400_e7583;

        let assign6410_e7586: f64 = (locals.var_b4soicsdmin * p.p7);
        locals.var_b4soicdmin = assign6410_e7586;
        locals.var_b4soicdmin_dn3 = (locals.var_b4soicsdmin_dn3 * p.p7);
        locals.var_b4soicdmin_dn4 = (locals.var_b4soicsdmin_dn4 * p.p7);
        locals.var_b4soicdmin_dn5 = (locals.var_b4soicsdmin_dn5 * p.p7);
        locals.var_b4soicdmin_dn6 = (locals.var_b4soicsdmin_dn6 * p.p7);
        locals.var_b4soicdmin_dn7 = (locals.var_b4soicsdmin_dn7 * p.p7);
        locals.var_b4soicdmin_dn8 = (locals.var_b4soicsdmin_dn8 * p.p7);
        locals.var_b4soicdmin_dn9 = (locals.var_b4soicsdmin_dn9 * p.p7);
        locals.var_b4soicdmin_dn10 = (locals.var_b4soicsdmin_dn10 * p.p7);
        locals.var_b4soicdmin_dn11 = (locals.var_b4soicsdmin_dn11 * p.p7);
        locals.var_b4soicdmin_dn12 = (locals.var_b4soicsdmin_dn12 * p.p7);

        let assign6420_e7589: f64 = if locals.var_b4soicsdmin > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard583 = assign6420_e7589;

        let assign6430_e7604: f64 = if (((locals.var_pparam_b4soinsub > 0.0) && (p.p37 > 0.0)) || ((locals.var_pparam_b4soinsub < 0.0) && (p.p37 < 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard584 = assign6430_e7604;

        let (assign6440_e7612, assign6440_e7612_d_n3, assign6440_e7612_d_n4, assign6440_e7612_d_n5, assign6440_e7612_d_n6, assign6440_e7612_d_n7, assign6440_e7612_d_n8, assign6440_e7612_d_n9, assign6440_e7612_d_n10, assign6440_e7612_d_n11, assign6440_e7612_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign6440_e7610: f64 = (locals.var_pparam_b4soivsdth - locals.var_pparam_b4soivsdfb);
        (assign6440_e7610, (locals.var_pparam_b4soivsdth_dn3 - locals.var_pparam_b4soivsdfb_dn3), (locals.var_pparam_b4soivsdth_dn4 - locals.var_pparam_b4soivsdfb_dn4), (locals.var_pparam_b4soivsdth_dn5 - locals.var_pparam_b4soivsdfb_dn5), (locals.var_pparam_b4soivsdth_dn6 - locals.var_pparam_b4soivsdfb_dn6), (locals.var_pparam_b4soivsdth_dn7 - locals.var_pparam_b4soivsdfb_dn7), (locals.var_pparam_b4soivsdth_dn8 - locals.var_pparam_b4soivsdfb_dn8), (locals.var_pparam_b4soivsdth_dn9 - locals.var_pparam_b4soivsdfb_dn9), (locals.var_pparam_b4soivsdth_dn10 - locals.var_pparam_b4soivsdfb_dn10), (locals.var_pparam_b4soivsdth_dn11 - locals.var_pparam_b4soivsdfb_dn11), (locals.var_pparam_b4soivsdth_dn12 - locals.var_pparam_b4soivsdfb_dn12),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign6440_e7612;
        locals.var_t0_dn3 = assign6440_e7612_d_n3;
        locals.var_t0_dn4 = assign6440_e7612_d_n4;
        locals.var_t0_dn5 = assign6440_e7612_d_n5;
        locals.var_t0_dn6 = assign6440_e7612_d_n6;
        locals.var_t0_dn7 = assign6440_e7612_d_n7;
        locals.var_t0_dn8 = assign6440_e7612_d_n8;
        locals.var_t0_dn9 = assign6440_e7612_d_n9;
        locals.var_t0_dn10 = assign6440_e7612_d_n10;
        locals.var_t0_dn11 = assign6440_e7612_d_n11;
        locals.var_t0_dn12 = assign6440_e7612_d_n12;

        let (assign6450_e7622,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign6450_e7619: f64 = (p.p356 * locals.var_t0);
        let assign6450_e7620: f64 = (locals.var_pparam_b4soivsdfb + assign6450_e7619);
        (assign6450_e7620,)
    } else {
        (locals.var_pparam_b4soisdt1,)
    }
};
        locals.var_pparam_b4soisdt1 = assign6450_e7622;

        let (assign6460_e7630, assign6460_e7630_d_n3, assign6460_e7630_d_n4, assign6460_e7630_d_n5, assign6460_e7630_d_n6, assign6460_e7630_d_n7, assign6460_e7630_d_n8, assign6460_e7630_d_n9, assign6460_e7630_d_n10, assign6460_e7630_d_n11, assign6460_e7630_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign6460_e7628: f64 = (locals.var_b4soicsbox - locals.var_b4soicsmin);
        (assign6460_e7628, (-locals.var_b4soicsmin_dn3), (-locals.var_b4soicsmin_dn4), (-locals.var_b4soicsmin_dn5), (-locals.var_b4soicsmin_dn6), (-locals.var_b4soicsmin_dn7), (-locals.var_b4soicsmin_dn8), (-locals.var_b4soicsmin_dn9), (-locals.var_b4soicsmin_dn10), (-locals.var_b4soicsmin_dn11), (-locals.var_b4soicsmin_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign6460_e7630;
        locals.var_t1_dn3 = assign6460_e7630_d_n3;
        locals.var_t1_dn4 = assign6460_e7630_d_n4;
        locals.var_t1_dn5 = assign6460_e7630_d_n5;
        locals.var_t1_dn6 = assign6460_e7630_d_n6;
        locals.var_t1_dn7 = assign6460_e7630_d_n7;
        locals.var_t1_dn8 = assign6460_e7630_d_n8;
        locals.var_t1_dn9 = assign6460_e7630_d_n9;
        locals.var_t1_dn10 = assign6460_e7630_d_n10;
        locals.var_t1_dn11 = assign6460_e7630_d_n11;
        locals.var_t1_dn12 = assign6460_e7630_d_n12;

        let (assign6470_e7640, assign6470_e7640_d_n3, assign6470_e7640_d_n4, assign6470_e7640_d_n5, assign6470_e7640_d_n6, assign6470_e7640_d_n7, assign6470_e7640_d_n8, assign6470_e7640_d_n9, assign6470_e7640_d_n10, assign6470_e7640_d_n11, assign6470_e7640_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 != 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t0;
        let assign6470_e7636: f64 = (locals.var_t1 * __rspice_inv_cse_0);
        let assign6470_e7638: f64 = (assign6470_e7636 * __rspice_inv_cse_0);
        (assign6470_e7638, ((((((locals.var_t1_dn3 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6470_e7636 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn4 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6470_e7636 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn5 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6470_e7636 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn6 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6470_e7636 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn7 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6470_e7636 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn8 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6470_e7636 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn9 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6470_e7636 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn10 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6470_e7636 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn11 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6470_e7636 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn12 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6470_e7636 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign6470_e7640;
        locals.var_t2_dn3 = assign6470_e7640_d_n3;
        locals.var_t2_dn4 = assign6470_e7640_d_n4;
        locals.var_t2_dn5 = assign6470_e7640_d_n5;
        locals.var_t2_dn6 = assign6470_e7640_d_n6;
        locals.var_t2_dn7 = assign6470_e7640_d_n7;
        locals.var_t2_dn8 = assign6470_e7640_d_n8;
        locals.var_t2_dn9 = assign6470_e7640_d_n9;
        locals.var_t2_dn10 = assign6470_e7640_d_n10;
        locals.var_t2_dn11 = assign6470_e7640_d_n11;
        locals.var_t2_dn12 = assign6470_e7640_d_n12;

        let (assign6480_e7648, assign6480_e7648_d_n3, assign6480_e7648_d_n4, assign6480_e7648_d_n5, assign6480_e7648_d_n6, assign6480_e7648_d_n7, assign6480_e7648_d_n8, assign6480_e7648_d_n9, assign6480_e7648_d_n10, assign6480_e7648_d_n11, assign6480_e7648_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign6480_e7646: f64 = (locals.var_t2 / p.p356);
        (assign6480_e7646, (locals.var_t2_dn3 / p.p356), (locals.var_t2_dn4 / p.p356), (locals.var_t2_dn5 / p.p356), (locals.var_t2_dn6 / p.p356), (locals.var_t2_dn7 / p.p356), (locals.var_t2_dn8 / p.p356), (locals.var_t2_dn9 / p.p356), (locals.var_t2_dn10 / p.p356), (locals.var_t2_dn11 / p.p356), (locals.var_t2_dn12 / p.p356),)
    } else {
        (locals.var_pparam_b4soist2, locals.var_pparam_b4soist2_dn3, locals.var_pparam_b4soist2_dn4, locals.var_pparam_b4soist2_dn5, locals.var_pparam_b4soist2_dn6, locals.var_pparam_b4soist2_dn7, locals.var_pparam_b4soist2_dn8, locals.var_pparam_b4soist2_dn9, locals.var_pparam_b4soist2_dn10, locals.var_pparam_b4soist2_dn11, locals.var_pparam_b4soist2_dn12,)
    }
};
        locals.var_pparam_b4soist2 = assign6480_e7648;
        locals.var_pparam_b4soist2_dn3 = assign6480_e7648_d_n3;
        locals.var_pparam_b4soist2_dn4 = assign6480_e7648_d_n4;
        locals.var_pparam_b4soist2_dn5 = assign6480_e7648_d_n5;
        locals.var_pparam_b4soist2_dn6 = assign6480_e7648_d_n6;
        locals.var_pparam_b4soist2_dn7 = assign6480_e7648_d_n7;
        locals.var_pparam_b4soist2_dn8 = assign6480_e7648_d_n8;
        locals.var_pparam_b4soist2_dn9 = assign6480_e7648_d_n9;
        locals.var_pparam_b4soist2_dn10 = assign6480_e7648_d_n10;
        locals.var_pparam_b4soist2_dn11 = assign6480_e7648_d_n11;
        locals.var_pparam_b4soist2_dn12 = assign6480_e7648_d_n12;

        let (assign6490_e7658, assign6490_e7658_d_n3, assign6490_e7658_d_n4, assign6490_e7658_d_n5, assign6490_e7658_d_n6, assign6490_e7658_d_n7, assign6490_e7658_d_n8, assign6490_e7658_d_n9, assign6490_e7658_d_n10, assign6490_e7658_d_n11, assign6490_e7658_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign6490_e7655: f64 = (1.0 - p.p356);
        let assign6490_e7656: f64 = (locals.var_t2 / assign6490_e7655);
        (assign6490_e7656, (locals.var_t2_dn3 / assign6490_e7655), (locals.var_t2_dn4 / assign6490_e7655), (locals.var_t2_dn5 / assign6490_e7655), (locals.var_t2_dn6 / assign6490_e7655), (locals.var_t2_dn7 / assign6490_e7655), (locals.var_t2_dn8 / assign6490_e7655), (locals.var_t2_dn9 / assign6490_e7655), (locals.var_t2_dn10 / assign6490_e7655), (locals.var_t2_dn11 / assign6490_e7655), (locals.var_t2_dn12 / assign6490_e7655),)
    } else {
        (locals.var_pparam_b4soist3, locals.var_pparam_b4soist3_dn3, locals.var_pparam_b4soist3_dn4, locals.var_pparam_b4soist3_dn5, locals.var_pparam_b4soist3_dn6, locals.var_pparam_b4soist3_dn7, locals.var_pparam_b4soist3_dn8, locals.var_pparam_b4soist3_dn9, locals.var_pparam_b4soist3_dn10, locals.var_pparam_b4soist3_dn11, locals.var_pparam_b4soist3_dn12,)
    }
};
        locals.var_pparam_b4soist3 = assign6490_e7658;
        locals.var_pparam_b4soist3_dn3 = assign6490_e7658_d_n3;
        locals.var_pparam_b4soist3_dn4 = assign6490_e7658_d_n4;
        locals.var_pparam_b4soist3_dn5 = assign6490_e7658_d_n5;
        locals.var_pparam_b4soist3_dn6 = assign6490_e7658_d_n6;
        locals.var_pparam_b4soist3_dn7 = assign6490_e7658_d_n7;
        locals.var_pparam_b4soist3_dn8 = assign6490_e7658_d_n8;
        locals.var_pparam_b4soist3_dn9 = assign6490_e7658_d_n9;
        locals.var_pparam_b4soist3_dn10 = assign6490_e7658_d_n10;
        locals.var_pparam_b4soist3_dn11 = assign6490_e7658_d_n11;
        locals.var_pparam_b4soist3_dn12 = assign6490_e7658_d_n12;

    }

    pub(super) fn stamp_transient_block_18(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign6500_e7676, assign6500_e7676_d_n3, assign6500_e7676_d_n4, assign6500_e7676_d_n5, assign6500_e7676_d_n6, assign6500_e7676_d_n7, assign6500_e7676_d_n8, assign6500_e7676_d_n9, assign6500_e7676_d_n10, assign6500_e7676_d_n11, assign6500_e7676_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign6500_e7664: f64 = (locals.var_t0 * locals.var_t1);
        let assign6500_e7667: f64 = (1.0 + p.p356);
        let assign6500_e7668: f64 = (assign6500_e7664 * assign6500_e7667);
        let assign6500_e7670: f64 = (assign6500_e7668 / 3.0);
        let assign6500_e7673: f64 = (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb);
        let assign6500_e7674: f64 = (assign6500_e7670 - assign6500_e7673);
        (assign6500_e7674, (((((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3)) * assign6500_e7667) / 3.0) - ((locals.var_b4soicsmin_dn3 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb_dn3))), (((((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)) * assign6500_e7667) / 3.0) - ((locals.var_b4soicsmin_dn4 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb_dn4))), (((((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)) * assign6500_e7667) / 3.0) - ((locals.var_b4soicsmin_dn5 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb_dn5))), (((((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)) * assign6500_e7667) / 3.0) - ((locals.var_b4soicsmin_dn6 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb_dn6))), (((((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)) * assign6500_e7667) / 3.0) - ((locals.var_b4soicsmin_dn7 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb_dn7))), (((((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)) * assign6500_e7667) / 3.0) - ((locals.var_b4soicsmin_dn8 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb_dn8))), (((((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)) * assign6500_e7667) / 3.0) - ((locals.var_b4soicsmin_dn9 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb_dn9))), (((((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)) * assign6500_e7667) / 3.0) - ((locals.var_b4soicsmin_dn10 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb_dn10))), (((((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)) * assign6500_e7667) / 3.0) - ((locals.var_b4soicsmin_dn11 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb_dn11))), (((((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12)) * assign6500_e7667) / 3.0) - ((locals.var_b4soicsmin_dn12 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicsmin * locals.var_pparam_b4soivsdfb_dn12))),)
    } else {
        (locals.var_b4soist4, locals.var_b4soist4_dn3, locals.var_b4soist4_dn4, locals.var_b4soist4_dn5, locals.var_b4soist4_dn6, locals.var_b4soist4_dn7, locals.var_b4soist4_dn8, locals.var_b4soist4_dn9, locals.var_b4soist4_dn10, locals.var_b4soist4_dn11, locals.var_b4soist4_dn12,)
    }
};
        locals.var_b4soist4 = assign6500_e7676;
        locals.var_b4soist4_dn3 = assign6500_e7676_d_n3;
        locals.var_b4soist4_dn4 = assign6500_e7676_d_n4;
        locals.var_b4soist4_dn5 = assign6500_e7676_d_n5;
        locals.var_b4soist4_dn6 = assign6500_e7676_d_n6;
        locals.var_b4soist4_dn7 = assign6500_e7676_d_n7;
        locals.var_b4soist4_dn8 = assign6500_e7676_d_n8;
        locals.var_b4soist4_dn9 = assign6500_e7676_d_n9;
        locals.var_b4soist4_dn10 = assign6500_e7676_d_n10;
        locals.var_b4soist4_dn11 = assign6500_e7676_d_n11;
        locals.var_b4soist4_dn12 = assign6500_e7676_d_n12;

        let (assign6510_e7684, assign6510_e7684_d_n3, assign6510_e7684_d_n4, assign6510_e7684_d_n5, assign6510_e7684_d_n6, assign6510_e7684_d_n7, assign6510_e7684_d_n8, assign6510_e7684_d_n9, assign6510_e7684_d_n10, assign6510_e7684_d_n11, assign6510_e7684_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign6510_e7682: f64 = (locals.var_b4soicdbox - locals.var_b4soicdmin);
        (assign6510_e7682, (-locals.var_b4soicdmin_dn3), (-locals.var_b4soicdmin_dn4), (-locals.var_b4soicdmin_dn5), (-locals.var_b4soicdmin_dn6), (-locals.var_b4soicdmin_dn7), (-locals.var_b4soicdmin_dn8), (-locals.var_b4soicdmin_dn9), (-locals.var_b4soicdmin_dn10), (-locals.var_b4soicdmin_dn11), (-locals.var_b4soicdmin_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign6510_e7684;
        locals.var_t1_dn3 = assign6510_e7684_d_n3;
        locals.var_t1_dn4 = assign6510_e7684_d_n4;
        locals.var_t1_dn5 = assign6510_e7684_d_n5;
        locals.var_t1_dn6 = assign6510_e7684_d_n6;
        locals.var_t1_dn7 = assign6510_e7684_d_n7;
        locals.var_t1_dn8 = assign6510_e7684_d_n8;
        locals.var_t1_dn9 = assign6510_e7684_d_n9;
        locals.var_t1_dn10 = assign6510_e7684_d_n10;
        locals.var_t1_dn11 = assign6510_e7684_d_n11;
        locals.var_t1_dn12 = assign6510_e7684_d_n12;

        let (assign6520_e7694, assign6520_e7694_d_n3, assign6520_e7694_d_n4, assign6520_e7694_d_n5, assign6520_e7694_d_n6, assign6520_e7694_d_n7, assign6520_e7694_d_n8, assign6520_e7694_d_n9, assign6520_e7694_d_n10, assign6520_e7694_d_n11, assign6520_e7694_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 != 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t0;
        let assign6520_e7690: f64 = (locals.var_t1 * __rspice_inv_cse_0);
        let assign6520_e7692: f64 = (assign6520_e7690 * __rspice_inv_cse_0);
        (assign6520_e7692, ((((((locals.var_t1_dn3 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6520_e7690 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn4 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6520_e7690 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn5 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6520_e7690 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn6 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6520_e7690 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn7 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6520_e7690 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn8 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6520_e7690 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn9 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6520_e7690 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn10 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6520_e7690 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn11 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6520_e7690 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn12 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6520_e7690 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign6520_e7694;
        locals.var_t2_dn3 = assign6520_e7694_d_n3;
        locals.var_t2_dn4 = assign6520_e7694_d_n4;
        locals.var_t2_dn5 = assign6520_e7694_d_n5;
        locals.var_t2_dn6 = assign6520_e7694_d_n6;
        locals.var_t2_dn7 = assign6520_e7694_d_n7;
        locals.var_t2_dn8 = assign6520_e7694_d_n8;
        locals.var_t2_dn9 = assign6520_e7694_d_n9;
        locals.var_t2_dn10 = assign6520_e7694_d_n10;
        locals.var_t2_dn11 = assign6520_e7694_d_n11;
        locals.var_t2_dn12 = assign6520_e7694_d_n12;

        let (assign6530_e7702, assign6530_e7702_d_n3, assign6530_e7702_d_n4, assign6530_e7702_d_n5, assign6530_e7702_d_n6, assign6530_e7702_d_n7, assign6530_e7702_d_n8, assign6530_e7702_d_n9, assign6530_e7702_d_n10, assign6530_e7702_d_n11, assign6530_e7702_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign6530_e7700: f64 = (locals.var_t2 / p.p356);
        (assign6530_e7700, (locals.var_t2_dn3 / p.p356), (locals.var_t2_dn4 / p.p356), (locals.var_t2_dn5 / p.p356), (locals.var_t2_dn6 / p.p356), (locals.var_t2_dn7 / p.p356), (locals.var_t2_dn8 / p.p356), (locals.var_t2_dn9 / p.p356), (locals.var_t2_dn10 / p.p356), (locals.var_t2_dn11 / p.p356), (locals.var_t2_dn12 / p.p356),)
    } else {
        (locals.var_pparam_b4soidt2, locals.var_pparam_b4soidt2_dn3, locals.var_pparam_b4soidt2_dn4, locals.var_pparam_b4soidt2_dn5, locals.var_pparam_b4soidt2_dn6, locals.var_pparam_b4soidt2_dn7, locals.var_pparam_b4soidt2_dn8, locals.var_pparam_b4soidt2_dn9, locals.var_pparam_b4soidt2_dn10, locals.var_pparam_b4soidt2_dn11, locals.var_pparam_b4soidt2_dn12,)
    }
};
        locals.var_pparam_b4soidt2 = assign6530_e7702;
        locals.var_pparam_b4soidt2_dn3 = assign6530_e7702_d_n3;
        locals.var_pparam_b4soidt2_dn4 = assign6530_e7702_d_n4;
        locals.var_pparam_b4soidt2_dn5 = assign6530_e7702_d_n5;
        locals.var_pparam_b4soidt2_dn6 = assign6530_e7702_d_n6;
        locals.var_pparam_b4soidt2_dn7 = assign6530_e7702_d_n7;
        locals.var_pparam_b4soidt2_dn8 = assign6530_e7702_d_n8;
        locals.var_pparam_b4soidt2_dn9 = assign6530_e7702_d_n9;
        locals.var_pparam_b4soidt2_dn10 = assign6530_e7702_d_n10;
        locals.var_pparam_b4soidt2_dn11 = assign6530_e7702_d_n11;
        locals.var_pparam_b4soidt2_dn12 = assign6530_e7702_d_n12;

        let (assign6540_e7712, assign6540_e7712_d_n3, assign6540_e7712_d_n4, assign6540_e7712_d_n5, assign6540_e7712_d_n6, assign6540_e7712_d_n7, assign6540_e7712_d_n8, assign6540_e7712_d_n9, assign6540_e7712_d_n10, assign6540_e7712_d_n11, assign6540_e7712_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign6540_e7709: f64 = (1.0 - p.p356);
        let assign6540_e7710: f64 = (locals.var_t2 / assign6540_e7709);
        (assign6540_e7710, (locals.var_t2_dn3 / assign6540_e7709), (locals.var_t2_dn4 / assign6540_e7709), (locals.var_t2_dn5 / assign6540_e7709), (locals.var_t2_dn6 / assign6540_e7709), (locals.var_t2_dn7 / assign6540_e7709), (locals.var_t2_dn8 / assign6540_e7709), (locals.var_t2_dn9 / assign6540_e7709), (locals.var_t2_dn10 / assign6540_e7709), (locals.var_t2_dn11 / assign6540_e7709), (locals.var_t2_dn12 / assign6540_e7709),)
    } else {
        (locals.var_pparam_b4soidt3, locals.var_pparam_b4soidt3_dn3, locals.var_pparam_b4soidt3_dn4, locals.var_pparam_b4soidt3_dn5, locals.var_pparam_b4soidt3_dn6, locals.var_pparam_b4soidt3_dn7, locals.var_pparam_b4soidt3_dn8, locals.var_pparam_b4soidt3_dn9, locals.var_pparam_b4soidt3_dn10, locals.var_pparam_b4soidt3_dn11, locals.var_pparam_b4soidt3_dn12,)
    }
};
        locals.var_pparam_b4soidt3 = assign6540_e7712;
        locals.var_pparam_b4soidt3_dn3 = assign6540_e7712_d_n3;
        locals.var_pparam_b4soidt3_dn4 = assign6540_e7712_d_n4;
        locals.var_pparam_b4soidt3_dn5 = assign6540_e7712_d_n5;
        locals.var_pparam_b4soidt3_dn6 = assign6540_e7712_d_n6;
        locals.var_pparam_b4soidt3_dn7 = assign6540_e7712_d_n7;
        locals.var_pparam_b4soidt3_dn8 = assign6540_e7712_d_n8;
        locals.var_pparam_b4soidt3_dn9 = assign6540_e7712_d_n9;
        locals.var_pparam_b4soidt3_dn10 = assign6540_e7712_d_n10;
        locals.var_pparam_b4soidt3_dn11 = assign6540_e7712_d_n11;
        locals.var_pparam_b4soidt3_dn12 = assign6540_e7712_d_n12;

        let (assign6550_e7730, assign6550_e7730_d_n3, assign6550_e7730_d_n4, assign6550_e7730_d_n5, assign6550_e7730_d_n6, assign6550_e7730_d_n7, assign6550_e7730_d_n8, assign6550_e7730_d_n9, assign6550_e7730_d_n10, assign6550_e7730_d_n11, assign6550_e7730_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 != 0.0)) {
        let assign6550_e7718: f64 = (locals.var_t0 * locals.var_t1);
        let assign6550_e7721: f64 = (1.0 + p.p356);
        let assign6550_e7722: f64 = (assign6550_e7718 * assign6550_e7721);
        let assign6550_e7724: f64 = (assign6550_e7722 / 3.0);
        let assign6550_e7727: f64 = (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb);
        let assign6550_e7728: f64 = (assign6550_e7724 - assign6550_e7727);
        (assign6550_e7728, (((((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3)) * assign6550_e7721) / 3.0) - ((locals.var_b4soicdmin_dn3 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb_dn3))), (((((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)) * assign6550_e7721) / 3.0) - ((locals.var_b4soicdmin_dn4 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb_dn4))), (((((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)) * assign6550_e7721) / 3.0) - ((locals.var_b4soicdmin_dn5 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb_dn5))), (((((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)) * assign6550_e7721) / 3.0) - ((locals.var_b4soicdmin_dn6 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb_dn6))), (((((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)) * assign6550_e7721) / 3.0) - ((locals.var_b4soicdmin_dn7 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb_dn7))), (((((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)) * assign6550_e7721) / 3.0) - ((locals.var_b4soicdmin_dn8 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb_dn8))), (((((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)) * assign6550_e7721) / 3.0) - ((locals.var_b4soicdmin_dn9 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb_dn9))), (((((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)) * assign6550_e7721) / 3.0) - ((locals.var_b4soicdmin_dn10 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb_dn10))), (((((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)) * assign6550_e7721) / 3.0) - ((locals.var_b4soicdmin_dn11 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb_dn11))), (((((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12)) * assign6550_e7721) / 3.0) - ((locals.var_b4soicdmin_dn12 * locals.var_pparam_b4soivsdfb) + (locals.var_b4soicdmin * locals.var_pparam_b4soivsdfb_dn12))),)
    } else {
        (locals.var_b4soidt4, locals.var_b4soidt4_dn3, locals.var_b4soidt4_dn4, locals.var_b4soidt4_dn5, locals.var_b4soidt4_dn6, locals.var_b4soidt4_dn7, locals.var_b4soidt4_dn8, locals.var_b4soidt4_dn9, locals.var_b4soidt4_dn10, locals.var_b4soidt4_dn11, locals.var_b4soidt4_dn12,)
    }
};
        locals.var_b4soidt4 = assign6550_e7730;
        locals.var_b4soidt4_dn3 = assign6550_e7730_d_n3;
        locals.var_b4soidt4_dn4 = assign6550_e7730_d_n4;
        locals.var_b4soidt4_dn5 = assign6550_e7730_d_n5;
        locals.var_b4soidt4_dn6 = assign6550_e7730_d_n6;
        locals.var_b4soidt4_dn7 = assign6550_e7730_d_n7;
        locals.var_b4soidt4_dn8 = assign6550_e7730_d_n8;
        locals.var_b4soidt4_dn9 = assign6550_e7730_d_n9;
        locals.var_b4soidt4_dn10 = assign6550_e7730_d_n10;
        locals.var_b4soidt4_dn11 = assign6550_e7730_d_n11;
        locals.var_b4soidt4_dn12 = assign6550_e7730_d_n12;

        let (assign6560_e7739, assign6560_e7739_d_n3, assign6560_e7739_d_n4, assign6560_e7739_d_n5, assign6560_e7739_d_n6, assign6560_e7739_d_n7, assign6560_e7739_d_n8, assign6560_e7739_d_n9, assign6560_e7739_d_n10, assign6560_e7739_d_n11, assign6560_e7739_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 == 0.0)) {
        let assign6560_e7737: f64 = (locals.var_pparam_b4soivsdfb - locals.var_pparam_b4soivsdth);
        (assign6560_e7737, (locals.var_pparam_b4soivsdfb_dn3 - locals.var_pparam_b4soivsdth_dn3), (locals.var_pparam_b4soivsdfb_dn4 - locals.var_pparam_b4soivsdth_dn4), (locals.var_pparam_b4soivsdfb_dn5 - locals.var_pparam_b4soivsdth_dn5), (locals.var_pparam_b4soivsdfb_dn6 - locals.var_pparam_b4soivsdth_dn6), (locals.var_pparam_b4soivsdfb_dn7 - locals.var_pparam_b4soivsdth_dn7), (locals.var_pparam_b4soivsdfb_dn8 - locals.var_pparam_b4soivsdth_dn8), (locals.var_pparam_b4soivsdfb_dn9 - locals.var_pparam_b4soivsdth_dn9), (locals.var_pparam_b4soivsdfb_dn10 - locals.var_pparam_b4soivsdth_dn10), (locals.var_pparam_b4soivsdfb_dn11 - locals.var_pparam_b4soivsdth_dn11), (locals.var_pparam_b4soivsdfb_dn12 - locals.var_pparam_b4soivsdth_dn12),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign6560_e7739;
        locals.var_t0_dn3 = assign6560_e7739_d_n3;
        locals.var_t0_dn4 = assign6560_e7739_d_n4;
        locals.var_t0_dn5 = assign6560_e7739_d_n5;
        locals.var_t0_dn6 = assign6560_e7739_d_n6;
        locals.var_t0_dn7 = assign6560_e7739_d_n7;
        locals.var_t0_dn8 = assign6560_e7739_d_n8;
        locals.var_t0_dn9 = assign6560_e7739_d_n9;
        locals.var_t0_dn10 = assign6560_e7739_d_n10;
        locals.var_t0_dn11 = assign6560_e7739_d_n11;
        locals.var_t0_dn12 = assign6560_e7739_d_n12;

        let (assign6570_e7750,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 == 0.0)) {
        let assign6570_e7747: f64 = (p.p356 * locals.var_t0);
        let assign6570_e7748: f64 = (locals.var_pparam_b4soivsdth + assign6570_e7747);
        (assign6570_e7748,)
    } else {
        (locals.var_pparam_b4soisdt1,)
    }
};
        locals.var_pparam_b4soisdt1 = assign6570_e7750;

        let (assign6580_e7759, assign6580_e7759_d_n3, assign6580_e7759_d_n4, assign6580_e7759_d_n5, assign6580_e7759_d_n6, assign6580_e7759_d_n7, assign6580_e7759_d_n8, assign6580_e7759_d_n9, assign6580_e7759_d_n10, assign6580_e7759_d_n11, assign6580_e7759_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 == 0.0)) {
        let assign6580_e7757: f64 = (locals.var_b4soicsmin - locals.var_b4soicsbox);
        (assign6580_e7757, locals.var_b4soicsmin_dn3, locals.var_b4soicsmin_dn4, locals.var_b4soicsmin_dn5, locals.var_b4soicsmin_dn6, locals.var_b4soicsmin_dn7, locals.var_b4soicsmin_dn8, locals.var_b4soicsmin_dn9, locals.var_b4soicsmin_dn10, locals.var_b4soicsmin_dn11, locals.var_b4soicsmin_dn12,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign6580_e7759;
        locals.var_t1_dn3 = assign6580_e7759_d_n3;
        locals.var_t1_dn4 = assign6580_e7759_d_n4;
        locals.var_t1_dn5 = assign6580_e7759_d_n5;
        locals.var_t1_dn6 = assign6580_e7759_d_n6;
        locals.var_t1_dn7 = assign6580_e7759_d_n7;
        locals.var_t1_dn8 = assign6580_e7759_d_n8;
        locals.var_t1_dn9 = assign6580_e7759_d_n9;
        locals.var_t1_dn10 = assign6580_e7759_d_n10;
        locals.var_t1_dn11 = assign6580_e7759_d_n11;
        locals.var_t1_dn12 = assign6580_e7759_d_n12;

        let (assign6590_e7770, assign6590_e7770_d_n3, assign6590_e7770_d_n4, assign6590_e7770_d_n5, assign6590_e7770_d_n6, assign6590_e7770_d_n7, assign6590_e7770_d_n8, assign6590_e7770_d_n9, assign6590_e7770_d_n10, assign6590_e7770_d_n11, assign6590_e7770_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 == 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_t0;
        let assign6590_e7766: f64 = (locals.var_t1 * __rspice_inv_cse_1);
        let assign6590_e7768: f64 = (assign6590_e7766 * __rspice_inv_cse_1);
        (assign6590_e7768, ((((((locals.var_t1_dn3 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6590_e7766 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn4 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6590_e7766 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn5 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6590_e7766 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn6 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6590_e7766 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn7 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6590_e7766 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn8 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6590_e7766 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn9 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6590_e7766 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn10 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6590_e7766 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn11 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6590_e7766 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn12 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6590_e7766 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign6590_e7770;
        locals.var_t2_dn3 = assign6590_e7770_d_n3;
        locals.var_t2_dn4 = assign6590_e7770_d_n4;
        locals.var_t2_dn5 = assign6590_e7770_d_n5;
        locals.var_t2_dn6 = assign6590_e7770_d_n6;
        locals.var_t2_dn7 = assign6590_e7770_d_n7;
        locals.var_t2_dn8 = assign6590_e7770_d_n8;
        locals.var_t2_dn9 = assign6590_e7770_d_n9;
        locals.var_t2_dn10 = assign6590_e7770_d_n10;
        locals.var_t2_dn11 = assign6590_e7770_d_n11;
        locals.var_t2_dn12 = assign6590_e7770_d_n12;

        let (assign6600_e7779, assign6600_e7779_d_n3, assign6600_e7779_d_n4, assign6600_e7779_d_n5, assign6600_e7779_d_n6, assign6600_e7779_d_n7, assign6600_e7779_d_n8, assign6600_e7779_d_n9, assign6600_e7779_d_n10, assign6600_e7779_d_n11, assign6600_e7779_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 == 0.0)) {
        let assign6600_e7777: f64 = (locals.var_t2 / p.p356);
        (assign6600_e7777, (locals.var_t2_dn3 / p.p356), (locals.var_t2_dn4 / p.p356), (locals.var_t2_dn5 / p.p356), (locals.var_t2_dn6 / p.p356), (locals.var_t2_dn7 / p.p356), (locals.var_t2_dn8 / p.p356), (locals.var_t2_dn9 / p.p356), (locals.var_t2_dn10 / p.p356), (locals.var_t2_dn11 / p.p356), (locals.var_t2_dn12 / p.p356),)
    } else {
        (locals.var_pparam_b4soist2, locals.var_pparam_b4soist2_dn3, locals.var_pparam_b4soist2_dn4, locals.var_pparam_b4soist2_dn5, locals.var_pparam_b4soist2_dn6, locals.var_pparam_b4soist2_dn7, locals.var_pparam_b4soist2_dn8, locals.var_pparam_b4soist2_dn9, locals.var_pparam_b4soist2_dn10, locals.var_pparam_b4soist2_dn11, locals.var_pparam_b4soist2_dn12,)
    }
};
        locals.var_pparam_b4soist2 = assign6600_e7779;
        locals.var_pparam_b4soist2_dn3 = assign6600_e7779_d_n3;
        locals.var_pparam_b4soist2_dn4 = assign6600_e7779_d_n4;
        locals.var_pparam_b4soist2_dn5 = assign6600_e7779_d_n5;
        locals.var_pparam_b4soist2_dn6 = assign6600_e7779_d_n6;
        locals.var_pparam_b4soist2_dn7 = assign6600_e7779_d_n7;
        locals.var_pparam_b4soist2_dn8 = assign6600_e7779_d_n8;
        locals.var_pparam_b4soist2_dn9 = assign6600_e7779_d_n9;
        locals.var_pparam_b4soist2_dn10 = assign6600_e7779_d_n10;
        locals.var_pparam_b4soist2_dn11 = assign6600_e7779_d_n11;
        locals.var_pparam_b4soist2_dn12 = assign6600_e7779_d_n12;

        let (assign6610_e7790, assign6610_e7790_d_n3, assign6610_e7790_d_n4, assign6610_e7790_d_n5, assign6610_e7790_d_n6, assign6610_e7790_d_n7, assign6610_e7790_d_n8, assign6610_e7790_d_n9, assign6610_e7790_d_n10, assign6610_e7790_d_n11, assign6610_e7790_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 == 0.0)) {
        let assign6610_e7787: f64 = (1.0 - p.p356);
        let assign6610_e7788: f64 = (locals.var_t2 / assign6610_e7787);
        (assign6610_e7788, (locals.var_t2_dn3 / assign6610_e7787), (locals.var_t2_dn4 / assign6610_e7787), (locals.var_t2_dn5 / assign6610_e7787), (locals.var_t2_dn6 / assign6610_e7787), (locals.var_t2_dn7 / assign6610_e7787), (locals.var_t2_dn8 / assign6610_e7787), (locals.var_t2_dn9 / assign6610_e7787), (locals.var_t2_dn10 / assign6610_e7787), (locals.var_t2_dn11 / assign6610_e7787), (locals.var_t2_dn12 / assign6610_e7787),)
    } else {
        (locals.var_pparam_b4soist3, locals.var_pparam_b4soist3_dn3, locals.var_pparam_b4soist3_dn4, locals.var_pparam_b4soist3_dn5, locals.var_pparam_b4soist3_dn6, locals.var_pparam_b4soist3_dn7, locals.var_pparam_b4soist3_dn8, locals.var_pparam_b4soist3_dn9, locals.var_pparam_b4soist3_dn10, locals.var_pparam_b4soist3_dn11, locals.var_pparam_b4soist3_dn12,)
    }
};
        locals.var_pparam_b4soist3 = assign6610_e7790;
        locals.var_pparam_b4soist3_dn3 = assign6610_e7790_d_n3;
        locals.var_pparam_b4soist3_dn4 = assign6610_e7790_d_n4;
        locals.var_pparam_b4soist3_dn5 = assign6610_e7790_d_n5;
        locals.var_pparam_b4soist3_dn6 = assign6610_e7790_d_n6;
        locals.var_pparam_b4soist3_dn7 = assign6610_e7790_d_n7;
        locals.var_pparam_b4soist3_dn8 = assign6610_e7790_d_n8;
        locals.var_pparam_b4soist3_dn9 = assign6610_e7790_d_n9;
        locals.var_pparam_b4soist3_dn10 = assign6610_e7790_d_n10;
        locals.var_pparam_b4soist3_dn11 = assign6610_e7790_d_n11;
        locals.var_pparam_b4soist3_dn12 = assign6610_e7790_d_n12;

        let (assign6620_e7809, assign6620_e7809_d_n3, assign6620_e7809_d_n4, assign6620_e7809_d_n5, assign6620_e7809_d_n6, assign6620_e7809_d_n7, assign6620_e7809_d_n8, assign6620_e7809_d_n9, assign6620_e7809_d_n10, assign6620_e7809_d_n11, assign6620_e7809_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 == 0.0)) {
        let assign6620_e7797: f64 = (locals.var_t0 * locals.var_t1);
        let assign6620_e7800: f64 = (1.0 + p.p356);
        let assign6620_e7801: f64 = (assign6620_e7797 * assign6620_e7800);
        let assign6620_e7803: f64 = (assign6620_e7801 / 3.0);
        let assign6620_e7806: f64 = (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth);
        let assign6620_e7807: f64 = (assign6620_e7803 - assign6620_e7806);
        (assign6620_e7807, (((((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3)) * assign6620_e7800) / 3.0) - (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth_dn3)), (((((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)) * assign6620_e7800) / 3.0) - (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth_dn4)), (((((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)) * assign6620_e7800) / 3.0) - (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth_dn5)), (((((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)) * assign6620_e7800) / 3.0) - (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth_dn6)), (((((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)) * assign6620_e7800) / 3.0) - (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth_dn7)), (((((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)) * assign6620_e7800) / 3.0) - (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth_dn8)), (((((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)) * assign6620_e7800) / 3.0) - (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth_dn9)), (((((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)) * assign6620_e7800) / 3.0) - (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth_dn10)), (((((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)) * assign6620_e7800) / 3.0) - (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth_dn11)), (((((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12)) * assign6620_e7800) / 3.0) - (locals.var_b4soicsbox * locals.var_pparam_b4soivsdth_dn12)),)
    } else {
        (locals.var_b4soist4, locals.var_b4soist4_dn3, locals.var_b4soist4_dn4, locals.var_b4soist4_dn5, locals.var_b4soist4_dn6, locals.var_b4soist4_dn7, locals.var_b4soist4_dn8, locals.var_b4soist4_dn9, locals.var_b4soist4_dn10, locals.var_b4soist4_dn11, locals.var_b4soist4_dn12,)
    }
};
        locals.var_b4soist4 = assign6620_e7809;
        locals.var_b4soist4_dn3 = assign6620_e7809_d_n3;
        locals.var_b4soist4_dn4 = assign6620_e7809_d_n4;
        locals.var_b4soist4_dn5 = assign6620_e7809_d_n5;
        locals.var_b4soist4_dn6 = assign6620_e7809_d_n6;
        locals.var_b4soist4_dn7 = assign6620_e7809_d_n7;
        locals.var_b4soist4_dn8 = assign6620_e7809_d_n8;
        locals.var_b4soist4_dn9 = assign6620_e7809_d_n9;
        locals.var_b4soist4_dn10 = assign6620_e7809_d_n10;
        locals.var_b4soist4_dn11 = assign6620_e7809_d_n11;
        locals.var_b4soist4_dn12 = assign6620_e7809_d_n12;

        let (assign6630_e7818, assign6630_e7818_d_n3, assign6630_e7818_d_n4, assign6630_e7818_d_n5, assign6630_e7818_d_n6, assign6630_e7818_d_n7, assign6630_e7818_d_n8, assign6630_e7818_d_n9, assign6630_e7818_d_n10, assign6630_e7818_d_n11, assign6630_e7818_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 == 0.0)) {
        let assign6630_e7816: f64 = (locals.var_b4soicdmin - locals.var_b4soicdbox);
        (assign6630_e7816, locals.var_b4soicdmin_dn3, locals.var_b4soicdmin_dn4, locals.var_b4soicdmin_dn5, locals.var_b4soicdmin_dn6, locals.var_b4soicdmin_dn7, locals.var_b4soicdmin_dn8, locals.var_b4soicdmin_dn9, locals.var_b4soicdmin_dn10, locals.var_b4soicdmin_dn11, locals.var_b4soicdmin_dn12,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign6630_e7818;
        locals.var_t1_dn3 = assign6630_e7818_d_n3;
        locals.var_t1_dn4 = assign6630_e7818_d_n4;
        locals.var_t1_dn5 = assign6630_e7818_d_n5;
        locals.var_t1_dn6 = assign6630_e7818_d_n6;
        locals.var_t1_dn7 = assign6630_e7818_d_n7;
        locals.var_t1_dn8 = assign6630_e7818_d_n8;
        locals.var_t1_dn9 = assign6630_e7818_d_n9;
        locals.var_t1_dn10 = assign6630_e7818_d_n10;
        locals.var_t1_dn11 = assign6630_e7818_d_n11;
        locals.var_t1_dn12 = assign6630_e7818_d_n12;

        let (assign6640_e7829, assign6640_e7829_d_n3, assign6640_e7829_d_n4, assign6640_e7829_d_n5, assign6640_e7829_d_n6, assign6640_e7829_d_n7, assign6640_e7829_d_n8, assign6640_e7829_d_n9, assign6640_e7829_d_n10, assign6640_e7829_d_n11, assign6640_e7829_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 == 0.0)) {
        let __rspice_inv_cse_2: f64 = 1.0 / locals.var_t0;
        let assign6640_e7825: f64 = (locals.var_t1 * __rspice_inv_cse_2);
        let assign6640_e7827: f64 = (assign6640_e7825 * __rspice_inv_cse_2);
        (assign6640_e7827, ((((((locals.var_t1_dn3 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6640_e7825 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn4 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6640_e7825 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn5 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6640_e7825 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn6 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6640_e7825 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn7 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6640_e7825 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn8 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6640_e7825 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn9 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6640_e7825 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn10 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6640_e7825 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn11 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6640_e7825 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), ((((((locals.var_t1_dn12 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)) * locals.var_t0) - (assign6640_e7825 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign6640_e7829;
        locals.var_t2_dn3 = assign6640_e7829_d_n3;
        locals.var_t2_dn4 = assign6640_e7829_d_n4;
        locals.var_t2_dn5 = assign6640_e7829_d_n5;
        locals.var_t2_dn6 = assign6640_e7829_d_n6;
        locals.var_t2_dn7 = assign6640_e7829_d_n7;
        locals.var_t2_dn8 = assign6640_e7829_d_n8;
        locals.var_t2_dn9 = assign6640_e7829_d_n9;
        locals.var_t2_dn10 = assign6640_e7829_d_n10;
        locals.var_t2_dn11 = assign6640_e7829_d_n11;
        locals.var_t2_dn12 = assign6640_e7829_d_n12;

        let (assign6650_e7838, assign6650_e7838_d_n3, assign6650_e7838_d_n4, assign6650_e7838_d_n5, assign6650_e7838_d_n6, assign6650_e7838_d_n7, assign6650_e7838_d_n8, assign6650_e7838_d_n9, assign6650_e7838_d_n10, assign6650_e7838_d_n11, assign6650_e7838_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 == 0.0)) {
        let assign6650_e7836: f64 = (locals.var_t2 / p.p356);
        (assign6650_e7836, (locals.var_t2_dn3 / p.p356), (locals.var_t2_dn4 / p.p356), (locals.var_t2_dn5 / p.p356), (locals.var_t2_dn6 / p.p356), (locals.var_t2_dn7 / p.p356), (locals.var_t2_dn8 / p.p356), (locals.var_t2_dn9 / p.p356), (locals.var_t2_dn10 / p.p356), (locals.var_t2_dn11 / p.p356), (locals.var_t2_dn12 / p.p356),)
    } else {
        (locals.var_pparam_b4soidt2, locals.var_pparam_b4soidt2_dn3, locals.var_pparam_b4soidt2_dn4, locals.var_pparam_b4soidt2_dn5, locals.var_pparam_b4soidt2_dn6, locals.var_pparam_b4soidt2_dn7, locals.var_pparam_b4soidt2_dn8, locals.var_pparam_b4soidt2_dn9, locals.var_pparam_b4soidt2_dn10, locals.var_pparam_b4soidt2_dn11, locals.var_pparam_b4soidt2_dn12,)
    }
};
        locals.var_pparam_b4soidt2 = assign6650_e7838;
        locals.var_pparam_b4soidt2_dn3 = assign6650_e7838_d_n3;
        locals.var_pparam_b4soidt2_dn4 = assign6650_e7838_d_n4;
        locals.var_pparam_b4soidt2_dn5 = assign6650_e7838_d_n5;
        locals.var_pparam_b4soidt2_dn6 = assign6650_e7838_d_n6;
        locals.var_pparam_b4soidt2_dn7 = assign6650_e7838_d_n7;
        locals.var_pparam_b4soidt2_dn8 = assign6650_e7838_d_n8;
        locals.var_pparam_b4soidt2_dn9 = assign6650_e7838_d_n9;
        locals.var_pparam_b4soidt2_dn10 = assign6650_e7838_d_n10;
        locals.var_pparam_b4soidt2_dn11 = assign6650_e7838_d_n11;
        locals.var_pparam_b4soidt2_dn12 = assign6650_e7838_d_n12;

        let (assign6660_e7849, assign6660_e7849_d_n3, assign6660_e7849_d_n4, assign6660_e7849_d_n5, assign6660_e7849_d_n6, assign6660_e7849_d_n7, assign6660_e7849_d_n8, assign6660_e7849_d_n9, assign6660_e7849_d_n10, assign6660_e7849_d_n11, assign6660_e7849_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 == 0.0)) {
        let assign6660_e7846: f64 = (1.0 - p.p356);
        let assign6660_e7847: f64 = (locals.var_t2 / assign6660_e7846);
        (assign6660_e7847, (locals.var_t2_dn3 / assign6660_e7846), (locals.var_t2_dn4 / assign6660_e7846), (locals.var_t2_dn5 / assign6660_e7846), (locals.var_t2_dn6 / assign6660_e7846), (locals.var_t2_dn7 / assign6660_e7846), (locals.var_t2_dn8 / assign6660_e7846), (locals.var_t2_dn9 / assign6660_e7846), (locals.var_t2_dn10 / assign6660_e7846), (locals.var_t2_dn11 / assign6660_e7846), (locals.var_t2_dn12 / assign6660_e7846),)
    } else {
        (locals.var_pparam_b4soidt3, locals.var_pparam_b4soidt3_dn3, locals.var_pparam_b4soidt3_dn4, locals.var_pparam_b4soidt3_dn5, locals.var_pparam_b4soidt3_dn6, locals.var_pparam_b4soidt3_dn7, locals.var_pparam_b4soidt3_dn8, locals.var_pparam_b4soidt3_dn9, locals.var_pparam_b4soidt3_dn10, locals.var_pparam_b4soidt3_dn11, locals.var_pparam_b4soidt3_dn12,)
    }
};
        locals.var_pparam_b4soidt3 = assign6660_e7849;
        locals.var_pparam_b4soidt3_dn3 = assign6660_e7849_d_n3;
        locals.var_pparam_b4soidt3_dn4 = assign6660_e7849_d_n4;
        locals.var_pparam_b4soidt3_dn5 = assign6660_e7849_d_n5;
        locals.var_pparam_b4soidt3_dn6 = assign6660_e7849_d_n6;
        locals.var_pparam_b4soidt3_dn7 = assign6660_e7849_d_n7;
        locals.var_pparam_b4soidt3_dn8 = assign6660_e7849_d_n8;
        locals.var_pparam_b4soidt3_dn9 = assign6660_e7849_d_n9;
        locals.var_pparam_b4soidt3_dn10 = assign6660_e7849_d_n10;
        locals.var_pparam_b4soidt3_dn11 = assign6660_e7849_d_n11;
        locals.var_pparam_b4soidt3_dn12 = assign6660_e7849_d_n12;

        let (assign6670_e7868, assign6670_e7868_d_n3, assign6670_e7868_d_n4, assign6670_e7868_d_n5, assign6670_e7868_d_n6, assign6670_e7868_d_n7, assign6670_e7868_d_n8, assign6670_e7868_d_n9, assign6670_e7868_d_n10, assign6670_e7868_d_n11, assign6670_e7868_d_n12,) = {
    if ((locals.var_guard583 != 0.0) && (locals.var_guard584 == 0.0)) {
        let assign6670_e7856: f64 = (locals.var_t0 * locals.var_t1);
        let assign6670_e7859: f64 = (1.0 + p.p356);
        let assign6670_e7860: f64 = (assign6670_e7856 * assign6670_e7859);
        let assign6670_e7862: f64 = (assign6670_e7860 / 3.0);
        let assign6670_e7865: f64 = (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth);
        let assign6670_e7866: f64 = (assign6670_e7862 - assign6670_e7865);
        (assign6670_e7866, (((((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3)) * assign6670_e7859) / 3.0) - (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth_dn3)), (((((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)) * assign6670_e7859) / 3.0) - (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth_dn4)), (((((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)) * assign6670_e7859) / 3.0) - (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth_dn5)), (((((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)) * assign6670_e7859) / 3.0) - (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth_dn6)), (((((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)) * assign6670_e7859) / 3.0) - (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth_dn7)), (((((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)) * assign6670_e7859) / 3.0) - (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth_dn8)), (((((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)) * assign6670_e7859) / 3.0) - (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth_dn9)), (((((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)) * assign6670_e7859) / 3.0) - (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth_dn10)), (((((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)) * assign6670_e7859) / 3.0) - (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth_dn11)), (((((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12)) * assign6670_e7859) / 3.0) - (locals.var_b4soicdbox * locals.var_pparam_b4soivsdth_dn12)),)
    } else {
        (locals.var_b4soidt4, locals.var_b4soidt4_dn3, locals.var_b4soidt4_dn4, locals.var_b4soidt4_dn5, locals.var_b4soidt4_dn6, locals.var_b4soidt4_dn7, locals.var_b4soidt4_dn8, locals.var_b4soidt4_dn9, locals.var_b4soidt4_dn10, locals.var_b4soidt4_dn11, locals.var_b4soidt4_dn12,)
    }
};
        locals.var_b4soidt4 = assign6670_e7868;
        locals.var_b4soidt4_dn3 = assign6670_e7868_d_n3;
        locals.var_b4soidt4_dn4 = assign6670_e7868_d_n4;
        locals.var_b4soidt4_dn5 = assign6670_e7868_d_n5;
        locals.var_b4soidt4_dn6 = assign6670_e7868_d_n6;
        locals.var_b4soidt4_dn7 = assign6670_e7868_d_n7;
        locals.var_b4soidt4_dn8 = assign6670_e7868_d_n8;
        locals.var_b4soidt4_dn9 = assign6670_e7868_d_n9;
        locals.var_b4soidt4_dn10 = assign6670_e7868_d_n10;
        locals.var_b4soidt4_dn11 = assign6670_e7868_d_n11;
        locals.var_b4soidt4_dn12 = assign6670_e7868_d_n12;

        let (assign6680_e7873,) = {
    if (locals.var_guard583 == 0.0) {
        (0.0,)
    } else {
        (locals.var_pparam_b4soisdt1,)
    }
};
        locals.var_pparam_b4soisdt1 = assign6680_e7873;

        let (assign6690_e7878, assign6690_e7878_d_n3, assign6690_e7878_d_n4, assign6690_e7878_d_n5, assign6690_e7878_d_n6, assign6690_e7878_d_n7, assign6690_e7878_d_n8, assign6690_e7878_d_n9, assign6690_e7878_d_n10, assign6690_e7878_d_n11, assign6690_e7878_d_n12,) = {
    if (locals.var_guard583 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soist2, locals.var_pparam_b4soist2_dn3, locals.var_pparam_b4soist2_dn4, locals.var_pparam_b4soist2_dn5, locals.var_pparam_b4soist2_dn6, locals.var_pparam_b4soist2_dn7, locals.var_pparam_b4soist2_dn8, locals.var_pparam_b4soist2_dn9, locals.var_pparam_b4soist2_dn10, locals.var_pparam_b4soist2_dn11, locals.var_pparam_b4soist2_dn12,)
    }
};
        locals.var_pparam_b4soist2 = assign6690_e7878;
        locals.var_pparam_b4soist2_dn3 = assign6690_e7878_d_n3;
        locals.var_pparam_b4soist2_dn4 = assign6690_e7878_d_n4;
        locals.var_pparam_b4soist2_dn5 = assign6690_e7878_d_n5;
        locals.var_pparam_b4soist2_dn6 = assign6690_e7878_d_n6;
        locals.var_pparam_b4soist2_dn7 = assign6690_e7878_d_n7;
        locals.var_pparam_b4soist2_dn8 = assign6690_e7878_d_n8;
        locals.var_pparam_b4soist2_dn9 = assign6690_e7878_d_n9;
        locals.var_pparam_b4soist2_dn10 = assign6690_e7878_d_n10;
        locals.var_pparam_b4soist2_dn11 = assign6690_e7878_d_n11;
        locals.var_pparam_b4soist2_dn12 = assign6690_e7878_d_n12;

        let (assign6700_e7883, assign6700_e7883_d_n3, assign6700_e7883_d_n4, assign6700_e7883_d_n5, assign6700_e7883_d_n6, assign6700_e7883_d_n7, assign6700_e7883_d_n8, assign6700_e7883_d_n9, assign6700_e7883_d_n10, assign6700_e7883_d_n11, assign6700_e7883_d_n12,) = {
    if (locals.var_guard583 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soist3, locals.var_pparam_b4soist3_dn3, locals.var_pparam_b4soist3_dn4, locals.var_pparam_b4soist3_dn5, locals.var_pparam_b4soist3_dn6, locals.var_pparam_b4soist3_dn7, locals.var_pparam_b4soist3_dn8, locals.var_pparam_b4soist3_dn9, locals.var_pparam_b4soist3_dn10, locals.var_pparam_b4soist3_dn11, locals.var_pparam_b4soist3_dn12,)
    }
};
        locals.var_pparam_b4soist3 = assign6700_e7883;
        locals.var_pparam_b4soist3_dn3 = assign6700_e7883_d_n3;
        locals.var_pparam_b4soist3_dn4 = assign6700_e7883_d_n4;
        locals.var_pparam_b4soist3_dn5 = assign6700_e7883_d_n5;
        locals.var_pparam_b4soist3_dn6 = assign6700_e7883_d_n6;
        locals.var_pparam_b4soist3_dn7 = assign6700_e7883_d_n7;
        locals.var_pparam_b4soist3_dn8 = assign6700_e7883_d_n8;
        locals.var_pparam_b4soist3_dn9 = assign6700_e7883_d_n9;
        locals.var_pparam_b4soist3_dn10 = assign6700_e7883_d_n10;
        locals.var_pparam_b4soist3_dn11 = assign6700_e7883_d_n11;
        locals.var_pparam_b4soist3_dn12 = assign6700_e7883_d_n12;

        let (assign6710_e7888, assign6710_e7888_d_n3, assign6710_e7888_d_n4, assign6710_e7888_d_n5, assign6710_e7888_d_n6, assign6710_e7888_d_n7, assign6710_e7888_d_n8, assign6710_e7888_d_n9, assign6710_e7888_d_n10, assign6710_e7888_d_n11, assign6710_e7888_d_n12,) = {
    if (locals.var_guard583 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soist4, locals.var_b4soist4_dn3, locals.var_b4soist4_dn4, locals.var_b4soist4_dn5, locals.var_b4soist4_dn6, locals.var_b4soist4_dn7, locals.var_b4soist4_dn8, locals.var_b4soist4_dn9, locals.var_b4soist4_dn10, locals.var_b4soist4_dn11, locals.var_b4soist4_dn12,)
    }
};
        locals.var_b4soist4 = assign6710_e7888;
        locals.var_b4soist4_dn3 = assign6710_e7888_d_n3;
        locals.var_b4soist4_dn4 = assign6710_e7888_d_n4;
        locals.var_b4soist4_dn5 = assign6710_e7888_d_n5;
        locals.var_b4soist4_dn6 = assign6710_e7888_d_n6;
        locals.var_b4soist4_dn7 = assign6710_e7888_d_n7;
        locals.var_b4soist4_dn8 = assign6710_e7888_d_n8;
        locals.var_b4soist4_dn9 = assign6710_e7888_d_n9;
        locals.var_b4soist4_dn10 = assign6710_e7888_d_n10;
        locals.var_b4soist4_dn11 = assign6710_e7888_d_n11;
        locals.var_b4soist4_dn12 = assign6710_e7888_d_n12;

        let (assign6720_e7893, assign6720_e7893_d_n3, assign6720_e7893_d_n4, assign6720_e7893_d_n5, assign6720_e7893_d_n6, assign6720_e7893_d_n7, assign6720_e7893_d_n8, assign6720_e7893_d_n9, assign6720_e7893_d_n10, assign6720_e7893_d_n11, assign6720_e7893_d_n12,) = {
    if (locals.var_guard583 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soidt2, locals.var_pparam_b4soidt2_dn3, locals.var_pparam_b4soidt2_dn4, locals.var_pparam_b4soidt2_dn5, locals.var_pparam_b4soidt2_dn6, locals.var_pparam_b4soidt2_dn7, locals.var_pparam_b4soidt2_dn8, locals.var_pparam_b4soidt2_dn9, locals.var_pparam_b4soidt2_dn10, locals.var_pparam_b4soidt2_dn11, locals.var_pparam_b4soidt2_dn12,)
    }
};
        locals.var_pparam_b4soidt2 = assign6720_e7893;
        locals.var_pparam_b4soidt2_dn3 = assign6720_e7893_d_n3;
        locals.var_pparam_b4soidt2_dn4 = assign6720_e7893_d_n4;
        locals.var_pparam_b4soidt2_dn5 = assign6720_e7893_d_n5;
        locals.var_pparam_b4soidt2_dn6 = assign6720_e7893_d_n6;
        locals.var_pparam_b4soidt2_dn7 = assign6720_e7893_d_n7;
        locals.var_pparam_b4soidt2_dn8 = assign6720_e7893_d_n8;
        locals.var_pparam_b4soidt2_dn9 = assign6720_e7893_d_n9;
        locals.var_pparam_b4soidt2_dn10 = assign6720_e7893_d_n10;
        locals.var_pparam_b4soidt2_dn11 = assign6720_e7893_d_n11;
        locals.var_pparam_b4soidt2_dn12 = assign6720_e7893_d_n12;

        let (assign6730_e7898, assign6730_e7898_d_n3, assign6730_e7898_d_n4, assign6730_e7898_d_n5, assign6730_e7898_d_n6, assign6730_e7898_d_n7, assign6730_e7898_d_n8, assign6730_e7898_d_n9, assign6730_e7898_d_n10, assign6730_e7898_d_n11, assign6730_e7898_d_n12,) = {
    if (locals.var_guard583 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soidt3, locals.var_pparam_b4soidt3_dn3, locals.var_pparam_b4soidt3_dn4, locals.var_pparam_b4soidt3_dn5, locals.var_pparam_b4soidt3_dn6, locals.var_pparam_b4soidt3_dn7, locals.var_pparam_b4soidt3_dn8, locals.var_pparam_b4soidt3_dn9, locals.var_pparam_b4soidt3_dn10, locals.var_pparam_b4soidt3_dn11, locals.var_pparam_b4soidt3_dn12,)
    }
};
        locals.var_pparam_b4soidt3 = assign6730_e7898;
        locals.var_pparam_b4soidt3_dn3 = assign6730_e7898_d_n3;
        locals.var_pparam_b4soidt3_dn4 = assign6730_e7898_d_n4;
        locals.var_pparam_b4soidt3_dn5 = assign6730_e7898_d_n5;
        locals.var_pparam_b4soidt3_dn6 = assign6730_e7898_d_n6;
        locals.var_pparam_b4soidt3_dn7 = assign6730_e7898_d_n7;
        locals.var_pparam_b4soidt3_dn8 = assign6730_e7898_d_n8;
        locals.var_pparam_b4soidt3_dn9 = assign6730_e7898_d_n9;
        locals.var_pparam_b4soidt3_dn10 = assign6730_e7898_d_n10;
        locals.var_pparam_b4soidt3_dn11 = assign6730_e7898_d_n11;
        locals.var_pparam_b4soidt3_dn12 = assign6730_e7898_d_n12;

        let (assign6740_e7903, assign6740_e7903_d_n3, assign6740_e7903_d_n4, assign6740_e7903_d_n5, assign6740_e7903_d_n6, assign6740_e7903_d_n7, assign6740_e7903_d_n8, assign6740_e7903_d_n9, assign6740_e7903_d_n10, assign6740_e7903_d_n11, assign6740_e7903_d_n12,) = {
    if (locals.var_guard583 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soidt4, locals.var_b4soidt4_dn3, locals.var_b4soidt4_dn4, locals.var_b4soidt4_dn5, locals.var_b4soidt4_dn6, locals.var_b4soidt4_dn7, locals.var_b4soidt4_dn8, locals.var_b4soidt4_dn9, locals.var_b4soidt4_dn10, locals.var_b4soidt4_dn11, locals.var_b4soidt4_dn12,)
    }
};
        locals.var_b4soidt4 = assign6740_e7903;
        locals.var_b4soidt4_dn3 = assign6740_e7903_d_n3;
        locals.var_b4soidt4_dn4 = assign6740_e7903_d_n4;
        locals.var_b4soidt4_dn5 = assign6740_e7903_d_n5;
        locals.var_b4soidt4_dn6 = assign6740_e7903_d_n6;
        locals.var_b4soidt4_dn7 = assign6740_e7903_d_n7;
        locals.var_b4soidt4_dn8 = assign6740_e7903_d_n8;
        locals.var_b4soidt4_dn9 = assign6740_e7903_d_n9;
        locals.var_b4soidt4_dn10 = assign6740_e7903_d_n10;
        locals.var_b4soidt4_dn11 = assign6740_e7903_d_n11;
        locals.var_b4soidt4_dn12 = assign6740_e7903_d_n12;

        let assign6750_e7910: f64 = if ((locals.var_b4soicfrcoeff < 1.0) || (locals.var_b4soicfrcoeff > 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard585 = assign6750_e7910;

        let (assign6760_e7914,) = {
    if (locals.var_guard585 != 0.0) {
        (1.0,)
    } else {
        (locals.var_b4soicfrcoeff,)
    }
};
        locals.var_b4soicfrcoeff = assign6760_e7914;

    }

    pub(super) fn stamp_transient_block_19(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign6770_e7920: f64 = (p.p155 / p.p154);
        let assign6770_e7921: f64 = (1.0 + assign6770_e7920);
        let assign6770_e7922: f64 = (locals.var_b4soicfrcoeff * assign6770_e7921);
        let (assign6770_e7935,) = {
    if (assign6770_e7922 > 1e-38) {
        let assign6770_e7929: f64 = (p.p155 / p.p154);
        let assign6770_e7930: f64 = (1.0 + assign6770_e7929);
        let assign6770_e7931: f64 = (locals.var_b4soicfrcoeff * assign6770_e7930);
        let assign6770_e7932: f64 = (assign6770_e7931).ln();
        (assign6770_e7932,)
    } else {
        let assign6770_e7934: f64 = (-87.49823353377374);
        (assign6770_e7934,)
    }
};
        let assign6770_e7936: f64 = (p.p357 * assign6770_e7935);
        locals.var_t0 = assign6770_e7936;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;

        let assign6780_e7939: f64 = (p.p10 - p.p2);
        locals.var_t1 = assign6780_e7939;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;

        let assign6790_e7942: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard586 = assign6790_e7942;

        let (assign6800_e7948, assign6800_e7948_d_n3, assign6800_e7948_d_n4, assign6800_e7948_d_n5, assign6800_e7948_d_n6, assign6800_e7948_d_n7, assign6800_e7948_d_n8, assign6800_e7948_d_n9, assign6800_e7948_d_n10, assign6800_e7948_d_n11, assign6800_e7948_d_n12,) = {
    if (locals.var_guard586 != 0.0) {
        let assign6800_e7946: f64 = (locals.var_t0 * locals.var_t1);
        (assign6800_e7946, ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3)), ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)), ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)), ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)), ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)), ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)), ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)), ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)), ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)), ((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12)),)
    } else {
        (locals.var_b4soicsesw, locals.var_b4soicsesw_dn3, locals.var_b4soicsesw_dn4, locals.var_b4soicsesw_dn5, locals.var_b4soicsesw_dn6, locals.var_b4soicsesw_dn7, locals.var_b4soicsesw_dn8, locals.var_b4soicsesw_dn9, locals.var_b4soicsesw_dn10, locals.var_b4soicsesw_dn11, locals.var_b4soicsesw_dn12,)
    }
};
        locals.var_b4soicsesw = assign6800_e7948;
        locals.var_b4soicsesw_dn3 = assign6800_e7948_d_n3;
        locals.var_b4soicsesw_dn4 = assign6800_e7948_d_n4;
        locals.var_b4soicsesw_dn5 = assign6800_e7948_d_n5;
        locals.var_b4soicsesw_dn6 = assign6800_e7948_d_n6;
        locals.var_b4soicsesw_dn7 = assign6800_e7948_d_n7;
        locals.var_b4soicsesw_dn8 = assign6800_e7948_d_n8;
        locals.var_b4soicsesw_dn9 = assign6800_e7948_d_n9;
        locals.var_b4soicsesw_dn10 = assign6800_e7948_d_n10;
        locals.var_b4soicsesw_dn11 = assign6800_e7948_d_n11;
        locals.var_b4soicsesw_dn12 = assign6800_e7948_d_n12;

        let (assign6810_e7953, assign6810_e7953_d_n3, assign6810_e7953_d_n4, assign6810_e7953_d_n5, assign6810_e7953_d_n6, assign6810_e7953_d_n7, assign6810_e7953_d_n8, assign6810_e7953_d_n9, assign6810_e7953_d_n10, assign6810_e7953_d_n11, assign6810_e7953_d_n12,) = {
    if (locals.var_guard586 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soicsesw, locals.var_b4soicsesw_dn3, locals.var_b4soicsesw_dn4, locals.var_b4soicsesw_dn5, locals.var_b4soicsesw_dn6, locals.var_b4soicsesw_dn7, locals.var_b4soicsesw_dn8, locals.var_b4soicsesw_dn9, locals.var_b4soicsesw_dn10, locals.var_b4soicsesw_dn11, locals.var_b4soicsesw_dn12,)
    }
};
        locals.var_b4soicsesw = assign6810_e7953;
        locals.var_b4soicsesw_dn3 = assign6810_e7953_d_n3;
        locals.var_b4soicsesw_dn4 = assign6810_e7953_d_n4;
        locals.var_b4soicsesw_dn5 = assign6810_e7953_d_n5;
        locals.var_b4soicsesw_dn6 = assign6810_e7953_d_n6;
        locals.var_b4soicsesw_dn7 = assign6810_e7953_d_n7;
        locals.var_b4soicsesw_dn8 = assign6810_e7953_d_n8;
        locals.var_b4soicsesw_dn9 = assign6810_e7953_d_n9;
        locals.var_b4soicsesw_dn10 = assign6810_e7953_d_n10;
        locals.var_b4soicsesw_dn11 = assign6810_e7953_d_n11;
        locals.var_b4soicsesw_dn12 = assign6810_e7953_d_n12;

        let assign6820_e7956: f64 = (p.p9 - p.p2);
        locals.var_t1 = assign6820_e7956;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;

        let assign6830_e7959: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard587 = assign6830_e7959;

        let (assign6840_e7965, assign6840_e7965_d_n3, assign6840_e7965_d_n4, assign6840_e7965_d_n5, assign6840_e7965_d_n6, assign6840_e7965_d_n7, assign6840_e7965_d_n8, assign6840_e7965_d_n9, assign6840_e7965_d_n10, assign6840_e7965_d_n11, assign6840_e7965_d_n12,) = {
    if (locals.var_guard587 != 0.0) {
        let assign6840_e7963: f64 = (locals.var_t0 * locals.var_t1);
        (assign6840_e7963, ((locals.var_t0_dn3 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn3)), ((locals.var_t0_dn4 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn4)), ((locals.var_t0_dn5 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn5)), ((locals.var_t0_dn6 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn6)), ((locals.var_t0_dn7 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn7)), ((locals.var_t0_dn8 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn8)), ((locals.var_t0_dn9 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn9)), ((locals.var_t0_dn10 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn10)), ((locals.var_t0_dn11 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn11)), ((locals.var_t0_dn12 * locals.var_t1) + (locals.var_t0 * locals.var_t1_dn12)),)
    } else {
        (locals.var_b4soicdesw, locals.var_b4soicdesw_dn3, locals.var_b4soicdesw_dn4, locals.var_b4soicdesw_dn5, locals.var_b4soicdesw_dn6, locals.var_b4soicdesw_dn7, locals.var_b4soicdesw_dn8, locals.var_b4soicdesw_dn9, locals.var_b4soicdesw_dn10, locals.var_b4soicdesw_dn11, locals.var_b4soicdesw_dn12,)
    }
};
        locals.var_b4soicdesw = assign6840_e7965;
        locals.var_b4soicdesw_dn3 = assign6840_e7965_d_n3;
        locals.var_b4soicdesw_dn4 = assign6840_e7965_d_n4;
        locals.var_b4soicdesw_dn5 = assign6840_e7965_d_n5;
        locals.var_b4soicdesw_dn6 = assign6840_e7965_d_n6;
        locals.var_b4soicdesw_dn7 = assign6840_e7965_d_n7;
        locals.var_b4soicdesw_dn8 = assign6840_e7965_d_n8;
        locals.var_b4soicdesw_dn9 = assign6840_e7965_d_n9;
        locals.var_b4soicdesw_dn10 = assign6840_e7965_d_n10;
        locals.var_b4soicdesw_dn11 = assign6840_e7965_d_n11;
        locals.var_b4soicdesw_dn12 = assign6840_e7965_d_n12;

        let (assign6850_e7970, assign6850_e7970_d_n3, assign6850_e7970_d_n4, assign6850_e7970_d_n5, assign6850_e7970_d_n6, assign6850_e7970_d_n7, assign6850_e7970_d_n8, assign6850_e7970_d_n9, assign6850_e7970_d_n10, assign6850_e7970_d_n11, assign6850_e7970_d_n12,) = {
    if (locals.var_guard587 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soicdesw, locals.var_b4soicdesw_dn3, locals.var_b4soicdesw_dn4, locals.var_b4soicdesw_dn5, locals.var_b4soicdesw_dn6, locals.var_b4soicdesw_dn7, locals.var_b4soicdesw_dn8, locals.var_b4soicdesw_dn9, locals.var_b4soicdesw_dn10, locals.var_b4soicdesw_dn11, locals.var_b4soicdesw_dn12,)
    }
};
        locals.var_b4soicdesw = assign6850_e7970;
        locals.var_b4soicdesw_dn3 = assign6850_e7970_d_n3;
        locals.var_b4soicdesw_dn4 = assign6850_e7970_d_n4;
        locals.var_b4soicdesw_dn5 = assign6850_e7970_d_n5;
        locals.var_b4soicdesw_dn6 = assign6850_e7970_d_n6;
        locals.var_b4soicdesw_dn7 = assign6850_e7970_d_n7;
        locals.var_b4soicdesw_dn8 = assign6850_e7970_d_n8;
        locals.var_b4soicdesw_dn9 = assign6850_e7970_d_n9;
        locals.var_b4soicdesw_dn10 = assign6850_e7970_d_n10;
        locals.var_b4soicdesw_dn11 = assign6850_e7970_d_n11;
        locals.var_b4soicdesw_dn12 = assign6850_e7970_d_n12;

        let assign6860_e7973: f64 = (p.p131 * p.p11);
        locals.var_b4soidrainresistance = assign6860_e7973;

        let assign6870_e7980: f64 = if ((p.p429 == 1.0) && (locals.var_b4soidrainresistance < p.p431)) { 1.0 } else { 0.0 };
        locals.var_guard588 = assign6870_e7980;

        let (assign6880_e7984,) = {
    if (locals.var_guard588 != 0.0) {
        (p.p431,)
    } else {
        (locals.var_b4soidrainresistance,)
    }
};
        locals.var_b4soidrainresistance = assign6880_e7984;

        let assign6890_e7987: f64 = (p.p131 * p.p12);
        locals.var_b4soisourceresistance = assign6890_e7987;

        let assign6900_e7994: f64 = if ((p.p429 == 1.0) && (locals.var_b4soisourceresistance < p.p431)) { 1.0 } else { 0.0 };
        locals.var_guard589 = assign6900_e7994;

        let (assign6910_e7998,) = {
    if (locals.var_guard589 != 0.0) {
        (p.p431,)
    } else {
        (locals.var_b4soisourceresistance,)
    }
};
        locals.var_b4soisourceresistance = assign6910_e7998;

        let assign6920_e8001: f64 = if locals.var_b4soiln < 1e-15 { 1.0 } else { 0.0 };
        locals.var_guard590 = assign6920_e8001;

        let (assign6930_e8005,) = {
    if (locals.var_guard590 != 0.0) {
        (1e-15,)
    } else {
        (locals.var_b4soiln,)
    }
};
        locals.var_b4soiln = assign6930_e8005;

        let assign6940_e8007: f64 = (-0.5);
        let assign6940_e8009: f64 = (assign6940_e8007 * locals.var_pparam_b4soileff);
        let assign6940_e8011: f64 = (assign6940_e8009 * locals.var_pparam_b4soileff);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_b4soiln;
        let assign6940_e8013: f64 = (assign6940_e8011 * __rspice_inv_cse_0);
        let assign6940_e8015: f64 = (assign6940_e8013 * __rspice_inv_cse_0);
        locals.var_t0 = assign6940_e8015;
        locals.var_t0_dn3 = (((((assign6940_e8007 * locals.var_pparam_b4soileff_dn3) * locals.var_pparam_b4soileff) + (assign6940_e8009 * locals.var_pparam_b4soileff_dn3)) / locals.var_b4soiln) / locals.var_b4soiln);
        locals.var_t0_dn4 = (((((assign6940_e8007 * locals.var_pparam_b4soileff_dn4) * locals.var_pparam_b4soileff) + (assign6940_e8009 * locals.var_pparam_b4soileff_dn4)) / locals.var_b4soiln) / locals.var_b4soiln);
        locals.var_t0_dn5 = (((((assign6940_e8007 * locals.var_pparam_b4soileff_dn5) * locals.var_pparam_b4soileff) + (assign6940_e8009 * locals.var_pparam_b4soileff_dn5)) / locals.var_b4soiln) / locals.var_b4soiln);
        locals.var_t0_dn6 = (((((assign6940_e8007 * locals.var_pparam_b4soileff_dn6) * locals.var_pparam_b4soileff) + (assign6940_e8009 * locals.var_pparam_b4soileff_dn6)) / locals.var_b4soiln) / locals.var_b4soiln);
        locals.var_t0_dn7 = (((((assign6940_e8007 * locals.var_pparam_b4soileff_dn7) * locals.var_pparam_b4soileff) + (assign6940_e8009 * locals.var_pparam_b4soileff_dn7)) / locals.var_b4soiln) / locals.var_b4soiln);
        locals.var_t0_dn8 = (((((assign6940_e8007 * locals.var_pparam_b4soileff_dn8) * locals.var_pparam_b4soileff) + (assign6940_e8009 * locals.var_pparam_b4soileff_dn8)) / locals.var_b4soiln) / locals.var_b4soiln);
        locals.var_t0_dn9 = (((((assign6940_e8007 * locals.var_pparam_b4soileff_dn9) * locals.var_pparam_b4soileff) + (assign6940_e8009 * locals.var_pparam_b4soileff_dn9)) / locals.var_b4soiln) / locals.var_b4soiln);
        locals.var_t0_dn10 = (((((assign6940_e8007 * locals.var_pparam_b4soileff_dn10) * locals.var_pparam_b4soileff) + (assign6940_e8009 * locals.var_pparam_b4soileff_dn10)) / locals.var_b4soiln) / locals.var_b4soiln);
        locals.var_t0_dn11 = (((((assign6940_e8007 * locals.var_pparam_b4soileff_dn11) * locals.var_pparam_b4soileff) + (assign6940_e8009 * locals.var_pparam_b4soileff_dn11)) / locals.var_b4soiln) / locals.var_b4soiln);
        locals.var_t0_dn12 = (((((assign6940_e8007 * locals.var_pparam_b4soileff_dn12) * locals.var_pparam_b4soileff) + (assign6940_e8009 * locals.var_pparam_b4soileff_dn12)) / locals.var_b4soiln) / locals.var_b4soiln);

        let assign6950_e8018: f64 = if locals.var_t0 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard591 = assign6950_e8018;

        let (assign6960_e8028, assign6960_e8028_d_n3, assign6960_e8028_d_n4, assign6960_e8028_d_n5, assign6960_e8028_d_n6, assign6960_e8028_d_n7, assign6960_e8028_d_n8, assign6960_e8028_d_n9, assign6960_e8028_d_n10, assign6960_e8028_d_n11, assign6960_e8028_d_n12,) = {
    if (locals.var_guard591 != 0.0) {
        let assign6960_e8023: f64 = (1.0 + locals.var_t0);
        let assign6960_e8025: f64 = (assign6960_e8023 - 100.0);
        let assign6960_e8026: f64 = (2.688117142e43 * assign6960_e8025);
        (assign6960_e8026, (2.688117142e43 * locals.var_t0_dn3), (2.688117142e43 * locals.var_t0_dn4), (2.688117142e43 * locals.var_t0_dn5), (2.688117142e43 * locals.var_t0_dn6), (2.688117142e43 * locals.var_t0_dn7), (2.688117142e43 * locals.var_t0_dn8), (2.688117142e43 * locals.var_t0_dn9), (2.688117142e43 * locals.var_t0_dn10), (2.688117142e43 * locals.var_t0_dn11), (2.688117142e43 * locals.var_t0_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign6960_e8028;
        locals.var_t1_dn3 = assign6960_e8028_d_n3;
        locals.var_t1_dn4 = assign6960_e8028_d_n4;
        locals.var_t1_dn5 = assign6960_e8028_d_n5;
        locals.var_t1_dn6 = assign6960_e8028_d_n6;
        locals.var_t1_dn7 = assign6960_e8028_d_n7;
        locals.var_t1_dn8 = assign6960_e8028_d_n8;
        locals.var_t1_dn9 = assign6960_e8028_d_n9;
        locals.var_t1_dn10 = assign6960_e8028_d_n10;
        locals.var_t1_dn11 = assign6960_e8028_d_n11;
        locals.var_t1_dn12 = assign6960_e8028_d_n12;

        let assign6970_e8031: f64 = (-100.0);
        let assign6970_e8032: f64 = if locals.var_t0 < assign6970_e8031 { 1.0 } else { 0.0 };
        locals.var_guard592 = assign6970_e8032;

        let (assign6980_e8039, assign6980_e8039_d_n3, assign6980_e8039_d_n4, assign6980_e8039_d_n5, assign6980_e8039_d_n6, assign6980_e8039_d_n7, assign6980_e8039_d_n8, assign6980_e8039_d_n9, assign6980_e8039_d_n10, assign6980_e8039_d_n11, assign6980_e8039_d_n12,) = {
    if ((locals.var_guard591 == 0.0) && (locals.var_guard592 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign6980_e8039;
        locals.var_t1_dn3 = assign6980_e8039_d_n3;
        locals.var_t1_dn4 = assign6980_e8039_d_n4;
        locals.var_t1_dn5 = assign6980_e8039_d_n5;
        locals.var_t1_dn6 = assign6980_e8039_d_n6;
        locals.var_t1_dn7 = assign6980_e8039_d_n7;
        locals.var_t1_dn8 = assign6980_e8039_d_n8;
        locals.var_t1_dn9 = assign6980_e8039_d_n9;
        locals.var_t1_dn10 = assign6980_e8039_d_n10;
        locals.var_t1_dn11 = assign6980_e8039_d_n11;
        locals.var_t1_dn12 = assign6980_e8039_d_n12;

        let (assign6990_e8048, assign6990_e8048_d_n3, assign6990_e8048_d_n4, assign6990_e8048_d_n5, assign6990_e8048_d_n6, assign6990_e8048_d_n7, assign6990_e8048_d_n8, assign6990_e8048_d_n9, assign6990_e8048_d_n10, assign6990_e8048_d_n11, assign6990_e8048_d_n12,) = {
    if ((locals.var_guard591 == 0.0) && (locals.var_guard592 == 0.0)) {
        let assign6990_e8046: f64 = (locals.var_t0).exp();
        (assign6990_e8046, (assign6990_e8046 * locals.var_t0_dn3), (assign6990_e8046 * locals.var_t0_dn4), (assign6990_e8046 * locals.var_t0_dn5), (assign6990_e8046 * locals.var_t0_dn6), (assign6990_e8046 * locals.var_t0_dn7), (assign6990_e8046 * locals.var_t0_dn8), (assign6990_e8046 * locals.var_t0_dn9), (assign6990_e8046 * locals.var_t0_dn10), (assign6990_e8046 * locals.var_t0_dn11), (assign6990_e8046 * locals.var_t0_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign6990_e8048;
        locals.var_t1_dn3 = assign6990_e8048_d_n3;
        locals.var_t1_dn4 = assign6990_e8048_d_n4;
        locals.var_t1_dn5 = assign6990_e8048_d_n5;
        locals.var_t1_dn6 = assign6990_e8048_d_n6;
        locals.var_t1_dn7 = assign6990_e8048_d_n7;
        locals.var_t1_dn8 = assign6990_e8048_d_n8;
        locals.var_t1_dn9 = assign6990_e8048_d_n9;
        locals.var_t1_dn10 = assign6990_e8048_d_n10;
        locals.var_t1_dn11 = assign6990_e8048_d_n11;
        locals.var_t1_dn12 = assign6990_e8048_d_n12;

        locals.var_pparam_b4soiarfabjt = locals.var_t1;
        locals.var_pparam_b4soiarfabjt_dn3 = locals.var_t1_dn3;
        locals.var_pparam_b4soiarfabjt_dn4 = locals.var_t1_dn4;
        locals.var_pparam_b4soiarfabjt_dn5 = locals.var_t1_dn5;
        locals.var_pparam_b4soiarfabjt_dn6 = locals.var_t1_dn6;
        locals.var_pparam_b4soiarfabjt_dn7 = locals.var_t1_dn7;
        locals.var_pparam_b4soiarfabjt_dn8 = locals.var_t1_dn8;
        locals.var_pparam_b4soiarfabjt_dn9 = locals.var_t1_dn9;
        locals.var_pparam_b4soiarfabjt_dn10 = locals.var_t1_dn10;
        locals.var_pparam_b4soiarfabjt_dn11 = locals.var_t1_dn11;
        locals.var_pparam_b4soiarfabjt_dn12 = locals.var_t1_dn12;

        let assign7010_e8053: f64 = (1.0 / locals.var_pparam_b4soileff);
        let assign7010_e8056: f64 = (1.0 / locals.var_b4soiln);
        let assign7010_e8057: f64 = (assign7010_e8053 + assign7010_e8056);
        let assign7010_e8058: f64 = (locals.var_pparam_b4soilbjt0 * assign7010_e8057);
        locals.var_t0 = assign7010_e8058;
        locals.var_t0_dn3 = ((locals.var_pparam_b4soilbjt0_dn3 * assign7010_e8057) + (locals.var_pparam_b4soilbjt0 * (-(locals.var_pparam_b4soileff_dn3 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))));
        locals.var_t0_dn4 = ((locals.var_pparam_b4soilbjt0_dn4 * assign7010_e8057) + (locals.var_pparam_b4soilbjt0 * (-(locals.var_pparam_b4soileff_dn4 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))));
        locals.var_t0_dn5 = ((locals.var_pparam_b4soilbjt0_dn5 * assign7010_e8057) + (locals.var_pparam_b4soilbjt0 * (-(locals.var_pparam_b4soileff_dn5 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))));
        locals.var_t0_dn6 = ((locals.var_pparam_b4soilbjt0_dn6 * assign7010_e8057) + (locals.var_pparam_b4soilbjt0 * (-(locals.var_pparam_b4soileff_dn6 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))));
        locals.var_t0_dn7 = ((locals.var_pparam_b4soilbjt0_dn7 * assign7010_e8057) + (locals.var_pparam_b4soilbjt0 * (-(locals.var_pparam_b4soileff_dn7 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))));
        locals.var_t0_dn8 = ((locals.var_pparam_b4soilbjt0_dn8 * assign7010_e8057) + (locals.var_pparam_b4soilbjt0 * (-(locals.var_pparam_b4soileff_dn8 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))));
        locals.var_t0_dn9 = ((locals.var_pparam_b4soilbjt0_dn9 * assign7010_e8057) + (locals.var_pparam_b4soilbjt0 * (-(locals.var_pparam_b4soileff_dn9 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))));
        locals.var_t0_dn10 = ((locals.var_pparam_b4soilbjt0_dn10 * assign7010_e8057) + (locals.var_pparam_b4soilbjt0 * (-(locals.var_pparam_b4soileff_dn10 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))));
        locals.var_t0_dn11 = ((locals.var_pparam_b4soilbjt0_dn11 * assign7010_e8057) + (locals.var_pparam_b4soilbjt0 * (-(locals.var_pparam_b4soileff_dn11 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))));
        locals.var_t0_dn12 = ((locals.var_pparam_b4soilbjt0_dn12 * assign7010_e8057) + (locals.var_pparam_b4soilbjt0 * (-(locals.var_pparam_b4soileff_dn12 / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))));

        let assign7020_e8061: f64 = (locals.var_t0).powf(locals.var_pparam_b4soinbjt);
        locals.var_pparam_b4soilratio = assign7020_e8061;
        locals.var_pparam_b4soilratio_dn3 = if locals.var_pparam_b4soinbjt_dn3 == 0.0 && ((locals.var_pparam_b4soinbjt) as f64).is_finite() && ((locals.var_pparam_b4soinbjt) as f64).fract() == 0.0 { if locals.var_pparam_b4soinbjt == 0.0 { 0.0 } else { (locals.var_pparam_b4soinbjt * ((locals.var_t0).powf(locals.var_pparam_b4soinbjt - 1.0) * locals.var_t0_dn3)) } } else { (assign7020_e8061 * ((locals.var_pparam_b4soinbjt_dn3 * (locals.var_t0).ln()) + (locals.var_pparam_b4soinbjt * (locals.var_t0_dn3 / locals.var_t0)))) };
        locals.var_pparam_b4soilratio_dn4 = if locals.var_pparam_b4soinbjt_dn4 == 0.0 && ((locals.var_pparam_b4soinbjt) as f64).is_finite() && ((locals.var_pparam_b4soinbjt) as f64).fract() == 0.0 { if locals.var_pparam_b4soinbjt == 0.0 { 0.0 } else { (locals.var_pparam_b4soinbjt * ((locals.var_t0).powf(locals.var_pparam_b4soinbjt - 1.0) * locals.var_t0_dn4)) } } else { (assign7020_e8061 * ((locals.var_pparam_b4soinbjt_dn4 * (locals.var_t0).ln()) + (locals.var_pparam_b4soinbjt * (locals.var_t0_dn4 / locals.var_t0)))) };
        locals.var_pparam_b4soilratio_dn5 = if locals.var_pparam_b4soinbjt_dn5 == 0.0 && ((locals.var_pparam_b4soinbjt) as f64).is_finite() && ((locals.var_pparam_b4soinbjt) as f64).fract() == 0.0 { if locals.var_pparam_b4soinbjt == 0.0 { 0.0 } else { (locals.var_pparam_b4soinbjt * ((locals.var_t0).powf(locals.var_pparam_b4soinbjt - 1.0) * locals.var_t0_dn5)) } } else { (assign7020_e8061 * ((locals.var_pparam_b4soinbjt_dn5 * (locals.var_t0).ln()) + (locals.var_pparam_b4soinbjt * (locals.var_t0_dn5 / locals.var_t0)))) };
        locals.var_pparam_b4soilratio_dn6 = if locals.var_pparam_b4soinbjt_dn6 == 0.0 && ((locals.var_pparam_b4soinbjt) as f64).is_finite() && ((locals.var_pparam_b4soinbjt) as f64).fract() == 0.0 { if locals.var_pparam_b4soinbjt == 0.0 { 0.0 } else { (locals.var_pparam_b4soinbjt * ((locals.var_t0).powf(locals.var_pparam_b4soinbjt - 1.0) * locals.var_t0_dn6)) } } else { (assign7020_e8061 * ((locals.var_pparam_b4soinbjt_dn6 * (locals.var_t0).ln()) + (locals.var_pparam_b4soinbjt * (locals.var_t0_dn6 / locals.var_t0)))) };
        locals.var_pparam_b4soilratio_dn7 = if locals.var_pparam_b4soinbjt_dn7 == 0.0 && ((locals.var_pparam_b4soinbjt) as f64).is_finite() && ((locals.var_pparam_b4soinbjt) as f64).fract() == 0.0 { if locals.var_pparam_b4soinbjt == 0.0 { 0.0 } else { (locals.var_pparam_b4soinbjt * ((locals.var_t0).powf(locals.var_pparam_b4soinbjt - 1.0) * locals.var_t0_dn7)) } } else { (assign7020_e8061 * ((locals.var_pparam_b4soinbjt_dn7 * (locals.var_t0).ln()) + (locals.var_pparam_b4soinbjt * (locals.var_t0_dn7 / locals.var_t0)))) };
        locals.var_pparam_b4soilratio_dn8 = if locals.var_pparam_b4soinbjt_dn8 == 0.0 && ((locals.var_pparam_b4soinbjt) as f64).is_finite() && ((locals.var_pparam_b4soinbjt) as f64).fract() == 0.0 { if locals.var_pparam_b4soinbjt == 0.0 { 0.0 } else { (locals.var_pparam_b4soinbjt * ((locals.var_t0).powf(locals.var_pparam_b4soinbjt - 1.0) * locals.var_t0_dn8)) } } else { (assign7020_e8061 * ((locals.var_pparam_b4soinbjt_dn8 * (locals.var_t0).ln()) + (locals.var_pparam_b4soinbjt * (locals.var_t0_dn8 / locals.var_t0)))) };
        locals.var_pparam_b4soilratio_dn9 = if locals.var_pparam_b4soinbjt_dn9 == 0.0 && ((locals.var_pparam_b4soinbjt) as f64).is_finite() && ((locals.var_pparam_b4soinbjt) as f64).fract() == 0.0 { if locals.var_pparam_b4soinbjt == 0.0 { 0.0 } else { (locals.var_pparam_b4soinbjt * ((locals.var_t0).powf(locals.var_pparam_b4soinbjt - 1.0) * locals.var_t0_dn9)) } } else { (assign7020_e8061 * ((locals.var_pparam_b4soinbjt_dn9 * (locals.var_t0).ln()) + (locals.var_pparam_b4soinbjt * (locals.var_t0_dn9 / locals.var_t0)))) };
        locals.var_pparam_b4soilratio_dn10 = if locals.var_pparam_b4soinbjt_dn10 == 0.0 && ((locals.var_pparam_b4soinbjt) as f64).is_finite() && ((locals.var_pparam_b4soinbjt) as f64).fract() == 0.0 { if locals.var_pparam_b4soinbjt == 0.0 { 0.0 } else { (locals.var_pparam_b4soinbjt * ((locals.var_t0).powf(locals.var_pparam_b4soinbjt - 1.0) * locals.var_t0_dn10)) } } else { (assign7020_e8061 * ((locals.var_pparam_b4soinbjt_dn10 * (locals.var_t0).ln()) + (locals.var_pparam_b4soinbjt * (locals.var_t0_dn10 / locals.var_t0)))) };
        locals.var_pparam_b4soilratio_dn11 = if locals.var_pparam_b4soinbjt_dn11 == 0.0 && ((locals.var_pparam_b4soinbjt) as f64).is_finite() && ((locals.var_pparam_b4soinbjt) as f64).fract() == 0.0 { if locals.var_pparam_b4soinbjt == 0.0 { 0.0 } else { (locals.var_pparam_b4soinbjt * ((locals.var_t0).powf(locals.var_pparam_b4soinbjt - 1.0) * locals.var_t0_dn11)) } } else { (assign7020_e8061 * ((locals.var_pparam_b4soinbjt_dn11 * (locals.var_t0).ln()) + (locals.var_pparam_b4soinbjt * (locals.var_t0_dn11 / locals.var_t0)))) };
        locals.var_pparam_b4soilratio_dn12 = if locals.var_pparam_b4soinbjt_dn12 == 0.0 && ((locals.var_pparam_b4soinbjt) as f64).is_finite() && ((locals.var_pparam_b4soinbjt) as f64).fract() == 0.0 { if locals.var_pparam_b4soinbjt == 0.0 { 0.0 } else { (locals.var_pparam_b4soinbjt * ((locals.var_t0).powf(locals.var_pparam_b4soinbjt - 1.0) * locals.var_t0_dn12)) } } else { (assign7020_e8061 * ((locals.var_pparam_b4soinbjt_dn12 * (locals.var_t0).ln()) + (locals.var_pparam_b4soinbjt * (locals.var_t0_dn12 / locals.var_t0)))) };

        let assign7030_e8066: f64 = (locals.var_t0).powf(locals.var_pparam_b4soindif);
        let assign7030_e8067: f64 = (p.p343 * assign7030_e8066);
        let assign7030_e8068: f64 = (1.0 + assign7030_e8067);
        locals.var_pparam_b4soilratiodif = assign7030_e8068;
        locals.var_pparam_b4soilratiodif_dn3 = (p.p343 * if locals.var_pparam_b4soindif_dn3 == 0.0 && ((locals.var_pparam_b4soindif) as f64).is_finite() && ((locals.var_pparam_b4soindif) as f64).fract() == 0.0 { if locals.var_pparam_b4soindif == 0.0 { 0.0 } else { (locals.var_pparam_b4soindif * ((locals.var_t0).powf(locals.var_pparam_b4soindif - 1.0) * locals.var_t0_dn3)) } } else { (assign7030_e8066 * ((locals.var_pparam_b4soindif_dn3 * (locals.var_t0).ln()) + (locals.var_pparam_b4soindif * (locals.var_t0_dn3 / locals.var_t0)))) });
        locals.var_pparam_b4soilratiodif_dn4 = (p.p343 * if locals.var_pparam_b4soindif_dn4 == 0.0 && ((locals.var_pparam_b4soindif) as f64).is_finite() && ((locals.var_pparam_b4soindif) as f64).fract() == 0.0 { if locals.var_pparam_b4soindif == 0.0 { 0.0 } else { (locals.var_pparam_b4soindif * ((locals.var_t0).powf(locals.var_pparam_b4soindif - 1.0) * locals.var_t0_dn4)) } } else { (assign7030_e8066 * ((locals.var_pparam_b4soindif_dn4 * (locals.var_t0).ln()) + (locals.var_pparam_b4soindif * (locals.var_t0_dn4 / locals.var_t0)))) });
        locals.var_pparam_b4soilratiodif_dn5 = (p.p343 * if locals.var_pparam_b4soindif_dn5 == 0.0 && ((locals.var_pparam_b4soindif) as f64).is_finite() && ((locals.var_pparam_b4soindif) as f64).fract() == 0.0 { if locals.var_pparam_b4soindif == 0.0 { 0.0 } else { (locals.var_pparam_b4soindif * ((locals.var_t0).powf(locals.var_pparam_b4soindif - 1.0) * locals.var_t0_dn5)) } } else { (assign7030_e8066 * ((locals.var_pparam_b4soindif_dn5 * (locals.var_t0).ln()) + (locals.var_pparam_b4soindif * (locals.var_t0_dn5 / locals.var_t0)))) });
        locals.var_pparam_b4soilratiodif_dn6 = (p.p343 * if locals.var_pparam_b4soindif_dn6 == 0.0 && ((locals.var_pparam_b4soindif) as f64).is_finite() && ((locals.var_pparam_b4soindif) as f64).fract() == 0.0 { if locals.var_pparam_b4soindif == 0.0 { 0.0 } else { (locals.var_pparam_b4soindif * ((locals.var_t0).powf(locals.var_pparam_b4soindif - 1.0) * locals.var_t0_dn6)) } } else { (assign7030_e8066 * ((locals.var_pparam_b4soindif_dn6 * (locals.var_t0).ln()) + (locals.var_pparam_b4soindif * (locals.var_t0_dn6 / locals.var_t0)))) });
        locals.var_pparam_b4soilratiodif_dn7 = (p.p343 * if locals.var_pparam_b4soindif_dn7 == 0.0 && ((locals.var_pparam_b4soindif) as f64).is_finite() && ((locals.var_pparam_b4soindif) as f64).fract() == 0.0 { if locals.var_pparam_b4soindif == 0.0 { 0.0 } else { (locals.var_pparam_b4soindif * ((locals.var_t0).powf(locals.var_pparam_b4soindif - 1.0) * locals.var_t0_dn7)) } } else { (assign7030_e8066 * ((locals.var_pparam_b4soindif_dn7 * (locals.var_t0).ln()) + (locals.var_pparam_b4soindif * (locals.var_t0_dn7 / locals.var_t0)))) });
        locals.var_pparam_b4soilratiodif_dn8 = (p.p343 * if locals.var_pparam_b4soindif_dn8 == 0.0 && ((locals.var_pparam_b4soindif) as f64).is_finite() && ((locals.var_pparam_b4soindif) as f64).fract() == 0.0 { if locals.var_pparam_b4soindif == 0.0 { 0.0 } else { (locals.var_pparam_b4soindif * ((locals.var_t0).powf(locals.var_pparam_b4soindif - 1.0) * locals.var_t0_dn8)) } } else { (assign7030_e8066 * ((locals.var_pparam_b4soindif_dn8 * (locals.var_t0).ln()) + (locals.var_pparam_b4soindif * (locals.var_t0_dn8 / locals.var_t0)))) });
        locals.var_pparam_b4soilratiodif_dn9 = (p.p343 * if locals.var_pparam_b4soindif_dn9 == 0.0 && ((locals.var_pparam_b4soindif) as f64).is_finite() && ((locals.var_pparam_b4soindif) as f64).fract() == 0.0 { if locals.var_pparam_b4soindif == 0.0 { 0.0 } else { (locals.var_pparam_b4soindif * ((locals.var_t0).powf(locals.var_pparam_b4soindif - 1.0) * locals.var_t0_dn9)) } } else { (assign7030_e8066 * ((locals.var_pparam_b4soindif_dn9 * (locals.var_t0).ln()) + (locals.var_pparam_b4soindif * (locals.var_t0_dn9 / locals.var_t0)))) });
        locals.var_pparam_b4soilratiodif_dn10 = (p.p343 * if locals.var_pparam_b4soindif_dn10 == 0.0 && ((locals.var_pparam_b4soindif) as f64).is_finite() && ((locals.var_pparam_b4soindif) as f64).fract() == 0.0 { if locals.var_pparam_b4soindif == 0.0 { 0.0 } else { (locals.var_pparam_b4soindif * ((locals.var_t0).powf(locals.var_pparam_b4soindif - 1.0) * locals.var_t0_dn10)) } } else { (assign7030_e8066 * ((locals.var_pparam_b4soindif_dn10 * (locals.var_t0).ln()) + (locals.var_pparam_b4soindif * (locals.var_t0_dn10 / locals.var_t0)))) });
        locals.var_pparam_b4soilratiodif_dn11 = (p.p343 * if locals.var_pparam_b4soindif_dn11 == 0.0 && ((locals.var_pparam_b4soindif) as f64).is_finite() && ((locals.var_pparam_b4soindif) as f64).fract() == 0.0 { if locals.var_pparam_b4soindif == 0.0 { 0.0 } else { (locals.var_pparam_b4soindif * ((locals.var_t0).powf(locals.var_pparam_b4soindif - 1.0) * locals.var_t0_dn11)) } } else { (assign7030_e8066 * ((locals.var_pparam_b4soindif_dn11 * (locals.var_t0).ln()) + (locals.var_pparam_b4soindif * (locals.var_t0_dn11 / locals.var_t0)))) });
        locals.var_pparam_b4soilratiodif_dn12 = (p.p343 * if locals.var_pparam_b4soindif_dn12 == 0.0 && ((locals.var_pparam_b4soindif) as f64).is_finite() && ((locals.var_pparam_b4soindif) as f64).fract() == 0.0 { if locals.var_pparam_b4soindif == 0.0 { 0.0 } else { (locals.var_pparam_b4soindif * ((locals.var_t0).powf(locals.var_pparam_b4soindif - 1.0) * locals.var_t0_dn12)) } } else { (assign7030_e8066 * ((locals.var_pparam_b4soindif_dn12 * (locals.var_t0).ln()) + (locals.var_pparam_b4soindif * (locals.var_t0_dn12 / locals.var_t0)))) });

        let assign7040_e8072: f64 = (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff);
        let assign7040_e8073: f64 = (locals.var_pparam_b4soivabjt + assign7040_e8072);
        locals.var_pparam_b4soivearly = assign7040_e8073;
        locals.var_pparam_b4soivearly_dn3 = (locals.var_pparam_b4soivabjt_dn3 + ((locals.var_pparam_b4soiaely_dn3 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff_dn3)));
        locals.var_pparam_b4soivearly_dn4 = (locals.var_pparam_b4soivabjt_dn4 + ((locals.var_pparam_b4soiaely_dn4 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff_dn4)));
        locals.var_pparam_b4soivearly_dn5 = (locals.var_pparam_b4soivabjt_dn5 + ((locals.var_pparam_b4soiaely_dn5 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff_dn5)));
        locals.var_pparam_b4soivearly_dn6 = (locals.var_pparam_b4soivabjt_dn6 + ((locals.var_pparam_b4soiaely_dn6 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff_dn6)));
        locals.var_pparam_b4soivearly_dn7 = (locals.var_pparam_b4soivabjt_dn7 + ((locals.var_pparam_b4soiaely_dn7 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff_dn7)));
        locals.var_pparam_b4soivearly_dn8 = (locals.var_pparam_b4soivabjt_dn8 + ((locals.var_pparam_b4soiaely_dn8 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff_dn8)));
        locals.var_pparam_b4soivearly_dn9 = (locals.var_pparam_b4soivabjt_dn9 + ((locals.var_pparam_b4soiaely_dn9 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff_dn9)));
        locals.var_pparam_b4soivearly_dn10 = (locals.var_pparam_b4soivabjt_dn10 + ((locals.var_pparam_b4soiaely_dn10 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff_dn10)));
        locals.var_pparam_b4soivearly_dn11 = (locals.var_pparam_b4soivabjt_dn11 + ((locals.var_pparam_b4soiaely_dn11 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff_dn11)));
        locals.var_pparam_b4soivearly_dn12 = (locals.var_pparam_b4soivabjt_dn12 + ((locals.var_pparam_b4soiaely_dn12 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soiaely * locals.var_pparam_b4soileff_dn12)));

        let assign7050_e8076: f64 = if locals.var_pparam_b4soivearly < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard593 = assign7050_e8076;

        let (assign7060_e8080, assign7060_e8080_d_n3, assign7060_e8080_d_n4, assign7060_e8080_d_n5, assign7060_e8080_d_n6, assign7060_e8080_d_n7, assign7060_e8080_d_n8, assign7060_e8080_d_n9, assign7060_e8080_d_n10, assign7060_e8080_d_n11, assign7060_e8080_d_n12,) = {
    if (locals.var_guard593 != 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soivearly, locals.var_pparam_b4soivearly_dn3, locals.var_pparam_b4soivearly_dn4, locals.var_pparam_b4soivearly_dn5, locals.var_pparam_b4soivearly_dn6, locals.var_pparam_b4soivearly_dn7, locals.var_pparam_b4soivearly_dn8, locals.var_pparam_b4soivearly_dn9, locals.var_pparam_b4soivearly_dn10, locals.var_pparam_b4soivearly_dn11, locals.var_pparam_b4soivearly_dn12,)
    }
};
        locals.var_pparam_b4soivearly = assign7060_e8080;
        locals.var_pparam_b4soivearly_dn3 = assign7060_e8080_d_n3;
        locals.var_pparam_b4soivearly_dn4 = assign7060_e8080_d_n4;
        locals.var_pparam_b4soivearly_dn5 = assign7060_e8080_d_n5;
        locals.var_pparam_b4soivearly_dn6 = assign7060_e8080_d_n6;
        locals.var_pparam_b4soivearly_dn7 = assign7060_e8080_d_n7;
        locals.var_pparam_b4soivearly_dn8 = assign7060_e8080_d_n8;
        locals.var_pparam_b4soivearly_dn9 = assign7060_e8080_d_n9;
        locals.var_pparam_b4soivearly_dn10 = assign7060_e8080_d_n10;
        locals.var_pparam_b4soivearly_dn11 = assign7060_e8080_d_n11;
        locals.var_pparam_b4soivearly_dn12 = assign7060_e8080_d_n12;

        let assign7070_e8083: f64 = if p.p41 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard594 = assign7070_e8083;

        let (assign7080_e8089, assign7080_e8089_d_n3, assign7080_e8089_d_n4, assign7080_e8089_d_n5, assign7080_e8089_d_n6, assign7080_e8089_d_n7, assign7080_e8089_d_n8, assign7080_e8089_d_n9, assign7080_e8089_d_n10, assign7080_e8089_d_n11, assign7080_e8089_d_n12,) = {
    if (locals.var_guard594 != 0.0) {
        let assign7080_e8087: f64 = (p.p66 - p.p68);
        (assign7080_e8087, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soitoxp, locals.var_b4soitoxp_dn3, locals.var_b4soitoxp_dn4, locals.var_b4soitoxp_dn5, locals.var_b4soitoxp_dn6, locals.var_b4soitoxp_dn7, locals.var_b4soitoxp_dn8, locals.var_b4soitoxp_dn9, locals.var_b4soitoxp_dn10, locals.var_b4soitoxp_dn11, locals.var_b4soitoxp_dn12,)
    }
};
        locals.var_b4soitoxp = assign7080_e8089;
        locals.var_b4soitoxp_dn3 = assign7080_e8089_d_n3;
        locals.var_b4soitoxp_dn4 = assign7080_e8089_d_n4;
        locals.var_b4soitoxp_dn5 = assign7080_e8089_d_n5;
        locals.var_b4soitoxp_dn6 = assign7080_e8089_d_n6;
        locals.var_b4soitoxp_dn7 = assign7080_e8089_d_n7;
        locals.var_b4soitoxp_dn8 = assign7080_e8089_d_n8;
        locals.var_b4soitoxp_dn9 = assign7080_e8089_d_n9;
        locals.var_b4soitoxp_dn10 = assign7080_e8089_d_n10;
        locals.var_b4soitoxp_dn11 = assign7080_e8089_d_n11;
        locals.var_b4soitoxp_dn12 = assign7080_e8089_d_n12;

        let (assign7090_e8096,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7090_e8094: f64 = (8.617087e-5 * p.p57);
        (assign7090_e8094,)
    } else {
        (locals.var_vtm0eot,)
    }
};
        locals.var_vtm0eot = assign7090_e8096;

        let (assign7100_e8101,) = {
    if (locals.var_guard594 == 0.0) {
        (locals.var_vtm0eot,)
    } else {
        (locals.var_vtmeot,)
    }
};
        locals.var_vtmeot = assign7100_e8101;

        let (assign7110_e8123, assign7110_e8123_d_n3, assign7110_e8123_d_n4, assign7110_e8123_d_n5, assign7110_e8123_d_n6, assign7110_e8123_d_n7, assign7110_e8123_d_n8, assign7110_e8123_d_n9, assign7110_e8123_d_n10, assign7110_e8123_d_n11, assign7110_e8123_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7110_e8107: f64 = (1e20 * locals.var_pparam_b4soinpeak);
        let (assign7110_e8116, assign7110_e8116_d_n3, assign7110_e8116_d_n4, assign7110_e8116_d_n5, assign7110_e8116_d_n6, assign7110_e8116_d_n7, assign7110_e8116_d_n8, assign7110_e8116_d_n9, assign7110_e8116_d_n10, assign7110_e8116_d_n11, assign7110_e8116_d_n12,) = {
            if (assign7110_e8107 > 1e-38) {
                let assign7110_e8112: f64 = (1e20 * locals.var_pparam_b4soinpeak);
                let assign7110_e8113: f64 = (assign7110_e8112).ln();
                (assign7110_e8113, ((1e20 * locals.var_pparam_b4soinpeak_dn3) / assign7110_e8112), ((1e20 * locals.var_pparam_b4soinpeak_dn4) / assign7110_e8112), ((1e20 * locals.var_pparam_b4soinpeak_dn5) / assign7110_e8112), ((1e20 * locals.var_pparam_b4soinpeak_dn6) / assign7110_e8112), ((1e20 * locals.var_pparam_b4soinpeak_dn7) / assign7110_e8112), ((1e20 * locals.var_pparam_b4soinpeak_dn8) / assign7110_e8112), ((1e20 * locals.var_pparam_b4soinpeak_dn9) / assign7110_e8112), ((1e20 * locals.var_pparam_b4soinpeak_dn10) / assign7110_e8112), ((1e20 * locals.var_pparam_b4soinpeak_dn11) / assign7110_e8112), ((1e20 * locals.var_pparam_b4soinpeak_dn12) / assign7110_e8112),)
            } else {
                let assign7110_e8115: f64 = (-87.49823353377374);
                (assign7110_e8115, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign7110_e8119: f64 = (2.0 * locals.var_lln_ni);
        let assign7110_e8120: f64 = (assign7110_e8116 - assign7110_e8119);
        let assign7110_e8121: f64 = (locals.var_vtm0eot * assign7110_e8120);
        (assign7110_e8121, (locals.var_vtm0eot * assign7110_e8116_d_n3), (locals.var_vtm0eot * (assign7110_e8116_d_n4 - (2.0 * locals.var_lln_ni_dn4))), (locals.var_vtm0eot * (assign7110_e8116_d_n5 - (2.0 * locals.var_lln_ni_dn5))), (locals.var_vtm0eot * (assign7110_e8116_d_n6 - (2.0 * locals.var_lln_ni_dn6))), (locals.var_vtm0eot * assign7110_e8116_d_n7), (locals.var_vtm0eot * assign7110_e8116_d_n8), (locals.var_vtm0eot * assign7110_e8116_d_n9), (locals.var_vtm0eot * assign7110_e8116_d_n10), (locals.var_vtm0eot * assign7110_e8116_d_n11), (locals.var_vtm0eot * assign7110_e8116_d_n12),)
    } else {
        (locals.var_vbieot, locals.var_vbieot_dn3, locals.var_vbieot_dn4, locals.var_vbieot_dn5, locals.var_vbieot_dn6, locals.var_vbieot_dn7, locals.var_vbieot_dn8, locals.var_vbieot_dn9, locals.var_vbieot_dn10, locals.var_vbieot_dn11, locals.var_vbieot_dn12,)
    }
};
        locals.var_vbieot = assign7110_e8123;
        locals.var_vbieot_dn3 = assign7110_e8123_d_n3;
        locals.var_vbieot_dn4 = assign7110_e8123_d_n4;
        locals.var_vbieot_dn5 = assign7110_e8123_d_n5;
        locals.var_vbieot_dn6 = assign7110_e8123_d_n6;
        locals.var_vbieot_dn7 = assign7110_e8123_d_n7;
        locals.var_vbieot_dn8 = assign7110_e8123_d_n8;
        locals.var_vbieot_dn9 = assign7110_e8123_d_n9;
        locals.var_vbieot_dn10 = assign7110_e8123_d_n10;
        locals.var_vbieot_dn11 = assign7110_e8123_d_n11;
        locals.var_vbieot_dn12 = assign7110_e8123_d_n12;

        let (assign7120_e8141, assign7120_e8141_d_n3, assign7120_e8141_d_n4, assign7120_e8141_d_n5, assign7120_e8141_d_n6, assign7120_e8141_d_n7, assign7120_e8141_d_n8, assign7120_e8141_d_n9, assign7120_e8141_d_n10, assign7120_e8141_d_n11, assign7120_e8141_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7120_e8128: f64 = (2.0 * locals.var_vtm0eot);
        let (assign7120_e8136, assign7120_e8136_d_n3, assign7120_e8136_d_n4, assign7120_e8136_d_n5, assign7120_e8136_d_n6, assign7120_e8136_d_n7, assign7120_e8136_d_n8, assign7120_e8136_d_n9, assign7120_e8136_d_n10, assign7120_e8136_d_n11, assign7120_e8136_d_n12,) = {
            if (locals.var_pparam_b4soinpeak > 1e-38) {
                let assign7120_e8133: f64 = (locals.var_pparam_b4soinpeak).ln();
                (assign7120_e8133, (locals.var_pparam_b4soinpeak_dn3 / locals.var_pparam_b4soinpeak), (locals.var_pparam_b4soinpeak_dn4 / locals.var_pparam_b4soinpeak), (locals.var_pparam_b4soinpeak_dn5 / locals.var_pparam_b4soinpeak), (locals.var_pparam_b4soinpeak_dn6 / locals.var_pparam_b4soinpeak), (locals.var_pparam_b4soinpeak_dn7 / locals.var_pparam_b4soinpeak), (locals.var_pparam_b4soinpeak_dn8 / locals.var_pparam_b4soinpeak), (locals.var_pparam_b4soinpeak_dn9 / locals.var_pparam_b4soinpeak), (locals.var_pparam_b4soinpeak_dn10 / locals.var_pparam_b4soinpeak), (locals.var_pparam_b4soinpeak_dn11 / locals.var_pparam_b4soinpeak), (locals.var_pparam_b4soinpeak_dn12 / locals.var_pparam_b4soinpeak),)
            } else {
                let assign7120_e8135: f64 = (-87.49823353377374);
                (assign7120_e8135, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign7120_e8138: f64 = (assign7120_e8136 - locals.var_lln_ni);
        let assign7120_e8139: f64 = (assign7120_e8128 * assign7120_e8138);
        (assign7120_e8139, (assign7120_e8128 * assign7120_e8136_d_n3), (assign7120_e8128 * (assign7120_e8136_d_n4 - locals.var_lln_ni_dn4)), (assign7120_e8128 * (assign7120_e8136_d_n5 - locals.var_lln_ni_dn5)), (assign7120_e8128 * (assign7120_e8136_d_n6 - locals.var_lln_ni_dn6)), (assign7120_e8128 * assign7120_e8136_d_n7), (assign7120_e8128 * assign7120_e8136_d_n8), (assign7120_e8128 * assign7120_e8136_d_n9), (assign7120_e8128 * assign7120_e8136_d_n10), (assign7120_e8128 * assign7120_e8136_d_n11), (assign7120_e8128 * assign7120_e8136_d_n12),)
    } else {
        (locals.var_phieot, locals.var_phieot_dn3, locals.var_phieot_dn4, locals.var_phieot_dn5, locals.var_phieot_dn6, locals.var_phieot_dn7, locals.var_phieot_dn8, locals.var_phieot_dn9, locals.var_phieot_dn10, locals.var_phieot_dn11, locals.var_phieot_dn12,)
    }
};
        locals.var_phieot = assign7120_e8141;
        locals.var_phieot_dn3 = assign7120_e8141_d_n3;
        locals.var_phieot_dn4 = assign7120_e8141_d_n4;
        locals.var_phieot_dn5 = assign7120_e8141_d_n5;
        locals.var_phieot_dn6 = assign7120_e8141_d_n6;
        locals.var_phieot_dn7 = assign7120_e8141_d_n7;
        locals.var_phieot_dn8 = assign7120_e8141_d_n8;
        locals.var_phieot_dn9 = assign7120_e8141_d_n9;
        locals.var_phieot_dn10 = assign7120_e8141_d_n10;
        locals.var_phieot_dn11 = assign7120_e8141_d_n11;
        locals.var_phieot_dn12 = assign7120_e8141_d_n12;

        let (assign7130_e8147, assign7130_e8147_d_n3, assign7130_e8147_d_n4, assign7130_e8147_d_n5, assign7130_e8147_d_n6, assign7130_e8147_d_n7, assign7130_e8147_d_n8, assign7130_e8147_d_n9, assign7130_e8147_d_n10, assign7130_e8147_d_n11, assign7130_e8147_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7130_e8145: f64 = (locals.var_phieot).sqrt();
        (assign7130_e8145, (locals.var_phieot_dn3 / (2.0 * assign7130_e8145)), (locals.var_phieot_dn4 / (2.0 * assign7130_e8145)), (locals.var_phieot_dn5 / (2.0 * assign7130_e8145)), (locals.var_phieot_dn6 / (2.0 * assign7130_e8145)), (locals.var_phieot_dn7 / (2.0 * assign7130_e8145)), (locals.var_phieot_dn8 / (2.0 * assign7130_e8145)), (locals.var_phieot_dn9 / (2.0 * assign7130_e8145)), (locals.var_phieot_dn10 / (2.0 * assign7130_e8145)), (locals.var_phieot_dn11 / (2.0 * assign7130_e8145)), (locals.var_phieot_dn12 / (2.0 * assign7130_e8145)),)
    } else {
        (locals.var_sqrtphieot, locals.var_sqrtphieot_dn3, locals.var_sqrtphieot_dn4, locals.var_sqrtphieot_dn5, locals.var_sqrtphieot_dn6, locals.var_sqrtphieot_dn7, locals.var_sqrtphieot_dn8, locals.var_sqrtphieot_dn9, locals.var_sqrtphieot_dn10, locals.var_sqrtphieot_dn11, locals.var_sqrtphieot_dn12,)
    }
};
        locals.var_sqrtphieot = assign7130_e8147;
        locals.var_sqrtphieot_dn3 = assign7130_e8147_d_n3;
        locals.var_sqrtphieot_dn4 = assign7130_e8147_d_n4;
        locals.var_sqrtphieot_dn5 = assign7130_e8147_d_n5;
        locals.var_sqrtphieot_dn6 = assign7130_e8147_d_n6;
        locals.var_sqrtphieot_dn7 = assign7130_e8147_d_n7;
        locals.var_sqrtphieot_dn8 = assign7130_e8147_d_n8;
        locals.var_sqrtphieot_dn9 = assign7130_e8147_d_n9;
        locals.var_sqrtphieot_dn10 = assign7130_e8147_d_n10;
        locals.var_sqrtphieot_dn11 = assign7130_e8147_d_n11;
        locals.var_sqrtphieot_dn12 = assign7130_e8147_d_n12;

        let (assign7140_e8154, assign7140_e8154_d_n3, assign7140_e8154_d_n4, assign7140_e8154_d_n5, assign7140_e8154_d_n6, assign7140_e8154_d_n7, assign7140_e8154_d_n8, assign7140_e8154_d_n9, assign7140_e8154_d_n10, assign7140_e8154_d_n11, assign7140_e8154_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7140_e8152: f64 = (locals.var_here_b4soivfb + locals.var_phieot);
        (assign7140_e8152, (locals.var_here_b4soivfb_dn3 + locals.var_phieot_dn3), (locals.var_here_b4soivfb_dn4 + locals.var_phieot_dn4), (locals.var_here_b4soivfb_dn5 + locals.var_phieot_dn5), (locals.var_here_b4soivfb_dn6 + locals.var_phieot_dn6), (locals.var_here_b4soivfb_dn7 + locals.var_phieot_dn7), (locals.var_here_b4soivfb_dn8 + locals.var_phieot_dn8), (locals.var_here_b4soivfb_dn9 + locals.var_phieot_dn9), (locals.var_here_b4soivfb_dn10 + locals.var_phieot_dn10), (locals.var_here_b4soivfb_dn11 + locals.var_phieot_dn11), (locals.var_here_b4soivfb_dn12 + locals.var_phieot_dn12),)
    } else {
        (locals.var_tmp2, locals.var_tmp2_dn3, locals.var_tmp2_dn4, locals.var_tmp2_dn5, locals.var_tmp2_dn6, locals.var_tmp2_dn7, locals.var_tmp2_dn8, locals.var_tmp2_dn9, locals.var_tmp2_dn10, locals.var_tmp2_dn11, locals.var_tmp2_dn12,)
    }
};
        locals.var_tmp2 = assign7140_e8154;
        locals.var_tmp2_dn3 = assign7140_e8154_d_n3;
        locals.var_tmp2_dn4 = assign7140_e8154_d_n4;
        locals.var_tmp2_dn5 = assign7140_e8154_d_n5;
        locals.var_tmp2_dn6 = assign7140_e8154_d_n6;
        locals.var_tmp2_dn7 = assign7140_e8154_d_n7;
        locals.var_tmp2_dn8 = assign7140_e8154_d_n8;
        locals.var_tmp2_dn9 = assign7140_e8154_d_n9;
        locals.var_tmp2_dn10 = assign7140_e8154_d_n10;
        locals.var_tmp2_dn11 = assign7140_e8154_d_n11;
        locals.var_tmp2_dn12 = assign7140_e8154_d_n12;

    }

    pub(super) fn stamp_transient_block_20(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign7150_e8161,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7150_e8159: f64 = (p.p37 * p.p56);
        (assign7150_e8159,)
    } else {
        (locals.var_vddeot,)
    }
};
        locals.var_vddeot = assign7150_e8161;

        let (assign7160_e8168, assign7160_e8168_d_n3, assign7160_e8168_d_n4, assign7160_e8168_d_n5, assign7160_e8168_d_n6, assign7160_e8168_d_n7, assign7160_e8168_d_n8, assign7160_e8168_d_n9, assign7160_e8168_d_n10, assign7160_e8168_d_n11, assign7160_e8168_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7160_e8166: f64 = (p.p60 * 8.85418e-12);
        (assign7160_e8166, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign7160_e8168;
        locals.var_t0_dn3 = assign7160_e8168_d_n3;
        locals.var_t0_dn4 = assign7160_e8168_d_n4;
        locals.var_t0_dn5 = assign7160_e8168_d_n5;
        locals.var_t0_dn6 = assign7160_e8168_d_n6;
        locals.var_t0_dn7 = assign7160_e8168_d_n7;
        locals.var_t0_dn8 = assign7160_e8168_d_n8;
        locals.var_t0_dn9 = assign7160_e8168_d_n9;
        locals.var_t0_dn10 = assign7160_e8168_d_n10;
        locals.var_t0_dn11 = assign7160_e8168_d_n11;
        locals.var_t0_dn12 = assign7160_e8168_d_n12;

        let assign7170_e8183: f64 = if ((((locals.var_pparam_b4soingate > 1e18) && (locals.var_pparam_b4soingate < 1e25)) && (locals.var_vddeot > locals.var_tmp2)) && (locals.var_t0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard595 = assign7170_e8183;

        let (assign7180_e8200, assign7180_e8200_d_n3, assign7180_e8200_d_n4, assign7180_e8200_d_n5, assign7180_e8200_d_n6, assign7180_e8200_d_n7, assign7180_e8200_d_n8, assign7180_e8200_d_n9, assign7180_e8200_d_n10, assign7180_e8200_d_n11, assign7180_e8200_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard595 != 0.0)) {
        let assign7180_e8190: f64 = (1000000.0 * 1.602176462e-19);
        let assign7180_e8192: f64 = (assign7180_e8190 * locals.var_epssub);
        let assign7180_e8194: f64 = (assign7180_e8192 * locals.var_pparam_b4soingate);
        let assign7180_e8197: f64 = (locals.var_b4soicox * locals.var_b4soicox);
        let assign7180_e8198: f64 = (assign7180_e8194 / assign7180_e8197);
        (assign7180_e8198, ((assign7180_e8192 * locals.var_pparam_b4soingate_dn3) / assign7180_e8197), ((assign7180_e8192 * locals.var_pparam_b4soingate_dn4) / assign7180_e8197), ((assign7180_e8192 * locals.var_pparam_b4soingate_dn5) / assign7180_e8197), ((assign7180_e8192 * locals.var_pparam_b4soingate_dn6) / assign7180_e8197), ((assign7180_e8192 * locals.var_pparam_b4soingate_dn7) / assign7180_e8197), ((assign7180_e8192 * locals.var_pparam_b4soingate_dn8) / assign7180_e8197), ((assign7180_e8192 * locals.var_pparam_b4soingate_dn9) / assign7180_e8197), ((assign7180_e8192 * locals.var_pparam_b4soingate_dn10) / assign7180_e8197), ((assign7180_e8192 * locals.var_pparam_b4soingate_dn11) / assign7180_e8197), ((assign7180_e8192 * locals.var_pparam_b4soingate_dn12) / assign7180_e8197),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign7180_e8200;
        locals.var_t1_dn3 = assign7180_e8200_d_n3;
        locals.var_t1_dn4 = assign7180_e8200_d_n4;
        locals.var_t1_dn5 = assign7180_e8200_d_n5;
        locals.var_t1_dn6 = assign7180_e8200_d_n6;
        locals.var_t1_dn7 = assign7180_e8200_d_n7;
        locals.var_t1_dn8 = assign7180_e8200_d_n8;
        locals.var_t1_dn9 = assign7180_e8200_d_n9;
        locals.var_t1_dn10 = assign7180_e8200_d_n10;
        locals.var_t1_dn11 = assign7180_e8200_d_n11;
        locals.var_t1_dn12 = assign7180_e8200_d_n12;

        let (assign7190_e8216, assign7190_e8216_d_n3, assign7190_e8216_d_n4, assign7190_e8216_d_n5, assign7190_e8216_d_n6, assign7190_e8216_d_n7, assign7190_e8216_d_n8, assign7190_e8216_d_n9, assign7190_e8216_d_n10, assign7190_e8216_d_n11, assign7190_e8216_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard595 != 0.0)) {
        let assign7190_e8209: f64 = (locals.var_vddeot - locals.var_t0);
        let assign7190_e8210: f64 = (2.0 * assign7190_e8209);
        let assign7190_e8212: f64 = (assign7190_e8210 / locals.var_t1);
        let assign7190_e8213: f64 = (1.0 + assign7190_e8212);
        let assign7190_e8214: f64 = (assign7190_e8213).sqrt();
        (assign7190_e8214, (((((2.0 * (-locals.var_t0_dn3)) * locals.var_t1) - (assign7190_e8210 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)) / (2.0 * assign7190_e8214)), (((((2.0 * (-locals.var_t0_dn4)) * locals.var_t1) - (assign7190_e8210 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)) / (2.0 * assign7190_e8214)), (((((2.0 * (-locals.var_t0_dn5)) * locals.var_t1) - (assign7190_e8210 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)) / (2.0 * assign7190_e8214)), (((((2.0 * (-locals.var_t0_dn6)) * locals.var_t1) - (assign7190_e8210 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)) / (2.0 * assign7190_e8214)), (((((2.0 * (-locals.var_t0_dn7)) * locals.var_t1) - (assign7190_e8210 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)) / (2.0 * assign7190_e8214)), (((((2.0 * (-locals.var_t0_dn8)) * locals.var_t1) - (assign7190_e8210 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)) / (2.0 * assign7190_e8214)), (((((2.0 * (-locals.var_t0_dn9)) * locals.var_t1) - (assign7190_e8210 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)) / (2.0 * assign7190_e8214)), (((((2.0 * (-locals.var_t0_dn10)) * locals.var_t1) - (assign7190_e8210 * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)) / (2.0 * assign7190_e8214)), (((((2.0 * (-locals.var_t0_dn11)) * locals.var_t1) - (assign7190_e8210 * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)) / (2.0 * assign7190_e8214)), (((((2.0 * (-locals.var_t0_dn12)) * locals.var_t1) - (assign7190_e8210 * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1)) / (2.0 * assign7190_e8214)),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign7190_e8216;
        locals.var_t4_dn3 = assign7190_e8216_d_n3;
        locals.var_t4_dn4 = assign7190_e8216_d_n4;
        locals.var_t4_dn5 = assign7190_e8216_d_n5;
        locals.var_t4_dn6 = assign7190_e8216_d_n6;
        locals.var_t4_dn7 = assign7190_e8216_d_n7;
        locals.var_t4_dn8 = assign7190_e8216_d_n8;
        locals.var_t4_dn9 = assign7190_e8216_d_n9;
        locals.var_t4_dn10 = assign7190_e8216_d_n10;
        locals.var_t4_dn11 = assign7190_e8216_d_n11;
        locals.var_t4_dn12 = assign7190_e8216_d_n12;

        let (assign7200_e8227, assign7200_e8227_d_n3, assign7200_e8227_d_n4, assign7200_e8227_d_n5, assign7200_e8227_d_n6, assign7200_e8227_d_n7, assign7200_e8227_d_n8, assign7200_e8227_d_n9, assign7200_e8227_d_n10, assign7200_e8227_d_n11, assign7200_e8227_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard595 != 0.0)) {
        let assign7200_e8224: f64 = (locals.var_t4 - 1.0);
        let assign7200_e8225: f64 = (locals.var_t1 * assign7200_e8224);
        (assign7200_e8225, ((locals.var_t1_dn3 * assign7200_e8224) + (locals.var_t1 * locals.var_t4_dn3)), ((locals.var_t1_dn4 * assign7200_e8224) + (locals.var_t1 * locals.var_t4_dn4)), ((locals.var_t1_dn5 * assign7200_e8224) + (locals.var_t1 * locals.var_t4_dn5)), ((locals.var_t1_dn6 * assign7200_e8224) + (locals.var_t1 * locals.var_t4_dn6)), ((locals.var_t1_dn7 * assign7200_e8224) + (locals.var_t1 * locals.var_t4_dn7)), ((locals.var_t1_dn8 * assign7200_e8224) + (locals.var_t1 * locals.var_t4_dn8)), ((locals.var_t1_dn9 * assign7200_e8224) + (locals.var_t1 * locals.var_t4_dn9)), ((locals.var_t1_dn10 * assign7200_e8224) + (locals.var_t1 * locals.var_t4_dn10)), ((locals.var_t1_dn11 * assign7200_e8224) + (locals.var_t1 * locals.var_t4_dn11)), ((locals.var_t1_dn12 * assign7200_e8224) + (locals.var_t1 * locals.var_t4_dn12)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign7200_e8227;
        locals.var_t2_dn3 = assign7200_e8227_d_n3;
        locals.var_t2_dn4 = assign7200_e8227_d_n4;
        locals.var_t2_dn5 = assign7200_e8227_d_n5;
        locals.var_t2_dn6 = assign7200_e8227_d_n6;
        locals.var_t2_dn7 = assign7200_e8227_d_n7;
        locals.var_t2_dn8 = assign7200_e8227_d_n8;
        locals.var_t2_dn9 = assign7200_e8227_d_n9;
        locals.var_t2_dn10 = assign7200_e8227_d_n10;
        locals.var_t2_dn11 = assign7200_e8227_d_n11;
        locals.var_t2_dn12 = assign7200_e8227_d_n12;

        let (assign7210_e8240, assign7210_e8240_d_n3, assign7210_e8240_d_n4, assign7210_e8240_d_n5, assign7210_e8240_d_n6, assign7210_e8240_d_n7, assign7210_e8240_d_n8, assign7210_e8240_d_n9, assign7210_e8240_d_n10, assign7210_e8240_d_n11, assign7210_e8240_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard595 != 0.0)) {
        let assign7210_e8234: f64 = (0.5 * locals.var_t2);
        let assign7210_e8236: f64 = (assign7210_e8234 * locals.var_t2);
        let assign7210_e8238: f64 = (assign7210_e8236 / locals.var_t1);
        (assign7210_e8238, ((((((0.5 * locals.var_t2_dn3) * locals.var_t2) + (assign7210_e8234 * locals.var_t2_dn3)) * locals.var_t1) - (assign7210_e8236 * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), ((((((0.5 * locals.var_t2_dn4) * locals.var_t2) + (assign7210_e8234 * locals.var_t2_dn4)) * locals.var_t1) - (assign7210_e8236 * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), ((((((0.5 * locals.var_t2_dn5) * locals.var_t2) + (assign7210_e8234 * locals.var_t2_dn5)) * locals.var_t1) - (assign7210_e8236 * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), ((((((0.5 * locals.var_t2_dn6) * locals.var_t2) + (assign7210_e8234 * locals.var_t2_dn6)) * locals.var_t1) - (assign7210_e8236 * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), ((((((0.5 * locals.var_t2_dn7) * locals.var_t2) + (assign7210_e8234 * locals.var_t2_dn7)) * locals.var_t1) - (assign7210_e8236 * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), ((((((0.5 * locals.var_t2_dn8) * locals.var_t2) + (assign7210_e8234 * locals.var_t2_dn8)) * locals.var_t1) - (assign7210_e8236 * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), ((((((0.5 * locals.var_t2_dn9) * locals.var_t2) + (assign7210_e8234 * locals.var_t2_dn9)) * locals.var_t1) - (assign7210_e8236 * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), ((((((0.5 * locals.var_t2_dn10) * locals.var_t2) + (assign7210_e8234 * locals.var_t2_dn10)) * locals.var_t1) - (assign7210_e8236 * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), ((((((0.5 * locals.var_t2_dn11) * locals.var_t2) + (assign7210_e8234 * locals.var_t2_dn11)) * locals.var_t1) - (assign7210_e8236 * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)), ((((((0.5 * locals.var_t2_dn12) * locals.var_t2) + (assign7210_e8234 * locals.var_t2_dn12)) * locals.var_t1) - (assign7210_e8236 * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign7210_e8240;
        locals.var_t3_dn3 = assign7210_e8240_d_n3;
        locals.var_t3_dn4 = assign7210_e8240_d_n4;
        locals.var_t3_dn5 = assign7210_e8240_d_n5;
        locals.var_t3_dn6 = assign7210_e8240_d_n6;
        locals.var_t3_dn7 = assign7210_e8240_d_n7;
        locals.var_t3_dn8 = assign7210_e8240_d_n8;
        locals.var_t3_dn9 = assign7210_e8240_d_n9;
        locals.var_t3_dn10 = assign7210_e8240_d_n10;
        locals.var_t3_dn11 = assign7210_e8240_d_n11;
        locals.var_t3_dn12 = assign7210_e8240_d_n12;

        let (assign7220_e8251, assign7220_e8251_d_n3, assign7220_e8251_d_n4, assign7220_e8251_d_n5, assign7220_e8251_d_n6, assign7220_e8251_d_n7, assign7220_e8251_d_n8, assign7220_e8251_d_n9, assign7220_e8251_d_n10, assign7220_e8251_d_n11, assign7220_e8251_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard595 != 0.0)) {
        let assign7220_e8247: f64 = (p.p1034 - locals.var_t3);
        let assign7220_e8249: f64 = (assign7220_e8247 - 0.05);
        (assign7220_e8249, (-locals.var_t3_dn3), (-locals.var_t3_dn4), (-locals.var_t3_dn5), (-locals.var_t3_dn6), (-locals.var_t3_dn7), (-locals.var_t3_dn8), (-locals.var_t3_dn9), (-locals.var_t3_dn10), (-locals.var_t3_dn11), (-locals.var_t3_dn12),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12,)
    }
};
        locals.var_t7 = assign7220_e8251;
        locals.var_t7_dn3 = assign7220_e8251_d_n3;
        locals.var_t7_dn4 = assign7220_e8251_d_n4;
        locals.var_t7_dn5 = assign7220_e8251_d_n5;
        locals.var_t7_dn6 = assign7220_e8251_d_n6;
        locals.var_t7_dn7 = assign7220_e8251_d_n7;
        locals.var_t7_dn8 = assign7220_e8251_d_n8;
        locals.var_t7_dn9 = assign7220_e8251_d_n9;
        locals.var_t7_dn10 = assign7220_e8251_d_n10;
        locals.var_t7_dn11 = assign7220_e8251_d_n11;
        locals.var_t7_dn12 = assign7220_e8251_d_n12;

        let (assign7230_e8263, assign7230_e8263_d_n3, assign7230_e8263_d_n4, assign7230_e8263_d_n5, assign7230_e8263_d_n6, assign7230_e8263_d_n7, assign7230_e8263_d_n8, assign7230_e8263_d_n9, assign7230_e8263_d_n10, assign7230_e8263_d_n11, assign7230_e8263_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard595 != 0.0)) {
        let assign7230_e8258: f64 = (locals.var_t7 * locals.var_t7);
        let assign7230_e8260: f64 = (assign7230_e8258 + 0.224);
        let assign7230_e8261: f64 = (assign7230_e8260).sqrt();
        (assign7230_e8261, (((locals.var_t7_dn3 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn3)) / (2.0 * assign7230_e8261)), (((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)) / (2.0 * assign7230_e8261)), (((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)) / (2.0 * assign7230_e8261)), (((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)) / (2.0 * assign7230_e8261)), (((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)) / (2.0 * assign7230_e8261)), (((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)) / (2.0 * assign7230_e8261)), (((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)) / (2.0 * assign7230_e8261)), (((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)) / (2.0 * assign7230_e8261)), (((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)) / (2.0 * assign7230_e8261)), (((locals.var_t7_dn12 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn12)) / (2.0 * assign7230_e8261)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12,)
    }
};
        locals.var_t6 = assign7230_e8263;
        locals.var_t6_dn3 = assign7230_e8263_d_n3;
        locals.var_t6_dn4 = assign7230_e8263_d_n4;
        locals.var_t6_dn5 = assign7230_e8263_d_n5;
        locals.var_t6_dn6 = assign7230_e8263_d_n6;
        locals.var_t6_dn7 = assign7230_e8263_d_n7;
        locals.var_t6_dn8 = assign7230_e8263_d_n8;
        locals.var_t6_dn9 = assign7230_e8263_d_n9;
        locals.var_t6_dn10 = assign7230_e8263_d_n10;
        locals.var_t6_dn11 = assign7230_e8263_d_n11;
        locals.var_t6_dn12 = assign7230_e8263_d_n12;

        let (assign7240_e8276, assign7240_e8276_d_n3, assign7240_e8276_d_n4, assign7240_e8276_d_n5, assign7240_e8276_d_n6, assign7240_e8276_d_n7, assign7240_e8276_d_n8, assign7240_e8276_d_n9, assign7240_e8276_d_n10, assign7240_e8276_d_n11, assign7240_e8276_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard595 != 0.0)) {
        let assign7240_e8272: f64 = (locals.var_t7 + locals.var_t6);
        let assign7240_e8273: f64 = (0.5 * assign7240_e8272);
        let assign7240_e8274: f64 = (p.p1034 - assign7240_e8273);
        (assign7240_e8274, (-(0.5 * (locals.var_t7_dn3 + locals.var_t6_dn3))), (-(0.5 * (locals.var_t7_dn4 + locals.var_t6_dn4))), (-(0.5 * (locals.var_t7_dn5 + locals.var_t6_dn5))), (-(0.5 * (locals.var_t7_dn6 + locals.var_t6_dn6))), (-(0.5 * (locals.var_t7_dn7 + locals.var_t6_dn7))), (-(0.5 * (locals.var_t7_dn8 + locals.var_t6_dn8))), (-(0.5 * (locals.var_t7_dn9 + locals.var_t6_dn9))), (-(0.5 * (locals.var_t7_dn10 + locals.var_t6_dn10))), (-(0.5 * (locals.var_t7_dn11 + locals.var_t6_dn11))), (-(0.5 * (locals.var_t7_dn12 + locals.var_t6_dn12))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12,)
    }
};
        locals.var_t5 = assign7240_e8276;
        locals.var_t5_dn3 = assign7240_e8276_d_n3;
        locals.var_t5_dn4 = assign7240_e8276_d_n4;
        locals.var_t5_dn5 = assign7240_e8276_d_n5;
        locals.var_t5_dn6 = assign7240_e8276_d_n6;
        locals.var_t5_dn7 = assign7240_e8276_d_n7;
        locals.var_t5_dn8 = assign7240_e8276_d_n8;
        locals.var_t5_dn9 = assign7240_e8276_d_n9;
        locals.var_t5_dn10 = assign7240_e8276_d_n10;
        locals.var_t5_dn11 = assign7240_e8276_d_n11;
        locals.var_t5_dn12 = assign7240_e8276_d_n12;

        let (assign7250_e8285, assign7250_e8285_d_n3, assign7250_e8285_d_n4, assign7250_e8285_d_n5, assign7250_e8285_d_n6, assign7250_e8285_d_n7, assign7250_e8285_d_n8, assign7250_e8285_d_n9, assign7250_e8285_d_n10, assign7250_e8285_d_n11, assign7250_e8285_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard595 != 0.0)) {
        let assign7250_e8283: f64 = (locals.var_vddeot - locals.var_t5);
        (assign7250_e8283, (-locals.var_t5_dn3), (-locals.var_t5_dn4), (-locals.var_t5_dn5), (-locals.var_t5_dn6), (-locals.var_t5_dn7), (-locals.var_t5_dn8), (-locals.var_t5_dn9), (-locals.var_t5_dn10), (-locals.var_t5_dn11), (-locals.var_t5_dn12),)
    } else {
        (locals.var_vgs_eff, locals.var_vgs_eff_dn3, locals.var_vgs_eff_dn4, locals.var_vgs_eff_dn5, locals.var_vgs_eff_dn6, locals.var_vgs_eff_dn7, locals.var_vgs_eff_dn8, locals.var_vgs_eff_dn9, locals.var_vgs_eff_dn10, locals.var_vgs_eff_dn11, locals.var_vgs_eff_dn12,)
    }
};
        locals.var_vgs_eff = assign7250_e8285;
        locals.var_vgs_eff_dn3 = assign7250_e8285_d_n3;
        locals.var_vgs_eff_dn4 = assign7250_e8285_d_n4;
        locals.var_vgs_eff_dn5 = assign7250_e8285_d_n5;
        locals.var_vgs_eff_dn6 = assign7250_e8285_d_n6;
        locals.var_vgs_eff_dn7 = assign7250_e8285_d_n7;
        locals.var_vgs_eff_dn8 = assign7250_e8285_d_n8;
        locals.var_vgs_eff_dn9 = assign7250_e8285_d_n9;
        locals.var_vgs_eff_dn10 = assign7250_e8285_d_n10;
        locals.var_vgs_eff_dn11 = assign7250_e8285_d_n11;
        locals.var_vgs_eff_dn12 = assign7250_e8285_d_n12;

        let (assign7260_e8293, assign7260_e8293_d_n3, assign7260_e8293_d_n4, assign7260_e8293_d_n5, assign7260_e8293_d_n6, assign7260_e8293_d_n7, assign7260_e8293_d_n8, assign7260_e8293_d_n9, assign7260_e8293_d_n10, assign7260_e8293_d_n11, assign7260_e8293_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard595 == 0.0)) {
        (locals.var_vddeot, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgs_eff, locals.var_vgs_eff_dn3, locals.var_vgs_eff_dn4, locals.var_vgs_eff_dn5, locals.var_vgs_eff_dn6, locals.var_vgs_eff_dn7, locals.var_vgs_eff_dn8, locals.var_vgs_eff_dn9, locals.var_vgs_eff_dn10, locals.var_vgs_eff_dn11, locals.var_vgs_eff_dn12,)
    }
};
        locals.var_vgs_eff = assign7260_e8293;
        locals.var_vgs_eff_dn3 = assign7260_e8293_d_n3;
        locals.var_vgs_eff_dn4 = assign7260_e8293_d_n4;
        locals.var_vgs_eff_dn5 = assign7260_e8293_d_n5;
        locals.var_vgs_eff_dn6 = assign7260_e8293_d_n6;
        locals.var_vgs_eff_dn7 = assign7260_e8293_d_n7;
        locals.var_vgs_eff_dn8 = assign7260_e8293_d_n8;
        locals.var_vgs_eff_dn9 = assign7260_e8293_d_n9;
        locals.var_vgs_eff_dn10 = assign7260_e8293_d_n10;
        locals.var_vgs_eff_dn11 = assign7260_e8293_d_n11;
        locals.var_vgs_eff_dn12 = assign7260_e8293_d_n12;

        let (assign7270_e8300, assign7270_e8300_d_n3, assign7270_e8300_d_n4, assign7270_e8300_d_n5, assign7270_e8300_d_n6, assign7270_e8300_d_n7, assign7270_e8300_d_n8, assign7270_e8300_d_n9, assign7270_e8300_d_n10, assign7270_e8300_d_n11, assign7270_e8300_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7270_e8298: f64 = (locals.var_vbieot - locals.var_phieot);
        (assign7270_e8298, (locals.var_vbieot_dn3 - locals.var_phieot_dn3), (locals.var_vbieot_dn4 - locals.var_phieot_dn4), (locals.var_vbieot_dn5 - locals.var_phieot_dn5), (locals.var_vbieot_dn6 - locals.var_phieot_dn6), (locals.var_vbieot_dn7 - locals.var_phieot_dn7), (locals.var_vbieot_dn8 - locals.var_phieot_dn8), (locals.var_vbieot_dn9 - locals.var_phieot_dn9), (locals.var_vbieot_dn10 - locals.var_phieot_dn10), (locals.var_vbieot_dn11 - locals.var_phieot_dn11), (locals.var_vbieot_dn12 - locals.var_phieot_dn12),)
    } else {
        (locals.var_v0, locals.var_v0_dn3, locals.var_v0_dn4, locals.var_v0_dn5, locals.var_v0_dn6, locals.var_v0_dn7, locals.var_v0_dn8, locals.var_v0_dn9, locals.var_v0_dn10, locals.var_v0_dn11, locals.var_v0_dn12,)
    }
};
        locals.var_v0 = assign7270_e8300;
        locals.var_v0_dn3 = assign7270_e8300_d_n3;
        locals.var_v0_dn4 = assign7270_e8300_d_n4;
        locals.var_v0_dn5 = assign7270_e8300_d_n5;
        locals.var_v0_dn6 = assign7270_e8300_d_n6;
        locals.var_v0_dn7 = assign7270_e8300_d_n7;
        locals.var_v0_dn8 = assign7270_e8300_d_n8;
        locals.var_v0_dn9 = assign7270_e8300_d_n9;
        locals.var_v0_dn10 = assign7270_e8300_d_n10;
        locals.var_v0_dn11 = assign7270_e8300_d_n11;
        locals.var_v0_dn12 = assign7270_e8300_d_n12;

        let (assign7280_e8305, assign7280_e8305_d_n3, assign7280_e8305_d_n4, assign7280_e8305_d_n5, assign7280_e8305_d_n6, assign7280_e8305_d_n7, assign7280_e8305_d_n8, assign7280_e8305_d_n9, assign7280_e8305_d_n10, assign7280_e8305_d_n11, assign7280_e8305_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        (locals.var_pparam_b4soisqrtxdep0, locals.var_pparam_b4soisqrtxdep0_dn3, locals.var_pparam_b4soisqrtxdep0_dn4, locals.var_pparam_b4soisqrtxdep0_dn5, locals.var_pparam_b4soisqrtxdep0_dn6, locals.var_pparam_b4soisqrtxdep0_dn7, locals.var_pparam_b4soisqrtxdep0_dn8, locals.var_pparam_b4soisqrtxdep0_dn9, locals.var_pparam_b4soisqrtxdep0_dn10, locals.var_pparam_b4soisqrtxdep0_dn11, locals.var_pparam_b4soisqrtxdep0_dn12,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign7280_e8305;
        locals.var_t3_dn3 = assign7280_e8305_d_n3;
        locals.var_t3_dn4 = assign7280_e8305_d_n4;
        locals.var_t3_dn5 = assign7280_e8305_d_n5;
        locals.var_t3_dn6 = assign7280_e8305_d_n6;
        locals.var_t3_dn7 = assign7280_e8305_d_n7;
        locals.var_t3_dn8 = assign7280_e8305_d_n8;
        locals.var_t3_dn9 = assign7280_e8305_d_n9;
        locals.var_t3_dn10 = assign7280_e8305_d_n10;
        locals.var_t3_dn11 = assign7280_e8305_d_n11;
        locals.var_t3_dn12 = assign7280_e8305_d_n12;

        let (assign7290_e8312, assign7290_e8312_d_n3, assign7290_e8312_d_n4, assign7290_e8312_d_n5, assign7290_e8312_d_n6, assign7290_e8312_d_n7, assign7290_e8312_d_n8, assign7290_e8312_d_n9, assign7290_e8312_d_n10, assign7290_e8312_d_n11, assign7290_e8312_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7290_e8310: f64 = (locals.var_b4soifactor1 * locals.var_t3);
        (assign7290_e8310, (locals.var_b4soifactor1 * locals.var_t3_dn3), (locals.var_b4soifactor1 * locals.var_t3_dn4), (locals.var_b4soifactor1 * locals.var_t3_dn5), (locals.var_b4soifactor1 * locals.var_t3_dn6), (locals.var_b4soifactor1 * locals.var_t3_dn7), (locals.var_b4soifactor1 * locals.var_t3_dn8), (locals.var_b4soifactor1 * locals.var_t3_dn9), (locals.var_b4soifactor1 * locals.var_t3_dn10), (locals.var_b4soifactor1 * locals.var_t3_dn11), (locals.var_b4soifactor1 * locals.var_t3_dn12),)
    } else {
        (locals.var_lt1, locals.var_lt1_dn3, locals.var_lt1_dn4, locals.var_lt1_dn5, locals.var_lt1_dn6, locals.var_lt1_dn7, locals.var_lt1_dn8, locals.var_lt1_dn9, locals.var_lt1_dn10, locals.var_lt1_dn11, locals.var_lt1_dn12,)
    }
};
        locals.var_lt1 = assign7290_e8312;
        locals.var_lt1_dn3 = assign7290_e8312_d_n3;
        locals.var_lt1_dn4 = assign7290_e8312_d_n4;
        locals.var_lt1_dn5 = assign7290_e8312_d_n5;
        locals.var_lt1_dn6 = assign7290_e8312_d_n6;
        locals.var_lt1_dn7 = assign7290_e8312_d_n7;
        locals.var_lt1_dn8 = assign7290_e8312_d_n8;
        locals.var_lt1_dn9 = assign7290_e8312_d_n9;
        locals.var_lt1_dn10 = assign7290_e8312_d_n10;
        locals.var_lt1_dn11 = assign7290_e8312_d_n11;
        locals.var_lt1_dn12 = assign7290_e8312_d_n12;

        let (assign7300_e8319, assign7300_e8319_d_n3, assign7300_e8319_d_n4, assign7300_e8319_d_n5, assign7300_e8319_d_n6, assign7300_e8319_d_n7, assign7300_e8319_d_n8, assign7300_e8319_d_n9, assign7300_e8319_d_n10, assign7300_e8319_d_n11, assign7300_e8319_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7300_e8317: f64 = (locals.var_b4soifactor1 * locals.var_t3);
        (assign7300_e8317, (locals.var_b4soifactor1 * locals.var_t3_dn3), (locals.var_b4soifactor1 * locals.var_t3_dn4), (locals.var_b4soifactor1 * locals.var_t3_dn5), (locals.var_b4soifactor1 * locals.var_t3_dn6), (locals.var_b4soifactor1 * locals.var_t3_dn7), (locals.var_b4soifactor1 * locals.var_t3_dn8), (locals.var_b4soifactor1 * locals.var_t3_dn9), (locals.var_b4soifactor1 * locals.var_t3_dn10), (locals.var_b4soifactor1 * locals.var_t3_dn11), (locals.var_b4soifactor1 * locals.var_t3_dn12),)
    } else {
        (locals.var_ltw, locals.var_ltw_dn3, locals.var_ltw_dn4, locals.var_ltw_dn5, locals.var_ltw_dn6, locals.var_ltw_dn7, locals.var_ltw_dn8, locals.var_ltw_dn9, locals.var_ltw_dn10, locals.var_ltw_dn11, locals.var_ltw_dn12,)
    }
};
        locals.var_ltw = assign7300_e8319;
        locals.var_ltw_dn3 = assign7300_e8319_d_n3;
        locals.var_ltw_dn4 = assign7300_e8319_d_n4;
        locals.var_ltw_dn5 = assign7300_e8319_d_n5;
        locals.var_ltw_dn6 = assign7300_e8319_d_n6;
        locals.var_ltw_dn7 = assign7300_e8319_d_n7;
        locals.var_ltw_dn8 = assign7300_e8319_d_n8;
        locals.var_ltw_dn9 = assign7300_e8319_d_n9;
        locals.var_ltw_dn10 = assign7300_e8319_d_n10;
        locals.var_ltw_dn11 = assign7300_e8319_d_n11;
        locals.var_ltw_dn12 = assign7300_e8319_d_n12;

        let (assign7310_e8331, assign7310_e8331_d_n3, assign7310_e8331_d_n4, assign7310_e8331_d_n5, assign7310_e8331_d_n6, assign7310_e8331_d_n7, assign7310_e8331_d_n8, assign7310_e8331_d_n9, assign7310_e8331_d_n10, assign7310_e8331_d_n11, assign7310_e8331_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7310_e8323: f64 = (-0.5);
        let assign7310_e8325: f64 = (assign7310_e8323 * locals.var_pparam_b4soidvt1);
        let assign7310_e8327: f64 = (assign7310_e8325 * p.p54);
        let assign7310_e8329: f64 = (assign7310_e8327 / locals.var_lt1);
        (assign7310_e8329, (((((assign7310_e8323 * locals.var_pparam_b4soidvt1_dn3) * p.p54) * locals.var_lt1) - (assign7310_e8327 * locals.var_lt1_dn3)) / (locals.var_lt1 * locals.var_lt1)), (((((assign7310_e8323 * locals.var_pparam_b4soidvt1_dn4) * p.p54) * locals.var_lt1) - (assign7310_e8327 * locals.var_lt1_dn4)) / (locals.var_lt1 * locals.var_lt1)), (((((assign7310_e8323 * locals.var_pparam_b4soidvt1_dn5) * p.p54) * locals.var_lt1) - (assign7310_e8327 * locals.var_lt1_dn5)) / (locals.var_lt1 * locals.var_lt1)), (((((assign7310_e8323 * locals.var_pparam_b4soidvt1_dn6) * p.p54) * locals.var_lt1) - (assign7310_e8327 * locals.var_lt1_dn6)) / (locals.var_lt1 * locals.var_lt1)), (((((assign7310_e8323 * locals.var_pparam_b4soidvt1_dn7) * p.p54) * locals.var_lt1) - (assign7310_e8327 * locals.var_lt1_dn7)) / (locals.var_lt1 * locals.var_lt1)), (((((assign7310_e8323 * locals.var_pparam_b4soidvt1_dn8) * p.p54) * locals.var_lt1) - (assign7310_e8327 * locals.var_lt1_dn8)) / (locals.var_lt1 * locals.var_lt1)), (((((assign7310_e8323 * locals.var_pparam_b4soidvt1_dn9) * p.p54) * locals.var_lt1) - (assign7310_e8327 * locals.var_lt1_dn9)) / (locals.var_lt1 * locals.var_lt1)), (((((assign7310_e8323 * locals.var_pparam_b4soidvt1_dn10) * p.p54) * locals.var_lt1) - (assign7310_e8327 * locals.var_lt1_dn10)) / (locals.var_lt1 * locals.var_lt1)), (((((assign7310_e8323 * locals.var_pparam_b4soidvt1_dn11) * p.p54) * locals.var_lt1) - (assign7310_e8327 * locals.var_lt1_dn11)) / (locals.var_lt1 * locals.var_lt1)), (((((assign7310_e8323 * locals.var_pparam_b4soidvt1_dn12) * p.p54) * locals.var_lt1) - (assign7310_e8327 * locals.var_lt1_dn12)) / (locals.var_lt1 * locals.var_lt1)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign7310_e8331;
        locals.var_t0_dn3 = assign7310_e8331_d_n3;
        locals.var_t0_dn4 = assign7310_e8331_d_n4;
        locals.var_t0_dn5 = assign7310_e8331_d_n5;
        locals.var_t0_dn6 = assign7310_e8331_d_n6;
        locals.var_t0_dn7 = assign7310_e8331_d_n7;
        locals.var_t0_dn8 = assign7310_e8331_d_n8;
        locals.var_t0_dn9 = assign7310_e8331_d_n9;
        locals.var_t0_dn10 = assign7310_e8331_d_n10;
        locals.var_t0_dn11 = assign7310_e8331_d_n11;
        locals.var_t0_dn12 = assign7310_e8331_d_n12;

        let assign7320_e8334: f64 = (-100.0);
        let assign7320_e8335: f64 = if locals.var_t0 > assign7320_e8334 { 1.0 } else { 0.0 };
        locals.var_guard596 = assign7320_e8335;

        let (assign7330_e8343, assign7330_e8343_d_n3, assign7330_e8343_d_n4, assign7330_e8343_d_n5, assign7330_e8343_d_n6, assign7330_e8343_d_n7, assign7330_e8343_d_n8, assign7330_e8343_d_n9, assign7330_e8343_d_n10, assign7330_e8343_d_n11, assign7330_e8343_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard596 != 0.0)) {
        let assign7330_e8341: f64 = (locals.var_t0).exp();
        (assign7330_e8341, (assign7330_e8341 * locals.var_t0_dn3), (assign7330_e8341 * locals.var_t0_dn4), (assign7330_e8341 * locals.var_t0_dn5), (assign7330_e8341 * locals.var_t0_dn6), (assign7330_e8341 * locals.var_t0_dn7), (assign7330_e8341 * locals.var_t0_dn8), (assign7330_e8341 * locals.var_t0_dn9), (assign7330_e8341 * locals.var_t0_dn10), (assign7330_e8341 * locals.var_t0_dn11), (assign7330_e8341 * locals.var_t0_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign7330_e8343;
        locals.var_t1_dn3 = assign7330_e8343_d_n3;
        locals.var_t1_dn4 = assign7330_e8343_d_n4;
        locals.var_t1_dn5 = assign7330_e8343_d_n5;
        locals.var_t1_dn6 = assign7330_e8343_d_n6;
        locals.var_t1_dn7 = assign7330_e8343_d_n7;
        locals.var_t1_dn8 = assign7330_e8343_d_n8;
        locals.var_t1_dn9 = assign7330_e8343_d_n9;
        locals.var_t1_dn10 = assign7330_e8343_d_n10;
        locals.var_t1_dn11 = assign7330_e8343_d_n11;
        locals.var_t1_dn12 = assign7330_e8343_d_n12;

        let (assign7340_e8356, assign7340_e8356_d_n3, assign7340_e8356_d_n4, assign7340_e8356_d_n5, assign7340_e8356_d_n6, assign7340_e8356_d_n7, assign7340_e8356_d_n8, assign7340_e8356_d_n9, assign7340_e8356_d_n10, assign7340_e8356_d_n11, assign7340_e8356_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard596 != 0.0)) {
        let assign7340_e8352: f64 = (2.0 * locals.var_t1);
        let assign7340_e8353: f64 = (1.0 + assign7340_e8352);
        let assign7340_e8354: f64 = (locals.var_t1 * assign7340_e8353);
        (assign7340_e8354, ((locals.var_t1_dn3 * assign7340_e8353) + (locals.var_t1 * (2.0 * locals.var_t1_dn3))), ((locals.var_t1_dn4 * assign7340_e8353) + (locals.var_t1 * (2.0 * locals.var_t1_dn4))), ((locals.var_t1_dn5 * assign7340_e8353) + (locals.var_t1 * (2.0 * locals.var_t1_dn5))), ((locals.var_t1_dn6 * assign7340_e8353) + (locals.var_t1 * (2.0 * locals.var_t1_dn6))), ((locals.var_t1_dn7 * assign7340_e8353) + (locals.var_t1 * (2.0 * locals.var_t1_dn7))), ((locals.var_t1_dn8 * assign7340_e8353) + (locals.var_t1 * (2.0 * locals.var_t1_dn8))), ((locals.var_t1_dn9 * assign7340_e8353) + (locals.var_t1 * (2.0 * locals.var_t1_dn9))), ((locals.var_t1_dn10 * assign7340_e8353) + (locals.var_t1 * (2.0 * locals.var_t1_dn10))), ((locals.var_t1_dn11 * assign7340_e8353) + (locals.var_t1 * (2.0 * locals.var_t1_dn11))), ((locals.var_t1_dn12 * assign7340_e8353) + (locals.var_t1 * (2.0 * locals.var_t1_dn12))),)
    } else {
        (locals.var_theta0, locals.var_theta0_dn3, locals.var_theta0_dn4, locals.var_theta0_dn5, locals.var_theta0_dn6, locals.var_theta0_dn7, locals.var_theta0_dn8, locals.var_theta0_dn9, locals.var_theta0_dn10, locals.var_theta0_dn11, locals.var_theta0_dn12,)
    }
};
        locals.var_theta0 = assign7340_e8356;
        locals.var_theta0_dn3 = assign7340_e8356_d_n3;
        locals.var_theta0_dn4 = assign7340_e8356_d_n4;
        locals.var_theta0_dn5 = assign7340_e8356_d_n5;
        locals.var_theta0_dn6 = assign7340_e8356_d_n6;
        locals.var_theta0_dn7 = assign7340_e8356_d_n7;
        locals.var_theta0_dn8 = assign7340_e8356_d_n8;
        locals.var_theta0_dn9 = assign7340_e8356_d_n9;
        locals.var_theta0_dn10 = assign7340_e8356_d_n10;
        locals.var_theta0_dn11 = assign7340_e8356_d_n11;
        locals.var_theta0_dn12 = assign7340_e8356_d_n12;

        let (assign7350_e8364, assign7350_e8364_d_n3, assign7350_e8364_d_n4, assign7350_e8364_d_n5, assign7350_e8364_d_n6, assign7350_e8364_d_n7, assign7350_e8364_d_n8, assign7350_e8364_d_n9, assign7350_e8364_d_n10, assign7350_e8364_d_n11, assign7350_e8364_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard596 == 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign7350_e8364;
        locals.var_t1_dn3 = assign7350_e8364_d_n3;
        locals.var_t1_dn4 = assign7350_e8364_d_n4;
        locals.var_t1_dn5 = assign7350_e8364_d_n5;
        locals.var_t1_dn6 = assign7350_e8364_d_n6;
        locals.var_t1_dn7 = assign7350_e8364_d_n7;
        locals.var_t1_dn8 = assign7350_e8364_d_n8;
        locals.var_t1_dn9 = assign7350_e8364_d_n9;
        locals.var_t1_dn10 = assign7350_e8364_d_n10;
        locals.var_t1_dn11 = assign7350_e8364_d_n11;
        locals.var_t1_dn12 = assign7350_e8364_d_n12;

        let (assign7360_e8378, assign7360_e8378_d_n3, assign7360_e8378_d_n4, assign7360_e8378_d_n5, assign7360_e8378_d_n6, assign7360_e8378_d_n7, assign7360_e8378_d_n8, assign7360_e8378_d_n9, assign7360_e8378_d_n10, assign7360_e8378_d_n11, assign7360_e8378_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard596 == 0.0)) {
        let assign7360_e8374: f64 = (2.0 * locals.var_t1);
        let assign7360_e8375: f64 = (1.0 + assign7360_e8374);
        let assign7360_e8376: f64 = (locals.var_t1 * assign7360_e8375);
        (assign7360_e8376, ((locals.var_t1_dn3 * assign7360_e8375) + (locals.var_t1 * (2.0 * locals.var_t1_dn3))), ((locals.var_t1_dn4 * assign7360_e8375) + (locals.var_t1 * (2.0 * locals.var_t1_dn4))), ((locals.var_t1_dn5 * assign7360_e8375) + (locals.var_t1 * (2.0 * locals.var_t1_dn5))), ((locals.var_t1_dn6 * assign7360_e8375) + (locals.var_t1 * (2.0 * locals.var_t1_dn6))), ((locals.var_t1_dn7 * assign7360_e8375) + (locals.var_t1 * (2.0 * locals.var_t1_dn7))), ((locals.var_t1_dn8 * assign7360_e8375) + (locals.var_t1 * (2.0 * locals.var_t1_dn8))), ((locals.var_t1_dn9 * assign7360_e8375) + (locals.var_t1 * (2.0 * locals.var_t1_dn9))), ((locals.var_t1_dn10 * assign7360_e8375) + (locals.var_t1 * (2.0 * locals.var_t1_dn10))), ((locals.var_t1_dn11 * assign7360_e8375) + (locals.var_t1 * (2.0 * locals.var_t1_dn11))), ((locals.var_t1_dn12 * assign7360_e8375) + (locals.var_t1 * (2.0 * locals.var_t1_dn12))),)
    } else {
        (locals.var_theta0, locals.var_theta0_dn3, locals.var_theta0_dn4, locals.var_theta0_dn5, locals.var_theta0_dn6, locals.var_theta0_dn7, locals.var_theta0_dn8, locals.var_theta0_dn9, locals.var_theta0_dn10, locals.var_theta0_dn11, locals.var_theta0_dn12,)
    }
};
        locals.var_theta0 = assign7360_e8378;
        locals.var_theta0_dn3 = assign7360_e8378_d_n3;
        locals.var_theta0_dn4 = assign7360_e8378_d_n4;
        locals.var_theta0_dn5 = assign7360_e8378_d_n5;
        locals.var_theta0_dn6 = assign7360_e8378_d_n6;
        locals.var_theta0_dn7 = assign7360_e8378_d_n7;
        locals.var_theta0_dn8 = assign7360_e8378_d_n8;
        locals.var_theta0_dn9 = assign7360_e8378_d_n9;
        locals.var_theta0_dn10 = assign7360_e8378_d_n10;
        locals.var_theta0_dn11 = assign7360_e8378_d_n11;
        locals.var_theta0_dn12 = assign7360_e8378_d_n12;

        let (assign7370_e8387, assign7370_e8387_d_n3, assign7370_e8387_d_n4, assign7370_e8387_d_n5, assign7370_e8387_d_n6, assign7370_e8387_d_n7, assign7370_e8387_d_n8, assign7370_e8387_d_n9, assign7370_e8387_d_n10, assign7370_e8387_d_n11, assign7370_e8387_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7370_e8383: f64 = (locals.var_pparam_b4soinfactor * locals.var_epssub);
        let assign7370_e8385: f64 = (assign7370_e8383 / locals.var_pparam_b4soixdep0);
        (assign7370_e8385, ((((locals.var_pparam_b4soinfactor_dn3 * locals.var_epssub) * locals.var_pparam_b4soixdep0) - (assign7370_e8383 * locals.var_pparam_b4soixdep0_dn3)) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0)), ((((locals.var_pparam_b4soinfactor_dn4 * locals.var_epssub) * locals.var_pparam_b4soixdep0) - (assign7370_e8383 * locals.var_pparam_b4soixdep0_dn4)) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0)), ((((locals.var_pparam_b4soinfactor_dn5 * locals.var_epssub) * locals.var_pparam_b4soixdep0) - (assign7370_e8383 * locals.var_pparam_b4soixdep0_dn5)) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0)), ((((locals.var_pparam_b4soinfactor_dn6 * locals.var_epssub) * locals.var_pparam_b4soixdep0) - (assign7370_e8383 * locals.var_pparam_b4soixdep0_dn6)) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0)), ((((locals.var_pparam_b4soinfactor_dn7 * locals.var_epssub) * locals.var_pparam_b4soixdep0) - (assign7370_e8383 * locals.var_pparam_b4soixdep0_dn7)) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0)), ((((locals.var_pparam_b4soinfactor_dn8 * locals.var_epssub) * locals.var_pparam_b4soixdep0) - (assign7370_e8383 * locals.var_pparam_b4soixdep0_dn8)) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0)), ((((locals.var_pparam_b4soinfactor_dn9 * locals.var_epssub) * locals.var_pparam_b4soixdep0) - (assign7370_e8383 * locals.var_pparam_b4soixdep0_dn9)) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0)), ((((locals.var_pparam_b4soinfactor_dn10 * locals.var_epssub) * locals.var_pparam_b4soixdep0) - (assign7370_e8383 * locals.var_pparam_b4soixdep0_dn10)) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0)), ((((locals.var_pparam_b4soinfactor_dn11 * locals.var_epssub) * locals.var_pparam_b4soixdep0) - (assign7370_e8383 * locals.var_pparam_b4soixdep0_dn11)) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0)), ((((locals.var_pparam_b4soinfactor_dn12 * locals.var_epssub) * locals.var_pparam_b4soixdep0) - (assign7370_e8383 * locals.var_pparam_b4soixdep0_dn12)) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign7370_e8387;
        locals.var_t2_dn3 = assign7370_e8387_d_n3;
        locals.var_t2_dn4 = assign7370_e8387_d_n4;
        locals.var_t2_dn5 = assign7370_e8387_d_n5;
        locals.var_t2_dn6 = assign7370_e8387_d_n6;
        locals.var_t2_dn7 = assign7370_e8387_d_n7;
        locals.var_t2_dn8 = assign7370_e8387_d_n8;
        locals.var_t2_dn9 = assign7370_e8387_d_n9;
        locals.var_t2_dn10 = assign7370_e8387_d_n10;
        locals.var_t2_dn11 = assign7370_e8387_d_n11;
        locals.var_t2_dn12 = assign7370_e8387_d_n12;

        let (assign7380_e8392, assign7380_e8392_d_n3, assign7380_e8392_d_n4, assign7380_e8392_d_n5, assign7380_e8392_d_n6, assign7380_e8392_d_n7, assign7380_e8392_d_n8, assign7380_e8392_d_n9, assign7380_e8392_d_n10, assign7380_e8392_d_n11, assign7380_e8392_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        (locals.var_pparam_b4soicdsc, locals.var_pparam_b4soicdsc_dn3, locals.var_pparam_b4soicdsc_dn4, locals.var_pparam_b4soicdsc_dn5, locals.var_pparam_b4soicdsc_dn6, locals.var_pparam_b4soicdsc_dn7, locals.var_pparam_b4soicdsc_dn8, locals.var_pparam_b4soicdsc_dn9, locals.var_pparam_b4soicdsc_dn10, locals.var_pparam_b4soicdsc_dn11, locals.var_pparam_b4soicdsc_dn12,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign7380_e8392;
        locals.var_t3_dn3 = assign7380_e8392_d_n3;
        locals.var_t3_dn4 = assign7380_e8392_d_n4;
        locals.var_t3_dn5 = assign7380_e8392_d_n5;
        locals.var_t3_dn6 = assign7380_e8392_d_n6;
        locals.var_t3_dn7 = assign7380_e8392_d_n7;
        locals.var_t3_dn8 = assign7380_e8392_d_n8;
        locals.var_t3_dn9 = assign7380_e8392_d_n9;
        locals.var_t3_dn10 = assign7380_e8392_d_n10;
        locals.var_t3_dn11 = assign7380_e8392_d_n11;
        locals.var_t3_dn12 = assign7380_e8392_d_n12;

        let (assign7390_e8405, assign7390_e8405_d_n3, assign7390_e8405_d_n4, assign7390_e8405_d_n5, assign7390_e8405_d_n6, assign7390_e8405_d_n7, assign7390_e8405_d_n8, assign7390_e8405_d_n9, assign7390_e8405_d_n10, assign7390_e8405_d_n11, assign7390_e8405_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7390_e8398: f64 = (locals.var_t3 * locals.var_theta0);
        let assign7390_e8399: f64 = (locals.var_t2 + assign7390_e8398);
        let assign7390_e8401: f64 = (assign7390_e8399 + locals.var_pparam_b4soicit);
        let assign7390_e8403: f64 = (assign7390_e8401 / locals.var_b4soicox);
        (assign7390_e8403, (((locals.var_t2_dn3 + ((locals.var_t3_dn3 * locals.var_theta0) + (locals.var_t3 * locals.var_theta0_dn3))) + locals.var_pparam_b4soicit_dn3) / locals.var_b4soicox), (((locals.var_t2_dn4 + ((locals.var_t3_dn4 * locals.var_theta0) + (locals.var_t3 * locals.var_theta0_dn4))) + locals.var_pparam_b4soicit_dn4) / locals.var_b4soicox), (((locals.var_t2_dn5 + ((locals.var_t3_dn5 * locals.var_theta0) + (locals.var_t3 * locals.var_theta0_dn5))) + locals.var_pparam_b4soicit_dn5) / locals.var_b4soicox), (((locals.var_t2_dn6 + ((locals.var_t3_dn6 * locals.var_theta0) + (locals.var_t3 * locals.var_theta0_dn6))) + locals.var_pparam_b4soicit_dn6) / locals.var_b4soicox), (((locals.var_t2_dn7 + ((locals.var_t3_dn7 * locals.var_theta0) + (locals.var_t3 * locals.var_theta0_dn7))) + locals.var_pparam_b4soicit_dn7) / locals.var_b4soicox), (((locals.var_t2_dn8 + ((locals.var_t3_dn8 * locals.var_theta0) + (locals.var_t3 * locals.var_theta0_dn8))) + locals.var_pparam_b4soicit_dn8) / locals.var_b4soicox), (((locals.var_t2_dn9 + ((locals.var_t3_dn9 * locals.var_theta0) + (locals.var_t3 * locals.var_theta0_dn9))) + locals.var_pparam_b4soicit_dn9) / locals.var_b4soicox), (((locals.var_t2_dn10 + ((locals.var_t3_dn10 * locals.var_theta0) + (locals.var_t3 * locals.var_theta0_dn10))) + locals.var_pparam_b4soicit_dn10) / locals.var_b4soicox), (((locals.var_t2_dn11 + ((locals.var_t3_dn11 * locals.var_theta0) + (locals.var_t3 * locals.var_theta0_dn11))) + locals.var_pparam_b4soicit_dn11) / locals.var_b4soicox), (((locals.var_t2_dn12 + ((locals.var_t3_dn12 * locals.var_theta0) + (locals.var_t3 * locals.var_theta0_dn12))) + locals.var_pparam_b4soicit_dn12) / locals.var_b4soicox),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign7390_e8405;
        locals.var_t4_dn3 = assign7390_e8405_d_n3;
        locals.var_t4_dn4 = assign7390_e8405_d_n4;
        locals.var_t4_dn5 = assign7390_e8405_d_n5;
        locals.var_t4_dn6 = assign7390_e8405_d_n6;
        locals.var_t4_dn7 = assign7390_e8405_d_n7;
        locals.var_t4_dn8 = assign7390_e8405_d_n8;
        locals.var_t4_dn9 = assign7390_e8405_d_n9;
        locals.var_t4_dn10 = assign7390_e8405_d_n10;
        locals.var_t4_dn11 = assign7390_e8405_d_n11;
        locals.var_t4_dn12 = assign7390_e8405_d_n12;

        let assign7400_e8408: f64 = (-0.5);
        let assign7400_e8409: f64 = if locals.var_t4 >= assign7400_e8408 { 1.0 } else { 0.0 };
        locals.var_guard597 = assign7400_e8409;

        let (assign7410_e8418, assign7410_e8418_d_n3, assign7410_e8418_d_n4, assign7410_e8418_d_n5, assign7410_e8418_d_n6, assign7410_e8418_d_n7, assign7410_e8418_d_n8, assign7410_e8418_d_n9, assign7410_e8418_d_n10, assign7410_e8418_d_n11, assign7410_e8418_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard597 != 0.0)) {
        let assign7410_e8416: f64 = (1.0 + locals.var_t4);
        (assign7410_e8416, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    } else {
        (locals.var_n, locals.var_n_dn3, locals.var_n_dn4, locals.var_n_dn5, locals.var_n_dn6, locals.var_n_dn7, locals.var_n_dn8, locals.var_n_dn9, locals.var_n_dn10, locals.var_n_dn11, locals.var_n_dn12,)
    }
};
        locals.var_n = assign7410_e8418;
        locals.var_n_dn3 = assign7410_e8418_d_n3;
        locals.var_n_dn4 = assign7410_e8418_d_n4;
        locals.var_n_dn5 = assign7410_e8418_d_n5;
        locals.var_n_dn6 = assign7410_e8418_d_n6;
        locals.var_n_dn7 = assign7410_e8418_d_n7;
        locals.var_n_dn8 = assign7410_e8418_d_n8;
        locals.var_n_dn9 = assign7410_e8418_d_n9;
        locals.var_n_dn10 = assign7410_e8418_d_n10;
        locals.var_n_dn11 = assign7410_e8418_d_n11;
        locals.var_n_dn12 = assign7410_e8418_d_n12;

    }

    pub(super) fn stamp_transient_block_21(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign7420_e8432, assign7420_e8432_d_n3, assign7420_e8432_d_n4, assign7420_e8432_d_n5, assign7420_e8432_d_n6, assign7420_e8432_d_n7, assign7420_e8432_d_n8, assign7420_e8432_d_n9, assign7420_e8432_d_n10, assign7420_e8432_d_n11, assign7420_e8432_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard597 == 0.0)) {
        let assign7420_e8428: f64 = (8.0 * locals.var_t4);
        let assign7420_e8429: f64 = (3.0 + assign7420_e8428);
        let assign7420_e8430: f64 = (1.0 / assign7420_e8429);
        (assign7420_e8430, (-((8.0 * locals.var_t4_dn3) / (assign7420_e8429 * assign7420_e8429))), (-((8.0 * locals.var_t4_dn4) / (assign7420_e8429 * assign7420_e8429))), (-((8.0 * locals.var_t4_dn5) / (assign7420_e8429 * assign7420_e8429))), (-((8.0 * locals.var_t4_dn6) / (assign7420_e8429 * assign7420_e8429))), (-((8.0 * locals.var_t4_dn7) / (assign7420_e8429 * assign7420_e8429))), (-((8.0 * locals.var_t4_dn8) / (assign7420_e8429 * assign7420_e8429))), (-((8.0 * locals.var_t4_dn9) / (assign7420_e8429 * assign7420_e8429))), (-((8.0 * locals.var_t4_dn10) / (assign7420_e8429 * assign7420_e8429))), (-((8.0 * locals.var_t4_dn11) / (assign7420_e8429 * assign7420_e8429))), (-((8.0 * locals.var_t4_dn12) / (assign7420_e8429 * assign7420_e8429))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign7420_e8432;
        locals.var_t0_dn3 = assign7420_e8432_d_n3;
        locals.var_t0_dn4 = assign7420_e8432_d_n4;
        locals.var_t0_dn5 = assign7420_e8432_d_n5;
        locals.var_t0_dn6 = assign7420_e8432_d_n6;
        locals.var_t0_dn7 = assign7420_e8432_d_n7;
        locals.var_t0_dn8 = assign7420_e8432_d_n8;
        locals.var_t0_dn9 = assign7420_e8432_d_n9;
        locals.var_t0_dn10 = assign7420_e8432_d_n10;
        locals.var_t0_dn11 = assign7420_e8432_d_n11;
        locals.var_t0_dn12 = assign7420_e8432_d_n12;

        let (assign7430_e8446, assign7430_e8446_d_n3, assign7430_e8446_d_n4, assign7430_e8446_d_n5, assign7430_e8446_d_n6, assign7430_e8446_d_n7, assign7430_e8446_d_n8, assign7430_e8446_d_n9, assign7430_e8446_d_n10, assign7430_e8446_d_n11, assign7430_e8446_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard597 == 0.0)) {
        let assign7430_e8441: f64 = (3.0 * locals.var_t4);
        let assign7430_e8442: f64 = (1.0 + assign7430_e8441);
        let assign7430_e8444: f64 = (assign7430_e8442 * locals.var_t0);
        (assign7430_e8444, (((3.0 * locals.var_t4_dn3) * locals.var_t0) + (assign7430_e8442 * locals.var_t0_dn3)), (((3.0 * locals.var_t4_dn4) * locals.var_t0) + (assign7430_e8442 * locals.var_t0_dn4)), (((3.0 * locals.var_t4_dn5) * locals.var_t0) + (assign7430_e8442 * locals.var_t0_dn5)), (((3.0 * locals.var_t4_dn6) * locals.var_t0) + (assign7430_e8442 * locals.var_t0_dn6)), (((3.0 * locals.var_t4_dn7) * locals.var_t0) + (assign7430_e8442 * locals.var_t0_dn7)), (((3.0 * locals.var_t4_dn8) * locals.var_t0) + (assign7430_e8442 * locals.var_t0_dn8)), (((3.0 * locals.var_t4_dn9) * locals.var_t0) + (assign7430_e8442 * locals.var_t0_dn9)), (((3.0 * locals.var_t4_dn10) * locals.var_t0) + (assign7430_e8442 * locals.var_t0_dn10)), (((3.0 * locals.var_t4_dn11) * locals.var_t0) + (assign7430_e8442 * locals.var_t0_dn11)), (((3.0 * locals.var_t4_dn12) * locals.var_t0) + (assign7430_e8442 * locals.var_t0_dn12)),)
    } else {
        (locals.var_n, locals.var_n_dn3, locals.var_n_dn4, locals.var_n_dn5, locals.var_n_dn6, locals.var_n_dn7, locals.var_n_dn8, locals.var_n_dn9, locals.var_n_dn10, locals.var_n_dn11, locals.var_n_dn12,)
    }
};
        locals.var_n = assign7430_e8446;
        locals.var_n_dn3 = assign7430_e8446_d_n3;
        locals.var_n_dn4 = assign7430_e8446_d_n4;
        locals.var_n_dn5 = assign7430_e8446_d_n5;
        locals.var_n_dn6 = assign7430_e8446_d_n6;
        locals.var_n_dn7 = assign7430_e8446_d_n7;
        locals.var_n_dn8 = assign7430_e8446_d_n8;
        locals.var_n_dn9 = assign7430_e8446_d_n9;
        locals.var_n_dn10 = assign7430_e8446_d_n10;
        locals.var_n_dn11 = assign7430_e8446_d_n11;
        locals.var_n_dn12 = assign7430_e8446_d_n12;

        let assign7440_e8449: f64 = if locals.var_pparam_b4soidvtp0 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard598 = assign7440_e8449;

        let (assign7450_e8460, assign7450_e8460_d_n3, assign7450_e8460_d_n4, assign7450_e8460_d_n5, assign7450_e8460_d_n6, assign7450_e8460_d_n7, assign7450_e8460_d_n8, assign7450_e8460_d_n9, assign7450_e8460_d_n10, assign7450_e8460_d_n11, assign7450_e8460_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard598 != 0.0)) {
        let assign7450_e8457: f64 = (2.0 * locals.var_pparam_b4soidvtp0);
        let assign7450_e8458: f64 = (p.p54 + assign7450_e8457);
        (assign7450_e8458, (2.0 * locals.var_pparam_b4soidvtp0_dn3), (2.0 * locals.var_pparam_b4soidvtp0_dn4), (2.0 * locals.var_pparam_b4soidvtp0_dn5), (2.0 * locals.var_pparam_b4soidvtp0_dn6), (2.0 * locals.var_pparam_b4soidvtp0_dn7), (2.0 * locals.var_pparam_b4soidvtp0_dn8), (2.0 * locals.var_pparam_b4soidvtp0_dn9), (2.0 * locals.var_pparam_b4soidvtp0_dn10), (2.0 * locals.var_pparam_b4soidvtp0_dn11), (2.0 * locals.var_pparam_b4soidvtp0_dn12),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign7450_e8460;
        locals.var_t3_dn3 = assign7450_e8460_d_n3;
        locals.var_t3_dn4 = assign7450_e8460_d_n4;
        locals.var_t3_dn5 = assign7450_e8460_d_n5;
        locals.var_t3_dn6 = assign7450_e8460_d_n6;
        locals.var_t3_dn7 = assign7450_e8460_d_n7;
        locals.var_t3_dn8 = assign7450_e8460_d_n8;
        locals.var_t3_dn9 = assign7450_e8460_d_n9;
        locals.var_t3_dn10 = assign7450_e8460_d_n10;
        locals.var_t3_dn11 = assign7450_e8460_d_n11;
        locals.var_t3_dn12 = assign7450_e8460_d_n12;

        let (assign7460_e8480, assign7460_e8480_d_n3, assign7460_e8480_d_n4, assign7460_e8480_d_n5, assign7460_e8480_d_n6, assign7460_e8480_d_n7, assign7460_e8480_d_n8, assign7460_e8480_d_n9, assign7460_e8480_d_n10, assign7460_e8480_d_n11, assign7460_e8480_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard598 != 0.0)) {
        let assign7460_e8468: f64 = (p.p54 / locals.var_t3);
        let (assign7460_e8477, assign7460_e8477_d_n3, assign7460_e8477_d_n4, assign7460_e8477_d_n5, assign7460_e8477_d_n6, assign7460_e8477_d_n7, assign7460_e8477_d_n8, assign7460_e8477_d_n9, assign7460_e8477_d_n10, assign7460_e8477_d_n11, assign7460_e8477_d_n12,) = {
            if (assign7460_e8468 > 1e-38) {
                let assign7460_e8473: f64 = (p.p54 / locals.var_t3);
                let assign7460_e8474: f64 = (assign7460_e8473).ln();
                (assign7460_e8474, ((-((p.p54 * locals.var_t3_dn3) / (locals.var_t3 * locals.var_t3))) / assign7460_e8473), ((-((p.p54 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))) / assign7460_e8473), ((-((p.p54 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))) / assign7460_e8473), ((-((p.p54 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))) / assign7460_e8473), ((-((p.p54 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))) / assign7460_e8473), ((-((p.p54 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))) / assign7460_e8473), ((-((p.p54 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))) / assign7460_e8473), ((-((p.p54 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))) / assign7460_e8473), ((-((p.p54 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))) / assign7460_e8473), ((-((p.p54 * locals.var_t3_dn12) / (locals.var_t3 * locals.var_t3))) / assign7460_e8473),)
            } else {
                let assign7460_e8476: f64 = (-87.49823353377374);
                (assign7460_e8476, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign7460_e8478: f64 = (locals.var_vtmeot * assign7460_e8477);
        (assign7460_e8478, (locals.var_vtmeot * assign7460_e8477_d_n3), (locals.var_vtmeot * assign7460_e8477_d_n4), (locals.var_vtmeot * assign7460_e8477_d_n5), (locals.var_vtmeot * assign7460_e8477_d_n6), (locals.var_vtmeot * assign7460_e8477_d_n7), (locals.var_vtmeot * assign7460_e8477_d_n8), (locals.var_vtmeot * assign7460_e8477_d_n9), (locals.var_vtmeot * assign7460_e8477_d_n10), (locals.var_vtmeot * assign7460_e8477_d_n11), (locals.var_vtmeot * assign7460_e8477_d_n12),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign7460_e8480;
        locals.var_t4_dn3 = assign7460_e8480_d_n3;
        locals.var_t4_dn4 = assign7460_e8480_d_n4;
        locals.var_t4_dn5 = assign7460_e8480_d_n5;
        locals.var_t4_dn6 = assign7460_e8480_d_n6;
        locals.var_t4_dn7 = assign7460_e8480_d_n7;
        locals.var_t4_dn8 = assign7460_e8480_d_n8;
        locals.var_t4_dn9 = assign7460_e8480_d_n9;
        locals.var_t4_dn10 = assign7460_e8480_d_n10;
        locals.var_t4_dn11 = assign7460_e8480_d_n11;
        locals.var_t4_dn12 = assign7460_e8480_d_n12;

        let (assign7470_e8489, assign7470_e8489_d_n3, assign7470_e8489_d_n4, assign7470_e8489_d_n5, assign7470_e8489_d_n6, assign7470_e8489_d_n7, assign7470_e8489_d_n8, assign7470_e8489_d_n9, assign7470_e8489_d_n10, assign7470_e8489_d_n11, assign7470_e8489_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard598 != 0.0)) {
        let assign7470_e8487: f64 = (locals.var_n * locals.var_t4);
        (assign7470_e8487, ((locals.var_n_dn3 * locals.var_t4) + (locals.var_n * locals.var_t4_dn3)), ((locals.var_n_dn4 * locals.var_t4) + (locals.var_n * locals.var_t4_dn4)), ((locals.var_n_dn5 * locals.var_t4) + (locals.var_n * locals.var_t4_dn5)), ((locals.var_n_dn6 * locals.var_t4) + (locals.var_n * locals.var_t4_dn6)), ((locals.var_n_dn7 * locals.var_t4) + (locals.var_n * locals.var_t4_dn7)), ((locals.var_n_dn8 * locals.var_t4) + (locals.var_n * locals.var_t4_dn8)), ((locals.var_n_dn9 * locals.var_t4) + (locals.var_n * locals.var_t4_dn9)), ((locals.var_n_dn10 * locals.var_t4) + (locals.var_n * locals.var_t4_dn10)), ((locals.var_n_dn11 * locals.var_t4) + (locals.var_n * locals.var_t4_dn11)), ((locals.var_n_dn12 * locals.var_t4) + (locals.var_n * locals.var_t4_dn12)),)
    } else {
        (locals.var_dits_sft, locals.var_dits_sft_dn3, locals.var_dits_sft_dn4, locals.var_dits_sft_dn5, locals.var_dits_sft_dn6, locals.var_dits_sft_dn7, locals.var_dits_sft_dn8, locals.var_dits_sft_dn9, locals.var_dits_sft_dn10, locals.var_dits_sft_dn11, locals.var_dits_sft_dn12,)
    }
};
        locals.var_dits_sft = assign7470_e8489;
        locals.var_dits_sft_dn3 = assign7470_e8489_d_n3;
        locals.var_dits_sft_dn4 = assign7470_e8489_d_n4;
        locals.var_dits_sft_dn5 = assign7470_e8489_d_n5;
        locals.var_dits_sft_dn6 = assign7470_e8489_d_n6;
        locals.var_dits_sft_dn7 = assign7470_e8489_d_n7;
        locals.var_dits_sft_dn8 = assign7470_e8489_d_n8;
        locals.var_dits_sft_dn9 = assign7470_e8489_d_n9;
        locals.var_dits_sft_dn10 = assign7470_e8489_d_n10;
        locals.var_dits_sft_dn11 = assign7470_e8489_d_n11;
        locals.var_dits_sft_dn12 = assign7470_e8489_d_n12;

        let (assign7480_e8497, assign7480_e8497_d_n3, assign7480_e8497_d_n4, assign7480_e8497_d_n5, assign7480_e8497_d_n6, assign7480_e8497_d_n7, assign7480_e8497_d_n8, assign7480_e8497_d_n9, assign7480_e8497_d_n10, assign7480_e8497_d_n11, assign7480_e8497_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard598 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dits_sft, locals.var_dits_sft_dn3, locals.var_dits_sft_dn4, locals.var_dits_sft_dn5, locals.var_dits_sft_dn6, locals.var_dits_sft_dn7, locals.var_dits_sft_dn8, locals.var_dits_sft_dn9, locals.var_dits_sft_dn10, locals.var_dits_sft_dn11, locals.var_dits_sft_dn12,)
    }
};
        locals.var_dits_sft = assign7480_e8497;
        locals.var_dits_sft_dn3 = assign7480_e8497_d_n3;
        locals.var_dits_sft_dn4 = assign7480_e8497_d_n4;
        locals.var_dits_sft_dn5 = assign7480_e8497_d_n5;
        locals.var_dits_sft_dn6 = assign7480_e8497_d_n6;
        locals.var_dits_sft_dn7 = assign7480_e8497_d_n7;
        locals.var_dits_sft_dn8 = assign7480_e8497_d_n8;
        locals.var_dits_sft_dn9 = assign7480_e8497_d_n9;
        locals.var_dits_sft_dn10 = assign7480_e8497_d_n10;
        locals.var_dits_sft_dn11 = assign7480_e8497_d_n11;
        locals.var_dits_sft_dn12 = assign7480_e8497_d_n12;

        let (assign7490_e8504, assign7490_e8504_d_n3, assign7490_e8504_d_n4, assign7490_e8504_d_n5, assign7490_e8504_d_n6, assign7490_e8504_d_n7, assign7490_e8504_d_n8, assign7490_e8504_d_n9, assign7490_e8504_d_n10, assign7490_e8504_d_n11, assign7490_e8504_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7490_e8502: f64 = (locals.var_pparam_b4soidvt0 * locals.var_theta0);
        (assign7490_e8502, ((locals.var_pparam_b4soidvt0_dn3 * locals.var_theta0) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_dn3)), ((locals.var_pparam_b4soidvt0_dn4 * locals.var_theta0) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_dn4)), ((locals.var_pparam_b4soidvt0_dn5 * locals.var_theta0) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_dn5)), ((locals.var_pparam_b4soidvt0_dn6 * locals.var_theta0) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_dn6)), ((locals.var_pparam_b4soidvt0_dn7 * locals.var_theta0) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_dn7)), ((locals.var_pparam_b4soidvt0_dn8 * locals.var_theta0) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_dn8)), ((locals.var_pparam_b4soidvt0_dn9 * locals.var_theta0) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_dn9)), ((locals.var_pparam_b4soidvt0_dn10 * locals.var_theta0) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_dn10)), ((locals.var_pparam_b4soidvt0_dn11 * locals.var_theta0) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_dn11)), ((locals.var_pparam_b4soidvt0_dn12 * locals.var_theta0) + (locals.var_pparam_b4soidvt0 * locals.var_theta0_dn12)),)
    } else {
        (locals.var_b4soithetavth, locals.var_b4soithetavth_dn3, locals.var_b4soithetavth_dn4, locals.var_b4soithetavth_dn5, locals.var_b4soithetavth_dn6, locals.var_b4soithetavth_dn7, locals.var_b4soithetavth_dn8, locals.var_b4soithetavth_dn9, locals.var_b4soithetavth_dn10, locals.var_b4soithetavth_dn11, locals.var_b4soithetavth_dn12,)
    }
};
        locals.var_b4soithetavth = assign7490_e8504;
        locals.var_b4soithetavth_dn3 = assign7490_e8504_d_n3;
        locals.var_b4soithetavth_dn4 = assign7490_e8504_d_n4;
        locals.var_b4soithetavth_dn5 = assign7490_e8504_d_n5;
        locals.var_b4soithetavth_dn6 = assign7490_e8504_d_n6;
        locals.var_b4soithetavth_dn7 = assign7490_e8504_d_n7;
        locals.var_b4soithetavth_dn8 = assign7490_e8504_d_n8;
        locals.var_b4soithetavth_dn9 = assign7490_e8504_d_n9;
        locals.var_b4soithetavth_dn10 = assign7490_e8504_d_n10;
        locals.var_b4soithetavth_dn11 = assign7490_e8504_d_n11;
        locals.var_b4soithetavth_dn12 = assign7490_e8504_d_n12;

        let (assign7500_e8511, assign7500_e8511_d_n3, assign7500_e8511_d_n4, assign7500_e8511_d_n5, assign7500_e8511_d_n6, assign7500_e8511_d_n7, assign7500_e8511_d_n8, assign7500_e8511_d_n9, assign7500_e8511_d_n10, assign7500_e8511_d_n11, assign7500_e8511_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7500_e8509: f64 = (locals.var_b4soithetavth * locals.var_v0);
        (assign7500_e8509, ((locals.var_b4soithetavth_dn3 * locals.var_v0) + (locals.var_b4soithetavth * locals.var_v0_dn3)), ((locals.var_b4soithetavth_dn4 * locals.var_v0) + (locals.var_b4soithetavth * locals.var_v0_dn4)), ((locals.var_b4soithetavth_dn5 * locals.var_v0) + (locals.var_b4soithetavth * locals.var_v0_dn5)), ((locals.var_b4soithetavth_dn6 * locals.var_v0) + (locals.var_b4soithetavth * locals.var_v0_dn6)), ((locals.var_b4soithetavth_dn7 * locals.var_v0) + (locals.var_b4soithetavth * locals.var_v0_dn7)), ((locals.var_b4soithetavth_dn8 * locals.var_v0) + (locals.var_b4soithetavth * locals.var_v0_dn8)), ((locals.var_b4soithetavth_dn9 * locals.var_v0) + (locals.var_b4soithetavth * locals.var_v0_dn9)), ((locals.var_b4soithetavth_dn10 * locals.var_v0) + (locals.var_b4soithetavth * locals.var_v0_dn10)), ((locals.var_b4soithetavth_dn11 * locals.var_v0) + (locals.var_b4soithetavth * locals.var_v0_dn11)), ((locals.var_b4soithetavth_dn12 * locals.var_v0) + (locals.var_b4soithetavth * locals.var_v0_dn12)),)
    } else {
        (locals.var_delt_vth, locals.var_delt_vth_dn3, locals.var_delt_vth_dn4, locals.var_delt_vth_dn5, locals.var_delt_vth_dn6, locals.var_delt_vth_dn7, locals.var_delt_vth_dn8, locals.var_delt_vth_dn9, locals.var_delt_vth_dn10, locals.var_delt_vth_dn11, locals.var_delt_vth_dn12,)
    }
};
        locals.var_delt_vth = assign7500_e8511;
        locals.var_delt_vth_dn3 = assign7500_e8511_d_n3;
        locals.var_delt_vth_dn4 = assign7500_e8511_d_n4;
        locals.var_delt_vth_dn5 = assign7500_e8511_d_n5;
        locals.var_delt_vth_dn6 = assign7500_e8511_d_n6;
        locals.var_delt_vth_dn7 = assign7500_e8511_d_n7;
        locals.var_delt_vth_dn8 = assign7500_e8511_d_n8;
        locals.var_delt_vth_dn9 = assign7500_e8511_d_n9;
        locals.var_delt_vth_dn10 = assign7500_e8511_d_n10;
        locals.var_delt_vth_dn11 = assign7500_e8511_d_n11;
        locals.var_delt_vth_dn12 = assign7500_e8511_d_n12;

        let (assign7510_e8525, assign7510_e8525_d_n3, assign7510_e8525_d_n4, assign7510_e8525_d_n5, assign7510_e8525_d_n6, assign7510_e8525_d_n7, assign7510_e8525_d_n8, assign7510_e8525_d_n9, assign7510_e8525_d_n10, assign7510_e8525_d_n11, assign7510_e8525_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7510_e8515: f64 = (-0.5);
        let assign7510_e8517: f64 = (assign7510_e8515 * locals.var_pparam_b4soidvt1w);
        let assign7510_e8519: f64 = (assign7510_e8517 * p.p55);
        let assign7510_e8521: f64 = (assign7510_e8519 * p.p54);
        let assign7510_e8523: f64 = (assign7510_e8521 / locals.var_ltw);
        (assign7510_e8523, ((((((assign7510_e8515 * locals.var_pparam_b4soidvt1w_dn3) * p.p55) * p.p54) * locals.var_ltw) - (assign7510_e8521 * locals.var_ltw_dn3)) / (locals.var_ltw * locals.var_ltw)), ((((((assign7510_e8515 * locals.var_pparam_b4soidvt1w_dn4) * p.p55) * p.p54) * locals.var_ltw) - (assign7510_e8521 * locals.var_ltw_dn4)) / (locals.var_ltw * locals.var_ltw)), ((((((assign7510_e8515 * locals.var_pparam_b4soidvt1w_dn5) * p.p55) * p.p54) * locals.var_ltw) - (assign7510_e8521 * locals.var_ltw_dn5)) / (locals.var_ltw * locals.var_ltw)), ((((((assign7510_e8515 * locals.var_pparam_b4soidvt1w_dn6) * p.p55) * p.p54) * locals.var_ltw) - (assign7510_e8521 * locals.var_ltw_dn6)) / (locals.var_ltw * locals.var_ltw)), ((((((assign7510_e8515 * locals.var_pparam_b4soidvt1w_dn7) * p.p55) * p.p54) * locals.var_ltw) - (assign7510_e8521 * locals.var_ltw_dn7)) / (locals.var_ltw * locals.var_ltw)), ((((((assign7510_e8515 * locals.var_pparam_b4soidvt1w_dn8) * p.p55) * p.p54) * locals.var_ltw) - (assign7510_e8521 * locals.var_ltw_dn8)) / (locals.var_ltw * locals.var_ltw)), ((((((assign7510_e8515 * locals.var_pparam_b4soidvt1w_dn9) * p.p55) * p.p54) * locals.var_ltw) - (assign7510_e8521 * locals.var_ltw_dn9)) / (locals.var_ltw * locals.var_ltw)), ((((((assign7510_e8515 * locals.var_pparam_b4soidvt1w_dn10) * p.p55) * p.p54) * locals.var_ltw) - (assign7510_e8521 * locals.var_ltw_dn10)) / (locals.var_ltw * locals.var_ltw)), ((((((assign7510_e8515 * locals.var_pparam_b4soidvt1w_dn11) * p.p55) * p.p54) * locals.var_ltw) - (assign7510_e8521 * locals.var_ltw_dn11)) / (locals.var_ltw * locals.var_ltw)), ((((((assign7510_e8515 * locals.var_pparam_b4soidvt1w_dn12) * p.p55) * p.p54) * locals.var_ltw) - (assign7510_e8521 * locals.var_ltw_dn12)) / (locals.var_ltw * locals.var_ltw)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign7510_e8525;
        locals.var_t0_dn3 = assign7510_e8525_d_n3;
        locals.var_t0_dn4 = assign7510_e8525_d_n4;
        locals.var_t0_dn5 = assign7510_e8525_d_n5;
        locals.var_t0_dn6 = assign7510_e8525_d_n6;
        locals.var_t0_dn7 = assign7510_e8525_d_n7;
        locals.var_t0_dn8 = assign7510_e8525_d_n8;
        locals.var_t0_dn9 = assign7510_e8525_d_n9;
        locals.var_t0_dn10 = assign7510_e8525_d_n10;
        locals.var_t0_dn11 = assign7510_e8525_d_n11;
        locals.var_t0_dn12 = assign7510_e8525_d_n12;

        let assign7520_e8528: f64 = (-100.0);
        let assign7520_e8529: f64 = if locals.var_t0 > assign7520_e8528 { 1.0 } else { 0.0 };
        locals.var_guard599 = assign7520_e8529;

        let (assign7530_e8537, assign7530_e8537_d_n3, assign7530_e8537_d_n4, assign7530_e8537_d_n5, assign7530_e8537_d_n6, assign7530_e8537_d_n7, assign7530_e8537_d_n8, assign7530_e8537_d_n9, assign7530_e8537_d_n10, assign7530_e8537_d_n11, assign7530_e8537_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard599 != 0.0)) {
        let assign7530_e8535: f64 = (locals.var_t0).exp();
        (assign7530_e8535, (assign7530_e8535 * locals.var_t0_dn3), (assign7530_e8535 * locals.var_t0_dn4), (assign7530_e8535 * locals.var_t0_dn5), (assign7530_e8535 * locals.var_t0_dn6), (assign7530_e8535 * locals.var_t0_dn7), (assign7530_e8535 * locals.var_t0_dn8), (assign7530_e8535 * locals.var_t0_dn9), (assign7530_e8535 * locals.var_t0_dn10), (assign7530_e8535 * locals.var_t0_dn11), (assign7530_e8535 * locals.var_t0_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign7530_e8537;
        locals.var_t1_dn3 = assign7530_e8537_d_n3;
        locals.var_t1_dn4 = assign7530_e8537_d_n4;
        locals.var_t1_dn5 = assign7530_e8537_d_n5;
        locals.var_t1_dn6 = assign7530_e8537_d_n6;
        locals.var_t1_dn7 = assign7530_e8537_d_n7;
        locals.var_t1_dn8 = assign7530_e8537_d_n8;
        locals.var_t1_dn9 = assign7530_e8537_d_n9;
        locals.var_t1_dn10 = assign7530_e8537_d_n10;
        locals.var_t1_dn11 = assign7530_e8537_d_n11;
        locals.var_t1_dn12 = assign7530_e8537_d_n12;

        let (assign7540_e8550, assign7540_e8550_d_n3, assign7540_e8550_d_n4, assign7540_e8550_d_n5, assign7540_e8550_d_n6, assign7540_e8550_d_n7, assign7540_e8550_d_n8, assign7540_e8550_d_n9, assign7540_e8550_d_n10, assign7540_e8550_d_n11, assign7540_e8550_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard599 != 0.0)) {
        let assign7540_e8546: f64 = (2.0 * locals.var_t1);
        let assign7540_e8547: f64 = (1.0 + assign7540_e8546);
        let assign7540_e8548: f64 = (locals.var_t1 * assign7540_e8547);
        (assign7540_e8548, ((locals.var_t1_dn3 * assign7540_e8547) + (locals.var_t1 * (2.0 * locals.var_t1_dn3))), ((locals.var_t1_dn4 * assign7540_e8547) + (locals.var_t1 * (2.0 * locals.var_t1_dn4))), ((locals.var_t1_dn5 * assign7540_e8547) + (locals.var_t1 * (2.0 * locals.var_t1_dn5))), ((locals.var_t1_dn6 * assign7540_e8547) + (locals.var_t1 * (2.0 * locals.var_t1_dn6))), ((locals.var_t1_dn7 * assign7540_e8547) + (locals.var_t1 * (2.0 * locals.var_t1_dn7))), ((locals.var_t1_dn8 * assign7540_e8547) + (locals.var_t1 * (2.0 * locals.var_t1_dn8))), ((locals.var_t1_dn9 * assign7540_e8547) + (locals.var_t1 * (2.0 * locals.var_t1_dn9))), ((locals.var_t1_dn10 * assign7540_e8547) + (locals.var_t1 * (2.0 * locals.var_t1_dn10))), ((locals.var_t1_dn11 * assign7540_e8547) + (locals.var_t1 * (2.0 * locals.var_t1_dn11))), ((locals.var_t1_dn12 * assign7540_e8547) + (locals.var_t1 * (2.0 * locals.var_t1_dn12))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign7540_e8550;
        locals.var_t2_dn3 = assign7540_e8550_d_n3;
        locals.var_t2_dn4 = assign7540_e8550_d_n4;
        locals.var_t2_dn5 = assign7540_e8550_d_n5;
        locals.var_t2_dn6 = assign7540_e8550_d_n6;
        locals.var_t2_dn7 = assign7540_e8550_d_n7;
        locals.var_t2_dn8 = assign7540_e8550_d_n8;
        locals.var_t2_dn9 = assign7540_e8550_d_n9;
        locals.var_t2_dn10 = assign7540_e8550_d_n10;
        locals.var_t2_dn11 = assign7540_e8550_d_n11;
        locals.var_t2_dn12 = assign7540_e8550_d_n12;

        let (assign7550_e8558, assign7550_e8558_d_n3, assign7550_e8558_d_n4, assign7550_e8558_d_n5, assign7550_e8558_d_n6, assign7550_e8558_d_n7, assign7550_e8558_d_n8, assign7550_e8558_d_n9, assign7550_e8558_d_n10, assign7550_e8558_d_n11, assign7550_e8558_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard599 == 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign7550_e8558;
        locals.var_t1_dn3 = assign7550_e8558_d_n3;
        locals.var_t1_dn4 = assign7550_e8558_d_n4;
        locals.var_t1_dn5 = assign7550_e8558_d_n5;
        locals.var_t1_dn6 = assign7550_e8558_d_n6;
        locals.var_t1_dn7 = assign7550_e8558_d_n7;
        locals.var_t1_dn8 = assign7550_e8558_d_n8;
        locals.var_t1_dn9 = assign7550_e8558_d_n9;
        locals.var_t1_dn10 = assign7550_e8558_d_n10;
        locals.var_t1_dn11 = assign7550_e8558_d_n11;
        locals.var_t1_dn12 = assign7550_e8558_d_n12;

        let (assign7560_e8572, assign7560_e8572_d_n3, assign7560_e8572_d_n4, assign7560_e8572_d_n5, assign7560_e8572_d_n6, assign7560_e8572_d_n7, assign7560_e8572_d_n8, assign7560_e8572_d_n9, assign7560_e8572_d_n10, assign7560_e8572_d_n11, assign7560_e8572_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard599 == 0.0)) {
        let assign7560_e8568: f64 = (2.0 * locals.var_t1);
        let assign7560_e8569: f64 = (1.0 + assign7560_e8568);
        let assign7560_e8570: f64 = (locals.var_t1 * assign7560_e8569);
        (assign7560_e8570, ((locals.var_t1_dn3 * assign7560_e8569) + (locals.var_t1 * (2.0 * locals.var_t1_dn3))), ((locals.var_t1_dn4 * assign7560_e8569) + (locals.var_t1 * (2.0 * locals.var_t1_dn4))), ((locals.var_t1_dn5 * assign7560_e8569) + (locals.var_t1 * (2.0 * locals.var_t1_dn5))), ((locals.var_t1_dn6 * assign7560_e8569) + (locals.var_t1 * (2.0 * locals.var_t1_dn6))), ((locals.var_t1_dn7 * assign7560_e8569) + (locals.var_t1 * (2.0 * locals.var_t1_dn7))), ((locals.var_t1_dn8 * assign7560_e8569) + (locals.var_t1 * (2.0 * locals.var_t1_dn8))), ((locals.var_t1_dn9 * assign7560_e8569) + (locals.var_t1 * (2.0 * locals.var_t1_dn9))), ((locals.var_t1_dn10 * assign7560_e8569) + (locals.var_t1 * (2.0 * locals.var_t1_dn10))), ((locals.var_t1_dn11 * assign7560_e8569) + (locals.var_t1 * (2.0 * locals.var_t1_dn11))), ((locals.var_t1_dn12 * assign7560_e8569) + (locals.var_t1 * (2.0 * locals.var_t1_dn12))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign7560_e8572;
        locals.var_t2_dn3 = assign7560_e8572_d_n3;
        locals.var_t2_dn4 = assign7560_e8572_d_n4;
        locals.var_t2_dn5 = assign7560_e8572_d_n5;
        locals.var_t2_dn6 = assign7560_e8572_d_n6;
        locals.var_t2_dn7 = assign7560_e8572_d_n7;
        locals.var_t2_dn8 = assign7560_e8572_d_n8;
        locals.var_t2_dn9 = assign7560_e8572_d_n9;
        locals.var_t2_dn10 = assign7560_e8572_d_n10;
        locals.var_t2_dn11 = assign7560_e8572_d_n11;
        locals.var_t2_dn12 = assign7560_e8572_d_n12;

        let (assign7570_e8579, assign7570_e8579_d_n3, assign7570_e8579_d_n4, assign7570_e8579_d_n5, assign7570_e8579_d_n6, assign7570_e8579_d_n7, assign7570_e8579_d_n8, assign7570_e8579_d_n9, assign7570_e8579_d_n10, assign7570_e8579_d_n11, assign7570_e8579_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7570_e8577: f64 = (locals.var_pparam_b4soidvt0w * locals.var_t2);
        (assign7570_e8577, ((locals.var_pparam_b4soidvt0w_dn3 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn3)), ((locals.var_pparam_b4soidvt0w_dn4 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn4)), ((locals.var_pparam_b4soidvt0w_dn5 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn5)), ((locals.var_pparam_b4soidvt0w_dn6 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn6)), ((locals.var_pparam_b4soidvt0w_dn7 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn7)), ((locals.var_pparam_b4soidvt0w_dn8 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn8)), ((locals.var_pparam_b4soidvt0w_dn9 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn9)), ((locals.var_pparam_b4soidvt0w_dn10 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn10)), ((locals.var_pparam_b4soidvt0w_dn11 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn11)), ((locals.var_pparam_b4soidvt0w_dn12 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn12)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign7570_e8579;
        locals.var_t0_dn3 = assign7570_e8579_d_n3;
        locals.var_t0_dn4 = assign7570_e8579_d_n4;
        locals.var_t0_dn5 = assign7570_e8579_d_n5;
        locals.var_t0_dn6 = assign7570_e8579_d_n6;
        locals.var_t0_dn7 = assign7570_e8579_d_n7;
        locals.var_t0_dn8 = assign7570_e8579_d_n8;
        locals.var_t0_dn9 = assign7570_e8579_d_n9;
        locals.var_t0_dn10 = assign7570_e8579_d_n10;
        locals.var_t0_dn11 = assign7570_e8579_d_n11;
        locals.var_t0_dn12 = assign7570_e8579_d_n12;

        let (assign7580_e8586, assign7580_e8586_d_n3, assign7580_e8586_d_n4, assign7580_e8586_d_n5, assign7580_e8586_d_n6, assign7580_e8586_d_n7, assign7580_e8586_d_n8, assign7580_e8586_d_n9, assign7580_e8586_d_n10, assign7580_e8586_d_n11, assign7580_e8586_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7580_e8584: f64 = (locals.var_t0 * locals.var_v0);
        (assign7580_e8584, ((locals.var_t0_dn3 * locals.var_v0) + (locals.var_t0 * locals.var_v0_dn3)), ((locals.var_t0_dn4 * locals.var_v0) + (locals.var_t0 * locals.var_v0_dn4)), ((locals.var_t0_dn5 * locals.var_v0) + (locals.var_t0 * locals.var_v0_dn5)), ((locals.var_t0_dn6 * locals.var_v0) + (locals.var_t0 * locals.var_v0_dn6)), ((locals.var_t0_dn7 * locals.var_v0) + (locals.var_t0 * locals.var_v0_dn7)), ((locals.var_t0_dn8 * locals.var_v0) + (locals.var_t0 * locals.var_v0_dn8)), ((locals.var_t0_dn9 * locals.var_v0) + (locals.var_t0 * locals.var_v0_dn9)), ((locals.var_t0_dn10 * locals.var_v0) + (locals.var_t0 * locals.var_v0_dn10)), ((locals.var_t0_dn11 * locals.var_v0) + (locals.var_t0 * locals.var_v0_dn11)), ((locals.var_t0_dn12 * locals.var_v0) + (locals.var_t0 * locals.var_v0_dn12)),)
    } else {
        (locals.var_deltvthw, locals.var_deltvthw_dn3, locals.var_deltvthw_dn4, locals.var_deltvthw_dn5, locals.var_deltvthw_dn6, locals.var_deltvthw_dn7, locals.var_deltvthw_dn8, locals.var_deltvthw_dn9, locals.var_deltvthw_dn10, locals.var_deltvthw_dn11, locals.var_deltvthw_dn12,)
    }
};
        locals.var_deltvthw = assign7580_e8586;
        locals.var_deltvthw_dn3 = assign7580_e8586_d_n3;
        locals.var_deltvthw_dn4 = assign7580_e8586_d_n4;
        locals.var_deltvthw_dn5 = assign7580_e8586_d_n5;
        locals.var_deltvthw_dn6 = assign7580_e8586_d_n6;
        locals.var_deltvthw_dn7 = assign7580_e8586_d_n7;
        locals.var_deltvthw_dn8 = assign7580_e8586_d_n8;
        locals.var_deltvthw_dn9 = assign7580_e8586_d_n9;
        locals.var_deltvthw_dn10 = assign7580_e8586_d_n10;
        locals.var_deltvthw_dn11 = assign7580_e8586_d_n11;
        locals.var_deltvthw_dn12 = assign7580_e8586_d_n12;

        let (assign7590_e8595, assign7590_e8595_d_n4, assign7590_e8595_d_n5, assign7590_e8595_d_n6,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7590_e8591: f64 = (p.p57 / locals.var_tnom);
        let assign7590_e8593: f64 = (assign7590_e8591 - 1.0);
        (assign7590_e8593, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_trm1, locals.var_trm1_dn4, locals.var_trm1_dn5, locals.var_trm1_dn6,)
    }
};
        locals.var_trm1 = assign7590_e8595;
        locals.var_trm1_dn4 = assign7590_e8595_d_n4;
        locals.var_trm1_dn5 = assign7590_e8595_d_n5;
        locals.var_trm1_dn6 = assign7590_e8595_d_n6;

        let (assign7600_e8605, assign7600_e8605_d_n3, assign7600_e8605_d_n4, assign7600_e8605_d_n5, assign7600_e8605_d_n6, assign7600_e8605_d_n7, assign7600_e8605_d_n8, assign7600_e8605_d_n9, assign7600_e8605_d_n10, assign7600_e8605_d_n11, assign7600_e8605_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7600_e8601: f64 = (locals.var_pparam_b4soilpe0 / p.p54);
        let assign7600_e8602: f64 = (1.0 + assign7600_e8601);
        let assign7600_e8603: f64 = (assign7600_e8602).sqrt();
        (assign7600_e8603, ((locals.var_pparam_b4soilpe0_dn3 / p.p54) / (2.0 * assign7600_e8603)), ((locals.var_pparam_b4soilpe0_dn4 / p.p54) / (2.0 * assign7600_e8603)), ((locals.var_pparam_b4soilpe0_dn5 / p.p54) / (2.0 * assign7600_e8603)), ((locals.var_pparam_b4soilpe0_dn6 / p.p54) / (2.0 * assign7600_e8603)), ((locals.var_pparam_b4soilpe0_dn7 / p.p54) / (2.0 * assign7600_e8603)), ((locals.var_pparam_b4soilpe0_dn8 / p.p54) / (2.0 * assign7600_e8603)), ((locals.var_pparam_b4soilpe0_dn9 / p.p54) / (2.0 * assign7600_e8603)), ((locals.var_pparam_b4soilpe0_dn10 / p.p54) / (2.0 * assign7600_e8603)), ((locals.var_pparam_b4soilpe0_dn11 / p.p54) / (2.0 * assign7600_e8603)), ((locals.var_pparam_b4soilpe0_dn12 / p.p54) / (2.0 * assign7600_e8603)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign7600_e8605;
        locals.var_t0_dn3 = assign7600_e8605_d_n3;
        locals.var_t0_dn4 = assign7600_e8605_d_n4;
        locals.var_t0_dn5 = assign7600_e8605_d_n5;
        locals.var_t0_dn6 = assign7600_e8605_d_n6;
        locals.var_t0_dn7 = assign7600_e8605_d_n7;
        locals.var_t0_dn8 = assign7600_e8605_d_n8;
        locals.var_t0_dn9 = assign7600_e8605_d_n9;
        locals.var_t0_dn10 = assign7600_e8605_d_n10;
        locals.var_t0_dn11 = assign7600_e8605_d_n11;
        locals.var_t0_dn12 = assign7600_e8605_d_n12;

        let (assign7610_e8614, assign7610_e8614_d_n3, assign7610_e8614_d_n4, assign7610_e8614_d_n5, assign7610_e8614_d_n6, assign7610_e8614_d_n7, assign7610_e8614_d_n8, assign7610_e8614_d_n9, assign7610_e8614_d_n10, assign7610_e8614_d_n11, assign7610_e8614_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7610_e8611: f64 = (locals.var_pparam_b4soikt1l / p.p54);
        let assign7610_e8612: f64 = (locals.var_pparam_b4soikt1 + assign7610_e8611);
        (assign7610_e8612, (locals.var_pparam_b4soikt1_dn3 + (locals.var_pparam_b4soikt1l_dn3 / p.p54)), (locals.var_pparam_b4soikt1_dn4 + (locals.var_pparam_b4soikt1l_dn4 / p.p54)), (locals.var_pparam_b4soikt1_dn5 + (locals.var_pparam_b4soikt1l_dn5 / p.p54)), (locals.var_pparam_b4soikt1_dn6 + (locals.var_pparam_b4soikt1l_dn6 / p.p54)), (locals.var_pparam_b4soikt1_dn7 + (locals.var_pparam_b4soikt1l_dn7 / p.p54)), (locals.var_pparam_b4soikt1_dn8 + (locals.var_pparam_b4soikt1l_dn8 / p.p54)), (locals.var_pparam_b4soikt1_dn9 + (locals.var_pparam_b4soikt1l_dn9 / p.p54)), (locals.var_pparam_b4soikt1_dn10 + (locals.var_pparam_b4soikt1l_dn10 / p.p54)), (locals.var_pparam_b4soikt1_dn11 + (locals.var_pparam_b4soikt1l_dn11 / p.p54)), (locals.var_pparam_b4soikt1_dn12 + (locals.var_pparam_b4soikt1l_dn12 / p.p54)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign7610_e8614;
        locals.var_t1_dn3 = assign7610_e8614_d_n3;
        locals.var_t1_dn4 = assign7610_e8614_d_n4;
        locals.var_t1_dn5 = assign7610_e8614_d_n5;
        locals.var_t1_dn6 = assign7610_e8614_d_n6;
        locals.var_t1_dn7 = assign7610_e8614_d_n7;
        locals.var_t1_dn8 = assign7610_e8614_d_n8;
        locals.var_t1_dn9 = assign7610_e8614_d_n9;
        locals.var_t1_dn10 = assign7610_e8614_d_n10;
        locals.var_t1_dn11 = assign7610_e8614_d_n11;
        locals.var_t1_dn12 = assign7610_e8614_d_n12;

        let (assign7620_e8629, assign7620_e8629_d_n3, assign7620_e8629_d_n4, assign7620_e8629_d_n5, assign7620_e8629_d_n6, assign7620_e8629_d_n7, assign7620_e8629_d_n8, assign7620_e8629_d_n9, assign7620_e8629_d_n10, assign7620_e8629_d_n11, assign7620_e8629_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7620_e8620: f64 = (locals.var_t0 - 1.0);
        let assign7620_e8621: f64 = (locals.var_here_b4soik1ox * assign7620_e8620);
        let assign7620_e8623: f64 = (assign7620_e8621 * locals.var_sqrtphieot);
        let assign7620_e8626: f64 = (locals.var_t1 * locals.var_trm1);
        let assign7620_e8627: f64 = (assign7620_e8623 + assign7620_e8626);
        (assign7620_e8627, (((((locals.var_here_b4soik1ox_dn3 * assign7620_e8620) + (locals.var_here_b4soik1ox * locals.var_t0_dn3)) * locals.var_sqrtphieot) + (assign7620_e8621 * locals.var_sqrtphieot_dn3)) + (locals.var_t1_dn3 * locals.var_trm1)), (((((locals.var_here_b4soik1ox_dn4 * assign7620_e8620) + (locals.var_here_b4soik1ox * locals.var_t0_dn4)) * locals.var_sqrtphieot) + (assign7620_e8621 * locals.var_sqrtphieot_dn4)) + ((locals.var_t1_dn4 * locals.var_trm1) + (locals.var_t1 * locals.var_trm1_dn4))), (((((locals.var_here_b4soik1ox_dn5 * assign7620_e8620) + (locals.var_here_b4soik1ox * locals.var_t0_dn5)) * locals.var_sqrtphieot) + (assign7620_e8621 * locals.var_sqrtphieot_dn5)) + ((locals.var_t1_dn5 * locals.var_trm1) + (locals.var_t1 * locals.var_trm1_dn5))), (((((locals.var_here_b4soik1ox_dn6 * assign7620_e8620) + (locals.var_here_b4soik1ox * locals.var_t0_dn6)) * locals.var_sqrtphieot) + (assign7620_e8621 * locals.var_sqrtphieot_dn6)) + ((locals.var_t1_dn6 * locals.var_trm1) + (locals.var_t1 * locals.var_trm1_dn6))), (((((locals.var_here_b4soik1ox_dn7 * assign7620_e8620) + (locals.var_here_b4soik1ox * locals.var_t0_dn7)) * locals.var_sqrtphieot) + (assign7620_e8621 * locals.var_sqrtphieot_dn7)) + (locals.var_t1_dn7 * locals.var_trm1)), (((((locals.var_here_b4soik1ox_dn8 * assign7620_e8620) + (locals.var_here_b4soik1ox * locals.var_t0_dn8)) * locals.var_sqrtphieot) + (assign7620_e8621 * locals.var_sqrtphieot_dn8)) + (locals.var_t1_dn8 * locals.var_trm1)), (((((locals.var_here_b4soik1ox_dn9 * assign7620_e8620) + (locals.var_here_b4soik1ox * locals.var_t0_dn9)) * locals.var_sqrtphieot) + (assign7620_e8621 * locals.var_sqrtphieot_dn9)) + (locals.var_t1_dn9 * locals.var_trm1)), (((((locals.var_here_b4soik1ox_dn10 * assign7620_e8620) + (locals.var_here_b4soik1ox * locals.var_t0_dn10)) * locals.var_sqrtphieot) + (assign7620_e8621 * locals.var_sqrtphieot_dn10)) + (locals.var_t1_dn10 * locals.var_trm1)), (((((locals.var_here_b4soik1ox_dn11 * assign7620_e8620) + (locals.var_here_b4soik1ox * locals.var_t0_dn11)) * locals.var_sqrtphieot) + (assign7620_e8621 * locals.var_sqrtphieot_dn11)) + (locals.var_t1_dn11 * locals.var_trm1)), (((((locals.var_here_b4soik1ox_dn12 * assign7620_e8620) + (locals.var_here_b4soik1ox * locals.var_t0_dn12)) * locals.var_sqrtphieot) + (assign7620_e8621 * locals.var_sqrtphieot_dn12)) + (locals.var_t1_dn12 * locals.var_trm1)),)
    } else {
        (locals.var_deltvthtemp, locals.var_deltvthtemp_dn3, locals.var_deltvthtemp_dn4, locals.var_deltvthtemp_dn5, locals.var_deltvthtemp_dn6, locals.var_deltvthtemp_dn7, locals.var_deltvthtemp_dn8, locals.var_deltvthtemp_dn9, locals.var_deltvthtemp_dn10, locals.var_deltvthtemp_dn11, locals.var_deltvthtemp_dn12,)
    }
};
        locals.var_deltvthtemp = assign7620_e8629;
        locals.var_deltvthtemp_dn3 = assign7620_e8629_d_n3;
        locals.var_deltvthtemp_dn4 = assign7620_e8629_d_n4;
        locals.var_deltvthtemp_dn5 = assign7620_e8629_d_n5;
        locals.var_deltvthtemp_dn6 = assign7620_e8629_d_n6;
        locals.var_deltvthtemp_dn7 = assign7620_e8629_d_n7;
        locals.var_deltvthtemp_dn8 = assign7620_e8629_d_n8;
        locals.var_deltvthtemp_dn9 = assign7620_e8629_d_n9;
        locals.var_deltvthtemp_dn10 = assign7620_e8629_d_n10;
        locals.var_deltvthtemp_dn11 = assign7620_e8629_d_n11;
        locals.var_deltvthtemp_dn12 = assign7620_e8629_d_n12;

        let (assign7630_e8640, assign7630_e8640_d_n3, assign7630_e8640_d_n4, assign7630_e8640_d_n5, assign7630_e8640_d_n6, assign7630_e8640_d_n7, assign7630_e8640_d_n8, assign7630_e8640_d_n9, assign7630_e8640_d_n10, assign7630_e8640_d_n11, assign7630_e8640_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7630_e8634: f64 = (locals.var_toxe * locals.var_phieot);
        let assign7630_e8637: f64 = (p.p55 + locals.var_pparam_b4soiw0);
        let assign7630_e8638: f64 = (assign7630_e8634 / assign7630_e8637);
        (assign7630_e8638, ((((locals.var_toxe * locals.var_phieot_dn3) * assign7630_e8637) - (assign7630_e8634 * locals.var_pparam_b4soiw0_dn3)) / (assign7630_e8637 * assign7630_e8637)), ((((locals.var_toxe * locals.var_phieot_dn4) * assign7630_e8637) - (assign7630_e8634 * locals.var_pparam_b4soiw0_dn4)) / (assign7630_e8637 * assign7630_e8637)), ((((locals.var_toxe * locals.var_phieot_dn5) * assign7630_e8637) - (assign7630_e8634 * locals.var_pparam_b4soiw0_dn5)) / (assign7630_e8637 * assign7630_e8637)), ((((locals.var_toxe * locals.var_phieot_dn6) * assign7630_e8637) - (assign7630_e8634 * locals.var_pparam_b4soiw0_dn6)) / (assign7630_e8637 * assign7630_e8637)), ((((locals.var_toxe * locals.var_phieot_dn7) * assign7630_e8637) - (assign7630_e8634 * locals.var_pparam_b4soiw0_dn7)) / (assign7630_e8637 * assign7630_e8637)), ((((locals.var_toxe * locals.var_phieot_dn8) * assign7630_e8637) - (assign7630_e8634 * locals.var_pparam_b4soiw0_dn8)) / (assign7630_e8637 * assign7630_e8637)), ((((locals.var_toxe * locals.var_phieot_dn9) * assign7630_e8637) - (assign7630_e8634 * locals.var_pparam_b4soiw0_dn9)) / (assign7630_e8637 * assign7630_e8637)), ((((locals.var_toxe * locals.var_phieot_dn10) * assign7630_e8637) - (assign7630_e8634 * locals.var_pparam_b4soiw0_dn10)) / (assign7630_e8637 * assign7630_e8637)), ((((locals.var_toxe * locals.var_phieot_dn11) * assign7630_e8637) - (assign7630_e8634 * locals.var_pparam_b4soiw0_dn11)) / (assign7630_e8637 * assign7630_e8637)), ((((locals.var_toxe * locals.var_phieot_dn12) * assign7630_e8637) - (assign7630_e8634 * locals.var_pparam_b4soiw0_dn12)) / (assign7630_e8637 * assign7630_e8637)),)
    } else {
        (locals.var_tmp2, locals.var_tmp2_dn3, locals.var_tmp2_dn4, locals.var_tmp2_dn5, locals.var_tmp2_dn6, locals.var_tmp2_dn7, locals.var_tmp2_dn8, locals.var_tmp2_dn9, locals.var_tmp2_dn10, locals.var_tmp2_dn11, locals.var_tmp2_dn12,)
    }
};
        locals.var_tmp2 = assign7630_e8640;
        locals.var_tmp2_dn3 = assign7630_e8640_d_n3;
        locals.var_tmp2_dn4 = assign7630_e8640_d_n4;
        locals.var_tmp2_dn5 = assign7630_e8640_d_n5;
        locals.var_tmp2_dn6 = assign7630_e8640_d_n6;
        locals.var_tmp2_dn7 = assign7630_e8640_d_n7;
        locals.var_tmp2_dn8 = assign7630_e8640_d_n8;
        locals.var_tmp2_dn9 = assign7630_e8640_d_n9;
        locals.var_tmp2_dn10 = assign7630_e8640_d_n10;
        locals.var_tmp2_dn11 = assign7630_e8640_d_n11;
        locals.var_tmp2_dn12 = assign7630_e8640_d_n12;

        let (assign7640_e8645,) = {
    if (locals.var_guard594 == 0.0) {
        (0.0,)
    } else {
        (locals.var_dibl_sft,)
    }
};
        locals.var_dibl_sft = assign7640_e8645;

        let (assign7650_e8650,) = {
    if (locals.var_guard594 == 0.0) {
        (0.0,)
    } else {
        (locals.var_dits_sft2,)
    }
};
        locals.var_dits_sft2 = assign7650_e8650;

        let (assign7660_e8660, assign7660_e8660_d_n3, assign7660_e8660_d_n4, assign7660_e8660_d_n5, assign7660_e8660_d_n6, assign7660_e8660_d_n7, assign7660_e8660_d_n8, assign7660_e8660_d_n9, assign7660_e8660_d_n10, assign7660_e8660_d_n11, assign7660_e8660_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7660_e8656: f64 = (locals.var_pparam_b4soilpeb / p.p54);
        let assign7660_e8657: f64 = (1.0 + assign7660_e8656);
        let assign7660_e8658: f64 = (assign7660_e8657).sqrt();
        (assign7660_e8658, ((locals.var_pparam_b4soilpeb_dn3 / p.p54) / (2.0 * assign7660_e8658)), ((locals.var_pparam_b4soilpeb_dn4 / p.p54) / (2.0 * assign7660_e8658)), ((locals.var_pparam_b4soilpeb_dn5 / p.p54) / (2.0 * assign7660_e8658)), ((locals.var_pparam_b4soilpeb_dn6 / p.p54) / (2.0 * assign7660_e8658)), ((locals.var_pparam_b4soilpeb_dn7 / p.p54) / (2.0 * assign7660_e8658)), ((locals.var_pparam_b4soilpeb_dn8 / p.p54) / (2.0 * assign7660_e8658)), ((locals.var_pparam_b4soilpeb_dn9 / p.p54) / (2.0 * assign7660_e8658)), ((locals.var_pparam_b4soilpeb_dn10 / p.p54) / (2.0 * assign7660_e8658)), ((locals.var_pparam_b4soilpeb_dn11 / p.p54) / (2.0 * assign7660_e8658)), ((locals.var_pparam_b4soilpeb_dn12 / p.p54) / (2.0 * assign7660_e8658)),)
    } else {
        (locals.var_lpe_vb, locals.var_lpe_vb_dn3, locals.var_lpe_vb_dn4, locals.var_lpe_vb_dn5, locals.var_lpe_vb_dn6, locals.var_lpe_vb_dn7, locals.var_lpe_vb_dn8, locals.var_lpe_vb_dn9, locals.var_lpe_vb_dn10, locals.var_lpe_vb_dn11, locals.var_lpe_vb_dn12,)
    }
};
        locals.var_lpe_vb = assign7660_e8660;
        locals.var_lpe_vb_dn3 = assign7660_e8660_d_n3;
        locals.var_lpe_vb_dn4 = assign7660_e8660_d_n4;
        locals.var_lpe_vb_dn5 = assign7660_e8660_d_n5;
        locals.var_lpe_vb_dn6 = assign7660_e8660_d_n6;
        locals.var_lpe_vb_dn7 = assign7660_e8660_d_n7;
        locals.var_lpe_vb_dn8 = assign7660_e8660_d_n8;
        locals.var_lpe_vb_dn9 = assign7660_e8660_d_n9;
        locals.var_lpe_vb_dn10 = assign7660_e8660_d_n10;
        locals.var_lpe_vb_dn11 = assign7660_e8660_d_n11;
        locals.var_lpe_vb_dn12 = assign7660_e8660_d_n12;

        let (assign7670_e8665, assign7670_e8665_d_n3, assign7670_e8665_d_n4, assign7670_e8665_d_n5, assign7670_e8665_d_n6, assign7670_e8665_d_n7, assign7670_e8665_d_n8, assign7670_e8665_d_n9, assign7670_e8665_d_n10, assign7670_e8665_d_n11, assign7670_e8665_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        (locals.var_sqrtphieot, locals.var_sqrtphieot_dn3, locals.var_sqrtphieot_dn4, locals.var_sqrtphieot_dn5, locals.var_sqrtphieot_dn6, locals.var_sqrtphieot_dn7, locals.var_sqrtphieot_dn8, locals.var_sqrtphieot_dn9, locals.var_sqrtphieot_dn10, locals.var_sqrtphieot_dn11, locals.var_sqrtphieot_dn12,)
    } else {
        (locals.var_sqrtphisext, locals.var_sqrtphisext_dn3, locals.var_sqrtphisext_dn4, locals.var_sqrtphisext_dn5, locals.var_sqrtphisext_dn6, locals.var_sqrtphisext_dn7, locals.var_sqrtphisext_dn8, locals.var_sqrtphisext_dn9, locals.var_sqrtphisext_dn10, locals.var_sqrtphisext_dn11, locals.var_sqrtphisext_dn12,)
    }
};
        locals.var_sqrtphisext = assign7670_e8665;
        locals.var_sqrtphisext_dn3 = assign7670_e8665_d_n3;
        locals.var_sqrtphisext_dn4 = assign7670_e8665_d_n4;
        locals.var_sqrtphisext_dn5 = assign7670_e8665_d_n5;
        locals.var_sqrtphisext_dn6 = assign7670_e8665_d_n6;
        locals.var_sqrtphisext_dn7 = assign7670_e8665_d_n7;
        locals.var_sqrtphisext_dn8 = assign7670_e8665_d_n8;
        locals.var_sqrtphisext_dn9 = assign7670_e8665_d_n9;
        locals.var_sqrtphisext_dn10 = assign7670_e8665_d_n10;
        locals.var_sqrtphisext_dn11 = assign7670_e8665_d_n11;
        locals.var_sqrtphisext_dn12 = assign7670_e8665_d_n12;

    }

    pub(super) fn stamp_transient_block_22(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign7680_e8698, assign7680_e8698_d_n3, assign7680_e8698_d_n4, assign7680_e8698_d_n5, assign7680_e8698_d_n6, assign7680_e8698_d_n7, assign7680_e8698_d_n8, assign7680_e8698_d_n9, assign7680_e8698_d_n10, assign7680_e8698_d_n11, assign7680_e8698_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7680_e8670: f64 = (p.p37 * locals.var_here_b4soivth0);
        let assign7680_e8673: f64 = (locals.var_here_b4soik1ox * locals.var_sqrtphisext);
        let assign7680_e8676: f64 = (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot);
        let assign7680_e8677: f64 = (assign7680_e8673 - assign7680_e8676);
        let assign7680_e8679: f64 = (assign7680_e8677 * locals.var_lpe_vb);
        let assign7680_e8680: f64 = (assign7680_e8670 + assign7680_e8679);
        let assign7680_e8682: f64 = (assign7680_e8680 - locals.var_delt_vth);
        let assign7680_e8684: f64 = (assign7680_e8682 - locals.var_deltvthw);
        let assign7680_e8687: f64 = (locals.var_pparam_b4soik3 * locals.var_tmp2);
        let assign7680_e8688: f64 = (assign7680_e8684 + assign7680_e8687);
        let assign7680_e8690: f64 = (assign7680_e8688 + locals.var_deltvthtemp);
        let assign7680_e8692: f64 = (assign7680_e8690 - locals.var_dibl_sft);
        let assign7680_e8694: f64 = (assign7680_e8692 - locals.var_dits_sft);
        let assign7680_e8696: f64 = (assign7680_e8694 - locals.var_dits_sft2);
        (assign7680_e8696, (((((((p.p37 * locals.var_here_b4soivth0_dn3) + (((((locals.var_here_b4soik1ox_dn3 * locals.var_sqrtphisext) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext_dn3)) - ((locals.var_pparam_b4soik1eff_dn3 * locals.var_sqrtphieot) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot_dn3))) * locals.var_lpe_vb) + (assign7680_e8677 * locals.var_lpe_vb_dn3))) - locals.var_delt_vth_dn3) - locals.var_deltvthw_dn3) + ((locals.var_pparam_b4soik3_dn3 * locals.var_tmp2) + (locals.var_pparam_b4soik3 * locals.var_tmp2_dn3))) + locals.var_deltvthtemp_dn3) - locals.var_dits_sft_dn3), (((((((p.p37 * locals.var_here_b4soivth0_dn4) + (((((locals.var_here_b4soik1ox_dn4 * locals.var_sqrtphisext) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext_dn4)) - ((locals.var_pparam_b4soik1eff_dn4 * locals.var_sqrtphieot) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot_dn4))) * locals.var_lpe_vb) + (assign7680_e8677 * locals.var_lpe_vb_dn4))) - locals.var_delt_vth_dn4) - locals.var_deltvthw_dn4) + ((locals.var_pparam_b4soik3_dn4 * locals.var_tmp2) + (locals.var_pparam_b4soik3 * locals.var_tmp2_dn4))) + locals.var_deltvthtemp_dn4) - locals.var_dits_sft_dn4), (((((((p.p37 * locals.var_here_b4soivth0_dn5) + (((((locals.var_here_b4soik1ox_dn5 * locals.var_sqrtphisext) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext_dn5)) - ((locals.var_pparam_b4soik1eff_dn5 * locals.var_sqrtphieot) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot_dn5))) * locals.var_lpe_vb) + (assign7680_e8677 * locals.var_lpe_vb_dn5))) - locals.var_delt_vth_dn5) - locals.var_deltvthw_dn5) + ((locals.var_pparam_b4soik3_dn5 * locals.var_tmp2) + (locals.var_pparam_b4soik3 * locals.var_tmp2_dn5))) + locals.var_deltvthtemp_dn5) - locals.var_dits_sft_dn5), (((((((p.p37 * locals.var_here_b4soivth0_dn6) + (((((locals.var_here_b4soik1ox_dn6 * locals.var_sqrtphisext) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext_dn6)) - ((locals.var_pparam_b4soik1eff_dn6 * locals.var_sqrtphieot) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot_dn6))) * locals.var_lpe_vb) + (assign7680_e8677 * locals.var_lpe_vb_dn6))) - locals.var_delt_vth_dn6) - locals.var_deltvthw_dn6) + ((locals.var_pparam_b4soik3_dn6 * locals.var_tmp2) + (locals.var_pparam_b4soik3 * locals.var_tmp2_dn6))) + locals.var_deltvthtemp_dn6) - locals.var_dits_sft_dn6), (((((((p.p37 * locals.var_here_b4soivth0_dn7) + (((((locals.var_here_b4soik1ox_dn7 * locals.var_sqrtphisext) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext_dn7)) - ((locals.var_pparam_b4soik1eff_dn7 * locals.var_sqrtphieot) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot_dn7))) * locals.var_lpe_vb) + (assign7680_e8677 * locals.var_lpe_vb_dn7))) - locals.var_delt_vth_dn7) - locals.var_deltvthw_dn7) + ((locals.var_pparam_b4soik3_dn7 * locals.var_tmp2) + (locals.var_pparam_b4soik3 * locals.var_tmp2_dn7))) + locals.var_deltvthtemp_dn7) - locals.var_dits_sft_dn7), (((((((p.p37 * locals.var_here_b4soivth0_dn8) + (((((locals.var_here_b4soik1ox_dn8 * locals.var_sqrtphisext) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext_dn8)) - ((locals.var_pparam_b4soik1eff_dn8 * locals.var_sqrtphieot) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot_dn8))) * locals.var_lpe_vb) + (assign7680_e8677 * locals.var_lpe_vb_dn8))) - locals.var_delt_vth_dn8) - locals.var_deltvthw_dn8) + ((locals.var_pparam_b4soik3_dn8 * locals.var_tmp2) + (locals.var_pparam_b4soik3 * locals.var_tmp2_dn8))) + locals.var_deltvthtemp_dn8) - locals.var_dits_sft_dn8), (((((((p.p37 * locals.var_here_b4soivth0_dn9) + (((((locals.var_here_b4soik1ox_dn9 * locals.var_sqrtphisext) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext_dn9)) - ((locals.var_pparam_b4soik1eff_dn9 * locals.var_sqrtphieot) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot_dn9))) * locals.var_lpe_vb) + (assign7680_e8677 * locals.var_lpe_vb_dn9))) - locals.var_delt_vth_dn9) - locals.var_deltvthw_dn9) + ((locals.var_pparam_b4soik3_dn9 * locals.var_tmp2) + (locals.var_pparam_b4soik3 * locals.var_tmp2_dn9))) + locals.var_deltvthtemp_dn9) - locals.var_dits_sft_dn9), (((((((p.p37 * locals.var_here_b4soivth0_dn10) + (((((locals.var_here_b4soik1ox_dn10 * locals.var_sqrtphisext) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext_dn10)) - ((locals.var_pparam_b4soik1eff_dn10 * locals.var_sqrtphieot) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot_dn10))) * locals.var_lpe_vb) + (assign7680_e8677 * locals.var_lpe_vb_dn10))) - locals.var_delt_vth_dn10) - locals.var_deltvthw_dn10) + ((locals.var_pparam_b4soik3_dn10 * locals.var_tmp2) + (locals.var_pparam_b4soik3 * locals.var_tmp2_dn10))) + locals.var_deltvthtemp_dn10) - locals.var_dits_sft_dn10), (((((((p.p37 * locals.var_here_b4soivth0_dn11) + (((((locals.var_here_b4soik1ox_dn11 * locals.var_sqrtphisext) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext_dn11)) - ((locals.var_pparam_b4soik1eff_dn11 * locals.var_sqrtphieot) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot_dn11))) * locals.var_lpe_vb) + (assign7680_e8677 * locals.var_lpe_vb_dn11))) - locals.var_delt_vth_dn11) - locals.var_deltvthw_dn11) + ((locals.var_pparam_b4soik3_dn11 * locals.var_tmp2) + (locals.var_pparam_b4soik3 * locals.var_tmp2_dn11))) + locals.var_deltvthtemp_dn11) - locals.var_dits_sft_dn11), (((((((p.p37 * locals.var_here_b4soivth0_dn12) + (((((locals.var_here_b4soik1ox_dn12 * locals.var_sqrtphisext) + (locals.var_here_b4soik1ox * locals.var_sqrtphisext_dn12)) - ((locals.var_pparam_b4soik1eff_dn12 * locals.var_sqrtphieot) + (locals.var_pparam_b4soik1eff * locals.var_sqrtphieot_dn12))) * locals.var_lpe_vb) + (assign7680_e8677 * locals.var_lpe_vb_dn12))) - locals.var_delt_vth_dn12) - locals.var_deltvthw_dn12) + ((locals.var_pparam_b4soik3_dn12 * locals.var_tmp2) + (locals.var_pparam_b4soik3 * locals.var_tmp2_dn12))) + locals.var_deltvthtemp_dn12) - locals.var_dits_sft_dn12),)
    } else {
        (locals.var_vth_1, locals.var_vth_1_dn3, locals.var_vth_1_dn4, locals.var_vth_1_dn5, locals.var_vth_1_dn6, locals.var_vth_1_dn7, locals.var_vth_1_dn8, locals.var_vth_1_dn9, locals.var_vth_1_dn10, locals.var_vth_1_dn11, locals.var_vth_1_dn12,)
    }
};
        locals.var_vth_1 = assign7680_e8698;
        locals.var_vth_1_dn3 = assign7680_e8698_d_n3;
        locals.var_vth_1_dn4 = assign7680_e8698_d_n4;
        locals.var_vth_1_dn5 = assign7680_e8698_d_n5;
        locals.var_vth_1_dn6 = assign7680_e8698_d_n6;
        locals.var_vth_1_dn7 = assign7680_e8698_d_n7;
        locals.var_vth_1_dn8 = assign7680_e8698_d_n8;
        locals.var_vth_1_dn9 = assign7680_e8698_d_n9;
        locals.var_vth_1_dn10 = assign7680_e8698_d_n10;
        locals.var_vth_1_dn11 = assign7680_e8698_d_n11;
        locals.var_vth_1_dn12 = assign7680_e8698_d_n12;

        let (assign7690_e8705, assign7690_e8705_d_n3, assign7690_e8705_d_n4, assign7690_e8705_d_n5, assign7690_e8705_d_n6, assign7690_e8705_d_n7, assign7690_e8705_d_n8, assign7690_e8705_d_n9, assign7690_e8705_d_n10, assign7690_e8705_d_n11, assign7690_e8705_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7690_e8703: f64 = (locals.var_vgs_eff - locals.var_vth_1);
        (assign7690_e8703, (locals.var_vgs_eff_dn3 - locals.var_vth_1_dn3), (locals.var_vgs_eff_dn4 - locals.var_vth_1_dn4), (locals.var_vgs_eff_dn5 - locals.var_vth_1_dn5), (locals.var_vgs_eff_dn6 - locals.var_vth_1_dn6), (locals.var_vgs_eff_dn7 - locals.var_vth_1_dn7), (locals.var_vgs_eff_dn8 - locals.var_vth_1_dn8), (locals.var_vgs_eff_dn9 - locals.var_vth_1_dn9), (locals.var_vgs_eff_dn10 - locals.var_vth_1_dn10), (locals.var_vgs_eff_dn11 - locals.var_vth_1_dn11), (locals.var_vgs_eff_dn12 - locals.var_vth_1_dn12),)
    } else {
        (locals.var_vgst, locals.var_vgst_dn3, locals.var_vgst_dn4, locals.var_vgst_dn5, locals.var_vgst_dn6, locals.var_vgst_dn7, locals.var_vgst_dn8, locals.var_vgst_dn9, locals.var_vgst_dn10, locals.var_vgst_dn11, locals.var_vgst_dn12,)
    }
};
        locals.var_vgst = assign7690_e8705;
        locals.var_vgst_dn3 = assign7690_e8705_d_n3;
        locals.var_vgst_dn4 = assign7690_e8705_d_n4;
        locals.var_vgst_dn5 = assign7690_e8705_d_n5;
        locals.var_vgst_dn6 = assign7690_e8705_d_n6;
        locals.var_vgst_dn7 = assign7690_e8705_d_n7;
        locals.var_vgst_dn8 = assign7690_e8705_d_n8;
        locals.var_vgst_dn9 = assign7690_e8705_d_n9;
        locals.var_vgst_dn10 = assign7690_e8705_d_n10;
        locals.var_vgst_dn11 = assign7690_e8705_d_n11;
        locals.var_vgst_dn12 = assign7690_e8705_d_n12;

        let (assign7700_e8712, assign7700_e8712_d_n3, assign7700_e8712_d_n4, assign7700_e8712_d_n5, assign7700_e8712_d_n6, assign7700_e8712_d_n7, assign7700_e8712_d_n8, assign7700_e8712_d_n9, assign7700_e8712_d_n10, assign7700_e8712_d_n11, assign7700_e8712_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7700_e8710: f64 = (locals.var_n * locals.var_vtmeot);
        (assign7700_e8710, (locals.var_n_dn3 * locals.var_vtmeot), (locals.var_n_dn4 * locals.var_vtmeot), (locals.var_n_dn5 * locals.var_vtmeot), (locals.var_n_dn6 * locals.var_vtmeot), (locals.var_n_dn7 * locals.var_vtmeot), (locals.var_n_dn8 * locals.var_vtmeot), (locals.var_n_dn9 * locals.var_vtmeot), (locals.var_n_dn10 * locals.var_vtmeot), (locals.var_n_dn11 * locals.var_vtmeot), (locals.var_n_dn12 * locals.var_vtmeot),)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn12,)
    }
};
        locals.var_t10 = assign7700_e8712;
        locals.var_t10_dn3 = assign7700_e8712_d_n3;
        locals.var_t10_dn4 = assign7700_e8712_d_n4;
        locals.var_t10_dn5 = assign7700_e8712_d_n5;
        locals.var_t10_dn6 = assign7700_e8712_d_n6;
        locals.var_t10_dn7 = assign7700_e8712_d_n7;
        locals.var_t10_dn8 = assign7700_e8712_d_n8;
        locals.var_t10_dn9 = assign7700_e8712_d_n9;
        locals.var_t10_dn10 = assign7700_e8712_d_n10;
        locals.var_t10_dn11 = assign7700_e8712_d_n11;
        locals.var_t10_dn12 = assign7700_e8712_d_n12;

        let (assign7710_e8721, assign7710_e8721_d_n3, assign7710_e8721_d_n4, assign7710_e8721_d_n5, assign7710_e8721_d_n6, assign7710_e8721_d_n7, assign7710_e8721_d_n8, assign7710_e8721_d_n9, assign7710_e8721_d_n10, assign7710_e8721_d_n11, assign7710_e8721_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7710_e8717: f64 = (locals.var_pparam_b4soimstar * locals.var_vgst);
        let assign7710_e8719: f64 = (assign7710_e8717 / locals.var_t10);
        (assign7710_e8719, (((((locals.var_pparam_b4soimstar_dn3 * locals.var_vgst) + (locals.var_pparam_b4soimstar * locals.var_vgst_dn3)) * locals.var_t10) - (assign7710_e8717 * locals.var_t10_dn3)) / (locals.var_t10 * locals.var_t10)), (((((locals.var_pparam_b4soimstar_dn4 * locals.var_vgst) + (locals.var_pparam_b4soimstar * locals.var_vgst_dn4)) * locals.var_t10) - (assign7710_e8717 * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)), (((((locals.var_pparam_b4soimstar_dn5 * locals.var_vgst) + (locals.var_pparam_b4soimstar * locals.var_vgst_dn5)) * locals.var_t10) - (assign7710_e8717 * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)), (((((locals.var_pparam_b4soimstar_dn6 * locals.var_vgst) + (locals.var_pparam_b4soimstar * locals.var_vgst_dn6)) * locals.var_t10) - (assign7710_e8717 * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)), (((((locals.var_pparam_b4soimstar_dn7 * locals.var_vgst) + (locals.var_pparam_b4soimstar * locals.var_vgst_dn7)) * locals.var_t10) - (assign7710_e8717 * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)), (((((locals.var_pparam_b4soimstar_dn8 * locals.var_vgst) + (locals.var_pparam_b4soimstar * locals.var_vgst_dn8)) * locals.var_t10) - (assign7710_e8717 * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)), (((((locals.var_pparam_b4soimstar_dn9 * locals.var_vgst) + (locals.var_pparam_b4soimstar * locals.var_vgst_dn9)) * locals.var_t10) - (assign7710_e8717 * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)), (((((locals.var_pparam_b4soimstar_dn10 * locals.var_vgst) + (locals.var_pparam_b4soimstar * locals.var_vgst_dn10)) * locals.var_t10) - (assign7710_e8717 * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)), (((((locals.var_pparam_b4soimstar_dn11 * locals.var_vgst) + (locals.var_pparam_b4soimstar * locals.var_vgst_dn11)) * locals.var_t10) - (assign7710_e8717 * locals.var_t10_dn11)) / (locals.var_t10 * locals.var_t10)), (((((locals.var_pparam_b4soimstar_dn12 * locals.var_vgst) + (locals.var_pparam_b4soimstar * locals.var_vgst_dn12)) * locals.var_t10) - (assign7710_e8717 * locals.var_t10_dn12)) / (locals.var_t10 * locals.var_t10)),)
    } else {
        (locals.var_vgstnvt, locals.var_vgstnvt_dn3, locals.var_vgstnvt_dn4, locals.var_vgstnvt_dn5, locals.var_vgstnvt_dn6, locals.var_vgstnvt_dn7, locals.var_vgstnvt_dn8, locals.var_vgstnvt_dn9, locals.var_vgstnvt_dn10, locals.var_vgstnvt_dn11, locals.var_vgstnvt_dn12,)
    }
};
        locals.var_vgstnvt = assign7710_e8721;
        locals.var_vgstnvt_dn3 = assign7710_e8721_d_n3;
        locals.var_vgstnvt_dn4 = assign7710_e8721_d_n4;
        locals.var_vgstnvt_dn5 = assign7710_e8721_d_n5;
        locals.var_vgstnvt_dn6 = assign7710_e8721_d_n6;
        locals.var_vgstnvt_dn7 = assign7710_e8721_d_n7;
        locals.var_vgstnvt_dn8 = assign7710_e8721_d_n8;
        locals.var_vgstnvt_dn9 = assign7710_e8721_d_n9;
        locals.var_vgstnvt_dn10 = assign7710_e8721_d_n10;
        locals.var_vgstnvt_dn11 = assign7710_e8721_d_n11;
        locals.var_vgstnvt_dn12 = assign7710_e8721_d_n12;

        let (assign7720_e8734, assign7720_e8734_d_n3, assign7720_e8734_d_n4, assign7720_e8734_d_n5, assign7720_e8734_d_n6, assign7720_e8734_d_n7, assign7720_e8734_d_n8, assign7720_e8734_d_n9, assign7720_e8734_d_n10, assign7720_e8734_d_n11, assign7720_e8734_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7720_e8727: f64 = (1.0 - locals.var_pparam_b4soimstar);
        let assign7720_e8729: f64 = (assign7720_e8727 * locals.var_vgst);
        let assign7720_e8730: f64 = (locals.var_pparam_b4soivoff - assign7720_e8729);
        let assign7720_e8732: f64 = (assign7720_e8730 / locals.var_t10);
        (assign7720_e8732, ((((locals.var_pparam_b4soivoff_dn3 - (((-locals.var_pparam_b4soimstar_dn3) * locals.var_vgst) + (assign7720_e8727 * locals.var_vgst_dn3))) * locals.var_t10) - (assign7720_e8730 * locals.var_t10_dn3)) / (locals.var_t10 * locals.var_t10)), ((((locals.var_pparam_b4soivoff_dn4 - (((-locals.var_pparam_b4soimstar_dn4) * locals.var_vgst) + (assign7720_e8727 * locals.var_vgst_dn4))) * locals.var_t10) - (assign7720_e8730 * locals.var_t10_dn4)) / (locals.var_t10 * locals.var_t10)), ((((locals.var_pparam_b4soivoff_dn5 - (((-locals.var_pparam_b4soimstar_dn5) * locals.var_vgst) + (assign7720_e8727 * locals.var_vgst_dn5))) * locals.var_t10) - (assign7720_e8730 * locals.var_t10_dn5)) / (locals.var_t10 * locals.var_t10)), ((((locals.var_pparam_b4soivoff_dn6 - (((-locals.var_pparam_b4soimstar_dn6) * locals.var_vgst) + (assign7720_e8727 * locals.var_vgst_dn6))) * locals.var_t10) - (assign7720_e8730 * locals.var_t10_dn6)) / (locals.var_t10 * locals.var_t10)), ((((locals.var_pparam_b4soivoff_dn7 - (((-locals.var_pparam_b4soimstar_dn7) * locals.var_vgst) + (assign7720_e8727 * locals.var_vgst_dn7))) * locals.var_t10) - (assign7720_e8730 * locals.var_t10_dn7)) / (locals.var_t10 * locals.var_t10)), ((((locals.var_pparam_b4soivoff_dn8 - (((-locals.var_pparam_b4soimstar_dn8) * locals.var_vgst) + (assign7720_e8727 * locals.var_vgst_dn8))) * locals.var_t10) - (assign7720_e8730 * locals.var_t10_dn8)) / (locals.var_t10 * locals.var_t10)), ((((locals.var_pparam_b4soivoff_dn9 - (((-locals.var_pparam_b4soimstar_dn9) * locals.var_vgst) + (assign7720_e8727 * locals.var_vgst_dn9))) * locals.var_t10) - (assign7720_e8730 * locals.var_t10_dn9)) / (locals.var_t10 * locals.var_t10)), ((((locals.var_pparam_b4soivoff_dn10 - (((-locals.var_pparam_b4soimstar_dn10) * locals.var_vgst) + (assign7720_e8727 * locals.var_vgst_dn10))) * locals.var_t10) - (assign7720_e8730 * locals.var_t10_dn10)) / (locals.var_t10 * locals.var_t10)), ((((locals.var_pparam_b4soivoff_dn11 - (((-locals.var_pparam_b4soimstar_dn11) * locals.var_vgst) + (assign7720_e8727 * locals.var_vgst_dn11))) * locals.var_t10) - (assign7720_e8730 * locals.var_t10_dn11)) / (locals.var_t10 * locals.var_t10)), ((((locals.var_pparam_b4soivoff_dn12 - (((-locals.var_pparam_b4soimstar_dn12) * locals.var_vgst) + (assign7720_e8727 * locals.var_vgst_dn12))) * locals.var_t10) - (assign7720_e8730 * locals.var_t10_dn12)) / (locals.var_t10 * locals.var_t10)),)
    } else {
        (locals.var_exparg, locals.var_exparg_dn3, locals.var_exparg_dn4, locals.var_exparg_dn5, locals.var_exparg_dn6, locals.var_exparg_dn7, locals.var_exparg_dn8, locals.var_exparg_dn9, locals.var_exparg_dn10, locals.var_exparg_dn11, locals.var_exparg_dn12,)
    }
};
        locals.var_exparg = assign7720_e8734;
        locals.var_exparg_dn3 = assign7720_e8734_d_n3;
        locals.var_exparg_dn4 = assign7720_e8734_d_n4;
        locals.var_exparg_dn5 = assign7720_e8734_d_n5;
        locals.var_exparg_dn6 = assign7720_e8734_d_n6;
        locals.var_exparg_dn7 = assign7720_e8734_d_n7;
        locals.var_exparg_dn8 = assign7720_e8734_d_n8;
        locals.var_exparg_dn9 = assign7720_e8734_d_n9;
        locals.var_exparg_dn10 = assign7720_e8734_d_n10;
        locals.var_exparg_dn11 = assign7720_e8734_d_n11;
        locals.var_exparg_dn12 = assign7720_e8734_d_n12;

        let assign7730_e8737: f64 = if locals.var_vgstnvt > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard600 = assign7730_e8737;

        let (assign7740_e8744, assign7740_e8744_d_n3, assign7740_e8744_d_n4, assign7740_e8744_d_n5, assign7740_e8744_d_n6, assign7740_e8744_d_n7, assign7740_e8744_d_n8, assign7740_e8744_d_n9, assign7740_e8744_d_n10, assign7740_e8744_d_n11, assign7740_e8744_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard600 != 0.0)) {
        (locals.var_vgst, locals.var_vgst_dn3, locals.var_vgst_dn4, locals.var_vgst_dn5, locals.var_vgst_dn6, locals.var_vgst_dn7, locals.var_vgst_dn8, locals.var_vgst_dn9, locals.var_vgst_dn10, locals.var_vgst_dn11, locals.var_vgst_dn12,)
    } else {
        (locals.var_vgsteff, locals.var_vgsteff_dn3, locals.var_vgsteff_dn4, locals.var_vgsteff_dn5, locals.var_vgsteff_dn6, locals.var_vgsteff_dn7, locals.var_vgsteff_dn8, locals.var_vgsteff_dn9, locals.var_vgsteff_dn10, locals.var_vgsteff_dn11, locals.var_vgsteff_dn12,)
    }
};
        locals.var_vgsteff = assign7740_e8744;
        locals.var_vgsteff_dn3 = assign7740_e8744_d_n3;
        locals.var_vgsteff_dn4 = assign7740_e8744_d_n4;
        locals.var_vgsteff_dn5 = assign7740_e8744_d_n5;
        locals.var_vgsteff_dn6 = assign7740_e8744_d_n6;
        locals.var_vgsteff_dn7 = assign7740_e8744_d_n7;
        locals.var_vgsteff_dn8 = assign7740_e8744_d_n8;
        locals.var_vgsteff_dn9 = assign7740_e8744_d_n9;
        locals.var_vgsteff_dn10 = assign7740_e8744_d_n10;
        locals.var_vgsteff_dn11 = assign7740_e8744_d_n11;
        locals.var_vgsteff_dn12 = assign7740_e8744_d_n12;

        let assign7750_e8747: f64 = if locals.var_exparg > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard601 = assign7750_e8747;

        let (assign7760_e8763, assign7760_e8763_d_n3, assign7760_e8763_d_n4, assign7760_e8763_d_n5, assign7760_e8763_d_n6, assign7760_e8763_d_n7, assign7760_e8763_d_n8, assign7760_e8763_d_n9, assign7760_e8763_d_n10, assign7760_e8763_d_n11, assign7760_e8763_d_n12,) = {
    if (((locals.var_guard594 == 0.0) && (locals.var_guard600 == 0.0)) && (locals.var_guard601 != 0.0)) {
        let assign7760_e8757: f64 = (locals.var_vgst - locals.var_pparam_b4soivoff);
        let assign7760_e8760: f64 = (locals.var_n * locals.var_vtmeot);
        let assign7760_e8761: f64 = (assign7760_e8757 / assign7760_e8760);
        (assign7760_e8761, ((((locals.var_vgst_dn3 - locals.var_pparam_b4soivoff_dn3) * assign7760_e8760) - (assign7760_e8757 * (locals.var_n_dn3 * locals.var_vtmeot))) / (assign7760_e8760 * assign7760_e8760)), ((((locals.var_vgst_dn4 - locals.var_pparam_b4soivoff_dn4) * assign7760_e8760) - (assign7760_e8757 * (locals.var_n_dn4 * locals.var_vtmeot))) / (assign7760_e8760 * assign7760_e8760)), ((((locals.var_vgst_dn5 - locals.var_pparam_b4soivoff_dn5) * assign7760_e8760) - (assign7760_e8757 * (locals.var_n_dn5 * locals.var_vtmeot))) / (assign7760_e8760 * assign7760_e8760)), ((((locals.var_vgst_dn6 - locals.var_pparam_b4soivoff_dn6) * assign7760_e8760) - (assign7760_e8757 * (locals.var_n_dn6 * locals.var_vtmeot))) / (assign7760_e8760 * assign7760_e8760)), ((((locals.var_vgst_dn7 - locals.var_pparam_b4soivoff_dn7) * assign7760_e8760) - (assign7760_e8757 * (locals.var_n_dn7 * locals.var_vtmeot))) / (assign7760_e8760 * assign7760_e8760)), ((((locals.var_vgst_dn8 - locals.var_pparam_b4soivoff_dn8) * assign7760_e8760) - (assign7760_e8757 * (locals.var_n_dn8 * locals.var_vtmeot))) / (assign7760_e8760 * assign7760_e8760)), ((((locals.var_vgst_dn9 - locals.var_pparam_b4soivoff_dn9) * assign7760_e8760) - (assign7760_e8757 * (locals.var_n_dn9 * locals.var_vtmeot))) / (assign7760_e8760 * assign7760_e8760)), ((((locals.var_vgst_dn10 - locals.var_pparam_b4soivoff_dn10) * assign7760_e8760) - (assign7760_e8757 * (locals.var_n_dn10 * locals.var_vtmeot))) / (assign7760_e8760 * assign7760_e8760)), ((((locals.var_vgst_dn11 - locals.var_pparam_b4soivoff_dn11) * assign7760_e8760) - (assign7760_e8757 * (locals.var_n_dn11 * locals.var_vtmeot))) / (assign7760_e8760 * assign7760_e8760)), ((((locals.var_vgst_dn12 - locals.var_pparam_b4soivoff_dn12) * assign7760_e8760) - (assign7760_e8757 * (locals.var_n_dn12 * locals.var_vtmeot))) / (assign7760_e8760 * assign7760_e8760)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign7760_e8763;
        locals.var_t0_dn3 = assign7760_e8763_d_n3;
        locals.var_t0_dn4 = assign7760_e8763_d_n4;
        locals.var_t0_dn5 = assign7760_e8763_d_n5;
        locals.var_t0_dn6 = assign7760_e8763_d_n6;
        locals.var_t0_dn7 = assign7760_e8763_d_n7;
        locals.var_t0_dn8 = assign7760_e8763_d_n8;
        locals.var_t0_dn9 = assign7760_e8763_d_n9;
        locals.var_t0_dn10 = assign7760_e8763_d_n10;
        locals.var_t0_dn11 = assign7760_e8763_d_n11;
        locals.var_t0_dn12 = assign7760_e8763_d_n12;

        let (assign7770_e8774, assign7770_e8774_d_n3, assign7770_e8774_d_n4, assign7770_e8774_d_n5, assign7770_e8774_d_n6, assign7770_e8774_d_n7, assign7770_e8774_d_n8, assign7770_e8774_d_n9, assign7770_e8774_d_n10, assign7770_e8774_d_n11, assign7770_e8774_d_n12,) = {
    if (((locals.var_guard594 == 0.0) && (locals.var_guard600 == 0.0)) && (locals.var_guard601 != 0.0)) {
        let assign7770_e8772: f64 = (locals.var_t0).exp();
        (assign7770_e8772, (assign7770_e8772 * locals.var_t0_dn3), (assign7770_e8772 * locals.var_t0_dn4), (assign7770_e8772 * locals.var_t0_dn5), (assign7770_e8772 * locals.var_t0_dn6), (assign7770_e8772 * locals.var_t0_dn7), (assign7770_e8772 * locals.var_t0_dn8), (assign7770_e8772 * locals.var_t0_dn9), (assign7770_e8772 * locals.var_t0_dn10), (assign7770_e8772 * locals.var_t0_dn11), (assign7770_e8772 * locals.var_t0_dn12),)
    } else {
        (locals.var_expvgst, locals.var_expvgst_dn3, locals.var_expvgst_dn4, locals.var_expvgst_dn5, locals.var_expvgst_dn6, locals.var_expvgst_dn7, locals.var_expvgst_dn8, locals.var_expvgst_dn9, locals.var_expvgst_dn10, locals.var_expvgst_dn11, locals.var_expvgst_dn12,)
    }
};
        locals.var_expvgst = assign7770_e8774;
        locals.var_expvgst_dn3 = assign7770_e8774_d_n3;
        locals.var_expvgst_dn4 = assign7770_e8774_d_n4;
        locals.var_expvgst_dn5 = assign7770_e8774_d_n5;
        locals.var_expvgst_dn6 = assign7770_e8774_d_n6;
        locals.var_expvgst_dn7 = assign7770_e8774_d_n7;
        locals.var_expvgst_dn8 = assign7770_e8774_d_n8;
        locals.var_expvgst_dn9 = assign7770_e8774_d_n9;
        locals.var_expvgst_dn10 = assign7770_e8774_d_n10;
        locals.var_expvgst_dn11 = assign7770_e8774_d_n11;
        locals.var_expvgst_dn12 = assign7770_e8774_d_n12;

        let (assign7780_e8790, assign7780_e8790_d_n3, assign7780_e8790_d_n4, assign7780_e8790_d_n5, assign7780_e8790_d_n6, assign7780_e8790_d_n7, assign7780_e8790_d_n8, assign7780_e8790_d_n9, assign7780_e8790_d_n10, assign7780_e8790_d_n11, assign7780_e8790_d_n12,) = {
    if (((locals.var_guard594 == 0.0) && (locals.var_guard600 == 0.0)) && (locals.var_guard601 != 0.0)) {
        let assign7780_e8784: f64 = (locals.var_vtmeot * locals.var_pparam_b4soicdep0);
        let assign7780_e8786: f64 = (assign7780_e8784 / locals.var_b4soicox);
        let assign7780_e8788: f64 = (assign7780_e8786 * locals.var_expvgst);
        (assign7780_e8788, ((((locals.var_vtmeot * locals.var_pparam_b4soicdep0_dn3) / locals.var_b4soicox) * locals.var_expvgst) + (assign7780_e8786 * locals.var_expvgst_dn3)), ((((locals.var_vtmeot * locals.var_pparam_b4soicdep0_dn4) / locals.var_b4soicox) * locals.var_expvgst) + (assign7780_e8786 * locals.var_expvgst_dn4)), ((((locals.var_vtmeot * locals.var_pparam_b4soicdep0_dn5) / locals.var_b4soicox) * locals.var_expvgst) + (assign7780_e8786 * locals.var_expvgst_dn5)), ((((locals.var_vtmeot * locals.var_pparam_b4soicdep0_dn6) / locals.var_b4soicox) * locals.var_expvgst) + (assign7780_e8786 * locals.var_expvgst_dn6)), ((((locals.var_vtmeot * locals.var_pparam_b4soicdep0_dn7) / locals.var_b4soicox) * locals.var_expvgst) + (assign7780_e8786 * locals.var_expvgst_dn7)), ((((locals.var_vtmeot * locals.var_pparam_b4soicdep0_dn8) / locals.var_b4soicox) * locals.var_expvgst) + (assign7780_e8786 * locals.var_expvgst_dn8)), ((((locals.var_vtmeot * locals.var_pparam_b4soicdep0_dn9) / locals.var_b4soicox) * locals.var_expvgst) + (assign7780_e8786 * locals.var_expvgst_dn9)), ((((locals.var_vtmeot * locals.var_pparam_b4soicdep0_dn10) / locals.var_b4soicox) * locals.var_expvgst) + (assign7780_e8786 * locals.var_expvgst_dn10)), ((((locals.var_vtmeot * locals.var_pparam_b4soicdep0_dn11) / locals.var_b4soicox) * locals.var_expvgst) + (assign7780_e8786 * locals.var_expvgst_dn11)), ((((locals.var_vtmeot * locals.var_pparam_b4soicdep0_dn12) / locals.var_b4soicox) * locals.var_expvgst) + (assign7780_e8786 * locals.var_expvgst_dn12)),)
    } else {
        (locals.var_vgsteff, locals.var_vgsteff_dn3, locals.var_vgsteff_dn4, locals.var_vgsteff_dn5, locals.var_vgsteff_dn6, locals.var_vgsteff_dn7, locals.var_vgsteff_dn8, locals.var_vgsteff_dn9, locals.var_vgsteff_dn10, locals.var_vgsteff_dn11, locals.var_vgsteff_dn12,)
    }
};
        locals.var_vgsteff = assign7780_e8790;
        locals.var_vgsteff_dn3 = assign7780_e8790_d_n3;
        locals.var_vgsteff_dn4 = assign7780_e8790_d_n4;
        locals.var_vgsteff_dn5 = assign7780_e8790_d_n5;
        locals.var_vgsteff_dn6 = assign7780_e8790_d_n6;
        locals.var_vgsteff_dn7 = assign7780_e8790_d_n7;
        locals.var_vgsteff_dn8 = assign7780_e8790_d_n8;
        locals.var_vgsteff_dn9 = assign7780_e8790_d_n9;
        locals.var_vgsteff_dn10 = assign7780_e8790_d_n10;
        locals.var_vgsteff_dn11 = assign7780_e8790_d_n11;
        locals.var_vgsteff_dn12 = assign7780_e8790_d_n12;

        let (assign7790_e8802, assign7790_e8802_d_n3, assign7790_e8802_d_n4, assign7790_e8802_d_n5, assign7790_e8802_d_n6, assign7790_e8802_d_n7, assign7790_e8802_d_n8, assign7790_e8802_d_n9, assign7790_e8802_d_n10, assign7790_e8802_d_n11, assign7790_e8802_d_n12,) = {
    if (((locals.var_guard594 == 0.0) && (locals.var_guard600 == 0.0)) && (locals.var_guard601 == 0.0)) {
        let assign7790_e8800: f64 = (locals.var_vgstnvt).exp();
        (assign7790_e8800, (assign7790_e8800 * locals.var_vgstnvt_dn3), (assign7790_e8800 * locals.var_vgstnvt_dn4), (assign7790_e8800 * locals.var_vgstnvt_dn5), (assign7790_e8800 * locals.var_vgstnvt_dn6), (assign7790_e8800 * locals.var_vgstnvt_dn7), (assign7790_e8800 * locals.var_vgstnvt_dn8), (assign7790_e8800 * locals.var_vgstnvt_dn9), (assign7790_e8800 * locals.var_vgstnvt_dn10), (assign7790_e8800 * locals.var_vgstnvt_dn11), (assign7790_e8800 * locals.var_vgstnvt_dn12),)
    } else {
        (locals.var_expvgst, locals.var_expvgst_dn3, locals.var_expvgst_dn4, locals.var_expvgst_dn5, locals.var_expvgst_dn6, locals.var_expvgst_dn7, locals.var_expvgst_dn8, locals.var_expvgst_dn9, locals.var_expvgst_dn10, locals.var_expvgst_dn11, locals.var_expvgst_dn12,)
    }
};
        locals.var_expvgst = assign7790_e8802;
        locals.var_expvgst_dn3 = assign7790_e8802_d_n3;
        locals.var_expvgst_dn4 = assign7790_e8802_d_n4;
        locals.var_expvgst_dn5 = assign7790_e8802_d_n5;
        locals.var_expvgst_dn6 = assign7790_e8802_d_n6;
        locals.var_expvgst_dn7 = assign7790_e8802_d_n7;
        locals.var_expvgst_dn8 = assign7790_e8802_d_n8;
        locals.var_expvgst_dn9 = assign7790_e8802_d_n9;
        locals.var_expvgst_dn10 = assign7790_e8802_d_n10;
        locals.var_expvgst_dn11 = assign7790_e8802_d_n11;
        locals.var_expvgst_dn12 = assign7790_e8802_d_n12;

        let (assign7800_e8826, assign7800_e8826_d_n3, assign7800_e8826_d_n4, assign7800_e8826_d_n5, assign7800_e8826_d_n6, assign7800_e8826_d_n7, assign7800_e8826_d_n8, assign7800_e8826_d_n9, assign7800_e8826_d_n10, assign7800_e8826_d_n11, assign7800_e8826_d_n12,) = {
    if (((locals.var_guard594 == 0.0) && (locals.var_guard600 == 0.0)) && (locals.var_guard601 == 0.0)) {
        let assign7800_e8814: f64 = (1.0 + locals.var_expvgst);
        let (assign7800_e8823, assign7800_e8823_d_n3, assign7800_e8823_d_n4, assign7800_e8823_d_n5, assign7800_e8823_d_n6, assign7800_e8823_d_n7, assign7800_e8823_d_n8, assign7800_e8823_d_n9, assign7800_e8823_d_n10, assign7800_e8823_d_n11, assign7800_e8823_d_n12,) = {
            if (assign7800_e8814 > 1e-38) {
                let assign7800_e8819: f64 = (1.0 + locals.var_expvgst);
                let assign7800_e8820: f64 = (assign7800_e8819).ln();
                (assign7800_e8820, (locals.var_expvgst_dn3 / assign7800_e8819), (locals.var_expvgst_dn4 / assign7800_e8819), (locals.var_expvgst_dn5 / assign7800_e8819), (locals.var_expvgst_dn6 / assign7800_e8819), (locals.var_expvgst_dn7 / assign7800_e8819), (locals.var_expvgst_dn8 / assign7800_e8819), (locals.var_expvgst_dn9 / assign7800_e8819), (locals.var_expvgst_dn10 / assign7800_e8819), (locals.var_expvgst_dn11 / assign7800_e8819), (locals.var_expvgst_dn12 / assign7800_e8819),)
            } else {
                let assign7800_e8822: f64 = (-87.49823353377374);
                (assign7800_e8822, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign7800_e8824: f64 = (locals.var_t10 * assign7800_e8823);
        (assign7800_e8824, ((locals.var_t10_dn3 * assign7800_e8823) + (locals.var_t10 * assign7800_e8823_d_n3)), ((locals.var_t10_dn4 * assign7800_e8823) + (locals.var_t10 * assign7800_e8823_d_n4)), ((locals.var_t10_dn5 * assign7800_e8823) + (locals.var_t10 * assign7800_e8823_d_n5)), ((locals.var_t10_dn6 * assign7800_e8823) + (locals.var_t10 * assign7800_e8823_d_n6)), ((locals.var_t10_dn7 * assign7800_e8823) + (locals.var_t10 * assign7800_e8823_d_n7)), ((locals.var_t10_dn8 * assign7800_e8823) + (locals.var_t10 * assign7800_e8823_d_n8)), ((locals.var_t10_dn9 * assign7800_e8823) + (locals.var_t10 * assign7800_e8823_d_n9)), ((locals.var_t10_dn10 * assign7800_e8823) + (locals.var_t10 * assign7800_e8823_d_n10)), ((locals.var_t10_dn11 * assign7800_e8823) + (locals.var_t10 * assign7800_e8823_d_n11)), ((locals.var_t10_dn12 * assign7800_e8823) + (locals.var_t10 * assign7800_e8823_d_n12)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign7800_e8826;
        locals.var_t1_dn3 = assign7800_e8826_d_n3;
        locals.var_t1_dn4 = assign7800_e8826_d_n4;
        locals.var_t1_dn5 = assign7800_e8826_d_n5;
        locals.var_t1_dn6 = assign7800_e8826_d_n6;
        locals.var_t1_dn7 = assign7800_e8826_d_n7;
        locals.var_t1_dn8 = assign7800_e8826_d_n8;
        locals.var_t1_dn9 = assign7800_e8826_d_n9;
        locals.var_t1_dn10 = assign7800_e8826_d_n10;
        locals.var_t1_dn11 = assign7800_e8826_d_n11;
        locals.var_t1_dn12 = assign7800_e8826_d_n12;

        let (assign7810_e8849, assign7810_e8849_d_n3, assign7810_e8849_d_n4, assign7810_e8849_d_n5, assign7810_e8849_d_n6, assign7810_e8849_d_n7, assign7810_e8849_d_n8, assign7810_e8849_d_n9, assign7810_e8849_d_n10, assign7810_e8849_d_n11, assign7810_e8849_d_n12,) = {
    if (((locals.var_guard594 == 0.0) && (locals.var_guard600 == 0.0)) && (locals.var_guard601 == 0.0)) {
        let assign7810_e8836: f64 = (-locals.var_b4soicox);
        let assign7810_e8839: f64 = (locals.var_vtm0eot * locals.var_pparam_b4soicdep0);
        let assign7810_e8840: f64 = (assign7810_e8836 / assign7810_e8839);
        let assign7810_e8842: f64 = (locals.var_exparg).exp();
        let assign7810_e8843: f64 = (assign7810_e8840 * assign7810_e8842);
        let assign7810_e8846: f64 = (1.0 - locals.var_pparam_b4soimstar);
        let assign7810_e8847: f64 = (assign7810_e8843 * assign7810_e8846);
        (assign7810_e8847, (((((-((assign7810_e8836 * (locals.var_vtm0eot * locals.var_pparam_b4soicdep0_dn3)) / (assign7810_e8839 * assign7810_e8839))) * assign7810_e8842) + (assign7810_e8840 * (assign7810_e8842 * locals.var_exparg_dn3))) * assign7810_e8846) + (assign7810_e8843 * (-locals.var_pparam_b4soimstar_dn3))), (((((-((assign7810_e8836 * (locals.var_vtm0eot * locals.var_pparam_b4soicdep0_dn4)) / (assign7810_e8839 * assign7810_e8839))) * assign7810_e8842) + (assign7810_e8840 * (assign7810_e8842 * locals.var_exparg_dn4))) * assign7810_e8846) + (assign7810_e8843 * (-locals.var_pparam_b4soimstar_dn4))), (((((-((assign7810_e8836 * (locals.var_vtm0eot * locals.var_pparam_b4soicdep0_dn5)) / (assign7810_e8839 * assign7810_e8839))) * assign7810_e8842) + (assign7810_e8840 * (assign7810_e8842 * locals.var_exparg_dn5))) * assign7810_e8846) + (assign7810_e8843 * (-locals.var_pparam_b4soimstar_dn5))), (((((-((assign7810_e8836 * (locals.var_vtm0eot * locals.var_pparam_b4soicdep0_dn6)) / (assign7810_e8839 * assign7810_e8839))) * assign7810_e8842) + (assign7810_e8840 * (assign7810_e8842 * locals.var_exparg_dn6))) * assign7810_e8846) + (assign7810_e8843 * (-locals.var_pparam_b4soimstar_dn6))), (((((-((assign7810_e8836 * (locals.var_vtm0eot * locals.var_pparam_b4soicdep0_dn7)) / (assign7810_e8839 * assign7810_e8839))) * assign7810_e8842) + (assign7810_e8840 * (assign7810_e8842 * locals.var_exparg_dn7))) * assign7810_e8846) + (assign7810_e8843 * (-locals.var_pparam_b4soimstar_dn7))), (((((-((assign7810_e8836 * (locals.var_vtm0eot * locals.var_pparam_b4soicdep0_dn8)) / (assign7810_e8839 * assign7810_e8839))) * assign7810_e8842) + (assign7810_e8840 * (assign7810_e8842 * locals.var_exparg_dn8))) * assign7810_e8846) + (assign7810_e8843 * (-locals.var_pparam_b4soimstar_dn8))), (((((-((assign7810_e8836 * (locals.var_vtm0eot * locals.var_pparam_b4soicdep0_dn9)) / (assign7810_e8839 * assign7810_e8839))) * assign7810_e8842) + (assign7810_e8840 * (assign7810_e8842 * locals.var_exparg_dn9))) * assign7810_e8846) + (assign7810_e8843 * (-locals.var_pparam_b4soimstar_dn9))), (((((-((assign7810_e8836 * (locals.var_vtm0eot * locals.var_pparam_b4soicdep0_dn10)) / (assign7810_e8839 * assign7810_e8839))) * assign7810_e8842) + (assign7810_e8840 * (assign7810_e8842 * locals.var_exparg_dn10))) * assign7810_e8846) + (assign7810_e8843 * (-locals.var_pparam_b4soimstar_dn10))), (((((-((assign7810_e8836 * (locals.var_vtm0eot * locals.var_pparam_b4soicdep0_dn11)) / (assign7810_e8839 * assign7810_e8839))) * assign7810_e8842) + (assign7810_e8840 * (assign7810_e8842 * locals.var_exparg_dn11))) * assign7810_e8846) + (assign7810_e8843 * (-locals.var_pparam_b4soimstar_dn11))), (((((-((assign7810_e8836 * (locals.var_vtm0eot * locals.var_pparam_b4soicdep0_dn12)) / (assign7810_e8839 * assign7810_e8839))) * assign7810_e8842) + (assign7810_e8840 * (assign7810_e8842 * locals.var_exparg_dn12))) * assign7810_e8846) + (assign7810_e8843 * (-locals.var_pparam_b4soimstar_dn12))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign7810_e8849;
        locals.var_t4_dn3 = assign7810_e8849_d_n3;
        locals.var_t4_dn4 = assign7810_e8849_d_n4;
        locals.var_t4_dn5 = assign7810_e8849_d_n5;
        locals.var_t4_dn6 = assign7810_e8849_d_n6;
        locals.var_t4_dn7 = assign7810_e8849_d_n7;
        locals.var_t4_dn8 = assign7810_e8849_d_n8;
        locals.var_t4_dn9 = assign7810_e8849_d_n9;
        locals.var_t4_dn10 = assign7810_e8849_d_n10;
        locals.var_t4_dn11 = assign7810_e8849_d_n11;
        locals.var_t4_dn12 = assign7810_e8849_d_n12;

        let (assign7820_e8868, assign7820_e8868_d_n3, assign7820_e8868_d_n4, assign7820_e8868_d_n5, assign7820_e8868_d_n6, assign7820_e8868_d_n7, assign7820_e8868_d_n8, assign7820_e8868_d_n9, assign7820_e8868_d_n10, assign7820_e8868_d_n11, assign7820_e8868_d_n12,) = {
    if (((locals.var_guard594 == 0.0) && (locals.var_guard600 == 0.0)) && (locals.var_guard601 == 0.0)) {
        let assign7820_e8861: f64 = (locals.var_t10 * locals.var_t4);
        let assign7820_e8864: f64 = (1.0 - locals.var_pparam_b4soimstar);
        let assign7820_e8865: f64 = (assign7820_e8861 / assign7820_e8864);
        let assign7820_e8866: f64 = (locals.var_pparam_b4soimstar - assign7820_e8865);
        (assign7820_e8866, (locals.var_pparam_b4soimstar_dn3 - (((((locals.var_t10_dn3 * locals.var_t4) + (locals.var_t10 * locals.var_t4_dn3)) * assign7820_e8864) - (assign7820_e8861 * (-locals.var_pparam_b4soimstar_dn3))) / (assign7820_e8864 * assign7820_e8864))), (locals.var_pparam_b4soimstar_dn4 - (((((locals.var_t10_dn4 * locals.var_t4) + (locals.var_t10 * locals.var_t4_dn4)) * assign7820_e8864) - (assign7820_e8861 * (-locals.var_pparam_b4soimstar_dn4))) / (assign7820_e8864 * assign7820_e8864))), (locals.var_pparam_b4soimstar_dn5 - (((((locals.var_t10_dn5 * locals.var_t4) + (locals.var_t10 * locals.var_t4_dn5)) * assign7820_e8864) - (assign7820_e8861 * (-locals.var_pparam_b4soimstar_dn5))) / (assign7820_e8864 * assign7820_e8864))), (locals.var_pparam_b4soimstar_dn6 - (((((locals.var_t10_dn6 * locals.var_t4) + (locals.var_t10 * locals.var_t4_dn6)) * assign7820_e8864) - (assign7820_e8861 * (-locals.var_pparam_b4soimstar_dn6))) / (assign7820_e8864 * assign7820_e8864))), (locals.var_pparam_b4soimstar_dn7 - (((((locals.var_t10_dn7 * locals.var_t4) + (locals.var_t10 * locals.var_t4_dn7)) * assign7820_e8864) - (assign7820_e8861 * (-locals.var_pparam_b4soimstar_dn7))) / (assign7820_e8864 * assign7820_e8864))), (locals.var_pparam_b4soimstar_dn8 - (((((locals.var_t10_dn8 * locals.var_t4) + (locals.var_t10 * locals.var_t4_dn8)) * assign7820_e8864) - (assign7820_e8861 * (-locals.var_pparam_b4soimstar_dn8))) / (assign7820_e8864 * assign7820_e8864))), (locals.var_pparam_b4soimstar_dn9 - (((((locals.var_t10_dn9 * locals.var_t4) + (locals.var_t10 * locals.var_t4_dn9)) * assign7820_e8864) - (assign7820_e8861 * (-locals.var_pparam_b4soimstar_dn9))) / (assign7820_e8864 * assign7820_e8864))), (locals.var_pparam_b4soimstar_dn10 - (((((locals.var_t10_dn10 * locals.var_t4) + (locals.var_t10 * locals.var_t4_dn10)) * assign7820_e8864) - (assign7820_e8861 * (-locals.var_pparam_b4soimstar_dn10))) / (assign7820_e8864 * assign7820_e8864))), (locals.var_pparam_b4soimstar_dn11 - (((((locals.var_t10_dn11 * locals.var_t4) + (locals.var_t10 * locals.var_t4_dn11)) * assign7820_e8864) - (assign7820_e8861 * (-locals.var_pparam_b4soimstar_dn11))) / (assign7820_e8864 * assign7820_e8864))), (locals.var_pparam_b4soimstar_dn12 - (((((locals.var_t10_dn12 * locals.var_t4) + (locals.var_t10 * locals.var_t4_dn12)) * assign7820_e8864) - (assign7820_e8861 * (-locals.var_pparam_b4soimstar_dn12))) / (assign7820_e8864 * assign7820_e8864))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign7820_e8868;
        locals.var_t2_dn3 = assign7820_e8868_d_n3;
        locals.var_t2_dn4 = assign7820_e8868_d_n4;
        locals.var_t2_dn5 = assign7820_e8868_d_n5;
        locals.var_t2_dn6 = assign7820_e8868_d_n6;
        locals.var_t2_dn7 = assign7820_e8868_d_n7;
        locals.var_t2_dn8 = assign7820_e8868_d_n8;
        locals.var_t2_dn9 = assign7820_e8868_d_n9;
        locals.var_t2_dn10 = assign7820_e8868_d_n10;
        locals.var_t2_dn11 = assign7820_e8868_d_n11;
        locals.var_t2_dn12 = assign7820_e8868_d_n12;

        let (assign7830_e8881, assign7830_e8881_d_n3, assign7830_e8881_d_n4, assign7830_e8881_d_n5, assign7830_e8881_d_n6, assign7830_e8881_d_n7, assign7830_e8881_d_n8, assign7830_e8881_d_n9, assign7830_e8881_d_n10, assign7830_e8881_d_n11, assign7830_e8881_d_n12,) = {
    if (((locals.var_guard594 == 0.0) && (locals.var_guard600 == 0.0)) && (locals.var_guard601 == 0.0)) {
        let assign7830_e8879: f64 = (locals.var_t1 / locals.var_t2);
        (assign7830_e8879, (((locals.var_t1_dn3 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn3)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn12 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn12)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_vgsteff, locals.var_vgsteff_dn3, locals.var_vgsteff_dn4, locals.var_vgsteff_dn5, locals.var_vgsteff_dn6, locals.var_vgsteff_dn7, locals.var_vgsteff_dn8, locals.var_vgsteff_dn9, locals.var_vgsteff_dn10, locals.var_vgsteff_dn11, locals.var_vgsteff_dn12,)
    }
};
        locals.var_vgsteff = assign7830_e8881;
        locals.var_vgsteff_dn3 = assign7830_e8881_d_n3;
        locals.var_vgsteff_dn4 = assign7830_e8881_d_n4;
        locals.var_vgsteff_dn5 = assign7830_e8881_d_n5;
        locals.var_vgsteff_dn6 = assign7830_e8881_d_n6;
        locals.var_vgsteff_dn7 = assign7830_e8881_d_n7;
        locals.var_vgsteff_dn8 = assign7830_e8881_d_n8;
        locals.var_vgsteff_dn9 = assign7830_e8881_d_n9;
        locals.var_vgsteff_dn10 = assign7830_e8881_d_n10;
        locals.var_vgsteff_dn11 = assign7830_e8881_d_n11;
        locals.var_vgsteff_dn12 = assign7830_e8881_d_n12;

        let (assign7840_e8892, assign7840_e8892_d_n3, assign7840_e8892_d_n4, assign7840_e8892_d_n5, assign7840_e8892_d_n6, assign7840_e8892_d_n7, assign7840_e8892_d_n8, assign7840_e8892_d_n9, assign7840_e8892_d_n10, assign7840_e8892_d_n11, assign7840_e8892_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7840_e8886: f64 = (p.p37 * locals.var_here_b4soivth0);
        let assign7840_e8888: f64 = (assign7840_e8886 - locals.var_here_b4soivfb);
        let assign7840_e8890: f64 = (assign7840_e8888 - locals.var_phieot);
        (assign7840_e8890, (((p.p37 * locals.var_here_b4soivth0_dn3) - locals.var_here_b4soivfb_dn3) - locals.var_phieot_dn3), (((p.p37 * locals.var_here_b4soivth0_dn4) - locals.var_here_b4soivfb_dn4) - locals.var_phieot_dn4), (((p.p37 * locals.var_here_b4soivth0_dn5) - locals.var_here_b4soivfb_dn5) - locals.var_phieot_dn5), (((p.p37 * locals.var_here_b4soivth0_dn6) - locals.var_here_b4soivfb_dn6) - locals.var_phieot_dn6), (((p.p37 * locals.var_here_b4soivth0_dn7) - locals.var_here_b4soivfb_dn7) - locals.var_phieot_dn7), (((p.p37 * locals.var_here_b4soivth0_dn8) - locals.var_here_b4soivfb_dn8) - locals.var_phieot_dn8), (((p.p37 * locals.var_here_b4soivth0_dn9) - locals.var_here_b4soivfb_dn9) - locals.var_phieot_dn9), (((p.p37 * locals.var_here_b4soivth0_dn10) - locals.var_here_b4soivfb_dn10) - locals.var_phieot_dn10), (((p.p37 * locals.var_here_b4soivth0_dn11) - locals.var_here_b4soivfb_dn11) - locals.var_phieot_dn11), (((p.p37 * locals.var_here_b4soivth0_dn12) - locals.var_here_b4soivfb_dn12) - locals.var_phieot_dn12),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign7840_e8892;
        locals.var_t3_dn3 = assign7840_e8892_d_n3;
        locals.var_t3_dn4 = assign7840_e8892_d_n4;
        locals.var_t3_dn5 = assign7840_e8892_d_n5;
        locals.var_t3_dn6 = assign7840_e8892_d_n6;
        locals.var_t3_dn7 = assign7840_e8892_d_n7;
        locals.var_t3_dn8 = assign7840_e8892_d_n8;
        locals.var_t3_dn9 = assign7840_e8892_d_n9;
        locals.var_t3_dn10 = assign7840_e8892_d_n10;
        locals.var_t3_dn11 = assign7840_e8892_d_n11;
        locals.var_t3_dn12 = assign7840_e8892_d_n12;

        let (assign7850_e8899, assign7850_e8899_d_n3, assign7850_e8899_d_n4, assign7850_e8899_d_n5, assign7850_e8899_d_n6, assign7850_e8899_d_n7, assign7850_e8899_d_n8, assign7850_e8899_d_n9, assign7850_e8899_d_n10, assign7850_e8899_d_n11, assign7850_e8899_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7850_e8897: f64 = (4.0 * locals.var_t3);
        (assign7850_e8897, (4.0 * locals.var_t3_dn3), (4.0 * locals.var_t3_dn4), (4.0 * locals.var_t3_dn5), (4.0 * locals.var_t3_dn6), (4.0 * locals.var_t3_dn7), (4.0 * locals.var_t3_dn8), (4.0 * locals.var_t3_dn9), (4.0 * locals.var_t3_dn10), (4.0 * locals.var_t3_dn11), (4.0 * locals.var_t3_dn12),)
    } else {
        (locals.var_vtfbphi2eot, locals.var_vtfbphi2eot_dn3, locals.var_vtfbphi2eot_dn4, locals.var_vtfbphi2eot_dn5, locals.var_vtfbphi2eot_dn6, locals.var_vtfbphi2eot_dn7, locals.var_vtfbphi2eot_dn8, locals.var_vtfbphi2eot_dn9, locals.var_vtfbphi2eot_dn10, locals.var_vtfbphi2eot_dn11, locals.var_vtfbphi2eot_dn12,)
    }
};
        locals.var_vtfbphi2eot = assign7850_e8899;
        locals.var_vtfbphi2eot_dn3 = assign7850_e8899_d_n3;
        locals.var_vtfbphi2eot_dn4 = assign7850_e8899_d_n4;
        locals.var_vtfbphi2eot_dn5 = assign7850_e8899_d_n5;
        locals.var_vtfbphi2eot_dn6 = assign7850_e8899_d_n6;
        locals.var_vtfbphi2eot_dn7 = assign7850_e8899_d_n7;
        locals.var_vtfbphi2eot_dn8 = assign7850_e8899_d_n8;
        locals.var_vtfbphi2eot_dn9 = assign7850_e8899_d_n9;
        locals.var_vtfbphi2eot_dn10 = assign7850_e8899_d_n10;
        locals.var_vtfbphi2eot_dn11 = assign7850_e8899_d_n11;
        locals.var_vtfbphi2eot_dn12 = assign7850_e8899_d_n12;

        let assign7860_e8902: f64 = if locals.var_vtfbphi2eot < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard602 = assign7860_e8902;

        let (assign7870_e8909, assign7870_e8909_d_n3, assign7870_e8909_d_n4, assign7870_e8909_d_n5, assign7870_e8909_d_n6, assign7870_e8909_d_n7, assign7870_e8909_d_n8, assign7870_e8909_d_n9, assign7870_e8909_d_n10, assign7870_e8909_d_n11, assign7870_e8909_d_n12,) = {
    if ((locals.var_guard594 == 0.0) && (locals.var_guard602 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vtfbphi2eot, locals.var_vtfbphi2eot_dn3, locals.var_vtfbphi2eot_dn4, locals.var_vtfbphi2eot_dn5, locals.var_vtfbphi2eot_dn6, locals.var_vtfbphi2eot_dn7, locals.var_vtfbphi2eot_dn8, locals.var_vtfbphi2eot_dn9, locals.var_vtfbphi2eot_dn10, locals.var_vtfbphi2eot_dn11, locals.var_vtfbphi2eot_dn12,)
    }
};
        locals.var_vtfbphi2eot = assign7870_e8909;
        locals.var_vtfbphi2eot_dn3 = assign7870_e8909_d_n3;
        locals.var_vtfbphi2eot_dn4 = assign7870_e8909_d_n4;
        locals.var_vtfbphi2eot_dn5 = assign7870_e8909_d_n5;
        locals.var_vtfbphi2eot_dn6 = assign7870_e8909_d_n6;
        locals.var_vtfbphi2eot_dn7 = assign7870_e8909_d_n7;
        locals.var_vtfbphi2eot_dn8 = assign7870_e8909_d_n8;
        locals.var_vtfbphi2eot_dn9 = assign7870_e8909_d_n9;
        locals.var_vtfbphi2eot_dn10 = assign7870_e8909_d_n10;
        locals.var_vtfbphi2eot_dn11 = assign7870_e8909_d_n11;
        locals.var_vtfbphi2eot_dn12 = assign7870_e8909_d_n12;

        let (assign7880_e8914,) = {
    if (locals.var_guard594 == 0.0) {
        (0.0,)
    } else {
        (locals.var_niter,)
    }
};
        locals.var_niter = assign7880_e8914;

        let (assign7890_e8919, assign7890_e8919_d_n3, assign7890_e8919_d_n4, assign7890_e8919_d_n5, assign7890_e8919_d_n6, assign7890_e8919_d_n7, assign7890_e8919_d_n8, assign7890_e8919_d_n9, assign7890_e8919_d_n10, assign7890_e8919_d_n11, assign7890_e8919_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        (locals.var_toxe, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_toxpf, locals.var_toxpf_dn3, locals.var_toxpf_dn4, locals.var_toxpf_dn5, locals.var_toxpf_dn6, locals.var_toxpf_dn7, locals.var_toxpf_dn8, locals.var_toxpf_dn9, locals.var_toxpf_dn10, locals.var_toxpf_dn11, locals.var_toxpf_dn12,)
    }
};
        locals.var_toxpf = assign7890_e8919;
        locals.var_toxpf_dn3 = assign7890_e8919_d_n3;
        locals.var_toxpf_dn4 = assign7890_e8919_d_n4;
        locals.var_toxpf_dn5 = assign7890_e8919_d_n5;
        locals.var_toxpf_dn6 = assign7890_e8919_d_n6;
        locals.var_toxpf_dn7 = assign7890_e8919_d_n7;
        locals.var_toxpf_dn8 = assign7890_e8919_d_n8;
        locals.var_toxpf_dn9 = assign7890_e8919_d_n9;
        locals.var_toxpf_dn10 = assign7890_e8919_d_n10;
        locals.var_toxpf_dn11 = assign7890_e8919_d_n11;
        locals.var_toxpf_dn12 = assign7890_e8919_d_n12;

        let (assign7900_e8924,) = {
    if (locals.var_guard594 == 0.0) {
        (1000000.0,)
    } else {
        (locals.var_toxpi,)
    }
};
        locals.var_toxpi = assign7900_e8924;

    }

    pub(super) fn stamp_transient_block_23(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let mut assign7910_loop_guard: usize = 0;
        while {
            let assign7910_cond_e8932: f64 = (locals.var_toxpf - locals.var_toxpi);
            let assign7910_cond_e8933: f64 = (assign7910_cond_e8932).abs();
            let assign7910_cond_e8933_d_n3: f64 = if assign7910_cond_e8932 >= 0.0 { locals.var_toxpf_dn3 } else { (-locals.var_toxpf_dn3) };
            let assign7910_cond_e8933_d_n4: f64 = if assign7910_cond_e8932 >= 0.0 { locals.var_toxpf_dn4 } else { (-locals.var_toxpf_dn4) };
            let assign7910_cond_e8933_d_n5: f64 = if assign7910_cond_e8932 >= 0.0 { locals.var_toxpf_dn5 } else { (-locals.var_toxpf_dn5) };
            let assign7910_cond_e8933_d_n6: f64 = if assign7910_cond_e8932 >= 0.0 { locals.var_toxpf_dn6 } else { (-locals.var_toxpf_dn6) };
            let assign7910_cond_e8933_d_n7: f64 = if assign7910_cond_e8932 >= 0.0 { locals.var_toxpf_dn7 } else { (-locals.var_toxpf_dn7) };
            let assign7910_cond_e8933_d_n8: f64 = if assign7910_cond_e8932 >= 0.0 { locals.var_toxpf_dn8 } else { (-locals.var_toxpf_dn8) };
            let assign7910_cond_e8933_d_n9: f64 = if assign7910_cond_e8932 >= 0.0 { locals.var_toxpf_dn9 } else { (-locals.var_toxpf_dn9) };
            let assign7910_cond_e8933_d_n10: f64 = if assign7910_cond_e8932 >= 0.0 { locals.var_toxpf_dn10 } else { (-locals.var_toxpf_dn10) };
            let assign7910_cond_e8933_d_n11: f64 = if assign7910_cond_e8932 >= 0.0 { locals.var_toxpf_dn11 } else { (-locals.var_toxpf_dn11) };
            let assign7910_cond_e8933_d_n12: f64 = if assign7910_cond_e8932 >= 0.0 { locals.var_toxpf_dn12 } else { (-locals.var_toxpf_dn12) };
            let assign7910_cond_e8937: f64 = if ((locals.var_guard594 == 0.0) && ((locals.var_niter <= 4.0) && (assign7910_cond_e8933 > 1e-12))) { 1.0 } else { 0.0 };
            assign7910_cond_e8937 != 0.0
        } {
            assign7910_loop_guard += 1;
            assert!(assign7910_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign7910_body0_e8942,) = {
    if (locals.var_guard594 == 0.0) {
        (locals.var_toxpf,)
    } else {
        (locals.var_toxpi,)
    }
};
            locals.var_toxpi = assign7910_body0_e8942;
            let (assign7910_body1_e8949, assign7910_body1_e8949_d_n3, assign7910_body1_e8949_d_n4, assign7910_body1_e8949_d_n5, assign7910_body1_e8949_d_n6, assign7910_body1_e8949_d_n7, assign7910_body1_e8949_d_n8, assign7910_body1_e8949_d_n9, assign7910_body1_e8949_d_n10, assign7910_body1_e8949_d_n11, assign7910_body1_e8949_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7910_body1_e8947: f64 = (200000000.0 * locals.var_toxpf);
        (assign7910_body1_e8947, (200000000.0 * locals.var_toxpf_dn3), (200000000.0 * locals.var_toxpf_dn4), (200000000.0 * locals.var_toxpf_dn5), (200000000.0 * locals.var_toxpf_dn6), (200000000.0 * locals.var_toxpf_dn7), (200000000.0 * locals.var_toxpf_dn8), (200000000.0 * locals.var_toxpf_dn9), (200000000.0 * locals.var_toxpf_dn10), (200000000.0 * locals.var_toxpf_dn11), (200000000.0 * locals.var_toxpf_dn12),)
    } else {
        (locals.var_tmp2, locals.var_tmp2_dn3, locals.var_tmp2_dn4, locals.var_tmp2_dn5, locals.var_tmp2_dn6, locals.var_tmp2_dn7, locals.var_tmp2_dn8, locals.var_tmp2_dn9, locals.var_tmp2_dn10, locals.var_tmp2_dn11, locals.var_tmp2_dn12,)
    }
};
            locals.var_tmp2 = assign7910_body1_e8949;
            locals.var_tmp2_dn3 = assign7910_body1_e8949_d_n3;
            locals.var_tmp2_dn4 = assign7910_body1_e8949_d_n4;
            locals.var_tmp2_dn5 = assign7910_body1_e8949_d_n5;
            locals.var_tmp2_dn6 = assign7910_body1_e8949_d_n6;
            locals.var_tmp2_dn7 = assign7910_body1_e8949_d_n7;
            locals.var_tmp2_dn8 = assign7910_body1_e8949_d_n8;
            locals.var_tmp2_dn9 = assign7910_body1_e8949_d_n9;
            locals.var_tmp2_dn10 = assign7910_body1_e8949_d_n10;
            locals.var_tmp2_dn11 = assign7910_body1_e8949_d_n11;
            locals.var_tmp2_dn12 = assign7910_body1_e8949_d_n12;
            let (assign7910_body2_e8958, assign7910_body2_e8958_d_n3, assign7910_body2_e8958_d_n4, assign7910_body2_e8958_d_n5, assign7910_body2_e8958_d_n6, assign7910_body2_e8958_d_n7, assign7910_body2_e8958_d_n8, assign7910_body2_e8958_d_n9, assign7910_body2_e8958_d_n10, assign7910_body2_e8958_d_n11, assign7910_body2_e8958_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7910_body2_e8954: f64 = (locals.var_vgsteff + locals.var_vtfbphi2eot);
        let assign7910_body2_e8956: f64 = (assign7910_body2_e8954 / locals.var_tmp2);
        (assign7910_body2_e8956, ((((locals.var_vgsteff_dn3 + locals.var_vtfbphi2eot_dn3) * locals.var_tmp2) - (assign7910_body2_e8954 * locals.var_tmp2_dn3)) / (locals.var_tmp2 * locals.var_tmp2)), ((((locals.var_vgsteff_dn4 + locals.var_vtfbphi2eot_dn4) * locals.var_tmp2) - (assign7910_body2_e8954 * locals.var_tmp2_dn4)) / (locals.var_tmp2 * locals.var_tmp2)), ((((locals.var_vgsteff_dn5 + locals.var_vtfbphi2eot_dn5) * locals.var_tmp2) - (assign7910_body2_e8954 * locals.var_tmp2_dn5)) / (locals.var_tmp2 * locals.var_tmp2)), ((((locals.var_vgsteff_dn6 + locals.var_vtfbphi2eot_dn6) * locals.var_tmp2) - (assign7910_body2_e8954 * locals.var_tmp2_dn6)) / (locals.var_tmp2 * locals.var_tmp2)), ((((locals.var_vgsteff_dn7 + locals.var_vtfbphi2eot_dn7) * locals.var_tmp2) - (assign7910_body2_e8954 * locals.var_tmp2_dn7)) / (locals.var_tmp2 * locals.var_tmp2)), ((((locals.var_vgsteff_dn8 + locals.var_vtfbphi2eot_dn8) * locals.var_tmp2) - (assign7910_body2_e8954 * locals.var_tmp2_dn8)) / (locals.var_tmp2 * locals.var_tmp2)), ((((locals.var_vgsteff_dn9 + locals.var_vtfbphi2eot_dn9) * locals.var_tmp2) - (assign7910_body2_e8954 * locals.var_tmp2_dn9)) / (locals.var_tmp2 * locals.var_tmp2)), ((((locals.var_vgsteff_dn10 + locals.var_vtfbphi2eot_dn10) * locals.var_tmp2) - (assign7910_body2_e8954 * locals.var_tmp2_dn10)) / (locals.var_tmp2 * locals.var_tmp2)), ((((locals.var_vgsteff_dn11 + locals.var_vtfbphi2eot_dn11) * locals.var_tmp2) - (assign7910_body2_e8954 * locals.var_tmp2_dn11)) / (locals.var_tmp2 * locals.var_tmp2)), ((((locals.var_vgsteff_dn12 + locals.var_vtfbphi2eot_dn12) * locals.var_tmp2) - (assign7910_body2_e8954 * locals.var_tmp2_dn12)) / (locals.var_tmp2 * locals.var_tmp2)),)
    } else {
        (locals.var_t0__blk603, locals.var_t0__blk603_dn3, locals.var_t0__blk603_dn4, locals.var_t0__blk603_dn5, locals.var_t0__blk603_dn6, locals.var_t0__blk603_dn7, locals.var_t0__blk603_dn8, locals.var_t0__blk603_dn9, locals.var_t0__blk603_dn10, locals.var_t0__blk603_dn11, locals.var_t0__blk603_dn12,)
    }
};
            locals.var_t0__blk603 = assign7910_body2_e8958;
            locals.var_t0__blk603_dn3 = assign7910_body2_e8958_d_n3;
            locals.var_t0__blk603_dn4 = assign7910_body2_e8958_d_n4;
            locals.var_t0__blk603_dn5 = assign7910_body2_e8958_d_n5;
            locals.var_t0__blk603_dn6 = assign7910_body2_e8958_d_n6;
            locals.var_t0__blk603_dn7 = assign7910_body2_e8958_d_n7;
            locals.var_t0__blk603_dn8 = assign7910_body2_e8958_d_n8;
            locals.var_t0__blk603_dn9 = assign7910_body2_e8958_d_n9;
            locals.var_t0__blk603_dn10 = assign7910_body2_e8958_d_n10;
            locals.var_t0__blk603_dn11 = assign7910_body2_e8958_d_n11;
            locals.var_t0__blk603_dn12 = assign7910_body2_e8958_d_n12;
            let (assign7910_body3_e8977, assign7910_body3_e8977_d_n3, assign7910_body3_e8977_d_n4, assign7910_body3_e8977_d_n5, assign7910_body3_e8977_d_n6, assign7910_body3_e8977_d_n7, assign7910_body3_e8977_d_n8, assign7910_body3_e8977_d_n9, assign7910_body3_e8977_d_n10, assign7910_body3_e8977_d_n11, assign7910_body3_e8977_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7910_body3_e8964: f64 = (p.p59 * 0.7);
        let (assign7910_body3_e8972, assign7910_body3_e8972_d_n3, assign7910_body3_e8972_d_n4, assign7910_body3_e8972_d_n5, assign7910_body3_e8972_d_n6, assign7910_body3_e8972_d_n7, assign7910_body3_e8972_d_n8, assign7910_body3_e8972_d_n9, assign7910_body3_e8972_d_n10, assign7910_body3_e8972_d_n11, assign7910_body3_e8972_d_n12,) = {
            if (locals.var_t0__blk603 > 1e-38) {
                let assign7910_body3_e8969: f64 = (locals.var_t0__blk603).ln();
                (assign7910_body3_e8969, (locals.var_t0__blk603_dn3 / locals.var_t0__blk603), (locals.var_t0__blk603_dn4 / locals.var_t0__blk603), (locals.var_t0__blk603_dn5 / locals.var_t0__blk603), (locals.var_t0__blk603_dn6 / locals.var_t0__blk603), (locals.var_t0__blk603_dn7 / locals.var_t0__blk603), (locals.var_t0__blk603_dn8 / locals.var_t0__blk603), (locals.var_t0__blk603_dn9 / locals.var_t0__blk603), (locals.var_t0__blk603_dn10 / locals.var_t0__blk603), (locals.var_t0__blk603_dn11 / locals.var_t0__blk603), (locals.var_t0__blk603_dn12 / locals.var_t0__blk603),)
            } else {
                let assign7910_body3_e8971: f64 = (-87.49823353377374);
                (assign7910_body3_e8971, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign7910_body3_e8973: f64 = (assign7910_body3_e8964 * assign7910_body3_e8972);
        let assign7910_body3_e8974: f64 = (assign7910_body3_e8973).exp();
        let assign7910_body3_e8975: f64 = (1.0 + assign7910_body3_e8974);
        (assign7910_body3_e8975, (assign7910_body3_e8974 * (assign7910_body3_e8964 * assign7910_body3_e8972_d_n3)), (assign7910_body3_e8974 * (assign7910_body3_e8964 * assign7910_body3_e8972_d_n4)), (assign7910_body3_e8974 * (assign7910_body3_e8964 * assign7910_body3_e8972_d_n5)), (assign7910_body3_e8974 * (assign7910_body3_e8964 * assign7910_body3_e8972_d_n6)), (assign7910_body3_e8974 * (assign7910_body3_e8964 * assign7910_body3_e8972_d_n7)), (assign7910_body3_e8974 * (assign7910_body3_e8964 * assign7910_body3_e8972_d_n8)), (assign7910_body3_e8974 * (assign7910_body3_e8964 * assign7910_body3_e8972_d_n9)), (assign7910_body3_e8974 * (assign7910_body3_e8964 * assign7910_body3_e8972_d_n10)), (assign7910_body3_e8974 * (assign7910_body3_e8964 * assign7910_body3_e8972_d_n11)), (assign7910_body3_e8974 * (assign7910_body3_e8964 * assign7910_body3_e8972_d_n12)),)
    } else {
        (locals.var_t1__blk604, locals.var_t1__blk604_dn3, locals.var_t1__blk604_dn4, locals.var_t1__blk604_dn5, locals.var_t1__blk604_dn6, locals.var_t1__blk604_dn7, locals.var_t1__blk604_dn8, locals.var_t1__blk604_dn9, locals.var_t1__blk604_dn10, locals.var_t1__blk604_dn11, locals.var_t1__blk604_dn12,)
    }
};
            locals.var_t1__blk604 = assign7910_body3_e8977;
            locals.var_t1__blk604_dn3 = assign7910_body3_e8977_d_n3;
            locals.var_t1__blk604_dn4 = assign7910_body3_e8977_d_n4;
            locals.var_t1__blk604_dn5 = assign7910_body3_e8977_d_n5;
            locals.var_t1__blk604_dn6 = assign7910_body3_e8977_d_n6;
            locals.var_t1__blk604_dn7 = assign7910_body3_e8977_d_n7;
            locals.var_t1__blk604_dn8 = assign7910_body3_e8977_d_n8;
            locals.var_t1__blk604_dn9 = assign7910_body3_e8977_d_n9;
            locals.var_t1__blk604_dn10 = assign7910_body3_e8977_d_n10;
            locals.var_t1__blk604_dn11 = assign7910_body3_e8977_d_n11;
            locals.var_t1__blk604_dn12 = assign7910_body3_e8977_d_n12;
            let (assign7910_body4_e8986, assign7910_body4_e8986_d_n3, assign7910_body4_e8986_d_n4, assign7910_body4_e8986_d_n5, assign7910_body4_e8986_d_n6, assign7910_body4_e8986_d_n7, assign7910_body4_e8986_d_n8, assign7910_body4_e8986_d_n9, assign7910_body4_e8986_d_n10, assign7910_body4_e8986_d_n11, assign7910_body4_e8986_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7910_body4_e8982: f64 = (p.p58 * 1.9e-9);
        let assign7910_body4_e8984: f64 = (assign7910_body4_e8982 / locals.var_t1__blk604);
        (assign7910_body4_e8984, (-((assign7910_body4_e8982 * locals.var_t1__blk604_dn3) / (locals.var_t1__blk604 * locals.var_t1__blk604))), (-((assign7910_body4_e8982 * locals.var_t1__blk604_dn4) / (locals.var_t1__blk604 * locals.var_t1__blk604))), (-((assign7910_body4_e8982 * locals.var_t1__blk604_dn5) / (locals.var_t1__blk604 * locals.var_t1__blk604))), (-((assign7910_body4_e8982 * locals.var_t1__blk604_dn6) / (locals.var_t1__blk604 * locals.var_t1__blk604))), (-((assign7910_body4_e8982 * locals.var_t1__blk604_dn7) / (locals.var_t1__blk604 * locals.var_t1__blk604))), (-((assign7910_body4_e8982 * locals.var_t1__blk604_dn8) / (locals.var_t1__blk604 * locals.var_t1__blk604))), (-((assign7910_body4_e8982 * locals.var_t1__blk604_dn9) / (locals.var_t1__blk604 * locals.var_t1__blk604))), (-((assign7910_body4_e8982 * locals.var_t1__blk604_dn10) / (locals.var_t1__blk604 * locals.var_t1__blk604))), (-((assign7910_body4_e8982 * locals.var_t1__blk604_dn11) / (locals.var_t1__blk604 * locals.var_t1__blk604))), (-((assign7910_body4_e8982 * locals.var_t1__blk604_dn12) / (locals.var_t1__blk604 * locals.var_t1__blk604))),)
    } else {
        (locals.var_tcen, locals.var_tcen_dn3, locals.var_tcen_dn4, locals.var_tcen_dn5, locals.var_tcen_dn6, locals.var_tcen_dn7, locals.var_tcen_dn8, locals.var_tcen_dn9, locals.var_tcen_dn10, locals.var_tcen_dn11, locals.var_tcen_dn12,)
    }
};
            locals.var_tcen = assign7910_body4_e8986;
            locals.var_tcen_dn3 = assign7910_body4_e8986_d_n3;
            locals.var_tcen_dn4 = assign7910_body4_e8986_d_n4;
            locals.var_tcen_dn5 = assign7910_body4_e8986_d_n5;
            locals.var_tcen_dn6 = assign7910_body4_e8986_d_n6;
            locals.var_tcen_dn7 = assign7910_body4_e8986_d_n7;
            locals.var_tcen_dn8 = assign7910_body4_e8986_d_n8;
            locals.var_tcen_dn9 = assign7910_body4_e8986_d_n9;
            locals.var_tcen_dn10 = assign7910_body4_e8986_d_n10;
            locals.var_tcen_dn11 = assign7910_body4_e8986_d_n11;
            locals.var_tcen_dn12 = assign7910_body4_e8986_d_n12;
            let (assign7910_body5_e8997, assign7910_body5_e8997_d_n3, assign7910_body5_e8997_d_n4, assign7910_body5_e8997_d_n5, assign7910_body5_e8997_d_n6, assign7910_body5_e8997_d_n7, assign7910_body5_e8997_d_n8, assign7910_body5_e8997_d_n9, assign7910_body5_e8997_d_n10, assign7910_body5_e8997_d_n11, assign7910_body5_e8997_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7910_body5_e8992: f64 = (locals.var_epsrox / p.p47);
        let assign7910_body5_e8994: f64 = (assign7910_body5_e8992 * locals.var_tcen);
        let assign7910_body5_e8995: f64 = (locals.var_toxe - assign7910_body5_e8994);
        (assign7910_body5_e8995, (-(assign7910_body5_e8992 * locals.var_tcen_dn3)), (-(assign7910_body5_e8992 * locals.var_tcen_dn4)), (-(assign7910_body5_e8992 * locals.var_tcen_dn5)), (-(assign7910_body5_e8992 * locals.var_tcen_dn6)), (-(assign7910_body5_e8992 * locals.var_tcen_dn7)), (-(assign7910_body5_e8992 * locals.var_tcen_dn8)), (-(assign7910_body5_e8992 * locals.var_tcen_dn9)), (-(assign7910_body5_e8992 * locals.var_tcen_dn10)), (-(assign7910_body5_e8992 * locals.var_tcen_dn11)), (-(assign7910_body5_e8992 * locals.var_tcen_dn12)),)
    } else {
        (locals.var_toxpf, locals.var_toxpf_dn3, locals.var_toxpf_dn4, locals.var_toxpf_dn5, locals.var_toxpf_dn6, locals.var_toxpf_dn7, locals.var_toxpf_dn8, locals.var_toxpf_dn9, locals.var_toxpf_dn10, locals.var_toxpf_dn11, locals.var_toxpf_dn12,)
    }
};
            locals.var_toxpf = assign7910_body5_e8997;
            locals.var_toxpf_dn3 = assign7910_body5_e8997_d_n3;
            locals.var_toxpf_dn4 = assign7910_body5_e8997_d_n4;
            locals.var_toxpf_dn5 = assign7910_body5_e8997_d_n5;
            locals.var_toxpf_dn6 = assign7910_body5_e8997_d_n6;
            locals.var_toxpf_dn7 = assign7910_body5_e8997_d_n7;
            locals.var_toxpf_dn8 = assign7910_body5_e8997_d_n8;
            locals.var_toxpf_dn9 = assign7910_body5_e8997_d_n9;
            locals.var_toxpf_dn10 = assign7910_body5_e8997_d_n10;
            locals.var_toxpf_dn11 = assign7910_body5_e8997_d_n11;
            locals.var_toxpf_dn12 = assign7910_body5_e8997_d_n12;
            let (assign7910_body6_e9004,) = {
    if (locals.var_guard594 == 0.0) {
        let assign7910_body6_e9002: f64 = (locals.var_niter + 1.0);
        (assign7910_body6_e9002,)
    } else {
        (locals.var_niter,)
    }
};
            locals.var_niter = assign7910_body6_e9004;
        }

        let (assign7920_e9009, assign7920_e9009_d_n3, assign7920_e9009_d_n4, assign7920_e9009_d_n5, assign7920_e9009_d_n6, assign7920_e9009_d_n7, assign7920_e9009_d_n8, assign7920_e9009_d_n9, assign7920_e9009_d_n10, assign7920_e9009_d_n11, assign7920_e9009_d_n12,) = {
    if (locals.var_guard594 == 0.0) {
        (locals.var_toxpf, locals.var_toxpf_dn3, locals.var_toxpf_dn4, locals.var_toxpf_dn5, locals.var_toxpf_dn6, locals.var_toxpf_dn7, locals.var_toxpf_dn8, locals.var_toxpf_dn9, locals.var_toxpf_dn10, locals.var_toxpf_dn11, locals.var_toxpf_dn12,)
    } else {
        (locals.var_b4soitoxp, locals.var_b4soitoxp_dn3, locals.var_b4soitoxp_dn4, locals.var_b4soitoxp_dn5, locals.var_b4soitoxp_dn6, locals.var_b4soitoxp_dn7, locals.var_b4soitoxp_dn8, locals.var_b4soitoxp_dn9, locals.var_b4soitoxp_dn10, locals.var_b4soitoxp_dn11, locals.var_b4soitoxp_dn12,)
    }
};
        locals.var_b4soitoxp = assign7920_e9009;
        locals.var_b4soitoxp_dn3 = assign7920_e9009_d_n3;
        locals.var_b4soitoxp_dn4 = assign7920_e9009_d_n4;
        locals.var_b4soitoxp_dn5 = assign7920_e9009_d_n5;
        locals.var_b4soitoxp_dn6 = assign7920_e9009_d_n6;
        locals.var_b4soitoxp_dn7 = assign7920_e9009_d_n7;
        locals.var_b4soitoxp_dn8 = assign7920_e9009_d_n8;
        locals.var_b4soitoxp_dn9 = assign7920_e9009_d_n9;
        locals.var_b4soitoxp_dn10 = assign7920_e9009_d_n10;
        locals.var_b4soitoxp_dn11 = assign7920_e9009_d_n11;
        locals.var_b4soitoxp_dn12 = assign7920_e9009_d_n12;

        locals.var_tmp = locals.var_pparam_b4soisqrtxdep0;
        locals.var_tmp_dn3 = locals.var_pparam_b4soisqrtxdep0_dn3;
        locals.var_tmp_dn4 = locals.var_pparam_b4soisqrtxdep0_dn4;
        locals.var_tmp_dn5 = locals.var_pparam_b4soisqrtxdep0_dn5;
        locals.var_tmp_dn6 = locals.var_pparam_b4soisqrtxdep0_dn6;
        locals.var_tmp_dn7 = locals.var_pparam_b4soisqrtxdep0_dn7;
        locals.var_tmp_dn8 = locals.var_pparam_b4soisqrtxdep0_dn8;
        locals.var_tmp_dn9 = locals.var_pparam_b4soisqrtxdep0_dn9;
        locals.var_tmp_dn10 = locals.var_pparam_b4soisqrtxdep0_dn10;
        locals.var_tmp_dn11 = locals.var_pparam_b4soisqrtxdep0_dn11;
        locals.var_tmp_dn12 = locals.var_pparam_b4soisqrtxdep0_dn12;

        let assign7940_e9013: f64 = (locals.var_pparam_b4soivbi - locals.var_pparam_b4soiphi);
        locals.var_tmp1 = assign7940_e9013;
        locals.var_tmp1_dn3 = (locals.var_pparam_b4soivbi_dn3 - locals.var_pparam_b4soiphi_dn3);
        locals.var_tmp1_dn4 = (locals.var_pparam_b4soivbi_dn4 - locals.var_pparam_b4soiphi_dn4);
        locals.var_tmp1_dn5 = (locals.var_pparam_b4soivbi_dn5 - locals.var_pparam_b4soiphi_dn5);
        locals.var_tmp1_dn6 = (locals.var_pparam_b4soivbi_dn6 - locals.var_pparam_b4soiphi_dn6);
        locals.var_tmp1_dn7 = (locals.var_pparam_b4soivbi_dn7 - locals.var_pparam_b4soiphi_dn7);
        locals.var_tmp1_dn8 = (locals.var_pparam_b4soivbi_dn8 - locals.var_pparam_b4soiphi_dn8);
        locals.var_tmp1_dn9 = (locals.var_pparam_b4soivbi_dn9 - locals.var_pparam_b4soiphi_dn9);
        locals.var_tmp1_dn10 = (locals.var_pparam_b4soivbi_dn10 - locals.var_pparam_b4soiphi_dn10);
        locals.var_tmp1_dn11 = (locals.var_pparam_b4soivbi_dn11 - locals.var_pparam_b4soiphi_dn11);
        locals.var_tmp1_dn12 = (locals.var_pparam_b4soivbi_dn12 - locals.var_pparam_b4soiphi_dn12);

        let assign7950_e9016: f64 = (locals.var_b4soifactor1 * locals.var_tmp);
        locals.var_tmp2 = assign7950_e9016;
        locals.var_tmp2_dn3 = (locals.var_b4soifactor1 * locals.var_tmp_dn3);
        locals.var_tmp2_dn4 = (locals.var_b4soifactor1 * locals.var_tmp_dn4);
        locals.var_tmp2_dn5 = (locals.var_b4soifactor1 * locals.var_tmp_dn5);
        locals.var_tmp2_dn6 = (locals.var_b4soifactor1 * locals.var_tmp_dn6);
        locals.var_tmp2_dn7 = (locals.var_b4soifactor1 * locals.var_tmp_dn7);
        locals.var_tmp2_dn8 = (locals.var_b4soifactor1 * locals.var_tmp_dn8);
        locals.var_tmp2_dn9 = (locals.var_b4soifactor1 * locals.var_tmp_dn9);
        locals.var_tmp2_dn10 = (locals.var_b4soifactor1 * locals.var_tmp_dn10);
        locals.var_tmp2_dn11 = (locals.var_b4soifactor1 * locals.var_tmp_dn11);
        locals.var_tmp2_dn12 = (locals.var_b4soifactor1 * locals.var_tmp_dn12);

        let assign7960_e9018: f64 = (-0.5);
        let assign7960_e9020: f64 = (assign7960_e9018 * locals.var_pparam_b4soidvt1w);
        let assign7960_e9022: f64 = (assign7960_e9020 * locals.var_pparam_b4soiweff);
        let assign7960_e9024: f64 = (assign7960_e9022 * locals.var_pparam_b4soileff);
        let assign7960_e9026: f64 = (assign7960_e9024 / locals.var_tmp2);
        locals.var_t0 = assign7960_e9026;
        locals.var_t0_dn3 = ((((((((assign7960_e9018 * locals.var_pparam_b4soidvt1w_dn3) * locals.var_pparam_b4soiweff) + (assign7960_e9020 * locals.var_pparam_b4soiweff_dn3)) * locals.var_pparam_b4soileff) + (assign7960_e9022 * locals.var_pparam_b4soileff_dn3)) * locals.var_tmp2) - (assign7960_e9024 * locals.var_tmp2_dn3)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn4 = ((((((((assign7960_e9018 * locals.var_pparam_b4soidvt1w_dn4) * locals.var_pparam_b4soiweff) + (assign7960_e9020 * locals.var_pparam_b4soiweff_dn4)) * locals.var_pparam_b4soileff) + (assign7960_e9022 * locals.var_pparam_b4soileff_dn4)) * locals.var_tmp2) - (assign7960_e9024 * locals.var_tmp2_dn4)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn5 = ((((((((assign7960_e9018 * locals.var_pparam_b4soidvt1w_dn5) * locals.var_pparam_b4soiweff) + (assign7960_e9020 * locals.var_pparam_b4soiweff_dn5)) * locals.var_pparam_b4soileff) + (assign7960_e9022 * locals.var_pparam_b4soileff_dn5)) * locals.var_tmp2) - (assign7960_e9024 * locals.var_tmp2_dn5)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn6 = ((((((((assign7960_e9018 * locals.var_pparam_b4soidvt1w_dn6) * locals.var_pparam_b4soiweff) + (assign7960_e9020 * locals.var_pparam_b4soiweff_dn6)) * locals.var_pparam_b4soileff) + (assign7960_e9022 * locals.var_pparam_b4soileff_dn6)) * locals.var_tmp2) - (assign7960_e9024 * locals.var_tmp2_dn6)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn7 = ((((((((assign7960_e9018 * locals.var_pparam_b4soidvt1w_dn7) * locals.var_pparam_b4soiweff) + (assign7960_e9020 * locals.var_pparam_b4soiweff_dn7)) * locals.var_pparam_b4soileff) + (assign7960_e9022 * locals.var_pparam_b4soileff_dn7)) * locals.var_tmp2) - (assign7960_e9024 * locals.var_tmp2_dn7)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn8 = ((((((((assign7960_e9018 * locals.var_pparam_b4soidvt1w_dn8) * locals.var_pparam_b4soiweff) + (assign7960_e9020 * locals.var_pparam_b4soiweff_dn8)) * locals.var_pparam_b4soileff) + (assign7960_e9022 * locals.var_pparam_b4soileff_dn8)) * locals.var_tmp2) - (assign7960_e9024 * locals.var_tmp2_dn8)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn9 = ((((((((assign7960_e9018 * locals.var_pparam_b4soidvt1w_dn9) * locals.var_pparam_b4soiweff) + (assign7960_e9020 * locals.var_pparam_b4soiweff_dn9)) * locals.var_pparam_b4soileff) + (assign7960_e9022 * locals.var_pparam_b4soileff_dn9)) * locals.var_tmp2) - (assign7960_e9024 * locals.var_tmp2_dn9)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn10 = ((((((((assign7960_e9018 * locals.var_pparam_b4soidvt1w_dn10) * locals.var_pparam_b4soiweff) + (assign7960_e9020 * locals.var_pparam_b4soiweff_dn10)) * locals.var_pparam_b4soileff) + (assign7960_e9022 * locals.var_pparam_b4soileff_dn10)) * locals.var_tmp2) - (assign7960_e9024 * locals.var_tmp2_dn10)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn11 = ((((((((assign7960_e9018 * locals.var_pparam_b4soidvt1w_dn11) * locals.var_pparam_b4soiweff) + (assign7960_e9020 * locals.var_pparam_b4soiweff_dn11)) * locals.var_pparam_b4soileff) + (assign7960_e9022 * locals.var_pparam_b4soileff_dn11)) * locals.var_tmp2) - (assign7960_e9024 * locals.var_tmp2_dn11)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn12 = ((((((((assign7960_e9018 * locals.var_pparam_b4soidvt1w_dn12) * locals.var_pparam_b4soiweff) + (assign7960_e9020 * locals.var_pparam_b4soiweff_dn12)) * locals.var_pparam_b4soileff) + (assign7960_e9022 * locals.var_pparam_b4soileff_dn12)) * locals.var_tmp2) - (assign7960_e9024 * locals.var_tmp2_dn12)) / (locals.var_tmp2 * locals.var_tmp2));

        let assign7970_e9029: f64 = (-100.0);
        let assign7970_e9030: f64 = if locals.var_t0 > assign7970_e9029 { 1.0 } else { 0.0 };
        locals.var_guard605 = assign7970_e9030;

        let (assign7980_e9035, assign7980_e9035_d_n3, assign7980_e9035_d_n4, assign7980_e9035_d_n5, assign7980_e9035_d_n6, assign7980_e9035_d_n7, assign7980_e9035_d_n8, assign7980_e9035_d_n9, assign7980_e9035_d_n10, assign7980_e9035_d_n11, assign7980_e9035_d_n12,) = {
    if (locals.var_guard605 != 0.0) {
        let assign7980_e9033: f64 = (locals.var_t0).exp();
        (assign7980_e9033, (assign7980_e9033 * locals.var_t0_dn3), (assign7980_e9033 * locals.var_t0_dn4), (assign7980_e9033 * locals.var_t0_dn5), (assign7980_e9033 * locals.var_t0_dn6), (assign7980_e9033 * locals.var_t0_dn7), (assign7980_e9033 * locals.var_t0_dn8), (assign7980_e9033 * locals.var_t0_dn9), (assign7980_e9033 * locals.var_t0_dn10), (assign7980_e9033 * locals.var_t0_dn11), (assign7980_e9033 * locals.var_t0_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign7980_e9035;
        locals.var_t1_dn3 = assign7980_e9035_d_n3;
        locals.var_t1_dn4 = assign7980_e9035_d_n4;
        locals.var_t1_dn5 = assign7980_e9035_d_n5;
        locals.var_t1_dn6 = assign7980_e9035_d_n6;
        locals.var_t1_dn7 = assign7980_e9035_d_n7;
        locals.var_t1_dn8 = assign7980_e9035_d_n8;
        locals.var_t1_dn9 = assign7980_e9035_d_n9;
        locals.var_t1_dn10 = assign7980_e9035_d_n10;
        locals.var_t1_dn11 = assign7980_e9035_d_n11;
        locals.var_t1_dn12 = assign7980_e9035_d_n12;

        let (assign7990_e9045, assign7990_e9045_d_n3, assign7990_e9045_d_n4, assign7990_e9045_d_n5, assign7990_e9045_d_n6, assign7990_e9045_d_n7, assign7990_e9045_d_n8, assign7990_e9045_d_n9, assign7990_e9045_d_n10, assign7990_e9045_d_n11, assign7990_e9045_d_n12,) = {
    if (locals.var_guard605 != 0.0) {
        let assign7990_e9041: f64 = (2.0 * locals.var_t1);
        let assign7990_e9042: f64 = (1.0 + assign7990_e9041);
        let assign7990_e9043: f64 = (locals.var_t1 * assign7990_e9042);
        (assign7990_e9043, ((locals.var_t1_dn3 * assign7990_e9042) + (locals.var_t1 * (2.0 * locals.var_t1_dn3))), ((locals.var_t1_dn4 * assign7990_e9042) + (locals.var_t1 * (2.0 * locals.var_t1_dn4))), ((locals.var_t1_dn5 * assign7990_e9042) + (locals.var_t1 * (2.0 * locals.var_t1_dn5))), ((locals.var_t1_dn6 * assign7990_e9042) + (locals.var_t1 * (2.0 * locals.var_t1_dn6))), ((locals.var_t1_dn7 * assign7990_e9042) + (locals.var_t1 * (2.0 * locals.var_t1_dn7))), ((locals.var_t1_dn8 * assign7990_e9042) + (locals.var_t1 * (2.0 * locals.var_t1_dn8))), ((locals.var_t1_dn9 * assign7990_e9042) + (locals.var_t1 * (2.0 * locals.var_t1_dn9))), ((locals.var_t1_dn10 * assign7990_e9042) + (locals.var_t1 * (2.0 * locals.var_t1_dn10))), ((locals.var_t1_dn11 * assign7990_e9042) + (locals.var_t1 * (2.0 * locals.var_t1_dn11))), ((locals.var_t1_dn12 * assign7990_e9042) + (locals.var_t1 * (2.0 * locals.var_t1_dn12))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign7990_e9045;
        locals.var_t2_dn3 = assign7990_e9045_d_n3;
        locals.var_t2_dn4 = assign7990_e9045_d_n4;
        locals.var_t2_dn5 = assign7990_e9045_d_n5;
        locals.var_t2_dn6 = assign7990_e9045_d_n6;
        locals.var_t2_dn7 = assign7990_e9045_d_n7;
        locals.var_t2_dn8 = assign7990_e9045_d_n8;
        locals.var_t2_dn9 = assign7990_e9045_d_n9;
        locals.var_t2_dn10 = assign7990_e9045_d_n10;
        locals.var_t2_dn11 = assign7990_e9045_d_n11;
        locals.var_t2_dn12 = assign7990_e9045_d_n12;

        let (assign8000_e9050, assign8000_e9050_d_n3, assign8000_e9050_d_n4, assign8000_e9050_d_n5, assign8000_e9050_d_n6, assign8000_e9050_d_n7, assign8000_e9050_d_n8, assign8000_e9050_d_n9, assign8000_e9050_d_n10, assign8000_e9050_d_n11, assign8000_e9050_d_n12,) = {
    if (locals.var_guard605 == 0.0) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign8000_e9050;
        locals.var_t1_dn3 = assign8000_e9050_d_n3;
        locals.var_t1_dn4 = assign8000_e9050_d_n4;
        locals.var_t1_dn5 = assign8000_e9050_d_n5;
        locals.var_t1_dn6 = assign8000_e9050_d_n6;
        locals.var_t1_dn7 = assign8000_e9050_d_n7;
        locals.var_t1_dn8 = assign8000_e9050_d_n8;
        locals.var_t1_dn9 = assign8000_e9050_d_n9;
        locals.var_t1_dn10 = assign8000_e9050_d_n10;
        locals.var_t1_dn11 = assign8000_e9050_d_n11;
        locals.var_t1_dn12 = assign8000_e9050_d_n12;

        let (assign8010_e9061, assign8010_e9061_d_n3, assign8010_e9061_d_n4, assign8010_e9061_d_n5, assign8010_e9061_d_n6, assign8010_e9061_d_n7, assign8010_e9061_d_n8, assign8010_e9061_d_n9, assign8010_e9061_d_n10, assign8010_e9061_d_n11, assign8010_e9061_d_n12,) = {
    if (locals.var_guard605 == 0.0) {
        let assign8010_e9057: f64 = (2.0 * locals.var_t1);
        let assign8010_e9058: f64 = (1.0 + assign8010_e9057);
        let assign8010_e9059: f64 = (locals.var_t1 * assign8010_e9058);
        (assign8010_e9059, ((locals.var_t1_dn3 * assign8010_e9058) + (locals.var_t1 * (2.0 * locals.var_t1_dn3))), ((locals.var_t1_dn4 * assign8010_e9058) + (locals.var_t1 * (2.0 * locals.var_t1_dn4))), ((locals.var_t1_dn5 * assign8010_e9058) + (locals.var_t1 * (2.0 * locals.var_t1_dn5))), ((locals.var_t1_dn6 * assign8010_e9058) + (locals.var_t1 * (2.0 * locals.var_t1_dn6))), ((locals.var_t1_dn7 * assign8010_e9058) + (locals.var_t1 * (2.0 * locals.var_t1_dn7))), ((locals.var_t1_dn8 * assign8010_e9058) + (locals.var_t1 * (2.0 * locals.var_t1_dn8))), ((locals.var_t1_dn9 * assign8010_e9058) + (locals.var_t1 * (2.0 * locals.var_t1_dn9))), ((locals.var_t1_dn10 * assign8010_e9058) + (locals.var_t1 * (2.0 * locals.var_t1_dn10))), ((locals.var_t1_dn11 * assign8010_e9058) + (locals.var_t1 * (2.0 * locals.var_t1_dn11))), ((locals.var_t1_dn12 * assign8010_e9058) + (locals.var_t1 * (2.0 * locals.var_t1_dn12))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign8010_e9061;
        locals.var_t2_dn3 = assign8010_e9061_d_n3;
        locals.var_t2_dn4 = assign8010_e9061_d_n4;
        locals.var_t2_dn5 = assign8010_e9061_d_n5;
        locals.var_t2_dn6 = assign8010_e9061_d_n6;
        locals.var_t2_dn7 = assign8010_e9061_d_n7;
        locals.var_t2_dn8 = assign8010_e9061_d_n8;
        locals.var_t2_dn9 = assign8010_e9061_d_n9;
        locals.var_t2_dn10 = assign8010_e9061_d_n10;
        locals.var_t2_dn11 = assign8010_e9061_d_n11;
        locals.var_t2_dn12 = assign8010_e9061_d_n12;

        let assign8020_e9064: f64 = (locals.var_pparam_b4soidvt0w * locals.var_t2);
        locals.var_t0 = assign8020_e9064;
        locals.var_t0_dn3 = ((locals.var_pparam_b4soidvt0w_dn3 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn3));
        locals.var_t0_dn4 = ((locals.var_pparam_b4soidvt0w_dn4 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn4));
        locals.var_t0_dn5 = ((locals.var_pparam_b4soidvt0w_dn5 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn5));
        locals.var_t0_dn6 = ((locals.var_pparam_b4soidvt0w_dn6 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn6));
        locals.var_t0_dn7 = ((locals.var_pparam_b4soidvt0w_dn7 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn7));
        locals.var_t0_dn8 = ((locals.var_pparam_b4soidvt0w_dn8 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn8));
        locals.var_t0_dn9 = ((locals.var_pparam_b4soidvt0w_dn9 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn9));
        locals.var_t0_dn10 = ((locals.var_pparam_b4soidvt0w_dn10 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn10));
        locals.var_t0_dn11 = ((locals.var_pparam_b4soidvt0w_dn11 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn11));
        locals.var_t0_dn12 = ((locals.var_pparam_b4soidvt0w_dn12 * locals.var_t2) + (locals.var_pparam_b4soidvt0w * locals.var_t2_dn12));

        let assign8030_e9067: f64 = (locals.var_t0 * locals.var_tmp1);
        locals.var_t2 = assign8030_e9067;
        locals.var_t2_dn3 = ((locals.var_t0_dn3 * locals.var_tmp1) + (locals.var_t0 * locals.var_tmp1_dn3));
        locals.var_t2_dn4 = ((locals.var_t0_dn4 * locals.var_tmp1) + (locals.var_t0 * locals.var_tmp1_dn4));
        locals.var_t2_dn5 = ((locals.var_t0_dn5 * locals.var_tmp1) + (locals.var_t0 * locals.var_tmp1_dn5));
        locals.var_t2_dn6 = ((locals.var_t0_dn6 * locals.var_tmp1) + (locals.var_t0 * locals.var_tmp1_dn6));
        locals.var_t2_dn7 = ((locals.var_t0_dn7 * locals.var_tmp1) + (locals.var_t0 * locals.var_tmp1_dn7));
        locals.var_t2_dn8 = ((locals.var_t0_dn8 * locals.var_tmp1) + (locals.var_t0 * locals.var_tmp1_dn8));
        locals.var_t2_dn9 = ((locals.var_t0_dn9 * locals.var_tmp1) + (locals.var_t0 * locals.var_tmp1_dn9));
        locals.var_t2_dn10 = ((locals.var_t0_dn10 * locals.var_tmp1) + (locals.var_t0 * locals.var_tmp1_dn10));
        locals.var_t2_dn11 = ((locals.var_t0_dn11 * locals.var_tmp1) + (locals.var_t0 * locals.var_tmp1_dn11));
        locals.var_t2_dn12 = ((locals.var_t0_dn12 * locals.var_tmp1) + (locals.var_t0 * locals.var_tmp1_dn12));

        let assign8040_e9069: f64 = (-0.5);
        let assign8040_e9071: f64 = (assign8040_e9069 * locals.var_pparam_b4soidvt1);
        let assign8040_e9073: f64 = (assign8040_e9071 * locals.var_pparam_b4soileff);
        let assign8040_e9075: f64 = (assign8040_e9073 / locals.var_tmp2);
        locals.var_t0 = assign8040_e9075;
        locals.var_t0_dn3 = ((((((assign8040_e9069 * locals.var_pparam_b4soidvt1_dn3) * locals.var_pparam_b4soileff) + (assign8040_e9071 * locals.var_pparam_b4soileff_dn3)) * locals.var_tmp2) - (assign8040_e9073 * locals.var_tmp2_dn3)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn4 = ((((((assign8040_e9069 * locals.var_pparam_b4soidvt1_dn4) * locals.var_pparam_b4soileff) + (assign8040_e9071 * locals.var_pparam_b4soileff_dn4)) * locals.var_tmp2) - (assign8040_e9073 * locals.var_tmp2_dn4)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn5 = ((((((assign8040_e9069 * locals.var_pparam_b4soidvt1_dn5) * locals.var_pparam_b4soileff) + (assign8040_e9071 * locals.var_pparam_b4soileff_dn5)) * locals.var_tmp2) - (assign8040_e9073 * locals.var_tmp2_dn5)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn6 = ((((((assign8040_e9069 * locals.var_pparam_b4soidvt1_dn6) * locals.var_pparam_b4soileff) + (assign8040_e9071 * locals.var_pparam_b4soileff_dn6)) * locals.var_tmp2) - (assign8040_e9073 * locals.var_tmp2_dn6)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn7 = ((((((assign8040_e9069 * locals.var_pparam_b4soidvt1_dn7) * locals.var_pparam_b4soileff) + (assign8040_e9071 * locals.var_pparam_b4soileff_dn7)) * locals.var_tmp2) - (assign8040_e9073 * locals.var_tmp2_dn7)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn8 = ((((((assign8040_e9069 * locals.var_pparam_b4soidvt1_dn8) * locals.var_pparam_b4soileff) + (assign8040_e9071 * locals.var_pparam_b4soileff_dn8)) * locals.var_tmp2) - (assign8040_e9073 * locals.var_tmp2_dn8)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn9 = ((((((assign8040_e9069 * locals.var_pparam_b4soidvt1_dn9) * locals.var_pparam_b4soileff) + (assign8040_e9071 * locals.var_pparam_b4soileff_dn9)) * locals.var_tmp2) - (assign8040_e9073 * locals.var_tmp2_dn9)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn10 = ((((((assign8040_e9069 * locals.var_pparam_b4soidvt1_dn10) * locals.var_pparam_b4soileff) + (assign8040_e9071 * locals.var_pparam_b4soileff_dn10)) * locals.var_tmp2) - (assign8040_e9073 * locals.var_tmp2_dn10)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn11 = ((((((assign8040_e9069 * locals.var_pparam_b4soidvt1_dn11) * locals.var_pparam_b4soileff) + (assign8040_e9071 * locals.var_pparam_b4soileff_dn11)) * locals.var_tmp2) - (assign8040_e9073 * locals.var_tmp2_dn11)) / (locals.var_tmp2 * locals.var_tmp2));
        locals.var_t0_dn12 = ((((((assign8040_e9069 * locals.var_pparam_b4soidvt1_dn12) * locals.var_pparam_b4soileff) + (assign8040_e9071 * locals.var_pparam_b4soileff_dn12)) * locals.var_tmp2) - (assign8040_e9073 * locals.var_tmp2_dn12)) / (locals.var_tmp2 * locals.var_tmp2));

        let assign8050_e9078: f64 = (-100.0);
        let assign8050_e9079: f64 = if locals.var_t0 > assign8050_e9078 { 1.0 } else { 0.0 };
        locals.var_guard606 = assign8050_e9079;

        let (assign8060_e9084, assign8060_e9084_d_n3, assign8060_e9084_d_n4, assign8060_e9084_d_n5, assign8060_e9084_d_n6, assign8060_e9084_d_n7, assign8060_e9084_d_n8, assign8060_e9084_d_n9, assign8060_e9084_d_n10, assign8060_e9084_d_n11, assign8060_e9084_d_n12,) = {
    if (locals.var_guard606 != 0.0) {
        let assign8060_e9082: f64 = (locals.var_t0).exp();
        (assign8060_e9082, (assign8060_e9082 * locals.var_t0_dn3), (assign8060_e9082 * locals.var_t0_dn4), (assign8060_e9082 * locals.var_t0_dn5), (assign8060_e9082 * locals.var_t0_dn6), (assign8060_e9082 * locals.var_t0_dn7), (assign8060_e9082 * locals.var_t0_dn8), (assign8060_e9082 * locals.var_t0_dn9), (assign8060_e9082 * locals.var_t0_dn10), (assign8060_e9082 * locals.var_t0_dn11), (assign8060_e9082 * locals.var_t0_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign8060_e9084;
        locals.var_t1_dn3 = assign8060_e9084_d_n3;
        locals.var_t1_dn4 = assign8060_e9084_d_n4;
        locals.var_t1_dn5 = assign8060_e9084_d_n5;
        locals.var_t1_dn6 = assign8060_e9084_d_n6;
        locals.var_t1_dn7 = assign8060_e9084_d_n7;
        locals.var_t1_dn8 = assign8060_e9084_d_n8;
        locals.var_t1_dn9 = assign8060_e9084_d_n9;
        locals.var_t1_dn10 = assign8060_e9084_d_n10;
        locals.var_t1_dn11 = assign8060_e9084_d_n11;
        locals.var_t1_dn12 = assign8060_e9084_d_n12;

        let (assign8070_e9094, assign8070_e9094_d_n3, assign8070_e9094_d_n4, assign8070_e9094_d_n5, assign8070_e9094_d_n6, assign8070_e9094_d_n7, assign8070_e9094_d_n8, assign8070_e9094_d_n9, assign8070_e9094_d_n10, assign8070_e9094_d_n11, assign8070_e9094_d_n12,) = {
    if (locals.var_guard606 != 0.0) {
        let assign8070_e9090: f64 = (2.0 * locals.var_t1);
        let assign8070_e9091: f64 = (1.0 + assign8070_e9090);
        let assign8070_e9092: f64 = (locals.var_t1 * assign8070_e9091);
        (assign8070_e9092, ((locals.var_t1_dn3 * assign8070_e9091) + (locals.var_t1 * (2.0 * locals.var_t1_dn3))), ((locals.var_t1_dn4 * assign8070_e9091) + (locals.var_t1 * (2.0 * locals.var_t1_dn4))), ((locals.var_t1_dn5 * assign8070_e9091) + (locals.var_t1 * (2.0 * locals.var_t1_dn5))), ((locals.var_t1_dn6 * assign8070_e9091) + (locals.var_t1 * (2.0 * locals.var_t1_dn6))), ((locals.var_t1_dn7 * assign8070_e9091) + (locals.var_t1 * (2.0 * locals.var_t1_dn7))), ((locals.var_t1_dn8 * assign8070_e9091) + (locals.var_t1 * (2.0 * locals.var_t1_dn8))), ((locals.var_t1_dn9 * assign8070_e9091) + (locals.var_t1 * (2.0 * locals.var_t1_dn9))), ((locals.var_t1_dn10 * assign8070_e9091) + (locals.var_t1 * (2.0 * locals.var_t1_dn10))), ((locals.var_t1_dn11 * assign8070_e9091) + (locals.var_t1 * (2.0 * locals.var_t1_dn11))), ((locals.var_t1_dn12 * assign8070_e9091) + (locals.var_t1 * (2.0 * locals.var_t1_dn12))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign8070_e9094;
        locals.var_t3_dn3 = assign8070_e9094_d_n3;
        locals.var_t3_dn4 = assign8070_e9094_d_n4;
        locals.var_t3_dn5 = assign8070_e9094_d_n5;
        locals.var_t3_dn6 = assign8070_e9094_d_n6;
        locals.var_t3_dn7 = assign8070_e9094_d_n7;
        locals.var_t3_dn8 = assign8070_e9094_d_n8;
        locals.var_t3_dn9 = assign8070_e9094_d_n9;
        locals.var_t3_dn10 = assign8070_e9094_d_n10;
        locals.var_t3_dn11 = assign8070_e9094_d_n11;
        locals.var_t3_dn12 = assign8070_e9094_d_n12;

        let (assign8080_e9099, assign8080_e9099_d_n3, assign8080_e9099_d_n4, assign8080_e9099_d_n5, assign8080_e9099_d_n6, assign8080_e9099_d_n7, assign8080_e9099_d_n8, assign8080_e9099_d_n9, assign8080_e9099_d_n10, assign8080_e9099_d_n11, assign8080_e9099_d_n12,) = {
    if (locals.var_guard606 == 0.0) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign8080_e9099;
        locals.var_t1_dn3 = assign8080_e9099_d_n3;
        locals.var_t1_dn4 = assign8080_e9099_d_n4;
        locals.var_t1_dn5 = assign8080_e9099_d_n5;
        locals.var_t1_dn6 = assign8080_e9099_d_n6;
        locals.var_t1_dn7 = assign8080_e9099_d_n7;
        locals.var_t1_dn8 = assign8080_e9099_d_n8;
        locals.var_t1_dn9 = assign8080_e9099_d_n9;
        locals.var_t1_dn10 = assign8080_e9099_d_n10;
        locals.var_t1_dn11 = assign8080_e9099_d_n11;
        locals.var_t1_dn12 = assign8080_e9099_d_n12;

        let (assign8090_e9110, assign8090_e9110_d_n3, assign8090_e9110_d_n4, assign8090_e9110_d_n5, assign8090_e9110_d_n6, assign8090_e9110_d_n7, assign8090_e9110_d_n8, assign8090_e9110_d_n9, assign8090_e9110_d_n10, assign8090_e9110_d_n11, assign8090_e9110_d_n12,) = {
    if (locals.var_guard606 == 0.0) {
        let assign8090_e9106: f64 = (2.0 * locals.var_t1);
        let assign8090_e9107: f64 = (1.0 + assign8090_e9106);
        let assign8090_e9108: f64 = (locals.var_t1 * assign8090_e9107);
        (assign8090_e9108, ((locals.var_t1_dn3 * assign8090_e9107) + (locals.var_t1 * (2.0 * locals.var_t1_dn3))), ((locals.var_t1_dn4 * assign8090_e9107) + (locals.var_t1 * (2.0 * locals.var_t1_dn4))), ((locals.var_t1_dn5 * assign8090_e9107) + (locals.var_t1 * (2.0 * locals.var_t1_dn5))), ((locals.var_t1_dn6 * assign8090_e9107) + (locals.var_t1 * (2.0 * locals.var_t1_dn6))), ((locals.var_t1_dn7 * assign8090_e9107) + (locals.var_t1 * (2.0 * locals.var_t1_dn7))), ((locals.var_t1_dn8 * assign8090_e9107) + (locals.var_t1 * (2.0 * locals.var_t1_dn8))), ((locals.var_t1_dn9 * assign8090_e9107) + (locals.var_t1 * (2.0 * locals.var_t1_dn9))), ((locals.var_t1_dn10 * assign8090_e9107) + (locals.var_t1 * (2.0 * locals.var_t1_dn10))), ((locals.var_t1_dn11 * assign8090_e9107) + (locals.var_t1 * (2.0 * locals.var_t1_dn11))), ((locals.var_t1_dn12 * assign8090_e9107) + (locals.var_t1 * (2.0 * locals.var_t1_dn12))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign8090_e9110;
        locals.var_t3_dn3 = assign8090_e9110_d_n3;
        locals.var_t3_dn4 = assign8090_e9110_d_n4;
        locals.var_t3_dn5 = assign8090_e9110_d_n5;
        locals.var_t3_dn6 = assign8090_e9110_d_n6;
        locals.var_t3_dn7 = assign8090_e9110_d_n7;
        locals.var_t3_dn8 = assign8090_e9110_d_n8;
        locals.var_t3_dn9 = assign8090_e9110_d_n9;
        locals.var_t3_dn10 = assign8090_e9110_d_n10;
        locals.var_t3_dn11 = assign8090_e9110_d_n11;
        locals.var_t3_dn12 = assign8090_e9110_d_n12;

        let assign8100_e9113: f64 = (locals.var_pparam_b4soidvt0 * locals.var_t3);
        let assign8100_e9115: f64 = (assign8100_e9113 * locals.var_tmp1);
        locals.var_t3 = assign8100_e9115;
        locals.var_t3_dn3 = ((((locals.var_pparam_b4soidvt0_dn3 * locals.var_t3) + (locals.var_pparam_b4soidvt0 * locals.var_t3_dn3)) * locals.var_tmp1) + (assign8100_e9113 * locals.var_tmp1_dn3));
        locals.var_t3_dn4 = ((((locals.var_pparam_b4soidvt0_dn4 * locals.var_t3) + (locals.var_pparam_b4soidvt0 * locals.var_t3_dn4)) * locals.var_tmp1) + (assign8100_e9113 * locals.var_tmp1_dn4));
        locals.var_t3_dn5 = ((((locals.var_pparam_b4soidvt0_dn5 * locals.var_t3) + (locals.var_pparam_b4soidvt0 * locals.var_t3_dn5)) * locals.var_tmp1) + (assign8100_e9113 * locals.var_tmp1_dn5));
        locals.var_t3_dn6 = ((((locals.var_pparam_b4soidvt0_dn6 * locals.var_t3) + (locals.var_pparam_b4soidvt0 * locals.var_t3_dn6)) * locals.var_tmp1) + (assign8100_e9113 * locals.var_tmp1_dn6));
        locals.var_t3_dn7 = ((((locals.var_pparam_b4soidvt0_dn7 * locals.var_t3) + (locals.var_pparam_b4soidvt0 * locals.var_t3_dn7)) * locals.var_tmp1) + (assign8100_e9113 * locals.var_tmp1_dn7));
        locals.var_t3_dn8 = ((((locals.var_pparam_b4soidvt0_dn8 * locals.var_t3) + (locals.var_pparam_b4soidvt0 * locals.var_t3_dn8)) * locals.var_tmp1) + (assign8100_e9113 * locals.var_tmp1_dn8));
        locals.var_t3_dn9 = ((((locals.var_pparam_b4soidvt0_dn9 * locals.var_t3) + (locals.var_pparam_b4soidvt0 * locals.var_t3_dn9)) * locals.var_tmp1) + (assign8100_e9113 * locals.var_tmp1_dn9));
        locals.var_t3_dn10 = ((((locals.var_pparam_b4soidvt0_dn10 * locals.var_t3) + (locals.var_pparam_b4soidvt0 * locals.var_t3_dn10)) * locals.var_tmp1) + (assign8100_e9113 * locals.var_tmp1_dn10));
        locals.var_t3_dn11 = ((((locals.var_pparam_b4soidvt0_dn11 * locals.var_t3) + (locals.var_pparam_b4soidvt0 * locals.var_t3_dn11)) * locals.var_tmp1) + (assign8100_e9113 * locals.var_tmp1_dn11));
        locals.var_t3_dn12 = ((((locals.var_pparam_b4soidvt0_dn12 * locals.var_t3) + (locals.var_pparam_b4soidvt0 * locals.var_t3_dn12)) * locals.var_tmp1) + (assign8100_e9113 * locals.var_tmp1_dn12));

        let assign8110_e9118: f64 = (locals.var_b4soitoxp * locals.var_pparam_b4soiphi);
        let assign8110_e9121: f64 = (locals.var_pparam_b4soiweff + locals.var_pparam_b4soiw0);
        let assign8110_e9122: f64 = (assign8110_e9118 / assign8110_e9121);
        locals.var_t4 = assign8110_e9122;
        locals.var_t4_dn3 = (((((locals.var_b4soitoxp_dn3 * locals.var_pparam_b4soiphi) + (locals.var_b4soitoxp * locals.var_pparam_b4soiphi_dn3)) * assign8110_e9121) - (assign8110_e9118 * (locals.var_pparam_b4soiweff_dn3 + locals.var_pparam_b4soiw0_dn3))) / (assign8110_e9121 * assign8110_e9121));
        locals.var_t4_dn4 = (((((locals.var_b4soitoxp_dn4 * locals.var_pparam_b4soiphi) + (locals.var_b4soitoxp * locals.var_pparam_b4soiphi_dn4)) * assign8110_e9121) - (assign8110_e9118 * (locals.var_pparam_b4soiweff_dn4 + locals.var_pparam_b4soiw0_dn4))) / (assign8110_e9121 * assign8110_e9121));
        locals.var_t4_dn5 = (((((locals.var_b4soitoxp_dn5 * locals.var_pparam_b4soiphi) + (locals.var_b4soitoxp * locals.var_pparam_b4soiphi_dn5)) * assign8110_e9121) - (assign8110_e9118 * (locals.var_pparam_b4soiweff_dn5 + locals.var_pparam_b4soiw0_dn5))) / (assign8110_e9121 * assign8110_e9121));
        locals.var_t4_dn6 = (((((locals.var_b4soitoxp_dn6 * locals.var_pparam_b4soiphi) + (locals.var_b4soitoxp * locals.var_pparam_b4soiphi_dn6)) * assign8110_e9121) - (assign8110_e9118 * (locals.var_pparam_b4soiweff_dn6 + locals.var_pparam_b4soiw0_dn6))) / (assign8110_e9121 * assign8110_e9121));
        locals.var_t4_dn7 = (((((locals.var_b4soitoxp_dn7 * locals.var_pparam_b4soiphi) + (locals.var_b4soitoxp * locals.var_pparam_b4soiphi_dn7)) * assign8110_e9121) - (assign8110_e9118 * (locals.var_pparam_b4soiweff_dn7 + locals.var_pparam_b4soiw0_dn7))) / (assign8110_e9121 * assign8110_e9121));
        locals.var_t4_dn8 = (((((locals.var_b4soitoxp_dn8 * locals.var_pparam_b4soiphi) + (locals.var_b4soitoxp * locals.var_pparam_b4soiphi_dn8)) * assign8110_e9121) - (assign8110_e9118 * (locals.var_pparam_b4soiweff_dn8 + locals.var_pparam_b4soiw0_dn8))) / (assign8110_e9121 * assign8110_e9121));
        locals.var_t4_dn9 = (((((locals.var_b4soitoxp_dn9 * locals.var_pparam_b4soiphi) + (locals.var_b4soitoxp * locals.var_pparam_b4soiphi_dn9)) * assign8110_e9121) - (assign8110_e9118 * (locals.var_pparam_b4soiweff_dn9 + locals.var_pparam_b4soiw0_dn9))) / (assign8110_e9121 * assign8110_e9121));
        locals.var_t4_dn10 = (((((locals.var_b4soitoxp_dn10 * locals.var_pparam_b4soiphi) + (locals.var_b4soitoxp * locals.var_pparam_b4soiphi_dn10)) * assign8110_e9121) - (assign8110_e9118 * (locals.var_pparam_b4soiweff_dn10 + locals.var_pparam_b4soiw0_dn10))) / (assign8110_e9121 * assign8110_e9121));
        locals.var_t4_dn11 = (((((locals.var_b4soitoxp_dn11 * locals.var_pparam_b4soiphi) + (locals.var_b4soitoxp * locals.var_pparam_b4soiphi_dn11)) * assign8110_e9121) - (assign8110_e9118 * (locals.var_pparam_b4soiweff_dn11 + locals.var_pparam_b4soiw0_dn11))) / (assign8110_e9121 * assign8110_e9121));
        locals.var_t4_dn12 = (((((locals.var_b4soitoxp_dn12 * locals.var_pparam_b4soiphi) + (locals.var_b4soitoxp * locals.var_pparam_b4soiphi_dn12)) * assign8110_e9121) - (assign8110_e9118 * (locals.var_pparam_b4soiweff_dn12 + locals.var_pparam_b4soiw0_dn12))) / (assign8110_e9121 * assign8110_e9121));

        let assign8120_e9126: f64 = (locals.var_pparam_b4soilpe0 / locals.var_pparam_b4soileff);
        let assign8120_e9127: f64 = (1.0 + assign8120_e9126);
        let assign8120_e9128: f64 = (assign8120_e9127).sqrt();
        locals.var_t0 = assign8120_e9128;
        locals.var_t0_dn3 = ((((locals.var_pparam_b4soilpe0_dn3 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn3)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)) / (2.0 * assign8120_e9128));
        locals.var_t0_dn4 = ((((locals.var_pparam_b4soilpe0_dn4 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn4)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)) / (2.0 * assign8120_e9128));
        locals.var_t0_dn5 = ((((locals.var_pparam_b4soilpe0_dn5 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn5)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)) / (2.0 * assign8120_e9128));
        locals.var_t0_dn6 = ((((locals.var_pparam_b4soilpe0_dn6 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn6)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)) / (2.0 * assign8120_e9128));
        locals.var_t0_dn7 = ((((locals.var_pparam_b4soilpe0_dn7 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn7)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)) / (2.0 * assign8120_e9128));
        locals.var_t0_dn8 = ((((locals.var_pparam_b4soilpe0_dn8 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn8)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)) / (2.0 * assign8120_e9128));
        locals.var_t0_dn9 = ((((locals.var_pparam_b4soilpe0_dn9 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn9)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)) / (2.0 * assign8120_e9128));
        locals.var_t0_dn10 = ((((locals.var_pparam_b4soilpe0_dn10 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn10)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)) / (2.0 * assign8120_e9128));
        locals.var_t0_dn11 = ((((locals.var_pparam_b4soilpe0_dn11 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn11)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)) / (2.0 * assign8120_e9128));
        locals.var_t0_dn12 = ((((locals.var_pparam_b4soilpe0_dn12 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn12)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)) / (2.0 * assign8120_e9128));

        let assign8130_e9132: f64 = (locals.var_t0 - 1.0);
        let assign8130_e9133: f64 = (locals.var_here_b4soik1ox * assign8130_e9132);
        let assign8130_e9135: f64 = (assign8130_e9133 * locals.var_pparam_b4soisqrtphi);
        let assign8130_e9139: f64 = (locals.var_pparam_b4soikt1l / locals.var_pparam_b4soileff);
        let assign8130_e9140: f64 = (locals.var_pparam_b4soikt1 + assign8130_e9139);
        let assign8130_e9142: f64 = (assign8130_e9140 * locals.var_trm1);
        let assign8130_e9143: f64 = (assign8130_e9135 + assign8130_e9142);
        locals.var_t5 = assign8130_e9143;
        locals.var_t5_dn3 = (((((locals.var_here_b4soik1ox_dn3 * assign8130_e9132) + (locals.var_here_b4soik1ox * locals.var_t0_dn3)) * locals.var_pparam_b4soisqrtphi) + (assign8130_e9133 * locals.var_pparam_b4soisqrtphi_dn3)) + ((locals.var_pparam_b4soikt1_dn3 + (((locals.var_pparam_b4soikt1l_dn3 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soikt1l * locals.var_pparam_b4soileff_dn3)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) * locals.var_trm1));
        locals.var_t5_dn4 = (((((locals.var_here_b4soik1ox_dn4 * assign8130_e9132) + (locals.var_here_b4soik1ox * locals.var_t0_dn4)) * locals.var_pparam_b4soisqrtphi) + (assign8130_e9133 * locals.var_pparam_b4soisqrtphi_dn4)) + (((locals.var_pparam_b4soikt1_dn4 + (((locals.var_pparam_b4soikt1l_dn4 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soikt1l * locals.var_pparam_b4soileff_dn4)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) * locals.var_trm1) + (assign8130_e9140 * locals.var_trm1_dn4)));
        locals.var_t5_dn5 = (((((locals.var_here_b4soik1ox_dn5 * assign8130_e9132) + (locals.var_here_b4soik1ox * locals.var_t0_dn5)) * locals.var_pparam_b4soisqrtphi) + (assign8130_e9133 * locals.var_pparam_b4soisqrtphi_dn5)) + (((locals.var_pparam_b4soikt1_dn5 + (((locals.var_pparam_b4soikt1l_dn5 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soikt1l * locals.var_pparam_b4soileff_dn5)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) * locals.var_trm1) + (assign8130_e9140 * locals.var_trm1_dn5)));
        locals.var_t5_dn6 = (((((locals.var_here_b4soik1ox_dn6 * assign8130_e9132) + (locals.var_here_b4soik1ox * locals.var_t0_dn6)) * locals.var_pparam_b4soisqrtphi) + (assign8130_e9133 * locals.var_pparam_b4soisqrtphi_dn6)) + (((locals.var_pparam_b4soikt1_dn6 + (((locals.var_pparam_b4soikt1l_dn6 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soikt1l * locals.var_pparam_b4soileff_dn6)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) * locals.var_trm1) + (assign8130_e9140 * locals.var_trm1_dn6)));
        locals.var_t5_dn7 = (((((locals.var_here_b4soik1ox_dn7 * assign8130_e9132) + (locals.var_here_b4soik1ox * locals.var_t0_dn7)) * locals.var_pparam_b4soisqrtphi) + (assign8130_e9133 * locals.var_pparam_b4soisqrtphi_dn7)) + ((locals.var_pparam_b4soikt1_dn7 + (((locals.var_pparam_b4soikt1l_dn7 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soikt1l * locals.var_pparam_b4soileff_dn7)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) * locals.var_trm1));
        locals.var_t5_dn8 = (((((locals.var_here_b4soik1ox_dn8 * assign8130_e9132) + (locals.var_here_b4soik1ox * locals.var_t0_dn8)) * locals.var_pparam_b4soisqrtphi) + (assign8130_e9133 * locals.var_pparam_b4soisqrtphi_dn8)) + ((locals.var_pparam_b4soikt1_dn8 + (((locals.var_pparam_b4soikt1l_dn8 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soikt1l * locals.var_pparam_b4soileff_dn8)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) * locals.var_trm1));
        locals.var_t5_dn9 = (((((locals.var_here_b4soik1ox_dn9 * assign8130_e9132) + (locals.var_here_b4soik1ox * locals.var_t0_dn9)) * locals.var_pparam_b4soisqrtphi) + (assign8130_e9133 * locals.var_pparam_b4soisqrtphi_dn9)) + ((locals.var_pparam_b4soikt1_dn9 + (((locals.var_pparam_b4soikt1l_dn9 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soikt1l * locals.var_pparam_b4soileff_dn9)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) * locals.var_trm1));
        locals.var_t5_dn10 = (((((locals.var_here_b4soik1ox_dn10 * assign8130_e9132) + (locals.var_here_b4soik1ox * locals.var_t0_dn10)) * locals.var_pparam_b4soisqrtphi) + (assign8130_e9133 * locals.var_pparam_b4soisqrtphi_dn10)) + ((locals.var_pparam_b4soikt1_dn10 + (((locals.var_pparam_b4soikt1l_dn10 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soikt1l * locals.var_pparam_b4soileff_dn10)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) * locals.var_trm1));
        locals.var_t5_dn11 = (((((locals.var_here_b4soik1ox_dn11 * assign8130_e9132) + (locals.var_here_b4soik1ox * locals.var_t0_dn11)) * locals.var_pparam_b4soisqrtphi) + (assign8130_e9133 * locals.var_pparam_b4soisqrtphi_dn11)) + ((locals.var_pparam_b4soikt1_dn11 + (((locals.var_pparam_b4soikt1l_dn11 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soikt1l * locals.var_pparam_b4soileff_dn11)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) * locals.var_trm1));
        locals.var_t5_dn12 = (((((locals.var_here_b4soik1ox_dn12 * assign8130_e9132) + (locals.var_here_b4soik1ox * locals.var_t0_dn12)) * locals.var_pparam_b4soisqrtphi) + (assign8130_e9133 * locals.var_pparam_b4soisqrtphi_dn12)) + ((locals.var_pparam_b4soikt1_dn12 + (((locals.var_pparam_b4soikt1l_dn12 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soikt1l * locals.var_pparam_b4soileff_dn12)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff))) * locals.var_trm1));

    }

    pub(super) fn stamp_transient_block_24(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign8140_e9146: f64 = (p.p37 * locals.var_here_b4soivth0);
        let assign8140_e9148: f64 = (assign8140_e9146 - locals.var_t2);
        let assign8140_e9150: f64 = (assign8140_e9148 - locals.var_t3);
        let assign8140_e9153: f64 = (locals.var_pparam_b4soik3 * locals.var_t4);
        let assign8140_e9154: f64 = (assign8140_e9150 + assign8140_e9153);
        let assign8140_e9156: f64 = (assign8140_e9154 + locals.var_t5);
        locals.var_tmp3 = assign8140_e9156;
        locals.var_tmp3_dn3 = (((((p.p37 * locals.var_here_b4soivth0_dn3) - locals.var_t2_dn3) - locals.var_t3_dn3) + ((locals.var_pparam_b4soik3_dn3 * locals.var_t4) + (locals.var_pparam_b4soik3 * locals.var_t4_dn3))) + locals.var_t5_dn3);
        locals.var_tmp3_dn4 = (((((p.p37 * locals.var_here_b4soivth0_dn4) - locals.var_t2_dn4) - locals.var_t3_dn4) + ((locals.var_pparam_b4soik3_dn4 * locals.var_t4) + (locals.var_pparam_b4soik3 * locals.var_t4_dn4))) + locals.var_t5_dn4);
        locals.var_tmp3_dn5 = (((((p.p37 * locals.var_here_b4soivth0_dn5) - locals.var_t2_dn5) - locals.var_t3_dn5) + ((locals.var_pparam_b4soik3_dn5 * locals.var_t4) + (locals.var_pparam_b4soik3 * locals.var_t4_dn5))) + locals.var_t5_dn5);
        locals.var_tmp3_dn6 = (((((p.p37 * locals.var_here_b4soivth0_dn6) - locals.var_t2_dn6) - locals.var_t3_dn6) + ((locals.var_pparam_b4soik3_dn6 * locals.var_t4) + (locals.var_pparam_b4soik3 * locals.var_t4_dn6))) + locals.var_t5_dn6);
        locals.var_tmp3_dn7 = (((((p.p37 * locals.var_here_b4soivth0_dn7) - locals.var_t2_dn7) - locals.var_t3_dn7) + ((locals.var_pparam_b4soik3_dn7 * locals.var_t4) + (locals.var_pparam_b4soik3 * locals.var_t4_dn7))) + locals.var_t5_dn7);
        locals.var_tmp3_dn8 = (((((p.p37 * locals.var_here_b4soivth0_dn8) - locals.var_t2_dn8) - locals.var_t3_dn8) + ((locals.var_pparam_b4soik3_dn8 * locals.var_t4) + (locals.var_pparam_b4soik3 * locals.var_t4_dn8))) + locals.var_t5_dn8);
        locals.var_tmp3_dn9 = (((((p.p37 * locals.var_here_b4soivth0_dn9) - locals.var_t2_dn9) - locals.var_t3_dn9) + ((locals.var_pparam_b4soik3_dn9 * locals.var_t4) + (locals.var_pparam_b4soik3 * locals.var_t4_dn9))) + locals.var_t5_dn9);
        locals.var_tmp3_dn10 = (((((p.p37 * locals.var_here_b4soivth0_dn10) - locals.var_t2_dn10) - locals.var_t3_dn10) + ((locals.var_pparam_b4soik3_dn10 * locals.var_t4) + (locals.var_pparam_b4soik3 * locals.var_t4_dn10))) + locals.var_t5_dn10);
        locals.var_tmp3_dn11 = (((((p.p37 * locals.var_here_b4soivth0_dn11) - locals.var_t2_dn11) - locals.var_t3_dn11) + ((locals.var_pparam_b4soik3_dn11 * locals.var_t4) + (locals.var_pparam_b4soik3 * locals.var_t4_dn11))) + locals.var_t5_dn11);
        locals.var_tmp3_dn12 = (((((p.p37 * locals.var_here_b4soivth0_dn12) - locals.var_t2_dn12) - locals.var_t3_dn12) + ((locals.var_pparam_b4soik3_dn12 * locals.var_t4) + (locals.var_pparam_b4soik3 * locals.var_t4_dn12))) + locals.var_t5_dn12);

        let assign8150_e9159: f64 = (locals.var_tmp3 - locals.var_pparam_b4soiphi);
        let assign8150_e9162: f64 = (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi);
        let assign8150_e9163: f64 = (assign8150_e9159 - assign8150_e9162);
        locals.var_pparam_b4soivfbzb = assign8150_e9163;
        locals.var_pparam_b4soivfbzb_dn3 = ((locals.var_tmp3_dn3 - locals.var_pparam_b4soiphi_dn3) - ((locals.var_pparam_b4soik1_dn3 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi_dn3)));
        locals.var_pparam_b4soivfbzb_dn4 = ((locals.var_tmp3_dn4 - locals.var_pparam_b4soiphi_dn4) - ((locals.var_pparam_b4soik1_dn4 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi_dn4)));
        locals.var_pparam_b4soivfbzb_dn5 = ((locals.var_tmp3_dn5 - locals.var_pparam_b4soiphi_dn5) - ((locals.var_pparam_b4soik1_dn5 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi_dn5)));
        locals.var_pparam_b4soivfbzb_dn6 = ((locals.var_tmp3_dn6 - locals.var_pparam_b4soiphi_dn6) - ((locals.var_pparam_b4soik1_dn6 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi_dn6)));
        locals.var_pparam_b4soivfbzb_dn7 = ((locals.var_tmp3_dn7 - locals.var_pparam_b4soiphi_dn7) - ((locals.var_pparam_b4soik1_dn7 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi_dn7)));
        locals.var_pparam_b4soivfbzb_dn8 = ((locals.var_tmp3_dn8 - locals.var_pparam_b4soiphi_dn8) - ((locals.var_pparam_b4soik1_dn8 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi_dn8)));
        locals.var_pparam_b4soivfbzb_dn9 = ((locals.var_tmp3_dn9 - locals.var_pparam_b4soiphi_dn9) - ((locals.var_pparam_b4soik1_dn9 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi_dn9)));
        locals.var_pparam_b4soivfbzb_dn10 = ((locals.var_tmp3_dn10 - locals.var_pparam_b4soiphi_dn10) - ((locals.var_pparam_b4soik1_dn10 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi_dn10)));
        locals.var_pparam_b4soivfbzb_dn11 = ((locals.var_tmp3_dn11 - locals.var_pparam_b4soiphi_dn11) - ((locals.var_pparam_b4soik1_dn11 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi_dn11)));
        locals.var_pparam_b4soivfbzb_dn12 = ((locals.var_tmp3_dn12 - locals.var_pparam_b4soiphi_dn12) - ((locals.var_pparam_b4soik1_dn12 * locals.var_pparam_b4soisqrtphi) + (locals.var_pparam_b4soik1 * locals.var_pparam_b4soisqrtphi_dn12)));

        let assign8160_e9166: f64 = (1.602176462e-19 * locals.var_pparam_b4soinpeak);
        let assign8160_e9170: f64 = (locals.var_pparam_b4soilpe0 / locals.var_pparam_b4soileff);
        let assign8160_e9171: f64 = (1.0 + assign8160_e9170);
        let assign8160_e9172: f64 = (assign8160_e9166 * assign8160_e9171);
        let assign8160_e9174: f64 = (assign8160_e9172 * 1000000.0);
        let assign8160_e9176: f64 = (assign8160_e9174 * p.p155);
        locals.var_pparam_b4soiqsi = assign8160_e9176;
        locals.var_pparam_b4soiqsi_dn3 = (((((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn3) * assign8160_e9171) + (assign8160_e9166 * (((locals.var_pparam_b4soilpe0_dn3 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn3)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))) * 1000000.0) * p.p155);
        locals.var_pparam_b4soiqsi_dn4 = (((((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn4) * assign8160_e9171) + (assign8160_e9166 * (((locals.var_pparam_b4soilpe0_dn4 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn4)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))) * 1000000.0) * p.p155);
        locals.var_pparam_b4soiqsi_dn5 = (((((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn5) * assign8160_e9171) + (assign8160_e9166 * (((locals.var_pparam_b4soilpe0_dn5 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn5)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))) * 1000000.0) * p.p155);
        locals.var_pparam_b4soiqsi_dn6 = (((((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn6) * assign8160_e9171) + (assign8160_e9166 * (((locals.var_pparam_b4soilpe0_dn6 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn6)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))) * 1000000.0) * p.p155);
        locals.var_pparam_b4soiqsi_dn7 = (((((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn7) * assign8160_e9171) + (assign8160_e9166 * (((locals.var_pparam_b4soilpe0_dn7 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn7)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))) * 1000000.0) * p.p155);
        locals.var_pparam_b4soiqsi_dn8 = (((((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn8) * assign8160_e9171) + (assign8160_e9166 * (((locals.var_pparam_b4soilpe0_dn8 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn8)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))) * 1000000.0) * p.p155);
        locals.var_pparam_b4soiqsi_dn9 = (((((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn9) * assign8160_e9171) + (assign8160_e9166 * (((locals.var_pparam_b4soilpe0_dn9 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn9)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))) * 1000000.0) * p.p155);
        locals.var_pparam_b4soiqsi_dn10 = (((((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn10) * assign8160_e9171) + (assign8160_e9166 * (((locals.var_pparam_b4soilpe0_dn10 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn10)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))) * 1000000.0) * p.p155);
        locals.var_pparam_b4soiqsi_dn11 = (((((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn11) * assign8160_e9171) + (assign8160_e9166 * (((locals.var_pparam_b4soilpe0_dn11 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn11)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))) * 1000000.0) * p.p155);
        locals.var_pparam_b4soiqsi_dn12 = (((((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn12) * assign8160_e9171) + (assign8160_e9166 * (((locals.var_pparam_b4soilpe0_dn12 * locals.var_pparam_b4soileff) - (locals.var_pparam_b4soilpe0 * locals.var_pparam_b4soileff_dn12)) / (locals.var_pparam_b4soileff * locals.var_pparam_b4soileff)))) * 1000000.0) * p.p155);

        let assign8170_e9181: f64 = (locals.var_pparam_b4soiweff / p.p23);
        let assign8170_e9183: f64 = (assign8170_e9181 / 3.0);
        let assign8170_e9185: f64 = (assign8170_e9183 / p.p425);
        let assign8170_e9186: f64 = (p.p427 + assign8170_e9185);
        let assign8170_e9187: f64 = (p.p424 * assign8170_e9186);
        let assign8170_e9190: f64 = (p.p425 * p.p3);
        let assign8170_e9193: f64 = (p.p1 - p.p428);
        let assign8170_e9194: f64 = (assign8170_e9190 * assign8170_e9193);
        let assign8170_e9195: f64 = (assign8170_e9187 / assign8170_e9194);
        let assign8170_e9199: f64 = (p.p1 * locals.var_pparam_b4soiweff);
        let assign8170_e9201: f64 = (assign8170_e9199 * p.p3);
        let assign8170_e9202: f64 = (p.p426 / assign8170_e9201);
        let assign8170_e9203: f64 = (assign8170_e9195 + assign8170_e9202);
        locals.var_b4soigrgeltd = assign8170_e9203;
        locals.var_b4soigrgeltd_dn3 = (((p.p424 * (((locals.var_pparam_b4soiweff_dn3 / p.p23) / 3.0) / p.p425)) / assign8170_e9194) + (-((p.p426 * ((p.p1 * locals.var_pparam_b4soiweff_dn3) * p.p3)) / (assign8170_e9201 * assign8170_e9201))));
        locals.var_b4soigrgeltd_dn4 = (((p.p424 * (((locals.var_pparam_b4soiweff_dn4 / p.p23) / 3.0) / p.p425)) / assign8170_e9194) + (-((p.p426 * ((p.p1 * locals.var_pparam_b4soiweff_dn4) * p.p3)) / (assign8170_e9201 * assign8170_e9201))));
        locals.var_b4soigrgeltd_dn5 = (((p.p424 * (((locals.var_pparam_b4soiweff_dn5 / p.p23) / 3.0) / p.p425)) / assign8170_e9194) + (-((p.p426 * ((p.p1 * locals.var_pparam_b4soiweff_dn5) * p.p3)) / (assign8170_e9201 * assign8170_e9201))));
        locals.var_b4soigrgeltd_dn6 = (((p.p424 * (((locals.var_pparam_b4soiweff_dn6 / p.p23) / 3.0) / p.p425)) / assign8170_e9194) + (-((p.p426 * ((p.p1 * locals.var_pparam_b4soiweff_dn6) * p.p3)) / (assign8170_e9201 * assign8170_e9201))));
        locals.var_b4soigrgeltd_dn7 = (((p.p424 * (((locals.var_pparam_b4soiweff_dn7 / p.p23) / 3.0) / p.p425)) / assign8170_e9194) + (-((p.p426 * ((p.p1 * locals.var_pparam_b4soiweff_dn7) * p.p3)) / (assign8170_e9201 * assign8170_e9201))));
        locals.var_b4soigrgeltd_dn8 = (((p.p424 * (((locals.var_pparam_b4soiweff_dn8 / p.p23) / 3.0) / p.p425)) / assign8170_e9194) + (-((p.p426 * ((p.p1 * locals.var_pparam_b4soiweff_dn8) * p.p3)) / (assign8170_e9201 * assign8170_e9201))));
        locals.var_b4soigrgeltd_dn9 = (((p.p424 * (((locals.var_pparam_b4soiweff_dn9 / p.p23) / 3.0) / p.p425)) / assign8170_e9194) + (-((p.p426 * ((p.p1 * locals.var_pparam_b4soiweff_dn9) * p.p3)) / (assign8170_e9201 * assign8170_e9201))));
        locals.var_b4soigrgeltd_dn10 = (((p.p424 * (((locals.var_pparam_b4soiweff_dn10 / p.p23) / 3.0) / p.p425)) / assign8170_e9194) + (-((p.p426 * ((p.p1 * locals.var_pparam_b4soiweff_dn10) * p.p3)) / (assign8170_e9201 * assign8170_e9201))));
        locals.var_b4soigrgeltd_dn11 = (((p.p424 * (((locals.var_pparam_b4soiweff_dn11 / p.p23) / 3.0) / p.p425)) / assign8170_e9194) + (-((p.p426 * ((p.p1 * locals.var_pparam_b4soiweff_dn11) * p.p3)) / (assign8170_e9201 * assign8170_e9201))));
        locals.var_b4soigrgeltd_dn12 = (((p.p424 * (((locals.var_pparam_b4soiweff_dn12 / p.p23) / 3.0) / p.p425)) / assign8170_e9194) + (-((p.p426 * ((p.p1 * locals.var_pparam_b4soiweff_dn12) * p.p3)) / (assign8170_e9201 * assign8170_e9201))));

        let assign8180_e9206: f64 = if locals.var_b4soigrgeltd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard607 = assign8180_e9206;

        let (assign8190_e9212, assign8190_e9212_d_n3, assign8190_e9212_d_n4, assign8190_e9212_d_n5, assign8190_e9212_d_n6, assign8190_e9212_d_n7, assign8190_e9212_d_n8, assign8190_e9212_d_n9, assign8190_e9212_d_n10, assign8190_e9212_d_n11, assign8190_e9212_d_n12,) = {
    if (locals.var_guard607 != 0.0) {
        let assign8190_e9210: f64 = (1.0 / locals.var_b4soigrgeltd);
        (assign8190_e9210, (-(locals.var_b4soigrgeltd_dn3 / (locals.var_b4soigrgeltd * locals.var_b4soigrgeltd))), (-(locals.var_b4soigrgeltd_dn4 / (locals.var_b4soigrgeltd * locals.var_b4soigrgeltd))), (-(locals.var_b4soigrgeltd_dn5 / (locals.var_b4soigrgeltd * locals.var_b4soigrgeltd))), (-(locals.var_b4soigrgeltd_dn6 / (locals.var_b4soigrgeltd * locals.var_b4soigrgeltd))), (-(locals.var_b4soigrgeltd_dn7 / (locals.var_b4soigrgeltd * locals.var_b4soigrgeltd))), (-(locals.var_b4soigrgeltd_dn8 / (locals.var_b4soigrgeltd * locals.var_b4soigrgeltd))), (-(locals.var_b4soigrgeltd_dn9 / (locals.var_b4soigrgeltd * locals.var_b4soigrgeltd))), (-(locals.var_b4soigrgeltd_dn10 / (locals.var_b4soigrgeltd * locals.var_b4soigrgeltd))), (-(locals.var_b4soigrgeltd_dn11 / (locals.var_b4soigrgeltd * locals.var_b4soigrgeltd))), (-(locals.var_b4soigrgeltd_dn12 / (locals.var_b4soigrgeltd * locals.var_b4soigrgeltd))),)
    } else {
        (locals.var_b4soigrgeltd, locals.var_b4soigrgeltd_dn3, locals.var_b4soigrgeltd_dn4, locals.var_b4soigrgeltd_dn5, locals.var_b4soigrgeltd_dn6, locals.var_b4soigrgeltd_dn7, locals.var_b4soigrgeltd_dn8, locals.var_b4soigrgeltd_dn9, locals.var_b4soigrgeltd_dn10, locals.var_b4soigrgeltd_dn11, locals.var_b4soigrgeltd_dn12,)
    }
};
        locals.var_b4soigrgeltd = assign8190_e9212;
        locals.var_b4soigrgeltd_dn3 = assign8190_e9212_d_n3;
        locals.var_b4soigrgeltd_dn4 = assign8190_e9212_d_n4;
        locals.var_b4soigrgeltd_dn5 = assign8190_e9212_d_n5;
        locals.var_b4soigrgeltd_dn6 = assign8190_e9212_d_n6;
        locals.var_b4soigrgeltd_dn7 = assign8190_e9212_d_n7;
        locals.var_b4soigrgeltd_dn8 = assign8190_e9212_d_n8;
        locals.var_b4soigrgeltd_dn9 = assign8190_e9212_d_n9;
        locals.var_b4soigrgeltd_dn10 = assign8190_e9212_d_n10;
        locals.var_b4soigrgeltd_dn11 = assign8190_e9212_d_n11;
        locals.var_b4soigrgeltd_dn12 = assign8190_e9212_d_n12;

        let (assign8200_e9217, assign8200_e9217_d_n3, assign8200_e9217_d_n4, assign8200_e9217_d_n5, assign8200_e9217_d_n6, assign8200_e9217_d_n7, assign8200_e9217_d_n8, assign8200_e9217_d_n9, assign8200_e9217_d_n10, assign8200_e9217_d_n11, assign8200_e9217_d_n12,) = {
    if (locals.var_guard607 == 0.0) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soigrgeltd, locals.var_b4soigrgeltd_dn3, locals.var_b4soigrgeltd_dn4, locals.var_b4soigrgeltd_dn5, locals.var_b4soigrgeltd_dn6, locals.var_b4soigrgeltd_dn7, locals.var_b4soigrgeltd_dn8, locals.var_b4soigrgeltd_dn9, locals.var_b4soigrgeltd_dn10, locals.var_b4soigrgeltd_dn11, locals.var_b4soigrgeltd_dn12,)
    }
};
        locals.var_b4soigrgeltd = assign8200_e9217;
        locals.var_b4soigrgeltd_dn3 = assign8200_e9217_d_n3;
        locals.var_b4soigrgeltd_dn4 = assign8200_e9217_d_n4;
        locals.var_b4soigrgeltd_dn5 = assign8200_e9217_d_n5;
        locals.var_b4soigrgeltd_dn6 = assign8200_e9217_d_n6;
        locals.var_b4soigrgeltd_dn7 = assign8200_e9217_d_n7;
        locals.var_b4soigrgeltd_dn8 = assign8200_e9217_d_n8;
        locals.var_b4soigrgeltd_dn9 = assign8200_e9217_d_n9;
        locals.var_b4soigrgeltd_dn10 = assign8200_e9217_d_n10;
        locals.var_b4soigrgeltd_dn11 = assign8200_e9217_d_n11;
        locals.var_b4soigrgeltd_dn12 = assign8200_e9217_d_n12;

        let assign8300_e9274: f64 = (p.p37 * p.p20);
        let assign8300_e9275: f64 = (locals.var_pparam_b4soivfbzb + assign8300_e9274);
        locals.var_b4soivfbzb = assign8300_e9275;
        locals.var_b4soivfbzb_dn3 = locals.var_pparam_b4soivfbzb_dn3;
        locals.var_b4soivfbzb_dn4 = locals.var_pparam_b4soivfbzb_dn4;
        locals.var_b4soivfbzb_dn5 = locals.var_pparam_b4soivfbzb_dn5;
        locals.var_b4soivfbzb_dn6 = locals.var_pparam_b4soivfbzb_dn6;
        locals.var_b4soivfbzb_dn7 = locals.var_pparam_b4soivfbzb_dn7;
        locals.var_b4soivfbzb_dn8 = locals.var_pparam_b4soivfbzb_dn8;
        locals.var_b4soivfbzb_dn9 = locals.var_pparam_b4soivfbzb_dn9;
        locals.var_b4soivfbzb_dn10 = locals.var_pparam_b4soivfbzb_dn10;
        locals.var_b4soivfbzb_dn11 = locals.var_pparam_b4soivfbzb_dn11;
        locals.var_b4soivfbzb_dn12 = locals.var_pparam_b4soivfbzb_dn12;

        let assign8310_e9278: f64 = (locals.var_epssub * locals.var_vtm0);
        let assign8310_e9281: f64 = (1.602176462e-19 * locals.var_pparam_b4soinpeak);
        let assign8310_e9283: f64 = (assign8310_e9281 * 1000000.0);
        let assign8310_e9284: f64 = (assign8310_e9278 / assign8310_e9283);
        let assign8310_e9285: f64 = (assign8310_e9284).sqrt();
        let assign8310_e9287: f64 = (assign8310_e9285 / 3.0);
        locals.var_pparam_b4soildeb = assign8310_e9287;
        locals.var_pparam_b4soildeb_dn3 = (((-((assign8310_e9278 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn3) * 1000000.0)) / (assign8310_e9283 * assign8310_e9283))) / (2.0 * assign8310_e9285)) / 3.0);
        locals.var_pparam_b4soildeb_dn4 = (((-((assign8310_e9278 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn4) * 1000000.0)) / (assign8310_e9283 * assign8310_e9283))) / (2.0 * assign8310_e9285)) / 3.0);
        locals.var_pparam_b4soildeb_dn5 = (((-((assign8310_e9278 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn5) * 1000000.0)) / (assign8310_e9283 * assign8310_e9283))) / (2.0 * assign8310_e9285)) / 3.0);
        locals.var_pparam_b4soildeb_dn6 = (((-((assign8310_e9278 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn6) * 1000000.0)) / (assign8310_e9283 * assign8310_e9283))) / (2.0 * assign8310_e9285)) / 3.0);
        locals.var_pparam_b4soildeb_dn7 = (((-((assign8310_e9278 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn7) * 1000000.0)) / (assign8310_e9283 * assign8310_e9283))) / (2.0 * assign8310_e9285)) / 3.0);
        locals.var_pparam_b4soildeb_dn8 = (((-((assign8310_e9278 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn8) * 1000000.0)) / (assign8310_e9283 * assign8310_e9283))) / (2.0 * assign8310_e9285)) / 3.0);
        locals.var_pparam_b4soildeb_dn9 = (((-((assign8310_e9278 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn9) * 1000000.0)) / (assign8310_e9283 * assign8310_e9283))) / (2.0 * assign8310_e9285)) / 3.0);
        locals.var_pparam_b4soildeb_dn10 = (((-((assign8310_e9278 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn10) * 1000000.0)) / (assign8310_e9283 * assign8310_e9283))) / (2.0 * assign8310_e9285)) / 3.0);
        locals.var_pparam_b4soildeb_dn11 = (((-((assign8310_e9278 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn11) * 1000000.0)) / (assign8310_e9283 * assign8310_e9283))) / (2.0 * assign8310_e9285)) / 3.0);
        locals.var_pparam_b4soildeb_dn12 = (((-((assign8310_e9278 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn12) * 1000000.0)) / (assign8310_e9283 * assign8310_e9283))) / (2.0 * assign8310_e9285)) / 3.0);

        let assign8320_e9290: f64 = (p.p37 * locals.var_here_b4soivth0);
        let assign8320_e9292: f64 = (assign8320_e9290 - locals.var_here_b4soivfb);
        let assign8320_e9294: f64 = (assign8320_e9292 - locals.var_pparam_b4soiphi);
        locals.var_t1 = assign8320_e9294;
        locals.var_t1_dn3 = (((p.p37 * locals.var_here_b4soivth0_dn3) - locals.var_here_b4soivfb_dn3) - locals.var_pparam_b4soiphi_dn3);
        locals.var_t1_dn4 = (((p.p37 * locals.var_here_b4soivth0_dn4) - locals.var_here_b4soivfb_dn4) - locals.var_pparam_b4soiphi_dn4);
        locals.var_t1_dn5 = (((p.p37 * locals.var_here_b4soivth0_dn5) - locals.var_here_b4soivfb_dn5) - locals.var_pparam_b4soiphi_dn5);
        locals.var_t1_dn6 = (((p.p37 * locals.var_here_b4soivth0_dn6) - locals.var_here_b4soivfb_dn6) - locals.var_pparam_b4soiphi_dn6);
        locals.var_t1_dn7 = (((p.p37 * locals.var_here_b4soivth0_dn7) - locals.var_here_b4soivfb_dn7) - locals.var_pparam_b4soiphi_dn7);
        locals.var_t1_dn8 = (((p.p37 * locals.var_here_b4soivth0_dn8) - locals.var_here_b4soivfb_dn8) - locals.var_pparam_b4soiphi_dn8);
        locals.var_t1_dn9 = (((p.p37 * locals.var_here_b4soivth0_dn9) - locals.var_here_b4soivfb_dn9) - locals.var_pparam_b4soiphi_dn9);
        locals.var_t1_dn10 = (((p.p37 * locals.var_here_b4soivth0_dn10) - locals.var_here_b4soivfb_dn10) - locals.var_pparam_b4soiphi_dn10);
        locals.var_t1_dn11 = (((p.p37 * locals.var_here_b4soivth0_dn11) - locals.var_here_b4soivfb_dn11) - locals.var_pparam_b4soiphi_dn11);
        locals.var_t1_dn12 = (((p.p37 * locals.var_here_b4soivth0_dn12) - locals.var_here_b4soivfb_dn12) - locals.var_pparam_b4soiphi_dn12);

        let assign8330_e9297: f64 = (locals.var_t1 + locals.var_t1);
        locals.var_t2 = assign8330_e9297;
        locals.var_t2_dn3 = (locals.var_t1_dn3 + locals.var_t1_dn3);
        locals.var_t2_dn4 = (locals.var_t1_dn4 + locals.var_t1_dn4);
        locals.var_t2_dn5 = (locals.var_t1_dn5 + locals.var_t1_dn5);
        locals.var_t2_dn6 = (locals.var_t1_dn6 + locals.var_t1_dn6);
        locals.var_t2_dn7 = (locals.var_t1_dn7 + locals.var_t1_dn7);
        locals.var_t2_dn8 = (locals.var_t1_dn8 + locals.var_t1_dn8);
        locals.var_t2_dn9 = (locals.var_t1_dn9 + locals.var_t1_dn9);
        locals.var_t2_dn10 = (locals.var_t1_dn10 + locals.var_t1_dn10);
        locals.var_t2_dn11 = (locals.var_t1_dn11 + locals.var_t1_dn11);
        locals.var_t2_dn12 = (locals.var_t1_dn12 + locals.var_t1_dn12);

        let assign8340_e9300: f64 = (2.5 * locals.var_t1);
        locals.var_t3 = assign8340_e9300;
        locals.var_t3_dn3 = (2.5 * locals.var_t1_dn3);
        locals.var_t3_dn4 = (2.5 * locals.var_t1_dn4);
        locals.var_t3_dn5 = (2.5 * locals.var_t1_dn5);
        locals.var_t3_dn6 = (2.5 * locals.var_t1_dn6);
        locals.var_t3_dn7 = (2.5 * locals.var_t1_dn7);
        locals.var_t3_dn8 = (2.5 * locals.var_t1_dn8);
        locals.var_t3_dn9 = (2.5 * locals.var_t1_dn9);
        locals.var_t3_dn10 = (2.5 * locals.var_t1_dn10);
        locals.var_t3_dn11 = (2.5 * locals.var_t1_dn11);
        locals.var_t3_dn12 = (2.5 * locals.var_t1_dn12);

        let (assign8350_e9306, assign8350_e9306_d_n3, assign8350_e9306_d_n4, assign8350_e9306_d_n5, assign8350_e9306_d_n6, assign8350_e9306_d_n7, assign8350_e9306_d_n8, assign8350_e9306_d_n9, assign8350_e9306_d_n10, assign8350_e9306_d_n11, assign8350_e9306_d_n12,) = {
    if (p.p37 == 1.0) {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_b4soivtfbphi1 = assign8350_e9306;
        locals.var_b4soivtfbphi1_dn3 = assign8350_e9306_d_n3;
        locals.var_b4soivtfbphi1_dn4 = assign8350_e9306_d_n4;
        locals.var_b4soivtfbphi1_dn5 = assign8350_e9306_d_n5;
        locals.var_b4soivtfbphi1_dn6 = assign8350_e9306_d_n6;
        locals.var_b4soivtfbphi1_dn7 = assign8350_e9306_d_n7;
        locals.var_b4soivtfbphi1_dn8 = assign8350_e9306_d_n8;
        locals.var_b4soivtfbphi1_dn9 = assign8350_e9306_d_n9;
        locals.var_b4soivtfbphi1_dn10 = assign8350_e9306_d_n10;
        locals.var_b4soivtfbphi1_dn11 = assign8350_e9306_d_n11;
        locals.var_b4soivtfbphi1_dn12 = assign8350_e9306_d_n12;

        let assign8360_e9309: f64 = if locals.var_b4soivtfbphi1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard611 = assign8360_e9309;

        let (assign8370_e9313, assign8370_e9313_d_n3, assign8370_e9313_d_n4, assign8370_e9313_d_n5, assign8370_e9313_d_n6, assign8370_e9313_d_n7, assign8370_e9313_d_n8, assign8370_e9313_d_n9, assign8370_e9313_d_n10, assign8370_e9313_d_n11, assign8370_e9313_d_n12,) = {
    if (locals.var_guard611 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soivtfbphi1, locals.var_b4soivtfbphi1_dn3, locals.var_b4soivtfbphi1_dn4, locals.var_b4soivtfbphi1_dn5, locals.var_b4soivtfbphi1_dn6, locals.var_b4soivtfbphi1_dn7, locals.var_b4soivtfbphi1_dn8, locals.var_b4soivtfbphi1_dn9, locals.var_b4soivtfbphi1_dn10, locals.var_b4soivtfbphi1_dn11, locals.var_b4soivtfbphi1_dn12,)
    }
};
        locals.var_b4soivtfbphi1 = assign8370_e9313;
        locals.var_b4soivtfbphi1_dn3 = assign8370_e9313_d_n3;
        locals.var_b4soivtfbphi1_dn4 = assign8370_e9313_d_n4;
        locals.var_b4soivtfbphi1_dn5 = assign8370_e9313_d_n5;
        locals.var_b4soivtfbphi1_dn6 = assign8370_e9313_d_n6;
        locals.var_b4soivtfbphi1_dn7 = assign8370_e9313_d_n7;
        locals.var_b4soivtfbphi1_dn8 = assign8370_e9313_d_n8;
        locals.var_b4soivtfbphi1_dn9 = assign8370_e9313_d_n9;
        locals.var_b4soivtfbphi1_dn10 = assign8370_e9313_d_n10;
        locals.var_b4soivtfbphi1_dn11 = assign8370_e9313_d_n11;
        locals.var_b4soivtfbphi1_dn12 = assign8370_e9313_d_n12;

        let assign8380_e9316: f64 = if p.p62 == 4.0 { 1.0 } else { 0.0 };
        locals.var_guard612 = assign8380_e9316;

        let (assign8390_e9322, assign8390_e9322_d_n3, assign8390_e9322_d_n4, assign8390_e9322_d_n5, assign8390_e9322_d_n6, assign8390_e9322_d_n7, assign8390_e9322_d_n8, assign8390_e9322_d_n9, assign8390_e9322_d_n10, assign8390_e9322_d_n11, assign8390_e9322_d_n12,) = {
    if (locals.var_guard612 != 0.0) {
        let assign8390_e9320: f64 = (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0);
        (assign8390_e9320, (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn3), (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn4), (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn5), (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn6), (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn7), (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn8), (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn9), (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn10), (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn11), (locals.var_b4soifactor1 * locals.var_pparam_b4soisqrtxdep0_dn12),)
    } else {
        (locals.var_lt1, locals.var_lt1_dn3, locals.var_lt1_dn4, locals.var_lt1_dn5, locals.var_lt1_dn6, locals.var_lt1_dn7, locals.var_lt1_dn8, locals.var_lt1_dn9, locals.var_lt1_dn10, locals.var_lt1_dn11, locals.var_lt1_dn12,)
    }
};
        locals.var_lt1 = assign8390_e9322;
        locals.var_lt1_dn3 = assign8390_e9322_d_n3;
        locals.var_lt1_dn4 = assign8390_e9322_d_n4;
        locals.var_lt1_dn5 = assign8390_e9322_d_n5;
        locals.var_lt1_dn6 = assign8390_e9322_d_n6;
        locals.var_lt1_dn7 = assign8390_e9322_d_n7;
        locals.var_lt1_dn8 = assign8390_e9322_d_n8;
        locals.var_lt1_dn9 = assign8390_e9322_d_n9;
        locals.var_lt1_dn10 = assign8390_e9322_d_n10;
        locals.var_lt1_dn11 = assign8390_e9322_d_n11;
        locals.var_lt1_dn12 = assign8390_e9322_d_n12;

        let (assign8400_e9330, assign8400_e9330_d_n3, assign8400_e9330_d_n4, assign8400_e9330_d_n5, assign8400_e9330_d_n6, assign8400_e9330_d_n7, assign8400_e9330_d_n8, assign8400_e9330_d_n9, assign8400_e9330_d_n10, assign8400_e9330_d_n11, assign8400_e9330_d_n12,) = {
    if (locals.var_guard612 != 0.0) {
        let assign8400_e9326: f64 = (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff);
        let assign8400_e9328: f64 = (assign8400_e9326 / locals.var_lt1);
        (assign8400_e9328, (((((locals.var_pparam_b4soidvt1_dn3 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff_dn3)) * locals.var_lt1) - (assign8400_e9326 * locals.var_lt1_dn3)) / (locals.var_lt1 * locals.var_lt1)), (((((locals.var_pparam_b4soidvt1_dn4 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff_dn4)) * locals.var_lt1) - (assign8400_e9326 * locals.var_lt1_dn4)) / (locals.var_lt1 * locals.var_lt1)), (((((locals.var_pparam_b4soidvt1_dn5 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff_dn5)) * locals.var_lt1) - (assign8400_e9326 * locals.var_lt1_dn5)) / (locals.var_lt1 * locals.var_lt1)), (((((locals.var_pparam_b4soidvt1_dn6 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff_dn6)) * locals.var_lt1) - (assign8400_e9326 * locals.var_lt1_dn6)) / (locals.var_lt1 * locals.var_lt1)), (((((locals.var_pparam_b4soidvt1_dn7 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff_dn7)) * locals.var_lt1) - (assign8400_e9326 * locals.var_lt1_dn7)) / (locals.var_lt1 * locals.var_lt1)), (((((locals.var_pparam_b4soidvt1_dn8 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff_dn8)) * locals.var_lt1) - (assign8400_e9326 * locals.var_lt1_dn8)) / (locals.var_lt1 * locals.var_lt1)), (((((locals.var_pparam_b4soidvt1_dn9 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff_dn9)) * locals.var_lt1) - (assign8400_e9326 * locals.var_lt1_dn9)) / (locals.var_lt1 * locals.var_lt1)), (((((locals.var_pparam_b4soidvt1_dn10 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff_dn10)) * locals.var_lt1) - (assign8400_e9326 * locals.var_lt1_dn10)) / (locals.var_lt1 * locals.var_lt1)), (((((locals.var_pparam_b4soidvt1_dn11 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff_dn11)) * locals.var_lt1) - (assign8400_e9326 * locals.var_lt1_dn11)) / (locals.var_lt1 * locals.var_lt1)), (((((locals.var_pparam_b4soidvt1_dn12 * locals.var_pparam_b4soileff) + (locals.var_pparam_b4soidvt1 * locals.var_pparam_b4soileff_dn12)) * locals.var_lt1) - (assign8400_e9326 * locals.var_lt1_dn12)) / (locals.var_lt1 * locals.var_lt1)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign8400_e9330;
        locals.var_t0_dn3 = assign8400_e9330_d_n3;
        locals.var_t0_dn4 = assign8400_e9330_d_n4;
        locals.var_t0_dn5 = assign8400_e9330_d_n5;
        locals.var_t0_dn6 = assign8400_e9330_d_n6;
        locals.var_t0_dn7 = assign8400_e9330_d_n7;
        locals.var_t0_dn8 = assign8400_e9330_d_n8;
        locals.var_t0_dn9 = assign8400_e9330_d_n9;
        locals.var_t0_dn10 = assign8400_e9330_d_n10;
        locals.var_t0_dn11 = assign8400_e9330_d_n11;
        locals.var_t0_dn12 = assign8400_e9330_d_n12;

        let assign8410_e9333: f64 = if locals.var_t0 < 100.0 { 1.0 } else { 0.0 };
        locals.var_guard613 = assign8410_e9333;

        let (assign8420_e9340, assign8420_e9340_d_n3, assign8420_e9340_d_n4, assign8420_e9340_d_n5, assign8420_e9340_d_n6, assign8420_e9340_d_n7, assign8420_e9340_d_n8, assign8420_e9340_d_n9, assign8420_e9340_d_n10, assign8420_e9340_d_n11, assign8420_e9340_d_n12,) = {
    if ((locals.var_guard612 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign8420_e9338: f64 = (locals.var_t0).exp();
        (assign8420_e9338, (assign8420_e9338 * locals.var_t0_dn3), (assign8420_e9338 * locals.var_t0_dn4), (assign8420_e9338 * locals.var_t0_dn5), (assign8420_e9338 * locals.var_t0_dn6), (assign8420_e9338 * locals.var_t0_dn7), (assign8420_e9338 * locals.var_t0_dn8), (assign8420_e9338 * locals.var_t0_dn9), (assign8420_e9338 * locals.var_t0_dn10), (assign8420_e9338 * locals.var_t0_dn11), (assign8420_e9338 * locals.var_t0_dn12),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign8420_e9340;
        locals.var_t1_dn3 = assign8420_e9340_d_n3;
        locals.var_t1_dn4 = assign8420_e9340_d_n4;
        locals.var_t1_dn5 = assign8420_e9340_d_n5;
        locals.var_t1_dn6 = assign8420_e9340_d_n6;
        locals.var_t1_dn7 = assign8420_e9340_d_n7;
        locals.var_t1_dn8 = assign8420_e9340_d_n8;
        locals.var_t1_dn9 = assign8420_e9340_d_n9;
        locals.var_t1_dn10 = assign8420_e9340_d_n10;
        locals.var_t1_dn11 = assign8420_e9340_d_n11;
        locals.var_t1_dn12 = assign8420_e9340_d_n12;

        let (assign8430_e9348, assign8430_e9348_d_n3, assign8430_e9348_d_n4, assign8430_e9348_d_n5, assign8430_e9348_d_n6, assign8430_e9348_d_n7, assign8430_e9348_d_n8, assign8430_e9348_d_n9, assign8430_e9348_d_n10, assign8430_e9348_d_n11, assign8430_e9348_d_n12,) = {
    if ((locals.var_guard612 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign8430_e9346: f64 = (locals.var_t1 - 1.0);
        (assign8430_e9346, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign8430_e9348;
        locals.var_t2_dn3 = assign8430_e9348_d_n3;
        locals.var_t2_dn4 = assign8430_e9348_d_n4;
        locals.var_t2_dn5 = assign8430_e9348_d_n5;
        locals.var_t2_dn6 = assign8430_e9348_d_n6;
        locals.var_t2_dn7 = assign8430_e9348_d_n7;
        locals.var_t2_dn8 = assign8430_e9348_d_n8;
        locals.var_t2_dn9 = assign8430_e9348_d_n9;
        locals.var_t2_dn10 = assign8430_e9348_d_n10;
        locals.var_t2_dn11 = assign8430_e9348_d_n11;
        locals.var_t2_dn12 = assign8430_e9348_d_n12;

        let (assign8440_e9356, assign8440_e9356_d_n3, assign8440_e9356_d_n4, assign8440_e9356_d_n5, assign8440_e9356_d_n6, assign8440_e9356_d_n7, assign8440_e9356_d_n8, assign8440_e9356_d_n9, assign8440_e9356_d_n10, assign8440_e9356_d_n11, assign8440_e9356_d_n12,) = {
    if ((locals.var_guard612 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign8440_e9354: f64 = (locals.var_t2 * locals.var_t2);
        (assign8440_e9354, ((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)), ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)), ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)), ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)), ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)), ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)), ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)), ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)), ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)), ((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign8440_e9356;
        locals.var_t3_dn3 = assign8440_e9356_d_n3;
        locals.var_t3_dn4 = assign8440_e9356_d_n4;
        locals.var_t3_dn5 = assign8440_e9356_d_n5;
        locals.var_t3_dn6 = assign8440_e9356_d_n6;
        locals.var_t3_dn7 = assign8440_e9356_d_n7;
        locals.var_t3_dn8 = assign8440_e9356_d_n8;
        locals.var_t3_dn9 = assign8440_e9356_d_n9;
        locals.var_t3_dn10 = assign8440_e9356_d_n10;
        locals.var_t3_dn11 = assign8440_e9356_d_n11;
        locals.var_t3_dn12 = assign8440_e9356_d_n12;

        let (assign8450_e9368, assign8450_e9368_d_n3, assign8450_e9368_d_n4, assign8450_e9368_d_n5, assign8450_e9368_d_n6, assign8450_e9368_d_n7, assign8450_e9368_d_n8, assign8450_e9368_d_n9, assign8450_e9368_d_n10, assign8450_e9368_d_n11, assign8450_e9368_d_n12,) = {
    if ((locals.var_guard612 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign8450_e9363: f64 = (2.0 * locals.var_t1);
        let assign8450_e9365: f64 = (assign8450_e9363 * 3.720075976e-44);
        let assign8450_e9366: f64 = (locals.var_t3 + assign8450_e9365);
        (assign8450_e9366, (locals.var_t3_dn3 + ((2.0 * locals.var_t1_dn3) * 3.720075976e-44)), (locals.var_t3_dn4 + ((2.0 * locals.var_t1_dn4) * 3.720075976e-44)), (locals.var_t3_dn5 + ((2.0 * locals.var_t1_dn5) * 3.720075976e-44)), (locals.var_t3_dn6 + ((2.0 * locals.var_t1_dn6) * 3.720075976e-44)), (locals.var_t3_dn7 + ((2.0 * locals.var_t1_dn7) * 3.720075976e-44)), (locals.var_t3_dn8 + ((2.0 * locals.var_t1_dn8) * 3.720075976e-44)), (locals.var_t3_dn9 + ((2.0 * locals.var_t1_dn9) * 3.720075976e-44)), (locals.var_t3_dn10 + ((2.0 * locals.var_t1_dn10) * 3.720075976e-44)), (locals.var_t3_dn11 + ((2.0 * locals.var_t1_dn11) * 3.720075976e-44)), (locals.var_t3_dn12 + ((2.0 * locals.var_t1_dn12) * 3.720075976e-44)),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign8450_e9368;
        locals.var_t4_dn3 = assign8450_e9368_d_n3;
        locals.var_t4_dn4 = assign8450_e9368_d_n4;
        locals.var_t4_dn5 = assign8450_e9368_d_n5;
        locals.var_t4_dn6 = assign8450_e9368_d_n6;
        locals.var_t4_dn7 = assign8450_e9368_d_n7;
        locals.var_t4_dn8 = assign8450_e9368_d_n8;
        locals.var_t4_dn9 = assign8450_e9368_d_n9;
        locals.var_t4_dn10 = assign8450_e9368_d_n10;
        locals.var_t4_dn11 = assign8450_e9368_d_n11;
        locals.var_t4_dn12 = assign8450_e9368_d_n12;

        let (assign8460_e9376, assign8460_e9376_d_n3, assign8460_e9376_d_n4, assign8460_e9376_d_n5, assign8460_e9376_d_n6, assign8460_e9376_d_n7, assign8460_e9376_d_n8, assign8460_e9376_d_n9, assign8460_e9376_d_n10, assign8460_e9376_d_n11, assign8460_e9376_d_n12,) = {
    if ((locals.var_guard612 != 0.0) && (locals.var_guard613 != 0.0)) {
        let assign8460_e9374: f64 = (locals.var_t1 / locals.var_t4);
        (assign8460_e9374, (((locals.var_t1_dn3 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t1_dn4 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t1_dn5 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t1_dn6 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t1_dn7 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t1_dn8 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t1_dn9 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t1_dn10 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t1_dn11 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), (((locals.var_t1_dn12 * locals.var_t4) - (locals.var_t1 * locals.var_t4_dn12)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_theta0, locals.var_theta0_dn3, locals.var_theta0_dn4, locals.var_theta0_dn5, locals.var_theta0_dn6, locals.var_theta0_dn7, locals.var_theta0_dn8, locals.var_theta0_dn9, locals.var_theta0_dn10, locals.var_theta0_dn11, locals.var_theta0_dn12,)
    }
};
        locals.var_theta0 = assign8460_e9376;
        locals.var_theta0_dn3 = assign8460_e9376_d_n3;
        locals.var_theta0_dn4 = assign8460_e9376_d_n4;
        locals.var_theta0_dn5 = assign8460_e9376_d_n5;
        locals.var_theta0_dn6 = assign8460_e9376_d_n6;
        locals.var_theta0_dn7 = assign8460_e9376_d_n7;
        locals.var_theta0_dn8 = assign8460_e9376_d_n8;
        locals.var_theta0_dn9 = assign8460_e9376_d_n9;
        locals.var_theta0_dn10 = assign8460_e9376_d_n10;
        locals.var_theta0_dn11 = assign8460_e9376_d_n11;
        locals.var_theta0_dn12 = assign8460_e9376_d_n12;

        let (assign8470_e9387, assign8470_e9387_d_n3, assign8470_e9387_d_n4, assign8470_e9387_d_n5, assign8470_e9387_d_n6, assign8470_e9387_d_n7, assign8470_e9387_d_n8, assign8470_e9387_d_n9, assign8470_e9387_d_n10, assign8470_e9387_d_n11, assign8470_e9387_d_n12,) = {
    if ((locals.var_guard612 != 0.0) && (locals.var_guard613 == 0.0)) {
        let assign8470_e9384: f64 = (2.688117142e43 - 2.0);
        let assign8470_e9385: f64 = (1.0 / assign8470_e9384);
        (assign8470_e9385, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_theta0, locals.var_theta0_dn3, locals.var_theta0_dn4, locals.var_theta0_dn5, locals.var_theta0_dn6, locals.var_theta0_dn7, locals.var_theta0_dn8, locals.var_theta0_dn9, locals.var_theta0_dn10, locals.var_theta0_dn11, locals.var_theta0_dn12,)
    }
};
        locals.var_theta0 = assign8470_e9387;
        locals.var_theta0_dn3 = assign8470_e9387_d_n3;
        locals.var_theta0_dn4 = assign8470_e9387_d_n4;
        locals.var_theta0_dn5 = assign8470_e9387_d_n5;
        locals.var_theta0_dn6 = assign8470_e9387_d_n6;
        locals.var_theta0_dn7 = assign8470_e9387_d_n7;
        locals.var_theta0_dn8 = assign8470_e9387_d_n8;
        locals.var_theta0_dn9 = assign8470_e9387_d_n9;
        locals.var_theta0_dn10 = assign8470_e9387_d_n10;
        locals.var_theta0_dn11 = assign8470_e9387_d_n11;
        locals.var_theta0_dn12 = assign8470_e9387_d_n12;

        let (assign8480_e9393, assign8480_e9393_d_n3, assign8480_e9393_d_n4, assign8480_e9393_d_n5, assign8480_e9393_d_n6, assign8480_e9393_d_n7, assign8480_e9393_d_n8, assign8480_e9393_d_n9, assign8480_e9393_d_n10, assign8480_e9393_d_n11, assign8480_e9393_d_n12,) = {
    if (locals.var_guard612 != 0.0) {
        let assign8480_e9391: f64 = (locals.var_epssub / locals.var_pparam_b4soixdep0);
        (assign8480_e9391, (-((locals.var_epssub * locals.var_pparam_b4soixdep0_dn3) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0))), (-((locals.var_epssub * locals.var_pparam_b4soixdep0_dn4) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0))), (-((locals.var_epssub * locals.var_pparam_b4soixdep0_dn5) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0))), (-((locals.var_epssub * locals.var_pparam_b4soixdep0_dn6) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0))), (-((locals.var_epssub * locals.var_pparam_b4soixdep0_dn7) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0))), (-((locals.var_epssub * locals.var_pparam_b4soixdep0_dn8) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0))), (-((locals.var_epssub * locals.var_pparam_b4soixdep0_dn9) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0))), (-((locals.var_epssub * locals.var_pparam_b4soixdep0_dn10) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0))), (-((locals.var_epssub * locals.var_pparam_b4soixdep0_dn11) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0))), (-((locals.var_epssub * locals.var_pparam_b4soixdep0_dn12) / (locals.var_pparam_b4soixdep0 * locals.var_pparam_b4soixdep0))),)
    } else {
        (locals.var_tmp1, locals.var_tmp1_dn3, locals.var_tmp1_dn4, locals.var_tmp1_dn5, locals.var_tmp1_dn6, locals.var_tmp1_dn7, locals.var_tmp1_dn8, locals.var_tmp1_dn9, locals.var_tmp1_dn10, locals.var_tmp1_dn11, locals.var_tmp1_dn12,)
    }
};
        locals.var_tmp1 = assign8480_e9393;
        locals.var_tmp1_dn3 = assign8480_e9393_d_n3;
        locals.var_tmp1_dn4 = assign8480_e9393_d_n4;
        locals.var_tmp1_dn5 = assign8480_e9393_d_n5;
        locals.var_tmp1_dn6 = assign8480_e9393_d_n6;
        locals.var_tmp1_dn7 = assign8480_e9393_d_n7;
        locals.var_tmp1_dn8 = assign8480_e9393_d_n8;
        locals.var_tmp1_dn9 = assign8480_e9393_d_n9;
        locals.var_tmp1_dn10 = assign8480_e9393_d_n10;
        locals.var_tmp1_dn11 = assign8480_e9393_d_n11;
        locals.var_tmp1_dn12 = assign8480_e9393_d_n12;

        let (assign8490_e9399, assign8490_e9399_d_n3, assign8490_e9399_d_n4, assign8490_e9399_d_n5, assign8490_e9399_d_n6, assign8490_e9399_d_n7, assign8490_e9399_d_n8, assign8490_e9399_d_n9, assign8490_e9399_d_n10, assign8490_e9399_d_n11, assign8490_e9399_d_n12,) = {
    if (locals.var_guard612 != 0.0) {
        let assign8490_e9397: f64 = (locals.var_pparam_b4soinfactor * locals.var_tmp1);
        (assign8490_e9397, ((locals.var_pparam_b4soinfactor_dn3 * locals.var_tmp1) + (locals.var_pparam_b4soinfactor * locals.var_tmp1_dn3)), ((locals.var_pparam_b4soinfactor_dn4 * locals.var_tmp1) + (locals.var_pparam_b4soinfactor * locals.var_tmp1_dn4)), ((locals.var_pparam_b4soinfactor_dn5 * locals.var_tmp1) + (locals.var_pparam_b4soinfactor * locals.var_tmp1_dn5)), ((locals.var_pparam_b4soinfactor_dn6 * locals.var_tmp1) + (locals.var_pparam_b4soinfactor * locals.var_tmp1_dn6)), ((locals.var_pparam_b4soinfactor_dn7 * locals.var_tmp1) + (locals.var_pparam_b4soinfactor * locals.var_tmp1_dn7)), ((locals.var_pparam_b4soinfactor_dn8 * locals.var_tmp1) + (locals.var_pparam_b4soinfactor * locals.var_tmp1_dn8)), ((locals.var_pparam_b4soinfactor_dn9 * locals.var_tmp1) + (locals.var_pparam_b4soinfactor * locals.var_tmp1_dn9)), ((locals.var_pparam_b4soinfactor_dn10 * locals.var_tmp1) + (locals.var_pparam_b4soinfactor * locals.var_tmp1_dn10)), ((locals.var_pparam_b4soinfactor_dn11 * locals.var_tmp1) + (locals.var_pparam_b4soinfactor * locals.var_tmp1_dn11)), ((locals.var_pparam_b4soinfactor_dn12 * locals.var_tmp1) + (locals.var_pparam_b4soinfactor * locals.var_tmp1_dn12)),)
    } else {
        (locals.var_tmp2, locals.var_tmp2_dn3, locals.var_tmp2_dn4, locals.var_tmp2_dn5, locals.var_tmp2_dn6, locals.var_tmp2_dn7, locals.var_tmp2_dn8, locals.var_tmp2_dn9, locals.var_tmp2_dn10, locals.var_tmp2_dn11, locals.var_tmp2_dn12,)
    }
};
        locals.var_tmp2 = assign8490_e9399;
        locals.var_tmp2_dn3 = assign8490_e9399_d_n3;
        locals.var_tmp2_dn4 = assign8490_e9399_d_n4;
        locals.var_tmp2_dn5 = assign8490_e9399_d_n5;
        locals.var_tmp2_dn6 = assign8490_e9399_d_n6;
        locals.var_tmp2_dn7 = assign8490_e9399_d_n7;
        locals.var_tmp2_dn8 = assign8490_e9399_d_n8;
        locals.var_tmp2_dn9 = assign8490_e9399_d_n9;
        locals.var_tmp2_dn10 = assign8490_e9399_d_n10;
        locals.var_tmp2_dn11 = assign8490_e9399_d_n11;
        locals.var_tmp2_dn12 = assign8490_e9399_d_n12;

        let (assign8500_e9411, assign8500_e9411_d_n3, assign8500_e9411_d_n4, assign8500_e9411_d_n5, assign8500_e9411_d_n6, assign8500_e9411_d_n7, assign8500_e9411_d_n8, assign8500_e9411_d_n9, assign8500_e9411_d_n10, assign8500_e9411_d_n11, assign8500_e9411_d_n12,) = {
    if (locals.var_guard612 != 0.0) {
        let assign8500_e9404: f64 = (locals.var_pparam_b4soicdsc * locals.var_theta0);
        let assign8500_e9405: f64 = (locals.var_tmp2 + assign8500_e9404);
        let assign8500_e9407: f64 = (assign8500_e9405 + locals.var_pparam_b4soicit);
        let assign8500_e9409: f64 = (assign8500_e9407 / locals.var_b4soicox);
        (assign8500_e9409, (((locals.var_tmp2_dn3 + ((locals.var_pparam_b4soicdsc_dn3 * locals.var_theta0) + (locals.var_pparam_b4soicdsc * locals.var_theta0_dn3))) + locals.var_pparam_b4soicit_dn3) / locals.var_b4soicox), (((locals.var_tmp2_dn4 + ((locals.var_pparam_b4soicdsc_dn4 * locals.var_theta0) + (locals.var_pparam_b4soicdsc * locals.var_theta0_dn4))) + locals.var_pparam_b4soicit_dn4) / locals.var_b4soicox), (((locals.var_tmp2_dn5 + ((locals.var_pparam_b4soicdsc_dn5 * locals.var_theta0) + (locals.var_pparam_b4soicdsc * locals.var_theta0_dn5))) + locals.var_pparam_b4soicit_dn5) / locals.var_b4soicox), (((locals.var_tmp2_dn6 + ((locals.var_pparam_b4soicdsc_dn6 * locals.var_theta0) + (locals.var_pparam_b4soicdsc * locals.var_theta0_dn6))) + locals.var_pparam_b4soicit_dn6) / locals.var_b4soicox), (((locals.var_tmp2_dn7 + ((locals.var_pparam_b4soicdsc_dn7 * locals.var_theta0) + (locals.var_pparam_b4soicdsc * locals.var_theta0_dn7))) + locals.var_pparam_b4soicit_dn7) / locals.var_b4soicox), (((locals.var_tmp2_dn8 + ((locals.var_pparam_b4soicdsc_dn8 * locals.var_theta0) + (locals.var_pparam_b4soicdsc * locals.var_theta0_dn8))) + locals.var_pparam_b4soicit_dn8) / locals.var_b4soicox), (((locals.var_tmp2_dn9 + ((locals.var_pparam_b4soicdsc_dn9 * locals.var_theta0) + (locals.var_pparam_b4soicdsc * locals.var_theta0_dn9))) + locals.var_pparam_b4soicit_dn9) / locals.var_b4soicox), (((locals.var_tmp2_dn10 + ((locals.var_pparam_b4soicdsc_dn10 * locals.var_theta0) + (locals.var_pparam_b4soicdsc * locals.var_theta0_dn10))) + locals.var_pparam_b4soicit_dn10) / locals.var_b4soicox), (((locals.var_tmp2_dn11 + ((locals.var_pparam_b4soicdsc_dn11 * locals.var_theta0) + (locals.var_pparam_b4soicdsc * locals.var_theta0_dn11))) + locals.var_pparam_b4soicit_dn11) / locals.var_b4soicox), (((locals.var_tmp2_dn12 + ((locals.var_pparam_b4soicdsc_dn12 * locals.var_theta0) + (locals.var_pparam_b4soicdsc * locals.var_theta0_dn12))) + locals.var_pparam_b4soicit_dn12) / locals.var_b4soicox),)
    } else {
        (locals.var_tmp3, locals.var_tmp3_dn3, locals.var_tmp3_dn4, locals.var_tmp3_dn5, locals.var_tmp3_dn6, locals.var_tmp3_dn7, locals.var_tmp3_dn8, locals.var_tmp3_dn9, locals.var_tmp3_dn10, locals.var_tmp3_dn11, locals.var_tmp3_dn12,)
    }
};
        locals.var_tmp3 = assign8500_e9411;
        locals.var_tmp3_dn3 = assign8500_e9411_d_n3;
        locals.var_tmp3_dn4 = assign8500_e9411_d_n4;
        locals.var_tmp3_dn5 = assign8500_e9411_d_n5;
        locals.var_tmp3_dn6 = assign8500_e9411_d_n6;
        locals.var_tmp3_dn7 = assign8500_e9411_d_n7;
        locals.var_tmp3_dn8 = assign8500_e9411_d_n8;
        locals.var_tmp3_dn9 = assign8500_e9411_d_n9;
        locals.var_tmp3_dn10 = assign8500_e9411_d_n10;
        locals.var_tmp3_dn11 = assign8500_e9411_d_n11;
        locals.var_tmp3_dn12 = assign8500_e9411_d_n12;

        let assign8510_e9414: f64 = (-0.5);
        let assign8510_e9415: f64 = if locals.var_tmp3 >= assign8510_e9414 { 1.0 } else { 0.0 };
        locals.var_guard614 = assign8510_e9415;

        let (assign8520_e9423, assign8520_e9423_d_n3, assign8520_e9423_d_n4, assign8520_e9423_d_n5, assign8520_e9423_d_n6, assign8520_e9423_d_n7, assign8520_e9423_d_n8, assign8520_e9423_d_n9, assign8520_e9423_d_n10, assign8520_e9423_d_n11, assign8520_e9423_d_n12,) = {
    if ((locals.var_guard612 != 0.0) && (locals.var_guard614 != 0.0)) {
        let assign8520_e9421: f64 = (1.0 + locals.var_tmp3);
        (assign8520_e9421, locals.var_tmp3_dn3, locals.var_tmp3_dn4, locals.var_tmp3_dn5, locals.var_tmp3_dn6, locals.var_tmp3_dn7, locals.var_tmp3_dn8, locals.var_tmp3_dn9, locals.var_tmp3_dn10, locals.var_tmp3_dn11, locals.var_tmp3_dn12,)
    } else {
        (locals.var_n0, locals.var_n0_dn3, locals.var_n0_dn4, locals.var_n0_dn5, locals.var_n0_dn6, locals.var_n0_dn7, locals.var_n0_dn8, locals.var_n0_dn9, locals.var_n0_dn10, locals.var_n0_dn11, locals.var_n0_dn12,)
    }
};
        locals.var_n0 = assign8520_e9423;
        locals.var_n0_dn3 = assign8520_e9423_d_n3;
        locals.var_n0_dn4 = assign8520_e9423_d_n4;
        locals.var_n0_dn5 = assign8520_e9423_d_n5;
        locals.var_n0_dn6 = assign8520_e9423_d_n6;
        locals.var_n0_dn7 = assign8520_e9423_d_n7;
        locals.var_n0_dn8 = assign8520_e9423_d_n8;
        locals.var_n0_dn9 = assign8520_e9423_d_n9;
        locals.var_n0_dn10 = assign8520_e9423_d_n10;
        locals.var_n0_dn11 = assign8520_e9423_d_n11;
        locals.var_n0_dn12 = assign8520_e9423_d_n12;

        let (assign8530_e9436, assign8530_e9436_d_n3, assign8530_e9436_d_n4, assign8530_e9436_d_n5, assign8530_e9436_d_n6, assign8530_e9436_d_n7, assign8530_e9436_d_n8, assign8530_e9436_d_n9, assign8530_e9436_d_n10, assign8530_e9436_d_n11, assign8530_e9436_d_n12,) = {
    if ((locals.var_guard612 != 0.0) && (locals.var_guard614 == 0.0)) {
        let assign8530_e9432: f64 = (8.0 * locals.var_tmp3);
        let assign8530_e9433: f64 = (3.0 + assign8530_e9432);
        let assign8530_e9434: f64 = (1.0 / assign8530_e9433);
        (assign8530_e9434, (-((8.0 * locals.var_tmp3_dn3) / (assign8530_e9433 * assign8530_e9433))), (-((8.0 * locals.var_tmp3_dn4) / (assign8530_e9433 * assign8530_e9433))), (-((8.0 * locals.var_tmp3_dn5) / (assign8530_e9433 * assign8530_e9433))), (-((8.0 * locals.var_tmp3_dn6) / (assign8530_e9433 * assign8530_e9433))), (-((8.0 * locals.var_tmp3_dn7) / (assign8530_e9433 * assign8530_e9433))), (-((8.0 * locals.var_tmp3_dn8) / (assign8530_e9433 * assign8530_e9433))), (-((8.0 * locals.var_tmp3_dn9) / (assign8530_e9433 * assign8530_e9433))), (-((8.0 * locals.var_tmp3_dn10) / (assign8530_e9433 * assign8530_e9433))), (-((8.0 * locals.var_tmp3_dn11) / (assign8530_e9433 * assign8530_e9433))), (-((8.0 * locals.var_tmp3_dn12) / (assign8530_e9433 * assign8530_e9433))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign8530_e9436;
        locals.var_t0_dn3 = assign8530_e9436_d_n3;
        locals.var_t0_dn4 = assign8530_e9436_d_n4;
        locals.var_t0_dn5 = assign8530_e9436_d_n5;
        locals.var_t0_dn6 = assign8530_e9436_d_n6;
        locals.var_t0_dn7 = assign8530_e9436_d_n7;
        locals.var_t0_dn8 = assign8530_e9436_d_n8;
        locals.var_t0_dn9 = assign8530_e9436_d_n9;
        locals.var_t0_dn10 = assign8530_e9436_d_n10;
        locals.var_t0_dn11 = assign8530_e9436_d_n11;
        locals.var_t0_dn12 = assign8530_e9436_d_n12;

    }

    pub(super) fn stamp_transient_block_25(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign8540_e9449, assign8540_e9449_d_n3, assign8540_e9449_d_n4, assign8540_e9449_d_n5, assign8540_e9449_d_n6, assign8540_e9449_d_n7, assign8540_e9449_d_n8, assign8540_e9449_d_n9, assign8540_e9449_d_n10, assign8540_e9449_d_n11, assign8540_e9449_d_n12,) = {
    if ((locals.var_guard612 != 0.0) && (locals.var_guard614 == 0.0)) {
        let assign8540_e9444: f64 = (3.0 * locals.var_tmp3);
        let assign8540_e9445: f64 = (1.0 + assign8540_e9444);
        let assign8540_e9447: f64 = (assign8540_e9445 * locals.var_t0);
        (assign8540_e9447, (((3.0 * locals.var_tmp3_dn3) * locals.var_t0) + (assign8540_e9445 * locals.var_t0_dn3)), (((3.0 * locals.var_tmp3_dn4) * locals.var_t0) + (assign8540_e9445 * locals.var_t0_dn4)), (((3.0 * locals.var_tmp3_dn5) * locals.var_t0) + (assign8540_e9445 * locals.var_t0_dn5)), (((3.0 * locals.var_tmp3_dn6) * locals.var_t0) + (assign8540_e9445 * locals.var_t0_dn6)), (((3.0 * locals.var_tmp3_dn7) * locals.var_t0) + (assign8540_e9445 * locals.var_t0_dn7)), (((3.0 * locals.var_tmp3_dn8) * locals.var_t0) + (assign8540_e9445 * locals.var_t0_dn8)), (((3.0 * locals.var_tmp3_dn9) * locals.var_t0) + (assign8540_e9445 * locals.var_t0_dn9)), (((3.0 * locals.var_tmp3_dn10) * locals.var_t0) + (assign8540_e9445 * locals.var_t0_dn10)), (((3.0 * locals.var_tmp3_dn11) * locals.var_t0) + (assign8540_e9445 * locals.var_t0_dn11)), (((3.0 * locals.var_tmp3_dn12) * locals.var_t0) + (assign8540_e9445 * locals.var_t0_dn12)),)
    } else {
        (locals.var_n0, locals.var_n0_dn3, locals.var_n0_dn4, locals.var_n0_dn5, locals.var_n0_dn6, locals.var_n0_dn7, locals.var_n0_dn8, locals.var_n0_dn9, locals.var_n0_dn10, locals.var_n0_dn11, locals.var_n0_dn12,)
    }
};
        locals.var_n0 = assign8540_e9449;
        locals.var_n0_dn3 = assign8540_e9449_d_n3;
        locals.var_n0_dn4 = assign8540_e9449_d_n4;
        locals.var_n0_dn5 = assign8540_e9449_d_n5;
        locals.var_n0_dn6 = assign8540_e9449_d_n6;
        locals.var_n0_dn7 = assign8540_e9449_d_n7;
        locals.var_n0_dn8 = assign8540_e9449_d_n8;
        locals.var_n0_dn9 = assign8540_e9449_d_n9;
        locals.var_n0_dn10 = assign8540_e9449_d_n10;
        locals.var_n0_dn11 = assign8540_e9449_d_n11;
        locals.var_n0_dn12 = assign8540_e9449_d_n12;

        let (assign8550_e9455, assign8550_e9455_d_n3, assign8550_e9455_d_n4, assign8550_e9455_d_n5, assign8550_e9455_d_n6, assign8550_e9455_d_n7, assign8550_e9455_d_n8, assign8550_e9455_d_n9, assign8550_e9455_d_n10, assign8550_e9455_d_n11, assign8550_e9455_d_n12,) = {
    if (locals.var_guard612 != 0.0) {
        let assign8550_e9453: f64 = (locals.var_n0 * locals.var_vtm0);
        (assign8550_e9453, (locals.var_n0_dn3 * locals.var_vtm0), (locals.var_n0_dn4 * locals.var_vtm0), (locals.var_n0_dn5 * locals.var_vtm0), (locals.var_n0_dn6 * locals.var_vtm0), (locals.var_n0_dn7 * locals.var_vtm0), (locals.var_n0_dn8 * locals.var_vtm0), (locals.var_n0_dn9 * locals.var_vtm0), (locals.var_n0_dn10 * locals.var_vtm0), (locals.var_n0_dn11 * locals.var_vtm0), (locals.var_n0_dn12 * locals.var_vtm0),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12,)
    }
};
        locals.var_t0 = assign8550_e9455;
        locals.var_t0_dn3 = assign8550_e9455_d_n3;
        locals.var_t0_dn4 = assign8550_e9455_d_n4;
        locals.var_t0_dn5 = assign8550_e9455_d_n5;
        locals.var_t0_dn6 = assign8550_e9455_d_n6;
        locals.var_t0_dn7 = assign8550_e9455_d_n7;
        locals.var_t0_dn8 = assign8550_e9455_d_n8;
        locals.var_t0_dn9 = assign8550_e9455_d_n9;
        locals.var_t0_dn10 = assign8550_e9455_d_n10;
        locals.var_t0_dn11 = assign8550_e9455_d_n11;
        locals.var_t0_dn12 = assign8550_e9455_d_n12;

        let (assign8560_e9459, assign8560_e9459_d_n3, assign8560_e9459_d_n4, assign8560_e9459_d_n5, assign8560_e9459_d_n6, assign8560_e9459_d_n7, assign8560_e9459_d_n8, assign8560_e9459_d_n9, assign8560_e9459_d_n10, assign8560_e9459_d_n11, assign8560_e9459_d_n12,) = {
    if (locals.var_guard612 != 0.0) {
        (locals.var_pparam_b4soivoff, locals.var_pparam_b4soivoff_dn3, locals.var_pparam_b4soivoff_dn4, locals.var_pparam_b4soivoff_dn5, locals.var_pparam_b4soivoff_dn6, locals.var_pparam_b4soivoff_dn7, locals.var_pparam_b4soivoff_dn8, locals.var_pparam_b4soivoff_dn9, locals.var_pparam_b4soivoff_dn10, locals.var_pparam_b4soivoff_dn11, locals.var_pparam_b4soivoff_dn12,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12,)
    }
};
        locals.var_t1 = assign8560_e9459;
        locals.var_t1_dn3 = assign8560_e9459_d_n3;
        locals.var_t1_dn4 = assign8560_e9459_d_n4;
        locals.var_t1_dn5 = assign8560_e9459_d_n5;
        locals.var_t1_dn6 = assign8560_e9459_d_n6;
        locals.var_t1_dn7 = assign8560_e9459_d_n7;
        locals.var_t1_dn8 = assign8560_e9459_d_n8;
        locals.var_t1_dn9 = assign8560_e9459_d_n9;
        locals.var_t1_dn10 = assign8560_e9459_d_n10;
        locals.var_t1_dn11 = assign8560_e9459_d_n11;
        locals.var_t1_dn12 = assign8560_e9459_d_n12;

        let (assign8570_e9465, assign8570_e9465_d_n3, assign8570_e9465_d_n4, assign8570_e9465_d_n5, assign8570_e9465_d_n6, assign8570_e9465_d_n7, assign8570_e9465_d_n8, assign8570_e9465_d_n9, assign8570_e9465_d_n10, assign8570_e9465_d_n11, assign8570_e9465_d_n12,) = {
    if (locals.var_guard612 != 0.0) {
        let assign8570_e9463: f64 = (locals.var_t1 / locals.var_t0);
        (assign8570_e9463, (((locals.var_t1_dn3 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t1_dn4 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t1_dn5 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t1_dn6 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t1_dn7 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t1_dn8 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t1_dn9 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t1_dn10 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t1_dn11 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_t1_dn12 * locals.var_t0) - (locals.var_t1 * locals.var_t0_dn12)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12,)
    }
};
        locals.var_t2 = assign8570_e9465;
        locals.var_t2_dn3 = assign8570_e9465_d_n3;
        locals.var_t2_dn4 = assign8570_e9465_d_n4;
        locals.var_t2_dn5 = assign8570_e9465_d_n5;
        locals.var_t2_dn6 = assign8570_e9465_d_n6;
        locals.var_t2_dn7 = assign8570_e9465_d_n7;
        locals.var_t2_dn8 = assign8570_e9465_d_n8;
        locals.var_t2_dn9 = assign8570_e9465_d_n9;
        locals.var_t2_dn10 = assign8570_e9465_d_n10;
        locals.var_t2_dn11 = assign8570_e9465_d_n11;
        locals.var_t2_dn12 = assign8570_e9465_d_n12;

        let assign8580_e9468: f64 = (-100.0);
        let assign8580_e9469: f64 = if locals.var_t2 < assign8580_e9468 { 1.0 } else { 0.0 };
        locals.var_guard615 = assign8580_e9469;

        let (assign8590_e9479, assign8590_e9479_d_n3, assign8590_e9479_d_n4, assign8590_e9479_d_n5, assign8590_e9479_d_n6, assign8590_e9479_d_n7, assign8590_e9479_d_n8, assign8590_e9479_d_n9, assign8590_e9479_d_n10, assign8590_e9479_d_n11, assign8590_e9479_d_n12,) = {
    if ((locals.var_guard612 != 0.0) && (locals.var_guard615 != 0.0)) {
        let assign8590_e9475: f64 = (locals.var_b4soicox * 3.720075976e-44);
        let assign8590_e9477: f64 = (assign8590_e9475 / locals.var_pparam_b4soicdep0);
        (assign8590_e9477, (-((assign8590_e9475 * locals.var_pparam_b4soicdep0_dn3) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign8590_e9475 * locals.var_pparam_b4soicdep0_dn4) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign8590_e9475 * locals.var_pparam_b4soicdep0_dn5) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign8590_e9475 * locals.var_pparam_b4soicdep0_dn6) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign8590_e9475 * locals.var_pparam_b4soicdep0_dn7) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign8590_e9475 * locals.var_pparam_b4soicdep0_dn8) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign8590_e9475 * locals.var_pparam_b4soicdep0_dn9) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign8590_e9475 * locals.var_pparam_b4soicdep0_dn10) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign8590_e9475 * locals.var_pparam_b4soicdep0_dn11) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign8590_e9475 * locals.var_pparam_b4soicdep0_dn12) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign8590_e9479;
        locals.var_t3_dn3 = assign8590_e9479_d_n3;
        locals.var_t3_dn4 = assign8590_e9479_d_n4;
        locals.var_t3_dn5 = assign8590_e9479_d_n5;
        locals.var_t3_dn6 = assign8590_e9479_d_n6;
        locals.var_t3_dn7 = assign8590_e9479_d_n7;
        locals.var_t3_dn8 = assign8590_e9479_d_n8;
        locals.var_t3_dn9 = assign8590_e9479_d_n9;
        locals.var_t3_dn10 = assign8590_e9479_d_n10;
        locals.var_t3_dn11 = assign8590_e9479_d_n11;
        locals.var_t3_dn12 = assign8590_e9479_d_n12;

        let (assign8600_e9489, assign8600_e9489_d_n3, assign8600_e9489_d_n4, assign8600_e9489_d_n5, assign8600_e9489_d_n6, assign8600_e9489_d_n7, assign8600_e9489_d_n8, assign8600_e9489_d_n9, assign8600_e9489_d_n10, assign8600_e9489_d_n11, assign8600_e9489_d_n12,) = {
    if ((locals.var_guard612 != 0.0) && (locals.var_guard615 != 0.0)) {
        let assign8600_e9486: f64 = (locals.var_t3 * locals.var_n0);
        let assign8600_e9487: f64 = (locals.var_pparam_b4soimstar + assign8600_e9486);
        (assign8600_e9487, (locals.var_pparam_b4soimstar_dn3 + ((locals.var_t3_dn3 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn3))), (locals.var_pparam_b4soimstar_dn4 + ((locals.var_t3_dn4 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn4))), (locals.var_pparam_b4soimstar_dn5 + ((locals.var_t3_dn5 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn5))), (locals.var_pparam_b4soimstar_dn6 + ((locals.var_t3_dn6 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn6))), (locals.var_pparam_b4soimstar_dn7 + ((locals.var_t3_dn7 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn7))), (locals.var_pparam_b4soimstar_dn8 + ((locals.var_t3_dn8 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn8))), (locals.var_pparam_b4soimstar_dn9 + ((locals.var_t3_dn9 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn9))), (locals.var_pparam_b4soimstar_dn10 + ((locals.var_t3_dn10 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn10))), (locals.var_pparam_b4soimstar_dn11 + ((locals.var_t3_dn11 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn11))), (locals.var_pparam_b4soimstar_dn12 + ((locals.var_t3_dn12 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn12))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign8600_e9489;
        locals.var_t4_dn3 = assign8600_e9489_d_n3;
        locals.var_t4_dn4 = assign8600_e9489_d_n4;
        locals.var_t4_dn5 = assign8600_e9489_d_n5;
        locals.var_t4_dn6 = assign8600_e9489_d_n6;
        locals.var_t4_dn7 = assign8600_e9489_d_n7;
        locals.var_t4_dn8 = assign8600_e9489_d_n8;
        locals.var_t4_dn9 = assign8600_e9489_d_n9;
        locals.var_t4_dn10 = assign8600_e9489_d_n10;
        locals.var_t4_dn11 = assign8600_e9489_d_n11;
        locals.var_t4_dn12 = assign8600_e9489_d_n12;

        let assign8610_e9492: f64 = if locals.var_t2 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard616 = assign8610_e9492;

        let (assign8620_e9505, assign8620_e9505_d_n3, assign8620_e9505_d_n4, assign8620_e9505_d_n5, assign8620_e9505_d_n6, assign8620_e9505_d_n7, assign8620_e9505_d_n8, assign8620_e9505_d_n9, assign8620_e9505_d_n10, assign8620_e9505_d_n11, assign8620_e9505_d_n12,) = {
    if (((locals.var_guard612 != 0.0) && (locals.var_guard615 == 0.0)) && (locals.var_guard616 != 0.0)) {
        let assign8620_e9501: f64 = (locals.var_b4soicox * 2.688117142e43);
        let assign8620_e9503: f64 = (assign8620_e9501 / locals.var_pparam_b4soicdep0);
        (assign8620_e9503, (-((assign8620_e9501 * locals.var_pparam_b4soicdep0_dn3) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign8620_e9501 * locals.var_pparam_b4soicdep0_dn4) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign8620_e9501 * locals.var_pparam_b4soicdep0_dn5) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign8620_e9501 * locals.var_pparam_b4soicdep0_dn6) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign8620_e9501 * locals.var_pparam_b4soicdep0_dn7) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign8620_e9501 * locals.var_pparam_b4soicdep0_dn8) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign8620_e9501 * locals.var_pparam_b4soicdep0_dn9) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign8620_e9501 * locals.var_pparam_b4soicdep0_dn10) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign8620_e9501 * locals.var_pparam_b4soicdep0_dn11) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))), (-((assign8620_e9501 * locals.var_pparam_b4soicdep0_dn12) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign8620_e9505;
        locals.var_t3_dn3 = assign8620_e9505_d_n3;
        locals.var_t3_dn4 = assign8620_e9505_d_n4;
        locals.var_t3_dn5 = assign8620_e9505_d_n5;
        locals.var_t3_dn6 = assign8620_e9505_d_n6;
        locals.var_t3_dn7 = assign8620_e9505_d_n7;
        locals.var_t3_dn8 = assign8620_e9505_d_n8;
        locals.var_t3_dn9 = assign8620_e9505_d_n9;
        locals.var_t3_dn10 = assign8620_e9505_d_n10;
        locals.var_t3_dn11 = assign8620_e9505_d_n11;
        locals.var_t3_dn12 = assign8620_e9505_d_n12;

        let (assign8630_e9518, assign8630_e9518_d_n3, assign8630_e9518_d_n4, assign8630_e9518_d_n5, assign8630_e9518_d_n6, assign8630_e9518_d_n7, assign8630_e9518_d_n8, assign8630_e9518_d_n9, assign8630_e9518_d_n10, assign8630_e9518_d_n11, assign8630_e9518_d_n12,) = {
    if (((locals.var_guard612 != 0.0) && (locals.var_guard615 == 0.0)) && (locals.var_guard616 != 0.0)) {
        let assign8630_e9515: f64 = (locals.var_t3 * locals.var_n0);
        let assign8630_e9516: f64 = (locals.var_pparam_b4soimstar + assign8630_e9515);
        (assign8630_e9516, (locals.var_pparam_b4soimstar_dn3 + ((locals.var_t3_dn3 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn3))), (locals.var_pparam_b4soimstar_dn4 + ((locals.var_t3_dn4 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn4))), (locals.var_pparam_b4soimstar_dn5 + ((locals.var_t3_dn5 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn5))), (locals.var_pparam_b4soimstar_dn6 + ((locals.var_t3_dn6 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn6))), (locals.var_pparam_b4soimstar_dn7 + ((locals.var_t3_dn7 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn7))), (locals.var_pparam_b4soimstar_dn8 + ((locals.var_t3_dn8 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn8))), (locals.var_pparam_b4soimstar_dn9 + ((locals.var_t3_dn9 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn9))), (locals.var_pparam_b4soimstar_dn10 + ((locals.var_t3_dn10 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn10))), (locals.var_pparam_b4soimstar_dn11 + ((locals.var_t3_dn11 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn11))), (locals.var_pparam_b4soimstar_dn12 + ((locals.var_t3_dn12 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn12))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign8630_e9518;
        locals.var_t4_dn3 = assign8630_e9518_d_n3;
        locals.var_t4_dn4 = assign8630_e9518_d_n4;
        locals.var_t4_dn5 = assign8630_e9518_d_n5;
        locals.var_t4_dn6 = assign8630_e9518_d_n6;
        locals.var_t4_dn7 = assign8630_e9518_d_n7;
        locals.var_t4_dn8 = assign8630_e9518_d_n8;
        locals.var_t4_dn9 = assign8630_e9518_d_n9;
        locals.var_t4_dn10 = assign8630_e9518_d_n10;
        locals.var_t4_dn11 = assign8630_e9518_d_n11;
        locals.var_t4_dn12 = assign8630_e9518_d_n12;

        let (assign8640_e9533, assign8640_e9533_d_n3, assign8640_e9533_d_n4, assign8640_e9533_d_n5, assign8640_e9533_d_n6, assign8640_e9533_d_n7, assign8640_e9533_d_n8, assign8640_e9533_d_n9, assign8640_e9533_d_n10, assign8640_e9533_d_n11, assign8640_e9533_d_n12,) = {
    if (((locals.var_guard612 != 0.0) && (locals.var_guard615 == 0.0)) && (locals.var_guard616 == 0.0)) {
        let assign8640_e9527: f64 = (locals.var_t2).exp();
        let assign8640_e9529: f64 = (assign8640_e9527 * locals.var_b4soicox);
        let assign8640_e9531: f64 = (assign8640_e9529 / locals.var_pparam_b4soicdep0);
        (assign8640_e9531, (((((assign8640_e9527 * locals.var_t2_dn3) * locals.var_b4soicox) * locals.var_pparam_b4soicdep0) - (assign8640_e9529 * locals.var_pparam_b4soicdep0_dn3)) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0)), (((((assign8640_e9527 * locals.var_t2_dn4) * locals.var_b4soicox) * locals.var_pparam_b4soicdep0) - (assign8640_e9529 * locals.var_pparam_b4soicdep0_dn4)) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0)), (((((assign8640_e9527 * locals.var_t2_dn5) * locals.var_b4soicox) * locals.var_pparam_b4soicdep0) - (assign8640_e9529 * locals.var_pparam_b4soicdep0_dn5)) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0)), (((((assign8640_e9527 * locals.var_t2_dn6) * locals.var_b4soicox) * locals.var_pparam_b4soicdep0) - (assign8640_e9529 * locals.var_pparam_b4soicdep0_dn6)) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0)), (((((assign8640_e9527 * locals.var_t2_dn7) * locals.var_b4soicox) * locals.var_pparam_b4soicdep0) - (assign8640_e9529 * locals.var_pparam_b4soicdep0_dn7)) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0)), (((((assign8640_e9527 * locals.var_t2_dn8) * locals.var_b4soicox) * locals.var_pparam_b4soicdep0) - (assign8640_e9529 * locals.var_pparam_b4soicdep0_dn8)) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0)), (((((assign8640_e9527 * locals.var_t2_dn9) * locals.var_b4soicox) * locals.var_pparam_b4soicdep0) - (assign8640_e9529 * locals.var_pparam_b4soicdep0_dn9)) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0)), (((((assign8640_e9527 * locals.var_t2_dn10) * locals.var_b4soicox) * locals.var_pparam_b4soicdep0) - (assign8640_e9529 * locals.var_pparam_b4soicdep0_dn10)) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0)), (((((assign8640_e9527 * locals.var_t2_dn11) * locals.var_b4soicox) * locals.var_pparam_b4soicdep0) - (assign8640_e9529 * locals.var_pparam_b4soicdep0_dn11)) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0)), (((((assign8640_e9527 * locals.var_t2_dn12) * locals.var_b4soicox) * locals.var_pparam_b4soicdep0) - (assign8640_e9529 * locals.var_pparam_b4soicdep0_dn12)) / (locals.var_pparam_b4soicdep0 * locals.var_pparam_b4soicdep0)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12,)
    }
};
        locals.var_t3 = assign8640_e9533;
        locals.var_t3_dn3 = assign8640_e9533_d_n3;
        locals.var_t3_dn4 = assign8640_e9533_d_n4;
        locals.var_t3_dn5 = assign8640_e9533_d_n5;
        locals.var_t3_dn6 = assign8640_e9533_d_n6;
        locals.var_t3_dn7 = assign8640_e9533_d_n7;
        locals.var_t3_dn8 = assign8640_e9533_d_n8;
        locals.var_t3_dn9 = assign8640_e9533_d_n9;
        locals.var_t3_dn10 = assign8640_e9533_d_n10;
        locals.var_t3_dn11 = assign8640_e9533_d_n11;
        locals.var_t3_dn12 = assign8640_e9533_d_n12;

        let (assign8650_e9547, assign8650_e9547_d_n3, assign8650_e9547_d_n4, assign8650_e9547_d_n5, assign8650_e9547_d_n6, assign8650_e9547_d_n7, assign8650_e9547_d_n8, assign8650_e9547_d_n9, assign8650_e9547_d_n10, assign8650_e9547_d_n11, assign8650_e9547_d_n12,) = {
    if (((locals.var_guard612 != 0.0) && (locals.var_guard615 == 0.0)) && (locals.var_guard616 == 0.0)) {
        let assign8650_e9544: f64 = (locals.var_t3 * locals.var_n0);
        let assign8650_e9545: f64 = (locals.var_pparam_b4soimstar + assign8650_e9544);
        (assign8650_e9545, (locals.var_pparam_b4soimstar_dn3 + ((locals.var_t3_dn3 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn3))), (locals.var_pparam_b4soimstar_dn4 + ((locals.var_t3_dn4 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn4))), (locals.var_pparam_b4soimstar_dn5 + ((locals.var_t3_dn5 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn5))), (locals.var_pparam_b4soimstar_dn6 + ((locals.var_t3_dn6 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn6))), (locals.var_pparam_b4soimstar_dn7 + ((locals.var_t3_dn7 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn7))), (locals.var_pparam_b4soimstar_dn8 + ((locals.var_t3_dn8 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn8))), (locals.var_pparam_b4soimstar_dn9 + ((locals.var_t3_dn9 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn9))), (locals.var_pparam_b4soimstar_dn10 + ((locals.var_t3_dn10 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn10))), (locals.var_pparam_b4soimstar_dn11 + ((locals.var_t3_dn11 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn11))), (locals.var_pparam_b4soimstar_dn12 + ((locals.var_t3_dn12 * locals.var_n0) + (locals.var_t3 * locals.var_n0_dn12))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12,)
    }
};
        locals.var_t4 = assign8650_e9547;
        locals.var_t4_dn3 = assign8650_e9547_d_n3;
        locals.var_t4_dn4 = assign8650_e9547_d_n4;
        locals.var_t4_dn5 = assign8650_e9547_d_n5;
        locals.var_t4_dn6 = assign8650_e9547_d_n6;
        locals.var_t4_dn7 = assign8650_e9547_d_n7;
        locals.var_t4_dn8 = assign8650_e9547_d_n8;
        locals.var_t4_dn9 = assign8650_e9547_d_n9;
        locals.var_t4_dn10 = assign8650_e9547_d_n10;
        locals.var_t4_dn11 = assign8650_e9547_d_n11;
        locals.var_t4_dn12 = assign8650_e9547_d_n12;

        let (assign8660_e9555, assign8660_e9555_d_n3, assign8660_e9555_d_n4, assign8660_e9555_d_n5, assign8660_e9555_d_n6, assign8660_e9555_d_n7, assign8660_e9555_d_n8, assign8660_e9555_d_n9, assign8660_e9555_d_n10, assign8660_e9555_d_n11, assign8660_e9555_d_n12,) = {
    if (locals.var_guard612 != 0.0) {
        let assign8660_e9551: f64 = (locals.var_t0 * 0.6931471805599453);
        let assign8660_e9553: f64 = (assign8660_e9551 / locals.var_t4);
        (assign8660_e9553, ((((locals.var_t0_dn3 * 0.6931471805599453) * locals.var_t4) - (assign8660_e9551 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)), ((((locals.var_t0_dn4 * 0.6931471805599453) * locals.var_t4) - (assign8660_e9551 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)), ((((locals.var_t0_dn5 * 0.6931471805599453) * locals.var_t4) - (assign8660_e9551 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)), ((((locals.var_t0_dn6 * 0.6931471805599453) * locals.var_t4) - (assign8660_e9551 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)), ((((locals.var_t0_dn7 * 0.6931471805599453) * locals.var_t4) - (assign8660_e9551 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)), ((((locals.var_t0_dn8 * 0.6931471805599453) * locals.var_t4) - (assign8660_e9551 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)), ((((locals.var_t0_dn9 * 0.6931471805599453) * locals.var_t4) - (assign8660_e9551 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)), ((((locals.var_t0_dn10 * 0.6931471805599453) * locals.var_t4) - (assign8660_e9551 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)), ((((locals.var_t0_dn11 * 0.6931471805599453) * locals.var_t4) - (assign8660_e9551 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)), ((((locals.var_t0_dn12 * 0.6931471805599453) * locals.var_t4) - (assign8660_e9551 * locals.var_t4_dn12)) / (locals.var_t4 * locals.var_t4)),)
    } else {
        (locals.var_b4soivgsteffvth, locals.var_b4soivgsteffvth_dn3, locals.var_b4soivgsteffvth_dn4, locals.var_b4soivgsteffvth_dn5, locals.var_b4soivgsteffvth_dn6, locals.var_b4soivgsteffvth_dn7, locals.var_b4soivgsteffvth_dn8, locals.var_b4soivgsteffvth_dn9, locals.var_b4soivgsteffvth_dn10, locals.var_b4soivgsteffvth_dn11, locals.var_b4soivgsteffvth_dn12,)
    }
};
        locals.var_b4soivgsteffvth = assign8660_e9555;
        locals.var_b4soivgsteffvth_dn3 = assign8660_e9555_d_n3;
        locals.var_b4soivgsteffvth_dn4 = assign8660_e9555_d_n4;
        locals.var_b4soivgsteffvth_dn5 = assign8660_e9555_d_n5;
        locals.var_b4soivgsteffvth_dn6 = assign8660_e9555_d_n6;
        locals.var_b4soivgsteffvth_dn7 = assign8660_e9555_d_n7;
        locals.var_b4soivgsteffvth_dn8 = assign8660_e9555_d_n8;
        locals.var_b4soivgsteffvth_dn9 = assign8660_e9555_d_n9;
        locals.var_b4soivgsteffvth_dn10 = assign8660_e9555_d_n10;
        locals.var_b4soivgsteffvth_dn11 = assign8660_e9555_d_n11;
        locals.var_b4soivgsteffvth_dn12 = assign8660_e9555_d_n12;

        let (assign8670_e9560, assign8670_e9560_d_n3, assign8670_e9560_d_n4, assign8670_e9560_d_n5, assign8670_e9560_d_n6, assign8670_e9560_d_n7, assign8670_e9560_d_n8, assign8670_e9560_d_n9, assign8670_e9560_d_n10, assign8670_e9560_d_n11, assign8670_e9560_d_n12,) = {
    if (locals.var_guard612 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soivgsteffvth, locals.var_b4soivgsteffvth_dn3, locals.var_b4soivgsteffvth_dn4, locals.var_b4soivgsteffvth_dn5, locals.var_b4soivgsteffvth_dn6, locals.var_b4soivgsteffvth_dn7, locals.var_b4soivgsteffvth_dn8, locals.var_b4soivgsteffvth_dn9, locals.var_b4soivgsteffvth_dn10, locals.var_b4soivgsteffvth_dn11, locals.var_b4soivgsteffvth_dn12,)
    }
};
        locals.var_b4soivgsteffvth = assign8670_e9560;
        locals.var_b4soivgsteffvth_dn3 = assign8670_e9560_d_n3;
        locals.var_b4soivgsteffvth_dn4 = assign8670_e9560_d_n4;
        locals.var_b4soivgsteffvth_dn5 = assign8670_e9560_d_n5;
        locals.var_b4soivgsteffvth_dn6 = assign8670_e9560_d_n6;
        locals.var_b4soivgsteffvth_dn7 = assign8670_e9560_d_n7;
        locals.var_b4soivgsteffvth_dn8 = assign8670_e9560_d_n8;
        locals.var_b4soivgsteffvth_dn9 = assign8670_e9560_d_n9;
        locals.var_b4soivgsteffvth_dn10 = assign8670_e9560_d_n10;
        locals.var_b4soivgsteffvth_dn11 = assign8670_e9560_d_n11;
        locals.var_b4soivgsteffvth_dn12 = assign8670_e9560_d_n12;

        let assign9560_e9905: f64 = if ((p.p38 >= 4.4) || (p.p63 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard669 = assign9560_e9905;

        let assign9570_e9908: f64 = if locals.var_pparam_b4soia2 < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard670 = assign9570_e9908;

        let (assign9580_e9914, assign9580_e9914_d_n3, assign9580_e9914_d_n4, assign9580_e9914_d_n5, assign9580_e9914_d_n6, assign9580_e9914_d_n7, assign9580_e9914_d_n8, assign9580_e9914_d_n9, assign9580_e9914_d_n10, assign9580_e9914_d_n11, assign9580_e9914_d_n12,) = {
    if ((locals.var_guard669 != 0.0) && (locals.var_guard670 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soia2, locals.var_pparam_b4soia2_dn3, locals.var_pparam_b4soia2_dn4, locals.var_pparam_b4soia2_dn5, locals.var_pparam_b4soia2_dn6, locals.var_pparam_b4soia2_dn7, locals.var_pparam_b4soia2_dn8, locals.var_pparam_b4soia2_dn9, locals.var_pparam_b4soia2_dn10, locals.var_pparam_b4soia2_dn11, locals.var_pparam_b4soia2_dn12,)
    }
};
        locals.var_pparam_b4soia2 = assign9580_e9914;
        locals.var_pparam_b4soia2_dn3 = assign9580_e9914_d_n3;
        locals.var_pparam_b4soia2_dn4 = assign9580_e9914_d_n4;
        locals.var_pparam_b4soia2_dn5 = assign9580_e9914_d_n5;
        locals.var_pparam_b4soia2_dn6 = assign9580_e9914_d_n6;
        locals.var_pparam_b4soia2_dn7 = assign9580_e9914_d_n7;
        locals.var_pparam_b4soia2_dn8 = assign9580_e9914_d_n8;
        locals.var_pparam_b4soia2_dn9 = assign9580_e9914_d_n9;
        locals.var_pparam_b4soia2_dn10 = assign9580_e9914_d_n10;
        locals.var_pparam_b4soia2_dn11 = assign9580_e9914_d_n11;
        locals.var_pparam_b4soia2_dn12 = assign9580_e9914_d_n12;

        let assign9590_e9917: f64 = if locals.var_pparam_b4soia2 > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard671 = assign9590_e9917;

        let (assign9600_e9926, assign9600_e9926_d_n3, assign9600_e9926_d_n4, assign9600_e9926_d_n5, assign9600_e9926_d_n6, assign9600_e9926_d_n7, assign9600_e9926_d_n8, assign9600_e9926_d_n9, assign9600_e9926_d_n10, assign9600_e9926_d_n11, assign9600_e9926_d_n12,) = {
    if (((locals.var_guard669 != 0.0) && (locals.var_guard670 == 0.0)) && (locals.var_guard671 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soia2, locals.var_pparam_b4soia2_dn3, locals.var_pparam_b4soia2_dn4, locals.var_pparam_b4soia2_dn5, locals.var_pparam_b4soia2_dn6, locals.var_pparam_b4soia2_dn7, locals.var_pparam_b4soia2_dn8, locals.var_pparam_b4soia2_dn9, locals.var_pparam_b4soia2_dn10, locals.var_pparam_b4soia2_dn11, locals.var_pparam_b4soia2_dn12,)
    }
};
        locals.var_pparam_b4soia2 = assign9600_e9926;
        locals.var_pparam_b4soia2_dn3 = assign9600_e9926_d_n3;
        locals.var_pparam_b4soia2_dn4 = assign9600_e9926_d_n4;
        locals.var_pparam_b4soia2_dn5 = assign9600_e9926_d_n5;
        locals.var_pparam_b4soia2_dn6 = assign9600_e9926_d_n6;
        locals.var_pparam_b4soia2_dn7 = assign9600_e9926_d_n7;
        locals.var_pparam_b4soia2_dn8 = assign9600_e9926_d_n8;
        locals.var_pparam_b4soia2_dn9 = assign9600_e9926_d_n9;
        locals.var_pparam_b4soia2_dn10 = assign9600_e9926_d_n10;
        locals.var_pparam_b4soia2_dn11 = assign9600_e9926_d_n11;
        locals.var_pparam_b4soia2_dn12 = assign9600_e9926_d_n12;

        let (assign9610_e9935, assign9610_e9935_d_n3, assign9610_e9935_d_n4, assign9610_e9935_d_n5, assign9610_e9935_d_n6, assign9610_e9935_d_n7, assign9610_e9935_d_n8, assign9610_e9935_d_n9, assign9610_e9935_d_n10, assign9610_e9935_d_n11, assign9610_e9935_d_n12,) = {
    if (((locals.var_guard669 != 0.0) && (locals.var_guard670 == 0.0)) && (locals.var_guard671 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soia1, locals.var_pparam_b4soia1_dn3, locals.var_pparam_b4soia1_dn4, locals.var_pparam_b4soia1_dn5, locals.var_pparam_b4soia1_dn6, locals.var_pparam_b4soia1_dn7, locals.var_pparam_b4soia1_dn8, locals.var_pparam_b4soia1_dn9, locals.var_pparam_b4soia1_dn10, locals.var_pparam_b4soia1_dn11, locals.var_pparam_b4soia1_dn12,)
    }
};
        locals.var_pparam_b4soia1 = assign9610_e9935;
        locals.var_pparam_b4soia1_dn3 = assign9610_e9935_d_n3;
        locals.var_pparam_b4soia1_dn4 = assign9610_e9935_d_n4;
        locals.var_pparam_b4soia1_dn5 = assign9610_e9935_d_n5;
        locals.var_pparam_b4soia1_dn6 = assign9610_e9935_d_n6;
        locals.var_pparam_b4soia1_dn7 = assign9610_e9935_d_n7;
        locals.var_pparam_b4soia1_dn8 = assign9610_e9935_d_n8;
        locals.var_pparam_b4soia1_dn9 = assign9610_e9935_d_n9;
        locals.var_pparam_b4soia1_dn10 = assign9610_e9935_d_n10;
        locals.var_pparam_b4soia1_dn11 = assign9610_e9935_d_n11;
        locals.var_pparam_b4soia1_dn12 = assign9610_e9935_d_n12;

        let assign9620_e9938: f64 = if locals.var_pparam_b4soirdsw < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard672 = assign9620_e9938;

        let (assign9630_e9942, assign9630_e9942_d_n3, assign9630_e9942_d_n4, assign9630_e9942_d_n5, assign9630_e9942_d_n6, assign9630_e9942_d_n7, assign9630_e9942_d_n8, assign9630_e9942_d_n9, assign9630_e9942_d_n10, assign9630_e9942_d_n11, assign9630_e9942_d_n12,) = {
    if (locals.var_guard672 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soirdsw, locals.var_pparam_b4soirdsw_dn3, locals.var_pparam_b4soirdsw_dn4, locals.var_pparam_b4soirdsw_dn5, locals.var_pparam_b4soirdsw_dn6, locals.var_pparam_b4soirdsw_dn7, locals.var_pparam_b4soirdsw_dn8, locals.var_pparam_b4soirdsw_dn9, locals.var_pparam_b4soirdsw_dn10, locals.var_pparam_b4soirdsw_dn11, locals.var_pparam_b4soirdsw_dn12,)
    }
};
        locals.var_pparam_b4soirdsw = assign9630_e9942;
        locals.var_pparam_b4soirdsw_dn3 = assign9630_e9942_d_n3;
        locals.var_pparam_b4soirdsw_dn4 = assign9630_e9942_d_n4;
        locals.var_pparam_b4soirdsw_dn5 = assign9630_e9942_d_n5;
        locals.var_pparam_b4soirdsw_dn6 = assign9630_e9942_d_n6;
        locals.var_pparam_b4soirdsw_dn7 = assign9630_e9942_d_n7;
        locals.var_pparam_b4soirdsw_dn8 = assign9630_e9942_d_n8;
        locals.var_pparam_b4soirdsw_dn9 = assign9630_e9942_d_n9;
        locals.var_pparam_b4soirdsw_dn10 = assign9630_e9942_d_n10;
        locals.var_pparam_b4soirdsw_dn11 = assign9630_e9942_d_n11;
        locals.var_pparam_b4soirdsw_dn12 = assign9630_e9942_d_n12;

        let (assign9640_e9946, assign9640_e9946_d_n3, assign9640_e9946_d_n4, assign9640_e9946_d_n5, assign9640_e9946_d_n6, assign9640_e9946_d_n7, assign9640_e9946_d_n8, assign9640_e9946_d_n9, assign9640_e9946_d_n10, assign9640_e9946_d_n11, assign9640_e9946_d_n12,) = {
    if (locals.var_guard672 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soirds0, locals.var_pparam_b4soirds0_dn3, locals.var_pparam_b4soirds0_dn4, locals.var_pparam_b4soirds0_dn5, locals.var_pparam_b4soirds0_dn6, locals.var_pparam_b4soirds0_dn7, locals.var_pparam_b4soirds0_dn8, locals.var_pparam_b4soirds0_dn9, locals.var_pparam_b4soirds0_dn10, locals.var_pparam_b4soirds0_dn11, locals.var_pparam_b4soirds0_dn12,)
    }
};
        locals.var_pparam_b4soirds0 = assign9640_e9946;
        locals.var_pparam_b4soirds0_dn3 = assign9640_e9946_d_n3;
        locals.var_pparam_b4soirds0_dn4 = assign9640_e9946_d_n4;
        locals.var_pparam_b4soirds0_dn5 = assign9640_e9946_d_n5;
        locals.var_pparam_b4soirds0_dn6 = assign9640_e9946_d_n6;
        locals.var_pparam_b4soirds0_dn7 = assign9640_e9946_d_n7;
        locals.var_pparam_b4soirds0_dn8 = assign9640_e9946_d_n8;
        locals.var_pparam_b4soirds0_dn9 = assign9640_e9946_d_n9;
        locals.var_pparam_b4soirds0_dn10 = assign9640_e9946_d_n10;
        locals.var_pparam_b4soirds0_dn11 = assign9640_e9946_d_n11;
        locals.var_pparam_b4soirds0_dn12 = assign9640_e9946_d_n12;

        let assign9650_e9953: f64 = if ((locals.var_pparam_b4soirds0 < 0.001) && (locals.var_pparam_b4soirds0 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard673 = assign9650_e9953;

        let (assign9660_e9960, assign9660_e9960_d_n3, assign9660_e9960_d_n4, assign9660_e9960_d_n5, assign9660_e9960_d_n6, assign9660_e9960_d_n7, assign9660_e9960_d_n8, assign9660_e9960_d_n9, assign9660_e9960_d_n10, assign9660_e9960_d_n11, assign9660_e9960_d_n12,) = {
    if ((locals.var_guard672 == 0.0) && (locals.var_guard673 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soirds0, locals.var_pparam_b4soirds0_dn3, locals.var_pparam_b4soirds0_dn4, locals.var_pparam_b4soirds0_dn5, locals.var_pparam_b4soirds0_dn6, locals.var_pparam_b4soirds0_dn7, locals.var_pparam_b4soirds0_dn8, locals.var_pparam_b4soirds0_dn9, locals.var_pparam_b4soirds0_dn10, locals.var_pparam_b4soirds0_dn11, locals.var_pparam_b4soirds0_dn12,)
    }
};
        locals.var_pparam_b4soirds0 = assign9660_e9960;
        locals.var_pparam_b4soirds0_dn3 = assign9660_e9960_d_n3;
        locals.var_pparam_b4soirds0_dn4 = assign9660_e9960_d_n4;
        locals.var_pparam_b4soirds0_dn5 = assign9660_e9960_d_n5;
        locals.var_pparam_b4soirds0_dn6 = assign9660_e9960_d_n6;
        locals.var_pparam_b4soirds0_dn7 = assign9660_e9960_d_n7;
        locals.var_pparam_b4soirds0_dn8 = assign9660_e9960_d_n8;
        locals.var_pparam_b4soirds0_dn9 = assign9660_e9960_d_n9;
        locals.var_pparam_b4soirds0_dn10 = assign9660_e9960_d_n10;
        locals.var_pparam_b4soirds0_dn11 = assign9660_e9960_d_n11;
        locals.var_pparam_b4soirds0_dn12 = assign9660_e9960_d_n12;

        let assign9960_e10065: f64 = if locals.var_pparam_b4soiisdif < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard703 = assign9960_e10065;

        let (assign9970_e10071, assign9970_e10071_d_n3, assign9970_e10071_d_n4, assign9970_e10071_d_n5, assign9970_e10071_d_n6, assign9970_e10071_d_n7, assign9970_e10071_d_n8, assign9970_e10071_d_n9, assign9970_e10071_d_n10, assign9970_e10071_d_n11, assign9970_e10071_d_n12,) = {
    if ((p.p63 != 0.0) && (locals.var_guard703 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soiisdif, locals.var_pparam_b4soiisdif_dn3, locals.var_pparam_b4soiisdif_dn4, locals.var_pparam_b4soiisdif_dn5, locals.var_pparam_b4soiisdif_dn6, locals.var_pparam_b4soiisdif_dn7, locals.var_pparam_b4soiisdif_dn8, locals.var_pparam_b4soiisdif_dn9, locals.var_pparam_b4soiisdif_dn10, locals.var_pparam_b4soiisdif_dn11, locals.var_pparam_b4soiisdif_dn12,)
    }
};
        locals.var_pparam_b4soiisdif = assign9970_e10071;
        locals.var_pparam_b4soiisdif_dn3 = assign9970_e10071_d_n3;
        locals.var_pparam_b4soiisdif_dn4 = assign9970_e10071_d_n4;
        locals.var_pparam_b4soiisdif_dn5 = assign9970_e10071_d_n5;
        locals.var_pparam_b4soiisdif_dn6 = assign9970_e10071_d_n6;
        locals.var_pparam_b4soiisdif_dn7 = assign9970_e10071_d_n7;
        locals.var_pparam_b4soiisdif_dn8 = assign9970_e10071_d_n8;
        locals.var_pparam_b4soiisdif_dn9 = assign9970_e10071_d_n9;
        locals.var_pparam_b4soiisdif_dn10 = assign9970_e10071_d_n10;
        locals.var_pparam_b4soiisdif_dn11 = assign9970_e10071_d_n11;
        locals.var_pparam_b4soiisdif_dn12 = assign9970_e10071_d_n12;

        let assign9980_e10074: f64 = if locals.var_pparam_b4soiiddif < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard704 = assign9980_e10074;

        let (assign9990_e10080, assign9990_e10080_d_n3, assign9990_e10080_d_n4, assign9990_e10080_d_n5, assign9990_e10080_d_n6, assign9990_e10080_d_n7, assign9990_e10080_d_n8, assign9990_e10080_d_n9, assign9990_e10080_d_n10, assign9990_e10080_d_n11, assign9990_e10080_d_n12,) = {
    if ((p.p63 != 0.0) && (locals.var_guard704 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soiiddif, locals.var_pparam_b4soiiddif_dn3, locals.var_pparam_b4soiiddif_dn4, locals.var_pparam_b4soiiddif_dn5, locals.var_pparam_b4soiiddif_dn6, locals.var_pparam_b4soiiddif_dn7, locals.var_pparam_b4soiiddif_dn8, locals.var_pparam_b4soiiddif_dn9, locals.var_pparam_b4soiiddif_dn10, locals.var_pparam_b4soiiddif_dn11, locals.var_pparam_b4soiiddif_dn12,)
    }
};
        locals.var_pparam_b4soiiddif = assign9990_e10080;
        locals.var_pparam_b4soiiddif_dn3 = assign9990_e10080_d_n3;
        locals.var_pparam_b4soiiddif_dn4 = assign9990_e10080_d_n4;
        locals.var_pparam_b4soiiddif_dn5 = assign9990_e10080_d_n5;
        locals.var_pparam_b4soiiddif_dn6 = assign9990_e10080_d_n6;
        locals.var_pparam_b4soiiddif_dn7 = assign9990_e10080_d_n7;
        locals.var_pparam_b4soiiddif_dn8 = assign9990_e10080_d_n8;
        locals.var_pparam_b4soiiddif_dn9 = assign9990_e10080_d_n9;
        locals.var_pparam_b4soiiddif_dn10 = assign9990_e10080_d_n10;
        locals.var_pparam_b4soiiddif_dn11 = assign9990_e10080_d_n11;
        locals.var_pparam_b4soiiddif_dn12 = assign9990_e10080_d_n12;

        let assign10000_e10083: f64 = if locals.var_pparam_b4soiisrec < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard705 = assign10000_e10083;

        let (assign10010_e10089, assign10010_e10089_d_n3, assign10010_e10089_d_n4, assign10010_e10089_d_n5, assign10010_e10089_d_n6, assign10010_e10089_d_n7, assign10010_e10089_d_n8, assign10010_e10089_d_n9, assign10010_e10089_d_n10, assign10010_e10089_d_n11, assign10010_e10089_d_n12,) = {
    if ((p.p63 != 0.0) && (locals.var_guard705 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soiisrec, locals.var_pparam_b4soiisrec_dn3, locals.var_pparam_b4soiisrec_dn4, locals.var_pparam_b4soiisrec_dn5, locals.var_pparam_b4soiisrec_dn6, locals.var_pparam_b4soiisrec_dn7, locals.var_pparam_b4soiisrec_dn8, locals.var_pparam_b4soiisrec_dn9, locals.var_pparam_b4soiisrec_dn10, locals.var_pparam_b4soiisrec_dn11, locals.var_pparam_b4soiisrec_dn12,)
    }
};
        locals.var_pparam_b4soiisrec = assign10010_e10089;
        locals.var_pparam_b4soiisrec_dn3 = assign10010_e10089_d_n3;
        locals.var_pparam_b4soiisrec_dn4 = assign10010_e10089_d_n4;
        locals.var_pparam_b4soiisrec_dn5 = assign10010_e10089_d_n5;
        locals.var_pparam_b4soiisrec_dn6 = assign10010_e10089_d_n6;
        locals.var_pparam_b4soiisrec_dn7 = assign10010_e10089_d_n7;
        locals.var_pparam_b4soiisrec_dn8 = assign10010_e10089_d_n8;
        locals.var_pparam_b4soiisrec_dn9 = assign10010_e10089_d_n9;
        locals.var_pparam_b4soiisrec_dn10 = assign10010_e10089_d_n10;
        locals.var_pparam_b4soiisrec_dn11 = assign10010_e10089_d_n11;
        locals.var_pparam_b4soiisrec_dn12 = assign10010_e10089_d_n12;

        let assign10020_e10092: f64 = if locals.var_pparam_b4soiidrec < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard706 = assign10020_e10092;

        let (assign10030_e10098, assign10030_e10098_d_n3, assign10030_e10098_d_n4, assign10030_e10098_d_n5, assign10030_e10098_d_n6, assign10030_e10098_d_n7, assign10030_e10098_d_n8, assign10030_e10098_d_n9, assign10030_e10098_d_n10, assign10030_e10098_d_n11, assign10030_e10098_d_n12,) = {
    if ((p.p63 != 0.0) && (locals.var_guard706 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soiidrec, locals.var_pparam_b4soiidrec_dn3, locals.var_pparam_b4soiidrec_dn4, locals.var_pparam_b4soiidrec_dn5, locals.var_pparam_b4soiidrec_dn6, locals.var_pparam_b4soiidrec_dn7, locals.var_pparam_b4soiidrec_dn8, locals.var_pparam_b4soiidrec_dn9, locals.var_pparam_b4soiidrec_dn10, locals.var_pparam_b4soiidrec_dn11, locals.var_pparam_b4soiidrec_dn12,)
    }
};
        locals.var_pparam_b4soiidrec = assign10030_e10098;
        locals.var_pparam_b4soiidrec_dn3 = assign10030_e10098_d_n3;
        locals.var_pparam_b4soiidrec_dn4 = assign10030_e10098_d_n4;
        locals.var_pparam_b4soiidrec_dn5 = assign10030_e10098_d_n5;
        locals.var_pparam_b4soiidrec_dn6 = assign10030_e10098_d_n6;
        locals.var_pparam_b4soiidrec_dn7 = assign10030_e10098_d_n7;
        locals.var_pparam_b4soiidrec_dn8 = assign10030_e10098_d_n8;
        locals.var_pparam_b4soiidrec_dn9 = assign10030_e10098_d_n9;
        locals.var_pparam_b4soiidrec_dn10 = assign10030_e10098_d_n10;
        locals.var_pparam_b4soiidrec_dn11 = assign10030_e10098_d_n11;
        locals.var_pparam_b4soiidrec_dn12 = assign10030_e10098_d_n12;

        let assign10040_e10101: f64 = if locals.var_pparam_b4soiistun < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard707 = assign10040_e10101;

        let (assign10050_e10107, assign10050_e10107_d_n3, assign10050_e10107_d_n4, assign10050_e10107_d_n5, assign10050_e10107_d_n6, assign10050_e10107_d_n7, assign10050_e10107_d_n8, assign10050_e10107_d_n9, assign10050_e10107_d_n10, assign10050_e10107_d_n11, assign10050_e10107_d_n12,) = {
    if ((p.p63 != 0.0) && (locals.var_guard707 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soiistun, locals.var_pparam_b4soiistun_dn3, locals.var_pparam_b4soiistun_dn4, locals.var_pparam_b4soiistun_dn5, locals.var_pparam_b4soiistun_dn6, locals.var_pparam_b4soiistun_dn7, locals.var_pparam_b4soiistun_dn8, locals.var_pparam_b4soiistun_dn9, locals.var_pparam_b4soiistun_dn10, locals.var_pparam_b4soiistun_dn11, locals.var_pparam_b4soiistun_dn12,)
    }
};
        locals.var_pparam_b4soiistun = assign10050_e10107;
        locals.var_pparam_b4soiistun_dn3 = assign10050_e10107_d_n3;
        locals.var_pparam_b4soiistun_dn4 = assign10050_e10107_d_n4;
        locals.var_pparam_b4soiistun_dn5 = assign10050_e10107_d_n5;
        locals.var_pparam_b4soiistun_dn6 = assign10050_e10107_d_n6;
        locals.var_pparam_b4soiistun_dn7 = assign10050_e10107_d_n7;
        locals.var_pparam_b4soiistun_dn8 = assign10050_e10107_d_n8;
        locals.var_pparam_b4soiistun_dn9 = assign10050_e10107_d_n9;
        locals.var_pparam_b4soiistun_dn10 = assign10050_e10107_d_n10;
        locals.var_pparam_b4soiistun_dn11 = assign10050_e10107_d_n11;
        locals.var_pparam_b4soiistun_dn12 = assign10050_e10107_d_n12;

        let assign10060_e10110: f64 = if locals.var_pparam_b4soiidtun < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard708 = assign10060_e10110;

    }

    pub(super) fn stamp_transient_block_26(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let (assign10070_e10116, assign10070_e10116_d_n3, assign10070_e10116_d_n4, assign10070_e10116_d_n5, assign10070_e10116_d_n6, assign10070_e10116_d_n7, assign10070_e10116_d_n8, assign10070_e10116_d_n9, assign10070_e10116_d_n10, assign10070_e10116_d_n11, assign10070_e10116_d_n12,) = {
    if ((p.p63 != 0.0) && (locals.var_guard708 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pparam_b4soiidtun, locals.var_pparam_b4soiidtun_dn3, locals.var_pparam_b4soiidtun_dn4, locals.var_pparam_b4soiidtun_dn5, locals.var_pparam_b4soiidtun_dn6, locals.var_pparam_b4soiidtun_dn7, locals.var_pparam_b4soiidtun_dn8, locals.var_pparam_b4soiidtun_dn9, locals.var_pparam_b4soiidtun_dn10, locals.var_pparam_b4soiidtun_dn11, locals.var_pparam_b4soiidtun_dn12,)
    }
};
        locals.var_pparam_b4soiidtun = assign10070_e10116;
        locals.var_pparam_b4soiidtun_dn3 = assign10070_e10116_d_n3;
        locals.var_pparam_b4soiidtun_dn4 = assign10070_e10116_d_n4;
        locals.var_pparam_b4soiidtun_dn5 = assign10070_e10116_d_n5;
        locals.var_pparam_b4soiidtun_dn6 = assign10070_e10116_d_n6;
        locals.var_pparam_b4soiidtun_dn7 = assign10070_e10116_d_n7;
        locals.var_pparam_b4soiidtun_dn8 = assign10070_e10116_d_n8;
        locals.var_pparam_b4soiidtun_dn9 = assign10070_e10116_d_n9;
        locals.var_pparam_b4soiidtun_dn10 = assign10070_e10116_d_n10;
        locals.var_pparam_b4soiidtun_dn11 = assign10070_e10116_d_n11;
        locals.var_pparam_b4soiidtun_dn12 = assign10070_e10116_d_n12;

        locals.var_deltemp = 0.0;
        locals.var_deltemp_dn4 = 0.0;
        locals.var_deltemp_dn5 = 0.0;
        locals.var_deltemp_dn6 = 0.0;

        let assign10700_e10333: f64 = if ((p.p36 == 1.0) && (p.p14 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard770 = assign10700_e10333;

        let assign10710_e10338: f64 = if ((p.p35 != 0.0) && (!true)) { 1.0 } else { 0.0 };
        locals.var_guard771 = assign10710_e10338;

        let assign10720_e10340: f64 = 1.0;
        locals.var_guard772 = assign10720_e10340;

        let (assign10730_e10348, assign10730_e10348_d_n4, assign10730_e10348_d_n5, assign10730_e10348_d_n6,) = {
    if (((locals.var_guard770 != 0.0) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 != 0.0)) {
        ((nv5 - 0.0), 0.0, 1.0, 0.0,)
    } else {
        (locals.var_deltemp, locals.var_deltemp_dn4, locals.var_deltemp_dn5, locals.var_deltemp_dn6,)
    }
};
        locals.var_deltemp = assign10730_e10348;
        locals.var_deltemp_dn4 = assign10730_e10348_d_n4;
        locals.var_deltemp_dn5 = assign10730_e10348_d_n5;
        locals.var_deltemp_dn6 = assign10730_e10348_d_n6;

        let assign10740_e10350: f64 = 1.0;
        locals.var_guard773 = assign10740_e10350;

        let (assign10750_e10361, assign10750_e10361_d_n4, assign10750_e10361_d_n5, assign10750_e10361_d_n6,) = {
    if ((((locals.var_guard770 != 0.0) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard773 != 0.0)) {
        ((nv4 - 0.0), 1.0, 0.0, 0.0,)
    } else {
        (locals.var_deltemp, locals.var_deltemp_dn4, locals.var_deltemp_dn5, locals.var_deltemp_dn6,)
    }
};
        locals.var_deltemp = assign10750_e10361;
        locals.var_deltemp_dn4 = assign10750_e10361_d_n4;
        locals.var_deltemp_dn5 = assign10750_e10361_d_n5;
        locals.var_deltemp_dn6 = assign10750_e10361_d_n6;

        let (assign10760_e10373, assign10760_e10373_d_n4, assign10760_e10373_d_n5, assign10760_e10373_d_n6,) = {
    if ((((locals.var_guard770 != 0.0) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) && (locals.var_guard773 == 0.0)) {
        ((nv6 - 0.0), 0.0, 0.0, 1.0,)
    } else {
        (locals.var_deltemp, locals.var_deltemp_dn4, locals.var_deltemp_dn5, locals.var_deltemp_dn6,)
    }
};
        locals.var_deltemp = assign10760_e10373;
        locals.var_deltemp_dn4 = assign10760_e10373_d_n4;
        locals.var_deltemp_dn5 = assign10760_e10373_d_n5;
        locals.var_deltemp_dn6 = assign10760_e10373_d_n6;

        let (assign10770_e10380, assign10770_e10380_d_n4, assign10770_e10380_d_n5, assign10770_e10380_d_n6,) = {
    if ((locals.var_guard770 != 0.0) && (locals.var_guard771 == 0.0)) {
        ((nv6 - 0.0), 0.0, 0.0, 1.0,)
    } else {
        (locals.var_deltemp, locals.var_deltemp_dn4, locals.var_deltemp_dn5, locals.var_deltemp_dn6,)
    }
};
        locals.var_deltemp = assign10770_e10380;
        locals.var_deltemp_dn4 = assign10770_e10380_d_n4;
        locals.var_deltemp_dn5 = assign10770_e10380_d_n5;
        locals.var_deltemp_dn6 = assign10770_e10380_d_n6;

        let assign10780_e10383: f64 = (locals.var_deltemp + locals.var_devtemp);
        locals.var_devtemp = assign10780_e10383;
        locals.var_devtemp_dn4 = (locals.var_deltemp_dn4 + locals.var_devtemp_dn4);
        locals.var_devtemp_dn5 = (locals.var_deltemp_dn5 + locals.var_devtemp_dn5);
        locals.var_devtemp_dn6 = (locals.var_deltemp_dn6 + locals.var_devtemp_dn6);

        let assign10790_e10386: f64 = (locals.var_devtemp / locals.var_tnom);
        locals.var_tempratio = assign10790_e10386;
        locals.var_tempratio_dn4 = (locals.var_devtemp_dn4 / locals.var_tnom);
        locals.var_tempratio_dn5 = (locals.var_devtemp_dn5 / locals.var_tnom);
        locals.var_tempratio_dn6 = (locals.var_devtemp_dn6 / locals.var_tnom);

        let assign10800_e10389: f64 = (locals.var_tempratio - 1.0);
        locals.var_trm1 = assign10800_e10389;
        locals.var_trm1_dn4 = locals.var_tempratio_dn4;
        locals.var_trm1_dn5 = locals.var_tempratio_dn5;
        locals.var_trm1_dn6 = locals.var_tempratio_dn6;

        locals.var_coxeff2 = 0.0;
        locals.var_coxeff2_dn3 = 0.0;
        locals.var_coxeff2_dn4 = 0.0;
        locals.var_coxeff2_dn5 = 0.0;
        locals.var_coxeff2_dn6 = 0.0;
        locals.var_coxeff2_dn7 = 0.0;
        locals.var_coxeff2_dn8 = 0.0;
        locals.var_coxeff2_dn9 = 0.0;
        locals.var_coxeff2_dn10 = 0.0;
        locals.var_coxeff2_dn11 = 0.0;
        locals.var_coxeff2_dn12 = 0.0;

        locals.var_coxwlcen2 = 0.0;
        locals.var_coxwlcen2_dn3 = 0.0;
        locals.var_coxwlcen2_dn4 = 0.0;
        locals.var_coxwlcen2_dn5 = 0.0;
        locals.var_coxwlcen2_dn6 = 0.0;
        locals.var_coxwlcen2_dn7 = 0.0;
        locals.var_coxwlcen2_dn8 = 0.0;
        locals.var_coxwlcen2_dn9 = 0.0;
        locals.var_coxwlcen2_dn10 = 0.0;
        locals.var_coxwlcen2_dn11 = 0.0;
        locals.var_coxwlcen2_dn12 = 0.0;

        locals.var_coxwlcenb2 = 0.0;
        locals.var_coxwlcenb2_dn3 = 0.0;
        locals.var_coxwlcenb2_dn4 = 0.0;
        locals.var_coxwlcenb2_dn5 = 0.0;
        locals.var_coxwlcenb2_dn6 = 0.0;
        locals.var_coxwlcenb2_dn7 = 0.0;
        locals.var_coxwlcenb2_dn8 = 0.0;
        locals.var_coxwlcenb2_dn9 = 0.0;
        locals.var_coxwlcenb2_dn10 = 0.0;
        locals.var_coxwlcenb2_dn11 = 0.0;
        locals.var_coxwlcenb2_dn12 = 0.0;

        locals.var_deltaphi2 = 0.0;
        locals.var_deltaphi2_dn3 = 0.0;
        locals.var_deltaphi2_dn4 = 0.0;
        locals.var_deltaphi2_dn5 = 0.0;
        locals.var_deltaphi2_dn6 = 0.0;
        locals.var_deltaphi2_dn7 = 0.0;
        locals.var_deltaphi2_dn8 = 0.0;
        locals.var_deltaphi2_dn9 = 0.0;
        locals.var_deltaphi2_dn10 = 0.0;
        locals.var_deltaphi2_dn11 = 0.0;
        locals.var_deltaphi2_dn12 = 0.0;

        locals.var_tcen2 = 0.0;
        locals.var_tcen2_dn3 = 0.0;
        locals.var_tcen2_dn4 = 0.0;
        locals.var_tcen2_dn5 = 0.0;
        locals.var_tcen2_dn6 = 0.0;
        locals.var_tcen2_dn7 = 0.0;
        locals.var_tcen2_dn8 = 0.0;
        locals.var_tcen2_dn9 = 0.0;
        locals.var_tcen2_dn10 = 0.0;
        locals.var_tcen2_dn11 = 0.0;
        locals.var_tcen2_dn12 = 0.0;

        locals.var_t02 = 0.0;
        locals.var_t02_dn3 = 0.0;
        locals.var_t02_dn4 = 0.0;
        locals.var_t02_dn5 = 0.0;
        locals.var_t02_dn6 = 0.0;
        locals.var_t02_dn7 = 0.0;
        locals.var_t02_dn8 = 0.0;
        locals.var_t02_dn9 = 0.0;
        locals.var_t02_dn10 = 0.0;
        locals.var_t02_dn11 = 0.0;
        locals.var_t02_dn12 = 0.0;

        locals.var_t12 = 0.0;
        locals.var_t12_dn3 = 0.0;
        locals.var_t12_dn4 = 0.0;
        locals.var_t12_dn5 = 0.0;
        locals.var_t12_dn6 = 0.0;
        locals.var_t12_dn7 = 0.0;
        locals.var_t12_dn8 = 0.0;
        locals.var_t12_dn9 = 0.0;
        locals.var_t12_dn10 = 0.0;
        locals.var_t12_dn11 = 0.0;
        locals.var_t12_dn12 = 0.0;

        locals.var_t22 = 0.0;
        locals.var_t22_dn3 = 0.0;
        locals.var_t22_dn4 = 0.0;
        locals.var_t22_dn5 = 0.0;
        locals.var_t22_dn6 = 0.0;
        locals.var_t22_dn7 = 0.0;
        locals.var_t22_dn8 = 0.0;
        locals.var_t22_dn9 = 0.0;
        locals.var_t22_dn10 = 0.0;
        locals.var_t22_dn11 = 0.0;
        locals.var_t22_dn12 = 0.0;

        locals.var_vdseffcv2 = 0.0;
        locals.var_vdseffcv2_dn3 = 0.0;
        locals.var_vdseffcv2_dn4 = 0.0;
        locals.var_vdseffcv2_dn5 = 0.0;
        locals.var_vdseffcv2_dn6 = 0.0;
        locals.var_vdseffcv2_dn7 = 0.0;
        locals.var_vdseffcv2_dn8 = 0.0;
        locals.var_vdseffcv2_dn9 = 0.0;
        locals.var_vdseffcv2_dn10 = 0.0;
        locals.var_vdseffcv2_dn11 = 0.0;
        locals.var_vdseffcv2_dn12 = 0.0;

        locals.var_vfb2 = 0.0;
        locals.var_vfb2_dn3 = 0.0;
        locals.var_vfb2_dn4 = 0.0;
        locals.var_vfb2_dn5 = 0.0;
        locals.var_vfb2_dn6 = 0.0;
        locals.var_vfb2_dn7 = 0.0;
        locals.var_vfb2_dn8 = 0.0;
        locals.var_vfb2_dn9 = 0.0;
        locals.var_vfb2_dn10 = 0.0;
        locals.var_vfb2_dn11 = 0.0;
        locals.var_vfb2_dn12 = 0.0;

        locals.var_vfbeff2 = 0.0;
        locals.var_vfbeff2_dn3 = 0.0;
        locals.var_vfbeff2_dn4 = 0.0;
        locals.var_vfbeff2_dn5 = 0.0;
        locals.var_vfbeff2_dn6 = 0.0;
        locals.var_vfbeff2_dn7 = 0.0;
        locals.var_vfbeff2_dn8 = 0.0;
        locals.var_vfbeff2_dn9 = 0.0;
        locals.var_vfbeff2_dn10 = 0.0;
        locals.var_vfbeff2_dn11 = 0.0;
        locals.var_vfbeff2_dn12 = 0.0;

        locals.var_vfbzb2 = 0.0;
        locals.var_vfbzb2_dn3 = 0.0;
        locals.var_vfbzb2_dn4 = 0.0;
        locals.var_vfbzb2_dn5 = 0.0;
        locals.var_vfbzb2_dn6 = 0.0;
        locals.var_vfbzb2_dn7 = 0.0;
        locals.var_vfbzb2_dn8 = 0.0;
        locals.var_vfbzb2_dn9 = 0.0;
        locals.var_vfbzb2_dn10 = 0.0;
        locals.var_vfbzb2_dn11 = 0.0;
        locals.var_vfbzb2_dn12 = 0.0;

        locals.var_vgsteff2 = 0.0;
        locals.var_vgsteff2_dn3 = 0.0;
        locals.var_vgsteff2_dn4 = 0.0;
        locals.var_vgsteff2_dn5 = 0.0;
        locals.var_vgsteff2_dn6 = 0.0;
        locals.var_vgsteff2_dn7 = 0.0;
        locals.var_vgsteff2_dn8 = 0.0;
        locals.var_vgsteff2_dn9 = 0.0;
        locals.var_vgsteff2_dn10 = 0.0;
        locals.var_vgsteff2_dn11 = 0.0;
        locals.var_vgsteff2_dn12 = 0.0;

        locals.var_rds0 = locals.var_pparam_b4soirds0;
        locals.var_rds0_dn3 = locals.var_pparam_b4soirds0_dn3;
        locals.var_rds0_dn4 = locals.var_pparam_b4soirds0_dn4;
        locals.var_rds0_dn5 = locals.var_pparam_b4soirds0_dn5;
        locals.var_rds0_dn6 = locals.var_pparam_b4soirds0_dn6;
        locals.var_rds0_dn7 = locals.var_pparam_b4soirds0_dn7;
        locals.var_rds0_dn8 = locals.var_pparam_b4soirds0_dn8;
        locals.var_rds0_dn9 = locals.var_pparam_b4soirds0_dn9;
        locals.var_rds0_dn10 = locals.var_pparam_b4soirds0_dn10;
        locals.var_rds0_dn11 = locals.var_pparam_b4soirds0_dn11;
        locals.var_rds0_dn12 = locals.var_pparam_b4soirds0_dn12;

        locals.var_rd0 = locals.var_pparam_b4soird0;
        locals.var_rd0_dn3 = locals.var_pparam_b4soird0_dn3;
        locals.var_rd0_dn4 = locals.var_pparam_b4soird0_dn4;
        locals.var_rd0_dn5 = locals.var_pparam_b4soird0_dn5;
        locals.var_rd0_dn6 = locals.var_pparam_b4soird0_dn6;
        locals.var_rd0_dn7 = locals.var_pparam_b4soird0_dn7;
        locals.var_rd0_dn8 = locals.var_pparam_b4soird0_dn8;
        locals.var_rd0_dn9 = locals.var_pparam_b4soird0_dn9;
        locals.var_rd0_dn10 = locals.var_pparam_b4soird0_dn10;
        locals.var_rd0_dn11 = locals.var_pparam_b4soird0_dn11;
        locals.var_rd0_dn12 = locals.var_pparam_b4soird0_dn12;

        locals.var_rs0 = locals.var_pparam_b4soirs0;
        locals.var_rs0_dn3 = locals.var_pparam_b4soirs0_dn3;
        locals.var_rs0_dn4 = locals.var_pparam_b4soirs0_dn4;
        locals.var_rs0_dn5 = locals.var_pparam_b4soirs0_dn5;
        locals.var_rs0_dn6 = locals.var_pparam_b4soirs0_dn6;
        locals.var_rs0_dn7 = locals.var_pparam_b4soirs0_dn7;
        locals.var_rs0_dn8 = locals.var_pparam_b4soirs0_dn8;
        locals.var_rs0_dn9 = locals.var_pparam_b4soirs0_dn9;
        locals.var_rs0_dn10 = locals.var_pparam_b4soirs0_dn10;
        locals.var_rs0_dn11 = locals.var_pparam_b4soirs0_dn11;
        locals.var_rs0_dn12 = locals.var_pparam_b4soirs0_dn12;

        locals.var_rdwmin = locals.var_pparam_b4soirdwmin;
        locals.var_rdwmin_dn3 = locals.var_pparam_b4soirdwmin_dn3;
        locals.var_rdwmin_dn4 = locals.var_pparam_b4soirdwmin_dn4;
        locals.var_rdwmin_dn5 = locals.var_pparam_b4soirdwmin_dn5;
        locals.var_rdwmin_dn6 = locals.var_pparam_b4soirdwmin_dn6;
        locals.var_rdwmin_dn7 = locals.var_pparam_b4soirdwmin_dn7;
        locals.var_rdwmin_dn8 = locals.var_pparam_b4soirdwmin_dn8;
        locals.var_rdwmin_dn9 = locals.var_pparam_b4soirdwmin_dn9;
        locals.var_rdwmin_dn10 = locals.var_pparam_b4soirdwmin_dn10;
        locals.var_rdwmin_dn11 = locals.var_pparam_b4soirdwmin_dn11;
        locals.var_rdwmin_dn12 = locals.var_pparam_b4soirdwmin_dn12;

        locals.var_rswmin = locals.var_pparam_b4soirswmin;
        locals.var_rswmin_dn3 = locals.var_pparam_b4soirswmin_dn3;
        locals.var_rswmin_dn4 = locals.var_pparam_b4soirswmin_dn4;
        locals.var_rswmin_dn5 = locals.var_pparam_b4soirswmin_dn5;
        locals.var_rswmin_dn6 = locals.var_pparam_b4soirswmin_dn6;
        locals.var_rswmin_dn7 = locals.var_pparam_b4soirswmin_dn7;
        locals.var_rswmin_dn8 = locals.var_pparam_b4soirswmin_dn8;
        locals.var_rswmin_dn9 = locals.var_pparam_b4soirswmin_dn9;
        locals.var_rswmin_dn10 = locals.var_pparam_b4soirswmin_dn10;
        locals.var_rswmin_dn11 = locals.var_pparam_b4soirswmin_dn11;
        locals.var_rswmin_dn12 = locals.var_pparam_b4soirswmin_dn12;

        let assign11030_e10420: f64 = if ((p.p36 == 1.0) && (p.p14 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1124 = assign11030_e10420;

        let assign11040_e10423: f64 = if p.p41 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1125 = assign11040_e10423;

        let (assign11050_e10431, assign11050_e10431_d_n4, assign11050_e10431_d_n5, assign11050_e10431_d_n6,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 != 0.0)) {
        let assign11050_e10429: f64 = (8.617087e-5 * locals.var_devtemp);
        (assign11050_e10429, (8.617087e-5 * locals.var_devtemp_dn4), (8.617087e-5 * locals.var_devtemp_dn5), (8.617087e-5 * locals.var_devtemp_dn6),)
    } else {
        (locals.var_vtm, locals.var_vtm_dn4, locals.var_vtm_dn5, locals.var_vtm_dn6,)
    }
};
        locals.var_vtm = assign11050_e10431;
        locals.var_vtm_dn4 = assign11050_e10431_d_n4;
        locals.var_vtm_dn5 = assign11050_e10431_d_n5;
        locals.var_vtm_dn6 = assign11050_e10431_d_n6;

        let (assign11060_e10439, assign11060_e10439_d_n3, assign11060_e10439_d_n4, assign11060_e10439_d_n5, assign11060_e10439_d_n6, assign11060_e10439_d_n7, assign11060_e10439_d_n8, assign11060_e10439_d_n9, assign11060_e10439_d_n10, assign11060_e10439_d_n11, assign11060_e10439_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 != 0.0)) {
        let assign11060_e10437: f64 = (1108.0 + locals.var_devtemp);
        (assign11060_e10437, 0.0, locals.var_devtemp_dn4, locals.var_devtemp_dn5, locals.var_devtemp_dn6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign11060_e10439;
        locals.var_t0__blk808_dn3 = assign11060_e10439_d_n3;
        locals.var_t0__blk808_dn4 = assign11060_e10439_d_n4;
        locals.var_t0__blk808_dn5 = assign11060_e10439_d_n5;
        locals.var_t0__blk808_dn6 = assign11060_e10439_d_n6;
        locals.var_t0__blk808_dn7 = assign11060_e10439_d_n7;
        locals.var_t0__blk808_dn8 = assign11060_e10439_d_n8;
        locals.var_t0__blk808_dn9 = assign11060_e10439_d_n9;
        locals.var_t0__blk808_dn10 = assign11060_e10439_d_n10;
        locals.var_t0__blk808_dn11 = assign11060_e10439_d_n11;
        locals.var_t0__blk808_dn12 = assign11060_e10439_d_n12;

        let (assign11070_e10447, assign11070_e10447_d_n3, assign11070_e10447_d_n4, assign11070_e10447_d_n5, assign11070_e10447_d_n6, assign11070_e10447_d_n7, assign11070_e10447_d_n8, assign11070_e10447_d_n9, assign11070_e10447_d_n10, assign11070_e10447_d_n11, assign11070_e10447_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 != 0.0)) {
        let assign11070_e10445: f64 = (locals.var_devtemp * locals.var_devtemp);
        (assign11070_e10445, 0.0, ((locals.var_devtemp_dn4 * locals.var_devtemp) + (locals.var_devtemp * locals.var_devtemp_dn4)), ((locals.var_devtemp_dn5 * locals.var_devtemp) + (locals.var_devtemp * locals.var_devtemp_dn5)), ((locals.var_devtemp_dn6 * locals.var_devtemp) + (locals.var_devtemp * locals.var_devtemp_dn6)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign11070_e10447;
        locals.var_t5__blk813_dn3 = assign11070_e10447_d_n3;
        locals.var_t5__blk813_dn4 = assign11070_e10447_d_n4;
        locals.var_t5__blk813_dn5 = assign11070_e10447_d_n5;
        locals.var_t5__blk813_dn6 = assign11070_e10447_d_n6;
        locals.var_t5__blk813_dn7 = assign11070_e10447_d_n7;
        locals.var_t5__blk813_dn8 = assign11070_e10447_d_n8;
        locals.var_t5__blk813_dn9 = assign11070_e10447_d_n9;
        locals.var_t5__blk813_dn10 = assign11070_e10447_d_n10;
        locals.var_t5__blk813_dn11 = assign11070_e10447_d_n11;
        locals.var_t5__blk813_dn12 = assign11070_e10447_d_n12;

        let (assign11080_e10459, assign11080_e10459_d_n3, assign11080_e10459_d_n4, assign11080_e10459_d_n5, assign11080_e10459_d_n6, assign11080_e10459_d_n7, assign11080_e10459_d_n8, assign11080_e10459_d_n9, assign11080_e10459_d_n10, assign11080_e10459_d_n11, assign11080_e10459_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 != 0.0)) {
        let assign11080_e10454: f64 = (0.000702 * locals.var_t5__blk813);
        let assign11080_e10456: f64 = (assign11080_e10454 / locals.var_t0__blk808);
        let assign11080_e10457: f64 = (1.16 - assign11080_e10456);
        (assign11080_e10457, (-((((0.000702 * locals.var_t5__blk813_dn3) * locals.var_t0__blk808) - (assign11080_e10454 * locals.var_t0__blk808_dn3)) / (locals.var_t0__blk808 * locals.var_t0__blk808))), (-((((0.000702 * locals.var_t5__blk813_dn4) * locals.var_t0__blk808) - (assign11080_e10454 * locals.var_t0__blk808_dn4)) / (locals.var_t0__blk808 * locals.var_t0__blk808))), (-((((0.000702 * locals.var_t5__blk813_dn5) * locals.var_t0__blk808) - (assign11080_e10454 * locals.var_t0__blk808_dn5)) / (locals.var_t0__blk808 * locals.var_t0__blk808))), (-((((0.000702 * locals.var_t5__blk813_dn6) * locals.var_t0__blk808) - (assign11080_e10454 * locals.var_t0__blk808_dn6)) / (locals.var_t0__blk808 * locals.var_t0__blk808))), (-((((0.000702 * locals.var_t5__blk813_dn7) * locals.var_t0__blk808) - (assign11080_e10454 * locals.var_t0__blk808_dn7)) / (locals.var_t0__blk808 * locals.var_t0__blk808))), (-((((0.000702 * locals.var_t5__blk813_dn8) * locals.var_t0__blk808) - (assign11080_e10454 * locals.var_t0__blk808_dn8)) / (locals.var_t0__blk808 * locals.var_t0__blk808))), (-((((0.000702 * locals.var_t5__blk813_dn9) * locals.var_t0__blk808) - (assign11080_e10454 * locals.var_t0__blk808_dn9)) / (locals.var_t0__blk808 * locals.var_t0__blk808))), (-((((0.000702 * locals.var_t5__blk813_dn10) * locals.var_t0__blk808) - (assign11080_e10454 * locals.var_t0__blk808_dn10)) / (locals.var_t0__blk808 * locals.var_t0__blk808))), (-((((0.000702 * locals.var_t5__blk813_dn11) * locals.var_t0__blk808) - (assign11080_e10454 * locals.var_t0__blk808_dn11)) / (locals.var_t0__blk808 * locals.var_t0__blk808))), (-((((0.000702 * locals.var_t5__blk813_dn12) * locals.var_t0__blk808) - (assign11080_e10454 * locals.var_t0__blk808_dn12)) / (locals.var_t0__blk808 * locals.var_t0__blk808))),)
    } else {
        (locals.var_eg__blk877, locals.var_eg__blk877_dn3, locals.var_eg__blk877_dn4, locals.var_eg__blk877_dn5, locals.var_eg__blk877_dn6, locals.var_eg__blk877_dn7, locals.var_eg__blk877_dn8, locals.var_eg__blk877_dn9, locals.var_eg__blk877_dn10, locals.var_eg__blk877_dn11, locals.var_eg__blk877_dn12,)
    }
};
        locals.var_eg__blk877 = assign11080_e10459;
        locals.var_eg__blk877_dn3 = assign11080_e10459_d_n3;
        locals.var_eg__blk877_dn4 = assign11080_e10459_d_n4;
        locals.var_eg__blk877_dn5 = assign11080_e10459_d_n5;
        locals.var_eg__blk877_dn6 = assign11080_e10459_d_n6;
        locals.var_eg__blk877_dn7 = assign11080_e10459_d_n7;
        locals.var_eg__blk877_dn8 = assign11080_e10459_d_n8;
        locals.var_eg__blk877_dn9 = assign11080_e10459_d_n9;
        locals.var_eg__blk877_dn10 = assign11080_e10459_d_n10;
        locals.var_eg__blk877_dn11 = assign11080_e10459_d_n11;
        locals.var_eg__blk877_dn12 = assign11080_e10459_d_n12;

        let (assign11090_e10465, assign11090_e10465_d_n3, assign11090_e10465_d_n4, assign11090_e10465_d_n5, assign11090_e10465_d_n6, assign11090_e10465_d_n7, assign11090_e10465_d_n8, assign11090_e10465_d_n9, assign11090_e10465_d_n10, assign11090_e10465_d_n11, assign11090_e10465_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 != 0.0)) {
        (0.00019230584, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign11090_e10465;
        locals.var_t2__blk810_dn3 = assign11090_e10465_d_n3;
        locals.var_t2__blk810_dn4 = assign11090_e10465_d_n4;
        locals.var_t2__blk810_dn5 = assign11090_e10465_d_n5;
        locals.var_t2__blk810_dn6 = assign11090_e10465_d_n6;
        locals.var_t2__blk810_dn7 = assign11090_e10465_d_n7;
        locals.var_t2__blk810_dn8 = assign11090_e10465_d_n8;
        locals.var_t2__blk810_dn9 = assign11090_e10465_d_n9;
        locals.var_t2__blk810_dn10 = assign11090_e10465_d_n10;
        locals.var_t2__blk810_dn11 = assign11090_e10465_d_n11;
        locals.var_t2__blk810_dn12 = assign11090_e10465_d_n12;

        let (assign11100_e10472, assign11100_e10472_d_n3, assign11100_e10472_d_n4, assign11100_e10472_d_n5, assign11100_e10472_d_n6, assign11100_e10472_d_n7, assign11100_e10472_d_n8, assign11100_e10472_d_n9, assign11100_e10472_d_n10, assign11100_e10472_d_n11, assign11100_e10472_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 != 0.0)) {
        let assign11100_e10470: f64 = (locals.var_devtemp).sqrt();
        (assign11100_e10470, 0.0, (locals.var_devtemp_dn4 / (2.0 * assign11100_e10470)), (locals.var_devtemp_dn5 / (2.0 * assign11100_e10470)), (locals.var_devtemp_dn6 / (2.0 * assign11100_e10470)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign11100_e10472;
        locals.var_t5__blk813_dn3 = assign11100_e10472_d_n3;
        locals.var_t5__blk813_dn4 = assign11100_e10472_d_n4;
        locals.var_t5__blk813_dn5 = assign11100_e10472_d_n5;
        locals.var_t5__blk813_dn6 = assign11100_e10472_d_n6;
        locals.var_t5__blk813_dn7 = assign11100_e10472_d_n7;
        locals.var_t5__blk813_dn8 = assign11100_e10472_d_n8;
        locals.var_t5__blk813_dn9 = assign11100_e10472_d_n9;
        locals.var_t5__blk813_dn10 = assign11100_e10472_d_n10;
        locals.var_t5__blk813_dn11 = assign11100_e10472_d_n11;
        locals.var_t5__blk813_dn12 = assign11100_e10472_d_n12;

        let (assign11110_e10484, assign11110_e10484_d_n3, assign11110_e10484_d_n4, assign11110_e10484_d_n5, assign11110_e10484_d_n6, assign11110_e10484_d_n7, assign11110_e10484_d_n8, assign11110_e10484_d_n9, assign11110_e10484_d_n10, assign11110_e10484_d_n11, assign11110_e10484_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 != 0.0)) {
        let assign11110_e10478: f64 = (14500000000.0 * locals.var_devtemp);
        let assign11110_e10480: f64 = (assign11110_e10478 * locals.var_t5__blk813);
        let assign11110_e10482: f64 = (assign11110_e10480 * locals.var_t2__blk810);
        (assign11110_e10482, (((assign11110_e10478 * locals.var_t5__blk813_dn3) * locals.var_t2__blk810) + (assign11110_e10480 * locals.var_t2__blk810_dn3)), (((((14500000000.0 * locals.var_devtemp_dn4) * locals.var_t5__blk813) + (assign11110_e10478 * locals.var_t5__blk813_dn4)) * locals.var_t2__blk810) + (assign11110_e10480 * locals.var_t2__blk810_dn4)), (((((14500000000.0 * locals.var_devtemp_dn5) * locals.var_t5__blk813) + (assign11110_e10478 * locals.var_t5__blk813_dn5)) * locals.var_t2__blk810) + (assign11110_e10480 * locals.var_t2__blk810_dn5)), (((((14500000000.0 * locals.var_devtemp_dn6) * locals.var_t5__blk813) + (assign11110_e10478 * locals.var_t5__blk813_dn6)) * locals.var_t2__blk810) + (assign11110_e10480 * locals.var_t2__blk810_dn6)), (((assign11110_e10478 * locals.var_t5__blk813_dn7) * locals.var_t2__blk810) + (assign11110_e10480 * locals.var_t2__blk810_dn7)), (((assign11110_e10478 * locals.var_t5__blk813_dn8) * locals.var_t2__blk810) + (assign11110_e10480 * locals.var_t2__blk810_dn8)), (((assign11110_e10478 * locals.var_t5__blk813_dn9) * locals.var_t2__blk810) + (assign11110_e10480 * locals.var_t2__blk810_dn9)), (((assign11110_e10478 * locals.var_t5__blk813_dn10) * locals.var_t2__blk810) + (assign11110_e10480 * locals.var_t2__blk810_dn10)), (((assign11110_e10478 * locals.var_t5__blk813_dn11) * locals.var_t2__blk810) + (assign11110_e10480 * locals.var_t2__blk810_dn11)), (((assign11110_e10478 * locals.var_t5__blk813_dn12) * locals.var_t2__blk810) + (assign11110_e10480 * locals.var_t2__blk810_dn12)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign11110_e10484;
        locals.var_t3__blk811_dn3 = assign11110_e10484_d_n3;
        locals.var_t3__blk811_dn4 = assign11110_e10484_d_n4;
        locals.var_t3__blk811_dn5 = assign11110_e10484_d_n5;
        locals.var_t3__blk811_dn6 = assign11110_e10484_d_n6;
        locals.var_t3__blk811_dn7 = assign11110_e10484_d_n7;
        locals.var_t3__blk811_dn8 = assign11110_e10484_d_n8;
        locals.var_t3__blk811_dn9 = assign11110_e10484_d_n9;
        locals.var_t3__blk811_dn10 = assign11110_e10484_d_n10;
        locals.var_t3__blk811_dn11 = assign11110_e10484_d_n11;
        locals.var_t3__blk811_dn12 = assign11110_e10484_d_n12;

        let (assign11120_e10496, assign11120_e10496_d_n3, assign11120_e10496_d_n4, assign11120_e10496_d_n5, assign11120_e10496_d_n6, assign11120_e10496_d_n7, assign11120_e10496_d_n8, assign11120_e10496_d_n9, assign11120_e10496_d_n10, assign11120_e10496_d_n11, assign11120_e10496_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 != 0.0)) {
        let assign11120_e10492: f64 = (2.0 * locals.var_vtm);
        let assign11120_e10493: f64 = (locals.var_eg__blk877 / assign11120_e10492);
        let assign11120_e10494: f64 = (21.5565981 - assign11120_e10493);
        (assign11120_e10494, (-(locals.var_eg__blk877_dn3 / assign11120_e10492)), (-(((locals.var_eg__blk877_dn4 * assign11120_e10492) - (locals.var_eg__blk877 * (2.0 * locals.var_vtm_dn4))) / (assign11120_e10492 * assign11120_e10492))), (-(((locals.var_eg__blk877_dn5 * assign11120_e10492) - (locals.var_eg__blk877 * (2.0 * locals.var_vtm_dn5))) / (assign11120_e10492 * assign11120_e10492))), (-(((locals.var_eg__blk877_dn6 * assign11120_e10492) - (locals.var_eg__blk877 * (2.0 * locals.var_vtm_dn6))) / (assign11120_e10492 * assign11120_e10492))), (-(locals.var_eg__blk877_dn7 / assign11120_e10492)), (-(locals.var_eg__blk877_dn8 / assign11120_e10492)), (-(locals.var_eg__blk877_dn9 / assign11120_e10492)), (-(locals.var_eg__blk877_dn10 / assign11120_e10492)), (-(locals.var_eg__blk877_dn11 / assign11120_e10492)), (-(locals.var_eg__blk877_dn12 / assign11120_e10492)),)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign11120_e10496;
        locals.var_t6__blk814_dn3 = assign11120_e10496_d_n3;
        locals.var_t6__blk814_dn4 = assign11120_e10496_d_n4;
        locals.var_t6__blk814_dn5 = assign11120_e10496_d_n5;
        locals.var_t6__blk814_dn6 = assign11120_e10496_d_n6;
        locals.var_t6__blk814_dn7 = assign11120_e10496_d_n7;
        locals.var_t6__blk814_dn8 = assign11120_e10496_d_n8;
        locals.var_t6__blk814_dn9 = assign11120_e10496_d_n9;
        locals.var_t6__blk814_dn10 = assign11120_e10496_d_n10;
        locals.var_t6__blk814_dn11 = assign11120_e10496_d_n11;
        locals.var_t6__blk814_dn12 = assign11120_e10496_d_n12;

        let assign11130_e10499: f64 = (-100.0);
        let assign11130_e10500: f64 = if locals.var_t6__blk814 > assign11130_e10499 { 1.0 } else { 0.0 };
        locals.var_guard1126 = assign11130_e10500;

        let (assign11140_e10509, assign11140_e10509_d_n3, assign11140_e10509_d_n4, assign11140_e10509_d_n5, assign11140_e10509_d_n6, assign11140_e10509_d_n7, assign11140_e10509_d_n8, assign11140_e10509_d_n9, assign11140_e10509_d_n10, assign11140_e10509_d_n11, assign11140_e10509_d_n12,) = {
    if (((locals.var_guard1124 != 0.0) && (locals.var_guard1125 != 0.0)) && (locals.var_guard1126 != 0.0)) {
        let assign11140_e10507: f64 = (locals.var_t6__blk814).exp();
        (assign11140_e10507, (assign11140_e10507 * locals.var_t6__blk814_dn3), (assign11140_e10507 * locals.var_t6__blk814_dn4), (assign11140_e10507 * locals.var_t6__blk814_dn5), (assign11140_e10507 * locals.var_t6__blk814_dn6), (assign11140_e10507 * locals.var_t6__blk814_dn7), (assign11140_e10507 * locals.var_t6__blk814_dn8), (assign11140_e10507 * locals.var_t6__blk814_dn9), (assign11140_e10507 * locals.var_t6__blk814_dn10), (assign11140_e10507 * locals.var_t6__blk814_dn11), (assign11140_e10507 * locals.var_t6__blk814_dn12),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign11140_e10509;
        locals.var_t4__blk812_dn3 = assign11140_e10509_d_n3;
        locals.var_t4__blk812_dn4 = assign11140_e10509_d_n4;
        locals.var_t4__blk812_dn5 = assign11140_e10509_d_n5;
        locals.var_t4__blk812_dn6 = assign11140_e10509_d_n6;
        locals.var_t4__blk812_dn7 = assign11140_e10509_d_n7;
        locals.var_t4__blk812_dn8 = assign11140_e10509_d_n8;
        locals.var_t4__blk812_dn9 = assign11140_e10509_d_n9;
        locals.var_t4__blk812_dn10 = assign11140_e10509_d_n10;
        locals.var_t4__blk812_dn11 = assign11140_e10509_d_n11;
        locals.var_t4__blk812_dn12 = assign11140_e10509_d_n12;

    }

    pub(super) fn stamp_transient_block_27(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11150_e10520, assign11150_e10520_d_n3, assign11150_e10520_d_n4, assign11150_e10520_d_n5, assign11150_e10520_d_n6, assign11150_e10520_d_n7, assign11150_e10520_d_n8, assign11150_e10520_d_n9, assign11150_e10520_d_n10, assign11150_e10520_d_n11, assign11150_e10520_d_n12,) = {
    if (((locals.var_guard1124 != 0.0) && (locals.var_guard1125 != 0.0)) && (locals.var_guard1126 == 0.0)) {
        let assign11150_e10517: f64 = (-100.0);
        let assign11150_e10518: f64 = (assign11150_e10517).exp();
        (assign11150_e10518, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign11150_e10520;
        locals.var_t4__blk812_dn3 = assign11150_e10520_d_n3;
        locals.var_t4__blk812_dn4 = assign11150_e10520_d_n4;
        locals.var_t4__blk812_dn5 = assign11150_e10520_d_n5;
        locals.var_t4__blk812_dn6 = assign11150_e10520_d_n6;
        locals.var_t4__blk812_dn7 = assign11150_e10520_d_n7;
        locals.var_t4__blk812_dn8 = assign11150_e10520_d_n8;
        locals.var_t4__blk812_dn9 = assign11150_e10520_d_n9;
        locals.var_t4__blk812_dn10 = assign11150_e10520_d_n10;
        locals.var_t4__blk812_dn11 = assign11150_e10520_d_n11;
        locals.var_t4__blk812_dn12 = assign11150_e10520_d_n12;

        let (assign11160_e10528, assign11160_e10528_d_n3, assign11160_e10528_d_n4, assign11160_e10528_d_n5, assign11160_e10528_d_n6, assign11160_e10528_d_n7, assign11160_e10528_d_n8, assign11160_e10528_d_n9, assign11160_e10528_d_n10, assign11160_e10528_d_n11, assign11160_e10528_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 != 0.0)) {
        let assign11160_e10526: f64 = (locals.var_t3__blk811 * locals.var_t4__blk812);
        (assign11160_e10526, ((locals.var_t3__blk811_dn3 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn3)), ((locals.var_t3__blk811_dn4 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn4)), ((locals.var_t3__blk811_dn5 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn5)), ((locals.var_t3__blk811_dn6 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn6)), ((locals.var_t3__blk811_dn7 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn7)), ((locals.var_t3__blk811_dn8 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn8)), ((locals.var_t3__blk811_dn9 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn9)), ((locals.var_t3__blk811_dn10 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn10)), ((locals.var_t3__blk811_dn11 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn11)), ((locals.var_t3__blk811_dn12 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn12)),)
    } else {
        (locals.var_ni, locals.var_ni_dn3, locals.var_ni_dn4, locals.var_ni_dn5, locals.var_ni_dn6, locals.var_ni_dn7, locals.var_ni_dn8, locals.var_ni_dn9, locals.var_ni_dn10, locals.var_ni_dn11, locals.var_ni_dn12,)
    }
};
        locals.var_ni = assign11160_e10528;
        locals.var_ni_dn3 = assign11160_e10528_d_n3;
        locals.var_ni_dn4 = assign11160_e10528_d_n4;
        locals.var_ni_dn5 = assign11160_e10528_d_n5;
        locals.var_ni_dn6 = assign11160_e10528_d_n6;
        locals.var_ni_dn7 = assign11160_e10528_d_n7;
        locals.var_ni_dn8 = assign11160_e10528_d_n8;
        locals.var_ni_dn9 = assign11160_e10528_d_n9;
        locals.var_ni_dn10 = assign11160_e10528_d_n10;
        locals.var_ni_dn11 = assign11160_e10528_d_n11;
        locals.var_ni_dn12 = assign11160_e10528_d_n12;

        let (assign11170_e10553, assign11170_e10553_d_n3, assign11170_e10553_d_n4, assign11170_e10553_d_n5, assign11170_e10553_d_n6, assign11170_e10553_d_n7, assign11170_e10553_d_n8, assign11170_e10553_d_n9, assign11170_e10553_d_n10, assign11170_e10553_d_n11, assign11170_e10553_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 != 0.0)) {
        let assign11170_e10534: f64 = (1e20 * locals.var_pparam_b4soinpeak);
        let assign11170_e10537: f64 = (locals.var_ni * locals.var_ni);
        let assign11170_e10538: f64 = (assign11170_e10534 / assign11170_e10537);
        let (assign11170_e10551, assign11170_e10551_d_n3, assign11170_e10551_d_n4, assign11170_e10551_d_n5, assign11170_e10551_d_n6, assign11170_e10551_d_n7, assign11170_e10551_d_n8, assign11170_e10551_d_n9, assign11170_e10551_d_n10, assign11170_e10551_d_n11, assign11170_e10551_d_n12,) = {
            if (assign11170_e10538 > 1e-38) {
                let assign11170_e10543: f64 = (1e20 * locals.var_pparam_b4soinpeak);
                let assign11170_e10546: f64 = (locals.var_ni * locals.var_ni);
                let assign11170_e10547: f64 = (assign11170_e10543 / assign11170_e10546);
                let assign11170_e10548: f64 = (assign11170_e10547).ln();
                (assign11170_e10548, (((((1e20 * locals.var_pparam_b4soinpeak_dn3) * assign11170_e10546) - (assign11170_e10543 * ((locals.var_ni_dn3 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn3)))) / (assign11170_e10546 * assign11170_e10546)) / assign11170_e10547), (((((1e20 * locals.var_pparam_b4soinpeak_dn4) * assign11170_e10546) - (assign11170_e10543 * ((locals.var_ni_dn4 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn4)))) / (assign11170_e10546 * assign11170_e10546)) / assign11170_e10547), (((((1e20 * locals.var_pparam_b4soinpeak_dn5) * assign11170_e10546) - (assign11170_e10543 * ((locals.var_ni_dn5 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn5)))) / (assign11170_e10546 * assign11170_e10546)) / assign11170_e10547), (((((1e20 * locals.var_pparam_b4soinpeak_dn6) * assign11170_e10546) - (assign11170_e10543 * ((locals.var_ni_dn6 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn6)))) / (assign11170_e10546 * assign11170_e10546)) / assign11170_e10547), (((((1e20 * locals.var_pparam_b4soinpeak_dn7) * assign11170_e10546) - (assign11170_e10543 * ((locals.var_ni_dn7 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn7)))) / (assign11170_e10546 * assign11170_e10546)) / assign11170_e10547), (((((1e20 * locals.var_pparam_b4soinpeak_dn8) * assign11170_e10546) - (assign11170_e10543 * ((locals.var_ni_dn8 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn8)))) / (assign11170_e10546 * assign11170_e10546)) / assign11170_e10547), (((((1e20 * locals.var_pparam_b4soinpeak_dn9) * assign11170_e10546) - (assign11170_e10543 * ((locals.var_ni_dn9 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn9)))) / (assign11170_e10546 * assign11170_e10546)) / assign11170_e10547), (((((1e20 * locals.var_pparam_b4soinpeak_dn10) * assign11170_e10546) - (assign11170_e10543 * ((locals.var_ni_dn10 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn10)))) / (assign11170_e10546 * assign11170_e10546)) / assign11170_e10547), (((((1e20 * locals.var_pparam_b4soinpeak_dn11) * assign11170_e10546) - (assign11170_e10543 * ((locals.var_ni_dn11 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn11)))) / (assign11170_e10546 * assign11170_e10546)) / assign11170_e10547), (((((1e20 * locals.var_pparam_b4soinpeak_dn12) * assign11170_e10546) - (assign11170_e10543 * ((locals.var_ni_dn12 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn12)))) / (assign11170_e10546 * assign11170_e10546)) / assign11170_e10547),)
            } else {
                let assign11170_e10550: f64 = (-87.49823353377374);
                (assign11170_e10550, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign11170_e10551, assign11170_e10551_d_n3, assign11170_e10551_d_n4, assign11170_e10551_d_n5, assign11170_e10551_d_n6, assign11170_e10551_d_n7, assign11170_e10551_d_n8, assign11170_e10551_d_n9, assign11170_e10551_d_n10, assign11170_e10551_d_n11, assign11170_e10551_d_n12,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign11170_e10553;
        locals.var_t0__blk808_dn3 = assign11170_e10553_d_n3;
        locals.var_t0__blk808_dn4 = assign11170_e10553_d_n4;
        locals.var_t0__blk808_dn5 = assign11170_e10553_d_n5;
        locals.var_t0__blk808_dn6 = assign11170_e10553_d_n6;
        locals.var_t0__blk808_dn7 = assign11170_e10553_d_n7;
        locals.var_t0__blk808_dn8 = assign11170_e10553_d_n8;
        locals.var_t0__blk808_dn9 = assign11170_e10553_d_n9;
        locals.var_t0__blk808_dn10 = assign11170_e10553_d_n10;
        locals.var_t0__blk808_dn11 = assign11170_e10553_d_n11;
        locals.var_t0__blk808_dn12 = assign11170_e10553_d_n12;

        let (assign11180_e10561, assign11180_e10561_d_n3, assign11180_e10561_d_n4, assign11180_e10561_d_n5, assign11180_e10561_d_n6, assign11180_e10561_d_n7, assign11180_e10561_d_n8, assign11180_e10561_d_n9, assign11180_e10561_d_n10, assign11180_e10561_d_n11, assign11180_e10561_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 != 0.0)) {
        let assign11180_e10559: f64 = (locals.var_vtm * locals.var_t0__blk808);
        (assign11180_e10559, (locals.var_vtm * locals.var_t0__blk808_dn3), ((locals.var_vtm_dn4 * locals.var_t0__blk808) + (locals.var_vtm * locals.var_t0__blk808_dn4)), ((locals.var_vtm_dn5 * locals.var_t0__blk808) + (locals.var_vtm * locals.var_t0__blk808_dn5)), ((locals.var_vtm_dn6 * locals.var_t0__blk808) + (locals.var_vtm * locals.var_t0__blk808_dn6)), (locals.var_vtm * locals.var_t0__blk808_dn7), (locals.var_vtm * locals.var_t0__blk808_dn8), (locals.var_vtm * locals.var_t0__blk808_dn9), (locals.var_vtm * locals.var_t0__blk808_dn10), (locals.var_vtm * locals.var_t0__blk808_dn11), (locals.var_vtm * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_vbi, locals.var_vbi_dn3, locals.var_vbi_dn4, locals.var_vbi_dn5, locals.var_vbi_dn6, locals.var_vbi_dn7, locals.var_vbi_dn8, locals.var_vbi_dn9, locals.var_vbi_dn10, locals.var_vbi_dn11, locals.var_vbi_dn12,)
    }
};
        locals.var_vbi = assign11180_e10561;
        locals.var_vbi_dn3 = assign11180_e10561_d_n3;
        locals.var_vbi_dn4 = assign11180_e10561_d_n4;
        locals.var_vbi_dn5 = assign11180_e10561_d_n5;
        locals.var_vbi_dn6 = assign11180_e10561_d_n6;
        locals.var_vbi_dn7 = assign11180_e10561_d_n7;
        locals.var_vbi_dn8 = assign11180_e10561_d_n8;
        locals.var_vbi_dn9 = assign11180_e10561_d_n9;
        locals.var_vbi_dn10 = assign11180_e10561_d_n10;
        locals.var_vbi_dn11 = assign11180_e10561_d_n11;
        locals.var_vbi_dn12 = assign11180_e10561_d_n12;

        let (assign11190_e10570,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 == 0.0)) {
        let assign11190_e10568: f64 = (p.p126 + 273.15);
        (assign11190_e10568,)
    } else {
        (locals.var_tnom,)
    }
};
        locals.var_tnom = assign11190_e10570;

        let (assign11200_e10579, assign11200_e10579_d_n4, assign11200_e10579_d_n5, assign11200_e10579_d_n6,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 == 0.0)) {
        let assign11200_e10577: f64 = (8.617087e-5 * locals.var_devtemp);
        (assign11200_e10577, (8.617087e-5 * locals.var_devtemp_dn4), (8.617087e-5 * locals.var_devtemp_dn5), (8.617087e-5 * locals.var_devtemp_dn6),)
    } else {
        (locals.var_vtm, locals.var_vtm_dn4, locals.var_vtm_dn5, locals.var_vtm_dn6,)
    }
};
        locals.var_vtm = assign11200_e10579;
        locals.var_vtm_dn4 = assign11200_e10579_d_n4;
        locals.var_vtm_dn5 = assign11200_e10579_d_n5;
        locals.var_vtm_dn6 = assign11200_e10579_d_n6;

        let (assign11210_e10588,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 == 0.0)) {
        let assign11210_e10586: f64 = (8.617087e-5 * locals.var_tnom);
        (assign11210_e10586,)
    } else {
        (locals.var_vtm0__blk1069,)
    }
};
        locals.var_vtm0__blk1069 = assign11210_e10588;

        let (assign11220_e10595,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 == 0.0)) {
        (locals.var_b4soieg0,)
    } else {
        (locals.var_eg0__blk1068,)
    }
};
        locals.var_eg0__blk1068 = assign11220_e10595;

        let (assign11230_e10612, assign11230_e10612_d_n3, assign11230_e10612_d_n4, assign11230_e10612_d_n5, assign11230_e10612_d_n6, assign11230_e10612_d_n7, assign11230_e10612_d_n8, assign11230_e10612_d_n9, assign11230_e10612_d_n10, assign11230_e10612_d_n11, assign11230_e10612_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 == 0.0)) {
        let assign11230_e10603: f64 = (p.p50 * locals.var_devtemp);
        let assign11230_e10605: f64 = (assign11230_e10603 * locals.var_devtemp);
        let assign11230_e10608: f64 = (locals.var_devtemp + p.p51);
        let assign11230_e10609: f64 = (assign11230_e10605 / assign11230_e10608);
        let assign11230_e10610: f64 = (p.p49 - assign11230_e10609);
        (assign11230_e10610, 0.0, (-((((((p.p50 * locals.var_devtemp_dn4) * locals.var_devtemp) + (assign11230_e10603 * locals.var_devtemp_dn4)) * assign11230_e10608) - (assign11230_e10605 * locals.var_devtemp_dn4)) / (assign11230_e10608 * assign11230_e10608))), (-((((((p.p50 * locals.var_devtemp_dn5) * locals.var_devtemp) + (assign11230_e10603 * locals.var_devtemp_dn5)) * assign11230_e10608) - (assign11230_e10605 * locals.var_devtemp_dn5)) / (assign11230_e10608 * assign11230_e10608))), (-((((((p.p50 * locals.var_devtemp_dn6) * locals.var_devtemp) + (assign11230_e10603 * locals.var_devtemp_dn6)) * assign11230_e10608) - (assign11230_e10605 * locals.var_devtemp_dn6)) / (assign11230_e10608 * assign11230_e10608))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eg__blk877, locals.var_eg__blk877_dn3, locals.var_eg__blk877_dn4, locals.var_eg__blk877_dn5, locals.var_eg__blk877_dn6, locals.var_eg__blk877_dn7, locals.var_eg__blk877_dn8, locals.var_eg__blk877_dn9, locals.var_eg__blk877_dn10, locals.var_eg__blk877_dn11, locals.var_eg__blk877_dn12,)
    }
};
        locals.var_eg__blk877 = assign11230_e10612;
        locals.var_eg__blk877_dn3 = assign11230_e10612_d_n3;
        locals.var_eg__blk877_dn4 = assign11230_e10612_d_n4;
        locals.var_eg__blk877_dn5 = assign11230_e10612_d_n5;
        locals.var_eg__blk877_dn6 = assign11230_e10612_d_n6;
        locals.var_eg__blk877_dn7 = assign11230_e10612_d_n7;
        locals.var_eg__blk877_dn8 = assign11230_e10612_d_n8;
        locals.var_eg__blk877_dn9 = assign11230_e10612_d_n9;
        locals.var_eg__blk877_dn10 = assign11230_e10612_d_n10;
        locals.var_eg__blk877_dn11 = assign11230_e10612_d_n11;
        locals.var_eg__blk877_dn12 = assign11230_e10612_d_n12;

        let (assign11240_e10626, assign11240_e10626_d_n3, assign11240_e10626_d_n4, assign11240_e10626_d_n5, assign11240_e10626_d_n6, assign11240_e10626_d_n7, assign11240_e10626_d_n8, assign11240_e10626_d_n9, assign11240_e10626_d_n10, assign11240_e10626_d_n11, assign11240_e10626_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 == 0.0)) {
        let assign11240_e10620: f64 = (locals.var_tnom * locals.var_tnom);
        let assign11240_e10622: f64 = (assign11240_e10620 * locals.var_tnom);
        let assign11240_e10623: f64 = (assign11240_e10622).sqrt();
        let assign11240_e10624: f64 = (1.0 / assign11240_e10623);
        (assign11240_e10624, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign11240_e10626;
        locals.var_t2__blk810_dn3 = assign11240_e10626_d_n3;
        locals.var_t2__blk810_dn4 = assign11240_e10626_d_n4;
        locals.var_t2__blk810_dn5 = assign11240_e10626_d_n5;
        locals.var_t2__blk810_dn6 = assign11240_e10626_d_n6;
        locals.var_t2__blk810_dn7 = assign11240_e10626_d_n7;
        locals.var_t2__blk810_dn8 = assign11240_e10626_d_n8;
        locals.var_t2__blk810_dn9 = assign11240_e10626_d_n9;
        locals.var_t2__blk810_dn10 = assign11240_e10626_d_n10;
        locals.var_t2__blk810_dn11 = assign11240_e10626_d_n11;
        locals.var_t2__blk810_dn12 = assign11240_e10626_d_n12;

        let (assign11250_e10634, assign11250_e10634_d_n3, assign11250_e10634_d_n4, assign11250_e10634_d_n5, assign11250_e10634_d_n6, assign11250_e10634_d_n7, assign11250_e10634_d_n8, assign11250_e10634_d_n9, assign11250_e10634_d_n10, assign11250_e10634_d_n11, assign11250_e10634_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 == 0.0)) {
        let assign11250_e10632: f64 = (locals.var_devtemp).sqrt();
        (assign11250_e10632, 0.0, (locals.var_devtemp_dn4 / (2.0 * assign11250_e10632)), (locals.var_devtemp_dn5 / (2.0 * assign11250_e10632)), (locals.var_devtemp_dn6 / (2.0 * assign11250_e10632)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign11250_e10634;
        locals.var_t5__blk813_dn3 = assign11250_e10634_d_n3;
        locals.var_t5__blk813_dn4 = assign11250_e10634_d_n4;
        locals.var_t5__blk813_dn5 = assign11250_e10634_d_n5;
        locals.var_t5__blk813_dn6 = assign11250_e10634_d_n6;
        locals.var_t5__blk813_dn7 = assign11250_e10634_d_n7;
        locals.var_t5__blk813_dn8 = assign11250_e10634_d_n8;
        locals.var_t5__blk813_dn9 = assign11250_e10634_d_n9;
        locals.var_t5__blk813_dn10 = assign11250_e10634_d_n10;
        locals.var_t5__blk813_dn11 = assign11250_e10634_d_n11;
        locals.var_t5__blk813_dn12 = assign11250_e10634_d_n12;

        let (assign11260_e10647, assign11260_e10647_d_n3, assign11260_e10647_d_n4, assign11260_e10647_d_n5, assign11260_e10647_d_n6, assign11260_e10647_d_n7, assign11260_e10647_d_n8, assign11260_e10647_d_n9, assign11260_e10647_d_n10, assign11260_e10647_d_n11, assign11260_e10647_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 == 0.0)) {
        let assign11260_e10641: f64 = (p.p48 * locals.var_devtemp);
        let assign11260_e10643: f64 = (assign11260_e10641 * locals.var_t5__blk813);
        let assign11260_e10645: f64 = (assign11260_e10643 * locals.var_t2__blk810);
        (assign11260_e10645, (((assign11260_e10641 * locals.var_t5__blk813_dn3) * locals.var_t2__blk810) + (assign11260_e10643 * locals.var_t2__blk810_dn3)), (((((p.p48 * locals.var_devtemp_dn4) * locals.var_t5__blk813) + (assign11260_e10641 * locals.var_t5__blk813_dn4)) * locals.var_t2__blk810) + (assign11260_e10643 * locals.var_t2__blk810_dn4)), (((((p.p48 * locals.var_devtemp_dn5) * locals.var_t5__blk813) + (assign11260_e10641 * locals.var_t5__blk813_dn5)) * locals.var_t2__blk810) + (assign11260_e10643 * locals.var_t2__blk810_dn5)), (((((p.p48 * locals.var_devtemp_dn6) * locals.var_t5__blk813) + (assign11260_e10641 * locals.var_t5__blk813_dn6)) * locals.var_t2__blk810) + (assign11260_e10643 * locals.var_t2__blk810_dn6)), (((assign11260_e10641 * locals.var_t5__blk813_dn7) * locals.var_t2__blk810) + (assign11260_e10643 * locals.var_t2__blk810_dn7)), (((assign11260_e10641 * locals.var_t5__blk813_dn8) * locals.var_t2__blk810) + (assign11260_e10643 * locals.var_t2__blk810_dn8)), (((assign11260_e10641 * locals.var_t5__blk813_dn9) * locals.var_t2__blk810) + (assign11260_e10643 * locals.var_t2__blk810_dn9)), (((assign11260_e10641 * locals.var_t5__blk813_dn10) * locals.var_t2__blk810) + (assign11260_e10643 * locals.var_t2__blk810_dn10)), (((assign11260_e10641 * locals.var_t5__blk813_dn11) * locals.var_t2__blk810) + (assign11260_e10643 * locals.var_t2__blk810_dn11)), (((assign11260_e10641 * locals.var_t5__blk813_dn12) * locals.var_t2__blk810) + (assign11260_e10643 * locals.var_t2__blk810_dn12)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign11260_e10647;
        locals.var_t3__blk811_dn3 = assign11260_e10647_d_n3;
        locals.var_t3__blk811_dn4 = assign11260_e10647_d_n4;
        locals.var_t3__blk811_dn5 = assign11260_e10647_d_n5;
        locals.var_t3__blk811_dn6 = assign11260_e10647_d_n6;
        locals.var_t3__blk811_dn7 = assign11260_e10647_d_n7;
        locals.var_t3__blk811_dn8 = assign11260_e10647_d_n8;
        locals.var_t3__blk811_dn9 = assign11260_e10647_d_n9;
        locals.var_t3__blk811_dn10 = assign11260_e10647_d_n10;
        locals.var_t3__blk811_dn11 = assign11260_e10647_d_n11;
        locals.var_t3__blk811_dn12 = assign11260_e10647_d_n12;

        let (assign11270_e10665, assign11270_e10665_d_n3, assign11270_e10665_d_n4, assign11270_e10665_d_n5, assign11270_e10665_d_n6, assign11270_e10665_d_n7, assign11270_e10665_d_n8, assign11270_e10665_d_n9, assign11270_e10665_d_n10, assign11270_e10665_d_n11, assign11270_e10665_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 == 0.0)) {
        let assign11270_e10655: f64 = (2.0 * locals.var_vtm0__blk1069);
        let assign11270_e10656: f64 = (locals.var_eg0__blk1068 / assign11270_e10655);
        let assign11270_e10660: f64 = (2.0 * locals.var_vtm);
        let assign11270_e10661: f64 = (locals.var_eg__blk877 / assign11270_e10660);
        let assign11270_e10662: f64 = (assign11270_e10656 - assign11270_e10661);
        let assign11270_e10663: f64 = (assign11270_e10662).exp();
        (assign11270_e10663, (assign11270_e10663 * (-(locals.var_eg__blk877_dn3 / assign11270_e10660))), (assign11270_e10663 * (-(((locals.var_eg__blk877_dn4 * assign11270_e10660) - (locals.var_eg__blk877 * (2.0 * locals.var_vtm_dn4))) / (assign11270_e10660 * assign11270_e10660)))), (assign11270_e10663 * (-(((locals.var_eg__blk877_dn5 * assign11270_e10660) - (locals.var_eg__blk877 * (2.0 * locals.var_vtm_dn5))) / (assign11270_e10660 * assign11270_e10660)))), (assign11270_e10663 * (-(((locals.var_eg__blk877_dn6 * assign11270_e10660) - (locals.var_eg__blk877 * (2.0 * locals.var_vtm_dn6))) / (assign11270_e10660 * assign11270_e10660)))), (assign11270_e10663 * (-(locals.var_eg__blk877_dn7 / assign11270_e10660))), (assign11270_e10663 * (-(locals.var_eg__blk877_dn8 / assign11270_e10660))), (assign11270_e10663 * (-(locals.var_eg__blk877_dn9 / assign11270_e10660))), (assign11270_e10663 * (-(locals.var_eg__blk877_dn10 / assign11270_e10660))), (assign11270_e10663 * (-(locals.var_eg__blk877_dn11 / assign11270_e10660))), (assign11270_e10663 * (-(locals.var_eg__blk877_dn12 / assign11270_e10660))),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign11270_e10665;
        locals.var_t4__blk812_dn3 = assign11270_e10665_d_n3;
        locals.var_t4__blk812_dn4 = assign11270_e10665_d_n4;
        locals.var_t4__blk812_dn5 = assign11270_e10665_d_n5;
        locals.var_t4__blk812_dn6 = assign11270_e10665_d_n6;
        locals.var_t4__blk812_dn7 = assign11270_e10665_d_n7;
        locals.var_t4__blk812_dn8 = assign11270_e10665_d_n8;
        locals.var_t4__blk812_dn9 = assign11270_e10665_d_n9;
        locals.var_t4__blk812_dn10 = assign11270_e10665_d_n10;
        locals.var_t4__blk812_dn11 = assign11270_e10665_d_n11;
        locals.var_t4__blk812_dn12 = assign11270_e10665_d_n12;

        let (assign11280_e10674, assign11280_e10674_d_n3, assign11280_e10674_d_n4, assign11280_e10674_d_n5, assign11280_e10674_d_n6, assign11280_e10674_d_n7, assign11280_e10674_d_n8, assign11280_e10674_d_n9, assign11280_e10674_d_n10, assign11280_e10674_d_n11, assign11280_e10674_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 == 0.0)) {
        let assign11280_e10672: f64 = (locals.var_t3__blk811 * locals.var_t4__blk812);
        (assign11280_e10672, ((locals.var_t3__blk811_dn3 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn3)), ((locals.var_t3__blk811_dn4 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn4)), ((locals.var_t3__blk811_dn5 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn5)), ((locals.var_t3__blk811_dn6 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn6)), ((locals.var_t3__blk811_dn7 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn7)), ((locals.var_t3__blk811_dn8 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn8)), ((locals.var_t3__blk811_dn9 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn9)), ((locals.var_t3__blk811_dn10 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn10)), ((locals.var_t3__blk811_dn11 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn11)), ((locals.var_t3__blk811_dn12 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn12)),)
    } else {
        (locals.var_ni, locals.var_ni_dn3, locals.var_ni_dn4, locals.var_ni_dn5, locals.var_ni_dn6, locals.var_ni_dn7, locals.var_ni_dn8, locals.var_ni_dn9, locals.var_ni_dn10, locals.var_ni_dn11, locals.var_ni_dn12,)
    }
};
        locals.var_ni = assign11280_e10674;
        locals.var_ni_dn3 = assign11280_e10674_d_n3;
        locals.var_ni_dn4 = assign11280_e10674_d_n4;
        locals.var_ni_dn5 = assign11280_e10674_d_n5;
        locals.var_ni_dn6 = assign11280_e10674_d_n6;
        locals.var_ni_dn7 = assign11280_e10674_d_n7;
        locals.var_ni_dn8 = assign11280_e10674_d_n8;
        locals.var_ni_dn9 = assign11280_e10674_d_n9;
        locals.var_ni_dn10 = assign11280_e10674_d_n10;
        locals.var_ni_dn11 = assign11280_e10674_d_n11;
        locals.var_ni_dn12 = assign11280_e10674_d_n12;

        let (assign11290_e10700, assign11290_e10700_d_n3, assign11290_e10700_d_n4, assign11290_e10700_d_n5, assign11290_e10700_d_n6, assign11290_e10700_d_n7, assign11290_e10700_d_n8, assign11290_e10700_d_n9, assign11290_e10700_d_n10, assign11290_e10700_d_n11, assign11290_e10700_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 == 0.0)) {
        let assign11290_e10681: f64 = (1e20 * locals.var_pparam_b4soinpeak);
        let assign11290_e10684: f64 = (locals.var_ni * locals.var_ni);
        let assign11290_e10685: f64 = (assign11290_e10681 / assign11290_e10684);
        let (assign11290_e10698, assign11290_e10698_d_n3, assign11290_e10698_d_n4, assign11290_e10698_d_n5, assign11290_e10698_d_n6, assign11290_e10698_d_n7, assign11290_e10698_d_n8, assign11290_e10698_d_n9, assign11290_e10698_d_n10, assign11290_e10698_d_n11, assign11290_e10698_d_n12,) = {
            if (assign11290_e10685 > 1e-38) {
                let assign11290_e10690: f64 = (1e20 * locals.var_pparam_b4soinpeak);
                let assign11290_e10693: f64 = (locals.var_ni * locals.var_ni);
                let assign11290_e10694: f64 = (assign11290_e10690 / assign11290_e10693);
                let assign11290_e10695: f64 = (assign11290_e10694).ln();
                (assign11290_e10695, (((((1e20 * locals.var_pparam_b4soinpeak_dn3) * assign11290_e10693) - (assign11290_e10690 * ((locals.var_ni_dn3 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn3)))) / (assign11290_e10693 * assign11290_e10693)) / assign11290_e10694), (((((1e20 * locals.var_pparam_b4soinpeak_dn4) * assign11290_e10693) - (assign11290_e10690 * ((locals.var_ni_dn4 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn4)))) / (assign11290_e10693 * assign11290_e10693)) / assign11290_e10694), (((((1e20 * locals.var_pparam_b4soinpeak_dn5) * assign11290_e10693) - (assign11290_e10690 * ((locals.var_ni_dn5 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn5)))) / (assign11290_e10693 * assign11290_e10693)) / assign11290_e10694), (((((1e20 * locals.var_pparam_b4soinpeak_dn6) * assign11290_e10693) - (assign11290_e10690 * ((locals.var_ni_dn6 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn6)))) / (assign11290_e10693 * assign11290_e10693)) / assign11290_e10694), (((((1e20 * locals.var_pparam_b4soinpeak_dn7) * assign11290_e10693) - (assign11290_e10690 * ((locals.var_ni_dn7 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn7)))) / (assign11290_e10693 * assign11290_e10693)) / assign11290_e10694), (((((1e20 * locals.var_pparam_b4soinpeak_dn8) * assign11290_e10693) - (assign11290_e10690 * ((locals.var_ni_dn8 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn8)))) / (assign11290_e10693 * assign11290_e10693)) / assign11290_e10694), (((((1e20 * locals.var_pparam_b4soinpeak_dn9) * assign11290_e10693) - (assign11290_e10690 * ((locals.var_ni_dn9 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn9)))) / (assign11290_e10693 * assign11290_e10693)) / assign11290_e10694), (((((1e20 * locals.var_pparam_b4soinpeak_dn10) * assign11290_e10693) - (assign11290_e10690 * ((locals.var_ni_dn10 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn10)))) / (assign11290_e10693 * assign11290_e10693)) / assign11290_e10694), (((((1e20 * locals.var_pparam_b4soinpeak_dn11) * assign11290_e10693) - (assign11290_e10690 * ((locals.var_ni_dn11 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn11)))) / (assign11290_e10693 * assign11290_e10693)) / assign11290_e10694), (((((1e20 * locals.var_pparam_b4soinpeak_dn12) * assign11290_e10693) - (assign11290_e10690 * ((locals.var_ni_dn12 * locals.var_ni) + (locals.var_ni * locals.var_ni_dn12)))) / (assign11290_e10693 * assign11290_e10693)) / assign11290_e10694),)
            } else {
                let assign11290_e10697: f64 = (-87.49823353377374);
                (assign11290_e10697, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign11290_e10698, assign11290_e10698_d_n3, assign11290_e10698_d_n4, assign11290_e10698_d_n5, assign11290_e10698_d_n6, assign11290_e10698_d_n7, assign11290_e10698_d_n8, assign11290_e10698_d_n9, assign11290_e10698_d_n10, assign11290_e10698_d_n11, assign11290_e10698_d_n12,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign11290_e10700;
        locals.var_t0__blk808_dn3 = assign11290_e10700_d_n3;
        locals.var_t0__blk808_dn4 = assign11290_e10700_d_n4;
        locals.var_t0__blk808_dn5 = assign11290_e10700_d_n5;
        locals.var_t0__blk808_dn6 = assign11290_e10700_d_n6;
        locals.var_t0__blk808_dn7 = assign11290_e10700_d_n7;
        locals.var_t0__blk808_dn8 = assign11290_e10700_d_n8;
        locals.var_t0__blk808_dn9 = assign11290_e10700_d_n9;
        locals.var_t0__blk808_dn10 = assign11290_e10700_d_n10;
        locals.var_t0__blk808_dn11 = assign11290_e10700_d_n11;
        locals.var_t0__blk808_dn12 = assign11290_e10700_d_n12;

        let (assign11300_e10709, assign11300_e10709_d_n3, assign11300_e10709_d_n4, assign11300_e10709_d_n5, assign11300_e10709_d_n6, assign11300_e10709_d_n7, assign11300_e10709_d_n8, assign11300_e10709_d_n9, assign11300_e10709_d_n10, assign11300_e10709_d_n11, assign11300_e10709_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1125 == 0.0)) {
        let assign11300_e10707: f64 = (locals.var_vtm * locals.var_t0__blk808);
        (assign11300_e10707, (locals.var_vtm * locals.var_t0__blk808_dn3), ((locals.var_vtm_dn4 * locals.var_t0__blk808) + (locals.var_vtm * locals.var_t0__blk808_dn4)), ((locals.var_vtm_dn5 * locals.var_t0__blk808) + (locals.var_vtm * locals.var_t0__blk808_dn5)), ((locals.var_vtm_dn6 * locals.var_t0__blk808) + (locals.var_vtm * locals.var_t0__blk808_dn6)), (locals.var_vtm * locals.var_t0__blk808_dn7), (locals.var_vtm * locals.var_t0__blk808_dn8), (locals.var_vtm * locals.var_t0__blk808_dn9), (locals.var_vtm * locals.var_t0__blk808_dn10), (locals.var_vtm * locals.var_t0__blk808_dn11), (locals.var_vtm * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_vbi, locals.var_vbi_dn3, locals.var_vbi_dn4, locals.var_vbi_dn5, locals.var_vbi_dn6, locals.var_vbi_dn7, locals.var_vbi_dn8, locals.var_vbi_dn9, locals.var_vbi_dn10, locals.var_vbi_dn11, locals.var_vbi_dn12,)
    }
};
        locals.var_vbi = assign11300_e10709;
        locals.var_vbi_dn3 = assign11300_e10709_d_n3;
        locals.var_vbi_dn4 = assign11300_e10709_d_n4;
        locals.var_vbi_dn5 = assign11300_e10709_d_n5;
        locals.var_vbi_dn6 = assign11300_e10709_d_n6;
        locals.var_vbi_dn7 = assign11300_e10709_d_n7;
        locals.var_vbi_dn8 = assign11300_e10709_d_n8;
        locals.var_vbi_dn9 = assign11300_e10709_d_n9;
        locals.var_vbi_dn10 = assign11300_e10709_d_n10;
        locals.var_vbi_dn11 = assign11300_e10709_d_n11;
        locals.var_vbi_dn12 = assign11300_e10709_d_n12;

        let assign11310_e10712: f64 = if locals.var_pparam_b4soinsub > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1127 = assign11310_e10712;

        let (assign11320_e10729, assign11320_e10729_d_n3, assign11320_e10729_d_n4, assign11320_e10729_d_n5, assign11320_e10729_d_n6, assign11320_e10729_d_n7, assign11320_e10729_d_n8, assign11320_e10729_d_n9, assign11320_e10729_d_n10, assign11320_e10729_d_n11, assign11320_e10729_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1127 != 0.0)) {
        let assign11320_e10718: f64 = (locals.var_pparam_b4soinpeak / locals.var_pparam_b4soinsub);
        let (assign11320_e10727, assign11320_e10727_d_n3, assign11320_e10727_d_n4, assign11320_e10727_d_n5, assign11320_e10727_d_n6, assign11320_e10727_d_n7, assign11320_e10727_d_n8, assign11320_e10727_d_n9, assign11320_e10727_d_n10, assign11320_e10727_d_n11, assign11320_e10727_d_n12,) = {
            if (assign11320_e10718 > 1e-38) {
                let assign11320_e10723: f64 = (locals.var_pparam_b4soinpeak / locals.var_pparam_b4soinsub);
                let assign11320_e10724: f64 = (assign11320_e10723).ln();
                (assign11320_e10724, ((((locals.var_pparam_b4soinpeak_dn3 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn3)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign11320_e10723), ((((locals.var_pparam_b4soinpeak_dn4 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn4)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign11320_e10723), ((((locals.var_pparam_b4soinpeak_dn5 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn5)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign11320_e10723), ((((locals.var_pparam_b4soinpeak_dn6 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn6)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign11320_e10723), ((((locals.var_pparam_b4soinpeak_dn7 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn7)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign11320_e10723), ((((locals.var_pparam_b4soinpeak_dn8 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn8)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign11320_e10723), ((((locals.var_pparam_b4soinpeak_dn9 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn9)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign11320_e10723), ((((locals.var_pparam_b4soinpeak_dn10 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn10)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign11320_e10723), ((((locals.var_pparam_b4soinpeak_dn11 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn11)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign11320_e10723), ((((locals.var_pparam_b4soinpeak_dn12 * locals.var_pparam_b4soinsub) - (locals.var_pparam_b4soinpeak * locals.var_pparam_b4soinsub_dn12)) / (locals.var_pparam_b4soinsub * locals.var_pparam_b4soinsub)) / assign11320_e10723),)
            } else {
                let assign11320_e10726: f64 = (-87.49823353377374);
                (assign11320_e10726, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign11320_e10727, assign11320_e10727_d_n3, assign11320_e10727_d_n4, assign11320_e10727_d_n5, assign11320_e10727_d_n6, assign11320_e10727_d_n7, assign11320_e10727_d_n8, assign11320_e10727_d_n9, assign11320_e10727_d_n10, assign11320_e10727_d_n11, assign11320_e10727_d_n12,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign11320_e10729;
        locals.var_t0__blk808_dn3 = assign11320_e10729_d_n3;
        locals.var_t0__blk808_dn4 = assign11320_e10729_d_n4;
        locals.var_t0__blk808_dn5 = assign11320_e10729_d_n5;
        locals.var_t0__blk808_dn6 = assign11320_e10729_d_n6;
        locals.var_t0__blk808_dn7 = assign11320_e10729_d_n7;
        locals.var_t0__blk808_dn8 = assign11320_e10729_d_n8;
        locals.var_t0__blk808_dn9 = assign11320_e10729_d_n9;
        locals.var_t0__blk808_dn10 = assign11320_e10729_d_n10;
        locals.var_t0__blk808_dn11 = assign11320_e10729_d_n11;
        locals.var_t0__blk808_dn12 = assign11320_e10729_d_n12;

        let (assign11330_e10740, assign11330_e10740_d_n3, assign11330_e10740_d_n4, assign11330_e10740_d_n5, assign11330_e10740_d_n6, assign11330_e10740_d_n7, assign11330_e10740_d_n8, assign11330_e10740_d_n9, assign11330_e10740_d_n10, assign11330_e10740_d_n11, assign11330_e10740_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1127 != 0.0)) {
        let assign11330_e10734: f64 = (-p.p37);
        let assign11330_e10736: f64 = (assign11330_e10734 * locals.var_vtm);
        let assign11330_e10738: f64 = (assign11330_e10736 * locals.var_t0__blk808);
        (assign11330_e10738, (assign11330_e10736 * locals.var_t0__blk808_dn3), (((assign11330_e10734 * locals.var_vtm_dn4) * locals.var_t0__blk808) + (assign11330_e10736 * locals.var_t0__blk808_dn4)), (((assign11330_e10734 * locals.var_vtm_dn5) * locals.var_t0__blk808) + (assign11330_e10736 * locals.var_t0__blk808_dn5)), (((assign11330_e10734 * locals.var_vtm_dn6) * locals.var_t0__blk808) + (assign11330_e10736 * locals.var_t0__blk808_dn6)), (assign11330_e10736 * locals.var_t0__blk808_dn7), (assign11330_e10736 * locals.var_t0__blk808_dn8), (assign11330_e10736 * locals.var_t0__blk808_dn9), (assign11330_e10736 * locals.var_t0__blk808_dn10), (assign11330_e10736 * locals.var_t0__blk808_dn11), (assign11330_e10736 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_vfbb, locals.var_vfbb_dn3, locals.var_vfbb_dn4, locals.var_vfbb_dn5, locals.var_vfbb_dn6, locals.var_vfbb_dn7, locals.var_vfbb_dn8, locals.var_vfbb_dn9, locals.var_vfbb_dn10, locals.var_vfbb_dn11, locals.var_vfbb_dn12,)
    }
};
        locals.var_vfbb = assign11330_e10740;
        locals.var_vfbb_dn3 = assign11330_e10740_d_n3;
        locals.var_vfbb_dn4 = assign11330_e10740_d_n4;
        locals.var_vfbb_dn5 = assign11330_e10740_d_n5;
        locals.var_vfbb_dn6 = assign11330_e10740_d_n6;
        locals.var_vfbb_dn7 = assign11330_e10740_d_n7;
        locals.var_vfbb_dn8 = assign11330_e10740_d_n8;
        locals.var_vfbb_dn9 = assign11330_e10740_d_n9;
        locals.var_vfbb_dn10 = assign11330_e10740_d_n10;
        locals.var_vfbb_dn11 = assign11330_e10740_d_n11;
        locals.var_vfbb_dn12 = assign11330_e10740_d_n12;

        let (assign11340_e10768, assign11340_e10768_d_n3, assign11340_e10768_d_n4, assign11340_e10768_d_n5, assign11340_e10768_d_n6, assign11340_e10768_d_n7, assign11340_e10768_d_n8, assign11340_e10768_d_n9, assign11340_e10768_d_n10, assign11340_e10768_d_n11, assign11340_e10768_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1127 == 0.0)) {
        let assign11340_e10746: f64 = (-locals.var_pparam_b4soinpeak);
        let assign11340_e10748: f64 = (assign11340_e10746 * locals.var_pparam_b4soinsub);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_ni;
        let assign11340_e10750: f64 = (assign11340_e10748 * __rspice_inv_cse_0);
        let assign11340_e10752: f64 = (assign11340_e10750 * __rspice_inv_cse_0);
        let (assign11340_e10766, assign11340_e10766_d_n3, assign11340_e10766_d_n4, assign11340_e10766_d_n5, assign11340_e10766_d_n6, assign11340_e10766_d_n7, assign11340_e10766_d_n8, assign11340_e10766_d_n9, assign11340_e10766_d_n10, assign11340_e10766_d_n11, assign11340_e10766_d_n12,) = {
            if (assign11340_e10752 > 1e-38) {
                let assign11340_e10756: f64 = (-locals.var_pparam_b4soinpeak);
                let assign11340_e10758: f64 = (assign11340_e10756 * locals.var_pparam_b4soinsub);
                let __rspice_inv_cse_1: f64 = 1.0 / locals.var_ni;
                let assign11340_e10760: f64 = (assign11340_e10758 * __rspice_inv_cse_1);
                let assign11340_e10762: f64 = (assign11340_e10760 * __rspice_inv_cse_1);
                let assign11340_e10763: f64 = (assign11340_e10762).ln();
                (assign11340_e10763, ((((((((((-locals.var_pparam_b4soinpeak_dn3) * locals.var_pparam_b4soinsub) + (assign11340_e10756 * locals.var_pparam_b4soinsub_dn3)) * locals.var_ni) - (assign11340_e10758 * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign11340_e10760 * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) / assign11340_e10762), ((((((((((-locals.var_pparam_b4soinpeak_dn4) * locals.var_pparam_b4soinsub) + (assign11340_e10756 * locals.var_pparam_b4soinsub_dn4)) * locals.var_ni) - (assign11340_e10758 * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign11340_e10760 * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) / assign11340_e10762), ((((((((((-locals.var_pparam_b4soinpeak_dn5) * locals.var_pparam_b4soinsub) + (assign11340_e10756 * locals.var_pparam_b4soinsub_dn5)) * locals.var_ni) - (assign11340_e10758 * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign11340_e10760 * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) / assign11340_e10762), ((((((((((-locals.var_pparam_b4soinpeak_dn6) * locals.var_pparam_b4soinsub) + (assign11340_e10756 * locals.var_pparam_b4soinsub_dn6)) * locals.var_ni) - (assign11340_e10758 * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign11340_e10760 * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) / assign11340_e10762), ((((((((((-locals.var_pparam_b4soinpeak_dn7) * locals.var_pparam_b4soinsub) + (assign11340_e10756 * locals.var_pparam_b4soinsub_dn7)) * locals.var_ni) - (assign11340_e10758 * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign11340_e10760 * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) / assign11340_e10762), ((((((((((-locals.var_pparam_b4soinpeak_dn8) * locals.var_pparam_b4soinsub) + (assign11340_e10756 * locals.var_pparam_b4soinsub_dn8)) * locals.var_ni) - (assign11340_e10758 * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign11340_e10760 * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) / assign11340_e10762), ((((((((((-locals.var_pparam_b4soinpeak_dn9) * locals.var_pparam_b4soinsub) + (assign11340_e10756 * locals.var_pparam_b4soinsub_dn9)) * locals.var_ni) - (assign11340_e10758 * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign11340_e10760 * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) / assign11340_e10762), ((((((((((-locals.var_pparam_b4soinpeak_dn10) * locals.var_pparam_b4soinsub) + (assign11340_e10756 * locals.var_pparam_b4soinsub_dn10)) * locals.var_ni) - (assign11340_e10758 * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign11340_e10760 * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) / assign11340_e10762), ((((((((((-locals.var_pparam_b4soinpeak_dn11) * locals.var_pparam_b4soinsub) + (assign11340_e10756 * locals.var_pparam_b4soinsub_dn11)) * locals.var_ni) - (assign11340_e10758 * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign11340_e10760 * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) / assign11340_e10762), ((((((((((-locals.var_pparam_b4soinpeak_dn12) * locals.var_pparam_b4soinsub) + (assign11340_e10756 * locals.var_pparam_b4soinsub_dn12)) * locals.var_ni) - (assign11340_e10758 * locals.var_ni_dn12)) / (locals.var_ni * locals.var_ni)) * locals.var_ni) - (assign11340_e10760 * locals.var_ni_dn12)) / (locals.var_ni * locals.var_ni)) / assign11340_e10762),)
            } else {
                let assign11340_e10765: f64 = (-87.49823353377374);
                (assign11340_e10765, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign11340_e10766, assign11340_e10766_d_n3, assign11340_e10766_d_n4, assign11340_e10766_d_n5, assign11340_e10766_d_n6, assign11340_e10766_d_n7, assign11340_e10766_d_n8, assign11340_e10766_d_n9, assign11340_e10766_d_n10, assign11340_e10766_d_n11, assign11340_e10766_d_n12,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign11340_e10768;
        locals.var_t0__blk808_dn3 = assign11340_e10768_d_n3;
        locals.var_t0__blk808_dn4 = assign11340_e10768_d_n4;
        locals.var_t0__blk808_dn5 = assign11340_e10768_d_n5;
        locals.var_t0__blk808_dn6 = assign11340_e10768_d_n6;
        locals.var_t0__blk808_dn7 = assign11340_e10768_d_n7;
        locals.var_t0__blk808_dn8 = assign11340_e10768_d_n8;
        locals.var_t0__blk808_dn9 = assign11340_e10768_d_n9;
        locals.var_t0__blk808_dn10 = assign11340_e10768_d_n10;
        locals.var_t0__blk808_dn11 = assign11340_e10768_d_n11;
        locals.var_t0__blk808_dn12 = assign11340_e10768_d_n12;

        let (assign11350_e10780, assign11350_e10780_d_n3, assign11350_e10780_d_n4, assign11350_e10780_d_n5, assign11350_e10780_d_n6, assign11350_e10780_d_n7, assign11350_e10780_d_n8, assign11350_e10780_d_n9, assign11350_e10780_d_n10, assign11350_e10780_d_n11, assign11350_e10780_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1127 == 0.0)) {
        let assign11350_e10774: f64 = (-p.p37);
        let assign11350_e10776: f64 = (assign11350_e10774 * locals.var_vtm);
        let assign11350_e10778: f64 = (assign11350_e10776 * locals.var_t0__blk808);
        (assign11350_e10778, (assign11350_e10776 * locals.var_t0__blk808_dn3), (((assign11350_e10774 * locals.var_vtm_dn4) * locals.var_t0__blk808) + (assign11350_e10776 * locals.var_t0__blk808_dn4)), (((assign11350_e10774 * locals.var_vtm_dn5) * locals.var_t0__blk808) + (assign11350_e10776 * locals.var_t0__blk808_dn5)), (((assign11350_e10774 * locals.var_vtm_dn6) * locals.var_t0__blk808) + (assign11350_e10776 * locals.var_t0__blk808_dn6)), (assign11350_e10776 * locals.var_t0__blk808_dn7), (assign11350_e10776 * locals.var_t0__blk808_dn8), (assign11350_e10776 * locals.var_t0__blk808_dn9), (assign11350_e10776 * locals.var_t0__blk808_dn10), (assign11350_e10776 * locals.var_t0__blk808_dn11), (assign11350_e10776 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_vfbb, locals.var_vfbb_dn3, locals.var_vfbb_dn4, locals.var_vfbb_dn5, locals.var_vfbb_dn6, locals.var_vfbb_dn7, locals.var_vfbb_dn8, locals.var_vfbb_dn9, locals.var_vfbb_dn10, locals.var_vfbb_dn11, locals.var_vfbb_dn12,)
    }
};
        locals.var_vfbb = assign11350_e10780;
        locals.var_vfbb_dn3 = assign11350_e10780_d_n3;
        locals.var_vfbb_dn4 = assign11350_e10780_d_n4;
        locals.var_vfbb_dn5 = assign11350_e10780_d_n5;
        locals.var_vfbb_dn6 = assign11350_e10780_d_n6;
        locals.var_vfbb_dn7 = assign11350_e10780_d_n7;
        locals.var_vfbb_dn8 = assign11350_e10780_d_n8;
        locals.var_vfbb_dn9 = assign11350_e10780_d_n9;
        locals.var_vfbb_dn10 = assign11350_e10780_d_n10;
        locals.var_vfbb_dn11 = assign11350_e10780_d_n11;
        locals.var_vfbb_dn12 = assign11350_e10780_d_n12;

        let (assign11360_e10799, assign11360_e10799_d_n3, assign11360_e10799_d_n4, assign11360_e10799_d_n5, assign11360_e10799_d_n6, assign11360_e10799_d_n7, assign11360_e10799_d_n8, assign11360_e10799_d_n9, assign11360_e10799_d_n10, assign11360_e10799_d_n11, assign11360_e10799_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11360_e10784: f64 = (2.0 * locals.var_vtm);
        let assign11360_e10787: f64 = (locals.var_pparam_b4soinpeak / locals.var_ni);
        let (assign11360_e10796, assign11360_e10796_d_n3, assign11360_e10796_d_n4, assign11360_e10796_d_n5, assign11360_e10796_d_n6, assign11360_e10796_d_n7, assign11360_e10796_d_n8, assign11360_e10796_d_n9, assign11360_e10796_d_n10, assign11360_e10796_d_n11, assign11360_e10796_d_n12,) = {
            if (assign11360_e10787 > 1e-38) {
                let assign11360_e10792: f64 = (locals.var_pparam_b4soinpeak / locals.var_ni);
                let assign11360_e10793: f64 = (assign11360_e10792).ln();
                (assign11360_e10793, ((((locals.var_pparam_b4soinpeak_dn3 * locals.var_ni) - (locals.var_pparam_b4soinpeak * locals.var_ni_dn3)) / (locals.var_ni * locals.var_ni)) / assign11360_e10792), ((((locals.var_pparam_b4soinpeak_dn4 * locals.var_ni) - (locals.var_pparam_b4soinpeak * locals.var_ni_dn4)) / (locals.var_ni * locals.var_ni)) / assign11360_e10792), ((((locals.var_pparam_b4soinpeak_dn5 * locals.var_ni) - (locals.var_pparam_b4soinpeak * locals.var_ni_dn5)) / (locals.var_ni * locals.var_ni)) / assign11360_e10792), ((((locals.var_pparam_b4soinpeak_dn6 * locals.var_ni) - (locals.var_pparam_b4soinpeak * locals.var_ni_dn6)) / (locals.var_ni * locals.var_ni)) / assign11360_e10792), ((((locals.var_pparam_b4soinpeak_dn7 * locals.var_ni) - (locals.var_pparam_b4soinpeak * locals.var_ni_dn7)) / (locals.var_ni * locals.var_ni)) / assign11360_e10792), ((((locals.var_pparam_b4soinpeak_dn8 * locals.var_ni) - (locals.var_pparam_b4soinpeak * locals.var_ni_dn8)) / (locals.var_ni * locals.var_ni)) / assign11360_e10792), ((((locals.var_pparam_b4soinpeak_dn9 * locals.var_ni) - (locals.var_pparam_b4soinpeak * locals.var_ni_dn9)) / (locals.var_ni * locals.var_ni)) / assign11360_e10792), ((((locals.var_pparam_b4soinpeak_dn10 * locals.var_ni) - (locals.var_pparam_b4soinpeak * locals.var_ni_dn10)) / (locals.var_ni * locals.var_ni)) / assign11360_e10792), ((((locals.var_pparam_b4soinpeak_dn11 * locals.var_ni) - (locals.var_pparam_b4soinpeak * locals.var_ni_dn11)) / (locals.var_ni * locals.var_ni)) / assign11360_e10792), ((((locals.var_pparam_b4soinpeak_dn12 * locals.var_ni) - (locals.var_pparam_b4soinpeak * locals.var_ni_dn12)) / (locals.var_ni * locals.var_ni)) / assign11360_e10792),)
            } else {
                let assign11360_e10795: f64 = (-87.49823353377374);
                (assign11360_e10795, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign11360_e10797: f64 = (assign11360_e10784 * assign11360_e10796);
        (assign11360_e10797, (assign11360_e10784 * assign11360_e10796_d_n3), (((2.0 * locals.var_vtm_dn4) * assign11360_e10796) + (assign11360_e10784 * assign11360_e10796_d_n4)), (((2.0 * locals.var_vtm_dn5) * assign11360_e10796) + (assign11360_e10784 * assign11360_e10796_d_n5)), (((2.0 * locals.var_vtm_dn6) * assign11360_e10796) + (assign11360_e10784 * assign11360_e10796_d_n6)), (assign11360_e10784 * assign11360_e10796_d_n7), (assign11360_e10784 * assign11360_e10796_d_n8), (assign11360_e10784 * assign11360_e10796_d_n9), (assign11360_e10784 * assign11360_e10796_d_n10), (assign11360_e10784 * assign11360_e10796_d_n11), (assign11360_e10784 * assign11360_e10796_d_n12),)
    } else {
        (locals.var_phi, locals.var_phi_dn3, locals.var_phi_dn4, locals.var_phi_dn5, locals.var_phi_dn6, locals.var_phi_dn7, locals.var_phi_dn8, locals.var_phi_dn9, locals.var_phi_dn10, locals.var_phi_dn11, locals.var_phi_dn12,)
    }
};
        locals.var_phi = assign11360_e10799;
        locals.var_phi_dn3 = assign11360_e10799_d_n3;
        locals.var_phi_dn4 = assign11360_e10799_d_n4;
        locals.var_phi_dn5 = assign11360_e10799_d_n5;
        locals.var_phi_dn6 = assign11360_e10799_d_n6;
        locals.var_phi_dn7 = assign11360_e10799_d_n7;
        locals.var_phi_dn8 = assign11360_e10799_d_n8;
        locals.var_phi_dn9 = assign11360_e10799_d_n9;
        locals.var_phi_dn10 = assign11360_e10799_d_n10;
        locals.var_phi_dn11 = assign11360_e10799_d_n11;
        locals.var_phi_dn12 = assign11360_e10799_d_n12;

        let (assign11370_e10804, assign11370_e10804_d_n3, assign11370_e10804_d_n4, assign11370_e10804_d_n5, assign11370_e10804_d_n6, assign11370_e10804_d_n7, assign11370_e10804_d_n8, assign11370_e10804_d_n9, assign11370_e10804_d_n10, assign11370_e10804_d_n11, assign11370_e10804_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11370_e10802: f64 = (locals.var_phi).sqrt();
        (assign11370_e10802, (locals.var_phi_dn3 / (2.0 * assign11370_e10802)), (locals.var_phi_dn4 / (2.0 * assign11370_e10802)), (locals.var_phi_dn5 / (2.0 * assign11370_e10802)), (locals.var_phi_dn6 / (2.0 * assign11370_e10802)), (locals.var_phi_dn7 / (2.0 * assign11370_e10802)), (locals.var_phi_dn8 / (2.0 * assign11370_e10802)), (locals.var_phi_dn9 / (2.0 * assign11370_e10802)), (locals.var_phi_dn10 / (2.0 * assign11370_e10802)), (locals.var_phi_dn11 / (2.0 * assign11370_e10802)), (locals.var_phi_dn12 / (2.0 * assign11370_e10802)),)
    } else {
        (locals.var_sqrtphi, locals.var_sqrtphi_dn3, locals.var_sqrtphi_dn4, locals.var_sqrtphi_dn5, locals.var_sqrtphi_dn6, locals.var_sqrtphi_dn7, locals.var_sqrtphi_dn8, locals.var_sqrtphi_dn9, locals.var_sqrtphi_dn10, locals.var_sqrtphi_dn11, locals.var_sqrtphi_dn12,)
    }
};
        locals.var_sqrtphi = assign11370_e10804;
        locals.var_sqrtphi_dn3 = assign11370_e10804_d_n3;
        locals.var_sqrtphi_dn4 = assign11370_e10804_d_n4;
        locals.var_sqrtphi_dn5 = assign11370_e10804_d_n5;
        locals.var_sqrtphi_dn6 = assign11370_e10804_d_n6;
        locals.var_sqrtphi_dn7 = assign11370_e10804_d_n7;
        locals.var_sqrtphi_dn8 = assign11370_e10804_d_n8;
        locals.var_sqrtphi_dn9 = assign11370_e10804_d_n9;
        locals.var_sqrtphi_dn10 = assign11370_e10804_d_n10;
        locals.var_sqrtphi_dn11 = assign11370_e10804_d_n11;
        locals.var_sqrtphi_dn12 = assign11370_e10804_d_n12;

    }

    pub(super) fn stamp_transient_block_28(
        locals: &mut StampLocals,
    ) {
        let (assign11380_e10819, assign11380_e10819_d_n3, assign11380_e10819_d_n4, assign11380_e10819_d_n5, assign11380_e10819_d_n6, assign11380_e10819_d_n7, assign11380_e10819_d_n8, assign11380_e10819_d_n9, assign11380_e10819_d_n10, assign11380_e10819_d_n11, assign11380_e10819_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11380_e10808: f64 = (2.0 * locals.var_epssub);
        let assign11380_e10811: f64 = (1.602176462e-19 * locals.var_pparam_b4soinpeak);
        let assign11380_e10813: f64 = (assign11380_e10811 * 1000000.0);
        let assign11380_e10814: f64 = (assign11380_e10808 / assign11380_e10813);
        let assign11380_e10815: f64 = (assign11380_e10814).sqrt();
        let assign11380_e10817: f64 = (assign11380_e10815 * locals.var_sqrtphi);
        (assign11380_e10817, ((((-((assign11380_e10808 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn3) * 1000000.0)) / (assign11380_e10813 * assign11380_e10813))) / (2.0 * assign11380_e10815)) * locals.var_sqrtphi) + (assign11380_e10815 * locals.var_sqrtphi_dn3)), ((((-((assign11380_e10808 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn4) * 1000000.0)) / (assign11380_e10813 * assign11380_e10813))) / (2.0 * assign11380_e10815)) * locals.var_sqrtphi) + (assign11380_e10815 * locals.var_sqrtphi_dn4)), ((((-((assign11380_e10808 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn5) * 1000000.0)) / (assign11380_e10813 * assign11380_e10813))) / (2.0 * assign11380_e10815)) * locals.var_sqrtphi) + (assign11380_e10815 * locals.var_sqrtphi_dn5)), ((((-((assign11380_e10808 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn6) * 1000000.0)) / (assign11380_e10813 * assign11380_e10813))) / (2.0 * assign11380_e10815)) * locals.var_sqrtphi) + (assign11380_e10815 * locals.var_sqrtphi_dn6)), ((((-((assign11380_e10808 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn7) * 1000000.0)) / (assign11380_e10813 * assign11380_e10813))) / (2.0 * assign11380_e10815)) * locals.var_sqrtphi) + (assign11380_e10815 * locals.var_sqrtphi_dn7)), ((((-((assign11380_e10808 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn8) * 1000000.0)) / (assign11380_e10813 * assign11380_e10813))) / (2.0 * assign11380_e10815)) * locals.var_sqrtphi) + (assign11380_e10815 * locals.var_sqrtphi_dn8)), ((((-((assign11380_e10808 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn9) * 1000000.0)) / (assign11380_e10813 * assign11380_e10813))) / (2.0 * assign11380_e10815)) * locals.var_sqrtphi) + (assign11380_e10815 * locals.var_sqrtphi_dn9)), ((((-((assign11380_e10808 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn10) * 1000000.0)) / (assign11380_e10813 * assign11380_e10813))) / (2.0 * assign11380_e10815)) * locals.var_sqrtphi) + (assign11380_e10815 * locals.var_sqrtphi_dn10)), ((((-((assign11380_e10808 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn11) * 1000000.0)) / (assign11380_e10813 * assign11380_e10813))) / (2.0 * assign11380_e10815)) * locals.var_sqrtphi) + (assign11380_e10815 * locals.var_sqrtphi_dn11)), ((((-((assign11380_e10808 * ((1.602176462e-19 * locals.var_pparam_b4soinpeak_dn12) * 1000000.0)) / (assign11380_e10813 * assign11380_e10813))) / (2.0 * assign11380_e10815)) * locals.var_sqrtphi) + (assign11380_e10815 * locals.var_sqrtphi_dn12)),)
    } else {
        (locals.var_xdep0, locals.var_xdep0_dn3, locals.var_xdep0_dn4, locals.var_xdep0_dn5, locals.var_xdep0_dn6, locals.var_xdep0_dn7, locals.var_xdep0_dn8, locals.var_xdep0_dn9, locals.var_xdep0_dn10, locals.var_xdep0_dn11, locals.var_xdep0_dn12,)
    }
};
        locals.var_xdep0 = assign11380_e10819;
        locals.var_xdep0_dn3 = assign11380_e10819_d_n3;
        locals.var_xdep0_dn4 = assign11380_e10819_d_n4;
        locals.var_xdep0_dn5 = assign11380_e10819_d_n5;
        locals.var_xdep0_dn6 = assign11380_e10819_d_n6;
        locals.var_xdep0_dn7 = assign11380_e10819_d_n7;
        locals.var_xdep0_dn8 = assign11380_e10819_d_n8;
        locals.var_xdep0_dn9 = assign11380_e10819_d_n9;
        locals.var_xdep0_dn10 = assign11380_e10819_d_n10;
        locals.var_xdep0_dn11 = assign11380_e10819_d_n11;
        locals.var_xdep0_dn12 = assign11380_e10819_d_n12;

        let (assign11390_e10834, assign11390_e10834_d_n3, assign11390_e10834_d_n4, assign11390_e10834_d_n5, assign11390_e10834_d_n6, assign11390_e10834_d_n7, assign11390_e10834_d_n8, assign11390_e10834_d_n9, assign11390_e10834_d_n10, assign11390_e10834_d_n11, assign11390_e10834_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11390_e10823: f64 = (1.602176462e-19 * locals.var_epssub);
        let assign11390_e10825: f64 = (assign11390_e10823 * locals.var_pparam_b4soinpeak);
        let assign11390_e10827: f64 = (assign11390_e10825 * 1000000.0);
        let assign11390_e10829: f64 = (assign11390_e10827 / 2.0);
        let assign11390_e10830: f64 = (assign11390_e10829).sqrt();
        let assign11390_e10832: f64 = (assign11390_e10830 / locals.var_sqrtphi);
        (assign11390_e10832, (((((((assign11390_e10823 * locals.var_pparam_b4soinpeak_dn3) * 1000000.0) / 2.0) / (2.0 * assign11390_e10830)) * locals.var_sqrtphi) - (assign11390_e10830 * locals.var_sqrtphi_dn3)) / (locals.var_sqrtphi * locals.var_sqrtphi)), (((((((assign11390_e10823 * locals.var_pparam_b4soinpeak_dn4) * 1000000.0) / 2.0) / (2.0 * assign11390_e10830)) * locals.var_sqrtphi) - (assign11390_e10830 * locals.var_sqrtphi_dn4)) / (locals.var_sqrtphi * locals.var_sqrtphi)), (((((((assign11390_e10823 * locals.var_pparam_b4soinpeak_dn5) * 1000000.0) / 2.0) / (2.0 * assign11390_e10830)) * locals.var_sqrtphi) - (assign11390_e10830 * locals.var_sqrtphi_dn5)) / (locals.var_sqrtphi * locals.var_sqrtphi)), (((((((assign11390_e10823 * locals.var_pparam_b4soinpeak_dn6) * 1000000.0) / 2.0) / (2.0 * assign11390_e10830)) * locals.var_sqrtphi) - (assign11390_e10830 * locals.var_sqrtphi_dn6)) / (locals.var_sqrtphi * locals.var_sqrtphi)), (((((((assign11390_e10823 * locals.var_pparam_b4soinpeak_dn7) * 1000000.0) / 2.0) / (2.0 * assign11390_e10830)) * locals.var_sqrtphi) - (assign11390_e10830 * locals.var_sqrtphi_dn7)) / (locals.var_sqrtphi * locals.var_sqrtphi)), (((((((assign11390_e10823 * locals.var_pparam_b4soinpeak_dn8) * 1000000.0) / 2.0) / (2.0 * assign11390_e10830)) * locals.var_sqrtphi) - (assign11390_e10830 * locals.var_sqrtphi_dn8)) / (locals.var_sqrtphi * locals.var_sqrtphi)), (((((((assign11390_e10823 * locals.var_pparam_b4soinpeak_dn9) * 1000000.0) / 2.0) / (2.0 * assign11390_e10830)) * locals.var_sqrtphi) - (assign11390_e10830 * locals.var_sqrtphi_dn9)) / (locals.var_sqrtphi * locals.var_sqrtphi)), (((((((assign11390_e10823 * locals.var_pparam_b4soinpeak_dn10) * 1000000.0) / 2.0) / (2.0 * assign11390_e10830)) * locals.var_sqrtphi) - (assign11390_e10830 * locals.var_sqrtphi_dn10)) / (locals.var_sqrtphi * locals.var_sqrtphi)), (((((((assign11390_e10823 * locals.var_pparam_b4soinpeak_dn11) * 1000000.0) / 2.0) / (2.0 * assign11390_e10830)) * locals.var_sqrtphi) - (assign11390_e10830 * locals.var_sqrtphi_dn11)) / (locals.var_sqrtphi * locals.var_sqrtphi)), (((((((assign11390_e10823 * locals.var_pparam_b4soinpeak_dn12) * 1000000.0) / 2.0) / (2.0 * assign11390_e10830)) * locals.var_sqrtphi) - (assign11390_e10830 * locals.var_sqrtphi_dn12)) / (locals.var_sqrtphi * locals.var_sqrtphi)),)
    } else {
        (locals.var_cdep0, locals.var_cdep0_dn3, locals.var_cdep0_dn4, locals.var_cdep0_dn5, locals.var_cdep0_dn6, locals.var_cdep0_dn7, locals.var_cdep0_dn8, locals.var_cdep0_dn9, locals.var_cdep0_dn10, locals.var_cdep0_dn11, locals.var_cdep0_dn12,)
    }
};
        locals.var_cdep0 = assign11390_e10834;
        locals.var_cdep0_dn3 = assign11390_e10834_d_n3;
        locals.var_cdep0_dn4 = assign11390_e10834_d_n4;
        locals.var_cdep0_dn5 = assign11390_e10834_d_n5;
        locals.var_cdep0_dn6 = assign11390_e10834_d_n6;
        locals.var_cdep0_dn7 = assign11390_e10834_d_n7;
        locals.var_cdep0_dn8 = assign11390_e10834_d_n8;
        locals.var_cdep0_dn9 = assign11390_e10834_d_n9;
        locals.var_cdep0_dn10 = assign11390_e10834_d_n10;
        locals.var_cdep0_dn11 = assign11390_e10834_d_n11;
        locals.var_cdep0_dn12 = assign11390_e10834_d_n12;

        let (assign11400_e10847, assign11400_e10847_d_n3, assign11400_e10847_d_n4, assign11400_e10847_d_n5, assign11400_e10847_d_n6, assign11400_e10847_d_n7, assign11400_e10847_d_n8, assign11400_e10847_d_n9, assign11400_e10847_d_n10, assign11400_e10847_d_n11, assign11400_e10847_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11400_e10839: f64 = (locals.var_epsrox * 8.85418e-12);
        let assign11400_e10840: f64 = (locals.var_epssub / assign11400_e10839);
        let assign11400_e10842: f64 = (assign11400_e10840 * locals.var_toxe);
        let assign11400_e10844: f64 = (assign11400_e10842 * locals.var_xdep0);
        let assign11400_e10845: f64 = (assign11400_e10844).sqrt();
        (assign11400_e10845, ((assign11400_e10842 * locals.var_xdep0_dn3) / (2.0 * assign11400_e10845)), ((assign11400_e10842 * locals.var_xdep0_dn4) / (2.0 * assign11400_e10845)), ((assign11400_e10842 * locals.var_xdep0_dn5) / (2.0 * assign11400_e10845)), ((assign11400_e10842 * locals.var_xdep0_dn6) / (2.0 * assign11400_e10845)), ((assign11400_e10842 * locals.var_xdep0_dn7) / (2.0 * assign11400_e10845)), ((assign11400_e10842 * locals.var_xdep0_dn8) / (2.0 * assign11400_e10845)), ((assign11400_e10842 * locals.var_xdep0_dn9) / (2.0 * assign11400_e10845)), ((assign11400_e10842 * locals.var_xdep0_dn10) / (2.0 * assign11400_e10845)), ((assign11400_e10842 * locals.var_xdep0_dn11) / (2.0 * assign11400_e10845)), ((assign11400_e10842 * locals.var_xdep0_dn12) / (2.0 * assign11400_e10845)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign11400_e10847;
        locals.var_t1__blk809_dn3 = assign11400_e10847_d_n3;
        locals.var_t1__blk809_dn4 = assign11400_e10847_d_n4;
        locals.var_t1__blk809_dn5 = assign11400_e10847_d_n5;
        locals.var_t1__blk809_dn6 = assign11400_e10847_d_n6;
        locals.var_t1__blk809_dn7 = assign11400_e10847_d_n7;
        locals.var_t1__blk809_dn8 = assign11400_e10847_d_n8;
        locals.var_t1__blk809_dn9 = assign11400_e10847_d_n9;
        locals.var_t1__blk809_dn10 = assign11400_e10847_d_n10;
        locals.var_t1__blk809_dn11 = assign11400_e10847_d_n11;
        locals.var_t1__blk809_dn12 = assign11400_e10847_d_n12;

        let (assign11410_e10859, assign11410_e10859_d_n3, assign11410_e10859_d_n4, assign11410_e10859_d_n5, assign11410_e10859_d_n6, assign11410_e10859_d_n7, assign11410_e10859_d_n8, assign11410_e10859_d_n9, assign11410_e10859_d_n10, assign11410_e10859_d_n11, assign11410_e10859_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11410_e10850: f64 = (-0.5);
        let assign11410_e10852: f64 = (assign11410_e10850 * locals.var_pparam_b4soidsub);
        let assign11410_e10854: f64 = (assign11410_e10852 * locals.var_pparam_b4soileff);
        let assign11410_e10856: f64 = (assign11410_e10854 / locals.var_t1__blk809);
        let assign11410_e10857: f64 = (assign11410_e10856).exp();
        (assign11410_e10857, (assign11410_e10857 * ((((((assign11410_e10850 * locals.var_pparam_b4soidsub_dn3) * locals.var_pparam_b4soileff) + (assign11410_e10852 * locals.var_pparam_b4soileff_dn3)) * locals.var_t1__blk809) - (assign11410_e10854 * locals.var_t1__blk809_dn3)) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (assign11410_e10857 * ((((((assign11410_e10850 * locals.var_pparam_b4soidsub_dn4) * locals.var_pparam_b4soileff) + (assign11410_e10852 * locals.var_pparam_b4soileff_dn4)) * locals.var_t1__blk809) - (assign11410_e10854 * locals.var_t1__blk809_dn4)) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (assign11410_e10857 * ((((((assign11410_e10850 * locals.var_pparam_b4soidsub_dn5) * locals.var_pparam_b4soileff) + (assign11410_e10852 * locals.var_pparam_b4soileff_dn5)) * locals.var_t1__blk809) - (assign11410_e10854 * locals.var_t1__blk809_dn5)) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (assign11410_e10857 * ((((((assign11410_e10850 * locals.var_pparam_b4soidsub_dn6) * locals.var_pparam_b4soileff) + (assign11410_e10852 * locals.var_pparam_b4soileff_dn6)) * locals.var_t1__blk809) - (assign11410_e10854 * locals.var_t1__blk809_dn6)) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (assign11410_e10857 * ((((((assign11410_e10850 * locals.var_pparam_b4soidsub_dn7) * locals.var_pparam_b4soileff) + (assign11410_e10852 * locals.var_pparam_b4soileff_dn7)) * locals.var_t1__blk809) - (assign11410_e10854 * locals.var_t1__blk809_dn7)) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (assign11410_e10857 * ((((((assign11410_e10850 * locals.var_pparam_b4soidsub_dn8) * locals.var_pparam_b4soileff) + (assign11410_e10852 * locals.var_pparam_b4soileff_dn8)) * locals.var_t1__blk809) - (assign11410_e10854 * locals.var_t1__blk809_dn8)) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (assign11410_e10857 * ((((((assign11410_e10850 * locals.var_pparam_b4soidsub_dn9) * locals.var_pparam_b4soileff) + (assign11410_e10852 * locals.var_pparam_b4soileff_dn9)) * locals.var_t1__blk809) - (assign11410_e10854 * locals.var_t1__blk809_dn9)) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (assign11410_e10857 * ((((((assign11410_e10850 * locals.var_pparam_b4soidsub_dn10) * locals.var_pparam_b4soileff) + (assign11410_e10852 * locals.var_pparam_b4soileff_dn10)) * locals.var_t1__blk809) - (assign11410_e10854 * locals.var_t1__blk809_dn10)) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (assign11410_e10857 * ((((((assign11410_e10850 * locals.var_pparam_b4soidsub_dn11) * locals.var_pparam_b4soileff) + (assign11410_e10852 * locals.var_pparam_b4soileff_dn11)) * locals.var_t1__blk809) - (assign11410_e10854 * locals.var_t1__blk809_dn11)) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (assign11410_e10857 * ((((((assign11410_e10850 * locals.var_pparam_b4soidsub_dn12) * locals.var_pparam_b4soileff) + (assign11410_e10852 * locals.var_pparam_b4soileff_dn12)) * locals.var_t1__blk809) - (assign11410_e10854 * locals.var_t1__blk809_dn12)) / (locals.var_t1__blk809 * locals.var_t1__blk809))),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign11410_e10859;
        locals.var_t0__blk808_dn3 = assign11410_e10859_d_n3;
        locals.var_t0__blk808_dn4 = assign11410_e10859_d_n4;
        locals.var_t0__blk808_dn5 = assign11410_e10859_d_n5;
        locals.var_t0__blk808_dn6 = assign11410_e10859_d_n6;
        locals.var_t0__blk808_dn7 = assign11410_e10859_d_n7;
        locals.var_t0__blk808_dn8 = assign11410_e10859_d_n8;
        locals.var_t0__blk808_dn9 = assign11410_e10859_d_n9;
        locals.var_t0__blk808_dn10 = assign11410_e10859_d_n10;
        locals.var_t0__blk808_dn11 = assign11410_e10859_d_n11;
        locals.var_t0__blk808_dn12 = assign11410_e10859_d_n12;

        let (assign11420_e10869, assign11420_e10869_d_n3, assign11420_e10869_d_n4, assign11420_e10869_d_n5, assign11420_e10869_d_n6, assign11420_e10869_d_n7, assign11420_e10869_d_n8, assign11420_e10869_d_n9, assign11420_e10869_d_n10, assign11420_e10869_d_n11, assign11420_e10869_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11420_e10864: f64 = (2.0 * locals.var_t0__blk808);
        let assign11420_e10866: f64 = (assign11420_e10864 * locals.var_t0__blk808);
        let assign11420_e10867: f64 = (locals.var_t0__blk808 + assign11420_e10866);
        (assign11420_e10867, (locals.var_t0__blk808_dn3 + (((2.0 * locals.var_t0__blk808_dn3) * locals.var_t0__blk808) + (assign11420_e10864 * locals.var_t0__blk808_dn3))), (locals.var_t0__blk808_dn4 + (((2.0 * locals.var_t0__blk808_dn4) * locals.var_t0__blk808) + (assign11420_e10864 * locals.var_t0__blk808_dn4))), (locals.var_t0__blk808_dn5 + (((2.0 * locals.var_t0__blk808_dn5) * locals.var_t0__blk808) + (assign11420_e10864 * locals.var_t0__blk808_dn5))), (locals.var_t0__blk808_dn6 + (((2.0 * locals.var_t0__blk808_dn6) * locals.var_t0__blk808) + (assign11420_e10864 * locals.var_t0__blk808_dn6))), (locals.var_t0__blk808_dn7 + (((2.0 * locals.var_t0__blk808_dn7) * locals.var_t0__blk808) + (assign11420_e10864 * locals.var_t0__blk808_dn7))), (locals.var_t0__blk808_dn8 + (((2.0 * locals.var_t0__blk808_dn8) * locals.var_t0__blk808) + (assign11420_e10864 * locals.var_t0__blk808_dn8))), (locals.var_t0__blk808_dn9 + (((2.0 * locals.var_t0__blk808_dn9) * locals.var_t0__blk808) + (assign11420_e10864 * locals.var_t0__blk808_dn9))), (locals.var_t0__blk808_dn10 + (((2.0 * locals.var_t0__blk808_dn10) * locals.var_t0__blk808) + (assign11420_e10864 * locals.var_t0__blk808_dn10))), (locals.var_t0__blk808_dn11 + (((2.0 * locals.var_t0__blk808_dn11) * locals.var_t0__blk808) + (assign11420_e10864 * locals.var_t0__blk808_dn11))), (locals.var_t0__blk808_dn12 + (((2.0 * locals.var_t0__blk808_dn12) * locals.var_t0__blk808) + (assign11420_e10864 * locals.var_t0__blk808_dn12))),)
    } else {
        (locals.var_theta0vb0, locals.var_theta0vb0_dn3, locals.var_theta0vb0_dn4, locals.var_theta0vb0_dn5, locals.var_theta0vb0_dn6, locals.var_theta0vb0_dn7, locals.var_theta0vb0_dn8, locals.var_theta0vb0_dn9, locals.var_theta0vb0_dn10, locals.var_theta0vb0_dn11, locals.var_theta0vb0_dn12,)
    }
};
        locals.var_theta0vb0 = assign11420_e10869;
        locals.var_theta0vb0_dn3 = assign11420_e10869_d_n3;
        locals.var_theta0vb0_dn4 = assign11420_e10869_d_n4;
        locals.var_theta0vb0_dn5 = assign11420_e10869_d_n5;
        locals.var_theta0vb0_dn6 = assign11420_e10869_d_n6;
        locals.var_theta0vb0_dn7 = assign11420_e10869_d_n7;
        locals.var_theta0vb0_dn8 = assign11420_e10869_d_n8;
        locals.var_theta0vb0_dn9 = assign11420_e10869_d_n9;
        locals.var_theta0vb0_dn10 = assign11420_e10869_d_n10;
        locals.var_theta0vb0_dn11 = assign11420_e10869_d_n11;
        locals.var_theta0vb0_dn12 = assign11420_e10869_d_n12;

        let (assign11430_e10881, assign11430_e10881_d_n3, assign11430_e10881_d_n4, assign11430_e10881_d_n5, assign11430_e10881_d_n6, assign11430_e10881_d_n7, assign11430_e10881_d_n8, assign11430_e10881_d_n9, assign11430_e10881_d_n10, assign11430_e10881_d_n11, assign11430_e10881_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11430_e10872: f64 = (-0.5);
        let assign11430_e10874: f64 = (assign11430_e10872 * locals.var_pparam_b4soidrout);
        let assign11430_e10876: f64 = (assign11430_e10874 * locals.var_pparam_b4soileff);
        let assign11430_e10878: f64 = (assign11430_e10876 / locals.var_t1__blk809);
        let assign11430_e10879: f64 = (assign11430_e10878).exp();
        (assign11430_e10879, (assign11430_e10879 * ((((((assign11430_e10872 * locals.var_pparam_b4soidrout_dn3) * locals.var_pparam_b4soileff) + (assign11430_e10874 * locals.var_pparam_b4soileff_dn3)) * locals.var_t1__blk809) - (assign11430_e10876 * locals.var_t1__blk809_dn3)) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (assign11430_e10879 * ((((((assign11430_e10872 * locals.var_pparam_b4soidrout_dn4) * locals.var_pparam_b4soileff) + (assign11430_e10874 * locals.var_pparam_b4soileff_dn4)) * locals.var_t1__blk809) - (assign11430_e10876 * locals.var_t1__blk809_dn4)) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (assign11430_e10879 * ((((((assign11430_e10872 * locals.var_pparam_b4soidrout_dn5) * locals.var_pparam_b4soileff) + (assign11430_e10874 * locals.var_pparam_b4soileff_dn5)) * locals.var_t1__blk809) - (assign11430_e10876 * locals.var_t1__blk809_dn5)) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (assign11430_e10879 * ((((((assign11430_e10872 * locals.var_pparam_b4soidrout_dn6) * locals.var_pparam_b4soileff) + (assign11430_e10874 * locals.var_pparam_b4soileff_dn6)) * locals.var_t1__blk809) - (assign11430_e10876 * locals.var_t1__blk809_dn6)) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (assign11430_e10879 * ((((((assign11430_e10872 * locals.var_pparam_b4soidrout_dn7) * locals.var_pparam_b4soileff) + (assign11430_e10874 * locals.var_pparam_b4soileff_dn7)) * locals.var_t1__blk809) - (assign11430_e10876 * locals.var_t1__blk809_dn7)) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (assign11430_e10879 * ((((((assign11430_e10872 * locals.var_pparam_b4soidrout_dn8) * locals.var_pparam_b4soileff) + (assign11430_e10874 * locals.var_pparam_b4soileff_dn8)) * locals.var_t1__blk809) - (assign11430_e10876 * locals.var_t1__blk809_dn8)) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (assign11430_e10879 * ((((((assign11430_e10872 * locals.var_pparam_b4soidrout_dn9) * locals.var_pparam_b4soileff) + (assign11430_e10874 * locals.var_pparam_b4soileff_dn9)) * locals.var_t1__blk809) - (assign11430_e10876 * locals.var_t1__blk809_dn9)) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (assign11430_e10879 * ((((((assign11430_e10872 * locals.var_pparam_b4soidrout_dn10) * locals.var_pparam_b4soileff) + (assign11430_e10874 * locals.var_pparam_b4soileff_dn10)) * locals.var_t1__blk809) - (assign11430_e10876 * locals.var_t1__blk809_dn10)) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (assign11430_e10879 * ((((((assign11430_e10872 * locals.var_pparam_b4soidrout_dn11) * locals.var_pparam_b4soileff) + (assign11430_e10874 * locals.var_pparam_b4soileff_dn11)) * locals.var_t1__blk809) - (assign11430_e10876 * locals.var_t1__blk809_dn11)) / (locals.var_t1__blk809 * locals.var_t1__blk809))), (assign11430_e10879 * ((((((assign11430_e10872 * locals.var_pparam_b4soidrout_dn12) * locals.var_pparam_b4soileff) + (assign11430_e10874 * locals.var_pparam_b4soileff_dn12)) * locals.var_t1__blk809) - (assign11430_e10876 * locals.var_t1__blk809_dn12)) / (locals.var_t1__blk809 * locals.var_t1__blk809))),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign11430_e10881;
        locals.var_t0__blk808_dn3 = assign11430_e10881_d_n3;
        locals.var_t0__blk808_dn4 = assign11430_e10881_d_n4;
        locals.var_t0__blk808_dn5 = assign11430_e10881_d_n5;
        locals.var_t0__blk808_dn6 = assign11430_e10881_d_n6;
        locals.var_t0__blk808_dn7 = assign11430_e10881_d_n7;
        locals.var_t0__blk808_dn8 = assign11430_e10881_d_n8;
        locals.var_t0__blk808_dn9 = assign11430_e10881_d_n9;
        locals.var_t0__blk808_dn10 = assign11430_e10881_d_n10;
        locals.var_t0__blk808_dn11 = assign11430_e10881_d_n11;
        locals.var_t0__blk808_dn12 = assign11430_e10881_d_n12;

        let (assign11440_e10891, assign11440_e10891_d_n3, assign11440_e10891_d_n4, assign11440_e10891_d_n5, assign11440_e10891_d_n6, assign11440_e10891_d_n7, assign11440_e10891_d_n8, assign11440_e10891_d_n9, assign11440_e10891_d_n10, assign11440_e10891_d_n11, assign11440_e10891_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11440_e10886: f64 = (2.0 * locals.var_t0__blk808);
        let assign11440_e10888: f64 = (assign11440_e10886 * locals.var_t0__blk808);
        let assign11440_e10889: f64 = (locals.var_t0__blk808 + assign11440_e10888);
        (assign11440_e10889, (locals.var_t0__blk808_dn3 + (((2.0 * locals.var_t0__blk808_dn3) * locals.var_t0__blk808) + (assign11440_e10886 * locals.var_t0__blk808_dn3))), (locals.var_t0__blk808_dn4 + (((2.0 * locals.var_t0__blk808_dn4) * locals.var_t0__blk808) + (assign11440_e10886 * locals.var_t0__blk808_dn4))), (locals.var_t0__blk808_dn5 + (((2.0 * locals.var_t0__blk808_dn5) * locals.var_t0__blk808) + (assign11440_e10886 * locals.var_t0__blk808_dn5))), (locals.var_t0__blk808_dn6 + (((2.0 * locals.var_t0__blk808_dn6) * locals.var_t0__blk808) + (assign11440_e10886 * locals.var_t0__blk808_dn6))), (locals.var_t0__blk808_dn7 + (((2.0 * locals.var_t0__blk808_dn7) * locals.var_t0__blk808) + (assign11440_e10886 * locals.var_t0__blk808_dn7))), (locals.var_t0__blk808_dn8 + (((2.0 * locals.var_t0__blk808_dn8) * locals.var_t0__blk808) + (assign11440_e10886 * locals.var_t0__blk808_dn8))), (locals.var_t0__blk808_dn9 + (((2.0 * locals.var_t0__blk808_dn9) * locals.var_t0__blk808) + (assign11440_e10886 * locals.var_t0__blk808_dn9))), (locals.var_t0__blk808_dn10 + (((2.0 * locals.var_t0__blk808_dn10) * locals.var_t0__blk808) + (assign11440_e10886 * locals.var_t0__blk808_dn10))), (locals.var_t0__blk808_dn11 + (((2.0 * locals.var_t0__blk808_dn11) * locals.var_t0__blk808) + (assign11440_e10886 * locals.var_t0__blk808_dn11))), (locals.var_t0__blk808_dn12 + (((2.0 * locals.var_t0__blk808_dn12) * locals.var_t0__blk808) + (assign11440_e10886 * locals.var_t0__blk808_dn12))),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign11440_e10891;
        locals.var_t2__blk810_dn3 = assign11440_e10891_d_n3;
        locals.var_t2__blk810_dn4 = assign11440_e10891_d_n4;
        locals.var_t2__blk810_dn5 = assign11440_e10891_d_n5;
        locals.var_t2__blk810_dn6 = assign11440_e10891_d_n6;
        locals.var_t2__blk810_dn7 = assign11440_e10891_d_n7;
        locals.var_t2__blk810_dn8 = assign11440_e10891_d_n8;
        locals.var_t2__blk810_dn9 = assign11440_e10891_d_n9;
        locals.var_t2__blk810_dn10 = assign11440_e10891_d_n10;
        locals.var_t2__blk810_dn11 = assign11440_e10891_d_n11;
        locals.var_t2__blk810_dn12 = assign11440_e10891_d_n12;

        let (assign11450_e10899, assign11450_e10899_d_n3, assign11450_e10899_d_n4, assign11450_e10899_d_n5, assign11450_e10899_d_n6, assign11450_e10899_d_n7, assign11450_e10899_d_n8, assign11450_e10899_d_n9, assign11450_e10899_d_n10, assign11450_e10899_d_n11, assign11450_e10899_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11450_e10895: f64 = (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk810);
        let assign11450_e10897: f64 = (assign11450_e10895 + locals.var_pparam_b4soipdibl2);
        (assign11450_e10897, (((locals.var_pparam_b4soipdibl1_dn3 * locals.var_t2__blk810) + (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk810_dn3)) + locals.var_pparam_b4soipdibl2_dn3), (((locals.var_pparam_b4soipdibl1_dn4 * locals.var_t2__blk810) + (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk810_dn4)) + locals.var_pparam_b4soipdibl2_dn4), (((locals.var_pparam_b4soipdibl1_dn5 * locals.var_t2__blk810) + (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk810_dn5)) + locals.var_pparam_b4soipdibl2_dn5), (((locals.var_pparam_b4soipdibl1_dn6 * locals.var_t2__blk810) + (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk810_dn6)) + locals.var_pparam_b4soipdibl2_dn6), (((locals.var_pparam_b4soipdibl1_dn7 * locals.var_t2__blk810) + (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk810_dn7)) + locals.var_pparam_b4soipdibl2_dn7), (((locals.var_pparam_b4soipdibl1_dn8 * locals.var_t2__blk810) + (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk810_dn8)) + locals.var_pparam_b4soipdibl2_dn8), (((locals.var_pparam_b4soipdibl1_dn9 * locals.var_t2__blk810) + (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk810_dn9)) + locals.var_pparam_b4soipdibl2_dn9), (((locals.var_pparam_b4soipdibl1_dn10 * locals.var_t2__blk810) + (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk810_dn10)) + locals.var_pparam_b4soipdibl2_dn10), (((locals.var_pparam_b4soipdibl1_dn11 * locals.var_t2__blk810) + (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk810_dn11)) + locals.var_pparam_b4soipdibl2_dn11), (((locals.var_pparam_b4soipdibl1_dn12 * locals.var_t2__blk810) + (locals.var_pparam_b4soipdibl1 * locals.var_t2__blk810_dn12)) + locals.var_pparam_b4soipdibl2_dn12),)
    } else {
        (locals.var_thetarout, locals.var_thetarout_dn3, locals.var_thetarout_dn4, locals.var_thetarout_dn5, locals.var_thetarout_dn6, locals.var_thetarout_dn7, locals.var_thetarout_dn8, locals.var_thetarout_dn9, locals.var_thetarout_dn10, locals.var_thetarout_dn11, locals.var_thetarout_dn12,)
    }
};
        locals.var_thetarout = assign11450_e10899;
        locals.var_thetarout_dn3 = assign11450_e10899_d_n3;
        locals.var_thetarout_dn4 = assign11450_e10899_d_n4;
        locals.var_thetarout_dn5 = assign11450_e10899_d_n5;
        locals.var_thetarout_dn6 = assign11450_e10899_d_n6;
        locals.var_thetarout_dn7 = assign11450_e10899_d_n7;
        locals.var_thetarout_dn8 = assign11450_e10899_d_n8;
        locals.var_thetarout_dn9 = assign11450_e10899_d_n9;
        locals.var_thetarout_dn10 = assign11450_e10899_d_n10;
        locals.var_thetarout_dn11 = assign11450_e10899_d_n11;
        locals.var_thetarout_dn12 = assign11450_e10899_d_n12;

        let (assign11460_e10903, assign11460_e10903_d_n4, assign11460_e10903_d_n5, assign11460_e10903_d_n6,) = {
    if (locals.var_guard1124 != 0.0) {
        (locals.var_vtm, locals.var_vtm_dn4, locals.var_vtm_dn5, locals.var_vtm_dn6,)
    } else {
        (locals.var_b4soivtm, locals.var_b4soivtm_dn4, locals.var_b4soivtm_dn5, locals.var_b4soivtm_dn6,)
    }
};
        locals.var_b4soivtm = assign11460_e10903;
        locals.var_b4soivtm_dn4 = assign11460_e10903_d_n4;
        locals.var_b4soivtm_dn5 = assign11460_e10903_d_n5;
        locals.var_b4soivtm_dn6 = assign11460_e10903_d_n6;

        let (assign11470_e10911, assign11470_e10911_d_n3, assign11470_e10911_d_n4, assign11470_e10911_d_n5, assign11470_e10911_d_n6, assign11470_e10911_d_n7, assign11470_e10911_d_n8, assign11470_e10911_d_n9, assign11470_e10911_d_n10, assign11470_e10911_d_n11, assign11470_e10911_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11470_e10907: f64 = (1.115 / locals.var_vtm);
        let assign11470_e10909: f64 = (assign11470_e10907 * locals.var_trm1);
        (assign11470_e10909, 0.0, (((-((1.115 * locals.var_vtm_dn4) / (locals.var_vtm * locals.var_vtm))) * locals.var_trm1) + (assign11470_e10907 * locals.var_trm1_dn4)), (((-((1.115 * locals.var_vtm_dn5) / (locals.var_vtm * locals.var_vtm))) * locals.var_trm1) + (assign11470_e10907 * locals.var_trm1_dn5)), (((-((1.115 * locals.var_vtm_dn6) / (locals.var_vtm * locals.var_vtm))) * locals.var_trm1) + (assign11470_e10907 * locals.var_trm1_dn6)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign11470_e10911;
        locals.var_t4__blk812_dn3 = assign11470_e10911_d_n3;
        locals.var_t4__blk812_dn4 = assign11470_e10911_d_n4;
        locals.var_t4__blk812_dn5 = assign11470_e10911_d_n5;
        locals.var_t4__blk812_dn6 = assign11470_e10911_d_n6;
        locals.var_t4__blk812_dn7 = assign11470_e10911_d_n7;
        locals.var_t4__blk812_dn8 = assign11470_e10911_d_n8;
        locals.var_t4__blk812_dn9 = assign11470_e10911_d_n9;
        locals.var_t4__blk812_dn10 = assign11470_e10911_d_n10;
        locals.var_t4__blk812_dn11 = assign11470_e10911_d_n11;
        locals.var_t4__blk812_dn12 = assign11470_e10911_d_n12;

        let (assign11480_e10919, assign11480_e10919_d_n3, assign11480_e10919_d_n4, assign11480_e10919_d_n5, assign11480_e10919_d_n6, assign11480_e10919_d_n7, assign11480_e10919_d_n8, assign11480_e10919_d_n9, assign11480_e10919_d_n10, assign11480_e10919_d_n11, assign11480_e10919_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11480_e10915: f64 = (locals.var_pparam_b4soixbjt * locals.var_t4__blk812);
        let assign11480_e10917: f64 = (assign11480_e10915 / locals.var_pparam_b4soindiode);
        (assign11480_e10917, (((((locals.var_pparam_b4soixbjt_dn3 * locals.var_t4__blk812) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk812_dn3)) * locals.var_pparam_b4soindiode) - (assign11480_e10915 * locals.var_pparam_b4soindiode_dn3)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixbjt_dn4 * locals.var_t4__blk812) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk812_dn4)) * locals.var_pparam_b4soindiode) - (assign11480_e10915 * locals.var_pparam_b4soindiode_dn4)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixbjt_dn5 * locals.var_t4__blk812) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk812_dn5)) * locals.var_pparam_b4soindiode) - (assign11480_e10915 * locals.var_pparam_b4soindiode_dn5)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixbjt_dn6 * locals.var_t4__blk812) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk812_dn6)) * locals.var_pparam_b4soindiode) - (assign11480_e10915 * locals.var_pparam_b4soindiode_dn6)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixbjt_dn7 * locals.var_t4__blk812) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk812_dn7)) * locals.var_pparam_b4soindiode) - (assign11480_e10915 * locals.var_pparam_b4soindiode_dn7)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixbjt_dn8 * locals.var_t4__blk812) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk812_dn8)) * locals.var_pparam_b4soindiode) - (assign11480_e10915 * locals.var_pparam_b4soindiode_dn8)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixbjt_dn9 * locals.var_t4__blk812) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk812_dn9)) * locals.var_pparam_b4soindiode) - (assign11480_e10915 * locals.var_pparam_b4soindiode_dn9)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixbjt_dn10 * locals.var_t4__blk812) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk812_dn10)) * locals.var_pparam_b4soindiode) - (assign11480_e10915 * locals.var_pparam_b4soindiode_dn10)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixbjt_dn11 * locals.var_t4__blk812) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk812_dn11)) * locals.var_pparam_b4soindiode) - (assign11480_e10915 * locals.var_pparam_b4soindiode_dn11)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixbjt_dn12 * locals.var_t4__blk812) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk812_dn12)) * locals.var_pparam_b4soindiode) - (assign11480_e10915 * locals.var_pparam_b4soindiode_dn12)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign11480_e10919;
        locals.var_t7__blk815_dn3 = assign11480_e10919_d_n3;
        locals.var_t7__blk815_dn4 = assign11480_e10919_d_n4;
        locals.var_t7__blk815_dn5 = assign11480_e10919_d_n5;
        locals.var_t7__blk815_dn6 = assign11480_e10919_d_n6;
        locals.var_t7__blk815_dn7 = assign11480_e10919_d_n7;
        locals.var_t7__blk815_dn8 = assign11480_e10919_d_n8;
        locals.var_t7__blk815_dn9 = assign11480_e10919_d_n9;
        locals.var_t7__blk815_dn10 = assign11480_e10919_d_n10;
        locals.var_t7__blk815_dn11 = assign11480_e10919_d_n11;
        locals.var_t7__blk815_dn12 = assign11480_e10919_d_n12;

        let assign11490_e10922: f64 = if locals.var_t7__blk815 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1128 = assign11490_e10922;

        let (assign11500_e10934, assign11500_e10934_d_n3, assign11500_e10934_d_n4, assign11500_e10934_d_n5, assign11500_e10934_d_n6, assign11500_e10934_d_n7, assign11500_e10934_d_n8, assign11500_e10934_d_n9, assign11500_e10934_d_n10, assign11500_e10934_d_n11, assign11500_e10934_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1128 != 0.0)) {
        let assign11500_e10929: f64 = (1.0 + locals.var_t7__blk815);
        let assign11500_e10931: f64 = (assign11500_e10929 - 100.0);
        let assign11500_e10932: f64 = (2.688117142e43 * assign11500_e10931);
        (assign11500_e10932, (2.688117142e43 * locals.var_t7__blk815_dn3), (2.688117142e43 * locals.var_t7__blk815_dn4), (2.688117142e43 * locals.var_t7__blk815_dn5), (2.688117142e43 * locals.var_t7__blk815_dn6), (2.688117142e43 * locals.var_t7__blk815_dn7), (2.688117142e43 * locals.var_t7__blk815_dn8), (2.688117142e43 * locals.var_t7__blk815_dn9), (2.688117142e43 * locals.var_t7__blk815_dn10), (2.688117142e43 * locals.var_t7__blk815_dn11), (2.688117142e43 * locals.var_t7__blk815_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign11500_e10934;
        locals.var_t0__blk808_dn3 = assign11500_e10934_d_n3;
        locals.var_t0__blk808_dn4 = assign11500_e10934_d_n4;
        locals.var_t0__blk808_dn5 = assign11500_e10934_d_n5;
        locals.var_t0__blk808_dn6 = assign11500_e10934_d_n6;
        locals.var_t0__blk808_dn7 = assign11500_e10934_d_n7;
        locals.var_t0__blk808_dn8 = assign11500_e10934_d_n8;
        locals.var_t0__blk808_dn9 = assign11500_e10934_d_n9;
        locals.var_t0__blk808_dn10 = assign11500_e10934_d_n10;
        locals.var_t0__blk808_dn11 = assign11500_e10934_d_n11;
        locals.var_t0__blk808_dn12 = assign11500_e10934_d_n12;

        let assign11510_e10937: f64 = (-100.0);
        let assign11510_e10938: f64 = if locals.var_t7__blk815 < assign11510_e10937 { 1.0 } else { 0.0 };
        locals.var_guard1129 = assign11510_e10938;

        let (assign11520_e10947, assign11520_e10947_d_n3, assign11520_e10947_d_n4, assign11520_e10947_d_n5, assign11520_e10947_d_n6, assign11520_e10947_d_n7, assign11520_e10947_d_n8, assign11520_e10947_d_n9, assign11520_e10947_d_n10, assign11520_e10947_d_n11, assign11520_e10947_d_n12,) = {
    if (((locals.var_guard1124 != 0.0) && (locals.var_guard1128 == 0.0)) && (locals.var_guard1129 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign11520_e10947;
        locals.var_t0__blk808_dn3 = assign11520_e10947_d_n3;
        locals.var_t0__blk808_dn4 = assign11520_e10947_d_n4;
        locals.var_t0__blk808_dn5 = assign11520_e10947_d_n5;
        locals.var_t0__blk808_dn6 = assign11520_e10947_d_n6;
        locals.var_t0__blk808_dn7 = assign11520_e10947_d_n7;
        locals.var_t0__blk808_dn8 = assign11520_e10947_d_n8;
        locals.var_t0__blk808_dn9 = assign11520_e10947_d_n9;
        locals.var_t0__blk808_dn10 = assign11520_e10947_d_n10;
        locals.var_t0__blk808_dn11 = assign11520_e10947_d_n11;
        locals.var_t0__blk808_dn12 = assign11520_e10947_d_n12;

        let (assign11530_e10958, assign11530_e10958_d_n3, assign11530_e10958_d_n4, assign11530_e10958_d_n5, assign11530_e10958_d_n6, assign11530_e10958_d_n7, assign11530_e10958_d_n8, assign11530_e10958_d_n9, assign11530_e10958_d_n10, assign11530_e10958_d_n11, assign11530_e10958_d_n12,) = {
    if (((locals.var_guard1124 != 0.0) && (locals.var_guard1128 == 0.0)) && (locals.var_guard1129 == 0.0)) {
        let assign11530_e10956: f64 = (locals.var_t7__blk815).exp();
        (assign11530_e10956, (assign11530_e10956 * locals.var_t7__blk815_dn3), (assign11530_e10956 * locals.var_t7__blk815_dn4), (assign11530_e10956 * locals.var_t7__blk815_dn5), (assign11530_e10956 * locals.var_t7__blk815_dn6), (assign11530_e10956 * locals.var_t7__blk815_dn7), (assign11530_e10956 * locals.var_t7__blk815_dn8), (assign11530_e10956 * locals.var_t7__blk815_dn9), (assign11530_e10956 * locals.var_t7__blk815_dn10), (assign11530_e10956 * locals.var_t7__blk815_dn11), (assign11530_e10956 * locals.var_t7__blk815_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign11530_e10958;
        locals.var_t0__blk808_dn3 = assign11530_e10958_d_n3;
        locals.var_t0__blk808_dn4 = assign11530_e10958_d_n4;
        locals.var_t0__blk808_dn5 = assign11530_e10958_d_n5;
        locals.var_t0__blk808_dn6 = assign11530_e10958_d_n6;
        locals.var_t0__blk808_dn7 = assign11530_e10958_d_n7;
        locals.var_t0__blk808_dn8 = assign11530_e10958_d_n8;
        locals.var_t0__blk808_dn9 = assign11530_e10958_d_n9;
        locals.var_t0__blk808_dn10 = assign11530_e10958_d_n10;
        locals.var_t0__blk808_dn11 = assign11530_e10958_d_n11;
        locals.var_t0__blk808_dn12 = assign11530_e10958_d_n12;

        let assign11540_e10961: f64 = if locals.var_pparam_b4soixbjt == locals.var_pparam_b4soixdif { 1.0 } else { 0.0 };
        locals.var_guard1130 = assign11540_e10961;

        let (assign11550_e10967, assign11550_e10967_d_n3, assign11550_e10967_d_n4, assign11550_e10967_d_n5, assign11550_e10967_d_n6, assign11550_e10967_d_n7, assign11550_e10967_d_n8, assign11550_e10967_d_n9, assign11550_e10967_d_n10, assign11550_e10967_d_n11, assign11550_e10967_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1130 != 0.0)) {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign11550_e10967;
        locals.var_t1__blk809_dn3 = assign11550_e10967_d_n3;
        locals.var_t1__blk809_dn4 = assign11550_e10967_d_n4;
        locals.var_t1__blk809_dn5 = assign11550_e10967_d_n5;
        locals.var_t1__blk809_dn6 = assign11550_e10967_d_n6;
        locals.var_t1__blk809_dn7 = assign11550_e10967_d_n7;
        locals.var_t1__blk809_dn8 = assign11550_e10967_d_n8;
        locals.var_t1__blk809_dn9 = assign11550_e10967_d_n9;
        locals.var_t1__blk809_dn10 = assign11550_e10967_d_n10;
        locals.var_t1__blk809_dn11 = assign11550_e10967_d_n11;
        locals.var_t1__blk809_dn12 = assign11550_e10967_d_n12;

        let (assign11560_e10978, assign11560_e10978_d_n3, assign11560_e10978_d_n4, assign11560_e10978_d_n5, assign11560_e10978_d_n6, assign11560_e10978_d_n7, assign11560_e10978_d_n8, assign11560_e10978_d_n9, assign11560_e10978_d_n10, assign11560_e10978_d_n11, assign11560_e10978_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1130 == 0.0)) {
        let assign11560_e10974: f64 = (locals.var_pparam_b4soixdif * locals.var_t4__blk812);
        let assign11560_e10976: f64 = (assign11560_e10974 / locals.var_pparam_b4soindiode);
        (assign11560_e10976, (((((locals.var_pparam_b4soixdif_dn3 * locals.var_t4__blk812) + (locals.var_pparam_b4soixdif * locals.var_t4__blk812_dn3)) * locals.var_pparam_b4soindiode) - (assign11560_e10974 * locals.var_pparam_b4soindiode_dn3)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixdif_dn4 * locals.var_t4__blk812) + (locals.var_pparam_b4soixdif * locals.var_t4__blk812_dn4)) * locals.var_pparam_b4soindiode) - (assign11560_e10974 * locals.var_pparam_b4soindiode_dn4)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixdif_dn5 * locals.var_t4__blk812) + (locals.var_pparam_b4soixdif * locals.var_t4__blk812_dn5)) * locals.var_pparam_b4soindiode) - (assign11560_e10974 * locals.var_pparam_b4soindiode_dn5)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixdif_dn6 * locals.var_t4__blk812) + (locals.var_pparam_b4soixdif * locals.var_t4__blk812_dn6)) * locals.var_pparam_b4soindiode) - (assign11560_e10974 * locals.var_pparam_b4soindiode_dn6)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixdif_dn7 * locals.var_t4__blk812) + (locals.var_pparam_b4soixdif * locals.var_t4__blk812_dn7)) * locals.var_pparam_b4soindiode) - (assign11560_e10974 * locals.var_pparam_b4soindiode_dn7)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixdif_dn8 * locals.var_t4__blk812) + (locals.var_pparam_b4soixdif * locals.var_t4__blk812_dn8)) * locals.var_pparam_b4soindiode) - (assign11560_e10974 * locals.var_pparam_b4soindiode_dn8)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixdif_dn9 * locals.var_t4__blk812) + (locals.var_pparam_b4soixdif * locals.var_t4__blk812_dn9)) * locals.var_pparam_b4soindiode) - (assign11560_e10974 * locals.var_pparam_b4soindiode_dn9)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixdif_dn10 * locals.var_t4__blk812) + (locals.var_pparam_b4soixdif * locals.var_t4__blk812_dn10)) * locals.var_pparam_b4soindiode) - (assign11560_e10974 * locals.var_pparam_b4soindiode_dn10)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixdif_dn11 * locals.var_t4__blk812) + (locals.var_pparam_b4soixdif * locals.var_t4__blk812_dn11)) * locals.var_pparam_b4soindiode) - (assign11560_e10974 * locals.var_pparam_b4soindiode_dn11)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)), (((((locals.var_pparam_b4soixdif_dn12 * locals.var_t4__blk812) + (locals.var_pparam_b4soixdif * locals.var_t4__blk812_dn12)) * locals.var_pparam_b4soindiode) - (assign11560_e10974 * locals.var_pparam_b4soindiode_dn12)) / (locals.var_pparam_b4soindiode * locals.var_pparam_b4soindiode)),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign11560_e10978;
        locals.var_t7__blk815_dn3 = assign11560_e10978_d_n3;
        locals.var_t7__blk815_dn4 = assign11560_e10978_d_n4;
        locals.var_t7__blk815_dn5 = assign11560_e10978_d_n5;
        locals.var_t7__blk815_dn6 = assign11560_e10978_d_n6;
        locals.var_t7__blk815_dn7 = assign11560_e10978_d_n7;
        locals.var_t7__blk815_dn8 = assign11560_e10978_d_n8;
        locals.var_t7__blk815_dn9 = assign11560_e10978_d_n9;
        locals.var_t7__blk815_dn10 = assign11560_e10978_d_n10;
        locals.var_t7__blk815_dn11 = assign11560_e10978_d_n11;
        locals.var_t7__blk815_dn12 = assign11560_e10978_d_n12;

        let assign11570_e10981: f64 = if locals.var_t7__blk815 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1131 = assign11570_e10981;

        let (assign11580_e10996, assign11580_e10996_d_n3, assign11580_e10996_d_n4, assign11580_e10996_d_n5, assign11580_e10996_d_n6, assign11580_e10996_d_n7, assign11580_e10996_d_n8, assign11580_e10996_d_n9, assign11580_e10996_d_n10, assign11580_e10996_d_n11, assign11580_e10996_d_n12,) = {
    if (((locals.var_guard1124 != 0.0) && (locals.var_guard1130 == 0.0)) && (locals.var_guard1131 != 0.0)) {
        let assign11580_e10991: f64 = (1.0 + locals.var_t7__blk815);
        let assign11580_e10993: f64 = (assign11580_e10991 - 100.0);
        let assign11580_e10994: f64 = (2.688117142e43 * assign11580_e10993);
        (assign11580_e10994, (2.688117142e43 * locals.var_t7__blk815_dn3), (2.688117142e43 * locals.var_t7__blk815_dn4), (2.688117142e43 * locals.var_t7__blk815_dn5), (2.688117142e43 * locals.var_t7__blk815_dn6), (2.688117142e43 * locals.var_t7__blk815_dn7), (2.688117142e43 * locals.var_t7__blk815_dn8), (2.688117142e43 * locals.var_t7__blk815_dn9), (2.688117142e43 * locals.var_t7__blk815_dn10), (2.688117142e43 * locals.var_t7__blk815_dn11), (2.688117142e43 * locals.var_t7__blk815_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign11580_e10996;
        locals.var_t1__blk809_dn3 = assign11580_e10996_d_n3;
        locals.var_t1__blk809_dn4 = assign11580_e10996_d_n4;
        locals.var_t1__blk809_dn5 = assign11580_e10996_d_n5;
        locals.var_t1__blk809_dn6 = assign11580_e10996_d_n6;
        locals.var_t1__blk809_dn7 = assign11580_e10996_d_n7;
        locals.var_t1__blk809_dn8 = assign11580_e10996_d_n8;
        locals.var_t1__blk809_dn9 = assign11580_e10996_d_n9;
        locals.var_t1__blk809_dn10 = assign11580_e10996_d_n10;
        locals.var_t1__blk809_dn11 = assign11580_e10996_d_n11;
        locals.var_t1__blk809_dn12 = assign11580_e10996_d_n12;

        let assign11590_e10999: f64 = (-100.0);
        let assign11590_e11000: f64 = if locals.var_t7__blk815 < assign11590_e10999 { 1.0 } else { 0.0 };
        locals.var_guard1132 = assign11590_e11000;

        let (assign11600_e11012, assign11600_e11012_d_n3, assign11600_e11012_d_n4, assign11600_e11012_d_n5, assign11600_e11012_d_n6, assign11600_e11012_d_n7, assign11600_e11012_d_n8, assign11600_e11012_d_n9, assign11600_e11012_d_n10, assign11600_e11012_d_n11, assign11600_e11012_d_n12,) = {
    if ((((locals.var_guard1124 != 0.0) && (locals.var_guard1130 == 0.0)) && (locals.var_guard1131 == 0.0)) && (locals.var_guard1132 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign11600_e11012;
        locals.var_t1__blk809_dn3 = assign11600_e11012_d_n3;
        locals.var_t1__blk809_dn4 = assign11600_e11012_d_n4;
        locals.var_t1__blk809_dn5 = assign11600_e11012_d_n5;
        locals.var_t1__blk809_dn6 = assign11600_e11012_d_n6;
        locals.var_t1__blk809_dn7 = assign11600_e11012_d_n7;
        locals.var_t1__blk809_dn8 = assign11600_e11012_d_n8;
        locals.var_t1__blk809_dn9 = assign11600_e11012_d_n9;
        locals.var_t1__blk809_dn10 = assign11600_e11012_d_n10;
        locals.var_t1__blk809_dn11 = assign11600_e11012_d_n11;
        locals.var_t1__blk809_dn12 = assign11600_e11012_d_n12;

        let (assign11610_e11026, assign11610_e11026_d_n3, assign11610_e11026_d_n4, assign11610_e11026_d_n5, assign11610_e11026_d_n6, assign11610_e11026_d_n7, assign11610_e11026_d_n8, assign11610_e11026_d_n9, assign11610_e11026_d_n10, assign11610_e11026_d_n11, assign11610_e11026_d_n12,) = {
    if ((((locals.var_guard1124 != 0.0) && (locals.var_guard1130 == 0.0)) && (locals.var_guard1131 == 0.0)) && (locals.var_guard1132 == 0.0)) {
        let assign11610_e11024: f64 = (locals.var_t7__blk815).exp();
        (assign11610_e11024, (assign11610_e11024 * locals.var_t7__blk815_dn3), (assign11610_e11024 * locals.var_t7__blk815_dn4), (assign11610_e11024 * locals.var_t7__blk815_dn5), (assign11610_e11024 * locals.var_t7__blk815_dn6), (assign11610_e11024 * locals.var_t7__blk815_dn7), (assign11610_e11024 * locals.var_t7__blk815_dn8), (assign11610_e11024 * locals.var_t7__blk815_dn9), (assign11610_e11024 * locals.var_t7__blk815_dn10), (assign11610_e11024 * locals.var_t7__blk815_dn11), (assign11610_e11024 * locals.var_t7__blk815_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign11610_e11026;
        locals.var_t1__blk809_dn3 = assign11610_e11026_d_n3;
        locals.var_t1__blk809_dn4 = assign11610_e11026_d_n4;
        locals.var_t1__blk809_dn5 = assign11610_e11026_d_n5;
        locals.var_t1__blk809_dn6 = assign11610_e11026_d_n6;
        locals.var_t1__blk809_dn7 = assign11610_e11026_d_n7;
        locals.var_t1__blk809_dn8 = assign11610_e11026_d_n8;
        locals.var_t1__blk809_dn9 = assign11610_e11026_d_n9;
        locals.var_t1__blk809_dn10 = assign11610_e11026_d_n10;
        locals.var_t1__blk809_dn11 = assign11610_e11026_d_n11;
        locals.var_t1__blk809_dn12 = assign11610_e11026_d_n12;

        let (assign11620_e11034, assign11620_e11034_d_n3, assign11620_e11034_d_n4, assign11620_e11034_d_n5, assign11620_e11034_d_n6, assign11620_e11034_d_n7, assign11620_e11034_d_n8, assign11620_e11034_d_n9, assign11620_e11034_d_n10, assign11620_e11034_d_n11, assign11620_e11034_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11620_e11030: f64 = (locals.var_pparam_b4soixrec * locals.var_t4__blk812);
        let assign11620_e11032: f64 = (assign11620_e11030 / locals.var_pparam_b4soinrecf0);
        (assign11620_e11032, (((((locals.var_pparam_b4soixrec_dn3 * locals.var_t4__blk812) + (locals.var_pparam_b4soixrec * locals.var_t4__blk812_dn3)) * locals.var_pparam_b4soinrecf0) - (assign11620_e11030 * locals.var_pparam_b4soinrecf0_dn3)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0)), (((((locals.var_pparam_b4soixrec_dn4 * locals.var_t4__blk812) + (locals.var_pparam_b4soixrec * locals.var_t4__blk812_dn4)) * locals.var_pparam_b4soinrecf0) - (assign11620_e11030 * locals.var_pparam_b4soinrecf0_dn4)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0)), (((((locals.var_pparam_b4soixrec_dn5 * locals.var_t4__blk812) + (locals.var_pparam_b4soixrec * locals.var_t4__blk812_dn5)) * locals.var_pparam_b4soinrecf0) - (assign11620_e11030 * locals.var_pparam_b4soinrecf0_dn5)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0)), (((((locals.var_pparam_b4soixrec_dn6 * locals.var_t4__blk812) + (locals.var_pparam_b4soixrec * locals.var_t4__blk812_dn6)) * locals.var_pparam_b4soinrecf0) - (assign11620_e11030 * locals.var_pparam_b4soinrecf0_dn6)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0)), (((((locals.var_pparam_b4soixrec_dn7 * locals.var_t4__blk812) + (locals.var_pparam_b4soixrec * locals.var_t4__blk812_dn7)) * locals.var_pparam_b4soinrecf0) - (assign11620_e11030 * locals.var_pparam_b4soinrecf0_dn7)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0)), (((((locals.var_pparam_b4soixrec_dn8 * locals.var_t4__blk812) + (locals.var_pparam_b4soixrec * locals.var_t4__blk812_dn8)) * locals.var_pparam_b4soinrecf0) - (assign11620_e11030 * locals.var_pparam_b4soinrecf0_dn8)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0)), (((((locals.var_pparam_b4soixrec_dn9 * locals.var_t4__blk812) + (locals.var_pparam_b4soixrec * locals.var_t4__blk812_dn9)) * locals.var_pparam_b4soinrecf0) - (assign11620_e11030 * locals.var_pparam_b4soinrecf0_dn9)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0)), (((((locals.var_pparam_b4soixrec_dn10 * locals.var_t4__blk812) + (locals.var_pparam_b4soixrec * locals.var_t4__blk812_dn10)) * locals.var_pparam_b4soinrecf0) - (assign11620_e11030 * locals.var_pparam_b4soinrecf0_dn10)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0)), (((((locals.var_pparam_b4soixrec_dn11 * locals.var_t4__blk812) + (locals.var_pparam_b4soixrec * locals.var_t4__blk812_dn11)) * locals.var_pparam_b4soinrecf0) - (assign11620_e11030 * locals.var_pparam_b4soinrecf0_dn11)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0)), (((((locals.var_pparam_b4soixrec_dn12 * locals.var_t4__blk812) + (locals.var_pparam_b4soixrec * locals.var_t4__blk812_dn12)) * locals.var_pparam_b4soinrecf0) - (assign11620_e11030 * locals.var_pparam_b4soinrecf0_dn12)) / (locals.var_pparam_b4soinrecf0 * locals.var_pparam_b4soinrecf0)),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign11620_e11034;
        locals.var_t7__blk815_dn3 = assign11620_e11034_d_n3;
        locals.var_t7__blk815_dn4 = assign11620_e11034_d_n4;
        locals.var_t7__blk815_dn5 = assign11620_e11034_d_n5;
        locals.var_t7__blk815_dn6 = assign11620_e11034_d_n6;
        locals.var_t7__blk815_dn7 = assign11620_e11034_d_n7;
        locals.var_t7__blk815_dn8 = assign11620_e11034_d_n8;
        locals.var_t7__blk815_dn9 = assign11620_e11034_d_n9;
        locals.var_t7__blk815_dn10 = assign11620_e11034_d_n10;
        locals.var_t7__blk815_dn11 = assign11620_e11034_d_n11;
        locals.var_t7__blk815_dn12 = assign11620_e11034_d_n12;

        let assign11630_e11037: f64 = if locals.var_t7__blk815 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1133 = assign11630_e11037;

        let (assign11640_e11049, assign11640_e11049_d_n3, assign11640_e11049_d_n4, assign11640_e11049_d_n5, assign11640_e11049_d_n6, assign11640_e11049_d_n7, assign11640_e11049_d_n8, assign11640_e11049_d_n9, assign11640_e11049_d_n10, assign11640_e11049_d_n11, assign11640_e11049_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1133 != 0.0)) {
        let assign11640_e11044: f64 = (1.0 + locals.var_t7__blk815);
        let assign11640_e11046: f64 = (assign11640_e11044 - 100.0);
        let assign11640_e11047: f64 = (2.688117142e43 * assign11640_e11046);
        (assign11640_e11047, (2.688117142e43 * locals.var_t7__blk815_dn3), (2.688117142e43 * locals.var_t7__blk815_dn4), (2.688117142e43 * locals.var_t7__blk815_dn5), (2.688117142e43 * locals.var_t7__blk815_dn6), (2.688117142e43 * locals.var_t7__blk815_dn7), (2.688117142e43 * locals.var_t7__blk815_dn8), (2.688117142e43 * locals.var_t7__blk815_dn9), (2.688117142e43 * locals.var_t7__blk815_dn10), (2.688117142e43 * locals.var_t7__blk815_dn11), (2.688117142e43 * locals.var_t7__blk815_dn12),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign11640_e11049;
        locals.var_t2__blk810_dn3 = assign11640_e11049_d_n3;
        locals.var_t2__blk810_dn4 = assign11640_e11049_d_n4;
        locals.var_t2__blk810_dn5 = assign11640_e11049_d_n5;
        locals.var_t2__blk810_dn6 = assign11640_e11049_d_n6;
        locals.var_t2__blk810_dn7 = assign11640_e11049_d_n7;
        locals.var_t2__blk810_dn8 = assign11640_e11049_d_n8;
        locals.var_t2__blk810_dn9 = assign11640_e11049_d_n9;
        locals.var_t2__blk810_dn10 = assign11640_e11049_d_n10;
        locals.var_t2__blk810_dn11 = assign11640_e11049_d_n11;
        locals.var_t2__blk810_dn12 = assign11640_e11049_d_n12;

        let assign11650_e11052: f64 = (-100.0);
        let assign11650_e11053: f64 = if locals.var_t7__blk815 < assign11650_e11052 { 1.0 } else { 0.0 };
        locals.var_guard1134 = assign11650_e11053;

        let (assign11660_e11062, assign11660_e11062_d_n3, assign11660_e11062_d_n4, assign11660_e11062_d_n5, assign11660_e11062_d_n6, assign11660_e11062_d_n7, assign11660_e11062_d_n8, assign11660_e11062_d_n9, assign11660_e11062_d_n10, assign11660_e11062_d_n11, assign11660_e11062_d_n12,) = {
    if (((locals.var_guard1124 != 0.0) && (locals.var_guard1133 == 0.0)) && (locals.var_guard1134 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign11660_e11062;
        locals.var_t2__blk810_dn3 = assign11660_e11062_d_n3;
        locals.var_t2__blk810_dn4 = assign11660_e11062_d_n4;
        locals.var_t2__blk810_dn5 = assign11660_e11062_d_n5;
        locals.var_t2__blk810_dn6 = assign11660_e11062_d_n6;
        locals.var_t2__blk810_dn7 = assign11660_e11062_d_n7;
        locals.var_t2__blk810_dn8 = assign11660_e11062_d_n8;
        locals.var_t2__blk810_dn9 = assign11660_e11062_d_n9;
        locals.var_t2__blk810_dn10 = assign11660_e11062_d_n10;
        locals.var_t2__blk810_dn11 = assign11660_e11062_d_n11;
        locals.var_t2__blk810_dn12 = assign11660_e11062_d_n12;

        let (assign11670_e11073, assign11670_e11073_d_n3, assign11670_e11073_d_n4, assign11670_e11073_d_n5, assign11670_e11073_d_n6, assign11670_e11073_d_n7, assign11670_e11073_d_n8, assign11670_e11073_d_n9, assign11670_e11073_d_n10, assign11670_e11073_d_n11, assign11670_e11073_d_n12,) = {
    if (((locals.var_guard1124 != 0.0) && (locals.var_guard1133 == 0.0)) && (locals.var_guard1134 == 0.0)) {
        let assign11670_e11071: f64 = (locals.var_t7__blk815).exp();
        (assign11670_e11071, (assign11670_e11071 * locals.var_t7__blk815_dn3), (assign11670_e11071 * locals.var_t7__blk815_dn4), (assign11670_e11071 * locals.var_t7__blk815_dn5), (assign11670_e11071 * locals.var_t7__blk815_dn6), (assign11670_e11071 * locals.var_t7__blk815_dn7), (assign11670_e11071 * locals.var_t7__blk815_dn8), (assign11670_e11071 * locals.var_t7__blk815_dn9), (assign11670_e11071 * locals.var_t7__blk815_dn10), (assign11670_e11071 * locals.var_t7__blk815_dn11), (assign11670_e11071 * locals.var_t7__blk815_dn12),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign11670_e11073;
        locals.var_t2__blk810_dn3 = assign11670_e11073_d_n3;
        locals.var_t2__blk810_dn4 = assign11670_e11073_d_n4;
        locals.var_t2__blk810_dn5 = assign11670_e11073_d_n5;
        locals.var_t2__blk810_dn6 = assign11670_e11073_d_n6;
        locals.var_t2__blk810_dn7 = assign11670_e11073_d_n7;
        locals.var_t2__blk810_dn8 = assign11670_e11073_d_n8;
        locals.var_t2__blk810_dn9 = assign11670_e11073_d_n9;
        locals.var_t2__blk810_dn10 = assign11670_e11073_d_n10;
        locals.var_t2__blk810_dn11 = assign11670_e11073_d_n11;
        locals.var_t2__blk810_dn12 = assign11670_e11073_d_n12;

    }

    pub(super) fn stamp_transient_block_29(
        locals: &mut StampLocals,
    ) {
        let (assign11680_e11079, assign11680_e11079_d_n3, assign11680_e11079_d_n4, assign11680_e11079_d_n5, assign11680_e11079_d_n6, assign11680_e11079_d_n7, assign11680_e11079_d_n8, assign11680_e11079_d_n9, assign11680_e11079_d_n10, assign11680_e11079_d_n11, assign11680_e11079_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11680_e11077: f64 = (locals.var_pparam_b4soiahli * locals.var_t0__blk808);
        (assign11680_e11077, ((locals.var_pparam_b4soiahli_dn3 * locals.var_t0__blk808) + (locals.var_pparam_b4soiahli * locals.var_t0__blk808_dn3)), ((locals.var_pparam_b4soiahli_dn4 * locals.var_t0__blk808) + (locals.var_pparam_b4soiahli * locals.var_t0__blk808_dn4)), ((locals.var_pparam_b4soiahli_dn5 * locals.var_t0__blk808) + (locals.var_pparam_b4soiahli * locals.var_t0__blk808_dn5)), ((locals.var_pparam_b4soiahli_dn6 * locals.var_t0__blk808) + (locals.var_pparam_b4soiahli * locals.var_t0__blk808_dn6)), ((locals.var_pparam_b4soiahli_dn7 * locals.var_t0__blk808) + (locals.var_pparam_b4soiahli * locals.var_t0__blk808_dn7)), ((locals.var_pparam_b4soiahli_dn8 * locals.var_t0__blk808) + (locals.var_pparam_b4soiahli * locals.var_t0__blk808_dn8)), ((locals.var_pparam_b4soiahli_dn9 * locals.var_t0__blk808) + (locals.var_pparam_b4soiahli * locals.var_t0__blk808_dn9)), ((locals.var_pparam_b4soiahli_dn10 * locals.var_t0__blk808) + (locals.var_pparam_b4soiahli * locals.var_t0__blk808_dn10)), ((locals.var_pparam_b4soiahli_dn11 * locals.var_t0__blk808) + (locals.var_pparam_b4soiahli * locals.var_t0__blk808_dn11)), ((locals.var_pparam_b4soiahli_dn12 * locals.var_t0__blk808) + (locals.var_pparam_b4soiahli * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_ahlis, locals.var_ahlis_dn3, locals.var_ahlis_dn4, locals.var_ahlis_dn5, locals.var_ahlis_dn6, locals.var_ahlis_dn7, locals.var_ahlis_dn8, locals.var_ahlis_dn9, locals.var_ahlis_dn10, locals.var_ahlis_dn11, locals.var_ahlis_dn12,)
    }
};
        locals.var_ahlis = assign11680_e11079;
        locals.var_ahlis_dn3 = assign11680_e11079_d_n3;
        locals.var_ahlis_dn4 = assign11680_e11079_d_n4;
        locals.var_ahlis_dn5 = assign11680_e11079_d_n5;
        locals.var_ahlis_dn6 = assign11680_e11079_d_n6;
        locals.var_ahlis_dn7 = assign11680_e11079_d_n7;
        locals.var_ahlis_dn8 = assign11680_e11079_d_n8;
        locals.var_ahlis_dn9 = assign11680_e11079_d_n9;
        locals.var_ahlis_dn10 = assign11680_e11079_d_n10;
        locals.var_ahlis_dn11 = assign11680_e11079_d_n11;
        locals.var_ahlis_dn12 = assign11680_e11079_d_n12;

        let (assign11690_e11085, assign11690_e11085_d_n3, assign11690_e11085_d_n4, assign11690_e11085_d_n5, assign11690_e11085_d_n6, assign11690_e11085_d_n7, assign11690_e11085_d_n8, assign11690_e11085_d_n9, assign11690_e11085_d_n10, assign11690_e11085_d_n11, assign11690_e11085_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11690_e11083: f64 = (locals.var_pparam_b4soiisbjt * locals.var_t0__blk808);
        (assign11690_e11083, ((locals.var_pparam_b4soiisbjt_dn3 * locals.var_t0__blk808) + (locals.var_pparam_b4soiisbjt * locals.var_t0__blk808_dn3)), ((locals.var_pparam_b4soiisbjt_dn4 * locals.var_t0__blk808) + (locals.var_pparam_b4soiisbjt * locals.var_t0__blk808_dn4)), ((locals.var_pparam_b4soiisbjt_dn5 * locals.var_t0__blk808) + (locals.var_pparam_b4soiisbjt * locals.var_t0__blk808_dn5)), ((locals.var_pparam_b4soiisbjt_dn6 * locals.var_t0__blk808) + (locals.var_pparam_b4soiisbjt * locals.var_t0__blk808_dn6)), ((locals.var_pparam_b4soiisbjt_dn7 * locals.var_t0__blk808) + (locals.var_pparam_b4soiisbjt * locals.var_t0__blk808_dn7)), ((locals.var_pparam_b4soiisbjt_dn8 * locals.var_t0__blk808) + (locals.var_pparam_b4soiisbjt * locals.var_t0__blk808_dn8)), ((locals.var_pparam_b4soiisbjt_dn9 * locals.var_t0__blk808) + (locals.var_pparam_b4soiisbjt * locals.var_t0__blk808_dn9)), ((locals.var_pparam_b4soiisbjt_dn10 * locals.var_t0__blk808) + (locals.var_pparam_b4soiisbjt * locals.var_t0__blk808_dn10)), ((locals.var_pparam_b4soiisbjt_dn11 * locals.var_t0__blk808) + (locals.var_pparam_b4soiisbjt * locals.var_t0__blk808_dn11)), ((locals.var_pparam_b4soiisbjt_dn12 * locals.var_t0__blk808) + (locals.var_pparam_b4soiisbjt * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_jbjts, locals.var_jbjts_dn3, locals.var_jbjts_dn4, locals.var_jbjts_dn5, locals.var_jbjts_dn6, locals.var_jbjts_dn7, locals.var_jbjts_dn8, locals.var_jbjts_dn9, locals.var_jbjts_dn10, locals.var_jbjts_dn11, locals.var_jbjts_dn12,)
    }
};
        locals.var_jbjts = assign11690_e11085;
        locals.var_jbjts_dn3 = assign11690_e11085_d_n3;
        locals.var_jbjts_dn4 = assign11690_e11085_d_n4;
        locals.var_jbjts_dn5 = assign11690_e11085_d_n5;
        locals.var_jbjts_dn6 = assign11690_e11085_d_n6;
        locals.var_jbjts_dn7 = assign11690_e11085_d_n7;
        locals.var_jbjts_dn8 = assign11690_e11085_d_n8;
        locals.var_jbjts_dn9 = assign11690_e11085_d_n9;
        locals.var_jbjts_dn10 = assign11690_e11085_d_n10;
        locals.var_jbjts_dn11 = assign11690_e11085_d_n11;
        locals.var_jbjts_dn12 = assign11690_e11085_d_n12;

        let (assign11700_e11091, assign11700_e11091_d_n3, assign11700_e11091_d_n4, assign11700_e11091_d_n5, assign11700_e11091_d_n6, assign11700_e11091_d_n7, assign11700_e11091_d_n8, assign11700_e11091_d_n9, assign11700_e11091_d_n10, assign11700_e11091_d_n11, assign11700_e11091_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11700_e11089: f64 = (locals.var_pparam_b4soiisdif * locals.var_t1__blk809);
        (assign11700_e11089, ((locals.var_pparam_b4soiisdif_dn3 * locals.var_t1__blk809) + (locals.var_pparam_b4soiisdif * locals.var_t1__blk809_dn3)), ((locals.var_pparam_b4soiisdif_dn4 * locals.var_t1__blk809) + (locals.var_pparam_b4soiisdif * locals.var_t1__blk809_dn4)), ((locals.var_pparam_b4soiisdif_dn5 * locals.var_t1__blk809) + (locals.var_pparam_b4soiisdif * locals.var_t1__blk809_dn5)), ((locals.var_pparam_b4soiisdif_dn6 * locals.var_t1__blk809) + (locals.var_pparam_b4soiisdif * locals.var_t1__blk809_dn6)), ((locals.var_pparam_b4soiisdif_dn7 * locals.var_t1__blk809) + (locals.var_pparam_b4soiisdif * locals.var_t1__blk809_dn7)), ((locals.var_pparam_b4soiisdif_dn8 * locals.var_t1__blk809) + (locals.var_pparam_b4soiisdif * locals.var_t1__blk809_dn8)), ((locals.var_pparam_b4soiisdif_dn9 * locals.var_t1__blk809) + (locals.var_pparam_b4soiisdif * locals.var_t1__blk809_dn9)), ((locals.var_pparam_b4soiisdif_dn10 * locals.var_t1__blk809) + (locals.var_pparam_b4soiisdif * locals.var_t1__blk809_dn10)), ((locals.var_pparam_b4soiisdif_dn11 * locals.var_t1__blk809) + (locals.var_pparam_b4soiisdif * locals.var_t1__blk809_dn11)), ((locals.var_pparam_b4soiisdif_dn12 * locals.var_t1__blk809) + (locals.var_pparam_b4soiisdif * locals.var_t1__blk809_dn12)),)
    } else {
        (locals.var_jdifs, locals.var_jdifs_dn3, locals.var_jdifs_dn4, locals.var_jdifs_dn5, locals.var_jdifs_dn6, locals.var_jdifs_dn7, locals.var_jdifs_dn8, locals.var_jdifs_dn9, locals.var_jdifs_dn10, locals.var_jdifs_dn11, locals.var_jdifs_dn12,)
    }
};
        locals.var_jdifs = assign11700_e11091;
        locals.var_jdifs_dn3 = assign11700_e11091_d_n3;
        locals.var_jdifs_dn4 = assign11700_e11091_d_n4;
        locals.var_jdifs_dn5 = assign11700_e11091_d_n5;
        locals.var_jdifs_dn6 = assign11700_e11091_d_n6;
        locals.var_jdifs_dn7 = assign11700_e11091_d_n7;
        locals.var_jdifs_dn8 = assign11700_e11091_d_n8;
        locals.var_jdifs_dn9 = assign11700_e11091_d_n9;
        locals.var_jdifs_dn10 = assign11700_e11091_d_n10;
        locals.var_jdifs_dn11 = assign11700_e11091_d_n11;
        locals.var_jdifs_dn12 = assign11700_e11091_d_n12;

        let (assign11710_e11097, assign11710_e11097_d_n3, assign11710_e11097_d_n4, assign11710_e11097_d_n5, assign11710_e11097_d_n6, assign11710_e11097_d_n7, assign11710_e11097_d_n8, assign11710_e11097_d_n9, assign11710_e11097_d_n10, assign11710_e11097_d_n11, assign11710_e11097_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11710_e11095: f64 = (locals.var_pparam_b4soiisrec * locals.var_t2__blk810);
        (assign11710_e11095, ((locals.var_pparam_b4soiisrec_dn3 * locals.var_t2__blk810) + (locals.var_pparam_b4soiisrec * locals.var_t2__blk810_dn3)), ((locals.var_pparam_b4soiisrec_dn4 * locals.var_t2__blk810) + (locals.var_pparam_b4soiisrec * locals.var_t2__blk810_dn4)), ((locals.var_pparam_b4soiisrec_dn5 * locals.var_t2__blk810) + (locals.var_pparam_b4soiisrec * locals.var_t2__blk810_dn5)), ((locals.var_pparam_b4soiisrec_dn6 * locals.var_t2__blk810) + (locals.var_pparam_b4soiisrec * locals.var_t2__blk810_dn6)), ((locals.var_pparam_b4soiisrec_dn7 * locals.var_t2__blk810) + (locals.var_pparam_b4soiisrec * locals.var_t2__blk810_dn7)), ((locals.var_pparam_b4soiisrec_dn8 * locals.var_t2__blk810) + (locals.var_pparam_b4soiisrec * locals.var_t2__blk810_dn8)), ((locals.var_pparam_b4soiisrec_dn9 * locals.var_t2__blk810) + (locals.var_pparam_b4soiisrec * locals.var_t2__blk810_dn9)), ((locals.var_pparam_b4soiisrec_dn10 * locals.var_t2__blk810) + (locals.var_pparam_b4soiisrec * locals.var_t2__blk810_dn10)), ((locals.var_pparam_b4soiisrec_dn11 * locals.var_t2__blk810) + (locals.var_pparam_b4soiisrec * locals.var_t2__blk810_dn11)), ((locals.var_pparam_b4soiisrec_dn12 * locals.var_t2__blk810) + (locals.var_pparam_b4soiisrec * locals.var_t2__blk810_dn12)),)
    } else {
        (locals.var_jrecs, locals.var_jrecs_dn3, locals.var_jrecs_dn4, locals.var_jrecs_dn5, locals.var_jrecs_dn6, locals.var_jrecs_dn7, locals.var_jrecs_dn8, locals.var_jrecs_dn9, locals.var_jrecs_dn10, locals.var_jrecs_dn11, locals.var_jrecs_dn12,)
    }
};
        locals.var_jrecs = assign11710_e11097;
        locals.var_jrecs_dn3 = assign11710_e11097_d_n3;
        locals.var_jrecs_dn4 = assign11710_e11097_d_n4;
        locals.var_jrecs_dn5 = assign11710_e11097_d_n5;
        locals.var_jrecs_dn6 = assign11710_e11097_d_n6;
        locals.var_jrecs_dn7 = assign11710_e11097_d_n7;
        locals.var_jrecs_dn8 = assign11710_e11097_d_n8;
        locals.var_jrecs_dn9 = assign11710_e11097_d_n9;
        locals.var_jrecs_dn10 = assign11710_e11097_d_n10;
        locals.var_jrecs_dn11 = assign11710_e11097_d_n11;
        locals.var_jrecs_dn12 = assign11710_e11097_d_n12;

        let (assign11720_e11103, assign11720_e11103_d_n3, assign11720_e11103_d_n4, assign11720_e11103_d_n5, assign11720_e11103_d_n6, assign11720_e11103_d_n7, assign11720_e11103_d_n8, assign11720_e11103_d_n9, assign11720_e11103_d_n10, assign11720_e11103_d_n11, assign11720_e11103_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11720_e11101: f64 = (locals.var_pparam_b4soixtun * locals.var_trm1);
        (assign11720_e11101, (locals.var_pparam_b4soixtun_dn3 * locals.var_trm1), ((locals.var_pparam_b4soixtun_dn4 * locals.var_trm1) + (locals.var_pparam_b4soixtun * locals.var_trm1_dn4)), ((locals.var_pparam_b4soixtun_dn5 * locals.var_trm1) + (locals.var_pparam_b4soixtun * locals.var_trm1_dn5)), ((locals.var_pparam_b4soixtun_dn6 * locals.var_trm1) + (locals.var_pparam_b4soixtun * locals.var_trm1_dn6)), (locals.var_pparam_b4soixtun_dn7 * locals.var_trm1), (locals.var_pparam_b4soixtun_dn8 * locals.var_trm1), (locals.var_pparam_b4soixtun_dn9 * locals.var_trm1), (locals.var_pparam_b4soixtun_dn10 * locals.var_trm1), (locals.var_pparam_b4soixtun_dn11 * locals.var_trm1), (locals.var_pparam_b4soixtun_dn12 * locals.var_trm1),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign11720_e11103;
        locals.var_t7__blk815_dn3 = assign11720_e11103_d_n3;
        locals.var_t7__blk815_dn4 = assign11720_e11103_d_n4;
        locals.var_t7__blk815_dn5 = assign11720_e11103_d_n5;
        locals.var_t7__blk815_dn6 = assign11720_e11103_d_n6;
        locals.var_t7__blk815_dn7 = assign11720_e11103_d_n7;
        locals.var_t7__blk815_dn8 = assign11720_e11103_d_n8;
        locals.var_t7__blk815_dn9 = assign11720_e11103_d_n9;
        locals.var_t7__blk815_dn10 = assign11720_e11103_d_n10;
        locals.var_t7__blk815_dn11 = assign11720_e11103_d_n11;
        locals.var_t7__blk815_dn12 = assign11720_e11103_d_n12;

        let assign11730_e11106: f64 = if locals.var_t7__blk815 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1135 = assign11730_e11106;

        let (assign11740_e11118, assign11740_e11118_d_n3, assign11740_e11118_d_n4, assign11740_e11118_d_n5, assign11740_e11118_d_n6, assign11740_e11118_d_n7, assign11740_e11118_d_n8, assign11740_e11118_d_n9, assign11740_e11118_d_n10, assign11740_e11118_d_n11, assign11740_e11118_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1135 != 0.0)) {
        let assign11740_e11113: f64 = (1.0 + locals.var_t7__blk815);
        let assign11740_e11115: f64 = (assign11740_e11113 - 100.0);
        let assign11740_e11116: f64 = (2.688117142e43 * assign11740_e11115);
        (assign11740_e11116, (2.688117142e43 * locals.var_t7__blk815_dn3), (2.688117142e43 * locals.var_t7__blk815_dn4), (2.688117142e43 * locals.var_t7__blk815_dn5), (2.688117142e43 * locals.var_t7__blk815_dn6), (2.688117142e43 * locals.var_t7__blk815_dn7), (2.688117142e43 * locals.var_t7__blk815_dn8), (2.688117142e43 * locals.var_t7__blk815_dn9), (2.688117142e43 * locals.var_t7__blk815_dn10), (2.688117142e43 * locals.var_t7__blk815_dn11), (2.688117142e43 * locals.var_t7__blk815_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign11740_e11118;
        locals.var_t0__blk808_dn3 = assign11740_e11118_d_n3;
        locals.var_t0__blk808_dn4 = assign11740_e11118_d_n4;
        locals.var_t0__blk808_dn5 = assign11740_e11118_d_n5;
        locals.var_t0__blk808_dn6 = assign11740_e11118_d_n6;
        locals.var_t0__blk808_dn7 = assign11740_e11118_d_n7;
        locals.var_t0__blk808_dn8 = assign11740_e11118_d_n8;
        locals.var_t0__blk808_dn9 = assign11740_e11118_d_n9;
        locals.var_t0__blk808_dn10 = assign11740_e11118_d_n10;
        locals.var_t0__blk808_dn11 = assign11740_e11118_d_n11;
        locals.var_t0__blk808_dn12 = assign11740_e11118_d_n12;

        let assign11750_e11121: f64 = (-100.0);
        let assign11750_e11122: f64 = if locals.var_t7__blk815 < assign11750_e11121 { 1.0 } else { 0.0 };
        locals.var_guard1136 = assign11750_e11122;

        let (assign11760_e11131, assign11760_e11131_d_n3, assign11760_e11131_d_n4, assign11760_e11131_d_n5, assign11760_e11131_d_n6, assign11760_e11131_d_n7, assign11760_e11131_d_n8, assign11760_e11131_d_n9, assign11760_e11131_d_n10, assign11760_e11131_d_n11, assign11760_e11131_d_n12,) = {
    if (((locals.var_guard1124 != 0.0) && (locals.var_guard1135 == 0.0)) && (locals.var_guard1136 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign11760_e11131;
        locals.var_t0__blk808_dn3 = assign11760_e11131_d_n3;
        locals.var_t0__blk808_dn4 = assign11760_e11131_d_n4;
        locals.var_t0__blk808_dn5 = assign11760_e11131_d_n5;
        locals.var_t0__blk808_dn6 = assign11760_e11131_d_n6;
        locals.var_t0__blk808_dn7 = assign11760_e11131_d_n7;
        locals.var_t0__blk808_dn8 = assign11760_e11131_d_n8;
        locals.var_t0__blk808_dn9 = assign11760_e11131_d_n9;
        locals.var_t0__blk808_dn10 = assign11760_e11131_d_n10;
        locals.var_t0__blk808_dn11 = assign11760_e11131_d_n11;
        locals.var_t0__blk808_dn12 = assign11760_e11131_d_n12;

        let (assign11770_e11142, assign11770_e11142_d_n3, assign11770_e11142_d_n4, assign11770_e11142_d_n5, assign11770_e11142_d_n6, assign11770_e11142_d_n7, assign11770_e11142_d_n8, assign11770_e11142_d_n9, assign11770_e11142_d_n10, assign11770_e11142_d_n11, assign11770_e11142_d_n12,) = {
    if (((locals.var_guard1124 != 0.0) && (locals.var_guard1135 == 0.0)) && (locals.var_guard1136 == 0.0)) {
        let assign11770_e11140: f64 = (locals.var_t7__blk815).exp();
        (assign11770_e11140, (assign11770_e11140 * locals.var_t7__blk815_dn3), (assign11770_e11140 * locals.var_t7__blk815_dn4), (assign11770_e11140 * locals.var_t7__blk815_dn5), (assign11770_e11140 * locals.var_t7__blk815_dn6), (assign11770_e11140 * locals.var_t7__blk815_dn7), (assign11770_e11140 * locals.var_t7__blk815_dn8), (assign11770_e11140 * locals.var_t7__blk815_dn9), (assign11770_e11140 * locals.var_t7__blk815_dn10), (assign11770_e11140 * locals.var_t7__blk815_dn11), (assign11770_e11140 * locals.var_t7__blk815_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign11770_e11142;
        locals.var_t0__blk808_dn3 = assign11770_e11142_d_n3;
        locals.var_t0__blk808_dn4 = assign11770_e11142_d_n4;
        locals.var_t0__blk808_dn5 = assign11770_e11142_d_n5;
        locals.var_t0__blk808_dn6 = assign11770_e11142_d_n6;
        locals.var_t0__blk808_dn7 = assign11770_e11142_d_n7;
        locals.var_t0__blk808_dn8 = assign11770_e11142_d_n8;
        locals.var_t0__blk808_dn9 = assign11770_e11142_d_n9;
        locals.var_t0__blk808_dn10 = assign11770_e11142_d_n10;
        locals.var_t0__blk808_dn11 = assign11770_e11142_d_n11;
        locals.var_t0__blk808_dn12 = assign11770_e11142_d_n12;

        let (assign11780_e11148, assign11780_e11148_d_n3, assign11780_e11148_d_n4, assign11780_e11148_d_n5, assign11780_e11148_d_n6, assign11780_e11148_d_n7, assign11780_e11148_d_n8, assign11780_e11148_d_n9, assign11780_e11148_d_n10, assign11780_e11148_d_n11, assign11780_e11148_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11780_e11146: f64 = (locals.var_pparam_b4soiistun * locals.var_t0__blk808);
        (assign11780_e11146, ((locals.var_pparam_b4soiistun_dn3 * locals.var_t0__blk808) + (locals.var_pparam_b4soiistun * locals.var_t0__blk808_dn3)), ((locals.var_pparam_b4soiistun_dn4 * locals.var_t0__blk808) + (locals.var_pparam_b4soiistun * locals.var_t0__blk808_dn4)), ((locals.var_pparam_b4soiistun_dn5 * locals.var_t0__blk808) + (locals.var_pparam_b4soiistun * locals.var_t0__blk808_dn5)), ((locals.var_pparam_b4soiistun_dn6 * locals.var_t0__blk808) + (locals.var_pparam_b4soiistun * locals.var_t0__blk808_dn6)), ((locals.var_pparam_b4soiistun_dn7 * locals.var_t0__blk808) + (locals.var_pparam_b4soiistun * locals.var_t0__blk808_dn7)), ((locals.var_pparam_b4soiistun_dn8 * locals.var_t0__blk808) + (locals.var_pparam_b4soiistun * locals.var_t0__blk808_dn8)), ((locals.var_pparam_b4soiistun_dn9 * locals.var_t0__blk808) + (locals.var_pparam_b4soiistun * locals.var_t0__blk808_dn9)), ((locals.var_pparam_b4soiistun_dn10 * locals.var_t0__blk808) + (locals.var_pparam_b4soiistun * locals.var_t0__blk808_dn10)), ((locals.var_pparam_b4soiistun_dn11 * locals.var_t0__blk808) + (locals.var_pparam_b4soiistun * locals.var_t0__blk808_dn11)), ((locals.var_pparam_b4soiistun_dn12 * locals.var_t0__blk808) + (locals.var_pparam_b4soiistun * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_jtuns, locals.var_jtuns_dn3, locals.var_jtuns_dn4, locals.var_jtuns_dn5, locals.var_jtuns_dn6, locals.var_jtuns_dn7, locals.var_jtuns_dn8, locals.var_jtuns_dn9, locals.var_jtuns_dn10, locals.var_jtuns_dn11, locals.var_jtuns_dn12,)
    }
};
        locals.var_jtuns = assign11780_e11148;
        locals.var_jtuns_dn3 = assign11780_e11148_d_n3;
        locals.var_jtuns_dn4 = assign11780_e11148_d_n4;
        locals.var_jtuns_dn5 = assign11780_e11148_d_n5;
        locals.var_jtuns_dn6 = assign11780_e11148_d_n6;
        locals.var_jtuns_dn7 = assign11780_e11148_d_n7;
        locals.var_jtuns_dn8 = assign11780_e11148_d_n8;
        locals.var_jtuns_dn9 = assign11780_e11148_d_n9;
        locals.var_jtuns_dn10 = assign11780_e11148_d_n10;
        locals.var_jtuns_dn11 = assign11780_e11148_d_n11;
        locals.var_jtuns_dn12 = assign11780_e11148_d_n12;

        let (assign11790_e11156, assign11790_e11156_d_n3, assign11790_e11156_d_n4, assign11790_e11156_d_n5, assign11790_e11156_d_n6, assign11790_e11156_d_n7, assign11790_e11156_d_n8, assign11790_e11156_d_n9, assign11790_e11156_d_n10, assign11790_e11156_d_n11, assign11790_e11156_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11790_e11152: f64 = (locals.var_pparam_b4soixbjt * locals.var_t4__blk812);
        let assign11790_e11154: f64 = (assign11790_e11152 / locals.var_pparam_b4soindioded);
        (assign11790_e11154, (((((locals.var_pparam_b4soixbjt_dn3 * locals.var_t4__blk812) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk812_dn3)) * locals.var_pparam_b4soindioded) - (assign11790_e11152 * locals.var_pparam_b4soindioded_dn3)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixbjt_dn4 * locals.var_t4__blk812) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk812_dn4)) * locals.var_pparam_b4soindioded) - (assign11790_e11152 * locals.var_pparam_b4soindioded_dn4)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixbjt_dn5 * locals.var_t4__blk812) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk812_dn5)) * locals.var_pparam_b4soindioded) - (assign11790_e11152 * locals.var_pparam_b4soindioded_dn5)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixbjt_dn6 * locals.var_t4__blk812) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk812_dn6)) * locals.var_pparam_b4soindioded) - (assign11790_e11152 * locals.var_pparam_b4soindioded_dn6)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixbjt_dn7 * locals.var_t4__blk812) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk812_dn7)) * locals.var_pparam_b4soindioded) - (assign11790_e11152 * locals.var_pparam_b4soindioded_dn7)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixbjt_dn8 * locals.var_t4__blk812) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk812_dn8)) * locals.var_pparam_b4soindioded) - (assign11790_e11152 * locals.var_pparam_b4soindioded_dn8)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixbjt_dn9 * locals.var_t4__blk812) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk812_dn9)) * locals.var_pparam_b4soindioded) - (assign11790_e11152 * locals.var_pparam_b4soindioded_dn9)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixbjt_dn10 * locals.var_t4__blk812) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk812_dn10)) * locals.var_pparam_b4soindioded) - (assign11790_e11152 * locals.var_pparam_b4soindioded_dn10)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixbjt_dn11 * locals.var_t4__blk812) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk812_dn11)) * locals.var_pparam_b4soindioded) - (assign11790_e11152 * locals.var_pparam_b4soindioded_dn11)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixbjt_dn12 * locals.var_t4__blk812) + (locals.var_pparam_b4soixbjt * locals.var_t4__blk812_dn12)) * locals.var_pparam_b4soindioded) - (assign11790_e11152 * locals.var_pparam_b4soindioded_dn12)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign11790_e11156;
        locals.var_t7__blk815_dn3 = assign11790_e11156_d_n3;
        locals.var_t7__blk815_dn4 = assign11790_e11156_d_n4;
        locals.var_t7__blk815_dn5 = assign11790_e11156_d_n5;
        locals.var_t7__blk815_dn6 = assign11790_e11156_d_n6;
        locals.var_t7__blk815_dn7 = assign11790_e11156_d_n7;
        locals.var_t7__blk815_dn8 = assign11790_e11156_d_n8;
        locals.var_t7__blk815_dn9 = assign11790_e11156_d_n9;
        locals.var_t7__blk815_dn10 = assign11790_e11156_d_n10;
        locals.var_t7__blk815_dn11 = assign11790_e11156_d_n11;
        locals.var_t7__blk815_dn12 = assign11790_e11156_d_n12;

        let assign11800_e11159: f64 = if locals.var_t7__blk815 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1137 = assign11800_e11159;

        let (assign11810_e11171, assign11810_e11171_d_n3, assign11810_e11171_d_n4, assign11810_e11171_d_n5, assign11810_e11171_d_n6, assign11810_e11171_d_n7, assign11810_e11171_d_n8, assign11810_e11171_d_n9, assign11810_e11171_d_n10, assign11810_e11171_d_n11, assign11810_e11171_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1137 != 0.0)) {
        let assign11810_e11166: f64 = (1.0 + locals.var_t7__blk815);
        let assign11810_e11168: f64 = (assign11810_e11166 - 100.0);
        let assign11810_e11169: f64 = (2.688117142e43 * assign11810_e11168);
        (assign11810_e11169, (2.688117142e43 * locals.var_t7__blk815_dn3), (2.688117142e43 * locals.var_t7__blk815_dn4), (2.688117142e43 * locals.var_t7__blk815_dn5), (2.688117142e43 * locals.var_t7__blk815_dn6), (2.688117142e43 * locals.var_t7__blk815_dn7), (2.688117142e43 * locals.var_t7__blk815_dn8), (2.688117142e43 * locals.var_t7__blk815_dn9), (2.688117142e43 * locals.var_t7__blk815_dn10), (2.688117142e43 * locals.var_t7__blk815_dn11), (2.688117142e43 * locals.var_t7__blk815_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign11810_e11171;
        locals.var_t0__blk808_dn3 = assign11810_e11171_d_n3;
        locals.var_t0__blk808_dn4 = assign11810_e11171_d_n4;
        locals.var_t0__blk808_dn5 = assign11810_e11171_d_n5;
        locals.var_t0__blk808_dn6 = assign11810_e11171_d_n6;
        locals.var_t0__blk808_dn7 = assign11810_e11171_d_n7;
        locals.var_t0__blk808_dn8 = assign11810_e11171_d_n8;
        locals.var_t0__blk808_dn9 = assign11810_e11171_d_n9;
        locals.var_t0__blk808_dn10 = assign11810_e11171_d_n10;
        locals.var_t0__blk808_dn11 = assign11810_e11171_d_n11;
        locals.var_t0__blk808_dn12 = assign11810_e11171_d_n12;

        let assign11820_e11174: f64 = (-100.0);
        let assign11820_e11175: f64 = if locals.var_t7__blk815 < assign11820_e11174 { 1.0 } else { 0.0 };
        locals.var_guard1138 = assign11820_e11175;

        let (assign11830_e11184, assign11830_e11184_d_n3, assign11830_e11184_d_n4, assign11830_e11184_d_n5, assign11830_e11184_d_n6, assign11830_e11184_d_n7, assign11830_e11184_d_n8, assign11830_e11184_d_n9, assign11830_e11184_d_n10, assign11830_e11184_d_n11, assign11830_e11184_d_n12,) = {
    if (((locals.var_guard1124 != 0.0) && (locals.var_guard1137 == 0.0)) && (locals.var_guard1138 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign11830_e11184;
        locals.var_t0__blk808_dn3 = assign11830_e11184_d_n3;
        locals.var_t0__blk808_dn4 = assign11830_e11184_d_n4;
        locals.var_t0__blk808_dn5 = assign11830_e11184_d_n5;
        locals.var_t0__blk808_dn6 = assign11830_e11184_d_n6;
        locals.var_t0__blk808_dn7 = assign11830_e11184_d_n7;
        locals.var_t0__blk808_dn8 = assign11830_e11184_d_n8;
        locals.var_t0__blk808_dn9 = assign11830_e11184_d_n9;
        locals.var_t0__blk808_dn10 = assign11830_e11184_d_n10;
        locals.var_t0__blk808_dn11 = assign11830_e11184_d_n11;
        locals.var_t0__blk808_dn12 = assign11830_e11184_d_n12;

        let (assign11840_e11195, assign11840_e11195_d_n3, assign11840_e11195_d_n4, assign11840_e11195_d_n5, assign11840_e11195_d_n6, assign11840_e11195_d_n7, assign11840_e11195_d_n8, assign11840_e11195_d_n9, assign11840_e11195_d_n10, assign11840_e11195_d_n11, assign11840_e11195_d_n12,) = {
    if (((locals.var_guard1124 != 0.0) && (locals.var_guard1137 == 0.0)) && (locals.var_guard1138 == 0.0)) {
        let assign11840_e11193: f64 = (locals.var_t7__blk815).exp();
        (assign11840_e11193, (assign11840_e11193 * locals.var_t7__blk815_dn3), (assign11840_e11193 * locals.var_t7__blk815_dn4), (assign11840_e11193 * locals.var_t7__blk815_dn5), (assign11840_e11193 * locals.var_t7__blk815_dn6), (assign11840_e11193 * locals.var_t7__blk815_dn7), (assign11840_e11193 * locals.var_t7__blk815_dn8), (assign11840_e11193 * locals.var_t7__blk815_dn9), (assign11840_e11193 * locals.var_t7__blk815_dn10), (assign11840_e11193 * locals.var_t7__blk815_dn11), (assign11840_e11193 * locals.var_t7__blk815_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign11840_e11195;
        locals.var_t0__blk808_dn3 = assign11840_e11195_d_n3;
        locals.var_t0__blk808_dn4 = assign11840_e11195_d_n4;
        locals.var_t0__blk808_dn5 = assign11840_e11195_d_n5;
        locals.var_t0__blk808_dn6 = assign11840_e11195_d_n6;
        locals.var_t0__blk808_dn7 = assign11840_e11195_d_n7;
        locals.var_t0__blk808_dn8 = assign11840_e11195_d_n8;
        locals.var_t0__blk808_dn9 = assign11840_e11195_d_n9;
        locals.var_t0__blk808_dn10 = assign11840_e11195_d_n10;
        locals.var_t0__blk808_dn11 = assign11840_e11195_d_n11;
        locals.var_t0__blk808_dn12 = assign11840_e11195_d_n12;

        let assign11850_e11198: f64 = if locals.var_pparam_b4soixbjt == locals.var_pparam_b4soixdifd { 1.0 } else { 0.0 };
        locals.var_guard1139 = assign11850_e11198;

        let (assign11860_e11204, assign11860_e11204_d_n3, assign11860_e11204_d_n4, assign11860_e11204_d_n5, assign11860_e11204_d_n6, assign11860_e11204_d_n7, assign11860_e11204_d_n8, assign11860_e11204_d_n9, assign11860_e11204_d_n10, assign11860_e11204_d_n11, assign11860_e11204_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1139 != 0.0)) {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign11860_e11204;
        locals.var_t1__blk809_dn3 = assign11860_e11204_d_n3;
        locals.var_t1__blk809_dn4 = assign11860_e11204_d_n4;
        locals.var_t1__blk809_dn5 = assign11860_e11204_d_n5;
        locals.var_t1__blk809_dn6 = assign11860_e11204_d_n6;
        locals.var_t1__blk809_dn7 = assign11860_e11204_d_n7;
        locals.var_t1__blk809_dn8 = assign11860_e11204_d_n8;
        locals.var_t1__blk809_dn9 = assign11860_e11204_d_n9;
        locals.var_t1__blk809_dn10 = assign11860_e11204_d_n10;
        locals.var_t1__blk809_dn11 = assign11860_e11204_d_n11;
        locals.var_t1__blk809_dn12 = assign11860_e11204_d_n12;

        let (assign11870_e11215, assign11870_e11215_d_n3, assign11870_e11215_d_n4, assign11870_e11215_d_n5, assign11870_e11215_d_n6, assign11870_e11215_d_n7, assign11870_e11215_d_n8, assign11870_e11215_d_n9, assign11870_e11215_d_n10, assign11870_e11215_d_n11, assign11870_e11215_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1139 == 0.0)) {
        let assign11870_e11211: f64 = (locals.var_pparam_b4soixdifd * locals.var_t4__blk812);
        let assign11870_e11213: f64 = (assign11870_e11211 / locals.var_pparam_b4soindioded);
        (assign11870_e11213, (((((locals.var_pparam_b4soixdifd_dn3 * locals.var_t4__blk812) + (locals.var_pparam_b4soixdifd * locals.var_t4__blk812_dn3)) * locals.var_pparam_b4soindioded) - (assign11870_e11211 * locals.var_pparam_b4soindioded_dn3)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixdifd_dn4 * locals.var_t4__blk812) + (locals.var_pparam_b4soixdifd * locals.var_t4__blk812_dn4)) * locals.var_pparam_b4soindioded) - (assign11870_e11211 * locals.var_pparam_b4soindioded_dn4)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixdifd_dn5 * locals.var_t4__blk812) + (locals.var_pparam_b4soixdifd * locals.var_t4__blk812_dn5)) * locals.var_pparam_b4soindioded) - (assign11870_e11211 * locals.var_pparam_b4soindioded_dn5)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixdifd_dn6 * locals.var_t4__blk812) + (locals.var_pparam_b4soixdifd * locals.var_t4__blk812_dn6)) * locals.var_pparam_b4soindioded) - (assign11870_e11211 * locals.var_pparam_b4soindioded_dn6)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixdifd_dn7 * locals.var_t4__blk812) + (locals.var_pparam_b4soixdifd * locals.var_t4__blk812_dn7)) * locals.var_pparam_b4soindioded) - (assign11870_e11211 * locals.var_pparam_b4soindioded_dn7)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixdifd_dn8 * locals.var_t4__blk812) + (locals.var_pparam_b4soixdifd * locals.var_t4__blk812_dn8)) * locals.var_pparam_b4soindioded) - (assign11870_e11211 * locals.var_pparam_b4soindioded_dn8)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixdifd_dn9 * locals.var_t4__blk812) + (locals.var_pparam_b4soixdifd * locals.var_t4__blk812_dn9)) * locals.var_pparam_b4soindioded) - (assign11870_e11211 * locals.var_pparam_b4soindioded_dn9)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixdifd_dn10 * locals.var_t4__blk812) + (locals.var_pparam_b4soixdifd * locals.var_t4__blk812_dn10)) * locals.var_pparam_b4soindioded) - (assign11870_e11211 * locals.var_pparam_b4soindioded_dn10)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixdifd_dn11 * locals.var_t4__blk812) + (locals.var_pparam_b4soixdifd * locals.var_t4__blk812_dn11)) * locals.var_pparam_b4soindioded) - (assign11870_e11211 * locals.var_pparam_b4soindioded_dn11)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)), (((((locals.var_pparam_b4soixdifd_dn12 * locals.var_t4__blk812) + (locals.var_pparam_b4soixdifd * locals.var_t4__blk812_dn12)) * locals.var_pparam_b4soindioded) - (assign11870_e11211 * locals.var_pparam_b4soindioded_dn12)) / (locals.var_pparam_b4soindioded * locals.var_pparam_b4soindioded)),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign11870_e11215;
        locals.var_t7__blk815_dn3 = assign11870_e11215_d_n3;
        locals.var_t7__blk815_dn4 = assign11870_e11215_d_n4;
        locals.var_t7__blk815_dn5 = assign11870_e11215_d_n5;
        locals.var_t7__blk815_dn6 = assign11870_e11215_d_n6;
        locals.var_t7__blk815_dn7 = assign11870_e11215_d_n7;
        locals.var_t7__blk815_dn8 = assign11870_e11215_d_n8;
        locals.var_t7__blk815_dn9 = assign11870_e11215_d_n9;
        locals.var_t7__blk815_dn10 = assign11870_e11215_d_n10;
        locals.var_t7__blk815_dn11 = assign11870_e11215_d_n11;
        locals.var_t7__blk815_dn12 = assign11870_e11215_d_n12;

        let assign11880_e11218: f64 = if locals.var_t7__blk815 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1140 = assign11880_e11218;

        let (assign11890_e11233, assign11890_e11233_d_n3, assign11890_e11233_d_n4, assign11890_e11233_d_n5, assign11890_e11233_d_n6, assign11890_e11233_d_n7, assign11890_e11233_d_n8, assign11890_e11233_d_n9, assign11890_e11233_d_n10, assign11890_e11233_d_n11, assign11890_e11233_d_n12,) = {
    if (((locals.var_guard1124 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1140 != 0.0)) {
        let assign11890_e11228: f64 = (1.0 + locals.var_t7__blk815);
        let assign11890_e11230: f64 = (assign11890_e11228 - 100.0);
        let assign11890_e11231: f64 = (2.688117142e43 * assign11890_e11230);
        (assign11890_e11231, (2.688117142e43 * locals.var_t7__blk815_dn3), (2.688117142e43 * locals.var_t7__blk815_dn4), (2.688117142e43 * locals.var_t7__blk815_dn5), (2.688117142e43 * locals.var_t7__blk815_dn6), (2.688117142e43 * locals.var_t7__blk815_dn7), (2.688117142e43 * locals.var_t7__blk815_dn8), (2.688117142e43 * locals.var_t7__blk815_dn9), (2.688117142e43 * locals.var_t7__blk815_dn10), (2.688117142e43 * locals.var_t7__blk815_dn11), (2.688117142e43 * locals.var_t7__blk815_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign11890_e11233;
        locals.var_t1__blk809_dn3 = assign11890_e11233_d_n3;
        locals.var_t1__blk809_dn4 = assign11890_e11233_d_n4;
        locals.var_t1__blk809_dn5 = assign11890_e11233_d_n5;
        locals.var_t1__blk809_dn6 = assign11890_e11233_d_n6;
        locals.var_t1__blk809_dn7 = assign11890_e11233_d_n7;
        locals.var_t1__blk809_dn8 = assign11890_e11233_d_n8;
        locals.var_t1__blk809_dn9 = assign11890_e11233_d_n9;
        locals.var_t1__blk809_dn10 = assign11890_e11233_d_n10;
        locals.var_t1__blk809_dn11 = assign11890_e11233_d_n11;
        locals.var_t1__blk809_dn12 = assign11890_e11233_d_n12;

        let assign11900_e11236: f64 = (-100.0);
        let assign11900_e11237: f64 = if locals.var_t7__blk815 < assign11900_e11236 { 1.0 } else { 0.0 };
        locals.var_guard1141 = assign11900_e11237;

        let (assign11910_e11249, assign11910_e11249_d_n3, assign11910_e11249_d_n4, assign11910_e11249_d_n5, assign11910_e11249_d_n6, assign11910_e11249_d_n7, assign11910_e11249_d_n8, assign11910_e11249_d_n9, assign11910_e11249_d_n10, assign11910_e11249_d_n11, assign11910_e11249_d_n12,) = {
    if ((((locals.var_guard1124 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1140 == 0.0)) && (locals.var_guard1141 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign11910_e11249;
        locals.var_t1__blk809_dn3 = assign11910_e11249_d_n3;
        locals.var_t1__blk809_dn4 = assign11910_e11249_d_n4;
        locals.var_t1__blk809_dn5 = assign11910_e11249_d_n5;
        locals.var_t1__blk809_dn6 = assign11910_e11249_d_n6;
        locals.var_t1__blk809_dn7 = assign11910_e11249_d_n7;
        locals.var_t1__blk809_dn8 = assign11910_e11249_d_n8;
        locals.var_t1__blk809_dn9 = assign11910_e11249_d_n9;
        locals.var_t1__blk809_dn10 = assign11910_e11249_d_n10;
        locals.var_t1__blk809_dn11 = assign11910_e11249_d_n11;
        locals.var_t1__blk809_dn12 = assign11910_e11249_d_n12;

        let (assign11920_e11263, assign11920_e11263_d_n3, assign11920_e11263_d_n4, assign11920_e11263_d_n5, assign11920_e11263_d_n6, assign11920_e11263_d_n7, assign11920_e11263_d_n8, assign11920_e11263_d_n9, assign11920_e11263_d_n10, assign11920_e11263_d_n11, assign11920_e11263_d_n12,) = {
    if ((((locals.var_guard1124 != 0.0) && (locals.var_guard1139 == 0.0)) && (locals.var_guard1140 == 0.0)) && (locals.var_guard1141 == 0.0)) {
        let assign11920_e11261: f64 = (locals.var_t7__blk815).exp();
        (assign11920_e11261, (assign11920_e11261 * locals.var_t7__blk815_dn3), (assign11920_e11261 * locals.var_t7__blk815_dn4), (assign11920_e11261 * locals.var_t7__blk815_dn5), (assign11920_e11261 * locals.var_t7__blk815_dn6), (assign11920_e11261 * locals.var_t7__blk815_dn7), (assign11920_e11261 * locals.var_t7__blk815_dn8), (assign11920_e11261 * locals.var_t7__blk815_dn9), (assign11920_e11261 * locals.var_t7__blk815_dn10), (assign11920_e11261 * locals.var_t7__blk815_dn11), (assign11920_e11261 * locals.var_t7__blk815_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign11920_e11263;
        locals.var_t1__blk809_dn3 = assign11920_e11263_d_n3;
        locals.var_t1__blk809_dn4 = assign11920_e11263_d_n4;
        locals.var_t1__blk809_dn5 = assign11920_e11263_d_n5;
        locals.var_t1__blk809_dn6 = assign11920_e11263_d_n6;
        locals.var_t1__blk809_dn7 = assign11920_e11263_d_n7;
        locals.var_t1__blk809_dn8 = assign11920_e11263_d_n8;
        locals.var_t1__blk809_dn9 = assign11920_e11263_d_n9;
        locals.var_t1__blk809_dn10 = assign11920_e11263_d_n10;
        locals.var_t1__blk809_dn11 = assign11920_e11263_d_n11;
        locals.var_t1__blk809_dn12 = assign11920_e11263_d_n12;

        let (assign11930_e11271, assign11930_e11271_d_n3, assign11930_e11271_d_n4, assign11930_e11271_d_n5, assign11930_e11271_d_n6, assign11930_e11271_d_n7, assign11930_e11271_d_n8, assign11930_e11271_d_n9, assign11930_e11271_d_n10, assign11930_e11271_d_n11, assign11930_e11271_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11930_e11267: f64 = (locals.var_pparam_b4soixrecd * locals.var_t4__blk812);
        let assign11930_e11269: f64 = (assign11930_e11267 / locals.var_pparam_b4soinrecf0d);
        (assign11930_e11269, (((((locals.var_pparam_b4soixrecd_dn3 * locals.var_t4__blk812) + (locals.var_pparam_b4soixrecd * locals.var_t4__blk812_dn3)) * locals.var_pparam_b4soinrecf0d) - (assign11930_e11267 * locals.var_pparam_b4soinrecf0d_dn3)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d)), (((((locals.var_pparam_b4soixrecd_dn4 * locals.var_t4__blk812) + (locals.var_pparam_b4soixrecd * locals.var_t4__blk812_dn4)) * locals.var_pparam_b4soinrecf0d) - (assign11930_e11267 * locals.var_pparam_b4soinrecf0d_dn4)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d)), (((((locals.var_pparam_b4soixrecd_dn5 * locals.var_t4__blk812) + (locals.var_pparam_b4soixrecd * locals.var_t4__blk812_dn5)) * locals.var_pparam_b4soinrecf0d) - (assign11930_e11267 * locals.var_pparam_b4soinrecf0d_dn5)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d)), (((((locals.var_pparam_b4soixrecd_dn6 * locals.var_t4__blk812) + (locals.var_pparam_b4soixrecd * locals.var_t4__blk812_dn6)) * locals.var_pparam_b4soinrecf0d) - (assign11930_e11267 * locals.var_pparam_b4soinrecf0d_dn6)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d)), (((((locals.var_pparam_b4soixrecd_dn7 * locals.var_t4__blk812) + (locals.var_pparam_b4soixrecd * locals.var_t4__blk812_dn7)) * locals.var_pparam_b4soinrecf0d) - (assign11930_e11267 * locals.var_pparam_b4soinrecf0d_dn7)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d)), (((((locals.var_pparam_b4soixrecd_dn8 * locals.var_t4__blk812) + (locals.var_pparam_b4soixrecd * locals.var_t4__blk812_dn8)) * locals.var_pparam_b4soinrecf0d) - (assign11930_e11267 * locals.var_pparam_b4soinrecf0d_dn8)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d)), (((((locals.var_pparam_b4soixrecd_dn9 * locals.var_t4__blk812) + (locals.var_pparam_b4soixrecd * locals.var_t4__blk812_dn9)) * locals.var_pparam_b4soinrecf0d) - (assign11930_e11267 * locals.var_pparam_b4soinrecf0d_dn9)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d)), (((((locals.var_pparam_b4soixrecd_dn10 * locals.var_t4__blk812) + (locals.var_pparam_b4soixrecd * locals.var_t4__blk812_dn10)) * locals.var_pparam_b4soinrecf0d) - (assign11930_e11267 * locals.var_pparam_b4soinrecf0d_dn10)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d)), (((((locals.var_pparam_b4soixrecd_dn11 * locals.var_t4__blk812) + (locals.var_pparam_b4soixrecd * locals.var_t4__blk812_dn11)) * locals.var_pparam_b4soinrecf0d) - (assign11930_e11267 * locals.var_pparam_b4soinrecf0d_dn11)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d)), (((((locals.var_pparam_b4soixrecd_dn12 * locals.var_t4__blk812) + (locals.var_pparam_b4soixrecd * locals.var_t4__blk812_dn12)) * locals.var_pparam_b4soinrecf0d) - (assign11930_e11267 * locals.var_pparam_b4soinrecf0d_dn12)) / (locals.var_pparam_b4soinrecf0d * locals.var_pparam_b4soinrecf0d)),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign11930_e11271;
        locals.var_t7__blk815_dn3 = assign11930_e11271_d_n3;
        locals.var_t7__blk815_dn4 = assign11930_e11271_d_n4;
        locals.var_t7__blk815_dn5 = assign11930_e11271_d_n5;
        locals.var_t7__blk815_dn6 = assign11930_e11271_d_n6;
        locals.var_t7__blk815_dn7 = assign11930_e11271_d_n7;
        locals.var_t7__blk815_dn8 = assign11930_e11271_d_n8;
        locals.var_t7__blk815_dn9 = assign11930_e11271_d_n9;
        locals.var_t7__blk815_dn10 = assign11930_e11271_d_n10;
        locals.var_t7__blk815_dn11 = assign11930_e11271_d_n11;
        locals.var_t7__blk815_dn12 = assign11930_e11271_d_n12;

        let assign11940_e11274: f64 = if locals.var_t7__blk815 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1142 = assign11940_e11274;

        let (assign11950_e11286, assign11950_e11286_d_n3, assign11950_e11286_d_n4, assign11950_e11286_d_n5, assign11950_e11286_d_n6, assign11950_e11286_d_n7, assign11950_e11286_d_n8, assign11950_e11286_d_n9, assign11950_e11286_d_n10, assign11950_e11286_d_n11, assign11950_e11286_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1142 != 0.0)) {
        let assign11950_e11281: f64 = (1.0 + locals.var_t7__blk815);
        let assign11950_e11283: f64 = (assign11950_e11281 - 100.0);
        let assign11950_e11284: f64 = (2.688117142e43 * assign11950_e11283);
        (assign11950_e11284, (2.688117142e43 * locals.var_t7__blk815_dn3), (2.688117142e43 * locals.var_t7__blk815_dn4), (2.688117142e43 * locals.var_t7__blk815_dn5), (2.688117142e43 * locals.var_t7__blk815_dn6), (2.688117142e43 * locals.var_t7__blk815_dn7), (2.688117142e43 * locals.var_t7__blk815_dn8), (2.688117142e43 * locals.var_t7__blk815_dn9), (2.688117142e43 * locals.var_t7__blk815_dn10), (2.688117142e43 * locals.var_t7__blk815_dn11), (2.688117142e43 * locals.var_t7__blk815_dn12),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign11950_e11286;
        locals.var_t2__blk810_dn3 = assign11950_e11286_d_n3;
        locals.var_t2__blk810_dn4 = assign11950_e11286_d_n4;
        locals.var_t2__blk810_dn5 = assign11950_e11286_d_n5;
        locals.var_t2__blk810_dn6 = assign11950_e11286_d_n6;
        locals.var_t2__blk810_dn7 = assign11950_e11286_d_n7;
        locals.var_t2__blk810_dn8 = assign11950_e11286_d_n8;
        locals.var_t2__blk810_dn9 = assign11950_e11286_d_n9;
        locals.var_t2__blk810_dn10 = assign11950_e11286_d_n10;
        locals.var_t2__blk810_dn11 = assign11950_e11286_d_n11;
        locals.var_t2__blk810_dn12 = assign11950_e11286_d_n12;

        let assign11960_e11289: f64 = (-100.0);
        let assign11960_e11290: f64 = if locals.var_t7__blk815 < assign11960_e11289 { 1.0 } else { 0.0 };
        locals.var_guard1143 = assign11960_e11290;

        let (assign11970_e11299, assign11970_e11299_d_n3, assign11970_e11299_d_n4, assign11970_e11299_d_n5, assign11970_e11299_d_n6, assign11970_e11299_d_n7, assign11970_e11299_d_n8, assign11970_e11299_d_n9, assign11970_e11299_d_n10, assign11970_e11299_d_n11, assign11970_e11299_d_n12,) = {
    if (((locals.var_guard1124 != 0.0) && (locals.var_guard1142 == 0.0)) && (locals.var_guard1143 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign11970_e11299;
        locals.var_t2__blk810_dn3 = assign11970_e11299_d_n3;
        locals.var_t2__blk810_dn4 = assign11970_e11299_d_n4;
        locals.var_t2__blk810_dn5 = assign11970_e11299_d_n5;
        locals.var_t2__blk810_dn6 = assign11970_e11299_d_n6;
        locals.var_t2__blk810_dn7 = assign11970_e11299_d_n7;
        locals.var_t2__blk810_dn8 = assign11970_e11299_d_n8;
        locals.var_t2__blk810_dn9 = assign11970_e11299_d_n9;
        locals.var_t2__blk810_dn10 = assign11970_e11299_d_n10;
        locals.var_t2__blk810_dn11 = assign11970_e11299_d_n11;
        locals.var_t2__blk810_dn12 = assign11970_e11299_d_n12;

        let (assign11980_e11310, assign11980_e11310_d_n3, assign11980_e11310_d_n4, assign11980_e11310_d_n5, assign11980_e11310_d_n6, assign11980_e11310_d_n7, assign11980_e11310_d_n8, assign11980_e11310_d_n9, assign11980_e11310_d_n10, assign11980_e11310_d_n11, assign11980_e11310_d_n12,) = {
    if (((locals.var_guard1124 != 0.0) && (locals.var_guard1142 == 0.0)) && (locals.var_guard1143 == 0.0)) {
        let assign11980_e11308: f64 = (locals.var_t7__blk815).exp();
        (assign11980_e11308, (assign11980_e11308 * locals.var_t7__blk815_dn3), (assign11980_e11308 * locals.var_t7__blk815_dn4), (assign11980_e11308 * locals.var_t7__blk815_dn5), (assign11980_e11308 * locals.var_t7__blk815_dn6), (assign11980_e11308 * locals.var_t7__blk815_dn7), (assign11980_e11308 * locals.var_t7__blk815_dn8), (assign11980_e11308 * locals.var_t7__blk815_dn9), (assign11980_e11308 * locals.var_t7__blk815_dn10), (assign11980_e11308 * locals.var_t7__blk815_dn11), (assign11980_e11308 * locals.var_t7__blk815_dn12),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign11980_e11310;
        locals.var_t2__blk810_dn3 = assign11980_e11310_d_n3;
        locals.var_t2__blk810_dn4 = assign11980_e11310_d_n4;
        locals.var_t2__blk810_dn5 = assign11980_e11310_d_n5;
        locals.var_t2__blk810_dn6 = assign11980_e11310_d_n6;
        locals.var_t2__blk810_dn7 = assign11980_e11310_d_n7;
        locals.var_t2__blk810_dn8 = assign11980_e11310_d_n8;
        locals.var_t2__blk810_dn9 = assign11980_e11310_d_n9;
        locals.var_t2__blk810_dn10 = assign11980_e11310_d_n10;
        locals.var_t2__blk810_dn11 = assign11980_e11310_d_n11;
        locals.var_t2__blk810_dn12 = assign11980_e11310_d_n12;

        let (assign11990_e11316, assign11990_e11316_d_n3, assign11990_e11316_d_n4, assign11990_e11316_d_n5, assign11990_e11316_d_n6, assign11990_e11316_d_n7, assign11990_e11316_d_n8, assign11990_e11316_d_n9, assign11990_e11316_d_n10, assign11990_e11316_d_n11, assign11990_e11316_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign11990_e11314: f64 = (locals.var_pparam_b4soiahlid * locals.var_t0__blk808);
        (assign11990_e11314, ((locals.var_pparam_b4soiahlid_dn3 * locals.var_t0__blk808) + (locals.var_pparam_b4soiahlid * locals.var_t0__blk808_dn3)), ((locals.var_pparam_b4soiahlid_dn4 * locals.var_t0__blk808) + (locals.var_pparam_b4soiahlid * locals.var_t0__blk808_dn4)), ((locals.var_pparam_b4soiahlid_dn5 * locals.var_t0__blk808) + (locals.var_pparam_b4soiahlid * locals.var_t0__blk808_dn5)), ((locals.var_pparam_b4soiahlid_dn6 * locals.var_t0__blk808) + (locals.var_pparam_b4soiahlid * locals.var_t0__blk808_dn6)), ((locals.var_pparam_b4soiahlid_dn7 * locals.var_t0__blk808) + (locals.var_pparam_b4soiahlid * locals.var_t0__blk808_dn7)), ((locals.var_pparam_b4soiahlid_dn8 * locals.var_t0__blk808) + (locals.var_pparam_b4soiahlid * locals.var_t0__blk808_dn8)), ((locals.var_pparam_b4soiahlid_dn9 * locals.var_t0__blk808) + (locals.var_pparam_b4soiahlid * locals.var_t0__blk808_dn9)), ((locals.var_pparam_b4soiahlid_dn10 * locals.var_t0__blk808) + (locals.var_pparam_b4soiahlid * locals.var_t0__blk808_dn10)), ((locals.var_pparam_b4soiahlid_dn11 * locals.var_t0__blk808) + (locals.var_pparam_b4soiahlid * locals.var_t0__blk808_dn11)), ((locals.var_pparam_b4soiahlid_dn12 * locals.var_t0__blk808) + (locals.var_pparam_b4soiahlid * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_ahlid, locals.var_ahlid_dn3, locals.var_ahlid_dn4, locals.var_ahlid_dn5, locals.var_ahlid_dn6, locals.var_ahlid_dn7, locals.var_ahlid_dn8, locals.var_ahlid_dn9, locals.var_ahlid_dn10, locals.var_ahlid_dn11, locals.var_ahlid_dn12,)
    }
};
        locals.var_ahlid = assign11990_e11316;
        locals.var_ahlid_dn3 = assign11990_e11316_d_n3;
        locals.var_ahlid_dn4 = assign11990_e11316_d_n4;
        locals.var_ahlid_dn5 = assign11990_e11316_d_n5;
        locals.var_ahlid_dn6 = assign11990_e11316_d_n6;
        locals.var_ahlid_dn7 = assign11990_e11316_d_n7;
        locals.var_ahlid_dn8 = assign11990_e11316_d_n8;
        locals.var_ahlid_dn9 = assign11990_e11316_d_n9;
        locals.var_ahlid_dn10 = assign11990_e11316_d_n10;
        locals.var_ahlid_dn11 = assign11990_e11316_d_n11;
        locals.var_ahlid_dn12 = assign11990_e11316_d_n12;

    }

    pub(super) fn stamp_transient_block_30(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12000_e11322, assign12000_e11322_d_n3, assign12000_e11322_d_n4, assign12000_e11322_d_n5, assign12000_e11322_d_n6, assign12000_e11322_d_n7, assign12000_e11322_d_n8, assign12000_e11322_d_n9, assign12000_e11322_d_n10, assign12000_e11322_d_n11, assign12000_e11322_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12000_e11320: f64 = (locals.var_pparam_b4soiidbjt * locals.var_t0__blk808);
        (assign12000_e11320, ((locals.var_pparam_b4soiidbjt_dn3 * locals.var_t0__blk808) + (locals.var_pparam_b4soiidbjt * locals.var_t0__blk808_dn3)), ((locals.var_pparam_b4soiidbjt_dn4 * locals.var_t0__blk808) + (locals.var_pparam_b4soiidbjt * locals.var_t0__blk808_dn4)), ((locals.var_pparam_b4soiidbjt_dn5 * locals.var_t0__blk808) + (locals.var_pparam_b4soiidbjt * locals.var_t0__blk808_dn5)), ((locals.var_pparam_b4soiidbjt_dn6 * locals.var_t0__blk808) + (locals.var_pparam_b4soiidbjt * locals.var_t0__blk808_dn6)), ((locals.var_pparam_b4soiidbjt_dn7 * locals.var_t0__blk808) + (locals.var_pparam_b4soiidbjt * locals.var_t0__blk808_dn7)), ((locals.var_pparam_b4soiidbjt_dn8 * locals.var_t0__blk808) + (locals.var_pparam_b4soiidbjt * locals.var_t0__blk808_dn8)), ((locals.var_pparam_b4soiidbjt_dn9 * locals.var_t0__blk808) + (locals.var_pparam_b4soiidbjt * locals.var_t0__blk808_dn9)), ((locals.var_pparam_b4soiidbjt_dn10 * locals.var_t0__blk808) + (locals.var_pparam_b4soiidbjt * locals.var_t0__blk808_dn10)), ((locals.var_pparam_b4soiidbjt_dn11 * locals.var_t0__blk808) + (locals.var_pparam_b4soiidbjt * locals.var_t0__blk808_dn11)), ((locals.var_pparam_b4soiidbjt_dn12 * locals.var_t0__blk808) + (locals.var_pparam_b4soiidbjt * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_jbjtd, locals.var_jbjtd_dn3, locals.var_jbjtd_dn4, locals.var_jbjtd_dn5, locals.var_jbjtd_dn6, locals.var_jbjtd_dn7, locals.var_jbjtd_dn8, locals.var_jbjtd_dn9, locals.var_jbjtd_dn10, locals.var_jbjtd_dn11, locals.var_jbjtd_dn12,)
    }
};
        locals.var_jbjtd = assign12000_e11322;
        locals.var_jbjtd_dn3 = assign12000_e11322_d_n3;
        locals.var_jbjtd_dn4 = assign12000_e11322_d_n4;
        locals.var_jbjtd_dn5 = assign12000_e11322_d_n5;
        locals.var_jbjtd_dn6 = assign12000_e11322_d_n6;
        locals.var_jbjtd_dn7 = assign12000_e11322_d_n7;
        locals.var_jbjtd_dn8 = assign12000_e11322_d_n8;
        locals.var_jbjtd_dn9 = assign12000_e11322_d_n9;
        locals.var_jbjtd_dn10 = assign12000_e11322_d_n10;
        locals.var_jbjtd_dn11 = assign12000_e11322_d_n11;
        locals.var_jbjtd_dn12 = assign12000_e11322_d_n12;

        let (assign12010_e11328, assign12010_e11328_d_n3, assign12010_e11328_d_n4, assign12010_e11328_d_n5, assign12010_e11328_d_n6, assign12010_e11328_d_n7, assign12010_e11328_d_n8, assign12010_e11328_d_n9, assign12010_e11328_d_n10, assign12010_e11328_d_n11, assign12010_e11328_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12010_e11326: f64 = (locals.var_pparam_b4soiiddif * locals.var_t1__blk809);
        (assign12010_e11326, ((locals.var_pparam_b4soiiddif_dn3 * locals.var_t1__blk809) + (locals.var_pparam_b4soiiddif * locals.var_t1__blk809_dn3)), ((locals.var_pparam_b4soiiddif_dn4 * locals.var_t1__blk809) + (locals.var_pparam_b4soiiddif * locals.var_t1__blk809_dn4)), ((locals.var_pparam_b4soiiddif_dn5 * locals.var_t1__blk809) + (locals.var_pparam_b4soiiddif * locals.var_t1__blk809_dn5)), ((locals.var_pparam_b4soiiddif_dn6 * locals.var_t1__blk809) + (locals.var_pparam_b4soiiddif * locals.var_t1__blk809_dn6)), ((locals.var_pparam_b4soiiddif_dn7 * locals.var_t1__blk809) + (locals.var_pparam_b4soiiddif * locals.var_t1__blk809_dn7)), ((locals.var_pparam_b4soiiddif_dn8 * locals.var_t1__blk809) + (locals.var_pparam_b4soiiddif * locals.var_t1__blk809_dn8)), ((locals.var_pparam_b4soiiddif_dn9 * locals.var_t1__blk809) + (locals.var_pparam_b4soiiddif * locals.var_t1__blk809_dn9)), ((locals.var_pparam_b4soiiddif_dn10 * locals.var_t1__blk809) + (locals.var_pparam_b4soiiddif * locals.var_t1__blk809_dn10)), ((locals.var_pparam_b4soiiddif_dn11 * locals.var_t1__blk809) + (locals.var_pparam_b4soiiddif * locals.var_t1__blk809_dn11)), ((locals.var_pparam_b4soiiddif_dn12 * locals.var_t1__blk809) + (locals.var_pparam_b4soiiddif * locals.var_t1__blk809_dn12)),)
    } else {
        (locals.var_jdifd, locals.var_jdifd_dn3, locals.var_jdifd_dn4, locals.var_jdifd_dn5, locals.var_jdifd_dn6, locals.var_jdifd_dn7, locals.var_jdifd_dn8, locals.var_jdifd_dn9, locals.var_jdifd_dn10, locals.var_jdifd_dn11, locals.var_jdifd_dn12,)
    }
};
        locals.var_jdifd = assign12010_e11328;
        locals.var_jdifd_dn3 = assign12010_e11328_d_n3;
        locals.var_jdifd_dn4 = assign12010_e11328_d_n4;
        locals.var_jdifd_dn5 = assign12010_e11328_d_n5;
        locals.var_jdifd_dn6 = assign12010_e11328_d_n6;
        locals.var_jdifd_dn7 = assign12010_e11328_d_n7;
        locals.var_jdifd_dn8 = assign12010_e11328_d_n8;
        locals.var_jdifd_dn9 = assign12010_e11328_d_n9;
        locals.var_jdifd_dn10 = assign12010_e11328_d_n10;
        locals.var_jdifd_dn11 = assign12010_e11328_d_n11;
        locals.var_jdifd_dn12 = assign12010_e11328_d_n12;

        let (assign12020_e11334, assign12020_e11334_d_n3, assign12020_e11334_d_n4, assign12020_e11334_d_n5, assign12020_e11334_d_n6, assign12020_e11334_d_n7, assign12020_e11334_d_n8, assign12020_e11334_d_n9, assign12020_e11334_d_n10, assign12020_e11334_d_n11, assign12020_e11334_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12020_e11332: f64 = (locals.var_pparam_b4soiidrec * locals.var_t2__blk810);
        (assign12020_e11332, ((locals.var_pparam_b4soiidrec_dn3 * locals.var_t2__blk810) + (locals.var_pparam_b4soiidrec * locals.var_t2__blk810_dn3)), ((locals.var_pparam_b4soiidrec_dn4 * locals.var_t2__blk810) + (locals.var_pparam_b4soiidrec * locals.var_t2__blk810_dn4)), ((locals.var_pparam_b4soiidrec_dn5 * locals.var_t2__blk810) + (locals.var_pparam_b4soiidrec * locals.var_t2__blk810_dn5)), ((locals.var_pparam_b4soiidrec_dn6 * locals.var_t2__blk810) + (locals.var_pparam_b4soiidrec * locals.var_t2__blk810_dn6)), ((locals.var_pparam_b4soiidrec_dn7 * locals.var_t2__blk810) + (locals.var_pparam_b4soiidrec * locals.var_t2__blk810_dn7)), ((locals.var_pparam_b4soiidrec_dn8 * locals.var_t2__blk810) + (locals.var_pparam_b4soiidrec * locals.var_t2__blk810_dn8)), ((locals.var_pparam_b4soiidrec_dn9 * locals.var_t2__blk810) + (locals.var_pparam_b4soiidrec * locals.var_t2__blk810_dn9)), ((locals.var_pparam_b4soiidrec_dn10 * locals.var_t2__blk810) + (locals.var_pparam_b4soiidrec * locals.var_t2__blk810_dn10)), ((locals.var_pparam_b4soiidrec_dn11 * locals.var_t2__blk810) + (locals.var_pparam_b4soiidrec * locals.var_t2__blk810_dn11)), ((locals.var_pparam_b4soiidrec_dn12 * locals.var_t2__blk810) + (locals.var_pparam_b4soiidrec * locals.var_t2__blk810_dn12)),)
    } else {
        (locals.var_jrecd, locals.var_jrecd_dn3, locals.var_jrecd_dn4, locals.var_jrecd_dn5, locals.var_jrecd_dn6, locals.var_jrecd_dn7, locals.var_jrecd_dn8, locals.var_jrecd_dn9, locals.var_jrecd_dn10, locals.var_jrecd_dn11, locals.var_jrecd_dn12,)
    }
};
        locals.var_jrecd = assign12020_e11334;
        locals.var_jrecd_dn3 = assign12020_e11334_d_n3;
        locals.var_jrecd_dn4 = assign12020_e11334_d_n4;
        locals.var_jrecd_dn5 = assign12020_e11334_d_n5;
        locals.var_jrecd_dn6 = assign12020_e11334_d_n6;
        locals.var_jrecd_dn7 = assign12020_e11334_d_n7;
        locals.var_jrecd_dn8 = assign12020_e11334_d_n8;
        locals.var_jrecd_dn9 = assign12020_e11334_d_n9;
        locals.var_jrecd_dn10 = assign12020_e11334_d_n10;
        locals.var_jrecd_dn11 = assign12020_e11334_d_n11;
        locals.var_jrecd_dn12 = assign12020_e11334_d_n12;

        let (assign12030_e11340, assign12030_e11340_d_n3, assign12030_e11340_d_n4, assign12030_e11340_d_n5, assign12030_e11340_d_n6, assign12030_e11340_d_n7, assign12030_e11340_d_n8, assign12030_e11340_d_n9, assign12030_e11340_d_n10, assign12030_e11340_d_n11, assign12030_e11340_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12030_e11338: f64 = (locals.var_pparam_b4soixtund * locals.var_trm1);
        (assign12030_e11338, (locals.var_pparam_b4soixtund_dn3 * locals.var_trm1), ((locals.var_pparam_b4soixtund_dn4 * locals.var_trm1) + (locals.var_pparam_b4soixtund * locals.var_trm1_dn4)), ((locals.var_pparam_b4soixtund_dn5 * locals.var_trm1) + (locals.var_pparam_b4soixtund * locals.var_trm1_dn5)), ((locals.var_pparam_b4soixtund_dn6 * locals.var_trm1) + (locals.var_pparam_b4soixtund * locals.var_trm1_dn6)), (locals.var_pparam_b4soixtund_dn7 * locals.var_trm1), (locals.var_pparam_b4soixtund_dn8 * locals.var_trm1), (locals.var_pparam_b4soixtund_dn9 * locals.var_trm1), (locals.var_pparam_b4soixtund_dn10 * locals.var_trm1), (locals.var_pparam_b4soixtund_dn11 * locals.var_trm1), (locals.var_pparam_b4soixtund_dn12 * locals.var_trm1),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign12030_e11340;
        locals.var_t7__blk815_dn3 = assign12030_e11340_d_n3;
        locals.var_t7__blk815_dn4 = assign12030_e11340_d_n4;
        locals.var_t7__blk815_dn5 = assign12030_e11340_d_n5;
        locals.var_t7__blk815_dn6 = assign12030_e11340_d_n6;
        locals.var_t7__blk815_dn7 = assign12030_e11340_d_n7;
        locals.var_t7__blk815_dn8 = assign12030_e11340_d_n8;
        locals.var_t7__blk815_dn9 = assign12030_e11340_d_n9;
        locals.var_t7__blk815_dn10 = assign12030_e11340_d_n10;
        locals.var_t7__blk815_dn11 = assign12030_e11340_d_n11;
        locals.var_t7__blk815_dn12 = assign12030_e11340_d_n12;

        let assign12040_e11343: f64 = if locals.var_t7__blk815 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1144 = assign12040_e11343;

        let (assign12050_e11355, assign12050_e11355_d_n3, assign12050_e11355_d_n4, assign12050_e11355_d_n5, assign12050_e11355_d_n6, assign12050_e11355_d_n7, assign12050_e11355_d_n8, assign12050_e11355_d_n9, assign12050_e11355_d_n10, assign12050_e11355_d_n11, assign12050_e11355_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1144 != 0.0)) {
        let assign12050_e11350: f64 = (1.0 + locals.var_t7__blk815);
        let assign12050_e11352: f64 = (assign12050_e11350 - 100.0);
        let assign12050_e11353: f64 = (2.688117142e43 * assign12050_e11352);
        (assign12050_e11353, (2.688117142e43 * locals.var_t7__blk815_dn3), (2.688117142e43 * locals.var_t7__blk815_dn4), (2.688117142e43 * locals.var_t7__blk815_dn5), (2.688117142e43 * locals.var_t7__blk815_dn6), (2.688117142e43 * locals.var_t7__blk815_dn7), (2.688117142e43 * locals.var_t7__blk815_dn8), (2.688117142e43 * locals.var_t7__blk815_dn9), (2.688117142e43 * locals.var_t7__blk815_dn10), (2.688117142e43 * locals.var_t7__blk815_dn11), (2.688117142e43 * locals.var_t7__blk815_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign12050_e11355;
        locals.var_t0__blk808_dn3 = assign12050_e11355_d_n3;
        locals.var_t0__blk808_dn4 = assign12050_e11355_d_n4;
        locals.var_t0__blk808_dn5 = assign12050_e11355_d_n5;
        locals.var_t0__blk808_dn6 = assign12050_e11355_d_n6;
        locals.var_t0__blk808_dn7 = assign12050_e11355_d_n7;
        locals.var_t0__blk808_dn8 = assign12050_e11355_d_n8;
        locals.var_t0__blk808_dn9 = assign12050_e11355_d_n9;
        locals.var_t0__blk808_dn10 = assign12050_e11355_d_n10;
        locals.var_t0__blk808_dn11 = assign12050_e11355_d_n11;
        locals.var_t0__blk808_dn12 = assign12050_e11355_d_n12;

        let assign12060_e11358: f64 = (-100.0);
        let assign12060_e11359: f64 = if locals.var_t7__blk815 < assign12060_e11358 { 1.0 } else { 0.0 };
        locals.var_guard1145 = assign12060_e11359;

        let (assign12070_e11368, assign12070_e11368_d_n3, assign12070_e11368_d_n4, assign12070_e11368_d_n5, assign12070_e11368_d_n6, assign12070_e11368_d_n7, assign12070_e11368_d_n8, assign12070_e11368_d_n9, assign12070_e11368_d_n10, assign12070_e11368_d_n11, assign12070_e11368_d_n12,) = {
    if (((locals.var_guard1124 != 0.0) && (locals.var_guard1144 == 0.0)) && (locals.var_guard1145 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign12070_e11368;
        locals.var_t0__blk808_dn3 = assign12070_e11368_d_n3;
        locals.var_t0__blk808_dn4 = assign12070_e11368_d_n4;
        locals.var_t0__blk808_dn5 = assign12070_e11368_d_n5;
        locals.var_t0__blk808_dn6 = assign12070_e11368_d_n6;
        locals.var_t0__blk808_dn7 = assign12070_e11368_d_n7;
        locals.var_t0__blk808_dn8 = assign12070_e11368_d_n8;
        locals.var_t0__blk808_dn9 = assign12070_e11368_d_n9;
        locals.var_t0__blk808_dn10 = assign12070_e11368_d_n10;
        locals.var_t0__blk808_dn11 = assign12070_e11368_d_n11;
        locals.var_t0__blk808_dn12 = assign12070_e11368_d_n12;

        let (assign12080_e11379, assign12080_e11379_d_n3, assign12080_e11379_d_n4, assign12080_e11379_d_n5, assign12080_e11379_d_n6, assign12080_e11379_d_n7, assign12080_e11379_d_n8, assign12080_e11379_d_n9, assign12080_e11379_d_n10, assign12080_e11379_d_n11, assign12080_e11379_d_n12,) = {
    if (((locals.var_guard1124 != 0.0) && (locals.var_guard1144 == 0.0)) && (locals.var_guard1145 == 0.0)) {
        let assign12080_e11377: f64 = (locals.var_t7__blk815).exp();
        (assign12080_e11377, (assign12080_e11377 * locals.var_t7__blk815_dn3), (assign12080_e11377 * locals.var_t7__blk815_dn4), (assign12080_e11377 * locals.var_t7__blk815_dn5), (assign12080_e11377 * locals.var_t7__blk815_dn6), (assign12080_e11377 * locals.var_t7__blk815_dn7), (assign12080_e11377 * locals.var_t7__blk815_dn8), (assign12080_e11377 * locals.var_t7__blk815_dn9), (assign12080_e11377 * locals.var_t7__blk815_dn10), (assign12080_e11377 * locals.var_t7__blk815_dn11), (assign12080_e11377 * locals.var_t7__blk815_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign12080_e11379;
        locals.var_t0__blk808_dn3 = assign12080_e11379_d_n3;
        locals.var_t0__blk808_dn4 = assign12080_e11379_d_n4;
        locals.var_t0__blk808_dn5 = assign12080_e11379_d_n5;
        locals.var_t0__blk808_dn6 = assign12080_e11379_d_n6;
        locals.var_t0__blk808_dn7 = assign12080_e11379_d_n7;
        locals.var_t0__blk808_dn8 = assign12080_e11379_d_n8;
        locals.var_t0__blk808_dn9 = assign12080_e11379_d_n9;
        locals.var_t0__blk808_dn10 = assign12080_e11379_d_n10;
        locals.var_t0__blk808_dn11 = assign12080_e11379_d_n11;
        locals.var_t0__blk808_dn12 = assign12080_e11379_d_n12;

        let (assign12090_e11385, assign12090_e11385_d_n3, assign12090_e11385_d_n4, assign12090_e11385_d_n5, assign12090_e11385_d_n6, assign12090_e11385_d_n7, assign12090_e11385_d_n8, assign12090_e11385_d_n9, assign12090_e11385_d_n10, assign12090_e11385_d_n11, assign12090_e11385_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12090_e11383: f64 = (locals.var_pparam_b4soiidtun * locals.var_t0__blk808);
        (assign12090_e11383, ((locals.var_pparam_b4soiidtun_dn3 * locals.var_t0__blk808) + (locals.var_pparam_b4soiidtun * locals.var_t0__blk808_dn3)), ((locals.var_pparam_b4soiidtun_dn4 * locals.var_t0__blk808) + (locals.var_pparam_b4soiidtun * locals.var_t0__blk808_dn4)), ((locals.var_pparam_b4soiidtun_dn5 * locals.var_t0__blk808) + (locals.var_pparam_b4soiidtun * locals.var_t0__blk808_dn5)), ((locals.var_pparam_b4soiidtun_dn6 * locals.var_t0__blk808) + (locals.var_pparam_b4soiidtun * locals.var_t0__blk808_dn6)), ((locals.var_pparam_b4soiidtun_dn7 * locals.var_t0__blk808) + (locals.var_pparam_b4soiidtun * locals.var_t0__blk808_dn7)), ((locals.var_pparam_b4soiidtun_dn8 * locals.var_t0__blk808) + (locals.var_pparam_b4soiidtun * locals.var_t0__blk808_dn8)), ((locals.var_pparam_b4soiidtun_dn9 * locals.var_t0__blk808) + (locals.var_pparam_b4soiidtun * locals.var_t0__blk808_dn9)), ((locals.var_pparam_b4soiidtun_dn10 * locals.var_t0__blk808) + (locals.var_pparam_b4soiidtun * locals.var_t0__blk808_dn10)), ((locals.var_pparam_b4soiidtun_dn11 * locals.var_t0__blk808) + (locals.var_pparam_b4soiidtun * locals.var_t0__blk808_dn11)), ((locals.var_pparam_b4soiidtun_dn12 * locals.var_t0__blk808) + (locals.var_pparam_b4soiidtun * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_jtund, locals.var_jtund_dn3, locals.var_jtund_dn4, locals.var_jtund_dn5, locals.var_jtund_dn6, locals.var_jtund_dn7, locals.var_jtund_dn8, locals.var_jtund_dn9, locals.var_jtund_dn10, locals.var_jtund_dn11, locals.var_jtund_dn12,)
    }
};
        locals.var_jtund = assign12090_e11385;
        locals.var_jtund_dn3 = assign12090_e11385_d_n3;
        locals.var_jtund_dn4 = assign12090_e11385_d_n4;
        locals.var_jtund_dn5 = assign12090_e11385_d_n5;
        locals.var_jtund_dn6 = assign12090_e11385_d_n6;
        locals.var_jtund_dn7 = assign12090_e11385_d_n7;
        locals.var_jtund_dn8 = assign12090_e11385_d_n8;
        locals.var_jtund_dn9 = assign12090_e11385_d_n9;
        locals.var_jtund_dn10 = assign12090_e11385_d_n10;
        locals.var_jtund_dn11 = assign12090_e11385_d_n11;
        locals.var_jtund_dn12 = assign12090_e11385_d_n12;

        let (assign12100_e11393, assign12100_e11393_d_n3, assign12100_e11393_d_n4, assign12100_e11393_d_n5, assign12100_e11393_d_n6, assign12100_e11393_d_n7, assign12100_e11393_d_n8, assign12100_e11393_d_n9, assign12100_e11393_d_n10, assign12100_e11393_d_n11, assign12100_e11393_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12100_e11390: f64 = (locals.var_tempratio).powf(locals.var_pparam_b4soiute);
        let assign12100_e11391: f64 = (locals.var_pparam_b4soiu0 * assign12100_e11390);
        (assign12100_e11391, ((locals.var_pparam_b4soiu0_dn3 * assign12100_e11390) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn3 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign12100_e11390 * (locals.var_pparam_b4soiute_dn3 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiu0_dn4 * assign12100_e11390) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn4 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { if locals.var_pparam_b4soiute == 0.0 { 0.0 } else { (locals.var_pparam_b4soiute * ((locals.var_tempratio).powf(locals.var_pparam_b4soiute - 1.0) * locals.var_tempratio_dn4)) } } else { (assign12100_e11390 * ((locals.var_pparam_b4soiute_dn4 * (locals.var_tempratio).ln()) + (locals.var_pparam_b4soiute * (locals.var_tempratio_dn4 / locals.var_tempratio)))) })), ((locals.var_pparam_b4soiu0_dn5 * assign12100_e11390) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn5 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { if locals.var_pparam_b4soiute == 0.0 { 0.0 } else { (locals.var_pparam_b4soiute * ((locals.var_tempratio).powf(locals.var_pparam_b4soiute - 1.0) * locals.var_tempratio_dn5)) } } else { (assign12100_e11390 * ((locals.var_pparam_b4soiute_dn5 * (locals.var_tempratio).ln()) + (locals.var_pparam_b4soiute * (locals.var_tempratio_dn5 / locals.var_tempratio)))) })), ((locals.var_pparam_b4soiu0_dn6 * assign12100_e11390) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn6 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { if locals.var_pparam_b4soiute == 0.0 { 0.0 } else { (locals.var_pparam_b4soiute * ((locals.var_tempratio).powf(locals.var_pparam_b4soiute - 1.0) * locals.var_tempratio_dn6)) } } else { (assign12100_e11390 * ((locals.var_pparam_b4soiute_dn6 * (locals.var_tempratio).ln()) + (locals.var_pparam_b4soiute * (locals.var_tempratio_dn6 / locals.var_tempratio)))) })), ((locals.var_pparam_b4soiu0_dn7 * assign12100_e11390) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn7 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign12100_e11390 * (locals.var_pparam_b4soiute_dn7 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiu0_dn8 * assign12100_e11390) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn8 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign12100_e11390 * (locals.var_pparam_b4soiute_dn8 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiu0_dn9 * assign12100_e11390) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn9 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign12100_e11390 * (locals.var_pparam_b4soiute_dn9 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiu0_dn10 * assign12100_e11390) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn10 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign12100_e11390 * (locals.var_pparam_b4soiute_dn10 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiu0_dn11 * assign12100_e11390) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn11 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign12100_e11390 * (locals.var_pparam_b4soiute_dn11 * (locals.var_tempratio).ln())) })), ((locals.var_pparam_b4soiu0_dn12 * assign12100_e11390) + (locals.var_pparam_b4soiu0 * if locals.var_pparam_b4soiute_dn12 == 0.0 && ((locals.var_pparam_b4soiute) as f64).is_finite() && ((locals.var_pparam_b4soiute) as f64).fract() == 0.0 { 0.0 } else { (assign12100_e11390 * (locals.var_pparam_b4soiute_dn12 * (locals.var_tempratio).ln())) })),)
    } else {
        (locals.var_u0temp, locals.var_u0temp_dn3, locals.var_u0temp_dn4, locals.var_u0temp_dn5, locals.var_u0temp_dn6, locals.var_u0temp_dn7, locals.var_u0temp_dn8, locals.var_u0temp_dn9, locals.var_u0temp_dn10, locals.var_u0temp_dn11, locals.var_u0temp_dn12,)
    }
};
        locals.var_u0temp = assign12100_e11393;
        locals.var_u0temp_dn3 = assign12100_e11393_d_n3;
        locals.var_u0temp_dn4 = assign12100_e11393_d_n4;
        locals.var_u0temp_dn5 = assign12100_e11393_d_n5;
        locals.var_u0temp_dn6 = assign12100_e11393_d_n6;
        locals.var_u0temp_dn7 = assign12100_e11393_d_n7;
        locals.var_u0temp_dn8 = assign12100_e11393_d_n8;
        locals.var_u0temp_dn9 = assign12100_e11393_d_n9;
        locals.var_u0temp_dn10 = assign12100_e11393_d_n10;
        locals.var_u0temp_dn11 = assign12100_e11393_d_n11;
        locals.var_u0temp_dn12 = assign12100_e11393_d_n12;

        let assign12110_e11396: f64 = if p.p38 < 4.2 { 1.0 } else { 0.0 };
        locals.var_guard1146 = assign12110_e11396;

        let (assign12120_e11410, assign12120_e11410_d_n3, assign12120_e11410_d_n4, assign12120_e11410_d_n5, assign12120_e11410_d_n6, assign12120_e11410_d_n7, assign12120_e11410_d_n8, assign12120_e11410_d_n9, assign12120_e11410_d_n10, assign12120_e11410_d_n11, assign12120_e11410_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1146 != 0.0)) {
        let assign12120_e11404: f64 = (p.p238 * locals.var_tempratio);
        let assign12120_e11405: f64 = (1.0 + assign12120_e11404);
        let assign12120_e11406: f64 = (locals.var_pparam_b4soiku0 * assign12120_e11405);
        let assign12120_e11408: f64 = (assign12120_e11406 + 1e-9);
        (assign12120_e11408, (locals.var_pparam_b4soiku0_dn3 * assign12120_e11405), ((locals.var_pparam_b4soiku0_dn4 * assign12120_e11405) + (locals.var_pparam_b4soiku0 * (p.p238 * locals.var_tempratio_dn4))), ((locals.var_pparam_b4soiku0_dn5 * assign12120_e11405) + (locals.var_pparam_b4soiku0 * (p.p238 * locals.var_tempratio_dn5))), ((locals.var_pparam_b4soiku0_dn6 * assign12120_e11405) + (locals.var_pparam_b4soiku0 * (p.p238 * locals.var_tempratio_dn6))), (locals.var_pparam_b4soiku0_dn7 * assign12120_e11405), (locals.var_pparam_b4soiku0_dn8 * assign12120_e11405), (locals.var_pparam_b4soiku0_dn9 * assign12120_e11405), (locals.var_pparam_b4soiku0_dn10 * assign12120_e11405), (locals.var_pparam_b4soiku0_dn11 * assign12120_e11405), (locals.var_pparam_b4soiku0_dn12 * assign12120_e11405),)
    } else {
        (locals.var_ku0temp, locals.var_ku0temp_dn3, locals.var_ku0temp_dn4, locals.var_ku0temp_dn5, locals.var_ku0temp_dn6, locals.var_ku0temp_dn7, locals.var_ku0temp_dn8, locals.var_ku0temp_dn9, locals.var_ku0temp_dn10, locals.var_ku0temp_dn11, locals.var_ku0temp_dn12,)
    }
};
        locals.var_ku0temp = assign12120_e11410;
        locals.var_ku0temp_dn3 = assign12120_e11410_d_n3;
        locals.var_ku0temp_dn4 = assign12120_e11410_d_n4;
        locals.var_ku0temp_dn5 = assign12120_e11410_d_n5;
        locals.var_ku0temp_dn6 = assign12120_e11410_d_n6;
        locals.var_ku0temp_dn7 = assign12120_e11410_d_n7;
        locals.var_ku0temp_dn8 = assign12120_e11410_d_n8;
        locals.var_ku0temp_dn9 = assign12120_e11410_d_n9;
        locals.var_ku0temp_dn10 = assign12120_e11410_d_n10;
        locals.var_ku0temp_dn11 = assign12120_e11410_d_n11;
        locals.var_ku0temp_dn12 = assign12120_e11410_d_n12;

        let (assign12130_e11425, assign12130_e11425_d_n3, assign12130_e11425_d_n4, assign12130_e11425_d_n5, assign12130_e11425_d_n6, assign12130_e11425_d_n7, assign12130_e11425_d_n8, assign12130_e11425_d_n9, assign12130_e11425_d_n10, assign12130_e11425_d_n11, assign12130_e11425_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1146 == 0.0)) {
        let assign12130_e11419: f64 = (p.p238 * locals.var_trm1);
        let assign12130_e11420: f64 = (1.0 + assign12130_e11419);
        let assign12130_e11421: f64 = (locals.var_pparam_b4soiku0 * assign12130_e11420);
        let assign12130_e11423: f64 = (assign12130_e11421 + 1e-9);
        (assign12130_e11423, (locals.var_pparam_b4soiku0_dn3 * assign12130_e11420), ((locals.var_pparam_b4soiku0_dn4 * assign12130_e11420) + (locals.var_pparam_b4soiku0 * (p.p238 * locals.var_trm1_dn4))), ((locals.var_pparam_b4soiku0_dn5 * assign12130_e11420) + (locals.var_pparam_b4soiku0 * (p.p238 * locals.var_trm1_dn5))), ((locals.var_pparam_b4soiku0_dn6 * assign12130_e11420) + (locals.var_pparam_b4soiku0 * (p.p238 * locals.var_trm1_dn6))), (locals.var_pparam_b4soiku0_dn7 * assign12130_e11420), (locals.var_pparam_b4soiku0_dn8 * assign12130_e11420), (locals.var_pparam_b4soiku0_dn9 * assign12130_e11420), (locals.var_pparam_b4soiku0_dn10 * assign12130_e11420), (locals.var_pparam_b4soiku0_dn11 * assign12130_e11420), (locals.var_pparam_b4soiku0_dn12 * assign12130_e11420),)
    } else {
        (locals.var_ku0temp, locals.var_ku0temp_dn3, locals.var_ku0temp_dn4, locals.var_ku0temp_dn5, locals.var_ku0temp_dn6, locals.var_ku0temp_dn7, locals.var_ku0temp_dn8, locals.var_ku0temp_dn9, locals.var_ku0temp_dn10, locals.var_ku0temp_dn11, locals.var_ku0temp_dn12,)
    }
};
        locals.var_ku0temp = assign12130_e11425;
        locals.var_ku0temp_dn3 = assign12130_e11425_d_n3;
        locals.var_ku0temp_dn4 = assign12130_e11425_d_n4;
        locals.var_ku0temp_dn5 = assign12130_e11425_d_n5;
        locals.var_ku0temp_dn6 = assign12130_e11425_d_n6;
        locals.var_ku0temp_dn7 = assign12130_e11425_d_n7;
        locals.var_ku0temp_dn8 = assign12130_e11425_d_n8;
        locals.var_ku0temp_dn9 = assign12130_e11425_d_n9;
        locals.var_ku0temp_dn10 = assign12130_e11425_d_n10;
        locals.var_ku0temp_dn11 = assign12130_e11425_d_n11;
        locals.var_ku0temp_dn12 = assign12130_e11425_d_n12;

        let (assign12140_e11431, assign12140_e11431_d_n3, assign12140_e11431_d_n4, assign12140_e11431_d_n5, assign12140_e11431_d_n6, assign12140_e11431_d_n7, assign12140_e11431_d_n8, assign12140_e11431_d_n9, assign12140_e11431_d_n10, assign12140_e11431_d_n11, assign12140_e11431_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12140_e11429: f64 = (p.p235 * locals.var_pparam_b4soiinv_od_ref);
        (assign12140_e11429, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign12140_e11431;
        locals.var_t7__blk815_dn3 = assign12140_e11431_d_n3;
        locals.var_t7__blk815_dn4 = assign12140_e11431_d_n4;
        locals.var_t7__blk815_dn5 = assign12140_e11431_d_n5;
        locals.var_t7__blk815_dn6 = assign12140_e11431_d_n6;
        locals.var_t7__blk815_dn7 = assign12140_e11431_d_n7;
        locals.var_t7__blk815_dn8 = assign12140_e11431_d_n8;
        locals.var_t7__blk815_dn9 = assign12140_e11431_d_n9;
        locals.var_t7__blk815_dn10 = assign12140_e11431_d_n10;
        locals.var_t7__blk815_dn11 = assign12140_e11431_d_n11;
        locals.var_t7__blk815_dn12 = assign12140_e11431_d_n12;

        let (assign12150_e11437, assign12150_e11437_d_n3, assign12150_e11437_d_n4, assign12150_e11437_d_n5, assign12150_e11437_d_n6, assign12150_e11437_d_n7, assign12150_e11437_d_n8, assign12150_e11437_d_n9, assign12150_e11437_d_n10, assign12150_e11437_d_n11, assign12150_e11437_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12150_e11435: f64 = (locals.var_t7__blk815 / locals.var_ku0temp);
        (assign12150_e11435, (((locals.var_t7__blk815_dn3 * locals.var_ku0temp) - (locals.var_t7__blk815 * locals.var_ku0temp_dn3)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t7__blk815_dn4 * locals.var_ku0temp) - (locals.var_t7__blk815 * locals.var_ku0temp_dn4)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t7__blk815_dn5 * locals.var_ku0temp) - (locals.var_t7__blk815 * locals.var_ku0temp_dn5)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t7__blk815_dn6 * locals.var_ku0temp) - (locals.var_t7__blk815 * locals.var_ku0temp_dn6)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t7__blk815_dn7 * locals.var_ku0temp) - (locals.var_t7__blk815 * locals.var_ku0temp_dn7)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t7__blk815_dn8 * locals.var_ku0temp) - (locals.var_t7__blk815 * locals.var_ku0temp_dn8)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t7__blk815_dn9 * locals.var_ku0temp) - (locals.var_t7__blk815 * locals.var_ku0temp_dn9)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t7__blk815_dn10 * locals.var_ku0temp) - (locals.var_t7__blk815 * locals.var_ku0temp_dn10)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t7__blk815_dn11 * locals.var_ku0temp) - (locals.var_t7__blk815 * locals.var_ku0temp_dn11)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t7__blk815_dn12 * locals.var_ku0temp) - (locals.var_t7__blk815 * locals.var_ku0temp_dn12)) / (locals.var_ku0temp * locals.var_ku0temp)),)
    } else {
        (locals.var_rho_ref, locals.var_rho_ref_dn3, locals.var_rho_ref_dn4, locals.var_rho_ref_dn5, locals.var_rho_ref_dn6, locals.var_rho_ref_dn7, locals.var_rho_ref_dn8, locals.var_rho_ref_dn9, locals.var_rho_ref_dn10, locals.var_rho_ref_dn11, locals.var_rho_ref_dn12,)
    }
};
        locals.var_rho_ref = assign12150_e11437;
        locals.var_rho_ref_dn3 = assign12150_e11437_d_n3;
        locals.var_rho_ref_dn4 = assign12150_e11437_d_n4;
        locals.var_rho_ref_dn5 = assign12150_e11437_d_n5;
        locals.var_rho_ref_dn6 = assign12150_e11437_d_n6;
        locals.var_rho_ref_dn7 = assign12150_e11437_d_n7;
        locals.var_rho_ref_dn8 = assign12150_e11437_d_n8;
        locals.var_rho_ref_dn9 = assign12150_e11437_d_n9;
        locals.var_rho_ref_dn10 = assign12150_e11437_d_n10;
        locals.var_rho_ref_dn11 = assign12150_e11437_d_n11;
        locals.var_rho_ref_dn12 = assign12150_e11437_d_n12;

        let (assign12160_e11443, assign12160_e11443_d_n3, assign12160_e11443_d_n4, assign12160_e11443_d_n5, assign12160_e11443_d_n6, assign12160_e11443_d_n7, assign12160_e11443_d_n8, assign12160_e11443_d_n9, assign12160_e11443_d_n10, assign12160_e11443_d_n11, assign12160_e11443_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12160_e11441: f64 = (p.p235 * locals.var_b4soiinv_odeff);
        (assign12160_e11441, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign12160_e11443;
        locals.var_t4__blk812_dn3 = assign12160_e11443_d_n3;
        locals.var_t4__blk812_dn4 = assign12160_e11443_d_n4;
        locals.var_t4__blk812_dn5 = assign12160_e11443_d_n5;
        locals.var_t4__blk812_dn6 = assign12160_e11443_d_n6;
        locals.var_t4__blk812_dn7 = assign12160_e11443_d_n7;
        locals.var_t4__blk812_dn8 = assign12160_e11443_d_n8;
        locals.var_t4__blk812_dn9 = assign12160_e11443_d_n9;
        locals.var_t4__blk812_dn10 = assign12160_e11443_d_n10;
        locals.var_t4__blk812_dn11 = assign12160_e11443_d_n11;
        locals.var_t4__blk812_dn12 = assign12160_e11443_d_n12;

        let (assign12170_e11449, assign12170_e11449_d_n3, assign12170_e11449_d_n4, assign12170_e11449_d_n5, assign12170_e11449_d_n6, assign12170_e11449_d_n7, assign12170_e11449_d_n8, assign12170_e11449_d_n9, assign12170_e11449_d_n10, assign12170_e11449_d_n11, assign12170_e11449_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12170_e11447: f64 = (locals.var_t4__blk812 / locals.var_ku0temp);
        (assign12170_e11447, (((locals.var_t4__blk812_dn3 * locals.var_ku0temp) - (locals.var_t4__blk812 * locals.var_ku0temp_dn3)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t4__blk812_dn4 * locals.var_ku0temp) - (locals.var_t4__blk812 * locals.var_ku0temp_dn4)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t4__blk812_dn5 * locals.var_ku0temp) - (locals.var_t4__blk812 * locals.var_ku0temp_dn5)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t4__blk812_dn6 * locals.var_ku0temp) - (locals.var_t4__blk812 * locals.var_ku0temp_dn6)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t4__blk812_dn7 * locals.var_ku0temp) - (locals.var_t4__blk812 * locals.var_ku0temp_dn7)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t4__blk812_dn8 * locals.var_ku0temp) - (locals.var_t4__blk812 * locals.var_ku0temp_dn8)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t4__blk812_dn9 * locals.var_ku0temp) - (locals.var_t4__blk812 * locals.var_ku0temp_dn9)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t4__blk812_dn10 * locals.var_ku0temp) - (locals.var_t4__blk812 * locals.var_ku0temp_dn10)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t4__blk812_dn11 * locals.var_ku0temp) - (locals.var_t4__blk812 * locals.var_ku0temp_dn11)) / (locals.var_ku0temp * locals.var_ku0temp)), (((locals.var_t4__blk812_dn12 * locals.var_ku0temp) - (locals.var_t4__blk812 * locals.var_ku0temp_dn12)) / (locals.var_ku0temp * locals.var_ku0temp)),)
    } else {
        (locals.var_rho__blk924, locals.var_rho__blk924_dn3, locals.var_rho__blk924_dn4, locals.var_rho__blk924_dn5, locals.var_rho__blk924_dn6, locals.var_rho__blk924_dn7, locals.var_rho__blk924_dn8, locals.var_rho__blk924_dn9, locals.var_rho__blk924_dn10, locals.var_rho__blk924_dn11, locals.var_rho__blk924_dn12,)
    }
};
        locals.var_rho__blk924 = assign12170_e11449;
        locals.var_rho__blk924_dn3 = assign12170_e11449_d_n3;
        locals.var_rho__blk924_dn4 = assign12170_e11449_d_n4;
        locals.var_rho__blk924_dn5 = assign12170_e11449_d_n5;
        locals.var_rho__blk924_dn6 = assign12170_e11449_d_n6;
        locals.var_rho__blk924_dn7 = assign12170_e11449_d_n7;
        locals.var_rho__blk924_dn8 = assign12170_e11449_d_n8;
        locals.var_rho__blk924_dn9 = assign12170_e11449_d_n9;
        locals.var_rho__blk924_dn10 = assign12170_e11449_d_n10;
        locals.var_rho__blk924_dn11 = assign12170_e11449_d_n11;
        locals.var_rho__blk924_dn12 = assign12170_e11449_d_n12;

        let (assign12180_e11455, assign12180_e11455_d_n3, assign12180_e11455_d_n4, assign12180_e11455_d_n5, assign12180_e11455_d_n6, assign12180_e11455_d_n7, assign12180_e11455_d_n8, assign12180_e11455_d_n9, assign12180_e11455_d_n10, assign12180_e11455_d_n11, assign12180_e11455_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12180_e11453: f64 = (1.0 + locals.var_rho__blk924);
        (assign12180_e11453, locals.var_rho__blk924_dn3, locals.var_rho__blk924_dn4, locals.var_rho__blk924_dn5, locals.var_rho__blk924_dn6, locals.var_rho__blk924_dn7, locals.var_rho__blk924_dn8, locals.var_rho__blk924_dn9, locals.var_rho__blk924_dn10, locals.var_rho__blk924_dn11, locals.var_rho__blk924_dn12,)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign12180_e11455;
        locals.var_t2__blk810_dn3 = assign12180_e11455_d_n3;
        locals.var_t2__blk810_dn4 = assign12180_e11455_d_n4;
        locals.var_t2__blk810_dn5 = assign12180_e11455_d_n5;
        locals.var_t2__blk810_dn6 = assign12180_e11455_d_n6;
        locals.var_t2__blk810_dn7 = assign12180_e11455_d_n7;
        locals.var_t2__blk810_dn8 = assign12180_e11455_d_n8;
        locals.var_t2__blk810_dn9 = assign12180_e11455_d_n9;
        locals.var_t2__blk810_dn10 = assign12180_e11455_d_n10;
        locals.var_t2__blk810_dn11 = assign12180_e11455_d_n11;
        locals.var_t2__blk810_dn12 = assign12180_e11455_d_n12;

        let (assign12190_e11461, assign12190_e11461_d_n3, assign12190_e11461_d_n4, assign12190_e11461_d_n5, assign12190_e11461_d_n6, assign12190_e11461_d_n7, assign12190_e11461_d_n8, assign12190_e11461_d_n9, assign12190_e11461_d_n10, assign12190_e11461_d_n11, assign12190_e11461_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12190_e11459: f64 = (1.0 + locals.var_rho_ref);
        (assign12190_e11459, locals.var_rho_ref_dn3, locals.var_rho_ref_dn4, locals.var_rho_ref_dn5, locals.var_rho_ref_dn6, locals.var_rho_ref_dn7, locals.var_rho_ref_dn8, locals.var_rho_ref_dn9, locals.var_rho_ref_dn10, locals.var_rho_ref_dn11, locals.var_rho_ref_dn12,)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign12190_e11461;
        locals.var_t7__blk815_dn3 = assign12190_e11461_d_n3;
        locals.var_t7__blk815_dn4 = assign12190_e11461_d_n4;
        locals.var_t7__blk815_dn5 = assign12190_e11461_d_n5;
        locals.var_t7__blk815_dn6 = assign12190_e11461_d_n6;
        locals.var_t7__blk815_dn7 = assign12190_e11461_d_n7;
        locals.var_t7__blk815_dn8 = assign12190_e11461_d_n8;
        locals.var_t7__blk815_dn9 = assign12190_e11461_d_n9;
        locals.var_t7__blk815_dn10 = assign12190_e11461_d_n10;
        locals.var_t7__blk815_dn11 = assign12190_e11461_d_n11;
        locals.var_t7__blk815_dn12 = assign12190_e11461_d_n12;

        let (assign12200_e11467, assign12200_e11467_d_n3, assign12200_e11467_d_n4, assign12200_e11467_d_n5, assign12200_e11467_d_n6, assign12200_e11467_d_n7, assign12200_e11467_d_n8, assign12200_e11467_d_n9, assign12200_e11467_d_n10, assign12200_e11467_d_n11, assign12200_e11467_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12200_e11465: f64 = (locals.var_t2__blk810 / locals.var_t7__blk815);
        (assign12200_e11465, (((locals.var_t2__blk810_dn3 * locals.var_t7__blk815) - (locals.var_t2__blk810 * locals.var_t7__blk815_dn3)) / (locals.var_t7__blk815 * locals.var_t7__blk815)), (((locals.var_t2__blk810_dn4 * locals.var_t7__blk815) - (locals.var_t2__blk810 * locals.var_t7__blk815_dn4)) / (locals.var_t7__blk815 * locals.var_t7__blk815)), (((locals.var_t2__blk810_dn5 * locals.var_t7__blk815) - (locals.var_t2__blk810 * locals.var_t7__blk815_dn5)) / (locals.var_t7__blk815 * locals.var_t7__blk815)), (((locals.var_t2__blk810_dn6 * locals.var_t7__blk815) - (locals.var_t2__blk810 * locals.var_t7__blk815_dn6)) / (locals.var_t7__blk815 * locals.var_t7__blk815)), (((locals.var_t2__blk810_dn7 * locals.var_t7__blk815) - (locals.var_t2__blk810 * locals.var_t7__blk815_dn7)) / (locals.var_t7__blk815 * locals.var_t7__blk815)), (((locals.var_t2__blk810_dn8 * locals.var_t7__blk815) - (locals.var_t2__blk810 * locals.var_t7__blk815_dn8)) / (locals.var_t7__blk815 * locals.var_t7__blk815)), (((locals.var_t2__blk810_dn9 * locals.var_t7__blk815) - (locals.var_t2__blk810 * locals.var_t7__blk815_dn9)) / (locals.var_t7__blk815 * locals.var_t7__blk815)), (((locals.var_t2__blk810_dn10 * locals.var_t7__blk815) - (locals.var_t2__blk810 * locals.var_t7__blk815_dn10)) / (locals.var_t7__blk815 * locals.var_t7__blk815)), (((locals.var_t2__blk810_dn11 * locals.var_t7__blk815) - (locals.var_t2__blk810 * locals.var_t7__blk815_dn11)) / (locals.var_t7__blk815 * locals.var_t7__blk815)), (((locals.var_t2__blk810_dn12 * locals.var_t7__blk815) - (locals.var_t2__blk810 * locals.var_t7__blk815_dn12)) / (locals.var_t7__blk815 * locals.var_t7__blk815)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign12200_e11467;
        locals.var_t0__blk808_dn3 = assign12200_e11467_d_n3;
        locals.var_t0__blk808_dn4 = assign12200_e11467_d_n4;
        locals.var_t0__blk808_dn5 = assign12200_e11467_d_n5;
        locals.var_t0__blk808_dn6 = assign12200_e11467_d_n6;
        locals.var_t0__blk808_dn7 = assign12200_e11467_d_n7;
        locals.var_t0__blk808_dn8 = assign12200_e11467_d_n8;
        locals.var_t0__blk808_dn9 = assign12200_e11467_d_n9;
        locals.var_t0__blk808_dn10 = assign12200_e11467_d_n10;
        locals.var_t0__blk808_dn11 = assign12200_e11467_d_n11;
        locals.var_t0__blk808_dn12 = assign12200_e11467_d_n12;

        let (assign12210_e11473, assign12210_e11473_d_n3, assign12210_e11473_d_n4, assign12210_e11473_d_n5, assign12210_e11473_d_n6, assign12210_e11473_d_n7, assign12210_e11473_d_n8, assign12210_e11473_d_n9, assign12210_e11473_d_n10, assign12210_e11473_d_n11, assign12210_e11473_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12210_e11471: f64 = (locals.var_u0temp * locals.var_t0__blk808);
        (assign12210_e11471, ((locals.var_u0temp_dn3 * locals.var_t0__blk808) + (locals.var_u0temp * locals.var_t0__blk808_dn3)), ((locals.var_u0temp_dn4 * locals.var_t0__blk808) + (locals.var_u0temp * locals.var_t0__blk808_dn4)), ((locals.var_u0temp_dn5 * locals.var_t0__blk808) + (locals.var_u0temp * locals.var_t0__blk808_dn5)), ((locals.var_u0temp_dn6 * locals.var_t0__blk808) + (locals.var_u0temp * locals.var_t0__blk808_dn6)), ((locals.var_u0temp_dn7 * locals.var_t0__blk808) + (locals.var_u0temp * locals.var_t0__blk808_dn7)), ((locals.var_u0temp_dn8 * locals.var_t0__blk808) + (locals.var_u0temp * locals.var_t0__blk808_dn8)), ((locals.var_u0temp_dn9 * locals.var_t0__blk808) + (locals.var_u0temp * locals.var_t0__blk808_dn9)), ((locals.var_u0temp_dn10 * locals.var_t0__blk808) + (locals.var_u0temp * locals.var_t0__blk808_dn10)), ((locals.var_u0temp_dn11 * locals.var_t0__blk808) + (locals.var_u0temp * locals.var_t0__blk808_dn11)), ((locals.var_u0temp_dn12 * locals.var_t0__blk808) + (locals.var_u0temp * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_u0temp, locals.var_u0temp_dn3, locals.var_u0temp_dn4, locals.var_u0temp_dn5, locals.var_u0temp_dn6, locals.var_u0temp_dn7, locals.var_u0temp_dn8, locals.var_u0temp_dn9, locals.var_u0temp_dn10, locals.var_u0temp_dn11, locals.var_u0temp_dn12,)
    }
};
        locals.var_u0temp = assign12210_e11473;
        locals.var_u0temp_dn3 = assign12210_e11473_d_n3;
        locals.var_u0temp_dn4 = assign12210_e11473_d_n4;
        locals.var_u0temp_dn5 = assign12210_e11473_d_n5;
        locals.var_u0temp_dn6 = assign12210_e11473_d_n6;
        locals.var_u0temp_dn7 = assign12210_e11473_d_n7;
        locals.var_u0temp_dn8 = assign12210_e11473_d_n8;
        locals.var_u0temp_dn9 = assign12210_e11473_d_n9;
        locals.var_u0temp_dn10 = assign12210_e11473_d_n10;
        locals.var_u0temp_dn11 = assign12210_e11473_d_n11;
        locals.var_u0temp_dn12 = assign12210_e11473_d_n12;

        let (assign12220_e11481, assign12220_e11481_d_n3, assign12220_e11481_d_n4, assign12220_e11481_d_n5, assign12220_e11481_d_n6, assign12220_e11481_d_n7, assign12220_e11481_d_n8, assign12220_e11481_d_n9, assign12220_e11481_d_n10, assign12220_e11481_d_n11, assign12220_e11481_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12220_e11478: f64 = (locals.var_pparam_b4soiat * locals.var_trm1);
        let assign12220_e11479: f64 = (locals.var_pparam_b4soivsat - assign12220_e11478);
        (assign12220_e11479, (locals.var_pparam_b4soivsat_dn3 - (locals.var_pparam_b4soiat_dn3 * locals.var_trm1)), (locals.var_pparam_b4soivsat_dn4 - ((locals.var_pparam_b4soiat_dn4 * locals.var_trm1) + (locals.var_pparam_b4soiat * locals.var_trm1_dn4))), (locals.var_pparam_b4soivsat_dn5 - ((locals.var_pparam_b4soiat_dn5 * locals.var_trm1) + (locals.var_pparam_b4soiat * locals.var_trm1_dn5))), (locals.var_pparam_b4soivsat_dn6 - ((locals.var_pparam_b4soiat_dn6 * locals.var_trm1) + (locals.var_pparam_b4soiat * locals.var_trm1_dn6))), (locals.var_pparam_b4soivsat_dn7 - (locals.var_pparam_b4soiat_dn7 * locals.var_trm1)), (locals.var_pparam_b4soivsat_dn8 - (locals.var_pparam_b4soiat_dn8 * locals.var_trm1)), (locals.var_pparam_b4soivsat_dn9 - (locals.var_pparam_b4soiat_dn9 * locals.var_trm1)), (locals.var_pparam_b4soivsat_dn10 - (locals.var_pparam_b4soiat_dn10 * locals.var_trm1)), (locals.var_pparam_b4soivsat_dn11 - (locals.var_pparam_b4soiat_dn11 * locals.var_trm1)), (locals.var_pparam_b4soivsat_dn12 - (locals.var_pparam_b4soiat_dn12 * locals.var_trm1)),)
    } else {
        (locals.var_vsattemp, locals.var_vsattemp_dn3, locals.var_vsattemp_dn4, locals.var_vsattemp_dn5, locals.var_vsattemp_dn6, locals.var_vsattemp_dn7, locals.var_vsattemp_dn8, locals.var_vsattemp_dn9, locals.var_vsattemp_dn10, locals.var_vsattemp_dn11, locals.var_vsattemp_dn12,)
    }
};
        locals.var_vsattemp = assign12220_e11481;
        locals.var_vsattemp_dn3 = assign12220_e11481_d_n3;
        locals.var_vsattemp_dn4 = assign12220_e11481_d_n4;
        locals.var_vsattemp_dn5 = assign12220_e11481_d_n5;
        locals.var_vsattemp_dn6 = assign12220_e11481_d_n6;
        locals.var_vsattemp_dn7 = assign12220_e11481_d_n7;
        locals.var_vsattemp_dn8 = assign12220_e11481_d_n8;
        locals.var_vsattemp_dn9 = assign12220_e11481_d_n9;
        locals.var_vsattemp_dn10 = assign12220_e11481_d_n10;
        locals.var_vsattemp_dn11 = assign12220_e11481_d_n11;
        locals.var_vsattemp_dn12 = assign12220_e11481_d_n12;

        let (assign12230_e11489, assign12230_e11489_d_n3, assign12230_e11489_d_n4, assign12230_e11489_d_n5, assign12230_e11489_d_n6, assign12230_e11489_d_n7, assign12230_e11489_d_n8, assign12230_e11489_d_n9, assign12230_e11489_d_n10, assign12230_e11489_d_n11, assign12230_e11489_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12230_e11486: f64 = (locals.var_b4soikvsat * locals.var_rho__blk924);
        let assign12230_e11487: f64 = (1.0 + assign12230_e11486);
        (assign12230_e11487, (locals.var_b4soikvsat * locals.var_rho__blk924_dn3), (locals.var_b4soikvsat * locals.var_rho__blk924_dn4), (locals.var_b4soikvsat * locals.var_rho__blk924_dn5), (locals.var_b4soikvsat * locals.var_rho__blk924_dn6), (locals.var_b4soikvsat * locals.var_rho__blk924_dn7), (locals.var_b4soikvsat * locals.var_rho__blk924_dn8), (locals.var_b4soikvsat * locals.var_rho__blk924_dn9), (locals.var_b4soikvsat * locals.var_rho__blk924_dn10), (locals.var_b4soikvsat * locals.var_rho__blk924_dn11), (locals.var_b4soikvsat * locals.var_rho__blk924_dn12),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign12230_e11489;
        locals.var_t2__blk810_dn3 = assign12230_e11489_d_n3;
        locals.var_t2__blk810_dn4 = assign12230_e11489_d_n4;
        locals.var_t2__blk810_dn5 = assign12230_e11489_d_n5;
        locals.var_t2__blk810_dn6 = assign12230_e11489_d_n6;
        locals.var_t2__blk810_dn7 = assign12230_e11489_d_n7;
        locals.var_t2__blk810_dn8 = assign12230_e11489_d_n8;
        locals.var_t2__blk810_dn9 = assign12230_e11489_d_n9;
        locals.var_t2__blk810_dn10 = assign12230_e11489_d_n10;
        locals.var_t2__blk810_dn11 = assign12230_e11489_d_n11;
        locals.var_t2__blk810_dn12 = assign12230_e11489_d_n12;

        let (assign12240_e11497, assign12240_e11497_d_n3, assign12240_e11497_d_n4, assign12240_e11497_d_n5, assign12240_e11497_d_n6, assign12240_e11497_d_n7, assign12240_e11497_d_n8, assign12240_e11497_d_n9, assign12240_e11497_d_n10, assign12240_e11497_d_n11, assign12240_e11497_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12240_e11494: f64 = (locals.var_b4soikvsat * locals.var_rho_ref);
        let assign12240_e11495: f64 = (1.0 + assign12240_e11494);
        (assign12240_e11495, (locals.var_b4soikvsat * locals.var_rho_ref_dn3), (locals.var_b4soikvsat * locals.var_rho_ref_dn4), (locals.var_b4soikvsat * locals.var_rho_ref_dn5), (locals.var_b4soikvsat * locals.var_rho_ref_dn6), (locals.var_b4soikvsat * locals.var_rho_ref_dn7), (locals.var_b4soikvsat * locals.var_rho_ref_dn8), (locals.var_b4soikvsat * locals.var_rho_ref_dn9), (locals.var_b4soikvsat * locals.var_rho_ref_dn10), (locals.var_b4soikvsat * locals.var_rho_ref_dn11), (locals.var_b4soikvsat * locals.var_rho_ref_dn12),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign12240_e11497;
        locals.var_t7__blk815_dn3 = assign12240_e11497_d_n3;
        locals.var_t7__blk815_dn4 = assign12240_e11497_d_n4;
        locals.var_t7__blk815_dn5 = assign12240_e11497_d_n5;
        locals.var_t7__blk815_dn6 = assign12240_e11497_d_n6;
        locals.var_t7__blk815_dn7 = assign12240_e11497_d_n7;
        locals.var_t7__blk815_dn8 = assign12240_e11497_d_n8;
        locals.var_t7__blk815_dn9 = assign12240_e11497_d_n9;
        locals.var_t7__blk815_dn10 = assign12240_e11497_d_n10;
        locals.var_t7__blk815_dn11 = assign12240_e11497_d_n11;
        locals.var_t7__blk815_dn12 = assign12240_e11497_d_n12;

        let (assign12250_e11503, assign12250_e11503_d_n3, assign12250_e11503_d_n4, assign12250_e11503_d_n5, assign12250_e11503_d_n6, assign12250_e11503_d_n7, assign12250_e11503_d_n8, assign12250_e11503_d_n9, assign12250_e11503_d_n10, assign12250_e11503_d_n11, assign12250_e11503_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12250_e11501: f64 = (locals.var_t2__blk810 / locals.var_t7__blk815);
        (assign12250_e11501, (((locals.var_t2__blk810_dn3 * locals.var_t7__blk815) - (locals.var_t2__blk810 * locals.var_t7__blk815_dn3)) / (locals.var_t7__blk815 * locals.var_t7__blk815)), (((locals.var_t2__blk810_dn4 * locals.var_t7__blk815) - (locals.var_t2__blk810 * locals.var_t7__blk815_dn4)) / (locals.var_t7__blk815 * locals.var_t7__blk815)), (((locals.var_t2__blk810_dn5 * locals.var_t7__blk815) - (locals.var_t2__blk810 * locals.var_t7__blk815_dn5)) / (locals.var_t7__blk815 * locals.var_t7__blk815)), (((locals.var_t2__blk810_dn6 * locals.var_t7__blk815) - (locals.var_t2__blk810 * locals.var_t7__blk815_dn6)) / (locals.var_t7__blk815 * locals.var_t7__blk815)), (((locals.var_t2__blk810_dn7 * locals.var_t7__blk815) - (locals.var_t2__blk810 * locals.var_t7__blk815_dn7)) / (locals.var_t7__blk815 * locals.var_t7__blk815)), (((locals.var_t2__blk810_dn8 * locals.var_t7__blk815) - (locals.var_t2__blk810 * locals.var_t7__blk815_dn8)) / (locals.var_t7__blk815 * locals.var_t7__blk815)), (((locals.var_t2__blk810_dn9 * locals.var_t7__blk815) - (locals.var_t2__blk810 * locals.var_t7__blk815_dn9)) / (locals.var_t7__blk815 * locals.var_t7__blk815)), (((locals.var_t2__blk810_dn10 * locals.var_t7__blk815) - (locals.var_t2__blk810 * locals.var_t7__blk815_dn10)) / (locals.var_t7__blk815 * locals.var_t7__blk815)), (((locals.var_t2__blk810_dn11 * locals.var_t7__blk815) - (locals.var_t2__blk810 * locals.var_t7__blk815_dn11)) / (locals.var_t7__blk815 * locals.var_t7__blk815)), (((locals.var_t2__blk810_dn12 * locals.var_t7__blk815) - (locals.var_t2__blk810 * locals.var_t7__blk815_dn12)) / (locals.var_t7__blk815 * locals.var_t7__blk815)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign12250_e11503;
        locals.var_t0__blk808_dn3 = assign12250_e11503_d_n3;
        locals.var_t0__blk808_dn4 = assign12250_e11503_d_n4;
        locals.var_t0__blk808_dn5 = assign12250_e11503_d_n5;
        locals.var_t0__blk808_dn6 = assign12250_e11503_d_n6;
        locals.var_t0__blk808_dn7 = assign12250_e11503_d_n7;
        locals.var_t0__blk808_dn8 = assign12250_e11503_d_n8;
        locals.var_t0__blk808_dn9 = assign12250_e11503_d_n9;
        locals.var_t0__blk808_dn10 = assign12250_e11503_d_n10;
        locals.var_t0__blk808_dn11 = assign12250_e11503_d_n11;
        locals.var_t0__blk808_dn12 = assign12250_e11503_d_n12;

        let (assign12260_e11509, assign12260_e11509_d_n3, assign12260_e11509_d_n4, assign12260_e11509_d_n5, assign12260_e11509_d_n6, assign12260_e11509_d_n7, assign12260_e11509_d_n8, assign12260_e11509_d_n9, assign12260_e11509_d_n10, assign12260_e11509_d_n11, assign12260_e11509_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12260_e11507: f64 = (locals.var_vsattemp * locals.var_t0__blk808);
        (assign12260_e11507, ((locals.var_vsattemp_dn3 * locals.var_t0__blk808) + (locals.var_vsattemp * locals.var_t0__blk808_dn3)), ((locals.var_vsattemp_dn4 * locals.var_t0__blk808) + (locals.var_vsattemp * locals.var_t0__blk808_dn4)), ((locals.var_vsattemp_dn5 * locals.var_t0__blk808) + (locals.var_vsattemp * locals.var_t0__blk808_dn5)), ((locals.var_vsattemp_dn6 * locals.var_t0__blk808) + (locals.var_vsattemp * locals.var_t0__blk808_dn6)), ((locals.var_vsattemp_dn7 * locals.var_t0__blk808) + (locals.var_vsattemp * locals.var_t0__blk808_dn7)), ((locals.var_vsattemp_dn8 * locals.var_t0__blk808) + (locals.var_vsattemp * locals.var_t0__blk808_dn8)), ((locals.var_vsattemp_dn9 * locals.var_t0__blk808) + (locals.var_vsattemp * locals.var_t0__blk808_dn9)), ((locals.var_vsattemp_dn10 * locals.var_t0__blk808) + (locals.var_vsattemp * locals.var_t0__blk808_dn10)), ((locals.var_vsattemp_dn11 * locals.var_t0__blk808) + (locals.var_vsattemp * locals.var_t0__blk808_dn11)), ((locals.var_vsattemp_dn12 * locals.var_t0__blk808) + (locals.var_vsattemp * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_vsattemp, locals.var_vsattemp_dn3, locals.var_vsattemp_dn4, locals.var_vsattemp_dn5, locals.var_vsattemp_dn6, locals.var_vsattemp_dn7, locals.var_vsattemp_dn8, locals.var_vsattemp_dn9, locals.var_vsattemp_dn10, locals.var_vsattemp_dn11, locals.var_vsattemp_dn12,)
    }
};
        locals.var_vsattemp = assign12260_e11509;
        locals.var_vsattemp_dn3 = assign12260_e11509_d_n3;
        locals.var_vsattemp_dn4 = assign12260_e11509_d_n4;
        locals.var_vsattemp_dn5 = assign12260_e11509_d_n5;
        locals.var_vsattemp_dn6 = assign12260_e11509_d_n6;
        locals.var_vsattemp_dn7 = assign12260_e11509_d_n7;
        locals.var_vsattemp_dn8 = assign12260_e11509_d_n8;
        locals.var_vsattemp_dn9 = assign12260_e11509_d_n9;
        locals.var_vsattemp_dn10 = assign12260_e11509_d_n10;
        locals.var_vsattemp_dn11 = assign12260_e11509_d_n11;
        locals.var_vsattemp_dn12 = assign12260_e11509_d_n12;

        let assign12270_e11512: f64 = if p.p429 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1147 = assign12270_e11512;

    }

    pub(super) fn stamp_transient_block_31(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12280_e11524, assign12280_e11524_d_n3, assign12280_e11524_d_n4, assign12280_e11524_d_n5, assign12280_e11524_d_n6, assign12280_e11524_d_n7, assign12280_e11524_d_n8, assign12280_e11524_d_n9, assign12280_e11524_d_n10, assign12280_e11524_d_n11, assign12280_e11524_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1147 != 0.0)) {
        let assign12280_e11519: f64 = (locals.var_pparam_b4soiprt * locals.var_trm1);
        let assign12280_e11520: f64 = (locals.var_pparam_b4soirdsw + assign12280_e11519);
        let assign12280_e11522: f64 = (assign12280_e11520 / locals.var_pparam_b4soirds0denom);
        (assign12280_e11522, ((((locals.var_pparam_b4soirdsw_dn3 + (locals.var_pparam_b4soiprt_dn3 * locals.var_trm1)) * locals.var_pparam_b4soirds0denom) - (assign12280_e11520 * locals.var_pparam_b4soirds0denom_dn3)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom)), ((((locals.var_pparam_b4soirdsw_dn4 + ((locals.var_pparam_b4soiprt_dn4 * locals.var_trm1) + (locals.var_pparam_b4soiprt * locals.var_trm1_dn4))) * locals.var_pparam_b4soirds0denom) - (assign12280_e11520 * locals.var_pparam_b4soirds0denom_dn4)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom)), ((((locals.var_pparam_b4soirdsw_dn5 + ((locals.var_pparam_b4soiprt_dn5 * locals.var_trm1) + (locals.var_pparam_b4soiprt * locals.var_trm1_dn5))) * locals.var_pparam_b4soirds0denom) - (assign12280_e11520 * locals.var_pparam_b4soirds0denom_dn5)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom)), ((((locals.var_pparam_b4soirdsw_dn6 + ((locals.var_pparam_b4soiprt_dn6 * locals.var_trm1) + (locals.var_pparam_b4soiprt * locals.var_trm1_dn6))) * locals.var_pparam_b4soirds0denom) - (assign12280_e11520 * locals.var_pparam_b4soirds0denom_dn6)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom)), ((((locals.var_pparam_b4soirdsw_dn7 + (locals.var_pparam_b4soiprt_dn7 * locals.var_trm1)) * locals.var_pparam_b4soirds0denom) - (assign12280_e11520 * locals.var_pparam_b4soirds0denom_dn7)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom)), ((((locals.var_pparam_b4soirdsw_dn8 + (locals.var_pparam_b4soiprt_dn8 * locals.var_trm1)) * locals.var_pparam_b4soirds0denom) - (assign12280_e11520 * locals.var_pparam_b4soirds0denom_dn8)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom)), ((((locals.var_pparam_b4soirdsw_dn9 + (locals.var_pparam_b4soiprt_dn9 * locals.var_trm1)) * locals.var_pparam_b4soirds0denom) - (assign12280_e11520 * locals.var_pparam_b4soirds0denom_dn9)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom)), ((((locals.var_pparam_b4soirdsw_dn10 + (locals.var_pparam_b4soiprt_dn10 * locals.var_trm1)) * locals.var_pparam_b4soirds0denom) - (assign12280_e11520 * locals.var_pparam_b4soirds0denom_dn10)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom)), ((((locals.var_pparam_b4soirdsw_dn11 + (locals.var_pparam_b4soiprt_dn11 * locals.var_trm1)) * locals.var_pparam_b4soirds0denom) - (assign12280_e11520 * locals.var_pparam_b4soirds0denom_dn11)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom)), ((((locals.var_pparam_b4soirdsw_dn12 + (locals.var_pparam_b4soiprt_dn12 * locals.var_trm1)) * locals.var_pparam_b4soirds0denom) - (assign12280_e11520 * locals.var_pparam_b4soirds0denom_dn12)) / (locals.var_pparam_b4soirds0denom * locals.var_pparam_b4soirds0denom)),)
    } else {
        (locals.var_rds0, locals.var_rds0_dn3, locals.var_rds0_dn4, locals.var_rds0_dn5, locals.var_rds0_dn6, locals.var_rds0_dn7, locals.var_rds0_dn8, locals.var_rds0_dn9, locals.var_rds0_dn10, locals.var_rds0_dn11, locals.var_rds0_dn12,)
    }
};
        locals.var_rds0 = assign12280_e11524;
        locals.var_rds0_dn3 = assign12280_e11524_d_n3;
        locals.var_rds0_dn4 = assign12280_e11524_d_n4;
        locals.var_rds0_dn5 = assign12280_e11524_d_n5;
        locals.var_rds0_dn6 = assign12280_e11524_d_n6;
        locals.var_rds0_dn7 = assign12280_e11524_d_n7;
        locals.var_rds0_dn8 = assign12280_e11524_d_n8;
        locals.var_rds0_dn9 = assign12280_e11524_d_n9;
        locals.var_rds0_dn10 = assign12280_e11524_d_n10;
        locals.var_rds0_dn11 = assign12280_e11524_d_n11;
        locals.var_rds0_dn12 = assign12280_e11524_d_n12;

        let (assign12290_e11530, assign12290_e11530_d_n3, assign12290_e11530_d_n4, assign12290_e11530_d_n5, assign12290_e11530_d_n6, assign12290_e11530_d_n7, assign12290_e11530_d_n8, assign12290_e11530_d_n9, assign12290_e11530_d_n10, assign12290_e11530_d_n11, assign12290_e11530_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1147 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rd0, locals.var_rd0_dn3, locals.var_rd0_dn4, locals.var_rd0_dn5, locals.var_rd0_dn6, locals.var_rd0_dn7, locals.var_rd0_dn8, locals.var_rd0_dn9, locals.var_rd0_dn10, locals.var_rd0_dn11, locals.var_rd0_dn12,)
    }
};
        locals.var_rd0 = assign12290_e11530;
        locals.var_rd0_dn3 = assign12290_e11530_d_n3;
        locals.var_rd0_dn4 = assign12290_e11530_d_n4;
        locals.var_rd0_dn5 = assign12290_e11530_d_n5;
        locals.var_rd0_dn6 = assign12290_e11530_d_n6;
        locals.var_rd0_dn7 = assign12290_e11530_d_n7;
        locals.var_rd0_dn8 = assign12290_e11530_d_n8;
        locals.var_rd0_dn9 = assign12290_e11530_d_n9;
        locals.var_rd0_dn10 = assign12290_e11530_d_n10;
        locals.var_rd0_dn11 = assign12290_e11530_d_n11;
        locals.var_rd0_dn12 = assign12290_e11530_d_n12;

        let (assign12300_e11536, assign12300_e11536_d_n3, assign12300_e11536_d_n4, assign12300_e11536_d_n5, assign12300_e11536_d_n6, assign12300_e11536_d_n7, assign12300_e11536_d_n8, assign12300_e11536_d_n9, assign12300_e11536_d_n10, assign12300_e11536_d_n11, assign12300_e11536_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1147 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rs0, locals.var_rs0_dn3, locals.var_rs0_dn4, locals.var_rs0_dn5, locals.var_rs0_dn6, locals.var_rs0_dn7, locals.var_rs0_dn8, locals.var_rs0_dn9, locals.var_rs0_dn10, locals.var_rs0_dn11, locals.var_rs0_dn12,)
    }
};
        locals.var_rs0 = assign12300_e11536;
        locals.var_rs0_dn3 = assign12300_e11536_d_n3;
        locals.var_rs0_dn4 = assign12300_e11536_d_n4;
        locals.var_rs0_dn5 = assign12300_e11536_d_n5;
        locals.var_rs0_dn6 = assign12300_e11536_d_n6;
        locals.var_rs0_dn7 = assign12300_e11536_d_n7;
        locals.var_rs0_dn8 = assign12300_e11536_d_n8;
        locals.var_rs0_dn9 = assign12300_e11536_d_n9;
        locals.var_rs0_dn10 = assign12300_e11536_d_n10;
        locals.var_rs0_dn11 = assign12300_e11536_d_n11;
        locals.var_rs0_dn12 = assign12300_e11536_d_n12;

        let (assign12310_e11543, assign12310_e11543_d_n3, assign12310_e11543_d_n4, assign12310_e11543_d_n5, assign12310_e11543_d_n6, assign12310_e11543_d_n7, assign12310_e11543_d_n8, assign12310_e11543_d_n9, assign12310_e11543_d_n10, assign12310_e11543_d_n11, assign12310_e11543_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1147 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rds0, locals.var_rds0_dn3, locals.var_rds0_dn4, locals.var_rds0_dn5, locals.var_rds0_dn6, locals.var_rds0_dn7, locals.var_rds0_dn8, locals.var_rds0_dn9, locals.var_rds0_dn10, locals.var_rds0_dn11, locals.var_rds0_dn12,)
    }
};
        locals.var_rds0 = assign12310_e11543;
        locals.var_rds0_dn3 = assign12310_e11543_d_n3;
        locals.var_rds0_dn4 = assign12310_e11543_d_n4;
        locals.var_rds0_dn5 = assign12310_e11543_d_n5;
        locals.var_rds0_dn6 = assign12310_e11543_d_n6;
        locals.var_rds0_dn7 = assign12310_e11543_d_n7;
        locals.var_rds0_dn8 = assign12310_e11543_d_n8;
        locals.var_rds0_dn9 = assign12310_e11543_d_n9;
        locals.var_rds0_dn10 = assign12310_e11543_d_n10;
        locals.var_rds0_dn11 = assign12310_e11543_d_n11;
        locals.var_rds0_dn12 = assign12310_e11543_d_n12;

        let (assign12320_e11552, assign12320_e11552_d_n3, assign12320_e11552_d_n4, assign12320_e11552_d_n5, assign12320_e11552_d_n6, assign12320_e11552_d_n7, assign12320_e11552_d_n8, assign12320_e11552_d_n9, assign12320_e11552_d_n10, assign12320_e11552_d_n11, assign12320_e11552_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1147 == 0.0)) {
        let assign12320_e11550: f64 = (locals.var_pparam_b4soirds0denom * p.p3);
        (assign12320_e11550, (locals.var_pparam_b4soirds0denom_dn3 * p.p3), (locals.var_pparam_b4soirds0denom_dn4 * p.p3), (locals.var_pparam_b4soirds0denom_dn5 * p.p3), (locals.var_pparam_b4soirds0denom_dn6 * p.p3), (locals.var_pparam_b4soirds0denom_dn7 * p.p3), (locals.var_pparam_b4soirds0denom_dn8 * p.p3), (locals.var_pparam_b4soirds0denom_dn9 * p.p3), (locals.var_pparam_b4soirds0denom_dn10 * p.p3), (locals.var_pparam_b4soirds0denom_dn11 * p.p3), (locals.var_pparam_b4soirds0denom_dn12 * p.p3),)
    } else {
        (locals.var_powweffwr__blk1059, locals.var_powweffwr__blk1059_dn3, locals.var_powweffwr__blk1059_dn4, locals.var_powweffwr__blk1059_dn5, locals.var_powweffwr__blk1059_dn6, locals.var_powweffwr__blk1059_dn7, locals.var_powweffwr__blk1059_dn8, locals.var_powweffwr__blk1059_dn9, locals.var_powweffwr__blk1059_dn10, locals.var_powweffwr__blk1059_dn11, locals.var_powweffwr__blk1059_dn12,)
    }
};
        locals.var_powweffwr__blk1059 = assign12320_e11552;
        locals.var_powweffwr__blk1059_dn3 = assign12320_e11552_d_n3;
        locals.var_powweffwr__blk1059_dn4 = assign12320_e11552_d_n4;
        locals.var_powweffwr__blk1059_dn5 = assign12320_e11552_d_n5;
        locals.var_powweffwr__blk1059_dn6 = assign12320_e11552_d_n6;
        locals.var_powweffwr__blk1059_dn7 = assign12320_e11552_d_n7;
        locals.var_powweffwr__blk1059_dn8 = assign12320_e11552_d_n8;
        locals.var_powweffwr__blk1059_dn9 = assign12320_e11552_d_n9;
        locals.var_powweffwr__blk1059_dn10 = assign12320_e11552_d_n10;
        locals.var_powweffwr__blk1059_dn11 = assign12320_e11552_d_n11;
        locals.var_powweffwr__blk1059_dn12 = assign12320_e11552_d_n12;

        let (assign12330_e11561, assign12330_e11561_d_n3, assign12330_e11561_d_n4, assign12330_e11561_d_n5, assign12330_e11561_d_n6, assign12330_e11561_d_n7, assign12330_e11561_d_n8, assign12330_e11561_d_n9, assign12330_e11561_d_n10, assign12330_e11561_d_n11, assign12330_e11561_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1147 == 0.0)) {
        let assign12330_e11559: f64 = (locals.var_pparam_b4soiprt * locals.var_trm1);
        (assign12330_e11559, (locals.var_pparam_b4soiprt_dn3 * locals.var_trm1), ((locals.var_pparam_b4soiprt_dn4 * locals.var_trm1) + (locals.var_pparam_b4soiprt * locals.var_trm1_dn4)), ((locals.var_pparam_b4soiprt_dn5 * locals.var_trm1) + (locals.var_pparam_b4soiprt * locals.var_trm1_dn5)), ((locals.var_pparam_b4soiprt_dn6 * locals.var_trm1) + (locals.var_pparam_b4soiprt * locals.var_trm1_dn6)), (locals.var_pparam_b4soiprt_dn7 * locals.var_trm1), (locals.var_pparam_b4soiprt_dn8 * locals.var_trm1), (locals.var_pparam_b4soiprt_dn9 * locals.var_trm1), (locals.var_pparam_b4soiprt_dn10 * locals.var_trm1), (locals.var_pparam_b4soiprt_dn11 * locals.var_trm1), (locals.var_pparam_b4soiprt_dn12 * locals.var_trm1),)
    } else {
        (locals.var_t10__blk818, locals.var_t10__blk818_dn3, locals.var_t10__blk818_dn4, locals.var_t10__blk818_dn5, locals.var_t10__blk818_dn6, locals.var_t10__blk818_dn7, locals.var_t10__blk818_dn8, locals.var_t10__blk818_dn9, locals.var_t10__blk818_dn10, locals.var_t10__blk818_dn11, locals.var_t10__blk818_dn12,)
    }
};
        locals.var_t10__blk818 = assign12330_e11561;
        locals.var_t10__blk818_dn3 = assign12330_e11561_d_n3;
        locals.var_t10__blk818_dn4 = assign12330_e11561_d_n4;
        locals.var_t10__blk818_dn5 = assign12330_e11561_d_n5;
        locals.var_t10__blk818_dn6 = assign12330_e11561_d_n6;
        locals.var_t10__blk818_dn7 = assign12330_e11561_d_n7;
        locals.var_t10__blk818_dn8 = assign12330_e11561_d_n8;
        locals.var_t10__blk818_dn9 = assign12330_e11561_d_n9;
        locals.var_t10__blk818_dn10 = assign12330_e11561_d_n10;
        locals.var_t10__blk818_dn11 = assign12330_e11561_d_n11;
        locals.var_t10__blk818_dn12 = assign12330_e11561_d_n12;

        let (assign12340_e11570, assign12340_e11570_d_n3, assign12340_e11570_d_n4, assign12340_e11570_d_n5, assign12340_e11570_d_n6, assign12340_e11570_d_n7, assign12340_e11570_d_n8, assign12340_e11570_d_n9, assign12340_e11570_d_n10, assign12340_e11570_d_n11, assign12340_e11570_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1147 == 0.0)) {
        let assign12340_e11568: f64 = (locals.var_pparam_b4soirdw + locals.var_t10__blk818);
        (assign12340_e11568, (locals.var_pparam_b4soirdw_dn3 + locals.var_t10__blk818_dn3), (locals.var_pparam_b4soirdw_dn4 + locals.var_t10__blk818_dn4), (locals.var_pparam_b4soirdw_dn5 + locals.var_t10__blk818_dn5), (locals.var_pparam_b4soirdw_dn6 + locals.var_t10__blk818_dn6), (locals.var_pparam_b4soirdw_dn7 + locals.var_t10__blk818_dn7), (locals.var_pparam_b4soirdw_dn8 + locals.var_t10__blk818_dn8), (locals.var_pparam_b4soirdw_dn9 + locals.var_t10__blk818_dn9), (locals.var_pparam_b4soirdw_dn10 + locals.var_t10__blk818_dn10), (locals.var_pparam_b4soirdw_dn11 + locals.var_t10__blk818_dn11), (locals.var_pparam_b4soirdw_dn12 + locals.var_t10__blk818_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign12340_e11570;
        locals.var_t1__blk809_dn3 = assign12340_e11570_d_n3;
        locals.var_t1__blk809_dn4 = assign12340_e11570_d_n4;
        locals.var_t1__blk809_dn5 = assign12340_e11570_d_n5;
        locals.var_t1__blk809_dn6 = assign12340_e11570_d_n6;
        locals.var_t1__blk809_dn7 = assign12340_e11570_d_n7;
        locals.var_t1__blk809_dn8 = assign12340_e11570_d_n8;
        locals.var_t1__blk809_dn9 = assign12340_e11570_d_n9;
        locals.var_t1__blk809_dn10 = assign12340_e11570_d_n10;
        locals.var_t1__blk809_dn11 = assign12340_e11570_d_n11;
        locals.var_t1__blk809_dn12 = assign12340_e11570_d_n12;

        let (assign12350_e11579, assign12350_e11579_d_n3, assign12350_e11579_d_n4, assign12350_e11579_d_n5, assign12350_e11579_d_n6, assign12350_e11579_d_n7, assign12350_e11579_d_n8, assign12350_e11579_d_n9, assign12350_e11579_d_n10, assign12350_e11579_d_n11, assign12350_e11579_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1147 == 0.0)) {
        let assign12350_e11577: f64 = (p.p140 + locals.var_t10__blk818);
        (assign12350_e11577, locals.var_t10__blk818_dn3, locals.var_t10__blk818_dn4, locals.var_t10__blk818_dn5, locals.var_t10__blk818_dn6, locals.var_t10__blk818_dn7, locals.var_t10__blk818_dn8, locals.var_t10__blk818_dn9, locals.var_t10__blk818_dn10, locals.var_t10__blk818_dn11, locals.var_t10__blk818_dn12,)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign12350_e11579;
        locals.var_t2__blk810_dn3 = assign12350_e11579_d_n3;
        locals.var_t2__blk810_dn4 = assign12350_e11579_d_n4;
        locals.var_t2__blk810_dn5 = assign12350_e11579_d_n5;
        locals.var_t2__blk810_dn6 = assign12350_e11579_d_n6;
        locals.var_t2__blk810_dn7 = assign12350_e11579_d_n7;
        locals.var_t2__blk810_dn8 = assign12350_e11579_d_n8;
        locals.var_t2__blk810_dn9 = assign12350_e11579_d_n9;
        locals.var_t2__blk810_dn10 = assign12350_e11579_d_n10;
        locals.var_t2__blk810_dn11 = assign12350_e11579_d_n11;
        locals.var_t2__blk810_dn12 = assign12350_e11579_d_n12;

        let (assign12360_e11588, assign12360_e11588_d_n3, assign12360_e11588_d_n4, assign12360_e11588_d_n5, assign12360_e11588_d_n6, assign12360_e11588_d_n7, assign12360_e11588_d_n8, assign12360_e11588_d_n9, assign12360_e11588_d_n10, assign12360_e11588_d_n11, assign12360_e11588_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1147 == 0.0)) {
        let assign12360_e11586: f64 = (locals.var_t1__blk809 / locals.var_powweffwr__blk1059);
        (assign12360_e11586, (((locals.var_t1__blk809_dn3 * locals.var_powweffwr__blk1059) - (locals.var_t1__blk809 * locals.var_powweffwr__blk1059_dn3)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t1__blk809_dn4 * locals.var_powweffwr__blk1059) - (locals.var_t1__blk809 * locals.var_powweffwr__blk1059_dn4)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t1__blk809_dn5 * locals.var_powweffwr__blk1059) - (locals.var_t1__blk809 * locals.var_powweffwr__blk1059_dn5)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t1__blk809_dn6 * locals.var_powweffwr__blk1059) - (locals.var_t1__blk809 * locals.var_powweffwr__blk1059_dn6)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t1__blk809_dn7 * locals.var_powweffwr__blk1059) - (locals.var_t1__blk809 * locals.var_powweffwr__blk1059_dn7)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t1__blk809_dn8 * locals.var_powweffwr__blk1059) - (locals.var_t1__blk809 * locals.var_powweffwr__blk1059_dn8)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t1__blk809_dn9 * locals.var_powweffwr__blk1059) - (locals.var_t1__blk809 * locals.var_powweffwr__blk1059_dn9)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t1__blk809_dn10 * locals.var_powweffwr__blk1059) - (locals.var_t1__blk809 * locals.var_powweffwr__blk1059_dn10)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t1__blk809_dn11 * locals.var_powweffwr__blk1059) - (locals.var_t1__blk809 * locals.var_powweffwr__blk1059_dn11)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t1__blk809_dn12 * locals.var_powweffwr__blk1059) - (locals.var_t1__blk809 * locals.var_powweffwr__blk1059_dn12)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)),)
    } else {
        (locals.var_rd0, locals.var_rd0_dn3, locals.var_rd0_dn4, locals.var_rd0_dn5, locals.var_rd0_dn6, locals.var_rd0_dn7, locals.var_rd0_dn8, locals.var_rd0_dn9, locals.var_rd0_dn10, locals.var_rd0_dn11, locals.var_rd0_dn12,)
    }
};
        locals.var_rd0 = assign12360_e11588;
        locals.var_rd0_dn3 = assign12360_e11588_d_n3;
        locals.var_rd0_dn4 = assign12360_e11588_d_n4;
        locals.var_rd0_dn5 = assign12360_e11588_d_n5;
        locals.var_rd0_dn6 = assign12360_e11588_d_n6;
        locals.var_rd0_dn7 = assign12360_e11588_d_n7;
        locals.var_rd0_dn8 = assign12360_e11588_d_n8;
        locals.var_rd0_dn9 = assign12360_e11588_d_n9;
        locals.var_rd0_dn10 = assign12360_e11588_d_n10;
        locals.var_rd0_dn11 = assign12360_e11588_d_n11;
        locals.var_rd0_dn12 = assign12360_e11588_d_n12;

        let (assign12370_e11597, assign12370_e11597_d_n3, assign12370_e11597_d_n4, assign12370_e11597_d_n5, assign12370_e11597_d_n6, assign12370_e11597_d_n7, assign12370_e11597_d_n8, assign12370_e11597_d_n9, assign12370_e11597_d_n10, assign12370_e11597_d_n11, assign12370_e11597_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1147 == 0.0)) {
        let assign12370_e11595: f64 = (locals.var_t2__blk810 / locals.var_powweffwr__blk1059);
        (assign12370_e11595, (((locals.var_t2__blk810_dn3 * locals.var_powweffwr__blk1059) - (locals.var_t2__blk810 * locals.var_powweffwr__blk1059_dn3)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t2__blk810_dn4 * locals.var_powweffwr__blk1059) - (locals.var_t2__blk810 * locals.var_powweffwr__blk1059_dn4)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t2__blk810_dn5 * locals.var_powweffwr__blk1059) - (locals.var_t2__blk810 * locals.var_powweffwr__blk1059_dn5)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t2__blk810_dn6 * locals.var_powweffwr__blk1059) - (locals.var_t2__blk810 * locals.var_powweffwr__blk1059_dn6)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t2__blk810_dn7 * locals.var_powweffwr__blk1059) - (locals.var_t2__blk810 * locals.var_powweffwr__blk1059_dn7)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t2__blk810_dn8 * locals.var_powweffwr__blk1059) - (locals.var_t2__blk810 * locals.var_powweffwr__blk1059_dn8)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t2__blk810_dn9 * locals.var_powweffwr__blk1059) - (locals.var_t2__blk810 * locals.var_powweffwr__blk1059_dn9)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t2__blk810_dn10 * locals.var_powweffwr__blk1059) - (locals.var_t2__blk810 * locals.var_powweffwr__blk1059_dn10)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t2__blk810_dn11 * locals.var_powweffwr__blk1059) - (locals.var_t2__blk810 * locals.var_powweffwr__blk1059_dn11)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t2__blk810_dn12 * locals.var_powweffwr__blk1059) - (locals.var_t2__blk810 * locals.var_powweffwr__blk1059_dn12)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)),)
    } else {
        (locals.var_rdwmin, locals.var_rdwmin_dn3, locals.var_rdwmin_dn4, locals.var_rdwmin_dn5, locals.var_rdwmin_dn6, locals.var_rdwmin_dn7, locals.var_rdwmin_dn8, locals.var_rdwmin_dn9, locals.var_rdwmin_dn10, locals.var_rdwmin_dn11, locals.var_rdwmin_dn12,)
    }
};
        locals.var_rdwmin = assign12370_e11597;
        locals.var_rdwmin_dn3 = assign12370_e11597_d_n3;
        locals.var_rdwmin_dn4 = assign12370_e11597_d_n4;
        locals.var_rdwmin_dn5 = assign12370_e11597_d_n5;
        locals.var_rdwmin_dn6 = assign12370_e11597_d_n6;
        locals.var_rdwmin_dn7 = assign12370_e11597_d_n7;
        locals.var_rdwmin_dn8 = assign12370_e11597_d_n8;
        locals.var_rdwmin_dn9 = assign12370_e11597_d_n9;
        locals.var_rdwmin_dn10 = assign12370_e11597_d_n10;
        locals.var_rdwmin_dn11 = assign12370_e11597_d_n11;
        locals.var_rdwmin_dn12 = assign12370_e11597_d_n12;

        let (assign12380_e11606, assign12380_e11606_d_n3, assign12380_e11606_d_n4, assign12380_e11606_d_n5, assign12380_e11606_d_n6, assign12380_e11606_d_n7, assign12380_e11606_d_n8, assign12380_e11606_d_n9, assign12380_e11606_d_n10, assign12380_e11606_d_n11, assign12380_e11606_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1147 == 0.0)) {
        let assign12380_e11604: f64 = (locals.var_pparam_b4soirsw + locals.var_t10__blk818);
        (assign12380_e11604, (locals.var_pparam_b4soirsw_dn3 + locals.var_t10__blk818_dn3), (locals.var_pparam_b4soirsw_dn4 + locals.var_t10__blk818_dn4), (locals.var_pparam_b4soirsw_dn5 + locals.var_t10__blk818_dn5), (locals.var_pparam_b4soirsw_dn6 + locals.var_t10__blk818_dn6), (locals.var_pparam_b4soirsw_dn7 + locals.var_t10__blk818_dn7), (locals.var_pparam_b4soirsw_dn8 + locals.var_t10__blk818_dn8), (locals.var_pparam_b4soirsw_dn9 + locals.var_t10__blk818_dn9), (locals.var_pparam_b4soirsw_dn10 + locals.var_t10__blk818_dn10), (locals.var_pparam_b4soirsw_dn11 + locals.var_t10__blk818_dn11), (locals.var_pparam_b4soirsw_dn12 + locals.var_t10__blk818_dn12),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign12380_e11606;
        locals.var_t7__blk815_dn3 = assign12380_e11606_d_n3;
        locals.var_t7__blk815_dn4 = assign12380_e11606_d_n4;
        locals.var_t7__blk815_dn5 = assign12380_e11606_d_n5;
        locals.var_t7__blk815_dn6 = assign12380_e11606_d_n6;
        locals.var_t7__blk815_dn7 = assign12380_e11606_d_n7;
        locals.var_t7__blk815_dn8 = assign12380_e11606_d_n8;
        locals.var_t7__blk815_dn9 = assign12380_e11606_d_n9;
        locals.var_t7__blk815_dn10 = assign12380_e11606_d_n10;
        locals.var_t7__blk815_dn11 = assign12380_e11606_d_n11;
        locals.var_t7__blk815_dn12 = assign12380_e11606_d_n12;

        let (assign12390_e11615, assign12390_e11615_d_n3, assign12390_e11615_d_n4, assign12390_e11615_d_n5, assign12390_e11615_d_n6, assign12390_e11615_d_n7, assign12390_e11615_d_n8, assign12390_e11615_d_n9, assign12390_e11615_d_n10, assign12390_e11615_d_n11, assign12390_e11615_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1147 == 0.0)) {
        let assign12390_e11613: f64 = (p.p139 + locals.var_t10__blk818);
        (assign12390_e11613, locals.var_t10__blk818_dn3, locals.var_t10__blk818_dn4, locals.var_t10__blk818_dn5, locals.var_t10__blk818_dn6, locals.var_t10__blk818_dn7, locals.var_t10__blk818_dn8, locals.var_t10__blk818_dn9, locals.var_t10__blk818_dn10, locals.var_t10__blk818_dn11, locals.var_t10__blk818_dn12,)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign12390_e11615;
        locals.var_t4__blk812_dn3 = assign12390_e11615_d_n3;
        locals.var_t4__blk812_dn4 = assign12390_e11615_d_n4;
        locals.var_t4__blk812_dn5 = assign12390_e11615_d_n5;
        locals.var_t4__blk812_dn6 = assign12390_e11615_d_n6;
        locals.var_t4__blk812_dn7 = assign12390_e11615_d_n7;
        locals.var_t4__blk812_dn8 = assign12390_e11615_d_n8;
        locals.var_t4__blk812_dn9 = assign12390_e11615_d_n9;
        locals.var_t4__blk812_dn10 = assign12390_e11615_d_n10;
        locals.var_t4__blk812_dn11 = assign12390_e11615_d_n11;
        locals.var_t4__blk812_dn12 = assign12390_e11615_d_n12;

        let (assign12400_e11624, assign12400_e11624_d_n3, assign12400_e11624_d_n4, assign12400_e11624_d_n5, assign12400_e11624_d_n6, assign12400_e11624_d_n7, assign12400_e11624_d_n8, assign12400_e11624_d_n9, assign12400_e11624_d_n10, assign12400_e11624_d_n11, assign12400_e11624_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1147 == 0.0)) {
        let assign12400_e11622: f64 = (locals.var_t7__blk815 / locals.var_powweffwr__blk1059);
        (assign12400_e11622, (((locals.var_t7__blk815_dn3 * locals.var_powweffwr__blk1059) - (locals.var_t7__blk815 * locals.var_powweffwr__blk1059_dn3)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t7__blk815_dn4 * locals.var_powweffwr__blk1059) - (locals.var_t7__blk815 * locals.var_powweffwr__blk1059_dn4)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t7__blk815_dn5 * locals.var_powweffwr__blk1059) - (locals.var_t7__blk815 * locals.var_powweffwr__blk1059_dn5)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t7__blk815_dn6 * locals.var_powweffwr__blk1059) - (locals.var_t7__blk815 * locals.var_powweffwr__blk1059_dn6)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t7__blk815_dn7 * locals.var_powweffwr__blk1059) - (locals.var_t7__blk815 * locals.var_powweffwr__blk1059_dn7)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t7__blk815_dn8 * locals.var_powweffwr__blk1059) - (locals.var_t7__blk815 * locals.var_powweffwr__blk1059_dn8)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t7__blk815_dn9 * locals.var_powweffwr__blk1059) - (locals.var_t7__blk815 * locals.var_powweffwr__blk1059_dn9)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t7__blk815_dn10 * locals.var_powweffwr__blk1059) - (locals.var_t7__blk815 * locals.var_powweffwr__blk1059_dn10)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t7__blk815_dn11 * locals.var_powweffwr__blk1059) - (locals.var_t7__blk815 * locals.var_powweffwr__blk1059_dn11)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t7__blk815_dn12 * locals.var_powweffwr__blk1059) - (locals.var_t7__blk815 * locals.var_powweffwr__blk1059_dn12)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)),)
    } else {
        (locals.var_rs0, locals.var_rs0_dn3, locals.var_rs0_dn4, locals.var_rs0_dn5, locals.var_rs0_dn6, locals.var_rs0_dn7, locals.var_rs0_dn8, locals.var_rs0_dn9, locals.var_rs0_dn10, locals.var_rs0_dn11, locals.var_rs0_dn12,)
    }
};
        locals.var_rs0 = assign12400_e11624;
        locals.var_rs0_dn3 = assign12400_e11624_d_n3;
        locals.var_rs0_dn4 = assign12400_e11624_d_n4;
        locals.var_rs0_dn5 = assign12400_e11624_d_n5;
        locals.var_rs0_dn6 = assign12400_e11624_d_n6;
        locals.var_rs0_dn7 = assign12400_e11624_d_n7;
        locals.var_rs0_dn8 = assign12400_e11624_d_n8;
        locals.var_rs0_dn9 = assign12400_e11624_d_n9;
        locals.var_rs0_dn10 = assign12400_e11624_d_n10;
        locals.var_rs0_dn11 = assign12400_e11624_d_n11;
        locals.var_rs0_dn12 = assign12400_e11624_d_n12;

        let (assign12410_e11633, assign12410_e11633_d_n3, assign12410_e11633_d_n4, assign12410_e11633_d_n5, assign12410_e11633_d_n6, assign12410_e11633_d_n7, assign12410_e11633_d_n8, assign12410_e11633_d_n9, assign12410_e11633_d_n10, assign12410_e11633_d_n11, assign12410_e11633_d_n12,) = {
    if ((locals.var_guard1124 != 0.0) && (locals.var_guard1147 == 0.0)) {
        let assign12410_e11631: f64 = (locals.var_t4__blk812 / locals.var_powweffwr__blk1059);
        (assign12410_e11631, (((locals.var_t4__blk812_dn3 * locals.var_powweffwr__blk1059) - (locals.var_t4__blk812 * locals.var_powweffwr__blk1059_dn3)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t4__blk812_dn4 * locals.var_powweffwr__blk1059) - (locals.var_t4__blk812 * locals.var_powweffwr__blk1059_dn4)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t4__blk812_dn5 * locals.var_powweffwr__blk1059) - (locals.var_t4__blk812 * locals.var_powweffwr__blk1059_dn5)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t4__blk812_dn6 * locals.var_powweffwr__blk1059) - (locals.var_t4__blk812 * locals.var_powweffwr__blk1059_dn6)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t4__blk812_dn7 * locals.var_powweffwr__blk1059) - (locals.var_t4__blk812 * locals.var_powweffwr__blk1059_dn7)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t4__blk812_dn8 * locals.var_powweffwr__blk1059) - (locals.var_t4__blk812 * locals.var_powweffwr__blk1059_dn8)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t4__blk812_dn9 * locals.var_powweffwr__blk1059) - (locals.var_t4__blk812 * locals.var_powweffwr__blk1059_dn9)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t4__blk812_dn10 * locals.var_powweffwr__blk1059) - (locals.var_t4__blk812 * locals.var_powweffwr__blk1059_dn10)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t4__blk812_dn11 * locals.var_powweffwr__blk1059) - (locals.var_t4__blk812 * locals.var_powweffwr__blk1059_dn11)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)), (((locals.var_t4__blk812_dn12 * locals.var_powweffwr__blk1059) - (locals.var_t4__blk812 * locals.var_powweffwr__blk1059_dn12)) / (locals.var_powweffwr__blk1059 * locals.var_powweffwr__blk1059)),)
    } else {
        (locals.var_rswmin, locals.var_rswmin_dn3, locals.var_rswmin_dn4, locals.var_rswmin_dn5, locals.var_rswmin_dn6, locals.var_rswmin_dn7, locals.var_rswmin_dn8, locals.var_rswmin_dn9, locals.var_rswmin_dn10, locals.var_rswmin_dn11, locals.var_rswmin_dn12,)
    }
};
        locals.var_rswmin = assign12410_e11633;
        locals.var_rswmin_dn3 = assign12410_e11633_d_n3;
        locals.var_rswmin_dn4 = assign12410_e11633_d_n4;
        locals.var_rswmin_dn5 = assign12410_e11633_d_n5;
        locals.var_rswmin_dn6 = assign12410_e11633_d_n6;
        locals.var_rswmin_dn7 = assign12410_e11633_d_n7;
        locals.var_rswmin_dn8 = assign12410_e11633_d_n8;
        locals.var_rswmin_dn9 = assign12410_e11633_d_n9;
        locals.var_rswmin_dn10 = assign12410_e11633_d_n10;
        locals.var_rswmin_dn11 = assign12410_e11633_d_n11;
        locals.var_rswmin_dn12 = assign12410_e11633_d_n12;

        let (assign12420_e11641, assign12420_e11641_d_n3, assign12420_e11641_d_n4, assign12420_e11641_d_n5, assign12420_e11641_d_n6, assign12420_e11641_d_n7, assign12420_e11641_d_n8, assign12420_e11641_d_n9, assign12420_e11641_d_n10, assign12420_e11641_d_n11, assign12420_e11641_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12420_e11638: f64 = (locals.var_pparam_b4soiua1 * locals.var_trm1);
        let assign12420_e11639: f64 = (locals.var_pparam_b4soiuatemp + assign12420_e11638);
        (assign12420_e11639, (locals.var_pparam_b4soiuatemp_dn3 + (locals.var_pparam_b4soiua1_dn3 * locals.var_trm1)), (locals.var_pparam_b4soiuatemp_dn4 + ((locals.var_pparam_b4soiua1_dn4 * locals.var_trm1) + (locals.var_pparam_b4soiua1 * locals.var_trm1_dn4))), (locals.var_pparam_b4soiuatemp_dn5 + ((locals.var_pparam_b4soiua1_dn5 * locals.var_trm1) + (locals.var_pparam_b4soiua1 * locals.var_trm1_dn5))), (locals.var_pparam_b4soiuatemp_dn6 + ((locals.var_pparam_b4soiua1_dn6 * locals.var_trm1) + (locals.var_pparam_b4soiua1 * locals.var_trm1_dn6))), (locals.var_pparam_b4soiuatemp_dn7 + (locals.var_pparam_b4soiua1_dn7 * locals.var_trm1)), (locals.var_pparam_b4soiuatemp_dn8 + (locals.var_pparam_b4soiua1_dn8 * locals.var_trm1)), (locals.var_pparam_b4soiuatemp_dn9 + (locals.var_pparam_b4soiua1_dn9 * locals.var_trm1)), (locals.var_pparam_b4soiuatemp_dn10 + (locals.var_pparam_b4soiua1_dn10 * locals.var_trm1)), (locals.var_pparam_b4soiuatemp_dn11 + (locals.var_pparam_b4soiua1_dn11 * locals.var_trm1)), (locals.var_pparam_b4soiuatemp_dn12 + (locals.var_pparam_b4soiua1_dn12 * locals.var_trm1)),)
    } else {
        (locals.var_ua, locals.var_ua_dn3, locals.var_ua_dn4, locals.var_ua_dn5, locals.var_ua_dn6, locals.var_ua_dn7, locals.var_ua_dn8, locals.var_ua_dn9, locals.var_ua_dn10, locals.var_ua_dn11, locals.var_ua_dn12,)
    }
};
        locals.var_ua = assign12420_e11641;
        locals.var_ua_dn3 = assign12420_e11641_d_n3;
        locals.var_ua_dn4 = assign12420_e11641_d_n4;
        locals.var_ua_dn5 = assign12420_e11641_d_n5;
        locals.var_ua_dn6 = assign12420_e11641_d_n6;
        locals.var_ua_dn7 = assign12420_e11641_d_n7;
        locals.var_ua_dn8 = assign12420_e11641_d_n8;
        locals.var_ua_dn9 = assign12420_e11641_d_n9;
        locals.var_ua_dn10 = assign12420_e11641_d_n10;
        locals.var_ua_dn11 = assign12420_e11641_d_n11;
        locals.var_ua_dn12 = assign12420_e11641_d_n12;

        let (assign12430_e11649, assign12430_e11649_d_n3, assign12430_e11649_d_n4, assign12430_e11649_d_n5, assign12430_e11649_d_n6, assign12430_e11649_d_n7, assign12430_e11649_d_n8, assign12430_e11649_d_n9, assign12430_e11649_d_n10, assign12430_e11649_d_n11, assign12430_e11649_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12430_e11646: f64 = (locals.var_pparam_b4soiub1 * locals.var_trm1);
        let assign12430_e11647: f64 = (locals.var_pparam_b4soiubtemp + assign12430_e11646);
        (assign12430_e11647, (locals.var_pparam_b4soiubtemp_dn3 + (locals.var_pparam_b4soiub1_dn3 * locals.var_trm1)), (locals.var_pparam_b4soiubtemp_dn4 + ((locals.var_pparam_b4soiub1_dn4 * locals.var_trm1) + (locals.var_pparam_b4soiub1 * locals.var_trm1_dn4))), (locals.var_pparam_b4soiubtemp_dn5 + ((locals.var_pparam_b4soiub1_dn5 * locals.var_trm1) + (locals.var_pparam_b4soiub1 * locals.var_trm1_dn5))), (locals.var_pparam_b4soiubtemp_dn6 + ((locals.var_pparam_b4soiub1_dn6 * locals.var_trm1) + (locals.var_pparam_b4soiub1 * locals.var_trm1_dn6))), (locals.var_pparam_b4soiubtemp_dn7 + (locals.var_pparam_b4soiub1_dn7 * locals.var_trm1)), (locals.var_pparam_b4soiubtemp_dn8 + (locals.var_pparam_b4soiub1_dn8 * locals.var_trm1)), (locals.var_pparam_b4soiubtemp_dn9 + (locals.var_pparam_b4soiub1_dn9 * locals.var_trm1)), (locals.var_pparam_b4soiubtemp_dn10 + (locals.var_pparam_b4soiub1_dn10 * locals.var_trm1)), (locals.var_pparam_b4soiubtemp_dn11 + (locals.var_pparam_b4soiub1_dn11 * locals.var_trm1)), (locals.var_pparam_b4soiubtemp_dn12 + (locals.var_pparam_b4soiub1_dn12 * locals.var_trm1)),)
    } else {
        (locals.var_ub, locals.var_ub_dn3, locals.var_ub_dn4, locals.var_ub_dn5, locals.var_ub_dn6, locals.var_ub_dn7, locals.var_ub_dn8, locals.var_ub_dn9, locals.var_ub_dn10, locals.var_ub_dn11, locals.var_ub_dn12,)
    }
};
        locals.var_ub = assign12430_e11649;
        locals.var_ub_dn3 = assign12430_e11649_d_n3;
        locals.var_ub_dn4 = assign12430_e11649_d_n4;
        locals.var_ub_dn5 = assign12430_e11649_d_n5;
        locals.var_ub_dn6 = assign12430_e11649_d_n6;
        locals.var_ub_dn7 = assign12430_e11649_d_n7;
        locals.var_ub_dn8 = assign12430_e11649_d_n8;
        locals.var_ub_dn9 = assign12430_e11649_d_n9;
        locals.var_ub_dn10 = assign12430_e11649_d_n10;
        locals.var_ub_dn11 = assign12430_e11649_d_n11;
        locals.var_ub_dn12 = assign12430_e11649_d_n12;

        let (assign12440_e11657, assign12440_e11657_d_n3, assign12440_e11657_d_n4, assign12440_e11657_d_n5, assign12440_e11657_d_n6, assign12440_e11657_d_n7, assign12440_e11657_d_n8, assign12440_e11657_d_n9, assign12440_e11657_d_n10, assign12440_e11657_d_n11, assign12440_e11657_d_n12,) = {
    if (locals.var_guard1124 != 0.0) {
        let assign12440_e11654: f64 = (locals.var_pparam_b4soiuc1 * locals.var_trm1);
        let assign12440_e11655: f64 = (locals.var_pparam_b4soiuctemp + assign12440_e11654);
        (assign12440_e11655, (locals.var_pparam_b4soiuctemp_dn3 + (locals.var_pparam_b4soiuc1_dn3 * locals.var_trm1)), (locals.var_pparam_b4soiuctemp_dn4 + ((locals.var_pparam_b4soiuc1_dn4 * locals.var_trm1) + (locals.var_pparam_b4soiuc1 * locals.var_trm1_dn4))), (locals.var_pparam_b4soiuctemp_dn5 + ((locals.var_pparam_b4soiuc1_dn5 * locals.var_trm1) + (locals.var_pparam_b4soiuc1 * locals.var_trm1_dn5))), (locals.var_pparam_b4soiuctemp_dn6 + ((locals.var_pparam_b4soiuc1_dn6 * locals.var_trm1) + (locals.var_pparam_b4soiuc1 * locals.var_trm1_dn6))), (locals.var_pparam_b4soiuctemp_dn7 + (locals.var_pparam_b4soiuc1_dn7 * locals.var_trm1)), (locals.var_pparam_b4soiuctemp_dn8 + (locals.var_pparam_b4soiuc1_dn8 * locals.var_trm1)), (locals.var_pparam_b4soiuctemp_dn9 + (locals.var_pparam_b4soiuc1_dn9 * locals.var_trm1)), (locals.var_pparam_b4soiuctemp_dn10 + (locals.var_pparam_b4soiuc1_dn10 * locals.var_trm1)), (locals.var_pparam_b4soiuctemp_dn11 + (locals.var_pparam_b4soiuc1_dn11 * locals.var_trm1)), (locals.var_pparam_b4soiuctemp_dn12 + (locals.var_pparam_b4soiuc1_dn12 * locals.var_trm1)),)
    } else {
        (locals.var_uc, locals.var_uc_dn3, locals.var_uc_dn4, locals.var_uc_dn5, locals.var_uc_dn6, locals.var_uc_dn7, locals.var_uc_dn8, locals.var_uc_dn9, locals.var_uc_dn10, locals.var_uc_dn11, locals.var_uc_dn12,)
    }
};
        locals.var_uc = assign12440_e11657;
        locals.var_uc_dn3 = assign12440_e11657_d_n3;
        locals.var_uc_dn4 = assign12440_e11657_d_n4;
        locals.var_uc_dn5 = assign12440_e11657_d_n5;
        locals.var_uc_dn6 = assign12440_e11657_d_n6;
        locals.var_uc_dn7 = assign12440_e11657_d_n7;
        locals.var_uc_dn8 = assign12440_e11657_d_n8;
        locals.var_uc_dn9 = assign12440_e11657_d_n9;
        locals.var_uc_dn10 = assign12440_e11657_d_n10;
        locals.var_uc_dn11 = assign12440_e11657_d_n11;
        locals.var_uc_dn12 = assign12440_e11657_d_n12;

        let (assign12450_e11662, assign12450_e11662_d_n3, assign12450_e11662_d_n4, assign12450_e11662_d_n5, assign12450_e11662_d_n6, assign12450_e11662_d_n7, assign12450_e11662_d_n8, assign12450_e11662_d_n9, assign12450_e11662_d_n10, assign12450_e11662_d_n11, assign12450_e11662_d_n12,) = {
    if (locals.var_guard1124 == 0.0) {
        (locals.var_pparam_b4soivbi, locals.var_pparam_b4soivbi_dn3, locals.var_pparam_b4soivbi_dn4, locals.var_pparam_b4soivbi_dn5, locals.var_pparam_b4soivbi_dn6, locals.var_pparam_b4soivbi_dn7, locals.var_pparam_b4soivbi_dn8, locals.var_pparam_b4soivbi_dn9, locals.var_pparam_b4soivbi_dn10, locals.var_pparam_b4soivbi_dn11, locals.var_pparam_b4soivbi_dn12,)
    } else {
        (locals.var_vbi, locals.var_vbi_dn3, locals.var_vbi_dn4, locals.var_vbi_dn5, locals.var_vbi_dn6, locals.var_vbi_dn7, locals.var_vbi_dn8, locals.var_vbi_dn9, locals.var_vbi_dn10, locals.var_vbi_dn11, locals.var_vbi_dn12,)
    }
};
        locals.var_vbi = assign12450_e11662;
        locals.var_vbi_dn3 = assign12450_e11662_d_n3;
        locals.var_vbi_dn4 = assign12450_e11662_d_n4;
        locals.var_vbi_dn5 = assign12450_e11662_d_n5;
        locals.var_vbi_dn6 = assign12450_e11662_d_n6;
        locals.var_vbi_dn7 = assign12450_e11662_d_n7;
        locals.var_vbi_dn8 = assign12450_e11662_d_n8;
        locals.var_vbi_dn9 = assign12450_e11662_d_n9;
        locals.var_vbi_dn10 = assign12450_e11662_d_n10;
        locals.var_vbi_dn11 = assign12450_e11662_d_n11;
        locals.var_vbi_dn12 = assign12450_e11662_d_n12;

        let (assign12460_e11667, assign12460_e11667_d_n3, assign12460_e11667_d_n4, assign12460_e11667_d_n5, assign12460_e11667_d_n6, assign12460_e11667_d_n7, assign12460_e11667_d_n8, assign12460_e11667_d_n9, assign12460_e11667_d_n10, assign12460_e11667_d_n11, assign12460_e11667_d_n12,) = {
    if (locals.var_guard1124 == 0.0) {
        (locals.var_pparam_b4soivfbb, locals.var_pparam_b4soivfbb_dn3, locals.var_pparam_b4soivfbb_dn4, locals.var_pparam_b4soivfbb_dn5, locals.var_pparam_b4soivfbb_dn6, locals.var_pparam_b4soivfbb_dn7, locals.var_pparam_b4soivfbb_dn8, locals.var_pparam_b4soivfbb_dn9, locals.var_pparam_b4soivfbb_dn10, locals.var_pparam_b4soivfbb_dn11, locals.var_pparam_b4soivfbb_dn12,)
    } else {
        (locals.var_vfbb, locals.var_vfbb_dn3, locals.var_vfbb_dn4, locals.var_vfbb_dn5, locals.var_vfbb_dn6, locals.var_vfbb_dn7, locals.var_vfbb_dn8, locals.var_vfbb_dn9, locals.var_vfbb_dn10, locals.var_vfbb_dn11, locals.var_vfbb_dn12,)
    }
};
        locals.var_vfbb = assign12460_e11667;
        locals.var_vfbb_dn3 = assign12460_e11667_d_n3;
        locals.var_vfbb_dn4 = assign12460_e11667_d_n4;
        locals.var_vfbb_dn5 = assign12460_e11667_d_n5;
        locals.var_vfbb_dn6 = assign12460_e11667_d_n6;
        locals.var_vfbb_dn7 = assign12460_e11667_d_n7;
        locals.var_vfbb_dn8 = assign12460_e11667_d_n8;
        locals.var_vfbb_dn9 = assign12460_e11667_d_n9;
        locals.var_vfbb_dn10 = assign12460_e11667_d_n10;
        locals.var_vfbb_dn11 = assign12460_e11667_d_n11;
        locals.var_vfbb_dn12 = assign12460_e11667_d_n12;

        let (assign12470_e11672, assign12470_e11672_d_n3, assign12470_e11672_d_n4, assign12470_e11672_d_n5, assign12470_e11672_d_n6, assign12470_e11672_d_n7, assign12470_e11672_d_n8, assign12470_e11672_d_n9, assign12470_e11672_d_n10, assign12470_e11672_d_n11, assign12470_e11672_d_n12,) = {
    if (locals.var_guard1124 == 0.0) {
        (locals.var_pparam_b4soiphi, locals.var_pparam_b4soiphi_dn3, locals.var_pparam_b4soiphi_dn4, locals.var_pparam_b4soiphi_dn5, locals.var_pparam_b4soiphi_dn6, locals.var_pparam_b4soiphi_dn7, locals.var_pparam_b4soiphi_dn8, locals.var_pparam_b4soiphi_dn9, locals.var_pparam_b4soiphi_dn10, locals.var_pparam_b4soiphi_dn11, locals.var_pparam_b4soiphi_dn12,)
    } else {
        (locals.var_phi, locals.var_phi_dn3, locals.var_phi_dn4, locals.var_phi_dn5, locals.var_phi_dn6, locals.var_phi_dn7, locals.var_phi_dn8, locals.var_phi_dn9, locals.var_phi_dn10, locals.var_phi_dn11, locals.var_phi_dn12,)
    }
};
        locals.var_phi = assign12470_e11672;
        locals.var_phi_dn3 = assign12470_e11672_d_n3;
        locals.var_phi_dn4 = assign12470_e11672_d_n4;
        locals.var_phi_dn5 = assign12470_e11672_d_n5;
        locals.var_phi_dn6 = assign12470_e11672_d_n6;
        locals.var_phi_dn7 = assign12470_e11672_d_n7;
        locals.var_phi_dn8 = assign12470_e11672_d_n8;
        locals.var_phi_dn9 = assign12470_e11672_d_n9;
        locals.var_phi_dn10 = assign12470_e11672_d_n10;
        locals.var_phi_dn11 = assign12470_e11672_d_n11;
        locals.var_phi_dn12 = assign12470_e11672_d_n12;

        let (assign12480_e11677, assign12480_e11677_d_n3, assign12480_e11677_d_n4, assign12480_e11677_d_n5, assign12480_e11677_d_n6, assign12480_e11677_d_n7, assign12480_e11677_d_n8, assign12480_e11677_d_n9, assign12480_e11677_d_n10, assign12480_e11677_d_n11, assign12480_e11677_d_n12,) = {
    if (locals.var_guard1124 == 0.0) {
        (locals.var_pparam_b4soisqrtphi, locals.var_pparam_b4soisqrtphi_dn3, locals.var_pparam_b4soisqrtphi_dn4, locals.var_pparam_b4soisqrtphi_dn5, locals.var_pparam_b4soisqrtphi_dn6, locals.var_pparam_b4soisqrtphi_dn7, locals.var_pparam_b4soisqrtphi_dn8, locals.var_pparam_b4soisqrtphi_dn9, locals.var_pparam_b4soisqrtphi_dn10, locals.var_pparam_b4soisqrtphi_dn11, locals.var_pparam_b4soisqrtphi_dn12,)
    } else {
        (locals.var_sqrtphi, locals.var_sqrtphi_dn3, locals.var_sqrtphi_dn4, locals.var_sqrtphi_dn5, locals.var_sqrtphi_dn6, locals.var_sqrtphi_dn7, locals.var_sqrtphi_dn8, locals.var_sqrtphi_dn9, locals.var_sqrtphi_dn10, locals.var_sqrtphi_dn11, locals.var_sqrtphi_dn12,)
    }
};
        locals.var_sqrtphi = assign12480_e11677;
        locals.var_sqrtphi_dn3 = assign12480_e11677_d_n3;
        locals.var_sqrtphi_dn4 = assign12480_e11677_d_n4;
        locals.var_sqrtphi_dn5 = assign12480_e11677_d_n5;
        locals.var_sqrtphi_dn6 = assign12480_e11677_d_n6;
        locals.var_sqrtphi_dn7 = assign12480_e11677_d_n7;
        locals.var_sqrtphi_dn8 = assign12480_e11677_d_n8;
        locals.var_sqrtphi_dn9 = assign12480_e11677_d_n9;
        locals.var_sqrtphi_dn10 = assign12480_e11677_d_n10;
        locals.var_sqrtphi_dn11 = assign12480_e11677_d_n11;
        locals.var_sqrtphi_dn12 = assign12480_e11677_d_n12;

        let (assign12490_e11682, assign12490_e11682_d_n3, assign12490_e11682_d_n4, assign12490_e11682_d_n5, assign12490_e11682_d_n6, assign12490_e11682_d_n7, assign12490_e11682_d_n8, assign12490_e11682_d_n9, assign12490_e11682_d_n10, assign12490_e11682_d_n11, assign12490_e11682_d_n12,) = {
    if (locals.var_guard1124 == 0.0) {
        (locals.var_pparam_b4soixdep0, locals.var_pparam_b4soixdep0_dn3, locals.var_pparam_b4soixdep0_dn4, locals.var_pparam_b4soixdep0_dn5, locals.var_pparam_b4soixdep0_dn6, locals.var_pparam_b4soixdep0_dn7, locals.var_pparam_b4soixdep0_dn8, locals.var_pparam_b4soixdep0_dn9, locals.var_pparam_b4soixdep0_dn10, locals.var_pparam_b4soixdep0_dn11, locals.var_pparam_b4soixdep0_dn12,)
    } else {
        (locals.var_xdep0, locals.var_xdep0_dn3, locals.var_xdep0_dn4, locals.var_xdep0_dn5, locals.var_xdep0_dn6, locals.var_xdep0_dn7, locals.var_xdep0_dn8, locals.var_xdep0_dn9, locals.var_xdep0_dn10, locals.var_xdep0_dn11, locals.var_xdep0_dn12,)
    }
};
        locals.var_xdep0 = assign12490_e11682;
        locals.var_xdep0_dn3 = assign12490_e11682_d_n3;
        locals.var_xdep0_dn4 = assign12490_e11682_d_n4;
        locals.var_xdep0_dn5 = assign12490_e11682_d_n5;
        locals.var_xdep0_dn6 = assign12490_e11682_d_n6;
        locals.var_xdep0_dn7 = assign12490_e11682_d_n7;
        locals.var_xdep0_dn8 = assign12490_e11682_d_n8;
        locals.var_xdep0_dn9 = assign12490_e11682_d_n9;
        locals.var_xdep0_dn10 = assign12490_e11682_d_n10;
        locals.var_xdep0_dn11 = assign12490_e11682_d_n11;
        locals.var_xdep0_dn12 = assign12490_e11682_d_n12;

        let (assign12500_e11687, assign12500_e11687_d_n3, assign12500_e11687_d_n4, assign12500_e11687_d_n5, assign12500_e11687_d_n6, assign12500_e11687_d_n7, assign12500_e11687_d_n8, assign12500_e11687_d_n9, assign12500_e11687_d_n10, assign12500_e11687_d_n11, assign12500_e11687_d_n12,) = {
    if (locals.var_guard1124 == 0.0) {
        (locals.var_b4soieg, 0.0, locals.var_b4soieg_dn4, locals.var_b4soieg_dn5, locals.var_b4soieg_dn6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_eg__blk877, locals.var_eg__blk877_dn3, locals.var_eg__blk877_dn4, locals.var_eg__blk877_dn5, locals.var_eg__blk877_dn6, locals.var_eg__blk877_dn7, locals.var_eg__blk877_dn8, locals.var_eg__blk877_dn9, locals.var_eg__blk877_dn10, locals.var_eg__blk877_dn11, locals.var_eg__blk877_dn12,)
    }
};
        locals.var_eg__blk877 = assign12500_e11687;
        locals.var_eg__blk877_dn3 = assign12500_e11687_d_n3;
        locals.var_eg__blk877_dn4 = assign12500_e11687_d_n4;
        locals.var_eg__blk877_dn5 = assign12500_e11687_d_n5;
        locals.var_eg__blk877_dn6 = assign12500_e11687_d_n6;
        locals.var_eg__blk877_dn7 = assign12500_e11687_d_n7;
        locals.var_eg__blk877_dn8 = assign12500_e11687_d_n8;
        locals.var_eg__blk877_dn9 = assign12500_e11687_d_n9;
        locals.var_eg__blk877_dn10 = assign12500_e11687_d_n10;
        locals.var_eg__blk877_dn11 = assign12500_e11687_d_n11;
        locals.var_eg__blk877_dn12 = assign12500_e11687_d_n12;

        let (assign12510_e11692, assign12510_e11692_d_n3, assign12510_e11692_d_n4, assign12510_e11692_d_n5, assign12510_e11692_d_n6, assign12510_e11692_d_n7, assign12510_e11692_d_n8, assign12510_e11692_d_n9, assign12510_e11692_d_n10, assign12510_e11692_d_n11, assign12510_e11692_d_n12,) = {
    if (locals.var_guard1124 == 0.0) {
        (locals.var_pparam_b4soicdep0, locals.var_pparam_b4soicdep0_dn3, locals.var_pparam_b4soicdep0_dn4, locals.var_pparam_b4soicdep0_dn5, locals.var_pparam_b4soicdep0_dn6, locals.var_pparam_b4soicdep0_dn7, locals.var_pparam_b4soicdep0_dn8, locals.var_pparam_b4soicdep0_dn9, locals.var_pparam_b4soicdep0_dn10, locals.var_pparam_b4soicdep0_dn11, locals.var_pparam_b4soicdep0_dn12,)
    } else {
        (locals.var_cdep0, locals.var_cdep0_dn3, locals.var_cdep0_dn4, locals.var_cdep0_dn5, locals.var_cdep0_dn6, locals.var_cdep0_dn7, locals.var_cdep0_dn8, locals.var_cdep0_dn9, locals.var_cdep0_dn10, locals.var_cdep0_dn11, locals.var_cdep0_dn12,)
    }
};
        locals.var_cdep0 = assign12510_e11692;
        locals.var_cdep0_dn3 = assign12510_e11692_d_n3;
        locals.var_cdep0_dn4 = assign12510_e11692_d_n4;
        locals.var_cdep0_dn5 = assign12510_e11692_d_n5;
        locals.var_cdep0_dn6 = assign12510_e11692_d_n6;
        locals.var_cdep0_dn7 = assign12510_e11692_d_n7;
        locals.var_cdep0_dn8 = assign12510_e11692_d_n8;
        locals.var_cdep0_dn9 = assign12510_e11692_d_n9;
        locals.var_cdep0_dn10 = assign12510_e11692_d_n10;
        locals.var_cdep0_dn11 = assign12510_e11692_d_n11;
        locals.var_cdep0_dn12 = assign12510_e11692_d_n12;

        let (assign12520_e11697, assign12520_e11697_d_n3, assign12520_e11697_d_n4, assign12520_e11697_d_n5, assign12520_e11697_d_n6, assign12520_e11697_d_n7, assign12520_e11697_d_n8, assign12520_e11697_d_n9, assign12520_e11697_d_n10, assign12520_e11697_d_n11, assign12520_e11697_d_n12,) = {
    if (locals.var_guard1124 == 0.0) {
        (locals.var_pparam_b4soitheta0vb0, locals.var_pparam_b4soitheta0vb0_dn3, locals.var_pparam_b4soitheta0vb0_dn4, locals.var_pparam_b4soitheta0vb0_dn5, locals.var_pparam_b4soitheta0vb0_dn6, locals.var_pparam_b4soitheta0vb0_dn7, locals.var_pparam_b4soitheta0vb0_dn8, locals.var_pparam_b4soitheta0vb0_dn9, locals.var_pparam_b4soitheta0vb0_dn10, locals.var_pparam_b4soitheta0vb0_dn11, locals.var_pparam_b4soitheta0vb0_dn12,)
    } else {
        (locals.var_theta0vb0, locals.var_theta0vb0_dn3, locals.var_theta0vb0_dn4, locals.var_theta0vb0_dn5, locals.var_theta0vb0_dn6, locals.var_theta0vb0_dn7, locals.var_theta0vb0_dn8, locals.var_theta0vb0_dn9, locals.var_theta0vb0_dn10, locals.var_theta0vb0_dn11, locals.var_theta0vb0_dn12,)
    }
};
        locals.var_theta0vb0 = assign12520_e11697;
        locals.var_theta0vb0_dn3 = assign12520_e11697_d_n3;
        locals.var_theta0vb0_dn4 = assign12520_e11697_d_n4;
        locals.var_theta0vb0_dn5 = assign12520_e11697_d_n5;
        locals.var_theta0vb0_dn6 = assign12520_e11697_d_n6;
        locals.var_theta0vb0_dn7 = assign12520_e11697_d_n7;
        locals.var_theta0vb0_dn8 = assign12520_e11697_d_n8;
        locals.var_theta0vb0_dn9 = assign12520_e11697_d_n9;
        locals.var_theta0vb0_dn10 = assign12520_e11697_d_n10;
        locals.var_theta0vb0_dn11 = assign12520_e11697_d_n11;
        locals.var_theta0vb0_dn12 = assign12520_e11697_d_n12;

    }
}

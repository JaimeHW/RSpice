#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_48(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign20170_e14986: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard409 = assign20170_e14986;

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

        let assign20240_e15046: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard410 = assign20240_e15046;

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

    }

    pub(super) fn stamp_transient_block_49(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign20380_e15166: f64 = if ((locals.var_flg_rsrd == 1.0) || (locals.var_flg_rsrd == 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard411 = assign20380_e15166;

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

        let assign20420_e15181: f64 = if locals.var_vdsegmt >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard412 = assign20420_e15181;

        let (assign20430_e15187,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard412 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_vdsemodenml,)
    }
};
        locals.var_vdsemodenml = assign20430_e15187;

        let (assign20440_e15193,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard412 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_vdsemodervs,)
    }
};
        locals.var_vdsemodervs = assign20440_e15193;

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

        let (assign20490_e15224,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard412 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_vdsemodenml,)
    }
};
        locals.var_vdsemodenml = assign20490_e15224;

        let (assign20500_e15231,) = {
    if ((locals.var_guard411 != 0.0) && (locals.var_guard412 == 0.0)) {
        (1.0,)
    } else {
        (locals.var_vdsemodervs,)
    }
};
        locals.var_vdsemodervs = assign20500_e15231;

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

        let assign20550_e15285: f64 = if (((((locals.var_rdvde > 0.0) || (locals.var_rsvde > 0.0)) || (locals.var_uc_rdvg11 > 0.0)) || (locals.var_uc_rdvb > 0.0)) || (p.p54 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard413 = assign20550_e15285;

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

        let assign20610_e15401: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard414 = assign20610_e15401;

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

        let assign20660_e15442: f64 = if ((p.p34 == 1.0) || (locals.var_vdsemodenml == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard415 = assign20660_e15442;

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

    }

    pub(super) fn stamp_transient_block_50(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign20730_e15532: f64 = if locals.var_t10 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard416 = assign20730_e15532;

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

    }

    pub(super) fn stamp_transient_block_51(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign20960_e15850: f64 = if locals.var_t3 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard417 = assign20960_e15850;

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

        let assign21040_e15926: f64 = if ((p.p34 == 1.0) || (locals.var_vdsemodervs == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard418 = assign21040_e15926;

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

    }

    pub(super) fn stamp_transient_block_52(
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

        let assign21280_e16262: f64 = if locals.var_t3 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard419 = assign21280_e16262;

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

        let assign21350_e16330: f64 = (locals.var_mks_nsubsub + locals.var_uc_nover);
        let assign21350_e16331: f64 = (locals.var_uc_nover * assign21350_e16330);
        let assign21350_e16334: f64 = if (((p.p54 == 1.0) && (p.p34 == 0.0)) && (assign21350_e16331 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard420 = assign21350_e16334;

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

        let assign21400_e16396: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard421 = assign21400_e16396;

    }

    pub(super) fn stamp_transient_block_53(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

    }

    pub(super) fn stamp_transient_block_54(
        locals: &mut StampLocals,
    ) {
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

        let assign21750_e16762: f64 = if locals.var_vbs > locals.var_vbs_bnd { 1.0 } else { 0.0 };
        locals.var_guard424 = assign21750_e16762;

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

    }

    pub(super) fn stamp_transient_block_55(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
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

        let assign22010_e17020: f64 = if locals.var_vzadd < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard425 = assign22010_e17020;

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

        let assign22120_e17081: f64 = if locals.var_t4 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard426 = assign22120_e17081;

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

        let assign22160_e17094: f64 = (locals.var_t4).sqrt();
        locals.var_tx = assign22160_e17094;
        locals.var_tx_dn0 = (locals.var_t4_dn0 / (2.0 * assign22160_e17094));
        locals.var_tx_dn2 = (locals.var_t4_dn2 / (2.0 * assign22160_e17094));
        locals.var_tx_dn4 = (locals.var_t4_dn4 / (2.0 * assign22160_e17094));
        locals.var_tx_dn5 = (locals.var_t4_dn5 / (2.0 * assign22160_e17094));
        locals.var_tx_dn6 = (locals.var_t4_dn6 / (2.0 * assign22160_e17094));
        locals.var_tx_dn7 = (locals.var_t4_dn7 / (2.0 * assign22160_e17094));
        locals.var_tx_dn8 = (locals.var_t4_dn8 / (2.0 * assign22160_e17094));
        locals.var_tx_dn9 = (locals.var_t4_dn9 / (2.0 * assign22160_e17094));
        locals.var_tx_dn10 = (locals.var_t4_dn10 / (2.0 * assign22160_e17094));
        locals.var_tx_dn11 = (locals.var_t4_dn11 / (2.0 * assign22160_e17094));
        locals.var_tx_dn14 = (locals.var_t4_dn14 / (2.0 * assign22160_e17094));

        let assign22170_e17099: f64 = (1.0 - locals.var_tx);
        let assign22170_e17100: f64 = (locals.var_t1 * assign22170_e17099);
        let assign22170_e17101: f64 = (locals.var_t2 + assign22170_e17100);
        locals.var_pslsat = assign22170_e17101;
        locals.var_pslsat_dn0 = (locals.var_t2_dn0 + ((locals.var_t1_dn0 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn0))));
        locals.var_pslsat_dn2 = (locals.var_t2_dn2 + ((locals.var_t1_dn2 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn2))));
        locals.var_pslsat_dn4 = (locals.var_t2_dn4 + ((locals.var_t1_dn4 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn4))));
        locals.var_pslsat_dn5 = (locals.var_t2_dn5 + ((locals.var_t1_dn5 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn5))));
        locals.var_pslsat_dn6 = (locals.var_t2_dn6 + ((locals.var_t1_dn6 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn6))));
        locals.var_pslsat_dn7 = (locals.var_t2_dn7 + ((locals.var_t1_dn7 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn7))));
        locals.var_pslsat_dn8 = (locals.var_t2_dn8 + ((locals.var_t1_dn8 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn8))));
        locals.var_pslsat_dn9 = (locals.var_t2_dn9 + ((locals.var_t1_dn9 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn9))));
        locals.var_pslsat_dn10 = (locals.var_t2_dn10 + ((locals.var_t1_dn10 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn10))));
        locals.var_pslsat_dn11 = (locals.var_t2_dn11 + ((locals.var_t1_dn11 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn11))));
        locals.var_pslsat_dn14 = (locals.var_t2_dn14 + ((locals.var_t1_dn14 * assign22170_e17099) + (locals.var_t1 * (-locals.var_tx_dn14))));

        let assign22180_e17104: f64 = (locals.var_pslsat - locals.var_pb2c);
        locals.var_vdsats = assign22180_e17104;
        locals.var_vdsats_dn0 = (locals.var_pslsat_dn0 - locals.var_pb2c_dn0);
        locals.var_vdsats_dn2 = (locals.var_pslsat_dn2 - locals.var_pb2c_dn2);
        locals.var_vdsats_dn4 = (locals.var_pslsat_dn4 - locals.var_pb2c_dn4);
        locals.var_vdsats_dn5 = (locals.var_pslsat_dn5 - locals.var_pb2c_dn5);
        locals.var_vdsats_dn6 = (locals.var_pslsat_dn6 - locals.var_pb2c_dn6);
        locals.var_vdsats_dn7 = (locals.var_pslsat_dn7 - locals.var_pb2c_dn7);
        locals.var_vdsats_dn8 = (locals.var_pslsat_dn8 - locals.var_pb2c_dn8);
        locals.var_vdsats_dn9 = (locals.var_pslsat_dn9 - locals.var_pb2c_dn9);
        locals.var_vdsats_dn10 = (locals.var_pslsat_dn10 - locals.var_pb2c_dn10);
        locals.var_vdsats_dn11 = (locals.var_pslsat_dn11 - locals.var_pb2c_dn11);
        locals.var_vdsats_dn14 = (locals.var_pslsat_dn14 - locals.var_pb2c_dn14);

        let assign22190_e17107: f64 = (locals.var_vdsats - 0.1);
        let assign22190_e17109: f64 = (assign22190_e17107 - 0.05);
        locals.var_tmf1 = assign22190_e17109;
        locals.var_tmf1_dn0 = locals.var_vdsats_dn0;
        locals.var_tmf1_dn2 = locals.var_vdsats_dn2;
        locals.var_tmf1_dn4 = locals.var_vdsats_dn4;
        locals.var_tmf1_dn5 = locals.var_vdsats_dn5;
        locals.var_tmf1_dn6 = locals.var_vdsats_dn6;
        locals.var_tmf1_dn7 = locals.var_vdsats_dn7;
        locals.var_tmf1_dn8 = locals.var_vdsats_dn8;
        locals.var_tmf1_dn9 = locals.var_vdsats_dn9;
        locals.var_tmf1_dn10 = locals.var_vdsats_dn10;
        locals.var_tmf1_dn11 = locals.var_vdsats_dn11;
        locals.var_tmf1_dn14 = locals.var_vdsats_dn14;

        let assign22200_e17112: f64 = (4.0 * 0.1);
        let assign22200_e17114: f64 = (assign22200_e17112 * 0.05);
        locals.var_tmf2 = assign22200_e17114;
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

        let (assign22210_e17121, assign22210_e17121_d_n0, assign22210_e17121_d_n2, assign22210_e17121_d_n4, assign22210_e17121_d_n5, assign22210_e17121_d_n6, assign22210_e17121_d_n7, assign22210_e17121_d_n8, assign22210_e17121_d_n9, assign22210_e17121_d_n10, assign22210_e17121_d_n11, assign22210_e17121_d_n14,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    } else {
        let assign22210_e17120: f64 = (-locals.var_tmf2);
        (assign22210_e17120, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
    }
};
        locals.var_tmf2 = assign22210_e17121;
        locals.var_tmf2_dn0 = assign22210_e17121_d_n0;
        locals.var_tmf2_dn2 = assign22210_e17121_d_n2;
        locals.var_tmf2_dn4 = assign22210_e17121_d_n4;
        locals.var_tmf2_dn5 = assign22210_e17121_d_n5;
        locals.var_tmf2_dn6 = assign22210_e17121_d_n6;
        locals.var_tmf2_dn7 = assign22210_e17121_d_n7;
        locals.var_tmf2_dn8 = assign22210_e17121_d_n8;
        locals.var_tmf2_dn9 = assign22210_e17121_d_n9;
        locals.var_tmf2_dn10 = assign22210_e17121_d_n10;
        locals.var_tmf2_dn11 = assign22210_e17121_d_n11;
        locals.var_tmf2_dn14 = assign22210_e17121_d_n14;

        let assign22220_e17124: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign22220_e17126: f64 = (assign22220_e17124 + locals.var_tmf2);
        let assign22220_e17127: f64 = (assign22220_e17126).sqrt();
        locals.var_tmf2 = assign22220_e17127;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign22220_e17127));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign22220_e17127));
        locals.var_tmf2_dn4 = ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign22220_e17127));
        locals.var_tmf2_dn5 = ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign22220_e17127));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign22220_e17127));
        locals.var_tmf2_dn7 = ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign22220_e17127));
        locals.var_tmf2_dn8 = ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign22220_e17127));
        locals.var_tmf2_dn9 = ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign22220_e17127));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign22220_e17127));
        locals.var_tmf2_dn11 = ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign22220_e17127));
        locals.var_tmf2_dn14 = ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign22220_e17127));

        let assign22230_e17132: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign22230_e17133: f64 = (1.0 + assign22230_e17132);
        let assign22230_e17134: f64 = (0.5 * assign22230_e17133);
        locals.var_t6 = assign22230_e17134;
        locals.var_t6_dn0 = (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn2 = (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn4 = (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn5 = (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn6 = (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn7 = (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn8 = (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn9 = (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn10 = (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn11 = (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t6_dn14 = (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)));

    }

    pub(super) fn stamp_transient_block_56(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign22240_e17139: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign22240_e17140: f64 = (0.5 * assign22240_e17139);
        let assign22240_e17141: f64 = (0.1 + assign22240_e17140);
        locals.var_vdsats = assign22240_e17141;
        locals.var_vdsats_dn0 = (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0));
        locals.var_vdsats_dn2 = (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2));
        locals.var_vdsats_dn4 = (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4));
        locals.var_vdsats_dn5 = (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5));
        locals.var_vdsats_dn6 = (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6));
        locals.var_vdsats_dn7 = (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7));
        locals.var_vdsats_dn8 = (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8));
        locals.var_vdsats_dn9 = (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9));
        locals.var_vdsats_dn10 = (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10));
        locals.var_vdsats_dn11 = (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11));
        locals.var_vdsats_dn14 = (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14));

        let assign22250_e17144: f64 = (locals.var_vds / locals.var_vdsats);
        locals.var_t1 = assign22250_e17144;
        locals.var_t1_dn0 = (((locals.var_vds_dn0 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn0)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn2 = (((locals.var_vds_dn2 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn2)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn4 = (((locals.var_vds_dn4 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn4)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn5 = (((locals.var_vds_dn5 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn5)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn6 = (((locals.var_vds_dn6 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn6)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn7 = (((locals.var_vds_dn7 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn7)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn8 = (((locals.var_vds_dn8 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn8)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn9 = (((locals.var_vds_dn9 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn9)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn10 = (((locals.var_vds_dn10 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn10)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn11 = (((locals.var_vds_dn11 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn11)) / (locals.var_vdsats * locals.var_vdsats));
        locals.var_t1_dn14 = (((locals.var_vds_dn14 * locals.var_vdsats) - (locals.var_vds * locals.var_vdsats_dn14)) / (locals.var_vdsats * locals.var_vdsats));

        let assign22260_e17147: f64 = locals.var_t1;
        locals.var_tmf1 = assign22260_e17147;
        locals.var_tmf1_dn0 = locals.var_t1_dn0;
        locals.var_tmf1_dn2 = locals.var_t1_dn2;
        locals.var_tmf1_dn4 = locals.var_t1_dn4;
        locals.var_tmf1_dn5 = locals.var_t1_dn5;
        locals.var_tmf1_dn6 = locals.var_t1_dn6;
        locals.var_tmf1_dn7 = locals.var_t1_dn7;
        locals.var_tmf1_dn8 = locals.var_t1_dn8;
        locals.var_tmf1_dn9 = locals.var_t1_dn9;
        locals.var_tmf1_dn10 = locals.var_t1_dn10;
        locals.var_tmf1_dn11 = locals.var_t1_dn11;
        locals.var_tmf1_dn14 = locals.var_t1_dn14;

        let assign22270_e17150: f64 = (locals.var_tmf1 * locals.var_tmf1);
        locals.var_tmf2 = assign22270_e17150;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2));
        locals.var_tmf2_dn4 = ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4));
        locals.var_tmf2_dn5 = ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6));
        locals.var_tmf2_dn7 = ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7));
        locals.var_tmf2_dn8 = ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8));
        locals.var_tmf2_dn9 = ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10));
        locals.var_tmf2_dn11 = ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11));
        locals.var_tmf2_dn14 = ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14));

        let assign22280_e17153: f64 = (locals.var_tmf2 * locals.var_tmf1);
        locals.var_tmf3 = assign22280_e17153;
        locals.var_tmf3_dn0 = ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0));
        locals.var_tmf3_dn2 = ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2));
        locals.var_tmf3_dn4 = ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4));
        locals.var_tmf3_dn5 = ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5));
        locals.var_tmf3_dn6 = ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6));
        locals.var_tmf3_dn7 = ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7));
        locals.var_tmf3_dn8 = ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8));
        locals.var_tmf3_dn9 = ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9));
        locals.var_tmf3_dn10 = ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10));
        locals.var_tmf3_dn11 = ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11));
        locals.var_tmf3_dn14 = ((locals.var_tmf2_dn14 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn14));

        let assign22290_e17156: f64 = (locals.var_tmf2 * locals.var_tmf2);
        locals.var_tmf4 = assign22290_e17156;
        locals.var_tmf4_dn0 = ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0));
        locals.var_tmf4_dn2 = ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2));
        locals.var_tmf4_dn4 = ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4));
        locals.var_tmf4_dn5 = ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5));
        locals.var_tmf4_dn6 = ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6));
        locals.var_tmf4_dn7 = ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7));
        locals.var_tmf4_dn8 = ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8));
        locals.var_tmf4_dn9 = ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9));
        locals.var_tmf4_dn10 = ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10));
        locals.var_tmf4_dn11 = ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11));
        locals.var_tmf4_dn14 = ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14));

        let assign22300_e17160: f64 = (1.0 + locals.var_tmf1);
        let assign22300_e17162: f64 = (assign22300_e17160 + locals.var_tmf2);
        let assign22300_e17164: f64 = (assign22300_e17162 + locals.var_tmf3);
        let assign22300_e17166: f64 = (assign22300_e17164 + locals.var_tmf4);
        let assign22300_e17167: f64 = (1.0 / assign22300_e17166);
        locals.var_tx = assign22300_e17167;
        locals.var_tx_dn0 = (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign22300_e17166 * assign22300_e17166)));
        locals.var_tx_dn2 = (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign22300_e17166 * assign22300_e17166)));
        locals.var_tx_dn4 = (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign22300_e17166 * assign22300_e17166)));
        locals.var_tx_dn5 = (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign22300_e17166 * assign22300_e17166)));
        locals.var_tx_dn6 = (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign22300_e17166 * assign22300_e17166)));
        locals.var_tx_dn7 = (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign22300_e17166 * assign22300_e17166)));
        locals.var_tx_dn8 = (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign22300_e17166 * assign22300_e17166)));
        locals.var_tx_dn9 = (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign22300_e17166 * assign22300_e17166)));
        locals.var_tx_dn10 = (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign22300_e17166 * assign22300_e17166)));
        locals.var_tx_dn11 = (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign22300_e17166 * assign22300_e17166)));
        locals.var_tx_dn14 = (-((((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) + locals.var_tmf3_dn14) + locals.var_tmf4_dn14) / (assign22300_e17166 * assign22300_e17166)));

        let assign22310_e17171: f64 = (2.0 * locals.var_tmf1);
        let assign22310_e17172: f64 = (1.0 + assign22310_e17171);
        let assign22310_e17175: f64 = (3.0 * locals.var_tmf2);
        let assign22310_e17176: f64 = (assign22310_e17172 + assign22310_e17175);
        let assign22310_e17179: f64 = (4.0 * locals.var_tmf3);
        let assign22310_e17180: f64 = (assign22310_e17176 + assign22310_e17179);
        let assign22310_e17181: f64 = (-assign22310_e17180);
        let assign22310_e17183: f64 = (assign22310_e17181 * locals.var_tx);
        let assign22310_e17185: f64 = (assign22310_e17183 * locals.var_tx);
        locals.var_t0 = assign22310_e17185;
        locals.var_t0_dn0 = (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn0)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn0));
        locals.var_t0_dn2 = (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn2)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn2));
        locals.var_t0_dn4 = (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn4)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn4));
        locals.var_t0_dn5 = (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn5)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn5));
        locals.var_t0_dn6 = (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn6)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn6));
        locals.var_t0_dn7 = (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn7)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn7));
        locals.var_t0_dn8 = (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn8)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn8));
        locals.var_t0_dn9 = (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn9)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn9));
        locals.var_t0_dn10 = (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn10)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn10));
        locals.var_t0_dn11 = (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn11)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn11));
        locals.var_t0_dn14 = (((((-(((2.0 * locals.var_tmf1_dn14) + (3.0 * locals.var_tmf2_dn14)) + (4.0 * locals.var_tmf3_dn14))) * locals.var_tx) + (assign22310_e17181 * locals.var_tx_dn14)) * locals.var_tx) + (assign22310_e17183 * locals.var_tx_dn14));

        let assign22320_e17189: f64 = (1.0 - locals.var_tx);
        let assign22320_e17190: f64 = assign22320_e17189;
        locals.var_tx = assign22320_e17190;
        locals.var_tx_dn0 = (-locals.var_tx_dn0);
        locals.var_tx_dn2 = (-locals.var_tx_dn2);
        locals.var_tx_dn4 = (-locals.var_tx_dn4);
        locals.var_tx_dn5 = (-locals.var_tx_dn5);
        locals.var_tx_dn6 = (-locals.var_tx_dn6);
        locals.var_tx_dn7 = (-locals.var_tx_dn7);
        locals.var_tx_dn8 = (-locals.var_tx_dn8);
        locals.var_tx_dn9 = (-locals.var_tx_dn9);
        locals.var_tx_dn10 = (-locals.var_tx_dn10);
        locals.var_tx_dn11 = (-locals.var_tx_dn11);
        locals.var_tx_dn14 = (-locals.var_tx_dn14);

        let assign22330_e17192: f64 = (-locals.var_t0);
        locals.var_t0 = assign22330_e17192;
        locals.var_t0_dn0 = (-locals.var_t0_dn0);
        locals.var_t0_dn2 = (-locals.var_t0_dn2);
        locals.var_t0_dn4 = (-locals.var_t0_dn4);
        locals.var_t0_dn5 = (-locals.var_t0_dn5);
        locals.var_t0_dn6 = (-locals.var_t0_dn6);
        locals.var_t0_dn7 = (-locals.var_t0_dn7);
        locals.var_t0_dn8 = (-locals.var_t0_dn8);
        locals.var_t0_dn9 = (-locals.var_t0_dn9);
        locals.var_t0_dn10 = (-locals.var_t0_dn10);
        locals.var_t0_dn11 = (-locals.var_t0_dn11);
        locals.var_t0_dn14 = (-locals.var_t0_dn14);

        let assign22340_e17195: f64 = (locals.var_tx * locals.var_tx);
        locals.var_fmdvds = assign22340_e17195;
        locals.var_fmdvds_dn0 = ((locals.var_tx_dn0 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn0));
        locals.var_fmdvds_dn2 = ((locals.var_tx_dn2 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn2));
        locals.var_fmdvds_dn4 = ((locals.var_tx_dn4 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn4));
        locals.var_fmdvds_dn5 = ((locals.var_tx_dn5 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn5));
        locals.var_fmdvds_dn6 = ((locals.var_tx_dn6 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn6));
        locals.var_fmdvds_dn7 = ((locals.var_tx_dn7 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn7));
        locals.var_fmdvds_dn8 = ((locals.var_tx_dn8 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn8));
        locals.var_fmdvds_dn9 = ((locals.var_tx_dn9 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn9));
        locals.var_fmdvds_dn10 = ((locals.var_tx_dn10 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn10));
        locals.var_fmdvds_dn11 = ((locals.var_tx_dn11 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn11));
        locals.var_fmdvds_dn14 = ((locals.var_tx_dn14 * locals.var_tx) + (locals.var_tx * locals.var_tx_dn14));

        let assign22350_e17198: f64 = if locals.var_flg_qmetemp == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard427 = assign22350_e17198;

        let (assign22360_e17202,) = {
    if (locals.var_guard427 != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign22360_e17202;

        let (assign22370_e17207,) = {
    if (locals.var_guard427 == 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign22370_e17207;

        locals.var_t1 = locals.var_qnsub_esi2;
        locals.var_t1_dn0 = locals.var_qnsub_esi2_dn0;
        locals.var_t1_dn2 = locals.var_qnsub_esi2_dn2;
        locals.var_t1_dn4 = locals.var_qnsub_esi2_dn4;
        locals.var_t1_dn5 = locals.var_qnsub_esi2_dn5;
        locals.var_t1_dn6 = locals.var_qnsub_esi2_dn6;
        locals.var_t1_dn7 = locals.var_qnsub_esi2_dn7;
        locals.var_t1_dn8 = locals.var_qnsub_esi2_dn8;
        locals.var_t1_dn9 = locals.var_qnsub_esi2_dn9;
        locals.var_t1_dn10 = locals.var_qnsub_esi2_dn10;
        locals.var_t1_dn11 = locals.var_qnsub_esi2_dn11;
        locals.var_t1_dn14 = locals.var_qnsub_esi2_dn14;

        let assign22390_e17211: f64 = (locals.var_t1 * locals.var_pb20);
        let assign22390_e17212: f64 = (assign22390_e17211).sqrt();
        locals.var_t2 = assign22390_e17212;
        locals.var_t2_dn0 = (((locals.var_t1_dn0 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn0)) / (2.0 * assign22390_e17212));
        locals.var_t2_dn2 = (((locals.var_t1_dn2 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn2)) / (2.0 * assign22390_e17212));
        locals.var_t2_dn4 = (((locals.var_t1_dn4 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn4)) / (2.0 * assign22390_e17212));
        locals.var_t2_dn5 = (((locals.var_t1_dn5 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn5)) / (2.0 * assign22390_e17212));
        locals.var_t2_dn6 = (((locals.var_t1_dn6 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn6)) / (2.0 * assign22390_e17212));
        locals.var_t2_dn7 = (((locals.var_t1_dn7 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn7)) / (2.0 * assign22390_e17212));
        locals.var_t2_dn8 = (((locals.var_t1_dn8 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn8)) / (2.0 * assign22390_e17212));
        locals.var_t2_dn9 = (((locals.var_t1_dn9 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn9)) / (2.0 * assign22390_e17212));
        locals.var_t2_dn10 = (((locals.var_t1_dn10 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn10)) / (2.0 * assign22390_e17212));
        locals.var_t2_dn11 = (((locals.var_t1_dn11 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn11)) / (2.0 * assign22390_e17212));
        locals.var_t2_dn14 = (((locals.var_t1_dn14 * locals.var_pb20) + (locals.var_t1 * locals.var_pb20_dn14)) / (2.0 * assign22390_e17212));

        let assign22400_e17215: f64 = (locals.var_pb20 + locals.var_vfb);
        let assign22400_e17218: f64 = (locals.var_t2 * locals.var_cox0_inv);
        let assign22400_e17219: f64 = (assign22400_e17215 + assign22400_e17218);
        locals.var_vthq = assign22400_e17219;
        locals.var_vthq_dn0 = (locals.var_pb20_dn0 + (locals.var_t2_dn0 * locals.var_cox0_inv));
        locals.var_vthq_dn2 = (locals.var_pb20_dn2 + (locals.var_t2_dn2 * locals.var_cox0_inv));
        locals.var_vthq_dn4 = (locals.var_pb20_dn4 + (locals.var_t2_dn4 * locals.var_cox0_inv));
        locals.var_vthq_dn5 = (locals.var_pb20_dn5 + (locals.var_t2_dn5 * locals.var_cox0_inv));
        locals.var_vthq_dn6 = (locals.var_pb20_dn6 + (locals.var_t2_dn6 * locals.var_cox0_inv));
        locals.var_vthq_dn7 = (locals.var_pb20_dn7 + (locals.var_t2_dn7 * locals.var_cox0_inv));
        locals.var_vthq_dn8 = (locals.var_pb20_dn8 + (locals.var_t2_dn8 * locals.var_cox0_inv));
        locals.var_vthq_dn9 = (locals.var_pb20_dn9 + (locals.var_t2_dn9 * locals.var_cox0_inv));
        locals.var_vthq_dn10 = (locals.var_pb20_dn10 + (locals.var_t2_dn10 * locals.var_cox0_inv));
        locals.var_vthq_dn11 = (locals.var_pb20_dn11 + (locals.var_t2_dn11 * locals.var_cox0_inv));
        locals.var_vthq_dn14 = (locals.var_pb20_dn14 + (locals.var_t2_dn14 * locals.var_cox0_inv));

        let assign22410_e17222: f64 = if locals.var_flg_qme == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard428 = assign22410_e17222;

        let (assign22420_e17226, assign22420_e17226_d_n0, assign22420_e17226_d_n2, assign22420_e17226_d_n4, assign22420_e17226_d_n5, assign22420_e17226_d_n6, assign22420_e17226_d_n7, assign22420_e17226_d_n8, assign22420_e17226_d_n9, assign22420_e17226_d_n10, assign22420_e17226_d_n11, assign22420_e17226_d_n14,) = {
    if (locals.var_guard428 != 0.0) {
        (locals.var_tox0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_toxe, locals.var_toxe_dn0, locals.var_toxe_dn2, locals.var_toxe_dn4, locals.var_toxe_dn5, locals.var_toxe_dn6, locals.var_toxe_dn7, locals.var_toxe_dn8, locals.var_toxe_dn9, locals.var_toxe_dn10, locals.var_toxe_dn11, locals.var_toxe_dn14,)
    }
};
        locals.var_toxe = assign22420_e17226;
        locals.var_toxe_dn0 = assign22420_e17226_d_n0;
        locals.var_toxe_dn2 = assign22420_e17226_d_n2;
        locals.var_toxe_dn4 = assign22420_e17226_d_n4;
        locals.var_toxe_dn5 = assign22420_e17226_d_n5;
        locals.var_toxe_dn6 = assign22420_e17226_d_n6;
        locals.var_toxe_dn7 = assign22420_e17226_d_n7;
        locals.var_toxe_dn8 = assign22420_e17226_d_n8;
        locals.var_toxe_dn9 = assign22420_e17226_d_n9;
        locals.var_toxe_dn10 = assign22420_e17226_d_n10;
        locals.var_toxe_dn11 = assign22420_e17226_d_n11;
        locals.var_toxe_dn14 = assign22420_e17226_d_n14;

        let (assign22430_e17230, assign22430_e17230_d_n0, assign22430_e17230_d_n2, assign22430_e17230_d_n4, assign22430_e17230_d_n5, assign22430_e17230_d_n6, assign22430_e17230_d_n7, assign22430_e17230_d_n8, assign22430_e17230_d_n9, assign22430_e17230_d_n10, assign22430_e17230_d_n11, assign22430_e17230_d_n14,) = {
    if (locals.var_guard428 != 0.0) {
        (locals.var_cox0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cox, locals.var_cox_dn0, locals.var_cox_dn2, locals.var_cox_dn4, locals.var_cox_dn5, locals.var_cox_dn6, locals.var_cox_dn7, locals.var_cox_dn8, locals.var_cox_dn9, locals.var_cox_dn10, locals.var_cox_dn11, locals.var_cox_dn14,)
    }
};
        locals.var_cox = assign22430_e17230;
        locals.var_cox_dn0 = assign22430_e17230_d_n0;
        locals.var_cox_dn2 = assign22430_e17230_d_n2;
        locals.var_cox_dn4 = assign22430_e17230_d_n4;
        locals.var_cox_dn5 = assign22430_e17230_d_n5;
        locals.var_cox_dn6 = assign22430_e17230_d_n6;
        locals.var_cox_dn7 = assign22430_e17230_d_n7;
        locals.var_cox_dn8 = assign22430_e17230_d_n8;
        locals.var_cox_dn9 = assign22430_e17230_d_n9;
        locals.var_cox_dn10 = assign22430_e17230_d_n10;
        locals.var_cox_dn11 = assign22430_e17230_d_n11;
        locals.var_cox_dn14 = assign22430_e17230_d_n14;

        let (assign22440_e17234, assign22440_e17234_d_n0, assign22440_e17234_d_n2, assign22440_e17234_d_n4, assign22440_e17234_d_n5, assign22440_e17234_d_n6, assign22440_e17234_d_n7, assign22440_e17234_d_n8, assign22440_e17234_d_n9, assign22440_e17234_d_n10, assign22440_e17234_d_n11, assign22440_e17234_d_n14,) = {
    if (locals.var_guard428 != 0.0) {
        (locals.var_cox0_inv, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cox_inv, locals.var_cox_inv_dn0, locals.var_cox_inv_dn2, locals.var_cox_inv_dn4, locals.var_cox_inv_dn5, locals.var_cox_inv_dn6, locals.var_cox_inv_dn7, locals.var_cox_inv_dn8, locals.var_cox_inv_dn9, locals.var_cox_inv_dn10, locals.var_cox_inv_dn11, locals.var_cox_inv_dn14,)
    }
};
        locals.var_cox_inv = assign22440_e17234;
        locals.var_cox_inv_dn0 = assign22440_e17234_d_n0;
        locals.var_cox_inv_dn2 = assign22440_e17234_d_n2;
        locals.var_cox_inv_dn4 = assign22440_e17234_d_n4;
        locals.var_cox_inv_dn5 = assign22440_e17234_d_n5;
        locals.var_cox_inv_dn6 = assign22440_e17234_d_n6;
        locals.var_cox_inv_dn7 = assign22440_e17234_d_n7;
        locals.var_cox_inv_dn8 = assign22440_e17234_d_n8;
        locals.var_cox_inv_dn9 = assign22440_e17234_d_n9;
        locals.var_cox_inv_dn10 = assign22440_e17234_d_n10;
        locals.var_cox_inv_dn11 = assign22440_e17234_d_n11;
        locals.var_cox_inv_dn14 = assign22440_e17234_d_n14;

        let (assign22450_e17242, assign22450_e17242_d_n0, assign22450_e17242_d_n2, assign22450_e17242_d_n4, assign22450_e17242_d_n5, assign22450_e17242_d_n6, assign22450_e17242_d_n7, assign22450_e17242_d_n8, assign22450_e17242_d_n9, assign22450_e17242_d_n10, assign22450_e17242_d_n11, assign22450_e17242_d_n14,) = {
    if (locals.var_guard428 != 0.0) {
        let assign22450_e17238: f64 = (locals.var_cnst0 * locals.var_cnst0);
        let assign22450_e17240: f64 = (assign22450_e17238 * locals.var_cox_inv);
        (assign22450_e17240, ((((locals.var_cnst0_dn0 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn0)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn0)), ((((locals.var_cnst0_dn2 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn2)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn2)), ((((locals.var_cnst0_dn4 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn4)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn4)), ((((locals.var_cnst0_dn5 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn5)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn5)), ((((locals.var_cnst0_dn6 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn6)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn6)), ((((locals.var_cnst0_dn7 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn7)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn7)), ((((locals.var_cnst0_dn8 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn8)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn8)), ((((locals.var_cnst0_dn9 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn9)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn9)), ((((locals.var_cnst0_dn10 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn10)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn10)), ((((locals.var_cnst0_dn11 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn11)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn11)), ((((locals.var_cnst0_dn14 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn14)) * locals.var_cox_inv) + (assign22450_e17238 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign22450_e17242;
        locals.var_t0_dn0 = assign22450_e17242_d_n0;
        locals.var_t0_dn2 = assign22450_e17242_d_n2;
        locals.var_t0_dn4 = assign22450_e17242_d_n4;
        locals.var_t0_dn5 = assign22450_e17242_d_n5;
        locals.var_t0_dn6 = assign22450_e17242_d_n6;
        locals.var_t0_dn7 = assign22450_e17242_d_n7;
        locals.var_t0_dn8 = assign22450_e17242_d_n8;
        locals.var_t0_dn9 = assign22450_e17242_d_n9;
        locals.var_t0_dn10 = assign22450_e17242_d_n10;
        locals.var_t0_dn11 = assign22450_e17242_d_n11;
        locals.var_t0_dn14 = assign22450_e17242_d_n14;

        let (assign22460_e17248, assign22460_e17248_d_n0, assign22460_e17248_d_n2, assign22460_e17248_d_n4, assign22460_e17248_d_n5, assign22460_e17248_d_n6, assign22460_e17248_d_n7, assign22460_e17248_d_n8, assign22460_e17248_d_n9, assign22460_e17248_d_n10, assign22460_e17248_d_n11, assign22460_e17248_d_n14,) = {
    if (locals.var_guard428 != 0.0) {
        let assign22460_e17246: f64 = (locals.var_t0 * locals.var_cox_inv);
        (assign22460_e17246, ((locals.var_t0_dn0 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn0)), ((locals.var_t0_dn2 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn2)), ((locals.var_t0_dn4 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn4)), ((locals.var_t0_dn5 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn5)), ((locals.var_t0_dn6 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn6)), ((locals.var_t0_dn7 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn7)), ((locals.var_t0_dn8 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn8)), ((locals.var_t0_dn9 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn9)), ((locals.var_t0_dn10 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn10)), ((locals.var_t0_dn11 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn11)), ((locals.var_t0_dn14 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_cnstcoxi, locals.var_cnstcoxi_dn0, locals.var_cnstcoxi_dn2, locals.var_cnstcoxi_dn4, locals.var_cnstcoxi_dn5, locals.var_cnstcoxi_dn6, locals.var_cnstcoxi_dn7, locals.var_cnstcoxi_dn8, locals.var_cnstcoxi_dn9, locals.var_cnstcoxi_dn10, locals.var_cnstcoxi_dn11, locals.var_cnstcoxi_dn14,)
    }
};
        locals.var_cnstcoxi = assign22460_e17248;
        locals.var_cnstcoxi_dn0 = assign22460_e17248_d_n0;
        locals.var_cnstcoxi_dn2 = assign22460_e17248_d_n2;
        locals.var_cnstcoxi_dn4 = assign22460_e17248_d_n4;
        locals.var_cnstcoxi_dn5 = assign22460_e17248_d_n5;
        locals.var_cnstcoxi_dn6 = assign22460_e17248_d_n6;
        locals.var_cnstcoxi_dn7 = assign22460_e17248_d_n7;
        locals.var_cnstcoxi_dn8 = assign22460_e17248_d_n8;
        locals.var_cnstcoxi_dn9 = assign22460_e17248_d_n9;
        locals.var_cnstcoxi_dn10 = assign22460_e17248_d_n10;
        locals.var_cnstcoxi_dn11 = assign22460_e17248_d_n11;
        locals.var_cnstcoxi_dn14 = assign22460_e17248_d_n14;

        let (assign22470_e17259, assign22470_e17259_d_n0, assign22470_e17259_d_n2, assign22470_e17259_d_n4, assign22470_e17259_d_n5, assign22470_e17259_d_n6, assign22470_e17259_d_n7, assign22470_e17259_d_n8, assign22470_e17259_d_n9, assign22470_e17259_d_n10, assign22470_e17259_d_n11, assign22470_e17259_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22470_e17253: f64 = (locals.var_vgs - locals.var_vbs);
        let assign22470_e17255: f64 = (assign22470_e17253 - locals.var_vthq);
        let assign22470_e17257: f64 = (assign22470_e17255 + p.p236);
        (assign22470_e17257, (-locals.var_vthq_dn0), (-locals.var_vthq_dn2), (-locals.var_vthq_dn4), (-locals.var_vthq_dn5), ((locals.var_vgs_dn6 - locals.var_vbs_dn6) - locals.var_vthq_dn6), (locals.var_vgs_dn7 - locals.var_vthq_dn7), ((locals.var_vgs_dn8 - locals.var_vbs_dn8) - locals.var_vthq_dn8), ((-locals.var_vbs_dn9) - locals.var_vthq_dn9), (-locals.var_vthq_dn10), (-locals.var_vthq_dn11), (-locals.var_vthq_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign22470_e17259;
        locals.var_t5_dn0 = assign22470_e17259_d_n0;
        locals.var_t5_dn2 = assign22470_e17259_d_n2;
        locals.var_t5_dn4 = assign22470_e17259_d_n4;
        locals.var_t5_dn5 = assign22470_e17259_d_n5;
        locals.var_t5_dn6 = assign22470_e17259_d_n6;
        locals.var_t5_dn7 = assign22470_e17259_d_n7;
        locals.var_t5_dn8 = assign22470_e17259_d_n8;
        locals.var_t5_dn9 = assign22470_e17259_d_n9;
        locals.var_t5_dn10 = assign22470_e17259_d_n10;
        locals.var_t5_dn11 = assign22470_e17259_d_n11;
        locals.var_t5_dn14 = assign22470_e17259_d_n14;

        let (assign22480_e17277, assign22480_e17277_d_n0, assign22480_e17277_d_n2, assign22480_e17277_d_n4, assign22480_e17277_d_n5, assign22480_e17277_d_n6, assign22480_e17277_d_n7, assign22480_e17277_d_n8, assign22480_e17277_d_n9, assign22480_e17277_d_n10, assign22480_e17277_d_n11, assign22480_e17277_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22480_e17264: f64 = (locals.var_t5 * locals.var_t5);
        let assign22480_e17268: f64 = (1e-9 * 0.01);
        let assign22480_e17269: f64 = (4.0 * assign22480_e17268);
        let assign22480_e17272: f64 = (1e-9 * 0.01);
        let assign22480_e17273: f64 = (assign22480_e17269 * assign22480_e17272);
        let assign22480_e17274: f64 = (assign22480_e17264 + assign22480_e17273);
        let assign22480_e17275: f64 = (assign22480_e17274).sqrt();
        (assign22480_e17275, (((locals.var_t5_dn0 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn0)) / (2.0 * assign22480_e17275)), (((locals.var_t5_dn2 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn2)) / (2.0 * assign22480_e17275)), (((locals.var_t5_dn4 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn4)) / (2.0 * assign22480_e17275)), (((locals.var_t5_dn5 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn5)) / (2.0 * assign22480_e17275)), (((locals.var_t5_dn6 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn6)) / (2.0 * assign22480_e17275)), (((locals.var_t5_dn7 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn7)) / (2.0 * assign22480_e17275)), (((locals.var_t5_dn8 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn8)) / (2.0 * assign22480_e17275)), (((locals.var_t5_dn9 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn9)) / (2.0 * assign22480_e17275)), (((locals.var_t5_dn10 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn10)) / (2.0 * assign22480_e17275)), (((locals.var_t5_dn11 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn11)) / (2.0 * assign22480_e17275)), (((locals.var_t5_dn14 * locals.var_t5) + (locals.var_t5 * locals.var_t5_dn14)) / (2.0 * assign22480_e17275)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign22480_e17277;
        locals.var_tmf2_dn0 = assign22480_e17277_d_n0;
        locals.var_tmf2_dn2 = assign22480_e17277_d_n2;
        locals.var_tmf2_dn4 = assign22480_e17277_d_n4;
        locals.var_tmf2_dn5 = assign22480_e17277_d_n5;
        locals.var_tmf2_dn6 = assign22480_e17277_d_n6;
        locals.var_tmf2_dn7 = assign22480_e17277_d_n7;
        locals.var_tmf2_dn8 = assign22480_e17277_d_n8;
        locals.var_tmf2_dn9 = assign22480_e17277_d_n9;
        locals.var_tmf2_dn10 = assign22480_e17277_d_n10;
        locals.var_tmf2_dn11 = assign22480_e17277_d_n11;
        locals.var_tmf2_dn14 = assign22480_e17277_d_n14;

        let (assign22490_e17288, assign22490_e17288_d_n0, assign22490_e17288_d_n2, assign22490_e17288_d_n4, assign22490_e17288_d_n5, assign22490_e17288_d_n6, assign22490_e17288_d_n7, assign22490_e17288_d_n8, assign22490_e17288_d_n9, assign22490_e17288_d_n10, assign22490_e17288_d_n11, assign22490_e17288_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22490_e17284: f64 = (locals.var_t5 / locals.var_tmf2);
        let assign22490_e17285: f64 = (1.0 + assign22490_e17284);
        let assign22490_e17286: f64 = (0.5 * assign22490_e17285);
        (assign22490_e17286, (0.5 * (((locals.var_t5_dn0 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn2 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn4 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn5 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn6 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn7 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn8 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn9 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn10 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn11 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_t5_dn14 * locals.var_tmf2) - (locals.var_t5 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign22490_e17288;
        locals.var_t3_dn0 = assign22490_e17288_d_n0;
        locals.var_t3_dn2 = assign22490_e17288_d_n2;
        locals.var_t3_dn4 = assign22490_e17288_d_n4;
        locals.var_t3_dn5 = assign22490_e17288_d_n5;
        locals.var_t3_dn6 = assign22490_e17288_d_n6;
        locals.var_t3_dn7 = assign22490_e17288_d_n7;
        locals.var_t3_dn8 = assign22490_e17288_d_n8;
        locals.var_t3_dn9 = assign22490_e17288_d_n9;
        locals.var_t3_dn10 = assign22490_e17288_d_n10;
        locals.var_t3_dn11 = assign22490_e17288_d_n11;
        locals.var_t3_dn14 = assign22490_e17288_d_n14;

        let (assign22500_e17297, assign22500_e17297_d_n0, assign22500_e17297_d_n2, assign22500_e17297_d_n4, assign22500_e17297_d_n5, assign22500_e17297_d_n6, assign22500_e17297_d_n7, assign22500_e17297_d_n8, assign22500_e17297_d_n9, assign22500_e17297_d_n10, assign22500_e17297_d_n11, assign22500_e17297_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22500_e17294: f64 = (locals.var_t5 + locals.var_tmf2);
        let assign22500_e17295: f64 = (0.5 * assign22500_e17294);
        (assign22500_e17295, (0.5 * (locals.var_t5_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_t5_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_t5_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_t5_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_t5_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_t5_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_t5_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_t5_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_t5_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_t5_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_t5_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign22500_e17297;
        locals.var_t2_dn0 = assign22500_e17297_d_n0;
        locals.var_t2_dn2 = assign22500_e17297_d_n2;
        locals.var_t2_dn4 = assign22500_e17297_d_n4;
        locals.var_t2_dn5 = assign22500_e17297_d_n5;
        locals.var_t2_dn6 = assign22500_e17297_d_n6;
        locals.var_t2_dn7 = assign22500_e17297_d_n7;
        locals.var_t2_dn8 = assign22500_e17297_d_n8;
        locals.var_t2_dn9 = assign22500_e17297_d_n9;
        locals.var_t2_dn10 = assign22500_e17297_d_n10;
        locals.var_t2_dn11 = assign22500_e17297_d_n11;
        locals.var_t2_dn14 = assign22500_e17297_d_n14;

        let assign22510_e17300: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard429 = assign22510_e17300;

        let (assign22520_e17307, assign22520_e17307_d_n0, assign22520_e17307_d_n2, assign22520_e17307_d_n4, assign22520_e17307_d_n5, assign22520_e17307_d_n6, assign22520_e17307_d_n7, assign22520_e17307_d_n8, assign22520_e17307_d_n9, assign22520_e17307_d_n10, assign22520_e17307_d_n11, assign22520_e17307_d_n14,) = {
    if ((locals.var_guard428 == 0.0) && (locals.var_guard429 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign22520_e17307;
        locals.var_t2_dn0 = assign22520_e17307_d_n0;
        locals.var_t2_dn2 = assign22520_e17307_d_n2;
        locals.var_t2_dn4 = assign22520_e17307_d_n4;
        locals.var_t2_dn5 = assign22520_e17307_d_n5;
        locals.var_t2_dn6 = assign22520_e17307_d_n6;
        locals.var_t2_dn7 = assign22520_e17307_d_n7;
        locals.var_t2_dn8 = assign22520_e17307_d_n8;
        locals.var_t2_dn9 = assign22520_e17307_d_n9;
        locals.var_t2_dn10 = assign22520_e17307_d_n10;
        locals.var_t2_dn11 = assign22520_e17307_d_n11;
        locals.var_t2_dn14 = assign22520_e17307_d_n14;

        let (assign22530_e17314, assign22530_e17314_d_n0, assign22530_e17314_d_n2, assign22530_e17314_d_n4, assign22530_e17314_d_n5, assign22530_e17314_d_n6, assign22530_e17314_d_n7, assign22530_e17314_d_n8, assign22530_e17314_d_n9, assign22530_e17314_d_n10, assign22530_e17314_d_n11, assign22530_e17314_d_n14,) = {
    if ((locals.var_guard428 == 0.0) && (locals.var_guard429 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign22530_e17314;
        locals.var_t3_dn0 = assign22530_e17314_d_n0;
        locals.var_t3_dn2 = assign22530_e17314_d_n2;
        locals.var_t3_dn4 = assign22530_e17314_d_n4;
        locals.var_t3_dn5 = assign22530_e17314_d_n5;
        locals.var_t3_dn6 = assign22530_e17314_d_n6;
        locals.var_t3_dn7 = assign22530_e17314_d_n7;
        locals.var_t3_dn8 = assign22530_e17314_d_n8;
        locals.var_t3_dn9 = assign22530_e17314_d_n9;
        locals.var_t3_dn10 = assign22530_e17314_d_n10;
        locals.var_t3_dn11 = assign22530_e17314_d_n11;
        locals.var_t3_dn14 = assign22530_e17314_d_n14;

        let (assign22540_e17321, assign22540_e17321_d_n0, assign22540_e17321_d_n2, assign22540_e17321_d_n4, assign22540_e17321_d_n5, assign22540_e17321_d_n6, assign22540_e17321_d_n7, assign22540_e17321_d_n8, assign22540_e17321_d_n9, assign22540_e17321_d_n10, assign22540_e17321_d_n11, assign22540_e17321_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22540_e17319: f64 = (locals.var_t2 + 1e-25);
        (assign22540_e17319, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign22540_e17321;
        locals.var_t2_dn0 = assign22540_e17321_d_n0;
        locals.var_t2_dn2 = assign22540_e17321_d_n2;
        locals.var_t2_dn4 = assign22540_e17321_d_n4;
        locals.var_t2_dn5 = assign22540_e17321_d_n5;
        locals.var_t2_dn6 = assign22540_e17321_d_n6;
        locals.var_t2_dn7 = assign22540_e17321_d_n7;
        locals.var_t2_dn8 = assign22540_e17321_d_n8;
        locals.var_t2_dn9 = assign22540_e17321_d_n9;
        locals.var_t2_dn10 = assign22540_e17321_d_n10;
        locals.var_t2_dn11 = assign22540_e17321_d_n11;
        locals.var_t2_dn14 = assign22540_e17321_d_n14;

    }

    pub(super) fn stamp_transient_block_57(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign22550_e17328, assign22550_e17328_d_n0, assign22550_e17328_d_n2, assign22550_e17328_d_n4, assign22550_e17328_d_n5, assign22550_e17328_d_n6, assign22550_e17328_d_n7, assign22550_e17328_d_n8, assign22550_e17328_d_n9, assign22550_e17328_d_n10, assign22550_e17328_d_n11, assign22550_e17328_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22550_e17326: f64 = (1.0 / locals.var_t2);
        (assign22550_e17326, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn14 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign22550_e17328;
        locals.var_t3_dn0 = assign22550_e17328_d_n0;
        locals.var_t3_dn2 = assign22550_e17328_d_n2;
        locals.var_t3_dn4 = assign22550_e17328_d_n4;
        locals.var_t3_dn5 = assign22550_e17328_d_n5;
        locals.var_t3_dn6 = assign22550_e17328_d_n6;
        locals.var_t3_dn7 = assign22550_e17328_d_n7;
        locals.var_t3_dn8 = assign22550_e17328_d_n8;
        locals.var_t3_dn9 = assign22550_e17328_d_n9;
        locals.var_t3_dn10 = assign22550_e17328_d_n10;
        locals.var_t3_dn11 = assign22550_e17328_d_n11;
        locals.var_t3_dn14 = assign22550_e17328_d_n14;

        let (assign22560_e17338, assign22560_e17338_d_n0, assign22560_e17338_d_n2, assign22560_e17338_d_n4, assign22560_e17338_d_n5, assign22560_e17338_d_n6, assign22560_e17338_d_n7, assign22560_e17338_d_n8, assign22560_e17338_d_n9, assign22560_e17338_d_n10, assign22560_e17338_d_n11, assign22560_e17338_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22560_e17332: f64 = (-1.0);
        let assign22560_e17335: f64 = (locals.var_t2 * locals.var_t2);
        let assign22560_e17336: f64 = (assign22560_e17332 / assign22560_e17335);
        (assign22560_e17336, (-((assign22560_e17332 * ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))) / (assign22560_e17335 * assign22560_e17335))), (-((assign22560_e17332 * ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))) / (assign22560_e17335 * assign22560_e17335))), (-((assign22560_e17332 * ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))) / (assign22560_e17335 * assign22560_e17335))), (-((assign22560_e17332 * ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))) / (assign22560_e17335 * assign22560_e17335))), (-((assign22560_e17332 * ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))) / (assign22560_e17335 * assign22560_e17335))), (-((assign22560_e17332 * ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))) / (assign22560_e17335 * assign22560_e17335))), (-((assign22560_e17332 * ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))) / (assign22560_e17335 * assign22560_e17335))), (-((assign22560_e17332 * ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))) / (assign22560_e17335 * assign22560_e17335))), (-((assign22560_e17332 * ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))) / (assign22560_e17335 * assign22560_e17335))), (-((assign22560_e17332 * ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11))) / (assign22560_e17335 * assign22560_e17335))), (-((assign22560_e17332 * ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14))) / (assign22560_e17335 * assign22560_e17335))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign22560_e17338;
        locals.var_t7_dn0 = assign22560_e17338_d_n0;
        locals.var_t7_dn2 = assign22560_e17338_d_n2;
        locals.var_t7_dn4 = assign22560_e17338_d_n4;
        locals.var_t7_dn5 = assign22560_e17338_d_n5;
        locals.var_t7_dn6 = assign22560_e17338_d_n6;
        locals.var_t7_dn7 = assign22560_e17338_d_n7;
        locals.var_t7_dn8 = assign22560_e17338_d_n8;
        locals.var_t7_dn9 = assign22560_e17338_d_n9;
        locals.var_t7_dn10 = assign22560_e17338_d_n10;
        locals.var_t7_dn11 = assign22560_e17338_d_n11;
        locals.var_t7_dn14 = assign22560_e17338_d_n14;

        let (assign22570_e17346, assign22570_e17346_d_n0, assign22570_e17346_d_n2, assign22570_e17346_d_n4, assign22570_e17346_d_n5, assign22570_e17346_d_n6, assign22570_e17346_d_n7, assign22570_e17346_d_n8, assign22570_e17346_d_n9, assign22570_e17346_d_n10, assign22570_e17346_d_n11, assign22570_e17346_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22570_e17343: f64 = (locals.var_vthq).abs();
        let assign22570_e17344: f64 = (2.0 * assign22570_e17343);
        (assign22570_e17344, (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn0 } else { (-locals.var_vthq_dn0) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn2 } else { (-locals.var_vthq_dn2) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn4 } else { (-locals.var_vthq_dn4) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn5 } else { (-locals.var_vthq_dn5) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn6 } else { (-locals.var_vthq_dn6) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn7 } else { (-locals.var_vthq_dn7) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn8 } else { (-locals.var_vthq_dn8) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn9 } else { (-locals.var_vthq_dn9) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn10 } else { (-locals.var_vthq_dn10) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn11 } else { (-locals.var_vthq_dn11) }), (2.0 * if locals.var_vthq >= 0.0 { locals.var_vthq_dn14 } else { (-locals.var_vthq_dn14) }),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign22570_e17346;
        locals.var_t4_dn0 = assign22570_e17346_d_n0;
        locals.var_t4_dn2 = assign22570_e17346_d_n2;
        locals.var_t4_dn4 = assign22570_e17346_d_n4;
        locals.var_t4_dn5 = assign22570_e17346_d_n5;
        locals.var_t4_dn6 = assign22570_e17346_d_n6;
        locals.var_t4_dn7 = assign22570_e17346_d_n7;
        locals.var_t4_dn8 = assign22570_e17346_d_n8;
        locals.var_t4_dn9 = assign22570_e17346_d_n9;
        locals.var_t4_dn10 = assign22570_e17346_d_n10;
        locals.var_t4_dn11 = assign22570_e17346_d_n11;
        locals.var_t4_dn14 = assign22570_e17346_d_n14;

        let (assign22580_e17355, assign22580_e17355_d_n0, assign22580_e17355_d_n2, assign22580_e17355_d_n4, assign22580_e17355_d_n5, assign22580_e17355_d_n6, assign22580_e17355_d_n7, assign22580_e17355_d_n8, assign22580_e17355_d_n9, assign22580_e17355_d_n10, assign22580_e17355_d_n11, assign22580_e17355_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22580_e17351: f64 = (locals.var_t5 - locals.var_vgs);
        let assign22580_e17353: f64 = (assign22580_e17351 + locals.var_vfb);
        (assign22580_e17353, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, (locals.var_t5_dn6 - locals.var_vgs_dn6), (locals.var_t5_dn7 - locals.var_vgs_dn7), (locals.var_t5_dn8 - locals.var_vgs_dn8), locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign22580_e17355;
        locals.var_t6_dn0 = assign22580_e17355_d_n0;
        locals.var_t6_dn2 = assign22580_e17355_d_n2;
        locals.var_t6_dn4 = assign22580_e17355_d_n4;
        locals.var_t6_dn5 = assign22580_e17355_d_n5;
        locals.var_t6_dn6 = assign22580_e17355_d_n6;
        locals.var_t6_dn7 = assign22580_e17355_d_n7;
        locals.var_t6_dn8 = assign22580_e17355_d_n8;
        locals.var_t6_dn9 = assign22580_e17355_d_n9;
        locals.var_t6_dn10 = assign22580_e17355_d_n10;
        locals.var_t6_dn11 = assign22580_e17355_d_n11;
        locals.var_t6_dn14 = assign22580_e17355_d_n14;

        let assign22590_e17358: f64 = if locals.var_t6 > locals.var_t4 { 1.0 } else { 0.0 };
        locals.var_guard430 = assign22590_e17358;

        let (assign22600_e17365, assign22600_e17365_d_n0, assign22600_e17365_d_n2, assign22600_e17365_d_n4, assign22600_e17365_d_n5, assign22600_e17365_d_n6, assign22600_e17365_d_n7, assign22600_e17365_d_n8, assign22600_e17365_d_n9, assign22600_e17365_d_n10, assign22600_e17365_d_n11, assign22600_e17365_d_n14,) = {
    if ((locals.var_guard428 == 0.0) && (locals.var_guard430 != 0.0)) {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign22600_e17365;
        locals.var_t4_dn0 = assign22600_e17365_d_n0;
        locals.var_t4_dn2 = assign22600_e17365_d_n2;
        locals.var_t4_dn4 = assign22600_e17365_d_n4;
        locals.var_t4_dn5 = assign22600_e17365_d_n5;
        locals.var_t4_dn6 = assign22600_e17365_d_n6;
        locals.var_t4_dn7 = assign22600_e17365_d_n7;
        locals.var_t4_dn8 = assign22600_e17365_d_n8;
        locals.var_t4_dn9 = assign22600_e17365_d_n9;
        locals.var_t4_dn10 = assign22600_e17365_d_n10;
        locals.var_t4_dn11 = assign22600_e17365_d_n11;
        locals.var_t4_dn14 = assign22600_e17365_d_n14;

        let (assign22610_e17378, assign22610_e17378_d_n0, assign22610_e17378_d_n2, assign22610_e17378_d_n4, assign22610_e17378_d_n5, assign22610_e17378_d_n6, assign22610_e17378_d_n7, assign22610_e17378_d_n8, assign22610_e17378_d_n9, assign22610_e17378_d_n10, assign22610_e17378_d_n11, assign22610_e17378_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22610_e17370: f64 = (1.0 / locals.var_t4);
        let assign22610_e17372: f64 = (assign22610_e17370 - locals.var_t3);
        let assign22610_e17375: f64 = (1e-9 * 0.01);
        let assign22610_e17376: f64 = (assign22610_e17372 - assign22610_e17375);
        (assign22610_e17376, ((-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn0), ((-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn2), ((-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn4), ((-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn5), ((-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn6), ((-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn7), ((-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn8), ((-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn9), ((-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn10), ((-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn11), ((-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))) - locals.var_t3_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign22610_e17378;
        locals.var_tmf1_dn0 = assign22610_e17378_d_n0;
        locals.var_tmf1_dn2 = assign22610_e17378_d_n2;
        locals.var_tmf1_dn4 = assign22610_e17378_d_n4;
        locals.var_tmf1_dn5 = assign22610_e17378_d_n5;
        locals.var_tmf1_dn6 = assign22610_e17378_d_n6;
        locals.var_tmf1_dn7 = assign22610_e17378_d_n7;
        locals.var_tmf1_dn8 = assign22610_e17378_d_n8;
        locals.var_tmf1_dn9 = assign22610_e17378_d_n9;
        locals.var_tmf1_dn10 = assign22610_e17378_d_n10;
        locals.var_tmf1_dn11 = assign22610_e17378_d_n11;
        locals.var_tmf1_dn14 = assign22610_e17378_d_n14;

        let (assign22620_e17391, assign22620_e17391_d_n0, assign22620_e17391_d_n2, assign22620_e17391_d_n4, assign22620_e17391_d_n5, assign22620_e17391_d_n6, assign22620_e17391_d_n7, assign22620_e17391_d_n8, assign22620_e17391_d_n9, assign22620_e17391_d_n10, assign22620_e17391_d_n11, assign22620_e17391_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22620_e17384: f64 = (1.0 / locals.var_t4);
        let assign22620_e17385: f64 = (4.0 * assign22620_e17384);
        let assign22620_e17388: f64 = (1e-9 * 0.01);
        let assign22620_e17389: f64 = (assign22620_e17385 * assign22620_e17388);
        (assign22620_e17389, ((4.0 * (-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388), ((4.0 * (-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388), ((4.0 * (-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388), ((4.0 * (-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388), ((4.0 * (-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388), ((4.0 * (-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388), ((4.0 * (-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388), ((4.0 * (-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388), ((4.0 * (-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388), ((4.0 * (-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388), ((4.0 * (-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4)))) * assign22620_e17388),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign22620_e17391;
        locals.var_tmf2_dn0 = assign22620_e17391_d_n0;
        locals.var_tmf2_dn2 = assign22620_e17391_d_n2;
        locals.var_tmf2_dn4 = assign22620_e17391_d_n4;
        locals.var_tmf2_dn5 = assign22620_e17391_d_n5;
        locals.var_tmf2_dn6 = assign22620_e17391_d_n6;
        locals.var_tmf2_dn7 = assign22620_e17391_d_n7;
        locals.var_tmf2_dn8 = assign22620_e17391_d_n8;
        locals.var_tmf2_dn9 = assign22620_e17391_d_n9;
        locals.var_tmf2_dn10 = assign22620_e17391_d_n10;
        locals.var_tmf2_dn11 = assign22620_e17391_d_n11;
        locals.var_tmf2_dn14 = assign22620_e17391_d_n14;

        let (assign22630_e17402, assign22630_e17402_d_n0, assign22630_e17402_d_n2, assign22630_e17402_d_n4, assign22630_e17402_d_n5, assign22630_e17402_d_n6, assign22630_e17402_d_n7, assign22630_e17402_d_n8, assign22630_e17402_d_n9, assign22630_e17402_d_n10, assign22630_e17402_d_n11, assign22630_e17402_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let (assign22630_e17400, assign22630_e17400_d_n0, assign22630_e17400_d_n2, assign22630_e17400_d_n4, assign22630_e17400_d_n5, assign22630_e17400_d_n6, assign22630_e17400_d_n7, assign22630_e17400_d_n8, assign22630_e17400_d_n9, assign22630_e17400_d_n10, assign22630_e17400_d_n11, assign22630_e17400_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign22630_e17399: f64 = (-locals.var_tmf2);
                (assign22630_e17399, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign22630_e17400, assign22630_e17400_d_n0, assign22630_e17400_d_n2, assign22630_e17400_d_n4, assign22630_e17400_d_n5, assign22630_e17400_d_n6, assign22630_e17400_d_n7, assign22630_e17400_d_n8, assign22630_e17400_d_n9, assign22630_e17400_d_n10, assign22630_e17400_d_n11, assign22630_e17400_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign22630_e17402;
        locals.var_tmf2_dn0 = assign22630_e17402_d_n0;
        locals.var_tmf2_dn2 = assign22630_e17402_d_n2;
        locals.var_tmf2_dn4 = assign22630_e17402_d_n4;
        locals.var_tmf2_dn5 = assign22630_e17402_d_n5;
        locals.var_tmf2_dn6 = assign22630_e17402_d_n6;
        locals.var_tmf2_dn7 = assign22630_e17402_d_n7;
        locals.var_tmf2_dn8 = assign22630_e17402_d_n8;
        locals.var_tmf2_dn9 = assign22630_e17402_d_n9;
        locals.var_tmf2_dn10 = assign22630_e17402_d_n10;
        locals.var_tmf2_dn11 = assign22630_e17402_d_n11;
        locals.var_tmf2_dn14 = assign22630_e17402_d_n14;

        let (assign22640_e17412, assign22640_e17412_d_n0, assign22640_e17412_d_n2, assign22640_e17412_d_n4, assign22640_e17412_d_n5, assign22640_e17412_d_n6, assign22640_e17412_d_n7, assign22640_e17412_d_n8, assign22640_e17412_d_n9, assign22640_e17412_d_n10, assign22640_e17412_d_n11, assign22640_e17412_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22640_e17407: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign22640_e17409: f64 = (assign22640_e17407 + locals.var_tmf2);
        let assign22640_e17410: f64 = (assign22640_e17409).sqrt();
        (assign22640_e17410, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign22640_e17410)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign22640_e17410)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign22640_e17410)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign22640_e17410)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign22640_e17410)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign22640_e17410)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign22640_e17410)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign22640_e17410)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign22640_e17410)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign22640_e17410)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign22640_e17410)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign22640_e17412;
        locals.var_tmf2_dn0 = assign22640_e17412_d_n0;
        locals.var_tmf2_dn2 = assign22640_e17412_d_n2;
        locals.var_tmf2_dn4 = assign22640_e17412_d_n4;
        locals.var_tmf2_dn5 = assign22640_e17412_d_n5;
        locals.var_tmf2_dn6 = assign22640_e17412_d_n6;
        locals.var_tmf2_dn7 = assign22640_e17412_d_n7;
        locals.var_tmf2_dn8 = assign22640_e17412_d_n8;
        locals.var_tmf2_dn9 = assign22640_e17412_d_n9;
        locals.var_tmf2_dn10 = assign22640_e17412_d_n10;
        locals.var_tmf2_dn11 = assign22640_e17412_d_n11;
        locals.var_tmf2_dn14 = assign22640_e17412_d_n14;

        let (assign22650_e17423, assign22650_e17423_d_n0, assign22650_e17423_d_n2, assign22650_e17423_d_n4, assign22650_e17423_d_n5, assign22650_e17423_d_n6, assign22650_e17423_d_n7, assign22650_e17423_d_n8, assign22650_e17423_d_n9, assign22650_e17423_d_n10, assign22650_e17423_d_n11, assign22650_e17423_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22650_e17419: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign22650_e17420: f64 = (1.0 + assign22650_e17419);
        let assign22650_e17421: f64 = (0.5 * assign22650_e17420);
        (assign22650_e17421, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign22650_e17423;
        locals.var_t6_dn0 = assign22650_e17423_d_n0;
        locals.var_t6_dn2 = assign22650_e17423_d_n2;
        locals.var_t6_dn4 = assign22650_e17423_d_n4;
        locals.var_t6_dn5 = assign22650_e17423_d_n5;
        locals.var_t6_dn6 = assign22650_e17423_d_n6;
        locals.var_t6_dn7 = assign22650_e17423_d_n7;
        locals.var_t6_dn8 = assign22650_e17423_d_n8;
        locals.var_t6_dn9 = assign22650_e17423_d_n9;
        locals.var_t6_dn10 = assign22650_e17423_d_n10;
        locals.var_t6_dn11 = assign22650_e17423_d_n11;
        locals.var_t6_dn14 = assign22650_e17423_d_n14;

        let (assign22660_e17436, assign22660_e17436_d_n0, assign22660_e17436_d_n2, assign22660_e17436_d_n4, assign22660_e17436_d_n5, assign22660_e17436_d_n6, assign22660_e17436_d_n7, assign22660_e17436_d_n8, assign22660_e17436_d_n9, assign22660_e17436_d_n10, assign22660_e17436_d_n11, assign22660_e17436_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22660_e17428: f64 = (1.0 / locals.var_t4);
        let assign22660_e17432: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign22660_e17433: f64 = (0.5 * assign22660_e17432);
        let assign22660_e17434: f64 = (assign22660_e17428 - assign22660_e17433);
        (assign22660_e17434, ((-(locals.var_t4_dn0 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((-(locals.var_t4_dn2 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((-(locals.var_t4_dn4 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((-(locals.var_t4_dn5 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((-(locals.var_t4_dn6 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((-(locals.var_t4_dn7 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((-(locals.var_t4_dn8 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((-(locals.var_t4_dn9 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((-(locals.var_t4_dn10 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((-(locals.var_t4_dn11 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((-(locals.var_t4_dn14 / (locals.var_t4 * locals.var_t4))) - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign22660_e17436;
        locals.var_t2_dn0 = assign22660_e17436_d_n0;
        locals.var_t2_dn2 = assign22660_e17436_d_n2;
        locals.var_t2_dn4 = assign22660_e17436_d_n4;
        locals.var_t2_dn5 = assign22660_e17436_d_n5;
        locals.var_t2_dn6 = assign22660_e17436_d_n6;
        locals.var_t2_dn7 = assign22660_e17436_d_n7;
        locals.var_t2_dn8 = assign22660_e17436_d_n8;
        locals.var_t2_dn9 = assign22660_e17436_d_n9;
        locals.var_t2_dn10 = assign22660_e17436_d_n10;
        locals.var_t2_dn11 = assign22660_e17436_d_n11;
        locals.var_t2_dn14 = assign22660_e17436_d_n14;

        let (assign22670_e17445, assign22670_e17445_d_n0, assign22670_e17445_d_n2, assign22670_e17445_d_n4, assign22670_e17445_d_n5, assign22670_e17445_d_n6, assign22670_e17445_d_n7, assign22670_e17445_d_n8, assign22670_e17445_d_n9, assign22670_e17445_d_n10, assign22670_e17445_d_n11, assign22670_e17445_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22670_e17441: f64 = (p.p235 * locals.var_t2);
        let assign22670_e17443: f64 = (assign22670_e17441 + p.p237);
        (assign22670_e17443, (p.p235 * locals.var_t2_dn0), (p.p235 * locals.var_t2_dn2), (p.p235 * locals.var_t2_dn4), (p.p235 * locals.var_t2_dn5), (p.p235 * locals.var_t2_dn6), (p.p235 * locals.var_t2_dn7), (p.p235 * locals.var_t2_dn8), (p.p235 * locals.var_t2_dn9), (p.p235 * locals.var_t2_dn10), (p.p235 * locals.var_t2_dn11), (p.p235 * locals.var_t2_dn14),)
    } else {
        (locals.var_dtox, locals.var_dtox_dn0, locals.var_dtox_dn2, locals.var_dtox_dn4, locals.var_dtox_dn5, locals.var_dtox_dn6, locals.var_dtox_dn7, locals.var_dtox_dn8, locals.var_dtox_dn9, locals.var_dtox_dn10, locals.var_dtox_dn11, locals.var_dtox_dn14,)
    }
};
        locals.var_dtox = assign22670_e17445;
        locals.var_dtox_dn0 = assign22670_e17445_d_n0;
        locals.var_dtox_dn2 = assign22670_e17445_d_n2;
        locals.var_dtox_dn4 = assign22670_e17445_d_n4;
        locals.var_dtox_dn5 = assign22670_e17445_d_n5;
        locals.var_dtox_dn6 = assign22670_e17445_d_n6;
        locals.var_dtox_dn7 = assign22670_e17445_d_n7;
        locals.var_dtox_dn8 = assign22670_e17445_d_n8;
        locals.var_dtox_dn9 = assign22670_e17445_d_n9;
        locals.var_dtox_dn10 = assign22670_e17445_d_n10;
        locals.var_dtox_dn11 = assign22670_e17445_d_n11;
        locals.var_dtox_dn14 = assign22670_e17445_d_n14;

        let (assign22680_e17450, assign22680_e17450_d_n0, assign22680_e17450_d_n2, assign22680_e17450_d_n4, assign22680_e17450_d_n5, assign22680_e17450_d_n6, assign22680_e17450_d_n7, assign22680_e17450_d_n8, assign22680_e17450_d_n9, assign22680_e17450_d_n10, assign22680_e17450_d_n11, assign22680_e17450_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        (p.p235, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign22680_e17450;
        locals.var_t7_dn0 = assign22680_e17450_d_n0;
        locals.var_t7_dn2 = assign22680_e17450_d_n2;
        locals.var_t7_dn4 = assign22680_e17450_d_n4;
        locals.var_t7_dn5 = assign22680_e17450_d_n5;
        locals.var_t7_dn6 = assign22680_e17450_d_n6;
        locals.var_t7_dn7 = assign22680_e17450_d_n7;
        locals.var_t7_dn8 = assign22680_e17450_d_n8;
        locals.var_t7_dn9 = assign22680_e17450_d_n9;
        locals.var_t7_dn10 = assign22680_e17450_d_n10;
        locals.var_t7_dn11 = assign22680_e17450_d_n11;
        locals.var_t7_dn14 = assign22680_e17450_d_n14;

        let assign22690_e17453: f64 = (locals.var_dtox * 1000000000000.0);
        let assign22690_e17455: f64 = if assign22690_e17453 < locals.var_tox0 { 1.0 } else { 0.0 };
        locals.var_guard431 = assign22690_e17455;

        let (assign22700_e17462, assign22700_e17462_d_n0, assign22700_e17462_d_n2, assign22700_e17462_d_n4, assign22700_e17462_d_n5, assign22700_e17462_d_n6, assign22700_e17462_d_n7, assign22700_e17462_d_n8, assign22700_e17462_d_n9, assign22700_e17462_d_n10, assign22700_e17462_d_n11, assign22700_e17462_d_n14,) = {
    if ((locals.var_guard428 == 0.0) && (locals.var_guard431 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dtox, locals.var_dtox_dn0, locals.var_dtox_dn2, locals.var_dtox_dn4, locals.var_dtox_dn5, locals.var_dtox_dn6, locals.var_dtox_dn7, locals.var_dtox_dn8, locals.var_dtox_dn9, locals.var_dtox_dn10, locals.var_dtox_dn11, locals.var_dtox_dn14,)
    }
};
        locals.var_dtox = assign22700_e17462;
        locals.var_dtox_dn0 = assign22700_e17462_d_n0;
        locals.var_dtox_dn2 = assign22700_e17462_d_n2;
        locals.var_dtox_dn4 = assign22700_e17462_d_n4;
        locals.var_dtox_dn5 = assign22700_e17462_d_n5;
        locals.var_dtox_dn6 = assign22700_e17462_d_n6;
        locals.var_dtox_dn7 = assign22700_e17462_d_n7;
        locals.var_dtox_dn8 = assign22700_e17462_d_n8;
        locals.var_dtox_dn9 = assign22700_e17462_d_n9;
        locals.var_dtox_dn10 = assign22700_e17462_d_n10;
        locals.var_dtox_dn11 = assign22700_e17462_d_n11;
        locals.var_dtox_dn14 = assign22700_e17462_d_n14;

        let (assign22710_e17469,) = {
    if ((locals.var_guard428 == 0.0) && (locals.var_guard431 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_flg_qme,)
    }
};
        locals.var_flg_qme = assign22710_e17469;

        let (assign22720_e17476, assign22720_e17476_d_n0, assign22720_e17476_d_n2, assign22720_e17476_d_n4, assign22720_e17476_d_n5, assign22720_e17476_d_n6, assign22720_e17476_d_n7, assign22720_e17476_d_n8, assign22720_e17476_d_n9, assign22720_e17476_d_n10, assign22720_e17476_d_n11, assign22720_e17476_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22720_e17474: f64 = (locals.var_tox0 + locals.var_dtox);
        (assign22720_e17474, locals.var_dtox_dn0, locals.var_dtox_dn2, locals.var_dtox_dn4, locals.var_dtox_dn5, locals.var_dtox_dn6, locals.var_dtox_dn7, locals.var_dtox_dn8, locals.var_dtox_dn9, locals.var_dtox_dn10, locals.var_dtox_dn11, locals.var_dtox_dn14,)
    } else {
        (locals.var_toxe, locals.var_toxe_dn0, locals.var_toxe_dn2, locals.var_toxe_dn4, locals.var_toxe_dn5, locals.var_toxe_dn6, locals.var_toxe_dn7, locals.var_toxe_dn8, locals.var_toxe_dn9, locals.var_toxe_dn10, locals.var_toxe_dn11, locals.var_toxe_dn14,)
    }
};
        locals.var_toxe = assign22720_e17476;
        locals.var_toxe_dn0 = assign22720_e17476_d_n0;
        locals.var_toxe_dn2 = assign22720_e17476_d_n2;
        locals.var_toxe_dn4 = assign22720_e17476_d_n4;
        locals.var_toxe_dn5 = assign22720_e17476_d_n5;
        locals.var_toxe_dn6 = assign22720_e17476_d_n6;
        locals.var_toxe_dn7 = assign22720_e17476_d_n7;
        locals.var_toxe_dn8 = assign22720_e17476_d_n8;
        locals.var_toxe_dn9 = assign22720_e17476_d_n9;
        locals.var_toxe_dn10 = assign22720_e17476_d_n10;
        locals.var_toxe_dn11 = assign22720_e17476_d_n11;
        locals.var_toxe_dn14 = assign22720_e17476_d_n14;

        let (assign22730_e17483, assign22730_e17483_d_n0, assign22730_e17483_d_n2, assign22730_e17483_d_n4, assign22730_e17483_d_n5, assign22730_e17483_d_n6, assign22730_e17483_d_n7, assign22730_e17483_d_n8, assign22730_e17483_d_n9, assign22730_e17483_d_n10, assign22730_e17483_d_n11, assign22730_e17483_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22730_e17481: f64 = (locals.var_c_eox / locals.var_toxe);
        (assign22730_e17481, (-((locals.var_c_eox * locals.var_toxe_dn0) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn2) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn4) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn5) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn6) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn7) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn8) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn9) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn10) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn11) / (locals.var_toxe * locals.var_toxe))), (-((locals.var_c_eox * locals.var_toxe_dn14) / (locals.var_toxe * locals.var_toxe))),)
    } else {
        (locals.var_cox, locals.var_cox_dn0, locals.var_cox_dn2, locals.var_cox_dn4, locals.var_cox_dn5, locals.var_cox_dn6, locals.var_cox_dn7, locals.var_cox_dn8, locals.var_cox_dn9, locals.var_cox_dn10, locals.var_cox_dn11, locals.var_cox_dn14,)
    }
};
        locals.var_cox = assign22730_e17483;
        locals.var_cox_dn0 = assign22730_e17483_d_n0;
        locals.var_cox_dn2 = assign22730_e17483_d_n2;
        locals.var_cox_dn4 = assign22730_e17483_d_n4;
        locals.var_cox_dn5 = assign22730_e17483_d_n5;
        locals.var_cox_dn6 = assign22730_e17483_d_n6;
        locals.var_cox_dn7 = assign22730_e17483_d_n7;
        locals.var_cox_dn8 = assign22730_e17483_d_n8;
        locals.var_cox_dn9 = assign22730_e17483_d_n9;
        locals.var_cox_dn10 = assign22730_e17483_d_n10;
        locals.var_cox_dn11 = assign22730_e17483_d_n11;
        locals.var_cox_dn14 = assign22730_e17483_d_n14;

        let (assign22740_e17493, assign22740_e17493_d_n0, assign22740_e17493_d_n2, assign22740_e17493_d_n4, assign22740_e17493_d_n5, assign22740_e17493_d_n6, assign22740_e17493_d_n7, assign22740_e17493_d_n8, assign22740_e17493_d_n9, assign22740_e17493_d_n10, assign22740_e17493_d_n11, assign22740_e17493_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22740_e17487: f64 = (-locals.var_c_eox);
        let assign22740_e17490: f64 = (locals.var_toxe * locals.var_toxe);
        let assign22740_e17491: f64 = (assign22740_e17487 / assign22740_e17490);
        (assign22740_e17491, (-((assign22740_e17487 * ((locals.var_toxe_dn0 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn0))) / (assign22740_e17490 * assign22740_e17490))), (-((assign22740_e17487 * ((locals.var_toxe_dn2 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn2))) / (assign22740_e17490 * assign22740_e17490))), (-((assign22740_e17487 * ((locals.var_toxe_dn4 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn4))) / (assign22740_e17490 * assign22740_e17490))), (-((assign22740_e17487 * ((locals.var_toxe_dn5 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn5))) / (assign22740_e17490 * assign22740_e17490))), (-((assign22740_e17487 * ((locals.var_toxe_dn6 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn6))) / (assign22740_e17490 * assign22740_e17490))), (-((assign22740_e17487 * ((locals.var_toxe_dn7 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn7))) / (assign22740_e17490 * assign22740_e17490))), (-((assign22740_e17487 * ((locals.var_toxe_dn8 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn8))) / (assign22740_e17490 * assign22740_e17490))), (-((assign22740_e17487 * ((locals.var_toxe_dn9 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn9))) / (assign22740_e17490 * assign22740_e17490))), (-((assign22740_e17487 * ((locals.var_toxe_dn10 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn10))) / (assign22740_e17490 * assign22740_e17490))), (-((assign22740_e17487 * ((locals.var_toxe_dn11 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn11))) / (assign22740_e17490 * assign22740_e17490))), (-((assign22740_e17487 * ((locals.var_toxe_dn14 * locals.var_toxe) + (locals.var_toxe * locals.var_toxe_dn14))) / (assign22740_e17490 * assign22740_e17490))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign22740_e17493;
        locals.var_t1_dn0 = assign22740_e17493_d_n0;
        locals.var_t1_dn2 = assign22740_e17493_d_n2;
        locals.var_t1_dn4 = assign22740_e17493_d_n4;
        locals.var_t1_dn5 = assign22740_e17493_d_n5;
        locals.var_t1_dn6 = assign22740_e17493_d_n6;
        locals.var_t1_dn7 = assign22740_e17493_d_n7;
        locals.var_t1_dn8 = assign22740_e17493_d_n8;
        locals.var_t1_dn9 = assign22740_e17493_d_n9;
        locals.var_t1_dn10 = assign22740_e17493_d_n10;
        locals.var_t1_dn11 = assign22740_e17493_d_n11;
        locals.var_t1_dn14 = assign22740_e17493_d_n14;

        let (assign22750_e17500, assign22750_e17500_d_n0, assign22750_e17500_d_n2, assign22750_e17500_d_n4, assign22750_e17500_d_n5, assign22750_e17500_d_n6, assign22750_e17500_d_n7, assign22750_e17500_d_n8, assign22750_e17500_d_n9, assign22750_e17500_d_n10, assign22750_e17500_d_n11, assign22750_e17500_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22750_e17498: f64 = (locals.var_toxe / locals.var_c_eox);
        (assign22750_e17498, (locals.var_toxe_dn0 / locals.var_c_eox), (locals.var_toxe_dn2 / locals.var_c_eox), (locals.var_toxe_dn4 / locals.var_c_eox), (locals.var_toxe_dn5 / locals.var_c_eox), (locals.var_toxe_dn6 / locals.var_c_eox), (locals.var_toxe_dn7 / locals.var_c_eox), (locals.var_toxe_dn8 / locals.var_c_eox), (locals.var_toxe_dn9 / locals.var_c_eox), (locals.var_toxe_dn10 / locals.var_c_eox), (locals.var_toxe_dn11 / locals.var_c_eox), (locals.var_toxe_dn14 / locals.var_c_eox),)
    } else {
        (locals.var_cox_inv, locals.var_cox_inv_dn0, locals.var_cox_inv_dn2, locals.var_cox_inv_dn4, locals.var_cox_inv_dn5, locals.var_cox_inv_dn6, locals.var_cox_inv_dn7, locals.var_cox_inv_dn8, locals.var_cox_inv_dn9, locals.var_cox_inv_dn10, locals.var_cox_inv_dn11, locals.var_cox_inv_dn14,)
    }
};
        locals.var_cox_inv = assign22750_e17500;
        locals.var_cox_inv_dn0 = assign22750_e17500_d_n0;
        locals.var_cox_inv_dn2 = assign22750_e17500_d_n2;
        locals.var_cox_inv_dn4 = assign22750_e17500_d_n4;
        locals.var_cox_inv_dn5 = assign22750_e17500_d_n5;
        locals.var_cox_inv_dn6 = assign22750_e17500_d_n6;
        locals.var_cox_inv_dn7 = assign22750_e17500_d_n7;
        locals.var_cox_inv_dn8 = assign22750_e17500_d_n8;
        locals.var_cox_inv_dn9 = assign22750_e17500_d_n9;
        locals.var_cox_inv_dn10 = assign22750_e17500_d_n10;
        locals.var_cox_inv_dn11 = assign22750_e17500_d_n11;
        locals.var_cox_inv_dn14 = assign22750_e17500_d_n14;

        let (assign22760_e17507, assign22760_e17507_d_n0, assign22760_e17507_d_n2, assign22760_e17507_d_n4, assign22760_e17507_d_n5, assign22760_e17507_d_n6, assign22760_e17507_d_n7, assign22760_e17507_d_n8, assign22760_e17507_d_n9, assign22760_e17507_d_n10, assign22760_e17507_d_n11, assign22760_e17507_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22760_e17505: f64 = (1.0 / locals.var_c_eox);
        (assign22760_e17505, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign22760_e17507;
        locals.var_t1_dn0 = assign22760_e17507_d_n0;
        locals.var_t1_dn2 = assign22760_e17507_d_n2;
        locals.var_t1_dn4 = assign22760_e17507_d_n4;
        locals.var_t1_dn5 = assign22760_e17507_d_n5;
        locals.var_t1_dn6 = assign22760_e17507_d_n6;
        locals.var_t1_dn7 = assign22760_e17507_d_n7;
        locals.var_t1_dn8 = assign22760_e17507_d_n8;
        locals.var_t1_dn9 = assign22760_e17507_d_n9;
        locals.var_t1_dn10 = assign22760_e17507_d_n10;
        locals.var_t1_dn11 = assign22760_e17507_d_n11;
        locals.var_t1_dn14 = assign22760_e17507_d_n14;

        let (assign22770_e17516, assign22770_e17516_d_n0, assign22770_e17516_d_n2, assign22770_e17516_d_n4, assign22770_e17516_d_n5, assign22770_e17516_d_n6, assign22770_e17516_d_n7, assign22770_e17516_d_n8, assign22770_e17516_d_n9, assign22770_e17516_d_n10, assign22770_e17516_d_n11, assign22770_e17516_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22770_e17512: f64 = (locals.var_cnst0 * locals.var_cnst0);
        let assign22770_e17514: f64 = (assign22770_e17512 * locals.var_cox_inv);
        (assign22770_e17514, ((((locals.var_cnst0_dn0 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn0)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn0)), ((((locals.var_cnst0_dn2 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn2)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn2)), ((((locals.var_cnst0_dn4 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn4)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn4)), ((((locals.var_cnst0_dn5 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn5)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn5)), ((((locals.var_cnst0_dn6 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn6)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn6)), ((((locals.var_cnst0_dn7 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn7)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn7)), ((((locals.var_cnst0_dn8 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn8)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn8)), ((((locals.var_cnst0_dn9 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn9)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn9)), ((((locals.var_cnst0_dn10 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn10)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn10)), ((((locals.var_cnst0_dn11 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn11)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn11)), ((((locals.var_cnst0_dn14 * locals.var_cnst0) + (locals.var_cnst0 * locals.var_cnst0_dn14)) * locals.var_cox_inv) + (assign22770_e17512 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign22770_e17516;
        locals.var_t0_dn0 = assign22770_e17516_d_n0;
        locals.var_t0_dn2 = assign22770_e17516_d_n2;
        locals.var_t0_dn4 = assign22770_e17516_d_n4;
        locals.var_t0_dn5 = assign22770_e17516_d_n5;
        locals.var_t0_dn6 = assign22770_e17516_d_n6;
        locals.var_t0_dn7 = assign22770_e17516_d_n7;
        locals.var_t0_dn8 = assign22770_e17516_d_n8;
        locals.var_t0_dn9 = assign22770_e17516_d_n9;
        locals.var_t0_dn10 = assign22770_e17516_d_n10;
        locals.var_t0_dn11 = assign22770_e17516_d_n11;
        locals.var_t0_dn14 = assign22770_e17516_d_n14;

        let (assign22780_e17523, assign22780_e17523_d_n0, assign22780_e17523_d_n2, assign22780_e17523_d_n4, assign22780_e17523_d_n5, assign22780_e17523_d_n6, assign22780_e17523_d_n7, assign22780_e17523_d_n8, assign22780_e17523_d_n9, assign22780_e17523_d_n10, assign22780_e17523_d_n11, assign22780_e17523_d_n14,) = {
    if (locals.var_guard428 == 0.0) {
        let assign22780_e17521: f64 = (locals.var_t0 * locals.var_cox_inv);
        (assign22780_e17521, ((locals.var_t0_dn0 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn0)), ((locals.var_t0_dn2 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn2)), ((locals.var_t0_dn4 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn4)), ((locals.var_t0_dn5 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn5)), ((locals.var_t0_dn6 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn6)), ((locals.var_t0_dn7 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn7)), ((locals.var_t0_dn8 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn8)), ((locals.var_t0_dn9 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn9)), ((locals.var_t0_dn10 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn10)), ((locals.var_t0_dn11 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn11)), ((locals.var_t0_dn14 * locals.var_cox_inv) + (locals.var_t0 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_cnstcoxi, locals.var_cnstcoxi_dn0, locals.var_cnstcoxi_dn2, locals.var_cnstcoxi_dn4, locals.var_cnstcoxi_dn5, locals.var_cnstcoxi_dn6, locals.var_cnstcoxi_dn7, locals.var_cnstcoxi_dn8, locals.var_cnstcoxi_dn9, locals.var_cnstcoxi_dn10, locals.var_cnstcoxi_dn11, locals.var_cnstcoxi_dn14,)
    }
};
        locals.var_cnstcoxi = assign22780_e17523;
        locals.var_cnstcoxi_dn0 = assign22780_e17523_d_n0;
        locals.var_cnstcoxi_dn2 = assign22780_e17523_d_n2;
        locals.var_cnstcoxi_dn4 = assign22780_e17523_d_n4;
        locals.var_cnstcoxi_dn5 = assign22780_e17523_d_n5;
        locals.var_cnstcoxi_dn6 = assign22780_e17523_d_n6;
        locals.var_cnstcoxi_dn7 = assign22780_e17523_d_n7;
        locals.var_cnstcoxi_dn8 = assign22780_e17523_d_n8;
        locals.var_cnstcoxi_dn9 = assign22780_e17523_d_n9;
        locals.var_cnstcoxi_dn10 = assign22780_e17523_d_n10;
        locals.var_cnstcoxi_dn11 = assign22780_e17523_d_n11;
        locals.var_cnstcoxi_dn14 = assign22780_e17523_d_n14;

        locals.var_vbsz2 = locals.var_vbsz;
        locals.var_vbsz2_dn0 = locals.var_vbsz_dn0;
        locals.var_vbsz2_dn2 = locals.var_vbsz_dn2;
        locals.var_vbsz2_dn4 = locals.var_vbsz_dn4;
        locals.var_vbsz2_dn5 = locals.var_vbsz_dn5;
        locals.var_vbsz2_dn6 = locals.var_vbsz_dn6;
        locals.var_vbsz2_dn7 = locals.var_vbsz_dn7;
        locals.var_vbsz2_dn8 = locals.var_vbsz_dn8;
        locals.var_vbsz2_dn9 = locals.var_vbsz_dn9;
        locals.var_vbsz2_dn10 = locals.var_vbsz_dn10;
        locals.var_vbsz2_dn11 = locals.var_vbsz_dn11;
        locals.var_vbsz2_dn14 = locals.var_vbsz_dn14;

        locals.var_t1 = locals.var_qnsub_esi2;
        locals.var_t1_dn0 = locals.var_qnsub_esi2_dn0;
        locals.var_t1_dn2 = locals.var_qnsub_esi2_dn2;
        locals.var_t1_dn4 = locals.var_qnsub_esi2_dn4;
        locals.var_t1_dn5 = locals.var_qnsub_esi2_dn5;
        locals.var_t1_dn6 = locals.var_qnsub_esi2_dn6;
        locals.var_t1_dn7 = locals.var_qnsub_esi2_dn7;
        locals.var_t1_dn8 = locals.var_qnsub_esi2_dn8;
        locals.var_t1_dn9 = locals.var_qnsub_esi2_dn9;
        locals.var_t1_dn10 = locals.var_qnsub_esi2_dn10;
        locals.var_t1_dn11 = locals.var_qnsub_esi2_dn11;
        locals.var_t1_dn14 = locals.var_qnsub_esi2_dn14;

    }

    pub(super) fn stamp_transient_block_58(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign22810_e17529: f64 = (locals.var_pb20 - locals.var_vbsz2);
        let assign22810_e17530: f64 = (locals.var_t1 * assign22810_e17529);
        let assign22810_e17531: f64 = (assign22810_e17530).sqrt();
        locals.var_qb0 = assign22810_e17531;
        locals.var_qb0_dn0 = (((locals.var_t1_dn0 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn0 - locals.var_vbsz2_dn0))) / (2.0 * assign22810_e17531));
        locals.var_qb0_dn2 = (((locals.var_t1_dn2 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn2 - locals.var_vbsz2_dn2))) / (2.0 * assign22810_e17531));
        locals.var_qb0_dn4 = (((locals.var_t1_dn4 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn4 - locals.var_vbsz2_dn4))) / (2.0 * assign22810_e17531));
        locals.var_qb0_dn5 = (((locals.var_t1_dn5 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn5 - locals.var_vbsz2_dn5))) / (2.0 * assign22810_e17531));
        locals.var_qb0_dn6 = (((locals.var_t1_dn6 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn6 - locals.var_vbsz2_dn6))) / (2.0 * assign22810_e17531));
        locals.var_qb0_dn7 = (((locals.var_t1_dn7 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn7 - locals.var_vbsz2_dn7))) / (2.0 * assign22810_e17531));
        locals.var_qb0_dn8 = (((locals.var_t1_dn8 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn8 - locals.var_vbsz2_dn8))) / (2.0 * assign22810_e17531));
        locals.var_qb0_dn9 = (((locals.var_t1_dn9 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn9 - locals.var_vbsz2_dn9))) / (2.0 * assign22810_e17531));
        locals.var_qb0_dn10 = (((locals.var_t1_dn10 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn10 - locals.var_vbsz2_dn10))) / (2.0 * assign22810_e17531));
        locals.var_qb0_dn11 = (((locals.var_t1_dn11 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn11 - locals.var_vbsz2_dn11))) / (2.0 * assign22810_e17531));
        locals.var_qb0_dn14 = (((locals.var_t1_dn14 * assign22810_e17529) + (locals.var_t1 * (locals.var_pb20_dn14 - locals.var_vbsz2_dn14))) / (2.0 * assign22810_e17531));

        let assign22820_e17534: f64 = (0.5 * locals.var_t1);
        let assign22820_e17536: f64 = (assign22820_e17534 / locals.var_qb0);
        locals.var_t2 = assign22820_e17536;
        locals.var_t2_dn0 = ((((0.5 * locals.var_t1_dn0) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn0)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn2 = ((((0.5 * locals.var_t1_dn2) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn2)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn4 = ((((0.5 * locals.var_t1_dn4) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn4)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn5 = ((((0.5 * locals.var_t1_dn5) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn5)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn6 = ((((0.5 * locals.var_t1_dn6) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn6)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn7 = ((((0.5 * locals.var_t1_dn7) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn7)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn8 = ((((0.5 * locals.var_t1_dn8) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn8)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn9 = ((((0.5 * locals.var_t1_dn9) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn9)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn10 = ((((0.5 * locals.var_t1_dn10) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn10)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn11 = ((((0.5 * locals.var_t1_dn11) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn11)) / (locals.var_qb0 * locals.var_qb0));
        locals.var_t2_dn14 = ((((0.5 * locals.var_t1_dn14) * locals.var_qb0) - (assign22820_e17534 * locals.var_qb0_dn14)) / (locals.var_qb0 * locals.var_qb0));

        let assign22830_e17539: f64 = (locals.var_pb20 + locals.var_vfb);
        let assign22830_e17542: f64 = (locals.var_qb0 * locals.var_cox_inv);
        let assign22830_e17543: f64 = (assign22830_e17539 + assign22830_e17542);
        let assign22830_e17545: f64 = (assign22830_e17543 + locals.var_ptovr);
        locals.var_vthp = assign22830_e17545;
        locals.var_vthp_dn0 = ((locals.var_pb20_dn0 + ((locals.var_qb0_dn0 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn0))) + locals.var_ptovr_dn0);
        locals.var_vthp_dn2 = ((locals.var_pb20_dn2 + ((locals.var_qb0_dn2 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn2))) + locals.var_ptovr_dn2);
        locals.var_vthp_dn4 = ((locals.var_pb20_dn4 + ((locals.var_qb0_dn4 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn4))) + locals.var_ptovr_dn4);
        locals.var_vthp_dn5 = ((locals.var_pb20_dn5 + ((locals.var_qb0_dn5 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn5))) + locals.var_ptovr_dn5);
        locals.var_vthp_dn6 = ((locals.var_pb20_dn6 + ((locals.var_qb0_dn6 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn6))) + locals.var_ptovr_dn6);
        locals.var_vthp_dn7 = ((locals.var_pb20_dn7 + ((locals.var_qb0_dn7 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn7))) + locals.var_ptovr_dn7);
        locals.var_vthp_dn8 = ((locals.var_pb20_dn8 + ((locals.var_qb0_dn8 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn8))) + locals.var_ptovr_dn8);
        locals.var_vthp_dn9 = ((locals.var_pb20_dn9 + ((locals.var_qb0_dn9 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn9))) + locals.var_ptovr_dn9);
        locals.var_vthp_dn10 = ((locals.var_pb20_dn10 + ((locals.var_qb0_dn10 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn10))) + locals.var_ptovr_dn10);
        locals.var_vthp_dn11 = ((locals.var_pb20_dn11 + ((locals.var_qb0_dn11 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn11))) + locals.var_ptovr_dn11);
        locals.var_vthp_dn14 = ((locals.var_pb20_dn14 + ((locals.var_qb0_dn14 * locals.var_cox_inv) + (locals.var_qb0 * locals.var_cox_inv_dn14))) + locals.var_ptovr_dn14);

        locals.var_pb20b = locals.var_pb20;
        locals.var_pb20b_dn0 = locals.var_pb20_dn0;
        locals.var_pb20b_dn2 = locals.var_pb20_dn2;
        locals.var_pb20b_dn4 = locals.var_pb20_dn4;
        locals.var_pb20b_dn5 = locals.var_pb20_dn5;
        locals.var_pb20b_dn6 = locals.var_pb20_dn6;
        locals.var_pb20b_dn7 = locals.var_pb20_dn7;
        locals.var_pb20b_dn8 = locals.var_pb20_dn8;
        locals.var_pb20b_dn9 = locals.var_pb20_dn9;
        locals.var_pb20b_dn10 = locals.var_pb20_dn10;
        locals.var_pb20b_dn11 = locals.var_pb20_dn11;
        locals.var_pb20b_dn14 = locals.var_pb20_dn14;

        locals.var_t0 = 0.95;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let (assign22860_e17553,) = {
    if (locals.var_uc_codep > 1.0) {
        (0.0,)
    } else {
        (1.0,)
    }
};
        locals.var_t4 = assign22860_e17553;
        locals.var_t4_dn0 = 0.0;
        locals.var_t4_dn2 = 0.0;
        locals.var_t4_dn4 = 0.0;
        locals.var_t4_dn5 = 0.0;
        locals.var_t4_dn6 = 0.0;
        locals.var_t4_dn7 = 0.0;
        locals.var_t4_dn8 = 0.0;
        locals.var_t4_dn9 = 0.0;
        locals.var_t4_dn10 = 0.0;
        locals.var_t4_dn11 = 0.0;
        locals.var_t4_dn14 = 0.0;

        let assign22870_e17556: f64 = (locals.var_t0 * locals.var_pb20b);
        let assign22870_e17559: f64 = (locals.var_t4 * locals.var_vbsz2);
        let assign22870_e17560: f64 = (assign22870_e17556 - assign22870_e17559);
        let assign22870_e17562: f64 = (assign22870_e17560 - 0.001);
        locals.var_t1 = assign22870_e17562;
        locals.var_t1_dn0 = (((locals.var_t0_dn0 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn0)) - ((locals.var_t4_dn0 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn0)));
        locals.var_t1_dn2 = (((locals.var_t0_dn2 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn2)) - ((locals.var_t4_dn2 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn2)));
        locals.var_t1_dn4 = (((locals.var_t0_dn4 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn4)) - ((locals.var_t4_dn4 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn4)));
        locals.var_t1_dn5 = (((locals.var_t0_dn5 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn5)) - ((locals.var_t4_dn5 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn5)));
        locals.var_t1_dn6 = (((locals.var_t0_dn6 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn6)) - ((locals.var_t4_dn6 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn6)));
        locals.var_t1_dn7 = (((locals.var_t0_dn7 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn7)) - ((locals.var_t4_dn7 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn7)));
        locals.var_t1_dn8 = (((locals.var_t0_dn8 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn8)) - ((locals.var_t4_dn8 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn8)));
        locals.var_t1_dn9 = (((locals.var_t0_dn9 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn9)) - ((locals.var_t4_dn9 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn9)));
        locals.var_t1_dn10 = (((locals.var_t0_dn10 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn10)) - ((locals.var_t4_dn10 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn10)));
        locals.var_t1_dn11 = (((locals.var_t0_dn11 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn11)) - ((locals.var_t4_dn11 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn11)));
        locals.var_t1_dn14 = (((locals.var_t0_dn14 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn14)) - ((locals.var_t4_dn14 * locals.var_vbsz2) + (locals.var_t4 * locals.var_vbsz2_dn14)));

        let assign22880_e17565: f64 = (locals.var_t1 * locals.var_t1);
        let assign22880_e17568: f64 = (4.0 * locals.var_t0);
        let assign22880_e17570: f64 = (assign22880_e17568 * locals.var_pb20b);
        let assign22880_e17572: f64 = (assign22880_e17570 * 0.001);
        let assign22880_e17573: f64 = (assign22880_e17565 + assign22880_e17572);
        let assign22880_e17574: f64 = (assign22880_e17573).sqrt();
        locals.var_t2 = assign22880_e17574;
        locals.var_t2_dn0 = ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + ((((4.0 * locals.var_t0_dn0) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn0)) * 0.001)) / (2.0 * assign22880_e17574));
        locals.var_t2_dn2 = ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + ((((4.0 * locals.var_t0_dn2) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn2)) * 0.001)) / (2.0 * assign22880_e17574));
        locals.var_t2_dn4 = ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + ((((4.0 * locals.var_t0_dn4) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn4)) * 0.001)) / (2.0 * assign22880_e17574));
        locals.var_t2_dn5 = ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + ((((4.0 * locals.var_t0_dn5) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn5)) * 0.001)) / (2.0 * assign22880_e17574));
        locals.var_t2_dn6 = ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + ((((4.0 * locals.var_t0_dn6) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn6)) * 0.001)) / (2.0 * assign22880_e17574));
        locals.var_t2_dn7 = ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + ((((4.0 * locals.var_t0_dn7) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn7)) * 0.001)) / (2.0 * assign22880_e17574));
        locals.var_t2_dn8 = ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + ((((4.0 * locals.var_t0_dn8) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn8)) * 0.001)) / (2.0 * assign22880_e17574));
        locals.var_t2_dn9 = ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + ((((4.0 * locals.var_t0_dn9) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn9)) * 0.001)) / (2.0 * assign22880_e17574));
        locals.var_t2_dn10 = ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + ((((4.0 * locals.var_t0_dn10) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn10)) * 0.001)) / (2.0 * assign22880_e17574));
        locals.var_t2_dn11 = ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + ((((4.0 * locals.var_t0_dn11) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn11)) * 0.001)) / (2.0 * assign22880_e17574));
        locals.var_t2_dn14 = ((((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) + ((((4.0 * locals.var_t0_dn14) * locals.var_pb20b) + (assign22880_e17568 * locals.var_pb20b_dn14)) * 0.001)) / (2.0 * assign22880_e17574));

        let assign22890_e17577: f64 = (locals.var_t0 * locals.var_pb20b);
        let assign22890_e17581: f64 = (locals.var_t1 + locals.var_t2);
        let assign22890_e17582: f64 = (0.5 * assign22890_e17581);
        let assign22890_e17583: f64 = (assign22890_e17577 - assign22890_e17582);
        locals.var_t3 = assign22890_e17583;
        locals.var_t3_dn0 = (((locals.var_t0_dn0 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn0)) - (0.5 * (locals.var_t1_dn0 + locals.var_t2_dn0)));
        locals.var_t3_dn2 = (((locals.var_t0_dn2 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn2)) - (0.5 * (locals.var_t1_dn2 + locals.var_t2_dn2)));
        locals.var_t3_dn4 = (((locals.var_t0_dn4 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn4)) - (0.5 * (locals.var_t1_dn4 + locals.var_t2_dn4)));
        locals.var_t3_dn5 = (((locals.var_t0_dn5 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn5)) - (0.5 * (locals.var_t1_dn5 + locals.var_t2_dn5)));
        locals.var_t3_dn6 = (((locals.var_t0_dn6 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn6)) - (0.5 * (locals.var_t1_dn6 + locals.var_t2_dn6)));
        locals.var_t3_dn7 = (((locals.var_t0_dn7 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn7)) - (0.5 * (locals.var_t1_dn7 + locals.var_t2_dn7)));
        locals.var_t3_dn8 = (((locals.var_t0_dn8 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn8)) - (0.5 * (locals.var_t1_dn8 + locals.var_t2_dn8)));
        locals.var_t3_dn9 = (((locals.var_t0_dn9 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn9)) - (0.5 * (locals.var_t1_dn9 + locals.var_t2_dn9)));
        locals.var_t3_dn10 = (((locals.var_t0_dn10 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn10)) - (0.5 * (locals.var_t1_dn10 + locals.var_t2_dn10)));
        locals.var_t3_dn11 = (((locals.var_t0_dn11 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn11)) - (0.5 * (locals.var_t1_dn11 + locals.var_t2_dn11)));
        locals.var_t3_dn14 = (((locals.var_t0_dn14 * locals.var_pb20b) + (locals.var_t0 * locals.var_pb20b_dn14)) - (0.5 * (locals.var_t1_dn14 + locals.var_t2_dn14)));

        let (assign22900_e17591, assign22900_e17591_d_n0, assign22900_e17591_d_n2, assign22900_e17591_d_n4, assign22900_e17591_d_n5, assign22900_e17591_d_n6, assign22900_e17591_d_n7, assign22900_e17591_d_n8, assign22900_e17591_d_n9, assign22900_e17591_d_n10, assign22900_e17591_d_n11, assign22900_e17591_d_n14,) = {
    if (locals.var_uc_codep == 1.0) {
        let assign22900_e17589: f64 = (p.p366 * locals.var_vdsz);
        (assign22900_e17589, (p.p366 * locals.var_vdsz_dn0), (p.p366 * locals.var_vdsz_dn2), (p.p366 * locals.var_vdsz_dn4), (p.p366 * locals.var_vdsz_dn5), (p.p366 * locals.var_vdsz_dn6), (p.p366 * locals.var_vdsz_dn7), (p.p366 * locals.var_vdsz_dn8), (p.p366 * locals.var_vdsz_dn9), (p.p366 * locals.var_vdsz_dn10), (p.p366 * locals.var_vdsz_dn11), (p.p366 * locals.var_vdsz_dn14),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        locals.var_t5 = assign22900_e17591;
        locals.var_t5_dn0 = assign22900_e17591_d_n0;
        locals.var_t5_dn2 = assign22900_e17591_d_n2;
        locals.var_t5_dn4 = assign22900_e17591_d_n4;
        locals.var_t5_dn5 = assign22900_e17591_d_n5;
        locals.var_t5_dn6 = assign22900_e17591_d_n6;
        locals.var_t5_dn7 = assign22900_e17591_d_n7;
        locals.var_t5_dn8 = assign22900_e17591_d_n8;
        locals.var_t5_dn9 = assign22900_e17591_d_n9;
        locals.var_t5_dn10 = assign22900_e17591_d_n10;
        locals.var_t5_dn11 = assign22900_e17591_d_n11;
        locals.var_t5_dn14 = assign22900_e17591_d_n14;

        let assign22910_e17594: f64 = (locals.var_pb20b - locals.var_t3);
        let assign22910_e17596: f64 = (assign22910_e17594 + locals.var_t5);
        locals.var_pbsum = assign22910_e17596;
        locals.var_pbsum_dn0 = ((locals.var_pb20b_dn0 - locals.var_t3_dn0) + locals.var_t5_dn0);
        locals.var_pbsum_dn2 = ((locals.var_pb20b_dn2 - locals.var_t3_dn2) + locals.var_t5_dn2);
        locals.var_pbsum_dn4 = ((locals.var_pb20b_dn4 - locals.var_t3_dn4) + locals.var_t5_dn4);
        locals.var_pbsum_dn5 = ((locals.var_pb20b_dn5 - locals.var_t3_dn5) + locals.var_t5_dn5);
        locals.var_pbsum_dn6 = ((locals.var_pb20b_dn6 - locals.var_t3_dn6) + locals.var_t5_dn6);
        locals.var_pbsum_dn7 = ((locals.var_pb20b_dn7 - locals.var_t3_dn7) + locals.var_t5_dn7);
        locals.var_pbsum_dn8 = ((locals.var_pb20b_dn8 - locals.var_t3_dn8) + locals.var_t5_dn8);
        locals.var_pbsum_dn9 = ((locals.var_pb20b_dn9 - locals.var_t3_dn9) + locals.var_t5_dn9);
        locals.var_pbsum_dn10 = ((locals.var_pb20b_dn10 - locals.var_t3_dn10) + locals.var_t5_dn10);
        locals.var_pbsum_dn11 = ((locals.var_pb20b_dn11 - locals.var_t3_dn11) + locals.var_t5_dn11);
        locals.var_pbsum_dn14 = ((locals.var_pb20b_dn14 - locals.var_t3_dn14) + locals.var_t5_dn14);

        let assign22920_e17598: f64 = (locals.var_pbsum).sqrt();
        locals.var_sqrt_pbsum = assign22920_e17598;
        locals.var_sqrt_pbsum_dn0 = (locals.var_pbsum_dn0 / (2.0 * assign22920_e17598));
        locals.var_sqrt_pbsum_dn2 = (locals.var_pbsum_dn2 / (2.0 * assign22920_e17598));
        locals.var_sqrt_pbsum_dn4 = (locals.var_pbsum_dn4 / (2.0 * assign22920_e17598));
        locals.var_sqrt_pbsum_dn5 = (locals.var_pbsum_dn5 / (2.0 * assign22920_e17598));
        locals.var_sqrt_pbsum_dn6 = (locals.var_pbsum_dn6 / (2.0 * assign22920_e17598));
        locals.var_sqrt_pbsum_dn7 = (locals.var_pbsum_dn7 / (2.0 * assign22920_e17598));
        locals.var_sqrt_pbsum_dn8 = (locals.var_pbsum_dn8 / (2.0 * assign22920_e17598));
        locals.var_sqrt_pbsum_dn9 = (locals.var_pbsum_dn9 / (2.0 * assign22920_e17598));
        locals.var_sqrt_pbsum_dn10 = (locals.var_pbsum_dn10 / (2.0 * assign22920_e17598));
        locals.var_sqrt_pbsum_dn11 = (locals.var_pbsum_dn11 / (2.0 * assign22920_e17598));
        locals.var_sqrt_pbsum_dn14 = (locals.var_pbsum_dn14 / (2.0 * assign22920_e17598));

        let assign22930_e17601: f64 = if p.p140 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard432 = assign22930_e17601;

        let (assign22940_e17605, assign22940_e17605_d_n0, assign22940_e17605_d_n2, assign22940_e17605_d_n4, assign22940_e17605_d_n5, assign22940_e17605_d_n6, assign22940_e17605_d_n7, assign22940_e17605_d_n8, assign22940_e17605_d_n9, assign22940_e17605_d_n10, assign22940_e17605_d_n11, assign22940_e17605_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        (locals.var_qnsub_esi2, locals.var_qnsub_esi2_dn0, locals.var_qnsub_esi2_dn2, locals.var_qnsub_esi2_dn4, locals.var_qnsub_esi2_dn5, locals.var_qnsub_esi2_dn6, locals.var_qnsub_esi2_dn7, locals.var_qnsub_esi2_dn8, locals.var_qnsub_esi2_dn9, locals.var_qnsub_esi2_dn10, locals.var_qnsub_esi2_dn11, locals.var_qnsub_esi2_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign22940_e17605;
        locals.var_t1_dn0 = assign22940_e17605_d_n0;
        locals.var_t1_dn2 = assign22940_e17605_d_n2;
        locals.var_t1_dn4 = assign22940_e17605_d_n4;
        locals.var_t1_dn5 = assign22940_e17605_d_n5;
        locals.var_t1_dn6 = assign22940_e17605_d_n6;
        locals.var_t1_dn7 = assign22940_e17605_d_n7;
        locals.var_t1_dn8 = assign22940_e17605_d_n8;
        locals.var_t1_dn9 = assign22940_e17605_d_n9;
        locals.var_t1_dn10 = assign22940_e17605_d_n10;
        locals.var_t1_dn11 = assign22940_e17605_d_n11;
        locals.var_t1_dn14 = assign22940_e17605_d_n14;

        let (assign22950_e17611, assign22950_e17611_d_n0, assign22950_e17611_d_n2, assign22950_e17611_d_n4, assign22950_e17611_d_n5, assign22950_e17611_d_n6, assign22950_e17611_d_n7, assign22950_e17611_d_n8, assign22950_e17611_d_n9, assign22950_e17611_d_n10, assign22950_e17611_d_n11, assign22950_e17611_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign22950_e17609: f64 = (p.p224 - locals.var_vbsz2);
        (assign22950_e17609, (-locals.var_vbsz2_dn0), (-locals.var_vbsz2_dn2), (-locals.var_vbsz2_dn4), (-locals.var_vbsz2_dn5), (-locals.var_vbsz2_dn6), (-locals.var_vbsz2_dn7), (-locals.var_vbsz2_dn8), (-locals.var_vbsz2_dn9), (-locals.var_vbsz2_dn10), (-locals.var_vbsz2_dn11), (-locals.var_vbsz2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign22950_e17611;
        locals.var_t2_dn0 = assign22950_e17611_d_n0;
        locals.var_t2_dn2 = assign22950_e17611_d_n2;
        locals.var_t2_dn4 = assign22950_e17611_d_n4;
        locals.var_t2_dn5 = assign22950_e17611_d_n5;
        locals.var_t2_dn6 = assign22950_e17611_d_n6;
        locals.var_t2_dn7 = assign22950_e17611_d_n7;
        locals.var_t2_dn8 = assign22950_e17611_d_n8;
        locals.var_t2_dn9 = assign22950_e17611_d_n9;
        locals.var_t2_dn10 = assign22950_e17611_d_n10;
        locals.var_t2_dn11 = assign22950_e17611_d_n11;
        locals.var_t2_dn14 = assign22950_e17611_d_n14;

        let (assign22960_e17617, assign22960_e17617_d_n0, assign22960_e17617_d_n2, assign22960_e17617_d_n4, assign22960_e17617_d_n5, assign22960_e17617_d_n6, assign22960_e17617_d_n7, assign22960_e17617_d_n8, assign22960_e17617_d_n9, assign22960_e17617_d_n10, assign22960_e17617_d_n11, assign22960_e17617_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign22960_e17615: f64 = (locals.var_t2 + 1e-25);
        (assign22960_e17615, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign22960_e17617;
        locals.var_t3_dn0 = assign22960_e17617_d_n0;
        locals.var_t3_dn2 = assign22960_e17617_d_n2;
        locals.var_t3_dn4 = assign22960_e17617_d_n4;
        locals.var_t3_dn5 = assign22960_e17617_d_n5;
        locals.var_t3_dn6 = assign22960_e17617_d_n6;
        locals.var_t3_dn7 = assign22960_e17617_d_n7;
        locals.var_t3_dn8 = assign22960_e17617_d_n8;
        locals.var_t3_dn9 = assign22960_e17617_d_n9;
        locals.var_t3_dn10 = assign22960_e17617_d_n10;
        locals.var_t3_dn11 = assign22960_e17617_d_n11;
        locals.var_t3_dn14 = assign22960_e17617_d_n14;

        let (assign22970_e17628, assign22970_e17628_d_n0, assign22970_e17628_d_n2, assign22970_e17628_d_n4, assign22970_e17628_d_n5, assign22970_e17628_d_n6, assign22970_e17628_d_n7, assign22970_e17628_d_n8, assign22970_e17628_d_n9, assign22970_e17628_d_n10, assign22970_e17628_d_n11, assign22970_e17628_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign22970_e17621: f64 = (locals.var_t3 * locals.var_t3);
        let assign22970_e17624: f64 = (4.0 * 0.001);
        let assign22970_e17625: f64 = (assign22970_e17621 + assign22970_e17624);
        let assign22970_e17626: f64 = (assign22970_e17625).sqrt();
        (assign22970_e17626, (((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (2.0 * assign22970_e17626)), (((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (2.0 * assign22970_e17626)), (((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (2.0 * assign22970_e17626)), (((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (2.0 * assign22970_e17626)), (((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (2.0 * assign22970_e17626)), (((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) / (2.0 * assign22970_e17626)), (((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (2.0 * assign22970_e17626)), (((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) / (2.0 * assign22970_e17626)), (((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (2.0 * assign22970_e17626)), (((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)) / (2.0 * assign22970_e17626)), (((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14)) / (2.0 * assign22970_e17626)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign22970_e17628;
        locals.var_t4_dn0 = assign22970_e17628_d_n0;
        locals.var_t4_dn2 = assign22970_e17628_d_n2;
        locals.var_t4_dn4 = assign22970_e17628_d_n4;
        locals.var_t4_dn5 = assign22970_e17628_d_n5;
        locals.var_t4_dn6 = assign22970_e17628_d_n6;
        locals.var_t4_dn7 = assign22970_e17628_d_n7;
        locals.var_t4_dn8 = assign22970_e17628_d_n8;
        locals.var_t4_dn9 = assign22970_e17628_d_n9;
        locals.var_t4_dn10 = assign22970_e17628_d_n10;
        locals.var_t4_dn11 = assign22970_e17628_d_n11;
        locals.var_t4_dn14 = assign22970_e17628_d_n14;

        let (assign22980_e17636, assign22980_e17636_d_n0, assign22980_e17636_d_n2, assign22980_e17636_d_n4, assign22980_e17636_d_n5, assign22980_e17636_d_n6, assign22980_e17636_d_n7, assign22980_e17636_d_n8, assign22980_e17636_d_n9, assign22980_e17636_d_n10, assign22980_e17636_d_n11, assign22980_e17636_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign22980_e17633: f64 = (locals.var_t3 + locals.var_t4);
        let assign22980_e17634: f64 = (0.5 * assign22980_e17633);
        (assign22980_e17634, (0.5 * (locals.var_t3_dn0 + locals.var_t4_dn0)), (0.5 * (locals.var_t3_dn2 + locals.var_t4_dn2)), (0.5 * (locals.var_t3_dn4 + locals.var_t4_dn4)), (0.5 * (locals.var_t3_dn5 + locals.var_t4_dn5)), (0.5 * (locals.var_t3_dn6 + locals.var_t4_dn6)), (0.5 * (locals.var_t3_dn7 + locals.var_t4_dn7)), (0.5 * (locals.var_t3_dn8 + locals.var_t4_dn8)), (0.5 * (locals.var_t3_dn9 + locals.var_t4_dn9)), (0.5 * (locals.var_t3_dn10 + locals.var_t4_dn10)), (0.5 * (locals.var_t3_dn11 + locals.var_t4_dn11)), (0.5 * (locals.var_t3_dn14 + locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign22980_e17636;
        locals.var_t5_dn0 = assign22980_e17636_d_n0;
        locals.var_t5_dn2 = assign22980_e17636_d_n2;
        locals.var_t5_dn4 = assign22980_e17636_d_n4;
        locals.var_t5_dn5 = assign22980_e17636_d_n5;
        locals.var_t5_dn6 = assign22980_e17636_d_n6;
        locals.var_t5_dn7 = assign22980_e17636_d_n7;
        locals.var_t5_dn8 = assign22980_e17636_d_n8;
        locals.var_t5_dn9 = assign22980_e17636_d_n9;
        locals.var_t5_dn10 = assign22980_e17636_d_n10;
        locals.var_t5_dn11 = assign22980_e17636_d_n11;
        locals.var_t5_dn14 = assign22980_e17636_d_n14;

        let (assign22990_e17646, assign22990_e17646_d_n0, assign22990_e17646_d_n2, assign22990_e17646_d_n4, assign22990_e17646_d_n5, assign22990_e17646_d_n6, assign22990_e17646_d_n7, assign22990_e17646_d_n8, assign22990_e17646_d_n9, assign22990_e17646_d_n10, assign22990_e17646_d_n11, assign22990_e17646_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign22990_e17642: f64 = (locals.var_t3 / locals.var_t4);
        let assign22990_e17643: f64 = (1.0 + assign22990_e17642);
        let assign22990_e17644: f64 = (0.5 * assign22990_e17643);
        (assign22990_e17644, (0.5 * (((locals.var_t3_dn0 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn2 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4))), (0.5 * (((locals.var_t3_dn14 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign22990_e17646;
        locals.var_t6_dn0 = assign22990_e17646_d_n0;
        locals.var_t6_dn2 = assign22990_e17646_d_n2;
        locals.var_t6_dn4 = assign22990_e17646_d_n4;
        locals.var_t6_dn5 = assign22990_e17646_d_n5;
        locals.var_t6_dn6 = assign22990_e17646_d_n6;
        locals.var_t6_dn7 = assign22990_e17646_d_n7;
        locals.var_t6_dn8 = assign22990_e17646_d_n8;
        locals.var_t6_dn9 = assign22990_e17646_d_n9;
        locals.var_t6_dn10 = assign22990_e17646_d_n10;
        locals.var_t6_dn11 = assign22990_e17646_d_n11;
        locals.var_t6_dn14 = assign22990_e17646_d_n14;

        let (assign23000_e17652, assign23000_e17652_d_n0, assign23000_e17652_d_n2, assign23000_e17652_d_n4, assign23000_e17652_d_n5, assign23000_e17652_d_n6, assign23000_e17652_d_n7, assign23000_e17652_d_n8, assign23000_e17652_d_n9, assign23000_e17652_d_n10, assign23000_e17652_d_n11, assign23000_e17652_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23000_e17650: f64 = (1.0 / locals.var_t5);
        (assign23000_e17650, (-(locals.var_t5_dn0 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn2 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn4 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn5 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn6 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn7 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn8 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn9 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn10 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn11 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn14 / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign23000_e17652;
        locals.var_t7_dn0 = assign23000_e17652_d_n0;
        locals.var_t7_dn2 = assign23000_e17652_d_n2;
        locals.var_t7_dn4 = assign23000_e17652_d_n4;
        locals.var_t7_dn5 = assign23000_e17652_d_n5;
        locals.var_t7_dn6 = assign23000_e17652_d_n6;
        locals.var_t7_dn7 = assign23000_e17652_d_n7;
        locals.var_t7_dn8 = assign23000_e17652_d_n8;
        locals.var_t7_dn9 = assign23000_e17652_d_n9;
        locals.var_t7_dn10 = assign23000_e17652_d_n10;
        locals.var_t7_dn11 = assign23000_e17652_d_n11;
        locals.var_t7_dn14 = assign23000_e17652_d_n14;

        let (assign23010_e17658, assign23010_e17658_d_n0, assign23010_e17658_d_n2, assign23010_e17658_d_n4, assign23010_e17658_d_n5, assign23010_e17658_d_n6, assign23010_e17658_d_n7, assign23010_e17658_d_n8, assign23010_e17658_d_n9, assign23010_e17658_d_n10, assign23010_e17658_d_n11, assign23010_e17658_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23010_e17656: f64 = (p.p223 * locals.var_t7);
        (assign23010_e17656, (p.p223 * locals.var_t7_dn0), (p.p223 * locals.var_t7_dn2), (p.p223 * locals.var_t7_dn4), (p.p223 * locals.var_t7_dn5), (p.p223 * locals.var_t7_dn6), (p.p223 * locals.var_t7_dn7), (p.p223 * locals.var_t7_dn8), (p.p223 * locals.var_t7_dn9), (p.p223 * locals.var_t7_dn10), (p.p223 * locals.var_t7_dn11), (p.p223 * locals.var_t7_dn14),)
    } else {
        (locals.var_bs12, locals.var_bs12_dn0, locals.var_bs12_dn2, locals.var_bs12_dn4, locals.var_bs12_dn5, locals.var_bs12_dn6, locals.var_bs12_dn7, locals.var_bs12_dn8, locals.var_bs12_dn9, locals.var_bs12_dn10, locals.var_bs12_dn11, locals.var_bs12_dn14,)
    }
};
        locals.var_bs12 = assign23010_e17658;
        locals.var_bs12_dn0 = assign23010_e17658_d_n0;
        locals.var_bs12_dn2 = assign23010_e17658_d_n2;
        locals.var_bs12_dn4 = assign23010_e17658_d_n4;
        locals.var_bs12_dn5 = assign23010_e17658_d_n5;
        locals.var_bs12_dn6 = assign23010_e17658_d_n6;
        locals.var_bs12_dn7 = assign23010_e17658_d_n7;
        locals.var_bs12_dn8 = assign23010_e17658_d_n8;
        locals.var_bs12_dn9 = assign23010_e17658_d_n9;
        locals.var_bs12_dn10 = assign23010_e17658_d_n10;
        locals.var_bs12_dn11 = assign23010_e17658_d_n11;
        locals.var_bs12_dn14 = assign23010_e17658_d_n14;

        let (assign23020_e17665, assign23020_e17665_d_n0, assign23020_e17665_d_n2, assign23020_e17665_d_n4, assign23020_e17665_d_n5, assign23020_e17665_d_n6, assign23020_e17665_d_n7, assign23020_e17665_d_n8, assign23020_e17665_d_n9, assign23020_e17665_d_n10, assign23020_e17665_d_n11, assign23020_e17665_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23020_e17661: f64 = (-locals.var_bs12);
        let assign23020_e17663: f64 = (assign23020_e17661 * locals.var_t7);
        (assign23020_e17663, (((-locals.var_bs12_dn0) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn0)), (((-locals.var_bs12_dn2) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn2)), (((-locals.var_bs12_dn4) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn4)), (((-locals.var_bs12_dn5) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn5)), (((-locals.var_bs12_dn6) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn6)), (((-locals.var_bs12_dn7) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn7)), (((-locals.var_bs12_dn8) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn8)), (((-locals.var_bs12_dn9) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn9)), (((-locals.var_bs12_dn10) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn10)), (((-locals.var_bs12_dn11) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn11)), (((-locals.var_bs12_dn14) * locals.var_t7) + (assign23020_e17661 * locals.var_t7_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign23020_e17665;
        locals.var_t8_dn0 = assign23020_e17665_d_n0;
        locals.var_t8_dn2 = assign23020_e17665_d_n2;
        locals.var_t8_dn4 = assign23020_e17665_d_n4;
        locals.var_t8_dn5 = assign23020_e17665_d_n5;
        locals.var_t8_dn6 = assign23020_e17665_d_n6;
        locals.var_t8_dn7 = assign23020_e17665_d_n7;
        locals.var_t8_dn8 = assign23020_e17665_d_n8;
        locals.var_t8_dn9 = assign23020_e17665_d_n9;
        locals.var_t8_dn10 = assign23020_e17665_d_n10;
        locals.var_t8_dn11 = assign23020_e17665_d_n11;
        locals.var_t8_dn14 = assign23020_e17665_d_n14;

        let (assign23030_e17677, assign23030_e17677_d_n0, assign23030_e17677_d_n2, assign23030_e17677_d_n4, assign23030_e17677_d_n5, assign23030_e17677_d_n6, assign23030_e17677_d_n7, assign23030_e17677_d_n8, assign23030_e17677_d_n9, assign23030_e17677_d_n10, assign23030_e17677_d_n11, assign23030_e17677_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23030_e17669: f64 = (0.93 * locals.var_pb20);
        let assign23030_e17672: f64 = (locals.var_vbsz2 + locals.var_bs12);
        let assign23030_e17673: f64 = (assign23030_e17669 - assign23030_e17672);
        let assign23030_e17675: f64 = (assign23030_e17673 - 0.001);
        (assign23030_e17675, ((0.93 * locals.var_pb20_dn0) - (locals.var_vbsz2_dn0 + locals.var_bs12_dn0)), ((0.93 * locals.var_pb20_dn2) - (locals.var_vbsz2_dn2 + locals.var_bs12_dn2)), ((0.93 * locals.var_pb20_dn4) - (locals.var_vbsz2_dn4 + locals.var_bs12_dn4)), ((0.93 * locals.var_pb20_dn5) - (locals.var_vbsz2_dn5 + locals.var_bs12_dn5)), ((0.93 * locals.var_pb20_dn6) - (locals.var_vbsz2_dn6 + locals.var_bs12_dn6)), ((0.93 * locals.var_pb20_dn7) - (locals.var_vbsz2_dn7 + locals.var_bs12_dn7)), ((0.93 * locals.var_pb20_dn8) - (locals.var_vbsz2_dn8 + locals.var_bs12_dn8)), ((0.93 * locals.var_pb20_dn9) - (locals.var_vbsz2_dn9 + locals.var_bs12_dn9)), ((0.93 * locals.var_pb20_dn10) - (locals.var_vbsz2_dn10 + locals.var_bs12_dn10)), ((0.93 * locals.var_pb20_dn11) - (locals.var_vbsz2_dn11 + locals.var_bs12_dn11)), ((0.93 * locals.var_pb20_dn14) - (locals.var_vbsz2_dn14 + locals.var_bs12_dn14)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign23030_e17677;
        locals.var_tmf1_dn0 = assign23030_e17677_d_n0;
        locals.var_tmf1_dn2 = assign23030_e17677_d_n2;
        locals.var_tmf1_dn4 = assign23030_e17677_d_n4;
        locals.var_tmf1_dn5 = assign23030_e17677_d_n5;
        locals.var_tmf1_dn6 = assign23030_e17677_d_n6;
        locals.var_tmf1_dn7 = assign23030_e17677_d_n7;
        locals.var_tmf1_dn8 = assign23030_e17677_d_n8;
        locals.var_tmf1_dn9 = assign23030_e17677_d_n9;
        locals.var_tmf1_dn10 = assign23030_e17677_d_n10;
        locals.var_tmf1_dn11 = assign23030_e17677_d_n11;
        locals.var_tmf1_dn14 = assign23030_e17677_d_n14;

        let (assign23040_e17687, assign23040_e17687_d_n0, assign23040_e17687_d_n2, assign23040_e17687_d_n4, assign23040_e17687_d_n5, assign23040_e17687_d_n6, assign23040_e17687_d_n7, assign23040_e17687_d_n8, assign23040_e17687_d_n9, assign23040_e17687_d_n10, assign23040_e17687_d_n11, assign23040_e17687_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23040_e17682: f64 = (0.93 * locals.var_pb20);
        let assign23040_e17683: f64 = (4.0 * assign23040_e17682);
        let assign23040_e17685: f64 = (assign23040_e17683 * 0.001);
        (assign23040_e17685, ((4.0 * (0.93 * locals.var_pb20_dn0)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn2)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn4)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn5)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn6)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn7)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn8)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn9)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn10)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn11)) * 0.001), ((4.0 * (0.93 * locals.var_pb20_dn14)) * 0.001),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23040_e17687;
        locals.var_tmf2_dn0 = assign23040_e17687_d_n0;
        locals.var_tmf2_dn2 = assign23040_e17687_d_n2;
        locals.var_tmf2_dn4 = assign23040_e17687_d_n4;
        locals.var_tmf2_dn5 = assign23040_e17687_d_n5;
        locals.var_tmf2_dn6 = assign23040_e17687_d_n6;
        locals.var_tmf2_dn7 = assign23040_e17687_d_n7;
        locals.var_tmf2_dn8 = assign23040_e17687_d_n8;
        locals.var_tmf2_dn9 = assign23040_e17687_d_n9;
        locals.var_tmf2_dn10 = assign23040_e17687_d_n10;
        locals.var_tmf2_dn11 = assign23040_e17687_d_n11;
        locals.var_tmf2_dn14 = assign23040_e17687_d_n14;

        let (assign23050_e17697, assign23050_e17697_d_n0, assign23050_e17697_d_n2, assign23050_e17697_d_n4, assign23050_e17697_d_n5, assign23050_e17697_d_n6, assign23050_e17697_d_n7, assign23050_e17697_d_n8, assign23050_e17697_d_n9, assign23050_e17697_d_n10, assign23050_e17697_d_n11, assign23050_e17697_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let (assign23050_e17695, assign23050_e17695_d_n0, assign23050_e17695_d_n2, assign23050_e17695_d_n4, assign23050_e17695_d_n5, assign23050_e17695_d_n6, assign23050_e17695_d_n7, assign23050_e17695_d_n8, assign23050_e17695_d_n9, assign23050_e17695_d_n10, assign23050_e17695_d_n11, assign23050_e17695_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign23050_e17694: f64 = (-locals.var_tmf2);
                (assign23050_e17694, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign23050_e17695, assign23050_e17695_d_n0, assign23050_e17695_d_n2, assign23050_e17695_d_n4, assign23050_e17695_d_n5, assign23050_e17695_d_n6, assign23050_e17695_d_n7, assign23050_e17695_d_n8, assign23050_e17695_d_n9, assign23050_e17695_d_n10, assign23050_e17695_d_n11, assign23050_e17695_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23050_e17697;
        locals.var_tmf2_dn0 = assign23050_e17697_d_n0;
        locals.var_tmf2_dn2 = assign23050_e17697_d_n2;
        locals.var_tmf2_dn4 = assign23050_e17697_d_n4;
        locals.var_tmf2_dn5 = assign23050_e17697_d_n5;
        locals.var_tmf2_dn6 = assign23050_e17697_d_n6;
        locals.var_tmf2_dn7 = assign23050_e17697_d_n7;
        locals.var_tmf2_dn8 = assign23050_e17697_d_n8;
        locals.var_tmf2_dn9 = assign23050_e17697_d_n9;
        locals.var_tmf2_dn10 = assign23050_e17697_d_n10;
        locals.var_tmf2_dn11 = assign23050_e17697_d_n11;
        locals.var_tmf2_dn14 = assign23050_e17697_d_n14;

        let (assign23060_e17706, assign23060_e17706_d_n0, assign23060_e17706_d_n2, assign23060_e17706_d_n4, assign23060_e17706_d_n5, assign23060_e17706_d_n6, assign23060_e17706_d_n7, assign23060_e17706_d_n8, assign23060_e17706_d_n9, assign23060_e17706_d_n10, assign23060_e17706_d_n11, assign23060_e17706_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23060_e17701: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign23060_e17703: f64 = (assign23060_e17701 + locals.var_tmf2);
        let assign23060_e17704: f64 = (assign23060_e17703).sqrt();
        (assign23060_e17704, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign23060_e17704)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign23060_e17704)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign23060_e17704)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign23060_e17704)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign23060_e17704)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign23060_e17704)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign23060_e17704)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign23060_e17704)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign23060_e17704)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign23060_e17704)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign23060_e17704)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23060_e17706;
        locals.var_tmf2_dn0 = assign23060_e17706_d_n0;
        locals.var_tmf2_dn2 = assign23060_e17706_d_n2;
        locals.var_tmf2_dn4 = assign23060_e17706_d_n4;
        locals.var_tmf2_dn5 = assign23060_e17706_d_n5;
        locals.var_tmf2_dn6 = assign23060_e17706_d_n6;
        locals.var_tmf2_dn7 = assign23060_e17706_d_n7;
        locals.var_tmf2_dn8 = assign23060_e17706_d_n8;
        locals.var_tmf2_dn9 = assign23060_e17706_d_n9;
        locals.var_tmf2_dn10 = assign23060_e17706_d_n10;
        locals.var_tmf2_dn11 = assign23060_e17706_d_n11;
        locals.var_tmf2_dn14 = assign23060_e17706_d_n14;

    }

    pub(super) fn stamp_transient_block_59(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23070_e17716, assign23070_e17716_d_n0, assign23070_e17716_d_n2, assign23070_e17716_d_n4, assign23070_e17716_d_n5, assign23070_e17716_d_n6, assign23070_e17716_d_n7, assign23070_e17716_d_n8, assign23070_e17716_d_n9, assign23070_e17716_d_n10, assign23070_e17716_d_n11, assign23070_e17716_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23070_e17712: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign23070_e17713: f64 = (1.0 + assign23070_e17712);
        let assign23070_e17714: f64 = (0.5 * assign23070_e17713);
        (assign23070_e17714, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign23070_e17716;
        locals.var_t0_dn0 = assign23070_e17716_d_n0;
        locals.var_t0_dn2 = assign23070_e17716_d_n2;
        locals.var_t0_dn4 = assign23070_e17716_d_n4;
        locals.var_t0_dn5 = assign23070_e17716_d_n5;
        locals.var_t0_dn6 = assign23070_e17716_d_n6;
        locals.var_t0_dn7 = assign23070_e17716_d_n7;
        locals.var_t0_dn8 = assign23070_e17716_d_n8;
        locals.var_t0_dn9 = assign23070_e17716_d_n9;
        locals.var_t0_dn10 = assign23070_e17716_d_n10;
        locals.var_t0_dn11 = assign23070_e17716_d_n11;
        locals.var_t0_dn14 = assign23070_e17716_d_n14;

        let (assign23080_e17728, assign23080_e17728_d_n0, assign23080_e17728_d_n2, assign23080_e17728_d_n4, assign23080_e17728_d_n5, assign23080_e17728_d_n6, assign23080_e17728_d_n7, assign23080_e17728_d_n8, assign23080_e17728_d_n9, assign23080_e17728_d_n10, assign23080_e17728_d_n11, assign23080_e17728_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23080_e17720: f64 = (0.93 * locals.var_pb20);
        let assign23080_e17724: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign23080_e17725: f64 = (0.5 * assign23080_e17724);
        let assign23080_e17726: f64 = (assign23080_e17720 - assign23080_e17725);
        (assign23080_e17726, ((0.93 * locals.var_pb20_dn0) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((0.93 * locals.var_pb20_dn2) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((0.93 * locals.var_pb20_dn4) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((0.93 * locals.var_pb20_dn5) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((0.93 * locals.var_pb20_dn6) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((0.93 * locals.var_pb20_dn7) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((0.93 * locals.var_pb20_dn8) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((0.93 * locals.var_pb20_dn9) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((0.93 * locals.var_pb20_dn10) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((0.93 * locals.var_pb20_dn11) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((0.93 * locals.var_pb20_dn14) - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t10, locals.var_t10_dn0, locals.var_t10_dn2, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11, locals.var_t10_dn14,)
    }
};
        locals.var_t10 = assign23080_e17728;
        locals.var_t10_dn0 = assign23080_e17728_d_n0;
        locals.var_t10_dn2 = assign23080_e17728_d_n2;
        locals.var_t10_dn4 = assign23080_e17728_d_n4;
        locals.var_t10_dn5 = assign23080_e17728_d_n5;
        locals.var_t10_dn6 = assign23080_e17728_d_n6;
        locals.var_t10_dn7 = assign23080_e17728_d_n7;
        locals.var_t10_dn8 = assign23080_e17728_d_n8;
        locals.var_t10_dn9 = assign23080_e17728_d_n9;
        locals.var_t10_dn10 = assign23080_e17728_d_n10;
        locals.var_t10_dn11 = assign23080_e17728_d_n11;
        locals.var_t10_dn14 = assign23080_e17728_d_n14;

        let (assign23090_e17737, assign23090_e17737_d_n0, assign23090_e17737_d_n2, assign23090_e17737_d_n4, assign23090_e17737_d_n5, assign23090_e17737_d_n6, assign23090_e17737_d_n7, assign23090_e17737_d_n8, assign23090_e17737_d_n9, assign23090_e17737_d_n10, assign23090_e17737_d_n11, assign23090_e17737_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23090_e17733: f64 = (locals.var_pb20 - locals.var_t10);
        let assign23090_e17734: f64 = (locals.var_t1 * assign23090_e17733);
        let assign23090_e17735: f64 = (assign23090_e17734).sqrt();
        (assign23090_e17735, (((locals.var_t1_dn0 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn0 - locals.var_t10_dn0))) / (2.0 * assign23090_e17735)), (((locals.var_t1_dn2 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn2 - locals.var_t10_dn2))) / (2.0 * assign23090_e17735)), (((locals.var_t1_dn4 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn4 - locals.var_t10_dn4))) / (2.0 * assign23090_e17735)), (((locals.var_t1_dn5 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn5 - locals.var_t10_dn5))) / (2.0 * assign23090_e17735)), (((locals.var_t1_dn6 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn6 - locals.var_t10_dn6))) / (2.0 * assign23090_e17735)), (((locals.var_t1_dn7 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn7 - locals.var_t10_dn7))) / (2.0 * assign23090_e17735)), (((locals.var_t1_dn8 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn8 - locals.var_t10_dn8))) / (2.0 * assign23090_e17735)), (((locals.var_t1_dn9 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn9 - locals.var_t10_dn9))) / (2.0 * assign23090_e17735)), (((locals.var_t1_dn10 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn10 - locals.var_t10_dn10))) / (2.0 * assign23090_e17735)), (((locals.var_t1_dn11 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn11 - locals.var_t10_dn11))) / (2.0 * assign23090_e17735)), (((locals.var_t1_dn14 * assign23090_e17733) + (locals.var_t1 * (locals.var_pb20_dn14 - locals.var_t10_dn14))) / (2.0 * assign23090_e17735)),)
    } else {
        (locals.var_qbmm, locals.var_qbmm_dn0, locals.var_qbmm_dn2, locals.var_qbmm_dn4, locals.var_qbmm_dn5, locals.var_qbmm_dn6, locals.var_qbmm_dn7, locals.var_qbmm_dn8, locals.var_qbmm_dn9, locals.var_qbmm_dn10, locals.var_qbmm_dn11, locals.var_qbmm_dn14,)
    }
};
        locals.var_qbmm = assign23090_e17737;
        locals.var_qbmm_dn0 = assign23090_e17737_d_n0;
        locals.var_qbmm_dn2 = assign23090_e17737_d_n2;
        locals.var_qbmm_dn4 = assign23090_e17737_d_n4;
        locals.var_qbmm_dn5 = assign23090_e17737_d_n5;
        locals.var_qbmm_dn6 = assign23090_e17737_d_n6;
        locals.var_qbmm_dn7 = assign23090_e17737_d_n7;
        locals.var_qbmm_dn8 = assign23090_e17737_d_n8;
        locals.var_qbmm_dn9 = assign23090_e17737_d_n9;
        locals.var_qbmm_dn10 = assign23090_e17737_d_n10;
        locals.var_qbmm_dn11 = assign23090_e17737_d_n11;
        locals.var_qbmm_dn14 = assign23090_e17737_d_n14;

        let (assign23100_e17743, assign23100_e17743_d_n0, assign23100_e17743_d_n2, assign23100_e17743_d_n4, assign23100_e17743_d_n5, assign23100_e17743_d_n6, assign23100_e17743_d_n7, assign23100_e17743_d_n8, assign23100_e17743_d_n9, assign23100_e17743_d_n10, assign23100_e17743_d_n11, assign23100_e17743_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23100_e17741: f64 = (locals.var_t0 / locals.var_qbmm);
        (assign23100_e17741, (((locals.var_t0_dn0 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn0)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn2 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn2)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn4 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn4)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn5 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn5)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn6 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn6)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn7 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn7)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn8 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn8)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn9 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn9)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn10 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn10)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn11 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn11)) / (locals.var_qbmm * locals.var_qbmm)), (((locals.var_t0_dn14 * locals.var_qbmm) - (locals.var_t0 * locals.var_qbmm_dn14)) / (locals.var_qbmm * locals.var_qbmm)),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign23100_e17743;
        locals.var_t9_dn0 = assign23100_e17743_d_n0;
        locals.var_t9_dn2 = assign23100_e17743_d_n2;
        locals.var_t9_dn4 = assign23100_e17743_d_n4;
        locals.var_t9_dn5 = assign23100_e17743_d_n5;
        locals.var_t9_dn6 = assign23100_e17743_d_n6;
        locals.var_t9_dn7 = assign23100_e17743_d_n7;
        locals.var_t9_dn8 = assign23100_e17743_d_n8;
        locals.var_t9_dn9 = assign23100_e17743_d_n9;
        locals.var_t9_dn10 = assign23100_e17743_d_n10;
        locals.var_t9_dn11 = assign23100_e17743_d_n11;
        locals.var_t9_dn14 = assign23100_e17743_d_n14;

        let (assign23110_e17751, assign23110_e17751_d_n0, assign23110_e17751_d_n2, assign23110_e17751_d_n4, assign23110_e17751_d_n5, assign23110_e17751_d_n6, assign23110_e17751_d_n7, assign23110_e17751_d_n8, assign23110_e17751_d_n9, assign23110_e17751_d_n10, assign23110_e17751_d_n11, assign23110_e17751_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23110_e17747: f64 = (locals.var_qb0 - locals.var_qbmm);
        let assign23110_e17749: f64 = (assign23110_e17747 * locals.var_cox_inv);
        (assign23110_e17749, (((locals.var_qb0_dn0 - locals.var_qbmm_dn0) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn0)), (((locals.var_qb0_dn2 - locals.var_qbmm_dn2) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn2)), (((locals.var_qb0_dn4 - locals.var_qbmm_dn4) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn4)), (((locals.var_qb0_dn5 - locals.var_qbmm_dn5) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn5)), (((locals.var_qb0_dn6 - locals.var_qbmm_dn6) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn6)), (((locals.var_qb0_dn7 - locals.var_qbmm_dn7) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn7)), (((locals.var_qb0_dn8 - locals.var_qbmm_dn8) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn8)), (((locals.var_qb0_dn9 - locals.var_qbmm_dn9) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn9)), (((locals.var_qb0_dn10 - locals.var_qbmm_dn10) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn10)), (((locals.var_qb0_dn11 - locals.var_qbmm_dn11) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn11)), (((locals.var_qb0_dn14 - locals.var_qbmm_dn14) * locals.var_cox_inv) + (assign23110_e17747 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_dqb, locals.var_dqb_dn0, locals.var_dqb_dn2, locals.var_dqb_dn4, locals.var_dqb_dn5, locals.var_dqb_dn6, locals.var_dqb_dn7, locals.var_dqb_dn8, locals.var_dqb_dn9, locals.var_dqb_dn10, locals.var_dqb_dn11, locals.var_dqb_dn14,)
    }
};
        locals.var_dqb = assign23110_e17751;
        locals.var_dqb_dn0 = assign23110_e17751_d_n0;
        locals.var_dqb_dn2 = assign23110_e17751_d_n2;
        locals.var_dqb_dn4 = assign23110_e17751_d_n4;
        locals.var_dqb_dn5 = assign23110_e17751_d_n5;
        locals.var_dqb_dn6 = assign23110_e17751_d_n6;
        locals.var_dqb_dn7 = assign23110_e17751_d_n7;
        locals.var_dqb_dn8 = assign23110_e17751_d_n8;
        locals.var_dqb_dn9 = assign23110_e17751_d_n9;
        locals.var_dqb_dn10 = assign23110_e17751_d_n10;
        locals.var_dqb_dn11 = assign23110_e17751_d_n11;
        locals.var_dqb_dn14 = assign23110_e17751_d_n14;

        let (assign23120_e17761, assign23120_e17761_d_n0, assign23120_e17761_d_n2, assign23120_e17761_d_n4, assign23120_e17761_d_n5, assign23120_e17761_d_n6, assign23120_e17761_d_n7, assign23120_e17761_d_n8, assign23120_e17761_d_n9, assign23120_e17761_d_n10, assign23120_e17761_d_n11, assign23120_e17761_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23120_e17755: f64 = (2.0 * 1.6021918e-19);
        let assign23120_e17757: f64 = (assign23120_e17755 * locals.var_ef_nsubc);
        let assign23120_e17759: f64 = (assign23120_e17757 * 1.034943e-10);
        (assign23120_e17759, ((assign23120_e17755 * locals.var_ef_nsubc_dn0) * 1.034943e-10), ((assign23120_e17755 * locals.var_ef_nsubc_dn2) * 1.034943e-10), ((assign23120_e17755 * locals.var_ef_nsubc_dn4) * 1.034943e-10), ((assign23120_e17755 * locals.var_ef_nsubc_dn5) * 1.034943e-10), ((assign23120_e17755 * locals.var_ef_nsubc_dn6) * 1.034943e-10), ((assign23120_e17755 * locals.var_ef_nsubc_dn7) * 1.034943e-10), ((assign23120_e17755 * locals.var_ef_nsubc_dn8) * 1.034943e-10), ((assign23120_e17755 * locals.var_ef_nsubc_dn9) * 1.034943e-10), ((assign23120_e17755 * locals.var_ef_nsubc_dn10) * 1.034943e-10), ((assign23120_e17755 * locals.var_ef_nsubc_dn11) * 1.034943e-10), ((assign23120_e17755 * locals.var_ef_nsubc_dn14) * 1.034943e-10),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23120_e17761;
        locals.var_t1_dn0 = assign23120_e17761_d_n0;
        locals.var_t1_dn2 = assign23120_e17761_d_n2;
        locals.var_t1_dn4 = assign23120_e17761_d_n4;
        locals.var_t1_dn5 = assign23120_e17761_d_n5;
        locals.var_t1_dn6 = assign23120_e17761_d_n6;
        locals.var_t1_dn7 = assign23120_e17761_d_n7;
        locals.var_t1_dn8 = assign23120_e17761_d_n8;
        locals.var_t1_dn9 = assign23120_e17761_d_n9;
        locals.var_t1_dn10 = assign23120_e17761_d_n10;
        locals.var_t1_dn11 = assign23120_e17761_d_n11;
        locals.var_t1_dn14 = assign23120_e17761_d_n14;

        let (assign23130_e17770, assign23130_e17770_d_n0, assign23130_e17770_d_n2, assign23130_e17770_d_n4, assign23130_e17770_d_n5, assign23130_e17770_d_n6, assign23130_e17770_d_n7, assign23130_e17770_d_n8, assign23130_e17770_d_n9, assign23130_e17770_d_n10, assign23130_e17770_d_n11, assign23130_e17770_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23130_e17766: f64 = (locals.var_pb2c - locals.var_vbsz2);
        let assign23130_e17767: f64 = (locals.var_t1 * assign23130_e17766);
        let assign23130_e17768: f64 = (assign23130_e17767).sqrt();
        (assign23130_e17768, (((locals.var_t1_dn0 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn0 - locals.var_vbsz2_dn0))) / (2.0 * assign23130_e17768)), (((locals.var_t1_dn2 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn2 - locals.var_vbsz2_dn2))) / (2.0 * assign23130_e17768)), (((locals.var_t1_dn4 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn4 - locals.var_vbsz2_dn4))) / (2.0 * assign23130_e17768)), (((locals.var_t1_dn5 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn5 - locals.var_vbsz2_dn5))) / (2.0 * assign23130_e17768)), (((locals.var_t1_dn6 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn6 - locals.var_vbsz2_dn6))) / (2.0 * assign23130_e17768)), (((locals.var_t1_dn7 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn7 - locals.var_vbsz2_dn7))) / (2.0 * assign23130_e17768)), (((locals.var_t1_dn8 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn8 - locals.var_vbsz2_dn8))) / (2.0 * assign23130_e17768)), (((locals.var_t1_dn9 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn9 - locals.var_vbsz2_dn9))) / (2.0 * assign23130_e17768)), (((locals.var_t1_dn10 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn10 - locals.var_vbsz2_dn10))) / (2.0 * assign23130_e17768)), (((locals.var_t1_dn11 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn11 - locals.var_vbsz2_dn11))) / (2.0 * assign23130_e17768)), (((locals.var_t1_dn14 * assign23130_e17766) + (locals.var_t1 * (locals.var_pb2c_dn14 - locals.var_vbsz2_dn14))) / (2.0 * assign23130_e17768)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign23130_e17770;
        locals.var_t2_dn0 = assign23130_e17770_d_n0;
        locals.var_t2_dn2 = assign23130_e17770_d_n2;
        locals.var_t2_dn4 = assign23130_e17770_d_n4;
        locals.var_t2_dn5 = assign23130_e17770_d_n5;
        locals.var_t2_dn6 = assign23130_e17770_d_n6;
        locals.var_t2_dn7 = assign23130_e17770_d_n7;
        locals.var_t2_dn8 = assign23130_e17770_d_n8;
        locals.var_t2_dn9 = assign23130_e17770_d_n9;
        locals.var_t2_dn10 = assign23130_e17770_d_n10;
        locals.var_t2_dn11 = assign23130_e17770_d_n11;
        locals.var_t2_dn14 = assign23130_e17770_d_n14;

        let (assign23140_e17780, assign23140_e17780_d_n0, assign23140_e17780_d_n2, assign23140_e17780_d_n4, assign23140_e17780_d_n5, assign23140_e17780_d_n6, assign23140_e17780_d_n7, assign23140_e17780_d_n8, assign23140_e17780_d_n9, assign23140_e17780_d_n10, assign23140_e17780_d_n11, assign23140_e17780_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23140_e17774: f64 = (locals.var_pb2c + locals.var_vfb);
        let assign23140_e17777: f64 = (locals.var_t2 * locals.var_cox_inv);
        let assign23140_e17778: f64 = (assign23140_e17774 + assign23140_e17777);
        (assign23140_e17778, (locals.var_pb2c_dn0 + ((locals.var_t2_dn0 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn0))), (locals.var_pb2c_dn2 + ((locals.var_t2_dn2 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn2))), (locals.var_pb2c_dn4 + ((locals.var_t2_dn4 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn4))), (locals.var_pb2c_dn5 + ((locals.var_t2_dn5 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn5))), (locals.var_pb2c_dn6 + ((locals.var_t2_dn6 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn6))), (locals.var_pb2c_dn7 + ((locals.var_t2_dn7 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn7))), (locals.var_pb2c_dn8 + ((locals.var_t2_dn8 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn8))), (locals.var_pb2c_dn9 + ((locals.var_t2_dn9 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn9))), (locals.var_pb2c_dn10 + ((locals.var_t2_dn10 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn10))), (locals.var_pb2c_dn11 + ((locals.var_t2_dn11 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn11))), (locals.var_pb2c_dn14 + ((locals.var_t2_dn14 * locals.var_cox_inv) + (locals.var_t2 * locals.var_cox_inv_dn14))),)
    } else {
        (locals.var_vth0, locals.var_vth0_dn0, locals.var_vth0_dn2, locals.var_vth0_dn4, locals.var_vth0_dn5, locals.var_vth0_dn6, locals.var_vth0_dn7, locals.var_vth0_dn8, locals.var_vth0_dn9, locals.var_vth0_dn10, locals.var_vth0_dn11, locals.var_vth0_dn14,)
    }
};
        locals.var_vth0 = assign23140_e17780;
        locals.var_vth0_dn0 = assign23140_e17780_d_n0;
        locals.var_vth0_dn2 = assign23140_e17780_d_n2;
        locals.var_vth0_dn4 = assign23140_e17780_d_n4;
        locals.var_vth0_dn5 = assign23140_e17780_d_n5;
        locals.var_vth0_dn6 = assign23140_e17780_d_n6;
        locals.var_vth0_dn7 = assign23140_e17780_d_n7;
        locals.var_vth0_dn8 = assign23140_e17780_d_n8;
        locals.var_vth0_dn9 = assign23140_e17780_d_n9;
        locals.var_vth0_dn10 = assign23140_e17780_d_n10;
        locals.var_vth0_dn11 = assign23140_e17780_d_n11;
        locals.var_vth0_dn14 = assign23140_e17780_d_n14;

        let (assign23150_e17790, assign23150_e17790_d_n0, assign23150_e17790_d_n2, assign23150_e17790_d_n4, assign23150_e17790_d_n5, assign23150_e17790_d_n6, assign23150_e17790_d_n7, assign23150_e17790_d_n8, assign23150_e17790_d_n9, assign23150_e17790_d_n10, assign23150_e17790_d_n11, assign23150_e17790_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23150_e17784: f64 = (0.5 * locals.var_t1);
        let assign23150_e17786: f64 = (assign23150_e17784 / locals.var_t2);
        let assign23150_e17788: f64 = (assign23150_e17786 * locals.var_cox_inv);
        (assign23150_e17788, ((((((0.5 * locals.var_t1_dn0) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn0)), ((((((0.5 * locals.var_t1_dn2) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn2)), ((((((0.5 * locals.var_t1_dn4) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn4)), ((((((0.5 * locals.var_t1_dn5) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn5)), ((((((0.5 * locals.var_t1_dn6) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn6)), ((((((0.5 * locals.var_t1_dn7) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn7)), ((((((0.5 * locals.var_t1_dn8) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn8)), ((((((0.5 * locals.var_t1_dn9) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn9)), ((((((0.5 * locals.var_t1_dn10) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn10)), ((((((0.5 * locals.var_t1_dn11) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn11)), ((((((0.5 * locals.var_t1_dn14) * locals.var_t2) - (assign23150_e17784 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)) * locals.var_cox_inv) + (assign23150_e17786 * locals.var_cox_inv_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign23150_e17790;
        locals.var_t3_dn0 = assign23150_e17790_d_n0;
        locals.var_t3_dn2 = assign23150_e17790_d_n2;
        locals.var_t3_dn4 = assign23150_e17790_d_n4;
        locals.var_t3_dn5 = assign23150_e17790_d_n5;
        locals.var_t3_dn6 = assign23150_e17790_d_n6;
        locals.var_t3_dn7 = assign23150_e17790_d_n7;
        locals.var_t3_dn8 = assign23150_e17790_d_n8;
        locals.var_t3_dn9 = assign23150_e17790_d_n9;
        locals.var_t3_dn10 = assign23150_e17790_d_n10;
        locals.var_t3_dn11 = assign23150_e17790_d_n11;
        locals.var_t3_dn14 = assign23150_e17790_d_n14;

        let (assign23160_e17796, assign23160_e17796_d_n0, assign23160_e17796_d_n2, assign23160_e17796_d_n4, assign23160_e17796_d_n5, assign23160_e17796_d_n6, assign23160_e17796_d_n7, assign23160_e17796_d_n8, assign23160_e17796_d_n9, assign23160_e17796_d_n10, assign23160_e17796_d_n11, assign23160_e17796_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23160_e17794: f64 = (1.034943e-10 * locals.var_cox_inv);
        (assign23160_e17794, (1.034943e-10 * locals.var_cox_inv_dn0), (1.034943e-10 * locals.var_cox_inv_dn2), (1.034943e-10 * locals.var_cox_inv_dn4), (1.034943e-10 * locals.var_cox_inv_dn5), (1.034943e-10 * locals.var_cox_inv_dn6), (1.034943e-10 * locals.var_cox_inv_dn7), (1.034943e-10 * locals.var_cox_inv_dn8), (1.034943e-10 * locals.var_cox_inv_dn9), (1.034943e-10 * locals.var_cox_inv_dn10), (1.034943e-10 * locals.var_cox_inv_dn11), (1.034943e-10 * locals.var_cox_inv_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23160_e17796;
        locals.var_t1_dn0 = assign23160_e17796_d_n0;
        locals.var_t1_dn2 = assign23160_e17796_d_n2;
        locals.var_t1_dn4 = assign23160_e17796_d_n4;
        locals.var_t1_dn5 = assign23160_e17796_d_n5;
        locals.var_t1_dn6 = assign23160_e17796_d_n6;
        locals.var_t1_dn7 = assign23160_e17796_d_n7;
        locals.var_t1_dn8 = assign23160_e17796_d_n8;
        locals.var_t1_dn9 = assign23160_e17796_d_n9;
        locals.var_t1_dn10 = assign23160_e17796_d_n10;
        locals.var_t1_dn11 = assign23160_e17796_d_n11;
        locals.var_t1_dn14 = assign23160_e17796_d_n14;

        let (assign23170_e17800, assign23170_e17800_d_n0, assign23170_e17800_d_n2, assign23170_e17800_d_n4, assign23170_e17800_d_n5, assign23170_e17800_d_n6, assign23170_e17800_d_n7, assign23170_e17800_d_n8, assign23170_e17800_d_n9, assign23170_e17800_d_n10, assign23170_e17800_d_n11, assign23170_e17800_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        (locals.var_wdplp, locals.var_wdplp_dn0, locals.var_wdplp_dn2, locals.var_wdplp_dn4, locals.var_wdplp_dn5, locals.var_wdplp_dn6, locals.var_wdplp_dn7, locals.var_wdplp_dn8, locals.var_wdplp_dn9, locals.var_wdplp_dn10, locals.var_wdplp_dn11, locals.var_wdplp_dn14,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign23170_e17800;
        locals.var_t2_dn0 = assign23170_e17800_d_n0;
        locals.var_t2_dn2 = assign23170_e17800_d_n2;
        locals.var_t2_dn4 = assign23170_e17800_d_n4;
        locals.var_t2_dn5 = assign23170_e17800_d_n5;
        locals.var_t2_dn6 = assign23170_e17800_d_n6;
        locals.var_t2_dn7 = assign23170_e17800_d_n7;
        locals.var_t2_dn8 = assign23170_e17800_d_n8;
        locals.var_t2_dn9 = assign23170_e17800_d_n9;
        locals.var_t2_dn10 = assign23170_e17800_d_n10;
        locals.var_t2_dn11 = assign23170_e17800_d_n11;
        locals.var_t2_dn14 = assign23170_e17800_d_n14;

        let (assign23180_e17808, assign23180_e17808_d_n0, assign23180_e17808_d_n2, assign23180_e17808_d_n4, assign23180_e17808_d_n5, assign23180_e17808_d_n6, assign23180_e17808_d_n7, assign23180_e17808_d_n8, assign23180_e17808_d_n9, assign23180_e17808_d_n10, assign23180_e17808_d_n11, assign23180_e17808_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23180_e17805: f64 = (p.p140 * p.p140);
        let assign23180_e17806: f64 = (1.0 / assign23180_e17805);
        (assign23180_e17806, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign23180_e17808;
        locals.var_t4_dn0 = assign23180_e17808_d_n0;
        locals.var_t4_dn2 = assign23180_e17808_d_n2;
        locals.var_t4_dn4 = assign23180_e17808_d_n4;
        locals.var_t4_dn5 = assign23180_e17808_d_n5;
        locals.var_t4_dn6 = assign23180_e17808_d_n6;
        locals.var_t4_dn7 = assign23180_e17808_d_n7;
        locals.var_t4_dn8 = assign23180_e17808_d_n8;
        locals.var_t4_dn9 = assign23180_e17808_d_n9;
        locals.var_t4_dn10 = assign23180_e17808_d_n10;
        locals.var_t4_dn11 = assign23180_e17808_d_n11;
        locals.var_t4_dn14 = assign23180_e17808_d_n14;

        let (assign23190_e17822, assign23190_e17822_d_n0, assign23190_e17822_d_n2, assign23190_e17822_d_n4, assign23190_e17822_d_n5, assign23190_e17822_d_n6, assign23190_e17822_d_n7, assign23190_e17822_d_n8, assign23190_e17822_d_n9, assign23190_e17822_d_n10, assign23190_e17822_d_n11, assign23190_e17822_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23190_e17813: f64 = (p.p137 - locals.var_pb20b);
        let assign23190_e17814: f64 = (2.0 * assign23190_e17813);
        let assign23190_e17816: f64 = (assign23190_e17814 * locals.var_t1);
        let assign23190_e17818: f64 = (assign23190_e17816 * locals.var_t2);
        let assign23190_e17820: f64 = (assign23190_e17818 * locals.var_t4);
        (assign23190_e17820, (((((((2.0 * (-locals.var_pb20b_dn0)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn0)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn0)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn0)), (((((((2.0 * (-locals.var_pb20b_dn2)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn2)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn2)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn2)), (((((((2.0 * (-locals.var_pb20b_dn4)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn4)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn4)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn4)), (((((((2.0 * (-locals.var_pb20b_dn5)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn5)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn5)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn5)), (((((((2.0 * (-locals.var_pb20b_dn6)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn6)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn6)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn6)), (((((((2.0 * (-locals.var_pb20b_dn7)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn7)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn7)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn7)), (((((((2.0 * (-locals.var_pb20b_dn8)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn8)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn8)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn8)), (((((((2.0 * (-locals.var_pb20b_dn9)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn9)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn9)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn9)), (((((((2.0 * (-locals.var_pb20b_dn10)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn10)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn10)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn10)), (((((((2.0 * (-locals.var_pb20b_dn11)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn11)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn11)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn11)), (((((((2.0 * (-locals.var_pb20b_dn14)) * locals.var_t1) + (assign23190_e17814 * locals.var_t1_dn14)) * locals.var_t2) + (assign23190_e17816 * locals.var_t2_dn14)) * locals.var_t4) + (assign23190_e17818 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign23190_e17822;
        locals.var_t5_dn0 = assign23190_e17822_d_n0;
        locals.var_t5_dn2 = assign23190_e17822_d_n2;
        locals.var_t5_dn4 = assign23190_e17822_d_n4;
        locals.var_t5_dn5 = assign23190_e17822_d_n5;
        locals.var_t5_dn6 = assign23190_e17822_d_n6;
        locals.var_t5_dn7 = assign23190_e17822_d_n7;
        locals.var_t5_dn8 = assign23190_e17822_d_n8;
        locals.var_t5_dn9 = assign23190_e17822_d_n9;
        locals.var_t5_dn10 = assign23190_e17822_d_n10;
        locals.var_t5_dn11 = assign23190_e17822_d_n11;
        locals.var_t5_dn14 = assign23190_e17822_d_n14;

        let (assign23200_e17828, assign23200_e17828_d_n0, assign23200_e17828_d_n2, assign23200_e17828_d_n4, assign23200_e17828_d_n5, assign23200_e17828_d_n6, assign23200_e17828_d_n7, assign23200_e17828_d_n8, assign23200_e17828_d_n9, assign23200_e17828_d_n10, assign23200_e17828_d_n11, assign23200_e17828_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23200_e17826: f64 = (locals.var_t5 * locals.var_sqrt_pbsum);
        (assign23200_e17826, ((locals.var_t5_dn0 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn0)), ((locals.var_t5_dn2 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn2)), ((locals.var_t5_dn4 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn4)), ((locals.var_t5_dn5 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn5)), ((locals.var_t5_dn6 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn6)), ((locals.var_t5_dn7 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn7)), ((locals.var_t5_dn8 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn8)), ((locals.var_t5_dn9 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn9)), ((locals.var_t5_dn10 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn10)), ((locals.var_t5_dn11 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn11)), ((locals.var_t5_dn14 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn14)),)
    } else {
        (locals.var_dvth0, locals.var_dvth0_dn0, locals.var_dvth0_dn2, locals.var_dvth0_dn4, locals.var_dvth0_dn5, locals.var_dvth0_dn6, locals.var_dvth0_dn7, locals.var_dvth0_dn8, locals.var_dvth0_dn9, locals.var_dvth0_dn10, locals.var_dvth0_dn11, locals.var_dvth0_dn14,)
    }
};
        locals.var_dvth0 = assign23200_e17828;
        locals.var_dvth0_dn0 = assign23200_e17828_d_n0;
        locals.var_dvth0_dn2 = assign23200_e17828_d_n2;
        locals.var_dvth0_dn4 = assign23200_e17828_d_n4;
        locals.var_dvth0_dn5 = assign23200_e17828_d_n5;
        locals.var_dvth0_dn6 = assign23200_e17828_d_n6;
        locals.var_dvth0_dn7 = assign23200_e17828_d_n7;
        locals.var_dvth0_dn8 = assign23200_e17828_d_n8;
        locals.var_dvth0_dn9 = assign23200_e17828_d_n9;
        locals.var_dvth0_dn10 = assign23200_e17828_d_n10;
        locals.var_dvth0_dn11 = assign23200_e17828_d_n11;
        locals.var_dvth0_dn14 = assign23200_e17828_d_n14;

        let (assign23210_e17836, assign23210_e17836_d_n0, assign23210_e17836_d_n2, assign23210_e17836_d_n4, assign23210_e17836_d_n5, assign23210_e17836_d_n6, assign23210_e17836_d_n7, assign23210_e17836_d_n8, assign23210_e17836_d_n9, assign23210_e17836_d_n10, assign23210_e17836_d_n11, assign23210_e17836_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23210_e17832: f64 = (0.5 * locals.var_t5);
        let assign23210_e17834: f64 = (assign23210_e17832 / locals.var_sqrt_pbsum);
        (assign23210_e17834, ((((0.5 * locals.var_t5_dn0) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn0)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn2) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn2)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn4) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn4)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn5) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn5)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn6) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn6)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn7) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn7)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn8) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn8)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn9) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn9)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn10) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn10)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn11) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn11)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)), ((((0.5 * locals.var_t5_dn14) * locals.var_sqrt_pbsum) - (assign23210_e17832 * locals.var_sqrt_pbsum_dn14)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23210_e17836;
        locals.var_t6_dn0 = assign23210_e17836_d_n0;
        locals.var_t6_dn2 = assign23210_e17836_d_n2;
        locals.var_t6_dn4 = assign23210_e17836_d_n4;
        locals.var_t6_dn5 = assign23210_e17836_d_n5;
        locals.var_t6_dn6 = assign23210_e17836_d_n6;
        locals.var_t6_dn7 = assign23210_e17836_d_n7;
        locals.var_t6_dn8 = assign23210_e17836_d_n8;
        locals.var_t6_dn9 = assign23210_e17836_d_n9;
        locals.var_t6_dn10 = assign23210_e17836_d_n10;
        locals.var_t6_dn11 = assign23210_e17836_d_n11;
        locals.var_t6_dn14 = assign23210_e17836_d_n14;

        let (assign23220_e17852, assign23220_e17852_d_n0, assign23220_e17852_d_n2, assign23220_e17852_d_n4, assign23220_e17852_d_n5, assign23220_e17852_d_n6, assign23220_e17852_d_n7, assign23220_e17852_d_n8, assign23220_e17852_d_n9, assign23220_e17852_d_n10, assign23220_e17852_d_n11, assign23220_e17852_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23220_e17841: f64 = (p.p137 - locals.var_pb20b);
        let assign23220_e17842: f64 = (2.0 * assign23220_e17841);
        let assign23220_e17844: f64 = (assign23220_e17842 * 1.034943e-10);
        let assign23220_e17846: f64 = (assign23220_e17844 * locals.var_t2);
        let assign23220_e17848: f64 = (assign23220_e17846 * locals.var_t4);
        let assign23220_e17850: f64 = (assign23220_e17848 * locals.var_sqrt_pbsum);
        (assign23220_e17850, ((((((((2.0 * (-locals.var_pb20b_dn0)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn0)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn0)), ((((((((2.0 * (-locals.var_pb20b_dn2)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn2)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn2)), ((((((((2.0 * (-locals.var_pb20b_dn4)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn4)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn4)), ((((((((2.0 * (-locals.var_pb20b_dn5)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn5)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn5)), ((((((((2.0 * (-locals.var_pb20b_dn6)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn6)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn6)), ((((((((2.0 * (-locals.var_pb20b_dn7)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn7)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn7)), ((((((((2.0 * (-locals.var_pb20b_dn8)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn8)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn8)), ((((((((2.0 * (-locals.var_pb20b_dn9)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn9)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn9)), ((((((((2.0 * (-locals.var_pb20b_dn10)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn10)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn10)), ((((((((2.0 * (-locals.var_pb20b_dn11)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn11)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn11)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn11)), ((((((((2.0 * (-locals.var_pb20b_dn14)) * 1.034943e-10) * locals.var_t2) + (assign23220_e17844 * locals.var_t2_dn14)) * locals.var_t4) + (assign23220_e17846 * locals.var_t4_dn14)) * locals.var_sqrt_pbsum) + (assign23220_e17848 * locals.var_sqrt_pbsum_dn14)),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign23220_e17852;
        locals.var_t7_dn0 = assign23220_e17852_d_n0;
        locals.var_t7_dn2 = assign23220_e17852_d_n2;
        locals.var_t7_dn4 = assign23220_e17852_d_n4;
        locals.var_t7_dn5 = assign23220_e17852_d_n5;
        locals.var_t7_dn6 = assign23220_e17852_d_n6;
        locals.var_t7_dn7 = assign23220_e17852_d_n7;
        locals.var_t7_dn8 = assign23220_e17852_d_n8;
        locals.var_t7_dn9 = assign23220_e17852_d_n9;
        locals.var_t7_dn10 = assign23220_e17852_d_n10;
        locals.var_t7_dn11 = assign23220_e17852_d_n11;
        locals.var_t7_dn14 = assign23220_e17852_d_n14;

        let (assign23230_e17865, assign23230_e17865_d_n0, assign23230_e17865_d_n2, assign23230_e17865_d_n4, assign23230_e17865_d_n5, assign23230_e17865_d_n6, assign23230_e17865_d_n7, assign23230_e17865_d_n8, assign23230_e17865_d_n9, assign23230_e17865_d_n10, assign23230_e17865_d_n11, assign23230_e17865_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23230_e17855: f64 = (-2.0);
        let assign23230_e17857: f64 = (assign23230_e17855 * locals.var_t1);
        let assign23230_e17859: f64 = (assign23230_e17857 * locals.var_t2);
        let assign23230_e17861: f64 = (assign23230_e17859 * locals.var_t4);
        let assign23230_e17863: f64 = (assign23230_e17861 * locals.var_sqrt_pbsum);
        (assign23230_e17863, (((((((assign23230_e17855 * locals.var_t1_dn0) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn0)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn0)), (((((((assign23230_e17855 * locals.var_t1_dn2) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn2)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn2)), (((((((assign23230_e17855 * locals.var_t1_dn4) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn4)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn4)), (((((((assign23230_e17855 * locals.var_t1_dn5) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn5)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn5)), (((((((assign23230_e17855 * locals.var_t1_dn6) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn6)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn6)), (((((((assign23230_e17855 * locals.var_t1_dn7) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn7)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn7)), (((((((assign23230_e17855 * locals.var_t1_dn8) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn8)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn8)), (((((((assign23230_e17855 * locals.var_t1_dn9) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn9)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn9)), (((((((assign23230_e17855 * locals.var_t1_dn10) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn10)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn10)), (((((((assign23230_e17855 * locals.var_t1_dn11) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn11)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn11)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn11)), (((((((assign23230_e17855 * locals.var_t1_dn14) * locals.var_t2) + (assign23230_e17857 * locals.var_t2_dn14)) * locals.var_t4) + (assign23230_e17859 * locals.var_t4_dn14)) * locals.var_sqrt_pbsum) + (assign23230_e17861 * locals.var_sqrt_pbsum_dn14)),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign23230_e17865;
        locals.var_t8_dn0 = assign23230_e17865_d_n0;
        locals.var_t8_dn2 = assign23230_e17865_d_n2;
        locals.var_t8_dn4 = assign23230_e17865_d_n4;
        locals.var_t8_dn5 = assign23230_e17865_d_n5;
        locals.var_t8_dn6 = assign23230_e17865_d_n6;
        locals.var_t8_dn7 = assign23230_e17865_d_n7;
        locals.var_t8_dn8 = assign23230_e17865_d_n8;
        locals.var_t8_dn9 = assign23230_e17865_d_n9;
        locals.var_t8_dn10 = assign23230_e17865_d_n10;
        locals.var_t8_dn11 = assign23230_e17865_d_n11;
        locals.var_t8_dn14 = assign23230_e17865_d_n14;

        let (assign23240_e17871, assign23240_e17871_d_n0, assign23240_e17871_d_n2, assign23240_e17871_d_n4, assign23240_e17871_d_n5, assign23240_e17871_d_n6, assign23240_e17871_d_n7, assign23240_e17871_d_n8, assign23240_e17871_d_n9, assign23240_e17871_d_n10, assign23240_e17871_d_n11, assign23240_e17871_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23240_e17869: f64 = (locals.var_vthp - locals.var_vth0);
        (assign23240_e17869, (locals.var_vthp_dn0 - locals.var_vth0_dn0), (locals.var_vthp_dn2 - locals.var_vth0_dn2), (locals.var_vthp_dn4 - locals.var_vth0_dn4), (locals.var_vthp_dn5 - locals.var_vth0_dn5), (locals.var_vthp_dn6 - locals.var_vth0_dn6), (locals.var_vthp_dn7 - locals.var_vth0_dn7), (locals.var_vthp_dn8 - locals.var_vth0_dn8), (locals.var_vthp_dn9 - locals.var_vth0_dn9), (locals.var_vthp_dn10 - locals.var_vth0_dn10), (locals.var_vthp_dn11 - locals.var_vth0_dn11), (locals.var_vthp_dn14 - locals.var_vth0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23240_e17871;
        locals.var_t1_dn0 = assign23240_e17871_d_n0;
        locals.var_t1_dn2 = assign23240_e17871_d_n2;
        locals.var_t1_dn4 = assign23240_e17871_d_n4;
        locals.var_t1_dn5 = assign23240_e17871_d_n5;
        locals.var_t1_dn6 = assign23240_e17871_d_n6;
        locals.var_t1_dn7 = assign23240_e17871_d_n7;
        locals.var_t1_dn8 = assign23240_e17871_d_n8;
        locals.var_t1_dn9 = assign23240_e17871_d_n9;
        locals.var_t1_dn10 = assign23240_e17871_d_n10;
        locals.var_t1_dn11 = assign23240_e17871_d_n11;
        locals.var_t1_dn14 = assign23240_e17871_d_n14;

        let (assign23250_e17881, assign23250_e17881_d_n0, assign23250_e17881_d_n2, assign23250_e17881_d_n4, assign23250_e17881_d_n5, assign23250_e17881_d_n6, assign23250_e17881_d_n7, assign23250_e17881_d_n8, assign23250_e17881_d_n9, assign23250_e17881_d_n10, assign23250_e17881_d_n11, assign23250_e17881_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23250_e17876: f64 = (locals.var_uc_scp3 * locals.var_pbsum);
        let assign23250_e17878: f64 = (assign23250_e17876 / p.p140);
        let assign23250_e17879: f64 = (locals.var_uc_scp1 + assign23250_e17878);
        (assign23250_e17879, ((locals.var_uc_scp3 * locals.var_pbsum_dn0) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn2) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn4) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn5) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn6) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn7) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn8) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn9) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn10) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn11) / p.p140), ((locals.var_uc_scp3 * locals.var_pbsum_dn14) / p.p140),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign23250_e17881;
        locals.var_t2_dn0 = assign23250_e17881_d_n0;
        locals.var_t2_dn2 = assign23250_e17881_d_n2;
        locals.var_t2_dn4 = assign23250_e17881_d_n4;
        locals.var_t2_dn5 = assign23250_e17881_d_n5;
        locals.var_t2_dn6 = assign23250_e17881_d_n6;
        locals.var_t2_dn7 = assign23250_e17881_d_n7;
        locals.var_t2_dn8 = assign23250_e17881_d_n8;
        locals.var_t2_dn9 = assign23250_e17881_d_n9;
        locals.var_t2_dn10 = assign23250_e17881_d_n10;
        locals.var_t2_dn11 = assign23250_e17881_d_n11;
        locals.var_t2_dn14 = assign23250_e17881_d_n14;

        let (assign23260_e17889, assign23260_e17889_d_n0, assign23260_e17889_d_n2, assign23260_e17889_d_n4, assign23260_e17889_d_n5, assign23260_e17889_d_n6, assign23260_e17889_d_n7, assign23260_e17889_d_n8, assign23260_e17889_d_n9, assign23260_e17889_d_n10, assign23260_e17889_d_n11, assign23260_e17889_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23260_e17886: f64 = (locals.var_uc_scp2 * locals.var_vdsz);
        let assign23260_e17887: f64 = (locals.var_t2 + assign23260_e17886);
        (assign23260_e17887, (locals.var_t2_dn0 + (locals.var_uc_scp2 * locals.var_vdsz_dn0)), (locals.var_t2_dn2 + (locals.var_uc_scp2 * locals.var_vdsz_dn2)), (locals.var_t2_dn4 + (locals.var_uc_scp2 * locals.var_vdsz_dn4)), (locals.var_t2_dn5 + (locals.var_uc_scp2 * locals.var_vdsz_dn5)), (locals.var_t2_dn6 + (locals.var_uc_scp2 * locals.var_vdsz_dn6)), (locals.var_t2_dn7 + (locals.var_uc_scp2 * locals.var_vdsz_dn7)), (locals.var_t2_dn8 + (locals.var_uc_scp2 * locals.var_vdsz_dn8)), (locals.var_t2_dn9 + (locals.var_uc_scp2 * locals.var_vdsz_dn9)), (locals.var_t2_dn10 + (locals.var_uc_scp2 * locals.var_vdsz_dn10)), (locals.var_t2_dn11 + (locals.var_uc_scp2 * locals.var_vdsz_dn11)), (locals.var_t2_dn14 + (locals.var_uc_scp2 * locals.var_vdsz_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign23260_e17889;
        locals.var_t3_dn0 = assign23260_e17889_d_n0;
        locals.var_t3_dn2 = assign23260_e17889_d_n2;
        locals.var_t3_dn4 = assign23260_e17889_d_n4;
        locals.var_t3_dn5 = assign23260_e17889_d_n5;
        locals.var_t3_dn6 = assign23260_e17889_d_n6;
        locals.var_t3_dn7 = assign23260_e17889_d_n7;
        locals.var_t3_dn8 = assign23260_e17889_d_n8;
        locals.var_t3_dn9 = assign23260_e17889_d_n9;
        locals.var_t3_dn10 = assign23260_e17889_d_n10;
        locals.var_t3_dn11 = assign23260_e17889_d_n11;
        locals.var_t3_dn14 = assign23260_e17889_d_n14;

        let (assign23270_e17895, assign23270_e17895_d_n0, assign23270_e17895_d_n2, assign23270_e17895_d_n4, assign23270_e17895_d_n5, assign23270_e17895_d_n6, assign23270_e17895_d_n7, assign23270_e17895_d_n8, assign23270_e17895_d_n9, assign23270_e17895_d_n10, assign23270_e17895_d_n11, assign23270_e17895_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23270_e17893: f64 = (p.p221 + locals.var_vdsz);
        (assign23270_e17893, locals.var_vdsz_dn0, locals.var_vdsz_dn2, locals.var_vdsz_dn4, locals.var_vdsz_dn5, locals.var_vdsz_dn6, locals.var_vdsz_dn7, locals.var_vdsz_dn8, locals.var_vdsz_dn9, locals.var_vdsz_dn10, locals.var_vdsz_dn11, locals.var_vdsz_dn14,)
    } else {
        (locals.var_vdx, locals.var_vdx_dn0, locals.var_vdx_dn2, locals.var_vdx_dn4, locals.var_vdx_dn5, locals.var_vdx_dn6, locals.var_vdx_dn7, locals.var_vdx_dn8, locals.var_vdx_dn9, locals.var_vdx_dn10, locals.var_vdx_dn11, locals.var_vdx_dn14,)
    }
};
        locals.var_vdx = assign23270_e17895;
        locals.var_vdx_dn0 = assign23270_e17895_d_n0;
        locals.var_vdx_dn2 = assign23270_e17895_d_n2;
        locals.var_vdx_dn4 = assign23270_e17895_d_n4;
        locals.var_vdx_dn5 = assign23270_e17895_d_n5;
        locals.var_vdx_dn6 = assign23270_e17895_d_n6;
        locals.var_vdx_dn7 = assign23270_e17895_d_n7;
        locals.var_vdx_dn8 = assign23270_e17895_d_n8;
        locals.var_vdx_dn9 = assign23270_e17895_d_n9;
        locals.var_vdx_dn10 = assign23270_e17895_d_n10;
        locals.var_vdx_dn11 = assign23270_e17895_d_n11;
        locals.var_vdx_dn14 = assign23270_e17895_d_n14;

        let (assign23280_e17901, assign23280_e17901_d_n0, assign23280_e17901_d_n2, assign23280_e17901_d_n4, assign23280_e17901_d_n5, assign23280_e17901_d_n6, assign23280_e17901_d_n7, assign23280_e17901_d_n8, assign23280_e17901_d_n9, assign23280_e17901_d_n10, assign23280_e17901_d_n11, assign23280_e17901_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23280_e17899: f64 = (locals.var_vdx * locals.var_vdx);
        (assign23280_e17899, ((locals.var_vdx_dn0 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn0)), ((locals.var_vdx_dn2 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn2)), ((locals.var_vdx_dn4 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn4)), ((locals.var_vdx_dn5 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn5)), ((locals.var_vdx_dn6 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn6)), ((locals.var_vdx_dn7 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn7)), ((locals.var_vdx_dn8 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn8)), ((locals.var_vdx_dn9 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn9)), ((locals.var_vdx_dn10 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn10)), ((locals.var_vdx_dn11 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn11)), ((locals.var_vdx_dn14 * locals.var_vdx) + (locals.var_vdx * locals.var_vdx_dn14)),)
    } else {
        (locals.var_vdx2, locals.var_vdx2_dn0, locals.var_vdx2_dn2, locals.var_vdx2_dn4, locals.var_vdx2_dn5, locals.var_vdx2_dn6, locals.var_vdx2_dn7, locals.var_vdx2_dn8, locals.var_vdx2_dn9, locals.var_vdx2_dn10, locals.var_vdx2_dn11, locals.var_vdx2_dn14,)
    }
};
        locals.var_vdx2 = assign23280_e17901;
        locals.var_vdx2_dn0 = assign23280_e17901_d_n0;
        locals.var_vdx2_dn2 = assign23280_e17901_d_n2;
        locals.var_vdx2_dn4 = assign23280_e17901_d_n4;
        locals.var_vdx2_dn5 = assign23280_e17901_d_n5;
        locals.var_vdx2_dn6 = assign23280_e17901_d_n6;
        locals.var_vdx2_dn7 = assign23280_e17901_d_n7;
        locals.var_vdx2_dn8 = assign23280_e17901_d_n8;
        locals.var_vdx2_dn9 = assign23280_e17901_d_n9;
        locals.var_vdx2_dn10 = assign23280_e17901_d_n10;
        locals.var_vdx2_dn11 = assign23280_e17901_d_n11;
        locals.var_vdx2_dn14 = assign23280_e17901_d_n14;

    }

    pub(super) fn stamp_transient_block_60(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23290_e17915, assign23290_e17915_d_n0, assign23290_e17915_d_n2, assign23290_e17915_d_n4, assign23290_e17915_d_n5, assign23290_e17915_d_n6, assign23290_e17915_d_n7, assign23290_e17915_d_n8, assign23290_e17915_d_n9, assign23290_e17915_d_n10, assign23290_e17915_d_n11, assign23290_e17915_d_n14,) = {
    if (locals.var_guard432 != 0.0) {
        let assign23290_e17905: f64 = (locals.var_t1 * locals.var_dvth0);
        let assign23290_e17907: f64 = (assign23290_e17905 * locals.var_t3);
        let assign23290_e17909: f64 = (assign23290_e17907 + locals.var_dqb);
        let assign23290_e17912: f64 = (locals.var_msc / locals.var_vdx2);
        let assign23290_e17913: f64 = (assign23290_e17909 - assign23290_e17912);
        (assign23290_e17913, ((((((locals.var_t1_dn0 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn0)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn0)) + locals.var_dqb_dn0) - (-((locals.var_msc * locals.var_vdx2_dn0) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn2 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn2)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn2)) + locals.var_dqb_dn2) - (-((locals.var_msc * locals.var_vdx2_dn2) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn4 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn4)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn4)) + locals.var_dqb_dn4) - (-((locals.var_msc * locals.var_vdx2_dn4) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn5 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn5)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn5)) + locals.var_dqb_dn5) - (-((locals.var_msc * locals.var_vdx2_dn5) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn6 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn6)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn6)) + locals.var_dqb_dn6) - (-((locals.var_msc * locals.var_vdx2_dn6) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn7 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn7)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn7)) + locals.var_dqb_dn7) - (-((locals.var_msc * locals.var_vdx2_dn7) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn8 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn8)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn8)) + locals.var_dqb_dn8) - (-((locals.var_msc * locals.var_vdx2_dn8) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn9 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn9)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn9)) + locals.var_dqb_dn9) - (-((locals.var_msc * locals.var_vdx2_dn9) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn10 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn10)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn10)) + locals.var_dqb_dn10) - (-((locals.var_msc * locals.var_vdx2_dn10) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn11 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn11)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn11)) + locals.var_dqb_dn11) - (-((locals.var_msc * locals.var_vdx2_dn11) / (locals.var_vdx2 * locals.var_vdx2)))), ((((((locals.var_t1_dn14 * locals.var_dvth0) + (locals.var_t1 * locals.var_dvth0_dn14)) * locals.var_t3) + (assign23290_e17905 * locals.var_t3_dn14)) + locals.var_dqb_dn14) - (-((locals.var_msc * locals.var_vdx2_dn14) / (locals.var_vdx2 * locals.var_vdx2)))),)
    } else {
        (locals.var_dvthlp, locals.var_dvthlp_dn0, locals.var_dvthlp_dn2, locals.var_dvthlp_dn4, locals.var_dvthlp_dn5, locals.var_dvthlp_dn6, locals.var_dvthlp_dn7, locals.var_dvthlp_dn8, locals.var_dvthlp_dn9, locals.var_dvthlp_dn10, locals.var_dvthlp_dn11, locals.var_dvthlp_dn14,)
    }
};
        locals.var_dvthlp = assign23290_e17915;
        locals.var_dvthlp_dn0 = assign23290_e17915_d_n0;
        locals.var_dvthlp_dn2 = assign23290_e17915_d_n2;
        locals.var_dvthlp_dn4 = assign23290_e17915_d_n4;
        locals.var_dvthlp_dn5 = assign23290_e17915_d_n5;
        locals.var_dvthlp_dn6 = assign23290_e17915_d_n6;
        locals.var_dvthlp_dn7 = assign23290_e17915_d_n7;
        locals.var_dvthlp_dn8 = assign23290_e17915_d_n8;
        locals.var_dvthlp_dn9 = assign23290_e17915_d_n9;
        locals.var_dvthlp_dn10 = assign23290_e17915_d_n10;
        locals.var_dvthlp_dn11 = assign23290_e17915_d_n11;
        locals.var_dvthlp_dn14 = assign23290_e17915_d_n14;

        let (assign23300_e17920, assign23300_e17920_d_n0, assign23300_e17920_d_n2, assign23300_e17920_d_n4, assign23300_e17920_d_n5, assign23300_e17920_d_n6, assign23300_e17920_d_n7, assign23300_e17920_d_n8, assign23300_e17920_d_n9, assign23300_e17920_d_n10, assign23300_e17920_d_n11, assign23300_e17920_d_n14,) = {
    if (locals.var_guard432 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dvthlp, locals.var_dvthlp_dn0, locals.var_dvthlp_dn2, locals.var_dvthlp_dn4, locals.var_dvthlp_dn5, locals.var_dvthlp_dn6, locals.var_dvthlp_dn7, locals.var_dvthlp_dn8, locals.var_dvthlp_dn9, locals.var_dvthlp_dn10, locals.var_dvthlp_dn11, locals.var_dvthlp_dn14,)
    }
};
        locals.var_dvthlp = assign23300_e17920;
        locals.var_dvthlp_dn0 = assign23300_e17920_d_n0;
        locals.var_dvthlp_dn2 = assign23300_e17920_d_n2;
        locals.var_dvthlp_dn4 = assign23300_e17920_d_n4;
        locals.var_dvthlp_dn5 = assign23300_e17920_d_n5;
        locals.var_dvthlp_dn6 = assign23300_e17920_d_n6;
        locals.var_dvthlp_dn7 = assign23300_e17920_d_n7;
        locals.var_dvthlp_dn8 = assign23300_e17920_d_n8;
        locals.var_dvthlp_dn9 = assign23300_e17920_d_n9;
        locals.var_dvthlp_dn10 = assign23300_e17920_d_n10;
        locals.var_dvthlp_dn11 = assign23300_e17920_d_n11;
        locals.var_dvthlp_dn14 = assign23300_e17920_d_n14;

        let assign23310_e17923: f64 = (1.034943e-10 * locals.var_cox_inv);
        locals.var_t1 = assign23310_e17923;
        locals.var_t1_dn0 = (1.034943e-10 * locals.var_cox_inv_dn0);
        locals.var_t1_dn2 = (1.034943e-10 * locals.var_cox_inv_dn2);
        locals.var_t1_dn4 = (1.034943e-10 * locals.var_cox_inv_dn4);
        locals.var_t1_dn5 = (1.034943e-10 * locals.var_cox_inv_dn5);
        locals.var_t1_dn6 = (1.034943e-10 * locals.var_cox_inv_dn6);
        locals.var_t1_dn7 = (1.034943e-10 * locals.var_cox_inv_dn7);
        locals.var_t1_dn8 = (1.034943e-10 * locals.var_cox_inv_dn8);
        locals.var_t1_dn9 = (1.034943e-10 * locals.var_cox_inv_dn9);
        locals.var_t1_dn10 = (1.034943e-10 * locals.var_cox_inv_dn10);
        locals.var_t1_dn11 = (1.034943e-10 * locals.var_cox_inv_dn11);
        locals.var_t1_dn14 = (1.034943e-10 * locals.var_cox_inv_dn14);

        locals.var_t2 = locals.var_wdpl;
        locals.var_t2_dn0 = locals.var_wdpl_dn0;
        locals.var_t2_dn2 = locals.var_wdpl_dn2;
        locals.var_t2_dn4 = locals.var_wdpl_dn4;
        locals.var_t2_dn5 = locals.var_wdpl_dn5;
        locals.var_t2_dn6 = locals.var_wdpl_dn6;
        locals.var_t2_dn7 = locals.var_wdpl_dn7;
        locals.var_t2_dn8 = locals.var_wdpl_dn8;
        locals.var_t2_dn9 = locals.var_wdpl_dn9;
        locals.var_t2_dn10 = locals.var_wdpl_dn10;
        locals.var_t2_dn11 = locals.var_wdpl_dn11;
        locals.var_t2_dn14 = locals.var_wdpl_dn14;

        let assign23330_e17927: f64 = (locals.var_lgate - p.p139);
        locals.var_t3 = assign23330_e17927;
        locals.var_t3_dn0 = 0.0;
        locals.var_t3_dn2 = 0.0;
        locals.var_t3_dn4 = 0.0;
        locals.var_t3_dn5 = 0.0;
        locals.var_t3_dn6 = 0.0;
        locals.var_t3_dn7 = 0.0;
        locals.var_t3_dn8 = 0.0;
        locals.var_t3_dn9 = 0.0;
        locals.var_t3_dn10 = 0.0;
        locals.var_t3_dn11 = 0.0;
        locals.var_t3_dn14 = 0.0;

        let assign23340_e17931: f64 = (locals.var_t3 * locals.var_t3);
        let assign23340_e17932: f64 = (1.0 / assign23340_e17931);
        locals.var_t4 = assign23340_e17932;
        locals.var_t4_dn0 = (-(((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (assign23340_e17931 * assign23340_e17931)));
        locals.var_t4_dn2 = (-(((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (assign23340_e17931 * assign23340_e17931)));
        locals.var_t4_dn4 = (-(((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (assign23340_e17931 * assign23340_e17931)));
        locals.var_t4_dn5 = (-(((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (assign23340_e17931 * assign23340_e17931)));
        locals.var_t4_dn6 = (-(((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (assign23340_e17931 * assign23340_e17931)));
        locals.var_t4_dn7 = (-(((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) / (assign23340_e17931 * assign23340_e17931)));
        locals.var_t4_dn8 = (-(((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (assign23340_e17931 * assign23340_e17931)));
        locals.var_t4_dn9 = (-(((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) / (assign23340_e17931 * assign23340_e17931)));
        locals.var_t4_dn10 = (-(((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (assign23340_e17931 * assign23340_e17931)));
        locals.var_t4_dn11 = (-(((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)) / (assign23340_e17931 * assign23340_e17931)));
        locals.var_t4_dn14 = (-(((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14)) / (assign23340_e17931 * assign23340_e17931)));

        let assign23350_e17936: f64 = (p.p137 - locals.var_pb20b);
        let assign23350_e17937: f64 = (2.0 * assign23350_e17936);
        let assign23350_e17939: f64 = (assign23350_e17937 * locals.var_t1);
        let assign23350_e17941: f64 = (assign23350_e17939 * locals.var_t2);
        let assign23350_e17943: f64 = (assign23350_e17941 * locals.var_t4);
        locals.var_t5 = assign23350_e17943;
        locals.var_t5_dn0 = (((((((2.0 * (-locals.var_pb20b_dn0)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn0)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn0)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn0));
        locals.var_t5_dn2 = (((((((2.0 * (-locals.var_pb20b_dn2)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn2)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn2)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn2));
        locals.var_t5_dn4 = (((((((2.0 * (-locals.var_pb20b_dn4)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn4)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn4)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn4));
        locals.var_t5_dn5 = (((((((2.0 * (-locals.var_pb20b_dn5)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn5)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn5)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn5));
        locals.var_t5_dn6 = (((((((2.0 * (-locals.var_pb20b_dn6)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn6)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn6)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn6));
        locals.var_t5_dn7 = (((((((2.0 * (-locals.var_pb20b_dn7)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn7)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn7)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn7));
        locals.var_t5_dn8 = (((((((2.0 * (-locals.var_pb20b_dn8)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn8)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn8)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn8));
        locals.var_t5_dn9 = (((((((2.0 * (-locals.var_pb20b_dn9)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn9)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn9)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn9));
        locals.var_t5_dn10 = (((((((2.0 * (-locals.var_pb20b_dn10)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn10)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn10)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn10));
        locals.var_t5_dn11 = (((((((2.0 * (-locals.var_pb20b_dn11)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn11)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn11)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn11));
        locals.var_t5_dn14 = (((((((2.0 * (-locals.var_pb20b_dn14)) * locals.var_t1) + (assign23350_e17937 * locals.var_t1_dn14)) * locals.var_t2) + (assign23350_e17939 * locals.var_t2_dn14)) * locals.var_t4) + (assign23350_e17941 * locals.var_t4_dn14));

        let assign23360_e17946: f64 = (locals.var_t5 * locals.var_sqrt_pbsum);
        locals.var_dvth0 = assign23360_e17946;
        locals.var_dvth0_dn0 = ((locals.var_t5_dn0 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn0));
        locals.var_dvth0_dn2 = ((locals.var_t5_dn2 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn2));
        locals.var_dvth0_dn4 = ((locals.var_t5_dn4 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn4));
        locals.var_dvth0_dn5 = ((locals.var_t5_dn5 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn5));
        locals.var_dvth0_dn6 = ((locals.var_t5_dn6 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn6));
        locals.var_dvth0_dn7 = ((locals.var_t5_dn7 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn7));
        locals.var_dvth0_dn8 = ((locals.var_t5_dn8 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn8));
        locals.var_dvth0_dn9 = ((locals.var_t5_dn9 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn9));
        locals.var_dvth0_dn10 = ((locals.var_t5_dn10 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn10));
        locals.var_dvth0_dn11 = ((locals.var_t5_dn11 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn11));
        locals.var_dvth0_dn14 = ((locals.var_t5_dn14 * locals.var_sqrt_pbsum) + (locals.var_t5 * locals.var_sqrt_pbsum_dn14));

        let assign23370_e17949: f64 = (locals.var_t5 / 2.0);
        let assign23370_e17951: f64 = (assign23370_e17949 / locals.var_sqrt_pbsum);
        locals.var_t6 = assign23370_e17951;
        locals.var_t6_dn0 = ((((locals.var_t5_dn0 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn0)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn2 = ((((locals.var_t5_dn2 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn2)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn4 = ((((locals.var_t5_dn4 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn4)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn5 = ((((locals.var_t5_dn5 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn5)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn6 = ((((locals.var_t5_dn6 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn6)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn7 = ((((locals.var_t5_dn7 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn7)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn8 = ((((locals.var_t5_dn8 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn8)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn9 = ((((locals.var_t5_dn9 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn9)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn10 = ((((locals.var_t5_dn10 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn10)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn11 = ((((locals.var_t5_dn11 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn11)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));
        locals.var_t6_dn14 = ((((locals.var_t5_dn14 / 2.0) * locals.var_sqrt_pbsum) - (assign23370_e17949 * locals.var_sqrt_pbsum_dn14)) / (locals.var_sqrt_pbsum * locals.var_sqrt_pbsum));

        let assign23380_e17955: f64 = (p.p137 - locals.var_pb20b);
        let assign23380_e17956: f64 = (2.0 * assign23380_e17955);
        let assign23380_e17958: f64 = (assign23380_e17956 * 1.034943e-10);
        let assign23380_e17960: f64 = (assign23380_e17958 * locals.var_t2);
        let assign23380_e17962: f64 = (assign23380_e17960 * locals.var_t4);
        let assign23380_e17964: f64 = (assign23380_e17962 * locals.var_sqrt_pbsum);
        locals.var_t7 = assign23380_e17964;
        locals.var_t7_dn0 = ((((((((2.0 * (-locals.var_pb20b_dn0)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn0)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn0));
        locals.var_t7_dn2 = ((((((((2.0 * (-locals.var_pb20b_dn2)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn2)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn2));
        locals.var_t7_dn4 = ((((((((2.0 * (-locals.var_pb20b_dn4)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn4)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn4));
        locals.var_t7_dn5 = ((((((((2.0 * (-locals.var_pb20b_dn5)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn5)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn5));
        locals.var_t7_dn6 = ((((((((2.0 * (-locals.var_pb20b_dn6)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn6)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn6));
        locals.var_t7_dn7 = ((((((((2.0 * (-locals.var_pb20b_dn7)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn7)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn7));
        locals.var_t7_dn8 = ((((((((2.0 * (-locals.var_pb20b_dn8)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn8)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn8));
        locals.var_t7_dn9 = ((((((((2.0 * (-locals.var_pb20b_dn9)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn9)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn9));
        locals.var_t7_dn10 = ((((((((2.0 * (-locals.var_pb20b_dn10)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn10)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn10));
        locals.var_t7_dn11 = ((((((((2.0 * (-locals.var_pb20b_dn11)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn11)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn11)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn11));
        locals.var_t7_dn14 = ((((((((2.0 * (-locals.var_pb20b_dn14)) * 1.034943e-10) * locals.var_t2) + (assign23380_e17958 * locals.var_t2_dn14)) * locals.var_t4) + (assign23380_e17960 * locals.var_t4_dn14)) * locals.var_sqrt_pbsum) + (assign23380_e17962 * locals.var_sqrt_pbsum_dn14));

        let assign23390_e17966: f64 = (-2.0);
        let assign23390_e17968: f64 = (assign23390_e17966 * locals.var_t1);
        let assign23390_e17970: f64 = (assign23390_e17968 * locals.var_t2);
        let assign23390_e17972: f64 = (assign23390_e17970 * locals.var_t4);
        let assign23390_e17974: f64 = (assign23390_e17972 * locals.var_sqrt_pbsum);
        locals.var_t8 = assign23390_e17974;
        locals.var_t8_dn0 = (((((((assign23390_e17966 * locals.var_t1_dn0) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn0)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn0)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn0));
        locals.var_t8_dn2 = (((((((assign23390_e17966 * locals.var_t1_dn2) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn2)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn2)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn2));
        locals.var_t8_dn4 = (((((((assign23390_e17966 * locals.var_t1_dn4) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn4)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn4)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn4));
        locals.var_t8_dn5 = (((((((assign23390_e17966 * locals.var_t1_dn5) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn5)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn5)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn5));
        locals.var_t8_dn6 = (((((((assign23390_e17966 * locals.var_t1_dn6) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn6)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn6)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn6));
        locals.var_t8_dn7 = (((((((assign23390_e17966 * locals.var_t1_dn7) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn7)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn7)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn7));
        locals.var_t8_dn8 = (((((((assign23390_e17966 * locals.var_t1_dn8) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn8)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn8)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn8));
        locals.var_t8_dn9 = (((((((assign23390_e17966 * locals.var_t1_dn9) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn9)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn9)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn9));
        locals.var_t8_dn10 = (((((((assign23390_e17966 * locals.var_t1_dn10) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn10)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn10)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn10));
        locals.var_t8_dn11 = (((((((assign23390_e17966 * locals.var_t1_dn11) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn11)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn11)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn11));
        locals.var_t8_dn14 = (((((((assign23390_e17966 * locals.var_t1_dn14) * locals.var_t2) + (assign23390_e17968 * locals.var_t2_dn14)) * locals.var_t4) + (assign23390_e17970 * locals.var_t4_dn14)) * locals.var_sqrt_pbsum) + (assign23390_e17972 * locals.var_sqrt_pbsum_dn14));

        let assign23400_e17977: f64 = (locals.var_uc_sc3 / locals.var_lgate);
        locals.var_t1 = assign23400_e17977;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn14 = 0.0;

        let assign23410_e17981: f64 = (locals.var_t1 * locals.var_pbsum);
        let assign23410_e17982: f64 = (locals.var_uc_sc1 + assign23410_e17981);
        locals.var_t4 = assign23410_e17982;
        locals.var_t4_dn0 = ((locals.var_t1_dn0 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn0));
        locals.var_t4_dn2 = ((locals.var_t1_dn2 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn2));
        locals.var_t4_dn4 = ((locals.var_t1_dn4 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn4));
        locals.var_t4_dn5 = ((locals.var_t1_dn5 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn5));
        locals.var_t4_dn6 = ((locals.var_t1_dn6 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn6));
        locals.var_t4_dn7 = ((locals.var_t1_dn7 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn7));
        locals.var_t4_dn8 = ((locals.var_t1_dn8 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn8));
        locals.var_t4_dn9 = ((locals.var_t1_dn9 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn9));
        locals.var_t4_dn10 = ((locals.var_t1_dn10 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn10));
        locals.var_t4_dn11 = ((locals.var_t1_dn11 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn11));
        locals.var_t4_dn14 = ((locals.var_t1_dn14 * locals.var_pbsum) + (locals.var_t1 * locals.var_pbsum_dn14));

        let assign23420_e17986: f64 = (locals.var_uc_sc2 * locals.var_vdsz);
        let assign23420_e17990: f64 = (p.p150 * locals.var_pbsum);
        let assign23420_e17991: f64 = (1.0 + assign23420_e17990);
        let assign23420_e17992: f64 = (assign23420_e17986 * assign23420_e17991);
        let assign23420_e17993: f64 = (locals.var_t4 + assign23420_e17992);
        locals.var_t5 = assign23420_e17993;
        locals.var_t5_dn0 = (locals.var_t4_dn0 + (((locals.var_uc_sc2 * locals.var_vdsz_dn0) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn0))));
        locals.var_t5_dn2 = (locals.var_t4_dn2 + (((locals.var_uc_sc2 * locals.var_vdsz_dn2) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn2))));
        locals.var_t5_dn4 = (locals.var_t4_dn4 + (((locals.var_uc_sc2 * locals.var_vdsz_dn4) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn4))));
        locals.var_t5_dn5 = (locals.var_t4_dn5 + (((locals.var_uc_sc2 * locals.var_vdsz_dn5) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn5))));
        locals.var_t5_dn6 = (locals.var_t4_dn6 + (((locals.var_uc_sc2 * locals.var_vdsz_dn6) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn6))));
        locals.var_t5_dn7 = (locals.var_t4_dn7 + (((locals.var_uc_sc2 * locals.var_vdsz_dn7) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn7))));
        locals.var_t5_dn8 = (locals.var_t4_dn8 + (((locals.var_uc_sc2 * locals.var_vdsz_dn8) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn8))));
        locals.var_t5_dn9 = (locals.var_t4_dn9 + (((locals.var_uc_sc2 * locals.var_vdsz_dn9) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn9))));
        locals.var_t5_dn10 = (locals.var_t4_dn10 + (((locals.var_uc_sc2 * locals.var_vdsz_dn10) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn10))));
        locals.var_t5_dn11 = (locals.var_t4_dn11 + (((locals.var_uc_sc2 * locals.var_vdsz_dn11) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn11))));
        locals.var_t5_dn14 = (locals.var_t4_dn14 + (((locals.var_uc_sc2 * locals.var_vdsz_dn14) * assign23420_e17991) + (assign23420_e17986 * (p.p150 * locals.var_pbsum_dn14))));

        let assign23430_e17996: f64 = (locals.var_dvth0 * locals.var_t5);
        locals.var_dvthsc = assign23430_e17996;
        locals.var_dvthsc_dn0 = ((locals.var_dvth0_dn0 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn0));
        locals.var_dvthsc_dn2 = ((locals.var_dvth0_dn2 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn2));
        locals.var_dvthsc_dn4 = ((locals.var_dvth0_dn4 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn4));
        locals.var_dvthsc_dn5 = ((locals.var_dvth0_dn5 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn5));
        locals.var_dvthsc_dn6 = ((locals.var_dvth0_dn6 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn6));
        locals.var_dvthsc_dn7 = ((locals.var_dvth0_dn7 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn7));
        locals.var_dvthsc_dn8 = ((locals.var_dvth0_dn8 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn8));
        locals.var_dvthsc_dn9 = ((locals.var_dvth0_dn9 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn9));
        locals.var_dvthsc_dn10 = ((locals.var_dvth0_dn10 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn10));
        locals.var_dvthsc_dn11 = ((locals.var_dvth0_dn11 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn11));
        locals.var_dvthsc_dn14 = ((locals.var_dvth0_dn14 * locals.var_t5) + (locals.var_dvth0 * locals.var_t5_dn14));

        let assign23440_e17999: f64 = (1.0 / locals.var_cox);
        locals.var_t1 = assign23440_e17999;
        locals.var_t1_dn0 = (-(locals.var_cox_dn0 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn2 = (-(locals.var_cox_dn2 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn4 = (-(locals.var_cox_dn4 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn5 = (-(locals.var_cox_dn5 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn6 = (-(locals.var_cox_dn6 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn7 = (-(locals.var_cox_dn7 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn8 = (-(locals.var_cox_dn8 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn9 = (-(locals.var_cox_dn9 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn10 = (-(locals.var_cox_dn10 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn11 = (-(locals.var_cox_dn11 / (locals.var_cox * locals.var_cox)));
        locals.var_t1_dn14 = (-(locals.var_cox_dn14 / (locals.var_cox * locals.var_cox)));

        let assign23450_e18002: f64 = (locals.var_t1 * locals.var_t1);
        locals.var_t2 = assign23450_e18002;
        locals.var_t2_dn0 = ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0));
        locals.var_t2_dn2 = ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2));
        locals.var_t2_dn4 = ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4));
        locals.var_t2_dn5 = ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5));
        locals.var_t2_dn6 = ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6));
        locals.var_t2_dn7 = ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7));
        locals.var_t2_dn8 = ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8));
        locals.var_t2_dn9 = ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9));
        locals.var_t2_dn10 = ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10));
        locals.var_t2_dn11 = ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11));
        locals.var_t2_dn14 = ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14));

        let assign23460_e18007: f64 = (locals.var_uc_wfc / locals.var_weff);
        let assign23460_e18008: f64 = (locals.var_cox + assign23460_e18007);
        let assign23460_e18009: f64 = (1.0 / assign23460_e18008);
        locals.var_t3 = assign23460_e18009;
        locals.var_t3_dn0 = (-(locals.var_cox_dn0 / (assign23460_e18008 * assign23460_e18008)));
        locals.var_t3_dn2 = (-(locals.var_cox_dn2 / (assign23460_e18008 * assign23460_e18008)));
        locals.var_t3_dn4 = (-(locals.var_cox_dn4 / (assign23460_e18008 * assign23460_e18008)));
        locals.var_t3_dn5 = (-(locals.var_cox_dn5 / (assign23460_e18008 * assign23460_e18008)));
        locals.var_t3_dn6 = (-(locals.var_cox_dn6 / (assign23460_e18008 * assign23460_e18008)));
        locals.var_t3_dn7 = (-(locals.var_cox_dn7 / (assign23460_e18008 * assign23460_e18008)));
        locals.var_t3_dn8 = (-(locals.var_cox_dn8 / (assign23460_e18008 * assign23460_e18008)));
        locals.var_t3_dn9 = (-(locals.var_cox_dn9 / (assign23460_e18008 * assign23460_e18008)));
        locals.var_t3_dn10 = (-(locals.var_cox_dn10 / (assign23460_e18008 * assign23460_e18008)));
        locals.var_t3_dn11 = (-(locals.var_cox_dn11 / (assign23460_e18008 * assign23460_e18008)));
        locals.var_t3_dn14 = (-(locals.var_cox_dn14 / (assign23460_e18008 * assign23460_e18008)));

        let assign23470_e18012: f64 = (locals.var_t3 * locals.var_t3);
        locals.var_t4 = assign23470_e18012;
        locals.var_t4_dn0 = ((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0));
        locals.var_t4_dn2 = ((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2));
        locals.var_t4_dn4 = ((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4));
        locals.var_t4_dn5 = ((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5));
        locals.var_t4_dn6 = ((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6));
        locals.var_t4_dn7 = ((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7));
        locals.var_t4_dn8 = ((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8));
        locals.var_t4_dn9 = ((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9));
        locals.var_t4_dn10 = ((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10));
        locals.var_t4_dn11 = ((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11));
        locals.var_t4_dn14 = ((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14));

        let assign23480_e18015: f64 = (locals.var_t1 - locals.var_t3);
        locals.var_t5 = assign23480_e18015;
        locals.var_t5_dn0 = (locals.var_t1_dn0 - locals.var_t3_dn0);
        locals.var_t5_dn2 = (locals.var_t1_dn2 - locals.var_t3_dn2);
        locals.var_t5_dn4 = (locals.var_t1_dn4 - locals.var_t3_dn4);
        locals.var_t5_dn5 = (locals.var_t1_dn5 - locals.var_t3_dn5);
        locals.var_t5_dn6 = (locals.var_t1_dn6 - locals.var_t3_dn6);
        locals.var_t5_dn7 = (locals.var_t1_dn7 - locals.var_t3_dn7);
        locals.var_t5_dn8 = (locals.var_t1_dn8 - locals.var_t3_dn8);
        locals.var_t5_dn9 = (locals.var_t1_dn9 - locals.var_t3_dn9);
        locals.var_t5_dn10 = (locals.var_t1_dn10 - locals.var_t3_dn10);
        locals.var_t5_dn11 = (locals.var_t1_dn11 - locals.var_t3_dn11);
        locals.var_t5_dn14 = (locals.var_t1_dn14 - locals.var_t3_dn14);

        let assign23490_e18019: f64 = (locals.var_t2 - locals.var_t4);
        let assign23490_e18020: f64 = (locals.var_qb0 * assign23490_e18019);
        locals.var_t6 = assign23490_e18020;
        locals.var_t6_dn0 = ((locals.var_qb0_dn0 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn0 - locals.var_t4_dn0)));
        locals.var_t6_dn2 = ((locals.var_qb0_dn2 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn2 - locals.var_t4_dn2)));
        locals.var_t6_dn4 = ((locals.var_qb0_dn4 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn4 - locals.var_t4_dn4)));
        locals.var_t6_dn5 = ((locals.var_qb0_dn5 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn5 - locals.var_t4_dn5)));
        locals.var_t6_dn6 = ((locals.var_qb0_dn6 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn6 - locals.var_t4_dn6)));
        locals.var_t6_dn7 = ((locals.var_qb0_dn7 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn7 - locals.var_t4_dn7)));
        locals.var_t6_dn8 = ((locals.var_qb0_dn8 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn8 - locals.var_t4_dn8)));
        locals.var_t6_dn9 = ((locals.var_qb0_dn9 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn9 - locals.var_t4_dn9)));
        locals.var_t6_dn10 = ((locals.var_qb0_dn10 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn10 - locals.var_t4_dn10)));
        locals.var_t6_dn11 = ((locals.var_qb0_dn11 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn11 - locals.var_t4_dn11)));
        locals.var_t6_dn14 = ((locals.var_qb0_dn14 * assign23490_e18019) + (locals.var_qb0 * (locals.var_t2_dn14 - locals.var_t4_dn14)));

        let assign23500_e18023: f64 = (locals.var_qb0 * locals.var_t5);
        let assign23500_e18026: f64 = (locals.var_uc_wvth0 / locals.var_wg);
        let assign23500_e18027: f64 = (assign23500_e18023 + assign23500_e18026);
        locals.var_dvthw = assign23500_e18027;
        locals.var_dvthw_dn0 = ((locals.var_qb0_dn0 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn0));
        locals.var_dvthw_dn2 = ((locals.var_qb0_dn2 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn2));
        locals.var_dvthw_dn4 = ((locals.var_qb0_dn4 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn4));
        locals.var_dvthw_dn5 = ((locals.var_qb0_dn5 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn5));
        locals.var_dvthw_dn6 = ((locals.var_qb0_dn6 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn6));
        locals.var_dvthw_dn7 = ((locals.var_qb0_dn7 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn7));
        locals.var_dvthw_dn8 = ((locals.var_qb0_dn8 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn8));
        locals.var_dvthw_dn9 = ((locals.var_qb0_dn9 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn9));
        locals.var_dvthw_dn10 = ((locals.var_qb0_dn10 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn10));
        locals.var_dvthw_dn11 = ((locals.var_qb0_dn11 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn11));
        locals.var_dvthw_dn14 = ((locals.var_qb0_dn14 * locals.var_t5) + (locals.var_qb0 * locals.var_t5_dn14));

        let assign23510_e18030: f64 = (locals.var_dvthsc + locals.var_dvthlp);
        let assign23510_e18032: f64 = (assign23510_e18030 + locals.var_dvthw);
        let assign23510_e18034: f64 = (assign23510_e18032 + locals.var_dvthsm);
        locals.var_dvth = assign23510_e18034;
        locals.var_dvth_dn0 = ((locals.var_dvthsc_dn0 + locals.var_dvthlp_dn0) + locals.var_dvthw_dn0);
        locals.var_dvth_dn2 = ((locals.var_dvthsc_dn2 + locals.var_dvthlp_dn2) + locals.var_dvthw_dn2);
        locals.var_dvth_dn4 = ((locals.var_dvthsc_dn4 + locals.var_dvthlp_dn4) + locals.var_dvthw_dn4);
        locals.var_dvth_dn5 = ((locals.var_dvthsc_dn5 + locals.var_dvthlp_dn5) + locals.var_dvthw_dn5);
        locals.var_dvth_dn6 = ((locals.var_dvthsc_dn6 + locals.var_dvthlp_dn6) + locals.var_dvthw_dn6);
        locals.var_dvth_dn7 = ((locals.var_dvthsc_dn7 + locals.var_dvthlp_dn7) + locals.var_dvthw_dn7);
        locals.var_dvth_dn8 = ((locals.var_dvthsc_dn8 + locals.var_dvthlp_dn8) + locals.var_dvthw_dn8);
        locals.var_dvth_dn9 = ((locals.var_dvthsc_dn9 + locals.var_dvthlp_dn9) + locals.var_dvthw_dn9);
        locals.var_dvth_dn10 = ((locals.var_dvthsc_dn10 + locals.var_dvthlp_dn10) + locals.var_dvthw_dn10);
        locals.var_dvth_dn11 = ((locals.var_dvthsc_dn11 + locals.var_dvthlp_dn11) + locals.var_dvthw_dn11);
        locals.var_dvth_dn14 = ((locals.var_dvthsc_dn14 + locals.var_dvthlp_dn14) + locals.var_dvthw_dn14);

        let assign23520_e18038: f64 = (locals.var_pb2 - locals.var_vbsz);
        let assign23520_e18039: f64 = (locals.var_qnsub_esi2 * assign23520_e18038);
        let assign23520_e18040: f64 = (assign23520_e18039).sqrt();
        locals.var_t2 = assign23520_e18040;
        locals.var_t2_dn0 = (((locals.var_qnsub_esi2_dn0 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn0 - locals.var_vbsz_dn0))) / (2.0 * assign23520_e18040));
        locals.var_t2_dn2 = (((locals.var_qnsub_esi2_dn2 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn2 - locals.var_vbsz_dn2))) / (2.0 * assign23520_e18040));
        locals.var_t2_dn4 = (((locals.var_qnsub_esi2_dn4 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn4 - locals.var_vbsz_dn4))) / (2.0 * assign23520_e18040));
        locals.var_t2_dn5 = (((locals.var_qnsub_esi2_dn5 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn5 - locals.var_vbsz_dn5))) / (2.0 * assign23520_e18040));
        locals.var_t2_dn6 = (((locals.var_qnsub_esi2_dn6 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn6 - locals.var_vbsz_dn6))) / (2.0 * assign23520_e18040));
        locals.var_t2_dn7 = (((locals.var_qnsub_esi2_dn7 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn7 - locals.var_vbsz_dn7))) / (2.0 * assign23520_e18040));
        locals.var_t2_dn8 = (((locals.var_qnsub_esi2_dn8 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn8 - locals.var_vbsz_dn8))) / (2.0 * assign23520_e18040));
        locals.var_t2_dn9 = (((locals.var_qnsub_esi2_dn9 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn9 - locals.var_vbsz_dn9))) / (2.0 * assign23520_e18040));
        locals.var_t2_dn10 = (((locals.var_qnsub_esi2_dn10 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn10 - locals.var_vbsz_dn10))) / (2.0 * assign23520_e18040));
        locals.var_t2_dn11 = (((locals.var_qnsub_esi2_dn11 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn11 - locals.var_vbsz_dn11))) / (2.0 * assign23520_e18040));
        locals.var_t2_dn14 = (((locals.var_qnsub_esi2_dn14 * assign23520_e18038) + (locals.var_qnsub_esi2 * (locals.var_pb2_dn14 - locals.var_vbsz_dn14))) / (2.0 * assign23520_e18040));

        let assign23530_e18043: f64 = (locals.var_pb2 + locals.var_vfb);
        let assign23530_e18046: f64 = (locals.var_t2 * locals.var_cox0_inv);
        let assign23530_e18047: f64 = (assign23530_e18043 + assign23530_e18046);
        let assign23530_e18049: f64 = (assign23530_e18047 - locals.var_dvth);
        locals.var_vth = assign23530_e18049;

        let assign23540_e18052: f64 = (locals.var_cnst0 * locals.var_cox_inv);
        locals.var_fac1 = assign23540_e18052;
        locals.var_fac1_dn0 = ((locals.var_cnst0_dn0 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn0));
        locals.var_fac1_dn2 = ((locals.var_cnst0_dn2 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn2));
        locals.var_fac1_dn4 = ((locals.var_cnst0_dn4 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn4));
        locals.var_fac1_dn5 = ((locals.var_cnst0_dn5 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn5));
        locals.var_fac1_dn6 = ((locals.var_cnst0_dn6 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn6));
        locals.var_fac1_dn7 = ((locals.var_cnst0_dn7 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn7));
        locals.var_fac1_dn8 = ((locals.var_cnst0_dn8 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn8));
        locals.var_fac1_dn9 = ((locals.var_cnst0_dn9 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn9));
        locals.var_fac1_dn10 = ((locals.var_cnst0_dn10 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn10));
        locals.var_fac1_dn11 = ((locals.var_cnst0_dn11 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn11));
        locals.var_fac1_dn14 = ((locals.var_cnst0_dn14 * locals.var_cox_inv) + (locals.var_cnst0 * locals.var_cox_inv_dn14));

        let assign23550_e18055: f64 = (locals.var_fac1 * locals.var_fac1);
        locals.var_fac1p2 = assign23550_e18055;
        locals.var_fac1p2_dn0 = ((locals.var_fac1_dn0 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn0));
        locals.var_fac1p2_dn2 = ((locals.var_fac1_dn2 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn2));
        locals.var_fac1p2_dn4 = ((locals.var_fac1_dn4 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn4));
        locals.var_fac1p2_dn5 = ((locals.var_fac1_dn5 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn5));
        locals.var_fac1p2_dn6 = ((locals.var_fac1_dn6 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn6));
        locals.var_fac1p2_dn7 = ((locals.var_fac1_dn7 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn7));
        locals.var_fac1p2_dn8 = ((locals.var_fac1_dn8 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn8));
        locals.var_fac1p2_dn9 = ((locals.var_fac1_dn9 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn9));
        locals.var_fac1p2_dn10 = ((locals.var_fac1_dn10 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn10));
        locals.var_fac1p2_dn11 = ((locals.var_fac1_dn11 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn11));
        locals.var_fac1p2_dn14 = ((locals.var_fac1_dn14 * locals.var_fac1) + (locals.var_fac1 * locals.var_fac1_dn14));

        locals.var_dppg = 0.0;
        locals.var_dppg_dn0 = 0.0;
        locals.var_dppg_dn2 = 0.0;
        locals.var_dppg_dn4 = 0.0;
        locals.var_dppg_dn5 = 0.0;
        locals.var_dppg_dn6 = 0.0;
        locals.var_dppg_dn7 = 0.0;
        locals.var_dppg_dn8 = 0.0;
        locals.var_dppg_dn9 = 0.0;
        locals.var_dppg_dn10 = 0.0;
        locals.var_dppg_dn11 = 0.0;
        locals.var_dppg_dn14 = 0.0;

        let assign23570_e18059: f64 = if locals.var_flg_pgd == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard433 = assign23570_e18059;

        let (assign23580_e18063, assign23580_e18063_d_n0, assign23580_e18063_d_n2, assign23580_e18063_d_n4, assign23580_e18063_d_n5, assign23580_e18063_d_n6, assign23580_e18063_d_n7, assign23580_e18063_d_n8, assign23580_e18063_d_n9, assign23580_e18063_d_n10, assign23580_e18063_d_n11, assign23580_e18063_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        (locals.var_vgsz, locals.var_vgsz_dn0, locals.var_vgsz_dn2, locals.var_vgsz_dn4, locals.var_vgsz_dn5, locals.var_vgsz_dn6, locals.var_vgsz_dn7, locals.var_vgsz_dn8, locals.var_vgsz_dn9, locals.var_vgsz_dn10, locals.var_vgsz_dn11, locals.var_vgsz_dn14,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign23580_e18063;
        locals.var_t7_dn0 = assign23580_e18063_d_n0;
        locals.var_t7_dn2 = assign23580_e18063_d_n2;
        locals.var_t7_dn4 = assign23580_e18063_d_n4;
        locals.var_t7_dn5 = assign23580_e18063_d_n5;
        locals.var_t7_dn6 = assign23580_e18063_d_n6;
        locals.var_t7_dn7 = assign23580_e18063_d_n7;
        locals.var_t7_dn8 = assign23580_e18063_d_n8;
        locals.var_t7_dn9 = assign23580_e18063_d_n9;
        locals.var_t7_dn10 = assign23580_e18063_d_n10;
        locals.var_t7_dn11 = assign23580_e18063_d_n11;
        locals.var_t7_dn14 = assign23580_e18063_d_n14;

        let (assign23590_e18067, assign23590_e18067_d_n0, assign23590_e18067_d_n2, assign23590_e18067_d_n4, assign23590_e18067_d_n5, assign23590_e18067_d_n6, assign23590_e18067_d_n7, assign23590_e18067_d_n8, assign23590_e18067_d_n9, assign23590_e18067_d_n10, assign23590_e18067_d_n11, assign23590_e18067_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        (locals.var_cnstpgd, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign23590_e18067;
        locals.var_t0_dn0 = assign23590_e18067_d_n0;
        locals.var_t0_dn2 = assign23590_e18067_d_n2;
        locals.var_t0_dn4 = assign23590_e18067_d_n4;
        locals.var_t0_dn5 = assign23590_e18067_d_n5;
        locals.var_t0_dn6 = assign23590_e18067_d_n6;
        locals.var_t0_dn7 = assign23590_e18067_d_n7;
        locals.var_t0_dn8 = assign23590_e18067_d_n8;
        locals.var_t0_dn9 = assign23590_e18067_d_n9;
        locals.var_t0_dn10 = assign23590_e18067_d_n10;
        locals.var_t0_dn11 = assign23590_e18067_d_n11;
        locals.var_t0_dn14 = assign23590_e18067_d_n14;

        let (assign23600_e18073, assign23600_e18073_d_n0, assign23600_e18073_d_n2, assign23600_e18073_d_n4, assign23600_e18073_d_n5, assign23600_e18073_d_n6, assign23600_e18073_d_n7, assign23600_e18073_d_n8, assign23600_e18073_d_n9, assign23600_e18073_d_n10, assign23600_e18073_d_n11, assign23600_e18073_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let assign23600_e18071: f64 = (locals.var_t7 - p.p152);
        (assign23600_e18071, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign23600_e18073;
        locals.var_t3_dn0 = assign23600_e18073_d_n0;
        locals.var_t3_dn2 = assign23600_e18073_d_n2;
        locals.var_t3_dn4 = assign23600_e18073_d_n4;
        locals.var_t3_dn5 = assign23600_e18073_d_n5;
        locals.var_t3_dn6 = assign23600_e18073_d_n6;
        locals.var_t3_dn7 = assign23600_e18073_d_n7;
        locals.var_t3_dn8 = assign23600_e18073_d_n8;
        locals.var_t3_dn9 = assign23600_e18073_d_n9;
        locals.var_t3_dn10 = assign23600_e18073_d_n10;
        locals.var_t3_dn11 = assign23600_e18073_d_n11;
        locals.var_t3_dn14 = assign23600_e18073_d_n14;

        let assign23610_e18076: f64 = (-3.0);
        let assign23610_e18077: f64 = if locals.var_t3 < assign23610_e18076 { 1.0 } else { 0.0 };
        locals.var_guard434 = assign23610_e18077;

    }

    pub(super) fn stamp_transient_block_61(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23620_e18083, assign23620_e18083_d_n0, assign23620_e18083_d_n2, assign23620_e18083_d_n4, assign23620_e18083_d_n5, assign23620_e18083_d_n6, assign23620_e18083_d_n7, assign23620_e18083_d_n8, assign23620_e18083_d_n9, assign23620_e18083_d_n10, assign23620_e18083_d_n11, assign23620_e18083_d_n14,) = {
    if ((locals.var_guard433 != 0.0) && (locals.var_guard434 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23620_e18083;
        locals.var_t6_dn0 = assign23620_e18083_d_n0;
        locals.var_t6_dn2 = assign23620_e18083_d_n2;
        locals.var_t6_dn4 = assign23620_e18083_d_n4;
        locals.var_t6_dn5 = assign23620_e18083_d_n5;
        locals.var_t6_dn6 = assign23620_e18083_d_n6;
        locals.var_t6_dn7 = assign23620_e18083_d_n7;
        locals.var_t6_dn8 = assign23620_e18083_d_n8;
        locals.var_t6_dn9 = assign23620_e18083_d_n9;
        locals.var_t6_dn10 = assign23620_e18083_d_n10;
        locals.var_t6_dn11 = assign23620_e18083_d_n11;
        locals.var_t6_dn14 = assign23620_e18083_d_n14;

        let (assign23630_e18089, assign23630_e18089_d_n0, assign23630_e18089_d_n2, assign23630_e18089_d_n4, assign23630_e18089_d_n5, assign23630_e18089_d_n6, assign23630_e18089_d_n7, assign23630_e18089_d_n8, assign23630_e18089_d_n9, assign23630_e18089_d_n10, assign23630_e18089_d_n11, assign23630_e18089_d_n14,) = {
    if ((locals.var_guard433 != 0.0) && (locals.var_guard434 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn14,)
    }
};
        locals.var_dppg = assign23630_e18089;
        locals.var_dppg_dn0 = assign23630_e18089_d_n0;
        locals.var_dppg_dn2 = assign23630_e18089_d_n2;
        locals.var_dppg_dn4 = assign23630_e18089_d_n4;
        locals.var_dppg_dn5 = assign23630_e18089_d_n5;
        locals.var_dppg_dn6 = assign23630_e18089_d_n6;
        locals.var_dppg_dn7 = assign23630_e18089_d_n7;
        locals.var_dppg_dn8 = assign23630_e18089_d_n8;
        locals.var_dppg_dn9 = assign23630_e18089_d_n9;
        locals.var_dppg_dn10 = assign23630_e18089_d_n10;
        locals.var_dppg_dn11 = assign23630_e18089_d_n11;
        locals.var_dppg_dn14 = assign23630_e18089_d_n14;

        let assign23640_e18092: f64 = if locals.var_t3 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard435 = assign23640_e18092;

        let (assign23650_e18117, assign23650_e18117_d_n0, assign23650_e18117_d_n2, assign23650_e18117_d_n4, assign23650_e18117_d_n5, assign23650_e18117_d_n6, assign23650_e18117_d_n7, assign23650_e18117_d_n8, assign23650_e18117_d_n9, assign23650_e18117_d_n10, assign23650_e18117_d_n11, assign23650_e18117_d_n14,) = {
    if (((locals.var_guard433 != 0.0) && (locals.var_guard434 == 0.0)) && (locals.var_guard435 != 0.0)) {
        let assign23650_e18104: f64 = (1.0 / 3.0);
        let assign23650_e18105: f64 = (2.0 * assign23650_e18104);
        let assign23650_e18108: f64 = (locals.var_t3 * 3.0);
        let assign23650_e18111: f64 = (1.0 / 27.0);
        let assign23650_e18112: f64 = (assign23650_e18108 * assign23650_e18111);
        let assign23650_e18113: f64 = (assign23650_e18105 + assign23650_e18112);
        let assign23650_e18114: f64 = (locals.var_t3 * assign23650_e18113);
        let assign23650_e18115: f64 = (1.0 + assign23650_e18114);
        (assign23650_e18115, ((locals.var_t3_dn0 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn0 * 3.0) * assign23650_e18111))), ((locals.var_t3_dn2 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn2 * 3.0) * assign23650_e18111))), ((locals.var_t3_dn4 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn4 * 3.0) * assign23650_e18111))), ((locals.var_t3_dn5 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn5 * 3.0) * assign23650_e18111))), ((locals.var_t3_dn6 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn6 * 3.0) * assign23650_e18111))), ((locals.var_t3_dn7 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn7 * 3.0) * assign23650_e18111))), ((locals.var_t3_dn8 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn8 * 3.0) * assign23650_e18111))), ((locals.var_t3_dn9 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn9 * 3.0) * assign23650_e18111))), ((locals.var_t3_dn10 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn10 * 3.0) * assign23650_e18111))), ((locals.var_t3_dn11 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn11 * 3.0) * assign23650_e18111))), ((locals.var_t3_dn14 * assign23650_e18113) + (locals.var_t3 * ((locals.var_t3_dn14 * 3.0) * assign23650_e18111))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23650_e18117;
        locals.var_t6_dn0 = assign23650_e18117_d_n0;
        locals.var_t6_dn2 = assign23650_e18117_d_n2;
        locals.var_t6_dn4 = assign23650_e18117_d_n4;
        locals.var_t6_dn5 = assign23650_e18117_d_n5;
        locals.var_t6_dn6 = assign23650_e18117_d_n6;
        locals.var_t6_dn7 = assign23650_e18117_d_n7;
        locals.var_t6_dn8 = assign23650_e18117_d_n8;
        locals.var_t6_dn9 = assign23650_e18117_d_n9;
        locals.var_t6_dn10 = assign23650_e18117_d_n10;
        locals.var_t6_dn11 = assign23650_e18117_d_n11;
        locals.var_t6_dn14 = assign23650_e18117_d_n14;

        let (assign23660_e18142, assign23660_e18142_d_n0, assign23660_e18142_d_n2, assign23660_e18142_d_n4, assign23660_e18142_d_n5, assign23660_e18142_d_n6, assign23660_e18142_d_n7, assign23660_e18142_d_n8, assign23660_e18142_d_n9, assign23660_e18142_d_n10, assign23660_e18142_d_n11, assign23660_e18142_d_n14,) = {
    if (((locals.var_guard433 != 0.0) && (locals.var_guard434 == 0.0)) && (locals.var_guard435 != 0.0)) {
        let assign23660_e18130: f64 = (1.0 / 3.0);
        let assign23660_e18134: f64 = (1.0 / 27.0);
        let assign23660_e18135: f64 = (locals.var_t3 * assign23660_e18134);
        let assign23660_e18136: f64 = (assign23660_e18130 + assign23660_e18135);
        let assign23660_e18137: f64 = (locals.var_t3 * assign23660_e18136);
        let assign23660_e18138: f64 = (1.0 + assign23660_e18137);
        let assign23660_e18139: f64 = (locals.var_t3 * assign23660_e18138);
        let assign23660_e18140: f64 = (1.0 + assign23660_e18139);
        (assign23660_e18140, ((locals.var_t3_dn0 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn0 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn0 * assign23660_e18134))))), ((locals.var_t3_dn2 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn2 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn2 * assign23660_e18134))))), ((locals.var_t3_dn4 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn4 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn4 * assign23660_e18134))))), ((locals.var_t3_dn5 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn5 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn5 * assign23660_e18134))))), ((locals.var_t3_dn6 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn6 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn6 * assign23660_e18134))))), ((locals.var_t3_dn7 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn7 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn7 * assign23660_e18134))))), ((locals.var_t3_dn8 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn8 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn8 * assign23660_e18134))))), ((locals.var_t3_dn9 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn9 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn9 * assign23660_e18134))))), ((locals.var_t3_dn10 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn10 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn10 * assign23660_e18134))))), ((locals.var_t3_dn11 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn11 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn11 * assign23660_e18134))))), ((locals.var_t3_dn14 * assign23660_e18138) + (locals.var_t3 * ((locals.var_t3_dn14 * assign23660_e18136) + (locals.var_t3 * (locals.var_t3_dn14 * assign23660_e18134))))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn14,)
    }
};
        locals.var_dppg = assign23660_e18142;
        locals.var_dppg_dn0 = assign23660_e18142_d_n0;
        locals.var_dppg_dn2 = assign23660_e18142_d_n2;
        locals.var_dppg_dn4 = assign23660_e18142_d_n4;
        locals.var_dppg_dn5 = assign23660_e18142_d_n5;
        locals.var_dppg_dn6 = assign23660_e18142_d_n6;
        locals.var_dppg_dn7 = assign23660_e18142_d_n7;
        locals.var_dppg_dn8 = assign23660_e18142_d_n8;
        locals.var_dppg_dn9 = assign23660_e18142_d_n9;
        locals.var_dppg_dn10 = assign23660_e18142_d_n10;
        locals.var_dppg_dn11 = assign23660_e18142_d_n11;
        locals.var_dppg_dn14 = assign23660_e18142_d_n14;

        let (assign23670_e18172, assign23670_e18172_d_n0, assign23670_e18172_d_n2, assign23670_e18172_d_n4, assign23670_e18172_d_n5, assign23670_e18172_d_n6, assign23670_e18172_d_n7, assign23670_e18172_d_n8, assign23670_e18172_d_n9, assign23670_e18172_d_n10, assign23670_e18172_d_n11, assign23670_e18172_d_n14,) = {
    if (((locals.var_guard433 != 0.0) && (locals.var_guard434 == 0.0)) && (locals.var_guard435 == 0.0)) {
        let assign23670_e18155: f64 = (1.0 / 3.0);
        let assign23670_e18156: f64 = (2.0 * assign23670_e18155);
        let assign23670_e18160: f64 = (3.0 * 0.0402052934513951);
        let assign23670_e18163: f64 = (locals.var_t3 * 4.0);
        let assign23670_e18165: f64 = (assign23670_e18163 * 0.148148111111111);
        let assign23670_e18166: f64 = (assign23670_e18160 + assign23670_e18165);
        let assign23670_e18167: f64 = (locals.var_t3 * assign23670_e18166);
        let assign23670_e18168: f64 = (assign23670_e18156 + assign23670_e18167);
        let assign23670_e18169: f64 = (locals.var_t3 * assign23670_e18168);
        let assign23670_e18170: f64 = (1.0 + assign23670_e18169);
        (assign23670_e18170, ((locals.var_t3_dn0 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn0 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn0 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn2 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn2 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn2 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn4 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn4 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn4 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn5 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn5 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn5 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn6 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn6 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn6 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn7 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn7 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn7 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn8 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn8 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn8 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn9 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn9 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn9 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn10 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn10 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn10 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn11 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn11 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn11 * 4.0) * 0.148148111111111))))), ((locals.var_t3_dn14 * assign23670_e18168) + (locals.var_t3 * ((locals.var_t3_dn14 * assign23670_e18166) + (locals.var_t3 * ((locals.var_t3_dn14 * 4.0) * 0.148148111111111))))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23670_e18172;
        locals.var_t6_dn0 = assign23670_e18172_d_n0;
        locals.var_t6_dn2 = assign23670_e18172_d_n2;
        locals.var_t6_dn4 = assign23670_e18172_d_n4;
        locals.var_t6_dn5 = assign23670_e18172_d_n5;
        locals.var_t6_dn6 = assign23670_e18172_d_n6;
        locals.var_t6_dn7 = assign23670_e18172_d_n7;
        locals.var_t6_dn8 = assign23670_e18172_d_n8;
        locals.var_t6_dn9 = assign23670_e18172_d_n9;
        locals.var_t6_dn10 = assign23670_e18172_d_n10;
        locals.var_t6_dn11 = assign23670_e18172_d_n11;
        locals.var_t6_dn14 = assign23670_e18172_d_n14;

        let (assign23680_e18200, assign23680_e18200_d_n0, assign23680_e18200_d_n2, assign23680_e18200_d_n4, assign23680_e18200_d_n5, assign23680_e18200_d_n6, assign23680_e18200_d_n7, assign23680_e18200_d_n8, assign23680_e18200_d_n9, assign23680_e18200_d_n10, assign23680_e18200_d_n11, assign23680_e18200_d_n14,) = {
    if (((locals.var_guard433 != 0.0) && (locals.var_guard434 == 0.0)) && (locals.var_guard435 == 0.0)) {
        let assign23680_e18186: f64 = (1.0 / 3.0);
        let assign23680_e18191: f64 = (locals.var_t3 * 0.148148111111111);
        let assign23680_e18192: f64 = (0.0402052934513951 + assign23680_e18191);
        let assign23680_e18193: f64 = (locals.var_t3 * assign23680_e18192);
        let assign23680_e18194: f64 = (assign23680_e18186 + assign23680_e18193);
        let assign23680_e18195: f64 = (locals.var_t3 * assign23680_e18194);
        let assign23680_e18196: f64 = (1.0 + assign23680_e18195);
        let assign23680_e18197: f64 = (locals.var_t3 * assign23680_e18196);
        let assign23680_e18198: f64 = (1.0 + assign23680_e18197);
        (assign23680_e18198, ((locals.var_t3_dn0 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn0 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn0 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn0 * 0.148148111111111))))))), ((locals.var_t3_dn2 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn2 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn2 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn2 * 0.148148111111111))))))), ((locals.var_t3_dn4 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn4 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn4 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn4 * 0.148148111111111))))))), ((locals.var_t3_dn5 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn5 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn5 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn5 * 0.148148111111111))))))), ((locals.var_t3_dn6 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn6 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn6 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn6 * 0.148148111111111))))))), ((locals.var_t3_dn7 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn7 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn7 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn7 * 0.148148111111111))))))), ((locals.var_t3_dn8 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn8 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn8 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn8 * 0.148148111111111))))))), ((locals.var_t3_dn9 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn9 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn9 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn9 * 0.148148111111111))))))), ((locals.var_t3_dn10 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn10 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn10 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn10 * 0.148148111111111))))))), ((locals.var_t3_dn11 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn11 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn11 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn11 * 0.148148111111111))))))), ((locals.var_t3_dn14 * assign23680_e18196) + (locals.var_t3 * ((locals.var_t3_dn14 * assign23680_e18194) + (locals.var_t3 * ((locals.var_t3_dn14 * assign23680_e18192) + (locals.var_t3 * (locals.var_t3_dn14 * 0.148148111111111))))))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn14,)
    }
};
        locals.var_dppg = assign23680_e18200;
        locals.var_dppg_dn0 = assign23680_e18200_d_n0;
        locals.var_dppg_dn2 = assign23680_e18200_d_n2;
        locals.var_dppg_dn4 = assign23680_e18200_d_n4;
        locals.var_dppg_dn5 = assign23680_e18200_d_n5;
        locals.var_dppg_dn6 = assign23680_e18200_d_n6;
        locals.var_dppg_dn7 = assign23680_e18200_d_n7;
        locals.var_dppg_dn8 = assign23680_e18200_d_n8;
        locals.var_dppg_dn9 = assign23680_e18200_d_n9;
        locals.var_dppg_dn10 = assign23680_e18200_d_n10;
        locals.var_dppg_dn11 = assign23680_e18200_d_n11;
        locals.var_dppg_dn14 = assign23680_e18200_d_n14;

        let (assign23690_e18217, assign23690_e18217_d_n0, assign23690_e18217_d_n2, assign23690_e18217_d_n4, assign23690_e18217_d_n5, assign23690_e18217_d_n6, assign23690_e18217_d_n7, assign23690_e18217_d_n8, assign23690_e18217_d_n9, assign23690_e18217_d_n10, assign23690_e18217_d_n11, assign23690_e18217_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let assign23690_e18204: f64 = (locals.var_dppg - 1.0);
        let assign23690_e18207: f64 = (locals.var_dppg - 1.0);
        let assign23690_e18208: f64 = (assign23690_e18204 * assign23690_e18207);
        let assign23690_e18211: f64 = (4.0 * 0.05);
        let assign23690_e18213: f64 = (assign23690_e18211 * 0.05);
        let assign23690_e18214: f64 = (assign23690_e18208 + assign23690_e18213);
        let assign23690_e18215: f64 = (assign23690_e18214).sqrt();
        (assign23690_e18215, (((locals.var_dppg_dn0 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn0)) / (2.0 * assign23690_e18215)), (((locals.var_dppg_dn2 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn2)) / (2.0 * assign23690_e18215)), (((locals.var_dppg_dn4 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn4)) / (2.0 * assign23690_e18215)), (((locals.var_dppg_dn5 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn5)) / (2.0 * assign23690_e18215)), (((locals.var_dppg_dn6 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn6)) / (2.0 * assign23690_e18215)), (((locals.var_dppg_dn7 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn7)) / (2.0 * assign23690_e18215)), (((locals.var_dppg_dn8 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn8)) / (2.0 * assign23690_e18215)), (((locals.var_dppg_dn9 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn9)) / (2.0 * assign23690_e18215)), (((locals.var_dppg_dn10 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn10)) / (2.0 * assign23690_e18215)), (((locals.var_dppg_dn11 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn11)) / (2.0 * assign23690_e18215)), (((locals.var_dppg_dn14 * assign23690_e18207) + (assign23690_e18204 * locals.var_dppg_dn14)) / (2.0 * assign23690_e18215)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23690_e18217;
        locals.var_tmf2_dn0 = assign23690_e18217_d_n0;
        locals.var_tmf2_dn2 = assign23690_e18217_d_n2;
        locals.var_tmf2_dn4 = assign23690_e18217_d_n4;
        locals.var_tmf2_dn5 = assign23690_e18217_d_n5;
        locals.var_tmf2_dn6 = assign23690_e18217_d_n6;
        locals.var_tmf2_dn7 = assign23690_e18217_d_n7;
        locals.var_tmf2_dn8 = assign23690_e18217_d_n8;
        locals.var_tmf2_dn9 = assign23690_e18217_d_n9;
        locals.var_tmf2_dn10 = assign23690_e18217_d_n10;
        locals.var_tmf2_dn11 = assign23690_e18217_d_n11;
        locals.var_tmf2_dn14 = assign23690_e18217_d_n14;

        let (assign23700_e18229, assign23700_e18229_d_n0, assign23700_e18229_d_n2, assign23700_e18229_d_n4, assign23700_e18229_d_n5, assign23700_e18229_d_n6, assign23700_e18229_d_n7, assign23700_e18229_d_n8, assign23700_e18229_d_n9, assign23700_e18229_d_n10, assign23700_e18229_d_n11, assign23700_e18229_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let assign23700_e18223: f64 = (locals.var_dppg - 1.0);
        let assign23700_e18225: f64 = (assign23700_e18223 / locals.var_tmf2);
        let assign23700_e18226: f64 = (1.0 + assign23700_e18225);
        let assign23700_e18227: f64 = (0.5 * assign23700_e18226);
        (assign23700_e18227, (0.5 * (((locals.var_dppg_dn0 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn2 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn4 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn5 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn6 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn7 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn8 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn9 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn10 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn11 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_dppg_dn14 * locals.var_tmf2) - (assign23700_e18223 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23700_e18229;
        locals.var_t6_dn0 = assign23700_e18229_d_n0;
        locals.var_t6_dn2 = assign23700_e18229_d_n2;
        locals.var_t6_dn4 = assign23700_e18229_d_n4;
        locals.var_t6_dn5 = assign23700_e18229_d_n5;
        locals.var_t6_dn6 = assign23700_e18229_d_n6;
        locals.var_t6_dn7 = assign23700_e18229_d_n7;
        locals.var_t6_dn8 = assign23700_e18229_d_n8;
        locals.var_t6_dn9 = assign23700_e18229_d_n9;
        locals.var_t6_dn10 = assign23700_e18229_d_n10;
        locals.var_t6_dn11 = assign23700_e18229_d_n11;
        locals.var_t6_dn14 = assign23700_e18229_d_n14;

        let (assign23710_e18239, assign23710_e18239_d_n0, assign23710_e18239_d_n2, assign23710_e18239_d_n4, assign23710_e18239_d_n5, assign23710_e18239_d_n6, assign23710_e18239_d_n7, assign23710_e18239_d_n8, assign23710_e18239_d_n9, assign23710_e18239_d_n10, assign23710_e18239_d_n11, assign23710_e18239_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let assign23710_e18234: f64 = (locals.var_dppg - 1.0);
        let assign23710_e18236: f64 = (assign23710_e18234 + locals.var_tmf2);
        let assign23710_e18237: f64 = (0.5 * assign23710_e18236);
        (assign23710_e18237, (0.5 * (locals.var_dppg_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_dppg_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_dppg_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_dppg_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_dppg_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_dppg_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_dppg_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_dppg_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_dppg_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_dppg_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_dppg_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn14,)
    }
};
        locals.var_dppg = assign23710_e18239;
        locals.var_dppg_dn0 = assign23710_e18239_d_n0;
        locals.var_dppg_dn2 = assign23710_e18239_d_n2;
        locals.var_dppg_dn4 = assign23710_e18239_d_n4;
        locals.var_dppg_dn5 = assign23710_e18239_d_n5;
        locals.var_dppg_dn6 = assign23710_e18239_d_n6;
        locals.var_dppg_dn7 = assign23710_e18239_d_n7;
        locals.var_dppg_dn8 = assign23710_e18239_d_n8;
        locals.var_dppg_dn9 = assign23710_e18239_d_n9;
        locals.var_dppg_dn10 = assign23710_e18239_d_n10;
        locals.var_dppg_dn11 = assign23710_e18239_d_n11;
        locals.var_dppg_dn14 = assign23710_e18239_d_n14;

        let assign23720_e18242: f64 = if locals.var_dppg < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard436 = assign23720_e18242;

        let (assign23730_e18248, assign23730_e18248_d_n0, assign23730_e18248_d_n2, assign23730_e18248_d_n4, assign23730_e18248_d_n5, assign23730_e18248_d_n6, assign23730_e18248_d_n7, assign23730_e18248_d_n8, assign23730_e18248_d_n9, assign23730_e18248_d_n10, assign23730_e18248_d_n11, assign23730_e18248_d_n14,) = {
    if ((locals.var_guard433 != 0.0) && (locals.var_guard436 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn14,)
    }
};
        locals.var_dppg = assign23730_e18248;
        locals.var_dppg_dn0 = assign23730_e18248_d_n0;
        locals.var_dppg_dn2 = assign23730_e18248_d_n2;
        locals.var_dppg_dn4 = assign23730_e18248_d_n4;
        locals.var_dppg_dn5 = assign23730_e18248_d_n5;
        locals.var_dppg_dn6 = assign23730_e18248_d_n6;
        locals.var_dppg_dn7 = assign23730_e18248_d_n7;
        locals.var_dppg_dn8 = assign23730_e18248_d_n8;
        locals.var_dppg_dn9 = assign23730_e18248_d_n9;
        locals.var_dppg_dn10 = assign23730_e18248_d_n10;
        locals.var_dppg_dn11 = assign23730_e18248_d_n11;
        locals.var_dppg_dn14 = assign23730_e18248_d_n14;

        let (assign23740_e18254, assign23740_e18254_d_n0, assign23740_e18254_d_n2, assign23740_e18254_d_n4, assign23740_e18254_d_n5, assign23740_e18254_d_n6, assign23740_e18254_d_n7, assign23740_e18254_d_n8, assign23740_e18254_d_n9, assign23740_e18254_d_n10, assign23740_e18254_d_n11, assign23740_e18254_d_n14,) = {
    if ((locals.var_guard433 != 0.0) && (locals.var_guard436 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23740_e18254;
        locals.var_t6_dn0 = assign23740_e18254_d_n0;
        locals.var_t6_dn2 = assign23740_e18254_d_n2;
        locals.var_t6_dn4 = assign23740_e18254_d_n4;
        locals.var_t6_dn5 = assign23740_e18254_d_n5;
        locals.var_t6_dn6 = assign23740_e18254_d_n6;
        locals.var_t6_dn7 = assign23740_e18254_d_n7;
        locals.var_t6_dn8 = assign23740_e18254_d_n8;
        locals.var_t6_dn9 = assign23740_e18254_d_n9;
        locals.var_t6_dn10 = assign23740_e18254_d_n10;
        locals.var_t6_dn11 = assign23740_e18254_d_n11;
        locals.var_t6_dn14 = assign23740_e18254_d_n14;

        let (assign23750_e18260, assign23750_e18260_d_n0, assign23750_e18260_d_n2, assign23750_e18260_d_n4, assign23750_e18260_d_n5, assign23750_e18260_d_n6, assign23750_e18260_d_n7, assign23750_e18260_d_n8, assign23750_e18260_d_n9, assign23750_e18260_d_n10, assign23750_e18260_d_n11, assign23750_e18260_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let assign23750_e18258: f64 = (locals.var_dppg * locals.var_t0);
        (assign23750_e18258, ((locals.var_dppg_dn0 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn0)), ((locals.var_dppg_dn2 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn2)), ((locals.var_dppg_dn4 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn4)), ((locals.var_dppg_dn5 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn5)), ((locals.var_dppg_dn6 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn6)), ((locals.var_dppg_dn7 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn7)), ((locals.var_dppg_dn8 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn8)), ((locals.var_dppg_dn9 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn9)), ((locals.var_dppg_dn10 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn10)), ((locals.var_dppg_dn11 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn11)), ((locals.var_dppg_dn14 * locals.var_t0) + (locals.var_dppg * locals.var_t0_dn14)),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn14,)
    }
};
        locals.var_dppg = assign23750_e18260;
        locals.var_dppg_dn0 = assign23750_e18260_d_n0;
        locals.var_dppg_dn2 = assign23750_e18260_d_n2;
        locals.var_dppg_dn4 = assign23750_e18260_d_n4;
        locals.var_dppg_dn5 = assign23750_e18260_d_n5;
        locals.var_dppg_dn6 = assign23750_e18260_d_n6;
        locals.var_dppg_dn7 = assign23750_e18260_d_n7;
        locals.var_dppg_dn8 = assign23750_e18260_d_n8;
        locals.var_dppg_dn9 = assign23750_e18260_d_n9;
        locals.var_dppg_dn10 = assign23750_e18260_d_n10;
        locals.var_dppg_dn11 = assign23750_e18260_d_n11;
        locals.var_dppg_dn14 = assign23750_e18260_d_n14;

        let (assign23760_e18268, assign23760_e18268_d_n0, assign23760_e18268_d_n2, assign23760_e18268_d_n4, assign23760_e18268_d_n5, assign23760_e18268_d_n6, assign23760_e18268_d_n7, assign23760_e18268_d_n8, assign23760_e18268_d_n9, assign23760_e18268_d_n10, assign23760_e18268_d_n11, assign23760_e18268_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let assign23760_e18264: f64 = (1.0 - locals.var_dppg);
        let assign23760_e18266: f64 = (assign23760_e18264 - 0.05);
        (assign23760_e18266, (-locals.var_dppg_dn0), (-locals.var_dppg_dn2), (-locals.var_dppg_dn4), (-locals.var_dppg_dn5), (-locals.var_dppg_dn6), (-locals.var_dppg_dn7), (-locals.var_dppg_dn8), (-locals.var_dppg_dn9), (-locals.var_dppg_dn10), (-locals.var_dppg_dn11), (-locals.var_dppg_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign23760_e18268;
        locals.var_tmf1_dn0 = assign23760_e18268_d_n0;
        locals.var_tmf1_dn2 = assign23760_e18268_d_n2;
        locals.var_tmf1_dn4 = assign23760_e18268_d_n4;
        locals.var_tmf1_dn5 = assign23760_e18268_d_n5;
        locals.var_tmf1_dn6 = assign23760_e18268_d_n6;
        locals.var_tmf1_dn7 = assign23760_e18268_d_n7;
        locals.var_tmf1_dn8 = assign23760_e18268_d_n8;
        locals.var_tmf1_dn9 = assign23760_e18268_d_n9;
        locals.var_tmf1_dn10 = assign23760_e18268_d_n10;
        locals.var_tmf1_dn11 = assign23760_e18268_d_n11;
        locals.var_tmf1_dn14 = assign23760_e18268_d_n14;

        let (assign23770_e18276, assign23770_e18276_d_n0, assign23770_e18276_d_n2, assign23770_e18276_d_n4, assign23770_e18276_d_n5, assign23770_e18276_d_n6, assign23770_e18276_d_n7, assign23770_e18276_d_n8, assign23770_e18276_d_n9, assign23770_e18276_d_n10, assign23770_e18276_d_n11, assign23770_e18276_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let assign23770_e18272: f64 = 4.0;
        let assign23770_e18274: f64 = (assign23770_e18272 * 0.05);
        (assign23770_e18274, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23770_e18276;
        locals.var_tmf2_dn0 = assign23770_e18276_d_n0;
        locals.var_tmf2_dn2 = assign23770_e18276_d_n2;
        locals.var_tmf2_dn4 = assign23770_e18276_d_n4;
        locals.var_tmf2_dn5 = assign23770_e18276_d_n5;
        locals.var_tmf2_dn6 = assign23770_e18276_d_n6;
        locals.var_tmf2_dn7 = assign23770_e18276_d_n7;
        locals.var_tmf2_dn8 = assign23770_e18276_d_n8;
        locals.var_tmf2_dn9 = assign23770_e18276_d_n9;
        locals.var_tmf2_dn10 = assign23770_e18276_d_n10;
        locals.var_tmf2_dn11 = assign23770_e18276_d_n11;
        locals.var_tmf2_dn14 = assign23770_e18276_d_n14;

        let (assign23780_e18286, assign23780_e18286_d_n0, assign23780_e18286_d_n2, assign23780_e18286_d_n4, assign23780_e18286_d_n5, assign23780_e18286_d_n6, assign23780_e18286_d_n7, assign23780_e18286_d_n8, assign23780_e18286_d_n9, assign23780_e18286_d_n10, assign23780_e18286_d_n11, assign23780_e18286_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let (assign23780_e18284, assign23780_e18284_d_n0, assign23780_e18284_d_n2, assign23780_e18284_d_n4, assign23780_e18284_d_n5, assign23780_e18284_d_n6, assign23780_e18284_d_n7, assign23780_e18284_d_n8, assign23780_e18284_d_n9, assign23780_e18284_d_n10, assign23780_e18284_d_n11, assign23780_e18284_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign23780_e18283: f64 = (-locals.var_tmf2);
                (assign23780_e18283, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign23780_e18284, assign23780_e18284_d_n0, assign23780_e18284_d_n2, assign23780_e18284_d_n4, assign23780_e18284_d_n5, assign23780_e18284_d_n6, assign23780_e18284_d_n7, assign23780_e18284_d_n8, assign23780_e18284_d_n9, assign23780_e18284_d_n10, assign23780_e18284_d_n11, assign23780_e18284_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23780_e18286;
        locals.var_tmf2_dn0 = assign23780_e18286_d_n0;
        locals.var_tmf2_dn2 = assign23780_e18286_d_n2;
        locals.var_tmf2_dn4 = assign23780_e18286_d_n4;
        locals.var_tmf2_dn5 = assign23780_e18286_d_n5;
        locals.var_tmf2_dn6 = assign23780_e18286_d_n6;
        locals.var_tmf2_dn7 = assign23780_e18286_d_n7;
        locals.var_tmf2_dn8 = assign23780_e18286_d_n8;
        locals.var_tmf2_dn9 = assign23780_e18286_d_n9;
        locals.var_tmf2_dn10 = assign23780_e18286_d_n10;
        locals.var_tmf2_dn11 = assign23780_e18286_d_n11;
        locals.var_tmf2_dn14 = assign23780_e18286_d_n14;

        let (assign23790_e18295, assign23790_e18295_d_n0, assign23790_e18295_d_n2, assign23790_e18295_d_n4, assign23790_e18295_d_n5, assign23790_e18295_d_n6, assign23790_e18295_d_n7, assign23790_e18295_d_n8, assign23790_e18295_d_n9, assign23790_e18295_d_n10, assign23790_e18295_d_n11, assign23790_e18295_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let assign23790_e18290: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign23790_e18292: f64 = (assign23790_e18290 + locals.var_tmf2);
        let assign23790_e18293: f64 = (assign23790_e18292).sqrt();
        (assign23790_e18293, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign23790_e18293)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign23790_e18293)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign23790_e18293)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign23790_e18293)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign23790_e18293)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign23790_e18293)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign23790_e18293)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign23790_e18293)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign23790_e18293)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign23790_e18293)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign23790_e18293)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23790_e18295;
        locals.var_tmf2_dn0 = assign23790_e18295_d_n0;
        locals.var_tmf2_dn2 = assign23790_e18295_d_n2;
        locals.var_tmf2_dn4 = assign23790_e18295_d_n4;
        locals.var_tmf2_dn5 = assign23790_e18295_d_n5;
        locals.var_tmf2_dn6 = assign23790_e18295_d_n6;
        locals.var_tmf2_dn7 = assign23790_e18295_d_n7;
        locals.var_tmf2_dn8 = assign23790_e18295_d_n8;
        locals.var_tmf2_dn9 = assign23790_e18295_d_n9;
        locals.var_tmf2_dn10 = assign23790_e18295_d_n10;
        locals.var_tmf2_dn11 = assign23790_e18295_d_n11;
        locals.var_tmf2_dn14 = assign23790_e18295_d_n14;

        let (assign23800_e18305, assign23800_e18305_d_n0, assign23800_e18305_d_n2, assign23800_e18305_d_n4, assign23800_e18305_d_n5, assign23800_e18305_d_n6, assign23800_e18305_d_n7, assign23800_e18305_d_n8, assign23800_e18305_d_n9, assign23800_e18305_d_n10, assign23800_e18305_d_n11, assign23800_e18305_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let assign23800_e18301: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign23800_e18302: f64 = (1.0 + assign23800_e18301);
        let assign23800_e18303: f64 = (0.5 * assign23800_e18302);
        (assign23800_e18303, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t9, locals.var_t9_dn0, locals.var_t9_dn2, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn14,)
    }
};
        locals.var_t9 = assign23800_e18305;
        locals.var_t9_dn0 = assign23800_e18305_d_n0;
        locals.var_t9_dn2 = assign23800_e18305_d_n2;
        locals.var_t9_dn4 = assign23800_e18305_d_n4;
        locals.var_t9_dn5 = assign23800_e18305_d_n5;
        locals.var_t9_dn6 = assign23800_e18305_d_n6;
        locals.var_t9_dn7 = assign23800_e18305_d_n7;
        locals.var_t9_dn8 = assign23800_e18305_d_n8;
        locals.var_t9_dn9 = assign23800_e18305_d_n9;
        locals.var_t9_dn10 = assign23800_e18305_d_n10;
        locals.var_t9_dn11 = assign23800_e18305_d_n11;
        locals.var_t9_dn14 = assign23800_e18305_d_n14;

        let (assign23810_e18315, assign23810_e18315_d_n0, assign23810_e18315_d_n2, assign23810_e18315_d_n4, assign23810_e18315_d_n5, assign23810_e18315_d_n6, assign23810_e18315_d_n7, assign23810_e18315_d_n8, assign23810_e18315_d_n9, assign23810_e18315_d_n10, assign23810_e18315_d_n11, assign23810_e18315_d_n14,) = {
    if (locals.var_guard433 != 0.0) {
        let assign23810_e18311: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign23810_e18312: f64 = (0.5 * assign23810_e18311);
        let assign23810_e18313: f64 = (1.0 - assign23810_e18312);
        (assign23810_e18313, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_dppg, locals.var_dppg_dn0, locals.var_dppg_dn2, locals.var_dppg_dn4, locals.var_dppg_dn5, locals.var_dppg_dn6, locals.var_dppg_dn7, locals.var_dppg_dn8, locals.var_dppg_dn9, locals.var_dppg_dn10, locals.var_dppg_dn11, locals.var_dppg_dn14,)
    }
};
        locals.var_dppg = assign23810_e18315;
        locals.var_dppg_dn0 = assign23810_e18315_d_n0;
        locals.var_dppg_dn2 = assign23810_e18315_d_n2;
        locals.var_dppg_dn4 = assign23810_e18315_d_n4;
        locals.var_dppg_dn5 = assign23810_e18315_d_n5;
        locals.var_dppg_dn6 = assign23810_e18315_d_n6;
        locals.var_dppg_dn7 = assign23810_e18315_d_n7;
        locals.var_dppg_dn8 = assign23810_e18315_d_n8;
        locals.var_dppg_dn9 = assign23810_e18315_d_n9;
        locals.var_dppg_dn10 = assign23810_e18315_d_n10;
        locals.var_dppg_dn11 = assign23810_e18315_d_n11;
        locals.var_dppg_dn14 = assign23810_e18315_d_n14;

        let assign23820_e18318: f64 = if locals.var_vbs > locals.var_vbs_bnd_local { 1.0 } else { 0.0 };
        locals.var_guard443 = assign23820_e18318;

        let (assign23830_e18326, assign23830_e18326_d_n0, assign23830_e18326_d_n2, assign23830_e18326_d_n4, assign23830_e18326_d_n5, assign23830_e18326_d_n6, assign23830_e18326_d_n7, assign23830_e18326_d_n8, assign23830_e18326_d_n9, assign23830_e18326_d_n10, assign23830_e18326_d_n11, assign23830_e18326_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23830_e18324: f64 = (locals.var_vbs - locals.var_vbs_bnd_local);
        (assign23830_e18324, (-locals.var_vbs_bnd_local_dn0), (-locals.var_vbs_bnd_local_dn2), (-locals.var_vbs_bnd_local_dn4), (-locals.var_vbs_bnd_local_dn5), (locals.var_vbs_dn6 - locals.var_vbs_bnd_local_dn6), (-locals.var_vbs_bnd_local_dn7), (locals.var_vbs_dn8 - locals.var_vbs_bnd_local_dn8), (locals.var_vbs_dn9 - locals.var_vbs_bnd_local_dn9), (-locals.var_vbs_bnd_local_dn10), (-locals.var_vbs_bnd_local_dn11), (-locals.var_vbs_bnd_local_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23830_e18326;
        locals.var_t1_dn0 = assign23830_e18326_d_n0;
        locals.var_t1_dn2 = assign23830_e18326_d_n2;
        locals.var_t1_dn4 = assign23830_e18326_d_n4;
        locals.var_t1_dn5 = assign23830_e18326_d_n5;
        locals.var_t1_dn6 = assign23830_e18326_d_n6;
        locals.var_t1_dn7 = assign23830_e18326_d_n7;
        locals.var_t1_dn8 = assign23830_e18326_d_n8;
        locals.var_t1_dn9 = assign23830_e18326_d_n9;
        locals.var_t1_dn10 = assign23830_e18326_d_n10;
        locals.var_t1_dn11 = assign23830_e18326_d_n11;
        locals.var_t1_dn14 = assign23830_e18326_d_n14;

        let (assign23840_e18334, assign23840_e18334_d_n0, assign23840_e18334_d_n2, assign23840_e18334_d_n4, assign23840_e18334_d_n5, assign23840_e18334_d_n6, assign23840_e18334_d_n7, assign23840_e18334_d_n8, assign23840_e18334_d_n9, assign23840_e18334_d_n10, assign23840_e18334_d_n11, assign23840_e18334_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23840_e18332: f64 = (locals.var_vbs_max_local - locals.var_vbs_bnd_local);
        (assign23840_e18332, (locals.var_vbs_max_local_dn0 - locals.var_vbs_bnd_local_dn0), (locals.var_vbs_max_local_dn2 - locals.var_vbs_bnd_local_dn2), (locals.var_vbs_max_local_dn4 - locals.var_vbs_bnd_local_dn4), (locals.var_vbs_max_local_dn5 - locals.var_vbs_bnd_local_dn5), (locals.var_vbs_max_local_dn6 - locals.var_vbs_bnd_local_dn6), (locals.var_vbs_max_local_dn7 - locals.var_vbs_bnd_local_dn7), (locals.var_vbs_max_local_dn8 - locals.var_vbs_bnd_local_dn8), (locals.var_vbs_max_local_dn9 - locals.var_vbs_bnd_local_dn9), (locals.var_vbs_max_local_dn10 - locals.var_vbs_bnd_local_dn10), (locals.var_vbs_max_local_dn11 - locals.var_vbs_bnd_local_dn11), (locals.var_vbs_max_local_dn14 - locals.var_vbs_bnd_local_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign23840_e18334;
        locals.var_t2_dn0 = assign23840_e18334_d_n0;
        locals.var_t2_dn2 = assign23840_e18334_d_n2;
        locals.var_t2_dn4 = assign23840_e18334_d_n4;
        locals.var_t2_dn5 = assign23840_e18334_d_n5;
        locals.var_t2_dn6 = assign23840_e18334_d_n6;
        locals.var_t2_dn7 = assign23840_e18334_d_n7;
        locals.var_t2_dn8 = assign23840_e18334_d_n8;
        locals.var_t2_dn9 = assign23840_e18334_d_n9;
        locals.var_t2_dn10 = assign23840_e18334_d_n10;
        locals.var_t2_dn11 = assign23840_e18334_d_n11;
        locals.var_t2_dn14 = assign23840_e18334_d_n14;

        let (assign23850_e18342, assign23850_e18342_d_n0, assign23850_e18342_d_n2, assign23850_e18342_d_n4, assign23850_e18342_d_n5, assign23850_e18342_d_n6, assign23850_e18342_d_n7, assign23850_e18342_d_n8, assign23850_e18342_d_n9, assign23850_e18342_d_n10, assign23850_e18342_d_n11, assign23850_e18342_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23850_e18340: f64 = (locals.var_t1 / locals.var_t2);
        (assign23850_e18340, (((locals.var_t1_dn0 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn2 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn4 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn5 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn6 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn7 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn8 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn9 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn10 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn11 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2)), (((locals.var_t1_dn14 * locals.var_t2) - (locals.var_t1 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2)),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign23850_e18342;
        locals.var_tmf1_dn0 = assign23850_e18342_d_n0;
        locals.var_tmf1_dn2 = assign23850_e18342_d_n2;
        locals.var_tmf1_dn4 = assign23850_e18342_d_n4;
        locals.var_tmf1_dn5 = assign23850_e18342_d_n5;
        locals.var_tmf1_dn6 = assign23850_e18342_d_n6;
        locals.var_tmf1_dn7 = assign23850_e18342_d_n7;
        locals.var_tmf1_dn8 = assign23850_e18342_d_n8;
        locals.var_tmf1_dn9 = assign23850_e18342_d_n9;
        locals.var_tmf1_dn10 = assign23850_e18342_d_n10;
        locals.var_tmf1_dn11 = assign23850_e18342_d_n11;
        locals.var_tmf1_dn14 = assign23850_e18342_d_n14;

    }

    pub(super) fn stamp_transient_block_62(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23860_e18350, assign23860_e18350_d_n0, assign23860_e18350_d_n2, assign23860_e18350_d_n4, assign23860_e18350_d_n5, assign23860_e18350_d_n6, assign23860_e18350_d_n7, assign23860_e18350_d_n8, assign23860_e18350_d_n9, assign23860_e18350_d_n10, assign23860_e18350_d_n11, assign23860_e18350_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23860_e18348: f64 = (locals.var_tmf1 * locals.var_tmf1);
        (assign23860_e18348, ((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)), ((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)), ((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)), ((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)), ((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)), ((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)), ((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)), ((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)), ((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)), ((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)), ((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign23860_e18350;
        locals.var_tmf2_dn0 = assign23860_e18350_d_n0;
        locals.var_tmf2_dn2 = assign23860_e18350_d_n2;
        locals.var_tmf2_dn4 = assign23860_e18350_d_n4;
        locals.var_tmf2_dn5 = assign23860_e18350_d_n5;
        locals.var_tmf2_dn6 = assign23860_e18350_d_n6;
        locals.var_tmf2_dn7 = assign23860_e18350_d_n7;
        locals.var_tmf2_dn8 = assign23860_e18350_d_n8;
        locals.var_tmf2_dn9 = assign23860_e18350_d_n9;
        locals.var_tmf2_dn10 = assign23860_e18350_d_n10;
        locals.var_tmf2_dn11 = assign23860_e18350_d_n11;
        locals.var_tmf2_dn14 = assign23860_e18350_d_n14;

        let (assign23870_e18358, assign23870_e18358_d_n0, assign23870_e18358_d_n2, assign23870_e18358_d_n4, assign23870_e18358_d_n5, assign23870_e18358_d_n6, assign23870_e18358_d_n7, assign23870_e18358_d_n8, assign23870_e18358_d_n9, assign23870_e18358_d_n10, assign23870_e18358_d_n11, assign23870_e18358_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23870_e18356: f64 = (locals.var_tmf2 * locals.var_tmf1);
        (assign23870_e18356, ((locals.var_tmf2_dn0 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf1) + (locals.var_tmf2 * locals.var_tmf1_dn14)),)
    } else {
        (locals.var_tmf3, locals.var_tmf3_dn0, locals.var_tmf3_dn2, locals.var_tmf3_dn4, locals.var_tmf3_dn5, locals.var_tmf3_dn6, locals.var_tmf3_dn7, locals.var_tmf3_dn8, locals.var_tmf3_dn9, locals.var_tmf3_dn10, locals.var_tmf3_dn11, locals.var_tmf3_dn14,)
    }
};
        locals.var_tmf3 = assign23870_e18358;
        locals.var_tmf3_dn0 = assign23870_e18358_d_n0;
        locals.var_tmf3_dn2 = assign23870_e18358_d_n2;
        locals.var_tmf3_dn4 = assign23870_e18358_d_n4;
        locals.var_tmf3_dn5 = assign23870_e18358_d_n5;
        locals.var_tmf3_dn6 = assign23870_e18358_d_n6;
        locals.var_tmf3_dn7 = assign23870_e18358_d_n7;
        locals.var_tmf3_dn8 = assign23870_e18358_d_n8;
        locals.var_tmf3_dn9 = assign23870_e18358_d_n9;
        locals.var_tmf3_dn10 = assign23870_e18358_d_n10;
        locals.var_tmf3_dn11 = assign23870_e18358_d_n11;
        locals.var_tmf3_dn14 = assign23870_e18358_d_n14;

        let (assign23880_e18366, assign23880_e18366_d_n0, assign23880_e18366_d_n2, assign23880_e18366_d_n4, assign23880_e18366_d_n5, assign23880_e18366_d_n6, assign23880_e18366_d_n7, assign23880_e18366_d_n8, assign23880_e18366_d_n9, assign23880_e18366_d_n10, assign23880_e18366_d_n11, assign23880_e18366_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23880_e18364: f64 = (locals.var_tmf2 * locals.var_tmf2);
        (assign23880_e18364, ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)), ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)), ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)), ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)), ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)), ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)), ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)), ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)), ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)), ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)), ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)),)
    } else {
        (locals.var_tmf4, locals.var_tmf4_dn0, locals.var_tmf4_dn2, locals.var_tmf4_dn4, locals.var_tmf4_dn5, locals.var_tmf4_dn6, locals.var_tmf4_dn7, locals.var_tmf4_dn8, locals.var_tmf4_dn9, locals.var_tmf4_dn10, locals.var_tmf4_dn11, locals.var_tmf4_dn14,)
    }
};
        locals.var_tmf4 = assign23880_e18366;
        locals.var_tmf4_dn0 = assign23880_e18366_d_n0;
        locals.var_tmf4_dn2 = assign23880_e18366_d_n2;
        locals.var_tmf4_dn4 = assign23880_e18366_d_n4;
        locals.var_tmf4_dn5 = assign23880_e18366_d_n5;
        locals.var_tmf4_dn6 = assign23880_e18366_d_n6;
        locals.var_tmf4_dn7 = assign23880_e18366_d_n7;
        locals.var_tmf4_dn8 = assign23880_e18366_d_n8;
        locals.var_tmf4_dn9 = assign23880_e18366_d_n9;
        locals.var_tmf4_dn10 = assign23880_e18366_d_n10;
        locals.var_tmf4_dn11 = assign23880_e18366_d_n11;
        locals.var_tmf4_dn14 = assign23880_e18366_d_n14;

        let (assign23890_e18382, assign23890_e18382_d_n0, assign23890_e18382_d_n2, assign23890_e18382_d_n4, assign23890_e18382_d_n5, assign23890_e18382_d_n6, assign23890_e18382_d_n7, assign23890_e18382_d_n8, assign23890_e18382_d_n9, assign23890_e18382_d_n10, assign23890_e18382_d_n11, assign23890_e18382_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23890_e18373: f64 = (1.0 + locals.var_tmf1);
        let assign23890_e18375: f64 = (assign23890_e18373 + locals.var_tmf2);
        let assign23890_e18377: f64 = (assign23890_e18375 + locals.var_tmf3);
        let assign23890_e18379: f64 = (assign23890_e18377 + locals.var_tmf4);
        let assign23890_e18380: f64 = (1.0 / assign23890_e18379);
        (assign23890_e18380, (-((((locals.var_tmf1_dn0 + locals.var_tmf2_dn0) + locals.var_tmf3_dn0) + locals.var_tmf4_dn0) / (assign23890_e18379 * assign23890_e18379))), (-((((locals.var_tmf1_dn2 + locals.var_tmf2_dn2) + locals.var_tmf3_dn2) + locals.var_tmf4_dn2) / (assign23890_e18379 * assign23890_e18379))), (-((((locals.var_tmf1_dn4 + locals.var_tmf2_dn4) + locals.var_tmf3_dn4) + locals.var_tmf4_dn4) / (assign23890_e18379 * assign23890_e18379))), (-((((locals.var_tmf1_dn5 + locals.var_tmf2_dn5) + locals.var_tmf3_dn5) + locals.var_tmf4_dn5) / (assign23890_e18379 * assign23890_e18379))), (-((((locals.var_tmf1_dn6 + locals.var_tmf2_dn6) + locals.var_tmf3_dn6) + locals.var_tmf4_dn6) / (assign23890_e18379 * assign23890_e18379))), (-((((locals.var_tmf1_dn7 + locals.var_tmf2_dn7) + locals.var_tmf3_dn7) + locals.var_tmf4_dn7) / (assign23890_e18379 * assign23890_e18379))), (-((((locals.var_tmf1_dn8 + locals.var_tmf2_dn8) + locals.var_tmf3_dn8) + locals.var_tmf4_dn8) / (assign23890_e18379 * assign23890_e18379))), (-((((locals.var_tmf1_dn9 + locals.var_tmf2_dn9) + locals.var_tmf3_dn9) + locals.var_tmf4_dn9) / (assign23890_e18379 * assign23890_e18379))), (-((((locals.var_tmf1_dn10 + locals.var_tmf2_dn10) + locals.var_tmf3_dn10) + locals.var_tmf4_dn10) / (assign23890_e18379 * assign23890_e18379))), (-((((locals.var_tmf1_dn11 + locals.var_tmf2_dn11) + locals.var_tmf3_dn11) + locals.var_tmf4_dn11) / (assign23890_e18379 * assign23890_e18379))), (-((((locals.var_tmf1_dn14 + locals.var_tmf2_dn14) + locals.var_tmf3_dn14) + locals.var_tmf4_dn14) / (assign23890_e18379 * assign23890_e18379))),)
    } else {
        (locals.var_tmf0, locals.var_tmf0_dn0, locals.var_tmf0_dn2, locals.var_tmf0_dn4, locals.var_tmf0_dn5, locals.var_tmf0_dn6, locals.var_tmf0_dn7, locals.var_tmf0_dn8, locals.var_tmf0_dn9, locals.var_tmf0_dn10, locals.var_tmf0_dn11, locals.var_tmf0_dn14,)
    }
};
        locals.var_tmf0 = assign23890_e18382;
        locals.var_tmf0_dn0 = assign23890_e18382_d_n0;
        locals.var_tmf0_dn2 = assign23890_e18382_d_n2;
        locals.var_tmf0_dn4 = assign23890_e18382_d_n4;
        locals.var_tmf0_dn5 = assign23890_e18382_d_n5;
        locals.var_tmf0_dn6 = assign23890_e18382_d_n6;
        locals.var_tmf0_dn7 = assign23890_e18382_d_n7;
        locals.var_tmf0_dn8 = assign23890_e18382_d_n8;
        locals.var_tmf0_dn9 = assign23890_e18382_d_n9;
        locals.var_tmf0_dn10 = assign23890_e18382_d_n10;
        locals.var_tmf0_dn11 = assign23890_e18382_d_n11;
        locals.var_tmf0_dn14 = assign23890_e18382_d_n14;

        let (assign23900_e18405, assign23900_e18405_d_n0, assign23900_e18405_d_n2, assign23900_e18405_d_n4, assign23900_e18405_d_n5, assign23900_e18405_d_n6, assign23900_e18405_d_n7, assign23900_e18405_d_n8, assign23900_e18405_d_n9, assign23900_e18405_d_n10, assign23900_e18405_d_n11, assign23900_e18405_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23900_e18389: f64 = (2.0 * locals.var_tmf1);
        let assign23900_e18390: f64 = (1.0 + assign23900_e18389);
        let assign23900_e18393: f64 = (3.0 * locals.var_tmf2);
        let assign23900_e18394: f64 = (assign23900_e18390 + assign23900_e18393);
        let assign23900_e18397: f64 = (4.0 * locals.var_tmf3);
        let assign23900_e18398: f64 = (assign23900_e18394 + assign23900_e18397);
        let assign23900_e18399: f64 = (-assign23900_e18398);
        let assign23900_e18401: f64 = (assign23900_e18399 * locals.var_tmf0);
        let assign23900_e18403: f64 = (assign23900_e18401 * locals.var_tmf0);
        (assign23900_e18403, (((((-(((2.0 * locals.var_tmf1_dn0) + (3.0 * locals.var_tmf2_dn0)) + (4.0 * locals.var_tmf3_dn0))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn0)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn0)), (((((-(((2.0 * locals.var_tmf1_dn2) + (3.0 * locals.var_tmf2_dn2)) + (4.0 * locals.var_tmf3_dn2))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn2)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn2)), (((((-(((2.0 * locals.var_tmf1_dn4) + (3.0 * locals.var_tmf2_dn4)) + (4.0 * locals.var_tmf3_dn4))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn4)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn4)), (((((-(((2.0 * locals.var_tmf1_dn5) + (3.0 * locals.var_tmf2_dn5)) + (4.0 * locals.var_tmf3_dn5))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn5)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn5)), (((((-(((2.0 * locals.var_tmf1_dn6) + (3.0 * locals.var_tmf2_dn6)) + (4.0 * locals.var_tmf3_dn6))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn6)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn6)), (((((-(((2.0 * locals.var_tmf1_dn7) + (3.0 * locals.var_tmf2_dn7)) + (4.0 * locals.var_tmf3_dn7))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn7)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn7)), (((((-(((2.0 * locals.var_tmf1_dn8) + (3.0 * locals.var_tmf2_dn8)) + (4.0 * locals.var_tmf3_dn8))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn8)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn8)), (((((-(((2.0 * locals.var_tmf1_dn9) + (3.0 * locals.var_tmf2_dn9)) + (4.0 * locals.var_tmf3_dn9))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn9)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn9)), (((((-(((2.0 * locals.var_tmf1_dn10) + (3.0 * locals.var_tmf2_dn10)) + (4.0 * locals.var_tmf3_dn10))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn10)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn10)), (((((-(((2.0 * locals.var_tmf1_dn11) + (3.0 * locals.var_tmf2_dn11)) + (4.0 * locals.var_tmf3_dn11))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn11)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn11)), (((((-(((2.0 * locals.var_tmf1_dn14) + (3.0 * locals.var_tmf2_dn14)) + (4.0 * locals.var_tmf3_dn14))) * locals.var_tmf0) + (assign23900_e18399 * locals.var_tmf0_dn14)) * locals.var_tmf0) + (assign23900_e18401 * locals.var_tmf0_dn14)),)
    } else {
        (locals.var_vbscldvbs__blk438, locals.var_vbscldvbs__blk438_dn0, locals.var_vbscldvbs__blk438_dn2, locals.var_vbscldvbs__blk438_dn4, locals.var_vbscldvbs__blk438_dn5, locals.var_vbscldvbs__blk438_dn6, locals.var_vbscldvbs__blk438_dn7, locals.var_vbscldvbs__blk438_dn8, locals.var_vbscldvbs__blk438_dn9, locals.var_vbscldvbs__blk438_dn10, locals.var_vbscldvbs__blk438_dn11, locals.var_vbscldvbs__blk438_dn14,)
    }
};
        locals.var_vbscldvbs__blk438 = assign23900_e18405;
        locals.var_vbscldvbs__blk438_dn0 = assign23900_e18405_d_n0;
        locals.var_vbscldvbs__blk438_dn2 = assign23900_e18405_d_n2;
        locals.var_vbscldvbs__blk438_dn4 = assign23900_e18405_d_n4;
        locals.var_vbscldvbs__blk438_dn5 = assign23900_e18405_d_n5;
        locals.var_vbscldvbs__blk438_dn6 = assign23900_e18405_d_n6;
        locals.var_vbscldvbs__blk438_dn7 = assign23900_e18405_d_n7;
        locals.var_vbscldvbs__blk438_dn8 = assign23900_e18405_d_n8;
        locals.var_vbscldvbs__blk438_dn9 = assign23900_e18405_d_n9;
        locals.var_vbscldvbs__blk438_dn10 = assign23900_e18405_d_n10;
        locals.var_vbscldvbs__blk438_dn11 = assign23900_e18405_d_n11;
        locals.var_vbscldvbs__blk438_dn14 = assign23900_e18405_d_n14;

        let (assign23910_e18415, assign23910_e18415_d_n0, assign23910_e18415_d_n2, assign23910_e18415_d_n4, assign23910_e18415_d_n5, assign23910_e18415_d_n6, assign23910_e18415_d_n7, assign23910_e18415_d_n8, assign23910_e18415_d_n9, assign23910_e18415_d_n10, assign23910_e18415_d_n11, assign23910_e18415_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23910_e18412: f64 = (1.0 - locals.var_tmf0);
        let assign23910_e18413: f64 = (locals.var_t2 * assign23910_e18412);
        (assign23910_e18413, ((locals.var_t2_dn0 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn0))), ((locals.var_t2_dn2 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn2))), ((locals.var_t2_dn4 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn4))), ((locals.var_t2_dn5 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn5))), ((locals.var_t2_dn6 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn6))), ((locals.var_t2_dn7 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn7))), ((locals.var_t2_dn8 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn8))), ((locals.var_t2_dn9 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn9))), ((locals.var_t2_dn10 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn10))), ((locals.var_t2_dn11 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn11))), ((locals.var_t2_dn14 * assign23910_e18412) + (locals.var_t2 * (-locals.var_tmf0_dn14))),)
    } else {
        (locals.var_ty, locals.var_ty_dn0, locals.var_ty_dn2, locals.var_ty_dn4, locals.var_ty_dn5, locals.var_ty_dn6, locals.var_ty_dn7, locals.var_ty_dn8, locals.var_ty_dn9, locals.var_ty_dn10, locals.var_ty_dn11, locals.var_ty_dn14,)
    }
};
        locals.var_ty = assign23910_e18415;
        locals.var_ty_dn0 = assign23910_e18415_d_n0;
        locals.var_ty_dn2 = assign23910_e18415_d_n2;
        locals.var_ty_dn4 = assign23910_e18415_d_n4;
        locals.var_ty_dn5 = assign23910_e18415_d_n5;
        locals.var_ty_dn6 = assign23910_e18415_d_n6;
        locals.var_ty_dn7 = assign23910_e18415_d_n7;
        locals.var_ty_dn8 = assign23910_e18415_d_n8;
        locals.var_ty_dn9 = assign23910_e18415_d_n9;
        locals.var_ty_dn10 = assign23910_e18415_d_n10;
        locals.var_ty_dn11 = assign23910_e18415_d_n11;
        locals.var_ty_dn14 = assign23910_e18415_d_n14;

        let (assign23920_e18427, assign23920_e18427_d_n0, assign23920_e18427_d_n2, assign23920_e18427_d_n4, assign23920_e18427_d_n5, assign23920_e18427_d_n6, assign23920_e18427_d_n7, assign23920_e18427_d_n8, assign23920_e18427_d_n9, assign23920_e18427_d_n10, assign23920_e18427_d_n11, assign23920_e18427_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23920_e18421: f64 = (1.0 - locals.var_tmf0);
        let assign23920_e18424: f64 = (locals.var_tmf1 * locals.var_vbscldvbs__blk438);
        let assign23920_e18425: f64 = (assign23920_e18421 + assign23920_e18424);
        (assign23920_e18425, ((-locals.var_tmf0_dn0) + ((locals.var_tmf1_dn0 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn0))), ((-locals.var_tmf0_dn2) + ((locals.var_tmf1_dn2 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn2))), ((-locals.var_tmf0_dn4) + ((locals.var_tmf1_dn4 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn4))), ((-locals.var_tmf0_dn5) + ((locals.var_tmf1_dn5 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn5))), ((-locals.var_tmf0_dn6) + ((locals.var_tmf1_dn6 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn6))), ((-locals.var_tmf0_dn7) + ((locals.var_tmf1_dn7 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn7))), ((-locals.var_tmf0_dn8) + ((locals.var_tmf1_dn8 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn8))), ((-locals.var_tmf0_dn9) + ((locals.var_tmf1_dn9 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn9))), ((-locals.var_tmf0_dn10) + ((locals.var_tmf1_dn10 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn10))), ((-locals.var_tmf0_dn11) + ((locals.var_tmf1_dn11 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn11))), ((-locals.var_tmf0_dn14) + ((locals.var_tmf1_dn14 * locals.var_vbscldvbs__blk438) + (locals.var_tmf1 * locals.var_vbscldvbs__blk438_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign23920_e18427;
        locals.var_t0_dn0 = assign23920_e18427_d_n0;
        locals.var_t0_dn2 = assign23920_e18427_d_n2;
        locals.var_t0_dn4 = assign23920_e18427_d_n4;
        locals.var_t0_dn5 = assign23920_e18427_d_n5;
        locals.var_t0_dn6 = assign23920_e18427_d_n6;
        locals.var_t0_dn7 = assign23920_e18427_d_n7;
        locals.var_t0_dn8 = assign23920_e18427_d_n8;
        locals.var_t0_dn9 = assign23920_e18427_d_n9;
        locals.var_t0_dn10 = assign23920_e18427_d_n10;
        locals.var_t0_dn11 = assign23920_e18427_d_n11;
        locals.var_t0_dn14 = assign23920_e18427_d_n14;

        let (assign23930_e18434, assign23930_e18434_d_n0, assign23930_e18434_d_n2, assign23930_e18434_d_n4, assign23930_e18434_d_n5, assign23930_e18434_d_n6, assign23930_e18434_d_n7, assign23930_e18434_d_n8, assign23930_e18434_d_n9, assign23930_e18434_d_n10, assign23930_e18434_d_n11, assign23930_e18434_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23930_e18432: f64 = (-locals.var_vbscldvbs__blk438);
        (assign23930_e18432, (-locals.var_vbscldvbs__blk438_dn0), (-locals.var_vbscldvbs__blk438_dn2), (-locals.var_vbscldvbs__blk438_dn4), (-locals.var_vbscldvbs__blk438_dn5), (-locals.var_vbscldvbs__blk438_dn6), (-locals.var_vbscldvbs__blk438_dn7), (-locals.var_vbscldvbs__blk438_dn8), (-locals.var_vbscldvbs__blk438_dn9), (-locals.var_vbscldvbs__blk438_dn10), (-locals.var_vbscldvbs__blk438_dn11), (-locals.var_vbscldvbs__blk438_dn14),)
    } else {
        (locals.var_vbscldvbs__blk438, locals.var_vbscldvbs__blk438_dn0, locals.var_vbscldvbs__blk438_dn2, locals.var_vbscldvbs__blk438_dn4, locals.var_vbscldvbs__blk438_dn5, locals.var_vbscldvbs__blk438_dn6, locals.var_vbscldvbs__blk438_dn7, locals.var_vbscldvbs__blk438_dn8, locals.var_vbscldvbs__blk438_dn9, locals.var_vbscldvbs__blk438_dn10, locals.var_vbscldvbs__blk438_dn11, locals.var_vbscldvbs__blk438_dn14,)
    }
};
        locals.var_vbscldvbs__blk438 = assign23930_e18434;
        locals.var_vbscldvbs__blk438_dn0 = assign23930_e18434_d_n0;
        locals.var_vbscldvbs__blk438_dn2 = assign23930_e18434_d_n2;
        locals.var_vbscldvbs__blk438_dn4 = assign23930_e18434_d_n4;
        locals.var_vbscldvbs__blk438_dn5 = assign23930_e18434_d_n5;
        locals.var_vbscldvbs__blk438_dn6 = assign23930_e18434_d_n6;
        locals.var_vbscldvbs__blk438_dn7 = assign23930_e18434_d_n7;
        locals.var_vbscldvbs__blk438_dn8 = assign23930_e18434_d_n8;
        locals.var_vbscldvbs__blk438_dn9 = assign23930_e18434_d_n9;
        locals.var_vbscldvbs__blk438_dn10 = assign23930_e18434_d_n10;
        locals.var_vbscldvbs__blk438_dn11 = assign23930_e18434_d_n11;
        locals.var_vbscldvbs__blk438_dn14 = assign23930_e18434_d_n14;

        let (assign23940_e18442, assign23940_e18442_d_n0, assign23940_e18442_d_n2, assign23940_e18442_d_n4, assign23940_e18442_d_n5, assign23940_e18442_d_n6, assign23940_e18442_d_n7, assign23940_e18442_d_n8, assign23940_e18442_d_n9, assign23940_e18442_d_n10, assign23940_e18442_d_n11, assign23940_e18442_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23940_e18440: f64 = (locals.var_vbs_bnd_local + locals.var_ty);
        (assign23940_e18440, (locals.var_vbs_bnd_local_dn0 + locals.var_ty_dn0), (locals.var_vbs_bnd_local_dn2 + locals.var_ty_dn2), (locals.var_vbs_bnd_local_dn4 + locals.var_ty_dn4), (locals.var_vbs_bnd_local_dn5 + locals.var_ty_dn5), (locals.var_vbs_bnd_local_dn6 + locals.var_ty_dn6), (locals.var_vbs_bnd_local_dn7 + locals.var_ty_dn7), (locals.var_vbs_bnd_local_dn8 + locals.var_ty_dn8), (locals.var_vbs_bnd_local_dn9 + locals.var_ty_dn9), (locals.var_vbs_bnd_local_dn10 + locals.var_ty_dn10), (locals.var_vbs_bnd_local_dn11 + locals.var_ty_dn11), (locals.var_vbs_bnd_local_dn14 + locals.var_ty_dn14),)
    } else {
        (locals.var_vbscl__blk437, locals.var_vbscl__blk437_dn0, locals.var_vbscl__blk437_dn2, locals.var_vbscl__blk437_dn4, locals.var_vbscl__blk437_dn5, locals.var_vbscl__blk437_dn6, locals.var_vbscl__blk437_dn7, locals.var_vbscl__blk437_dn8, locals.var_vbscl__blk437_dn9, locals.var_vbscl__blk437_dn10, locals.var_vbscl__blk437_dn11, locals.var_vbscl__blk437_dn14,)
    }
};
        locals.var_vbscl__blk437 = assign23940_e18442;
        locals.var_vbscl__blk437_dn0 = assign23940_e18442_d_n0;
        locals.var_vbscl__blk437_dn2 = assign23940_e18442_d_n2;
        locals.var_vbscl__blk437_dn4 = assign23940_e18442_d_n4;
        locals.var_vbscl__blk437_dn5 = assign23940_e18442_d_n5;
        locals.var_vbscl__blk437_dn6 = assign23940_e18442_d_n6;
        locals.var_vbscl__blk437_dn7 = assign23940_e18442_d_n7;
        locals.var_vbscl__blk437_dn8 = assign23940_e18442_d_n8;
        locals.var_vbscl__blk437_dn9 = assign23940_e18442_d_n9;
        locals.var_vbscl__blk437_dn10 = assign23940_e18442_d_n10;
        locals.var_vbscl__blk437_dn11 = assign23940_e18442_d_n11;
        locals.var_vbscl__blk437_dn14 = assign23940_e18442_d_n14;

        let (assign23950_e18450, assign23950_e18450_d_n0, assign23950_e18450_d_n2, assign23950_e18450_d_n4, assign23950_e18450_d_n5, assign23950_e18450_d_n6, assign23950_e18450_d_n7, assign23950_e18450_d_n8, assign23950_e18450_d_n9, assign23950_e18450_d_n10, assign23950_e18450_d_n11, assign23950_e18450_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23950_e18448: f64 = (1.0 / locals.var_t2);
        (assign23950_e18448, (-(locals.var_t2_dn0 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn2 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn4 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn5 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn6 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn7 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn8 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn9 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn10 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn11 / (locals.var_t2 * locals.var_t2))), (-(locals.var_t2_dn14 / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign23950_e18450;
        locals.var_t3_dn0 = assign23950_e18450_d_n0;
        locals.var_t3_dn2 = assign23950_e18450_d_n2;
        locals.var_t3_dn4 = assign23950_e18450_d_n4;
        locals.var_t3_dn5 = assign23950_e18450_d_n5;
        locals.var_t3_dn6 = assign23950_e18450_d_n6;
        locals.var_t3_dn7 = assign23950_e18450_d_n7;
        locals.var_t3_dn8 = assign23950_e18450_d_n8;
        locals.var_t3_dn9 = assign23950_e18450_d_n9;
        locals.var_t3_dn10 = assign23950_e18450_d_n10;
        locals.var_t3_dn11 = assign23950_e18450_d_n11;
        locals.var_t3_dn14 = assign23950_e18450_d_n14;

        let (assign23960_e18458, assign23960_e18458_d_n0, assign23960_e18458_d_n2, assign23960_e18458_d_n4, assign23960_e18458_d_n5, assign23960_e18458_d_n6, assign23960_e18458_d_n7, assign23960_e18458_d_n8, assign23960_e18458_d_n9, assign23960_e18458_d_n10, assign23960_e18458_d_n11, assign23960_e18458_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23960_e18456: f64 = (locals.var_t1 * locals.var_t3);
        (assign23960_e18456, ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)), ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)), ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)), ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)), ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)), ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)), ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)), ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)), ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)), ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)), ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign23960_e18458;
        locals.var_t4_dn0 = assign23960_e18458_d_n0;
        locals.var_t4_dn2 = assign23960_e18458_d_n2;
        locals.var_t4_dn4 = assign23960_e18458_d_n4;
        locals.var_t4_dn5 = assign23960_e18458_d_n5;
        locals.var_t4_dn6 = assign23960_e18458_d_n6;
        locals.var_t4_dn7 = assign23960_e18458_d_n7;
        locals.var_t4_dn8 = assign23960_e18458_d_n8;
        locals.var_t4_dn9 = assign23960_e18458_d_n9;
        locals.var_t4_dn10 = assign23960_e18458_d_n10;
        locals.var_t4_dn11 = assign23960_e18458_d_n11;
        locals.var_t4_dn14 = assign23960_e18458_d_n14;

        let (assign23970_e18466, assign23970_e18466_d_n0, assign23970_e18466_d_n2, assign23970_e18466_d_n4, assign23970_e18466_d_n5, assign23970_e18466_d_n6, assign23970_e18466_d_n7, assign23970_e18466_d_n8, assign23970_e18466_d_n9, assign23970_e18466_d_n10, assign23970_e18466_d_n11, assign23970_e18466_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23970_e18464: f64 = (locals.var_t4 * locals.var_t4);
        (assign23970_e18464, ((locals.var_t4_dn0 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn0)), ((locals.var_t4_dn2 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn2)), ((locals.var_t4_dn4 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn4)), ((locals.var_t4_dn5 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn5)), ((locals.var_t4_dn6 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn6)), ((locals.var_t4_dn7 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn7)), ((locals.var_t4_dn8 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn8)), ((locals.var_t4_dn9 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn9)), ((locals.var_t4_dn10 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn10)), ((locals.var_t4_dn11 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn11)), ((locals.var_t4_dn14 * locals.var_t4) + (locals.var_t4 * locals.var_t4_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign23970_e18466;
        locals.var_t5_dn0 = assign23970_e18466_d_n0;
        locals.var_t5_dn2 = assign23970_e18466_d_n2;
        locals.var_t5_dn4 = assign23970_e18466_d_n4;
        locals.var_t5_dn5 = assign23970_e18466_d_n5;
        locals.var_t5_dn6 = assign23970_e18466_d_n6;
        locals.var_t5_dn7 = assign23970_e18466_d_n7;
        locals.var_t5_dn8 = assign23970_e18466_d_n8;
        locals.var_t5_dn9 = assign23970_e18466_d_n9;
        locals.var_t5_dn10 = assign23970_e18466_d_n10;
        locals.var_t5_dn11 = assign23970_e18466_d_n11;
        locals.var_t5_dn14 = assign23970_e18466_d_n14;

        let (assign23980_e18482, assign23980_e18482_d_n0, assign23980_e18482_d_n2, assign23980_e18482_d_n4, assign23980_e18482_d_n5, assign23980_e18482_d_n6, assign23980_e18482_d_n7, assign23980_e18482_d_n8, assign23980_e18482_d_n9, assign23980_e18482_d_n10, assign23980_e18482_d_n11, assign23980_e18482_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23980_e18472: f64 = (1.0 + locals.var_t4);
        let assign23980_e18476: f64 = (1.0 + locals.var_t4);
        let assign23980_e18478: f64 = (assign23980_e18476 + locals.var_t5);
        let assign23980_e18479: f64 = (locals.var_t5 * assign23980_e18478);
        let assign23980_e18480: f64 = (assign23980_e18472 + assign23980_e18479);
        (assign23980_e18480, (locals.var_t4_dn0 + ((locals.var_t5_dn0 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn0 + locals.var_t5_dn0)))), (locals.var_t4_dn2 + ((locals.var_t5_dn2 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn2 + locals.var_t5_dn2)))), (locals.var_t4_dn4 + ((locals.var_t5_dn4 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn4 + locals.var_t5_dn4)))), (locals.var_t4_dn5 + ((locals.var_t5_dn5 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn5 + locals.var_t5_dn5)))), (locals.var_t4_dn6 + ((locals.var_t5_dn6 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn6 + locals.var_t5_dn6)))), (locals.var_t4_dn7 + ((locals.var_t5_dn7 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn7 + locals.var_t5_dn7)))), (locals.var_t4_dn8 + ((locals.var_t5_dn8 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn8 + locals.var_t5_dn8)))), (locals.var_t4_dn9 + ((locals.var_t5_dn9 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn9 + locals.var_t5_dn9)))), (locals.var_t4_dn10 + ((locals.var_t5_dn10 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn10 + locals.var_t5_dn10)))), (locals.var_t4_dn11 + ((locals.var_t5_dn11 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn11 + locals.var_t5_dn11)))), (locals.var_t4_dn14 + ((locals.var_t5_dn14 * assign23980_e18478) + (locals.var_t5 * (locals.var_t4_dn14 + locals.var_t5_dn14)))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign23980_e18482;
        locals.var_t7_dn0 = assign23980_e18482_d_n0;
        locals.var_t7_dn2 = assign23980_e18482_d_n2;
        locals.var_t7_dn4 = assign23980_e18482_d_n4;
        locals.var_t7_dn5 = assign23980_e18482_d_n5;
        locals.var_t7_dn6 = assign23980_e18482_d_n6;
        locals.var_t7_dn7 = assign23980_e18482_d_n7;
        locals.var_t7_dn8 = assign23980_e18482_d_n8;
        locals.var_t7_dn9 = assign23980_e18482_d_n9;
        locals.var_t7_dn10 = assign23980_e18482_d_n10;
        locals.var_t7_dn11 = assign23980_e18482_d_n11;
        locals.var_t7_dn14 = assign23980_e18482_d_n14;

        let (assign23990_e18506, assign23990_e18506_d_n0, assign23990_e18506_d_n2, assign23990_e18506_d_n4, assign23990_e18506_d_n5, assign23990_e18506_d_n6, assign23990_e18506_d_n7, assign23990_e18506_d_n8, assign23990_e18506_d_n9, assign23990_e18506_d_n10, assign23990_e18506_d_n11, assign23990_e18506_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 != 0.0)) {
        let assign23990_e18489: f64 = (2.0 * locals.var_t4);
        let assign23990_e18490: f64 = (1.0 + assign23990_e18489);
        let assign23990_e18493: f64 = (3.0 * locals.var_t5);
        let assign23990_e18494: f64 = (assign23990_e18490 + assign23990_e18493);
        let assign23990_e18497: f64 = (4.0 * locals.var_t4);
        let assign23990_e18499: f64 = (assign23990_e18497 * locals.var_t5);
        let assign23990_e18500: f64 = (assign23990_e18494 + assign23990_e18499);
        let assign23990_e18503: f64 = (locals.var_t7 * locals.var_t7);
        let assign23990_e18504: f64 = (assign23990_e18500 / assign23990_e18503);
        (assign23990_e18504, ((((((2.0 * locals.var_t4_dn0) + (3.0 * locals.var_t5_dn0)) + (((4.0 * locals.var_t4_dn0) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn0))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn0 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn0)))) / (assign23990_e18503 * assign23990_e18503)), ((((((2.0 * locals.var_t4_dn2) + (3.0 * locals.var_t5_dn2)) + (((4.0 * locals.var_t4_dn2) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn2))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn2 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn2)))) / (assign23990_e18503 * assign23990_e18503)), ((((((2.0 * locals.var_t4_dn4) + (3.0 * locals.var_t5_dn4)) + (((4.0 * locals.var_t4_dn4) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn4))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn4 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn4)))) / (assign23990_e18503 * assign23990_e18503)), ((((((2.0 * locals.var_t4_dn5) + (3.0 * locals.var_t5_dn5)) + (((4.0 * locals.var_t4_dn5) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn5))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn5 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn5)))) / (assign23990_e18503 * assign23990_e18503)), ((((((2.0 * locals.var_t4_dn6) + (3.0 * locals.var_t5_dn6)) + (((4.0 * locals.var_t4_dn6) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn6))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn6 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn6)))) / (assign23990_e18503 * assign23990_e18503)), ((((((2.0 * locals.var_t4_dn7) + (3.0 * locals.var_t5_dn7)) + (((4.0 * locals.var_t4_dn7) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn7))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn7 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn7)))) / (assign23990_e18503 * assign23990_e18503)), ((((((2.0 * locals.var_t4_dn8) + (3.0 * locals.var_t5_dn8)) + (((4.0 * locals.var_t4_dn8) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn8))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn8 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn8)))) / (assign23990_e18503 * assign23990_e18503)), ((((((2.0 * locals.var_t4_dn9) + (3.0 * locals.var_t5_dn9)) + (((4.0 * locals.var_t4_dn9) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn9))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn9 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn9)))) / (assign23990_e18503 * assign23990_e18503)), ((((((2.0 * locals.var_t4_dn10) + (3.0 * locals.var_t5_dn10)) + (((4.0 * locals.var_t4_dn10) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn10))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn10 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn10)))) / (assign23990_e18503 * assign23990_e18503)), ((((((2.0 * locals.var_t4_dn11) + (3.0 * locals.var_t5_dn11)) + (((4.0 * locals.var_t4_dn11) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn11))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn11 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn11)))) / (assign23990_e18503 * assign23990_e18503)), ((((((2.0 * locals.var_t4_dn14) + (3.0 * locals.var_t5_dn14)) + (((4.0 * locals.var_t4_dn14) * locals.var_t5) + (assign23990_e18497 * locals.var_t5_dn14))) * assign23990_e18503) - (assign23990_e18500 * ((locals.var_t7_dn14 * locals.var_t7) + (locals.var_t7 * locals.var_t7_dn14)))) / (assign23990_e18503 * assign23990_e18503)),)
    } else {
        (locals.var_vbscldvbs__blk438, locals.var_vbscldvbs__blk438_dn0, locals.var_vbscldvbs__blk438_dn2, locals.var_vbscldvbs__blk438_dn4, locals.var_vbscldvbs__blk438_dn5, locals.var_vbscldvbs__blk438_dn6, locals.var_vbscldvbs__blk438_dn7, locals.var_vbscldvbs__blk438_dn8, locals.var_vbscldvbs__blk438_dn9, locals.var_vbscldvbs__blk438_dn10, locals.var_vbscldvbs__blk438_dn11, locals.var_vbscldvbs__blk438_dn14,)
    }
};
        locals.var_vbscldvbs__blk438 = assign23990_e18506;
        locals.var_vbscldvbs__blk438_dn0 = assign23990_e18506_d_n0;
        locals.var_vbscldvbs__blk438_dn2 = assign23990_e18506_d_n2;
        locals.var_vbscldvbs__blk438_dn4 = assign23990_e18506_d_n4;
        locals.var_vbscldvbs__blk438_dn5 = assign23990_e18506_d_n5;
        locals.var_vbscldvbs__blk438_dn6 = assign23990_e18506_d_n6;
        locals.var_vbscldvbs__blk438_dn7 = assign23990_e18506_d_n7;
        locals.var_vbscldvbs__blk438_dn8 = assign23990_e18506_d_n8;
        locals.var_vbscldvbs__blk438_dn9 = assign23990_e18506_d_n9;
        locals.var_vbscldvbs__blk438_dn10 = assign23990_e18506_d_n10;
        locals.var_vbscldvbs__blk438_dn11 = assign23990_e18506_d_n11;
        locals.var_vbscldvbs__blk438_dn14 = assign23990_e18506_d_n14;

        let (assign24000_e18513, assign24000_e18513_d_n0, assign24000_e18513_d_n2, assign24000_e18513_d_n4, assign24000_e18513_d_n5, assign24000_e18513_d_n6, assign24000_e18513_d_n7, assign24000_e18513_d_n8, assign24000_e18513_d_n9, assign24000_e18513_d_n10, assign24000_e18513_d_n11, assign24000_e18513_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 == 0.0)) {
        (locals.var_vbs, 0.0, 0.0, 0.0, 0.0, locals.var_vbs_dn6, 0.0, locals.var_vbs_dn8, locals.var_vbs_dn9, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscl__blk437, locals.var_vbscl__blk437_dn0, locals.var_vbscl__blk437_dn2, locals.var_vbscl__blk437_dn4, locals.var_vbscl__blk437_dn5, locals.var_vbscl__blk437_dn6, locals.var_vbscl__blk437_dn7, locals.var_vbscl__blk437_dn8, locals.var_vbscl__blk437_dn9, locals.var_vbscl__blk437_dn10, locals.var_vbscl__blk437_dn11, locals.var_vbscl__blk437_dn14,)
    }
};
        locals.var_vbscl__blk437 = assign24000_e18513;
        locals.var_vbscl__blk437_dn0 = assign24000_e18513_d_n0;
        locals.var_vbscl__blk437_dn2 = assign24000_e18513_d_n2;
        locals.var_vbscl__blk437_dn4 = assign24000_e18513_d_n4;
        locals.var_vbscl__blk437_dn5 = assign24000_e18513_d_n5;
        locals.var_vbscl__blk437_dn6 = assign24000_e18513_d_n6;
        locals.var_vbscl__blk437_dn7 = assign24000_e18513_d_n7;
        locals.var_vbscl__blk437_dn8 = assign24000_e18513_d_n8;
        locals.var_vbscl__blk437_dn9 = assign24000_e18513_d_n9;
        locals.var_vbscl__blk437_dn10 = assign24000_e18513_d_n10;
        locals.var_vbscl__blk437_dn11 = assign24000_e18513_d_n11;
        locals.var_vbscl__blk437_dn14 = assign24000_e18513_d_n14;

        let (assign24010_e18520, assign24010_e18520_d_n0, assign24010_e18520_d_n2, assign24010_e18520_d_n4, assign24010_e18520_d_n5, assign24010_e18520_d_n6, assign24010_e18520_d_n7, assign24010_e18520_d_n8, assign24010_e18520_d_n9, assign24010_e18520_d_n10, assign24010_e18520_d_n11, assign24010_e18520_d_n14,) = {
    if ((p.p37 != 0.0) && (locals.var_guard443 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscldvbs__blk438, locals.var_vbscldvbs__blk438_dn0, locals.var_vbscldvbs__blk438_dn2, locals.var_vbscldvbs__blk438_dn4, locals.var_vbscldvbs__blk438_dn5, locals.var_vbscldvbs__blk438_dn6, locals.var_vbscldvbs__blk438_dn7, locals.var_vbscldvbs__blk438_dn8, locals.var_vbscldvbs__blk438_dn9, locals.var_vbscldvbs__blk438_dn10, locals.var_vbscldvbs__blk438_dn11, locals.var_vbscldvbs__blk438_dn14,)
    }
};
        locals.var_vbscldvbs__blk438 = assign24010_e18520;
        locals.var_vbscldvbs__blk438_dn0 = assign24010_e18520_d_n0;
        locals.var_vbscldvbs__blk438_dn2 = assign24010_e18520_d_n2;
        locals.var_vbscldvbs__blk438_dn4 = assign24010_e18520_d_n4;
        locals.var_vbscldvbs__blk438_dn5 = assign24010_e18520_d_n5;
        locals.var_vbscldvbs__blk438_dn6 = assign24010_e18520_d_n6;
        locals.var_vbscldvbs__blk438_dn7 = assign24010_e18520_d_n7;
        locals.var_vbscldvbs__blk438_dn8 = assign24010_e18520_d_n8;
        locals.var_vbscldvbs__blk438_dn9 = assign24010_e18520_d_n9;
        locals.var_vbscldvbs__blk438_dn10 = assign24010_e18520_d_n10;
        locals.var_vbscldvbs__blk438_dn11 = assign24010_e18520_d_n11;
        locals.var_vbscldvbs__blk438_dn14 = assign24010_e18520_d_n14;

        let (assign24020_e18525, assign24020_e18525_d_n0, assign24020_e18525_d_n2, assign24020_e18525_d_n4, assign24020_e18525_d_n5, assign24020_e18525_d_n6, assign24020_e18525_d_n7, assign24020_e18525_d_n8, assign24020_e18525_d_n9, assign24020_e18525_d_n10, assign24020_e18525_d_n11, assign24020_e18525_d_n14,) = {
    if (p.p37 == 0.0) {
        (locals.var_vbs, 0.0, 0.0, 0.0, 0.0, locals.var_vbs_dn6, 0.0, locals.var_vbs_dn8, locals.var_vbs_dn9, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscl__blk437, locals.var_vbscl__blk437_dn0, locals.var_vbscl__blk437_dn2, locals.var_vbscl__blk437_dn4, locals.var_vbscl__blk437_dn5, locals.var_vbscl__blk437_dn6, locals.var_vbscl__blk437_dn7, locals.var_vbscl__blk437_dn8, locals.var_vbscl__blk437_dn9, locals.var_vbscl__blk437_dn10, locals.var_vbscl__blk437_dn11, locals.var_vbscl__blk437_dn14,)
    }
};
        locals.var_vbscl__blk437 = assign24020_e18525;
        locals.var_vbscl__blk437_dn0 = assign24020_e18525_d_n0;
        locals.var_vbscl__blk437_dn2 = assign24020_e18525_d_n2;
        locals.var_vbscl__blk437_dn4 = assign24020_e18525_d_n4;
        locals.var_vbscl__blk437_dn5 = assign24020_e18525_d_n5;
        locals.var_vbscl__blk437_dn6 = assign24020_e18525_d_n6;
        locals.var_vbscl__blk437_dn7 = assign24020_e18525_d_n7;
        locals.var_vbscl__blk437_dn8 = assign24020_e18525_d_n8;
        locals.var_vbscl__blk437_dn9 = assign24020_e18525_d_n9;
        locals.var_vbscl__blk437_dn10 = assign24020_e18525_d_n10;
        locals.var_vbscl__blk437_dn11 = assign24020_e18525_d_n11;
        locals.var_vbscl__blk437_dn14 = assign24020_e18525_d_n14;

        let (assign24030_e18530, assign24030_e18530_d_n0, assign24030_e18530_d_n2, assign24030_e18530_d_n4, assign24030_e18530_d_n5, assign24030_e18530_d_n6, assign24030_e18530_d_n7, assign24030_e18530_d_n8, assign24030_e18530_d_n9, assign24030_e18530_d_n10, assign24030_e18530_d_n11, assign24030_e18530_d_n14,) = {
    if (p.p37 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vbscldvbs__blk438, locals.var_vbscldvbs__blk438_dn0, locals.var_vbscldvbs__blk438_dn2, locals.var_vbscldvbs__blk438_dn4, locals.var_vbscldvbs__blk438_dn5, locals.var_vbscldvbs__blk438_dn6, locals.var_vbscldvbs__blk438_dn7, locals.var_vbscldvbs__blk438_dn8, locals.var_vbscldvbs__blk438_dn9, locals.var_vbscldvbs__blk438_dn10, locals.var_vbscldvbs__blk438_dn11, locals.var_vbscldvbs__blk438_dn14,)
    }
};
        locals.var_vbscldvbs__blk438 = assign24030_e18530;
        locals.var_vbscldvbs__blk438_dn0 = assign24030_e18530_d_n0;
        locals.var_vbscldvbs__blk438_dn2 = assign24030_e18530_d_n2;
        locals.var_vbscldvbs__blk438_dn4 = assign24030_e18530_d_n4;
        locals.var_vbscldvbs__blk438_dn5 = assign24030_e18530_d_n5;
        locals.var_vbscldvbs__blk438_dn6 = assign24030_e18530_d_n6;
        locals.var_vbscldvbs__blk438_dn7 = assign24030_e18530_d_n7;
        locals.var_vbscldvbs__blk438_dn8 = assign24030_e18530_d_n8;
        locals.var_vbscldvbs__blk438_dn9 = assign24030_e18530_d_n9;
        locals.var_vbscldvbs__blk438_dn10 = assign24030_e18530_d_n10;
        locals.var_vbscldvbs__blk438_dn11 = assign24030_e18530_d_n11;
        locals.var_vbscldvbs__blk438_dn14 = assign24030_e18530_d_n14;

        let assign24040_e18533: f64 = (locals.var_vbscldvbs__blk438 * locals.var_vds);
        let assign24040_e18535: f64 = (assign24040_e18533 / 2.0);
        locals.var_t1 = assign24040_e18535;
        locals.var_t1_dn0 = (((locals.var_vbscldvbs__blk438_dn0 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn0)) / 2.0);
        locals.var_t1_dn2 = (((locals.var_vbscldvbs__blk438_dn2 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn2)) / 2.0);
        locals.var_t1_dn4 = (((locals.var_vbscldvbs__blk438_dn4 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn4)) / 2.0);
        locals.var_t1_dn5 = (((locals.var_vbscldvbs__blk438_dn5 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn5)) / 2.0);
        locals.var_t1_dn6 = (((locals.var_vbscldvbs__blk438_dn6 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn6)) / 2.0);
        locals.var_t1_dn7 = (((locals.var_vbscldvbs__blk438_dn7 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn7)) / 2.0);
        locals.var_t1_dn8 = (((locals.var_vbscldvbs__blk438_dn8 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn8)) / 2.0);
        locals.var_t1_dn9 = (((locals.var_vbscldvbs__blk438_dn9 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn9)) / 2.0);
        locals.var_t1_dn10 = (((locals.var_vbscldvbs__blk438_dn10 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn10)) / 2.0);
        locals.var_t1_dn11 = (((locals.var_vbscldvbs__blk438_dn11 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn11)) / 2.0);
        locals.var_t1_dn14 = (((locals.var_vbscldvbs__blk438_dn14 * locals.var_vds) + (locals.var_vbscldvbs__blk438 * locals.var_vds_dn14)) / 2.0);

        let assign24050_e18538: f64 = (2.0 * locals.var_t1);
        let assign24050_e18540: f64 = (assign24050_e18538 / p.p262);
        locals.var_tmf1 = assign24050_e18540;
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

        let assign24060_e18545: f64 = (1.0 / 2.0);
        let assign24060_e18549: f64 = (1.0 / 6.0);
        let assign24060_e18553: f64 = (1.0 / 24.0);
        let assign24060_e18557: f64 = (1.0 / 120.0);
        let assign24060_e18561: f64 = (1.0 / 720.0);
        let assign24060_e18565: f64 = (1.0 / 5040.0);
        let assign24060_e18566: f64 = (locals.var_tmf1 * assign24060_e18565);
        let assign24060_e18567: f64 = (assign24060_e18561 + assign24060_e18566);
        let assign24060_e18568: f64 = (locals.var_tmf1 * assign24060_e18567);
        let assign24060_e18569: f64 = (assign24060_e18557 + assign24060_e18568);
        let assign24060_e18570: f64 = (locals.var_tmf1 * assign24060_e18569);
        let assign24060_e18571: f64 = (assign24060_e18553 + assign24060_e18570);
        let assign24060_e18572: f64 = (locals.var_tmf1 * assign24060_e18571);
        let assign24060_e18573: f64 = (assign24060_e18549 + assign24060_e18572);
        let assign24060_e18574: f64 = (locals.var_tmf1 * assign24060_e18573);
        let assign24060_e18575: f64 = (assign24060_e18545 + assign24060_e18574);
        let assign24060_e18576: f64 = (locals.var_tmf1 * assign24060_e18575);
        let assign24060_e18577: f64 = (1.0 + assign24060_e18576);
        locals.var_tmf2 = assign24060_e18577;
        locals.var_tmf2_dn0 = ((locals.var_tmf1_dn0 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign24060_e18565)))))))))));
        locals.var_tmf2_dn2 = ((locals.var_tmf1_dn2 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign24060_e18565)))))))))));
        locals.var_tmf2_dn4 = ((locals.var_tmf1_dn4 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign24060_e18565)))))))))));
        locals.var_tmf2_dn5 = ((locals.var_tmf1_dn5 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign24060_e18565)))))))))));
        locals.var_tmf2_dn6 = ((locals.var_tmf1_dn6 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign24060_e18565)))))))))));
        locals.var_tmf2_dn7 = ((locals.var_tmf1_dn7 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign24060_e18565)))))))))));
        locals.var_tmf2_dn8 = ((locals.var_tmf1_dn8 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign24060_e18565)))))))))));
        locals.var_tmf2_dn9 = ((locals.var_tmf1_dn9 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign24060_e18565)))))))))));
        locals.var_tmf2_dn10 = ((locals.var_tmf1_dn10 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign24060_e18565)))))))))));
        locals.var_tmf2_dn11 = ((locals.var_tmf1_dn11 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign24060_e18565)))))))))));
        locals.var_tmf2_dn14 = ((locals.var_tmf1_dn14 * assign24060_e18575) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign24060_e18573) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign24060_e18571) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign24060_e18569) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign24060_e18567) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign24060_e18565)))))))))));

        let assign24070_e18580: f64 = (1.0 / 2.0);
        let assign24070_e18584: f64 = (1.0 / 3.0);
        let assign24070_e18588: f64 = (1.0 / 8.0);
        let assign24070_e18592: f64 = (1.0 / 30.0);
        let assign24070_e18596: f64 = (1.0 / 144.0);
        let assign24070_e18600: f64 = (1.0 / 840.0);
        let assign24070_e18601: f64 = (locals.var_tmf1 * assign24070_e18600);
        let assign24070_e18602: f64 = (assign24070_e18596 + assign24070_e18601);
        let assign24070_e18603: f64 = (locals.var_tmf1 * assign24070_e18602);
        let assign24070_e18604: f64 = (assign24070_e18592 + assign24070_e18603);
        let assign24070_e18605: f64 = (locals.var_tmf1 * assign24070_e18604);
        let assign24070_e18606: f64 = (assign24070_e18588 + assign24070_e18605);
        let assign24070_e18607: f64 = (locals.var_tmf1 * assign24070_e18606);
        let assign24070_e18608: f64 = (assign24070_e18584 + assign24070_e18607);
        let assign24070_e18609: f64 = (locals.var_tmf1 * assign24070_e18608);
        let assign24070_e18610: f64 = (assign24070_e18580 + assign24070_e18609);
        locals.var_tmf3 = assign24070_e18610;
        locals.var_tmf3_dn0 = ((locals.var_tmf1_dn0 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn0 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn0 * assign24070_e18600)))))))));
        locals.var_tmf3_dn2 = ((locals.var_tmf1_dn2 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn2 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn2 * assign24070_e18600)))))))));
        locals.var_tmf3_dn4 = ((locals.var_tmf1_dn4 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn4 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn4 * assign24070_e18600)))))))));
        locals.var_tmf3_dn5 = ((locals.var_tmf1_dn5 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn5 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn5 * assign24070_e18600)))))))));
        locals.var_tmf3_dn6 = ((locals.var_tmf1_dn6 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn6 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn6 * assign24070_e18600)))))))));
        locals.var_tmf3_dn7 = ((locals.var_tmf1_dn7 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn7 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn7 * assign24070_e18600)))))))));
        locals.var_tmf3_dn8 = ((locals.var_tmf1_dn8 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn8 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn8 * assign24070_e18600)))))))));
        locals.var_tmf3_dn9 = ((locals.var_tmf1_dn9 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn9 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn9 * assign24070_e18600)))))))));
        locals.var_tmf3_dn10 = ((locals.var_tmf1_dn10 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn10 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn10 * assign24070_e18600)))))))));
        locals.var_tmf3_dn11 = ((locals.var_tmf1_dn11 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn11 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn11 * assign24070_e18600)))))))));
        locals.var_tmf3_dn14 = ((locals.var_tmf1_dn14 * assign24070_e18608) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign24070_e18606) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign24070_e18604) + (locals.var_tmf1 * ((locals.var_tmf1_dn14 * assign24070_e18602) + (locals.var_tmf1 * (locals.var_tmf1_dn14 * assign24070_e18600)))))))));

        let assign24080_e18613: f64 = (p.p262 / locals.var_tmf2);
        locals.var_vzadd__blk439 = assign24080_e18613;
        locals.var_vzadd__blk439_dn0 = (-((p.p262 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk439_dn2 = (-((p.p262 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk439_dn4 = (-((p.p262 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk439_dn5 = (-((p.p262 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk439_dn6 = (-((p.p262 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk439_dn7 = (-((p.p262 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk439_dn8 = (-((p.p262 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk439_dn9 = (-((p.p262 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk439_dn10 = (-((p.p262 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk439_dn11 = (-((p.p262 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_vzadd__blk439_dn14 = (-((p.p262 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)));

    }

    pub(super) fn stamp_transient_block_63(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign24090_e18615: f64 = (-2.0);
        let assign24090_e18617: f64 = (assign24090_e18615 * locals.var_tmf3);
        let assign24090_e18620: f64 = (locals.var_tmf2 * locals.var_tmf2);
        let assign24090_e18621: f64 = (assign24090_e18617 / assign24090_e18620);
        locals.var_t2 = assign24090_e18621;
        locals.var_t2_dn0 = ((((assign24090_e18615 * locals.var_tmf3_dn0) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn0 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn0)))) / (assign24090_e18620 * assign24090_e18620));
        locals.var_t2_dn2 = ((((assign24090_e18615 * locals.var_tmf3_dn2) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn2 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn2)))) / (assign24090_e18620 * assign24090_e18620));
        locals.var_t2_dn4 = ((((assign24090_e18615 * locals.var_tmf3_dn4) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn4 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn4)))) / (assign24090_e18620 * assign24090_e18620));
        locals.var_t2_dn5 = ((((assign24090_e18615 * locals.var_tmf3_dn5) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn5 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn5)))) / (assign24090_e18620 * assign24090_e18620));
        locals.var_t2_dn6 = ((((assign24090_e18615 * locals.var_tmf3_dn6) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn6 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn6)))) / (assign24090_e18620 * assign24090_e18620));
        locals.var_t2_dn7 = ((((assign24090_e18615 * locals.var_tmf3_dn7) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn7 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn7)))) / (assign24090_e18620 * assign24090_e18620));
        locals.var_t2_dn8 = ((((assign24090_e18615 * locals.var_tmf3_dn8) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn8 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn8)))) / (assign24090_e18620 * assign24090_e18620));
        locals.var_t2_dn9 = ((((assign24090_e18615 * locals.var_tmf3_dn9) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn9 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn9)))) / (assign24090_e18620 * assign24090_e18620));
        locals.var_t2_dn10 = ((((assign24090_e18615 * locals.var_tmf3_dn10) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn10 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn10)))) / (assign24090_e18620 * assign24090_e18620));
        locals.var_t2_dn11 = ((((assign24090_e18615 * locals.var_tmf3_dn11) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn11 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn11)))) / (assign24090_e18620 * assign24090_e18620));
        locals.var_t2_dn14 = ((((assign24090_e18615 * locals.var_tmf3_dn14) * assign24090_e18620) - (assign24090_e18617 * ((locals.var_tmf2_dn14 * locals.var_tmf2) + (locals.var_tmf2 * locals.var_tmf2_dn14)))) / (assign24090_e18620 * assign24090_e18620));

        let assign24100_e18624: f64 = if locals.var_vzadd__blk439 < 1e-12 { 1.0 } else { 0.0 };
        locals.var_guard444 = assign24100_e18624;

        let (assign24110_e18628, assign24110_e18628_d_n0, assign24110_e18628_d_n2, assign24110_e18628_d_n4, assign24110_e18628_d_n5, assign24110_e18628_d_n6, assign24110_e18628_d_n7, assign24110_e18628_d_n8, assign24110_e18628_d_n9, assign24110_e18628_d_n10, assign24110_e18628_d_n11, assign24110_e18628_d_n14,) = {
    if (locals.var_guard444 != 0.0) {
        (1e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vzadd__blk439, locals.var_vzadd__blk439_dn0, locals.var_vzadd__blk439_dn2, locals.var_vzadd__blk439_dn4, locals.var_vzadd__blk439_dn5, locals.var_vzadd__blk439_dn6, locals.var_vzadd__blk439_dn7, locals.var_vzadd__blk439_dn8, locals.var_vzadd__blk439_dn9, locals.var_vzadd__blk439_dn10, locals.var_vzadd__blk439_dn11, locals.var_vzadd__blk439_dn14,)
    }
};
        locals.var_vzadd__blk439 = assign24110_e18628;
        locals.var_vzadd__blk439_dn0 = assign24110_e18628_d_n0;
        locals.var_vzadd__blk439_dn2 = assign24110_e18628_d_n2;
        locals.var_vzadd__blk439_dn4 = assign24110_e18628_d_n4;
        locals.var_vzadd__blk439_dn5 = assign24110_e18628_d_n5;
        locals.var_vzadd__blk439_dn6 = assign24110_e18628_d_n6;
        locals.var_vzadd__blk439_dn7 = assign24110_e18628_d_n7;
        locals.var_vzadd__blk439_dn8 = assign24110_e18628_d_n8;
        locals.var_vzadd__blk439_dn9 = assign24110_e18628_d_n9;
        locals.var_vzadd__blk439_dn10 = assign24110_e18628_d_n10;
        locals.var_vzadd__blk439_dn11 = assign24110_e18628_d_n11;
        locals.var_vzadd__blk439_dn14 = assign24110_e18628_d_n14;

        let assign24120_e18631: f64 = (locals.var_vbscl__blk437 + locals.var_vzadd__blk439);
        locals.var_vbsz__blk440 = assign24120_e18631;
        locals.var_vbsz__blk440_dn0 = (locals.var_vbscl__blk437_dn0 + locals.var_vzadd__blk439_dn0);
        locals.var_vbsz__blk440_dn2 = (locals.var_vbscl__blk437_dn2 + locals.var_vzadd__blk439_dn2);
        locals.var_vbsz__blk440_dn4 = (locals.var_vbscl__blk437_dn4 + locals.var_vzadd__blk439_dn4);
        locals.var_vbsz__blk440_dn5 = (locals.var_vbscl__blk437_dn5 + locals.var_vzadd__blk439_dn5);
        locals.var_vbsz__blk440_dn6 = (locals.var_vbscl__blk437_dn6 + locals.var_vzadd__blk439_dn6);
        locals.var_vbsz__blk440_dn7 = (locals.var_vbscl__blk437_dn7 + locals.var_vzadd__blk439_dn7);
        locals.var_vbsz__blk440_dn8 = (locals.var_vbscl__blk437_dn8 + locals.var_vzadd__blk439_dn8);
        locals.var_vbsz__blk440_dn9 = (locals.var_vbscl__blk437_dn9 + locals.var_vzadd__blk439_dn9);
        locals.var_vbsz__blk440_dn10 = (locals.var_vbscl__blk437_dn10 + locals.var_vzadd__blk439_dn10);
        locals.var_vbsz__blk440_dn11 = (locals.var_vbscl__blk437_dn11 + locals.var_vzadd__blk439_dn11);
        locals.var_vbsz__blk440_dn14 = (locals.var_vbscl__blk437_dn14 + locals.var_vzadd__blk439_dn14);

        let assign24130_e18635: f64 = (2.0 * locals.var_vzadd__blk439);
        let assign24130_e18636: f64 = (locals.var_vds + assign24130_e18635);
        locals.var_vdsz__blk441 = assign24130_e18636;
        locals.var_vdsz__blk441_dn0 = (locals.var_vds_dn0 + (2.0 * locals.var_vzadd__blk439_dn0));
        locals.var_vdsz__blk441_dn2 = (locals.var_vds_dn2 + (2.0 * locals.var_vzadd__blk439_dn2));
        locals.var_vdsz__blk441_dn4 = (locals.var_vds_dn4 + (2.0 * locals.var_vzadd__blk439_dn4));
        locals.var_vdsz__blk441_dn5 = (locals.var_vds_dn5 + (2.0 * locals.var_vzadd__blk439_dn5));
        locals.var_vdsz__blk441_dn6 = (locals.var_vds_dn6 + (2.0 * locals.var_vzadd__blk439_dn6));
        locals.var_vdsz__blk441_dn7 = (locals.var_vds_dn7 + (2.0 * locals.var_vzadd__blk439_dn7));
        locals.var_vdsz__blk441_dn8 = (locals.var_vds_dn8 + (2.0 * locals.var_vzadd__blk439_dn8));
        locals.var_vdsz__blk441_dn9 = (locals.var_vds_dn9 + (2.0 * locals.var_vzadd__blk439_dn9));
        locals.var_vdsz__blk441_dn10 = (locals.var_vds_dn10 + (2.0 * locals.var_vzadd__blk439_dn10));
        locals.var_vdsz__blk441_dn11 = (locals.var_vds_dn11 + (2.0 * locals.var_vzadd__blk439_dn11));
        locals.var_vdsz__blk441_dn14 = (locals.var_vds_dn14 + (2.0 * locals.var_vzadd__blk439_dn14));

        let assign24140_e18639: f64 = (locals.var_vgs + locals.var_vzadd__blk439);
        locals.var_vgsz__blk442 = assign24140_e18639;
        locals.var_vgsz__blk442_dn0 = locals.var_vzadd__blk439_dn0;
        locals.var_vgsz__blk442_dn2 = locals.var_vzadd__blk439_dn2;
        locals.var_vgsz__blk442_dn4 = locals.var_vzadd__blk439_dn4;
        locals.var_vgsz__blk442_dn5 = locals.var_vzadd__blk439_dn5;
        locals.var_vgsz__blk442_dn6 = (locals.var_vgs_dn6 + locals.var_vzadd__blk439_dn6);
        locals.var_vgsz__blk442_dn7 = (locals.var_vgs_dn7 + locals.var_vzadd__blk439_dn7);
        locals.var_vgsz__blk442_dn8 = (locals.var_vgs_dn8 + locals.var_vzadd__blk439_dn8);
        locals.var_vgsz__blk442_dn9 = locals.var_vzadd__blk439_dn9;
        locals.var_vgsz__blk442_dn10 = locals.var_vzadd__blk439_dn10;
        locals.var_vgsz__blk442_dn11 = locals.var_vzadd__blk439_dn11;
        locals.var_vgsz__blk442_dn14 = locals.var_vzadd__blk439_dn14;

        let assign24150_e18642: f64 = (locals.var_vgs - locals.var_vfb);
        let assign24150_e18644: f64 = (assign24150_e18642 + locals.var_dvth);
        let assign24150_e18646: f64 = (assign24150_e18644 - locals.var_dppg);
        locals.var_vgp = assign24150_e18646;
        locals.var_vgp_dn0 = (locals.var_dvth_dn0 - locals.var_dppg_dn0);
        locals.var_vgp_dn2 = (locals.var_dvth_dn2 - locals.var_dppg_dn2);
        locals.var_vgp_dn4 = (locals.var_dvth_dn4 - locals.var_dppg_dn4);
        locals.var_vgp_dn5 = (locals.var_dvth_dn5 - locals.var_dppg_dn5);
        locals.var_vgp_dn6 = ((locals.var_vgs_dn6 + locals.var_dvth_dn6) - locals.var_dppg_dn6);
        locals.var_vgp_dn7 = ((locals.var_vgs_dn7 + locals.var_dvth_dn7) - locals.var_dppg_dn7);
        locals.var_vgp_dn8 = ((locals.var_vgs_dn8 + locals.var_dvth_dn8) - locals.var_dppg_dn8);
        locals.var_vgp_dn9 = (locals.var_dvth_dn9 - locals.var_dppg_dn9);
        locals.var_vgp_dn10 = (locals.var_dvth_dn10 - locals.var_dppg_dn10);
        locals.var_vgp_dn11 = (locals.var_dvth_dn11 - locals.var_dppg_dn11);
        locals.var_vgp_dn14 = (locals.var_dvth_dn14 - locals.var_dppg_dn14);

        let assign24160_e18649: f64 = (locals.var_vfb - locals.var_dvth);
        let assign24160_e18651: f64 = (assign24160_e18649 + locals.var_dppg);
        let assign24160_e18653: f64 = (assign24160_e18651 + locals.var_vbscl__blk437);
        locals.var_vgs_fb = assign24160_e18653;

        let assign24170_e18656: f64 = if locals.var_uc_codep != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard445 = assign24170_e18656;

        let assign24180_e18659: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard446 = assign24180_e18659;

        let assign24190_e18662: f64 = if p.p42 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard447 = assign24190_e18662;

        let assign24200_e18665: f64 = if p.p42 == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard448 = assign24200_e18665;

        let (assign24210_e18671, assign24210_e18671_d_n0, assign24210_e18671_d_n2, assign24210_e18671_d_n4, assign24210_e18671_d_n5, assign24210_e18671_d_n6, assign24210_e18671_d_n7, assign24210_e18671_d_n8, assign24210_e18671_d_n9, assign24210_e18671_d_n10, assign24210_e18671_d_n11, assign24210_e18671_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn14,)
    } else {
        (locals.var_vbi_dep, locals.var_vbi_dep_dn0, locals.var_vbi_dep_dn2, locals.var_vbi_dep_dn4, locals.var_vbi_dep_dn5, locals.var_vbi_dep_dn6, locals.var_vbi_dep_dn7, locals.var_vbi_dep_dn8, locals.var_vbi_dep_dn9, locals.var_vbi_dep_dn10, locals.var_vbi_dep_dn11, locals.var_vbi_dep_dn14,)
    }
};
        locals.var_vbi_dep = assign24210_e18671;
        locals.var_vbi_dep_dn0 = assign24210_e18671_d_n0;
        locals.var_vbi_dep_dn2 = assign24210_e18671_d_n2;
        locals.var_vbi_dep_dn4 = assign24210_e18671_d_n4;
        locals.var_vbi_dep_dn5 = assign24210_e18671_d_n5;
        locals.var_vbi_dep_dn6 = assign24210_e18671_d_n6;
        locals.var_vbi_dep_dn7 = assign24210_e18671_d_n7;
        locals.var_vbi_dep_dn8 = assign24210_e18671_d_n8;
        locals.var_vbi_dep_dn9 = assign24210_e18671_d_n9;
        locals.var_vbi_dep_dn10 = assign24210_e18671_d_n10;
        locals.var_vbi_dep_dn11 = assign24210_e18671_d_n11;
        locals.var_vbi_dep_dn14 = assign24210_e18671_d_n14;

        let (assign24220_e18679, assign24220_e18679_d_n0, assign24220_e18679_d_n2, assign24220_e18679_d_n4, assign24220_e18679_d_n5, assign24220_e18679_d_n6, assign24220_e18679_d_n7, assign24220_e18679_d_n8, assign24220_e18679_d_n9, assign24220_e18679_d_n10, assign24220_e18679_d_n11, assign24220_e18679_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24220_e18677: f64 = (1.6021918e-19 * locals.var_uc_ndepm);
        (assign24220_e18677, (1.6021918e-19 * locals.var_uc_ndepm_dn0), (1.6021918e-19 * locals.var_uc_ndepm_dn2), (1.6021918e-19 * locals.var_uc_ndepm_dn4), (1.6021918e-19 * locals.var_uc_ndepm_dn5), (1.6021918e-19 * locals.var_uc_ndepm_dn6), (1.6021918e-19 * locals.var_uc_ndepm_dn7), (1.6021918e-19 * locals.var_uc_ndepm_dn8), (1.6021918e-19 * locals.var_uc_ndepm_dn9), (1.6021918e-19 * locals.var_uc_ndepm_dn10), (1.6021918e-19 * locals.var_uc_ndepm_dn11), (1.6021918e-19 * locals.var_uc_ndepm_dn14),)
    } else {
        (locals.var_q_ndepm, locals.var_q_ndepm_dn0, locals.var_q_ndepm_dn2, locals.var_q_ndepm_dn4, locals.var_q_ndepm_dn5, locals.var_q_ndepm_dn6, locals.var_q_ndepm_dn7, locals.var_q_ndepm_dn8, locals.var_q_ndepm_dn9, locals.var_q_ndepm_dn10, locals.var_q_ndepm_dn11, locals.var_q_ndepm_dn14,)
    }
};
        locals.var_q_ndepm = assign24220_e18679;
        locals.var_q_ndepm_dn0 = assign24220_e18679_d_n0;
        locals.var_q_ndepm_dn2 = assign24220_e18679_d_n2;
        locals.var_q_ndepm_dn4 = assign24220_e18679_d_n4;
        locals.var_q_ndepm_dn5 = assign24220_e18679_d_n5;
        locals.var_q_ndepm_dn6 = assign24220_e18679_d_n6;
        locals.var_q_ndepm_dn7 = assign24220_e18679_d_n7;
        locals.var_q_ndepm_dn8 = assign24220_e18679_d_n8;
        locals.var_q_ndepm_dn9 = assign24220_e18679_d_n9;
        locals.var_q_ndepm_dn10 = assign24220_e18679_d_n10;
        locals.var_q_ndepm_dn11 = assign24220_e18679_d_n11;
        locals.var_q_ndepm_dn14 = assign24220_e18679_d_n14;

        let (assign24230_e18687, assign24230_e18687_d_n0, assign24230_e18687_d_n2, assign24230_e18687_d_n4, assign24230_e18687_d_n5, assign24230_e18687_d_n6, assign24230_e18687_d_n7, assign24230_e18687_d_n8, assign24230_e18687_d_n9, assign24230_e18687_d_n10, assign24230_e18687_d_n11, assign24230_e18687_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24230_e18685: f64 = (locals.var_uc_ndepm * locals.var_uc_ndepm);
        (assign24230_e18685, ((locals.var_uc_ndepm_dn0 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn0)), ((locals.var_uc_ndepm_dn2 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn2)), ((locals.var_uc_ndepm_dn4 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn4)), ((locals.var_uc_ndepm_dn5 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn5)), ((locals.var_uc_ndepm_dn6 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn6)), ((locals.var_uc_ndepm_dn7 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn7)), ((locals.var_uc_ndepm_dn8 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn8)), ((locals.var_uc_ndepm_dn9 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn9)), ((locals.var_uc_ndepm_dn10 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn10)), ((locals.var_uc_ndepm_dn11 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn11)), ((locals.var_uc_ndepm_dn14 * locals.var_uc_ndepm) + (locals.var_uc_ndepm * locals.var_uc_ndepm_dn14)),)
    } else {
        (locals.var_ndepm2, locals.var_ndepm2_dn0, locals.var_ndepm2_dn2, locals.var_ndepm2_dn4, locals.var_ndepm2_dn5, locals.var_ndepm2_dn6, locals.var_ndepm2_dn7, locals.var_ndepm2_dn8, locals.var_ndepm2_dn9, locals.var_ndepm2_dn10, locals.var_ndepm2_dn11, locals.var_ndepm2_dn14,)
    }
};
        locals.var_ndepm2 = assign24230_e18687;
        locals.var_ndepm2_dn0 = assign24230_e18687_d_n0;
        locals.var_ndepm2_dn2 = assign24230_e18687_d_n2;
        locals.var_ndepm2_dn4 = assign24230_e18687_d_n4;
        locals.var_ndepm2_dn5 = assign24230_e18687_d_n5;
        locals.var_ndepm2_dn6 = assign24230_e18687_d_n6;
        locals.var_ndepm2_dn7 = assign24230_e18687_d_n7;
        locals.var_ndepm2_dn8 = assign24230_e18687_d_n8;
        locals.var_ndepm2_dn9 = assign24230_e18687_d_n9;
        locals.var_ndepm2_dn10 = assign24230_e18687_d_n10;
        locals.var_ndepm2_dn11 = assign24230_e18687_d_n11;
        locals.var_ndepm2_dn14 = assign24230_e18687_d_n14;

        let (assign24240_e18697, assign24240_e18697_d_n0, assign24240_e18697_d_n2, assign24240_e18697_d_n4, assign24240_e18697_d_n5, assign24240_e18697_d_n6, assign24240_e18697_d_n7, assign24240_e18697_d_n8, assign24240_e18697_d_n9, assign24240_e18697_d_n10, assign24240_e18697_d_n11, assign24240_e18697_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24240_e18693: f64 = (1.6021918e-19 * locals.var_uc_ndepm);
        let assign24240_e18695: f64 = (assign24240_e18693 * 1.034943e-10);
        (assign24240_e18695, ((1.6021918e-19 * locals.var_uc_ndepm_dn0) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn2) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn4) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn5) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn6) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn7) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn8) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn9) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn10) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn11) * 1.034943e-10), ((1.6021918e-19 * locals.var_uc_ndepm_dn14) * 1.034943e-10),)
    } else {
        (locals.var_q_ndepm_esi, locals.var_q_ndepm_esi_dn0, locals.var_q_ndepm_esi_dn2, locals.var_q_ndepm_esi_dn4, locals.var_q_ndepm_esi_dn5, locals.var_q_ndepm_esi_dn6, locals.var_q_ndepm_esi_dn7, locals.var_q_ndepm_esi_dn8, locals.var_q_ndepm_esi_dn9, locals.var_q_ndepm_esi_dn10, locals.var_q_ndepm_esi_dn11, locals.var_q_ndepm_esi_dn14,)
    }
};
        locals.var_q_ndepm_esi = assign24240_e18697;
        locals.var_q_ndepm_esi_dn0 = assign24240_e18697_d_n0;
        locals.var_q_ndepm_esi_dn2 = assign24240_e18697_d_n2;
        locals.var_q_ndepm_esi_dn4 = assign24240_e18697_d_n4;
        locals.var_q_ndepm_esi_dn5 = assign24240_e18697_d_n5;
        locals.var_q_ndepm_esi_dn6 = assign24240_e18697_d_n6;
        locals.var_q_ndepm_esi_dn7 = assign24240_e18697_d_n7;
        locals.var_q_ndepm_esi_dn8 = assign24240_e18697_d_n8;
        locals.var_q_ndepm_esi_dn9 = assign24240_e18697_d_n9;
        locals.var_q_ndepm_esi_dn10 = assign24240_e18697_d_n10;
        locals.var_q_ndepm_esi_dn11 = assign24240_e18697_d_n11;
        locals.var_q_ndepm_esi_dn14 = assign24240_e18697_d_n14;

        let (assign24250_e18705, assign24250_e18705_d_n0, assign24250_e18705_d_n2, assign24250_e18705_d_n4, assign24250_e18705_d_n5, assign24250_e18705_d_n6, assign24250_e18705_d_n7, assign24250_e18705_d_n8, assign24250_e18705_d_n9, assign24250_e18705_d_n10, assign24250_e18705_d_n11, assign24250_e18705_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24250_e18703: f64 = (1.6021918e-19 * locals.var_ef_nsubc);
        (assign24250_e18703, (1.6021918e-19 * locals.var_ef_nsubc_dn0), (1.6021918e-19 * locals.var_ef_nsubc_dn2), (1.6021918e-19 * locals.var_ef_nsubc_dn4), (1.6021918e-19 * locals.var_ef_nsubc_dn5), (1.6021918e-19 * locals.var_ef_nsubc_dn6), (1.6021918e-19 * locals.var_ef_nsubc_dn7), (1.6021918e-19 * locals.var_ef_nsubc_dn8), (1.6021918e-19 * locals.var_ef_nsubc_dn9), (1.6021918e-19 * locals.var_ef_nsubc_dn10), (1.6021918e-19 * locals.var_ef_nsubc_dn11), (1.6021918e-19 * locals.var_ef_nsubc_dn14),)
    } else {
        (locals.var_q_nsub__blk546, locals.var_q_nsub__blk546_dn0, locals.var_q_nsub__blk546_dn2, locals.var_q_nsub__blk546_dn4, locals.var_q_nsub__blk546_dn5, locals.var_q_nsub__blk546_dn6, locals.var_q_nsub__blk546_dn7, locals.var_q_nsub__blk546_dn8, locals.var_q_nsub__blk546_dn9, locals.var_q_nsub__blk546_dn10, locals.var_q_nsub__blk546_dn11, locals.var_q_nsub__blk546_dn14,)
    }
};
        locals.var_q_nsub__blk546 = assign24250_e18705;
        locals.var_q_nsub__blk546_dn0 = assign24250_e18705_d_n0;
        locals.var_q_nsub__blk546_dn2 = assign24250_e18705_d_n2;
        locals.var_q_nsub__blk546_dn4 = assign24250_e18705_d_n4;
        locals.var_q_nsub__blk546_dn5 = assign24250_e18705_d_n5;
        locals.var_q_nsub__blk546_dn6 = assign24250_e18705_d_n6;
        locals.var_q_nsub__blk546_dn7 = assign24250_e18705_d_n7;
        locals.var_q_nsub__blk546_dn8 = assign24250_e18705_d_n8;
        locals.var_q_nsub__blk546_dn9 = assign24250_e18705_d_n9;
        locals.var_q_nsub__blk546_dn10 = assign24250_e18705_d_n10;
        locals.var_q_nsub__blk546_dn11 = assign24250_e18705_d_n11;
        locals.var_q_nsub__blk546_dn14 = assign24250_e18705_d_n14;

        let (assign24260_e18713,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24260_e18711: f64 = (1.6021918e-19 * 1.6021918e-19);
        (assign24260_e18711,)
    } else {
        (locals.var_c_qe2,)
    }
};
        locals.var_c_qe2 = assign24260_e18713;

        let (assign24270_e18721,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24270_e18719: f64 = (1.034943e-10 * 1.034943e-10);
        (assign24270_e18719,)
    } else {
        (locals.var_c_esi2,)
    }
};
        locals.var_c_esi2 = assign24270_e18721;

        let (assign24280_e18729, assign24280_e18729_d_n0, assign24280_e18729_d_n2, assign24280_e18729_d_n4, assign24280_e18729_d_n5, assign24280_e18729_d_n6, assign24280_e18729_d_n7, assign24280_e18729_d_n8, assign24280_e18729_d_n9, assign24280_e18729_d_n10, assign24280_e18729_d_n11, assign24280_e18729_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24280_e18727: f64 = (locals.var_uc_depthn * locals.var_uc_depthn);
        (assign24280_e18727, ((locals.var_uc_depthn_dn0 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn0)), ((locals.var_uc_depthn_dn2 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn2)), ((locals.var_uc_depthn_dn4 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn4)), ((locals.var_uc_depthn_dn5 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn5)), ((locals.var_uc_depthn_dn6 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn6)), ((locals.var_uc_depthn_dn7 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn7)), ((locals.var_uc_depthn_dn8 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn8)), ((locals.var_uc_depthn_dn9 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn9)), ((locals.var_uc_depthn_dn10 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn10)), ((locals.var_uc_depthn_dn11 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn11)), ((locals.var_uc_depthn_dn14 * locals.var_uc_depthn) + (locals.var_uc_depthn * locals.var_uc_depthn_dn14)),)
    } else {
        (locals.var_tn2, locals.var_tn2_dn0, locals.var_tn2_dn2, locals.var_tn2_dn4, locals.var_tn2_dn5, locals.var_tn2_dn6, locals.var_tn2_dn7, locals.var_tn2_dn8, locals.var_tn2_dn9, locals.var_tn2_dn10, locals.var_tn2_dn11, locals.var_tn2_dn14,)
    }
};
        locals.var_tn2 = assign24280_e18729;
        locals.var_tn2_dn0 = assign24280_e18729_d_n0;
        locals.var_tn2_dn2 = assign24280_e18729_d_n2;
        locals.var_tn2_dn4 = assign24280_e18729_d_n4;
        locals.var_tn2_dn5 = assign24280_e18729_d_n5;
        locals.var_tn2_dn6 = assign24280_e18729_d_n6;
        locals.var_tn2_dn7 = assign24280_e18729_d_n7;
        locals.var_tn2_dn8 = assign24280_e18729_d_n8;
        locals.var_tn2_dn9 = assign24280_e18729_d_n9;
        locals.var_tn2_dn10 = assign24280_e18729_d_n10;
        locals.var_tn2_dn11 = assign24280_e18729_d_n11;
        locals.var_tn2_dn14 = assign24280_e18729_d_n14;

        let (assign24290_e18739, assign24290_e18739_d_n0, assign24290_e18739_d_n2, assign24290_e18739_d_n4, assign24290_e18739_d_n5, assign24290_e18739_d_n6, assign24290_e18739_d_n7, assign24290_e18739_d_n8, assign24290_e18739_d_n9, assign24290_e18739_d_n10, assign24290_e18739_d_n11, assign24290_e18739_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24290_e18735: f64 = (2.0 * 1.034943e-10);
        let assign24290_e18737: f64 = (assign24290_e18735 / locals.var_q_ndepm);
        (assign24290_e18737, (-((assign24290_e18735 * locals.var_q_ndepm_dn0) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24290_e18735 * locals.var_q_ndepm_dn2) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24290_e18735 * locals.var_q_ndepm_dn4) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24290_e18735 * locals.var_q_ndepm_dn5) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24290_e18735 * locals.var_q_ndepm_dn6) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24290_e18735 * locals.var_q_ndepm_dn7) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24290_e18735 * locals.var_q_ndepm_dn8) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24290_e18735 * locals.var_q_ndepm_dn9) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24290_e18735 * locals.var_q_ndepm_dn10) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24290_e18735 * locals.var_q_ndepm_dn11) / (locals.var_q_ndepm * locals.var_q_ndepm))), (-((assign24290_e18735 * locals.var_q_ndepm_dn14) / (locals.var_q_ndepm * locals.var_q_ndepm))),)
    } else {
        (locals.var_c_2esipq_ndepm, locals.var_c_2esipq_ndepm_dn0, locals.var_c_2esipq_ndepm_dn2, locals.var_c_2esipq_ndepm_dn4, locals.var_c_2esipq_ndepm_dn5, locals.var_c_2esipq_ndepm_dn6, locals.var_c_2esipq_ndepm_dn7, locals.var_c_2esipq_ndepm_dn8, locals.var_c_2esipq_ndepm_dn9, locals.var_c_2esipq_ndepm_dn10, locals.var_c_2esipq_ndepm_dn11, locals.var_c_2esipq_ndepm_dn14,)
    }
};
        locals.var_c_2esipq_ndepm = assign24290_e18739;
        locals.var_c_2esipq_ndepm_dn0 = assign24290_e18739_d_n0;
        locals.var_c_2esipq_ndepm_dn2 = assign24290_e18739_d_n2;
        locals.var_c_2esipq_ndepm_dn4 = assign24290_e18739_d_n4;
        locals.var_c_2esipq_ndepm_dn5 = assign24290_e18739_d_n5;
        locals.var_c_2esipq_ndepm_dn6 = assign24290_e18739_d_n6;
        locals.var_c_2esipq_ndepm_dn7 = assign24290_e18739_d_n7;
        locals.var_c_2esipq_ndepm_dn8 = assign24290_e18739_d_n8;
        locals.var_c_2esipq_ndepm_dn9 = assign24290_e18739_d_n9;
        locals.var_c_2esipq_ndepm_dn10 = assign24290_e18739_d_n10;
        locals.var_c_2esipq_ndepm_dn11 = assign24290_e18739_d_n11;
        locals.var_c_2esipq_ndepm_dn14 = assign24290_e18739_d_n14;

        let (assign24300_e18749, assign24300_e18749_d_n0, assign24300_e18749_d_n2, assign24300_e18749_d_n4, assign24300_e18749_d_n5, assign24300_e18749_d_n6, assign24300_e18749_d_n7, assign24300_e18749_d_n8, assign24300_e18749_d_n9, assign24300_e18749_d_n10, assign24300_e18749_d_n11, assign24300_e18749_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24300_e18746: f64 = (2.0 * 1.034943e-10);
        let assign24300_e18747: f64 = (locals.var_q_ndepm / assign24300_e18746);
        (assign24300_e18747, (locals.var_q_ndepm_dn0 / assign24300_e18746), (locals.var_q_ndepm_dn2 / assign24300_e18746), (locals.var_q_ndepm_dn4 / assign24300_e18746), (locals.var_q_ndepm_dn5 / assign24300_e18746), (locals.var_q_ndepm_dn6 / assign24300_e18746), (locals.var_q_ndepm_dn7 / assign24300_e18746), (locals.var_q_ndepm_dn8 / assign24300_e18746), (locals.var_q_ndepm_dn9 / assign24300_e18746), (locals.var_q_ndepm_dn10 / assign24300_e18746), (locals.var_q_ndepm_dn11 / assign24300_e18746), (locals.var_q_ndepm_dn14 / assign24300_e18746),)
    } else {
        (locals.var_c_2esipq_ndepm_inv, locals.var_c_2esipq_ndepm_inv_dn0, locals.var_c_2esipq_ndepm_inv_dn2, locals.var_c_2esipq_ndepm_inv_dn4, locals.var_c_2esipq_ndepm_inv_dn5, locals.var_c_2esipq_ndepm_inv_dn6, locals.var_c_2esipq_ndepm_inv_dn7, locals.var_c_2esipq_ndepm_inv_dn8, locals.var_c_2esipq_ndepm_inv_dn9, locals.var_c_2esipq_ndepm_inv_dn10, locals.var_c_2esipq_ndepm_inv_dn11, locals.var_c_2esipq_ndepm_inv_dn14,)
    }
};
        locals.var_c_2esipq_ndepm_inv = assign24300_e18749;
        locals.var_c_2esipq_ndepm_inv_dn0 = assign24300_e18749_d_n0;
        locals.var_c_2esipq_ndepm_inv_dn2 = assign24300_e18749_d_n2;
        locals.var_c_2esipq_ndepm_inv_dn4 = assign24300_e18749_d_n4;
        locals.var_c_2esipq_ndepm_inv_dn5 = assign24300_e18749_d_n5;
        locals.var_c_2esipq_ndepm_inv_dn6 = assign24300_e18749_d_n6;
        locals.var_c_2esipq_ndepm_inv_dn7 = assign24300_e18749_d_n7;
        locals.var_c_2esipq_ndepm_inv_dn8 = assign24300_e18749_d_n8;
        locals.var_c_2esipq_ndepm_inv_dn9 = assign24300_e18749_d_n9;
        locals.var_c_2esipq_ndepm_inv_dn10 = assign24300_e18749_d_n10;
        locals.var_c_2esipq_ndepm_inv_dn11 = assign24300_e18749_d_n11;
        locals.var_c_2esipq_ndepm_inv_dn14 = assign24300_e18749_d_n14;

        let (assign24310_e18759, assign24310_e18759_d_n0, assign24310_e18759_d_n2, assign24310_e18759_d_n4, assign24310_e18759_d_n5, assign24310_e18759_d_n6, assign24310_e18759_d_n7, assign24310_e18759_d_n8, assign24310_e18759_d_n9, assign24310_e18759_d_n10, assign24310_e18759_d_n11, assign24310_e18759_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24310_e18755: f64 = (2.0 * 1.034943e-10);
        let assign24310_e18757: f64 = (assign24310_e18755 * locals.var_q_ndepm);
        (assign24310_e18757, (assign24310_e18755 * locals.var_q_ndepm_dn0), (assign24310_e18755 * locals.var_q_ndepm_dn2), (assign24310_e18755 * locals.var_q_ndepm_dn4), (assign24310_e18755 * locals.var_q_ndepm_dn5), (assign24310_e18755 * locals.var_q_ndepm_dn6), (assign24310_e18755 * locals.var_q_ndepm_dn7), (assign24310_e18755 * locals.var_q_ndepm_dn8), (assign24310_e18755 * locals.var_q_ndepm_dn9), (assign24310_e18755 * locals.var_q_ndepm_dn10), (assign24310_e18755 * locals.var_q_ndepm_dn11), (assign24310_e18755 * locals.var_q_ndepm_dn14),)
    } else {
        (locals.var_c_2esi_q_ndepm, locals.var_c_2esi_q_ndepm_dn0, locals.var_c_2esi_q_ndepm_dn2, locals.var_c_2esi_q_ndepm_dn4, locals.var_c_2esi_q_ndepm_dn5, locals.var_c_2esi_q_ndepm_dn6, locals.var_c_2esi_q_ndepm_dn7, locals.var_c_2esi_q_ndepm_dn8, locals.var_c_2esi_q_ndepm_dn9, locals.var_c_2esi_q_ndepm_dn10, locals.var_c_2esi_q_ndepm_dn11, locals.var_c_2esi_q_ndepm_dn14,)
    }
};
        locals.var_c_2esi_q_ndepm = assign24310_e18759;
        locals.var_c_2esi_q_ndepm_dn0 = assign24310_e18759_d_n0;
        locals.var_c_2esi_q_ndepm_dn2 = assign24310_e18759_d_n2;
        locals.var_c_2esi_q_ndepm_dn4 = assign24310_e18759_d_n4;
        locals.var_c_2esi_q_ndepm_dn5 = assign24310_e18759_d_n5;
        locals.var_c_2esi_q_ndepm_dn6 = assign24310_e18759_d_n6;
        locals.var_c_2esi_q_ndepm_dn7 = assign24310_e18759_d_n7;
        locals.var_c_2esi_q_ndepm_dn8 = assign24310_e18759_d_n8;
        locals.var_c_2esi_q_ndepm_dn9 = assign24310_e18759_d_n9;
        locals.var_c_2esi_q_ndepm_dn10 = assign24310_e18759_d_n10;
        locals.var_c_2esi_q_ndepm_dn11 = assign24310_e18759_d_n11;
        locals.var_c_2esi_q_ndepm_dn14 = assign24310_e18759_d_n14;

        let (assign24320_e18769, assign24320_e18769_d_n0, assign24320_e18769_d_n2, assign24320_e18769_d_n4, assign24320_e18769_d_n5, assign24320_e18769_d_n6, assign24320_e18769_d_n7, assign24320_e18769_d_n8, assign24320_e18769_d_n9, assign24320_e18769_d_n10, assign24320_e18769_d_n11, assign24320_e18769_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24320_e18765: f64 = (2.0 * 1.034943e-10);
        let assign24320_e18767: f64 = (assign24320_e18765 / locals.var_q_nsub__blk546);
        (assign24320_e18767, (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn0) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))), (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn2) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))), (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn4) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))), (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn5) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))), (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn6) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))), (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn7) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))), (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn8) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))), (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn9) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))), (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn10) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))), (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn11) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))), (-((assign24320_e18765 * locals.var_q_nsub__blk546_dn14) / (locals.var_q_nsub__blk546 * locals.var_q_nsub__blk546))),)
    } else {
        (locals.var_c_2esipq_nsub, locals.var_c_2esipq_nsub_dn0, locals.var_c_2esipq_nsub_dn2, locals.var_c_2esipq_nsub_dn4, locals.var_c_2esipq_nsub_dn5, locals.var_c_2esipq_nsub_dn6, locals.var_c_2esipq_nsub_dn7, locals.var_c_2esipq_nsub_dn8, locals.var_c_2esipq_nsub_dn9, locals.var_c_2esipq_nsub_dn10, locals.var_c_2esipq_nsub_dn11, locals.var_c_2esipq_nsub_dn14,)
    }
};
        locals.var_c_2esipq_nsub = assign24320_e18769;
        locals.var_c_2esipq_nsub_dn0 = assign24320_e18769_d_n0;
        locals.var_c_2esipq_nsub_dn2 = assign24320_e18769_d_n2;
        locals.var_c_2esipq_nsub_dn4 = assign24320_e18769_d_n4;
        locals.var_c_2esipq_nsub_dn5 = assign24320_e18769_d_n5;
        locals.var_c_2esipq_nsub_dn6 = assign24320_e18769_d_n6;
        locals.var_c_2esipq_nsub_dn7 = assign24320_e18769_d_n7;
        locals.var_c_2esipq_nsub_dn8 = assign24320_e18769_d_n8;
        locals.var_c_2esipq_nsub_dn9 = assign24320_e18769_d_n9;
        locals.var_c_2esipq_nsub_dn10 = assign24320_e18769_d_n10;
        locals.var_c_2esipq_nsub_dn11 = assign24320_e18769_d_n11;
        locals.var_c_2esipq_nsub_dn14 = assign24320_e18769_d_n14;

        let (assign24330_e18779, assign24330_e18779_d_n0, assign24330_e18779_d_n2, assign24330_e18779_d_n4, assign24330_e18779_d_n5, assign24330_e18779_d_n6, assign24330_e18779_d_n7, assign24330_e18779_d_n8, assign24330_e18779_d_n9, assign24330_e18779_d_n10, assign24330_e18779_d_n11, assign24330_e18779_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24330_e18776: f64 = (2.0 * 1.034943e-10);
        let assign24330_e18777: f64 = (locals.var_q_nsub__blk546 / assign24330_e18776);
        (assign24330_e18777, (locals.var_q_nsub__blk546_dn0 / assign24330_e18776), (locals.var_q_nsub__blk546_dn2 / assign24330_e18776), (locals.var_q_nsub__blk546_dn4 / assign24330_e18776), (locals.var_q_nsub__blk546_dn5 / assign24330_e18776), (locals.var_q_nsub__blk546_dn6 / assign24330_e18776), (locals.var_q_nsub__blk546_dn7 / assign24330_e18776), (locals.var_q_nsub__blk546_dn8 / assign24330_e18776), (locals.var_q_nsub__blk546_dn9 / assign24330_e18776), (locals.var_q_nsub__blk546_dn10 / assign24330_e18776), (locals.var_q_nsub__blk546_dn11 / assign24330_e18776), (locals.var_q_nsub__blk546_dn14 / assign24330_e18776),)
    } else {
        (locals.var_c_2esipq_nsub_inv, locals.var_c_2esipq_nsub_inv_dn0, locals.var_c_2esipq_nsub_inv_dn2, locals.var_c_2esipq_nsub_inv_dn4, locals.var_c_2esipq_nsub_inv_dn5, locals.var_c_2esipq_nsub_inv_dn6, locals.var_c_2esipq_nsub_inv_dn7, locals.var_c_2esipq_nsub_inv_dn8, locals.var_c_2esipq_nsub_inv_dn9, locals.var_c_2esipq_nsub_inv_dn10, locals.var_c_2esipq_nsub_inv_dn11, locals.var_c_2esipq_nsub_inv_dn14,)
    }
};
        locals.var_c_2esipq_nsub_inv = assign24330_e18779;
        locals.var_c_2esipq_nsub_inv_dn0 = assign24330_e18779_d_n0;
        locals.var_c_2esipq_nsub_inv_dn2 = assign24330_e18779_d_n2;
        locals.var_c_2esipq_nsub_inv_dn4 = assign24330_e18779_d_n4;
        locals.var_c_2esipq_nsub_inv_dn5 = assign24330_e18779_d_n5;
        locals.var_c_2esipq_nsub_inv_dn6 = assign24330_e18779_d_n6;
        locals.var_c_2esipq_nsub_inv_dn7 = assign24330_e18779_d_n7;
        locals.var_c_2esipq_nsub_inv_dn8 = assign24330_e18779_d_n8;
        locals.var_c_2esipq_nsub_inv_dn9 = assign24330_e18779_d_n9;
        locals.var_c_2esipq_nsub_inv_dn10 = assign24330_e18779_d_n10;
        locals.var_c_2esipq_nsub_inv_dn11 = assign24330_e18779_d_n11;
        locals.var_c_2esipq_nsub_inv_dn14 = assign24330_e18779_d_n14;

        let (assign24340_e18787, assign24340_e18787_d_n0, assign24340_e18787_d_n2, assign24340_e18787_d_n4, assign24340_e18787_d_n5, assign24340_e18787_d_n6, assign24340_e18787_d_n7, assign24340_e18787_d_n8, assign24340_e18787_d_n9, assign24340_e18787_d_n10, assign24340_e18787_d_n11, assign24340_e18787_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24340_e18785: f64 = (locals.var_uc_ndepm / locals.var_ef_nsubc);
        (assign24340_e18785, (((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn11 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn11)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)), (((locals.var_uc_ndepm_dn14 * locals.var_ef_nsubc) - (locals.var_uc_ndepm * locals.var_ef_nsubc_dn14)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)),)
    } else {
        (locals.var_ndepmpnsub, locals.var_ndepmpnsub_dn0, locals.var_ndepmpnsub_dn2, locals.var_ndepmpnsub_dn4, locals.var_ndepmpnsub_dn5, locals.var_ndepmpnsub_dn6, locals.var_ndepmpnsub_dn7, locals.var_ndepmpnsub_dn8, locals.var_ndepmpnsub_dn9, locals.var_ndepmpnsub_dn10, locals.var_ndepmpnsub_dn11, locals.var_ndepmpnsub_dn14,)
    }
};
        locals.var_ndepmpnsub = assign24340_e18787;
        locals.var_ndepmpnsub_dn0 = assign24340_e18787_d_n0;
        locals.var_ndepmpnsub_dn2 = assign24340_e18787_d_n2;
        locals.var_ndepmpnsub_dn4 = assign24340_e18787_d_n4;
        locals.var_ndepmpnsub_dn5 = assign24340_e18787_d_n5;
        locals.var_ndepmpnsub_dn6 = assign24340_e18787_d_n6;
        locals.var_ndepmpnsub_dn7 = assign24340_e18787_d_n7;
        locals.var_ndepmpnsub_dn8 = assign24340_e18787_d_n8;
        locals.var_ndepmpnsub_dn9 = assign24340_e18787_d_n9;
        locals.var_ndepmpnsub_dn10 = assign24340_e18787_d_n10;
        locals.var_ndepmpnsub_dn11 = assign24340_e18787_d_n11;
        locals.var_ndepmpnsub_dn14 = assign24340_e18787_d_n14;

        let (assign24350_e18797, assign24350_e18797_d_n0, assign24350_e18797_d_n2, assign24350_e18797_d_n4, assign24350_e18797_d_n5, assign24350_e18797_d_n6, assign24350_e18797_d_n7, assign24350_e18797_d_n8, assign24350_e18797_d_n9, assign24350_e18797_d_n10, assign24350_e18797_d_n11, assign24350_e18797_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24350_e18794: f64 = (1.0 + locals.var_ndepmpnsub);
        let assign24350_e18795: f64 = (1.0 / assign24350_e18794);
        (assign24350_e18795, (-(locals.var_ndepmpnsub_dn0 / (assign24350_e18794 * assign24350_e18794))), (-(locals.var_ndepmpnsub_dn2 / (assign24350_e18794 * assign24350_e18794))), (-(locals.var_ndepmpnsub_dn4 / (assign24350_e18794 * assign24350_e18794))), (-(locals.var_ndepmpnsub_dn5 / (assign24350_e18794 * assign24350_e18794))), (-(locals.var_ndepmpnsub_dn6 / (assign24350_e18794 * assign24350_e18794))), (-(locals.var_ndepmpnsub_dn7 / (assign24350_e18794 * assign24350_e18794))), (-(locals.var_ndepmpnsub_dn8 / (assign24350_e18794 * assign24350_e18794))), (-(locals.var_ndepmpnsub_dn9 / (assign24350_e18794 * assign24350_e18794))), (-(locals.var_ndepmpnsub_dn10 / (assign24350_e18794 * assign24350_e18794))), (-(locals.var_ndepmpnsub_dn11 / (assign24350_e18794 * assign24350_e18794))), (-(locals.var_ndepmpnsub_dn14 / (assign24350_e18794 * assign24350_e18794))),)
    } else {
        (locals.var_ndepmpnsub_inv1, locals.var_ndepmpnsub_inv1_dn0, locals.var_ndepmpnsub_inv1_dn2, locals.var_ndepmpnsub_inv1_dn4, locals.var_ndepmpnsub_inv1_dn5, locals.var_ndepmpnsub_inv1_dn6, locals.var_ndepmpnsub_inv1_dn7, locals.var_ndepmpnsub_inv1_dn8, locals.var_ndepmpnsub_inv1_dn9, locals.var_ndepmpnsub_inv1_dn10, locals.var_ndepmpnsub_inv1_dn11, locals.var_ndepmpnsub_inv1_dn14,)
    }
};
        locals.var_ndepmpnsub_inv1 = assign24350_e18797;
        locals.var_ndepmpnsub_inv1_dn0 = assign24350_e18797_d_n0;
        locals.var_ndepmpnsub_inv1_dn2 = assign24350_e18797_d_n2;
        locals.var_ndepmpnsub_inv1_dn4 = assign24350_e18797_d_n4;
        locals.var_ndepmpnsub_inv1_dn5 = assign24350_e18797_d_n5;
        locals.var_ndepmpnsub_inv1_dn6 = assign24350_e18797_d_n6;
        locals.var_ndepmpnsub_inv1_dn7 = assign24350_e18797_d_n7;
        locals.var_ndepmpnsub_inv1_dn8 = assign24350_e18797_d_n8;
        locals.var_ndepmpnsub_inv1_dn9 = assign24350_e18797_d_n9;
        locals.var_ndepmpnsub_inv1_dn10 = assign24350_e18797_d_n10;
        locals.var_ndepmpnsub_inv1_dn11 = assign24350_e18797_d_n11;
        locals.var_ndepmpnsub_inv1_dn14 = assign24350_e18797_d_n14;

        let (assign24360_e18805,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24360_e18803: f64 = (1e-12 * 1000.0);
        (assign24360_e18803,)
    } else {
        (locals.var_ps_conv3,)
    }
};
        locals.var_ps_conv3 = assign24360_e18805;

        let (assign24370_e18813,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        let assign24370_e18811: f64 = (1e-10 * 1000.0);
        (assign24370_e18811,)
    } else {
        (locals.var_ps_conv23,)
    }
};
        locals.var_ps_conv23 = assign24370_e18813;

        let (assign24380_e18819, assign24380_e18819_d_n0, assign24380_e18819_d_n2, assign24380_e18819_d_n4, assign24380_e18819_d_n5, assign24380_e18819_d_n6, assign24380_e18819_d_n7, assign24380_e18819_d_n8, assign24380_e18819_d_n9, assign24380_e18819_d_n10, assign24380_e18819_d_n11, assign24380_e18819_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_s0_dep, locals.var_phi_s0_dep_dn0, locals.var_phi_s0_dep_dn2, locals.var_phi_s0_dep_dn4, locals.var_phi_s0_dep_dn5, locals.var_phi_s0_dep_dn6, locals.var_phi_s0_dep_dn7, locals.var_phi_s0_dep_dn8, locals.var_phi_s0_dep_dn9, locals.var_phi_s0_dep_dn10, locals.var_phi_s0_dep_dn11, locals.var_phi_s0_dep_dn14,)
    }
};
        locals.var_phi_s0_dep = assign24380_e18819;
        locals.var_phi_s0_dep_dn0 = assign24380_e18819_d_n0;
        locals.var_phi_s0_dep_dn2 = assign24380_e18819_d_n2;
        locals.var_phi_s0_dep_dn4 = assign24380_e18819_d_n4;
        locals.var_phi_s0_dep_dn5 = assign24380_e18819_d_n5;
        locals.var_phi_s0_dep_dn6 = assign24380_e18819_d_n6;
        locals.var_phi_s0_dep_dn7 = assign24380_e18819_d_n7;
        locals.var_phi_s0_dep_dn8 = assign24380_e18819_d_n8;
        locals.var_phi_s0_dep_dn9 = assign24380_e18819_d_n9;
        locals.var_phi_s0_dep_dn10 = assign24380_e18819_d_n10;
        locals.var_phi_s0_dep_dn11 = assign24380_e18819_d_n11;
        locals.var_phi_s0_dep_dn14 = assign24380_e18819_d_n14;

        let (assign24390_e18825, assign24390_e18825_d_n0, assign24390_e18825_d_n2, assign24390_e18825_d_n4, assign24390_e18825_d_n5, assign24390_e18825_d_n6, assign24390_e18825_d_n7, assign24390_e18825_d_n8, assign24390_e18825_d_n9, assign24390_e18825_d_n10, assign24390_e18825_d_n11, assign24390_e18825_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_phi_sl_dep, locals.var_phi_sl_dep_dn0, locals.var_phi_sl_dep_dn2, locals.var_phi_sl_dep_dn4, locals.var_phi_sl_dep_dn5, locals.var_phi_sl_dep_dn6, locals.var_phi_sl_dep_dn7, locals.var_phi_sl_dep_dn8, locals.var_phi_sl_dep_dn9, locals.var_phi_sl_dep_dn10, locals.var_phi_sl_dep_dn11, locals.var_phi_sl_dep_dn14,)
    }
};
        locals.var_phi_sl_dep = assign24390_e18825;
        locals.var_phi_sl_dep_dn0 = assign24390_e18825_d_n0;
        locals.var_phi_sl_dep_dn2 = assign24390_e18825_d_n2;
        locals.var_phi_sl_dep_dn4 = assign24390_e18825_d_n4;
        locals.var_phi_sl_dep_dn5 = assign24390_e18825_d_n5;
        locals.var_phi_sl_dep_dn6 = assign24390_e18825_d_n6;
        locals.var_phi_sl_dep_dn7 = assign24390_e18825_d_n7;
        locals.var_phi_sl_dep_dn8 = assign24390_e18825_d_n8;
        locals.var_phi_sl_dep_dn9 = assign24390_e18825_d_n9;
        locals.var_phi_sl_dep_dn10 = assign24390_e18825_d_n10;
        locals.var_phi_sl_dep_dn11 = assign24390_e18825_d_n11;
        locals.var_phi_sl_dep_dn14 = assign24390_e18825_d_n14;

        let (assign24400_e18831, assign24400_e18831_d_n0, assign24400_e18831_d_n2, assign24400_e18831_d_n4, assign24400_e18831_d_n5, assign24400_e18831_d_n6, assign24400_e18831_d_n7, assign24400_e18831_d_n8, assign24400_e18831_d_n9, assign24400_e18831_d_n10, assign24400_e18831_d_n11, assign24400_e18831_d_n14,) = {
    if ((locals.var_guard445 != 0.0) && (locals.var_guard446 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_q_s0, locals.var_q_s0_dn0, locals.var_q_s0_dn2, locals.var_q_s0_dn4, locals.var_q_s0_dn5, locals.var_q_s0_dn6, locals.var_q_s0_dn7, locals.var_q_s0_dn8, locals.var_q_s0_dn9, locals.var_q_s0_dn10, locals.var_q_s0_dn11, locals.var_q_s0_dn14,)
    }
};
        locals.var_q_s0 = assign24400_e18831;
        locals.var_q_s0_dn0 = assign24400_e18831_d_n0;
        locals.var_q_s0_dn2 = assign24400_e18831_d_n2;
        locals.var_q_s0_dn4 = assign24400_e18831_d_n4;
        locals.var_q_s0_dn5 = assign24400_e18831_d_n5;
        locals.var_q_s0_dn6 = assign24400_e18831_d_n6;
        locals.var_q_s0_dn7 = assign24400_e18831_d_n7;
        locals.var_q_s0_dn8 = assign24400_e18831_d_n8;
        locals.var_q_s0_dn9 = assign24400_e18831_d_n9;
        locals.var_q_s0_dn10 = assign24400_e18831_d_n10;
        locals.var_q_s0_dn11 = assign24400_e18831_d_n11;
        locals.var_q_s0_dn14 = assign24400_e18831_d_n14;

    }
}

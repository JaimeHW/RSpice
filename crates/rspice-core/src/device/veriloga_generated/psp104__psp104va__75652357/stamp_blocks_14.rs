#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        var_chnl_type: f64,
        var_gbulk: f64,
        var_gdrain: f64,
        var_ggate: f64,
        var_gjuns: f64,
        var_gsource: f64,
        var_guard1718: f64,
        var_guard1719: f64,
        var_guard1720: f64,
        var_guard1721: f64,
        var_guard1722: f64,
        var_guard1723: f64,
        var_i_ds: f64,
        var_i_ds_dn5: f64,
        var_i_ds_dn6: f64,
        var_i_ds_dn7: f64,
        var_i_ds_dn8: f64,
        var_i_dsedge: f64,
        var_i_dsedge_dn5: f64,
        var_i_dsedge_dn6: f64,
        var_i_dsedge_dn7: f64,
        var_i_dsedge_dn8: f64,
        var_i_gb: f64,
        var_i_gb_dn5: f64,
        var_i_gb_dn6: f64,
        var_i_gb_dn7: f64,
        var_i_gb_dn8: f64,
        var_i_gcd: f64,
        var_i_gcd_dn5: f64,
        var_i_gcd_dn6: f64,
        var_i_gcd_dn7: f64,
        var_i_gcd_dn8: f64,
        var_i_gcs: f64,
        var_i_gcs_dn5: f64,
        var_i_gcs_dn6: f64,
        var_i_gcs_dn7: f64,
        var_i_gcs_dn8: f64,
        var_i_gidl: f64,
        var_i_gidl_dn5: f64,
        var_i_gidl_dn6: f64,
        var_i_gidl_dn7: f64,
        var_i_gidl_dn8: f64,
        var_i_gisl: f64,
        var_i_gisl_dn5: f64,
        var_i_gisl_dn6: f64,
        var_i_gisl_dn7: f64,
        var_i_gisl_dn8: f64,
        var_igdov: f64,
        var_igdov_dn5: f64,
        var_igdov_dn6: f64,
        var_igdov_dn7: f64,
        var_igdov_dn8: f64,
        var_igsov: f64,
        var_igsov_dn5: f64,
        var_igsov_dn6: f64,
        var_igsov_dn7: f64,
        var_igsov_dn8: f64,
        var_iimpact: f64,
        var_iimpact_dn5: f64,
        var_iimpact_dn6: f64,
        var_iimpact_dn7: f64,
        var_iimpact_dn8: f64,
        var_ijun_d: f64,
        var_ijun_d_dn10: f64,
        var_ijun_d_dn11: f64,
        var_ijun_d_dn5: f64,
        var_ijun_d_dn6: f64,
        var_ijun_d_dn7: f64,
        var_ijun_d_dn8: f64,
        var_ijun_s: f64,
        var_ijun_s_dn10: f64,
        var_ijun_s_dn11: f64,
        var_ijun_s_dn5: f64,
        var_ijun_s_dn6: f64,
        var_ijun_s_dn7: f64,
        var_ijun_s_dn8: f64,
        var_mult_inst: f64,
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
        let (eq0_e948, eq0_e948_d_n5, eq0_e948_d_n6, eq0_e948_d_n7, eq0_e948_d_n8,) = {
    if (var_guard1718 != 0.0) {
        let eq0_e942: f64 = (var_chnl_type * var_mult_inst);
        let eq0_e944: f64 = (eq0_e942 * p.p32);
        let eq0_e946: f64 = (eq0_e944 * var_iimpact);
        let eq0_e946_d_n5: f64 = (eq0_e944 * var_iimpact_dn5);
        let eq0_e946_d_n6: f64 = (eq0_e944 * var_iimpact_dn6);
        let eq0_e946_d_n7: f64 = (eq0_e944 * var_iimpact_dn7);
        let eq0_e946_d_n8: f64 = (eq0_e944 * var_iimpact_dn8);
        (eq0_e946, eq0_e946_d_n5, eq0_e946_d_n6, eq0_e946_d_n7, eq0_e946_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e948;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(8),
            multiplicity * (eq0_value),
            [5, 6, 7, 8],
            [multiplicity * (eq0_e948_d_n5), multiplicity * (eq0_e948_d_n6), multiplicity * (eq0_e948_d_n7), multiplicity * (eq0_e948_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq1_e960, eq1_e960_d_n5, eq1_e960_d_n6, eq1_e960_d_n7, eq1_e960_d_n8,) = {
    if (var_guard1718 != 0.0) {
        let eq1_e952: f64 = (var_chnl_type * var_mult_inst);
        let eq1_e954: f64 = (eq1_e952 * p.p32);
        let eq1_e957: f64 = (var_i_ds + var_i_dsedge);
        let eq1_e957_d_n5: f64 = (var_i_ds_dn5 + var_i_dsedge_dn5);
        let eq1_e957_d_n6: f64 = (var_i_ds_dn6 + var_i_dsedge_dn6);
        let eq1_e957_d_n7: f64 = (var_i_ds_dn7 + var_i_dsedge_dn7);
        let eq1_e957_d_n8: f64 = (var_i_ds_dn8 + var_i_dsedge_dn8);
        let eq1_e958: f64 = (eq1_e954 * eq1_e957);
        let eq1_e958_d_n5: f64 = (eq1_e954 * eq1_e957_d_n5);
        let eq1_e958_d_n6: f64 = (eq1_e954 * eq1_e957_d_n6);
        let eq1_e958_d_n7: f64 = (eq1_e954 * eq1_e957_d_n7);
        let eq1_e958_d_n8: f64 = (eq1_e954 * eq1_e957_d_n8);
        (eq1_e958, eq1_e958_d_n5, eq1_e958_d_n6, eq1_e958_d_n7, eq1_e958_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e960;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq1_value),
            [5, 6, 7, 8],
            [multiplicity * (eq1_e960_d_n5), multiplicity * (eq1_e960_d_n6), multiplicity * (eq1_e960_d_n7), multiplicity * (eq1_e960_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq2_e970, eq2_e970_d_n5, eq2_e970_d_n6, eq2_e970_d_n7, eq2_e970_d_n8,) = {
    if (var_guard1718 != 0.0) {
        let eq2_e964: f64 = (var_chnl_type * var_mult_inst);
        let eq2_e966: f64 = (eq2_e964 * p.p32);
        let eq2_e968: f64 = (eq2_e966 * var_i_gcs);
        let eq2_e968_d_n5: f64 = (eq2_e966 * var_i_gcs_dn5);
        let eq2_e968_d_n6: f64 = (eq2_e966 * var_i_gcs_dn6);
        let eq2_e968_d_n7: f64 = (eq2_e966 * var_i_gcs_dn7);
        let eq2_e968_d_n8: f64 = (eq2_e966 * var_i_gcs_dn8);
        (eq2_e968, eq2_e968_d_n5, eq2_e968_d_n6, eq2_e968_d_n7, eq2_e968_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e970;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq2_value),
            [5, 6, 7, 8],
            [multiplicity * (eq2_e970_d_n5), multiplicity * (eq2_e970_d_n6), multiplicity * (eq2_e970_d_n7), multiplicity * (eq2_e970_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq3_e980, eq3_e980_d_n5, eq3_e980_d_n6, eq3_e980_d_n7, eq3_e980_d_n8,) = {
    if (var_guard1718 != 0.0) {
        let eq3_e974: f64 = (var_chnl_type * var_mult_inst);
        let eq3_e976: f64 = (eq3_e974 * p.p32);
        let eq3_e978: f64 = (eq3_e976 * var_i_gcd);
        let eq3_e978_d_n5: f64 = (eq3_e976 * var_i_gcd_dn5);
        let eq3_e978_d_n6: f64 = (eq3_e976 * var_i_gcd_dn6);
        let eq3_e978_d_n7: f64 = (eq3_e976 * var_i_gcd_dn7);
        let eq3_e978_d_n8: f64 = (eq3_e976 * var_i_gcd_dn8);
        (eq3_e978, eq3_e978_d_n5, eq3_e978_d_n6, eq3_e978_d_n7, eq3_e978_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e980;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(7),
            multiplicity * (eq3_value),
            [5, 6, 7, 8],
            [multiplicity * (eq3_e980_d_n5), multiplicity * (eq3_e980_d_n6), multiplicity * (eq3_e980_d_n7), multiplicity * (eq3_e980_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq4_e991, eq4_e991_d_n5, eq4_e991_d_n6, eq4_e991_d_n7, eq4_e991_d_n8,) = {
    if (var_guard1718 == 0.0) {
        let eq4_e985: f64 = (var_chnl_type * var_mult_inst);
        let eq4_e987: f64 = (eq4_e985 * p.p32);
        let eq4_e989: f64 = (eq4_e987 * var_iimpact);
        let eq4_e989_d_n5: f64 = (eq4_e987 * var_iimpact_dn5);
        let eq4_e989_d_n6: f64 = (eq4_e987 * var_iimpact_dn6);
        let eq4_e989_d_n7: f64 = (eq4_e987 * var_iimpact_dn7);
        let eq4_e989_d_n8: f64 = (eq4_e987 * var_iimpact_dn8);
        (eq4_e989, eq4_e989_d_n5, eq4_e989_d_n6, eq4_e989_d_n7, eq4_e989_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e991;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq4_value),
            [5, 6, 7, 8],
            [multiplicity * (eq4_e991_d_n5), multiplicity * (eq4_e991_d_n6), multiplicity * (eq4_e991_d_n7), multiplicity * (eq4_e991_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq5_e1004, eq5_e1004_d_n5, eq5_e1004_d_n6, eq5_e1004_d_n7, eq5_e1004_d_n8,) = {
    if (var_guard1718 == 0.0) {
        let eq5_e996: f64 = (var_chnl_type * var_mult_inst);
        let eq5_e998: f64 = (eq5_e996 * p.p32);
        let eq5_e1001: f64 = (var_i_ds + var_i_dsedge);
        let eq5_e1001_d_n5: f64 = (var_i_ds_dn5 + var_i_dsedge_dn5);
        let eq5_e1001_d_n6: f64 = (var_i_ds_dn6 + var_i_dsedge_dn6);
        let eq5_e1001_d_n7: f64 = (var_i_ds_dn7 + var_i_dsedge_dn7);
        let eq5_e1001_d_n8: f64 = (var_i_ds_dn8 + var_i_dsedge_dn8);
        let eq5_e1002: f64 = (eq5_e998 * eq5_e1001);
        let eq5_e1002_d_n5: f64 = (eq5_e998 * eq5_e1001_d_n5);
        let eq5_e1002_d_n6: f64 = (eq5_e998 * eq5_e1001_d_n6);
        let eq5_e1002_d_n7: f64 = (eq5_e998 * eq5_e1001_d_n7);
        let eq5_e1002_d_n8: f64 = (eq5_e998 * eq5_e1001_d_n8);
        (eq5_e1002, eq5_e1002_d_n5, eq5_e1002_d_n6, eq5_e1002_d_n7, eq5_e1002_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1004;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq5_value),
            [5, 6, 7, 8],
            [multiplicity * (eq5_e1004_d_n5), multiplicity * (eq5_e1004_d_n6), multiplicity * (eq5_e1004_d_n7), multiplicity * (eq5_e1004_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq6_e1015, eq6_e1015_d_n5, eq6_e1015_d_n6, eq6_e1015_d_n7, eq6_e1015_d_n8,) = {
    if (var_guard1718 == 0.0) {
        let eq6_e1009: f64 = (var_chnl_type * var_mult_inst);
        let eq6_e1011: f64 = (eq6_e1009 * p.p32);
        let eq6_e1013: f64 = (eq6_e1011 * var_i_gcs);
        let eq6_e1013_d_n5: f64 = (eq6_e1011 * var_i_gcs_dn5);
        let eq6_e1013_d_n6: f64 = (eq6_e1011 * var_i_gcs_dn6);
        let eq6_e1013_d_n7: f64 = (eq6_e1011 * var_i_gcs_dn7);
        let eq6_e1013_d_n8: f64 = (eq6_e1011 * var_i_gcs_dn8);
        (eq6_e1013, eq6_e1013_d_n5, eq6_e1013_d_n6, eq6_e1013_d_n7, eq6_e1013_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1015;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(7),
            multiplicity * (eq6_value),
            [5, 6, 7, 8],
            [multiplicity * (eq6_e1015_d_n5), multiplicity * (eq6_e1015_d_n6), multiplicity * (eq6_e1015_d_n7), multiplicity * (eq6_e1015_d_n8)],
            [],
            [],
            1.0,
        );
        let (eq7_e1026, eq7_e1026_d_n5, eq7_e1026_d_n6, eq7_e1026_d_n7, eq7_e1026_d_n8,) = {
    if (var_guard1718 == 0.0) {
        let eq7_e1020: f64 = (var_chnl_type * var_mult_inst);
        let eq7_e1022: f64 = (eq7_e1020 * p.p32);
        let eq7_e1024: f64 = (eq7_e1022 * var_i_gcd);
        let eq7_e1024_d_n5: f64 = (eq7_e1022 * var_i_gcd_dn5);
        let eq7_e1024_d_n6: f64 = (eq7_e1022 * var_i_gcd_dn6);
        let eq7_e1024_d_n7: f64 = (eq7_e1022 * var_i_gcd_dn7);
        let eq7_e1024_d_n8: f64 = (eq7_e1022 * var_i_gcd_dn8);
        (eq7_e1024, eq7_e1024_d_n5, eq7_e1024_d_n6, eq7_e1024_d_n7, eq7_e1024_d_n8,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1026;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq7_value),
            [5, 6, 7, 8],
            [multiplicity * (eq7_e1026_d_n5), multiplicity * (eq7_e1026_d_n6), multiplicity * (eq7_e1026_d_n7), multiplicity * (eq7_e1026_d_n8)],
            [],
            [],
            1.0,
        );
        let eq8_e1029: f64 = (var_chnl_type * var_mult_inst);
        let eq8_e1031: f64 = (eq8_e1029 * p.p32);
        let eq8_e1033: f64 = (eq8_e1031 * var_i_gb);
        let eq8_e1033_d_n5: f64 = (eq8_e1031 * var_i_gb_dn5);
        let eq8_e1033_d_n6: f64 = (eq8_e1031 * var_i_gb_dn6);
        let eq8_e1033_d_n7: f64 = (eq8_e1031 * var_i_gb_dn7);
        let eq8_e1033_d_n8: f64 = (eq8_e1031 * var_i_gb_dn8);
        let eq8_value: f64 = eq8_e1033;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(8),
            multiplicity * (eq8_value),
            [5, 6, 7, 8],
            [multiplicity * (eq8_e1033_d_n5), multiplicity * (eq8_e1033_d_n6), multiplicity * (eq8_e1033_d_n7), multiplicity * (eq8_e1033_d_n8)],
            [],
            [],
            1.0,
        );
        let eq9_e1036: f64 = (var_chnl_type * var_mult_inst);
        let eq9_e1038: f64 = (eq9_e1036 * p.p32);
        let eq9_e1040: f64 = (eq9_e1038 * var_igsov);
        let eq9_e1040_d_n5: f64 = (eq9_e1038 * var_igsov_dn5);
        let eq9_e1040_d_n6: f64 = (eq9_e1038 * var_igsov_dn6);
        let eq9_e1040_d_n7: f64 = (eq9_e1038 * var_igsov_dn7);
        let eq9_e1040_d_n8: f64 = (eq9_e1038 * var_igsov_dn8);
        let eq9_value: f64 = eq9_e1040;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq9_value),
            [5, 6, 7, 8],
            [multiplicity * (eq9_e1040_d_n5), multiplicity * (eq9_e1040_d_n6), multiplicity * (eq9_e1040_d_n7), multiplicity * (eq9_e1040_d_n8)],
            [],
            [],
            1.0,
        );
        let eq10_e1043: f64 = (var_chnl_type * var_mult_inst);
        let eq10_e1045: f64 = (eq10_e1043 * p.p32);
        let eq10_e1047: f64 = (eq10_e1045 * var_igdov);
        let eq10_e1047_d_n5: f64 = (eq10_e1045 * var_igdov_dn5);
        let eq10_e1047_d_n6: f64 = (eq10_e1045 * var_igdov_dn6);
        let eq10_e1047_d_n7: f64 = (eq10_e1045 * var_igdov_dn7);
        let eq10_e1047_d_n8: f64 = (eq10_e1045 * var_igdov_dn8);
        let eq10_value: f64 = eq10_e1047;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(7),
            multiplicity * (eq10_value),
            [5, 6, 7, 8],
            [multiplicity * (eq10_e1047_d_n5), multiplicity * (eq10_e1047_d_n6), multiplicity * (eq10_e1047_d_n7), multiplicity * (eq10_e1047_d_n8)],
            [],
            [],
            1.0,
        );
        let eq11_e1050: f64 = (var_chnl_type * var_mult_inst);
        let eq11_e1052: f64 = (eq11_e1050 * p.p32);
        let eq11_e1054: f64 = (eq11_e1052 * var_i_gisl);
        let eq11_e1054_d_n5: f64 = (eq11_e1052 * var_i_gisl_dn5);
        let eq11_e1054_d_n6: f64 = (eq11_e1052 * var_i_gisl_dn6);
        let eq11_e1054_d_n7: f64 = (eq11_e1052 * var_i_gisl_dn7);
        let eq11_e1054_d_n8: f64 = (eq11_e1052 * var_i_gisl_dn8);
        let eq11_value: f64 = eq11_e1054;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(6),
            Some(8),
            multiplicity * (eq11_value),
            [5, 6, 7, 8],
            [multiplicity * (eq11_e1054_d_n5), multiplicity * (eq11_e1054_d_n6), multiplicity * (eq11_e1054_d_n7), multiplicity * (eq11_e1054_d_n8)],
            [],
            [],
            1.0,
        );
        let eq12_e1057: f64 = (var_chnl_type * var_mult_inst);
        let eq12_e1059: f64 = (eq12_e1057 * p.p32);
        let eq12_e1061: f64 = (eq12_e1059 * var_i_gidl);
        let eq12_e1061_d_n5: f64 = (eq12_e1059 * var_i_gidl_dn5);
        let eq12_e1061_d_n6: f64 = (eq12_e1059 * var_i_gidl_dn6);
        let eq12_e1061_d_n7: f64 = (eq12_e1059 * var_i_gidl_dn7);
        let eq12_e1061_d_n8: f64 = (eq12_e1059 * var_i_gidl_dn8);
        let eq12_value: f64 = eq12_e1061;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(8),
            multiplicity * (eq12_value),
            [5, 6, 7, 8],
            [multiplicity * (eq12_e1061_d_n5), multiplicity * (eq12_e1061_d_n6), multiplicity * (eq12_e1061_d_n7), multiplicity * (eq12_e1061_d_n8)],
            [],
            [],
            1.0,
        );
        let eq13_e1064: f64 = (var_chnl_type * var_mult_inst);
        let eq13_e1066: f64 = (eq13_e1064 * p.p32);
        let eq13_e1068: f64 = (eq13_e1066 * var_ijun_s);
        let eq13_e1068_d_n5: f64 = (eq13_e1066 * var_ijun_s_dn5);
        let eq13_e1068_d_n6: f64 = (eq13_e1066 * var_ijun_s_dn6);
        let eq13_e1068_d_n7: f64 = (eq13_e1066 * var_ijun_s_dn7);
        let eq13_e1068_d_n8: f64 = (eq13_e1066 * var_ijun_s_dn8);
        let eq13_e1068_d_n10: f64 = (eq13_e1066 * var_ijun_s_dn10);
        let eq13_e1068_d_n11: f64 = (eq13_e1066 * var_ijun_s_dn11);
        let eq13_value: f64 = eq13_e1068;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            Some(6),
            multiplicity * (eq13_value),
            [5, 6, 7, 8, 10, 11],
            [multiplicity * (eq13_e1068_d_n5), multiplicity * (eq13_e1068_d_n6), multiplicity * (eq13_e1068_d_n7), multiplicity * (eq13_e1068_d_n8), multiplicity * (eq13_e1068_d_n10), multiplicity * (eq13_e1068_d_n11)],
            [],
            [],
            1.0,
        );
        let eq14_e1071: f64 = (var_chnl_type * var_mult_inst);
        let eq14_e1073: f64 = (eq14_e1071 * p.p32);
        let eq14_e1075: f64 = (eq14_e1073 * var_ijun_d);
        let eq14_e1075_d_n5: f64 = (eq14_e1073 * var_ijun_d_dn5);
        let eq14_e1075_d_n6: f64 = (eq14_e1073 * var_ijun_d_dn6);
        let eq14_e1075_d_n7: f64 = (eq14_e1073 * var_ijun_d_dn7);
        let eq14_e1075_d_n8: f64 = (eq14_e1073 * var_ijun_d_dn8);
        let eq14_e1075_d_n10: f64 = (eq14_e1073 * var_ijun_d_dn10);
        let eq14_e1075_d_n11: f64 = (eq14_e1073 * var_ijun_d_dn11);
        let eq14_value: f64 = eq14_e1075;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(7),
            multiplicity * (eq14_value),
            [5, 6, 7, 8, 10, 11],
            [multiplicity * (eq14_e1075_d_n5), multiplicity * (eq14_e1075_d_n6), multiplicity * (eq14_e1075_d_n7), multiplicity * (eq14_e1075_d_n8), multiplicity * (eq14_e1075_d_n10), multiplicity * (eq14_e1075_d_n11)],
            [],
            [],
            1.0,
        );
        let (eq15_e1085, eq15_e1085_d_n1, eq15_e1085_d_n5,) = {
    if (var_guard1719 != 0.0) {
        let eq15_e1079: f64 = (var_mult_inst * p.p32);
        let eq15_e1081: f64 = (eq15_e1079 * var_ggate);
        let eq15_e1083: f64 = (eq15_e1081 * (nv1 - nv5));
        (eq15_e1083, eq15_e1081, (-eq15_e1081),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e1085;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(5),
            multiplicity * (eq15_value),
            1,
            multiplicity * (eq15_e1085_d_n1),
            5,
            multiplicity * (eq15_e1085_d_n5),
        );
        let (eq17_e1100,) = {
    if (var_guard1719 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e1100;
        stamper.stamp_potential_const_local(
            0,
            eq17_value,
        );
        let (eq18_e1110, eq18_e1110_d_n2, eq18_e1110_d_n6,) = {
    if (var_guard1720 != 0.0) {
        let eq18_e1104: f64 = (var_mult_inst * p.p32);
        let eq18_e1106: f64 = (eq18_e1104 * var_gsource);
        let eq18_e1108: f64 = (eq18_e1106 * (nv2 - nv6));
        (eq18_e1108, eq18_e1106, (-eq18_e1106),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1110;
        stamper.stamp_current_node2_local(
            Some(2),
            Some(6),
            multiplicity * (eq18_value),
            2,
            multiplicity * (eq18_e1110_d_n2),
            6,
            multiplicity * (eq18_e1110_d_n6),
        );
        let (eq20_e1125,) = {
    if (var_guard1720 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e1125;
        stamper.stamp_potential_const_local(
            1,
            eq20_value,
        );
        let (eq21_e1135, eq21_e1135_d_n0, eq21_e1135_d_n7,) = {
    if (var_guard1721 != 0.0) {
        let eq21_e1129: f64 = (var_mult_inst * p.p32);
        let eq21_e1131: f64 = (eq21_e1129 * var_gdrain);
        let eq21_e1133: f64 = (eq21_e1131 * (nv0 - nv7));
        (eq21_e1133, eq21_e1131, (-eq21_e1131),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1135;
        stamper.stamp_current_node2_local(
            Some(0),
            Some(7),
            multiplicity * (eq21_value),
            0,
            multiplicity * (eq21_e1135_d_n0),
            7,
            multiplicity * (eq21_e1135_d_n7),
        );
        let (eq23_e1150,) = {
    if (var_guard1721 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e1150;
        stamper.stamp_potential_const_local(
            2,
            eq23_value,
        );
        let (eq24_e1160, eq24_e1160_d_n8, eq24_e1160_d_n9,) = {
    if (var_guard1722 != 0.0) {
        let eq24_e1154: f64 = (var_mult_inst * p.p32);
        let eq24_e1156: f64 = (eq24_e1154 * var_gbulk);
        let eq24_e1158: f64 = (eq24_e1156 * (nv8 - nv9));
        (eq24_e1158, eq24_e1156, (-eq24_e1156),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1160;
        stamper.stamp_current_node2_local(
            Some(8),
            Some(9),
            multiplicity * (eq24_value),
            8,
            multiplicity * (eq24_e1160_d_n8),
            9,
            multiplicity * (eq24_e1160_d_n9),
        );
        let (eq26_e1175,) = {
    if (var_guard1722 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e1175;
        stamper.stamp_potential_const_local(
            3,
            eq26_value,
        );
        let (eq27_e1185, eq27_e1185_d_n9, eq27_e1185_d_n10,) = {
    if (var_guard1723 != 0.0) {
        let eq27_e1179: f64 = (var_mult_inst * p.p32);
        let eq27_e1181: f64 = (eq27_e1179 * var_gjuns);
        let eq27_e1183: f64 = (eq27_e1181 * (nv10 - nv9));
        (eq27_e1183, (-eq27_e1181), eq27_e1181,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1185;
        stamper.stamp_current_node2_local(
            Some(10),
            Some(9),
            multiplicity * (eq27_value),
            9,
            multiplicity * (eq27_e1185_d_n9),
            10,
            multiplicity * (eq27_e1185_d_n10),
        );
        let (eq29_e1200,) = {
    if (var_guard1723 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e1200;
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
        var_chnl_type: f64,
        var_gjund: f64,
        var_guard1724: f64,
        var_guard1725: f64,
        var_gwell: f64,
        var_mult_inst: f64,
        var_qb: f64,
        var_qb_dn5: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qb_dn8: f64,
        var_qd: f64,
        var_qd_dn5: f64,
        var_qd_dn6: f64,
        var_qd_dn7: f64,
        var_qd_dn8: f64,
        var_qfgd: f64,
        var_qfgd_dn5: f64,
        var_qfgd_dn6: f64,
        var_qfgd_dn7: f64,
        var_qfgs: f64,
        var_qfgs_dn5: f64,
        var_qfgs_dn6: f64,
        var_qfgs_dn7: f64,
        var_qg: f64,
        var_qg_dn5: f64,
        var_qg_dn6: f64,
        var_qg_dn7: f64,
        var_qg_dn8: f64,
        var_qgb_ov: f64,
        var_qgb_ov_dn5: f64,
        var_qgb_ov_dn6: f64,
        var_qgb_ov_dn7: f64,
        var_qgb_ov_dn8: f64,
        var_qjun_d: f64,
        var_qjun_d_dn10: f64,
        var_qjun_d_dn11: f64,
        var_qjun_d_dn5: f64,
        var_qjun_d_dn6: f64,
        var_qjun_d_dn7: f64,
        var_qjun_d_dn8: f64,
        var_qjun_s: f64,
        var_qjun_s_dn10: f64,
        var_qjun_s_dn11: f64,
        var_qjun_s_dn5: f64,
        var_qjun_s_dn6: f64,
        var_qjun_s_dn7: f64,
        var_qjun_s_dn8: f64,
    ) {
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq30_e1210, eq30_e1210_d_n9, eq30_e1210_d_n11,) = {
    if (var_guard1724 != 0.0) {
        let eq30_e1204: f64 = (var_mult_inst * p.p32);
        let eq30_e1206: f64 = (eq30_e1204 * var_gjund);
        let eq30_e1208: f64 = (eq30_e1206 * (nv11 - nv9));
        (eq30_e1208, (-eq30_e1206), eq30_e1206,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1210;
        stamper.stamp_current_node2_local(
            Some(11),
            Some(9),
            multiplicity * (eq30_value),
            9,
            multiplicity * (eq30_e1210_d_n9),
            11,
            multiplicity * (eq30_e1210_d_n11),
        );
        let (eq32_e1225,) = {
    if (var_guard1724 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e1225;
        stamper.stamp_potential_const_local(
            5,
            eq32_value,
        );
        let (eq33_e1235, eq33_e1235_d_n3, eq33_e1235_d_n9,) = {
    if (var_guard1725 != 0.0) {
        let eq33_e1229: f64 = (var_mult_inst * p.p32);
        let eq33_e1231: f64 = (eq33_e1229 * var_gwell);
        let eq33_e1233: f64 = (eq33_e1231 * (nv3 - nv9));
        (eq33_e1233, eq33_e1231, (-eq33_e1231),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1235;
        stamper.stamp_current_node2_local(
            Some(3),
            Some(9),
            multiplicity * (eq33_value),
            3,
            multiplicity * (eq33_e1235_d_n3),
            9,
            multiplicity * (eq33_e1235_d_n9),
        );
        let (eq35_e1250,) = {
    if (var_guard1725 == 0.0) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e1250;
        stamper.stamp_potential_const_local(
            6,
            eq35_value,
        );
        let eq38_e1263: f64 = (var_chnl_type * var_mult_inst);
        let eq38_e1265: f64 = (eq38_e1263 * p.p33);
        let eq38_e1267: f64 = (eq38_e1265 * var_qg);
        let eq38_e1267_d_n5: f64 = (eq38_e1265 * var_qg_dn5);
        let eq38_e1267_d_n6: f64 = (eq38_e1265 * var_qg_dn6);
        let eq38_e1267_d_n7: f64 = (eq38_e1265 * var_qg_dn7);
        let eq38_e1267_d_n8: f64 = (eq38_e1265 * var_qg_dn8);
        let eq38_e1268: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, eq38_e1267);
        let eq38_value: f64 = eq38_e1268;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq38_value),
            [5, 6, 7, 8],
            [multiplicity * ((eq38_e1267_d_n5 * ddt_scale)), multiplicity * ((eq38_e1267_d_n6 * ddt_scale)), multiplicity * ((eq38_e1267_d_n7 * ddt_scale)), multiplicity * ((eq38_e1267_d_n8 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq39_e1271: f64 = (var_chnl_type * var_mult_inst);
        let eq39_e1273: f64 = (eq39_e1271 * p.p33);
        let eq39_e1275: f64 = (eq39_e1273 * var_qb);
        let eq39_e1275_d_n5: f64 = (eq39_e1273 * var_qb_dn5);
        let eq39_e1275_d_n6: f64 = (eq39_e1273 * var_qb_dn6);
        let eq39_e1275_d_n7: f64 = (eq39_e1273 * var_qb_dn7);
        let eq39_e1275_d_n8: f64 = (eq39_e1273 * var_qb_dn8);
        let eq39_e1276: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, eq39_e1275);
        let eq39_value: f64 = eq39_e1276;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq39_value),
            [5, 6, 7, 8],
            [multiplicity * ((eq39_e1275_d_n5 * ddt_scale)), multiplicity * ((eq39_e1275_d_n6 * ddt_scale)), multiplicity * ((eq39_e1275_d_n7 * ddt_scale)), multiplicity * ((eq39_e1275_d_n8 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq40_e1279: f64 = (var_chnl_type * var_mult_inst);
        let eq40_e1281: f64 = (eq40_e1279 * p.p33);
        let eq40_e1283: f64 = (eq40_e1281 * var_qd);
        let eq40_e1283_d_n5: f64 = (eq40_e1281 * var_qd_dn5);
        let eq40_e1283_d_n6: f64 = (eq40_e1281 * var_qd_dn6);
        let eq40_e1283_d_n7: f64 = (eq40_e1281 * var_qd_dn7);
        let eq40_e1283_d_n8: f64 = (eq40_e1281 * var_qd_dn8);
        let eq40_e1284: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, eq40_e1283);
        let eq40_value: f64 = eq40_e1284;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq40_value),
            [5, 6, 7, 8],
            [multiplicity * ((eq40_e1283_d_n5 * ddt_scale)), multiplicity * ((eq40_e1283_d_n6 * ddt_scale)), multiplicity * ((eq40_e1283_d_n7 * ddt_scale)), multiplicity * ((eq40_e1283_d_n8 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq41_e1287: f64 = (var_chnl_type * var_mult_inst);
        let eq41_e1289: f64 = (eq41_e1287 * p.p33);
        let eq41_e1291: f64 = (eq41_e1289 * var_qfgs);
        let eq41_e1291_d_n5: f64 = (eq41_e1289 * var_qfgs_dn5);
        let eq41_e1291_d_n6: f64 = (eq41_e1289 * var_qfgs_dn6);
        let eq41_e1291_d_n7: f64 = (eq41_e1289 * var_qfgs_dn7);
        let eq41_e1292: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq41_e1291);
        let eq41_value: f64 = eq41_e1292;
        stamper.stamp_current_node3_local(
            Some(5),
            Some(6),
            multiplicity * (eq41_value),
            5,
            multiplicity * ((eq41_e1291_d_n5 * ddt_scale)),
            6,
            multiplicity * ((eq41_e1291_d_n6 * ddt_scale)),
            7,
            multiplicity * ((eq41_e1291_d_n7 * ddt_scale)),
        );
        let eq42_e1295: f64 = (var_chnl_type * var_mult_inst);
        let eq42_e1297: f64 = (eq42_e1295 * p.p33);
        let eq42_e1299: f64 = (eq42_e1297 * var_qfgd);
        let eq42_e1299_d_n5: f64 = (eq42_e1297 * var_qfgd_dn5);
        let eq42_e1299_d_n6: f64 = (eq42_e1297 * var_qfgd_dn6);
        let eq42_e1299_d_n7: f64 = (eq42_e1297 * var_qfgd_dn7);
        let eq42_e1300: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq42_e1299);
        let eq42_value: f64 = eq42_e1300;
        stamper.stamp_current_node3_local(
            Some(5),
            Some(7),
            multiplicity * (eq42_value),
            5,
            multiplicity * ((eq42_e1299_d_n5 * ddt_scale)),
            6,
            multiplicity * ((eq42_e1299_d_n6 * ddt_scale)),
            7,
            multiplicity * ((eq42_e1299_d_n7 * ddt_scale)),
        );
        let eq43_e1303: f64 = (var_chnl_type * var_mult_inst);
        let eq43_e1305: f64 = (eq43_e1303 * p.p33);
        let eq43_e1307: f64 = (eq43_e1305 * var_qgb_ov);
        let eq43_e1307_d_n5: f64 = (eq43_e1305 * var_qgb_ov_dn5);
        let eq43_e1307_d_n6: f64 = (eq43_e1305 * var_qgb_ov_dn6);
        let eq43_e1307_d_n7: f64 = (eq43_e1305 * var_qgb_ov_dn7);
        let eq43_e1307_d_n8: f64 = (eq43_e1305 * var_qgb_ov_dn8);
        let eq43_e1308: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq43_e1307);
        let eq43_value: f64 = eq43_e1308;
        stamper.stamp_current_sparse_local::<4, 0>(
            Some(5),
            Some(8),
            multiplicity * (eq43_value),
            [5, 6, 7, 8],
            [multiplicity * ((eq43_e1307_d_n5 * ddt_scale)), multiplicity * ((eq43_e1307_d_n6 * ddt_scale)), multiplicity * ((eq43_e1307_d_n7 * ddt_scale)), multiplicity * ((eq43_e1307_d_n8 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq44_e1311: f64 = (var_chnl_type * var_mult_inst);
        let eq44_e1313: f64 = (eq44_e1311 * p.p33);
        let eq44_e1315: f64 = (eq44_e1313 * var_qjun_s);
        let eq44_e1315_d_n5: f64 = (eq44_e1313 * var_qjun_s_dn5);
        let eq44_e1315_d_n6: f64 = (eq44_e1313 * var_qjun_s_dn6);
        let eq44_e1315_d_n7: f64 = (eq44_e1313 * var_qjun_s_dn7);
        let eq44_e1315_d_n8: f64 = (eq44_e1313 * var_qjun_s_dn8);
        let eq44_e1315_d_n10: f64 = (eq44_e1313 * var_qjun_s_dn10);
        let eq44_e1315_d_n11: f64 = (eq44_e1313 * var_qjun_s_dn11);
        let eq44_e1316: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, eq44_e1315);
        let eq44_value: f64 = eq44_e1316;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(10),
            Some(6),
            multiplicity * (eq44_value),
            [5, 6, 7, 8, 10, 11],
            [multiplicity * ((eq44_e1315_d_n5 * ddt_scale)), multiplicity * ((eq44_e1315_d_n6 * ddt_scale)), multiplicity * ((eq44_e1315_d_n7 * ddt_scale)), multiplicity * ((eq44_e1315_d_n8 * ddt_scale)), multiplicity * ((eq44_e1315_d_n10 * ddt_scale)), multiplicity * ((eq44_e1315_d_n11 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq45_e1319: f64 = (var_chnl_type * var_mult_inst);
        let eq45_e1321: f64 = (eq45_e1319 * p.p33);
        let eq45_e1323: f64 = (eq45_e1321 * var_qjun_d);
        let eq45_e1323_d_n5: f64 = (eq45_e1321 * var_qjun_d_dn5);
        let eq45_e1323_d_n6: f64 = (eq45_e1321 * var_qjun_d_dn6);
        let eq45_e1323_d_n7: f64 = (eq45_e1321 * var_qjun_d_dn7);
        let eq45_e1323_d_n8: f64 = (eq45_e1321 * var_qjun_d_dn8);
        let eq45_e1323_d_n10: f64 = (eq45_e1321 * var_qjun_d_dn10);
        let eq45_e1323_d_n11: f64 = (eq45_e1321 * var_qjun_d_dn11);
        let eq45_e1324: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, eq45_e1323);
        let eq45_value: f64 = eq45_e1324;
        stamper.stamp_current_sparse_local::<6, 0>(
            Some(11),
            Some(7),
            multiplicity * (eq45_value),
            [5, 6, 7, 8, 10, 11],
            [multiplicity * ((eq45_e1323_d_n5 * ddt_scale)), multiplicity * ((eq45_e1323_d_n6 * ddt_scale)), multiplicity * ((eq45_e1323_d_n7 * ddt_scale)), multiplicity * ((eq45_e1323_d_n8 * ddt_scale)), multiplicity * ((eq45_e1323_d_n10 * ddt_scale)), multiplicity * ((eq45_e1323_d_n11 * ddt_scale))],
            [],
            [],
            1.0,
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
        var_cgeff: f64,
        var_cgeff_dn5: f64,
        var_cgeff_dn6: f64,
        var_cgeff_dn7: f64,
        var_cgeff_dn8: f64,
        var_mig: f64,
        var_mig_dn5: f64,
        var_mig_dn6: f64,
        var_mig_dn7: f64,
        var_mig_dn8: f64,
        var_mult_inst: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let __rspice_inv_cse_0: f64 = 1.0 / var_mig;
        let eq47_e1332: f64 = ((nv4 - 0.0) * __rspice_inv_cse_0);
        let eq47_e1332_d_n4: f64 = (1.0 * __rspice_inv_cse_0);
        let eq47_e1332_d_n5: f64 = (-(((nv4 - 0.0) * var_mig_dn5) / (var_mig * var_mig)));
        let eq47_e1332_d_n6: f64 = (-(((nv4 - 0.0) * var_mig_dn6) / (var_mig * var_mig)));
        let eq47_e1332_d_n7: f64 = (-(((nv4 - 0.0) * var_mig_dn7) / (var_mig * var_mig)));
        let eq47_e1332_d_n8: f64 = (-(((nv4 - 0.0) * var_mig_dn8) / (var_mig * var_mig)));
        let eq47_value: f64 = eq47_e1332;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            None,
            multiplicity * (eq47_value),
            [4, 5, 6, 7, 8],
            [multiplicity * (eq47_e1332_d_n4), multiplicity * (eq47_e1332_d_n5), multiplicity * (eq47_e1332_d_n6), multiplicity * (eq47_e1332_d_n7), multiplicity * (eq47_e1332_d_n8)],
            [],
            [],
            1.0,
        );
        let eq48_e1335: f64 = (var_cgeff * (nv4 - 0.0));
        let eq48_e1335_d_n5: f64 = (var_cgeff_dn5 * (nv4 - 0.0));
        let eq48_e1335_d_n6: f64 = (var_cgeff_dn6 * (nv4 - 0.0));
        let eq48_e1335_d_n7: f64 = (var_cgeff_dn7 * (nv4 - 0.0));
        let eq48_e1335_d_n8: f64 = (var_cgeff_dn8 * (nv4 - 0.0));
        let eq48_e1336: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq48_e1335);
        let eq48_value: f64 = eq48_e1336;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(4),
            None,
            multiplicity * (eq48_value),
            [4, 5, 6, 7, 8],
            [multiplicity * ((var_cgeff * ddt_scale)), multiplicity * ((eq48_e1335_d_n5 * ddt_scale)), multiplicity * ((eq48_e1335_d_n6 * ddt_scale)), multiplicity * ((eq48_e1335_d_n7 * ddt_scale)), multiplicity * ((eq48_e1335_d_n8 * ddt_scale))],
            [],
            [],
            1.0,
        );
        let eq49_e1339: f64 = (var_mult_inst * p.p32);
        let eq49_e1340: f64 = (eq49_e1339).sqrt();
        let eq49_e1342: f64 = (eq49_e1340 * 0.5);
        let eq49_e1344: f64 = (eq49_e1342 * var_cgeff);
        let eq49_e1344_d_n5: f64 = (eq49_e1342 * var_cgeff_dn5);
        let eq49_e1344_d_n6: f64 = (eq49_e1342 * var_cgeff_dn6);
        let eq49_e1344_d_n7: f64 = (eq49_e1342 * var_cgeff_dn7);
        let eq49_e1344_d_n8: f64 = (eq49_e1342 * var_cgeff_dn8);
        let eq49_e1346: f64 = (eq49_e1344 * (nv4 - 0.0));
        let eq49_e1346_d_n5: f64 = (eq49_e1344_d_n5 * (nv4 - 0.0));
        let eq49_e1346_d_n6: f64 = (eq49_e1344_d_n6 * (nv4 - 0.0));
        let eq49_e1346_d_n7: f64 = (eq49_e1344_d_n7 * (nv4 - 0.0));
        let eq49_e1346_d_n8: f64 = (eq49_e1344_d_n8 * (nv4 - 0.0));
        let eq49_e1347: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq49_e1346);
        let eq49_e1348: f64 = (-eq49_e1347);
        let eq49_e1348_d_n4: f64 = (-(eq49_e1344 * ddt_scale));
        let eq49_e1348_d_n5: f64 = (-(eq49_e1346_d_n5 * ddt_scale));
        let eq49_e1348_d_n6: f64 = (-(eq49_e1346_d_n6 * ddt_scale));
        let eq49_e1348_d_n7: f64 = (-(eq49_e1346_d_n7 * ddt_scale));
        let eq49_e1348_d_n8: f64 = (-(eq49_e1346_d_n8 * ddt_scale));
        let eq49_value: f64 = eq49_e1348;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(6),
            multiplicity * (eq49_value),
            [4, 5, 6, 7, 8],
            [multiplicity * (eq49_e1348_d_n4), multiplicity * (eq49_e1348_d_n5), multiplicity * (eq49_e1348_d_n6), multiplicity * (eq49_e1348_d_n7), multiplicity * (eq49_e1348_d_n8)],
            [],
            [],
            1.0,
        );
        let eq50_e1351: f64 = (var_mult_inst * p.p32);
        let eq50_e1352: f64 = (eq50_e1351).sqrt();
        let eq50_e1354: f64 = (eq50_e1352 * 0.5);
        let eq50_e1356: f64 = (eq50_e1354 * var_cgeff);
        let eq50_e1356_d_n5: f64 = (eq50_e1354 * var_cgeff_dn5);
        let eq50_e1356_d_n6: f64 = (eq50_e1354 * var_cgeff_dn6);
        let eq50_e1356_d_n7: f64 = (eq50_e1354 * var_cgeff_dn7);
        let eq50_e1356_d_n8: f64 = (eq50_e1354 * var_cgeff_dn8);
        let eq50_e1358: f64 = (eq50_e1356 * (nv4 - 0.0));
        let eq50_e1358_d_n5: f64 = (eq50_e1356_d_n5 * (nv4 - 0.0));
        let eq50_e1358_d_n6: f64 = (eq50_e1356_d_n6 * (nv4 - 0.0));
        let eq50_e1358_d_n7: f64 = (eq50_e1356_d_n7 * (nv4 - 0.0));
        let eq50_e1358_d_n8: f64 = (eq50_e1356_d_n8 * (nv4 - 0.0));
        let eq50_e1359: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq50_e1358);
        let eq50_e1360: f64 = (-eq50_e1359);
        let eq50_e1360_d_n4: f64 = (-(eq50_e1356 * ddt_scale));
        let eq50_e1360_d_n5: f64 = (-(eq50_e1358_d_n5 * ddt_scale));
        let eq50_e1360_d_n6: f64 = (-(eq50_e1358_d_n6 * ddt_scale));
        let eq50_e1360_d_n7: f64 = (-(eq50_e1358_d_n7 * ddt_scale));
        let eq50_e1360_d_n8: f64 = (-(eq50_e1358_d_n8 * ddt_scale));
        let eq50_value: f64 = eq50_e1360;
        stamper.stamp_current_sparse_local::<5, 0>(
            Some(5),
            Some(7),
            multiplicity * (eq50_value),
            [4, 5, 6, 7, 8],
            [multiplicity * (eq50_e1360_d_n4), multiplicity * (eq50_e1360_d_n5), multiplicity * (eq50_e1360_d_n6), multiplicity * (eq50_e1360_d_n7), multiplicity * (eq50_e1360_d_n8)],
            [],
            [],
            1.0,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
        var_cgeff: f64,
        var_cgeff_dn5: f64,
        var_cgeff_dn6: f64,
        var_cgeff_dn7: f64,
        var_cgeff_dn8: f64,
        var_chnl_type: f64,
        var_mult_inst: f64,
        var_qb: f64,
        var_qb_dn5: f64,
        var_qb_dn6: f64,
        var_qb_dn7: f64,
        var_qb_dn8: f64,
        var_qd: f64,
        var_qd_dn5: f64,
        var_qd_dn6: f64,
        var_qd_dn7: f64,
        var_qd_dn8: f64,
        var_qfgd: f64,
        var_qfgd_dn5: f64,
        var_qfgd_dn6: f64,
        var_qfgd_dn7: f64,
        var_qfgs: f64,
        var_qfgs_dn5: f64,
        var_qfgs_dn6: f64,
        var_qfgs_dn7: f64,
        var_qg: f64,
        var_qg_dn5: f64,
        var_qg_dn6: f64,
        var_qg_dn7: f64,
        var_qg_dn8: f64,
        var_qgb_ov: f64,
        var_qgb_ov_dn5: f64,
        var_qgb_ov_dn6: f64,
        var_qgb_ov_dn7: f64,
        var_qgb_ov_dn8: f64,
        var_qjun_d: f64,
        var_qjun_d_dn10: f64,
        var_qjun_d_dn11: f64,
        var_qjun_d_dn5: f64,
        var_qjun_d_dn6: f64,
        var_qjun_d_dn7: f64,
        var_qjun_d_dn8: f64,
        var_qjun_s: f64,
        var_qjun_s_dn10: f64,
        var_qjun_s_dn11: f64,
        var_qjun_s_dn5: f64,
        var_qjun_s_dn6: f64,
        var_qjun_s_dn7: f64,
        var_qjun_s_dn8: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq38_e1263: f64 = (var_chnl_type * var_mult_inst);
        let eq38_e1265: f64 = (eq38_e1263 * p.p33);
        let eq38_e1267: f64 = (eq38_e1265 * var_qg);
        let eq38_e1267_d_n5: f64 = (eq38_e1265 * var_qg_dn5);
        let eq38_e1267_d_n6: f64 = (eq38_e1265 * var_qg_dn6);
        let eq38_e1267_d_n7: f64 = (eq38_e1265 * var_qg_dn7);
        let eq38_e1267_d_n8: f64 = (eq38_e1265 * var_qg_dn8);
        let eq38_e1268_q: f64 = eq38_e1267;
        stamper.stamp_current_reactive(
            Some(nodes[5]),
            Some(nodes[6]),
            &[
                GeneratedDerivative::node(nodes[5], multiplicity * (eq38_e1267_d_n5)),
                GeneratedDerivative::node(nodes[6], multiplicity * (eq38_e1267_d_n6)),
                GeneratedDerivative::node(nodes[7], multiplicity * (eq38_e1267_d_n7)),
                GeneratedDerivative::node(nodes[8], multiplicity * (eq38_e1267_d_n8)),
            ],
        );
        let eq39_e1271: f64 = (var_chnl_type * var_mult_inst);
        let eq39_e1273: f64 = (eq39_e1271 * p.p33);
        let eq39_e1275: f64 = (eq39_e1273 * var_qb);
        let eq39_e1275_d_n5: f64 = (eq39_e1273 * var_qb_dn5);
        let eq39_e1275_d_n6: f64 = (eq39_e1273 * var_qb_dn6);
        let eq39_e1275_d_n7: f64 = (eq39_e1273 * var_qb_dn7);
        let eq39_e1275_d_n8: f64 = (eq39_e1273 * var_qb_dn8);
        let eq39_e1276_q: f64 = eq39_e1275;
        stamper.stamp_current_reactive(
            Some(nodes[8]),
            Some(nodes[6]),
            &[
                GeneratedDerivative::node(nodes[5], multiplicity * (eq39_e1275_d_n5)),
                GeneratedDerivative::node(nodes[6], multiplicity * (eq39_e1275_d_n6)),
                GeneratedDerivative::node(nodes[7], multiplicity * (eq39_e1275_d_n7)),
                GeneratedDerivative::node(nodes[8], multiplicity * (eq39_e1275_d_n8)),
            ],
        );
        let eq40_e1279: f64 = (var_chnl_type * var_mult_inst);
        let eq40_e1281: f64 = (eq40_e1279 * p.p33);
        let eq40_e1283: f64 = (eq40_e1281 * var_qd);
        let eq40_e1283_d_n5: f64 = (eq40_e1281 * var_qd_dn5);
        let eq40_e1283_d_n6: f64 = (eq40_e1281 * var_qd_dn6);
        let eq40_e1283_d_n7: f64 = (eq40_e1281 * var_qd_dn7);
        let eq40_e1283_d_n8: f64 = (eq40_e1281 * var_qd_dn8);
        let eq40_e1284_q: f64 = eq40_e1283;
        stamper.stamp_current_reactive(
            Some(nodes[7]),
            Some(nodes[6]),
            &[
                GeneratedDerivative::node(nodes[5], multiplicity * (eq40_e1283_d_n5)),
                GeneratedDerivative::node(nodes[6], multiplicity * (eq40_e1283_d_n6)),
                GeneratedDerivative::node(nodes[7], multiplicity * (eq40_e1283_d_n7)),
                GeneratedDerivative::node(nodes[8], multiplicity * (eq40_e1283_d_n8)),
            ],
        );
        let eq41_e1287: f64 = (var_chnl_type * var_mult_inst);
        let eq41_e1289: f64 = (eq41_e1287 * p.p33);
        let eq41_e1291: f64 = (eq41_e1289 * var_qfgs);
        let eq41_e1291_d_n5: f64 = (eq41_e1289 * var_qfgs_dn5);
        let eq41_e1291_d_n6: f64 = (eq41_e1289 * var_qfgs_dn6);
        let eq41_e1291_d_n7: f64 = (eq41_e1289 * var_qfgs_dn7);
        let eq41_e1292_q: f64 = eq41_e1291;
        stamper.stamp_current_reactive_node3(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes[5],
            multiplicity * (eq41_e1291_d_n5),
            nodes[6],
            multiplicity * (eq41_e1291_d_n6),
            nodes[7],
            multiplicity * (eq41_e1291_d_n7),
        );
        let eq42_e1295: f64 = (var_chnl_type * var_mult_inst);
        let eq42_e1297: f64 = (eq42_e1295 * p.p33);
        let eq42_e1299: f64 = (eq42_e1297 * var_qfgd);
        let eq42_e1299_d_n5: f64 = (eq42_e1297 * var_qfgd_dn5);
        let eq42_e1299_d_n6: f64 = (eq42_e1297 * var_qfgd_dn6);
        let eq42_e1299_d_n7: f64 = (eq42_e1297 * var_qfgd_dn7);
        let eq42_e1300_q: f64 = eq42_e1299;
        stamper.stamp_current_reactive_node3(
            Some(nodes[5]),
            Some(nodes[7]),
            nodes[5],
            multiplicity * (eq42_e1299_d_n5),
            nodes[6],
            multiplicity * (eq42_e1299_d_n6),
            nodes[7],
            multiplicity * (eq42_e1299_d_n7),
        );
        let eq43_e1303: f64 = (var_chnl_type * var_mult_inst);
        let eq43_e1305: f64 = (eq43_e1303 * p.p33);
        let eq43_e1307: f64 = (eq43_e1305 * var_qgb_ov);
        let eq43_e1307_d_n5: f64 = (eq43_e1305 * var_qgb_ov_dn5);
        let eq43_e1307_d_n6: f64 = (eq43_e1305 * var_qgb_ov_dn6);
        let eq43_e1307_d_n7: f64 = (eq43_e1305 * var_qgb_ov_dn7);
        let eq43_e1307_d_n8: f64 = (eq43_e1305 * var_qgb_ov_dn8);
        let eq43_e1308_q: f64 = eq43_e1307;
        stamper.stamp_current_reactive(
            Some(nodes[5]),
            Some(nodes[8]),
            &[
                GeneratedDerivative::node(nodes[5], multiplicity * (eq43_e1307_d_n5)),
                GeneratedDerivative::node(nodes[6], multiplicity * (eq43_e1307_d_n6)),
                GeneratedDerivative::node(nodes[7], multiplicity * (eq43_e1307_d_n7)),
                GeneratedDerivative::node(nodes[8], multiplicity * (eq43_e1307_d_n8)),
            ],
        );
        let eq44_e1311: f64 = (var_chnl_type * var_mult_inst);
        let eq44_e1313: f64 = (eq44_e1311 * p.p33);
        let eq44_e1315: f64 = (eq44_e1313 * var_qjun_s);
        let eq44_e1315_d_n5: f64 = (eq44_e1313 * var_qjun_s_dn5);
        let eq44_e1315_d_n6: f64 = (eq44_e1313 * var_qjun_s_dn6);
        let eq44_e1315_d_n7: f64 = (eq44_e1313 * var_qjun_s_dn7);
        let eq44_e1315_d_n8: f64 = (eq44_e1313 * var_qjun_s_dn8);
        let eq44_e1315_d_n10: f64 = (eq44_e1313 * var_qjun_s_dn10);
        let eq44_e1315_d_n11: f64 = (eq44_e1313 * var_qjun_s_dn11);
        let eq44_e1316_q: f64 = eq44_e1315;
        let eq44_reactive_node_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, eq44_e1315_d_n5, eq44_e1315_d_n6, eq44_e1315_d_n7, eq44_e1315_d_n8, 0.0, eq44_e1315_d_n10, eq44_e1315_d_n11];
        let eq44_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            nodes,
            &eq44_reactive_node_derivatives,
            branches,
            &eq44_reactive_branch_derivatives,
            multiplicity,
        );
        let eq45_e1319: f64 = (var_chnl_type * var_mult_inst);
        let eq45_e1321: f64 = (eq45_e1319 * p.p33);
        let eq45_e1323: f64 = (eq45_e1321 * var_qjun_d);
        let eq45_e1323_d_n5: f64 = (eq45_e1321 * var_qjun_d_dn5);
        let eq45_e1323_d_n6: f64 = (eq45_e1321 * var_qjun_d_dn6);
        let eq45_e1323_d_n7: f64 = (eq45_e1321 * var_qjun_d_dn7);
        let eq45_e1323_d_n8: f64 = (eq45_e1321 * var_qjun_d_dn8);
        let eq45_e1323_d_n10: f64 = (eq45_e1321 * var_qjun_d_dn10);
        let eq45_e1323_d_n11: f64 = (eq45_e1321 * var_qjun_d_dn11);
        let eq45_e1324_q: f64 = eq45_e1323;
        let eq45_reactive_node_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, 0.0, eq45_e1323_d_n5, eq45_e1323_d_n6, eq45_e1323_d_n7, eq45_e1323_d_n8, 0.0, eq45_e1323_d_n10, eq45_e1323_d_n11];
        let eq45_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq45_reactive_node_derivatives,
            branches,
            &eq45_reactive_branch_derivatives,
            multiplicity,
        );
        let eq48_e1335: f64 = (var_cgeff * (nv4 - 0.0));
        let eq48_e1335_d_n5: f64 = (var_cgeff_dn5 * (nv4 - 0.0));
        let eq48_e1335_d_n6: f64 = (var_cgeff_dn6 * (nv4 - 0.0));
        let eq48_e1335_d_n7: f64 = (var_cgeff_dn7 * (nv4 - 0.0));
        let eq48_e1335_d_n8: f64 = (var_cgeff_dn8 * (nv4 - 0.0));
        let eq48_e1336_q: f64 = eq48_e1335;
        let eq48_reactive_node_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, var_cgeff, eq48_e1335_d_n5, eq48_e1335_d_n6, eq48_e1335_d_n7, eq48_e1335_d_n8, 0.0, 0.0, 0.0];
        let eq48_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq48_reactive_node_derivatives,
            branches,
            &eq48_reactive_branch_derivatives,
            multiplicity,
        );
        let eq49_e1339: f64 = (var_mult_inst * p.p32);
        let eq49_e1340: f64 = (eq49_e1339).sqrt();
        let eq49_e1342: f64 = (eq49_e1340 * 0.5);
        let eq49_e1344: f64 = (eq49_e1342 * var_cgeff);
        let eq49_e1344_d_n5: f64 = (eq49_e1342 * var_cgeff_dn5);
        let eq49_e1344_d_n6: f64 = (eq49_e1342 * var_cgeff_dn6);
        let eq49_e1344_d_n7: f64 = (eq49_e1342 * var_cgeff_dn7);
        let eq49_e1344_d_n8: f64 = (eq49_e1342 * var_cgeff_dn8);
        let eq49_e1346: f64 = (eq49_e1344 * (nv4 - 0.0));
        let eq49_e1346_d_n5: f64 = (eq49_e1344_d_n5 * (nv4 - 0.0));
        let eq49_e1346_d_n6: f64 = (eq49_e1344_d_n6 * (nv4 - 0.0));
        let eq49_e1346_d_n7: f64 = (eq49_e1344_d_n7 * (nv4 - 0.0));
        let eq49_e1346_d_n8: f64 = (eq49_e1344_d_n8 * (nv4 - 0.0));
        let eq49_e1347_q: f64 = eq49_e1346;
        let eq49_e1348: f64 = (-eq49_e1346);
        let eq49_e1348_q: f64 = (-eq49_e1347_q);
        let eq49_reactive_node_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, (-eq49_e1344), (-eq49_e1346_d_n5), (-eq49_e1346_d_n6), (-eq49_e1346_d_n7), (-eq49_e1346_d_n8), 0.0, 0.0, 0.0];
        let eq49_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq49_reactive_node_derivatives,
            branches,
            &eq49_reactive_branch_derivatives,
            multiplicity,
        );
        let eq50_e1351: f64 = (var_mult_inst * p.p32);
        let eq50_e1352: f64 = (eq50_e1351).sqrt();
        let eq50_e1354: f64 = (eq50_e1352 * 0.5);
        let eq50_e1356: f64 = (eq50_e1354 * var_cgeff);
        let eq50_e1356_d_n5: f64 = (eq50_e1354 * var_cgeff_dn5);
        let eq50_e1356_d_n6: f64 = (eq50_e1354 * var_cgeff_dn6);
        let eq50_e1356_d_n7: f64 = (eq50_e1354 * var_cgeff_dn7);
        let eq50_e1356_d_n8: f64 = (eq50_e1354 * var_cgeff_dn8);
        let eq50_e1358: f64 = (eq50_e1356 * (nv4 - 0.0));
        let eq50_e1358_d_n5: f64 = (eq50_e1356_d_n5 * (nv4 - 0.0));
        let eq50_e1358_d_n6: f64 = (eq50_e1356_d_n6 * (nv4 - 0.0));
        let eq50_e1358_d_n7: f64 = (eq50_e1356_d_n7 * (nv4 - 0.0));
        let eq50_e1358_d_n8: f64 = (eq50_e1356_d_n8 * (nv4 - 0.0));
        let eq50_e1359_q: f64 = eq50_e1358;
        let eq50_e1360: f64 = (-eq50_e1358);
        let eq50_e1360_q: f64 = (-eq50_e1359_q);
        let eq50_reactive_node_derivatives: [f64; 12] = [0.0, 0.0, 0.0, 0.0, (-eq50_e1356), (-eq50_e1358_d_n5), (-eq50_e1358_d_n6), (-eq50_e1358_d_n7), (-eq50_e1358_d_n8), 0.0, 0.0, 0.0];
        let eq50_reactive_branch_derivatives: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            nodes,
            &eq50_reactive_node_derivatives,
            branches,
            &eq50_reactive_branch_derivatives,
            multiplicity,
        );
    }
}

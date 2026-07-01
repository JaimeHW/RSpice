#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_64(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21920_e19788, assign21920_e19788_d_n3, assign21920_e19788_d_n4, assign21920_e19788_d_n5, assign21920_e19788_d_n6, assign21920_e19788_d_n7, assign21920_e19788_d_n8, assign21920_e19788_d_n9, assign21920_e19788_d_n10, assign21920_e19788_d_n11, assign21920_e19788_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1275 == 0.0)) && (locals.var_guard1277 == 0.0)) {
        let assign21920_e19784: f64 = (1.0 + locals.var_ehlid);
        let assign21920_e19785: f64 = (assign21920_e19784).sqrt();
        let assign21920_e19786: f64 = (1.0 / assign21920_e19785);
        (assign21920_e19786, (-((locals.var_ehlid_dn3 / (2.0 * assign21920_e19785)) / (assign21920_e19785 * assign21920_e19785))), (-((locals.var_ehlid_dn4 / (2.0 * assign21920_e19785)) / (assign21920_e19785 * assign21920_e19785))), (-((locals.var_ehlid_dn5 / (2.0 * assign21920_e19785)) / (assign21920_e19785 * assign21920_e19785))), (-((locals.var_ehlid_dn6 / (2.0 * assign21920_e19785)) / (assign21920_e19785 * assign21920_e19785))), (-((locals.var_ehlid_dn7 / (2.0 * assign21920_e19785)) / (assign21920_e19785 * assign21920_e19785))), (-((locals.var_ehlid_dn8 / (2.0 * assign21920_e19785)) / (assign21920_e19785 * assign21920_e19785))), (-((locals.var_ehlid_dn9 / (2.0 * assign21920_e19785)) / (assign21920_e19785 * assign21920_e19785))), (-((locals.var_ehlid_dn10 / (2.0 * assign21920_e19785)) / (assign21920_e19785 * assign21920_e19785))), (-((locals.var_ehlid_dn11 / (2.0 * assign21920_e19785)) / (assign21920_e19785 * assign21920_e19785))), (-((locals.var_ehlid_dn12 / (2.0 * assign21920_e19785)) / (assign21920_e19785 * assign21920_e19785))),)
    } else {
        (locals.var_ehlidfactor, locals.var_ehlidfactor_dn3, locals.var_ehlidfactor_dn4, locals.var_ehlidfactor_dn5, locals.var_ehlidfactor_dn6, locals.var_ehlidfactor_dn7, locals.var_ehlidfactor_dn8, locals.var_ehlidfactor_dn9, locals.var_ehlidfactor_dn10, locals.var_ehlidfactor_dn11, locals.var_ehlidfactor_dn12,)
    }
};
        locals.var_ehlidfactor = assign21920_e19788;
        locals.var_ehlidfactor_dn3 = assign21920_e19788_d_n3;
        locals.var_ehlidfactor_dn4 = assign21920_e19788_d_n4;
        locals.var_ehlidfactor_dn5 = assign21920_e19788_d_n5;
        locals.var_ehlidfactor_dn6 = assign21920_e19788_d_n6;
        locals.var_ehlidfactor_dn7 = assign21920_e19788_d_n7;
        locals.var_ehlidfactor_dn8 = assign21920_e19788_d_n8;
        locals.var_ehlidfactor_dn9 = assign21920_e19788_d_n9;
        locals.var_ehlidfactor_dn10 = assign21920_e19788_d_n10;
        locals.var_ehlidfactor_dn11 = assign21920_e19788_d_n11;
        locals.var_ehlidfactor_dn12 = assign21920_e19788_d_n12;

        let (assign21930_e19797, assign21930_e19797_d_n3, assign21930_e19797_d_n4, assign21930_e19797_d_n5, assign21930_e19797_d_n6, assign21930_e19797_d_n7, assign21930_e19797_d_n8, assign21930_e19797_d_n9, assign21930_e19797_d_n10, assign21930_e19797_d_n11, assign21930_e19797_d_n12,) = {
    if ((locals.var_guard1240 != 0.0) && (locals.var_guard1275 == 0.0)) {
        let assign21930_e19795: f64 = (1.0 - locals.var_pparam_b4soiarfabjt);
        (assign21930_e19795, (-locals.var_pparam_b4soiarfabjt_dn3), (-locals.var_pparam_b4soiarfabjt_dn4), (-locals.var_pparam_b4soiarfabjt_dn5), (-locals.var_pparam_b4soiarfabjt_dn6), (-locals.var_pparam_b4soiarfabjt_dn7), (-locals.var_pparam_b4soiarfabjt_dn8), (-locals.var_pparam_b4soiarfabjt_dn9), (-locals.var_pparam_b4soiarfabjt_dn10), (-locals.var_pparam_b4soiarfabjt_dn11), (-locals.var_pparam_b4soiarfabjt_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign21930_e19797;
        locals.var_t0__blk808_dn3 = assign21930_e19797_d_n3;
        locals.var_t0__blk808_dn4 = assign21930_e19797_d_n4;
        locals.var_t0__blk808_dn5 = assign21930_e19797_d_n5;
        locals.var_t0__blk808_dn6 = assign21930_e19797_d_n6;
        locals.var_t0__blk808_dn7 = assign21930_e19797_d_n7;
        locals.var_t0__blk808_dn8 = assign21930_e19797_d_n8;
        locals.var_t0__blk808_dn9 = assign21930_e19797_d_n9;
        locals.var_t0__blk808_dn10 = assign21930_e19797_d_n10;
        locals.var_t0__blk808_dn11 = assign21930_e19797_d_n11;
        locals.var_t0__blk808_dn12 = assign21930_e19797_d_n12;

        let (assign21940_e19808, assign21940_e19808_d_n3, assign21940_e19808_d_n4, assign21940_e19808_d_n5, assign21940_e19808_d_n6, assign21940_e19808_d_n7, assign21940_e19808_d_n8, assign21940_e19808_d_n9, assign21940_e19808_d_n10, assign21940_e19808_d_n11, assign21940_e19808_d_n12,) = {
    if ((locals.var_guard1240 != 0.0) && (locals.var_guard1275 == 0.0)) {
        let assign21940_e19804: f64 = (locals.var_wtsi * locals.var_jbjts);
        let assign21940_e19806: f64 = (assign21940_e19804 * locals.var_pparam_b4soilratio);
        (assign21940_e19806, ((((locals.var_wtsi_dn3 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn3)) * locals.var_pparam_b4soilratio) + (assign21940_e19804 * locals.var_pparam_b4soilratio_dn3)), ((((locals.var_wtsi_dn4 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn4)) * locals.var_pparam_b4soilratio) + (assign21940_e19804 * locals.var_pparam_b4soilratio_dn4)), ((((locals.var_wtsi_dn5 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn5)) * locals.var_pparam_b4soilratio) + (assign21940_e19804 * locals.var_pparam_b4soilratio_dn5)), ((((locals.var_wtsi_dn6 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn6)) * locals.var_pparam_b4soilratio) + (assign21940_e19804 * locals.var_pparam_b4soilratio_dn6)), ((((locals.var_wtsi_dn7 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn7)) * locals.var_pparam_b4soilratio) + (assign21940_e19804 * locals.var_pparam_b4soilratio_dn7)), ((((locals.var_wtsi_dn8 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn8)) * locals.var_pparam_b4soilratio) + (assign21940_e19804 * locals.var_pparam_b4soilratio_dn8)), ((((locals.var_wtsi_dn9 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn9)) * locals.var_pparam_b4soilratio) + (assign21940_e19804 * locals.var_pparam_b4soilratio_dn9)), ((((locals.var_wtsi_dn10 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn10)) * locals.var_pparam_b4soilratio) + (assign21940_e19804 * locals.var_pparam_b4soilratio_dn10)), ((((locals.var_wtsi_dn11 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn11)) * locals.var_pparam_b4soilratio) + (assign21940_e19804 * locals.var_pparam_b4soilratio_dn11)), ((((locals.var_wtsi_dn12 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn12)) * locals.var_pparam_b4soilratio) + (assign21940_e19804 * locals.var_pparam_b4soilratio_dn12)),)
    } else {
        (locals.var_ien, locals.var_ien_dn3, locals.var_ien_dn4, locals.var_ien_dn5, locals.var_ien_dn6, locals.var_ien_dn7, locals.var_ien_dn8, locals.var_ien_dn9, locals.var_ien_dn10, locals.var_ien_dn11, locals.var_ien_dn12,)
    }
};
        locals.var_ien = assign21940_e19808;
        locals.var_ien_dn3 = assign21940_e19808_d_n3;
        locals.var_ien_dn4 = assign21940_e19808_d_n4;
        locals.var_ien_dn5 = assign21940_e19808_d_n5;
        locals.var_ien_dn6 = assign21940_e19808_d_n6;
        locals.var_ien_dn7 = assign21940_e19808_d_n7;
        locals.var_ien_dn8 = assign21940_e19808_d_n8;
        locals.var_ien_dn9 = assign21940_e19808_d_n9;
        locals.var_ien_dn10 = assign21940_e19808_d_n10;
        locals.var_ien_dn11 = assign21940_e19808_d_n11;
        locals.var_ien_dn12 = assign21940_e19808_d_n12;

        let (assign21950_e19817, assign21950_e19817_d_n3, assign21950_e19817_d_n4, assign21950_e19817_d_n5, assign21950_e19817_d_n6, assign21950_e19817_d_n7, assign21950_e19817_d_n8, assign21950_e19817_d_n9, assign21950_e19817_d_n10, assign21950_e19817_d_n11, assign21950_e19817_d_n12,) = {
    if ((locals.var_guard1240 != 0.0) && (locals.var_guard1275 == 0.0)) {
        let assign21950_e19815: f64 = (locals.var_t0__blk808 * locals.var_ien);
        (assign21950_e19815, ((locals.var_t0__blk808_dn3 * locals.var_ien) + (locals.var_t0__blk808 * locals.var_ien_dn3)), ((locals.var_t0__blk808_dn4 * locals.var_ien) + (locals.var_t0__blk808 * locals.var_ien_dn4)), ((locals.var_t0__blk808_dn5 * locals.var_ien) + (locals.var_t0__blk808 * locals.var_ien_dn5)), ((locals.var_t0__blk808_dn6 * locals.var_ien) + (locals.var_t0__blk808 * locals.var_ien_dn6)), ((locals.var_t0__blk808_dn7 * locals.var_ien) + (locals.var_t0__blk808 * locals.var_ien_dn7)), ((locals.var_t0__blk808_dn8 * locals.var_ien) + (locals.var_t0__blk808 * locals.var_ien_dn8)), ((locals.var_t0__blk808_dn9 * locals.var_ien) + (locals.var_t0__blk808 * locals.var_ien_dn9)), ((locals.var_t0__blk808_dn10 * locals.var_ien) + (locals.var_t0__blk808 * locals.var_ien_dn10)), ((locals.var_t0__blk808_dn11 * locals.var_ien) + (locals.var_t0__blk808 * locals.var_ien_dn11)), ((locals.var_t0__blk808_dn12 * locals.var_ien) + (locals.var_t0__blk808 * locals.var_ien_dn12)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign21950_e19817;
        locals.var_t1__blk809_dn3 = assign21950_e19817_d_n3;
        locals.var_t1__blk809_dn4 = assign21950_e19817_d_n4;
        locals.var_t1__blk809_dn5 = assign21950_e19817_d_n5;
        locals.var_t1__blk809_dn6 = assign21950_e19817_d_n6;
        locals.var_t1__blk809_dn7 = assign21950_e19817_d_n7;
        locals.var_t1__blk809_dn8 = assign21950_e19817_d_n8;
        locals.var_t1__blk809_dn9 = assign21950_e19817_d_n9;
        locals.var_t1__blk809_dn10 = assign21950_e19817_d_n10;
        locals.var_t1__blk809_dn11 = assign21950_e19817_d_n11;
        locals.var_t1__blk809_dn12 = assign21950_e19817_d_n12;

        let (assign21960_e19830, assign21960_e19830_d_n3, assign21960_e19830_d_n4, assign21960_e19830_d_n5, assign21960_e19830_d_n6, assign21960_e19830_d_n7, assign21960_e19830_d_n8, assign21960_e19830_d_n9, assign21960_e19830_d_n10, assign21960_e19830_d_n11, assign21960_e19830_d_n12,) = {
    if ((locals.var_guard1240 != 0.0) && (locals.var_guard1275 == 0.0)) {
        let assign21960_e19825: f64 = (locals.var_expvbsnvtm - 1.0);
        let assign21960_e19826: f64 = (locals.var_t1__blk809 * assign21960_e19825);
        let assign21960_e19828: f64 = (assign21960_e19826 * locals.var_ehlisfactor);
        (assign21960_e19828, ((((locals.var_t1__blk809_dn3 * assign21960_e19825) + (locals.var_t1__blk809 * locals.var_expvbsnvtm_dn3)) * locals.var_ehlisfactor) + (assign21960_e19826 * locals.var_ehlisfactor_dn3)), ((((locals.var_t1__blk809_dn4 * assign21960_e19825) + (locals.var_t1__blk809 * locals.var_expvbsnvtm_dn4)) * locals.var_ehlisfactor) + (assign21960_e19826 * locals.var_ehlisfactor_dn4)), ((((locals.var_t1__blk809_dn5 * assign21960_e19825) + (locals.var_t1__blk809 * locals.var_expvbsnvtm_dn5)) * locals.var_ehlisfactor) + (assign21960_e19826 * locals.var_ehlisfactor_dn5)), ((((locals.var_t1__blk809_dn6 * assign21960_e19825) + (locals.var_t1__blk809 * locals.var_expvbsnvtm_dn6)) * locals.var_ehlisfactor) + (assign21960_e19826 * locals.var_ehlisfactor_dn6)), ((((locals.var_t1__blk809_dn7 * assign21960_e19825) + (locals.var_t1__blk809 * locals.var_expvbsnvtm_dn7)) * locals.var_ehlisfactor) + (assign21960_e19826 * locals.var_ehlisfactor_dn7)), ((((locals.var_t1__blk809_dn8 * assign21960_e19825) + (locals.var_t1__blk809 * locals.var_expvbsnvtm_dn8)) * locals.var_ehlisfactor) + (assign21960_e19826 * locals.var_ehlisfactor_dn8)), ((((locals.var_t1__blk809_dn9 * assign21960_e19825) + (locals.var_t1__blk809 * locals.var_expvbsnvtm_dn9)) * locals.var_ehlisfactor) + (assign21960_e19826 * locals.var_ehlisfactor_dn9)), ((((locals.var_t1__blk809_dn10 * assign21960_e19825) + (locals.var_t1__blk809 * locals.var_expvbsnvtm_dn10)) * locals.var_ehlisfactor) + (assign21960_e19826 * locals.var_ehlisfactor_dn10)), ((((locals.var_t1__blk809_dn11 * assign21960_e19825) + (locals.var_t1__blk809 * locals.var_expvbsnvtm_dn11)) * locals.var_ehlisfactor) + (assign21960_e19826 * locals.var_ehlisfactor_dn11)), ((((locals.var_t1__blk809_dn12 * assign21960_e19825) + (locals.var_t1__blk809 * locals.var_expvbsnvtm_dn12)) * locals.var_ehlisfactor) + (assign21960_e19826 * locals.var_ehlisfactor_dn12)),)
    } else {
        (locals.var_ibs3, locals.var_ibs3_dn3, locals.var_ibs3_dn4, locals.var_ibs3_dn5, locals.var_ibs3_dn6, locals.var_ibs3_dn7, locals.var_ibs3_dn8, locals.var_ibs3_dn9, locals.var_ibs3_dn10, locals.var_ibs3_dn11, locals.var_ibs3_dn12,)
    }
};
        locals.var_ibs3 = assign21960_e19830;
        locals.var_ibs3_dn3 = assign21960_e19830_d_n3;
        locals.var_ibs3_dn4 = assign21960_e19830_d_n4;
        locals.var_ibs3_dn5 = assign21960_e19830_d_n5;
        locals.var_ibs3_dn6 = assign21960_e19830_d_n6;
        locals.var_ibs3_dn7 = assign21960_e19830_d_n7;
        locals.var_ibs3_dn8 = assign21960_e19830_d_n8;
        locals.var_ibs3_dn9 = assign21960_e19830_d_n9;
        locals.var_ibs3_dn10 = assign21960_e19830_d_n10;
        locals.var_ibs3_dn11 = assign21960_e19830_d_n11;
        locals.var_ibs3_dn12 = assign21960_e19830_d_n12;

        let (assign21970_e19841, assign21970_e19841_d_n3, assign21970_e19841_d_n4, assign21970_e19841_d_n5, assign21970_e19841_d_n6, assign21970_e19841_d_n7, assign21970_e19841_d_n8, assign21970_e19841_d_n9, assign21970_e19841_d_n10, assign21970_e19841_d_n11, assign21970_e19841_d_n12,) = {
    if ((locals.var_guard1240 != 0.0) && (locals.var_guard1275 == 0.0)) {
        let assign21970_e19837: f64 = (locals.var_wtsi * locals.var_jbjtd);
        let assign21970_e19839: f64 = (assign21970_e19837 * locals.var_pparam_b4soilratio);
        (assign21970_e19839, ((((locals.var_wtsi_dn3 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn3)) * locals.var_pparam_b4soilratio) + (assign21970_e19837 * locals.var_pparam_b4soilratio_dn3)), ((((locals.var_wtsi_dn4 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn4)) * locals.var_pparam_b4soilratio) + (assign21970_e19837 * locals.var_pparam_b4soilratio_dn4)), ((((locals.var_wtsi_dn5 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn5)) * locals.var_pparam_b4soilratio) + (assign21970_e19837 * locals.var_pparam_b4soilratio_dn5)), ((((locals.var_wtsi_dn6 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn6)) * locals.var_pparam_b4soilratio) + (assign21970_e19837 * locals.var_pparam_b4soilratio_dn6)), ((((locals.var_wtsi_dn7 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn7)) * locals.var_pparam_b4soilratio) + (assign21970_e19837 * locals.var_pparam_b4soilratio_dn7)), ((((locals.var_wtsi_dn8 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn8)) * locals.var_pparam_b4soilratio) + (assign21970_e19837 * locals.var_pparam_b4soilratio_dn8)), ((((locals.var_wtsi_dn9 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn9)) * locals.var_pparam_b4soilratio) + (assign21970_e19837 * locals.var_pparam_b4soilratio_dn9)), ((((locals.var_wtsi_dn10 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn10)) * locals.var_pparam_b4soilratio) + (assign21970_e19837 * locals.var_pparam_b4soilratio_dn10)), ((((locals.var_wtsi_dn11 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn11)) * locals.var_pparam_b4soilratio) + (assign21970_e19837 * locals.var_pparam_b4soilratio_dn11)), ((((locals.var_wtsi_dn12 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn12)) * locals.var_pparam_b4soilratio) + (assign21970_e19837 * locals.var_pparam_b4soilratio_dn12)),)
    } else {
        (locals.var_ien, locals.var_ien_dn3, locals.var_ien_dn4, locals.var_ien_dn5, locals.var_ien_dn6, locals.var_ien_dn7, locals.var_ien_dn8, locals.var_ien_dn9, locals.var_ien_dn10, locals.var_ien_dn11, locals.var_ien_dn12,)
    }
};
        locals.var_ien = assign21970_e19841;
        locals.var_ien_dn3 = assign21970_e19841_d_n3;
        locals.var_ien_dn4 = assign21970_e19841_d_n4;
        locals.var_ien_dn5 = assign21970_e19841_d_n5;
        locals.var_ien_dn6 = assign21970_e19841_d_n6;
        locals.var_ien_dn7 = assign21970_e19841_d_n7;
        locals.var_ien_dn8 = assign21970_e19841_d_n8;
        locals.var_ien_dn9 = assign21970_e19841_d_n9;
        locals.var_ien_dn10 = assign21970_e19841_d_n10;
        locals.var_ien_dn11 = assign21970_e19841_d_n11;
        locals.var_ien_dn12 = assign21970_e19841_d_n12;

        let (assign21980_e19850, assign21980_e19850_d_n3, assign21980_e19850_d_n4, assign21980_e19850_d_n5, assign21980_e19850_d_n6, assign21980_e19850_d_n7, assign21980_e19850_d_n8, assign21980_e19850_d_n9, assign21980_e19850_d_n10, assign21980_e19850_d_n11, assign21980_e19850_d_n12,) = {
    if ((locals.var_guard1240 != 0.0) && (locals.var_guard1275 == 0.0)) {
        let assign21980_e19848: f64 = (locals.var_t0__blk808 * locals.var_ien);
        (assign21980_e19848, ((locals.var_t0__blk808_dn3 * locals.var_ien) + (locals.var_t0__blk808 * locals.var_ien_dn3)), ((locals.var_t0__blk808_dn4 * locals.var_ien) + (locals.var_t0__blk808 * locals.var_ien_dn4)), ((locals.var_t0__blk808_dn5 * locals.var_ien) + (locals.var_t0__blk808 * locals.var_ien_dn5)), ((locals.var_t0__blk808_dn6 * locals.var_ien) + (locals.var_t0__blk808 * locals.var_ien_dn6)), ((locals.var_t0__blk808_dn7 * locals.var_ien) + (locals.var_t0__blk808 * locals.var_ien_dn7)), ((locals.var_t0__blk808_dn8 * locals.var_ien) + (locals.var_t0__blk808 * locals.var_ien_dn8)), ((locals.var_t0__blk808_dn9 * locals.var_ien) + (locals.var_t0__blk808 * locals.var_ien_dn9)), ((locals.var_t0__blk808_dn10 * locals.var_ien) + (locals.var_t0__blk808 * locals.var_ien_dn10)), ((locals.var_t0__blk808_dn11 * locals.var_ien) + (locals.var_t0__blk808 * locals.var_ien_dn11)), ((locals.var_t0__blk808_dn12 * locals.var_ien) + (locals.var_t0__blk808 * locals.var_ien_dn12)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign21980_e19850;
        locals.var_t1__blk809_dn3 = assign21980_e19850_d_n3;
        locals.var_t1__blk809_dn4 = assign21980_e19850_d_n4;
        locals.var_t1__blk809_dn5 = assign21980_e19850_d_n5;
        locals.var_t1__blk809_dn6 = assign21980_e19850_d_n6;
        locals.var_t1__blk809_dn7 = assign21980_e19850_d_n7;
        locals.var_t1__blk809_dn8 = assign21980_e19850_d_n8;
        locals.var_t1__blk809_dn9 = assign21980_e19850_d_n9;
        locals.var_t1__blk809_dn10 = assign21980_e19850_d_n10;
        locals.var_t1__blk809_dn11 = assign21980_e19850_d_n11;
        locals.var_t1__blk809_dn12 = assign21980_e19850_d_n12;

        let (assign21990_e19863, assign21990_e19863_d_n3, assign21990_e19863_d_n4, assign21990_e19863_d_n5, assign21990_e19863_d_n6, assign21990_e19863_d_n7, assign21990_e19863_d_n8, assign21990_e19863_d_n9, assign21990_e19863_d_n10, assign21990_e19863_d_n11, assign21990_e19863_d_n12,) = {
    if ((locals.var_guard1240 != 0.0) && (locals.var_guard1275 == 0.0)) {
        let assign21990_e19858: f64 = (locals.var_expvbdnvtm - 1.0);
        let assign21990_e19859: f64 = (locals.var_t1__blk809 * assign21990_e19858);
        let assign21990_e19861: f64 = (assign21990_e19859 * locals.var_ehlidfactor);
        (assign21990_e19861, ((((locals.var_t1__blk809_dn3 * assign21990_e19858) + (locals.var_t1__blk809 * locals.var_expvbdnvtm_dn3)) * locals.var_ehlidfactor) + (assign21990_e19859 * locals.var_ehlidfactor_dn3)), ((((locals.var_t1__blk809_dn4 * assign21990_e19858) + (locals.var_t1__blk809 * locals.var_expvbdnvtm_dn4)) * locals.var_ehlidfactor) + (assign21990_e19859 * locals.var_ehlidfactor_dn4)), ((((locals.var_t1__blk809_dn5 * assign21990_e19858) + (locals.var_t1__blk809 * locals.var_expvbdnvtm_dn5)) * locals.var_ehlidfactor) + (assign21990_e19859 * locals.var_ehlidfactor_dn5)), ((((locals.var_t1__blk809_dn6 * assign21990_e19858) + (locals.var_t1__blk809 * locals.var_expvbdnvtm_dn6)) * locals.var_ehlidfactor) + (assign21990_e19859 * locals.var_ehlidfactor_dn6)), ((((locals.var_t1__blk809_dn7 * assign21990_e19858) + (locals.var_t1__blk809 * locals.var_expvbdnvtm_dn7)) * locals.var_ehlidfactor) + (assign21990_e19859 * locals.var_ehlidfactor_dn7)), ((((locals.var_t1__blk809_dn8 * assign21990_e19858) + (locals.var_t1__blk809 * locals.var_expvbdnvtm_dn8)) * locals.var_ehlidfactor) + (assign21990_e19859 * locals.var_ehlidfactor_dn8)), ((((locals.var_t1__blk809_dn9 * assign21990_e19858) + (locals.var_t1__blk809 * locals.var_expvbdnvtm_dn9)) * locals.var_ehlidfactor) + (assign21990_e19859 * locals.var_ehlidfactor_dn9)), ((((locals.var_t1__blk809_dn10 * assign21990_e19858) + (locals.var_t1__blk809 * locals.var_expvbdnvtm_dn10)) * locals.var_ehlidfactor) + (assign21990_e19859 * locals.var_ehlidfactor_dn10)), ((((locals.var_t1__blk809_dn11 * assign21990_e19858) + (locals.var_t1__blk809 * locals.var_expvbdnvtm_dn11)) * locals.var_ehlidfactor) + (assign21990_e19859 * locals.var_ehlidfactor_dn11)), ((((locals.var_t1__blk809_dn12 * assign21990_e19858) + (locals.var_t1__blk809 * locals.var_expvbdnvtm_dn12)) * locals.var_ehlidfactor) + (assign21990_e19859 * locals.var_ehlidfactor_dn12)),)
    } else {
        (locals.var_ibd3, locals.var_ibd3_dn3, locals.var_ibd3_dn4, locals.var_ibd3_dn5, locals.var_ibd3_dn6, locals.var_ibd3_dn7, locals.var_ibd3_dn8, locals.var_ibd3_dn9, locals.var_ibd3_dn10, locals.var_ibd3_dn11, locals.var_ibd3_dn12,)
    }
};
        locals.var_ibd3 = assign21990_e19863;
        locals.var_ibd3_dn3 = assign21990_e19863_d_n3;
        locals.var_ibd3_dn4 = assign21990_e19863_d_n4;
        locals.var_ibd3_dn5 = assign21990_e19863_d_n5;
        locals.var_ibd3_dn6 = assign21990_e19863_d_n6;
        locals.var_ibd3_dn7 = assign21990_e19863_d_n7;
        locals.var_ibd3_dn8 = assign21990_e19863_d_n8;
        locals.var_ibd3_dn9 = assign21990_e19863_d_n9;
        locals.var_ibd3_dn10 = assign21990_e19863_d_n10;
        locals.var_ibd3_dn11 = assign21990_e19863_d_n11;
        locals.var_ibd3_dn12 = assign21990_e19863_d_n12;

        let (assign22000_e19874, assign22000_e19874_d_n3, assign22000_e19874_d_n4, assign22000_e19874_d_n5, assign22000_e19874_d_n6, assign22000_e19874_d_n7, assign22000_e19874_d_n8, assign22000_e19874_d_n9, assign22000_e19874_d_n10, assign22000_e19874_d_n11, assign22000_e19874_d_n12,) = {
    if ((locals.var_guard1240 != 0.0) && (locals.var_guard1275 == 0.0)) {
        let assign22000_e19870: f64 = (locals.var_wtsi * locals.var_jbjts);
        let assign22000_e19872: f64 = (assign22000_e19870 * locals.var_pparam_b4soilratiodif);
        (assign22000_e19872, ((((locals.var_wtsi_dn3 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn3)) * locals.var_pparam_b4soilratiodif) + (assign22000_e19870 * locals.var_pparam_b4soilratiodif_dn3)), ((((locals.var_wtsi_dn4 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn4)) * locals.var_pparam_b4soilratiodif) + (assign22000_e19870 * locals.var_pparam_b4soilratiodif_dn4)), ((((locals.var_wtsi_dn5 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn5)) * locals.var_pparam_b4soilratiodif) + (assign22000_e19870 * locals.var_pparam_b4soilratiodif_dn5)), ((((locals.var_wtsi_dn6 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn6)) * locals.var_pparam_b4soilratiodif) + (assign22000_e19870 * locals.var_pparam_b4soilratiodif_dn6)), ((((locals.var_wtsi_dn7 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn7)) * locals.var_pparam_b4soilratiodif) + (assign22000_e19870 * locals.var_pparam_b4soilratiodif_dn7)), ((((locals.var_wtsi_dn8 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn8)) * locals.var_pparam_b4soilratiodif) + (assign22000_e19870 * locals.var_pparam_b4soilratiodif_dn8)), ((((locals.var_wtsi_dn9 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn9)) * locals.var_pparam_b4soilratiodif) + (assign22000_e19870 * locals.var_pparam_b4soilratiodif_dn9)), ((((locals.var_wtsi_dn10 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn10)) * locals.var_pparam_b4soilratiodif) + (assign22000_e19870 * locals.var_pparam_b4soilratiodif_dn10)), ((((locals.var_wtsi_dn11 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn11)) * locals.var_pparam_b4soilratiodif) + (assign22000_e19870 * locals.var_pparam_b4soilratiodif_dn11)), ((((locals.var_wtsi_dn12 * locals.var_jbjts) + (locals.var_wtsi * locals.var_jbjts_dn12)) * locals.var_pparam_b4soilratiodif) + (assign22000_e19870 * locals.var_pparam_b4soilratiodif_dn12)),)
    } else {
        (locals.var_iendif, locals.var_iendif_dn3, locals.var_iendif_dn4, locals.var_iendif_dn5, locals.var_iendif_dn6, locals.var_iendif_dn7, locals.var_iendif_dn8, locals.var_iendif_dn9, locals.var_iendif_dn10, locals.var_iendif_dn11, locals.var_iendif_dn12,)
    }
};
        locals.var_iendif = assign22000_e19874;
        locals.var_iendif_dn3 = assign22000_e19874_d_n3;
        locals.var_iendif_dn4 = assign22000_e19874_d_n4;
        locals.var_iendif_dn5 = assign22000_e19874_d_n5;
        locals.var_iendif_dn6 = assign22000_e19874_d_n6;
        locals.var_iendif_dn7 = assign22000_e19874_d_n7;
        locals.var_iendif_dn8 = assign22000_e19874_d_n8;
        locals.var_iendif_dn9 = assign22000_e19874_d_n9;
        locals.var_iendif_dn10 = assign22000_e19874_d_n10;
        locals.var_iendif_dn11 = assign22000_e19874_d_n11;
        locals.var_iendif_dn12 = assign22000_e19874_d_n12;

        let (assign22010_e19887, assign22010_e19887_d_n3, assign22010_e19887_d_n4, assign22010_e19887_d_n5, assign22010_e19887_d_n6, assign22010_e19887_d_n7, assign22010_e19887_d_n8, assign22010_e19887_d_n9, assign22010_e19887_d_n10, assign22010_e19887_d_n11, assign22010_e19887_d_n12,) = {
    if ((locals.var_guard1240 != 0.0) && (locals.var_guard1275 == 0.0)) {
        let assign22010_e19882: f64 = (locals.var_expvbsnvtm - 1.0);
        let assign22010_e19883: f64 = (locals.var_iendif * assign22010_e19882);
        let assign22010_e19885: f64 = (assign22010_e19883 * locals.var_ehlisfactor);
        (assign22010_e19885, ((((locals.var_iendif_dn3 * assign22010_e19882) + (locals.var_iendif * locals.var_expvbsnvtm_dn3)) * locals.var_ehlisfactor) + (assign22010_e19883 * locals.var_ehlisfactor_dn3)), ((((locals.var_iendif_dn4 * assign22010_e19882) + (locals.var_iendif * locals.var_expvbsnvtm_dn4)) * locals.var_ehlisfactor) + (assign22010_e19883 * locals.var_ehlisfactor_dn4)), ((((locals.var_iendif_dn5 * assign22010_e19882) + (locals.var_iendif * locals.var_expvbsnvtm_dn5)) * locals.var_ehlisfactor) + (assign22010_e19883 * locals.var_ehlisfactor_dn5)), ((((locals.var_iendif_dn6 * assign22010_e19882) + (locals.var_iendif * locals.var_expvbsnvtm_dn6)) * locals.var_ehlisfactor) + (assign22010_e19883 * locals.var_ehlisfactor_dn6)), ((((locals.var_iendif_dn7 * assign22010_e19882) + (locals.var_iendif * locals.var_expvbsnvtm_dn7)) * locals.var_ehlisfactor) + (assign22010_e19883 * locals.var_ehlisfactor_dn7)), ((((locals.var_iendif_dn8 * assign22010_e19882) + (locals.var_iendif * locals.var_expvbsnvtm_dn8)) * locals.var_ehlisfactor) + (assign22010_e19883 * locals.var_ehlisfactor_dn8)), ((((locals.var_iendif_dn9 * assign22010_e19882) + (locals.var_iendif * locals.var_expvbsnvtm_dn9)) * locals.var_ehlisfactor) + (assign22010_e19883 * locals.var_ehlisfactor_dn9)), ((((locals.var_iendif_dn10 * assign22010_e19882) + (locals.var_iendif * locals.var_expvbsnvtm_dn10)) * locals.var_ehlisfactor) + (assign22010_e19883 * locals.var_ehlisfactor_dn10)), ((((locals.var_iendif_dn11 * assign22010_e19882) + (locals.var_iendif * locals.var_expvbsnvtm_dn11)) * locals.var_ehlisfactor) + (assign22010_e19883 * locals.var_ehlisfactor_dn11)), ((((locals.var_iendif_dn12 * assign22010_e19882) + (locals.var_iendif * locals.var_expvbsnvtm_dn12)) * locals.var_ehlisfactor) + (assign22010_e19883 * locals.var_ehlisfactor_dn12)),)
    } else {
        (locals.var_ibsdif, locals.var_ibsdif_dn3, locals.var_ibsdif_dn4, locals.var_ibsdif_dn5, locals.var_ibsdif_dn6, locals.var_ibsdif_dn7, locals.var_ibsdif_dn8, locals.var_ibsdif_dn9, locals.var_ibsdif_dn10, locals.var_ibsdif_dn11, locals.var_ibsdif_dn12,)
    }
};
        locals.var_ibsdif = assign22010_e19887;
        locals.var_ibsdif_dn3 = assign22010_e19887_d_n3;
        locals.var_ibsdif_dn4 = assign22010_e19887_d_n4;
        locals.var_ibsdif_dn5 = assign22010_e19887_d_n5;
        locals.var_ibsdif_dn6 = assign22010_e19887_d_n6;
        locals.var_ibsdif_dn7 = assign22010_e19887_d_n7;
        locals.var_ibsdif_dn8 = assign22010_e19887_d_n8;
        locals.var_ibsdif_dn9 = assign22010_e19887_d_n9;
        locals.var_ibsdif_dn10 = assign22010_e19887_d_n10;
        locals.var_ibsdif_dn11 = assign22010_e19887_d_n11;
        locals.var_ibsdif_dn12 = assign22010_e19887_d_n12;

        let (assign22020_e19898, assign22020_e19898_d_n3, assign22020_e19898_d_n4, assign22020_e19898_d_n5, assign22020_e19898_d_n6, assign22020_e19898_d_n7, assign22020_e19898_d_n8, assign22020_e19898_d_n9, assign22020_e19898_d_n10, assign22020_e19898_d_n11, assign22020_e19898_d_n12,) = {
    if ((locals.var_guard1240 != 0.0) && (locals.var_guard1275 == 0.0)) {
        let assign22020_e19894: f64 = (locals.var_wtsi * locals.var_jbjtd);
        let assign22020_e19896: f64 = (assign22020_e19894 * locals.var_pparam_b4soilratiodif);
        (assign22020_e19896, ((((locals.var_wtsi_dn3 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn3)) * locals.var_pparam_b4soilratiodif) + (assign22020_e19894 * locals.var_pparam_b4soilratiodif_dn3)), ((((locals.var_wtsi_dn4 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn4)) * locals.var_pparam_b4soilratiodif) + (assign22020_e19894 * locals.var_pparam_b4soilratiodif_dn4)), ((((locals.var_wtsi_dn5 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn5)) * locals.var_pparam_b4soilratiodif) + (assign22020_e19894 * locals.var_pparam_b4soilratiodif_dn5)), ((((locals.var_wtsi_dn6 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn6)) * locals.var_pparam_b4soilratiodif) + (assign22020_e19894 * locals.var_pparam_b4soilratiodif_dn6)), ((((locals.var_wtsi_dn7 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn7)) * locals.var_pparam_b4soilratiodif) + (assign22020_e19894 * locals.var_pparam_b4soilratiodif_dn7)), ((((locals.var_wtsi_dn8 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn8)) * locals.var_pparam_b4soilratiodif) + (assign22020_e19894 * locals.var_pparam_b4soilratiodif_dn8)), ((((locals.var_wtsi_dn9 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn9)) * locals.var_pparam_b4soilratiodif) + (assign22020_e19894 * locals.var_pparam_b4soilratiodif_dn9)), ((((locals.var_wtsi_dn10 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn10)) * locals.var_pparam_b4soilratiodif) + (assign22020_e19894 * locals.var_pparam_b4soilratiodif_dn10)), ((((locals.var_wtsi_dn11 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn11)) * locals.var_pparam_b4soilratiodif) + (assign22020_e19894 * locals.var_pparam_b4soilratiodif_dn11)), ((((locals.var_wtsi_dn12 * locals.var_jbjtd) + (locals.var_wtsi * locals.var_jbjtd_dn12)) * locals.var_pparam_b4soilratiodif) + (assign22020_e19894 * locals.var_pparam_b4soilratiodif_dn12)),)
    } else {
        (locals.var_iendif, locals.var_iendif_dn3, locals.var_iendif_dn4, locals.var_iendif_dn5, locals.var_iendif_dn6, locals.var_iendif_dn7, locals.var_iendif_dn8, locals.var_iendif_dn9, locals.var_iendif_dn10, locals.var_iendif_dn11, locals.var_iendif_dn12,)
    }
};
        locals.var_iendif = assign22020_e19898;
        locals.var_iendif_dn3 = assign22020_e19898_d_n3;
        locals.var_iendif_dn4 = assign22020_e19898_d_n4;
        locals.var_iendif_dn5 = assign22020_e19898_d_n5;
        locals.var_iendif_dn6 = assign22020_e19898_d_n6;
        locals.var_iendif_dn7 = assign22020_e19898_d_n7;
        locals.var_iendif_dn8 = assign22020_e19898_d_n8;
        locals.var_iendif_dn9 = assign22020_e19898_d_n9;
        locals.var_iendif_dn10 = assign22020_e19898_d_n10;
        locals.var_iendif_dn11 = assign22020_e19898_d_n11;
        locals.var_iendif_dn12 = assign22020_e19898_d_n12;

        let (assign22030_e19911, assign22030_e19911_d_n3, assign22030_e19911_d_n4, assign22030_e19911_d_n5, assign22030_e19911_d_n6, assign22030_e19911_d_n7, assign22030_e19911_d_n8, assign22030_e19911_d_n9, assign22030_e19911_d_n10, assign22030_e19911_d_n11, assign22030_e19911_d_n12,) = {
    if ((locals.var_guard1240 != 0.0) && (locals.var_guard1275 == 0.0)) {
        let assign22030_e19906: f64 = (locals.var_expvbdnvtm - 1.0);
        let assign22030_e19907: f64 = (locals.var_iendif * assign22030_e19906);
        let assign22030_e19909: f64 = (assign22030_e19907 * locals.var_ehlidfactor);
        (assign22030_e19909, ((((locals.var_iendif_dn3 * assign22030_e19906) + (locals.var_iendif * locals.var_expvbdnvtm_dn3)) * locals.var_ehlidfactor) + (assign22030_e19907 * locals.var_ehlidfactor_dn3)), ((((locals.var_iendif_dn4 * assign22030_e19906) + (locals.var_iendif * locals.var_expvbdnvtm_dn4)) * locals.var_ehlidfactor) + (assign22030_e19907 * locals.var_ehlidfactor_dn4)), ((((locals.var_iendif_dn5 * assign22030_e19906) + (locals.var_iendif * locals.var_expvbdnvtm_dn5)) * locals.var_ehlidfactor) + (assign22030_e19907 * locals.var_ehlidfactor_dn5)), ((((locals.var_iendif_dn6 * assign22030_e19906) + (locals.var_iendif * locals.var_expvbdnvtm_dn6)) * locals.var_ehlidfactor) + (assign22030_e19907 * locals.var_ehlidfactor_dn6)), ((((locals.var_iendif_dn7 * assign22030_e19906) + (locals.var_iendif * locals.var_expvbdnvtm_dn7)) * locals.var_ehlidfactor) + (assign22030_e19907 * locals.var_ehlidfactor_dn7)), ((((locals.var_iendif_dn8 * assign22030_e19906) + (locals.var_iendif * locals.var_expvbdnvtm_dn8)) * locals.var_ehlidfactor) + (assign22030_e19907 * locals.var_ehlidfactor_dn8)), ((((locals.var_iendif_dn9 * assign22030_e19906) + (locals.var_iendif * locals.var_expvbdnvtm_dn9)) * locals.var_ehlidfactor) + (assign22030_e19907 * locals.var_ehlidfactor_dn9)), ((((locals.var_iendif_dn10 * assign22030_e19906) + (locals.var_iendif * locals.var_expvbdnvtm_dn10)) * locals.var_ehlidfactor) + (assign22030_e19907 * locals.var_ehlidfactor_dn10)), ((((locals.var_iendif_dn11 * assign22030_e19906) + (locals.var_iendif * locals.var_expvbdnvtm_dn11)) * locals.var_ehlidfactor) + (assign22030_e19907 * locals.var_ehlidfactor_dn11)), ((((locals.var_iendif_dn12 * assign22030_e19906) + (locals.var_iendif * locals.var_expvbdnvtm_dn12)) * locals.var_ehlidfactor) + (assign22030_e19907 * locals.var_ehlidfactor_dn12)),)
    } else {
        (locals.var_ibddif, locals.var_ibddif_dn3, locals.var_ibddif_dn4, locals.var_ibddif_dn5, locals.var_ibddif_dn6, locals.var_ibddif_dn7, locals.var_ibddif_dn8, locals.var_ibddif_dn9, locals.var_ibddif_dn10, locals.var_ibddif_dn11, locals.var_ibddif_dn12,)
    }
};
        locals.var_ibddif = assign22030_e19911;
        locals.var_ibddif_dn3 = assign22030_e19911_d_n3;
        locals.var_ibddif_dn4 = assign22030_e19911_d_n4;
        locals.var_ibddif_dn5 = assign22030_e19911_d_n5;
        locals.var_ibddif_dn6 = assign22030_e19911_d_n6;
        locals.var_ibddif_dn7 = assign22030_e19911_d_n7;
        locals.var_ibddif_dn8 = assign22030_e19911_d_n8;
        locals.var_ibddif_dn9 = assign22030_e19911_d_n9;
        locals.var_ibddif_dn10 = assign22030_e19911_d_n10;
        locals.var_ibddif_dn11 = assign22030_e19911_d_n11;
        locals.var_ibddif_dn12 = assign22030_e19911_d_n12;

        let assign22040_e19914: f64 = if p.p13 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1278 = assign22040_e19914;

        let (assign22050_e19923, assign22050_e19923_d_n3, assign22050_e19923_d_n4, assign22050_e19923_d_n5, assign22050_e19923_d_n6, assign22050_e19923_d_n7, assign22050_e19923_d_n8, assign22050_e19923_d_n9, assign22050_e19923_d_n10, assign22050_e19923_d_n11, assign22050_e19923_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1275 == 0.0)) && (locals.var_guard1278 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ic_1, locals.var_ic_1_dn3, locals.var_ic_1_dn4, locals.var_ic_1_dn5, locals.var_ic_1_dn6, locals.var_ic_1_dn7, locals.var_ic_1_dn8, locals.var_ic_1_dn9, locals.var_ic_1_dn10, locals.var_ic_1_dn11, locals.var_ic_1_dn12,)
    }
};
        locals.var_ic_1 = assign22050_e19923;
        locals.var_ic_1_dn3 = assign22050_e19923_d_n3;
        locals.var_ic_1_dn4 = assign22050_e19923_d_n4;
        locals.var_ic_1_dn5 = assign22050_e19923_d_n5;
        locals.var_ic_1_dn6 = assign22050_e19923_d_n6;
        locals.var_ic_1_dn7 = assign22050_e19923_d_n7;
        locals.var_ic_1_dn8 = assign22050_e19923_d_n8;
        locals.var_ic_1_dn9 = assign22050_e19923_d_n9;
        locals.var_ic_1_dn10 = assign22050_e19923_d_n10;
        locals.var_ic_1_dn11 = assign22050_e19923_d_n11;
        locals.var_ic_1_dn12 = assign22050_e19923_d_n12;

        let (assign22060_e19939, assign22060_e19939_d_n3, assign22060_e19939_d_n4, assign22060_e19939_d_n5, assign22060_e19939_d_n6, assign22060_e19939_d_n7, assign22060_e19939_d_n8, assign22060_e19939_d_n9, assign22060_e19939_d_n10, assign22060_e19939_d_n11, assign22060_e19939_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1275 == 0.0)) && (locals.var_guard1278 == 0.0)) {
        let assign22060_e19934: f64 = (locals.var_vsbs + locals.var_vdbd);
        let assign22060_e19936: f64 = (assign22060_e19934 / locals.var_pparam_b4soivearly);
        let assign22060_e19937: f64 = (1.0 + assign22060_e19936);
        (assign22060_e19937, (-((assign22060_e19934 * locals.var_pparam_b4soivearly_dn3) / (locals.var_pparam_b4soivearly * locals.var_pparam_b4soivearly))), (-((assign22060_e19934 * locals.var_pparam_b4soivearly_dn4) / (locals.var_pparam_b4soivearly * locals.var_pparam_b4soivearly))), (-((assign22060_e19934 * locals.var_pparam_b4soivearly_dn5) / (locals.var_pparam_b4soivearly * locals.var_pparam_b4soivearly))), (-((assign22060_e19934 * locals.var_pparam_b4soivearly_dn6) / (locals.var_pparam_b4soivearly * locals.var_pparam_b4soivearly))), (((locals.var_vdbd_dn7 * locals.var_pparam_b4soivearly) - (assign22060_e19934 * locals.var_pparam_b4soivearly_dn7)) / (locals.var_pparam_b4soivearly * locals.var_pparam_b4soivearly)), (((locals.var_vsbs_dn8 * locals.var_pparam_b4soivearly) - (assign22060_e19934 * locals.var_pparam_b4soivearly_dn8)) / (locals.var_pparam_b4soivearly * locals.var_pparam_b4soivearly)), (-((assign22060_e19934 * locals.var_pparam_b4soivearly_dn9) / (locals.var_pparam_b4soivearly * locals.var_pparam_b4soivearly))), (-((assign22060_e19934 * locals.var_pparam_b4soivearly_dn10) / (locals.var_pparam_b4soivearly * locals.var_pparam_b4soivearly))), (((locals.var_vsbs_dn11 * locals.var_pparam_b4soivearly) - (assign22060_e19934 * locals.var_pparam_b4soivearly_dn11)) / (locals.var_pparam_b4soivearly * locals.var_pparam_b4soivearly)), (((locals.var_vdbd_dn12 * locals.var_pparam_b4soivearly) - (assign22060_e19934 * locals.var_pparam_b4soivearly_dn12)) / (locals.var_pparam_b4soivearly * locals.var_pparam_b4soivearly)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign22060_e19939;
        locals.var_t0__blk808_dn3 = assign22060_e19939_d_n3;
        locals.var_t0__blk808_dn4 = assign22060_e19939_d_n4;
        locals.var_t0__blk808_dn5 = assign22060_e19939_d_n5;
        locals.var_t0__blk808_dn6 = assign22060_e19939_d_n6;
        locals.var_t0__blk808_dn7 = assign22060_e19939_d_n7;
        locals.var_t0__blk808_dn8 = assign22060_e19939_d_n8;
        locals.var_t0__blk808_dn9 = assign22060_e19939_d_n9;
        locals.var_t0__blk808_dn10 = assign22060_e19939_d_n10;
        locals.var_t0__blk808_dn11 = assign22060_e19939_d_n11;
        locals.var_t0__blk808_dn12 = assign22060_e19939_d_n12;

        let (assign22070_e19951, assign22070_e19951_d_n3, assign22070_e19951_d_n4, assign22070_e19951_d_n5, assign22070_e19951_d_n6, assign22070_e19951_d_n7, assign22070_e19951_d_n8, assign22070_e19951_d_n9, assign22070_e19951_d_n10, assign22070_e19951_d_n11, assign22070_e19951_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1275 == 0.0)) && (locals.var_guard1278 == 0.0)) {
        let assign22070_e19949: f64 = (locals.var_ehlis + locals.var_ehlid);
        (assign22070_e19949, (locals.var_ehlis_dn3 + locals.var_ehlid_dn3), (locals.var_ehlis_dn4 + locals.var_ehlid_dn4), (locals.var_ehlis_dn5 + locals.var_ehlid_dn5), (locals.var_ehlis_dn6 + locals.var_ehlid_dn6), (locals.var_ehlis_dn7 + locals.var_ehlid_dn7), (locals.var_ehlis_dn8 + locals.var_ehlid_dn8), (locals.var_ehlis_dn9 + locals.var_ehlid_dn9), (locals.var_ehlis_dn10 + locals.var_ehlid_dn10), (locals.var_ehlis_dn11 + locals.var_ehlid_dn11), (locals.var_ehlis_dn12 + locals.var_ehlid_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign22070_e19951;
        locals.var_t1__blk809_dn3 = assign22070_e19951_d_n3;
        locals.var_t1__blk809_dn4 = assign22070_e19951_d_n4;
        locals.var_t1__blk809_dn5 = assign22070_e19951_d_n5;
        locals.var_t1__blk809_dn6 = assign22070_e19951_d_n6;
        locals.var_t1__blk809_dn7 = assign22070_e19951_d_n7;
        locals.var_t1__blk809_dn8 = assign22070_e19951_d_n8;
        locals.var_t1__blk809_dn9 = assign22070_e19951_d_n9;
        locals.var_t1__blk809_dn10 = assign22070_e19951_d_n10;
        locals.var_t1__blk809_dn11 = assign22070_e19951_d_n11;
        locals.var_t1__blk809_dn12 = assign22070_e19951_d_n12;

        let (assign22080_e19968, assign22080_e19968_d_n3, assign22080_e19968_d_n4, assign22080_e19968_d_n5, assign22080_e19968_d_n6, assign22080_e19968_d_n7, assign22080_e19968_d_n8, assign22080_e19968_d_n9, assign22080_e19968_d_n10, assign22080_e19968_d_n11, assign22080_e19968_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1275 == 0.0)) && (locals.var_guard1278 == 0.0)) {
        let assign22080_e19961: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        let assign22080_e19964: f64 = (4.0 * locals.var_t1__blk809);
        let assign22080_e19965: f64 = (assign22080_e19961 + assign22080_e19964);
        let assign22080_e19966: f64 = (assign22080_e19965).sqrt();
        (assign22080_e19966, ((((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)) + (4.0 * locals.var_t1__blk809_dn3)) / (2.0 * assign22080_e19966)), ((((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)) + (4.0 * locals.var_t1__blk809_dn4)) / (2.0 * assign22080_e19966)), ((((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)) + (4.0 * locals.var_t1__blk809_dn5)) / (2.0 * assign22080_e19966)), ((((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)) + (4.0 * locals.var_t1__blk809_dn6)) / (2.0 * assign22080_e19966)), ((((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)) + (4.0 * locals.var_t1__blk809_dn7)) / (2.0 * assign22080_e19966)), ((((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)) + (4.0 * locals.var_t1__blk809_dn8)) / (2.0 * assign22080_e19966)), ((((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)) + (4.0 * locals.var_t1__blk809_dn9)) / (2.0 * assign22080_e19966)), ((((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)) + (4.0 * locals.var_t1__blk809_dn10)) / (2.0 * assign22080_e19966)), ((((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)) + (4.0 * locals.var_t1__blk809_dn11)) / (2.0 * assign22080_e19966)), ((((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)) + (4.0 * locals.var_t1__blk809_dn12)) / (2.0 * assign22080_e19966)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign22080_e19968;
        locals.var_t3__blk811_dn3 = assign22080_e19968_d_n3;
        locals.var_t3__blk811_dn4 = assign22080_e19968_d_n4;
        locals.var_t3__blk811_dn5 = assign22080_e19968_d_n5;
        locals.var_t3__blk811_dn6 = assign22080_e19968_d_n6;
        locals.var_t3__blk811_dn7 = assign22080_e19968_d_n7;
        locals.var_t3__blk811_dn8 = assign22080_e19968_d_n8;
        locals.var_t3__blk811_dn9 = assign22080_e19968_d_n9;
        locals.var_t3__blk811_dn10 = assign22080_e19968_d_n10;
        locals.var_t3__blk811_dn11 = assign22080_e19968_d_n11;
        locals.var_t3__blk811_dn12 = assign22080_e19968_d_n12;

        let (assign22090_e19982, assign22090_e19982_d_n3, assign22090_e19982_d_n4, assign22090_e19982_d_n5, assign22090_e19982_d_n6, assign22090_e19982_d_n7, assign22090_e19982_d_n8, assign22090_e19982_d_n9, assign22090_e19982_d_n10, assign22090_e19982_d_n11, assign22090_e19982_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1275 == 0.0)) && (locals.var_guard1278 == 0.0)) {
        let assign22090_e19978: f64 = (locals.var_t0__blk808 + locals.var_t3__blk811);
        let assign22090_e19980: f64 = (assign22090_e19978 / 2.0);
        (assign22090_e19980, ((locals.var_t0__blk808_dn3 + locals.var_t3__blk811_dn3) / 2.0), ((locals.var_t0__blk808_dn4 + locals.var_t3__blk811_dn4) / 2.0), ((locals.var_t0__blk808_dn5 + locals.var_t3__blk811_dn5) / 2.0), ((locals.var_t0__blk808_dn6 + locals.var_t3__blk811_dn6) / 2.0), ((locals.var_t0__blk808_dn7 + locals.var_t3__blk811_dn7) / 2.0), ((locals.var_t0__blk808_dn8 + locals.var_t3__blk811_dn8) / 2.0), ((locals.var_t0__blk808_dn9 + locals.var_t3__blk811_dn9) / 2.0), ((locals.var_t0__blk808_dn10 + locals.var_t3__blk811_dn10) / 2.0), ((locals.var_t0__blk808_dn11 + locals.var_t3__blk811_dn11) / 2.0), ((locals.var_t0__blk808_dn12 + locals.var_t3__blk811_dn12) / 2.0),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign22090_e19982;
        locals.var_t2__blk810_dn3 = assign22090_e19982_d_n3;
        locals.var_t2__blk810_dn4 = assign22090_e19982_d_n4;
        locals.var_t2__blk810_dn5 = assign22090_e19982_d_n5;
        locals.var_t2__blk810_dn6 = assign22090_e19982_d_n6;
        locals.var_t2__blk810_dn7 = assign22090_e19982_d_n7;
        locals.var_t2__blk810_dn8 = assign22090_e19982_d_n8;
        locals.var_t2__blk810_dn9 = assign22090_e19982_d_n9;
        locals.var_t2__blk810_dn10 = assign22090_e19982_d_n10;
        locals.var_t2__blk810_dn11 = assign22090_e19982_d_n11;
        locals.var_t2__blk810_dn12 = assign22090_e19982_d_n12;

        let assign22100_e19985: f64 = if locals.var_t2__blk810 < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard1279 = assign22100_e19985;

        let (assign22110_e19997, assign22110_e19997_d_n3, assign22110_e19997_d_n4, assign22110_e19997_d_n5, assign22110_e19997_d_n6, assign22110_e19997_d_n7, assign22110_e19997_d_n8, assign22110_e19997_d_n9, assign22110_e19997_d_n10, assign22110_e19997_d_n11, assign22110_e19997_d_n12,) = {
    if ((((locals.var_guard1240 != 0.0) && (locals.var_guard1275 == 0.0)) && (locals.var_guard1278 == 0.0)) && (locals.var_guard1279 != 0.0)) {
        (10.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_e2ndfactor, locals.var_e2ndfactor_dn3, locals.var_e2ndfactor_dn4, locals.var_e2ndfactor_dn5, locals.var_e2ndfactor_dn6, locals.var_e2ndfactor_dn7, locals.var_e2ndfactor_dn8, locals.var_e2ndfactor_dn9, locals.var_e2ndfactor_dn10, locals.var_e2ndfactor_dn11, locals.var_e2ndfactor_dn12,)
    }
};
        locals.var_e2ndfactor = assign22110_e19997;
        locals.var_e2ndfactor_dn3 = assign22110_e19997_d_n3;
        locals.var_e2ndfactor_dn4 = assign22110_e19997_d_n4;
        locals.var_e2ndfactor_dn5 = assign22110_e19997_d_n5;
        locals.var_e2ndfactor_dn6 = assign22110_e19997_d_n6;
        locals.var_e2ndfactor_dn7 = assign22110_e19997_d_n7;
        locals.var_e2ndfactor_dn8 = assign22110_e19997_d_n8;
        locals.var_e2ndfactor_dn9 = assign22110_e19997_d_n9;
        locals.var_e2ndfactor_dn10 = assign22110_e19997_d_n10;
        locals.var_e2ndfactor_dn11 = assign22110_e19997_d_n11;
        locals.var_e2ndfactor_dn12 = assign22110_e19997_d_n12;

        let (assign22120_e20012, assign22120_e20012_d_n3, assign22120_e20012_d_n4, assign22120_e20012_d_n5, assign22120_e20012_d_n6, assign22120_e20012_d_n7, assign22120_e20012_d_n8, assign22120_e20012_d_n9, assign22120_e20012_d_n10, assign22120_e20012_d_n11, assign22120_e20012_d_n12,) = {
    if ((((locals.var_guard1240 != 0.0) && (locals.var_guard1275 == 0.0)) && (locals.var_guard1278 == 0.0)) && (locals.var_guard1279 == 0.0)) {
        let assign22120_e20010: f64 = (1.0 / locals.var_t2__blk810);
        (assign22120_e20010, (-(locals.var_t2__blk810_dn3 / (locals.var_t2__blk810 * locals.var_t2__blk810))), (-(locals.var_t2__blk810_dn4 / (locals.var_t2__blk810 * locals.var_t2__blk810))), (-(locals.var_t2__blk810_dn5 / (locals.var_t2__blk810 * locals.var_t2__blk810))), (-(locals.var_t2__blk810_dn6 / (locals.var_t2__blk810 * locals.var_t2__blk810))), (-(locals.var_t2__blk810_dn7 / (locals.var_t2__blk810 * locals.var_t2__blk810))), (-(locals.var_t2__blk810_dn8 / (locals.var_t2__blk810 * locals.var_t2__blk810))), (-(locals.var_t2__blk810_dn9 / (locals.var_t2__blk810 * locals.var_t2__blk810))), (-(locals.var_t2__blk810_dn10 / (locals.var_t2__blk810 * locals.var_t2__blk810))), (-(locals.var_t2__blk810_dn11 / (locals.var_t2__blk810 * locals.var_t2__blk810))), (-(locals.var_t2__blk810_dn12 / (locals.var_t2__blk810 * locals.var_t2__blk810))),)
    } else {
        (locals.var_e2ndfactor, locals.var_e2ndfactor_dn3, locals.var_e2ndfactor_dn4, locals.var_e2ndfactor_dn5, locals.var_e2ndfactor_dn6, locals.var_e2ndfactor_dn7, locals.var_e2ndfactor_dn8, locals.var_e2ndfactor_dn9, locals.var_e2ndfactor_dn10, locals.var_e2ndfactor_dn11, locals.var_e2ndfactor_dn12,)
    }
};
        locals.var_e2ndfactor = assign22120_e20012;
        locals.var_e2ndfactor_dn3 = assign22120_e20012_d_n3;
        locals.var_e2ndfactor_dn4 = assign22120_e20012_d_n4;
        locals.var_e2ndfactor_dn5 = assign22120_e20012_d_n5;
        locals.var_e2ndfactor_dn6 = assign22120_e20012_d_n6;
        locals.var_e2ndfactor_dn7 = assign22120_e20012_d_n7;
        locals.var_e2ndfactor_dn8 = assign22120_e20012_d_n8;
        locals.var_e2ndfactor_dn9 = assign22120_e20012_d_n9;
        locals.var_e2ndfactor_dn10 = assign22120_e20012_d_n10;
        locals.var_e2ndfactor_dn11 = assign22120_e20012_d_n11;
        locals.var_e2ndfactor_dn12 = assign22120_e20012_d_n12;

        let (assign22130_e20024, assign22130_e20024_d_n3, assign22130_e20024_d_n4, assign22130_e20024_d_n5, assign22130_e20024_d_n6, assign22130_e20024_d_n7, assign22130_e20024_d_n8, assign22130_e20024_d_n9, assign22130_e20024_d_n10, assign22130_e20024_d_n11, assign22130_e20024_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1275 == 0.0)) && (locals.var_guard1278 == 0.0)) {
        let assign22130_e20022: f64 = (locals.var_pparam_b4soiarfabjt * locals.var_ien);
        (assign22130_e20022, ((locals.var_pparam_b4soiarfabjt_dn3 * locals.var_ien) + (locals.var_pparam_b4soiarfabjt * locals.var_ien_dn3)), ((locals.var_pparam_b4soiarfabjt_dn4 * locals.var_ien) + (locals.var_pparam_b4soiarfabjt * locals.var_ien_dn4)), ((locals.var_pparam_b4soiarfabjt_dn5 * locals.var_ien) + (locals.var_pparam_b4soiarfabjt * locals.var_ien_dn5)), ((locals.var_pparam_b4soiarfabjt_dn6 * locals.var_ien) + (locals.var_pparam_b4soiarfabjt * locals.var_ien_dn6)), ((locals.var_pparam_b4soiarfabjt_dn7 * locals.var_ien) + (locals.var_pparam_b4soiarfabjt * locals.var_ien_dn7)), ((locals.var_pparam_b4soiarfabjt_dn8 * locals.var_ien) + (locals.var_pparam_b4soiarfabjt * locals.var_ien_dn8)), ((locals.var_pparam_b4soiarfabjt_dn9 * locals.var_ien) + (locals.var_pparam_b4soiarfabjt * locals.var_ien_dn9)), ((locals.var_pparam_b4soiarfabjt_dn10 * locals.var_ien) + (locals.var_pparam_b4soiarfabjt * locals.var_ien_dn10)), ((locals.var_pparam_b4soiarfabjt_dn11 * locals.var_ien) + (locals.var_pparam_b4soiarfabjt * locals.var_ien_dn11)), ((locals.var_pparam_b4soiarfabjt_dn12 * locals.var_ien) + (locals.var_pparam_b4soiarfabjt * locals.var_ien_dn12)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign22130_e20024;
        locals.var_t0__blk808_dn3 = assign22130_e20024_d_n3;
        locals.var_t0__blk808_dn4 = assign22130_e20024_d_n4;
        locals.var_t0__blk808_dn5 = assign22130_e20024_d_n5;
        locals.var_t0__blk808_dn6 = assign22130_e20024_d_n6;
        locals.var_t0__blk808_dn7 = assign22130_e20024_d_n7;
        locals.var_t0__blk808_dn8 = assign22130_e20024_d_n8;
        locals.var_t0__blk808_dn9 = assign22130_e20024_d_n9;
        locals.var_t0__blk808_dn10 = assign22130_e20024_d_n10;
        locals.var_t0__blk808_dn11 = assign22130_e20024_d_n11;
        locals.var_t0__blk808_dn12 = assign22130_e20024_d_n12;

        let (assign22140_e20040, assign22140_e20040_d_n3, assign22140_e20040_d_n4, assign22140_e20040_d_n5, assign22140_e20040_d_n6, assign22140_e20040_d_n7, assign22140_e20040_d_n8, assign22140_e20040_d_n9, assign22140_e20040_d_n10, assign22140_e20040_d_n11, assign22140_e20040_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1275 == 0.0)) && (locals.var_guard1278 == 0.0)) {
        let assign22140_e20035: f64 = (locals.var_expvbsnvtm - locals.var_expvbdnvtm);
        let assign22140_e20036: f64 = (locals.var_t0__blk808 * assign22140_e20035);
        let assign22140_e20038: f64 = (assign22140_e20036 * locals.var_e2ndfactor);
        (assign22140_e20038, ((((locals.var_t0__blk808_dn3 * assign22140_e20035) + (locals.var_t0__blk808 * (locals.var_expvbsnvtm_dn3 - locals.var_expvbdnvtm_dn3))) * locals.var_e2ndfactor) + (assign22140_e20036 * locals.var_e2ndfactor_dn3)), ((((locals.var_t0__blk808_dn4 * assign22140_e20035) + (locals.var_t0__blk808 * (locals.var_expvbsnvtm_dn4 - locals.var_expvbdnvtm_dn4))) * locals.var_e2ndfactor) + (assign22140_e20036 * locals.var_e2ndfactor_dn4)), ((((locals.var_t0__blk808_dn5 * assign22140_e20035) + (locals.var_t0__blk808 * (locals.var_expvbsnvtm_dn5 - locals.var_expvbdnvtm_dn5))) * locals.var_e2ndfactor) + (assign22140_e20036 * locals.var_e2ndfactor_dn5)), ((((locals.var_t0__blk808_dn6 * assign22140_e20035) + (locals.var_t0__blk808 * (locals.var_expvbsnvtm_dn6 - locals.var_expvbdnvtm_dn6))) * locals.var_e2ndfactor) + (assign22140_e20036 * locals.var_e2ndfactor_dn6)), ((((locals.var_t0__blk808_dn7 * assign22140_e20035) + (locals.var_t0__blk808 * (locals.var_expvbsnvtm_dn7 - locals.var_expvbdnvtm_dn7))) * locals.var_e2ndfactor) + (assign22140_e20036 * locals.var_e2ndfactor_dn7)), ((((locals.var_t0__blk808_dn8 * assign22140_e20035) + (locals.var_t0__blk808 * (locals.var_expvbsnvtm_dn8 - locals.var_expvbdnvtm_dn8))) * locals.var_e2ndfactor) + (assign22140_e20036 * locals.var_e2ndfactor_dn8)), ((((locals.var_t0__blk808_dn9 * assign22140_e20035) + (locals.var_t0__blk808 * (locals.var_expvbsnvtm_dn9 - locals.var_expvbdnvtm_dn9))) * locals.var_e2ndfactor) + (assign22140_e20036 * locals.var_e2ndfactor_dn9)), ((((locals.var_t0__blk808_dn10 * assign22140_e20035) + (locals.var_t0__blk808 * (locals.var_expvbsnvtm_dn10 - locals.var_expvbdnvtm_dn10))) * locals.var_e2ndfactor) + (assign22140_e20036 * locals.var_e2ndfactor_dn10)), ((((locals.var_t0__blk808_dn11 * assign22140_e20035) + (locals.var_t0__blk808 * (locals.var_expvbsnvtm_dn11 - locals.var_expvbdnvtm_dn11))) * locals.var_e2ndfactor) + (assign22140_e20036 * locals.var_e2ndfactor_dn11)), ((((locals.var_t0__blk808_dn12 * assign22140_e20035) + (locals.var_t0__blk808 * (locals.var_expvbsnvtm_dn12 - locals.var_expvbdnvtm_dn12))) * locals.var_e2ndfactor) + (assign22140_e20036 * locals.var_e2ndfactor_dn12)),)
    } else {
        (locals.var_ic_1, locals.var_ic_1_dn3, locals.var_ic_1_dn4, locals.var_ic_1_dn5, locals.var_ic_1_dn6, locals.var_ic_1_dn7, locals.var_ic_1_dn8, locals.var_ic_1_dn9, locals.var_ic_1_dn10, locals.var_ic_1_dn11, locals.var_ic_1_dn12,)
    }
};
        locals.var_ic_1 = assign22140_e20040;
        locals.var_ic_1_dn3 = assign22140_e20040_d_n3;
        locals.var_ic_1_dn4 = assign22140_e20040_d_n4;
        locals.var_ic_1_dn5 = assign22140_e20040_d_n5;
        locals.var_ic_1_dn6 = assign22140_e20040_d_n6;
        locals.var_ic_1_dn7 = assign22140_e20040_d_n7;
        locals.var_ic_1_dn8 = assign22140_e20040_d_n8;
        locals.var_ic_1_dn9 = assign22140_e20040_d_n9;
        locals.var_ic_1_dn10 = assign22140_e20040_d_n10;
        locals.var_ic_1_dn11 = assign22140_e20040_d_n11;
        locals.var_ic_1_dn12 = assign22140_e20040_d_n12;

        let assign22150_e20047: f64 = if ((locals.var_jtuns <= 0.0) && (locals.var_jtund <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1280 = assign22150_e20047;

        let (assign22160_e20053, assign22160_e20053_d_n3, assign22160_e20053_d_n4, assign22160_e20053_d_n5, assign22160_e20053_d_n6, assign22160_e20053_d_n7, assign22160_e20053_d_n8, assign22160_e20053_d_n9, assign22160_e20053_d_n10, assign22160_e20053_d_n11, assign22160_e20053_d_n12,) = {
    if ((locals.var_guard1240 != 0.0) && (locals.var_guard1280 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibd4, locals.var_ibd4_dn3, locals.var_ibd4_dn4, locals.var_ibd4_dn5, locals.var_ibd4_dn6, locals.var_ibd4_dn7, locals.var_ibd4_dn8, locals.var_ibd4_dn9, locals.var_ibd4_dn10, locals.var_ibd4_dn11, locals.var_ibd4_dn12,)
    }
};
        locals.var_ibd4 = assign22160_e20053;
        locals.var_ibd4_dn3 = assign22160_e20053_d_n3;
        locals.var_ibd4_dn4 = assign22160_e20053_d_n4;
        locals.var_ibd4_dn5 = assign22160_e20053_d_n5;
        locals.var_ibd4_dn6 = assign22160_e20053_d_n6;
        locals.var_ibd4_dn7 = assign22160_e20053_d_n7;
        locals.var_ibd4_dn8 = assign22160_e20053_d_n8;
        locals.var_ibd4_dn9 = assign22160_e20053_d_n9;
        locals.var_ibd4_dn10 = assign22160_e20053_d_n10;
        locals.var_ibd4_dn11 = assign22160_e20053_d_n11;
        locals.var_ibd4_dn12 = assign22160_e20053_d_n12;

        let (assign22170_e20059, assign22170_e20059_d_n3, assign22170_e20059_d_n4, assign22170_e20059_d_n5, assign22170_e20059_d_n6, assign22170_e20059_d_n7, assign22170_e20059_d_n8, assign22170_e20059_d_n9, assign22170_e20059_d_n10, assign22170_e20059_d_n11, assign22170_e20059_d_n12,) = {
    if ((locals.var_guard1240 != 0.0) && (locals.var_guard1280 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibs4, locals.var_ibs4_dn3, locals.var_ibs4_dn4, locals.var_ibs4_dn5, locals.var_ibs4_dn6, locals.var_ibs4_dn7, locals.var_ibs4_dn8, locals.var_ibs4_dn9, locals.var_ibs4_dn10, locals.var_ibs4_dn11, locals.var_ibs4_dn12,)
    }
};
        locals.var_ibs4 = assign22170_e20059;
        locals.var_ibs4_dn3 = assign22170_e20059_d_n3;
        locals.var_ibs4_dn4 = assign22170_e20059_d_n4;
        locals.var_ibs4_dn5 = assign22170_e20059_d_n5;
        locals.var_ibs4_dn6 = assign22170_e20059_d_n6;
        locals.var_ibs4_dn7 = assign22170_e20059_d_n7;
        locals.var_ibs4_dn8 = assign22170_e20059_d_n8;
        locals.var_ibs4_dn9 = assign22170_e20059_d_n9;
        locals.var_ibs4_dn10 = assign22170_e20059_d_n10;
        locals.var_ibs4_dn11 = assign22170_e20059_d_n11;
        locals.var_ibs4_dn12 = assign22170_e20059_d_n12;

        let (assign22180_e20068, assign22180_e20068_d_n3, assign22180_e20068_d_n4, assign22180_e20068_d_n5, assign22180_e20068_d_n6, assign22180_e20068_d_n7, assign22180_e20068_d_n8, assign22180_e20068_d_n9, assign22180_e20068_d_n10, assign22180_e20068_d_n11, assign22180_e20068_d_n12,) = {
    if ((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) {
        let assign22180_e20066: f64 = (p.p1043 * locals.var_pparam_b4sointun);
        (assign22180_e20066, (p.p1043 * locals.var_pparam_b4sointun_dn3), (p.p1043 * locals.var_pparam_b4sointun_dn4), (p.p1043 * locals.var_pparam_b4sointun_dn5), (p.p1043 * locals.var_pparam_b4sointun_dn6), (p.p1043 * locals.var_pparam_b4sointun_dn7), (p.p1043 * locals.var_pparam_b4sointun_dn8), (p.p1043 * locals.var_pparam_b4sointun_dn9), (p.p1043 * locals.var_pparam_b4sointun_dn10), (p.p1043 * locals.var_pparam_b4sointun_dn11), (p.p1043 * locals.var_pparam_b4sointun_dn12),)
    } else {
        (locals.var_nvtm2, locals.var_nvtm2_dn3, locals.var_nvtm2_dn4, locals.var_nvtm2_dn5, locals.var_nvtm2_dn6, locals.var_nvtm2_dn7, locals.var_nvtm2_dn8, locals.var_nvtm2_dn9, locals.var_nvtm2_dn10, locals.var_nvtm2_dn11, locals.var_nvtm2_dn12,)
    }
};
        locals.var_nvtm2 = assign22180_e20068;
        locals.var_nvtm2_dn3 = assign22180_e20068_d_n3;
        locals.var_nvtm2_dn4 = assign22180_e20068_d_n4;
        locals.var_nvtm2_dn5 = assign22180_e20068_d_n5;
        locals.var_nvtm2_dn6 = assign22180_e20068_d_n6;
        locals.var_nvtm2_dn7 = assign22180_e20068_d_n7;
        locals.var_nvtm2_dn8 = assign22180_e20068_d_n8;
        locals.var_nvtm2_dn9 = assign22180_e20068_d_n9;
        locals.var_nvtm2_dn10 = assign22180_e20068_d_n10;
        locals.var_nvtm2_dn11 = assign22180_e20068_d_n11;
        locals.var_nvtm2_dn12 = assign22180_e20068_d_n12;

        let assign22190_e20071: f64 = (locals.var_pparam_b4soivtun0 - locals.var_vsbs);
        let assign22190_e20073: f64 = if assign22190_e20071 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard1281 = assign22190_e20073;

    }

    pub(super) fn stamp_transient_block_65(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign22200_e20082, assign22200_e20082_d_n3, assign22200_e20082_d_n4, assign22200_e20082_d_n5, assign22200_e20082_d_n6, assign22200_e20082_d_n7, assign22200_e20082_d_n8, assign22200_e20082_d_n9, assign22200_e20082_d_n10, assign22200_e20082_d_n11, assign22200_e20082_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1281 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign22200_e20082;
        locals.var_t1__blk809_dn3 = assign22200_e20082_d_n3;
        locals.var_t1__blk809_dn4 = assign22200_e20082_d_n4;
        locals.var_t1__blk809_dn5 = assign22200_e20082_d_n5;
        locals.var_t1__blk809_dn6 = assign22200_e20082_d_n6;
        locals.var_t1__blk809_dn7 = assign22200_e20082_d_n7;
        locals.var_t1__blk809_dn8 = assign22200_e20082_d_n8;
        locals.var_t1__blk809_dn9 = assign22200_e20082_d_n9;
        locals.var_t1__blk809_dn10 = assign22200_e20082_d_n10;
        locals.var_t1__blk809_dn11 = assign22200_e20082_d_n11;
        locals.var_t1__blk809_dn12 = assign22200_e20082_d_n12;

        let (assign22210_e20098, assign22210_e20098_d_n3, assign22210_e20098_d_n4, assign22210_e20098_d_n5, assign22210_e20098_d_n6, assign22210_e20098_d_n7, assign22210_e20098_d_n8, assign22210_e20098_d_n9, assign22210_e20098_d_n10, assign22210_e20098_d_n11, assign22210_e20098_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1281 != 0.0)) {
        let assign22210_e20090: f64 = (-locals.var_vsbs);
        let assign22210_e20092: f64 = (assign22210_e20090 / locals.var_nvtm2);
        let assign22210_e20094: f64 = (assign22210_e20092 * locals.var_pparam_b4soivtun0);
        let assign22210_e20096: f64 = (assign22210_e20094 * locals.var_t1__blk809);
        (assign22210_e20096, (((((-((assign22210_e20090 * locals.var_nvtm2_dn3) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign22210_e20092 * locals.var_pparam_b4soivtun0_dn3)) * locals.var_t1__blk809) + (assign22210_e20094 * locals.var_t1__blk809_dn3)), (((((-((assign22210_e20090 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign22210_e20092 * locals.var_pparam_b4soivtun0_dn4)) * locals.var_t1__blk809) + (assign22210_e20094 * locals.var_t1__blk809_dn4)), (((((-((assign22210_e20090 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign22210_e20092 * locals.var_pparam_b4soivtun0_dn5)) * locals.var_t1__blk809) + (assign22210_e20094 * locals.var_t1__blk809_dn5)), (((((-((assign22210_e20090 * locals.var_nvtm2_dn6) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign22210_e20092 * locals.var_pparam_b4soivtun0_dn6)) * locals.var_t1__blk809) + (assign22210_e20094 * locals.var_t1__blk809_dn6)), (((((-((assign22210_e20090 * locals.var_nvtm2_dn7) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign22210_e20092 * locals.var_pparam_b4soivtun0_dn7)) * locals.var_t1__blk809) + (assign22210_e20094 * locals.var_t1__blk809_dn7)), ((((((((-locals.var_vsbs_dn8) * locals.var_nvtm2) - (assign22210_e20090 * locals.var_nvtm2_dn8)) / (locals.var_nvtm2 * locals.var_nvtm2)) * locals.var_pparam_b4soivtun0) + (assign22210_e20092 * locals.var_pparam_b4soivtun0_dn8)) * locals.var_t1__blk809) + (assign22210_e20094 * locals.var_t1__blk809_dn8)), (((((-((assign22210_e20090 * locals.var_nvtm2_dn9) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign22210_e20092 * locals.var_pparam_b4soivtun0_dn9)) * locals.var_t1__blk809) + (assign22210_e20094 * locals.var_t1__blk809_dn9)), (((((-((assign22210_e20090 * locals.var_nvtm2_dn10) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign22210_e20092 * locals.var_pparam_b4soivtun0_dn10)) * locals.var_t1__blk809) + (assign22210_e20094 * locals.var_t1__blk809_dn10)), ((((((((-locals.var_vsbs_dn11) * locals.var_nvtm2) - (assign22210_e20090 * locals.var_nvtm2_dn11)) / (locals.var_nvtm2 * locals.var_nvtm2)) * locals.var_pparam_b4soivtun0) + (assign22210_e20092 * locals.var_pparam_b4soivtun0_dn11)) * locals.var_t1__blk809) + (assign22210_e20094 * locals.var_t1__blk809_dn11)), (((((-((assign22210_e20090 * locals.var_nvtm2_dn12) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign22210_e20092 * locals.var_pparam_b4soivtun0_dn12)) * locals.var_t1__blk809) + (assign22210_e20094 * locals.var_t1__blk809_dn12)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign22210_e20098;
        locals.var_t0__blk808_dn3 = assign22210_e20098_d_n3;
        locals.var_t0__blk808_dn4 = assign22210_e20098_d_n4;
        locals.var_t0__blk808_dn5 = assign22210_e20098_d_n5;
        locals.var_t0__blk808_dn6 = assign22210_e20098_d_n6;
        locals.var_t0__blk808_dn7 = assign22210_e20098_d_n7;
        locals.var_t0__blk808_dn8 = assign22210_e20098_d_n8;
        locals.var_t0__blk808_dn9 = assign22210_e20098_d_n9;
        locals.var_t0__blk808_dn10 = assign22210_e20098_d_n10;
        locals.var_t0__blk808_dn11 = assign22210_e20098_d_n11;
        locals.var_t0__blk808_dn12 = assign22210_e20098_d_n12;

        let assign22220_e20101: f64 = if locals.var_t0__blk808 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1282 = assign22220_e20101;

        let (assign22230_e20118, assign22230_e20118_d_n3, assign22230_e20118_d_n4, assign22230_e20118_d_n5, assign22230_e20118_d_n6, assign22230_e20118_d_n7, assign22230_e20118_d_n8, assign22230_e20118_d_n9, assign22230_e20118_d_n10, assign22230_e20118_d_n11, assign22230_e20118_d_n12,) = {
    if ((((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1281 != 0.0)) && (locals.var_guard1282 != 0.0)) {
        let assign22230_e20113: f64 = (1.0 + locals.var_t0__blk808);
        let assign22230_e20115: f64 = (assign22230_e20113 - 100.0);
        let assign22230_e20116: f64 = (2.688117142e43 * assign22230_e20115);
        (assign22230_e20116, (2.688117142e43 * locals.var_t0__blk808_dn3), (2.688117142e43 * locals.var_t0__blk808_dn4), (2.688117142e43 * locals.var_t0__blk808_dn5), (2.688117142e43 * locals.var_t0__blk808_dn6), (2.688117142e43 * locals.var_t0__blk808_dn7), (2.688117142e43 * locals.var_t0__blk808_dn8), (2.688117142e43 * locals.var_t0__blk808_dn9), (2.688117142e43 * locals.var_t0__blk808_dn10), (2.688117142e43 * locals.var_t0__blk808_dn11), (2.688117142e43 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign22230_e20118;
        locals.var_t1__blk809_dn3 = assign22230_e20118_d_n3;
        locals.var_t1__blk809_dn4 = assign22230_e20118_d_n4;
        locals.var_t1__blk809_dn5 = assign22230_e20118_d_n5;
        locals.var_t1__blk809_dn6 = assign22230_e20118_d_n6;
        locals.var_t1__blk809_dn7 = assign22230_e20118_d_n7;
        locals.var_t1__blk809_dn8 = assign22230_e20118_d_n8;
        locals.var_t1__blk809_dn9 = assign22230_e20118_d_n9;
        locals.var_t1__blk809_dn10 = assign22230_e20118_d_n10;
        locals.var_t1__blk809_dn11 = assign22230_e20118_d_n11;
        locals.var_t1__blk809_dn12 = assign22230_e20118_d_n12;

        let assign22240_e20121: f64 = (-100.0);
        let assign22240_e20122: f64 = if locals.var_t0__blk808 < assign22240_e20121 { 1.0 } else { 0.0 };
        locals.var_guard1283 = assign22240_e20122;

        let (assign22250_e20136, assign22250_e20136_d_n3, assign22250_e20136_d_n4, assign22250_e20136_d_n5, assign22250_e20136_d_n6, assign22250_e20136_d_n7, assign22250_e20136_d_n8, assign22250_e20136_d_n9, assign22250_e20136_d_n10, assign22250_e20136_d_n11, assign22250_e20136_d_n12,) = {
    if (((((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1281 != 0.0)) && (locals.var_guard1282 == 0.0)) && (locals.var_guard1283 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign22250_e20136;
        locals.var_t1__blk809_dn3 = assign22250_e20136_d_n3;
        locals.var_t1__blk809_dn4 = assign22250_e20136_d_n4;
        locals.var_t1__blk809_dn5 = assign22250_e20136_d_n5;
        locals.var_t1__blk809_dn6 = assign22250_e20136_d_n6;
        locals.var_t1__blk809_dn7 = assign22250_e20136_d_n7;
        locals.var_t1__blk809_dn8 = assign22250_e20136_d_n8;
        locals.var_t1__blk809_dn9 = assign22250_e20136_d_n9;
        locals.var_t1__blk809_dn10 = assign22250_e20136_d_n10;
        locals.var_t1__blk809_dn11 = assign22250_e20136_d_n11;
        locals.var_t1__blk809_dn12 = assign22250_e20136_d_n12;

        let (assign22260_e20152, assign22260_e20152_d_n3, assign22260_e20152_d_n4, assign22260_e20152_d_n5, assign22260_e20152_d_n6, assign22260_e20152_d_n7, assign22260_e20152_d_n8, assign22260_e20152_d_n9, assign22260_e20152_d_n10, assign22260_e20152_d_n11, assign22260_e20152_d_n12,) = {
    if (((((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1281 != 0.0)) && (locals.var_guard1282 == 0.0)) && (locals.var_guard1283 == 0.0)) {
        let assign22260_e20150: f64 = (locals.var_t0__blk808).exp();
        (assign22260_e20150, (assign22260_e20150 * locals.var_t0__blk808_dn3), (assign22260_e20150 * locals.var_t0__blk808_dn4), (assign22260_e20150 * locals.var_t0__blk808_dn5), (assign22260_e20150 * locals.var_t0__blk808_dn6), (assign22260_e20150 * locals.var_t0__blk808_dn7), (assign22260_e20150 * locals.var_t0__blk808_dn8), (assign22260_e20150 * locals.var_t0__blk808_dn9), (assign22260_e20150 * locals.var_t0__blk808_dn10), (assign22260_e20150 * locals.var_t0__blk808_dn11), (assign22260_e20150 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign22260_e20152;
        locals.var_t1__blk809_dn3 = assign22260_e20152_d_n3;
        locals.var_t1__blk809_dn4 = assign22260_e20152_d_n4;
        locals.var_t1__blk809_dn5 = assign22260_e20152_d_n5;
        locals.var_t1__blk809_dn6 = assign22260_e20152_d_n6;
        locals.var_t1__blk809_dn7 = assign22260_e20152_d_n7;
        locals.var_t1__blk809_dn8 = assign22260_e20152_d_n8;
        locals.var_t1__blk809_dn9 = assign22260_e20152_d_n9;
        locals.var_t1__blk809_dn10 = assign22260_e20152_d_n10;
        locals.var_t1__blk809_dn11 = assign22260_e20152_d_n11;
        locals.var_t1__blk809_dn12 = assign22260_e20152_d_n12;

        let (assign22270_e20163, assign22270_e20163_d_n3, assign22270_e20163_d_n4, assign22270_e20163_d_n5, assign22270_e20163_d_n6, assign22270_e20163_d_n7, assign22270_e20163_d_n8, assign22270_e20163_d_n9, assign22270_e20163_d_n10, assign22270_e20163_d_n11, assign22270_e20163_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1281 != 0.0)) {
        let assign22270_e20161: f64 = (locals.var_wstsi * locals.var_jtuns);
        (assign22270_e20161, ((locals.var_wstsi_dn3 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn3)), ((locals.var_wstsi_dn4 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn4)), ((locals.var_wstsi_dn5 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn5)), ((locals.var_wstsi_dn6 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn6)), ((locals.var_wstsi_dn7 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn7)), ((locals.var_wstsi_dn8 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn8)), ((locals.var_wstsi_dn9 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn9)), ((locals.var_wstsi_dn10 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn10)), ((locals.var_wstsi_dn11 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn11)), ((locals.var_wstsi_dn12 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn12)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign22270_e20163;
        locals.var_t3__blk811_dn3 = assign22270_e20163_d_n3;
        locals.var_t3__blk811_dn4 = assign22270_e20163_d_n4;
        locals.var_t3__blk811_dn5 = assign22270_e20163_d_n5;
        locals.var_t3__blk811_dn6 = assign22270_e20163_d_n6;
        locals.var_t3__blk811_dn7 = assign22270_e20163_d_n7;
        locals.var_t3__blk811_dn8 = assign22270_e20163_d_n8;
        locals.var_t3__blk811_dn9 = assign22270_e20163_d_n9;
        locals.var_t3__blk811_dn10 = assign22270_e20163_d_n10;
        locals.var_t3__blk811_dn11 = assign22270_e20163_d_n11;
        locals.var_t3__blk811_dn12 = assign22270_e20163_d_n12;

        let (assign22280_e20176, assign22280_e20176_d_n3, assign22280_e20176_d_n4, assign22280_e20176_d_n5, assign22280_e20176_d_n6, assign22280_e20176_d_n7, assign22280_e20176_d_n8, assign22280_e20176_d_n9, assign22280_e20176_d_n10, assign22280_e20176_d_n11, assign22280_e20176_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1281 != 0.0)) {
        let assign22280_e20173: f64 = (1.0 - locals.var_t1__blk809);
        let assign22280_e20174: f64 = (locals.var_t3__blk811 * assign22280_e20173);
        (assign22280_e20174, ((locals.var_t3__blk811_dn3 * assign22280_e20173) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn3))), ((locals.var_t3__blk811_dn4 * assign22280_e20173) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn4))), ((locals.var_t3__blk811_dn5 * assign22280_e20173) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn5))), ((locals.var_t3__blk811_dn6 * assign22280_e20173) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn6))), ((locals.var_t3__blk811_dn7 * assign22280_e20173) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn7))), ((locals.var_t3__blk811_dn8 * assign22280_e20173) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn8))), ((locals.var_t3__blk811_dn9 * assign22280_e20173) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn9))), ((locals.var_t3__blk811_dn10 * assign22280_e20173) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn10))), ((locals.var_t3__blk811_dn11 * assign22280_e20173) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn11))), ((locals.var_t3__blk811_dn12 * assign22280_e20173) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn12))),)
    } else {
        (locals.var_ibs4, locals.var_ibs4_dn3, locals.var_ibs4_dn4, locals.var_ibs4_dn5, locals.var_ibs4_dn6, locals.var_ibs4_dn7, locals.var_ibs4_dn8, locals.var_ibs4_dn9, locals.var_ibs4_dn10, locals.var_ibs4_dn11, locals.var_ibs4_dn12,)
    }
};
        locals.var_ibs4 = assign22280_e20176;
        locals.var_ibs4_dn3 = assign22280_e20176_d_n3;
        locals.var_ibs4_dn4 = assign22280_e20176_d_n4;
        locals.var_ibs4_dn5 = assign22280_e20176_d_n5;
        locals.var_ibs4_dn6 = assign22280_e20176_d_n6;
        locals.var_ibs4_dn7 = assign22280_e20176_d_n7;
        locals.var_ibs4_dn8 = assign22280_e20176_d_n8;
        locals.var_ibs4_dn9 = assign22280_e20176_d_n9;
        locals.var_ibs4_dn10 = assign22280_e20176_d_n10;
        locals.var_ibs4_dn11 = assign22280_e20176_d_n11;
        locals.var_ibs4_dn12 = assign22280_e20176_d_n12;

        let (assign22290_e20190, assign22290_e20190_d_n3, assign22290_e20190_d_n4, assign22290_e20190_d_n5, assign22290_e20190_d_n6, assign22290_e20190_d_n7, assign22290_e20190_d_n8, assign22290_e20190_d_n9, assign22290_e20190_d_n10, assign22290_e20190_d_n11, assign22290_e20190_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1281 == 0.0)) {
        let assign22290_e20187: f64 = (locals.var_pparam_b4soivtun0 - locals.var_vsbs);
        let assign22290_e20188: f64 = (1.0 / assign22290_e20187);
        (assign22290_e20188, (-(locals.var_pparam_b4soivtun0_dn3 / (assign22290_e20187 * assign22290_e20187))), (-(locals.var_pparam_b4soivtun0_dn4 / (assign22290_e20187 * assign22290_e20187))), (-(locals.var_pparam_b4soivtun0_dn5 / (assign22290_e20187 * assign22290_e20187))), (-(locals.var_pparam_b4soivtun0_dn6 / (assign22290_e20187 * assign22290_e20187))), (-(locals.var_pparam_b4soivtun0_dn7 / (assign22290_e20187 * assign22290_e20187))), (-((locals.var_pparam_b4soivtun0_dn8 - locals.var_vsbs_dn8) / (assign22290_e20187 * assign22290_e20187))), (-(locals.var_pparam_b4soivtun0_dn9 / (assign22290_e20187 * assign22290_e20187))), (-(locals.var_pparam_b4soivtun0_dn10 / (assign22290_e20187 * assign22290_e20187))), (-((locals.var_pparam_b4soivtun0_dn11 - locals.var_vsbs_dn11) / (assign22290_e20187 * assign22290_e20187))), (-(locals.var_pparam_b4soivtun0_dn12 / (assign22290_e20187 * assign22290_e20187))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign22290_e20190;
        locals.var_t1__blk809_dn3 = assign22290_e20190_d_n3;
        locals.var_t1__blk809_dn4 = assign22290_e20190_d_n4;
        locals.var_t1__blk809_dn5 = assign22290_e20190_d_n5;
        locals.var_t1__blk809_dn6 = assign22290_e20190_d_n6;
        locals.var_t1__blk809_dn7 = assign22290_e20190_d_n7;
        locals.var_t1__blk809_dn8 = assign22290_e20190_d_n8;
        locals.var_t1__blk809_dn9 = assign22290_e20190_d_n9;
        locals.var_t1__blk809_dn10 = assign22290_e20190_d_n10;
        locals.var_t1__blk809_dn11 = assign22290_e20190_d_n11;
        locals.var_t1__blk809_dn12 = assign22290_e20190_d_n12;

        let (assign22300_e20207, assign22300_e20207_d_n3, assign22300_e20207_d_n4, assign22300_e20207_d_n5, assign22300_e20207_d_n6, assign22300_e20207_d_n7, assign22300_e20207_d_n8, assign22300_e20207_d_n9, assign22300_e20207_d_n10, assign22300_e20207_d_n11, assign22300_e20207_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1281 == 0.0)) {
        let assign22300_e20199: f64 = (-locals.var_vsbs);
        let assign22300_e20201: f64 = (assign22300_e20199 / locals.var_nvtm2);
        let assign22300_e20203: f64 = (assign22300_e20201 * locals.var_pparam_b4soivtun0);
        let assign22300_e20205: f64 = (assign22300_e20203 * locals.var_t1__blk809);
        (assign22300_e20205, (((((-((assign22300_e20199 * locals.var_nvtm2_dn3) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign22300_e20201 * locals.var_pparam_b4soivtun0_dn3)) * locals.var_t1__blk809) + (assign22300_e20203 * locals.var_t1__blk809_dn3)), (((((-((assign22300_e20199 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign22300_e20201 * locals.var_pparam_b4soivtun0_dn4)) * locals.var_t1__blk809) + (assign22300_e20203 * locals.var_t1__blk809_dn4)), (((((-((assign22300_e20199 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign22300_e20201 * locals.var_pparam_b4soivtun0_dn5)) * locals.var_t1__blk809) + (assign22300_e20203 * locals.var_t1__blk809_dn5)), (((((-((assign22300_e20199 * locals.var_nvtm2_dn6) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign22300_e20201 * locals.var_pparam_b4soivtun0_dn6)) * locals.var_t1__blk809) + (assign22300_e20203 * locals.var_t1__blk809_dn6)), (((((-((assign22300_e20199 * locals.var_nvtm2_dn7) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign22300_e20201 * locals.var_pparam_b4soivtun0_dn7)) * locals.var_t1__blk809) + (assign22300_e20203 * locals.var_t1__blk809_dn7)), ((((((((-locals.var_vsbs_dn8) * locals.var_nvtm2) - (assign22300_e20199 * locals.var_nvtm2_dn8)) / (locals.var_nvtm2 * locals.var_nvtm2)) * locals.var_pparam_b4soivtun0) + (assign22300_e20201 * locals.var_pparam_b4soivtun0_dn8)) * locals.var_t1__blk809) + (assign22300_e20203 * locals.var_t1__blk809_dn8)), (((((-((assign22300_e20199 * locals.var_nvtm2_dn9) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign22300_e20201 * locals.var_pparam_b4soivtun0_dn9)) * locals.var_t1__blk809) + (assign22300_e20203 * locals.var_t1__blk809_dn9)), (((((-((assign22300_e20199 * locals.var_nvtm2_dn10) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign22300_e20201 * locals.var_pparam_b4soivtun0_dn10)) * locals.var_t1__blk809) + (assign22300_e20203 * locals.var_t1__blk809_dn10)), ((((((((-locals.var_vsbs_dn11) * locals.var_nvtm2) - (assign22300_e20199 * locals.var_nvtm2_dn11)) / (locals.var_nvtm2 * locals.var_nvtm2)) * locals.var_pparam_b4soivtun0) + (assign22300_e20201 * locals.var_pparam_b4soivtun0_dn11)) * locals.var_t1__blk809) + (assign22300_e20203 * locals.var_t1__blk809_dn11)), (((((-((assign22300_e20199 * locals.var_nvtm2_dn12) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0) + (assign22300_e20201 * locals.var_pparam_b4soivtun0_dn12)) * locals.var_t1__blk809) + (assign22300_e20203 * locals.var_t1__blk809_dn12)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign22300_e20207;
        locals.var_t0__blk808_dn3 = assign22300_e20207_d_n3;
        locals.var_t0__blk808_dn4 = assign22300_e20207_d_n4;
        locals.var_t0__blk808_dn5 = assign22300_e20207_d_n5;
        locals.var_t0__blk808_dn6 = assign22300_e20207_d_n6;
        locals.var_t0__blk808_dn7 = assign22300_e20207_d_n7;
        locals.var_t0__blk808_dn8 = assign22300_e20207_d_n8;
        locals.var_t0__blk808_dn9 = assign22300_e20207_d_n9;
        locals.var_t0__blk808_dn10 = assign22300_e20207_d_n10;
        locals.var_t0__blk808_dn11 = assign22300_e20207_d_n11;
        locals.var_t0__blk808_dn12 = assign22300_e20207_d_n12;

        let assign22310_e20210: f64 = if locals.var_t0__blk808 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1284 = assign22310_e20210;

        let (assign22320_e20228, assign22320_e20228_d_n3, assign22320_e20228_d_n4, assign22320_e20228_d_n5, assign22320_e20228_d_n6, assign22320_e20228_d_n7, assign22320_e20228_d_n8, assign22320_e20228_d_n9, assign22320_e20228_d_n10, assign22320_e20228_d_n11, assign22320_e20228_d_n12,) = {
    if ((((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1281 == 0.0)) && (locals.var_guard1284 != 0.0)) {
        let assign22320_e20223: f64 = (1.0 + locals.var_t0__blk808);
        let assign22320_e20225: f64 = (assign22320_e20223 - 100.0);
        let assign22320_e20226: f64 = (2.688117142e43 * assign22320_e20225);
        (assign22320_e20226, (2.688117142e43 * locals.var_t0__blk808_dn3), (2.688117142e43 * locals.var_t0__blk808_dn4), (2.688117142e43 * locals.var_t0__blk808_dn5), (2.688117142e43 * locals.var_t0__blk808_dn6), (2.688117142e43 * locals.var_t0__blk808_dn7), (2.688117142e43 * locals.var_t0__blk808_dn8), (2.688117142e43 * locals.var_t0__blk808_dn9), (2.688117142e43 * locals.var_t0__blk808_dn10), (2.688117142e43 * locals.var_t0__blk808_dn11), (2.688117142e43 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign22320_e20228;
        locals.var_t1__blk809_dn3 = assign22320_e20228_d_n3;
        locals.var_t1__blk809_dn4 = assign22320_e20228_d_n4;
        locals.var_t1__blk809_dn5 = assign22320_e20228_d_n5;
        locals.var_t1__blk809_dn6 = assign22320_e20228_d_n6;
        locals.var_t1__blk809_dn7 = assign22320_e20228_d_n7;
        locals.var_t1__blk809_dn8 = assign22320_e20228_d_n8;
        locals.var_t1__blk809_dn9 = assign22320_e20228_d_n9;
        locals.var_t1__blk809_dn10 = assign22320_e20228_d_n10;
        locals.var_t1__blk809_dn11 = assign22320_e20228_d_n11;
        locals.var_t1__blk809_dn12 = assign22320_e20228_d_n12;

        let assign22330_e20231: f64 = (-100.0);
        let assign22330_e20232: f64 = if locals.var_t0__blk808 < assign22330_e20231 { 1.0 } else { 0.0 };
        locals.var_guard1285 = assign22330_e20232;

        let (assign22340_e20247, assign22340_e20247_d_n3, assign22340_e20247_d_n4, assign22340_e20247_d_n5, assign22340_e20247_d_n6, assign22340_e20247_d_n7, assign22340_e20247_d_n8, assign22340_e20247_d_n9, assign22340_e20247_d_n10, assign22340_e20247_d_n11, assign22340_e20247_d_n12,) = {
    if (((((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1281 == 0.0)) && (locals.var_guard1284 == 0.0)) && (locals.var_guard1285 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign22340_e20247;
        locals.var_t1__blk809_dn3 = assign22340_e20247_d_n3;
        locals.var_t1__blk809_dn4 = assign22340_e20247_d_n4;
        locals.var_t1__blk809_dn5 = assign22340_e20247_d_n5;
        locals.var_t1__blk809_dn6 = assign22340_e20247_d_n6;
        locals.var_t1__blk809_dn7 = assign22340_e20247_d_n7;
        locals.var_t1__blk809_dn8 = assign22340_e20247_d_n8;
        locals.var_t1__blk809_dn9 = assign22340_e20247_d_n9;
        locals.var_t1__blk809_dn10 = assign22340_e20247_d_n10;
        locals.var_t1__blk809_dn11 = assign22340_e20247_d_n11;
        locals.var_t1__blk809_dn12 = assign22340_e20247_d_n12;

        let (assign22350_e20264, assign22350_e20264_d_n3, assign22350_e20264_d_n4, assign22350_e20264_d_n5, assign22350_e20264_d_n6, assign22350_e20264_d_n7, assign22350_e20264_d_n8, assign22350_e20264_d_n9, assign22350_e20264_d_n10, assign22350_e20264_d_n11, assign22350_e20264_d_n12,) = {
    if (((((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1281 == 0.0)) && (locals.var_guard1284 == 0.0)) && (locals.var_guard1285 == 0.0)) {
        let assign22350_e20262: f64 = (locals.var_t0__blk808).exp();
        (assign22350_e20262, (assign22350_e20262 * locals.var_t0__blk808_dn3), (assign22350_e20262 * locals.var_t0__blk808_dn4), (assign22350_e20262 * locals.var_t0__blk808_dn5), (assign22350_e20262 * locals.var_t0__blk808_dn6), (assign22350_e20262 * locals.var_t0__blk808_dn7), (assign22350_e20262 * locals.var_t0__blk808_dn8), (assign22350_e20262 * locals.var_t0__blk808_dn9), (assign22350_e20262 * locals.var_t0__blk808_dn10), (assign22350_e20262 * locals.var_t0__blk808_dn11), (assign22350_e20262 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign22350_e20264;
        locals.var_t1__blk809_dn3 = assign22350_e20264_d_n3;
        locals.var_t1__blk809_dn4 = assign22350_e20264_d_n4;
        locals.var_t1__blk809_dn5 = assign22350_e20264_d_n5;
        locals.var_t1__blk809_dn6 = assign22350_e20264_d_n6;
        locals.var_t1__blk809_dn7 = assign22350_e20264_d_n7;
        locals.var_t1__blk809_dn8 = assign22350_e20264_d_n8;
        locals.var_t1__blk809_dn9 = assign22350_e20264_d_n9;
        locals.var_t1__blk809_dn10 = assign22350_e20264_d_n10;
        locals.var_t1__blk809_dn11 = assign22350_e20264_d_n11;
        locals.var_t1__blk809_dn12 = assign22350_e20264_d_n12;

        let (assign22360_e20276, assign22360_e20276_d_n3, assign22360_e20276_d_n4, assign22360_e20276_d_n5, assign22360_e20276_d_n6, assign22360_e20276_d_n7, assign22360_e20276_d_n8, assign22360_e20276_d_n9, assign22360_e20276_d_n10, assign22360_e20276_d_n11, assign22360_e20276_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1281 == 0.0)) {
        let assign22360_e20274: f64 = (locals.var_wstsi * locals.var_jtuns);
        (assign22360_e20274, ((locals.var_wstsi_dn3 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn3)), ((locals.var_wstsi_dn4 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn4)), ((locals.var_wstsi_dn5 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn5)), ((locals.var_wstsi_dn6 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn6)), ((locals.var_wstsi_dn7 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn7)), ((locals.var_wstsi_dn8 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn8)), ((locals.var_wstsi_dn9 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn9)), ((locals.var_wstsi_dn10 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn10)), ((locals.var_wstsi_dn11 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn11)), ((locals.var_wstsi_dn12 * locals.var_jtuns) + (locals.var_wstsi * locals.var_jtuns_dn12)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign22360_e20276;
        locals.var_t3__blk811_dn3 = assign22360_e20276_d_n3;
        locals.var_t3__blk811_dn4 = assign22360_e20276_d_n4;
        locals.var_t3__blk811_dn5 = assign22360_e20276_d_n5;
        locals.var_t3__blk811_dn6 = assign22360_e20276_d_n6;
        locals.var_t3__blk811_dn7 = assign22360_e20276_d_n7;
        locals.var_t3__blk811_dn8 = assign22360_e20276_d_n8;
        locals.var_t3__blk811_dn9 = assign22360_e20276_d_n9;
        locals.var_t3__blk811_dn10 = assign22360_e20276_d_n10;
        locals.var_t3__blk811_dn11 = assign22360_e20276_d_n11;
        locals.var_t3__blk811_dn12 = assign22360_e20276_d_n12;

        let (assign22370_e20290, assign22370_e20290_d_n3, assign22370_e20290_d_n4, assign22370_e20290_d_n5, assign22370_e20290_d_n6, assign22370_e20290_d_n7, assign22370_e20290_d_n8, assign22370_e20290_d_n9, assign22370_e20290_d_n10, assign22370_e20290_d_n11, assign22370_e20290_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1281 == 0.0)) {
        let assign22370_e20287: f64 = (1.0 - locals.var_t1__blk809);
        let assign22370_e20288: f64 = (locals.var_t3__blk811 * assign22370_e20287);
        (assign22370_e20288, ((locals.var_t3__blk811_dn3 * assign22370_e20287) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn3))), ((locals.var_t3__blk811_dn4 * assign22370_e20287) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn4))), ((locals.var_t3__blk811_dn5 * assign22370_e20287) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn5))), ((locals.var_t3__blk811_dn6 * assign22370_e20287) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn6))), ((locals.var_t3__blk811_dn7 * assign22370_e20287) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn7))), ((locals.var_t3__blk811_dn8 * assign22370_e20287) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn8))), ((locals.var_t3__blk811_dn9 * assign22370_e20287) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn9))), ((locals.var_t3__blk811_dn10 * assign22370_e20287) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn10))), ((locals.var_t3__blk811_dn11 * assign22370_e20287) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn11))), ((locals.var_t3__blk811_dn12 * assign22370_e20287) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn12))),)
    } else {
        (locals.var_ibs4, locals.var_ibs4_dn3, locals.var_ibs4_dn4, locals.var_ibs4_dn5, locals.var_ibs4_dn6, locals.var_ibs4_dn7, locals.var_ibs4_dn8, locals.var_ibs4_dn9, locals.var_ibs4_dn10, locals.var_ibs4_dn11, locals.var_ibs4_dn12,)
    }
};
        locals.var_ibs4 = assign22370_e20290;
        locals.var_ibs4_dn3 = assign22370_e20290_d_n3;
        locals.var_ibs4_dn4 = assign22370_e20290_d_n4;
        locals.var_ibs4_dn5 = assign22370_e20290_d_n5;
        locals.var_ibs4_dn6 = assign22370_e20290_d_n6;
        locals.var_ibs4_dn7 = assign22370_e20290_d_n7;
        locals.var_ibs4_dn8 = assign22370_e20290_d_n8;
        locals.var_ibs4_dn9 = assign22370_e20290_d_n9;
        locals.var_ibs4_dn10 = assign22370_e20290_d_n10;
        locals.var_ibs4_dn11 = assign22370_e20290_d_n11;
        locals.var_ibs4_dn12 = assign22370_e20290_d_n12;

        let (assign22380_e20299, assign22380_e20299_d_n3, assign22380_e20299_d_n4, assign22380_e20299_d_n5, assign22380_e20299_d_n6, assign22380_e20299_d_n7, assign22380_e20299_d_n8, assign22380_e20299_d_n9, assign22380_e20299_d_n10, assign22380_e20299_d_n11, assign22380_e20299_d_n12,) = {
    if ((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) {
        let assign22380_e20297: f64 = (p.p1043 * locals.var_pparam_b4sointund);
        (assign22380_e20297, (p.p1043 * locals.var_pparam_b4sointund_dn3), (p.p1043 * locals.var_pparam_b4sointund_dn4), (p.p1043 * locals.var_pparam_b4sointund_dn5), (p.p1043 * locals.var_pparam_b4sointund_dn6), (p.p1043 * locals.var_pparam_b4sointund_dn7), (p.p1043 * locals.var_pparam_b4sointund_dn8), (p.p1043 * locals.var_pparam_b4sointund_dn9), (p.p1043 * locals.var_pparam_b4sointund_dn10), (p.p1043 * locals.var_pparam_b4sointund_dn11), (p.p1043 * locals.var_pparam_b4sointund_dn12),)
    } else {
        (locals.var_nvtm2, locals.var_nvtm2_dn3, locals.var_nvtm2_dn4, locals.var_nvtm2_dn5, locals.var_nvtm2_dn6, locals.var_nvtm2_dn7, locals.var_nvtm2_dn8, locals.var_nvtm2_dn9, locals.var_nvtm2_dn10, locals.var_nvtm2_dn11, locals.var_nvtm2_dn12,)
    }
};
        locals.var_nvtm2 = assign22380_e20299;
        locals.var_nvtm2_dn3 = assign22380_e20299_d_n3;
        locals.var_nvtm2_dn4 = assign22380_e20299_d_n4;
        locals.var_nvtm2_dn5 = assign22380_e20299_d_n5;
        locals.var_nvtm2_dn6 = assign22380_e20299_d_n6;
        locals.var_nvtm2_dn7 = assign22380_e20299_d_n7;
        locals.var_nvtm2_dn8 = assign22380_e20299_d_n8;
        locals.var_nvtm2_dn9 = assign22380_e20299_d_n9;
        locals.var_nvtm2_dn10 = assign22380_e20299_d_n10;
        locals.var_nvtm2_dn11 = assign22380_e20299_d_n11;
        locals.var_nvtm2_dn12 = assign22380_e20299_d_n12;

        let assign22390_e20302: f64 = (locals.var_pparam_b4soivtun0d - locals.var_vdbd);
        let assign22390_e20304: f64 = if assign22390_e20302 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard1286 = assign22390_e20304;

        let (assign22400_e20313, assign22400_e20313_d_n3, assign22400_e20313_d_n4, assign22400_e20313_d_n5, assign22400_e20313_d_n6, assign22400_e20313_d_n7, assign22400_e20313_d_n8, assign22400_e20313_d_n9, assign22400_e20313_d_n10, assign22400_e20313_d_n11, assign22400_e20313_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1286 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign22400_e20313;
        locals.var_t1__blk809_dn3 = assign22400_e20313_d_n3;
        locals.var_t1__blk809_dn4 = assign22400_e20313_d_n4;
        locals.var_t1__blk809_dn5 = assign22400_e20313_d_n5;
        locals.var_t1__blk809_dn6 = assign22400_e20313_d_n6;
        locals.var_t1__blk809_dn7 = assign22400_e20313_d_n7;
        locals.var_t1__blk809_dn8 = assign22400_e20313_d_n8;
        locals.var_t1__blk809_dn9 = assign22400_e20313_d_n9;
        locals.var_t1__blk809_dn10 = assign22400_e20313_d_n10;
        locals.var_t1__blk809_dn11 = assign22400_e20313_d_n11;
        locals.var_t1__blk809_dn12 = assign22400_e20313_d_n12;

        let (assign22410_e20329, assign22410_e20329_d_n3, assign22410_e20329_d_n4, assign22410_e20329_d_n5, assign22410_e20329_d_n6, assign22410_e20329_d_n7, assign22410_e20329_d_n8, assign22410_e20329_d_n9, assign22410_e20329_d_n10, assign22410_e20329_d_n11, assign22410_e20329_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1286 != 0.0)) {
        let assign22410_e20321: f64 = (-locals.var_vdbd);
        let assign22410_e20323: f64 = (assign22410_e20321 / locals.var_nvtm2);
        let assign22410_e20325: f64 = (assign22410_e20323 * locals.var_pparam_b4soivtun0d);
        let assign22410_e20327: f64 = (assign22410_e20325 * locals.var_t1__blk809);
        (assign22410_e20327, (((((-((assign22410_e20321 * locals.var_nvtm2_dn3) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign22410_e20323 * locals.var_pparam_b4soivtun0d_dn3)) * locals.var_t1__blk809) + (assign22410_e20325 * locals.var_t1__blk809_dn3)), (((((-((assign22410_e20321 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign22410_e20323 * locals.var_pparam_b4soivtun0d_dn4)) * locals.var_t1__blk809) + (assign22410_e20325 * locals.var_t1__blk809_dn4)), (((((-((assign22410_e20321 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign22410_e20323 * locals.var_pparam_b4soivtun0d_dn5)) * locals.var_t1__blk809) + (assign22410_e20325 * locals.var_t1__blk809_dn5)), (((((-((assign22410_e20321 * locals.var_nvtm2_dn6) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign22410_e20323 * locals.var_pparam_b4soivtun0d_dn6)) * locals.var_t1__blk809) + (assign22410_e20325 * locals.var_t1__blk809_dn6)), ((((((((-locals.var_vdbd_dn7) * locals.var_nvtm2) - (assign22410_e20321 * locals.var_nvtm2_dn7)) / (locals.var_nvtm2 * locals.var_nvtm2)) * locals.var_pparam_b4soivtun0d) + (assign22410_e20323 * locals.var_pparam_b4soivtun0d_dn7)) * locals.var_t1__blk809) + (assign22410_e20325 * locals.var_t1__blk809_dn7)), (((((-((assign22410_e20321 * locals.var_nvtm2_dn8) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign22410_e20323 * locals.var_pparam_b4soivtun0d_dn8)) * locals.var_t1__blk809) + (assign22410_e20325 * locals.var_t1__blk809_dn8)), (((((-((assign22410_e20321 * locals.var_nvtm2_dn9) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign22410_e20323 * locals.var_pparam_b4soivtun0d_dn9)) * locals.var_t1__blk809) + (assign22410_e20325 * locals.var_t1__blk809_dn9)), (((((-((assign22410_e20321 * locals.var_nvtm2_dn10) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign22410_e20323 * locals.var_pparam_b4soivtun0d_dn10)) * locals.var_t1__blk809) + (assign22410_e20325 * locals.var_t1__blk809_dn10)), (((((-((assign22410_e20321 * locals.var_nvtm2_dn11) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign22410_e20323 * locals.var_pparam_b4soivtun0d_dn11)) * locals.var_t1__blk809) + (assign22410_e20325 * locals.var_t1__blk809_dn11)), ((((((((-locals.var_vdbd_dn12) * locals.var_nvtm2) - (assign22410_e20321 * locals.var_nvtm2_dn12)) / (locals.var_nvtm2 * locals.var_nvtm2)) * locals.var_pparam_b4soivtun0d) + (assign22410_e20323 * locals.var_pparam_b4soivtun0d_dn12)) * locals.var_t1__blk809) + (assign22410_e20325 * locals.var_t1__blk809_dn12)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign22410_e20329;
        locals.var_t0__blk808_dn3 = assign22410_e20329_d_n3;
        locals.var_t0__blk808_dn4 = assign22410_e20329_d_n4;
        locals.var_t0__blk808_dn5 = assign22410_e20329_d_n5;
        locals.var_t0__blk808_dn6 = assign22410_e20329_d_n6;
        locals.var_t0__blk808_dn7 = assign22410_e20329_d_n7;
        locals.var_t0__blk808_dn8 = assign22410_e20329_d_n8;
        locals.var_t0__blk808_dn9 = assign22410_e20329_d_n9;
        locals.var_t0__blk808_dn10 = assign22410_e20329_d_n10;
        locals.var_t0__blk808_dn11 = assign22410_e20329_d_n11;
        locals.var_t0__blk808_dn12 = assign22410_e20329_d_n12;

        let assign22420_e20332: f64 = if locals.var_t0__blk808 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1287 = assign22420_e20332;

        let (assign22430_e20349, assign22430_e20349_d_n3, assign22430_e20349_d_n4, assign22430_e20349_d_n5, assign22430_e20349_d_n6, assign22430_e20349_d_n7, assign22430_e20349_d_n8, assign22430_e20349_d_n9, assign22430_e20349_d_n10, assign22430_e20349_d_n11, assign22430_e20349_d_n12,) = {
    if ((((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1286 != 0.0)) && (locals.var_guard1287 != 0.0)) {
        let assign22430_e20344: f64 = (1.0 + locals.var_t0__blk808);
        let assign22430_e20346: f64 = (assign22430_e20344 - 100.0);
        let assign22430_e20347: f64 = (2.688117142e43 * assign22430_e20346);
        (assign22430_e20347, (2.688117142e43 * locals.var_t0__blk808_dn3), (2.688117142e43 * locals.var_t0__blk808_dn4), (2.688117142e43 * locals.var_t0__blk808_dn5), (2.688117142e43 * locals.var_t0__blk808_dn6), (2.688117142e43 * locals.var_t0__blk808_dn7), (2.688117142e43 * locals.var_t0__blk808_dn8), (2.688117142e43 * locals.var_t0__blk808_dn9), (2.688117142e43 * locals.var_t0__blk808_dn10), (2.688117142e43 * locals.var_t0__blk808_dn11), (2.688117142e43 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign22430_e20349;
        locals.var_t1__blk809_dn3 = assign22430_e20349_d_n3;
        locals.var_t1__blk809_dn4 = assign22430_e20349_d_n4;
        locals.var_t1__blk809_dn5 = assign22430_e20349_d_n5;
        locals.var_t1__blk809_dn6 = assign22430_e20349_d_n6;
        locals.var_t1__blk809_dn7 = assign22430_e20349_d_n7;
        locals.var_t1__blk809_dn8 = assign22430_e20349_d_n8;
        locals.var_t1__blk809_dn9 = assign22430_e20349_d_n9;
        locals.var_t1__blk809_dn10 = assign22430_e20349_d_n10;
        locals.var_t1__blk809_dn11 = assign22430_e20349_d_n11;
        locals.var_t1__blk809_dn12 = assign22430_e20349_d_n12;

        let assign22440_e20352: f64 = (-100.0);
        let assign22440_e20353: f64 = if locals.var_t0__blk808 < assign22440_e20352 { 1.0 } else { 0.0 };
        locals.var_guard1288 = assign22440_e20353;

        let (assign22450_e20367, assign22450_e20367_d_n3, assign22450_e20367_d_n4, assign22450_e20367_d_n5, assign22450_e20367_d_n6, assign22450_e20367_d_n7, assign22450_e20367_d_n8, assign22450_e20367_d_n9, assign22450_e20367_d_n10, assign22450_e20367_d_n11, assign22450_e20367_d_n12,) = {
    if (((((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1286 != 0.0)) && (locals.var_guard1287 == 0.0)) && (locals.var_guard1288 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign22450_e20367;
        locals.var_t1__blk809_dn3 = assign22450_e20367_d_n3;
        locals.var_t1__blk809_dn4 = assign22450_e20367_d_n4;
        locals.var_t1__blk809_dn5 = assign22450_e20367_d_n5;
        locals.var_t1__blk809_dn6 = assign22450_e20367_d_n6;
        locals.var_t1__blk809_dn7 = assign22450_e20367_d_n7;
        locals.var_t1__blk809_dn8 = assign22450_e20367_d_n8;
        locals.var_t1__blk809_dn9 = assign22450_e20367_d_n9;
        locals.var_t1__blk809_dn10 = assign22450_e20367_d_n10;
        locals.var_t1__blk809_dn11 = assign22450_e20367_d_n11;
        locals.var_t1__blk809_dn12 = assign22450_e20367_d_n12;

        let (assign22460_e20383, assign22460_e20383_d_n3, assign22460_e20383_d_n4, assign22460_e20383_d_n5, assign22460_e20383_d_n6, assign22460_e20383_d_n7, assign22460_e20383_d_n8, assign22460_e20383_d_n9, assign22460_e20383_d_n10, assign22460_e20383_d_n11, assign22460_e20383_d_n12,) = {
    if (((((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1286 != 0.0)) && (locals.var_guard1287 == 0.0)) && (locals.var_guard1288 == 0.0)) {
        let assign22460_e20381: f64 = (locals.var_t0__blk808).exp();
        (assign22460_e20381, (assign22460_e20381 * locals.var_t0__blk808_dn3), (assign22460_e20381 * locals.var_t0__blk808_dn4), (assign22460_e20381 * locals.var_t0__blk808_dn5), (assign22460_e20381 * locals.var_t0__blk808_dn6), (assign22460_e20381 * locals.var_t0__blk808_dn7), (assign22460_e20381 * locals.var_t0__blk808_dn8), (assign22460_e20381 * locals.var_t0__blk808_dn9), (assign22460_e20381 * locals.var_t0__blk808_dn10), (assign22460_e20381 * locals.var_t0__blk808_dn11), (assign22460_e20381 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign22460_e20383;
        locals.var_t1__blk809_dn3 = assign22460_e20383_d_n3;
        locals.var_t1__blk809_dn4 = assign22460_e20383_d_n4;
        locals.var_t1__blk809_dn5 = assign22460_e20383_d_n5;
        locals.var_t1__blk809_dn6 = assign22460_e20383_d_n6;
        locals.var_t1__blk809_dn7 = assign22460_e20383_d_n7;
        locals.var_t1__blk809_dn8 = assign22460_e20383_d_n8;
        locals.var_t1__blk809_dn9 = assign22460_e20383_d_n9;
        locals.var_t1__blk809_dn10 = assign22460_e20383_d_n10;
        locals.var_t1__blk809_dn11 = assign22460_e20383_d_n11;
        locals.var_t1__blk809_dn12 = assign22460_e20383_d_n12;

        let (assign22470_e20394, assign22470_e20394_d_n3, assign22470_e20394_d_n4, assign22470_e20394_d_n5, assign22470_e20394_d_n6, assign22470_e20394_d_n7, assign22470_e20394_d_n8, assign22470_e20394_d_n9, assign22470_e20394_d_n10, assign22470_e20394_d_n11, assign22470_e20394_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1286 != 0.0)) {
        let assign22470_e20392: f64 = (locals.var_wdtsi * locals.var_jtund);
        (assign22470_e20392, ((locals.var_wdtsi_dn3 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn3)), ((locals.var_wdtsi_dn4 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn4)), ((locals.var_wdtsi_dn5 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn5)), ((locals.var_wdtsi_dn6 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn6)), ((locals.var_wdtsi_dn7 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn7)), ((locals.var_wdtsi_dn8 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn8)), ((locals.var_wdtsi_dn9 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn9)), ((locals.var_wdtsi_dn10 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn10)), ((locals.var_wdtsi_dn11 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn11)), ((locals.var_wdtsi_dn12 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn12)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign22470_e20394;
        locals.var_t3__blk811_dn3 = assign22470_e20394_d_n3;
        locals.var_t3__blk811_dn4 = assign22470_e20394_d_n4;
        locals.var_t3__blk811_dn5 = assign22470_e20394_d_n5;
        locals.var_t3__blk811_dn6 = assign22470_e20394_d_n6;
        locals.var_t3__blk811_dn7 = assign22470_e20394_d_n7;
        locals.var_t3__blk811_dn8 = assign22470_e20394_d_n8;
        locals.var_t3__blk811_dn9 = assign22470_e20394_d_n9;
        locals.var_t3__blk811_dn10 = assign22470_e20394_d_n10;
        locals.var_t3__blk811_dn11 = assign22470_e20394_d_n11;
        locals.var_t3__blk811_dn12 = assign22470_e20394_d_n12;

        let (assign22480_e20407, assign22480_e20407_d_n3, assign22480_e20407_d_n4, assign22480_e20407_d_n5, assign22480_e20407_d_n6, assign22480_e20407_d_n7, assign22480_e20407_d_n8, assign22480_e20407_d_n9, assign22480_e20407_d_n10, assign22480_e20407_d_n11, assign22480_e20407_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1286 != 0.0)) {
        let assign22480_e20404: f64 = (1.0 - locals.var_t1__blk809);
        let assign22480_e20405: f64 = (locals.var_t3__blk811 * assign22480_e20404);
        (assign22480_e20405, ((locals.var_t3__blk811_dn3 * assign22480_e20404) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn3))), ((locals.var_t3__blk811_dn4 * assign22480_e20404) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn4))), ((locals.var_t3__blk811_dn5 * assign22480_e20404) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn5))), ((locals.var_t3__blk811_dn6 * assign22480_e20404) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn6))), ((locals.var_t3__blk811_dn7 * assign22480_e20404) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn7))), ((locals.var_t3__blk811_dn8 * assign22480_e20404) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn8))), ((locals.var_t3__blk811_dn9 * assign22480_e20404) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn9))), ((locals.var_t3__blk811_dn10 * assign22480_e20404) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn10))), ((locals.var_t3__blk811_dn11 * assign22480_e20404) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn11))), ((locals.var_t3__blk811_dn12 * assign22480_e20404) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn12))),)
    } else {
        (locals.var_ibd4, locals.var_ibd4_dn3, locals.var_ibd4_dn4, locals.var_ibd4_dn5, locals.var_ibd4_dn6, locals.var_ibd4_dn7, locals.var_ibd4_dn8, locals.var_ibd4_dn9, locals.var_ibd4_dn10, locals.var_ibd4_dn11, locals.var_ibd4_dn12,)
    }
};
        locals.var_ibd4 = assign22480_e20407;
        locals.var_ibd4_dn3 = assign22480_e20407_d_n3;
        locals.var_ibd4_dn4 = assign22480_e20407_d_n4;
        locals.var_ibd4_dn5 = assign22480_e20407_d_n5;
        locals.var_ibd4_dn6 = assign22480_e20407_d_n6;
        locals.var_ibd4_dn7 = assign22480_e20407_d_n7;
        locals.var_ibd4_dn8 = assign22480_e20407_d_n8;
        locals.var_ibd4_dn9 = assign22480_e20407_d_n9;
        locals.var_ibd4_dn10 = assign22480_e20407_d_n10;
        locals.var_ibd4_dn11 = assign22480_e20407_d_n11;
        locals.var_ibd4_dn12 = assign22480_e20407_d_n12;

        let (assign22490_e20421, assign22490_e20421_d_n3, assign22490_e20421_d_n4, assign22490_e20421_d_n5, assign22490_e20421_d_n6, assign22490_e20421_d_n7, assign22490_e20421_d_n8, assign22490_e20421_d_n9, assign22490_e20421_d_n10, assign22490_e20421_d_n11, assign22490_e20421_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1286 == 0.0)) {
        let assign22490_e20418: f64 = (locals.var_pparam_b4soivtun0d - locals.var_vdbd);
        let assign22490_e20419: f64 = (1.0 / assign22490_e20418);
        (assign22490_e20419, (-(locals.var_pparam_b4soivtun0d_dn3 / (assign22490_e20418 * assign22490_e20418))), (-(locals.var_pparam_b4soivtun0d_dn4 / (assign22490_e20418 * assign22490_e20418))), (-(locals.var_pparam_b4soivtun0d_dn5 / (assign22490_e20418 * assign22490_e20418))), (-(locals.var_pparam_b4soivtun0d_dn6 / (assign22490_e20418 * assign22490_e20418))), (-((locals.var_pparam_b4soivtun0d_dn7 - locals.var_vdbd_dn7) / (assign22490_e20418 * assign22490_e20418))), (-(locals.var_pparam_b4soivtun0d_dn8 / (assign22490_e20418 * assign22490_e20418))), (-(locals.var_pparam_b4soivtun0d_dn9 / (assign22490_e20418 * assign22490_e20418))), (-(locals.var_pparam_b4soivtun0d_dn10 / (assign22490_e20418 * assign22490_e20418))), (-(locals.var_pparam_b4soivtun0d_dn11 / (assign22490_e20418 * assign22490_e20418))), (-((locals.var_pparam_b4soivtun0d_dn12 - locals.var_vdbd_dn12) / (assign22490_e20418 * assign22490_e20418))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign22490_e20421;
        locals.var_t1__blk809_dn3 = assign22490_e20421_d_n3;
        locals.var_t1__blk809_dn4 = assign22490_e20421_d_n4;
        locals.var_t1__blk809_dn5 = assign22490_e20421_d_n5;
        locals.var_t1__blk809_dn6 = assign22490_e20421_d_n6;
        locals.var_t1__blk809_dn7 = assign22490_e20421_d_n7;
        locals.var_t1__blk809_dn8 = assign22490_e20421_d_n8;
        locals.var_t1__blk809_dn9 = assign22490_e20421_d_n9;
        locals.var_t1__blk809_dn10 = assign22490_e20421_d_n10;
        locals.var_t1__blk809_dn11 = assign22490_e20421_d_n11;
        locals.var_t1__blk809_dn12 = assign22490_e20421_d_n12;

    }

    pub(super) fn stamp_transient_block_66(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign22500_e20438, assign22500_e20438_d_n3, assign22500_e20438_d_n4, assign22500_e20438_d_n5, assign22500_e20438_d_n6, assign22500_e20438_d_n7, assign22500_e20438_d_n8, assign22500_e20438_d_n9, assign22500_e20438_d_n10, assign22500_e20438_d_n11, assign22500_e20438_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1286 == 0.0)) {
        let assign22500_e20430: f64 = (-locals.var_vdbd);
        let assign22500_e20432: f64 = (assign22500_e20430 / locals.var_nvtm2);
        let assign22500_e20434: f64 = (assign22500_e20432 * locals.var_pparam_b4soivtun0d);
        let assign22500_e20436: f64 = (assign22500_e20434 * locals.var_t1__blk809);
        (assign22500_e20436, (((((-((assign22500_e20430 * locals.var_nvtm2_dn3) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign22500_e20432 * locals.var_pparam_b4soivtun0d_dn3)) * locals.var_t1__blk809) + (assign22500_e20434 * locals.var_t1__blk809_dn3)), (((((-((assign22500_e20430 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign22500_e20432 * locals.var_pparam_b4soivtun0d_dn4)) * locals.var_t1__blk809) + (assign22500_e20434 * locals.var_t1__blk809_dn4)), (((((-((assign22500_e20430 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign22500_e20432 * locals.var_pparam_b4soivtun0d_dn5)) * locals.var_t1__blk809) + (assign22500_e20434 * locals.var_t1__blk809_dn5)), (((((-((assign22500_e20430 * locals.var_nvtm2_dn6) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign22500_e20432 * locals.var_pparam_b4soivtun0d_dn6)) * locals.var_t1__blk809) + (assign22500_e20434 * locals.var_t1__blk809_dn6)), ((((((((-locals.var_vdbd_dn7) * locals.var_nvtm2) - (assign22500_e20430 * locals.var_nvtm2_dn7)) / (locals.var_nvtm2 * locals.var_nvtm2)) * locals.var_pparam_b4soivtun0d) + (assign22500_e20432 * locals.var_pparam_b4soivtun0d_dn7)) * locals.var_t1__blk809) + (assign22500_e20434 * locals.var_t1__blk809_dn7)), (((((-((assign22500_e20430 * locals.var_nvtm2_dn8) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign22500_e20432 * locals.var_pparam_b4soivtun0d_dn8)) * locals.var_t1__blk809) + (assign22500_e20434 * locals.var_t1__blk809_dn8)), (((((-((assign22500_e20430 * locals.var_nvtm2_dn9) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign22500_e20432 * locals.var_pparam_b4soivtun0d_dn9)) * locals.var_t1__blk809) + (assign22500_e20434 * locals.var_t1__blk809_dn9)), (((((-((assign22500_e20430 * locals.var_nvtm2_dn10) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign22500_e20432 * locals.var_pparam_b4soivtun0d_dn10)) * locals.var_t1__blk809) + (assign22500_e20434 * locals.var_t1__blk809_dn10)), (((((-((assign22500_e20430 * locals.var_nvtm2_dn11) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_pparam_b4soivtun0d) + (assign22500_e20432 * locals.var_pparam_b4soivtun0d_dn11)) * locals.var_t1__blk809) + (assign22500_e20434 * locals.var_t1__blk809_dn11)), ((((((((-locals.var_vdbd_dn12) * locals.var_nvtm2) - (assign22500_e20430 * locals.var_nvtm2_dn12)) / (locals.var_nvtm2 * locals.var_nvtm2)) * locals.var_pparam_b4soivtun0d) + (assign22500_e20432 * locals.var_pparam_b4soivtun0d_dn12)) * locals.var_t1__blk809) + (assign22500_e20434 * locals.var_t1__blk809_dn12)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign22500_e20438;
        locals.var_t0__blk808_dn3 = assign22500_e20438_d_n3;
        locals.var_t0__blk808_dn4 = assign22500_e20438_d_n4;
        locals.var_t0__blk808_dn5 = assign22500_e20438_d_n5;
        locals.var_t0__blk808_dn6 = assign22500_e20438_d_n6;
        locals.var_t0__blk808_dn7 = assign22500_e20438_d_n7;
        locals.var_t0__blk808_dn8 = assign22500_e20438_d_n8;
        locals.var_t0__blk808_dn9 = assign22500_e20438_d_n9;
        locals.var_t0__blk808_dn10 = assign22500_e20438_d_n10;
        locals.var_t0__blk808_dn11 = assign22500_e20438_d_n11;
        locals.var_t0__blk808_dn12 = assign22500_e20438_d_n12;

        let assign22510_e20441: f64 = if locals.var_t0__blk808 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1289 = assign22510_e20441;

        let (assign22520_e20459, assign22520_e20459_d_n3, assign22520_e20459_d_n4, assign22520_e20459_d_n5, assign22520_e20459_d_n6, assign22520_e20459_d_n7, assign22520_e20459_d_n8, assign22520_e20459_d_n9, assign22520_e20459_d_n10, assign22520_e20459_d_n11, assign22520_e20459_d_n12,) = {
    if ((((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1286 == 0.0)) && (locals.var_guard1289 != 0.0)) {
        let assign22520_e20454: f64 = (1.0 + locals.var_t0__blk808);
        let assign22520_e20456: f64 = (assign22520_e20454 - 100.0);
        let assign22520_e20457: f64 = (2.688117142e43 * assign22520_e20456);
        (assign22520_e20457, (2.688117142e43 * locals.var_t0__blk808_dn3), (2.688117142e43 * locals.var_t0__blk808_dn4), (2.688117142e43 * locals.var_t0__blk808_dn5), (2.688117142e43 * locals.var_t0__blk808_dn6), (2.688117142e43 * locals.var_t0__blk808_dn7), (2.688117142e43 * locals.var_t0__blk808_dn8), (2.688117142e43 * locals.var_t0__blk808_dn9), (2.688117142e43 * locals.var_t0__blk808_dn10), (2.688117142e43 * locals.var_t0__blk808_dn11), (2.688117142e43 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign22520_e20459;
        locals.var_t1__blk809_dn3 = assign22520_e20459_d_n3;
        locals.var_t1__blk809_dn4 = assign22520_e20459_d_n4;
        locals.var_t1__blk809_dn5 = assign22520_e20459_d_n5;
        locals.var_t1__blk809_dn6 = assign22520_e20459_d_n6;
        locals.var_t1__blk809_dn7 = assign22520_e20459_d_n7;
        locals.var_t1__blk809_dn8 = assign22520_e20459_d_n8;
        locals.var_t1__blk809_dn9 = assign22520_e20459_d_n9;
        locals.var_t1__blk809_dn10 = assign22520_e20459_d_n10;
        locals.var_t1__blk809_dn11 = assign22520_e20459_d_n11;
        locals.var_t1__blk809_dn12 = assign22520_e20459_d_n12;

        let assign22530_e20462: f64 = (-100.0);
        let assign22530_e20463: f64 = if locals.var_t0__blk808 < assign22530_e20462 { 1.0 } else { 0.0 };
        locals.var_guard1290 = assign22530_e20463;

        let (assign22540_e20478, assign22540_e20478_d_n3, assign22540_e20478_d_n4, assign22540_e20478_d_n5, assign22540_e20478_d_n6, assign22540_e20478_d_n7, assign22540_e20478_d_n8, assign22540_e20478_d_n9, assign22540_e20478_d_n10, assign22540_e20478_d_n11, assign22540_e20478_d_n12,) = {
    if (((((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1286 == 0.0)) && (locals.var_guard1289 == 0.0)) && (locals.var_guard1290 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign22540_e20478;
        locals.var_t1__blk809_dn3 = assign22540_e20478_d_n3;
        locals.var_t1__blk809_dn4 = assign22540_e20478_d_n4;
        locals.var_t1__blk809_dn5 = assign22540_e20478_d_n5;
        locals.var_t1__blk809_dn6 = assign22540_e20478_d_n6;
        locals.var_t1__blk809_dn7 = assign22540_e20478_d_n7;
        locals.var_t1__blk809_dn8 = assign22540_e20478_d_n8;
        locals.var_t1__blk809_dn9 = assign22540_e20478_d_n9;
        locals.var_t1__blk809_dn10 = assign22540_e20478_d_n10;
        locals.var_t1__blk809_dn11 = assign22540_e20478_d_n11;
        locals.var_t1__blk809_dn12 = assign22540_e20478_d_n12;

        let (assign22550_e20495, assign22550_e20495_d_n3, assign22550_e20495_d_n4, assign22550_e20495_d_n5, assign22550_e20495_d_n6, assign22550_e20495_d_n7, assign22550_e20495_d_n8, assign22550_e20495_d_n9, assign22550_e20495_d_n10, assign22550_e20495_d_n11, assign22550_e20495_d_n12,) = {
    if (((((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1286 == 0.0)) && (locals.var_guard1289 == 0.0)) && (locals.var_guard1290 == 0.0)) {
        let assign22550_e20493: f64 = (locals.var_t0__blk808).exp();
        (assign22550_e20493, (assign22550_e20493 * locals.var_t0__blk808_dn3), (assign22550_e20493 * locals.var_t0__blk808_dn4), (assign22550_e20493 * locals.var_t0__blk808_dn5), (assign22550_e20493 * locals.var_t0__blk808_dn6), (assign22550_e20493 * locals.var_t0__blk808_dn7), (assign22550_e20493 * locals.var_t0__blk808_dn8), (assign22550_e20493 * locals.var_t0__blk808_dn9), (assign22550_e20493 * locals.var_t0__blk808_dn10), (assign22550_e20493 * locals.var_t0__blk808_dn11), (assign22550_e20493 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign22550_e20495;
        locals.var_t1__blk809_dn3 = assign22550_e20495_d_n3;
        locals.var_t1__blk809_dn4 = assign22550_e20495_d_n4;
        locals.var_t1__blk809_dn5 = assign22550_e20495_d_n5;
        locals.var_t1__blk809_dn6 = assign22550_e20495_d_n6;
        locals.var_t1__blk809_dn7 = assign22550_e20495_d_n7;
        locals.var_t1__blk809_dn8 = assign22550_e20495_d_n8;
        locals.var_t1__blk809_dn9 = assign22550_e20495_d_n9;
        locals.var_t1__blk809_dn10 = assign22550_e20495_d_n10;
        locals.var_t1__blk809_dn11 = assign22550_e20495_d_n11;
        locals.var_t1__blk809_dn12 = assign22550_e20495_d_n12;

        let (assign22560_e20507, assign22560_e20507_d_n3, assign22560_e20507_d_n4, assign22560_e20507_d_n5, assign22560_e20507_d_n6, assign22560_e20507_d_n7, assign22560_e20507_d_n8, assign22560_e20507_d_n9, assign22560_e20507_d_n10, assign22560_e20507_d_n11, assign22560_e20507_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1286 == 0.0)) {
        let assign22560_e20505: f64 = (locals.var_wdtsi * locals.var_jtund);
        (assign22560_e20505, ((locals.var_wdtsi_dn3 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn3)), ((locals.var_wdtsi_dn4 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn4)), ((locals.var_wdtsi_dn5 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn5)), ((locals.var_wdtsi_dn6 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn6)), ((locals.var_wdtsi_dn7 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn7)), ((locals.var_wdtsi_dn8 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn8)), ((locals.var_wdtsi_dn9 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn9)), ((locals.var_wdtsi_dn10 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn10)), ((locals.var_wdtsi_dn11 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn11)), ((locals.var_wdtsi_dn12 * locals.var_jtund) + (locals.var_wdtsi * locals.var_jtund_dn12)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign22560_e20507;
        locals.var_t3__blk811_dn3 = assign22560_e20507_d_n3;
        locals.var_t3__blk811_dn4 = assign22560_e20507_d_n4;
        locals.var_t3__blk811_dn5 = assign22560_e20507_d_n5;
        locals.var_t3__blk811_dn6 = assign22560_e20507_d_n6;
        locals.var_t3__blk811_dn7 = assign22560_e20507_d_n7;
        locals.var_t3__blk811_dn8 = assign22560_e20507_d_n8;
        locals.var_t3__blk811_dn9 = assign22560_e20507_d_n9;
        locals.var_t3__blk811_dn10 = assign22560_e20507_d_n10;
        locals.var_t3__blk811_dn11 = assign22560_e20507_d_n11;
        locals.var_t3__blk811_dn12 = assign22560_e20507_d_n12;

        let (assign22570_e20521, assign22570_e20521_d_n3, assign22570_e20521_d_n4, assign22570_e20521_d_n5, assign22570_e20521_d_n6, assign22570_e20521_d_n7, assign22570_e20521_d_n8, assign22570_e20521_d_n9, assign22570_e20521_d_n10, assign22570_e20521_d_n11, assign22570_e20521_d_n12,) = {
    if (((locals.var_guard1240 != 0.0) && (locals.var_guard1280 == 0.0)) && (locals.var_guard1286 == 0.0)) {
        let assign22570_e20518: f64 = (1.0 - locals.var_t1__blk809);
        let assign22570_e20519: f64 = (locals.var_t3__blk811 * assign22570_e20518);
        (assign22570_e20519, ((locals.var_t3__blk811_dn3 * assign22570_e20518) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn3))), ((locals.var_t3__blk811_dn4 * assign22570_e20518) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn4))), ((locals.var_t3__blk811_dn5 * assign22570_e20518) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn5))), ((locals.var_t3__blk811_dn6 * assign22570_e20518) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn6))), ((locals.var_t3__blk811_dn7 * assign22570_e20518) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn7))), ((locals.var_t3__blk811_dn8 * assign22570_e20518) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn8))), ((locals.var_t3__blk811_dn9 * assign22570_e20518) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn9))), ((locals.var_t3__blk811_dn10 * assign22570_e20518) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn10))), ((locals.var_t3__blk811_dn11 * assign22570_e20518) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn11))), ((locals.var_t3__blk811_dn12 * assign22570_e20518) + (locals.var_t3__blk811 * (-locals.var_t1__blk809_dn12))),)
    } else {
        (locals.var_ibd4, locals.var_ibd4_dn3, locals.var_ibd4_dn4, locals.var_ibd4_dn5, locals.var_ibd4_dn6, locals.var_ibd4_dn7, locals.var_ibd4_dn8, locals.var_ibd4_dn9, locals.var_ibd4_dn10, locals.var_ibd4_dn11, locals.var_ibd4_dn12,)
    }
};
        locals.var_ibd4 = assign22570_e20521;
        locals.var_ibd4_dn3 = assign22570_e20521_d_n3;
        locals.var_ibd4_dn4 = assign22570_e20521_d_n4;
        locals.var_ibd4_dn5 = assign22570_e20521_d_n5;
        locals.var_ibd4_dn6 = assign22570_e20521_d_n6;
        locals.var_ibd4_dn7 = assign22570_e20521_d_n7;
        locals.var_ibd4_dn8 = assign22570_e20521_d_n8;
        locals.var_ibd4_dn9 = assign22570_e20521_d_n9;
        locals.var_ibd4_dn10 = assign22570_e20521_d_n10;
        locals.var_ibd4_dn11 = assign22570_e20521_d_n11;
        locals.var_ibd4_dn12 = assign22570_e20521_d_n12;

        let (assign22580_e20531, assign22580_e20531_d_n3, assign22580_e20531_d_n4, assign22580_e20531_d_n5, assign22580_e20531_d_n6, assign22580_e20531_d_n7, assign22580_e20531_d_n8, assign22580_e20531_d_n9, assign22580_e20531_d_n10, assign22580_e20531_d_n11, assign22580_e20531_d_n12,) = {
    if (locals.var_guard1240 != 0.0) {
        let assign22580_e20525: f64 = (locals.var_ibs1 + locals.var_ibs2);
        let assign22580_e20527: f64 = (assign22580_e20525 + locals.var_ibs3);
        let assign22580_e20529: f64 = (assign22580_e20527 + locals.var_ibs4);
        (assign22580_e20529, (((locals.var_ibs1_dn3 + locals.var_ibs2_dn3) + locals.var_ibs3_dn3) + locals.var_ibs4_dn3), (((locals.var_ibs1_dn4 + locals.var_ibs2_dn4) + locals.var_ibs3_dn4) + locals.var_ibs4_dn4), (((locals.var_ibs1_dn5 + locals.var_ibs2_dn5) + locals.var_ibs3_dn5) + locals.var_ibs4_dn5), (((locals.var_ibs1_dn6 + locals.var_ibs2_dn6) + locals.var_ibs3_dn6) + locals.var_ibs4_dn6), (((locals.var_ibs1_dn7 + locals.var_ibs2_dn7) + locals.var_ibs3_dn7) + locals.var_ibs4_dn7), (((locals.var_ibs1_dn8 + locals.var_ibs2_dn8) + locals.var_ibs3_dn8) + locals.var_ibs4_dn8), (((locals.var_ibs1_dn9 + locals.var_ibs2_dn9) + locals.var_ibs3_dn9) + locals.var_ibs4_dn9), (((locals.var_ibs1_dn10 + locals.var_ibs2_dn10) + locals.var_ibs3_dn10) + locals.var_ibs4_dn10), (((locals.var_ibs1_dn11 + locals.var_ibs2_dn11) + locals.var_ibs3_dn11) + locals.var_ibs4_dn11), (((locals.var_ibs1_dn12 + locals.var_ibs2_dn12) + locals.var_ibs3_dn12) + locals.var_ibs4_dn12),)
    } else {
        (locals.var_ibs_1, locals.var_ibs_1_dn3, locals.var_ibs_1_dn4, locals.var_ibs_1_dn5, locals.var_ibs_1_dn6, locals.var_ibs_1_dn7, locals.var_ibs_1_dn8, locals.var_ibs_1_dn9, locals.var_ibs_1_dn10, locals.var_ibs_1_dn11, locals.var_ibs_1_dn12,)
    }
};
        locals.var_ibs_1 = assign22580_e20531;
        locals.var_ibs_1_dn3 = assign22580_e20531_d_n3;
        locals.var_ibs_1_dn4 = assign22580_e20531_d_n4;
        locals.var_ibs_1_dn5 = assign22580_e20531_d_n5;
        locals.var_ibs_1_dn6 = assign22580_e20531_d_n6;
        locals.var_ibs_1_dn7 = assign22580_e20531_d_n7;
        locals.var_ibs_1_dn8 = assign22580_e20531_d_n8;
        locals.var_ibs_1_dn9 = assign22580_e20531_d_n9;
        locals.var_ibs_1_dn10 = assign22580_e20531_d_n10;
        locals.var_ibs_1_dn11 = assign22580_e20531_d_n11;
        locals.var_ibs_1_dn12 = assign22580_e20531_d_n12;

        let (assign22590_e20541, assign22590_e20541_d_n3, assign22590_e20541_d_n4, assign22590_e20541_d_n5, assign22590_e20541_d_n6, assign22590_e20541_d_n7, assign22590_e20541_d_n8, assign22590_e20541_d_n9, assign22590_e20541_d_n10, assign22590_e20541_d_n11, assign22590_e20541_d_n12,) = {
    if (locals.var_guard1240 != 0.0) {
        let assign22590_e20535: f64 = (locals.var_ibd1 + locals.var_ibd2);
        let assign22590_e20537: f64 = (assign22590_e20535 + locals.var_ibd3);
        let assign22590_e20539: f64 = (assign22590_e20537 + locals.var_ibd4);
        (assign22590_e20539, (((locals.var_ibd1_dn3 + locals.var_ibd2_dn3) + locals.var_ibd3_dn3) + locals.var_ibd4_dn3), (((locals.var_ibd1_dn4 + locals.var_ibd2_dn4) + locals.var_ibd3_dn4) + locals.var_ibd4_dn4), (((locals.var_ibd1_dn5 + locals.var_ibd2_dn5) + locals.var_ibd3_dn5) + locals.var_ibd4_dn5), (((locals.var_ibd1_dn6 + locals.var_ibd2_dn6) + locals.var_ibd3_dn6) + locals.var_ibd4_dn6), (((locals.var_ibd1_dn7 + locals.var_ibd2_dn7) + locals.var_ibd3_dn7) + locals.var_ibd4_dn7), (((locals.var_ibd1_dn8 + locals.var_ibd2_dn8) + locals.var_ibd3_dn8) + locals.var_ibd4_dn8), (((locals.var_ibd1_dn9 + locals.var_ibd2_dn9) + locals.var_ibd3_dn9) + locals.var_ibd4_dn9), (((locals.var_ibd1_dn10 + locals.var_ibd2_dn10) + locals.var_ibd3_dn10) + locals.var_ibd4_dn10), (((locals.var_ibd1_dn11 + locals.var_ibd2_dn11) + locals.var_ibd3_dn11) + locals.var_ibd4_dn11), (((locals.var_ibd1_dn12 + locals.var_ibd2_dn12) + locals.var_ibd3_dn12) + locals.var_ibd4_dn12),)
    } else {
        (locals.var_ibd_1, locals.var_ibd_1_dn3, locals.var_ibd_1_dn4, locals.var_ibd_1_dn5, locals.var_ibd_1_dn6, locals.var_ibd_1_dn7, locals.var_ibd_1_dn8, locals.var_ibd_1_dn9, locals.var_ibd_1_dn10, locals.var_ibd_1_dn11, locals.var_ibd_1_dn12,)
    }
};
        locals.var_ibd_1 = assign22590_e20541;
        locals.var_ibd_1_dn3 = assign22590_e20541_d_n3;
        locals.var_ibd_1_dn4 = assign22590_e20541_d_n4;
        locals.var_ibd_1_dn5 = assign22590_e20541_d_n5;
        locals.var_ibd_1_dn6 = assign22590_e20541_d_n6;
        locals.var_ibd_1_dn7 = assign22590_e20541_d_n7;
        locals.var_ibd_1_dn8 = assign22590_e20541_d_n8;
        locals.var_ibd_1_dn9 = assign22590_e20541_d_n9;
        locals.var_ibd_1_dn10 = assign22590_e20541_d_n10;
        locals.var_ibd_1_dn11 = assign22590_e20541_d_n11;
        locals.var_ibd_1_dn12 = assign22590_e20541_d_n12;

        let (assign22600_e20546, assign22600_e20546_d_n3, assign22600_e20546_d_n4, assign22600_e20546_d_n5, assign22600_e20546_d_n6, assign22600_e20546_d_n7, assign22600_e20546_d_n8, assign22600_e20546_d_n9, assign22600_e20546_d_n10, assign22600_e20546_d_n11, assign22600_e20546_d_n12,) = {
    if (locals.var_guard1240 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igidl_1, locals.var_igidl_1_dn3, locals.var_igidl_1_dn4, locals.var_igidl_1_dn5, locals.var_igidl_1_dn6, locals.var_igidl_1_dn7, locals.var_igidl_1_dn8, locals.var_igidl_1_dn9, locals.var_igidl_1_dn10, locals.var_igidl_1_dn11, locals.var_igidl_1_dn12,)
    }
};
        locals.var_igidl_1 = assign22600_e20546;
        locals.var_igidl_1_dn3 = assign22600_e20546_d_n3;
        locals.var_igidl_1_dn4 = assign22600_e20546_d_n4;
        locals.var_igidl_1_dn5 = assign22600_e20546_d_n5;
        locals.var_igidl_1_dn6 = assign22600_e20546_d_n6;
        locals.var_igidl_1_dn7 = assign22600_e20546_d_n7;
        locals.var_igidl_1_dn8 = assign22600_e20546_d_n8;
        locals.var_igidl_1_dn9 = assign22600_e20546_d_n9;
        locals.var_igidl_1_dn10 = assign22600_e20546_d_n10;
        locals.var_igidl_1_dn11 = assign22600_e20546_d_n11;
        locals.var_igidl_1_dn12 = assign22600_e20546_d_n12;

        let (assign22610_e20551, assign22610_e20551_d_n3, assign22610_e20551_d_n4, assign22610_e20551_d_n5, assign22610_e20551_d_n6, assign22610_e20551_d_n7, assign22610_e20551_d_n8, assign22610_e20551_d_n9, assign22610_e20551_d_n10, assign22610_e20551_d_n11, assign22610_e20551_d_n12,) = {
    if (locals.var_guard1240 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igisl_1, locals.var_igisl_1_dn3, locals.var_igisl_1_dn4, locals.var_igisl_1_dn5, locals.var_igisl_1_dn6, locals.var_igisl_1_dn7, locals.var_igisl_1_dn8, locals.var_igisl_1_dn9, locals.var_igisl_1_dn10, locals.var_igisl_1_dn11, locals.var_igisl_1_dn12,)
    }
};
        locals.var_igisl_1 = assign22610_e20551;
        locals.var_igisl_1_dn3 = assign22610_e20551_d_n3;
        locals.var_igisl_1_dn4 = assign22610_e20551_d_n4;
        locals.var_igisl_1_dn5 = assign22610_e20551_d_n5;
        locals.var_igisl_1_dn6 = assign22610_e20551_d_n6;
        locals.var_igisl_1_dn7 = assign22610_e20551_d_n7;
        locals.var_igisl_1_dn8 = assign22610_e20551_d_n8;
        locals.var_igisl_1_dn9 = assign22610_e20551_d_n9;
        locals.var_igisl_1_dn10 = assign22610_e20551_d_n10;
        locals.var_igisl_1_dn11 = assign22610_e20551_d_n11;
        locals.var_igisl_1_dn12 = assign22610_e20551_d_n12;

        let (assign22620_e20556, assign22620_e20556_d_n3, assign22620_e20556_d_n4, assign22620_e20556_d_n5, assign22620_e20556_d_n6, assign22620_e20556_d_n7, assign22620_e20556_d_n8, assign22620_e20556_d_n9, assign22620_e20556_d_n10, assign22620_e20556_d_n11, assign22620_e20556_d_n12,) = {
    if (locals.var_guard1240 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibs_1, locals.var_ibs_1_dn3, locals.var_ibs_1_dn4, locals.var_ibs_1_dn5, locals.var_ibs_1_dn6, locals.var_ibs_1_dn7, locals.var_ibs_1_dn8, locals.var_ibs_1_dn9, locals.var_ibs_1_dn10, locals.var_ibs_1_dn11, locals.var_ibs_1_dn12,)
    }
};
        locals.var_ibs_1 = assign22620_e20556;
        locals.var_ibs_1_dn3 = assign22620_e20556_d_n3;
        locals.var_ibs_1_dn4 = assign22620_e20556_d_n4;
        locals.var_ibs_1_dn5 = assign22620_e20556_d_n5;
        locals.var_ibs_1_dn6 = assign22620_e20556_d_n6;
        locals.var_ibs_1_dn7 = assign22620_e20556_d_n7;
        locals.var_ibs_1_dn8 = assign22620_e20556_d_n8;
        locals.var_ibs_1_dn9 = assign22620_e20556_d_n9;
        locals.var_ibs_1_dn10 = assign22620_e20556_d_n10;
        locals.var_ibs_1_dn11 = assign22620_e20556_d_n11;
        locals.var_ibs_1_dn12 = assign22620_e20556_d_n12;

        let (assign22630_e20561, assign22630_e20561_d_n3, assign22630_e20561_d_n4, assign22630_e20561_d_n5, assign22630_e20561_d_n6, assign22630_e20561_d_n7, assign22630_e20561_d_n8, assign22630_e20561_d_n9, assign22630_e20561_d_n10, assign22630_e20561_d_n11, assign22630_e20561_d_n12,) = {
    if (locals.var_guard1240 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibd_1, locals.var_ibd_1_dn3, locals.var_ibd_1_dn4, locals.var_ibd_1_dn5, locals.var_ibd_1_dn6, locals.var_ibd_1_dn7, locals.var_ibd_1_dn8, locals.var_ibd_1_dn9, locals.var_ibd_1_dn10, locals.var_ibd_1_dn11, locals.var_ibd_1_dn12,)
    }
};
        locals.var_ibd_1 = assign22630_e20561;
        locals.var_ibd_1_dn3 = assign22630_e20561_d_n3;
        locals.var_ibd_1_dn4 = assign22630_e20561_d_n4;
        locals.var_ibd_1_dn5 = assign22630_e20561_d_n5;
        locals.var_ibd_1_dn6 = assign22630_e20561_d_n6;
        locals.var_ibd_1_dn7 = assign22630_e20561_d_n7;
        locals.var_ibd_1_dn8 = assign22630_e20561_d_n8;
        locals.var_ibd_1_dn9 = assign22630_e20561_d_n9;
        locals.var_ibd_1_dn10 = assign22630_e20561_d_n10;
        locals.var_ibd_1_dn11 = assign22630_e20561_d_n11;
        locals.var_ibd_1_dn12 = assign22630_e20561_d_n12;

        let (assign22640_e20566, assign22640_e20566_d_n3, assign22640_e20566_d_n4, assign22640_e20566_d_n5, assign22640_e20566_d_n6, assign22640_e20566_d_n7, assign22640_e20566_d_n8, assign22640_e20566_d_n9, assign22640_e20566_d_n10, assign22640_e20566_d_n11, assign22640_e20566_d_n12,) = {
    if (locals.var_guard1240 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibsdif, locals.var_ibsdif_dn3, locals.var_ibsdif_dn4, locals.var_ibsdif_dn5, locals.var_ibsdif_dn6, locals.var_ibsdif_dn7, locals.var_ibsdif_dn8, locals.var_ibsdif_dn9, locals.var_ibsdif_dn10, locals.var_ibsdif_dn11, locals.var_ibsdif_dn12,)
    }
};
        locals.var_ibsdif = assign22640_e20566;
        locals.var_ibsdif_dn3 = assign22640_e20566_d_n3;
        locals.var_ibsdif_dn4 = assign22640_e20566_d_n4;
        locals.var_ibsdif_dn5 = assign22640_e20566_d_n5;
        locals.var_ibsdif_dn6 = assign22640_e20566_d_n6;
        locals.var_ibsdif_dn7 = assign22640_e20566_d_n7;
        locals.var_ibsdif_dn8 = assign22640_e20566_d_n8;
        locals.var_ibsdif_dn9 = assign22640_e20566_d_n9;
        locals.var_ibsdif_dn10 = assign22640_e20566_d_n10;
        locals.var_ibsdif_dn11 = assign22640_e20566_d_n11;
        locals.var_ibsdif_dn12 = assign22640_e20566_d_n12;

        let (assign22650_e20571, assign22650_e20571_d_n3, assign22650_e20571_d_n4, assign22650_e20571_d_n5, assign22650_e20571_d_n6, assign22650_e20571_d_n7, assign22650_e20571_d_n8, assign22650_e20571_d_n9, assign22650_e20571_d_n10, assign22650_e20571_d_n11, assign22650_e20571_d_n12,) = {
    if (locals.var_guard1240 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibddif, locals.var_ibddif_dn3, locals.var_ibddif_dn4, locals.var_ibddif_dn5, locals.var_ibddif_dn6, locals.var_ibddif_dn7, locals.var_ibddif_dn8, locals.var_ibddif_dn9, locals.var_ibddif_dn10, locals.var_ibddif_dn11, locals.var_ibddif_dn12,)
    }
};
        locals.var_ibddif = assign22650_e20571;
        locals.var_ibddif_dn3 = assign22650_e20571_d_n3;
        locals.var_ibddif_dn4 = assign22650_e20571_d_n4;
        locals.var_ibddif_dn5 = assign22650_e20571_d_n5;
        locals.var_ibddif_dn6 = assign22650_e20571_d_n6;
        locals.var_ibddif_dn7 = assign22650_e20571_d_n7;
        locals.var_ibddif_dn8 = assign22650_e20571_d_n8;
        locals.var_ibddif_dn9 = assign22650_e20571_d_n9;
        locals.var_ibddif_dn10 = assign22650_e20571_d_n10;
        locals.var_ibddif_dn11 = assign22650_e20571_d_n11;
        locals.var_ibddif_dn12 = assign22650_e20571_d_n12;

        let (assign22660_e20576, assign22660_e20576_d_n3, assign22660_e20576_d_n4, assign22660_e20576_d_n5, assign22660_e20576_d_n6, assign22660_e20576_d_n7, assign22660_e20576_d_n8, assign22660_e20576_d_n9, assign22660_e20576_d_n10, assign22660_e20576_d_n11, assign22660_e20576_d_n12,) = {
    if (locals.var_guard1240 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ic_1, locals.var_ic_1_dn3, locals.var_ic_1_dn4, locals.var_ic_1_dn5, locals.var_ic_1_dn6, locals.var_ic_1_dn7, locals.var_ic_1_dn8, locals.var_ic_1_dn9, locals.var_ic_1_dn10, locals.var_ic_1_dn11, locals.var_ic_1_dn12,)
    }
};
        locals.var_ic_1 = assign22660_e20576;
        locals.var_ic_1_dn3 = assign22660_e20576_d_n3;
        locals.var_ic_1_dn4 = assign22660_e20576_d_n4;
        locals.var_ic_1_dn5 = assign22660_e20576_d_n5;
        locals.var_ic_1_dn6 = assign22660_e20576_d_n6;
        locals.var_ic_1_dn7 = assign22660_e20576_d_n7;
        locals.var_ic_1_dn8 = assign22660_e20576_d_n8;
        locals.var_ic_1_dn9 = assign22660_e20576_d_n9;
        locals.var_ic_1_dn10 = assign22660_e20576_d_n10;
        locals.var_ic_1_dn11 = assign22660_e20576_d_n11;
        locals.var_ic_1_dn12 = assign22660_e20576_d_n12;

        let (assign22670_e20585, assign22670_e20585_d_n4, assign22670_e20585_d_n5, assign22670_e20585_d_n6,) = {
    if (locals.var_tempratio > 1e-38) {
        let assign22670_e20582: f64 = (locals.var_tempratio).ln();
        (assign22670_e20582, (locals.var_tempratio_dn4 / locals.var_tempratio), (locals.var_tempratio_dn5 / locals.var_tempratio), (locals.var_tempratio_dn6 / locals.var_tempratio),)
    } else {
        let assign22670_e20584: f64 = (-87.49823353377374);
        (assign22670_e20584, 0.0, 0.0, 0.0,)
    }
};
        let assign22670_e20586: f64 = (locals.var_pparam_b4soiigt * assign22670_e20585);
        let assign22670_e20587: f64 = (assign22670_e20586).exp();
        locals.var_igtemp = assign22670_e20587;
        locals.var_igtemp_dn3 = (assign22670_e20587 * (locals.var_pparam_b4soiigt_dn3 * assign22670_e20585));
        locals.var_igtemp_dn4 = (assign22670_e20587 * ((locals.var_pparam_b4soiigt_dn4 * assign22670_e20585) + (locals.var_pparam_b4soiigt * assign22670_e20585_d_n4)));
        locals.var_igtemp_dn5 = (assign22670_e20587 * ((locals.var_pparam_b4soiigt_dn5 * assign22670_e20585) + (locals.var_pparam_b4soiigt * assign22670_e20585_d_n5)));
        locals.var_igtemp_dn6 = (assign22670_e20587 * ((locals.var_pparam_b4soiigt_dn6 * assign22670_e20585) + (locals.var_pparam_b4soiigt * assign22670_e20585_d_n6)));
        locals.var_igtemp_dn7 = (assign22670_e20587 * (locals.var_pparam_b4soiigt_dn7 * assign22670_e20585));
        locals.var_igtemp_dn8 = (assign22670_e20587 * (locals.var_pparam_b4soiigt_dn8 * assign22670_e20585));
        locals.var_igtemp_dn9 = (assign22670_e20587 * (locals.var_pparam_b4soiigt_dn9 * assign22670_e20585));
        locals.var_igtemp_dn10 = (assign22670_e20587 * (locals.var_pparam_b4soiigt_dn10 * assign22670_e20585));
        locals.var_igtemp_dn11 = (assign22670_e20587 * (locals.var_pparam_b4soiigt_dn11 * assign22670_e20585));
        locals.var_igtemp_dn12 = (assign22670_e20587 * (locals.var_pparam_b4soiigt_dn12 * assign22670_e20585));

        let assign22680_e20591: f64 = (locals.var_pparam_b4soiaigc1 * locals.var_trm1);
        let assign22680_e20592: f64 = (locals.var_pparam_b4soiaigc + assign22680_e20591);
        locals.var_pparam_b4soiaigc = assign22680_e20592;
        locals.var_pparam_b4soiaigc_dn3 = (locals.var_pparam_b4soiaigc_dn3 + (locals.var_pparam_b4soiaigc1_dn3 * locals.var_trm1));
        locals.var_pparam_b4soiaigc_dn4 = (locals.var_pparam_b4soiaigc_dn4 + ((locals.var_pparam_b4soiaigc1_dn4 * locals.var_trm1) + (locals.var_pparam_b4soiaigc1 * locals.var_trm1_dn4)));
        locals.var_pparam_b4soiaigc_dn5 = (locals.var_pparam_b4soiaigc_dn5 + ((locals.var_pparam_b4soiaigc1_dn5 * locals.var_trm1) + (locals.var_pparam_b4soiaigc1 * locals.var_trm1_dn5)));
        locals.var_pparam_b4soiaigc_dn6 = (locals.var_pparam_b4soiaigc_dn6 + ((locals.var_pparam_b4soiaigc1_dn6 * locals.var_trm1) + (locals.var_pparam_b4soiaigc1 * locals.var_trm1_dn6)));
        locals.var_pparam_b4soiaigc_dn7 = (locals.var_pparam_b4soiaigc_dn7 + (locals.var_pparam_b4soiaigc1_dn7 * locals.var_trm1));
        locals.var_pparam_b4soiaigc_dn8 = (locals.var_pparam_b4soiaigc_dn8 + (locals.var_pparam_b4soiaigc1_dn8 * locals.var_trm1));
        locals.var_pparam_b4soiaigc_dn9 = (locals.var_pparam_b4soiaigc_dn9 + (locals.var_pparam_b4soiaigc1_dn9 * locals.var_trm1));
        locals.var_pparam_b4soiaigc_dn10 = (locals.var_pparam_b4soiaigc_dn10 + (locals.var_pparam_b4soiaigc1_dn10 * locals.var_trm1));
        locals.var_pparam_b4soiaigc_dn11 = (locals.var_pparam_b4soiaigc_dn11 + (locals.var_pparam_b4soiaigc1_dn11 * locals.var_trm1));
        locals.var_pparam_b4soiaigc_dn12 = (locals.var_pparam_b4soiaigc_dn12 + (locals.var_pparam_b4soiaigc1_dn12 * locals.var_trm1));

        let assign22690_e20596: f64 = (locals.var_pparam_b4soiaigsd1 * locals.var_trm1);
        let assign22690_e20597: f64 = (locals.var_pparam_b4soiaigsd + assign22690_e20596);
        locals.var_pparam_b4soiaigsd = assign22690_e20597;
        locals.var_pparam_b4soiaigsd_dn3 = (locals.var_pparam_b4soiaigsd_dn3 + (locals.var_pparam_b4soiaigsd1_dn3 * locals.var_trm1));
        locals.var_pparam_b4soiaigsd_dn4 = (locals.var_pparam_b4soiaigsd_dn4 + ((locals.var_pparam_b4soiaigsd1_dn4 * locals.var_trm1) + (locals.var_pparam_b4soiaigsd1 * locals.var_trm1_dn4)));
        locals.var_pparam_b4soiaigsd_dn5 = (locals.var_pparam_b4soiaigsd_dn5 + ((locals.var_pparam_b4soiaigsd1_dn5 * locals.var_trm1) + (locals.var_pparam_b4soiaigsd1 * locals.var_trm1_dn5)));
        locals.var_pparam_b4soiaigsd_dn6 = (locals.var_pparam_b4soiaigsd_dn6 + ((locals.var_pparam_b4soiaigsd1_dn6 * locals.var_trm1) + (locals.var_pparam_b4soiaigsd1 * locals.var_trm1_dn6)));
        locals.var_pparam_b4soiaigsd_dn7 = (locals.var_pparam_b4soiaigsd_dn7 + (locals.var_pparam_b4soiaigsd1_dn7 * locals.var_trm1));
        locals.var_pparam_b4soiaigsd_dn8 = (locals.var_pparam_b4soiaigsd_dn8 + (locals.var_pparam_b4soiaigsd1_dn8 * locals.var_trm1));
        locals.var_pparam_b4soiaigsd_dn9 = (locals.var_pparam_b4soiaigsd_dn9 + (locals.var_pparam_b4soiaigsd1_dn9 * locals.var_trm1));
        locals.var_pparam_b4soiaigsd_dn10 = (locals.var_pparam_b4soiaigsd_dn10 + (locals.var_pparam_b4soiaigsd1_dn10 * locals.var_trm1));
        locals.var_pparam_b4soiaigsd_dn11 = (locals.var_pparam_b4soiaigsd_dn11 + (locals.var_pparam_b4soiaigsd1_dn11 * locals.var_trm1));
        locals.var_pparam_b4soiaigsd_dn12 = (locals.var_pparam_b4soiaigsd_dn12 + (locals.var_pparam_b4soiaigsd1_dn12 * locals.var_trm1));

        let assign22700_e20601: f64 = (locals.var_pparam_b4soialphagb1_t * locals.var_trm1);
        let assign22700_e20602: f64 = (locals.var_pparam_b4soialphagb1 + assign22700_e20601);
        locals.var_pparam_b4soialphagb1 = assign22700_e20602;
        locals.var_pparam_b4soialphagb1_dn3 = (locals.var_pparam_b4soialphagb1_dn3 + (locals.var_pparam_b4soialphagb1_t_dn3 * locals.var_trm1));
        locals.var_pparam_b4soialphagb1_dn4 = (locals.var_pparam_b4soialphagb1_dn4 + ((locals.var_pparam_b4soialphagb1_t_dn4 * locals.var_trm1) + (locals.var_pparam_b4soialphagb1_t * locals.var_trm1_dn4)));
        locals.var_pparam_b4soialphagb1_dn5 = (locals.var_pparam_b4soialphagb1_dn5 + ((locals.var_pparam_b4soialphagb1_t_dn5 * locals.var_trm1) + (locals.var_pparam_b4soialphagb1_t * locals.var_trm1_dn5)));
        locals.var_pparam_b4soialphagb1_dn6 = (locals.var_pparam_b4soialphagb1_dn6 + ((locals.var_pparam_b4soialphagb1_t_dn6 * locals.var_trm1) + (locals.var_pparam_b4soialphagb1_t * locals.var_trm1_dn6)));
        locals.var_pparam_b4soialphagb1_dn7 = (locals.var_pparam_b4soialphagb1_dn7 + (locals.var_pparam_b4soialphagb1_t_dn7 * locals.var_trm1));
        locals.var_pparam_b4soialphagb1_dn8 = (locals.var_pparam_b4soialphagb1_dn8 + (locals.var_pparam_b4soialphagb1_t_dn8 * locals.var_trm1));
        locals.var_pparam_b4soialphagb1_dn9 = (locals.var_pparam_b4soialphagb1_dn9 + (locals.var_pparam_b4soialphagb1_t_dn9 * locals.var_trm1));
        locals.var_pparam_b4soialphagb1_dn10 = (locals.var_pparam_b4soialphagb1_dn10 + (locals.var_pparam_b4soialphagb1_t_dn10 * locals.var_trm1));
        locals.var_pparam_b4soialphagb1_dn11 = (locals.var_pparam_b4soialphagb1_dn11 + (locals.var_pparam_b4soialphagb1_t_dn11 * locals.var_trm1));
        locals.var_pparam_b4soialphagb1_dn12 = (locals.var_pparam_b4soialphagb1_dn12 + (locals.var_pparam_b4soialphagb1_t_dn12 * locals.var_trm1));

        let assign22710_e20606: f64 = (locals.var_pparam_b4soialphagb2_t * locals.var_trm1);
        let assign22710_e20607: f64 = (locals.var_pparam_b4soialphagb2 + assign22710_e20606);
        locals.var_pparam_b4soialphagb2 = assign22710_e20607;
        locals.var_pparam_b4soialphagb2_dn3 = (locals.var_pparam_b4soialphagb2_dn3 + (locals.var_pparam_b4soialphagb2_t_dn3 * locals.var_trm1));
        locals.var_pparam_b4soialphagb2_dn4 = (locals.var_pparam_b4soialphagb2_dn4 + ((locals.var_pparam_b4soialphagb2_t_dn4 * locals.var_trm1) + (locals.var_pparam_b4soialphagb2_t * locals.var_trm1_dn4)));
        locals.var_pparam_b4soialphagb2_dn5 = (locals.var_pparam_b4soialphagb2_dn5 + ((locals.var_pparam_b4soialphagb2_t_dn5 * locals.var_trm1) + (locals.var_pparam_b4soialphagb2_t * locals.var_trm1_dn5)));
        locals.var_pparam_b4soialphagb2_dn6 = (locals.var_pparam_b4soialphagb2_dn6 + ((locals.var_pparam_b4soialphagb2_t_dn6 * locals.var_trm1) + (locals.var_pparam_b4soialphagb2_t * locals.var_trm1_dn6)));
        locals.var_pparam_b4soialphagb2_dn7 = (locals.var_pparam_b4soialphagb2_dn7 + (locals.var_pparam_b4soialphagb2_t_dn7 * locals.var_trm1));
        locals.var_pparam_b4soialphagb2_dn8 = (locals.var_pparam_b4soialphagb2_dn8 + (locals.var_pparam_b4soialphagb2_t_dn8 * locals.var_trm1));
        locals.var_pparam_b4soialphagb2_dn9 = (locals.var_pparam_b4soialphagb2_dn9 + (locals.var_pparam_b4soialphagb2_t_dn9 * locals.var_trm1));
        locals.var_pparam_b4soialphagb2_dn10 = (locals.var_pparam_b4soialphagb2_dn10 + (locals.var_pparam_b4soialphagb2_t_dn10 * locals.var_trm1));
        locals.var_pparam_b4soialphagb2_dn11 = (locals.var_pparam_b4soialphagb2_dn11 + (locals.var_pparam_b4soialphagb2_t_dn11 * locals.var_trm1));
        locals.var_pparam_b4soialphagb2_dn12 = (locals.var_pparam_b4soialphagb2_dn12 + (locals.var_pparam_b4soialphagb2_t_dn12 * locals.var_trm1));

        let assign22720_e20611: f64 = (locals.var_pparam_b4soiaigbcp2_t * locals.var_trm1);
        let assign22720_e20612: f64 = (locals.var_pparam_b4soiaigbcp2 + assign22720_e20611);
        locals.var_pparam_b4soiaigbcp2 = assign22720_e20612;
        locals.var_pparam_b4soiaigbcp2_dn3 = (locals.var_pparam_b4soiaigbcp2_dn3 + (locals.var_pparam_b4soiaigbcp2_t_dn3 * locals.var_trm1));
        locals.var_pparam_b4soiaigbcp2_dn4 = (locals.var_pparam_b4soiaigbcp2_dn4 + ((locals.var_pparam_b4soiaigbcp2_t_dn4 * locals.var_trm1) + (locals.var_pparam_b4soiaigbcp2_t * locals.var_trm1_dn4)));
        locals.var_pparam_b4soiaigbcp2_dn5 = (locals.var_pparam_b4soiaigbcp2_dn5 + ((locals.var_pparam_b4soiaigbcp2_t_dn5 * locals.var_trm1) + (locals.var_pparam_b4soiaigbcp2_t * locals.var_trm1_dn5)));
        locals.var_pparam_b4soiaigbcp2_dn6 = (locals.var_pparam_b4soiaigbcp2_dn6 + ((locals.var_pparam_b4soiaigbcp2_t_dn6 * locals.var_trm1) + (locals.var_pparam_b4soiaigbcp2_t * locals.var_trm1_dn6)));
        locals.var_pparam_b4soiaigbcp2_dn7 = (locals.var_pparam_b4soiaigbcp2_dn7 + (locals.var_pparam_b4soiaigbcp2_t_dn7 * locals.var_trm1));
        locals.var_pparam_b4soiaigbcp2_dn8 = (locals.var_pparam_b4soiaigbcp2_dn8 + (locals.var_pparam_b4soiaigbcp2_t_dn8 * locals.var_trm1));
        locals.var_pparam_b4soiaigbcp2_dn9 = (locals.var_pparam_b4soiaigbcp2_dn9 + (locals.var_pparam_b4soiaigbcp2_t_dn9 * locals.var_trm1));
        locals.var_pparam_b4soiaigbcp2_dn10 = (locals.var_pparam_b4soiaigbcp2_dn10 + (locals.var_pparam_b4soiaigbcp2_t_dn10 * locals.var_trm1));
        locals.var_pparam_b4soiaigbcp2_dn11 = (locals.var_pparam_b4soiaigbcp2_dn11 + (locals.var_pparam_b4soiaigbcp2_t_dn11 * locals.var_trm1));
        locals.var_pparam_b4soiaigbcp2_dn12 = (locals.var_pparam_b4soiaigbcp2_dn12 + (locals.var_pparam_b4soiaigbcp2_t_dn12 * locals.var_trm1));

        let assign22730_e20619: f64 = if ((p.p374 != 0.0) || (p.p375 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1291 = assign22730_e20619;

        let (assign22740_e20625, assign22740_e20625_d_n3, assign22740_e20625_d_n4, assign22740_e20625_d_n5, assign22740_e20625_d_n6, assign22740_e20625_d_n7, assign22740_e20625_d_n8, assign22740_e20625_d_n9, assign22740_e20625_d_n10, assign22740_e20625_d_n11, assign22740_e20625_d_n12,) = {
    if (locals.var_guard1291 != 0.0) {
        let assign22740_e20623: f64 = (locals.var_vgs_eff__blk790 - locals.var_vbs_1);
        (assign22740_e20623, (locals.var_vgs_eff__blk790_dn3 - locals.var_vbs_1_dn3), (locals.var_vgs_eff__blk790_dn4 - locals.var_vbs_1_dn4), (locals.var_vgs_eff__blk790_dn5 - locals.var_vbs_1_dn5), (locals.var_vgs_eff__blk790_dn6 - locals.var_vbs_1_dn6), (locals.var_vgs_eff__blk790_dn7 - locals.var_vbs_1_dn7), (locals.var_vgs_eff__blk790_dn8 - locals.var_vbs_1_dn8), (locals.var_vgs_eff__blk790_dn9 - locals.var_vbs_1_dn9), (locals.var_vgs_eff__blk790_dn10 - locals.var_vbs_1_dn10), (locals.var_vgs_eff__blk790_dn11 - locals.var_vbs_1_dn11), (locals.var_vgs_eff__blk790_dn12 - locals.var_vbs_1_dn12),)
    } else {
        (locals.var_vgb, locals.var_vgb_dn3, locals.var_vgb_dn4, locals.var_vgb_dn5, locals.var_vgb_dn6, locals.var_vgb_dn7, locals.var_vgb_dn8, locals.var_vgb_dn9, locals.var_vgb_dn10, locals.var_vgb_dn11, locals.var_vgb_dn12,)
    }
};
        locals.var_vgb = assign22740_e20625;
        locals.var_vgb_dn3 = assign22740_e20625_d_n3;
        locals.var_vgb_dn4 = assign22740_e20625_d_n4;
        locals.var_vgb_dn5 = assign22740_e20625_d_n5;
        locals.var_vgb_dn6 = assign22740_e20625_d_n6;
        locals.var_vgb_dn7 = assign22740_e20625_d_n7;
        locals.var_vgb_dn8 = assign22740_e20625_d_n8;
        locals.var_vgb_dn9 = assign22740_e20625_d_n9;
        locals.var_vgb_dn10 = assign22740_e20625_d_n10;
        locals.var_vgb_dn11 = assign22740_e20625_d_n11;
        locals.var_vgb_dn12 = assign22740_e20625_d_n12;

        let (assign22750_e20637, assign22750_e20637_d_n3, assign22750_e20637_d_n4, assign22750_e20637_d_n5, assign22750_e20637_d_n6, assign22750_e20637_d_n7, assign22750_e20637_d_n8, assign22750_e20637_d_n9, assign22750_e20637_d_n10, assign22750_e20637_d_n11, assign22750_e20637_d_n12,) = {
    if (locals.var_guard1291 != 0.0) {
        let assign22750_e20629: f64 = (p.p37 * locals.var_here_b4soivth0);
        let assign22750_e20631: f64 = (assign22750_e20629 - locals.var_phi);
        let assign22750_e20634: f64 = (locals.var_here_b4soik1eff * locals.var_sqrtphi);
        let assign22750_e20635: f64 = (assign22750_e20631 - assign22750_e20634);
        (assign22750_e20635, (((p.p37 * locals.var_here_b4soivth0_dn3) - locals.var_phi_dn3) - ((locals.var_here_b4soik1eff_dn3 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn3))), (((p.p37 * locals.var_here_b4soivth0_dn4) - locals.var_phi_dn4) - ((locals.var_here_b4soik1eff_dn4 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn4))), (((p.p37 * locals.var_here_b4soivth0_dn5) - locals.var_phi_dn5) - ((locals.var_here_b4soik1eff_dn5 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn5))), (((p.p37 * locals.var_here_b4soivth0_dn6) - locals.var_phi_dn6) - ((locals.var_here_b4soik1eff_dn6 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn6))), (((p.p37 * locals.var_here_b4soivth0_dn7) - locals.var_phi_dn7) - ((locals.var_here_b4soik1eff_dn7 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn7))), (((p.p37 * locals.var_here_b4soivth0_dn8) - locals.var_phi_dn8) - ((locals.var_here_b4soik1eff_dn8 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn8))), (((p.p37 * locals.var_here_b4soivth0_dn9) - locals.var_phi_dn9) - ((locals.var_here_b4soik1eff_dn9 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn9))), (((p.p37 * locals.var_here_b4soivth0_dn10) - locals.var_phi_dn10) - ((locals.var_here_b4soik1eff_dn10 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn10))), (((p.p37 * locals.var_here_b4soivth0_dn11) - locals.var_phi_dn11) - ((locals.var_here_b4soik1eff_dn11 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn11))), (((p.p37 * locals.var_here_b4soivth0_dn12) - locals.var_phi_dn12) - ((locals.var_here_b4soik1eff_dn12 * locals.var_sqrtphi) + (locals.var_here_b4soik1eff * locals.var_sqrtphi_dn12))),)
    } else {
        (locals.var_vfb, locals.var_vfb_dn3, locals.var_vfb_dn4, locals.var_vfb_dn5, locals.var_vfb_dn6, locals.var_vfb_dn7, locals.var_vfb_dn8, locals.var_vfb_dn9, locals.var_vfb_dn10, locals.var_vfb_dn11, locals.var_vfb_dn12,)
    }
};
        locals.var_vfb = assign22750_e20637;
        locals.var_vfb_dn3 = assign22750_e20637_d_n3;
        locals.var_vfb_dn4 = assign22750_e20637_d_n4;
        locals.var_vfb_dn5 = assign22750_e20637_d_n5;
        locals.var_vfb_dn6 = assign22750_e20637_d_n6;
        locals.var_vfb_dn7 = assign22750_e20637_d_n7;
        locals.var_vfb_dn8 = assign22750_e20637_d_n8;
        locals.var_vfb_dn9 = assign22750_e20637_d_n9;
        locals.var_vfb_dn10 = assign22750_e20637_d_n10;
        locals.var_vfb_dn11 = assign22750_e20637_d_n11;
        locals.var_vfb_dn12 = assign22750_e20637_d_n12;

        let (assign22760_e20647, assign22760_e20647_d_n3, assign22760_e20647_d_n4, assign22760_e20647_d_n5, assign22760_e20647_d_n6, assign22760_e20647_d_n7, assign22760_e20647_d_n8, assign22760_e20647_d_n9, assign22760_e20647_d_n10, assign22760_e20647_d_n11, assign22760_e20647_d_n12,) = {
    if (locals.var_guard1291 != 0.0) {
        let assign22760_e20641: f64 = (locals.var_vfb - locals.var_vgs_eff__blk790);
        let assign22760_e20643: f64 = (assign22760_e20641 + locals.var_vbs_1);
        let assign22760_e20645: f64 = (assign22760_e20643 - 0.02);
        (assign22760_e20645, ((locals.var_vfb_dn3 - locals.var_vgs_eff__blk790_dn3) + locals.var_vbs_1_dn3), ((locals.var_vfb_dn4 - locals.var_vgs_eff__blk790_dn4) + locals.var_vbs_1_dn4), ((locals.var_vfb_dn5 - locals.var_vgs_eff__blk790_dn5) + locals.var_vbs_1_dn5), ((locals.var_vfb_dn6 - locals.var_vgs_eff__blk790_dn6) + locals.var_vbs_1_dn6), ((locals.var_vfb_dn7 - locals.var_vgs_eff__blk790_dn7) + locals.var_vbs_1_dn7), ((locals.var_vfb_dn8 - locals.var_vgs_eff__blk790_dn8) + locals.var_vbs_1_dn8), ((locals.var_vfb_dn9 - locals.var_vgs_eff__blk790_dn9) + locals.var_vbs_1_dn9), ((locals.var_vfb_dn10 - locals.var_vgs_eff__blk790_dn10) + locals.var_vbs_1_dn10), ((locals.var_vfb_dn11 - locals.var_vgs_eff__blk790_dn11) + locals.var_vbs_1_dn11), ((locals.var_vfb_dn12 - locals.var_vgs_eff__blk790_dn12) + locals.var_vbs_1_dn12),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign22760_e20647;
        locals.var_t3__blk811_dn3 = assign22760_e20647_d_n3;
        locals.var_t3__blk811_dn4 = assign22760_e20647_d_n4;
        locals.var_t3__blk811_dn5 = assign22760_e20647_d_n5;
        locals.var_t3__blk811_dn6 = assign22760_e20647_d_n6;
        locals.var_t3__blk811_dn7 = assign22760_e20647_d_n7;
        locals.var_t3__blk811_dn8 = assign22760_e20647_d_n8;
        locals.var_t3__blk811_dn9 = assign22760_e20647_d_n9;
        locals.var_t3__blk811_dn10 = assign22760_e20647_d_n10;
        locals.var_t3__blk811_dn11 = assign22760_e20647_d_n11;
        locals.var_t3__blk811_dn12 = assign22760_e20647_d_n12;

        let assign22770_e20650: f64 = if locals.var_vfb <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1292 = assign22770_e20650;

        let (assign22780_e20665, assign22780_e20665_d_n3, assign22780_e20665_d_n4, assign22780_e20665_d_n5, assign22780_e20665_d_n6, assign22780_e20665_d_n7, assign22780_e20665_d_n8, assign22780_e20665_d_n9, assign22780_e20665_d_n10, assign22780_e20665_d_n11, assign22780_e20665_d_n12,) = {
    if ((locals.var_guard1291 != 0.0) && (locals.var_guard1292 != 0.0)) {
        let assign22780_e20656: f64 = (locals.var_t3__blk811 * locals.var_t3__blk811);
        let assign22780_e20659: f64 = (4.0 * 0.02);
        let assign22780_e20661: f64 = (assign22780_e20659 * locals.var_vfb);
        let assign22780_e20662: f64 = (assign22780_e20656 - assign22780_e20661);
        let assign22780_e20663: f64 = (assign22780_e20662).sqrt();
        (assign22780_e20663, ((((locals.var_t3__blk811_dn3 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn3)) - (assign22780_e20659 * locals.var_vfb_dn3)) / (2.0 * assign22780_e20663)), ((((locals.var_t3__blk811_dn4 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn4)) - (assign22780_e20659 * locals.var_vfb_dn4)) / (2.0 * assign22780_e20663)), ((((locals.var_t3__blk811_dn5 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn5)) - (assign22780_e20659 * locals.var_vfb_dn5)) / (2.0 * assign22780_e20663)), ((((locals.var_t3__blk811_dn6 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn6)) - (assign22780_e20659 * locals.var_vfb_dn6)) / (2.0 * assign22780_e20663)), ((((locals.var_t3__blk811_dn7 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn7)) - (assign22780_e20659 * locals.var_vfb_dn7)) / (2.0 * assign22780_e20663)), ((((locals.var_t3__blk811_dn8 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn8)) - (assign22780_e20659 * locals.var_vfb_dn8)) / (2.0 * assign22780_e20663)), ((((locals.var_t3__blk811_dn9 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn9)) - (assign22780_e20659 * locals.var_vfb_dn9)) / (2.0 * assign22780_e20663)), ((((locals.var_t3__blk811_dn10 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn10)) - (assign22780_e20659 * locals.var_vfb_dn10)) / (2.0 * assign22780_e20663)), ((((locals.var_t3__blk811_dn11 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn11)) - (assign22780_e20659 * locals.var_vfb_dn11)) / (2.0 * assign22780_e20663)), ((((locals.var_t3__blk811_dn12 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn12)) - (assign22780_e20659 * locals.var_vfb_dn12)) / (2.0 * assign22780_e20663)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign22780_e20665;
        locals.var_t0__blk808_dn3 = assign22780_e20665_d_n3;
        locals.var_t0__blk808_dn4 = assign22780_e20665_d_n4;
        locals.var_t0__blk808_dn5 = assign22780_e20665_d_n5;
        locals.var_t0__blk808_dn6 = assign22780_e20665_d_n6;
        locals.var_t0__blk808_dn7 = assign22780_e20665_d_n7;
        locals.var_t0__blk808_dn8 = assign22780_e20665_d_n8;
        locals.var_t0__blk808_dn9 = assign22780_e20665_d_n9;
        locals.var_t0__blk808_dn10 = assign22780_e20665_d_n10;
        locals.var_t0__blk808_dn11 = assign22780_e20665_d_n11;
        locals.var_t0__blk808_dn12 = assign22780_e20665_d_n12;

    }

    pub(super) fn stamp_transient_block_67(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign22790_e20681, assign22790_e20681_d_n3, assign22790_e20681_d_n4, assign22790_e20681_d_n5, assign22790_e20681_d_n6, assign22790_e20681_d_n7, assign22790_e20681_d_n8, assign22790_e20681_d_n9, assign22790_e20681_d_n10, assign22790_e20681_d_n11, assign22790_e20681_d_n12,) = {
    if ((locals.var_guard1291 != 0.0) && (locals.var_guard1292 == 0.0)) {
        let assign22790_e20672: f64 = (locals.var_t3__blk811 * locals.var_t3__blk811);
        let assign22790_e20675: f64 = (4.0 * 0.02);
        let assign22790_e20677: f64 = (assign22790_e20675 * locals.var_vfb);
        let assign22790_e20678: f64 = (assign22790_e20672 + assign22790_e20677);
        let assign22790_e20679: f64 = (assign22790_e20678).sqrt();
        (assign22790_e20679, ((((locals.var_t3__blk811_dn3 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn3)) + (assign22790_e20675 * locals.var_vfb_dn3)) / (2.0 * assign22790_e20679)), ((((locals.var_t3__blk811_dn4 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn4)) + (assign22790_e20675 * locals.var_vfb_dn4)) / (2.0 * assign22790_e20679)), ((((locals.var_t3__blk811_dn5 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn5)) + (assign22790_e20675 * locals.var_vfb_dn5)) / (2.0 * assign22790_e20679)), ((((locals.var_t3__blk811_dn6 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn6)) + (assign22790_e20675 * locals.var_vfb_dn6)) / (2.0 * assign22790_e20679)), ((((locals.var_t3__blk811_dn7 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn7)) + (assign22790_e20675 * locals.var_vfb_dn7)) / (2.0 * assign22790_e20679)), ((((locals.var_t3__blk811_dn8 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn8)) + (assign22790_e20675 * locals.var_vfb_dn8)) / (2.0 * assign22790_e20679)), ((((locals.var_t3__blk811_dn9 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn9)) + (assign22790_e20675 * locals.var_vfb_dn9)) / (2.0 * assign22790_e20679)), ((((locals.var_t3__blk811_dn10 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn10)) + (assign22790_e20675 * locals.var_vfb_dn10)) / (2.0 * assign22790_e20679)), ((((locals.var_t3__blk811_dn11 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn11)) + (assign22790_e20675 * locals.var_vfb_dn11)) / (2.0 * assign22790_e20679)), ((((locals.var_t3__blk811_dn12 * locals.var_t3__blk811) + (locals.var_t3__blk811 * locals.var_t3__blk811_dn12)) + (assign22790_e20675 * locals.var_vfb_dn12)) / (2.0 * assign22790_e20679)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign22790_e20681;
        locals.var_t0__blk808_dn3 = assign22790_e20681_d_n3;
        locals.var_t0__blk808_dn4 = assign22790_e20681_d_n4;
        locals.var_t0__blk808_dn5 = assign22790_e20681_d_n5;
        locals.var_t0__blk808_dn6 = assign22790_e20681_d_n6;
        locals.var_t0__blk808_dn7 = assign22790_e20681_d_n7;
        locals.var_t0__blk808_dn8 = assign22790_e20681_d_n8;
        locals.var_t0__blk808_dn9 = assign22790_e20681_d_n9;
        locals.var_t0__blk808_dn10 = assign22790_e20681_d_n10;
        locals.var_t0__blk808_dn11 = assign22790_e20681_d_n11;
        locals.var_t0__blk808_dn12 = assign22790_e20681_d_n12;

        let (assign22800_e20691, assign22800_e20691_d_n3, assign22800_e20691_d_n4, assign22800_e20691_d_n5, assign22800_e20691_d_n6, assign22800_e20691_d_n7, assign22800_e20691_d_n8, assign22800_e20691_d_n9, assign22800_e20691_d_n10, assign22800_e20691_d_n11, assign22800_e20691_d_n12,) = {
    if (locals.var_guard1291 != 0.0) {
        let assign22800_e20687: f64 = (locals.var_t3__blk811 + locals.var_t0__blk808);
        let assign22800_e20688: f64 = (0.5 * assign22800_e20687);
        let assign22800_e20689: f64 = (locals.var_vfb - assign22800_e20688);
        (assign22800_e20689, (locals.var_vfb_dn3 - (0.5 * (locals.var_t3__blk811_dn3 + locals.var_t0__blk808_dn3))), (locals.var_vfb_dn4 - (0.5 * (locals.var_t3__blk811_dn4 + locals.var_t0__blk808_dn4))), (locals.var_vfb_dn5 - (0.5 * (locals.var_t3__blk811_dn5 + locals.var_t0__blk808_dn5))), (locals.var_vfb_dn6 - (0.5 * (locals.var_t3__blk811_dn6 + locals.var_t0__blk808_dn6))), (locals.var_vfb_dn7 - (0.5 * (locals.var_t3__blk811_dn7 + locals.var_t0__blk808_dn7))), (locals.var_vfb_dn8 - (0.5 * (locals.var_t3__blk811_dn8 + locals.var_t0__blk808_dn8))), (locals.var_vfb_dn9 - (0.5 * (locals.var_t3__blk811_dn9 + locals.var_t0__blk808_dn9))), (locals.var_vfb_dn10 - (0.5 * (locals.var_t3__blk811_dn10 + locals.var_t0__blk808_dn10))), (locals.var_vfb_dn11 - (0.5 * (locals.var_t3__blk811_dn11 + locals.var_t0__blk808_dn11))), (locals.var_vfb_dn12 - (0.5 * (locals.var_t3__blk811_dn12 + locals.var_t0__blk808_dn12))),)
    } else {
        (locals.var_vfbeff, locals.var_vfbeff_dn3, locals.var_vfbeff_dn4, locals.var_vfbeff_dn5, locals.var_vfbeff_dn6, locals.var_vfbeff_dn7, locals.var_vfbeff_dn8, locals.var_vfbeff_dn9, locals.var_vfbeff_dn10, locals.var_vfbeff_dn11, locals.var_vfbeff_dn12,)
    }
};
        locals.var_vfbeff = assign22800_e20691;
        locals.var_vfbeff_dn3 = assign22800_e20691_d_n3;
        locals.var_vfbeff_dn4 = assign22800_e20691_d_n4;
        locals.var_vfbeff_dn5 = assign22800_e20691_d_n5;
        locals.var_vfbeff_dn6 = assign22800_e20691_d_n6;
        locals.var_vfbeff_dn7 = assign22800_e20691_d_n7;
        locals.var_vfbeff_dn8 = assign22800_e20691_d_n8;
        locals.var_vfbeff_dn9 = assign22800_e20691_d_n9;
        locals.var_vfbeff_dn10 = assign22800_e20691_d_n10;
        locals.var_vfbeff_dn11 = assign22800_e20691_d_n11;
        locals.var_vfbeff_dn12 = assign22800_e20691_d_n12;

        let (assign22810_e20697, assign22810_e20697_d_n3, assign22810_e20697_d_n4, assign22810_e20697_d_n5, assign22810_e20697_d_n6, assign22810_e20697_d_n7, assign22810_e20697_d_n8, assign22810_e20697_d_n9, assign22810_e20697_d_n10, assign22810_e20697_d_n11, assign22810_e20697_d_n12,) = {
    if (locals.var_guard1291 != 0.0) {
        let assign22810_e20695: f64 = (locals.var_vfb - locals.var_vfbeff);
        (assign22810_e20695, (locals.var_vfb_dn3 - locals.var_vfbeff_dn3), (locals.var_vfb_dn4 - locals.var_vfbeff_dn4), (locals.var_vfb_dn5 - locals.var_vfbeff_dn5), (locals.var_vfb_dn6 - locals.var_vfbeff_dn6), (locals.var_vfb_dn7 - locals.var_vfbeff_dn7), (locals.var_vfb_dn8 - locals.var_vfbeff_dn8), (locals.var_vfb_dn9 - locals.var_vfbeff_dn9), (locals.var_vfb_dn10 - locals.var_vfbeff_dn10), (locals.var_vfb_dn11 - locals.var_vfbeff_dn11), (locals.var_vfb_dn12 - locals.var_vfbeff_dn12),)
    } else {
        (locals.var_voxacc, locals.var_voxacc_dn3, locals.var_voxacc_dn4, locals.var_voxacc_dn5, locals.var_voxacc_dn6, locals.var_voxacc_dn7, locals.var_voxacc_dn8, locals.var_voxacc_dn9, locals.var_voxacc_dn10, locals.var_voxacc_dn11, locals.var_voxacc_dn12,)
    }
};
        locals.var_voxacc = assign22810_e20697;
        locals.var_voxacc_dn3 = assign22810_e20697_d_n3;
        locals.var_voxacc_dn4 = assign22810_e20697_d_n4;
        locals.var_voxacc_dn5 = assign22810_e20697_d_n5;
        locals.var_voxacc_dn6 = assign22810_e20697_d_n6;
        locals.var_voxacc_dn7 = assign22810_e20697_d_n7;
        locals.var_voxacc_dn8 = assign22810_e20697_d_n8;
        locals.var_voxacc_dn9 = assign22810_e20697_d_n9;
        locals.var_voxacc_dn10 = assign22810_e20697_d_n10;
        locals.var_voxacc_dn11 = assign22810_e20697_d_n11;
        locals.var_voxacc_dn12 = assign22810_e20697_d_n12;

        let assign22820_e20700: f64 = if locals.var_voxacc < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1293 = assign22820_e20700;

        let (assign22830_e20706, assign22830_e20706_d_n3, assign22830_e20706_d_n4, assign22830_e20706_d_n5, assign22830_e20706_d_n6, assign22830_e20706_d_n7, assign22830_e20706_d_n8, assign22830_e20706_d_n9, assign22830_e20706_d_n10, assign22830_e20706_d_n11, assign22830_e20706_d_n12,) = {
    if ((locals.var_guard1291 != 0.0) && (locals.var_guard1293 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_voxacc, locals.var_voxacc_dn3, locals.var_voxacc_dn4, locals.var_voxacc_dn5, locals.var_voxacc_dn6, locals.var_voxacc_dn7, locals.var_voxacc_dn8, locals.var_voxacc_dn9, locals.var_voxacc_dn10, locals.var_voxacc_dn11, locals.var_voxacc_dn12,)
    }
};
        locals.var_voxacc = assign22830_e20706;
        locals.var_voxacc_dn3 = assign22830_e20706_d_n3;
        locals.var_voxacc_dn4 = assign22830_e20706_d_n4;
        locals.var_voxacc_dn5 = assign22830_e20706_d_n5;
        locals.var_voxacc_dn6 = assign22830_e20706_d_n6;
        locals.var_voxacc_dn7 = assign22830_e20706_d_n7;
        locals.var_voxacc_dn8 = assign22830_e20706_d_n8;
        locals.var_voxacc_dn9 = assign22830_e20706_d_n9;
        locals.var_voxacc_dn10 = assign22830_e20706_d_n10;
        locals.var_voxacc_dn11 = assign22830_e20706_d_n11;
        locals.var_voxacc_dn12 = assign22830_e20706_d_n12;

        let assign22840_e20709: f64 = if locals.var_here_b4soik1ox == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1294 = assign22840_e20709;

        let (assign22850_e20715, assign22850_e20715_d_n3, assign22850_e20715_d_n4, assign22850_e20715_d_n5, assign22850_e20715_d_n6, assign22850_e20715_d_n7, assign22850_e20715_d_n8, assign22850_e20715_d_n9, assign22850_e20715_d_n10, assign22850_e20715_d_n11, assign22850_e20715_d_n12,) = {
    if ((locals.var_guard1291 != 0.0) && (locals.var_guard1294 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_voxdepinv, locals.var_voxdepinv_dn3, locals.var_voxdepinv_dn4, locals.var_voxdepinv_dn5, locals.var_voxdepinv_dn6, locals.var_voxdepinv_dn7, locals.var_voxdepinv_dn8, locals.var_voxdepinv_dn9, locals.var_voxdepinv_dn10, locals.var_voxdepinv_dn11, locals.var_voxdepinv_dn12,)
    }
};
        locals.var_voxdepinv = assign22850_e20715;
        locals.var_voxdepinv_dn3 = assign22850_e20715_d_n3;
        locals.var_voxdepinv_dn4 = assign22850_e20715_d_n4;
        locals.var_voxdepinv_dn5 = assign22850_e20715_d_n5;
        locals.var_voxdepinv_dn6 = assign22850_e20715_d_n6;
        locals.var_voxdepinv_dn7 = assign22850_e20715_d_n7;
        locals.var_voxdepinv_dn8 = assign22850_e20715_d_n8;
        locals.var_voxdepinv_dn9 = assign22850_e20715_d_n9;
        locals.var_voxdepinv_dn10 = assign22850_e20715_d_n10;
        locals.var_voxdepinv_dn11 = assign22850_e20715_d_n11;
        locals.var_voxdepinv_dn12 = assign22850_e20715_d_n12;

        let (assign22860_e20728, assign22860_e20728_d_n3, assign22860_e20728_d_n4, assign22860_e20728_d_n5, assign22860_e20728_d_n6, assign22860_e20728_d_n7, assign22860_e20728_d_n8, assign22860_e20728_d_n9, assign22860_e20728_d_n10, assign22860_e20728_d_n11, assign22860_e20728_d_n12,) = {
    if ((locals.var_guard1291 != 0.0) && (locals.var_guard1294 == 0.0)) {
        let assign22860_e20722: f64 = (locals.var_vgs_eff__blk790 - locals.var_vgsteff__blk840);
        let assign22860_e20724: f64 = (assign22860_e20722 - locals.var_vfbeff);
        let assign22860_e20726: f64 = (assign22860_e20724 - locals.var_vbseff);
        (assign22860_e20726, (((locals.var_vgs_eff__blk790_dn3 - locals.var_vgsteff__blk840_dn3) - locals.var_vfbeff_dn3) - locals.var_vbseff_dn3), (((locals.var_vgs_eff__blk790_dn4 - locals.var_vgsteff__blk840_dn4) - locals.var_vfbeff_dn4) - locals.var_vbseff_dn4), (((locals.var_vgs_eff__blk790_dn5 - locals.var_vgsteff__blk840_dn5) - locals.var_vfbeff_dn5) - locals.var_vbseff_dn5), (((locals.var_vgs_eff__blk790_dn6 - locals.var_vgsteff__blk840_dn6) - locals.var_vfbeff_dn6) - locals.var_vbseff_dn6), (((locals.var_vgs_eff__blk790_dn7 - locals.var_vgsteff__blk840_dn7) - locals.var_vfbeff_dn7) - locals.var_vbseff_dn7), (((locals.var_vgs_eff__blk790_dn8 - locals.var_vgsteff__blk840_dn8) - locals.var_vfbeff_dn8) - locals.var_vbseff_dn8), (((locals.var_vgs_eff__blk790_dn9 - locals.var_vgsteff__blk840_dn9) - locals.var_vfbeff_dn9) - locals.var_vbseff_dn9), (((locals.var_vgs_eff__blk790_dn10 - locals.var_vgsteff__blk840_dn10) - locals.var_vfbeff_dn10) - locals.var_vbseff_dn10), (((locals.var_vgs_eff__blk790_dn11 - locals.var_vgsteff__blk840_dn11) - locals.var_vfbeff_dn11) - locals.var_vbseff_dn11), (((locals.var_vgs_eff__blk790_dn12 - locals.var_vgsteff__blk840_dn12) - locals.var_vfbeff_dn12) - locals.var_vbseff_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign22860_e20728;
        locals.var_t0__blk808_dn3 = assign22860_e20728_d_n3;
        locals.var_t0__blk808_dn4 = assign22860_e20728_d_n4;
        locals.var_t0__blk808_dn5 = assign22860_e20728_d_n5;
        locals.var_t0__blk808_dn6 = assign22860_e20728_d_n6;
        locals.var_t0__blk808_dn7 = assign22860_e20728_d_n7;
        locals.var_t0__blk808_dn8 = assign22860_e20728_d_n8;
        locals.var_t0__blk808_dn9 = assign22860_e20728_d_n9;
        locals.var_t0__blk808_dn10 = assign22860_e20728_d_n10;
        locals.var_t0__blk808_dn11 = assign22860_e20728_d_n11;
        locals.var_t0__blk808_dn12 = assign22860_e20728_d_n12;

        let assign22870_e20731: f64 = if locals.var_t0__blk808 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1295 = assign22870_e20731;

        let (assign22880_e20742, assign22880_e20742_d_n3, assign22880_e20742_d_n4, assign22880_e20742_d_n5, assign22880_e20742_d_n6, assign22880_e20742_d_n7, assign22880_e20742_d_n8, assign22880_e20742_d_n9, assign22880_e20742_d_n10, assign22880_e20742_d_n11, assign22880_e20742_d_n12,) = {
    if (((locals.var_guard1291 != 0.0) && (locals.var_guard1294 == 0.0)) && (locals.var_guard1295 != 0.0)) {
        let assign22880_e20740: f64 = (locals.var_t0__blk808 / locals.var_here_b4soik1ox);
        (assign22880_e20740, (((locals.var_t0__blk808_dn3 * locals.var_here_b4soik1ox) - (locals.var_t0__blk808 * locals.var_here_b4soik1ox_dn3)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)), (((locals.var_t0__blk808_dn4 * locals.var_here_b4soik1ox) - (locals.var_t0__blk808 * locals.var_here_b4soik1ox_dn4)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)), (((locals.var_t0__blk808_dn5 * locals.var_here_b4soik1ox) - (locals.var_t0__blk808 * locals.var_here_b4soik1ox_dn5)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)), (((locals.var_t0__blk808_dn6 * locals.var_here_b4soik1ox) - (locals.var_t0__blk808 * locals.var_here_b4soik1ox_dn6)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)), (((locals.var_t0__blk808_dn7 * locals.var_here_b4soik1ox) - (locals.var_t0__blk808 * locals.var_here_b4soik1ox_dn7)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)), (((locals.var_t0__blk808_dn8 * locals.var_here_b4soik1ox) - (locals.var_t0__blk808 * locals.var_here_b4soik1ox_dn8)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)), (((locals.var_t0__blk808_dn9 * locals.var_here_b4soik1ox) - (locals.var_t0__blk808 * locals.var_here_b4soik1ox_dn9)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)), (((locals.var_t0__blk808_dn10 * locals.var_here_b4soik1ox) - (locals.var_t0__blk808 * locals.var_here_b4soik1ox_dn10)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)), (((locals.var_t0__blk808_dn11 * locals.var_here_b4soik1ox) - (locals.var_t0__blk808 * locals.var_here_b4soik1ox_dn11)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)), (((locals.var_t0__blk808_dn12 * locals.var_here_b4soik1ox) - (locals.var_t0__blk808 * locals.var_here_b4soik1ox_dn12)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign22880_e20742;
        locals.var_t1__blk809_dn3 = assign22880_e20742_d_n3;
        locals.var_t1__blk809_dn4 = assign22880_e20742_d_n4;
        locals.var_t1__blk809_dn5 = assign22880_e20742_d_n5;
        locals.var_t1__blk809_dn6 = assign22880_e20742_d_n6;
        locals.var_t1__blk809_dn7 = assign22880_e20742_d_n7;
        locals.var_t1__blk809_dn8 = assign22880_e20742_d_n8;
        locals.var_t1__blk809_dn9 = assign22880_e20742_d_n9;
        locals.var_t1__blk809_dn10 = assign22880_e20742_d_n10;
        locals.var_t1__blk809_dn11 = assign22880_e20742_d_n11;
        locals.var_t1__blk809_dn12 = assign22880_e20742_d_n12;

        let (assign22890_e20768, assign22890_e20768_d_n3, assign22890_e20768_d_n4, assign22890_e20768_d_n5, assign22890_e20768_d_n6, assign22890_e20768_d_n7, assign22890_e20768_d_n8, assign22890_e20768_d_n9, assign22890_e20768_d_n10, assign22890_e20768_d_n11, assign22890_e20768_d_n12,) = {
    if (((locals.var_guard1291 != 0.0) && (locals.var_guard1294 == 0.0)) && (locals.var_guard1295 == 0.0)) {
        let assign22890_e20752: f64 = (locals.var_here_b4soik1ox / 2.0);
        let assign22890_e20754: f64 = (-1.0);
        let assign22890_e20758: f64 = (4.0 * locals.var_t0__blk808);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_here_b4soik1ox;
        let assign22890_e20760: f64 = (assign22890_e20758 * __rspice_inv_cse_0);
        let assign22890_e20762: f64 = (assign22890_e20760 * __rspice_inv_cse_0);
        let assign22890_e20763: f64 = (1.0 + assign22890_e20762);
        let assign22890_e20764: f64 = (assign22890_e20763).sqrt();
        let assign22890_e20765: f64 = (assign22890_e20754 + assign22890_e20764);
        let assign22890_e20766: f64 = (assign22890_e20752 * assign22890_e20765);
        (assign22890_e20766, (((locals.var_here_b4soik1ox_dn3 / 2.0) * assign22890_e20765) + (assign22890_e20752 * ((((((((4.0 * locals.var_t0__blk808_dn3) * locals.var_here_b4soik1ox) - (assign22890_e20758 * locals.var_here_b4soik1ox_dn3)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)) * locals.var_here_b4soik1ox) - (assign22890_e20760 * locals.var_here_b4soik1ox_dn3)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)) / (2.0 * assign22890_e20764)))), (((locals.var_here_b4soik1ox_dn4 / 2.0) * assign22890_e20765) + (assign22890_e20752 * ((((((((4.0 * locals.var_t0__blk808_dn4) * locals.var_here_b4soik1ox) - (assign22890_e20758 * locals.var_here_b4soik1ox_dn4)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)) * locals.var_here_b4soik1ox) - (assign22890_e20760 * locals.var_here_b4soik1ox_dn4)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)) / (2.0 * assign22890_e20764)))), (((locals.var_here_b4soik1ox_dn5 / 2.0) * assign22890_e20765) + (assign22890_e20752 * ((((((((4.0 * locals.var_t0__blk808_dn5) * locals.var_here_b4soik1ox) - (assign22890_e20758 * locals.var_here_b4soik1ox_dn5)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)) * locals.var_here_b4soik1ox) - (assign22890_e20760 * locals.var_here_b4soik1ox_dn5)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)) / (2.0 * assign22890_e20764)))), (((locals.var_here_b4soik1ox_dn6 / 2.0) * assign22890_e20765) + (assign22890_e20752 * ((((((((4.0 * locals.var_t0__blk808_dn6) * locals.var_here_b4soik1ox) - (assign22890_e20758 * locals.var_here_b4soik1ox_dn6)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)) * locals.var_here_b4soik1ox) - (assign22890_e20760 * locals.var_here_b4soik1ox_dn6)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)) / (2.0 * assign22890_e20764)))), (((locals.var_here_b4soik1ox_dn7 / 2.0) * assign22890_e20765) + (assign22890_e20752 * ((((((((4.0 * locals.var_t0__blk808_dn7) * locals.var_here_b4soik1ox) - (assign22890_e20758 * locals.var_here_b4soik1ox_dn7)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)) * locals.var_here_b4soik1ox) - (assign22890_e20760 * locals.var_here_b4soik1ox_dn7)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)) / (2.0 * assign22890_e20764)))), (((locals.var_here_b4soik1ox_dn8 / 2.0) * assign22890_e20765) + (assign22890_e20752 * ((((((((4.0 * locals.var_t0__blk808_dn8) * locals.var_here_b4soik1ox) - (assign22890_e20758 * locals.var_here_b4soik1ox_dn8)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)) * locals.var_here_b4soik1ox) - (assign22890_e20760 * locals.var_here_b4soik1ox_dn8)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)) / (2.0 * assign22890_e20764)))), (((locals.var_here_b4soik1ox_dn9 / 2.0) * assign22890_e20765) + (assign22890_e20752 * ((((((((4.0 * locals.var_t0__blk808_dn9) * locals.var_here_b4soik1ox) - (assign22890_e20758 * locals.var_here_b4soik1ox_dn9)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)) * locals.var_here_b4soik1ox) - (assign22890_e20760 * locals.var_here_b4soik1ox_dn9)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)) / (2.0 * assign22890_e20764)))), (((locals.var_here_b4soik1ox_dn10 / 2.0) * assign22890_e20765) + (assign22890_e20752 * ((((((((4.0 * locals.var_t0__blk808_dn10) * locals.var_here_b4soik1ox) - (assign22890_e20758 * locals.var_here_b4soik1ox_dn10)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)) * locals.var_here_b4soik1ox) - (assign22890_e20760 * locals.var_here_b4soik1ox_dn10)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)) / (2.0 * assign22890_e20764)))), (((locals.var_here_b4soik1ox_dn11 / 2.0) * assign22890_e20765) + (assign22890_e20752 * ((((((((4.0 * locals.var_t0__blk808_dn11) * locals.var_here_b4soik1ox) - (assign22890_e20758 * locals.var_here_b4soik1ox_dn11)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)) * locals.var_here_b4soik1ox) - (assign22890_e20760 * locals.var_here_b4soik1ox_dn11)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)) / (2.0 * assign22890_e20764)))), (((locals.var_here_b4soik1ox_dn12 / 2.0) * assign22890_e20765) + (assign22890_e20752 * ((((((((4.0 * locals.var_t0__blk808_dn12) * locals.var_here_b4soik1ox) - (assign22890_e20758 * locals.var_here_b4soik1ox_dn12)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)) * locals.var_here_b4soik1ox) - (assign22890_e20760 * locals.var_here_b4soik1ox_dn12)) / (locals.var_here_b4soik1ox * locals.var_here_b4soik1ox)) / (2.0 * assign22890_e20764)))),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign22890_e20768;
        locals.var_t1__blk809_dn3 = assign22890_e20768_d_n3;
        locals.var_t1__blk809_dn4 = assign22890_e20768_d_n4;
        locals.var_t1__blk809_dn5 = assign22890_e20768_d_n5;
        locals.var_t1__blk809_dn6 = assign22890_e20768_d_n6;
        locals.var_t1__blk809_dn7 = assign22890_e20768_d_n7;
        locals.var_t1__blk809_dn8 = assign22890_e20768_d_n8;
        locals.var_t1__blk809_dn9 = assign22890_e20768_d_n9;
        locals.var_t1__blk809_dn10 = assign22890_e20768_d_n10;
        locals.var_t1__blk809_dn11 = assign22890_e20768_d_n11;
        locals.var_t1__blk809_dn12 = assign22890_e20768_d_n12;

        let (assign22900_e20783, assign22900_e20783_d_n3, assign22900_e20783_d_n4, assign22900_e20783_d_n5, assign22900_e20783_d_n6, assign22900_e20783_d_n7, assign22900_e20783_d_n8, assign22900_e20783_d_n9, assign22900_e20783_d_n10, assign22900_e20783_d_n11, assign22900_e20783_d_n12,) = {
    if ((locals.var_guard1291 != 0.0) && (locals.var_guard1294 == 0.0)) {
        let assign22900_e20776: f64 = (locals.var_t1__blk809 * locals.var_t1__blk809);
        let assign22900_e20778: f64 = (assign22900_e20776 + locals.var_vbs_1);
        let assign22900_e20779: f64 = (locals.var_vgs_eff__blk790 - assign22900_e20778);
        let assign22900_e20781: f64 = (assign22900_e20779 - locals.var_vfb);
        (assign22900_e20781, ((locals.var_vgs_eff__blk790_dn3 - (((locals.var_t1__blk809_dn3 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn3)) + locals.var_vbs_1_dn3)) - locals.var_vfb_dn3), ((locals.var_vgs_eff__blk790_dn4 - (((locals.var_t1__blk809_dn4 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn4)) + locals.var_vbs_1_dn4)) - locals.var_vfb_dn4), ((locals.var_vgs_eff__blk790_dn5 - (((locals.var_t1__blk809_dn5 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn5)) + locals.var_vbs_1_dn5)) - locals.var_vfb_dn5), ((locals.var_vgs_eff__blk790_dn6 - (((locals.var_t1__blk809_dn6 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn6)) + locals.var_vbs_1_dn6)) - locals.var_vfb_dn6), ((locals.var_vgs_eff__blk790_dn7 - (((locals.var_t1__blk809_dn7 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn7)) + locals.var_vbs_1_dn7)) - locals.var_vfb_dn7), ((locals.var_vgs_eff__blk790_dn8 - (((locals.var_t1__blk809_dn8 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn8)) + locals.var_vbs_1_dn8)) - locals.var_vfb_dn8), ((locals.var_vgs_eff__blk790_dn9 - (((locals.var_t1__blk809_dn9 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn9)) + locals.var_vbs_1_dn9)) - locals.var_vfb_dn9), ((locals.var_vgs_eff__blk790_dn10 - (((locals.var_t1__blk809_dn10 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn10)) + locals.var_vbs_1_dn10)) - locals.var_vfb_dn10), ((locals.var_vgs_eff__blk790_dn11 - (((locals.var_t1__blk809_dn11 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn11)) + locals.var_vbs_1_dn11)) - locals.var_vfb_dn11), ((locals.var_vgs_eff__blk790_dn12 - (((locals.var_t1__blk809_dn12 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn12)) + locals.var_vbs_1_dn12)) - locals.var_vfb_dn12),)
    } else {
        (locals.var_voxdepinv, locals.var_voxdepinv_dn3, locals.var_voxdepinv_dn4, locals.var_voxdepinv_dn5, locals.var_voxdepinv_dn6, locals.var_voxdepinv_dn7, locals.var_voxdepinv_dn8, locals.var_voxdepinv_dn9, locals.var_voxdepinv_dn10, locals.var_voxdepinv_dn11, locals.var_voxdepinv_dn12,)
    }
};
        locals.var_voxdepinv = assign22900_e20783;
        locals.var_voxdepinv_dn3 = assign22900_e20783_d_n3;
        locals.var_voxdepinv_dn4 = assign22900_e20783_d_n4;
        locals.var_voxdepinv_dn5 = assign22900_e20783_d_n5;
        locals.var_voxdepinv_dn6 = assign22900_e20783_d_n6;
        locals.var_voxdepinv_dn7 = assign22900_e20783_d_n7;
        locals.var_voxdepinv_dn8 = assign22900_e20783_d_n8;
        locals.var_voxdepinv_dn9 = assign22900_e20783_d_n9;
        locals.var_voxdepinv_dn10 = assign22900_e20783_d_n10;
        locals.var_voxdepinv_dn11 = assign22900_e20783_d_n11;
        locals.var_voxdepinv_dn12 = assign22900_e20783_d_n12;

        let (assign22910_e20788, assign22910_e20788_d_n3, assign22910_e20788_d_n4, assign22910_e20788_d_n5, assign22910_e20788_d_n6, assign22910_e20788_d_n7, assign22910_e20788_d_n8, assign22910_e20788_d_n9, assign22910_e20788_d_n10, assign22910_e20788_d_n11, assign22910_e20788_d_n12,) = {
    if (locals.var_guard1291 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vfb, locals.var_vfb_dn3, locals.var_vfb_dn4, locals.var_vfb_dn5, locals.var_vfb_dn6, locals.var_vfb_dn7, locals.var_vfb_dn8, locals.var_vfb_dn9, locals.var_vfb_dn10, locals.var_vfb_dn11, locals.var_vfb_dn12,)
    }
};
        locals.var_vfb = assign22910_e20788;
        locals.var_vfb_dn3 = assign22910_e20788_d_n3;
        locals.var_vfb_dn4 = assign22910_e20788_d_n4;
        locals.var_vfb_dn5 = assign22910_e20788_d_n5;
        locals.var_vfb_dn6 = assign22910_e20788_d_n6;
        locals.var_vfb_dn7 = assign22910_e20788_d_n7;
        locals.var_vfb_dn8 = assign22910_e20788_d_n8;
        locals.var_vfb_dn9 = assign22910_e20788_d_n9;
        locals.var_vfb_dn10 = assign22910_e20788_d_n10;
        locals.var_vfb_dn11 = assign22910_e20788_d_n11;
        locals.var_vfb_dn12 = assign22910_e20788_d_n12;

        let (assign22920_e20793, assign22920_e20793_d_n3, assign22920_e20793_d_n4, assign22920_e20793_d_n5, assign22920_e20793_d_n6, assign22920_e20793_d_n7, assign22920_e20793_d_n8, assign22920_e20793_d_n9, assign22920_e20793_d_n10, assign22920_e20793_d_n11, assign22920_e20793_d_n12,) = {
    if (locals.var_guard1291 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vgb, locals.var_vgb_dn3, locals.var_vgb_dn4, locals.var_vgb_dn5, locals.var_vgb_dn6, locals.var_vgb_dn7, locals.var_vgb_dn8, locals.var_vgb_dn9, locals.var_vgb_dn10, locals.var_vgb_dn11, locals.var_vgb_dn12,)
    }
};
        locals.var_vgb = assign22920_e20793;
        locals.var_vgb_dn3 = assign22920_e20793_d_n3;
        locals.var_vgb_dn4 = assign22920_e20793_d_n4;
        locals.var_vgb_dn5 = assign22920_e20793_d_n5;
        locals.var_vgb_dn6 = assign22920_e20793_d_n6;
        locals.var_vgb_dn7 = assign22920_e20793_d_n7;
        locals.var_vgb_dn8 = assign22920_e20793_d_n8;
        locals.var_vgb_dn9 = assign22920_e20793_d_n9;
        locals.var_vgb_dn10 = assign22920_e20793_d_n10;
        locals.var_vgb_dn11 = assign22920_e20793_d_n11;
        locals.var_vgb_dn12 = assign22920_e20793_d_n12;

        let (assign22930_e20798, assign22930_e20798_d_n3, assign22930_e20798_d_n4, assign22930_e20798_d_n5, assign22930_e20798_d_n6, assign22930_e20798_d_n7, assign22930_e20798_d_n8, assign22930_e20798_d_n9, assign22930_e20798_d_n10, assign22930_e20798_d_n11, assign22930_e20798_d_n12,) = {
    if (locals.var_guard1291 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_voxacc, locals.var_voxacc_dn3, locals.var_voxacc_dn4, locals.var_voxacc_dn5, locals.var_voxacc_dn6, locals.var_voxacc_dn7, locals.var_voxacc_dn8, locals.var_voxacc_dn9, locals.var_voxacc_dn10, locals.var_voxacc_dn11, locals.var_voxacc_dn12,)
    }
};
        locals.var_voxacc = assign22930_e20798;
        locals.var_voxacc_dn3 = assign22930_e20798_d_n3;
        locals.var_voxacc_dn4 = assign22930_e20798_d_n4;
        locals.var_voxacc_dn5 = assign22930_e20798_d_n5;
        locals.var_voxacc_dn6 = assign22930_e20798_d_n6;
        locals.var_voxacc_dn7 = assign22930_e20798_d_n7;
        locals.var_voxacc_dn8 = assign22930_e20798_d_n8;
        locals.var_voxacc_dn9 = assign22930_e20798_d_n9;
        locals.var_voxacc_dn10 = assign22930_e20798_d_n10;
        locals.var_voxacc_dn11 = assign22930_e20798_d_n11;
        locals.var_voxacc_dn12 = assign22930_e20798_d_n12;

        let (assign22940_e20803, assign22940_e20803_d_n3, assign22940_e20803_d_n4, assign22940_e20803_d_n5, assign22940_e20803_d_n6, assign22940_e20803_d_n7, assign22940_e20803_d_n8, assign22940_e20803_d_n9, assign22940_e20803_d_n10, assign22940_e20803_d_n11, assign22940_e20803_d_n12,) = {
    if (locals.var_guard1291 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_voxdepinv, locals.var_voxdepinv_dn3, locals.var_voxdepinv_dn4, locals.var_voxdepinv_dn5, locals.var_voxdepinv_dn6, locals.var_voxdepinv_dn7, locals.var_voxdepinv_dn8, locals.var_voxdepinv_dn9, locals.var_voxdepinv_dn10, locals.var_voxdepinv_dn11, locals.var_voxdepinv_dn12,)
    }
};
        locals.var_voxdepinv = assign22940_e20803;
        locals.var_voxdepinv_dn3 = assign22940_e20803_d_n3;
        locals.var_voxdepinv_dn4 = assign22940_e20803_d_n4;
        locals.var_voxdepinv_dn5 = assign22940_e20803_d_n5;
        locals.var_voxdepinv_dn6 = assign22940_e20803_d_n6;
        locals.var_voxdepinv_dn7 = assign22940_e20803_d_n7;
        locals.var_voxdepinv_dn8 = assign22940_e20803_d_n8;
        locals.var_voxdepinv_dn9 = assign22940_e20803_d_n9;
        locals.var_voxdepinv_dn10 = assign22940_e20803_d_n10;
        locals.var_voxdepinv_dn11 = assign22940_e20803_d_n11;
        locals.var_voxdepinv_dn12 = assign22940_e20803_d_n12;

        let (assign22950_e20809, assign22950_e20809_d_n3, assign22950_e20809_d_n4, assign22950_e20809_d_n5, assign22950_e20809_d_n6, assign22950_e20809_d_n7, assign22950_e20809_d_n8, assign22950_e20809_d_n9, assign22950_e20809_d_n10, assign22950_e20809_d_n11, assign22950_e20809_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign22950_e20807: f64 = (locals.var_vtm * locals.var_pparam_b4soinigc);
        (assign22950_e20807, (locals.var_vtm * locals.var_pparam_b4soinigc_dn3), ((locals.var_vtm_dn4 * locals.var_pparam_b4soinigc) + (locals.var_vtm * locals.var_pparam_b4soinigc_dn4)), ((locals.var_vtm_dn5 * locals.var_pparam_b4soinigc) + (locals.var_vtm * locals.var_pparam_b4soinigc_dn5)), ((locals.var_vtm_dn6 * locals.var_pparam_b4soinigc) + (locals.var_vtm * locals.var_pparam_b4soinigc_dn6)), (locals.var_vtm * locals.var_pparam_b4soinigc_dn7), (locals.var_vtm * locals.var_pparam_b4soinigc_dn8), (locals.var_vtm * locals.var_pparam_b4soinigc_dn9), (locals.var_vtm * locals.var_pparam_b4soinigc_dn10), (locals.var_vtm * locals.var_pparam_b4soinigc_dn11), (locals.var_vtm * locals.var_pparam_b4soinigc_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign22950_e20809;
        locals.var_t0__blk808_dn3 = assign22950_e20809_d_n3;
        locals.var_t0__blk808_dn4 = assign22950_e20809_d_n4;
        locals.var_t0__blk808_dn5 = assign22950_e20809_d_n5;
        locals.var_t0__blk808_dn6 = assign22950_e20809_d_n6;
        locals.var_t0__blk808_dn7 = assign22950_e20809_d_n7;
        locals.var_t0__blk808_dn8 = assign22950_e20809_d_n8;
        locals.var_t0__blk808_dn9 = assign22950_e20809_d_n9;
        locals.var_t0__blk808_dn10 = assign22950_e20809_d_n10;
        locals.var_t0__blk808_dn11 = assign22950_e20809_d_n11;
        locals.var_t0__blk808_dn12 = assign22950_e20809_d_n12;

        let (assign22960_e20819, assign22960_e20819_d_n3, assign22960_e20819_d_n4, assign22960_e20819_d_n5, assign22960_e20819_d_n6, assign22960_e20819_d_n7, assign22960_e20819_d_n8, assign22960_e20819_d_n9, assign22960_e20819_d_n10, assign22960_e20819_d_n11, assign22960_e20819_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign22960_e20814: f64 = (p.p37 * locals.var_here_b4soivth0);
        let assign22960_e20815: f64 = (locals.var_vgs_eff__blk790 - assign22960_e20814);
        let assign22960_e20817: f64 = (assign22960_e20815 / locals.var_t0__blk808);
        (assign22960_e20817, ((((locals.var_vgs_eff__blk790_dn3 - (p.p37 * locals.var_here_b4soivth0_dn3)) * locals.var_t0__blk808) - (assign22960_e20815 * locals.var_t0__blk808_dn3)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((locals.var_vgs_eff__blk790_dn4 - (p.p37 * locals.var_here_b4soivth0_dn4)) * locals.var_t0__blk808) - (assign22960_e20815 * locals.var_t0__blk808_dn4)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((locals.var_vgs_eff__blk790_dn5 - (p.p37 * locals.var_here_b4soivth0_dn5)) * locals.var_t0__blk808) - (assign22960_e20815 * locals.var_t0__blk808_dn5)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((locals.var_vgs_eff__blk790_dn6 - (p.p37 * locals.var_here_b4soivth0_dn6)) * locals.var_t0__blk808) - (assign22960_e20815 * locals.var_t0__blk808_dn6)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((locals.var_vgs_eff__blk790_dn7 - (p.p37 * locals.var_here_b4soivth0_dn7)) * locals.var_t0__blk808) - (assign22960_e20815 * locals.var_t0__blk808_dn7)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((locals.var_vgs_eff__blk790_dn8 - (p.p37 * locals.var_here_b4soivth0_dn8)) * locals.var_t0__blk808) - (assign22960_e20815 * locals.var_t0__blk808_dn8)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((locals.var_vgs_eff__blk790_dn9 - (p.p37 * locals.var_here_b4soivth0_dn9)) * locals.var_t0__blk808) - (assign22960_e20815 * locals.var_t0__blk808_dn9)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((locals.var_vgs_eff__blk790_dn10 - (p.p37 * locals.var_here_b4soivth0_dn10)) * locals.var_t0__blk808) - (assign22960_e20815 * locals.var_t0__blk808_dn10)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((locals.var_vgs_eff__blk790_dn11 - (p.p37 * locals.var_here_b4soivth0_dn11)) * locals.var_t0__blk808) - (assign22960_e20815 * locals.var_t0__blk808_dn11)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), ((((locals.var_vgs_eff__blk790_dn12 - (p.p37 * locals.var_here_b4soivth0_dn12)) * locals.var_t0__blk808) - (assign22960_e20815 * locals.var_t0__blk808_dn12)) / (locals.var_t0__blk808 * locals.var_t0__blk808)),)
    } else {
        (locals.var_vxnvt, locals.var_vxnvt_dn3, locals.var_vxnvt_dn4, locals.var_vxnvt_dn5, locals.var_vxnvt_dn6, locals.var_vxnvt_dn7, locals.var_vxnvt_dn8, locals.var_vxnvt_dn9, locals.var_vxnvt_dn10, locals.var_vxnvt_dn11, locals.var_vxnvt_dn12,)
    }
};
        locals.var_vxnvt = assign22960_e20819;
        locals.var_vxnvt_dn3 = assign22960_e20819_d_n3;
        locals.var_vxnvt_dn4 = assign22960_e20819_d_n4;
        locals.var_vxnvt_dn5 = assign22960_e20819_d_n5;
        locals.var_vxnvt_dn6 = assign22960_e20819_d_n6;
        locals.var_vxnvt_dn7 = assign22960_e20819_d_n7;
        locals.var_vxnvt_dn8 = assign22960_e20819_d_n8;
        locals.var_vxnvt_dn9 = assign22960_e20819_d_n9;
        locals.var_vxnvt_dn10 = assign22960_e20819_d_n10;
        locals.var_vxnvt_dn11 = assign22960_e20819_d_n11;
        locals.var_vxnvt_dn12 = assign22960_e20819_d_n12;

        let assign22970_e20822: f64 = if locals.var_vxnvt > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1296 = assign22970_e20822;

        let (assign22980_e20832, assign22980_e20832_d_n3, assign22980_e20832_d_n4, assign22980_e20832_d_n5, assign22980_e20832_d_n6, assign22980_e20832_d_n7, assign22980_e20832_d_n8, assign22980_e20832_d_n9, assign22980_e20832_d_n10, assign22980_e20832_d_n11, assign22980_e20832_d_n12,) = {
    if ((p.p375 != 0.0) && (locals.var_guard1296 != 0.0)) {
        let assign22980_e20829: f64 = (p.p37 * locals.var_here_b4soivth0);
        let assign22980_e20830: f64 = (locals.var_vgs_eff__blk790 - assign22980_e20829);
        (assign22980_e20830, (locals.var_vgs_eff__blk790_dn3 - (p.p37 * locals.var_here_b4soivth0_dn3)), (locals.var_vgs_eff__blk790_dn4 - (p.p37 * locals.var_here_b4soivth0_dn4)), (locals.var_vgs_eff__blk790_dn5 - (p.p37 * locals.var_here_b4soivth0_dn5)), (locals.var_vgs_eff__blk790_dn6 - (p.p37 * locals.var_here_b4soivth0_dn6)), (locals.var_vgs_eff__blk790_dn7 - (p.p37 * locals.var_here_b4soivth0_dn7)), (locals.var_vgs_eff__blk790_dn8 - (p.p37 * locals.var_here_b4soivth0_dn8)), (locals.var_vgs_eff__blk790_dn9 - (p.p37 * locals.var_here_b4soivth0_dn9)), (locals.var_vgs_eff__blk790_dn10 - (p.p37 * locals.var_here_b4soivth0_dn10)), (locals.var_vgs_eff__blk790_dn11 - (p.p37 * locals.var_here_b4soivth0_dn11)), (locals.var_vgs_eff__blk790_dn12 - (p.p37 * locals.var_here_b4soivth0_dn12)),)
    } else {
        (locals.var_vaux, locals.var_vaux_dn3, locals.var_vaux_dn4, locals.var_vaux_dn5, locals.var_vaux_dn6, locals.var_vaux_dn7, locals.var_vaux_dn8, locals.var_vaux_dn9, locals.var_vaux_dn10, locals.var_vaux_dn11, locals.var_vaux_dn12,)
    }
};
        locals.var_vaux = assign22980_e20832;
        locals.var_vaux_dn3 = assign22980_e20832_d_n3;
        locals.var_vaux_dn4 = assign22980_e20832_d_n4;
        locals.var_vaux_dn5 = assign22980_e20832_d_n5;
        locals.var_vaux_dn6 = assign22980_e20832_d_n6;
        locals.var_vaux_dn7 = assign22980_e20832_d_n7;
        locals.var_vaux_dn8 = assign22980_e20832_d_n8;
        locals.var_vaux_dn9 = assign22980_e20832_d_n9;
        locals.var_vaux_dn10 = assign22980_e20832_d_n10;
        locals.var_vaux_dn11 = assign22980_e20832_d_n11;
        locals.var_vaux_dn12 = assign22980_e20832_d_n12;

        let assign22990_e20835: f64 = (-100.0);
        let assign22990_e20836: f64 = if locals.var_vxnvt < assign22990_e20835 { 1.0 } else { 0.0 };
        locals.var_guard1297 = assign22990_e20836;

        let (assign23000_e20850, assign23000_e20850_d_n3, assign23000_e20850_d_n4, assign23000_e20850_d_n5, assign23000_e20850_d_n6, assign23000_e20850_d_n7, assign23000_e20850_d_n8, assign23000_e20850_d_n9, assign23000_e20850_d_n10, assign23000_e20850_d_n11, assign23000_e20850_d_n12,) = {
    if (((p.p375 != 0.0) && (locals.var_guard1296 == 0.0)) && (locals.var_guard1297 != 0.0)) {
        let assign23000_e20846: f64 = (1.0 + 3.720075976e-44);
        let assign23000_e20847: f64 = (assign23000_e20846).ln();
        let assign23000_e20848: f64 = (locals.var_t0__blk808 * assign23000_e20847);
        (assign23000_e20848, (locals.var_t0__blk808_dn3 * assign23000_e20847), (locals.var_t0__blk808_dn4 * assign23000_e20847), (locals.var_t0__blk808_dn5 * assign23000_e20847), (locals.var_t0__blk808_dn6 * assign23000_e20847), (locals.var_t0__blk808_dn7 * assign23000_e20847), (locals.var_t0__blk808_dn8 * assign23000_e20847), (locals.var_t0__blk808_dn9 * assign23000_e20847), (locals.var_t0__blk808_dn10 * assign23000_e20847), (locals.var_t0__blk808_dn11 * assign23000_e20847), (locals.var_t0__blk808_dn12 * assign23000_e20847),)
    } else {
        (locals.var_vaux, locals.var_vaux_dn3, locals.var_vaux_dn4, locals.var_vaux_dn5, locals.var_vaux_dn6, locals.var_vaux_dn7, locals.var_vaux_dn8, locals.var_vaux_dn9, locals.var_vaux_dn10, locals.var_vaux_dn11, locals.var_vaux_dn12,)
    }
};
        locals.var_vaux = assign23000_e20850;
        locals.var_vaux_dn3 = assign23000_e20850_d_n3;
        locals.var_vaux_dn4 = assign23000_e20850_d_n4;
        locals.var_vaux_dn5 = assign23000_e20850_d_n5;
        locals.var_vaux_dn6 = assign23000_e20850_d_n6;
        locals.var_vaux_dn7 = assign23000_e20850_d_n7;
        locals.var_vaux_dn8 = assign23000_e20850_d_n8;
        locals.var_vaux_dn9 = assign23000_e20850_d_n9;
        locals.var_vaux_dn10 = assign23000_e20850_d_n10;
        locals.var_vaux_dn11 = assign23000_e20850_d_n11;
        locals.var_vaux_dn12 = assign23000_e20850_d_n12;

        let (assign23010_e20861, assign23010_e20861_d_n3, assign23010_e20861_d_n4, assign23010_e20861_d_n5, assign23010_e20861_d_n6, assign23010_e20861_d_n7, assign23010_e20861_d_n8, assign23010_e20861_d_n9, assign23010_e20861_d_n10, assign23010_e20861_d_n11, assign23010_e20861_d_n12,) = {
    if (((p.p375 != 0.0) && (locals.var_guard1296 == 0.0)) && (locals.var_guard1297 == 0.0)) {
        let assign23010_e20859: f64 = (locals.var_vxnvt).exp();
        (assign23010_e20859, (assign23010_e20859 * locals.var_vxnvt_dn3), (assign23010_e20859 * locals.var_vxnvt_dn4), (assign23010_e20859 * locals.var_vxnvt_dn5), (assign23010_e20859 * locals.var_vxnvt_dn6), (assign23010_e20859 * locals.var_vxnvt_dn7), (assign23010_e20859 * locals.var_vxnvt_dn8), (assign23010_e20859 * locals.var_vxnvt_dn9), (assign23010_e20859 * locals.var_vxnvt_dn10), (assign23010_e20859 * locals.var_vxnvt_dn11), (assign23010_e20859 * locals.var_vxnvt_dn12),)
    } else {
        (locals.var_expvxnvt, locals.var_expvxnvt_dn3, locals.var_expvxnvt_dn4, locals.var_expvxnvt_dn5, locals.var_expvxnvt_dn6, locals.var_expvxnvt_dn7, locals.var_expvxnvt_dn8, locals.var_expvxnvt_dn9, locals.var_expvxnvt_dn10, locals.var_expvxnvt_dn11, locals.var_expvxnvt_dn12,)
    }
};
        locals.var_expvxnvt = assign23010_e20861;
        locals.var_expvxnvt_dn3 = assign23010_e20861_d_n3;
        locals.var_expvxnvt_dn4 = assign23010_e20861_d_n4;
        locals.var_expvxnvt_dn5 = assign23010_e20861_d_n5;
        locals.var_expvxnvt_dn6 = assign23010_e20861_d_n6;
        locals.var_expvxnvt_dn7 = assign23010_e20861_d_n7;
        locals.var_expvxnvt_dn8 = assign23010_e20861_d_n8;
        locals.var_expvxnvt_dn9 = assign23010_e20861_d_n9;
        locals.var_expvxnvt_dn10 = assign23010_e20861_d_n10;
        locals.var_expvxnvt_dn11 = assign23010_e20861_d_n11;
        locals.var_expvxnvt_dn12 = assign23010_e20861_d_n12;

        let (assign23020_e20876, assign23020_e20876_d_n3, assign23020_e20876_d_n4, assign23020_e20876_d_n5, assign23020_e20876_d_n6, assign23020_e20876_d_n7, assign23020_e20876_d_n8, assign23020_e20876_d_n9, assign23020_e20876_d_n10, assign23020_e20876_d_n11, assign23020_e20876_d_n12,) = {
    if (((p.p375 != 0.0) && (locals.var_guard1296 == 0.0)) && (locals.var_guard1297 == 0.0)) {
        let assign23020_e20872: f64 = (1.0 + locals.var_expvxnvt);
        let assign23020_e20873: f64 = (assign23020_e20872).ln();
        let assign23020_e20874: f64 = (locals.var_t0__blk808 * assign23020_e20873);
        (assign23020_e20874, ((locals.var_t0__blk808_dn3 * assign23020_e20873) + (locals.var_t0__blk808 * (locals.var_expvxnvt_dn3 / assign23020_e20872))), ((locals.var_t0__blk808_dn4 * assign23020_e20873) + (locals.var_t0__blk808 * (locals.var_expvxnvt_dn4 / assign23020_e20872))), ((locals.var_t0__blk808_dn5 * assign23020_e20873) + (locals.var_t0__blk808 * (locals.var_expvxnvt_dn5 / assign23020_e20872))), ((locals.var_t0__blk808_dn6 * assign23020_e20873) + (locals.var_t0__blk808 * (locals.var_expvxnvt_dn6 / assign23020_e20872))), ((locals.var_t0__blk808_dn7 * assign23020_e20873) + (locals.var_t0__blk808 * (locals.var_expvxnvt_dn7 / assign23020_e20872))), ((locals.var_t0__blk808_dn8 * assign23020_e20873) + (locals.var_t0__blk808 * (locals.var_expvxnvt_dn8 / assign23020_e20872))), ((locals.var_t0__blk808_dn9 * assign23020_e20873) + (locals.var_t0__blk808 * (locals.var_expvxnvt_dn9 / assign23020_e20872))), ((locals.var_t0__blk808_dn10 * assign23020_e20873) + (locals.var_t0__blk808 * (locals.var_expvxnvt_dn10 / assign23020_e20872))), ((locals.var_t0__blk808_dn11 * assign23020_e20873) + (locals.var_t0__blk808 * (locals.var_expvxnvt_dn11 / assign23020_e20872))), ((locals.var_t0__blk808_dn12 * assign23020_e20873) + (locals.var_t0__blk808 * (locals.var_expvxnvt_dn12 / assign23020_e20872))),)
    } else {
        (locals.var_vaux, locals.var_vaux_dn3, locals.var_vaux_dn4, locals.var_vaux_dn5, locals.var_vaux_dn6, locals.var_vaux_dn7, locals.var_vaux_dn8, locals.var_vaux_dn9, locals.var_vaux_dn10, locals.var_vaux_dn11, locals.var_vaux_dn12,)
    }
};
        locals.var_vaux = assign23020_e20876;
        locals.var_vaux_dn3 = assign23020_e20876_d_n3;
        locals.var_vaux_dn4 = assign23020_e20876_d_n4;
        locals.var_vaux_dn5 = assign23020_e20876_d_n5;
        locals.var_vaux_dn6 = assign23020_e20876_d_n6;
        locals.var_vaux_dn7 = assign23020_e20876_d_n7;
        locals.var_vaux_dn8 = assign23020_e20876_d_n8;
        locals.var_vaux_dn9 = assign23020_e20876_d_n9;
        locals.var_vaux_dn10 = assign23020_e20876_d_n10;
        locals.var_vaux_dn11 = assign23020_e20876_d_n11;
        locals.var_vaux_dn12 = assign23020_e20876_d_n12;

        let (assign23030_e20882, assign23030_e20882_d_n3, assign23030_e20882_d_n4, assign23030_e20882_d_n5, assign23030_e20882_d_n6, assign23030_e20882_d_n7, assign23030_e20882_d_n8, assign23030_e20882_d_n9, assign23030_e20882_d_n10, assign23030_e20882_d_n11, assign23030_e20882_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23030_e20880: f64 = (locals.var_vgs_eff__blk790 * locals.var_vaux);
        (assign23030_e20880, ((locals.var_vgs_eff__blk790_dn3 * locals.var_vaux) + (locals.var_vgs_eff__blk790 * locals.var_vaux_dn3)), ((locals.var_vgs_eff__blk790_dn4 * locals.var_vaux) + (locals.var_vgs_eff__blk790 * locals.var_vaux_dn4)), ((locals.var_vgs_eff__blk790_dn5 * locals.var_vaux) + (locals.var_vgs_eff__blk790 * locals.var_vaux_dn5)), ((locals.var_vgs_eff__blk790_dn6 * locals.var_vaux) + (locals.var_vgs_eff__blk790 * locals.var_vaux_dn6)), ((locals.var_vgs_eff__blk790_dn7 * locals.var_vaux) + (locals.var_vgs_eff__blk790 * locals.var_vaux_dn7)), ((locals.var_vgs_eff__blk790_dn8 * locals.var_vaux) + (locals.var_vgs_eff__blk790 * locals.var_vaux_dn8)), ((locals.var_vgs_eff__blk790_dn9 * locals.var_vaux) + (locals.var_vgs_eff__blk790 * locals.var_vaux_dn9)), ((locals.var_vgs_eff__blk790_dn10 * locals.var_vaux) + (locals.var_vgs_eff__blk790 * locals.var_vaux_dn10)), ((locals.var_vgs_eff__blk790_dn11 * locals.var_vaux) + (locals.var_vgs_eff__blk790 * locals.var_vaux_dn11)), ((locals.var_vgs_eff__blk790_dn12 * locals.var_vaux) + (locals.var_vgs_eff__blk790 * locals.var_vaux_dn12)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign23030_e20882;
        locals.var_t2__blk810_dn3 = assign23030_e20882_d_n3;
        locals.var_t2__blk810_dn4 = assign23030_e20882_d_n4;
        locals.var_t2__blk810_dn5 = assign23030_e20882_d_n5;
        locals.var_t2__blk810_dn6 = assign23030_e20882_d_n6;
        locals.var_t2__blk810_dn7 = assign23030_e20882_d_n7;
        locals.var_t2__blk810_dn8 = assign23030_e20882_d_n8;
        locals.var_t2__blk810_dn9 = assign23030_e20882_d_n9;
        locals.var_t2__blk810_dn10 = assign23030_e20882_d_n10;
        locals.var_t2__blk810_dn11 = assign23030_e20882_d_n11;
        locals.var_t2__blk810_dn12 = assign23030_e20882_d_n12;

        let (assign23040_e20886, assign23040_e20886_d_n3, assign23040_e20886_d_n4, assign23040_e20886_d_n5, assign23040_e20886_d_n6, assign23040_e20886_d_n7, assign23040_e20886_d_n8, assign23040_e20886_d_n9, assign23040_e20886_d_n10, assign23040_e20886_d_n11, assign23040_e20886_d_n12,) = {
    if (p.p375 != 0.0) {
        (locals.var_pparam_b4soiaechvb, locals.var_pparam_b4soiaechvb_dn3, locals.var_pparam_b4soiaechvb_dn4, locals.var_pparam_b4soiaechvb_dn5, locals.var_pparam_b4soiaechvb_dn6, locals.var_pparam_b4soiaechvb_dn7, locals.var_pparam_b4soiaechvb_dn8, locals.var_pparam_b4soiaechvb_dn9, locals.var_pparam_b4soiaechvb_dn10, locals.var_pparam_b4soiaechvb_dn11, locals.var_pparam_b4soiaechvb_dn12,)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12,)
    }
};
        locals.var_t11 = assign23040_e20886;
        locals.var_t11_dn3 = assign23040_e20886_d_n3;
        locals.var_t11_dn4 = assign23040_e20886_d_n4;
        locals.var_t11_dn5 = assign23040_e20886_d_n5;
        locals.var_t11_dn6 = assign23040_e20886_d_n6;
        locals.var_t11_dn7 = assign23040_e20886_d_n7;
        locals.var_t11_dn8 = assign23040_e20886_d_n8;
        locals.var_t11_dn9 = assign23040_e20886_d_n9;
        locals.var_t11_dn10 = assign23040_e20886_d_n10;
        locals.var_t11_dn11 = assign23040_e20886_d_n11;
        locals.var_t11_dn12 = assign23040_e20886_d_n12;

        let (assign23050_e20890, assign23050_e20890_d_n3, assign23050_e20890_d_n4, assign23050_e20890_d_n5, assign23050_e20890_d_n6, assign23050_e20890_d_n7, assign23050_e20890_d_n8, assign23050_e20890_d_n9, assign23050_e20890_d_n10, assign23050_e20890_d_n11, assign23050_e20890_d_n12,) = {
    if (p.p375 != 0.0) {
        (locals.var_pparam_b4soibechvb, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12,)
    }
};
        locals.var_t12 = assign23050_e20890;
        locals.var_t12_dn3 = assign23050_e20890_d_n3;
        locals.var_t12_dn4 = assign23050_e20890_d_n4;
        locals.var_t12_dn5 = assign23050_e20890_d_n5;
        locals.var_t12_dn6 = assign23050_e20890_d_n6;
        locals.var_t12_dn7 = assign23050_e20890_d_n7;
        locals.var_t12_dn8 = assign23050_e20890_d_n8;
        locals.var_t12_dn9 = assign23050_e20890_d_n9;
        locals.var_t12_dn10 = assign23050_e20890_d_n10;
        locals.var_t12_dn11 = assign23050_e20890_d_n11;
        locals.var_t12_dn12 = assign23050_e20890_d_n12;

        let (assign23060_e20898, assign23060_e20898_d_n3, assign23060_e20898_d_n4, assign23060_e20898_d_n5, assign23060_e20898_d_n6, assign23060_e20898_d_n7, assign23060_e20898_d_n8, assign23060_e20898_d_n9, assign23060_e20898_d_n10, assign23060_e20898_d_n11, assign23060_e20898_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23060_e20894: f64 = (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc);
        let assign23060_e20896: f64 = (assign23060_e20894 - locals.var_pparam_b4soibigc);
        (assign23060_e20896, (((locals.var_pparam_b4soiaigc_dn3 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc_dn3)) - locals.var_pparam_b4soibigc_dn3), (((locals.var_pparam_b4soiaigc_dn4 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc_dn4)) - locals.var_pparam_b4soibigc_dn4), (((locals.var_pparam_b4soiaigc_dn5 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc_dn5)) - locals.var_pparam_b4soibigc_dn5), (((locals.var_pparam_b4soiaigc_dn6 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc_dn6)) - locals.var_pparam_b4soibigc_dn6), (((locals.var_pparam_b4soiaigc_dn7 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc_dn7)) - locals.var_pparam_b4soibigc_dn7), (((locals.var_pparam_b4soiaigc_dn8 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc_dn8)) - locals.var_pparam_b4soibigc_dn8), (((locals.var_pparam_b4soiaigc_dn9 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc_dn9)) - locals.var_pparam_b4soibigc_dn9), (((locals.var_pparam_b4soiaigc_dn10 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc_dn10)) - locals.var_pparam_b4soibigc_dn10), (((locals.var_pparam_b4soiaigc_dn11 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc_dn11)) - locals.var_pparam_b4soibigc_dn11), (((locals.var_pparam_b4soiaigc_dn12 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soiaigc * locals.var_pparam_b4soicigc_dn12)) - locals.var_pparam_b4soibigc_dn12),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign23060_e20898;
        locals.var_t3__blk811_dn3 = assign23060_e20898_d_n3;
        locals.var_t3__blk811_dn4 = assign23060_e20898_d_n4;
        locals.var_t3__blk811_dn5 = assign23060_e20898_d_n5;
        locals.var_t3__blk811_dn6 = assign23060_e20898_d_n6;
        locals.var_t3__blk811_dn7 = assign23060_e20898_d_n7;
        locals.var_t3__blk811_dn8 = assign23060_e20898_d_n8;
        locals.var_t3__blk811_dn9 = assign23060_e20898_d_n9;
        locals.var_t3__blk811_dn10 = assign23060_e20898_d_n10;
        locals.var_t3__blk811_dn11 = assign23060_e20898_d_n11;
        locals.var_t3__blk811_dn12 = assign23060_e20898_d_n12;

    }

    pub(super) fn stamp_transient_block_68(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23070_e20904, assign23070_e20904_d_n3, assign23070_e20904_d_n4, assign23070_e20904_d_n5, assign23070_e20904_d_n6, assign23070_e20904_d_n7, assign23070_e20904_d_n8, assign23070_e20904_d_n9, assign23070_e20904_d_n10, assign23070_e20904_d_n11, assign23070_e20904_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23070_e20902: f64 = (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc);
        (assign23070_e20902, ((locals.var_pparam_b4soibigc_dn3 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc_dn3)), ((locals.var_pparam_b4soibigc_dn4 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc_dn4)), ((locals.var_pparam_b4soibigc_dn5 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc_dn5)), ((locals.var_pparam_b4soibigc_dn6 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc_dn6)), ((locals.var_pparam_b4soibigc_dn7 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc_dn7)), ((locals.var_pparam_b4soibigc_dn8 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc_dn8)), ((locals.var_pparam_b4soibigc_dn9 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc_dn9)), ((locals.var_pparam_b4soibigc_dn10 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc_dn10)), ((locals.var_pparam_b4soibigc_dn11 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc_dn11)), ((locals.var_pparam_b4soibigc_dn12 * locals.var_pparam_b4soicigc) + (locals.var_pparam_b4soibigc * locals.var_pparam_b4soicigc_dn12)),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign23070_e20904;
        locals.var_t4__blk812_dn3 = assign23070_e20904_d_n3;
        locals.var_t4__blk812_dn4 = assign23070_e20904_d_n4;
        locals.var_t4__blk812_dn5 = assign23070_e20904_d_n5;
        locals.var_t4__blk812_dn6 = assign23070_e20904_d_n6;
        locals.var_t4__blk812_dn7 = assign23070_e20904_d_n7;
        locals.var_t4__blk812_dn8 = assign23070_e20904_d_n8;
        locals.var_t4__blk812_dn9 = assign23070_e20904_d_n9;
        locals.var_t4__blk812_dn10 = assign23070_e20904_d_n10;
        locals.var_t4__blk812_dn11 = assign23070_e20904_d_n11;
        locals.var_t4__blk812_dn12 = assign23070_e20904_d_n12;

        let (assign23080_e20920, assign23080_e20920_d_n3, assign23080_e20920_d_n4, assign23080_e20920_d_n5, assign23080_e20920_d_n6, assign23080_e20920_d_n7, assign23080_e20920_d_n8, assign23080_e20920_d_n9, assign23080_e20920_d_n10, assign23080_e20920_d_n11, assign23080_e20920_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23080_e20910: f64 = (locals.var_t3__blk811 * locals.var_voxdepinv);
        let assign23080_e20911: f64 = (locals.var_pparam_b4soiaigc + assign23080_e20910);
        let assign23080_e20914: f64 = (locals.var_t4__blk812 * locals.var_voxdepinv);
        let assign23080_e20916: f64 = (assign23080_e20914 * locals.var_voxdepinv);
        let assign23080_e20917: f64 = (assign23080_e20911 - assign23080_e20916);
        let assign23080_e20918: f64 = (locals.var_t12 * assign23080_e20917);
        (assign23080_e20918, ((locals.var_t12_dn3 * assign23080_e20917) + (locals.var_t12 * ((locals.var_pparam_b4soiaigc_dn3 + ((locals.var_t3__blk811_dn3 * locals.var_voxdepinv) + (locals.var_t3__blk811 * locals.var_voxdepinv_dn3))) - ((((locals.var_t4__blk812_dn3 * locals.var_voxdepinv) + (locals.var_t4__blk812 * locals.var_voxdepinv_dn3)) * locals.var_voxdepinv) + (assign23080_e20914 * locals.var_voxdepinv_dn3))))), ((locals.var_t12_dn4 * assign23080_e20917) + (locals.var_t12 * ((locals.var_pparam_b4soiaigc_dn4 + ((locals.var_t3__blk811_dn4 * locals.var_voxdepinv) + (locals.var_t3__blk811 * locals.var_voxdepinv_dn4))) - ((((locals.var_t4__blk812_dn4 * locals.var_voxdepinv) + (locals.var_t4__blk812 * locals.var_voxdepinv_dn4)) * locals.var_voxdepinv) + (assign23080_e20914 * locals.var_voxdepinv_dn4))))), ((locals.var_t12_dn5 * assign23080_e20917) + (locals.var_t12 * ((locals.var_pparam_b4soiaigc_dn5 + ((locals.var_t3__blk811_dn5 * locals.var_voxdepinv) + (locals.var_t3__blk811 * locals.var_voxdepinv_dn5))) - ((((locals.var_t4__blk812_dn5 * locals.var_voxdepinv) + (locals.var_t4__blk812 * locals.var_voxdepinv_dn5)) * locals.var_voxdepinv) + (assign23080_e20914 * locals.var_voxdepinv_dn5))))), ((locals.var_t12_dn6 * assign23080_e20917) + (locals.var_t12 * ((locals.var_pparam_b4soiaigc_dn6 + ((locals.var_t3__blk811_dn6 * locals.var_voxdepinv) + (locals.var_t3__blk811 * locals.var_voxdepinv_dn6))) - ((((locals.var_t4__blk812_dn6 * locals.var_voxdepinv) + (locals.var_t4__blk812 * locals.var_voxdepinv_dn6)) * locals.var_voxdepinv) + (assign23080_e20914 * locals.var_voxdepinv_dn6))))), ((locals.var_t12_dn7 * assign23080_e20917) + (locals.var_t12 * ((locals.var_pparam_b4soiaigc_dn7 + ((locals.var_t3__blk811_dn7 * locals.var_voxdepinv) + (locals.var_t3__blk811 * locals.var_voxdepinv_dn7))) - ((((locals.var_t4__blk812_dn7 * locals.var_voxdepinv) + (locals.var_t4__blk812 * locals.var_voxdepinv_dn7)) * locals.var_voxdepinv) + (assign23080_e20914 * locals.var_voxdepinv_dn7))))), ((locals.var_t12_dn8 * assign23080_e20917) + (locals.var_t12 * ((locals.var_pparam_b4soiaigc_dn8 + ((locals.var_t3__blk811_dn8 * locals.var_voxdepinv) + (locals.var_t3__blk811 * locals.var_voxdepinv_dn8))) - ((((locals.var_t4__blk812_dn8 * locals.var_voxdepinv) + (locals.var_t4__blk812 * locals.var_voxdepinv_dn8)) * locals.var_voxdepinv) + (assign23080_e20914 * locals.var_voxdepinv_dn8))))), ((locals.var_t12_dn9 * assign23080_e20917) + (locals.var_t12 * ((locals.var_pparam_b4soiaigc_dn9 + ((locals.var_t3__blk811_dn9 * locals.var_voxdepinv) + (locals.var_t3__blk811 * locals.var_voxdepinv_dn9))) - ((((locals.var_t4__blk812_dn9 * locals.var_voxdepinv) + (locals.var_t4__blk812 * locals.var_voxdepinv_dn9)) * locals.var_voxdepinv) + (assign23080_e20914 * locals.var_voxdepinv_dn9))))), ((locals.var_t12_dn10 * assign23080_e20917) + (locals.var_t12 * ((locals.var_pparam_b4soiaigc_dn10 + ((locals.var_t3__blk811_dn10 * locals.var_voxdepinv) + (locals.var_t3__blk811 * locals.var_voxdepinv_dn10))) - ((((locals.var_t4__blk812_dn10 * locals.var_voxdepinv) + (locals.var_t4__blk812 * locals.var_voxdepinv_dn10)) * locals.var_voxdepinv) + (assign23080_e20914 * locals.var_voxdepinv_dn10))))), ((locals.var_t12_dn11 * assign23080_e20917) + (locals.var_t12 * ((locals.var_pparam_b4soiaigc_dn11 + ((locals.var_t3__blk811_dn11 * locals.var_voxdepinv) + (locals.var_t3__blk811 * locals.var_voxdepinv_dn11))) - ((((locals.var_t4__blk812_dn11 * locals.var_voxdepinv) + (locals.var_t4__blk812 * locals.var_voxdepinv_dn11)) * locals.var_voxdepinv) + (assign23080_e20914 * locals.var_voxdepinv_dn11))))), ((locals.var_t12_dn12 * assign23080_e20917) + (locals.var_t12 * ((locals.var_pparam_b4soiaigc_dn12 + ((locals.var_t3__blk811_dn12 * locals.var_voxdepinv) + (locals.var_t3__blk811 * locals.var_voxdepinv_dn12))) - ((((locals.var_t4__blk812_dn12 * locals.var_voxdepinv) + (locals.var_t4__blk812 * locals.var_voxdepinv_dn12)) * locals.var_voxdepinv) + (assign23080_e20914 * locals.var_voxdepinv_dn12))))),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign23080_e20920;
        locals.var_t5__blk813_dn3 = assign23080_e20920_d_n3;
        locals.var_t5__blk813_dn4 = assign23080_e20920_d_n4;
        locals.var_t5__blk813_dn5 = assign23080_e20920_d_n5;
        locals.var_t5__blk813_dn6 = assign23080_e20920_d_n6;
        locals.var_t5__blk813_dn7 = assign23080_e20920_d_n7;
        locals.var_t5__blk813_dn8 = assign23080_e20920_d_n8;
        locals.var_t5__blk813_dn9 = assign23080_e20920_d_n9;
        locals.var_t5__blk813_dn10 = assign23080_e20920_d_n10;
        locals.var_t5__blk813_dn11 = assign23080_e20920_d_n11;
        locals.var_t5__blk813_dn12 = assign23080_e20920_d_n12;

        let assign23090_e20923: f64 = if locals.var_t5__blk813 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1298 = assign23090_e20923;

        let (assign23100_e20929, assign23100_e20929_d_n3, assign23100_e20929_d_n4, assign23100_e20929_d_n5, assign23100_e20929_d_n6, assign23100_e20929_d_n7, assign23100_e20929_d_n8, assign23100_e20929_d_n9, assign23100_e20929_d_n10, assign23100_e20929_d_n11, assign23100_e20929_d_n12,) = {
    if ((p.p375 != 0.0) && (locals.var_guard1298 != 0.0)) {
        (2.688117142e43, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign23100_e20929;
        locals.var_t6__blk814_dn3 = assign23100_e20929_d_n3;
        locals.var_t6__blk814_dn4 = assign23100_e20929_d_n4;
        locals.var_t6__blk814_dn5 = assign23100_e20929_d_n5;
        locals.var_t6__blk814_dn6 = assign23100_e20929_d_n6;
        locals.var_t6__blk814_dn7 = assign23100_e20929_d_n7;
        locals.var_t6__blk814_dn8 = assign23100_e20929_d_n8;
        locals.var_t6__blk814_dn9 = assign23100_e20929_d_n9;
        locals.var_t6__blk814_dn10 = assign23100_e20929_d_n10;
        locals.var_t6__blk814_dn11 = assign23100_e20929_d_n11;
        locals.var_t6__blk814_dn12 = assign23100_e20929_d_n12;

        let assign23110_e20932: f64 = (-100.0);
        let assign23110_e20933: f64 = if locals.var_t5__blk813 < assign23110_e20932 { 1.0 } else { 0.0 };
        locals.var_guard1299 = assign23110_e20933;

        let (assign23120_e20942, assign23120_e20942_d_n3, assign23120_e20942_d_n4, assign23120_e20942_d_n5, assign23120_e20942_d_n6, assign23120_e20942_d_n7, assign23120_e20942_d_n8, assign23120_e20942_d_n9, assign23120_e20942_d_n10, assign23120_e20942_d_n11, assign23120_e20942_d_n12,) = {
    if (((p.p375 != 0.0) && (locals.var_guard1298 == 0.0)) && (locals.var_guard1299 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign23120_e20942;
        locals.var_t6__blk814_dn3 = assign23120_e20942_d_n3;
        locals.var_t6__blk814_dn4 = assign23120_e20942_d_n4;
        locals.var_t6__blk814_dn5 = assign23120_e20942_d_n5;
        locals.var_t6__blk814_dn6 = assign23120_e20942_d_n6;
        locals.var_t6__blk814_dn7 = assign23120_e20942_d_n7;
        locals.var_t6__blk814_dn8 = assign23120_e20942_d_n8;
        locals.var_t6__blk814_dn9 = assign23120_e20942_d_n9;
        locals.var_t6__blk814_dn10 = assign23120_e20942_d_n10;
        locals.var_t6__blk814_dn11 = assign23120_e20942_d_n11;
        locals.var_t6__blk814_dn12 = assign23120_e20942_d_n12;

        let (assign23130_e20953, assign23130_e20953_d_n3, assign23130_e20953_d_n4, assign23130_e20953_d_n5, assign23130_e20953_d_n6, assign23130_e20953_d_n7, assign23130_e20953_d_n8, assign23130_e20953_d_n9, assign23130_e20953_d_n10, assign23130_e20953_d_n11, assign23130_e20953_d_n12,) = {
    if (((p.p375 != 0.0) && (locals.var_guard1298 == 0.0)) && (locals.var_guard1299 == 0.0)) {
        let assign23130_e20951: f64 = (locals.var_t5__blk813).exp();
        (assign23130_e20951, (assign23130_e20951 * locals.var_t5__blk813_dn3), (assign23130_e20951 * locals.var_t5__blk813_dn4), (assign23130_e20951 * locals.var_t5__blk813_dn5), (assign23130_e20951 * locals.var_t5__blk813_dn6), (assign23130_e20951 * locals.var_t5__blk813_dn7), (assign23130_e20951 * locals.var_t5__blk813_dn8), (assign23130_e20951 * locals.var_t5__blk813_dn9), (assign23130_e20951 * locals.var_t5__blk813_dn10), (assign23130_e20951 * locals.var_t5__blk813_dn11), (assign23130_e20951 * locals.var_t5__blk813_dn12),)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign23130_e20953;
        locals.var_t6__blk814_dn3 = assign23130_e20953_d_n3;
        locals.var_t6__blk814_dn4 = assign23130_e20953_d_n4;
        locals.var_t6__blk814_dn5 = assign23130_e20953_d_n5;
        locals.var_t6__blk814_dn6 = assign23130_e20953_d_n6;
        locals.var_t6__blk814_dn7 = assign23130_e20953_d_n7;
        locals.var_t6__blk814_dn8 = assign23130_e20953_d_n8;
        locals.var_t6__blk814_dn9 = assign23130_e20953_d_n9;
        locals.var_t6__blk814_dn10 = assign23130_e20953_d_n10;
        locals.var_t6__blk814_dn11 = assign23130_e20953_d_n11;
        locals.var_t6__blk814_dn12 = assign23130_e20953_d_n12;

        let (assign23140_e20963, assign23140_e20963_d_n3, assign23140_e20963_d_n4, assign23140_e20963_d_n5, assign23140_e20963_d_n6, assign23140_e20963_d_n7, assign23140_e20963_d_n8, assign23140_e20963_d_n9, assign23140_e20963_d_n10, assign23140_e20963_d_n11, assign23140_e20963_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23140_e20957: f64 = (locals.var_t11 * locals.var_t2__blk810);
        let assign23140_e20959: f64 = (assign23140_e20957 * locals.var_t6__blk814);
        let assign23140_e20961: f64 = (assign23140_e20959 * locals.var_igtemp);
        (assign23140_e20961, ((((((locals.var_t11_dn3 * locals.var_t2__blk810) + (locals.var_t11 * locals.var_t2__blk810_dn3)) * locals.var_t6__blk814) + (assign23140_e20957 * locals.var_t6__blk814_dn3)) * locals.var_igtemp) + (assign23140_e20959 * locals.var_igtemp_dn3)), ((((((locals.var_t11_dn4 * locals.var_t2__blk810) + (locals.var_t11 * locals.var_t2__blk810_dn4)) * locals.var_t6__blk814) + (assign23140_e20957 * locals.var_t6__blk814_dn4)) * locals.var_igtemp) + (assign23140_e20959 * locals.var_igtemp_dn4)), ((((((locals.var_t11_dn5 * locals.var_t2__blk810) + (locals.var_t11 * locals.var_t2__blk810_dn5)) * locals.var_t6__blk814) + (assign23140_e20957 * locals.var_t6__blk814_dn5)) * locals.var_igtemp) + (assign23140_e20959 * locals.var_igtemp_dn5)), ((((((locals.var_t11_dn6 * locals.var_t2__blk810) + (locals.var_t11 * locals.var_t2__blk810_dn6)) * locals.var_t6__blk814) + (assign23140_e20957 * locals.var_t6__blk814_dn6)) * locals.var_igtemp) + (assign23140_e20959 * locals.var_igtemp_dn6)), ((((((locals.var_t11_dn7 * locals.var_t2__blk810) + (locals.var_t11 * locals.var_t2__blk810_dn7)) * locals.var_t6__blk814) + (assign23140_e20957 * locals.var_t6__blk814_dn7)) * locals.var_igtemp) + (assign23140_e20959 * locals.var_igtemp_dn7)), ((((((locals.var_t11_dn8 * locals.var_t2__blk810) + (locals.var_t11 * locals.var_t2__blk810_dn8)) * locals.var_t6__blk814) + (assign23140_e20957 * locals.var_t6__blk814_dn8)) * locals.var_igtemp) + (assign23140_e20959 * locals.var_igtemp_dn8)), ((((((locals.var_t11_dn9 * locals.var_t2__blk810) + (locals.var_t11 * locals.var_t2__blk810_dn9)) * locals.var_t6__blk814) + (assign23140_e20957 * locals.var_t6__blk814_dn9)) * locals.var_igtemp) + (assign23140_e20959 * locals.var_igtemp_dn9)), ((((((locals.var_t11_dn10 * locals.var_t2__blk810) + (locals.var_t11 * locals.var_t2__blk810_dn10)) * locals.var_t6__blk814) + (assign23140_e20957 * locals.var_t6__blk814_dn10)) * locals.var_igtemp) + (assign23140_e20959 * locals.var_igtemp_dn10)), ((((((locals.var_t11_dn11 * locals.var_t2__blk810) + (locals.var_t11 * locals.var_t2__blk810_dn11)) * locals.var_t6__blk814) + (assign23140_e20957 * locals.var_t6__blk814_dn11)) * locals.var_igtemp) + (assign23140_e20959 * locals.var_igtemp_dn11)), ((((((locals.var_t11_dn12 * locals.var_t2__blk810) + (locals.var_t11 * locals.var_t2__blk810_dn12)) * locals.var_t6__blk814) + (assign23140_e20957 * locals.var_t6__blk814_dn12)) * locals.var_igtemp) + (assign23140_e20959 * locals.var_igtemp_dn12)),)
    } else {
        (locals.var_igc, locals.var_igc_dn3, locals.var_igc_dn4, locals.var_igc_dn5, locals.var_igc_dn6, locals.var_igc_dn7, locals.var_igc_dn8, locals.var_igc_dn9, locals.var_igc_dn10, locals.var_igc_dn11, locals.var_igc_dn12,)
    }
};
        locals.var_igc = assign23140_e20963;
        locals.var_igc_dn3 = assign23140_e20963_d_n3;
        locals.var_igc_dn4 = assign23140_e20963_d_n4;
        locals.var_igc_dn5 = assign23140_e20963_d_n5;
        locals.var_igc_dn6 = assign23140_e20963_d_n6;
        locals.var_igc_dn7 = assign23140_e20963_d_n7;
        locals.var_igc_dn8 = assign23140_e20963_d_n8;
        locals.var_igc_dn9 = assign23140_e20963_d_n9;
        locals.var_igc_dn10 = assign23140_e20963_d_n10;
        locals.var_igc_dn11 = assign23140_e20963_d_n11;
        locals.var_igc_dn12 = assign23140_e20963_d_n12;

        let (assign23150_e20970, assign23150_e20970_d_n3, assign23150_e20970_d_n4, assign23150_e20970_d_n5, assign23150_e20970_d_n6, assign23150_e20970_d_n7, assign23150_e20970_d_n8, assign23150_e20970_d_n9, assign23150_e20970_d_n10, assign23150_e20970_d_n11, assign23150_e20970_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23150_e20966: f64 = (-locals.var_pparam_b4soipigcd);
        let assign23150_e20968: f64 = (assign23150_e20966 * locals.var_vds_1);
        (assign23150_e20968, ((-locals.var_pparam_b4soipigcd_dn3) * locals.var_vds_1), ((-locals.var_pparam_b4soipigcd_dn4) * locals.var_vds_1), ((-locals.var_pparam_b4soipigcd_dn5) * locals.var_vds_1), ((-locals.var_pparam_b4soipigcd_dn6) * locals.var_vds_1), (((-locals.var_pparam_b4soipigcd_dn7) * locals.var_vds_1) + (assign23150_e20966 * locals.var_vds_1_dn7)), (((-locals.var_pparam_b4soipigcd_dn8) * locals.var_vds_1) + (assign23150_e20966 * locals.var_vds_1_dn8)), ((-locals.var_pparam_b4soipigcd_dn9) * locals.var_vds_1), ((-locals.var_pparam_b4soipigcd_dn10) * locals.var_vds_1), ((-locals.var_pparam_b4soipigcd_dn11) * locals.var_vds_1), ((-locals.var_pparam_b4soipigcd_dn12) * locals.var_vds_1),)
    } else {
        (locals.var_t7__blk815, locals.var_t7__blk815_dn3, locals.var_t7__blk815_dn4, locals.var_t7__blk815_dn5, locals.var_t7__blk815_dn6, locals.var_t7__blk815_dn7, locals.var_t7__blk815_dn8, locals.var_t7__blk815_dn9, locals.var_t7__blk815_dn10, locals.var_t7__blk815_dn11, locals.var_t7__blk815_dn12,)
    }
};
        locals.var_t7__blk815 = assign23150_e20970;
        locals.var_t7__blk815_dn3 = assign23150_e20970_d_n3;
        locals.var_t7__blk815_dn4 = assign23150_e20970_d_n4;
        locals.var_t7__blk815_dn5 = assign23150_e20970_d_n5;
        locals.var_t7__blk815_dn6 = assign23150_e20970_d_n6;
        locals.var_t7__blk815_dn7 = assign23150_e20970_d_n7;
        locals.var_t7__blk815_dn8 = assign23150_e20970_d_n8;
        locals.var_t7__blk815_dn9 = assign23150_e20970_d_n9;
        locals.var_t7__blk815_dn10 = assign23150_e20970_d_n10;
        locals.var_t7__blk815_dn11 = assign23150_e20970_d_n11;
        locals.var_t7__blk815_dn12 = assign23150_e20970_d_n12;

        let (assign23160_e20978, assign23160_e20978_d_n3, assign23160_e20978_d_n4, assign23160_e20978_d_n5, assign23160_e20978_d_n6, assign23160_e20978_d_n7, assign23160_e20978_d_n8, assign23160_e20978_d_n9, assign23160_e20978_d_n10, assign23160_e20978_d_n11, assign23160_e20978_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23160_e20974: f64 = (locals.var_t7__blk815 * locals.var_t7__blk815);
        let assign23160_e20976: f64 = (assign23160_e20974 + 0.0002);
        (assign23160_e20976, ((locals.var_t7__blk815_dn3 * locals.var_t7__blk815) + (locals.var_t7__blk815 * locals.var_t7__blk815_dn3)), ((locals.var_t7__blk815_dn4 * locals.var_t7__blk815) + (locals.var_t7__blk815 * locals.var_t7__blk815_dn4)), ((locals.var_t7__blk815_dn5 * locals.var_t7__blk815) + (locals.var_t7__blk815 * locals.var_t7__blk815_dn5)), ((locals.var_t7__blk815_dn6 * locals.var_t7__blk815) + (locals.var_t7__blk815 * locals.var_t7__blk815_dn6)), ((locals.var_t7__blk815_dn7 * locals.var_t7__blk815) + (locals.var_t7__blk815 * locals.var_t7__blk815_dn7)), ((locals.var_t7__blk815_dn8 * locals.var_t7__blk815) + (locals.var_t7__blk815 * locals.var_t7__blk815_dn8)), ((locals.var_t7__blk815_dn9 * locals.var_t7__blk815) + (locals.var_t7__blk815 * locals.var_t7__blk815_dn9)), ((locals.var_t7__blk815_dn10 * locals.var_t7__blk815) + (locals.var_t7__blk815 * locals.var_t7__blk815_dn10)), ((locals.var_t7__blk815_dn11 * locals.var_t7__blk815) + (locals.var_t7__blk815 * locals.var_t7__blk815_dn11)), ((locals.var_t7__blk815_dn12 * locals.var_t7__blk815) + (locals.var_t7__blk815 * locals.var_t7__blk815_dn12)),)
    } else {
        (locals.var_t8, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12,)
    }
};
        locals.var_t8 = assign23160_e20978;
        locals.var_t8_dn3 = assign23160_e20978_d_n3;
        locals.var_t8_dn4 = assign23160_e20978_d_n4;
        locals.var_t8_dn5 = assign23160_e20978_d_n5;
        locals.var_t8_dn6 = assign23160_e20978_d_n6;
        locals.var_t8_dn7 = assign23160_e20978_d_n7;
        locals.var_t8_dn8 = assign23160_e20978_d_n8;
        locals.var_t8_dn9 = assign23160_e20978_d_n9;
        locals.var_t8_dn10 = assign23160_e20978_d_n10;
        locals.var_t8_dn11 = assign23160_e20978_d_n11;
        locals.var_t8_dn12 = assign23160_e20978_d_n12;

        let assign23170_e20981: f64 = if locals.var_t7__blk815 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1300 = assign23170_e20981;

        let (assign23180_e20987, assign23180_e20987_d_n3, assign23180_e20987_d_n4, assign23180_e20987_d_n5, assign23180_e20987_d_n6, assign23180_e20987_d_n7, assign23180_e20987_d_n8, assign23180_e20987_d_n9, assign23180_e20987_d_n10, assign23180_e20987_d_n11, assign23180_e20987_d_n12,) = {
    if ((p.p375 != 0.0) && (locals.var_guard1300 != 0.0)) {
        (2.688117142e43, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign23180_e20987;
        locals.var_t9_dn3 = assign23180_e20987_d_n3;
        locals.var_t9_dn4 = assign23180_e20987_d_n4;
        locals.var_t9_dn5 = assign23180_e20987_d_n5;
        locals.var_t9_dn6 = assign23180_e20987_d_n6;
        locals.var_t9_dn7 = assign23180_e20987_d_n7;
        locals.var_t9_dn8 = assign23180_e20987_d_n8;
        locals.var_t9_dn9 = assign23180_e20987_d_n9;
        locals.var_t9_dn10 = assign23180_e20987_d_n10;
        locals.var_t9_dn11 = assign23180_e20987_d_n11;
        locals.var_t9_dn12 = assign23180_e20987_d_n12;

        let assign23190_e20990: f64 = (-100.0);
        let assign23190_e20991: f64 = if locals.var_t7__blk815 < assign23190_e20990 { 1.0 } else { 0.0 };
        locals.var_guard1301 = assign23190_e20991;

        let (assign23200_e21000, assign23200_e21000_d_n3, assign23200_e21000_d_n4, assign23200_e21000_d_n5, assign23200_e21000_d_n6, assign23200_e21000_d_n7, assign23200_e21000_d_n8, assign23200_e21000_d_n9, assign23200_e21000_d_n10, assign23200_e21000_d_n11, assign23200_e21000_d_n12,) = {
    if (((p.p375 != 0.0) && (locals.var_guard1300 == 0.0)) && (locals.var_guard1301 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign23200_e21000;
        locals.var_t9_dn3 = assign23200_e21000_d_n3;
        locals.var_t9_dn4 = assign23200_e21000_d_n4;
        locals.var_t9_dn5 = assign23200_e21000_d_n5;
        locals.var_t9_dn6 = assign23200_e21000_d_n6;
        locals.var_t9_dn7 = assign23200_e21000_d_n7;
        locals.var_t9_dn8 = assign23200_e21000_d_n8;
        locals.var_t9_dn9 = assign23200_e21000_d_n9;
        locals.var_t9_dn10 = assign23200_e21000_d_n10;
        locals.var_t9_dn11 = assign23200_e21000_d_n11;
        locals.var_t9_dn12 = assign23200_e21000_d_n12;

        let (assign23210_e21011, assign23210_e21011_d_n3, assign23210_e21011_d_n4, assign23210_e21011_d_n5, assign23210_e21011_d_n6, assign23210_e21011_d_n7, assign23210_e21011_d_n8, assign23210_e21011_d_n9, assign23210_e21011_d_n10, assign23210_e21011_d_n11, assign23210_e21011_d_n12,) = {
    if (((p.p375 != 0.0) && (locals.var_guard1300 == 0.0)) && (locals.var_guard1301 == 0.0)) {
        let assign23210_e21009: f64 = (locals.var_t7__blk815).exp();
        (assign23210_e21009, (assign23210_e21009 * locals.var_t7__blk815_dn3), (assign23210_e21009 * locals.var_t7__blk815_dn4), (assign23210_e21009 * locals.var_t7__blk815_dn5), (assign23210_e21009 * locals.var_t7__blk815_dn6), (assign23210_e21009 * locals.var_t7__blk815_dn7), (assign23210_e21009 * locals.var_t7__blk815_dn8), (assign23210_e21009 * locals.var_t7__blk815_dn9), (assign23210_e21009 * locals.var_t7__blk815_dn10), (assign23210_e21009 * locals.var_t7__blk815_dn11), (assign23210_e21009 * locals.var_t7__blk815_dn12),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign23210_e21011;
        locals.var_t9_dn3 = assign23210_e21011_d_n3;
        locals.var_t9_dn4 = assign23210_e21011_d_n4;
        locals.var_t9_dn5 = assign23210_e21011_d_n5;
        locals.var_t9_dn6 = assign23210_e21011_d_n6;
        locals.var_t9_dn7 = assign23210_e21011_d_n7;
        locals.var_t9_dn8 = assign23210_e21011_d_n8;
        locals.var_t9_dn9 = assign23210_e21011_d_n9;
        locals.var_t9_dn10 = assign23210_e21011_d_n10;
        locals.var_t9_dn11 = assign23210_e21011_d_n11;
        locals.var_t9_dn12 = assign23210_e21011_d_n12;

        let (assign23220_e21019, assign23220_e21019_d_n3, assign23220_e21019_d_n4, assign23220_e21019_d_n5, assign23220_e21019_d_n6, assign23220_e21019_d_n7, assign23220_e21019_d_n8, assign23220_e21019_d_n9, assign23220_e21019_d_n10, assign23220_e21019_d_n11, assign23220_e21019_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23220_e21015: f64 = (locals.var_t9 - 1.0);
        let assign23220_e21017: f64 = (assign23220_e21015 + 0.0001);
        (assign23220_e21017, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign23220_e21019;
        locals.var_t1__blk809_dn3 = assign23220_e21019_d_n3;
        locals.var_t1__blk809_dn4 = assign23220_e21019_d_n4;
        locals.var_t1__blk809_dn5 = assign23220_e21019_d_n5;
        locals.var_t1__blk809_dn6 = assign23220_e21019_d_n6;
        locals.var_t1__blk809_dn7 = assign23220_e21019_d_n7;
        locals.var_t1__blk809_dn8 = assign23220_e21019_d_n8;
        locals.var_t1__blk809_dn9 = assign23220_e21019_d_n9;
        locals.var_t1__blk809_dn10 = assign23220_e21019_d_n10;
        locals.var_t1__blk809_dn11 = assign23220_e21019_d_n11;
        locals.var_t1__blk809_dn12 = assign23220_e21019_d_n12;

        let (assign23230_e21027, assign23230_e21027_d_n3, assign23230_e21027_d_n4, assign23230_e21027_d_n5, assign23230_e21027_d_n6, assign23230_e21027_d_n7, assign23230_e21027_d_n8, assign23230_e21027_d_n9, assign23230_e21027_d_n10, assign23230_e21027_d_n11, assign23230_e21027_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23230_e21023: f64 = (locals.var_t1__blk809 - locals.var_t7__blk815);
        let assign23230_e21025: f64 = (assign23230_e21023 / locals.var_t8);
        (assign23230_e21025, ((((locals.var_t1__blk809_dn3 - locals.var_t7__blk815_dn3) * locals.var_t8) - (assign23230_e21023 * locals.var_t8_dn3)) / (locals.var_t8 * locals.var_t8)), ((((locals.var_t1__blk809_dn4 - locals.var_t7__blk815_dn4) * locals.var_t8) - (assign23230_e21023 * locals.var_t8_dn4)) / (locals.var_t8 * locals.var_t8)), ((((locals.var_t1__blk809_dn5 - locals.var_t7__blk815_dn5) * locals.var_t8) - (assign23230_e21023 * locals.var_t8_dn5)) / (locals.var_t8 * locals.var_t8)), ((((locals.var_t1__blk809_dn6 - locals.var_t7__blk815_dn6) * locals.var_t8) - (assign23230_e21023 * locals.var_t8_dn6)) / (locals.var_t8 * locals.var_t8)), ((((locals.var_t1__blk809_dn7 - locals.var_t7__blk815_dn7) * locals.var_t8) - (assign23230_e21023 * locals.var_t8_dn7)) / (locals.var_t8 * locals.var_t8)), ((((locals.var_t1__blk809_dn8 - locals.var_t7__blk815_dn8) * locals.var_t8) - (assign23230_e21023 * locals.var_t8_dn8)) / (locals.var_t8 * locals.var_t8)), ((((locals.var_t1__blk809_dn9 - locals.var_t7__blk815_dn9) * locals.var_t8) - (assign23230_e21023 * locals.var_t8_dn9)) / (locals.var_t8 * locals.var_t8)), ((((locals.var_t1__blk809_dn10 - locals.var_t7__blk815_dn10) * locals.var_t8) - (assign23230_e21023 * locals.var_t8_dn10)) / (locals.var_t8 * locals.var_t8)), ((((locals.var_t1__blk809_dn11 - locals.var_t7__blk815_dn11) * locals.var_t8) - (assign23230_e21023 * locals.var_t8_dn11)) / (locals.var_t8 * locals.var_t8)), ((((locals.var_t1__blk809_dn12 - locals.var_t7__blk815_dn12) * locals.var_t8) - (assign23230_e21023 * locals.var_t8_dn12)) / (locals.var_t8 * locals.var_t8)),)
    } else {
        (locals.var_t10__blk818, locals.var_t10__blk818_dn3, locals.var_t10__blk818_dn4, locals.var_t10__blk818_dn5, locals.var_t10__blk818_dn6, locals.var_t10__blk818_dn7, locals.var_t10__blk818_dn8, locals.var_t10__blk818_dn9, locals.var_t10__blk818_dn10, locals.var_t10__blk818_dn11, locals.var_t10__blk818_dn12,)
    }
};
        locals.var_t10__blk818 = assign23230_e21027;
        locals.var_t10__blk818_dn3 = assign23230_e21027_d_n3;
        locals.var_t10__blk818_dn4 = assign23230_e21027_d_n4;
        locals.var_t10__blk818_dn5 = assign23230_e21027_d_n5;
        locals.var_t10__blk818_dn6 = assign23230_e21027_d_n6;
        locals.var_t10__blk818_dn7 = assign23230_e21027_d_n7;
        locals.var_t10__blk818_dn8 = assign23230_e21027_d_n8;
        locals.var_t10__blk818_dn9 = assign23230_e21027_d_n9;
        locals.var_t10__blk818_dn10 = assign23230_e21027_d_n10;
        locals.var_t10__blk818_dn11 = assign23230_e21027_d_n11;
        locals.var_t10__blk818_dn12 = assign23230_e21027_d_n12;

        let (assign23240_e21033, assign23240_e21033_d_n3, assign23240_e21033_d_n4, assign23240_e21033_d_n5, assign23240_e21033_d_n6, assign23240_e21033_d_n7, assign23240_e21033_d_n8, assign23240_e21033_d_n9, assign23240_e21033_d_n10, assign23240_e21033_d_n11, assign23240_e21033_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23240_e21031: f64 = (locals.var_igc * locals.var_t10__blk818);
        (assign23240_e21031, ((locals.var_igc_dn3 * locals.var_t10__blk818) + (locals.var_igc * locals.var_t10__blk818_dn3)), ((locals.var_igc_dn4 * locals.var_t10__blk818) + (locals.var_igc * locals.var_t10__blk818_dn4)), ((locals.var_igc_dn5 * locals.var_t10__blk818) + (locals.var_igc * locals.var_t10__blk818_dn5)), ((locals.var_igc_dn6 * locals.var_t10__blk818) + (locals.var_igc * locals.var_t10__blk818_dn6)), ((locals.var_igc_dn7 * locals.var_t10__blk818) + (locals.var_igc * locals.var_t10__blk818_dn7)), ((locals.var_igc_dn8 * locals.var_t10__blk818) + (locals.var_igc * locals.var_t10__blk818_dn8)), ((locals.var_igc_dn9 * locals.var_t10__blk818) + (locals.var_igc * locals.var_t10__blk818_dn9)), ((locals.var_igc_dn10 * locals.var_t10__blk818) + (locals.var_igc * locals.var_t10__blk818_dn10)), ((locals.var_igc_dn11 * locals.var_t10__blk818) + (locals.var_igc * locals.var_t10__blk818_dn11)), ((locals.var_igc_dn12 * locals.var_t10__blk818) + (locals.var_igc * locals.var_t10__blk818_dn12)),)
    } else {
        (locals.var_igcs_1, locals.var_igcs_1_dn3, locals.var_igcs_1_dn4, locals.var_igcs_1_dn5, locals.var_igcs_1_dn6, locals.var_igcs_1_dn7, locals.var_igcs_1_dn8, locals.var_igcs_1_dn9, locals.var_igcs_1_dn10, locals.var_igcs_1_dn11, locals.var_igcs_1_dn12,)
    }
};
        locals.var_igcs_1 = assign23240_e21033;
        locals.var_igcs_1_dn3 = assign23240_e21033_d_n3;
        locals.var_igcs_1_dn4 = assign23240_e21033_d_n4;
        locals.var_igcs_1_dn5 = assign23240_e21033_d_n5;
        locals.var_igcs_1_dn6 = assign23240_e21033_d_n6;
        locals.var_igcs_1_dn7 = assign23240_e21033_d_n7;
        locals.var_igcs_1_dn8 = assign23240_e21033_d_n8;
        locals.var_igcs_1_dn9 = assign23240_e21033_d_n9;
        locals.var_igcs_1_dn10 = assign23240_e21033_d_n10;
        locals.var_igcs_1_dn11 = assign23240_e21033_d_n11;
        locals.var_igcs_1_dn12 = assign23240_e21033_d_n12;

        let (assign23250_e21041, assign23250_e21041_d_n3, assign23250_e21041_d_n4, assign23250_e21041_d_n5, assign23250_e21041_d_n6, assign23250_e21041_d_n7, assign23250_e21041_d_n8, assign23250_e21041_d_n9, assign23250_e21041_d_n10, assign23250_e21041_d_n11, assign23250_e21041_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23250_e21037: f64 = (locals.var_t9 - 1.0);
        let assign23250_e21039: f64 = (assign23250_e21037 - 0.0001);
        (assign23250_e21039, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign23250_e21041;
        locals.var_t1__blk809_dn3 = assign23250_e21041_d_n3;
        locals.var_t1__blk809_dn4 = assign23250_e21041_d_n4;
        locals.var_t1__blk809_dn5 = assign23250_e21041_d_n5;
        locals.var_t1__blk809_dn6 = assign23250_e21041_d_n6;
        locals.var_t1__blk809_dn7 = assign23250_e21041_d_n7;
        locals.var_t1__blk809_dn8 = assign23250_e21041_d_n8;
        locals.var_t1__blk809_dn9 = assign23250_e21041_d_n9;
        locals.var_t1__blk809_dn10 = assign23250_e21041_d_n10;
        locals.var_t1__blk809_dn11 = assign23250_e21041_d_n11;
        locals.var_t1__blk809_dn12 = assign23250_e21041_d_n12;

        let (assign23260_e21051, assign23260_e21051_d_n3, assign23260_e21051_d_n4, assign23260_e21051_d_n5, assign23260_e21051_d_n6, assign23260_e21051_d_n7, assign23260_e21051_d_n8, assign23260_e21051_d_n9, assign23260_e21051_d_n10, assign23260_e21051_d_n11, assign23260_e21051_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23260_e21045: f64 = (locals.var_t7__blk815 * locals.var_t9);
        let assign23260_e21047: f64 = (assign23260_e21045 - locals.var_t1__blk809);
        let assign23260_e21049: f64 = (assign23260_e21047 / locals.var_t8);
        (assign23260_e21049, ((((((locals.var_t7__blk815_dn3 * locals.var_t9) + (locals.var_t7__blk815 * locals.var_t9_dn3)) - locals.var_t1__blk809_dn3) * locals.var_t8) - (assign23260_e21047 * locals.var_t8_dn3)) / (locals.var_t8 * locals.var_t8)), ((((((locals.var_t7__blk815_dn4 * locals.var_t9) + (locals.var_t7__blk815 * locals.var_t9_dn4)) - locals.var_t1__blk809_dn4) * locals.var_t8) - (assign23260_e21047 * locals.var_t8_dn4)) / (locals.var_t8 * locals.var_t8)), ((((((locals.var_t7__blk815_dn5 * locals.var_t9) + (locals.var_t7__blk815 * locals.var_t9_dn5)) - locals.var_t1__blk809_dn5) * locals.var_t8) - (assign23260_e21047 * locals.var_t8_dn5)) / (locals.var_t8 * locals.var_t8)), ((((((locals.var_t7__blk815_dn6 * locals.var_t9) + (locals.var_t7__blk815 * locals.var_t9_dn6)) - locals.var_t1__blk809_dn6) * locals.var_t8) - (assign23260_e21047 * locals.var_t8_dn6)) / (locals.var_t8 * locals.var_t8)), ((((((locals.var_t7__blk815_dn7 * locals.var_t9) + (locals.var_t7__blk815 * locals.var_t9_dn7)) - locals.var_t1__blk809_dn7) * locals.var_t8) - (assign23260_e21047 * locals.var_t8_dn7)) / (locals.var_t8 * locals.var_t8)), ((((((locals.var_t7__blk815_dn8 * locals.var_t9) + (locals.var_t7__blk815 * locals.var_t9_dn8)) - locals.var_t1__blk809_dn8) * locals.var_t8) - (assign23260_e21047 * locals.var_t8_dn8)) / (locals.var_t8 * locals.var_t8)), ((((((locals.var_t7__blk815_dn9 * locals.var_t9) + (locals.var_t7__blk815 * locals.var_t9_dn9)) - locals.var_t1__blk809_dn9) * locals.var_t8) - (assign23260_e21047 * locals.var_t8_dn9)) / (locals.var_t8 * locals.var_t8)), ((((((locals.var_t7__blk815_dn10 * locals.var_t9) + (locals.var_t7__blk815 * locals.var_t9_dn10)) - locals.var_t1__blk809_dn10) * locals.var_t8) - (assign23260_e21047 * locals.var_t8_dn10)) / (locals.var_t8 * locals.var_t8)), ((((((locals.var_t7__blk815_dn11 * locals.var_t9) + (locals.var_t7__blk815 * locals.var_t9_dn11)) - locals.var_t1__blk809_dn11) * locals.var_t8) - (assign23260_e21047 * locals.var_t8_dn11)) / (locals.var_t8 * locals.var_t8)), ((((((locals.var_t7__blk815_dn12 * locals.var_t9) + (locals.var_t7__blk815 * locals.var_t9_dn12)) - locals.var_t1__blk809_dn12) * locals.var_t8) - (assign23260_e21047 * locals.var_t8_dn12)) / (locals.var_t8 * locals.var_t8)),)
    } else {
        (locals.var_t10__blk818, locals.var_t10__blk818_dn3, locals.var_t10__blk818_dn4, locals.var_t10__blk818_dn5, locals.var_t10__blk818_dn6, locals.var_t10__blk818_dn7, locals.var_t10__blk818_dn8, locals.var_t10__blk818_dn9, locals.var_t10__blk818_dn10, locals.var_t10__blk818_dn11, locals.var_t10__blk818_dn12,)
    }
};
        locals.var_t10__blk818 = assign23260_e21051;
        locals.var_t10__blk818_dn3 = assign23260_e21051_d_n3;
        locals.var_t10__blk818_dn4 = assign23260_e21051_d_n4;
        locals.var_t10__blk818_dn5 = assign23260_e21051_d_n5;
        locals.var_t10__blk818_dn6 = assign23260_e21051_d_n6;
        locals.var_t10__blk818_dn7 = assign23260_e21051_d_n7;
        locals.var_t10__blk818_dn8 = assign23260_e21051_d_n8;
        locals.var_t10__blk818_dn9 = assign23260_e21051_d_n9;
        locals.var_t10__blk818_dn10 = assign23260_e21051_d_n10;
        locals.var_t10__blk818_dn11 = assign23260_e21051_d_n11;
        locals.var_t10__blk818_dn12 = assign23260_e21051_d_n12;

        let (assign23270_e21057, assign23270_e21057_d_n3, assign23270_e21057_d_n4, assign23270_e21057_d_n5, assign23270_e21057_d_n6, assign23270_e21057_d_n7, assign23270_e21057_d_n8, assign23270_e21057_d_n9, assign23270_e21057_d_n10, assign23270_e21057_d_n11, assign23270_e21057_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23270_e21055: f64 = (locals.var_igc * locals.var_t10__blk818);
        (assign23270_e21055, ((locals.var_igc_dn3 * locals.var_t10__blk818) + (locals.var_igc * locals.var_t10__blk818_dn3)), ((locals.var_igc_dn4 * locals.var_t10__blk818) + (locals.var_igc * locals.var_t10__blk818_dn4)), ((locals.var_igc_dn5 * locals.var_t10__blk818) + (locals.var_igc * locals.var_t10__blk818_dn5)), ((locals.var_igc_dn6 * locals.var_t10__blk818) + (locals.var_igc * locals.var_t10__blk818_dn6)), ((locals.var_igc_dn7 * locals.var_t10__blk818) + (locals.var_igc * locals.var_t10__blk818_dn7)), ((locals.var_igc_dn8 * locals.var_t10__blk818) + (locals.var_igc * locals.var_t10__blk818_dn8)), ((locals.var_igc_dn9 * locals.var_t10__blk818) + (locals.var_igc * locals.var_t10__blk818_dn9)), ((locals.var_igc_dn10 * locals.var_t10__blk818) + (locals.var_igc * locals.var_t10__blk818_dn10)), ((locals.var_igc_dn11 * locals.var_t10__blk818) + (locals.var_igc * locals.var_t10__blk818_dn11)), ((locals.var_igc_dn12 * locals.var_t10__blk818) + (locals.var_igc * locals.var_t10__blk818_dn12)),)
    } else {
        (locals.var_igcd_1, locals.var_igcd_1_dn3, locals.var_igcd_1_dn4, locals.var_igcd_1_dn5, locals.var_igcd_1_dn6, locals.var_igcd_1_dn7, locals.var_igcd_1_dn8, locals.var_igcd_1_dn9, locals.var_igcd_1_dn10, locals.var_igcd_1_dn11, locals.var_igcd_1_dn12,)
    }
};
        locals.var_igcd_1 = assign23270_e21057;
        locals.var_igcd_1_dn3 = assign23270_e21057_d_n3;
        locals.var_igcd_1_dn4 = assign23270_e21057_d_n4;
        locals.var_igcd_1_dn5 = assign23270_e21057_d_n5;
        locals.var_igcd_1_dn6 = assign23270_e21057_d_n6;
        locals.var_igcd_1_dn7 = assign23270_e21057_d_n7;
        locals.var_igcd_1_dn8 = assign23270_e21057_d_n8;
        locals.var_igcd_1_dn9 = assign23270_e21057_d_n9;
        locals.var_igcd_1_dn10 = assign23270_e21057_d_n10;
        locals.var_igcd_1_dn11 = assign23270_e21057_d_n11;
        locals.var_igcd_1_dn12 = assign23270_e21057_d_n12;

        let (assign23280_e21063, assign23280_e21063_d_n3, assign23280_e21063_d_n4, assign23280_e21063_d_n5, assign23280_e21063_d_n6, assign23280_e21063_d_n7, assign23280_e21063_d_n8, assign23280_e21063_d_n9, assign23280_e21063_d_n10, assign23280_e21063_d_n11, assign23280_e21063_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23280_e21061: f64 = (locals.var_vgs - locals.var_pparam_b4soivfbsd);
        (assign23280_e21061, (-locals.var_pparam_b4soivfbsd_dn3), (-locals.var_pparam_b4soivfbsd_dn4), (-locals.var_pparam_b4soivfbsd_dn5), (-locals.var_pparam_b4soivfbsd_dn6), (-locals.var_pparam_b4soivfbsd_dn7), (locals.var_vgs_dn8 - locals.var_pparam_b4soivfbsd_dn8), (locals.var_vgs_dn9 - locals.var_pparam_b4soivfbsd_dn9), (-locals.var_pparam_b4soivfbsd_dn10), (-locals.var_pparam_b4soivfbsd_dn11), (-locals.var_pparam_b4soivfbsd_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign23280_e21063;
        locals.var_t0__blk808_dn3 = assign23280_e21063_d_n3;
        locals.var_t0__blk808_dn4 = assign23280_e21063_d_n4;
        locals.var_t0__blk808_dn5 = assign23280_e21063_d_n5;
        locals.var_t0__blk808_dn6 = assign23280_e21063_d_n6;
        locals.var_t0__blk808_dn7 = assign23280_e21063_d_n7;
        locals.var_t0__blk808_dn8 = assign23280_e21063_d_n8;
        locals.var_t0__blk808_dn9 = assign23280_e21063_d_n9;
        locals.var_t0__blk808_dn10 = assign23280_e21063_d_n10;
        locals.var_t0__blk808_dn11 = assign23280_e21063_d_n11;
        locals.var_t0__blk808_dn12 = assign23280_e21063_d_n12;

        let (assign23290_e21072, assign23290_e21072_d_n3, assign23290_e21072_d_n4, assign23290_e21072_d_n5, assign23290_e21072_d_n6, assign23290_e21072_d_n7, assign23290_e21072_d_n8, assign23290_e21072_d_n9, assign23290_e21072_d_n10, assign23290_e21072_d_n11, assign23290_e21072_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23290_e21067: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        let assign23290_e21069: f64 = (assign23290_e21067 + 0.0001);
        let assign23290_e21070: f64 = (assign23290_e21069).sqrt();
        (assign23290_e21070, (((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)) / (2.0 * assign23290_e21070)), (((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)) / (2.0 * assign23290_e21070)), (((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)) / (2.0 * assign23290_e21070)), (((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)) / (2.0 * assign23290_e21070)), (((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)) / (2.0 * assign23290_e21070)), (((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)) / (2.0 * assign23290_e21070)), (((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)) / (2.0 * assign23290_e21070)), (((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)) / (2.0 * assign23290_e21070)), (((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)) / (2.0 * assign23290_e21070)), (((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)) / (2.0 * assign23290_e21070)),)
    } else {
        (locals.var_vgs_eff_1, locals.var_vgs_eff_1_dn3, locals.var_vgs_eff_1_dn4, locals.var_vgs_eff_1_dn5, locals.var_vgs_eff_1_dn6, locals.var_vgs_eff_1_dn7, locals.var_vgs_eff_1_dn8, locals.var_vgs_eff_1_dn9, locals.var_vgs_eff_1_dn10, locals.var_vgs_eff_1_dn11, locals.var_vgs_eff_1_dn12,)
    }
};
        locals.var_vgs_eff_1 = assign23290_e21072;
        locals.var_vgs_eff_1_dn3 = assign23290_e21072_d_n3;
        locals.var_vgs_eff_1_dn4 = assign23290_e21072_d_n4;
        locals.var_vgs_eff_1_dn5 = assign23290_e21072_d_n5;
        locals.var_vgs_eff_1_dn6 = assign23290_e21072_d_n6;
        locals.var_vgs_eff_1_dn7 = assign23290_e21072_d_n7;
        locals.var_vgs_eff_1_dn8 = assign23290_e21072_d_n8;
        locals.var_vgs_eff_1_dn9 = assign23290_e21072_d_n9;
        locals.var_vgs_eff_1_dn10 = assign23290_e21072_d_n10;
        locals.var_vgs_eff_1_dn11 = assign23290_e21072_d_n11;
        locals.var_vgs_eff_1_dn12 = assign23290_e21072_d_n12;

        let (assign23300_e21078, assign23300_e21078_d_n3, assign23300_e21078_d_n4, assign23300_e21078_d_n5, assign23300_e21078_d_n6, assign23300_e21078_d_n7, assign23300_e21078_d_n8, assign23300_e21078_d_n9, assign23300_e21078_d_n10, assign23300_e21078_d_n11, assign23300_e21078_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23300_e21076: f64 = (locals.var_vgs * locals.var_vgs_eff_1);
        (assign23300_e21076, (locals.var_vgs * locals.var_vgs_eff_1_dn3), (locals.var_vgs * locals.var_vgs_eff_1_dn4), (locals.var_vgs * locals.var_vgs_eff_1_dn5), (locals.var_vgs * locals.var_vgs_eff_1_dn6), (locals.var_vgs * locals.var_vgs_eff_1_dn7), ((locals.var_vgs_dn8 * locals.var_vgs_eff_1) + (locals.var_vgs * locals.var_vgs_eff_1_dn8)), ((locals.var_vgs_dn9 * locals.var_vgs_eff_1) + (locals.var_vgs * locals.var_vgs_eff_1_dn9)), (locals.var_vgs * locals.var_vgs_eff_1_dn10), (locals.var_vgs * locals.var_vgs_eff_1_dn11), (locals.var_vgs * locals.var_vgs_eff_1_dn12),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign23300_e21078;
        locals.var_t2__blk810_dn3 = assign23300_e21078_d_n3;
        locals.var_t2__blk810_dn4 = assign23300_e21078_d_n4;
        locals.var_t2__blk810_dn5 = assign23300_e21078_d_n5;
        locals.var_t2__blk810_dn6 = assign23300_e21078_d_n6;
        locals.var_t2__blk810_dn7 = assign23300_e21078_d_n7;
        locals.var_t2__blk810_dn8 = assign23300_e21078_d_n8;
        locals.var_t2__blk810_dn9 = assign23300_e21078_d_n9;
        locals.var_t2__blk810_dn10 = assign23300_e21078_d_n10;
        locals.var_t2__blk810_dn11 = assign23300_e21078_d_n11;
        locals.var_t2__blk810_dn12 = assign23300_e21078_d_n12;

        let (assign23310_e21082, assign23310_e21082_d_n3, assign23310_e21082_d_n4, assign23310_e21082_d_n5, assign23310_e21082_d_n6, assign23310_e21082_d_n7, assign23310_e21082_d_n8, assign23310_e21082_d_n9, assign23310_e21082_d_n10, assign23310_e21082_d_n11, assign23310_e21082_d_n12,) = {
    if (p.p375 != 0.0) {
        (locals.var_pparam_b4soiaechvbedges, locals.var_pparam_b4soiaechvbedges_dn3, locals.var_pparam_b4soiaechvbedges_dn4, locals.var_pparam_b4soiaechvbedges_dn5, locals.var_pparam_b4soiaechvbedges_dn6, locals.var_pparam_b4soiaechvbedges_dn7, locals.var_pparam_b4soiaechvbedges_dn8, locals.var_pparam_b4soiaechvbedges_dn9, locals.var_pparam_b4soiaechvbedges_dn10, locals.var_pparam_b4soiaechvbedges_dn11, locals.var_pparam_b4soiaechvbedges_dn12,)
    } else {
        (locals.var_t13, locals.var_t13_dn3, locals.var_t13_dn4, locals.var_t13_dn5, locals.var_t13_dn6, locals.var_t13_dn7, locals.var_t13_dn8, locals.var_t13_dn9, locals.var_t13_dn10, locals.var_t13_dn11, locals.var_t13_dn12,)
    }
};
        locals.var_t13 = assign23310_e21082;
        locals.var_t13_dn3 = assign23310_e21082_d_n3;
        locals.var_t13_dn4 = assign23310_e21082_d_n4;
        locals.var_t13_dn5 = assign23310_e21082_d_n5;
        locals.var_t13_dn6 = assign23310_e21082_d_n6;
        locals.var_t13_dn7 = assign23310_e21082_d_n7;
        locals.var_t13_dn8 = assign23310_e21082_d_n8;
        locals.var_t13_dn9 = assign23310_e21082_d_n9;
        locals.var_t13_dn10 = assign23310_e21082_d_n10;
        locals.var_t13_dn11 = assign23310_e21082_d_n11;
        locals.var_t13_dn12 = assign23310_e21082_d_n12;

        let (assign23320_e21086, assign23320_e21086_d_n3, assign23320_e21086_d_n4, assign23320_e21086_d_n5, assign23320_e21086_d_n6, assign23320_e21086_d_n7, assign23320_e21086_d_n8, assign23320_e21086_d_n9, assign23320_e21086_d_n10, assign23320_e21086_d_n11, assign23320_e21086_d_n12,) = {
    if (p.p375 != 0.0) {
        (locals.var_pparam_b4soiaechvbedged, locals.var_pparam_b4soiaechvbedged_dn3, locals.var_pparam_b4soiaechvbedged_dn4, locals.var_pparam_b4soiaechvbedged_dn5, locals.var_pparam_b4soiaechvbedged_dn6, locals.var_pparam_b4soiaechvbedged_dn7, locals.var_pparam_b4soiaechvbedged_dn8, locals.var_pparam_b4soiaechvbedged_dn9, locals.var_pparam_b4soiaechvbedged_dn10, locals.var_pparam_b4soiaechvbedged_dn11, locals.var_pparam_b4soiaechvbedged_dn12,)
    } else {
        (locals.var_t14, locals.var_t14_dn3, locals.var_t14_dn4, locals.var_t14_dn5, locals.var_t14_dn6, locals.var_t14_dn7, locals.var_t14_dn8, locals.var_t14_dn9, locals.var_t14_dn10, locals.var_t14_dn11, locals.var_t14_dn12,)
    }
};
        locals.var_t14 = assign23320_e21086;
        locals.var_t14_dn3 = assign23320_e21086_d_n3;
        locals.var_t14_dn4 = assign23320_e21086_d_n4;
        locals.var_t14_dn5 = assign23320_e21086_d_n5;
        locals.var_t14_dn6 = assign23320_e21086_d_n6;
        locals.var_t14_dn7 = assign23320_e21086_d_n7;
        locals.var_t14_dn8 = assign23320_e21086_d_n8;
        locals.var_t14_dn9 = assign23320_e21086_d_n9;
        locals.var_t14_dn10 = assign23320_e21086_d_n10;
        locals.var_t14_dn11 = assign23320_e21086_d_n11;
        locals.var_t14_dn12 = assign23320_e21086_d_n12;

        let (assign23330_e21090, assign23330_e21090_d_n3, assign23330_e21090_d_n4, assign23330_e21090_d_n5, assign23330_e21090_d_n6, assign23330_e21090_d_n7, assign23330_e21090_d_n8, assign23330_e21090_d_n9, assign23330_e21090_d_n10, assign23330_e21090_d_n11, assign23330_e21090_d_n12,) = {
    if (p.p375 != 0.0) {
        (locals.var_pparam_b4soibechvbedge, locals.var_pparam_b4soibechvbedge_dn3, locals.var_pparam_b4soibechvbedge_dn4, locals.var_pparam_b4soibechvbedge_dn5, locals.var_pparam_b4soibechvbedge_dn6, locals.var_pparam_b4soibechvbedge_dn7, locals.var_pparam_b4soibechvbedge_dn8, locals.var_pparam_b4soibechvbedge_dn9, locals.var_pparam_b4soibechvbedge_dn10, locals.var_pparam_b4soibechvbedge_dn11, locals.var_pparam_b4soibechvbedge_dn12,)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12,)
    }
};
        locals.var_t12 = assign23330_e21090;
        locals.var_t12_dn3 = assign23330_e21090_d_n3;
        locals.var_t12_dn4 = assign23330_e21090_d_n4;
        locals.var_t12_dn5 = assign23330_e21090_d_n5;
        locals.var_t12_dn6 = assign23330_e21090_d_n6;
        locals.var_t12_dn7 = assign23330_e21090_d_n7;
        locals.var_t12_dn8 = assign23330_e21090_d_n8;
        locals.var_t12_dn9 = assign23330_e21090_d_n9;
        locals.var_t12_dn10 = assign23330_e21090_d_n10;
        locals.var_t12_dn11 = assign23330_e21090_d_n11;
        locals.var_t12_dn12 = assign23330_e21090_d_n12;

        let (assign23340_e21098, assign23340_e21098_d_n3, assign23340_e21098_d_n4, assign23340_e21098_d_n5, assign23340_e21098_d_n6, assign23340_e21098_d_n7, assign23340_e21098_d_n8, assign23340_e21098_d_n9, assign23340_e21098_d_n10, assign23340_e21098_d_n11, assign23340_e21098_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23340_e21094: f64 = (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd);
        let assign23340_e21096: f64 = (assign23340_e21094 - locals.var_pparam_b4soibigsd);
        (assign23340_e21096, (((locals.var_pparam_b4soiaigsd_dn3 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd_dn3)) - locals.var_pparam_b4soibigsd_dn3), (((locals.var_pparam_b4soiaigsd_dn4 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd_dn4)) - locals.var_pparam_b4soibigsd_dn4), (((locals.var_pparam_b4soiaigsd_dn5 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd_dn5)) - locals.var_pparam_b4soibigsd_dn5), (((locals.var_pparam_b4soiaigsd_dn6 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd_dn6)) - locals.var_pparam_b4soibigsd_dn6), (((locals.var_pparam_b4soiaigsd_dn7 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd_dn7)) - locals.var_pparam_b4soibigsd_dn7), (((locals.var_pparam_b4soiaigsd_dn8 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd_dn8)) - locals.var_pparam_b4soibigsd_dn8), (((locals.var_pparam_b4soiaigsd_dn9 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd_dn9)) - locals.var_pparam_b4soibigsd_dn9), (((locals.var_pparam_b4soiaigsd_dn10 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd_dn10)) - locals.var_pparam_b4soibigsd_dn10), (((locals.var_pparam_b4soiaigsd_dn11 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd_dn11)) - locals.var_pparam_b4soibigsd_dn11), (((locals.var_pparam_b4soiaigsd_dn12 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soiaigsd * locals.var_pparam_b4soicigsd_dn12)) - locals.var_pparam_b4soibigsd_dn12),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign23340_e21098;
        locals.var_t3__blk811_dn3 = assign23340_e21098_d_n3;
        locals.var_t3__blk811_dn4 = assign23340_e21098_d_n4;
        locals.var_t3__blk811_dn5 = assign23340_e21098_d_n5;
        locals.var_t3__blk811_dn6 = assign23340_e21098_d_n6;
        locals.var_t3__blk811_dn7 = assign23340_e21098_d_n7;
        locals.var_t3__blk811_dn8 = assign23340_e21098_d_n8;
        locals.var_t3__blk811_dn9 = assign23340_e21098_d_n9;
        locals.var_t3__blk811_dn10 = assign23340_e21098_d_n10;
        locals.var_t3__blk811_dn11 = assign23340_e21098_d_n11;
        locals.var_t3__blk811_dn12 = assign23340_e21098_d_n12;

    }

    pub(super) fn stamp_transient_block_69(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23350_e21104, assign23350_e21104_d_n3, assign23350_e21104_d_n4, assign23350_e21104_d_n5, assign23350_e21104_d_n6, assign23350_e21104_d_n7, assign23350_e21104_d_n8, assign23350_e21104_d_n9, assign23350_e21104_d_n10, assign23350_e21104_d_n11, assign23350_e21104_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23350_e21102: f64 = (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd);
        (assign23350_e21102, ((locals.var_pparam_b4soibigsd_dn3 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd_dn3)), ((locals.var_pparam_b4soibigsd_dn4 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd_dn4)), ((locals.var_pparam_b4soibigsd_dn5 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd_dn5)), ((locals.var_pparam_b4soibigsd_dn6 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd_dn6)), ((locals.var_pparam_b4soibigsd_dn7 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd_dn7)), ((locals.var_pparam_b4soibigsd_dn8 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd_dn8)), ((locals.var_pparam_b4soibigsd_dn9 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd_dn9)), ((locals.var_pparam_b4soibigsd_dn10 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd_dn10)), ((locals.var_pparam_b4soibigsd_dn11 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd_dn11)), ((locals.var_pparam_b4soibigsd_dn12 * locals.var_pparam_b4soicigsd) + (locals.var_pparam_b4soibigsd * locals.var_pparam_b4soicigsd_dn12)),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign23350_e21104;
        locals.var_t4__blk812_dn3 = assign23350_e21104_d_n3;
        locals.var_t4__blk812_dn4 = assign23350_e21104_d_n4;
        locals.var_t4__blk812_dn5 = assign23350_e21104_d_n5;
        locals.var_t4__blk812_dn6 = assign23350_e21104_d_n6;
        locals.var_t4__blk812_dn7 = assign23350_e21104_d_n7;
        locals.var_t4__blk812_dn8 = assign23350_e21104_d_n8;
        locals.var_t4__blk812_dn9 = assign23350_e21104_d_n9;
        locals.var_t4__blk812_dn10 = assign23350_e21104_d_n10;
        locals.var_t4__blk812_dn11 = assign23350_e21104_d_n11;
        locals.var_t4__blk812_dn12 = assign23350_e21104_d_n12;

        let (assign23360_e21120, assign23360_e21120_d_n3, assign23360_e21120_d_n4, assign23360_e21120_d_n5, assign23360_e21120_d_n6, assign23360_e21120_d_n7, assign23360_e21120_d_n8, assign23360_e21120_d_n9, assign23360_e21120_d_n10, assign23360_e21120_d_n11, assign23360_e21120_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23360_e21110: f64 = (locals.var_t3__blk811 * locals.var_vgs_eff_1);
        let assign23360_e21111: f64 = (locals.var_pparam_b4soiaigsd + assign23360_e21110);
        let assign23360_e21114: f64 = (locals.var_t4__blk812 * locals.var_vgs_eff_1);
        let assign23360_e21116: f64 = (assign23360_e21114 * locals.var_vgs_eff_1);
        let assign23360_e21117: f64 = (assign23360_e21111 - assign23360_e21116);
        let assign23360_e21118: f64 = (locals.var_t12 * assign23360_e21117);
        (assign23360_e21118, ((locals.var_t12_dn3 * assign23360_e21117) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn3 + ((locals.var_t3__blk811_dn3 * locals.var_vgs_eff_1) + (locals.var_t3__blk811 * locals.var_vgs_eff_1_dn3))) - ((((locals.var_t4__blk812_dn3 * locals.var_vgs_eff_1) + (locals.var_t4__blk812 * locals.var_vgs_eff_1_dn3)) * locals.var_vgs_eff_1) + (assign23360_e21114 * locals.var_vgs_eff_1_dn3))))), ((locals.var_t12_dn4 * assign23360_e21117) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn4 + ((locals.var_t3__blk811_dn4 * locals.var_vgs_eff_1) + (locals.var_t3__blk811 * locals.var_vgs_eff_1_dn4))) - ((((locals.var_t4__blk812_dn4 * locals.var_vgs_eff_1) + (locals.var_t4__blk812 * locals.var_vgs_eff_1_dn4)) * locals.var_vgs_eff_1) + (assign23360_e21114 * locals.var_vgs_eff_1_dn4))))), ((locals.var_t12_dn5 * assign23360_e21117) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn5 + ((locals.var_t3__blk811_dn5 * locals.var_vgs_eff_1) + (locals.var_t3__blk811 * locals.var_vgs_eff_1_dn5))) - ((((locals.var_t4__blk812_dn5 * locals.var_vgs_eff_1) + (locals.var_t4__blk812 * locals.var_vgs_eff_1_dn5)) * locals.var_vgs_eff_1) + (assign23360_e21114 * locals.var_vgs_eff_1_dn5))))), ((locals.var_t12_dn6 * assign23360_e21117) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn6 + ((locals.var_t3__blk811_dn6 * locals.var_vgs_eff_1) + (locals.var_t3__blk811 * locals.var_vgs_eff_1_dn6))) - ((((locals.var_t4__blk812_dn6 * locals.var_vgs_eff_1) + (locals.var_t4__blk812 * locals.var_vgs_eff_1_dn6)) * locals.var_vgs_eff_1) + (assign23360_e21114 * locals.var_vgs_eff_1_dn6))))), ((locals.var_t12_dn7 * assign23360_e21117) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn7 + ((locals.var_t3__blk811_dn7 * locals.var_vgs_eff_1) + (locals.var_t3__blk811 * locals.var_vgs_eff_1_dn7))) - ((((locals.var_t4__blk812_dn7 * locals.var_vgs_eff_1) + (locals.var_t4__blk812 * locals.var_vgs_eff_1_dn7)) * locals.var_vgs_eff_1) + (assign23360_e21114 * locals.var_vgs_eff_1_dn7))))), ((locals.var_t12_dn8 * assign23360_e21117) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn8 + ((locals.var_t3__blk811_dn8 * locals.var_vgs_eff_1) + (locals.var_t3__blk811 * locals.var_vgs_eff_1_dn8))) - ((((locals.var_t4__blk812_dn8 * locals.var_vgs_eff_1) + (locals.var_t4__blk812 * locals.var_vgs_eff_1_dn8)) * locals.var_vgs_eff_1) + (assign23360_e21114 * locals.var_vgs_eff_1_dn8))))), ((locals.var_t12_dn9 * assign23360_e21117) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn9 + ((locals.var_t3__blk811_dn9 * locals.var_vgs_eff_1) + (locals.var_t3__blk811 * locals.var_vgs_eff_1_dn9))) - ((((locals.var_t4__blk812_dn9 * locals.var_vgs_eff_1) + (locals.var_t4__blk812 * locals.var_vgs_eff_1_dn9)) * locals.var_vgs_eff_1) + (assign23360_e21114 * locals.var_vgs_eff_1_dn9))))), ((locals.var_t12_dn10 * assign23360_e21117) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn10 + ((locals.var_t3__blk811_dn10 * locals.var_vgs_eff_1) + (locals.var_t3__blk811 * locals.var_vgs_eff_1_dn10))) - ((((locals.var_t4__blk812_dn10 * locals.var_vgs_eff_1) + (locals.var_t4__blk812 * locals.var_vgs_eff_1_dn10)) * locals.var_vgs_eff_1) + (assign23360_e21114 * locals.var_vgs_eff_1_dn10))))), ((locals.var_t12_dn11 * assign23360_e21117) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn11 + ((locals.var_t3__blk811_dn11 * locals.var_vgs_eff_1) + (locals.var_t3__blk811 * locals.var_vgs_eff_1_dn11))) - ((((locals.var_t4__blk812_dn11 * locals.var_vgs_eff_1) + (locals.var_t4__blk812 * locals.var_vgs_eff_1_dn11)) * locals.var_vgs_eff_1) + (assign23360_e21114 * locals.var_vgs_eff_1_dn11))))), ((locals.var_t12_dn12 * assign23360_e21117) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn12 + ((locals.var_t3__blk811_dn12 * locals.var_vgs_eff_1) + (locals.var_t3__blk811 * locals.var_vgs_eff_1_dn12))) - ((((locals.var_t4__blk812_dn12 * locals.var_vgs_eff_1) + (locals.var_t4__blk812 * locals.var_vgs_eff_1_dn12)) * locals.var_vgs_eff_1) + (assign23360_e21114 * locals.var_vgs_eff_1_dn12))))),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign23360_e21120;
        locals.var_t5__blk813_dn3 = assign23360_e21120_d_n3;
        locals.var_t5__blk813_dn4 = assign23360_e21120_d_n4;
        locals.var_t5__blk813_dn5 = assign23360_e21120_d_n5;
        locals.var_t5__blk813_dn6 = assign23360_e21120_d_n6;
        locals.var_t5__blk813_dn7 = assign23360_e21120_d_n7;
        locals.var_t5__blk813_dn8 = assign23360_e21120_d_n8;
        locals.var_t5__blk813_dn9 = assign23360_e21120_d_n9;
        locals.var_t5__blk813_dn10 = assign23360_e21120_d_n10;
        locals.var_t5__blk813_dn11 = assign23360_e21120_d_n11;
        locals.var_t5__blk813_dn12 = assign23360_e21120_d_n12;

        let assign23370_e21123: f64 = if locals.var_t5__blk813 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1302 = assign23370_e21123;

        let (assign23380_e21129, assign23380_e21129_d_n3, assign23380_e21129_d_n4, assign23380_e21129_d_n5, assign23380_e21129_d_n6, assign23380_e21129_d_n7, assign23380_e21129_d_n8, assign23380_e21129_d_n9, assign23380_e21129_d_n10, assign23380_e21129_d_n11, assign23380_e21129_d_n12,) = {
    if ((p.p375 != 0.0) && (locals.var_guard1302 != 0.0)) {
        (2.688117142e43, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign23380_e21129;
        locals.var_t6__blk814_dn3 = assign23380_e21129_d_n3;
        locals.var_t6__blk814_dn4 = assign23380_e21129_d_n4;
        locals.var_t6__blk814_dn5 = assign23380_e21129_d_n5;
        locals.var_t6__blk814_dn6 = assign23380_e21129_d_n6;
        locals.var_t6__blk814_dn7 = assign23380_e21129_d_n7;
        locals.var_t6__blk814_dn8 = assign23380_e21129_d_n8;
        locals.var_t6__blk814_dn9 = assign23380_e21129_d_n9;
        locals.var_t6__blk814_dn10 = assign23380_e21129_d_n10;
        locals.var_t6__blk814_dn11 = assign23380_e21129_d_n11;
        locals.var_t6__blk814_dn12 = assign23380_e21129_d_n12;

        let assign23390_e21132: f64 = (-100.0);
        let assign23390_e21133: f64 = if locals.var_t5__blk813 < assign23390_e21132 { 1.0 } else { 0.0 };
        locals.var_guard1303 = assign23390_e21133;

        let (assign23400_e21142, assign23400_e21142_d_n3, assign23400_e21142_d_n4, assign23400_e21142_d_n5, assign23400_e21142_d_n6, assign23400_e21142_d_n7, assign23400_e21142_d_n8, assign23400_e21142_d_n9, assign23400_e21142_d_n10, assign23400_e21142_d_n11, assign23400_e21142_d_n12,) = {
    if (((p.p375 != 0.0) && (locals.var_guard1302 == 0.0)) && (locals.var_guard1303 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign23400_e21142;
        locals.var_t6__blk814_dn3 = assign23400_e21142_d_n3;
        locals.var_t6__blk814_dn4 = assign23400_e21142_d_n4;
        locals.var_t6__blk814_dn5 = assign23400_e21142_d_n5;
        locals.var_t6__blk814_dn6 = assign23400_e21142_d_n6;
        locals.var_t6__blk814_dn7 = assign23400_e21142_d_n7;
        locals.var_t6__blk814_dn8 = assign23400_e21142_d_n8;
        locals.var_t6__blk814_dn9 = assign23400_e21142_d_n9;
        locals.var_t6__blk814_dn10 = assign23400_e21142_d_n10;
        locals.var_t6__blk814_dn11 = assign23400_e21142_d_n11;
        locals.var_t6__blk814_dn12 = assign23400_e21142_d_n12;

        let (assign23410_e21153, assign23410_e21153_d_n3, assign23410_e21153_d_n4, assign23410_e21153_d_n5, assign23410_e21153_d_n6, assign23410_e21153_d_n7, assign23410_e21153_d_n8, assign23410_e21153_d_n9, assign23410_e21153_d_n10, assign23410_e21153_d_n11, assign23410_e21153_d_n12,) = {
    if (((p.p375 != 0.0) && (locals.var_guard1302 == 0.0)) && (locals.var_guard1303 == 0.0)) {
        let assign23410_e21151: f64 = (locals.var_t5__blk813).exp();
        (assign23410_e21151, (assign23410_e21151 * locals.var_t5__blk813_dn3), (assign23410_e21151 * locals.var_t5__blk813_dn4), (assign23410_e21151 * locals.var_t5__blk813_dn5), (assign23410_e21151 * locals.var_t5__blk813_dn6), (assign23410_e21151 * locals.var_t5__blk813_dn7), (assign23410_e21151 * locals.var_t5__blk813_dn8), (assign23410_e21151 * locals.var_t5__blk813_dn9), (assign23410_e21151 * locals.var_t5__blk813_dn10), (assign23410_e21151 * locals.var_t5__blk813_dn11), (assign23410_e21151 * locals.var_t5__blk813_dn12),)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign23410_e21153;
        locals.var_t6__blk814_dn3 = assign23410_e21153_d_n3;
        locals.var_t6__blk814_dn4 = assign23410_e21153_d_n4;
        locals.var_t6__blk814_dn5 = assign23410_e21153_d_n5;
        locals.var_t6__blk814_dn6 = assign23410_e21153_d_n6;
        locals.var_t6__blk814_dn7 = assign23410_e21153_d_n7;
        locals.var_t6__blk814_dn8 = assign23410_e21153_d_n8;
        locals.var_t6__blk814_dn9 = assign23410_e21153_d_n9;
        locals.var_t6__blk814_dn10 = assign23410_e21153_d_n10;
        locals.var_t6__blk814_dn11 = assign23410_e21153_d_n11;
        locals.var_t6__blk814_dn12 = assign23410_e21153_d_n12;

        let (assign23420_e21163, assign23420_e21163_d_n3, assign23420_e21163_d_n4, assign23420_e21163_d_n5, assign23420_e21163_d_n6, assign23420_e21163_d_n7, assign23420_e21163_d_n8, assign23420_e21163_d_n9, assign23420_e21163_d_n10, assign23420_e21163_d_n11, assign23420_e21163_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23420_e21157: f64 = (locals.var_t13 * locals.var_t2__blk810);
        let assign23420_e21159: f64 = (assign23420_e21157 * locals.var_t6__blk814);
        let assign23420_e21161: f64 = (assign23420_e21159 * locals.var_igtemp);
        (assign23420_e21161, ((((((locals.var_t13_dn3 * locals.var_t2__blk810) + (locals.var_t13 * locals.var_t2__blk810_dn3)) * locals.var_t6__blk814) + (assign23420_e21157 * locals.var_t6__blk814_dn3)) * locals.var_igtemp) + (assign23420_e21159 * locals.var_igtemp_dn3)), ((((((locals.var_t13_dn4 * locals.var_t2__blk810) + (locals.var_t13 * locals.var_t2__blk810_dn4)) * locals.var_t6__blk814) + (assign23420_e21157 * locals.var_t6__blk814_dn4)) * locals.var_igtemp) + (assign23420_e21159 * locals.var_igtemp_dn4)), ((((((locals.var_t13_dn5 * locals.var_t2__blk810) + (locals.var_t13 * locals.var_t2__blk810_dn5)) * locals.var_t6__blk814) + (assign23420_e21157 * locals.var_t6__blk814_dn5)) * locals.var_igtemp) + (assign23420_e21159 * locals.var_igtemp_dn5)), ((((((locals.var_t13_dn6 * locals.var_t2__blk810) + (locals.var_t13 * locals.var_t2__blk810_dn6)) * locals.var_t6__blk814) + (assign23420_e21157 * locals.var_t6__blk814_dn6)) * locals.var_igtemp) + (assign23420_e21159 * locals.var_igtemp_dn6)), ((((((locals.var_t13_dn7 * locals.var_t2__blk810) + (locals.var_t13 * locals.var_t2__blk810_dn7)) * locals.var_t6__blk814) + (assign23420_e21157 * locals.var_t6__blk814_dn7)) * locals.var_igtemp) + (assign23420_e21159 * locals.var_igtemp_dn7)), ((((((locals.var_t13_dn8 * locals.var_t2__blk810) + (locals.var_t13 * locals.var_t2__blk810_dn8)) * locals.var_t6__blk814) + (assign23420_e21157 * locals.var_t6__blk814_dn8)) * locals.var_igtemp) + (assign23420_e21159 * locals.var_igtemp_dn8)), ((((((locals.var_t13_dn9 * locals.var_t2__blk810) + (locals.var_t13 * locals.var_t2__blk810_dn9)) * locals.var_t6__blk814) + (assign23420_e21157 * locals.var_t6__blk814_dn9)) * locals.var_igtemp) + (assign23420_e21159 * locals.var_igtemp_dn9)), ((((((locals.var_t13_dn10 * locals.var_t2__blk810) + (locals.var_t13 * locals.var_t2__blk810_dn10)) * locals.var_t6__blk814) + (assign23420_e21157 * locals.var_t6__blk814_dn10)) * locals.var_igtemp) + (assign23420_e21159 * locals.var_igtemp_dn10)), ((((((locals.var_t13_dn11 * locals.var_t2__blk810) + (locals.var_t13 * locals.var_t2__blk810_dn11)) * locals.var_t6__blk814) + (assign23420_e21157 * locals.var_t6__blk814_dn11)) * locals.var_igtemp) + (assign23420_e21159 * locals.var_igtemp_dn11)), ((((((locals.var_t13_dn12 * locals.var_t2__blk810) + (locals.var_t13 * locals.var_t2__blk810_dn12)) * locals.var_t6__blk814) + (assign23420_e21157 * locals.var_t6__blk814_dn12)) * locals.var_igtemp) + (assign23420_e21159 * locals.var_igtemp_dn12)),)
    } else {
        (locals.var_igs_1, locals.var_igs_1_dn3, locals.var_igs_1_dn4, locals.var_igs_1_dn5, locals.var_igs_1_dn6, locals.var_igs_1_dn7, locals.var_igs_1_dn8, locals.var_igs_1_dn9, locals.var_igs_1_dn10, locals.var_igs_1_dn11, locals.var_igs_1_dn12,)
    }
};
        locals.var_igs_1 = assign23420_e21163;
        locals.var_igs_1_dn3 = assign23420_e21163_d_n3;
        locals.var_igs_1_dn4 = assign23420_e21163_d_n4;
        locals.var_igs_1_dn5 = assign23420_e21163_d_n5;
        locals.var_igs_1_dn6 = assign23420_e21163_d_n6;
        locals.var_igs_1_dn7 = assign23420_e21163_d_n7;
        locals.var_igs_1_dn8 = assign23420_e21163_d_n8;
        locals.var_igs_1_dn9 = assign23420_e21163_d_n9;
        locals.var_igs_1_dn10 = assign23420_e21163_d_n10;
        locals.var_igs_1_dn11 = assign23420_e21163_d_n11;
        locals.var_igs_1_dn12 = assign23420_e21163_d_n12;

        let (assign23430_e21169, assign23430_e21169_d_n3, assign23430_e21169_d_n4, assign23430_e21169_d_n5, assign23430_e21169_d_n6, assign23430_e21169_d_n7, assign23430_e21169_d_n8, assign23430_e21169_d_n9, assign23430_e21169_d_n10, assign23430_e21169_d_n11, assign23430_e21169_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23430_e21167: f64 = (locals.var_vgd - locals.var_pparam_b4soivfbsd);
        (assign23430_e21167, (-locals.var_pparam_b4soivfbsd_dn3), (-locals.var_pparam_b4soivfbsd_dn4), (-locals.var_pparam_b4soivfbsd_dn5), (-locals.var_pparam_b4soivfbsd_dn6), (locals.var_vgd_dn7 - locals.var_pparam_b4soivfbsd_dn7), (locals.var_vgd_dn8 - locals.var_pparam_b4soivfbsd_dn8), (locals.var_vgd_dn9 - locals.var_pparam_b4soivfbsd_dn9), (-locals.var_pparam_b4soivfbsd_dn10), (-locals.var_pparam_b4soivfbsd_dn11), (-locals.var_pparam_b4soivfbsd_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign23430_e21169;
        locals.var_t0__blk808_dn3 = assign23430_e21169_d_n3;
        locals.var_t0__blk808_dn4 = assign23430_e21169_d_n4;
        locals.var_t0__blk808_dn5 = assign23430_e21169_d_n5;
        locals.var_t0__blk808_dn6 = assign23430_e21169_d_n6;
        locals.var_t0__blk808_dn7 = assign23430_e21169_d_n7;
        locals.var_t0__blk808_dn8 = assign23430_e21169_d_n8;
        locals.var_t0__blk808_dn9 = assign23430_e21169_d_n9;
        locals.var_t0__blk808_dn10 = assign23430_e21169_d_n10;
        locals.var_t0__blk808_dn11 = assign23430_e21169_d_n11;
        locals.var_t0__blk808_dn12 = assign23430_e21169_d_n12;

        let (assign23440_e21178, assign23440_e21178_d_n3, assign23440_e21178_d_n4, assign23440_e21178_d_n5, assign23440_e21178_d_n6, assign23440_e21178_d_n7, assign23440_e21178_d_n8, assign23440_e21178_d_n9, assign23440_e21178_d_n10, assign23440_e21178_d_n11, assign23440_e21178_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23440_e21173: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        let assign23440_e21175: f64 = (assign23440_e21173 + 0.0001);
        let assign23440_e21176: f64 = (assign23440_e21175).sqrt();
        (assign23440_e21176, (((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)) / (2.0 * assign23440_e21176)), (((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)) / (2.0 * assign23440_e21176)), (((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)) / (2.0 * assign23440_e21176)), (((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)) / (2.0 * assign23440_e21176)), (((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)) / (2.0 * assign23440_e21176)), (((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)) / (2.0 * assign23440_e21176)), (((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)) / (2.0 * assign23440_e21176)), (((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)) / (2.0 * assign23440_e21176)), (((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)) / (2.0 * assign23440_e21176)), (((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)) / (2.0 * assign23440_e21176)),)
    } else {
        (locals.var_vgd_eff, locals.var_vgd_eff_dn3, locals.var_vgd_eff_dn4, locals.var_vgd_eff_dn5, locals.var_vgd_eff_dn6, locals.var_vgd_eff_dn7, locals.var_vgd_eff_dn8, locals.var_vgd_eff_dn9, locals.var_vgd_eff_dn10, locals.var_vgd_eff_dn11, locals.var_vgd_eff_dn12,)
    }
};
        locals.var_vgd_eff = assign23440_e21178;
        locals.var_vgd_eff_dn3 = assign23440_e21178_d_n3;
        locals.var_vgd_eff_dn4 = assign23440_e21178_d_n4;
        locals.var_vgd_eff_dn5 = assign23440_e21178_d_n5;
        locals.var_vgd_eff_dn6 = assign23440_e21178_d_n6;
        locals.var_vgd_eff_dn7 = assign23440_e21178_d_n7;
        locals.var_vgd_eff_dn8 = assign23440_e21178_d_n8;
        locals.var_vgd_eff_dn9 = assign23440_e21178_d_n9;
        locals.var_vgd_eff_dn10 = assign23440_e21178_d_n10;
        locals.var_vgd_eff_dn11 = assign23440_e21178_d_n11;
        locals.var_vgd_eff_dn12 = assign23440_e21178_d_n12;

        let (assign23450_e21184, assign23450_e21184_d_n3, assign23450_e21184_d_n4, assign23450_e21184_d_n5, assign23450_e21184_d_n6, assign23450_e21184_d_n7, assign23450_e21184_d_n8, assign23450_e21184_d_n9, assign23450_e21184_d_n10, assign23450_e21184_d_n11, assign23450_e21184_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23450_e21182: f64 = (locals.var_vgd * locals.var_vgd_eff);
        (assign23450_e21182, (locals.var_vgd * locals.var_vgd_eff_dn3), (locals.var_vgd * locals.var_vgd_eff_dn4), (locals.var_vgd * locals.var_vgd_eff_dn5), (locals.var_vgd * locals.var_vgd_eff_dn6), ((locals.var_vgd_dn7 * locals.var_vgd_eff) + (locals.var_vgd * locals.var_vgd_eff_dn7)), ((locals.var_vgd_dn8 * locals.var_vgd_eff) + (locals.var_vgd * locals.var_vgd_eff_dn8)), ((locals.var_vgd_dn9 * locals.var_vgd_eff) + (locals.var_vgd * locals.var_vgd_eff_dn9)), (locals.var_vgd * locals.var_vgd_eff_dn10), (locals.var_vgd * locals.var_vgd_eff_dn11), (locals.var_vgd * locals.var_vgd_eff_dn12),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign23450_e21184;
        locals.var_t2__blk810_dn3 = assign23450_e21184_d_n3;
        locals.var_t2__blk810_dn4 = assign23450_e21184_d_n4;
        locals.var_t2__blk810_dn5 = assign23450_e21184_d_n5;
        locals.var_t2__blk810_dn6 = assign23450_e21184_d_n6;
        locals.var_t2__blk810_dn7 = assign23450_e21184_d_n7;
        locals.var_t2__blk810_dn8 = assign23450_e21184_d_n8;
        locals.var_t2__blk810_dn9 = assign23450_e21184_d_n9;
        locals.var_t2__blk810_dn10 = assign23450_e21184_d_n10;
        locals.var_t2__blk810_dn11 = assign23450_e21184_d_n11;
        locals.var_t2__blk810_dn12 = assign23450_e21184_d_n12;

        let (assign23460_e21200, assign23460_e21200_d_n3, assign23460_e21200_d_n4, assign23460_e21200_d_n5, assign23460_e21200_d_n6, assign23460_e21200_d_n7, assign23460_e21200_d_n8, assign23460_e21200_d_n9, assign23460_e21200_d_n10, assign23460_e21200_d_n11, assign23460_e21200_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23460_e21190: f64 = (locals.var_t3__blk811 * locals.var_vgd_eff);
        let assign23460_e21191: f64 = (locals.var_pparam_b4soiaigsd + assign23460_e21190);
        let assign23460_e21194: f64 = (locals.var_t4__blk812 * locals.var_vgd_eff);
        let assign23460_e21196: f64 = (assign23460_e21194 * locals.var_vgd_eff);
        let assign23460_e21197: f64 = (assign23460_e21191 - assign23460_e21196);
        let assign23460_e21198: f64 = (locals.var_t12 * assign23460_e21197);
        (assign23460_e21198, ((locals.var_t12_dn3 * assign23460_e21197) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn3 + ((locals.var_t3__blk811_dn3 * locals.var_vgd_eff) + (locals.var_t3__blk811 * locals.var_vgd_eff_dn3))) - ((((locals.var_t4__blk812_dn3 * locals.var_vgd_eff) + (locals.var_t4__blk812 * locals.var_vgd_eff_dn3)) * locals.var_vgd_eff) + (assign23460_e21194 * locals.var_vgd_eff_dn3))))), ((locals.var_t12_dn4 * assign23460_e21197) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn4 + ((locals.var_t3__blk811_dn4 * locals.var_vgd_eff) + (locals.var_t3__blk811 * locals.var_vgd_eff_dn4))) - ((((locals.var_t4__blk812_dn4 * locals.var_vgd_eff) + (locals.var_t4__blk812 * locals.var_vgd_eff_dn4)) * locals.var_vgd_eff) + (assign23460_e21194 * locals.var_vgd_eff_dn4))))), ((locals.var_t12_dn5 * assign23460_e21197) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn5 + ((locals.var_t3__blk811_dn5 * locals.var_vgd_eff) + (locals.var_t3__blk811 * locals.var_vgd_eff_dn5))) - ((((locals.var_t4__blk812_dn5 * locals.var_vgd_eff) + (locals.var_t4__blk812 * locals.var_vgd_eff_dn5)) * locals.var_vgd_eff) + (assign23460_e21194 * locals.var_vgd_eff_dn5))))), ((locals.var_t12_dn6 * assign23460_e21197) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn6 + ((locals.var_t3__blk811_dn6 * locals.var_vgd_eff) + (locals.var_t3__blk811 * locals.var_vgd_eff_dn6))) - ((((locals.var_t4__blk812_dn6 * locals.var_vgd_eff) + (locals.var_t4__blk812 * locals.var_vgd_eff_dn6)) * locals.var_vgd_eff) + (assign23460_e21194 * locals.var_vgd_eff_dn6))))), ((locals.var_t12_dn7 * assign23460_e21197) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn7 + ((locals.var_t3__blk811_dn7 * locals.var_vgd_eff) + (locals.var_t3__blk811 * locals.var_vgd_eff_dn7))) - ((((locals.var_t4__blk812_dn7 * locals.var_vgd_eff) + (locals.var_t4__blk812 * locals.var_vgd_eff_dn7)) * locals.var_vgd_eff) + (assign23460_e21194 * locals.var_vgd_eff_dn7))))), ((locals.var_t12_dn8 * assign23460_e21197) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn8 + ((locals.var_t3__blk811_dn8 * locals.var_vgd_eff) + (locals.var_t3__blk811 * locals.var_vgd_eff_dn8))) - ((((locals.var_t4__blk812_dn8 * locals.var_vgd_eff) + (locals.var_t4__blk812 * locals.var_vgd_eff_dn8)) * locals.var_vgd_eff) + (assign23460_e21194 * locals.var_vgd_eff_dn8))))), ((locals.var_t12_dn9 * assign23460_e21197) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn9 + ((locals.var_t3__blk811_dn9 * locals.var_vgd_eff) + (locals.var_t3__blk811 * locals.var_vgd_eff_dn9))) - ((((locals.var_t4__blk812_dn9 * locals.var_vgd_eff) + (locals.var_t4__blk812 * locals.var_vgd_eff_dn9)) * locals.var_vgd_eff) + (assign23460_e21194 * locals.var_vgd_eff_dn9))))), ((locals.var_t12_dn10 * assign23460_e21197) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn10 + ((locals.var_t3__blk811_dn10 * locals.var_vgd_eff) + (locals.var_t3__blk811 * locals.var_vgd_eff_dn10))) - ((((locals.var_t4__blk812_dn10 * locals.var_vgd_eff) + (locals.var_t4__blk812 * locals.var_vgd_eff_dn10)) * locals.var_vgd_eff) + (assign23460_e21194 * locals.var_vgd_eff_dn10))))), ((locals.var_t12_dn11 * assign23460_e21197) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn11 + ((locals.var_t3__blk811_dn11 * locals.var_vgd_eff) + (locals.var_t3__blk811 * locals.var_vgd_eff_dn11))) - ((((locals.var_t4__blk812_dn11 * locals.var_vgd_eff) + (locals.var_t4__blk812 * locals.var_vgd_eff_dn11)) * locals.var_vgd_eff) + (assign23460_e21194 * locals.var_vgd_eff_dn11))))), ((locals.var_t12_dn12 * assign23460_e21197) + (locals.var_t12 * ((locals.var_pparam_b4soiaigsd_dn12 + ((locals.var_t3__blk811_dn12 * locals.var_vgd_eff) + (locals.var_t3__blk811 * locals.var_vgd_eff_dn12))) - ((((locals.var_t4__blk812_dn12 * locals.var_vgd_eff) + (locals.var_t4__blk812 * locals.var_vgd_eff_dn12)) * locals.var_vgd_eff) + (assign23460_e21194 * locals.var_vgd_eff_dn12))))),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign23460_e21200;
        locals.var_t5__blk813_dn3 = assign23460_e21200_d_n3;
        locals.var_t5__blk813_dn4 = assign23460_e21200_d_n4;
        locals.var_t5__blk813_dn5 = assign23460_e21200_d_n5;
        locals.var_t5__blk813_dn6 = assign23460_e21200_d_n6;
        locals.var_t5__blk813_dn7 = assign23460_e21200_d_n7;
        locals.var_t5__blk813_dn8 = assign23460_e21200_d_n8;
        locals.var_t5__blk813_dn9 = assign23460_e21200_d_n9;
        locals.var_t5__blk813_dn10 = assign23460_e21200_d_n10;
        locals.var_t5__blk813_dn11 = assign23460_e21200_d_n11;
        locals.var_t5__blk813_dn12 = assign23460_e21200_d_n12;

        let assign23470_e21203: f64 = if locals.var_t5__blk813 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1304 = assign23470_e21203;

        let (assign23480_e21209, assign23480_e21209_d_n3, assign23480_e21209_d_n4, assign23480_e21209_d_n5, assign23480_e21209_d_n6, assign23480_e21209_d_n7, assign23480_e21209_d_n8, assign23480_e21209_d_n9, assign23480_e21209_d_n10, assign23480_e21209_d_n11, assign23480_e21209_d_n12,) = {
    if ((p.p375 != 0.0) && (locals.var_guard1304 != 0.0)) {
        (2.688117142e43, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign23480_e21209;
        locals.var_t6__blk814_dn3 = assign23480_e21209_d_n3;
        locals.var_t6__blk814_dn4 = assign23480_e21209_d_n4;
        locals.var_t6__blk814_dn5 = assign23480_e21209_d_n5;
        locals.var_t6__blk814_dn6 = assign23480_e21209_d_n6;
        locals.var_t6__blk814_dn7 = assign23480_e21209_d_n7;
        locals.var_t6__blk814_dn8 = assign23480_e21209_d_n8;
        locals.var_t6__blk814_dn9 = assign23480_e21209_d_n9;
        locals.var_t6__blk814_dn10 = assign23480_e21209_d_n10;
        locals.var_t6__blk814_dn11 = assign23480_e21209_d_n11;
        locals.var_t6__blk814_dn12 = assign23480_e21209_d_n12;

        let assign23490_e21212: f64 = (-100.0);
        let assign23490_e21213: f64 = if locals.var_t5__blk813 < assign23490_e21212 { 1.0 } else { 0.0 };
        locals.var_guard1305 = assign23490_e21213;

        let (assign23500_e21222, assign23500_e21222_d_n3, assign23500_e21222_d_n4, assign23500_e21222_d_n5, assign23500_e21222_d_n6, assign23500_e21222_d_n7, assign23500_e21222_d_n8, assign23500_e21222_d_n9, assign23500_e21222_d_n10, assign23500_e21222_d_n11, assign23500_e21222_d_n12,) = {
    if (((p.p375 != 0.0) && (locals.var_guard1304 == 0.0)) && (locals.var_guard1305 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign23500_e21222;
        locals.var_t6__blk814_dn3 = assign23500_e21222_d_n3;
        locals.var_t6__blk814_dn4 = assign23500_e21222_d_n4;
        locals.var_t6__blk814_dn5 = assign23500_e21222_d_n5;
        locals.var_t6__blk814_dn6 = assign23500_e21222_d_n6;
        locals.var_t6__blk814_dn7 = assign23500_e21222_d_n7;
        locals.var_t6__blk814_dn8 = assign23500_e21222_d_n8;
        locals.var_t6__blk814_dn9 = assign23500_e21222_d_n9;
        locals.var_t6__blk814_dn10 = assign23500_e21222_d_n10;
        locals.var_t6__blk814_dn11 = assign23500_e21222_d_n11;
        locals.var_t6__blk814_dn12 = assign23500_e21222_d_n12;

        let (assign23510_e21233, assign23510_e21233_d_n3, assign23510_e21233_d_n4, assign23510_e21233_d_n5, assign23510_e21233_d_n6, assign23510_e21233_d_n7, assign23510_e21233_d_n8, assign23510_e21233_d_n9, assign23510_e21233_d_n10, assign23510_e21233_d_n11, assign23510_e21233_d_n12,) = {
    if (((p.p375 != 0.0) && (locals.var_guard1304 == 0.0)) && (locals.var_guard1305 == 0.0)) {
        let assign23510_e21231: f64 = (locals.var_t5__blk813).exp();
        (assign23510_e21231, (assign23510_e21231 * locals.var_t5__blk813_dn3), (assign23510_e21231 * locals.var_t5__blk813_dn4), (assign23510_e21231 * locals.var_t5__blk813_dn5), (assign23510_e21231 * locals.var_t5__blk813_dn6), (assign23510_e21231 * locals.var_t5__blk813_dn7), (assign23510_e21231 * locals.var_t5__blk813_dn8), (assign23510_e21231 * locals.var_t5__blk813_dn9), (assign23510_e21231 * locals.var_t5__blk813_dn10), (assign23510_e21231 * locals.var_t5__blk813_dn11), (assign23510_e21231 * locals.var_t5__blk813_dn12),)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign23510_e21233;
        locals.var_t6__blk814_dn3 = assign23510_e21233_d_n3;
        locals.var_t6__blk814_dn4 = assign23510_e21233_d_n4;
        locals.var_t6__blk814_dn5 = assign23510_e21233_d_n5;
        locals.var_t6__blk814_dn6 = assign23510_e21233_d_n6;
        locals.var_t6__blk814_dn7 = assign23510_e21233_d_n7;
        locals.var_t6__blk814_dn8 = assign23510_e21233_d_n8;
        locals.var_t6__blk814_dn9 = assign23510_e21233_d_n9;
        locals.var_t6__blk814_dn10 = assign23510_e21233_d_n10;
        locals.var_t6__blk814_dn11 = assign23510_e21233_d_n11;
        locals.var_t6__blk814_dn12 = assign23510_e21233_d_n12;

        let (assign23520_e21243, assign23520_e21243_d_n3, assign23520_e21243_d_n4, assign23520_e21243_d_n5, assign23520_e21243_d_n6, assign23520_e21243_d_n7, assign23520_e21243_d_n8, assign23520_e21243_d_n9, assign23520_e21243_d_n10, assign23520_e21243_d_n11, assign23520_e21243_d_n12,) = {
    if (p.p375 != 0.0) {
        let assign23520_e21237: f64 = (locals.var_t14 * locals.var_t2__blk810);
        let assign23520_e21239: f64 = (assign23520_e21237 * locals.var_t6__blk814);
        let assign23520_e21241: f64 = (assign23520_e21239 * locals.var_igtemp);
        (assign23520_e21241, ((((((locals.var_t14_dn3 * locals.var_t2__blk810) + (locals.var_t14 * locals.var_t2__blk810_dn3)) * locals.var_t6__blk814) + (assign23520_e21237 * locals.var_t6__blk814_dn3)) * locals.var_igtemp) + (assign23520_e21239 * locals.var_igtemp_dn3)), ((((((locals.var_t14_dn4 * locals.var_t2__blk810) + (locals.var_t14 * locals.var_t2__blk810_dn4)) * locals.var_t6__blk814) + (assign23520_e21237 * locals.var_t6__blk814_dn4)) * locals.var_igtemp) + (assign23520_e21239 * locals.var_igtemp_dn4)), ((((((locals.var_t14_dn5 * locals.var_t2__blk810) + (locals.var_t14 * locals.var_t2__blk810_dn5)) * locals.var_t6__blk814) + (assign23520_e21237 * locals.var_t6__blk814_dn5)) * locals.var_igtemp) + (assign23520_e21239 * locals.var_igtemp_dn5)), ((((((locals.var_t14_dn6 * locals.var_t2__blk810) + (locals.var_t14 * locals.var_t2__blk810_dn6)) * locals.var_t6__blk814) + (assign23520_e21237 * locals.var_t6__blk814_dn6)) * locals.var_igtemp) + (assign23520_e21239 * locals.var_igtemp_dn6)), ((((((locals.var_t14_dn7 * locals.var_t2__blk810) + (locals.var_t14 * locals.var_t2__blk810_dn7)) * locals.var_t6__blk814) + (assign23520_e21237 * locals.var_t6__blk814_dn7)) * locals.var_igtemp) + (assign23520_e21239 * locals.var_igtemp_dn7)), ((((((locals.var_t14_dn8 * locals.var_t2__blk810) + (locals.var_t14 * locals.var_t2__blk810_dn8)) * locals.var_t6__blk814) + (assign23520_e21237 * locals.var_t6__blk814_dn8)) * locals.var_igtemp) + (assign23520_e21239 * locals.var_igtemp_dn8)), ((((((locals.var_t14_dn9 * locals.var_t2__blk810) + (locals.var_t14 * locals.var_t2__blk810_dn9)) * locals.var_t6__blk814) + (assign23520_e21237 * locals.var_t6__blk814_dn9)) * locals.var_igtemp) + (assign23520_e21239 * locals.var_igtemp_dn9)), ((((((locals.var_t14_dn10 * locals.var_t2__blk810) + (locals.var_t14 * locals.var_t2__blk810_dn10)) * locals.var_t6__blk814) + (assign23520_e21237 * locals.var_t6__blk814_dn10)) * locals.var_igtemp) + (assign23520_e21239 * locals.var_igtemp_dn10)), ((((((locals.var_t14_dn11 * locals.var_t2__blk810) + (locals.var_t14 * locals.var_t2__blk810_dn11)) * locals.var_t6__blk814) + (assign23520_e21237 * locals.var_t6__blk814_dn11)) * locals.var_igtemp) + (assign23520_e21239 * locals.var_igtemp_dn11)), ((((((locals.var_t14_dn12 * locals.var_t2__blk810) + (locals.var_t14 * locals.var_t2__blk810_dn12)) * locals.var_t6__blk814) + (assign23520_e21237 * locals.var_t6__blk814_dn12)) * locals.var_igtemp) + (assign23520_e21239 * locals.var_igtemp_dn12)),)
    } else {
        (locals.var_igd_1, locals.var_igd_1_dn3, locals.var_igd_1_dn4, locals.var_igd_1_dn5, locals.var_igd_1_dn6, locals.var_igd_1_dn7, locals.var_igd_1_dn8, locals.var_igd_1_dn9, locals.var_igd_1_dn10, locals.var_igd_1_dn11, locals.var_igd_1_dn12,)
    }
};
        locals.var_igd_1 = assign23520_e21243;
        locals.var_igd_1_dn3 = assign23520_e21243_d_n3;
        locals.var_igd_1_dn4 = assign23520_e21243_d_n4;
        locals.var_igd_1_dn5 = assign23520_e21243_d_n5;
        locals.var_igd_1_dn6 = assign23520_e21243_d_n6;
        locals.var_igd_1_dn7 = assign23520_e21243_d_n7;
        locals.var_igd_1_dn8 = assign23520_e21243_d_n8;
        locals.var_igd_1_dn9 = assign23520_e21243_d_n9;
        locals.var_igd_1_dn10 = assign23520_e21243_d_n10;
        locals.var_igd_1_dn11 = assign23520_e21243_d_n11;
        locals.var_igd_1_dn12 = assign23520_e21243_d_n12;

        let (assign23530_e21248, assign23530_e21248_d_n3, assign23530_e21248_d_n4, assign23530_e21248_d_n5, assign23530_e21248_d_n6, assign23530_e21248_d_n7, assign23530_e21248_d_n8, assign23530_e21248_d_n9, assign23530_e21248_d_n10, assign23530_e21248_d_n11, assign23530_e21248_d_n12,) = {
    if (p.p375 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igd_1, locals.var_igd_1_dn3, locals.var_igd_1_dn4, locals.var_igd_1_dn5, locals.var_igd_1_dn6, locals.var_igd_1_dn7, locals.var_igd_1_dn8, locals.var_igd_1_dn9, locals.var_igd_1_dn10, locals.var_igd_1_dn11, locals.var_igd_1_dn12,)
    }
};
        locals.var_igd_1 = assign23530_e21248;
        locals.var_igd_1_dn3 = assign23530_e21248_d_n3;
        locals.var_igd_1_dn4 = assign23530_e21248_d_n4;
        locals.var_igd_1_dn5 = assign23530_e21248_d_n5;
        locals.var_igd_1_dn6 = assign23530_e21248_d_n6;
        locals.var_igd_1_dn7 = assign23530_e21248_d_n7;
        locals.var_igd_1_dn8 = assign23530_e21248_d_n8;
        locals.var_igd_1_dn9 = assign23530_e21248_d_n9;
        locals.var_igd_1_dn10 = assign23530_e21248_d_n10;
        locals.var_igd_1_dn11 = assign23530_e21248_d_n11;
        locals.var_igd_1_dn12 = assign23530_e21248_d_n12;

        let (assign23540_e21253, assign23540_e21253_d_n3, assign23540_e21253_d_n4, assign23540_e21253_d_n5, assign23540_e21253_d_n6, assign23540_e21253_d_n7, assign23540_e21253_d_n8, assign23540_e21253_d_n9, assign23540_e21253_d_n10, assign23540_e21253_d_n11, assign23540_e21253_d_n12,) = {
    if (p.p375 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igs_1, locals.var_igs_1_dn3, locals.var_igs_1_dn4, locals.var_igs_1_dn5, locals.var_igs_1_dn6, locals.var_igs_1_dn7, locals.var_igs_1_dn8, locals.var_igs_1_dn9, locals.var_igs_1_dn10, locals.var_igs_1_dn11, locals.var_igs_1_dn12,)
    }
};
        locals.var_igs_1 = assign23540_e21253;
        locals.var_igs_1_dn3 = assign23540_e21253_d_n3;
        locals.var_igs_1_dn4 = assign23540_e21253_d_n4;
        locals.var_igs_1_dn5 = assign23540_e21253_d_n5;
        locals.var_igs_1_dn6 = assign23540_e21253_d_n6;
        locals.var_igs_1_dn7 = assign23540_e21253_d_n7;
        locals.var_igs_1_dn8 = assign23540_e21253_d_n8;
        locals.var_igs_1_dn9 = assign23540_e21253_d_n9;
        locals.var_igs_1_dn10 = assign23540_e21253_d_n10;
        locals.var_igs_1_dn11 = assign23540_e21253_d_n11;
        locals.var_igs_1_dn12 = assign23540_e21253_d_n12;

        let (assign23550_e21258, assign23550_e21258_d_n3, assign23550_e21258_d_n4, assign23550_e21258_d_n5, assign23550_e21258_d_n6, assign23550_e21258_d_n7, assign23550_e21258_d_n8, assign23550_e21258_d_n9, assign23550_e21258_d_n10, assign23550_e21258_d_n11, assign23550_e21258_d_n12,) = {
    if (p.p375 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igcd_1, locals.var_igcd_1_dn3, locals.var_igcd_1_dn4, locals.var_igcd_1_dn5, locals.var_igcd_1_dn6, locals.var_igcd_1_dn7, locals.var_igcd_1_dn8, locals.var_igcd_1_dn9, locals.var_igcd_1_dn10, locals.var_igcd_1_dn11, locals.var_igcd_1_dn12,)
    }
};
        locals.var_igcd_1 = assign23550_e21258;
        locals.var_igcd_1_dn3 = assign23550_e21258_d_n3;
        locals.var_igcd_1_dn4 = assign23550_e21258_d_n4;
        locals.var_igcd_1_dn5 = assign23550_e21258_d_n5;
        locals.var_igcd_1_dn6 = assign23550_e21258_d_n6;
        locals.var_igcd_1_dn7 = assign23550_e21258_d_n7;
        locals.var_igcd_1_dn8 = assign23550_e21258_d_n8;
        locals.var_igcd_1_dn9 = assign23550_e21258_d_n9;
        locals.var_igcd_1_dn10 = assign23550_e21258_d_n10;
        locals.var_igcd_1_dn11 = assign23550_e21258_d_n11;
        locals.var_igcd_1_dn12 = assign23550_e21258_d_n12;

        let (assign23560_e21263, assign23560_e21263_d_n3, assign23560_e21263_d_n4, assign23560_e21263_d_n5, assign23560_e21263_d_n6, assign23560_e21263_d_n7, assign23560_e21263_d_n8, assign23560_e21263_d_n9, assign23560_e21263_d_n10, assign23560_e21263_d_n11, assign23560_e21263_d_n12,) = {
    if (p.p375 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igcs_1, locals.var_igcs_1_dn3, locals.var_igcs_1_dn4, locals.var_igcs_1_dn5, locals.var_igcs_1_dn6, locals.var_igcs_1_dn7, locals.var_igcs_1_dn8, locals.var_igcs_1_dn9, locals.var_igcs_1_dn10, locals.var_igcs_1_dn11, locals.var_igcs_1_dn12,)
    }
};
        locals.var_igcs_1 = assign23560_e21263;
        locals.var_igcs_1_dn3 = assign23560_e21263_d_n3;
        locals.var_igcs_1_dn4 = assign23560_e21263_d_n4;
        locals.var_igcs_1_dn5 = assign23560_e21263_d_n5;
        locals.var_igcs_1_dn6 = assign23560_e21263_d_n6;
        locals.var_igcs_1_dn7 = assign23560_e21263_d_n7;
        locals.var_igcs_1_dn8 = assign23560_e21263_d_n8;
        locals.var_igcs_1_dn9 = assign23560_e21263_d_n9;
        locals.var_igcs_1_dn10 = assign23560_e21263_d_n10;
        locals.var_igcs_1_dn11 = assign23560_e21263_d_n11;
        locals.var_igcs_1_dn12 = assign23560_e21263_d_n12;

        let assign23570_e21270: f64 = if ((p.p374 != 0.0) && (locals.var_b4soisoimod != 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard1306 = assign23570_e21270;

        let (assign23580_e21274,) = {
    if (locals.var_guard1306 != 0.0) {
        (locals.var_pparam_b4soioxideratio,)
    } else {
        (locals.var_oxideratio,)
    }
};
        locals.var_oxideratio = assign23580_e21274;

        let (assign23590_e21278, assign23590_e21278_d_n3, assign23590_e21278_d_n4, assign23590_e21278_d_n5, assign23590_e21278_d_n6, assign23590_e21278_d_n7, assign23590_e21278_d_n8, assign23590_e21278_d_n9, assign23590_e21278_d_n10, assign23590_e21278_d_n11, assign23590_e21278_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        (locals.var_voxdepinv, locals.var_voxdepinv_dn3, locals.var_voxdepinv_dn4, locals.var_voxdepinv_dn5, locals.var_voxdepinv_dn6, locals.var_voxdepinv_dn7, locals.var_voxdepinv_dn8, locals.var_voxdepinv_dn9, locals.var_voxdepinv_dn10, locals.var_voxdepinv_dn11, locals.var_voxdepinv_dn12,)
    } else {
        (locals.var_vox, locals.var_vox_dn3, locals.var_vox_dn4, locals.var_vox_dn5, locals.var_vox_dn6, locals.var_vox_dn7, locals.var_vox_dn8, locals.var_vox_dn9, locals.var_vox_dn10, locals.var_vox_dn11, locals.var_vox_dn12,)
    }
};
        locals.var_vox = assign23590_e21278;
        locals.var_vox_dn3 = assign23590_e21278_d_n3;
        locals.var_vox_dn4 = assign23590_e21278_d_n4;
        locals.var_vox_dn5 = assign23590_e21278_d_n5;
        locals.var_vox_dn6 = assign23590_e21278_d_n6;
        locals.var_vox_dn7 = assign23590_e21278_d_n7;
        locals.var_vox_dn8 = assign23590_e21278_d_n8;
        locals.var_vox_dn9 = assign23590_e21278_d_n9;
        locals.var_vox_dn10 = assign23590_e21278_d_n10;
        locals.var_vox_dn11 = assign23590_e21278_d_n11;
        locals.var_vox_dn12 = assign23590_e21278_d_n12;

        let (assign23600_e21282, assign23600_e21282_d_n3, assign23600_e21282_d_n4, assign23600_e21282_d_n5, assign23600_e21282_d_n6, assign23600_e21282_d_n7, assign23600_e21282_d_n8, assign23600_e21282_d_n9, assign23600_e21282_d_n10, assign23600_e21282_d_n11, assign23600_e21282_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        (p.p396, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign23600_e21282;
        locals.var_t0__blk808_dn3 = assign23600_e21282_d_n3;
        locals.var_t0__blk808_dn4 = assign23600_e21282_d_n4;
        locals.var_t0__blk808_dn5 = assign23600_e21282_d_n5;
        locals.var_t0__blk808_dn6 = assign23600_e21282_d_n6;
        locals.var_t0__blk808_dn7 = assign23600_e21282_d_n7;
        locals.var_t0__blk808_dn8 = assign23600_e21282_d_n8;
        locals.var_t0__blk808_dn9 = assign23600_e21282_d_n9;
        locals.var_t0__blk808_dn10 = assign23600_e21282_d_n10;
        locals.var_t0__blk808_dn11 = assign23600_e21282_d_n11;
        locals.var_t0__blk808_dn12 = assign23600_e21282_d_n12;

        let (assign23610_e21290, assign23610_e21290_d_n3, assign23610_e21290_d_n4, assign23610_e21290_d_n5, assign23610_e21290_d_n6, assign23610_e21290_d_n7, assign23610_e21290_d_n8, assign23610_e21290_d_n9, assign23610_e21290_d_n10, assign23610_e21290_d_n11, assign23610_e21290_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        let assign23610_e21286: f64 = (locals.var_t0__blk808 - locals.var_vox);
        let assign23610_e21288: f64 = (assign23610_e21286 - p.p397);
        (assign23610_e21288, (locals.var_t0__blk808_dn3 - locals.var_vox_dn3), (locals.var_t0__blk808_dn4 - locals.var_vox_dn4), (locals.var_t0__blk808_dn5 - locals.var_vox_dn5), (locals.var_t0__blk808_dn6 - locals.var_vox_dn6), (locals.var_t0__blk808_dn7 - locals.var_vox_dn7), (locals.var_t0__blk808_dn8 - locals.var_vox_dn8), (locals.var_t0__blk808_dn9 - locals.var_vox_dn9), (locals.var_t0__blk808_dn10 - locals.var_vox_dn10), (locals.var_t0__blk808_dn11 - locals.var_vox_dn11), (locals.var_t0__blk808_dn12 - locals.var_vox_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign23610_e21290;
        locals.var_t1__blk809_dn3 = assign23610_e21290_d_n3;
        locals.var_t1__blk809_dn4 = assign23610_e21290_d_n4;
        locals.var_t1__blk809_dn5 = assign23610_e21290_d_n5;
        locals.var_t1__blk809_dn6 = assign23610_e21290_d_n6;
        locals.var_t1__blk809_dn7 = assign23610_e21290_d_n7;
        locals.var_t1__blk809_dn8 = assign23610_e21290_d_n8;
        locals.var_t1__blk809_dn9 = assign23610_e21290_d_n9;
        locals.var_t1__blk809_dn10 = assign23610_e21290_d_n10;
        locals.var_t1__blk809_dn11 = assign23610_e21290_d_n11;
        locals.var_t1__blk809_dn12 = assign23610_e21290_d_n12;

        let (assign23620_e21303, assign23620_e21303_d_n3, assign23620_e21303_d_n4, assign23620_e21303_d_n5, assign23620_e21303_d_n6, assign23620_e21303_d_n7, assign23620_e21303_d_n8, assign23620_e21303_d_n9, assign23620_e21303_d_n10, assign23620_e21303_d_n11, assign23620_e21303_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        let assign23620_e21294: f64 = (locals.var_t1__blk809 * locals.var_t1__blk809);
        let assign23620_e21297: f64 = (4.0 * p.p397);
        let assign23620_e21299: f64 = (assign23620_e21297 * locals.var_t0__blk808);
        let assign23620_e21300: f64 = (assign23620_e21294 + assign23620_e21299);
        let assign23620_e21301: f64 = (assign23620_e21300).sqrt();
        (assign23620_e21301, ((((locals.var_t1__blk809_dn3 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn3)) + (assign23620_e21297 * locals.var_t0__blk808_dn3)) / (2.0 * assign23620_e21301)), ((((locals.var_t1__blk809_dn4 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn4)) + (assign23620_e21297 * locals.var_t0__blk808_dn4)) / (2.0 * assign23620_e21301)), ((((locals.var_t1__blk809_dn5 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn5)) + (assign23620_e21297 * locals.var_t0__blk808_dn5)) / (2.0 * assign23620_e21301)), ((((locals.var_t1__blk809_dn6 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn6)) + (assign23620_e21297 * locals.var_t0__blk808_dn6)) / (2.0 * assign23620_e21301)), ((((locals.var_t1__blk809_dn7 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn7)) + (assign23620_e21297 * locals.var_t0__blk808_dn7)) / (2.0 * assign23620_e21301)), ((((locals.var_t1__blk809_dn8 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn8)) + (assign23620_e21297 * locals.var_t0__blk808_dn8)) / (2.0 * assign23620_e21301)), ((((locals.var_t1__blk809_dn9 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn9)) + (assign23620_e21297 * locals.var_t0__blk808_dn9)) / (2.0 * assign23620_e21301)), ((((locals.var_t1__blk809_dn10 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn10)) + (assign23620_e21297 * locals.var_t0__blk808_dn10)) / (2.0 * assign23620_e21301)), ((((locals.var_t1__blk809_dn11 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn11)) + (assign23620_e21297 * locals.var_t0__blk808_dn11)) / (2.0 * assign23620_e21301)), ((((locals.var_t1__blk809_dn12 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn12)) + (assign23620_e21297 * locals.var_t0__blk808_dn12)) / (2.0 * assign23620_e21301)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign23620_e21303;
        locals.var_t3__blk811_dn3 = assign23620_e21303_d_n3;
        locals.var_t3__blk811_dn4 = assign23620_e21303_d_n4;
        locals.var_t3__blk811_dn5 = assign23620_e21303_d_n5;
        locals.var_t3__blk811_dn6 = assign23620_e21303_d_n6;
        locals.var_t3__blk811_dn7 = assign23620_e21303_d_n7;
        locals.var_t3__blk811_dn8 = assign23620_e21303_d_n8;
        locals.var_t3__blk811_dn9 = assign23620_e21303_d_n9;
        locals.var_t3__blk811_dn10 = assign23620_e21303_d_n10;
        locals.var_t3__blk811_dn11 = assign23620_e21303_d_n11;
        locals.var_t3__blk811_dn12 = assign23620_e21303_d_n12;

        let (assign23630_e21313, assign23630_e21313_d_n3, assign23630_e21313_d_n4, assign23630_e21313_d_n5, assign23630_e21313_d_n6, assign23630_e21313_d_n7, assign23630_e21313_d_n8, assign23630_e21313_d_n9, assign23630_e21313_d_n10, assign23630_e21313_d_n11, assign23630_e21313_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        let assign23630_e21309: f64 = (locals.var_t1__blk809 + locals.var_t3__blk811);
        let assign23630_e21310: f64 = (0.5 * assign23630_e21309);
        let assign23630_e21311: f64 = (locals.var_t0__blk808 - assign23630_e21310);
        (assign23630_e21311, (locals.var_t0__blk808_dn3 - (0.5 * (locals.var_t1__blk809_dn3 + locals.var_t3__blk811_dn3))), (locals.var_t0__blk808_dn4 - (0.5 * (locals.var_t1__blk809_dn4 + locals.var_t3__blk811_dn4))), (locals.var_t0__blk808_dn5 - (0.5 * (locals.var_t1__blk809_dn5 + locals.var_t3__blk811_dn5))), (locals.var_t0__blk808_dn6 - (0.5 * (locals.var_t1__blk809_dn6 + locals.var_t3__blk811_dn6))), (locals.var_t0__blk808_dn7 - (0.5 * (locals.var_t1__blk809_dn7 + locals.var_t3__blk811_dn7))), (locals.var_t0__blk808_dn8 - (0.5 * (locals.var_t1__blk809_dn8 + locals.var_t3__blk811_dn8))), (locals.var_t0__blk808_dn9 - (0.5 * (locals.var_t1__blk809_dn9 + locals.var_t3__blk811_dn9))), (locals.var_t0__blk808_dn10 - (0.5 * (locals.var_t1__blk809_dn10 + locals.var_t3__blk811_dn10))), (locals.var_t0__blk808_dn11 - (0.5 * (locals.var_t1__blk809_dn11 + locals.var_t3__blk811_dn11))), (locals.var_t0__blk808_dn12 - (0.5 * (locals.var_t1__blk809_dn12 + locals.var_t3__blk811_dn12))),)
    } else {
        (locals.var_voxeff, locals.var_voxeff_dn3, locals.var_voxeff_dn4, locals.var_voxeff_dn5, locals.var_voxeff_dn6, locals.var_voxeff_dn7, locals.var_voxeff_dn8, locals.var_voxeff_dn9, locals.var_voxeff_dn10, locals.var_voxeff_dn11, locals.var_voxeff_dn12,)
    }
};
        locals.var_voxeff = assign23630_e21313;
        locals.var_voxeff_dn3 = assign23630_e21313_d_n3;
        locals.var_voxeff_dn4 = assign23630_e21313_d_n4;
        locals.var_voxeff_dn5 = assign23630_e21313_d_n5;
        locals.var_voxeff_dn6 = assign23630_e21313_d_n6;
        locals.var_voxeff_dn7 = assign23630_e21313_d_n7;
        locals.var_voxeff_dn8 = assign23630_e21313_d_n8;
        locals.var_voxeff_dn9 = assign23630_e21313_d_n9;
        locals.var_voxeff_dn10 = assign23630_e21313_d_n10;
        locals.var_voxeff_dn11 = assign23630_e21313_d_n11;
        locals.var_voxeff_dn12 = assign23630_e21313_d_n12;

    }

    pub(super) fn stamp_transient_block_70(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23640_e21317, assign23640_e21317_d_n3, assign23640_e21317_d_n4, assign23640_e21317_d_n5, assign23640_e21317_d_n6, assign23640_e21317_d_n7, assign23640_e21317_d_n8, assign23640_e21317_d_n9, assign23640_e21317_d_n10, assign23640_e21317_d_n11, assign23640_e21317_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        (locals.var_voxeff, locals.var_voxeff_dn3, locals.var_voxeff_dn4, locals.var_voxeff_dn5, locals.var_voxeff_dn6, locals.var_voxeff_dn7, locals.var_voxeff_dn8, locals.var_voxeff_dn9, locals.var_voxeff_dn10, locals.var_voxeff_dn11, locals.var_voxeff_dn12,)
    } else {
        (locals.var_vox, locals.var_vox_dn3, locals.var_vox_dn4, locals.var_vox_dn5, locals.var_vox_dn6, locals.var_vox_dn7, locals.var_vox_dn8, locals.var_vox_dn9, locals.var_vox_dn10, locals.var_vox_dn11, locals.var_vox_dn12,)
    }
};
        locals.var_vox = assign23640_e21317;
        locals.var_vox_dn3 = assign23640_e21317_d_n3;
        locals.var_vox_dn4 = assign23640_e21317_d_n4;
        locals.var_vox_dn5 = assign23640_e21317_d_n5;
        locals.var_vox_dn6 = assign23640_e21317_d_n6;
        locals.var_vox_dn7 = assign23640_e21317_d_n7;
        locals.var_vox_dn8 = assign23640_e21317_d_n8;
        locals.var_vox_dn9 = assign23640_e21317_d_n9;
        locals.var_vox_dn10 = assign23640_e21317_d_n10;
        locals.var_vox_dn11 = assign23640_e21317_d_n11;
        locals.var_vox_dn12 = assign23640_e21317_d_n12;

        let (assign23650_e21325, assign23650_e21325_d_n3, assign23650_e21325_d_n4, assign23650_e21325_d_n5, assign23650_e21325_d_n6, assign23650_e21325_d_n7, assign23650_e21325_d_n8, assign23650_e21325_d_n9, assign23650_e21325_d_n10, assign23650_e21325_d_n11, assign23650_e21325_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        let assign23650_e21321: f64 = (locals.var_vox - p.p381);
        let assign23650_e21323: f64 = (assign23650_e21321 / p.p382);
        (assign23650_e21323, (locals.var_vox_dn3 / p.p382), (locals.var_vox_dn4 / p.p382), (locals.var_vox_dn5 / p.p382), (locals.var_vox_dn6 / p.p382), (locals.var_vox_dn7 / p.p382), (locals.var_vox_dn8 / p.p382), (locals.var_vox_dn9 / p.p382), (locals.var_vox_dn10 / p.p382), (locals.var_vox_dn11 / p.p382), (locals.var_vox_dn12 / p.p382),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign23650_e21325;
        locals.var_t0__blk808_dn3 = assign23650_e21325_d_n3;
        locals.var_t0__blk808_dn4 = assign23650_e21325_d_n4;
        locals.var_t0__blk808_dn5 = assign23650_e21325_d_n5;
        locals.var_t0__blk808_dn6 = assign23650_e21325_d_n6;
        locals.var_t0__blk808_dn7 = assign23650_e21325_d_n7;
        locals.var_t0__blk808_dn8 = assign23650_e21325_d_n8;
        locals.var_t0__blk808_dn9 = assign23650_e21325_d_n9;
        locals.var_t0__blk808_dn10 = assign23650_e21325_d_n10;
        locals.var_t0__blk808_dn11 = assign23650_e21325_d_n11;
        locals.var_t0__blk808_dn12 = assign23650_e21325_d_n12;

        let assign23660_e21328: f64 = if locals.var_t0__blk808 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1307 = assign23660_e21328;

        let (assign23670_e21340, assign23670_e21340_d_n3, assign23670_e21340_d_n4, assign23670_e21340_d_n5, assign23670_e21340_d_n6, assign23670_e21340_d_n7, assign23670_e21340_d_n8, assign23670_e21340_d_n9, assign23670_e21340_d_n10, assign23670_e21340_d_n11, assign23670_e21340_d_n12,) = {
    if ((locals.var_guard1306 != 0.0) && (locals.var_guard1307 != 0.0)) {
        let assign23670_e21335: f64 = (1.0 + locals.var_t0__blk808);
        let assign23670_e21337: f64 = (assign23670_e21335 - 100.0);
        let assign23670_e21338: f64 = (2.688117142e43 * assign23670_e21337);
        (assign23670_e21338, (2.688117142e43 * locals.var_t0__blk808_dn3), (2.688117142e43 * locals.var_t0__blk808_dn4), (2.688117142e43 * locals.var_t0__blk808_dn5), (2.688117142e43 * locals.var_t0__blk808_dn6), (2.688117142e43 * locals.var_t0__blk808_dn7), (2.688117142e43 * locals.var_t0__blk808_dn8), (2.688117142e43 * locals.var_t0__blk808_dn9), (2.688117142e43 * locals.var_t0__blk808_dn10), (2.688117142e43 * locals.var_t0__blk808_dn11), (2.688117142e43 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign23670_e21340;
        locals.var_t1__blk809_dn3 = assign23670_e21340_d_n3;
        locals.var_t1__blk809_dn4 = assign23670_e21340_d_n4;
        locals.var_t1__blk809_dn5 = assign23670_e21340_d_n5;
        locals.var_t1__blk809_dn6 = assign23670_e21340_d_n6;
        locals.var_t1__blk809_dn7 = assign23670_e21340_d_n7;
        locals.var_t1__blk809_dn8 = assign23670_e21340_d_n8;
        locals.var_t1__blk809_dn9 = assign23670_e21340_d_n9;
        locals.var_t1__blk809_dn10 = assign23670_e21340_d_n10;
        locals.var_t1__blk809_dn11 = assign23670_e21340_d_n11;
        locals.var_t1__blk809_dn12 = assign23670_e21340_d_n12;

        let assign23680_e21343: f64 = (-100.0);
        let assign23680_e21344: f64 = if locals.var_t0__blk808 < assign23680_e21343 { 1.0 } else { 0.0 };
        locals.var_guard1308 = assign23680_e21344;

        let (assign23690_e21353, assign23690_e21353_d_n3, assign23690_e21353_d_n4, assign23690_e21353_d_n5, assign23690_e21353_d_n6, assign23690_e21353_d_n7, assign23690_e21353_d_n8, assign23690_e21353_d_n9, assign23690_e21353_d_n10, assign23690_e21353_d_n11, assign23690_e21353_d_n12,) = {
    if (((locals.var_guard1306 != 0.0) && (locals.var_guard1307 == 0.0)) && (locals.var_guard1308 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign23690_e21353;
        locals.var_t1__blk809_dn3 = assign23690_e21353_d_n3;
        locals.var_t1__blk809_dn4 = assign23690_e21353_d_n4;
        locals.var_t1__blk809_dn5 = assign23690_e21353_d_n5;
        locals.var_t1__blk809_dn6 = assign23690_e21353_d_n6;
        locals.var_t1__blk809_dn7 = assign23690_e21353_d_n7;
        locals.var_t1__blk809_dn8 = assign23690_e21353_d_n8;
        locals.var_t1__blk809_dn9 = assign23690_e21353_d_n9;
        locals.var_t1__blk809_dn10 = assign23690_e21353_d_n10;
        locals.var_t1__blk809_dn11 = assign23690_e21353_d_n11;
        locals.var_t1__blk809_dn12 = assign23690_e21353_d_n12;

        let (assign23700_e21364, assign23700_e21364_d_n3, assign23700_e21364_d_n4, assign23700_e21364_d_n5, assign23700_e21364_d_n6, assign23700_e21364_d_n7, assign23700_e21364_d_n8, assign23700_e21364_d_n9, assign23700_e21364_d_n10, assign23700_e21364_d_n11, assign23700_e21364_d_n12,) = {
    if (((locals.var_guard1306 != 0.0) && (locals.var_guard1307 == 0.0)) && (locals.var_guard1308 == 0.0)) {
        let assign23700_e21362: f64 = (locals.var_t0__blk808).exp();
        (assign23700_e21362, (assign23700_e21362 * locals.var_t0__blk808_dn3), (assign23700_e21362 * locals.var_t0__blk808_dn4), (assign23700_e21362 * locals.var_t0__blk808_dn5), (assign23700_e21362 * locals.var_t0__blk808_dn6), (assign23700_e21362 * locals.var_t0__blk808_dn7), (assign23700_e21362 * locals.var_t0__blk808_dn8), (assign23700_e21362 * locals.var_t0__blk808_dn9), (assign23700_e21362 * locals.var_t0__blk808_dn10), (assign23700_e21362 * locals.var_t0__blk808_dn11), (assign23700_e21362 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign23700_e21364;
        locals.var_t1__blk809_dn3 = assign23700_e21364_d_n3;
        locals.var_t1__blk809_dn4 = assign23700_e21364_d_n4;
        locals.var_t1__blk809_dn5 = assign23700_e21364_d_n5;
        locals.var_t1__blk809_dn6 = assign23700_e21364_d_n6;
        locals.var_t1__blk809_dn7 = assign23700_e21364_d_n7;
        locals.var_t1__blk809_dn8 = assign23700_e21364_d_n8;
        locals.var_t1__blk809_dn9 = assign23700_e21364_d_n9;
        locals.var_t1__blk809_dn10 = assign23700_e21364_d_n10;
        locals.var_t1__blk809_dn11 = assign23700_e21364_d_n11;
        locals.var_t1__blk809_dn12 = assign23700_e21364_d_n12;

        let (assign23710_e21373, assign23710_e21373_d_n3, assign23710_e21373_d_n4, assign23710_e21373_d_n5, assign23710_e21373_d_n6, assign23710_e21373_d_n7, assign23710_e21373_d_n8, assign23710_e21373_d_n9, assign23710_e21373_d_n10, assign23710_e21373_d_n11, assign23710_e21373_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        let assign23710_e21369: f64 = (1.0 + locals.var_t1__blk809);
        let assign23710_e21370: f64 = (assign23710_e21369).ln();
        let assign23710_e21371: f64 = (p.p382 * assign23710_e21370);
        (assign23710_e21371, (p.p382 * (locals.var_t1__blk809_dn3 / assign23710_e21369)), (p.p382 * (locals.var_t1__blk809_dn4 / assign23710_e21369)), (p.p382 * (locals.var_t1__blk809_dn5 / assign23710_e21369)), (p.p382 * (locals.var_t1__blk809_dn6 / assign23710_e21369)), (p.p382 * (locals.var_t1__blk809_dn7 / assign23710_e21369)), (p.p382 * (locals.var_t1__blk809_dn8 / assign23710_e21369)), (p.p382 * (locals.var_t1__blk809_dn9 / assign23710_e21369)), (p.p382 * (locals.var_t1__blk809_dn10 / assign23710_e21369)), (p.p382 * (locals.var_t1__blk809_dn11 / assign23710_e21369)), (p.p382 * (locals.var_t1__blk809_dn12 / assign23710_e21369)),)
    } else {
        (locals.var_vaux, locals.var_vaux_dn3, locals.var_vaux_dn4, locals.var_vaux_dn5, locals.var_vaux_dn6, locals.var_vaux_dn7, locals.var_vaux_dn8, locals.var_vaux_dn9, locals.var_vaux_dn10, locals.var_vaux_dn11, locals.var_vaux_dn12,)
    }
};
        locals.var_vaux = assign23710_e21373;
        locals.var_vaux_dn3 = assign23710_e21373_d_n3;
        locals.var_vaux_dn4 = assign23710_e21373_d_n4;
        locals.var_vaux_dn5 = assign23710_e21373_d_n5;
        locals.var_vaux_dn6 = assign23710_e21373_d_n6;
        locals.var_vaux_dn7 = assign23710_e21373_d_n7;
        locals.var_vaux_dn8 = assign23710_e21373_d_n8;
        locals.var_vaux_dn9 = assign23710_e21373_d_n9;
        locals.var_vaux_dn10 = assign23710_e21373_d_n10;
        locals.var_vaux_dn11 = assign23710_e21373_d_n11;
        locals.var_vaux_dn12 = assign23710_e21373_d_n12;

        let assign23720_e21376: f64 = if p.p386 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1309 = assign23720_e21376;

        let (assign23730_e21386, assign23730_e21386_d_n3, assign23730_e21386_d_n4, assign23730_e21386_d_n5, assign23730_e21386_d_n6, assign23730_e21386_d_n7, assign23730_e21386_d_n8, assign23730_e21386_d_n9, assign23730_e21386_d_n10, assign23730_e21386_d_n11, assign23730_e21386_d_n12,) = {
    if ((locals.var_guard1306 != 0.0) && (locals.var_guard1309 != 0.0)) {
        let assign23730_e21383: f64 = (locals.var_vox / p.p386);
        let assign23730_e21384: f64 = (1.0 - assign23730_e21383);
        (assign23730_e21384, (-(locals.var_vox_dn3 / p.p386)), (-(locals.var_vox_dn4 / p.p386)), (-(locals.var_vox_dn5 / p.p386)), (-(locals.var_vox_dn6 / p.p386)), (-(locals.var_vox_dn7 / p.p386)), (-(locals.var_vox_dn8 / p.p386)), (-(locals.var_vox_dn9 / p.p386)), (-(locals.var_vox_dn10 / p.p386)), (-(locals.var_vox_dn11 / p.p386)), (-(locals.var_vox_dn12 / p.p386)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign23730_e21386;
        locals.var_t0__blk808_dn3 = assign23730_e21386_d_n3;
        locals.var_t0__blk808_dn4 = assign23730_e21386_d_n4;
        locals.var_t0__blk808_dn5 = assign23730_e21386_d_n5;
        locals.var_t0__blk808_dn6 = assign23730_e21386_d_n6;
        locals.var_t0__blk808_dn7 = assign23730_e21386_d_n7;
        locals.var_t0__blk808_dn8 = assign23730_e21386_d_n8;
        locals.var_t0__blk808_dn9 = assign23730_e21386_d_n9;
        locals.var_t0__blk808_dn10 = assign23730_e21386_d_n10;
        locals.var_t0__blk808_dn11 = assign23730_e21386_d_n11;
        locals.var_t0__blk808_dn12 = assign23730_e21386_d_n12;

        let (assign23740_e21393, assign23740_e21393_d_n3, assign23740_e21393_d_n4, assign23740_e21393_d_n5, assign23740_e21393_d_n6, assign23740_e21393_d_n7, assign23740_e21393_d_n8, assign23740_e21393_d_n9, assign23740_e21393_d_n10, assign23740_e21393_d_n11, assign23740_e21393_d_n12,) = {
    if ((locals.var_guard1306 != 0.0) && (locals.var_guard1309 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign23740_e21393;
        locals.var_t0__blk808_dn3 = assign23740_e21393_d_n3;
        locals.var_t0__blk808_dn4 = assign23740_e21393_d_n4;
        locals.var_t0__blk808_dn5 = assign23740_e21393_d_n5;
        locals.var_t0__blk808_dn6 = assign23740_e21393_d_n6;
        locals.var_t0__blk808_dn7 = assign23740_e21393_d_n7;
        locals.var_t0__blk808_dn8 = assign23740_e21393_d_n8;
        locals.var_t0__blk808_dn9 = assign23740_e21393_d_n9;
        locals.var_t0__blk808_dn10 = assign23740_e21393_d_n10;
        locals.var_t0__blk808_dn11 = assign23740_e21393_d_n11;
        locals.var_t0__blk808_dn12 = assign23740_e21393_d_n12;

        let assign23750_e21396: f64 = if locals.var_t0__blk808 < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard1310 = assign23750_e21396;

        let (assign23760_e21402, assign23760_e21402_d_n3, assign23760_e21402_d_n4, assign23760_e21402_d_n5, assign23760_e21402_d_n6, assign23760_e21402_d_n7, assign23760_e21402_d_n8, assign23760_e21402_d_n9, assign23760_e21402_d_n10, assign23760_e21402_d_n11, assign23760_e21402_d_n12,) = {
    if ((locals.var_guard1306 != 0.0) && (locals.var_guard1310 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign23760_e21402;
        locals.var_t0__blk808_dn3 = assign23760_e21402_d_n3;
        locals.var_t0__blk808_dn4 = assign23760_e21402_d_n4;
        locals.var_t0__blk808_dn5 = assign23760_e21402_d_n5;
        locals.var_t0__blk808_dn6 = assign23760_e21402_d_n6;
        locals.var_t0__blk808_dn7 = assign23760_e21402_d_n7;
        locals.var_t0__blk808_dn8 = assign23760_e21402_d_n8;
        locals.var_t0__blk808_dn9 = assign23760_e21402_d_n9;
        locals.var_t0__blk808_dn10 = assign23760_e21402_d_n10;
        locals.var_t0__blk808_dn11 = assign23760_e21402_d_n11;
        locals.var_t0__blk808_dn12 = assign23760_e21402_d_n12;

        let (assign23770_e21418, assign23770_e21418_d_n3, assign23770_e21418_d_n4, assign23770_e21418_d_n5, assign23770_e21418_d_n6, assign23770_e21418_d_n7, assign23770_e21418_d_n8, assign23770_e21418_d_n9, assign23770_e21418_d_n10, assign23770_e21418_d_n11, assign23770_e21418_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        let assign23770_e21406: f64 = (locals.var_leff * locals.var_weff);
        let assign23770_e21408: f64 = (assign23770_e21406 / p.p23);
        let assign23770_e21411: f64 = (p.p28 / p.p3);
        let assign23770_e21412: f64 = (assign23770_e21408 + assign23770_e21411);
        let assign23770_e21414: f64 = (assign23770_e21412 * p.p1035);
        let assign23770_e21416: f64 = (assign23770_e21414 * locals.var_oxideratio);
        (assign23770_e21416, (((((locals.var_leff_dn3 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn3)) / p.p23) * p.p1035) * locals.var_oxideratio), (((((locals.var_leff_dn4 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn4)) / p.p23) * p.p1035) * locals.var_oxideratio), (((((locals.var_leff_dn5 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn5)) / p.p23) * p.p1035) * locals.var_oxideratio), (((((locals.var_leff_dn6 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn6)) / p.p23) * p.p1035) * locals.var_oxideratio), (((((locals.var_leff_dn7 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn7)) / p.p23) * p.p1035) * locals.var_oxideratio), (((((locals.var_leff_dn8 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn8)) / p.p23) * p.p1035) * locals.var_oxideratio), (((((locals.var_leff_dn9 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn9)) / p.p23) * p.p1035) * locals.var_oxideratio), (((((locals.var_leff_dn10 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn10)) / p.p23) * p.p1035) * locals.var_oxideratio), (((((locals.var_leff_dn11 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn11)) / p.p23) * p.p1035) * locals.var_oxideratio), (((((locals.var_leff_dn12 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn12)) / p.p23) * p.p1035) * locals.var_oxideratio),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign23770_e21418;
        locals.var_t1__blk809_dn3 = assign23770_e21418_d_n3;
        locals.var_t1__blk809_dn4 = assign23770_e21418_d_n4;
        locals.var_t1__blk809_dn5 = assign23770_e21418_d_n5;
        locals.var_t1__blk809_dn6 = assign23770_e21418_d_n6;
        locals.var_t1__blk809_dn7 = assign23770_e21418_d_n7;
        locals.var_t1__blk809_dn8 = assign23770_e21418_d_n8;
        locals.var_t1__blk809_dn9 = assign23770_e21418_d_n9;
        locals.var_t1__blk809_dn10 = assign23770_e21418_d_n10;
        locals.var_t1__blk809_dn11 = assign23770_e21418_d_n11;
        locals.var_t1__blk809_dn12 = assign23770_e21418_d_n12;

        let (assign23780_e21424, assign23780_e21424_d_n3, assign23780_e21424_d_n4, assign23780_e21424_d_n5, assign23780_e21424_d_n6, assign23780_e21424_d_n7, assign23780_e21424_d_n8, assign23780_e21424_d_n9, assign23780_e21424_d_n10, assign23780_e21424_d_n11, assign23780_e21424_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        let assign23780_e21422: f64 = (p.p1036 * p.p376);
        (assign23780_e21422, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign23780_e21424;
        locals.var_t2__blk810_dn3 = assign23780_e21424_d_n3;
        locals.var_t2__blk810_dn4 = assign23780_e21424_d_n4;
        locals.var_t2__blk810_dn5 = assign23780_e21424_d_n5;
        locals.var_t2__blk810_dn6 = assign23780_e21424_d_n6;
        locals.var_t2__blk810_dn7 = assign23780_e21424_d_n7;
        locals.var_t2__blk810_dn8 = assign23780_e21424_d_n8;
        locals.var_t2__blk810_dn9 = assign23780_e21424_d_n9;
        locals.var_t2__blk810_dn10 = assign23780_e21424_d_n10;
        locals.var_t2__blk810_dn11 = assign23780_e21424_d_n11;
        locals.var_t2__blk810_dn12 = assign23780_e21424_d_n12;

        let (assign23790_e21428, assign23790_e21428_d_n3, assign23790_e21428_d_n4, assign23790_e21428_d_n5, assign23790_e21428_d_n6, assign23790_e21428_d_n7, assign23790_e21428_d_n8, assign23790_e21428_d_n9, assign23790_e21428_d_n10, assign23790_e21428_d_n11, assign23790_e21428_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        (locals.var_pparam_b4soialphagb1, locals.var_pparam_b4soialphagb1_dn3, locals.var_pparam_b4soialphagb1_dn4, locals.var_pparam_b4soialphagb1_dn5, locals.var_pparam_b4soialphagb1_dn6, locals.var_pparam_b4soialphagb1_dn7, locals.var_pparam_b4soialphagb1_dn8, locals.var_pparam_b4soialphagb1_dn9, locals.var_pparam_b4soialphagb1_dn10, locals.var_pparam_b4soialphagb1_dn11, locals.var_pparam_b4soialphagb1_dn12,)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign23790_e21428;
        locals.var_t3__blk811_dn3 = assign23790_e21428_d_n3;
        locals.var_t3__blk811_dn4 = assign23790_e21428_d_n4;
        locals.var_t3__blk811_dn5 = assign23790_e21428_d_n5;
        locals.var_t3__blk811_dn6 = assign23790_e21428_d_n6;
        locals.var_t3__blk811_dn7 = assign23790_e21428_d_n7;
        locals.var_t3__blk811_dn8 = assign23790_e21428_d_n8;
        locals.var_t3__blk811_dn9 = assign23790_e21428_d_n9;
        locals.var_t3__blk811_dn10 = assign23790_e21428_d_n10;
        locals.var_t3__blk811_dn11 = assign23790_e21428_d_n11;
        locals.var_t3__blk811_dn12 = assign23790_e21428_d_n12;

        let (assign23800_e21432, assign23800_e21432_d_n3, assign23800_e21432_d_n4, assign23800_e21432_d_n5, assign23800_e21432_d_n6, assign23800_e21432_d_n7, assign23800_e21432_d_n8, assign23800_e21432_d_n9, assign23800_e21432_d_n10, assign23800_e21432_d_n11, assign23800_e21432_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        (locals.var_pparam_b4soibetagb1, locals.var_pparam_b4soibetagb1_dn3, locals.var_pparam_b4soibetagb1_dn4, locals.var_pparam_b4soibetagb1_dn5, locals.var_pparam_b4soibetagb1_dn6, locals.var_pparam_b4soibetagb1_dn7, locals.var_pparam_b4soibetagb1_dn8, locals.var_pparam_b4soibetagb1_dn9, locals.var_pparam_b4soibetagb1_dn10, locals.var_pparam_b4soibetagb1_dn11, locals.var_pparam_b4soibetagb1_dn12,)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign23800_e21432;
        locals.var_t4__blk812_dn3 = assign23800_e21432_d_n3;
        locals.var_t4__blk812_dn4 = assign23800_e21432_d_n4;
        locals.var_t4__blk812_dn5 = assign23800_e21432_d_n5;
        locals.var_t4__blk812_dn6 = assign23800_e21432_d_n6;
        locals.var_t4__blk812_dn7 = assign23800_e21432_d_n7;
        locals.var_t4__blk812_dn8 = assign23800_e21432_d_n8;
        locals.var_t4__blk812_dn9 = assign23800_e21432_d_n9;
        locals.var_t4__blk812_dn10 = assign23800_e21432_d_n10;
        locals.var_t4__blk812_dn11 = assign23800_e21432_d_n11;
        locals.var_t4__blk812_dn12 = assign23800_e21432_d_n12;

        let (assign23810_e21444, assign23810_e21444_d_n3, assign23810_e21444_d_n4, assign23810_e21444_d_n5, assign23810_e21444_d_n6, assign23810_e21444_d_n7, assign23810_e21444_d_n8, assign23810_e21444_d_n9, assign23810_e21444_d_n10, assign23810_e21444_d_n11, assign23810_e21444_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        let assign23810_e21438: f64 = (locals.var_t4__blk812 * locals.var_vox);
        let assign23810_e21439: f64 = (locals.var_t3__blk811 - assign23810_e21438);
        let assign23810_e21440: f64 = (locals.var_t2__blk810 * assign23810_e21439);
        let assign23810_e21442: f64 = (assign23810_e21440 / locals.var_t0__blk808);
        (assign23810_e21442, (((((locals.var_t2__blk810_dn3 * assign23810_e21439) + (locals.var_t2__blk810 * (locals.var_t3__blk811_dn3 - ((locals.var_t4__blk812_dn3 * locals.var_vox) + (locals.var_t4__blk812 * locals.var_vox_dn3))))) * locals.var_t0__blk808) - (assign23810_e21440 * locals.var_t0__blk808_dn3)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_t2__blk810_dn4 * assign23810_e21439) + (locals.var_t2__blk810 * (locals.var_t3__blk811_dn4 - ((locals.var_t4__blk812_dn4 * locals.var_vox) + (locals.var_t4__blk812 * locals.var_vox_dn4))))) * locals.var_t0__blk808) - (assign23810_e21440 * locals.var_t0__blk808_dn4)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_t2__blk810_dn5 * assign23810_e21439) + (locals.var_t2__blk810 * (locals.var_t3__blk811_dn5 - ((locals.var_t4__blk812_dn5 * locals.var_vox) + (locals.var_t4__blk812 * locals.var_vox_dn5))))) * locals.var_t0__blk808) - (assign23810_e21440 * locals.var_t0__blk808_dn5)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_t2__blk810_dn6 * assign23810_e21439) + (locals.var_t2__blk810 * (locals.var_t3__blk811_dn6 - ((locals.var_t4__blk812_dn6 * locals.var_vox) + (locals.var_t4__blk812 * locals.var_vox_dn6))))) * locals.var_t0__blk808) - (assign23810_e21440 * locals.var_t0__blk808_dn6)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_t2__blk810_dn7 * assign23810_e21439) + (locals.var_t2__blk810 * (locals.var_t3__blk811_dn7 - ((locals.var_t4__blk812_dn7 * locals.var_vox) + (locals.var_t4__blk812 * locals.var_vox_dn7))))) * locals.var_t0__blk808) - (assign23810_e21440 * locals.var_t0__blk808_dn7)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_t2__blk810_dn8 * assign23810_e21439) + (locals.var_t2__blk810 * (locals.var_t3__blk811_dn8 - ((locals.var_t4__blk812_dn8 * locals.var_vox) + (locals.var_t4__blk812 * locals.var_vox_dn8))))) * locals.var_t0__blk808) - (assign23810_e21440 * locals.var_t0__blk808_dn8)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_t2__blk810_dn9 * assign23810_e21439) + (locals.var_t2__blk810 * (locals.var_t3__blk811_dn9 - ((locals.var_t4__blk812_dn9 * locals.var_vox) + (locals.var_t4__blk812 * locals.var_vox_dn9))))) * locals.var_t0__blk808) - (assign23810_e21440 * locals.var_t0__blk808_dn9)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_t2__blk810_dn10 * assign23810_e21439) + (locals.var_t2__blk810 * (locals.var_t3__blk811_dn10 - ((locals.var_t4__blk812_dn10 * locals.var_vox) + (locals.var_t4__blk812 * locals.var_vox_dn10))))) * locals.var_t0__blk808) - (assign23810_e21440 * locals.var_t0__blk808_dn10)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_t2__blk810_dn11 * assign23810_e21439) + (locals.var_t2__blk810 * (locals.var_t3__blk811_dn11 - ((locals.var_t4__blk812_dn11 * locals.var_vox) + (locals.var_t4__blk812 * locals.var_vox_dn11))))) * locals.var_t0__blk808) - (assign23810_e21440 * locals.var_t0__blk808_dn11)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_t2__blk810_dn12 * assign23810_e21439) + (locals.var_t2__blk810 * (locals.var_t3__blk811_dn12 - ((locals.var_t4__blk812_dn12 * locals.var_vox) + (locals.var_t4__blk812 * locals.var_vox_dn12))))) * locals.var_t0__blk808) - (assign23810_e21440 * locals.var_t0__blk808_dn12)) / (locals.var_t0__blk808 * locals.var_t0__blk808)),)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign23810_e21444;
        locals.var_t6__blk814_dn3 = assign23810_e21444_d_n3;
        locals.var_t6__blk814_dn4 = assign23810_e21444_d_n4;
        locals.var_t6__blk814_dn5 = assign23810_e21444_d_n5;
        locals.var_t6__blk814_dn6 = assign23810_e21444_d_n6;
        locals.var_t6__blk814_dn7 = assign23810_e21444_d_n7;
        locals.var_t6__blk814_dn8 = assign23810_e21444_d_n8;
        locals.var_t6__blk814_dn9 = assign23810_e21444_d_n9;
        locals.var_t6__blk814_dn10 = assign23810_e21444_d_n10;
        locals.var_t6__blk814_dn11 = assign23810_e21444_d_n11;
        locals.var_t6__blk814_dn12 = assign23810_e21444_d_n12;

        let assign23820_e21447: f64 = if locals.var_t6__blk814 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1311 = assign23820_e21447;

        let (assign23830_e21459, assign23830_e21459_d_n3, assign23830_e21459_d_n4, assign23830_e21459_d_n5, assign23830_e21459_d_n6, assign23830_e21459_d_n7, assign23830_e21459_d_n8, assign23830_e21459_d_n9, assign23830_e21459_d_n10, assign23830_e21459_d_n11, assign23830_e21459_d_n12,) = {
    if ((locals.var_guard1306 != 0.0) && (locals.var_guard1311 != 0.0)) {
        let assign23830_e21454: f64 = (1.0 + locals.var_t6__blk814);
        let assign23830_e21456: f64 = (assign23830_e21454 - 100.0);
        let assign23830_e21457: f64 = (2.688117142e43 * assign23830_e21456);
        (assign23830_e21457, (2.688117142e43 * locals.var_t6__blk814_dn3), (2.688117142e43 * locals.var_t6__blk814_dn4), (2.688117142e43 * locals.var_t6__blk814_dn5), (2.688117142e43 * locals.var_t6__blk814_dn6), (2.688117142e43 * locals.var_t6__blk814_dn7), (2.688117142e43 * locals.var_t6__blk814_dn8), (2.688117142e43 * locals.var_t6__blk814_dn9), (2.688117142e43 * locals.var_t6__blk814_dn10), (2.688117142e43 * locals.var_t6__blk814_dn11), (2.688117142e43 * locals.var_t6__blk814_dn12),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign23830_e21459;
        locals.var_t5__blk813_dn3 = assign23830_e21459_d_n3;
        locals.var_t5__blk813_dn4 = assign23830_e21459_d_n4;
        locals.var_t5__blk813_dn5 = assign23830_e21459_d_n5;
        locals.var_t5__blk813_dn6 = assign23830_e21459_d_n6;
        locals.var_t5__blk813_dn7 = assign23830_e21459_d_n7;
        locals.var_t5__blk813_dn8 = assign23830_e21459_d_n8;
        locals.var_t5__blk813_dn9 = assign23830_e21459_d_n9;
        locals.var_t5__blk813_dn10 = assign23830_e21459_d_n10;
        locals.var_t5__blk813_dn11 = assign23830_e21459_d_n11;
        locals.var_t5__blk813_dn12 = assign23830_e21459_d_n12;

        let assign23840_e21462: f64 = (-100.0);
        let assign23840_e21463: f64 = if locals.var_t6__blk814 < assign23840_e21462 { 1.0 } else { 0.0 };
        locals.var_guard1312 = assign23840_e21463;

        let (assign23850_e21472, assign23850_e21472_d_n3, assign23850_e21472_d_n4, assign23850_e21472_d_n5, assign23850_e21472_d_n6, assign23850_e21472_d_n7, assign23850_e21472_d_n8, assign23850_e21472_d_n9, assign23850_e21472_d_n10, assign23850_e21472_d_n11, assign23850_e21472_d_n12,) = {
    if (((locals.var_guard1306 != 0.0) && (locals.var_guard1311 == 0.0)) && (locals.var_guard1312 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign23850_e21472;
        locals.var_t5__blk813_dn3 = assign23850_e21472_d_n3;
        locals.var_t5__blk813_dn4 = assign23850_e21472_d_n4;
        locals.var_t5__blk813_dn5 = assign23850_e21472_d_n5;
        locals.var_t5__blk813_dn6 = assign23850_e21472_d_n6;
        locals.var_t5__blk813_dn7 = assign23850_e21472_d_n7;
        locals.var_t5__blk813_dn8 = assign23850_e21472_d_n8;
        locals.var_t5__blk813_dn9 = assign23850_e21472_d_n9;
        locals.var_t5__blk813_dn10 = assign23850_e21472_d_n10;
        locals.var_t5__blk813_dn11 = assign23850_e21472_d_n11;
        locals.var_t5__blk813_dn12 = assign23850_e21472_d_n12;

        let (assign23860_e21483, assign23860_e21483_d_n3, assign23860_e21483_d_n4, assign23860_e21483_d_n5, assign23860_e21483_d_n6, assign23860_e21483_d_n7, assign23860_e21483_d_n8, assign23860_e21483_d_n9, assign23860_e21483_d_n10, assign23860_e21483_d_n11, assign23860_e21483_d_n12,) = {
    if (((locals.var_guard1306 != 0.0) && (locals.var_guard1311 == 0.0)) && (locals.var_guard1312 == 0.0)) {
        let assign23860_e21481: f64 = (locals.var_t6__blk814).exp();
        (assign23860_e21481, (assign23860_e21481 * locals.var_t6__blk814_dn3), (assign23860_e21481 * locals.var_t6__blk814_dn4), (assign23860_e21481 * locals.var_t6__blk814_dn5), (assign23860_e21481 * locals.var_t6__blk814_dn6), (assign23860_e21481 * locals.var_t6__blk814_dn7), (assign23860_e21481 * locals.var_t6__blk814_dn8), (assign23860_e21481 * locals.var_t6__blk814_dn9), (assign23860_e21481 * locals.var_t6__blk814_dn10), (assign23860_e21481 * locals.var_t6__blk814_dn11), (assign23860_e21481 * locals.var_t6__blk814_dn12),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign23860_e21483;
        locals.var_t5__blk813_dn3 = assign23860_e21483_d_n3;
        locals.var_t5__blk813_dn4 = assign23860_e21483_d_n4;
        locals.var_t5__blk813_dn5 = assign23860_e21483_d_n5;
        locals.var_t5__blk813_dn6 = assign23860_e21483_d_n6;
        locals.var_t5__blk813_dn7 = assign23860_e21483_d_n7;
        locals.var_t5__blk813_dn8 = assign23860_e21483_d_n8;
        locals.var_t5__blk813_dn9 = assign23860_e21483_d_n9;
        locals.var_t5__blk813_dn10 = assign23860_e21483_d_n10;
        locals.var_t5__blk813_dn11 = assign23860_e21483_d_n11;
        locals.var_t5__blk813_dn12 = assign23860_e21483_d_n12;

        let (assign23870_e21495, assign23870_e21495_d_n3, assign23870_e21495_d_n4, assign23870_e21495_d_n5, assign23870_e21495_d_n6, assign23870_e21495_d_n7, assign23870_e21495_d_n8, assign23870_e21495_d_n9, assign23870_e21495_d_n10, assign23870_e21495_d_n11, assign23870_e21495_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        let assign23870_e21487: f64 = (locals.var_t1__blk809 * locals.var_vgb);
        let assign23870_e21489: f64 = (assign23870_e21487 * locals.var_vaux);
        let assign23870_e21491: f64 = (assign23870_e21489 * locals.var_t5__blk813);
        let assign23870_e21493: f64 = (assign23870_e21491 * locals.var_igtemp);
        (assign23870_e21493, ((((((((locals.var_t1__blk809_dn3 * locals.var_vgb) + (locals.var_t1__blk809 * locals.var_vgb_dn3)) * locals.var_vaux) + (assign23870_e21487 * locals.var_vaux_dn3)) * locals.var_t5__blk813) + (assign23870_e21489 * locals.var_t5__blk813_dn3)) * locals.var_igtemp) + (assign23870_e21491 * locals.var_igtemp_dn3)), ((((((((locals.var_t1__blk809_dn4 * locals.var_vgb) + (locals.var_t1__blk809 * locals.var_vgb_dn4)) * locals.var_vaux) + (assign23870_e21487 * locals.var_vaux_dn4)) * locals.var_t5__blk813) + (assign23870_e21489 * locals.var_t5__blk813_dn4)) * locals.var_igtemp) + (assign23870_e21491 * locals.var_igtemp_dn4)), ((((((((locals.var_t1__blk809_dn5 * locals.var_vgb) + (locals.var_t1__blk809 * locals.var_vgb_dn5)) * locals.var_vaux) + (assign23870_e21487 * locals.var_vaux_dn5)) * locals.var_t5__blk813) + (assign23870_e21489 * locals.var_t5__blk813_dn5)) * locals.var_igtemp) + (assign23870_e21491 * locals.var_igtemp_dn5)), ((((((((locals.var_t1__blk809_dn6 * locals.var_vgb) + (locals.var_t1__blk809 * locals.var_vgb_dn6)) * locals.var_vaux) + (assign23870_e21487 * locals.var_vaux_dn6)) * locals.var_t5__blk813) + (assign23870_e21489 * locals.var_t5__blk813_dn6)) * locals.var_igtemp) + (assign23870_e21491 * locals.var_igtemp_dn6)), ((((((((locals.var_t1__blk809_dn7 * locals.var_vgb) + (locals.var_t1__blk809 * locals.var_vgb_dn7)) * locals.var_vaux) + (assign23870_e21487 * locals.var_vaux_dn7)) * locals.var_t5__blk813) + (assign23870_e21489 * locals.var_t5__blk813_dn7)) * locals.var_igtemp) + (assign23870_e21491 * locals.var_igtemp_dn7)), ((((((((locals.var_t1__blk809_dn8 * locals.var_vgb) + (locals.var_t1__blk809 * locals.var_vgb_dn8)) * locals.var_vaux) + (assign23870_e21487 * locals.var_vaux_dn8)) * locals.var_t5__blk813) + (assign23870_e21489 * locals.var_t5__blk813_dn8)) * locals.var_igtemp) + (assign23870_e21491 * locals.var_igtemp_dn8)), ((((((((locals.var_t1__blk809_dn9 * locals.var_vgb) + (locals.var_t1__blk809 * locals.var_vgb_dn9)) * locals.var_vaux) + (assign23870_e21487 * locals.var_vaux_dn9)) * locals.var_t5__blk813) + (assign23870_e21489 * locals.var_t5__blk813_dn9)) * locals.var_igtemp) + (assign23870_e21491 * locals.var_igtemp_dn9)), ((((((((locals.var_t1__blk809_dn10 * locals.var_vgb) + (locals.var_t1__blk809 * locals.var_vgb_dn10)) * locals.var_vaux) + (assign23870_e21487 * locals.var_vaux_dn10)) * locals.var_t5__blk813) + (assign23870_e21489 * locals.var_t5__blk813_dn10)) * locals.var_igtemp) + (assign23870_e21491 * locals.var_igtemp_dn10)), ((((((((locals.var_t1__blk809_dn11 * locals.var_vgb) + (locals.var_t1__blk809 * locals.var_vgb_dn11)) * locals.var_vaux) + (assign23870_e21487 * locals.var_vaux_dn11)) * locals.var_t5__blk813) + (assign23870_e21489 * locals.var_t5__blk813_dn11)) * locals.var_igtemp) + (assign23870_e21491 * locals.var_igtemp_dn11)), ((((((((locals.var_t1__blk809_dn12 * locals.var_vgb) + (locals.var_t1__blk809 * locals.var_vgb_dn12)) * locals.var_vaux) + (assign23870_e21487 * locals.var_vaux_dn12)) * locals.var_t5__blk813) + (assign23870_e21489 * locals.var_t5__blk813_dn12)) * locals.var_igtemp) + (assign23870_e21491 * locals.var_igtemp_dn12)),)
    } else {
        (locals.var_igb1, locals.var_igb1_dn3, locals.var_igb1_dn4, locals.var_igb1_dn5, locals.var_igb1_dn6, locals.var_igb1_dn7, locals.var_igb1_dn8, locals.var_igb1_dn9, locals.var_igb1_dn10, locals.var_igb1_dn11, locals.var_igb1_dn12,)
    }
};
        locals.var_igb1 = assign23870_e21495;
        locals.var_igb1_dn3 = assign23870_e21495_d_n3;
        locals.var_igb1_dn4 = assign23870_e21495_d_n4;
        locals.var_igb1_dn5 = assign23870_e21495_d_n5;
        locals.var_igb1_dn6 = assign23870_e21495_d_n6;
        locals.var_igb1_dn7 = assign23870_e21495_d_n7;
        locals.var_igb1_dn8 = assign23870_e21495_d_n8;
        locals.var_igb1_dn9 = assign23870_e21495_d_n9;
        locals.var_igb1_dn10 = assign23870_e21495_d_n10;
        locals.var_igb1_dn11 = assign23870_e21495_d_n11;
        locals.var_igb1_dn12 = assign23870_e21495_d_n12;

        let (assign23880_e21499, assign23880_e21499_d_n3, assign23880_e21499_d_n4, assign23880_e21499_d_n5, assign23880_e21499_d_n6, assign23880_e21499_d_n7, assign23880_e21499_d_n8, assign23880_e21499_d_n9, assign23880_e21499_d_n10, assign23880_e21499_d_n11, assign23880_e21499_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        (locals.var_voxacc, locals.var_voxacc_dn3, locals.var_voxacc_dn4, locals.var_voxacc_dn5, locals.var_voxacc_dn6, locals.var_voxacc_dn7, locals.var_voxacc_dn8, locals.var_voxacc_dn9, locals.var_voxacc_dn10, locals.var_voxacc_dn11, locals.var_voxacc_dn12,)
    } else {
        (locals.var_vox, locals.var_vox_dn3, locals.var_vox_dn4, locals.var_vox_dn5, locals.var_vox_dn6, locals.var_vox_dn7, locals.var_vox_dn8, locals.var_vox_dn9, locals.var_vox_dn10, locals.var_vox_dn11, locals.var_vox_dn12,)
    }
};
        locals.var_vox = assign23880_e21499;
        locals.var_vox_dn3 = assign23880_e21499_d_n3;
        locals.var_vox_dn4 = assign23880_e21499_d_n4;
        locals.var_vox_dn5 = assign23880_e21499_d_n5;
        locals.var_vox_dn6 = assign23880_e21499_d_n6;
        locals.var_vox_dn7 = assign23880_e21499_d_n7;
        locals.var_vox_dn8 = assign23880_e21499_d_n8;
        locals.var_vox_dn9 = assign23880_e21499_d_n9;
        locals.var_vox_dn10 = assign23880_e21499_d_n10;
        locals.var_vox_dn11 = assign23880_e21499_d_n11;
        locals.var_vox_dn12 = assign23880_e21499_d_n12;

        let (assign23890_e21503, assign23890_e21503_d_n3, assign23890_e21503_d_n4, assign23890_e21503_d_n5, assign23890_e21503_d_n6, assign23890_e21503_d_n7, assign23890_e21503_d_n8, assign23890_e21503_d_n9, assign23890_e21503_d_n10, assign23890_e21503_d_n11, assign23890_e21503_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        (p.p396, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign23890_e21503;
        locals.var_t0__blk808_dn3 = assign23890_e21503_d_n3;
        locals.var_t0__blk808_dn4 = assign23890_e21503_d_n4;
        locals.var_t0__blk808_dn5 = assign23890_e21503_d_n5;
        locals.var_t0__blk808_dn6 = assign23890_e21503_d_n6;
        locals.var_t0__blk808_dn7 = assign23890_e21503_d_n7;
        locals.var_t0__blk808_dn8 = assign23890_e21503_d_n8;
        locals.var_t0__blk808_dn9 = assign23890_e21503_d_n9;
        locals.var_t0__blk808_dn10 = assign23890_e21503_d_n10;
        locals.var_t0__blk808_dn11 = assign23890_e21503_d_n11;
        locals.var_t0__blk808_dn12 = assign23890_e21503_d_n12;

        let (assign23900_e21511, assign23900_e21511_d_n3, assign23900_e21511_d_n4, assign23900_e21511_d_n5, assign23900_e21511_d_n6, assign23900_e21511_d_n7, assign23900_e21511_d_n8, assign23900_e21511_d_n9, assign23900_e21511_d_n10, assign23900_e21511_d_n11, assign23900_e21511_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        let assign23900_e21507: f64 = (locals.var_t0__blk808 - locals.var_vox);
        let assign23900_e21509: f64 = (assign23900_e21507 - p.p397);
        (assign23900_e21509, (locals.var_t0__blk808_dn3 - locals.var_vox_dn3), (locals.var_t0__blk808_dn4 - locals.var_vox_dn4), (locals.var_t0__blk808_dn5 - locals.var_vox_dn5), (locals.var_t0__blk808_dn6 - locals.var_vox_dn6), (locals.var_t0__blk808_dn7 - locals.var_vox_dn7), (locals.var_t0__blk808_dn8 - locals.var_vox_dn8), (locals.var_t0__blk808_dn9 - locals.var_vox_dn9), (locals.var_t0__blk808_dn10 - locals.var_vox_dn10), (locals.var_t0__blk808_dn11 - locals.var_vox_dn11), (locals.var_t0__blk808_dn12 - locals.var_vox_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign23900_e21511;
        locals.var_t1__blk809_dn3 = assign23900_e21511_d_n3;
        locals.var_t1__blk809_dn4 = assign23900_e21511_d_n4;
        locals.var_t1__blk809_dn5 = assign23900_e21511_d_n5;
        locals.var_t1__blk809_dn6 = assign23900_e21511_d_n6;
        locals.var_t1__blk809_dn7 = assign23900_e21511_d_n7;
        locals.var_t1__blk809_dn8 = assign23900_e21511_d_n8;
        locals.var_t1__blk809_dn9 = assign23900_e21511_d_n9;
        locals.var_t1__blk809_dn10 = assign23900_e21511_d_n10;
        locals.var_t1__blk809_dn11 = assign23900_e21511_d_n11;
        locals.var_t1__blk809_dn12 = assign23900_e21511_d_n12;

        let (assign23910_e21524, assign23910_e21524_d_n3, assign23910_e21524_d_n4, assign23910_e21524_d_n5, assign23910_e21524_d_n6, assign23910_e21524_d_n7, assign23910_e21524_d_n8, assign23910_e21524_d_n9, assign23910_e21524_d_n10, assign23910_e21524_d_n11, assign23910_e21524_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        let assign23910_e21515: f64 = (locals.var_t1__blk809 * locals.var_t1__blk809);
        let assign23910_e21518: f64 = (4.0 * p.p397);
        let assign23910_e21520: f64 = (assign23910_e21518 * locals.var_t0__blk808);
        let assign23910_e21521: f64 = (assign23910_e21515 + assign23910_e21520);
        let assign23910_e21522: f64 = (assign23910_e21521).sqrt();
        (assign23910_e21522, ((((locals.var_t1__blk809_dn3 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn3)) + (assign23910_e21518 * locals.var_t0__blk808_dn3)) / (2.0 * assign23910_e21522)), ((((locals.var_t1__blk809_dn4 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn4)) + (assign23910_e21518 * locals.var_t0__blk808_dn4)) / (2.0 * assign23910_e21522)), ((((locals.var_t1__blk809_dn5 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn5)) + (assign23910_e21518 * locals.var_t0__blk808_dn5)) / (2.0 * assign23910_e21522)), ((((locals.var_t1__blk809_dn6 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn6)) + (assign23910_e21518 * locals.var_t0__blk808_dn6)) / (2.0 * assign23910_e21522)), ((((locals.var_t1__blk809_dn7 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn7)) + (assign23910_e21518 * locals.var_t0__blk808_dn7)) / (2.0 * assign23910_e21522)), ((((locals.var_t1__blk809_dn8 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn8)) + (assign23910_e21518 * locals.var_t0__blk808_dn8)) / (2.0 * assign23910_e21522)), ((((locals.var_t1__blk809_dn9 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn9)) + (assign23910_e21518 * locals.var_t0__blk808_dn9)) / (2.0 * assign23910_e21522)), ((((locals.var_t1__blk809_dn10 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn10)) + (assign23910_e21518 * locals.var_t0__blk808_dn10)) / (2.0 * assign23910_e21522)), ((((locals.var_t1__blk809_dn11 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn11)) + (assign23910_e21518 * locals.var_t0__blk808_dn11)) / (2.0 * assign23910_e21522)), ((((locals.var_t1__blk809_dn12 * locals.var_t1__blk809) + (locals.var_t1__blk809 * locals.var_t1__blk809_dn12)) + (assign23910_e21518 * locals.var_t0__blk808_dn12)) / (2.0 * assign23910_e21522)),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign23910_e21524;
        locals.var_t3__blk811_dn3 = assign23910_e21524_d_n3;
        locals.var_t3__blk811_dn4 = assign23910_e21524_d_n4;
        locals.var_t3__blk811_dn5 = assign23910_e21524_d_n5;
        locals.var_t3__blk811_dn6 = assign23910_e21524_d_n6;
        locals.var_t3__blk811_dn7 = assign23910_e21524_d_n7;
        locals.var_t3__blk811_dn8 = assign23910_e21524_d_n8;
        locals.var_t3__blk811_dn9 = assign23910_e21524_d_n9;
        locals.var_t3__blk811_dn10 = assign23910_e21524_d_n10;
        locals.var_t3__blk811_dn11 = assign23910_e21524_d_n11;
        locals.var_t3__blk811_dn12 = assign23910_e21524_d_n12;

        let (assign23920_e21534, assign23920_e21534_d_n3, assign23920_e21534_d_n4, assign23920_e21534_d_n5, assign23920_e21534_d_n6, assign23920_e21534_d_n7, assign23920_e21534_d_n8, assign23920_e21534_d_n9, assign23920_e21534_d_n10, assign23920_e21534_d_n11, assign23920_e21534_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        let assign23920_e21530: f64 = (locals.var_t1__blk809 + locals.var_t3__blk811);
        let assign23920_e21531: f64 = (0.5 * assign23920_e21530);
        let assign23920_e21532: f64 = (locals.var_t0__blk808 - assign23920_e21531);
        (assign23920_e21532, (locals.var_t0__blk808_dn3 - (0.5 * (locals.var_t1__blk809_dn3 + locals.var_t3__blk811_dn3))), (locals.var_t0__blk808_dn4 - (0.5 * (locals.var_t1__blk809_dn4 + locals.var_t3__blk811_dn4))), (locals.var_t0__blk808_dn5 - (0.5 * (locals.var_t1__blk809_dn5 + locals.var_t3__blk811_dn5))), (locals.var_t0__blk808_dn6 - (0.5 * (locals.var_t1__blk809_dn6 + locals.var_t3__blk811_dn6))), (locals.var_t0__blk808_dn7 - (0.5 * (locals.var_t1__blk809_dn7 + locals.var_t3__blk811_dn7))), (locals.var_t0__blk808_dn8 - (0.5 * (locals.var_t1__blk809_dn8 + locals.var_t3__blk811_dn8))), (locals.var_t0__blk808_dn9 - (0.5 * (locals.var_t1__blk809_dn9 + locals.var_t3__blk811_dn9))), (locals.var_t0__blk808_dn10 - (0.5 * (locals.var_t1__blk809_dn10 + locals.var_t3__blk811_dn10))), (locals.var_t0__blk808_dn11 - (0.5 * (locals.var_t1__blk809_dn11 + locals.var_t3__blk811_dn11))), (locals.var_t0__blk808_dn12 - (0.5 * (locals.var_t1__blk809_dn12 + locals.var_t3__blk811_dn12))),)
    } else {
        (locals.var_voxeff, locals.var_voxeff_dn3, locals.var_voxeff_dn4, locals.var_voxeff_dn5, locals.var_voxeff_dn6, locals.var_voxeff_dn7, locals.var_voxeff_dn8, locals.var_voxeff_dn9, locals.var_voxeff_dn10, locals.var_voxeff_dn11, locals.var_voxeff_dn12,)
    }
};
        locals.var_voxeff = assign23920_e21534;
        locals.var_voxeff_dn3 = assign23920_e21534_d_n3;
        locals.var_voxeff_dn4 = assign23920_e21534_d_n4;
        locals.var_voxeff_dn5 = assign23920_e21534_d_n5;
        locals.var_voxeff_dn6 = assign23920_e21534_d_n6;
        locals.var_voxeff_dn7 = assign23920_e21534_d_n7;
        locals.var_voxeff_dn8 = assign23920_e21534_d_n8;
        locals.var_voxeff_dn9 = assign23920_e21534_d_n9;
        locals.var_voxeff_dn10 = assign23920_e21534_d_n10;
        locals.var_voxeff_dn11 = assign23920_e21534_d_n11;
        locals.var_voxeff_dn12 = assign23920_e21534_d_n12;

    }

    pub(super) fn stamp_transient_block_71(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23930_e21538, assign23930_e21538_d_n3, assign23930_e21538_d_n4, assign23930_e21538_d_n5, assign23930_e21538_d_n6, assign23930_e21538_d_n7, assign23930_e21538_d_n8, assign23930_e21538_d_n9, assign23930_e21538_d_n10, assign23930_e21538_d_n11, assign23930_e21538_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        (locals.var_voxeff, locals.var_voxeff_dn3, locals.var_voxeff_dn4, locals.var_voxeff_dn5, locals.var_voxeff_dn6, locals.var_voxeff_dn7, locals.var_voxeff_dn8, locals.var_voxeff_dn9, locals.var_voxeff_dn10, locals.var_voxeff_dn11, locals.var_voxeff_dn12,)
    } else {
        (locals.var_vox, locals.var_vox_dn3, locals.var_vox_dn4, locals.var_vox_dn5, locals.var_vox_dn6, locals.var_vox_dn7, locals.var_vox_dn8, locals.var_vox_dn9, locals.var_vox_dn10, locals.var_vox_dn11, locals.var_vox_dn12,)
    }
};
        locals.var_vox = assign23930_e21538;
        locals.var_vox_dn3 = assign23930_e21538_d_n3;
        locals.var_vox_dn4 = assign23930_e21538_d_n4;
        locals.var_vox_dn5 = assign23930_e21538_d_n5;
        locals.var_vox_dn6 = assign23930_e21538_d_n6;
        locals.var_vox_dn7 = assign23930_e21538_d_n7;
        locals.var_vox_dn8 = assign23930_e21538_d_n8;
        locals.var_vox_dn9 = assign23930_e21538_d_n9;
        locals.var_vox_dn10 = assign23930_e21538_d_n10;
        locals.var_vox_dn11 = assign23930_e21538_d_n11;
        locals.var_vox_dn12 = assign23930_e21538_d_n12;

        let (assign23940_e21547, assign23940_e21547_d_n3, assign23940_e21547_d_n4, assign23940_e21547_d_n5, assign23940_e21547_d_n6, assign23940_e21547_d_n7, assign23940_e21547_d_n8, assign23940_e21547_d_n9, assign23940_e21547_d_n10, assign23940_e21547_d_n11, assign23940_e21547_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        let assign23940_e21541: f64 = (-locals.var_vgb);
        let assign23940_e21543: f64 = (assign23940_e21541 + locals.var_vfb);
        let assign23940_e21545: f64 = (assign23940_e21543 / p.p387);
        (assign23940_e21545, (((-locals.var_vgb_dn3) + locals.var_vfb_dn3) / p.p387), (((-locals.var_vgb_dn4) + locals.var_vfb_dn4) / p.p387), (((-locals.var_vgb_dn5) + locals.var_vfb_dn5) / p.p387), (((-locals.var_vgb_dn6) + locals.var_vfb_dn6) / p.p387), (((-locals.var_vgb_dn7) + locals.var_vfb_dn7) / p.p387), (((-locals.var_vgb_dn8) + locals.var_vfb_dn8) / p.p387), (((-locals.var_vgb_dn9) + locals.var_vfb_dn9) / p.p387), (((-locals.var_vgb_dn10) + locals.var_vfb_dn10) / p.p387), (((-locals.var_vgb_dn11) + locals.var_vfb_dn11) / p.p387), (((-locals.var_vgb_dn12) + locals.var_vfb_dn12) / p.p387),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign23940_e21547;
        locals.var_t0__blk808_dn3 = assign23940_e21547_d_n3;
        locals.var_t0__blk808_dn4 = assign23940_e21547_d_n4;
        locals.var_t0__blk808_dn5 = assign23940_e21547_d_n5;
        locals.var_t0__blk808_dn6 = assign23940_e21547_d_n6;
        locals.var_t0__blk808_dn7 = assign23940_e21547_d_n7;
        locals.var_t0__blk808_dn8 = assign23940_e21547_d_n8;
        locals.var_t0__blk808_dn9 = assign23940_e21547_d_n9;
        locals.var_t0__blk808_dn10 = assign23940_e21547_d_n10;
        locals.var_t0__blk808_dn11 = assign23940_e21547_d_n11;
        locals.var_t0__blk808_dn12 = assign23940_e21547_d_n12;

        let assign23950_e21550: f64 = if locals.var_t0__blk808 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1313 = assign23950_e21550;

        let (assign23960_e21562, assign23960_e21562_d_n3, assign23960_e21562_d_n4, assign23960_e21562_d_n5, assign23960_e21562_d_n6, assign23960_e21562_d_n7, assign23960_e21562_d_n8, assign23960_e21562_d_n9, assign23960_e21562_d_n10, assign23960_e21562_d_n11, assign23960_e21562_d_n12,) = {
    if ((locals.var_guard1306 != 0.0) && (locals.var_guard1313 != 0.0)) {
        let assign23960_e21557: f64 = (1.0 + locals.var_t0__blk808);
        let assign23960_e21559: f64 = (assign23960_e21557 - 100.0);
        let assign23960_e21560: f64 = (2.688117142e43 * assign23960_e21559);
        (assign23960_e21560, (2.688117142e43 * locals.var_t0__blk808_dn3), (2.688117142e43 * locals.var_t0__blk808_dn4), (2.688117142e43 * locals.var_t0__blk808_dn5), (2.688117142e43 * locals.var_t0__blk808_dn6), (2.688117142e43 * locals.var_t0__blk808_dn7), (2.688117142e43 * locals.var_t0__blk808_dn8), (2.688117142e43 * locals.var_t0__blk808_dn9), (2.688117142e43 * locals.var_t0__blk808_dn10), (2.688117142e43 * locals.var_t0__blk808_dn11), (2.688117142e43 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign23960_e21562;
        locals.var_t1__blk809_dn3 = assign23960_e21562_d_n3;
        locals.var_t1__blk809_dn4 = assign23960_e21562_d_n4;
        locals.var_t1__blk809_dn5 = assign23960_e21562_d_n5;
        locals.var_t1__blk809_dn6 = assign23960_e21562_d_n6;
        locals.var_t1__blk809_dn7 = assign23960_e21562_d_n7;
        locals.var_t1__blk809_dn8 = assign23960_e21562_d_n8;
        locals.var_t1__blk809_dn9 = assign23960_e21562_d_n9;
        locals.var_t1__blk809_dn10 = assign23960_e21562_d_n10;
        locals.var_t1__blk809_dn11 = assign23960_e21562_d_n11;
        locals.var_t1__blk809_dn12 = assign23960_e21562_d_n12;

        let assign23970_e21565: f64 = (-100.0);
        let assign23970_e21566: f64 = if locals.var_t0__blk808 < assign23970_e21565 { 1.0 } else { 0.0 };
        locals.var_guard1314 = assign23970_e21566;

        let (assign23980_e21575, assign23980_e21575_d_n3, assign23980_e21575_d_n4, assign23980_e21575_d_n5, assign23980_e21575_d_n6, assign23980_e21575_d_n7, assign23980_e21575_d_n8, assign23980_e21575_d_n9, assign23980_e21575_d_n10, assign23980_e21575_d_n11, assign23980_e21575_d_n12,) = {
    if (((locals.var_guard1306 != 0.0) && (locals.var_guard1313 == 0.0)) && (locals.var_guard1314 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign23980_e21575;
        locals.var_t1__blk809_dn3 = assign23980_e21575_d_n3;
        locals.var_t1__blk809_dn4 = assign23980_e21575_d_n4;
        locals.var_t1__blk809_dn5 = assign23980_e21575_d_n5;
        locals.var_t1__blk809_dn6 = assign23980_e21575_d_n6;
        locals.var_t1__blk809_dn7 = assign23980_e21575_d_n7;
        locals.var_t1__blk809_dn8 = assign23980_e21575_d_n8;
        locals.var_t1__blk809_dn9 = assign23980_e21575_d_n9;
        locals.var_t1__blk809_dn10 = assign23980_e21575_d_n10;
        locals.var_t1__blk809_dn11 = assign23980_e21575_d_n11;
        locals.var_t1__blk809_dn12 = assign23980_e21575_d_n12;

        let (assign23990_e21586, assign23990_e21586_d_n3, assign23990_e21586_d_n4, assign23990_e21586_d_n5, assign23990_e21586_d_n6, assign23990_e21586_d_n7, assign23990_e21586_d_n8, assign23990_e21586_d_n9, assign23990_e21586_d_n10, assign23990_e21586_d_n11, assign23990_e21586_d_n12,) = {
    if (((locals.var_guard1306 != 0.0) && (locals.var_guard1313 == 0.0)) && (locals.var_guard1314 == 0.0)) {
        let assign23990_e21584: f64 = (locals.var_t0__blk808).exp();
        (assign23990_e21584, (assign23990_e21584 * locals.var_t0__blk808_dn3), (assign23990_e21584 * locals.var_t0__blk808_dn4), (assign23990_e21584 * locals.var_t0__blk808_dn5), (assign23990_e21584 * locals.var_t0__blk808_dn6), (assign23990_e21584 * locals.var_t0__blk808_dn7), (assign23990_e21584 * locals.var_t0__blk808_dn8), (assign23990_e21584 * locals.var_t0__blk808_dn9), (assign23990_e21584 * locals.var_t0__blk808_dn10), (assign23990_e21584 * locals.var_t0__blk808_dn11), (assign23990_e21584 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign23990_e21586;
        locals.var_t1__blk809_dn3 = assign23990_e21586_d_n3;
        locals.var_t1__blk809_dn4 = assign23990_e21586_d_n4;
        locals.var_t1__blk809_dn5 = assign23990_e21586_d_n5;
        locals.var_t1__blk809_dn6 = assign23990_e21586_d_n6;
        locals.var_t1__blk809_dn7 = assign23990_e21586_d_n7;
        locals.var_t1__blk809_dn8 = assign23990_e21586_d_n8;
        locals.var_t1__blk809_dn9 = assign23990_e21586_d_n9;
        locals.var_t1__blk809_dn10 = assign23990_e21586_d_n10;
        locals.var_t1__blk809_dn11 = assign23990_e21586_d_n11;
        locals.var_t1__blk809_dn12 = assign23990_e21586_d_n12;

        let (assign24000_e21595, assign24000_e21595_d_n3, assign24000_e21595_d_n4, assign24000_e21595_d_n5, assign24000_e21595_d_n6, assign24000_e21595_d_n7, assign24000_e21595_d_n8, assign24000_e21595_d_n9, assign24000_e21595_d_n10, assign24000_e21595_d_n11, assign24000_e21595_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        let assign24000_e21591: f64 = (1.0 + locals.var_t1__blk809);
        let assign24000_e21592: f64 = (assign24000_e21591).ln();
        let assign24000_e21593: f64 = (p.p387 * assign24000_e21592);
        (assign24000_e21593, (p.p387 * (locals.var_t1__blk809_dn3 / assign24000_e21591)), (p.p387 * (locals.var_t1__blk809_dn4 / assign24000_e21591)), (p.p387 * (locals.var_t1__blk809_dn5 / assign24000_e21591)), (p.p387 * (locals.var_t1__blk809_dn6 / assign24000_e21591)), (p.p387 * (locals.var_t1__blk809_dn7 / assign24000_e21591)), (p.p387 * (locals.var_t1__blk809_dn8 / assign24000_e21591)), (p.p387 * (locals.var_t1__blk809_dn9 / assign24000_e21591)), (p.p387 * (locals.var_t1__blk809_dn10 / assign24000_e21591)), (p.p387 * (locals.var_t1__blk809_dn11 / assign24000_e21591)), (p.p387 * (locals.var_t1__blk809_dn12 / assign24000_e21591)),)
    } else {
        (locals.var_vaux, locals.var_vaux_dn3, locals.var_vaux_dn4, locals.var_vaux_dn5, locals.var_vaux_dn6, locals.var_vaux_dn7, locals.var_vaux_dn8, locals.var_vaux_dn9, locals.var_vaux_dn10, locals.var_vaux_dn11, locals.var_vaux_dn12,)
    }
};
        locals.var_vaux = assign24000_e21595;
        locals.var_vaux_dn3 = assign24000_e21595_d_n3;
        locals.var_vaux_dn4 = assign24000_e21595_d_n4;
        locals.var_vaux_dn5 = assign24000_e21595_d_n5;
        locals.var_vaux_dn6 = assign24000_e21595_d_n6;
        locals.var_vaux_dn7 = assign24000_e21595_d_n7;
        locals.var_vaux_dn8 = assign24000_e21595_d_n8;
        locals.var_vaux_dn9 = assign24000_e21595_d_n9;
        locals.var_vaux_dn10 = assign24000_e21595_d_n10;
        locals.var_vaux_dn11 = assign24000_e21595_d_n11;
        locals.var_vaux_dn12 = assign24000_e21595_d_n12;

        let assign24010_e21598: f64 = if p.p391 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1315 = assign24010_e21598;

        let (assign24020_e21608, assign24020_e21608_d_n3, assign24020_e21608_d_n4, assign24020_e21608_d_n5, assign24020_e21608_d_n6, assign24020_e21608_d_n7, assign24020_e21608_d_n8, assign24020_e21608_d_n9, assign24020_e21608_d_n10, assign24020_e21608_d_n11, assign24020_e21608_d_n12,) = {
    if ((locals.var_guard1306 != 0.0) && (locals.var_guard1315 != 0.0)) {
        let assign24020_e21605: f64 = (locals.var_vox / p.p391);
        let assign24020_e21606: f64 = (1.0 - assign24020_e21605);
        (assign24020_e21606, (-(locals.var_vox_dn3 / p.p391)), (-(locals.var_vox_dn4 / p.p391)), (-(locals.var_vox_dn5 / p.p391)), (-(locals.var_vox_dn6 / p.p391)), (-(locals.var_vox_dn7 / p.p391)), (-(locals.var_vox_dn8 / p.p391)), (-(locals.var_vox_dn9 / p.p391)), (-(locals.var_vox_dn10 / p.p391)), (-(locals.var_vox_dn11 / p.p391)), (-(locals.var_vox_dn12 / p.p391)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign24020_e21608;
        locals.var_t0__blk808_dn3 = assign24020_e21608_d_n3;
        locals.var_t0__blk808_dn4 = assign24020_e21608_d_n4;
        locals.var_t0__blk808_dn5 = assign24020_e21608_d_n5;
        locals.var_t0__blk808_dn6 = assign24020_e21608_d_n6;
        locals.var_t0__blk808_dn7 = assign24020_e21608_d_n7;
        locals.var_t0__blk808_dn8 = assign24020_e21608_d_n8;
        locals.var_t0__blk808_dn9 = assign24020_e21608_d_n9;
        locals.var_t0__blk808_dn10 = assign24020_e21608_d_n10;
        locals.var_t0__blk808_dn11 = assign24020_e21608_d_n11;
        locals.var_t0__blk808_dn12 = assign24020_e21608_d_n12;

        let (assign24030_e21615, assign24030_e21615_d_n3, assign24030_e21615_d_n4, assign24030_e21615_d_n5, assign24030_e21615_d_n6, assign24030_e21615_d_n7, assign24030_e21615_d_n8, assign24030_e21615_d_n9, assign24030_e21615_d_n10, assign24030_e21615_d_n11, assign24030_e21615_d_n12,) = {
    if ((locals.var_guard1306 != 0.0) && (locals.var_guard1315 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign24030_e21615;
        locals.var_t0__blk808_dn3 = assign24030_e21615_d_n3;
        locals.var_t0__blk808_dn4 = assign24030_e21615_d_n4;
        locals.var_t0__blk808_dn5 = assign24030_e21615_d_n5;
        locals.var_t0__blk808_dn6 = assign24030_e21615_d_n6;
        locals.var_t0__blk808_dn7 = assign24030_e21615_d_n7;
        locals.var_t0__blk808_dn8 = assign24030_e21615_d_n8;
        locals.var_t0__blk808_dn9 = assign24030_e21615_d_n9;
        locals.var_t0__blk808_dn10 = assign24030_e21615_d_n10;
        locals.var_t0__blk808_dn11 = assign24030_e21615_d_n11;
        locals.var_t0__blk808_dn12 = assign24030_e21615_d_n12;

        let assign24040_e21618: f64 = if locals.var_t0__blk808 < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard1316 = assign24040_e21618;

        let (assign24050_e21624, assign24050_e21624_d_n3, assign24050_e21624_d_n4, assign24050_e21624_d_n5, assign24050_e21624_d_n6, assign24050_e21624_d_n7, assign24050_e21624_d_n8, assign24050_e21624_d_n9, assign24050_e21624_d_n10, assign24050_e21624_d_n11, assign24050_e21624_d_n12,) = {
    if ((locals.var_guard1306 != 0.0) && (locals.var_guard1316 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign24050_e21624;
        locals.var_t0__blk808_dn3 = assign24050_e21624_d_n3;
        locals.var_t0__blk808_dn4 = assign24050_e21624_d_n4;
        locals.var_t0__blk808_dn5 = assign24050_e21624_d_n5;
        locals.var_t0__blk808_dn6 = assign24050_e21624_d_n6;
        locals.var_t0__blk808_dn7 = assign24050_e21624_d_n7;
        locals.var_t0__blk808_dn8 = assign24050_e21624_d_n8;
        locals.var_t0__blk808_dn9 = assign24050_e21624_d_n9;
        locals.var_t0__blk808_dn10 = assign24050_e21624_d_n10;
        locals.var_t0__blk808_dn11 = assign24050_e21624_d_n11;
        locals.var_t0__blk808_dn12 = assign24050_e21624_d_n12;

        let (assign24060_e21640, assign24060_e21640_d_n3, assign24060_e21640_d_n4, assign24060_e21640_d_n5, assign24060_e21640_d_n6, assign24060_e21640_d_n7, assign24060_e21640_d_n8, assign24060_e21640_d_n9, assign24060_e21640_d_n10, assign24060_e21640_d_n11, assign24060_e21640_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        let assign24060_e21628: f64 = (locals.var_leff * locals.var_weff);
        let assign24060_e21630: f64 = (assign24060_e21628 / p.p23);
        let assign24060_e21633: f64 = (p.p28 / p.p3);
        let assign24060_e21634: f64 = (assign24060_e21630 + assign24060_e21633);
        let assign24060_e21636: f64 = (assign24060_e21634 * p.p1037);
        let assign24060_e21638: f64 = (assign24060_e21636 * locals.var_oxideratio);
        (assign24060_e21638, (((((locals.var_leff_dn3 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn3)) / p.p23) * p.p1037) * locals.var_oxideratio), (((((locals.var_leff_dn4 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn4)) / p.p23) * p.p1037) * locals.var_oxideratio), (((((locals.var_leff_dn5 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn5)) / p.p23) * p.p1037) * locals.var_oxideratio), (((((locals.var_leff_dn6 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn6)) / p.p23) * p.p1037) * locals.var_oxideratio), (((((locals.var_leff_dn7 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn7)) / p.p23) * p.p1037) * locals.var_oxideratio), (((((locals.var_leff_dn8 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn8)) / p.p23) * p.p1037) * locals.var_oxideratio), (((((locals.var_leff_dn9 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn9)) / p.p23) * p.p1037) * locals.var_oxideratio), (((((locals.var_leff_dn10 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn10)) / p.p23) * p.p1037) * locals.var_oxideratio), (((((locals.var_leff_dn11 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn11)) / p.p23) * p.p1037) * locals.var_oxideratio), (((((locals.var_leff_dn12 * locals.var_weff) + (locals.var_leff * locals.var_weff_dn12)) / p.p23) * p.p1037) * locals.var_oxideratio),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign24060_e21640;
        locals.var_t1__blk809_dn3 = assign24060_e21640_d_n3;
        locals.var_t1__blk809_dn4 = assign24060_e21640_d_n4;
        locals.var_t1__blk809_dn5 = assign24060_e21640_d_n5;
        locals.var_t1__blk809_dn6 = assign24060_e21640_d_n6;
        locals.var_t1__blk809_dn7 = assign24060_e21640_d_n7;
        locals.var_t1__blk809_dn8 = assign24060_e21640_d_n8;
        locals.var_t1__blk809_dn9 = assign24060_e21640_d_n9;
        locals.var_t1__blk809_dn10 = assign24060_e21640_d_n10;
        locals.var_t1__blk809_dn11 = assign24060_e21640_d_n11;
        locals.var_t1__blk809_dn12 = assign24060_e21640_d_n12;

        let (assign24070_e21646, assign24070_e21646_d_n3, assign24070_e21646_d_n4, assign24070_e21646_d_n5, assign24070_e21646_d_n6, assign24070_e21646_d_n7, assign24070_e21646_d_n8, assign24070_e21646_d_n9, assign24070_e21646_d_n10, assign24070_e21646_d_n11, assign24070_e21646_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        let assign24070_e21644: f64 = (p.p1038 * p.p376);
        (assign24070_e21644, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign24070_e21646;
        locals.var_t2__blk810_dn3 = assign24070_e21646_d_n3;
        locals.var_t2__blk810_dn4 = assign24070_e21646_d_n4;
        locals.var_t2__blk810_dn5 = assign24070_e21646_d_n5;
        locals.var_t2__blk810_dn6 = assign24070_e21646_d_n6;
        locals.var_t2__blk810_dn7 = assign24070_e21646_d_n7;
        locals.var_t2__blk810_dn8 = assign24070_e21646_d_n8;
        locals.var_t2__blk810_dn9 = assign24070_e21646_d_n9;
        locals.var_t2__blk810_dn10 = assign24070_e21646_d_n10;
        locals.var_t2__blk810_dn11 = assign24070_e21646_d_n11;
        locals.var_t2__blk810_dn12 = assign24070_e21646_d_n12;

        let (assign24080_e21650, assign24080_e21650_d_n3, assign24080_e21650_d_n4, assign24080_e21650_d_n5, assign24080_e21650_d_n6, assign24080_e21650_d_n7, assign24080_e21650_d_n8, assign24080_e21650_d_n9, assign24080_e21650_d_n10, assign24080_e21650_d_n11, assign24080_e21650_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        (locals.var_pparam_b4soialphagb2, locals.var_pparam_b4soialphagb2_dn3, locals.var_pparam_b4soialphagb2_dn4, locals.var_pparam_b4soialphagb2_dn5, locals.var_pparam_b4soialphagb2_dn6, locals.var_pparam_b4soialphagb2_dn7, locals.var_pparam_b4soialphagb2_dn8, locals.var_pparam_b4soialphagb2_dn9, locals.var_pparam_b4soialphagb2_dn10, locals.var_pparam_b4soialphagb2_dn11, locals.var_pparam_b4soialphagb2_dn12,)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign24080_e21650;
        locals.var_t3__blk811_dn3 = assign24080_e21650_d_n3;
        locals.var_t3__blk811_dn4 = assign24080_e21650_d_n4;
        locals.var_t3__blk811_dn5 = assign24080_e21650_d_n5;
        locals.var_t3__blk811_dn6 = assign24080_e21650_d_n6;
        locals.var_t3__blk811_dn7 = assign24080_e21650_d_n7;
        locals.var_t3__blk811_dn8 = assign24080_e21650_d_n8;
        locals.var_t3__blk811_dn9 = assign24080_e21650_d_n9;
        locals.var_t3__blk811_dn10 = assign24080_e21650_d_n10;
        locals.var_t3__blk811_dn11 = assign24080_e21650_d_n11;
        locals.var_t3__blk811_dn12 = assign24080_e21650_d_n12;

        let (assign24090_e21654, assign24090_e21654_d_n3, assign24090_e21654_d_n4, assign24090_e21654_d_n5, assign24090_e21654_d_n6, assign24090_e21654_d_n7, assign24090_e21654_d_n8, assign24090_e21654_d_n9, assign24090_e21654_d_n10, assign24090_e21654_d_n11, assign24090_e21654_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        (locals.var_pparam_b4soibetagb2, locals.var_pparam_b4soibetagb2_dn3, locals.var_pparam_b4soibetagb2_dn4, locals.var_pparam_b4soibetagb2_dn5, locals.var_pparam_b4soibetagb2_dn6, locals.var_pparam_b4soibetagb2_dn7, locals.var_pparam_b4soibetagb2_dn8, locals.var_pparam_b4soibetagb2_dn9, locals.var_pparam_b4soibetagb2_dn10, locals.var_pparam_b4soibetagb2_dn11, locals.var_pparam_b4soibetagb2_dn12,)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign24090_e21654;
        locals.var_t4__blk812_dn3 = assign24090_e21654_d_n3;
        locals.var_t4__blk812_dn4 = assign24090_e21654_d_n4;
        locals.var_t4__blk812_dn5 = assign24090_e21654_d_n5;
        locals.var_t4__blk812_dn6 = assign24090_e21654_d_n6;
        locals.var_t4__blk812_dn7 = assign24090_e21654_d_n7;
        locals.var_t4__blk812_dn8 = assign24090_e21654_d_n8;
        locals.var_t4__blk812_dn9 = assign24090_e21654_d_n9;
        locals.var_t4__blk812_dn10 = assign24090_e21654_d_n10;
        locals.var_t4__blk812_dn11 = assign24090_e21654_d_n11;
        locals.var_t4__blk812_dn12 = assign24090_e21654_d_n12;

        let (assign24100_e21666, assign24100_e21666_d_n3, assign24100_e21666_d_n4, assign24100_e21666_d_n5, assign24100_e21666_d_n6, assign24100_e21666_d_n7, assign24100_e21666_d_n8, assign24100_e21666_d_n9, assign24100_e21666_d_n10, assign24100_e21666_d_n11, assign24100_e21666_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        let assign24100_e21660: f64 = (locals.var_t4__blk812 * locals.var_vox);
        let assign24100_e21661: f64 = (locals.var_t3__blk811 - assign24100_e21660);
        let assign24100_e21662: f64 = (locals.var_t2__blk810 * assign24100_e21661);
        let assign24100_e21664: f64 = (assign24100_e21662 / locals.var_t0__blk808);
        (assign24100_e21664, (((((locals.var_t2__blk810_dn3 * assign24100_e21661) + (locals.var_t2__blk810 * (locals.var_t3__blk811_dn3 - ((locals.var_t4__blk812_dn3 * locals.var_vox) + (locals.var_t4__blk812 * locals.var_vox_dn3))))) * locals.var_t0__blk808) - (assign24100_e21662 * locals.var_t0__blk808_dn3)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_t2__blk810_dn4 * assign24100_e21661) + (locals.var_t2__blk810 * (locals.var_t3__blk811_dn4 - ((locals.var_t4__blk812_dn4 * locals.var_vox) + (locals.var_t4__blk812 * locals.var_vox_dn4))))) * locals.var_t0__blk808) - (assign24100_e21662 * locals.var_t0__blk808_dn4)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_t2__blk810_dn5 * assign24100_e21661) + (locals.var_t2__blk810 * (locals.var_t3__blk811_dn5 - ((locals.var_t4__blk812_dn5 * locals.var_vox) + (locals.var_t4__blk812 * locals.var_vox_dn5))))) * locals.var_t0__blk808) - (assign24100_e21662 * locals.var_t0__blk808_dn5)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_t2__blk810_dn6 * assign24100_e21661) + (locals.var_t2__blk810 * (locals.var_t3__blk811_dn6 - ((locals.var_t4__blk812_dn6 * locals.var_vox) + (locals.var_t4__blk812 * locals.var_vox_dn6))))) * locals.var_t0__blk808) - (assign24100_e21662 * locals.var_t0__blk808_dn6)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_t2__blk810_dn7 * assign24100_e21661) + (locals.var_t2__blk810 * (locals.var_t3__blk811_dn7 - ((locals.var_t4__blk812_dn7 * locals.var_vox) + (locals.var_t4__blk812 * locals.var_vox_dn7))))) * locals.var_t0__blk808) - (assign24100_e21662 * locals.var_t0__blk808_dn7)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_t2__blk810_dn8 * assign24100_e21661) + (locals.var_t2__blk810 * (locals.var_t3__blk811_dn8 - ((locals.var_t4__blk812_dn8 * locals.var_vox) + (locals.var_t4__blk812 * locals.var_vox_dn8))))) * locals.var_t0__blk808) - (assign24100_e21662 * locals.var_t0__blk808_dn8)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_t2__blk810_dn9 * assign24100_e21661) + (locals.var_t2__blk810 * (locals.var_t3__blk811_dn9 - ((locals.var_t4__blk812_dn9 * locals.var_vox) + (locals.var_t4__blk812 * locals.var_vox_dn9))))) * locals.var_t0__blk808) - (assign24100_e21662 * locals.var_t0__blk808_dn9)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_t2__blk810_dn10 * assign24100_e21661) + (locals.var_t2__blk810 * (locals.var_t3__blk811_dn10 - ((locals.var_t4__blk812_dn10 * locals.var_vox) + (locals.var_t4__blk812 * locals.var_vox_dn10))))) * locals.var_t0__blk808) - (assign24100_e21662 * locals.var_t0__blk808_dn10)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_t2__blk810_dn11 * assign24100_e21661) + (locals.var_t2__blk810 * (locals.var_t3__blk811_dn11 - ((locals.var_t4__blk812_dn11 * locals.var_vox) + (locals.var_t4__blk812 * locals.var_vox_dn11))))) * locals.var_t0__blk808) - (assign24100_e21662 * locals.var_t0__blk808_dn11)) / (locals.var_t0__blk808 * locals.var_t0__blk808)), (((((locals.var_t2__blk810_dn12 * assign24100_e21661) + (locals.var_t2__blk810 * (locals.var_t3__blk811_dn12 - ((locals.var_t4__blk812_dn12 * locals.var_vox) + (locals.var_t4__blk812 * locals.var_vox_dn12))))) * locals.var_t0__blk808) - (assign24100_e21662 * locals.var_t0__blk808_dn12)) / (locals.var_t0__blk808 * locals.var_t0__blk808)),)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign24100_e21666;
        locals.var_t6__blk814_dn3 = assign24100_e21666_d_n3;
        locals.var_t6__blk814_dn4 = assign24100_e21666_d_n4;
        locals.var_t6__blk814_dn5 = assign24100_e21666_d_n5;
        locals.var_t6__blk814_dn6 = assign24100_e21666_d_n6;
        locals.var_t6__blk814_dn7 = assign24100_e21666_d_n7;
        locals.var_t6__blk814_dn8 = assign24100_e21666_d_n8;
        locals.var_t6__blk814_dn9 = assign24100_e21666_d_n9;
        locals.var_t6__blk814_dn10 = assign24100_e21666_d_n10;
        locals.var_t6__blk814_dn11 = assign24100_e21666_d_n11;
        locals.var_t6__blk814_dn12 = assign24100_e21666_d_n12;

        let assign24110_e21669: f64 = if locals.var_t6__blk814 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1317 = assign24110_e21669;

        let (assign24120_e21681, assign24120_e21681_d_n3, assign24120_e21681_d_n4, assign24120_e21681_d_n5, assign24120_e21681_d_n6, assign24120_e21681_d_n7, assign24120_e21681_d_n8, assign24120_e21681_d_n9, assign24120_e21681_d_n10, assign24120_e21681_d_n11, assign24120_e21681_d_n12,) = {
    if ((locals.var_guard1306 != 0.0) && (locals.var_guard1317 != 0.0)) {
        let assign24120_e21676: f64 = (1.0 + locals.var_t6__blk814);
        let assign24120_e21678: f64 = (assign24120_e21676 - 100.0);
        let assign24120_e21679: f64 = (2.688117142e43 * assign24120_e21678);
        (assign24120_e21679, (2.688117142e43 * locals.var_t6__blk814_dn3), (2.688117142e43 * locals.var_t6__blk814_dn4), (2.688117142e43 * locals.var_t6__blk814_dn5), (2.688117142e43 * locals.var_t6__blk814_dn6), (2.688117142e43 * locals.var_t6__blk814_dn7), (2.688117142e43 * locals.var_t6__blk814_dn8), (2.688117142e43 * locals.var_t6__blk814_dn9), (2.688117142e43 * locals.var_t6__blk814_dn10), (2.688117142e43 * locals.var_t6__blk814_dn11), (2.688117142e43 * locals.var_t6__blk814_dn12),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign24120_e21681;
        locals.var_t5__blk813_dn3 = assign24120_e21681_d_n3;
        locals.var_t5__blk813_dn4 = assign24120_e21681_d_n4;
        locals.var_t5__blk813_dn5 = assign24120_e21681_d_n5;
        locals.var_t5__blk813_dn6 = assign24120_e21681_d_n6;
        locals.var_t5__blk813_dn7 = assign24120_e21681_d_n7;
        locals.var_t5__blk813_dn8 = assign24120_e21681_d_n8;
        locals.var_t5__blk813_dn9 = assign24120_e21681_d_n9;
        locals.var_t5__blk813_dn10 = assign24120_e21681_d_n10;
        locals.var_t5__blk813_dn11 = assign24120_e21681_d_n11;
        locals.var_t5__blk813_dn12 = assign24120_e21681_d_n12;

        let assign24130_e21684: f64 = (-100.0);
        let assign24130_e21685: f64 = if locals.var_t6__blk814 < assign24130_e21684 { 1.0 } else { 0.0 };
        locals.var_guard1318 = assign24130_e21685;

        let (assign24140_e21694, assign24140_e21694_d_n3, assign24140_e21694_d_n4, assign24140_e21694_d_n5, assign24140_e21694_d_n6, assign24140_e21694_d_n7, assign24140_e21694_d_n8, assign24140_e21694_d_n9, assign24140_e21694_d_n10, assign24140_e21694_d_n11, assign24140_e21694_d_n12,) = {
    if (((locals.var_guard1306 != 0.0) && (locals.var_guard1317 == 0.0)) && (locals.var_guard1318 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign24140_e21694;
        locals.var_t5__blk813_dn3 = assign24140_e21694_d_n3;
        locals.var_t5__blk813_dn4 = assign24140_e21694_d_n4;
        locals.var_t5__blk813_dn5 = assign24140_e21694_d_n5;
        locals.var_t5__blk813_dn6 = assign24140_e21694_d_n6;
        locals.var_t5__blk813_dn7 = assign24140_e21694_d_n7;
        locals.var_t5__blk813_dn8 = assign24140_e21694_d_n8;
        locals.var_t5__blk813_dn9 = assign24140_e21694_d_n9;
        locals.var_t5__blk813_dn10 = assign24140_e21694_d_n10;
        locals.var_t5__blk813_dn11 = assign24140_e21694_d_n11;
        locals.var_t5__blk813_dn12 = assign24140_e21694_d_n12;

        let (assign24150_e21705, assign24150_e21705_d_n3, assign24150_e21705_d_n4, assign24150_e21705_d_n5, assign24150_e21705_d_n6, assign24150_e21705_d_n7, assign24150_e21705_d_n8, assign24150_e21705_d_n9, assign24150_e21705_d_n10, assign24150_e21705_d_n11, assign24150_e21705_d_n12,) = {
    if (((locals.var_guard1306 != 0.0) && (locals.var_guard1317 == 0.0)) && (locals.var_guard1318 == 0.0)) {
        let assign24150_e21703: f64 = (locals.var_t6__blk814).exp();
        (assign24150_e21703, (assign24150_e21703 * locals.var_t6__blk814_dn3), (assign24150_e21703 * locals.var_t6__blk814_dn4), (assign24150_e21703 * locals.var_t6__blk814_dn5), (assign24150_e21703 * locals.var_t6__blk814_dn6), (assign24150_e21703 * locals.var_t6__blk814_dn7), (assign24150_e21703 * locals.var_t6__blk814_dn8), (assign24150_e21703 * locals.var_t6__blk814_dn9), (assign24150_e21703 * locals.var_t6__blk814_dn10), (assign24150_e21703 * locals.var_t6__blk814_dn11), (assign24150_e21703 * locals.var_t6__blk814_dn12),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign24150_e21705;
        locals.var_t5__blk813_dn3 = assign24150_e21705_d_n3;
        locals.var_t5__blk813_dn4 = assign24150_e21705_d_n4;
        locals.var_t5__blk813_dn5 = assign24150_e21705_d_n5;
        locals.var_t5__blk813_dn6 = assign24150_e21705_d_n6;
        locals.var_t5__blk813_dn7 = assign24150_e21705_d_n7;
        locals.var_t5__blk813_dn8 = assign24150_e21705_d_n8;
        locals.var_t5__blk813_dn9 = assign24150_e21705_d_n9;
        locals.var_t5__blk813_dn10 = assign24150_e21705_d_n10;
        locals.var_t5__blk813_dn11 = assign24150_e21705_d_n11;
        locals.var_t5__blk813_dn12 = assign24150_e21705_d_n12;

        let (assign24160_e21717, assign24160_e21717_d_n3, assign24160_e21717_d_n4, assign24160_e21717_d_n5, assign24160_e21717_d_n6, assign24160_e21717_d_n7, assign24160_e21717_d_n8, assign24160_e21717_d_n9, assign24160_e21717_d_n10, assign24160_e21717_d_n11, assign24160_e21717_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        let assign24160_e21709: f64 = (locals.var_t1__blk809 * locals.var_vgb);
        let assign24160_e21711: f64 = (assign24160_e21709 * locals.var_vaux);
        let assign24160_e21713: f64 = (assign24160_e21711 * locals.var_t5__blk813);
        let assign24160_e21715: f64 = (assign24160_e21713 * locals.var_igtemp);
        (assign24160_e21715, ((((((((locals.var_t1__blk809_dn3 * locals.var_vgb) + (locals.var_t1__blk809 * locals.var_vgb_dn3)) * locals.var_vaux) + (assign24160_e21709 * locals.var_vaux_dn3)) * locals.var_t5__blk813) + (assign24160_e21711 * locals.var_t5__blk813_dn3)) * locals.var_igtemp) + (assign24160_e21713 * locals.var_igtemp_dn3)), ((((((((locals.var_t1__blk809_dn4 * locals.var_vgb) + (locals.var_t1__blk809 * locals.var_vgb_dn4)) * locals.var_vaux) + (assign24160_e21709 * locals.var_vaux_dn4)) * locals.var_t5__blk813) + (assign24160_e21711 * locals.var_t5__blk813_dn4)) * locals.var_igtemp) + (assign24160_e21713 * locals.var_igtemp_dn4)), ((((((((locals.var_t1__blk809_dn5 * locals.var_vgb) + (locals.var_t1__blk809 * locals.var_vgb_dn5)) * locals.var_vaux) + (assign24160_e21709 * locals.var_vaux_dn5)) * locals.var_t5__blk813) + (assign24160_e21711 * locals.var_t5__blk813_dn5)) * locals.var_igtemp) + (assign24160_e21713 * locals.var_igtemp_dn5)), ((((((((locals.var_t1__blk809_dn6 * locals.var_vgb) + (locals.var_t1__blk809 * locals.var_vgb_dn6)) * locals.var_vaux) + (assign24160_e21709 * locals.var_vaux_dn6)) * locals.var_t5__blk813) + (assign24160_e21711 * locals.var_t5__blk813_dn6)) * locals.var_igtemp) + (assign24160_e21713 * locals.var_igtemp_dn6)), ((((((((locals.var_t1__blk809_dn7 * locals.var_vgb) + (locals.var_t1__blk809 * locals.var_vgb_dn7)) * locals.var_vaux) + (assign24160_e21709 * locals.var_vaux_dn7)) * locals.var_t5__blk813) + (assign24160_e21711 * locals.var_t5__blk813_dn7)) * locals.var_igtemp) + (assign24160_e21713 * locals.var_igtemp_dn7)), ((((((((locals.var_t1__blk809_dn8 * locals.var_vgb) + (locals.var_t1__blk809 * locals.var_vgb_dn8)) * locals.var_vaux) + (assign24160_e21709 * locals.var_vaux_dn8)) * locals.var_t5__blk813) + (assign24160_e21711 * locals.var_t5__blk813_dn8)) * locals.var_igtemp) + (assign24160_e21713 * locals.var_igtemp_dn8)), ((((((((locals.var_t1__blk809_dn9 * locals.var_vgb) + (locals.var_t1__blk809 * locals.var_vgb_dn9)) * locals.var_vaux) + (assign24160_e21709 * locals.var_vaux_dn9)) * locals.var_t5__blk813) + (assign24160_e21711 * locals.var_t5__blk813_dn9)) * locals.var_igtemp) + (assign24160_e21713 * locals.var_igtemp_dn9)), ((((((((locals.var_t1__blk809_dn10 * locals.var_vgb) + (locals.var_t1__blk809 * locals.var_vgb_dn10)) * locals.var_vaux) + (assign24160_e21709 * locals.var_vaux_dn10)) * locals.var_t5__blk813) + (assign24160_e21711 * locals.var_t5__blk813_dn10)) * locals.var_igtemp) + (assign24160_e21713 * locals.var_igtemp_dn10)), ((((((((locals.var_t1__blk809_dn11 * locals.var_vgb) + (locals.var_t1__blk809 * locals.var_vgb_dn11)) * locals.var_vaux) + (assign24160_e21709 * locals.var_vaux_dn11)) * locals.var_t5__blk813) + (assign24160_e21711 * locals.var_t5__blk813_dn11)) * locals.var_igtemp) + (assign24160_e21713 * locals.var_igtemp_dn11)), ((((((((locals.var_t1__blk809_dn12 * locals.var_vgb) + (locals.var_t1__blk809 * locals.var_vgb_dn12)) * locals.var_vaux) + (assign24160_e21709 * locals.var_vaux_dn12)) * locals.var_t5__blk813) + (assign24160_e21711 * locals.var_t5__blk813_dn12)) * locals.var_igtemp) + (assign24160_e21713 * locals.var_igtemp_dn12)),)
    } else {
        (locals.var_igb2, locals.var_igb2_dn3, locals.var_igb2_dn4, locals.var_igb2_dn5, locals.var_igb2_dn6, locals.var_igb2_dn7, locals.var_igb2_dn8, locals.var_igb2_dn9, locals.var_igb2_dn10, locals.var_igb2_dn11, locals.var_igb2_dn12,)
    }
};
        locals.var_igb2 = assign24160_e21717;
        locals.var_igb2_dn3 = assign24160_e21717_d_n3;
        locals.var_igb2_dn4 = assign24160_e21717_d_n4;
        locals.var_igb2_dn5 = assign24160_e21717_d_n5;
        locals.var_igb2_dn6 = assign24160_e21717_d_n6;
        locals.var_igb2_dn7 = assign24160_e21717_d_n7;
        locals.var_igb2_dn8 = assign24160_e21717_d_n8;
        locals.var_igb2_dn9 = assign24160_e21717_d_n9;
        locals.var_igb2_dn10 = assign24160_e21717_d_n10;
        locals.var_igb2_dn11 = assign24160_e21717_d_n11;
        locals.var_igb2_dn12 = assign24160_e21717_d_n12;

        let assign24170_e21720: f64 = if locals.var_vgb >= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1319 = assign24170_e21720;

        let (assign24180_e21726, assign24180_e21726_d_n3, assign24180_e21726_d_n4, assign24180_e21726_d_n5, assign24180_e21726_d_n6, assign24180_e21726_d_n7, assign24180_e21726_d_n8, assign24180_e21726_d_n9, assign24180_e21726_d_n10, assign24180_e21726_d_n11, assign24180_e21726_d_n12,) = {
    if ((locals.var_guard1306 != 0.0) && (locals.var_guard1319 != 0.0)) {
        (locals.var_igb1, locals.var_igb1_dn3, locals.var_igb1_dn4, locals.var_igb1_dn5, locals.var_igb1_dn6, locals.var_igb1_dn7, locals.var_igb1_dn8, locals.var_igb1_dn9, locals.var_igb1_dn10, locals.var_igb1_dn11, locals.var_igb1_dn12,)
    } else {
        (locals.var_igb_1, locals.var_igb_1_dn3, locals.var_igb_1_dn4, locals.var_igb_1_dn5, locals.var_igb_1_dn6, locals.var_igb_1_dn7, locals.var_igb_1_dn8, locals.var_igb_1_dn9, locals.var_igb_1_dn10, locals.var_igb_1_dn11, locals.var_igb_1_dn12,)
    }
};
        locals.var_igb_1 = assign24180_e21726;
        locals.var_igb_1_dn3 = assign24180_e21726_d_n3;
        locals.var_igb_1_dn4 = assign24180_e21726_d_n4;
        locals.var_igb_1_dn5 = assign24180_e21726_d_n5;
        locals.var_igb_1_dn6 = assign24180_e21726_d_n6;
        locals.var_igb_1_dn7 = assign24180_e21726_d_n7;
        locals.var_igb_1_dn8 = assign24180_e21726_d_n8;
        locals.var_igb_1_dn9 = assign24180_e21726_d_n9;
        locals.var_igb_1_dn10 = assign24180_e21726_d_n10;
        locals.var_igb_1_dn11 = assign24180_e21726_d_n11;
        locals.var_igb_1_dn12 = assign24180_e21726_d_n12;

        let (assign24190_e21733, assign24190_e21733_d_n3, assign24190_e21733_d_n4, assign24190_e21733_d_n5, assign24190_e21733_d_n6, assign24190_e21733_d_n7, assign24190_e21733_d_n8, assign24190_e21733_d_n9, assign24190_e21733_d_n10, assign24190_e21733_d_n11, assign24190_e21733_d_n12,) = {
    if ((locals.var_guard1306 != 0.0) && (locals.var_guard1319 == 0.0)) {
        (locals.var_igb2, locals.var_igb2_dn3, locals.var_igb2_dn4, locals.var_igb2_dn5, locals.var_igb2_dn6, locals.var_igb2_dn7, locals.var_igb2_dn8, locals.var_igb2_dn9, locals.var_igb2_dn10, locals.var_igb2_dn11, locals.var_igb2_dn12,)
    } else {
        (locals.var_igb_1, locals.var_igb_1_dn3, locals.var_igb_1_dn4, locals.var_igb_1_dn5, locals.var_igb_1_dn6, locals.var_igb_1_dn7, locals.var_igb_1_dn8, locals.var_igb_1_dn9, locals.var_igb_1_dn10, locals.var_igb_1_dn11, locals.var_igb_1_dn12,)
    }
};
        locals.var_igb_1 = assign24190_e21733;
        locals.var_igb_1_dn3 = assign24190_e21733_d_n3;
        locals.var_igb_1_dn4 = assign24190_e21733_d_n4;
        locals.var_igb_1_dn5 = assign24190_e21733_d_n5;
        locals.var_igb_1_dn6 = assign24190_e21733_d_n6;
        locals.var_igb_1_dn7 = assign24190_e21733_d_n7;
        locals.var_igb_1_dn8 = assign24190_e21733_d_n8;
        locals.var_igb_1_dn9 = assign24190_e21733_d_n9;
        locals.var_igb_1_dn10 = assign24190_e21733_d_n10;
        locals.var_igb_1_dn11 = assign24190_e21733_d_n11;
        locals.var_igb_1_dn12 = assign24190_e21733_d_n12;

        let (assign24200_e21739, assign24200_e21739_d_n3, assign24200_e21739_d_n4, assign24200_e21739_d_n5, assign24200_e21739_d_n6, assign24200_e21739_d_n7, assign24200_e21739_d_n8, assign24200_e21739_d_n9, assign24200_e21739_d_n10, assign24200_e21739_d_n11, assign24200_e21739_d_n12,) = {
    if (locals.var_guard1306 != 0.0) {
        let assign24200_e21737: f64 = (locals.var_vfb + p.p1033);
        (assign24200_e21737, locals.var_vfb_dn3, locals.var_vfb_dn4, locals.var_vfb_dn5, locals.var_vfb_dn6, locals.var_vfb_dn7, locals.var_vfb_dn8, locals.var_vfb_dn9, locals.var_vfb_dn10, locals.var_vfb_dn11, locals.var_vfb_dn12,)
    } else {
        (locals.var_vfb2, locals.var_vfb2_dn3, locals.var_vfb2_dn4, locals.var_vfb2_dn5, locals.var_vfb2_dn6, locals.var_vfb2_dn7, locals.var_vfb2_dn8, locals.var_vfb2_dn9, locals.var_vfb2_dn10, locals.var_vfb2_dn11, locals.var_vfb2_dn12,)
    }
};
        locals.var_vfb2 = assign24200_e21739;
        locals.var_vfb2_dn3 = assign24200_e21739_d_n3;
        locals.var_vfb2_dn4 = assign24200_e21739_d_n4;
        locals.var_vfb2_dn5 = assign24200_e21739_d_n5;
        locals.var_vfb2_dn6 = assign24200_e21739_d_n6;
        locals.var_vfb2_dn7 = assign24200_e21739_d_n7;
        locals.var_vfb2_dn8 = assign24200_e21739_d_n8;
        locals.var_vfb2_dn9 = assign24200_e21739_d_n9;
        locals.var_vfb2_dn10 = assign24200_e21739_d_n10;
        locals.var_vfb2_dn11 = assign24200_e21739_d_n11;
        locals.var_vfb2_dn12 = assign24200_e21739_d_n12;

        let (assign24210_e21744, assign24210_e21744_d_n3, assign24210_e21744_d_n4, assign24210_e21744_d_n5, assign24210_e21744_d_n6, assign24210_e21744_d_n7, assign24210_e21744_d_n8, assign24210_e21744_d_n9, assign24210_e21744_d_n10, assign24210_e21744_d_n11, assign24210_e21744_d_n12,) = {
    if (locals.var_guard1306 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_igb_1, locals.var_igb_1_dn3, locals.var_igb_1_dn4, locals.var_igb_1_dn5, locals.var_igb_1_dn6, locals.var_igb_1_dn7, locals.var_igb_1_dn8, locals.var_igb_1_dn9, locals.var_igb_1_dn10, locals.var_igb_1_dn11, locals.var_igb_1_dn12,)
    }
};
        locals.var_igb_1 = assign24210_e21744;
        locals.var_igb_1_dn3 = assign24210_e21744_d_n3;
        locals.var_igb_1_dn4 = assign24210_e21744_d_n4;
        locals.var_igb_1_dn5 = assign24210_e21744_d_n5;
        locals.var_igb_1_dn6 = assign24210_e21744_d_n6;
        locals.var_igb_1_dn7 = assign24210_e21744_d_n7;
        locals.var_igb_1_dn8 = assign24210_e21744_d_n8;
        locals.var_igb_1_dn9 = assign24210_e21744_d_n9;
        locals.var_igb_1_dn10 = assign24210_e21744_d_n10;
        locals.var_igb_1_dn11 = assign24210_e21744_d_n11;
        locals.var_igb_1_dn12 = assign24210_e21744_d_n12;

        let assign24220_e21747: f64 = (p.p37 * locals.var_igb_1);
        locals.var_b4soiig = assign24220_e21747;
        locals.var_b4soiig_dn3 = (p.p37 * locals.var_igb_1_dn3);
        locals.var_b4soiig_dn4 = (p.p37 * locals.var_igb_1_dn4);
        locals.var_b4soiig_dn5 = (p.p37 * locals.var_igb_1_dn5);
        locals.var_b4soiig_dn6 = (p.p37 * locals.var_igb_1_dn6);
        locals.var_b4soiig_dn7 = (p.p37 * locals.var_igb_1_dn7);
        locals.var_b4soiig_dn8 = (p.p37 * locals.var_igb_1_dn8);
        locals.var_b4soiig_dn9 = (p.p37 * locals.var_igb_1_dn9);
        locals.var_b4soiig_dn10 = (p.p37 * locals.var_igb_1_dn10);
        locals.var_b4soiig_dn11 = (p.p37 * locals.var_igb_1_dn11);
        locals.var_b4soiig_dn12 = (p.p37 * locals.var_igb_1_dn12);

        let assign24230_e21766: f64 = if (((((p.p374 != 0.0) && (locals.var_b4soisoimod != 2.0)) && (locals.var_b4soibodymod != 0.0)) && (p.p27 > 0.0)) && (locals.var_vgp < locals.var_vfb2)) { 1.0 } else { 0.0 };
        locals.var_guard1320 = assign24230_e21766;

        let (assign24240_e21772, assign24240_e21772_d_n3, assign24240_e21772_d_n4, assign24240_e21772_d_n5, assign24240_e21772_d_n6, assign24240_e21772_d_n7, assign24240_e21772_d_n8, assign24240_e21772_d_n9, assign24240_e21772_d_n10, assign24240_e21772_d_n11, assign24240_e21772_d_n12,) = {
    if (locals.var_guard1320 != 0.0) {
        let assign24240_e21770: f64 = (locals.var_vgp - locals.var_vfb2);
        (assign24240_e21770, (-locals.var_vfb2_dn3), (locals.var_vgp_dn4 - locals.var_vfb2_dn4), (-locals.var_vfb2_dn5), (-locals.var_vfb2_dn6), (-locals.var_vfb2_dn7), (-locals.var_vfb2_dn8), (locals.var_vgp_dn9 - locals.var_vfb2_dn9), (-locals.var_vfb2_dn10), (-locals.var_vfb2_dn11), (-locals.var_vfb2_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign24240_e21772;
        locals.var_t0__blk808_dn3 = assign24240_e21772_d_n3;
        locals.var_t0__blk808_dn4 = assign24240_e21772_d_n4;
        locals.var_t0__blk808_dn5 = assign24240_e21772_d_n5;
        locals.var_t0__blk808_dn6 = assign24240_e21772_d_n6;
        locals.var_t0__blk808_dn7 = assign24240_e21772_d_n7;
        locals.var_t0__blk808_dn8 = assign24240_e21772_d_n8;
        locals.var_t0__blk808_dn9 = assign24240_e21772_d_n9;
        locals.var_t0__blk808_dn10 = assign24240_e21772_d_n10;
        locals.var_t0__blk808_dn11 = assign24240_e21772_d_n11;
        locals.var_t0__blk808_dn12 = assign24240_e21772_d_n12;

    }

    pub(super) fn stamp_transient_block_72(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24250_e21781, assign24250_e21781_d_n3, assign24250_e21781_d_n4, assign24250_e21781_d_n5, assign24250_e21781_d_n6, assign24250_e21781_d_n7, assign24250_e21781_d_n8, assign24250_e21781_d_n9, assign24250_e21781_d_n10, assign24250_e21781_d_n11, assign24250_e21781_d_n12,) = {
    if (locals.var_guard1320 != 0.0) {
        let assign24250_e21776: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        let assign24250_e21778: f64 = (assign24250_e21776 + 0.0001);
        let assign24250_e21779: f64 = (assign24250_e21778).sqrt();
        (assign24250_e21779, (((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)) / (2.0 * assign24250_e21779)), (((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)) / (2.0 * assign24250_e21779)), (((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)) / (2.0 * assign24250_e21779)), (((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)) / (2.0 * assign24250_e21779)), (((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)) / (2.0 * assign24250_e21779)), (((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)) / (2.0 * assign24250_e21779)), (((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)) / (2.0 * assign24250_e21779)), (((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)) / (2.0 * assign24250_e21779)), (((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)) / (2.0 * assign24250_e21779)), (((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)) / (2.0 * assign24250_e21779)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign24250_e21781;
        locals.var_t1__blk809_dn3 = assign24250_e21781_d_n3;
        locals.var_t1__blk809_dn4 = assign24250_e21781_d_n4;
        locals.var_t1__blk809_dn5 = assign24250_e21781_d_n5;
        locals.var_t1__blk809_dn6 = assign24250_e21781_d_n6;
        locals.var_t1__blk809_dn7 = assign24250_e21781_d_n7;
        locals.var_t1__blk809_dn8 = assign24250_e21781_d_n8;
        locals.var_t1__blk809_dn9 = assign24250_e21781_d_n9;
        locals.var_t1__blk809_dn10 = assign24250_e21781_d_n10;
        locals.var_t1__blk809_dn11 = assign24250_e21781_d_n11;
        locals.var_t1__blk809_dn12 = assign24250_e21781_d_n12;

        let (assign24260_e21792, assign24260_e21792_d_n3, assign24260_e21792_d_n4, assign24260_e21792_d_n5, assign24260_e21792_d_n6, assign24260_e21792_d_n7, assign24260_e21792_d_n8, assign24260_e21792_d_n9, assign24260_e21792_d_n10, assign24260_e21792_d_n11, assign24260_e21792_d_n12,) = {
    if (locals.var_guard1320 != 0.0) {
        let assign24260_e21785: f64 = (-locals.var_t0__blk808);
        let assign24260_e21787: f64 = (assign24260_e21785 + locals.var_t1__blk809);
        let assign24260_e21789: f64 = (assign24260_e21787 - 0.01);
        let assign24260_e21790: f64 = (0.5 * assign24260_e21789);
        (assign24260_e21790, (0.5 * ((-locals.var_t0__blk808_dn3) + locals.var_t1__blk809_dn3)), (0.5 * ((-locals.var_t0__blk808_dn4) + locals.var_t1__blk809_dn4)), (0.5 * ((-locals.var_t0__blk808_dn5) + locals.var_t1__blk809_dn5)), (0.5 * ((-locals.var_t0__blk808_dn6) + locals.var_t1__blk809_dn6)), (0.5 * ((-locals.var_t0__blk808_dn7) + locals.var_t1__blk809_dn7)), (0.5 * ((-locals.var_t0__blk808_dn8) + locals.var_t1__blk809_dn8)), (0.5 * ((-locals.var_t0__blk808_dn9) + locals.var_t1__blk809_dn9)), (0.5 * ((-locals.var_t0__blk808_dn10) + locals.var_t1__blk809_dn10)), (0.5 * ((-locals.var_t0__blk808_dn11) + locals.var_t1__blk809_dn11)), (0.5 * ((-locals.var_t0__blk808_dn12) + locals.var_t1__blk809_dn12)),)
    } else {
        (locals.var_vgp_eff, locals.var_vgp_eff_dn3, locals.var_vgp_eff_dn4, locals.var_vgp_eff_dn5, locals.var_vgp_eff_dn6, locals.var_vgp_eff_dn7, locals.var_vgp_eff_dn8, locals.var_vgp_eff_dn9, locals.var_vgp_eff_dn10, locals.var_vgp_eff_dn11, locals.var_vgp_eff_dn12,)
    }
};
        locals.var_vgp_eff = assign24260_e21792;
        locals.var_vgp_eff_dn3 = assign24260_e21792_d_n3;
        locals.var_vgp_eff_dn4 = assign24260_e21792_d_n4;
        locals.var_vgp_eff_dn5 = assign24260_e21792_d_n5;
        locals.var_vgp_eff_dn6 = assign24260_e21792_d_n6;
        locals.var_vgp_eff_dn7 = assign24260_e21792_d_n7;
        locals.var_vgp_eff_dn8 = assign24260_e21792_d_n8;
        locals.var_vgp_eff_dn9 = assign24260_e21792_d_n9;
        locals.var_vgp_eff_dn10 = assign24260_e21792_d_n10;
        locals.var_vgp_eff_dn11 = assign24260_e21792_d_n11;
        locals.var_vgp_eff_dn12 = assign24260_e21792_d_n12;

        let (assign24270_e21801, assign24270_e21801_d_n3, assign24270_e21801_d_n4, assign24270_e21801_d_n5, assign24270_e21801_d_n6, assign24270_e21801_d_n7, assign24270_e21801_d_n8, assign24270_e21801_d_n9, assign24270_e21801_d_n10, assign24270_e21801_d_n11, assign24270_e21801_d_n12,) = {
    if (locals.var_guard1320 != 0.0) {
        let (assign24270_e21799,) = {
            if (p.p37 == 1.0) {
                (p.p1039,)
            } else {
                (p.p1040,)
            }
        };
        (assign24270_e21799, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12,)
    }
};
        locals.var_t11 = assign24270_e21801;
        locals.var_t11_dn3 = assign24270_e21801_d_n3;
        locals.var_t11_dn4 = assign24270_e21801_d_n4;
        locals.var_t11_dn5 = assign24270_e21801_d_n5;
        locals.var_t11_dn6 = assign24270_e21801_d_n6;
        locals.var_t11_dn7 = assign24270_e21801_d_n7;
        locals.var_t11_dn8 = assign24270_e21801_d_n8;
        locals.var_t11_dn9 = assign24270_e21801_d_n9;
        locals.var_t11_dn10 = assign24270_e21801_d_n10;
        locals.var_t11_dn11 = assign24270_e21801_d_n11;
        locals.var_t11_dn12 = assign24270_e21801_d_n12;

        let (assign24280_e21810, assign24280_e21810_d_n3, assign24280_e21810_d_n4, assign24280_e21810_d_n5, assign24280_e21810_d_n6, assign24280_e21810_d_n7, assign24280_e21810_d_n8, assign24280_e21810_d_n9, assign24280_e21810_d_n10, assign24280_e21810_d_n11, assign24280_e21810_d_n12,) = {
    if (locals.var_guard1320 != 0.0) {
        let (assign24280_e21808,) = {
            if (p.p37 == 1.0) {
                (p.p1041,)
            } else {
                (p.p1042,)
            }
        };
        (assign24280_e21808, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11, locals.var_t12_dn12,)
    }
};
        locals.var_t12 = assign24280_e21810;
        locals.var_t12_dn3 = assign24280_e21810_d_n3;
        locals.var_t12_dn4 = assign24280_e21810_d_n4;
        locals.var_t12_dn5 = assign24280_e21810_d_n5;
        locals.var_t12_dn6 = assign24280_e21810_d_n6;
        locals.var_t12_dn7 = assign24280_e21810_d_n7;
        locals.var_t12_dn8 = assign24280_e21810_d_n8;
        locals.var_t12_dn9 = assign24280_e21810_d_n9;
        locals.var_t12_dn10 = assign24280_e21810_d_n10;
        locals.var_t12_dn11 = assign24280_e21810_d_n11;
        locals.var_t12_dn12 = assign24280_e21810_d_n12;

        let (assign24290_e21816, assign24290_e21816_d_n3, assign24290_e21816_d_n4, assign24290_e21816_d_n5, assign24290_e21816_d_n6, assign24290_e21816_d_n7, assign24290_e21816_d_n8, assign24290_e21816_d_n9, assign24290_e21816_d_n10, assign24290_e21816_d_n11, assign24290_e21816_d_n12,) = {
    if (locals.var_guard1320 != 0.0) {
        let assign24290_e21814: f64 = (locals.var_vgp * locals.var_vgp_eff);
        (assign24290_e21814, (locals.var_vgp * locals.var_vgp_eff_dn3), ((locals.var_vgp_dn4 * locals.var_vgp_eff) + (locals.var_vgp * locals.var_vgp_eff_dn4)), (locals.var_vgp * locals.var_vgp_eff_dn5), (locals.var_vgp * locals.var_vgp_eff_dn6), (locals.var_vgp * locals.var_vgp_eff_dn7), (locals.var_vgp * locals.var_vgp_eff_dn8), ((locals.var_vgp_dn9 * locals.var_vgp_eff) + (locals.var_vgp * locals.var_vgp_eff_dn9)), (locals.var_vgp * locals.var_vgp_eff_dn10), (locals.var_vgp * locals.var_vgp_eff_dn11), (locals.var_vgp * locals.var_vgp_eff_dn12),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign24290_e21816;
        locals.var_t2__blk810_dn3 = assign24290_e21816_d_n3;
        locals.var_t2__blk810_dn4 = assign24290_e21816_d_n4;
        locals.var_t2__blk810_dn5 = assign24290_e21816_d_n5;
        locals.var_t2__blk810_dn6 = assign24290_e21816_d_n6;
        locals.var_t2__blk810_dn7 = assign24290_e21816_d_n7;
        locals.var_t2__blk810_dn8 = assign24290_e21816_d_n8;
        locals.var_t2__blk810_dn9 = assign24290_e21816_d_n9;
        locals.var_t2__blk810_dn10 = assign24290_e21816_d_n10;
        locals.var_t2__blk810_dn11 = assign24290_e21816_d_n11;
        locals.var_t2__blk810_dn12 = assign24290_e21816_d_n12;

        let (assign24300_e21824, assign24300_e21824_d_n3, assign24300_e21824_d_n4, assign24300_e21824_d_n5, assign24300_e21824_d_n6, assign24300_e21824_d_n7, assign24300_e21824_d_n8, assign24300_e21824_d_n9, assign24300_e21824_d_n10, assign24300_e21824_d_n11, assign24300_e21824_d_n12,) = {
    if (locals.var_guard1320 != 0.0) {
        let assign24300_e21820: f64 = (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2);
        let assign24300_e21822: f64 = (assign24300_e21820 - locals.var_pparam_b4soibigbcp2);
        (assign24300_e21822, (((locals.var_pparam_b4soiaigbcp2_dn3 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2_dn3)) - locals.var_pparam_b4soibigbcp2_dn3), (((locals.var_pparam_b4soiaigbcp2_dn4 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2_dn4)) - locals.var_pparam_b4soibigbcp2_dn4), (((locals.var_pparam_b4soiaigbcp2_dn5 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2_dn5)) - locals.var_pparam_b4soibigbcp2_dn5), (((locals.var_pparam_b4soiaigbcp2_dn6 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2_dn6)) - locals.var_pparam_b4soibigbcp2_dn6), (((locals.var_pparam_b4soiaigbcp2_dn7 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2_dn7)) - locals.var_pparam_b4soibigbcp2_dn7), (((locals.var_pparam_b4soiaigbcp2_dn8 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2_dn8)) - locals.var_pparam_b4soibigbcp2_dn8), (((locals.var_pparam_b4soiaigbcp2_dn9 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2_dn9)) - locals.var_pparam_b4soibigbcp2_dn9), (((locals.var_pparam_b4soiaigbcp2_dn10 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2_dn10)) - locals.var_pparam_b4soibigbcp2_dn10), (((locals.var_pparam_b4soiaigbcp2_dn11 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2_dn11)) - locals.var_pparam_b4soibigbcp2_dn11), (((locals.var_pparam_b4soiaigbcp2_dn12 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soiaigbcp2 * locals.var_pparam_b4soicigbcp2_dn12)) - locals.var_pparam_b4soibigbcp2_dn12),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign24300_e21824;
        locals.var_t3__blk811_dn3 = assign24300_e21824_d_n3;
        locals.var_t3__blk811_dn4 = assign24300_e21824_d_n4;
        locals.var_t3__blk811_dn5 = assign24300_e21824_d_n5;
        locals.var_t3__blk811_dn6 = assign24300_e21824_d_n6;
        locals.var_t3__blk811_dn7 = assign24300_e21824_d_n7;
        locals.var_t3__blk811_dn8 = assign24300_e21824_d_n8;
        locals.var_t3__blk811_dn9 = assign24300_e21824_d_n9;
        locals.var_t3__blk811_dn10 = assign24300_e21824_d_n10;
        locals.var_t3__blk811_dn11 = assign24300_e21824_d_n11;
        locals.var_t3__blk811_dn12 = assign24300_e21824_d_n12;

        let (assign24310_e21830, assign24310_e21830_d_n3, assign24310_e21830_d_n4, assign24310_e21830_d_n5, assign24310_e21830_d_n6, assign24310_e21830_d_n7, assign24310_e21830_d_n8, assign24310_e21830_d_n9, assign24310_e21830_d_n10, assign24310_e21830_d_n11, assign24310_e21830_d_n12,) = {
    if (locals.var_guard1320 != 0.0) {
        let assign24310_e21828: f64 = (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2);
        (assign24310_e21828, ((locals.var_pparam_b4soibigbcp2_dn3 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2_dn3)), ((locals.var_pparam_b4soibigbcp2_dn4 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2_dn4)), ((locals.var_pparam_b4soibigbcp2_dn5 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2_dn5)), ((locals.var_pparam_b4soibigbcp2_dn6 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2_dn6)), ((locals.var_pparam_b4soibigbcp2_dn7 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2_dn7)), ((locals.var_pparam_b4soibigbcp2_dn8 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2_dn8)), ((locals.var_pparam_b4soibigbcp2_dn9 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2_dn9)), ((locals.var_pparam_b4soibigbcp2_dn10 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2_dn10)), ((locals.var_pparam_b4soibigbcp2_dn11 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2_dn11)), ((locals.var_pparam_b4soibigbcp2_dn12 * locals.var_pparam_b4soicigbcp2) + (locals.var_pparam_b4soibigbcp2 * locals.var_pparam_b4soicigbcp2_dn12)),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign24310_e21830;
        locals.var_t4__blk812_dn3 = assign24310_e21830_d_n3;
        locals.var_t4__blk812_dn4 = assign24310_e21830_d_n4;
        locals.var_t4__blk812_dn5 = assign24310_e21830_d_n5;
        locals.var_t4__blk812_dn6 = assign24310_e21830_d_n6;
        locals.var_t4__blk812_dn7 = assign24310_e21830_d_n7;
        locals.var_t4__blk812_dn8 = assign24310_e21830_d_n8;
        locals.var_t4__blk812_dn9 = assign24310_e21830_d_n9;
        locals.var_t4__blk812_dn10 = assign24310_e21830_d_n10;
        locals.var_t4__blk812_dn11 = assign24310_e21830_d_n11;
        locals.var_t4__blk812_dn12 = assign24310_e21830_d_n12;

        let (assign24320_e21849, assign24320_e21849_d_n3, assign24320_e21849_d_n4, assign24320_e21849_d_n5, assign24320_e21849_d_n6, assign24320_e21849_d_n7, assign24320_e21849_d_n8, assign24320_e21849_d_n9, assign24320_e21849_d_n10, assign24320_e21849_d_n11, assign24320_e21849_d_n12,) = {
    if (locals.var_guard1320 != 0.0) {
        let assign24320_e21833: f64 = (-locals.var_t12);
        let assign24320_e21835: f64 = (assign24320_e21833 * p.p376);
        let assign24320_e21839: f64 = (locals.var_t3__blk811 * locals.var_vgp_eff);
        let assign24320_e21840: f64 = (locals.var_pparam_b4soiaigbcp2 + assign24320_e21839);
        let assign24320_e21843: f64 = (locals.var_t4__blk812 * locals.var_vgp_eff);
        let assign24320_e21845: f64 = (assign24320_e21843 * locals.var_vgp_eff);
        let assign24320_e21846: f64 = (assign24320_e21840 - assign24320_e21845);
        let assign24320_e21847: f64 = (assign24320_e21835 * assign24320_e21846);
        (assign24320_e21847, ((((-locals.var_t12_dn3) * p.p376) * assign24320_e21846) + (assign24320_e21835 * ((locals.var_pparam_b4soiaigbcp2_dn3 + ((locals.var_t3__blk811_dn3 * locals.var_vgp_eff) + (locals.var_t3__blk811 * locals.var_vgp_eff_dn3))) - ((((locals.var_t4__blk812_dn3 * locals.var_vgp_eff) + (locals.var_t4__blk812 * locals.var_vgp_eff_dn3)) * locals.var_vgp_eff) + (assign24320_e21843 * locals.var_vgp_eff_dn3))))), ((((-locals.var_t12_dn4) * p.p376) * assign24320_e21846) + (assign24320_e21835 * ((locals.var_pparam_b4soiaigbcp2_dn4 + ((locals.var_t3__blk811_dn4 * locals.var_vgp_eff) + (locals.var_t3__blk811 * locals.var_vgp_eff_dn4))) - ((((locals.var_t4__blk812_dn4 * locals.var_vgp_eff) + (locals.var_t4__blk812 * locals.var_vgp_eff_dn4)) * locals.var_vgp_eff) + (assign24320_e21843 * locals.var_vgp_eff_dn4))))), ((((-locals.var_t12_dn5) * p.p376) * assign24320_e21846) + (assign24320_e21835 * ((locals.var_pparam_b4soiaigbcp2_dn5 + ((locals.var_t3__blk811_dn5 * locals.var_vgp_eff) + (locals.var_t3__blk811 * locals.var_vgp_eff_dn5))) - ((((locals.var_t4__blk812_dn5 * locals.var_vgp_eff) + (locals.var_t4__blk812 * locals.var_vgp_eff_dn5)) * locals.var_vgp_eff) + (assign24320_e21843 * locals.var_vgp_eff_dn5))))), ((((-locals.var_t12_dn6) * p.p376) * assign24320_e21846) + (assign24320_e21835 * ((locals.var_pparam_b4soiaigbcp2_dn6 + ((locals.var_t3__blk811_dn6 * locals.var_vgp_eff) + (locals.var_t3__blk811 * locals.var_vgp_eff_dn6))) - ((((locals.var_t4__blk812_dn6 * locals.var_vgp_eff) + (locals.var_t4__blk812 * locals.var_vgp_eff_dn6)) * locals.var_vgp_eff) + (assign24320_e21843 * locals.var_vgp_eff_dn6))))), ((((-locals.var_t12_dn7) * p.p376) * assign24320_e21846) + (assign24320_e21835 * ((locals.var_pparam_b4soiaigbcp2_dn7 + ((locals.var_t3__blk811_dn7 * locals.var_vgp_eff) + (locals.var_t3__blk811 * locals.var_vgp_eff_dn7))) - ((((locals.var_t4__blk812_dn7 * locals.var_vgp_eff) + (locals.var_t4__blk812 * locals.var_vgp_eff_dn7)) * locals.var_vgp_eff) + (assign24320_e21843 * locals.var_vgp_eff_dn7))))), ((((-locals.var_t12_dn8) * p.p376) * assign24320_e21846) + (assign24320_e21835 * ((locals.var_pparam_b4soiaigbcp2_dn8 + ((locals.var_t3__blk811_dn8 * locals.var_vgp_eff) + (locals.var_t3__blk811 * locals.var_vgp_eff_dn8))) - ((((locals.var_t4__blk812_dn8 * locals.var_vgp_eff) + (locals.var_t4__blk812 * locals.var_vgp_eff_dn8)) * locals.var_vgp_eff) + (assign24320_e21843 * locals.var_vgp_eff_dn8))))), ((((-locals.var_t12_dn9) * p.p376) * assign24320_e21846) + (assign24320_e21835 * ((locals.var_pparam_b4soiaigbcp2_dn9 + ((locals.var_t3__blk811_dn9 * locals.var_vgp_eff) + (locals.var_t3__blk811 * locals.var_vgp_eff_dn9))) - ((((locals.var_t4__blk812_dn9 * locals.var_vgp_eff) + (locals.var_t4__blk812 * locals.var_vgp_eff_dn9)) * locals.var_vgp_eff) + (assign24320_e21843 * locals.var_vgp_eff_dn9))))), ((((-locals.var_t12_dn10) * p.p376) * assign24320_e21846) + (assign24320_e21835 * ((locals.var_pparam_b4soiaigbcp2_dn10 + ((locals.var_t3__blk811_dn10 * locals.var_vgp_eff) + (locals.var_t3__blk811 * locals.var_vgp_eff_dn10))) - ((((locals.var_t4__blk812_dn10 * locals.var_vgp_eff) + (locals.var_t4__blk812 * locals.var_vgp_eff_dn10)) * locals.var_vgp_eff) + (assign24320_e21843 * locals.var_vgp_eff_dn10))))), ((((-locals.var_t12_dn11) * p.p376) * assign24320_e21846) + (assign24320_e21835 * ((locals.var_pparam_b4soiaigbcp2_dn11 + ((locals.var_t3__blk811_dn11 * locals.var_vgp_eff) + (locals.var_t3__blk811 * locals.var_vgp_eff_dn11))) - ((((locals.var_t4__blk812_dn11 * locals.var_vgp_eff) + (locals.var_t4__blk812 * locals.var_vgp_eff_dn11)) * locals.var_vgp_eff) + (assign24320_e21843 * locals.var_vgp_eff_dn11))))), ((((-locals.var_t12_dn12) * p.p376) * assign24320_e21846) + (assign24320_e21835 * ((locals.var_pparam_b4soiaigbcp2_dn12 + ((locals.var_t3__blk811_dn12 * locals.var_vgp_eff) + (locals.var_t3__blk811 * locals.var_vgp_eff_dn12))) - ((((locals.var_t4__blk812_dn12 * locals.var_vgp_eff) + (locals.var_t4__blk812 * locals.var_vgp_eff_dn12)) * locals.var_vgp_eff) + (assign24320_e21843 * locals.var_vgp_eff_dn12))))),)
    } else {
        (locals.var_t5__blk813, locals.var_t5__blk813_dn3, locals.var_t5__blk813_dn4, locals.var_t5__blk813_dn5, locals.var_t5__blk813_dn6, locals.var_t5__blk813_dn7, locals.var_t5__blk813_dn8, locals.var_t5__blk813_dn9, locals.var_t5__blk813_dn10, locals.var_t5__blk813_dn11, locals.var_t5__blk813_dn12,)
    }
};
        locals.var_t5__blk813 = assign24320_e21849;
        locals.var_t5__blk813_dn3 = assign24320_e21849_d_n3;
        locals.var_t5__blk813_dn4 = assign24320_e21849_d_n4;
        locals.var_t5__blk813_dn5 = assign24320_e21849_d_n5;
        locals.var_t5__blk813_dn6 = assign24320_e21849_d_n6;
        locals.var_t5__blk813_dn7 = assign24320_e21849_d_n7;
        locals.var_t5__blk813_dn8 = assign24320_e21849_d_n8;
        locals.var_t5__blk813_dn9 = assign24320_e21849_d_n9;
        locals.var_t5__blk813_dn10 = assign24320_e21849_d_n10;
        locals.var_t5__blk813_dn11 = assign24320_e21849_d_n11;
        locals.var_t5__blk813_dn12 = assign24320_e21849_d_n12;

        let assign24330_e21852: f64 = if locals.var_t5__blk813 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1321 = assign24330_e21852;

        let (assign24340_e21858, assign24340_e21858_d_n3, assign24340_e21858_d_n4, assign24340_e21858_d_n5, assign24340_e21858_d_n6, assign24340_e21858_d_n7, assign24340_e21858_d_n8, assign24340_e21858_d_n9, assign24340_e21858_d_n10, assign24340_e21858_d_n11, assign24340_e21858_d_n12,) = {
    if ((locals.var_guard1320 != 0.0) && (locals.var_guard1321 != 0.0)) {
        (2.688117142e43, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign24340_e21858;
        locals.var_t6__blk814_dn3 = assign24340_e21858_d_n3;
        locals.var_t6__blk814_dn4 = assign24340_e21858_d_n4;
        locals.var_t6__blk814_dn5 = assign24340_e21858_d_n5;
        locals.var_t6__blk814_dn6 = assign24340_e21858_d_n6;
        locals.var_t6__blk814_dn7 = assign24340_e21858_d_n7;
        locals.var_t6__blk814_dn8 = assign24340_e21858_d_n8;
        locals.var_t6__blk814_dn9 = assign24340_e21858_d_n9;
        locals.var_t6__blk814_dn10 = assign24340_e21858_d_n10;
        locals.var_t6__blk814_dn11 = assign24340_e21858_d_n11;
        locals.var_t6__blk814_dn12 = assign24340_e21858_d_n12;

        let assign24350_e21861: f64 = (-100.0);
        let assign24350_e21862: f64 = if locals.var_t5__blk813 < assign24350_e21861 { 1.0 } else { 0.0 };
        locals.var_guard1322 = assign24350_e21862;

        let (assign24360_e21871, assign24360_e21871_d_n3, assign24360_e21871_d_n4, assign24360_e21871_d_n5, assign24360_e21871_d_n6, assign24360_e21871_d_n7, assign24360_e21871_d_n8, assign24360_e21871_d_n9, assign24360_e21871_d_n10, assign24360_e21871_d_n11, assign24360_e21871_d_n12,) = {
    if (((locals.var_guard1320 != 0.0) && (locals.var_guard1321 == 0.0)) && (locals.var_guard1322 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign24360_e21871;
        locals.var_t6__blk814_dn3 = assign24360_e21871_d_n3;
        locals.var_t6__blk814_dn4 = assign24360_e21871_d_n4;
        locals.var_t6__blk814_dn5 = assign24360_e21871_d_n5;
        locals.var_t6__blk814_dn6 = assign24360_e21871_d_n6;
        locals.var_t6__blk814_dn7 = assign24360_e21871_d_n7;
        locals.var_t6__blk814_dn8 = assign24360_e21871_d_n8;
        locals.var_t6__blk814_dn9 = assign24360_e21871_d_n9;
        locals.var_t6__blk814_dn10 = assign24360_e21871_d_n10;
        locals.var_t6__blk814_dn11 = assign24360_e21871_d_n11;
        locals.var_t6__blk814_dn12 = assign24360_e21871_d_n12;

        let (assign24370_e21882, assign24370_e21882_d_n3, assign24370_e21882_d_n4, assign24370_e21882_d_n5, assign24370_e21882_d_n6, assign24370_e21882_d_n7, assign24370_e21882_d_n8, assign24370_e21882_d_n9, assign24370_e21882_d_n10, assign24370_e21882_d_n11, assign24370_e21882_d_n12,) = {
    if (((locals.var_guard1320 != 0.0) && (locals.var_guard1321 == 0.0)) && (locals.var_guard1322 == 0.0)) {
        let assign24370_e21880: f64 = (locals.var_t5__blk813).exp();
        (assign24370_e21880, (assign24370_e21880 * locals.var_t5__blk813_dn3), (assign24370_e21880 * locals.var_t5__blk813_dn4), (assign24370_e21880 * locals.var_t5__blk813_dn5), (assign24370_e21880 * locals.var_t5__blk813_dn6), (assign24370_e21880 * locals.var_t5__blk813_dn7), (assign24370_e21880 * locals.var_t5__blk813_dn8), (assign24370_e21880 * locals.var_t5__blk813_dn9), (assign24370_e21880 * locals.var_t5__blk813_dn10), (assign24370_e21880 * locals.var_t5__blk813_dn11), (assign24370_e21880 * locals.var_t5__blk813_dn12),)
    } else {
        (locals.var_t6__blk814, locals.var_t6__blk814_dn3, locals.var_t6__blk814_dn4, locals.var_t6__blk814_dn5, locals.var_t6__blk814_dn6, locals.var_t6__blk814_dn7, locals.var_t6__blk814_dn8, locals.var_t6__blk814_dn9, locals.var_t6__blk814_dn10, locals.var_t6__blk814_dn11, locals.var_t6__blk814_dn12,)
    }
};
        locals.var_t6__blk814 = assign24370_e21882;
        locals.var_t6__blk814_dn3 = assign24370_e21882_d_n3;
        locals.var_t6__blk814_dn4 = assign24370_e21882_d_n4;
        locals.var_t6__blk814_dn5 = assign24370_e21882_d_n5;
        locals.var_t6__blk814_dn6 = assign24370_e21882_d_n6;
        locals.var_t6__blk814_dn7 = assign24370_e21882_d_n7;
        locals.var_t6__blk814_dn8 = assign24370_e21882_d_n8;
        locals.var_t6__blk814_dn9 = assign24370_e21882_d_n9;
        locals.var_t6__blk814_dn10 = assign24370_e21882_d_n10;
        locals.var_t6__blk814_dn11 = assign24370_e21882_d_n11;
        locals.var_t6__blk814_dn12 = assign24370_e21882_d_n12;

        let (assign24380_e21890, assign24380_e21890_d_n3, assign24380_e21890_d_n4, assign24380_e21890_d_n5, assign24380_e21890_d_n6, assign24380_e21890_d_n7, assign24380_e21890_d_n8, assign24380_e21890_d_n9, assign24380_e21890_d_n10, assign24380_e21890_d_n11, assign24380_e21890_d_n12,) = {
    if (locals.var_guard1320 != 0.0) {
        let assign24380_e21886: f64 = (locals.var_t11 * p.p27);
        let assign24380_e21888: f64 = (assign24380_e21886 * locals.var_pparam_b4soioxideratio);
        (assign24380_e21888, ((locals.var_t11_dn3 * p.p27) * locals.var_pparam_b4soioxideratio), ((locals.var_t11_dn4 * p.p27) * locals.var_pparam_b4soioxideratio), ((locals.var_t11_dn5 * p.p27) * locals.var_pparam_b4soioxideratio), ((locals.var_t11_dn6 * p.p27) * locals.var_pparam_b4soioxideratio), ((locals.var_t11_dn7 * p.p27) * locals.var_pparam_b4soioxideratio), ((locals.var_t11_dn8 * p.p27) * locals.var_pparam_b4soioxideratio), ((locals.var_t11_dn9 * p.p27) * locals.var_pparam_b4soioxideratio), ((locals.var_t11_dn10 * p.p27) * locals.var_pparam_b4soioxideratio), ((locals.var_t11_dn11 * p.p27) * locals.var_pparam_b4soioxideratio), ((locals.var_t11_dn12 * p.p27) * locals.var_pparam_b4soioxideratio),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12,)
    }
};
        locals.var_t11 = assign24380_e21890;
        locals.var_t11_dn3 = assign24380_e21890_d_n3;
        locals.var_t11_dn4 = assign24380_e21890_d_n4;
        locals.var_t11_dn5 = assign24380_e21890_d_n5;
        locals.var_t11_dn6 = assign24380_e21890_d_n6;
        locals.var_t11_dn7 = assign24380_e21890_d_n7;
        locals.var_t11_dn8 = assign24380_e21890_d_n8;
        locals.var_t11_dn9 = assign24380_e21890_d_n9;
        locals.var_t11_dn10 = assign24380_e21890_d_n10;
        locals.var_t11_dn11 = assign24380_e21890_d_n11;
        locals.var_t11_dn12 = assign24380_e21890_d_n12;

        let (assign24390_e21900, assign24390_e21900_d_n3, assign24390_e21900_d_n4, assign24390_e21900_d_n5, assign24390_e21900_d_n6, assign24390_e21900_d_n7, assign24390_e21900_d_n8, assign24390_e21900_d_n9, assign24390_e21900_d_n10, assign24390_e21900_d_n11, assign24390_e21900_d_n12,) = {
    if (locals.var_guard1320 != 0.0) {
        let assign24390_e21894: f64 = (locals.var_t11 * locals.var_t2__blk810);
        let assign24390_e21896: f64 = (assign24390_e21894 * locals.var_t6__blk814);
        let assign24390_e21898: f64 = (assign24390_e21896 * locals.var_igtemp);
        (assign24390_e21898, ((((((locals.var_t11_dn3 * locals.var_t2__blk810) + (locals.var_t11 * locals.var_t2__blk810_dn3)) * locals.var_t6__blk814) + (assign24390_e21894 * locals.var_t6__blk814_dn3)) * locals.var_igtemp) + (assign24390_e21896 * locals.var_igtemp_dn3)), ((((((locals.var_t11_dn4 * locals.var_t2__blk810) + (locals.var_t11 * locals.var_t2__blk810_dn4)) * locals.var_t6__blk814) + (assign24390_e21894 * locals.var_t6__blk814_dn4)) * locals.var_igtemp) + (assign24390_e21896 * locals.var_igtemp_dn4)), ((((((locals.var_t11_dn5 * locals.var_t2__blk810) + (locals.var_t11 * locals.var_t2__blk810_dn5)) * locals.var_t6__blk814) + (assign24390_e21894 * locals.var_t6__blk814_dn5)) * locals.var_igtemp) + (assign24390_e21896 * locals.var_igtemp_dn5)), ((((((locals.var_t11_dn6 * locals.var_t2__blk810) + (locals.var_t11 * locals.var_t2__blk810_dn6)) * locals.var_t6__blk814) + (assign24390_e21894 * locals.var_t6__blk814_dn6)) * locals.var_igtemp) + (assign24390_e21896 * locals.var_igtemp_dn6)), ((((((locals.var_t11_dn7 * locals.var_t2__blk810) + (locals.var_t11 * locals.var_t2__blk810_dn7)) * locals.var_t6__blk814) + (assign24390_e21894 * locals.var_t6__blk814_dn7)) * locals.var_igtemp) + (assign24390_e21896 * locals.var_igtemp_dn7)), ((((((locals.var_t11_dn8 * locals.var_t2__blk810) + (locals.var_t11 * locals.var_t2__blk810_dn8)) * locals.var_t6__blk814) + (assign24390_e21894 * locals.var_t6__blk814_dn8)) * locals.var_igtemp) + (assign24390_e21896 * locals.var_igtemp_dn8)), ((((((locals.var_t11_dn9 * locals.var_t2__blk810) + (locals.var_t11 * locals.var_t2__blk810_dn9)) * locals.var_t6__blk814) + (assign24390_e21894 * locals.var_t6__blk814_dn9)) * locals.var_igtemp) + (assign24390_e21896 * locals.var_igtemp_dn9)), ((((((locals.var_t11_dn10 * locals.var_t2__blk810) + (locals.var_t11 * locals.var_t2__blk810_dn10)) * locals.var_t6__blk814) + (assign24390_e21894 * locals.var_t6__blk814_dn10)) * locals.var_igtemp) + (assign24390_e21896 * locals.var_igtemp_dn10)), ((((((locals.var_t11_dn11 * locals.var_t2__blk810) + (locals.var_t11 * locals.var_t2__blk810_dn11)) * locals.var_t6__blk814) + (assign24390_e21894 * locals.var_t6__blk814_dn11)) * locals.var_igtemp) + (assign24390_e21896 * locals.var_igtemp_dn11)), ((((((locals.var_t11_dn12 * locals.var_t2__blk810) + (locals.var_t11 * locals.var_t2__blk810_dn12)) * locals.var_t6__blk814) + (assign24390_e21894 * locals.var_t6__blk814_dn12)) * locals.var_igtemp) + (assign24390_e21896 * locals.var_igtemp_dn12)),)
    } else {
        (locals.var_ig_agbcp2, locals.var_ig_agbcp2_dn3, locals.var_ig_agbcp2_dn4, locals.var_ig_agbcp2_dn5, locals.var_ig_agbcp2_dn6, locals.var_ig_agbcp2_dn7, locals.var_ig_agbcp2_dn8, locals.var_ig_agbcp2_dn9, locals.var_ig_agbcp2_dn10, locals.var_ig_agbcp2_dn11, locals.var_ig_agbcp2_dn12,)
    }
};
        locals.var_ig_agbcp2 = assign24390_e21900;
        locals.var_ig_agbcp2_dn3 = assign24390_e21900_d_n3;
        locals.var_ig_agbcp2_dn4 = assign24390_e21900_d_n4;
        locals.var_ig_agbcp2_dn5 = assign24390_e21900_d_n5;
        locals.var_ig_agbcp2_dn6 = assign24390_e21900_d_n6;
        locals.var_ig_agbcp2_dn7 = assign24390_e21900_d_n7;
        locals.var_ig_agbcp2_dn8 = assign24390_e21900_d_n8;
        locals.var_ig_agbcp2_dn9 = assign24390_e21900_d_n9;
        locals.var_ig_agbcp2_dn10 = assign24390_e21900_d_n10;
        locals.var_ig_agbcp2_dn11 = assign24390_e21900_d_n11;
        locals.var_ig_agbcp2_dn12 = assign24390_e21900_d_n12;

        let (assign24400_e21905, assign24400_e21905_d_n3, assign24400_e21905_d_n4, assign24400_e21905_d_n5, assign24400_e21905_d_n6, assign24400_e21905_d_n7, assign24400_e21905_d_n8, assign24400_e21905_d_n9, assign24400_e21905_d_n10, assign24400_e21905_d_n11, assign24400_e21905_d_n12,) = {
    if (locals.var_guard1320 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ig_agbcp2, locals.var_ig_agbcp2_dn3, locals.var_ig_agbcp2_dn4, locals.var_ig_agbcp2_dn5, locals.var_ig_agbcp2_dn6, locals.var_ig_agbcp2_dn7, locals.var_ig_agbcp2_dn8, locals.var_ig_agbcp2_dn9, locals.var_ig_agbcp2_dn10, locals.var_ig_agbcp2_dn11, locals.var_ig_agbcp2_dn12,)
    }
};
        locals.var_ig_agbcp2 = assign24400_e21905;
        locals.var_ig_agbcp2_dn3 = assign24400_e21905_d_n3;
        locals.var_ig_agbcp2_dn4 = assign24400_e21905_d_n4;
        locals.var_ig_agbcp2_dn5 = assign24400_e21905_d_n5;
        locals.var_ig_agbcp2_dn6 = assign24400_e21905_d_n6;
        locals.var_ig_agbcp2_dn7 = assign24400_e21905_d_n7;
        locals.var_ig_agbcp2_dn8 = assign24400_e21905_d_n8;
        locals.var_ig_agbcp2_dn9 = assign24400_e21905_d_n9;
        locals.var_ig_agbcp2_dn10 = assign24400_e21905_d_n10;
        locals.var_ig_agbcp2_dn11 = assign24400_e21905_d_n11;
        locals.var_ig_agbcp2_dn12 = assign24400_e21905_d_n12;

        let assign24410_e21908: f64 = (p.p37 * locals.var_ig_agbcp2);
        locals.var_b4soiigp = assign24410_e21908;
        locals.var_b4soiigp_dn3 = (p.p37 * locals.var_ig_agbcp2_dn3);
        locals.var_b4soiigp_dn4 = (p.p37 * locals.var_ig_agbcp2_dn4);
        locals.var_b4soiigp_dn5 = (p.p37 * locals.var_ig_agbcp2_dn5);
        locals.var_b4soiigp_dn6 = (p.p37 * locals.var_ig_agbcp2_dn6);
        locals.var_b4soiigp_dn7 = (p.p37 * locals.var_ig_agbcp2_dn7);
        locals.var_b4soiigp_dn8 = (p.p37 * locals.var_ig_agbcp2_dn8);
        locals.var_b4soiigp_dn9 = (p.p37 * locals.var_ig_agbcp2_dn9);
        locals.var_b4soiigp_dn10 = (p.p37 * locals.var_ig_agbcp2_dn10);
        locals.var_b4soiigp_dn11 = (p.p37 * locals.var_ig_agbcp2_dn11);
        locals.var_b4soiigp_dn12 = (p.p37 * locals.var_ig_agbcp2_dn12);

        let assign24420_e21911: f64 = if locals.var_b4soisoimod != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1323 = assign24420_e21911;

        let assign24430_e21914: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1324 = assign24430_e21914;

        let assign24440_e21917: f64 = if locals.var_pparam_b4soialpha0 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1325 = assign24440_e21917;

        let (assign24450_e21925, assign24450_e21925_d_n3, assign24450_e21925_d_n4, assign24450_e21925_d_n5, assign24450_e21925_d_n6, assign24450_e21925_d_n7, assign24450_e21925_d_n8, assign24450_e21925_d_n9, assign24450_e21925_d_n10, assign24450_e21925_d_n11, assign24450_e21925_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 != 0.0)) && (locals.var_guard1325 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_iii, locals.var_iii_dn3, locals.var_iii_dn4, locals.var_iii_dn5, locals.var_iii_dn6, locals.var_iii_dn7, locals.var_iii_dn8, locals.var_iii_dn9, locals.var_iii_dn10, locals.var_iii_dn11, locals.var_iii_dn12,)
    }
};
        locals.var_iii = assign24450_e21925;
        locals.var_iii_dn3 = assign24450_e21925_d_n3;
        locals.var_iii_dn4 = assign24450_e21925_d_n4;
        locals.var_iii_dn5 = assign24450_e21925_d_n5;
        locals.var_iii_dn6 = assign24450_e21925_d_n6;
        locals.var_iii_dn7 = assign24450_e21925_d_n7;
        locals.var_iii_dn8 = assign24450_e21925_d_n8;
        locals.var_iii_dn9 = assign24450_e21925_d_n9;
        locals.var_iii_dn10 = assign24450_e21925_d_n10;
        locals.var_iii_dn11 = assign24450_e21925_d_n11;
        locals.var_iii_dn12 = assign24450_e21925_d_n12;

        let (assign24460_e21944, assign24460_e21944_d_n3, assign24460_e21944_d_n4, assign24460_e21944_d_n5, assign24460_e21944_d_n6, assign24460_e21944_d_n7, assign24460_e21944_d_n8, assign24460_e21944_d_n9, assign24460_e21944_d_n10, assign24460_e21944_d_n11, assign24460_e21944_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 != 0.0)) && (locals.var_guard1325 == 0.0)) {
        let assign24460_e21936: f64 = (p.p308 * locals.var_trm1);
        let assign24460_e21937: f64 = (1.0 + assign24460_e21936);
        let assign24460_e21938: f64 = (locals.var_pparam_b4soivdsatii0 * assign24460_e21937);
        let assign24460_e21941: f64 = (locals.var_pparam_b4soilii / locals.var_leff);
        let assign24460_e21942: f64 = (assign24460_e21938 - assign24460_e21941);
        (assign24460_e21942, ((locals.var_pparam_b4soivdsatii0_dn3 * assign24460_e21937) - (((locals.var_pparam_b4soilii_dn3 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn3)) / (locals.var_leff * locals.var_leff))), (((locals.var_pparam_b4soivdsatii0_dn4 * assign24460_e21937) + (locals.var_pparam_b4soivdsatii0 * (p.p308 * locals.var_trm1_dn4))) - (((locals.var_pparam_b4soilii_dn4 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn4)) / (locals.var_leff * locals.var_leff))), (((locals.var_pparam_b4soivdsatii0_dn5 * assign24460_e21937) + (locals.var_pparam_b4soivdsatii0 * (p.p308 * locals.var_trm1_dn5))) - (((locals.var_pparam_b4soilii_dn5 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn5)) / (locals.var_leff * locals.var_leff))), (((locals.var_pparam_b4soivdsatii0_dn6 * assign24460_e21937) + (locals.var_pparam_b4soivdsatii0 * (p.p308 * locals.var_trm1_dn6))) - (((locals.var_pparam_b4soilii_dn6 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn6)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn7 * assign24460_e21937) - (((locals.var_pparam_b4soilii_dn7 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn7)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn8 * assign24460_e21937) - (((locals.var_pparam_b4soilii_dn8 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn8)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn9 * assign24460_e21937) - (((locals.var_pparam_b4soilii_dn9 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn9)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn10 * assign24460_e21937) - (((locals.var_pparam_b4soilii_dn10 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn10)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn11 * assign24460_e21937) - (((locals.var_pparam_b4soilii_dn11 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn11)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn12 * assign24460_e21937) - (((locals.var_pparam_b4soilii_dn12 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn12)) / (locals.var_leff * locals.var_leff))),)
    } else {
        (locals.var_vdsatii0, locals.var_vdsatii0_dn3, locals.var_vdsatii0_dn4, locals.var_vdsatii0_dn5, locals.var_vdsatii0_dn6, locals.var_vdsatii0_dn7, locals.var_vdsatii0_dn8, locals.var_vdsatii0_dn9, locals.var_vdsatii0_dn10, locals.var_vdsatii0_dn11, locals.var_vdsatii0_dn12,)
    }
};
        locals.var_vdsatii0 = assign24460_e21944;
        locals.var_vdsatii0_dn3 = assign24460_e21944_d_n3;
        locals.var_vdsatii0_dn4 = assign24460_e21944_d_n4;
        locals.var_vdsatii0_dn5 = assign24460_e21944_d_n5;
        locals.var_vdsatii0_dn6 = assign24460_e21944_d_n6;
        locals.var_vdsatii0_dn7 = assign24460_e21944_d_n7;
        locals.var_vdsatii0_dn8 = assign24460_e21944_d_n8;
        locals.var_vdsatii0_dn9 = assign24460_e21944_d_n9;
        locals.var_vdsatii0_dn10 = assign24460_e21944_d_n10;
        locals.var_vdsatii0_dn11 = assign24460_e21944_d_n11;
        locals.var_vdsatii0_dn12 = assign24460_e21944_d_n12;

        let (assign24470_e21955, assign24470_e21955_d_n3, assign24470_e21955_d_n4, assign24470_e21955_d_n5, assign24470_e21955_d_n6, assign24470_e21955_d_n7, assign24470_e21955_d_n8, assign24470_e21955_d_n9, assign24470_e21955_d_n10, assign24470_e21955_d_n11, assign24470_e21955_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 != 0.0)) && (locals.var_guard1325 == 0.0)) {
        let assign24470_e21953: f64 = (locals.var_pparam_b4soiesatii * locals.var_leff);
        (assign24470_e21953, ((locals.var_pparam_b4soiesatii_dn3 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn3)), ((locals.var_pparam_b4soiesatii_dn4 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn4)), ((locals.var_pparam_b4soiesatii_dn5 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn5)), ((locals.var_pparam_b4soiesatii_dn6 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn6)), ((locals.var_pparam_b4soiesatii_dn7 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn7)), ((locals.var_pparam_b4soiesatii_dn8 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn8)), ((locals.var_pparam_b4soiesatii_dn9 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn9)), ((locals.var_pparam_b4soiesatii_dn10 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn10)), ((locals.var_pparam_b4soiesatii_dn11 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn11)), ((locals.var_pparam_b4soiesatii_dn12 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn12)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign24470_e21955;
        locals.var_t0__blk808_dn3 = assign24470_e21955_d_n3;
        locals.var_t0__blk808_dn4 = assign24470_e21955_d_n4;
        locals.var_t0__blk808_dn5 = assign24470_e21955_d_n5;
        locals.var_t0__blk808_dn6 = assign24470_e21955_d_n6;
        locals.var_t0__blk808_dn7 = assign24470_e21955_d_n7;
        locals.var_t0__blk808_dn8 = assign24470_e21955_d_n8;
        locals.var_t0__blk808_dn9 = assign24470_e21955_d_n9;
        locals.var_t0__blk808_dn10 = assign24470_e21955_d_n10;
        locals.var_t0__blk808_dn11 = assign24470_e21955_d_n11;
        locals.var_t0__blk808_dn12 = assign24470_e21955_d_n12;

        let (assign24480_e21970, assign24480_e21970_d_n3, assign24480_e21970_d_n4, assign24480_e21970_d_n5, assign24480_e21970_d_n6, assign24480_e21970_d_n7, assign24480_e21970_d_n8, assign24480_e21970_d_n9, assign24480_e21970_d_n10, assign24480_e21970_d_n11, assign24480_e21970_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 != 0.0)) && (locals.var_guard1325 == 0.0)) {
        let assign24480_e21964: f64 = (locals.var_pparam_b4soisii0 * locals.var_t0__blk808);
        let assign24480_e21967: f64 = (1.0 + locals.var_t0__blk808);
        let assign24480_e21968: f64 = (assign24480_e21964 / assign24480_e21967);
        (assign24480_e21968, (((((locals.var_pparam_b4soisii0_dn3 * locals.var_t0__blk808) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk808_dn3)) * assign24480_e21967) - (assign24480_e21964 * locals.var_t0__blk808_dn3)) / (assign24480_e21967 * assign24480_e21967)), (((((locals.var_pparam_b4soisii0_dn4 * locals.var_t0__blk808) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk808_dn4)) * assign24480_e21967) - (assign24480_e21964 * locals.var_t0__blk808_dn4)) / (assign24480_e21967 * assign24480_e21967)), (((((locals.var_pparam_b4soisii0_dn5 * locals.var_t0__blk808) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk808_dn5)) * assign24480_e21967) - (assign24480_e21964 * locals.var_t0__blk808_dn5)) / (assign24480_e21967 * assign24480_e21967)), (((((locals.var_pparam_b4soisii0_dn6 * locals.var_t0__blk808) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk808_dn6)) * assign24480_e21967) - (assign24480_e21964 * locals.var_t0__blk808_dn6)) / (assign24480_e21967 * assign24480_e21967)), (((((locals.var_pparam_b4soisii0_dn7 * locals.var_t0__blk808) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk808_dn7)) * assign24480_e21967) - (assign24480_e21964 * locals.var_t0__blk808_dn7)) / (assign24480_e21967 * assign24480_e21967)), (((((locals.var_pparam_b4soisii0_dn8 * locals.var_t0__blk808) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk808_dn8)) * assign24480_e21967) - (assign24480_e21964 * locals.var_t0__blk808_dn8)) / (assign24480_e21967 * assign24480_e21967)), (((((locals.var_pparam_b4soisii0_dn9 * locals.var_t0__blk808) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk808_dn9)) * assign24480_e21967) - (assign24480_e21964 * locals.var_t0__blk808_dn9)) / (assign24480_e21967 * assign24480_e21967)), (((((locals.var_pparam_b4soisii0_dn10 * locals.var_t0__blk808) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk808_dn10)) * assign24480_e21967) - (assign24480_e21964 * locals.var_t0__blk808_dn10)) / (assign24480_e21967 * assign24480_e21967)), (((((locals.var_pparam_b4soisii0_dn11 * locals.var_t0__blk808) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk808_dn11)) * assign24480_e21967) - (assign24480_e21964 * locals.var_t0__blk808_dn11)) / (assign24480_e21967 * assign24480_e21967)), (((((locals.var_pparam_b4soisii0_dn12 * locals.var_t0__blk808) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk808_dn12)) * assign24480_e21967) - (assign24480_e21964 * locals.var_t0__blk808_dn12)) / (assign24480_e21967 * assign24480_e21967)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign24480_e21970;
        locals.var_t1__blk809_dn3 = assign24480_e21970_d_n3;
        locals.var_t1__blk809_dn4 = assign24480_e21970_d_n4;
        locals.var_t1__blk809_dn5 = assign24480_e21970_d_n5;
        locals.var_t1__blk809_dn6 = assign24480_e21970_d_n6;
        locals.var_t1__blk809_dn7 = assign24480_e21970_d_n7;
        locals.var_t1__blk809_dn8 = assign24480_e21970_d_n8;
        locals.var_t1__blk809_dn9 = assign24480_e21970_d_n9;
        locals.var_t1__blk809_dn10 = assign24480_e21970_d_n10;
        locals.var_t1__blk809_dn11 = assign24480_e21970_d_n11;
        locals.var_t1__blk809_dn12 = assign24480_e21970_d_n12;

        let (assign24490_e21985, assign24490_e21985_d_n3, assign24490_e21985_d_n4, assign24490_e21985_d_n5, assign24490_e21985_d_n6, assign24490_e21985_d_n7, assign24490_e21985_d_n8, assign24490_e21985_d_n9, assign24490_e21985_d_n10, assign24490_e21985_d_n11, assign24490_e21985_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 != 0.0)) && (locals.var_guard1325 == 0.0)) {
        let assign24490_e21981: f64 = (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840);
        let assign24490_e21982: f64 = (1.0 + assign24490_e21981);
        let assign24490_e21983: f64 = (1.0 / assign24490_e21982);
        (assign24490_e21983, (-(((locals.var_pparam_b4soisii1_dn3 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840_dn3)) / (assign24490_e21982 * assign24490_e21982))), (-(((locals.var_pparam_b4soisii1_dn4 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840_dn4)) / (assign24490_e21982 * assign24490_e21982))), (-(((locals.var_pparam_b4soisii1_dn5 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840_dn5)) / (assign24490_e21982 * assign24490_e21982))), (-(((locals.var_pparam_b4soisii1_dn6 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840_dn6)) / (assign24490_e21982 * assign24490_e21982))), (-(((locals.var_pparam_b4soisii1_dn7 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840_dn7)) / (assign24490_e21982 * assign24490_e21982))), (-(((locals.var_pparam_b4soisii1_dn8 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840_dn8)) / (assign24490_e21982 * assign24490_e21982))), (-(((locals.var_pparam_b4soisii1_dn9 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840_dn9)) / (assign24490_e21982 * assign24490_e21982))), (-(((locals.var_pparam_b4soisii1_dn10 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840_dn10)) / (assign24490_e21982 * assign24490_e21982))), (-(((locals.var_pparam_b4soisii1_dn11 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840_dn11)) / (assign24490_e21982 * assign24490_e21982))), (-(((locals.var_pparam_b4soisii1_dn12 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840_dn12)) / (assign24490_e21982 * assign24490_e21982))),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign24490_e21985;
        locals.var_t0__blk808_dn3 = assign24490_e21985_d_n3;
        locals.var_t0__blk808_dn4 = assign24490_e21985_d_n4;
        locals.var_t0__blk808_dn5 = assign24490_e21985_d_n5;
        locals.var_t0__blk808_dn6 = assign24490_e21985_d_n6;
        locals.var_t0__blk808_dn7 = assign24490_e21985_d_n7;
        locals.var_t0__blk808_dn8 = assign24490_e21985_d_n8;
        locals.var_t0__blk808_dn9 = assign24490_e21985_d_n9;
        locals.var_t0__blk808_dn10 = assign24490_e21985_d_n10;
        locals.var_t0__blk808_dn11 = assign24490_e21985_d_n11;
        locals.var_t0__blk808_dn12 = assign24490_e21985_d_n12;

        let (assign24500_e21996, assign24500_e21996_d_n3, assign24500_e21996_d_n4, assign24500_e21996_d_n5, assign24500_e21996_d_n6, assign24500_e21996_d_n7, assign24500_e21996_d_n8, assign24500_e21996_d_n9, assign24500_e21996_d_n10, assign24500_e21996_d_n11, assign24500_e21996_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 != 0.0)) && (locals.var_guard1325 == 0.0)) {
        let assign24500_e21994: f64 = (locals.var_t0__blk808 + locals.var_pparam_b4soisii2);
        (assign24500_e21994, (locals.var_t0__blk808_dn3 + locals.var_pparam_b4soisii2_dn3), (locals.var_t0__blk808_dn4 + locals.var_pparam_b4soisii2_dn4), (locals.var_t0__blk808_dn5 + locals.var_pparam_b4soisii2_dn5), (locals.var_t0__blk808_dn6 + locals.var_pparam_b4soisii2_dn6), (locals.var_t0__blk808_dn7 + locals.var_pparam_b4soisii2_dn7), (locals.var_t0__blk808_dn8 + locals.var_pparam_b4soisii2_dn8), (locals.var_t0__blk808_dn9 + locals.var_pparam_b4soisii2_dn9), (locals.var_t0__blk808_dn10 + locals.var_pparam_b4soisii2_dn10), (locals.var_t0__blk808_dn11 + locals.var_pparam_b4soisii2_dn11), (locals.var_t0__blk808_dn12 + locals.var_pparam_b4soisii2_dn12),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign24500_e21996;
        locals.var_t3__blk811_dn3 = assign24500_e21996_d_n3;
        locals.var_t3__blk811_dn4 = assign24500_e21996_d_n4;
        locals.var_t3__blk811_dn5 = assign24500_e21996_d_n5;
        locals.var_t3__blk811_dn6 = assign24500_e21996_d_n6;
        locals.var_t3__blk811_dn7 = assign24500_e21996_d_n7;
        locals.var_t3__blk811_dn8 = assign24500_e21996_d_n8;
        locals.var_t3__blk811_dn9 = assign24500_e21996_d_n9;
        locals.var_t3__blk811_dn10 = assign24500_e21996_d_n10;
        locals.var_t3__blk811_dn11 = assign24500_e21996_d_n11;
        locals.var_t3__blk811_dn12 = assign24500_e21996_d_n12;

        let (assign24510_e22007, assign24510_e22007_d_n3, assign24510_e22007_d_n4, assign24510_e22007_d_n5, assign24510_e22007_d_n6, assign24510_e22007_d_n7, assign24510_e22007_d_n8, assign24510_e22007_d_n9, assign24510_e22007_d_n10, assign24510_e22007_d_n11, assign24510_e22007_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 != 0.0)) && (locals.var_guard1325 == 0.0)) {
        let assign24510_e22005: f64 = (locals.var_vgst__blk795 * locals.var_t3__blk811);
        (assign24510_e22005, ((locals.var_vgst__blk795_dn3 * locals.var_t3__blk811) + (locals.var_vgst__blk795 * locals.var_t3__blk811_dn3)), ((locals.var_vgst__blk795_dn4 * locals.var_t3__blk811) + (locals.var_vgst__blk795 * locals.var_t3__blk811_dn4)), ((locals.var_vgst__blk795_dn5 * locals.var_t3__blk811) + (locals.var_vgst__blk795 * locals.var_t3__blk811_dn5)), ((locals.var_vgst__blk795_dn6 * locals.var_t3__blk811) + (locals.var_vgst__blk795 * locals.var_t3__blk811_dn6)), ((locals.var_vgst__blk795_dn7 * locals.var_t3__blk811) + (locals.var_vgst__blk795 * locals.var_t3__blk811_dn7)), ((locals.var_vgst__blk795_dn8 * locals.var_t3__blk811) + (locals.var_vgst__blk795 * locals.var_t3__blk811_dn8)), ((locals.var_vgst__blk795_dn9 * locals.var_t3__blk811) + (locals.var_vgst__blk795 * locals.var_t3__blk811_dn9)), ((locals.var_vgst__blk795_dn10 * locals.var_t3__blk811) + (locals.var_vgst__blk795 * locals.var_t3__blk811_dn10)), ((locals.var_vgst__blk795_dn11 * locals.var_t3__blk811) + (locals.var_vgst__blk795 * locals.var_t3__blk811_dn11)), ((locals.var_vgst__blk795_dn12 * locals.var_t3__blk811) + (locals.var_vgst__blk795 * locals.var_t3__blk811_dn12)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign24510_e22007;
        locals.var_t2__blk810_dn3 = assign24510_e22007_d_n3;
        locals.var_t2__blk810_dn4 = assign24510_e22007_d_n4;
        locals.var_t2__blk810_dn5 = assign24510_e22007_d_n5;
        locals.var_t2__blk810_dn6 = assign24510_e22007_d_n6;
        locals.var_t2__blk810_dn7 = assign24510_e22007_d_n7;
        locals.var_t2__blk810_dn8 = assign24510_e22007_d_n8;
        locals.var_t2__blk810_dn9 = assign24510_e22007_d_n9;
        locals.var_t2__blk810_dn10 = assign24510_e22007_d_n10;
        locals.var_t2__blk810_dn11 = assign24510_e22007_d_n11;
        locals.var_t2__blk810_dn12 = assign24510_e22007_d_n12;

        let (assign24520_e22022, assign24520_e22022_d_n3, assign24520_e22022_d_n4, assign24520_e22022_d_n5, assign24520_e22022_d_n6, assign24520_e22022_d_n7, assign24520_e22022_d_n8, assign24520_e22022_d_n9, assign24520_e22022_d_n10, assign24520_e22022_d_n11, assign24520_e22022_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 != 0.0)) && (locals.var_guard1325 == 0.0)) {
        let assign24520_e22018: f64 = (locals.var_pparam_b4soisiid * locals.var_vds_1);
        let assign24520_e22019: f64 = (1.0 + assign24520_e22018);
        let assign24520_e22020: f64 = (1.0 / assign24520_e22019);
        (assign24520_e22020, (-((locals.var_pparam_b4soisiid_dn3 * locals.var_vds_1) / (assign24520_e22019 * assign24520_e22019))), (-((locals.var_pparam_b4soisiid_dn4 * locals.var_vds_1) / (assign24520_e22019 * assign24520_e22019))), (-((locals.var_pparam_b4soisiid_dn5 * locals.var_vds_1) / (assign24520_e22019 * assign24520_e22019))), (-((locals.var_pparam_b4soisiid_dn6 * locals.var_vds_1) / (assign24520_e22019 * assign24520_e22019))), (-(((locals.var_pparam_b4soisiid_dn7 * locals.var_vds_1) + (locals.var_pparam_b4soisiid * locals.var_vds_1_dn7)) / (assign24520_e22019 * assign24520_e22019))), (-(((locals.var_pparam_b4soisiid_dn8 * locals.var_vds_1) + (locals.var_pparam_b4soisiid * locals.var_vds_1_dn8)) / (assign24520_e22019 * assign24520_e22019))), (-((locals.var_pparam_b4soisiid_dn9 * locals.var_vds_1) / (assign24520_e22019 * assign24520_e22019))), (-((locals.var_pparam_b4soisiid_dn10 * locals.var_vds_1) / (assign24520_e22019 * assign24520_e22019))), (-((locals.var_pparam_b4soisiid_dn11 * locals.var_vds_1) / (assign24520_e22019 * assign24520_e22019))), (-((locals.var_pparam_b4soisiid_dn12 * locals.var_vds_1) / (assign24520_e22019 * assign24520_e22019))),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign24520_e22022;
        locals.var_t3__blk811_dn3 = assign24520_e22022_d_n3;
        locals.var_t3__blk811_dn4 = assign24520_e22022_d_n4;
        locals.var_t3__blk811_dn5 = assign24520_e22022_d_n5;
        locals.var_t3__blk811_dn6 = assign24520_e22022_d_n6;
        locals.var_t3__blk811_dn7 = assign24520_e22022_d_n7;
        locals.var_t3__blk811_dn8 = assign24520_e22022_d_n8;
        locals.var_t3__blk811_dn9 = assign24520_e22022_d_n9;
        locals.var_t3__blk811_dn10 = assign24520_e22022_d_n10;
        locals.var_t3__blk811_dn11 = assign24520_e22022_d_n11;
        locals.var_t3__blk811_dn12 = assign24520_e22022_d_n12;

    }

    pub(super) fn stamp_transient_block_73(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24530_e22035, assign24530_e22035_d_n3, assign24530_e22035_d_n4, assign24530_e22035_d_n5, assign24530_e22035_d_n6, assign24530_e22035_d_n7, assign24530_e22035_d_n8, assign24530_e22035_d_n9, assign24530_e22035_d_n10, assign24530_e22035_d_n11, assign24530_e22035_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 != 0.0)) && (locals.var_guard1325 == 0.0)) {
        let assign24530_e22031: f64 = (locals.var_t1__blk809 * locals.var_t2__blk810);
        let assign24530_e22033: f64 = (assign24530_e22031 * locals.var_t3__blk811);
        (assign24530_e22033, ((((locals.var_t1__blk809_dn3 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn3)) * locals.var_t3__blk811) + (assign24530_e22031 * locals.var_t3__blk811_dn3)), ((((locals.var_t1__blk809_dn4 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn4)) * locals.var_t3__blk811) + (assign24530_e22031 * locals.var_t3__blk811_dn4)), ((((locals.var_t1__blk809_dn5 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn5)) * locals.var_t3__blk811) + (assign24530_e22031 * locals.var_t3__blk811_dn5)), ((((locals.var_t1__blk809_dn6 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn6)) * locals.var_t3__blk811) + (assign24530_e22031 * locals.var_t3__blk811_dn6)), ((((locals.var_t1__blk809_dn7 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn7)) * locals.var_t3__blk811) + (assign24530_e22031 * locals.var_t3__blk811_dn7)), ((((locals.var_t1__blk809_dn8 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn8)) * locals.var_t3__blk811) + (assign24530_e22031 * locals.var_t3__blk811_dn8)), ((((locals.var_t1__blk809_dn9 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn9)) * locals.var_t3__blk811) + (assign24530_e22031 * locals.var_t3__blk811_dn9)), ((((locals.var_t1__blk809_dn10 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn10)) * locals.var_t3__blk811) + (assign24530_e22031 * locals.var_t3__blk811_dn10)), ((((locals.var_t1__blk809_dn11 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn11)) * locals.var_t3__blk811) + (assign24530_e22031 * locals.var_t3__blk811_dn11)), ((((locals.var_t1__blk809_dn12 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn12)) * locals.var_t3__blk811) + (assign24530_e22031 * locals.var_t3__blk811_dn12)),)
    } else {
        (locals.var_vgsstep, locals.var_vgsstep_dn3, locals.var_vgsstep_dn4, locals.var_vgsstep_dn5, locals.var_vgsstep_dn6, locals.var_vgsstep_dn7, locals.var_vgsstep_dn8, locals.var_vgsstep_dn9, locals.var_vgsstep_dn10, locals.var_vgsstep_dn11, locals.var_vgsstep_dn12,)
    }
};
        locals.var_vgsstep = assign24530_e22035;
        locals.var_vgsstep_dn3 = assign24530_e22035_d_n3;
        locals.var_vgsstep_dn4 = assign24530_e22035_d_n4;
        locals.var_vgsstep_dn5 = assign24530_e22035_d_n5;
        locals.var_vgsstep_dn6 = assign24530_e22035_d_n6;
        locals.var_vgsstep_dn7 = assign24530_e22035_d_n7;
        locals.var_vgsstep_dn8 = assign24530_e22035_d_n8;
        locals.var_vgsstep_dn9 = assign24530_e22035_d_n9;
        locals.var_vgsstep_dn10 = assign24530_e22035_d_n10;
        locals.var_vgsstep_dn11 = assign24530_e22035_d_n11;
        locals.var_vgsstep_dn12 = assign24530_e22035_d_n12;

        let (assign24540_e22046, assign24540_e22046_d_n3, assign24540_e22046_d_n4, assign24540_e22046_d_n5, assign24540_e22046_d_n6, assign24540_e22046_d_n7, assign24540_e22046_d_n8, assign24540_e22046_d_n9, assign24540_e22046_d_n10, assign24540_e22046_d_n11, assign24540_e22046_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 != 0.0)) && (locals.var_guard1325 == 0.0)) {
        let assign24540_e22044: f64 = (locals.var_vdsatii0 + locals.var_vgsstep);
        (assign24540_e22044, (locals.var_vdsatii0_dn3 + locals.var_vgsstep_dn3), (locals.var_vdsatii0_dn4 + locals.var_vgsstep_dn4), (locals.var_vdsatii0_dn5 + locals.var_vgsstep_dn5), (locals.var_vdsatii0_dn6 + locals.var_vgsstep_dn6), (locals.var_vdsatii0_dn7 + locals.var_vgsstep_dn7), (locals.var_vdsatii0_dn8 + locals.var_vgsstep_dn8), (locals.var_vdsatii0_dn9 + locals.var_vgsstep_dn9), (locals.var_vdsatii0_dn10 + locals.var_vgsstep_dn10), (locals.var_vdsatii0_dn11 + locals.var_vgsstep_dn11), (locals.var_vdsatii0_dn12 + locals.var_vgsstep_dn12),)
    } else {
        (locals.var_vdsatii, locals.var_vdsatii_dn3, locals.var_vdsatii_dn4, locals.var_vdsatii_dn5, locals.var_vdsatii_dn6, locals.var_vdsatii_dn7, locals.var_vdsatii_dn8, locals.var_vdsatii_dn9, locals.var_vdsatii_dn10, locals.var_vdsatii_dn11, locals.var_vdsatii_dn12,)
    }
};
        locals.var_vdsatii = assign24540_e22046;
        locals.var_vdsatii_dn3 = assign24540_e22046_d_n3;
        locals.var_vdsatii_dn4 = assign24540_e22046_d_n4;
        locals.var_vdsatii_dn5 = assign24540_e22046_d_n5;
        locals.var_vdsatii_dn6 = assign24540_e22046_d_n6;
        locals.var_vdsatii_dn7 = assign24540_e22046_d_n7;
        locals.var_vdsatii_dn8 = assign24540_e22046_d_n8;
        locals.var_vdsatii_dn9 = assign24540_e22046_d_n9;
        locals.var_vdsatii_dn10 = assign24540_e22046_d_n10;
        locals.var_vdsatii_dn11 = assign24540_e22046_d_n11;
        locals.var_vdsatii_dn12 = assign24540_e22046_d_n12;

        let (assign24550_e22057, assign24550_e22057_d_n3, assign24550_e22057_d_n4, assign24550_e22057_d_n5, assign24550_e22057_d_n6, assign24550_e22057_d_n7, assign24550_e22057_d_n8, assign24550_e22057_d_n9, assign24550_e22057_d_n10, assign24550_e22057_d_n11, assign24550_e22057_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 != 0.0)) && (locals.var_guard1325 == 0.0)) {
        let assign24550_e22055: f64 = (locals.var_vds_1 - locals.var_vdsatii);
        (assign24550_e22055, (-locals.var_vdsatii_dn3), (-locals.var_vdsatii_dn4), (-locals.var_vdsatii_dn5), (-locals.var_vdsatii_dn6), (locals.var_vds_1_dn7 - locals.var_vdsatii_dn7), (locals.var_vds_1_dn8 - locals.var_vdsatii_dn8), (-locals.var_vdsatii_dn9), (-locals.var_vdsatii_dn10), (-locals.var_vdsatii_dn11), (-locals.var_vdsatii_dn12),)
    } else {
        (locals.var_vdiff, locals.var_vdiff_dn3, locals.var_vdiff_dn4, locals.var_vdiff_dn5, locals.var_vdiff_dn6, locals.var_vdiff_dn7, locals.var_vdiff_dn8, locals.var_vdiff_dn9, locals.var_vdiff_dn10, locals.var_vdiff_dn11, locals.var_vdiff_dn12,)
    }
};
        locals.var_vdiff = assign24550_e22057;
        locals.var_vdiff_dn3 = assign24550_e22057_d_n3;
        locals.var_vdiff_dn4 = assign24550_e22057_d_n4;
        locals.var_vdiff_dn5 = assign24550_e22057_d_n5;
        locals.var_vdiff_dn6 = assign24550_e22057_d_n6;
        locals.var_vdiff_dn7 = assign24550_e22057_d_n7;
        locals.var_vdiff_dn8 = assign24550_e22057_d_n8;
        locals.var_vdiff_dn9 = assign24550_e22057_d_n9;
        locals.var_vdiff_dn10 = assign24550_e22057_d_n10;
        locals.var_vdiff_dn11 = assign24550_e22057_d_n11;
        locals.var_vdiff_dn12 = assign24550_e22057_d_n12;

        let (assign24560_e22076, assign24560_e22076_d_n3, assign24560_e22076_d_n4, assign24560_e22076_d_n5, assign24560_e22076_d_n6, assign24560_e22076_d_n7, assign24560_e22076_d_n8, assign24560_e22076_d_n9, assign24560_e22076_d_n10, assign24560_e22076_d_n11, assign24560_e22076_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 != 0.0)) && (locals.var_guard1325 == 0.0)) {
        let assign24560_e22067: f64 = (locals.var_pparam_b4soibeta1 * locals.var_vdiff);
        let assign24560_e22068: f64 = (locals.var_pparam_b4soibeta2 + assign24560_e22067);
        let assign24560_e22071: f64 = (locals.var_pparam_b4soibeta0 * locals.var_vdiff);
        let assign24560_e22073: f64 = (assign24560_e22071 * locals.var_vdiff);
        let assign24560_e22074: f64 = (assign24560_e22068 + assign24560_e22073);
        (assign24560_e22074, ((locals.var_pparam_b4soibeta2_dn3 + ((locals.var_pparam_b4soibeta1_dn3 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn3))) + ((((locals.var_pparam_b4soibeta0_dn3 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn3)) * locals.var_vdiff) + (assign24560_e22071 * locals.var_vdiff_dn3))), ((locals.var_pparam_b4soibeta2_dn4 + ((locals.var_pparam_b4soibeta1_dn4 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn4))) + ((((locals.var_pparam_b4soibeta0_dn4 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn4)) * locals.var_vdiff) + (assign24560_e22071 * locals.var_vdiff_dn4))), ((locals.var_pparam_b4soibeta2_dn5 + ((locals.var_pparam_b4soibeta1_dn5 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn5))) + ((((locals.var_pparam_b4soibeta0_dn5 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn5)) * locals.var_vdiff) + (assign24560_e22071 * locals.var_vdiff_dn5))), ((locals.var_pparam_b4soibeta2_dn6 + ((locals.var_pparam_b4soibeta1_dn6 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn6))) + ((((locals.var_pparam_b4soibeta0_dn6 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn6)) * locals.var_vdiff) + (assign24560_e22071 * locals.var_vdiff_dn6))), ((locals.var_pparam_b4soibeta2_dn7 + ((locals.var_pparam_b4soibeta1_dn7 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn7))) + ((((locals.var_pparam_b4soibeta0_dn7 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn7)) * locals.var_vdiff) + (assign24560_e22071 * locals.var_vdiff_dn7))), ((locals.var_pparam_b4soibeta2_dn8 + ((locals.var_pparam_b4soibeta1_dn8 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn8))) + ((((locals.var_pparam_b4soibeta0_dn8 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn8)) * locals.var_vdiff) + (assign24560_e22071 * locals.var_vdiff_dn8))), ((locals.var_pparam_b4soibeta2_dn9 + ((locals.var_pparam_b4soibeta1_dn9 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn9))) + ((((locals.var_pparam_b4soibeta0_dn9 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn9)) * locals.var_vdiff) + (assign24560_e22071 * locals.var_vdiff_dn9))), ((locals.var_pparam_b4soibeta2_dn10 + ((locals.var_pparam_b4soibeta1_dn10 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn10))) + ((((locals.var_pparam_b4soibeta0_dn10 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn10)) * locals.var_vdiff) + (assign24560_e22071 * locals.var_vdiff_dn10))), ((locals.var_pparam_b4soibeta2_dn11 + ((locals.var_pparam_b4soibeta1_dn11 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn11))) + ((((locals.var_pparam_b4soibeta0_dn11 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn11)) * locals.var_vdiff) + (assign24560_e22071 * locals.var_vdiff_dn11))), ((locals.var_pparam_b4soibeta2_dn12 + ((locals.var_pparam_b4soibeta1_dn12 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn12))) + ((((locals.var_pparam_b4soibeta0_dn12 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn12)) * locals.var_vdiff) + (assign24560_e22071 * locals.var_vdiff_dn12))),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign24560_e22076;
        locals.var_t0__blk808_dn3 = assign24560_e22076_d_n3;
        locals.var_t0__blk808_dn4 = assign24560_e22076_d_n4;
        locals.var_t0__blk808_dn5 = assign24560_e22076_d_n5;
        locals.var_t0__blk808_dn6 = assign24560_e22076_d_n6;
        locals.var_t0__blk808_dn7 = assign24560_e22076_d_n7;
        locals.var_t0__blk808_dn8 = assign24560_e22076_d_n8;
        locals.var_t0__blk808_dn9 = assign24560_e22076_d_n9;
        locals.var_t0__blk808_dn10 = assign24560_e22076_d_n10;
        locals.var_t0__blk808_dn11 = assign24560_e22076_d_n11;
        locals.var_t0__blk808_dn12 = assign24560_e22076_d_n12;

        let assign24570_e22079: f64 = if locals.var_t0__blk808 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1326 = assign24570_e22079;

        let (assign24580_e22090, assign24580_e22090_d_n3, assign24580_e22090_d_n4, assign24580_e22090_d_n5, assign24580_e22090_d_n6, assign24580_e22090_d_n7, assign24580_e22090_d_n8, assign24580_e22090_d_n9, assign24580_e22090_d_n10, assign24580_e22090_d_n11, assign24580_e22090_d_n12,) = {
    if ((((locals.var_guard1323 != 0.0) && (locals.var_guard1324 != 0.0)) && (locals.var_guard1325 == 0.0)) && (locals.var_guard1326 != 0.0)) {
        (1e-5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign24580_e22090;
        locals.var_t0__blk808_dn3 = assign24580_e22090_d_n3;
        locals.var_t0__blk808_dn4 = assign24580_e22090_d_n4;
        locals.var_t0__blk808_dn5 = assign24580_e22090_d_n5;
        locals.var_t0__blk808_dn6 = assign24580_e22090_d_n6;
        locals.var_t0__blk808_dn7 = assign24580_e22090_d_n7;
        locals.var_t0__blk808_dn8 = assign24580_e22090_d_n8;
        locals.var_t0__blk808_dn9 = assign24580_e22090_d_n9;
        locals.var_t0__blk808_dn10 = assign24580_e22090_d_n10;
        locals.var_t0__blk808_dn11 = assign24580_e22090_d_n11;
        locals.var_t0__blk808_dn12 = assign24580_e22090_d_n12;

        let assign24590_e22094: f64 = (locals.var_vdiff / 100.0);
        let assign24590_e22099: f64 = if ((locals.var_t0__blk808 < assign24590_e22094) && (locals.var_vdiff > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1327 = assign24590_e22099;

        let (assign24600_e22112, assign24600_e22112_d_n3, assign24600_e22112_d_n4, assign24600_e22112_d_n5, assign24600_e22112_d_n6, assign24600_e22112_d_n7, assign24600_e22112_d_n8, assign24600_e22112_d_n9, assign24600_e22112_d_n10, assign24600_e22112_d_n11, assign24600_e22112_d_n12,) = {
    if ((((locals.var_guard1323 != 0.0) && (locals.var_guard1324 != 0.0)) && (locals.var_guard1325 == 0.0)) && (locals.var_guard1327 != 0.0)) {
        let assign24600_e22110: f64 = (locals.var_pparam_b4soialpha0 * 2.688117142e43);
        (assign24600_e22110, (locals.var_pparam_b4soialpha0_dn3 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn4 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn5 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn6 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn7 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn8 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn9 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn10 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn11 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn12 * 2.688117142e43),)
    } else {
        (locals.var_ratio, locals.var_ratio_dn3, locals.var_ratio_dn4, locals.var_ratio_dn5, locals.var_ratio_dn6, locals.var_ratio_dn7, locals.var_ratio_dn8, locals.var_ratio_dn9, locals.var_ratio_dn10, locals.var_ratio_dn11, locals.var_ratio_dn12,)
    }
};
        locals.var_ratio = assign24600_e22112;
        locals.var_ratio_dn3 = assign24600_e22112_d_n3;
        locals.var_ratio_dn4 = assign24600_e22112_d_n4;
        locals.var_ratio_dn5 = assign24600_e22112_d_n5;
        locals.var_ratio_dn6 = assign24600_e22112_d_n6;
        locals.var_ratio_dn7 = assign24600_e22112_d_n7;
        locals.var_ratio_dn8 = assign24600_e22112_d_n8;
        locals.var_ratio_dn9 = assign24600_e22112_d_n9;
        locals.var_ratio_dn10 = assign24600_e22112_d_n10;
        locals.var_ratio_dn11 = assign24600_e22112_d_n11;
        locals.var_ratio_dn12 = assign24600_e22112_d_n12;

        let assign24610_e22115: f64 = (-locals.var_vdiff);
        let assign24610_e22117: f64 = (assign24610_e22115 / 100.0);
        let assign24610_e22122: f64 = if ((locals.var_t0__blk808 < assign24610_e22117) && (locals.var_vdiff < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1328 = assign24610_e22122;

        let (assign24620_e22138, assign24620_e22138_d_n3, assign24620_e22138_d_n4, assign24620_e22138_d_n5, assign24620_e22138_d_n6, assign24620_e22138_d_n7, assign24620_e22138_d_n8, assign24620_e22138_d_n9, assign24620_e22138_d_n10, assign24620_e22138_d_n11, assign24620_e22138_d_n12,) = {
    if (((((locals.var_guard1323 != 0.0) && (locals.var_guard1324 != 0.0)) && (locals.var_guard1325 == 0.0)) && (locals.var_guard1327 == 0.0)) && (locals.var_guard1328 != 0.0)) {
        let assign24620_e22136: f64 = (locals.var_pparam_b4soialpha0 * 3.720075976e-44);
        (assign24620_e22136, (locals.var_pparam_b4soialpha0_dn3 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn4 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn5 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn6 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn7 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn8 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn9 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn10 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn11 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn12 * 3.720075976e-44),)
    } else {
        (locals.var_ratio, locals.var_ratio_dn3, locals.var_ratio_dn4, locals.var_ratio_dn5, locals.var_ratio_dn6, locals.var_ratio_dn7, locals.var_ratio_dn8, locals.var_ratio_dn9, locals.var_ratio_dn10, locals.var_ratio_dn11, locals.var_ratio_dn12,)
    }
};
        locals.var_ratio = assign24620_e22138;
        locals.var_ratio_dn3 = assign24620_e22138_d_n3;
        locals.var_ratio_dn4 = assign24620_e22138_d_n4;
        locals.var_ratio_dn5 = assign24620_e22138_d_n5;
        locals.var_ratio_dn6 = assign24620_e22138_d_n6;
        locals.var_ratio_dn7 = assign24620_e22138_d_n7;
        locals.var_ratio_dn8 = assign24620_e22138_d_n8;
        locals.var_ratio_dn9 = assign24620_e22138_d_n9;
        locals.var_ratio_dn10 = assign24620_e22138_d_n10;
        locals.var_ratio_dn11 = assign24620_e22138_d_n11;
        locals.var_ratio_dn12 = assign24620_e22138_d_n12;

        let (assign24630_e22158, assign24630_e22158_d_n3, assign24630_e22158_d_n4, assign24630_e22158_d_n5, assign24630_e22158_d_n6, assign24630_e22158_d_n7, assign24630_e22158_d_n8, assign24630_e22158_d_n9, assign24630_e22158_d_n10, assign24630_e22158_d_n11, assign24630_e22158_d_n12,) = {
    if (((((locals.var_guard1323 != 0.0) && (locals.var_guard1324 != 0.0)) && (locals.var_guard1325 == 0.0)) && (locals.var_guard1327 == 0.0)) && (locals.var_guard1328 == 0.0)) {
        let assign24630_e22154: f64 = (locals.var_vdiff / locals.var_t0__blk808);
        let assign24630_e22155: f64 = (assign24630_e22154).exp();
        let assign24630_e22156: f64 = (locals.var_pparam_b4soialpha0 * assign24630_e22155);
        (assign24630_e22156, ((locals.var_pparam_b4soialpha0_dn3 * assign24630_e22155) + (locals.var_pparam_b4soialpha0 * (assign24630_e22155 * (((locals.var_vdiff_dn3 * locals.var_t0__blk808) - (locals.var_vdiff * locals.var_t0__blk808_dn3)) / (locals.var_t0__blk808 * locals.var_t0__blk808))))), ((locals.var_pparam_b4soialpha0_dn4 * assign24630_e22155) + (locals.var_pparam_b4soialpha0 * (assign24630_e22155 * (((locals.var_vdiff_dn4 * locals.var_t0__blk808) - (locals.var_vdiff * locals.var_t0__blk808_dn4)) / (locals.var_t0__blk808 * locals.var_t0__blk808))))), ((locals.var_pparam_b4soialpha0_dn5 * assign24630_e22155) + (locals.var_pparam_b4soialpha0 * (assign24630_e22155 * (((locals.var_vdiff_dn5 * locals.var_t0__blk808) - (locals.var_vdiff * locals.var_t0__blk808_dn5)) / (locals.var_t0__blk808 * locals.var_t0__blk808))))), ((locals.var_pparam_b4soialpha0_dn6 * assign24630_e22155) + (locals.var_pparam_b4soialpha0 * (assign24630_e22155 * (((locals.var_vdiff_dn6 * locals.var_t0__blk808) - (locals.var_vdiff * locals.var_t0__blk808_dn6)) / (locals.var_t0__blk808 * locals.var_t0__blk808))))), ((locals.var_pparam_b4soialpha0_dn7 * assign24630_e22155) + (locals.var_pparam_b4soialpha0 * (assign24630_e22155 * (((locals.var_vdiff_dn7 * locals.var_t0__blk808) - (locals.var_vdiff * locals.var_t0__blk808_dn7)) / (locals.var_t0__blk808 * locals.var_t0__blk808))))), ((locals.var_pparam_b4soialpha0_dn8 * assign24630_e22155) + (locals.var_pparam_b4soialpha0 * (assign24630_e22155 * (((locals.var_vdiff_dn8 * locals.var_t0__blk808) - (locals.var_vdiff * locals.var_t0__blk808_dn8)) / (locals.var_t0__blk808 * locals.var_t0__blk808))))), ((locals.var_pparam_b4soialpha0_dn9 * assign24630_e22155) + (locals.var_pparam_b4soialpha0 * (assign24630_e22155 * (((locals.var_vdiff_dn9 * locals.var_t0__blk808) - (locals.var_vdiff * locals.var_t0__blk808_dn9)) / (locals.var_t0__blk808 * locals.var_t0__blk808))))), ((locals.var_pparam_b4soialpha0_dn10 * assign24630_e22155) + (locals.var_pparam_b4soialpha0 * (assign24630_e22155 * (((locals.var_vdiff_dn10 * locals.var_t0__blk808) - (locals.var_vdiff * locals.var_t0__blk808_dn10)) / (locals.var_t0__blk808 * locals.var_t0__blk808))))), ((locals.var_pparam_b4soialpha0_dn11 * assign24630_e22155) + (locals.var_pparam_b4soialpha0 * (assign24630_e22155 * (((locals.var_vdiff_dn11 * locals.var_t0__blk808) - (locals.var_vdiff * locals.var_t0__blk808_dn11)) / (locals.var_t0__blk808 * locals.var_t0__blk808))))), ((locals.var_pparam_b4soialpha0_dn12 * assign24630_e22155) + (locals.var_pparam_b4soialpha0 * (assign24630_e22155 * (((locals.var_vdiff_dn12 * locals.var_t0__blk808) - (locals.var_vdiff * locals.var_t0__blk808_dn12)) / (locals.var_t0__blk808 * locals.var_t0__blk808))))),)
    } else {
        (locals.var_ratio, locals.var_ratio_dn3, locals.var_ratio_dn4, locals.var_ratio_dn5, locals.var_ratio_dn6, locals.var_ratio_dn7, locals.var_ratio_dn8, locals.var_ratio_dn9, locals.var_ratio_dn10, locals.var_ratio_dn11, locals.var_ratio_dn12,)
    }
};
        locals.var_ratio = assign24630_e22158;
        locals.var_ratio_dn3 = assign24630_e22158_d_n3;
        locals.var_ratio_dn4 = assign24630_e22158_d_n4;
        locals.var_ratio_dn5 = assign24630_e22158_d_n5;
        locals.var_ratio_dn6 = assign24630_e22158_d_n6;
        locals.var_ratio_dn7 = assign24630_e22158_d_n7;
        locals.var_ratio_dn8 = assign24630_e22158_d_n8;
        locals.var_ratio_dn9 = assign24630_e22158_d_n9;
        locals.var_ratio_dn10 = assign24630_e22158_d_n10;
        locals.var_ratio_dn11 = assign24630_e22158_d_n11;
        locals.var_ratio_dn12 = assign24630_e22158_d_n12;

        let assign24640_e22161: f64 = if locals.var_ratio > 10.0 { 1.0 } else { 0.0 };
        locals.var_guard1329 = assign24640_e22161;

        let (assign24650_e22172, assign24650_e22172_d_n3, assign24650_e22172_d_n4, assign24650_e22172_d_n5, assign24650_e22172_d_n6, assign24650_e22172_d_n7, assign24650_e22172_d_n8, assign24650_e22172_d_n9, assign24650_e22172_d_n10, assign24650_e22172_d_n11, assign24650_e22172_d_n12,) = {
    if ((((locals.var_guard1323 != 0.0) && (locals.var_guard1324 != 0.0)) && (locals.var_guard1325 == 0.0)) && (locals.var_guard1329 != 0.0)) {
        (10.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ratio, locals.var_ratio_dn3, locals.var_ratio_dn4, locals.var_ratio_dn5, locals.var_ratio_dn6, locals.var_ratio_dn7, locals.var_ratio_dn8, locals.var_ratio_dn9, locals.var_ratio_dn10, locals.var_ratio_dn11, locals.var_ratio_dn12,)
    }
};
        locals.var_ratio = assign24650_e22172;
        locals.var_ratio_dn3 = assign24650_e22172_d_n3;
        locals.var_ratio_dn4 = assign24650_e22172_d_n4;
        locals.var_ratio_dn5 = assign24650_e22172_d_n5;
        locals.var_ratio_dn6 = assign24650_e22172_d_n6;
        locals.var_ratio_dn7 = assign24650_e22172_d_n7;
        locals.var_ratio_dn8 = assign24650_e22172_d_n8;
        locals.var_ratio_dn9 = assign24650_e22172_d_n9;
        locals.var_ratio_dn10 = assign24650_e22172_d_n10;
        locals.var_ratio_dn11 = assign24650_e22172_d_n11;
        locals.var_ratio_dn12 = assign24650_e22172_d_n12;

        let (assign24660_e22187, assign24660_e22187_d_n3, assign24660_e22187_d_n4, assign24660_e22187_d_n5, assign24660_e22187_d_n6, assign24660_e22187_d_n7, assign24660_e22187_d_n8, assign24660_e22187_d_n9, assign24660_e22187_d_n10, assign24660_e22187_d_n11, assign24660_e22187_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 != 0.0)) && (locals.var_guard1325 == 0.0)) {
        let assign24660_e22182: f64 = (locals.var_pparam_b4soifbjtii * locals.var_b4soimode);
        let assign24660_e22184: f64 = (assign24660_e22182 * locals.var_ic_1);
        let assign24660_e22185: f64 = (locals.var_ids_1 + assign24660_e22184);
        (assign24660_e22185, (locals.var_ids_1_dn3 + (((locals.var_pparam_b4soifbjtii_dn3 * locals.var_b4soimode) * locals.var_ic_1) + (assign24660_e22182 * locals.var_ic_1_dn3))), (locals.var_ids_1_dn4 + (((locals.var_pparam_b4soifbjtii_dn4 * locals.var_b4soimode) * locals.var_ic_1) + (assign24660_e22182 * locals.var_ic_1_dn4))), (locals.var_ids_1_dn5 + (((locals.var_pparam_b4soifbjtii_dn5 * locals.var_b4soimode) * locals.var_ic_1) + (assign24660_e22182 * locals.var_ic_1_dn5))), (locals.var_ids_1_dn6 + (((locals.var_pparam_b4soifbjtii_dn6 * locals.var_b4soimode) * locals.var_ic_1) + (assign24660_e22182 * locals.var_ic_1_dn6))), (locals.var_ids_1_dn7 + (((locals.var_pparam_b4soifbjtii_dn7 * locals.var_b4soimode) * locals.var_ic_1) + (assign24660_e22182 * locals.var_ic_1_dn7))), (locals.var_ids_1_dn8 + (((locals.var_pparam_b4soifbjtii_dn8 * locals.var_b4soimode) * locals.var_ic_1) + (assign24660_e22182 * locals.var_ic_1_dn8))), (locals.var_ids_1_dn9 + (((locals.var_pparam_b4soifbjtii_dn9 * locals.var_b4soimode) * locals.var_ic_1) + (assign24660_e22182 * locals.var_ic_1_dn9))), (locals.var_ids_1_dn10 + (((locals.var_pparam_b4soifbjtii_dn10 * locals.var_b4soimode) * locals.var_ic_1) + (assign24660_e22182 * locals.var_ic_1_dn10))), (locals.var_ids_1_dn11 + (((locals.var_pparam_b4soifbjtii_dn11 * locals.var_b4soimode) * locals.var_ic_1) + (assign24660_e22182 * locals.var_ic_1_dn11))), (locals.var_ids_1_dn12 + (((locals.var_pparam_b4soifbjtii_dn12 * locals.var_b4soimode) * locals.var_ic_1) + (assign24660_e22182 * locals.var_ic_1_dn12))),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign24660_e22187;
        locals.var_t0__blk808_dn3 = assign24660_e22187_d_n3;
        locals.var_t0__blk808_dn4 = assign24660_e22187_d_n4;
        locals.var_t0__blk808_dn5 = assign24660_e22187_d_n5;
        locals.var_t0__blk808_dn6 = assign24660_e22187_d_n6;
        locals.var_t0__blk808_dn7 = assign24660_e22187_d_n7;
        locals.var_t0__blk808_dn8 = assign24660_e22187_d_n8;
        locals.var_t0__blk808_dn9 = assign24660_e22187_d_n9;
        locals.var_t0__blk808_dn10 = assign24660_e22187_d_n10;
        locals.var_t0__blk808_dn11 = assign24660_e22187_d_n11;
        locals.var_t0__blk808_dn12 = assign24660_e22187_d_n12;

        let (assign24670_e22198, assign24670_e22198_d_n3, assign24670_e22198_d_n4, assign24670_e22198_d_n5, assign24670_e22198_d_n6, assign24670_e22198_d_n7, assign24670_e22198_d_n8, assign24670_e22198_d_n9, assign24670_e22198_d_n10, assign24670_e22198_d_n11, assign24670_e22198_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 != 0.0)) && (locals.var_guard1325 == 0.0)) {
        let assign24670_e22196: f64 = (locals.var_ratio * locals.var_t0__blk808);
        (assign24670_e22196, ((locals.var_ratio_dn3 * locals.var_t0__blk808) + (locals.var_ratio * locals.var_t0__blk808_dn3)), ((locals.var_ratio_dn4 * locals.var_t0__blk808) + (locals.var_ratio * locals.var_t0__blk808_dn4)), ((locals.var_ratio_dn5 * locals.var_t0__blk808) + (locals.var_ratio * locals.var_t0__blk808_dn5)), ((locals.var_ratio_dn6 * locals.var_t0__blk808) + (locals.var_ratio * locals.var_t0__blk808_dn6)), ((locals.var_ratio_dn7 * locals.var_t0__blk808) + (locals.var_ratio * locals.var_t0__blk808_dn7)), ((locals.var_ratio_dn8 * locals.var_t0__blk808) + (locals.var_ratio * locals.var_t0__blk808_dn8)), ((locals.var_ratio_dn9 * locals.var_t0__blk808) + (locals.var_ratio * locals.var_t0__blk808_dn9)), ((locals.var_ratio_dn10 * locals.var_t0__blk808) + (locals.var_ratio * locals.var_t0__blk808_dn10)), ((locals.var_ratio_dn11 * locals.var_t0__blk808) + (locals.var_ratio * locals.var_t0__blk808_dn11)), ((locals.var_ratio_dn12 * locals.var_t0__blk808) + (locals.var_ratio * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_iii, locals.var_iii_dn3, locals.var_iii_dn4, locals.var_iii_dn5, locals.var_iii_dn6, locals.var_iii_dn7, locals.var_iii_dn8, locals.var_iii_dn9, locals.var_iii_dn10, locals.var_iii_dn11, locals.var_iii_dn12,)
    }
};
        locals.var_iii = assign24670_e22198;
        locals.var_iii_dn3 = assign24670_e22198_d_n3;
        locals.var_iii_dn4 = assign24670_e22198_d_n4;
        locals.var_iii_dn5 = assign24670_e22198_d_n5;
        locals.var_iii_dn6 = assign24670_e22198_d_n6;
        locals.var_iii_dn7 = assign24670_e22198_d_n7;
        locals.var_iii_dn8 = assign24670_e22198_d_n8;
        locals.var_iii_dn9 = assign24670_e22198_d_n9;
        locals.var_iii_dn10 = assign24670_e22198_d_n10;
        locals.var_iii_dn11 = assign24670_e22198_d_n11;
        locals.var_iii_dn12 = assign24670_e22198_d_n12;

        let assign24680_e22201: f64 = if locals.var_pparam_b4soialpha0 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1330 = assign24680_e22201;

        let (assign24690_e22210, assign24690_e22210_d_n3, assign24690_e22210_d_n4, assign24690_e22210_d_n5, assign24690_e22210_d_n6, assign24690_e22210_d_n7, assign24690_e22210_d_n8, assign24690_e22210_d_n9, assign24690_e22210_d_n10, assign24690_e22210_d_n11, assign24690_e22210_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1330 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_idsmosfet, locals.var_idsmosfet_dn3, locals.var_idsmosfet_dn4, locals.var_idsmosfet_dn5, locals.var_idsmosfet_dn6, locals.var_idsmosfet_dn7, locals.var_idsmosfet_dn8, locals.var_idsmosfet_dn9, locals.var_idsmosfet_dn10, locals.var_idsmosfet_dn11, locals.var_idsmosfet_dn12,)
    }
};
        locals.var_idsmosfet = assign24690_e22210;
        locals.var_idsmosfet_dn3 = assign24690_e22210_d_n3;
        locals.var_idsmosfet_dn4 = assign24690_e22210_d_n4;
        locals.var_idsmosfet_dn5 = assign24690_e22210_d_n5;
        locals.var_idsmosfet_dn6 = assign24690_e22210_d_n6;
        locals.var_idsmosfet_dn7 = assign24690_e22210_d_n7;
        locals.var_idsmosfet_dn8 = assign24690_e22210_d_n8;
        locals.var_idsmosfet_dn9 = assign24690_e22210_d_n9;
        locals.var_idsmosfet_dn10 = assign24690_e22210_d_n10;
        locals.var_idsmosfet_dn11 = assign24690_e22210_d_n11;
        locals.var_idsmosfet_dn12 = assign24690_e22210_d_n12;

        let (assign24700_e22230, assign24700_e22230_d_n3, assign24700_e22230_d_n4, assign24700_e22230_d_n5, assign24700_e22230_d_n6, assign24700_e22230_d_n7, assign24700_e22230_d_n8, assign24700_e22230_d_n9, assign24700_e22230_d_n10, assign24700_e22230_d_n11, assign24700_e22230_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1330 == 0.0)) {
        let assign24700_e22222: f64 = (p.p308 * locals.var_trm1);
        let assign24700_e22223: f64 = (1.0 + assign24700_e22222);
        let assign24700_e22224: f64 = (locals.var_pparam_b4soivdsatii0 * assign24700_e22223);
        let assign24700_e22227: f64 = (locals.var_pparam_b4soilii / locals.var_leff);
        let assign24700_e22228: f64 = (assign24700_e22224 - assign24700_e22227);
        (assign24700_e22228, ((locals.var_pparam_b4soivdsatii0_dn3 * assign24700_e22223) - (((locals.var_pparam_b4soilii_dn3 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn3)) / (locals.var_leff * locals.var_leff))), (((locals.var_pparam_b4soivdsatii0_dn4 * assign24700_e22223) + (locals.var_pparam_b4soivdsatii0 * (p.p308 * locals.var_trm1_dn4))) - (((locals.var_pparam_b4soilii_dn4 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn4)) / (locals.var_leff * locals.var_leff))), (((locals.var_pparam_b4soivdsatii0_dn5 * assign24700_e22223) + (locals.var_pparam_b4soivdsatii0 * (p.p308 * locals.var_trm1_dn5))) - (((locals.var_pparam_b4soilii_dn5 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn5)) / (locals.var_leff * locals.var_leff))), (((locals.var_pparam_b4soivdsatii0_dn6 * assign24700_e22223) + (locals.var_pparam_b4soivdsatii0 * (p.p308 * locals.var_trm1_dn6))) - (((locals.var_pparam_b4soilii_dn6 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn6)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn7 * assign24700_e22223) - (((locals.var_pparam_b4soilii_dn7 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn7)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn8 * assign24700_e22223) - (((locals.var_pparam_b4soilii_dn8 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn8)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn9 * assign24700_e22223) - (((locals.var_pparam_b4soilii_dn9 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn9)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn10 * assign24700_e22223) - (((locals.var_pparam_b4soilii_dn10 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn10)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn11 * assign24700_e22223) - (((locals.var_pparam_b4soilii_dn11 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn11)) / (locals.var_leff * locals.var_leff))), ((locals.var_pparam_b4soivdsatii0_dn12 * assign24700_e22223) - (((locals.var_pparam_b4soilii_dn12 * locals.var_leff) - (locals.var_pparam_b4soilii * locals.var_leff_dn12)) / (locals.var_leff * locals.var_leff))),)
    } else {
        (locals.var_vdsatii0, locals.var_vdsatii0_dn3, locals.var_vdsatii0_dn4, locals.var_vdsatii0_dn5, locals.var_vdsatii0_dn6, locals.var_vdsatii0_dn7, locals.var_vdsatii0_dn8, locals.var_vdsatii0_dn9, locals.var_vdsatii0_dn10, locals.var_vdsatii0_dn11, locals.var_vdsatii0_dn12,)
    }
};
        locals.var_vdsatii0 = assign24700_e22230;
        locals.var_vdsatii0_dn3 = assign24700_e22230_d_n3;
        locals.var_vdsatii0_dn4 = assign24700_e22230_d_n4;
        locals.var_vdsatii0_dn5 = assign24700_e22230_d_n5;
        locals.var_vdsatii0_dn6 = assign24700_e22230_d_n6;
        locals.var_vdsatii0_dn7 = assign24700_e22230_d_n7;
        locals.var_vdsatii0_dn8 = assign24700_e22230_d_n8;
        locals.var_vdsatii0_dn9 = assign24700_e22230_d_n9;
        locals.var_vdsatii0_dn10 = assign24700_e22230_d_n10;
        locals.var_vdsatii0_dn11 = assign24700_e22230_d_n11;
        locals.var_vdsatii0_dn12 = assign24700_e22230_d_n12;

        let (assign24710_e22242, assign24710_e22242_d_n3, assign24710_e22242_d_n4, assign24710_e22242_d_n5, assign24710_e22242_d_n6, assign24710_e22242_d_n7, assign24710_e22242_d_n8, assign24710_e22242_d_n9, assign24710_e22242_d_n10, assign24710_e22242_d_n11, assign24710_e22242_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1330 == 0.0)) {
        let assign24710_e22240: f64 = (locals.var_pparam_b4soiesatii * locals.var_leff);
        (assign24710_e22240, ((locals.var_pparam_b4soiesatii_dn3 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn3)), ((locals.var_pparam_b4soiesatii_dn4 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn4)), ((locals.var_pparam_b4soiesatii_dn5 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn5)), ((locals.var_pparam_b4soiesatii_dn6 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn6)), ((locals.var_pparam_b4soiesatii_dn7 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn7)), ((locals.var_pparam_b4soiesatii_dn8 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn8)), ((locals.var_pparam_b4soiesatii_dn9 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn9)), ((locals.var_pparam_b4soiesatii_dn10 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn10)), ((locals.var_pparam_b4soiesatii_dn11 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn11)), ((locals.var_pparam_b4soiesatii_dn12 * locals.var_leff) + (locals.var_pparam_b4soiesatii * locals.var_leff_dn12)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign24710_e22242;
        locals.var_t0__blk808_dn3 = assign24710_e22242_d_n3;
        locals.var_t0__blk808_dn4 = assign24710_e22242_d_n4;
        locals.var_t0__blk808_dn5 = assign24710_e22242_d_n5;
        locals.var_t0__blk808_dn6 = assign24710_e22242_d_n6;
        locals.var_t0__blk808_dn7 = assign24710_e22242_d_n7;
        locals.var_t0__blk808_dn8 = assign24710_e22242_d_n8;
        locals.var_t0__blk808_dn9 = assign24710_e22242_d_n9;
        locals.var_t0__blk808_dn10 = assign24710_e22242_d_n10;
        locals.var_t0__blk808_dn11 = assign24710_e22242_d_n11;
        locals.var_t0__blk808_dn12 = assign24710_e22242_d_n12;

        let (assign24720_e22258, assign24720_e22258_d_n3, assign24720_e22258_d_n4, assign24720_e22258_d_n5, assign24720_e22258_d_n6, assign24720_e22258_d_n7, assign24720_e22258_d_n8, assign24720_e22258_d_n9, assign24720_e22258_d_n10, assign24720_e22258_d_n11, assign24720_e22258_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1330 == 0.0)) {
        let assign24720_e22252: f64 = (locals.var_pparam_b4soisii0 * locals.var_t0__blk808);
        let assign24720_e22255: f64 = (1.0 + locals.var_t0__blk808);
        let assign24720_e22256: f64 = (assign24720_e22252 / assign24720_e22255);
        (assign24720_e22256, (((((locals.var_pparam_b4soisii0_dn3 * locals.var_t0__blk808) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk808_dn3)) * assign24720_e22255) - (assign24720_e22252 * locals.var_t0__blk808_dn3)) / (assign24720_e22255 * assign24720_e22255)), (((((locals.var_pparam_b4soisii0_dn4 * locals.var_t0__blk808) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk808_dn4)) * assign24720_e22255) - (assign24720_e22252 * locals.var_t0__blk808_dn4)) / (assign24720_e22255 * assign24720_e22255)), (((((locals.var_pparam_b4soisii0_dn5 * locals.var_t0__blk808) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk808_dn5)) * assign24720_e22255) - (assign24720_e22252 * locals.var_t0__blk808_dn5)) / (assign24720_e22255 * assign24720_e22255)), (((((locals.var_pparam_b4soisii0_dn6 * locals.var_t0__blk808) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk808_dn6)) * assign24720_e22255) - (assign24720_e22252 * locals.var_t0__blk808_dn6)) / (assign24720_e22255 * assign24720_e22255)), (((((locals.var_pparam_b4soisii0_dn7 * locals.var_t0__blk808) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk808_dn7)) * assign24720_e22255) - (assign24720_e22252 * locals.var_t0__blk808_dn7)) / (assign24720_e22255 * assign24720_e22255)), (((((locals.var_pparam_b4soisii0_dn8 * locals.var_t0__blk808) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk808_dn8)) * assign24720_e22255) - (assign24720_e22252 * locals.var_t0__blk808_dn8)) / (assign24720_e22255 * assign24720_e22255)), (((((locals.var_pparam_b4soisii0_dn9 * locals.var_t0__blk808) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk808_dn9)) * assign24720_e22255) - (assign24720_e22252 * locals.var_t0__blk808_dn9)) / (assign24720_e22255 * assign24720_e22255)), (((((locals.var_pparam_b4soisii0_dn10 * locals.var_t0__blk808) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk808_dn10)) * assign24720_e22255) - (assign24720_e22252 * locals.var_t0__blk808_dn10)) / (assign24720_e22255 * assign24720_e22255)), (((((locals.var_pparam_b4soisii0_dn11 * locals.var_t0__blk808) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk808_dn11)) * assign24720_e22255) - (assign24720_e22252 * locals.var_t0__blk808_dn11)) / (assign24720_e22255 * assign24720_e22255)), (((((locals.var_pparam_b4soisii0_dn12 * locals.var_t0__blk808) + (locals.var_pparam_b4soisii0 * locals.var_t0__blk808_dn12)) * assign24720_e22255) - (assign24720_e22252 * locals.var_t0__blk808_dn12)) / (assign24720_e22255 * assign24720_e22255)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign24720_e22258;
        locals.var_t1__blk809_dn3 = assign24720_e22258_d_n3;
        locals.var_t1__blk809_dn4 = assign24720_e22258_d_n4;
        locals.var_t1__blk809_dn5 = assign24720_e22258_d_n5;
        locals.var_t1__blk809_dn6 = assign24720_e22258_d_n6;
        locals.var_t1__blk809_dn7 = assign24720_e22258_d_n7;
        locals.var_t1__blk809_dn8 = assign24720_e22258_d_n8;
        locals.var_t1__blk809_dn9 = assign24720_e22258_d_n9;
        locals.var_t1__blk809_dn10 = assign24720_e22258_d_n10;
        locals.var_t1__blk809_dn11 = assign24720_e22258_d_n11;
        locals.var_t1__blk809_dn12 = assign24720_e22258_d_n12;

        let (assign24730_e22274, assign24730_e22274_d_n3, assign24730_e22274_d_n4, assign24730_e22274_d_n5, assign24730_e22274_d_n6, assign24730_e22274_d_n7, assign24730_e22274_d_n8, assign24730_e22274_d_n9, assign24730_e22274_d_n10, assign24730_e22274_d_n11, assign24730_e22274_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1330 == 0.0)) {
        let assign24730_e22270: f64 = (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840);
        let assign24730_e22271: f64 = (1.0 + assign24730_e22270);
        let assign24730_e22272: f64 = (1.0 / assign24730_e22271);
        (assign24730_e22272, (-(((locals.var_pparam_b4soisii1_dn3 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840_dn3)) / (assign24730_e22271 * assign24730_e22271))), (-(((locals.var_pparam_b4soisii1_dn4 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840_dn4)) / (assign24730_e22271 * assign24730_e22271))), (-(((locals.var_pparam_b4soisii1_dn5 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840_dn5)) / (assign24730_e22271 * assign24730_e22271))), (-(((locals.var_pparam_b4soisii1_dn6 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840_dn6)) / (assign24730_e22271 * assign24730_e22271))), (-(((locals.var_pparam_b4soisii1_dn7 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840_dn7)) / (assign24730_e22271 * assign24730_e22271))), (-(((locals.var_pparam_b4soisii1_dn8 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840_dn8)) / (assign24730_e22271 * assign24730_e22271))), (-(((locals.var_pparam_b4soisii1_dn9 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840_dn9)) / (assign24730_e22271 * assign24730_e22271))), (-(((locals.var_pparam_b4soisii1_dn10 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840_dn10)) / (assign24730_e22271 * assign24730_e22271))), (-(((locals.var_pparam_b4soisii1_dn11 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840_dn11)) / (assign24730_e22271 * assign24730_e22271))), (-(((locals.var_pparam_b4soisii1_dn12 * locals.var_vgsteff__blk840) + (locals.var_pparam_b4soisii1 * locals.var_vgsteff__blk840_dn12)) / (assign24730_e22271 * assign24730_e22271))),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign24730_e22274;
        locals.var_t0__blk808_dn3 = assign24730_e22274_d_n3;
        locals.var_t0__blk808_dn4 = assign24730_e22274_d_n4;
        locals.var_t0__blk808_dn5 = assign24730_e22274_d_n5;
        locals.var_t0__blk808_dn6 = assign24730_e22274_d_n6;
        locals.var_t0__blk808_dn7 = assign24730_e22274_d_n7;
        locals.var_t0__blk808_dn8 = assign24730_e22274_d_n8;
        locals.var_t0__blk808_dn9 = assign24730_e22274_d_n9;
        locals.var_t0__blk808_dn10 = assign24730_e22274_d_n10;
        locals.var_t0__blk808_dn11 = assign24730_e22274_d_n11;
        locals.var_t0__blk808_dn12 = assign24730_e22274_d_n12;

        let (assign24740_e22286, assign24740_e22286_d_n3, assign24740_e22286_d_n4, assign24740_e22286_d_n5, assign24740_e22286_d_n6, assign24740_e22286_d_n7, assign24740_e22286_d_n8, assign24740_e22286_d_n9, assign24740_e22286_d_n10, assign24740_e22286_d_n11, assign24740_e22286_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1330 == 0.0)) {
        let assign24740_e22284: f64 = (locals.var_t0__blk808 + locals.var_pparam_b4soisii2);
        (assign24740_e22284, (locals.var_t0__blk808_dn3 + locals.var_pparam_b4soisii2_dn3), (locals.var_t0__blk808_dn4 + locals.var_pparam_b4soisii2_dn4), (locals.var_t0__blk808_dn5 + locals.var_pparam_b4soisii2_dn5), (locals.var_t0__blk808_dn6 + locals.var_pparam_b4soisii2_dn6), (locals.var_t0__blk808_dn7 + locals.var_pparam_b4soisii2_dn7), (locals.var_t0__blk808_dn8 + locals.var_pparam_b4soisii2_dn8), (locals.var_t0__blk808_dn9 + locals.var_pparam_b4soisii2_dn9), (locals.var_t0__blk808_dn10 + locals.var_pparam_b4soisii2_dn10), (locals.var_t0__blk808_dn11 + locals.var_pparam_b4soisii2_dn11), (locals.var_t0__blk808_dn12 + locals.var_pparam_b4soisii2_dn12),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign24740_e22286;
        locals.var_t3__blk811_dn3 = assign24740_e22286_d_n3;
        locals.var_t3__blk811_dn4 = assign24740_e22286_d_n4;
        locals.var_t3__blk811_dn5 = assign24740_e22286_d_n5;
        locals.var_t3__blk811_dn6 = assign24740_e22286_d_n6;
        locals.var_t3__blk811_dn7 = assign24740_e22286_d_n7;
        locals.var_t3__blk811_dn8 = assign24740_e22286_d_n8;
        locals.var_t3__blk811_dn9 = assign24740_e22286_d_n9;
        locals.var_t3__blk811_dn10 = assign24740_e22286_d_n10;
        locals.var_t3__blk811_dn11 = assign24740_e22286_d_n11;
        locals.var_t3__blk811_dn12 = assign24740_e22286_d_n12;

        let (assign24750_e22298, assign24750_e22298_d_n3, assign24750_e22298_d_n4, assign24750_e22298_d_n5, assign24750_e22298_d_n6, assign24750_e22298_d_n7, assign24750_e22298_d_n8, assign24750_e22298_d_n9, assign24750_e22298_d_n10, assign24750_e22298_d_n11, assign24750_e22298_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1330 == 0.0)) {
        let assign24750_e22296: f64 = (locals.var_vgst__blk795 * locals.var_t3__blk811);
        (assign24750_e22296, ((locals.var_vgst__blk795_dn3 * locals.var_t3__blk811) + (locals.var_vgst__blk795 * locals.var_t3__blk811_dn3)), ((locals.var_vgst__blk795_dn4 * locals.var_t3__blk811) + (locals.var_vgst__blk795 * locals.var_t3__blk811_dn4)), ((locals.var_vgst__blk795_dn5 * locals.var_t3__blk811) + (locals.var_vgst__blk795 * locals.var_t3__blk811_dn5)), ((locals.var_vgst__blk795_dn6 * locals.var_t3__blk811) + (locals.var_vgst__blk795 * locals.var_t3__blk811_dn6)), ((locals.var_vgst__blk795_dn7 * locals.var_t3__blk811) + (locals.var_vgst__blk795 * locals.var_t3__blk811_dn7)), ((locals.var_vgst__blk795_dn8 * locals.var_t3__blk811) + (locals.var_vgst__blk795 * locals.var_t3__blk811_dn8)), ((locals.var_vgst__blk795_dn9 * locals.var_t3__blk811) + (locals.var_vgst__blk795 * locals.var_t3__blk811_dn9)), ((locals.var_vgst__blk795_dn10 * locals.var_t3__blk811) + (locals.var_vgst__blk795 * locals.var_t3__blk811_dn10)), ((locals.var_vgst__blk795_dn11 * locals.var_t3__blk811) + (locals.var_vgst__blk795 * locals.var_t3__blk811_dn11)), ((locals.var_vgst__blk795_dn12 * locals.var_t3__blk811) + (locals.var_vgst__blk795 * locals.var_t3__blk811_dn12)),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign24750_e22298;
        locals.var_t2__blk810_dn3 = assign24750_e22298_d_n3;
        locals.var_t2__blk810_dn4 = assign24750_e22298_d_n4;
        locals.var_t2__blk810_dn5 = assign24750_e22298_d_n5;
        locals.var_t2__blk810_dn6 = assign24750_e22298_d_n6;
        locals.var_t2__blk810_dn7 = assign24750_e22298_d_n7;
        locals.var_t2__blk810_dn8 = assign24750_e22298_d_n8;
        locals.var_t2__blk810_dn9 = assign24750_e22298_d_n9;
        locals.var_t2__blk810_dn10 = assign24750_e22298_d_n10;
        locals.var_t2__blk810_dn11 = assign24750_e22298_d_n11;
        locals.var_t2__blk810_dn12 = assign24750_e22298_d_n12;

        let (assign24760_e22314, assign24760_e22314_d_n3, assign24760_e22314_d_n4, assign24760_e22314_d_n5, assign24760_e22314_d_n6, assign24760_e22314_d_n7, assign24760_e22314_d_n8, assign24760_e22314_d_n9, assign24760_e22314_d_n10, assign24760_e22314_d_n11, assign24760_e22314_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1330 == 0.0)) {
        let assign24760_e22310: f64 = (locals.var_pparam_b4soisiid * locals.var_vds_1);
        let assign24760_e22311: f64 = (1.0 + assign24760_e22310);
        let assign24760_e22312: f64 = (1.0 / assign24760_e22311);
        (assign24760_e22312, (-((locals.var_pparam_b4soisiid_dn3 * locals.var_vds_1) / (assign24760_e22311 * assign24760_e22311))), (-((locals.var_pparam_b4soisiid_dn4 * locals.var_vds_1) / (assign24760_e22311 * assign24760_e22311))), (-((locals.var_pparam_b4soisiid_dn5 * locals.var_vds_1) / (assign24760_e22311 * assign24760_e22311))), (-((locals.var_pparam_b4soisiid_dn6 * locals.var_vds_1) / (assign24760_e22311 * assign24760_e22311))), (-(((locals.var_pparam_b4soisiid_dn7 * locals.var_vds_1) + (locals.var_pparam_b4soisiid * locals.var_vds_1_dn7)) / (assign24760_e22311 * assign24760_e22311))), (-(((locals.var_pparam_b4soisiid_dn8 * locals.var_vds_1) + (locals.var_pparam_b4soisiid * locals.var_vds_1_dn8)) / (assign24760_e22311 * assign24760_e22311))), (-((locals.var_pparam_b4soisiid_dn9 * locals.var_vds_1) / (assign24760_e22311 * assign24760_e22311))), (-((locals.var_pparam_b4soisiid_dn10 * locals.var_vds_1) / (assign24760_e22311 * assign24760_e22311))), (-((locals.var_pparam_b4soisiid_dn11 * locals.var_vds_1) / (assign24760_e22311 * assign24760_e22311))), (-((locals.var_pparam_b4soisiid_dn12 * locals.var_vds_1) / (assign24760_e22311 * assign24760_e22311))),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign24760_e22314;
        locals.var_t3__blk811_dn3 = assign24760_e22314_d_n3;
        locals.var_t3__blk811_dn4 = assign24760_e22314_d_n4;
        locals.var_t3__blk811_dn5 = assign24760_e22314_d_n5;
        locals.var_t3__blk811_dn6 = assign24760_e22314_d_n6;
        locals.var_t3__blk811_dn7 = assign24760_e22314_d_n7;
        locals.var_t3__blk811_dn8 = assign24760_e22314_d_n8;
        locals.var_t3__blk811_dn9 = assign24760_e22314_d_n9;
        locals.var_t3__blk811_dn10 = assign24760_e22314_d_n10;
        locals.var_t3__blk811_dn11 = assign24760_e22314_d_n11;
        locals.var_t3__blk811_dn12 = assign24760_e22314_d_n12;

        let (assign24770_e22328, assign24770_e22328_d_n3, assign24770_e22328_d_n4, assign24770_e22328_d_n5, assign24770_e22328_d_n6, assign24770_e22328_d_n7, assign24770_e22328_d_n8, assign24770_e22328_d_n9, assign24770_e22328_d_n10, assign24770_e22328_d_n11, assign24770_e22328_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1330 == 0.0)) {
        let assign24770_e22324: f64 = (locals.var_t1__blk809 * locals.var_t2__blk810);
        let assign24770_e22326: f64 = (assign24770_e22324 * locals.var_t3__blk811);
        (assign24770_e22326, ((((locals.var_t1__blk809_dn3 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn3)) * locals.var_t3__blk811) + (assign24770_e22324 * locals.var_t3__blk811_dn3)), ((((locals.var_t1__blk809_dn4 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn4)) * locals.var_t3__blk811) + (assign24770_e22324 * locals.var_t3__blk811_dn4)), ((((locals.var_t1__blk809_dn5 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn5)) * locals.var_t3__blk811) + (assign24770_e22324 * locals.var_t3__blk811_dn5)), ((((locals.var_t1__blk809_dn6 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn6)) * locals.var_t3__blk811) + (assign24770_e22324 * locals.var_t3__blk811_dn6)), ((((locals.var_t1__blk809_dn7 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn7)) * locals.var_t3__blk811) + (assign24770_e22324 * locals.var_t3__blk811_dn7)), ((((locals.var_t1__blk809_dn8 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn8)) * locals.var_t3__blk811) + (assign24770_e22324 * locals.var_t3__blk811_dn8)), ((((locals.var_t1__blk809_dn9 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn9)) * locals.var_t3__blk811) + (assign24770_e22324 * locals.var_t3__blk811_dn9)), ((((locals.var_t1__blk809_dn10 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn10)) * locals.var_t3__blk811) + (assign24770_e22324 * locals.var_t3__blk811_dn10)), ((((locals.var_t1__blk809_dn11 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn11)) * locals.var_t3__blk811) + (assign24770_e22324 * locals.var_t3__blk811_dn11)), ((((locals.var_t1__blk809_dn12 * locals.var_t2__blk810) + (locals.var_t1__blk809 * locals.var_t2__blk810_dn12)) * locals.var_t3__blk811) + (assign24770_e22324 * locals.var_t3__blk811_dn12)),)
    } else {
        (locals.var_vgsstep, locals.var_vgsstep_dn3, locals.var_vgsstep_dn4, locals.var_vgsstep_dn5, locals.var_vgsstep_dn6, locals.var_vgsstep_dn7, locals.var_vgsstep_dn8, locals.var_vgsstep_dn9, locals.var_vgsstep_dn10, locals.var_vgsstep_dn11, locals.var_vgsstep_dn12,)
    }
};
        locals.var_vgsstep = assign24770_e22328;
        locals.var_vgsstep_dn3 = assign24770_e22328_d_n3;
        locals.var_vgsstep_dn4 = assign24770_e22328_d_n4;
        locals.var_vgsstep_dn5 = assign24770_e22328_d_n5;
        locals.var_vgsstep_dn6 = assign24770_e22328_d_n6;
        locals.var_vgsstep_dn7 = assign24770_e22328_d_n7;
        locals.var_vgsstep_dn8 = assign24770_e22328_d_n8;
        locals.var_vgsstep_dn9 = assign24770_e22328_d_n9;
        locals.var_vgsstep_dn10 = assign24770_e22328_d_n10;
        locals.var_vgsstep_dn11 = assign24770_e22328_d_n11;
        locals.var_vgsstep_dn12 = assign24770_e22328_d_n12;

        let (assign24780_e22340, assign24780_e22340_d_n3, assign24780_e22340_d_n4, assign24780_e22340_d_n5, assign24780_e22340_d_n6, assign24780_e22340_d_n7, assign24780_e22340_d_n8, assign24780_e22340_d_n9, assign24780_e22340_d_n10, assign24780_e22340_d_n11, assign24780_e22340_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1330 == 0.0)) {
        let assign24780_e22338: f64 = (locals.var_vdsatii0 + locals.var_vgsstep);
        (assign24780_e22338, (locals.var_vdsatii0_dn3 + locals.var_vgsstep_dn3), (locals.var_vdsatii0_dn4 + locals.var_vgsstep_dn4), (locals.var_vdsatii0_dn5 + locals.var_vgsstep_dn5), (locals.var_vdsatii0_dn6 + locals.var_vgsstep_dn6), (locals.var_vdsatii0_dn7 + locals.var_vgsstep_dn7), (locals.var_vdsatii0_dn8 + locals.var_vgsstep_dn8), (locals.var_vdsatii0_dn9 + locals.var_vgsstep_dn9), (locals.var_vdsatii0_dn10 + locals.var_vgsstep_dn10), (locals.var_vdsatii0_dn11 + locals.var_vgsstep_dn11), (locals.var_vdsatii0_dn12 + locals.var_vgsstep_dn12),)
    } else {
        (locals.var_vdsatii, locals.var_vdsatii_dn3, locals.var_vdsatii_dn4, locals.var_vdsatii_dn5, locals.var_vdsatii_dn6, locals.var_vdsatii_dn7, locals.var_vdsatii_dn8, locals.var_vdsatii_dn9, locals.var_vdsatii_dn10, locals.var_vdsatii_dn11, locals.var_vdsatii_dn12,)
    }
};
        locals.var_vdsatii = assign24780_e22340;
        locals.var_vdsatii_dn3 = assign24780_e22340_d_n3;
        locals.var_vdsatii_dn4 = assign24780_e22340_d_n4;
        locals.var_vdsatii_dn5 = assign24780_e22340_d_n5;
        locals.var_vdsatii_dn6 = assign24780_e22340_d_n6;
        locals.var_vdsatii_dn7 = assign24780_e22340_d_n7;
        locals.var_vdsatii_dn8 = assign24780_e22340_d_n8;
        locals.var_vdsatii_dn9 = assign24780_e22340_d_n9;
        locals.var_vdsatii_dn10 = assign24780_e22340_d_n10;
        locals.var_vdsatii_dn11 = assign24780_e22340_d_n11;
        locals.var_vdsatii_dn12 = assign24780_e22340_d_n12;

        let (assign24790_e22352, assign24790_e22352_d_n3, assign24790_e22352_d_n4, assign24790_e22352_d_n5, assign24790_e22352_d_n6, assign24790_e22352_d_n7, assign24790_e22352_d_n8, assign24790_e22352_d_n9, assign24790_e22352_d_n10, assign24790_e22352_d_n11, assign24790_e22352_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1330 == 0.0)) {
        let assign24790_e22350: f64 = (locals.var_vds_1 - locals.var_vdsatii);
        (assign24790_e22350, (-locals.var_vdsatii_dn3), (-locals.var_vdsatii_dn4), (-locals.var_vdsatii_dn5), (-locals.var_vdsatii_dn6), (locals.var_vds_1_dn7 - locals.var_vdsatii_dn7), (locals.var_vds_1_dn8 - locals.var_vdsatii_dn8), (-locals.var_vdsatii_dn9), (-locals.var_vdsatii_dn10), (-locals.var_vdsatii_dn11), (-locals.var_vdsatii_dn12),)
    } else {
        (locals.var_vdiff, locals.var_vdiff_dn3, locals.var_vdiff_dn4, locals.var_vdiff_dn5, locals.var_vdiff_dn6, locals.var_vdiff_dn7, locals.var_vdiff_dn8, locals.var_vdiff_dn9, locals.var_vdiff_dn10, locals.var_vdiff_dn11, locals.var_vdiff_dn12,)
    }
};
        locals.var_vdiff = assign24790_e22352;
        locals.var_vdiff_dn3 = assign24790_e22352_d_n3;
        locals.var_vdiff_dn4 = assign24790_e22352_d_n4;
        locals.var_vdiff_dn5 = assign24790_e22352_d_n5;
        locals.var_vdiff_dn6 = assign24790_e22352_d_n6;
        locals.var_vdiff_dn7 = assign24790_e22352_d_n7;
        locals.var_vdiff_dn8 = assign24790_e22352_d_n8;
        locals.var_vdiff_dn9 = assign24790_e22352_d_n9;
        locals.var_vdiff_dn10 = assign24790_e22352_d_n10;
        locals.var_vdiff_dn11 = assign24790_e22352_d_n11;
        locals.var_vdiff_dn12 = assign24790_e22352_d_n12;

        let (assign24800_e22372, assign24800_e22372_d_n3, assign24800_e22372_d_n4, assign24800_e22372_d_n5, assign24800_e22372_d_n6, assign24800_e22372_d_n7, assign24800_e22372_d_n8, assign24800_e22372_d_n9, assign24800_e22372_d_n10, assign24800_e22372_d_n11, assign24800_e22372_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1330 == 0.0)) {
        let assign24800_e22363: f64 = (locals.var_pparam_b4soibeta1 * locals.var_vdiff);
        let assign24800_e22364: f64 = (locals.var_pparam_b4soibeta2 + assign24800_e22363);
        let assign24800_e22367: f64 = (locals.var_pparam_b4soibeta0 * locals.var_vdiff);
        let assign24800_e22369: f64 = (assign24800_e22367 * locals.var_vdiff);
        let assign24800_e22370: f64 = (assign24800_e22364 + assign24800_e22369);
        (assign24800_e22370, ((locals.var_pparam_b4soibeta2_dn3 + ((locals.var_pparam_b4soibeta1_dn3 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn3))) + ((((locals.var_pparam_b4soibeta0_dn3 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn3)) * locals.var_vdiff) + (assign24800_e22367 * locals.var_vdiff_dn3))), ((locals.var_pparam_b4soibeta2_dn4 + ((locals.var_pparam_b4soibeta1_dn4 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn4))) + ((((locals.var_pparam_b4soibeta0_dn4 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn4)) * locals.var_vdiff) + (assign24800_e22367 * locals.var_vdiff_dn4))), ((locals.var_pparam_b4soibeta2_dn5 + ((locals.var_pparam_b4soibeta1_dn5 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn5))) + ((((locals.var_pparam_b4soibeta0_dn5 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn5)) * locals.var_vdiff) + (assign24800_e22367 * locals.var_vdiff_dn5))), ((locals.var_pparam_b4soibeta2_dn6 + ((locals.var_pparam_b4soibeta1_dn6 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn6))) + ((((locals.var_pparam_b4soibeta0_dn6 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn6)) * locals.var_vdiff) + (assign24800_e22367 * locals.var_vdiff_dn6))), ((locals.var_pparam_b4soibeta2_dn7 + ((locals.var_pparam_b4soibeta1_dn7 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn7))) + ((((locals.var_pparam_b4soibeta0_dn7 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn7)) * locals.var_vdiff) + (assign24800_e22367 * locals.var_vdiff_dn7))), ((locals.var_pparam_b4soibeta2_dn8 + ((locals.var_pparam_b4soibeta1_dn8 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn8))) + ((((locals.var_pparam_b4soibeta0_dn8 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn8)) * locals.var_vdiff) + (assign24800_e22367 * locals.var_vdiff_dn8))), ((locals.var_pparam_b4soibeta2_dn9 + ((locals.var_pparam_b4soibeta1_dn9 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn9))) + ((((locals.var_pparam_b4soibeta0_dn9 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn9)) * locals.var_vdiff) + (assign24800_e22367 * locals.var_vdiff_dn9))), ((locals.var_pparam_b4soibeta2_dn10 + ((locals.var_pparam_b4soibeta1_dn10 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn10))) + ((((locals.var_pparam_b4soibeta0_dn10 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn10)) * locals.var_vdiff) + (assign24800_e22367 * locals.var_vdiff_dn10))), ((locals.var_pparam_b4soibeta2_dn11 + ((locals.var_pparam_b4soibeta1_dn11 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn11))) + ((((locals.var_pparam_b4soibeta0_dn11 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn11)) * locals.var_vdiff) + (assign24800_e22367 * locals.var_vdiff_dn11))), ((locals.var_pparam_b4soibeta2_dn12 + ((locals.var_pparam_b4soibeta1_dn12 * locals.var_vdiff) + (locals.var_pparam_b4soibeta1 * locals.var_vdiff_dn12))) + ((((locals.var_pparam_b4soibeta0_dn12 * locals.var_vdiff) + (locals.var_pparam_b4soibeta0 * locals.var_vdiff_dn12)) * locals.var_vdiff) + (assign24800_e22367 * locals.var_vdiff_dn12))),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign24800_e22372;
        locals.var_t0__blk808_dn3 = assign24800_e22372_d_n3;
        locals.var_t0__blk808_dn4 = assign24800_e22372_d_n4;
        locals.var_t0__blk808_dn5 = assign24800_e22372_d_n5;
        locals.var_t0__blk808_dn6 = assign24800_e22372_d_n6;
        locals.var_t0__blk808_dn7 = assign24800_e22372_d_n7;
        locals.var_t0__blk808_dn8 = assign24800_e22372_d_n8;
        locals.var_t0__blk808_dn9 = assign24800_e22372_d_n9;
        locals.var_t0__blk808_dn10 = assign24800_e22372_d_n10;
        locals.var_t0__blk808_dn11 = assign24800_e22372_d_n11;
        locals.var_t0__blk808_dn12 = assign24800_e22372_d_n12;

        let assign24810_e22375: f64 = if locals.var_t0__blk808 < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard1331 = assign24810_e22375;

    }

    pub(super) fn stamp_transient_block_74(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24820_e22387, assign24820_e22387_d_n3, assign24820_e22387_d_n4, assign24820_e22387_d_n5, assign24820_e22387_d_n6, assign24820_e22387_d_n7, assign24820_e22387_d_n8, assign24820_e22387_d_n9, assign24820_e22387_d_n10, assign24820_e22387_d_n11, assign24820_e22387_d_n12,) = {
    if ((((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1331 != 0.0)) {
        (1e-5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign24820_e22387;
        locals.var_t0__blk808_dn3 = assign24820_e22387_d_n3;
        locals.var_t0__blk808_dn4 = assign24820_e22387_d_n4;
        locals.var_t0__blk808_dn5 = assign24820_e22387_d_n5;
        locals.var_t0__blk808_dn6 = assign24820_e22387_d_n6;
        locals.var_t0__blk808_dn7 = assign24820_e22387_d_n7;
        locals.var_t0__blk808_dn8 = assign24820_e22387_d_n8;
        locals.var_t0__blk808_dn9 = assign24820_e22387_d_n9;
        locals.var_t0__blk808_dn10 = assign24820_e22387_d_n10;
        locals.var_t0__blk808_dn11 = assign24820_e22387_d_n11;
        locals.var_t0__blk808_dn12 = assign24820_e22387_d_n12;

        let assign24830_e22391: f64 = (locals.var_vdiff / 100.0);
        let assign24830_e22396: f64 = if ((locals.var_t0__blk808 < assign24830_e22391) && (locals.var_vdiff > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1332 = assign24830_e22396;

        let (assign24840_e22410, assign24840_e22410_d_n3, assign24840_e22410_d_n4, assign24840_e22410_d_n5, assign24840_e22410_d_n6, assign24840_e22410_d_n7, assign24840_e22410_d_n8, assign24840_e22410_d_n9, assign24840_e22410_d_n10, assign24840_e22410_d_n11, assign24840_e22410_d_n12,) = {
    if ((((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1332 != 0.0)) {
        let assign24840_e22408: f64 = (locals.var_pparam_b4soialpha0 * 2.688117142e43);
        (assign24840_e22408, (locals.var_pparam_b4soialpha0_dn3 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn4 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn5 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn6 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn7 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn8 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn9 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn10 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn11 * 2.688117142e43), (locals.var_pparam_b4soialpha0_dn12 * 2.688117142e43),)
    } else {
        (locals.var_ratio, locals.var_ratio_dn3, locals.var_ratio_dn4, locals.var_ratio_dn5, locals.var_ratio_dn6, locals.var_ratio_dn7, locals.var_ratio_dn8, locals.var_ratio_dn9, locals.var_ratio_dn10, locals.var_ratio_dn11, locals.var_ratio_dn12,)
    }
};
        locals.var_ratio = assign24840_e22410;
        locals.var_ratio_dn3 = assign24840_e22410_d_n3;
        locals.var_ratio_dn4 = assign24840_e22410_d_n4;
        locals.var_ratio_dn5 = assign24840_e22410_d_n5;
        locals.var_ratio_dn6 = assign24840_e22410_d_n6;
        locals.var_ratio_dn7 = assign24840_e22410_d_n7;
        locals.var_ratio_dn8 = assign24840_e22410_d_n8;
        locals.var_ratio_dn9 = assign24840_e22410_d_n9;
        locals.var_ratio_dn10 = assign24840_e22410_d_n10;
        locals.var_ratio_dn11 = assign24840_e22410_d_n11;
        locals.var_ratio_dn12 = assign24840_e22410_d_n12;

        let assign24850_e22413: f64 = (-locals.var_vdiff);
        let assign24850_e22415: f64 = (assign24850_e22413 / 100.0);
        let assign24850_e22420: f64 = if ((locals.var_t0__blk808 < assign24850_e22415) && (locals.var_vdiff < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1333 = assign24850_e22420;

        let (assign24860_e22437, assign24860_e22437_d_n3, assign24860_e22437_d_n4, assign24860_e22437_d_n5, assign24860_e22437_d_n6, assign24860_e22437_d_n7, assign24860_e22437_d_n8, assign24860_e22437_d_n9, assign24860_e22437_d_n10, assign24860_e22437_d_n11, assign24860_e22437_d_n12,) = {
    if (((((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1333 != 0.0)) {
        let assign24860_e22435: f64 = (locals.var_pparam_b4soialpha0 * 3.720075976e-44);
        (assign24860_e22435, (locals.var_pparam_b4soialpha0_dn3 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn4 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn5 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn6 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn7 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn8 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn9 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn10 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn11 * 3.720075976e-44), (locals.var_pparam_b4soialpha0_dn12 * 3.720075976e-44),)
    } else {
        (locals.var_ratio, locals.var_ratio_dn3, locals.var_ratio_dn4, locals.var_ratio_dn5, locals.var_ratio_dn6, locals.var_ratio_dn7, locals.var_ratio_dn8, locals.var_ratio_dn9, locals.var_ratio_dn10, locals.var_ratio_dn11, locals.var_ratio_dn12,)
    }
};
        locals.var_ratio = assign24860_e22437;
        locals.var_ratio_dn3 = assign24860_e22437_d_n3;
        locals.var_ratio_dn4 = assign24860_e22437_d_n4;
        locals.var_ratio_dn5 = assign24860_e22437_d_n5;
        locals.var_ratio_dn6 = assign24860_e22437_d_n6;
        locals.var_ratio_dn7 = assign24860_e22437_d_n7;
        locals.var_ratio_dn8 = assign24860_e22437_d_n8;
        locals.var_ratio_dn9 = assign24860_e22437_d_n9;
        locals.var_ratio_dn10 = assign24860_e22437_d_n10;
        locals.var_ratio_dn11 = assign24860_e22437_d_n11;
        locals.var_ratio_dn12 = assign24860_e22437_d_n12;

        let (assign24870_e22458, assign24870_e22458_d_n3, assign24870_e22458_d_n4, assign24870_e22458_d_n5, assign24870_e22458_d_n6, assign24870_e22458_d_n7, assign24870_e22458_d_n8, assign24870_e22458_d_n9, assign24870_e22458_d_n10, assign24870_e22458_d_n11, assign24870_e22458_d_n12,) = {
    if (((((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1332 == 0.0)) && (locals.var_guard1333 == 0.0)) {
        let assign24870_e22454: f64 = (locals.var_vdiff / locals.var_t0__blk808);
        let assign24870_e22455: f64 = (assign24870_e22454).exp();
        let assign24870_e22456: f64 = (locals.var_pparam_b4soialpha0 * assign24870_e22455);
        (assign24870_e22456, ((locals.var_pparam_b4soialpha0_dn3 * assign24870_e22455) + (locals.var_pparam_b4soialpha0 * (assign24870_e22455 * (((locals.var_vdiff_dn3 * locals.var_t0__blk808) - (locals.var_vdiff * locals.var_t0__blk808_dn3)) / (locals.var_t0__blk808 * locals.var_t0__blk808))))), ((locals.var_pparam_b4soialpha0_dn4 * assign24870_e22455) + (locals.var_pparam_b4soialpha0 * (assign24870_e22455 * (((locals.var_vdiff_dn4 * locals.var_t0__blk808) - (locals.var_vdiff * locals.var_t0__blk808_dn4)) / (locals.var_t0__blk808 * locals.var_t0__blk808))))), ((locals.var_pparam_b4soialpha0_dn5 * assign24870_e22455) + (locals.var_pparam_b4soialpha0 * (assign24870_e22455 * (((locals.var_vdiff_dn5 * locals.var_t0__blk808) - (locals.var_vdiff * locals.var_t0__blk808_dn5)) / (locals.var_t0__blk808 * locals.var_t0__blk808))))), ((locals.var_pparam_b4soialpha0_dn6 * assign24870_e22455) + (locals.var_pparam_b4soialpha0 * (assign24870_e22455 * (((locals.var_vdiff_dn6 * locals.var_t0__blk808) - (locals.var_vdiff * locals.var_t0__blk808_dn6)) / (locals.var_t0__blk808 * locals.var_t0__blk808))))), ((locals.var_pparam_b4soialpha0_dn7 * assign24870_e22455) + (locals.var_pparam_b4soialpha0 * (assign24870_e22455 * (((locals.var_vdiff_dn7 * locals.var_t0__blk808) - (locals.var_vdiff * locals.var_t0__blk808_dn7)) / (locals.var_t0__blk808 * locals.var_t0__blk808))))), ((locals.var_pparam_b4soialpha0_dn8 * assign24870_e22455) + (locals.var_pparam_b4soialpha0 * (assign24870_e22455 * (((locals.var_vdiff_dn8 * locals.var_t0__blk808) - (locals.var_vdiff * locals.var_t0__blk808_dn8)) / (locals.var_t0__blk808 * locals.var_t0__blk808))))), ((locals.var_pparam_b4soialpha0_dn9 * assign24870_e22455) + (locals.var_pparam_b4soialpha0 * (assign24870_e22455 * (((locals.var_vdiff_dn9 * locals.var_t0__blk808) - (locals.var_vdiff * locals.var_t0__blk808_dn9)) / (locals.var_t0__blk808 * locals.var_t0__blk808))))), ((locals.var_pparam_b4soialpha0_dn10 * assign24870_e22455) + (locals.var_pparam_b4soialpha0 * (assign24870_e22455 * (((locals.var_vdiff_dn10 * locals.var_t0__blk808) - (locals.var_vdiff * locals.var_t0__blk808_dn10)) / (locals.var_t0__blk808 * locals.var_t0__blk808))))), ((locals.var_pparam_b4soialpha0_dn11 * assign24870_e22455) + (locals.var_pparam_b4soialpha0 * (assign24870_e22455 * (((locals.var_vdiff_dn11 * locals.var_t0__blk808) - (locals.var_vdiff * locals.var_t0__blk808_dn11)) / (locals.var_t0__blk808 * locals.var_t0__blk808))))), ((locals.var_pparam_b4soialpha0_dn12 * assign24870_e22455) + (locals.var_pparam_b4soialpha0 * (assign24870_e22455 * (((locals.var_vdiff_dn12 * locals.var_t0__blk808) - (locals.var_vdiff * locals.var_t0__blk808_dn12)) / (locals.var_t0__blk808 * locals.var_t0__blk808))))),)
    } else {
        (locals.var_ratio, locals.var_ratio_dn3, locals.var_ratio_dn4, locals.var_ratio_dn5, locals.var_ratio_dn6, locals.var_ratio_dn7, locals.var_ratio_dn8, locals.var_ratio_dn9, locals.var_ratio_dn10, locals.var_ratio_dn11, locals.var_ratio_dn12,)
    }
};
        locals.var_ratio = assign24870_e22458;
        locals.var_ratio_dn3 = assign24870_e22458_d_n3;
        locals.var_ratio_dn4 = assign24870_e22458_d_n4;
        locals.var_ratio_dn5 = assign24870_e22458_d_n5;
        locals.var_ratio_dn6 = assign24870_e22458_d_n6;
        locals.var_ratio_dn7 = assign24870_e22458_d_n7;
        locals.var_ratio_dn8 = assign24870_e22458_d_n8;
        locals.var_ratio_dn9 = assign24870_e22458_d_n9;
        locals.var_ratio_dn10 = assign24870_e22458_d_n10;
        locals.var_ratio_dn11 = assign24870_e22458_d_n11;
        locals.var_ratio_dn12 = assign24870_e22458_d_n12;

        let assign24880_e22461: f64 = if locals.var_ratio > 10.0 { 1.0 } else { 0.0 };
        locals.var_guard1334 = assign24880_e22461;

        let (assign24890_e22473, assign24890_e22473_d_n3, assign24890_e22473_d_n4, assign24890_e22473_d_n5, assign24890_e22473_d_n6, assign24890_e22473_d_n7, assign24890_e22473_d_n8, assign24890_e22473_d_n9, assign24890_e22473_d_n10, assign24890_e22473_d_n11, assign24890_e22473_d_n12,) = {
    if ((((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1330 == 0.0)) && (locals.var_guard1334 != 0.0)) {
        (10.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ratio, locals.var_ratio_dn3, locals.var_ratio_dn4, locals.var_ratio_dn5, locals.var_ratio_dn6, locals.var_ratio_dn7, locals.var_ratio_dn8, locals.var_ratio_dn9, locals.var_ratio_dn10, locals.var_ratio_dn11, locals.var_ratio_dn12,)
    }
};
        locals.var_ratio = assign24890_e22473;
        locals.var_ratio_dn3 = assign24890_e22473_d_n3;
        locals.var_ratio_dn4 = assign24890_e22473_d_n4;
        locals.var_ratio_dn5 = assign24890_e22473_d_n5;
        locals.var_ratio_dn6 = assign24890_e22473_d_n6;
        locals.var_ratio_dn7 = assign24890_e22473_d_n7;
        locals.var_ratio_dn8 = assign24890_e22473_d_n8;
        locals.var_ratio_dn9 = assign24890_e22473_d_n9;
        locals.var_ratio_dn10 = assign24890_e22473_d_n10;
        locals.var_ratio_dn11 = assign24890_e22473_d_n11;
        locals.var_ratio_dn12 = assign24890_e22473_d_n12;

        let (assign24900_e22483, assign24900_e22483_d_n3, assign24900_e22483_d_n4, assign24900_e22483_d_n5, assign24900_e22483_d_n6, assign24900_e22483_d_n7, assign24900_e22483_d_n8, assign24900_e22483_d_n9, assign24900_e22483_d_n10, assign24900_e22483_d_n11, assign24900_e22483_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1330 == 0.0)) {
        (locals.var_ids_1, locals.var_ids_1_dn3, locals.var_ids_1_dn4, locals.var_ids_1_dn5, locals.var_ids_1_dn6, locals.var_ids_1_dn7, locals.var_ids_1_dn8, locals.var_ids_1_dn9, locals.var_ids_1_dn10, locals.var_ids_1_dn11, locals.var_ids_1_dn12,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign24900_e22483;
        locals.var_t0__blk808_dn3 = assign24900_e22483_d_n3;
        locals.var_t0__blk808_dn4 = assign24900_e22483_d_n4;
        locals.var_t0__blk808_dn5 = assign24900_e22483_d_n5;
        locals.var_t0__blk808_dn6 = assign24900_e22483_d_n6;
        locals.var_t0__blk808_dn7 = assign24900_e22483_d_n7;
        locals.var_t0__blk808_dn8 = assign24900_e22483_d_n8;
        locals.var_t0__blk808_dn9 = assign24900_e22483_d_n9;
        locals.var_t0__blk808_dn10 = assign24900_e22483_d_n10;
        locals.var_t0__blk808_dn11 = assign24900_e22483_d_n11;
        locals.var_t0__blk808_dn12 = assign24900_e22483_d_n12;

        let (assign24910_e22495, assign24910_e22495_d_n3, assign24910_e22495_d_n4, assign24910_e22495_d_n5, assign24910_e22495_d_n6, assign24910_e22495_d_n7, assign24910_e22495_d_n8, assign24910_e22495_d_n9, assign24910_e22495_d_n10, assign24910_e22495_d_n11, assign24910_e22495_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1330 == 0.0)) {
        let assign24910_e22493: f64 = (locals.var_ratio * locals.var_t0__blk808);
        (assign24910_e22493, ((locals.var_ratio_dn3 * locals.var_t0__blk808) + (locals.var_ratio * locals.var_t0__blk808_dn3)), ((locals.var_ratio_dn4 * locals.var_t0__blk808) + (locals.var_ratio * locals.var_t0__blk808_dn4)), ((locals.var_ratio_dn5 * locals.var_t0__blk808) + (locals.var_ratio * locals.var_t0__blk808_dn5)), ((locals.var_ratio_dn6 * locals.var_t0__blk808) + (locals.var_ratio * locals.var_t0__blk808_dn6)), ((locals.var_ratio_dn7 * locals.var_t0__blk808) + (locals.var_ratio * locals.var_t0__blk808_dn7)), ((locals.var_ratio_dn8 * locals.var_t0__blk808) + (locals.var_ratio * locals.var_t0__blk808_dn8)), ((locals.var_ratio_dn9 * locals.var_t0__blk808) + (locals.var_ratio * locals.var_t0__blk808_dn9)), ((locals.var_ratio_dn10 * locals.var_t0__blk808) + (locals.var_ratio * locals.var_t0__blk808_dn10)), ((locals.var_ratio_dn11 * locals.var_t0__blk808) + (locals.var_ratio * locals.var_t0__blk808_dn11)), ((locals.var_ratio_dn12 * locals.var_t0__blk808) + (locals.var_ratio * locals.var_t0__blk808_dn12)),)
    } else {
        (locals.var_idsmosfet, locals.var_idsmosfet_dn3, locals.var_idsmosfet_dn4, locals.var_idsmosfet_dn5, locals.var_idsmosfet_dn6, locals.var_idsmosfet_dn7, locals.var_idsmosfet_dn8, locals.var_idsmosfet_dn9, locals.var_idsmosfet_dn10, locals.var_idsmosfet_dn11, locals.var_idsmosfet_dn12,)
    }
};
        locals.var_idsmosfet = assign24910_e22495;
        locals.var_idsmosfet_dn3 = assign24910_e22495_d_n3;
        locals.var_idsmosfet_dn4 = assign24910_e22495_d_n4;
        locals.var_idsmosfet_dn5 = assign24910_e22495_d_n5;
        locals.var_idsmosfet_dn6 = assign24910_e22495_d_n6;
        locals.var_idsmosfet_dn7 = assign24910_e22495_d_n7;
        locals.var_idsmosfet_dn8 = assign24910_e22495_d_n8;
        locals.var_idsmosfet_dn9 = assign24910_e22495_d_n9;
        locals.var_idsmosfet_dn10 = assign24910_e22495_d_n10;
        locals.var_idsmosfet_dn11 = assign24910_e22495_d_n11;
        locals.var_idsmosfet_dn12 = assign24910_e22495_d_n12;

        let (assign24920_e22508, assign24920_e22508_d_n3, assign24920_e22508_d_n4, assign24920_e22508_d_n5, assign24920_e22508_d_n6, assign24920_e22508_d_n7, assign24920_e22508_d_n8, assign24920_e22508_d_n9, assign24920_e22508_d_n10, assign24920_e22508_d_n11, assign24920_e22508_d_n12,) = {
    if ((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) {
        let assign24920_e22503: f64 = (locals.var_pparam_b4soiebjtii * locals.var_leff);
        let assign24920_e22504: f64 = (locals.var_pparam_b4soicbjtii + assign24920_e22503);
        let assign24920_e22506: f64 = (assign24920_e22504 / locals.var_leff);
        (assign24920_e22506, ((((locals.var_pparam_b4soicbjtii_dn3 + ((locals.var_pparam_b4soiebjtii_dn3 * locals.var_leff) + (locals.var_pparam_b4soiebjtii * locals.var_leff_dn3))) * locals.var_leff) - (assign24920_e22504 * locals.var_leff_dn3)) / (locals.var_leff * locals.var_leff)), ((((locals.var_pparam_b4soicbjtii_dn4 + ((locals.var_pparam_b4soiebjtii_dn4 * locals.var_leff) + (locals.var_pparam_b4soiebjtii * locals.var_leff_dn4))) * locals.var_leff) - (assign24920_e22504 * locals.var_leff_dn4)) / (locals.var_leff * locals.var_leff)), ((((locals.var_pparam_b4soicbjtii_dn5 + ((locals.var_pparam_b4soiebjtii_dn5 * locals.var_leff) + (locals.var_pparam_b4soiebjtii * locals.var_leff_dn5))) * locals.var_leff) - (assign24920_e22504 * locals.var_leff_dn5)) / (locals.var_leff * locals.var_leff)), ((((locals.var_pparam_b4soicbjtii_dn6 + ((locals.var_pparam_b4soiebjtii_dn6 * locals.var_leff) + (locals.var_pparam_b4soiebjtii * locals.var_leff_dn6))) * locals.var_leff) - (assign24920_e22504 * locals.var_leff_dn6)) / (locals.var_leff * locals.var_leff)), ((((locals.var_pparam_b4soicbjtii_dn7 + ((locals.var_pparam_b4soiebjtii_dn7 * locals.var_leff) + (locals.var_pparam_b4soiebjtii * locals.var_leff_dn7))) * locals.var_leff) - (assign24920_e22504 * locals.var_leff_dn7)) / (locals.var_leff * locals.var_leff)), ((((locals.var_pparam_b4soicbjtii_dn8 + ((locals.var_pparam_b4soiebjtii_dn8 * locals.var_leff) + (locals.var_pparam_b4soiebjtii * locals.var_leff_dn8))) * locals.var_leff) - (assign24920_e22504 * locals.var_leff_dn8)) / (locals.var_leff * locals.var_leff)), ((((locals.var_pparam_b4soicbjtii_dn9 + ((locals.var_pparam_b4soiebjtii_dn9 * locals.var_leff) + (locals.var_pparam_b4soiebjtii * locals.var_leff_dn9))) * locals.var_leff) - (assign24920_e22504 * locals.var_leff_dn9)) / (locals.var_leff * locals.var_leff)), ((((locals.var_pparam_b4soicbjtii_dn10 + ((locals.var_pparam_b4soiebjtii_dn10 * locals.var_leff) + (locals.var_pparam_b4soiebjtii * locals.var_leff_dn10))) * locals.var_leff) - (assign24920_e22504 * locals.var_leff_dn10)) / (locals.var_leff * locals.var_leff)), ((((locals.var_pparam_b4soicbjtii_dn11 + ((locals.var_pparam_b4soiebjtii_dn11 * locals.var_leff) + (locals.var_pparam_b4soiebjtii * locals.var_leff_dn11))) * locals.var_leff) - (assign24920_e22504 * locals.var_leff_dn11)) / (locals.var_leff * locals.var_leff)), ((((locals.var_pparam_b4soicbjtii_dn12 + ((locals.var_pparam_b4soiebjtii_dn12 * locals.var_leff) + (locals.var_pparam_b4soiebjtii * locals.var_leff_dn12))) * locals.var_leff) - (assign24920_e22504 * locals.var_leff_dn12)) / (locals.var_leff * locals.var_leff)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign24920_e22508;
        locals.var_t0__blk808_dn3 = assign24920_e22508_d_n3;
        locals.var_t0__blk808_dn4 = assign24920_e22508_d_n4;
        locals.var_t0__blk808_dn5 = assign24920_e22508_d_n5;
        locals.var_t0__blk808_dn6 = assign24920_e22508_d_n6;
        locals.var_t0__blk808_dn7 = assign24920_e22508_d_n7;
        locals.var_t0__blk808_dn8 = assign24920_e22508_d_n8;
        locals.var_t0__blk808_dn9 = assign24920_e22508_d_n9;
        locals.var_t0__blk808_dn10 = assign24920_e22508_d_n10;
        locals.var_t0__blk808_dn11 = assign24920_e22508_d_n11;
        locals.var_t0__blk808_dn12 = assign24920_e22508_d_n12;

        let (assign24930_e22521, assign24930_e22521_d_n3, assign24930_e22521_d_n4, assign24930_e22521_d_n5, assign24930_e22521_d_n6, assign24930_e22521_d_n7, assign24930_e22521_d_n8, assign24930_e22521_d_n9, assign24930_e22521_d_n10, assign24930_e22521_d_n11, assign24930_e22521_d_n12,) = {
    if ((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) {
        let assign24930_e22517: f64 = (p.p320 * locals.var_trm1);
        let assign24930_e22518: f64 = (1.0 + assign24930_e22517);
        let assign24930_e22519: f64 = (locals.var_pparam_b4soivbci * assign24930_e22518);
        (assign24930_e22519, (locals.var_pparam_b4soivbci_dn3 * assign24930_e22518), ((locals.var_pparam_b4soivbci_dn4 * assign24930_e22518) + (locals.var_pparam_b4soivbci * (p.p320 * locals.var_trm1_dn4))), ((locals.var_pparam_b4soivbci_dn5 * assign24930_e22518) + (locals.var_pparam_b4soivbci * (p.p320 * locals.var_trm1_dn5))), ((locals.var_pparam_b4soivbci_dn6 * assign24930_e22518) + (locals.var_pparam_b4soivbci * (p.p320 * locals.var_trm1_dn6))), (locals.var_pparam_b4soivbci_dn7 * assign24930_e22518), (locals.var_pparam_b4soivbci_dn8 * assign24930_e22518), (locals.var_pparam_b4soivbci_dn9 * assign24930_e22518), (locals.var_pparam_b4soivbci_dn10 * assign24930_e22518), (locals.var_pparam_b4soivbci_dn11 * assign24930_e22518), (locals.var_pparam_b4soivbci_dn12 * assign24930_e22518),)
    } else {
        (locals.var_vbci, locals.var_vbci_dn3, locals.var_vbci_dn4, locals.var_vbci_dn5, locals.var_vbci_dn6, locals.var_vbci_dn7, locals.var_vbci_dn8, locals.var_vbci_dn9, locals.var_vbci_dn10, locals.var_vbci_dn11, locals.var_vbci_dn12,)
    }
};
        locals.var_vbci = assign24930_e22521;
        locals.var_vbci_dn3 = assign24930_e22521_d_n3;
        locals.var_vbci_dn4 = assign24930_e22521_d_n4;
        locals.var_vbci_dn5 = assign24930_e22521_d_n5;
        locals.var_vbci_dn6 = assign24930_e22521_d_n6;
        locals.var_vbci_dn7 = assign24930_e22521_d_n7;
        locals.var_vbci_dn8 = assign24930_e22521_d_n8;
        locals.var_vbci_dn9 = assign24930_e22521_d_n9;
        locals.var_vbci_dn10 = assign24930_e22521_d_n10;
        locals.var_vbci_dn11 = assign24930_e22521_d_n11;
        locals.var_vbci_dn12 = assign24930_e22521_d_n12;

        let assign24940_e22524: f64 = if locals.var_b4soimode > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1335 = assign24940_e22524;

        let (assign24950_e22535, assign24950_e22535_d_n3, assign24950_e22535_d_n4, assign24950_e22535_d_n5, assign24950_e22535_d_n6, assign24950_e22535_d_n7, assign24950_e22535_d_n8, assign24950_e22535_d_n9, assign24950_e22535_d_n10, assign24950_e22535_d_n11, assign24950_e22535_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1335 != 0.0)) {
        let assign24950_e22533: f64 = (locals.var_vbci - locals.var_vdbd);
        (assign24950_e22533, locals.var_vbci_dn3, locals.var_vbci_dn4, locals.var_vbci_dn5, locals.var_vbci_dn6, (locals.var_vbci_dn7 - locals.var_vdbd_dn7), locals.var_vbci_dn8, locals.var_vbci_dn9, locals.var_vbci_dn10, locals.var_vbci_dn11, (locals.var_vbci_dn12 - locals.var_vdbd_dn12),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign24950_e22535;
        locals.var_t1__blk809_dn3 = assign24950_e22535_d_n3;
        locals.var_t1__blk809_dn4 = assign24950_e22535_d_n4;
        locals.var_t1__blk809_dn5 = assign24950_e22535_d_n5;
        locals.var_t1__blk809_dn6 = assign24950_e22535_d_n6;
        locals.var_t1__blk809_dn7 = assign24950_e22535_d_n7;
        locals.var_t1__blk809_dn8 = assign24950_e22535_d_n8;
        locals.var_t1__blk809_dn9 = assign24950_e22535_d_n9;
        locals.var_t1__blk809_dn10 = assign24950_e22535_d_n10;
        locals.var_t1__blk809_dn11 = assign24950_e22535_d_n11;
        locals.var_t1__blk809_dn12 = assign24950_e22535_d_n12;

        let (assign24960_e22547, assign24960_e22547_d_n3, assign24960_e22547_d_n4, assign24960_e22547_d_n5, assign24960_e22547_d_n6, assign24960_e22547_d_n7, assign24960_e22547_d_n8, assign24960_e22547_d_n9, assign24960_e22547_d_n10, assign24960_e22547_d_n11, assign24960_e22547_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1335 == 0.0)) {
        let assign24960_e22545: f64 = (locals.var_vbci - locals.var_vsbs);
        (assign24960_e22545, locals.var_vbci_dn3, locals.var_vbci_dn4, locals.var_vbci_dn5, locals.var_vbci_dn6, locals.var_vbci_dn7, (locals.var_vbci_dn8 - locals.var_vsbs_dn8), locals.var_vbci_dn9, locals.var_vbci_dn10, (locals.var_vbci_dn11 - locals.var_vsbs_dn11), locals.var_vbci_dn12,)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign24960_e22547;
        locals.var_t1__blk809_dn3 = assign24960_e22547_d_n3;
        locals.var_t1__blk809_dn4 = assign24960_e22547_d_n4;
        locals.var_t1__blk809_dn5 = assign24960_e22547_d_n5;
        locals.var_t1__blk809_dn6 = assign24960_e22547_d_n6;
        locals.var_t1__blk809_dn7 = assign24960_e22547_d_n7;
        locals.var_t1__blk809_dn8 = assign24960_e22547_d_n8;
        locals.var_t1__blk809_dn9 = assign24960_e22547_d_n9;
        locals.var_t1__blk809_dn10 = assign24960_e22547_d_n10;
        locals.var_t1__blk809_dn11 = assign24960_e22547_d_n11;
        locals.var_t1__blk809_dn12 = assign24960_e22547_d_n12;

        let (assign24970_e22556, assign24970_e22556_d_n3, assign24970_e22556_d_n4, assign24970_e22556_d_n5, assign24970_e22556_d_n6, assign24970_e22556_d_n7, assign24970_e22556_d_n8, assign24970_e22556_d_n9, assign24970_e22556_d_n10, assign24970_e22556_d_n11, assign24970_e22556_d_n12,) = {
    if ((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) {
        let assign24970_e22554: f64 = (locals.var_pparam_b4soimbjtii - 1.0);
        (assign24970_e22554, locals.var_pparam_b4soimbjtii_dn3, locals.var_pparam_b4soimbjtii_dn4, locals.var_pparam_b4soimbjtii_dn5, locals.var_pparam_b4soimbjtii_dn6, locals.var_pparam_b4soimbjtii_dn7, locals.var_pparam_b4soimbjtii_dn8, locals.var_pparam_b4soimbjtii_dn9, locals.var_pparam_b4soimbjtii_dn10, locals.var_pparam_b4soimbjtii_dn11, locals.var_pparam_b4soimbjtii_dn12,)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign24970_e22556;
        locals.var_t2__blk810_dn3 = assign24970_e22556_d_n3;
        locals.var_t2__blk810_dn4 = assign24970_e22556_d_n4;
        locals.var_t2__blk810_dn5 = assign24970_e22556_d_n5;
        locals.var_t2__blk810_dn6 = assign24970_e22556_d_n6;
        locals.var_t2__blk810_dn7 = assign24970_e22556_d_n7;
        locals.var_t2__blk810_dn8 = assign24970_e22556_d_n8;
        locals.var_t2__blk810_dn9 = assign24970_e22556_d_n9;
        locals.var_t2__blk810_dn10 = assign24970_e22556_d_n10;
        locals.var_t2__blk810_dn11 = assign24970_e22556_d_n11;
        locals.var_t2__blk810_dn12 = assign24970_e22556_d_n12;

        let assign24980_e22559: f64 = if locals.var_t1__blk809 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1336 = assign24980_e22559;

        let (assign24990_e22568, assign24990_e22568_d_n3, assign24990_e22568_d_n4, assign24990_e22568_d_n5, assign24990_e22568_d_n6, assign24990_e22568_d_n7, assign24990_e22568_d_n8, assign24990_e22568_d_n9, assign24990_e22568_d_n10, assign24990_e22568_d_n11, assign24990_e22568_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1336 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign24990_e22568;
        locals.var_t3__blk811_dn3 = assign24990_e22568_d_n3;
        locals.var_t3__blk811_dn4 = assign24990_e22568_d_n4;
        locals.var_t3__blk811_dn5 = assign24990_e22568_d_n5;
        locals.var_t3__blk811_dn6 = assign24990_e22568_d_n6;
        locals.var_t3__blk811_dn7 = assign24990_e22568_d_n7;
        locals.var_t3__blk811_dn8 = assign24990_e22568_d_n8;
        locals.var_t3__blk811_dn9 = assign24990_e22568_d_n9;
        locals.var_t3__blk811_dn10 = assign24990_e22568_d_n10;
        locals.var_t3__blk811_dn11 = assign24990_e22568_d_n11;
        locals.var_t3__blk811_dn12 = assign24990_e22568_d_n12;

        let (assign25000_e22583, assign25000_e22583_d_n3, assign25000_e22583_d_n4, assign25000_e22583_d_n5, assign25000_e22583_d_n6, assign25000_e22583_d_n7, assign25000_e22583_d_n8, assign25000_e22583_d_n9, assign25000_e22583_d_n10, assign25000_e22583_d_n11, assign25000_e22583_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1336 == 0.0)) {
        let assign25000_e22577: f64 = (-locals.var_pparam_b4soiabjtii);
        let assign25000_e22580: f64 = (locals.var_t1__blk809).powf(locals.var_t2__blk810);
        let assign25000_e22581: f64 = (assign25000_e22577 * assign25000_e22580);
        (assign25000_e22581, (((-locals.var_pparam_b4soiabjtii_dn3) * assign25000_e22580) + (assign25000_e22577 * if locals.var_t2__blk810_dn3 == 0.0 && ((locals.var_t2__blk810) as f64).is_finite() && ((locals.var_t2__blk810) as f64).fract() == 0.0 { if locals.var_t2__blk810 == 0.0 { 0.0 } else { (locals.var_t2__blk810 * ((locals.var_t1__blk809).powf(locals.var_t2__blk810 - 1.0) * locals.var_t1__blk809_dn3)) } } else { (assign25000_e22580 * ((locals.var_t2__blk810_dn3 * (locals.var_t1__blk809).ln()) + (locals.var_t2__blk810 * (locals.var_t1__blk809_dn3 / locals.var_t1__blk809)))) })), (((-locals.var_pparam_b4soiabjtii_dn4) * assign25000_e22580) + (assign25000_e22577 * if locals.var_t2__blk810_dn4 == 0.0 && ((locals.var_t2__blk810) as f64).is_finite() && ((locals.var_t2__blk810) as f64).fract() == 0.0 { if locals.var_t2__blk810 == 0.0 { 0.0 } else { (locals.var_t2__blk810 * ((locals.var_t1__blk809).powf(locals.var_t2__blk810 - 1.0) * locals.var_t1__blk809_dn4)) } } else { (assign25000_e22580 * ((locals.var_t2__blk810_dn4 * (locals.var_t1__blk809).ln()) + (locals.var_t2__blk810 * (locals.var_t1__blk809_dn4 / locals.var_t1__blk809)))) })), (((-locals.var_pparam_b4soiabjtii_dn5) * assign25000_e22580) + (assign25000_e22577 * if locals.var_t2__blk810_dn5 == 0.0 && ((locals.var_t2__blk810) as f64).is_finite() && ((locals.var_t2__blk810) as f64).fract() == 0.0 { if locals.var_t2__blk810 == 0.0 { 0.0 } else { (locals.var_t2__blk810 * ((locals.var_t1__blk809).powf(locals.var_t2__blk810 - 1.0) * locals.var_t1__blk809_dn5)) } } else { (assign25000_e22580 * ((locals.var_t2__blk810_dn5 * (locals.var_t1__blk809).ln()) + (locals.var_t2__blk810 * (locals.var_t1__blk809_dn5 / locals.var_t1__blk809)))) })), (((-locals.var_pparam_b4soiabjtii_dn6) * assign25000_e22580) + (assign25000_e22577 * if locals.var_t2__blk810_dn6 == 0.0 && ((locals.var_t2__blk810) as f64).is_finite() && ((locals.var_t2__blk810) as f64).fract() == 0.0 { if locals.var_t2__blk810 == 0.0 { 0.0 } else { (locals.var_t2__blk810 * ((locals.var_t1__blk809).powf(locals.var_t2__blk810 - 1.0) * locals.var_t1__blk809_dn6)) } } else { (assign25000_e22580 * ((locals.var_t2__blk810_dn6 * (locals.var_t1__blk809).ln()) + (locals.var_t2__blk810 * (locals.var_t1__blk809_dn6 / locals.var_t1__blk809)))) })), (((-locals.var_pparam_b4soiabjtii_dn7) * assign25000_e22580) + (assign25000_e22577 * if locals.var_t2__blk810_dn7 == 0.0 && ((locals.var_t2__blk810) as f64).is_finite() && ((locals.var_t2__blk810) as f64).fract() == 0.0 { if locals.var_t2__blk810 == 0.0 { 0.0 } else { (locals.var_t2__blk810 * ((locals.var_t1__blk809).powf(locals.var_t2__blk810 - 1.0) * locals.var_t1__blk809_dn7)) } } else { (assign25000_e22580 * ((locals.var_t2__blk810_dn7 * (locals.var_t1__blk809).ln()) + (locals.var_t2__blk810 * (locals.var_t1__blk809_dn7 / locals.var_t1__blk809)))) })), (((-locals.var_pparam_b4soiabjtii_dn8) * assign25000_e22580) + (assign25000_e22577 * if locals.var_t2__blk810_dn8 == 0.0 && ((locals.var_t2__blk810) as f64).is_finite() && ((locals.var_t2__blk810) as f64).fract() == 0.0 { if locals.var_t2__blk810 == 0.0 { 0.0 } else { (locals.var_t2__blk810 * ((locals.var_t1__blk809).powf(locals.var_t2__blk810 - 1.0) * locals.var_t1__blk809_dn8)) } } else { (assign25000_e22580 * ((locals.var_t2__blk810_dn8 * (locals.var_t1__blk809).ln()) + (locals.var_t2__blk810 * (locals.var_t1__blk809_dn8 / locals.var_t1__blk809)))) })), (((-locals.var_pparam_b4soiabjtii_dn9) * assign25000_e22580) + (assign25000_e22577 * if locals.var_t2__blk810_dn9 == 0.0 && ((locals.var_t2__blk810) as f64).is_finite() && ((locals.var_t2__blk810) as f64).fract() == 0.0 { if locals.var_t2__blk810 == 0.0 { 0.0 } else { (locals.var_t2__blk810 * ((locals.var_t1__blk809).powf(locals.var_t2__blk810 - 1.0) * locals.var_t1__blk809_dn9)) } } else { (assign25000_e22580 * ((locals.var_t2__blk810_dn9 * (locals.var_t1__blk809).ln()) + (locals.var_t2__blk810 * (locals.var_t1__blk809_dn9 / locals.var_t1__blk809)))) })), (((-locals.var_pparam_b4soiabjtii_dn10) * assign25000_e22580) + (assign25000_e22577 * if locals.var_t2__blk810_dn10 == 0.0 && ((locals.var_t2__blk810) as f64).is_finite() && ((locals.var_t2__blk810) as f64).fract() == 0.0 { if locals.var_t2__blk810 == 0.0 { 0.0 } else { (locals.var_t2__blk810 * ((locals.var_t1__blk809).powf(locals.var_t2__blk810 - 1.0) * locals.var_t1__blk809_dn10)) } } else { (assign25000_e22580 * ((locals.var_t2__blk810_dn10 * (locals.var_t1__blk809).ln()) + (locals.var_t2__blk810 * (locals.var_t1__blk809_dn10 / locals.var_t1__blk809)))) })), (((-locals.var_pparam_b4soiabjtii_dn11) * assign25000_e22580) + (assign25000_e22577 * if locals.var_t2__blk810_dn11 == 0.0 && ((locals.var_t2__blk810) as f64).is_finite() && ((locals.var_t2__blk810) as f64).fract() == 0.0 { if locals.var_t2__blk810 == 0.0 { 0.0 } else { (locals.var_t2__blk810 * ((locals.var_t1__blk809).powf(locals.var_t2__blk810 - 1.0) * locals.var_t1__blk809_dn11)) } } else { (assign25000_e22580 * ((locals.var_t2__blk810_dn11 * (locals.var_t1__blk809).ln()) + (locals.var_t2__blk810 * (locals.var_t1__blk809_dn11 / locals.var_t1__blk809)))) })), (((-locals.var_pparam_b4soiabjtii_dn12) * assign25000_e22580) + (assign25000_e22577 * if locals.var_t2__blk810_dn12 == 0.0 && ((locals.var_t2__blk810) as f64).is_finite() && ((locals.var_t2__blk810) as f64).fract() == 0.0 { if locals.var_t2__blk810 == 0.0 { 0.0 } else { (locals.var_t2__blk810 * ((locals.var_t1__blk809).powf(locals.var_t2__blk810 - 1.0) * locals.var_t1__blk809_dn12)) } } else { (assign25000_e22580 * ((locals.var_t2__blk810_dn12 * (locals.var_t1__blk809).ln()) + (locals.var_t2__blk810 * (locals.var_t1__blk809_dn12 / locals.var_t1__blk809)))) })),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign25000_e22583;
        locals.var_t3__blk811_dn3 = assign25000_e22583_d_n3;
        locals.var_t3__blk811_dn4 = assign25000_e22583_d_n4;
        locals.var_t3__blk811_dn5 = assign25000_e22583_d_n5;
        locals.var_t3__blk811_dn6 = assign25000_e22583_d_n6;
        locals.var_t3__blk811_dn7 = assign25000_e22583_d_n7;
        locals.var_t3__blk811_dn8 = assign25000_e22583_d_n8;
        locals.var_t3__blk811_dn9 = assign25000_e22583_d_n9;
        locals.var_t3__blk811_dn10 = assign25000_e22583_d_n10;
        locals.var_t3__blk811_dn11 = assign25000_e22583_d_n11;
        locals.var_t3__blk811_dn12 = assign25000_e22583_d_n12;

        let assign25010_e22586: f64 = if locals.var_t3__blk811 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1337 = assign25010_e22586;

        let (assign25020_e22595, assign25020_e22595_d_n3, assign25020_e22595_d_n4, assign25020_e22595_d_n5, assign25020_e22595_d_n6, assign25020_e22595_d_n7, assign25020_e22595_d_n8, assign25020_e22595_d_n9, assign25020_e22595_d_n10, assign25020_e22595_d_n11, assign25020_e22595_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1337 != 0.0)) {
        (2.688117142e43, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign25020_e22595;
        locals.var_t4__blk812_dn3 = assign25020_e22595_d_n3;
        locals.var_t4__blk812_dn4 = assign25020_e22595_d_n4;
        locals.var_t4__blk812_dn5 = assign25020_e22595_d_n5;
        locals.var_t4__blk812_dn6 = assign25020_e22595_d_n6;
        locals.var_t4__blk812_dn7 = assign25020_e22595_d_n7;
        locals.var_t4__blk812_dn8 = assign25020_e22595_d_n8;
        locals.var_t4__blk812_dn9 = assign25020_e22595_d_n9;
        locals.var_t4__blk812_dn10 = assign25020_e22595_d_n10;
        locals.var_t4__blk812_dn11 = assign25020_e22595_d_n11;
        locals.var_t4__blk812_dn12 = assign25020_e22595_d_n12;

        let assign25030_e22598: f64 = (-100.0);
        let assign25030_e22599: f64 = if locals.var_t3__blk811 < assign25030_e22598 { 1.0 } else { 0.0 };
        locals.var_guard1338 = assign25030_e22599;

        let (assign25040_e22611, assign25040_e22611_d_n3, assign25040_e22611_d_n4, assign25040_e22611_d_n5, assign25040_e22611_d_n6, assign25040_e22611_d_n7, assign25040_e22611_d_n8, assign25040_e22611_d_n9, assign25040_e22611_d_n10, assign25040_e22611_d_n11, assign25040_e22611_d_n12,) = {
    if ((((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 != 0.0)) {
        (3.720075976e-44, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign25040_e22611;
        locals.var_t4__blk812_dn3 = assign25040_e22611_d_n3;
        locals.var_t4__blk812_dn4 = assign25040_e22611_d_n4;
        locals.var_t4__blk812_dn5 = assign25040_e22611_d_n5;
        locals.var_t4__blk812_dn6 = assign25040_e22611_d_n6;
        locals.var_t4__blk812_dn7 = assign25040_e22611_d_n7;
        locals.var_t4__blk812_dn8 = assign25040_e22611_d_n8;
        locals.var_t4__blk812_dn9 = assign25040_e22611_d_n9;
        locals.var_t4__blk812_dn10 = assign25040_e22611_d_n10;
        locals.var_t4__blk812_dn11 = assign25040_e22611_d_n11;
        locals.var_t4__blk812_dn12 = assign25040_e22611_d_n12;

        let (assign25050_e22625, assign25050_e22625_d_n3, assign25050_e22625_d_n4, assign25050_e22625_d_n5, assign25050_e22625_d_n6, assign25050_e22625_d_n7, assign25050_e22625_d_n8, assign25050_e22625_d_n9, assign25050_e22625_d_n10, assign25050_e22625_d_n11, assign25050_e22625_d_n12,) = {
    if ((((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) && (locals.var_guard1337 == 0.0)) && (locals.var_guard1338 == 0.0)) {
        let assign25050_e22623: f64 = (locals.var_t3__blk811).exp();
        (assign25050_e22623, (assign25050_e22623 * locals.var_t3__blk811_dn3), (assign25050_e22623 * locals.var_t3__blk811_dn4), (assign25050_e22623 * locals.var_t3__blk811_dn5), (assign25050_e22623 * locals.var_t3__blk811_dn6), (assign25050_e22623 * locals.var_t3__blk811_dn7), (assign25050_e22623 * locals.var_t3__blk811_dn8), (assign25050_e22623 * locals.var_t3__blk811_dn9), (assign25050_e22623 * locals.var_t3__blk811_dn10), (assign25050_e22623 * locals.var_t3__blk811_dn11), (assign25050_e22623 * locals.var_t3__blk811_dn12),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign25050_e22625;
        locals.var_t4__blk812_dn3 = assign25050_e22625_d_n3;
        locals.var_t4__blk812_dn4 = assign25050_e22625_d_n4;
        locals.var_t4__blk812_dn5 = assign25050_e22625_d_n5;
        locals.var_t4__blk812_dn6 = assign25050_e22625_d_n6;
        locals.var_t4__blk812_dn7 = assign25050_e22625_d_n7;
        locals.var_t4__blk812_dn8 = assign25050_e22625_d_n8;
        locals.var_t4__blk812_dn9 = assign25050_e22625_d_n9;
        locals.var_t4__blk812_dn10 = assign25050_e22625_d_n10;
        locals.var_t4__blk812_dn11 = assign25050_e22625_d_n11;
        locals.var_t4__blk812_dn12 = assign25050_e22625_d_n12;

        let (assign25060_e22640, assign25060_e22640_d_n3, assign25060_e22640_d_n4, assign25060_e22640_d_n5, assign25060_e22640_d_n6, assign25060_e22640_d_n7, assign25060_e22640_d_n8, assign25060_e22640_d_n9, assign25060_e22640_d_n10, assign25060_e22640_d_n11, assign25060_e22640_d_n12,) = {
    if ((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) {
        let assign25060_e22632: f64 = (locals.var_t0__blk808 * locals.var_b4soimode);
        let assign25060_e22634: f64 = (assign25060_e22632 * locals.var_ic_1);
        let assign25060_e22636: f64 = (assign25060_e22634 * locals.var_t1__blk809);
        let assign25060_e22638: f64 = (assign25060_e22636 * locals.var_t4__blk812);
        (assign25060_e22638, (((((((locals.var_t0__blk808_dn3 * locals.var_b4soimode) * locals.var_ic_1) + (assign25060_e22632 * locals.var_ic_1_dn3)) * locals.var_t1__blk809) + (assign25060_e22634 * locals.var_t1__blk809_dn3)) * locals.var_t4__blk812) + (assign25060_e22636 * locals.var_t4__blk812_dn3)), (((((((locals.var_t0__blk808_dn4 * locals.var_b4soimode) * locals.var_ic_1) + (assign25060_e22632 * locals.var_ic_1_dn4)) * locals.var_t1__blk809) + (assign25060_e22634 * locals.var_t1__blk809_dn4)) * locals.var_t4__blk812) + (assign25060_e22636 * locals.var_t4__blk812_dn4)), (((((((locals.var_t0__blk808_dn5 * locals.var_b4soimode) * locals.var_ic_1) + (assign25060_e22632 * locals.var_ic_1_dn5)) * locals.var_t1__blk809) + (assign25060_e22634 * locals.var_t1__blk809_dn5)) * locals.var_t4__blk812) + (assign25060_e22636 * locals.var_t4__blk812_dn5)), (((((((locals.var_t0__blk808_dn6 * locals.var_b4soimode) * locals.var_ic_1) + (assign25060_e22632 * locals.var_ic_1_dn6)) * locals.var_t1__blk809) + (assign25060_e22634 * locals.var_t1__blk809_dn6)) * locals.var_t4__blk812) + (assign25060_e22636 * locals.var_t4__blk812_dn6)), (((((((locals.var_t0__blk808_dn7 * locals.var_b4soimode) * locals.var_ic_1) + (assign25060_e22632 * locals.var_ic_1_dn7)) * locals.var_t1__blk809) + (assign25060_e22634 * locals.var_t1__blk809_dn7)) * locals.var_t4__blk812) + (assign25060_e22636 * locals.var_t4__blk812_dn7)), (((((((locals.var_t0__blk808_dn8 * locals.var_b4soimode) * locals.var_ic_1) + (assign25060_e22632 * locals.var_ic_1_dn8)) * locals.var_t1__blk809) + (assign25060_e22634 * locals.var_t1__blk809_dn8)) * locals.var_t4__blk812) + (assign25060_e22636 * locals.var_t4__blk812_dn8)), (((((((locals.var_t0__blk808_dn9 * locals.var_b4soimode) * locals.var_ic_1) + (assign25060_e22632 * locals.var_ic_1_dn9)) * locals.var_t1__blk809) + (assign25060_e22634 * locals.var_t1__blk809_dn9)) * locals.var_t4__blk812) + (assign25060_e22636 * locals.var_t4__blk812_dn9)), (((((((locals.var_t0__blk808_dn10 * locals.var_b4soimode) * locals.var_ic_1) + (assign25060_e22632 * locals.var_ic_1_dn10)) * locals.var_t1__blk809) + (assign25060_e22634 * locals.var_t1__blk809_dn10)) * locals.var_t4__blk812) + (assign25060_e22636 * locals.var_t4__blk812_dn10)), (((((((locals.var_t0__blk808_dn11 * locals.var_b4soimode) * locals.var_ic_1) + (assign25060_e22632 * locals.var_ic_1_dn11)) * locals.var_t1__blk809) + (assign25060_e22634 * locals.var_t1__blk809_dn11)) * locals.var_t4__blk812) + (assign25060_e22636 * locals.var_t4__blk812_dn11)), (((((((locals.var_t0__blk808_dn12 * locals.var_b4soimode) * locals.var_ic_1) + (assign25060_e22632 * locals.var_ic_1_dn12)) * locals.var_t1__blk809) + (assign25060_e22634 * locals.var_t1__blk809_dn12)) * locals.var_t4__blk812) + (assign25060_e22636 * locals.var_t4__blk812_dn12)),)
    } else {
        (locals.var_iiibjt, locals.var_iiibjt_dn3, locals.var_iiibjt_dn4, locals.var_iiibjt_dn5, locals.var_iiibjt_dn6, locals.var_iiibjt_dn7, locals.var_iiibjt_dn8, locals.var_iiibjt_dn9, locals.var_iiibjt_dn10, locals.var_iiibjt_dn11, locals.var_iiibjt_dn12,)
    }
};
        locals.var_iiibjt = assign25060_e22640;
        locals.var_iiibjt_dn3 = assign25060_e22640_d_n3;
        locals.var_iiibjt_dn4 = assign25060_e22640_d_n4;
        locals.var_iiibjt_dn5 = assign25060_e22640_d_n5;
        locals.var_iiibjt_dn6 = assign25060_e22640_d_n6;
        locals.var_iiibjt_dn7 = assign25060_e22640_d_n7;
        locals.var_iiibjt_dn8 = assign25060_e22640_d_n8;
        locals.var_iiibjt_dn9 = assign25060_e22640_d_n9;
        locals.var_iiibjt_dn10 = assign25060_e22640_d_n10;
        locals.var_iiibjt_dn11 = assign25060_e22640_d_n11;
        locals.var_iiibjt_dn12 = assign25060_e22640_d_n12;

        let (assign25070_e22649, assign25070_e22649_d_n3, assign25070_e22649_d_n4, assign25070_e22649_d_n5, assign25070_e22649_d_n6, assign25070_e22649_d_n7, assign25070_e22649_d_n8, assign25070_e22649_d_n9, assign25070_e22649_d_n10, assign25070_e22649_d_n11, assign25070_e22649_d_n12,) = {
    if ((locals.var_guard1323 != 0.0) && (locals.var_guard1324 == 0.0)) {
        let assign25070_e22647: f64 = (locals.var_idsmosfet + locals.var_iiibjt);
        (assign25070_e22647, (locals.var_idsmosfet_dn3 + locals.var_iiibjt_dn3), (locals.var_idsmosfet_dn4 + locals.var_iiibjt_dn4), (locals.var_idsmosfet_dn5 + locals.var_iiibjt_dn5), (locals.var_idsmosfet_dn6 + locals.var_iiibjt_dn6), (locals.var_idsmosfet_dn7 + locals.var_iiibjt_dn7), (locals.var_idsmosfet_dn8 + locals.var_iiibjt_dn8), (locals.var_idsmosfet_dn9 + locals.var_iiibjt_dn9), (locals.var_idsmosfet_dn10 + locals.var_iiibjt_dn10), (locals.var_idsmosfet_dn11 + locals.var_iiibjt_dn11), (locals.var_idsmosfet_dn12 + locals.var_iiibjt_dn12),)
    } else {
        (locals.var_iii, locals.var_iii_dn3, locals.var_iii_dn4, locals.var_iii_dn5, locals.var_iii_dn6, locals.var_iii_dn7, locals.var_iii_dn8, locals.var_iii_dn9, locals.var_iii_dn10, locals.var_iii_dn11, locals.var_iii_dn12,)
    }
};
        locals.var_iii = assign25070_e22649;
        locals.var_iii_dn3 = assign25070_e22649_d_n3;
        locals.var_iii_dn4 = assign25070_e22649_d_n4;
        locals.var_iii_dn5 = assign25070_e22649_d_n5;
        locals.var_iii_dn6 = assign25070_e22649_d_n6;
        locals.var_iii_dn7 = assign25070_e22649_d_n7;
        locals.var_iii_dn8 = assign25070_e22649_d_n8;
        locals.var_iii_dn9 = assign25070_e22649_d_n9;
        locals.var_iii_dn10 = assign25070_e22649_d_n10;
        locals.var_iii_dn11 = assign25070_e22649_d_n11;
        locals.var_iii_dn12 = assign25070_e22649_d_n12;

        let assign25080_e22656: f64 = if ((locals.var_b4soibodymod == 0.0) || (locals.var_b4soibodymod == 2.0)) { 1.0 } else { 0.0 };
        locals.var_guard1339 = assign25080_e22656;

        let (assign25090_e22662, assign25090_e22662_d_n3, assign25090_e22662_d_n4, assign25090_e22662_d_n5, assign25090_e22662_d_n6, assign25090_e22662_d_n7, assign25090_e22662_d_n8, assign25090_e22662_d_n9, assign25090_e22662_d_n10, assign25090_e22662_d_n11, assign25090_e22662_d_n12,) = {
    if ((locals.var_guard1323 != 0.0) && (locals.var_guard1339 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibp, locals.var_ibp_dn3, locals.var_ibp_dn4, locals.var_ibp_dn5, locals.var_ibp_dn6, locals.var_ibp_dn7, locals.var_ibp_dn8, locals.var_ibp_dn9, locals.var_ibp_dn10, locals.var_ibp_dn11, locals.var_ibp_dn12,)
    }
};
        locals.var_ibp = assign25090_e22662;
        locals.var_ibp_dn3 = assign25090_e22662_d_n3;
        locals.var_ibp_dn4 = assign25090_e22662_d_n4;
        locals.var_ibp_dn5 = assign25090_e22662_d_n5;
        locals.var_ibp_dn6 = assign25090_e22662_d_n6;
        locals.var_ibp_dn7 = assign25090_e22662_d_n7;
        locals.var_ibp_dn8 = assign25090_e22662_d_n8;
        locals.var_ibp_dn9 = assign25090_e22662_d_n9;
        locals.var_ibp_dn10 = assign25090_e22662_d_n10;
        locals.var_ibp_dn11 = assign25090_e22662_d_n11;
        locals.var_ibp_dn12 = assign25090_e22662_d_n12;

        let assign25100_e22665: f64 = if locals.var_pparam_b4soirbody < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard1340 = assign25100_e22665;

        let assign25110_e22668: f64 = if locals.var_b4soirbodyext <= 0.001 { 1.0 } else { 0.0 };
        locals.var_guard1341 = assign25110_e22668;

        let (assign25120_e22681, assign25120_e22681_d_n3, assign25120_e22681_d_n4, assign25120_e22681_d_n5, assign25120_e22681_d_n6, assign25120_e22681_d_n7, assign25120_e22681_d_n8, assign25120_e22681_d_n9, assign25120_e22681_d_n10, assign25120_e22681_d_n11, assign25120_e22681_d_n12,) = {
    if ((((locals.var_guard1323 != 0.0) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) && (locals.var_guard1341 != 0.0)) {
        let assign25120_e22679: f64 = (1.0 / 0.001);
        (assign25120_e22679, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign25120_e22681;
        locals.var_t0__blk808_dn3 = assign25120_e22681_d_n3;
        locals.var_t0__blk808_dn4 = assign25120_e22681_d_n4;
        locals.var_t0__blk808_dn5 = assign25120_e22681_d_n5;
        locals.var_t0__blk808_dn6 = assign25120_e22681_d_n6;
        locals.var_t0__blk808_dn7 = assign25120_e22681_d_n7;
        locals.var_t0__blk808_dn8 = assign25120_e22681_d_n8;
        locals.var_t0__blk808_dn9 = assign25120_e22681_d_n9;
        locals.var_t0__blk808_dn10 = assign25120_e22681_d_n10;
        locals.var_t0__blk808_dn11 = assign25120_e22681_d_n11;
        locals.var_t0__blk808_dn12 = assign25120_e22681_d_n12;

        let (assign25130_e22695, assign25130_e22695_d_n3, assign25130_e22695_d_n4, assign25130_e22695_d_n5, assign25130_e22695_d_n6, assign25130_e22695_d_n7, assign25130_e22695_d_n8, assign25130_e22695_d_n9, assign25130_e22695_d_n10, assign25130_e22695_d_n11, assign25130_e22695_d_n12,) = {
    if ((((locals.var_guard1323 != 0.0) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) && (locals.var_guard1341 == 0.0)) {
        let assign25130_e22693: f64 = (1.0 / locals.var_b4soirbodyext);
        (assign25130_e22693, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign25130_e22695;
        locals.var_t0__blk808_dn3 = assign25130_e22695_d_n3;
        locals.var_t0__blk808_dn4 = assign25130_e22695_d_n4;
        locals.var_t0__blk808_dn5 = assign25130_e22695_d_n5;
        locals.var_t0__blk808_dn6 = assign25130_e22695_d_n6;
        locals.var_t0__blk808_dn7 = assign25130_e22695_d_n7;
        locals.var_t0__blk808_dn8 = assign25130_e22695_d_n8;
        locals.var_t0__blk808_dn9 = assign25130_e22695_d_n9;
        locals.var_t0__blk808_dn10 = assign25130_e22695_d_n10;
        locals.var_t0__blk808_dn11 = assign25130_e22695_d_n11;
        locals.var_t0__blk808_dn12 = assign25130_e22695_d_n12;

        let (assign25140_e22706, assign25140_e22706_d_n3, assign25140_e22706_d_n4, assign25140_e22706_d_n5, assign25140_e22706_d_n6, assign25140_e22706_d_n7, assign25140_e22706_d_n8, assign25140_e22706_d_n9, assign25140_e22706_d_n10, assign25140_e22706_d_n11, assign25140_e22706_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 != 0.0)) {
        let assign25140_e22704: f64 = (locals.var_vbp * locals.var_t0__blk808);
        (assign25140_e22704, (locals.var_vbp * locals.var_t0__blk808_dn3), ((locals.var_vbp_dn4 * locals.var_t0__blk808) + (locals.var_vbp * locals.var_t0__blk808_dn4)), ((locals.var_vbp_dn5 * locals.var_t0__blk808) + (locals.var_vbp * locals.var_t0__blk808_dn5)), (locals.var_vbp * locals.var_t0__blk808_dn6), (locals.var_vbp * locals.var_t0__blk808_dn7), (locals.var_vbp * locals.var_t0__blk808_dn8), (locals.var_vbp * locals.var_t0__blk808_dn9), (locals.var_vbp * locals.var_t0__blk808_dn10), (locals.var_vbp * locals.var_t0__blk808_dn11), (locals.var_vbp * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_ibp, locals.var_ibp_dn3, locals.var_ibp_dn4, locals.var_ibp_dn5, locals.var_ibp_dn6, locals.var_ibp_dn7, locals.var_ibp_dn8, locals.var_ibp_dn9, locals.var_ibp_dn10, locals.var_ibp_dn11, locals.var_ibp_dn12,)
    }
};
        locals.var_ibp = assign25140_e22706;
        locals.var_ibp_dn3 = assign25140_e22706_d_n3;
        locals.var_ibp_dn4 = assign25140_e22706_d_n4;
        locals.var_ibp_dn5 = assign25140_e22706_d_n5;
        locals.var_ibp_dn6 = assign25140_e22706_d_n6;
        locals.var_ibp_dn7 = assign25140_e22706_d_n7;
        locals.var_ibp_dn8 = assign25140_e22706_d_n8;
        locals.var_ibp_dn9 = assign25140_e22706_d_n9;
        locals.var_ibp_dn10 = assign25140_e22706_d_n10;
        locals.var_ibp_dn11 = assign25140_e22706_d_n11;
        locals.var_ibp_dn12 = assign25140_e22706_d_n12;

    }

    pub(super) fn stamp_transient_block_75(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25150_e22720, assign25150_e22720_d_n3, assign25150_e22720_d_n4, assign25150_e22720_d_n5, assign25150_e22720_d_n6, assign25150_e22720_d_n7, assign25150_e22720_d_n8, assign25150_e22720_d_n9, assign25150_e22720_d_n10, assign25150_e22720_d_n11, assign25150_e22720_d_n12,) = {
    if (((locals.var_guard1323 != 0.0) && (locals.var_guard1339 == 0.0)) && (locals.var_guard1340 == 0.0)) {
        let assign25150_e22717: f64 = (locals.var_pparam_b4soirbody + locals.var_b4soirbodyext);
        let assign25150_e22718: f64 = (locals.var_vbp / assign25150_e22717);
        (assign25150_e22718, (-((locals.var_vbp * locals.var_pparam_b4soirbody_dn3) / (assign25150_e22717 * assign25150_e22717))), (((locals.var_vbp_dn4 * assign25150_e22717) - (locals.var_vbp * locals.var_pparam_b4soirbody_dn4)) / (assign25150_e22717 * assign25150_e22717)), (((locals.var_vbp_dn5 * assign25150_e22717) - (locals.var_vbp * locals.var_pparam_b4soirbody_dn5)) / (assign25150_e22717 * assign25150_e22717)), (-((locals.var_vbp * locals.var_pparam_b4soirbody_dn6) / (assign25150_e22717 * assign25150_e22717))), (-((locals.var_vbp * locals.var_pparam_b4soirbody_dn7) / (assign25150_e22717 * assign25150_e22717))), (-((locals.var_vbp * locals.var_pparam_b4soirbody_dn8) / (assign25150_e22717 * assign25150_e22717))), (-((locals.var_vbp * locals.var_pparam_b4soirbody_dn9) / (assign25150_e22717 * assign25150_e22717))), (-((locals.var_vbp * locals.var_pparam_b4soirbody_dn10) / (assign25150_e22717 * assign25150_e22717))), (-((locals.var_vbp * locals.var_pparam_b4soirbody_dn11) / (assign25150_e22717 * assign25150_e22717))), (-((locals.var_vbp * locals.var_pparam_b4soirbody_dn12) / (assign25150_e22717 * assign25150_e22717))),)
    } else {
        (locals.var_ibp, locals.var_ibp_dn3, locals.var_ibp_dn4, locals.var_ibp_dn5, locals.var_ibp_dn6, locals.var_ibp_dn7, locals.var_ibp_dn8, locals.var_ibp_dn9, locals.var_ibp_dn10, locals.var_ibp_dn11, locals.var_ibp_dn12,)
    }
};
        locals.var_ibp = assign25150_e22720;
        locals.var_ibp_dn3 = assign25150_e22720_d_n3;
        locals.var_ibp_dn4 = assign25150_e22720_d_n4;
        locals.var_ibp_dn5 = assign25150_e22720_d_n5;
        locals.var_ibp_dn6 = assign25150_e22720_d_n6;
        locals.var_ibp_dn7 = assign25150_e22720_d_n7;
        locals.var_ibp_dn8 = assign25150_e22720_d_n8;
        locals.var_ibp_dn9 = assign25150_e22720_d_n9;
        locals.var_ibp_dn10 = assign25150_e22720_d_n10;
        locals.var_ibp_dn11 = assign25150_e22720_d_n11;
        locals.var_ibp_dn12 = assign25150_e22720_d_n12;

        let (assign25160_e22725, assign25160_e22725_d_n3, assign25160_e22725_d_n4, assign25160_e22725_d_n5, assign25160_e22725_d_n6, assign25160_e22725_d_n7, assign25160_e22725_d_n8, assign25160_e22725_d_n9, assign25160_e22725_d_n10, assign25160_e22725_d_n11, assign25160_e22725_d_n12,) = {
    if (locals.var_guard1323 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_iii, locals.var_iii_dn3, locals.var_iii_dn4, locals.var_iii_dn5, locals.var_iii_dn6, locals.var_iii_dn7, locals.var_iii_dn8, locals.var_iii_dn9, locals.var_iii_dn10, locals.var_iii_dn11, locals.var_iii_dn12,)
    }
};
        locals.var_iii = assign25160_e22725;
        locals.var_iii_dn3 = assign25160_e22725_d_n3;
        locals.var_iii_dn4 = assign25160_e22725_d_n4;
        locals.var_iii_dn5 = assign25160_e22725_d_n5;
        locals.var_iii_dn6 = assign25160_e22725_d_n6;
        locals.var_iii_dn7 = assign25160_e22725_d_n7;
        locals.var_iii_dn8 = assign25160_e22725_d_n8;
        locals.var_iii_dn9 = assign25160_e22725_d_n9;
        locals.var_iii_dn10 = assign25160_e22725_d_n10;
        locals.var_iii_dn11 = assign25160_e22725_d_n11;
        locals.var_iii_dn12 = assign25160_e22725_d_n12;

        let (assign25170_e22730, assign25170_e22730_d_n3, assign25170_e22730_d_n4, assign25170_e22730_d_n5, assign25170_e22730_d_n6, assign25170_e22730_d_n7, assign25170_e22730_d_n8, assign25170_e22730_d_n9, assign25170_e22730_d_n10, assign25170_e22730_d_n11, assign25170_e22730_d_n12,) = {
    if (locals.var_guard1323 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ibp, locals.var_ibp_dn3, locals.var_ibp_dn4, locals.var_ibp_dn5, locals.var_ibp_dn6, locals.var_ibp_dn7, locals.var_ibp_dn8, locals.var_ibp_dn9, locals.var_ibp_dn10, locals.var_ibp_dn11, locals.var_ibp_dn12,)
    }
};
        locals.var_ibp = assign25170_e22730;
        locals.var_ibp_dn3 = assign25170_e22730_d_n3;
        locals.var_ibp_dn4 = assign25170_e22730_d_n4;
        locals.var_ibp_dn5 = assign25170_e22730_d_n5;
        locals.var_ibp_dn6 = assign25170_e22730_d_n6;
        locals.var_ibp_dn7 = assign25170_e22730_d_n7;
        locals.var_ibp_dn8 = assign25170_e22730_d_n8;
        locals.var_ibp_dn9 = assign25170_e22730_d_n9;
        locals.var_ibp_dn10 = assign25170_e22730_d_n10;
        locals.var_ibp_dn11 = assign25170_e22730_d_n11;
        locals.var_ibp_dn12 = assign25170_e22730_d_n12;

        let assign25180_e22733: f64 = if p.p39 > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1342 = assign25180_e22733;

        let (assign25190_e22739, assign25190_e22739_d_n3, assign25190_e22739_d_n4, assign25190_e22739_d_n5, assign25190_e22739_d_n6, assign25190_e22739_d_n7, assign25190_e22739_d_n8, assign25190_e22739_d_n9, assign25190_e22739_d_n10, assign25190_e22739_d_n11, assign25190_e22739_d_n12,) = {
    if (locals.var_guard1342 != 0.0) {
        let assign25190_e22737: f64 = (locals.var_pparam_b4soixrcrg2 * locals.var_b4soivtm);
        (assign25190_e22737, (locals.var_pparam_b4soixrcrg2_dn3 * locals.var_b4soivtm), ((locals.var_pparam_b4soixrcrg2_dn4 * locals.var_b4soivtm) + (locals.var_pparam_b4soixrcrg2 * locals.var_b4soivtm_dn4)), ((locals.var_pparam_b4soixrcrg2_dn5 * locals.var_b4soivtm) + (locals.var_pparam_b4soixrcrg2 * locals.var_b4soivtm_dn5)), ((locals.var_pparam_b4soixrcrg2_dn6 * locals.var_b4soivtm) + (locals.var_pparam_b4soixrcrg2 * locals.var_b4soivtm_dn6)), (locals.var_pparam_b4soixrcrg2_dn7 * locals.var_b4soivtm), (locals.var_pparam_b4soixrcrg2_dn8 * locals.var_b4soivtm), (locals.var_pparam_b4soixrcrg2_dn9 * locals.var_b4soivtm), (locals.var_pparam_b4soixrcrg2_dn10 * locals.var_b4soivtm), (locals.var_pparam_b4soixrcrg2_dn11 * locals.var_b4soivtm), (locals.var_pparam_b4soixrcrg2_dn12 * locals.var_b4soivtm),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11, locals.var_t9_dn12,)
    }
};
        locals.var_t9 = assign25190_e22739;
        locals.var_t9_dn3 = assign25190_e22739_d_n3;
        locals.var_t9_dn4 = assign25190_e22739_d_n4;
        locals.var_t9_dn5 = assign25190_e22739_d_n5;
        locals.var_t9_dn6 = assign25190_e22739_d_n6;
        locals.var_t9_dn7 = assign25190_e22739_d_n7;
        locals.var_t9_dn8 = assign25190_e22739_d_n8;
        locals.var_t9_dn9 = assign25190_e22739_d_n9;
        locals.var_t9_dn10 = assign25190_e22739_d_n10;
        locals.var_t9_dn11 = assign25190_e22739_d_n11;
        locals.var_t9_dn12 = assign25190_e22739_d_n12;

        let (assign25200_e22745, assign25200_e22745_d_n3, assign25200_e22745_d_n4, assign25200_e22745_d_n5, assign25200_e22745_d_n6, assign25200_e22745_d_n7, assign25200_e22745_d_n8, assign25200_e22745_d_n9, assign25200_e22745_d_n10, assign25200_e22745_d_n11, assign25200_e22745_d_n12,) = {
    if (locals.var_guard1342 != 0.0) {
        let assign25200_e22743: f64 = (locals.var_t9 * locals.var_beta);
        (assign25200_e22743, ((locals.var_t9_dn3 * locals.var_beta) + (locals.var_t9 * locals.var_beta_dn3)), ((locals.var_t9_dn4 * locals.var_beta) + (locals.var_t9 * locals.var_beta_dn4)), ((locals.var_t9_dn5 * locals.var_beta) + (locals.var_t9 * locals.var_beta_dn5)), ((locals.var_t9_dn6 * locals.var_beta) + (locals.var_t9 * locals.var_beta_dn6)), ((locals.var_t9_dn7 * locals.var_beta) + (locals.var_t9 * locals.var_beta_dn7)), ((locals.var_t9_dn8 * locals.var_beta) + (locals.var_t9 * locals.var_beta_dn8)), ((locals.var_t9_dn9 * locals.var_beta) + (locals.var_t9 * locals.var_beta_dn9)), ((locals.var_t9_dn10 * locals.var_beta) + (locals.var_t9 * locals.var_beta_dn10)), ((locals.var_t9_dn11 * locals.var_beta) + (locals.var_t9 * locals.var_beta_dn11)), ((locals.var_t9_dn12 * locals.var_beta) + (locals.var_t9 * locals.var_beta_dn12)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign25200_e22745;
        locals.var_t0__blk808_dn3 = assign25200_e22745_d_n3;
        locals.var_t0__blk808_dn4 = assign25200_e22745_d_n4;
        locals.var_t0__blk808_dn5 = assign25200_e22745_d_n5;
        locals.var_t0__blk808_dn6 = assign25200_e22745_d_n6;
        locals.var_t0__blk808_dn7 = assign25200_e22745_d_n7;
        locals.var_t0__blk808_dn8 = assign25200_e22745_d_n8;
        locals.var_t0__blk808_dn9 = assign25200_e22745_d_n9;
        locals.var_t0__blk808_dn10 = assign25200_e22745_d_n10;
        locals.var_t0__blk808_dn11 = assign25200_e22745_d_n11;
        locals.var_t0__blk808_dn12 = assign25200_e22745_d_n12;

        let (assign25210_e22753, assign25210_e22753_d_n3, assign25210_e22753_d_n4, assign25210_e22753_d_n5, assign25210_e22753_d_n6, assign25210_e22753_d_n7, assign25210_e22753_d_n8, assign25210_e22753_d_n9, assign25210_e22753_d_n10, assign25210_e22753_d_n11, assign25210_e22753_d_n12,) = {
    if (locals.var_guard1342 != 0.0) {
        let assign25210_e22750: f64 = (locals.var_t0__blk808 + locals.var_idovvds);
        let assign25210_e22751: f64 = (locals.var_pparam_b4soixrcrg1 * assign25210_e22750);
        (assign25210_e22751, ((locals.var_pparam_b4soixrcrg1_dn3 * assign25210_e22750) + (locals.var_pparam_b4soixrcrg1 * (locals.var_t0__blk808_dn3 + locals.var_idovvds_dn3))), ((locals.var_pparam_b4soixrcrg1_dn4 * assign25210_e22750) + (locals.var_pparam_b4soixrcrg1 * (locals.var_t0__blk808_dn4 + locals.var_idovvds_dn4))), ((locals.var_pparam_b4soixrcrg1_dn5 * assign25210_e22750) + (locals.var_pparam_b4soixrcrg1 * (locals.var_t0__blk808_dn5 + locals.var_idovvds_dn5))), ((locals.var_pparam_b4soixrcrg1_dn6 * assign25210_e22750) + (locals.var_pparam_b4soixrcrg1 * (locals.var_t0__blk808_dn6 + locals.var_idovvds_dn6))), ((locals.var_pparam_b4soixrcrg1_dn7 * assign25210_e22750) + (locals.var_pparam_b4soixrcrg1 * (locals.var_t0__blk808_dn7 + locals.var_idovvds_dn7))), ((locals.var_pparam_b4soixrcrg1_dn8 * assign25210_e22750) + (locals.var_pparam_b4soixrcrg1 * (locals.var_t0__blk808_dn8 + locals.var_idovvds_dn8))), ((locals.var_pparam_b4soixrcrg1_dn9 * assign25210_e22750) + (locals.var_pparam_b4soixrcrg1 * (locals.var_t0__blk808_dn9 + locals.var_idovvds_dn9))), ((locals.var_pparam_b4soixrcrg1_dn10 * assign25210_e22750) + (locals.var_pparam_b4soixrcrg1 * (locals.var_t0__blk808_dn10 + locals.var_idovvds_dn10))), ((locals.var_pparam_b4soixrcrg1_dn11 * assign25210_e22750) + (locals.var_pparam_b4soixrcrg1 * (locals.var_t0__blk808_dn11 + locals.var_idovvds_dn11))), ((locals.var_pparam_b4soixrcrg1_dn12 * assign25210_e22750) + (locals.var_pparam_b4soixrcrg1 * (locals.var_t0__blk808_dn12 + locals.var_idovvds_dn12))),)
    } else {
        (locals.var_b4soigcrg, locals.var_b4soigcrg_dn3, locals.var_b4soigcrg_dn4, locals.var_b4soigcrg_dn5, locals.var_b4soigcrg_dn6, locals.var_b4soigcrg_dn7, locals.var_b4soigcrg_dn8, locals.var_b4soigcrg_dn9, locals.var_b4soigcrg_dn10, locals.var_b4soigcrg_dn11, locals.var_b4soigcrg_dn12,)
    }
};
        locals.var_b4soigcrg = assign25210_e22753;
        locals.var_b4soigcrg_dn3 = assign25210_e22753_d_n3;
        locals.var_b4soigcrg_dn4 = assign25210_e22753_d_n4;
        locals.var_b4soigcrg_dn5 = assign25210_e22753_d_n5;
        locals.var_b4soigcrg_dn6 = assign25210_e22753_d_n6;
        locals.var_b4soigcrg_dn7 = assign25210_e22753_d_n7;
        locals.var_b4soigcrg_dn8 = assign25210_e22753_d_n8;
        locals.var_b4soigcrg_dn9 = assign25210_e22753_d_n9;
        locals.var_b4soigcrg_dn10 = assign25210_e22753_d_n10;
        locals.var_b4soigcrg_dn11 = assign25210_e22753_d_n11;
        locals.var_b4soigcrg_dn12 = assign25210_e22753_d_n12;

        let assign25220_e22756: f64 = if p.p3 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1343 = assign25220_e22756;

        let (assign25230_e22764, assign25230_e22764_d_n3, assign25230_e22764_d_n4, assign25230_e22764_d_n5, assign25230_e22764_d_n6, assign25230_e22764_d_n7, assign25230_e22764_d_n8, assign25230_e22764_d_n9, assign25230_e22764_d_n10, assign25230_e22764_d_n11, assign25230_e22764_d_n12,) = {
    if ((locals.var_guard1342 != 0.0) && (locals.var_guard1343 != 0.0)) {
        let assign25230_e22762: f64 = (locals.var_b4soigcrg * p.p3);
        (assign25230_e22762, (locals.var_b4soigcrg_dn3 * p.p3), (locals.var_b4soigcrg_dn4 * p.p3), (locals.var_b4soigcrg_dn5 * p.p3), (locals.var_b4soigcrg_dn6 * p.p3), (locals.var_b4soigcrg_dn7 * p.p3), (locals.var_b4soigcrg_dn8 * p.p3), (locals.var_b4soigcrg_dn9 * p.p3), (locals.var_b4soigcrg_dn10 * p.p3), (locals.var_b4soigcrg_dn11 * p.p3), (locals.var_b4soigcrg_dn12 * p.p3),)
    } else {
        (locals.var_b4soigcrg, locals.var_b4soigcrg_dn3, locals.var_b4soigcrg_dn4, locals.var_b4soigcrg_dn5, locals.var_b4soigcrg_dn6, locals.var_b4soigcrg_dn7, locals.var_b4soigcrg_dn8, locals.var_b4soigcrg_dn9, locals.var_b4soigcrg_dn10, locals.var_b4soigcrg_dn11, locals.var_b4soigcrg_dn12,)
    }
};
        locals.var_b4soigcrg = assign25230_e22764;
        locals.var_b4soigcrg_dn3 = assign25230_e22764_d_n3;
        locals.var_b4soigcrg_dn4 = assign25230_e22764_d_n4;
        locals.var_b4soigcrg_dn5 = assign25230_e22764_d_n5;
        locals.var_b4soigcrg_dn6 = assign25230_e22764_d_n6;
        locals.var_b4soigcrg_dn7 = assign25230_e22764_d_n7;
        locals.var_b4soigcrg_dn8 = assign25230_e22764_d_n8;
        locals.var_b4soigcrg_dn9 = assign25230_e22764_d_n9;
        locals.var_b4soigcrg_dn10 = assign25230_e22764_d_n10;
        locals.var_b4soigcrg_dn11 = assign25230_e22764_d_n11;
        locals.var_b4soigcrg_dn12 = assign25230_e22764_d_n12;

        let assign25240_e22767: f64 = if p.p39 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1344 = assign25240_e22767;

        let (assign25250_e22775, assign25250_e22775_d_n3, assign25250_e22775_d_n4, assign25250_e22775_d_n5, assign25250_e22775_d_n6, assign25250_e22775_d_n7, assign25250_e22775_d_n8, assign25250_e22775_d_n9, assign25250_e22775_d_n10, assign25250_e22775_d_n11, assign25250_e22775_d_n12,) = {
    if ((locals.var_guard1342 != 0.0) && (locals.var_guard1344 != 0.0)) {
        let assign25250_e22773: f64 = (locals.var_b4soigrgeltd + locals.var_b4soigcrg);
        (assign25250_e22773, (locals.var_b4soigrgeltd_dn3 + locals.var_b4soigcrg_dn3), (locals.var_b4soigrgeltd_dn4 + locals.var_b4soigcrg_dn4), (locals.var_b4soigrgeltd_dn5 + locals.var_b4soigcrg_dn5), (locals.var_b4soigrgeltd_dn6 + locals.var_b4soigcrg_dn6), (locals.var_b4soigrgeltd_dn7 + locals.var_b4soigcrg_dn7), (locals.var_b4soigrgeltd_dn8 + locals.var_b4soigcrg_dn8), (locals.var_b4soigrgeltd_dn9 + locals.var_b4soigcrg_dn9), (locals.var_b4soigrgeltd_dn10 + locals.var_b4soigcrg_dn10), (locals.var_b4soigrgeltd_dn11 + locals.var_b4soigcrg_dn11), (locals.var_b4soigrgeltd_dn12 + locals.var_b4soigcrg_dn12),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11, locals.var_t11_dn12,)
    }
};
        locals.var_t11 = assign25250_e22775;
        locals.var_t11_dn3 = assign25250_e22775_d_n3;
        locals.var_t11_dn4 = assign25250_e22775_d_n4;
        locals.var_t11_dn5 = assign25250_e22775_d_n5;
        locals.var_t11_dn6 = assign25250_e22775_d_n6;
        locals.var_t11_dn7 = assign25250_e22775_d_n7;
        locals.var_t11_dn8 = assign25250_e22775_d_n8;
        locals.var_t11_dn9 = assign25250_e22775_d_n9;
        locals.var_t11_dn10 = assign25250_e22775_d_n10;
        locals.var_t11_dn11 = assign25250_e22775_d_n11;
        locals.var_t11_dn12 = assign25250_e22775_d_n12;

        let (assign25260_e22785, assign25260_e22785_d_n3, assign25260_e22785_d_n4, assign25260_e22785_d_n5, assign25260_e22785_d_n6, assign25260_e22785_d_n7, assign25260_e22785_d_n8, assign25260_e22785_d_n9, assign25260_e22785_d_n10, assign25260_e22785_d_n11, assign25260_e22785_d_n12,) = {
    if ((locals.var_guard1342 != 0.0) && (locals.var_guard1344 != 0.0)) {
        let assign25260_e22781: f64 = (locals.var_b4soigrgeltd * locals.var_b4soigcrg);
        let assign25260_e22783: f64 = (assign25260_e22781 / locals.var_t11);
        (assign25260_e22783, (((((locals.var_b4soigrgeltd_dn3 * locals.var_b4soigcrg) + (locals.var_b4soigrgeltd * locals.var_b4soigcrg_dn3)) * locals.var_t11) - (assign25260_e22781 * locals.var_t11_dn3)) / (locals.var_t11 * locals.var_t11)), (((((locals.var_b4soigrgeltd_dn4 * locals.var_b4soigcrg) + (locals.var_b4soigrgeltd * locals.var_b4soigcrg_dn4)) * locals.var_t11) - (assign25260_e22781 * locals.var_t11_dn4)) / (locals.var_t11 * locals.var_t11)), (((((locals.var_b4soigrgeltd_dn5 * locals.var_b4soigcrg) + (locals.var_b4soigrgeltd * locals.var_b4soigcrg_dn5)) * locals.var_t11) - (assign25260_e22781 * locals.var_t11_dn5)) / (locals.var_t11 * locals.var_t11)), (((((locals.var_b4soigrgeltd_dn6 * locals.var_b4soigcrg) + (locals.var_b4soigrgeltd * locals.var_b4soigcrg_dn6)) * locals.var_t11) - (assign25260_e22781 * locals.var_t11_dn6)) / (locals.var_t11 * locals.var_t11)), (((((locals.var_b4soigrgeltd_dn7 * locals.var_b4soigcrg) + (locals.var_b4soigrgeltd * locals.var_b4soigcrg_dn7)) * locals.var_t11) - (assign25260_e22781 * locals.var_t11_dn7)) / (locals.var_t11 * locals.var_t11)), (((((locals.var_b4soigrgeltd_dn8 * locals.var_b4soigcrg) + (locals.var_b4soigrgeltd * locals.var_b4soigcrg_dn8)) * locals.var_t11) - (assign25260_e22781 * locals.var_t11_dn8)) / (locals.var_t11 * locals.var_t11)), (((((locals.var_b4soigrgeltd_dn9 * locals.var_b4soigcrg) + (locals.var_b4soigrgeltd * locals.var_b4soigcrg_dn9)) * locals.var_t11) - (assign25260_e22781 * locals.var_t11_dn9)) / (locals.var_t11 * locals.var_t11)), (((((locals.var_b4soigrgeltd_dn10 * locals.var_b4soigcrg) + (locals.var_b4soigrgeltd * locals.var_b4soigcrg_dn10)) * locals.var_t11) - (assign25260_e22781 * locals.var_t11_dn10)) / (locals.var_t11 * locals.var_t11)), (((((locals.var_b4soigrgeltd_dn11 * locals.var_b4soigcrg) + (locals.var_b4soigrgeltd * locals.var_b4soigcrg_dn11)) * locals.var_t11) - (assign25260_e22781 * locals.var_t11_dn11)) / (locals.var_t11 * locals.var_t11)), (((((locals.var_b4soigrgeltd_dn12 * locals.var_b4soigcrg) + (locals.var_b4soigrgeltd * locals.var_b4soigcrg_dn12)) * locals.var_t11) - (assign25260_e22781 * locals.var_t11_dn12)) / (locals.var_t11 * locals.var_t11)),)
    } else {
        (locals.var_b4soigcrg, locals.var_b4soigcrg_dn3, locals.var_b4soigcrg_dn4, locals.var_b4soigcrg_dn5, locals.var_b4soigcrg_dn6, locals.var_b4soigcrg_dn7, locals.var_b4soigcrg_dn8, locals.var_b4soigcrg_dn9, locals.var_b4soigcrg_dn10, locals.var_b4soigcrg_dn11, locals.var_b4soigcrg_dn12,)
    }
};
        locals.var_b4soigcrg = assign25260_e22785;
        locals.var_b4soigcrg_dn3 = assign25260_e22785_d_n3;
        locals.var_b4soigcrg_dn4 = assign25260_e22785_d_n4;
        locals.var_b4soigcrg_dn5 = assign25260_e22785_d_n5;
        locals.var_b4soigcrg_dn6 = assign25260_e22785_d_n6;
        locals.var_b4soigcrg_dn7 = assign25260_e22785_d_n7;
        locals.var_b4soigcrg_dn8 = assign25260_e22785_d_n8;
        locals.var_b4soigcrg_dn9 = assign25260_e22785_d_n9;
        locals.var_b4soigcrg_dn10 = assign25260_e22785_d_n10;
        locals.var_b4soigcrg_dn11 = assign25260_e22785_d_n11;
        locals.var_b4soigcrg_dn12 = assign25260_e22785_d_n12;

        let (assign25270_e22790, assign25270_e22790_d_n3, assign25270_e22790_d_n4, assign25270_e22790_d_n5, assign25270_e22790_d_n6, assign25270_e22790_d_n7, assign25270_e22790_d_n8, assign25270_e22790_d_n9, assign25270_e22790_d_n10, assign25270_e22790_d_n11, assign25270_e22790_d_n12,) = {
    if (locals.var_guard1342 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soigcrg, locals.var_b4soigcrg_dn3, locals.var_b4soigcrg_dn4, locals.var_b4soigcrg_dn5, locals.var_b4soigcrg_dn6, locals.var_b4soigcrg_dn7, locals.var_b4soigcrg_dn8, locals.var_b4soigcrg_dn9, locals.var_b4soigcrg_dn10, locals.var_b4soigcrg_dn11, locals.var_b4soigcrg_dn12,)
    }
};
        locals.var_b4soigcrg = assign25270_e22790;
        locals.var_b4soigcrg_dn3 = assign25270_e22790_d_n3;
        locals.var_b4soigcrg_dn4 = assign25270_e22790_d_n4;
        locals.var_b4soigcrg_dn5 = assign25270_e22790_d_n5;
        locals.var_b4soigcrg_dn6 = assign25270_e22790_d_n6;
        locals.var_b4soigcrg_dn7 = assign25270_e22790_d_n7;
        locals.var_b4soigcrg_dn8 = assign25270_e22790_d_n8;
        locals.var_b4soigcrg_dn9 = assign25270_e22790_d_n9;
        locals.var_b4soigcrg_dn10 = assign25270_e22790_d_n10;
        locals.var_b4soigcrg_dn11 = assign25270_e22790_d_n11;
        locals.var_b4soigcrg_dn12 = assign25270_e22790_d_n12;

        let assign25280_e22793: f64 = if p.p429 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1345 = assign25280_e22793;

        let assign25290_e22796: f64 = (locals.var_b4soisourceresistance + p.p135);
        let assign25290_e22798: f64 = if assign25290_e22796 > p.p431 { 1.0 } else { 0.0 };
        locals.var_guard1346 = assign25290_e22798;

        let (assign25300_e22806, assign25300_e22806_d_n3, assign25300_e22806_d_n4, assign25300_e22806_d_n5, assign25300_e22806_d_n6, assign25300_e22806_d_n7, assign25300_e22806_d_n8, assign25300_e22806_d_n9, assign25300_e22806_d_n10, assign25300_e22806_d_n11, assign25300_e22806_d_n12,) = {
    if ((locals.var_guard1345 != 0.0) && (locals.var_guard1346 != 0.0)) {
        let assign25300_e22804: f64 = (locals.var_b4soisourceresistance + locals.var_rsc_t);
        (assign25300_e22804, 0.0, locals.var_rsc_t_dn4, locals.var_rsc_t_dn5, locals.var_rsc_t_dn6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rs, locals.var_rs_dn3, locals.var_rs_dn4, locals.var_rs_dn5, locals.var_rs_dn6, locals.var_rs_dn7, locals.var_rs_dn8, locals.var_rs_dn9, locals.var_rs_dn10, locals.var_rs_dn11, locals.var_rs_dn12,)
    }
};
        locals.var_rs = assign25300_e22806;
        locals.var_rs_dn3 = assign25300_e22806_d_n3;
        locals.var_rs_dn4 = assign25300_e22806_d_n4;
        locals.var_rs_dn5 = assign25300_e22806_d_n5;
        locals.var_rs_dn6 = assign25300_e22806_d_n6;
        locals.var_rs_dn7 = assign25300_e22806_d_n7;
        locals.var_rs_dn8 = assign25300_e22806_d_n8;
        locals.var_rs_dn9 = assign25300_e22806_d_n9;
        locals.var_rs_dn10 = assign25300_e22806_d_n10;
        locals.var_rs_dn11 = assign25300_e22806_d_n11;
        locals.var_rs_dn12 = assign25300_e22806_d_n12;

        let assign25310_e22809: f64 = if locals.var_rs < p.p431 { 1.0 } else { 0.0 };
        locals.var_guard1347 = assign25310_e22809;

        let (assign25320_e22817, assign25320_e22817_d_n3, assign25320_e22817_d_n4, assign25320_e22817_d_n5, assign25320_e22817_d_n6, assign25320_e22817_d_n7, assign25320_e22817_d_n8, assign25320_e22817_d_n9, assign25320_e22817_d_n10, assign25320_e22817_d_n11, assign25320_e22817_d_n12,) = {
    if (((locals.var_guard1345 != 0.0) && (locals.var_guard1346 != 0.0)) && (locals.var_guard1347 != 0.0)) {
        (p.p431, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rs, locals.var_rs_dn3, locals.var_rs_dn4, locals.var_rs_dn5, locals.var_rs_dn6, locals.var_rs_dn7, locals.var_rs_dn8, locals.var_rs_dn9, locals.var_rs_dn10, locals.var_rs_dn11, locals.var_rs_dn12,)
    }
};
        locals.var_rs = assign25320_e22817;
        locals.var_rs_dn3 = assign25320_e22817_d_n3;
        locals.var_rs_dn4 = assign25320_e22817_d_n4;
        locals.var_rs_dn5 = assign25320_e22817_d_n5;
        locals.var_rs_dn6 = assign25320_e22817_d_n6;
        locals.var_rs_dn7 = assign25320_e22817_d_n7;
        locals.var_rs_dn8 = assign25320_e22817_d_n8;
        locals.var_rs_dn9 = assign25320_e22817_d_n9;
        locals.var_rs_dn10 = assign25320_e22817_d_n10;
        locals.var_rs_dn11 = assign25320_e22817_d_n11;
        locals.var_rs_dn12 = assign25320_e22817_d_n12;

        let (assign25330_e22824, assign25330_e22824_d_n3, assign25330_e22824_d_n4, assign25330_e22824_d_n5, assign25330_e22824_d_n6, assign25330_e22824_d_n7, assign25330_e22824_d_n8, assign25330_e22824_d_n9, assign25330_e22824_d_n10, assign25330_e22824_d_n11, assign25330_e22824_d_n12,) = {
    if ((locals.var_guard1345 != 0.0) && (locals.var_guard1346 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rs, locals.var_rs_dn3, locals.var_rs_dn4, locals.var_rs_dn5, locals.var_rs_dn6, locals.var_rs_dn7, locals.var_rs_dn8, locals.var_rs_dn9, locals.var_rs_dn10, locals.var_rs_dn11, locals.var_rs_dn12,)
    }
};
        locals.var_rs = assign25330_e22824;
        locals.var_rs_dn3 = assign25330_e22824_d_n3;
        locals.var_rs_dn4 = assign25330_e22824_d_n4;
        locals.var_rs_dn5 = assign25330_e22824_d_n5;
        locals.var_rs_dn6 = assign25330_e22824_d_n6;
        locals.var_rs_dn7 = assign25330_e22824_d_n7;
        locals.var_rs_dn8 = assign25330_e22824_d_n8;
        locals.var_rs_dn9 = assign25330_e22824_d_n9;
        locals.var_rs_dn10 = assign25330_e22824_d_n10;
        locals.var_rs_dn11 = assign25330_e22824_d_n11;
        locals.var_rs_dn12 = assign25330_e22824_d_n12;

        let assign25340_e22827: f64 = (locals.var_b4soidrainresistance + p.p136);
        let assign25340_e22829: f64 = if assign25340_e22827 > p.p431 { 1.0 } else { 0.0 };
        locals.var_guard1348 = assign25340_e22829;

        let (assign25350_e22837, assign25350_e22837_d_n3, assign25350_e22837_d_n4, assign25350_e22837_d_n5, assign25350_e22837_d_n6, assign25350_e22837_d_n7, assign25350_e22837_d_n8, assign25350_e22837_d_n9, assign25350_e22837_d_n10, assign25350_e22837_d_n11, assign25350_e22837_d_n12,) = {
    if ((locals.var_guard1345 != 0.0) && (locals.var_guard1348 != 0.0)) {
        let assign25350_e22835: f64 = (locals.var_b4soidrainresistance + locals.var_rdc_t);
        (assign25350_e22835, 0.0, locals.var_rdc_t_dn4, locals.var_rdc_t_dn5, locals.var_rdc_t_dn6, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rd, locals.var_rd_dn3, locals.var_rd_dn4, locals.var_rd_dn5, locals.var_rd_dn6, locals.var_rd_dn7, locals.var_rd_dn8, locals.var_rd_dn9, locals.var_rd_dn10, locals.var_rd_dn11, locals.var_rd_dn12,)
    }
};
        locals.var_rd = assign25350_e22837;
        locals.var_rd_dn3 = assign25350_e22837_d_n3;
        locals.var_rd_dn4 = assign25350_e22837_d_n4;
        locals.var_rd_dn5 = assign25350_e22837_d_n5;
        locals.var_rd_dn6 = assign25350_e22837_d_n6;
        locals.var_rd_dn7 = assign25350_e22837_d_n7;
        locals.var_rd_dn8 = assign25350_e22837_d_n8;
        locals.var_rd_dn9 = assign25350_e22837_d_n9;
        locals.var_rd_dn10 = assign25350_e22837_d_n10;
        locals.var_rd_dn11 = assign25350_e22837_d_n11;
        locals.var_rd_dn12 = assign25350_e22837_d_n12;

        let assign25360_e22840: f64 = if locals.var_rd < p.p431 { 1.0 } else { 0.0 };
        locals.var_guard1349 = assign25360_e22840;

        let (assign25370_e22848, assign25370_e22848_d_n3, assign25370_e22848_d_n4, assign25370_e22848_d_n5, assign25370_e22848_d_n6, assign25370_e22848_d_n7, assign25370_e22848_d_n8, assign25370_e22848_d_n9, assign25370_e22848_d_n10, assign25370_e22848_d_n11, assign25370_e22848_d_n12,) = {
    if (((locals.var_guard1345 != 0.0) && (locals.var_guard1348 != 0.0)) && (locals.var_guard1349 != 0.0)) {
        (p.p431, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rd, locals.var_rd_dn3, locals.var_rd_dn4, locals.var_rd_dn5, locals.var_rd_dn6, locals.var_rd_dn7, locals.var_rd_dn8, locals.var_rd_dn9, locals.var_rd_dn10, locals.var_rd_dn11, locals.var_rd_dn12,)
    }
};
        locals.var_rd = assign25370_e22848;
        locals.var_rd_dn3 = assign25370_e22848_d_n3;
        locals.var_rd_dn4 = assign25370_e22848_d_n4;
        locals.var_rd_dn5 = assign25370_e22848_d_n5;
        locals.var_rd_dn6 = assign25370_e22848_d_n6;
        locals.var_rd_dn7 = assign25370_e22848_d_n7;
        locals.var_rd_dn8 = assign25370_e22848_d_n8;
        locals.var_rd_dn9 = assign25370_e22848_d_n9;
        locals.var_rd_dn10 = assign25370_e22848_d_n10;
        locals.var_rd_dn11 = assign25370_e22848_d_n11;
        locals.var_rd_dn12 = assign25370_e22848_d_n12;

        let (assign25380_e22855, assign25380_e22855_d_n3, assign25380_e22855_d_n4, assign25380_e22855_d_n5, assign25380_e22855_d_n6, assign25380_e22855_d_n7, assign25380_e22855_d_n8, assign25380_e22855_d_n9, assign25380_e22855_d_n10, assign25380_e22855_d_n11, assign25380_e22855_d_n12,) = {
    if ((locals.var_guard1345 != 0.0) && (locals.var_guard1348 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rd, locals.var_rd_dn3, locals.var_rd_dn4, locals.var_rd_dn5, locals.var_rd_dn6, locals.var_rd_dn7, locals.var_rd_dn8, locals.var_rd_dn9, locals.var_rd_dn10, locals.var_rd_dn11, locals.var_rd_dn12,)
    }
};
        locals.var_rd = assign25380_e22855;
        locals.var_rd_dn3 = assign25380_e22855_d_n3;
        locals.var_rd_dn4 = assign25380_e22855_d_n4;
        locals.var_rd_dn5 = assign25380_e22855_d_n5;
        locals.var_rd_dn6 = assign25380_e22855_d_n6;
        locals.var_rd_dn7 = assign25380_e22855_d_n7;
        locals.var_rd_dn8 = assign25380_e22855_d_n8;
        locals.var_rd_dn9 = assign25380_e22855_d_n9;
        locals.var_rd_dn10 = assign25380_e22855_d_n10;
        locals.var_rd_dn11 = assign25380_e22855_d_n11;
        locals.var_rd_dn12 = assign25380_e22855_d_n12;

        let assign25390_e22858: f64 = if p.p429 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1350 = assign25390_e22858;

        let (assign25400_e22865, assign25400_e22865_d_n3, assign25400_e22865_d_n4, assign25400_e22865_d_n5, assign25400_e22865_d_n6, assign25400_e22865_d_n7, assign25400_e22865_d_n8, assign25400_e22865_d_n9, assign25400_e22865_d_n10, assign25400_e22865_d_n11, assign25400_e22865_d_n12,) = {
    if ((locals.var_guard1345 == 0.0) && (locals.var_guard1350 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rds, locals.var_rds_dn3, locals.var_rds_dn4, locals.var_rds_dn5, locals.var_rds_dn6, locals.var_rds_dn7, locals.var_rds_dn8, locals.var_rds_dn9, locals.var_rds_dn10, locals.var_rds_dn11, locals.var_rds_dn12,)
    }
};
        locals.var_rds = assign25400_e22865;
        locals.var_rds_dn3 = assign25400_e22865_d_n3;
        locals.var_rds_dn4 = assign25400_e22865_d_n4;
        locals.var_rds_dn5 = assign25400_e22865_d_n5;
        locals.var_rds_dn6 = assign25400_e22865_d_n6;
        locals.var_rds_dn7 = assign25400_e22865_d_n7;
        locals.var_rds_dn8 = assign25400_e22865_d_n8;
        locals.var_rds_dn9 = assign25400_e22865_d_n9;
        locals.var_rds_dn10 = assign25400_e22865_d_n10;
        locals.var_rds_dn11 = assign25400_e22865_d_n11;
        locals.var_rds_dn12 = assign25400_e22865_d_n12;

        let (assign25410_e22874, assign25410_e22874_d_n3, assign25410_e22874_d_n4, assign25410_e22874_d_n5, assign25410_e22874_d_n6, assign25410_e22874_d_n7, assign25410_e22874_d_n8, assign25410_e22874_d_n9, assign25410_e22874_d_n10, assign25410_e22874_d_n11, assign25410_e22874_d_n12,) = {
    if ((locals.var_guard1345 == 0.0) && (locals.var_guard1350 != 0.0)) {
        let assign25410_e22872: f64 = (locals.var_vgs - locals.var_pparam_b4soivfbsd);
        (assign25410_e22872, (-locals.var_pparam_b4soivfbsd_dn3), (-locals.var_pparam_b4soivfbsd_dn4), (-locals.var_pparam_b4soivfbsd_dn5), (-locals.var_pparam_b4soivfbsd_dn6), (-locals.var_pparam_b4soivfbsd_dn7), (locals.var_vgs_dn8 - locals.var_pparam_b4soivfbsd_dn8), (locals.var_vgs_dn9 - locals.var_pparam_b4soivfbsd_dn9), (-locals.var_pparam_b4soivfbsd_dn10), (-locals.var_pparam_b4soivfbsd_dn11), (-locals.var_pparam_b4soivfbsd_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign25410_e22874;
        locals.var_t0__blk808_dn3 = assign25410_e22874_d_n3;
        locals.var_t0__blk808_dn4 = assign25410_e22874_d_n4;
        locals.var_t0__blk808_dn5 = assign25410_e22874_d_n5;
        locals.var_t0__blk808_dn6 = assign25410_e22874_d_n6;
        locals.var_t0__blk808_dn7 = assign25410_e22874_d_n7;
        locals.var_t0__blk808_dn8 = assign25410_e22874_d_n8;
        locals.var_t0__blk808_dn9 = assign25410_e22874_d_n9;
        locals.var_t0__blk808_dn10 = assign25410_e22874_d_n10;
        locals.var_t0__blk808_dn11 = assign25410_e22874_d_n11;
        locals.var_t0__blk808_dn12 = assign25410_e22874_d_n12;

        let (assign25420_e22886, assign25420_e22886_d_n3, assign25420_e22886_d_n4, assign25420_e22886_d_n5, assign25420_e22886_d_n6, assign25420_e22886_d_n7, assign25420_e22886_d_n8, assign25420_e22886_d_n9, assign25420_e22886_d_n10, assign25420_e22886_d_n11, assign25420_e22886_d_n12,) = {
    if ((locals.var_guard1345 == 0.0) && (locals.var_guard1350 != 0.0)) {
        let assign25420_e22881: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        let assign25420_e22883: f64 = (assign25420_e22881 + 0.0001);
        let assign25420_e22884: f64 = (assign25420_e22883).sqrt();
        (assign25420_e22884, (((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)) / (2.0 * assign25420_e22884)), (((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)) / (2.0 * assign25420_e22884)), (((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)) / (2.0 * assign25420_e22884)), (((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)) / (2.0 * assign25420_e22884)), (((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)) / (2.0 * assign25420_e22884)), (((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)) / (2.0 * assign25420_e22884)), (((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)) / (2.0 * assign25420_e22884)), (((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)) / (2.0 * assign25420_e22884)), (((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)) / (2.0 * assign25420_e22884)), (((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)) / (2.0 * assign25420_e22884)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign25420_e22886;
        locals.var_t1__blk809_dn3 = assign25420_e22886_d_n3;
        locals.var_t1__blk809_dn4 = assign25420_e22886_d_n4;
        locals.var_t1__blk809_dn5 = assign25420_e22886_d_n5;
        locals.var_t1__blk809_dn6 = assign25420_e22886_d_n6;
        locals.var_t1__blk809_dn7 = assign25420_e22886_d_n7;
        locals.var_t1__blk809_dn8 = assign25420_e22886_d_n8;
        locals.var_t1__blk809_dn9 = assign25420_e22886_d_n9;
        locals.var_t1__blk809_dn10 = assign25420_e22886_d_n10;
        locals.var_t1__blk809_dn11 = assign25420_e22886_d_n11;
        locals.var_t1__blk809_dn12 = assign25420_e22886_d_n12;

        let (assign25430_e22897, assign25430_e22897_d_n3, assign25430_e22897_d_n4, assign25430_e22897_d_n5, assign25430_e22897_d_n6, assign25430_e22897_d_n7, assign25430_e22897_d_n8, assign25430_e22897_d_n9, assign25430_e22897_d_n10, assign25430_e22897_d_n11, assign25430_e22897_d_n12,) = {
    if ((locals.var_guard1345 == 0.0) && (locals.var_guard1350 != 0.0)) {
        let assign25430_e22894: f64 = (locals.var_t0__blk808 + locals.var_t1__blk809);
        let assign25430_e22895: f64 = (0.5 * assign25430_e22894);
        (assign25430_e22895, (0.5 * (locals.var_t0__blk808_dn3 + locals.var_t1__blk809_dn3)), (0.5 * (locals.var_t0__blk808_dn4 + locals.var_t1__blk809_dn4)), (0.5 * (locals.var_t0__blk808_dn5 + locals.var_t1__blk809_dn5)), (0.5 * (locals.var_t0__blk808_dn6 + locals.var_t1__blk809_dn6)), (0.5 * (locals.var_t0__blk808_dn7 + locals.var_t1__blk809_dn7)), (0.5 * (locals.var_t0__blk808_dn8 + locals.var_t1__blk809_dn8)), (0.5 * (locals.var_t0__blk808_dn9 + locals.var_t1__blk809_dn9)), (0.5 * (locals.var_t0__blk808_dn10 + locals.var_t1__blk809_dn10)), (0.5 * (locals.var_t0__blk808_dn11 + locals.var_t1__blk809_dn11)), (0.5 * (locals.var_t0__blk808_dn12 + locals.var_t1__blk809_dn12)),)
    } else {
        (locals.var_vgs_eff_1, locals.var_vgs_eff_1_dn3, locals.var_vgs_eff_1_dn4, locals.var_vgs_eff_1_dn5, locals.var_vgs_eff_1_dn6, locals.var_vgs_eff_1_dn7, locals.var_vgs_eff_1_dn8, locals.var_vgs_eff_1_dn9, locals.var_vgs_eff_1_dn10, locals.var_vgs_eff_1_dn11, locals.var_vgs_eff_1_dn12,)
    }
};
        locals.var_vgs_eff_1 = assign25430_e22897;
        locals.var_vgs_eff_1_dn3 = assign25430_e22897_d_n3;
        locals.var_vgs_eff_1_dn4 = assign25430_e22897_d_n4;
        locals.var_vgs_eff_1_dn5 = assign25430_e22897_d_n5;
        locals.var_vgs_eff_1_dn6 = assign25430_e22897_d_n6;
        locals.var_vgs_eff_1_dn7 = assign25430_e22897_d_n7;
        locals.var_vgs_eff_1_dn8 = assign25430_e22897_d_n8;
        locals.var_vgs_eff_1_dn9 = assign25430_e22897_d_n9;
        locals.var_vgs_eff_1_dn10 = assign25430_e22897_d_n10;
        locals.var_vgs_eff_1_dn11 = assign25430_e22897_d_n11;
        locals.var_vgs_eff_1_dn12 = assign25430_e22897_d_n12;

        let (assign25440_e22908, assign25440_e22908_d_n3, assign25440_e22908_d_n4, assign25440_e22908_d_n5, assign25440_e22908_d_n6, assign25440_e22908_d_n7, assign25440_e22908_d_n8, assign25440_e22908_d_n9, assign25440_e22908_d_n10, assign25440_e22908_d_n11, assign25440_e22908_d_n12,) = {
    if ((locals.var_guard1345 == 0.0) && (locals.var_guard1350 != 0.0)) {
        let assign25440_e22905: f64 = (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1);
        let assign25440_e22906: f64 = (1.0 + assign25440_e22905);
        (assign25440_e22906, ((locals.var_pparam_b4soiprwg_dn3 * locals.var_vgs_eff_1) + (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1_dn3)), ((locals.var_pparam_b4soiprwg_dn4 * locals.var_vgs_eff_1) + (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1_dn4)), ((locals.var_pparam_b4soiprwg_dn5 * locals.var_vgs_eff_1) + (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1_dn5)), ((locals.var_pparam_b4soiprwg_dn6 * locals.var_vgs_eff_1) + (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1_dn6)), ((locals.var_pparam_b4soiprwg_dn7 * locals.var_vgs_eff_1) + (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1_dn7)), ((locals.var_pparam_b4soiprwg_dn8 * locals.var_vgs_eff_1) + (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1_dn8)), ((locals.var_pparam_b4soiprwg_dn9 * locals.var_vgs_eff_1) + (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1_dn9)), ((locals.var_pparam_b4soiprwg_dn10 * locals.var_vgs_eff_1) + (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1_dn10)), ((locals.var_pparam_b4soiprwg_dn11 * locals.var_vgs_eff_1) + (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1_dn11)), ((locals.var_pparam_b4soiprwg_dn12 * locals.var_vgs_eff_1) + (locals.var_pparam_b4soiprwg * locals.var_vgs_eff_1_dn12)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign25440_e22908;
        locals.var_t0__blk808_dn3 = assign25440_e22908_d_n3;
        locals.var_t0__blk808_dn4 = assign25440_e22908_d_n4;
        locals.var_t0__blk808_dn5 = assign25440_e22908_d_n5;
        locals.var_t0__blk808_dn6 = assign25440_e22908_d_n6;
        locals.var_t0__blk808_dn7 = assign25440_e22908_d_n7;
        locals.var_t0__blk808_dn8 = assign25440_e22908_d_n8;
        locals.var_t0__blk808_dn9 = assign25440_e22908_d_n9;
        locals.var_t0__blk808_dn10 = assign25440_e22908_d_n10;
        locals.var_t0__blk808_dn11 = assign25440_e22908_d_n11;
        locals.var_t0__blk808_dn12 = assign25440_e22908_d_n12;

        let (assign25450_e22918, assign25450_e22918_d_n3, assign25450_e22918_d_n4, assign25450_e22918_d_n5, assign25450_e22918_d_n6, assign25450_e22918_d_n7, assign25450_e22918_d_n8, assign25450_e22918_d_n9, assign25450_e22918_d_n10, assign25450_e22918_d_n11, assign25450_e22918_d_n12,) = {
    if ((locals.var_guard1345 == 0.0) && (locals.var_guard1350 != 0.0)) {
        let assign25450_e22914: f64 = (-locals.var_pparam_b4soiprwb);
        let assign25450_e22916: f64 = (assign25450_e22914 * locals.var_vbs);
        (assign25450_e22916, ((-locals.var_pparam_b4soiprwb_dn3) * locals.var_vbs), ((-locals.var_pparam_b4soiprwb_dn4) * locals.var_vbs), (((-locals.var_pparam_b4soiprwb_dn5) * locals.var_vbs) + (assign25450_e22914 * locals.var_vbs_dn5)), ((-locals.var_pparam_b4soiprwb_dn6) * locals.var_vbs), ((-locals.var_pparam_b4soiprwb_dn7) * locals.var_vbs), (((-locals.var_pparam_b4soiprwb_dn8) * locals.var_vbs) + (assign25450_e22914 * locals.var_vbs_dn8)), ((-locals.var_pparam_b4soiprwb_dn9) * locals.var_vbs), ((-locals.var_pparam_b4soiprwb_dn10) * locals.var_vbs), ((-locals.var_pparam_b4soiprwb_dn11) * locals.var_vbs), ((-locals.var_pparam_b4soiprwb_dn12) * locals.var_vbs),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign25450_e22918;
        locals.var_t1__blk809_dn3 = assign25450_e22918_d_n3;
        locals.var_t1__blk809_dn4 = assign25450_e22918_d_n4;
        locals.var_t1__blk809_dn5 = assign25450_e22918_d_n5;
        locals.var_t1__blk809_dn6 = assign25450_e22918_d_n6;
        locals.var_t1__blk809_dn7 = assign25450_e22918_d_n7;
        locals.var_t1__blk809_dn8 = assign25450_e22918_d_n8;
        locals.var_t1__blk809_dn9 = assign25450_e22918_d_n9;
        locals.var_t1__blk809_dn10 = assign25450_e22918_d_n10;
        locals.var_t1__blk809_dn11 = assign25450_e22918_d_n11;
        locals.var_t1__blk809_dn12 = assign25450_e22918_d_n12;

        let (assign25460_e22935, assign25460_e22935_d_n3, assign25460_e22935_d_n4, assign25460_e22935_d_n5, assign25460_e22935_d_n6, assign25460_e22935_d_n7, assign25460_e22935_d_n8, assign25460_e22935_d_n9, assign25460_e22935_d_n10, assign25460_e22935_d_n11, assign25460_e22935_d_n12,) = {
    if ((locals.var_guard1345 == 0.0) && (locals.var_guard1350 != 0.0)) {
        let assign25460_e22925: f64 = (1.0 / locals.var_t0__blk808);
        let assign25460_e22927: f64 = (assign25460_e22925 + locals.var_t1__blk809);
        let assign25460_e22931: f64 = (locals.var_ves - locals.var_vfbb);
        let assign25460_e22932: f64 = (locals.var_pparam_b4soiprwe * assign25460_e22931);
        let assign25460_e22933: f64 = (assign25460_e22927 + assign25460_e22932);
        (assign25460_e22933, (((-(locals.var_t0__blk808_dn3 / (locals.var_t0__blk808 * locals.var_t0__blk808))) + locals.var_t1__blk809_dn3) + ((locals.var_pparam_b4soiprwe_dn3 * assign25460_e22931) + (locals.var_pparam_b4soiprwe * (locals.var_ves_dn3 - locals.var_vfbb_dn3)))), (((-(locals.var_t0__blk808_dn4 / (locals.var_t0__blk808 * locals.var_t0__blk808))) + locals.var_t1__blk809_dn4) + ((locals.var_pparam_b4soiprwe_dn4 * assign25460_e22931) + (locals.var_pparam_b4soiprwe * (-locals.var_vfbb_dn4)))), (((-(locals.var_t0__blk808_dn5 / (locals.var_t0__blk808 * locals.var_t0__blk808))) + locals.var_t1__blk809_dn5) + ((locals.var_pparam_b4soiprwe_dn5 * assign25460_e22931) + (locals.var_pparam_b4soiprwe * (-locals.var_vfbb_dn5)))), (((-(locals.var_t0__blk808_dn6 / (locals.var_t0__blk808 * locals.var_t0__blk808))) + locals.var_t1__blk809_dn6) + ((locals.var_pparam_b4soiprwe_dn6 * assign25460_e22931) + (locals.var_pparam_b4soiprwe * (-locals.var_vfbb_dn6)))), (((-(locals.var_t0__blk808_dn7 / (locals.var_t0__blk808 * locals.var_t0__blk808))) + locals.var_t1__blk809_dn7) + ((locals.var_pparam_b4soiprwe_dn7 * assign25460_e22931) + (locals.var_pparam_b4soiprwe * (-locals.var_vfbb_dn7)))), (((-(locals.var_t0__blk808_dn8 / (locals.var_t0__blk808 * locals.var_t0__blk808))) + locals.var_t1__blk809_dn8) + ((locals.var_pparam_b4soiprwe_dn8 * assign25460_e22931) + (locals.var_pparam_b4soiprwe * (locals.var_ves_dn8 - locals.var_vfbb_dn8)))), (((-(locals.var_t0__blk808_dn9 / (locals.var_t0__blk808 * locals.var_t0__blk808))) + locals.var_t1__blk809_dn9) + ((locals.var_pparam_b4soiprwe_dn9 * assign25460_e22931) + (locals.var_pparam_b4soiprwe * (-locals.var_vfbb_dn9)))), (((-(locals.var_t0__blk808_dn10 / (locals.var_t0__blk808 * locals.var_t0__blk808))) + locals.var_t1__blk809_dn10) + ((locals.var_pparam_b4soiprwe_dn10 * assign25460_e22931) + (locals.var_pparam_b4soiprwe * (-locals.var_vfbb_dn10)))), (((-(locals.var_t0__blk808_dn11 / (locals.var_t0__blk808 * locals.var_t0__blk808))) + locals.var_t1__blk809_dn11) + ((locals.var_pparam_b4soiprwe_dn11 * assign25460_e22931) + (locals.var_pparam_b4soiprwe * (-locals.var_vfbb_dn11)))), (((-(locals.var_t0__blk808_dn12 / (locals.var_t0__blk808 * locals.var_t0__blk808))) + locals.var_t1__blk809_dn12) + ((locals.var_pparam_b4soiprwe_dn12 * assign25460_e22931) + (locals.var_pparam_b4soiprwe * (-locals.var_vfbb_dn12)))),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign25460_e22935;
        locals.var_t2__blk810_dn3 = assign25460_e22935_d_n3;
        locals.var_t2__blk810_dn4 = assign25460_e22935_d_n4;
        locals.var_t2__blk810_dn5 = assign25460_e22935_d_n5;
        locals.var_t2__blk810_dn6 = assign25460_e22935_d_n6;
        locals.var_t2__blk810_dn7 = assign25460_e22935_d_n7;
        locals.var_t2__blk810_dn8 = assign25460_e22935_d_n8;
        locals.var_t2__blk810_dn9 = assign25460_e22935_d_n9;
        locals.var_t2__blk810_dn10 = assign25460_e22935_d_n10;
        locals.var_t2__blk810_dn11 = assign25460_e22935_d_n11;
        locals.var_t2__blk810_dn12 = assign25460_e22935_d_n12;

    }

    pub(super) fn stamp_transient_block_76(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25470_e22949, assign25470_e22949_d_n3, assign25470_e22949_d_n4, assign25470_e22949_d_n5, assign25470_e22949_d_n6, assign25470_e22949_d_n7, assign25470_e22949_d_n8, assign25470_e22949_d_n9, assign25470_e22949_d_n10, assign25470_e22949_d_n11, assign25470_e22949_d_n12,) = {
    if ((locals.var_guard1345 == 0.0) && (locals.var_guard1350 != 0.0)) {
        let assign25470_e22943: f64 = (locals.var_t2__blk810 * locals.var_t2__blk810);
        let assign25470_e22945: f64 = (assign25470_e22943 + 0.01);
        let assign25470_e22946: f64 = (assign25470_e22945).sqrt();
        let assign25470_e22947: f64 = (locals.var_t2__blk810 + assign25470_e22946);
        (assign25470_e22947, (locals.var_t2__blk810_dn3 + (((locals.var_t2__blk810_dn3 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn3)) / (2.0 * assign25470_e22946))), (locals.var_t2__blk810_dn4 + (((locals.var_t2__blk810_dn4 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn4)) / (2.0 * assign25470_e22946))), (locals.var_t2__blk810_dn5 + (((locals.var_t2__blk810_dn5 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn5)) / (2.0 * assign25470_e22946))), (locals.var_t2__blk810_dn6 + (((locals.var_t2__blk810_dn6 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn6)) / (2.0 * assign25470_e22946))), (locals.var_t2__blk810_dn7 + (((locals.var_t2__blk810_dn7 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn7)) / (2.0 * assign25470_e22946))), (locals.var_t2__blk810_dn8 + (((locals.var_t2__blk810_dn8 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn8)) / (2.0 * assign25470_e22946))), (locals.var_t2__blk810_dn9 + (((locals.var_t2__blk810_dn9 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn9)) / (2.0 * assign25470_e22946))), (locals.var_t2__blk810_dn10 + (((locals.var_t2__blk810_dn10 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn10)) / (2.0 * assign25470_e22946))), (locals.var_t2__blk810_dn11 + (((locals.var_t2__blk810_dn11 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn11)) / (2.0 * assign25470_e22946))), (locals.var_t2__blk810_dn12 + (((locals.var_t2__blk810_dn12 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn12)) / (2.0 * assign25470_e22946))),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign25470_e22949;
        locals.var_t3__blk811_dn3 = assign25470_e22949_d_n3;
        locals.var_t3__blk811_dn4 = assign25470_e22949_d_n4;
        locals.var_t3__blk811_dn5 = assign25470_e22949_d_n5;
        locals.var_t3__blk811_dn6 = assign25470_e22949_d_n6;
        locals.var_t3__blk811_dn7 = assign25470_e22949_d_n7;
        locals.var_t3__blk811_dn8 = assign25470_e22949_d_n8;
        locals.var_t3__blk811_dn9 = assign25470_e22949_d_n9;
        locals.var_t3__blk811_dn10 = assign25470_e22949_d_n10;
        locals.var_t3__blk811_dn11 = assign25470_e22949_d_n11;
        locals.var_t3__blk811_dn12 = assign25470_e22949_d_n12;

        let (assign25480_e22958, assign25480_e22958_d_n3, assign25480_e22958_d_n4, assign25480_e22958_d_n5, assign25480_e22958_d_n6, assign25480_e22958_d_n7, assign25480_e22958_d_n8, assign25480_e22958_d_n9, assign25480_e22958_d_n10, assign25480_e22958_d_n11, assign25480_e22958_d_n12,) = {
    if ((locals.var_guard1345 == 0.0) && (locals.var_guard1350 != 0.0)) {
        let assign25480_e22956: f64 = (locals.var_rs0 * 0.5);
        (assign25480_e22956, (locals.var_rs0_dn3 * 0.5), (locals.var_rs0_dn4 * 0.5), (locals.var_rs0_dn5 * 0.5), (locals.var_rs0_dn6 * 0.5), (locals.var_rs0_dn7 * 0.5), (locals.var_rs0_dn8 * 0.5), (locals.var_rs0_dn9 * 0.5), (locals.var_rs0_dn10 * 0.5), (locals.var_rs0_dn11 * 0.5), (locals.var_rs0_dn12 * 0.5),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign25480_e22958;
        locals.var_t4__blk812_dn3 = assign25480_e22958_d_n3;
        locals.var_t4__blk812_dn4 = assign25480_e22958_d_n4;
        locals.var_t4__blk812_dn5 = assign25480_e22958_d_n5;
        locals.var_t4__blk812_dn6 = assign25480_e22958_d_n6;
        locals.var_t4__blk812_dn7 = assign25480_e22958_d_n7;
        locals.var_t4__blk812_dn8 = assign25480_e22958_d_n8;
        locals.var_t4__blk812_dn9 = assign25480_e22958_d_n9;
        locals.var_t4__blk812_dn10 = assign25480_e22958_d_n10;
        locals.var_t4__blk812_dn11 = assign25480_e22958_d_n11;
        locals.var_t4__blk812_dn12 = assign25480_e22958_d_n12;

        let (assign25490_e22973, assign25490_e22973_d_n3, assign25490_e22973_d_n4, assign25490_e22973_d_n5, assign25490_e22973_d_n6, assign25490_e22973_d_n7, assign25490_e22973_d_n8, assign25490_e22973_d_n9, assign25490_e22973_d_n10, assign25490_e22973_d_n11, assign25490_e22973_d_n12,) = {
    if ((locals.var_guard1345 == 0.0) && (locals.var_guard1350 != 0.0)) {
        let assign25490_e22966: f64 = (locals.var_t3__blk811 * locals.var_t4__blk812);
        let assign25490_e22967: f64 = (locals.var_rswmin + assign25490_e22966);
        let assign25490_e22969: f64 = (assign25490_e22967 + locals.var_b4soisourceresistance);
        let assign25490_e22971: f64 = (assign25490_e22969 + locals.var_rsc_t);
        (assign25490_e22971, (locals.var_rswmin_dn3 + ((locals.var_t3__blk811_dn3 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn3))), ((locals.var_rswmin_dn4 + ((locals.var_t3__blk811_dn4 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn4))) + locals.var_rsc_t_dn4), ((locals.var_rswmin_dn5 + ((locals.var_t3__blk811_dn5 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn5))) + locals.var_rsc_t_dn5), ((locals.var_rswmin_dn6 + ((locals.var_t3__blk811_dn6 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn6))) + locals.var_rsc_t_dn6), (locals.var_rswmin_dn7 + ((locals.var_t3__blk811_dn7 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn7))), (locals.var_rswmin_dn8 + ((locals.var_t3__blk811_dn8 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn8))), (locals.var_rswmin_dn9 + ((locals.var_t3__blk811_dn9 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn9))), (locals.var_rswmin_dn10 + ((locals.var_t3__blk811_dn10 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn10))), (locals.var_rswmin_dn11 + ((locals.var_t3__blk811_dn11 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn11))), (locals.var_rswmin_dn12 + ((locals.var_t3__blk811_dn12 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn12))),)
    } else {
        (locals.var_rs, locals.var_rs_dn3, locals.var_rs_dn4, locals.var_rs_dn5, locals.var_rs_dn6, locals.var_rs_dn7, locals.var_rs_dn8, locals.var_rs_dn9, locals.var_rs_dn10, locals.var_rs_dn11, locals.var_rs_dn12,)
    }
};
        locals.var_rs = assign25490_e22973;
        locals.var_rs_dn3 = assign25490_e22973_d_n3;
        locals.var_rs_dn4 = assign25490_e22973_d_n4;
        locals.var_rs_dn5 = assign25490_e22973_d_n5;
        locals.var_rs_dn6 = assign25490_e22973_d_n6;
        locals.var_rs_dn7 = assign25490_e22973_d_n7;
        locals.var_rs_dn8 = assign25490_e22973_d_n8;
        locals.var_rs_dn9 = assign25490_e22973_d_n9;
        locals.var_rs_dn10 = assign25490_e22973_d_n10;
        locals.var_rs_dn11 = assign25490_e22973_d_n11;
        locals.var_rs_dn12 = assign25490_e22973_d_n12;

        let assign25500_e22976: f64 = if locals.var_rs < p.p431 { 1.0 } else { 0.0 };
        locals.var_guard1351 = assign25500_e22976;

        let (assign25510_e22985, assign25510_e22985_d_n3, assign25510_e22985_d_n4, assign25510_e22985_d_n5, assign25510_e22985_d_n6, assign25510_e22985_d_n7, assign25510_e22985_d_n8, assign25510_e22985_d_n9, assign25510_e22985_d_n10, assign25510_e22985_d_n11, assign25510_e22985_d_n12,) = {
    if (((locals.var_guard1345 == 0.0) && (locals.var_guard1350 != 0.0)) && (locals.var_guard1351 != 0.0)) {
        (p.p431, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rs, locals.var_rs_dn3, locals.var_rs_dn4, locals.var_rs_dn5, locals.var_rs_dn6, locals.var_rs_dn7, locals.var_rs_dn8, locals.var_rs_dn9, locals.var_rs_dn10, locals.var_rs_dn11, locals.var_rs_dn12,)
    }
};
        locals.var_rs = assign25510_e22985;
        locals.var_rs_dn3 = assign25510_e22985_d_n3;
        locals.var_rs_dn4 = assign25510_e22985_d_n4;
        locals.var_rs_dn5 = assign25510_e22985_d_n5;
        locals.var_rs_dn6 = assign25510_e22985_d_n6;
        locals.var_rs_dn7 = assign25510_e22985_d_n7;
        locals.var_rs_dn8 = assign25510_e22985_d_n8;
        locals.var_rs_dn9 = assign25510_e22985_d_n9;
        locals.var_rs_dn10 = assign25510_e22985_d_n10;
        locals.var_rs_dn11 = assign25510_e22985_d_n11;
        locals.var_rs_dn12 = assign25510_e22985_d_n12;

        let (assign25520_e22994, assign25520_e22994_d_n3, assign25520_e22994_d_n4, assign25520_e22994_d_n5, assign25520_e22994_d_n6, assign25520_e22994_d_n7, assign25520_e22994_d_n8, assign25520_e22994_d_n9, assign25520_e22994_d_n10, assign25520_e22994_d_n11, assign25520_e22994_d_n12,) = {
    if ((locals.var_guard1345 == 0.0) && (locals.var_guard1350 != 0.0)) {
        let assign25520_e22992: f64 = (locals.var_vgd - locals.var_pparam_b4soivfbsd);
        (assign25520_e22992, (-locals.var_pparam_b4soivfbsd_dn3), (-locals.var_pparam_b4soivfbsd_dn4), (-locals.var_pparam_b4soivfbsd_dn5), (-locals.var_pparam_b4soivfbsd_dn6), (locals.var_vgd_dn7 - locals.var_pparam_b4soivfbsd_dn7), (locals.var_vgd_dn8 - locals.var_pparam_b4soivfbsd_dn8), (locals.var_vgd_dn9 - locals.var_pparam_b4soivfbsd_dn9), (-locals.var_pparam_b4soivfbsd_dn10), (-locals.var_pparam_b4soivfbsd_dn11), (-locals.var_pparam_b4soivfbsd_dn12),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign25520_e22994;
        locals.var_t0__blk808_dn3 = assign25520_e22994_d_n3;
        locals.var_t0__blk808_dn4 = assign25520_e22994_d_n4;
        locals.var_t0__blk808_dn5 = assign25520_e22994_d_n5;
        locals.var_t0__blk808_dn6 = assign25520_e22994_d_n6;
        locals.var_t0__blk808_dn7 = assign25520_e22994_d_n7;
        locals.var_t0__blk808_dn8 = assign25520_e22994_d_n8;
        locals.var_t0__blk808_dn9 = assign25520_e22994_d_n9;
        locals.var_t0__blk808_dn10 = assign25520_e22994_d_n10;
        locals.var_t0__blk808_dn11 = assign25520_e22994_d_n11;
        locals.var_t0__blk808_dn12 = assign25520_e22994_d_n12;

        let (assign25530_e23006, assign25530_e23006_d_n3, assign25530_e23006_d_n4, assign25530_e23006_d_n5, assign25530_e23006_d_n6, assign25530_e23006_d_n7, assign25530_e23006_d_n8, assign25530_e23006_d_n9, assign25530_e23006_d_n10, assign25530_e23006_d_n11, assign25530_e23006_d_n12,) = {
    if ((locals.var_guard1345 == 0.0) && (locals.var_guard1350 != 0.0)) {
        let assign25530_e23001: f64 = (locals.var_t0__blk808 * locals.var_t0__blk808);
        let assign25530_e23003: f64 = (assign25530_e23001 + 0.0001);
        let assign25530_e23004: f64 = (assign25530_e23003).sqrt();
        (assign25530_e23004, (((locals.var_t0__blk808_dn3 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn3)) / (2.0 * assign25530_e23004)), (((locals.var_t0__blk808_dn4 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn4)) / (2.0 * assign25530_e23004)), (((locals.var_t0__blk808_dn5 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn5)) / (2.0 * assign25530_e23004)), (((locals.var_t0__blk808_dn6 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn6)) / (2.0 * assign25530_e23004)), (((locals.var_t0__blk808_dn7 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn7)) / (2.0 * assign25530_e23004)), (((locals.var_t0__blk808_dn8 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn8)) / (2.0 * assign25530_e23004)), (((locals.var_t0__blk808_dn9 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn9)) / (2.0 * assign25530_e23004)), (((locals.var_t0__blk808_dn10 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn10)) / (2.0 * assign25530_e23004)), (((locals.var_t0__blk808_dn11 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn11)) / (2.0 * assign25530_e23004)), (((locals.var_t0__blk808_dn12 * locals.var_t0__blk808) + (locals.var_t0__blk808 * locals.var_t0__blk808_dn12)) / (2.0 * assign25530_e23004)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign25530_e23006;
        locals.var_t1__blk809_dn3 = assign25530_e23006_d_n3;
        locals.var_t1__blk809_dn4 = assign25530_e23006_d_n4;
        locals.var_t1__blk809_dn5 = assign25530_e23006_d_n5;
        locals.var_t1__blk809_dn6 = assign25530_e23006_d_n6;
        locals.var_t1__blk809_dn7 = assign25530_e23006_d_n7;
        locals.var_t1__blk809_dn8 = assign25530_e23006_d_n8;
        locals.var_t1__blk809_dn9 = assign25530_e23006_d_n9;
        locals.var_t1__blk809_dn10 = assign25530_e23006_d_n10;
        locals.var_t1__blk809_dn11 = assign25530_e23006_d_n11;
        locals.var_t1__blk809_dn12 = assign25530_e23006_d_n12;

        let (assign25540_e23017, assign25540_e23017_d_n3, assign25540_e23017_d_n4, assign25540_e23017_d_n5, assign25540_e23017_d_n6, assign25540_e23017_d_n7, assign25540_e23017_d_n8, assign25540_e23017_d_n9, assign25540_e23017_d_n10, assign25540_e23017_d_n11, assign25540_e23017_d_n12,) = {
    if ((locals.var_guard1345 == 0.0) && (locals.var_guard1350 != 0.0)) {
        let assign25540_e23014: f64 = (locals.var_t0__blk808 + locals.var_t1__blk809);
        let assign25540_e23015: f64 = (0.5 * assign25540_e23014);
        (assign25540_e23015, (0.5 * (locals.var_t0__blk808_dn3 + locals.var_t1__blk809_dn3)), (0.5 * (locals.var_t0__blk808_dn4 + locals.var_t1__blk809_dn4)), (0.5 * (locals.var_t0__blk808_dn5 + locals.var_t1__blk809_dn5)), (0.5 * (locals.var_t0__blk808_dn6 + locals.var_t1__blk809_dn6)), (0.5 * (locals.var_t0__blk808_dn7 + locals.var_t1__blk809_dn7)), (0.5 * (locals.var_t0__blk808_dn8 + locals.var_t1__blk809_dn8)), (0.5 * (locals.var_t0__blk808_dn9 + locals.var_t1__blk809_dn9)), (0.5 * (locals.var_t0__blk808_dn10 + locals.var_t1__blk809_dn10)), (0.5 * (locals.var_t0__blk808_dn11 + locals.var_t1__blk809_dn11)), (0.5 * (locals.var_t0__blk808_dn12 + locals.var_t1__blk809_dn12)),)
    } else {
        (locals.var_vgd_eff, locals.var_vgd_eff_dn3, locals.var_vgd_eff_dn4, locals.var_vgd_eff_dn5, locals.var_vgd_eff_dn6, locals.var_vgd_eff_dn7, locals.var_vgd_eff_dn8, locals.var_vgd_eff_dn9, locals.var_vgd_eff_dn10, locals.var_vgd_eff_dn11, locals.var_vgd_eff_dn12,)
    }
};
        locals.var_vgd_eff = assign25540_e23017;
        locals.var_vgd_eff_dn3 = assign25540_e23017_d_n3;
        locals.var_vgd_eff_dn4 = assign25540_e23017_d_n4;
        locals.var_vgd_eff_dn5 = assign25540_e23017_d_n5;
        locals.var_vgd_eff_dn6 = assign25540_e23017_d_n6;
        locals.var_vgd_eff_dn7 = assign25540_e23017_d_n7;
        locals.var_vgd_eff_dn8 = assign25540_e23017_d_n8;
        locals.var_vgd_eff_dn9 = assign25540_e23017_d_n9;
        locals.var_vgd_eff_dn10 = assign25540_e23017_d_n10;
        locals.var_vgd_eff_dn11 = assign25540_e23017_d_n11;
        locals.var_vgd_eff_dn12 = assign25540_e23017_d_n12;

        let (assign25550_e23028, assign25550_e23028_d_n3, assign25550_e23028_d_n4, assign25550_e23028_d_n5, assign25550_e23028_d_n6, assign25550_e23028_d_n7, assign25550_e23028_d_n8, assign25550_e23028_d_n9, assign25550_e23028_d_n10, assign25550_e23028_d_n11, assign25550_e23028_d_n12,) = {
    if ((locals.var_guard1345 == 0.0) && (locals.var_guard1350 != 0.0)) {
        let assign25550_e23025: f64 = (locals.var_pparam_b4soiprwg * locals.var_vgd_eff);
        let assign25550_e23026: f64 = (1.0 + assign25550_e23025);
        (assign25550_e23026, ((locals.var_pparam_b4soiprwg_dn3 * locals.var_vgd_eff) + (locals.var_pparam_b4soiprwg * locals.var_vgd_eff_dn3)), ((locals.var_pparam_b4soiprwg_dn4 * locals.var_vgd_eff) + (locals.var_pparam_b4soiprwg * locals.var_vgd_eff_dn4)), ((locals.var_pparam_b4soiprwg_dn5 * locals.var_vgd_eff) + (locals.var_pparam_b4soiprwg * locals.var_vgd_eff_dn5)), ((locals.var_pparam_b4soiprwg_dn6 * locals.var_vgd_eff) + (locals.var_pparam_b4soiprwg * locals.var_vgd_eff_dn6)), ((locals.var_pparam_b4soiprwg_dn7 * locals.var_vgd_eff) + (locals.var_pparam_b4soiprwg * locals.var_vgd_eff_dn7)), ((locals.var_pparam_b4soiprwg_dn8 * locals.var_vgd_eff) + (locals.var_pparam_b4soiprwg * locals.var_vgd_eff_dn8)), ((locals.var_pparam_b4soiprwg_dn9 * locals.var_vgd_eff) + (locals.var_pparam_b4soiprwg * locals.var_vgd_eff_dn9)), ((locals.var_pparam_b4soiprwg_dn10 * locals.var_vgd_eff) + (locals.var_pparam_b4soiprwg * locals.var_vgd_eff_dn10)), ((locals.var_pparam_b4soiprwg_dn11 * locals.var_vgd_eff) + (locals.var_pparam_b4soiprwg * locals.var_vgd_eff_dn11)), ((locals.var_pparam_b4soiprwg_dn12 * locals.var_vgd_eff) + (locals.var_pparam_b4soiprwg * locals.var_vgd_eff_dn12)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign25550_e23028;
        locals.var_t0__blk808_dn3 = assign25550_e23028_d_n3;
        locals.var_t0__blk808_dn4 = assign25550_e23028_d_n4;
        locals.var_t0__blk808_dn5 = assign25550_e23028_d_n5;
        locals.var_t0__blk808_dn6 = assign25550_e23028_d_n6;
        locals.var_t0__blk808_dn7 = assign25550_e23028_d_n7;
        locals.var_t0__blk808_dn8 = assign25550_e23028_d_n8;
        locals.var_t0__blk808_dn9 = assign25550_e23028_d_n9;
        locals.var_t0__blk808_dn10 = assign25550_e23028_d_n10;
        locals.var_t0__blk808_dn11 = assign25550_e23028_d_n11;
        locals.var_t0__blk808_dn12 = assign25550_e23028_d_n12;

        let (assign25560_e23038, assign25560_e23038_d_n3, assign25560_e23038_d_n4, assign25560_e23038_d_n5, assign25560_e23038_d_n6, assign25560_e23038_d_n7, assign25560_e23038_d_n8, assign25560_e23038_d_n9, assign25560_e23038_d_n10, assign25560_e23038_d_n11, assign25560_e23038_d_n12,) = {
    if ((locals.var_guard1345 == 0.0) && (locals.var_guard1350 != 0.0)) {
        let assign25560_e23034: f64 = (-locals.var_pparam_b4soiprwb);
        let assign25560_e23036: f64 = (assign25560_e23034 * locals.var_vbd);
        (assign25560_e23036, ((-locals.var_pparam_b4soiprwb_dn3) * locals.var_vbd), ((-locals.var_pparam_b4soiprwb_dn4) * locals.var_vbd), (((-locals.var_pparam_b4soiprwb_dn5) * locals.var_vbd) + (assign25560_e23034 * locals.var_vbd_dn5)), ((-locals.var_pparam_b4soiprwb_dn6) * locals.var_vbd), (((-locals.var_pparam_b4soiprwb_dn7) * locals.var_vbd) + (assign25560_e23034 * locals.var_vbd_dn7)), (((-locals.var_pparam_b4soiprwb_dn8) * locals.var_vbd) + (assign25560_e23034 * locals.var_vbd_dn8)), ((-locals.var_pparam_b4soiprwb_dn9) * locals.var_vbd), ((-locals.var_pparam_b4soiprwb_dn10) * locals.var_vbd), ((-locals.var_pparam_b4soiprwb_dn11) * locals.var_vbd), ((-locals.var_pparam_b4soiprwb_dn12) * locals.var_vbd),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign25560_e23038;
        locals.var_t1__blk809_dn3 = assign25560_e23038_d_n3;
        locals.var_t1__blk809_dn4 = assign25560_e23038_d_n4;
        locals.var_t1__blk809_dn5 = assign25560_e23038_d_n5;
        locals.var_t1__blk809_dn6 = assign25560_e23038_d_n6;
        locals.var_t1__blk809_dn7 = assign25560_e23038_d_n7;
        locals.var_t1__blk809_dn8 = assign25560_e23038_d_n8;
        locals.var_t1__blk809_dn9 = assign25560_e23038_d_n9;
        locals.var_t1__blk809_dn10 = assign25560_e23038_d_n10;
        locals.var_t1__blk809_dn11 = assign25560_e23038_d_n11;
        locals.var_t1__blk809_dn12 = assign25560_e23038_d_n12;

        let (assign25570_e23055, assign25570_e23055_d_n3, assign25570_e23055_d_n4, assign25570_e23055_d_n5, assign25570_e23055_d_n6, assign25570_e23055_d_n7, assign25570_e23055_d_n8, assign25570_e23055_d_n9, assign25570_e23055_d_n10, assign25570_e23055_d_n11, assign25570_e23055_d_n12,) = {
    if ((locals.var_guard1345 == 0.0) && (locals.var_guard1350 != 0.0)) {
        let assign25570_e23045: f64 = (1.0 / locals.var_t0__blk808);
        let assign25570_e23047: f64 = (assign25570_e23045 + locals.var_t1__blk809);
        let assign25570_e23051: f64 = (locals.var_ves - locals.var_vfbb);
        let assign25570_e23052: f64 = (locals.var_pparam_b4soiprwe * assign25570_e23051);
        let assign25570_e23053: f64 = (assign25570_e23047 + assign25570_e23052);
        (assign25570_e23053, (((-(locals.var_t0__blk808_dn3 / (locals.var_t0__blk808 * locals.var_t0__blk808))) + locals.var_t1__blk809_dn3) + ((locals.var_pparam_b4soiprwe_dn3 * assign25570_e23051) + (locals.var_pparam_b4soiprwe * (locals.var_ves_dn3 - locals.var_vfbb_dn3)))), (((-(locals.var_t0__blk808_dn4 / (locals.var_t0__blk808 * locals.var_t0__blk808))) + locals.var_t1__blk809_dn4) + ((locals.var_pparam_b4soiprwe_dn4 * assign25570_e23051) + (locals.var_pparam_b4soiprwe * (-locals.var_vfbb_dn4)))), (((-(locals.var_t0__blk808_dn5 / (locals.var_t0__blk808 * locals.var_t0__blk808))) + locals.var_t1__blk809_dn5) + ((locals.var_pparam_b4soiprwe_dn5 * assign25570_e23051) + (locals.var_pparam_b4soiprwe * (-locals.var_vfbb_dn5)))), (((-(locals.var_t0__blk808_dn6 / (locals.var_t0__blk808 * locals.var_t0__blk808))) + locals.var_t1__blk809_dn6) + ((locals.var_pparam_b4soiprwe_dn6 * assign25570_e23051) + (locals.var_pparam_b4soiprwe * (-locals.var_vfbb_dn6)))), (((-(locals.var_t0__blk808_dn7 / (locals.var_t0__blk808 * locals.var_t0__blk808))) + locals.var_t1__blk809_dn7) + ((locals.var_pparam_b4soiprwe_dn7 * assign25570_e23051) + (locals.var_pparam_b4soiprwe * (-locals.var_vfbb_dn7)))), (((-(locals.var_t0__blk808_dn8 / (locals.var_t0__blk808 * locals.var_t0__blk808))) + locals.var_t1__blk809_dn8) + ((locals.var_pparam_b4soiprwe_dn8 * assign25570_e23051) + (locals.var_pparam_b4soiprwe * (locals.var_ves_dn8 - locals.var_vfbb_dn8)))), (((-(locals.var_t0__blk808_dn9 / (locals.var_t0__blk808 * locals.var_t0__blk808))) + locals.var_t1__blk809_dn9) + ((locals.var_pparam_b4soiprwe_dn9 * assign25570_e23051) + (locals.var_pparam_b4soiprwe * (-locals.var_vfbb_dn9)))), (((-(locals.var_t0__blk808_dn10 / (locals.var_t0__blk808 * locals.var_t0__blk808))) + locals.var_t1__blk809_dn10) + ((locals.var_pparam_b4soiprwe_dn10 * assign25570_e23051) + (locals.var_pparam_b4soiprwe * (-locals.var_vfbb_dn10)))), (((-(locals.var_t0__blk808_dn11 / (locals.var_t0__blk808 * locals.var_t0__blk808))) + locals.var_t1__blk809_dn11) + ((locals.var_pparam_b4soiprwe_dn11 * assign25570_e23051) + (locals.var_pparam_b4soiprwe * (-locals.var_vfbb_dn11)))), (((-(locals.var_t0__blk808_dn12 / (locals.var_t0__blk808 * locals.var_t0__blk808))) + locals.var_t1__blk809_dn12) + ((locals.var_pparam_b4soiprwe_dn12 * assign25570_e23051) + (locals.var_pparam_b4soiprwe * (-locals.var_vfbb_dn12)))),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign25570_e23055;
        locals.var_t2__blk810_dn3 = assign25570_e23055_d_n3;
        locals.var_t2__blk810_dn4 = assign25570_e23055_d_n4;
        locals.var_t2__blk810_dn5 = assign25570_e23055_d_n5;
        locals.var_t2__blk810_dn6 = assign25570_e23055_d_n6;
        locals.var_t2__blk810_dn7 = assign25570_e23055_d_n7;
        locals.var_t2__blk810_dn8 = assign25570_e23055_d_n8;
        locals.var_t2__blk810_dn9 = assign25570_e23055_d_n9;
        locals.var_t2__blk810_dn10 = assign25570_e23055_d_n10;
        locals.var_t2__blk810_dn11 = assign25570_e23055_d_n11;
        locals.var_t2__blk810_dn12 = assign25570_e23055_d_n12;

        let (assign25580_e23069, assign25580_e23069_d_n3, assign25580_e23069_d_n4, assign25580_e23069_d_n5, assign25580_e23069_d_n6, assign25580_e23069_d_n7, assign25580_e23069_d_n8, assign25580_e23069_d_n9, assign25580_e23069_d_n10, assign25580_e23069_d_n11, assign25580_e23069_d_n12,) = {
    if ((locals.var_guard1345 == 0.0) && (locals.var_guard1350 != 0.0)) {
        let assign25580_e23063: f64 = (locals.var_t2__blk810 * locals.var_t2__blk810);
        let assign25580_e23065: f64 = (assign25580_e23063 + 0.01);
        let assign25580_e23066: f64 = (assign25580_e23065).sqrt();
        let assign25580_e23067: f64 = (locals.var_t2__blk810 + assign25580_e23066);
        (assign25580_e23067, (locals.var_t2__blk810_dn3 + (((locals.var_t2__blk810_dn3 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn3)) / (2.0 * assign25580_e23066))), (locals.var_t2__blk810_dn4 + (((locals.var_t2__blk810_dn4 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn4)) / (2.0 * assign25580_e23066))), (locals.var_t2__blk810_dn5 + (((locals.var_t2__blk810_dn5 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn5)) / (2.0 * assign25580_e23066))), (locals.var_t2__blk810_dn6 + (((locals.var_t2__blk810_dn6 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn6)) / (2.0 * assign25580_e23066))), (locals.var_t2__blk810_dn7 + (((locals.var_t2__blk810_dn7 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn7)) / (2.0 * assign25580_e23066))), (locals.var_t2__blk810_dn8 + (((locals.var_t2__blk810_dn8 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn8)) / (2.0 * assign25580_e23066))), (locals.var_t2__blk810_dn9 + (((locals.var_t2__blk810_dn9 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn9)) / (2.0 * assign25580_e23066))), (locals.var_t2__blk810_dn10 + (((locals.var_t2__blk810_dn10 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn10)) / (2.0 * assign25580_e23066))), (locals.var_t2__blk810_dn11 + (((locals.var_t2__blk810_dn11 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn11)) / (2.0 * assign25580_e23066))), (locals.var_t2__blk810_dn12 + (((locals.var_t2__blk810_dn12 * locals.var_t2__blk810) + (locals.var_t2__blk810 * locals.var_t2__blk810_dn12)) / (2.0 * assign25580_e23066))),)
    } else {
        (locals.var_t3__blk811, locals.var_t3__blk811_dn3, locals.var_t3__blk811_dn4, locals.var_t3__blk811_dn5, locals.var_t3__blk811_dn6, locals.var_t3__blk811_dn7, locals.var_t3__blk811_dn8, locals.var_t3__blk811_dn9, locals.var_t3__blk811_dn10, locals.var_t3__blk811_dn11, locals.var_t3__blk811_dn12,)
    }
};
        locals.var_t3__blk811 = assign25580_e23069;
        locals.var_t3__blk811_dn3 = assign25580_e23069_d_n3;
        locals.var_t3__blk811_dn4 = assign25580_e23069_d_n4;
        locals.var_t3__blk811_dn5 = assign25580_e23069_d_n5;
        locals.var_t3__blk811_dn6 = assign25580_e23069_d_n6;
        locals.var_t3__blk811_dn7 = assign25580_e23069_d_n7;
        locals.var_t3__blk811_dn8 = assign25580_e23069_d_n8;
        locals.var_t3__blk811_dn9 = assign25580_e23069_d_n9;
        locals.var_t3__blk811_dn10 = assign25580_e23069_d_n10;
        locals.var_t3__blk811_dn11 = assign25580_e23069_d_n11;
        locals.var_t3__blk811_dn12 = assign25580_e23069_d_n12;

        let (assign25590_e23078, assign25590_e23078_d_n3, assign25590_e23078_d_n4, assign25590_e23078_d_n5, assign25590_e23078_d_n6, assign25590_e23078_d_n7, assign25590_e23078_d_n8, assign25590_e23078_d_n9, assign25590_e23078_d_n10, assign25590_e23078_d_n11, assign25590_e23078_d_n12,) = {
    if ((locals.var_guard1345 == 0.0) && (locals.var_guard1350 != 0.0)) {
        let assign25590_e23076: f64 = (locals.var_rd0 * 0.5);
        (assign25590_e23076, (locals.var_rd0_dn3 * 0.5), (locals.var_rd0_dn4 * 0.5), (locals.var_rd0_dn5 * 0.5), (locals.var_rd0_dn6 * 0.5), (locals.var_rd0_dn7 * 0.5), (locals.var_rd0_dn8 * 0.5), (locals.var_rd0_dn9 * 0.5), (locals.var_rd0_dn10 * 0.5), (locals.var_rd0_dn11 * 0.5), (locals.var_rd0_dn12 * 0.5),)
    } else {
        (locals.var_t4__blk812, locals.var_t4__blk812_dn3, locals.var_t4__blk812_dn4, locals.var_t4__blk812_dn5, locals.var_t4__blk812_dn6, locals.var_t4__blk812_dn7, locals.var_t4__blk812_dn8, locals.var_t4__blk812_dn9, locals.var_t4__blk812_dn10, locals.var_t4__blk812_dn11, locals.var_t4__blk812_dn12,)
    }
};
        locals.var_t4__blk812 = assign25590_e23078;
        locals.var_t4__blk812_dn3 = assign25590_e23078_d_n3;
        locals.var_t4__blk812_dn4 = assign25590_e23078_d_n4;
        locals.var_t4__blk812_dn5 = assign25590_e23078_d_n5;
        locals.var_t4__blk812_dn6 = assign25590_e23078_d_n6;
        locals.var_t4__blk812_dn7 = assign25590_e23078_d_n7;
        locals.var_t4__blk812_dn8 = assign25590_e23078_d_n8;
        locals.var_t4__blk812_dn9 = assign25590_e23078_d_n9;
        locals.var_t4__blk812_dn10 = assign25590_e23078_d_n10;
        locals.var_t4__blk812_dn11 = assign25590_e23078_d_n11;
        locals.var_t4__blk812_dn12 = assign25590_e23078_d_n12;

        let (assign25600_e23093, assign25600_e23093_d_n3, assign25600_e23093_d_n4, assign25600_e23093_d_n5, assign25600_e23093_d_n6, assign25600_e23093_d_n7, assign25600_e23093_d_n8, assign25600_e23093_d_n9, assign25600_e23093_d_n10, assign25600_e23093_d_n11, assign25600_e23093_d_n12,) = {
    if ((locals.var_guard1345 == 0.0) && (locals.var_guard1350 != 0.0)) {
        let assign25600_e23086: f64 = (locals.var_t3__blk811 * locals.var_t4__blk812);
        let assign25600_e23087: f64 = (locals.var_rdwmin + assign25600_e23086);
        let assign25600_e23089: f64 = (assign25600_e23087 + locals.var_b4soidrainresistance);
        let assign25600_e23091: f64 = (assign25600_e23089 + locals.var_rdc_t);
        (assign25600_e23091, (locals.var_rdwmin_dn3 + ((locals.var_t3__blk811_dn3 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn3))), ((locals.var_rdwmin_dn4 + ((locals.var_t3__blk811_dn4 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn4))) + locals.var_rdc_t_dn4), ((locals.var_rdwmin_dn5 + ((locals.var_t3__blk811_dn5 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn5))) + locals.var_rdc_t_dn5), ((locals.var_rdwmin_dn6 + ((locals.var_t3__blk811_dn6 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn6))) + locals.var_rdc_t_dn6), (locals.var_rdwmin_dn7 + ((locals.var_t3__blk811_dn7 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn7))), (locals.var_rdwmin_dn8 + ((locals.var_t3__blk811_dn8 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn8))), (locals.var_rdwmin_dn9 + ((locals.var_t3__blk811_dn9 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn9))), (locals.var_rdwmin_dn10 + ((locals.var_t3__blk811_dn10 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn10))), (locals.var_rdwmin_dn11 + ((locals.var_t3__blk811_dn11 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn11))), (locals.var_rdwmin_dn12 + ((locals.var_t3__blk811_dn12 * locals.var_t4__blk812) + (locals.var_t3__blk811 * locals.var_t4__blk812_dn12))),)
    } else {
        (locals.var_rd, locals.var_rd_dn3, locals.var_rd_dn4, locals.var_rd_dn5, locals.var_rd_dn6, locals.var_rd_dn7, locals.var_rd_dn8, locals.var_rd_dn9, locals.var_rd_dn10, locals.var_rd_dn11, locals.var_rd_dn12,)
    }
};
        locals.var_rd = assign25600_e23093;
        locals.var_rd_dn3 = assign25600_e23093_d_n3;
        locals.var_rd_dn4 = assign25600_e23093_d_n4;
        locals.var_rd_dn5 = assign25600_e23093_d_n5;
        locals.var_rd_dn6 = assign25600_e23093_d_n6;
        locals.var_rd_dn7 = assign25600_e23093_d_n7;
        locals.var_rd_dn8 = assign25600_e23093_d_n8;
        locals.var_rd_dn9 = assign25600_e23093_d_n9;
        locals.var_rd_dn10 = assign25600_e23093_d_n10;
        locals.var_rd_dn11 = assign25600_e23093_d_n11;
        locals.var_rd_dn12 = assign25600_e23093_d_n12;

        let assign25610_e23096: f64 = if locals.var_rd < p.p431 { 1.0 } else { 0.0 };
        locals.var_guard1352 = assign25610_e23096;

        let (assign25620_e23105, assign25620_e23105_d_n3, assign25620_e23105_d_n4, assign25620_e23105_d_n5, assign25620_e23105_d_n6, assign25620_e23105_d_n7, assign25620_e23105_d_n8, assign25620_e23105_d_n9, assign25620_e23105_d_n10, assign25620_e23105_d_n11, assign25620_e23105_d_n12,) = {
    if (((locals.var_guard1345 == 0.0) && (locals.var_guard1350 != 0.0)) && (locals.var_guard1352 != 0.0)) {
        (p.p431, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rd, locals.var_rd_dn3, locals.var_rd_dn4, locals.var_rd_dn5, locals.var_rd_dn6, locals.var_rd_dn7, locals.var_rd_dn8, locals.var_rd_dn9, locals.var_rd_dn10, locals.var_rd_dn11, locals.var_rd_dn12,)
    }
};
        locals.var_rd = assign25620_e23105;
        locals.var_rd_dn3 = assign25620_e23105_d_n3;
        locals.var_rd_dn4 = assign25620_e23105_d_n4;
        locals.var_rd_dn5 = assign25620_e23105_d_n5;
        locals.var_rd_dn6 = assign25620_e23105_d_n6;
        locals.var_rd_dn7 = assign25620_e23105_d_n7;
        locals.var_rd_dn8 = assign25620_e23105_d_n8;
        locals.var_rd_dn9 = assign25620_e23105_d_n9;
        locals.var_rd_dn10 = assign25620_e23105_d_n10;
        locals.var_rd_dn11 = assign25620_e23105_d_n11;
        locals.var_rd_dn12 = assign25620_e23105_d_n12;

        let (assign25630_e23113, assign25630_e23113_d_n3, assign25630_e23113_d_n4, assign25630_e23113_d_n5, assign25630_e23113_d_n6, assign25630_e23113_d_n7, assign25630_e23113_d_n8, assign25630_e23113_d_n9, assign25630_e23113_d_n10, assign25630_e23113_d_n11, assign25630_e23113_d_n12,) = {
    if ((locals.var_guard1345 == 0.0) && (locals.var_guard1350 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rs, locals.var_rs_dn3, locals.var_rs_dn4, locals.var_rs_dn5, locals.var_rs_dn6, locals.var_rs_dn7, locals.var_rs_dn8, locals.var_rs_dn9, locals.var_rs_dn10, locals.var_rs_dn11, locals.var_rs_dn12,)
    }
};
        locals.var_rs = assign25630_e23113;
        locals.var_rs_dn3 = assign25630_e23113_d_n3;
        locals.var_rs_dn4 = assign25630_e23113_d_n4;
        locals.var_rs_dn5 = assign25630_e23113_d_n5;
        locals.var_rs_dn6 = assign25630_e23113_d_n6;
        locals.var_rs_dn7 = assign25630_e23113_d_n7;
        locals.var_rs_dn8 = assign25630_e23113_d_n8;
        locals.var_rs_dn9 = assign25630_e23113_d_n9;
        locals.var_rs_dn10 = assign25630_e23113_d_n10;
        locals.var_rs_dn11 = assign25630_e23113_d_n11;
        locals.var_rs_dn12 = assign25630_e23113_d_n12;

        let (assign25640_e23121, assign25640_e23121_d_n3, assign25640_e23121_d_n4, assign25640_e23121_d_n5, assign25640_e23121_d_n6, assign25640_e23121_d_n7, assign25640_e23121_d_n8, assign25640_e23121_d_n9, assign25640_e23121_d_n10, assign25640_e23121_d_n11, assign25640_e23121_d_n12,) = {
    if ((locals.var_guard1345 == 0.0) && (locals.var_guard1350 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rd, locals.var_rd_dn3, locals.var_rd_dn4, locals.var_rd_dn5, locals.var_rd_dn6, locals.var_rd_dn7, locals.var_rd_dn8, locals.var_rd_dn9, locals.var_rd_dn10, locals.var_rd_dn11, locals.var_rd_dn12,)
    }
};
        locals.var_rd = assign25640_e23121;
        locals.var_rd_dn3 = assign25640_e23121_d_n3;
        locals.var_rd_dn4 = assign25640_e23121_d_n4;
        locals.var_rd_dn5 = assign25640_e23121_d_n5;
        locals.var_rd_dn6 = assign25640_e23121_d_n6;
        locals.var_rd_dn7 = assign25640_e23121_d_n7;
        locals.var_rd_dn8 = assign25640_e23121_d_n8;
        locals.var_rd_dn9 = assign25640_e23121_d_n9;
        locals.var_rd_dn10 = assign25640_e23121_d_n10;
        locals.var_rd_dn11 = assign25640_e23121_d_n11;
        locals.var_rd_dn12 = assign25640_e23121_d_n12;

        let assign25650_e23124: f64 = if p.p430 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1353 = assign25650_e23124;

        let (assign25660_e23130, assign25660_e23130_d_n3, assign25660_e23130_d_n4, assign25660_e23130_d_n5, assign25660_e23130_d_n6, assign25660_e23130_d_n7, assign25660_e23130_d_n8, assign25660_e23130_d_n9, assign25660_e23130_d_n10, assign25660_e23130_d_n11, assign25660_e23130_d_n12,) = {
    if (locals.var_guard1353 != 0.0) {
        let assign25660_e23128: f64 = (locals.var_rs / p.p30);
        (assign25660_e23128, (locals.var_rs_dn3 / p.p30), (locals.var_rs_dn4 / p.p30), (locals.var_rs_dn5 / p.p30), (locals.var_rs_dn6 / p.p30), (locals.var_rs_dn7 / p.p30), (locals.var_rs_dn8 / p.p30), (locals.var_rs_dn9 / p.p30), (locals.var_rs_dn10 / p.p30), (locals.var_rs_dn11 / p.p30), (locals.var_rs_dn12 / p.p30),)
    } else {
        (locals.var_rs, locals.var_rs_dn3, locals.var_rs_dn4, locals.var_rs_dn5, locals.var_rs_dn6, locals.var_rs_dn7, locals.var_rs_dn8, locals.var_rs_dn9, locals.var_rs_dn10, locals.var_rs_dn11, locals.var_rs_dn12,)
    }
};
        locals.var_rs = assign25660_e23130;
        locals.var_rs_dn3 = assign25660_e23130_d_n3;
        locals.var_rs_dn4 = assign25660_e23130_d_n4;
        locals.var_rs_dn5 = assign25660_e23130_d_n5;
        locals.var_rs_dn6 = assign25660_e23130_d_n6;
        locals.var_rs_dn7 = assign25660_e23130_d_n7;
        locals.var_rs_dn8 = assign25660_e23130_d_n8;
        locals.var_rs_dn9 = assign25660_e23130_d_n9;
        locals.var_rs_dn10 = assign25660_e23130_d_n10;
        locals.var_rs_dn11 = assign25660_e23130_d_n11;
        locals.var_rs_dn12 = assign25660_e23130_d_n12;

        let (assign25670_e23136, assign25670_e23136_d_n3, assign25670_e23136_d_n4, assign25670_e23136_d_n5, assign25670_e23136_d_n6, assign25670_e23136_d_n7, assign25670_e23136_d_n8, assign25670_e23136_d_n9, assign25670_e23136_d_n10, assign25670_e23136_d_n11, assign25670_e23136_d_n12,) = {
    if (locals.var_guard1353 != 0.0) {
        let assign25670_e23134: f64 = (locals.var_rd / p.p30);
        (assign25670_e23134, (locals.var_rd_dn3 / p.p30), (locals.var_rd_dn4 / p.p30), (locals.var_rd_dn5 / p.p30), (locals.var_rd_dn6 / p.p30), (locals.var_rd_dn7 / p.p30), (locals.var_rd_dn8 / p.p30), (locals.var_rd_dn9 / p.p30), (locals.var_rd_dn10 / p.p30), (locals.var_rd_dn11 / p.p30), (locals.var_rd_dn12 / p.p30),)
    } else {
        (locals.var_rd, locals.var_rd_dn3, locals.var_rd_dn4, locals.var_rd_dn5, locals.var_rd_dn6, locals.var_rd_dn7, locals.var_rd_dn8, locals.var_rd_dn9, locals.var_rd_dn10, locals.var_rd_dn11, locals.var_rd_dn12,)
    }
};
        locals.var_rd = assign25670_e23136;
        locals.var_rd_dn3 = assign25670_e23136_d_n3;
        locals.var_rd_dn4 = assign25670_e23136_d_n4;
        locals.var_rd_dn5 = assign25670_e23136_d_n5;
        locals.var_rd_dn6 = assign25670_e23136_d_n6;
        locals.var_rd_dn7 = assign25670_e23136_d_n7;
        locals.var_rd_dn8 = assign25670_e23136_d_n8;
        locals.var_rd_dn9 = assign25670_e23136_d_n9;
        locals.var_rd_dn10 = assign25670_e23136_d_n10;
        locals.var_rd_dn11 = assign25670_e23136_d_n11;
        locals.var_rd_dn12 = assign25670_e23136_d_n12;

        let assign25680_e23141: f64 = (0.5 * locals.var_abulk);
        let assign25680_e23143: f64 = (assign25680_e23141 * locals.var_vdseff);
        let assign25680_e23145: f64 = (assign25680_e23143 / locals.var_vgst2vtm);
        let assign25680_e23146: f64 = (1.0 - assign25680_e23145);
        let assign25680_e23147: f64 = (locals.var_vgsteff__blk840 * assign25680_e23146);
        locals.var_t1__blk809 = assign25680_e23147;
        locals.var_t1__blk809_dn3 = ((locals.var_vgsteff__blk840_dn3 * assign25680_e23146) + (locals.var_vgsteff__blk840 * (-((((((0.5 * locals.var_abulk_dn3) * locals.var_vdseff) + (assign25680_e23141 * locals.var_vdseff_dn3)) * locals.var_vgst2vtm) - (assign25680_e23143 * locals.var_vgst2vtm_dn3)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)))));
        locals.var_t1__blk809_dn4 = ((locals.var_vgsteff__blk840_dn4 * assign25680_e23146) + (locals.var_vgsteff__blk840 * (-((((((0.5 * locals.var_abulk_dn4) * locals.var_vdseff) + (assign25680_e23141 * locals.var_vdseff_dn4)) * locals.var_vgst2vtm) - (assign25680_e23143 * locals.var_vgst2vtm_dn4)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)))));
        locals.var_t1__blk809_dn5 = ((locals.var_vgsteff__blk840_dn5 * assign25680_e23146) + (locals.var_vgsteff__blk840 * (-((((((0.5 * locals.var_abulk_dn5) * locals.var_vdseff) + (assign25680_e23141 * locals.var_vdseff_dn5)) * locals.var_vgst2vtm) - (assign25680_e23143 * locals.var_vgst2vtm_dn5)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)))));
        locals.var_t1__blk809_dn6 = ((locals.var_vgsteff__blk840_dn6 * assign25680_e23146) + (locals.var_vgsteff__blk840 * (-((((((0.5 * locals.var_abulk_dn6) * locals.var_vdseff) + (assign25680_e23141 * locals.var_vdseff_dn6)) * locals.var_vgst2vtm) - (assign25680_e23143 * locals.var_vgst2vtm_dn6)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)))));
        locals.var_t1__blk809_dn7 = ((locals.var_vgsteff__blk840_dn7 * assign25680_e23146) + (locals.var_vgsteff__blk840 * (-((((((0.5 * locals.var_abulk_dn7) * locals.var_vdseff) + (assign25680_e23141 * locals.var_vdseff_dn7)) * locals.var_vgst2vtm) - (assign25680_e23143 * locals.var_vgst2vtm_dn7)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)))));
        locals.var_t1__blk809_dn8 = ((locals.var_vgsteff__blk840_dn8 * assign25680_e23146) + (locals.var_vgsteff__blk840 * (-((((((0.5 * locals.var_abulk_dn8) * locals.var_vdseff) + (assign25680_e23141 * locals.var_vdseff_dn8)) * locals.var_vgst2vtm) - (assign25680_e23143 * locals.var_vgst2vtm_dn8)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)))));
        locals.var_t1__blk809_dn9 = ((locals.var_vgsteff__blk840_dn9 * assign25680_e23146) + (locals.var_vgsteff__blk840 * (-((((((0.5 * locals.var_abulk_dn9) * locals.var_vdseff) + (assign25680_e23141 * locals.var_vdseff_dn9)) * locals.var_vgst2vtm) - (assign25680_e23143 * locals.var_vgst2vtm_dn9)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)))));
        locals.var_t1__blk809_dn10 = ((locals.var_vgsteff__blk840_dn10 * assign25680_e23146) + (locals.var_vgsteff__blk840 * (-((((((0.5 * locals.var_abulk_dn10) * locals.var_vdseff) + (assign25680_e23141 * locals.var_vdseff_dn10)) * locals.var_vgst2vtm) - (assign25680_e23143 * locals.var_vgst2vtm_dn10)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)))));
        locals.var_t1__blk809_dn11 = ((locals.var_vgsteff__blk840_dn11 * assign25680_e23146) + (locals.var_vgsteff__blk840 * (-((((((0.5 * locals.var_abulk_dn11) * locals.var_vdseff) + (assign25680_e23141 * locals.var_vdseff_dn11)) * locals.var_vgst2vtm) - (assign25680_e23143 * locals.var_vgst2vtm_dn11)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)))));
        locals.var_t1__blk809_dn12 = ((locals.var_vgsteff__blk840_dn12 * assign25680_e23146) + (locals.var_vgsteff__blk840 * (-((((((0.5 * locals.var_abulk_dn12) * locals.var_vdseff) + (assign25680_e23141 * locals.var_vdseff_dn12)) * locals.var_vgst2vtm) - (assign25680_e23143 * locals.var_vgst2vtm_dn12)) / (locals.var_vgst2vtm * locals.var_vgst2vtm)))));

        let assign25700_e23160: f64 = if p.p3 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1354 = assign25700_e23160;

        let (assign25710_e23166, assign25710_e23166_d_n3, assign25710_e23166_d_n4, assign25710_e23166_d_n5, assign25710_e23166_d_n6, assign25710_e23166_d_n7, assign25710_e23166_d_n8, assign25710_e23166_d_n9, assign25710_e23166_d_n10, assign25710_e23166_d_n11, assign25710_e23166_d_n12,) = {
    if (locals.var_guard1354 != 0.0) {
        let assign25710_e23164: f64 = (locals.var_ids_1 * p.p3);
        (assign25710_e23164, (locals.var_ids_1_dn3 * p.p3), (locals.var_ids_1_dn4 * p.p3), (locals.var_ids_1_dn5 * p.p3), (locals.var_ids_1_dn6 * p.p3), (locals.var_ids_1_dn7 * p.p3), (locals.var_ids_1_dn8 * p.p3), (locals.var_ids_1_dn9 * p.p3), (locals.var_ids_1_dn10 * p.p3), (locals.var_ids_1_dn11 * p.p3), (locals.var_ids_1_dn12 * p.p3),)
    } else {
        (locals.var_ids_1, locals.var_ids_1_dn3, locals.var_ids_1_dn4, locals.var_ids_1_dn5, locals.var_ids_1_dn6, locals.var_ids_1_dn7, locals.var_ids_1_dn8, locals.var_ids_1_dn9, locals.var_ids_1_dn10, locals.var_ids_1_dn11, locals.var_ids_1_dn12,)
    }
};
        locals.var_ids_1 = assign25710_e23166;
        locals.var_ids_1_dn3 = assign25710_e23166_d_n3;
        locals.var_ids_1_dn4 = assign25710_e23166_d_n4;
        locals.var_ids_1_dn5 = assign25710_e23166_d_n5;
        locals.var_ids_1_dn6 = assign25710_e23166_d_n6;
        locals.var_ids_1_dn7 = assign25710_e23166_d_n7;
        locals.var_ids_1_dn8 = assign25710_e23166_d_n8;
        locals.var_ids_1_dn9 = assign25710_e23166_d_n9;
        locals.var_ids_1_dn10 = assign25710_e23166_d_n10;
        locals.var_ids_1_dn11 = assign25710_e23166_d_n11;
        locals.var_ids_1_dn12 = assign25710_e23166_d_n12;

        let (assign25720_e23172, assign25720_e23172_d_n3, assign25720_e23172_d_n4, assign25720_e23172_d_n5, assign25720_e23172_d_n6, assign25720_e23172_d_n7, assign25720_e23172_d_n8, assign25720_e23172_d_n9, assign25720_e23172_d_n10, assign25720_e23172_d_n11, assign25720_e23172_d_n12,) = {
    if (locals.var_guard1354 != 0.0) {
        let assign25720_e23170: f64 = (locals.var_ic_1 * p.p3);
        (assign25720_e23170, (locals.var_ic_1_dn3 * p.p3), (locals.var_ic_1_dn4 * p.p3), (locals.var_ic_1_dn5 * p.p3), (locals.var_ic_1_dn6 * p.p3), (locals.var_ic_1_dn7 * p.p3), (locals.var_ic_1_dn8 * p.p3), (locals.var_ic_1_dn9 * p.p3), (locals.var_ic_1_dn10 * p.p3), (locals.var_ic_1_dn11 * p.p3), (locals.var_ic_1_dn12 * p.p3),)
    } else {
        (locals.var_ic_1, locals.var_ic_1_dn3, locals.var_ic_1_dn4, locals.var_ic_1_dn5, locals.var_ic_1_dn6, locals.var_ic_1_dn7, locals.var_ic_1_dn8, locals.var_ic_1_dn9, locals.var_ic_1_dn10, locals.var_ic_1_dn11, locals.var_ic_1_dn12,)
    }
};
        locals.var_ic_1 = assign25720_e23172;
        locals.var_ic_1_dn3 = assign25720_e23172_d_n3;
        locals.var_ic_1_dn4 = assign25720_e23172_d_n4;
        locals.var_ic_1_dn5 = assign25720_e23172_d_n5;
        locals.var_ic_1_dn6 = assign25720_e23172_d_n6;
        locals.var_ic_1_dn7 = assign25720_e23172_d_n7;
        locals.var_ic_1_dn8 = assign25720_e23172_d_n8;
        locals.var_ic_1_dn9 = assign25720_e23172_d_n9;
        locals.var_ic_1_dn10 = assign25720_e23172_d_n10;
        locals.var_ic_1_dn11 = assign25720_e23172_d_n11;
        locals.var_ic_1_dn12 = assign25720_e23172_d_n12;

        let (assign25730_e23178, assign25730_e23178_d_n3, assign25730_e23178_d_n4, assign25730_e23178_d_n5, assign25730_e23178_d_n6, assign25730_e23178_d_n7, assign25730_e23178_d_n8, assign25730_e23178_d_n9, assign25730_e23178_d_n10, assign25730_e23178_d_n11, assign25730_e23178_d_n12,) = {
    if (locals.var_guard1354 != 0.0) {
        let assign25730_e23176: f64 = (locals.var_b4soiidovvds * p.p3);
        (assign25730_e23176, (locals.var_b4soiidovvds_dn3 * p.p3), (locals.var_b4soiidovvds_dn4 * p.p3), (locals.var_b4soiidovvds_dn5 * p.p3), (locals.var_b4soiidovvds_dn6 * p.p3), (locals.var_b4soiidovvds_dn7 * p.p3), (locals.var_b4soiidovvds_dn8 * p.p3), (locals.var_b4soiidovvds_dn9 * p.p3), (locals.var_b4soiidovvds_dn10 * p.p3), (locals.var_b4soiidovvds_dn11 * p.p3), (locals.var_b4soiidovvds_dn12 * p.p3),)
    } else {
        (locals.var_b4soiidovvds, locals.var_b4soiidovvds_dn3, locals.var_b4soiidovvds_dn4, locals.var_b4soiidovvds_dn5, locals.var_b4soiidovvds_dn6, locals.var_b4soiidovvds_dn7, locals.var_b4soiidovvds_dn8, locals.var_b4soiidovvds_dn9, locals.var_b4soiidovvds_dn10, locals.var_b4soiidovvds_dn11, locals.var_b4soiidovvds_dn12,)
    }
};
        locals.var_b4soiidovvds = assign25730_e23178;
        locals.var_b4soiidovvds_dn3 = assign25730_e23178_d_n3;
        locals.var_b4soiidovvds_dn4 = assign25730_e23178_d_n4;
        locals.var_b4soiidovvds_dn5 = assign25730_e23178_d_n5;
        locals.var_b4soiidovvds_dn6 = assign25730_e23178_d_n6;
        locals.var_b4soiidovvds_dn7 = assign25730_e23178_d_n7;
        locals.var_b4soiidovvds_dn8 = assign25730_e23178_d_n8;
        locals.var_b4soiidovvds_dn9 = assign25730_e23178_d_n9;
        locals.var_b4soiidovvds_dn10 = assign25730_e23178_d_n10;
        locals.var_b4soiidovvds_dn11 = assign25730_e23178_d_n11;
        locals.var_b4soiidovvds_dn12 = assign25730_e23178_d_n12;

        let (assign25740_e23184, assign25740_e23184_d_n3, assign25740_e23184_d_n4, assign25740_e23184_d_n5, assign25740_e23184_d_n6, assign25740_e23184_d_n7, assign25740_e23184_d_n8, assign25740_e23184_d_n9, assign25740_e23184_d_n10, assign25740_e23184_d_n11, assign25740_e23184_d_n12,) = {
    if (locals.var_guard1354 != 0.0) {
        let assign25740_e23182: f64 = (locals.var_ibs_1 * p.p3);
        (assign25740_e23182, (locals.var_ibs_1_dn3 * p.p3), (locals.var_ibs_1_dn4 * p.p3), (locals.var_ibs_1_dn5 * p.p3), (locals.var_ibs_1_dn6 * p.p3), (locals.var_ibs_1_dn7 * p.p3), (locals.var_ibs_1_dn8 * p.p3), (locals.var_ibs_1_dn9 * p.p3), (locals.var_ibs_1_dn10 * p.p3), (locals.var_ibs_1_dn11 * p.p3), (locals.var_ibs_1_dn12 * p.p3),)
    } else {
        (locals.var_ibs_1, locals.var_ibs_1_dn3, locals.var_ibs_1_dn4, locals.var_ibs_1_dn5, locals.var_ibs_1_dn6, locals.var_ibs_1_dn7, locals.var_ibs_1_dn8, locals.var_ibs_1_dn9, locals.var_ibs_1_dn10, locals.var_ibs_1_dn11, locals.var_ibs_1_dn12,)
    }
};
        locals.var_ibs_1 = assign25740_e23184;
        locals.var_ibs_1_dn3 = assign25740_e23184_d_n3;
        locals.var_ibs_1_dn4 = assign25740_e23184_d_n4;
        locals.var_ibs_1_dn5 = assign25740_e23184_d_n5;
        locals.var_ibs_1_dn6 = assign25740_e23184_d_n6;
        locals.var_ibs_1_dn7 = assign25740_e23184_d_n7;
        locals.var_ibs_1_dn8 = assign25740_e23184_d_n8;
        locals.var_ibs_1_dn9 = assign25740_e23184_d_n9;
        locals.var_ibs_1_dn10 = assign25740_e23184_d_n10;
        locals.var_ibs_1_dn11 = assign25740_e23184_d_n11;
        locals.var_ibs_1_dn12 = assign25740_e23184_d_n12;

        let (assign25750_e23190, assign25750_e23190_d_n3, assign25750_e23190_d_n4, assign25750_e23190_d_n5, assign25750_e23190_d_n6, assign25750_e23190_d_n7, assign25750_e23190_d_n8, assign25750_e23190_d_n9, assign25750_e23190_d_n10, assign25750_e23190_d_n11, assign25750_e23190_d_n12,) = {
    if (locals.var_guard1354 != 0.0) {
        let assign25750_e23188: f64 = (locals.var_ibd_1 * p.p3);
        (assign25750_e23188, (locals.var_ibd_1_dn3 * p.p3), (locals.var_ibd_1_dn4 * p.p3), (locals.var_ibd_1_dn5 * p.p3), (locals.var_ibd_1_dn6 * p.p3), (locals.var_ibd_1_dn7 * p.p3), (locals.var_ibd_1_dn8 * p.p3), (locals.var_ibd_1_dn9 * p.p3), (locals.var_ibd_1_dn10 * p.p3), (locals.var_ibd_1_dn11 * p.p3), (locals.var_ibd_1_dn12 * p.p3),)
    } else {
        (locals.var_ibd_1, locals.var_ibd_1_dn3, locals.var_ibd_1_dn4, locals.var_ibd_1_dn5, locals.var_ibd_1_dn6, locals.var_ibd_1_dn7, locals.var_ibd_1_dn8, locals.var_ibd_1_dn9, locals.var_ibd_1_dn10, locals.var_ibd_1_dn11, locals.var_ibd_1_dn12,)
    }
};
        locals.var_ibd_1 = assign25750_e23190;
        locals.var_ibd_1_dn3 = assign25750_e23190_d_n3;
        locals.var_ibd_1_dn4 = assign25750_e23190_d_n4;
        locals.var_ibd_1_dn5 = assign25750_e23190_d_n5;
        locals.var_ibd_1_dn6 = assign25750_e23190_d_n6;
        locals.var_ibd_1_dn7 = assign25750_e23190_d_n7;
        locals.var_ibd_1_dn8 = assign25750_e23190_d_n8;
        locals.var_ibd_1_dn9 = assign25750_e23190_d_n9;
        locals.var_ibd_1_dn10 = assign25750_e23190_d_n10;
        locals.var_ibd_1_dn11 = assign25750_e23190_d_n11;
        locals.var_ibd_1_dn12 = assign25750_e23190_d_n12;

    }

    pub(super) fn stamp_transient_block_77(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25760_e23196, assign25760_e23196_d_n3, assign25760_e23196_d_n4, assign25760_e23196_d_n5, assign25760_e23196_d_n6, assign25760_e23196_d_n7, assign25760_e23196_d_n8, assign25760_e23196_d_n9, assign25760_e23196_d_n10, assign25760_e23196_d_n11, assign25760_e23196_d_n12,) = {
    if (locals.var_guard1354 != 0.0) {
        let assign25760_e23194: f64 = (locals.var_igcs_1 * p.p3);
        (assign25760_e23194, (locals.var_igcs_1_dn3 * p.p3), (locals.var_igcs_1_dn4 * p.p3), (locals.var_igcs_1_dn5 * p.p3), (locals.var_igcs_1_dn6 * p.p3), (locals.var_igcs_1_dn7 * p.p3), (locals.var_igcs_1_dn8 * p.p3), (locals.var_igcs_1_dn9 * p.p3), (locals.var_igcs_1_dn10 * p.p3), (locals.var_igcs_1_dn11 * p.p3), (locals.var_igcs_1_dn12 * p.p3),)
    } else {
        (locals.var_igcs_1, locals.var_igcs_1_dn3, locals.var_igcs_1_dn4, locals.var_igcs_1_dn5, locals.var_igcs_1_dn6, locals.var_igcs_1_dn7, locals.var_igcs_1_dn8, locals.var_igcs_1_dn9, locals.var_igcs_1_dn10, locals.var_igcs_1_dn11, locals.var_igcs_1_dn12,)
    }
};
        locals.var_igcs_1 = assign25760_e23196;
        locals.var_igcs_1_dn3 = assign25760_e23196_d_n3;
        locals.var_igcs_1_dn4 = assign25760_e23196_d_n4;
        locals.var_igcs_1_dn5 = assign25760_e23196_d_n5;
        locals.var_igcs_1_dn6 = assign25760_e23196_d_n6;
        locals.var_igcs_1_dn7 = assign25760_e23196_d_n7;
        locals.var_igcs_1_dn8 = assign25760_e23196_d_n8;
        locals.var_igcs_1_dn9 = assign25760_e23196_d_n9;
        locals.var_igcs_1_dn10 = assign25760_e23196_d_n10;
        locals.var_igcs_1_dn11 = assign25760_e23196_d_n11;
        locals.var_igcs_1_dn12 = assign25760_e23196_d_n12;

        let (assign25770_e23202, assign25770_e23202_d_n3, assign25770_e23202_d_n4, assign25770_e23202_d_n5, assign25770_e23202_d_n6, assign25770_e23202_d_n7, assign25770_e23202_d_n8, assign25770_e23202_d_n9, assign25770_e23202_d_n10, assign25770_e23202_d_n11, assign25770_e23202_d_n12,) = {
    if (locals.var_guard1354 != 0.0) {
        let assign25770_e23200: f64 = (locals.var_igcd_1 * p.p3);
        (assign25770_e23200, (locals.var_igcd_1_dn3 * p.p3), (locals.var_igcd_1_dn4 * p.p3), (locals.var_igcd_1_dn5 * p.p3), (locals.var_igcd_1_dn6 * p.p3), (locals.var_igcd_1_dn7 * p.p3), (locals.var_igcd_1_dn8 * p.p3), (locals.var_igcd_1_dn9 * p.p3), (locals.var_igcd_1_dn10 * p.p3), (locals.var_igcd_1_dn11 * p.p3), (locals.var_igcd_1_dn12 * p.p3),)
    } else {
        (locals.var_igcd_1, locals.var_igcd_1_dn3, locals.var_igcd_1_dn4, locals.var_igcd_1_dn5, locals.var_igcd_1_dn6, locals.var_igcd_1_dn7, locals.var_igcd_1_dn8, locals.var_igcd_1_dn9, locals.var_igcd_1_dn10, locals.var_igcd_1_dn11, locals.var_igcd_1_dn12,)
    }
};
        locals.var_igcd_1 = assign25770_e23202;
        locals.var_igcd_1_dn3 = assign25770_e23202_d_n3;
        locals.var_igcd_1_dn4 = assign25770_e23202_d_n4;
        locals.var_igcd_1_dn5 = assign25770_e23202_d_n5;
        locals.var_igcd_1_dn6 = assign25770_e23202_d_n6;
        locals.var_igcd_1_dn7 = assign25770_e23202_d_n7;
        locals.var_igcd_1_dn8 = assign25770_e23202_d_n8;
        locals.var_igcd_1_dn9 = assign25770_e23202_d_n9;
        locals.var_igcd_1_dn10 = assign25770_e23202_d_n10;
        locals.var_igcd_1_dn11 = assign25770_e23202_d_n11;
        locals.var_igcd_1_dn12 = assign25770_e23202_d_n12;

        let (assign25780_e23208, assign25780_e23208_d_n3, assign25780_e23208_d_n4, assign25780_e23208_d_n5, assign25780_e23208_d_n6, assign25780_e23208_d_n7, assign25780_e23208_d_n8, assign25780_e23208_d_n9, assign25780_e23208_d_n10, assign25780_e23208_d_n11, assign25780_e23208_d_n12,) = {
    if (locals.var_guard1354 != 0.0) {
        let assign25780_e23206: f64 = (locals.var_igs_1 * p.p3);
        (assign25780_e23206, (locals.var_igs_1_dn3 * p.p3), (locals.var_igs_1_dn4 * p.p3), (locals.var_igs_1_dn5 * p.p3), (locals.var_igs_1_dn6 * p.p3), (locals.var_igs_1_dn7 * p.p3), (locals.var_igs_1_dn8 * p.p3), (locals.var_igs_1_dn9 * p.p3), (locals.var_igs_1_dn10 * p.p3), (locals.var_igs_1_dn11 * p.p3), (locals.var_igs_1_dn12 * p.p3),)
    } else {
        (locals.var_igs_1, locals.var_igs_1_dn3, locals.var_igs_1_dn4, locals.var_igs_1_dn5, locals.var_igs_1_dn6, locals.var_igs_1_dn7, locals.var_igs_1_dn8, locals.var_igs_1_dn9, locals.var_igs_1_dn10, locals.var_igs_1_dn11, locals.var_igs_1_dn12,)
    }
};
        locals.var_igs_1 = assign25780_e23208;
        locals.var_igs_1_dn3 = assign25780_e23208_d_n3;
        locals.var_igs_1_dn4 = assign25780_e23208_d_n4;
        locals.var_igs_1_dn5 = assign25780_e23208_d_n5;
        locals.var_igs_1_dn6 = assign25780_e23208_d_n6;
        locals.var_igs_1_dn7 = assign25780_e23208_d_n7;
        locals.var_igs_1_dn8 = assign25780_e23208_d_n8;
        locals.var_igs_1_dn9 = assign25780_e23208_d_n9;
        locals.var_igs_1_dn10 = assign25780_e23208_d_n10;
        locals.var_igs_1_dn11 = assign25780_e23208_d_n11;
        locals.var_igs_1_dn12 = assign25780_e23208_d_n12;

        let (assign25790_e23214, assign25790_e23214_d_n3, assign25790_e23214_d_n4, assign25790_e23214_d_n5, assign25790_e23214_d_n6, assign25790_e23214_d_n7, assign25790_e23214_d_n8, assign25790_e23214_d_n9, assign25790_e23214_d_n10, assign25790_e23214_d_n11, assign25790_e23214_d_n12,) = {
    if (locals.var_guard1354 != 0.0) {
        let assign25790_e23212: f64 = (locals.var_igd_1 * p.p3);
        (assign25790_e23212, (locals.var_igd_1_dn3 * p.p3), (locals.var_igd_1_dn4 * p.p3), (locals.var_igd_1_dn5 * p.p3), (locals.var_igd_1_dn6 * p.p3), (locals.var_igd_1_dn7 * p.p3), (locals.var_igd_1_dn8 * p.p3), (locals.var_igd_1_dn9 * p.p3), (locals.var_igd_1_dn10 * p.p3), (locals.var_igd_1_dn11 * p.p3), (locals.var_igd_1_dn12 * p.p3),)
    } else {
        (locals.var_igd_1, locals.var_igd_1_dn3, locals.var_igd_1_dn4, locals.var_igd_1_dn5, locals.var_igd_1_dn6, locals.var_igd_1_dn7, locals.var_igd_1_dn8, locals.var_igd_1_dn9, locals.var_igd_1_dn10, locals.var_igd_1_dn11, locals.var_igd_1_dn12,)
    }
};
        locals.var_igd_1 = assign25790_e23214;
        locals.var_igd_1_dn3 = assign25790_e23214_d_n3;
        locals.var_igd_1_dn4 = assign25790_e23214_d_n4;
        locals.var_igd_1_dn5 = assign25790_e23214_d_n5;
        locals.var_igd_1_dn6 = assign25790_e23214_d_n6;
        locals.var_igd_1_dn7 = assign25790_e23214_d_n7;
        locals.var_igd_1_dn8 = assign25790_e23214_d_n8;
        locals.var_igd_1_dn9 = assign25790_e23214_d_n9;
        locals.var_igd_1_dn10 = assign25790_e23214_d_n10;
        locals.var_igd_1_dn11 = assign25790_e23214_d_n11;
        locals.var_igd_1_dn12 = assign25790_e23214_d_n12;

        let (assign25800_e23220, assign25800_e23220_d_n3, assign25800_e23220_d_n4, assign25800_e23220_d_n5, assign25800_e23220_d_n6, assign25800_e23220_d_n7, assign25800_e23220_d_n8, assign25800_e23220_d_n9, assign25800_e23220_d_n10, assign25800_e23220_d_n11, assign25800_e23220_d_n12,) = {
    if (locals.var_guard1354 != 0.0) {
        let assign25800_e23218: f64 = (locals.var_iii * p.p3);
        (assign25800_e23218, (locals.var_iii_dn3 * p.p3), (locals.var_iii_dn4 * p.p3), (locals.var_iii_dn5 * p.p3), (locals.var_iii_dn6 * p.p3), (locals.var_iii_dn7 * p.p3), (locals.var_iii_dn8 * p.p3), (locals.var_iii_dn9 * p.p3), (locals.var_iii_dn10 * p.p3), (locals.var_iii_dn11 * p.p3), (locals.var_iii_dn12 * p.p3),)
    } else {
        (locals.var_iii, locals.var_iii_dn3, locals.var_iii_dn4, locals.var_iii_dn5, locals.var_iii_dn6, locals.var_iii_dn7, locals.var_iii_dn8, locals.var_iii_dn9, locals.var_iii_dn10, locals.var_iii_dn11, locals.var_iii_dn12,)
    }
};
        locals.var_iii = assign25800_e23220;
        locals.var_iii_dn3 = assign25800_e23220_d_n3;
        locals.var_iii_dn4 = assign25800_e23220_d_n4;
        locals.var_iii_dn5 = assign25800_e23220_d_n5;
        locals.var_iii_dn6 = assign25800_e23220_d_n6;
        locals.var_iii_dn7 = assign25800_e23220_d_n7;
        locals.var_iii_dn8 = assign25800_e23220_d_n8;
        locals.var_iii_dn9 = assign25800_e23220_d_n9;
        locals.var_iii_dn10 = assign25800_e23220_d_n10;
        locals.var_iii_dn11 = assign25800_e23220_d_n11;
        locals.var_iii_dn12 = assign25800_e23220_d_n12;

        let (assign25810_e23226, assign25810_e23226_d_n3, assign25810_e23226_d_n4, assign25810_e23226_d_n5, assign25810_e23226_d_n6, assign25810_e23226_d_n7, assign25810_e23226_d_n8, assign25810_e23226_d_n9, assign25810_e23226_d_n10, assign25810_e23226_d_n11, assign25810_e23226_d_n12,) = {
    if (locals.var_guard1354 != 0.0) {
        let assign25810_e23224: f64 = (locals.var_b4soiig * p.p3);
        (assign25810_e23224, (locals.var_b4soiig_dn3 * p.p3), (locals.var_b4soiig_dn4 * p.p3), (locals.var_b4soiig_dn5 * p.p3), (locals.var_b4soiig_dn6 * p.p3), (locals.var_b4soiig_dn7 * p.p3), (locals.var_b4soiig_dn8 * p.p3), (locals.var_b4soiig_dn9 * p.p3), (locals.var_b4soiig_dn10 * p.p3), (locals.var_b4soiig_dn11 * p.p3), (locals.var_b4soiig_dn12 * p.p3),)
    } else {
        (locals.var_b4soiig, locals.var_b4soiig_dn3, locals.var_b4soiig_dn4, locals.var_b4soiig_dn5, locals.var_b4soiig_dn6, locals.var_b4soiig_dn7, locals.var_b4soiig_dn8, locals.var_b4soiig_dn9, locals.var_b4soiig_dn10, locals.var_b4soiig_dn11, locals.var_b4soiig_dn12,)
    }
};
        locals.var_b4soiig = assign25810_e23226;
        locals.var_b4soiig_dn3 = assign25810_e23226_d_n3;
        locals.var_b4soiig_dn4 = assign25810_e23226_d_n4;
        locals.var_b4soiig_dn5 = assign25810_e23226_d_n5;
        locals.var_b4soiig_dn6 = assign25810_e23226_d_n6;
        locals.var_b4soiig_dn7 = assign25810_e23226_d_n7;
        locals.var_b4soiig_dn8 = assign25810_e23226_d_n8;
        locals.var_b4soiig_dn9 = assign25810_e23226_d_n9;
        locals.var_b4soiig_dn10 = assign25810_e23226_d_n10;
        locals.var_b4soiig_dn11 = assign25810_e23226_d_n11;
        locals.var_b4soiig_dn12 = assign25810_e23226_d_n12;

        let (assign25820_e23232, assign25820_e23232_d_n3, assign25820_e23232_d_n4, assign25820_e23232_d_n5, assign25820_e23232_d_n6, assign25820_e23232_d_n7, assign25820_e23232_d_n8, assign25820_e23232_d_n9, assign25820_e23232_d_n10, assign25820_e23232_d_n11, assign25820_e23232_d_n12,) = {
    if (locals.var_guard1354 != 0.0) {
        let assign25820_e23230: f64 = (locals.var_igidl_1 * p.p3);
        (assign25820_e23230, (locals.var_igidl_1_dn3 * p.p3), (locals.var_igidl_1_dn4 * p.p3), (locals.var_igidl_1_dn5 * p.p3), (locals.var_igidl_1_dn6 * p.p3), (locals.var_igidl_1_dn7 * p.p3), (locals.var_igidl_1_dn8 * p.p3), (locals.var_igidl_1_dn9 * p.p3), (locals.var_igidl_1_dn10 * p.p3), (locals.var_igidl_1_dn11 * p.p3), (locals.var_igidl_1_dn12 * p.p3),)
    } else {
        (locals.var_igidl_1, locals.var_igidl_1_dn3, locals.var_igidl_1_dn4, locals.var_igidl_1_dn5, locals.var_igidl_1_dn6, locals.var_igidl_1_dn7, locals.var_igidl_1_dn8, locals.var_igidl_1_dn9, locals.var_igidl_1_dn10, locals.var_igidl_1_dn11, locals.var_igidl_1_dn12,)
    }
};
        locals.var_igidl_1 = assign25820_e23232;
        locals.var_igidl_1_dn3 = assign25820_e23232_d_n3;
        locals.var_igidl_1_dn4 = assign25820_e23232_d_n4;
        locals.var_igidl_1_dn5 = assign25820_e23232_d_n5;
        locals.var_igidl_1_dn6 = assign25820_e23232_d_n6;
        locals.var_igidl_1_dn7 = assign25820_e23232_d_n7;
        locals.var_igidl_1_dn8 = assign25820_e23232_d_n8;
        locals.var_igidl_1_dn9 = assign25820_e23232_d_n9;
        locals.var_igidl_1_dn10 = assign25820_e23232_d_n10;
        locals.var_igidl_1_dn11 = assign25820_e23232_d_n11;
        locals.var_igidl_1_dn12 = assign25820_e23232_d_n12;

        let (assign25830_e23238, assign25830_e23238_d_n3, assign25830_e23238_d_n4, assign25830_e23238_d_n5, assign25830_e23238_d_n6, assign25830_e23238_d_n7, assign25830_e23238_d_n8, assign25830_e23238_d_n9, assign25830_e23238_d_n10, assign25830_e23238_d_n11, assign25830_e23238_d_n12,) = {
    if (locals.var_guard1354 != 0.0) {
        let assign25830_e23236: f64 = (locals.var_igisl_1 * p.p3);
        (assign25830_e23236, (locals.var_igisl_1_dn3 * p.p3), (locals.var_igisl_1_dn4 * p.p3), (locals.var_igisl_1_dn5 * p.p3), (locals.var_igisl_1_dn6 * p.p3), (locals.var_igisl_1_dn7 * p.p3), (locals.var_igisl_1_dn8 * p.p3), (locals.var_igisl_1_dn9 * p.p3), (locals.var_igisl_1_dn10 * p.p3), (locals.var_igisl_1_dn11 * p.p3), (locals.var_igisl_1_dn12 * p.p3),)
    } else {
        (locals.var_igisl_1, locals.var_igisl_1_dn3, locals.var_igisl_1_dn4, locals.var_igisl_1_dn5, locals.var_igisl_1_dn6, locals.var_igisl_1_dn7, locals.var_igisl_1_dn8, locals.var_igisl_1_dn9, locals.var_igisl_1_dn10, locals.var_igisl_1_dn11, locals.var_igisl_1_dn12,)
    }
};
        locals.var_igisl_1 = assign25830_e23238;
        locals.var_igisl_1_dn3 = assign25830_e23238_d_n3;
        locals.var_igisl_1_dn4 = assign25830_e23238_d_n4;
        locals.var_igisl_1_dn5 = assign25830_e23238_d_n5;
        locals.var_igisl_1_dn6 = assign25830_e23238_d_n6;
        locals.var_igisl_1_dn7 = assign25830_e23238_d_n7;
        locals.var_igisl_1_dn8 = assign25830_e23238_d_n8;
        locals.var_igisl_1_dn9 = assign25830_e23238_d_n9;
        locals.var_igisl_1_dn10 = assign25830_e23238_d_n10;
        locals.var_igisl_1_dn11 = assign25830_e23238_d_n11;
        locals.var_igisl_1_dn12 = assign25830_e23238_d_n12;

        let assign25840_e23242: f64 = locals.var_ids_1_dn9;
        let assign25840_e23243: f64 = (p.p37 * assign25840_e23242);
        locals.var_b4soigm = assign25840_e23243;
        locals.var_b4soigm_dn3 = 0.0;
        locals.var_b4soigm_dn4 = 0.0;
        locals.var_b4soigm_dn5 = 0.0;
        locals.var_b4soigm_dn6 = 0.0;
        locals.var_b4soigm_dn7 = 0.0;
        locals.var_b4soigm_dn8 = 0.0;
        locals.var_b4soigm_dn9 = 0.0;
        locals.var_b4soigm_dn10 = 0.0;
        locals.var_b4soigm_dn11 = 0.0;
        locals.var_b4soigm_dn12 = 0.0;

        let assign25850_e23246: f64 = if locals.var_b4soimode > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1355 = assign25850_e23246;

        let (assign25860_e23254, assign25860_e23254_d_n3, assign25860_e23254_d_n4, assign25860_e23254_d_n5, assign25860_e23254_d_n6, assign25860_e23254_d_n7, assign25860_e23254_d_n8, assign25860_e23254_d_n9, assign25860_e23254_d_n10, assign25860_e23254_d_n11, assign25860_e23254_d_n12,) = {
    if (locals.var_guard1355 != 0.0) {
        let assign25860_e23251: f64 = locals.var_ids_1_dn7;
        let assign25860_e23252: f64 = (p.p37 * assign25860_e23251);
        (assign25860_e23252, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soigds, locals.var_b4soigds_dn3, locals.var_b4soigds_dn4, locals.var_b4soigds_dn5, locals.var_b4soigds_dn6, locals.var_b4soigds_dn7, locals.var_b4soigds_dn8, locals.var_b4soigds_dn9, locals.var_b4soigds_dn10, locals.var_b4soigds_dn11, locals.var_b4soigds_dn12,)
    }
};
        locals.var_b4soigds = assign25860_e23254;
        locals.var_b4soigds_dn3 = assign25860_e23254_d_n3;
        locals.var_b4soigds_dn4 = assign25860_e23254_d_n4;
        locals.var_b4soigds_dn5 = assign25860_e23254_d_n5;
        locals.var_b4soigds_dn6 = assign25860_e23254_d_n6;
        locals.var_b4soigds_dn7 = assign25860_e23254_d_n7;
        locals.var_b4soigds_dn8 = assign25860_e23254_d_n8;
        locals.var_b4soigds_dn9 = assign25860_e23254_d_n9;
        locals.var_b4soigds_dn10 = assign25860_e23254_d_n10;
        locals.var_b4soigds_dn11 = assign25860_e23254_d_n11;
        locals.var_b4soigds_dn12 = assign25860_e23254_d_n12;

        let (assign25870_e23263, assign25870_e23263_d_n3, assign25870_e23263_d_n4, assign25870_e23263_d_n5, assign25870_e23263_d_n6, assign25870_e23263_d_n7, assign25870_e23263_d_n8, assign25870_e23263_d_n9, assign25870_e23263_d_n10, assign25870_e23263_d_n11, assign25870_e23263_d_n12,) = {
    if (locals.var_guard1355 == 0.0) {
        let assign25870_e23260: f64 = locals.var_ids_1_dn8;
        let assign25870_e23261: f64 = (p.p37 * assign25870_e23260);
        (assign25870_e23261, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_b4soigds, locals.var_b4soigds_dn3, locals.var_b4soigds_dn4, locals.var_b4soigds_dn5, locals.var_b4soigds_dn6, locals.var_b4soigds_dn7, locals.var_b4soigds_dn8, locals.var_b4soigds_dn9, locals.var_b4soigds_dn10, locals.var_b4soigds_dn11, locals.var_b4soigds_dn12,)
    }
};
        locals.var_b4soigds = assign25870_e23263;
        locals.var_b4soigds_dn3 = assign25870_e23263_d_n3;
        locals.var_b4soigds_dn4 = assign25870_e23263_d_n4;
        locals.var_b4soigds_dn5 = assign25870_e23263_d_n5;
        locals.var_b4soigds_dn6 = assign25870_e23263_d_n6;
        locals.var_b4soigds_dn7 = assign25870_e23263_d_n7;
        locals.var_b4soigds_dn8 = assign25870_e23263_d_n8;
        locals.var_b4soigds_dn9 = assign25870_e23263_d_n9;
        locals.var_b4soigds_dn10 = assign25870_e23263_d_n10;
        locals.var_b4soigds_dn11 = assign25870_e23263_d_n11;
        locals.var_b4soigds_dn12 = assign25870_e23263_d_n12;

        let assign25880_e23267: f64 = locals.var_ids_1_dn5;
        let assign25880_e23268: f64 = (p.p37 * assign25880_e23267);
        locals.var_b4soigmbs = assign25880_e23268;
        locals.var_b4soigmbs_dn3 = 0.0;
        locals.var_b4soigmbs_dn4 = 0.0;
        locals.var_b4soigmbs_dn5 = 0.0;
        locals.var_b4soigmbs_dn6 = 0.0;
        locals.var_b4soigmbs_dn7 = 0.0;
        locals.var_b4soigmbs_dn8 = 0.0;
        locals.var_b4soigmbs_dn9 = 0.0;
        locals.var_b4soigmbs_dn10 = 0.0;
        locals.var_b4soigmbs_dn11 = 0.0;
        locals.var_b4soigmbs_dn12 = 0.0;

        let assign25890_e23272: f64 = (locals.var_pparam_b4soiweffcv / p.p23);
        let assign25890_e23274: f64 = (assign25890_e23272 * p.p3);
        let assign25890_e23276: f64 = (assign25890_e23274 * locals.var_pparam_b4soileffcv);
        let assign25890_e23278: f64 = (assign25890_e23276 + p.p26);
        let assign25890_e23279: f64 = (locals.var_b4soicox * assign25890_e23278);
        locals.var_coxwl = assign25890_e23279;
        locals.var_coxwl_dn3 = (locals.var_b4soicox * ((((locals.var_pparam_b4soiweffcv_dn3 / p.p23) * p.p3) * locals.var_pparam_b4soileffcv) + (assign25890_e23274 * locals.var_pparam_b4soileffcv_dn3)));
        locals.var_coxwl_dn4 = (locals.var_b4soicox * ((((locals.var_pparam_b4soiweffcv_dn4 / p.p23) * p.p3) * locals.var_pparam_b4soileffcv) + (assign25890_e23274 * locals.var_pparam_b4soileffcv_dn4)));
        locals.var_coxwl_dn5 = (locals.var_b4soicox * ((((locals.var_pparam_b4soiweffcv_dn5 / p.p23) * p.p3) * locals.var_pparam_b4soileffcv) + (assign25890_e23274 * locals.var_pparam_b4soileffcv_dn5)));
        locals.var_coxwl_dn6 = (locals.var_b4soicox * ((((locals.var_pparam_b4soiweffcv_dn6 / p.p23) * p.p3) * locals.var_pparam_b4soileffcv) + (assign25890_e23274 * locals.var_pparam_b4soileffcv_dn6)));
        locals.var_coxwl_dn7 = (locals.var_b4soicox * ((((locals.var_pparam_b4soiweffcv_dn7 / p.p23) * p.p3) * locals.var_pparam_b4soileffcv) + (assign25890_e23274 * locals.var_pparam_b4soileffcv_dn7)));
        locals.var_coxwl_dn8 = (locals.var_b4soicox * ((((locals.var_pparam_b4soiweffcv_dn8 / p.p23) * p.p3) * locals.var_pparam_b4soileffcv) + (assign25890_e23274 * locals.var_pparam_b4soileffcv_dn8)));
        locals.var_coxwl_dn9 = (locals.var_b4soicox * ((((locals.var_pparam_b4soiweffcv_dn9 / p.p23) * p.p3) * locals.var_pparam_b4soileffcv) + (assign25890_e23274 * locals.var_pparam_b4soileffcv_dn9)));
        locals.var_coxwl_dn10 = (locals.var_b4soicox * ((((locals.var_pparam_b4soiweffcv_dn10 / p.p23) * p.p3) * locals.var_pparam_b4soileffcv) + (assign25890_e23274 * locals.var_pparam_b4soileffcv_dn10)));
        locals.var_coxwl_dn11 = (locals.var_b4soicox * ((((locals.var_pparam_b4soiweffcv_dn11 / p.p23) * p.p3) * locals.var_pparam_b4soileffcv) + (assign25890_e23274 * locals.var_pparam_b4soileffcv_dn11)));
        locals.var_coxwl_dn12 = (locals.var_b4soicox * ((((locals.var_pparam_b4soiweffcv_dn12 / p.p23) * p.p3) * locals.var_pparam_b4soileffcv) + (assign25890_e23274 * locals.var_pparam_b4soileffcv_dn12)));

        let assign25900_e23282: f64 = (p.p361 * locals.var_b4soicox);
        let assign25900_e23285: f64 = (locals.var_pparam_b4soiweffcv / p.p23);
        let assign25900_e23287: f64 = (assign25900_e23285 * p.p3);
        let assign25900_e23289: f64 = (assign25900_e23287 * locals.var_pparam_b4soileffcvb);
        let assign25900_e23291: f64 = (assign25900_e23289 + p.p26);
        let assign25900_e23292: f64 = (assign25900_e23282 * assign25900_e23291);
        locals.var_coxwlb = assign25900_e23292;
        locals.var_coxwlb_dn3 = (assign25900_e23282 * ((((locals.var_pparam_b4soiweffcv_dn3 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvb) + (assign25900_e23287 * locals.var_pparam_b4soileffcvb_dn3)));
        locals.var_coxwlb_dn4 = (assign25900_e23282 * ((((locals.var_pparam_b4soiweffcv_dn4 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvb) + (assign25900_e23287 * locals.var_pparam_b4soileffcvb_dn4)));
        locals.var_coxwlb_dn5 = (assign25900_e23282 * ((((locals.var_pparam_b4soiweffcv_dn5 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvb) + (assign25900_e23287 * locals.var_pparam_b4soileffcvb_dn5)));
        locals.var_coxwlb_dn6 = (assign25900_e23282 * ((((locals.var_pparam_b4soiweffcv_dn6 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvb) + (assign25900_e23287 * locals.var_pparam_b4soileffcvb_dn6)));
        locals.var_coxwlb_dn7 = (assign25900_e23282 * ((((locals.var_pparam_b4soiweffcv_dn7 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvb) + (assign25900_e23287 * locals.var_pparam_b4soileffcvb_dn7)));
        locals.var_coxwlb_dn8 = (assign25900_e23282 * ((((locals.var_pparam_b4soiweffcv_dn8 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvb) + (assign25900_e23287 * locals.var_pparam_b4soileffcvb_dn8)));
        locals.var_coxwlb_dn9 = (assign25900_e23282 * ((((locals.var_pparam_b4soiweffcv_dn9 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvb) + (assign25900_e23287 * locals.var_pparam_b4soileffcvb_dn9)));
        locals.var_coxwlb_dn10 = (assign25900_e23282 * ((((locals.var_pparam_b4soiweffcv_dn10 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvb) + (assign25900_e23287 * locals.var_pparam_b4soileffcvb_dn10)));
        locals.var_coxwlb_dn11 = (assign25900_e23282 * ((((locals.var_pparam_b4soiweffcv_dn11 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvb) + (assign25900_e23287 * locals.var_pparam_b4soileffcvb_dn11)));
        locals.var_coxwlb_dn12 = (assign25900_e23282 * ((((locals.var_pparam_b4soiweffcv_dn12 / p.p23) * p.p3) * locals.var_pparam_b4soileffcvb) + (assign25900_e23287 * locals.var_pparam_b4soileffcvb_dn12)));

        let assign25910_e23295: f64 = (locals.var_b4soicox * p.p27);
        locals.var_coxwl2 = assign25910_e23295;
        locals.var_coxwl2_dn3 = 0.0;
        locals.var_coxwl2_dn4 = 0.0;
        locals.var_coxwl2_dn5 = 0.0;
        locals.var_coxwl2_dn6 = 0.0;
        locals.var_coxwl2_dn7 = 0.0;
        locals.var_coxwl2_dn8 = 0.0;
        locals.var_coxwl2_dn9 = 0.0;
        locals.var_coxwl2_dn10 = 0.0;
        locals.var_coxwl2_dn11 = 0.0;
        locals.var_coxwl2_dn12 = 0.0;

        let assign25920_e23298: f64 = (p.p361 * locals.var_b4soicox);
        let assign25920_e23300: f64 = (assign25920_e23298 * p.p27);
        locals.var_coxwlb2 = assign25920_e23300;
        locals.var_coxwlb2_dn3 = 0.0;
        locals.var_coxwlb2_dn4 = 0.0;
        locals.var_coxwlb2_dn5 = 0.0;
        locals.var_coxwlb2_dn6 = 0.0;
        locals.var_coxwlb2_dn7 = 0.0;
        locals.var_coxwlb2_dn8 = 0.0;
        locals.var_coxwlb2_dn9 = 0.0;
        locals.var_coxwlb2_dn10 = 0.0;
        locals.var_coxwlb2_dn11 = 0.0;
        locals.var_coxwlb2_dn12 = 0.0;

        let assign25930_e23303: f64 = (locals.var_vgs_eff__blk790 - locals.var_vth_cv);
        locals.var_vgst__blk795 = assign25930_e23303;
        locals.var_vgst__blk795_dn3 = (locals.var_vgs_eff__blk790_dn3 - locals.var_vth_cv_dn3);
        locals.var_vgst__blk795_dn4 = (locals.var_vgs_eff__blk790_dn4 - locals.var_vth_cv_dn4);
        locals.var_vgst__blk795_dn5 = (locals.var_vgs_eff__blk790_dn5 - locals.var_vth_cv_dn5);
        locals.var_vgst__blk795_dn6 = (locals.var_vgs_eff__blk790_dn6 - locals.var_vth_cv_dn6);
        locals.var_vgst__blk795_dn7 = (locals.var_vgs_eff__blk790_dn7 - locals.var_vth_cv_dn7);
        locals.var_vgst__blk795_dn8 = (locals.var_vgs_eff__blk790_dn8 - locals.var_vth_cv_dn8);
        locals.var_vgst__blk795_dn9 = (locals.var_vgs_eff__blk790_dn9 - locals.var_vth_cv_dn9);
        locals.var_vgst__blk795_dn10 = (locals.var_vgs_eff__blk790_dn10 - locals.var_vth_cv_dn10);
        locals.var_vgst__blk795_dn11 = (locals.var_vgs_eff__blk790_dn11 - locals.var_vth_cv_dn11);
        locals.var_vgst__blk795_dn12 = (locals.var_vgs_eff__blk790_dn12 - locals.var_vth_cv_dn12);

        let assign25940_e23306: f64 = (locals.var_n_cv * locals.var_vtm);
        locals.var_t10__blk818 = assign25940_e23306;
        locals.var_t10__blk818_dn3 = (locals.var_n_cv_dn3 * locals.var_vtm);
        locals.var_t10__blk818_dn4 = ((locals.var_n_cv_dn4 * locals.var_vtm) + (locals.var_n_cv * locals.var_vtm_dn4));
        locals.var_t10__blk818_dn5 = ((locals.var_n_cv_dn5 * locals.var_vtm) + (locals.var_n_cv * locals.var_vtm_dn5));
        locals.var_t10__blk818_dn6 = ((locals.var_n_cv_dn6 * locals.var_vtm) + (locals.var_n_cv * locals.var_vtm_dn6));
        locals.var_t10__blk818_dn7 = (locals.var_n_cv_dn7 * locals.var_vtm);
        locals.var_t10__blk818_dn8 = (locals.var_n_cv_dn8 * locals.var_vtm);
        locals.var_t10__blk818_dn9 = (locals.var_n_cv_dn9 * locals.var_vtm);
        locals.var_t10__blk818_dn10 = (locals.var_n_cv_dn10 * locals.var_vtm);
        locals.var_t10__blk818_dn11 = (locals.var_n_cv_dn11 * locals.var_vtm);
        locals.var_t10__blk818_dn12 = (locals.var_n_cv_dn12 * locals.var_vtm);

        let assign25950_e23309: f64 = (locals.var_pparam_b4soimstar * locals.var_vgst__blk795);
        let assign25950_e23311: f64 = (assign25950_e23309 / locals.var_t10__blk818);
        locals.var_vgstnvt__blk774 = assign25950_e23311;
        locals.var_vgstnvt__blk774_dn3 = (((((locals.var_pparam_b4soimstar_dn3 * locals.var_vgst__blk795) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk795_dn3)) * locals.var_t10__blk818) - (assign25950_e23309 * locals.var_t10__blk818_dn3)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_vgstnvt__blk774_dn4 = (((((locals.var_pparam_b4soimstar_dn4 * locals.var_vgst__blk795) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk795_dn4)) * locals.var_t10__blk818) - (assign25950_e23309 * locals.var_t10__blk818_dn4)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_vgstnvt__blk774_dn5 = (((((locals.var_pparam_b4soimstar_dn5 * locals.var_vgst__blk795) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk795_dn5)) * locals.var_t10__blk818) - (assign25950_e23309 * locals.var_t10__blk818_dn5)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_vgstnvt__blk774_dn6 = (((((locals.var_pparam_b4soimstar_dn6 * locals.var_vgst__blk795) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk795_dn6)) * locals.var_t10__blk818) - (assign25950_e23309 * locals.var_t10__blk818_dn6)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_vgstnvt__blk774_dn7 = (((((locals.var_pparam_b4soimstar_dn7 * locals.var_vgst__blk795) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk795_dn7)) * locals.var_t10__blk818) - (assign25950_e23309 * locals.var_t10__blk818_dn7)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_vgstnvt__blk774_dn8 = (((((locals.var_pparam_b4soimstar_dn8 * locals.var_vgst__blk795) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk795_dn8)) * locals.var_t10__blk818) - (assign25950_e23309 * locals.var_t10__blk818_dn8)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_vgstnvt__blk774_dn9 = (((((locals.var_pparam_b4soimstar_dn9 * locals.var_vgst__blk795) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk795_dn9)) * locals.var_t10__blk818) - (assign25950_e23309 * locals.var_t10__blk818_dn9)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_vgstnvt__blk774_dn10 = (((((locals.var_pparam_b4soimstar_dn10 * locals.var_vgst__blk795) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk795_dn10)) * locals.var_t10__blk818) - (assign25950_e23309 * locals.var_t10__blk818_dn10)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_vgstnvt__blk774_dn11 = (((((locals.var_pparam_b4soimstar_dn11 * locals.var_vgst__blk795) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk795_dn11)) * locals.var_t10__blk818) - (assign25950_e23309 * locals.var_t10__blk818_dn11)) / (locals.var_t10__blk818 * locals.var_t10__blk818));
        locals.var_vgstnvt__blk774_dn12 = (((((locals.var_pparam_b4soimstar_dn12 * locals.var_vgst__blk795) + (locals.var_pparam_b4soimstar * locals.var_vgst__blk795_dn12)) * locals.var_t10__blk818) - (assign25950_e23309 * locals.var_t10__blk818_dn12)) / (locals.var_t10__blk818 * locals.var_t10__blk818));

        let assign25960_e23314: f64 = (locals.var_n_cv * locals.var_pparam_b4soinoff);
        let assign25960_e23316: f64 = (assign25960_e23314 * locals.var_vtm);
        locals.var_noff = assign25960_e23316;
        locals.var_noff_dn3 = (((locals.var_n_cv_dn3 * locals.var_pparam_b4soinoff) + (locals.var_n_cv * locals.var_pparam_b4soinoff_dn3)) * locals.var_vtm);
        locals.var_noff_dn4 = ((((locals.var_n_cv_dn4 * locals.var_pparam_b4soinoff) + (locals.var_n_cv * locals.var_pparam_b4soinoff_dn4)) * locals.var_vtm) + (assign25960_e23314 * locals.var_vtm_dn4));
        locals.var_noff_dn5 = ((((locals.var_n_cv_dn5 * locals.var_pparam_b4soinoff) + (locals.var_n_cv * locals.var_pparam_b4soinoff_dn5)) * locals.var_vtm) + (assign25960_e23314 * locals.var_vtm_dn5));
        locals.var_noff_dn6 = ((((locals.var_n_cv_dn6 * locals.var_pparam_b4soinoff) + (locals.var_n_cv * locals.var_pparam_b4soinoff_dn6)) * locals.var_vtm) + (assign25960_e23314 * locals.var_vtm_dn6));
        locals.var_noff_dn7 = (((locals.var_n_cv_dn7 * locals.var_pparam_b4soinoff) + (locals.var_n_cv * locals.var_pparam_b4soinoff_dn7)) * locals.var_vtm);
        locals.var_noff_dn8 = (((locals.var_n_cv_dn8 * locals.var_pparam_b4soinoff) + (locals.var_n_cv * locals.var_pparam_b4soinoff_dn8)) * locals.var_vtm);
        locals.var_noff_dn9 = (((locals.var_n_cv_dn9 * locals.var_pparam_b4soinoff) + (locals.var_n_cv * locals.var_pparam_b4soinoff_dn9)) * locals.var_vtm);
        locals.var_noff_dn10 = (((locals.var_n_cv_dn10 * locals.var_pparam_b4soinoff) + (locals.var_n_cv * locals.var_pparam_b4soinoff_dn10)) * locals.var_vtm);
        locals.var_noff_dn11 = (((locals.var_n_cv_dn11 * locals.var_pparam_b4soinoff) + (locals.var_n_cv * locals.var_pparam_b4soinoff_dn11)) * locals.var_vtm);
        locals.var_noff_dn12 = (((locals.var_n_cv_dn12 * locals.var_pparam_b4soinoff) + (locals.var_n_cv * locals.var_pparam_b4soinoff_dn12)) * locals.var_vtm);

        let assign25970_e23319: f64 = (locals.var_n_cv * locals.var_pparam_b4soinoff2);
        let assign25970_e23321: f64 = (assign25970_e23319 * locals.var_vtm);
        locals.var_noff2 = assign25970_e23321;
        locals.var_noff2_dn3 = (((locals.var_n_cv_dn3 * locals.var_pparam_b4soinoff2) + (locals.var_n_cv * locals.var_pparam_b4soinoff2_dn3)) * locals.var_vtm);
        locals.var_noff2_dn4 = ((((locals.var_n_cv_dn4 * locals.var_pparam_b4soinoff2) + (locals.var_n_cv * locals.var_pparam_b4soinoff2_dn4)) * locals.var_vtm) + (assign25970_e23319 * locals.var_vtm_dn4));
        locals.var_noff2_dn5 = ((((locals.var_n_cv_dn5 * locals.var_pparam_b4soinoff2) + (locals.var_n_cv * locals.var_pparam_b4soinoff2_dn5)) * locals.var_vtm) + (assign25970_e23319 * locals.var_vtm_dn5));
        locals.var_noff2_dn6 = ((((locals.var_n_cv_dn6 * locals.var_pparam_b4soinoff2) + (locals.var_n_cv * locals.var_pparam_b4soinoff2_dn6)) * locals.var_vtm) + (assign25970_e23319 * locals.var_vtm_dn6));
        locals.var_noff2_dn7 = (((locals.var_n_cv_dn7 * locals.var_pparam_b4soinoff2) + (locals.var_n_cv * locals.var_pparam_b4soinoff2_dn7)) * locals.var_vtm);
        locals.var_noff2_dn8 = (((locals.var_n_cv_dn8 * locals.var_pparam_b4soinoff2) + (locals.var_n_cv * locals.var_pparam_b4soinoff2_dn8)) * locals.var_vtm);
        locals.var_noff2_dn9 = (((locals.var_n_cv_dn9 * locals.var_pparam_b4soinoff2) + (locals.var_n_cv * locals.var_pparam_b4soinoff2_dn9)) * locals.var_vtm);
        locals.var_noff2_dn10 = (((locals.var_n_cv_dn10 * locals.var_pparam_b4soinoff2) + (locals.var_n_cv * locals.var_pparam_b4soinoff2_dn10)) * locals.var_vtm);
        locals.var_noff2_dn11 = (((locals.var_n_cv_dn11 * locals.var_pparam_b4soinoff2) + (locals.var_n_cv * locals.var_pparam_b4soinoff2_dn11)) * locals.var_vtm);
        locals.var_noff2_dn12 = (((locals.var_n_cv_dn12 * locals.var_pparam_b4soinoff2) + (locals.var_n_cv * locals.var_pparam_b4soinoff2_dn12)) * locals.var_vtm);

        let assign25980_e23324: f64 = if p.p42 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1356 = assign25980_e23324;

        let assign25990_e23327: f64 = (-100.0);
        let assign25990_e23332: f64 = if ((locals.var_vgstnvt__blk774 > assign25990_e23327) && (locals.var_vgstnvt__blk774 < 100.0)) { 1.0 } else { 0.0 };
        locals.var_guard1357 = assign25990_e23332;

        let (assign26000_e23342, assign26000_e23342_d_n3, assign26000_e23342_d_n4, assign26000_e23342_d_n5, assign26000_e23342_d_n6, assign26000_e23342_d_n7, assign26000_e23342_d_n8, assign26000_e23342_d_n9, assign26000_e23342_d_n10, assign26000_e23342_d_n11, assign26000_e23342_d_n12,) = {
    if ((locals.var_guard1356 != 0.0) && (locals.var_guard1357 != 0.0)) {
        let assign26000_e23337: f64 = (locals.var_vgstnvt__blk774).exp();
        let assign26000_e23339: f64 = (locals.var_vgstnvt__blk774).exp();
        let assign26000_e23340: f64 = (assign26000_e23337 * assign26000_e23339);
        (assign26000_e23340, (((assign26000_e23337 * locals.var_vgstnvt__blk774_dn3) * assign26000_e23339) + (assign26000_e23337 * (assign26000_e23339 * locals.var_vgstnvt__blk774_dn3))), (((assign26000_e23337 * locals.var_vgstnvt__blk774_dn4) * assign26000_e23339) + (assign26000_e23337 * (assign26000_e23339 * locals.var_vgstnvt__blk774_dn4))), (((assign26000_e23337 * locals.var_vgstnvt__blk774_dn5) * assign26000_e23339) + (assign26000_e23337 * (assign26000_e23339 * locals.var_vgstnvt__blk774_dn5))), (((assign26000_e23337 * locals.var_vgstnvt__blk774_dn6) * assign26000_e23339) + (assign26000_e23337 * (assign26000_e23339 * locals.var_vgstnvt__blk774_dn6))), (((assign26000_e23337 * locals.var_vgstnvt__blk774_dn7) * assign26000_e23339) + (assign26000_e23337 * (assign26000_e23339 * locals.var_vgstnvt__blk774_dn7))), (((assign26000_e23337 * locals.var_vgstnvt__blk774_dn8) * assign26000_e23339) + (assign26000_e23337 * (assign26000_e23339 * locals.var_vgstnvt__blk774_dn8))), (((assign26000_e23337 * locals.var_vgstnvt__blk774_dn9) * assign26000_e23339) + (assign26000_e23337 * (assign26000_e23339 * locals.var_vgstnvt__blk774_dn9))), (((assign26000_e23337 * locals.var_vgstnvt__blk774_dn10) * assign26000_e23339) + (assign26000_e23337 * (assign26000_e23339 * locals.var_vgstnvt__blk774_dn10))), (((assign26000_e23337 * locals.var_vgstnvt__blk774_dn11) * assign26000_e23339) + (assign26000_e23337 * (assign26000_e23339 * locals.var_vgstnvt__blk774_dn11))), (((assign26000_e23337 * locals.var_vgstnvt__blk774_dn12) * assign26000_e23339) + (assign26000_e23337 * (assign26000_e23339 * locals.var_vgstnvt__blk774_dn12))),)
    } else {
        (locals.var_expvgst__blk775, locals.var_expvgst__blk775_dn3, locals.var_expvgst__blk775_dn4, locals.var_expvgst__blk775_dn5, locals.var_expvgst__blk775_dn6, locals.var_expvgst__blk775_dn7, locals.var_expvgst__blk775_dn8, locals.var_expvgst__blk775_dn9, locals.var_expvgst__blk775_dn10, locals.var_expvgst__blk775_dn11, locals.var_expvgst__blk775_dn12,)
    }
};
        locals.var_expvgst__blk775 = assign26000_e23342;
        locals.var_expvgst__blk775_dn3 = assign26000_e23342_d_n3;
        locals.var_expvgst__blk775_dn4 = assign26000_e23342_d_n4;
        locals.var_expvgst__blk775_dn5 = assign26000_e23342_d_n5;
        locals.var_expvgst__blk775_dn6 = assign26000_e23342_d_n6;
        locals.var_expvgst__blk775_dn7 = assign26000_e23342_d_n7;
        locals.var_expvgst__blk775_dn8 = assign26000_e23342_d_n8;
        locals.var_expvgst__blk775_dn9 = assign26000_e23342_d_n9;
        locals.var_expvgst__blk775_dn10 = assign26000_e23342_d_n10;
        locals.var_expvgst__blk775_dn11 = assign26000_e23342_d_n11;
        locals.var_expvgst__blk775_dn12 = assign26000_e23342_d_n12;

        let (assign26010_e23354, assign26010_e23354_d_n3, assign26010_e23354_d_n4, assign26010_e23354_d_n5, assign26010_e23354_d_n6, assign26010_e23354_d_n7, assign26010_e23354_d_n8, assign26010_e23354_d_n9, assign26010_e23354_d_n10, assign26010_e23354_d_n11, assign26010_e23354_d_n12,) = {
    if ((locals.var_guard1356 != 0.0) && (locals.var_guard1357 != 0.0)) {
        let assign26010_e23349: f64 = (locals.var_pparam_b4soidelvt / locals.var_noff);
        let assign26010_e23350: f64 = (-assign26010_e23349);
        let assign26010_e23351: f64 = (assign26010_e23350).exp();
        let assign26010_e23352: f64 = (locals.var_expvgst__blk775 * assign26010_e23351);
        (assign26010_e23352, ((locals.var_expvgst__blk775_dn3 * assign26010_e23351) + (locals.var_expvgst__blk775 * (assign26010_e23351 * (-(((locals.var_pparam_b4soidelvt_dn3 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn3)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk775_dn4 * assign26010_e23351) + (locals.var_expvgst__blk775 * (assign26010_e23351 * (-(((locals.var_pparam_b4soidelvt_dn4 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn4)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk775_dn5 * assign26010_e23351) + (locals.var_expvgst__blk775 * (assign26010_e23351 * (-(((locals.var_pparam_b4soidelvt_dn5 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn5)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk775_dn6 * assign26010_e23351) + (locals.var_expvgst__blk775 * (assign26010_e23351 * (-(((locals.var_pparam_b4soidelvt_dn6 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn6)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk775_dn7 * assign26010_e23351) + (locals.var_expvgst__blk775 * (assign26010_e23351 * (-(((locals.var_pparam_b4soidelvt_dn7 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn7)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk775_dn8 * assign26010_e23351) + (locals.var_expvgst__blk775 * (assign26010_e23351 * (-(((locals.var_pparam_b4soidelvt_dn8 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn8)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk775_dn9 * assign26010_e23351) + (locals.var_expvgst__blk775 * (assign26010_e23351 * (-(((locals.var_pparam_b4soidelvt_dn9 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn9)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk775_dn10 * assign26010_e23351) + (locals.var_expvgst__blk775 * (assign26010_e23351 * (-(((locals.var_pparam_b4soidelvt_dn10 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn10)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk775_dn11 * assign26010_e23351) + (locals.var_expvgst__blk775 * (assign26010_e23351 * (-(((locals.var_pparam_b4soidelvt_dn11 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn11)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk775_dn12 * assign26010_e23351) + (locals.var_expvgst__blk775 * (assign26010_e23351 * (-(((locals.var_pparam_b4soidelvt_dn12 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn12)) / (locals.var_noff * locals.var_noff)))))),)
    } else {
        (locals.var_expvgst__blk775, locals.var_expvgst__blk775_dn3, locals.var_expvgst__blk775_dn4, locals.var_expvgst__blk775_dn5, locals.var_expvgst__blk775_dn6, locals.var_expvgst__blk775_dn7, locals.var_expvgst__blk775_dn8, locals.var_expvgst__blk775_dn9, locals.var_expvgst__blk775_dn10, locals.var_expvgst__blk775_dn11, locals.var_expvgst__blk775_dn12,)
    }
};
        locals.var_expvgst__blk775 = assign26010_e23354;
        locals.var_expvgst__blk775_dn3 = assign26010_e23354_d_n3;
        locals.var_expvgst__blk775_dn4 = assign26010_e23354_d_n4;
        locals.var_expvgst__blk775_dn5 = assign26010_e23354_d_n5;
        locals.var_expvgst__blk775_dn6 = assign26010_e23354_d_n6;
        locals.var_expvgst__blk775_dn7 = assign26010_e23354_d_n7;
        locals.var_expvgst__blk775_dn8 = assign26010_e23354_d_n8;
        locals.var_expvgst__blk775_dn9 = assign26010_e23354_d_n9;
        locals.var_expvgst__blk775_dn10 = assign26010_e23354_d_n10;
        locals.var_expvgst__blk775_dn11 = assign26010_e23354_d_n11;
        locals.var_expvgst__blk775_dn12 = assign26010_e23354_d_n12;

        let (assign26020_e23373, assign26020_e23373_d_n3, assign26020_e23373_d_n4, assign26020_e23373_d_n5, assign26020_e23373_d_n6, assign26020_e23373_d_n7, assign26020_e23373_d_n8, assign26020_e23373_d_n9, assign26020_e23373_d_n10, assign26020_e23373_d_n11, assign26020_e23373_d_n12,) = {
    if ((locals.var_guard1356 != 0.0) && (locals.var_guard1357 != 0.0)) {
        let assign26020_e23361: f64 = (1.0 + locals.var_expvgst__blk775);
        let (assign26020_e23370, assign26020_e23370_d_n3, assign26020_e23370_d_n4, assign26020_e23370_d_n5, assign26020_e23370_d_n6, assign26020_e23370_d_n7, assign26020_e23370_d_n8, assign26020_e23370_d_n9, assign26020_e23370_d_n10, assign26020_e23370_d_n11, assign26020_e23370_d_n12,) = {
            if (assign26020_e23361 > 1e-38) {
                let assign26020_e23366: f64 = (1.0 + locals.var_expvgst__blk775);
                let assign26020_e23367: f64 = (assign26020_e23366).ln();
                (assign26020_e23367, (locals.var_expvgst__blk775_dn3 / assign26020_e23366), (locals.var_expvgst__blk775_dn4 / assign26020_e23366), (locals.var_expvgst__blk775_dn5 / assign26020_e23366), (locals.var_expvgst__blk775_dn6 / assign26020_e23366), (locals.var_expvgst__blk775_dn7 / assign26020_e23366), (locals.var_expvgst__blk775_dn8 / assign26020_e23366), (locals.var_expvgst__blk775_dn9 / assign26020_e23366), (locals.var_expvgst__blk775_dn10 / assign26020_e23366), (locals.var_expvgst__blk775_dn11 / assign26020_e23366), (locals.var_expvgst__blk775_dn12 / assign26020_e23366),)
            } else {
                let assign26020_e23369: f64 = (-87.49823353377374);
                (assign26020_e23369, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign26020_e23371: f64 = (locals.var_noff * assign26020_e23370);
        (assign26020_e23371, ((locals.var_noff_dn3 * assign26020_e23370) + (locals.var_noff * assign26020_e23370_d_n3)), ((locals.var_noff_dn4 * assign26020_e23370) + (locals.var_noff * assign26020_e23370_d_n4)), ((locals.var_noff_dn5 * assign26020_e23370) + (locals.var_noff * assign26020_e23370_d_n5)), ((locals.var_noff_dn6 * assign26020_e23370) + (locals.var_noff * assign26020_e23370_d_n6)), ((locals.var_noff_dn7 * assign26020_e23370) + (locals.var_noff * assign26020_e23370_d_n7)), ((locals.var_noff_dn8 * assign26020_e23370) + (locals.var_noff * assign26020_e23370_d_n8)), ((locals.var_noff_dn9 * assign26020_e23370) + (locals.var_noff * assign26020_e23370_d_n9)), ((locals.var_noff_dn10 * assign26020_e23370) + (locals.var_noff * assign26020_e23370_d_n10)), ((locals.var_noff_dn11 * assign26020_e23370) + (locals.var_noff * assign26020_e23370_d_n11)), ((locals.var_noff_dn12 * assign26020_e23370) + (locals.var_noff * assign26020_e23370_d_n12)),)
    } else {
        (locals.var_vgsteff__blk840, locals.var_vgsteff__blk840_dn3, locals.var_vgsteff__blk840_dn4, locals.var_vgsteff__blk840_dn5, locals.var_vgsteff__blk840_dn6, locals.var_vgsteff__blk840_dn7, locals.var_vgsteff__blk840_dn8, locals.var_vgsteff__blk840_dn9, locals.var_vgsteff__blk840_dn10, locals.var_vgsteff__blk840_dn11, locals.var_vgsteff__blk840_dn12,)
    }
};
        locals.var_vgsteff__blk840 = assign26020_e23373;
        locals.var_vgsteff__blk840_dn3 = assign26020_e23373_d_n3;
        locals.var_vgsteff__blk840_dn4 = assign26020_e23373_d_n4;
        locals.var_vgsteff__blk840_dn5 = assign26020_e23373_d_n5;
        locals.var_vgsteff__blk840_dn6 = assign26020_e23373_d_n6;
        locals.var_vgsteff__blk840_dn7 = assign26020_e23373_d_n7;
        locals.var_vgsteff__blk840_dn8 = assign26020_e23373_d_n8;
        locals.var_vgsteff__blk840_dn9 = assign26020_e23373_d_n9;
        locals.var_vgsteff__blk840_dn10 = assign26020_e23373_d_n10;
        locals.var_vgsteff__blk840_dn11 = assign26020_e23373_d_n11;
        locals.var_vgsteff__blk840_dn12 = assign26020_e23373_d_n12;

        let assign26030_e23376: f64 = if p.p27 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1358 = assign26030_e23376;

        let (assign26040_e23394, assign26040_e23394_d_n3, assign26040_e23394_d_n4, assign26040_e23394_d_n5, assign26040_e23394_d_n6, assign26040_e23394_d_n7, assign26040_e23394_d_n8, assign26040_e23394_d_n9, assign26040_e23394_d_n10, assign26040_e23394_d_n11, assign26040_e23394_d_n12,) = {
    if (((locals.var_guard1356 != 0.0) && (locals.var_guard1357 != 0.0)) && (locals.var_guard1358 != 0.0)) {
        let assign26040_e23384: f64 = (-p.p1033);
        let assign26040_e23386: f64 = (assign26040_e23384 / locals.var_noff2);
        let assign26040_e23389: f64 = (locals.var_vtm * locals.var_vtm);
        let assign26040_e23390: f64 = (assign26040_e23386 / assign26040_e23389);
        let assign26040_e23391: f64 = (assign26040_e23390).exp();
        let assign26040_e23392: f64 = (locals.var_expvgst__blk775 * assign26040_e23391);
        (assign26040_e23392, ((locals.var_expvgst__blk775_dn3 * assign26040_e23391) + (locals.var_expvgst__blk775 * (assign26040_e23391 * ((-((assign26040_e23384 * locals.var_noff2_dn3) / (locals.var_noff2 * locals.var_noff2))) / assign26040_e23389)))), ((locals.var_expvgst__blk775_dn4 * assign26040_e23391) + (locals.var_expvgst__blk775 * (assign26040_e23391 * ((((-((assign26040_e23384 * locals.var_noff2_dn4) / (locals.var_noff2 * locals.var_noff2))) * assign26040_e23389) - (assign26040_e23386 * ((locals.var_vtm_dn4 * locals.var_vtm) + (locals.var_vtm * locals.var_vtm_dn4)))) / (assign26040_e23389 * assign26040_e23389))))), ((locals.var_expvgst__blk775_dn5 * assign26040_e23391) + (locals.var_expvgst__blk775 * (assign26040_e23391 * ((((-((assign26040_e23384 * locals.var_noff2_dn5) / (locals.var_noff2 * locals.var_noff2))) * assign26040_e23389) - (assign26040_e23386 * ((locals.var_vtm_dn5 * locals.var_vtm) + (locals.var_vtm * locals.var_vtm_dn5)))) / (assign26040_e23389 * assign26040_e23389))))), ((locals.var_expvgst__blk775_dn6 * assign26040_e23391) + (locals.var_expvgst__blk775 * (assign26040_e23391 * ((((-((assign26040_e23384 * locals.var_noff2_dn6) / (locals.var_noff2 * locals.var_noff2))) * assign26040_e23389) - (assign26040_e23386 * ((locals.var_vtm_dn6 * locals.var_vtm) + (locals.var_vtm * locals.var_vtm_dn6)))) / (assign26040_e23389 * assign26040_e23389))))), ((locals.var_expvgst__blk775_dn7 * assign26040_e23391) + (locals.var_expvgst__blk775 * (assign26040_e23391 * ((-((assign26040_e23384 * locals.var_noff2_dn7) / (locals.var_noff2 * locals.var_noff2))) / assign26040_e23389)))), ((locals.var_expvgst__blk775_dn8 * assign26040_e23391) + (locals.var_expvgst__blk775 * (assign26040_e23391 * ((-((assign26040_e23384 * locals.var_noff2_dn8) / (locals.var_noff2 * locals.var_noff2))) / assign26040_e23389)))), ((locals.var_expvgst__blk775_dn9 * assign26040_e23391) + (locals.var_expvgst__blk775 * (assign26040_e23391 * ((-((assign26040_e23384 * locals.var_noff2_dn9) / (locals.var_noff2 * locals.var_noff2))) / assign26040_e23389)))), ((locals.var_expvgst__blk775_dn10 * assign26040_e23391) + (locals.var_expvgst__blk775 * (assign26040_e23391 * ((-((assign26040_e23384 * locals.var_noff2_dn10) / (locals.var_noff2 * locals.var_noff2))) / assign26040_e23389)))), ((locals.var_expvgst__blk775_dn11 * assign26040_e23391) + (locals.var_expvgst__blk775 * (assign26040_e23391 * ((-((assign26040_e23384 * locals.var_noff2_dn11) / (locals.var_noff2 * locals.var_noff2))) / assign26040_e23389)))), ((locals.var_expvgst__blk775_dn12 * assign26040_e23391) + (locals.var_expvgst__blk775 * (assign26040_e23391 * ((-((assign26040_e23384 * locals.var_noff2_dn12) / (locals.var_noff2 * locals.var_noff2))) / assign26040_e23389)))),)
    } else {
        (locals.var_expvgst2, locals.var_expvgst2_dn3, locals.var_expvgst2_dn4, locals.var_expvgst2_dn5, locals.var_expvgst2_dn6, locals.var_expvgst2_dn7, locals.var_expvgst2_dn8, locals.var_expvgst2_dn9, locals.var_expvgst2_dn10, locals.var_expvgst2_dn11, locals.var_expvgst2_dn12,)
    }
};
        locals.var_expvgst2 = assign26040_e23394;
        locals.var_expvgst2_dn3 = assign26040_e23394_d_n3;
        locals.var_expvgst2_dn4 = assign26040_e23394_d_n4;
        locals.var_expvgst2_dn5 = assign26040_e23394_d_n5;
        locals.var_expvgst2_dn6 = assign26040_e23394_d_n6;
        locals.var_expvgst2_dn7 = assign26040_e23394_d_n7;
        locals.var_expvgst2_dn8 = assign26040_e23394_d_n8;
        locals.var_expvgst2_dn9 = assign26040_e23394_d_n9;
        locals.var_expvgst2_dn10 = assign26040_e23394_d_n10;
        locals.var_expvgst2_dn11 = assign26040_e23394_d_n11;
        locals.var_expvgst2_dn12 = assign26040_e23394_d_n12;

        let (assign26050_e23415, assign26050_e23415_d_n3, assign26050_e23415_d_n4, assign26050_e23415_d_n5, assign26050_e23415_d_n6, assign26050_e23415_d_n7, assign26050_e23415_d_n8, assign26050_e23415_d_n9, assign26050_e23415_d_n10, assign26050_e23415_d_n11, assign26050_e23415_d_n12,) = {
    if (((locals.var_guard1356 != 0.0) && (locals.var_guard1357 != 0.0)) && (locals.var_guard1358 != 0.0)) {
        let assign26050_e23403: f64 = (1.0 + locals.var_expvgst2);
        let (assign26050_e23412, assign26050_e23412_d_n3, assign26050_e23412_d_n4, assign26050_e23412_d_n5, assign26050_e23412_d_n6, assign26050_e23412_d_n7, assign26050_e23412_d_n8, assign26050_e23412_d_n9, assign26050_e23412_d_n10, assign26050_e23412_d_n11, assign26050_e23412_d_n12,) = {
            if (assign26050_e23403 > 1e-38) {
                let assign26050_e23408: f64 = (1.0 + locals.var_expvgst2);
                let assign26050_e23409: f64 = (assign26050_e23408).ln();
                (assign26050_e23409, (locals.var_expvgst2_dn3 / assign26050_e23408), (locals.var_expvgst2_dn4 / assign26050_e23408), (locals.var_expvgst2_dn5 / assign26050_e23408), (locals.var_expvgst2_dn6 / assign26050_e23408), (locals.var_expvgst2_dn7 / assign26050_e23408), (locals.var_expvgst2_dn8 / assign26050_e23408), (locals.var_expvgst2_dn9 / assign26050_e23408), (locals.var_expvgst2_dn10 / assign26050_e23408), (locals.var_expvgst2_dn11 / assign26050_e23408), (locals.var_expvgst2_dn12 / assign26050_e23408),)
            } else {
                let assign26050_e23411: f64 = (-87.49823353377374);
                (assign26050_e23411, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign26050_e23413: f64 = (locals.var_noff2 * assign26050_e23412);
        (assign26050_e23413, ((locals.var_noff2_dn3 * assign26050_e23412) + (locals.var_noff2 * assign26050_e23412_d_n3)), ((locals.var_noff2_dn4 * assign26050_e23412) + (locals.var_noff2 * assign26050_e23412_d_n4)), ((locals.var_noff2_dn5 * assign26050_e23412) + (locals.var_noff2 * assign26050_e23412_d_n5)), ((locals.var_noff2_dn6 * assign26050_e23412) + (locals.var_noff2 * assign26050_e23412_d_n6)), ((locals.var_noff2_dn7 * assign26050_e23412) + (locals.var_noff2 * assign26050_e23412_d_n7)), ((locals.var_noff2_dn8 * assign26050_e23412) + (locals.var_noff2 * assign26050_e23412_d_n8)), ((locals.var_noff2_dn9 * assign26050_e23412) + (locals.var_noff2 * assign26050_e23412_d_n9)), ((locals.var_noff2_dn10 * assign26050_e23412) + (locals.var_noff2 * assign26050_e23412_d_n10)), ((locals.var_noff2_dn11 * assign26050_e23412) + (locals.var_noff2 * assign26050_e23412_d_n11)), ((locals.var_noff2_dn12 * assign26050_e23412) + (locals.var_noff2 * assign26050_e23412_d_n12)),)
    } else {
        (locals.var_vgsteff2, locals.var_vgsteff2_dn3, locals.var_vgsteff2_dn4, locals.var_vgsteff2_dn5, locals.var_vgsteff2_dn6, locals.var_vgsteff2_dn7, locals.var_vgsteff2_dn8, locals.var_vgsteff2_dn9, locals.var_vgsteff2_dn10, locals.var_vgsteff2_dn11, locals.var_vgsteff2_dn12,)
    }
};
        locals.var_vgsteff2 = assign26050_e23415;
        locals.var_vgsteff2_dn3 = assign26050_e23415_d_n3;
        locals.var_vgsteff2_dn4 = assign26050_e23415_d_n4;
        locals.var_vgsteff2_dn5 = assign26050_e23415_d_n5;
        locals.var_vgsteff2_dn6 = assign26050_e23415_d_n6;
        locals.var_vgsteff2_dn7 = assign26050_e23415_d_n7;
        locals.var_vgsteff2_dn8 = assign26050_e23415_d_n8;
        locals.var_vgsteff2_dn9 = assign26050_e23415_d_n9;
        locals.var_vgsteff2_dn10 = assign26050_e23415_d_n10;
        locals.var_vgsteff2_dn11 = assign26050_e23415_d_n11;
        locals.var_vgsteff2_dn12 = assign26050_e23415_d_n12;

        let assign26060_e23418: f64 = if p.p42 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1359 = assign26060_e23418;

        let assign26070_e23421: f64 = (-100.0);
        let assign26070_e23426: f64 = if ((locals.var_vgstnvt__blk774 > assign26070_e23421) && (locals.var_vgstnvt__blk774 < 100.0)) { 1.0 } else { 0.0 };
        locals.var_guard1360 = assign26070_e23426;

    }

    pub(super) fn stamp_transient_block_78(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26080_e23440, assign26080_e23440_d_n3, assign26080_e23440_d_n4, assign26080_e23440_d_n5, assign26080_e23440_d_n6, assign26080_e23440_d_n7, assign26080_e23440_d_n8, assign26080_e23440_d_n9, assign26080_e23440_d_n10, assign26080_e23440_d_n11, assign26080_e23440_d_n12,) = {
    if (((locals.var_guard1356 == 0.0) && (locals.var_guard1359 != 0.0)) && (locals.var_guard1360 != 0.0)) {
        let assign26080_e23436: f64 = (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff);
        let assign26080_e23437: f64 = (locals.var_vgstnvt__blk774 / assign26080_e23436);
        let assign26080_e23438: f64 = (assign26080_e23437).exp();
        (assign26080_e23438, (assign26080_e23438 * (((locals.var_vgstnvt__blk774_dn3 * assign26080_e23436) - (locals.var_vgstnvt__blk774 * ((locals.var_pparam_b4soimstar_dn3 * locals.var_pparam_b4soinoff) + (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff_dn3)))) / (assign26080_e23436 * assign26080_e23436))), (assign26080_e23438 * (((locals.var_vgstnvt__blk774_dn4 * assign26080_e23436) - (locals.var_vgstnvt__blk774 * ((locals.var_pparam_b4soimstar_dn4 * locals.var_pparam_b4soinoff) + (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff_dn4)))) / (assign26080_e23436 * assign26080_e23436))), (assign26080_e23438 * (((locals.var_vgstnvt__blk774_dn5 * assign26080_e23436) - (locals.var_vgstnvt__blk774 * ((locals.var_pparam_b4soimstar_dn5 * locals.var_pparam_b4soinoff) + (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff_dn5)))) / (assign26080_e23436 * assign26080_e23436))), (assign26080_e23438 * (((locals.var_vgstnvt__blk774_dn6 * assign26080_e23436) - (locals.var_vgstnvt__blk774 * ((locals.var_pparam_b4soimstar_dn6 * locals.var_pparam_b4soinoff) + (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff_dn6)))) / (assign26080_e23436 * assign26080_e23436))), (assign26080_e23438 * (((locals.var_vgstnvt__blk774_dn7 * assign26080_e23436) - (locals.var_vgstnvt__blk774 * ((locals.var_pparam_b4soimstar_dn7 * locals.var_pparam_b4soinoff) + (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff_dn7)))) / (assign26080_e23436 * assign26080_e23436))), (assign26080_e23438 * (((locals.var_vgstnvt__blk774_dn8 * assign26080_e23436) - (locals.var_vgstnvt__blk774 * ((locals.var_pparam_b4soimstar_dn8 * locals.var_pparam_b4soinoff) + (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff_dn8)))) / (assign26080_e23436 * assign26080_e23436))), (assign26080_e23438 * (((locals.var_vgstnvt__blk774_dn9 * assign26080_e23436) - (locals.var_vgstnvt__blk774 * ((locals.var_pparam_b4soimstar_dn9 * locals.var_pparam_b4soinoff) + (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff_dn9)))) / (assign26080_e23436 * assign26080_e23436))), (assign26080_e23438 * (((locals.var_vgstnvt__blk774_dn10 * assign26080_e23436) - (locals.var_vgstnvt__blk774 * ((locals.var_pparam_b4soimstar_dn10 * locals.var_pparam_b4soinoff) + (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff_dn10)))) / (assign26080_e23436 * assign26080_e23436))), (assign26080_e23438 * (((locals.var_vgstnvt__blk774_dn11 * assign26080_e23436) - (locals.var_vgstnvt__blk774 * ((locals.var_pparam_b4soimstar_dn11 * locals.var_pparam_b4soinoff) + (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff_dn11)))) / (assign26080_e23436 * assign26080_e23436))), (assign26080_e23438 * (((locals.var_vgstnvt__blk774_dn12 * assign26080_e23436) - (locals.var_vgstnvt__blk774 * ((locals.var_pparam_b4soimstar_dn12 * locals.var_pparam_b4soinoff) + (locals.var_pparam_b4soimstar * locals.var_pparam_b4soinoff_dn12)))) / (assign26080_e23436 * assign26080_e23436))),)
    } else {
        (locals.var_expvgst__blk775, locals.var_expvgst__blk775_dn3, locals.var_expvgst__blk775_dn4, locals.var_expvgst__blk775_dn5, locals.var_expvgst__blk775_dn6, locals.var_expvgst__blk775_dn7, locals.var_expvgst__blk775_dn8, locals.var_expvgst__blk775_dn9, locals.var_expvgst__blk775_dn10, locals.var_expvgst__blk775_dn11, locals.var_expvgst__blk775_dn12,)
    }
};
        locals.var_expvgst__blk775 = assign26080_e23440;
        locals.var_expvgst__blk775_dn3 = assign26080_e23440_d_n3;
        locals.var_expvgst__blk775_dn4 = assign26080_e23440_d_n4;
        locals.var_expvgst__blk775_dn5 = assign26080_e23440_d_n5;
        locals.var_expvgst__blk775_dn6 = assign26080_e23440_d_n6;
        locals.var_expvgst__blk775_dn7 = assign26080_e23440_d_n7;
        locals.var_expvgst__blk775_dn8 = assign26080_e23440_d_n8;
        locals.var_expvgst__blk775_dn9 = assign26080_e23440_d_n9;
        locals.var_expvgst__blk775_dn10 = assign26080_e23440_d_n10;
        locals.var_expvgst__blk775_dn11 = assign26080_e23440_d_n11;
        locals.var_expvgst__blk775_dn12 = assign26080_e23440_d_n12;

        let (assign26090_e23455, assign26090_e23455_d_n3, assign26090_e23455_d_n4, assign26090_e23455_d_n5, assign26090_e23455_d_n6, assign26090_e23455_d_n7, assign26090_e23455_d_n8, assign26090_e23455_d_n9, assign26090_e23455_d_n10, assign26090_e23455_d_n11, assign26090_e23455_d_n12,) = {
    if (((locals.var_guard1356 == 0.0) && (locals.var_guard1359 != 0.0)) && (locals.var_guard1360 != 0.0)) {
        let assign26090_e23450: f64 = (locals.var_pparam_b4soidelvt / locals.var_noff);
        let assign26090_e23451: f64 = (-assign26090_e23450);
        let assign26090_e23452: f64 = (assign26090_e23451).exp();
        let assign26090_e23453: f64 = (locals.var_expvgst__blk775 * assign26090_e23452);
        (assign26090_e23453, ((locals.var_expvgst__blk775_dn3 * assign26090_e23452) + (locals.var_expvgst__blk775 * (assign26090_e23452 * (-(((locals.var_pparam_b4soidelvt_dn3 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn3)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk775_dn4 * assign26090_e23452) + (locals.var_expvgst__blk775 * (assign26090_e23452 * (-(((locals.var_pparam_b4soidelvt_dn4 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn4)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk775_dn5 * assign26090_e23452) + (locals.var_expvgst__blk775 * (assign26090_e23452 * (-(((locals.var_pparam_b4soidelvt_dn5 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn5)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk775_dn6 * assign26090_e23452) + (locals.var_expvgst__blk775 * (assign26090_e23452 * (-(((locals.var_pparam_b4soidelvt_dn6 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn6)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk775_dn7 * assign26090_e23452) + (locals.var_expvgst__blk775 * (assign26090_e23452 * (-(((locals.var_pparam_b4soidelvt_dn7 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn7)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk775_dn8 * assign26090_e23452) + (locals.var_expvgst__blk775 * (assign26090_e23452 * (-(((locals.var_pparam_b4soidelvt_dn8 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn8)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk775_dn9 * assign26090_e23452) + (locals.var_expvgst__blk775 * (assign26090_e23452 * (-(((locals.var_pparam_b4soidelvt_dn9 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn9)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk775_dn10 * assign26090_e23452) + (locals.var_expvgst__blk775 * (assign26090_e23452 * (-(((locals.var_pparam_b4soidelvt_dn10 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn10)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk775_dn11 * assign26090_e23452) + (locals.var_expvgst__blk775 * (assign26090_e23452 * (-(((locals.var_pparam_b4soidelvt_dn11 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn11)) / (locals.var_noff * locals.var_noff)))))), ((locals.var_expvgst__blk775_dn12 * assign26090_e23452) + (locals.var_expvgst__blk775 * (assign26090_e23452 * (-(((locals.var_pparam_b4soidelvt_dn12 * locals.var_noff) - (locals.var_pparam_b4soidelvt * locals.var_noff_dn12)) / (locals.var_noff * locals.var_noff)))))),)
    } else {
        (locals.var_expvgst__blk775, locals.var_expvgst__blk775_dn3, locals.var_expvgst__blk775_dn4, locals.var_expvgst__blk775_dn5, locals.var_expvgst__blk775_dn6, locals.var_expvgst__blk775_dn7, locals.var_expvgst__blk775_dn8, locals.var_expvgst__blk775_dn9, locals.var_expvgst__blk775_dn10, locals.var_expvgst__blk775_dn11, locals.var_expvgst__blk775_dn12,)
    }
};
        locals.var_expvgst__blk775 = assign26090_e23455;
        locals.var_expvgst__blk775_dn3 = assign26090_e23455_d_n3;
        locals.var_expvgst__blk775_dn4 = assign26090_e23455_d_n4;
        locals.var_expvgst__blk775_dn5 = assign26090_e23455_d_n5;
        locals.var_expvgst__blk775_dn6 = assign26090_e23455_d_n6;
        locals.var_expvgst__blk775_dn7 = assign26090_e23455_d_n7;
        locals.var_expvgst__blk775_dn8 = assign26090_e23455_d_n8;
        locals.var_expvgst__blk775_dn9 = assign26090_e23455_d_n9;
        locals.var_expvgst__blk775_dn10 = assign26090_e23455_d_n10;
        locals.var_expvgst__blk775_dn11 = assign26090_e23455_d_n11;
        locals.var_expvgst__blk775_dn12 = assign26090_e23455_d_n12;

        let (assign26100_e23477, assign26100_e23477_d_n3, assign26100_e23477_d_n4, assign26100_e23477_d_n5, assign26100_e23477_d_n6, assign26100_e23477_d_n7, assign26100_e23477_d_n8, assign26100_e23477_d_n9, assign26100_e23477_d_n10, assign26100_e23477_d_n11, assign26100_e23477_d_n12,) = {
    if (((locals.var_guard1356 == 0.0) && (locals.var_guard1359 != 0.0)) && (locals.var_guard1360 != 0.0)) {
        let assign26100_e23465: f64 = (1.0 + locals.var_expvgst__blk775);
        let (assign26100_e23474, assign26100_e23474_d_n3, assign26100_e23474_d_n4, assign26100_e23474_d_n5, assign26100_e23474_d_n6, assign26100_e23474_d_n7, assign26100_e23474_d_n8, assign26100_e23474_d_n9, assign26100_e23474_d_n10, assign26100_e23474_d_n11, assign26100_e23474_d_n12,) = {
            if (assign26100_e23465 > 1e-38) {
                let assign26100_e23470: f64 = (1.0 + locals.var_expvgst__blk775);
                let assign26100_e23471: f64 = (assign26100_e23470).ln();
                (assign26100_e23471, (locals.var_expvgst__blk775_dn3 / assign26100_e23470), (locals.var_expvgst__blk775_dn4 / assign26100_e23470), (locals.var_expvgst__blk775_dn5 / assign26100_e23470), (locals.var_expvgst__blk775_dn6 / assign26100_e23470), (locals.var_expvgst__blk775_dn7 / assign26100_e23470), (locals.var_expvgst__blk775_dn8 / assign26100_e23470), (locals.var_expvgst__blk775_dn9 / assign26100_e23470), (locals.var_expvgst__blk775_dn10 / assign26100_e23470), (locals.var_expvgst__blk775_dn11 / assign26100_e23470), (locals.var_expvgst__blk775_dn12 / assign26100_e23470),)
            } else {
                let assign26100_e23473: f64 = (-87.49823353377374);
                (assign26100_e23473, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign26100_e23475: f64 = (locals.var_noff * assign26100_e23474);
        (assign26100_e23475, ((locals.var_noff_dn3 * assign26100_e23474) + (locals.var_noff * assign26100_e23474_d_n3)), ((locals.var_noff_dn4 * assign26100_e23474) + (locals.var_noff * assign26100_e23474_d_n4)), ((locals.var_noff_dn5 * assign26100_e23474) + (locals.var_noff * assign26100_e23474_d_n5)), ((locals.var_noff_dn6 * assign26100_e23474) + (locals.var_noff * assign26100_e23474_d_n6)), ((locals.var_noff_dn7 * assign26100_e23474) + (locals.var_noff * assign26100_e23474_d_n7)), ((locals.var_noff_dn8 * assign26100_e23474) + (locals.var_noff * assign26100_e23474_d_n8)), ((locals.var_noff_dn9 * assign26100_e23474) + (locals.var_noff * assign26100_e23474_d_n9)), ((locals.var_noff_dn10 * assign26100_e23474) + (locals.var_noff * assign26100_e23474_d_n10)), ((locals.var_noff_dn11 * assign26100_e23474) + (locals.var_noff * assign26100_e23474_d_n11)), ((locals.var_noff_dn12 * assign26100_e23474) + (locals.var_noff * assign26100_e23474_d_n12)),)
    } else {
        (locals.var_vgsteff__blk840, locals.var_vgsteff__blk840_dn3, locals.var_vgsteff__blk840_dn4, locals.var_vgsteff__blk840_dn5, locals.var_vgsteff__blk840_dn6, locals.var_vgsteff__blk840_dn7, locals.var_vgsteff__blk840_dn8, locals.var_vgsteff__blk840_dn9, locals.var_vgsteff__blk840_dn10, locals.var_vgsteff__blk840_dn11, locals.var_vgsteff__blk840_dn12,)
    }
};
        locals.var_vgsteff__blk840 = assign26100_e23477;
        locals.var_vgsteff__blk840_dn3 = assign26100_e23477_d_n3;
        locals.var_vgsteff__blk840_dn4 = assign26100_e23477_d_n4;
        locals.var_vgsteff__blk840_dn5 = assign26100_e23477_d_n5;
        locals.var_vgsteff__blk840_dn6 = assign26100_e23477_d_n6;
        locals.var_vgsteff__blk840_dn7 = assign26100_e23477_d_n7;
        locals.var_vgsteff__blk840_dn8 = assign26100_e23477_d_n8;
        locals.var_vgsteff__blk840_dn9 = assign26100_e23477_d_n9;
        locals.var_vgsteff__blk840_dn10 = assign26100_e23477_d_n10;
        locals.var_vgsteff__blk840_dn11 = assign26100_e23477_d_n11;
        locals.var_vgsteff__blk840_dn12 = assign26100_e23477_d_n12;

        let assign26110_e23480: f64 = if p.p27 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1361 = assign26110_e23480;

        let (assign26120_e23501, assign26120_e23501_d_n3, assign26120_e23501_d_n4, assign26120_e23501_d_n5, assign26120_e23501_d_n6, assign26120_e23501_d_n7, assign26120_e23501_d_n8, assign26120_e23501_d_n9, assign26120_e23501_d_n10, assign26120_e23501_d_n11, assign26120_e23501_d_n12,) = {
    if ((((locals.var_guard1356 == 0.0) && (locals.var_guard1359 != 0.0)) && (locals.var_guard1360 != 0.0)) && (locals.var_guard1361 != 0.0)) {
        let assign26120_e23491: f64 = (-p.p1033);
        let assign26120_e23493: f64 = (assign26120_e23491 / locals.var_noff2);
        let assign26120_e23496: f64 = (locals.var_vtm * locals.var_vtm);
        let assign26120_e23497: f64 = (assign26120_e23493 / assign26120_e23496);
        let assign26120_e23498: f64 = (assign26120_e23497).exp();
        let assign26120_e23499: f64 = (locals.var_expvgst__blk775 * assign26120_e23498);
        (assign26120_e23499, ((locals.var_expvgst__blk775_dn3 * assign26120_e23498) + (locals.var_expvgst__blk775 * (assign26120_e23498 * ((-((assign26120_e23491 * locals.var_noff2_dn3) / (locals.var_noff2 * locals.var_noff2))) / assign26120_e23496)))), ((locals.var_expvgst__blk775_dn4 * assign26120_e23498) + (locals.var_expvgst__blk775 * (assign26120_e23498 * ((((-((assign26120_e23491 * locals.var_noff2_dn4) / (locals.var_noff2 * locals.var_noff2))) * assign26120_e23496) - (assign26120_e23493 * ((locals.var_vtm_dn4 * locals.var_vtm) + (locals.var_vtm * locals.var_vtm_dn4)))) / (assign26120_e23496 * assign26120_e23496))))), ((locals.var_expvgst__blk775_dn5 * assign26120_e23498) + (locals.var_expvgst__blk775 * (assign26120_e23498 * ((((-((assign26120_e23491 * locals.var_noff2_dn5) / (locals.var_noff2 * locals.var_noff2))) * assign26120_e23496) - (assign26120_e23493 * ((locals.var_vtm_dn5 * locals.var_vtm) + (locals.var_vtm * locals.var_vtm_dn5)))) / (assign26120_e23496 * assign26120_e23496))))), ((locals.var_expvgst__blk775_dn6 * assign26120_e23498) + (locals.var_expvgst__blk775 * (assign26120_e23498 * ((((-((assign26120_e23491 * locals.var_noff2_dn6) / (locals.var_noff2 * locals.var_noff2))) * assign26120_e23496) - (assign26120_e23493 * ((locals.var_vtm_dn6 * locals.var_vtm) + (locals.var_vtm * locals.var_vtm_dn6)))) / (assign26120_e23496 * assign26120_e23496))))), ((locals.var_expvgst__blk775_dn7 * assign26120_e23498) + (locals.var_expvgst__blk775 * (assign26120_e23498 * ((-((assign26120_e23491 * locals.var_noff2_dn7) / (locals.var_noff2 * locals.var_noff2))) / assign26120_e23496)))), ((locals.var_expvgst__blk775_dn8 * assign26120_e23498) + (locals.var_expvgst__blk775 * (assign26120_e23498 * ((-((assign26120_e23491 * locals.var_noff2_dn8) / (locals.var_noff2 * locals.var_noff2))) / assign26120_e23496)))), ((locals.var_expvgst__blk775_dn9 * assign26120_e23498) + (locals.var_expvgst__blk775 * (assign26120_e23498 * ((-((assign26120_e23491 * locals.var_noff2_dn9) / (locals.var_noff2 * locals.var_noff2))) / assign26120_e23496)))), ((locals.var_expvgst__blk775_dn10 * assign26120_e23498) + (locals.var_expvgst__blk775 * (assign26120_e23498 * ((-((assign26120_e23491 * locals.var_noff2_dn10) / (locals.var_noff2 * locals.var_noff2))) / assign26120_e23496)))), ((locals.var_expvgst__blk775_dn11 * assign26120_e23498) + (locals.var_expvgst__blk775 * (assign26120_e23498 * ((-((assign26120_e23491 * locals.var_noff2_dn11) / (locals.var_noff2 * locals.var_noff2))) / assign26120_e23496)))), ((locals.var_expvgst__blk775_dn12 * assign26120_e23498) + (locals.var_expvgst__blk775 * (assign26120_e23498 * ((-((assign26120_e23491 * locals.var_noff2_dn12) / (locals.var_noff2 * locals.var_noff2))) / assign26120_e23496)))),)
    } else {
        (locals.var_expvgst2, locals.var_expvgst2_dn3, locals.var_expvgst2_dn4, locals.var_expvgst2_dn5, locals.var_expvgst2_dn6, locals.var_expvgst2_dn7, locals.var_expvgst2_dn8, locals.var_expvgst2_dn9, locals.var_expvgst2_dn10, locals.var_expvgst2_dn11, locals.var_expvgst2_dn12,)
    }
};
        locals.var_expvgst2 = assign26120_e23501;
        locals.var_expvgst2_dn3 = assign26120_e23501_d_n3;
        locals.var_expvgst2_dn4 = assign26120_e23501_d_n4;
        locals.var_expvgst2_dn5 = assign26120_e23501_d_n5;
        locals.var_expvgst2_dn6 = assign26120_e23501_d_n6;
        locals.var_expvgst2_dn7 = assign26120_e23501_d_n7;
        locals.var_expvgst2_dn8 = assign26120_e23501_d_n8;
        locals.var_expvgst2_dn9 = assign26120_e23501_d_n9;
        locals.var_expvgst2_dn10 = assign26120_e23501_d_n10;
        locals.var_expvgst2_dn11 = assign26120_e23501_d_n11;
        locals.var_expvgst2_dn12 = assign26120_e23501_d_n12;

        let (assign26130_e23525, assign26130_e23525_d_n3, assign26130_e23525_d_n4, assign26130_e23525_d_n5, assign26130_e23525_d_n6, assign26130_e23525_d_n7, assign26130_e23525_d_n8, assign26130_e23525_d_n9, assign26130_e23525_d_n10, assign26130_e23525_d_n11, assign26130_e23525_d_n12,) = {
    if ((((locals.var_guard1356 == 0.0) && (locals.var_guard1359 != 0.0)) && (locals.var_guard1360 != 0.0)) && (locals.var_guard1361 != 0.0)) {
        let assign26130_e23513: f64 = (1.0 + locals.var_expvgst2);
        let (assign26130_e23522, assign26130_e23522_d_n3, assign26130_e23522_d_n4, assign26130_e23522_d_n5, assign26130_e23522_d_n6, assign26130_e23522_d_n7, assign26130_e23522_d_n8, assign26130_e23522_d_n9, assign26130_e23522_d_n10, assign26130_e23522_d_n11, assign26130_e23522_d_n12,) = {
            if (assign26130_e23513 > 1e-38) {
                let assign26130_e23518: f64 = (1.0 + locals.var_expvgst2);
                let assign26130_e23519: f64 = (assign26130_e23518).ln();
                (assign26130_e23519, (locals.var_expvgst2_dn3 / assign26130_e23518), (locals.var_expvgst2_dn4 / assign26130_e23518), (locals.var_expvgst2_dn5 / assign26130_e23518), (locals.var_expvgst2_dn6 / assign26130_e23518), (locals.var_expvgst2_dn7 / assign26130_e23518), (locals.var_expvgst2_dn8 / assign26130_e23518), (locals.var_expvgst2_dn9 / assign26130_e23518), (locals.var_expvgst2_dn10 / assign26130_e23518), (locals.var_expvgst2_dn11 / assign26130_e23518), (locals.var_expvgst2_dn12 / assign26130_e23518),)
            } else {
                let assign26130_e23521: f64 = (-87.49823353377374);
                (assign26130_e23521, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign26130_e23523: f64 = (locals.var_noff2 * assign26130_e23522);
        (assign26130_e23523, ((locals.var_noff2_dn3 * assign26130_e23522) + (locals.var_noff2 * assign26130_e23522_d_n3)), ((locals.var_noff2_dn4 * assign26130_e23522) + (locals.var_noff2 * assign26130_e23522_d_n4)), ((locals.var_noff2_dn5 * assign26130_e23522) + (locals.var_noff2 * assign26130_e23522_d_n5)), ((locals.var_noff2_dn6 * assign26130_e23522) + (locals.var_noff2 * assign26130_e23522_d_n6)), ((locals.var_noff2_dn7 * assign26130_e23522) + (locals.var_noff2 * assign26130_e23522_d_n7)), ((locals.var_noff2_dn8 * assign26130_e23522) + (locals.var_noff2 * assign26130_e23522_d_n8)), ((locals.var_noff2_dn9 * assign26130_e23522) + (locals.var_noff2 * assign26130_e23522_d_n9)), ((locals.var_noff2_dn10 * assign26130_e23522) + (locals.var_noff2 * assign26130_e23522_d_n10)), ((locals.var_noff2_dn11 * assign26130_e23522) + (locals.var_noff2 * assign26130_e23522_d_n11)), ((locals.var_noff2_dn12 * assign26130_e23522) + (locals.var_noff2 * assign26130_e23522_d_n12)),)
    } else {
        (locals.var_vgsteff2, locals.var_vgsteff2_dn3, locals.var_vgsteff2_dn4, locals.var_vgsteff2_dn5, locals.var_vgsteff2_dn6, locals.var_vgsteff2_dn7, locals.var_vgsteff2_dn8, locals.var_vgsteff2_dn9, locals.var_vgsteff2_dn10, locals.var_vgsteff2_dn11, locals.var_vgsteff2_dn12,)
    }
};
        locals.var_vgsteff2 = assign26130_e23525;
        locals.var_vgsteff2_dn3 = assign26130_e23525_d_n3;
        locals.var_vgsteff2_dn4 = assign26130_e23525_d_n4;
        locals.var_vgsteff2_dn5 = assign26130_e23525_d_n5;
        locals.var_vgsteff2_dn6 = assign26130_e23525_d_n6;
        locals.var_vgsteff2_dn7 = assign26130_e23525_d_n7;
        locals.var_vgsteff2_dn8 = assign26130_e23525_d_n8;
        locals.var_vgsteff2_dn9 = assign26130_e23525_d_n9;
        locals.var_vgsteff2_dn10 = assign26130_e23525_d_n10;
        locals.var_vgsteff2_dn11 = assign26130_e23525_d_n11;
        locals.var_vgsteff2_dn12 = assign26130_e23525_d_n12;

        let (assign26140_e23539, assign26140_e23539_d_n3, assign26140_e23539_d_n4, assign26140_e23539_d_n5, assign26140_e23539_d_n6, assign26140_e23539_d_n7, assign26140_e23539_d_n8, assign26140_e23539_d_n9, assign26140_e23539_d_n10, assign26140_e23539_d_n11, assign26140_e23539_d_n12,) = {
    if ((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) {
        let assign26140_e23534: f64 = (locals.var_vgst__blk795 - locals.var_pparam_b4soidelvt);
        let assign26140_e23535: f64 = (locals.var_pparam_b4soimstarcv * assign26140_e23534);
        let assign26140_e23537: f64 = (assign26140_e23535 / locals.var_noff);
        (assign26140_e23537, (((((locals.var_pparam_b4soimstarcv_dn3 * assign26140_e23534) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk795_dn3 - locals.var_pparam_b4soidelvt_dn3))) * locals.var_noff) - (assign26140_e23535 * locals.var_noff_dn3)) / (locals.var_noff * locals.var_noff)), (((((locals.var_pparam_b4soimstarcv_dn4 * assign26140_e23534) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk795_dn4 - locals.var_pparam_b4soidelvt_dn4))) * locals.var_noff) - (assign26140_e23535 * locals.var_noff_dn4)) / (locals.var_noff * locals.var_noff)), (((((locals.var_pparam_b4soimstarcv_dn5 * assign26140_e23534) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk795_dn5 - locals.var_pparam_b4soidelvt_dn5))) * locals.var_noff) - (assign26140_e23535 * locals.var_noff_dn5)) / (locals.var_noff * locals.var_noff)), (((((locals.var_pparam_b4soimstarcv_dn6 * assign26140_e23534) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk795_dn6 - locals.var_pparam_b4soidelvt_dn6))) * locals.var_noff) - (assign26140_e23535 * locals.var_noff_dn6)) / (locals.var_noff * locals.var_noff)), (((((locals.var_pparam_b4soimstarcv_dn7 * assign26140_e23534) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk795_dn7 - locals.var_pparam_b4soidelvt_dn7))) * locals.var_noff) - (assign26140_e23535 * locals.var_noff_dn7)) / (locals.var_noff * locals.var_noff)), (((((locals.var_pparam_b4soimstarcv_dn8 * assign26140_e23534) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk795_dn8 - locals.var_pparam_b4soidelvt_dn8))) * locals.var_noff) - (assign26140_e23535 * locals.var_noff_dn8)) / (locals.var_noff * locals.var_noff)), (((((locals.var_pparam_b4soimstarcv_dn9 * assign26140_e23534) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk795_dn9 - locals.var_pparam_b4soidelvt_dn9))) * locals.var_noff) - (assign26140_e23535 * locals.var_noff_dn9)) / (locals.var_noff * locals.var_noff)), (((((locals.var_pparam_b4soimstarcv_dn10 * assign26140_e23534) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk795_dn10 - locals.var_pparam_b4soidelvt_dn10))) * locals.var_noff) - (assign26140_e23535 * locals.var_noff_dn10)) / (locals.var_noff * locals.var_noff)), (((((locals.var_pparam_b4soimstarcv_dn11 * assign26140_e23534) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk795_dn11 - locals.var_pparam_b4soidelvt_dn11))) * locals.var_noff) - (assign26140_e23535 * locals.var_noff_dn11)) / (locals.var_noff * locals.var_noff)), (((((locals.var_pparam_b4soimstarcv_dn12 * assign26140_e23534) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk795_dn12 - locals.var_pparam_b4soidelvt_dn12))) * locals.var_noff) - (assign26140_e23535 * locals.var_noff_dn12)) / (locals.var_noff * locals.var_noff)),)
    } else {
        (locals.var_vgstnvt__blk774, locals.var_vgstnvt__blk774_dn3, locals.var_vgstnvt__blk774_dn4, locals.var_vgstnvt__blk774_dn5, locals.var_vgstnvt__blk774_dn6, locals.var_vgstnvt__blk774_dn7, locals.var_vgstnvt__blk774_dn8, locals.var_vgstnvt__blk774_dn9, locals.var_vgstnvt__blk774_dn10, locals.var_vgstnvt__blk774_dn11, locals.var_vgstnvt__blk774_dn12,)
    }
};
        locals.var_vgstnvt__blk774 = assign26140_e23539;
        locals.var_vgstnvt__blk774_dn3 = assign26140_e23539_d_n3;
        locals.var_vgstnvt__blk774_dn4 = assign26140_e23539_d_n4;
        locals.var_vgstnvt__blk774_dn5 = assign26140_e23539_d_n5;
        locals.var_vgstnvt__blk774_dn6 = assign26140_e23539_d_n6;
        locals.var_vgstnvt__blk774_dn7 = assign26140_e23539_d_n7;
        locals.var_vgstnvt__blk774_dn8 = assign26140_e23539_d_n8;
        locals.var_vgstnvt__blk774_dn9 = assign26140_e23539_d_n9;
        locals.var_vgstnvt__blk774_dn10 = assign26140_e23539_d_n10;
        locals.var_vgstnvt__blk774_dn11 = assign26140_e23539_d_n11;
        locals.var_vgstnvt__blk774_dn12 = assign26140_e23539_d_n12;

        let (assign26150_e23557, assign26150_e23557_d_n3, assign26150_e23557_d_n4, assign26150_e23557_d_n5, assign26150_e23557_d_n6, assign26150_e23557_d_n7, assign26150_e23557_d_n8, assign26150_e23557_d_n9, assign26150_e23557_d_n10, assign26150_e23557_d_n11, assign26150_e23557_d_n12,) = {
    if ((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) {
        let assign26150_e23548: f64 = (1.0 - locals.var_pparam_b4soimstarcv);
        let assign26150_e23551: f64 = (locals.var_vgst__blk795 - locals.var_pparam_b4soidelvt);
        let assign26150_e23552: f64 = (assign26150_e23548 * assign26150_e23551);
        let assign26150_e23553: f64 = (locals.var_pparam_b4soivoffcv - assign26150_e23552);
        let assign26150_e23555: f64 = (assign26150_e23553 / locals.var_noff);
        (assign26150_e23555, ((((locals.var_pparam_b4soivoffcv_dn3 - (((-locals.var_pparam_b4soimstarcv_dn3) * assign26150_e23551) + (assign26150_e23548 * (locals.var_vgst__blk795_dn3 - locals.var_pparam_b4soidelvt_dn3)))) * locals.var_noff) - (assign26150_e23553 * locals.var_noff_dn3)) / (locals.var_noff * locals.var_noff)), ((((locals.var_pparam_b4soivoffcv_dn4 - (((-locals.var_pparam_b4soimstarcv_dn4) * assign26150_e23551) + (assign26150_e23548 * (locals.var_vgst__blk795_dn4 - locals.var_pparam_b4soidelvt_dn4)))) * locals.var_noff) - (assign26150_e23553 * locals.var_noff_dn4)) / (locals.var_noff * locals.var_noff)), ((((locals.var_pparam_b4soivoffcv_dn5 - (((-locals.var_pparam_b4soimstarcv_dn5) * assign26150_e23551) + (assign26150_e23548 * (locals.var_vgst__blk795_dn5 - locals.var_pparam_b4soidelvt_dn5)))) * locals.var_noff) - (assign26150_e23553 * locals.var_noff_dn5)) / (locals.var_noff * locals.var_noff)), ((((locals.var_pparam_b4soivoffcv_dn6 - (((-locals.var_pparam_b4soimstarcv_dn6) * assign26150_e23551) + (assign26150_e23548 * (locals.var_vgst__blk795_dn6 - locals.var_pparam_b4soidelvt_dn6)))) * locals.var_noff) - (assign26150_e23553 * locals.var_noff_dn6)) / (locals.var_noff * locals.var_noff)), ((((locals.var_pparam_b4soivoffcv_dn7 - (((-locals.var_pparam_b4soimstarcv_dn7) * assign26150_e23551) + (assign26150_e23548 * (locals.var_vgst__blk795_dn7 - locals.var_pparam_b4soidelvt_dn7)))) * locals.var_noff) - (assign26150_e23553 * locals.var_noff_dn7)) / (locals.var_noff * locals.var_noff)), ((((locals.var_pparam_b4soivoffcv_dn8 - (((-locals.var_pparam_b4soimstarcv_dn8) * assign26150_e23551) + (assign26150_e23548 * (locals.var_vgst__blk795_dn8 - locals.var_pparam_b4soidelvt_dn8)))) * locals.var_noff) - (assign26150_e23553 * locals.var_noff_dn8)) / (locals.var_noff * locals.var_noff)), ((((locals.var_pparam_b4soivoffcv_dn9 - (((-locals.var_pparam_b4soimstarcv_dn9) * assign26150_e23551) + (assign26150_e23548 * (locals.var_vgst__blk795_dn9 - locals.var_pparam_b4soidelvt_dn9)))) * locals.var_noff) - (assign26150_e23553 * locals.var_noff_dn9)) / (locals.var_noff * locals.var_noff)), ((((locals.var_pparam_b4soivoffcv_dn10 - (((-locals.var_pparam_b4soimstarcv_dn10) * assign26150_e23551) + (assign26150_e23548 * (locals.var_vgst__blk795_dn10 - locals.var_pparam_b4soidelvt_dn10)))) * locals.var_noff) - (assign26150_e23553 * locals.var_noff_dn10)) / (locals.var_noff * locals.var_noff)), ((((locals.var_pparam_b4soivoffcv_dn11 - (((-locals.var_pparam_b4soimstarcv_dn11) * assign26150_e23551) + (assign26150_e23548 * (locals.var_vgst__blk795_dn11 - locals.var_pparam_b4soidelvt_dn11)))) * locals.var_noff) - (assign26150_e23553 * locals.var_noff_dn11)) / (locals.var_noff * locals.var_noff)), ((((locals.var_pparam_b4soivoffcv_dn12 - (((-locals.var_pparam_b4soimstarcv_dn12) * assign26150_e23551) + (assign26150_e23548 * (locals.var_vgst__blk795_dn12 - locals.var_pparam_b4soidelvt_dn12)))) * locals.var_noff) - (assign26150_e23553 * locals.var_noff_dn12)) / (locals.var_noff * locals.var_noff)),)
    } else {
        (locals.var_exparg__blk798, locals.var_exparg__blk798_dn3, locals.var_exparg__blk798_dn4, locals.var_exparg__blk798_dn5, locals.var_exparg__blk798_dn6, locals.var_exparg__blk798_dn7, locals.var_exparg__blk798_dn8, locals.var_exparg__blk798_dn9, locals.var_exparg__blk798_dn10, locals.var_exparg__blk798_dn11, locals.var_exparg__blk798_dn12,)
    }
};
        locals.var_exparg__blk798 = assign26150_e23557;
        locals.var_exparg__blk798_dn3 = assign26150_e23557_d_n3;
        locals.var_exparg__blk798_dn4 = assign26150_e23557_d_n4;
        locals.var_exparg__blk798_dn5 = assign26150_e23557_d_n5;
        locals.var_exparg__blk798_dn6 = assign26150_e23557_d_n6;
        locals.var_exparg__blk798_dn7 = assign26150_e23557_d_n7;
        locals.var_exparg__blk798_dn8 = assign26150_e23557_d_n8;
        locals.var_exparg__blk798_dn9 = assign26150_e23557_d_n9;
        locals.var_exparg__blk798_dn10 = assign26150_e23557_d_n10;
        locals.var_exparg__blk798_dn11 = assign26150_e23557_d_n11;
        locals.var_exparg__blk798_dn12 = assign26150_e23557_d_n12;

        let assign26160_e23560: f64 = if locals.var_vgstnvt__blk774 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1362 = assign26160_e23560;

        let (assign26170_e23572, assign26170_e23572_d_n3, assign26170_e23572_d_n4, assign26170_e23572_d_n5, assign26170_e23572_d_n6, assign26170_e23572_d_n7, assign26170_e23572_d_n8, assign26170_e23572_d_n9, assign26170_e23572_d_n10, assign26170_e23572_d_n11, assign26170_e23572_d_n12,) = {
    if (((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1362 != 0.0)) {
        let assign26170_e23570: f64 = (locals.var_vgst__blk795 - locals.var_pparam_b4soidelvt);
        (assign26170_e23570, (locals.var_vgst__blk795_dn3 - locals.var_pparam_b4soidelvt_dn3), (locals.var_vgst__blk795_dn4 - locals.var_pparam_b4soidelvt_dn4), (locals.var_vgst__blk795_dn5 - locals.var_pparam_b4soidelvt_dn5), (locals.var_vgst__blk795_dn6 - locals.var_pparam_b4soidelvt_dn6), (locals.var_vgst__blk795_dn7 - locals.var_pparam_b4soidelvt_dn7), (locals.var_vgst__blk795_dn8 - locals.var_pparam_b4soidelvt_dn8), (locals.var_vgst__blk795_dn9 - locals.var_pparam_b4soidelvt_dn9), (locals.var_vgst__blk795_dn10 - locals.var_pparam_b4soidelvt_dn10), (locals.var_vgst__blk795_dn11 - locals.var_pparam_b4soidelvt_dn11), (locals.var_vgst__blk795_dn12 - locals.var_pparam_b4soidelvt_dn12),)
    } else {
        (locals.var_vgsteff__blk840, locals.var_vgsteff__blk840_dn3, locals.var_vgsteff__blk840_dn4, locals.var_vgsteff__blk840_dn5, locals.var_vgsteff__blk840_dn6, locals.var_vgsteff__blk840_dn7, locals.var_vgsteff__blk840_dn8, locals.var_vgsteff__blk840_dn9, locals.var_vgsteff__blk840_dn10, locals.var_vgsteff__blk840_dn11, locals.var_vgsteff__blk840_dn12,)
    }
};
        locals.var_vgsteff__blk840 = assign26170_e23572;
        locals.var_vgsteff__blk840_dn3 = assign26170_e23572_d_n3;
        locals.var_vgsteff__blk840_dn4 = assign26170_e23572_d_n4;
        locals.var_vgsteff__blk840_dn5 = assign26170_e23572_d_n5;
        locals.var_vgsteff__blk840_dn6 = assign26170_e23572_d_n6;
        locals.var_vgsteff__blk840_dn7 = assign26170_e23572_d_n7;
        locals.var_vgsteff__blk840_dn8 = assign26170_e23572_d_n8;
        locals.var_vgsteff__blk840_dn9 = assign26170_e23572_d_n9;
        locals.var_vgsteff__blk840_dn10 = assign26170_e23572_d_n10;
        locals.var_vgsteff__blk840_dn11 = assign26170_e23572_d_n11;
        locals.var_vgsteff__blk840_dn12 = assign26170_e23572_d_n12;

        let assign26180_e23575: f64 = if locals.var_exparg__blk798 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1363 = assign26180_e23575;

        let (assign26190_e23594, assign26190_e23594_d_n3, assign26190_e23594_d_n4, assign26190_e23594_d_n5, assign26190_e23594_d_n6, assign26190_e23594_d_n7, assign26190_e23594_d_n8, assign26190_e23594_d_n9, assign26190_e23594_d_n10, assign26190_e23594_d_n11, assign26190_e23594_d_n12,) = {
    if ((((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1362 == 0.0)) && (locals.var_guard1363 != 0.0)) {
        let assign26190_e23588: f64 = (locals.var_vgst__blk795 - locals.var_pparam_b4soidelvt);
        let assign26190_e23590: f64 = (assign26190_e23588 - locals.var_pparam_b4soivoffcv);
        let assign26190_e23592: f64 = (assign26190_e23590 / locals.var_noff);
        (assign26190_e23592, (((((locals.var_vgst__blk795_dn3 - locals.var_pparam_b4soidelvt_dn3) - locals.var_pparam_b4soivoffcv_dn3) * locals.var_noff) - (assign26190_e23590 * locals.var_noff_dn3)) / (locals.var_noff * locals.var_noff)), (((((locals.var_vgst__blk795_dn4 - locals.var_pparam_b4soidelvt_dn4) - locals.var_pparam_b4soivoffcv_dn4) * locals.var_noff) - (assign26190_e23590 * locals.var_noff_dn4)) / (locals.var_noff * locals.var_noff)), (((((locals.var_vgst__blk795_dn5 - locals.var_pparam_b4soidelvt_dn5) - locals.var_pparam_b4soivoffcv_dn5) * locals.var_noff) - (assign26190_e23590 * locals.var_noff_dn5)) / (locals.var_noff * locals.var_noff)), (((((locals.var_vgst__blk795_dn6 - locals.var_pparam_b4soidelvt_dn6) - locals.var_pparam_b4soivoffcv_dn6) * locals.var_noff) - (assign26190_e23590 * locals.var_noff_dn6)) / (locals.var_noff * locals.var_noff)), (((((locals.var_vgst__blk795_dn7 - locals.var_pparam_b4soidelvt_dn7) - locals.var_pparam_b4soivoffcv_dn7) * locals.var_noff) - (assign26190_e23590 * locals.var_noff_dn7)) / (locals.var_noff * locals.var_noff)), (((((locals.var_vgst__blk795_dn8 - locals.var_pparam_b4soidelvt_dn8) - locals.var_pparam_b4soivoffcv_dn8) * locals.var_noff) - (assign26190_e23590 * locals.var_noff_dn8)) / (locals.var_noff * locals.var_noff)), (((((locals.var_vgst__blk795_dn9 - locals.var_pparam_b4soidelvt_dn9) - locals.var_pparam_b4soivoffcv_dn9) * locals.var_noff) - (assign26190_e23590 * locals.var_noff_dn9)) / (locals.var_noff * locals.var_noff)), (((((locals.var_vgst__blk795_dn10 - locals.var_pparam_b4soidelvt_dn10) - locals.var_pparam_b4soivoffcv_dn10) * locals.var_noff) - (assign26190_e23590 * locals.var_noff_dn10)) / (locals.var_noff * locals.var_noff)), (((((locals.var_vgst__blk795_dn11 - locals.var_pparam_b4soidelvt_dn11) - locals.var_pparam_b4soivoffcv_dn11) * locals.var_noff) - (assign26190_e23590 * locals.var_noff_dn11)) / (locals.var_noff * locals.var_noff)), (((((locals.var_vgst__blk795_dn12 - locals.var_pparam_b4soidelvt_dn12) - locals.var_pparam_b4soivoffcv_dn12) * locals.var_noff) - (assign26190_e23590 * locals.var_noff_dn12)) / (locals.var_noff * locals.var_noff)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign26190_e23594;
        locals.var_t0__blk808_dn3 = assign26190_e23594_d_n3;
        locals.var_t0__blk808_dn4 = assign26190_e23594_d_n4;
        locals.var_t0__blk808_dn5 = assign26190_e23594_d_n5;
        locals.var_t0__blk808_dn6 = assign26190_e23594_d_n6;
        locals.var_t0__blk808_dn7 = assign26190_e23594_d_n7;
        locals.var_t0__blk808_dn8 = assign26190_e23594_d_n8;
        locals.var_t0__blk808_dn9 = assign26190_e23594_d_n9;
        locals.var_t0__blk808_dn10 = assign26190_e23594_d_n10;
        locals.var_t0__blk808_dn11 = assign26190_e23594_d_n11;
        locals.var_t0__blk808_dn12 = assign26190_e23594_d_n12;

        let (assign26200_e23608, assign26200_e23608_d_n3, assign26200_e23608_d_n4, assign26200_e23608_d_n5, assign26200_e23608_d_n6, assign26200_e23608_d_n7, assign26200_e23608_d_n8, assign26200_e23608_d_n9, assign26200_e23608_d_n10, assign26200_e23608_d_n11, assign26200_e23608_d_n12,) = {
    if ((((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1362 == 0.0)) && (locals.var_guard1363 != 0.0)) {
        let assign26200_e23606: f64 = (locals.var_t0__blk808).exp();
        (assign26200_e23606, (assign26200_e23606 * locals.var_t0__blk808_dn3), (assign26200_e23606 * locals.var_t0__blk808_dn4), (assign26200_e23606 * locals.var_t0__blk808_dn5), (assign26200_e23606 * locals.var_t0__blk808_dn6), (assign26200_e23606 * locals.var_t0__blk808_dn7), (assign26200_e23606 * locals.var_t0__blk808_dn8), (assign26200_e23606 * locals.var_t0__blk808_dn9), (assign26200_e23606 * locals.var_t0__blk808_dn10), (assign26200_e23606 * locals.var_t0__blk808_dn11), (assign26200_e23606 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_expvgst__blk775, locals.var_expvgst__blk775_dn3, locals.var_expvgst__blk775_dn4, locals.var_expvgst__blk775_dn5, locals.var_expvgst__blk775_dn6, locals.var_expvgst__blk775_dn7, locals.var_expvgst__blk775_dn8, locals.var_expvgst__blk775_dn9, locals.var_expvgst__blk775_dn10, locals.var_expvgst__blk775_dn11, locals.var_expvgst__blk775_dn12,)
    }
};
        locals.var_expvgst__blk775 = assign26200_e23608;
        locals.var_expvgst__blk775_dn3 = assign26200_e23608_d_n3;
        locals.var_expvgst__blk775_dn4 = assign26200_e23608_d_n4;
        locals.var_expvgst__blk775_dn5 = assign26200_e23608_d_n5;
        locals.var_expvgst__blk775_dn6 = assign26200_e23608_d_n6;
        locals.var_expvgst__blk775_dn7 = assign26200_e23608_d_n7;
        locals.var_expvgst__blk775_dn8 = assign26200_e23608_d_n8;
        locals.var_expvgst__blk775_dn9 = assign26200_e23608_d_n9;
        locals.var_expvgst__blk775_dn10 = assign26200_e23608_d_n10;
        locals.var_expvgst__blk775_dn11 = assign26200_e23608_d_n11;
        locals.var_expvgst__blk775_dn12 = assign26200_e23608_d_n12;

        let (assign26210_e23627, assign26210_e23627_d_n3, assign26210_e23627_d_n4, assign26210_e23627_d_n5, assign26210_e23627_d_n6, assign26210_e23627_d_n7, assign26210_e23627_d_n8, assign26210_e23627_d_n9, assign26210_e23627_d_n10, assign26210_e23627_d_n11, assign26210_e23627_d_n12,) = {
    if ((((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1362 == 0.0)) && (locals.var_guard1363 != 0.0)) {
        let assign26210_e23621: f64 = (locals.var_vtm * locals.var_cdep0);
        let assign26210_e23623: f64 = (assign26210_e23621 / locals.var_b4soicox);
        let assign26210_e23625: f64 = (assign26210_e23623 * locals.var_expvgst__blk775);
        (assign26210_e23625, ((((locals.var_vtm * locals.var_cdep0_dn3) / locals.var_b4soicox) * locals.var_expvgst__blk775) + (assign26210_e23623 * locals.var_expvgst__blk775_dn3)), (((((locals.var_vtm_dn4 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn4)) / locals.var_b4soicox) * locals.var_expvgst__blk775) + (assign26210_e23623 * locals.var_expvgst__blk775_dn4)), (((((locals.var_vtm_dn5 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn5)) / locals.var_b4soicox) * locals.var_expvgst__blk775) + (assign26210_e23623 * locals.var_expvgst__blk775_dn5)), (((((locals.var_vtm_dn6 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn6)) / locals.var_b4soicox) * locals.var_expvgst__blk775) + (assign26210_e23623 * locals.var_expvgst__blk775_dn6)), ((((locals.var_vtm * locals.var_cdep0_dn7) / locals.var_b4soicox) * locals.var_expvgst__blk775) + (assign26210_e23623 * locals.var_expvgst__blk775_dn7)), ((((locals.var_vtm * locals.var_cdep0_dn8) / locals.var_b4soicox) * locals.var_expvgst__blk775) + (assign26210_e23623 * locals.var_expvgst__blk775_dn8)), ((((locals.var_vtm * locals.var_cdep0_dn9) / locals.var_b4soicox) * locals.var_expvgst__blk775) + (assign26210_e23623 * locals.var_expvgst__blk775_dn9)), ((((locals.var_vtm * locals.var_cdep0_dn10) / locals.var_b4soicox) * locals.var_expvgst__blk775) + (assign26210_e23623 * locals.var_expvgst__blk775_dn10)), ((((locals.var_vtm * locals.var_cdep0_dn11) / locals.var_b4soicox) * locals.var_expvgst__blk775) + (assign26210_e23623 * locals.var_expvgst__blk775_dn11)), ((((locals.var_vtm * locals.var_cdep0_dn12) / locals.var_b4soicox) * locals.var_expvgst__blk775) + (assign26210_e23623 * locals.var_expvgst__blk775_dn12)),)
    } else {
        (locals.var_vgsteff__blk840, locals.var_vgsteff__blk840_dn3, locals.var_vgsteff__blk840_dn4, locals.var_vgsteff__blk840_dn5, locals.var_vgsteff__blk840_dn6, locals.var_vgsteff__blk840_dn7, locals.var_vgsteff__blk840_dn8, locals.var_vgsteff__blk840_dn9, locals.var_vgsteff__blk840_dn10, locals.var_vgsteff__blk840_dn11, locals.var_vgsteff__blk840_dn12,)
    }
};
        locals.var_vgsteff__blk840 = assign26210_e23627;
        locals.var_vgsteff__blk840_dn3 = assign26210_e23627_d_n3;
        locals.var_vgsteff__blk840_dn4 = assign26210_e23627_d_n4;
        locals.var_vgsteff__blk840_dn5 = assign26210_e23627_d_n5;
        locals.var_vgsteff__blk840_dn6 = assign26210_e23627_d_n6;
        locals.var_vgsteff__blk840_dn7 = assign26210_e23627_d_n7;
        locals.var_vgsteff__blk840_dn8 = assign26210_e23627_d_n8;
        locals.var_vgsteff__blk840_dn9 = assign26210_e23627_d_n9;
        locals.var_vgsteff__blk840_dn10 = assign26210_e23627_d_n10;
        locals.var_vgsteff__blk840_dn11 = assign26210_e23627_d_n11;
        locals.var_vgsteff__blk840_dn12 = assign26210_e23627_d_n12;

        let (assign26220_e23642, assign26220_e23642_d_n3, assign26220_e23642_d_n4, assign26220_e23642_d_n5, assign26220_e23642_d_n6, assign26220_e23642_d_n7, assign26220_e23642_d_n8, assign26220_e23642_d_n9, assign26220_e23642_d_n10, assign26220_e23642_d_n11, assign26220_e23642_d_n12,) = {
    if ((((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1362 == 0.0)) && (locals.var_guard1363 == 0.0)) {
        let assign26220_e23640: f64 = (locals.var_vgstnvt__blk774).exp();
        (assign26220_e23640, (assign26220_e23640 * locals.var_vgstnvt__blk774_dn3), (assign26220_e23640 * locals.var_vgstnvt__blk774_dn4), (assign26220_e23640 * locals.var_vgstnvt__blk774_dn5), (assign26220_e23640 * locals.var_vgstnvt__blk774_dn6), (assign26220_e23640 * locals.var_vgstnvt__blk774_dn7), (assign26220_e23640 * locals.var_vgstnvt__blk774_dn8), (assign26220_e23640 * locals.var_vgstnvt__blk774_dn9), (assign26220_e23640 * locals.var_vgstnvt__blk774_dn10), (assign26220_e23640 * locals.var_vgstnvt__blk774_dn11), (assign26220_e23640 * locals.var_vgstnvt__blk774_dn12),)
    } else {
        (locals.var_expvgst__blk775, locals.var_expvgst__blk775_dn3, locals.var_expvgst__blk775_dn4, locals.var_expvgst__blk775_dn5, locals.var_expvgst__blk775_dn6, locals.var_expvgst__blk775_dn7, locals.var_expvgst__blk775_dn8, locals.var_expvgst__blk775_dn9, locals.var_expvgst__blk775_dn10, locals.var_expvgst__blk775_dn11, locals.var_expvgst__blk775_dn12,)
    }
};
        locals.var_expvgst__blk775 = assign26220_e23642;
        locals.var_expvgst__blk775_dn3 = assign26220_e23642_d_n3;
        locals.var_expvgst__blk775_dn4 = assign26220_e23642_d_n4;
        locals.var_expvgst__blk775_dn5 = assign26220_e23642_d_n5;
        locals.var_expvgst__blk775_dn6 = assign26220_e23642_d_n6;
        locals.var_expvgst__blk775_dn7 = assign26220_e23642_d_n7;
        locals.var_expvgst__blk775_dn8 = assign26220_e23642_d_n8;
        locals.var_expvgst__blk775_dn9 = assign26220_e23642_d_n9;
        locals.var_expvgst__blk775_dn10 = assign26220_e23642_d_n10;
        locals.var_expvgst__blk775_dn11 = assign26220_e23642_d_n11;
        locals.var_expvgst__blk775_dn12 = assign26220_e23642_d_n12;

        let (assign26230_e23669, assign26230_e23669_d_n3, assign26230_e23669_d_n4, assign26230_e23669_d_n5, assign26230_e23669_d_n6, assign26230_e23669_d_n7, assign26230_e23669_d_n8, assign26230_e23669_d_n9, assign26230_e23669_d_n10, assign26230_e23669_d_n11, assign26230_e23669_d_n12,) = {
    if ((((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1362 == 0.0)) && (locals.var_guard1363 == 0.0)) {
        let assign26230_e23657: f64 = (1.0 + locals.var_expvgst__blk775);
        let (assign26230_e23666, assign26230_e23666_d_n3, assign26230_e23666_d_n4, assign26230_e23666_d_n5, assign26230_e23666_d_n6, assign26230_e23666_d_n7, assign26230_e23666_d_n8, assign26230_e23666_d_n9, assign26230_e23666_d_n10, assign26230_e23666_d_n11, assign26230_e23666_d_n12,) = {
            if (assign26230_e23657 > 1e-38) {
                let assign26230_e23662: f64 = (1.0 + locals.var_expvgst__blk775);
                let assign26230_e23663: f64 = (assign26230_e23662).ln();
                (assign26230_e23663, (locals.var_expvgst__blk775_dn3 / assign26230_e23662), (locals.var_expvgst__blk775_dn4 / assign26230_e23662), (locals.var_expvgst__blk775_dn5 / assign26230_e23662), (locals.var_expvgst__blk775_dn6 / assign26230_e23662), (locals.var_expvgst__blk775_dn7 / assign26230_e23662), (locals.var_expvgst__blk775_dn8 / assign26230_e23662), (locals.var_expvgst__blk775_dn9 / assign26230_e23662), (locals.var_expvgst__blk775_dn10 / assign26230_e23662), (locals.var_expvgst__blk775_dn11 / assign26230_e23662), (locals.var_expvgst__blk775_dn12 / assign26230_e23662),)
            } else {
                let assign26230_e23665: f64 = (-87.49823353377374);
                (assign26230_e23665, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign26230_e23667: f64 = (locals.var_noff * assign26230_e23666);
        (assign26230_e23667, ((locals.var_noff_dn3 * assign26230_e23666) + (locals.var_noff * assign26230_e23666_d_n3)), ((locals.var_noff_dn4 * assign26230_e23666) + (locals.var_noff * assign26230_e23666_d_n4)), ((locals.var_noff_dn5 * assign26230_e23666) + (locals.var_noff * assign26230_e23666_d_n5)), ((locals.var_noff_dn6 * assign26230_e23666) + (locals.var_noff * assign26230_e23666_d_n6)), ((locals.var_noff_dn7 * assign26230_e23666) + (locals.var_noff * assign26230_e23666_d_n7)), ((locals.var_noff_dn8 * assign26230_e23666) + (locals.var_noff * assign26230_e23666_d_n8)), ((locals.var_noff_dn9 * assign26230_e23666) + (locals.var_noff * assign26230_e23666_d_n9)), ((locals.var_noff_dn10 * assign26230_e23666) + (locals.var_noff * assign26230_e23666_d_n10)), ((locals.var_noff_dn11 * assign26230_e23666) + (locals.var_noff * assign26230_e23666_d_n11)), ((locals.var_noff_dn12 * assign26230_e23666) + (locals.var_noff * assign26230_e23666_d_n12)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign26230_e23669;
        locals.var_t1__blk809_dn3 = assign26230_e23669_d_n3;
        locals.var_t1__blk809_dn4 = assign26230_e23669_d_n4;
        locals.var_t1__blk809_dn5 = assign26230_e23669_d_n5;
        locals.var_t1__blk809_dn6 = assign26230_e23669_d_n6;
        locals.var_t1__blk809_dn7 = assign26230_e23669_d_n7;
        locals.var_t1__blk809_dn8 = assign26230_e23669_d_n8;
        locals.var_t1__blk809_dn9 = assign26230_e23669_d_n9;
        locals.var_t1__blk809_dn10 = assign26230_e23669_d_n10;
        locals.var_t1__blk809_dn11 = assign26230_e23669_d_n11;
        locals.var_t1__blk809_dn12 = assign26230_e23669_d_n12;

        let (assign26240_e23695, assign26240_e23695_d_n3, assign26240_e23695_d_n4, assign26240_e23695_d_n5, assign26240_e23695_d_n6, assign26240_e23695_d_n7, assign26240_e23695_d_n8, assign26240_e23695_d_n9, assign26240_e23695_d_n10, assign26240_e23695_d_n11, assign26240_e23695_d_n12,) = {
    if ((((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1362 == 0.0)) && (locals.var_guard1363 == 0.0)) {
        let assign26240_e23682: f64 = (-locals.var_b4soicox);
        let assign26240_e23685: f64 = (locals.var_vtm * locals.var_cdep0);
        let assign26240_e23686: f64 = (assign26240_e23682 / assign26240_e23685);
        let assign26240_e23688: f64 = (locals.var_exparg__blk798).exp();
        let assign26240_e23689: f64 = (assign26240_e23686 * assign26240_e23688);
        let assign26240_e23692: f64 = (1.0 - locals.var_pparam_b4soimstarcv);
        let assign26240_e23693: f64 = (assign26240_e23689 * assign26240_e23692);
        (assign26240_e23693, (((((-((assign26240_e23682 * (locals.var_vtm * locals.var_cdep0_dn3)) / (assign26240_e23685 * assign26240_e23685))) * assign26240_e23688) + (assign26240_e23686 * (assign26240_e23688 * locals.var_exparg__blk798_dn3))) * assign26240_e23692) + (assign26240_e23689 * (-locals.var_pparam_b4soimstarcv_dn3))), (((((-((assign26240_e23682 * ((locals.var_vtm_dn4 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn4))) / (assign26240_e23685 * assign26240_e23685))) * assign26240_e23688) + (assign26240_e23686 * (assign26240_e23688 * locals.var_exparg__blk798_dn4))) * assign26240_e23692) + (assign26240_e23689 * (-locals.var_pparam_b4soimstarcv_dn4))), (((((-((assign26240_e23682 * ((locals.var_vtm_dn5 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn5))) / (assign26240_e23685 * assign26240_e23685))) * assign26240_e23688) + (assign26240_e23686 * (assign26240_e23688 * locals.var_exparg__blk798_dn5))) * assign26240_e23692) + (assign26240_e23689 * (-locals.var_pparam_b4soimstarcv_dn5))), (((((-((assign26240_e23682 * ((locals.var_vtm_dn6 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn6))) / (assign26240_e23685 * assign26240_e23685))) * assign26240_e23688) + (assign26240_e23686 * (assign26240_e23688 * locals.var_exparg__blk798_dn6))) * assign26240_e23692) + (assign26240_e23689 * (-locals.var_pparam_b4soimstarcv_dn6))), (((((-((assign26240_e23682 * (locals.var_vtm * locals.var_cdep0_dn7)) / (assign26240_e23685 * assign26240_e23685))) * assign26240_e23688) + (assign26240_e23686 * (assign26240_e23688 * locals.var_exparg__blk798_dn7))) * assign26240_e23692) + (assign26240_e23689 * (-locals.var_pparam_b4soimstarcv_dn7))), (((((-((assign26240_e23682 * (locals.var_vtm * locals.var_cdep0_dn8)) / (assign26240_e23685 * assign26240_e23685))) * assign26240_e23688) + (assign26240_e23686 * (assign26240_e23688 * locals.var_exparg__blk798_dn8))) * assign26240_e23692) + (assign26240_e23689 * (-locals.var_pparam_b4soimstarcv_dn8))), (((((-((assign26240_e23682 * (locals.var_vtm * locals.var_cdep0_dn9)) / (assign26240_e23685 * assign26240_e23685))) * assign26240_e23688) + (assign26240_e23686 * (assign26240_e23688 * locals.var_exparg__blk798_dn9))) * assign26240_e23692) + (assign26240_e23689 * (-locals.var_pparam_b4soimstarcv_dn9))), (((((-((assign26240_e23682 * (locals.var_vtm * locals.var_cdep0_dn10)) / (assign26240_e23685 * assign26240_e23685))) * assign26240_e23688) + (assign26240_e23686 * (assign26240_e23688 * locals.var_exparg__blk798_dn10))) * assign26240_e23692) + (assign26240_e23689 * (-locals.var_pparam_b4soimstarcv_dn10))), (((((-((assign26240_e23682 * (locals.var_vtm * locals.var_cdep0_dn11)) / (assign26240_e23685 * assign26240_e23685))) * assign26240_e23688) + (assign26240_e23686 * (assign26240_e23688 * locals.var_exparg__blk798_dn11))) * assign26240_e23692) + (assign26240_e23689 * (-locals.var_pparam_b4soimstarcv_dn11))), (((((-((assign26240_e23682 * (locals.var_vtm * locals.var_cdep0_dn12)) / (assign26240_e23685 * assign26240_e23685))) * assign26240_e23688) + (assign26240_e23686 * (assign26240_e23688 * locals.var_exparg__blk798_dn12))) * assign26240_e23692) + (assign26240_e23689 * (-locals.var_pparam_b4soimstarcv_dn12))),)
    } else {
        (locals.var_dt2_dvg, locals.var_dt2_dvg_dn3, locals.var_dt2_dvg_dn4, locals.var_dt2_dvg_dn5, locals.var_dt2_dvg_dn6, locals.var_dt2_dvg_dn7, locals.var_dt2_dvg_dn8, locals.var_dt2_dvg_dn9, locals.var_dt2_dvg_dn10, locals.var_dt2_dvg_dn11, locals.var_dt2_dvg_dn12,)
    }
};
        locals.var_dt2_dvg = assign26240_e23695;
        locals.var_dt2_dvg_dn3 = assign26240_e23695_d_n3;
        locals.var_dt2_dvg_dn4 = assign26240_e23695_d_n4;
        locals.var_dt2_dvg_dn5 = assign26240_e23695_d_n5;
        locals.var_dt2_dvg_dn6 = assign26240_e23695_d_n6;
        locals.var_dt2_dvg_dn7 = assign26240_e23695_d_n7;
        locals.var_dt2_dvg_dn8 = assign26240_e23695_d_n8;
        locals.var_dt2_dvg_dn9 = assign26240_e23695_d_n9;
        locals.var_dt2_dvg_dn10 = assign26240_e23695_d_n10;
        locals.var_dt2_dvg_dn11 = assign26240_e23695_d_n11;
        locals.var_dt2_dvg_dn12 = assign26240_e23695_d_n12;

        let (assign26250_e23717, assign26250_e23717_d_n3, assign26250_e23717_d_n4, assign26250_e23717_d_n5, assign26250_e23717_d_n6, assign26250_e23717_d_n7, assign26250_e23717_d_n8, assign26250_e23717_d_n9, assign26250_e23717_d_n10, assign26250_e23717_d_n11, assign26250_e23717_d_n12,) = {
    if ((((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1362 == 0.0)) && (locals.var_guard1363 == 0.0)) {
        let assign26250_e23710: f64 = (locals.var_noff * locals.var_dt2_dvg);
        let assign26250_e23713: f64 = (1.0 - locals.var_pparam_b4soimstarcv);
        let assign26250_e23714: f64 = (assign26250_e23710 / assign26250_e23713);
        let assign26250_e23715: f64 = (locals.var_pparam_b4soimstarcv - assign26250_e23714);
        (assign26250_e23715, (locals.var_pparam_b4soimstarcv_dn3 - (((((locals.var_noff_dn3 * locals.var_dt2_dvg) + (locals.var_noff * locals.var_dt2_dvg_dn3)) * assign26250_e23713) - (assign26250_e23710 * (-locals.var_pparam_b4soimstarcv_dn3))) / (assign26250_e23713 * assign26250_e23713))), (locals.var_pparam_b4soimstarcv_dn4 - (((((locals.var_noff_dn4 * locals.var_dt2_dvg) + (locals.var_noff * locals.var_dt2_dvg_dn4)) * assign26250_e23713) - (assign26250_e23710 * (-locals.var_pparam_b4soimstarcv_dn4))) / (assign26250_e23713 * assign26250_e23713))), (locals.var_pparam_b4soimstarcv_dn5 - (((((locals.var_noff_dn5 * locals.var_dt2_dvg) + (locals.var_noff * locals.var_dt2_dvg_dn5)) * assign26250_e23713) - (assign26250_e23710 * (-locals.var_pparam_b4soimstarcv_dn5))) / (assign26250_e23713 * assign26250_e23713))), (locals.var_pparam_b4soimstarcv_dn6 - (((((locals.var_noff_dn6 * locals.var_dt2_dvg) + (locals.var_noff * locals.var_dt2_dvg_dn6)) * assign26250_e23713) - (assign26250_e23710 * (-locals.var_pparam_b4soimstarcv_dn6))) / (assign26250_e23713 * assign26250_e23713))), (locals.var_pparam_b4soimstarcv_dn7 - (((((locals.var_noff_dn7 * locals.var_dt2_dvg) + (locals.var_noff * locals.var_dt2_dvg_dn7)) * assign26250_e23713) - (assign26250_e23710 * (-locals.var_pparam_b4soimstarcv_dn7))) / (assign26250_e23713 * assign26250_e23713))), (locals.var_pparam_b4soimstarcv_dn8 - (((((locals.var_noff_dn8 * locals.var_dt2_dvg) + (locals.var_noff * locals.var_dt2_dvg_dn8)) * assign26250_e23713) - (assign26250_e23710 * (-locals.var_pparam_b4soimstarcv_dn8))) / (assign26250_e23713 * assign26250_e23713))), (locals.var_pparam_b4soimstarcv_dn9 - (((((locals.var_noff_dn9 * locals.var_dt2_dvg) + (locals.var_noff * locals.var_dt2_dvg_dn9)) * assign26250_e23713) - (assign26250_e23710 * (-locals.var_pparam_b4soimstarcv_dn9))) / (assign26250_e23713 * assign26250_e23713))), (locals.var_pparam_b4soimstarcv_dn10 - (((((locals.var_noff_dn10 * locals.var_dt2_dvg) + (locals.var_noff * locals.var_dt2_dvg_dn10)) * assign26250_e23713) - (assign26250_e23710 * (-locals.var_pparam_b4soimstarcv_dn10))) / (assign26250_e23713 * assign26250_e23713))), (locals.var_pparam_b4soimstarcv_dn11 - (((((locals.var_noff_dn11 * locals.var_dt2_dvg) + (locals.var_noff * locals.var_dt2_dvg_dn11)) * assign26250_e23713) - (assign26250_e23710 * (-locals.var_pparam_b4soimstarcv_dn11))) / (assign26250_e23713 * assign26250_e23713))), (locals.var_pparam_b4soimstarcv_dn12 - (((((locals.var_noff_dn12 * locals.var_dt2_dvg) + (locals.var_noff * locals.var_dt2_dvg_dn12)) * assign26250_e23713) - (assign26250_e23710 * (-locals.var_pparam_b4soimstarcv_dn12))) / (assign26250_e23713 * assign26250_e23713))),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign26250_e23717;
        locals.var_t2__blk810_dn3 = assign26250_e23717_d_n3;
        locals.var_t2__blk810_dn4 = assign26250_e23717_d_n4;
        locals.var_t2__blk810_dn5 = assign26250_e23717_d_n5;
        locals.var_t2__blk810_dn6 = assign26250_e23717_d_n6;
        locals.var_t2__blk810_dn7 = assign26250_e23717_d_n7;
        locals.var_t2__blk810_dn8 = assign26250_e23717_d_n8;
        locals.var_t2__blk810_dn9 = assign26250_e23717_d_n9;
        locals.var_t2__blk810_dn10 = assign26250_e23717_d_n10;
        locals.var_t2__blk810_dn11 = assign26250_e23717_d_n11;
        locals.var_t2__blk810_dn12 = assign26250_e23717_d_n12;

        let (assign26260_e23733, assign26260_e23733_d_n3, assign26260_e23733_d_n4, assign26260_e23733_d_n5, assign26260_e23733_d_n6, assign26260_e23733_d_n7, assign26260_e23733_d_n8, assign26260_e23733_d_n9, assign26260_e23733_d_n10, assign26260_e23733_d_n11, assign26260_e23733_d_n12,) = {
    if ((((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1362 == 0.0)) && (locals.var_guard1363 == 0.0)) {
        let assign26260_e23731: f64 = (locals.var_t1__blk809 / locals.var_t2__blk810);
        (assign26260_e23731, (((locals.var_t1__blk809_dn3 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn3)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn4 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn4)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn5 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn5)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn6 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn6)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn7 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn7)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn8 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn8)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn9 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn9)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn10 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn10)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn11 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn11)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn12 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn12)) / (locals.var_t2__blk810 * locals.var_t2__blk810)),)
    } else {
        (locals.var_vgsteff__blk840, locals.var_vgsteff__blk840_dn3, locals.var_vgsteff__blk840_dn4, locals.var_vgsteff__blk840_dn5, locals.var_vgsteff__blk840_dn6, locals.var_vgsteff__blk840_dn7, locals.var_vgsteff__blk840_dn8, locals.var_vgsteff__blk840_dn9, locals.var_vgsteff__blk840_dn10, locals.var_vgsteff__blk840_dn11, locals.var_vgsteff__blk840_dn12,)
    }
};
        locals.var_vgsteff__blk840 = assign26260_e23733;
        locals.var_vgsteff__blk840_dn3 = assign26260_e23733_d_n3;
        locals.var_vgsteff__blk840_dn4 = assign26260_e23733_d_n4;
        locals.var_vgsteff__blk840_dn5 = assign26260_e23733_d_n5;
        locals.var_vgsteff__blk840_dn6 = assign26260_e23733_d_n6;
        locals.var_vgsteff__blk840_dn7 = assign26260_e23733_d_n7;
        locals.var_vgsteff__blk840_dn8 = assign26260_e23733_d_n8;
        locals.var_vgsteff__blk840_dn9 = assign26260_e23733_d_n9;
        locals.var_vgsteff__blk840_dn10 = assign26260_e23733_d_n10;
        locals.var_vgsteff__blk840_dn11 = assign26260_e23733_d_n11;
        locals.var_vgsteff__blk840_dn12 = assign26260_e23733_d_n12;

        let assign26270_e23736: f64 = if p.p27 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1364 = assign26270_e23736;

        let (assign26280_e23754, assign26280_e23754_d_n3, assign26280_e23754_d_n4, assign26280_e23754_d_n5, assign26280_e23754_d_n6, assign26280_e23754_d_n7, assign26280_e23754_d_n8, assign26280_e23754_d_n9, assign26280_e23754_d_n10, assign26280_e23754_d_n11, assign26280_e23754_d_n12,) = {
    if (((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1364 != 0.0)) {
        let assign26280_e23747: f64 = (locals.var_vgst__blk795 - locals.var_pparam_b4soidelvt);
        let assign26280_e23749: f64 = (assign26280_e23747 - p.p1033);
        let assign26280_e23750: f64 = (locals.var_pparam_b4soimstarcv * assign26280_e23749);
        let assign26280_e23752: f64 = (assign26280_e23750 / locals.var_noff2);
        (assign26280_e23752, (((((locals.var_pparam_b4soimstarcv_dn3 * assign26280_e23749) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk795_dn3 - locals.var_pparam_b4soidelvt_dn3))) * locals.var_noff2) - (assign26280_e23750 * locals.var_noff2_dn3)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_pparam_b4soimstarcv_dn4 * assign26280_e23749) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk795_dn4 - locals.var_pparam_b4soidelvt_dn4))) * locals.var_noff2) - (assign26280_e23750 * locals.var_noff2_dn4)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_pparam_b4soimstarcv_dn5 * assign26280_e23749) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk795_dn5 - locals.var_pparam_b4soidelvt_dn5))) * locals.var_noff2) - (assign26280_e23750 * locals.var_noff2_dn5)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_pparam_b4soimstarcv_dn6 * assign26280_e23749) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk795_dn6 - locals.var_pparam_b4soidelvt_dn6))) * locals.var_noff2) - (assign26280_e23750 * locals.var_noff2_dn6)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_pparam_b4soimstarcv_dn7 * assign26280_e23749) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk795_dn7 - locals.var_pparam_b4soidelvt_dn7))) * locals.var_noff2) - (assign26280_e23750 * locals.var_noff2_dn7)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_pparam_b4soimstarcv_dn8 * assign26280_e23749) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk795_dn8 - locals.var_pparam_b4soidelvt_dn8))) * locals.var_noff2) - (assign26280_e23750 * locals.var_noff2_dn8)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_pparam_b4soimstarcv_dn9 * assign26280_e23749) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk795_dn9 - locals.var_pparam_b4soidelvt_dn9))) * locals.var_noff2) - (assign26280_e23750 * locals.var_noff2_dn9)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_pparam_b4soimstarcv_dn10 * assign26280_e23749) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk795_dn10 - locals.var_pparam_b4soidelvt_dn10))) * locals.var_noff2) - (assign26280_e23750 * locals.var_noff2_dn10)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_pparam_b4soimstarcv_dn11 * assign26280_e23749) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk795_dn11 - locals.var_pparam_b4soidelvt_dn11))) * locals.var_noff2) - (assign26280_e23750 * locals.var_noff2_dn11)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_pparam_b4soimstarcv_dn12 * assign26280_e23749) + (locals.var_pparam_b4soimstarcv * (locals.var_vgst__blk795_dn12 - locals.var_pparam_b4soidelvt_dn12))) * locals.var_noff2) - (assign26280_e23750 * locals.var_noff2_dn12)) / (locals.var_noff2 * locals.var_noff2)),)
    } else {
        (locals.var_vgstnvt2, locals.var_vgstnvt2_dn3, locals.var_vgstnvt2_dn4, locals.var_vgstnvt2_dn5, locals.var_vgstnvt2_dn6, locals.var_vgstnvt2_dn7, locals.var_vgstnvt2_dn8, locals.var_vgstnvt2_dn9, locals.var_vgstnvt2_dn10, locals.var_vgstnvt2_dn11, locals.var_vgstnvt2_dn12,)
    }
};
        locals.var_vgstnvt2 = assign26280_e23754;
        locals.var_vgstnvt2_dn3 = assign26280_e23754_d_n3;
        locals.var_vgstnvt2_dn4 = assign26280_e23754_d_n4;
        locals.var_vgstnvt2_dn5 = assign26280_e23754_d_n5;
        locals.var_vgstnvt2_dn6 = assign26280_e23754_d_n6;
        locals.var_vgstnvt2_dn7 = assign26280_e23754_d_n7;
        locals.var_vgstnvt2_dn8 = assign26280_e23754_d_n8;
        locals.var_vgstnvt2_dn9 = assign26280_e23754_d_n9;
        locals.var_vgstnvt2_dn10 = assign26280_e23754_d_n10;
        locals.var_vgstnvt2_dn11 = assign26280_e23754_d_n11;
        locals.var_vgstnvt2_dn12 = assign26280_e23754_d_n12;

        let (assign26290_e23776, assign26290_e23776_d_n3, assign26290_e23776_d_n4, assign26290_e23776_d_n5, assign26290_e23776_d_n6, assign26290_e23776_d_n7, assign26290_e23776_d_n8, assign26290_e23776_d_n9, assign26290_e23776_d_n10, assign26290_e23776_d_n11, assign26290_e23776_d_n12,) = {
    if (((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1364 != 0.0)) {
        let assign26290_e23765: f64 = (1.0 - locals.var_pparam_b4soimstarcv);
        let assign26290_e23768: f64 = (locals.var_vgst__blk795 - locals.var_pparam_b4soidelvt);
        let assign26290_e23770: f64 = (assign26290_e23768 - p.p1033);
        let assign26290_e23771: f64 = (assign26290_e23765 * assign26290_e23770);
        let assign26290_e23772: f64 = (locals.var_pparam_b4soivoffcv - assign26290_e23771);
        let assign26290_e23774: f64 = (assign26290_e23772 / locals.var_noff2);
        (assign26290_e23774, ((((locals.var_pparam_b4soivoffcv_dn3 - (((-locals.var_pparam_b4soimstarcv_dn3) * assign26290_e23770) + (assign26290_e23765 * (locals.var_vgst__blk795_dn3 - locals.var_pparam_b4soidelvt_dn3)))) * locals.var_noff2) - (assign26290_e23772 * locals.var_noff2_dn3)) / (locals.var_noff2 * locals.var_noff2)), ((((locals.var_pparam_b4soivoffcv_dn4 - (((-locals.var_pparam_b4soimstarcv_dn4) * assign26290_e23770) + (assign26290_e23765 * (locals.var_vgst__blk795_dn4 - locals.var_pparam_b4soidelvt_dn4)))) * locals.var_noff2) - (assign26290_e23772 * locals.var_noff2_dn4)) / (locals.var_noff2 * locals.var_noff2)), ((((locals.var_pparam_b4soivoffcv_dn5 - (((-locals.var_pparam_b4soimstarcv_dn5) * assign26290_e23770) + (assign26290_e23765 * (locals.var_vgst__blk795_dn5 - locals.var_pparam_b4soidelvt_dn5)))) * locals.var_noff2) - (assign26290_e23772 * locals.var_noff2_dn5)) / (locals.var_noff2 * locals.var_noff2)), ((((locals.var_pparam_b4soivoffcv_dn6 - (((-locals.var_pparam_b4soimstarcv_dn6) * assign26290_e23770) + (assign26290_e23765 * (locals.var_vgst__blk795_dn6 - locals.var_pparam_b4soidelvt_dn6)))) * locals.var_noff2) - (assign26290_e23772 * locals.var_noff2_dn6)) / (locals.var_noff2 * locals.var_noff2)), ((((locals.var_pparam_b4soivoffcv_dn7 - (((-locals.var_pparam_b4soimstarcv_dn7) * assign26290_e23770) + (assign26290_e23765 * (locals.var_vgst__blk795_dn7 - locals.var_pparam_b4soidelvt_dn7)))) * locals.var_noff2) - (assign26290_e23772 * locals.var_noff2_dn7)) / (locals.var_noff2 * locals.var_noff2)), ((((locals.var_pparam_b4soivoffcv_dn8 - (((-locals.var_pparam_b4soimstarcv_dn8) * assign26290_e23770) + (assign26290_e23765 * (locals.var_vgst__blk795_dn8 - locals.var_pparam_b4soidelvt_dn8)))) * locals.var_noff2) - (assign26290_e23772 * locals.var_noff2_dn8)) / (locals.var_noff2 * locals.var_noff2)), ((((locals.var_pparam_b4soivoffcv_dn9 - (((-locals.var_pparam_b4soimstarcv_dn9) * assign26290_e23770) + (assign26290_e23765 * (locals.var_vgst__blk795_dn9 - locals.var_pparam_b4soidelvt_dn9)))) * locals.var_noff2) - (assign26290_e23772 * locals.var_noff2_dn9)) / (locals.var_noff2 * locals.var_noff2)), ((((locals.var_pparam_b4soivoffcv_dn10 - (((-locals.var_pparam_b4soimstarcv_dn10) * assign26290_e23770) + (assign26290_e23765 * (locals.var_vgst__blk795_dn10 - locals.var_pparam_b4soidelvt_dn10)))) * locals.var_noff2) - (assign26290_e23772 * locals.var_noff2_dn10)) / (locals.var_noff2 * locals.var_noff2)), ((((locals.var_pparam_b4soivoffcv_dn11 - (((-locals.var_pparam_b4soimstarcv_dn11) * assign26290_e23770) + (assign26290_e23765 * (locals.var_vgst__blk795_dn11 - locals.var_pparam_b4soidelvt_dn11)))) * locals.var_noff2) - (assign26290_e23772 * locals.var_noff2_dn11)) / (locals.var_noff2 * locals.var_noff2)), ((((locals.var_pparam_b4soivoffcv_dn12 - (((-locals.var_pparam_b4soimstarcv_dn12) * assign26290_e23770) + (assign26290_e23765 * (locals.var_vgst__blk795_dn12 - locals.var_pparam_b4soidelvt_dn12)))) * locals.var_noff2) - (assign26290_e23772 * locals.var_noff2_dn12)) / (locals.var_noff2 * locals.var_noff2)),)
    } else {
        (locals.var_exparg2, locals.var_exparg2_dn3, locals.var_exparg2_dn4, locals.var_exparg2_dn5, locals.var_exparg2_dn6, locals.var_exparg2_dn7, locals.var_exparg2_dn8, locals.var_exparg2_dn9, locals.var_exparg2_dn10, locals.var_exparg2_dn11, locals.var_exparg2_dn12,)
    }
};
        locals.var_exparg2 = assign26290_e23776;
        locals.var_exparg2_dn3 = assign26290_e23776_d_n3;
        locals.var_exparg2_dn4 = assign26290_e23776_d_n4;
        locals.var_exparg2_dn5 = assign26290_e23776_d_n5;
        locals.var_exparg2_dn6 = assign26290_e23776_d_n6;
        locals.var_exparg2_dn7 = assign26290_e23776_d_n7;
        locals.var_exparg2_dn8 = assign26290_e23776_d_n8;
        locals.var_exparg2_dn9 = assign26290_e23776_d_n9;
        locals.var_exparg2_dn10 = assign26290_e23776_d_n10;
        locals.var_exparg2_dn11 = assign26290_e23776_d_n11;
        locals.var_exparg2_dn12 = assign26290_e23776_d_n12;

        let assign26300_e23779: f64 = if locals.var_vgstnvt2 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1365 = assign26300_e23779;

        let (assign26310_e23795, assign26310_e23795_d_n3, assign26310_e23795_d_n4, assign26310_e23795_d_n5, assign26310_e23795_d_n6, assign26310_e23795_d_n7, assign26310_e23795_d_n8, assign26310_e23795_d_n9, assign26310_e23795_d_n10, assign26310_e23795_d_n11, assign26310_e23795_d_n12,) = {
    if ((((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1364 != 0.0)) && (locals.var_guard1365 != 0.0)) {
        let assign26310_e23791: f64 = (locals.var_vgst__blk795 - locals.var_pparam_b4soidelvt);
        let assign26310_e23793: f64 = (assign26310_e23791 - p.p1033);
        (assign26310_e23793, (locals.var_vgst__blk795_dn3 - locals.var_pparam_b4soidelvt_dn3), (locals.var_vgst__blk795_dn4 - locals.var_pparam_b4soidelvt_dn4), (locals.var_vgst__blk795_dn5 - locals.var_pparam_b4soidelvt_dn5), (locals.var_vgst__blk795_dn6 - locals.var_pparam_b4soidelvt_dn6), (locals.var_vgst__blk795_dn7 - locals.var_pparam_b4soidelvt_dn7), (locals.var_vgst__blk795_dn8 - locals.var_pparam_b4soidelvt_dn8), (locals.var_vgst__blk795_dn9 - locals.var_pparam_b4soidelvt_dn9), (locals.var_vgst__blk795_dn10 - locals.var_pparam_b4soidelvt_dn10), (locals.var_vgst__blk795_dn11 - locals.var_pparam_b4soidelvt_dn11), (locals.var_vgst__blk795_dn12 - locals.var_pparam_b4soidelvt_dn12),)
    } else {
        (locals.var_vgsteff2, locals.var_vgsteff2_dn3, locals.var_vgsteff2_dn4, locals.var_vgsteff2_dn5, locals.var_vgsteff2_dn6, locals.var_vgsteff2_dn7, locals.var_vgsteff2_dn8, locals.var_vgsteff2_dn9, locals.var_vgsteff2_dn10, locals.var_vgsteff2_dn11, locals.var_vgsteff2_dn12,)
    }
};
        locals.var_vgsteff2 = assign26310_e23795;
        locals.var_vgsteff2_dn3 = assign26310_e23795_d_n3;
        locals.var_vgsteff2_dn4 = assign26310_e23795_d_n4;
        locals.var_vgsteff2_dn5 = assign26310_e23795_d_n5;
        locals.var_vgsteff2_dn6 = assign26310_e23795_d_n6;
        locals.var_vgsteff2_dn7 = assign26310_e23795_d_n7;
        locals.var_vgsteff2_dn8 = assign26310_e23795_d_n8;
        locals.var_vgsteff2_dn9 = assign26310_e23795_d_n9;
        locals.var_vgsteff2_dn10 = assign26310_e23795_d_n10;
        locals.var_vgsteff2_dn11 = assign26310_e23795_d_n11;
        locals.var_vgsteff2_dn12 = assign26310_e23795_d_n12;

        let assign26320_e23798: f64 = if locals.var_exparg2 > 100.0 { 1.0 } else { 0.0 };
        locals.var_guard1366 = assign26320_e23798;

        let (assign26330_e23821, assign26330_e23821_d_n3, assign26330_e23821_d_n4, assign26330_e23821_d_n5, assign26330_e23821_d_n6, assign26330_e23821_d_n7, assign26330_e23821_d_n8, assign26330_e23821_d_n9, assign26330_e23821_d_n10, assign26330_e23821_d_n11, assign26330_e23821_d_n12,) = {
    if (((((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1364 != 0.0)) && (locals.var_guard1365 == 0.0)) && (locals.var_guard1366 != 0.0)) {
        let assign26330_e23813: f64 = (locals.var_vgst__blk795 - locals.var_pparam_b4soidelvt);
        let assign26330_e23815: f64 = (assign26330_e23813 - locals.var_pparam_b4soivoffcv);
        let assign26330_e23817: f64 = (assign26330_e23815 - p.p1033);
        let assign26330_e23819: f64 = (assign26330_e23817 / locals.var_noff2);
        (assign26330_e23819, (((((locals.var_vgst__blk795_dn3 - locals.var_pparam_b4soidelvt_dn3) - locals.var_pparam_b4soivoffcv_dn3) * locals.var_noff2) - (assign26330_e23817 * locals.var_noff2_dn3)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_vgst__blk795_dn4 - locals.var_pparam_b4soidelvt_dn4) - locals.var_pparam_b4soivoffcv_dn4) * locals.var_noff2) - (assign26330_e23817 * locals.var_noff2_dn4)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_vgst__blk795_dn5 - locals.var_pparam_b4soidelvt_dn5) - locals.var_pparam_b4soivoffcv_dn5) * locals.var_noff2) - (assign26330_e23817 * locals.var_noff2_dn5)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_vgst__blk795_dn6 - locals.var_pparam_b4soidelvt_dn6) - locals.var_pparam_b4soivoffcv_dn6) * locals.var_noff2) - (assign26330_e23817 * locals.var_noff2_dn6)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_vgst__blk795_dn7 - locals.var_pparam_b4soidelvt_dn7) - locals.var_pparam_b4soivoffcv_dn7) * locals.var_noff2) - (assign26330_e23817 * locals.var_noff2_dn7)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_vgst__blk795_dn8 - locals.var_pparam_b4soidelvt_dn8) - locals.var_pparam_b4soivoffcv_dn8) * locals.var_noff2) - (assign26330_e23817 * locals.var_noff2_dn8)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_vgst__blk795_dn9 - locals.var_pparam_b4soidelvt_dn9) - locals.var_pparam_b4soivoffcv_dn9) * locals.var_noff2) - (assign26330_e23817 * locals.var_noff2_dn9)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_vgst__blk795_dn10 - locals.var_pparam_b4soidelvt_dn10) - locals.var_pparam_b4soivoffcv_dn10) * locals.var_noff2) - (assign26330_e23817 * locals.var_noff2_dn10)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_vgst__blk795_dn11 - locals.var_pparam_b4soidelvt_dn11) - locals.var_pparam_b4soivoffcv_dn11) * locals.var_noff2) - (assign26330_e23817 * locals.var_noff2_dn11)) / (locals.var_noff2 * locals.var_noff2)), (((((locals.var_vgst__blk795_dn12 - locals.var_pparam_b4soidelvt_dn12) - locals.var_pparam_b4soivoffcv_dn12) * locals.var_noff2) - (assign26330_e23817 * locals.var_noff2_dn12)) / (locals.var_noff2 * locals.var_noff2)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign26330_e23821;
        locals.var_t0__blk808_dn3 = assign26330_e23821_d_n3;
        locals.var_t0__blk808_dn4 = assign26330_e23821_d_n4;
        locals.var_t0__blk808_dn5 = assign26330_e23821_d_n5;
        locals.var_t0__blk808_dn6 = assign26330_e23821_d_n6;
        locals.var_t0__blk808_dn7 = assign26330_e23821_d_n7;
        locals.var_t0__blk808_dn8 = assign26330_e23821_d_n8;
        locals.var_t0__blk808_dn9 = assign26330_e23821_d_n9;
        locals.var_t0__blk808_dn10 = assign26330_e23821_d_n10;
        locals.var_t0__blk808_dn11 = assign26330_e23821_d_n11;
        locals.var_t0__blk808_dn12 = assign26330_e23821_d_n12;

        let (assign26340_e23837, assign26340_e23837_d_n3, assign26340_e23837_d_n4, assign26340_e23837_d_n5, assign26340_e23837_d_n6, assign26340_e23837_d_n7, assign26340_e23837_d_n8, assign26340_e23837_d_n9, assign26340_e23837_d_n10, assign26340_e23837_d_n11, assign26340_e23837_d_n12,) = {
    if (((((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1364 != 0.0)) && (locals.var_guard1365 == 0.0)) && (locals.var_guard1366 != 0.0)) {
        let assign26340_e23835: f64 = (locals.var_t0__blk808).exp();
        (assign26340_e23835, (assign26340_e23835 * locals.var_t0__blk808_dn3), (assign26340_e23835 * locals.var_t0__blk808_dn4), (assign26340_e23835 * locals.var_t0__blk808_dn5), (assign26340_e23835 * locals.var_t0__blk808_dn6), (assign26340_e23835 * locals.var_t0__blk808_dn7), (assign26340_e23835 * locals.var_t0__blk808_dn8), (assign26340_e23835 * locals.var_t0__blk808_dn9), (assign26340_e23835 * locals.var_t0__blk808_dn10), (assign26340_e23835 * locals.var_t0__blk808_dn11), (assign26340_e23835 * locals.var_t0__blk808_dn12),)
    } else {
        (locals.var_expvgst2, locals.var_expvgst2_dn3, locals.var_expvgst2_dn4, locals.var_expvgst2_dn5, locals.var_expvgst2_dn6, locals.var_expvgst2_dn7, locals.var_expvgst2_dn8, locals.var_expvgst2_dn9, locals.var_expvgst2_dn10, locals.var_expvgst2_dn11, locals.var_expvgst2_dn12,)
    }
};
        locals.var_expvgst2 = assign26340_e23837;
        locals.var_expvgst2_dn3 = assign26340_e23837_d_n3;
        locals.var_expvgst2_dn4 = assign26340_e23837_d_n4;
        locals.var_expvgst2_dn5 = assign26340_e23837_d_n5;
        locals.var_expvgst2_dn6 = assign26340_e23837_d_n6;
        locals.var_expvgst2_dn7 = assign26340_e23837_d_n7;
        locals.var_expvgst2_dn8 = assign26340_e23837_d_n8;
        locals.var_expvgst2_dn9 = assign26340_e23837_d_n9;
        locals.var_expvgst2_dn10 = assign26340_e23837_d_n10;
        locals.var_expvgst2_dn11 = assign26340_e23837_d_n11;
        locals.var_expvgst2_dn12 = assign26340_e23837_d_n12;

    }

    pub(super) fn stamp_transient_block_79(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26350_e23858, assign26350_e23858_d_n3, assign26350_e23858_d_n4, assign26350_e23858_d_n5, assign26350_e23858_d_n6, assign26350_e23858_d_n7, assign26350_e23858_d_n8, assign26350_e23858_d_n9, assign26350_e23858_d_n10, assign26350_e23858_d_n11, assign26350_e23858_d_n12,) = {
    if (((((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1364 != 0.0)) && (locals.var_guard1365 == 0.0)) && (locals.var_guard1366 != 0.0)) {
        let assign26350_e23852: f64 = (locals.var_vtm * locals.var_cdep0);
        let assign26350_e23854: f64 = (assign26350_e23852 / locals.var_b4soicox);
        let assign26350_e23856: f64 = (assign26350_e23854 * locals.var_expvgst2);
        (assign26350_e23856, ((((locals.var_vtm * locals.var_cdep0_dn3) / locals.var_b4soicox) * locals.var_expvgst2) + (assign26350_e23854 * locals.var_expvgst2_dn3)), (((((locals.var_vtm_dn4 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn4)) / locals.var_b4soicox) * locals.var_expvgst2) + (assign26350_e23854 * locals.var_expvgst2_dn4)), (((((locals.var_vtm_dn5 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn5)) / locals.var_b4soicox) * locals.var_expvgst2) + (assign26350_e23854 * locals.var_expvgst2_dn5)), (((((locals.var_vtm_dn6 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn6)) / locals.var_b4soicox) * locals.var_expvgst2) + (assign26350_e23854 * locals.var_expvgst2_dn6)), ((((locals.var_vtm * locals.var_cdep0_dn7) / locals.var_b4soicox) * locals.var_expvgst2) + (assign26350_e23854 * locals.var_expvgst2_dn7)), ((((locals.var_vtm * locals.var_cdep0_dn8) / locals.var_b4soicox) * locals.var_expvgst2) + (assign26350_e23854 * locals.var_expvgst2_dn8)), ((((locals.var_vtm * locals.var_cdep0_dn9) / locals.var_b4soicox) * locals.var_expvgst2) + (assign26350_e23854 * locals.var_expvgst2_dn9)), ((((locals.var_vtm * locals.var_cdep0_dn10) / locals.var_b4soicox) * locals.var_expvgst2) + (assign26350_e23854 * locals.var_expvgst2_dn10)), ((((locals.var_vtm * locals.var_cdep0_dn11) / locals.var_b4soicox) * locals.var_expvgst2) + (assign26350_e23854 * locals.var_expvgst2_dn11)), ((((locals.var_vtm * locals.var_cdep0_dn12) / locals.var_b4soicox) * locals.var_expvgst2) + (assign26350_e23854 * locals.var_expvgst2_dn12)),)
    } else {
        (locals.var_vgsteff2, locals.var_vgsteff2_dn3, locals.var_vgsteff2_dn4, locals.var_vgsteff2_dn5, locals.var_vgsteff2_dn6, locals.var_vgsteff2_dn7, locals.var_vgsteff2_dn8, locals.var_vgsteff2_dn9, locals.var_vgsteff2_dn10, locals.var_vgsteff2_dn11, locals.var_vgsteff2_dn12,)
    }
};
        locals.var_vgsteff2 = assign26350_e23858;
        locals.var_vgsteff2_dn3 = assign26350_e23858_d_n3;
        locals.var_vgsteff2_dn4 = assign26350_e23858_d_n4;
        locals.var_vgsteff2_dn5 = assign26350_e23858_d_n5;
        locals.var_vgsteff2_dn6 = assign26350_e23858_d_n6;
        locals.var_vgsteff2_dn7 = assign26350_e23858_d_n7;
        locals.var_vgsteff2_dn8 = assign26350_e23858_d_n8;
        locals.var_vgsteff2_dn9 = assign26350_e23858_d_n9;
        locals.var_vgsteff2_dn10 = assign26350_e23858_d_n10;
        locals.var_vgsteff2_dn11 = assign26350_e23858_d_n11;
        locals.var_vgsteff2_dn12 = assign26350_e23858_d_n12;

        let (assign26360_e23875, assign26360_e23875_d_n3, assign26360_e23875_d_n4, assign26360_e23875_d_n5, assign26360_e23875_d_n6, assign26360_e23875_d_n7, assign26360_e23875_d_n8, assign26360_e23875_d_n9, assign26360_e23875_d_n10, assign26360_e23875_d_n11, assign26360_e23875_d_n12,) = {
    if (((((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1364 != 0.0)) && (locals.var_guard1365 == 0.0)) && (locals.var_guard1366 == 0.0)) {
        let assign26360_e23873: f64 = (locals.var_vgstnvt2).exp();
        (assign26360_e23873, (assign26360_e23873 * locals.var_vgstnvt2_dn3), (assign26360_e23873 * locals.var_vgstnvt2_dn4), (assign26360_e23873 * locals.var_vgstnvt2_dn5), (assign26360_e23873 * locals.var_vgstnvt2_dn6), (assign26360_e23873 * locals.var_vgstnvt2_dn7), (assign26360_e23873 * locals.var_vgstnvt2_dn8), (assign26360_e23873 * locals.var_vgstnvt2_dn9), (assign26360_e23873 * locals.var_vgstnvt2_dn10), (assign26360_e23873 * locals.var_vgstnvt2_dn11), (assign26360_e23873 * locals.var_vgstnvt2_dn12),)
    } else {
        (locals.var_expvgst2, locals.var_expvgst2_dn3, locals.var_expvgst2_dn4, locals.var_expvgst2_dn5, locals.var_expvgst2_dn6, locals.var_expvgst2_dn7, locals.var_expvgst2_dn8, locals.var_expvgst2_dn9, locals.var_expvgst2_dn10, locals.var_expvgst2_dn11, locals.var_expvgst2_dn12,)
    }
};
        locals.var_expvgst2 = assign26360_e23875;
        locals.var_expvgst2_dn3 = assign26360_e23875_d_n3;
        locals.var_expvgst2_dn4 = assign26360_e23875_d_n4;
        locals.var_expvgst2_dn5 = assign26360_e23875_d_n5;
        locals.var_expvgst2_dn6 = assign26360_e23875_d_n6;
        locals.var_expvgst2_dn7 = assign26360_e23875_d_n7;
        locals.var_expvgst2_dn8 = assign26360_e23875_d_n8;
        locals.var_expvgst2_dn9 = assign26360_e23875_d_n9;
        locals.var_expvgst2_dn10 = assign26360_e23875_d_n10;
        locals.var_expvgst2_dn11 = assign26360_e23875_d_n11;
        locals.var_expvgst2_dn12 = assign26360_e23875_d_n12;

        let (assign26370_e23904, assign26370_e23904_d_n3, assign26370_e23904_d_n4, assign26370_e23904_d_n5, assign26370_e23904_d_n6, assign26370_e23904_d_n7, assign26370_e23904_d_n8, assign26370_e23904_d_n9, assign26370_e23904_d_n10, assign26370_e23904_d_n11, assign26370_e23904_d_n12,) = {
    if (((((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1364 != 0.0)) && (locals.var_guard1365 == 0.0)) && (locals.var_guard1366 == 0.0)) {
        let assign26370_e23892: f64 = (1.0 + locals.var_expvgst2);
        let (assign26370_e23901, assign26370_e23901_d_n3, assign26370_e23901_d_n4, assign26370_e23901_d_n5, assign26370_e23901_d_n6, assign26370_e23901_d_n7, assign26370_e23901_d_n8, assign26370_e23901_d_n9, assign26370_e23901_d_n10, assign26370_e23901_d_n11, assign26370_e23901_d_n12,) = {
            if (assign26370_e23892 > 1e-38) {
                let assign26370_e23897: f64 = (1.0 + locals.var_expvgst2);
                let assign26370_e23898: f64 = (assign26370_e23897).ln();
                (assign26370_e23898, (locals.var_expvgst2_dn3 / assign26370_e23897), (locals.var_expvgst2_dn4 / assign26370_e23897), (locals.var_expvgst2_dn5 / assign26370_e23897), (locals.var_expvgst2_dn6 / assign26370_e23897), (locals.var_expvgst2_dn7 / assign26370_e23897), (locals.var_expvgst2_dn8 / assign26370_e23897), (locals.var_expvgst2_dn9 / assign26370_e23897), (locals.var_expvgst2_dn10 / assign26370_e23897), (locals.var_expvgst2_dn11 / assign26370_e23897), (locals.var_expvgst2_dn12 / assign26370_e23897),)
            } else {
                let assign26370_e23900: f64 = (-87.49823353377374);
                (assign26370_e23900, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        let assign26370_e23902: f64 = (locals.var_noff2 * assign26370_e23901);
        (assign26370_e23902, ((locals.var_noff2_dn3 * assign26370_e23901) + (locals.var_noff2 * assign26370_e23901_d_n3)), ((locals.var_noff2_dn4 * assign26370_e23901) + (locals.var_noff2 * assign26370_e23901_d_n4)), ((locals.var_noff2_dn5 * assign26370_e23901) + (locals.var_noff2 * assign26370_e23901_d_n5)), ((locals.var_noff2_dn6 * assign26370_e23901) + (locals.var_noff2 * assign26370_e23901_d_n6)), ((locals.var_noff2_dn7 * assign26370_e23901) + (locals.var_noff2 * assign26370_e23901_d_n7)), ((locals.var_noff2_dn8 * assign26370_e23901) + (locals.var_noff2 * assign26370_e23901_d_n8)), ((locals.var_noff2_dn9 * assign26370_e23901) + (locals.var_noff2 * assign26370_e23901_d_n9)), ((locals.var_noff2_dn10 * assign26370_e23901) + (locals.var_noff2 * assign26370_e23901_d_n10)), ((locals.var_noff2_dn11 * assign26370_e23901) + (locals.var_noff2 * assign26370_e23901_d_n11)), ((locals.var_noff2_dn12 * assign26370_e23901) + (locals.var_noff2 * assign26370_e23901_d_n12)),)
    } else {
        (locals.var_t1__blk809, locals.var_t1__blk809_dn3, locals.var_t1__blk809_dn4, locals.var_t1__blk809_dn5, locals.var_t1__blk809_dn6, locals.var_t1__blk809_dn7, locals.var_t1__blk809_dn8, locals.var_t1__blk809_dn9, locals.var_t1__blk809_dn10, locals.var_t1__blk809_dn11, locals.var_t1__blk809_dn12,)
    }
};
        locals.var_t1__blk809 = assign26370_e23904;
        locals.var_t1__blk809_dn3 = assign26370_e23904_d_n3;
        locals.var_t1__blk809_dn4 = assign26370_e23904_d_n4;
        locals.var_t1__blk809_dn5 = assign26370_e23904_d_n5;
        locals.var_t1__blk809_dn6 = assign26370_e23904_d_n6;
        locals.var_t1__blk809_dn7 = assign26370_e23904_d_n7;
        locals.var_t1__blk809_dn8 = assign26370_e23904_d_n8;
        locals.var_t1__blk809_dn9 = assign26370_e23904_d_n9;
        locals.var_t1__blk809_dn10 = assign26370_e23904_d_n10;
        locals.var_t1__blk809_dn11 = assign26370_e23904_d_n11;
        locals.var_t1__blk809_dn12 = assign26370_e23904_d_n12;

        let (assign26380_e23932, assign26380_e23932_d_n3, assign26380_e23932_d_n4, assign26380_e23932_d_n5, assign26380_e23932_d_n6, assign26380_e23932_d_n7, assign26380_e23932_d_n8, assign26380_e23932_d_n9, assign26380_e23932_d_n10, assign26380_e23932_d_n11, assign26380_e23932_d_n12,) = {
    if (((((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1364 != 0.0)) && (locals.var_guard1365 == 0.0)) && (locals.var_guard1366 == 0.0)) {
        let assign26380_e23919: f64 = (-locals.var_b4soicox);
        let assign26380_e23922: f64 = (locals.var_vtm * locals.var_cdep0);
        let assign26380_e23923: f64 = (assign26380_e23919 / assign26380_e23922);
        let assign26380_e23925: f64 = (locals.var_exparg2).exp();
        let assign26380_e23926: f64 = (assign26380_e23923 * assign26380_e23925);
        let assign26380_e23929: f64 = (1.0 - locals.var_pparam_b4soimstarcv);
        let assign26380_e23930: f64 = (assign26380_e23926 * assign26380_e23929);
        (assign26380_e23930, (((((-((assign26380_e23919 * (locals.var_vtm * locals.var_cdep0_dn3)) / (assign26380_e23922 * assign26380_e23922))) * assign26380_e23925) + (assign26380_e23923 * (assign26380_e23925 * locals.var_exparg2_dn3))) * assign26380_e23929) + (assign26380_e23926 * (-locals.var_pparam_b4soimstarcv_dn3))), (((((-((assign26380_e23919 * ((locals.var_vtm_dn4 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn4))) / (assign26380_e23922 * assign26380_e23922))) * assign26380_e23925) + (assign26380_e23923 * (assign26380_e23925 * locals.var_exparg2_dn4))) * assign26380_e23929) + (assign26380_e23926 * (-locals.var_pparam_b4soimstarcv_dn4))), (((((-((assign26380_e23919 * ((locals.var_vtm_dn5 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn5))) / (assign26380_e23922 * assign26380_e23922))) * assign26380_e23925) + (assign26380_e23923 * (assign26380_e23925 * locals.var_exparg2_dn5))) * assign26380_e23929) + (assign26380_e23926 * (-locals.var_pparam_b4soimstarcv_dn5))), (((((-((assign26380_e23919 * ((locals.var_vtm_dn6 * locals.var_cdep0) + (locals.var_vtm * locals.var_cdep0_dn6))) / (assign26380_e23922 * assign26380_e23922))) * assign26380_e23925) + (assign26380_e23923 * (assign26380_e23925 * locals.var_exparg2_dn6))) * assign26380_e23929) + (assign26380_e23926 * (-locals.var_pparam_b4soimstarcv_dn6))), (((((-((assign26380_e23919 * (locals.var_vtm * locals.var_cdep0_dn7)) / (assign26380_e23922 * assign26380_e23922))) * assign26380_e23925) + (assign26380_e23923 * (assign26380_e23925 * locals.var_exparg2_dn7))) * assign26380_e23929) + (assign26380_e23926 * (-locals.var_pparam_b4soimstarcv_dn7))), (((((-((assign26380_e23919 * (locals.var_vtm * locals.var_cdep0_dn8)) / (assign26380_e23922 * assign26380_e23922))) * assign26380_e23925) + (assign26380_e23923 * (assign26380_e23925 * locals.var_exparg2_dn8))) * assign26380_e23929) + (assign26380_e23926 * (-locals.var_pparam_b4soimstarcv_dn8))), (((((-((assign26380_e23919 * (locals.var_vtm * locals.var_cdep0_dn9)) / (assign26380_e23922 * assign26380_e23922))) * assign26380_e23925) + (assign26380_e23923 * (assign26380_e23925 * locals.var_exparg2_dn9))) * assign26380_e23929) + (assign26380_e23926 * (-locals.var_pparam_b4soimstarcv_dn9))), (((((-((assign26380_e23919 * (locals.var_vtm * locals.var_cdep0_dn10)) / (assign26380_e23922 * assign26380_e23922))) * assign26380_e23925) + (assign26380_e23923 * (assign26380_e23925 * locals.var_exparg2_dn10))) * assign26380_e23929) + (assign26380_e23926 * (-locals.var_pparam_b4soimstarcv_dn10))), (((((-((assign26380_e23919 * (locals.var_vtm * locals.var_cdep0_dn11)) / (assign26380_e23922 * assign26380_e23922))) * assign26380_e23925) + (assign26380_e23923 * (assign26380_e23925 * locals.var_exparg2_dn11))) * assign26380_e23929) + (assign26380_e23926 * (-locals.var_pparam_b4soimstarcv_dn11))), (((((-((assign26380_e23919 * (locals.var_vtm * locals.var_cdep0_dn12)) / (assign26380_e23922 * assign26380_e23922))) * assign26380_e23925) + (assign26380_e23923 * (assign26380_e23925 * locals.var_exparg2_dn12))) * assign26380_e23929) + (assign26380_e23926 * (-locals.var_pparam_b4soimstarcv_dn12))),)
    } else {
        (locals.var_dt2_dvg, locals.var_dt2_dvg_dn3, locals.var_dt2_dvg_dn4, locals.var_dt2_dvg_dn5, locals.var_dt2_dvg_dn6, locals.var_dt2_dvg_dn7, locals.var_dt2_dvg_dn8, locals.var_dt2_dvg_dn9, locals.var_dt2_dvg_dn10, locals.var_dt2_dvg_dn11, locals.var_dt2_dvg_dn12,)
    }
};
        locals.var_dt2_dvg = assign26380_e23932;
        locals.var_dt2_dvg_dn3 = assign26380_e23932_d_n3;
        locals.var_dt2_dvg_dn4 = assign26380_e23932_d_n4;
        locals.var_dt2_dvg_dn5 = assign26380_e23932_d_n5;
        locals.var_dt2_dvg_dn6 = assign26380_e23932_d_n6;
        locals.var_dt2_dvg_dn7 = assign26380_e23932_d_n7;
        locals.var_dt2_dvg_dn8 = assign26380_e23932_d_n8;
        locals.var_dt2_dvg_dn9 = assign26380_e23932_d_n9;
        locals.var_dt2_dvg_dn10 = assign26380_e23932_d_n10;
        locals.var_dt2_dvg_dn11 = assign26380_e23932_d_n11;
        locals.var_dt2_dvg_dn12 = assign26380_e23932_d_n12;

        let (assign26390_e23956, assign26390_e23956_d_n3, assign26390_e23956_d_n4, assign26390_e23956_d_n5, assign26390_e23956_d_n6, assign26390_e23956_d_n7, assign26390_e23956_d_n8, assign26390_e23956_d_n9, assign26390_e23956_d_n10, assign26390_e23956_d_n11, assign26390_e23956_d_n12,) = {
    if (((((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1364 != 0.0)) && (locals.var_guard1365 == 0.0)) && (locals.var_guard1366 == 0.0)) {
        let assign26390_e23949: f64 = (locals.var_noff2 * locals.var_dt2_dvg);
        let assign26390_e23952: f64 = (1.0 - locals.var_pparam_b4soimstarcv);
        let assign26390_e23953: f64 = (assign26390_e23949 / assign26390_e23952);
        let assign26390_e23954: f64 = (locals.var_pparam_b4soimstarcv - assign26390_e23953);
        (assign26390_e23954, (locals.var_pparam_b4soimstarcv_dn3 - (((((locals.var_noff2_dn3 * locals.var_dt2_dvg) + (locals.var_noff2 * locals.var_dt2_dvg_dn3)) * assign26390_e23952) - (assign26390_e23949 * (-locals.var_pparam_b4soimstarcv_dn3))) / (assign26390_e23952 * assign26390_e23952))), (locals.var_pparam_b4soimstarcv_dn4 - (((((locals.var_noff2_dn4 * locals.var_dt2_dvg) + (locals.var_noff2 * locals.var_dt2_dvg_dn4)) * assign26390_e23952) - (assign26390_e23949 * (-locals.var_pparam_b4soimstarcv_dn4))) / (assign26390_e23952 * assign26390_e23952))), (locals.var_pparam_b4soimstarcv_dn5 - (((((locals.var_noff2_dn5 * locals.var_dt2_dvg) + (locals.var_noff2 * locals.var_dt2_dvg_dn5)) * assign26390_e23952) - (assign26390_e23949 * (-locals.var_pparam_b4soimstarcv_dn5))) / (assign26390_e23952 * assign26390_e23952))), (locals.var_pparam_b4soimstarcv_dn6 - (((((locals.var_noff2_dn6 * locals.var_dt2_dvg) + (locals.var_noff2 * locals.var_dt2_dvg_dn6)) * assign26390_e23952) - (assign26390_e23949 * (-locals.var_pparam_b4soimstarcv_dn6))) / (assign26390_e23952 * assign26390_e23952))), (locals.var_pparam_b4soimstarcv_dn7 - (((((locals.var_noff2_dn7 * locals.var_dt2_dvg) + (locals.var_noff2 * locals.var_dt2_dvg_dn7)) * assign26390_e23952) - (assign26390_e23949 * (-locals.var_pparam_b4soimstarcv_dn7))) / (assign26390_e23952 * assign26390_e23952))), (locals.var_pparam_b4soimstarcv_dn8 - (((((locals.var_noff2_dn8 * locals.var_dt2_dvg) + (locals.var_noff2 * locals.var_dt2_dvg_dn8)) * assign26390_e23952) - (assign26390_e23949 * (-locals.var_pparam_b4soimstarcv_dn8))) / (assign26390_e23952 * assign26390_e23952))), (locals.var_pparam_b4soimstarcv_dn9 - (((((locals.var_noff2_dn9 * locals.var_dt2_dvg) + (locals.var_noff2 * locals.var_dt2_dvg_dn9)) * assign26390_e23952) - (assign26390_e23949 * (-locals.var_pparam_b4soimstarcv_dn9))) / (assign26390_e23952 * assign26390_e23952))), (locals.var_pparam_b4soimstarcv_dn10 - (((((locals.var_noff2_dn10 * locals.var_dt2_dvg) + (locals.var_noff2 * locals.var_dt2_dvg_dn10)) * assign26390_e23952) - (assign26390_e23949 * (-locals.var_pparam_b4soimstarcv_dn10))) / (assign26390_e23952 * assign26390_e23952))), (locals.var_pparam_b4soimstarcv_dn11 - (((((locals.var_noff2_dn11 * locals.var_dt2_dvg) + (locals.var_noff2 * locals.var_dt2_dvg_dn11)) * assign26390_e23952) - (assign26390_e23949 * (-locals.var_pparam_b4soimstarcv_dn11))) / (assign26390_e23952 * assign26390_e23952))), (locals.var_pparam_b4soimstarcv_dn12 - (((((locals.var_noff2_dn12 * locals.var_dt2_dvg) + (locals.var_noff2 * locals.var_dt2_dvg_dn12)) * assign26390_e23952) - (assign26390_e23949 * (-locals.var_pparam_b4soimstarcv_dn12))) / (assign26390_e23952 * assign26390_e23952))),)
    } else {
        (locals.var_t2__blk810, locals.var_t2__blk810_dn3, locals.var_t2__blk810_dn4, locals.var_t2__blk810_dn5, locals.var_t2__blk810_dn6, locals.var_t2__blk810_dn7, locals.var_t2__blk810_dn8, locals.var_t2__blk810_dn9, locals.var_t2__blk810_dn10, locals.var_t2__blk810_dn11, locals.var_t2__blk810_dn12,)
    }
};
        locals.var_t2__blk810 = assign26390_e23956;
        locals.var_t2__blk810_dn3 = assign26390_e23956_d_n3;
        locals.var_t2__blk810_dn4 = assign26390_e23956_d_n4;
        locals.var_t2__blk810_dn5 = assign26390_e23956_d_n5;
        locals.var_t2__blk810_dn6 = assign26390_e23956_d_n6;
        locals.var_t2__blk810_dn7 = assign26390_e23956_d_n7;
        locals.var_t2__blk810_dn8 = assign26390_e23956_d_n8;
        locals.var_t2__blk810_dn9 = assign26390_e23956_d_n9;
        locals.var_t2__blk810_dn10 = assign26390_e23956_d_n10;
        locals.var_t2__blk810_dn11 = assign26390_e23956_d_n11;
        locals.var_t2__blk810_dn12 = assign26390_e23956_d_n12;

        let (assign26400_e23974, assign26400_e23974_d_n3, assign26400_e23974_d_n4, assign26400_e23974_d_n5, assign26400_e23974_d_n6, assign26400_e23974_d_n7, assign26400_e23974_d_n8, assign26400_e23974_d_n9, assign26400_e23974_d_n10, assign26400_e23974_d_n11, assign26400_e23974_d_n12,) = {
    if (((((locals.var_guard1356 == 0.0) && (locals.var_guard1359 == 0.0)) && (locals.var_guard1364 != 0.0)) && (locals.var_guard1365 == 0.0)) && (locals.var_guard1366 == 0.0)) {
        let assign26400_e23972: f64 = (locals.var_t1__blk809 / locals.var_t2__blk810);
        (assign26400_e23972, (((locals.var_t1__blk809_dn3 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn3)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn4 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn4)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn5 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn5)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn6 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn6)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn7 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn7)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn8 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn8)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn9 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn9)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn10 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn10)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn11 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn11)) / (locals.var_t2__blk810 * locals.var_t2__blk810)), (((locals.var_t1__blk809_dn12 * locals.var_t2__blk810) - (locals.var_t1__blk809 * locals.var_t2__blk810_dn12)) / (locals.var_t2__blk810 * locals.var_t2__blk810)),)
    } else {
        (locals.var_vgsteff2, locals.var_vgsteff2_dn3, locals.var_vgsteff2_dn4, locals.var_vgsteff2_dn5, locals.var_vgsteff2_dn6, locals.var_vgsteff2_dn7, locals.var_vgsteff2_dn8, locals.var_vgsteff2_dn9, locals.var_vgsteff2_dn10, locals.var_vgsteff2_dn11, locals.var_vgsteff2_dn12,)
    }
};
        locals.var_vgsteff2 = assign26400_e23974;
        locals.var_vgsteff2_dn3 = assign26400_e23974_d_n3;
        locals.var_vgsteff2_dn4 = assign26400_e23974_d_n4;
        locals.var_vgsteff2_dn5 = assign26400_e23974_d_n5;
        locals.var_vgsteff2_dn6 = assign26400_e23974_d_n6;
        locals.var_vgsteff2_dn7 = assign26400_e23974_d_n7;
        locals.var_vgsteff2_dn8 = assign26400_e23974_d_n8;
        locals.var_vgsteff2_dn9 = assign26400_e23974_d_n9;
        locals.var_vgsteff2_dn10 = assign26400_e23974_d_n10;
        locals.var_vgsteff2_dn11 = assign26400_e23974_d_n11;
        locals.var_vgsteff2_dn12 = assign26400_e23974_d_n12;

        locals.var_vth__blk794 = locals.var_vth_cv;
        locals.var_vth__blk794_dn3 = locals.var_vth_cv_dn3;
        locals.var_vth__blk794_dn4 = locals.var_vth_cv_dn4;
        locals.var_vth__blk794_dn5 = locals.var_vth_cv_dn5;
        locals.var_vth__blk794_dn6 = locals.var_vth_cv_dn6;
        locals.var_vth__blk794_dn7 = locals.var_vth_cv_dn7;
        locals.var_vth__blk794_dn8 = locals.var_vth_cv_dn8;
        locals.var_vth__blk794_dn9 = locals.var_vth_cv_dn9;
        locals.var_vth__blk794_dn10 = locals.var_vth_cv_dn10;
        locals.var_vth__blk794_dn11 = locals.var_vth_cv_dn11;
        locals.var_vth__blk794_dn12 = locals.var_vth_cv_dn12;

        locals.var_sqrtphis = locals.var_sqrtphis_cv;
        locals.var_sqrtphis_dn3 = locals.var_sqrtphis_cv_dn3;
        locals.var_sqrtphis_dn4 = locals.var_sqrtphis_cv_dn4;
        locals.var_sqrtphis_dn5 = locals.var_sqrtphis_cv_dn5;
        locals.var_sqrtphis_dn6 = locals.var_sqrtphis_cv_dn6;
        locals.var_sqrtphis_dn7 = locals.var_sqrtphis_cv_dn7;
        locals.var_sqrtphis_dn8 = locals.var_sqrtphis_cv_dn8;
        locals.var_sqrtphis_dn9 = locals.var_sqrtphis_cv_dn9;
        locals.var_sqrtphis_dn10 = locals.var_sqrtphis_cv_dn10;
        locals.var_sqrtphis_dn11 = locals.var_sqrtphis_cv_dn11;
        locals.var_sqrtphis_dn12 = locals.var_sqrtphis_cv_dn12;

        locals.var_vbseff = locals.var_vbseff_cv;
        locals.var_vbseff_dn3 = locals.var_vbseff_cv_dn3;
        locals.var_vbseff_dn4 = locals.var_vbseff_cv_dn4;
        locals.var_vbseff_dn5 = locals.var_vbseff_cv_dn5;
        locals.var_vbseff_dn6 = locals.var_vbseff_cv_dn6;
        locals.var_vbseff_dn7 = locals.var_vbseff_cv_dn7;
        locals.var_vbseff_dn8 = locals.var_vbseff_cv_dn8;
        locals.var_vbseff_dn9 = locals.var_vbseff_cv_dn9;
        locals.var_vbseff_dn10 = locals.var_vbseff_cv_dn10;
        locals.var_vbseff_dn11 = locals.var_vbseff_cv_dn11;
        locals.var_vbseff_dn12 = locals.var_vbseff_cv_dn12;

        let assign26440_e23980: f64 = if p.p61 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1367 = assign26440_e23980;

        let assign26450_e23983: f64 = if locals.var_b4soisoimod == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard1368 = assign26450_e23983;

        let (assign26460_e23989, assign26460_e23989_d_n3, assign26460_e23989_d_n4, assign26460_e23989_d_n5, assign26460_e23989_d_n6, assign26460_e23989_d_n7, assign26460_e23989_d_n8, assign26460_e23989_d_n9, assign26460_e23989_d_n10, assign26460_e23989_d_n11, assign26460_e23989_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1368 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qac0, locals.var_qac0_dn3, locals.var_qac0_dn4, locals.var_qac0_dn5, locals.var_qac0_dn6, locals.var_qac0_dn7, locals.var_qac0_dn8, locals.var_qac0_dn9, locals.var_qac0_dn10, locals.var_qac0_dn11, locals.var_qac0_dn12,)
    }
};
        locals.var_qac0 = assign26460_e23989;
        locals.var_qac0_dn3 = assign26460_e23989_d_n3;
        locals.var_qac0_dn4 = assign26460_e23989_d_n4;
        locals.var_qac0_dn5 = assign26460_e23989_d_n5;
        locals.var_qac0_dn6 = assign26460_e23989_d_n6;
        locals.var_qac0_dn7 = assign26460_e23989_d_n7;
        locals.var_qac0_dn8 = assign26460_e23989_d_n8;
        locals.var_qac0_dn9 = assign26460_e23989_d_n9;
        locals.var_qac0_dn10 = assign26460_e23989_d_n10;
        locals.var_qac0_dn11 = assign26460_e23989_d_n11;
        locals.var_qac0_dn12 = assign26460_e23989_d_n12;

        let (assign26470_e23995, assign26470_e23995_d_n3, assign26470_e23995_d_n4, assign26470_e23995_d_n5, assign26470_e23995_d_n6, assign26470_e23995_d_n7, assign26470_e23995_d_n8, assign26470_e23995_d_n9, assign26470_e23995_d_n10, assign26470_e23995_d_n11, assign26470_e23995_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1368 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qsub0, locals.var_qsub0_dn3, locals.var_qsub0_dn4, locals.var_qsub0_dn5, locals.var_qsub0_dn6, locals.var_qsub0_dn7, locals.var_qsub0_dn8, locals.var_qsub0_dn9, locals.var_qsub0_dn10, locals.var_qsub0_dn11, locals.var_qsub0_dn12,)
    }
};
        locals.var_qsub0 = assign26470_e23995;
        locals.var_qsub0_dn3 = assign26470_e23995_d_n3;
        locals.var_qsub0_dn4 = assign26470_e23995_d_n4;
        locals.var_qsub0_dn5 = assign26470_e23995_d_n5;
        locals.var_qsub0_dn6 = assign26470_e23995_d_n6;
        locals.var_qsub0_dn7 = assign26470_e23995_d_n7;
        locals.var_qsub0_dn8 = assign26470_e23995_d_n8;
        locals.var_qsub0_dn9 = assign26470_e23995_d_n9;
        locals.var_qsub0_dn10 = assign26470_e23995_d_n10;
        locals.var_qsub0_dn11 = assign26470_e23995_d_n11;
        locals.var_qsub0_dn12 = assign26470_e23995_d_n12;

        let (assign26480_e24010, assign26480_e24010_d_n3, assign26480_e24010_d_n4, assign26480_e24010_d_n5, assign26480_e24010_d_n6, assign26480_e24010_d_n7, assign26480_e24010_d_n8, assign26480_e24010_d_n9, assign26480_e24010_d_n10, assign26480_e24010_d_n11, assign26480_e24010_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) {
        let assign26480_e24002: f64 = (locals.var_vth__blk794 - locals.var_phi);
        let assign26480_e24005: f64 = (locals.var_here_b4soik1eff * locals.var_sqrtphis);
        let assign26480_e24006: f64 = (assign26480_e24002 - assign26480_e24005);
        let assign26480_e24008: f64 = (assign26480_e24006 + locals.var_pparam_b4soidelvt);
        (assign26480_e24008, (((locals.var_vth__blk794_dn3 - locals.var_phi_dn3) - ((locals.var_here_b4soik1eff_dn3 * locals.var_sqrtphis) + (locals.var_here_b4soik1eff * locals.var_sqrtphis_dn3))) + locals.var_pparam_b4soidelvt_dn3), (((locals.var_vth__blk794_dn4 - locals.var_phi_dn4) - ((locals.var_here_b4soik1eff_dn4 * locals.var_sqrtphis) + (locals.var_here_b4soik1eff * locals.var_sqrtphis_dn4))) + locals.var_pparam_b4soidelvt_dn4), (((locals.var_vth__blk794_dn5 - locals.var_phi_dn5) - ((locals.var_here_b4soik1eff_dn5 * locals.var_sqrtphis) + (locals.var_here_b4soik1eff * locals.var_sqrtphis_dn5))) + locals.var_pparam_b4soidelvt_dn5), (((locals.var_vth__blk794_dn6 - locals.var_phi_dn6) - ((locals.var_here_b4soik1eff_dn6 * locals.var_sqrtphis) + (locals.var_here_b4soik1eff * locals.var_sqrtphis_dn6))) + locals.var_pparam_b4soidelvt_dn6), (((locals.var_vth__blk794_dn7 - locals.var_phi_dn7) - ((locals.var_here_b4soik1eff_dn7 * locals.var_sqrtphis) + (locals.var_here_b4soik1eff * locals.var_sqrtphis_dn7))) + locals.var_pparam_b4soidelvt_dn7), (((locals.var_vth__blk794_dn8 - locals.var_phi_dn8) - ((locals.var_here_b4soik1eff_dn8 * locals.var_sqrtphis) + (locals.var_here_b4soik1eff * locals.var_sqrtphis_dn8))) + locals.var_pparam_b4soidelvt_dn8), (((locals.var_vth__blk794_dn9 - locals.var_phi_dn9) - ((locals.var_here_b4soik1eff_dn9 * locals.var_sqrtphis) + (locals.var_here_b4soik1eff * locals.var_sqrtphis_dn9))) + locals.var_pparam_b4soidelvt_dn9), (((locals.var_vth__blk794_dn10 - locals.var_phi_dn10) - ((locals.var_here_b4soik1eff_dn10 * locals.var_sqrtphis) + (locals.var_here_b4soik1eff * locals.var_sqrtphis_dn10))) + locals.var_pparam_b4soidelvt_dn10), (((locals.var_vth__blk794_dn11 - locals.var_phi_dn11) - ((locals.var_here_b4soik1eff_dn11 * locals.var_sqrtphis) + (locals.var_here_b4soik1eff * locals.var_sqrtphis_dn11))) + locals.var_pparam_b4soidelvt_dn11), (((locals.var_vth__blk794_dn12 - locals.var_phi_dn12) - ((locals.var_here_b4soik1eff_dn12 * locals.var_sqrtphis) + (locals.var_here_b4soik1eff * locals.var_sqrtphis_dn12))) + locals.var_pparam_b4soidelvt_dn12),)
    } else {
        (locals.var_vfb, locals.var_vfb_dn3, locals.var_vfb_dn4, locals.var_vfb_dn5, locals.var_vfb_dn6, locals.var_vfb_dn7, locals.var_vfb_dn8, locals.var_vfb_dn9, locals.var_vfb_dn10, locals.var_vfb_dn11, locals.var_vfb_dn12,)
    }
};
        locals.var_vfb = assign26480_e24010;
        locals.var_vfb_dn3 = assign26480_e24010_d_n3;
        locals.var_vfb_dn4 = assign26480_e24010_d_n4;
        locals.var_vfb_dn5 = assign26480_e24010_d_n5;
        locals.var_vfb_dn6 = assign26480_e24010_d_n6;
        locals.var_vfb_dn7 = assign26480_e24010_d_n7;
        locals.var_vfb_dn8 = assign26480_e24010_d_n8;
        locals.var_vfb_dn9 = assign26480_e24010_d_n9;
        locals.var_vfb_dn10 = assign26480_e24010_d_n10;
        locals.var_vfb_dn11 = assign26480_e24010_d_n11;
        locals.var_vfb_dn12 = assign26480_e24010_d_n12;

        let (assign26490_e24023, assign26490_e24023_d_n3, assign26490_e24023_d_n4, assign26490_e24023_d_n5, assign26490_e24023_d_n6, assign26490_e24023_d_n7, assign26490_e24023_d_n8, assign26490_e24023_d_n9, assign26490_e24023_d_n10, assign26490_e24023_d_n11, assign26490_e24023_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) {
        let assign26490_e24017: f64 = (locals.var_vfb - locals.var_vgs_eff__blk790);
        let assign26490_e24019: f64 = (assign26490_e24017 + locals.var_vbseff);
        let assign26490_e24021: f64 = (assign26490_e24019 - 0.08);
        (assign26490_e24021, ((locals.var_vfb_dn3 - locals.var_vgs_eff__blk790_dn3) + locals.var_vbseff_dn3), ((locals.var_vfb_dn4 - locals.var_vgs_eff__blk790_dn4) + locals.var_vbseff_dn4), ((locals.var_vfb_dn5 - locals.var_vgs_eff__blk790_dn5) + locals.var_vbseff_dn5), ((locals.var_vfb_dn6 - locals.var_vgs_eff__blk790_dn6) + locals.var_vbseff_dn6), ((locals.var_vfb_dn7 - locals.var_vgs_eff__blk790_dn7) + locals.var_vbseff_dn7), ((locals.var_vfb_dn8 - locals.var_vgs_eff__blk790_dn8) + locals.var_vbseff_dn8), ((locals.var_vfb_dn9 - locals.var_vgs_eff__blk790_dn9) + locals.var_vbseff_dn9), ((locals.var_vfb_dn10 - locals.var_vgs_eff__blk790_dn10) + locals.var_vbseff_dn10), ((locals.var_vfb_dn11 - locals.var_vgs_eff__blk790_dn11) + locals.var_vbseff_dn11), ((locals.var_vfb_dn12 - locals.var_vgs_eff__blk790_dn12) + locals.var_vbseff_dn12),)
    } else {
        (locals.var_v3, locals.var_v3_dn3, locals.var_v3_dn4, locals.var_v3_dn5, locals.var_v3_dn6, locals.var_v3_dn7, locals.var_v3_dn8, locals.var_v3_dn9, locals.var_v3_dn10, locals.var_v3_dn11, locals.var_v3_dn12,)
    }
};
        locals.var_v3 = assign26490_e24023;
        locals.var_v3_dn3 = assign26490_e24023_d_n3;
        locals.var_v3_dn4 = assign26490_e24023_d_n4;
        locals.var_v3_dn5 = assign26490_e24023_d_n5;
        locals.var_v3_dn6 = assign26490_e24023_d_n6;
        locals.var_v3_dn7 = assign26490_e24023_d_n7;
        locals.var_v3_dn8 = assign26490_e24023_d_n8;
        locals.var_v3_dn9 = assign26490_e24023_d_n9;
        locals.var_v3_dn10 = assign26490_e24023_d_n10;
        locals.var_v3_dn11 = assign26490_e24023_d_n11;
        locals.var_v3_dn12 = assign26490_e24023_d_n12;

        let assign26500_e24026: f64 = if locals.var_vfb <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1369 = assign26500_e24026;

        let (assign26510_e24044, assign26510_e24044_d_n3, assign26510_e24044_d_n4, assign26510_e24044_d_n5, assign26510_e24044_d_n6, assign26510_e24044_d_n7, assign26510_e24044_d_n8, assign26510_e24044_d_n9, assign26510_e24044_d_n10, assign26510_e24044_d_n11, assign26510_e24044_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 != 0.0)) {
        let assign26510_e24035: f64 = (locals.var_v3 * locals.var_v3);
        let assign26510_e24038: f64 = (4.0 * 0.08);
        let assign26510_e24040: f64 = (assign26510_e24038 * locals.var_vfb);
        let assign26510_e24041: f64 = (assign26510_e24035 - assign26510_e24040);
        let assign26510_e24042: f64 = (assign26510_e24041).sqrt();
        (assign26510_e24042, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) - (assign26510_e24038 * locals.var_vfb_dn3)) / (2.0 * assign26510_e24042)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) - (assign26510_e24038 * locals.var_vfb_dn4)) / (2.0 * assign26510_e24042)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) - (assign26510_e24038 * locals.var_vfb_dn5)) / (2.0 * assign26510_e24042)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) - (assign26510_e24038 * locals.var_vfb_dn6)) / (2.0 * assign26510_e24042)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) - (assign26510_e24038 * locals.var_vfb_dn7)) / (2.0 * assign26510_e24042)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) - (assign26510_e24038 * locals.var_vfb_dn8)) / (2.0 * assign26510_e24042)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) - (assign26510_e24038 * locals.var_vfb_dn9)) / (2.0 * assign26510_e24042)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) - (assign26510_e24038 * locals.var_vfb_dn10)) / (2.0 * assign26510_e24042)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) - (assign26510_e24038 * locals.var_vfb_dn11)) / (2.0 * assign26510_e24042)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) - (assign26510_e24038 * locals.var_vfb_dn12)) / (2.0 * assign26510_e24042)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign26510_e24044;
        locals.var_t0__blk808_dn3 = assign26510_e24044_d_n3;
        locals.var_t0__blk808_dn4 = assign26510_e24044_d_n4;
        locals.var_t0__blk808_dn5 = assign26510_e24044_d_n5;
        locals.var_t0__blk808_dn6 = assign26510_e24044_d_n6;
        locals.var_t0__blk808_dn7 = assign26510_e24044_d_n7;
        locals.var_t0__blk808_dn8 = assign26510_e24044_d_n8;
        locals.var_t0__blk808_dn9 = assign26510_e24044_d_n9;
        locals.var_t0__blk808_dn10 = assign26510_e24044_d_n10;
        locals.var_t0__blk808_dn11 = assign26510_e24044_d_n11;
        locals.var_t0__blk808_dn12 = assign26510_e24044_d_n12;

        let (assign26520_e24063, assign26520_e24063_d_n3, assign26520_e24063_d_n4, assign26520_e24063_d_n5, assign26520_e24063_d_n6, assign26520_e24063_d_n7, assign26520_e24063_d_n8, assign26520_e24063_d_n9, assign26520_e24063_d_n10, assign26520_e24063_d_n11, assign26520_e24063_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1369 == 0.0)) {
        let assign26520_e24054: f64 = (locals.var_v3 * locals.var_v3);
        let assign26520_e24057: f64 = (4.0 * 0.08);
        let assign26520_e24059: f64 = (assign26520_e24057 * locals.var_vfb);
        let assign26520_e24060: f64 = (assign26520_e24054 + assign26520_e24059);
        let assign26520_e24061: f64 = (assign26520_e24060).sqrt();
        (assign26520_e24061, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) + (assign26520_e24057 * locals.var_vfb_dn3)) / (2.0 * assign26520_e24061)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) + (assign26520_e24057 * locals.var_vfb_dn4)) / (2.0 * assign26520_e24061)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) + (assign26520_e24057 * locals.var_vfb_dn5)) / (2.0 * assign26520_e24061)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) + (assign26520_e24057 * locals.var_vfb_dn6)) / (2.0 * assign26520_e24061)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) + (assign26520_e24057 * locals.var_vfb_dn7)) / (2.0 * assign26520_e24061)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) + (assign26520_e24057 * locals.var_vfb_dn8)) / (2.0 * assign26520_e24061)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) + (assign26520_e24057 * locals.var_vfb_dn9)) / (2.0 * assign26520_e24061)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) + (assign26520_e24057 * locals.var_vfb_dn10)) / (2.0 * assign26520_e24061)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) + (assign26520_e24057 * locals.var_vfb_dn11)) / (2.0 * assign26520_e24061)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) + (assign26520_e24057 * locals.var_vfb_dn12)) / (2.0 * assign26520_e24061)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign26520_e24063;
        locals.var_t0__blk808_dn3 = assign26520_e24063_d_n3;
        locals.var_t0__blk808_dn4 = assign26520_e24063_d_n4;
        locals.var_t0__blk808_dn5 = assign26520_e24063_d_n5;
        locals.var_t0__blk808_dn6 = assign26520_e24063_d_n6;
        locals.var_t0__blk808_dn7 = assign26520_e24063_d_n7;
        locals.var_t0__blk808_dn8 = assign26520_e24063_d_n8;
        locals.var_t0__blk808_dn9 = assign26520_e24063_d_n9;
        locals.var_t0__blk808_dn10 = assign26520_e24063_d_n10;
        locals.var_t0__blk808_dn11 = assign26520_e24063_d_n11;
        locals.var_t0__blk808_dn12 = assign26520_e24063_d_n12;

        let (assign26530_e24076, assign26530_e24076_d_n3, assign26530_e24076_d_n4, assign26530_e24076_d_n5, assign26530_e24076_d_n6, assign26530_e24076_d_n7, assign26530_e24076_d_n8, assign26530_e24076_d_n9, assign26530_e24076_d_n10, assign26530_e24076_d_n11, assign26530_e24076_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) {
        let assign26530_e24072: f64 = (locals.var_v3 + locals.var_t0__blk808);
        let assign26530_e24073: f64 = (0.5 * assign26530_e24072);
        let assign26530_e24074: f64 = (locals.var_vfb - assign26530_e24073);
        (assign26530_e24074, (locals.var_vfb_dn3 - (0.5 * (locals.var_v3_dn3 + locals.var_t0__blk808_dn3))), (locals.var_vfb_dn4 - (0.5 * (locals.var_v3_dn4 + locals.var_t0__blk808_dn4))), (locals.var_vfb_dn5 - (0.5 * (locals.var_v3_dn5 + locals.var_t0__blk808_dn5))), (locals.var_vfb_dn6 - (0.5 * (locals.var_v3_dn6 + locals.var_t0__blk808_dn6))), (locals.var_vfb_dn7 - (0.5 * (locals.var_v3_dn7 + locals.var_t0__blk808_dn7))), (locals.var_vfb_dn8 - (0.5 * (locals.var_v3_dn8 + locals.var_t0__blk808_dn8))), (locals.var_vfb_dn9 - (0.5 * (locals.var_v3_dn9 + locals.var_t0__blk808_dn9))), (locals.var_vfb_dn10 - (0.5 * (locals.var_v3_dn10 + locals.var_t0__blk808_dn10))), (locals.var_vfb_dn11 - (0.5 * (locals.var_v3_dn11 + locals.var_t0__blk808_dn11))), (locals.var_vfb_dn12 - (0.5 * (locals.var_v3_dn12 + locals.var_t0__blk808_dn12))),)
    } else {
        (locals.var_vfbeff, locals.var_vfbeff_dn3, locals.var_vfbeff_dn4, locals.var_vfbeff_dn5, locals.var_vfbeff_dn6, locals.var_vfbeff_dn7, locals.var_vfbeff_dn8, locals.var_vfbeff_dn9, locals.var_vfbeff_dn10, locals.var_vfbeff_dn11, locals.var_vfbeff_dn12,)
    }
};
        locals.var_vfbeff = assign26530_e24076;
        locals.var_vfbeff_dn3 = assign26530_e24076_d_n3;
        locals.var_vfbeff_dn4 = assign26530_e24076_d_n4;
        locals.var_vfbeff_dn5 = assign26530_e24076_d_n5;
        locals.var_vfbeff_dn6 = assign26530_e24076_d_n6;
        locals.var_vfbeff_dn7 = assign26530_e24076_d_n7;
        locals.var_vfbeff_dn8 = assign26530_e24076_d_n8;
        locals.var_vfbeff_dn9 = assign26530_e24076_d_n9;
        locals.var_vfbeff_dn10 = assign26530_e24076_d_n10;
        locals.var_vfbeff_dn11 = assign26530_e24076_d_n11;
        locals.var_vfbeff_dn12 = assign26530_e24076_d_n12;

        let (assign26540_e24087, assign26540_e24087_d_n3, assign26540_e24087_d_n4, assign26540_e24087_d_n5, assign26540_e24087_d_n6, assign26540_e24087_d_n7, assign26540_e24087_d_n8, assign26540_e24087_d_n9, assign26540_e24087_d_n10, assign26540_e24087_d_n11, assign26540_e24087_d_n12,) = {
    if ((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) {
        let assign26540_e24084: f64 = (locals.var_vfbeff - locals.var_vfb);
        let assign26540_e24085: f64 = (locals.var_coxwlb * assign26540_e24084);
        (assign26540_e24085, ((locals.var_coxwlb_dn3 * assign26540_e24084) + (locals.var_coxwlb * (locals.var_vfbeff_dn3 - locals.var_vfb_dn3))), ((locals.var_coxwlb_dn4 * assign26540_e24084) + (locals.var_coxwlb * (locals.var_vfbeff_dn4 - locals.var_vfb_dn4))), ((locals.var_coxwlb_dn5 * assign26540_e24084) + (locals.var_coxwlb * (locals.var_vfbeff_dn5 - locals.var_vfb_dn5))), ((locals.var_coxwlb_dn6 * assign26540_e24084) + (locals.var_coxwlb * (locals.var_vfbeff_dn6 - locals.var_vfb_dn6))), ((locals.var_coxwlb_dn7 * assign26540_e24084) + (locals.var_coxwlb * (locals.var_vfbeff_dn7 - locals.var_vfb_dn7))), ((locals.var_coxwlb_dn8 * assign26540_e24084) + (locals.var_coxwlb * (locals.var_vfbeff_dn8 - locals.var_vfb_dn8))), ((locals.var_coxwlb_dn9 * assign26540_e24084) + (locals.var_coxwlb * (locals.var_vfbeff_dn9 - locals.var_vfb_dn9))), ((locals.var_coxwlb_dn10 * assign26540_e24084) + (locals.var_coxwlb * (locals.var_vfbeff_dn10 - locals.var_vfb_dn10))), ((locals.var_coxwlb_dn11 * assign26540_e24084) + (locals.var_coxwlb * (locals.var_vfbeff_dn11 - locals.var_vfb_dn11))), ((locals.var_coxwlb_dn12 * assign26540_e24084) + (locals.var_coxwlb * (locals.var_vfbeff_dn12 - locals.var_vfb_dn12))),)
    } else {
        (locals.var_qac0, locals.var_qac0_dn3, locals.var_qac0_dn4, locals.var_qac0_dn5, locals.var_qac0_dn6, locals.var_qac0_dn7, locals.var_qac0_dn8, locals.var_qac0_dn9, locals.var_qac0_dn10, locals.var_qac0_dn11, locals.var_qac0_dn12,)
    }
};
        locals.var_qac0 = assign26540_e24087;
        locals.var_qac0_dn3 = assign26540_e24087_d_n3;
        locals.var_qac0_dn4 = assign26540_e24087_d_n4;
        locals.var_qac0_dn5 = assign26540_e24087_d_n5;
        locals.var_qac0_dn6 = assign26540_e24087_d_n6;
        locals.var_qac0_dn7 = assign26540_e24087_d_n7;
        locals.var_qac0_dn8 = assign26540_e24087_d_n8;
        locals.var_qac0_dn9 = assign26540_e24087_d_n9;
        locals.var_qac0_dn10 = assign26540_e24087_d_n10;
        locals.var_qac0_dn11 = assign26540_e24087_d_n11;
        locals.var_qac0_dn12 = assign26540_e24087_d_n12;

        let assign26550_e24098: f64 = if (((locals.var_b4soisoimod != 2.0) && (locals.var_b4soibodymod != 0.0)) && (p.p27 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1370 = assign26550_e24098;

        let (assign26560_e24109, assign26560_e24109_d_n3, assign26560_e24109_d_n4, assign26560_e24109_d_n5, assign26560_e24109_d_n6, assign26560_e24109_d_n7, assign26560_e24109_d_n8, assign26560_e24109_d_n9, assign26560_e24109_d_n10, assign26560_e24109_d_n11, assign26560_e24109_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1370 != 0.0)) {
        let assign26560_e24107: f64 = (locals.var_vfb + p.p1033);
        (assign26560_e24107, locals.var_vfb_dn3, locals.var_vfb_dn4, locals.var_vfb_dn5, locals.var_vfb_dn6, locals.var_vfb_dn7, locals.var_vfb_dn8, locals.var_vfb_dn9, locals.var_vfb_dn10, locals.var_vfb_dn11, locals.var_vfb_dn12,)
    } else {
        (locals.var_vfb2, locals.var_vfb2_dn3, locals.var_vfb2_dn4, locals.var_vfb2_dn5, locals.var_vfb2_dn6, locals.var_vfb2_dn7, locals.var_vfb2_dn8, locals.var_vfb2_dn9, locals.var_vfb2_dn10, locals.var_vfb2_dn11, locals.var_vfb2_dn12,)
    }
};
        locals.var_vfb2 = assign26560_e24109;
        locals.var_vfb2_dn3 = assign26560_e24109_d_n3;
        locals.var_vfb2_dn4 = assign26560_e24109_d_n4;
        locals.var_vfb2_dn5 = assign26560_e24109_d_n5;
        locals.var_vfb2_dn6 = assign26560_e24109_d_n6;
        locals.var_vfb2_dn7 = assign26560_e24109_d_n7;
        locals.var_vfb2_dn8 = assign26560_e24109_d_n8;
        locals.var_vfb2_dn9 = assign26560_e24109_d_n9;
        locals.var_vfb2_dn10 = assign26560_e24109_d_n10;
        locals.var_vfb2_dn11 = assign26560_e24109_d_n11;
        locals.var_vfb2_dn12 = assign26560_e24109_d_n12;

        let (assign26570_e24118,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1370 != 0.0)) {
        (0.08,)
    } else {
        (locals.var_delta_3_soi2,)
    }
};
        locals.var_delta_3_soi2 = assign26570_e24118;

        let (assign26580_e24133, assign26580_e24133_d_n3, assign26580_e24133_d_n4, assign26580_e24133_d_n5, assign26580_e24133_d_n6, assign26580_e24133_d_n7, assign26580_e24133_d_n8, assign26580_e24133_d_n9, assign26580_e24133_d_n10, assign26580_e24133_d_n11, assign26580_e24133_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1370 != 0.0)) {
        let assign26580_e24127: f64 = (locals.var_vfb2 - locals.var_vgs_eff2);
        let assign26580_e24129: f64 = (assign26580_e24127 + locals.var_vbseff);
        let assign26580_e24131: f64 = (assign26580_e24129 - locals.var_delta_3_soi2);
        (assign26580_e24131, (locals.var_vfb2_dn3 + locals.var_vbseff_dn3), (locals.var_vfb2_dn4 + locals.var_vbseff_dn4), (locals.var_vfb2_dn5 + locals.var_vbseff_dn5), (locals.var_vfb2_dn6 + locals.var_vbseff_dn6), ((locals.var_vfb2_dn7 - locals.var_vgs_eff2_dn7) + locals.var_vbseff_dn7), ((locals.var_vfb2_dn8 - locals.var_vgs_eff2_dn8) + locals.var_vbseff_dn8), ((locals.var_vfb2_dn9 - locals.var_vgs_eff2_dn9) + locals.var_vbseff_dn9), (locals.var_vfb2_dn10 + locals.var_vbseff_dn10), (locals.var_vfb2_dn11 + locals.var_vbseff_dn11), (locals.var_vfb2_dn12 + locals.var_vbseff_dn12),)
    } else {
        (locals.var_v3, locals.var_v3_dn3, locals.var_v3_dn4, locals.var_v3_dn5, locals.var_v3_dn6, locals.var_v3_dn7, locals.var_v3_dn8, locals.var_v3_dn9, locals.var_v3_dn10, locals.var_v3_dn11, locals.var_v3_dn12,)
    }
};
        locals.var_v3 = assign26580_e24133;
        locals.var_v3_dn3 = assign26580_e24133_d_n3;
        locals.var_v3_dn4 = assign26580_e24133_d_n4;
        locals.var_v3_dn5 = assign26580_e24133_d_n5;
        locals.var_v3_dn6 = assign26580_e24133_d_n6;
        locals.var_v3_dn7 = assign26580_e24133_d_n7;
        locals.var_v3_dn8 = assign26580_e24133_d_n8;
        locals.var_v3_dn9 = assign26580_e24133_d_n9;
        locals.var_v3_dn10 = assign26580_e24133_d_n10;
        locals.var_v3_dn11 = assign26580_e24133_d_n11;
        locals.var_v3_dn12 = assign26580_e24133_d_n12;

        let assign26590_e24136: f64 = if locals.var_vfb2 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard1371 = assign26590_e24136;

        let (assign26600_e24156, assign26600_e24156_d_n3, assign26600_e24156_d_n4, assign26600_e24156_d_n5, assign26600_e24156_d_n6, assign26600_e24156_d_n7, assign26600_e24156_d_n8, assign26600_e24156_d_n9, assign26600_e24156_d_n10, assign26600_e24156_d_n11, assign26600_e24156_d_n12,) = {
    if ((((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1370 != 0.0)) && (locals.var_guard1371 != 0.0)) {
        let assign26600_e24147: f64 = (locals.var_v3 * locals.var_v3);
        let assign26600_e24150: f64 = (100.0 * locals.var_delta_3_soi2);
        let assign26600_e24152: f64 = (assign26600_e24150 * locals.var_vfb2);
        let assign26600_e24153: f64 = (assign26600_e24147 - assign26600_e24152);
        let assign26600_e24154: f64 = (assign26600_e24153).sqrt();
        (assign26600_e24154, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) - (assign26600_e24150 * locals.var_vfb2_dn3)) / (2.0 * assign26600_e24154)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) - (assign26600_e24150 * locals.var_vfb2_dn4)) / (2.0 * assign26600_e24154)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) - (assign26600_e24150 * locals.var_vfb2_dn5)) / (2.0 * assign26600_e24154)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) - (assign26600_e24150 * locals.var_vfb2_dn6)) / (2.0 * assign26600_e24154)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) - (assign26600_e24150 * locals.var_vfb2_dn7)) / (2.0 * assign26600_e24154)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) - (assign26600_e24150 * locals.var_vfb2_dn8)) / (2.0 * assign26600_e24154)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) - (assign26600_e24150 * locals.var_vfb2_dn9)) / (2.0 * assign26600_e24154)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) - (assign26600_e24150 * locals.var_vfb2_dn10)) / (2.0 * assign26600_e24154)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) - (assign26600_e24150 * locals.var_vfb2_dn11)) / (2.0 * assign26600_e24154)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) - (assign26600_e24150 * locals.var_vfb2_dn12)) / (2.0 * assign26600_e24154)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign26600_e24156;
        locals.var_t0__blk808_dn3 = assign26600_e24156_d_n3;
        locals.var_t0__blk808_dn4 = assign26600_e24156_d_n4;
        locals.var_t0__blk808_dn5 = assign26600_e24156_d_n5;
        locals.var_t0__blk808_dn6 = assign26600_e24156_d_n6;
        locals.var_t0__blk808_dn7 = assign26600_e24156_d_n7;
        locals.var_t0__blk808_dn8 = assign26600_e24156_d_n8;
        locals.var_t0__blk808_dn9 = assign26600_e24156_d_n9;
        locals.var_t0__blk808_dn10 = assign26600_e24156_d_n10;
        locals.var_t0__blk808_dn11 = assign26600_e24156_d_n11;
        locals.var_t0__blk808_dn12 = assign26600_e24156_d_n12;

        let (assign26610_e24177, assign26610_e24177_d_n3, assign26610_e24177_d_n4, assign26610_e24177_d_n5, assign26610_e24177_d_n6, assign26610_e24177_d_n7, assign26610_e24177_d_n8, assign26610_e24177_d_n9, assign26610_e24177_d_n10, assign26610_e24177_d_n11, assign26610_e24177_d_n12,) = {
    if ((((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1370 != 0.0)) && (locals.var_guard1371 == 0.0)) {
        let assign26610_e24168: f64 = (locals.var_v3 * locals.var_v3);
        let assign26610_e24171: f64 = (100.0 * locals.var_delta_3_soi2);
        let assign26610_e24173: f64 = (assign26610_e24171 * locals.var_vfb2);
        let assign26610_e24174: f64 = (assign26610_e24168 + assign26610_e24173);
        let assign26610_e24175: f64 = (assign26610_e24174).sqrt();
        (assign26610_e24175, ((((locals.var_v3_dn3 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn3)) + (assign26610_e24171 * locals.var_vfb2_dn3)) / (2.0 * assign26610_e24175)), ((((locals.var_v3_dn4 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn4)) + (assign26610_e24171 * locals.var_vfb2_dn4)) / (2.0 * assign26610_e24175)), ((((locals.var_v3_dn5 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn5)) + (assign26610_e24171 * locals.var_vfb2_dn5)) / (2.0 * assign26610_e24175)), ((((locals.var_v3_dn6 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn6)) + (assign26610_e24171 * locals.var_vfb2_dn6)) / (2.0 * assign26610_e24175)), ((((locals.var_v3_dn7 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn7)) + (assign26610_e24171 * locals.var_vfb2_dn7)) / (2.0 * assign26610_e24175)), ((((locals.var_v3_dn8 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn8)) + (assign26610_e24171 * locals.var_vfb2_dn8)) / (2.0 * assign26610_e24175)), ((((locals.var_v3_dn9 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn9)) + (assign26610_e24171 * locals.var_vfb2_dn9)) / (2.0 * assign26610_e24175)), ((((locals.var_v3_dn10 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn10)) + (assign26610_e24171 * locals.var_vfb2_dn10)) / (2.0 * assign26610_e24175)), ((((locals.var_v3_dn11 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn11)) + (assign26610_e24171 * locals.var_vfb2_dn11)) / (2.0 * assign26610_e24175)), ((((locals.var_v3_dn12 * locals.var_v3) + (locals.var_v3 * locals.var_v3_dn12)) + (assign26610_e24171 * locals.var_vfb2_dn12)) / (2.0 * assign26610_e24175)),)
    } else {
        (locals.var_t0__blk808, locals.var_t0__blk808_dn3, locals.var_t0__blk808_dn4, locals.var_t0__blk808_dn5, locals.var_t0__blk808_dn6, locals.var_t0__blk808_dn7, locals.var_t0__blk808_dn8, locals.var_t0__blk808_dn9, locals.var_t0__blk808_dn10, locals.var_t0__blk808_dn11, locals.var_t0__blk808_dn12,)
    }
};
        locals.var_t0__blk808 = assign26610_e24177;
        locals.var_t0__blk808_dn3 = assign26610_e24177_d_n3;
        locals.var_t0__blk808_dn4 = assign26610_e24177_d_n4;
        locals.var_t0__blk808_dn5 = assign26610_e24177_d_n5;
        locals.var_t0__blk808_dn6 = assign26610_e24177_d_n6;
        locals.var_t0__blk808_dn7 = assign26610_e24177_d_n7;
        locals.var_t0__blk808_dn8 = assign26610_e24177_d_n8;
        locals.var_t0__blk808_dn9 = assign26610_e24177_d_n9;
        locals.var_t0__blk808_dn10 = assign26610_e24177_d_n10;
        locals.var_t0__blk808_dn11 = assign26610_e24177_d_n11;
        locals.var_t0__blk808_dn12 = assign26610_e24177_d_n12;

        let (assign26620_e24192, assign26620_e24192_d_n3, assign26620_e24192_d_n4, assign26620_e24192_d_n5, assign26620_e24192_d_n6, assign26620_e24192_d_n7, assign26620_e24192_d_n8, assign26620_e24192_d_n9, assign26620_e24192_d_n10, assign26620_e24192_d_n11, assign26620_e24192_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1370 != 0.0)) {
        let assign26620_e24188: f64 = (locals.var_v3 + locals.var_t0__blk808);
        let assign26620_e24189: f64 = (0.5 * assign26620_e24188);
        let assign26620_e24190: f64 = (locals.var_vfb2 - assign26620_e24189);
        (assign26620_e24190, (locals.var_vfb2_dn3 - (0.5 * (locals.var_v3_dn3 + locals.var_t0__blk808_dn3))), (locals.var_vfb2_dn4 - (0.5 * (locals.var_v3_dn4 + locals.var_t0__blk808_dn4))), (locals.var_vfb2_dn5 - (0.5 * (locals.var_v3_dn5 + locals.var_t0__blk808_dn5))), (locals.var_vfb2_dn6 - (0.5 * (locals.var_v3_dn6 + locals.var_t0__blk808_dn6))), (locals.var_vfb2_dn7 - (0.5 * (locals.var_v3_dn7 + locals.var_t0__blk808_dn7))), (locals.var_vfb2_dn8 - (0.5 * (locals.var_v3_dn8 + locals.var_t0__blk808_dn8))), (locals.var_vfb2_dn9 - (0.5 * (locals.var_v3_dn9 + locals.var_t0__blk808_dn9))), (locals.var_vfb2_dn10 - (0.5 * (locals.var_v3_dn10 + locals.var_t0__blk808_dn10))), (locals.var_vfb2_dn11 - (0.5 * (locals.var_v3_dn11 + locals.var_t0__blk808_dn11))), (locals.var_vfb2_dn12 - (0.5 * (locals.var_v3_dn12 + locals.var_t0__blk808_dn12))),)
    } else {
        (locals.var_vfbeff2, locals.var_vfbeff2_dn3, locals.var_vfbeff2_dn4, locals.var_vfbeff2_dn5, locals.var_vfbeff2_dn6, locals.var_vfbeff2_dn7, locals.var_vfbeff2_dn8, locals.var_vfbeff2_dn9, locals.var_vfbeff2_dn10, locals.var_vfbeff2_dn11, locals.var_vfbeff2_dn12,)
    }
};
        locals.var_vfbeff2 = assign26620_e24192;
        locals.var_vfbeff2_dn3 = assign26620_e24192_d_n3;
        locals.var_vfbeff2_dn4 = assign26620_e24192_d_n4;
        locals.var_vfbeff2_dn5 = assign26620_e24192_d_n5;
        locals.var_vfbeff2_dn6 = assign26620_e24192_d_n6;
        locals.var_vfbeff2_dn7 = assign26620_e24192_d_n7;
        locals.var_vfbeff2_dn8 = assign26620_e24192_d_n8;
        locals.var_vfbeff2_dn9 = assign26620_e24192_d_n9;
        locals.var_vfbeff2_dn10 = assign26620_e24192_d_n10;
        locals.var_vfbeff2_dn11 = assign26620_e24192_d_n11;
        locals.var_vfbeff2_dn12 = assign26620_e24192_d_n12;

        let (assign26630_e24207, assign26630_e24207_d_n3, assign26630_e24207_d_n4, assign26630_e24207_d_n5, assign26630_e24207_d_n6, assign26630_e24207_d_n7, assign26630_e24207_d_n8, assign26630_e24207_d_n9, assign26630_e24207_d_n10, assign26630_e24207_d_n11, assign26630_e24207_d_n12,) = {
    if (((locals.var_guard1367 != 0.0) && (locals.var_guard1368 == 0.0)) && (locals.var_guard1370 != 0.0)) {
        let assign26630_e24203: f64 = (locals.var_vfbeff2 - locals.var_vfb2);
        let assign26630_e24204: f64 = (locals.var_coxwlb2 * assign26630_e24203);
        let assign26630_e24205: f64 = (locals.var_qac0 + assign26630_e24204);
        (assign26630_e24205, (locals.var_qac0_dn3 + ((locals.var_coxwlb2_dn3 * assign26630_e24203) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn3 - locals.var_vfb2_dn3)))), (locals.var_qac0_dn4 + ((locals.var_coxwlb2_dn4 * assign26630_e24203) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn4 - locals.var_vfb2_dn4)))), (locals.var_qac0_dn5 + ((locals.var_coxwlb2_dn5 * assign26630_e24203) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn5 - locals.var_vfb2_dn5)))), (locals.var_qac0_dn6 + ((locals.var_coxwlb2_dn6 * assign26630_e24203) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn6 - locals.var_vfb2_dn6)))), (locals.var_qac0_dn7 + ((locals.var_coxwlb2_dn7 * assign26630_e24203) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn7 - locals.var_vfb2_dn7)))), (locals.var_qac0_dn8 + ((locals.var_coxwlb2_dn8 * assign26630_e24203) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn8 - locals.var_vfb2_dn8)))), (locals.var_qac0_dn9 + ((locals.var_coxwlb2_dn9 * assign26630_e24203) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn9 - locals.var_vfb2_dn9)))), (locals.var_qac0_dn10 + ((locals.var_coxwlb2_dn10 * assign26630_e24203) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn10 - locals.var_vfb2_dn10)))), (locals.var_qac0_dn11 + ((locals.var_coxwlb2_dn11 * assign26630_e24203) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn11 - locals.var_vfb2_dn11)))), (locals.var_qac0_dn12 + ((locals.var_coxwlb2_dn12 * assign26630_e24203) + (locals.var_coxwlb2 * (locals.var_vfbeff2_dn12 - locals.var_vfb2_dn12)))),)
    } else {
        (locals.var_qac0, locals.var_qac0_dn3, locals.var_qac0_dn4, locals.var_qac0_dn5, locals.var_qac0_dn6, locals.var_qac0_dn7, locals.var_qac0_dn8, locals.var_qac0_dn9, locals.var_qac0_dn10, locals.var_qac0_dn11, locals.var_qac0_dn12,)
    }
};
        locals.var_qac0 = assign26630_e24207;
        locals.var_qac0_dn3 = assign26630_e24207_d_n3;
        locals.var_qac0_dn4 = assign26630_e24207_d_n4;
        locals.var_qac0_dn5 = assign26630_e24207_d_n5;
        locals.var_qac0_dn6 = assign26630_e24207_d_n6;
        locals.var_qac0_dn7 = assign26630_e24207_d_n7;
        locals.var_qac0_dn8 = assign26630_e24207_d_n8;
        locals.var_qac0_dn9 = assign26630_e24207_d_n9;
        locals.var_qac0_dn10 = assign26630_e24207_d_n10;
        locals.var_qac0_dn11 = assign26630_e24207_d_n11;
        locals.var_qac0_dn12 = assign26630_e24207_d_n12;

    }
}

#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_166(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign47640_e80549, assign47640_e80549_d_n3, assign47640_e80549_d_n4, assign47640_e80549_d_n5, assign47640_e80549_d_n6, assign47640_e80549_d_n7, assign47640_e80549_d_n8, assign47640_e80549_d_n9, assign47640_e80549_d_n10, assign47640_e80549_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (locals.var_dvsat, locals.var_dvsat_dn3, locals.var_dvsat_dn4, locals.var_dvsat_dn5, locals.var_dvsat_dn6, locals.var_dvsat_dn7, locals.var_dvsat_dn8, locals.var_dvsat_dn9, locals.var_dvsat_dn10, locals.var_dvsat_dn11,)
    } else {
        (locals.var_dptwg, locals.var_dptwg_dn3, locals.var_dptwg_dn4, locals.var_dptwg_dn5, locals.var_dptwg_dn6, locals.var_dptwg_dn7, locals.var_dptwg_dn8, locals.var_dptwg_dn9, locals.var_dptwg_dn10, locals.var_dptwg_dn11,)
    }
};
        locals.var_dptwg = assign47640_e80549;
        locals.var_dptwg_dn3 = assign47640_e80549_d_n3;
        locals.var_dptwg_dn4 = assign47640_e80549_d_n4;
        locals.var_dptwg_dn5 = assign47640_e80549_d_n5;
        locals.var_dptwg_dn6 = assign47640_e80549_d_n6;
        locals.var_dptwg_dn7 = assign47640_e80549_d_n7;
        locals.var_dptwg_dn8 = assign47640_e80549_d_n8;
        locals.var_dptwg_dn9 = assign47640_e80549_d_n9;
        locals.var_dptwg_dn10 = assign47640_e80549_d_n10;
        locals.var_dptwg_dn11 = assign47640_e80549_d_n11;
        locals.var_dptwg_rv = 0.0;

        let (assign47650_e80554, assign47650_e80554_d_n3, assign47650_e80554_d_n4, assign47650_e80554_d_n5, assign47650_e80554_d_n6, assign47650_e80554_d_n7, assign47650_e80554_d_n8, assign47650_e80554_d_n9, assign47650_e80554_d_n10, assign47650_e80554_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsource, locals.var_rsource_dn3, locals.var_rsource_dn4, locals.var_rsource_dn5, locals.var_rsource_dn6, locals.var_rsource_dn7, locals.var_rsource_dn8, locals.var_rsource_dn9, locals.var_rsource_dn10, locals.var_rsource_dn11,)
    }
};
        locals.var_rsource = assign47650_e80554;
        locals.var_rsource_dn3 = assign47650_e80554_d_n3;
        locals.var_rsource_dn4 = assign47650_e80554_d_n4;
        locals.var_rsource_dn5 = assign47650_e80554_d_n5;
        locals.var_rsource_dn6 = assign47650_e80554_d_n6;
        locals.var_rsource_dn7 = assign47650_e80554_d_n7;
        locals.var_rsource_dn8 = assign47650_e80554_d_n8;
        locals.var_rsource_dn9 = assign47650_e80554_d_n9;
        locals.var_rsource_dn10 = assign47650_e80554_d_n10;
        locals.var_rsource_dn11 = assign47650_e80554_d_n11;
        locals.var_rsource_rv = 0.0;

        let (assign47660_e80559, assign47660_e80559_d_n3, assign47660_e80559_d_n4, assign47660_e80559_d_n5, assign47660_e80559_d_n6, assign47660_e80559_d_n7, assign47660_e80559_d_n8, assign47660_e80559_d_n9, assign47660_e80559_d_n10, assign47660_e80559_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrain, locals.var_rdrain_dn3, locals.var_rdrain_dn4, locals.var_rdrain_dn5, locals.var_rdrain_dn6, locals.var_rdrain_dn7, locals.var_rdrain_dn8, locals.var_rdrain_dn9, locals.var_rdrain_dn10, locals.var_rdrain_dn11,)
    }
};
        locals.var_rdrain = assign47660_e80559;
        locals.var_rdrain_dn3 = assign47660_e80559_d_n3;
        locals.var_rdrain_dn4 = assign47660_e80559_d_n4;
        locals.var_rdrain_dn5 = assign47660_e80559_d_n5;
        locals.var_rdrain_dn6 = assign47660_e80559_d_n6;
        locals.var_rdrain_dn7 = assign47660_e80559_d_n7;
        locals.var_rdrain_dn8 = assign47660_e80559_d_n8;
        locals.var_rdrain_dn9 = assign47660_e80559_d_n9;
        locals.var_rdrain_dn10 = assign47660_e80559_d_n10;
        locals.var_rdrain_dn11 = assign47660_e80559_d_n11;
        locals.var_rdrain_rv = 0.0;

        let assign47670_e80562: f64 = if p.p33 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard739 = assign47670_e80562;
        locals.var_guard739_rv = 0.0;

        let (assign47680_e80569, assign47680_e80569_d_n3, assign47680_e80569_d_n4, assign47680_e80569_d_n5, assign47680_e80569_d_n6, assign47680_e80569_d_n7, assign47680_e80569_d_n8, assign47680_e80569_d_n9, assign47680_e80569_d_n10, assign47680_e80569_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdsi, locals.var_rdsi_dn3, locals.var_rdsi_dn4, locals.var_rdsi_dn5, locals.var_rdsi_dn6, locals.var_rdsi_dn7, locals.var_rdsi_dn8, locals.var_rdsi_dn9, locals.var_rdsi_dn10, locals.var_rdsi_dn11,)
    }
};
        locals.var_rdsi = assign47680_e80569;
        locals.var_rdsi_dn3 = assign47680_e80569_d_n3;
        locals.var_rdsi_dn4 = assign47680_e80569_d_n4;
        locals.var_rdsi_dn5 = assign47680_e80569_d_n5;
        locals.var_rdsi_dn6 = assign47680_e80569_d_n6;
        locals.var_rdsi_dn7 = assign47680_e80569_d_n7;
        locals.var_rdsi_dn8 = assign47680_e80569_d_n8;
        locals.var_rdsi_dn9 = assign47680_e80569_d_n9;
        locals.var_rdsi_dn10 = assign47680_e80569_d_n10;
        locals.var_rdsi_dn11 = assign47680_e80569_d_n11;
        locals.var_rdsi_rv = 0.0;

        let (assign47690_e80576, assign47690_e80576_d_n3, assign47690_e80576_d_n4, assign47690_e80576_d_n5, assign47690_e80576_d_n6, assign47690_e80576_d_n7, assign47690_e80576_d_n8, assign47690_e80576_d_n9, assign47690_e80576_d_n10, assign47690_e80576_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_dr, locals.var_dr_dn3, locals.var_dr_dn4, locals.var_dr_dn5, locals.var_dr_dn6, locals.var_dr_dn7, locals.var_dr_dn8, locals.var_dr_dn9, locals.var_dr_dn10, locals.var_dr_dn11,)
    }
};
        locals.var_dr = assign47690_e80576;
        locals.var_dr_dn3 = assign47690_e80576_d_n3;
        locals.var_dr_dn4 = assign47690_e80576_d_n4;
        locals.var_dr_dn5 = assign47690_e80576_d_n5;
        locals.var_dr_dn6 = assign47690_e80576_d_n6;
        locals.var_dr_dn7 = assign47690_e80576_d_n7;
        locals.var_dr_dn8 = assign47690_e80576_d_n8;
        locals.var_dr_dn9 = assign47690_e80576_d_n9;
        locals.var_dr_dn10 = assign47690_e80576_d_n10;
        locals.var_dr_dn11 = assign47690_e80576_d_n11;
        locals.var_dr_rv = 0.0;

        let (assign47700_e80585, assign47700_e80585_d_n3, assign47700_e80585_d_n4, assign47700_e80585_d_n5, assign47700_e80585_d_n6, assign47700_e80585_d_n7, assign47700_e80585_d_n8, assign47700_e80585_d_n9, assign47700_e80585_d_n10, assign47700_e80585_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47700_e80583: f64 = (locals.var_vgs_noswap - locals.var_vfbsdr);
        (assign47700_e80583, 0.0, (-locals.var_vfbsdr_dn4), (-locals.var_vfbsdr_dn5), locals.var_vgs_noswap_dn6, locals.var_vgs_noswap_dn7, locals.var_vgs_noswap_dn8, 0.0, locals.var_vgs_noswap_dn10, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign47700_e80585;
        locals.var_t2_dn3 = assign47700_e80585_d_n3;
        locals.var_t2_dn4 = assign47700_e80585_d_n4;
        locals.var_t2_dn5 = assign47700_e80585_d_n5;
        locals.var_t2_dn6 = assign47700_e80585_d_n6;
        locals.var_t2_dn7 = assign47700_e80585_d_n7;
        locals.var_t2_dn8 = assign47700_e80585_d_n8;
        locals.var_t2_dn9 = assign47700_e80585_d_n9;
        locals.var_t2_dn10 = assign47700_e80585_d_n10;
        locals.var_t2_dn11 = assign47700_e80585_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign47710_e80597, assign47710_e80597_d_n3, assign47710_e80597_d_n4, assign47710_e80597_d_n5, assign47710_e80597_d_n6, assign47710_e80597_d_n7, assign47710_e80597_d_n8, assign47710_e80597_d_n9, assign47710_e80597_d_n10, assign47710_e80597_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47710_e80592: f64 = (locals.var_t2 * locals.var_t2);
        let assign47710_e80594: f64 = (assign47710_e80592 + 0.01);
        let assign47710_e80595: f64 = (assign47710_e80594).sqrt();
        (assign47710_e80595, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign47710_e80595)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign47710_e80595)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign47710_e80597;
        locals.var_t3_dn3 = assign47710_e80597_d_n3;
        locals.var_t3_dn4 = assign47710_e80597_d_n4;
        locals.var_t3_dn5 = assign47710_e80597_d_n5;
        locals.var_t3_dn6 = assign47710_e80597_d_n6;
        locals.var_t3_dn7 = assign47710_e80597_d_n7;
        locals.var_t3_dn8 = assign47710_e80597_d_n8;
        locals.var_t3_dn9 = assign47710_e80597_d_n9;
        locals.var_t3_dn10 = assign47710_e80597_d_n10;
        locals.var_t3_dn11 = assign47710_e80597_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign47720_e80608, assign47720_e80608_d_n3, assign47720_e80608_d_n4, assign47720_e80608_d_n5, assign47720_e80608_d_n6, assign47720_e80608_d_n7, assign47720_e80608_d_n8, assign47720_e80608_d_n9, assign47720_e80608_d_n10, assign47720_e80608_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47720_e80605: f64 = (locals.var_t2 + locals.var_t3);
        let assign47720_e80606: f64 = (0.5 * assign47720_e80605);
        (assign47720_e80606, (0.5 * (locals.var_t2_dn3 + locals.var_t3_dn3)), (0.5 * (locals.var_t2_dn4 + locals.var_t3_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_t3_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_t3_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_t3_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_t3_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_t3_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_t3_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_t3_dn11)),)
    } else {
        (locals.var_vgs_eff, locals.var_vgs_eff_dn3, locals.var_vgs_eff_dn4, locals.var_vgs_eff_dn5, locals.var_vgs_eff_dn6, locals.var_vgs_eff_dn7, locals.var_vgs_eff_dn8, locals.var_vgs_eff_dn9, locals.var_vgs_eff_dn10, locals.var_vgs_eff_dn11,)
    }
};
        locals.var_vgs_eff = assign47720_e80608;
        locals.var_vgs_eff_dn3 = assign47720_e80608_d_n3;
        locals.var_vgs_eff_dn4 = assign47720_e80608_d_n4;
        locals.var_vgs_eff_dn5 = assign47720_e80608_d_n5;
        locals.var_vgs_eff_dn6 = assign47720_e80608_d_n6;
        locals.var_vgs_eff_dn7 = assign47720_e80608_d_n7;
        locals.var_vgs_eff_dn8 = assign47720_e80608_d_n8;
        locals.var_vgs_eff_dn9 = assign47720_e80608_d_n9;
        locals.var_vgs_eff_dn10 = assign47720_e80608_d_n10;
        locals.var_vgs_eff_dn11 = assign47720_e80608_d_n11;
        locals.var_vgs_eff_rv = 0.0;

        let (assign47730_e80619, assign47730_e80619_d_n3, assign47730_e80619_d_n4, assign47730_e80619_d_n5, assign47730_e80619_d_n6, assign47730_e80619_d_n7, assign47730_e80619_d_n8, assign47730_e80619_d_n9, assign47730_e80619_d_n10, assign47730_e80619_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47730_e80616: f64 = (locals.var_prwg_i * locals.var_vgs_eff);
        let assign47730_e80617: f64 = (1.0 + assign47730_e80616);
        (assign47730_e80617, (locals.var_prwg_i * locals.var_vgs_eff_dn3), (locals.var_prwg_i * locals.var_vgs_eff_dn4), (locals.var_prwg_i * locals.var_vgs_eff_dn5), (locals.var_prwg_i * locals.var_vgs_eff_dn6), (locals.var_prwg_i * locals.var_vgs_eff_dn7), (locals.var_prwg_i * locals.var_vgs_eff_dn8), (locals.var_prwg_i * locals.var_vgs_eff_dn9), (locals.var_prwg_i * locals.var_vgs_eff_dn10), (locals.var_prwg_i * locals.var_vgs_eff_dn11),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign47730_e80619;
        locals.var_t5_dn3 = assign47730_e80619_d_n3;
        locals.var_t5_dn4 = assign47730_e80619_d_n4;
        locals.var_t5_dn5 = assign47730_e80619_d_n5;
        locals.var_t5_dn6 = assign47730_e80619_d_n6;
        locals.var_t5_dn7 = assign47730_e80619_d_n7;
        locals.var_t5_dn8 = assign47730_e80619_d_n8;
        locals.var_t5_dn9 = assign47730_e80619_d_n9;
        locals.var_t5_dn10 = assign47730_e80619_d_n10;
        locals.var_t5_dn11 = assign47730_e80619_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign47740_e80632, assign47740_e80632_d_n3, assign47740_e80632_d_n4, assign47740_e80632_d_n5, assign47740_e80632_d_n6, assign47740_e80632_d_n7, assign47740_e80632_d_n8, assign47740_e80632_d_n9, assign47740_e80632_d_n10, assign47740_e80632_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47740_e80626: f64 = (1.0 / locals.var_t5);
        let assign47740_e80629: f64 = (locals.var_prwb_i * locals.var_vsb_noswap);
        let assign47740_e80630: f64 = (assign47740_e80626 + assign47740_e80629);
        (assign47740_e80630, (-(locals.var_t5_dn3 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn4 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn5 / (locals.var_t5 * locals.var_t5))), ((-(locals.var_t5_dn6 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vsb_noswap_dn6)), ((-(locals.var_t5_dn7 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vsb_noswap_dn7)), (-(locals.var_t5_dn8 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn9 / (locals.var_t5 * locals.var_t5))), ((-(locals.var_t5_dn10 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vsb_noswap_dn10)), (-(locals.var_t5_dn11 / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign47740_e80632;
        locals.var_t6_dn3 = assign47740_e80632_d_n3;
        locals.var_t6_dn4 = assign47740_e80632_d_n4;
        locals.var_t6_dn5 = assign47740_e80632_d_n5;
        locals.var_t6_dn6 = assign47740_e80632_d_n6;
        locals.var_t6_dn7 = assign47740_e80632_d_n7;
        locals.var_t6_dn8 = assign47740_e80632_d_n8;
        locals.var_t6_dn9 = assign47740_e80632_d_n9;
        locals.var_t6_dn10 = assign47740_e80632_d_n10;
        locals.var_t6_dn11 = assign47740_e80632_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign47750_e80648, assign47750_e80648_d_n3, assign47750_e80648_d_n4, assign47750_e80648_d_n5, assign47750_e80648_d_n6, assign47750_e80648_d_n7, assign47750_e80648_d_n8, assign47750_e80648_d_n9, assign47750_e80648_d_n10, assign47750_e80648_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47750_e80641: f64 = (locals.var_t6 * locals.var_t6);
        let assign47750_e80643: f64 = (assign47750_e80641 + 0.01);
        let assign47750_e80644: f64 = (assign47750_e80643).sqrt();
        let assign47750_e80645: f64 = (locals.var_t6 + assign47750_e80644);
        let assign47750_e80646: f64 = (0.5 * assign47750_e80645);
        (assign47750_e80646, (0.5 * (locals.var_t6_dn3 + (((locals.var_t6_dn3 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn3)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn4 + (((locals.var_t6_dn4 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn4)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn5 + (((locals.var_t6_dn5 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn5)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn6 + (((locals.var_t6_dn6 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn6)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn7 + (((locals.var_t6_dn7 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn7)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn8 + (((locals.var_t6_dn8 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn8)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn9 + (((locals.var_t6_dn9 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn9)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn10 + (((locals.var_t6_dn10 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn10)) / (2.0 * assign47750_e80644)))), (0.5 * (locals.var_t6_dn11 + (((locals.var_t6_dn11 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn11)) / (2.0 * assign47750_e80644)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign47750_e80648;
        locals.var_t4_dn3 = assign47750_e80648_d_n3;
        locals.var_t4_dn4 = assign47750_e80648_d_n4;
        locals.var_t4_dn5 = assign47750_e80648_d_n5;
        locals.var_t4_dn6 = assign47750_e80648_d_n6;
        locals.var_t4_dn7 = assign47750_e80648_d_n7;
        locals.var_t4_dn8 = assign47750_e80648_d_n8;
        locals.var_t4_dn9 = assign47750_e80648_d_n9;
        locals.var_t4_dn10 = assign47750_e80648_d_n10;
        locals.var_t4_dn11 = assign47750_e80648_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign47760_e80665, assign47760_e80665_d_n3, assign47760_e80665_d_n4, assign47760_e80665_d_n5, assign47760_e80665_d_n6, assign47760_e80665_d_n7, assign47760_e80665_d_n8, assign47760_e80665_d_n9, assign47760_e80665_d_n10, assign47760_e80665_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47760_e80658: f64 = (locals.var_rsw_i * locals.var_t4);
        let assign47760_e80659: f64 = (locals.var_rswmin_i + assign47760_e80658);
        let assign47760_e80661: f64 = (assign47760_e80659 * locals.var_weffwrfactor);
        let assign47760_e80662: f64 = (locals.var_rsourcegeo + assign47760_e80661);
        let assign47760_e80663: f64 = (locals.var_rdstemp * assign47760_e80662);
        (assign47760_e80663, (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn3) * locals.var_weffwrfactor)), ((locals.var_rdstemp_dn4 * assign47760_e80662) + (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn4) * locals.var_weffwrfactor))), ((locals.var_rdstemp_dn5 * assign47760_e80662) + (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn5) * locals.var_weffwrfactor))), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn6) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn7) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn8) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn9) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn10) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rsw_i * locals.var_t4_dn11) * locals.var_weffwrfactor)),)
    } else {
        (locals.var_rsource, locals.var_rsource_dn3, locals.var_rsource_dn4, locals.var_rsource_dn5, locals.var_rsource_dn6, locals.var_rsource_dn7, locals.var_rsource_dn8, locals.var_rsource_dn9, locals.var_rsource_dn10, locals.var_rsource_dn11,)
    }
};
        locals.var_rsource = assign47760_e80665;
        locals.var_rsource_dn3 = assign47760_e80665_d_n3;
        locals.var_rsource_dn4 = assign47760_e80665_d_n4;
        locals.var_rsource_dn5 = assign47760_e80665_d_n5;
        locals.var_rsource_dn6 = assign47760_e80665_d_n6;
        locals.var_rsource_dn7 = assign47760_e80665_d_n7;
        locals.var_rsource_dn8 = assign47760_e80665_d_n8;
        locals.var_rsource_dn9 = assign47760_e80665_d_n9;
        locals.var_rsource_dn10 = assign47760_e80665_d_n10;
        locals.var_rsource_dn11 = assign47760_e80665_d_n11;
        locals.var_rsource_rv = 0.0;

        let (assign47770_e80674, assign47770_e80674_d_n3, assign47770_e80674_d_n4, assign47770_e80674_d_n5, assign47770_e80674_d_n6, assign47770_e80674_d_n7, assign47770_e80674_d_n8, assign47770_e80674_d_n9, assign47770_e80674_d_n10, assign47770_e80674_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47770_e80672: f64 = (locals.var_vgd_noswap - locals.var_vfbsdr);
        (assign47770_e80672, 0.0, (-locals.var_vfbsdr_dn4), (-locals.var_vfbsdr_dn5), locals.var_vgd_noswap_dn6, locals.var_vgd_noswap_dn7, locals.var_vgd_noswap_dn8, 0.0, locals.var_vgd_noswap_dn10, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign47770_e80674;
        locals.var_t2_dn3 = assign47770_e80674_d_n3;
        locals.var_t2_dn4 = assign47770_e80674_d_n4;
        locals.var_t2_dn5 = assign47770_e80674_d_n5;
        locals.var_t2_dn6 = assign47770_e80674_d_n6;
        locals.var_t2_dn7 = assign47770_e80674_d_n7;
        locals.var_t2_dn8 = assign47770_e80674_d_n8;
        locals.var_t2_dn9 = assign47770_e80674_d_n9;
        locals.var_t2_dn10 = assign47770_e80674_d_n10;
        locals.var_t2_dn11 = assign47770_e80674_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign47780_e80686, assign47780_e80686_d_n3, assign47780_e80686_d_n4, assign47780_e80686_d_n5, assign47780_e80686_d_n6, assign47780_e80686_d_n7, assign47780_e80686_d_n8, assign47780_e80686_d_n9, assign47780_e80686_d_n10, assign47780_e80686_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47780_e80681: f64 = (locals.var_t2 * locals.var_t2);
        let assign47780_e80683: f64 = (assign47780_e80681 + 0.01);
        let assign47780_e80684: f64 = (assign47780_e80683).sqrt();
        (assign47780_e80684, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign47780_e80684)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign47780_e80684)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign47780_e80686;
        locals.var_t3_dn3 = assign47780_e80686_d_n3;
        locals.var_t3_dn4 = assign47780_e80686_d_n4;
        locals.var_t3_dn5 = assign47780_e80686_d_n5;
        locals.var_t3_dn6 = assign47780_e80686_d_n6;
        locals.var_t3_dn7 = assign47780_e80686_d_n7;
        locals.var_t3_dn8 = assign47780_e80686_d_n8;
        locals.var_t3_dn9 = assign47780_e80686_d_n9;
        locals.var_t3_dn10 = assign47780_e80686_d_n10;
        locals.var_t3_dn11 = assign47780_e80686_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign47790_e80697, assign47790_e80697_d_n3, assign47790_e80697_d_n4, assign47790_e80697_d_n5, assign47790_e80697_d_n6, assign47790_e80697_d_n7, assign47790_e80697_d_n8, assign47790_e80697_d_n9, assign47790_e80697_d_n10, assign47790_e80697_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47790_e80694: f64 = (locals.var_t2 + locals.var_t3);
        let assign47790_e80695: f64 = (0.5 * assign47790_e80694);
        (assign47790_e80695, (0.5 * (locals.var_t2_dn3 + locals.var_t3_dn3)), (0.5 * (locals.var_t2_dn4 + locals.var_t3_dn4)), (0.5 * (locals.var_t2_dn5 + locals.var_t3_dn5)), (0.5 * (locals.var_t2_dn6 + locals.var_t3_dn6)), (0.5 * (locals.var_t2_dn7 + locals.var_t3_dn7)), (0.5 * (locals.var_t2_dn8 + locals.var_t3_dn8)), (0.5 * (locals.var_t2_dn9 + locals.var_t3_dn9)), (0.5 * (locals.var_t2_dn10 + locals.var_t3_dn10)), (0.5 * (locals.var_t2_dn11 + locals.var_t3_dn11)),)
    } else {
        (locals.var_vgd_eff, locals.var_vgd_eff_dn3, locals.var_vgd_eff_dn4, locals.var_vgd_eff_dn5, locals.var_vgd_eff_dn6, locals.var_vgd_eff_dn7, locals.var_vgd_eff_dn8, locals.var_vgd_eff_dn9, locals.var_vgd_eff_dn10, locals.var_vgd_eff_dn11,)
    }
};
        locals.var_vgd_eff = assign47790_e80697;
        locals.var_vgd_eff_dn3 = assign47790_e80697_d_n3;
        locals.var_vgd_eff_dn4 = assign47790_e80697_d_n4;
        locals.var_vgd_eff_dn5 = assign47790_e80697_d_n5;
        locals.var_vgd_eff_dn6 = assign47790_e80697_d_n6;
        locals.var_vgd_eff_dn7 = assign47790_e80697_d_n7;
        locals.var_vgd_eff_dn8 = assign47790_e80697_d_n8;
        locals.var_vgd_eff_dn9 = assign47790_e80697_d_n9;
        locals.var_vgd_eff_dn10 = assign47790_e80697_d_n10;
        locals.var_vgd_eff_dn11 = assign47790_e80697_d_n11;
        locals.var_vgd_eff_rv = 0.0;

        let (assign47800_e80708, assign47800_e80708_d_n3, assign47800_e80708_d_n4, assign47800_e80708_d_n5, assign47800_e80708_d_n6, assign47800_e80708_d_n7, assign47800_e80708_d_n8, assign47800_e80708_d_n9, assign47800_e80708_d_n10, assign47800_e80708_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47800_e80705: f64 = (locals.var_prwg_i * locals.var_vgd_eff);
        let assign47800_e80706: f64 = (1.0 + assign47800_e80705);
        (assign47800_e80706, (locals.var_prwg_i * locals.var_vgd_eff_dn3), (locals.var_prwg_i * locals.var_vgd_eff_dn4), (locals.var_prwg_i * locals.var_vgd_eff_dn5), (locals.var_prwg_i * locals.var_vgd_eff_dn6), (locals.var_prwg_i * locals.var_vgd_eff_dn7), (locals.var_prwg_i * locals.var_vgd_eff_dn8), (locals.var_prwg_i * locals.var_vgd_eff_dn9), (locals.var_prwg_i * locals.var_vgd_eff_dn10), (locals.var_prwg_i * locals.var_vgd_eff_dn11),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign47800_e80708;
        locals.var_t5_dn3 = assign47800_e80708_d_n3;
        locals.var_t5_dn4 = assign47800_e80708_d_n4;
        locals.var_t5_dn5 = assign47800_e80708_d_n5;
        locals.var_t5_dn6 = assign47800_e80708_d_n6;
        locals.var_t5_dn7 = assign47800_e80708_d_n7;
        locals.var_t5_dn8 = assign47800_e80708_d_n8;
        locals.var_t5_dn9 = assign47800_e80708_d_n9;
        locals.var_t5_dn10 = assign47800_e80708_d_n10;
        locals.var_t5_dn11 = assign47800_e80708_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign47810_e80721, assign47810_e80721_d_n3, assign47810_e80721_d_n4, assign47810_e80721_d_n5, assign47810_e80721_d_n6, assign47810_e80721_d_n7, assign47810_e80721_d_n8, assign47810_e80721_d_n9, assign47810_e80721_d_n10, assign47810_e80721_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47810_e80715: f64 = (1.0 / locals.var_t5);
        let assign47810_e80718: f64 = (locals.var_prwb_i * locals.var_vdb_noswap);
        let assign47810_e80719: f64 = (assign47810_e80715 + assign47810_e80718);
        (assign47810_e80719, (-(locals.var_t5_dn3 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn4 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn5 / (locals.var_t5 * locals.var_t5))), ((-(locals.var_t5_dn6 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vdb_noswap_dn6)), ((-(locals.var_t5_dn7 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vdb_noswap_dn7)), (-(locals.var_t5_dn8 / (locals.var_t5 * locals.var_t5))), (-(locals.var_t5_dn9 / (locals.var_t5 * locals.var_t5))), ((-(locals.var_t5_dn10 / (locals.var_t5 * locals.var_t5))) + (locals.var_prwb_i * locals.var_vdb_noswap_dn10)), (-(locals.var_t5_dn11 / (locals.var_t5 * locals.var_t5))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign47810_e80721;
        locals.var_t6_dn3 = assign47810_e80721_d_n3;
        locals.var_t6_dn4 = assign47810_e80721_d_n4;
        locals.var_t6_dn5 = assign47810_e80721_d_n5;
        locals.var_t6_dn6 = assign47810_e80721_d_n6;
        locals.var_t6_dn7 = assign47810_e80721_d_n7;
        locals.var_t6_dn8 = assign47810_e80721_d_n8;
        locals.var_t6_dn9 = assign47810_e80721_d_n9;
        locals.var_t6_dn10 = assign47810_e80721_d_n10;
        locals.var_t6_dn11 = assign47810_e80721_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign47820_e80737, assign47820_e80737_d_n3, assign47820_e80737_d_n4, assign47820_e80737_d_n5, assign47820_e80737_d_n6, assign47820_e80737_d_n7, assign47820_e80737_d_n8, assign47820_e80737_d_n9, assign47820_e80737_d_n10, assign47820_e80737_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47820_e80730: f64 = (locals.var_t6 * locals.var_t6);
        let assign47820_e80732: f64 = (assign47820_e80730 + 0.01);
        let assign47820_e80733: f64 = (assign47820_e80732).sqrt();
        let assign47820_e80734: f64 = (locals.var_t6 + assign47820_e80733);
        let assign47820_e80735: f64 = (0.5 * assign47820_e80734);
        (assign47820_e80735, (0.5 * (locals.var_t6_dn3 + (((locals.var_t6_dn3 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn3)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn4 + (((locals.var_t6_dn4 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn4)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn5 + (((locals.var_t6_dn5 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn5)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn6 + (((locals.var_t6_dn6 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn6)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn7 + (((locals.var_t6_dn7 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn7)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn8 + (((locals.var_t6_dn8 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn8)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn9 + (((locals.var_t6_dn9 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn9)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn10 + (((locals.var_t6_dn10 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn10)) / (2.0 * assign47820_e80733)))), (0.5 * (locals.var_t6_dn11 + (((locals.var_t6_dn11 * locals.var_t6) + (locals.var_t6 * locals.var_t6_dn11)) / (2.0 * assign47820_e80733)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign47820_e80737;
        locals.var_t4_dn3 = assign47820_e80737_d_n3;
        locals.var_t4_dn4 = assign47820_e80737_d_n4;
        locals.var_t4_dn5 = assign47820_e80737_d_n5;
        locals.var_t4_dn6 = assign47820_e80737_d_n6;
        locals.var_t4_dn7 = assign47820_e80737_d_n7;
        locals.var_t4_dn8 = assign47820_e80737_d_n8;
        locals.var_t4_dn9 = assign47820_e80737_d_n9;
        locals.var_t4_dn10 = assign47820_e80737_d_n10;
        locals.var_t4_dn11 = assign47820_e80737_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign47830_e80754, assign47830_e80754_d_n3, assign47830_e80754_d_n4, assign47830_e80754_d_n5, assign47830_e80754_d_n6, assign47830_e80754_d_n7, assign47830_e80754_d_n8, assign47830_e80754_d_n9, assign47830_e80754_d_n10, assign47830_e80754_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 != 0.0)) {
        let assign47830_e80747: f64 = (locals.var_rdw_i * locals.var_t4);
        let assign47830_e80748: f64 = (locals.var_rdwmin_i + assign47830_e80747);
        let assign47830_e80750: f64 = (assign47830_e80748 * locals.var_weffwrfactor);
        let assign47830_e80751: f64 = (locals.var_rdraingeo + assign47830_e80750);
        let assign47830_e80752: f64 = (locals.var_rdstemp * assign47830_e80751);
        (assign47830_e80752, (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn3) * locals.var_weffwrfactor)), ((locals.var_rdstemp_dn4 * assign47830_e80751) + (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn4) * locals.var_weffwrfactor))), ((locals.var_rdstemp_dn5 * assign47830_e80751) + (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn5) * locals.var_weffwrfactor))), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn6) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn7) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn8) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn9) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn10) * locals.var_weffwrfactor)), (locals.var_rdstemp * ((locals.var_rdw_i * locals.var_t4_dn11) * locals.var_weffwrfactor)),)
    } else {
        (locals.var_rdrain, locals.var_rdrain_dn3, locals.var_rdrain_dn4, locals.var_rdrain_dn5, locals.var_rdrain_dn6, locals.var_rdrain_dn7, locals.var_rdrain_dn8, locals.var_rdrain_dn9, locals.var_rdrain_dn10, locals.var_rdrain_dn11,)
    }
};
        locals.var_rdrain = assign47830_e80754;
        locals.var_rdrain_dn3 = assign47830_e80754_d_n3;
        locals.var_rdrain_dn4 = assign47830_e80754_d_n4;
        locals.var_rdrain_dn5 = assign47830_e80754_d_n5;
        locals.var_rdrain_dn6 = assign47830_e80754_d_n6;
        locals.var_rdrain_dn7 = assign47830_e80754_d_n7;
        locals.var_rdrain_dn8 = assign47830_e80754_d_n8;
        locals.var_rdrain_dn9 = assign47830_e80754_d_n9;
        locals.var_rdrain_dn10 = assign47830_e80754_d_n10;
        locals.var_rdrain_dn11 = assign47830_e80754_d_n11;
        locals.var_rdrain_rv = 0.0;

        let (assign47840_e80766, assign47840_e80766_d_n3, assign47840_e80766_d_n4, assign47840_e80766_d_n5, assign47840_e80766_d_n6, assign47840_e80766_d_n7, assign47840_e80766_d_n8, assign47840_e80766_d_n9, assign47840_e80766_d_n10, assign47840_e80766_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        let assign47840_e80763: f64 = (locals.var_prwg_i * locals.var_qia);
        let assign47840_e80764: f64 = (1.0 + assign47840_e80763);
        (assign47840_e80764, (locals.var_prwg_i * locals.var_qia_dn3), (locals.var_prwg_i * locals.var_qia_dn4), (locals.var_prwg_i * locals.var_qia_dn5), (locals.var_prwg_i * locals.var_qia_dn6), (locals.var_prwg_i * locals.var_qia_dn7), (locals.var_prwg_i * locals.var_qia_dn8), (locals.var_prwg_i * locals.var_qia_dn9), (locals.var_prwg_i * locals.var_qia_dn10), (locals.var_prwg_i * locals.var_qia_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign47840_e80766;
        locals.var_t0_dn3 = assign47840_e80766_d_n3;
        locals.var_t0_dn4 = assign47840_e80766_d_n4;
        locals.var_t0_dn5 = assign47840_e80766_d_n5;
        locals.var_t0_dn6 = assign47840_e80766_d_n6;
        locals.var_t0_dn7 = assign47840_e80766_d_n7;
        locals.var_t0_dn8 = assign47840_e80766_d_n8;
        locals.var_t0_dn9 = assign47840_e80766_d_n9;
        locals.var_t0_dn10 = assign47840_e80766_d_n10;
        locals.var_t0_dn11 = assign47840_e80766_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign47850_e80778, assign47850_e80778_d_n3, assign47850_e80778_d_n4, assign47850_e80778_d_n5, assign47850_e80778_d_n6, assign47850_e80778_d_n7, assign47850_e80778_d_n8, assign47850_e80778_d_n9, assign47850_e80778_d_n10, assign47850_e80778_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        let assign47850_e80775: f64 = (locals.var_sqrtphistvbs - locals.var_sqrtphist);
        let assign47850_e80776: f64 = (locals.var_prwb_i * assign47850_e80775);
        (assign47850_e80776, (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn3 - locals.var_sqrtphist_dn3)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn4 - locals.var_sqrtphist_dn4)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn5 - locals.var_sqrtphist_dn5)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn6 - locals.var_sqrtphist_dn6)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn7 - locals.var_sqrtphist_dn7)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn8 - locals.var_sqrtphist_dn8)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn9 - locals.var_sqrtphist_dn9)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn10 - locals.var_sqrtphist_dn10)), (locals.var_prwb_i * (locals.var_sqrtphistvbs_dn11 - locals.var_sqrtphist_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign47850_e80778;
        locals.var_t1_dn3 = assign47850_e80778_d_n3;
        locals.var_t1_dn4 = assign47850_e80778_d_n4;
        locals.var_t1_dn5 = assign47850_e80778_d_n5;
        locals.var_t1_dn6 = assign47850_e80778_d_n6;
        locals.var_t1_dn7 = assign47850_e80778_d_n7;
        locals.var_t1_dn8 = assign47850_e80778_d_n8;
        locals.var_t1_dn9 = assign47850_e80778_d_n9;
        locals.var_t1_dn10 = assign47850_e80778_d_n10;
        locals.var_t1_dn11 = assign47850_e80778_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign47860_e80790, assign47860_e80790_d_n3, assign47860_e80790_d_n4, assign47860_e80790_d_n5, assign47860_e80790_d_n6, assign47860_e80790_d_n7, assign47860_e80790_d_n8, assign47860_e80790_d_n9, assign47860_e80790_d_n10, assign47860_e80790_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        let assign47860_e80786: f64 = (1.0 / locals.var_t0);
        let assign47860_e80788: f64 = (assign47860_e80786 + locals.var_t1);
        (assign47860_e80788, ((-(locals.var_t0_dn3 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn3), ((-(locals.var_t0_dn4 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn4), ((-(locals.var_t0_dn5 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn5), ((-(locals.var_t0_dn6 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn6), ((-(locals.var_t0_dn7 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn7), ((-(locals.var_t0_dn8 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn8), ((-(locals.var_t0_dn9 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn9), ((-(locals.var_t0_dn10 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn10), ((-(locals.var_t0_dn11 / (locals.var_t0 * locals.var_t0))) + locals.var_t1_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign47860_e80790;
        locals.var_t2_dn3 = assign47860_e80790_d_n3;
        locals.var_t2_dn4 = assign47860_e80790_d_n4;
        locals.var_t2_dn5 = assign47860_e80790_d_n5;
        locals.var_t2_dn6 = assign47860_e80790_d_n6;
        locals.var_t2_dn7 = assign47860_e80790_d_n7;
        locals.var_t2_dn8 = assign47860_e80790_d_n8;
        locals.var_t2_dn9 = assign47860_e80790_d_n9;
        locals.var_t2_dn10 = assign47860_e80790_d_n10;
        locals.var_t2_dn11 = assign47860_e80790_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign47870_e80807, assign47870_e80807_d_n3, assign47870_e80807_d_n4, assign47870_e80807_d_n5, assign47870_e80807_d_n6, assign47870_e80807_d_n7, assign47870_e80807_d_n8, assign47870_e80807_d_n9, assign47870_e80807_d_n10, assign47870_e80807_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        let assign47870_e80800: f64 = (locals.var_t2 * locals.var_t2);
        let assign47870_e80802: f64 = (assign47870_e80800 + 0.01);
        let assign47870_e80803: f64 = (assign47870_e80802).sqrt();
        let assign47870_e80804: f64 = (locals.var_t2 + assign47870_e80803);
        let assign47870_e80805: f64 = (0.5 * assign47870_e80804);
        (assign47870_e80805, (0.5 * (locals.var_t2_dn3 + (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn4 + (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn5 + (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn6 + (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn7 + (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn8 + (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn9 + (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn10 + (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign47870_e80803)))), (0.5 * (locals.var_t2_dn11 + (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign47870_e80803)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign47870_e80807;
        locals.var_t3_dn3 = assign47870_e80807_d_n3;
        locals.var_t3_dn4 = assign47870_e80807_d_n4;
        locals.var_t3_dn5 = assign47870_e80807_d_n5;
        locals.var_t3_dn6 = assign47870_e80807_d_n6;
        locals.var_t3_dn7 = assign47870_e80807_d_n7;
        locals.var_t3_dn8 = assign47870_e80807_d_n8;
        locals.var_t3_dn9 = assign47870_e80807_d_n9;
        locals.var_t3_dn10 = assign47870_e80807_d_n10;
        locals.var_t3_dn11 = assign47870_e80807_d_n11;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_167(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign47880_e80825, assign47880_e80825_d_n3, assign47880_e80825_d_n4, assign47880_e80825_d_n5, assign47880_e80825_d_n6, assign47880_e80825_d_n7, assign47880_e80825_d_n8, assign47880_e80825_d_n9, assign47880_e80825_d_n10, assign47880_e80825_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        let assign47880_e80817: f64 = (locals.var_rdsw_i * locals.var_t3);
        let assign47880_e80818: f64 = (locals.var_rdswmin_i + assign47880_e80817);
        let assign47880_e80819: f64 = (locals.var_rdstemp * assign47880_e80818);
        let assign47880_e80821: f64 = (assign47880_e80819 * locals.var_weffwrfactor);
        let assign47880_e80823: f64 = (assign47880_e80821 * p.p2);
        (assign47880_e80823, (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn3)) * locals.var_weffwrfactor) * p.p2), ((((locals.var_rdstemp_dn4 * assign47880_e80818) + (locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn4))) * locals.var_weffwrfactor) * p.p2), ((((locals.var_rdstemp_dn5 * assign47880_e80818) + (locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn5))) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn6)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn7)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn8)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn9)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn10)) * locals.var_weffwrfactor) * p.p2), (((locals.var_rdstemp * (locals.var_rdsw_i * locals.var_t3_dn11)) * locals.var_weffwrfactor) * p.p2),)
    } else {
        (locals.var_rdsi, locals.var_rdsi_dn3, locals.var_rdsi_dn4, locals.var_rdsi_dn5, locals.var_rdsi_dn6, locals.var_rdsi_dn7, locals.var_rdsi_dn8, locals.var_rdsi_dn9, locals.var_rdsi_dn10, locals.var_rdsi_dn11,)
    }
};
        locals.var_rdsi = assign47880_e80825;
        locals.var_rdsi_dn3 = assign47880_e80825_d_n3;
        locals.var_rdsi_dn4 = assign47880_e80825_d_n4;
        locals.var_rdsi_dn5 = assign47880_e80825_d_n5;
        locals.var_rdsi_dn6 = assign47880_e80825_d_n6;
        locals.var_rdsi_dn7 = assign47880_e80825_d_n7;
        locals.var_rdsi_dn8 = assign47880_e80825_d_n8;
        locals.var_rdsi_dn9 = assign47880_e80825_d_n9;
        locals.var_rdsi_dn10 = assign47880_e80825_d_n10;
        locals.var_rdsi_dn11 = assign47880_e80825_d_n11;
        locals.var_rdsi_rv = 0.0;

        let (assign47890_e80833, assign47890_e80833_d_n3, assign47890_e80833_d_n4, assign47890_e80833_d_n5, assign47890_e80833_d_n6, assign47890_e80833_d_n7, assign47890_e80833_d_n8, assign47890_e80833_d_n9, assign47890_e80833_d_n10, assign47890_e80833_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        (locals.var_rdraingeo, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrain, locals.var_rdrain_dn3, locals.var_rdrain_dn4, locals.var_rdrain_dn5, locals.var_rdrain_dn6, locals.var_rdrain_dn7, locals.var_rdrain_dn8, locals.var_rdrain_dn9, locals.var_rdrain_dn10, locals.var_rdrain_dn11,)
    }
};
        locals.var_rdrain = assign47890_e80833;
        locals.var_rdrain_dn3 = assign47890_e80833_d_n3;
        locals.var_rdrain_dn4 = assign47890_e80833_d_n4;
        locals.var_rdrain_dn5 = assign47890_e80833_d_n5;
        locals.var_rdrain_dn6 = assign47890_e80833_d_n6;
        locals.var_rdrain_dn7 = assign47890_e80833_d_n7;
        locals.var_rdrain_dn8 = assign47890_e80833_d_n8;
        locals.var_rdrain_dn9 = assign47890_e80833_d_n9;
        locals.var_rdrain_dn10 = assign47890_e80833_d_n10;
        locals.var_rdrain_dn11 = assign47890_e80833_d_n11;
        locals.var_rdrain_rv = 0.0;

        let (assign47900_e80841, assign47900_e80841_d_n3, assign47900_e80841_d_n4, assign47900_e80841_d_n5, assign47900_e80841_d_n6, assign47900_e80841_d_n7, assign47900_e80841_d_n8, assign47900_e80841_d_n9, assign47900_e80841_d_n10, assign47900_e80841_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        (locals.var_rsourcegeo, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsource, locals.var_rsource_dn3, locals.var_rsource_dn4, locals.var_rsource_dn5, locals.var_rsource_dn6, locals.var_rsource_dn7, locals.var_rsource_dn8, locals.var_rsource_dn9, locals.var_rsource_dn10, locals.var_rsource_dn11,)
    }
};
        locals.var_rsource = assign47900_e80841;
        locals.var_rsource_dn3 = assign47900_e80841_d_n3;
        locals.var_rsource_dn4 = assign47900_e80841_d_n4;
        locals.var_rsource_dn5 = assign47900_e80841_d_n5;
        locals.var_rsource_dn6 = assign47900_e80841_d_n6;
        locals.var_rsource_dn7 = assign47900_e80841_d_n7;
        locals.var_rsource_dn8 = assign47900_e80841_d_n8;
        locals.var_rsource_dn9 = assign47900_e80841_d_n9;
        locals.var_rsource_dn10 = assign47900_e80841_d_n10;
        locals.var_rsource_dn11 = assign47900_e80841_d_n11;
        locals.var_rsource_rv = 0.0;

        let (assign47910_e80865, assign47910_e80865_d_n3, assign47910_e80865_d_n4, assign47910_e80865_d_n5, assign47910_e80865_d_n6, assign47910_e80865_d_n7, assign47910_e80865_d_n8, assign47910_e80865_d_n9, assign47910_e80865_d_n10, assign47910_e80865_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) {
        let assign47910_e80851: f64 = (locals.var_dvsat * locals.var_dmob);
        let assign47910_e80852: f64 = (locals.var_u0_a / assign47910_e80851);
        let assign47910_e80854: f64 = (assign47910_e80852 * locals.var_cox);
        let assign47910_e80856: f64 = (assign47910_e80854 * locals.var_weff);
        let assign47910_e80858: f64 = (assign47910_e80856 / locals.var_leff);
        let assign47910_e80860: f64 = (assign47910_e80858 * locals.var_qia);
        let assign47910_e80862: f64 = (assign47910_e80860 * locals.var_rdsi);
        let assign47910_e80863: f64 = (1.0 + assign47910_e80862);
        (assign47910_e80863, ((((((((((locals.var_u0_a_dn3 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn3 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn3)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn3)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn3)), ((((((((((locals.var_u0_a_dn4 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn4 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn4)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn4)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn4)), ((((((((((locals.var_u0_a_dn5 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn5 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn5)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn5)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn5)), ((((((((((locals.var_u0_a_dn6 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn6 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn6)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn6)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn6)), ((((((((((locals.var_u0_a_dn7 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn7 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn7)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn7)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn7)), ((((((((((locals.var_u0_a_dn8 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn8 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn8)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn8)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn8)), ((((((((((locals.var_u0_a_dn9 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn9 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn9)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn9)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn9)), ((((((((((locals.var_u0_a_dn10 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn10 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn10)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn10)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn10)), ((((((((((locals.var_u0_a_dn11 * assign47910_e80851) - (locals.var_u0_a * ((locals.var_dvsat_dn11 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn11)))) / (assign47910_e80851 * assign47910_e80851)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47910_e80858 * locals.var_qia_dn11)) * locals.var_rdsi) + (assign47910_e80860 * locals.var_rdsi_dn11)),)
    } else {
        (locals.var_dr, locals.var_dr_dn3, locals.var_dr_dn4, locals.var_dr_dn5, locals.var_dr_dn6, locals.var_dr_dn7, locals.var_dr_dn8, locals.var_dr_dn9, locals.var_dr_dn10, locals.var_dr_dn11,)
    }
};
        locals.var_dr = assign47910_e80865;
        locals.var_dr_dn3 = assign47910_e80865_d_n3;
        locals.var_dr_dn4 = assign47910_e80865_d_n4;
        locals.var_dr_dn5 = assign47910_e80865_d_n5;
        locals.var_dr_dn6 = assign47910_e80865_d_n6;
        locals.var_dr_dn7 = assign47910_e80865_d_n7;
        locals.var_dr_dn8 = assign47910_e80865_d_n8;
        locals.var_dr_dn9 = assign47910_e80865_d_n9;
        locals.var_dr_dn10 = assign47910_e80865_d_n10;
        locals.var_dr_dn11 = assign47910_e80865_d_n11;
        locals.var_dr_rv = 0.0;

        let assign47920_e80868: f64 = if p.p33 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard740 = assign47920_e80868;
        locals.var_guard740_rv = 0.0;

        let (assign47930_e80892, assign47930_e80892_d_n3, assign47930_e80892_d_n4, assign47930_e80892_d_n5, assign47930_e80892_d_n6, assign47930_e80892_d_n7, assign47930_e80892_d_n8, assign47930_e80892_d_n9, assign47930_e80892_d_n10, assign47930_e80892_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) && (locals.var_guard740 != 0.0)) {
        let assign47930_e80881: f64 = (locals.var_rdsw_i * locals.var_t3);
        let assign47930_e80882: f64 = (locals.var_rdswmin_i + assign47930_e80881);
        let assign47930_e80884: f64 = (assign47930_e80882 * locals.var_weffwrfactor);
        let assign47930_e80886: f64 = (assign47930_e80884 * p.p2);
        let assign47930_e80887: f64 = (locals.var_rsourcegeo + assign47930_e80886);
        let assign47930_e80889: f64 = (assign47930_e80887 + locals.var_rdraingeo);
        let assign47930_e80890: f64 = (locals.var_rdstemp * assign47930_e80889);
        (assign47930_e80890, (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn3) * locals.var_weffwrfactor) * p.p2)), ((locals.var_rdstemp_dn4 * assign47930_e80889) + (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn4) * locals.var_weffwrfactor) * p.p2))), ((locals.var_rdstemp_dn5 * assign47930_e80889) + (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn5) * locals.var_weffwrfactor) * p.p2))), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn6) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn7) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn8) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn9) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn10) * locals.var_weffwrfactor) * p.p2)), (locals.var_rdstemp * (((locals.var_rdsw_i * locals.var_t3_dn11) * locals.var_weffwrfactor) * p.p2)),)
    } else {
        (locals.var_rdsi, locals.var_rdsi_dn3, locals.var_rdsi_dn4, locals.var_rdsi_dn5, locals.var_rdsi_dn6, locals.var_rdsi_dn7, locals.var_rdsi_dn8, locals.var_rdsi_dn9, locals.var_rdsi_dn10, locals.var_rdsi_dn11,)
    }
};
        locals.var_rdsi = assign47930_e80892;
        locals.var_rdsi_dn3 = assign47930_e80892_d_n3;
        locals.var_rdsi_dn4 = assign47930_e80892_d_n4;
        locals.var_rdsi_dn5 = assign47930_e80892_d_n5;
        locals.var_rdsi_dn6 = assign47930_e80892_d_n6;
        locals.var_rdsi_dn7 = assign47930_e80892_d_n7;
        locals.var_rdsi_dn8 = assign47930_e80892_d_n8;
        locals.var_rdsi_dn9 = assign47930_e80892_d_n9;
        locals.var_rdsi_dn10 = assign47930_e80892_d_n10;
        locals.var_rdsi_dn11 = assign47930_e80892_d_n11;
        locals.var_rdsi_rv = 0.0;

        let (assign47940_e80902, assign47940_e80902_d_n3, assign47940_e80902_d_n4, assign47940_e80902_d_n5, assign47940_e80902_d_n6, assign47940_e80902_d_n7, assign47940_e80902_d_n8, assign47940_e80902_d_n9, assign47940_e80902_d_n10, assign47940_e80902_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) && (locals.var_guard740 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrain, locals.var_rdrain_dn3, locals.var_rdrain_dn4, locals.var_rdrain_dn5, locals.var_rdrain_dn6, locals.var_rdrain_dn7, locals.var_rdrain_dn8, locals.var_rdrain_dn9, locals.var_rdrain_dn10, locals.var_rdrain_dn11,)
    }
};
        locals.var_rdrain = assign47940_e80902;
        locals.var_rdrain_dn3 = assign47940_e80902_d_n3;
        locals.var_rdrain_dn4 = assign47940_e80902_d_n4;
        locals.var_rdrain_dn5 = assign47940_e80902_d_n5;
        locals.var_rdrain_dn6 = assign47940_e80902_d_n6;
        locals.var_rdrain_dn7 = assign47940_e80902_d_n7;
        locals.var_rdrain_dn8 = assign47940_e80902_d_n8;
        locals.var_rdrain_dn9 = assign47940_e80902_d_n9;
        locals.var_rdrain_dn10 = assign47940_e80902_d_n10;
        locals.var_rdrain_dn11 = assign47940_e80902_d_n11;
        locals.var_rdrain_rv = 0.0;

        let (assign47950_e80912, assign47950_e80912_d_n3, assign47950_e80912_d_n4, assign47950_e80912_d_n5, assign47950_e80912_d_n6, assign47950_e80912_d_n7, assign47950_e80912_d_n8, assign47950_e80912_d_n9, assign47950_e80912_d_n10, assign47950_e80912_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) && (locals.var_guard740 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsource, locals.var_rsource_dn3, locals.var_rsource_dn4, locals.var_rsource_dn5, locals.var_rsource_dn6, locals.var_rsource_dn7, locals.var_rsource_dn8, locals.var_rsource_dn9, locals.var_rsource_dn10, locals.var_rsource_dn11,)
    }
};
        locals.var_rsource = assign47950_e80912;
        locals.var_rsource_dn3 = assign47950_e80912_d_n3;
        locals.var_rsource_dn4 = assign47950_e80912_d_n4;
        locals.var_rsource_dn5 = assign47950_e80912_d_n5;
        locals.var_rsource_dn6 = assign47950_e80912_d_n6;
        locals.var_rsource_dn7 = assign47950_e80912_d_n7;
        locals.var_rsource_dn8 = assign47950_e80912_d_n8;
        locals.var_rsource_dn9 = assign47950_e80912_d_n9;
        locals.var_rsource_dn10 = assign47950_e80912_d_n10;
        locals.var_rsource_dn11 = assign47950_e80912_d_n11;
        locals.var_rsource_rv = 0.0;

        let (assign47960_e80938, assign47960_e80938_d_n3, assign47960_e80938_d_n4, assign47960_e80938_d_n5, assign47960_e80938_d_n6, assign47960_e80938_d_n7, assign47960_e80938_d_n8, assign47960_e80938_d_n9, assign47960_e80938_d_n10, assign47960_e80938_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard739 == 0.0)) && (locals.var_guard740 != 0.0)) {
        let assign47960_e80924: f64 = (locals.var_dvsat * locals.var_dmob);
        let assign47960_e80925: f64 = (locals.var_u0_a / assign47960_e80924);
        let assign47960_e80927: f64 = (assign47960_e80925 * locals.var_cox);
        let assign47960_e80929: f64 = (assign47960_e80927 * locals.var_weff);
        let assign47960_e80931: f64 = (assign47960_e80929 / locals.var_leff);
        let assign47960_e80933: f64 = (assign47960_e80931 * locals.var_qia);
        let assign47960_e80935: f64 = (assign47960_e80933 * locals.var_rdsi);
        let assign47960_e80936: f64 = (1.0 + assign47960_e80935);
        (assign47960_e80936, ((((((((((locals.var_u0_a_dn3 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn3 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn3)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn3)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn3)), ((((((((((locals.var_u0_a_dn4 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn4 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn4)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn4)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn4)), ((((((((((locals.var_u0_a_dn5 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn5 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn5)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn5)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn5)), ((((((((((locals.var_u0_a_dn6 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn6 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn6)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn6)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn6)), ((((((((((locals.var_u0_a_dn7 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn7 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn7)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn7)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn7)), ((((((((((locals.var_u0_a_dn8 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn8 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn8)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn8)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn8)), ((((((((((locals.var_u0_a_dn9 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn9 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn9)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn9)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn9)), ((((((((((locals.var_u0_a_dn10 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn10 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn10)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn10)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn10)), ((((((((((locals.var_u0_a_dn11 * assign47960_e80924) - (locals.var_u0_a * ((locals.var_dvsat_dn11 * locals.var_dmob) + (locals.var_dvsat * locals.var_dmob_dn11)))) / (assign47960_e80924 * assign47960_e80924)) * locals.var_cox) * locals.var_weff) / locals.var_leff) * locals.var_qia) + (assign47960_e80931 * locals.var_qia_dn11)) * locals.var_rdsi) + (assign47960_e80933 * locals.var_rdsi_dn11)),)
    } else {
        (locals.var_dr, locals.var_dr_dn3, locals.var_dr_dn4, locals.var_dr_dn5, locals.var_dr_dn6, locals.var_dr_dn7, locals.var_dr_dn8, locals.var_dr_dn9, locals.var_dr_dn10, locals.var_dr_dn11,)
    }
};
        locals.var_dr = assign47960_e80938;
        locals.var_dr_dn3 = assign47960_e80938_d_n3;
        locals.var_dr_dn4 = assign47960_e80938_d_n4;
        locals.var_dr_dn5 = assign47960_e80938_d_n5;
        locals.var_dr_dn6 = assign47960_e80938_d_n6;
        locals.var_dr_dn7 = assign47960_e80938_d_n7;
        locals.var_dr_dn8 = assign47960_e80938_d_n8;
        locals.var_dr_dn9 = assign47960_e80938_d_n9;
        locals.var_dr_dn10 = assign47960_e80938_d_n10;
        locals.var_dr_dn11 = assign47960_e80938_d_n11;
        locals.var_dr_rv = 0.0;

        let (assign47970_e80953, assign47970_e80953_d_n3, assign47970_e80953_d_n4, assign47970_e80953_d_n5, assign47970_e80953_d_n6, assign47970_e80953_d_n7, assign47970_e80953_d_n8, assign47970_e80953_d_n9, assign47970_e80953_d_n10, assign47970_e80953_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47970_e80946: f64 = (2.0 * locals.var_n);
        let assign47970_e80948: f64 = (assign47970_e80946 * locals.var_vtm);
        let assign47970_e80949: f64 = (locals.var_qia + assign47970_e80948);
        let assign47970_e80950: f64 = (locals.var_a2_t / assign47970_e80949);
        let assign47970_e80951: f64 = (locals.var_a1_t + assign47970_e80950);
        (assign47970_e80951, (-((locals.var_a2_t * (locals.var_qia_dn3 + ((2.0 * locals.var_n_dn3) * locals.var_vtm))) / (assign47970_e80949 * assign47970_e80949))), (locals.var_a1_t_dn4 + (((locals.var_a2_t_dn4 * assign47970_e80949) - (locals.var_a2_t * (locals.var_qia_dn4 + (((2.0 * locals.var_n_dn4) * locals.var_vtm) + (assign47970_e80946 * locals.var_vtm_dn4))))) / (assign47970_e80949 * assign47970_e80949))), (locals.var_a1_t_dn5 + (((locals.var_a2_t_dn5 * assign47970_e80949) - (locals.var_a2_t * (locals.var_qia_dn5 + (((2.0 * locals.var_n_dn5) * locals.var_vtm) + (assign47970_e80946 * locals.var_vtm_dn5))))) / (assign47970_e80949 * assign47970_e80949))), (-((locals.var_a2_t * (locals.var_qia_dn6 + ((2.0 * locals.var_n_dn6) * locals.var_vtm))) / (assign47970_e80949 * assign47970_e80949))), (-((locals.var_a2_t * (locals.var_qia_dn7 + ((2.0 * locals.var_n_dn7) * locals.var_vtm))) / (assign47970_e80949 * assign47970_e80949))), (-((locals.var_a2_t * (locals.var_qia_dn8 + ((2.0 * locals.var_n_dn8) * locals.var_vtm))) / (assign47970_e80949 * assign47970_e80949))), (-((locals.var_a2_t * (locals.var_qia_dn9 + ((2.0 * locals.var_n_dn9) * locals.var_vtm))) / (assign47970_e80949 * assign47970_e80949))), (-((locals.var_a2_t * (locals.var_qia_dn10 + ((2.0 * locals.var_n_dn10) * locals.var_vtm))) / (assign47970_e80949 * assign47970_e80949))), (-((locals.var_a2_t * (locals.var_qia_dn11 + ((2.0 * locals.var_n_dn11) * locals.var_vtm))) / (assign47970_e80949 * assign47970_e80949))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign47970_e80953;
        locals.var_t0_dn3 = assign47970_e80953_d_n3;
        locals.var_t0_dn4 = assign47970_e80953_d_n4;
        locals.var_t0_dn5 = assign47970_e80953_d_n5;
        locals.var_t0_dn6 = assign47970_e80953_d_n6;
        locals.var_t0_dn7 = assign47970_e80953_d_n7;
        locals.var_t0_dn8 = assign47970_e80953_d_n8;
        locals.var_t0_dn9 = assign47970_e80953_d_n9;
        locals.var_t0_dn10 = assign47970_e80953_d_n10;
        locals.var_t0_dn11 = assign47970_e80953_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign47980_e80960, assign47980_e80960_d_n3, assign47980_e80960_d_n4, assign47980_e80960_d_n5, assign47980_e80960_d_n6, assign47980_e80960_d_n7, assign47980_e80960_d_n8, assign47980_e80960_d_n9, assign47980_e80960_d_n10, assign47980_e80960_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47980_e80958: f64 = (locals.var_qs_1 - locals.var_qdeff);
        (assign47980_e80958, (locals.var_qs_1_dn3 - locals.var_qdeff_dn3), (locals.var_qs_1_dn4 - locals.var_qdeff_dn4), (locals.var_qs_1_dn5 - locals.var_qdeff_dn5), (locals.var_qs_1_dn6 - locals.var_qdeff_dn6), (locals.var_qs_1_dn7 - locals.var_qdeff_dn7), (locals.var_qs_1_dn8 - locals.var_qdeff_dn8), (locals.var_qs_1_dn9 - locals.var_qdeff_dn9), (locals.var_qs_1_dn10 - locals.var_qdeff_dn10), (locals.var_qs_1_dn11 - locals.var_qdeff_dn11),)
    } else {
        (locals.var_dqsd, locals.var_dqsd_dn3, locals.var_dqsd_dn4, locals.var_dqsd_dn5, locals.var_dqsd_dn6, locals.var_dqsd_dn7, locals.var_dqsd_dn8, locals.var_dqsd_dn9, locals.var_dqsd_dn10, locals.var_dqsd_dn11,)
    }
};
        locals.var_dqsd = assign47980_e80960;
        locals.var_dqsd_dn3 = assign47980_e80960_d_n3;
        locals.var_dqsd_dn4 = assign47980_e80960_d_n4;
        locals.var_dqsd_dn5 = assign47980_e80960_d_n5;
        locals.var_dqsd_dn6 = assign47980_e80960_d_n6;
        locals.var_dqsd_dn7 = assign47980_e80960_d_n7;
        locals.var_dqsd_dn8 = assign47980_e80960_d_n8;
        locals.var_dqsd_dn9 = assign47980_e80960_d_n9;
        locals.var_dqsd_dn10 = assign47980_e80960_d_n10;
        locals.var_dqsd_dn11 = assign47980_e80960_d_n11;
        locals.var_dqsd_rv = 0.0;

        let (assign47990_e80969, assign47990_e80969_d_n3, assign47990_e80969_d_n4, assign47990_e80969_d_n5, assign47990_e80969_d_n6, assign47990_e80969_d_n7, assign47990_e80969_d_n8, assign47990_e80969_d_n9, assign47990_e80969_d_n10, assign47990_e80969_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign47990_e80965: f64 = (locals.var_t0 * locals.var_dqsd);
        let assign47990_e80967: f64 = (assign47990_e80965 * locals.var_dqsd);
        (assign47990_e80967, ((((locals.var_t0_dn3 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn3)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn3)), ((((locals.var_t0_dn4 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn4)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn4)), ((((locals.var_t0_dn5 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn5)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn5)), ((((locals.var_t0_dn6 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn6)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn6)), ((((locals.var_t0_dn7 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn7)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn7)), ((((locals.var_t0_dn8 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn8)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn8)), ((((locals.var_t0_dn9 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn9)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn9)), ((((locals.var_t0_dn10 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn10)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn10)), ((((locals.var_t0_dn11 * locals.var_dqsd) + (locals.var_t0 * locals.var_dqsd_dn11)) * locals.var_dqsd) + (assign47990_e80965 * locals.var_dqsd_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign47990_e80969;
        locals.var_t1_dn3 = assign47990_e80969_d_n3;
        locals.var_t1_dn4 = assign47990_e80969_d_n4;
        locals.var_t1_dn5 = assign47990_e80969_d_n5;
        locals.var_t1_dn6 = assign47990_e80969_d_n6;
        locals.var_t1_dn7 = assign47990_e80969_d_n7;
        locals.var_t1_dn8 = assign47990_e80969_d_n8;
        locals.var_t1_dn9 = assign47990_e80969_d_n9;
        locals.var_t1_dn10 = assign47990_e80969_d_n10;
        locals.var_t1_dn11 = assign47990_e80969_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign48000_e80978, assign48000_e80978_d_n3, assign48000_e80978_d_n4, assign48000_e80978_d_n5, assign48000_e80978_d_n6, assign48000_e80978_d_n7, assign48000_e80978_d_n8, assign48000_e80978_d_n9, assign48000_e80978_d_n10, assign48000_e80978_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48000_e80974: f64 = (locals.var_t1 + 1.0);
        let assign48000_e80976: f64 = (assign48000_e80974 - 0.001);
        (assign48000_e80976, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign48000_e80978;
        locals.var_t2_dn3 = assign48000_e80978_d_n3;
        locals.var_t2_dn4 = assign48000_e80978_d_n4;
        locals.var_t2_dn5 = assign48000_e80978_d_n5;
        locals.var_t2_dn6 = assign48000_e80978_d_n6;
        locals.var_t2_dn7 = assign48000_e80978_d_n7;
        locals.var_t2_dn8 = assign48000_e80978_d_n8;
        locals.var_t2_dn9 = assign48000_e80978_d_n9;
        locals.var_t2_dn10 = assign48000_e80978_d_n10;
        locals.var_t2_dn11 = assign48000_e80978_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign48010_e80995, assign48010_e80995_d_n3, assign48010_e80995_d_n4, assign48010_e80995_d_n5, assign48010_e80995_d_n6, assign48010_e80995_d_n7, assign48010_e80995_d_n8, assign48010_e80995_d_n9, assign48010_e80995_d_n10, assign48010_e80995_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48010_e80982: f64 = (-1.0);
        let assign48010_e80987: f64 = (locals.var_t2 * locals.var_t2);
        let assign48010_e80989: f64 = (assign48010_e80987 + 0.004);
        let assign48010_e80990: f64 = (assign48010_e80989).sqrt();
        let assign48010_e80991: f64 = (locals.var_t2 + assign48010_e80990);
        let assign48010_e80992: f64 = (0.5 * assign48010_e80991);
        let assign48010_e80993: f64 = (assign48010_e80982 + assign48010_e80992);
        (assign48010_e80993, (0.5 * (locals.var_t2_dn3 + (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn4 + (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn5 + (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn6 + (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn7 + (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn8 + (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn9 + (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn10 + (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign48010_e80990)))), (0.5 * (locals.var_t2_dn11 + (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign48010_e80990)))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign48010_e80995;
        locals.var_t3_dn3 = assign48010_e80995_d_n3;
        locals.var_t3_dn4 = assign48010_e80995_d_n4;
        locals.var_t3_dn5 = assign48010_e80995_d_n5;
        locals.var_t3_dn6 = assign48010_e80995_d_n6;
        locals.var_t3_dn7 = assign48010_e80995_d_n7;
        locals.var_t3_dn8 = assign48010_e80995_d_n8;
        locals.var_t3_dn9 = assign48010_e80995_d_n9;
        locals.var_t3_dn10 = assign48010_e80995_d_n10;
        locals.var_t3_dn11 = assign48010_e80995_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign48020_e81007, assign48020_e81007_d_n3, assign48020_e81007_d_n4, assign48020_e81007_d_n5, assign48020_e81007_d_n6, assign48020_e81007_d_n7, assign48020_e81007_d_n8, assign48020_e81007_d_n9, assign48020_e81007_d_n10, assign48020_e81007_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48020_e81002: f64 = (1.0 + locals.var_t3);
        let assign48020_e81003: f64 = (assign48020_e81002).sqrt();
        let assign48020_e81004: f64 = (1.0 + assign48020_e81003);
        let assign48020_e81005: f64 = (0.5 * assign48020_e81004);
        (assign48020_e81005, (0.5 * (locals.var_t3_dn3 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn4 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn5 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn6 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn7 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn8 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn9 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn10 / (2.0 * assign48020_e81003))), (0.5 * (locals.var_t3_dn11 / (2.0 * assign48020_e81003))),)
    } else {
        (locals.var_nsat, locals.var_nsat_dn3, locals.var_nsat_dn4, locals.var_nsat_dn5, locals.var_nsat_dn6, locals.var_nsat_dn7, locals.var_nsat_dn8, locals.var_nsat_dn9, locals.var_nsat_dn10, locals.var_nsat_dn11,)
    }
};
        locals.var_nsat = assign48020_e81007;
        locals.var_nsat_dn3 = assign48020_e81007_d_n3;
        locals.var_nsat_dn4 = assign48020_e81007_d_n4;
        locals.var_nsat_dn5 = assign48020_e81007_d_n5;
        locals.var_nsat_dn6 = assign48020_e81007_d_n6;
        locals.var_nsat_dn7 = assign48020_e81007_d_n7;
        locals.var_nsat_dn8 = assign48020_e81007_d_n8;
        locals.var_nsat_dn9 = assign48020_e81007_d_n9;
        locals.var_nsat_dn10 = assign48020_e81007_d_n10;
        locals.var_nsat_dn11 = assign48020_e81007_d_n11;
        locals.var_nsat_rv = 0.0;

        let (assign48030_e81035, assign48030_e81035_d_n3, assign48030_e81035_d_n4, assign48030_e81035_d_n5, assign48030_e81035_d_n6, assign48030_e81035_d_n7, assign48030_e81035_d_n8, assign48030_e81035_d_n9, assign48030_e81035_d_n10, assign48030_e81035_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48030_e81013: f64 = (locals.var_nsat + 1.0);
        let assign48030_e81016: f64 = (locals.var_nsat - 1.0);
        let assign48030_e81019: f64 = (locals.var_nsat - 1.0);
        let assign48030_e81020: f64 = (assign48030_e81016 * assign48030_e81019);
        let assign48030_e81023: f64 = (0.25 * 0.01);
        let assign48030_e81025: f64 = (assign48030_e81023 * 0.01);
        let assign48030_e81026: f64 = (assign48030_e81020 + assign48030_e81025);
        let assign48030_e81027: f64 = (assign48030_e81026).sqrt();
        let assign48030_e81028: f64 = (assign48030_e81013 - assign48030_e81027);
        let assign48030_e81029: f64 = (0.5 * assign48030_e81028);
        let assign48030_e81032: f64 = (0.25 * 0.01);
        let assign48030_e81033: f64 = (assign48030_e81029 + assign48030_e81032);
        (assign48030_e81033, (0.5 * (locals.var_nsat_dn3 - (((locals.var_nsat_dn3 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn3)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn4 - (((locals.var_nsat_dn4 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn4)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn5 - (((locals.var_nsat_dn5 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn5)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn6 - (((locals.var_nsat_dn6 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn6)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn7 - (((locals.var_nsat_dn7 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn7)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn8 - (((locals.var_nsat_dn8 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn8)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn9 - (((locals.var_nsat_dn9 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn9)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn10 - (((locals.var_nsat_dn10 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn10)) / (2.0 * assign48030_e81027)))), (0.5 * (locals.var_nsat_dn11 - (((locals.var_nsat_dn11 * assign48030_e81019) + (assign48030_e81016 * locals.var_nsat_dn11)) / (2.0 * assign48030_e81027)))),)
    } else {
        (locals.var_nsat, locals.var_nsat_dn3, locals.var_nsat_dn4, locals.var_nsat_dn5, locals.var_nsat_dn6, locals.var_nsat_dn7, locals.var_nsat_dn8, locals.var_nsat_dn9, locals.var_nsat_dn10, locals.var_nsat_dn11,)
    }
};
        locals.var_nsat = assign48030_e81035;
        locals.var_nsat_dn3 = assign48030_e81035_d_n3;
        locals.var_nsat_dn4 = assign48030_e81035_d_n4;
        locals.var_nsat_dn5 = assign48030_e81035_d_n5;
        locals.var_nsat_dn6 = assign48030_e81035_d_n6;
        locals.var_nsat_dn7 = assign48030_e81035_d_n7;
        locals.var_nsat_dn8 = assign48030_e81035_d_n8;
        locals.var_nsat_dn9 = assign48030_e81035_d_n9;
        locals.var_nsat_dn10 = assign48030_e81035_d_n10;
        locals.var_nsat_dn11 = assign48030_e81035_d_n11;
        locals.var_nsat_rv = 0.0;

        let (assign48040_e81042, assign48040_e81042_d_n3, assign48040_e81042_d_n4, assign48040_e81042_d_n5, assign48040_e81042_d_n6, assign48040_e81042_d_n7, assign48040_e81042_d_n8, assign48040_e81042_d_n9, assign48040_e81042_d_n10, assign48040_e81042_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48040_e81040: f64 = (locals.var_qs_1 + locals.var_qdeff);
        (assign48040_e81040, (locals.var_qs_1_dn3 + locals.var_qdeff_dn3), (locals.var_qs_1_dn4 + locals.var_qdeff_dn4), (locals.var_qs_1_dn5 + locals.var_qdeff_dn5), (locals.var_qs_1_dn6 + locals.var_qdeff_dn6), (locals.var_qs_1_dn7 + locals.var_qdeff_dn7), (locals.var_qs_1_dn8 + locals.var_qdeff_dn8), (locals.var_qs_1_dn9 + locals.var_qdeff_dn9), (locals.var_qs_1_dn10 + locals.var_qdeff_dn10), (locals.var_qs_1_dn11 + locals.var_qdeff_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48040_e81042;
        locals.var_t0_dn3 = assign48040_e81042_d_n3;
        locals.var_t0_dn4 = assign48040_e81042_d_n4;
        locals.var_t0_dn5 = assign48040_e81042_d_n5;
        locals.var_t0_dn6 = assign48040_e81042_d_n6;
        locals.var_t0_dn7 = assign48040_e81042_d_n7;
        locals.var_t0_dn8 = assign48040_e81042_d_n8;
        locals.var_t0_dn9 = assign48040_e81042_d_n9;
        locals.var_t0_dn10 = assign48040_e81042_d_n10;
        locals.var_t0_dn11 = assign48040_e81042_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign48050_e81049, assign48050_e81049_d_n3, assign48050_e81049_d_n4, assign48050_e81049_d_n5, assign48050_e81049_d_n6, assign48050_e81049_d_n7, assign48050_e81049_d_n8, assign48050_e81049_d_n9, assign48050_e81049_d_n10, assign48050_e81049_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48050_e81047: f64 = (locals.var_qs_1 - locals.var_qdeff);
        (assign48050_e81047, (locals.var_qs_1_dn3 - locals.var_qdeff_dn3), (locals.var_qs_1_dn4 - locals.var_qdeff_dn4), (locals.var_qs_1_dn5 - locals.var_qdeff_dn5), (locals.var_qs_1_dn6 - locals.var_qdeff_dn6), (locals.var_qs_1_dn7 - locals.var_qdeff_dn7), (locals.var_qs_1_dn8 - locals.var_qdeff_dn8), (locals.var_qs_1_dn9 - locals.var_qdeff_dn9), (locals.var_qs_1_dn10 - locals.var_qdeff_dn10), (locals.var_qs_1_dn11 - locals.var_qdeff_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign48050_e81049;
        locals.var_t1_dn3 = assign48050_e81049_d_n3;
        locals.var_t1_dn4 = assign48050_e81049_d_n4;
        locals.var_t1_dn5 = assign48050_e81049_d_n5;
        locals.var_t1_dn6 = assign48050_e81049_d_n6;
        locals.var_t1_dn7 = assign48050_e81049_d_n7;
        locals.var_t1_dn8 = assign48050_e81049_d_n8;
        locals.var_t1_dn9 = assign48050_e81049_d_n9;
        locals.var_t1_dn10 = assign48050_e81049_d_n10;
        locals.var_t1_dn11 = assign48050_e81049_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign48060_e81058, assign48060_e81058_d_n3, assign48060_e81058_d_n4, assign48060_e81058_d_n5, assign48060_e81058_d_n6, assign48060_e81058_d_n7, assign48060_e81058_d_n8, assign48060_e81058_d_n9, assign48060_e81058_d_n10, assign48060_e81058_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48060_e81055: f64 = (locals.var_t0 + locals.var_m0_t);
        let assign48060_e81056: f64 = (locals.var_t1 / assign48060_e81055);
        (assign48060_e81056, (((locals.var_t1_dn3 * assign48060_e81055) - (locals.var_t1 * locals.var_t0_dn3)) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn4 * assign48060_e81055) - (locals.var_t1 * (locals.var_t0_dn4 + locals.var_m0_t_dn4))) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn5 * assign48060_e81055) - (locals.var_t1 * (locals.var_t0_dn5 + locals.var_m0_t_dn5))) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn6 * assign48060_e81055) - (locals.var_t1 * locals.var_t0_dn6)) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn7 * assign48060_e81055) - (locals.var_t1 * locals.var_t0_dn7)) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn8 * assign48060_e81055) - (locals.var_t1 * locals.var_t0_dn8)) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn9 * assign48060_e81055) - (locals.var_t1 * locals.var_t0_dn9)) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn10 * assign48060_e81055) - (locals.var_t1 * locals.var_t0_dn10)) / (assign48060_e81055 * assign48060_e81055)), (((locals.var_t1_dn11 * assign48060_e81055) - (locals.var_t1 * locals.var_t0_dn11)) / (assign48060_e81055 * assign48060_e81055)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign48060_e81058;
        locals.var_t2_dn3 = assign48060_e81058_d_n3;
        locals.var_t2_dn4 = assign48060_e81058_d_n4;
        locals.var_t2_dn5 = assign48060_e81058_d_n5;
        locals.var_t2_dn6 = assign48060_e81058_d_n6;
        locals.var_t2_dn7 = assign48060_e81058_d_n7;
        locals.var_t2_dn8 = assign48060_e81058_d_n8;
        locals.var_t2_dn9 = assign48060_e81058_d_n9;
        locals.var_t2_dn10 = assign48060_e81058_d_n10;
        locals.var_t2_dn11 = assign48060_e81058_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign48070_e81067, assign48070_e81067_d_n3, assign48070_e81067_d_n4, assign48070_e81067_d_n5, assign48070_e81067_d_n6, assign48070_e81067_d_n7, assign48070_e81067_d_n8, assign48070_e81067_d_n9, assign48070_e81067_d_n10, assign48070_e81067_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48070_e81063: f64 = (locals.var_k0_t * locals.var_t2);
        let assign48070_e81065: f64 = (assign48070_e81063 * locals.var_t2);
        (assign48070_e81065, (((locals.var_k0_t * locals.var_t2_dn3) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn3)), ((((locals.var_k0_t_dn4 * locals.var_t2) + (locals.var_k0_t * locals.var_t2_dn4)) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn4)), ((((locals.var_k0_t_dn5 * locals.var_t2) + (locals.var_k0_t * locals.var_t2_dn5)) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn5)), (((locals.var_k0_t * locals.var_t2_dn6) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn6)), (((locals.var_k0_t * locals.var_t2_dn7) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn7)), (((locals.var_k0_t * locals.var_t2_dn8) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn8)), (((locals.var_k0_t * locals.var_t2_dn9) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn9)), (((locals.var_k0_t * locals.var_t2_dn10) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn10)), (((locals.var_k0_t * locals.var_t2_dn11) * locals.var_t2) + (assign48070_e81063 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign48070_e81067;
        locals.var_t3_dn3 = assign48070_e81067_d_n3;
        locals.var_t3_dn4 = assign48070_e81067_d_n4;
        locals.var_t3_dn5 = assign48070_e81067_d_n5;
        locals.var_t3_dn6 = assign48070_e81067_d_n6;
        locals.var_t3_dn7 = assign48070_e81067_d_n7;
        locals.var_t3_dn8 = assign48070_e81067_d_n8;
        locals.var_t3_dn9 = assign48070_e81067_d_n9;
        locals.var_t3_dn10 = assign48070_e81067_d_n10;
        locals.var_t3_dn11 = assign48070_e81067_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign48080_e81074, assign48080_e81074_d_n3, assign48080_e81074_d_n4, assign48080_e81074_d_n5, assign48080_e81074_d_n6, assign48080_e81074_d_n7, assign48080_e81074_d_n8, assign48080_e81074_d_n9, assign48080_e81074_d_n10, assign48080_e81074_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48080_e81072: f64 = (1.0 + locals.var_t3);
        (assign48080_e81072, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    } else {
        (locals.var_mnud, locals.var_mnud_dn3, locals.var_mnud_dn4, locals.var_mnud_dn5, locals.var_mnud_dn6, locals.var_mnud_dn7, locals.var_mnud_dn8, locals.var_mnud_dn9, locals.var_mnud_dn10, locals.var_mnud_dn11,)
    }
};
        locals.var_mnud = assign48080_e81074;
        locals.var_mnud_dn3 = assign48080_e81074_d_n3;
        locals.var_mnud_dn4 = assign48080_e81074_d_n4;
        locals.var_mnud_dn5 = assign48080_e81074_d_n5;
        locals.var_mnud_dn6 = assign48080_e81074_d_n6;
        locals.var_mnud_dn7 = assign48080_e81074_d_n7;
        locals.var_mnud_dn8 = assign48080_e81074_d_n8;
        locals.var_mnud_dn9 = assign48080_e81074_d_n9;
        locals.var_mnud_dn10 = assign48080_e81074_d_n10;
        locals.var_mnud_dn11 = assign48080_e81074_d_n11;
        locals.var_mnud_rv = 0.0;

        let (assign48090_e81097, assign48090_e81097_d_n3, assign48090_e81097_d_n4, assign48090_e81097_d_n5, assign48090_e81097_d_n6, assign48090_e81097_d_n7, assign48090_e81097_d_n8, assign48090_e81097_d_n9, assign48090_e81097_d_n10, assign48090_e81097_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48090_e81082: f64 = (locals.var_c0sisat_t * locals.var_t1);
        let assign48090_e81084: f64 = (assign48090_e81082 * locals.var_t1);
        let assign48090_e81085: f64 = (locals.var_c0si_t + assign48090_e81084);
        let assign48090_e81086: f64 = (0.0_f64).max(assign48090_e81085);
        let assign48090_e81088: f64 = (assign48090_e81086 * locals.var_t0);
        let assign48090_e81091: f64 = (2.0 * locals.var_n);
        let assign48090_e81093: f64 = (assign48090_e81091 * locals.var_vtm);
        let assign48090_e81094: f64 = (assign48090_e81088 + assign48090_e81093);
        let assign48090_e81095: f64 = (locals.var_c0_t / assign48090_e81094);
        (assign48090_e81095, (-((locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn3) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn3)) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn3)) + ((2.0 * locals.var_n_dn3) * locals.var_vtm))) / (assign48090_e81094 * assign48090_e81094))), (((locals.var_c0_t_dn4 * assign48090_e81094) - (locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (locals.var_c0si_t_dn4 + ((((locals.var_c0sisat_t_dn4 * locals.var_t1) + (locals.var_c0sisat_t * locals.var_t1_dn4)) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn4))) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn4)) + (((2.0 * locals.var_n_dn4) * locals.var_vtm) + (assign48090_e81091 * locals.var_vtm_dn4))))) / (assign48090_e81094 * assign48090_e81094)), (((locals.var_c0_t_dn5 * assign48090_e81094) - (locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (locals.var_c0si_t_dn5 + ((((locals.var_c0sisat_t_dn5 * locals.var_t1) + (locals.var_c0sisat_t * locals.var_t1_dn5)) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn5))) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn5)) + (((2.0 * locals.var_n_dn5) * locals.var_vtm) + (assign48090_e81091 * locals.var_vtm_dn5))))) / (assign48090_e81094 * assign48090_e81094)), (-((locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn6) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn6)) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn6)) + ((2.0 * locals.var_n_dn6) * locals.var_vtm))) / (assign48090_e81094 * assign48090_e81094))), (-((locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn7) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn7)) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn7)) + ((2.0 * locals.var_n_dn7) * locals.var_vtm))) / (assign48090_e81094 * assign48090_e81094))), (-((locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn8) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn8)) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn8)) + ((2.0 * locals.var_n_dn8) * locals.var_vtm))) / (assign48090_e81094 * assign48090_e81094))), (-((locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn9) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn9)) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn9)) + ((2.0 * locals.var_n_dn9) * locals.var_vtm))) / (assign48090_e81094 * assign48090_e81094))), (-((locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn10) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn10)) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn10)) + ((2.0 * locals.var_n_dn10) * locals.var_vtm))) / (assign48090_e81094 * assign48090_e81094))), (-((locals.var_c0_t * (((if 0.0 >= assign48090_e81085 { 0.0 } else { (((locals.var_c0sisat_t * locals.var_t1_dn11) * locals.var_t1) + (assign48090_e81082 * locals.var_t1_dn11)) } * locals.var_t0) + (assign48090_e81086 * locals.var_t0_dn11)) + ((2.0 * locals.var_n_dn11) * locals.var_vtm))) / (assign48090_e81094 * assign48090_e81094))),)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11,)
    }
};
        locals.var_t9 = assign48090_e81097;
        locals.var_t9_dn3 = assign48090_e81097_d_n3;
        locals.var_t9_dn4 = assign48090_e81097_d_n4;
        locals.var_t9_dn5 = assign48090_e81097_d_n5;
        locals.var_t9_dn6 = assign48090_e81097_d_n6;
        locals.var_t9_dn7 = assign48090_e81097_d_n7;
        locals.var_t9_dn8 = assign48090_e81097_d_n8;
        locals.var_t9_dn9 = assign48090_e81097_d_n9;
        locals.var_t9_dn10 = assign48090_e81097_d_n10;
        locals.var_t9_dn11 = assign48090_e81097_d_n11;
        locals.var_t9_rv = 0.0;

        let (assign48100_e81104, assign48100_e81104_d_n3, assign48100_e81104_d_n4, assign48100_e81104_d_n5, assign48100_e81104_d_n6, assign48100_e81104_d_n7, assign48100_e81104_d_n8, assign48100_e81104_d_n9, assign48100_e81104_d_n10, assign48100_e81104_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48100_e81101: f64 = (-locals.var_t9);
        let assign48100_e81102: f64 = { let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48100_e81102, ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn3)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn4)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn5)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn6)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn7)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn8)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn9)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn10)), ({ let limited_exp_arg = assign48100_e81101; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t9_dn11)),)
    } else {
        (locals.var_mnud1, locals.var_mnud1_dn3, locals.var_mnud1_dn4, locals.var_mnud1_dn5, locals.var_mnud1_dn6, locals.var_mnud1_dn7, locals.var_mnud1_dn8, locals.var_mnud1_dn9, locals.var_mnud1_dn10, locals.var_mnud1_dn11,)
    }
};
        locals.var_mnud1 = assign48100_e81104;
        locals.var_mnud1_dn3 = assign48100_e81104_d_n3;
        locals.var_mnud1_dn4 = assign48100_e81104_d_n4;
        locals.var_mnud1_dn5 = assign48100_e81104_d_n5;
        locals.var_mnud1_dn6 = assign48100_e81104_d_n6;
        locals.var_mnud1_dn7 = assign48100_e81104_d_n7;
        locals.var_mnud1_dn8 = assign48100_e81104_d_n8;
        locals.var_mnud1_dn9 = assign48100_e81104_d_n9;
        locals.var_mnud1_dn10 = assign48100_e81104_d_n10;
        locals.var_mnud1_dn11 = assign48100_e81104_d_n11;
        locals.var_mnud1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_168(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign48110_e81113, assign48110_e81113_d_n3, assign48110_e81113_d_n4, assign48110_e81113_d_n5, assign48110_e81113_d_n6, assign48110_e81113_d_n7, assign48110_e81113_d_n8, assign48110_e81113_d_n9, assign48110_e81113_d_n10, assign48110_e81113_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48110_e81109: f64 = (locals.var_dmob * locals.var_dvsat);
        let assign48110_e81111: f64 = (assign48110_e81109 * locals.var_dr);
        (assign48110_e81111, ((((locals.var_dmob_dn3 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn3)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn3)), ((((locals.var_dmob_dn4 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn4)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn4)), ((((locals.var_dmob_dn5 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn5)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn5)), ((((locals.var_dmob_dn6 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn6)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn6)), ((((locals.var_dmob_dn7 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn7)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn7)), ((((locals.var_dmob_dn8 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn8)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn8)), ((((locals.var_dmob_dn9 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn9)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn9)), ((((locals.var_dmob_dn10 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn10)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn10)), ((((locals.var_dmob_dn11 * locals.var_dvsat) + (locals.var_dmob * locals.var_dvsat_dn11)) * locals.var_dr) + (assign48110_e81109 * locals.var_dr_dn11)),)
    } else {
        (locals.var_dtot, locals.var_dtot_dn3, locals.var_dtot_dn4, locals.var_dtot_dn5, locals.var_dtot_dn6, locals.var_dtot_dn7, locals.var_dtot_dn8, locals.var_dtot_dn9, locals.var_dtot_dn10, locals.var_dtot_dn11,)
    }
};
        locals.var_dtot = assign48110_e81113;
        locals.var_dtot_dn3 = assign48110_e81113_d_n3;
        locals.var_dtot_dn4 = assign48110_e81113_d_n4;
        locals.var_dtot_dn5 = assign48110_e81113_d_n5;
        locals.var_dtot_dn6 = assign48110_e81113_d_n6;
        locals.var_dtot_dn7 = assign48110_e81113_d_n7;
        locals.var_dtot_dn8 = assign48110_e81113_d_n8;
        locals.var_dtot_dn9 = assign48110_e81113_d_n9;
        locals.var_dtot_dn10 = assign48110_e81113_d_n10;
        locals.var_dtot_dn11 = assign48110_e81113_d_n11;
        locals.var_dtot_rv = 0.0;

        let (assign48120_e81120, assign48120_e81120_d_n3, assign48120_e81120_d_n4, assign48120_e81120_d_n5, assign48120_e81120_d_n6, assign48120_e81120_d_n7, assign48120_e81120_d_n8, assign48120_e81120_d_n9, assign48120_e81120_d_n10, assign48120_e81120_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48120_e81118: f64 = (locals.var_u0_a / locals.var_dtot);
        (assign48120_e81118, (((locals.var_u0_a_dn3 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn3)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn4 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn4)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn5 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn5)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn6 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn6)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn7 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn7)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn8 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn8)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn9 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn9)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn10 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn10)) / (locals.var_dtot * locals.var_dtot)), (((locals.var_u0_a_dn11 * locals.var_dtot) - (locals.var_u0_a * locals.var_dtot_dn11)) / (locals.var_dtot * locals.var_dtot)),)
    } else {
        (locals.var_ueff, locals.var_ueff_dn3, locals.var_ueff_dn4, locals.var_ueff_dn5, locals.var_ueff_dn6, locals.var_ueff_dn7, locals.var_ueff_dn8, locals.var_ueff_dn9, locals.var_ueff_dn10, locals.var_ueff_dn11,)
    }
};
        locals.var_ueff = assign48120_e81120;
        locals.var_ueff_dn3 = assign48120_e81120_d_n3;
        locals.var_ueff_dn4 = assign48120_e81120_d_n4;
        locals.var_ueff_dn5 = assign48120_e81120_d_n5;
        locals.var_ueff_dn6 = assign48120_e81120_d_n6;
        locals.var_ueff_dn7 = assign48120_e81120_d_n7;
        locals.var_ueff_dn8 = assign48120_e81120_d_n8;
        locals.var_ueff_dn9 = assign48120_e81120_d_n9;
        locals.var_ueff_dn10 = assign48120_e81120_d_n10;
        locals.var_ueff_dn11 = assign48120_e81120_d_n11;
        locals.var_ueff_rv = 0.0;

        let (assign48130_e81159, assign48130_e81159_d_n3, assign48130_e81159_d_n4, assign48130_e81159_d_n5, assign48130_e81159_d_n6, assign48130_e81159_d_n7, assign48130_e81159_d_n8, assign48130_e81159_d_n9, assign48130_e81159_d_n10, assign48130_e81159_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48130_e81125: f64 = (2.0 * p.p2);
        let assign48130_e81127: f64 = (assign48130_e81125 * locals.var_nq);
        let assign48130_e81129: f64 = (assign48130_e81127 * locals.var_ueff);
        let assign48130_e81131: f64 = (assign48130_e81129 * locals.var_weff);
        let assign48130_e81133: f64 = (assign48130_e81131 / locals.var_leff);
        let assign48130_e81135: f64 = (assign48130_e81133 * locals.var_cox);
        let assign48130_e81137: f64 = (assign48130_e81135 * locals.var_nvt);
        let assign48130_e81139: f64 = (assign48130_e81137 * locals.var_nvt);
        let assign48130_e81142: f64 = (locals.var_qs_1 - locals.var_qdeff);
        let assign48130_e81145: f64 = (1.0 + locals.var_qs_1);
        let assign48130_e81147: f64 = (assign48130_e81145 + locals.var_qdeff);
        let assign48130_e81148: f64 = (assign48130_e81142 * assign48130_e81147);
        let assign48130_e81149: f64 = (assign48130_e81139 * assign48130_e81148);
        let assign48130_e81151: f64 = (assign48130_e81149 * locals.var_moc);
        let assign48130_e81153: f64 = (assign48130_e81151 / locals.var_nsat);
        let assign48130_e81155: f64 = (assign48130_e81153 * locals.var_mnud);
        let assign48130_e81157: f64 = (assign48130_e81155 * locals.var_mnud1);
        (assign48130_e81157, (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn3) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn3)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn3)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn3)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn3 - locals.var_qdeff_dn3) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn3 + locals.var_qdeff_dn3))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn3)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn3)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn3)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn3)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn4) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn4)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn4)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn4)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn4 - locals.var_qdeff_dn4) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn4 + locals.var_qdeff_dn4))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn4)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn4)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn4)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn4)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn5) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn5)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn5)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn5)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn5 - locals.var_qdeff_dn5) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn5 + locals.var_qdeff_dn5))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn5)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn5)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn5)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn5)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn6) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn6)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn6)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn6)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn6 - locals.var_qdeff_dn6) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn6 + locals.var_qdeff_dn6))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn6)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn6)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn6)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn6)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn7) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn7)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn7)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn7)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn7 - locals.var_qdeff_dn7) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn7 + locals.var_qdeff_dn7))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn7)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn7)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn7)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn7)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn8) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn8)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn8)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn8)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn8 - locals.var_qdeff_dn8) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn8 + locals.var_qdeff_dn8))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn8)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn8)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn8)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn8)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn9) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn9)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn9)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn9)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn9 - locals.var_qdeff_dn9) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn9 + locals.var_qdeff_dn9))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn9)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn9)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn9)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn9)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn10) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn10)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn10)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn10)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn10 - locals.var_qdeff_dn10) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn10 + locals.var_qdeff_dn10))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn10)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn10)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn10)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn10)), (((((((((((((((((((((assign48130_e81125 * locals.var_nq_dn11) * locals.var_ueff) + (assign48130_e81127 * locals.var_ueff_dn11)) * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_nvt) + (assign48130_e81135 * locals.var_nvt_dn11)) * locals.var_nvt) + (assign48130_e81137 * locals.var_nvt_dn11)) * assign48130_e81148) + (assign48130_e81139 * (((locals.var_qs_1_dn11 - locals.var_qdeff_dn11) * assign48130_e81147) + (assign48130_e81142 * (locals.var_qs_1_dn11 + locals.var_qdeff_dn11))))) * locals.var_moc) + (assign48130_e81149 * locals.var_moc_dn11)) * locals.var_nsat) - (assign48130_e81151 * locals.var_nsat_dn11)) / (locals.var_nsat * locals.var_nsat)) * locals.var_mnud) + (assign48130_e81153 * locals.var_mnud_dn11)) * locals.var_mnud1) + (assign48130_e81155 * locals.var_mnud1_dn11)),)
    } else {
        (locals.var_ids, locals.var_ids_dn3, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11,)
    }
};
        locals.var_ids = assign48130_e81159;
        locals.var_ids_dn3 = assign48130_e81159_d_n3;
        locals.var_ids_dn4 = assign48130_e81159_d_n4;
        locals.var_ids_dn5 = assign48130_e81159_d_n5;
        locals.var_ids_dn6 = assign48130_e81159_d_n6;
        locals.var_ids_dn7 = assign48130_e81159_d_n7;
        locals.var_ids_dn8 = assign48130_e81159_d_n8;
        locals.var_ids_dn9 = assign48130_e81159_d_n9;
        locals.var_ids_dn10 = assign48130_e81159_d_n10;
        locals.var_ids_dn11 = assign48130_e81159_d_n11;
        locals.var_ids_rv = 0.0;

        let (assign48140_e81166, assign48140_e81166_d_n3, assign48140_e81166_d_n4, assign48140_e81166_d_n5, assign48140_e81166_d_n6, assign48140_e81166_d_n7, assign48140_e81166_d_n8, assign48140_e81166_d_n9, assign48140_e81166_d_n10, assign48140_e81166_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48140_e81164: f64 = (locals.var_ids * p.p26);
        (assign48140_e81164, (locals.var_ids_dn3 * p.p26), (locals.var_ids_dn4 * p.p26), (locals.var_ids_dn5 * p.p26), (locals.var_ids_dn6 * p.p26), (locals.var_ids_dn7 * p.p26), (locals.var_ids_dn8 * p.p26), (locals.var_ids_dn9 * p.p26), (locals.var_ids_dn10 * p.p26), (locals.var_ids_dn11 * p.p26),)
    } else {
        (locals.var_ids, locals.var_ids_dn3, locals.var_ids_dn4, locals.var_ids_dn5, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn8, locals.var_ids_dn9, locals.var_ids_dn10, locals.var_ids_dn11,)
    }
};
        locals.var_ids = assign48140_e81166;
        locals.var_ids_dn3 = assign48140_e81166_d_n3;
        locals.var_ids_dn4 = assign48140_e81166_d_n4;
        locals.var_ids_dn5 = assign48140_e81166_d_n5;
        locals.var_ids_dn6 = assign48140_e81166_d_n6;
        locals.var_ids_dn7 = assign48140_e81166_d_n7;
        locals.var_ids_dn8 = assign48140_e81166_d_n8;
        locals.var_ids_dn9 = assign48140_e81166_d_n9;
        locals.var_ids_dn10 = assign48140_e81166_d_n10;
        locals.var_ids_dn11 = assign48140_e81166_d_n11;
        locals.var_ids_rv = 0.0;

        let (assign48150_e81171, assign48150_e81171_d_n3, assign48150_e81171_d_n4, assign48150_e81171_d_n5, assign48150_e81171_d_n6, assign48150_e81171_d_n7, assign48150_e81171_d_n8, assign48150_e81171_d_n9, assign48150_e81171_d_n10, assign48150_e81171_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_gcrg, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11,)
    }
};
        locals.var_gcrg = assign48150_e81171;
        locals.var_gcrg_dn3 = assign48150_e81171_d_n3;
        locals.var_gcrg_dn4 = assign48150_e81171_d_n4;
        locals.var_gcrg_dn5 = assign48150_e81171_d_n5;
        locals.var_gcrg_dn6 = assign48150_e81171_d_n6;
        locals.var_gcrg_dn7 = assign48150_e81171_d_n7;
        locals.var_gcrg_dn8 = assign48150_e81171_d_n8;
        locals.var_gcrg_dn9 = assign48150_e81171_d_n9;
        locals.var_gcrg_dn10 = assign48150_e81171_d_n10;
        locals.var_gcrg_dn11 = assign48150_e81171_d_n11;
        locals.var_gcrg_rv = 0.0;

        let assign48160_e81174: f64 = if p.p7 > 1.0 { 1.0 } else { 0.0 };
        locals.var_guard741 = assign48160_e81174;
        locals.var_guard741_rv = 0.0;

        let (assign48170_e81189, assign48170_e81189_d_n3, assign48170_e81189_d_n4, assign48170_e81189_d_n5, assign48170_e81189_d_n6, assign48170_e81189_d_n7, assign48170_e81189_d_n8, assign48170_e81189_d_n9, assign48170_e81189_d_n10, assign48170_e81189_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign48170_e81181: f64 = (locals.var_ueff * locals.var_weff);
        let assign48170_e81183: f64 = (assign48170_e81181 / locals.var_leff);
        let assign48170_e81185: f64 = (assign48170_e81183 * locals.var_cox);
        let assign48170_e81187: f64 = (assign48170_e81185 * locals.var_qia);
        (assign48170_e81187, (((((locals.var_ueff_dn3 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn3)), (((((locals.var_ueff_dn4 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn4)), (((((locals.var_ueff_dn5 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn5)), (((((locals.var_ueff_dn6 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn6)), (((((locals.var_ueff_dn7 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn7)), (((((locals.var_ueff_dn8 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn8)), (((((locals.var_ueff_dn9 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn9)), (((((locals.var_ueff_dn10 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn10)), (((((locals.var_ueff_dn11 * locals.var_weff) / locals.var_leff) * locals.var_cox) * locals.var_qia) + (assign48170_e81185 * locals.var_qia_dn11)),)
    } else {
        (locals.var_idsovvds, locals.var_idsovvds_dn3, locals.var_idsovvds_dn4, locals.var_idsovvds_dn5, locals.var_idsovvds_dn6, locals.var_idsovvds_dn7, locals.var_idsovvds_dn8, locals.var_idsovvds_dn9, locals.var_idsovvds_dn10, locals.var_idsovvds_dn11,)
    }
};
        locals.var_idsovvds = assign48170_e81189;
        locals.var_idsovvds_dn3 = assign48170_e81189_d_n3;
        locals.var_idsovvds_dn4 = assign48170_e81189_d_n4;
        locals.var_idsovvds_dn5 = assign48170_e81189_d_n5;
        locals.var_idsovvds_dn6 = assign48170_e81189_d_n6;
        locals.var_idsovvds_dn7 = assign48170_e81189_d_n7;
        locals.var_idsovvds_dn8 = assign48170_e81189_d_n8;
        locals.var_idsovvds_dn9 = assign48170_e81189_d_n9;
        locals.var_idsovvds_dn10 = assign48170_e81189_d_n10;
        locals.var_idsovvds_dn11 = assign48170_e81189_d_n11;
        locals.var_idsovvds_rv = 0.0;

        let (assign48180_e81198, assign48180_e81198_d_n3, assign48180_e81198_d_n4, assign48180_e81198_d_n5, assign48180_e81198_d_n6, assign48180_e81198_d_n7, assign48180_e81198_d_n8, assign48180_e81198_d_n9, assign48180_e81198_d_n10, assign48180_e81198_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign48180_e81196: f64 = (p.p1009 * locals.var_vt);
        (assign48180_e81196, 0.0, (p.p1009 * locals.var_vt_dn4), (p.p1009 * locals.var_vt_dn5), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t9, locals.var_t9_dn3, locals.var_t9_dn4, locals.var_t9_dn5, locals.var_t9_dn6, locals.var_t9_dn7, locals.var_t9_dn8, locals.var_t9_dn9, locals.var_t9_dn10, locals.var_t9_dn11,)
    }
};
        locals.var_t9 = assign48180_e81198;
        locals.var_t9_dn3 = assign48180_e81198_d_n3;
        locals.var_t9_dn4 = assign48180_e81198_d_n4;
        locals.var_t9_dn5 = assign48180_e81198_d_n5;
        locals.var_t9_dn6 = assign48180_e81198_d_n6;
        locals.var_t9_dn7 = assign48180_e81198_d_n7;
        locals.var_t9_dn8 = assign48180_e81198_d_n8;
        locals.var_t9_dn9 = assign48180_e81198_d_n9;
        locals.var_t9_dn10 = assign48180_e81198_d_n10;
        locals.var_t9_dn11 = assign48180_e81198_d_n11;
        locals.var_t9_rv = 0.0;

        let (assign48190_e81213, assign48190_e81213_d_n3, assign48190_e81213_d_n4, assign48190_e81213_d_n5, assign48190_e81213_d_n6, assign48190_e81213_d_n7, assign48190_e81213_d_n8, assign48190_e81213_d_n9, assign48190_e81213_d_n10, assign48190_e81213_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign48190_e81205: f64 = (locals.var_t9 * locals.var_ueff);
        let assign48190_e81207: f64 = (assign48190_e81205 * locals.var_weff);
        let assign48190_e81209: f64 = (assign48190_e81207 / locals.var_leff);
        let assign48190_e81211: f64 = (assign48190_e81209 * locals.var_cox);
        (assign48190_e81211, (((((locals.var_t9_dn3 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn3)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn4 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn4)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn5 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn5)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn6 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn6)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn7 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn7)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn8 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn8)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn9 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn9)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn10 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn10)) * locals.var_weff) / locals.var_leff) * locals.var_cox), (((((locals.var_t9_dn11 * locals.var_ueff) + (locals.var_t9 * locals.var_ueff_dn11)) * locals.var_weff) / locals.var_leff) * locals.var_cox),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48190_e81213;
        locals.var_t0_dn3 = assign48190_e81213_d_n3;
        locals.var_t0_dn4 = assign48190_e81213_d_n4;
        locals.var_t0_dn5 = assign48190_e81213_d_n5;
        locals.var_t0_dn6 = assign48190_e81213_d_n6;
        locals.var_t0_dn7 = assign48190_e81213_d_n7;
        locals.var_t0_dn8 = assign48190_e81213_d_n8;
        locals.var_t0_dn9 = assign48190_e81213_d_n9;
        locals.var_t0_dn10 = assign48190_e81213_d_n10;
        locals.var_t0_dn11 = assign48190_e81213_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign48200_e81226, assign48200_e81226_d_n3, assign48200_e81226_d_n4, assign48200_e81226_d_n5, assign48200_e81226_d_n6, assign48200_e81226_d_n7, assign48200_e81226_d_n8, assign48200_e81226_d_n9, assign48200_e81226_d_n10, assign48200_e81226_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) {
        let assign48200_e81220: f64 = (p.p1008 * p.p2);
        let assign48200_e81223: f64 = (locals.var_t0 + locals.var_idsovvds);
        let assign48200_e81224: f64 = (assign48200_e81220 * assign48200_e81223);
        (assign48200_e81224, (assign48200_e81220 * (locals.var_t0_dn3 + locals.var_idsovvds_dn3)), (assign48200_e81220 * (locals.var_t0_dn4 + locals.var_idsovvds_dn4)), (assign48200_e81220 * (locals.var_t0_dn5 + locals.var_idsovvds_dn5)), (assign48200_e81220 * (locals.var_t0_dn6 + locals.var_idsovvds_dn6)), (assign48200_e81220 * (locals.var_t0_dn7 + locals.var_idsovvds_dn7)), (assign48200_e81220 * (locals.var_t0_dn8 + locals.var_idsovvds_dn8)), (assign48200_e81220 * (locals.var_t0_dn9 + locals.var_idsovvds_dn9)), (assign48200_e81220 * (locals.var_t0_dn10 + locals.var_idsovvds_dn10)), (assign48200_e81220 * (locals.var_t0_dn11 + locals.var_idsovvds_dn11)),)
    } else {
        (locals.var_gcrg, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11,)
    }
};
        locals.var_gcrg = assign48200_e81226;
        locals.var_gcrg_dn3 = assign48200_e81226_d_n3;
        locals.var_gcrg_dn4 = assign48200_e81226_d_n4;
        locals.var_gcrg_dn5 = assign48200_e81226_d_n5;
        locals.var_gcrg_dn6 = assign48200_e81226_d_n6;
        locals.var_gcrg_dn7 = assign48200_e81226_d_n7;
        locals.var_gcrg_dn8 = assign48200_e81226_d_n8;
        locals.var_gcrg_dn9 = assign48200_e81226_d_n9;
        locals.var_gcrg_dn10 = assign48200_e81226_d_n10;
        locals.var_gcrg_dn11 = assign48200_e81226_d_n11;
        locals.var_gcrg_rv = 0.0;

        let assign48210_e81229: f64 = if p.p7 == 2.0 { 1.0 } else { 0.0 };
        locals.var_guard742 = assign48210_e81229;
        locals.var_guard742_rv = 0.0;

        let (assign48220_e81240,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard742 != 0.0)) {
        let assign48220_e81238: f64 = (1.0 / locals.var_grgeltd);
        (assign48220_e81238,)
    } else {
        (locals.var_rgeltd,)
    }
};
        locals.var_rgeltd = assign48220_e81240;
        locals.var_rgeltd_rv = 0.0;

        let assign48230_e81243: f64 = if locals.var_rgeltd < p.p1347 { 1.0 } else { 0.0 };
        locals.var_guard743 = assign48230_e81243;
        locals.var_guard743_rv = 0.0;

        let (assign48240_e81254,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard742 != 0.0)) && (locals.var_guard743 != 0.0)) {
        (p.p1347,)
    } else {
        (locals.var_rgeltd,)
    }
};
        locals.var_rgeltd = assign48240_e81254;
        locals.var_rgeltd_rv = 0.0;

        let (assign48250_e81267,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard742 != 0.0)) && (locals.var_guard743 != 0.0)) {
        let assign48250_e81265: f64 = (1.0 / locals.var_rgeltd);
        (assign48250_e81265,)
    } else {
        (locals.var_grgeltd,)
    }
};
        locals.var_grgeltd = assign48250_e81267;
        locals.var_grgeltd_rv = 0.0;

        let (assign48260_e81278, assign48260_e81278_d_n3, assign48260_e81278_d_n4, assign48260_e81278_d_n5, assign48260_e81278_d_n6, assign48260_e81278_d_n7, assign48260_e81278_d_n8, assign48260_e81278_d_n9, assign48260_e81278_d_n10, assign48260_e81278_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard742 != 0.0)) {
        let assign48260_e81276: f64 = (locals.var_grgeltd + locals.var_gcrg);
        (assign48260_e81276, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11,)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48260_e81278;
        locals.var_t11_dn3 = assign48260_e81278_d_n3;
        locals.var_t11_dn4 = assign48260_e81278_d_n4;
        locals.var_t11_dn5 = assign48260_e81278_d_n5;
        locals.var_t11_dn6 = assign48260_e81278_d_n6;
        locals.var_t11_dn7 = assign48260_e81278_d_n7;
        locals.var_t11_dn8 = assign48260_e81278_d_n8;
        locals.var_t11_dn9 = assign48260_e81278_d_n9;
        locals.var_t11_dn10 = assign48260_e81278_d_n10;
        locals.var_t11_dn11 = assign48260_e81278_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign48270_e81291, assign48270_e81291_d_n3, assign48270_e81291_d_n4, assign48270_e81291_d_n5, assign48270_e81291_d_n6, assign48270_e81291_d_n7, assign48270_e81291_d_n8, assign48270_e81291_d_n9, assign48270_e81291_d_n10, assign48270_e81291_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard741 != 0.0)) && (locals.var_guard742 != 0.0)) {
        let assign48270_e81287: f64 = (locals.var_grgeltd * locals.var_gcrg);
        let assign48270_e81289: f64 = (assign48270_e81287 / locals.var_t11);
        (assign48270_e81289, ((((locals.var_grgeltd * locals.var_gcrg_dn3) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn3)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn4) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn4)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn5) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn5)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn6) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn6)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn7) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn7)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn8) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn8)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn9) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn9)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn10) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn10)) / (locals.var_t11 * locals.var_t11)), ((((locals.var_grgeltd * locals.var_gcrg_dn11) * locals.var_t11) - (assign48270_e81287 * locals.var_t11_dn11)) / (locals.var_t11 * locals.var_t11)),)
    } else {
        (locals.var_gcrg, locals.var_gcrg_dn3, locals.var_gcrg_dn4, locals.var_gcrg_dn5, locals.var_gcrg_dn6, locals.var_gcrg_dn7, locals.var_gcrg_dn8, locals.var_gcrg_dn9, locals.var_gcrg_dn10, locals.var_gcrg_dn11,)
    }
};
        locals.var_gcrg = assign48270_e81291;
        locals.var_gcrg_dn3 = assign48270_e81291_d_n3;
        locals.var_gcrg_dn4 = assign48270_e81291_d_n4;
        locals.var_gcrg_dn5 = assign48270_e81291_d_n5;
        locals.var_gcrg_dn6 = assign48270_e81291_d_n6;
        locals.var_gcrg_dn7 = assign48270_e81291_d_n7;
        locals.var_gcrg_dn8 = assign48270_e81291_d_n8;
        locals.var_gcrg_dn9 = assign48270_e81291_d_n9;
        locals.var_gcrg_dn10 = assign48270_e81291_d_n10;
        locals.var_gcrg_dn11 = assign48270_e81291_d_n11;
        locals.var_gcrg_rv = 0.0;

        let (assign48280_e81300,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48280_e81296: f64 = (locals.var_weff / p.p1373);
        let assign48280_e81298: f64 = (assign48280_e81296 + p.p1377);
        (assign48280_e81298,)
    } else {
        (locals.var_wdiod,)
    }
};
        locals.var_wdiod = assign48280_e81300;
        locals.var_wdiod_rv = 0.0;

        let (assign48290_e81309,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48290_e81305: f64 = (locals.var_weff / p.p1373);
        let assign48290_e81307: f64 = (assign48290_e81305 + p.p1378);
        (assign48290_e81307,)
    } else {
        (locals.var_wdios,)
    }
};
        locals.var_wdios = assign48290_e81309;
        locals.var_wdios_rv = 0.0;

        let (assign48300_e81316,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48300_e81314: f64 = (locals.var_wdios * p.p74);
        (assign48300_e81314,)
    } else {
        (locals.var_wstsi,)
    }
};
        locals.var_wstsi = assign48300_e81316;
        locals.var_wstsi_rv = 0.0;

        let (assign48310_e81323,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48310_e81321: f64 = (locals.var_wdiod * p.p74);
        (assign48310_e81321,)
    } else {
        (locals.var_wdtsi,)
    }
};
        locals.var_wdtsi = assign48310_e81323;
        locals.var_wdtsi_rv = 0.0;

        let (assign48320_e81330, assign48320_e81330_d_n4, assign48320_e81330_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48320_e81328: f64 = (locals.var_vtm * locals.var_ndiode_i);
        (assign48320_e81328, (locals.var_vtm_dn4 * locals.var_ndiode_i), (locals.var_vtm_dn5 * locals.var_ndiode_i),)
    } else {
        (locals.var_nvtm1, locals.var_nvtm1_dn4, locals.var_nvtm1_dn5,)
    }
};
        locals.var_nvtm1 = assign48320_e81330;
        locals.var_nvtm1_dn4 = assign48320_e81330_d_n4;
        locals.var_nvtm1_dn5 = assign48320_e81330_d_n5;
        locals.var_nvtm1_rv = 0.0;

        let (assign48330_e81337, assign48330_e81337_d_n3, assign48330_e81337_d_n4, assign48330_e81337_d_n5, assign48330_e81337_d_n6, assign48330_e81337_d_n7, assign48330_e81337_d_n8, assign48330_e81337_d_n9, assign48330_e81337_d_n10, assign48330_e81337_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48330_e81335: f64 = (locals.var_vbs_jct / locals.var_nvtm1);
        (assign48330_e81335, 0.0, (-((locals.var_vbs_jct * locals.var_nvtm1_dn4) / (locals.var_nvtm1 * locals.var_nvtm1))), (-((locals.var_vbs_jct * locals.var_nvtm1_dn5) / (locals.var_nvtm1 * locals.var_nvtm1))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_nvtm1), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_nvtm1), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48330_e81337;
        locals.var_t0_dn3 = assign48330_e81337_d_n3;
        locals.var_t0_dn4 = assign48330_e81337_d_n4;
        locals.var_t0_dn5 = assign48330_e81337_d_n5;
        locals.var_t0_dn6 = assign48330_e81337_d_n6;
        locals.var_t0_dn7 = assign48330_e81337_d_n7;
        locals.var_t0_dn8 = assign48330_e81337_d_n8;
        locals.var_t0_dn9 = assign48330_e81337_d_n9;
        locals.var_t0_dn10 = assign48330_e81337_d_n10;
        locals.var_t0_dn11 = assign48330_e81337_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign48340_e81343, assign48340_e81343_d_n3, assign48340_e81343_d_n4, assign48340_e81343_d_n5, assign48340_e81343_d_n6, assign48340_e81343_d_n7, assign48340_e81343_d_n8, assign48340_e81343_d_n9, assign48340_e81343_d_n10, assign48340_e81343_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48340_e81341: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48340_e81341, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_expvbsnvtm, locals.var_expvbsnvtm_dn3, locals.var_expvbsnvtm_dn4, locals.var_expvbsnvtm_dn5, locals.var_expvbsnvtm_dn6, locals.var_expvbsnvtm_dn7, locals.var_expvbsnvtm_dn8, locals.var_expvbsnvtm_dn9, locals.var_expvbsnvtm_dn10, locals.var_expvbsnvtm_dn11,)
    }
};
        locals.var_expvbsnvtm = assign48340_e81343;
        locals.var_expvbsnvtm_dn3 = assign48340_e81343_d_n3;
        locals.var_expvbsnvtm_dn4 = assign48340_e81343_d_n4;
        locals.var_expvbsnvtm_dn5 = assign48340_e81343_d_n5;
        locals.var_expvbsnvtm_dn6 = assign48340_e81343_d_n6;
        locals.var_expvbsnvtm_dn7 = assign48340_e81343_d_n7;
        locals.var_expvbsnvtm_dn8 = assign48340_e81343_d_n8;
        locals.var_expvbsnvtm_dn9 = assign48340_e81343_d_n9;
        locals.var_expvbsnvtm_dn10 = assign48340_e81343_d_n10;
        locals.var_expvbsnvtm_dn11 = assign48340_e81343_d_n11;
        locals.var_expvbsnvtm_rv = 0.0;

        let (assign48350_e81350, assign48350_e81350_d_n4, assign48350_e81350_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48350_e81348: f64 = (locals.var_vtm * locals.var_ndiode_i);
        (assign48350_e81348, (locals.var_vtm_dn4 * locals.var_ndiode_i), (locals.var_vtm_dn5 * locals.var_ndiode_i),)
    } else {
        (locals.var_nvtm2, locals.var_nvtm2_dn4, locals.var_nvtm2_dn5,)
    }
};
        locals.var_nvtm2 = assign48350_e81350;
        locals.var_nvtm2_dn4 = assign48350_e81350_d_n4;
        locals.var_nvtm2_dn5 = assign48350_e81350_d_n5;
        locals.var_nvtm2_rv = 0.0;

        let (assign48360_e81357, assign48360_e81357_d_n3, assign48360_e81357_d_n4, assign48360_e81357_d_n5, assign48360_e81357_d_n6, assign48360_e81357_d_n7, assign48360_e81357_d_n8, assign48360_e81357_d_n9, assign48360_e81357_d_n10, assign48360_e81357_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48360_e81355: f64 = (locals.var_vbd_jct / locals.var_nvtm2);
        (assign48360_e81355, 0.0, (-((locals.var_vbd_jct * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))), (-((locals.var_vbd_jct * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))), (locals.var_vbd_jct_dn6 / locals.var_nvtm2), 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn10 / locals.var_nvtm2), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48360_e81357;
        locals.var_t0_dn3 = assign48360_e81357_d_n3;
        locals.var_t0_dn4 = assign48360_e81357_d_n4;
        locals.var_t0_dn5 = assign48360_e81357_d_n5;
        locals.var_t0_dn6 = assign48360_e81357_d_n6;
        locals.var_t0_dn7 = assign48360_e81357_d_n7;
        locals.var_t0_dn8 = assign48360_e81357_d_n8;
        locals.var_t0_dn9 = assign48360_e81357_d_n9;
        locals.var_t0_dn10 = assign48360_e81357_d_n10;
        locals.var_t0_dn11 = assign48360_e81357_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign48370_e81363, assign48370_e81363_d_n3, assign48370_e81363_d_n4, assign48370_e81363_d_n5, assign48370_e81363_d_n6, assign48370_e81363_d_n7, assign48370_e81363_d_n8, assign48370_e81363_d_n9, assign48370_e81363_d_n10, assign48370_e81363_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48370_e81361: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48370_e81361, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_expvbdnvtm, locals.var_expvbdnvtm_dn3, locals.var_expvbdnvtm_dn4, locals.var_expvbdnvtm_dn5, locals.var_expvbdnvtm_dn6, locals.var_expvbdnvtm_dn7, locals.var_expvbdnvtm_dn8, locals.var_expvbdnvtm_dn9, locals.var_expvbdnvtm_dn10, locals.var_expvbdnvtm_dn11,)
    }
};
        locals.var_expvbdnvtm = assign48370_e81363;
        locals.var_expvbdnvtm_dn3 = assign48370_e81363_d_n3;
        locals.var_expvbdnvtm_dn4 = assign48370_e81363_d_n4;
        locals.var_expvbdnvtm_dn5 = assign48370_e81363_d_n5;
        locals.var_expvbdnvtm_dn6 = assign48370_e81363_d_n6;
        locals.var_expvbdnvtm_dn7 = assign48370_e81363_d_n7;
        locals.var_expvbdnvtm_dn8 = assign48370_e81363_d_n8;
        locals.var_expvbdnvtm_dn9 = assign48370_e81363_d_n9;
        locals.var_expvbdnvtm_dn10 = assign48370_e81363_d_n10;
        locals.var_expvbdnvtm_dn11 = assign48370_e81363_d_n11;
        locals.var_expvbdnvtm_rv = 0.0;

        let (assign48380_e81374, assign48380_e81374_d_n3, assign48380_e81374_d_n4, assign48380_e81374_d_n5, assign48380_e81374_d_n6, assign48380_e81374_d_n7, assign48380_e81374_d_n8, assign48380_e81374_d_n9, assign48380_e81374_d_n10, assign48380_e81374_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48380_e81368: f64 = (1.115 / locals.var_vtm);
        let assign48380_e81371: f64 = (locals.var_tratio - 1.0);
        let assign48380_e81372: f64 = (assign48380_e81368 * assign48380_e81371);
        (assign48380_e81372, 0.0, (((-((1.115 * locals.var_vtm_dn4) / (locals.var_vtm * locals.var_vtm))) * assign48380_e81371) + (assign48380_e81368 * locals.var_tratio_dn4)), (((-((1.115 * locals.var_vtm_dn5) / (locals.var_vtm * locals.var_vtm))) * assign48380_e81371) + (assign48380_e81368 * locals.var_tratio_dn5)), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign48380_e81374;
        locals.var_t4_dn3 = assign48380_e81374_d_n3;
        locals.var_t4_dn4 = assign48380_e81374_d_n4;
        locals.var_t4_dn5 = assign48380_e81374_d_n5;
        locals.var_t4_dn6 = assign48380_e81374_d_n6;
        locals.var_t4_dn7 = assign48380_e81374_d_n7;
        locals.var_t4_dn8 = assign48380_e81374_d_n8;
        locals.var_t4_dn9 = assign48380_e81374_d_n9;
        locals.var_t4_dn10 = assign48380_e81374_d_n10;
        locals.var_t4_dn11 = assign48380_e81374_d_n11;
        locals.var_t4_rv = 0.0;

        let assign48390_e81377: f64 = if locals.var_isdif_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard744 = assign48390_e81377;
        locals.var_guard744_rv = 0.0;

        let (assign48410_e81396, assign48410_e81396_d_n3, assign48410_e81396_d_n4, assign48410_e81396_d_n5, assign48410_e81396_d_n6, assign48410_e81396_d_n7, assign48410_e81396_d_n8, assign48410_e81396_d_n9, assign48410_e81396_d_n10, assign48410_e81396_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard744 == 0.0)) {
        let assign48410_e81392: f64 = (locals.var_xdif_i * locals.var_t4);
        let assign48410_e81394: f64 = (assign48410_e81392 / locals.var_ndiode_i);
        (assign48410_e81394, ((locals.var_xdif_i * locals.var_t4_dn3) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn4) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn5) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn6) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn7) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn8) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn9) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn10) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn11) / locals.var_ndiode_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign48410_e81396;
        locals.var_t7_dn3 = assign48410_e81396_d_n3;
        locals.var_t7_dn4 = assign48410_e81396_d_n4;
        locals.var_t7_dn5 = assign48410_e81396_d_n5;
        locals.var_t7_dn6 = assign48410_e81396_d_n6;
        locals.var_t7_dn7 = assign48410_e81396_d_n7;
        locals.var_t7_dn8 = assign48410_e81396_d_n8;
        locals.var_t7_dn9 = assign48410_e81396_d_n9;
        locals.var_t7_dn10 = assign48410_e81396_d_n10;
        locals.var_t7_dn11 = assign48410_e81396_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign48420_e81405, assign48420_e81405_d_n3, assign48420_e81405_d_n4, assign48420_e81405_d_n5, assign48420_e81405_d_n6, assign48420_e81405_d_n7, assign48420_e81405_d_n8, assign48420_e81405_d_n9, assign48420_e81405_d_n10, assign48420_e81405_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard744 == 0.0)) {
        let assign48420_e81403: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48420_e81403, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign48420_e81405;
        locals.var_t1_dn3 = assign48420_e81405_d_n3;
        locals.var_t1_dn4 = assign48420_e81405_d_n4;
        locals.var_t1_dn5 = assign48420_e81405_d_n5;
        locals.var_t1_dn6 = assign48420_e81405_d_n6;
        locals.var_t1_dn7 = assign48420_e81405_d_n7;
        locals.var_t1_dn8 = assign48420_e81405_d_n8;
        locals.var_t1_dn9 = assign48420_e81405_d_n9;
        locals.var_t1_dn10 = assign48420_e81405_d_n10;
        locals.var_t1_dn11 = assign48420_e81405_d_n11;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_169(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign48430_e81415, assign48430_e81415_d_n3, assign48430_e81415_d_n4, assign48430_e81415_d_n5, assign48430_e81415_d_n6, assign48430_e81415_d_n7, assign48430_e81415_d_n8, assign48430_e81415_d_n9, assign48430_e81415_d_n10, assign48430_e81415_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard744 == 0.0)) {
        let assign48430_e81413: f64 = (locals.var_isdif_i * locals.var_t1);
        (assign48430_e81413, (locals.var_isdif_i * locals.var_t1_dn3), (locals.var_isdif_i * locals.var_t1_dn4), (locals.var_isdif_i * locals.var_t1_dn5), (locals.var_isdif_i * locals.var_t1_dn6), (locals.var_isdif_i * locals.var_t1_dn7), (locals.var_isdif_i * locals.var_t1_dn8), (locals.var_isdif_i * locals.var_t1_dn9), (locals.var_isdif_i * locals.var_t1_dn10), (locals.var_isdif_i * locals.var_t1_dn11),)
    } else {
        (locals.var_jdifs, locals.var_jdifs_dn3, locals.var_jdifs_dn4, locals.var_jdifs_dn5, locals.var_jdifs_dn6, locals.var_jdifs_dn7, locals.var_jdifs_dn8, locals.var_jdifs_dn9, locals.var_jdifs_dn10, locals.var_jdifs_dn11,)
    }
};
        locals.var_jdifs = assign48430_e81415;
        locals.var_jdifs_dn3 = assign48430_e81415_d_n3;
        locals.var_jdifs_dn4 = assign48430_e81415_d_n4;
        locals.var_jdifs_dn5 = assign48430_e81415_d_n5;
        locals.var_jdifs_dn6 = assign48430_e81415_d_n6;
        locals.var_jdifs_dn7 = assign48430_e81415_d_n7;
        locals.var_jdifs_dn8 = assign48430_e81415_d_n8;
        locals.var_jdifs_dn9 = assign48430_e81415_d_n9;
        locals.var_jdifs_dn10 = assign48430_e81415_d_n10;
        locals.var_jdifs_dn11 = assign48430_e81415_d_n11;
        locals.var_jdifs_rv = 0.0;

        let (assign48440_e81425, assign48440_e81425_d_n3, assign48440_e81425_d_n4, assign48440_e81425_d_n5, assign48440_e81425_d_n6, assign48440_e81425_d_n7, assign48440_e81425_d_n8, assign48440_e81425_d_n9, assign48440_e81425_d_n10, assign48440_e81425_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard744 == 0.0)) {
        let assign48440_e81423: f64 = (locals.var_wstsi * locals.var_jdifs);
        (assign48440_e81423, (locals.var_wstsi * locals.var_jdifs_dn3), (locals.var_wstsi * locals.var_jdifs_dn4), (locals.var_wstsi * locals.var_jdifs_dn5), (locals.var_wstsi * locals.var_jdifs_dn6), (locals.var_wstsi * locals.var_jdifs_dn7), (locals.var_wstsi * locals.var_jdifs_dn8), (locals.var_wstsi * locals.var_jdifs_dn9), (locals.var_wstsi * locals.var_jdifs_dn10), (locals.var_wstsi * locals.var_jdifs_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48440_e81425;
        locals.var_t0_dn3 = assign48440_e81425_d_n3;
        locals.var_t0_dn4 = assign48440_e81425_d_n4;
        locals.var_t0_dn5 = assign48440_e81425_d_n5;
        locals.var_t0_dn6 = assign48440_e81425_d_n6;
        locals.var_t0_dn7 = assign48440_e81425_d_n7;
        locals.var_t0_dn8 = assign48440_e81425_d_n8;
        locals.var_t0_dn9 = assign48440_e81425_d_n9;
        locals.var_t0_dn10 = assign48440_e81425_d_n10;
        locals.var_t0_dn11 = assign48440_e81425_d_n11;
        locals.var_t0_rv = 0.0;

        let assign48460_e81440: f64 = if locals.var_iddif_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard745 = assign48460_e81440;
        locals.var_guard745_rv = 0.0;

        let (assign48480_e81459, assign48480_e81459_d_n3, assign48480_e81459_d_n4, assign48480_e81459_d_n5, assign48480_e81459_d_n6, assign48480_e81459_d_n7, assign48480_e81459_d_n8, assign48480_e81459_d_n9, assign48480_e81459_d_n10, assign48480_e81459_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard745 == 0.0)) {
        let assign48480_e81455: f64 = (locals.var_xdif_i * locals.var_t4);
        let assign48480_e81457: f64 = (assign48480_e81455 / locals.var_ndiode_i);
        (assign48480_e81457, ((locals.var_xdif_i * locals.var_t4_dn3) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn4) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn5) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn6) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn7) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn8) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn9) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn10) / locals.var_ndiode_i), ((locals.var_xdif_i * locals.var_t4_dn11) / locals.var_ndiode_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign48480_e81459;
        locals.var_t7_dn3 = assign48480_e81459_d_n3;
        locals.var_t7_dn4 = assign48480_e81459_d_n4;
        locals.var_t7_dn5 = assign48480_e81459_d_n5;
        locals.var_t7_dn6 = assign48480_e81459_d_n6;
        locals.var_t7_dn7 = assign48480_e81459_d_n7;
        locals.var_t7_dn8 = assign48480_e81459_d_n8;
        locals.var_t7_dn9 = assign48480_e81459_d_n9;
        locals.var_t7_dn10 = assign48480_e81459_d_n10;
        locals.var_t7_dn11 = assign48480_e81459_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign48490_e81468, assign48490_e81468_d_n3, assign48490_e81468_d_n4, assign48490_e81468_d_n5, assign48490_e81468_d_n6, assign48490_e81468_d_n7, assign48490_e81468_d_n8, assign48490_e81468_d_n9, assign48490_e81468_d_n10, assign48490_e81468_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard745 == 0.0)) {
        let assign48490_e81466: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48490_e81466, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign48490_e81468;
        locals.var_t1_dn3 = assign48490_e81468_d_n3;
        locals.var_t1_dn4 = assign48490_e81468_d_n4;
        locals.var_t1_dn5 = assign48490_e81468_d_n5;
        locals.var_t1_dn6 = assign48490_e81468_d_n6;
        locals.var_t1_dn7 = assign48490_e81468_d_n7;
        locals.var_t1_dn8 = assign48490_e81468_d_n8;
        locals.var_t1_dn9 = assign48490_e81468_d_n9;
        locals.var_t1_dn10 = assign48490_e81468_d_n10;
        locals.var_t1_dn11 = assign48490_e81468_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign48500_e81478, assign48500_e81478_d_n3, assign48500_e81478_d_n4, assign48500_e81478_d_n5, assign48500_e81478_d_n6, assign48500_e81478_d_n7, assign48500_e81478_d_n8, assign48500_e81478_d_n9, assign48500_e81478_d_n10, assign48500_e81478_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard745 == 0.0)) {
        let assign48500_e81476: f64 = (locals.var_iddif_i * locals.var_t1);
        (assign48500_e81476, (locals.var_iddif_i * locals.var_t1_dn3), (locals.var_iddif_i * locals.var_t1_dn4), (locals.var_iddif_i * locals.var_t1_dn5), (locals.var_iddif_i * locals.var_t1_dn6), (locals.var_iddif_i * locals.var_t1_dn7), (locals.var_iddif_i * locals.var_t1_dn8), (locals.var_iddif_i * locals.var_t1_dn9), (locals.var_iddif_i * locals.var_t1_dn10), (locals.var_iddif_i * locals.var_t1_dn11),)
    } else {
        (locals.var_jdifd, locals.var_jdifd_dn3, locals.var_jdifd_dn4, locals.var_jdifd_dn5, locals.var_jdifd_dn6, locals.var_jdifd_dn7, locals.var_jdifd_dn8, locals.var_jdifd_dn9, locals.var_jdifd_dn10, locals.var_jdifd_dn11,)
    }
};
        locals.var_jdifd = assign48500_e81478;
        locals.var_jdifd_dn3 = assign48500_e81478_d_n3;
        locals.var_jdifd_dn4 = assign48500_e81478_d_n4;
        locals.var_jdifd_dn5 = assign48500_e81478_d_n5;
        locals.var_jdifd_dn6 = assign48500_e81478_d_n6;
        locals.var_jdifd_dn7 = assign48500_e81478_d_n7;
        locals.var_jdifd_dn8 = assign48500_e81478_d_n8;
        locals.var_jdifd_dn9 = assign48500_e81478_d_n9;
        locals.var_jdifd_dn10 = assign48500_e81478_d_n10;
        locals.var_jdifd_dn11 = assign48500_e81478_d_n11;
        locals.var_jdifd_rv = 0.0;

        let (assign48510_e81488, assign48510_e81488_d_n3, assign48510_e81488_d_n4, assign48510_e81488_d_n5, assign48510_e81488_d_n6, assign48510_e81488_d_n7, assign48510_e81488_d_n8, assign48510_e81488_d_n9, assign48510_e81488_d_n10, assign48510_e81488_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard745 == 0.0)) {
        let assign48510_e81486: f64 = (locals.var_wdtsi * locals.var_jdifd);
        (assign48510_e81486, (locals.var_wdtsi * locals.var_jdifd_dn3), (locals.var_wdtsi * locals.var_jdifd_dn4), (locals.var_wdtsi * locals.var_jdifd_dn5), (locals.var_wdtsi * locals.var_jdifd_dn6), (locals.var_wdtsi * locals.var_jdifd_dn7), (locals.var_wdtsi * locals.var_jdifd_dn8), (locals.var_wdtsi * locals.var_jdifd_dn9), (locals.var_wdtsi * locals.var_jdifd_dn10), (locals.var_wdtsi * locals.var_jdifd_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48510_e81488;
        locals.var_t0_dn3 = assign48510_e81488_d_n3;
        locals.var_t0_dn4 = assign48510_e81488_d_n4;
        locals.var_t0_dn5 = assign48510_e81488_d_n5;
        locals.var_t0_dn6 = assign48510_e81488_d_n6;
        locals.var_t0_dn7 = assign48510_e81488_d_n7;
        locals.var_t0_dn8 = assign48510_e81488_d_n8;
        locals.var_t0_dn9 = assign48510_e81488_d_n9;
        locals.var_t0_dn10 = assign48510_e81488_d_n10;
        locals.var_t0_dn11 = assign48510_e81488_d_n11;
        locals.var_t0_rv = 0.0;

        let assign48530_e81503: f64 = if locals.var_isrec_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard746 = assign48530_e81503;
        locals.var_guard746_rv = 0.0;

        let (assign48550_e81522, assign48550_e81522_d_n3, assign48550_e81522_d_n4, assign48550_e81522_d_n5, assign48550_e81522_d_n6, assign48550_e81522_d_n7, assign48550_e81522_d_n8, assign48550_e81522_d_n9, assign48550_e81522_d_n10, assign48550_e81522_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48550_e81518: f64 = (locals.var_xrec_i * locals.var_t4);
        let assign48550_e81520: f64 = (assign48550_e81518 / locals.var_nrecf0_i);
        (assign48550_e81520, ((locals.var_xrec_i * locals.var_t4_dn3) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn4) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn5) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn6) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn7) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn8) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn9) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn10) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn11) / locals.var_nrecf0_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign48550_e81522;
        locals.var_t7_dn3 = assign48550_e81522_d_n3;
        locals.var_t7_dn4 = assign48550_e81522_d_n4;
        locals.var_t7_dn5 = assign48550_e81522_d_n5;
        locals.var_t7_dn6 = assign48550_e81522_d_n6;
        locals.var_t7_dn7 = assign48550_e81522_d_n7;
        locals.var_t7_dn8 = assign48550_e81522_d_n8;
        locals.var_t7_dn9 = assign48550_e81522_d_n9;
        locals.var_t7_dn10 = assign48550_e81522_d_n10;
        locals.var_t7_dn11 = assign48550_e81522_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign48560_e81531, assign48560_e81531_d_n3, assign48560_e81531_d_n4, assign48560_e81531_d_n5, assign48560_e81531_d_n6, assign48560_e81531_d_n7, assign48560_e81531_d_n8, assign48560_e81531_d_n9, assign48560_e81531_d_n10, assign48560_e81531_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48560_e81529: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48560_e81529, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign48560_e81531;
        locals.var_t2_dn3 = assign48560_e81531_d_n3;
        locals.var_t2_dn4 = assign48560_e81531_d_n4;
        locals.var_t2_dn5 = assign48560_e81531_d_n5;
        locals.var_t2_dn6 = assign48560_e81531_d_n6;
        locals.var_t2_dn7 = assign48560_e81531_d_n7;
        locals.var_t2_dn8 = assign48560_e81531_d_n8;
        locals.var_t2_dn9 = assign48560_e81531_d_n9;
        locals.var_t2_dn10 = assign48560_e81531_d_n10;
        locals.var_t2_dn11 = assign48560_e81531_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign48570_e81541, assign48570_e81541_d_n3, assign48570_e81541_d_n4, assign48570_e81541_d_n5, assign48570_e81541_d_n6, assign48570_e81541_d_n7, assign48570_e81541_d_n8, assign48570_e81541_d_n9, assign48570_e81541_d_n10, assign48570_e81541_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48570_e81539: f64 = (locals.var_isrec_i * locals.var_t2);
        (assign48570_e81539, (locals.var_isrec_i * locals.var_t2_dn3), (locals.var_isrec_i * locals.var_t2_dn4), (locals.var_isrec_i * locals.var_t2_dn5), (locals.var_isrec_i * locals.var_t2_dn6), (locals.var_isrec_i * locals.var_t2_dn7), (locals.var_isrec_i * locals.var_t2_dn8), (locals.var_isrec_i * locals.var_t2_dn9), (locals.var_isrec_i * locals.var_t2_dn10), (locals.var_isrec_i * locals.var_t2_dn11),)
    } else {
        (locals.var_jrecs, locals.var_jrecs_dn3, locals.var_jrecs_dn4, locals.var_jrecs_dn5, locals.var_jrecs_dn6, locals.var_jrecs_dn7, locals.var_jrecs_dn8, locals.var_jrecs_dn9, locals.var_jrecs_dn10, locals.var_jrecs_dn11,)
    }
};
        locals.var_jrecs = assign48570_e81541;
        locals.var_jrecs_dn3 = assign48570_e81541_d_n3;
        locals.var_jrecs_dn4 = assign48570_e81541_d_n4;
        locals.var_jrecs_dn5 = assign48570_e81541_d_n5;
        locals.var_jrecs_dn6 = assign48570_e81541_d_n6;
        locals.var_jrecs_dn7 = assign48570_e81541_d_n7;
        locals.var_jrecs_dn8 = assign48570_e81541_d_n8;
        locals.var_jrecs_dn9 = assign48570_e81541_d_n9;
        locals.var_jrecs_dn10 = assign48570_e81541_d_n10;
        locals.var_jrecs_dn11 = assign48570_e81541_d_n11;
        locals.var_jrecs_rv = 0.0;

        let (assign48580_e81559, assign48580_e81559_d_n4, assign48580_e81559_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48580_e81549: f64 = (p.p925 * locals.var_nrecf0_i);
        let assign48580_e81554: f64 = (locals.var_tratio - 1.0);
        let assign48580_e81555: f64 = (locals.var_ntrecf_i * assign48580_e81554);
        let assign48580_e81556: f64 = (1.0 + assign48580_e81555);
        let assign48580_e81557: f64 = (assign48580_e81549 * assign48580_e81556);
        (assign48580_e81557, (assign48580_e81549 * (locals.var_ntrecf_i * locals.var_tratio_dn4)), (assign48580_e81549 * (locals.var_ntrecf_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_nvtmf, locals.var_nvtmf_dn4, locals.var_nvtmf_dn5,)
    }
};
        locals.var_nvtmf = assign48580_e81559;
        locals.var_nvtmf_dn4 = assign48580_e81559_d_n4;
        locals.var_nvtmf_dn5 = assign48580_e81559_d_n5;
        locals.var_nvtmf_rv = 0.0;

        let (assign48590_e81577, assign48590_e81577_d_n4, assign48590_e81577_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48590_e81567: f64 = (p.p925 * locals.var_nrecr0_i);
        let assign48590_e81572: f64 = (locals.var_tratio - 1.0);
        let assign48590_e81573: f64 = (locals.var_ntrecr_i * assign48590_e81572);
        let assign48590_e81574: f64 = (1.0 + assign48590_e81573);
        let assign48590_e81575: f64 = (assign48590_e81567 * assign48590_e81574);
        (assign48590_e81575, (assign48590_e81567 * (locals.var_ntrecr_i * locals.var_tratio_dn4)), (assign48590_e81567 * (locals.var_ntrecr_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_nvtmr, locals.var_nvtmr_dn4, locals.var_nvtmr_dn5,)
    }
};
        locals.var_nvtmr = assign48590_e81577;
        locals.var_nvtmr_dn4 = assign48590_e81577_d_n4;
        locals.var_nvtmr_dn5 = assign48590_e81577_d_n5;
        locals.var_nvtmr_rv = 0.0;

        let (assign48600_e81587, assign48600_e81587_d_n3, assign48600_e81587_d_n4, assign48600_e81587_d_n5, assign48600_e81587_d_n6, assign48600_e81587_d_n7, assign48600_e81587_d_n8, assign48600_e81587_d_n9, assign48600_e81587_d_n10, assign48600_e81587_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48600_e81585: f64 = (locals.var_vbs_jct / locals.var_nvtmf);
        (assign48600_e81585, 0.0, (-((locals.var_vbs_jct * locals.var_nvtmf_dn4) / (locals.var_nvtmf * locals.var_nvtmf))), (-((locals.var_vbs_jct * locals.var_nvtmf_dn5) / (locals.var_nvtmf * locals.var_nvtmf))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_nvtmf), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_nvtmf), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48600_e81587;
        locals.var_t0_dn3 = assign48600_e81587_d_n3;
        locals.var_t0_dn4 = assign48600_e81587_d_n4;
        locals.var_t0_dn5 = assign48600_e81587_d_n5;
        locals.var_t0_dn6 = assign48600_e81587_d_n6;
        locals.var_t0_dn7 = assign48600_e81587_d_n7;
        locals.var_t0_dn8 = assign48600_e81587_d_n8;
        locals.var_t0_dn9 = assign48600_e81587_d_n9;
        locals.var_t0_dn10 = assign48600_e81587_d_n10;
        locals.var_t0_dn11 = assign48600_e81587_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign48610_e81596, assign48610_e81596_d_n3, assign48610_e81596_d_n4, assign48610_e81596_d_n5, assign48610_e81596_d_n6, assign48610_e81596_d_n7, assign48610_e81596_d_n8, assign48610_e81596_d_n9, assign48610_e81596_d_n10, assign48610_e81596_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48610_e81594: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48610_e81594, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11,)
    }
};
        locals.var_t10 = assign48610_e81596;
        locals.var_t10_dn3 = assign48610_e81596_d_n3;
        locals.var_t10_dn4 = assign48610_e81596_d_n4;
        locals.var_t10_dn5 = assign48610_e81596_d_n5;
        locals.var_t10_dn6 = assign48610_e81596_d_n6;
        locals.var_t10_dn7 = assign48610_e81596_d_n7;
        locals.var_t10_dn8 = assign48610_e81596_d_n8;
        locals.var_t10_dn9 = assign48610_e81596_d_n9;
        locals.var_t10_dn10 = assign48610_e81596_d_n10;
        locals.var_t10_dn11 = assign48610_e81596_d_n11;
        locals.var_t10_rv = 0.0;

        let assign48620_e81599: f64 = (locals.var_vrec0_i - locals.var_vbs_jct);
        let assign48620_e81601: f64 = if assign48620_e81599 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard747 = assign48620_e81601;
        locals.var_guard747_rv = 0.0;

        let (assign48630_e81611, assign48630_e81611_d_n3, assign48630_e81611_d_n4, assign48630_e81611_d_n5, assign48630_e81611_d_n6, assign48630_e81611_d_n7, assign48630_e81611_d_n8, assign48630_e81611_d_n9, assign48630_e81611_d_n10, assign48630_e81611_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign48630_e81611;
        locals.var_t1_dn3 = assign48630_e81611_d_n3;
        locals.var_t1_dn4 = assign48630_e81611_d_n4;
        locals.var_t1_dn5 = assign48630_e81611_d_n5;
        locals.var_t1_dn6 = assign48630_e81611_d_n6;
        locals.var_t1_dn7 = assign48630_e81611_d_n7;
        locals.var_t1_dn8 = assign48630_e81611_d_n8;
        locals.var_t1_dn9 = assign48630_e81611_d_n9;
        locals.var_t1_dn10 = assign48630_e81611_d_n10;
        locals.var_t1_dn11 = assign48630_e81611_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign48640_e81628, assign48640_e81628_d_n3, assign48640_e81628_d_n4, assign48640_e81628_d_n5, assign48640_e81628_d_n6, assign48640_e81628_d_n7, assign48640_e81628_d_n8, assign48640_e81628_d_n9, assign48640_e81628_d_n10, assign48640_e81628_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 != 0.0)) {
        let assign48640_e81620: f64 = (-locals.var_vbs_jct);
        let assign48640_e81622: f64 = (assign48640_e81620 / locals.var_nvtmr);
        let assign48640_e81624: f64 = (assign48640_e81622 * locals.var_vrec0_i);
        let assign48640_e81626: f64 = (assign48640_e81624 * locals.var_t1);
        (assign48640_e81626, (assign48640_e81624 * locals.var_t1_dn3), ((((-((assign48640_e81620 * locals.var_nvtmr_dn4) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0_i) * locals.var_t1) + (assign48640_e81624 * locals.var_t1_dn4)), ((((-((assign48640_e81620 * locals.var_nvtmr_dn5) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0_i) * locals.var_t1) + (assign48640_e81624 * locals.var_t1_dn5)), (assign48640_e81624 * locals.var_t1_dn6), (((((-locals.var_vbs_jct_dn7) / locals.var_nvtmr) * locals.var_vrec0_i) * locals.var_t1) + (assign48640_e81624 * locals.var_t1_dn7)), (assign48640_e81624 * locals.var_t1_dn8), (assign48640_e81624 * locals.var_t1_dn9), (((((-locals.var_vbs_jct_dn10) / locals.var_nvtmr) * locals.var_vrec0_i) * locals.var_t1) + (assign48640_e81624 * locals.var_t1_dn10)), (assign48640_e81624 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48640_e81628;
        locals.var_t0_dn3 = assign48640_e81628_d_n3;
        locals.var_t0_dn4 = assign48640_e81628_d_n4;
        locals.var_t0_dn5 = assign48640_e81628_d_n5;
        locals.var_t0_dn6 = assign48640_e81628_d_n6;
        locals.var_t0_dn7 = assign48640_e81628_d_n7;
        locals.var_t0_dn8 = assign48640_e81628_d_n8;
        locals.var_t0_dn9 = assign48640_e81628_d_n9;
        locals.var_t0_dn10 = assign48640_e81628_d_n10;
        locals.var_t0_dn11 = assign48640_e81628_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign48650_e81639, assign48650_e81639_d_n3, assign48650_e81639_d_n4, assign48650_e81639_d_n5, assign48650_e81639_d_n6, assign48650_e81639_d_n7, assign48650_e81639_d_n8, assign48650_e81639_d_n9, assign48650_e81639_d_n10, assign48650_e81639_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 != 0.0)) {
        let assign48650_e81637: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48650_e81637, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48650_e81639;
        locals.var_t11_dn3 = assign48650_e81639_d_n3;
        locals.var_t11_dn4 = assign48650_e81639_d_n4;
        locals.var_t11_dn5 = assign48650_e81639_d_n5;
        locals.var_t11_dn6 = assign48650_e81639_d_n6;
        locals.var_t11_dn7 = assign48650_e81639_d_n7;
        locals.var_t11_dn8 = assign48650_e81639_d_n8;
        locals.var_t11_dn9 = assign48650_e81639_d_n9;
        locals.var_t11_dn10 = assign48650_e81639_d_n10;
        locals.var_t11_dn11 = assign48650_e81639_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign48660_e81650, assign48660_e81650_d_n3, assign48660_e81650_d_n4, assign48660_e81650_d_n5, assign48660_e81650_d_n6, assign48660_e81650_d_n7, assign48660_e81650_d_n8, assign48660_e81650_d_n9, assign48660_e81650_d_n10, assign48660_e81650_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 != 0.0)) {
        let assign48660_e81648: f64 = (-locals.var_t11);
        (assign48660_e81648, (-locals.var_t11_dn3), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48660_e81650;
        locals.var_t11_dn3 = assign48660_e81650_d_n3;
        locals.var_t11_dn4 = assign48660_e81650_d_n4;
        locals.var_t11_dn5 = assign48660_e81650_d_n5;
        locals.var_t11_dn6 = assign48660_e81650_d_n6;
        locals.var_t11_dn7 = assign48660_e81650_d_n7;
        locals.var_t11_dn8 = assign48660_e81650_d_n8;
        locals.var_t11_dn9 = assign48660_e81650_d_n9;
        locals.var_t11_dn10 = assign48660_e81650_d_n10;
        locals.var_t11_dn11 = assign48660_e81650_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign48670_e81665, assign48670_e81665_d_n3, assign48670_e81665_d_n4, assign48670_e81665_d_n5, assign48670_e81665_d_n6, assign48670_e81665_d_n7, assign48670_e81665_d_n8, assign48670_e81665_d_n9, assign48670_e81665_d_n10, assign48670_e81665_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 == 0.0)) {
        let assign48670_e81662: f64 = (locals.var_vrec0_i - locals.var_vbs_jct);
        let assign48670_e81663: f64 = (1.0 / assign48670_e81662);
        (assign48670_e81663, 0.0, 0.0, 0.0, 0.0, (-((-locals.var_vbs_jct_dn7) / (assign48670_e81662 * assign48670_e81662))), 0.0, 0.0, (-((-locals.var_vbs_jct_dn10) / (assign48670_e81662 * assign48670_e81662))), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign48670_e81665;
        locals.var_t1_dn3 = assign48670_e81665_d_n3;
        locals.var_t1_dn4 = assign48670_e81665_d_n4;
        locals.var_t1_dn5 = assign48670_e81665_d_n5;
        locals.var_t1_dn6 = assign48670_e81665_d_n6;
        locals.var_t1_dn7 = assign48670_e81665_d_n7;
        locals.var_t1_dn8 = assign48670_e81665_d_n8;
        locals.var_t1_dn9 = assign48670_e81665_d_n9;
        locals.var_t1_dn10 = assign48670_e81665_d_n10;
        locals.var_t1_dn11 = assign48670_e81665_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign48680_e81683, assign48680_e81683_d_n3, assign48680_e81683_d_n4, assign48680_e81683_d_n5, assign48680_e81683_d_n6, assign48680_e81683_d_n7, assign48680_e81683_d_n8, assign48680_e81683_d_n9, assign48680_e81683_d_n10, assign48680_e81683_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 == 0.0)) {
        let assign48680_e81675: f64 = (-locals.var_vbs_jct);
        let assign48680_e81677: f64 = (assign48680_e81675 / locals.var_nvtmr);
        let assign48680_e81679: f64 = (assign48680_e81677 * locals.var_vrec0_i);
        let assign48680_e81681: f64 = (assign48680_e81679 * locals.var_t1);
        (assign48680_e81681, (assign48680_e81679 * locals.var_t1_dn3), ((((-((assign48680_e81675 * locals.var_nvtmr_dn4) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0_i) * locals.var_t1) + (assign48680_e81679 * locals.var_t1_dn4)), ((((-((assign48680_e81675 * locals.var_nvtmr_dn5) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0_i) * locals.var_t1) + (assign48680_e81679 * locals.var_t1_dn5)), (assign48680_e81679 * locals.var_t1_dn6), (((((-locals.var_vbs_jct_dn7) / locals.var_nvtmr) * locals.var_vrec0_i) * locals.var_t1) + (assign48680_e81679 * locals.var_t1_dn7)), (assign48680_e81679 * locals.var_t1_dn8), (assign48680_e81679 * locals.var_t1_dn9), (((((-locals.var_vbs_jct_dn10) / locals.var_nvtmr) * locals.var_vrec0_i) * locals.var_t1) + (assign48680_e81679 * locals.var_t1_dn10)), (assign48680_e81679 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48680_e81683;
        locals.var_t0_dn3 = assign48680_e81683_d_n3;
        locals.var_t0_dn4 = assign48680_e81683_d_n4;
        locals.var_t0_dn5 = assign48680_e81683_d_n5;
        locals.var_t0_dn6 = assign48680_e81683_d_n6;
        locals.var_t0_dn7 = assign48680_e81683_d_n7;
        locals.var_t0_dn8 = assign48680_e81683_d_n8;
        locals.var_t0_dn9 = assign48680_e81683_d_n9;
        locals.var_t0_dn10 = assign48680_e81683_d_n10;
        locals.var_t0_dn11 = assign48680_e81683_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign48690_e81695, assign48690_e81695_d_n3, assign48690_e81695_d_n4, assign48690_e81695_d_n5, assign48690_e81695_d_n6, assign48690_e81695_d_n7, assign48690_e81695_d_n8, assign48690_e81695_d_n9, assign48690_e81695_d_n10, assign48690_e81695_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 == 0.0)) {
        let assign48690_e81693: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48690_e81693, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48690_e81695;
        locals.var_t11_dn3 = assign48690_e81695_d_n3;
        locals.var_t11_dn4 = assign48690_e81695_d_n4;
        locals.var_t11_dn5 = assign48690_e81695_d_n5;
        locals.var_t11_dn6 = assign48690_e81695_d_n6;
        locals.var_t11_dn7 = assign48690_e81695_d_n7;
        locals.var_t11_dn8 = assign48690_e81695_d_n8;
        locals.var_t11_dn9 = assign48690_e81695_d_n9;
        locals.var_t11_dn10 = assign48690_e81695_d_n10;
        locals.var_t11_dn11 = assign48690_e81695_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign48700_e81707, assign48700_e81707_d_n3, assign48700_e81707_d_n4, assign48700_e81707_d_n5, assign48700_e81707_d_n6, assign48700_e81707_d_n7, assign48700_e81707_d_n8, assign48700_e81707_d_n9, assign48700_e81707_d_n10, assign48700_e81707_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) && (locals.var_guard747 == 0.0)) {
        let assign48700_e81705: f64 = (-locals.var_t11);
        (assign48700_e81705, (-locals.var_t11_dn3), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48700_e81707;
        locals.var_t11_dn3 = assign48700_e81707_d_n3;
        locals.var_t11_dn4 = assign48700_e81707_d_n4;
        locals.var_t11_dn5 = assign48700_e81707_d_n5;
        locals.var_t11_dn6 = assign48700_e81707_d_n6;
        locals.var_t11_dn7 = assign48700_e81707_d_n7;
        locals.var_t11_dn8 = assign48700_e81707_d_n8;
        locals.var_t11_dn9 = assign48700_e81707_d_n9;
        locals.var_t11_dn10 = assign48700_e81707_d_n10;
        locals.var_t11_dn11 = assign48700_e81707_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign48710_e81717, assign48710_e81717_d_n3, assign48710_e81717_d_n4, assign48710_e81717_d_n5, assign48710_e81717_d_n6, assign48710_e81717_d_n7, assign48710_e81717_d_n8, assign48710_e81717_d_n9, assign48710_e81717_d_n10, assign48710_e81717_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard746 == 0.0)) {
        let assign48710_e81715: f64 = (locals.var_wstsi * locals.var_jrecs);
        (assign48710_e81715, (locals.var_wstsi * locals.var_jrecs_dn3), (locals.var_wstsi * locals.var_jrecs_dn4), (locals.var_wstsi * locals.var_jrecs_dn5), (locals.var_wstsi * locals.var_jrecs_dn6), (locals.var_wstsi * locals.var_jrecs_dn7), (locals.var_wstsi * locals.var_jrecs_dn8), (locals.var_wstsi * locals.var_jrecs_dn9), (locals.var_wstsi * locals.var_jrecs_dn10), (locals.var_wstsi * locals.var_jrecs_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign48710_e81717;
        locals.var_t3_dn3 = assign48710_e81717_d_n3;
        locals.var_t3_dn4 = assign48710_e81717_d_n4;
        locals.var_t3_dn5 = assign48710_e81717_d_n5;
        locals.var_t3_dn6 = assign48710_e81717_d_n6;
        locals.var_t3_dn7 = assign48710_e81717_d_n7;
        locals.var_t3_dn8 = assign48710_e81717_d_n8;
        locals.var_t3_dn9 = assign48710_e81717_d_n9;
        locals.var_t3_dn10 = assign48710_e81717_d_n10;
        locals.var_t3_dn11 = assign48710_e81717_d_n11;
        locals.var_t3_rv = 0.0;

        let assign48730_e81732: f64 = if locals.var_idrec_i == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard748 = assign48730_e81732;
        locals.var_guard748_rv = 0.0;

        let (assign48750_e81751, assign48750_e81751_d_n3, assign48750_e81751_d_n4, assign48750_e81751_d_n5, assign48750_e81751_d_n6, assign48750_e81751_d_n7, assign48750_e81751_d_n8, assign48750_e81751_d_n9, assign48750_e81751_d_n10, assign48750_e81751_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48750_e81747: f64 = (locals.var_xrec_i * locals.var_t4);
        let assign48750_e81749: f64 = (assign48750_e81747 / locals.var_nrecf0_i);
        (assign48750_e81749, ((locals.var_xrec_i * locals.var_t4_dn3) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn4) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn5) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn6) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn7) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn8) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn9) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn10) / locals.var_nrecf0_i), ((locals.var_xrec_i * locals.var_t4_dn11) / locals.var_nrecf0_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign48750_e81751;
        locals.var_t7_dn3 = assign48750_e81751_d_n3;
        locals.var_t7_dn4 = assign48750_e81751_d_n4;
        locals.var_t7_dn5 = assign48750_e81751_d_n5;
        locals.var_t7_dn6 = assign48750_e81751_d_n6;
        locals.var_t7_dn7 = assign48750_e81751_d_n7;
        locals.var_t7_dn8 = assign48750_e81751_d_n8;
        locals.var_t7_dn9 = assign48750_e81751_d_n9;
        locals.var_t7_dn10 = assign48750_e81751_d_n10;
        locals.var_t7_dn11 = assign48750_e81751_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign48760_e81760, assign48760_e81760_d_n3, assign48760_e81760_d_n4, assign48760_e81760_d_n5, assign48760_e81760_d_n6, assign48760_e81760_d_n7, assign48760_e81760_d_n8, assign48760_e81760_d_n9, assign48760_e81760_d_n10, assign48760_e81760_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48760_e81758: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48760_e81758, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign48760_e81760;
        locals.var_t2_dn3 = assign48760_e81760_d_n3;
        locals.var_t2_dn4 = assign48760_e81760_d_n4;
        locals.var_t2_dn5 = assign48760_e81760_d_n5;
        locals.var_t2_dn6 = assign48760_e81760_d_n6;
        locals.var_t2_dn7 = assign48760_e81760_d_n7;
        locals.var_t2_dn8 = assign48760_e81760_d_n8;
        locals.var_t2_dn9 = assign48760_e81760_d_n9;
        locals.var_t2_dn10 = assign48760_e81760_d_n10;
        locals.var_t2_dn11 = assign48760_e81760_d_n11;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_170(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign48770_e81770, assign48770_e81770_d_n3, assign48770_e81770_d_n4, assign48770_e81770_d_n5, assign48770_e81770_d_n6, assign48770_e81770_d_n7, assign48770_e81770_d_n8, assign48770_e81770_d_n9, assign48770_e81770_d_n10, assign48770_e81770_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48770_e81768: f64 = (locals.var_idrec_i * locals.var_t2);
        (assign48770_e81768, (locals.var_idrec_i * locals.var_t2_dn3), (locals.var_idrec_i * locals.var_t2_dn4), (locals.var_idrec_i * locals.var_t2_dn5), (locals.var_idrec_i * locals.var_t2_dn6), (locals.var_idrec_i * locals.var_t2_dn7), (locals.var_idrec_i * locals.var_t2_dn8), (locals.var_idrec_i * locals.var_t2_dn9), (locals.var_idrec_i * locals.var_t2_dn10), (locals.var_idrec_i * locals.var_t2_dn11),)
    } else {
        (locals.var_jrecd, locals.var_jrecd_dn3, locals.var_jrecd_dn4, locals.var_jrecd_dn5, locals.var_jrecd_dn6, locals.var_jrecd_dn7, locals.var_jrecd_dn8, locals.var_jrecd_dn9, locals.var_jrecd_dn10, locals.var_jrecd_dn11,)
    }
};
        locals.var_jrecd = assign48770_e81770;
        locals.var_jrecd_dn3 = assign48770_e81770_d_n3;
        locals.var_jrecd_dn4 = assign48770_e81770_d_n4;
        locals.var_jrecd_dn5 = assign48770_e81770_d_n5;
        locals.var_jrecd_dn6 = assign48770_e81770_d_n6;
        locals.var_jrecd_dn7 = assign48770_e81770_d_n7;
        locals.var_jrecd_dn8 = assign48770_e81770_d_n8;
        locals.var_jrecd_dn9 = assign48770_e81770_d_n9;
        locals.var_jrecd_dn10 = assign48770_e81770_d_n10;
        locals.var_jrecd_dn11 = assign48770_e81770_d_n11;
        locals.var_jrecd_rv = 0.0;

        let (assign48780_e81788, assign48780_e81788_d_n4, assign48780_e81788_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48780_e81778: f64 = (p.p925 * locals.var_nrecf0_i);
        let assign48780_e81783: f64 = (locals.var_tratio - 1.0);
        let assign48780_e81784: f64 = (locals.var_ntrecf_i * assign48780_e81783);
        let assign48780_e81785: f64 = (1.0 + assign48780_e81784);
        let assign48780_e81786: f64 = (assign48780_e81778 * assign48780_e81785);
        (assign48780_e81786, (assign48780_e81778 * (locals.var_ntrecf_i * locals.var_tratio_dn4)), (assign48780_e81778 * (locals.var_ntrecf_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_nvtmf, locals.var_nvtmf_dn4, locals.var_nvtmf_dn5,)
    }
};
        locals.var_nvtmf = assign48780_e81788;
        locals.var_nvtmf_dn4 = assign48780_e81788_d_n4;
        locals.var_nvtmf_dn5 = assign48780_e81788_d_n5;
        locals.var_nvtmf_rv = 0.0;

        let (assign48790_e81806, assign48790_e81806_d_n4, assign48790_e81806_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48790_e81796: f64 = (p.p925 * locals.var_nrecr0_i);
        let assign48790_e81801: f64 = (locals.var_tratio - 1.0);
        let assign48790_e81802: f64 = (locals.var_ntrecr_i * assign48790_e81801);
        let assign48790_e81803: f64 = (1.0 + assign48790_e81802);
        let assign48790_e81804: f64 = (assign48790_e81796 * assign48790_e81803);
        (assign48790_e81804, (assign48790_e81796 * (locals.var_ntrecr_i * locals.var_tratio_dn4)), (assign48790_e81796 * (locals.var_ntrecr_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_nvtmr, locals.var_nvtmr_dn4, locals.var_nvtmr_dn5,)
    }
};
        locals.var_nvtmr = assign48790_e81806;
        locals.var_nvtmr_dn4 = assign48790_e81806_d_n4;
        locals.var_nvtmr_dn5 = assign48790_e81806_d_n5;
        locals.var_nvtmr_rv = 0.0;

        let (assign48800_e81816, assign48800_e81816_d_n3, assign48800_e81816_d_n4, assign48800_e81816_d_n5, assign48800_e81816_d_n6, assign48800_e81816_d_n7, assign48800_e81816_d_n8, assign48800_e81816_d_n9, assign48800_e81816_d_n10, assign48800_e81816_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48800_e81814: f64 = (locals.var_vbd_jct / locals.var_nvtmf);
        (assign48800_e81814, 0.0, (-((locals.var_vbd_jct * locals.var_nvtmf_dn4) / (locals.var_nvtmf * locals.var_nvtmf))), (-((locals.var_vbd_jct * locals.var_nvtmf_dn5) / (locals.var_nvtmf * locals.var_nvtmf))), (locals.var_vbd_jct_dn6 / locals.var_nvtmf), 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn10 / locals.var_nvtmf), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48800_e81816;
        locals.var_t0_dn3 = assign48800_e81816_d_n3;
        locals.var_t0_dn4 = assign48800_e81816_d_n4;
        locals.var_t0_dn5 = assign48800_e81816_d_n5;
        locals.var_t0_dn6 = assign48800_e81816_d_n6;
        locals.var_t0_dn7 = assign48800_e81816_d_n7;
        locals.var_t0_dn8 = assign48800_e81816_d_n8;
        locals.var_t0_dn9 = assign48800_e81816_d_n9;
        locals.var_t0_dn10 = assign48800_e81816_d_n10;
        locals.var_t0_dn11 = assign48800_e81816_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign48810_e81825, assign48810_e81825_d_n3, assign48810_e81825_d_n4, assign48810_e81825_d_n5, assign48810_e81825_d_n6, assign48810_e81825_d_n7, assign48810_e81825_d_n8, assign48810_e81825_d_n9, assign48810_e81825_d_n10, assign48810_e81825_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48810_e81823: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48810_e81823, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t10, locals.var_t10_dn3, locals.var_t10_dn4, locals.var_t10_dn5, locals.var_t10_dn6, locals.var_t10_dn7, locals.var_t10_dn8, locals.var_t10_dn9, locals.var_t10_dn10, locals.var_t10_dn11,)
    }
};
        locals.var_t10 = assign48810_e81825;
        locals.var_t10_dn3 = assign48810_e81825_d_n3;
        locals.var_t10_dn4 = assign48810_e81825_d_n4;
        locals.var_t10_dn5 = assign48810_e81825_d_n5;
        locals.var_t10_dn6 = assign48810_e81825_d_n6;
        locals.var_t10_dn7 = assign48810_e81825_d_n7;
        locals.var_t10_dn8 = assign48810_e81825_d_n8;
        locals.var_t10_dn9 = assign48810_e81825_d_n9;
        locals.var_t10_dn10 = assign48810_e81825_d_n10;
        locals.var_t10_dn11 = assign48810_e81825_d_n11;
        locals.var_t10_rv = 0.0;

        let assign48820_e81828: f64 = (locals.var_vrec0d_i - locals.var_vbd_jct);
        let assign48820_e81830: f64 = if assign48820_e81828 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard749 = assign48820_e81830;
        locals.var_guard749_rv = 0.0;

        let (assign48830_e81840, assign48830_e81840_d_n3, assign48830_e81840_d_n4, assign48830_e81840_d_n5, assign48830_e81840_d_n6, assign48830_e81840_d_n7, assign48830_e81840_d_n8, assign48830_e81840_d_n9, assign48830_e81840_d_n10, assign48830_e81840_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign48830_e81840;
        locals.var_t1_dn3 = assign48830_e81840_d_n3;
        locals.var_t1_dn4 = assign48830_e81840_d_n4;
        locals.var_t1_dn5 = assign48830_e81840_d_n5;
        locals.var_t1_dn6 = assign48830_e81840_d_n6;
        locals.var_t1_dn7 = assign48830_e81840_d_n7;
        locals.var_t1_dn8 = assign48830_e81840_d_n8;
        locals.var_t1_dn9 = assign48830_e81840_d_n9;
        locals.var_t1_dn10 = assign48830_e81840_d_n10;
        locals.var_t1_dn11 = assign48830_e81840_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign48840_e81857, assign48840_e81857_d_n3, assign48840_e81857_d_n4, assign48840_e81857_d_n5, assign48840_e81857_d_n6, assign48840_e81857_d_n7, assign48840_e81857_d_n8, assign48840_e81857_d_n9, assign48840_e81857_d_n10, assign48840_e81857_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 != 0.0)) {
        let assign48840_e81849: f64 = (-locals.var_vbd_jct);
        let assign48840_e81851: f64 = (assign48840_e81849 / locals.var_nvtmr);
        let assign48840_e81853: f64 = (assign48840_e81851 * locals.var_vrec0d_i);
        let assign48840_e81855: f64 = (assign48840_e81853 * locals.var_t1);
        (assign48840_e81855, (assign48840_e81853 * locals.var_t1_dn3), ((((-((assign48840_e81849 * locals.var_nvtmr_dn4) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0d_i) * locals.var_t1) + (assign48840_e81853 * locals.var_t1_dn4)), ((((-((assign48840_e81849 * locals.var_nvtmr_dn5) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0d_i) * locals.var_t1) + (assign48840_e81853 * locals.var_t1_dn5)), (((((-locals.var_vbd_jct_dn6) / locals.var_nvtmr) * locals.var_vrec0d_i) * locals.var_t1) + (assign48840_e81853 * locals.var_t1_dn6)), (assign48840_e81853 * locals.var_t1_dn7), (assign48840_e81853 * locals.var_t1_dn8), (assign48840_e81853 * locals.var_t1_dn9), (((((-locals.var_vbd_jct_dn10) / locals.var_nvtmr) * locals.var_vrec0d_i) * locals.var_t1) + (assign48840_e81853 * locals.var_t1_dn10)), (assign48840_e81853 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48840_e81857;
        locals.var_t0_dn3 = assign48840_e81857_d_n3;
        locals.var_t0_dn4 = assign48840_e81857_d_n4;
        locals.var_t0_dn5 = assign48840_e81857_d_n5;
        locals.var_t0_dn6 = assign48840_e81857_d_n6;
        locals.var_t0_dn7 = assign48840_e81857_d_n7;
        locals.var_t0_dn8 = assign48840_e81857_d_n8;
        locals.var_t0_dn9 = assign48840_e81857_d_n9;
        locals.var_t0_dn10 = assign48840_e81857_d_n10;
        locals.var_t0_dn11 = assign48840_e81857_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign48850_e81868, assign48850_e81868_d_n3, assign48850_e81868_d_n4, assign48850_e81868_d_n5, assign48850_e81868_d_n6, assign48850_e81868_d_n7, assign48850_e81868_d_n8, assign48850_e81868_d_n9, assign48850_e81868_d_n10, assign48850_e81868_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 != 0.0)) {
        let assign48850_e81866: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48850_e81866, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48850_e81868;
        locals.var_t11_dn3 = assign48850_e81868_d_n3;
        locals.var_t11_dn4 = assign48850_e81868_d_n4;
        locals.var_t11_dn5 = assign48850_e81868_d_n5;
        locals.var_t11_dn6 = assign48850_e81868_d_n6;
        locals.var_t11_dn7 = assign48850_e81868_d_n7;
        locals.var_t11_dn8 = assign48850_e81868_d_n8;
        locals.var_t11_dn9 = assign48850_e81868_d_n9;
        locals.var_t11_dn10 = assign48850_e81868_d_n10;
        locals.var_t11_dn11 = assign48850_e81868_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign48860_e81879, assign48860_e81879_d_n3, assign48860_e81879_d_n4, assign48860_e81879_d_n5, assign48860_e81879_d_n6, assign48860_e81879_d_n7, assign48860_e81879_d_n8, assign48860_e81879_d_n9, assign48860_e81879_d_n10, assign48860_e81879_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 != 0.0)) {
        let assign48860_e81877: f64 = (-locals.var_t11);
        (assign48860_e81877, (-locals.var_t11_dn3), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48860_e81879;
        locals.var_t11_dn3 = assign48860_e81879_d_n3;
        locals.var_t11_dn4 = assign48860_e81879_d_n4;
        locals.var_t11_dn5 = assign48860_e81879_d_n5;
        locals.var_t11_dn6 = assign48860_e81879_d_n6;
        locals.var_t11_dn7 = assign48860_e81879_d_n7;
        locals.var_t11_dn8 = assign48860_e81879_d_n8;
        locals.var_t11_dn9 = assign48860_e81879_d_n9;
        locals.var_t11_dn10 = assign48860_e81879_d_n10;
        locals.var_t11_dn11 = assign48860_e81879_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign48870_e81894, assign48870_e81894_d_n3, assign48870_e81894_d_n4, assign48870_e81894_d_n5, assign48870_e81894_d_n6, assign48870_e81894_d_n7, assign48870_e81894_d_n8, assign48870_e81894_d_n9, assign48870_e81894_d_n10, assign48870_e81894_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 == 0.0)) {
        let assign48870_e81891: f64 = (locals.var_vrec0d_i - locals.var_vbd_jct);
        let assign48870_e81892: f64 = (1.0 / assign48870_e81891);
        (assign48870_e81892, 0.0, 0.0, 0.0, (-((-locals.var_vbd_jct_dn6) / (assign48870_e81891 * assign48870_e81891))), 0.0, 0.0, 0.0, (-((-locals.var_vbd_jct_dn10) / (assign48870_e81891 * assign48870_e81891))), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign48870_e81894;
        locals.var_t1_dn3 = assign48870_e81894_d_n3;
        locals.var_t1_dn4 = assign48870_e81894_d_n4;
        locals.var_t1_dn5 = assign48870_e81894_d_n5;
        locals.var_t1_dn6 = assign48870_e81894_d_n6;
        locals.var_t1_dn7 = assign48870_e81894_d_n7;
        locals.var_t1_dn8 = assign48870_e81894_d_n8;
        locals.var_t1_dn9 = assign48870_e81894_d_n9;
        locals.var_t1_dn10 = assign48870_e81894_d_n10;
        locals.var_t1_dn11 = assign48870_e81894_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign48880_e81912, assign48880_e81912_d_n3, assign48880_e81912_d_n4, assign48880_e81912_d_n5, assign48880_e81912_d_n6, assign48880_e81912_d_n7, assign48880_e81912_d_n8, assign48880_e81912_d_n9, assign48880_e81912_d_n10, assign48880_e81912_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 == 0.0)) {
        let assign48880_e81904: f64 = (-locals.var_vbd_jct);
        let assign48880_e81906: f64 = (assign48880_e81904 / locals.var_nvtmr);
        let assign48880_e81908: f64 = (assign48880_e81906 * locals.var_vrec0d_i);
        let assign48880_e81910: f64 = (assign48880_e81908 * locals.var_t1);
        (assign48880_e81910, (assign48880_e81908 * locals.var_t1_dn3), ((((-((assign48880_e81904 * locals.var_nvtmr_dn4) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0d_i) * locals.var_t1) + (assign48880_e81908 * locals.var_t1_dn4)), ((((-((assign48880_e81904 * locals.var_nvtmr_dn5) / (locals.var_nvtmr * locals.var_nvtmr))) * locals.var_vrec0d_i) * locals.var_t1) + (assign48880_e81908 * locals.var_t1_dn5)), (((((-locals.var_vbd_jct_dn6) / locals.var_nvtmr) * locals.var_vrec0d_i) * locals.var_t1) + (assign48880_e81908 * locals.var_t1_dn6)), (assign48880_e81908 * locals.var_t1_dn7), (assign48880_e81908 * locals.var_t1_dn8), (assign48880_e81908 * locals.var_t1_dn9), (((((-locals.var_vbd_jct_dn10) / locals.var_nvtmr) * locals.var_vrec0d_i) * locals.var_t1) + (assign48880_e81908 * locals.var_t1_dn10)), (assign48880_e81908 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48880_e81912;
        locals.var_t0_dn3 = assign48880_e81912_d_n3;
        locals.var_t0_dn4 = assign48880_e81912_d_n4;
        locals.var_t0_dn5 = assign48880_e81912_d_n5;
        locals.var_t0_dn6 = assign48880_e81912_d_n6;
        locals.var_t0_dn7 = assign48880_e81912_d_n7;
        locals.var_t0_dn8 = assign48880_e81912_d_n8;
        locals.var_t0_dn9 = assign48880_e81912_d_n9;
        locals.var_t0_dn10 = assign48880_e81912_d_n10;
        locals.var_t0_dn11 = assign48880_e81912_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign48890_e81924, assign48890_e81924_d_n3, assign48890_e81924_d_n4, assign48890_e81924_d_n5, assign48890_e81924_d_n6, assign48890_e81924_d_n7, assign48890_e81924_d_n8, assign48890_e81924_d_n9, assign48890_e81924_d_n10, assign48890_e81924_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 == 0.0)) {
        let assign48890_e81922: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48890_e81922, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48890_e81924;
        locals.var_t11_dn3 = assign48890_e81924_d_n3;
        locals.var_t11_dn4 = assign48890_e81924_d_n4;
        locals.var_t11_dn5 = assign48890_e81924_d_n5;
        locals.var_t11_dn6 = assign48890_e81924_d_n6;
        locals.var_t11_dn7 = assign48890_e81924_d_n7;
        locals.var_t11_dn8 = assign48890_e81924_d_n8;
        locals.var_t11_dn9 = assign48890_e81924_d_n9;
        locals.var_t11_dn10 = assign48890_e81924_d_n10;
        locals.var_t11_dn11 = assign48890_e81924_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign48900_e81936, assign48900_e81936_d_n3, assign48900_e81936_d_n4, assign48900_e81936_d_n5, assign48900_e81936_d_n6, assign48900_e81936_d_n7, assign48900_e81936_d_n8, assign48900_e81936_d_n9, assign48900_e81936_d_n10, assign48900_e81936_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) && (locals.var_guard749 == 0.0)) {
        let assign48900_e81934: f64 = (-locals.var_t11);
        (assign48900_e81934, (-locals.var_t11_dn3), (-locals.var_t11_dn4), (-locals.var_t11_dn5), (-locals.var_t11_dn6), (-locals.var_t11_dn7), (-locals.var_t11_dn8), (-locals.var_t11_dn9), (-locals.var_t11_dn10), (-locals.var_t11_dn11),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign48900_e81936;
        locals.var_t11_dn3 = assign48900_e81936_d_n3;
        locals.var_t11_dn4 = assign48900_e81936_d_n4;
        locals.var_t11_dn5 = assign48900_e81936_d_n5;
        locals.var_t11_dn6 = assign48900_e81936_d_n6;
        locals.var_t11_dn7 = assign48900_e81936_d_n7;
        locals.var_t11_dn8 = assign48900_e81936_d_n8;
        locals.var_t11_dn9 = assign48900_e81936_d_n9;
        locals.var_t11_dn10 = assign48900_e81936_d_n10;
        locals.var_t11_dn11 = assign48900_e81936_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign48910_e81946, assign48910_e81946_d_n3, assign48910_e81946_d_n4, assign48910_e81946_d_n5, assign48910_e81946_d_n6, assign48910_e81946_d_n7, assign48910_e81946_d_n8, assign48910_e81946_d_n9, assign48910_e81946_d_n10, assign48910_e81946_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard748 == 0.0)) {
        let assign48910_e81944: f64 = (locals.var_wdtsi * locals.var_jrecd);
        (assign48910_e81944, (locals.var_wdtsi * locals.var_jrecd_dn3), (locals.var_wdtsi * locals.var_jrecd_dn4), (locals.var_wdtsi * locals.var_jrecd_dn5), (locals.var_wdtsi * locals.var_jrecd_dn6), (locals.var_wdtsi * locals.var_jrecd_dn7), (locals.var_wdtsi * locals.var_jrecd_dn8), (locals.var_wdtsi * locals.var_jrecd_dn9), (locals.var_wdtsi * locals.var_jrecd_dn10), (locals.var_wdtsi * locals.var_jrecd_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign48910_e81946;
        locals.var_t3_dn3 = assign48910_e81946_d_n3;
        locals.var_t3_dn4 = assign48910_e81946_d_n4;
        locals.var_t3_dn5 = assign48910_e81946_d_n5;
        locals.var_t3_dn6 = assign48910_e81946_d_n6;
        locals.var_t3_dn7 = assign48910_e81946_d_n7;
        locals.var_t3_dn8 = assign48910_e81946_d_n8;
        locals.var_t3_dn9 = assign48910_e81946_d_n9;
        locals.var_t3_dn10 = assign48910_e81946_d_n10;
        locals.var_t3_dn11 = assign48910_e81946_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign48930_e81967,) = {
    if (locals.var_guard492 == 0.0) {
        let assign48930_e81963: f64 = (locals.var_weff / p.p1373);
        let assign48930_e81965: f64 = (assign48930_e81963 * p.p74);
        (assign48930_e81965,)
    } else {
        (locals.var_wtsi,)
    }
};
        locals.var_wtsi = assign48930_e81967;
        locals.var_wtsi_rv = 0.0;

        let assign48940_e81974: f64 = if ((locals.var_isbjt_i == 0.0) && (locals.var_idbjt_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard750 = assign48940_e81974;
        locals.var_guard750_rv = 0.0;

        let (assign48980_e82007, assign48980_e82007_d_n3, assign48980_e82007_d_n4, assign48980_e82007_d_n5, assign48980_e82007_d_n6, assign48980_e82007_d_n7, assign48980_e82007_d_n8, assign48980_e82007_d_n9, assign48980_e82007_d_n10, assign48980_e82007_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign48980_e82003: f64 = (locals.var_xbjt_i * locals.var_t4);
        let assign48980_e82005: f64 = (assign48980_e82003 / locals.var_ndiode_i);
        (assign48980_e82005, ((locals.var_xbjt_i * locals.var_t4_dn3) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn4) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn5) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn6) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn7) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn8) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn9) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn10) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn11) / locals.var_ndiode_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign48980_e82007;
        locals.var_t7_dn3 = assign48980_e82007_d_n3;
        locals.var_t7_dn4 = assign48980_e82007_d_n4;
        locals.var_t7_dn5 = assign48980_e82007_d_n5;
        locals.var_t7_dn6 = assign48980_e82007_d_n6;
        locals.var_t7_dn7 = assign48980_e82007_d_n7;
        locals.var_t7_dn8 = assign48980_e82007_d_n8;
        locals.var_t7_dn9 = assign48980_e82007_d_n9;
        locals.var_t7_dn10 = assign48980_e82007_d_n10;
        locals.var_t7_dn11 = assign48980_e82007_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign48990_e82016, assign48990_e82016_d_n3, assign48990_e82016_d_n4, assign48990_e82016_d_n5, assign48990_e82016_d_n6, assign48990_e82016_d_n7, assign48990_e82016_d_n8, assign48990_e82016_d_n9, assign48990_e82016_d_n10, assign48990_e82016_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign48990_e82014: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign48990_e82014, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign48990_e82016;
        locals.var_t0_dn3 = assign48990_e82016_d_n3;
        locals.var_t0_dn4 = assign48990_e82016_d_n4;
        locals.var_t0_dn5 = assign48990_e82016_d_n5;
        locals.var_t0_dn6 = assign48990_e82016_d_n6;
        locals.var_t0_dn7 = assign48990_e82016_d_n7;
        locals.var_t0_dn8 = assign48990_e82016_d_n8;
        locals.var_t0_dn9 = assign48990_e82016_d_n9;
        locals.var_t0_dn10 = assign48990_e82016_d_n10;
        locals.var_t0_dn11 = assign48990_e82016_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49000_e82026, assign49000_e82026_d_n3, assign49000_e82026_d_n4, assign49000_e82026_d_n5, assign49000_e82026_d_n6, assign49000_e82026_d_n7, assign49000_e82026_d_n8, assign49000_e82026_d_n9, assign49000_e82026_d_n10, assign49000_e82026_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49000_e82024: f64 = (locals.var_ahli_i * locals.var_t0);
        (assign49000_e82024, (locals.var_ahli_i * locals.var_t0_dn3), (locals.var_ahli_i * locals.var_t0_dn4), (locals.var_ahli_i * locals.var_t0_dn5), (locals.var_ahli_i * locals.var_t0_dn6), (locals.var_ahli_i * locals.var_t0_dn7), (locals.var_ahli_i * locals.var_t0_dn8), (locals.var_ahli_i * locals.var_t0_dn9), (locals.var_ahli_i * locals.var_t0_dn10), (locals.var_ahli_i * locals.var_t0_dn11),)
    } else {
        (locals.var_ahlis, locals.var_ahlis_dn3, locals.var_ahlis_dn4, locals.var_ahlis_dn5, locals.var_ahlis_dn6, locals.var_ahlis_dn7, locals.var_ahlis_dn8, locals.var_ahlis_dn9, locals.var_ahlis_dn10, locals.var_ahlis_dn11,)
    }
};
        locals.var_ahlis = assign49000_e82026;
        locals.var_ahlis_dn3 = assign49000_e82026_d_n3;
        locals.var_ahlis_dn4 = assign49000_e82026_d_n4;
        locals.var_ahlis_dn5 = assign49000_e82026_d_n5;
        locals.var_ahlis_dn6 = assign49000_e82026_d_n6;
        locals.var_ahlis_dn7 = assign49000_e82026_d_n7;
        locals.var_ahlis_dn8 = assign49000_e82026_d_n8;
        locals.var_ahlis_dn9 = assign49000_e82026_d_n9;
        locals.var_ahlis_dn10 = assign49000_e82026_d_n10;
        locals.var_ahlis_dn11 = assign49000_e82026_d_n11;
        locals.var_ahlis_rv = 0.0;

        let (assign49010_e82036, assign49010_e82036_d_n3, assign49010_e82036_d_n4, assign49010_e82036_d_n5, assign49010_e82036_d_n6, assign49010_e82036_d_n7, assign49010_e82036_d_n8, assign49010_e82036_d_n9, assign49010_e82036_d_n10, assign49010_e82036_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49010_e82034: f64 = (locals.var_isbjt_i * locals.var_t0);
        (assign49010_e82034, (locals.var_isbjt_i * locals.var_t0_dn3), (locals.var_isbjt_i * locals.var_t0_dn4), (locals.var_isbjt_i * locals.var_t0_dn5), (locals.var_isbjt_i * locals.var_t0_dn6), (locals.var_isbjt_i * locals.var_t0_dn7), (locals.var_isbjt_i * locals.var_t0_dn8), (locals.var_isbjt_i * locals.var_t0_dn9), (locals.var_isbjt_i * locals.var_t0_dn10), (locals.var_isbjt_i * locals.var_t0_dn11),)
    } else {
        (locals.var_jbjts, locals.var_jbjts_dn3, locals.var_jbjts_dn4, locals.var_jbjts_dn5, locals.var_jbjts_dn6, locals.var_jbjts_dn7, locals.var_jbjts_dn8, locals.var_jbjts_dn9, locals.var_jbjts_dn10, locals.var_jbjts_dn11,)
    }
};
        locals.var_jbjts = assign49010_e82036;
        locals.var_jbjts_dn3 = assign49010_e82036_d_n3;
        locals.var_jbjts_dn4 = assign49010_e82036_d_n4;
        locals.var_jbjts_dn5 = assign49010_e82036_d_n5;
        locals.var_jbjts_dn6 = assign49010_e82036_d_n6;
        locals.var_jbjts_dn7 = assign49010_e82036_d_n7;
        locals.var_jbjts_dn8 = assign49010_e82036_d_n8;
        locals.var_jbjts_dn9 = assign49010_e82036_d_n9;
        locals.var_jbjts_dn10 = assign49010_e82036_d_n10;
        locals.var_jbjts_dn11 = assign49010_e82036_d_n11;
        locals.var_jbjts_rv = 0.0;

        let (assign49020_e82048, assign49020_e82048_d_n3, assign49020_e82048_d_n4, assign49020_e82048_d_n5, assign49020_e82048_d_n6, assign49020_e82048_d_n7, assign49020_e82048_d_n8, assign49020_e82048_d_n9, assign49020_e82048_d_n10, assign49020_e82048_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49020_e82044: f64 = (locals.var_xbjt_i * locals.var_t4);
        let assign49020_e82046: f64 = (assign49020_e82044 / locals.var_ndiode_i);
        (assign49020_e82046, ((locals.var_xbjt_i * locals.var_t4_dn3) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn4) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn5) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn6) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn7) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn8) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn9) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn10) / locals.var_ndiode_i), ((locals.var_xbjt_i * locals.var_t4_dn11) / locals.var_ndiode_i),)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign49020_e82048;
        locals.var_t7_dn3 = assign49020_e82048_d_n3;
        locals.var_t7_dn4 = assign49020_e82048_d_n4;
        locals.var_t7_dn5 = assign49020_e82048_d_n5;
        locals.var_t7_dn6 = assign49020_e82048_d_n6;
        locals.var_t7_dn7 = assign49020_e82048_d_n7;
        locals.var_t7_dn8 = assign49020_e82048_d_n8;
        locals.var_t7_dn9 = assign49020_e82048_d_n9;
        locals.var_t7_dn10 = assign49020_e82048_d_n10;
        locals.var_t7_dn11 = assign49020_e82048_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign49030_e82057, assign49030_e82057_d_n3, assign49030_e82057_d_n4, assign49030_e82057_d_n5, assign49030_e82057_d_n6, assign49030_e82057_d_n7, assign49030_e82057_d_n8, assign49030_e82057_d_n9, assign49030_e82057_d_n10, assign49030_e82057_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49030_e82055: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49030_e82055, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49030_e82057;
        locals.var_t0_dn3 = assign49030_e82057_d_n3;
        locals.var_t0_dn4 = assign49030_e82057_d_n4;
        locals.var_t0_dn5 = assign49030_e82057_d_n5;
        locals.var_t0_dn6 = assign49030_e82057_d_n6;
        locals.var_t0_dn7 = assign49030_e82057_d_n7;
        locals.var_t0_dn8 = assign49030_e82057_d_n8;
        locals.var_t0_dn9 = assign49030_e82057_d_n9;
        locals.var_t0_dn10 = assign49030_e82057_d_n10;
        locals.var_t0_dn11 = assign49030_e82057_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49040_e82067, assign49040_e82067_d_n3, assign49040_e82067_d_n4, assign49040_e82067_d_n5, assign49040_e82067_d_n6, assign49040_e82067_d_n7, assign49040_e82067_d_n8, assign49040_e82067_d_n9, assign49040_e82067_d_n10, assign49040_e82067_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49040_e82065: f64 = (locals.var_ahlid_i * locals.var_t0);
        (assign49040_e82065, (locals.var_ahlid_i * locals.var_t0_dn3), (locals.var_ahlid_i * locals.var_t0_dn4), (locals.var_ahlid_i * locals.var_t0_dn5), (locals.var_ahlid_i * locals.var_t0_dn6), (locals.var_ahlid_i * locals.var_t0_dn7), (locals.var_ahlid_i * locals.var_t0_dn8), (locals.var_ahlid_i * locals.var_t0_dn9), (locals.var_ahlid_i * locals.var_t0_dn10), (locals.var_ahlid_i * locals.var_t0_dn11),)
    } else {
        (locals.var_ahlid, locals.var_ahlid_dn3, locals.var_ahlid_dn4, locals.var_ahlid_dn5, locals.var_ahlid_dn6, locals.var_ahlid_dn7, locals.var_ahlid_dn8, locals.var_ahlid_dn9, locals.var_ahlid_dn10, locals.var_ahlid_dn11,)
    }
};
        locals.var_ahlid = assign49040_e82067;
        locals.var_ahlid_dn3 = assign49040_e82067_d_n3;
        locals.var_ahlid_dn4 = assign49040_e82067_d_n4;
        locals.var_ahlid_dn5 = assign49040_e82067_d_n5;
        locals.var_ahlid_dn6 = assign49040_e82067_d_n6;
        locals.var_ahlid_dn7 = assign49040_e82067_d_n7;
        locals.var_ahlid_dn8 = assign49040_e82067_d_n8;
        locals.var_ahlid_dn9 = assign49040_e82067_d_n9;
        locals.var_ahlid_dn10 = assign49040_e82067_d_n10;
        locals.var_ahlid_dn11 = assign49040_e82067_d_n11;
        locals.var_ahlid_rv = 0.0;

        let (assign49050_e82077, assign49050_e82077_d_n3, assign49050_e82077_d_n4, assign49050_e82077_d_n5, assign49050_e82077_d_n6, assign49050_e82077_d_n7, assign49050_e82077_d_n8, assign49050_e82077_d_n9, assign49050_e82077_d_n10, assign49050_e82077_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49050_e82075: f64 = (locals.var_idbjt_i * locals.var_t0);
        (assign49050_e82075, (locals.var_idbjt_i * locals.var_t0_dn3), (locals.var_idbjt_i * locals.var_t0_dn4), (locals.var_idbjt_i * locals.var_t0_dn5), (locals.var_idbjt_i * locals.var_t0_dn6), (locals.var_idbjt_i * locals.var_t0_dn7), (locals.var_idbjt_i * locals.var_t0_dn8), (locals.var_idbjt_i * locals.var_t0_dn9), (locals.var_idbjt_i * locals.var_t0_dn10), (locals.var_idbjt_i * locals.var_t0_dn11),)
    } else {
        (locals.var_jbjtd, locals.var_jbjtd_dn3, locals.var_jbjtd_dn4, locals.var_jbjtd_dn5, locals.var_jbjtd_dn6, locals.var_jbjtd_dn7, locals.var_jbjtd_dn8, locals.var_jbjtd_dn9, locals.var_jbjtd_dn10, locals.var_jbjtd_dn11,)
    }
};
        locals.var_jbjtd = assign49050_e82077;
        locals.var_jbjtd_dn3 = assign49050_e82077_d_n3;
        locals.var_jbjtd_dn4 = assign49050_e82077_d_n4;
        locals.var_jbjtd_dn5 = assign49050_e82077_d_n5;
        locals.var_jbjtd_dn6 = assign49050_e82077_d_n6;
        locals.var_jbjtd_dn7 = assign49050_e82077_d_n7;
        locals.var_jbjtd_dn8 = assign49050_e82077_d_n8;
        locals.var_jbjtd_dn9 = assign49050_e82077_d_n9;
        locals.var_jbjtd_dn10 = assign49050_e82077_d_n10;
        locals.var_jbjtd_dn11 = assign49050_e82077_d_n11;
        locals.var_jbjtd_rv = 0.0;

        let (assign49060_e82089, assign49060_e82089_d_n3, assign49060_e82089_d_n4, assign49060_e82089_d_n5, assign49060_e82089_d_n6, assign49060_e82089_d_n7, assign49060_e82089_d_n8, assign49060_e82089_d_n9, assign49060_e82089_d_n10, assign49060_e82089_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49060_e82086: f64 = (locals.var_expvbsnvtm - 1.0);
        let assign49060_e82087: f64 = (locals.var_ahlis * assign49060_e82086);
        (assign49060_e82087, ((locals.var_ahlis_dn3 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn3)), ((locals.var_ahlis_dn4 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn4)), ((locals.var_ahlis_dn5 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn5)), ((locals.var_ahlis_dn6 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn6)), ((locals.var_ahlis_dn7 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn7)), ((locals.var_ahlis_dn8 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn8)), ((locals.var_ahlis_dn9 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn9)), ((locals.var_ahlis_dn10 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn10)), ((locals.var_ahlis_dn11 * assign49060_e82086) + (locals.var_ahlis * locals.var_expvbsnvtm_dn11)),)
    } else {
        (locals.var_ehlis, locals.var_ehlis_dn3, locals.var_ehlis_dn4, locals.var_ehlis_dn5, locals.var_ehlis_dn6, locals.var_ehlis_dn7, locals.var_ehlis_dn8, locals.var_ehlis_dn9, locals.var_ehlis_dn10, locals.var_ehlis_dn11,)
    }
};
        locals.var_ehlis = assign49060_e82089;
        locals.var_ehlis_dn3 = assign49060_e82089_d_n3;
        locals.var_ehlis_dn4 = assign49060_e82089_d_n4;
        locals.var_ehlis_dn5 = assign49060_e82089_d_n5;
        locals.var_ehlis_dn6 = assign49060_e82089_d_n6;
        locals.var_ehlis_dn7 = assign49060_e82089_d_n7;
        locals.var_ehlis_dn8 = assign49060_e82089_d_n8;
        locals.var_ehlis_dn9 = assign49060_e82089_d_n9;
        locals.var_ehlis_dn10 = assign49060_e82089_d_n10;
        locals.var_ehlis_dn11 = assign49060_e82089_d_n11;
        locals.var_ehlis_rv = 0.0;

        let assign49070_e82092: f64 = if locals.var_ehlis < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard751 = assign49070_e82092;
        locals.var_guard751_rv = 0.0;

        let (assign49080_e82102, assign49080_e82102_d_n3, assign49080_e82102_d_n4, assign49080_e82102_d_n5, assign49080_e82102_d_n6, assign49080_e82102_d_n7, assign49080_e82102_d_n8, assign49080_e82102_d_n9, assign49080_e82102_d_n10, assign49080_e82102_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard751 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlis, locals.var_ehlis_dn3, locals.var_ehlis_dn4, locals.var_ehlis_dn5, locals.var_ehlis_dn6, locals.var_ehlis_dn7, locals.var_ehlis_dn8, locals.var_ehlis_dn9, locals.var_ehlis_dn10, locals.var_ehlis_dn11,)
    }
};
        locals.var_ehlis = assign49080_e82102;
        locals.var_ehlis_dn3 = assign49080_e82102_d_n3;
        locals.var_ehlis_dn4 = assign49080_e82102_d_n4;
        locals.var_ehlis_dn5 = assign49080_e82102_d_n5;
        locals.var_ehlis_dn6 = assign49080_e82102_d_n6;
        locals.var_ehlis_dn7 = assign49080_e82102_d_n7;
        locals.var_ehlis_dn8 = assign49080_e82102_d_n8;
        locals.var_ehlis_dn9 = assign49080_e82102_d_n9;
        locals.var_ehlis_dn10 = assign49080_e82102_d_n10;
        locals.var_ehlis_dn11 = assign49080_e82102_d_n11;
        locals.var_ehlis_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_171(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign49090_e82112, assign49090_e82112_d_n3, assign49090_e82112_d_n4, assign49090_e82112_d_n5, assign49090_e82112_d_n6, assign49090_e82112_d_n7, assign49090_e82112_d_n8, assign49090_e82112_d_n9, assign49090_e82112_d_n10, assign49090_e82112_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard751 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlisfactor, locals.var_ehlisfactor_dn3, locals.var_ehlisfactor_dn4, locals.var_ehlisfactor_dn5, locals.var_ehlisfactor_dn6, locals.var_ehlisfactor_dn7, locals.var_ehlisfactor_dn8, locals.var_ehlisfactor_dn9, locals.var_ehlisfactor_dn10, locals.var_ehlisfactor_dn11,)
    }
};
        locals.var_ehlisfactor = assign49090_e82112;
        locals.var_ehlisfactor_dn3 = assign49090_e82112_d_n3;
        locals.var_ehlisfactor_dn4 = assign49090_e82112_d_n4;
        locals.var_ehlisfactor_dn5 = assign49090_e82112_d_n5;
        locals.var_ehlisfactor_dn6 = assign49090_e82112_d_n6;
        locals.var_ehlisfactor_dn7 = assign49090_e82112_d_n7;
        locals.var_ehlisfactor_dn8 = assign49090_e82112_d_n8;
        locals.var_ehlisfactor_dn9 = assign49090_e82112_d_n9;
        locals.var_ehlisfactor_dn10 = assign49090_e82112_d_n10;
        locals.var_ehlisfactor_dn11 = assign49090_e82112_d_n11;
        locals.var_ehlisfactor_rv = 0.0;

        let (assign49100_e82128, assign49100_e82128_d_n3, assign49100_e82128_d_n4, assign49100_e82128_d_n5, assign49100_e82128_d_n6, assign49100_e82128_d_n7, assign49100_e82128_d_n8, assign49100_e82128_d_n9, assign49100_e82128_d_n10, assign49100_e82128_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard751 == 0.0)) {
        let assign49100_e82124: f64 = (1.0 + locals.var_ehlis);
        let assign49100_e82125: f64 = (assign49100_e82124).sqrt();
        let assign49100_e82126: f64 = (1.0 / assign49100_e82125);
        (assign49100_e82126, (-((locals.var_ehlis_dn3 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn4 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn5 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn6 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn7 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn8 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn9 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn10 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))), (-((locals.var_ehlis_dn11 / (2.0 * assign49100_e82125)) / (assign49100_e82125 * assign49100_e82125))),)
    } else {
        (locals.var_ehlisfactor, locals.var_ehlisfactor_dn3, locals.var_ehlisfactor_dn4, locals.var_ehlisfactor_dn5, locals.var_ehlisfactor_dn6, locals.var_ehlisfactor_dn7, locals.var_ehlisfactor_dn8, locals.var_ehlisfactor_dn9, locals.var_ehlisfactor_dn10, locals.var_ehlisfactor_dn11,)
    }
};
        locals.var_ehlisfactor = assign49100_e82128;
        locals.var_ehlisfactor_dn3 = assign49100_e82128_d_n3;
        locals.var_ehlisfactor_dn4 = assign49100_e82128_d_n4;
        locals.var_ehlisfactor_dn5 = assign49100_e82128_d_n5;
        locals.var_ehlisfactor_dn6 = assign49100_e82128_d_n6;
        locals.var_ehlisfactor_dn7 = assign49100_e82128_d_n7;
        locals.var_ehlisfactor_dn8 = assign49100_e82128_d_n8;
        locals.var_ehlisfactor_dn9 = assign49100_e82128_d_n9;
        locals.var_ehlisfactor_dn10 = assign49100_e82128_d_n10;
        locals.var_ehlisfactor_dn11 = assign49100_e82128_d_n11;
        locals.var_ehlisfactor_rv = 0.0;

        let (assign49110_e82140, assign49110_e82140_d_n3, assign49110_e82140_d_n4, assign49110_e82140_d_n5, assign49110_e82140_d_n6, assign49110_e82140_d_n7, assign49110_e82140_d_n8, assign49110_e82140_d_n9, assign49110_e82140_d_n10, assign49110_e82140_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49110_e82137: f64 = (locals.var_expvbdnvtm - 1.0);
        let assign49110_e82138: f64 = (locals.var_ahlid * assign49110_e82137);
        (assign49110_e82138, ((locals.var_ahlid_dn3 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn3)), ((locals.var_ahlid_dn4 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn4)), ((locals.var_ahlid_dn5 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn5)), ((locals.var_ahlid_dn6 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn6)), ((locals.var_ahlid_dn7 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn7)), ((locals.var_ahlid_dn8 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn8)), ((locals.var_ahlid_dn9 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn9)), ((locals.var_ahlid_dn10 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn10)), ((locals.var_ahlid_dn11 * assign49110_e82137) + (locals.var_ahlid * locals.var_expvbdnvtm_dn11)),)
    } else {
        (locals.var_ehlid, locals.var_ehlid_dn3, locals.var_ehlid_dn4, locals.var_ehlid_dn5, locals.var_ehlid_dn6, locals.var_ehlid_dn7, locals.var_ehlid_dn8, locals.var_ehlid_dn9, locals.var_ehlid_dn10, locals.var_ehlid_dn11,)
    }
};
        locals.var_ehlid = assign49110_e82140;
        locals.var_ehlid_dn3 = assign49110_e82140_d_n3;
        locals.var_ehlid_dn4 = assign49110_e82140_d_n4;
        locals.var_ehlid_dn5 = assign49110_e82140_d_n5;
        locals.var_ehlid_dn6 = assign49110_e82140_d_n6;
        locals.var_ehlid_dn7 = assign49110_e82140_d_n7;
        locals.var_ehlid_dn8 = assign49110_e82140_d_n8;
        locals.var_ehlid_dn9 = assign49110_e82140_d_n9;
        locals.var_ehlid_dn10 = assign49110_e82140_d_n10;
        locals.var_ehlid_dn11 = assign49110_e82140_d_n11;
        locals.var_ehlid_rv = 0.0;

        let assign49120_e82143: f64 = if locals.var_ehlid < 1e-5 { 1.0 } else { 0.0 };
        locals.var_guard752 = assign49120_e82143;
        locals.var_guard752_rv = 0.0;

        let (assign49130_e82153, assign49130_e82153_d_n3, assign49130_e82153_d_n4, assign49130_e82153_d_n5, assign49130_e82153_d_n6, assign49130_e82153_d_n7, assign49130_e82153_d_n8, assign49130_e82153_d_n9, assign49130_e82153_d_n10, assign49130_e82153_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard752 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlid, locals.var_ehlid_dn3, locals.var_ehlid_dn4, locals.var_ehlid_dn5, locals.var_ehlid_dn6, locals.var_ehlid_dn7, locals.var_ehlid_dn8, locals.var_ehlid_dn9, locals.var_ehlid_dn10, locals.var_ehlid_dn11,)
    }
};
        locals.var_ehlid = assign49130_e82153;
        locals.var_ehlid_dn3 = assign49130_e82153_d_n3;
        locals.var_ehlid_dn4 = assign49130_e82153_d_n4;
        locals.var_ehlid_dn5 = assign49130_e82153_d_n5;
        locals.var_ehlid_dn6 = assign49130_e82153_d_n6;
        locals.var_ehlid_dn7 = assign49130_e82153_d_n7;
        locals.var_ehlid_dn8 = assign49130_e82153_d_n8;
        locals.var_ehlid_dn9 = assign49130_e82153_d_n9;
        locals.var_ehlid_dn10 = assign49130_e82153_d_n10;
        locals.var_ehlid_dn11 = assign49130_e82153_d_n11;
        locals.var_ehlid_rv = 0.0;

        let (assign49140_e82163, assign49140_e82163_d_n3, assign49140_e82163_d_n4, assign49140_e82163_d_n5, assign49140_e82163_d_n6, assign49140_e82163_d_n7, assign49140_e82163_d_n8, assign49140_e82163_d_n9, assign49140_e82163_d_n10, assign49140_e82163_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard752 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ehlidfactor, locals.var_ehlidfactor_dn3, locals.var_ehlidfactor_dn4, locals.var_ehlidfactor_dn5, locals.var_ehlidfactor_dn6, locals.var_ehlidfactor_dn7, locals.var_ehlidfactor_dn8, locals.var_ehlidfactor_dn9, locals.var_ehlidfactor_dn10, locals.var_ehlidfactor_dn11,)
    }
};
        locals.var_ehlidfactor = assign49140_e82163;
        locals.var_ehlidfactor_dn3 = assign49140_e82163_d_n3;
        locals.var_ehlidfactor_dn4 = assign49140_e82163_d_n4;
        locals.var_ehlidfactor_dn5 = assign49140_e82163_d_n5;
        locals.var_ehlidfactor_dn6 = assign49140_e82163_d_n6;
        locals.var_ehlidfactor_dn7 = assign49140_e82163_d_n7;
        locals.var_ehlidfactor_dn8 = assign49140_e82163_d_n8;
        locals.var_ehlidfactor_dn9 = assign49140_e82163_d_n9;
        locals.var_ehlidfactor_dn10 = assign49140_e82163_d_n10;
        locals.var_ehlidfactor_dn11 = assign49140_e82163_d_n11;
        locals.var_ehlidfactor_rv = 0.0;

        let (assign49150_e82179, assign49150_e82179_d_n3, assign49150_e82179_d_n4, assign49150_e82179_d_n5, assign49150_e82179_d_n6, assign49150_e82179_d_n7, assign49150_e82179_d_n8, assign49150_e82179_d_n9, assign49150_e82179_d_n10, assign49150_e82179_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard752 == 0.0)) {
        let assign49150_e82175: f64 = (1.0 + locals.var_ehlid);
        let assign49150_e82176: f64 = (assign49150_e82175).sqrt();
        let assign49150_e82177: f64 = (1.0 / assign49150_e82176);
        (assign49150_e82177, (-((locals.var_ehlid_dn3 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn4 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn5 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn6 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn7 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn8 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn9 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn10 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))), (-((locals.var_ehlid_dn11 / (2.0 * assign49150_e82176)) / (assign49150_e82176 * assign49150_e82176))),)
    } else {
        (locals.var_ehlidfactor, locals.var_ehlidfactor_dn3, locals.var_ehlidfactor_dn4, locals.var_ehlidfactor_dn5, locals.var_ehlidfactor_dn6, locals.var_ehlidfactor_dn7, locals.var_ehlidfactor_dn8, locals.var_ehlidfactor_dn9, locals.var_ehlidfactor_dn10, locals.var_ehlidfactor_dn11,)
    }
};
        locals.var_ehlidfactor = assign49150_e82179;
        locals.var_ehlidfactor_dn3 = assign49150_e82179_d_n3;
        locals.var_ehlidfactor_dn4 = assign49150_e82179_d_n4;
        locals.var_ehlidfactor_dn5 = assign49150_e82179_d_n5;
        locals.var_ehlidfactor_dn6 = assign49150_e82179_d_n6;
        locals.var_ehlidfactor_dn7 = assign49150_e82179_d_n7;
        locals.var_ehlidfactor_dn8 = assign49150_e82179_d_n8;
        locals.var_ehlidfactor_dn9 = assign49150_e82179_d_n9;
        locals.var_ehlidfactor_dn10 = assign49150_e82179_d_n10;
        locals.var_ehlidfactor_dn11 = assign49150_e82179_d_n11;
        locals.var_ehlidfactor_rv = 0.0;

        let (assign49160_e82196, assign49160_e82196_d_n3, assign49160_e82196_d_n4, assign49160_e82196_d_n5, assign49160_e82196_d_n6, assign49160_e82196_d_n7, assign49160_e82196_d_n8, assign49160_e82196_d_n9, assign49160_e82196_d_n10, assign49160_e82196_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49160_e82186: f64 = (-0.5);
        let assign49160_e82188: f64 = (assign49160_e82186 * locals.var_leff);
        let assign49160_e82190: f64 = (assign49160_e82188 * locals.var_leff);
        let __rspice_inv_cse_0: f64 = 1.0 / p.p595;
        let assign49160_e82192: f64 = (assign49160_e82190 * __rspice_inv_cse_0);
        let assign49160_e82194: f64 = (assign49160_e82192 * __rspice_inv_cse_0);
        (assign49160_e82194, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49160_e82196;
        locals.var_t0_dn3 = assign49160_e82196_d_n3;
        locals.var_t0_dn4 = assign49160_e82196_d_n4;
        locals.var_t0_dn5 = assign49160_e82196_d_n5;
        locals.var_t0_dn6 = assign49160_e82196_d_n6;
        locals.var_t0_dn7 = assign49160_e82196_d_n7;
        locals.var_t0_dn8 = assign49160_e82196_d_n8;
        locals.var_t0_dn9 = assign49160_e82196_d_n9;
        locals.var_t0_dn10 = assign49160_e82196_d_n10;
        locals.var_t0_dn11 = assign49160_e82196_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49170_e82205, assign49170_e82205_d_n3, assign49170_e82205_d_n4, assign49170_e82205_d_n5, assign49170_e82205_d_n6, assign49170_e82205_d_n7, assign49170_e82205_d_n8, assign49170_e82205_d_n9, assign49170_e82205_d_n10, assign49170_e82205_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49170_e82203: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49170_e82203, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_alphabjt, locals.var_alphabjt_dn3, locals.var_alphabjt_dn4, locals.var_alphabjt_dn5, locals.var_alphabjt_dn6, locals.var_alphabjt_dn7, locals.var_alphabjt_dn8, locals.var_alphabjt_dn9, locals.var_alphabjt_dn10, locals.var_alphabjt_dn11,)
    }
};
        locals.var_alphabjt = assign49170_e82205;
        locals.var_alphabjt_dn3 = assign49170_e82205_d_n3;
        locals.var_alphabjt_dn4 = assign49170_e82205_d_n4;
        locals.var_alphabjt_dn5 = assign49170_e82205_d_n5;
        locals.var_alphabjt_dn6 = assign49170_e82205_d_n6;
        locals.var_alphabjt_dn7 = assign49170_e82205_d_n7;
        locals.var_alphabjt_dn8 = assign49170_e82205_d_n8;
        locals.var_alphabjt_dn9 = assign49170_e82205_d_n9;
        locals.var_alphabjt_dn10 = assign49170_e82205_d_n10;
        locals.var_alphabjt_dn11 = assign49170_e82205_d_n11;
        locals.var_alphabjt_rv = 0.0;

        let (assign49180_e82215, assign49180_e82215_d_n3, assign49180_e82215_d_n4, assign49180_e82215_d_n5, assign49180_e82215_d_n6, assign49180_e82215_d_n7, assign49180_e82215_d_n8, assign49180_e82215_d_n9, assign49180_e82215_d_n10, assign49180_e82215_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49180_e82213: f64 = (1.0 - locals.var_alphabjt);
        (assign49180_e82213, (-locals.var_alphabjt_dn3), (-locals.var_alphabjt_dn4), (-locals.var_alphabjt_dn5), (-locals.var_alphabjt_dn6), (-locals.var_alphabjt_dn7), (-locals.var_alphabjt_dn8), (-locals.var_alphabjt_dn9), (-locals.var_alphabjt_dn10), (-locals.var_alphabjt_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign49180_e82215;
        locals.var_t2_dn3 = assign49180_e82215_d_n3;
        locals.var_t2_dn4 = assign49180_e82215_d_n4;
        locals.var_t2_dn5 = assign49180_e82215_d_n5;
        locals.var_t2_dn6 = assign49180_e82215_d_n6;
        locals.var_t2_dn7 = assign49180_e82215_d_n7;
        locals.var_t2_dn8 = assign49180_e82215_d_n8;
        locals.var_t2_dn9 = assign49180_e82215_d_n9;
        locals.var_t2_dn10 = assign49180_e82215_d_n10;
        locals.var_t2_dn11 = assign49180_e82215_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign49190_e82231, assign49190_e82231_d_n3, assign49190_e82231_d_n4, assign49190_e82231_d_n5, assign49190_e82231_d_n6, assign49190_e82231_d_n7, assign49190_e82231_d_n8, assign49190_e82231_d_n9, assign49190_e82231_d_n10, assign49190_e82231_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49190_e82224: f64 = (1.0 / locals.var_leff);
        let assign49190_e82227: f64 = (1.0 / p.p595);
        let assign49190_e82228: f64 = (assign49190_e82224 + assign49190_e82227);
        let assign49190_e82229: f64 = (locals.var_lbjt0_i * assign49190_e82228);
        (assign49190_e82229, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49190_e82231;
        locals.var_t0_dn3 = assign49190_e82231_d_n3;
        locals.var_t0_dn4 = assign49190_e82231_d_n4;
        locals.var_t0_dn5 = assign49190_e82231_d_n5;
        locals.var_t0_dn6 = assign49190_e82231_d_n6;
        locals.var_t0_dn7 = assign49190_e82231_d_n7;
        locals.var_t0_dn8 = assign49190_e82231_d_n8;
        locals.var_t0_dn9 = assign49190_e82231_d_n9;
        locals.var_t0_dn10 = assign49190_e82231_d_n10;
        locals.var_t0_dn11 = assign49190_e82231_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49200_e82241, assign49200_e82241_d_n3, assign49200_e82241_d_n4, assign49200_e82241_d_n5, assign49200_e82241_d_n6, assign49200_e82241_d_n7, assign49200_e82241_d_n8, assign49200_e82241_d_n9, assign49200_e82241_d_n10, assign49200_e82241_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49200_e82239: f64 = (locals.var_t0).powf(locals.var_nbjt_i);
        (assign49200_e82239, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn3)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn3 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn4)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn4 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn5)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn5 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn6)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn6 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn7)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn7 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn8)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn8 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn9)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn9 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn10)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn10 / locals.var_t0))) }, if 0.0 == 0.0 && ((locals.var_nbjt_i) as f64).is_finite() && ((locals.var_nbjt_i) as f64).fract() == 0.0 { if locals.var_nbjt_i == 0.0 { 0.0 } else { (locals.var_nbjt_i * ((locals.var_t0).powf(locals.var_nbjt_i - 1.0) * locals.var_t0_dn11)) } } else { (assign49200_e82239 * (locals.var_nbjt_i * (locals.var_t0_dn11 / locals.var_t0))) },)
    } else {
        (locals.var_lratio, locals.var_lratio_dn3, locals.var_lratio_dn4, locals.var_lratio_dn5, locals.var_lratio_dn6, locals.var_lratio_dn7, locals.var_lratio_dn8, locals.var_lratio_dn9, locals.var_lratio_dn10, locals.var_lratio_dn11,)
    }
};
        locals.var_lratio = assign49200_e82241;
        locals.var_lratio_dn3 = assign49200_e82241_d_n3;
        locals.var_lratio_dn4 = assign49200_e82241_d_n4;
        locals.var_lratio_dn5 = assign49200_e82241_d_n5;
        locals.var_lratio_dn6 = assign49200_e82241_d_n6;
        locals.var_lratio_dn7 = assign49200_e82241_d_n7;
        locals.var_lratio_dn8 = assign49200_e82241_d_n8;
        locals.var_lratio_dn9 = assign49200_e82241_d_n9;
        locals.var_lratio_dn10 = assign49200_e82241_d_n10;
        locals.var_lratio_dn11 = assign49200_e82241_d_n11;
        locals.var_lratio_rv = 0.0;

        let (assign49210_e82253, assign49210_e82253_d_n3, assign49210_e82253_d_n4, assign49210_e82253_d_n5, assign49210_e82253_d_n6, assign49210_e82253_d_n7, assign49210_e82253_d_n8, assign49210_e82253_d_n9, assign49210_e82253_d_n10, assign49210_e82253_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49210_e82249: f64 = (locals.var_wtsi * locals.var_jbjts);
        let assign49210_e82251: f64 = (assign49210_e82249 * locals.var_lratio);
        (assign49210_e82251, (((locals.var_wtsi * locals.var_jbjts_dn3) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn3)), (((locals.var_wtsi * locals.var_jbjts_dn4) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn4)), (((locals.var_wtsi * locals.var_jbjts_dn5) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn5)), (((locals.var_wtsi * locals.var_jbjts_dn6) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn6)), (((locals.var_wtsi * locals.var_jbjts_dn7) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn7)), (((locals.var_wtsi * locals.var_jbjts_dn8) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn8)), (((locals.var_wtsi * locals.var_jbjts_dn9) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn9)), (((locals.var_wtsi * locals.var_jbjts_dn10) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn10)), (((locals.var_wtsi * locals.var_jbjts_dn11) * locals.var_lratio) + (assign49210_e82249 * locals.var_lratio_dn11)),)
    } else {
        (locals.var_ien, locals.var_ien_dn3, locals.var_ien_dn4, locals.var_ien_dn5, locals.var_ien_dn6, locals.var_ien_dn7, locals.var_ien_dn8, locals.var_ien_dn9, locals.var_ien_dn10, locals.var_ien_dn11,)
    }
};
        locals.var_ien = assign49210_e82253;
        locals.var_ien_dn3 = assign49210_e82253_d_n3;
        locals.var_ien_dn4 = assign49210_e82253_d_n4;
        locals.var_ien_dn5 = assign49210_e82253_d_n5;
        locals.var_ien_dn6 = assign49210_e82253_d_n6;
        locals.var_ien_dn7 = assign49210_e82253_d_n7;
        locals.var_ien_dn8 = assign49210_e82253_d_n8;
        locals.var_ien_dn9 = assign49210_e82253_d_n9;
        locals.var_ien_dn10 = assign49210_e82253_d_n10;
        locals.var_ien_dn11 = assign49210_e82253_d_n11;
        locals.var_ien_rv = 0.0;

        let (assign49220_e82263, assign49220_e82263_d_n3, assign49220_e82263_d_n4, assign49220_e82263_d_n5, assign49220_e82263_d_n6, assign49220_e82263_d_n7, assign49220_e82263_d_n8, assign49220_e82263_d_n9, assign49220_e82263_d_n10, assign49220_e82263_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49220_e82261: f64 = (locals.var_t0 * locals.var_ien);
        (assign49220_e82261, ((locals.var_t0_dn3 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn3)), ((locals.var_t0_dn4 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn4)), ((locals.var_t0_dn5 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn5)), ((locals.var_t0_dn6 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn6)), ((locals.var_t0_dn7 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn7)), ((locals.var_t0_dn8 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn8)), ((locals.var_t0_dn9 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn9)), ((locals.var_t0_dn10 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn10)), ((locals.var_t0_dn11 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49220_e82263;
        locals.var_t1_dn3 = assign49220_e82263_d_n3;
        locals.var_t1_dn4 = assign49220_e82263_d_n4;
        locals.var_t1_dn5 = assign49220_e82263_d_n5;
        locals.var_t1_dn6 = assign49220_e82263_d_n6;
        locals.var_t1_dn7 = assign49220_e82263_d_n7;
        locals.var_t1_dn8 = assign49220_e82263_d_n8;
        locals.var_t1_dn9 = assign49220_e82263_d_n9;
        locals.var_t1_dn10 = assign49220_e82263_d_n10;
        locals.var_t1_dn11 = assign49220_e82263_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49240_e82289, assign49240_e82289_d_n3, assign49240_e82289_d_n4, assign49240_e82289_d_n5, assign49240_e82289_d_n6, assign49240_e82289_d_n7, assign49240_e82289_d_n8, assign49240_e82289_d_n9, assign49240_e82289_d_n10, assign49240_e82289_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49240_e82285: f64 = (locals.var_wtsi * locals.var_jbjtd);
        let assign49240_e82287: f64 = (assign49240_e82285 * locals.var_lratio);
        (assign49240_e82287, (((locals.var_wtsi * locals.var_jbjtd_dn3) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn3)), (((locals.var_wtsi * locals.var_jbjtd_dn4) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn4)), (((locals.var_wtsi * locals.var_jbjtd_dn5) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn5)), (((locals.var_wtsi * locals.var_jbjtd_dn6) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn6)), (((locals.var_wtsi * locals.var_jbjtd_dn7) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn7)), (((locals.var_wtsi * locals.var_jbjtd_dn8) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn8)), (((locals.var_wtsi * locals.var_jbjtd_dn9) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn9)), (((locals.var_wtsi * locals.var_jbjtd_dn10) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn10)), (((locals.var_wtsi * locals.var_jbjtd_dn11) * locals.var_lratio) + (assign49240_e82285 * locals.var_lratio_dn11)),)
    } else {
        (locals.var_ien, locals.var_ien_dn3, locals.var_ien_dn4, locals.var_ien_dn5, locals.var_ien_dn6, locals.var_ien_dn7, locals.var_ien_dn8, locals.var_ien_dn9, locals.var_ien_dn10, locals.var_ien_dn11,)
    }
};
        locals.var_ien = assign49240_e82289;
        locals.var_ien_dn3 = assign49240_e82289_d_n3;
        locals.var_ien_dn4 = assign49240_e82289_d_n4;
        locals.var_ien_dn5 = assign49240_e82289_d_n5;
        locals.var_ien_dn6 = assign49240_e82289_d_n6;
        locals.var_ien_dn7 = assign49240_e82289_d_n7;
        locals.var_ien_dn8 = assign49240_e82289_d_n8;
        locals.var_ien_dn9 = assign49240_e82289_d_n9;
        locals.var_ien_dn10 = assign49240_e82289_d_n10;
        locals.var_ien_dn11 = assign49240_e82289_d_n11;
        locals.var_ien_rv = 0.0;

        let (assign49250_e82299, assign49250_e82299_d_n3, assign49250_e82299_d_n4, assign49250_e82299_d_n5, assign49250_e82299_d_n6, assign49250_e82299_d_n7, assign49250_e82299_d_n8, assign49250_e82299_d_n9, assign49250_e82299_d_n10, assign49250_e82299_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49250_e82297: f64 = (locals.var_t0 * locals.var_ien);
        (assign49250_e82297, ((locals.var_t0_dn3 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn3)), ((locals.var_t0_dn4 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn4)), ((locals.var_t0_dn5 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn5)), ((locals.var_t0_dn6 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn6)), ((locals.var_t0_dn7 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn7)), ((locals.var_t0_dn8 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn8)), ((locals.var_t0_dn9 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn9)), ((locals.var_t0_dn10 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn10)), ((locals.var_t0_dn11 * locals.var_ien) + (locals.var_t0 * locals.var_ien_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49250_e82299;
        locals.var_t1_dn3 = assign49250_e82299_d_n3;
        locals.var_t1_dn4 = assign49250_e82299_d_n4;
        locals.var_t1_dn5 = assign49250_e82299_d_n5;
        locals.var_t1_dn6 = assign49250_e82299_d_n6;
        locals.var_t1_dn7 = assign49250_e82299_d_n7;
        locals.var_t1_dn8 = assign49250_e82299_d_n8;
        locals.var_t1_dn9 = assign49250_e82299_d_n9;
        locals.var_t1_dn10 = assign49250_e82299_d_n10;
        locals.var_t1_dn11 = assign49250_e82299_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49270_e82327, assign49270_e82327_d_n3, assign49270_e82327_d_n4, assign49270_e82327_d_n5, assign49270_e82327_d_n6, assign49270_e82327_d_n7, assign49270_e82327_d_n8, assign49270_e82327_d_n9, assign49270_e82327_d_n10, assign49270_e82327_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49270_e82323: f64 = (locals.var_t0).powf(locals.var_ndif_i);
        let assign49270_e82324: f64 = (p.p920 * assign49270_e82323);
        let assign49270_e82325: f64 = (1.0 + assign49270_e82324);
        (assign49270_e82325, (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn3)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn3 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn4)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn4 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn5)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn5 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn6)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn6 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn7)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn7 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn8)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn8 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn9)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn9 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn10)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn10 / locals.var_t0))) }), (p.p920 * if 0.0 == 0.0 && ((locals.var_ndif_i) as f64).is_finite() && ((locals.var_ndif_i) as f64).fract() == 0.0 { if locals.var_ndif_i == 0.0 { 0.0 } else { (locals.var_ndif_i * ((locals.var_t0).powf(locals.var_ndif_i - 1.0) * locals.var_t0_dn11)) } } else { (assign49270_e82323 * (locals.var_ndif_i * (locals.var_t0_dn11 / locals.var_t0))) }),)
    } else {
        (locals.var_lratiodif, locals.var_lratiodif_dn3, locals.var_lratiodif_dn4, locals.var_lratiodif_dn5, locals.var_lratiodif_dn6, locals.var_lratiodif_dn7, locals.var_lratiodif_dn8, locals.var_lratiodif_dn9, locals.var_lratiodif_dn10, locals.var_lratiodif_dn11,)
    }
};
        locals.var_lratiodif = assign49270_e82327;
        locals.var_lratiodif_dn3 = assign49270_e82327_d_n3;
        locals.var_lratiodif_dn4 = assign49270_e82327_d_n4;
        locals.var_lratiodif_dn5 = assign49270_e82327_d_n5;
        locals.var_lratiodif_dn6 = assign49270_e82327_d_n6;
        locals.var_lratiodif_dn7 = assign49270_e82327_d_n7;
        locals.var_lratiodif_dn8 = assign49270_e82327_d_n8;
        locals.var_lratiodif_dn9 = assign49270_e82327_d_n9;
        locals.var_lratiodif_dn10 = assign49270_e82327_d_n10;
        locals.var_lratiodif_dn11 = assign49270_e82327_d_n11;
        locals.var_lratiodif_rv = 0.0;

        let (assign49280_e82339, assign49280_e82339_d_n3, assign49280_e82339_d_n4, assign49280_e82339_d_n5, assign49280_e82339_d_n6, assign49280_e82339_d_n7, assign49280_e82339_d_n8, assign49280_e82339_d_n9, assign49280_e82339_d_n10, assign49280_e82339_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49280_e82335: f64 = (locals.var_wtsi * locals.var_jbjts);
        let assign49280_e82337: f64 = (assign49280_e82335 * locals.var_lratiodif);
        (assign49280_e82337, (((locals.var_wtsi * locals.var_jbjts_dn3) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn3)), (((locals.var_wtsi * locals.var_jbjts_dn4) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn4)), (((locals.var_wtsi * locals.var_jbjts_dn5) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn5)), (((locals.var_wtsi * locals.var_jbjts_dn6) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn6)), (((locals.var_wtsi * locals.var_jbjts_dn7) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn7)), (((locals.var_wtsi * locals.var_jbjts_dn8) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn8)), (((locals.var_wtsi * locals.var_jbjts_dn9) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn9)), (((locals.var_wtsi * locals.var_jbjts_dn10) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn10)), (((locals.var_wtsi * locals.var_jbjts_dn11) * locals.var_lratiodif) + (assign49280_e82335 * locals.var_lratiodif_dn11)),)
    } else {
        (locals.var_iendif, locals.var_iendif_dn3, locals.var_iendif_dn4, locals.var_iendif_dn5, locals.var_iendif_dn6, locals.var_iendif_dn7, locals.var_iendif_dn8, locals.var_iendif_dn9, locals.var_iendif_dn10, locals.var_iendif_dn11,)
    }
};
        locals.var_iendif = assign49280_e82339;
        locals.var_iendif_dn3 = assign49280_e82339_d_n3;
        locals.var_iendif_dn4 = assign49280_e82339_d_n4;
        locals.var_iendif_dn5 = assign49280_e82339_d_n5;
        locals.var_iendif_dn6 = assign49280_e82339_d_n6;
        locals.var_iendif_dn7 = assign49280_e82339_d_n7;
        locals.var_iendif_dn8 = assign49280_e82339_d_n8;
        locals.var_iendif_dn9 = assign49280_e82339_d_n9;
        locals.var_iendif_dn10 = assign49280_e82339_d_n10;
        locals.var_iendif_dn11 = assign49280_e82339_d_n11;
        locals.var_iendif_rv = 0.0;

        let (assign49290_e82353, assign49290_e82353_d_n3, assign49290_e82353_d_n4, assign49290_e82353_d_n5, assign49290_e82353_d_n6, assign49290_e82353_d_n7, assign49290_e82353_d_n8, assign49290_e82353_d_n9, assign49290_e82353_d_n10, assign49290_e82353_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49290_e82348: f64 = (locals.var_expvbsnvtm - 1.0);
        let assign49290_e82349: f64 = (locals.var_iendif * assign49290_e82348);
        let assign49290_e82351: f64 = (assign49290_e82349 * locals.var_ehlisfactor);
        (assign49290_e82351, ((((locals.var_iendif_dn3 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn3)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn3)), ((((locals.var_iendif_dn4 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn4)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn4)), ((((locals.var_iendif_dn5 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn5)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn5)), ((((locals.var_iendif_dn6 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn6)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn6)), ((((locals.var_iendif_dn7 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn7)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn7)), ((((locals.var_iendif_dn8 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn8)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn8)), ((((locals.var_iendif_dn9 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn9)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn9)), ((((locals.var_iendif_dn10 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn10)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn10)), ((((locals.var_iendif_dn11 * assign49290_e82348) + (locals.var_iendif * locals.var_expvbsnvtm_dn11)) * locals.var_ehlisfactor) + (assign49290_e82349 * locals.var_ehlisfactor_dn11)),)
    } else {
        (locals.var_ibsdif, locals.var_ibsdif_dn3, locals.var_ibsdif_dn4, locals.var_ibsdif_dn5, locals.var_ibsdif_dn6, locals.var_ibsdif_dn7, locals.var_ibsdif_dn8, locals.var_ibsdif_dn9, locals.var_ibsdif_dn10, locals.var_ibsdif_dn11,)
    }
};
        locals.var_ibsdif = assign49290_e82353;
        locals.var_ibsdif_dn3 = assign49290_e82353_d_n3;
        locals.var_ibsdif_dn4 = assign49290_e82353_d_n4;
        locals.var_ibsdif_dn5 = assign49290_e82353_d_n5;
        locals.var_ibsdif_dn6 = assign49290_e82353_d_n6;
        locals.var_ibsdif_dn7 = assign49290_e82353_d_n7;
        locals.var_ibsdif_dn8 = assign49290_e82353_d_n8;
        locals.var_ibsdif_dn9 = assign49290_e82353_d_n9;
        locals.var_ibsdif_dn10 = assign49290_e82353_d_n10;
        locals.var_ibsdif_dn11 = assign49290_e82353_d_n11;
        locals.var_ibsdif_rv = 0.0;

        let (assign49300_e82365, assign49300_e82365_d_n3, assign49300_e82365_d_n4, assign49300_e82365_d_n5, assign49300_e82365_d_n6, assign49300_e82365_d_n7, assign49300_e82365_d_n8, assign49300_e82365_d_n9, assign49300_e82365_d_n10, assign49300_e82365_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49300_e82361: f64 = (locals.var_wtsi * locals.var_jbjtd);
        let assign49300_e82363: f64 = (assign49300_e82361 * locals.var_lratiodif);
        (assign49300_e82363, (((locals.var_wtsi * locals.var_jbjtd_dn3) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn3)), (((locals.var_wtsi * locals.var_jbjtd_dn4) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn4)), (((locals.var_wtsi * locals.var_jbjtd_dn5) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn5)), (((locals.var_wtsi * locals.var_jbjtd_dn6) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn6)), (((locals.var_wtsi * locals.var_jbjtd_dn7) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn7)), (((locals.var_wtsi * locals.var_jbjtd_dn8) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn8)), (((locals.var_wtsi * locals.var_jbjtd_dn9) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn9)), (((locals.var_wtsi * locals.var_jbjtd_dn10) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn10)), (((locals.var_wtsi * locals.var_jbjtd_dn11) * locals.var_lratiodif) + (assign49300_e82361 * locals.var_lratiodif_dn11)),)
    } else {
        (locals.var_iendif, locals.var_iendif_dn3, locals.var_iendif_dn4, locals.var_iendif_dn5, locals.var_iendif_dn6, locals.var_iendif_dn7, locals.var_iendif_dn8, locals.var_iendif_dn9, locals.var_iendif_dn10, locals.var_iendif_dn11,)
    }
};
        locals.var_iendif = assign49300_e82365;
        locals.var_iendif_dn3 = assign49300_e82365_d_n3;
        locals.var_iendif_dn4 = assign49300_e82365_d_n4;
        locals.var_iendif_dn5 = assign49300_e82365_d_n5;
        locals.var_iendif_dn6 = assign49300_e82365_d_n6;
        locals.var_iendif_dn7 = assign49300_e82365_d_n7;
        locals.var_iendif_dn8 = assign49300_e82365_d_n8;
        locals.var_iendif_dn9 = assign49300_e82365_d_n9;
        locals.var_iendif_dn10 = assign49300_e82365_d_n10;
        locals.var_iendif_dn11 = assign49300_e82365_d_n11;
        locals.var_iendif_rv = 0.0;

        let (assign49310_e82379, assign49310_e82379_d_n3, assign49310_e82379_d_n4, assign49310_e82379_d_n5, assign49310_e82379_d_n6, assign49310_e82379_d_n7, assign49310_e82379_d_n8, assign49310_e82379_d_n9, assign49310_e82379_d_n10, assign49310_e82379_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49310_e82374: f64 = (locals.var_expvbdnvtm - 1.0);
        let assign49310_e82375: f64 = (locals.var_iendif * assign49310_e82374);
        let assign49310_e82377: f64 = (assign49310_e82375 * locals.var_ehlidfactor);
        (assign49310_e82377, ((((locals.var_iendif_dn3 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn3)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn3)), ((((locals.var_iendif_dn4 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn4)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn4)), ((((locals.var_iendif_dn5 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn5)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn5)), ((((locals.var_iendif_dn6 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn6)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn6)), ((((locals.var_iendif_dn7 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn7)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn7)), ((((locals.var_iendif_dn8 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn8)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn8)), ((((locals.var_iendif_dn9 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn9)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn9)), ((((locals.var_iendif_dn10 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn10)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn10)), ((((locals.var_iendif_dn11 * assign49310_e82374) + (locals.var_iendif * locals.var_expvbdnvtm_dn11)) * locals.var_ehlidfactor) + (assign49310_e82375 * locals.var_ehlidfactor_dn11)),)
    } else {
        (locals.var_ibddif, locals.var_ibddif_dn3, locals.var_ibddif_dn4, locals.var_ibddif_dn5, locals.var_ibddif_dn6, locals.var_ibddif_dn7, locals.var_ibddif_dn8, locals.var_ibddif_dn9, locals.var_ibddif_dn10, locals.var_ibddif_dn11,)
    }
};
        locals.var_ibddif = assign49310_e82379;
        locals.var_ibddif_dn3 = assign49310_e82379_d_n3;
        locals.var_ibddif_dn4 = assign49310_e82379_d_n4;
        locals.var_ibddif_dn5 = assign49310_e82379_d_n5;
        locals.var_ibddif_dn6 = assign49310_e82379_d_n6;
        locals.var_ibddif_dn7 = assign49310_e82379_d_n7;
        locals.var_ibddif_dn8 = assign49310_e82379_d_n8;
        locals.var_ibddif_dn9 = assign49310_e82379_d_n9;
        locals.var_ibddif_dn10 = assign49310_e82379_d_n10;
        locals.var_ibddif_dn11 = assign49310_e82379_d_n11;
        locals.var_ibddif_rv = 0.0;

        let (assign49320_e82391,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) {
        let assign49320_e82388: f64 = (locals.var_aely_i * locals.var_leff);
        let assign49320_e82389: f64 = (locals.var_vabjt_i + assign49320_e82388);
        (assign49320_e82389,)
    } else {
        (locals.var_vearly,)
    }
};
        locals.var_vearly = assign49320_e82391;
        locals.var_vearly_rv = 0.0;

        let assign49330_e82394: f64 = if locals.var_vearly < 1.0 { 1.0 } else { 0.0 };
        locals.var_guard753 = assign49330_e82394;
        locals.var_guard753_rv = 0.0;

        let (assign49340_e82404,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard753 != 0.0)) {
        (1.0,)
    } else {
        (locals.var_vearly,)
    }
};
        locals.var_vearly = assign49340_e82404;
        locals.var_vearly_rv = 0.0;

        let assign49350_e82407: f64 = if p.p554 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard754 = assign49350_e82407;
        locals.var_guard754_rv = 0.0;

        let (assign49370_e82434, assign49370_e82434_d_n3, assign49370_e82434_d_n4, assign49370_e82434_d_n5, assign49370_e82434_d_n6, assign49370_e82434_d_n7, assign49370_e82434_d_n8, assign49370_e82434_d_n9, assign49370_e82434_d_n10, assign49370_e82434_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard754 == 0.0)) {
        let assign49370_e82429: f64 = (locals.var_vbs_jct + locals.var_vbd_jct);
        let assign49370_e82431: f64 = (assign49370_e82429 / locals.var_vearly);
        let assign49370_e82432: f64 = (1.0 + assign49370_e82431);
        (assign49370_e82432, 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn6 / locals.var_vearly), (locals.var_vbs_jct_dn7 / locals.var_vearly), 0.0, 0.0, ((locals.var_vbs_jct_dn10 + locals.var_vbd_jct_dn10) / locals.var_vearly), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49370_e82434;
        locals.var_t0_dn3 = assign49370_e82434_d_n3;
        locals.var_t0_dn4 = assign49370_e82434_d_n4;
        locals.var_t0_dn5 = assign49370_e82434_d_n5;
        locals.var_t0_dn6 = assign49370_e82434_d_n6;
        locals.var_t0_dn7 = assign49370_e82434_d_n7;
        locals.var_t0_dn8 = assign49370_e82434_d_n8;
        locals.var_t0_dn9 = assign49370_e82434_d_n9;
        locals.var_t0_dn10 = assign49370_e82434_d_n10;
        locals.var_t0_dn11 = assign49370_e82434_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49380_e82447, assign49380_e82447_d_n3, assign49380_e82447_d_n4, assign49380_e82447_d_n5, assign49380_e82447_d_n6, assign49380_e82447_d_n7, assign49380_e82447_d_n8, assign49380_e82447_d_n9, assign49380_e82447_d_n10, assign49380_e82447_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard754 == 0.0)) {
        let assign49380_e82445: f64 = (locals.var_ehlis + locals.var_ehlid);
        (assign49380_e82445, (locals.var_ehlis_dn3 + locals.var_ehlid_dn3), (locals.var_ehlis_dn4 + locals.var_ehlid_dn4), (locals.var_ehlis_dn5 + locals.var_ehlid_dn5), (locals.var_ehlis_dn6 + locals.var_ehlid_dn6), (locals.var_ehlis_dn7 + locals.var_ehlid_dn7), (locals.var_ehlis_dn8 + locals.var_ehlid_dn8), (locals.var_ehlis_dn9 + locals.var_ehlid_dn9), (locals.var_ehlis_dn10 + locals.var_ehlid_dn10), (locals.var_ehlis_dn11 + locals.var_ehlid_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49380_e82447;
        locals.var_t1_dn3 = assign49380_e82447_d_n3;
        locals.var_t1_dn4 = assign49380_e82447_d_n4;
        locals.var_t1_dn5 = assign49380_e82447_d_n5;
        locals.var_t1_dn6 = assign49380_e82447_d_n6;
        locals.var_t1_dn7 = assign49380_e82447_d_n7;
        locals.var_t1_dn8 = assign49380_e82447_d_n8;
        locals.var_t1_dn9 = assign49380_e82447_d_n9;
        locals.var_t1_dn10 = assign49380_e82447_d_n10;
        locals.var_t1_dn11 = assign49380_e82447_d_n11;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_172(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign49390_e82465, assign49390_e82465_d_n3, assign49390_e82465_d_n4, assign49390_e82465_d_n5, assign49390_e82465_d_n6, assign49390_e82465_d_n7, assign49390_e82465_d_n8, assign49390_e82465_d_n9, assign49390_e82465_d_n10, assign49390_e82465_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard754 == 0.0)) {
        let assign49390_e82458: f64 = (locals.var_t0 * locals.var_t0);
        let assign49390_e82461: f64 = (4.0 * locals.var_t1);
        let assign49390_e82462: f64 = (assign49390_e82458 + assign49390_e82461);
        let assign49390_e82463: f64 = (assign49390_e82462).sqrt();
        (assign49390_e82463, ((((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) + (4.0 * locals.var_t1_dn3)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) + (4.0 * locals.var_t1_dn4)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) + (4.0 * locals.var_t1_dn5)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) + (4.0 * locals.var_t1_dn6)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) + (4.0 * locals.var_t1_dn7)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) + (4.0 * locals.var_t1_dn8)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) + (4.0 * locals.var_t1_dn9)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) + (4.0 * locals.var_t1_dn10)) / (2.0 * assign49390_e82463)), ((((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) + (4.0 * locals.var_t1_dn11)) / (2.0 * assign49390_e82463)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign49390_e82465;
        locals.var_t3_dn3 = assign49390_e82465_d_n3;
        locals.var_t3_dn4 = assign49390_e82465_d_n4;
        locals.var_t3_dn5 = assign49390_e82465_d_n5;
        locals.var_t3_dn6 = assign49390_e82465_d_n6;
        locals.var_t3_dn7 = assign49390_e82465_d_n7;
        locals.var_t3_dn8 = assign49390_e82465_d_n8;
        locals.var_t3_dn9 = assign49390_e82465_d_n9;
        locals.var_t3_dn10 = assign49390_e82465_d_n10;
        locals.var_t3_dn11 = assign49390_e82465_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign49400_e82480, assign49400_e82480_d_n3, assign49400_e82480_d_n4, assign49400_e82480_d_n5, assign49400_e82480_d_n6, assign49400_e82480_d_n7, assign49400_e82480_d_n8, assign49400_e82480_d_n9, assign49400_e82480_d_n10, assign49400_e82480_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard754 == 0.0)) {
        let assign49400_e82476: f64 = (locals.var_t0 + locals.var_t3);
        let assign49400_e82478: f64 = (assign49400_e82476 / 2.0);
        (assign49400_e82478, ((locals.var_t0_dn3 + locals.var_t3_dn3) / 2.0), ((locals.var_t0_dn4 + locals.var_t3_dn4) / 2.0), ((locals.var_t0_dn5 + locals.var_t3_dn5) / 2.0), ((locals.var_t0_dn6 + locals.var_t3_dn6) / 2.0), ((locals.var_t0_dn7 + locals.var_t3_dn7) / 2.0), ((locals.var_t0_dn8 + locals.var_t3_dn8) / 2.0), ((locals.var_t0_dn9 + locals.var_t3_dn9) / 2.0), ((locals.var_t0_dn10 + locals.var_t3_dn10) / 2.0), ((locals.var_t0_dn11 + locals.var_t3_dn11) / 2.0),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign49400_e82480;
        locals.var_t2_dn3 = assign49400_e82480_d_n3;
        locals.var_t2_dn4 = assign49400_e82480_d_n4;
        locals.var_t2_dn5 = assign49400_e82480_d_n5;
        locals.var_t2_dn6 = assign49400_e82480_d_n6;
        locals.var_t2_dn7 = assign49400_e82480_d_n7;
        locals.var_t2_dn8 = assign49400_e82480_d_n8;
        locals.var_t2_dn9 = assign49400_e82480_d_n9;
        locals.var_t2_dn10 = assign49400_e82480_d_n10;
        locals.var_t2_dn11 = assign49400_e82480_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign49440_e82525, assign49440_e82525_d_n3, assign49440_e82525_d_n4, assign49440_e82525_d_n5, assign49440_e82525_d_n6, assign49440_e82525_d_n7, assign49440_e82525_d_n8, assign49440_e82525_d_n9, assign49440_e82525_d_n10, assign49440_e82525_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard750 == 0.0)) && (locals.var_guard754 == 0.0)) {
        let assign49440_e82523: f64 = (locals.var_alphabjt * locals.var_ien);
        (assign49440_e82523, ((locals.var_alphabjt_dn3 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn3)), ((locals.var_alphabjt_dn4 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn4)), ((locals.var_alphabjt_dn5 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn5)), ((locals.var_alphabjt_dn6 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn6)), ((locals.var_alphabjt_dn7 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn7)), ((locals.var_alphabjt_dn8 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn8)), ((locals.var_alphabjt_dn9 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn9)), ((locals.var_alphabjt_dn10 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn10)), ((locals.var_alphabjt_dn11 * locals.var_ien) + (locals.var_alphabjt * locals.var_ien_dn11)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49440_e82525;
        locals.var_t0_dn3 = assign49440_e82525_d_n3;
        locals.var_t0_dn4 = assign49440_e82525_d_n4;
        locals.var_t0_dn5 = assign49440_e82525_d_n5;
        locals.var_t0_dn6 = assign49440_e82525_d_n6;
        locals.var_t0_dn7 = assign49440_e82525_d_n7;
        locals.var_t0_dn8 = assign49440_e82525_d_n8;
        locals.var_t0_dn9 = assign49440_e82525_d_n9;
        locals.var_t0_dn10 = assign49440_e82525_d_n10;
        locals.var_t0_dn11 = assign49440_e82525_d_n11;
        locals.var_t0_rv = 0.0;

        let assign49460_e82551: f64 = if ((locals.var_istun_i == 0.0) && (locals.var_idtun_i == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard756 = assign49460_e82551;
        locals.var_guard756_rv = 0.0;

        let (assign49490_e82577, assign49490_e82577_d_n3, assign49490_e82577_d_n4, assign49490_e82577_d_n5, assign49490_e82577_d_n6, assign49490_e82577_d_n7, assign49490_e82577_d_n8, assign49490_e82577_d_n9, assign49490_e82577_d_n10, assign49490_e82577_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49490_e82574: f64 = (locals.var_tratio - 1.0);
        let assign49490_e82575: f64 = (locals.var_xtun_i * assign49490_e82574);
        (assign49490_e82575, 0.0, (locals.var_xtun_i * locals.var_tratio_dn4), (locals.var_xtun_i * locals.var_tratio_dn5), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign49490_e82577;
        locals.var_t7_dn3 = assign49490_e82577_d_n3;
        locals.var_t7_dn4 = assign49490_e82577_d_n4;
        locals.var_t7_dn5 = assign49490_e82577_d_n5;
        locals.var_t7_dn6 = assign49490_e82577_d_n6;
        locals.var_t7_dn7 = assign49490_e82577_d_n7;
        locals.var_t7_dn8 = assign49490_e82577_d_n8;
        locals.var_t7_dn9 = assign49490_e82577_d_n9;
        locals.var_t7_dn10 = assign49490_e82577_d_n10;
        locals.var_t7_dn11 = assign49490_e82577_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign49500_e82586, assign49500_e82586_d_n3, assign49500_e82586_d_n4, assign49500_e82586_d_n5, assign49500_e82586_d_n6, assign49500_e82586_d_n7, assign49500_e82586_d_n8, assign49500_e82586_d_n9, assign49500_e82586_d_n10, assign49500_e82586_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49500_e82584: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49500_e82584, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49500_e82586;
        locals.var_t0_dn3 = assign49500_e82586_d_n3;
        locals.var_t0_dn4 = assign49500_e82586_d_n4;
        locals.var_t0_dn5 = assign49500_e82586_d_n5;
        locals.var_t0_dn6 = assign49500_e82586_d_n6;
        locals.var_t0_dn7 = assign49500_e82586_d_n7;
        locals.var_t0_dn8 = assign49500_e82586_d_n8;
        locals.var_t0_dn9 = assign49500_e82586_d_n9;
        locals.var_t0_dn10 = assign49500_e82586_d_n10;
        locals.var_t0_dn11 = assign49500_e82586_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49510_e82596, assign49510_e82596_d_n3, assign49510_e82596_d_n4, assign49510_e82596_d_n5, assign49510_e82596_d_n6, assign49510_e82596_d_n7, assign49510_e82596_d_n8, assign49510_e82596_d_n9, assign49510_e82596_d_n10, assign49510_e82596_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49510_e82594: f64 = (locals.var_istun_i * locals.var_t0);
        (assign49510_e82594, (locals.var_istun_i * locals.var_t0_dn3), (locals.var_istun_i * locals.var_t0_dn4), (locals.var_istun_i * locals.var_t0_dn5), (locals.var_istun_i * locals.var_t0_dn6), (locals.var_istun_i * locals.var_t0_dn7), (locals.var_istun_i * locals.var_t0_dn8), (locals.var_istun_i * locals.var_t0_dn9), (locals.var_istun_i * locals.var_t0_dn10), (locals.var_istun_i * locals.var_t0_dn11),)
    } else {
        (locals.var_jtuns, locals.var_jtuns_dn3, locals.var_jtuns_dn4, locals.var_jtuns_dn5, locals.var_jtuns_dn6, locals.var_jtuns_dn7, locals.var_jtuns_dn8, locals.var_jtuns_dn9, locals.var_jtuns_dn10, locals.var_jtuns_dn11,)
    }
};
        locals.var_jtuns = assign49510_e82596;
        locals.var_jtuns_dn3 = assign49510_e82596_d_n3;
        locals.var_jtuns_dn4 = assign49510_e82596_d_n4;
        locals.var_jtuns_dn5 = assign49510_e82596_d_n5;
        locals.var_jtuns_dn6 = assign49510_e82596_d_n6;
        locals.var_jtuns_dn7 = assign49510_e82596_d_n7;
        locals.var_jtuns_dn8 = assign49510_e82596_d_n8;
        locals.var_jtuns_dn9 = assign49510_e82596_d_n9;
        locals.var_jtuns_dn10 = assign49510_e82596_d_n10;
        locals.var_jtuns_dn11 = assign49510_e82596_d_n11;
        locals.var_jtuns_rv = 0.0;

        let (assign49520_e82608, assign49520_e82608_d_n3, assign49520_e82608_d_n4, assign49520_e82608_d_n5, assign49520_e82608_d_n6, assign49520_e82608_d_n7, assign49520_e82608_d_n8, assign49520_e82608_d_n9, assign49520_e82608_d_n10, assign49520_e82608_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49520_e82605: f64 = (locals.var_tratio - 1.0);
        let assign49520_e82606: f64 = (locals.var_xtund_i * assign49520_e82605);
        (assign49520_e82606, 0.0, (locals.var_xtund_i * locals.var_tratio_dn4), (locals.var_xtund_i * locals.var_tratio_dn5), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11,)
    }
};
        locals.var_t7 = assign49520_e82608;
        locals.var_t7_dn3 = assign49520_e82608_d_n3;
        locals.var_t7_dn4 = assign49520_e82608_d_n4;
        locals.var_t7_dn5 = assign49520_e82608_d_n5;
        locals.var_t7_dn6 = assign49520_e82608_d_n6;
        locals.var_t7_dn7 = assign49520_e82608_d_n7;
        locals.var_t7_dn8 = assign49520_e82608_d_n8;
        locals.var_t7_dn9 = assign49520_e82608_d_n9;
        locals.var_t7_dn10 = assign49520_e82608_d_n10;
        locals.var_t7_dn11 = assign49520_e82608_d_n11;
        locals.var_t7_rv = 0.0;

        let (assign49530_e82617, assign49530_e82617_d_n3, assign49530_e82617_d_n4, assign49530_e82617_d_n5, assign49530_e82617_d_n6, assign49530_e82617_d_n7, assign49530_e82617_d_n8, assign49530_e82617_d_n9, assign49530_e82617_d_n10, assign49530_e82617_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49530_e82615: f64 = { let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49530_e82615, ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn3), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn4), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn5), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn6), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn7), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn8), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn9), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn10), ({ let limited_exp_arg = locals.var_t7; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t7_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49530_e82617;
        locals.var_t0_dn3 = assign49530_e82617_d_n3;
        locals.var_t0_dn4 = assign49530_e82617_d_n4;
        locals.var_t0_dn5 = assign49530_e82617_d_n5;
        locals.var_t0_dn6 = assign49530_e82617_d_n6;
        locals.var_t0_dn7 = assign49530_e82617_d_n7;
        locals.var_t0_dn8 = assign49530_e82617_d_n8;
        locals.var_t0_dn9 = assign49530_e82617_d_n9;
        locals.var_t0_dn10 = assign49530_e82617_d_n10;
        locals.var_t0_dn11 = assign49530_e82617_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49540_e82627, assign49540_e82627_d_n3, assign49540_e82627_d_n4, assign49540_e82627_d_n5, assign49540_e82627_d_n6, assign49540_e82627_d_n7, assign49540_e82627_d_n8, assign49540_e82627_d_n9, assign49540_e82627_d_n10, assign49540_e82627_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49540_e82625: f64 = (locals.var_idtun_i * locals.var_t0);
        (assign49540_e82625, (locals.var_idtun_i * locals.var_t0_dn3), (locals.var_idtun_i * locals.var_t0_dn4), (locals.var_idtun_i * locals.var_t0_dn5), (locals.var_idtun_i * locals.var_t0_dn6), (locals.var_idtun_i * locals.var_t0_dn7), (locals.var_idtun_i * locals.var_t0_dn8), (locals.var_idtun_i * locals.var_t0_dn9), (locals.var_idtun_i * locals.var_t0_dn10), (locals.var_idtun_i * locals.var_t0_dn11),)
    } else {
        (locals.var_jtund, locals.var_jtund_dn3, locals.var_jtund_dn4, locals.var_jtund_dn5, locals.var_jtund_dn6, locals.var_jtund_dn7, locals.var_jtund_dn8, locals.var_jtund_dn9, locals.var_jtund_dn10, locals.var_jtund_dn11,)
    }
};
        locals.var_jtund = assign49540_e82627;
        locals.var_jtund_dn3 = assign49540_e82627_d_n3;
        locals.var_jtund_dn4 = assign49540_e82627_d_n4;
        locals.var_jtund_dn5 = assign49540_e82627_d_n5;
        locals.var_jtund_dn6 = assign49540_e82627_d_n6;
        locals.var_jtund_dn7 = assign49540_e82627_d_n7;
        locals.var_jtund_dn8 = assign49540_e82627_d_n8;
        locals.var_jtund_dn9 = assign49540_e82627_d_n9;
        locals.var_jtund_dn10 = assign49540_e82627_d_n10;
        locals.var_jtund_dn11 = assign49540_e82627_d_n11;
        locals.var_jtund_rv = 0.0;

        let (assign49550_e82637, assign49550_e82637_d_n4, assign49550_e82637_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49550_e82635: f64 = (p.p925 * locals.var_ntun_i);
        (assign49550_e82635, 0.0, 0.0,)
    } else {
        (locals.var_nvtm2, locals.var_nvtm2_dn4, locals.var_nvtm2_dn5,)
    }
};
        locals.var_nvtm2 = assign49550_e82637;
        locals.var_nvtm2_dn4 = assign49550_e82637_d_n4;
        locals.var_nvtm2_dn5 = assign49550_e82637_d_n5;
        locals.var_nvtm2_rv = 0.0;

        let assign49560_e82640: f64 = (locals.var_vtun0_i - locals.var_vbs_jct);
        let assign49560_e82642: f64 = if assign49560_e82640 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard757 = assign49560_e82642;
        locals.var_guard757_rv = 0.0;

        let (assign49570_e82652, assign49570_e82652_d_n3, assign49570_e82652_d_n4, assign49570_e82652_d_n5, assign49570_e82652_d_n6, assign49570_e82652_d_n7, assign49570_e82652_d_n8, assign49570_e82652_d_n9, assign49570_e82652_d_n10, assign49570_e82652_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49570_e82652;
        locals.var_t1_dn3 = assign49570_e82652_d_n3;
        locals.var_t1_dn4 = assign49570_e82652_d_n4;
        locals.var_t1_dn5 = assign49570_e82652_d_n5;
        locals.var_t1_dn6 = assign49570_e82652_d_n6;
        locals.var_t1_dn7 = assign49570_e82652_d_n7;
        locals.var_t1_dn8 = assign49570_e82652_d_n8;
        locals.var_t1_dn9 = assign49570_e82652_d_n9;
        locals.var_t1_dn10 = assign49570_e82652_d_n10;
        locals.var_t1_dn11 = assign49570_e82652_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49580_e82669, assign49580_e82669_d_n3, assign49580_e82669_d_n4, assign49580_e82669_d_n5, assign49580_e82669_d_n6, assign49580_e82669_d_n7, assign49580_e82669_d_n8, assign49580_e82669_d_n9, assign49580_e82669_d_n10, assign49580_e82669_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 != 0.0)) {
        let assign49580_e82661: f64 = (-locals.var_vbs_jct);
        let assign49580_e82663: f64 = (assign49580_e82661 / locals.var_nvtm2);
        let assign49580_e82665: f64 = (assign49580_e82663 * locals.var_vtun0_i);
        let assign49580_e82667: f64 = (assign49580_e82665 * locals.var_t1);
        (assign49580_e82667, (assign49580_e82665 * locals.var_t1_dn3), ((((-((assign49580_e82661 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0_i) * locals.var_t1) + (assign49580_e82665 * locals.var_t1_dn4)), ((((-((assign49580_e82661 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0_i) * locals.var_t1) + (assign49580_e82665 * locals.var_t1_dn5)), (assign49580_e82665 * locals.var_t1_dn6), (((((-locals.var_vbs_jct_dn7) / locals.var_nvtm2) * locals.var_vtun0_i) * locals.var_t1) + (assign49580_e82665 * locals.var_t1_dn7)), (assign49580_e82665 * locals.var_t1_dn8), (assign49580_e82665 * locals.var_t1_dn9), (((((-locals.var_vbs_jct_dn10) / locals.var_nvtm2) * locals.var_vtun0_i) * locals.var_t1) + (assign49580_e82665 * locals.var_t1_dn10)), (assign49580_e82665 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49580_e82669;
        locals.var_t0_dn3 = assign49580_e82669_d_n3;
        locals.var_t0_dn4 = assign49580_e82669_d_n4;
        locals.var_t0_dn5 = assign49580_e82669_d_n5;
        locals.var_t0_dn6 = assign49580_e82669_d_n6;
        locals.var_t0_dn7 = assign49580_e82669_d_n7;
        locals.var_t0_dn8 = assign49580_e82669_d_n8;
        locals.var_t0_dn9 = assign49580_e82669_d_n9;
        locals.var_t0_dn10 = assign49580_e82669_d_n10;
        locals.var_t0_dn11 = assign49580_e82669_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49590_e82680, assign49590_e82680_d_n3, assign49590_e82680_d_n4, assign49590_e82680_d_n5, assign49590_e82680_d_n6, assign49590_e82680_d_n7, assign49590_e82680_d_n8, assign49590_e82680_d_n9, assign49590_e82680_d_n10, assign49590_e82680_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 != 0.0)) {
        let assign49590_e82678: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49590_e82678, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49590_e82680;
        locals.var_t1_dn3 = assign49590_e82680_d_n3;
        locals.var_t1_dn4 = assign49590_e82680_d_n4;
        locals.var_t1_dn5 = assign49590_e82680_d_n5;
        locals.var_t1_dn6 = assign49590_e82680_d_n6;
        locals.var_t1_dn7 = assign49590_e82680_d_n7;
        locals.var_t1_dn8 = assign49590_e82680_d_n8;
        locals.var_t1_dn9 = assign49590_e82680_d_n9;
        locals.var_t1_dn10 = assign49590_e82680_d_n10;
        locals.var_t1_dn11 = assign49590_e82680_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49600_e82692, assign49600_e82692_d_n3, assign49600_e82692_d_n4, assign49600_e82692_d_n5, assign49600_e82692_d_n6, assign49600_e82692_d_n7, assign49600_e82692_d_n8, assign49600_e82692_d_n9, assign49600_e82692_d_n10, assign49600_e82692_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 != 0.0)) {
        let assign49600_e82690: f64 = (locals.var_wstsi * locals.var_jtuns);
        (assign49600_e82690, (locals.var_wstsi * locals.var_jtuns_dn3), (locals.var_wstsi * locals.var_jtuns_dn4), (locals.var_wstsi * locals.var_jtuns_dn5), (locals.var_wstsi * locals.var_jtuns_dn6), (locals.var_wstsi * locals.var_jtuns_dn7), (locals.var_wstsi * locals.var_jtuns_dn8), (locals.var_wstsi * locals.var_jtuns_dn9), (locals.var_wstsi * locals.var_jtuns_dn10), (locals.var_wstsi * locals.var_jtuns_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign49600_e82692;
        locals.var_t3_dn3 = assign49600_e82692_d_n3;
        locals.var_t3_dn4 = assign49600_e82692_d_n4;
        locals.var_t3_dn5 = assign49600_e82692_d_n5;
        locals.var_t3_dn6 = assign49600_e82692_d_n6;
        locals.var_t3_dn7 = assign49600_e82692_d_n7;
        locals.var_t3_dn8 = assign49600_e82692_d_n8;
        locals.var_t3_dn9 = assign49600_e82692_d_n9;
        locals.var_t3_dn10 = assign49600_e82692_d_n10;
        locals.var_t3_dn11 = assign49600_e82692_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign49620_e82721, assign49620_e82721_d_n3, assign49620_e82721_d_n4, assign49620_e82721_d_n5, assign49620_e82721_d_n6, assign49620_e82721_d_n7, assign49620_e82721_d_n8, assign49620_e82721_d_n9, assign49620_e82721_d_n10, assign49620_e82721_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 == 0.0)) {
        let assign49620_e82718: f64 = (locals.var_vtun0_i - locals.var_vbs_jct);
        let assign49620_e82719: f64 = (1.0 / assign49620_e82718);
        (assign49620_e82719, 0.0, 0.0, 0.0, 0.0, (-((-locals.var_vbs_jct_dn7) / (assign49620_e82718 * assign49620_e82718))), 0.0, 0.0, (-((-locals.var_vbs_jct_dn10) / (assign49620_e82718 * assign49620_e82718))), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49620_e82721;
        locals.var_t1_dn3 = assign49620_e82721_d_n3;
        locals.var_t1_dn4 = assign49620_e82721_d_n4;
        locals.var_t1_dn5 = assign49620_e82721_d_n5;
        locals.var_t1_dn6 = assign49620_e82721_d_n6;
        locals.var_t1_dn7 = assign49620_e82721_d_n7;
        locals.var_t1_dn8 = assign49620_e82721_d_n8;
        locals.var_t1_dn9 = assign49620_e82721_d_n9;
        locals.var_t1_dn10 = assign49620_e82721_d_n10;
        locals.var_t1_dn11 = assign49620_e82721_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49630_e82739, assign49630_e82739_d_n3, assign49630_e82739_d_n4, assign49630_e82739_d_n5, assign49630_e82739_d_n6, assign49630_e82739_d_n7, assign49630_e82739_d_n8, assign49630_e82739_d_n9, assign49630_e82739_d_n10, assign49630_e82739_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 == 0.0)) {
        let assign49630_e82731: f64 = (-locals.var_vbs_jct);
        let assign49630_e82733: f64 = (assign49630_e82731 / locals.var_nvtm2);
        let assign49630_e82735: f64 = (assign49630_e82733 * locals.var_vtun0_i);
        let assign49630_e82737: f64 = (assign49630_e82735 * locals.var_t1);
        (assign49630_e82737, (assign49630_e82735 * locals.var_t1_dn3), ((((-((assign49630_e82731 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0_i) * locals.var_t1) + (assign49630_e82735 * locals.var_t1_dn4)), ((((-((assign49630_e82731 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0_i) * locals.var_t1) + (assign49630_e82735 * locals.var_t1_dn5)), (assign49630_e82735 * locals.var_t1_dn6), (((((-locals.var_vbs_jct_dn7) / locals.var_nvtm2) * locals.var_vtun0_i) * locals.var_t1) + (assign49630_e82735 * locals.var_t1_dn7)), (assign49630_e82735 * locals.var_t1_dn8), (assign49630_e82735 * locals.var_t1_dn9), (((((-locals.var_vbs_jct_dn10) / locals.var_nvtm2) * locals.var_vtun0_i) * locals.var_t1) + (assign49630_e82735 * locals.var_t1_dn10)), (assign49630_e82735 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49630_e82739;
        locals.var_t0_dn3 = assign49630_e82739_d_n3;
        locals.var_t0_dn4 = assign49630_e82739_d_n4;
        locals.var_t0_dn5 = assign49630_e82739_d_n5;
        locals.var_t0_dn6 = assign49630_e82739_d_n6;
        locals.var_t0_dn7 = assign49630_e82739_d_n7;
        locals.var_t0_dn8 = assign49630_e82739_d_n8;
        locals.var_t0_dn9 = assign49630_e82739_d_n9;
        locals.var_t0_dn10 = assign49630_e82739_d_n10;
        locals.var_t0_dn11 = assign49630_e82739_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49640_e82751, assign49640_e82751_d_n3, assign49640_e82751_d_n4, assign49640_e82751_d_n5, assign49640_e82751_d_n6, assign49640_e82751_d_n7, assign49640_e82751_d_n8, assign49640_e82751_d_n9, assign49640_e82751_d_n10, assign49640_e82751_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 == 0.0)) {
        let assign49640_e82749: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49640_e82749, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49640_e82751;
        locals.var_t1_dn3 = assign49640_e82751_d_n3;
        locals.var_t1_dn4 = assign49640_e82751_d_n4;
        locals.var_t1_dn5 = assign49640_e82751_d_n5;
        locals.var_t1_dn6 = assign49640_e82751_d_n6;
        locals.var_t1_dn7 = assign49640_e82751_d_n7;
        locals.var_t1_dn8 = assign49640_e82751_d_n8;
        locals.var_t1_dn9 = assign49640_e82751_d_n9;
        locals.var_t1_dn10 = assign49640_e82751_d_n10;
        locals.var_t1_dn11 = assign49640_e82751_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49650_e82764, assign49650_e82764_d_n3, assign49650_e82764_d_n4, assign49650_e82764_d_n5, assign49650_e82764_d_n6, assign49650_e82764_d_n7, assign49650_e82764_d_n8, assign49650_e82764_d_n9, assign49650_e82764_d_n10, assign49650_e82764_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard757 == 0.0)) {
        let assign49650_e82762: f64 = (locals.var_wstsi * locals.var_jtuns);
        (assign49650_e82762, (locals.var_wstsi * locals.var_jtuns_dn3), (locals.var_wstsi * locals.var_jtuns_dn4), (locals.var_wstsi * locals.var_jtuns_dn5), (locals.var_wstsi * locals.var_jtuns_dn6), (locals.var_wstsi * locals.var_jtuns_dn7), (locals.var_wstsi * locals.var_jtuns_dn8), (locals.var_wstsi * locals.var_jtuns_dn9), (locals.var_wstsi * locals.var_jtuns_dn10), (locals.var_wstsi * locals.var_jtuns_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign49650_e82764;
        locals.var_t3_dn3 = assign49650_e82764_d_n3;
        locals.var_t3_dn4 = assign49650_e82764_d_n4;
        locals.var_t3_dn5 = assign49650_e82764_d_n5;
        locals.var_t3_dn6 = assign49650_e82764_d_n6;
        locals.var_t3_dn7 = assign49650_e82764_d_n7;
        locals.var_t3_dn8 = assign49650_e82764_d_n8;
        locals.var_t3_dn9 = assign49650_e82764_d_n9;
        locals.var_t3_dn10 = assign49650_e82764_d_n10;
        locals.var_t3_dn11 = assign49650_e82764_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign49670_e82789, assign49670_e82789_d_n4, assign49670_e82789_d_n5,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) {
        let assign49670_e82787: f64 = (p.p925 * locals.var_ntund_i);
        (assign49670_e82787, 0.0, 0.0,)
    } else {
        (locals.var_nvtm2, locals.var_nvtm2_dn4, locals.var_nvtm2_dn5,)
    }
};
        locals.var_nvtm2 = assign49670_e82789;
        locals.var_nvtm2_dn4 = assign49670_e82789_d_n4;
        locals.var_nvtm2_dn5 = assign49670_e82789_d_n5;
        locals.var_nvtm2_rv = 0.0;

        let assign49680_e82792: f64 = (locals.var_vtun0d_i - locals.var_vbd_jct);
        let assign49680_e82794: f64 = if assign49680_e82792 < 0.001 { 1.0 } else { 0.0 };
        locals.var_guard758 = assign49680_e82794;
        locals.var_guard758_rv = 0.0;

        let (assign49690_e82804, assign49690_e82804_d_n3, assign49690_e82804_d_n4, assign49690_e82804_d_n5, assign49690_e82804_d_n6, assign49690_e82804_d_n7, assign49690_e82804_d_n8, assign49690_e82804_d_n9, assign49690_e82804_d_n10, assign49690_e82804_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49690_e82804;
        locals.var_t1_dn3 = assign49690_e82804_d_n3;
        locals.var_t1_dn4 = assign49690_e82804_d_n4;
        locals.var_t1_dn5 = assign49690_e82804_d_n5;
        locals.var_t1_dn6 = assign49690_e82804_d_n6;
        locals.var_t1_dn7 = assign49690_e82804_d_n7;
        locals.var_t1_dn8 = assign49690_e82804_d_n8;
        locals.var_t1_dn9 = assign49690_e82804_d_n9;
        locals.var_t1_dn10 = assign49690_e82804_d_n10;
        locals.var_t1_dn11 = assign49690_e82804_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49700_e82821, assign49700_e82821_d_n3, assign49700_e82821_d_n4, assign49700_e82821_d_n5, assign49700_e82821_d_n6, assign49700_e82821_d_n7, assign49700_e82821_d_n8, assign49700_e82821_d_n9, assign49700_e82821_d_n10, assign49700_e82821_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 != 0.0)) {
        let assign49700_e82813: f64 = (-locals.var_vbd_jct);
        let assign49700_e82815: f64 = (assign49700_e82813 / locals.var_nvtm2);
        let assign49700_e82817: f64 = (assign49700_e82815 * locals.var_vtun0d_i);
        let assign49700_e82819: f64 = (assign49700_e82817 * locals.var_t1);
        (assign49700_e82819, (assign49700_e82817 * locals.var_t1_dn3), ((((-((assign49700_e82813 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0d_i) * locals.var_t1) + (assign49700_e82817 * locals.var_t1_dn4)), ((((-((assign49700_e82813 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0d_i) * locals.var_t1) + (assign49700_e82817 * locals.var_t1_dn5)), (((((-locals.var_vbd_jct_dn6) / locals.var_nvtm2) * locals.var_vtun0d_i) * locals.var_t1) + (assign49700_e82817 * locals.var_t1_dn6)), (assign49700_e82817 * locals.var_t1_dn7), (assign49700_e82817 * locals.var_t1_dn8), (assign49700_e82817 * locals.var_t1_dn9), (((((-locals.var_vbd_jct_dn10) / locals.var_nvtm2) * locals.var_vtun0d_i) * locals.var_t1) + (assign49700_e82817 * locals.var_t1_dn10)), (assign49700_e82817 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49700_e82821;
        locals.var_t0_dn3 = assign49700_e82821_d_n3;
        locals.var_t0_dn4 = assign49700_e82821_d_n4;
        locals.var_t0_dn5 = assign49700_e82821_d_n5;
        locals.var_t0_dn6 = assign49700_e82821_d_n6;
        locals.var_t0_dn7 = assign49700_e82821_d_n7;
        locals.var_t0_dn8 = assign49700_e82821_d_n8;
        locals.var_t0_dn9 = assign49700_e82821_d_n9;
        locals.var_t0_dn10 = assign49700_e82821_d_n10;
        locals.var_t0_dn11 = assign49700_e82821_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49710_e82832, assign49710_e82832_d_n3, assign49710_e82832_d_n4, assign49710_e82832_d_n5, assign49710_e82832_d_n6, assign49710_e82832_d_n7, assign49710_e82832_d_n8, assign49710_e82832_d_n9, assign49710_e82832_d_n10, assign49710_e82832_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 != 0.0)) {
        let assign49710_e82830: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49710_e82830, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49710_e82832;
        locals.var_t1_dn3 = assign49710_e82832_d_n3;
        locals.var_t1_dn4 = assign49710_e82832_d_n4;
        locals.var_t1_dn5 = assign49710_e82832_d_n5;
        locals.var_t1_dn6 = assign49710_e82832_d_n6;
        locals.var_t1_dn7 = assign49710_e82832_d_n7;
        locals.var_t1_dn8 = assign49710_e82832_d_n8;
        locals.var_t1_dn9 = assign49710_e82832_d_n9;
        locals.var_t1_dn10 = assign49710_e82832_d_n10;
        locals.var_t1_dn11 = assign49710_e82832_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49720_e82844, assign49720_e82844_d_n3, assign49720_e82844_d_n4, assign49720_e82844_d_n5, assign49720_e82844_d_n6, assign49720_e82844_d_n7, assign49720_e82844_d_n8, assign49720_e82844_d_n9, assign49720_e82844_d_n10, assign49720_e82844_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 != 0.0)) {
        let assign49720_e82842: f64 = (locals.var_wstsi * locals.var_jtund);
        (assign49720_e82842, (locals.var_wstsi * locals.var_jtund_dn3), (locals.var_wstsi * locals.var_jtund_dn4), (locals.var_wstsi * locals.var_jtund_dn5), (locals.var_wstsi * locals.var_jtund_dn6), (locals.var_wstsi * locals.var_jtund_dn7), (locals.var_wstsi * locals.var_jtund_dn8), (locals.var_wstsi * locals.var_jtund_dn9), (locals.var_wstsi * locals.var_jtund_dn10), (locals.var_wstsi * locals.var_jtund_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign49720_e82844;
        locals.var_t3_dn3 = assign49720_e82844_d_n3;
        locals.var_t3_dn4 = assign49720_e82844_d_n4;
        locals.var_t3_dn5 = assign49720_e82844_d_n5;
        locals.var_t3_dn6 = assign49720_e82844_d_n6;
        locals.var_t3_dn7 = assign49720_e82844_d_n7;
        locals.var_t3_dn8 = assign49720_e82844_d_n8;
        locals.var_t3_dn9 = assign49720_e82844_d_n9;
        locals.var_t3_dn10 = assign49720_e82844_d_n10;
        locals.var_t3_dn11 = assign49720_e82844_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign49740_e82873, assign49740_e82873_d_n3, assign49740_e82873_d_n4, assign49740_e82873_d_n5, assign49740_e82873_d_n6, assign49740_e82873_d_n7, assign49740_e82873_d_n8, assign49740_e82873_d_n9, assign49740_e82873_d_n10, assign49740_e82873_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 == 0.0)) {
        let assign49740_e82870: f64 = (locals.var_vtun0d_i - locals.var_vbd_jct);
        let assign49740_e82871: f64 = (1.0 / assign49740_e82870);
        (assign49740_e82871, 0.0, 0.0, 0.0, (-((-locals.var_vbd_jct_dn6) / (assign49740_e82870 * assign49740_e82870))), 0.0, 0.0, 0.0, (-((-locals.var_vbd_jct_dn10) / (assign49740_e82870 * assign49740_e82870))), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49740_e82873;
        locals.var_t1_dn3 = assign49740_e82873_d_n3;
        locals.var_t1_dn4 = assign49740_e82873_d_n4;
        locals.var_t1_dn5 = assign49740_e82873_d_n5;
        locals.var_t1_dn6 = assign49740_e82873_d_n6;
        locals.var_t1_dn7 = assign49740_e82873_d_n7;
        locals.var_t1_dn8 = assign49740_e82873_d_n8;
        locals.var_t1_dn9 = assign49740_e82873_d_n9;
        locals.var_t1_dn10 = assign49740_e82873_d_n10;
        locals.var_t1_dn11 = assign49740_e82873_d_n11;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_173(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign49750_e82891, assign49750_e82891_d_n3, assign49750_e82891_d_n4, assign49750_e82891_d_n5, assign49750_e82891_d_n6, assign49750_e82891_d_n7, assign49750_e82891_d_n8, assign49750_e82891_d_n9, assign49750_e82891_d_n10, assign49750_e82891_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 == 0.0)) {
        let assign49750_e82883: f64 = (-locals.var_vbd_jct);
        let assign49750_e82885: f64 = (assign49750_e82883 / locals.var_nvtm2);
        let assign49750_e82887: f64 = (assign49750_e82885 * locals.var_vtun0d_i);
        let assign49750_e82889: f64 = (assign49750_e82887 * locals.var_t1);
        (assign49750_e82889, (assign49750_e82887 * locals.var_t1_dn3), ((((-((assign49750_e82883 * locals.var_nvtm2_dn4) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0d_i) * locals.var_t1) + (assign49750_e82887 * locals.var_t1_dn4)), ((((-((assign49750_e82883 * locals.var_nvtm2_dn5) / (locals.var_nvtm2 * locals.var_nvtm2))) * locals.var_vtun0d_i) * locals.var_t1) + (assign49750_e82887 * locals.var_t1_dn5)), (((((-locals.var_vbd_jct_dn6) / locals.var_nvtm2) * locals.var_vtun0d_i) * locals.var_t1) + (assign49750_e82887 * locals.var_t1_dn6)), (assign49750_e82887 * locals.var_t1_dn7), (assign49750_e82887 * locals.var_t1_dn8), (assign49750_e82887 * locals.var_t1_dn9), (((((-locals.var_vbd_jct_dn10) / locals.var_nvtm2) * locals.var_vtun0d_i) * locals.var_t1) + (assign49750_e82887 * locals.var_t1_dn10)), (assign49750_e82887 * locals.var_t1_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49750_e82891;
        locals.var_t0_dn3 = assign49750_e82891_d_n3;
        locals.var_t0_dn4 = assign49750_e82891_d_n4;
        locals.var_t0_dn5 = assign49750_e82891_d_n5;
        locals.var_t0_dn6 = assign49750_e82891_d_n6;
        locals.var_t0_dn7 = assign49750_e82891_d_n7;
        locals.var_t0_dn8 = assign49750_e82891_d_n8;
        locals.var_t0_dn9 = assign49750_e82891_d_n9;
        locals.var_t0_dn10 = assign49750_e82891_d_n10;
        locals.var_t0_dn11 = assign49750_e82891_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign49760_e82903, assign49760_e82903_d_n3, assign49760_e82903_d_n4, assign49760_e82903_d_n5, assign49760_e82903_d_n6, assign49760_e82903_d_n7, assign49760_e82903_d_n8, assign49760_e82903_d_n9, assign49760_e82903_d_n10, assign49760_e82903_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 == 0.0)) {
        let assign49760_e82901: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign49760_e82901, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49760_e82903;
        locals.var_t1_dn3 = assign49760_e82903_d_n3;
        locals.var_t1_dn4 = assign49760_e82903_d_n4;
        locals.var_t1_dn5 = assign49760_e82903_d_n5;
        locals.var_t1_dn6 = assign49760_e82903_d_n6;
        locals.var_t1_dn7 = assign49760_e82903_d_n7;
        locals.var_t1_dn8 = assign49760_e82903_d_n8;
        locals.var_t1_dn9 = assign49760_e82903_d_n9;
        locals.var_t1_dn10 = assign49760_e82903_d_n10;
        locals.var_t1_dn11 = assign49760_e82903_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49770_e82916, assign49770_e82916_d_n3, assign49770_e82916_d_n4, assign49770_e82916_d_n5, assign49770_e82916_d_n6, assign49770_e82916_d_n7, assign49770_e82916_d_n8, assign49770_e82916_d_n9, assign49770_e82916_d_n10, assign49770_e82916_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard756 == 0.0)) && (locals.var_guard758 == 0.0)) {
        let assign49770_e82914: f64 = (locals.var_wstsi * locals.var_jtund);
        (assign49770_e82914, (locals.var_wstsi * locals.var_jtund_dn3), (locals.var_wstsi * locals.var_jtund_dn4), (locals.var_wstsi * locals.var_jtund_dn5), (locals.var_wstsi * locals.var_jtund_dn6), (locals.var_wstsi * locals.var_jtund_dn7), (locals.var_wstsi * locals.var_jtund_dn8), (locals.var_wstsi * locals.var_jtund_dn9), (locals.var_wstsi * locals.var_jtund_dn10), (locals.var_wstsi * locals.var_jtund_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign49770_e82916;
        locals.var_t3_dn3 = assign49770_e82916_d_n3;
        locals.var_t3_dn4 = assign49770_e82916_d_n4;
        locals.var_t3_dn5 = assign49770_e82916_d_n5;
        locals.var_t3_dn6 = assign49770_e82916_d_n6;
        locals.var_t3_dn7 = assign49770_e82916_d_n7;
        locals.var_t3_dn8 = assign49770_e82916_d_n8;
        locals.var_t3_dn9 = assign49770_e82916_d_n9;
        locals.var_t3_dn10 = assign49770_e82916_d_n10;
        locals.var_t3_dn11 = assign49770_e82916_d_n11;
        locals.var_t3_rv = 0.0;

        let assign49830_e82970: f64 = if p.p36 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard759 = assign49830_e82970;
        locals.var_guard759_rv = 0.0;

        let (assign49840_e82979, assign49840_e82979_d_n3, assign49840_e82979_d_n4, assign49840_e82979_d_n5, assign49840_e82979_d_n6, assign49840_e82979_d_n7, assign49840_e82979_d_n8, assign49840_e82979_d_n9, assign49840_e82979_d_n10, assign49840_e82979_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) {
        let assign49840_e82977: f64 = (locals.var_epsratio * p.p76);
        (assign49840_e82977, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign49840_e82979;
        locals.var_t0_dn3 = assign49840_e82979_d_n3;
        locals.var_t0_dn4 = assign49840_e82979_d_n4;
        locals.var_t0_dn5 = assign49840_e82979_d_n5;
        locals.var_t0_dn6 = assign49840_e82979_d_n6;
        locals.var_t0_dn7 = assign49840_e82979_d_n7;
        locals.var_t0_dn8 = assign49840_e82979_d_n8;
        locals.var_t0_dn9 = assign49840_e82979_d_n9;
        locals.var_t0_dn10 = assign49840_e82979_d_n10;
        locals.var_t0_dn11 = assign49840_e82979_d_n11;
        locals.var_t0_rv = 0.0;

        let assign49850_e82990: f64 = if (((locals.var_agidl_i <= 0.0) || (locals.var_bgidl_t <= 0.0)) || (locals.var_cgidl_i < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard760 = assign49850_e82990;
        locals.var_guard760_rv = 0.0;

        let (assign49860_e82999, assign49860_e82999_d_n3, assign49860_e82999_d_n4, assign49860_e82999_d_n5, assign49860_e82999_d_n6, assign49860_e82999_d_n7, assign49860_e82999_d_n8, assign49860_e82999_d_n9, assign49860_e82999_d_n10, assign49860_e82999_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign49860_e82999;
        locals.var_t6_dn3 = assign49860_e82999_d_n3;
        locals.var_t6_dn4 = assign49860_e82999_d_n4;
        locals.var_t6_dn5 = assign49860_e82999_d_n5;
        locals.var_t6_dn6 = assign49860_e82999_d_n6;
        locals.var_t6_dn7 = assign49860_e82999_d_n7;
        locals.var_t6_dn8 = assign49860_e82999_d_n8;
        locals.var_t6_dn9 = assign49860_e82999_d_n9;
        locals.var_t6_dn10 = assign49860_e82999_d_n10;
        locals.var_t6_dn11 = assign49860_e82999_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign49870_e83016, assign49870_e83016_d_n3, assign49870_e83016_d_n4, assign49870_e83016_d_n5, assign49870_e83016_d_n6, assign49870_e83016_d_n7, assign49870_e83016_d_n8, assign49870_e83016_d_n9, assign49870_e83016_d_n10, assign49870_e83016_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) {
        let assign49870_e83008: f64 = (-locals.var_vgd_noswap);
        let assign49870_e83010: f64 = (assign49870_e83008 - locals.var_egidl_i);
        let assign49870_e83012: f64 = (assign49870_e83010 + locals.var_vfbsdr);
        let assign49870_e83014: f64 = (assign49870_e83012 / locals.var_t0);
        (assign49870_e83014, (-((assign49870_e83012 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign49870_e83012 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsdr_dn5 * locals.var_t0) - (assign49870_e83012 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_dn6) * locals.var_t0) - (assign49870_e83012 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_dn7) * locals.var_t0) - (assign49870_e83012 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_dn8) * locals.var_t0) - (assign49870_e83012 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (-((assign49870_e83012 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgd_noswap_dn10) * locals.var_t0) - (assign49870_e83012 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (-((assign49870_e83012 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49870_e83016;
        locals.var_t1_dn3 = assign49870_e83016_d_n3;
        locals.var_t1_dn4 = assign49870_e83016_d_n4;
        locals.var_t1_dn5 = assign49870_e83016_d_n5;
        locals.var_t1_dn6 = assign49870_e83016_d_n6;
        locals.var_t1_dn7 = assign49870_e83016_d_n7;
        locals.var_t1_dn8 = assign49870_e83016_d_n8;
        locals.var_t1_dn9 = assign49870_e83016_d_n9;
        locals.var_t1_dn10 = assign49870_e83016_d_n10;
        locals.var_t1_dn11 = assign49870_e83016_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49880_e83039, assign49880_e83039_d_n3, assign49880_e83039_d_n4, assign49880_e83039_d_n5, assign49880_e83039_d_n6, assign49880_e83039_d_n7, assign49880_e83039_d_n8, assign49880_e83039_d_n9, assign49880_e83039_d_n10, assign49880_e83039_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) {
        let assign49880_e83028: f64 = (locals.var_t1 * locals.var_t1);
        let assign49880_e83031: f64 = (4.0 * 0.01);
        let assign49880_e83033: f64 = (assign49880_e83031 * 0.01);
        let assign49880_e83034: f64 = (assign49880_e83028 + assign49880_e83033);
        let assign49880_e83035: f64 = (assign49880_e83034).sqrt();
        let assign49880_e83036: f64 = (locals.var_t1 + assign49880_e83035);
        let assign49880_e83037: f64 = (0.5 * assign49880_e83036);
        (assign49880_e83037, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign49880_e83035)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign49880_e83035)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49880_e83039;
        locals.var_t1_dn3 = assign49880_e83039_d_n3;
        locals.var_t1_dn4 = assign49880_e83039_d_n4;
        locals.var_t1_dn5 = assign49880_e83039_d_n5;
        locals.var_t1_dn6 = assign49880_e83039_d_n6;
        locals.var_t1_dn7 = assign49880_e83039_d_n7;
        locals.var_t1_dn8 = assign49880_e83039_d_n8;
        locals.var_t1_dn9 = assign49880_e83039_d_n9;
        locals.var_t1_dn10 = assign49880_e83039_d_n10;
        locals.var_t1_dn11 = assign49880_e83039_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign49890_e83053, assign49890_e83053_d_n3, assign49890_e83053_d_n4, assign49890_e83053_d_n5, assign49890_e83053_d_n6, assign49890_e83053_d_n7, assign49890_e83053_d_n8, assign49890_e83053_d_n9, assign49890_e83053_d_n10, assign49890_e83053_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) {
        let assign49890_e83050: f64 = (locals.var_t1 + 0.001);
        let assign49890_e83051: f64 = (locals.var_bgidl_t / assign49890_e83050);
        (assign49890_e83051, (-((locals.var_bgidl_t * locals.var_t1_dn3) / (assign49890_e83050 * assign49890_e83050))), (((locals.var_bgidl_t_dn4 * assign49890_e83050) - (locals.var_bgidl_t * locals.var_t1_dn4)) / (assign49890_e83050 * assign49890_e83050)), (((locals.var_bgidl_t_dn5 * assign49890_e83050) - (locals.var_bgidl_t * locals.var_t1_dn5)) / (assign49890_e83050 * assign49890_e83050)), (-((locals.var_bgidl_t * locals.var_t1_dn6) / (assign49890_e83050 * assign49890_e83050))), (-((locals.var_bgidl_t * locals.var_t1_dn7) / (assign49890_e83050 * assign49890_e83050))), (-((locals.var_bgidl_t * locals.var_t1_dn8) / (assign49890_e83050 * assign49890_e83050))), (-((locals.var_bgidl_t * locals.var_t1_dn9) / (assign49890_e83050 * assign49890_e83050))), (-((locals.var_bgidl_t * locals.var_t1_dn10) / (assign49890_e83050 * assign49890_e83050))), (-((locals.var_bgidl_t * locals.var_t1_dn11) / (assign49890_e83050 * assign49890_e83050))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign49890_e83053;
        locals.var_t2_dn3 = assign49890_e83053_d_n3;
        locals.var_t2_dn4 = assign49890_e83053_d_n4;
        locals.var_t2_dn5 = assign49890_e83053_d_n5;
        locals.var_t2_dn6 = assign49890_e83053_d_n6;
        locals.var_t2_dn7 = assign49890_e83053_d_n7;
        locals.var_t2_dn8 = assign49890_e83053_d_n8;
        locals.var_t2_dn9 = assign49890_e83053_d_n9;
        locals.var_t2_dn10 = assign49890_e83053_d_n10;
        locals.var_t2_dn11 = assign49890_e83053_d_n11;
        locals.var_t2_rv = 0.0;

        let assign49900_e83056: f64 = if locals.var_cgidl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard761 = assign49900_e83056;
        locals.var_guard761_rv = 0.0;

        let (assign49910_e83072, assign49910_e83072_d_n3, assign49910_e83072_d_n4, assign49910_e83072_d_n5, assign49910_e83072_d_n6, assign49910_e83072_d_n7, assign49910_e83072_d_n8, assign49910_e83072_d_n9, assign49910_e83072_d_n10, assign49910_e83072_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) && (locals.var_guard761 != 0.0)) {
        let assign49910_e83068: f64 = (locals.var_vdb_noswap * locals.var_vdb_noswap);
        let assign49910_e83070: f64 = (assign49910_e83068 * locals.var_vdb_noswap);
        (assign49910_e83070, 0.0, 0.0, 0.0, ((((locals.var_vdb_noswap_dn6 * locals.var_vdb_noswap) + (locals.var_vdb_noswap * locals.var_vdb_noswap_dn6)) * locals.var_vdb_noswap) + (assign49910_e83068 * locals.var_vdb_noswap_dn6)), ((((locals.var_vdb_noswap_dn7 * locals.var_vdb_noswap) + (locals.var_vdb_noswap * locals.var_vdb_noswap_dn7)) * locals.var_vdb_noswap) + (assign49910_e83068 * locals.var_vdb_noswap_dn7)), 0.0, 0.0, ((((locals.var_vdb_noswap_dn10 * locals.var_vdb_noswap) + (locals.var_vdb_noswap * locals.var_vdb_noswap_dn10)) * locals.var_vdb_noswap) + (assign49910_e83068 * locals.var_vdb_noswap_dn10)), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign49910_e83072;
        locals.var_t3_dn3 = assign49910_e83072_d_n3;
        locals.var_t3_dn4 = assign49910_e83072_d_n4;
        locals.var_t3_dn5 = assign49910_e83072_d_n5;
        locals.var_t3_dn6 = assign49910_e83072_d_n6;
        locals.var_t3_dn7 = assign49910_e83072_d_n7;
        locals.var_t3_dn8 = assign49910_e83072_d_n8;
        locals.var_t3_dn9 = assign49910_e83072_d_n9;
        locals.var_t3_dn10 = assign49910_e83072_d_n10;
        locals.var_t3_dn11 = assign49910_e83072_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign49920_e83089, assign49920_e83089_d_n3, assign49920_e83089_d_n4, assign49920_e83089_d_n5, assign49920_e83089_d_n6, assign49920_e83089_d_n7, assign49920_e83089_d_n8, assign49920_e83089_d_n9, assign49920_e83089_d_n10, assign49920_e83089_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) && (locals.var_guard761 != 0.0)) {
        let assign49920_e83084: f64 = (locals.var_t3).abs();
        let assign49920_e83085: f64 = (locals.var_cgidl_i + assign49920_e83084);
        let assign49920_e83087: f64 = (assign49920_e83085 + 0.0001);
        (assign49920_e83087, if locals.var_t3 >= 0.0 { locals.var_t3_dn3 } else { (-locals.var_t3_dn3) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn4 } else { (-locals.var_t3_dn4) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn5 } else { (-locals.var_t3_dn5) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn6 } else { (-locals.var_t3_dn6) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn7 } else { (-locals.var_t3_dn7) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn8 } else { (-locals.var_t3_dn8) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn9 } else { (-locals.var_t3_dn9) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn10 } else { (-locals.var_t3_dn10) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn11 } else { (-locals.var_t3_dn11) },)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign49920_e83089;
        locals.var_t4_dn3 = assign49920_e83089_d_n3;
        locals.var_t4_dn4 = assign49920_e83089_d_n4;
        locals.var_t4_dn5 = assign49920_e83089_d_n5;
        locals.var_t4_dn6 = assign49920_e83089_d_n6;
        locals.var_t4_dn7 = assign49920_e83089_d_n7;
        locals.var_t4_dn8 = assign49920_e83089_d_n8;
        locals.var_t4_dn9 = assign49920_e83089_d_n9;
        locals.var_t4_dn10 = assign49920_e83089_d_n10;
        locals.var_t4_dn11 = assign49920_e83089_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign49930_e83122, assign49930_e83122_d_n3, assign49930_e83122_d_n4, assign49930_e83122_d_n5, assign49930_e83122_d_n6, assign49930_e83122_d_n7, assign49930_e83122_d_n8, assign49930_e83122_d_n9, assign49930_e83122_d_n10, assign49930_e83122_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) && (locals.var_guard761 != 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t4;
        let assign49930_e83102: f64 = (locals.var_t3 * __rspice_inv_cse_0);
        let assign49930_e83105: f64 = (locals.var_t3 * __rspice_inv_cse_0);
        let assign49930_e83108: f64 = (locals.var_t3 * __rspice_inv_cse_0);
        let assign49930_e83109: f64 = (assign49930_e83105 * assign49930_e83108);
        let assign49930_e83112: f64 = (4.0 * 1e-6);
        let assign49930_e83114: f64 = (assign49930_e83112 * 1e-6);
        let assign49930_e83115: f64 = (assign49930_e83109 + assign49930_e83114);
        let assign49930_e83116: f64 = (assign49930_e83115).sqrt();
        let assign49930_e83117: f64 = (assign49930_e83102 + assign49930_e83116);
        let assign49930_e83118: f64 = (0.5 * assign49930_e83117);
        let assign49930_e83120: f64 = (assign49930_e83118 - 1e-6);
        (assign49930_e83120, (0.5 * ((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))), (0.5 * ((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * assign49930_e83108) + (assign49930_e83105 * (((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign49930_e83116)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign49930_e83122;
        locals.var_t5_dn3 = assign49930_e83122_d_n3;
        locals.var_t5_dn4 = assign49930_e83122_d_n4;
        locals.var_t5_dn5 = assign49930_e83122_d_n5;
        locals.var_t5_dn6 = assign49930_e83122_d_n6;
        locals.var_t5_dn7 = assign49930_e83122_d_n7;
        locals.var_t5_dn8 = assign49930_e83122_d_n8;
        locals.var_t5_dn9 = assign49930_e83122_d_n9;
        locals.var_t5_dn10 = assign49930_e83122_d_n10;
        locals.var_t5_dn11 = assign49930_e83122_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign49940_e83135, assign49940_e83135_d_n3, assign49940_e83135_d_n4, assign49940_e83135_d_n5, assign49940_e83135_d_n6, assign49940_e83135_d_n7, assign49940_e83135_d_n8, assign49940_e83135_d_n9, assign49940_e83135_d_n10, assign49940_e83135_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) && (locals.var_guard761 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign49940_e83135;
        locals.var_t5_dn3 = assign49940_e83135_d_n3;
        locals.var_t5_dn4 = assign49940_e83135_d_n4;
        locals.var_t5_dn5 = assign49940_e83135_d_n5;
        locals.var_t5_dn6 = assign49940_e83135_d_n6;
        locals.var_t5_dn7 = assign49940_e83135_d_n7;
        locals.var_t5_dn8 = assign49940_e83135_d_n8;
        locals.var_t5_dn9 = assign49940_e83135_d_n9;
        locals.var_t5_dn10 = assign49940_e83135_d_n10;
        locals.var_t5_dn11 = assign49940_e83135_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign49950_e83155, assign49950_e83155_d_n3, assign49950_e83155_d_n4, assign49950_e83155_d_n5, assign49950_e83155_d_n6, assign49950_e83155_d_n7, assign49950_e83155_d_n8, assign49950_e83155_d_n9, assign49950_e83155_d_n10, assign49950_e83155_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard760 == 0.0)) {
        let assign49950_e83145: f64 = (locals.var_agidl_i * locals.var_wdiod);
        let assign49950_e83147: f64 = (assign49950_e83145 * locals.var_t1);
        let assign49950_e83149: f64 = (-locals.var_t2);
        let assign49950_e83150: f64 = { let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign49950_e83151: f64 = (assign49950_e83147 * assign49950_e83150);
        let assign49950_e83153: f64 = (assign49950_e83151 * locals.var_t5);
        (assign49950_e83153, (((((assign49950_e83145 * locals.var_t1_dn3) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn3)), (((((assign49950_e83145 * locals.var_t1_dn4) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn4)), (((((assign49950_e83145 * locals.var_t1_dn5) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn5)), (((((assign49950_e83145 * locals.var_t1_dn6) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn6)), (((((assign49950_e83145 * locals.var_t1_dn7) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn7)), (((((assign49950_e83145 * locals.var_t1_dn8) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn8)), (((((assign49950_e83145 * locals.var_t1_dn9) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn9)), (((((assign49950_e83145 * locals.var_t1_dn10) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn10)), (((((assign49950_e83145 * locals.var_t1_dn11) * assign49950_e83150) + (assign49950_e83147 * ({ let limited_exp_arg = assign49950_e83149; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * locals.var_t5) + (assign49950_e83151 * locals.var_t5_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign49950_e83155;
        locals.var_t6_dn3 = assign49950_e83155_d_n3;
        locals.var_t6_dn4 = assign49950_e83155_d_n4;
        locals.var_t6_dn5 = assign49950_e83155_d_n5;
        locals.var_t6_dn6 = assign49950_e83155_d_n6;
        locals.var_t6_dn7 = assign49950_e83155_d_n7;
        locals.var_t6_dn8 = assign49950_e83155_d_n8;
        locals.var_t6_dn9 = assign49950_e83155_d_n9;
        locals.var_t6_dn10 = assign49950_e83155_d_n10;
        locals.var_t6_dn11 = assign49950_e83155_d_n11;
        locals.var_t6_rv = 0.0;

        let assign49970_e83173: f64 = if (((locals.var_agisl_i <= 0.0) || (locals.var_bgisl_t <= 0.0)) || (locals.var_cgisl_i < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard762 = assign49970_e83173;
        locals.var_guard762_rv = 0.0;

        let (assign49980_e83182, assign49980_e83182_d_n3, assign49980_e83182_d_n4, assign49980_e83182_d_n5, assign49980_e83182_d_n6, assign49980_e83182_d_n7, assign49980_e83182_d_n8, assign49980_e83182_d_n9, assign49980_e83182_d_n10, assign49980_e83182_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign49980_e83182;
        locals.var_t6_dn3 = assign49980_e83182_d_n3;
        locals.var_t6_dn4 = assign49980_e83182_d_n4;
        locals.var_t6_dn5 = assign49980_e83182_d_n5;
        locals.var_t6_dn6 = assign49980_e83182_d_n6;
        locals.var_t6_dn7 = assign49980_e83182_d_n7;
        locals.var_t6_dn8 = assign49980_e83182_d_n8;
        locals.var_t6_dn9 = assign49980_e83182_d_n9;
        locals.var_t6_dn10 = assign49980_e83182_d_n10;
        locals.var_t6_dn11 = assign49980_e83182_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign49990_e83199, assign49990_e83199_d_n3, assign49990_e83199_d_n4, assign49990_e83199_d_n5, assign49990_e83199_d_n6, assign49990_e83199_d_n7, assign49990_e83199_d_n8, assign49990_e83199_d_n9, assign49990_e83199_d_n10, assign49990_e83199_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) {
        let assign49990_e83191: f64 = (-locals.var_vgs_noswap);
        let assign49990_e83193: f64 = (assign49990_e83191 - locals.var_egisl_i);
        let assign49990_e83195: f64 = (assign49990_e83193 + locals.var_vfbsdr);
        let assign49990_e83197: f64 = (assign49990_e83195 / locals.var_t0);
        (assign49990_e83197, (-((assign49990_e83195 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign49990_e83195 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsdr_dn5 * locals.var_t0) - (assign49990_e83195 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_dn6) * locals.var_t0) - (assign49990_e83195 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_dn7) * locals.var_t0) - (assign49990_e83195 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_dn8) * locals.var_t0) - (assign49990_e83195 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (-((assign49990_e83195 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgs_noswap_dn10) * locals.var_t0) - (assign49990_e83195 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (-((assign49990_e83195 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign49990_e83199;
        locals.var_t1_dn3 = assign49990_e83199_d_n3;
        locals.var_t1_dn4 = assign49990_e83199_d_n4;
        locals.var_t1_dn5 = assign49990_e83199_d_n5;
        locals.var_t1_dn6 = assign49990_e83199_d_n6;
        locals.var_t1_dn7 = assign49990_e83199_d_n7;
        locals.var_t1_dn8 = assign49990_e83199_d_n8;
        locals.var_t1_dn9 = assign49990_e83199_d_n9;
        locals.var_t1_dn10 = assign49990_e83199_d_n10;
        locals.var_t1_dn11 = assign49990_e83199_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50000_e83222, assign50000_e83222_d_n3, assign50000_e83222_d_n4, assign50000_e83222_d_n5, assign50000_e83222_d_n6, assign50000_e83222_d_n7, assign50000_e83222_d_n8, assign50000_e83222_d_n9, assign50000_e83222_d_n10, assign50000_e83222_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) {
        let assign50000_e83211: f64 = (locals.var_t1 * locals.var_t1);
        let assign50000_e83214: f64 = (4.0 * 0.01);
        let assign50000_e83216: f64 = (assign50000_e83214 * 0.01);
        let assign50000_e83217: f64 = (assign50000_e83211 + assign50000_e83216);
        let assign50000_e83218: f64 = (assign50000_e83217).sqrt();
        let assign50000_e83219: f64 = (locals.var_t1 + assign50000_e83218);
        let assign50000_e83220: f64 = (0.5 * assign50000_e83219);
        (assign50000_e83220, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign50000_e83218)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign50000_e83218)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50000_e83222;
        locals.var_t1_dn3 = assign50000_e83222_d_n3;
        locals.var_t1_dn4 = assign50000_e83222_d_n4;
        locals.var_t1_dn5 = assign50000_e83222_d_n5;
        locals.var_t1_dn6 = assign50000_e83222_d_n6;
        locals.var_t1_dn7 = assign50000_e83222_d_n7;
        locals.var_t1_dn8 = assign50000_e83222_d_n8;
        locals.var_t1_dn9 = assign50000_e83222_d_n9;
        locals.var_t1_dn10 = assign50000_e83222_d_n10;
        locals.var_t1_dn11 = assign50000_e83222_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50010_e83236, assign50010_e83236_d_n3, assign50010_e83236_d_n4, assign50010_e83236_d_n5, assign50010_e83236_d_n6, assign50010_e83236_d_n7, assign50010_e83236_d_n8, assign50010_e83236_d_n9, assign50010_e83236_d_n10, assign50010_e83236_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) {
        let assign50010_e83233: f64 = (locals.var_t1 + 0.001);
        let assign50010_e83234: f64 = (locals.var_bgisl_t / assign50010_e83233);
        (assign50010_e83234, (-((locals.var_bgisl_t * locals.var_t1_dn3) / (assign50010_e83233 * assign50010_e83233))), (((locals.var_bgisl_t_dn4 * assign50010_e83233) - (locals.var_bgisl_t * locals.var_t1_dn4)) / (assign50010_e83233 * assign50010_e83233)), (((locals.var_bgisl_t_dn5 * assign50010_e83233) - (locals.var_bgisl_t * locals.var_t1_dn5)) / (assign50010_e83233 * assign50010_e83233)), (-((locals.var_bgisl_t * locals.var_t1_dn6) / (assign50010_e83233 * assign50010_e83233))), (-((locals.var_bgisl_t * locals.var_t1_dn7) / (assign50010_e83233 * assign50010_e83233))), (-((locals.var_bgisl_t * locals.var_t1_dn8) / (assign50010_e83233 * assign50010_e83233))), (-((locals.var_bgisl_t * locals.var_t1_dn9) / (assign50010_e83233 * assign50010_e83233))), (-((locals.var_bgisl_t * locals.var_t1_dn10) / (assign50010_e83233 * assign50010_e83233))), (-((locals.var_bgisl_t * locals.var_t1_dn11) / (assign50010_e83233 * assign50010_e83233))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign50010_e83236;
        locals.var_t2_dn3 = assign50010_e83236_d_n3;
        locals.var_t2_dn4 = assign50010_e83236_d_n4;
        locals.var_t2_dn5 = assign50010_e83236_d_n5;
        locals.var_t2_dn6 = assign50010_e83236_d_n6;
        locals.var_t2_dn7 = assign50010_e83236_d_n7;
        locals.var_t2_dn8 = assign50010_e83236_d_n8;
        locals.var_t2_dn9 = assign50010_e83236_d_n9;
        locals.var_t2_dn10 = assign50010_e83236_d_n10;
        locals.var_t2_dn11 = assign50010_e83236_d_n11;
        locals.var_t2_rv = 0.0;

        let assign50020_e83239: f64 = if locals.var_cgisl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard763 = assign50020_e83239;
        locals.var_guard763_rv = 0.0;

        let (assign50030_e83255, assign50030_e83255_d_n3, assign50030_e83255_d_n4, assign50030_e83255_d_n5, assign50030_e83255_d_n6, assign50030_e83255_d_n7, assign50030_e83255_d_n8, assign50030_e83255_d_n9, assign50030_e83255_d_n10, assign50030_e83255_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) && (locals.var_guard763 != 0.0)) {
        let assign50030_e83251: f64 = (locals.var_vsb_noswap * locals.var_vsb_noswap);
        let assign50030_e83253: f64 = (assign50030_e83251 * locals.var_vsb_noswap);
        (assign50030_e83253, 0.0, 0.0, 0.0, ((((locals.var_vsb_noswap_dn6 * locals.var_vsb_noswap) + (locals.var_vsb_noswap * locals.var_vsb_noswap_dn6)) * locals.var_vsb_noswap) + (assign50030_e83251 * locals.var_vsb_noswap_dn6)), ((((locals.var_vsb_noswap_dn7 * locals.var_vsb_noswap) + (locals.var_vsb_noswap * locals.var_vsb_noswap_dn7)) * locals.var_vsb_noswap) + (assign50030_e83251 * locals.var_vsb_noswap_dn7)), 0.0, 0.0, ((((locals.var_vsb_noswap_dn10 * locals.var_vsb_noswap) + (locals.var_vsb_noswap * locals.var_vsb_noswap_dn10)) * locals.var_vsb_noswap) + (assign50030_e83251 * locals.var_vsb_noswap_dn10)), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50030_e83255;
        locals.var_t3_dn3 = assign50030_e83255_d_n3;
        locals.var_t3_dn4 = assign50030_e83255_d_n4;
        locals.var_t3_dn5 = assign50030_e83255_d_n5;
        locals.var_t3_dn6 = assign50030_e83255_d_n6;
        locals.var_t3_dn7 = assign50030_e83255_d_n7;
        locals.var_t3_dn8 = assign50030_e83255_d_n8;
        locals.var_t3_dn9 = assign50030_e83255_d_n9;
        locals.var_t3_dn10 = assign50030_e83255_d_n10;
        locals.var_t3_dn11 = assign50030_e83255_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign50040_e83272, assign50040_e83272_d_n3, assign50040_e83272_d_n4, assign50040_e83272_d_n5, assign50040_e83272_d_n6, assign50040_e83272_d_n7, assign50040_e83272_d_n8, assign50040_e83272_d_n9, assign50040_e83272_d_n10, assign50040_e83272_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) && (locals.var_guard763 != 0.0)) {
        let assign50040_e83267: f64 = (locals.var_t3).abs();
        let assign50040_e83268: f64 = (locals.var_cgisl_i + assign50040_e83267);
        let assign50040_e83270: f64 = (assign50040_e83268 + 0.0001);
        (assign50040_e83270, if locals.var_t3 >= 0.0 { locals.var_t3_dn3 } else { (-locals.var_t3_dn3) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn4 } else { (-locals.var_t3_dn4) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn5 } else { (-locals.var_t3_dn5) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn6 } else { (-locals.var_t3_dn6) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn7 } else { (-locals.var_t3_dn7) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn8 } else { (-locals.var_t3_dn8) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn9 } else { (-locals.var_t3_dn9) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn10 } else { (-locals.var_t3_dn10) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn11 } else { (-locals.var_t3_dn11) },)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign50040_e83272;
        locals.var_t4_dn3 = assign50040_e83272_d_n3;
        locals.var_t4_dn4 = assign50040_e83272_d_n4;
        locals.var_t4_dn5 = assign50040_e83272_d_n5;
        locals.var_t4_dn6 = assign50040_e83272_d_n6;
        locals.var_t4_dn7 = assign50040_e83272_d_n7;
        locals.var_t4_dn8 = assign50040_e83272_d_n8;
        locals.var_t4_dn9 = assign50040_e83272_d_n9;
        locals.var_t4_dn10 = assign50040_e83272_d_n10;
        locals.var_t4_dn11 = assign50040_e83272_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign50050_e83305, assign50050_e83305_d_n3, assign50050_e83305_d_n4, assign50050_e83305_d_n5, assign50050_e83305_d_n6, assign50050_e83305_d_n7, assign50050_e83305_d_n8, assign50050_e83305_d_n9, assign50050_e83305_d_n10, assign50050_e83305_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) && (locals.var_guard763 != 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_t4;
        let assign50050_e83285: f64 = (locals.var_t3 * __rspice_inv_cse_1);
        let assign50050_e83288: f64 = (locals.var_t3 * __rspice_inv_cse_1);
        let assign50050_e83291: f64 = (locals.var_t3 * __rspice_inv_cse_1);
        let assign50050_e83292: f64 = (assign50050_e83288 * assign50050_e83291);
        let assign50050_e83295: f64 = (4.0 * 1e-6);
        let assign50050_e83297: f64 = (assign50050_e83295 * 1e-6);
        let assign50050_e83298: f64 = (assign50050_e83292 + assign50050_e83297);
        let assign50050_e83299: f64 = (assign50050_e83298).sqrt();
        let assign50050_e83300: f64 = (assign50050_e83285 + assign50050_e83299);
        let assign50050_e83301: f64 = (0.5 * assign50050_e83300);
        let assign50050_e83303: f64 = (assign50050_e83301 - 1e-6);
        (assign50050_e83303, (0.5 * ((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))), (0.5 * ((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * assign50050_e83291) + (assign50050_e83288 * (((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign50050_e83299)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign50050_e83305;
        locals.var_t5_dn3 = assign50050_e83305_d_n3;
        locals.var_t5_dn4 = assign50050_e83305_d_n4;
        locals.var_t5_dn5 = assign50050_e83305_d_n5;
        locals.var_t5_dn6 = assign50050_e83305_d_n6;
        locals.var_t5_dn7 = assign50050_e83305_d_n7;
        locals.var_t5_dn8 = assign50050_e83305_d_n8;
        locals.var_t5_dn9 = assign50050_e83305_d_n9;
        locals.var_t5_dn10 = assign50050_e83305_d_n10;
        locals.var_t5_dn11 = assign50050_e83305_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign50060_e83318, assign50060_e83318_d_n3, assign50060_e83318_d_n4, assign50060_e83318_d_n5, assign50060_e83318_d_n6, assign50060_e83318_d_n7, assign50060_e83318_d_n8, assign50060_e83318_d_n9, assign50060_e83318_d_n10, assign50060_e83318_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) && (locals.var_guard763 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign50060_e83318;
        locals.var_t5_dn3 = assign50060_e83318_d_n3;
        locals.var_t5_dn4 = assign50060_e83318_d_n4;
        locals.var_t5_dn5 = assign50060_e83318_d_n5;
        locals.var_t5_dn6 = assign50060_e83318_d_n6;
        locals.var_t5_dn7 = assign50060_e83318_d_n7;
        locals.var_t5_dn8 = assign50060_e83318_d_n8;
        locals.var_t5_dn9 = assign50060_e83318_d_n9;
        locals.var_t5_dn10 = assign50060_e83318_d_n10;
        locals.var_t5_dn11 = assign50060_e83318_d_n11;
        locals.var_t5_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_174(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign50070_e83338, assign50070_e83338_d_n3, assign50070_e83338_d_n4, assign50070_e83338_d_n5, assign50070_e83338_d_n6, assign50070_e83338_d_n7, assign50070_e83338_d_n8, assign50070_e83338_d_n9, assign50070_e83338_d_n10, assign50070_e83338_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 != 0.0)) && (locals.var_guard762 == 0.0)) {
        let assign50070_e83328: f64 = (locals.var_agisl_i * locals.var_wdios);
        let assign50070_e83330: f64 = (assign50070_e83328 * locals.var_t1);
        let assign50070_e83332: f64 = (-locals.var_t2);
        let assign50070_e83333: f64 = { let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50070_e83334: f64 = (assign50070_e83330 * assign50070_e83333);
        let assign50070_e83336: f64 = (assign50070_e83334 * locals.var_t5);
        (assign50070_e83336, (((((assign50070_e83328 * locals.var_t1_dn3) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn3)), (((((assign50070_e83328 * locals.var_t1_dn4) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn4)), (((((assign50070_e83328 * locals.var_t1_dn5) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn5)), (((((assign50070_e83328 * locals.var_t1_dn6) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn6)), (((((assign50070_e83328 * locals.var_t1_dn7) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn7)), (((((assign50070_e83328 * locals.var_t1_dn8) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn8)), (((((assign50070_e83328 * locals.var_t1_dn9) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn9)), (((((assign50070_e83328 * locals.var_t1_dn10) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn10)), (((((assign50070_e83328 * locals.var_t1_dn11) * assign50070_e83333) + (assign50070_e83330 * ({ let limited_exp_arg = assign50070_e83332; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * locals.var_t5) + (assign50070_e83334 * locals.var_t5_dn11)),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign50070_e83338;
        locals.var_t6_dn3 = assign50070_e83338_d_n3;
        locals.var_t6_dn4 = assign50070_e83338_d_n4;
        locals.var_t6_dn5 = assign50070_e83338_d_n5;
        locals.var_t6_dn6 = assign50070_e83338_d_n6;
        locals.var_t6_dn7 = assign50070_e83338_d_n7;
        locals.var_t6_dn8 = assign50070_e83338_d_n8;
        locals.var_t6_dn9 = assign50070_e83338_d_n9;
        locals.var_t6_dn10 = assign50070_e83338_d_n10;
        locals.var_t6_dn11 = assign50070_e83338_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign50090_e83355, assign50090_e83355_d_n3, assign50090_e83355_d_n4, assign50090_e83355_d_n5, assign50090_e83355_d_n6, assign50090_e83355_d_n7, assign50090_e83355_d_n8, assign50090_e83355_d_n9, assign50090_e83355_d_n10, assign50090_e83355_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) {
        let assign50090_e83353: f64 = (locals.var_epsratio * p.p76);
        (assign50090_e83353, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50090_e83355;
        locals.var_t0_dn3 = assign50090_e83355_d_n3;
        locals.var_t0_dn4 = assign50090_e83355_d_n4;
        locals.var_t0_dn5 = assign50090_e83355_d_n5;
        locals.var_t0_dn6 = assign50090_e83355_d_n6;
        locals.var_t0_dn7 = assign50090_e83355_d_n7;
        locals.var_t0_dn8 = assign50090_e83355_d_n8;
        locals.var_t0_dn9 = assign50090_e83355_d_n9;
        locals.var_t0_dn10 = assign50090_e83355_d_n10;
        locals.var_t0_dn11 = assign50090_e83355_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign50100_e83367, assign50100_e83367_d_n6, assign50100_e83367_d_n7, assign50100_e83367_d_n8, assign50100_e83367_d_n10,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) {
        let assign50100_e83363: f64 = (locals.var_rgisl_i * locals.var_vg);
        let assign50100_e83365: f64 = (assign50100_e83363 - locals.var_vd);
        (assign50100_e83365, (-locals.var_vd_dn6), (-locals.var_vd_dn7), (locals.var_rgisl_i * locals.var_vg_dn8), ((locals.var_rgisl_i * locals.var_vg_dn10) - locals.var_vd_dn10),)
    } else {
        (locals.var_vgd_noswap_1, locals.var_vgd_noswap_1_dn6, locals.var_vgd_noswap_1_dn7, locals.var_vgd_noswap_1_dn8, locals.var_vgd_noswap_1_dn10,)
    }
};
        locals.var_vgd_noswap_1 = assign50100_e83367;
        locals.var_vgd_noswap_1_dn6 = assign50100_e83367_d_n6;
        locals.var_vgd_noswap_1_dn7 = assign50100_e83367_d_n7;
        locals.var_vgd_noswap_1_dn8 = assign50100_e83367_d_n8;
        locals.var_vgd_noswap_1_dn10 = assign50100_e83367_d_n10;
        locals.var_vgd_noswap_1_rv = 0.0;

        let (assign50110_e83379, assign50110_e83379_d_n6, assign50110_e83379_d_n7, assign50110_e83379_d_n8, assign50110_e83379_d_n10,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) {
        let assign50110_e83375: f64 = (locals.var_rgidl_i * locals.var_vg);
        let assign50110_e83377: f64 = (assign50110_e83375 - locals.var_vs);
        (assign50110_e83377, (-locals.var_vs_dn6), (-locals.var_vs_dn7), (locals.var_rgidl_i * locals.var_vg_dn8), ((locals.var_rgidl_i * locals.var_vg_dn10) - locals.var_vs_dn10),)
    } else {
        (locals.var_vgs_noswap_1, locals.var_vgs_noswap_1_dn6, locals.var_vgs_noswap_1_dn7, locals.var_vgs_noswap_1_dn8, locals.var_vgs_noswap_1_dn10,)
    }
};
        locals.var_vgs_noswap_1 = assign50110_e83379;
        locals.var_vgs_noswap_1_dn6 = assign50110_e83379_d_n6;
        locals.var_vgs_noswap_1_dn7 = assign50110_e83379_d_n7;
        locals.var_vgs_noswap_1_dn8 = assign50110_e83379_d_n8;
        locals.var_vgs_noswap_1_dn10 = assign50110_e83379_d_n10;
        locals.var_vgs_noswap_1_rv = 0.0;

        let (assign50120_e83389, assign50120_e83389_d_n3, assign50120_e83389_d_n4, assign50120_e83389_d_n5, assign50120_e83389_d_n6, assign50120_e83389_d_n7, assign50120_e83389_d_n8, assign50120_e83389_d_n9, assign50120_e83389_d_n10, assign50120_e83389_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) {
        let assign50120_e83387: f64 = (locals.var_vgs_noswap - locals.var_vfbsdr);
        (assign50120_e83387, 0.0, (-locals.var_vfbsdr_dn4), (-locals.var_vfbsdr_dn5), locals.var_vgs_noswap_dn6, locals.var_vgs_noswap_dn7, locals.var_vgs_noswap_dn8, 0.0, locals.var_vgs_noswap_dn10, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign50120_e83389;
        locals.var_t2_dn3 = assign50120_e83389_d_n3;
        locals.var_t2_dn4 = assign50120_e83389_d_n4;
        locals.var_t2_dn5 = assign50120_e83389_d_n5;
        locals.var_t2_dn6 = assign50120_e83389_d_n6;
        locals.var_t2_dn7 = assign50120_e83389_d_n7;
        locals.var_t2_dn8 = assign50120_e83389_d_n8;
        locals.var_t2_dn9 = assign50120_e83389_d_n9;
        locals.var_t2_dn10 = assign50120_e83389_d_n10;
        locals.var_t2_dn11 = assign50120_e83389_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign50130_e83402, assign50130_e83402_d_n3, assign50130_e83402_d_n4, assign50130_e83402_d_n5, assign50130_e83402_d_n6, assign50130_e83402_d_n7, assign50130_e83402_d_n8, assign50130_e83402_d_n9, assign50130_e83402_d_n10, assign50130_e83402_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) {
        let assign50130_e83397: f64 = (locals.var_t2 * locals.var_t2);
        let assign50130_e83399: f64 = (assign50130_e83397 + 0.0001);
        let assign50130_e83400: f64 = (assign50130_e83399).sqrt();
        (assign50130_e83400, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign50130_e83400)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign50130_e83400)),)
    } else {
        (locals.var_vgs_eff, locals.var_vgs_eff_dn3, locals.var_vgs_eff_dn4, locals.var_vgs_eff_dn5, locals.var_vgs_eff_dn6, locals.var_vgs_eff_dn7, locals.var_vgs_eff_dn8, locals.var_vgs_eff_dn9, locals.var_vgs_eff_dn10, locals.var_vgs_eff_dn11,)
    }
};
        locals.var_vgs_eff = assign50130_e83402;
        locals.var_vgs_eff_dn3 = assign50130_e83402_d_n3;
        locals.var_vgs_eff_dn4 = assign50130_e83402_d_n4;
        locals.var_vgs_eff_dn5 = assign50130_e83402_d_n5;
        locals.var_vgs_eff_dn6 = assign50130_e83402_d_n6;
        locals.var_vgs_eff_dn7 = assign50130_e83402_d_n7;
        locals.var_vgs_eff_dn8 = assign50130_e83402_d_n8;
        locals.var_vgs_eff_dn9 = assign50130_e83402_d_n9;
        locals.var_vgs_eff_dn10 = assign50130_e83402_d_n10;
        locals.var_vgs_eff_dn11 = assign50130_e83402_d_n11;
        locals.var_vgs_eff_rv = 0.0;

        let assign50140_e83409: f64 = if ((locals.var_agidl_i <= 0.0) || (locals.var_bgidl_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard764 = assign50140_e83409;
        locals.var_guard764_rv = 0.0;

        let (assign50150_e83419, assign50150_e83419_d_n3, assign50150_e83419_d_n4, assign50150_e83419_d_n5, assign50150_e83419_d_n6, assign50150_e83419_d_n7, assign50150_e83419_d_n8, assign50150_e83419_d_n9, assign50150_e83419_d_n10, assign50150_e83419_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign50150_e83419;
        locals.var_t6_dn3 = assign50150_e83419_d_n3;
        locals.var_t6_dn4 = assign50150_e83419_d_n4;
        locals.var_t6_dn5 = assign50150_e83419_d_n5;
        locals.var_t6_dn6 = assign50150_e83419_d_n6;
        locals.var_t6_dn7 = assign50150_e83419_d_n7;
        locals.var_t6_dn8 = assign50150_e83419_d_n8;
        locals.var_t6_dn9 = assign50150_e83419_d_n9;
        locals.var_t6_dn10 = assign50150_e83419_d_n10;
        locals.var_t6_dn11 = assign50150_e83419_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign50160_e83437, assign50160_e83437_d_n3, assign50160_e83437_d_n4, assign50160_e83437_d_n5, assign50160_e83437_d_n6, assign50160_e83437_d_n7, assign50160_e83437_d_n8, assign50160_e83437_d_n9, assign50160_e83437_d_n10, assign50160_e83437_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) {
        let assign50160_e83429: f64 = (-locals.var_vgd_noswap_1);
        let assign50160_e83431: f64 = (assign50160_e83429 - locals.var_egidl_i);
        let assign50160_e83433: f64 = (assign50160_e83431 + locals.var_vfbsdr);
        let assign50160_e83435: f64 = (assign50160_e83433 / locals.var_t0);
        (assign50160_e83435, (-((assign50160_e83433 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign50160_e83433 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsdr_dn5 * locals.var_t0) - (assign50160_e83433 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_1_dn6) * locals.var_t0) - (assign50160_e83433 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_1_dn7) * locals.var_t0) - (assign50160_e83433 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_1_dn8) * locals.var_t0) - (assign50160_e83433 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (-((assign50160_e83433 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgd_noswap_1_dn10) * locals.var_t0) - (assign50160_e83433 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (-((assign50160_e83433 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50160_e83437;
        locals.var_t1_dn3 = assign50160_e83437_d_n3;
        locals.var_t1_dn4 = assign50160_e83437_d_n4;
        locals.var_t1_dn5 = assign50160_e83437_d_n5;
        locals.var_t1_dn6 = assign50160_e83437_d_n6;
        locals.var_t1_dn7 = assign50160_e83437_d_n7;
        locals.var_t1_dn8 = assign50160_e83437_d_n8;
        locals.var_t1_dn9 = assign50160_e83437_d_n9;
        locals.var_t1_dn10 = assign50160_e83437_d_n10;
        locals.var_t1_dn11 = assign50160_e83437_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50170_e83461, assign50170_e83461_d_n3, assign50170_e83461_d_n4, assign50170_e83461_d_n5, assign50170_e83461_d_n6, assign50170_e83461_d_n7, assign50170_e83461_d_n8, assign50170_e83461_d_n9, assign50170_e83461_d_n10, assign50170_e83461_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) {
        let assign50170_e83450: f64 = (locals.var_t1 * locals.var_t1);
        let assign50170_e83453: f64 = (4.0 * 0.01);
        let assign50170_e83455: f64 = (assign50170_e83453 * 0.01);
        let assign50170_e83456: f64 = (assign50170_e83450 + assign50170_e83455);
        let assign50170_e83457: f64 = (assign50170_e83456).sqrt();
        let assign50170_e83458: f64 = (locals.var_t1 + assign50170_e83457);
        let assign50170_e83459: f64 = (0.5 * assign50170_e83458);
        (assign50170_e83459, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign50170_e83457)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign50170_e83457)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50170_e83461;
        locals.var_t1_dn3 = assign50170_e83461_d_n3;
        locals.var_t1_dn4 = assign50170_e83461_d_n4;
        locals.var_t1_dn5 = assign50170_e83461_d_n5;
        locals.var_t1_dn6 = assign50170_e83461_d_n6;
        locals.var_t1_dn7 = assign50170_e83461_d_n7;
        locals.var_t1_dn8 = assign50170_e83461_d_n8;
        locals.var_t1_dn9 = assign50170_e83461_d_n9;
        locals.var_t1_dn10 = assign50170_e83461_d_n10;
        locals.var_t1_dn11 = assign50170_e83461_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50180_e83476, assign50180_e83476_d_n3, assign50180_e83476_d_n4, assign50180_e83476_d_n5, assign50180_e83476_d_n6, assign50180_e83476_d_n7, assign50180_e83476_d_n8, assign50180_e83476_d_n9, assign50180_e83476_d_n10, assign50180_e83476_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) {
        let assign50180_e83473: f64 = (locals.var_t1 + 0.001);
        let assign50180_e83474: f64 = (locals.var_bgidl_t / assign50180_e83473);
        (assign50180_e83474, (-((locals.var_bgidl_t * locals.var_t1_dn3) / (assign50180_e83473 * assign50180_e83473))), (((locals.var_bgidl_t_dn4 * assign50180_e83473) - (locals.var_bgidl_t * locals.var_t1_dn4)) / (assign50180_e83473 * assign50180_e83473)), (((locals.var_bgidl_t_dn5 * assign50180_e83473) - (locals.var_bgidl_t * locals.var_t1_dn5)) / (assign50180_e83473 * assign50180_e83473)), (-((locals.var_bgidl_t * locals.var_t1_dn6) / (assign50180_e83473 * assign50180_e83473))), (-((locals.var_bgidl_t * locals.var_t1_dn7) / (assign50180_e83473 * assign50180_e83473))), (-((locals.var_bgidl_t * locals.var_t1_dn8) / (assign50180_e83473 * assign50180_e83473))), (-((locals.var_bgidl_t * locals.var_t1_dn9) / (assign50180_e83473 * assign50180_e83473))), (-((locals.var_bgidl_t * locals.var_t1_dn10) / (assign50180_e83473 * assign50180_e83473))), (-((locals.var_bgidl_t * locals.var_t1_dn11) / (assign50180_e83473 * assign50180_e83473))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign50180_e83476;
        locals.var_t2_dn3 = assign50180_e83476_d_n3;
        locals.var_t2_dn4 = assign50180_e83476_d_n4;
        locals.var_t2_dn5 = assign50180_e83476_d_n5;
        locals.var_t2_dn6 = assign50180_e83476_d_n6;
        locals.var_t2_dn7 = assign50180_e83476_d_n7;
        locals.var_t2_dn8 = assign50180_e83476_d_n8;
        locals.var_t2_dn9 = assign50180_e83476_d_n9;
        locals.var_t2_dn10 = assign50180_e83476_d_n10;
        locals.var_t2_dn11 = assign50180_e83476_d_n11;
        locals.var_t2_rv = 0.0;

        let assign50190_e83479: f64 = if locals.var_kgidl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard765 = assign50190_e83479;
        locals.var_guard765_rv = 0.0;

        let (assign50200_e83495, assign50200_e83495_d_n3, assign50200_e83495_d_n4, assign50200_e83495_d_n5, assign50200_e83495_d_n6, assign50200_e83495_d_n7, assign50200_e83495_d_n8, assign50200_e83495_d_n9, assign50200_e83495_d_n10, assign50200_e83495_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) && (locals.var_guard765 != 0.0)) {
        let assign50200_e83491: f64 = (-locals.var_vdb_noswap);
        let assign50200_e83493: f64 = (assign50200_e83491 - locals.var_fgidl_i);
        (assign50200_e83493, 0.0, 0.0, 0.0, (-locals.var_vdb_noswap_dn6), (-locals.var_vdb_noswap_dn7), 0.0, 0.0, (-locals.var_vdb_noswap_dn10), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50200_e83495;
        locals.var_t3_dn3 = assign50200_e83495_d_n3;
        locals.var_t3_dn4 = assign50200_e83495_d_n4;
        locals.var_t3_dn5 = assign50200_e83495_d_n5;
        locals.var_t3_dn6 = assign50200_e83495_d_n6;
        locals.var_t3_dn7 = assign50200_e83495_d_n7;
        locals.var_t3_dn8 = assign50200_e83495_d_n8;
        locals.var_t3_dn9 = assign50200_e83495_d_n9;
        locals.var_t3_dn10 = assign50200_e83495_d_n10;
        locals.var_t3_dn11 = assign50200_e83495_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign50210_e83510, assign50210_e83510_d_n3, assign50210_e83510_d_n4, assign50210_e83510_d_n5, assign50210_e83510_d_n6, assign50210_e83510_d_n7, assign50210_e83510_d_n8, assign50210_e83510_d_n9, assign50210_e83510_d_n10, assign50210_e83510_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) && (locals.var_guard765 != 0.0)) {
        let assign50210_e83508: f64 = (locals.var_t3 + 0.0001);
        (assign50210_e83508, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign50210_e83510;
        locals.var_t4_dn3 = assign50210_e83510_d_n3;
        locals.var_t4_dn4 = assign50210_e83510_d_n4;
        locals.var_t4_dn5 = assign50210_e83510_d_n5;
        locals.var_t4_dn6 = assign50210_e83510_d_n6;
        locals.var_t4_dn7 = assign50210_e83510_d_n7;
        locals.var_t4_dn8 = assign50210_e83510_d_n8;
        locals.var_t4_dn9 = assign50210_e83510_d_n9;
        locals.var_t4_dn10 = assign50210_e83510_d_n10;
        locals.var_t4_dn11 = assign50210_e83510_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign50220_e83544, assign50220_e83544_d_n3, assign50220_e83544_d_n4, assign50220_e83544_d_n5, assign50220_e83544_d_n6, assign50220_e83544_d_n7, assign50220_e83544_d_n8, assign50220_e83544_d_n9, assign50220_e83544_d_n10, assign50220_e83544_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) && (locals.var_guard765 != 0.0)) {
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t4;
        let assign50220_e83524: f64 = (locals.var_kgidl_i * __rspice_inv_cse_0);
        let assign50220_e83527: f64 = (locals.var_kgidl_i * __rspice_inv_cse_0);
        let assign50220_e83530: f64 = (locals.var_kgidl_i * __rspice_inv_cse_0);
        let assign50220_e83531: f64 = (assign50220_e83527 * assign50220_e83530);
        let assign50220_e83534: f64 = (4.0 * 1e-6);
        let assign50220_e83536: f64 = (assign50220_e83534 * 1e-6);
        let assign50220_e83537: f64 = (assign50220_e83531 + assign50220_e83536);
        let assign50220_e83538: f64 = (assign50220_e83537).sqrt();
        let assign50220_e83539: f64 = (assign50220_e83524 + assign50220_e83538);
        let assign50220_e83540: f64 = (0.5 * assign50220_e83539);
        let assign50220_e83542: f64 = (assign50220_e83540 - 1e-6);
        (assign50220_e83542, (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))), (0.5 * ((-((locals.var_kgidl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgidl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) * assign50220_e83530) + (assign50220_e83527 * (-((locals.var_kgidl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50220_e83538)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign50220_e83544;
        locals.var_t5_dn3 = assign50220_e83544_d_n3;
        locals.var_t5_dn4 = assign50220_e83544_d_n4;
        locals.var_t5_dn5 = assign50220_e83544_d_n5;
        locals.var_t5_dn6 = assign50220_e83544_d_n6;
        locals.var_t5_dn7 = assign50220_e83544_d_n7;
        locals.var_t5_dn8 = assign50220_e83544_d_n8;
        locals.var_t5_dn9 = assign50220_e83544_d_n9;
        locals.var_t5_dn10 = assign50220_e83544_d_n10;
        locals.var_t5_dn11 = assign50220_e83544_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign50230_e83558, assign50230_e83558_d_n3, assign50230_e83558_d_n4, assign50230_e83558_d_n5, assign50230_e83558_d_n6, assign50230_e83558_d_n7, assign50230_e83558_d_n8, assign50230_e83558_d_n9, assign50230_e83558_d_n10, assign50230_e83558_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) && (locals.var_guard765 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign50230_e83558;
        locals.var_t5_dn3 = assign50230_e83558_d_n3;
        locals.var_t5_dn4 = assign50230_e83558_d_n4;
        locals.var_t5_dn5 = assign50230_e83558_d_n5;
        locals.var_t5_dn6 = assign50230_e83558_d_n6;
        locals.var_t5_dn7 = assign50230_e83558_d_n7;
        locals.var_t5_dn8 = assign50230_e83558_d_n8;
        locals.var_t5_dn9 = assign50230_e83558_d_n9;
        locals.var_t5_dn10 = assign50230_e83558_d_n10;
        locals.var_t5_dn11 = assign50230_e83558_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign50240_e83580, assign50240_e83580_d_n3, assign50240_e83580_d_n4, assign50240_e83580_d_n5, assign50240_e83580_d_n6, assign50240_e83580_d_n7, assign50240_e83580_d_n8, assign50240_e83580_d_n9, assign50240_e83580_d_n10, assign50240_e83580_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard764 == 0.0)) {
        let assign50240_e83569: f64 = (locals.var_agidl_i * locals.var_wdiod);
        let assign50240_e83571: f64 = (assign50240_e83569 * locals.var_t1);
        let assign50240_e83573: f64 = (-locals.var_t2);
        let assign50240_e83574: f64 = { let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50240_e83575: f64 = (assign50240_e83571 * assign50240_e83574);
        let assign50240_e83577: f64 = { let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50240_e83578: f64 = (assign50240_e83575 * assign50240_e83577);
        (assign50240_e83578, (((((assign50240_e83569 * locals.var_t1_dn3) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn3))), (((((assign50240_e83569 * locals.var_t1_dn4) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn4))), (((((assign50240_e83569 * locals.var_t1_dn5) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn5))), (((((assign50240_e83569 * locals.var_t1_dn6) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn6))), (((((assign50240_e83569 * locals.var_t1_dn7) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn7))), (((((assign50240_e83569 * locals.var_t1_dn8) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn8))), (((((assign50240_e83569 * locals.var_t1_dn9) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn9))), (((((assign50240_e83569 * locals.var_t1_dn10) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn10))), (((((assign50240_e83569 * locals.var_t1_dn11) * assign50240_e83574) + (assign50240_e83571 * ({ let limited_exp_arg = assign50240_e83573; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * assign50240_e83577) + (assign50240_e83575 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn11))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign50240_e83580;
        locals.var_t6_dn3 = assign50240_e83580_d_n3;
        locals.var_t6_dn4 = assign50240_e83580_d_n4;
        locals.var_t6_dn5 = assign50240_e83580_d_n5;
        locals.var_t6_dn6 = assign50240_e83580_d_n6;
        locals.var_t6_dn7 = assign50240_e83580_d_n7;
        locals.var_t6_dn8 = assign50240_e83580_d_n8;
        locals.var_t6_dn9 = assign50240_e83580_d_n9;
        locals.var_t6_dn10 = assign50240_e83580_d_n10;
        locals.var_t6_dn11 = assign50240_e83580_d_n11;
        locals.var_t6_rv = 0.0;

        let assign50260_e83595: f64 = if ((locals.var_agisl_i <= 0.0) || (locals.var_bgisl_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard766 = assign50260_e83595;
        locals.var_guard766_rv = 0.0;

        let (assign50270_e83605, assign50270_e83605_d_n3, assign50270_e83605_d_n4, assign50270_e83605_d_n5, assign50270_e83605_d_n6, assign50270_e83605_d_n7, assign50270_e83605_d_n8, assign50270_e83605_d_n9, assign50270_e83605_d_n10, assign50270_e83605_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign50270_e83605;
        locals.var_t6_dn3 = assign50270_e83605_d_n3;
        locals.var_t6_dn4 = assign50270_e83605_d_n4;
        locals.var_t6_dn5 = assign50270_e83605_d_n5;
        locals.var_t6_dn6 = assign50270_e83605_d_n6;
        locals.var_t6_dn7 = assign50270_e83605_d_n7;
        locals.var_t6_dn8 = assign50270_e83605_d_n8;
        locals.var_t6_dn9 = assign50270_e83605_d_n9;
        locals.var_t6_dn10 = assign50270_e83605_d_n10;
        locals.var_t6_dn11 = assign50270_e83605_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign50280_e83623, assign50280_e83623_d_n3, assign50280_e83623_d_n4, assign50280_e83623_d_n5, assign50280_e83623_d_n6, assign50280_e83623_d_n7, assign50280_e83623_d_n8, assign50280_e83623_d_n9, assign50280_e83623_d_n10, assign50280_e83623_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) {
        let assign50280_e83615: f64 = (-locals.var_vgs_noswap_1);
        let assign50280_e83617: f64 = (assign50280_e83615 - locals.var_egisl_i);
        let assign50280_e83619: f64 = (assign50280_e83617 + locals.var_vfbsdr);
        let assign50280_e83621: f64 = (assign50280_e83619 / locals.var_t0);
        (assign50280_e83621, (-((assign50280_e83619 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign50280_e83619 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_vfbsdr_dn5 * locals.var_t0) - (assign50280_e83619 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_1_dn6) * locals.var_t0) - (assign50280_e83619 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_1_dn7) * locals.var_t0) - (assign50280_e83619 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_1_dn8) * locals.var_t0) - (assign50280_e83619 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (-((assign50280_e83619 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgs_noswap_1_dn10) * locals.var_t0) - (assign50280_e83619 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (-((assign50280_e83619 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50280_e83623;
        locals.var_t1_dn3 = assign50280_e83623_d_n3;
        locals.var_t1_dn4 = assign50280_e83623_d_n4;
        locals.var_t1_dn5 = assign50280_e83623_d_n5;
        locals.var_t1_dn6 = assign50280_e83623_d_n6;
        locals.var_t1_dn7 = assign50280_e83623_d_n7;
        locals.var_t1_dn8 = assign50280_e83623_d_n8;
        locals.var_t1_dn9 = assign50280_e83623_d_n9;
        locals.var_t1_dn10 = assign50280_e83623_d_n10;
        locals.var_t1_dn11 = assign50280_e83623_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50290_e83647, assign50290_e83647_d_n3, assign50290_e83647_d_n4, assign50290_e83647_d_n5, assign50290_e83647_d_n6, assign50290_e83647_d_n7, assign50290_e83647_d_n8, assign50290_e83647_d_n9, assign50290_e83647_d_n10, assign50290_e83647_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) {
        let assign50290_e83636: f64 = (locals.var_t1 * locals.var_t1);
        let assign50290_e83639: f64 = (4.0 * 0.01);
        let assign50290_e83641: f64 = (assign50290_e83639 * 0.01);
        let assign50290_e83642: f64 = (assign50290_e83636 + assign50290_e83641);
        let assign50290_e83643: f64 = (assign50290_e83642).sqrt();
        let assign50290_e83644: f64 = (locals.var_t1 + assign50290_e83643);
        let assign50290_e83645: f64 = (0.5 * assign50290_e83644);
        (assign50290_e83645, (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign50290_e83643)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign50290_e83643)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50290_e83647;
        locals.var_t1_dn3 = assign50290_e83647_d_n3;
        locals.var_t1_dn4 = assign50290_e83647_d_n4;
        locals.var_t1_dn5 = assign50290_e83647_d_n5;
        locals.var_t1_dn6 = assign50290_e83647_d_n6;
        locals.var_t1_dn7 = assign50290_e83647_d_n7;
        locals.var_t1_dn8 = assign50290_e83647_d_n8;
        locals.var_t1_dn9 = assign50290_e83647_d_n9;
        locals.var_t1_dn10 = assign50290_e83647_d_n10;
        locals.var_t1_dn11 = assign50290_e83647_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50300_e83662, assign50300_e83662_d_n3, assign50300_e83662_d_n4, assign50300_e83662_d_n5, assign50300_e83662_d_n6, assign50300_e83662_d_n7, assign50300_e83662_d_n8, assign50300_e83662_d_n9, assign50300_e83662_d_n10, assign50300_e83662_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) {
        let assign50300_e83659: f64 = (locals.var_t1 + 0.001);
        let assign50300_e83660: f64 = (locals.var_bgisl_t / assign50300_e83659);
        (assign50300_e83660, (-((locals.var_bgisl_t * locals.var_t1_dn3) / (assign50300_e83659 * assign50300_e83659))), (((locals.var_bgisl_t_dn4 * assign50300_e83659) - (locals.var_bgisl_t * locals.var_t1_dn4)) / (assign50300_e83659 * assign50300_e83659)), (((locals.var_bgisl_t_dn5 * assign50300_e83659) - (locals.var_bgisl_t * locals.var_t1_dn5)) / (assign50300_e83659 * assign50300_e83659)), (-((locals.var_bgisl_t * locals.var_t1_dn6) / (assign50300_e83659 * assign50300_e83659))), (-((locals.var_bgisl_t * locals.var_t1_dn7) / (assign50300_e83659 * assign50300_e83659))), (-((locals.var_bgisl_t * locals.var_t1_dn8) / (assign50300_e83659 * assign50300_e83659))), (-((locals.var_bgisl_t * locals.var_t1_dn9) / (assign50300_e83659 * assign50300_e83659))), (-((locals.var_bgisl_t * locals.var_t1_dn10) / (assign50300_e83659 * assign50300_e83659))), (-((locals.var_bgisl_t * locals.var_t1_dn11) / (assign50300_e83659 * assign50300_e83659))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign50300_e83662;
        locals.var_t2_dn3 = assign50300_e83662_d_n3;
        locals.var_t2_dn4 = assign50300_e83662_d_n4;
        locals.var_t2_dn5 = assign50300_e83662_d_n5;
        locals.var_t2_dn6 = assign50300_e83662_d_n6;
        locals.var_t2_dn7 = assign50300_e83662_d_n7;
        locals.var_t2_dn8 = assign50300_e83662_d_n8;
        locals.var_t2_dn9 = assign50300_e83662_d_n9;
        locals.var_t2_dn10 = assign50300_e83662_d_n10;
        locals.var_t2_dn11 = assign50300_e83662_d_n11;
        locals.var_t2_rv = 0.0;

        let assign50310_e83665: f64 = if locals.var_kgisl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard767 = assign50310_e83665;
        locals.var_guard767_rv = 0.0;

        let (assign50320_e83681, assign50320_e83681_d_n3, assign50320_e83681_d_n4, assign50320_e83681_d_n5, assign50320_e83681_d_n6, assign50320_e83681_d_n7, assign50320_e83681_d_n8, assign50320_e83681_d_n9, assign50320_e83681_d_n10, assign50320_e83681_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) && (locals.var_guard767 != 0.0)) {
        let assign50320_e83677: f64 = (-locals.var_vsb_noswap);
        let assign50320_e83679: f64 = (assign50320_e83677 - locals.var_fgisl_i);
        (assign50320_e83679, 0.0, 0.0, 0.0, (-locals.var_vsb_noswap_dn6), (-locals.var_vsb_noswap_dn7), 0.0, 0.0, (-locals.var_vsb_noswap_dn10), 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50320_e83681;
        locals.var_t3_dn3 = assign50320_e83681_d_n3;
        locals.var_t3_dn4 = assign50320_e83681_d_n4;
        locals.var_t3_dn5 = assign50320_e83681_d_n5;
        locals.var_t3_dn6 = assign50320_e83681_d_n6;
        locals.var_t3_dn7 = assign50320_e83681_d_n7;
        locals.var_t3_dn8 = assign50320_e83681_d_n8;
        locals.var_t3_dn9 = assign50320_e83681_d_n9;
        locals.var_t3_dn10 = assign50320_e83681_d_n10;
        locals.var_t3_dn11 = assign50320_e83681_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign50330_e83696, assign50330_e83696_d_n3, assign50330_e83696_d_n4, assign50330_e83696_d_n5, assign50330_e83696_d_n6, assign50330_e83696_d_n7, assign50330_e83696_d_n8, assign50330_e83696_d_n9, assign50330_e83696_d_n10, assign50330_e83696_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) && (locals.var_guard767 != 0.0)) {
        let assign50330_e83694: f64 = (locals.var_t3 + 0.0001);
        (assign50330_e83694, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign50330_e83696;
        locals.var_t4_dn3 = assign50330_e83696_d_n3;
        locals.var_t4_dn4 = assign50330_e83696_d_n4;
        locals.var_t4_dn5 = assign50330_e83696_d_n5;
        locals.var_t4_dn6 = assign50330_e83696_d_n6;
        locals.var_t4_dn7 = assign50330_e83696_d_n7;
        locals.var_t4_dn8 = assign50330_e83696_d_n8;
        locals.var_t4_dn9 = assign50330_e83696_d_n9;
        locals.var_t4_dn10 = assign50330_e83696_d_n10;
        locals.var_t4_dn11 = assign50330_e83696_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign50340_e83730, assign50340_e83730_d_n3, assign50340_e83730_d_n4, assign50340_e83730_d_n5, assign50340_e83730_d_n6, assign50340_e83730_d_n7, assign50340_e83730_d_n8, assign50340_e83730_d_n9, assign50340_e83730_d_n10, assign50340_e83730_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) && (locals.var_guard767 != 0.0)) {
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_t4;
        let assign50340_e83710: f64 = (locals.var_kgisl_i * __rspice_inv_cse_1);
        let assign50340_e83713: f64 = (locals.var_kgisl_i * __rspice_inv_cse_1);
        let assign50340_e83716: f64 = (locals.var_kgisl_i * __rspice_inv_cse_1);
        let assign50340_e83717: f64 = (assign50340_e83713 * assign50340_e83716);
        let assign50340_e83720: f64 = (4.0 * 1e-6);
        let assign50340_e83722: f64 = (assign50340_e83720 * 1e-6);
        let assign50340_e83723: f64 = (assign50340_e83717 + assign50340_e83722);
        let assign50340_e83724: f64 = (assign50340_e83723).sqrt();
        let assign50340_e83725: f64 = (assign50340_e83710 + assign50340_e83724);
        let assign50340_e83726: f64 = (0.5 * assign50340_e83725);
        let assign50340_e83728: f64 = (assign50340_e83726 - 1e-6);
        (assign50340_e83728, (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn3) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn4) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn5) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn6) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn7) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn8) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn9) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn10) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))), (0.5 * ((-((locals.var_kgisl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) + ((((-((locals.var_kgisl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))) * assign50340_e83716) + (assign50340_e83713 * (-((locals.var_kgisl_i * locals.var_t4_dn11) / (locals.var_t4 * locals.var_t4))))) / (2.0 * assign50340_e83724)))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign50340_e83730;
        locals.var_t5_dn3 = assign50340_e83730_d_n3;
        locals.var_t5_dn4 = assign50340_e83730_d_n4;
        locals.var_t5_dn5 = assign50340_e83730_d_n5;
        locals.var_t5_dn6 = assign50340_e83730_d_n6;
        locals.var_t5_dn7 = assign50340_e83730_d_n7;
        locals.var_t5_dn8 = assign50340_e83730_d_n8;
        locals.var_t5_dn9 = assign50340_e83730_d_n9;
        locals.var_t5_dn10 = assign50340_e83730_d_n10;
        locals.var_t5_dn11 = assign50340_e83730_d_n11;
        locals.var_t5_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_175(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign50350_e83744, assign50350_e83744_d_n3, assign50350_e83744_d_n4, assign50350_e83744_d_n5, assign50350_e83744_d_n6, assign50350_e83744_d_n7, assign50350_e83744_d_n8, assign50350_e83744_d_n9, assign50350_e83744_d_n10, assign50350_e83744_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) && (locals.var_guard767 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign50350_e83744;
        locals.var_t5_dn3 = assign50350_e83744_d_n3;
        locals.var_t5_dn4 = assign50350_e83744_d_n4;
        locals.var_t5_dn5 = assign50350_e83744_d_n5;
        locals.var_t5_dn6 = assign50350_e83744_d_n6;
        locals.var_t5_dn7 = assign50350_e83744_d_n7;
        locals.var_t5_dn8 = assign50350_e83744_d_n8;
        locals.var_t5_dn9 = assign50350_e83744_d_n9;
        locals.var_t5_dn10 = assign50350_e83744_d_n10;
        locals.var_t5_dn11 = assign50350_e83744_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign50360_e83766, assign50360_e83766_d_n3, assign50360_e83766_d_n4, assign50360_e83766_d_n5, assign50360_e83766_d_n6, assign50360_e83766_d_n7, assign50360_e83766_d_n8, assign50360_e83766_d_n9, assign50360_e83766_d_n10, assign50360_e83766_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard759 == 0.0)) && (locals.var_guard766 == 0.0)) {
        let assign50360_e83755: f64 = (locals.var_agisl_i * locals.var_wdios);
        let assign50360_e83757: f64 = (assign50360_e83755 * locals.var_t1);
        let assign50360_e83759: f64 = (-locals.var_t2);
        let assign50360_e83760: f64 = { let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50360_e83761: f64 = (assign50360_e83757 * assign50360_e83760);
        let assign50360_e83763: f64 = { let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign50360_e83764: f64 = (assign50360_e83761 * assign50360_e83763);
        (assign50360_e83764, (((((assign50360_e83755 * locals.var_t1_dn3) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn3))), (((((assign50360_e83755 * locals.var_t1_dn4) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn4))), (((((assign50360_e83755 * locals.var_t1_dn5) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn5))), (((((assign50360_e83755 * locals.var_t1_dn6) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn6))), (((((assign50360_e83755 * locals.var_t1_dn7) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn7))), (((((assign50360_e83755 * locals.var_t1_dn8) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn8))), (((((assign50360_e83755 * locals.var_t1_dn9) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn9))), (((((assign50360_e83755 * locals.var_t1_dn10) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn10))), (((((assign50360_e83755 * locals.var_t1_dn11) * assign50360_e83760) + (assign50360_e83757 * ({ let limited_exp_arg = assign50360_e83759; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * assign50360_e83763) + (assign50360_e83761 * ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn11))),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign50360_e83766;
        locals.var_t6_dn3 = assign50360_e83766_d_n3;
        locals.var_t6_dn4 = assign50360_e83766_d_n4;
        locals.var_t6_dn5 = assign50360_e83766_d_n5;
        locals.var_t6_dn6 = assign50360_e83766_d_n6;
        locals.var_t6_dn7 = assign50360_e83766_d_n7;
        locals.var_t6_dn8 = assign50360_e83766_d_n8;
        locals.var_t6_dn9 = assign50360_e83766_d_n9;
        locals.var_t6_dn10 = assign50360_e83766_d_n10;
        locals.var_t6_dn11 = assign50360_e83766_d_n11;
        locals.var_t6_rv = 0.0;

        let assign50400_e83795: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard768 = assign50400_e83795;
        locals.var_guard768_rv = 0.0;

        let assign50410_e83802: f64 = if ((locals.var_alpha0_i <= 0.0) || (locals.var_beta0_t <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard769 = assign50410_e83802;
        locals.var_guard769_rv = 0.0;

        let assign50430_e83815: f64 = (locals.var_beta0_t / 80.0);
        let assign50430_e83816: f64 = if locals.var_diffvds > assign50430_e83815 { 1.0 } else { 0.0 };
        locals.var_guard770 = assign50430_e83816;
        locals.var_guard770_rv = 0.0;

        let (assign50440_e83831, assign50440_e83831_d_n3, assign50440_e83831_d_n4, assign50440_e83831_d_n5, assign50440_e83831_d_n6, assign50440_e83831_d_n7, assign50440_e83831_d_n8, assign50440_e83831_d_n9, assign50440_e83831_d_n10, assign50440_e83831_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 != 0.0)) && (locals.var_guard769 == 0.0)) && (locals.var_guard770 != 0.0)) {
        let assign50440_e83827: f64 = (-locals.var_beta0_t);
        let assign50440_e83829: f64 = (assign50440_e83827 / locals.var_diffvds);
        (assign50440_e83829, (-((assign50440_e83827 * locals.var_diffvds_dn3) / (locals.var_diffvds * locals.var_diffvds))), ((((-locals.var_beta0_t_dn4) * locals.var_diffvds) - (assign50440_e83827 * locals.var_diffvds_dn4)) / (locals.var_diffvds * locals.var_diffvds)), ((((-locals.var_beta0_t_dn5) * locals.var_diffvds) - (assign50440_e83827 * locals.var_diffvds_dn5)) / (locals.var_diffvds * locals.var_diffvds)), (-((assign50440_e83827 * locals.var_diffvds_dn6) / (locals.var_diffvds * locals.var_diffvds))), (-((assign50440_e83827 * locals.var_diffvds_dn7) / (locals.var_diffvds * locals.var_diffvds))), (-((assign50440_e83827 * locals.var_diffvds_dn8) / (locals.var_diffvds * locals.var_diffvds))), (-((assign50440_e83827 * locals.var_diffvds_dn9) / (locals.var_diffvds * locals.var_diffvds))), (-((assign50440_e83827 * locals.var_diffvds_dn10) / (locals.var_diffvds * locals.var_diffvds))), (-((assign50440_e83827 * locals.var_diffvds_dn11) / (locals.var_diffvds * locals.var_diffvds))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50440_e83831;
        locals.var_t1_dn3 = assign50440_e83831_d_n3;
        locals.var_t1_dn4 = assign50440_e83831_d_n4;
        locals.var_t1_dn5 = assign50440_e83831_d_n5;
        locals.var_t1_dn6 = assign50440_e83831_d_n6;
        locals.var_t1_dn7 = assign50440_e83831_d_n7;
        locals.var_t1_dn8 = assign50440_e83831_d_n8;
        locals.var_t1_dn9 = assign50440_e83831_d_n9;
        locals.var_t1_dn10 = assign50440_e83831_d_n10;
        locals.var_t1_dn11 = assign50440_e83831_d_n11;
        locals.var_t1_rv = 0.0;

        let assign50470_e83876: f64 = if p.p44 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard771 = assign50470_e83876;
        locals.var_guard771_rv = 0.0;

        let assign50480_e83891: f64 = if ((locals.var_alpha0_i <= 0.0) || (((locals.var_beta2_i == 0.0) && (locals.var_beta1_i == 0.0)) && (locals.var_beta0_t == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard772 = assign50480_e83891;
        locals.var_guard772_rv = 0.0;

        let (assign50500_e83928, assign50500_e83928_d_n4, assign50500_e83928_d_n5,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50500_e83919: f64 = (locals.var_tratio - 1.0);
        let assign50500_e83920: f64 = (p.p600 * assign50500_e83919);
        let assign50500_e83921: f64 = (1.0 + assign50500_e83920);
        let assign50500_e83922: f64 = (locals.var_vdsatii0_i * assign50500_e83921);
        let assign50500_e83925: f64 = (locals.var_lii_i / locals.var_leff);
        let assign50500_e83926: f64 = (assign50500_e83922 - assign50500_e83925);
        (assign50500_e83926, (locals.var_vdsatii0_i * (p.p600 * locals.var_tratio_dn4)), (locals.var_vdsatii0_i * (p.p600 * locals.var_tratio_dn5)),)
    } else {
        (locals.var_vdsatii0, locals.var_vdsatii0_dn4, locals.var_vdsatii0_dn5,)
    }
};
        locals.var_vdsatii0 = assign50500_e83928;
        locals.var_vdsatii0_dn4 = assign50500_e83928_d_n4;
        locals.var_vdsatii0_dn5 = assign50500_e83928_d_n5;
        locals.var_vdsatii0_rv = 0.0;

        let (assign50510_e83943, assign50510_e83943_d_n3, assign50510_e83943_d_n4, assign50510_e83943_d_n5, assign50510_e83943_d_n6, assign50510_e83943_d_n7, assign50510_e83943_d_n8, assign50510_e83943_d_n9, assign50510_e83943_d_n10, assign50510_e83943_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50510_e83941: f64 = (locals.var_esatii_i * locals.var_leff);
        (assign50510_e83941, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50510_e83943;
        locals.var_t0_dn3 = assign50510_e83943_d_n3;
        locals.var_t0_dn4 = assign50510_e83943_d_n4;
        locals.var_t0_dn5 = assign50510_e83943_d_n5;
        locals.var_t0_dn6 = assign50510_e83943_d_n6;
        locals.var_t0_dn7 = assign50510_e83943_d_n7;
        locals.var_t0_dn8 = assign50510_e83943_d_n8;
        locals.var_t0_dn9 = assign50510_e83943_d_n9;
        locals.var_t0_dn10 = assign50510_e83943_d_n10;
        locals.var_t0_dn11 = assign50510_e83943_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign50520_e83962, assign50520_e83962_d_n3, assign50520_e83962_d_n4, assign50520_e83962_d_n5, assign50520_e83962_d_n6, assign50520_e83962_d_n7, assign50520_e83962_d_n8, assign50520_e83962_d_n9, assign50520_e83962_d_n10, assign50520_e83962_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50520_e83956: f64 = (locals.var_sii0_i * locals.var_t0);
        let assign50520_e83959: f64 = (1.0 + locals.var_t0);
        let assign50520_e83960: f64 = (assign50520_e83956 / assign50520_e83959);
        (assign50520_e83960, ((((locals.var_sii0_i * locals.var_t0_dn3) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn3)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn4) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn4)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn5) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn5)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn6) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn6)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn7) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn7)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn8) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn8)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn9) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn9)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn10) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn10)) / (assign50520_e83959 * assign50520_e83959)), ((((locals.var_sii0_i * locals.var_t0_dn11) * assign50520_e83959) - (assign50520_e83956 * locals.var_t0_dn11)) / (assign50520_e83959 * assign50520_e83959)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50520_e83962;
        locals.var_t1_dn3 = assign50520_e83962_d_n3;
        locals.var_t1_dn4 = assign50520_e83962_d_n4;
        locals.var_t1_dn5 = assign50520_e83962_d_n5;
        locals.var_t1_dn6 = assign50520_e83962_d_n6;
        locals.var_t1_dn7 = assign50520_e83962_d_n7;
        locals.var_t1_dn8 = assign50520_e83962_d_n8;
        locals.var_t1_dn9 = assign50520_e83962_d_n9;
        locals.var_t1_dn10 = assign50520_e83962_d_n10;
        locals.var_t1_dn11 = assign50520_e83962_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50530_e84004, assign50530_e84004_d_n3, assign50530_e84004_d_n4, assign50530_e84004_d_n5, assign50530_e84004_d_n6, assign50530_e84004_d_n7, assign50530_e84004_d_n8, assign50530_e84004_d_n9, assign50530_e84004_d_n10, assign50530_e84004_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50530_e83978: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign50530_e83980: f64 = (assign50530_e83978 * locals.var_nvt);
        let assign50530_e83983: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign50530_e83985: f64 = (assign50530_e83983 * locals.var_nvt);
        let assign50530_e83988: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign50530_e83990: f64 = (assign50530_e83988 * locals.var_nvt);
        let assign50530_e83991: f64 = (assign50530_e83985 * assign50530_e83990);
        let assign50530_e83994: f64 = (4.0 * p.p643);
        let assign50530_e83996: f64 = (assign50530_e83994 * p.p643);
        let assign50530_e83997: f64 = (assign50530_e83991 + assign50530_e83996);
        let assign50530_e83998: f64 = (assign50530_e83997).sqrt();
        let assign50530_e83999: f64 = (assign50530_e83980 + assign50530_e83998);
        let assign50530_e84000: f64 = (0.5 * assign50530_e83999);
        let assign50530_e84001: f64 = (1.0 + assign50530_e84000);
        let assign50530_e84002: f64 = (1.0 / assign50530_e84001);
        (assign50530_e84002, (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn3)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn3)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn3)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn4)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn4)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn4)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn5)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn5)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn5)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn6)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn6)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn6)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn7)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn7)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn7)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn8)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn8)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn8)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn9)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn9)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn9)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn10)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn10)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn10)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign50530_e83978 * locals.var_nvt_dn11)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign50530_e83983 * locals.var_nvt_dn11)) * assign50530_e83990) + (assign50530_e83985 * (((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign50530_e83988 * locals.var_nvt_dn11)))) / (2.0 * assign50530_e83998)))) / (assign50530_e84001 * assign50530_e84001))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50530_e84004;
        locals.var_t0_dn3 = assign50530_e84004_d_n3;
        locals.var_t0_dn4 = assign50530_e84004_d_n4;
        locals.var_t0_dn5 = assign50530_e84004_d_n5;
        locals.var_t0_dn6 = assign50530_e84004_d_n6;
        locals.var_t0_dn7 = assign50530_e84004_d_n7;
        locals.var_t0_dn8 = assign50530_e84004_d_n8;
        locals.var_t0_dn9 = assign50530_e84004_d_n9;
        locals.var_t0_dn10 = assign50530_e84004_d_n10;
        locals.var_t0_dn11 = assign50530_e84004_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign50540_e84019, assign50540_e84019_d_n3, assign50540_e84019_d_n4, assign50540_e84019_d_n5, assign50540_e84019_d_n6, assign50540_e84019_d_n7, assign50540_e84019_d_n8, assign50540_e84019_d_n9, assign50540_e84019_d_n10, assign50540_e84019_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50540_e84017: f64 = (locals.var_t0 + locals.var_sii2_i);
        (assign50540_e84017, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50540_e84019;
        locals.var_t3_dn3 = assign50540_e84019_d_n3;
        locals.var_t3_dn4 = assign50540_e84019_d_n4;
        locals.var_t3_dn5 = assign50540_e84019_d_n5;
        locals.var_t3_dn6 = assign50540_e84019_d_n6;
        locals.var_t3_dn7 = assign50540_e84019_d_n7;
        locals.var_t3_dn8 = assign50540_e84019_d_n8;
        locals.var_t3_dn9 = assign50540_e84019_d_n9;
        locals.var_t3_dn10 = assign50540_e84019_d_n10;
        locals.var_t3_dn11 = assign50540_e84019_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign50550_e84057, assign50550_e84057_d_n3, assign50550_e84057_d_n4, assign50550_e84057_d_n5, assign50550_e84057_d_n6, assign50550_e84057_d_n7, assign50550_e84057_d_n8, assign50550_e84057_d_n9, assign50550_e84057_d_n10, assign50550_e84057_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50550_e84033: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign50550_e84035: f64 = (assign50550_e84033 * locals.var_t3);
        let assign50550_e84038: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign50550_e84040: f64 = (assign50550_e84038 * locals.var_t3);
        let assign50550_e84043: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign50550_e84045: f64 = (assign50550_e84043 * locals.var_t3);
        let assign50550_e84046: f64 = (assign50550_e84040 * assign50550_e84045);
        let assign50550_e84049: f64 = (4.0 * p.p644);
        let assign50550_e84051: f64 = (assign50550_e84049 * p.p644);
        let assign50550_e84052: f64 = (assign50550_e84046 + assign50550_e84051);
        let assign50550_e84053: f64 = (assign50550_e84052).sqrt();
        let assign50550_e84054: f64 = (assign50550_e84035 + assign50550_e84053);
        let assign50550_e84055: f64 = (0.5 * assign50550_e84054);
        (assign50550_e84055, (0.5 * (((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn3)) + (((((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn3)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn3)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn4)) + (((((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn4)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn4)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn5)) + (((((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn5)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn5)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn6)) + (((((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn6)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn6)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn7)) + (((((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn7)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn7)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn8)) + (((((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn8)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn8)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn9)) + (((((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn9)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn9)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn10)) + (((((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn10)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn10)))) / (2.0 * assign50550_e84053)))), (0.5 * (((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign50550_e84033 * locals.var_t3_dn11)) + (((((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign50550_e84038 * locals.var_t3_dn11)) * assign50550_e84045) + (assign50550_e84040 * ((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign50550_e84043 * locals.var_t3_dn11)))) / (2.0 * assign50550_e84053)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign50550_e84057;
        locals.var_t2_dn3 = assign50550_e84057_d_n3;
        locals.var_t2_dn4 = assign50550_e84057_d_n4;
        locals.var_t2_dn5 = assign50550_e84057_d_n5;
        locals.var_t2_dn6 = assign50550_e84057_d_n6;
        locals.var_t2_dn7 = assign50550_e84057_d_n7;
        locals.var_t2_dn8 = assign50550_e84057_d_n8;
        locals.var_t2_dn9 = assign50550_e84057_d_n9;
        locals.var_t2_dn10 = assign50550_e84057_d_n10;
        locals.var_t2_dn11 = assign50550_e84057_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign50560_e84076, assign50560_e84076_d_n3, assign50560_e84076_d_n4, assign50560_e84076_d_n5, assign50560_e84076_d_n6, assign50560_e84076_d_n7, assign50560_e84076_d_n8, assign50560_e84076_d_n9, assign50560_e84076_d_n10, assign50560_e84076_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50560_e84072: f64 = (locals.var_siid_i * locals.var_vdsx);
        let assign50560_e84073: f64 = (1.0 + assign50560_e84072);
        let assign50560_e84074: f64 = (1.0 / assign50560_e84073);
        (assign50560_e84074, (-((locals.var_siid_i * locals.var_vdsx_dn3) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn4) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn5) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn6) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn7) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn8) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn9) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn10) / (assign50560_e84073 * assign50560_e84073))), (-((locals.var_siid_i * locals.var_vdsx_dn11) / (assign50560_e84073 * assign50560_e84073))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50560_e84076;
        locals.var_t3_dn3 = assign50560_e84076_d_n3;
        locals.var_t3_dn4 = assign50560_e84076_d_n4;
        locals.var_t3_dn5 = assign50560_e84076_d_n5;
        locals.var_t3_dn6 = assign50560_e84076_d_n6;
        locals.var_t3_dn7 = assign50560_e84076_d_n7;
        locals.var_t3_dn8 = assign50560_e84076_d_n8;
        locals.var_t3_dn9 = assign50560_e84076_d_n9;
        locals.var_t3_dn10 = assign50560_e84076_d_n10;
        locals.var_t3_dn11 = assign50560_e84076_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign50570_e84093, assign50570_e84093_d_n3, assign50570_e84093_d_n4, assign50570_e84093_d_n5, assign50570_e84093_d_n6, assign50570_e84093_d_n7, assign50570_e84093_d_n8, assign50570_e84093_d_n9, assign50570_e84093_d_n10, assign50570_e84093_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50570_e84089: f64 = (locals.var_t1 * locals.var_t2);
        let assign50570_e84091: f64 = (assign50570_e84089 * locals.var_t3);
        (assign50570_e84091, ((((locals.var_t1_dn3 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn3)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn3)), ((((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn4)), ((((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn5)), ((((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn6)), ((((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn7)), ((((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn8)), ((((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn9)), ((((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn10)), ((((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)) * locals.var_t3) + (assign50570_e84089 * locals.var_t3_dn11)),)
    } else {
        (locals.var_vgsstep, locals.var_vgsstep_dn3, locals.var_vgsstep_dn4, locals.var_vgsstep_dn5, locals.var_vgsstep_dn6, locals.var_vgsstep_dn7, locals.var_vgsstep_dn8, locals.var_vgsstep_dn9, locals.var_vgsstep_dn10, locals.var_vgsstep_dn11,)
    }
};
        locals.var_vgsstep = assign50570_e84093;
        locals.var_vgsstep_dn3 = assign50570_e84093_d_n3;
        locals.var_vgsstep_dn4 = assign50570_e84093_d_n4;
        locals.var_vgsstep_dn5 = assign50570_e84093_d_n5;
        locals.var_vgsstep_dn6 = assign50570_e84093_d_n6;
        locals.var_vgsstep_dn7 = assign50570_e84093_d_n7;
        locals.var_vgsstep_dn8 = assign50570_e84093_d_n8;
        locals.var_vgsstep_dn9 = assign50570_e84093_d_n9;
        locals.var_vgsstep_dn10 = assign50570_e84093_d_n10;
        locals.var_vgsstep_dn11 = assign50570_e84093_d_n11;
        locals.var_vgsstep_rv = 0.0;

        let (assign50580_e84108, assign50580_e84108_d_n3, assign50580_e84108_d_n4, assign50580_e84108_d_n5, assign50580_e84108_d_n6, assign50580_e84108_d_n7, assign50580_e84108_d_n8, assign50580_e84108_d_n9, assign50580_e84108_d_n10, assign50580_e84108_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50580_e84106: f64 = (locals.var_vdsatii0 + locals.var_vgsstep);
        (assign50580_e84106, locals.var_vgsstep_dn3, (locals.var_vdsatii0_dn4 + locals.var_vgsstep_dn4), (locals.var_vdsatii0_dn5 + locals.var_vgsstep_dn5), locals.var_vgsstep_dn6, locals.var_vgsstep_dn7, locals.var_vgsstep_dn8, locals.var_vgsstep_dn9, locals.var_vgsstep_dn10, locals.var_vgsstep_dn11,)
    } else {
        (locals.var_vdsatii, locals.var_vdsatii_dn3, locals.var_vdsatii_dn4, locals.var_vdsatii_dn5, locals.var_vdsatii_dn6, locals.var_vdsatii_dn7, locals.var_vdsatii_dn8, locals.var_vdsatii_dn9, locals.var_vdsatii_dn10, locals.var_vdsatii_dn11,)
    }
};
        locals.var_vdsatii = assign50580_e84108;
        locals.var_vdsatii_dn3 = assign50580_e84108_d_n3;
        locals.var_vdsatii_dn4 = assign50580_e84108_d_n4;
        locals.var_vdsatii_dn5 = assign50580_e84108_d_n5;
        locals.var_vdsatii_dn6 = assign50580_e84108_d_n6;
        locals.var_vdsatii_dn7 = assign50580_e84108_d_n7;
        locals.var_vdsatii_dn8 = assign50580_e84108_d_n8;
        locals.var_vdsatii_dn9 = assign50580_e84108_d_n9;
        locals.var_vdsatii_dn10 = assign50580_e84108_d_n10;
        locals.var_vdsatii_dn11 = assign50580_e84108_d_n11;
        locals.var_vdsatii_rv = 0.0;

        let (assign50590_e84123, assign50590_e84123_d_n3, assign50590_e84123_d_n4, assign50590_e84123_d_n5, assign50590_e84123_d_n6, assign50590_e84123_d_n7, assign50590_e84123_d_n8, assign50590_e84123_d_n9, assign50590_e84123_d_n10, assign50590_e84123_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50590_e84121: f64 = (locals.var_vdsx - locals.var_vdsatii);
        (assign50590_e84121, (locals.var_vdsx_dn3 - locals.var_vdsatii_dn3), (locals.var_vdsx_dn4 - locals.var_vdsatii_dn4), (locals.var_vdsx_dn5 - locals.var_vdsatii_dn5), (locals.var_vdsx_dn6 - locals.var_vdsatii_dn6), (locals.var_vdsx_dn7 - locals.var_vdsatii_dn7), (locals.var_vdsx_dn8 - locals.var_vdsatii_dn8), (locals.var_vdsx_dn9 - locals.var_vdsatii_dn9), (locals.var_vdsx_dn10 - locals.var_vdsatii_dn10), (locals.var_vdsx_dn11 - locals.var_vdsatii_dn11),)
    } else {
        (locals.var_vdiff, locals.var_vdiff_dn3, locals.var_vdiff_dn4, locals.var_vdiff_dn5, locals.var_vdiff_dn6, locals.var_vdiff_dn7, locals.var_vdiff_dn8, locals.var_vdiff_dn9, locals.var_vdiff_dn10, locals.var_vdiff_dn11,)
    }
};
        locals.var_vdiff = assign50590_e84123;
        locals.var_vdiff_dn3 = assign50590_e84123_d_n3;
        locals.var_vdiff_dn4 = assign50590_e84123_d_n4;
        locals.var_vdiff_dn5 = assign50590_e84123_d_n5;
        locals.var_vdiff_dn6 = assign50590_e84123_d_n6;
        locals.var_vdiff_dn7 = assign50590_e84123_d_n7;
        locals.var_vdiff_dn8 = assign50590_e84123_d_n8;
        locals.var_vdiff_dn9 = assign50590_e84123_d_n9;
        locals.var_vdiff_dn10 = assign50590_e84123_d_n10;
        locals.var_vdiff_dn11 = assign50590_e84123_d_n11;
        locals.var_vdiff_rv = 0.0;

        let (assign50600_e84146, assign50600_e84146_d_n3, assign50600_e84146_d_n4, assign50600_e84146_d_n5, assign50600_e84146_d_n6, assign50600_e84146_d_n7, assign50600_e84146_d_n8, assign50600_e84146_d_n9, assign50600_e84146_d_n10, assign50600_e84146_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50600_e84137: f64 = (locals.var_beta1_i * locals.var_vdiff);
        let assign50600_e84138: f64 = (locals.var_beta2_i + assign50600_e84137);
        let assign50600_e84141: f64 = (locals.var_beta0_t * locals.var_vdiff);
        let assign50600_e84143: f64 = (assign50600_e84141 * locals.var_vdiff);
        let assign50600_e84144: f64 = (assign50600_e84138 + assign50600_e84143);
        (assign50600_e84144, ((locals.var_beta1_i * locals.var_vdiff_dn3) + (((locals.var_beta0_t * locals.var_vdiff_dn3) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn3))), ((locals.var_beta1_i * locals.var_vdiff_dn4) + ((((locals.var_beta0_t_dn4 * locals.var_vdiff) + (locals.var_beta0_t * locals.var_vdiff_dn4)) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn4))), ((locals.var_beta1_i * locals.var_vdiff_dn5) + ((((locals.var_beta0_t_dn5 * locals.var_vdiff) + (locals.var_beta0_t * locals.var_vdiff_dn5)) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn5))), ((locals.var_beta1_i * locals.var_vdiff_dn6) + (((locals.var_beta0_t * locals.var_vdiff_dn6) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn6))), ((locals.var_beta1_i * locals.var_vdiff_dn7) + (((locals.var_beta0_t * locals.var_vdiff_dn7) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn7))), ((locals.var_beta1_i * locals.var_vdiff_dn8) + (((locals.var_beta0_t * locals.var_vdiff_dn8) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn8))), ((locals.var_beta1_i * locals.var_vdiff_dn9) + (((locals.var_beta0_t * locals.var_vdiff_dn9) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn9))), ((locals.var_beta1_i * locals.var_vdiff_dn10) + (((locals.var_beta0_t * locals.var_vdiff_dn10) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn10))), ((locals.var_beta1_i * locals.var_vdiff_dn11) + (((locals.var_beta0_t * locals.var_vdiff_dn11) * locals.var_vdiff) + (assign50600_e84141 * locals.var_vdiff_dn11))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50600_e84146;
        locals.var_t0_dn3 = assign50600_e84146_d_n3;
        locals.var_t0_dn4 = assign50600_e84146_d_n4;
        locals.var_t0_dn5 = assign50600_e84146_d_n5;
        locals.var_t0_dn6 = assign50600_e84146_d_n6;
        locals.var_t0_dn7 = assign50600_e84146_d_n7;
        locals.var_t0_dn8 = assign50600_e84146_d_n8;
        locals.var_t0_dn9 = assign50600_e84146_d_n9;
        locals.var_t0_dn10 = assign50600_e84146_d_n10;
        locals.var_t0_dn11 = assign50600_e84146_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign50610_e84164, assign50610_e84164_d_n3, assign50610_e84164_d_n4, assign50610_e84164_d_n5, assign50610_e84164_d_n6, assign50610_e84164_d_n7, assign50610_e84164_d_n8, assign50610_e84164_d_n9, assign50610_e84164_d_n10, assign50610_e84164_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 != 0.0)) && (locals.var_guard772 == 0.0)) {
        let assign50610_e84159: f64 = (locals.var_t0 * locals.var_t0);
        let assign50610_e84161: f64 = (assign50610_e84159 + 1e-10);
        let assign50610_e84162: f64 = (assign50610_e84161).sqrt();
        (assign50610_e84162, (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign50610_e84162)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign50610_e84162)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50610_e84164;
        locals.var_t1_dn3 = assign50610_e84164_d_n3;
        locals.var_t1_dn4 = assign50610_e84164_d_n4;
        locals.var_t1_dn5 = assign50610_e84164_d_n5;
        locals.var_t1_dn6 = assign50610_e84164_d_n6;
        locals.var_t1_dn7 = assign50610_e84164_d_n7;
        locals.var_t1_dn8 = assign50610_e84164_d_n8;
        locals.var_t1_dn9 = assign50610_e84164_d_n9;
        locals.var_t1_dn10 = assign50610_e84164_d_n10;
        locals.var_t1_dn11 = assign50610_e84164_d_n11;
        locals.var_t1_rv = 0.0;

        let assign50640_e84264: f64 = if ((locals.var_alpha0_i <= 0.0) || (((locals.var_beta2_i == 0.0) && (locals.var_beta1_i == 0.0)) && (locals.var_beta0_t == 0.0))) { 1.0 } else { 0.0 };
        locals.var_guard773 = assign50640_e84264;
        locals.var_guard773_rv = 0.0;

        let (assign50660_e84303, assign50660_e84303_d_n4, assign50660_e84303_d_n5,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50660_e84294: f64 = (locals.var_tratio - 1.0);
        let assign50660_e84295: f64 = (p.p600 * assign50660_e84294);
        let assign50660_e84296: f64 = (1.0 + assign50660_e84295);
        let assign50660_e84297: f64 = (locals.var_vdsatii0_i * assign50660_e84296);
        let assign50660_e84300: f64 = (locals.var_lii_i / locals.var_leff);
        let assign50660_e84301: f64 = (assign50660_e84297 - assign50660_e84300);
        (assign50660_e84301, (locals.var_vdsatii0_i * (p.p600 * locals.var_tratio_dn4)), (locals.var_vdsatii0_i * (p.p600 * locals.var_tratio_dn5)),)
    } else {
        (locals.var_vdsatii0, locals.var_vdsatii0_dn4, locals.var_vdsatii0_dn5,)
    }
};
        locals.var_vdsatii0 = assign50660_e84303;
        locals.var_vdsatii0_dn4 = assign50660_e84303_d_n4;
        locals.var_vdsatii0_dn5 = assign50660_e84303_d_n5;
        locals.var_vdsatii0_rv = 0.0;

        let (assign50670_e84319, assign50670_e84319_d_n3, assign50670_e84319_d_n4, assign50670_e84319_d_n5, assign50670_e84319_d_n6, assign50670_e84319_d_n7, assign50670_e84319_d_n8, assign50670_e84319_d_n9, assign50670_e84319_d_n10, assign50670_e84319_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50670_e84317: f64 = (locals.var_esatii_i * locals.var_leff);
        (assign50670_e84317, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50670_e84319;
        locals.var_t0_dn3 = assign50670_e84319_d_n3;
        locals.var_t0_dn4 = assign50670_e84319_d_n4;
        locals.var_t0_dn5 = assign50670_e84319_d_n5;
        locals.var_t0_dn6 = assign50670_e84319_d_n6;
        locals.var_t0_dn7 = assign50670_e84319_d_n7;
        locals.var_t0_dn8 = assign50670_e84319_d_n8;
        locals.var_t0_dn9 = assign50670_e84319_d_n9;
        locals.var_t0_dn10 = assign50670_e84319_d_n10;
        locals.var_t0_dn11 = assign50670_e84319_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign50680_e84339, assign50680_e84339_d_n3, assign50680_e84339_d_n4, assign50680_e84339_d_n5, assign50680_e84339_d_n6, assign50680_e84339_d_n7, assign50680_e84339_d_n8, assign50680_e84339_d_n9, assign50680_e84339_d_n10, assign50680_e84339_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50680_e84333: f64 = (locals.var_sii0_i * locals.var_t0);
        let assign50680_e84336: f64 = (1.0 + locals.var_t0);
        let assign50680_e84337: f64 = (assign50680_e84333 / assign50680_e84336);
        (assign50680_e84337, ((((locals.var_sii0_i * locals.var_t0_dn3) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn3)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn4) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn4)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn5) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn5)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn6) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn6)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn7) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn7)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn8) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn8)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn9) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn9)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn10) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn10)) / (assign50680_e84336 * assign50680_e84336)), ((((locals.var_sii0_i * locals.var_t0_dn11) * assign50680_e84336) - (assign50680_e84333 * locals.var_t0_dn11)) / (assign50680_e84336 * assign50680_e84336)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50680_e84339;
        locals.var_t1_dn3 = assign50680_e84339_d_n3;
        locals.var_t1_dn4 = assign50680_e84339_d_n4;
        locals.var_t1_dn5 = assign50680_e84339_d_n5;
        locals.var_t1_dn6 = assign50680_e84339_d_n6;
        locals.var_t1_dn7 = assign50680_e84339_d_n7;
        locals.var_t1_dn8 = assign50680_e84339_d_n8;
        locals.var_t1_dn9 = assign50680_e84339_d_n9;
        locals.var_t1_dn10 = assign50680_e84339_d_n10;
        locals.var_t1_dn11 = assign50680_e84339_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50690_e84382, assign50690_e84382_d_n3, assign50690_e84382_d_n4, assign50690_e84382_d_n5, assign50690_e84382_d_n6, assign50690_e84382_d_n7, assign50690_e84382_d_n8, assign50690_e84382_d_n9, assign50690_e84382_d_n10, assign50690_e84382_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50690_e84356: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign50690_e84358: f64 = (assign50690_e84356 * locals.var_nvt);
        let assign50690_e84361: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign50690_e84363: f64 = (assign50690_e84361 * locals.var_nvt);
        let assign50690_e84366: f64 = (locals.var_sii1_i * locals.var_vgsfb);
        let assign50690_e84368: f64 = (assign50690_e84366 * locals.var_nvt);
        let assign50690_e84369: f64 = (assign50690_e84363 * assign50690_e84368);
        let assign50690_e84372: f64 = (4.0 * p.p643);
        let assign50690_e84374: f64 = (assign50690_e84372 * p.p643);
        let assign50690_e84375: f64 = (assign50690_e84369 + assign50690_e84374);
        let assign50690_e84376: f64 = (assign50690_e84375).sqrt();
        let assign50690_e84377: f64 = (assign50690_e84358 + assign50690_e84376);
        let assign50690_e84378: f64 = (0.5 * assign50690_e84377);
        let assign50690_e84379: f64 = (1.0 + assign50690_e84378);
        let assign50690_e84380: f64 = (1.0 / assign50690_e84379);
        (assign50690_e84380, (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn3)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn3)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn3) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn3)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn4)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn4)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn4) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn4)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn5)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn5)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn5) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn5)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn6)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn6)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn6) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn6)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn7)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn7)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn7) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn7)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn8)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn8)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn8) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn8)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn9)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn9)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn9) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn9)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn10)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn10)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn10) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn10)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))), (-((0.5 * ((((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign50690_e84356 * locals.var_nvt_dn11)) + ((((((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign50690_e84361 * locals.var_nvt_dn11)) * assign50690_e84368) + (assign50690_e84363 * (((locals.var_sii1_i * locals.var_vgsfb_dn11) * locals.var_nvt) + (assign50690_e84366 * locals.var_nvt_dn11)))) / (2.0 * assign50690_e84376)))) / (assign50690_e84379 * assign50690_e84379))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50690_e84382;
        locals.var_t0_dn3 = assign50690_e84382_d_n3;
        locals.var_t0_dn4 = assign50690_e84382_d_n4;
        locals.var_t0_dn5 = assign50690_e84382_d_n5;
        locals.var_t0_dn6 = assign50690_e84382_d_n6;
        locals.var_t0_dn7 = assign50690_e84382_d_n7;
        locals.var_t0_dn8 = assign50690_e84382_d_n8;
        locals.var_t0_dn9 = assign50690_e84382_d_n9;
        locals.var_t0_dn10 = assign50690_e84382_d_n10;
        locals.var_t0_dn11 = assign50690_e84382_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign50700_e84398, assign50700_e84398_d_n3, assign50700_e84398_d_n4, assign50700_e84398_d_n5, assign50700_e84398_d_n6, assign50700_e84398_d_n7, assign50700_e84398_d_n8, assign50700_e84398_d_n9, assign50700_e84398_d_n10, assign50700_e84398_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50700_e84396: f64 = (locals.var_t0 + locals.var_sii2_i);
        (assign50700_e84396, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50700_e84398;
        locals.var_t3_dn3 = assign50700_e84398_d_n3;
        locals.var_t3_dn4 = assign50700_e84398_d_n4;
        locals.var_t3_dn5 = assign50700_e84398_d_n5;
        locals.var_t3_dn6 = assign50700_e84398_d_n6;
        locals.var_t3_dn7 = assign50700_e84398_d_n7;
        locals.var_t3_dn8 = assign50700_e84398_d_n8;
        locals.var_t3_dn9 = assign50700_e84398_d_n9;
        locals.var_t3_dn10 = assign50700_e84398_d_n10;
        locals.var_t3_dn11 = assign50700_e84398_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign50710_e84437, assign50710_e84437_d_n3, assign50710_e84437_d_n4, assign50710_e84437_d_n5, assign50710_e84437_d_n6, assign50710_e84437_d_n7, assign50710_e84437_d_n8, assign50710_e84437_d_n9, assign50710_e84437_d_n10, assign50710_e84437_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50710_e84413: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign50710_e84415: f64 = (assign50710_e84413 * locals.var_t3);
        let assign50710_e84418: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign50710_e84420: f64 = (assign50710_e84418 * locals.var_t3);
        let assign50710_e84423: f64 = (locals.var_vgsfb * locals.var_nvt);
        let assign50710_e84425: f64 = (assign50710_e84423 * locals.var_t3);
        let assign50710_e84426: f64 = (assign50710_e84420 * assign50710_e84425);
        let assign50710_e84429: f64 = (4.0 * p.p644);
        let assign50710_e84431: f64 = (assign50710_e84429 * p.p644);
        let assign50710_e84432: f64 = (assign50710_e84426 + assign50710_e84431);
        let assign50710_e84433: f64 = (assign50710_e84432).sqrt();
        let assign50710_e84434: f64 = (assign50710_e84415 + assign50710_e84433);
        let assign50710_e84435: f64 = (0.5 * assign50710_e84434);
        (assign50710_e84435, (0.5 * (((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn3)) + (((((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn3)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn3 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn3)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn3)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn4)) + (((((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn4)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn4 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn4)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn4)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn5)) + (((((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn5)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn5 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn5)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn5)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn6)) + (((((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn6)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn6 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn6)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn6)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn7)) + (((((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn7)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn7 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn7)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn7)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn8)) + (((((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn8)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn8 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn8)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn8)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn9)) + (((((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn9)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn9 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn9)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn9)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn10)) + (((((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn10)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn10 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn10)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn10)))) / (2.0 * assign50710_e84433)))), (0.5 * (((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign50710_e84413 * locals.var_t3_dn11)) + (((((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign50710_e84418 * locals.var_t3_dn11)) * assign50710_e84425) + (assign50710_e84420 * ((((locals.var_vgsfb_dn11 * locals.var_nvt) + (locals.var_vgsfb * locals.var_nvt_dn11)) * locals.var_t3) + (assign50710_e84423 * locals.var_t3_dn11)))) / (2.0 * assign50710_e84433)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign50710_e84437;
        locals.var_t2_dn3 = assign50710_e84437_d_n3;
        locals.var_t2_dn4 = assign50710_e84437_d_n4;
        locals.var_t2_dn5 = assign50710_e84437_d_n5;
        locals.var_t2_dn6 = assign50710_e84437_d_n6;
        locals.var_t2_dn7 = assign50710_e84437_d_n7;
        locals.var_t2_dn8 = assign50710_e84437_d_n8;
        locals.var_t2_dn9 = assign50710_e84437_d_n9;
        locals.var_t2_dn10 = assign50710_e84437_d_n10;
        locals.var_t2_dn11 = assign50710_e84437_d_n11;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_176(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign50720_e84457, assign50720_e84457_d_n3, assign50720_e84457_d_n4, assign50720_e84457_d_n5, assign50720_e84457_d_n6, assign50720_e84457_d_n7, assign50720_e84457_d_n8, assign50720_e84457_d_n9, assign50720_e84457_d_n10, assign50720_e84457_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50720_e84453: f64 = (locals.var_siid_i * locals.var_vdsx);
        let assign50720_e84454: f64 = (1.0 + assign50720_e84453);
        let assign50720_e84455: f64 = (1.0 / assign50720_e84454);
        (assign50720_e84455, (-((locals.var_siid_i * locals.var_vdsx_dn3) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn4) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn5) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn6) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn7) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn8) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn9) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn10) / (assign50720_e84454 * assign50720_e84454))), (-((locals.var_siid_i * locals.var_vdsx_dn11) / (assign50720_e84454 * assign50720_e84454))),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50720_e84457;
        locals.var_t3_dn3 = assign50720_e84457_d_n3;
        locals.var_t3_dn4 = assign50720_e84457_d_n4;
        locals.var_t3_dn5 = assign50720_e84457_d_n5;
        locals.var_t3_dn6 = assign50720_e84457_d_n6;
        locals.var_t3_dn7 = assign50720_e84457_d_n7;
        locals.var_t3_dn8 = assign50720_e84457_d_n8;
        locals.var_t3_dn9 = assign50720_e84457_d_n9;
        locals.var_t3_dn10 = assign50720_e84457_d_n10;
        locals.var_t3_dn11 = assign50720_e84457_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign50730_e84475, assign50730_e84475_d_n3, assign50730_e84475_d_n4, assign50730_e84475_d_n5, assign50730_e84475_d_n6, assign50730_e84475_d_n7, assign50730_e84475_d_n8, assign50730_e84475_d_n9, assign50730_e84475_d_n10, assign50730_e84475_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50730_e84471: f64 = (locals.var_t1 * locals.var_t2);
        let assign50730_e84473: f64 = (assign50730_e84471 * locals.var_t3);
        (assign50730_e84473, ((((locals.var_t1_dn3 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn3)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn3)), ((((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn4)), ((((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn5)), ((((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn6)), ((((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn7)), ((((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn8)), ((((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn9)), ((((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn10)), ((((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)) * locals.var_t3) + (assign50730_e84471 * locals.var_t3_dn11)),)
    } else {
        (locals.var_vgsstep, locals.var_vgsstep_dn3, locals.var_vgsstep_dn4, locals.var_vgsstep_dn5, locals.var_vgsstep_dn6, locals.var_vgsstep_dn7, locals.var_vgsstep_dn8, locals.var_vgsstep_dn9, locals.var_vgsstep_dn10, locals.var_vgsstep_dn11,)
    }
};
        locals.var_vgsstep = assign50730_e84475;
        locals.var_vgsstep_dn3 = assign50730_e84475_d_n3;
        locals.var_vgsstep_dn4 = assign50730_e84475_d_n4;
        locals.var_vgsstep_dn5 = assign50730_e84475_d_n5;
        locals.var_vgsstep_dn6 = assign50730_e84475_d_n6;
        locals.var_vgsstep_dn7 = assign50730_e84475_d_n7;
        locals.var_vgsstep_dn8 = assign50730_e84475_d_n8;
        locals.var_vgsstep_dn9 = assign50730_e84475_d_n9;
        locals.var_vgsstep_dn10 = assign50730_e84475_d_n10;
        locals.var_vgsstep_dn11 = assign50730_e84475_d_n11;
        locals.var_vgsstep_rv = 0.0;

        let (assign50740_e84491, assign50740_e84491_d_n3, assign50740_e84491_d_n4, assign50740_e84491_d_n5, assign50740_e84491_d_n6, assign50740_e84491_d_n7, assign50740_e84491_d_n8, assign50740_e84491_d_n9, assign50740_e84491_d_n10, assign50740_e84491_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50740_e84489: f64 = (locals.var_vdsatii0 + locals.var_vgsstep);
        (assign50740_e84489, locals.var_vgsstep_dn3, (locals.var_vdsatii0_dn4 + locals.var_vgsstep_dn4), (locals.var_vdsatii0_dn5 + locals.var_vgsstep_dn5), locals.var_vgsstep_dn6, locals.var_vgsstep_dn7, locals.var_vgsstep_dn8, locals.var_vgsstep_dn9, locals.var_vgsstep_dn10, locals.var_vgsstep_dn11,)
    } else {
        (locals.var_vdsatii, locals.var_vdsatii_dn3, locals.var_vdsatii_dn4, locals.var_vdsatii_dn5, locals.var_vdsatii_dn6, locals.var_vdsatii_dn7, locals.var_vdsatii_dn8, locals.var_vdsatii_dn9, locals.var_vdsatii_dn10, locals.var_vdsatii_dn11,)
    }
};
        locals.var_vdsatii = assign50740_e84491;
        locals.var_vdsatii_dn3 = assign50740_e84491_d_n3;
        locals.var_vdsatii_dn4 = assign50740_e84491_d_n4;
        locals.var_vdsatii_dn5 = assign50740_e84491_d_n5;
        locals.var_vdsatii_dn6 = assign50740_e84491_d_n6;
        locals.var_vdsatii_dn7 = assign50740_e84491_d_n7;
        locals.var_vdsatii_dn8 = assign50740_e84491_d_n8;
        locals.var_vdsatii_dn9 = assign50740_e84491_d_n9;
        locals.var_vdsatii_dn10 = assign50740_e84491_d_n10;
        locals.var_vdsatii_dn11 = assign50740_e84491_d_n11;
        locals.var_vdsatii_rv = 0.0;

        let (assign50750_e84507, assign50750_e84507_d_n3, assign50750_e84507_d_n4, assign50750_e84507_d_n5, assign50750_e84507_d_n6, assign50750_e84507_d_n7, assign50750_e84507_d_n8, assign50750_e84507_d_n9, assign50750_e84507_d_n10, assign50750_e84507_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50750_e84505: f64 = (locals.var_vdsx - locals.var_vdsatii);
        (assign50750_e84505, (locals.var_vdsx_dn3 - locals.var_vdsatii_dn3), (locals.var_vdsx_dn4 - locals.var_vdsatii_dn4), (locals.var_vdsx_dn5 - locals.var_vdsatii_dn5), (locals.var_vdsx_dn6 - locals.var_vdsatii_dn6), (locals.var_vdsx_dn7 - locals.var_vdsatii_dn7), (locals.var_vdsx_dn8 - locals.var_vdsatii_dn8), (locals.var_vdsx_dn9 - locals.var_vdsatii_dn9), (locals.var_vdsx_dn10 - locals.var_vdsatii_dn10), (locals.var_vdsx_dn11 - locals.var_vdsatii_dn11),)
    } else {
        (locals.var_vdiff, locals.var_vdiff_dn3, locals.var_vdiff_dn4, locals.var_vdiff_dn5, locals.var_vdiff_dn6, locals.var_vdiff_dn7, locals.var_vdiff_dn8, locals.var_vdiff_dn9, locals.var_vdiff_dn10, locals.var_vdiff_dn11,)
    }
};
        locals.var_vdiff = assign50750_e84507;
        locals.var_vdiff_dn3 = assign50750_e84507_d_n3;
        locals.var_vdiff_dn4 = assign50750_e84507_d_n4;
        locals.var_vdiff_dn5 = assign50750_e84507_d_n5;
        locals.var_vdiff_dn6 = assign50750_e84507_d_n6;
        locals.var_vdiff_dn7 = assign50750_e84507_d_n7;
        locals.var_vdiff_dn8 = assign50750_e84507_d_n8;
        locals.var_vdiff_dn9 = assign50750_e84507_d_n9;
        locals.var_vdiff_dn10 = assign50750_e84507_d_n10;
        locals.var_vdiff_dn11 = assign50750_e84507_d_n11;
        locals.var_vdiff_rv = 0.0;

        let (assign50760_e84531, assign50760_e84531_d_n3, assign50760_e84531_d_n4, assign50760_e84531_d_n5, assign50760_e84531_d_n6, assign50760_e84531_d_n7, assign50760_e84531_d_n8, assign50760_e84531_d_n9, assign50760_e84531_d_n10, assign50760_e84531_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50760_e84522: f64 = (locals.var_beta1_i * locals.var_vdiff);
        let assign50760_e84523: f64 = (locals.var_beta2_i + assign50760_e84522);
        let assign50760_e84526: f64 = (locals.var_beta0_t * locals.var_vdiff);
        let assign50760_e84528: f64 = (assign50760_e84526 * locals.var_vdiff);
        let assign50760_e84529: f64 = (assign50760_e84523 + assign50760_e84528);
        (assign50760_e84529, ((locals.var_beta1_i * locals.var_vdiff_dn3) + (((locals.var_beta0_t * locals.var_vdiff_dn3) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn3))), ((locals.var_beta1_i * locals.var_vdiff_dn4) + ((((locals.var_beta0_t_dn4 * locals.var_vdiff) + (locals.var_beta0_t * locals.var_vdiff_dn4)) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn4))), ((locals.var_beta1_i * locals.var_vdiff_dn5) + ((((locals.var_beta0_t_dn5 * locals.var_vdiff) + (locals.var_beta0_t * locals.var_vdiff_dn5)) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn5))), ((locals.var_beta1_i * locals.var_vdiff_dn6) + (((locals.var_beta0_t * locals.var_vdiff_dn6) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn6))), ((locals.var_beta1_i * locals.var_vdiff_dn7) + (((locals.var_beta0_t * locals.var_vdiff_dn7) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn7))), ((locals.var_beta1_i * locals.var_vdiff_dn8) + (((locals.var_beta0_t * locals.var_vdiff_dn8) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn8))), ((locals.var_beta1_i * locals.var_vdiff_dn9) + (((locals.var_beta0_t * locals.var_vdiff_dn9) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn9))), ((locals.var_beta1_i * locals.var_vdiff_dn10) + (((locals.var_beta0_t * locals.var_vdiff_dn10) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn10))), ((locals.var_beta1_i * locals.var_vdiff_dn11) + (((locals.var_beta0_t * locals.var_vdiff_dn11) * locals.var_vdiff) + (assign50760_e84526 * locals.var_vdiff_dn11))),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50760_e84531;
        locals.var_t0_dn3 = assign50760_e84531_d_n3;
        locals.var_t0_dn4 = assign50760_e84531_d_n4;
        locals.var_t0_dn5 = assign50760_e84531_d_n5;
        locals.var_t0_dn6 = assign50760_e84531_d_n6;
        locals.var_t0_dn7 = assign50760_e84531_d_n7;
        locals.var_t0_dn8 = assign50760_e84531_d_n8;
        locals.var_t0_dn9 = assign50760_e84531_d_n9;
        locals.var_t0_dn10 = assign50760_e84531_d_n10;
        locals.var_t0_dn11 = assign50760_e84531_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign50770_e84550, assign50770_e84550_d_n3, assign50770_e84550_d_n4, assign50770_e84550_d_n5, assign50770_e84550_d_n6, assign50770_e84550_d_n7, assign50770_e84550_d_n8, assign50770_e84550_d_n9, assign50770_e84550_d_n10, assign50770_e84550_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard773 == 0.0)) {
        let assign50770_e84545: f64 = (locals.var_t0 * locals.var_t0);
        let assign50770_e84547: f64 = (assign50770_e84545 + 1e-10);
        let assign50770_e84548: f64 = (assign50770_e84547).sqrt();
        (assign50770_e84548, (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign50770_e84548)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign50770_e84548)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50770_e84550;
        locals.var_t1_dn3 = assign50770_e84550_d_n3;
        locals.var_t1_dn4 = assign50770_e84550_d_n4;
        locals.var_t1_dn5 = assign50770_e84550_d_n5;
        locals.var_t1_dn6 = assign50770_e84550_d_n6;
        locals.var_t1_dn7 = assign50770_e84550_d_n7;
        locals.var_t1_dn8 = assign50770_e84550_d_n8;
        locals.var_t1_dn9 = assign50770_e84550_d_n9;
        locals.var_t1_dn10 = assign50770_e84550_d_n10;
        locals.var_t1_dn11 = assign50770_e84550_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50800_e84648, assign50800_e84648_d_n3, assign50800_e84648_d_n4, assign50800_e84648_d_n5, assign50800_e84648_d_n6, assign50800_e84648_d_n7, assign50800_e84648_d_n8, assign50800_e84648_d_n9, assign50800_e84648_d_n10, assign50800_e84648_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) {
        let assign50800_e84643: f64 = (locals.var_ebjtii_i * locals.var_leff);
        let assign50800_e84644: f64 = (locals.var_cbjtii_i + assign50800_e84643);
        let assign50800_e84646: f64 = (assign50800_e84644 / locals.var_leff);
        (assign50800_e84646, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign50800_e84648;
        locals.var_t0_dn3 = assign50800_e84648_d_n3;
        locals.var_t0_dn4 = assign50800_e84648_d_n4;
        locals.var_t0_dn5 = assign50800_e84648_d_n5;
        locals.var_t0_dn6 = assign50800_e84648_d_n6;
        locals.var_t0_dn7 = assign50800_e84648_d_n7;
        locals.var_t0_dn8 = assign50800_e84648_d_n8;
        locals.var_t0_dn9 = assign50800_e84648_d_n9;
        locals.var_t0_dn10 = assign50800_e84648_d_n10;
        locals.var_t0_dn11 = assign50800_e84648_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign50810_e84667, assign50810_e84667_d_n4, assign50810_e84667_d_n5,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) {
        let assign50810_e84662: f64 = (locals.var_tratio - 1.0);
        let assign50810_e84663: f64 = (p.p666 * assign50810_e84662);
        let assign50810_e84664: f64 = (1.0 + assign50810_e84663);
        let assign50810_e84665: f64 = (locals.var_vbci_i * assign50810_e84664);
        (assign50810_e84665, (locals.var_vbci_i * (p.p666 * locals.var_tratio_dn4)), (locals.var_vbci_i * (p.p666 * locals.var_tratio_dn5)),)
    } else {
        (locals.var_vbc, locals.var_vbc_dn4, locals.var_vbc_dn5,)
    }
};
        locals.var_vbc = assign50810_e84667;
        locals.var_vbc_dn4 = assign50810_e84667_d_n4;
        locals.var_vbc_dn5 = assign50810_e84667_d_n5;
        locals.var_vbc_rv = 0.0;

        let assign50820_e84670: f64 = if locals.var_sigvds > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard774 = assign50820_e84670;
        locals.var_guard774_rv = 0.0;

        let (assign50830_e84685, assign50830_e84685_d_n3, assign50830_e84685_d_n4, assign50830_e84685_d_n5, assign50830_e84685_d_n6, assign50830_e84685_d_n7, assign50830_e84685_d_n8, assign50830_e84685_d_n9, assign50830_e84685_d_n10, assign50830_e84685_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard774 != 0.0)) {
        let assign50830_e84683: f64 = (locals.var_vbc - locals.var_vbd_jct);
        (assign50830_e84683, 0.0, locals.var_vbc_dn4, locals.var_vbc_dn5, (-locals.var_vbd_jct_dn6), 0.0, 0.0, 0.0, (-locals.var_vbd_jct_dn10), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50830_e84685;
        locals.var_t1_dn3 = assign50830_e84685_d_n3;
        locals.var_t1_dn4 = assign50830_e84685_d_n4;
        locals.var_t1_dn5 = assign50830_e84685_d_n5;
        locals.var_t1_dn6 = assign50830_e84685_d_n6;
        locals.var_t1_dn7 = assign50830_e84685_d_n7;
        locals.var_t1_dn8 = assign50830_e84685_d_n8;
        locals.var_t1_dn9 = assign50830_e84685_d_n9;
        locals.var_t1_dn10 = assign50830_e84685_d_n10;
        locals.var_t1_dn11 = assign50830_e84685_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50840_e84701, assign50840_e84701_d_n3, assign50840_e84701_d_n4, assign50840_e84701_d_n5, assign50840_e84701_d_n6, assign50840_e84701_d_n7, assign50840_e84701_d_n8, assign50840_e84701_d_n9, assign50840_e84701_d_n10, assign50840_e84701_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard774 == 0.0)) {
        let assign50840_e84699: f64 = (locals.var_vbc - locals.var_vbs_jct);
        (assign50840_e84699, 0.0, locals.var_vbc_dn4, locals.var_vbc_dn5, 0.0, (-locals.var_vbs_jct_dn7), 0.0, 0.0, (-locals.var_vbs_jct_dn10), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign50840_e84701;
        locals.var_t1_dn3 = assign50840_e84701_d_n3;
        locals.var_t1_dn4 = assign50840_e84701_d_n4;
        locals.var_t1_dn5 = assign50840_e84701_d_n5;
        locals.var_t1_dn6 = assign50840_e84701_d_n6;
        locals.var_t1_dn7 = assign50840_e84701_d_n7;
        locals.var_t1_dn8 = assign50840_e84701_d_n8;
        locals.var_t1_dn9 = assign50840_e84701_d_n9;
        locals.var_t1_dn10 = assign50840_e84701_d_n10;
        locals.var_t1_dn11 = assign50840_e84701_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign50850_e84714, assign50850_e84714_d_n3, assign50850_e84714_d_n4, assign50850_e84714_d_n5, assign50850_e84714_d_n6, assign50850_e84714_d_n7, assign50850_e84714_d_n8, assign50850_e84714_d_n9, assign50850_e84714_d_n10, assign50850_e84714_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) {
        let assign50850_e84712: f64 = (locals.var_mbjtii_i - 1.0);
        (assign50850_e84712, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign50850_e84714;
        locals.var_t2_dn3 = assign50850_e84714_d_n3;
        locals.var_t2_dn4 = assign50850_e84714_d_n4;
        locals.var_t2_dn5 = assign50850_e84714_d_n5;
        locals.var_t2_dn6 = assign50850_e84714_d_n6;
        locals.var_t2_dn7 = assign50850_e84714_d_n7;
        locals.var_t2_dn8 = assign50850_e84714_d_n8;
        locals.var_t2_dn9 = assign50850_e84714_d_n9;
        locals.var_t2_dn10 = assign50850_e84714_d_n10;
        locals.var_t2_dn11 = assign50850_e84714_d_n11;
        locals.var_t2_rv = 0.0;

        let assign50860_e84717: f64 = if locals.var_t1 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard775 = assign50860_e84717;
        locals.var_guard775_rv = 0.0;

        let (assign50870_e84735, assign50870_e84735_d_n3, assign50870_e84735_d_n4, assign50870_e84735_d_n5, assign50870_e84735_d_n6, assign50870_e84735_d_n7, assign50870_e84735_d_n8, assign50870_e84735_d_n9, assign50870_e84735_d_n10, assign50870_e84735_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard775 != 0.0)) {
        let assign50870_e84729: f64 = (-locals.var_abjtii_i);
        let assign50870_e84732: f64 = (locals.var_t1).powf(locals.var_t2);
        let assign50870_e84733: f64 = (assign50870_e84729 * assign50870_e84732);
        (assign50870_e84733, (assign50870_e84729 * if locals.var_t2_dn3 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn3)) } } else { (assign50870_e84732 * ((locals.var_t2_dn3 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn3 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn4 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn4)) } } else { (assign50870_e84732 * ((locals.var_t2_dn4 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn4 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn5 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn5)) } } else { (assign50870_e84732 * ((locals.var_t2_dn5 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn5 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn6 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn6)) } } else { (assign50870_e84732 * ((locals.var_t2_dn6 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn6 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn7 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn7)) } } else { (assign50870_e84732 * ((locals.var_t2_dn7 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn7 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn8 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn8)) } } else { (assign50870_e84732 * ((locals.var_t2_dn8 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn8 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn9 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn9)) } } else { (assign50870_e84732 * ((locals.var_t2_dn9 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn9 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn10 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn10)) } } else { (assign50870_e84732 * ((locals.var_t2_dn10 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn10 / locals.var_t1)))) }), (assign50870_e84729 * if locals.var_t2_dn11 == 0.0 && ((locals.var_t2) as f64).is_finite() && ((locals.var_t2) as f64).fract() == 0.0 { if locals.var_t2 == 0.0 { 0.0 } else { (locals.var_t2 * ((locals.var_t1).powf(locals.var_t2 - 1.0) * locals.var_t1_dn11)) } } else { (assign50870_e84732 * ((locals.var_t2_dn11 * (locals.var_t1).ln()) + (locals.var_t2 * (locals.var_t1_dn11 / locals.var_t1)))) }),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50870_e84735;
        locals.var_t3_dn3 = assign50870_e84735_d_n3;
        locals.var_t3_dn4 = assign50870_e84735_d_n4;
        locals.var_t3_dn5 = assign50870_e84735_d_n5;
        locals.var_t3_dn6 = assign50870_e84735_d_n6;
        locals.var_t3_dn7 = assign50870_e84735_d_n7;
        locals.var_t3_dn8 = assign50870_e84735_d_n8;
        locals.var_t3_dn9 = assign50870_e84735_d_n9;
        locals.var_t3_dn10 = assign50870_e84735_d_n10;
        locals.var_t3_dn11 = assign50870_e84735_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign50880_e84749, assign50880_e84749_d_n3, assign50880_e84749_d_n4, assign50880_e84749_d_n5, assign50880_e84749_d_n6, assign50880_e84749_d_n7, assign50880_e84749_d_n8, assign50880_e84749_d_n9, assign50880_e84749_d_n10, assign50880_e84749_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) && (locals.var_guard775 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign50880_e84749;
        locals.var_t3_dn3 = assign50880_e84749_d_n3;
        locals.var_t3_dn4 = assign50880_e84749_d_n4;
        locals.var_t3_dn5 = assign50880_e84749_d_n5;
        locals.var_t3_dn6 = assign50880_e84749_d_n6;
        locals.var_t3_dn7 = assign50880_e84749_d_n7;
        locals.var_t3_dn8 = assign50880_e84749_d_n8;
        locals.var_t3_dn9 = assign50880_e84749_d_n9;
        locals.var_t3_dn10 = assign50880_e84749_d_n10;
        locals.var_t3_dn11 = assign50880_e84749_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign50890_e84761, assign50890_e84761_d_n3, assign50890_e84761_d_n4, assign50890_e84761_d_n5, assign50890_e84761_d_n6, assign50890_e84761_d_n7, assign50890_e84761_d_n8, assign50890_e84761_d_n9, assign50890_e84761_d_n10, assign50890_e84761_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard768 == 0.0)) && (locals.var_guard771 == 0.0)) {
        let assign50890_e84759: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign50890_e84759, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign50890_e84761;
        locals.var_t4_dn3 = assign50890_e84761_d_n3;
        locals.var_t4_dn4 = assign50890_e84761_d_n4;
        locals.var_t4_dn5 = assign50890_e84761_d_n5;
        locals.var_t4_dn6 = assign50890_e84761_d_n6;
        locals.var_t4_dn7 = assign50890_e84761_d_n7;
        locals.var_t4_dn8 = assign50890_e84761_d_n8;
        locals.var_t4_dn9 = assign50890_e84761_d_n9;
        locals.var_t4_dn10 = assign50890_e84761_d_n10;
        locals.var_t4_dn11 = assign50890_e84761_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign50930_e84811, assign50930_e84811_d_n4, assign50930_e84811_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign50930_e84807: f64 = (locals.var_tratio - 1.0);
        let assign50930_e84808: f64 = (locals.var_aigc1_i * assign50930_e84807);
        let assign50930_e84809: f64 = (locals.var_aigc_i + assign50930_e84808);
        (assign50930_e84809, (locals.var_aigc_i_dn4 + (locals.var_aigc1_i * locals.var_tratio_dn4)), (locals.var_aigc_i_dn5 + (locals.var_aigc1_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_aigc_i, locals.var_aigc_i_dn4, locals.var_aigc_i_dn5,)
    }
};
        locals.var_aigc_i = assign50930_e84811;
        locals.var_aigc_i_dn4 = assign50930_e84811_d_n4;
        locals.var_aigc_i_dn5 = assign50930_e84811_d_n5;
        locals.var_aigc_i_rv = 0.0;

        let (assign50940_e84822, assign50940_e84822_d_n4, assign50940_e84822_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign50940_e84818: f64 = (locals.var_tratio - 1.0);
        let assign50940_e84819: f64 = (locals.var_aigs1_i * assign50940_e84818);
        let assign50940_e84820: f64 = (locals.var_aigs_i + assign50940_e84819);
        (assign50940_e84820, (locals.var_aigs_i_dn4 + (locals.var_aigs1_i * locals.var_tratio_dn4)), (locals.var_aigs_i_dn5 + (locals.var_aigs1_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_aigs_i, locals.var_aigs_i_dn4, locals.var_aigs_i_dn5,)
    }
};
        locals.var_aigs_i = assign50940_e84822;
        locals.var_aigs_i_dn4 = assign50940_e84822_d_n4;
        locals.var_aigs_i_dn5 = assign50940_e84822_d_n5;
        locals.var_aigs_i_rv = 0.0;

        let (assign50950_e84833, assign50950_e84833_d_n4, assign50950_e84833_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign50950_e84829: f64 = (locals.var_tratio - 1.0);
        let assign50950_e84830: f64 = (locals.var_aigd1_i * assign50950_e84829);
        let assign50950_e84831: f64 = (locals.var_aigd_i + assign50950_e84830);
        (assign50950_e84831, (locals.var_aigd_i_dn4 + (locals.var_aigd1_i * locals.var_tratio_dn4)), (locals.var_aigd_i_dn5 + (locals.var_aigd1_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_aigd_i, locals.var_aigd_i_dn4, locals.var_aigd_i_dn5,)
    }
};
        locals.var_aigd_i = assign50950_e84833;
        locals.var_aigd_i_dn4 = assign50950_e84833_d_n4;
        locals.var_aigd_i_dn5 = assign50950_e84833_d_n5;
        locals.var_aigd_i_rv = 0.0;

        let (assign50960_e84844, assign50960_e84844_d_n4, assign50960_e84844_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign50960_e84840: f64 = (locals.var_tratio - 1.0);
        let assign50960_e84841: f64 = (locals.var_alphagb1_t_i * assign50960_e84840);
        let assign50960_e84842: f64 = (locals.var_alphagb1_i + assign50960_e84841);
        (assign50960_e84842, (locals.var_alphagb1_i_dn4 + (locals.var_alphagb1_t_i * locals.var_tratio_dn4)), (locals.var_alphagb1_i_dn5 + (locals.var_alphagb1_t_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_alphagb1_i, locals.var_alphagb1_i_dn4, locals.var_alphagb1_i_dn5,)
    }
};
        locals.var_alphagb1_i = assign50960_e84844;
        locals.var_alphagb1_i_dn4 = assign50960_e84844_d_n4;
        locals.var_alphagb1_i_dn5 = assign50960_e84844_d_n5;
        locals.var_alphagb1_i_rv = 0.0;

        let (assign50970_e84855, assign50970_e84855_d_n4, assign50970_e84855_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign50970_e84851: f64 = (locals.var_tratio - 1.0);
        let assign50970_e84852: f64 = (locals.var_alphagb2_t_i * assign50970_e84851);
        let assign50970_e84853: f64 = (locals.var_alphagb2_i + assign50970_e84852);
        (assign50970_e84853, (locals.var_alphagb2_i_dn4 + (locals.var_alphagb2_t_i * locals.var_tratio_dn4)), (locals.var_alphagb2_i_dn5 + (locals.var_alphagb2_t_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_alphagb2_i, locals.var_alphagb2_i_dn4, locals.var_alphagb2_i_dn5,)
    }
};
        locals.var_alphagb2_i = assign50970_e84855;
        locals.var_alphagb2_i_dn4 = assign50970_e84855_d_n4;
        locals.var_alphagb2_i_dn5 = assign50970_e84855_d_n5;
        locals.var_alphagb2_i_rv = 0.0;

        let (assign50980_e84866, assign50980_e84866_d_n4, assign50980_e84866_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign50980_e84862: f64 = (locals.var_tratio - 1.0);
        let assign50980_e84863: f64 = (locals.var_aigbcp2_t_i * assign50980_e84862);
        let assign50980_e84864: f64 = (locals.var_aigbcp2_i + assign50980_e84863);
        (assign50980_e84864, (locals.var_aigbcp2_i_dn4 + (locals.var_aigbcp2_t_i * locals.var_tratio_dn4)), (locals.var_aigbcp2_i_dn5 + (locals.var_aigbcp2_t_i * locals.var_tratio_dn5)),)
    } else {
        (locals.var_aigbcp2_i, locals.var_aigbcp2_i_dn4, locals.var_aigbcp2_i_dn5,)
    }
};
        locals.var_aigbcp2_i = assign50980_e84866;
        locals.var_aigbcp2_i_dn4 = assign50980_e84866_d_n4;
        locals.var_aigbcp2_i_dn5 = assign50980_e84866_d_n5;
        locals.var_aigbcp2_i_rv = 0.0;

        let assign51040_e84898: f64 = if ((p.p37 != 0.0) || (p.p38 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard776 = assign51040_e84898;
        locals.var_guard776_rv = 0.0;

        let (assign51050_e84913, assign51050_e84913_d_n3, assign51050_e84913_d_n4, assign51050_e84913_d_n5, assign51050_e84913_d_n6, assign51050_e84913_d_n7, assign51050_e84913_d_n8, assign51050_e84913_d_n9, assign51050_e84913_d_n10, assign51050_e84913_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) {
        let assign51050_e84906: f64 = (locals.var_vgfb - locals.var_psip);
        let assign51050_e84908: f64 = (assign51050_e84906 + locals.var_qs_1);
        let assign51050_e84910: f64 = (assign51050_e84908 + locals.var_qdeff);
        let assign51050_e84911: f64 = (locals.var_nvt * assign51050_e84910);
        (assign51050_e84911, ((locals.var_nvt_dn3 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn3 - locals.var_psip_dn3) + locals.var_qs_1_dn3) + locals.var_qdeff_dn3))), ((locals.var_nvt_dn4 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn4 - locals.var_psip_dn4) + locals.var_qs_1_dn4) + locals.var_qdeff_dn4))), ((locals.var_nvt_dn5 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn5 - locals.var_psip_dn5) + locals.var_qs_1_dn5) + locals.var_qdeff_dn5))), ((locals.var_nvt_dn6 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn6 - locals.var_psip_dn6) + locals.var_qs_1_dn6) + locals.var_qdeff_dn6))), ((locals.var_nvt_dn7 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn7 - locals.var_psip_dn7) + locals.var_qs_1_dn7) + locals.var_qdeff_dn7))), ((locals.var_nvt_dn8 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn8 - locals.var_psip_dn8) + locals.var_qs_1_dn8) + locals.var_qdeff_dn8))), ((locals.var_nvt_dn9 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn9 - locals.var_psip_dn9) + locals.var_qs_1_dn9) + locals.var_qdeff_dn9))), ((locals.var_nvt_dn10 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn10 - locals.var_psip_dn10) + locals.var_qs_1_dn10) + locals.var_qdeff_dn10))), ((locals.var_nvt_dn11 * assign51050_e84910) + (locals.var_nvt * (((locals.var_vgfb_dn11 - locals.var_psip_dn11) + locals.var_qs_1_dn11) + locals.var_qdeff_dn11))),)
    } else {
        (locals.var_voxm1, locals.var_voxm1_dn3, locals.var_voxm1_dn4, locals.var_voxm1_dn5, locals.var_voxm1_dn6, locals.var_voxm1_dn7, locals.var_voxm1_dn8, locals.var_voxm1_dn9, locals.var_voxm1_dn10, locals.var_voxm1_dn11,)
    }
};
        locals.var_voxm1 = assign51050_e84913;
        locals.var_voxm1_dn3 = assign51050_e84913_d_n3;
        locals.var_voxm1_dn4 = assign51050_e84913_d_n4;
        locals.var_voxm1_dn5 = assign51050_e84913_d_n5;
        locals.var_voxm1_dn6 = assign51050_e84913_d_n6;
        locals.var_voxm1_dn7 = assign51050_e84913_d_n7;
        locals.var_voxm1_dn8 = assign51050_e84913_d_n8;
        locals.var_voxm1_dn9 = assign51050_e84913_d_n9;
        locals.var_voxm1_dn10 = assign51050_e84913_d_n10;
        locals.var_voxm1_dn11 = assign51050_e84913_d_n11;
        locals.var_voxm1_rv = 0.0;

        let (assign51060_e84925, assign51060_e84925_d_n3, assign51060_e84925_d_n4, assign51060_e84925_d_n5, assign51060_e84925_d_n6, assign51060_e84925_d_n7, assign51060_e84925_d_n8, assign51060_e84925_d_n9, assign51060_e84925_d_n10, assign51060_e84925_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) {
        let assign51060_e84920: f64 = (locals.var_voxm1 * locals.var_voxm1);
        let assign51060_e84922: f64 = (assign51060_e84920 + 0.0001);
        let assign51060_e84923: f64 = (assign51060_e84922).sqrt();
        (assign51060_e84923, (((locals.var_voxm1_dn3 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn3)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn4 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn4)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn5 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn5)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn6 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn6)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn7 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn7)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn8 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn8)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn9 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn9)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn10 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn10)) / (2.0 * assign51060_e84923)), (((locals.var_voxm1_dn11 * locals.var_voxm1) + (locals.var_voxm1 * locals.var_voxm1_dn11)) / (2.0 * assign51060_e84923)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51060_e84925;
        locals.var_t1_dn3 = assign51060_e84925_d_n3;
        locals.var_t1_dn4 = assign51060_e84925_d_n4;
        locals.var_t1_dn5 = assign51060_e84925_d_n5;
        locals.var_t1_dn6 = assign51060_e84925_d_n6;
        locals.var_t1_dn7 = assign51060_e84925_d_n7;
        locals.var_t1_dn8 = assign51060_e84925_d_n8;
        locals.var_t1_dn9 = assign51060_e84925_d_n9;
        locals.var_t1_dn10 = assign51060_e84925_d_n10;
        locals.var_t1_dn11 = assign51060_e84925_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign51070_e84937, assign51070_e84937_d_n3, assign51070_e84937_d_n4, assign51070_e84937_d_n5, assign51070_e84937_d_n6, assign51070_e84937_d_n7, assign51070_e84937_d_n8, assign51070_e84937_d_n9, assign51070_e84937_d_n10, assign51070_e84937_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) {
        let assign51070_e84932: f64 = (-locals.var_voxm1);
        let assign51070_e84934: f64 = (assign51070_e84932 + locals.var_t1);
        let assign51070_e84935: f64 = (0.5 * assign51070_e84934);
        (assign51070_e84935, (0.5 * ((-locals.var_voxm1_dn3) + locals.var_t1_dn3)), (0.5 * ((-locals.var_voxm1_dn4) + locals.var_t1_dn4)), (0.5 * ((-locals.var_voxm1_dn5) + locals.var_t1_dn5)), (0.5 * ((-locals.var_voxm1_dn6) + locals.var_t1_dn6)), (0.5 * ((-locals.var_voxm1_dn7) + locals.var_t1_dn7)), (0.5 * ((-locals.var_voxm1_dn8) + locals.var_t1_dn8)), (0.5 * ((-locals.var_voxm1_dn9) + locals.var_t1_dn9)), (0.5 * ((-locals.var_voxm1_dn10) + locals.var_t1_dn10)), (0.5 * ((-locals.var_voxm1_dn11) + locals.var_t1_dn11)),)
    } else {
        (locals.var_voxmacc, locals.var_voxmacc_dn3, locals.var_voxmacc_dn4, locals.var_voxmacc_dn5, locals.var_voxmacc_dn6, locals.var_voxmacc_dn7, locals.var_voxmacc_dn8, locals.var_voxmacc_dn9, locals.var_voxmacc_dn10, locals.var_voxmacc_dn11,)
    }
};
        locals.var_voxmacc = assign51070_e84937;
        locals.var_voxmacc_dn3 = assign51070_e84937_d_n3;
        locals.var_voxmacc_dn4 = assign51070_e84937_d_n4;
        locals.var_voxmacc_dn5 = assign51070_e84937_d_n5;
        locals.var_voxmacc_dn6 = assign51070_e84937_d_n6;
        locals.var_voxmacc_dn7 = assign51070_e84937_d_n7;
        locals.var_voxmacc_dn8 = assign51070_e84937_d_n8;
        locals.var_voxmacc_dn9 = assign51070_e84937_d_n9;
        locals.var_voxmacc_dn10 = assign51070_e84937_d_n10;
        locals.var_voxmacc_dn11 = assign51070_e84937_d_n11;
        locals.var_voxmacc_rv = 0.0;

        let (assign51080_e84948, assign51080_e84948_d_n3, assign51080_e84948_d_n4, assign51080_e84948_d_n5, assign51080_e84948_d_n6, assign51080_e84948_d_n7, assign51080_e84948_d_n8, assign51080_e84948_d_n9, assign51080_e84948_d_n10, assign51080_e84948_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) {
        let assign51080_e84945: f64 = (locals.var_voxm1 + locals.var_t1);
        let assign51080_e84946: f64 = (0.5 * assign51080_e84945);
        (assign51080_e84946, (0.5 * (locals.var_voxm1_dn3 + locals.var_t1_dn3)), (0.5 * (locals.var_voxm1_dn4 + locals.var_t1_dn4)), (0.5 * (locals.var_voxm1_dn5 + locals.var_t1_dn5)), (0.5 * (locals.var_voxm1_dn6 + locals.var_t1_dn6)), (0.5 * (locals.var_voxm1_dn7 + locals.var_t1_dn7)), (0.5 * (locals.var_voxm1_dn8 + locals.var_t1_dn8)), (0.5 * (locals.var_voxm1_dn9 + locals.var_t1_dn9)), (0.5 * (locals.var_voxm1_dn10 + locals.var_t1_dn10)), (0.5 * (locals.var_voxm1_dn11 + locals.var_t1_dn11)),)
    } else {
        (locals.var_voxminv, locals.var_voxminv_dn3, locals.var_voxminv_dn4, locals.var_voxminv_dn5, locals.var_voxminv_dn6, locals.var_voxminv_dn7, locals.var_voxminv_dn8, locals.var_voxminv_dn9, locals.var_voxminv_dn10, locals.var_voxminv_dn11,)
    }
};
        locals.var_voxminv = assign51080_e84948;
        locals.var_voxminv_dn3 = assign51080_e84948_d_n3;
        locals.var_voxminv_dn4 = assign51080_e84948_d_n4;
        locals.var_voxminv_dn5 = assign51080_e84948_d_n5;
        locals.var_voxminv_dn6 = assign51080_e84948_d_n6;
        locals.var_voxminv_dn7 = assign51080_e84948_d_n7;
        locals.var_voxminv_dn8 = assign51080_e84948_d_n8;
        locals.var_voxminv_dn9 = assign51080_e84948_d_n9;
        locals.var_voxminv_dn10 = assign51080_e84948_d_n10;
        locals.var_voxminv_dn11 = assign51080_e84948_d_n11;
        locals.var_voxminv_rv = 0.0;

        let assign51090_e84951: f64 = if p.p38 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard777 = assign51090_e84951;
        locals.var_guard777_rv = 0.0;

        let (assign51100_e84962, assign51100_e84962_d_n3, assign51100_e84962_d_n4, assign51100_e84962_d_n5, assign51100_e84962_d_n6, assign51100_e84962_d_n7, assign51100_e84962_d_n8, assign51100_e84962_d_n9, assign51100_e84962_d_n10, assign51100_e84962_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51100_e84960: f64 = (locals.var_voxm1 / p.p671);
        (assign51100_e84960, (locals.var_voxm1_dn3 / p.p671), (locals.var_voxm1_dn4 / p.p671), (locals.var_voxm1_dn5 / p.p671), (locals.var_voxm1_dn6 / p.p671), (locals.var_voxm1_dn7 / p.p671), (locals.var_voxm1_dn8 / p.p671), (locals.var_voxm1_dn9 / p.p671), (locals.var_voxm1_dn10 / p.p671), (locals.var_voxm1_dn11 / p.p671),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51100_e84962;
        locals.var_t1_dn3 = assign51100_e84962_d_n3;
        locals.var_t1_dn4 = assign51100_e84962_d_n4;
        locals.var_t1_dn5 = assign51100_e84962_d_n5;
        locals.var_t1_dn6 = assign51100_e84962_d_n6;
        locals.var_t1_dn7 = assign51100_e84962_d_n7;
        locals.var_t1_dn8 = assign51100_e84962_d_n8;
        locals.var_t1_dn9 = assign51100_e84962_d_n9;
        locals.var_t1_dn10 = assign51100_e84962_d_n10;
        locals.var_t1_dn11 = assign51100_e84962_d_n11;
        locals.var_t1_rv = 0.0;

        let assign51120_e85017: f64 = if p.p696 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard778 = assign51120_e85017;
        locals.var_guard778_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_177(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (assign51130_e85032, assign51130_e85032_d_n3, assign51130_e85032_d_n4, assign51130_e85032_d_n5, assign51130_e85032_d_n6, assign51130_e85032_d_n7, assign51130_e85032_d_n8, assign51130_e85032_d_n9, assign51130_e85032_d_n10, assign51130_e85032_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard778 != 0.0)) {
        let assign51130_e85029: f64 = (locals.var_voxmacc / p.p696);
        let assign51130_e85030: f64 = (1.0 - assign51130_e85029);
        (assign51130_e85030, (-(locals.var_voxmacc_dn3 / p.p696)), (-(locals.var_voxmacc_dn4 / p.p696)), (-(locals.var_voxmacc_dn5 / p.p696)), (-(locals.var_voxmacc_dn6 / p.p696)), (-(locals.var_voxmacc_dn7 / p.p696)), (-(locals.var_voxmacc_dn8 / p.p696)), (-(locals.var_voxmacc_dn9 / p.p696)), (-(locals.var_voxmacc_dn10 / p.p696)), (-(locals.var_voxmacc_dn11 / p.p696)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign51130_e85032;
        locals.var_t0_dn3 = assign51130_e85032_d_n3;
        locals.var_t0_dn4 = assign51130_e85032_d_n4;
        locals.var_t0_dn5 = assign51130_e85032_d_n5;
        locals.var_t0_dn6 = assign51130_e85032_d_n6;
        locals.var_t0_dn7 = assign51130_e85032_d_n7;
        locals.var_t0_dn8 = assign51130_e85032_d_n8;
        locals.var_t0_dn9 = assign51130_e85032_d_n9;
        locals.var_t0_dn10 = assign51130_e85032_d_n10;
        locals.var_t0_dn11 = assign51130_e85032_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign51140_e85044, assign51140_e85044_d_n3, assign51140_e85044_d_n4, assign51140_e85044_d_n5, assign51140_e85044_d_n6, assign51140_e85044_d_n7, assign51140_e85044_d_n8, assign51140_e85044_d_n9, assign51140_e85044_d_n10, assign51140_e85044_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard778 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign51140_e85044;
        locals.var_t0_dn3 = assign51140_e85044_d_n3;
        locals.var_t0_dn4 = assign51140_e85044_d_n4;
        locals.var_t0_dn5 = assign51140_e85044_d_n5;
        locals.var_t0_dn6 = assign51140_e85044_d_n6;
        locals.var_t0_dn7 = assign51140_e85044_d_n7;
        locals.var_t0_dn8 = assign51140_e85044_d_n8;
        locals.var_t0_dn9 = assign51140_e85044_d_n9;
        locals.var_t0_dn10 = assign51140_e85044_d_n10;
        locals.var_t0_dn11 = assign51140_e85044_d_n11;
        locals.var_t0_rv = 0.0;

        let assign51150_e85047: f64 = if locals.var_t0 < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard779 = assign51150_e85047;
        locals.var_guard779_rv = 0.0;

        let (assign51160_e85058, assign51160_e85058_d_n3, assign51160_e85058_d_n4, assign51160_e85058_d_n5, assign51160_e85058_d_n6, assign51160_e85058_d_n7, assign51160_e85058_d_n8, assign51160_e85058_d_n9, assign51160_e85058_d_n10, assign51160_e85058_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard779 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign51160_e85058;
        locals.var_t0_dn3 = assign51160_e85058_d_n3;
        locals.var_t0_dn4 = assign51160_e85058_d_n4;
        locals.var_t0_dn5 = assign51160_e85058_d_n5;
        locals.var_t0_dn6 = assign51160_e85058_d_n6;
        locals.var_t0_dn7 = assign51160_e85058_d_n7;
        locals.var_t0_dn8 = assign51160_e85058_d_n8;
        locals.var_t0_dn9 = assign51160_e85058_d_n9;
        locals.var_t0_dn10 = assign51160_e85058_d_n10;
        locals.var_t0_dn11 = assign51160_e85058_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign51170_e85079, assign51170_e85079_d_n3, assign51170_e85079_d_n4, assign51170_e85079_d_n5, assign51170_e85079_d_n6, assign51170_e85079_d_n7, assign51170_e85079_d_n8, assign51170_e85079_d_n9, assign51170_e85079_d_n10, assign51170_e85079_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51170_e85067: f64 = (locals.var_leff * locals.var_weff);
        let assign51170_e85069: f64 = (assign51170_e85067 / p.p1373);
        let assign51170_e85072: f64 = (p.p1381 / p.p2);
        let assign51170_e85073: f64 = (assign51170_e85069 + assign51170_e85072);
        let assign51170_e85075: f64 = (assign51170_e85073 * p.p700);
        let assign51170_e85077: f64 = (assign51170_e85075 * locals.var_toxratio);
        (assign51170_e85077, (assign51170_e85075 * locals.var_toxratio_dn3), (assign51170_e85075 * locals.var_toxratio_dn4), (assign51170_e85075 * locals.var_toxratio_dn5), (assign51170_e85075 * locals.var_toxratio_dn6), (assign51170_e85075 * locals.var_toxratio_dn7), (assign51170_e85075 * locals.var_toxratio_dn8), (assign51170_e85075 * locals.var_toxratio_dn9), (assign51170_e85075 * locals.var_toxratio_dn10), (assign51170_e85075 * locals.var_toxratio_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51170_e85079;
        locals.var_t1_dn3 = assign51170_e85079_d_n3;
        locals.var_t1_dn4 = assign51170_e85079_d_n4;
        locals.var_t1_dn5 = assign51170_e85079_d_n5;
        locals.var_t1_dn6 = assign51170_e85079_d_n6;
        locals.var_t1_dn7 = assign51170_e85079_d_n7;
        locals.var_t1_dn8 = assign51170_e85079_d_n8;
        locals.var_t1_dn9 = assign51170_e85079_d_n9;
        locals.var_t1_dn10 = assign51170_e85079_d_n10;
        locals.var_t1_dn11 = assign51170_e85079_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign51180_e85090, assign51180_e85090_d_n3, assign51180_e85090_d_n4, assign51180_e85090_d_n5, assign51180_e85090_d_n6, assign51180_e85090_d_n7, assign51180_e85090_d_n8, assign51180_e85090_d_n9, assign51180_e85090_d_n10, assign51180_e85090_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51180_e85088: f64 = (p.p701 * p.p76);
        (assign51180_e85088, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51180_e85090;
        locals.var_t2_dn3 = assign51180_e85090_d_n3;
        locals.var_t2_dn4 = assign51180_e85090_d_n4;
        locals.var_t2_dn5 = assign51180_e85090_d_n5;
        locals.var_t2_dn6 = assign51180_e85090_d_n6;
        locals.var_t2_dn7 = assign51180_e85090_d_n7;
        locals.var_t2_dn8 = assign51180_e85090_d_n8;
        locals.var_t2_dn9 = assign51180_e85090_d_n9;
        locals.var_t2_dn10 = assign51180_e85090_d_n10;
        locals.var_t2_dn11 = assign51180_e85090_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign51190_e85107, assign51190_e85107_d_n3, assign51190_e85107_d_n4, assign51190_e85107_d_n5, assign51190_e85107_d_n6, assign51190_e85107_d_n7, assign51190_e85107_d_n8, assign51190_e85107_d_n9, assign51190_e85107_d_n10, assign51190_e85107_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51190_e85101: f64 = (locals.var_betagb2_i * locals.var_voxmacc);
        let assign51190_e85102: f64 = (locals.var_alphagb2_i - assign51190_e85101);
        let assign51190_e85103: f64 = (locals.var_t2 * assign51190_e85102);
        let assign51190_e85105: f64 = (assign51190_e85103 / locals.var_t0);
        (assign51190_e85105, (((((locals.var_t2_dn3 * assign51190_e85102) + (locals.var_t2 * (-(locals.var_betagb2_i * locals.var_voxmacc_dn3)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn4 * assign51190_e85102) + (locals.var_t2 * (locals.var_alphagb2_i_dn4 - (locals.var_betagb2_i * locals.var_voxmacc_dn4)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn5 * assign51190_e85102) + (locals.var_t2 * (locals.var_alphagb2_i_dn5 - (locals.var_betagb2_i * locals.var_voxmacc_dn5)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn6 * assign51190_e85102) + (locals.var_t2 * (-(locals.var_betagb2_i * locals.var_voxmacc_dn6)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn7 * assign51190_e85102) + (locals.var_t2 * (-(locals.var_betagb2_i * locals.var_voxmacc_dn7)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn8 * assign51190_e85102) + (locals.var_t2 * (-(locals.var_betagb2_i * locals.var_voxmacc_dn8)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn9 * assign51190_e85102) + (locals.var_t2 * (-(locals.var_betagb2_i * locals.var_voxmacc_dn9)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn10 * assign51190_e85102) + (locals.var_t2 * (-(locals.var_betagb2_i * locals.var_voxmacc_dn10)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn11 * assign51190_e85102) + (locals.var_t2 * (-(locals.var_betagb2_i * locals.var_voxmacc_dn11)))) * locals.var_t0) - (assign51190_e85103 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign51190_e85107;
        locals.var_t3_dn3 = assign51190_e85107_d_n3;
        locals.var_t3_dn4 = assign51190_e85107_d_n4;
        locals.var_t3_dn5 = assign51190_e85107_d_n5;
        locals.var_t3_dn6 = assign51190_e85107_d_n6;
        locals.var_t3_dn7 = assign51190_e85107_d_n7;
        locals.var_t3_dn8 = assign51190_e85107_d_n8;
        locals.var_t3_dn9 = assign51190_e85107_d_n9;
        locals.var_t3_dn10 = assign51190_e85107_d_n10;
        locals.var_t3_dn11 = assign51190_e85107_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign51200_e85117, assign51200_e85117_d_n3, assign51200_e85117_d_n4, assign51200_e85117_d_n5, assign51200_e85117_d_n6, assign51200_e85117_d_n7, assign51200_e85117_d_n8, assign51200_e85117_d_n9, assign51200_e85117_d_n10, assign51200_e85117_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51200_e85115: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign51200_e85115, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign51200_e85117;
        locals.var_t4_dn3 = assign51200_e85117_d_n3;
        locals.var_t4_dn4 = assign51200_e85117_d_n4;
        locals.var_t4_dn5 = assign51200_e85117_d_n5;
        locals.var_t4_dn6 = assign51200_e85117_d_n6;
        locals.var_t4_dn7 = assign51200_e85117_d_n7;
        locals.var_t4_dn8 = assign51200_e85117_d_n8;
        locals.var_t4_dn9 = assign51200_e85117_d_n9;
        locals.var_t4_dn10 = assign51200_e85117_d_n10;
        locals.var_t4_dn11 = assign51200_e85117_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign51230_e85156, assign51230_e85156_d_n3, assign51230_e85156_d_n4, assign51230_e85156_d_n5, assign51230_e85156_d_n6, assign51230_e85156_d_n7, assign51230_e85156_d_n8, assign51230_e85156_d_n9, assign51230_e85156_d_n10, assign51230_e85156_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51230_e85152: f64 = (locals.var_voxm1 - locals.var_eigbinv_i);
        let assign51230_e85154: f64 = (assign51230_e85152 / p.p671);
        (assign51230_e85154, (locals.var_voxm1_dn3 / p.p671), (locals.var_voxm1_dn4 / p.p671), (locals.var_voxm1_dn5 / p.p671), (locals.var_voxm1_dn6 / p.p671), (locals.var_voxm1_dn7 / p.p671), (locals.var_voxm1_dn8 / p.p671), (locals.var_voxm1_dn9 / p.p671), (locals.var_voxm1_dn10 / p.p671), (locals.var_voxm1_dn11 / p.p671),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51230_e85156;
        locals.var_t1_dn3 = assign51230_e85156_d_n3;
        locals.var_t1_dn4 = assign51230_e85156_d_n4;
        locals.var_t1_dn5 = assign51230_e85156_d_n5;
        locals.var_t1_dn6 = assign51230_e85156_d_n6;
        locals.var_t1_dn7 = assign51230_e85156_d_n7;
        locals.var_t1_dn8 = assign51230_e85156_d_n8;
        locals.var_t1_dn9 = assign51230_e85156_d_n9;
        locals.var_t1_dn10 = assign51230_e85156_d_n10;
        locals.var_t1_dn11 = assign51230_e85156_d_n11;
        locals.var_t1_rv = 0.0;

        let assign51250_e85203: f64 = if p.p697 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard780 = assign51250_e85203;
        locals.var_guard780_rv = 0.0;

        let (assign51260_e85218, assign51260_e85218_d_n3, assign51260_e85218_d_n4, assign51260_e85218_d_n5, assign51260_e85218_d_n6, assign51260_e85218_d_n7, assign51260_e85218_d_n8, assign51260_e85218_d_n9, assign51260_e85218_d_n10, assign51260_e85218_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard780 != 0.0)) {
        let assign51260_e85215: f64 = (locals.var_voxminv / p.p697);
        let assign51260_e85216: f64 = (1.0 - assign51260_e85215);
        (assign51260_e85216, (-(locals.var_voxminv_dn3 / p.p697)), (-(locals.var_voxminv_dn4 / p.p697)), (-(locals.var_voxminv_dn5 / p.p697)), (-(locals.var_voxminv_dn6 / p.p697)), (-(locals.var_voxminv_dn7 / p.p697)), (-(locals.var_voxminv_dn8 / p.p697)), (-(locals.var_voxminv_dn9 / p.p697)), (-(locals.var_voxminv_dn10 / p.p697)), (-(locals.var_voxminv_dn11 / p.p697)),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign51260_e85218;
        locals.var_t0_dn3 = assign51260_e85218_d_n3;
        locals.var_t0_dn4 = assign51260_e85218_d_n4;
        locals.var_t0_dn5 = assign51260_e85218_d_n5;
        locals.var_t0_dn6 = assign51260_e85218_d_n6;
        locals.var_t0_dn7 = assign51260_e85218_d_n7;
        locals.var_t0_dn8 = assign51260_e85218_d_n8;
        locals.var_t0_dn9 = assign51260_e85218_d_n9;
        locals.var_t0_dn10 = assign51260_e85218_d_n10;
        locals.var_t0_dn11 = assign51260_e85218_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign51270_e85230, assign51270_e85230_d_n3, assign51270_e85230_d_n4, assign51270_e85230_d_n5, assign51270_e85230_d_n6, assign51270_e85230_d_n7, assign51270_e85230_d_n8, assign51270_e85230_d_n9, assign51270_e85230_d_n10, assign51270_e85230_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard780 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign51270_e85230;
        locals.var_t0_dn3 = assign51270_e85230_d_n3;
        locals.var_t0_dn4 = assign51270_e85230_d_n4;
        locals.var_t0_dn5 = assign51270_e85230_d_n5;
        locals.var_t0_dn6 = assign51270_e85230_d_n6;
        locals.var_t0_dn7 = assign51270_e85230_d_n7;
        locals.var_t0_dn8 = assign51270_e85230_d_n8;
        locals.var_t0_dn9 = assign51270_e85230_d_n9;
        locals.var_t0_dn10 = assign51270_e85230_d_n10;
        locals.var_t0_dn11 = assign51270_e85230_d_n11;
        locals.var_t0_rv = 0.0;

        let assign51280_e85233: f64 = if locals.var_t0 < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard781 = assign51280_e85233;
        locals.var_guard781_rv = 0.0;

        let (assign51290_e85244, assign51290_e85244_d_n3, assign51290_e85244_d_n4, assign51290_e85244_d_n5, assign51290_e85244_d_n6, assign51290_e85244_d_n7, assign51290_e85244_d_n8, assign51290_e85244_d_n9, assign51290_e85244_d_n10, assign51290_e85244_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) && (locals.var_guard781 != 0.0)) {
        (0.01, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign51290_e85244;
        locals.var_t0_dn3 = assign51290_e85244_d_n3;
        locals.var_t0_dn4 = assign51290_e85244_d_n4;
        locals.var_t0_dn5 = assign51290_e85244_d_n5;
        locals.var_t0_dn6 = assign51290_e85244_d_n6;
        locals.var_t0_dn7 = assign51290_e85244_d_n7;
        locals.var_t0_dn8 = assign51290_e85244_d_n8;
        locals.var_t0_dn9 = assign51290_e85244_d_n9;
        locals.var_t0_dn10 = assign51290_e85244_d_n10;
        locals.var_t0_dn11 = assign51290_e85244_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign51300_e85265, assign51300_e85265_d_n3, assign51300_e85265_d_n4, assign51300_e85265_d_n5, assign51300_e85265_d_n6, assign51300_e85265_d_n7, assign51300_e85265_d_n8, assign51300_e85265_d_n9, assign51300_e85265_d_n10, assign51300_e85265_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51300_e85253: f64 = (locals.var_leff * locals.var_weff);
        let assign51300_e85255: f64 = (assign51300_e85253 / p.p1373);
        let assign51300_e85258: f64 = (p.p1381 / p.p2);
        let assign51300_e85259: f64 = (assign51300_e85255 + assign51300_e85258);
        let assign51300_e85261: f64 = (assign51300_e85259 * p.p698);
        let assign51300_e85263: f64 = (assign51300_e85261 * locals.var_toxratio);
        (assign51300_e85263, (assign51300_e85261 * locals.var_toxratio_dn3), (assign51300_e85261 * locals.var_toxratio_dn4), (assign51300_e85261 * locals.var_toxratio_dn5), (assign51300_e85261 * locals.var_toxratio_dn6), (assign51300_e85261 * locals.var_toxratio_dn7), (assign51300_e85261 * locals.var_toxratio_dn8), (assign51300_e85261 * locals.var_toxratio_dn9), (assign51300_e85261 * locals.var_toxratio_dn10), (assign51300_e85261 * locals.var_toxratio_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51300_e85265;
        locals.var_t1_dn3 = assign51300_e85265_d_n3;
        locals.var_t1_dn4 = assign51300_e85265_d_n4;
        locals.var_t1_dn5 = assign51300_e85265_d_n5;
        locals.var_t1_dn6 = assign51300_e85265_d_n6;
        locals.var_t1_dn7 = assign51300_e85265_d_n7;
        locals.var_t1_dn8 = assign51300_e85265_d_n8;
        locals.var_t1_dn9 = assign51300_e85265_d_n9;
        locals.var_t1_dn10 = assign51300_e85265_d_n10;
        locals.var_t1_dn11 = assign51300_e85265_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign51310_e85276, assign51310_e85276_d_n3, assign51310_e85276_d_n4, assign51310_e85276_d_n5, assign51310_e85276_d_n6, assign51310_e85276_d_n7, assign51310_e85276_d_n8, assign51310_e85276_d_n9, assign51310_e85276_d_n10, assign51310_e85276_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51310_e85274: f64 = (p.p699 * p.p76);
        (assign51310_e85274, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51310_e85276;
        locals.var_t2_dn3 = assign51310_e85276_d_n3;
        locals.var_t2_dn4 = assign51310_e85276_d_n4;
        locals.var_t2_dn5 = assign51310_e85276_d_n5;
        locals.var_t2_dn6 = assign51310_e85276_d_n6;
        locals.var_t2_dn7 = assign51310_e85276_d_n7;
        locals.var_t2_dn8 = assign51310_e85276_d_n8;
        locals.var_t2_dn9 = assign51310_e85276_d_n9;
        locals.var_t2_dn10 = assign51310_e85276_d_n10;
        locals.var_t2_dn11 = assign51310_e85276_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign51320_e85293, assign51320_e85293_d_n3, assign51320_e85293_d_n4, assign51320_e85293_d_n5, assign51320_e85293_d_n6, assign51320_e85293_d_n7, assign51320_e85293_d_n8, assign51320_e85293_d_n9, assign51320_e85293_d_n10, assign51320_e85293_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51320_e85287: f64 = (locals.var_betagb1_i * locals.var_voxminv);
        let assign51320_e85288: f64 = (locals.var_alphagb1_i - assign51320_e85287);
        let assign51320_e85289: f64 = (locals.var_t2 * assign51320_e85288);
        let assign51320_e85291: f64 = (assign51320_e85289 / locals.var_t0);
        (assign51320_e85291, (((((locals.var_t2_dn3 * assign51320_e85288) + (locals.var_t2 * (-(locals.var_betagb1_i * locals.var_voxminv_dn3)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn3)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn4 * assign51320_e85288) + (locals.var_t2 * (locals.var_alphagb1_i_dn4 - (locals.var_betagb1_i * locals.var_voxminv_dn4)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn5 * assign51320_e85288) + (locals.var_t2 * (locals.var_alphagb1_i_dn5 - (locals.var_betagb1_i * locals.var_voxminv_dn5)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn6 * assign51320_e85288) + (locals.var_t2 * (-(locals.var_betagb1_i * locals.var_voxminv_dn6)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn7 * assign51320_e85288) + (locals.var_t2 * (-(locals.var_betagb1_i * locals.var_voxminv_dn7)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn8 * assign51320_e85288) + (locals.var_t2 * (-(locals.var_betagb1_i * locals.var_voxminv_dn8)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn9 * assign51320_e85288) + (locals.var_t2 * (-(locals.var_betagb1_i * locals.var_voxminv_dn9)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn10 * assign51320_e85288) + (locals.var_t2 * (-(locals.var_betagb1_i * locals.var_voxminv_dn10)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((((locals.var_t2_dn11 * assign51320_e85288) + (locals.var_t2 * (-(locals.var_betagb1_i * locals.var_voxminv_dn11)))) * locals.var_t0) - (assign51320_e85289 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign51320_e85293;
        locals.var_t3_dn3 = assign51320_e85293_d_n3;
        locals.var_t3_dn4 = assign51320_e85293_d_n4;
        locals.var_t3_dn5 = assign51320_e85293_d_n5;
        locals.var_t3_dn6 = assign51320_e85293_d_n6;
        locals.var_t3_dn7 = assign51320_e85293_d_n7;
        locals.var_t3_dn8 = assign51320_e85293_d_n8;
        locals.var_t3_dn9 = assign51320_e85293_d_n9;
        locals.var_t3_dn10 = assign51320_e85293_d_n10;
        locals.var_t3_dn11 = assign51320_e85293_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign51330_e85303, assign51330_e85303_d_n3, assign51330_e85303_d_n4, assign51330_e85303_d_n5, assign51330_e85303_d_n6, assign51330_e85303_d_n7, assign51330_e85303_d_n8, assign51330_e85303_d_n9, assign51330_e85303_d_n10, assign51330_e85303_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51330_e85301: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign51330_e85301, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign51330_e85303;
        locals.var_t4_dn3 = assign51330_e85303_d_n3;
        locals.var_t4_dn4 = assign51330_e85303_d_n4;
        locals.var_t4_dn5 = assign51330_e85303_d_n5;
        locals.var_t4_dn6 = assign51330_e85303_d_n6;
        locals.var_t4_dn7 = assign51330_e85303_d_n7;
        locals.var_t4_dn8 = assign51330_e85303_d_n8;
        locals.var_t4_dn9 = assign51330_e85303_d_n9;
        locals.var_t4_dn10 = assign51330_e85303_d_n10;
        locals.var_t4_dn11 = assign51330_e85303_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign51370_e85355, assign51370_e85355_d_n3, assign51370_e85355_d_n4, assign51370_e85355_d_n5, assign51370_e85355_d_n6, assign51370_e85355_d_n7, assign51370_e85355_d_n8, assign51370_e85355_d_n9, assign51370_e85355_d_n10, assign51370_e85355_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard777 != 0.0)) {
        let assign51370_e85351: f64 = (locals.var_vfb * locals.var_nvt);
        let assign51370_e85353: f64 = (assign51370_e85351 + p.p1383);
        (assign51370_e85353, ((locals.var_vfb_dn3 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn3)), ((locals.var_vfb_dn4 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn4)), ((locals.var_vfb_dn5 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn5)), ((locals.var_vfb_dn6 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn6)), ((locals.var_vfb_dn7 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn7)), ((locals.var_vfb_dn8 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn8)), ((locals.var_vfb_dn9 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn9)), ((locals.var_vfb_dn10 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn10)), ((locals.var_vfb_dn11 * locals.var_nvt) + (locals.var_vfb * locals.var_nvt_dn11)),)
    } else {
        (locals.var_vfb2, locals.var_vfb2_dn3, locals.var_vfb2_dn4, locals.var_vfb2_dn5, locals.var_vfb2_dn6, locals.var_vfb2_dn7, locals.var_vfb2_dn8, locals.var_vfb2_dn9, locals.var_vfb2_dn10, locals.var_vfb2_dn11,)
    }
};
        locals.var_vfb2 = assign51370_e85355;
        locals.var_vfb2_dn3 = assign51370_e85355_d_n3;
        locals.var_vfb2_dn4 = assign51370_e85355_d_n4;
        locals.var_vfb2_dn5 = assign51370_e85355_d_n5;
        locals.var_vfb2_dn6 = assign51370_e85355_d_n6;
        locals.var_vfb2_dn7 = assign51370_e85355_d_n7;
        locals.var_vfb2_dn8 = assign51370_e85355_d_n8;
        locals.var_vfb2_dn9 = assign51370_e85355_d_n9;
        locals.var_vfb2_dn10 = assign51370_e85355_d_n10;
        locals.var_vfb2_dn11 = assign51370_e85355_d_n11;
        locals.var_vfb2_rv = 0.0;

        let assign51380_e85378: f64 = if (((((p.p43 != 0.0) && true) && (!((p.p40 != 0.0) && (!true)))) && (p.p45 == 1.0)) && (p.p1380 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard782 = assign51380_e85378;
        locals.var_guard782_rv = 0.0;

        let (assign51390_e85389, assign51390_e85389_d_n8, assign51390_e85389_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51390_e85387: f64 = (locals.var_devsign * (nv8 - nv11));
        (assign51390_e85387, locals.var_devsign, (-locals.var_devsign),)
    } else {
        (locals.var_vgb, locals.var_vgb_dn8, locals.var_vgb_dn11,)
    }
};
        locals.var_vgb = assign51390_e85389;
        locals.var_vgb_dn8 = assign51390_e85389_d_n8;
        locals.var_vgb_dn11 = assign51390_e85389_d_n11;
        locals.var_vgb_rv = 0.0;

        let (assign51400_e85400, assign51400_e85400_d_n3, assign51400_e85400_d_n4, assign51400_e85400_d_n5, assign51400_e85400_d_n6, assign51400_e85400_d_n7, assign51400_e85400_d_n8, assign51400_e85400_d_n9, assign51400_e85400_d_n10, assign51400_e85400_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51400_e85398: f64 = (locals.var_vgb - locals.var_vfb2);
        (assign51400_e85398, (-locals.var_vfb2_dn3), (-locals.var_vfb2_dn4), (-locals.var_vfb2_dn5), (-locals.var_vfb2_dn6), (-locals.var_vfb2_dn7), (locals.var_vgb_dn8 - locals.var_vfb2_dn8), (-locals.var_vfb2_dn9), (-locals.var_vfb2_dn10), (locals.var_vgb_dn11 - locals.var_vfb2_dn11),)
    } else {
        (locals.var_t0, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11,)
    }
};
        locals.var_t0 = assign51400_e85400;
        locals.var_t0_dn3 = assign51400_e85400_d_n3;
        locals.var_t0_dn4 = assign51400_e85400_d_n4;
        locals.var_t0_dn5 = assign51400_e85400_d_n5;
        locals.var_t0_dn6 = assign51400_e85400_d_n6;
        locals.var_t0_dn7 = assign51400_e85400_d_n7;
        locals.var_t0_dn8 = assign51400_e85400_d_n8;
        locals.var_t0_dn9 = assign51400_e85400_d_n9;
        locals.var_t0_dn10 = assign51400_e85400_d_n10;
        locals.var_t0_dn11 = assign51400_e85400_d_n11;
        locals.var_t0_rv = 0.0;

        let (assign51410_e85414, assign51410_e85414_d_n3, assign51410_e85414_d_n4, assign51410_e85414_d_n5, assign51410_e85414_d_n6, assign51410_e85414_d_n7, assign51410_e85414_d_n8, assign51410_e85414_d_n9, assign51410_e85414_d_n10, assign51410_e85414_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51410_e85409: f64 = (locals.var_t0 * locals.var_t0);
        let assign51410_e85411: f64 = (assign51410_e85409 + 0.0001);
        let assign51410_e85412: f64 = (assign51410_e85411).sqrt();
        (assign51410_e85412, (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / (2.0 * assign51410_e85412)), (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / (2.0 * assign51410_e85412)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51410_e85414;
        locals.var_t1_dn3 = assign51410_e85414_d_n3;
        locals.var_t1_dn4 = assign51410_e85414_d_n4;
        locals.var_t1_dn5 = assign51410_e85414_d_n5;
        locals.var_t1_dn6 = assign51410_e85414_d_n6;
        locals.var_t1_dn7 = assign51410_e85414_d_n7;
        locals.var_t1_dn8 = assign51410_e85414_d_n8;
        locals.var_t1_dn9 = assign51410_e85414_d_n9;
        locals.var_t1_dn10 = assign51410_e85414_d_n10;
        locals.var_t1_dn11 = assign51410_e85414_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign51420_e85430, assign51420_e85430_d_n3, assign51420_e85430_d_n4, assign51420_e85430_d_n5, assign51420_e85430_d_n6, assign51420_e85430_d_n7, assign51420_e85430_d_n8, assign51420_e85430_d_n9, assign51420_e85430_d_n10, assign51420_e85430_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51420_e85423: f64 = (-locals.var_t0);
        let assign51420_e85425: f64 = (assign51420_e85423 + locals.var_t1);
        let assign51420_e85427: f64 = (assign51420_e85425 - 0.01);
        let assign51420_e85428: f64 = (0.5 * assign51420_e85427);
        (assign51420_e85428, (0.5 * ((-locals.var_t0_dn3) + locals.var_t1_dn3)), (0.5 * ((-locals.var_t0_dn4) + locals.var_t1_dn4)), (0.5 * ((-locals.var_t0_dn5) + locals.var_t1_dn5)), (0.5 * ((-locals.var_t0_dn6) + locals.var_t1_dn6)), (0.5 * ((-locals.var_t0_dn7) + locals.var_t1_dn7)), (0.5 * ((-locals.var_t0_dn8) + locals.var_t1_dn8)), (0.5 * ((-locals.var_t0_dn9) + locals.var_t1_dn9)), (0.5 * ((-locals.var_t0_dn10) + locals.var_t1_dn10)), (0.5 * ((-locals.var_t0_dn11) + locals.var_t1_dn11)),)
    } else {
        (locals.var_vgp_eff, locals.var_vgp_eff_dn3, locals.var_vgp_eff_dn4, locals.var_vgp_eff_dn5, locals.var_vgp_eff_dn6, locals.var_vgp_eff_dn7, locals.var_vgp_eff_dn8, locals.var_vgp_eff_dn9, locals.var_vgp_eff_dn10, locals.var_vgp_eff_dn11,)
    }
};
        locals.var_vgp_eff = assign51420_e85430;
        locals.var_vgp_eff_dn3 = assign51420_e85430_d_n3;
        locals.var_vgp_eff_dn4 = assign51420_e85430_d_n4;
        locals.var_vgp_eff_dn5 = assign51420_e85430_d_n5;
        locals.var_vgp_eff_dn6 = assign51420_e85430_d_n6;
        locals.var_vgp_eff_dn7 = assign51420_e85430_d_n7;
        locals.var_vgp_eff_dn8 = assign51420_e85430_d_n8;
        locals.var_vgp_eff_dn9 = assign51420_e85430_d_n9;
        locals.var_vgp_eff_dn10 = assign51420_e85430_d_n10;
        locals.var_vgp_eff_dn11 = assign51420_e85430_d_n11;
        locals.var_vgp_eff_rv = 0.0;

        let (assign51430_e85444, assign51430_e85444_d_n3, assign51430_e85444_d_n4, assign51430_e85444_d_n5, assign51430_e85444_d_n6, assign51430_e85444_d_n7, assign51430_e85444_d_n8, assign51430_e85444_d_n9, assign51430_e85444_d_n10, assign51430_e85444_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let (assign51430_e85442,) = {
            if (p.p30 == 1.0) {
                (p.p702,)
            } else {
                (p.p703,)
            }
        };
        (assign51430_e85442, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign51430_e85444;
        locals.var_t11_dn3 = assign51430_e85444_d_n3;
        locals.var_t11_dn4 = assign51430_e85444_d_n4;
        locals.var_t11_dn5 = assign51430_e85444_d_n5;
        locals.var_t11_dn6 = assign51430_e85444_d_n6;
        locals.var_t11_dn7 = assign51430_e85444_d_n7;
        locals.var_t11_dn8 = assign51430_e85444_d_n8;
        locals.var_t11_dn9 = assign51430_e85444_d_n9;
        locals.var_t11_dn10 = assign51430_e85444_d_n10;
        locals.var_t11_dn11 = assign51430_e85444_d_n11;
        locals.var_t11_rv = 0.0;

        let (assign51440_e85458, assign51440_e85458_d_n3, assign51440_e85458_d_n4, assign51440_e85458_d_n5, assign51440_e85458_d_n6, assign51440_e85458_d_n7, assign51440_e85458_d_n8, assign51440_e85458_d_n9, assign51440_e85458_d_n10, assign51440_e85458_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let (assign51440_e85456,) = {
            if (p.p30 == 1.0) {
                (p.p704,)
            } else {
                (p.p705,)
            }
        };
        (assign51440_e85456, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t12, locals.var_t12_dn3, locals.var_t12_dn4, locals.var_t12_dn5, locals.var_t12_dn6, locals.var_t12_dn7, locals.var_t12_dn8, locals.var_t12_dn9, locals.var_t12_dn10, locals.var_t12_dn11,)
    }
};
        locals.var_t12 = assign51440_e85458;
        locals.var_t12_dn3 = assign51440_e85458_d_n3;
        locals.var_t12_dn4 = assign51440_e85458_d_n4;
        locals.var_t12_dn5 = assign51440_e85458_d_n5;
        locals.var_t12_dn6 = assign51440_e85458_d_n6;
        locals.var_t12_dn7 = assign51440_e85458_d_n7;
        locals.var_t12_dn8 = assign51440_e85458_d_n8;
        locals.var_t12_dn9 = assign51440_e85458_d_n9;
        locals.var_t12_dn10 = assign51440_e85458_d_n10;
        locals.var_t12_dn11 = assign51440_e85458_d_n11;
        locals.var_t12_rv = 0.0;

        let (assign51450_e85469, assign51450_e85469_d_n3, assign51450_e85469_d_n4, assign51450_e85469_d_n5, assign51450_e85469_d_n6, assign51450_e85469_d_n7, assign51450_e85469_d_n8, assign51450_e85469_d_n9, assign51450_e85469_d_n10, assign51450_e85469_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51450_e85467: f64 = (locals.var_vgb * locals.var_vgp_eff);
        (assign51450_e85467, (locals.var_vgb * locals.var_vgp_eff_dn3), (locals.var_vgb * locals.var_vgp_eff_dn4), (locals.var_vgb * locals.var_vgp_eff_dn5), (locals.var_vgb * locals.var_vgp_eff_dn6), (locals.var_vgb * locals.var_vgp_eff_dn7), ((locals.var_vgb_dn8 * locals.var_vgp_eff) + (locals.var_vgb * locals.var_vgp_eff_dn8)), (locals.var_vgb * locals.var_vgp_eff_dn9), (locals.var_vgb * locals.var_vgp_eff_dn10), ((locals.var_vgb_dn11 * locals.var_vgp_eff) + (locals.var_vgb * locals.var_vgp_eff_dn11)),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51450_e85469;
        locals.var_t2_dn3 = assign51450_e85469_d_n3;
        locals.var_t2_dn4 = assign51450_e85469_d_n4;
        locals.var_t2_dn5 = assign51450_e85469_d_n5;
        locals.var_t2_dn6 = assign51450_e85469_d_n6;
        locals.var_t2_dn7 = assign51450_e85469_d_n7;
        locals.var_t2_dn8 = assign51450_e85469_d_n8;
        locals.var_t2_dn9 = assign51450_e85469_d_n9;
        locals.var_t2_dn10 = assign51450_e85469_d_n10;
        locals.var_t2_dn11 = assign51450_e85469_d_n11;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_178(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign51460_e85482, assign51460_e85482_d_n3, assign51460_e85482_d_n4, assign51460_e85482_d_n5, assign51460_e85482_d_n6, assign51460_e85482_d_n7, assign51460_e85482_d_n8, assign51460_e85482_d_n9, assign51460_e85482_d_n10, assign51460_e85482_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51460_e85478: f64 = (locals.var_aigbcp2_i * locals.var_cigbcp2_i);
        let assign51460_e85480: f64 = (assign51460_e85478 - locals.var_bigbcp2_i);
        (assign51460_e85480, 0.0, (locals.var_aigbcp2_i_dn4 * locals.var_cigbcp2_i), (locals.var_aigbcp2_i_dn5 * locals.var_cigbcp2_i), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign51460_e85482;
        locals.var_t3_dn3 = assign51460_e85482_d_n3;
        locals.var_t3_dn4 = assign51460_e85482_d_n4;
        locals.var_t3_dn5 = assign51460_e85482_d_n5;
        locals.var_t3_dn6 = assign51460_e85482_d_n6;
        locals.var_t3_dn7 = assign51460_e85482_d_n7;
        locals.var_t3_dn8 = assign51460_e85482_d_n8;
        locals.var_t3_dn9 = assign51460_e85482_d_n9;
        locals.var_t3_dn10 = assign51460_e85482_d_n10;
        locals.var_t3_dn11 = assign51460_e85482_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign51470_e85493, assign51470_e85493_d_n3, assign51470_e85493_d_n4, assign51470_e85493_d_n5, assign51470_e85493_d_n6, assign51470_e85493_d_n7, assign51470_e85493_d_n8, assign51470_e85493_d_n9, assign51470_e85493_d_n10, assign51470_e85493_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51470_e85491: f64 = (locals.var_bigbcp2_i * locals.var_cigbcp2_i);
        (assign51470_e85491, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign51470_e85493;
        locals.var_t4_dn3 = assign51470_e85493_d_n3;
        locals.var_t4_dn4 = assign51470_e85493_d_n4;
        locals.var_t4_dn5 = assign51470_e85493_d_n5;
        locals.var_t4_dn6 = assign51470_e85493_d_n6;
        locals.var_t4_dn7 = assign51470_e85493_d_n7;
        locals.var_t4_dn8 = assign51470_e85493_d_n8;
        locals.var_t4_dn9 = assign51470_e85493_d_n9;
        locals.var_t4_dn10 = assign51470_e85493_d_n10;
        locals.var_t4_dn11 = assign51470_e85493_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign51480_e85517, assign51480_e85517_d_n3, assign51480_e85517_d_n4, assign51480_e85517_d_n5, assign51480_e85517_d_n6, assign51480_e85517_d_n7, assign51480_e85517_d_n8, assign51480_e85517_d_n9, assign51480_e85517_d_n10, assign51480_e85517_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51480_e85501: f64 = (-locals.var_t12);
        let assign51480_e85503: f64 = (assign51480_e85501 * p.p76);
        let assign51480_e85507: f64 = (locals.var_t3 * locals.var_vgp_eff);
        let assign51480_e85508: f64 = (locals.var_aigbcp2_i + assign51480_e85507);
        let assign51480_e85511: f64 = (locals.var_t4 * locals.var_vgp_eff);
        let assign51480_e85513: f64 = (assign51480_e85511 * locals.var_vgp_eff);
        let assign51480_e85514: f64 = (assign51480_e85508 - assign51480_e85513);
        let assign51480_e85515: f64 = (assign51480_e85503 * assign51480_e85514);
        (assign51480_e85515, ((((-locals.var_t12_dn3) * p.p76) * assign51480_e85514) + (assign51480_e85503 * (((locals.var_t3_dn3 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn3)) - ((((locals.var_t4_dn3 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn3)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn3))))), ((((-locals.var_t12_dn4) * p.p76) * assign51480_e85514) + (assign51480_e85503 * ((locals.var_aigbcp2_i_dn4 + ((locals.var_t3_dn4 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn4))) - ((((locals.var_t4_dn4 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn4)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn4))))), ((((-locals.var_t12_dn5) * p.p76) * assign51480_e85514) + (assign51480_e85503 * ((locals.var_aigbcp2_i_dn5 + ((locals.var_t3_dn5 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn5))) - ((((locals.var_t4_dn5 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn5)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn5))))), ((((-locals.var_t12_dn6) * p.p76) * assign51480_e85514) + (assign51480_e85503 * (((locals.var_t3_dn6 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn6)) - ((((locals.var_t4_dn6 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn6)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn6))))), ((((-locals.var_t12_dn7) * p.p76) * assign51480_e85514) + (assign51480_e85503 * (((locals.var_t3_dn7 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn7)) - ((((locals.var_t4_dn7 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn7)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn7))))), ((((-locals.var_t12_dn8) * p.p76) * assign51480_e85514) + (assign51480_e85503 * (((locals.var_t3_dn8 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn8)) - ((((locals.var_t4_dn8 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn8)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn8))))), ((((-locals.var_t12_dn9) * p.p76) * assign51480_e85514) + (assign51480_e85503 * (((locals.var_t3_dn9 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn9)) - ((((locals.var_t4_dn9 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn9)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn9))))), ((((-locals.var_t12_dn10) * p.p76) * assign51480_e85514) + (assign51480_e85503 * (((locals.var_t3_dn10 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn10)) - ((((locals.var_t4_dn10 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn10)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn10))))), ((((-locals.var_t12_dn11) * p.p76) * assign51480_e85514) + (assign51480_e85503 * (((locals.var_t3_dn11 * locals.var_vgp_eff) + (locals.var_t3 * locals.var_vgp_eff_dn11)) - ((((locals.var_t4_dn11 * locals.var_vgp_eff) + (locals.var_t4 * locals.var_vgp_eff_dn11)) * locals.var_vgp_eff) + (assign51480_e85511 * locals.var_vgp_eff_dn11))))),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign51480_e85517;
        locals.var_t5_dn3 = assign51480_e85517_d_n3;
        locals.var_t5_dn4 = assign51480_e85517_d_n4;
        locals.var_t5_dn5 = assign51480_e85517_d_n5;
        locals.var_t5_dn6 = assign51480_e85517_d_n6;
        locals.var_t5_dn7 = assign51480_e85517_d_n7;
        locals.var_t5_dn8 = assign51480_e85517_d_n8;
        locals.var_t5_dn9 = assign51480_e85517_d_n9;
        locals.var_t5_dn10 = assign51480_e85517_d_n10;
        locals.var_t5_dn11 = assign51480_e85517_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign51490_e85527, assign51490_e85527_d_n3, assign51490_e85527_d_n4, assign51490_e85527_d_n5, assign51490_e85527_d_n6, assign51490_e85527_d_n7, assign51490_e85527_d_n8, assign51490_e85527_d_n9, assign51490_e85527_d_n10, assign51490_e85527_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51490_e85525: f64 = { let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign51490_e85525, ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn3), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn4), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn5), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn6), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn7), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn8), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn9), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn10), ({ let limited_exp_arg = locals.var_t5; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t5_dn11),)
    } else {
        (locals.var_t6, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11,)
    }
};
        locals.var_t6 = assign51490_e85527;
        locals.var_t6_dn3 = assign51490_e85527_d_n3;
        locals.var_t6_dn4 = assign51490_e85527_d_n4;
        locals.var_t6_dn5 = assign51490_e85527_d_n5;
        locals.var_t6_dn6 = assign51490_e85527_d_n6;
        locals.var_t6_dn7 = assign51490_e85527_d_n7;
        locals.var_t6_dn8 = assign51490_e85527_d_n8;
        locals.var_t6_dn9 = assign51490_e85527_d_n9;
        locals.var_t6_dn10 = assign51490_e85527_d_n10;
        locals.var_t6_dn11 = assign51490_e85527_d_n11;
        locals.var_t6_rv = 0.0;

        let (assign51500_e85540, assign51500_e85540_d_n3, assign51500_e85540_d_n4, assign51500_e85540_d_n5, assign51500_e85540_d_n6, assign51500_e85540_d_n7, assign51500_e85540_d_n8, assign51500_e85540_d_n9, assign51500_e85540_d_n10, assign51500_e85540_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard782 != 0.0)) {
        let assign51500_e85536: f64 = (locals.var_t11 * p.p1380);
        let assign51500_e85538: f64 = (assign51500_e85536 * locals.var_toxratio);
        (assign51500_e85538, (((locals.var_t11_dn3 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn3)), (((locals.var_t11_dn4 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn4)), (((locals.var_t11_dn5 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn5)), (((locals.var_t11_dn6 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn6)), (((locals.var_t11_dn7 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn7)), (((locals.var_t11_dn8 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn8)), (((locals.var_t11_dn9 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn9)), (((locals.var_t11_dn10 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn10)), (((locals.var_t11_dn11 * p.p1380) * locals.var_toxratio) + (assign51500_e85536 * locals.var_toxratio_dn11)),)
    } else {
        (locals.var_t11, locals.var_t11_dn3, locals.var_t11_dn4, locals.var_t11_dn5, locals.var_t11_dn6, locals.var_t11_dn7, locals.var_t11_dn8, locals.var_t11_dn9, locals.var_t11_dn10, locals.var_t11_dn11,)
    }
};
        locals.var_t11 = assign51500_e85540;
        locals.var_t11_dn3 = assign51500_e85540_d_n3;
        locals.var_t11_dn4 = assign51500_e85540_d_n4;
        locals.var_t11_dn5 = assign51500_e85540_d_n5;
        locals.var_t11_dn6 = assign51500_e85540_d_n6;
        locals.var_t11_dn7 = assign51500_e85540_d_n7;
        locals.var_t11_dn8 = assign51500_e85540_d_n8;
        locals.var_t11_dn9 = assign51500_e85540_d_n9;
        locals.var_t11_dn10 = assign51500_e85540_d_n10;
        locals.var_t11_dn11 = assign51500_e85540_d_n11;
        locals.var_t11_rv = 0.0;

        let assign51530_e85568: f64 = if p.p37 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard783 = assign51530_e85568;
        locals.var_guard783_rv = 0.0;

        let (assign51540_e85581, assign51540_e85581_d_n3, assign51540_e85581_d_n4, assign51540_e85581_d_n5, assign51540_e85581_d_n6, assign51540_e85581_d_n7, assign51540_e85581_d_n8, assign51540_e85581_d_n9, assign51540_e85581_d_n10, assign51540_e85581_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51540_e85578: f64 = (locals.var_bigc_i * locals.var_voxminv);
        let assign51540_e85579: f64 = (locals.var_aigc_i - assign51540_e85578);
        (assign51540_e85579, (-(locals.var_bigc_i * locals.var_voxminv_dn3)), (locals.var_aigc_i_dn4 - (locals.var_bigc_i * locals.var_voxminv_dn4)), (locals.var_aigc_i_dn5 - (locals.var_bigc_i * locals.var_voxminv_dn5)), (-(locals.var_bigc_i * locals.var_voxminv_dn6)), (-(locals.var_bigc_i * locals.var_voxminv_dn7)), (-(locals.var_bigc_i * locals.var_voxminv_dn8)), (-(locals.var_bigc_i * locals.var_voxminv_dn9)), (-(locals.var_bigc_i * locals.var_voxminv_dn10)), (-(locals.var_bigc_i * locals.var_voxminv_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51540_e85581;
        locals.var_t1_dn3 = assign51540_e85581_d_n3;
        locals.var_t1_dn4 = assign51540_e85581_d_n4;
        locals.var_t1_dn5 = assign51540_e85581_d_n5;
        locals.var_t1_dn6 = assign51540_e85581_d_n6;
        locals.var_t1_dn7 = assign51540_e85581_d_n7;
        locals.var_t1_dn8 = assign51540_e85581_d_n8;
        locals.var_t1_dn9 = assign51540_e85581_d_n9;
        locals.var_t1_dn10 = assign51540_e85581_d_n10;
        locals.var_t1_dn11 = assign51540_e85581_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign51550_e85594, assign51550_e85594_d_n3, assign51550_e85594_d_n4, assign51550_e85594_d_n5, assign51550_e85594_d_n6, assign51550_e85594_d_n7, assign51550_e85594_d_n8, assign51550_e85594_d_n9, assign51550_e85594_d_n10, assign51550_e85594_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51550_e85591: f64 = (locals.var_cigc_i * locals.var_voxminv);
        let assign51550_e85592: f64 = (1.0 + assign51550_e85591);
        (assign51550_e85592, (locals.var_cigc_i * locals.var_voxminv_dn3), (locals.var_cigc_i * locals.var_voxminv_dn4), (locals.var_cigc_i * locals.var_voxminv_dn5), (locals.var_cigc_i * locals.var_voxminv_dn6), (locals.var_cigc_i * locals.var_voxminv_dn7), (locals.var_cigc_i * locals.var_voxminv_dn8), (locals.var_cigc_i * locals.var_voxminv_dn9), (locals.var_cigc_i * locals.var_voxminv_dn10), (locals.var_cigc_i * locals.var_voxminv_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51550_e85594;
        locals.var_t2_dn3 = assign51550_e85594_d_n3;
        locals.var_t2_dn4 = assign51550_e85594_d_n4;
        locals.var_t2_dn5 = assign51550_e85594_d_n5;
        locals.var_t2_dn6 = assign51550_e85594_d_n6;
        locals.var_t2_dn7 = assign51550_e85594_d_n7;
        locals.var_t2_dn8 = assign51550_e85594_d_n8;
        locals.var_t2_dn9 = assign51550_e85594_d_n9;
        locals.var_t2_dn10 = assign51550_e85594_d_n10;
        locals.var_t2_dn11 = assign51550_e85594_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign51560_e85607, assign51560_e85607_d_n3, assign51560_e85607_d_n4, assign51560_e85607_d_n5, assign51560_e85607_d_n6, assign51560_e85607_d_n7, assign51560_e85607_d_n8, assign51560_e85607_d_n9, assign51560_e85607_d_n10, assign51560_e85607_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51560_e85603: f64 = (locals.var_bechvb * locals.var_t1);
        let assign51560_e85605: f64 = (assign51560_e85603 * locals.var_t2);
        (assign51560_e85605, (((locals.var_bechvb * locals.var_t1_dn3) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn3)), (((locals.var_bechvb * locals.var_t1_dn4) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn4)), (((locals.var_bechvb * locals.var_t1_dn5) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn5)), (((locals.var_bechvb * locals.var_t1_dn6) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn6)), (((locals.var_bechvb * locals.var_t1_dn7) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn7)), (((locals.var_bechvb * locals.var_t1_dn8) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn8)), (((locals.var_bechvb * locals.var_t1_dn9) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn9)), (((locals.var_bechvb * locals.var_t1_dn10) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn10)), (((locals.var_bechvb * locals.var_t1_dn11) * locals.var_t2) + (assign51560_e85603 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign51560_e85607;
        locals.var_t3_dn3 = assign51560_e85607_d_n3;
        locals.var_t3_dn4 = assign51560_e85607_d_n4;
        locals.var_t3_dn5 = assign51560_e85607_d_n5;
        locals.var_t3_dn6 = assign51560_e85607_d_n6;
        locals.var_t3_dn7 = assign51560_e85607_d_n7;
        locals.var_t3_dn8 = assign51560_e85607_d_n8;
        locals.var_t3_dn9 = assign51560_e85607_d_n9;
        locals.var_t3_dn10 = assign51560_e85607_d_n10;
        locals.var_t3_dn11 = assign51560_e85607_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign51570_e85625, assign51570_e85625_d_n3, assign51570_e85625_d_n4, assign51570_e85625_d_n5, assign51570_e85625_d_n6, assign51570_e85625_d_n7, assign51570_e85625_d_n8, assign51570_e85625_d_n9, assign51570_e85625_d_n10, assign51570_e85625_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51570_e85616: f64 = (locals.var_nq * locals.var_nvt);
        let assign51570_e85619: f64 = (locals.var_qs_1 + locals.var_qdeff);
        let assign51570_e85620: f64 = (assign51570_e85616 * assign51570_e85619);
        let assign51570_e85622: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign51570_e85623: f64 = (assign51570_e85620 * assign51570_e85622);
        (assign51570_e85623, ((((((locals.var_nq_dn3 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn3)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn3 + locals.var_qdeff_dn3))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3))), ((((((locals.var_nq_dn4 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn4)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn4 + locals.var_qdeff_dn4))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4))), ((((((locals.var_nq_dn5 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn5)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn5 + locals.var_qdeff_dn5))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5))), ((((((locals.var_nq_dn6 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn6)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn6 + locals.var_qdeff_dn6))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6))), ((((((locals.var_nq_dn7 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn7)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn7 + locals.var_qdeff_dn7))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7))), ((((((locals.var_nq_dn8 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn8)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn8 + locals.var_qdeff_dn8))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8))), ((((((locals.var_nq_dn9 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn9)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn9 + locals.var_qdeff_dn9))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9))), ((((((locals.var_nq_dn10 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn10)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn10 + locals.var_qdeff_dn10))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10))), ((((((locals.var_nq_dn11 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn11)) * assign51570_e85619) + (assign51570_e85616 * (locals.var_qs_1_dn11 + locals.var_qdeff_dn11))) * assign51570_e85622) + (assign51570_e85620 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign51570_e85625;
        locals.var_t4_dn3 = assign51570_e85625_d_n3;
        locals.var_t4_dn4 = assign51570_e85625_d_n4;
        locals.var_t4_dn5 = assign51570_e85625_d_n5;
        locals.var_t4_dn6 = assign51570_e85625_d_n6;
        locals.var_t4_dn7 = assign51570_e85625_d_n7;
        locals.var_t4_dn8 = assign51570_e85625_d_n8;
        locals.var_t4_dn9 = assign51570_e85625_d_n9;
        locals.var_t4_dn10 = assign51570_e85625_d_n10;
        locals.var_t4_dn11 = assign51570_e85625_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign51590_e85668, assign51590_e85668_d_n3, assign51590_e85668_d_n4, assign51590_e85668_d_n5, assign51590_e85668_d_n6, assign51590_e85668_d_n7, assign51590_e85668_d_n8, assign51590_e85668_d_n9, assign51590_e85668_d_n10, assign51590_e85668_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51590_e85661: f64 = (locals.var_vdseff * locals.var_vdseff);
        let assign51590_e85663: f64 = (assign51590_e85661 + 0.01);
        let assign51590_e85664: f64 = (assign51590_e85663).sqrt();
        let assign51590_e85666: f64 = (assign51590_e85664 - 0.1);
        (assign51590_e85666, (((locals.var_vdseff_dn3 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn3)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn4 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn4)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn5 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn5)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn6 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn6)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn7 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn7)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn8 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn8)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn9 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn9)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn10 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn10)) / (2.0 * assign51590_e85664)), (((locals.var_vdseff_dn11 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn11)) / (2.0 * assign51590_e85664)),)
    } else {
        (locals.var_vdseffx, locals.var_vdseffx_dn3, locals.var_vdseffx_dn4, locals.var_vdseffx_dn5, locals.var_vdseffx_dn6, locals.var_vdseffx_dn7, locals.var_vdseffx_dn8, locals.var_vdseffx_dn9, locals.var_vdseffx_dn10, locals.var_vdseffx_dn11,)
    }
};
        locals.var_vdseffx = assign51590_e85668;
        locals.var_vdseffx_dn3 = assign51590_e85668_d_n3;
        locals.var_vdseffx_dn4 = assign51590_e85668_d_n4;
        locals.var_vdseffx_dn5 = assign51590_e85668_d_n5;
        locals.var_vdseffx_dn6 = assign51590_e85668_d_n6;
        locals.var_vdseffx_dn7 = assign51590_e85668_d_n7;
        locals.var_vdseffx_dn8 = assign51590_e85668_d_n8;
        locals.var_vdseffx_dn9 = assign51590_e85668_d_n9;
        locals.var_vdseffx_dn10 = assign51590_e85668_d_n10;
        locals.var_vdseffx_dn11 = assign51590_e85668_d_n11;
        locals.var_vdseffx_rv = 0.0;

        let (assign51600_e85679, assign51600_e85679_d_n3, assign51600_e85679_d_n4, assign51600_e85679_d_n5, assign51600_e85679_d_n6, assign51600_e85679_d_n7, assign51600_e85679_d_n8, assign51600_e85679_d_n9, assign51600_e85679_d_n10, assign51600_e85679_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51600_e85677: f64 = (locals.var_pigcd_i * locals.var_vdseffx);
        (assign51600_e85677, (locals.var_pigcd_i * locals.var_vdseffx_dn3), (locals.var_pigcd_i * locals.var_vdseffx_dn4), (locals.var_pigcd_i * locals.var_vdseffx_dn5), (locals.var_pigcd_i * locals.var_vdseffx_dn6), (locals.var_pigcd_i * locals.var_vdseffx_dn7), (locals.var_pigcd_i * locals.var_vdseffx_dn8), (locals.var_pigcd_i * locals.var_vdseffx_dn9), (locals.var_pigcd_i * locals.var_vdseffx_dn10), (locals.var_pigcd_i * locals.var_vdseffx_dn11),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51600_e85679;
        locals.var_t1_dn3 = assign51600_e85679_d_n3;
        locals.var_t1_dn4 = assign51600_e85679_d_n4;
        locals.var_t1_dn5 = assign51600_e85679_d_n5;
        locals.var_t1_dn6 = assign51600_e85679_d_n6;
        locals.var_t1_dn7 = assign51600_e85679_d_n7;
        locals.var_t1_dn8 = assign51600_e85679_d_n8;
        locals.var_t1_dn9 = assign51600_e85679_d_n9;
        locals.var_t1_dn10 = assign51600_e85679_d_n10;
        locals.var_t1_dn11 = assign51600_e85679_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign51610_e85690, assign51610_e85690_d_n3, assign51610_e85690_d_n4, assign51610_e85690_d_n5, assign51610_e85690_d_n6, assign51610_e85690_d_n7, assign51610_e85690_d_n8, assign51610_e85690_d_n9, assign51610_e85690_d_n10, assign51610_e85690_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51610_e85687: f64 = (-locals.var_t1);
        let assign51610_e85688: f64 = { let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign51610_e85688, ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn3)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn4)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn5)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn6)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn7)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn8)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn9)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn10)), ({ let limited_exp_arg = assign51610_e85687; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn11)),)
    } else {
        (locals.var_t1_exp, locals.var_t1_exp_dn3, locals.var_t1_exp_dn4, locals.var_t1_exp_dn5, locals.var_t1_exp_dn6, locals.var_t1_exp_dn7, locals.var_t1_exp_dn8, locals.var_t1_exp_dn9, locals.var_t1_exp_dn10, locals.var_t1_exp_dn11,)
    }
};
        locals.var_t1_exp = assign51610_e85690;
        locals.var_t1_exp_dn3 = assign51610_e85690_d_n3;
        locals.var_t1_exp_dn4 = assign51610_e85690_d_n4;
        locals.var_t1_exp_dn5 = assign51610_e85690_d_n5;
        locals.var_t1_exp_dn6 = assign51610_e85690_d_n6;
        locals.var_t1_exp_dn7 = assign51610_e85690_d_n7;
        locals.var_t1_exp_dn8 = assign51610_e85690_d_n8;
        locals.var_t1_exp_dn9 = assign51610_e85690_d_n9;
        locals.var_t1_exp_dn10 = assign51610_e85690_d_n10;
        locals.var_t1_exp_dn11 = assign51610_e85690_d_n11;
        locals.var_t1_exp_rv = 0.0;

        let (assign51620_e85705, assign51620_e85705_d_n3, assign51620_e85705_d_n4, assign51620_e85705_d_n5, assign51620_e85705_d_n6, assign51620_e85705_d_n7, assign51620_e85705_d_n8, assign51620_e85705_d_n9, assign51620_e85705_d_n10, assign51620_e85705_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51620_e85699: f64 = (locals.var_t1 + locals.var_t1_exp);
        let assign51620_e85701: f64 = (assign51620_e85699 - 1.0);
        let assign51620_e85703: f64 = (assign51620_e85701 + 0.0001);
        (assign51620_e85703, (locals.var_t1_dn3 + locals.var_t1_exp_dn3), (locals.var_t1_dn4 + locals.var_t1_exp_dn4), (locals.var_t1_dn5 + locals.var_t1_exp_dn5), (locals.var_t1_dn6 + locals.var_t1_exp_dn6), (locals.var_t1_dn7 + locals.var_t1_exp_dn7), (locals.var_t1_dn8 + locals.var_t1_exp_dn8), (locals.var_t1_dn9 + locals.var_t1_exp_dn9), (locals.var_t1_dn10 + locals.var_t1_exp_dn10), (locals.var_t1_dn11 + locals.var_t1_exp_dn11),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign51620_e85705;
        locals.var_t3_dn3 = assign51620_e85705_d_n3;
        locals.var_t3_dn4 = assign51620_e85705_d_n4;
        locals.var_t3_dn5 = assign51620_e85705_d_n5;
        locals.var_t3_dn6 = assign51620_e85705_d_n6;
        locals.var_t3_dn7 = assign51620_e85705_d_n7;
        locals.var_t3_dn8 = assign51620_e85705_d_n8;
        locals.var_t3_dn9 = assign51620_e85705_d_n9;
        locals.var_t3_dn10 = assign51620_e85705_d_n10;
        locals.var_t3_dn11 = assign51620_e85705_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign51630_e85722, assign51630_e85722_d_n3, assign51630_e85722_d_n4, assign51630_e85722_d_n5, assign51630_e85722_d_n6, assign51630_e85722_d_n7, assign51630_e85722_d_n8, assign51630_e85722_d_n9, assign51630_e85722_d_n10, assign51630_e85722_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51630_e85715: f64 = (locals.var_t1 + 1.0);
        let assign51630_e85717: f64 = (assign51630_e85715 * locals.var_t1_exp);
        let assign51630_e85718: f64 = (1.0 - assign51630_e85717);
        let assign51630_e85720: f64 = (assign51630_e85718 + 0.0001);
        (assign51630_e85720, (-((locals.var_t1_dn3 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn3))), (-((locals.var_t1_dn4 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn4))), (-((locals.var_t1_dn5 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn5))), (-((locals.var_t1_dn6 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn6))), (-((locals.var_t1_dn7 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn7))), (-((locals.var_t1_dn8 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn8))), (-((locals.var_t1_dn9 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn9))), (-((locals.var_t1_dn10 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn10))), (-((locals.var_t1_dn11 * locals.var_t1_exp) + (assign51630_e85715 * locals.var_t1_exp_dn11))),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign51630_e85722;
        locals.var_t4_dn3 = assign51630_e85722_d_n3;
        locals.var_t4_dn4 = assign51630_e85722_d_n4;
        locals.var_t4_dn5 = assign51630_e85722_d_n5;
        locals.var_t4_dn6 = assign51630_e85722_d_n6;
        locals.var_t4_dn7 = assign51630_e85722_d_n7;
        locals.var_t4_dn8 = assign51630_e85722_d_n8;
        locals.var_t4_dn9 = assign51630_e85722_d_n9;
        locals.var_t4_dn10 = assign51630_e85722_d_n10;
        locals.var_t4_dn11 = assign51630_e85722_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign51640_e85735, assign51640_e85735_d_n3, assign51640_e85735_d_n4, assign51640_e85735_d_n5, assign51640_e85735_d_n6, assign51640_e85735_d_n7, assign51640_e85735_d_n8, assign51640_e85735_d_n9, assign51640_e85735_d_n10, assign51640_e85735_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51640_e85731: f64 = (locals.var_t1 * locals.var_t1);
        let assign51640_e85733: f64 = (assign51640_e85731 + 0.0002);
        (assign51640_e85733, ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)),)
    } else {
        (locals.var_t5, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11,)
    }
};
        locals.var_t5 = assign51640_e85735;
        locals.var_t5_dn3 = assign51640_e85735_d_n3;
        locals.var_t5_dn4 = assign51640_e85735_d_n4;
        locals.var_t5_dn5 = assign51640_e85735_d_n5;
        locals.var_t5_dn6 = assign51640_e85735_d_n6;
        locals.var_t5_dn7 = assign51640_e85735_d_n7;
        locals.var_t5_dn8 = assign51640_e85735_d_n8;
        locals.var_t5_dn9 = assign51640_e85735_d_n9;
        locals.var_t5_dn10 = assign51640_e85735_d_n10;
        locals.var_t5_dn11 = assign51640_e85735_d_n11;
        locals.var_t5_rv = 0.0;

        let (assign51700_e85811, assign51700_e85811_d_n3, assign51700_e85811_d_n4, assign51700_e85811_d_n5, assign51700_e85811_d_n6, assign51700_e85811_d_n7, assign51700_e85811_d_n8, assign51700_e85811_d_n9, assign51700_e85811_d_n10, assign51700_e85811_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51700_e85809: f64 = (locals.var_vgs_noswap - locals.var_vfbsdr);
        (assign51700_e85809, 0.0, (-locals.var_vfbsdr_dn4), (-locals.var_vfbsdr_dn5), locals.var_vgs_noswap_dn6, locals.var_vgs_noswap_dn7, locals.var_vgs_noswap_dn8, 0.0, locals.var_vgs_noswap_dn10, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51700_e85811;
        locals.var_t2_dn3 = assign51700_e85811_d_n3;
        locals.var_t2_dn4 = assign51700_e85811_d_n4;
        locals.var_t2_dn5 = assign51700_e85811_d_n5;
        locals.var_t2_dn6 = assign51700_e85811_d_n6;
        locals.var_t2_dn7 = assign51700_e85811_d_n7;
        locals.var_t2_dn8 = assign51700_e85811_d_n8;
        locals.var_t2_dn9 = assign51700_e85811_d_n9;
        locals.var_t2_dn10 = assign51700_e85811_d_n10;
        locals.var_t2_dn11 = assign51700_e85811_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign51710_e85825, assign51710_e85825_d_n3, assign51710_e85825_d_n4, assign51710_e85825_d_n5, assign51710_e85825_d_n6, assign51710_e85825_d_n7, assign51710_e85825_d_n8, assign51710_e85825_d_n9, assign51710_e85825_d_n10, assign51710_e85825_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51710_e85820: f64 = (locals.var_t2 * locals.var_t2);
        let assign51710_e85822: f64 = (assign51710_e85820 + 0.0001);
        let assign51710_e85823: f64 = (assign51710_e85822).sqrt();
        (assign51710_e85823, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign51710_e85823)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign51710_e85823)),)
    } else {
        (locals.var_vgs_eff, locals.var_vgs_eff_dn3, locals.var_vgs_eff_dn4, locals.var_vgs_eff_dn5, locals.var_vgs_eff_dn6, locals.var_vgs_eff_dn7, locals.var_vgs_eff_dn8, locals.var_vgs_eff_dn9, locals.var_vgs_eff_dn10, locals.var_vgs_eff_dn11,)
    }
};
        locals.var_vgs_eff = assign51710_e85825;
        locals.var_vgs_eff_dn3 = assign51710_e85825_d_n3;
        locals.var_vgs_eff_dn4 = assign51710_e85825_d_n4;
        locals.var_vgs_eff_dn5 = assign51710_e85825_d_n5;
        locals.var_vgs_eff_dn6 = assign51710_e85825_d_n6;
        locals.var_vgs_eff_dn7 = assign51710_e85825_d_n7;
        locals.var_vgs_eff_dn8 = assign51710_e85825_d_n8;
        locals.var_vgs_eff_dn9 = assign51710_e85825_d_n9;
        locals.var_vgs_eff_dn10 = assign51710_e85825_d_n10;
        locals.var_vgs_eff_dn11 = assign51710_e85825_d_n11;
        locals.var_vgs_eff_rv = 0.0;

        let assign51720_e85828: f64 = if p.p1295 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard785 = assign51720_e85828;
        locals.var_guard785_rv = 0.0;

        let (assign51730_e85864, assign51730_e85864_d_n3, assign51730_e85864_d_n4, assign51730_e85864_d_n5, assign51730_e85864_d_n6, assign51730_e85864_d_n7, assign51730_e85864_d_n8, assign51730_e85864_d_n9, assign51730_e85864_d_n10, assign51730_e85864_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard785 != 0.0)) {
        let assign51730_e85841: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
        let assign51730_e85842: f64 = (locals.var_aigs_i - assign51730_e85841);
        let assign51730_e85846: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
        let assign51730_e85847: f64 = (locals.var_aigs_i - assign51730_e85846);
        let assign51730_e85851: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
        let assign51730_e85852: f64 = (locals.var_aigs_i - assign51730_e85851);
        let assign51730_e85853: f64 = (assign51730_e85847 * assign51730_e85852);
        let assign51730_e85856: f64 = (4.0 * 1e-6);
        let assign51730_e85858: f64 = (assign51730_e85856 * 1e-6);
        let assign51730_e85859: f64 = (assign51730_e85853 + assign51730_e85858);
        let assign51730_e85860: f64 = (assign51730_e85859).sqrt();
        let assign51730_e85861: f64 = (assign51730_e85842 + assign51730_e85860);
        let assign51730_e85862: f64 = (0.5 * assign51730_e85861);
        (assign51730_e85862, (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn3)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn3)) * assign51730_e85852) + (assign51730_e85847 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn3)))) / (2.0 * assign51730_e85860)))), (0.5 * ((locals.var_aigs_i_dn4 - (locals.var_bigs_i * locals.var_vgs_eff_dn4)) + ((((locals.var_aigs_i_dn4 - (locals.var_bigs_i * locals.var_vgs_eff_dn4)) * assign51730_e85852) + (assign51730_e85847 * (locals.var_aigs_i_dn4 - (locals.var_bigs_i * locals.var_vgs_eff_dn4)))) / (2.0 * assign51730_e85860)))), (0.5 * ((locals.var_aigs_i_dn5 - (locals.var_bigs_i * locals.var_vgs_eff_dn5)) + ((((locals.var_aigs_i_dn5 - (locals.var_bigs_i * locals.var_vgs_eff_dn5)) * assign51730_e85852) + (assign51730_e85847 * (locals.var_aigs_i_dn5 - (locals.var_bigs_i * locals.var_vgs_eff_dn5)))) / (2.0 * assign51730_e85860)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn6)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn6)) * assign51730_e85852) + (assign51730_e85847 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn6)))) / (2.0 * assign51730_e85860)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn7)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn7)) * assign51730_e85852) + (assign51730_e85847 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn7)))) / (2.0 * assign51730_e85860)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn8)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn8)) * assign51730_e85852) + (assign51730_e85847 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn8)))) / (2.0 * assign51730_e85860)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn9)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn9)) * assign51730_e85852) + (assign51730_e85847 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn9)))) / (2.0 * assign51730_e85860)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn10)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn10)) * assign51730_e85852) + (assign51730_e85847 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn10)))) / (2.0 * assign51730_e85860)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn11)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn11)) * assign51730_e85852) + (assign51730_e85847 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn11)))) / (2.0 * assign51730_e85860)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51730_e85864;
        locals.var_t1_dn3 = assign51730_e85864_d_n3;
        locals.var_t1_dn4 = assign51730_e85864_d_n4;
        locals.var_t1_dn5 = assign51730_e85864_d_n5;
        locals.var_t1_dn6 = assign51730_e85864_d_n6;
        locals.var_t1_dn7 = assign51730_e85864_d_n7;
        locals.var_t1_dn8 = assign51730_e85864_d_n8;
        locals.var_t1_dn9 = assign51730_e85864_d_n9;
        locals.var_t1_dn10 = assign51730_e85864_d_n10;
        locals.var_t1_dn11 = assign51730_e85864_d_n11;
        locals.var_t1_rv = 0.0;

        let assign51740_e85867: f64 = if locals.var_cigs_i < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard786 = assign51740_e85867;
        locals.var_guard786_rv = 0.0;

        let (assign51750_e85880,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard785 != 0.0)) && (locals.var_guard786 != 0.0)) {
        (0.01,)
    } else {
        (locals.var_cigs_i,)
    }
};
        locals.var_cigs_i = assign51750_e85880;
        locals.var_cigs_i_rv = 0.0;

        let (assign51760_e85896, assign51760_e85896_d_n3, assign51760_e85896_d_n4, assign51760_e85896_d_n5, assign51760_e85896_d_n6, assign51760_e85896_d_n7, assign51760_e85896_d_n8, assign51760_e85896_d_n9, assign51760_e85896_d_n10, assign51760_e85896_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard785 == 0.0)) {
        let assign51760_e85893: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
        let assign51760_e85894: f64 = (locals.var_aigs_i - assign51760_e85893);
        (assign51760_e85894, (-(locals.var_bigs_i * locals.var_vgs_eff_dn3)), (locals.var_aigs_i_dn4 - (locals.var_bigs_i * locals.var_vgs_eff_dn4)), (locals.var_aigs_i_dn5 - (locals.var_bigs_i * locals.var_vgs_eff_dn5)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn6)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn7)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn8)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn9)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn10)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51760_e85896;
        locals.var_t1_dn3 = assign51760_e85896_d_n3;
        locals.var_t1_dn4 = assign51760_e85896_d_n4;
        locals.var_t1_dn5 = assign51760_e85896_d_n5;
        locals.var_t1_dn6 = assign51760_e85896_d_n6;
        locals.var_t1_dn7 = assign51760_e85896_d_n7;
        locals.var_t1_dn8 = assign51760_e85896_d_n8;
        locals.var_t1_dn9 = assign51760_e85896_d_n9;
        locals.var_t1_dn10 = assign51760_e85896_d_n10;
        locals.var_t1_dn11 = assign51760_e85896_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign51770_e85909, assign51770_e85909_d_n3, assign51770_e85909_d_n4, assign51770_e85909_d_n5, assign51770_e85909_d_n6, assign51770_e85909_d_n7, assign51770_e85909_d_n8, assign51770_e85909_d_n9, assign51770_e85909_d_n10, assign51770_e85909_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51770_e85906: f64 = (locals.var_cigs_i * locals.var_vgs_eff);
        let assign51770_e85907: f64 = (1.0 + assign51770_e85906);
        (assign51770_e85907, (locals.var_cigs_i * locals.var_vgs_eff_dn3), (locals.var_cigs_i * locals.var_vgs_eff_dn4), (locals.var_cigs_i * locals.var_vgs_eff_dn5), (locals.var_cigs_i * locals.var_vgs_eff_dn6), (locals.var_cigs_i * locals.var_vgs_eff_dn7), (locals.var_cigs_i * locals.var_vgs_eff_dn8), (locals.var_cigs_i * locals.var_vgs_eff_dn9), (locals.var_cigs_i * locals.var_vgs_eff_dn10), (locals.var_cigs_i * locals.var_vgs_eff_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51770_e85909;
        locals.var_t2_dn3 = assign51770_e85909_d_n3;
        locals.var_t2_dn4 = assign51770_e85909_d_n4;
        locals.var_t2_dn5 = assign51770_e85909_d_n5;
        locals.var_t2_dn6 = assign51770_e85909_d_n6;
        locals.var_t2_dn7 = assign51770_e85909_d_n7;
        locals.var_t2_dn8 = assign51770_e85909_d_n8;
        locals.var_t2_dn9 = assign51770_e85909_d_n9;
        locals.var_t2_dn10 = assign51770_e85909_d_n10;
        locals.var_t2_dn11 = assign51770_e85909_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign51780_e85922, assign51780_e85922_d_n3, assign51780_e85922_d_n4, assign51780_e85922_d_n5, assign51780_e85922_d_n6, assign51780_e85922_d_n7, assign51780_e85922_d_n8, assign51780_e85922_d_n9, assign51780_e85922_d_n10, assign51780_e85922_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51780_e85918: f64 = (locals.var_bechvbedge * locals.var_t1);
        let assign51780_e85920: f64 = (assign51780_e85918 * locals.var_t2);
        (assign51780_e85920, (((locals.var_bechvbedge * locals.var_t1_dn3) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn3)), (((locals.var_bechvbedge * locals.var_t1_dn4) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn4)), (((locals.var_bechvbedge * locals.var_t1_dn5) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn5)), (((locals.var_bechvbedge * locals.var_t1_dn6) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn6)), (((locals.var_bechvbedge * locals.var_t1_dn7) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn7)), (((locals.var_bechvbedge * locals.var_t1_dn8) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn8)), (((locals.var_bechvbedge * locals.var_t1_dn9) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn9)), (((locals.var_bechvbedge * locals.var_t1_dn10) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn10)), (((locals.var_bechvbedge * locals.var_t1_dn11) * locals.var_t2) + (assign51780_e85918 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign51780_e85922;
        locals.var_t3_dn3 = assign51780_e85922_d_n3;
        locals.var_t3_dn4 = assign51780_e85922_d_n4;
        locals.var_t3_dn5 = assign51780_e85922_d_n5;
        locals.var_t3_dn6 = assign51780_e85922_d_n6;
        locals.var_t3_dn7 = assign51780_e85922_d_n7;
        locals.var_t3_dn8 = assign51780_e85922_d_n8;
        locals.var_t3_dn9 = assign51780_e85922_d_n9;
        locals.var_t3_dn10 = assign51780_e85922_d_n10;
        locals.var_t3_dn11 = assign51780_e85922_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign51790_e85932, assign51790_e85932_d_n3, assign51790_e85932_d_n4, assign51790_e85932_d_n5, assign51790_e85932_d_n6, assign51790_e85932_d_n7, assign51790_e85932_d_n8, assign51790_e85932_d_n9, assign51790_e85932_d_n10, assign51790_e85932_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51790_e85930: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign51790_e85930, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign51790_e85932;
        locals.var_t4_dn3 = assign51790_e85932_d_n3;
        locals.var_t4_dn4 = assign51790_e85932_d_n4;
        locals.var_t4_dn5 = assign51790_e85932_d_n5;
        locals.var_t4_dn6 = assign51790_e85932_d_n6;
        locals.var_t4_dn7 = assign51790_e85932_d_n7;
        locals.var_t4_dn8 = assign51790_e85932_d_n8;
        locals.var_t4_dn9 = assign51790_e85932_d_n9;
        locals.var_t4_dn10 = assign51790_e85932_d_n10;
        locals.var_t4_dn11 = assign51790_e85932_d_n11;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_179(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign51820_e85973, assign51820_e85973_d_n3, assign51820_e85973_d_n4, assign51820_e85973_d_n5, assign51820_e85973_d_n6, assign51820_e85973_d_n7, assign51820_e85973_d_n8, assign51820_e85973_d_n9, assign51820_e85973_d_n10, assign51820_e85973_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51820_e85971: f64 = (locals.var_vgd_noswap - locals.var_vfbsdr);
        (assign51820_e85971, 0.0, (-locals.var_vfbsdr_dn4), (-locals.var_vfbsdr_dn5), locals.var_vgd_noswap_dn6, locals.var_vgd_noswap_dn7, locals.var_vgd_noswap_dn8, 0.0, locals.var_vgd_noswap_dn10, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51820_e85973;
        locals.var_t2_dn3 = assign51820_e85973_d_n3;
        locals.var_t2_dn4 = assign51820_e85973_d_n4;
        locals.var_t2_dn5 = assign51820_e85973_d_n5;
        locals.var_t2_dn6 = assign51820_e85973_d_n6;
        locals.var_t2_dn7 = assign51820_e85973_d_n7;
        locals.var_t2_dn8 = assign51820_e85973_d_n8;
        locals.var_t2_dn9 = assign51820_e85973_d_n9;
        locals.var_t2_dn10 = assign51820_e85973_d_n10;
        locals.var_t2_dn11 = assign51820_e85973_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign51830_e85987, assign51830_e85987_d_n3, assign51830_e85987_d_n4, assign51830_e85987_d_n5, assign51830_e85987_d_n6, assign51830_e85987_d_n7, assign51830_e85987_d_n8, assign51830_e85987_d_n9, assign51830_e85987_d_n10, assign51830_e85987_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51830_e85982: f64 = (locals.var_t2 * locals.var_t2);
        let assign51830_e85984: f64 = (assign51830_e85982 + 0.0001);
        let assign51830_e85985: f64 = (assign51830_e85984).sqrt();
        (assign51830_e85985, (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign51830_e85985)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign51830_e85985)),)
    } else {
        (locals.var_vgd_eff, locals.var_vgd_eff_dn3, locals.var_vgd_eff_dn4, locals.var_vgd_eff_dn5, locals.var_vgd_eff_dn6, locals.var_vgd_eff_dn7, locals.var_vgd_eff_dn8, locals.var_vgd_eff_dn9, locals.var_vgd_eff_dn10, locals.var_vgd_eff_dn11,)
    }
};
        locals.var_vgd_eff = assign51830_e85987;
        locals.var_vgd_eff_dn3 = assign51830_e85987_d_n3;
        locals.var_vgd_eff_dn4 = assign51830_e85987_d_n4;
        locals.var_vgd_eff_dn5 = assign51830_e85987_d_n5;
        locals.var_vgd_eff_dn6 = assign51830_e85987_d_n6;
        locals.var_vgd_eff_dn7 = assign51830_e85987_d_n7;
        locals.var_vgd_eff_dn8 = assign51830_e85987_d_n8;
        locals.var_vgd_eff_dn9 = assign51830_e85987_d_n9;
        locals.var_vgd_eff_dn10 = assign51830_e85987_d_n10;
        locals.var_vgd_eff_dn11 = assign51830_e85987_d_n11;
        locals.var_vgd_eff_rv = 0.0;

        let assign51840_e85990: f64 = if p.p1295 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard787 = assign51840_e85990;
        locals.var_guard787_rv = 0.0;

        let (assign51850_e86026, assign51850_e86026_d_n3, assign51850_e86026_d_n4, assign51850_e86026_d_n5, assign51850_e86026_d_n6, assign51850_e86026_d_n7, assign51850_e86026_d_n8, assign51850_e86026_d_n9, assign51850_e86026_d_n10, assign51850_e86026_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard787 != 0.0)) {
        let assign51850_e86003: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
        let assign51850_e86004: f64 = (locals.var_aigd_i - assign51850_e86003);
        let assign51850_e86008: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
        let assign51850_e86009: f64 = (locals.var_aigd_i - assign51850_e86008);
        let assign51850_e86013: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
        let assign51850_e86014: f64 = (locals.var_aigd_i - assign51850_e86013);
        let assign51850_e86015: f64 = (assign51850_e86009 * assign51850_e86014);
        let assign51850_e86018: f64 = (4.0 * 1e-6);
        let assign51850_e86020: f64 = (assign51850_e86018 * 1e-6);
        let assign51850_e86021: f64 = (assign51850_e86015 + assign51850_e86020);
        let assign51850_e86022: f64 = (assign51850_e86021).sqrt();
        let assign51850_e86023: f64 = (assign51850_e86004 + assign51850_e86022);
        let assign51850_e86024: f64 = (0.5 * assign51850_e86023);
        (assign51850_e86024, (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn3)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn3)) * assign51850_e86014) + (assign51850_e86009 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn3)))) / (2.0 * assign51850_e86022)))), (0.5 * ((locals.var_aigd_i_dn4 - (locals.var_bigd_i * locals.var_vgd_eff_dn4)) + ((((locals.var_aigd_i_dn4 - (locals.var_bigd_i * locals.var_vgd_eff_dn4)) * assign51850_e86014) + (assign51850_e86009 * (locals.var_aigd_i_dn4 - (locals.var_bigd_i * locals.var_vgd_eff_dn4)))) / (2.0 * assign51850_e86022)))), (0.5 * ((locals.var_aigd_i_dn5 - (locals.var_bigd_i * locals.var_vgd_eff_dn5)) + ((((locals.var_aigd_i_dn5 - (locals.var_bigd_i * locals.var_vgd_eff_dn5)) * assign51850_e86014) + (assign51850_e86009 * (locals.var_aigd_i_dn5 - (locals.var_bigd_i * locals.var_vgd_eff_dn5)))) / (2.0 * assign51850_e86022)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn6)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn6)) * assign51850_e86014) + (assign51850_e86009 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn6)))) / (2.0 * assign51850_e86022)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn7)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn7)) * assign51850_e86014) + (assign51850_e86009 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn7)))) / (2.0 * assign51850_e86022)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn8)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn8)) * assign51850_e86014) + (assign51850_e86009 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn8)))) / (2.0 * assign51850_e86022)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn9)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn9)) * assign51850_e86014) + (assign51850_e86009 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn9)))) / (2.0 * assign51850_e86022)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn10)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn10)) * assign51850_e86014) + (assign51850_e86009 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn10)))) / (2.0 * assign51850_e86022)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn11)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn11)) * assign51850_e86014) + (assign51850_e86009 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn11)))) / (2.0 * assign51850_e86022)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51850_e86026;
        locals.var_t1_dn3 = assign51850_e86026_d_n3;
        locals.var_t1_dn4 = assign51850_e86026_d_n4;
        locals.var_t1_dn5 = assign51850_e86026_d_n5;
        locals.var_t1_dn6 = assign51850_e86026_d_n6;
        locals.var_t1_dn7 = assign51850_e86026_d_n7;
        locals.var_t1_dn8 = assign51850_e86026_d_n8;
        locals.var_t1_dn9 = assign51850_e86026_d_n9;
        locals.var_t1_dn10 = assign51850_e86026_d_n10;
        locals.var_t1_dn11 = assign51850_e86026_d_n11;
        locals.var_t1_rv = 0.0;

        let assign51860_e86029: f64 = if locals.var_cigd_i < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard788 = assign51860_e86029;
        locals.var_guard788_rv = 0.0;

        let (assign51870_e86042,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard787 != 0.0)) && (locals.var_guard788 != 0.0)) {
        (0.01,)
    } else {
        (locals.var_cigd_i,)
    }
};
        locals.var_cigd_i = assign51870_e86042;
        locals.var_cigd_i_rv = 0.0;

        let (assign51880_e86058, assign51880_e86058_d_n3, assign51880_e86058_d_n4, assign51880_e86058_d_n5, assign51880_e86058_d_n6, assign51880_e86058_d_n7, assign51880_e86058_d_n8, assign51880_e86058_d_n9, assign51880_e86058_d_n10, assign51880_e86058_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) && (locals.var_guard787 == 0.0)) {
        let assign51880_e86055: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
        let assign51880_e86056: f64 = (locals.var_aigd_i - assign51880_e86055);
        (assign51880_e86056, (-(locals.var_bigd_i * locals.var_vgd_eff_dn3)), (locals.var_aigd_i_dn4 - (locals.var_bigd_i * locals.var_vgd_eff_dn4)), (locals.var_aigd_i_dn5 - (locals.var_bigd_i * locals.var_vgd_eff_dn5)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn6)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn7)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn8)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn9)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn10)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn11)),)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign51880_e86058;
        locals.var_t1_dn3 = assign51880_e86058_d_n3;
        locals.var_t1_dn4 = assign51880_e86058_d_n4;
        locals.var_t1_dn5 = assign51880_e86058_d_n5;
        locals.var_t1_dn6 = assign51880_e86058_d_n6;
        locals.var_t1_dn7 = assign51880_e86058_d_n7;
        locals.var_t1_dn8 = assign51880_e86058_d_n8;
        locals.var_t1_dn9 = assign51880_e86058_d_n9;
        locals.var_t1_dn10 = assign51880_e86058_d_n10;
        locals.var_t1_dn11 = assign51880_e86058_d_n11;
        locals.var_t1_rv = 0.0;

        let (assign51890_e86071, assign51890_e86071_d_n3, assign51890_e86071_d_n4, assign51890_e86071_d_n5, assign51890_e86071_d_n6, assign51890_e86071_d_n7, assign51890_e86071_d_n8, assign51890_e86071_d_n9, assign51890_e86071_d_n10, assign51890_e86071_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51890_e86068: f64 = (locals.var_cigd_i * locals.var_vgd_eff);
        let assign51890_e86069: f64 = (1.0 + assign51890_e86068);
        (assign51890_e86069, (locals.var_cigd_i * locals.var_vgd_eff_dn3), (locals.var_cigd_i * locals.var_vgd_eff_dn4), (locals.var_cigd_i * locals.var_vgd_eff_dn5), (locals.var_cigd_i * locals.var_vgd_eff_dn6), (locals.var_cigd_i * locals.var_vgd_eff_dn7), (locals.var_cigd_i * locals.var_vgd_eff_dn8), (locals.var_cigd_i * locals.var_vgd_eff_dn9), (locals.var_cigd_i * locals.var_vgd_eff_dn10), (locals.var_cigd_i * locals.var_vgd_eff_dn11),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign51890_e86071;
        locals.var_t2_dn3 = assign51890_e86071_d_n3;
        locals.var_t2_dn4 = assign51890_e86071_d_n4;
        locals.var_t2_dn5 = assign51890_e86071_d_n5;
        locals.var_t2_dn6 = assign51890_e86071_d_n6;
        locals.var_t2_dn7 = assign51890_e86071_d_n7;
        locals.var_t2_dn8 = assign51890_e86071_d_n8;
        locals.var_t2_dn9 = assign51890_e86071_d_n9;
        locals.var_t2_dn10 = assign51890_e86071_d_n10;
        locals.var_t2_dn11 = assign51890_e86071_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign51900_e86084, assign51900_e86084_d_n3, assign51900_e86084_d_n4, assign51900_e86084_d_n5, assign51900_e86084_d_n6, assign51900_e86084_d_n7, assign51900_e86084_d_n8, assign51900_e86084_d_n9, assign51900_e86084_d_n10, assign51900_e86084_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51900_e86080: f64 = (locals.var_bechvbedge * locals.var_t1);
        let assign51900_e86082: f64 = (assign51900_e86080 * locals.var_t2);
        (assign51900_e86082, (((locals.var_bechvbedge * locals.var_t1_dn3) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn3)), (((locals.var_bechvbedge * locals.var_t1_dn4) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn4)), (((locals.var_bechvbedge * locals.var_t1_dn5) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn5)), (((locals.var_bechvbedge * locals.var_t1_dn6) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn6)), (((locals.var_bechvbedge * locals.var_t1_dn7) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn7)), (((locals.var_bechvbedge * locals.var_t1_dn8) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn8)), (((locals.var_bechvbedge * locals.var_t1_dn9) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn9)), (((locals.var_bechvbedge * locals.var_t1_dn10) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn10)), (((locals.var_bechvbedge * locals.var_t1_dn11) * locals.var_t2) + (assign51900_e86080 * locals.var_t2_dn11)),)
    } else {
        (locals.var_t3, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11,)
    }
};
        locals.var_t3 = assign51900_e86084;
        locals.var_t3_dn3 = assign51900_e86084_d_n3;
        locals.var_t3_dn4 = assign51900_e86084_d_n4;
        locals.var_t3_dn5 = assign51900_e86084_d_n5;
        locals.var_t3_dn6 = assign51900_e86084_d_n6;
        locals.var_t3_dn7 = assign51900_e86084_d_n7;
        locals.var_t3_dn8 = assign51900_e86084_d_n8;
        locals.var_t3_dn9 = assign51900_e86084_d_n9;
        locals.var_t3_dn10 = assign51900_e86084_d_n10;
        locals.var_t3_dn11 = assign51900_e86084_d_n11;
        locals.var_t3_rv = 0.0;

        let (assign51910_e86094, assign51910_e86094_d_n3, assign51910_e86094_d_n4, assign51910_e86094_d_n5, assign51910_e86094_d_n6, assign51910_e86094_d_n7, assign51910_e86094_d_n8, assign51910_e86094_d_n9, assign51910_e86094_d_n10, assign51910_e86094_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard776 != 0.0)) && (locals.var_guard783 != 0.0)) {
        let assign51910_e86092: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign51910_e86092, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11),)
    } else {
        (locals.var_t4, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11,)
    }
};
        locals.var_t4 = assign51910_e86094;
        locals.var_t4_dn3 = assign51910_e86094_d_n3;
        locals.var_t4_dn4 = assign51910_e86094_d_n4;
        locals.var_t4_dn5 = assign51910_e86094_d_n5;
        locals.var_t4_dn6 = assign51910_e86094_d_n6;
        locals.var_t4_dn7 = assign51910_e86094_d_n7;
        locals.var_t4_dn8 = assign51910_e86094_d_n8;
        locals.var_t4_dn9 = assign51910_e86094_d_n9;
        locals.var_t4_dn10 = assign51910_e86094_d_n10;
        locals.var_t4_dn11 = assign51910_e86094_d_n11;
        locals.var_t4_rv = 0.0;

        let (assign51990_e86166, assign51990_e86166_d_n3, assign51990_e86166_d_n4, assign51990_e86166_d_n5, assign51990_e86166_d_n6, assign51990_e86166_d_n7, assign51990_e86166_d_n8, assign51990_e86166_d_n9, assign51990_e86166_d_n10, assign51990_e86166_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign51990_e86164: f64 = (locals.var_cjs_t * locals.var_aseff);
        (assign51990_e86164, (locals.var_cjs_t * locals.var_aseff_dn3), ((locals.var_cjs_t_dn4 * locals.var_aseff) + (locals.var_cjs_t * locals.var_aseff_dn4)), ((locals.var_cjs_t_dn5 * locals.var_aseff) + (locals.var_cjs_t * locals.var_aseff_dn5)), (locals.var_cjs_t * locals.var_aseff_dn6), (locals.var_cjs_t * locals.var_aseff_dn7), (locals.var_cjs_t * locals.var_aseff_dn8), (locals.var_cjs_t * locals.var_aseff_dn9), (locals.var_cjs_t * locals.var_aseff_dn10), (locals.var_cjs_t * locals.var_aseff_dn11),)
    } else {
        (locals.var_czbs, locals.var_czbs_dn3, locals.var_czbs_dn4, locals.var_czbs_dn5, locals.var_czbs_dn6, locals.var_czbs_dn7, locals.var_czbs_dn8, locals.var_czbs_dn9, locals.var_czbs_dn10, locals.var_czbs_dn11,)
    }
};
        locals.var_czbs = assign51990_e86166;
        locals.var_czbs_dn3 = assign51990_e86166_d_n3;
        locals.var_czbs_dn4 = assign51990_e86166_d_n4;
        locals.var_czbs_dn5 = assign51990_e86166_d_n5;
        locals.var_czbs_dn6 = assign51990_e86166_d_n6;
        locals.var_czbs_dn7 = assign51990_e86166_d_n7;
        locals.var_czbs_dn8 = assign51990_e86166_d_n8;
        locals.var_czbs_dn9 = assign51990_e86166_d_n9;
        locals.var_czbs_dn10 = assign51990_e86166_d_n10;
        locals.var_czbs_dn11 = assign51990_e86166_d_n11;
        locals.var_czbs_rv = 0.0;

        let (assign52000_e86173, assign52000_e86173_d_n3, assign52000_e86173_d_n4, assign52000_e86173_d_n5, assign52000_e86173_d_n6, assign52000_e86173_d_n7, assign52000_e86173_d_n8, assign52000_e86173_d_n9, assign52000_e86173_d_n10, assign52000_e86173_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52000_e86171: f64 = (locals.var_cjsws_t * locals.var_pseff);
        (assign52000_e86171, (locals.var_cjsws_t * locals.var_pseff_dn3), ((locals.var_cjsws_t_dn4 * locals.var_pseff) + (locals.var_cjsws_t * locals.var_pseff_dn4)), ((locals.var_cjsws_t_dn5 * locals.var_pseff) + (locals.var_cjsws_t * locals.var_pseff_dn5)), (locals.var_cjsws_t * locals.var_pseff_dn6), (locals.var_cjsws_t * locals.var_pseff_dn7), (locals.var_cjsws_t * locals.var_pseff_dn8), (locals.var_cjsws_t * locals.var_pseff_dn9), (locals.var_cjsws_t * locals.var_pseff_dn10), (locals.var_cjsws_t * locals.var_pseff_dn11),)
    } else {
        (locals.var_czbssw, locals.var_czbssw_dn3, locals.var_czbssw_dn4, locals.var_czbssw_dn5, locals.var_czbssw_dn6, locals.var_czbssw_dn7, locals.var_czbssw_dn8, locals.var_czbssw_dn9, locals.var_czbssw_dn10, locals.var_czbssw_dn11,)
    }
};
        locals.var_czbssw = assign52000_e86173;
        locals.var_czbssw_dn3 = assign52000_e86173_d_n3;
        locals.var_czbssw_dn4 = assign52000_e86173_d_n4;
        locals.var_czbssw_dn5 = assign52000_e86173_d_n5;
        locals.var_czbssw_dn6 = assign52000_e86173_d_n6;
        locals.var_czbssw_dn7 = assign52000_e86173_d_n7;
        locals.var_czbssw_dn8 = assign52000_e86173_d_n8;
        locals.var_czbssw_dn9 = assign52000_e86173_d_n9;
        locals.var_czbssw_dn10 = assign52000_e86173_d_n10;
        locals.var_czbssw_dn11 = assign52000_e86173_d_n11;
        locals.var_czbssw_rv = 0.0;

        let (assign52010_e86182, assign52010_e86182_d_n4, assign52010_e86182_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52010_e86178: f64 = (locals.var_cjswgs_t * locals.var_weffcj);
        let assign52010_e86180: f64 = (assign52010_e86178 * p.p2);
        (assign52010_e86180, ((locals.var_cjswgs_t_dn4 * locals.var_weffcj) * p.p2), ((locals.var_cjswgs_t_dn5 * locals.var_weffcj) * p.p2),)
    } else {
        (locals.var_czbsswg, locals.var_czbsswg_dn4, locals.var_czbsswg_dn5,)
    }
};
        locals.var_czbsswg = assign52010_e86182;
        locals.var_czbsswg_dn4 = assign52010_e86182_d_n4;
        locals.var_czbsswg_dn5 = assign52010_e86182_d_n5;
        locals.var_czbsswg_rv = 0.0;

        let (assign52020_e86190,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52020_e86187: f64 = (-p.p913);
        let assign52020_e86188: f64 = (0.1_f64).powf(assign52020_e86187);
        (assign52020_e86188,)
    } else {
        (locals.var_czbs_p1,)
    }
};
        locals.var_czbs_p1 = assign52020_e86190;
        locals.var_czbs_p1_rv = 0.0;

        let assign52030_e86193: f64 = if p.p913 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard789 = assign52030_e86193;
        locals.var_guard789_rv = 0.0;

        let (assign52040_e86203,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard789 != 0.0)) {
        let assign52040_e86200: f64 = (0.1_f64).ln();
        let assign52040_e86201: f64 = (1.5 - assign52040_e86200);
        (assign52040_e86201,)
    } else {
        (locals.var_czbs_p2,)
    }
};
        locals.var_czbs_p2 = assign52040_e86203;
        locals.var_czbs_p2_rv = 0.0;

        let (assign52050_e86227,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard789 == 0.0)) {
        let assign52050_e86212: f64 = (1.0 - p.p913);
        let assign52050_e86213: f64 = (1.0 / assign52050_e86212);
        let assign52050_e86217: f64 = (0.05 * p.p913);
        let assign52050_e86220: f64 = (1.0 + p.p913);
        let assign52050_e86221: f64 = (assign52050_e86217 * assign52050_e86220);
        let assign52050_e86223: f64 = (assign52050_e86221 * locals.var_czbs_p1);
        let assign52050_e86224: f64 = (1.0 - assign52050_e86223);
        let assign52050_e86225: f64 = (assign52050_e86213 * assign52050_e86224);
        (assign52050_e86225,)
    } else {
        (locals.var_czbs_p2,)
    }
};
        locals.var_czbs_p2 = assign52050_e86227;
        locals.var_czbs_p2_rv = 0.0;

        let (assign52060_e86235,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52060_e86232: f64 = (-p.p915);
        let assign52060_e86233: f64 = (0.1_f64).powf(assign52060_e86232);
        (assign52060_e86233,)
    } else {
        (locals.var_czbssw_p1,)
    }
};
        locals.var_czbssw_p1 = assign52060_e86235;
        locals.var_czbssw_p1_rv = 0.0;

        let assign52070_e86238: f64 = if p.p915 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard790 = assign52070_e86238;
        locals.var_guard790_rv = 0.0;

        let (assign52080_e86248,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard790 != 0.0)) {
        let assign52080_e86245: f64 = (0.1_f64).ln();
        let assign52080_e86246: f64 = (1.5 - assign52080_e86245);
        (assign52080_e86246,)
    } else {
        (locals.var_czbssw_p2,)
    }
};
        locals.var_czbssw_p2 = assign52080_e86248;
        locals.var_czbssw_p2_rv = 0.0;

        let (assign52090_e86272,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard790 == 0.0)) {
        let assign52090_e86257: f64 = (1.0 - p.p915);
        let assign52090_e86258: f64 = (1.0 / assign52090_e86257);
        let assign52090_e86262: f64 = (0.05 * p.p915);
        let assign52090_e86265: f64 = (1.0 + p.p915);
        let assign52090_e86266: f64 = (assign52090_e86262 * assign52090_e86265);
        let assign52090_e86268: f64 = (assign52090_e86266 * locals.var_czbssw_p1);
        let assign52090_e86269: f64 = (1.0 - assign52090_e86268);
        let assign52090_e86270: f64 = (assign52090_e86258 * assign52090_e86269);
        (assign52090_e86270,)
    } else {
        (locals.var_czbssw_p2,)
    }
};
        locals.var_czbssw_p2 = assign52090_e86272;
        locals.var_czbssw_p2_rv = 0.0;

        let (assign52100_e86280,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52100_e86277: f64 = (-p.p917);
        let assign52100_e86278: f64 = (0.1_f64).powf(assign52100_e86277);
        (assign52100_e86278,)
    } else {
        (locals.var_czbsswg_p1,)
    }
};
        locals.var_czbsswg_p1 = assign52100_e86280;
        locals.var_czbsswg_p1_rv = 0.0;

        let assign52110_e86283: f64 = if p.p917 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard791 = assign52110_e86283;
        locals.var_guard791_rv = 0.0;

        let (assign52120_e86293,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard791 != 0.0)) {
        let assign52120_e86290: f64 = (0.1_f64).ln();
        let assign52120_e86291: f64 = (1.5 - assign52120_e86290);
        (assign52120_e86291,)
    } else {
        (locals.var_czbsswg_p2,)
    }
};
        locals.var_czbsswg_p2 = assign52120_e86293;
        locals.var_czbsswg_p2_rv = 0.0;

        let (assign52130_e86317,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard791 == 0.0)) {
        let assign52130_e86302: f64 = (1.0 - p.p917);
        let assign52130_e86303: f64 = (1.0 / assign52130_e86302);
        let assign52130_e86307: f64 = (0.05 * p.p917);
        let assign52130_e86310: f64 = (1.0 + p.p917);
        let assign52130_e86311: f64 = (assign52130_e86307 * assign52130_e86310);
        let assign52130_e86313: f64 = (assign52130_e86311 * locals.var_czbsswg_p1);
        let assign52130_e86314: f64 = (1.0 - assign52130_e86313);
        let assign52130_e86315: f64 = (assign52130_e86303 * assign52130_e86314);
        (assign52130_e86315,)
    } else {
        (locals.var_czbsswg_p2,)
    }
};
        locals.var_czbsswg_p2 = assign52130_e86317;
        locals.var_czbsswg_p2_rv = 0.0;

        let assign52140_e86320: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard792 = assign52140_e86320;
        locals.var_guard792_rv = 0.0;

        let (assign52150_e86329, assign52150_e86329_d_n3, assign52150_e86329_d_n4, assign52150_e86329_d_n5, assign52150_e86329_d_n6, assign52150_e86329_d_n7, assign52150_e86329_d_n8, assign52150_e86329_d_n9, assign52150_e86329_d_n10, assign52150_e86329_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) {
        let assign52150_e86327: f64 = (locals.var_vbs_jct / locals.var_pbs_t);
        (assign52150_e86327, 0.0, (-((locals.var_vbs_jct * locals.var_pbs_t_dn4) / (locals.var_pbs_t * locals.var_pbs_t))), (-((locals.var_vbs_jct * locals.var_pbs_t_dn5) / (locals.var_pbs_t * locals.var_pbs_t))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_pbs_t), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_pbs_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign52150_e86329;
        locals.var_t1_dn3 = assign52150_e86329_d_n3;
        locals.var_t1_dn4 = assign52150_e86329_d_n4;
        locals.var_t1_dn5 = assign52150_e86329_d_n5;
        locals.var_t1_dn6 = assign52150_e86329_d_n6;
        locals.var_t1_dn7 = assign52150_e86329_d_n7;
        locals.var_t1_dn8 = assign52150_e86329_d_n8;
        locals.var_t1_dn9 = assign52150_e86329_d_n9;
        locals.var_t1_dn10 = assign52150_e86329_d_n10;
        locals.var_t1_dn11 = assign52150_e86329_d_n11;
        locals.var_t1_rv = 0.0;

        let assign52160_e86332: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard793 = assign52160_e86332;
        locals.var_guard793_rv = 0.0;

        let (assign52170_e86343, assign52170_e86343_d_n3, assign52170_e86343_d_n4, assign52170_e86343_d_n5, assign52170_e86343_d_n6, assign52170_e86343_d_n7, assign52170_e86343_d_n8, assign52170_e86343_d_n9, assign52170_e86343_d_n10, assign52170_e86343_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) {
        let assign52170_e86341: f64 = (1.0 - locals.var_t1);
        (assign52170_e86341, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign52170_e86343;
        locals.var_arg_dn3 = assign52170_e86343_d_n3;
        locals.var_arg_dn4 = assign52170_e86343_d_n4;
        locals.var_arg_dn5 = assign52170_e86343_d_n5;
        locals.var_arg_dn6 = assign52170_e86343_d_n6;
        locals.var_arg_dn7 = assign52170_e86343_d_n7;
        locals.var_arg_dn8 = assign52170_e86343_d_n8;
        locals.var_arg_dn9 = assign52170_e86343_d_n9;
        locals.var_arg_dn10 = assign52170_e86343_d_n10;
        locals.var_arg_dn11 = assign52170_e86343_d_n11;
        locals.var_arg_rv = 0.0;

        let assign52180_e86346: f64 = if p.p913 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard794 = assign52180_e86346;
        locals.var_guard794_rv = 0.0;

        let assign52190_e86349: f64 = if p.p913 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard795 = assign52190_e86349;
        locals.var_guard795_rv = 0.0;

        let (assign52200_e86365, assign52200_e86365_d_n3, assign52200_e86365_d_n4, assign52200_e86365_d_n5, assign52200_e86365_d_n6, assign52200_e86365_d_n7, assign52200_e86365_d_n8, assign52200_e86365_d_n9, assign52200_e86365_d_n10, assign52200_e86365_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) && (locals.var_guard794 != 0.0)) && (locals.var_guard795 != 0.0)) {
        let assign52200_e86362: f64 = (locals.var_arg).sqrt();
        let assign52200_e86363: f64 = (1.0 / assign52200_e86362);
        (assign52200_e86363, (-((locals.var_arg_dn3 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn4 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn5 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn6 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn7 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn8 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn9 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn10 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))), (-((locals.var_arg_dn11 / (2.0 * assign52200_e86362)) / (assign52200_e86362 * assign52200_e86362))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52200_e86365;
        locals.var_sarg_dn3 = assign52200_e86365_d_n3;
        locals.var_sarg_dn4 = assign52200_e86365_d_n4;
        locals.var_sarg_dn5 = assign52200_e86365_d_n5;
        locals.var_sarg_dn6 = assign52200_e86365_d_n6;
        locals.var_sarg_dn7 = assign52200_e86365_d_n7;
        locals.var_sarg_dn8 = assign52200_e86365_d_n8;
        locals.var_sarg_dn9 = assign52200_e86365_d_n9;
        locals.var_sarg_dn10 = assign52200_e86365_d_n10;
        locals.var_sarg_dn11 = assign52200_e86365_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign52210_e86384, assign52210_e86384_d_n3, assign52210_e86384_d_n4, assign52210_e86384_d_n5, assign52210_e86384_d_n6, assign52210_e86384_d_n7, assign52210_e86384_d_n8, assign52210_e86384_d_n9, assign52210_e86384_d_n10, assign52210_e86384_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) && (locals.var_guard794 != 0.0)) && (locals.var_guard795 == 0.0)) {
        let assign52210_e86378: f64 = (-p.p913);
        let assign52210_e86380: f64 = (locals.var_arg).ln();
        let assign52210_e86381: f64 = (assign52210_e86378 * assign52210_e86380);
        let assign52210_e86382: f64 = { let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign52210_e86382, ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign52210_e86381; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52210_e86378 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52210_e86384;
        locals.var_sarg_dn3 = assign52210_e86384_d_n3;
        locals.var_sarg_dn4 = assign52210_e86384_d_n4;
        locals.var_sarg_dn5 = assign52210_e86384_d_n5;
        locals.var_sarg_dn6 = assign52210_e86384_d_n6;
        locals.var_sarg_dn7 = assign52210_e86384_d_n7;
        locals.var_sarg_dn8 = assign52210_e86384_d_n8;
        locals.var_sarg_dn9 = assign52210_e86384_d_n9;
        locals.var_sarg_dn10 = assign52210_e86384_d_n10;
        locals.var_sarg_dn11 = assign52210_e86384_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign52220_e86407, assign52220_e86407_d_n3, assign52220_e86407_d_n4, assign52220_e86407_d_n5, assign52220_e86407_d_n6, assign52220_e86407_d_n7, assign52220_e86407_d_n8, assign52220_e86407_d_n9, assign52220_e86407_d_n10, assign52220_e86407_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) && (locals.var_guard794 != 0.0)) {
        let assign52220_e86395: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign52220_e86399: f64 = (locals.var_arg * locals.var_sarg);
        let assign52220_e86400: f64 = (1.0 - assign52220_e86399);
        let assign52220_e86401: f64 = (assign52220_e86395 * assign52220_e86400);
        let assign52220_e86404: f64 = (1.0 - p.p913);
        let assign52220_e86405: f64 = (assign52220_e86401 / assign52220_e86404);
        (assign52220_e86405, ((((locals.var_pbs_t * locals.var_czbs_dn3) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3))))) / assign52220_e86404), (((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign52220_e86404), (((((locals.var_pbs_t_dn5 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn5)) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign52220_e86404), ((((locals.var_pbs_t * locals.var_czbs_dn6) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign52220_e86404), ((((locals.var_pbs_t * locals.var_czbs_dn7) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign52220_e86404), ((((locals.var_pbs_t * locals.var_czbs_dn8) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign52220_e86404), ((((locals.var_pbs_t * locals.var_czbs_dn9) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign52220_e86404), ((((locals.var_pbs_t * locals.var_czbs_dn10) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign52220_e86404), ((((locals.var_pbs_t * locals.var_czbs_dn11) * assign52220_e86400) + (assign52220_e86395 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign52220_e86404),)
    } else {
        (locals.var_qbsj1, locals.var_qbsj1_dn3, locals.var_qbsj1_dn4, locals.var_qbsj1_dn5, locals.var_qbsj1_dn6, locals.var_qbsj1_dn7, locals.var_qbsj1_dn8, locals.var_qbsj1_dn9, locals.var_qbsj1_dn10, locals.var_qbsj1_dn11,)
    }
};
        locals.var_qbsj1 = assign52220_e86407;
        locals.var_qbsj1_dn3 = assign52220_e86407_d_n3;
        locals.var_qbsj1_dn4 = assign52220_e86407_d_n4;
        locals.var_qbsj1_dn5 = assign52220_e86407_d_n5;
        locals.var_qbsj1_dn6 = assign52220_e86407_d_n6;
        locals.var_qbsj1_dn7 = assign52220_e86407_d_n7;
        locals.var_qbsj1_dn8 = assign52220_e86407_d_n8;
        locals.var_qbsj1_dn9 = assign52220_e86407_d_n9;
        locals.var_qbsj1_dn10 = assign52220_e86407_d_n10;
        locals.var_qbsj1_dn11 = assign52220_e86407_d_n11;
        locals.var_qbsj1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_180(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign52230_e86425, assign52230_e86425_d_n3, assign52230_e86425_d_n4, assign52230_e86425_d_n5, assign52230_e86425_d_n6, assign52230_e86425_d_n7, assign52230_e86425_d_n8, assign52230_e86425_d_n9, assign52230_e86425_d_n10, assign52230_e86425_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 != 0.0)) && (locals.var_guard794 == 0.0)) {
        let assign52230_e86419: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign52230_e86421: f64 = (locals.var_arg).ln();
        let assign52230_e86422: f64 = (-assign52230_e86421);
        let assign52230_e86423: f64 = (assign52230_e86419 * assign52230_e86422);
        (assign52230_e86423, (((locals.var_pbs_t * locals.var_czbs_dn3) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn3 / locals.var_arg)))), ((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbs_t_dn5 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn5)) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn5 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn6) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn6 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn7) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn7 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn8) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn8 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn9) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn9 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn10) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn10 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn11) * assign52230_e86422) + (assign52230_e86419 * (-(locals.var_arg_dn11 / locals.var_arg)))),)
    } else {
        (locals.var_qbsj1, locals.var_qbsj1_dn3, locals.var_qbsj1_dn4, locals.var_qbsj1_dn5, locals.var_qbsj1_dn6, locals.var_qbsj1_dn7, locals.var_qbsj1_dn8, locals.var_qbsj1_dn9, locals.var_qbsj1_dn10, locals.var_qbsj1_dn11,)
    }
};
        locals.var_qbsj1 = assign52230_e86425;
        locals.var_qbsj1_dn3 = assign52230_e86425_d_n3;
        locals.var_qbsj1_dn4 = assign52230_e86425_d_n4;
        locals.var_qbsj1_dn5 = assign52230_e86425_d_n5;
        locals.var_qbsj1_dn6 = assign52230_e86425_d_n6;
        locals.var_qbsj1_dn7 = assign52230_e86425_d_n7;
        locals.var_qbsj1_dn8 = assign52230_e86425_d_n8;
        locals.var_qbsj1_dn9 = assign52230_e86425_d_n9;
        locals.var_qbsj1_dn10 = assign52230_e86425_d_n10;
        locals.var_qbsj1_dn11 = assign52230_e86425_d_n11;
        locals.var_qbsj1_rv = 0.0;

        let (assign52240_e86451, assign52240_e86451_d_n3, assign52240_e86451_d_n4, assign52240_e86451_d_n5, assign52240_e86451_d_n6, assign52240_e86451_d_n7, assign52240_e86451_d_n8, assign52240_e86451_d_n9, assign52240_e86451_d_n10, assign52240_e86451_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 == 0.0)) {
        let assign52240_e86436: f64 = (locals.var_t1 - 1.0);
        let assign52240_e86437: f64 = (locals.var_czbs_p1 * assign52240_e86436);
        let assign52240_e86440: f64 = (5.0 * p.p913);
        let assign52240_e86443: f64 = (locals.var_t1 - 1.0);
        let assign52240_e86444: f64 = (assign52240_e86440 * assign52240_e86443);
        let assign52240_e86447: f64 = (1.0 + p.p913);
        let assign52240_e86448: f64 = (assign52240_e86444 + assign52240_e86447);
        let assign52240_e86449: f64 = (assign52240_e86437 * assign52240_e86448);
        (assign52240_e86449, (((locals.var_czbs_p1 * locals.var_t1_dn3) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn3))), (((locals.var_czbs_p1 * locals.var_t1_dn4) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn4))), (((locals.var_czbs_p1 * locals.var_t1_dn5) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn5))), (((locals.var_czbs_p1 * locals.var_t1_dn6) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn6))), (((locals.var_czbs_p1 * locals.var_t1_dn7) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn7))), (((locals.var_czbs_p1 * locals.var_t1_dn8) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn8))), (((locals.var_czbs_p1 * locals.var_t1_dn9) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn9))), (((locals.var_czbs_p1 * locals.var_t1_dn10) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn10))), (((locals.var_czbs_p1 * locals.var_t1_dn11) * assign52240_e86448) + (assign52240_e86437 * (assign52240_e86440 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign52240_e86451;
        locals.var_t2_dn3 = assign52240_e86451_d_n3;
        locals.var_t2_dn4 = assign52240_e86451_d_n4;
        locals.var_t2_dn5 = assign52240_e86451_d_n5;
        locals.var_t2_dn6 = assign52240_e86451_d_n6;
        locals.var_t2_dn7 = assign52240_e86451_d_n7;
        locals.var_t2_dn8 = assign52240_e86451_d_n8;
        locals.var_t2_dn9 = assign52240_e86451_d_n9;
        locals.var_t2_dn10 = assign52240_e86451_d_n10;
        locals.var_t2_dn11 = assign52240_e86451_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign52250_e86467, assign52250_e86467_d_n3, assign52250_e86467_d_n4, assign52250_e86467_d_n5, assign52250_e86467_d_n6, assign52250_e86467_d_n7, assign52250_e86467_d_n8, assign52250_e86467_d_n9, assign52250_e86467_d_n10, assign52250_e86467_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard792 != 0.0)) && (locals.var_guard793 == 0.0)) {
        let assign52250_e86461: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign52250_e86464: f64 = (locals.var_t2 + locals.var_czbs_p2);
        let assign52250_e86465: f64 = (assign52250_e86461 * assign52250_e86464);
        (assign52250_e86465, (((locals.var_pbs_t * locals.var_czbs_dn3) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn3)), ((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn4)), ((((locals.var_pbs_t_dn5 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn5)) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn5)), (((locals.var_pbs_t * locals.var_czbs_dn6) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn6)), (((locals.var_pbs_t * locals.var_czbs_dn7) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn7)), (((locals.var_pbs_t * locals.var_czbs_dn8) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn8)), (((locals.var_pbs_t * locals.var_czbs_dn9) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn9)), (((locals.var_pbs_t * locals.var_czbs_dn10) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn10)), (((locals.var_pbs_t * locals.var_czbs_dn11) * assign52250_e86464) + (assign52250_e86461 * locals.var_t2_dn11)),)
    } else {
        (locals.var_qbsj1, locals.var_qbsj1_dn3, locals.var_qbsj1_dn4, locals.var_qbsj1_dn5, locals.var_qbsj1_dn6, locals.var_qbsj1_dn7, locals.var_qbsj1_dn8, locals.var_qbsj1_dn9, locals.var_qbsj1_dn10, locals.var_qbsj1_dn11,)
    }
};
        locals.var_qbsj1 = assign52250_e86467;
        locals.var_qbsj1_dn3 = assign52250_e86467_d_n3;
        locals.var_qbsj1_dn4 = assign52250_e86467_d_n4;
        locals.var_qbsj1_dn5 = assign52250_e86467_d_n5;
        locals.var_qbsj1_dn6 = assign52250_e86467_d_n6;
        locals.var_qbsj1_dn7 = assign52250_e86467_d_n7;
        locals.var_qbsj1_dn8 = assign52250_e86467_d_n8;
        locals.var_qbsj1_dn9 = assign52250_e86467_d_n9;
        locals.var_qbsj1_dn10 = assign52250_e86467_d_n10;
        locals.var_qbsj1_dn11 = assign52250_e86467_d_n11;
        locals.var_qbsj1_rv = 0.0;

        let (assign52260_e86475, assign52260_e86475_d_n3, assign52260_e86475_d_n4, assign52260_e86475_d_n5, assign52260_e86475_d_n6, assign52260_e86475_d_n7, assign52260_e86475_d_n8, assign52260_e86475_d_n9, assign52260_e86475_d_n10, assign52260_e86475_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard792 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbsj1, locals.var_qbsj1_dn3, locals.var_qbsj1_dn4, locals.var_qbsj1_dn5, locals.var_qbsj1_dn6, locals.var_qbsj1_dn7, locals.var_qbsj1_dn8, locals.var_qbsj1_dn9, locals.var_qbsj1_dn10, locals.var_qbsj1_dn11,)
    }
};
        locals.var_qbsj1 = assign52260_e86475;
        locals.var_qbsj1_dn3 = assign52260_e86475_d_n3;
        locals.var_qbsj1_dn4 = assign52260_e86475_d_n4;
        locals.var_qbsj1_dn5 = assign52260_e86475_d_n5;
        locals.var_qbsj1_dn6 = assign52260_e86475_d_n6;
        locals.var_qbsj1_dn7 = assign52260_e86475_d_n7;
        locals.var_qbsj1_dn8 = assign52260_e86475_d_n8;
        locals.var_qbsj1_dn9 = assign52260_e86475_d_n9;
        locals.var_qbsj1_dn10 = assign52260_e86475_d_n10;
        locals.var_qbsj1_dn11 = assign52260_e86475_d_n11;
        locals.var_qbsj1_rv = 0.0;

        let assign52270_e86478: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard796 = assign52270_e86478;
        locals.var_guard796_rv = 0.0;

        let (assign52280_e86487, assign52280_e86487_d_n3, assign52280_e86487_d_n4, assign52280_e86487_d_n5, assign52280_e86487_d_n6, assign52280_e86487_d_n7, assign52280_e86487_d_n8, assign52280_e86487_d_n9, assign52280_e86487_d_n10, assign52280_e86487_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) {
        let assign52280_e86485: f64 = (locals.var_vbs_jct / locals.var_pbsws_t);
        (assign52280_e86485, 0.0, (-((locals.var_vbs_jct * locals.var_pbsws_t_dn4) / (locals.var_pbsws_t * locals.var_pbsws_t))), (-((locals.var_vbs_jct * locals.var_pbsws_t_dn5) / (locals.var_pbsws_t * locals.var_pbsws_t))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_pbsws_t), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_pbsws_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign52280_e86487;
        locals.var_t1_dn3 = assign52280_e86487_d_n3;
        locals.var_t1_dn4 = assign52280_e86487_d_n4;
        locals.var_t1_dn5 = assign52280_e86487_d_n5;
        locals.var_t1_dn6 = assign52280_e86487_d_n6;
        locals.var_t1_dn7 = assign52280_e86487_d_n7;
        locals.var_t1_dn8 = assign52280_e86487_d_n8;
        locals.var_t1_dn9 = assign52280_e86487_d_n9;
        locals.var_t1_dn10 = assign52280_e86487_d_n10;
        locals.var_t1_dn11 = assign52280_e86487_d_n11;
        locals.var_t1_rv = 0.0;

        let assign52290_e86490: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard797 = assign52290_e86490;
        locals.var_guard797_rv = 0.0;

        let (assign52300_e86501, assign52300_e86501_d_n3, assign52300_e86501_d_n4, assign52300_e86501_d_n5, assign52300_e86501_d_n6, assign52300_e86501_d_n7, assign52300_e86501_d_n8, assign52300_e86501_d_n9, assign52300_e86501_d_n10, assign52300_e86501_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) {
        let assign52300_e86499: f64 = (1.0 - locals.var_t1);
        (assign52300_e86499, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign52300_e86501;
        locals.var_arg_dn3 = assign52300_e86501_d_n3;
        locals.var_arg_dn4 = assign52300_e86501_d_n4;
        locals.var_arg_dn5 = assign52300_e86501_d_n5;
        locals.var_arg_dn6 = assign52300_e86501_d_n6;
        locals.var_arg_dn7 = assign52300_e86501_d_n7;
        locals.var_arg_dn8 = assign52300_e86501_d_n8;
        locals.var_arg_dn9 = assign52300_e86501_d_n9;
        locals.var_arg_dn10 = assign52300_e86501_d_n10;
        locals.var_arg_dn11 = assign52300_e86501_d_n11;
        locals.var_arg_rv = 0.0;

        let assign52310_e86504: f64 = if p.p915 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard798 = assign52310_e86504;
        locals.var_guard798_rv = 0.0;

        let assign52320_e86507: f64 = if p.p915 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard799 = assign52320_e86507;
        locals.var_guard799_rv = 0.0;

        let (assign52330_e86523, assign52330_e86523_d_n3, assign52330_e86523_d_n4, assign52330_e86523_d_n5, assign52330_e86523_d_n6, assign52330_e86523_d_n7, assign52330_e86523_d_n8, assign52330_e86523_d_n9, assign52330_e86523_d_n10, assign52330_e86523_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) && (locals.var_guard798 != 0.0)) && (locals.var_guard799 != 0.0)) {
        let assign52330_e86520: f64 = (locals.var_arg).sqrt();
        let assign52330_e86521: f64 = (1.0 / assign52330_e86520);
        (assign52330_e86521, (-((locals.var_arg_dn3 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn4 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn5 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn6 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn7 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn8 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn9 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn10 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))), (-((locals.var_arg_dn11 / (2.0 * assign52330_e86520)) / (assign52330_e86520 * assign52330_e86520))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52330_e86523;
        locals.var_sarg_dn3 = assign52330_e86523_d_n3;
        locals.var_sarg_dn4 = assign52330_e86523_d_n4;
        locals.var_sarg_dn5 = assign52330_e86523_d_n5;
        locals.var_sarg_dn6 = assign52330_e86523_d_n6;
        locals.var_sarg_dn7 = assign52330_e86523_d_n7;
        locals.var_sarg_dn8 = assign52330_e86523_d_n8;
        locals.var_sarg_dn9 = assign52330_e86523_d_n9;
        locals.var_sarg_dn10 = assign52330_e86523_d_n10;
        locals.var_sarg_dn11 = assign52330_e86523_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign52340_e86542, assign52340_e86542_d_n3, assign52340_e86542_d_n4, assign52340_e86542_d_n5, assign52340_e86542_d_n6, assign52340_e86542_d_n7, assign52340_e86542_d_n8, assign52340_e86542_d_n9, assign52340_e86542_d_n10, assign52340_e86542_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) && (locals.var_guard798 != 0.0)) && (locals.var_guard799 == 0.0)) {
        let assign52340_e86536: f64 = (-p.p915);
        let assign52340_e86538: f64 = (locals.var_arg).ln();
        let assign52340_e86539: f64 = (assign52340_e86536 * assign52340_e86538);
        let assign52340_e86540: f64 = { let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign52340_e86540, ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign52340_e86539; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52340_e86536 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52340_e86542;
        locals.var_sarg_dn3 = assign52340_e86542_d_n3;
        locals.var_sarg_dn4 = assign52340_e86542_d_n4;
        locals.var_sarg_dn5 = assign52340_e86542_d_n5;
        locals.var_sarg_dn6 = assign52340_e86542_d_n6;
        locals.var_sarg_dn7 = assign52340_e86542_d_n7;
        locals.var_sarg_dn8 = assign52340_e86542_d_n8;
        locals.var_sarg_dn9 = assign52340_e86542_d_n9;
        locals.var_sarg_dn10 = assign52340_e86542_d_n10;
        locals.var_sarg_dn11 = assign52340_e86542_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign52350_e86565, assign52350_e86565_d_n3, assign52350_e86565_d_n4, assign52350_e86565_d_n5, assign52350_e86565_d_n6, assign52350_e86565_d_n7, assign52350_e86565_d_n8, assign52350_e86565_d_n9, assign52350_e86565_d_n10, assign52350_e86565_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) && (locals.var_guard798 != 0.0)) {
        let assign52350_e86553: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign52350_e86557: f64 = (locals.var_arg * locals.var_sarg);
        let assign52350_e86558: f64 = (1.0 - assign52350_e86557);
        let assign52350_e86559: f64 = (assign52350_e86553 * assign52350_e86558);
        let assign52350_e86562: f64 = (1.0 - p.p915);
        let assign52350_e86563: f64 = (assign52350_e86559 / assign52350_e86562);
        (assign52350_e86563, ((((locals.var_pbsws_t * locals.var_czbssw_dn3) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3))))) / assign52350_e86562), (((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign52350_e86562), (((((locals.var_pbsws_t_dn5 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn5)) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign52350_e86562), ((((locals.var_pbsws_t * locals.var_czbssw_dn6) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign52350_e86562), ((((locals.var_pbsws_t * locals.var_czbssw_dn7) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign52350_e86562), ((((locals.var_pbsws_t * locals.var_czbssw_dn8) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign52350_e86562), ((((locals.var_pbsws_t * locals.var_czbssw_dn9) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign52350_e86562), ((((locals.var_pbsws_t * locals.var_czbssw_dn10) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign52350_e86562), ((((locals.var_pbsws_t * locals.var_czbssw_dn11) * assign52350_e86558) + (assign52350_e86553 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign52350_e86562),)
    } else {
        (locals.var_qbsj2, locals.var_qbsj2_dn3, locals.var_qbsj2_dn4, locals.var_qbsj2_dn5, locals.var_qbsj2_dn6, locals.var_qbsj2_dn7, locals.var_qbsj2_dn8, locals.var_qbsj2_dn9, locals.var_qbsj2_dn10, locals.var_qbsj2_dn11,)
    }
};
        locals.var_qbsj2 = assign52350_e86565;
        locals.var_qbsj2_dn3 = assign52350_e86565_d_n3;
        locals.var_qbsj2_dn4 = assign52350_e86565_d_n4;
        locals.var_qbsj2_dn5 = assign52350_e86565_d_n5;
        locals.var_qbsj2_dn6 = assign52350_e86565_d_n6;
        locals.var_qbsj2_dn7 = assign52350_e86565_d_n7;
        locals.var_qbsj2_dn8 = assign52350_e86565_d_n8;
        locals.var_qbsj2_dn9 = assign52350_e86565_d_n9;
        locals.var_qbsj2_dn10 = assign52350_e86565_d_n10;
        locals.var_qbsj2_dn11 = assign52350_e86565_d_n11;
        locals.var_qbsj2_rv = 0.0;

        let (assign52360_e86583, assign52360_e86583_d_n3, assign52360_e86583_d_n4, assign52360_e86583_d_n5, assign52360_e86583_d_n6, assign52360_e86583_d_n7, assign52360_e86583_d_n8, assign52360_e86583_d_n9, assign52360_e86583_d_n10, assign52360_e86583_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 != 0.0)) && (locals.var_guard798 == 0.0)) {
        let assign52360_e86577: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign52360_e86579: f64 = (locals.var_arg).ln();
        let assign52360_e86580: f64 = (-assign52360_e86579);
        let assign52360_e86581: f64 = (assign52360_e86577 * assign52360_e86580);
        (assign52360_e86581, (((locals.var_pbsws_t * locals.var_czbssw_dn3) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn3 / locals.var_arg)))), ((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbsws_t_dn5 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn5)) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn5 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn6) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn6 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn7) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn7 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn8) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn8 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn9) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn9 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn10) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn10 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn11) * assign52360_e86580) + (assign52360_e86577 * (-(locals.var_arg_dn11 / locals.var_arg)))),)
    } else {
        (locals.var_qbsj2, locals.var_qbsj2_dn3, locals.var_qbsj2_dn4, locals.var_qbsj2_dn5, locals.var_qbsj2_dn6, locals.var_qbsj2_dn7, locals.var_qbsj2_dn8, locals.var_qbsj2_dn9, locals.var_qbsj2_dn10, locals.var_qbsj2_dn11,)
    }
};
        locals.var_qbsj2 = assign52360_e86583;
        locals.var_qbsj2_dn3 = assign52360_e86583_d_n3;
        locals.var_qbsj2_dn4 = assign52360_e86583_d_n4;
        locals.var_qbsj2_dn5 = assign52360_e86583_d_n5;
        locals.var_qbsj2_dn6 = assign52360_e86583_d_n6;
        locals.var_qbsj2_dn7 = assign52360_e86583_d_n7;
        locals.var_qbsj2_dn8 = assign52360_e86583_d_n8;
        locals.var_qbsj2_dn9 = assign52360_e86583_d_n9;
        locals.var_qbsj2_dn10 = assign52360_e86583_d_n10;
        locals.var_qbsj2_dn11 = assign52360_e86583_d_n11;
        locals.var_qbsj2_rv = 0.0;

        let (assign52370_e86609, assign52370_e86609_d_n3, assign52370_e86609_d_n4, assign52370_e86609_d_n5, assign52370_e86609_d_n6, assign52370_e86609_d_n7, assign52370_e86609_d_n8, assign52370_e86609_d_n9, assign52370_e86609_d_n10, assign52370_e86609_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 == 0.0)) {
        let assign52370_e86594: f64 = (locals.var_t1 - 1.0);
        let assign52370_e86595: f64 = (locals.var_czbssw_p1 * assign52370_e86594);
        let assign52370_e86598: f64 = (5.0 * p.p915);
        let assign52370_e86601: f64 = (locals.var_t1 - 1.0);
        let assign52370_e86602: f64 = (assign52370_e86598 * assign52370_e86601);
        let assign52370_e86605: f64 = (1.0 + p.p915);
        let assign52370_e86606: f64 = (assign52370_e86602 + assign52370_e86605);
        let assign52370_e86607: f64 = (assign52370_e86595 * assign52370_e86606);
        (assign52370_e86607, (((locals.var_czbssw_p1 * locals.var_t1_dn3) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn3))), (((locals.var_czbssw_p1 * locals.var_t1_dn4) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn4))), (((locals.var_czbssw_p1 * locals.var_t1_dn5) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn5))), (((locals.var_czbssw_p1 * locals.var_t1_dn6) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn6))), (((locals.var_czbssw_p1 * locals.var_t1_dn7) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn7))), (((locals.var_czbssw_p1 * locals.var_t1_dn8) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn8))), (((locals.var_czbssw_p1 * locals.var_t1_dn9) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn9))), (((locals.var_czbssw_p1 * locals.var_t1_dn10) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn10))), (((locals.var_czbssw_p1 * locals.var_t1_dn11) * assign52370_e86606) + (assign52370_e86595 * (assign52370_e86598 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign52370_e86609;
        locals.var_t2_dn3 = assign52370_e86609_d_n3;
        locals.var_t2_dn4 = assign52370_e86609_d_n4;
        locals.var_t2_dn5 = assign52370_e86609_d_n5;
        locals.var_t2_dn6 = assign52370_e86609_d_n6;
        locals.var_t2_dn7 = assign52370_e86609_d_n7;
        locals.var_t2_dn8 = assign52370_e86609_d_n8;
        locals.var_t2_dn9 = assign52370_e86609_d_n9;
        locals.var_t2_dn10 = assign52370_e86609_d_n10;
        locals.var_t2_dn11 = assign52370_e86609_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign52380_e86625, assign52380_e86625_d_n3, assign52380_e86625_d_n4, assign52380_e86625_d_n5, assign52380_e86625_d_n6, assign52380_e86625_d_n7, assign52380_e86625_d_n8, assign52380_e86625_d_n9, assign52380_e86625_d_n10, assign52380_e86625_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard796 != 0.0)) && (locals.var_guard797 == 0.0)) {
        let assign52380_e86619: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign52380_e86622: f64 = (locals.var_t2 + locals.var_czbssw_p2);
        let assign52380_e86623: f64 = (assign52380_e86619 * assign52380_e86622);
        (assign52380_e86623, (((locals.var_pbsws_t * locals.var_czbssw_dn3) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn3)), ((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn4)), ((((locals.var_pbsws_t_dn5 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn5)) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn5)), (((locals.var_pbsws_t * locals.var_czbssw_dn6) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn6)), (((locals.var_pbsws_t * locals.var_czbssw_dn7) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn7)), (((locals.var_pbsws_t * locals.var_czbssw_dn8) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn8)), (((locals.var_pbsws_t * locals.var_czbssw_dn9) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn9)), (((locals.var_pbsws_t * locals.var_czbssw_dn10) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn10)), (((locals.var_pbsws_t * locals.var_czbssw_dn11) * assign52380_e86622) + (assign52380_e86619 * locals.var_t2_dn11)),)
    } else {
        (locals.var_qbsj2, locals.var_qbsj2_dn3, locals.var_qbsj2_dn4, locals.var_qbsj2_dn5, locals.var_qbsj2_dn6, locals.var_qbsj2_dn7, locals.var_qbsj2_dn8, locals.var_qbsj2_dn9, locals.var_qbsj2_dn10, locals.var_qbsj2_dn11,)
    }
};
        locals.var_qbsj2 = assign52380_e86625;
        locals.var_qbsj2_dn3 = assign52380_e86625_d_n3;
        locals.var_qbsj2_dn4 = assign52380_e86625_d_n4;
        locals.var_qbsj2_dn5 = assign52380_e86625_d_n5;
        locals.var_qbsj2_dn6 = assign52380_e86625_d_n6;
        locals.var_qbsj2_dn7 = assign52380_e86625_d_n7;
        locals.var_qbsj2_dn8 = assign52380_e86625_d_n8;
        locals.var_qbsj2_dn9 = assign52380_e86625_d_n9;
        locals.var_qbsj2_dn10 = assign52380_e86625_d_n10;
        locals.var_qbsj2_dn11 = assign52380_e86625_d_n11;
        locals.var_qbsj2_rv = 0.0;

        let (assign52390_e86633, assign52390_e86633_d_n3, assign52390_e86633_d_n4, assign52390_e86633_d_n5, assign52390_e86633_d_n6, assign52390_e86633_d_n7, assign52390_e86633_d_n8, assign52390_e86633_d_n9, assign52390_e86633_d_n10, assign52390_e86633_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard796 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbsj2, locals.var_qbsj2_dn3, locals.var_qbsj2_dn4, locals.var_qbsj2_dn5, locals.var_qbsj2_dn6, locals.var_qbsj2_dn7, locals.var_qbsj2_dn8, locals.var_qbsj2_dn9, locals.var_qbsj2_dn10, locals.var_qbsj2_dn11,)
    }
};
        locals.var_qbsj2 = assign52390_e86633;
        locals.var_qbsj2_dn3 = assign52390_e86633_d_n3;
        locals.var_qbsj2_dn4 = assign52390_e86633_d_n4;
        locals.var_qbsj2_dn5 = assign52390_e86633_d_n5;
        locals.var_qbsj2_dn6 = assign52390_e86633_d_n6;
        locals.var_qbsj2_dn7 = assign52390_e86633_d_n7;
        locals.var_qbsj2_dn8 = assign52390_e86633_d_n8;
        locals.var_qbsj2_dn9 = assign52390_e86633_d_n9;
        locals.var_qbsj2_dn10 = assign52390_e86633_d_n10;
        locals.var_qbsj2_dn11 = assign52390_e86633_d_n11;
        locals.var_qbsj2_rv = 0.0;

        let assign52400_e86636: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard800 = assign52400_e86636;
        locals.var_guard800_rv = 0.0;

        let (assign52410_e86645, assign52410_e86645_d_n3, assign52410_e86645_d_n4, assign52410_e86645_d_n5, assign52410_e86645_d_n6, assign52410_e86645_d_n7, assign52410_e86645_d_n8, assign52410_e86645_d_n9, assign52410_e86645_d_n10, assign52410_e86645_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) {
        let assign52410_e86643: f64 = (locals.var_vbs_jct / locals.var_pbswgs_t);
        (assign52410_e86643, 0.0, (-((locals.var_vbs_jct * locals.var_pbswgs_t_dn4) / (locals.var_pbswgs_t * locals.var_pbswgs_t))), (-((locals.var_vbs_jct * locals.var_pbswgs_t_dn5) / (locals.var_pbswgs_t * locals.var_pbswgs_t))), 0.0, (locals.var_vbs_jct_dn7 / locals.var_pbswgs_t), 0.0, 0.0, (locals.var_vbs_jct_dn10 / locals.var_pbswgs_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign52410_e86645;
        locals.var_t1_dn3 = assign52410_e86645_d_n3;
        locals.var_t1_dn4 = assign52410_e86645_d_n4;
        locals.var_t1_dn5 = assign52410_e86645_d_n5;
        locals.var_t1_dn6 = assign52410_e86645_d_n6;
        locals.var_t1_dn7 = assign52410_e86645_d_n7;
        locals.var_t1_dn8 = assign52410_e86645_d_n8;
        locals.var_t1_dn9 = assign52410_e86645_d_n9;
        locals.var_t1_dn10 = assign52410_e86645_d_n10;
        locals.var_t1_dn11 = assign52410_e86645_d_n11;
        locals.var_t1_rv = 0.0;

        let assign52420_e86648: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard801 = assign52420_e86648;
        locals.var_guard801_rv = 0.0;

        let (assign52430_e86659, assign52430_e86659_d_n3, assign52430_e86659_d_n4, assign52430_e86659_d_n5, assign52430_e86659_d_n6, assign52430_e86659_d_n7, assign52430_e86659_d_n8, assign52430_e86659_d_n9, assign52430_e86659_d_n10, assign52430_e86659_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) {
        let assign52430_e86657: f64 = (1.0 - locals.var_t1);
        (assign52430_e86657, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign52430_e86659;
        locals.var_arg_dn3 = assign52430_e86659_d_n3;
        locals.var_arg_dn4 = assign52430_e86659_d_n4;
        locals.var_arg_dn5 = assign52430_e86659_d_n5;
        locals.var_arg_dn6 = assign52430_e86659_d_n6;
        locals.var_arg_dn7 = assign52430_e86659_d_n7;
        locals.var_arg_dn8 = assign52430_e86659_d_n8;
        locals.var_arg_dn9 = assign52430_e86659_d_n9;
        locals.var_arg_dn10 = assign52430_e86659_d_n10;
        locals.var_arg_dn11 = assign52430_e86659_d_n11;
        locals.var_arg_rv = 0.0;

        let assign52440_e86662: f64 = if p.p917 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard802 = assign52440_e86662;
        locals.var_guard802_rv = 0.0;

        let assign52450_e86665: f64 = if p.p917 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard803 = assign52450_e86665;
        locals.var_guard803_rv = 0.0;

        let (assign52460_e86681, assign52460_e86681_d_n3, assign52460_e86681_d_n4, assign52460_e86681_d_n5, assign52460_e86681_d_n6, assign52460_e86681_d_n7, assign52460_e86681_d_n8, assign52460_e86681_d_n9, assign52460_e86681_d_n10, assign52460_e86681_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) && (locals.var_guard802 != 0.0)) && (locals.var_guard803 != 0.0)) {
        let assign52460_e86678: f64 = (locals.var_arg).sqrt();
        let assign52460_e86679: f64 = (1.0 / assign52460_e86678);
        (assign52460_e86679, (-((locals.var_arg_dn3 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn4 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn5 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn6 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn7 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn8 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn9 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn10 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))), (-((locals.var_arg_dn11 / (2.0 * assign52460_e86678)) / (assign52460_e86678 * assign52460_e86678))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52460_e86681;
        locals.var_sarg_dn3 = assign52460_e86681_d_n3;
        locals.var_sarg_dn4 = assign52460_e86681_d_n4;
        locals.var_sarg_dn5 = assign52460_e86681_d_n5;
        locals.var_sarg_dn6 = assign52460_e86681_d_n6;
        locals.var_sarg_dn7 = assign52460_e86681_d_n7;
        locals.var_sarg_dn8 = assign52460_e86681_d_n8;
        locals.var_sarg_dn9 = assign52460_e86681_d_n9;
        locals.var_sarg_dn10 = assign52460_e86681_d_n10;
        locals.var_sarg_dn11 = assign52460_e86681_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign52470_e86700, assign52470_e86700_d_n3, assign52470_e86700_d_n4, assign52470_e86700_d_n5, assign52470_e86700_d_n6, assign52470_e86700_d_n7, assign52470_e86700_d_n8, assign52470_e86700_d_n9, assign52470_e86700_d_n10, assign52470_e86700_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) && (locals.var_guard802 != 0.0)) && (locals.var_guard803 == 0.0)) {
        let assign52470_e86694: f64 = (-p.p917);
        let assign52470_e86696: f64 = (locals.var_arg).ln();
        let assign52470_e86697: f64 = (assign52470_e86694 * assign52470_e86696);
        let assign52470_e86698: f64 = { let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign52470_e86698, ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign52470_e86697; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52470_e86694 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52470_e86700;
        locals.var_sarg_dn3 = assign52470_e86700_d_n3;
        locals.var_sarg_dn4 = assign52470_e86700_d_n4;
        locals.var_sarg_dn5 = assign52470_e86700_d_n5;
        locals.var_sarg_dn6 = assign52470_e86700_d_n6;
        locals.var_sarg_dn7 = assign52470_e86700_d_n7;
        locals.var_sarg_dn8 = assign52470_e86700_d_n8;
        locals.var_sarg_dn9 = assign52470_e86700_d_n9;
        locals.var_sarg_dn10 = assign52470_e86700_d_n10;
        locals.var_sarg_dn11 = assign52470_e86700_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign52480_e86723, assign52480_e86723_d_n3, assign52480_e86723_d_n4, assign52480_e86723_d_n5, assign52480_e86723_d_n6, assign52480_e86723_d_n7, assign52480_e86723_d_n8, assign52480_e86723_d_n9, assign52480_e86723_d_n10, assign52480_e86723_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) && (locals.var_guard802 != 0.0)) {
        let assign52480_e86711: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign52480_e86715: f64 = (locals.var_arg * locals.var_sarg);
        let assign52480_e86716: f64 = (1.0 - assign52480_e86715);
        let assign52480_e86717: f64 = (assign52480_e86711 * assign52480_e86716);
        let assign52480_e86720: f64 = (1.0 - p.p917);
        let assign52480_e86721: f64 = (assign52480_e86717 / assign52480_e86720);
        (assign52480_e86721, ((assign52480_e86711 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3)))) / assign52480_e86720), (((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign52480_e86716) + (assign52480_e86711 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign52480_e86720), (((((locals.var_pbswgs_t_dn5 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn5)) * assign52480_e86716) + (assign52480_e86711 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign52480_e86720), ((assign52480_e86711 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6)))) / assign52480_e86720), ((assign52480_e86711 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7)))) / assign52480_e86720), ((assign52480_e86711 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8)))) / assign52480_e86720), ((assign52480_e86711 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9)))) / assign52480_e86720), ((assign52480_e86711 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10)))) / assign52480_e86720), ((assign52480_e86711 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11)))) / assign52480_e86720),)
    } else {
        (locals.var_qbsj3, locals.var_qbsj3_dn3, locals.var_qbsj3_dn4, locals.var_qbsj3_dn5, locals.var_qbsj3_dn6, locals.var_qbsj3_dn7, locals.var_qbsj3_dn8, locals.var_qbsj3_dn9, locals.var_qbsj3_dn10, locals.var_qbsj3_dn11,)
    }
};
        locals.var_qbsj3 = assign52480_e86723;
        locals.var_qbsj3_dn3 = assign52480_e86723_d_n3;
        locals.var_qbsj3_dn4 = assign52480_e86723_d_n4;
        locals.var_qbsj3_dn5 = assign52480_e86723_d_n5;
        locals.var_qbsj3_dn6 = assign52480_e86723_d_n6;
        locals.var_qbsj3_dn7 = assign52480_e86723_d_n7;
        locals.var_qbsj3_dn8 = assign52480_e86723_d_n8;
        locals.var_qbsj3_dn9 = assign52480_e86723_d_n9;
        locals.var_qbsj3_dn10 = assign52480_e86723_d_n10;
        locals.var_qbsj3_dn11 = assign52480_e86723_d_n11;
        locals.var_qbsj3_rv = 0.0;

        let (assign52490_e86741, assign52490_e86741_d_n3, assign52490_e86741_d_n4, assign52490_e86741_d_n5, assign52490_e86741_d_n6, assign52490_e86741_d_n7, assign52490_e86741_d_n8, assign52490_e86741_d_n9, assign52490_e86741_d_n10, assign52490_e86741_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 != 0.0)) && (locals.var_guard802 == 0.0)) {
        let assign52490_e86735: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign52490_e86737: f64 = (locals.var_arg).ln();
        let assign52490_e86738: f64 = (-assign52490_e86737);
        let assign52490_e86739: f64 = (assign52490_e86735 * assign52490_e86738);
        (assign52490_e86739, (assign52490_e86735 * (-(locals.var_arg_dn3 / locals.var_arg))), ((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign52490_e86738) + (assign52490_e86735 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbswgs_t_dn5 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn5)) * assign52490_e86738) + (assign52490_e86735 * (-(locals.var_arg_dn5 / locals.var_arg)))), (assign52490_e86735 * (-(locals.var_arg_dn6 / locals.var_arg))), (assign52490_e86735 * (-(locals.var_arg_dn7 / locals.var_arg))), (assign52490_e86735 * (-(locals.var_arg_dn8 / locals.var_arg))), (assign52490_e86735 * (-(locals.var_arg_dn9 / locals.var_arg))), (assign52490_e86735 * (-(locals.var_arg_dn10 / locals.var_arg))), (assign52490_e86735 * (-(locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_qbsj3, locals.var_qbsj3_dn3, locals.var_qbsj3_dn4, locals.var_qbsj3_dn5, locals.var_qbsj3_dn6, locals.var_qbsj3_dn7, locals.var_qbsj3_dn8, locals.var_qbsj3_dn9, locals.var_qbsj3_dn10, locals.var_qbsj3_dn11,)
    }
};
        locals.var_qbsj3 = assign52490_e86741;
        locals.var_qbsj3_dn3 = assign52490_e86741_d_n3;
        locals.var_qbsj3_dn4 = assign52490_e86741_d_n4;
        locals.var_qbsj3_dn5 = assign52490_e86741_d_n5;
        locals.var_qbsj3_dn6 = assign52490_e86741_d_n6;
        locals.var_qbsj3_dn7 = assign52490_e86741_d_n7;
        locals.var_qbsj3_dn8 = assign52490_e86741_d_n8;
        locals.var_qbsj3_dn9 = assign52490_e86741_d_n9;
        locals.var_qbsj3_dn10 = assign52490_e86741_d_n10;
        locals.var_qbsj3_dn11 = assign52490_e86741_d_n11;
        locals.var_qbsj3_rv = 0.0;

        let (assign52500_e86767, assign52500_e86767_d_n3, assign52500_e86767_d_n4, assign52500_e86767_d_n5, assign52500_e86767_d_n6, assign52500_e86767_d_n7, assign52500_e86767_d_n8, assign52500_e86767_d_n9, assign52500_e86767_d_n10, assign52500_e86767_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 == 0.0)) {
        let assign52500_e86752: f64 = (locals.var_t1 - 1.0);
        let assign52500_e86753: f64 = (locals.var_czbsswg_p1 * assign52500_e86752);
        let assign52500_e86756: f64 = (5.0 * p.p917);
        let assign52500_e86759: f64 = (locals.var_t1 - 1.0);
        let assign52500_e86760: f64 = (assign52500_e86756 * assign52500_e86759);
        let assign52500_e86763: f64 = (1.0 + p.p917);
        let assign52500_e86764: f64 = (assign52500_e86760 + assign52500_e86763);
        let assign52500_e86765: f64 = (assign52500_e86753 * assign52500_e86764);
        (assign52500_e86765, (((locals.var_czbsswg_p1 * locals.var_t1_dn3) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn3))), (((locals.var_czbsswg_p1 * locals.var_t1_dn4) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn4))), (((locals.var_czbsswg_p1 * locals.var_t1_dn5) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn5))), (((locals.var_czbsswg_p1 * locals.var_t1_dn6) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn6))), (((locals.var_czbsswg_p1 * locals.var_t1_dn7) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn7))), (((locals.var_czbsswg_p1 * locals.var_t1_dn8) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn8))), (((locals.var_czbsswg_p1 * locals.var_t1_dn9) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn9))), (((locals.var_czbsswg_p1 * locals.var_t1_dn10) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn10))), (((locals.var_czbsswg_p1 * locals.var_t1_dn11) * assign52500_e86764) + (assign52500_e86753 * (assign52500_e86756 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign52500_e86767;
        locals.var_t2_dn3 = assign52500_e86767_d_n3;
        locals.var_t2_dn4 = assign52500_e86767_d_n4;
        locals.var_t2_dn5 = assign52500_e86767_d_n5;
        locals.var_t2_dn6 = assign52500_e86767_d_n6;
        locals.var_t2_dn7 = assign52500_e86767_d_n7;
        locals.var_t2_dn8 = assign52500_e86767_d_n8;
        locals.var_t2_dn9 = assign52500_e86767_d_n9;
        locals.var_t2_dn10 = assign52500_e86767_d_n10;
        locals.var_t2_dn11 = assign52500_e86767_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign52510_e86783, assign52510_e86783_d_n3, assign52510_e86783_d_n4, assign52510_e86783_d_n5, assign52510_e86783_d_n6, assign52510_e86783_d_n7, assign52510_e86783_d_n8, assign52510_e86783_d_n9, assign52510_e86783_d_n10, assign52510_e86783_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard800 != 0.0)) && (locals.var_guard801 == 0.0)) {
        let assign52510_e86777: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign52510_e86780: f64 = (locals.var_t2 + locals.var_czbsswg_p2);
        let assign52510_e86781: f64 = (assign52510_e86777 * assign52510_e86780);
        (assign52510_e86781, (assign52510_e86777 * locals.var_t2_dn3), ((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign52510_e86780) + (assign52510_e86777 * locals.var_t2_dn4)), ((((locals.var_pbswgs_t_dn5 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn5)) * assign52510_e86780) + (assign52510_e86777 * locals.var_t2_dn5)), (assign52510_e86777 * locals.var_t2_dn6), (assign52510_e86777 * locals.var_t2_dn7), (assign52510_e86777 * locals.var_t2_dn8), (assign52510_e86777 * locals.var_t2_dn9), (assign52510_e86777 * locals.var_t2_dn10), (assign52510_e86777 * locals.var_t2_dn11),)
    } else {
        (locals.var_qbsj3, locals.var_qbsj3_dn3, locals.var_qbsj3_dn4, locals.var_qbsj3_dn5, locals.var_qbsj3_dn6, locals.var_qbsj3_dn7, locals.var_qbsj3_dn8, locals.var_qbsj3_dn9, locals.var_qbsj3_dn10, locals.var_qbsj3_dn11,)
    }
};
        locals.var_qbsj3 = assign52510_e86783;
        locals.var_qbsj3_dn3 = assign52510_e86783_d_n3;
        locals.var_qbsj3_dn4 = assign52510_e86783_d_n4;
        locals.var_qbsj3_dn5 = assign52510_e86783_d_n5;
        locals.var_qbsj3_dn6 = assign52510_e86783_d_n6;
        locals.var_qbsj3_dn7 = assign52510_e86783_d_n7;
        locals.var_qbsj3_dn8 = assign52510_e86783_d_n8;
        locals.var_qbsj3_dn9 = assign52510_e86783_d_n9;
        locals.var_qbsj3_dn10 = assign52510_e86783_d_n10;
        locals.var_qbsj3_dn11 = assign52510_e86783_d_n11;
        locals.var_qbsj3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_181(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign52520_e86791, assign52520_e86791_d_n3, assign52520_e86791_d_n4, assign52520_e86791_d_n5, assign52520_e86791_d_n6, assign52520_e86791_d_n7, assign52520_e86791_d_n8, assign52520_e86791_d_n9, assign52520_e86791_d_n10, assign52520_e86791_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard800 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbsj3, locals.var_qbsj3_dn3, locals.var_qbsj3_dn4, locals.var_qbsj3_dn5, locals.var_qbsj3_dn6, locals.var_qbsj3_dn7, locals.var_qbsj3_dn8, locals.var_qbsj3_dn9, locals.var_qbsj3_dn10, locals.var_qbsj3_dn11,)
    }
};
        locals.var_qbsj3 = assign52520_e86791;
        locals.var_qbsj3_dn3 = assign52520_e86791_d_n3;
        locals.var_qbsj3_dn4 = assign52520_e86791_d_n4;
        locals.var_qbsj3_dn5 = assign52520_e86791_d_n5;
        locals.var_qbsj3_dn6 = assign52520_e86791_d_n6;
        locals.var_qbsj3_dn7 = assign52520_e86791_d_n7;
        locals.var_qbsj3_dn8 = assign52520_e86791_d_n8;
        locals.var_qbsj3_dn9 = assign52520_e86791_d_n9;
        locals.var_qbsj3_dn10 = assign52520_e86791_d_n10;
        locals.var_qbsj3_dn11 = assign52520_e86791_d_n11;
        locals.var_qbsj3_rv = 0.0;

        let (assign52530_e86800, assign52530_e86800_d_n3, assign52530_e86800_d_n4, assign52530_e86800_d_n5, assign52530_e86800_d_n6, assign52530_e86800_d_n7, assign52530_e86800_d_n8, assign52530_e86800_d_n9, assign52530_e86800_d_n10, assign52530_e86800_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52530_e86796: f64 = (p.p919 * locals.var_ibsdif);
        let assign52530_e86798: f64 = (assign52530_e86796 * p.p2);
        (assign52530_e86798, ((p.p919 * locals.var_ibsdif_dn3) * p.p2), ((p.p919 * locals.var_ibsdif_dn4) * p.p2), ((p.p919 * locals.var_ibsdif_dn5) * p.p2), ((p.p919 * locals.var_ibsdif_dn6) * p.p2), ((p.p919 * locals.var_ibsdif_dn7) * p.p2), ((p.p919 * locals.var_ibsdif_dn8) * p.p2), ((p.p919 * locals.var_ibsdif_dn9) * p.p2), ((p.p919 * locals.var_ibsdif_dn10) * p.p2), ((p.p919 * locals.var_ibsdif_dn11) * p.p2),)
    } else {
        (locals.var_qbsj4, locals.var_qbsj4_dn3, locals.var_qbsj4_dn4, locals.var_qbsj4_dn5, locals.var_qbsj4_dn6, locals.var_qbsj4_dn7, locals.var_qbsj4_dn8, locals.var_qbsj4_dn9, locals.var_qbsj4_dn10, locals.var_qbsj4_dn11,)
    }
};
        locals.var_qbsj4 = assign52530_e86800;
        locals.var_qbsj4_dn3 = assign52530_e86800_d_n3;
        locals.var_qbsj4_dn4 = assign52530_e86800_d_n4;
        locals.var_qbsj4_dn5 = assign52530_e86800_d_n5;
        locals.var_qbsj4_dn6 = assign52530_e86800_d_n6;
        locals.var_qbsj4_dn7 = assign52530_e86800_d_n7;
        locals.var_qbsj4_dn8 = assign52530_e86800_d_n8;
        locals.var_qbsj4_dn9 = assign52530_e86800_d_n9;
        locals.var_qbsj4_dn10 = assign52530_e86800_d_n10;
        locals.var_qbsj4_dn11 = assign52530_e86800_d_n11;
        locals.var_qbsj4_rv = 0.0;

        let (assign52540_e86811, assign52540_e86811_d_n3, assign52540_e86811_d_n4, assign52540_e86811_d_n5, assign52540_e86811_d_n6, assign52540_e86811_d_n7, assign52540_e86811_d_n8, assign52540_e86811_d_n9, assign52540_e86811_d_n10, assign52540_e86811_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52540_e86805: f64 = (locals.var_qbsj1 + locals.var_qbsj2);
        let assign52540_e86807: f64 = (assign52540_e86805 + locals.var_qbsj3);
        let assign52540_e86809: f64 = (assign52540_e86807 + locals.var_qbsj4);
        (assign52540_e86809, (((locals.var_qbsj1_dn3 + locals.var_qbsj2_dn3) + locals.var_qbsj3_dn3) + locals.var_qbsj4_dn3), (((locals.var_qbsj1_dn4 + locals.var_qbsj2_dn4) + locals.var_qbsj3_dn4) + locals.var_qbsj4_dn4), (((locals.var_qbsj1_dn5 + locals.var_qbsj2_dn5) + locals.var_qbsj3_dn5) + locals.var_qbsj4_dn5), (((locals.var_qbsj1_dn6 + locals.var_qbsj2_dn6) + locals.var_qbsj3_dn6) + locals.var_qbsj4_dn6), (((locals.var_qbsj1_dn7 + locals.var_qbsj2_dn7) + locals.var_qbsj3_dn7) + locals.var_qbsj4_dn7), (((locals.var_qbsj1_dn8 + locals.var_qbsj2_dn8) + locals.var_qbsj3_dn8) + locals.var_qbsj4_dn8), (((locals.var_qbsj1_dn9 + locals.var_qbsj2_dn9) + locals.var_qbsj3_dn9) + locals.var_qbsj4_dn9), (((locals.var_qbsj1_dn10 + locals.var_qbsj2_dn10) + locals.var_qbsj3_dn10) + locals.var_qbsj4_dn10), (((locals.var_qbsj1_dn11 + locals.var_qbsj2_dn11) + locals.var_qbsj3_dn11) + locals.var_qbsj4_dn11),)
    } else {
        (locals.var_qbsj, locals.var_qbsj_dn3, locals.var_qbsj_dn4, locals.var_qbsj_dn5, locals.var_qbsj_dn6, locals.var_qbsj_dn7, locals.var_qbsj_dn8, locals.var_qbsj_dn9, locals.var_qbsj_dn10, locals.var_qbsj_dn11,)
    }
};
        locals.var_qbsj = assign52540_e86811;
        locals.var_qbsj_dn3 = assign52540_e86811_d_n3;
        locals.var_qbsj_dn4 = assign52540_e86811_d_n4;
        locals.var_qbsj_dn5 = assign52540_e86811_d_n5;
        locals.var_qbsj_dn6 = assign52540_e86811_d_n6;
        locals.var_qbsj_dn7 = assign52540_e86811_d_n7;
        locals.var_qbsj_dn8 = assign52540_e86811_d_n8;
        locals.var_qbsj_dn9 = assign52540_e86811_d_n9;
        locals.var_qbsj_dn10 = assign52540_e86811_d_n10;
        locals.var_qbsj_dn11 = assign52540_e86811_d_n11;
        locals.var_qbsj_rv = 0.0;

        let (assign52550_e86818, assign52550_e86818_d_n3, assign52550_e86818_d_n4, assign52550_e86818_d_n5, assign52550_e86818_d_n6, assign52550_e86818_d_n7, assign52550_e86818_d_n8, assign52550_e86818_d_n9, assign52550_e86818_d_n10, assign52550_e86818_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52550_e86816: f64 = (locals.var_cjd_t * locals.var_adeff);
        (assign52550_e86816, (locals.var_cjd_t * locals.var_adeff_dn3), ((locals.var_cjd_t_dn4 * locals.var_adeff) + (locals.var_cjd_t * locals.var_adeff_dn4)), ((locals.var_cjd_t_dn5 * locals.var_adeff) + (locals.var_cjd_t * locals.var_adeff_dn5)), (locals.var_cjd_t * locals.var_adeff_dn6), (locals.var_cjd_t * locals.var_adeff_dn7), (locals.var_cjd_t * locals.var_adeff_dn8), (locals.var_cjd_t * locals.var_adeff_dn9), (locals.var_cjd_t * locals.var_adeff_dn10), (locals.var_cjd_t * locals.var_adeff_dn11),)
    } else {
        (locals.var_czbd, locals.var_czbd_dn3, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn11,)
    }
};
        locals.var_czbd = assign52550_e86818;
        locals.var_czbd_dn3 = assign52550_e86818_d_n3;
        locals.var_czbd_dn4 = assign52550_e86818_d_n4;
        locals.var_czbd_dn5 = assign52550_e86818_d_n5;
        locals.var_czbd_dn6 = assign52550_e86818_d_n6;
        locals.var_czbd_dn7 = assign52550_e86818_d_n7;
        locals.var_czbd_dn8 = assign52550_e86818_d_n8;
        locals.var_czbd_dn9 = assign52550_e86818_d_n9;
        locals.var_czbd_dn10 = assign52550_e86818_d_n10;
        locals.var_czbd_dn11 = assign52550_e86818_d_n11;
        locals.var_czbd_rv = 0.0;

        let (assign52560_e86825, assign52560_e86825_d_n3, assign52560_e86825_d_n4, assign52560_e86825_d_n5, assign52560_e86825_d_n6, assign52560_e86825_d_n7, assign52560_e86825_d_n8, assign52560_e86825_d_n9, assign52560_e86825_d_n10, assign52560_e86825_d_n11,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52560_e86823: f64 = (locals.var_cjswd_t * locals.var_pdeff);
        (assign52560_e86823, (locals.var_cjswd_t * locals.var_pdeff_dn3), ((locals.var_cjswd_t_dn4 * locals.var_pdeff) + (locals.var_cjswd_t * locals.var_pdeff_dn4)), ((locals.var_cjswd_t_dn5 * locals.var_pdeff) + (locals.var_cjswd_t * locals.var_pdeff_dn5)), (locals.var_cjswd_t * locals.var_pdeff_dn6), (locals.var_cjswd_t * locals.var_pdeff_dn7), (locals.var_cjswd_t * locals.var_pdeff_dn8), (locals.var_cjswd_t * locals.var_pdeff_dn9), (locals.var_cjswd_t * locals.var_pdeff_dn10), (locals.var_cjswd_t * locals.var_pdeff_dn11),)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn3, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11,)
    }
};
        locals.var_czbdsw = assign52560_e86825;
        locals.var_czbdsw_dn3 = assign52560_e86825_d_n3;
        locals.var_czbdsw_dn4 = assign52560_e86825_d_n4;
        locals.var_czbdsw_dn5 = assign52560_e86825_d_n5;
        locals.var_czbdsw_dn6 = assign52560_e86825_d_n6;
        locals.var_czbdsw_dn7 = assign52560_e86825_d_n7;
        locals.var_czbdsw_dn8 = assign52560_e86825_d_n8;
        locals.var_czbdsw_dn9 = assign52560_e86825_d_n9;
        locals.var_czbdsw_dn10 = assign52560_e86825_d_n10;
        locals.var_czbdsw_dn11 = assign52560_e86825_d_n11;
        locals.var_czbdsw_rv = 0.0;

        let (assign52570_e86834, assign52570_e86834_d_n4, assign52570_e86834_d_n5,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52570_e86830: f64 = (locals.var_cjswgd_t * locals.var_weffcj);
        let assign52570_e86832: f64 = (assign52570_e86830 * p.p2);
        (assign52570_e86832, ((locals.var_cjswgd_t_dn4 * locals.var_weffcj) * p.p2), ((locals.var_cjswgd_t_dn5 * locals.var_weffcj) * p.p2),)
    } else {
        (locals.var_czbdswg, locals.var_czbdswg_dn4, locals.var_czbdswg_dn5,)
    }
};
        locals.var_czbdswg = assign52570_e86834;
        locals.var_czbdswg_dn4 = assign52570_e86834_d_n4;
        locals.var_czbdswg_dn5 = assign52570_e86834_d_n5;
        locals.var_czbdswg_rv = 0.0;

        let (assign52580_e86842,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52580_e86839: f64 = (-p.p914);
        let assign52580_e86840: f64 = (0.1_f64).powf(assign52580_e86839);
        (assign52580_e86840,)
    } else {
        (locals.var_czbd_p1,)
    }
};
        locals.var_czbd_p1 = assign52580_e86842;
        locals.var_czbd_p1_rv = 0.0;

        let assign52590_e86845: f64 = if p.p914 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard804 = assign52590_e86845;
        locals.var_guard804_rv = 0.0;

        let (assign52600_e86855,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard804 != 0.0)) {
        let assign52600_e86852: f64 = (0.1_f64).ln();
        let assign52600_e86853: f64 = (1.5 - assign52600_e86852);
        (assign52600_e86853,)
    } else {
        (locals.var_czbd_p2,)
    }
};
        locals.var_czbd_p2 = assign52600_e86855;
        locals.var_czbd_p2_rv = 0.0;

        let (assign52610_e86879,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard804 == 0.0)) {
        let assign52610_e86864: f64 = (1.0 - p.p914);
        let assign52610_e86865: f64 = (1.0 / assign52610_e86864);
        let assign52610_e86869: f64 = (0.05 * p.p914);
        let assign52610_e86872: f64 = (1.0 + p.p914);
        let assign52610_e86873: f64 = (assign52610_e86869 * assign52610_e86872);
        let assign52610_e86875: f64 = (assign52610_e86873 * locals.var_czbd_p1);
        let assign52610_e86876: f64 = (1.0 - assign52610_e86875);
        let assign52610_e86877: f64 = (assign52610_e86865 * assign52610_e86876);
        (assign52610_e86877,)
    } else {
        (locals.var_czbd_p2,)
    }
};
        locals.var_czbd_p2 = assign52610_e86879;
        locals.var_czbd_p2_rv = 0.0;

        let (assign52620_e86887,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52620_e86884: f64 = (-p.p916);
        let assign52620_e86885: f64 = (0.1_f64).powf(assign52620_e86884);
        (assign52620_e86885,)
    } else {
        (locals.var_czbdsw_p1,)
    }
};
        locals.var_czbdsw_p1 = assign52620_e86887;
        locals.var_czbdsw_p1_rv = 0.0;

        let assign52630_e86890: f64 = if p.p916 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard805 = assign52630_e86890;
        locals.var_guard805_rv = 0.0;

        let (assign52640_e86900,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard805 != 0.0)) {
        let assign52640_e86897: f64 = (0.1_f64).ln();
        let assign52640_e86898: f64 = (1.5 - assign52640_e86897);
        (assign52640_e86898,)
    } else {
        (locals.var_czbdsw_p2,)
    }
};
        locals.var_czbdsw_p2 = assign52640_e86900;
        locals.var_czbdsw_p2_rv = 0.0;

        let (assign52650_e86924,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard805 == 0.0)) {
        let assign52650_e86909: f64 = (1.0 - p.p916);
        let assign52650_e86910: f64 = (1.0 / assign52650_e86909);
        let assign52650_e86914: f64 = (0.05 * p.p916);
        let assign52650_e86917: f64 = (1.0 + p.p916);
        let assign52650_e86918: f64 = (assign52650_e86914 * assign52650_e86917);
        let assign52650_e86920: f64 = (assign52650_e86918 * locals.var_czbdsw_p1);
        let assign52650_e86921: f64 = (1.0 - assign52650_e86920);
        let assign52650_e86922: f64 = (assign52650_e86910 * assign52650_e86921);
        (assign52650_e86922,)
    } else {
        (locals.var_czbdsw_p2,)
    }
};
        locals.var_czbdsw_p2 = assign52650_e86924;
        locals.var_czbdsw_p2_rv = 0.0;

        let (assign52660_e86932,) = {
    if (locals.var_guard492 == 0.0) {
        let assign52660_e86929: f64 = (-p.p918);
        let assign52660_e86930: f64 = (0.1_f64).powf(assign52660_e86929);
        (assign52660_e86930,)
    } else {
        (locals.var_czbdswg_p1,)
    }
};
        locals.var_czbdswg_p1 = assign52660_e86932;
        locals.var_czbdswg_p1_rv = 0.0;

        let assign52670_e86935: f64 = if p.p918 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard806 = assign52670_e86935;
        locals.var_guard806_rv = 0.0;

        let (assign52680_e86945,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard806 != 0.0)) {
        let assign52680_e86942: f64 = (0.1_f64).ln();
        let assign52680_e86943: f64 = (1.5 - assign52680_e86942);
        (assign52680_e86943,)
    } else {
        (locals.var_czbdswg_p2,)
    }
};
        locals.var_czbdswg_p2 = assign52680_e86945;
        locals.var_czbdswg_p2_rv = 0.0;

        let (assign52690_e86969,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard806 == 0.0)) {
        let assign52690_e86954: f64 = (1.0 - p.p918);
        let assign52690_e86955: f64 = (1.0 / assign52690_e86954);
        let assign52690_e86959: f64 = (0.05 * p.p918);
        let assign52690_e86962: f64 = (1.0 + p.p918);
        let assign52690_e86963: f64 = (assign52690_e86959 * assign52690_e86962);
        let assign52690_e86965: f64 = (assign52690_e86963 * locals.var_czbdswg_p1);
        let assign52690_e86966: f64 = (1.0 - assign52690_e86965);
        let assign52690_e86967: f64 = (assign52690_e86955 * assign52690_e86966);
        (assign52690_e86967,)
    } else {
        (locals.var_czbdswg_p2,)
    }
};
        locals.var_czbdswg_p2 = assign52690_e86969;
        locals.var_czbdswg_p2_rv = 0.0;

        let assign52700_e86972: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard807 = assign52700_e86972;
        locals.var_guard807_rv = 0.0;

        let (assign52710_e86981, assign52710_e86981_d_n3, assign52710_e86981_d_n4, assign52710_e86981_d_n5, assign52710_e86981_d_n6, assign52710_e86981_d_n7, assign52710_e86981_d_n8, assign52710_e86981_d_n9, assign52710_e86981_d_n10, assign52710_e86981_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) {
        let assign52710_e86979: f64 = (locals.var_vbd_jct / locals.var_pbd_t);
        (assign52710_e86979, 0.0, (-((locals.var_vbd_jct * locals.var_pbd_t_dn4) / (locals.var_pbd_t * locals.var_pbd_t))), (-((locals.var_vbd_jct * locals.var_pbd_t_dn5) / (locals.var_pbd_t * locals.var_pbd_t))), (locals.var_vbd_jct_dn6 / locals.var_pbd_t), 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn10 / locals.var_pbd_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign52710_e86981;
        locals.var_t1_dn3 = assign52710_e86981_d_n3;
        locals.var_t1_dn4 = assign52710_e86981_d_n4;
        locals.var_t1_dn5 = assign52710_e86981_d_n5;
        locals.var_t1_dn6 = assign52710_e86981_d_n6;
        locals.var_t1_dn7 = assign52710_e86981_d_n7;
        locals.var_t1_dn8 = assign52710_e86981_d_n8;
        locals.var_t1_dn9 = assign52710_e86981_d_n9;
        locals.var_t1_dn10 = assign52710_e86981_d_n10;
        locals.var_t1_dn11 = assign52710_e86981_d_n11;
        locals.var_t1_rv = 0.0;

        let assign52720_e86984: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard808 = assign52720_e86984;
        locals.var_guard808_rv = 0.0;

        let (assign52730_e86995, assign52730_e86995_d_n3, assign52730_e86995_d_n4, assign52730_e86995_d_n5, assign52730_e86995_d_n6, assign52730_e86995_d_n7, assign52730_e86995_d_n8, assign52730_e86995_d_n9, assign52730_e86995_d_n10, assign52730_e86995_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) && (locals.var_guard808 != 0.0)) {
        let assign52730_e86993: f64 = (1.0 - locals.var_t1);
        (assign52730_e86993, (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11),)
    } else {
        (locals.var_arg, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11,)
    }
};
        locals.var_arg = assign52730_e86995;
        locals.var_arg_dn3 = assign52730_e86995_d_n3;
        locals.var_arg_dn4 = assign52730_e86995_d_n4;
        locals.var_arg_dn5 = assign52730_e86995_d_n5;
        locals.var_arg_dn6 = assign52730_e86995_d_n6;
        locals.var_arg_dn7 = assign52730_e86995_d_n7;
        locals.var_arg_dn8 = assign52730_e86995_d_n8;
        locals.var_arg_dn9 = assign52730_e86995_d_n9;
        locals.var_arg_dn10 = assign52730_e86995_d_n10;
        locals.var_arg_dn11 = assign52730_e86995_d_n11;
        locals.var_arg_rv = 0.0;

        let assign52740_e86998: f64 = if p.p914 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard809 = assign52740_e86998;
        locals.var_guard809_rv = 0.0;

        let assign52750_e87001: f64 = if p.p914 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard810 = assign52750_e87001;
        locals.var_guard810_rv = 0.0;

        let (assign52760_e87017, assign52760_e87017_d_n3, assign52760_e87017_d_n4, assign52760_e87017_d_n5, assign52760_e87017_d_n6, assign52760_e87017_d_n7, assign52760_e87017_d_n8, assign52760_e87017_d_n9, assign52760_e87017_d_n10, assign52760_e87017_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) && (locals.var_guard810 != 0.0)) {
        let assign52760_e87014: f64 = (locals.var_arg).sqrt();
        let assign52760_e87015: f64 = (1.0 / assign52760_e87014);
        (assign52760_e87015, (-((locals.var_arg_dn3 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn4 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn5 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn6 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn7 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn8 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn9 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn10 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))), (-((locals.var_arg_dn11 / (2.0 * assign52760_e87014)) / (assign52760_e87014 * assign52760_e87014))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52760_e87017;
        locals.var_sarg_dn3 = assign52760_e87017_d_n3;
        locals.var_sarg_dn4 = assign52760_e87017_d_n4;
        locals.var_sarg_dn5 = assign52760_e87017_d_n5;
        locals.var_sarg_dn6 = assign52760_e87017_d_n6;
        locals.var_sarg_dn7 = assign52760_e87017_d_n7;
        locals.var_sarg_dn8 = assign52760_e87017_d_n8;
        locals.var_sarg_dn9 = assign52760_e87017_d_n9;
        locals.var_sarg_dn10 = assign52760_e87017_d_n10;
        locals.var_sarg_dn11 = assign52760_e87017_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign52770_e87036, assign52770_e87036_d_n3, assign52770_e87036_d_n4, assign52770_e87036_d_n5, assign52770_e87036_d_n6, assign52770_e87036_d_n7, assign52770_e87036_d_n8, assign52770_e87036_d_n9, assign52770_e87036_d_n10, assign52770_e87036_d_n11,) = {
    if (((((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) && (locals.var_guard810 == 0.0)) {
        let assign52770_e87030: f64 = (-p.p914);
        let assign52770_e87032: f64 = (locals.var_arg).ln();
        let assign52770_e87033: f64 = (assign52770_e87030 * assign52770_e87032);
        let assign52770_e87034: f64 = { let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign52770_e87034, ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign52770_e87033; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign52770_e87030 * (locals.var_arg_dn11 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11,)
    }
};
        locals.var_sarg = assign52770_e87036;
        locals.var_sarg_dn3 = assign52770_e87036_d_n3;
        locals.var_sarg_dn4 = assign52770_e87036_d_n4;
        locals.var_sarg_dn5 = assign52770_e87036_d_n5;
        locals.var_sarg_dn6 = assign52770_e87036_d_n6;
        locals.var_sarg_dn7 = assign52770_e87036_d_n7;
        locals.var_sarg_dn8 = assign52770_e87036_d_n8;
        locals.var_sarg_dn9 = assign52770_e87036_d_n9;
        locals.var_sarg_dn10 = assign52770_e87036_d_n10;
        locals.var_sarg_dn11 = assign52770_e87036_d_n11;
        locals.var_sarg_rv = 0.0;

        let (assign52780_e87059, assign52780_e87059_d_n3, assign52780_e87059_d_n4, assign52780_e87059_d_n5, assign52780_e87059_d_n6, assign52780_e87059_d_n7, assign52780_e87059_d_n8, assign52780_e87059_d_n9, assign52780_e87059_d_n10, assign52780_e87059_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 != 0.0)) {
        let assign52780_e87047: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign52780_e87051: f64 = (locals.var_arg * locals.var_sarg);
        let assign52780_e87052: f64 = (1.0 - assign52780_e87051);
        let assign52780_e87053: f64 = (assign52780_e87047 * assign52780_e87052);
        let assign52780_e87056: f64 = (1.0 - p.p914);
        let assign52780_e87057: f64 = (assign52780_e87053 / assign52780_e87056);
        (assign52780_e87057, ((((locals.var_pbd_t * locals.var_czbd_dn3) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3))))) / assign52780_e87056), (((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign52780_e87056), (((((locals.var_pbd_t_dn5 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn5)) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign52780_e87056), ((((locals.var_pbd_t * locals.var_czbd_dn6) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign52780_e87056), ((((locals.var_pbd_t * locals.var_czbd_dn7) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign52780_e87056), ((((locals.var_pbd_t * locals.var_czbd_dn8) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign52780_e87056), ((((locals.var_pbd_t * locals.var_czbd_dn9) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign52780_e87056), ((((locals.var_pbd_t * locals.var_czbd_dn10) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign52780_e87056), ((((locals.var_pbd_t * locals.var_czbd_dn11) * assign52780_e87052) + (assign52780_e87047 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign52780_e87056),)
    } else {
        (locals.var_qbdj1, locals.var_qbdj1_dn3, locals.var_qbdj1_dn4, locals.var_qbdj1_dn5, locals.var_qbdj1_dn6, locals.var_qbdj1_dn7, locals.var_qbdj1_dn8, locals.var_qbdj1_dn9, locals.var_qbdj1_dn10, locals.var_qbdj1_dn11,)
    }
};
        locals.var_qbdj1 = assign52780_e87059;
        locals.var_qbdj1_dn3 = assign52780_e87059_d_n3;
        locals.var_qbdj1_dn4 = assign52780_e87059_d_n4;
        locals.var_qbdj1_dn5 = assign52780_e87059_d_n5;
        locals.var_qbdj1_dn6 = assign52780_e87059_d_n6;
        locals.var_qbdj1_dn7 = assign52780_e87059_d_n7;
        locals.var_qbdj1_dn8 = assign52780_e87059_d_n8;
        locals.var_qbdj1_dn9 = assign52780_e87059_d_n9;
        locals.var_qbdj1_dn10 = assign52780_e87059_d_n10;
        locals.var_qbdj1_dn11 = assign52780_e87059_d_n11;
        locals.var_qbdj1_rv = 0.0;

        let (assign52790_e87077, assign52790_e87077_d_n3, assign52790_e87077_d_n4, assign52790_e87077_d_n5, assign52790_e87077_d_n6, assign52790_e87077_d_n7, assign52790_e87077_d_n8, assign52790_e87077_d_n9, assign52790_e87077_d_n10, assign52790_e87077_d_n11,) = {
    if ((((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) && (locals.var_guard808 != 0.0)) && (locals.var_guard809 == 0.0)) {
        let assign52790_e87071: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign52790_e87073: f64 = (locals.var_arg).ln();
        let assign52790_e87074: f64 = (-assign52790_e87073);
        let assign52790_e87075: f64 = (assign52790_e87071 * assign52790_e87074);
        (assign52790_e87075, (((locals.var_pbd_t * locals.var_czbd_dn3) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn3 / locals.var_arg)))), ((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn4 / locals.var_arg)))), ((((locals.var_pbd_t_dn5 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn5)) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn5 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn6) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn6 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn7) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn7 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn8) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn8 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn9) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn9 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn10) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn10 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn11) * assign52790_e87074) + (assign52790_e87071 * (-(locals.var_arg_dn11 / locals.var_arg)))),)
    } else {
        (locals.var_qbdj1, locals.var_qbdj1_dn3, locals.var_qbdj1_dn4, locals.var_qbdj1_dn5, locals.var_qbdj1_dn6, locals.var_qbdj1_dn7, locals.var_qbdj1_dn8, locals.var_qbdj1_dn9, locals.var_qbdj1_dn10, locals.var_qbdj1_dn11,)
    }
};
        locals.var_qbdj1 = assign52790_e87077;
        locals.var_qbdj1_dn3 = assign52790_e87077_d_n3;
        locals.var_qbdj1_dn4 = assign52790_e87077_d_n4;
        locals.var_qbdj1_dn5 = assign52790_e87077_d_n5;
        locals.var_qbdj1_dn6 = assign52790_e87077_d_n6;
        locals.var_qbdj1_dn7 = assign52790_e87077_d_n7;
        locals.var_qbdj1_dn8 = assign52790_e87077_d_n8;
        locals.var_qbdj1_dn9 = assign52790_e87077_d_n9;
        locals.var_qbdj1_dn10 = assign52790_e87077_d_n10;
        locals.var_qbdj1_dn11 = assign52790_e87077_d_n11;
        locals.var_qbdj1_rv = 0.0;

        let (assign52800_e87103, assign52800_e87103_d_n3, assign52800_e87103_d_n4, assign52800_e87103_d_n5, assign52800_e87103_d_n6, assign52800_e87103_d_n7, assign52800_e87103_d_n8, assign52800_e87103_d_n9, assign52800_e87103_d_n10, assign52800_e87103_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) && (locals.var_guard808 == 0.0)) {
        let assign52800_e87088: f64 = (locals.var_t1 - 1.0);
        let assign52800_e87089: f64 = (locals.var_czbd_p1 * assign52800_e87088);
        let assign52800_e87092: f64 = (5.0 * p.p914);
        let assign52800_e87095: f64 = (locals.var_t1 - 1.0);
        let assign52800_e87096: f64 = (assign52800_e87092 * assign52800_e87095);
        let assign52800_e87099: f64 = (1.0 + p.p914);
        let assign52800_e87100: f64 = (assign52800_e87096 + assign52800_e87099);
        let assign52800_e87101: f64 = (assign52800_e87089 * assign52800_e87100);
        (assign52800_e87101, (((locals.var_czbd_p1 * locals.var_t1_dn3) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn3))), (((locals.var_czbd_p1 * locals.var_t1_dn4) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn4))), (((locals.var_czbd_p1 * locals.var_t1_dn5) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn5))), (((locals.var_czbd_p1 * locals.var_t1_dn6) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn6))), (((locals.var_czbd_p1 * locals.var_t1_dn7) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn7))), (((locals.var_czbd_p1 * locals.var_t1_dn8) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn8))), (((locals.var_czbd_p1 * locals.var_t1_dn9) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn9))), (((locals.var_czbd_p1 * locals.var_t1_dn10) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn10))), (((locals.var_czbd_p1 * locals.var_t1_dn11) * assign52800_e87100) + (assign52800_e87089 * (assign52800_e87092 * locals.var_t1_dn11))),)
    } else {
        (locals.var_t2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11,)
    }
};
        locals.var_t2 = assign52800_e87103;
        locals.var_t2_dn3 = assign52800_e87103_d_n3;
        locals.var_t2_dn4 = assign52800_e87103_d_n4;
        locals.var_t2_dn5 = assign52800_e87103_d_n5;
        locals.var_t2_dn6 = assign52800_e87103_d_n6;
        locals.var_t2_dn7 = assign52800_e87103_d_n7;
        locals.var_t2_dn8 = assign52800_e87103_d_n8;
        locals.var_t2_dn9 = assign52800_e87103_d_n9;
        locals.var_t2_dn10 = assign52800_e87103_d_n10;
        locals.var_t2_dn11 = assign52800_e87103_d_n11;
        locals.var_t2_rv = 0.0;

        let (assign52810_e87119, assign52810_e87119_d_n3, assign52810_e87119_d_n4, assign52810_e87119_d_n5, assign52810_e87119_d_n6, assign52810_e87119_d_n7, assign52810_e87119_d_n8, assign52810_e87119_d_n9, assign52810_e87119_d_n10, assign52810_e87119_d_n11,) = {
    if (((locals.var_guard492 == 0.0) && (locals.var_guard807 != 0.0)) && (locals.var_guard808 == 0.0)) {
        let assign52810_e87113: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign52810_e87116: f64 = (locals.var_t2 + locals.var_czbd_p2);
        let assign52810_e87117: f64 = (assign52810_e87113 * assign52810_e87116);
        (assign52810_e87117, (((locals.var_pbd_t * locals.var_czbd_dn3) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn3)), ((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn4)), ((((locals.var_pbd_t_dn5 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn5)) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn5)), (((locals.var_pbd_t * locals.var_czbd_dn6) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn6)), (((locals.var_pbd_t * locals.var_czbd_dn7) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn7)), (((locals.var_pbd_t * locals.var_czbd_dn8) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn8)), (((locals.var_pbd_t * locals.var_czbd_dn9) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn9)), (((locals.var_pbd_t * locals.var_czbd_dn10) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn10)), (((locals.var_pbd_t * locals.var_czbd_dn11) * assign52810_e87116) + (assign52810_e87113 * locals.var_t2_dn11)),)
    } else {
        (locals.var_qbdj1, locals.var_qbdj1_dn3, locals.var_qbdj1_dn4, locals.var_qbdj1_dn5, locals.var_qbdj1_dn6, locals.var_qbdj1_dn7, locals.var_qbdj1_dn8, locals.var_qbdj1_dn9, locals.var_qbdj1_dn10, locals.var_qbdj1_dn11,)
    }
};
        locals.var_qbdj1 = assign52810_e87119;
        locals.var_qbdj1_dn3 = assign52810_e87119_d_n3;
        locals.var_qbdj1_dn4 = assign52810_e87119_d_n4;
        locals.var_qbdj1_dn5 = assign52810_e87119_d_n5;
        locals.var_qbdj1_dn6 = assign52810_e87119_d_n6;
        locals.var_qbdj1_dn7 = assign52810_e87119_d_n7;
        locals.var_qbdj1_dn8 = assign52810_e87119_d_n8;
        locals.var_qbdj1_dn9 = assign52810_e87119_d_n9;
        locals.var_qbdj1_dn10 = assign52810_e87119_d_n10;
        locals.var_qbdj1_dn11 = assign52810_e87119_d_n11;
        locals.var_qbdj1_rv = 0.0;

        let (assign52820_e87127, assign52820_e87127_d_n3, assign52820_e87127_d_n4, assign52820_e87127_d_n5, assign52820_e87127_d_n6, assign52820_e87127_d_n7, assign52820_e87127_d_n8, assign52820_e87127_d_n9, assign52820_e87127_d_n10, assign52820_e87127_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard807 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdj1, locals.var_qbdj1_dn3, locals.var_qbdj1_dn4, locals.var_qbdj1_dn5, locals.var_qbdj1_dn6, locals.var_qbdj1_dn7, locals.var_qbdj1_dn8, locals.var_qbdj1_dn9, locals.var_qbdj1_dn10, locals.var_qbdj1_dn11,)
    }
};
        locals.var_qbdj1 = assign52820_e87127;
        locals.var_qbdj1_dn3 = assign52820_e87127_d_n3;
        locals.var_qbdj1_dn4 = assign52820_e87127_d_n4;
        locals.var_qbdj1_dn5 = assign52820_e87127_d_n5;
        locals.var_qbdj1_dn6 = assign52820_e87127_d_n6;
        locals.var_qbdj1_dn7 = assign52820_e87127_d_n7;
        locals.var_qbdj1_dn8 = assign52820_e87127_d_n8;
        locals.var_qbdj1_dn9 = assign52820_e87127_d_n9;
        locals.var_qbdj1_dn10 = assign52820_e87127_d_n10;
        locals.var_qbdj1_dn11 = assign52820_e87127_d_n11;
        locals.var_qbdj1_rv = 0.0;

        let assign52830_e87130: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard811 = assign52830_e87130;
        locals.var_guard811_rv = 0.0;

        let (assign52840_e87139, assign52840_e87139_d_n3, assign52840_e87139_d_n4, assign52840_e87139_d_n5, assign52840_e87139_d_n6, assign52840_e87139_d_n7, assign52840_e87139_d_n8, assign52840_e87139_d_n9, assign52840_e87139_d_n10, assign52840_e87139_d_n11,) = {
    if ((locals.var_guard492 == 0.0) && (locals.var_guard811 != 0.0)) {
        let assign52840_e87137: f64 = (locals.var_vbd_jct / locals.var_pbswd_t);
        (assign52840_e87137, 0.0, (-((locals.var_vbd_jct * locals.var_pbswd_t_dn4) / (locals.var_pbswd_t * locals.var_pbswd_t))), (-((locals.var_vbd_jct * locals.var_pbswd_t_dn5) / (locals.var_pbswd_t * locals.var_pbswd_t))), (locals.var_vbd_jct_dn6 / locals.var_pbswd_t), 0.0, 0.0, 0.0, (locals.var_vbd_jct_dn10 / locals.var_pbswd_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11,)
    }
};
        locals.var_t1 = assign52840_e87139;
        locals.var_t1_dn3 = assign52840_e87139_d_n3;
        locals.var_t1_dn4 = assign52840_e87139_d_n4;
        locals.var_t1_dn5 = assign52840_e87139_d_n5;
        locals.var_t1_dn6 = assign52840_e87139_d_n6;
        locals.var_t1_dn7 = assign52840_e87139_d_n7;
        locals.var_t1_dn8 = assign52840_e87139_d_n8;
        locals.var_t1_dn9 = assign52840_e87139_d_n9;
        locals.var_t1_dn10 = assign52840_e87139_d_n10;
        locals.var_t1_dn11 = assign52840_e87139_d_n11;
        locals.var_t1_rv = 0.0;

        let assign52850_e87142: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard812 = assign52850_e87142;
        locals.var_guard812_rv = 0.0;

    }
}

#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_67(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign22960_e31671, assign22960_e31671_d_n0, assign22960_e31671_d_n2, assign22960_e31671_d_n3, assign22960_e31671_d_n4, assign22960_e31671_d_n5, assign22960_e31671_d_n6, assign22960_e31671_d_n7, assign22960_e31671_d_n8, assign22960_e31671_d_n9, assign22960_e31671_d_n10, assign22960_e31671_d_n11, assign22960_e31671_d_n12, assign22960_e31671_d_n13, assign22960_e31671_d_n14,) = {
    if (((locals.var_guard585 != 0.0) && (locals.var_guard586 == 0.0)) && (locals.var_guard587 != 0.0)) {
        let assign22960_e31667: f64 = (-locals.var_beta0_a);
        let assign22960_e31669: f64 = (assign22960_e31667 / locals.var_diffvds);
        (assign22960_e31669, ((((-locals.var_beta0_a_dn0) * locals.var_diffvds) - (assign22960_e31667 * locals.var_diffvds_dn0)) / (locals.var_diffvds * locals.var_diffvds)), ((((-locals.var_beta0_a_dn2) * locals.var_diffvds) - (assign22960_e31667 * locals.var_diffvds_dn2)) / (locals.var_diffvds * locals.var_diffvds)), ((((-locals.var_beta0_a_dn3) * locals.var_diffvds) - (assign22960_e31667 * locals.var_diffvds_dn3)) / (locals.var_diffvds * locals.var_diffvds)), ((((-locals.var_beta0_a_dn4) * locals.var_diffvds) - (assign22960_e31667 * locals.var_diffvds_dn4)) / (locals.var_diffvds * locals.var_diffvds)), ((((-locals.var_beta0_a_dn5) * locals.var_diffvds) - (assign22960_e31667 * locals.var_diffvds_dn5)) / (locals.var_diffvds * locals.var_diffvds)), ((((-locals.var_beta0_a_dn6) * locals.var_diffvds) - (assign22960_e31667 * locals.var_diffvds_dn6)) / (locals.var_diffvds * locals.var_diffvds)), ((((-locals.var_beta0_a_dn7) * locals.var_diffvds) - (assign22960_e31667 * locals.var_diffvds_dn7)) / (locals.var_diffvds * locals.var_diffvds)), ((((-locals.var_beta0_a_dn8) * locals.var_diffvds) - (assign22960_e31667 * locals.var_diffvds_dn8)) / (locals.var_diffvds * locals.var_diffvds)), ((((-locals.var_beta0_a_dn9) * locals.var_diffvds) - (assign22960_e31667 * locals.var_diffvds_dn9)) / (locals.var_diffvds * locals.var_diffvds)), ((((-locals.var_beta0_a_dn10) * locals.var_diffvds) - (assign22960_e31667 * locals.var_diffvds_dn10)) / (locals.var_diffvds * locals.var_diffvds)), ((((-locals.var_beta0_a_dn11) * locals.var_diffvds) - (assign22960_e31667 * locals.var_diffvds_dn11)) / (locals.var_diffvds * locals.var_diffvds)), ((((-locals.var_beta0_a_dn12) * locals.var_diffvds) - (assign22960_e31667 * locals.var_diffvds_dn12)) / (locals.var_diffvds * locals.var_diffvds)), ((((-locals.var_beta0_a_dn13) * locals.var_diffvds) - (assign22960_e31667 * locals.var_diffvds_dn13)) / (locals.var_diffvds * locals.var_diffvds)), ((((-locals.var_beta0_a_dn14) * locals.var_diffvds) - (assign22960_e31667 * locals.var_diffvds_dn14)) / (locals.var_diffvds * locals.var_diffvds)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign22960_e31671;
        locals.var_t1_dn0 = assign22960_e31671_d_n0;
        locals.var_t1_dn2 = assign22960_e31671_d_n2;
        locals.var_t1_dn3 = assign22960_e31671_d_n3;
        locals.var_t1_dn4 = assign22960_e31671_d_n4;
        locals.var_t1_dn5 = assign22960_e31671_d_n5;
        locals.var_t1_dn6 = assign22960_e31671_d_n6;
        locals.var_t1_dn7 = assign22960_e31671_d_n7;
        locals.var_t1_dn8 = assign22960_e31671_d_n8;
        locals.var_t1_dn9 = assign22960_e31671_d_n9;
        locals.var_t1_dn10 = assign22960_e31671_d_n10;
        locals.var_t1_dn11 = assign22960_e31671_d_n11;
        locals.var_t1_dn12 = assign22960_e31671_d_n12;
        locals.var_t1_dn13 = assign22960_e31671_d_n13;
        locals.var_t1_dn14 = assign22960_e31671_d_n14;
        locals.var_t1_rv = 0.0;

        let assign22990_e31710: f64 = if p.p1094 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard588 = assign22990_e31710;
        locals.var_guard588_rv = 0.0;

        let (assign23000_e31723, assign23000_e31723_d_n0, assign23000_e31723_d_n2, assign23000_e31723_d_n3, assign23000_e31723_d_n4, assign23000_e31723_d_n5, assign23000_e31723_d_n6, assign23000_e31723_d_n7, assign23000_e31723_d_n8, assign23000_e31723_d_n9, assign23000_e31723_d_n10, assign23000_e31723_d_n11, assign23000_e31723_d_n12, assign23000_e31723_d_n13, assign23000_e31723_d_n14,) = {
    if ((locals.var_guard585 == 0.0) && (locals.var_guard588 != 0.0)) {
        let assign23000_e31718: f64 = (locals.var_beta1_i * locals.var_vds);
        let assign23000_e31719: f64 = (1.0 + assign23000_e31718);
        let assign23000_e31721: f64 = (assign23000_e31719 * locals.var_vdssat);
        (assign23000_e31721, (((locals.var_beta1_i_dn0 * locals.var_vds) * locals.var_vdssat) + (assign23000_e31719 * locals.var_vdssat_dn0)), (((locals.var_beta1_i_dn2 * locals.var_vds) * locals.var_vdssat) + (assign23000_e31719 * locals.var_vdssat_dn2)), (((locals.var_beta1_i_dn3 * locals.var_vds) * locals.var_vdssat) + (assign23000_e31719 * locals.var_vdssat_dn3)), (((locals.var_beta1_i_dn4 * locals.var_vds) * locals.var_vdssat) + (assign23000_e31719 * locals.var_vdssat_dn4)), ((((locals.var_beta1_i_dn5 * locals.var_vds) + (locals.var_beta1_i * locals.var_vds_dn5)) * locals.var_vdssat) + (assign23000_e31719 * locals.var_vdssat_dn5)), (((locals.var_beta1_i_dn6 * locals.var_vds) * locals.var_vdssat) + (assign23000_e31719 * locals.var_vdssat_dn6)), ((((locals.var_beta1_i_dn7 * locals.var_vds) + (locals.var_beta1_i * locals.var_vds_dn7)) * locals.var_vdssat) + (assign23000_e31719 * locals.var_vdssat_dn7)), (((locals.var_beta1_i_dn8 * locals.var_vds) * locals.var_vdssat) + (assign23000_e31719 * locals.var_vdssat_dn8)), (((locals.var_beta1_i_dn9 * locals.var_vds) * locals.var_vdssat) + (assign23000_e31719 * locals.var_vdssat_dn9)), (((locals.var_beta1_i_dn10 * locals.var_vds) * locals.var_vdssat) + (assign23000_e31719 * locals.var_vdssat_dn10)), ((((locals.var_beta1_i_dn11 * locals.var_vds) + (locals.var_beta1_i * locals.var_vds_dn11)) * locals.var_vdssat) + (assign23000_e31719 * locals.var_vdssat_dn11)), (((locals.var_beta1_i_dn12 * locals.var_vds) * locals.var_vdssat) + (assign23000_e31719 * locals.var_vdssat_dn12)), (((locals.var_beta1_i_dn13 * locals.var_vds) * locals.var_vdssat) + (assign23000_e31719 * locals.var_vdssat_dn13)), (((locals.var_beta1_i_dn14 * locals.var_vds) * locals.var_vdssat) + (assign23000_e31719 * locals.var_vdssat_dn14)),)
    } else {
        (locals.var_vdssatii, locals.var_vdssatii_dn0, locals.var_vdssatii_dn2, locals.var_vdssatii_dn3, locals.var_vdssatii_dn4, locals.var_vdssatii_dn5, locals.var_vdssatii_dn6, locals.var_vdssatii_dn7, locals.var_vdssatii_dn8, locals.var_vdssatii_dn9, locals.var_vdssatii_dn10, locals.var_vdssatii_dn11, locals.var_vdssatii_dn12, locals.var_vdssatii_dn13, locals.var_vdssatii_dn14,)
    }
};
        locals.var_vdssatii = assign23000_e31723;
        locals.var_vdssatii_dn0 = assign23000_e31723_d_n0;
        locals.var_vdssatii_dn2 = assign23000_e31723_d_n2;
        locals.var_vdssatii_dn3 = assign23000_e31723_d_n3;
        locals.var_vdssatii_dn4 = assign23000_e31723_d_n4;
        locals.var_vdssatii_dn5 = assign23000_e31723_d_n5;
        locals.var_vdssatii_dn6 = assign23000_e31723_d_n6;
        locals.var_vdssatii_dn7 = assign23000_e31723_d_n7;
        locals.var_vdssatii_dn8 = assign23000_e31723_d_n8;
        locals.var_vdssatii_dn9 = assign23000_e31723_d_n9;
        locals.var_vdssatii_dn10 = assign23000_e31723_d_n10;
        locals.var_vdssatii_dn11 = assign23000_e31723_d_n11;
        locals.var_vdssatii_dn12 = assign23000_e31723_d_n12;
        locals.var_vdssatii_dn13 = assign23000_e31723_d_n13;
        locals.var_vdssatii_dn14 = assign23000_e31723_d_n14;
        locals.var_vdssatii_rv = 0.0;

        let (assign23010_e31738, assign23010_e31738_d_n0, assign23010_e31738_d_n2, assign23010_e31738_d_n3, assign23010_e31738_d_n4, assign23010_e31738_d_n5, assign23010_e31738_d_n6, assign23010_e31738_d_n7, assign23010_e31738_d_n8, assign23010_e31738_d_n9, assign23010_e31738_d_n10, assign23010_e31738_d_n11, assign23010_e31738_d_n12, assign23010_e31738_d_n13, assign23010_e31738_d_n14,) = {
    if ((locals.var_guard585 == 0.0) && (locals.var_guard588 != 0.0)) {
        let assign23010_e31730: f64 = (locals.var_vds / locals.var_vdssatii);
        let assign23010_e31732: f64 = (assign23010_e31730 + 1e-6);
        let assign23010_e31735: f64 = (1.0 / locals.var_delta_t);
        let assign23010_e31736: f64 = (assign23010_e31732).powf(assign23010_e31735);
        (assign23010_e31736, if (-(locals.var_delta_t_dn0 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign23010_e31735) as f64).is_finite() && ((assign23010_e31735) as f64).fract() == 0.0 { if assign23010_e31735 == 0.0 { 0.0 } else { (assign23010_e31735 * ((assign23010_e31732).powf(assign23010_e31735 - 1.0) * (-((locals.var_vds * locals.var_vdssatii_dn0) / (locals.var_vdssatii * locals.var_vdssatii))))) } } else { (assign23010_e31736 * (((-(locals.var_delta_t_dn0 / (locals.var_delta_t * locals.var_delta_t))) * (assign23010_e31732).ln()) + (assign23010_e31735 * ((-((locals.var_vds * locals.var_vdssatii_dn0) / (locals.var_vdssatii * locals.var_vdssatii))) / assign23010_e31732)))) }, if (-(locals.var_delta_t_dn2 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign23010_e31735) as f64).is_finite() && ((assign23010_e31735) as f64).fract() == 0.0 { if assign23010_e31735 == 0.0 { 0.0 } else { (assign23010_e31735 * ((assign23010_e31732).powf(assign23010_e31735 - 1.0) * (-((locals.var_vds * locals.var_vdssatii_dn2) / (locals.var_vdssatii * locals.var_vdssatii))))) } } else { (assign23010_e31736 * (((-(locals.var_delta_t_dn2 / (locals.var_delta_t * locals.var_delta_t))) * (assign23010_e31732).ln()) + (assign23010_e31735 * ((-((locals.var_vds * locals.var_vdssatii_dn2) / (locals.var_vdssatii * locals.var_vdssatii))) / assign23010_e31732)))) }, if (-(locals.var_delta_t_dn3 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign23010_e31735) as f64).is_finite() && ((assign23010_e31735) as f64).fract() == 0.0 { if assign23010_e31735 == 0.0 { 0.0 } else { (assign23010_e31735 * ((assign23010_e31732).powf(assign23010_e31735 - 1.0) * (-((locals.var_vds * locals.var_vdssatii_dn3) / (locals.var_vdssatii * locals.var_vdssatii))))) } } else { (assign23010_e31736 * (((-(locals.var_delta_t_dn3 / (locals.var_delta_t * locals.var_delta_t))) * (assign23010_e31732).ln()) + (assign23010_e31735 * ((-((locals.var_vds * locals.var_vdssatii_dn3) / (locals.var_vdssatii * locals.var_vdssatii))) / assign23010_e31732)))) }, if (-(locals.var_delta_t_dn4 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign23010_e31735) as f64).is_finite() && ((assign23010_e31735) as f64).fract() == 0.0 { if assign23010_e31735 == 0.0 { 0.0 } else { (assign23010_e31735 * ((assign23010_e31732).powf(assign23010_e31735 - 1.0) * (-((locals.var_vds * locals.var_vdssatii_dn4) / (locals.var_vdssatii * locals.var_vdssatii))))) } } else { (assign23010_e31736 * (((-(locals.var_delta_t_dn4 / (locals.var_delta_t * locals.var_delta_t))) * (assign23010_e31732).ln()) + (assign23010_e31735 * ((-((locals.var_vds * locals.var_vdssatii_dn4) / (locals.var_vdssatii * locals.var_vdssatii))) / assign23010_e31732)))) }, if (-(locals.var_delta_t_dn5 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign23010_e31735) as f64).is_finite() && ((assign23010_e31735) as f64).fract() == 0.0 { if assign23010_e31735 == 0.0 { 0.0 } else { (assign23010_e31735 * ((assign23010_e31732).powf(assign23010_e31735 - 1.0) * (((locals.var_vds_dn5 * locals.var_vdssatii) - (locals.var_vds * locals.var_vdssatii_dn5)) / (locals.var_vdssatii * locals.var_vdssatii)))) } } else { (assign23010_e31736 * (((-(locals.var_delta_t_dn5 / (locals.var_delta_t * locals.var_delta_t))) * (assign23010_e31732).ln()) + (assign23010_e31735 * ((((locals.var_vds_dn5 * locals.var_vdssatii) - (locals.var_vds * locals.var_vdssatii_dn5)) / (locals.var_vdssatii * locals.var_vdssatii)) / assign23010_e31732)))) }, if (-(locals.var_delta_t_dn6 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign23010_e31735) as f64).is_finite() && ((assign23010_e31735) as f64).fract() == 0.0 { if assign23010_e31735 == 0.0 { 0.0 } else { (assign23010_e31735 * ((assign23010_e31732).powf(assign23010_e31735 - 1.0) * (-((locals.var_vds * locals.var_vdssatii_dn6) / (locals.var_vdssatii * locals.var_vdssatii))))) } } else { (assign23010_e31736 * (((-(locals.var_delta_t_dn6 / (locals.var_delta_t * locals.var_delta_t))) * (assign23010_e31732).ln()) + (assign23010_e31735 * ((-((locals.var_vds * locals.var_vdssatii_dn6) / (locals.var_vdssatii * locals.var_vdssatii))) / assign23010_e31732)))) }, if (-(locals.var_delta_t_dn7 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign23010_e31735) as f64).is_finite() && ((assign23010_e31735) as f64).fract() == 0.0 { if assign23010_e31735 == 0.0 { 0.0 } else { (assign23010_e31735 * ((assign23010_e31732).powf(assign23010_e31735 - 1.0) * (((locals.var_vds_dn7 * locals.var_vdssatii) - (locals.var_vds * locals.var_vdssatii_dn7)) / (locals.var_vdssatii * locals.var_vdssatii)))) } } else { (assign23010_e31736 * (((-(locals.var_delta_t_dn7 / (locals.var_delta_t * locals.var_delta_t))) * (assign23010_e31732).ln()) + (assign23010_e31735 * ((((locals.var_vds_dn7 * locals.var_vdssatii) - (locals.var_vds * locals.var_vdssatii_dn7)) / (locals.var_vdssatii * locals.var_vdssatii)) / assign23010_e31732)))) }, if (-(locals.var_delta_t_dn8 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign23010_e31735) as f64).is_finite() && ((assign23010_e31735) as f64).fract() == 0.0 { if assign23010_e31735 == 0.0 { 0.0 } else { (assign23010_e31735 * ((assign23010_e31732).powf(assign23010_e31735 - 1.0) * (-((locals.var_vds * locals.var_vdssatii_dn8) / (locals.var_vdssatii * locals.var_vdssatii))))) } } else { (assign23010_e31736 * (((-(locals.var_delta_t_dn8 / (locals.var_delta_t * locals.var_delta_t))) * (assign23010_e31732).ln()) + (assign23010_e31735 * ((-((locals.var_vds * locals.var_vdssatii_dn8) / (locals.var_vdssatii * locals.var_vdssatii))) / assign23010_e31732)))) }, if (-(locals.var_delta_t_dn9 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign23010_e31735) as f64).is_finite() && ((assign23010_e31735) as f64).fract() == 0.0 { if assign23010_e31735 == 0.0 { 0.0 } else { (assign23010_e31735 * ((assign23010_e31732).powf(assign23010_e31735 - 1.0) * (-((locals.var_vds * locals.var_vdssatii_dn9) / (locals.var_vdssatii * locals.var_vdssatii))))) } } else { (assign23010_e31736 * (((-(locals.var_delta_t_dn9 / (locals.var_delta_t * locals.var_delta_t))) * (assign23010_e31732).ln()) + (assign23010_e31735 * ((-((locals.var_vds * locals.var_vdssatii_dn9) / (locals.var_vdssatii * locals.var_vdssatii))) / assign23010_e31732)))) }, if (-(locals.var_delta_t_dn10 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign23010_e31735) as f64).is_finite() && ((assign23010_e31735) as f64).fract() == 0.0 { if assign23010_e31735 == 0.0 { 0.0 } else { (assign23010_e31735 * ((assign23010_e31732).powf(assign23010_e31735 - 1.0) * (-((locals.var_vds * locals.var_vdssatii_dn10) / (locals.var_vdssatii * locals.var_vdssatii))))) } } else { (assign23010_e31736 * (((-(locals.var_delta_t_dn10 / (locals.var_delta_t * locals.var_delta_t))) * (assign23010_e31732).ln()) + (assign23010_e31735 * ((-((locals.var_vds * locals.var_vdssatii_dn10) / (locals.var_vdssatii * locals.var_vdssatii))) / assign23010_e31732)))) }, if (-(locals.var_delta_t_dn11 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign23010_e31735) as f64).is_finite() && ((assign23010_e31735) as f64).fract() == 0.0 { if assign23010_e31735 == 0.0 { 0.0 } else { (assign23010_e31735 * ((assign23010_e31732).powf(assign23010_e31735 - 1.0) * (((locals.var_vds_dn11 * locals.var_vdssatii) - (locals.var_vds * locals.var_vdssatii_dn11)) / (locals.var_vdssatii * locals.var_vdssatii)))) } } else { (assign23010_e31736 * (((-(locals.var_delta_t_dn11 / (locals.var_delta_t * locals.var_delta_t))) * (assign23010_e31732).ln()) + (assign23010_e31735 * ((((locals.var_vds_dn11 * locals.var_vdssatii) - (locals.var_vds * locals.var_vdssatii_dn11)) / (locals.var_vdssatii * locals.var_vdssatii)) / assign23010_e31732)))) }, if (-(locals.var_delta_t_dn12 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign23010_e31735) as f64).is_finite() && ((assign23010_e31735) as f64).fract() == 0.0 { if assign23010_e31735 == 0.0 { 0.0 } else { (assign23010_e31735 * ((assign23010_e31732).powf(assign23010_e31735 - 1.0) * (-((locals.var_vds * locals.var_vdssatii_dn12) / (locals.var_vdssatii * locals.var_vdssatii))))) } } else { (assign23010_e31736 * (((-(locals.var_delta_t_dn12 / (locals.var_delta_t * locals.var_delta_t))) * (assign23010_e31732).ln()) + (assign23010_e31735 * ((-((locals.var_vds * locals.var_vdssatii_dn12) / (locals.var_vdssatii * locals.var_vdssatii))) / assign23010_e31732)))) }, if (-(locals.var_delta_t_dn13 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign23010_e31735) as f64).is_finite() && ((assign23010_e31735) as f64).fract() == 0.0 { if assign23010_e31735 == 0.0 { 0.0 } else { (assign23010_e31735 * ((assign23010_e31732).powf(assign23010_e31735 - 1.0) * (-((locals.var_vds * locals.var_vdssatii_dn13) / (locals.var_vdssatii * locals.var_vdssatii))))) } } else { (assign23010_e31736 * (((-(locals.var_delta_t_dn13 / (locals.var_delta_t * locals.var_delta_t))) * (assign23010_e31732).ln()) + (assign23010_e31735 * ((-((locals.var_vds * locals.var_vdssatii_dn13) / (locals.var_vdssatii * locals.var_vdssatii))) / assign23010_e31732)))) }, if (-(locals.var_delta_t_dn14 / (locals.var_delta_t * locals.var_delta_t))) == 0.0 && ((assign23010_e31735) as f64).is_finite() && ((assign23010_e31735) as f64).fract() == 0.0 { if assign23010_e31735 == 0.0 { 0.0 } else { (assign23010_e31735 * ((assign23010_e31732).powf(assign23010_e31735 - 1.0) * (-((locals.var_vds * locals.var_vdssatii_dn14) / (locals.var_vdssatii * locals.var_vdssatii))))) } } else { (assign23010_e31736 * (((-(locals.var_delta_t_dn14 / (locals.var_delta_t * locals.var_delta_t))) * (assign23010_e31732).ln()) + (assign23010_e31735 * ((-((locals.var_vds * locals.var_vdssatii_dn14) / (locals.var_vdssatii * locals.var_vdssatii))) / assign23010_e31732)))) },)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn3, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn12, locals.var_t7_dn13, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign23010_e31738;
        locals.var_t7_dn0 = assign23010_e31738_d_n0;
        locals.var_t7_dn2 = assign23010_e31738_d_n2;
        locals.var_t7_dn3 = assign23010_e31738_d_n3;
        locals.var_t7_dn4 = assign23010_e31738_d_n4;
        locals.var_t7_dn5 = assign23010_e31738_d_n5;
        locals.var_t7_dn6 = assign23010_e31738_d_n6;
        locals.var_t7_dn7 = assign23010_e31738_d_n7;
        locals.var_t7_dn8 = assign23010_e31738_d_n8;
        locals.var_t7_dn9 = assign23010_e31738_d_n9;
        locals.var_t7_dn10 = assign23010_e31738_d_n10;
        locals.var_t7_dn11 = assign23010_e31738_d_n11;
        locals.var_t7_dn12 = assign23010_e31738_d_n12;
        locals.var_t7_dn13 = assign23010_e31738_d_n13;
        locals.var_t7_dn14 = assign23010_e31738_d_n14;
        locals.var_t7_rv = 0.0;

        let (assign23020_e31750, assign23020_e31750_d_n0, assign23020_e31750_d_n2, assign23020_e31750_d_n3, assign23020_e31750_d_n4, assign23020_e31750_d_n5, assign23020_e31750_d_n6, assign23020_e31750_d_n7, assign23020_e31750_d_n8, assign23020_e31750_d_n9, assign23020_e31750_d_n10, assign23020_e31750_d_n11, assign23020_e31750_d_n12, assign23020_e31750_d_n13, assign23020_e31750_d_n14,) = {
    if ((locals.var_guard585 == 0.0) && (locals.var_guard588 != 0.0)) {
        let assign23020_e31745: f64 = (1.0 + locals.var_t7);
        let assign23020_e31747: f64 = (-locals.var_delta_t);
        let assign23020_e31748: f64 = (assign23020_e31745).powf(assign23020_e31747);
        (assign23020_e31748, if (-locals.var_delta_t_dn0) == 0.0 && ((assign23020_e31747) as f64).is_finite() && ((assign23020_e31747) as f64).fract() == 0.0 { if assign23020_e31747 == 0.0 { 0.0 } else { (assign23020_e31747 * ((assign23020_e31745).powf(assign23020_e31747 - 1.0) * locals.var_t7_dn0)) } } else { (assign23020_e31748 * (((-locals.var_delta_t_dn0) * (assign23020_e31745).ln()) + (assign23020_e31747 * (locals.var_t7_dn0 / assign23020_e31745)))) }, if (-locals.var_delta_t_dn2) == 0.0 && ((assign23020_e31747) as f64).is_finite() && ((assign23020_e31747) as f64).fract() == 0.0 { if assign23020_e31747 == 0.0 { 0.0 } else { (assign23020_e31747 * ((assign23020_e31745).powf(assign23020_e31747 - 1.0) * locals.var_t7_dn2)) } } else { (assign23020_e31748 * (((-locals.var_delta_t_dn2) * (assign23020_e31745).ln()) + (assign23020_e31747 * (locals.var_t7_dn2 / assign23020_e31745)))) }, if (-locals.var_delta_t_dn3) == 0.0 && ((assign23020_e31747) as f64).is_finite() && ((assign23020_e31747) as f64).fract() == 0.0 { if assign23020_e31747 == 0.0 { 0.0 } else { (assign23020_e31747 * ((assign23020_e31745).powf(assign23020_e31747 - 1.0) * locals.var_t7_dn3)) } } else { (assign23020_e31748 * (((-locals.var_delta_t_dn3) * (assign23020_e31745).ln()) + (assign23020_e31747 * (locals.var_t7_dn3 / assign23020_e31745)))) }, if (-locals.var_delta_t_dn4) == 0.0 && ((assign23020_e31747) as f64).is_finite() && ((assign23020_e31747) as f64).fract() == 0.0 { if assign23020_e31747 == 0.0 { 0.0 } else { (assign23020_e31747 * ((assign23020_e31745).powf(assign23020_e31747 - 1.0) * locals.var_t7_dn4)) } } else { (assign23020_e31748 * (((-locals.var_delta_t_dn4) * (assign23020_e31745).ln()) + (assign23020_e31747 * (locals.var_t7_dn4 / assign23020_e31745)))) }, if (-locals.var_delta_t_dn5) == 0.0 && ((assign23020_e31747) as f64).is_finite() && ((assign23020_e31747) as f64).fract() == 0.0 { if assign23020_e31747 == 0.0 { 0.0 } else { (assign23020_e31747 * ((assign23020_e31745).powf(assign23020_e31747 - 1.0) * locals.var_t7_dn5)) } } else { (assign23020_e31748 * (((-locals.var_delta_t_dn5) * (assign23020_e31745).ln()) + (assign23020_e31747 * (locals.var_t7_dn5 / assign23020_e31745)))) }, if (-locals.var_delta_t_dn6) == 0.0 && ((assign23020_e31747) as f64).is_finite() && ((assign23020_e31747) as f64).fract() == 0.0 { if assign23020_e31747 == 0.0 { 0.0 } else { (assign23020_e31747 * ((assign23020_e31745).powf(assign23020_e31747 - 1.0) * locals.var_t7_dn6)) } } else { (assign23020_e31748 * (((-locals.var_delta_t_dn6) * (assign23020_e31745).ln()) + (assign23020_e31747 * (locals.var_t7_dn6 / assign23020_e31745)))) }, if (-locals.var_delta_t_dn7) == 0.0 && ((assign23020_e31747) as f64).is_finite() && ((assign23020_e31747) as f64).fract() == 0.0 { if assign23020_e31747 == 0.0 { 0.0 } else { (assign23020_e31747 * ((assign23020_e31745).powf(assign23020_e31747 - 1.0) * locals.var_t7_dn7)) } } else { (assign23020_e31748 * (((-locals.var_delta_t_dn7) * (assign23020_e31745).ln()) + (assign23020_e31747 * (locals.var_t7_dn7 / assign23020_e31745)))) }, if (-locals.var_delta_t_dn8) == 0.0 && ((assign23020_e31747) as f64).is_finite() && ((assign23020_e31747) as f64).fract() == 0.0 { if assign23020_e31747 == 0.0 { 0.0 } else { (assign23020_e31747 * ((assign23020_e31745).powf(assign23020_e31747 - 1.0) * locals.var_t7_dn8)) } } else { (assign23020_e31748 * (((-locals.var_delta_t_dn8) * (assign23020_e31745).ln()) + (assign23020_e31747 * (locals.var_t7_dn8 / assign23020_e31745)))) }, if (-locals.var_delta_t_dn9) == 0.0 && ((assign23020_e31747) as f64).is_finite() && ((assign23020_e31747) as f64).fract() == 0.0 { if assign23020_e31747 == 0.0 { 0.0 } else { (assign23020_e31747 * ((assign23020_e31745).powf(assign23020_e31747 - 1.0) * locals.var_t7_dn9)) } } else { (assign23020_e31748 * (((-locals.var_delta_t_dn9) * (assign23020_e31745).ln()) + (assign23020_e31747 * (locals.var_t7_dn9 / assign23020_e31745)))) }, if (-locals.var_delta_t_dn10) == 0.0 && ((assign23020_e31747) as f64).is_finite() && ((assign23020_e31747) as f64).fract() == 0.0 { if assign23020_e31747 == 0.0 { 0.0 } else { (assign23020_e31747 * ((assign23020_e31745).powf(assign23020_e31747 - 1.0) * locals.var_t7_dn10)) } } else { (assign23020_e31748 * (((-locals.var_delta_t_dn10) * (assign23020_e31745).ln()) + (assign23020_e31747 * (locals.var_t7_dn10 / assign23020_e31745)))) }, if (-locals.var_delta_t_dn11) == 0.0 && ((assign23020_e31747) as f64).is_finite() && ((assign23020_e31747) as f64).fract() == 0.0 { if assign23020_e31747 == 0.0 { 0.0 } else { (assign23020_e31747 * ((assign23020_e31745).powf(assign23020_e31747 - 1.0) * locals.var_t7_dn11)) } } else { (assign23020_e31748 * (((-locals.var_delta_t_dn11) * (assign23020_e31745).ln()) + (assign23020_e31747 * (locals.var_t7_dn11 / assign23020_e31745)))) }, if (-locals.var_delta_t_dn12) == 0.0 && ((assign23020_e31747) as f64).is_finite() && ((assign23020_e31747) as f64).fract() == 0.0 { if assign23020_e31747 == 0.0 { 0.0 } else { (assign23020_e31747 * ((assign23020_e31745).powf(assign23020_e31747 - 1.0) * locals.var_t7_dn12)) } } else { (assign23020_e31748 * (((-locals.var_delta_t_dn12) * (assign23020_e31745).ln()) + (assign23020_e31747 * (locals.var_t7_dn12 / assign23020_e31745)))) }, if (-locals.var_delta_t_dn13) == 0.0 && ((assign23020_e31747) as f64).is_finite() && ((assign23020_e31747) as f64).fract() == 0.0 { if assign23020_e31747 == 0.0 { 0.0 } else { (assign23020_e31747 * ((assign23020_e31745).powf(assign23020_e31747 - 1.0) * locals.var_t7_dn13)) } } else { (assign23020_e31748 * (((-locals.var_delta_t_dn13) * (assign23020_e31745).ln()) + (assign23020_e31747 * (locals.var_t7_dn13 / assign23020_e31745)))) }, if (-locals.var_delta_t_dn14) == 0.0 && ((assign23020_e31747) as f64).is_finite() && ((assign23020_e31747) as f64).fract() == 0.0 { if assign23020_e31747 == 0.0 { 0.0 } else { (assign23020_e31747 * ((assign23020_e31745).powf(assign23020_e31747 - 1.0) * locals.var_t7_dn14)) } } else { (assign23020_e31748 * (((-locals.var_delta_t_dn14) * (assign23020_e31745).ln()) + (assign23020_e31747 * (locals.var_t7_dn14 / assign23020_e31745)))) },)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn3, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn12, locals.var_t8_dn13, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign23020_e31750;
        locals.var_t8_dn0 = assign23020_e31750_d_n0;
        locals.var_t8_dn2 = assign23020_e31750_d_n2;
        locals.var_t8_dn3 = assign23020_e31750_d_n3;
        locals.var_t8_dn4 = assign23020_e31750_d_n4;
        locals.var_t8_dn5 = assign23020_e31750_d_n5;
        locals.var_t8_dn6 = assign23020_e31750_d_n6;
        locals.var_t8_dn7 = assign23020_e31750_d_n7;
        locals.var_t8_dn8 = assign23020_e31750_d_n8;
        locals.var_t8_dn9 = assign23020_e31750_d_n9;
        locals.var_t8_dn10 = assign23020_e31750_d_n10;
        locals.var_t8_dn11 = assign23020_e31750_d_n11;
        locals.var_t8_dn12 = assign23020_e31750_d_n12;
        locals.var_t8_dn13 = assign23020_e31750_d_n13;
        locals.var_t8_dn14 = assign23020_e31750_d_n14;
        locals.var_t8_rv = 0.0;

        let (assign23030_e31759, assign23030_e31759_d_n0, assign23030_e31759_d_n2, assign23030_e31759_d_n3, assign23030_e31759_d_n4, assign23030_e31759_d_n5, assign23030_e31759_d_n6, assign23030_e31759_d_n7, assign23030_e31759_d_n8, assign23030_e31759_d_n9, assign23030_e31759_d_n10, assign23030_e31759_d_n11, assign23030_e31759_d_n12, assign23030_e31759_d_n13, assign23030_e31759_d_n14,) = {
    if ((locals.var_guard585 == 0.0) && (locals.var_guard588 != 0.0)) {
        let assign23030_e31757: f64 = (locals.var_vds * locals.var_t8);
        (assign23030_e31757, (locals.var_vds * locals.var_t8_dn0), (locals.var_vds * locals.var_t8_dn2), (locals.var_vds * locals.var_t8_dn3), (locals.var_vds * locals.var_t8_dn4), ((locals.var_vds_dn5 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn5)), (locals.var_vds * locals.var_t8_dn6), ((locals.var_vds_dn7 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn7)), (locals.var_vds * locals.var_t8_dn8), (locals.var_vds * locals.var_t8_dn9), (locals.var_vds * locals.var_t8_dn10), ((locals.var_vds_dn11 * locals.var_t8) + (locals.var_vds * locals.var_t8_dn11)), (locals.var_vds * locals.var_t8_dn12), (locals.var_vds * locals.var_t8_dn13), (locals.var_vds * locals.var_t8_dn14),)
    } else {
        (locals.var_vdseffii, locals.var_vdseffii_dn0, locals.var_vdseffii_dn2, locals.var_vdseffii_dn3, locals.var_vdseffii_dn4, locals.var_vdseffii_dn5, locals.var_vdseffii_dn6, locals.var_vdseffii_dn7, locals.var_vdseffii_dn8, locals.var_vdseffii_dn9, locals.var_vdseffii_dn10, locals.var_vdseffii_dn11, locals.var_vdseffii_dn12, locals.var_vdseffii_dn13, locals.var_vdseffii_dn14,)
    }
};
        locals.var_vdseffii = assign23030_e31759;
        locals.var_vdseffii_dn0 = assign23030_e31759_d_n0;
        locals.var_vdseffii_dn2 = assign23030_e31759_d_n2;
        locals.var_vdseffii_dn3 = assign23030_e31759_d_n3;
        locals.var_vdseffii_dn4 = assign23030_e31759_d_n4;
        locals.var_vdseffii_dn5 = assign23030_e31759_d_n5;
        locals.var_vdseffii_dn6 = assign23030_e31759_d_n6;
        locals.var_vdseffii_dn7 = assign23030_e31759_d_n7;
        locals.var_vdseffii_dn8 = assign23030_e31759_d_n8;
        locals.var_vdseffii_dn9 = assign23030_e31759_d_n9;
        locals.var_vdseffii_dn10 = assign23030_e31759_d_n10;
        locals.var_vdseffii_dn11 = assign23030_e31759_d_n11;
        locals.var_vdseffii_dn12 = assign23030_e31759_d_n12;
        locals.var_vdseffii_dn13 = assign23030_e31759_d_n13;
        locals.var_vdseffii_dn14 = assign23030_e31759_d_n14;
        locals.var_vdseffii_rv = 0.0;

        let (assign23040_e31768, assign23040_e31768_d_n0, assign23040_e31768_d_n2, assign23040_e31768_d_n3, assign23040_e31768_d_n4, assign23040_e31768_d_n5, assign23040_e31768_d_n6, assign23040_e31768_d_n7, assign23040_e31768_d_n8, assign23040_e31768_d_n9, assign23040_e31768_d_n10, assign23040_e31768_d_n11, assign23040_e31768_d_n12, assign23040_e31768_d_n13, assign23040_e31768_d_n14,) = {
    if ((locals.var_guard585 == 0.0) && (locals.var_guard588 != 0.0)) {
        let assign23040_e31766: f64 = (locals.var_vds - locals.var_vdseffii);
        (assign23040_e31766, (-locals.var_vdseffii_dn0), (-locals.var_vdseffii_dn2), (-locals.var_vdseffii_dn3), (-locals.var_vdseffii_dn4), (locals.var_vds_dn5 - locals.var_vdseffii_dn5), (-locals.var_vdseffii_dn6), (locals.var_vds_dn7 - locals.var_vdseffii_dn7), (-locals.var_vdseffii_dn8), (-locals.var_vdseffii_dn9), (-locals.var_vdseffii_dn10), (locals.var_vds_dn11 - locals.var_vdseffii_dn11), (-locals.var_vdseffii_dn12), (-locals.var_vdseffii_dn13), (-locals.var_vdseffii_dn14),)
    } else {
        (locals.var_diffvdsii, locals.var_diffvdsii_dn0, locals.var_diffvdsii_dn2, locals.var_diffvdsii_dn3, locals.var_diffvdsii_dn4, locals.var_diffvdsii_dn5, locals.var_diffvdsii_dn6, locals.var_diffvdsii_dn7, locals.var_diffvdsii_dn8, locals.var_diffvdsii_dn9, locals.var_diffvdsii_dn10, locals.var_diffvdsii_dn11, locals.var_diffvdsii_dn12, locals.var_diffvdsii_dn13, locals.var_diffvdsii_dn14,)
    }
};
        locals.var_diffvdsii = assign23040_e31768;
        locals.var_diffvdsii_dn0 = assign23040_e31768_d_n0;
        locals.var_diffvdsii_dn2 = assign23040_e31768_d_n2;
        locals.var_diffvdsii_dn3 = assign23040_e31768_d_n3;
        locals.var_diffvdsii_dn4 = assign23040_e31768_d_n4;
        locals.var_diffvdsii_dn5 = assign23040_e31768_d_n5;
        locals.var_diffvdsii_dn6 = assign23040_e31768_d_n6;
        locals.var_diffvdsii_dn7 = assign23040_e31768_d_n7;
        locals.var_diffvdsii_dn8 = assign23040_e31768_d_n8;
        locals.var_diffvdsii_dn9 = assign23040_e31768_d_n9;
        locals.var_diffvdsii_dn10 = assign23040_e31768_d_n10;
        locals.var_diffvdsii_dn11 = assign23040_e31768_d_n11;
        locals.var_diffvdsii_dn12 = assign23040_e31768_d_n12;
        locals.var_diffvdsii_dn13 = assign23040_e31768_d_n13;
        locals.var_diffvdsii_dn14 = assign23040_e31768_d_n14;
        locals.var_diffvdsii_rv = 0.0;

        let assign23050_e31774: f64 = (-2500.0);
        let assign23050_e31776: f64 = (assign23050_e31774 * 0.001);
        let assign23050_e31778: f64 = if ((0.0 == 0.0) && (locals.var_diffvdsii < assign23050_e31776)) { 1.0 } else { 0.0 };
        locals.var_guard589 = assign23050_e31778;
        locals.var_guard589_rv = 0.0;

        let (assign23060_e31794, assign23060_e31794_d_n0, assign23060_e31794_d_n2, assign23060_e31794_d_n3, assign23060_e31794_d_n4, assign23060_e31794_d_n5, assign23060_e31794_d_n6, assign23060_e31794_d_n7, assign23060_e31794_d_n8, assign23060_e31794_d_n9, assign23060_e31794_d_n10, assign23060_e31794_d_n11, assign23060_e31794_d_n12, assign23060_e31794_d_n13, assign23060_e31794_d_n14,) = {
    if (((locals.var_guard585 == 0.0) && (locals.var_guard588 != 0.0)) && (locals.var_guard589 != 0.0)) {
        let assign23060_e31786: f64 = (-0.001);
        let assign23060_e31788: f64 = (assign23060_e31786 * 0.001);
        let assign23060_e31791: f64 = (16.0 * locals.var_diffvdsii);
        let assign23060_e31792: f64 = (assign23060_e31788 / assign23060_e31791);
        (assign23060_e31792, (-((assign23060_e31788 * (16.0 * locals.var_diffvdsii_dn0)) / (assign23060_e31791 * assign23060_e31791))), (-((assign23060_e31788 * (16.0 * locals.var_diffvdsii_dn2)) / (assign23060_e31791 * assign23060_e31791))), (-((assign23060_e31788 * (16.0 * locals.var_diffvdsii_dn3)) / (assign23060_e31791 * assign23060_e31791))), (-((assign23060_e31788 * (16.0 * locals.var_diffvdsii_dn4)) / (assign23060_e31791 * assign23060_e31791))), (-((assign23060_e31788 * (16.0 * locals.var_diffvdsii_dn5)) / (assign23060_e31791 * assign23060_e31791))), (-((assign23060_e31788 * (16.0 * locals.var_diffvdsii_dn6)) / (assign23060_e31791 * assign23060_e31791))), (-((assign23060_e31788 * (16.0 * locals.var_diffvdsii_dn7)) / (assign23060_e31791 * assign23060_e31791))), (-((assign23060_e31788 * (16.0 * locals.var_diffvdsii_dn8)) / (assign23060_e31791 * assign23060_e31791))), (-((assign23060_e31788 * (16.0 * locals.var_diffvdsii_dn9)) / (assign23060_e31791 * assign23060_e31791))), (-((assign23060_e31788 * (16.0 * locals.var_diffvdsii_dn10)) / (assign23060_e31791 * assign23060_e31791))), (-((assign23060_e31788 * (16.0 * locals.var_diffvdsii_dn11)) / (assign23060_e31791 * assign23060_e31791))), (-((assign23060_e31788 * (16.0 * locals.var_diffvdsii_dn12)) / (assign23060_e31791 * assign23060_e31791))), (-((assign23060_e31788 * (16.0 * locals.var_diffvdsii_dn13)) / (assign23060_e31791 * assign23060_e31791))), (-((assign23060_e31788 * (16.0 * locals.var_diffvdsii_dn14)) / (assign23060_e31791 * assign23060_e31791))),)
    } else {
        (locals.var_diffvdsii, locals.var_diffvdsii_dn0, locals.var_diffvdsii_dn2, locals.var_diffvdsii_dn3, locals.var_diffvdsii_dn4, locals.var_diffvdsii_dn5, locals.var_diffvdsii_dn6, locals.var_diffvdsii_dn7, locals.var_diffvdsii_dn8, locals.var_diffvdsii_dn9, locals.var_diffvdsii_dn10, locals.var_diffvdsii_dn11, locals.var_diffvdsii_dn12, locals.var_diffvdsii_dn13, locals.var_diffvdsii_dn14,)
    }
};
        locals.var_diffvdsii = assign23060_e31794;
        locals.var_diffvdsii_dn0 = assign23060_e31794_d_n0;
        locals.var_diffvdsii_dn2 = assign23060_e31794_d_n2;
        locals.var_diffvdsii_dn3 = assign23060_e31794_d_n3;
        locals.var_diffvdsii_dn4 = assign23060_e31794_d_n4;
        locals.var_diffvdsii_dn5 = assign23060_e31794_d_n5;
        locals.var_diffvdsii_dn6 = assign23060_e31794_d_n6;
        locals.var_diffvdsii_dn7 = assign23060_e31794_d_n7;
        locals.var_diffvdsii_dn8 = assign23060_e31794_d_n8;
        locals.var_diffvdsii_dn9 = assign23060_e31794_d_n9;
        locals.var_diffvdsii_dn10 = assign23060_e31794_d_n10;
        locals.var_diffvdsii_dn11 = assign23060_e31794_d_n11;
        locals.var_diffvdsii_dn12 = assign23060_e31794_d_n12;
        locals.var_diffvdsii_dn13 = assign23060_e31794_d_n13;
        locals.var_diffvdsii_dn14 = assign23060_e31794_d_n14;
        locals.var_diffvdsii_rv = 0.0;

        let (assign23070_e31823, assign23070_e31823_d_n0, assign23070_e31823_d_n2, assign23070_e31823_d_n3, assign23070_e31823_d_n4, assign23070_e31823_d_n5, assign23070_e31823_d_n6, assign23070_e31823_d_n7, assign23070_e31823_d_n8, assign23070_e31823_d_n9, assign23070_e31823_d_n10, assign23070_e31823_d_n11, assign23070_e31823_d_n12, assign23070_e31823_d_n13, assign23070_e31823_d_n14,) = {
    if (((locals.var_guard585 == 0.0) && (locals.var_guard588 != 0.0)) && (locals.var_guard589 == 0.0)) {
        let assign23070_e31805: f64 = locals.var_diffvdsii;
        let assign23070_e31808: f64 = locals.var_diffvdsii;
        let assign23070_e31811: f64 = locals.var_diffvdsii;
        let assign23070_e31812: f64 = (assign23070_e31808 * assign23070_e31811);
        let assign23070_e31815: f64 = (0.25 * 0.001);
        let assign23070_e31817: f64 = (assign23070_e31815 * 0.001);
        let assign23070_e31818: f64 = (assign23070_e31812 + assign23070_e31817);
        let assign23070_e31819: f64 = (assign23070_e31818).sqrt();
        let assign23070_e31820: f64 = (assign23070_e31805 + assign23070_e31819);
        let assign23070_e31821: f64 = (0.5 * assign23070_e31820);
        (assign23070_e31821, (0.5 * (locals.var_diffvdsii_dn0 + (((locals.var_diffvdsii_dn0 * assign23070_e31811) + (assign23070_e31808 * locals.var_diffvdsii_dn0)) / (2.0 * assign23070_e31819)))), (0.5 * (locals.var_diffvdsii_dn2 + (((locals.var_diffvdsii_dn2 * assign23070_e31811) + (assign23070_e31808 * locals.var_diffvdsii_dn2)) / (2.0 * assign23070_e31819)))), (0.5 * (locals.var_diffvdsii_dn3 + (((locals.var_diffvdsii_dn3 * assign23070_e31811) + (assign23070_e31808 * locals.var_diffvdsii_dn3)) / (2.0 * assign23070_e31819)))), (0.5 * (locals.var_diffvdsii_dn4 + (((locals.var_diffvdsii_dn4 * assign23070_e31811) + (assign23070_e31808 * locals.var_diffvdsii_dn4)) / (2.0 * assign23070_e31819)))), (0.5 * (locals.var_diffvdsii_dn5 + (((locals.var_diffvdsii_dn5 * assign23070_e31811) + (assign23070_e31808 * locals.var_diffvdsii_dn5)) / (2.0 * assign23070_e31819)))), (0.5 * (locals.var_diffvdsii_dn6 + (((locals.var_diffvdsii_dn6 * assign23070_e31811) + (assign23070_e31808 * locals.var_diffvdsii_dn6)) / (2.0 * assign23070_e31819)))), (0.5 * (locals.var_diffvdsii_dn7 + (((locals.var_diffvdsii_dn7 * assign23070_e31811) + (assign23070_e31808 * locals.var_diffvdsii_dn7)) / (2.0 * assign23070_e31819)))), (0.5 * (locals.var_diffvdsii_dn8 + (((locals.var_diffvdsii_dn8 * assign23070_e31811) + (assign23070_e31808 * locals.var_diffvdsii_dn8)) / (2.0 * assign23070_e31819)))), (0.5 * (locals.var_diffvdsii_dn9 + (((locals.var_diffvdsii_dn9 * assign23070_e31811) + (assign23070_e31808 * locals.var_diffvdsii_dn9)) / (2.0 * assign23070_e31819)))), (0.5 * (locals.var_diffvdsii_dn10 + (((locals.var_diffvdsii_dn10 * assign23070_e31811) + (assign23070_e31808 * locals.var_diffvdsii_dn10)) / (2.0 * assign23070_e31819)))), (0.5 * (locals.var_diffvdsii_dn11 + (((locals.var_diffvdsii_dn11 * assign23070_e31811) + (assign23070_e31808 * locals.var_diffvdsii_dn11)) / (2.0 * assign23070_e31819)))), (0.5 * (locals.var_diffvdsii_dn12 + (((locals.var_diffvdsii_dn12 * assign23070_e31811) + (assign23070_e31808 * locals.var_diffvdsii_dn12)) / (2.0 * assign23070_e31819)))), (0.5 * (locals.var_diffvdsii_dn13 + (((locals.var_diffvdsii_dn13 * assign23070_e31811) + (assign23070_e31808 * locals.var_diffvdsii_dn13)) / (2.0 * assign23070_e31819)))), (0.5 * (locals.var_diffvdsii_dn14 + (((locals.var_diffvdsii_dn14 * assign23070_e31811) + (assign23070_e31808 * locals.var_diffvdsii_dn14)) / (2.0 * assign23070_e31819)))),)
    } else {
        (locals.var_diffvdsii, locals.var_diffvdsii_dn0, locals.var_diffvdsii_dn2, locals.var_diffvdsii_dn3, locals.var_diffvdsii_dn4, locals.var_diffvdsii_dn5, locals.var_diffvdsii_dn6, locals.var_diffvdsii_dn7, locals.var_diffvdsii_dn8, locals.var_diffvdsii_dn9, locals.var_diffvdsii_dn10, locals.var_diffvdsii_dn11, locals.var_diffvdsii_dn12, locals.var_diffvdsii_dn13, locals.var_diffvdsii_dn14,)
    }
};
        locals.var_diffvdsii = assign23070_e31823;
        locals.var_diffvdsii_dn0 = assign23070_e31823_d_n0;
        locals.var_diffvdsii_dn2 = assign23070_e31823_d_n2;
        locals.var_diffvdsii_dn3 = assign23070_e31823_d_n3;
        locals.var_diffvdsii_dn4 = assign23070_e31823_d_n4;
        locals.var_diffvdsii_dn5 = assign23070_e31823_d_n5;
        locals.var_diffvdsii_dn6 = assign23070_e31823_d_n6;
        locals.var_diffvdsii_dn7 = assign23070_e31823_d_n7;
        locals.var_diffvdsii_dn8 = assign23070_e31823_d_n8;
        locals.var_diffvdsii_dn9 = assign23070_e31823_d_n9;
        locals.var_diffvdsii_dn10 = assign23070_e31823_d_n10;
        locals.var_diffvdsii_dn11 = assign23070_e31823_d_n11;
        locals.var_diffvdsii_dn12 = assign23070_e31823_d_n12;
        locals.var_diffvdsii_dn13 = assign23070_e31823_d_n13;
        locals.var_diffvdsii_dn14 = assign23070_e31823_d_n14;
        locals.var_diffvdsii_rv = 0.0;

        let (assign23080_e31838, assign23080_e31838_d_n0, assign23080_e31838_d_n2, assign23080_e31838_d_n3, assign23080_e31838_d_n4, assign23080_e31838_d_n5, assign23080_e31838_d_n6, assign23080_e31838_d_n7, assign23080_e31838_d_n8, assign23080_e31838_d_n9, assign23080_e31838_d_n10, assign23080_e31838_d_n11, assign23080_e31838_d_n12, assign23080_e31838_d_n13, assign23080_e31838_d_n14,) = {
    if ((locals.var_guard585 == 0.0) && (locals.var_guard588 != 0.0)) {
        let assign23080_e31830: f64 = (0.5 * locals.var_beta0_a);
        let assign23080_e31834: f64 = (locals.var_vdseffii).powf(locals.var_beta2_i);
        let assign23080_e31835: f64 = (1.0 + assign23080_e31834);
        let assign23080_e31836: f64 = (assign23080_e31830 * assign23080_e31835);
        (assign23080_e31836, (((0.5 * locals.var_beta0_a_dn0) * assign23080_e31835) + (assign23080_e31830 * if locals.var_beta2_i_dn0 == 0.0 && ((locals.var_beta2_i) as f64).is_finite() && ((locals.var_beta2_i) as f64).fract() == 0.0 { if locals.var_beta2_i == 0.0 { 0.0 } else { (locals.var_beta2_i * ((locals.var_vdseffii).powf(locals.var_beta2_i - 1.0) * locals.var_vdseffii_dn0)) } } else { (assign23080_e31834 * ((locals.var_beta2_i_dn0 * (locals.var_vdseffii).ln()) + (locals.var_beta2_i * (locals.var_vdseffii_dn0 / locals.var_vdseffii)))) })), (((0.5 * locals.var_beta0_a_dn2) * assign23080_e31835) + (assign23080_e31830 * if locals.var_beta2_i_dn2 == 0.0 && ((locals.var_beta2_i) as f64).is_finite() && ((locals.var_beta2_i) as f64).fract() == 0.0 { if locals.var_beta2_i == 0.0 { 0.0 } else { (locals.var_beta2_i * ((locals.var_vdseffii).powf(locals.var_beta2_i - 1.0) * locals.var_vdseffii_dn2)) } } else { (assign23080_e31834 * ((locals.var_beta2_i_dn2 * (locals.var_vdseffii).ln()) + (locals.var_beta2_i * (locals.var_vdseffii_dn2 / locals.var_vdseffii)))) })), (((0.5 * locals.var_beta0_a_dn3) * assign23080_e31835) + (assign23080_e31830 * if locals.var_beta2_i_dn3 == 0.0 && ((locals.var_beta2_i) as f64).is_finite() && ((locals.var_beta2_i) as f64).fract() == 0.0 { if locals.var_beta2_i == 0.0 { 0.0 } else { (locals.var_beta2_i * ((locals.var_vdseffii).powf(locals.var_beta2_i - 1.0) * locals.var_vdseffii_dn3)) } } else { (assign23080_e31834 * ((locals.var_beta2_i_dn3 * (locals.var_vdseffii).ln()) + (locals.var_beta2_i * (locals.var_vdseffii_dn3 / locals.var_vdseffii)))) })), (((0.5 * locals.var_beta0_a_dn4) * assign23080_e31835) + (assign23080_e31830 * if locals.var_beta2_i_dn4 == 0.0 && ((locals.var_beta2_i) as f64).is_finite() && ((locals.var_beta2_i) as f64).fract() == 0.0 { if locals.var_beta2_i == 0.0 { 0.0 } else { (locals.var_beta2_i * ((locals.var_vdseffii).powf(locals.var_beta2_i - 1.0) * locals.var_vdseffii_dn4)) } } else { (assign23080_e31834 * ((locals.var_beta2_i_dn4 * (locals.var_vdseffii).ln()) + (locals.var_beta2_i * (locals.var_vdseffii_dn4 / locals.var_vdseffii)))) })), (((0.5 * locals.var_beta0_a_dn5) * assign23080_e31835) + (assign23080_e31830 * if locals.var_beta2_i_dn5 == 0.0 && ((locals.var_beta2_i) as f64).is_finite() && ((locals.var_beta2_i) as f64).fract() == 0.0 { if locals.var_beta2_i == 0.0 { 0.0 } else { (locals.var_beta2_i * ((locals.var_vdseffii).powf(locals.var_beta2_i - 1.0) * locals.var_vdseffii_dn5)) } } else { (assign23080_e31834 * ((locals.var_beta2_i_dn5 * (locals.var_vdseffii).ln()) + (locals.var_beta2_i * (locals.var_vdseffii_dn5 / locals.var_vdseffii)))) })), (((0.5 * locals.var_beta0_a_dn6) * assign23080_e31835) + (assign23080_e31830 * if locals.var_beta2_i_dn6 == 0.0 && ((locals.var_beta2_i) as f64).is_finite() && ((locals.var_beta2_i) as f64).fract() == 0.0 { if locals.var_beta2_i == 0.0 { 0.0 } else { (locals.var_beta2_i * ((locals.var_vdseffii).powf(locals.var_beta2_i - 1.0) * locals.var_vdseffii_dn6)) } } else { (assign23080_e31834 * ((locals.var_beta2_i_dn6 * (locals.var_vdseffii).ln()) + (locals.var_beta2_i * (locals.var_vdseffii_dn6 / locals.var_vdseffii)))) })), (((0.5 * locals.var_beta0_a_dn7) * assign23080_e31835) + (assign23080_e31830 * if locals.var_beta2_i_dn7 == 0.0 && ((locals.var_beta2_i) as f64).is_finite() && ((locals.var_beta2_i) as f64).fract() == 0.0 { if locals.var_beta2_i == 0.0 { 0.0 } else { (locals.var_beta2_i * ((locals.var_vdseffii).powf(locals.var_beta2_i - 1.0) * locals.var_vdseffii_dn7)) } } else { (assign23080_e31834 * ((locals.var_beta2_i_dn7 * (locals.var_vdseffii).ln()) + (locals.var_beta2_i * (locals.var_vdseffii_dn7 / locals.var_vdseffii)))) })), (((0.5 * locals.var_beta0_a_dn8) * assign23080_e31835) + (assign23080_e31830 * if locals.var_beta2_i_dn8 == 0.0 && ((locals.var_beta2_i) as f64).is_finite() && ((locals.var_beta2_i) as f64).fract() == 0.0 { if locals.var_beta2_i == 0.0 { 0.0 } else { (locals.var_beta2_i * ((locals.var_vdseffii).powf(locals.var_beta2_i - 1.0) * locals.var_vdseffii_dn8)) } } else { (assign23080_e31834 * ((locals.var_beta2_i_dn8 * (locals.var_vdseffii).ln()) + (locals.var_beta2_i * (locals.var_vdseffii_dn8 / locals.var_vdseffii)))) })), (((0.5 * locals.var_beta0_a_dn9) * assign23080_e31835) + (assign23080_e31830 * if locals.var_beta2_i_dn9 == 0.0 && ((locals.var_beta2_i) as f64).is_finite() && ((locals.var_beta2_i) as f64).fract() == 0.0 { if locals.var_beta2_i == 0.0 { 0.0 } else { (locals.var_beta2_i * ((locals.var_vdseffii).powf(locals.var_beta2_i - 1.0) * locals.var_vdseffii_dn9)) } } else { (assign23080_e31834 * ((locals.var_beta2_i_dn9 * (locals.var_vdseffii).ln()) + (locals.var_beta2_i * (locals.var_vdseffii_dn9 / locals.var_vdseffii)))) })), (((0.5 * locals.var_beta0_a_dn10) * assign23080_e31835) + (assign23080_e31830 * if locals.var_beta2_i_dn10 == 0.0 && ((locals.var_beta2_i) as f64).is_finite() && ((locals.var_beta2_i) as f64).fract() == 0.0 { if locals.var_beta2_i == 0.0 { 0.0 } else { (locals.var_beta2_i * ((locals.var_vdseffii).powf(locals.var_beta2_i - 1.0) * locals.var_vdseffii_dn10)) } } else { (assign23080_e31834 * ((locals.var_beta2_i_dn10 * (locals.var_vdseffii).ln()) + (locals.var_beta2_i * (locals.var_vdseffii_dn10 / locals.var_vdseffii)))) })), (((0.5 * locals.var_beta0_a_dn11) * assign23080_e31835) + (assign23080_e31830 * if locals.var_beta2_i_dn11 == 0.0 && ((locals.var_beta2_i) as f64).is_finite() && ((locals.var_beta2_i) as f64).fract() == 0.0 { if locals.var_beta2_i == 0.0 { 0.0 } else { (locals.var_beta2_i * ((locals.var_vdseffii).powf(locals.var_beta2_i - 1.0) * locals.var_vdseffii_dn11)) } } else { (assign23080_e31834 * ((locals.var_beta2_i_dn11 * (locals.var_vdseffii).ln()) + (locals.var_beta2_i * (locals.var_vdseffii_dn11 / locals.var_vdseffii)))) })), (((0.5 * locals.var_beta0_a_dn12) * assign23080_e31835) + (assign23080_e31830 * if locals.var_beta2_i_dn12 == 0.0 && ((locals.var_beta2_i) as f64).is_finite() && ((locals.var_beta2_i) as f64).fract() == 0.0 { if locals.var_beta2_i == 0.0 { 0.0 } else { (locals.var_beta2_i * ((locals.var_vdseffii).powf(locals.var_beta2_i - 1.0) * locals.var_vdseffii_dn12)) } } else { (assign23080_e31834 * ((locals.var_beta2_i_dn12 * (locals.var_vdseffii).ln()) + (locals.var_beta2_i * (locals.var_vdseffii_dn12 / locals.var_vdseffii)))) })), (((0.5 * locals.var_beta0_a_dn13) * assign23080_e31835) + (assign23080_e31830 * if locals.var_beta2_i_dn13 == 0.0 && ((locals.var_beta2_i) as f64).is_finite() && ((locals.var_beta2_i) as f64).fract() == 0.0 { if locals.var_beta2_i == 0.0 { 0.0 } else { (locals.var_beta2_i * ((locals.var_vdseffii).powf(locals.var_beta2_i - 1.0) * locals.var_vdseffii_dn13)) } } else { (assign23080_e31834 * ((locals.var_beta2_i_dn13 * (locals.var_vdseffii).ln()) + (locals.var_beta2_i * (locals.var_vdseffii_dn13 / locals.var_vdseffii)))) })), (((0.5 * locals.var_beta0_a_dn14) * assign23080_e31835) + (assign23080_e31830 * if locals.var_beta2_i_dn14 == 0.0 && ((locals.var_beta2_i) as f64).is_finite() && ((locals.var_beta2_i) as f64).fract() == 0.0 { if locals.var_beta2_i == 0.0 { 0.0 } else { (locals.var_beta2_i * ((locals.var_vdseffii).powf(locals.var_beta2_i - 1.0) * locals.var_vdseffii_dn14)) } } else { (assign23080_e31834 * ((locals.var_beta2_i_dn14 * (locals.var_vdseffii).ln()) + (locals.var_beta2_i * (locals.var_vdseffii_dn14 / locals.var_vdseffii)))) })),)
    } else {
        (locals.var_beta0_eff, locals.var_beta0_eff_dn0, locals.var_beta0_eff_dn2, locals.var_beta0_eff_dn3, locals.var_beta0_eff_dn4, locals.var_beta0_eff_dn5, locals.var_beta0_eff_dn6, locals.var_beta0_eff_dn7, locals.var_beta0_eff_dn8, locals.var_beta0_eff_dn9, locals.var_beta0_eff_dn10, locals.var_beta0_eff_dn11, locals.var_beta0_eff_dn12, locals.var_beta0_eff_dn13, locals.var_beta0_eff_dn14,)
    }
};
        locals.var_beta0_eff = assign23080_e31838;
        locals.var_beta0_eff_dn0 = assign23080_e31838_d_n0;
        locals.var_beta0_eff_dn2 = assign23080_e31838_d_n2;
        locals.var_beta0_eff_dn3 = assign23080_e31838_d_n3;
        locals.var_beta0_eff_dn4 = assign23080_e31838_d_n4;
        locals.var_beta0_eff_dn5 = assign23080_e31838_d_n5;
        locals.var_beta0_eff_dn6 = assign23080_e31838_d_n6;
        locals.var_beta0_eff_dn7 = assign23080_e31838_d_n7;
        locals.var_beta0_eff_dn8 = assign23080_e31838_d_n8;
        locals.var_beta0_eff_dn9 = assign23080_e31838_d_n9;
        locals.var_beta0_eff_dn10 = assign23080_e31838_d_n10;
        locals.var_beta0_eff_dn11 = assign23080_e31838_d_n11;
        locals.var_beta0_eff_dn12 = assign23080_e31838_d_n12;
        locals.var_beta0_eff_dn13 = assign23080_e31838_d_n13;
        locals.var_beta0_eff_dn14 = assign23080_e31838_d_n14;
        locals.var_beta0_eff_rv = 0.0;

        let (assign23090_e31852, assign23090_e31852_d_n0, assign23090_e31852_d_n2, assign23090_e31852_d_n3, assign23090_e31852_d_n4, assign23090_e31852_d_n5, assign23090_e31852_d_n6, assign23090_e31852_d_n7, assign23090_e31852_d_n8, assign23090_e31852_d_n9, assign23090_e31852_d_n10, assign23090_e31852_d_n11, assign23090_e31852_d_n12, assign23090_e31852_d_n13, assign23090_e31852_d_n14,) = {
    if ((locals.var_guard585 == 0.0) && (locals.var_guard588 != 0.0)) {
        let assign23090_e31847: f64 = (p.p492 * locals.var_vdsx);
        let assign23090_e31848: f64 = { let limited_exp_arg = assign23090_e31847; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign23090_e31849: f64 = (p.p493 * assign23090_e31848);
        let assign23090_e31850: f64 = (1.0 + assign23090_e31849);
        (assign23090_e31850, (p.p493 * ({ let limited_exp_arg = assign23090_e31847; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p492 * locals.var_vdsx_dn0))), (p.p493 * ({ let limited_exp_arg = assign23090_e31847; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p492 * locals.var_vdsx_dn2))), (p.p493 * ({ let limited_exp_arg = assign23090_e31847; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p492 * locals.var_vdsx_dn3))), (p.p493 * ({ let limited_exp_arg = assign23090_e31847; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p492 * locals.var_vdsx_dn4))), (p.p493 * ({ let limited_exp_arg = assign23090_e31847; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p492 * locals.var_vdsx_dn5))), (p.p493 * ({ let limited_exp_arg = assign23090_e31847; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p492 * locals.var_vdsx_dn6))), (p.p493 * ({ let limited_exp_arg = assign23090_e31847; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p492 * locals.var_vdsx_dn7))), (p.p493 * ({ let limited_exp_arg = assign23090_e31847; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p492 * locals.var_vdsx_dn8))), (p.p493 * ({ let limited_exp_arg = assign23090_e31847; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p492 * locals.var_vdsx_dn9))), (p.p493 * ({ let limited_exp_arg = assign23090_e31847; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p492 * locals.var_vdsx_dn10))), (p.p493 * ({ let limited_exp_arg = assign23090_e31847; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p492 * locals.var_vdsx_dn11))), (p.p493 * ({ let limited_exp_arg = assign23090_e31847; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p492 * locals.var_vdsx_dn12))), (p.p493 * ({ let limited_exp_arg = assign23090_e31847; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p492 * locals.var_vdsx_dn13))), (p.p493 * ({ let limited_exp_arg = assign23090_e31847; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (p.p492 * locals.var_vdsx_dn14))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23090_e31852;
        locals.var_t1_dn0 = assign23090_e31852_d_n0;
        locals.var_t1_dn2 = assign23090_e31852_d_n2;
        locals.var_t1_dn3 = assign23090_e31852_d_n3;
        locals.var_t1_dn4 = assign23090_e31852_d_n4;
        locals.var_t1_dn5 = assign23090_e31852_d_n5;
        locals.var_t1_dn6 = assign23090_e31852_d_n6;
        locals.var_t1_dn7 = assign23090_e31852_d_n7;
        locals.var_t1_dn8 = assign23090_e31852_d_n8;
        locals.var_t1_dn9 = assign23090_e31852_d_n9;
        locals.var_t1_dn10 = assign23090_e31852_d_n10;
        locals.var_t1_dn11 = assign23090_e31852_d_n11;
        locals.var_t1_dn12 = assign23090_e31852_d_n12;
        locals.var_t1_dn13 = assign23090_e31852_d_n13;
        locals.var_t1_dn14 = assign23090_e31852_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign23100_e31861, assign23100_e31861_d_n0, assign23100_e31861_d_n2, assign23100_e31861_d_n3, assign23100_e31861_d_n4, assign23100_e31861_d_n5, assign23100_e31861_d_n6, assign23100_e31861_d_n7, assign23100_e31861_d_n8, assign23100_e31861_d_n9, assign23100_e31861_d_n10, assign23100_e31861_d_n11, assign23100_e31861_d_n12, assign23100_e31861_d_n13, assign23100_e31861_d_n14,) = {
    if ((locals.var_guard585 == 0.0) && (locals.var_guard588 != 0.0)) {
        let assign23100_e31859: f64 = (locals.var_alpha0_a / locals.var_t1);
        (assign23100_e31859, (((locals.var_alpha0_a_dn0 * locals.var_t1) - (locals.var_alpha0_a * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1)), (((locals.var_alpha0_a_dn2 * locals.var_t1) - (locals.var_alpha0_a * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1)), (((locals.var_alpha0_a_dn3 * locals.var_t1) - (locals.var_alpha0_a * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), (((locals.var_alpha0_a_dn4 * locals.var_t1) - (locals.var_alpha0_a * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_alpha0_a_dn5 * locals.var_t1) - (locals.var_alpha0_a * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_alpha0_a_dn6 * locals.var_t1) - (locals.var_alpha0_a * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_alpha0_a_dn7 * locals.var_t1) - (locals.var_alpha0_a * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((locals.var_alpha0_a_dn8 * locals.var_t1) - (locals.var_alpha0_a * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_alpha0_a_dn9 * locals.var_t1) - (locals.var_alpha0_a * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((locals.var_alpha0_a_dn10 * locals.var_t1) - (locals.var_alpha0_a * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_alpha0_a_dn11 * locals.var_t1) - (locals.var_alpha0_a * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)), (((locals.var_alpha0_a_dn12 * locals.var_t1) - (locals.var_alpha0_a * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1)), (((locals.var_alpha0_a_dn13 * locals.var_t1) - (locals.var_alpha0_a * locals.var_t1_dn13)) / (locals.var_t1 * locals.var_t1)), (((locals.var_alpha0_a_dn14 * locals.var_t1) - (locals.var_alpha0_a * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_alpha0_eff, locals.var_alpha0_eff_dn0, locals.var_alpha0_eff_dn2, locals.var_alpha0_eff_dn3, locals.var_alpha0_eff_dn4, locals.var_alpha0_eff_dn5, locals.var_alpha0_eff_dn6, locals.var_alpha0_eff_dn7, locals.var_alpha0_eff_dn8, locals.var_alpha0_eff_dn9, locals.var_alpha0_eff_dn10, locals.var_alpha0_eff_dn11, locals.var_alpha0_eff_dn12, locals.var_alpha0_eff_dn13, locals.var_alpha0_eff_dn14,)
    }
};
        locals.var_alpha0_eff = assign23100_e31861;
        locals.var_alpha0_eff_dn0 = assign23100_e31861_d_n0;
        locals.var_alpha0_eff_dn2 = assign23100_e31861_d_n2;
        locals.var_alpha0_eff_dn3 = assign23100_e31861_d_n3;
        locals.var_alpha0_eff_dn4 = assign23100_e31861_d_n4;
        locals.var_alpha0_eff_dn5 = assign23100_e31861_d_n5;
        locals.var_alpha0_eff_dn6 = assign23100_e31861_d_n6;
        locals.var_alpha0_eff_dn7 = assign23100_e31861_d_n7;
        locals.var_alpha0_eff_dn8 = assign23100_e31861_d_n8;
        locals.var_alpha0_eff_dn9 = assign23100_e31861_d_n9;
        locals.var_alpha0_eff_dn10 = assign23100_e31861_d_n10;
        locals.var_alpha0_eff_dn11 = assign23100_e31861_d_n11;
        locals.var_alpha0_eff_dn12 = assign23100_e31861_d_n12;
        locals.var_alpha0_eff_dn13 = assign23100_e31861_d_n13;
        locals.var_alpha0_eff_dn14 = assign23100_e31861_d_n14;
        locals.var_alpha0_eff_rv = 0.0;

        let (assign23110_e31880, assign23110_e31880_d_n0, assign23110_e31880_d_n2, assign23110_e31880_d_n3, assign23110_e31880_d_n4, assign23110_e31880_d_n5, assign23110_e31880_d_n6, assign23110_e31880_d_n7, assign23110_e31880_d_n8, assign23110_e31880_d_n9, assign23110_e31880_d_n10, assign23110_e31880_d_n11, assign23110_e31880_d_n12, assign23110_e31880_d_n13, assign23110_e31880_d_n14,) = {
    if ((locals.var_guard585 == 0.0) && (locals.var_guard588 != 0.0)) {
        let assign23110_e31870: f64 = (p.p505 * locals.var_vbsx);
        let assign23110_e31871: f64 = (1.0 + assign23110_e31870);
        let assign23110_e31874: f64 = (p.p506 * locals.var_vbsx);
        let assign23110_e31876: f64 = (assign23110_e31874 * locals.var_vbsx);
        let assign23110_e31877: f64 = (assign23110_e31871 + assign23110_e31876);
        let assign23110_e31878: f64 = (locals.var_alpha0_eff * assign23110_e31877);
        (assign23110_e31878, ((locals.var_alpha0_eff_dn0 * assign23110_e31877) + (locals.var_alpha0_eff * ((p.p505 * locals.var_vbsx_dn0) + (((p.p506 * locals.var_vbsx_dn0) * locals.var_vbsx) + (assign23110_e31874 * locals.var_vbsx_dn0))))), ((locals.var_alpha0_eff_dn2 * assign23110_e31877) + (locals.var_alpha0_eff * ((p.p505 * locals.var_vbsx_dn2) + (((p.p506 * locals.var_vbsx_dn2) * locals.var_vbsx) + (assign23110_e31874 * locals.var_vbsx_dn2))))), ((locals.var_alpha0_eff_dn3 * assign23110_e31877) + (locals.var_alpha0_eff * ((p.p505 * locals.var_vbsx_dn3) + (((p.p506 * locals.var_vbsx_dn3) * locals.var_vbsx) + (assign23110_e31874 * locals.var_vbsx_dn3))))), ((locals.var_alpha0_eff_dn4 * assign23110_e31877) + (locals.var_alpha0_eff * ((p.p505 * locals.var_vbsx_dn4) + (((p.p506 * locals.var_vbsx_dn4) * locals.var_vbsx) + (assign23110_e31874 * locals.var_vbsx_dn4))))), ((locals.var_alpha0_eff_dn5 * assign23110_e31877) + (locals.var_alpha0_eff * ((p.p505 * locals.var_vbsx_dn5) + (((p.p506 * locals.var_vbsx_dn5) * locals.var_vbsx) + (assign23110_e31874 * locals.var_vbsx_dn5))))), ((locals.var_alpha0_eff_dn6 * assign23110_e31877) + (locals.var_alpha0_eff * ((p.p505 * locals.var_vbsx_dn6) + (((p.p506 * locals.var_vbsx_dn6) * locals.var_vbsx) + (assign23110_e31874 * locals.var_vbsx_dn6))))), ((locals.var_alpha0_eff_dn7 * assign23110_e31877) + (locals.var_alpha0_eff * ((p.p505 * locals.var_vbsx_dn7) + (((p.p506 * locals.var_vbsx_dn7) * locals.var_vbsx) + (assign23110_e31874 * locals.var_vbsx_dn7))))), ((locals.var_alpha0_eff_dn8 * assign23110_e31877) + (locals.var_alpha0_eff * ((p.p505 * locals.var_vbsx_dn8) + (((p.p506 * locals.var_vbsx_dn8) * locals.var_vbsx) + (assign23110_e31874 * locals.var_vbsx_dn8))))), ((locals.var_alpha0_eff_dn9 * assign23110_e31877) + (locals.var_alpha0_eff * ((p.p505 * locals.var_vbsx_dn9) + (((p.p506 * locals.var_vbsx_dn9) * locals.var_vbsx) + (assign23110_e31874 * locals.var_vbsx_dn9))))), ((locals.var_alpha0_eff_dn10 * assign23110_e31877) + (locals.var_alpha0_eff * ((p.p505 * locals.var_vbsx_dn10) + (((p.p506 * locals.var_vbsx_dn10) * locals.var_vbsx) + (assign23110_e31874 * locals.var_vbsx_dn10))))), ((locals.var_alpha0_eff_dn11 * assign23110_e31877) + (locals.var_alpha0_eff * ((p.p505 * locals.var_vbsx_dn11) + (((p.p506 * locals.var_vbsx_dn11) * locals.var_vbsx) + (assign23110_e31874 * locals.var_vbsx_dn11))))), ((locals.var_alpha0_eff_dn12 * assign23110_e31877) + (locals.var_alpha0_eff * ((p.p505 * locals.var_vbsx_dn12) + (((p.p506 * locals.var_vbsx_dn12) * locals.var_vbsx) + (assign23110_e31874 * locals.var_vbsx_dn12))))), ((locals.var_alpha0_eff_dn13 * assign23110_e31877) + (locals.var_alpha0_eff * ((p.p505 * locals.var_vbsx_dn13) + (((p.p506 * locals.var_vbsx_dn13) * locals.var_vbsx) + (assign23110_e31874 * locals.var_vbsx_dn13))))), ((locals.var_alpha0_eff_dn14 * assign23110_e31877) + (locals.var_alpha0_eff * ((p.p505 * locals.var_vbsx_dn14) + (((p.p506 * locals.var_vbsx_dn14) * locals.var_vbsx) + (assign23110_e31874 * locals.var_vbsx_dn14))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign23110_e31880;
        locals.var_t2_dn0 = assign23110_e31880_d_n0;
        locals.var_t2_dn2 = assign23110_e31880_d_n2;
        locals.var_t2_dn3 = assign23110_e31880_d_n3;
        locals.var_t2_dn4 = assign23110_e31880_d_n4;
        locals.var_t2_dn5 = assign23110_e31880_d_n5;
        locals.var_t2_dn6 = assign23110_e31880_d_n6;
        locals.var_t2_dn7 = assign23110_e31880_d_n7;
        locals.var_t2_dn8 = assign23110_e31880_d_n8;
        locals.var_t2_dn9 = assign23110_e31880_d_n9;
        locals.var_t2_dn10 = assign23110_e31880_d_n10;
        locals.var_t2_dn11 = assign23110_e31880_d_n11;
        locals.var_t2_dn12 = assign23110_e31880_d_n12;
        locals.var_t2_dn13 = assign23110_e31880_d_n13;
        locals.var_t2_dn14 = assign23110_e31880_d_n14;
        locals.var_t2_rv = 0.0;

        let assign23120_e31886: f64 = (-2500.0);
        let assign23120_e31888: f64 = (assign23120_e31886 * 1e-12);
        let assign23120_e31890: f64 = if ((0.0 == 0.0) && (locals.var_t2 < assign23120_e31888)) { 1.0 } else { 0.0 };
        locals.var_guard590 = assign23120_e31890;
        locals.var_guard590_rv = 0.0;

        let (assign23130_e31906, assign23130_e31906_d_n0, assign23130_e31906_d_n2, assign23130_e31906_d_n3, assign23130_e31906_d_n4, assign23130_e31906_d_n5, assign23130_e31906_d_n6, assign23130_e31906_d_n7, assign23130_e31906_d_n8, assign23130_e31906_d_n9, assign23130_e31906_d_n10, assign23130_e31906_d_n11, assign23130_e31906_d_n12, assign23130_e31906_d_n13, assign23130_e31906_d_n14,) = {
    if (((locals.var_guard585 == 0.0) && (locals.var_guard588 != 0.0)) && (locals.var_guard590 != 0.0)) {
        let assign23130_e31898: f64 = (-1e-12);
        let assign23130_e31900: f64 = (assign23130_e31898 * 1e-12);
        let assign23130_e31903: f64 = (16.0 * locals.var_t2);
        let assign23130_e31904: f64 = (assign23130_e31900 / assign23130_e31903);
        (assign23130_e31904, (-((assign23130_e31900 * (16.0 * locals.var_t2_dn0)) / (assign23130_e31903 * assign23130_e31903))), (-((assign23130_e31900 * (16.0 * locals.var_t2_dn2)) / (assign23130_e31903 * assign23130_e31903))), (-((assign23130_e31900 * (16.0 * locals.var_t2_dn3)) / (assign23130_e31903 * assign23130_e31903))), (-((assign23130_e31900 * (16.0 * locals.var_t2_dn4)) / (assign23130_e31903 * assign23130_e31903))), (-((assign23130_e31900 * (16.0 * locals.var_t2_dn5)) / (assign23130_e31903 * assign23130_e31903))), (-((assign23130_e31900 * (16.0 * locals.var_t2_dn6)) / (assign23130_e31903 * assign23130_e31903))), (-((assign23130_e31900 * (16.0 * locals.var_t2_dn7)) / (assign23130_e31903 * assign23130_e31903))), (-((assign23130_e31900 * (16.0 * locals.var_t2_dn8)) / (assign23130_e31903 * assign23130_e31903))), (-((assign23130_e31900 * (16.0 * locals.var_t2_dn9)) / (assign23130_e31903 * assign23130_e31903))), (-((assign23130_e31900 * (16.0 * locals.var_t2_dn10)) / (assign23130_e31903 * assign23130_e31903))), (-((assign23130_e31900 * (16.0 * locals.var_t2_dn11)) / (assign23130_e31903 * assign23130_e31903))), (-((assign23130_e31900 * (16.0 * locals.var_t2_dn12)) / (assign23130_e31903 * assign23130_e31903))), (-((assign23130_e31900 * (16.0 * locals.var_t2_dn13)) / (assign23130_e31903 * assign23130_e31903))), (-((assign23130_e31900 * (16.0 * locals.var_t2_dn14)) / (assign23130_e31903 * assign23130_e31903))),)
    } else {
        (locals.var_alpha0_eff, locals.var_alpha0_eff_dn0, locals.var_alpha0_eff_dn2, locals.var_alpha0_eff_dn3, locals.var_alpha0_eff_dn4, locals.var_alpha0_eff_dn5, locals.var_alpha0_eff_dn6, locals.var_alpha0_eff_dn7, locals.var_alpha0_eff_dn8, locals.var_alpha0_eff_dn9, locals.var_alpha0_eff_dn10, locals.var_alpha0_eff_dn11, locals.var_alpha0_eff_dn12, locals.var_alpha0_eff_dn13, locals.var_alpha0_eff_dn14,)
    }
};
        locals.var_alpha0_eff = assign23130_e31906;
        locals.var_alpha0_eff_dn0 = assign23130_e31906_d_n0;
        locals.var_alpha0_eff_dn2 = assign23130_e31906_d_n2;
        locals.var_alpha0_eff_dn3 = assign23130_e31906_d_n3;
        locals.var_alpha0_eff_dn4 = assign23130_e31906_d_n4;
        locals.var_alpha0_eff_dn5 = assign23130_e31906_d_n5;
        locals.var_alpha0_eff_dn6 = assign23130_e31906_d_n6;
        locals.var_alpha0_eff_dn7 = assign23130_e31906_d_n7;
        locals.var_alpha0_eff_dn8 = assign23130_e31906_d_n8;
        locals.var_alpha0_eff_dn9 = assign23130_e31906_d_n9;
        locals.var_alpha0_eff_dn10 = assign23130_e31906_d_n10;
        locals.var_alpha0_eff_dn11 = assign23130_e31906_d_n11;
        locals.var_alpha0_eff_dn12 = assign23130_e31906_d_n12;
        locals.var_alpha0_eff_dn13 = assign23130_e31906_d_n13;
        locals.var_alpha0_eff_dn14 = assign23130_e31906_d_n14;
        locals.var_alpha0_eff_rv = 0.0;

        let (assign23140_e31935, assign23140_e31935_d_n0, assign23140_e31935_d_n2, assign23140_e31935_d_n3, assign23140_e31935_d_n4, assign23140_e31935_d_n5, assign23140_e31935_d_n6, assign23140_e31935_d_n7, assign23140_e31935_d_n8, assign23140_e31935_d_n9, assign23140_e31935_d_n10, assign23140_e31935_d_n11, assign23140_e31935_d_n12, assign23140_e31935_d_n13, assign23140_e31935_d_n14,) = {
    if (((locals.var_guard585 == 0.0) && (locals.var_guard588 != 0.0)) && (locals.var_guard590 == 0.0)) {
        let assign23140_e31917: f64 = locals.var_t2;
        let assign23140_e31920: f64 = locals.var_t2;
        let assign23140_e31923: f64 = locals.var_t2;
        let assign23140_e31924: f64 = (assign23140_e31920 * assign23140_e31923);
        let assign23140_e31927: f64 = (0.25 * 1e-12);
        let assign23140_e31929: f64 = (assign23140_e31927 * 1e-12);
        let assign23140_e31930: f64 = (assign23140_e31924 + assign23140_e31929);
        let assign23140_e31931: f64 = (assign23140_e31930).sqrt();
        let assign23140_e31932: f64 = (assign23140_e31917 + assign23140_e31931);
        let assign23140_e31933: f64 = (0.5 * assign23140_e31932);
        (assign23140_e31933, (0.5 * (locals.var_t2_dn0 + (((locals.var_t2_dn0 * assign23140_e31923) + (assign23140_e31920 * locals.var_t2_dn0)) / (2.0 * assign23140_e31931)))), (0.5 * (locals.var_t2_dn2 + (((locals.var_t2_dn2 * assign23140_e31923) + (assign23140_e31920 * locals.var_t2_dn2)) / (2.0 * assign23140_e31931)))), (0.5 * (locals.var_t2_dn3 + (((locals.var_t2_dn3 * assign23140_e31923) + (assign23140_e31920 * locals.var_t2_dn3)) / (2.0 * assign23140_e31931)))), (0.5 * (locals.var_t2_dn4 + (((locals.var_t2_dn4 * assign23140_e31923) + (assign23140_e31920 * locals.var_t2_dn4)) / (2.0 * assign23140_e31931)))), (0.5 * (locals.var_t2_dn5 + (((locals.var_t2_dn5 * assign23140_e31923) + (assign23140_e31920 * locals.var_t2_dn5)) / (2.0 * assign23140_e31931)))), (0.5 * (locals.var_t2_dn6 + (((locals.var_t2_dn6 * assign23140_e31923) + (assign23140_e31920 * locals.var_t2_dn6)) / (2.0 * assign23140_e31931)))), (0.5 * (locals.var_t2_dn7 + (((locals.var_t2_dn7 * assign23140_e31923) + (assign23140_e31920 * locals.var_t2_dn7)) / (2.0 * assign23140_e31931)))), (0.5 * (locals.var_t2_dn8 + (((locals.var_t2_dn8 * assign23140_e31923) + (assign23140_e31920 * locals.var_t2_dn8)) / (2.0 * assign23140_e31931)))), (0.5 * (locals.var_t2_dn9 + (((locals.var_t2_dn9 * assign23140_e31923) + (assign23140_e31920 * locals.var_t2_dn9)) / (2.0 * assign23140_e31931)))), (0.5 * (locals.var_t2_dn10 + (((locals.var_t2_dn10 * assign23140_e31923) + (assign23140_e31920 * locals.var_t2_dn10)) / (2.0 * assign23140_e31931)))), (0.5 * (locals.var_t2_dn11 + (((locals.var_t2_dn11 * assign23140_e31923) + (assign23140_e31920 * locals.var_t2_dn11)) / (2.0 * assign23140_e31931)))), (0.5 * (locals.var_t2_dn12 + (((locals.var_t2_dn12 * assign23140_e31923) + (assign23140_e31920 * locals.var_t2_dn12)) / (2.0 * assign23140_e31931)))), (0.5 * (locals.var_t2_dn13 + (((locals.var_t2_dn13 * assign23140_e31923) + (assign23140_e31920 * locals.var_t2_dn13)) / (2.0 * assign23140_e31931)))), (0.5 * (locals.var_t2_dn14 + (((locals.var_t2_dn14 * assign23140_e31923) + (assign23140_e31920 * locals.var_t2_dn14)) / (2.0 * assign23140_e31931)))),)
    } else {
        (locals.var_alpha0_eff, locals.var_alpha0_eff_dn0, locals.var_alpha0_eff_dn2, locals.var_alpha0_eff_dn3, locals.var_alpha0_eff_dn4, locals.var_alpha0_eff_dn5, locals.var_alpha0_eff_dn6, locals.var_alpha0_eff_dn7, locals.var_alpha0_eff_dn8, locals.var_alpha0_eff_dn9, locals.var_alpha0_eff_dn10, locals.var_alpha0_eff_dn11, locals.var_alpha0_eff_dn12, locals.var_alpha0_eff_dn13, locals.var_alpha0_eff_dn14,)
    }
};
        locals.var_alpha0_eff = assign23140_e31935;
        locals.var_alpha0_eff_dn0 = assign23140_e31935_d_n0;
        locals.var_alpha0_eff_dn2 = assign23140_e31935_d_n2;
        locals.var_alpha0_eff_dn3 = assign23140_e31935_d_n3;
        locals.var_alpha0_eff_dn4 = assign23140_e31935_d_n4;
        locals.var_alpha0_eff_dn5 = assign23140_e31935_d_n5;
        locals.var_alpha0_eff_dn6 = assign23140_e31935_d_n6;
        locals.var_alpha0_eff_dn7 = assign23140_e31935_d_n7;
        locals.var_alpha0_eff_dn8 = assign23140_e31935_d_n8;
        locals.var_alpha0_eff_dn9 = assign23140_e31935_d_n9;
        locals.var_alpha0_eff_dn10 = assign23140_e31935_d_n10;
        locals.var_alpha0_eff_dn11 = assign23140_e31935_d_n11;
        locals.var_alpha0_eff_dn12 = assign23140_e31935_d_n12;
        locals.var_alpha0_eff_dn13 = assign23140_e31935_d_n13;
        locals.var_alpha0_eff_dn14 = assign23140_e31935_d_n14;
        locals.var_alpha0_eff_rv = 0.0;

        let assign23150_e31942: f64 = if ((locals.var_alpha0_a <= 0.0) || (locals.var_beta0_a <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard591 = assign23150_e31942;
        locals.var_guard591_rv = 0.0;

        let assign23170_e31955: f64 = (locals.var_beta0_eff / 80.0);
        let assign23170_e31956: f64 = if locals.var_diffvdsii > assign23170_e31955 { 1.0 } else { 0.0 };
        locals.var_guard592 = assign23170_e31956;
        locals.var_guard592_rv = 0.0;

        let (assign23180_e31973, assign23180_e31973_d_n0, assign23180_e31973_d_n2, assign23180_e31973_d_n3, assign23180_e31973_d_n4, assign23180_e31973_d_n5, assign23180_e31973_d_n6, assign23180_e31973_d_n7, assign23180_e31973_d_n8, assign23180_e31973_d_n9, assign23180_e31973_d_n10, assign23180_e31973_d_n11, assign23180_e31973_d_n12, assign23180_e31973_d_n13, assign23180_e31973_d_n14,) = {
    if ((((locals.var_guard585 == 0.0) && (locals.var_guard588 != 0.0)) && (locals.var_guard591 == 0.0)) && (locals.var_guard592 != 0.0)) {
        let assign23180_e31967: f64 = (-locals.var_beta0_eff);
        let assign23180_e31970: f64 = (locals.var_diffvdsii).powf(p.p524);
        let assign23180_e31971: f64 = (assign23180_e31967 / assign23180_e31970);
        (assign23180_e31971, ((((-locals.var_beta0_eff_dn0) * assign23180_e31970) - (assign23180_e31967 * if 0.0 == 0.0 && ((p.p524) as f64).is_finite() && ((p.p524) as f64).fract() == 0.0 { if p.p524 == 0.0 { 0.0 } else { (p.p524 * ((locals.var_diffvdsii).powf(p.p524 - 1.0) * locals.var_diffvdsii_dn0)) } } else { (assign23180_e31970 * (p.p524 * (locals.var_diffvdsii_dn0 / locals.var_diffvdsii))) })) / (assign23180_e31970 * assign23180_e31970)), ((((-locals.var_beta0_eff_dn2) * assign23180_e31970) - (assign23180_e31967 * if 0.0 == 0.0 && ((p.p524) as f64).is_finite() && ((p.p524) as f64).fract() == 0.0 { if p.p524 == 0.0 { 0.0 } else { (p.p524 * ((locals.var_diffvdsii).powf(p.p524 - 1.0) * locals.var_diffvdsii_dn2)) } } else { (assign23180_e31970 * (p.p524 * (locals.var_diffvdsii_dn2 / locals.var_diffvdsii))) })) / (assign23180_e31970 * assign23180_e31970)), ((((-locals.var_beta0_eff_dn3) * assign23180_e31970) - (assign23180_e31967 * if 0.0 == 0.0 && ((p.p524) as f64).is_finite() && ((p.p524) as f64).fract() == 0.0 { if p.p524 == 0.0 { 0.0 } else { (p.p524 * ((locals.var_diffvdsii).powf(p.p524 - 1.0) * locals.var_diffvdsii_dn3)) } } else { (assign23180_e31970 * (p.p524 * (locals.var_diffvdsii_dn3 / locals.var_diffvdsii))) })) / (assign23180_e31970 * assign23180_e31970)), ((((-locals.var_beta0_eff_dn4) * assign23180_e31970) - (assign23180_e31967 * if 0.0 == 0.0 && ((p.p524) as f64).is_finite() && ((p.p524) as f64).fract() == 0.0 { if p.p524 == 0.0 { 0.0 } else { (p.p524 * ((locals.var_diffvdsii).powf(p.p524 - 1.0) * locals.var_diffvdsii_dn4)) } } else { (assign23180_e31970 * (p.p524 * (locals.var_diffvdsii_dn4 / locals.var_diffvdsii))) })) / (assign23180_e31970 * assign23180_e31970)), ((((-locals.var_beta0_eff_dn5) * assign23180_e31970) - (assign23180_e31967 * if 0.0 == 0.0 && ((p.p524) as f64).is_finite() && ((p.p524) as f64).fract() == 0.0 { if p.p524 == 0.0 { 0.0 } else { (p.p524 * ((locals.var_diffvdsii).powf(p.p524 - 1.0) * locals.var_diffvdsii_dn5)) } } else { (assign23180_e31970 * (p.p524 * (locals.var_diffvdsii_dn5 / locals.var_diffvdsii))) })) / (assign23180_e31970 * assign23180_e31970)), ((((-locals.var_beta0_eff_dn6) * assign23180_e31970) - (assign23180_e31967 * if 0.0 == 0.0 && ((p.p524) as f64).is_finite() && ((p.p524) as f64).fract() == 0.0 { if p.p524 == 0.0 { 0.0 } else { (p.p524 * ((locals.var_diffvdsii).powf(p.p524 - 1.0) * locals.var_diffvdsii_dn6)) } } else { (assign23180_e31970 * (p.p524 * (locals.var_diffvdsii_dn6 / locals.var_diffvdsii))) })) / (assign23180_e31970 * assign23180_e31970)), ((((-locals.var_beta0_eff_dn7) * assign23180_e31970) - (assign23180_e31967 * if 0.0 == 0.0 && ((p.p524) as f64).is_finite() && ((p.p524) as f64).fract() == 0.0 { if p.p524 == 0.0 { 0.0 } else { (p.p524 * ((locals.var_diffvdsii).powf(p.p524 - 1.0) * locals.var_diffvdsii_dn7)) } } else { (assign23180_e31970 * (p.p524 * (locals.var_diffvdsii_dn7 / locals.var_diffvdsii))) })) / (assign23180_e31970 * assign23180_e31970)), ((((-locals.var_beta0_eff_dn8) * assign23180_e31970) - (assign23180_e31967 * if 0.0 == 0.0 && ((p.p524) as f64).is_finite() && ((p.p524) as f64).fract() == 0.0 { if p.p524 == 0.0 { 0.0 } else { (p.p524 * ((locals.var_diffvdsii).powf(p.p524 - 1.0) * locals.var_diffvdsii_dn8)) } } else { (assign23180_e31970 * (p.p524 * (locals.var_diffvdsii_dn8 / locals.var_diffvdsii))) })) / (assign23180_e31970 * assign23180_e31970)), ((((-locals.var_beta0_eff_dn9) * assign23180_e31970) - (assign23180_e31967 * if 0.0 == 0.0 && ((p.p524) as f64).is_finite() && ((p.p524) as f64).fract() == 0.0 { if p.p524 == 0.0 { 0.0 } else { (p.p524 * ((locals.var_diffvdsii).powf(p.p524 - 1.0) * locals.var_diffvdsii_dn9)) } } else { (assign23180_e31970 * (p.p524 * (locals.var_diffvdsii_dn9 / locals.var_diffvdsii))) })) / (assign23180_e31970 * assign23180_e31970)), ((((-locals.var_beta0_eff_dn10) * assign23180_e31970) - (assign23180_e31967 * if 0.0 == 0.0 && ((p.p524) as f64).is_finite() && ((p.p524) as f64).fract() == 0.0 { if p.p524 == 0.0 { 0.0 } else { (p.p524 * ((locals.var_diffvdsii).powf(p.p524 - 1.0) * locals.var_diffvdsii_dn10)) } } else { (assign23180_e31970 * (p.p524 * (locals.var_diffvdsii_dn10 / locals.var_diffvdsii))) })) / (assign23180_e31970 * assign23180_e31970)), ((((-locals.var_beta0_eff_dn11) * assign23180_e31970) - (assign23180_e31967 * if 0.0 == 0.0 && ((p.p524) as f64).is_finite() && ((p.p524) as f64).fract() == 0.0 { if p.p524 == 0.0 { 0.0 } else { (p.p524 * ((locals.var_diffvdsii).powf(p.p524 - 1.0) * locals.var_diffvdsii_dn11)) } } else { (assign23180_e31970 * (p.p524 * (locals.var_diffvdsii_dn11 / locals.var_diffvdsii))) })) / (assign23180_e31970 * assign23180_e31970)), ((((-locals.var_beta0_eff_dn12) * assign23180_e31970) - (assign23180_e31967 * if 0.0 == 0.0 && ((p.p524) as f64).is_finite() && ((p.p524) as f64).fract() == 0.0 { if p.p524 == 0.0 { 0.0 } else { (p.p524 * ((locals.var_diffvdsii).powf(p.p524 - 1.0) * locals.var_diffvdsii_dn12)) } } else { (assign23180_e31970 * (p.p524 * (locals.var_diffvdsii_dn12 / locals.var_diffvdsii))) })) / (assign23180_e31970 * assign23180_e31970)), ((((-locals.var_beta0_eff_dn13) * assign23180_e31970) - (assign23180_e31967 * if 0.0 == 0.0 && ((p.p524) as f64).is_finite() && ((p.p524) as f64).fract() == 0.0 { if p.p524 == 0.0 { 0.0 } else { (p.p524 * ((locals.var_diffvdsii).powf(p.p524 - 1.0) * locals.var_diffvdsii_dn13)) } } else { (assign23180_e31970 * (p.p524 * (locals.var_diffvdsii_dn13 / locals.var_diffvdsii))) })) / (assign23180_e31970 * assign23180_e31970)), ((((-locals.var_beta0_eff_dn14) * assign23180_e31970) - (assign23180_e31967 * if 0.0 == 0.0 && ((p.p524) as f64).is_finite() && ((p.p524) as f64).fract() == 0.0 { if p.p524 == 0.0 { 0.0 } else { (p.p524 * ((locals.var_diffvdsii).powf(p.p524 - 1.0) * locals.var_diffvdsii_dn14)) } } else { (assign23180_e31970 * (p.p524 * (locals.var_diffvdsii_dn14 / locals.var_diffvdsii))) })) / (assign23180_e31970 * assign23180_e31970)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23180_e31973;
        locals.var_t1_dn0 = assign23180_e31973_d_n0;
        locals.var_t1_dn2 = assign23180_e31973_d_n2;
        locals.var_t1_dn3 = assign23180_e31973_d_n3;
        locals.var_t1_dn4 = assign23180_e31973_d_n4;
        locals.var_t1_dn5 = assign23180_e31973_d_n5;
        locals.var_t1_dn6 = assign23180_e31973_d_n6;
        locals.var_t1_dn7 = assign23180_e31973_d_n7;
        locals.var_t1_dn8 = assign23180_e31973_d_n8;
        locals.var_t1_dn9 = assign23180_e31973_d_n9;
        locals.var_t1_dn10 = assign23180_e31973_d_n10;
        locals.var_t1_dn11 = assign23180_e31973_d_n11;
        locals.var_t1_dn12 = assign23180_e31973_d_n12;
        locals.var_t1_dn13 = assign23180_e31973_d_n13;
        locals.var_t1_dn14 = assign23180_e31973_d_n14;
        locals.var_t1_rv = 0.0;

        let assign23210_e32022: f64 = if ((p.p1094 == 1.0) && (p.p1098 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard593 = assign23210_e32022;
        locals.var_guard593_rv = 0.0;

        let (assign23220_e32028, assign23220_e32028_d_n0, assign23220_e32028_d_n2, assign23220_e32028_d_n3, assign23220_e32028_d_n4, assign23220_e32028_d_n5, assign23220_e32028_d_n6, assign23220_e32028_d_n7, assign23220_e32028_d_n8, assign23220_e32028_d_n9, assign23220_e32028_d_n10, assign23220_e32028_d_n11, assign23220_e32028_d_n12, assign23220_e32028_d_n13, assign23220_e32028_d_n14,) = {
    if (locals.var_guard593 != 0.0) {
        let assign23220_e32026: f64 = (locals.var_qs_1 - p.p1105);
        (assign23220_e32026, locals.var_qs_1_dn0, locals.var_qs_1_dn2, locals.var_qs_1_dn3, locals.var_qs_1_dn4, locals.var_qs_1_dn5, locals.var_qs_1_dn6, locals.var_qs_1_dn7, locals.var_qs_1_dn8, locals.var_qs_1_dn9, locals.var_qs_1_dn10, locals.var_qs_1_dn11, locals.var_qs_1_dn12, locals.var_qs_1_dn13, locals.var_qs_1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23220_e32028;
        locals.var_t1_dn0 = assign23220_e32028_d_n0;
        locals.var_t1_dn2 = assign23220_e32028_d_n2;
        locals.var_t1_dn3 = assign23220_e32028_d_n3;
        locals.var_t1_dn4 = assign23220_e32028_d_n4;
        locals.var_t1_dn5 = assign23220_e32028_d_n5;
        locals.var_t1_dn6 = assign23220_e32028_d_n6;
        locals.var_t1_dn7 = assign23220_e32028_d_n7;
        locals.var_t1_dn8 = assign23220_e32028_d_n8;
        locals.var_t1_dn9 = assign23220_e32028_d_n9;
        locals.var_t1_dn10 = assign23220_e32028_d_n10;
        locals.var_t1_dn11 = assign23220_e32028_d_n11;
        locals.var_t1_dn12 = assign23220_e32028_d_n12;
        locals.var_t1_dn13 = assign23220_e32028_d_n13;
        locals.var_t1_dn14 = assign23220_e32028_d_n14;
        locals.var_t1_rv = 0.0;

        let assign23230_e32034: f64 = (-2500.0);
        let assign23230_e32036: f64 = (assign23230_e32034 * 2.0);
        let assign23230_e32038: f64 = if ((0.1 == 0.0) && (locals.var_t1 < assign23230_e32036)) { 1.0 } else { 0.0 };
        locals.var_guard594 = assign23230_e32038;
        locals.var_guard594_rv = 0.0;

        let (assign23240_e32051, assign23240_e32051_d_n0, assign23240_e32051_d_n2, assign23240_e32051_d_n3, assign23240_e32051_d_n4, assign23240_e32051_d_n5, assign23240_e32051_d_n6, assign23240_e32051_d_n7, assign23240_e32051_d_n8, assign23240_e32051_d_n9, assign23240_e32051_d_n10, assign23240_e32051_d_n11, assign23240_e32051_d_n12, assign23240_e32051_d_n13, assign23240_e32051_d_n14,) = {
    if ((locals.var_guard593 != 0.0) && (locals.var_guard594 != 0.0)) {
        let assign23240_e32043: f64 = (-2.0);
        let assign23240_e32045: f64 = (assign23240_e32043 * 2.0);
        let assign23240_e32048: f64 = (16.0 * locals.var_t1);
        let assign23240_e32049: f64 = (assign23240_e32045 / assign23240_e32048);
        (assign23240_e32049, (-((assign23240_e32045 * (16.0 * locals.var_t1_dn0)) / (assign23240_e32048 * assign23240_e32048))), (-((assign23240_e32045 * (16.0 * locals.var_t1_dn2)) / (assign23240_e32048 * assign23240_e32048))), (-((assign23240_e32045 * (16.0 * locals.var_t1_dn3)) / (assign23240_e32048 * assign23240_e32048))), (-((assign23240_e32045 * (16.0 * locals.var_t1_dn4)) / (assign23240_e32048 * assign23240_e32048))), (-((assign23240_e32045 * (16.0 * locals.var_t1_dn5)) / (assign23240_e32048 * assign23240_e32048))), (-((assign23240_e32045 * (16.0 * locals.var_t1_dn6)) / (assign23240_e32048 * assign23240_e32048))), (-((assign23240_e32045 * (16.0 * locals.var_t1_dn7)) / (assign23240_e32048 * assign23240_e32048))), (-((assign23240_e32045 * (16.0 * locals.var_t1_dn8)) / (assign23240_e32048 * assign23240_e32048))), (-((assign23240_e32045 * (16.0 * locals.var_t1_dn9)) / (assign23240_e32048 * assign23240_e32048))), (-((assign23240_e32045 * (16.0 * locals.var_t1_dn10)) / (assign23240_e32048 * assign23240_e32048))), (-((assign23240_e32045 * (16.0 * locals.var_t1_dn11)) / (assign23240_e32048 * assign23240_e32048))), (-((assign23240_e32045 * (16.0 * locals.var_t1_dn12)) / (assign23240_e32048 * assign23240_e32048))), (-((assign23240_e32045 * (16.0 * locals.var_t1_dn13)) / (assign23240_e32048 * assign23240_e32048))), (-((assign23240_e32045 * (16.0 * locals.var_t1_dn14)) / (assign23240_e32048 * assign23240_e32048))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23240_e32051;
        locals.var_t1_dn0 = assign23240_e32051_d_n0;
        locals.var_t1_dn2 = assign23240_e32051_d_n2;
        locals.var_t1_dn3 = assign23240_e32051_d_n3;
        locals.var_t1_dn4 = assign23240_e32051_d_n4;
        locals.var_t1_dn5 = assign23240_e32051_d_n5;
        locals.var_t1_dn6 = assign23240_e32051_d_n6;
        locals.var_t1_dn7 = assign23240_e32051_d_n7;
        locals.var_t1_dn8 = assign23240_e32051_d_n8;
        locals.var_t1_dn9 = assign23240_e32051_d_n9;
        locals.var_t1_dn10 = assign23240_e32051_d_n10;
        locals.var_t1_dn11 = assign23240_e32051_d_n11;
        locals.var_t1_dn12 = assign23240_e32051_d_n12;
        locals.var_t1_dn13 = assign23240_e32051_d_n13;
        locals.var_t1_dn14 = assign23240_e32051_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_68(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let (assign23250_e32077, assign23250_e32077_d_n0, assign23250_e32077_d_n2, assign23250_e32077_d_n3, assign23250_e32077_d_n4, assign23250_e32077_d_n5, assign23250_e32077_d_n6, assign23250_e32077_d_n7, assign23250_e32077_d_n8, assign23250_e32077_d_n9, assign23250_e32077_d_n10, assign23250_e32077_d_n11, assign23250_e32077_d_n12, assign23250_e32077_d_n13, assign23250_e32077_d_n14,) = {
    if ((locals.var_guard593 != 0.0) && (locals.var_guard594 == 0.0)) {
        let assign23250_e32059: f64 = (locals.var_t1 + 0.1);
        let assign23250_e32062: f64 = (locals.var_t1 - 0.1);
        let assign23250_e32065: f64 = (locals.var_t1 - 0.1);
        let assign23250_e32066: f64 = (assign23250_e32062 * assign23250_e32065);
        let assign23250_e32069: f64 = (0.25 * 2.0);
        let assign23250_e32071: f64 = (assign23250_e32069 * 2.0);
        let assign23250_e32072: f64 = (assign23250_e32066 + assign23250_e32071);
        let assign23250_e32073: f64 = (assign23250_e32072).sqrt();
        let assign23250_e32074: f64 = (assign23250_e32059 + assign23250_e32073);
        let assign23250_e32075: f64 = (0.5 * assign23250_e32074);
        (assign23250_e32075, (0.5 * (locals.var_t1_dn0 + (((locals.var_t1_dn0 * assign23250_e32065) + (assign23250_e32062 * locals.var_t1_dn0)) / (2.0 * assign23250_e32073)))), (0.5 * (locals.var_t1_dn2 + (((locals.var_t1_dn2 * assign23250_e32065) + (assign23250_e32062 * locals.var_t1_dn2)) / (2.0 * assign23250_e32073)))), (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * assign23250_e32065) + (assign23250_e32062 * locals.var_t1_dn3)) / (2.0 * assign23250_e32073)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * assign23250_e32065) + (assign23250_e32062 * locals.var_t1_dn4)) / (2.0 * assign23250_e32073)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * assign23250_e32065) + (assign23250_e32062 * locals.var_t1_dn5)) / (2.0 * assign23250_e32073)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * assign23250_e32065) + (assign23250_e32062 * locals.var_t1_dn6)) / (2.0 * assign23250_e32073)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * assign23250_e32065) + (assign23250_e32062 * locals.var_t1_dn7)) / (2.0 * assign23250_e32073)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * assign23250_e32065) + (assign23250_e32062 * locals.var_t1_dn8)) / (2.0 * assign23250_e32073)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * assign23250_e32065) + (assign23250_e32062 * locals.var_t1_dn9)) / (2.0 * assign23250_e32073)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * assign23250_e32065) + (assign23250_e32062 * locals.var_t1_dn10)) / (2.0 * assign23250_e32073)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * assign23250_e32065) + (assign23250_e32062 * locals.var_t1_dn11)) / (2.0 * assign23250_e32073)))), (0.5 * (locals.var_t1_dn12 + (((locals.var_t1_dn12 * assign23250_e32065) + (assign23250_e32062 * locals.var_t1_dn12)) / (2.0 * assign23250_e32073)))), (0.5 * (locals.var_t1_dn13 + (((locals.var_t1_dn13 * assign23250_e32065) + (assign23250_e32062 * locals.var_t1_dn13)) / (2.0 * assign23250_e32073)))), (0.5 * (locals.var_t1_dn14 + (((locals.var_t1_dn14 * assign23250_e32065) + (assign23250_e32062 * locals.var_t1_dn14)) / (2.0 * assign23250_e32073)))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23250_e32077;
        locals.var_t1_dn0 = assign23250_e32077_d_n0;
        locals.var_t1_dn2 = assign23250_e32077_d_n2;
        locals.var_t1_dn3 = assign23250_e32077_d_n3;
        locals.var_t1_dn4 = assign23250_e32077_d_n4;
        locals.var_t1_dn5 = assign23250_e32077_d_n5;
        locals.var_t1_dn6 = assign23250_e32077_d_n6;
        locals.var_t1_dn7 = assign23250_e32077_d_n7;
        locals.var_t1_dn8 = assign23250_e32077_d_n8;
        locals.var_t1_dn9 = assign23250_e32077_d_n9;
        locals.var_t1_dn10 = assign23250_e32077_d_n10;
        locals.var_t1_dn11 = assign23250_e32077_d_n11;
        locals.var_t1_dn12 = assign23250_e32077_d_n12;
        locals.var_t1_dn13 = assign23250_e32077_d_n13;
        locals.var_t1_dn14 = assign23250_e32077_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign23260_e32091, assign23260_e32091_d_n0, assign23260_e32091_d_n2, assign23260_e32091_d_n3, assign23260_e32091_d_n4, assign23260_e32091_d_n5, assign23260_e32091_d_n6, assign23260_e32091_d_n7, assign23260_e32091_d_n8, assign23260_e32091_d_n9, assign23260_e32091_d_n10, assign23260_e32091_d_n11, assign23260_e32091_d_n12, assign23260_e32091_d_n13, assign23260_e32091_d_n14,) = {
    if (locals.var_guard593 != 0.0) {
        let assign23260_e32081: f64 = (10.0 * p.p1106);
        let assign23260_e32083: f64 = (assign23260_e32081 * locals.var_t1);
        let assign23260_e32086: f64 = (10.0 * p.p1106);
        let assign23260_e32088: f64 = (assign23260_e32086 + locals.var_t1);
        let assign23260_e32089: f64 = (assign23260_e32083 / assign23260_e32088);
        (assign23260_e32089, ((((assign23260_e32081 * locals.var_t1_dn0) * assign23260_e32088) - (assign23260_e32083 * locals.var_t1_dn0)) / (assign23260_e32088 * assign23260_e32088)), ((((assign23260_e32081 * locals.var_t1_dn2) * assign23260_e32088) - (assign23260_e32083 * locals.var_t1_dn2)) / (assign23260_e32088 * assign23260_e32088)), ((((assign23260_e32081 * locals.var_t1_dn3) * assign23260_e32088) - (assign23260_e32083 * locals.var_t1_dn3)) / (assign23260_e32088 * assign23260_e32088)), ((((assign23260_e32081 * locals.var_t1_dn4) * assign23260_e32088) - (assign23260_e32083 * locals.var_t1_dn4)) / (assign23260_e32088 * assign23260_e32088)), ((((assign23260_e32081 * locals.var_t1_dn5) * assign23260_e32088) - (assign23260_e32083 * locals.var_t1_dn5)) / (assign23260_e32088 * assign23260_e32088)), ((((assign23260_e32081 * locals.var_t1_dn6) * assign23260_e32088) - (assign23260_e32083 * locals.var_t1_dn6)) / (assign23260_e32088 * assign23260_e32088)), ((((assign23260_e32081 * locals.var_t1_dn7) * assign23260_e32088) - (assign23260_e32083 * locals.var_t1_dn7)) / (assign23260_e32088 * assign23260_e32088)), ((((assign23260_e32081 * locals.var_t1_dn8) * assign23260_e32088) - (assign23260_e32083 * locals.var_t1_dn8)) / (assign23260_e32088 * assign23260_e32088)), ((((assign23260_e32081 * locals.var_t1_dn9) * assign23260_e32088) - (assign23260_e32083 * locals.var_t1_dn9)) / (assign23260_e32088 * assign23260_e32088)), ((((assign23260_e32081 * locals.var_t1_dn10) * assign23260_e32088) - (assign23260_e32083 * locals.var_t1_dn10)) / (assign23260_e32088 * assign23260_e32088)), ((((assign23260_e32081 * locals.var_t1_dn11) * assign23260_e32088) - (assign23260_e32083 * locals.var_t1_dn11)) / (assign23260_e32088 * assign23260_e32088)), ((((assign23260_e32081 * locals.var_t1_dn12) * assign23260_e32088) - (assign23260_e32083 * locals.var_t1_dn12)) / (assign23260_e32088 * assign23260_e32088)), ((((assign23260_e32081 * locals.var_t1_dn13) * assign23260_e32088) - (assign23260_e32083 * locals.var_t1_dn13)) / (assign23260_e32088 * assign23260_e32088)), ((((assign23260_e32081 * locals.var_t1_dn14) * assign23260_e32088) - (assign23260_e32083 * locals.var_t1_dn14)) / (assign23260_e32088 * assign23260_e32088)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign23260_e32091;
        locals.var_t2_dn0 = assign23260_e32091_d_n0;
        locals.var_t2_dn2 = assign23260_e32091_d_n2;
        locals.var_t2_dn3 = assign23260_e32091_d_n3;
        locals.var_t2_dn4 = assign23260_e32091_d_n4;
        locals.var_t2_dn5 = assign23260_e32091_d_n5;
        locals.var_t2_dn6 = assign23260_e32091_d_n6;
        locals.var_t2_dn7 = assign23260_e32091_d_n7;
        locals.var_t2_dn8 = assign23260_e32091_d_n8;
        locals.var_t2_dn9 = assign23260_e32091_d_n9;
        locals.var_t2_dn10 = assign23260_e32091_d_n10;
        locals.var_t2_dn11 = assign23260_e32091_d_n11;
        locals.var_t2_dn12 = assign23260_e32091_d_n12;
        locals.var_t2_dn13 = assign23260_e32091_d_n13;
        locals.var_t2_dn14 = assign23260_e32091_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign23270_e32101, assign23270_e32101_d_n0, assign23270_e32101_d_n2, assign23270_e32101_d_n3, assign23270_e32101_d_n4, assign23270_e32101_d_n5, assign23270_e32101_d_n6, assign23270_e32101_d_n7, assign23270_e32101_d_n8, assign23270_e32101_d_n9, assign23270_e32101_d_n10, assign23270_e32101_d_n11, assign23270_e32101_d_n12, assign23270_e32101_d_n13, assign23270_e32101_d_n14,) = {
    if (locals.var_guard593 != 0.0) {
        let assign23270_e32097: f64 = (p.p1104 * locals.var_t2);
        let assign23270_e32098: f64 = (1.0 + assign23270_e32097);
        let assign23270_e32099: f64 = (locals.var_vdrift_t * assign23270_e32098);
        (assign23270_e32099, (locals.var_vdrift_t * (p.p1104 * locals.var_t2_dn0)), (locals.var_vdrift_t * (p.p1104 * locals.var_t2_dn2)), (locals.var_vdrift_t * (p.p1104 * locals.var_t2_dn3)), ((locals.var_vdrift_t_dn4 * assign23270_e32098) + (locals.var_vdrift_t * (p.p1104 * locals.var_t2_dn4))), (locals.var_vdrift_t * (p.p1104 * locals.var_t2_dn5)), (locals.var_vdrift_t * (p.p1104 * locals.var_t2_dn6)), (locals.var_vdrift_t * (p.p1104 * locals.var_t2_dn7)), (locals.var_vdrift_t * (p.p1104 * locals.var_t2_dn8)), (locals.var_vdrift_t * (p.p1104 * locals.var_t2_dn9)), (locals.var_vdrift_t * (p.p1104 * locals.var_t2_dn10)), (locals.var_vdrift_t * (p.p1104 * locals.var_t2_dn11)), (locals.var_vdrift_t * (p.p1104 * locals.var_t2_dn12)), (locals.var_vdrift_t * (p.p1104 * locals.var_t2_dn13)), (locals.var_vdrift_t * (p.p1104 * locals.var_t2_dn14)),)
    } else {
        (locals.var_vdrift_eff, locals.var_vdrift_eff_dn0, locals.var_vdrift_eff_dn2, locals.var_vdrift_eff_dn3, locals.var_vdrift_eff_dn4, locals.var_vdrift_eff_dn5, locals.var_vdrift_eff_dn6, locals.var_vdrift_eff_dn7, locals.var_vdrift_eff_dn8, locals.var_vdrift_eff_dn9, locals.var_vdrift_eff_dn10, locals.var_vdrift_eff_dn11, locals.var_vdrift_eff_dn12, locals.var_vdrift_eff_dn13, locals.var_vdrift_eff_dn14,)
    }
};
        locals.var_vdrift_eff = assign23270_e32101;
        locals.var_vdrift_eff_dn0 = assign23270_e32101_d_n0;
        locals.var_vdrift_eff_dn2 = assign23270_e32101_d_n2;
        locals.var_vdrift_eff_dn3 = assign23270_e32101_d_n3;
        locals.var_vdrift_eff_dn4 = assign23270_e32101_d_n4;
        locals.var_vdrift_eff_dn5 = assign23270_e32101_d_n5;
        locals.var_vdrift_eff_dn6 = assign23270_e32101_d_n6;
        locals.var_vdrift_eff_dn7 = assign23270_e32101_d_n7;
        locals.var_vdrift_eff_dn8 = assign23270_e32101_d_n8;
        locals.var_vdrift_eff_dn9 = assign23270_e32101_d_n9;
        locals.var_vdrift_eff_dn10 = assign23270_e32101_d_n10;
        locals.var_vdrift_eff_dn11 = assign23270_e32101_d_n11;
        locals.var_vdrift_eff_dn12 = assign23270_e32101_d_n12;
        locals.var_vdrift_eff_dn13 = assign23270_e32101_d_n13;
        locals.var_vdrift_eff_dn14 = assign23270_e32101_d_n14;
        locals.var_vdrift_eff_rv = 0.0;

        let (assign23280_e32115, assign23280_e32115_d_n0, assign23280_e32115_d_n2, assign23280_e32115_d_n3, assign23280_e32115_d_n4, assign23280_e32115_d_n5, assign23280_e32115_d_n6, assign23280_e32115_d_n7, assign23280_e32115_d_n8, assign23280_e32115_d_n9, assign23280_e32115_d_n10, assign23280_e32115_d_n11, assign23280_e32115_d_n12, assign23280_e32115_d_n13, assign23280_e32115_d_n14,) = {
    if (locals.var_guard593 != 0.0) {
        let assign23280_e32105: f64 = (p.p502 * locals.var_ids);
        let assign23280_e32108: f64 = (p.p2 * locals.var_weff);
        let assign23280_e32110: f64 = (assign23280_e32108 * 1.60219e-19);
        let assign23280_e32112: f64 = (assign23280_e32110 * locals.var_vdrift_eff);
        let assign23280_e32113: f64 = (assign23280_e32105 / assign23280_e32112);
        (assign23280_e32113, ((((p.p502 * locals.var_ids_dn0) * assign23280_e32112) - (assign23280_e32105 * (assign23280_e32110 * locals.var_vdrift_eff_dn0))) / (assign23280_e32112 * assign23280_e32112)), ((((p.p502 * locals.var_ids_dn2) * assign23280_e32112) - (assign23280_e32105 * (assign23280_e32110 * locals.var_vdrift_eff_dn2))) / (assign23280_e32112 * assign23280_e32112)), ((((p.p502 * locals.var_ids_dn3) * assign23280_e32112) - (assign23280_e32105 * (assign23280_e32110 * locals.var_vdrift_eff_dn3))) / (assign23280_e32112 * assign23280_e32112)), ((((p.p502 * locals.var_ids_dn4) * assign23280_e32112) - (assign23280_e32105 * (assign23280_e32110 * locals.var_vdrift_eff_dn4))) / (assign23280_e32112 * assign23280_e32112)), ((((p.p502 * locals.var_ids_dn5) * assign23280_e32112) - (assign23280_e32105 * (assign23280_e32110 * locals.var_vdrift_eff_dn5))) / (assign23280_e32112 * assign23280_e32112)), ((((p.p502 * locals.var_ids_dn6) * assign23280_e32112) - (assign23280_e32105 * (assign23280_e32110 * locals.var_vdrift_eff_dn6))) / (assign23280_e32112 * assign23280_e32112)), ((((p.p502 * locals.var_ids_dn7) * assign23280_e32112) - (assign23280_e32105 * (assign23280_e32110 * locals.var_vdrift_eff_dn7))) / (assign23280_e32112 * assign23280_e32112)), ((((p.p502 * locals.var_ids_dn8) * assign23280_e32112) - (assign23280_e32105 * (assign23280_e32110 * locals.var_vdrift_eff_dn8))) / (assign23280_e32112 * assign23280_e32112)), ((((p.p502 * locals.var_ids_dn9) * assign23280_e32112) - (assign23280_e32105 * (assign23280_e32110 * locals.var_vdrift_eff_dn9))) / (assign23280_e32112 * assign23280_e32112)), ((((p.p502 * locals.var_ids_dn10) * assign23280_e32112) - (assign23280_e32105 * (assign23280_e32110 * locals.var_vdrift_eff_dn10))) / (assign23280_e32112 * assign23280_e32112)), ((((p.p502 * locals.var_ids_dn11) * assign23280_e32112) - (assign23280_e32105 * (assign23280_e32110 * locals.var_vdrift_eff_dn11))) / (assign23280_e32112 * assign23280_e32112)), ((((p.p502 * locals.var_ids_dn12) * assign23280_e32112) - (assign23280_e32105 * (assign23280_e32110 * locals.var_vdrift_eff_dn12))) / (assign23280_e32112 * assign23280_e32112)), ((((p.p502 * locals.var_ids_dn13) * assign23280_e32112) - (assign23280_e32105 * (assign23280_e32110 * locals.var_vdrift_eff_dn13))) / (assign23280_e32112 * assign23280_e32112)), ((((p.p502 * locals.var_ids_dn14) * assign23280_e32112) - (assign23280_e32105 * (assign23280_e32110 * locals.var_vdrift_eff_dn14))) / (assign23280_e32112 * assign23280_e32112)),)
    } else {
        (locals.var_ntot, locals.var_ntot_dn0, locals.var_ntot_dn2, locals.var_ntot_dn3, locals.var_ntot_dn4, locals.var_ntot_dn5, locals.var_ntot_dn6, locals.var_ntot_dn7, locals.var_ntot_dn8, locals.var_ntot_dn9, locals.var_ntot_dn10, locals.var_ntot_dn11, locals.var_ntot_dn12, locals.var_ntot_dn13, locals.var_ntot_dn14,)
    }
};
        locals.var_ntot = assign23280_e32115;
        locals.var_ntot_dn0 = assign23280_e32115_d_n0;
        locals.var_ntot_dn2 = assign23280_e32115_d_n2;
        locals.var_ntot_dn3 = assign23280_e32115_d_n3;
        locals.var_ntot_dn4 = assign23280_e32115_d_n4;
        locals.var_ntot_dn5 = assign23280_e32115_d_n5;
        locals.var_ntot_dn6 = assign23280_e32115_d_n6;
        locals.var_ntot_dn7 = assign23280_e32115_d_n7;
        locals.var_ntot_dn8 = assign23280_e32115_d_n8;
        locals.var_ntot_dn9 = assign23280_e32115_d_n9;
        locals.var_ntot_dn10 = assign23280_e32115_d_n10;
        locals.var_ntot_dn11 = assign23280_e32115_d_n11;
        locals.var_ntot_dn12 = assign23280_e32115_d_n12;
        locals.var_ntot_dn13 = assign23280_e32115_d_n13;
        locals.var_ntot_dn14 = assign23280_e32115_d_n14;
        locals.var_ntot_rv = 0.0;

        let (assign23290_e32123, assign23290_e32123_d_n0, assign23290_e32123_d_n2, assign23290_e32123_d_n3, assign23290_e32123_d_n4, assign23290_e32123_d_n5, assign23290_e32123_d_n6, assign23290_e32123_d_n7, assign23290_e32123_d_n8, assign23290_e32123_d_n9, assign23290_e32123_d_n10, assign23290_e32123_d_n11, assign23290_e32123_d_n12, assign23290_e32123_d_n13, assign23290_e32123_d_n14,) = {
    if (locals.var_guard593 != 0.0) {
        let assign23290_e32119: f64 = (locals.var_ntot / p.p1099);
        let assign23290_e32121: f64 = (assign23290_e32119 - 1.0);
        (assign23290_e32121, (locals.var_ntot_dn0 / p.p1099), (locals.var_ntot_dn2 / p.p1099), (locals.var_ntot_dn3 / p.p1099), (locals.var_ntot_dn4 / p.p1099), (locals.var_ntot_dn5 / p.p1099), (locals.var_ntot_dn6 / p.p1099), (locals.var_ntot_dn7 / p.p1099), (locals.var_ntot_dn8 / p.p1099), (locals.var_ntot_dn9 / p.p1099), (locals.var_ntot_dn10 / p.p1099), (locals.var_ntot_dn11 / p.p1099), (locals.var_ntot_dn12 / p.p1099), (locals.var_ntot_dn13 / p.p1099), (locals.var_ntot_dn14 / p.p1099),)
    } else {
        (locals.var_nextra, locals.var_nextra_dn0, locals.var_nextra_dn2, locals.var_nextra_dn3, locals.var_nextra_dn4, locals.var_nextra_dn5, locals.var_nextra_dn6, locals.var_nextra_dn7, locals.var_nextra_dn8, locals.var_nextra_dn9, locals.var_nextra_dn10, locals.var_nextra_dn11, locals.var_nextra_dn12, locals.var_nextra_dn13, locals.var_nextra_dn14,)
    }
};
        locals.var_nextra = assign23290_e32123;
        locals.var_nextra_dn0 = assign23290_e32123_d_n0;
        locals.var_nextra_dn2 = assign23290_e32123_d_n2;
        locals.var_nextra_dn3 = assign23290_e32123_d_n3;
        locals.var_nextra_dn4 = assign23290_e32123_d_n4;
        locals.var_nextra_dn5 = assign23290_e32123_d_n5;
        locals.var_nextra_dn6 = assign23290_e32123_d_n6;
        locals.var_nextra_dn7 = assign23290_e32123_d_n7;
        locals.var_nextra_dn8 = assign23290_e32123_d_n8;
        locals.var_nextra_dn9 = assign23290_e32123_d_n9;
        locals.var_nextra_dn10 = assign23290_e32123_d_n10;
        locals.var_nextra_dn11 = assign23290_e32123_d_n11;
        locals.var_nextra_dn12 = assign23290_e32123_d_n12;
        locals.var_nextra_dn13 = assign23290_e32123_d_n13;
        locals.var_nextra_dn14 = assign23290_e32123_d_n14;
        locals.var_nextra_rv = 0.0;

        let assign23300_e32129: f64 = (-2500.0);
        let assign23300_e32131: f64 = (assign23300_e32129 * p.p504);
        let assign23300_e32133: f64 = if ((0.0 == 0.0) && (locals.var_nextra < assign23300_e32131)) { 1.0 } else { 0.0 };
        locals.var_guard595 = assign23300_e32133;
        locals.var_guard595_rv = 0.0;

        let (assign23310_e32146, assign23310_e32146_d_n0, assign23310_e32146_d_n2, assign23310_e32146_d_n3, assign23310_e32146_d_n4, assign23310_e32146_d_n5, assign23310_e32146_d_n6, assign23310_e32146_d_n7, assign23310_e32146_d_n8, assign23310_e32146_d_n9, assign23310_e32146_d_n10, assign23310_e32146_d_n11, assign23310_e32146_d_n12, assign23310_e32146_d_n13, assign23310_e32146_d_n14,) = {
    if ((locals.var_guard593 != 0.0) && (locals.var_guard595 != 0.0)) {
        let assign23310_e32138: f64 = (-p.p504);
        let assign23310_e32140: f64 = (assign23310_e32138 * p.p504);
        let assign23310_e32143: f64 = (16.0 * locals.var_nextra);
        let assign23310_e32144: f64 = (assign23310_e32140 / assign23310_e32143);
        (assign23310_e32144, (-((assign23310_e32140 * (16.0 * locals.var_nextra_dn0)) / (assign23310_e32143 * assign23310_e32143))), (-((assign23310_e32140 * (16.0 * locals.var_nextra_dn2)) / (assign23310_e32143 * assign23310_e32143))), (-((assign23310_e32140 * (16.0 * locals.var_nextra_dn3)) / (assign23310_e32143 * assign23310_e32143))), (-((assign23310_e32140 * (16.0 * locals.var_nextra_dn4)) / (assign23310_e32143 * assign23310_e32143))), (-((assign23310_e32140 * (16.0 * locals.var_nextra_dn5)) / (assign23310_e32143 * assign23310_e32143))), (-((assign23310_e32140 * (16.0 * locals.var_nextra_dn6)) / (assign23310_e32143 * assign23310_e32143))), (-((assign23310_e32140 * (16.0 * locals.var_nextra_dn7)) / (assign23310_e32143 * assign23310_e32143))), (-((assign23310_e32140 * (16.0 * locals.var_nextra_dn8)) / (assign23310_e32143 * assign23310_e32143))), (-((assign23310_e32140 * (16.0 * locals.var_nextra_dn9)) / (assign23310_e32143 * assign23310_e32143))), (-((assign23310_e32140 * (16.0 * locals.var_nextra_dn10)) / (assign23310_e32143 * assign23310_e32143))), (-((assign23310_e32140 * (16.0 * locals.var_nextra_dn11)) / (assign23310_e32143 * assign23310_e32143))), (-((assign23310_e32140 * (16.0 * locals.var_nextra_dn12)) / (assign23310_e32143 * assign23310_e32143))), (-((assign23310_e32140 * (16.0 * locals.var_nextra_dn13)) / (assign23310_e32143 * assign23310_e32143))), (-((assign23310_e32140 * (16.0 * locals.var_nextra_dn14)) / (assign23310_e32143 * assign23310_e32143))),)
    } else {
        (locals.var_nextra, locals.var_nextra_dn0, locals.var_nextra_dn2, locals.var_nextra_dn3, locals.var_nextra_dn4, locals.var_nextra_dn5, locals.var_nextra_dn6, locals.var_nextra_dn7, locals.var_nextra_dn8, locals.var_nextra_dn9, locals.var_nextra_dn10, locals.var_nextra_dn11, locals.var_nextra_dn12, locals.var_nextra_dn13, locals.var_nextra_dn14,)
    }
};
        locals.var_nextra = assign23310_e32146;
        locals.var_nextra_dn0 = assign23310_e32146_d_n0;
        locals.var_nextra_dn2 = assign23310_e32146_d_n2;
        locals.var_nextra_dn3 = assign23310_e32146_d_n3;
        locals.var_nextra_dn4 = assign23310_e32146_d_n4;
        locals.var_nextra_dn5 = assign23310_e32146_d_n5;
        locals.var_nextra_dn6 = assign23310_e32146_d_n6;
        locals.var_nextra_dn7 = assign23310_e32146_d_n7;
        locals.var_nextra_dn8 = assign23310_e32146_d_n8;
        locals.var_nextra_dn9 = assign23310_e32146_d_n9;
        locals.var_nextra_dn10 = assign23310_e32146_d_n10;
        locals.var_nextra_dn11 = assign23310_e32146_d_n11;
        locals.var_nextra_dn12 = assign23310_e32146_d_n12;
        locals.var_nextra_dn13 = assign23310_e32146_d_n13;
        locals.var_nextra_dn14 = assign23310_e32146_d_n14;
        locals.var_nextra_rv = 0.0;

        let (assign23320_e32172, assign23320_e32172_d_n0, assign23320_e32172_d_n2, assign23320_e32172_d_n3, assign23320_e32172_d_n4, assign23320_e32172_d_n5, assign23320_e32172_d_n6, assign23320_e32172_d_n7, assign23320_e32172_d_n8, assign23320_e32172_d_n9, assign23320_e32172_d_n10, assign23320_e32172_d_n11, assign23320_e32172_d_n12, assign23320_e32172_d_n13, assign23320_e32172_d_n14,) = {
    if ((locals.var_guard593 != 0.0) && (locals.var_guard595 == 0.0)) {
        let assign23320_e32154: f64 = locals.var_nextra;
        let assign23320_e32157: f64 = locals.var_nextra;
        let assign23320_e32160: f64 = locals.var_nextra;
        let assign23320_e32161: f64 = (assign23320_e32157 * assign23320_e32160);
        let assign23320_e32164: f64 = (0.25 * p.p504);
        let assign23320_e32166: f64 = (assign23320_e32164 * p.p504);
        let assign23320_e32167: f64 = (assign23320_e32161 + assign23320_e32166);
        let assign23320_e32168: f64 = (assign23320_e32167).sqrt();
        let assign23320_e32169: f64 = (assign23320_e32154 + assign23320_e32168);
        let assign23320_e32170: f64 = (0.5 * assign23320_e32169);
        (assign23320_e32170, (0.5 * (locals.var_nextra_dn0 + (((locals.var_nextra_dn0 * assign23320_e32160) + (assign23320_e32157 * locals.var_nextra_dn0)) / (2.0 * assign23320_e32168)))), (0.5 * (locals.var_nextra_dn2 + (((locals.var_nextra_dn2 * assign23320_e32160) + (assign23320_e32157 * locals.var_nextra_dn2)) / (2.0 * assign23320_e32168)))), (0.5 * (locals.var_nextra_dn3 + (((locals.var_nextra_dn3 * assign23320_e32160) + (assign23320_e32157 * locals.var_nextra_dn3)) / (2.0 * assign23320_e32168)))), (0.5 * (locals.var_nextra_dn4 + (((locals.var_nextra_dn4 * assign23320_e32160) + (assign23320_e32157 * locals.var_nextra_dn4)) / (2.0 * assign23320_e32168)))), (0.5 * (locals.var_nextra_dn5 + (((locals.var_nextra_dn5 * assign23320_e32160) + (assign23320_e32157 * locals.var_nextra_dn5)) / (2.0 * assign23320_e32168)))), (0.5 * (locals.var_nextra_dn6 + (((locals.var_nextra_dn6 * assign23320_e32160) + (assign23320_e32157 * locals.var_nextra_dn6)) / (2.0 * assign23320_e32168)))), (0.5 * (locals.var_nextra_dn7 + (((locals.var_nextra_dn7 * assign23320_e32160) + (assign23320_e32157 * locals.var_nextra_dn7)) / (2.0 * assign23320_e32168)))), (0.5 * (locals.var_nextra_dn8 + (((locals.var_nextra_dn8 * assign23320_e32160) + (assign23320_e32157 * locals.var_nextra_dn8)) / (2.0 * assign23320_e32168)))), (0.5 * (locals.var_nextra_dn9 + (((locals.var_nextra_dn9 * assign23320_e32160) + (assign23320_e32157 * locals.var_nextra_dn9)) / (2.0 * assign23320_e32168)))), (0.5 * (locals.var_nextra_dn10 + (((locals.var_nextra_dn10 * assign23320_e32160) + (assign23320_e32157 * locals.var_nextra_dn10)) / (2.0 * assign23320_e32168)))), (0.5 * (locals.var_nextra_dn11 + (((locals.var_nextra_dn11 * assign23320_e32160) + (assign23320_e32157 * locals.var_nextra_dn11)) / (2.0 * assign23320_e32168)))), (0.5 * (locals.var_nextra_dn12 + (((locals.var_nextra_dn12 * assign23320_e32160) + (assign23320_e32157 * locals.var_nextra_dn12)) / (2.0 * assign23320_e32168)))), (0.5 * (locals.var_nextra_dn13 + (((locals.var_nextra_dn13 * assign23320_e32160) + (assign23320_e32157 * locals.var_nextra_dn13)) / (2.0 * assign23320_e32168)))), (0.5 * (locals.var_nextra_dn14 + (((locals.var_nextra_dn14 * assign23320_e32160) + (assign23320_e32157 * locals.var_nextra_dn14)) / (2.0 * assign23320_e32168)))),)
    } else {
        (locals.var_nextra, locals.var_nextra_dn0, locals.var_nextra_dn2, locals.var_nextra_dn3, locals.var_nextra_dn4, locals.var_nextra_dn5, locals.var_nextra_dn6, locals.var_nextra_dn7, locals.var_nextra_dn8, locals.var_nextra_dn9, locals.var_nextra_dn10, locals.var_nextra_dn11, locals.var_nextra_dn12, locals.var_nextra_dn13, locals.var_nextra_dn14,)
    }
};
        locals.var_nextra = assign23320_e32172;
        locals.var_nextra_dn0 = assign23320_e32172_d_n0;
        locals.var_nextra_dn2 = assign23320_e32172_d_n2;
        locals.var_nextra_dn3 = assign23320_e32172_d_n3;
        locals.var_nextra_dn4 = assign23320_e32172_d_n4;
        locals.var_nextra_dn5 = assign23320_e32172_d_n5;
        locals.var_nextra_dn6 = assign23320_e32172_d_n6;
        locals.var_nextra_dn7 = assign23320_e32172_d_n7;
        locals.var_nextra_dn8 = assign23320_e32172_d_n8;
        locals.var_nextra_dn9 = assign23320_e32172_d_n9;
        locals.var_nextra_dn10 = assign23320_e32172_d_n10;
        locals.var_nextra_dn11 = assign23320_e32172_d_n11;
        locals.var_nextra_dn12 = assign23320_e32172_d_n12;
        locals.var_nextra_dn13 = assign23320_e32172_d_n13;
        locals.var_nextra_dn14 = assign23320_e32172_d_n14;
        locals.var_nextra_rv = 0.0;

        let (assign23330_e32178, assign23330_e32178_d_n0, assign23330_e32178_d_n2, assign23330_e32178_d_n3, assign23330_e32178_d_n4, assign23330_e32178_d_n5, assign23330_e32178_d_n6, assign23330_e32178_d_n7, assign23330_e32178_d_n8, assign23330_e32178_d_n9, assign23330_e32178_d_n10, assign23330_e32178_d_n11, assign23330_e32178_d_n12, assign23330_e32178_d_n13, assign23330_e32178_d_n14,) = {
    if (locals.var_guard593 != 0.0) {
        let assign23330_e32176: f64 = (p.p1099 * locals.var_nextra);
        (assign23330_e32176, (p.p1099 * locals.var_nextra_dn0), (p.p1099 * locals.var_nextra_dn2), (p.p1099 * locals.var_nextra_dn3), (p.p1099 * locals.var_nextra_dn4), (p.p1099 * locals.var_nextra_dn5), (p.p1099 * locals.var_nextra_dn6), (p.p1099 * locals.var_nextra_dn7), (p.p1099 * locals.var_nextra_dn8), (p.p1099 * locals.var_nextra_dn9), (p.p1099 * locals.var_nextra_dn10), (p.p1099 * locals.var_nextra_dn11), (p.p1099 * locals.var_nextra_dn12), (p.p1099 * locals.var_nextra_dn13), (p.p1099 * locals.var_nextra_dn14),)
    } else {
        (locals.var_nextra, locals.var_nextra_dn0, locals.var_nextra_dn2, locals.var_nextra_dn3, locals.var_nextra_dn4, locals.var_nextra_dn5, locals.var_nextra_dn6, locals.var_nextra_dn7, locals.var_nextra_dn8, locals.var_nextra_dn9, locals.var_nextra_dn10, locals.var_nextra_dn11, locals.var_nextra_dn12, locals.var_nextra_dn13, locals.var_nextra_dn14,)
    }
};
        locals.var_nextra = assign23330_e32178;
        locals.var_nextra_dn0 = assign23330_e32178_d_n0;
        locals.var_nextra_dn2 = assign23330_e32178_d_n2;
        locals.var_nextra_dn3 = assign23330_e32178_d_n3;
        locals.var_nextra_dn4 = assign23330_e32178_d_n4;
        locals.var_nextra_dn5 = assign23330_e32178_d_n5;
        locals.var_nextra_dn6 = assign23330_e32178_d_n6;
        locals.var_nextra_dn7 = assign23330_e32178_d_n7;
        locals.var_nextra_dn8 = assign23330_e32178_d_n8;
        locals.var_nextra_dn9 = assign23330_e32178_d_n9;
        locals.var_nextra_dn10 = assign23330_e32178_d_n10;
        locals.var_nextra_dn11 = assign23330_e32178_d_n11;
        locals.var_nextra_dn12 = assign23330_e32178_d_n12;
        locals.var_nextra_dn13 = assign23330_e32178_d_n13;
        locals.var_nextra_dn14 = assign23330_e32178_d_n14;
        locals.var_nextra_rv = 0.0;

        let assign23340_e32181: f64 = if p.p514 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard596 = assign23340_e32181;
        locals.var_guard596_rv = 0.0;

        let assign23350_e32187: f64 = (locals.var_devsign * (nv0 - nv2));
        let assign23350_e32190: f64 = (p.p512 * locals.var_vdseffii);
        let assign23350_e32191: f64 = (assign23350_e32187 - assign23350_e32190);
        let assign23350_e32193: f64 = (assign23350_e32191 - p.p503);
        let assign23350_e32197: f64 = (locals.var_vb_cm).powf(p.p513);
        let assign23350_e32198: f64 = (p.p514 * assign23350_e32197);
        let assign23350_e32199: f64 = (assign23350_e32193 - assign23350_e32198);
        let assign23350_e32201: f64 = (-2500.0);
        let assign23350_e32203: f64 = (assign23350_e32201 * 0.05);
        let assign23350_e32205: f64 = if ((0.0 == 0.0) && (assign23350_e32199 < assign23350_e32203)) { 1.0 } else { 0.0 };
        locals.var_guard597 = assign23350_e32205;
        locals.var_guard597_rv = 0.0;

        let (assign23360_e32234, assign23360_e32234_d_n0, assign23360_e32234_d_n2, assign23360_e32234_d_n3, assign23360_e32234_d_n4, assign23360_e32234_d_n5, assign23360_e32234_d_n6, assign23360_e32234_d_n7, assign23360_e32234_d_n8, assign23360_e32234_d_n9, assign23360_e32234_d_n10, assign23360_e32234_d_n11, assign23360_e32234_d_n12, assign23360_e32234_d_n13, assign23360_e32234_d_n14,) = {
    if (((locals.var_guard593 != 0.0) && (locals.var_guard596 != 0.0)) && (locals.var_guard597 != 0.0)) {
        let assign23360_e32212: f64 = (-0.05);
        let assign23360_e32214: f64 = (assign23360_e32212 * 0.05);
        let assign23360_e32218: f64 = (locals.var_devsign * (nv0 - nv2));
        let assign23360_e32221: f64 = (p.p512 * locals.var_vdseffii);
        let assign23360_e32222: f64 = (assign23360_e32218 - assign23360_e32221);
        let assign23360_e32224: f64 = (assign23360_e32222 - p.p503);
        let assign23360_e32228: f64 = (locals.var_vb_cm).powf(p.p513);
        let assign23360_e32229: f64 = (p.p514 * assign23360_e32228);
        let assign23360_e32230: f64 = (assign23360_e32224 - assign23360_e32229);
        let assign23360_e32231: f64 = (16.0 * assign23360_e32230);
        let assign23360_e32232: f64 = (assign23360_e32214 / assign23360_e32231);
        (assign23360_e32232, (-((assign23360_e32214 * (16.0 * (locals.var_devsign - (p.p512 * locals.var_vdseffii_dn0)))) / (assign23360_e32231 * assign23360_e32231))), (-((assign23360_e32214 * (16.0 * ((-locals.var_devsign) - (p.p512 * locals.var_vdseffii_dn2)))) / (assign23360_e32231 * assign23360_e32231))), (-((assign23360_e32214 * (16.0 * ((-(p.p512 * locals.var_vdseffii_dn3)) - (p.p514 * if 0.0 == 0.0 && ((p.p513) as f64).is_finite() && ((p.p513) as f64).fract() == 0.0 { if p.p513 == 0.0 { 0.0 } else { (p.p513 * ((locals.var_vb_cm).powf(p.p513 - 1.0) * locals.var_vb_cm_dn3)) } } else { (assign23360_e32228 * (p.p513 * (locals.var_vb_cm_dn3 / locals.var_vb_cm))) })))) / (assign23360_e32231 * assign23360_e32231))), (-((assign23360_e32214 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn4)))) / (assign23360_e32231 * assign23360_e32231))), (-((assign23360_e32214 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn5)))) / (assign23360_e32231 * assign23360_e32231))), (-((assign23360_e32214 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn6)))) / (assign23360_e32231 * assign23360_e32231))), (-((assign23360_e32214 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn7)))) / (assign23360_e32231 * assign23360_e32231))), (-((assign23360_e32214 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn8)))) / (assign23360_e32231 * assign23360_e32231))), (-((assign23360_e32214 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn9)))) / (assign23360_e32231 * assign23360_e32231))), (-((assign23360_e32214 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn10)))) / (assign23360_e32231 * assign23360_e32231))), (-((assign23360_e32214 * (16.0 * ((-(p.p512 * locals.var_vdseffii_dn11)) - (p.p514 * if 0.0 == 0.0 && ((p.p513) as f64).is_finite() && ((p.p513) as f64).fract() == 0.0 { if p.p513 == 0.0 { 0.0 } else { (p.p513 * ((locals.var_vb_cm).powf(p.p513 - 1.0) * locals.var_vb_cm_dn11)) } } else { (assign23360_e32228 * (p.p513 * (locals.var_vb_cm_dn11 / locals.var_vb_cm))) })))) / (assign23360_e32231 * assign23360_e32231))), (-((assign23360_e32214 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn12)))) / (assign23360_e32231 * assign23360_e32231))), (-((assign23360_e32214 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn13)))) / (assign23360_e32231 * assign23360_e32231))), (-((assign23360_e32214 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn14)))) / (assign23360_e32231 * assign23360_e32231))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign23360_e32234;
        locals.var_t2_dn0 = assign23360_e32234_d_n0;
        locals.var_t2_dn2 = assign23360_e32234_d_n2;
        locals.var_t2_dn3 = assign23360_e32234_d_n3;
        locals.var_t2_dn4 = assign23360_e32234_d_n4;
        locals.var_t2_dn5 = assign23360_e32234_d_n5;
        locals.var_t2_dn6 = assign23360_e32234_d_n6;
        locals.var_t2_dn7 = assign23360_e32234_d_n7;
        locals.var_t2_dn8 = assign23360_e32234_d_n8;
        locals.var_t2_dn9 = assign23360_e32234_d_n9;
        locals.var_t2_dn10 = assign23360_e32234_d_n10;
        locals.var_t2_dn11 = assign23360_e32234_d_n11;
        locals.var_t2_dn12 = assign23360_e32234_d_n12;
        locals.var_t2_dn13 = assign23360_e32234_d_n13;
        locals.var_t2_dn14 = assign23360_e32234_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign23370_e32304, assign23370_e32304_d_n0, assign23370_e32304_d_n2, assign23370_e32304_d_n3, assign23370_e32304_d_n4, assign23370_e32304_d_n5, assign23370_e32304_d_n6, assign23370_e32304_d_n7, assign23370_e32304_d_n8, assign23370_e32304_d_n9, assign23370_e32304_d_n10, assign23370_e32304_d_n11, assign23370_e32304_d_n12, assign23370_e32304_d_n13, assign23370_e32304_d_n14,) = {
    if (((locals.var_guard593 != 0.0) && (locals.var_guard596 != 0.0)) && (locals.var_guard597 == 0.0)) {
        let assign23370_e32244: f64 = (locals.var_devsign * (nv0 - nv2));
        let assign23370_e32247: f64 = (p.p512 * locals.var_vdseffii);
        let assign23370_e32248: f64 = (assign23370_e32244 - assign23370_e32247);
        let assign23370_e32250: f64 = (assign23370_e32248 - p.p503);
        let assign23370_e32254: f64 = (locals.var_vb_cm).powf(p.p513);
        let assign23370_e32255: f64 = (p.p514 * assign23370_e32254);
        let assign23370_e32256: f64 = (assign23370_e32250 - assign23370_e32255);
        let assign23370_e32258: f64 = assign23370_e32256;
        let assign23370_e32261: f64 = (locals.var_devsign * (nv0 - nv2));
        let assign23370_e32264: f64 = (p.p512 * locals.var_vdseffii);
        let assign23370_e32265: f64 = (assign23370_e32261 - assign23370_e32264);
        let assign23370_e32267: f64 = (assign23370_e32265 - p.p503);
        let assign23370_e32271: f64 = (locals.var_vb_cm).powf(p.p513);
        let assign23370_e32272: f64 = (p.p514 * assign23370_e32271);
        let assign23370_e32273: f64 = (assign23370_e32267 - assign23370_e32272);
        let assign23370_e32275: f64 = assign23370_e32273;
        let assign23370_e32278: f64 = (locals.var_devsign * (nv0 - nv2));
        let assign23370_e32281: f64 = (p.p512 * locals.var_vdseffii);
        let assign23370_e32282: f64 = (assign23370_e32278 - assign23370_e32281);
        let assign23370_e32284: f64 = (assign23370_e32282 - p.p503);
        let assign23370_e32288: f64 = (locals.var_vb_cm).powf(p.p513);
        let assign23370_e32289: f64 = (p.p514 * assign23370_e32288);
        let assign23370_e32290: f64 = (assign23370_e32284 - assign23370_e32289);
        let assign23370_e32292: f64 = assign23370_e32290;
        let assign23370_e32293: f64 = (assign23370_e32275 * assign23370_e32292);
        let assign23370_e32296: f64 = (0.25 * 0.05);
        let assign23370_e32298: f64 = (assign23370_e32296 * 0.05);
        let assign23370_e32299: f64 = (assign23370_e32293 + assign23370_e32298);
        let assign23370_e32300: f64 = (assign23370_e32299).sqrt();
        let assign23370_e32301: f64 = (assign23370_e32258 + assign23370_e32300);
        let assign23370_e32302: f64 = (0.5 * assign23370_e32301);
        (assign23370_e32302, (0.5 * ((locals.var_devsign - (p.p512 * locals.var_vdseffii_dn0)) + ((((locals.var_devsign - (p.p512 * locals.var_vdseffii_dn0)) * assign23370_e32292) + (assign23370_e32275 * (locals.var_devsign - (p.p512 * locals.var_vdseffii_dn0)))) / (2.0 * assign23370_e32300)))), (0.5 * (((-locals.var_devsign) - (p.p512 * locals.var_vdseffii_dn2)) + (((((-locals.var_devsign) - (p.p512 * locals.var_vdseffii_dn2)) * assign23370_e32292) + (assign23370_e32275 * ((-locals.var_devsign) - (p.p512 * locals.var_vdseffii_dn2)))) / (2.0 * assign23370_e32300)))), (0.5 * (((-(p.p512 * locals.var_vdseffii_dn3)) - (p.p514 * if 0.0 == 0.0 && ((p.p513) as f64).is_finite() && ((p.p513) as f64).fract() == 0.0 { if p.p513 == 0.0 { 0.0 } else { (p.p513 * ((locals.var_vb_cm).powf(p.p513 - 1.0) * locals.var_vb_cm_dn3)) } } else { (assign23370_e32254 * (p.p513 * (locals.var_vb_cm_dn3 / locals.var_vb_cm))) })) + (((((-(p.p512 * locals.var_vdseffii_dn3)) - (p.p514 * if 0.0 == 0.0 && ((p.p513) as f64).is_finite() && ((p.p513) as f64).fract() == 0.0 { if p.p513 == 0.0 { 0.0 } else { (p.p513 * ((locals.var_vb_cm).powf(p.p513 - 1.0) * locals.var_vb_cm_dn3)) } } else { (assign23370_e32271 * (p.p513 * (locals.var_vb_cm_dn3 / locals.var_vb_cm))) })) * assign23370_e32292) + (assign23370_e32275 * ((-(p.p512 * locals.var_vdseffii_dn3)) - (p.p514 * if 0.0 == 0.0 && ((p.p513) as f64).is_finite() && ((p.p513) as f64).fract() == 0.0 { if p.p513 == 0.0 { 0.0 } else { (p.p513 * ((locals.var_vb_cm).powf(p.p513 - 1.0) * locals.var_vb_cm_dn3)) } } else { (assign23370_e32288 * (p.p513 * (locals.var_vb_cm_dn3 / locals.var_vb_cm))) })))) / (2.0 * assign23370_e32300)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn4)) + ((((-(p.p512 * locals.var_vdseffii_dn4)) * assign23370_e32292) + (assign23370_e32275 * (-(p.p512 * locals.var_vdseffii_dn4)))) / (2.0 * assign23370_e32300)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn5)) + ((((-(p.p512 * locals.var_vdseffii_dn5)) * assign23370_e32292) + (assign23370_e32275 * (-(p.p512 * locals.var_vdseffii_dn5)))) / (2.0 * assign23370_e32300)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn6)) + ((((-(p.p512 * locals.var_vdseffii_dn6)) * assign23370_e32292) + (assign23370_e32275 * (-(p.p512 * locals.var_vdseffii_dn6)))) / (2.0 * assign23370_e32300)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn7)) + ((((-(p.p512 * locals.var_vdseffii_dn7)) * assign23370_e32292) + (assign23370_e32275 * (-(p.p512 * locals.var_vdseffii_dn7)))) / (2.0 * assign23370_e32300)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn8)) + ((((-(p.p512 * locals.var_vdseffii_dn8)) * assign23370_e32292) + (assign23370_e32275 * (-(p.p512 * locals.var_vdseffii_dn8)))) / (2.0 * assign23370_e32300)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn9)) + ((((-(p.p512 * locals.var_vdseffii_dn9)) * assign23370_e32292) + (assign23370_e32275 * (-(p.p512 * locals.var_vdseffii_dn9)))) / (2.0 * assign23370_e32300)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn10)) + ((((-(p.p512 * locals.var_vdseffii_dn10)) * assign23370_e32292) + (assign23370_e32275 * (-(p.p512 * locals.var_vdseffii_dn10)))) / (2.0 * assign23370_e32300)))), (0.5 * (((-(p.p512 * locals.var_vdseffii_dn11)) - (p.p514 * if 0.0 == 0.0 && ((p.p513) as f64).is_finite() && ((p.p513) as f64).fract() == 0.0 { if p.p513 == 0.0 { 0.0 } else { (p.p513 * ((locals.var_vb_cm).powf(p.p513 - 1.0) * locals.var_vb_cm_dn11)) } } else { (assign23370_e32254 * (p.p513 * (locals.var_vb_cm_dn11 / locals.var_vb_cm))) })) + (((((-(p.p512 * locals.var_vdseffii_dn11)) - (p.p514 * if 0.0 == 0.0 && ((p.p513) as f64).is_finite() && ((p.p513) as f64).fract() == 0.0 { if p.p513 == 0.0 { 0.0 } else { (p.p513 * ((locals.var_vb_cm).powf(p.p513 - 1.0) * locals.var_vb_cm_dn11)) } } else { (assign23370_e32271 * (p.p513 * (locals.var_vb_cm_dn11 / locals.var_vb_cm))) })) * assign23370_e32292) + (assign23370_e32275 * ((-(p.p512 * locals.var_vdseffii_dn11)) - (p.p514 * if 0.0 == 0.0 && ((p.p513) as f64).is_finite() && ((p.p513) as f64).fract() == 0.0 { if p.p513 == 0.0 { 0.0 } else { (p.p513 * ((locals.var_vb_cm).powf(p.p513 - 1.0) * locals.var_vb_cm_dn11)) } } else { (assign23370_e32288 * (p.p513 * (locals.var_vb_cm_dn11 / locals.var_vb_cm))) })))) / (2.0 * assign23370_e32300)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn12)) + ((((-(p.p512 * locals.var_vdseffii_dn12)) * assign23370_e32292) + (assign23370_e32275 * (-(p.p512 * locals.var_vdseffii_dn12)))) / (2.0 * assign23370_e32300)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn13)) + ((((-(p.p512 * locals.var_vdseffii_dn13)) * assign23370_e32292) + (assign23370_e32275 * (-(p.p512 * locals.var_vdseffii_dn13)))) / (2.0 * assign23370_e32300)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn14)) + ((((-(p.p512 * locals.var_vdseffii_dn14)) * assign23370_e32292) + (assign23370_e32275 * (-(p.p512 * locals.var_vdseffii_dn14)))) / (2.0 * assign23370_e32300)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign23370_e32304;
        locals.var_t2_dn0 = assign23370_e32304_d_n0;
        locals.var_t2_dn2 = assign23370_e32304_d_n2;
        locals.var_t2_dn3 = assign23370_e32304_d_n3;
        locals.var_t2_dn4 = assign23370_e32304_d_n4;
        locals.var_t2_dn5 = assign23370_e32304_d_n5;
        locals.var_t2_dn6 = assign23370_e32304_d_n6;
        locals.var_t2_dn7 = assign23370_e32304_d_n7;
        locals.var_t2_dn8 = assign23370_e32304_d_n8;
        locals.var_t2_dn9 = assign23370_e32304_d_n9;
        locals.var_t2_dn10 = assign23370_e32304_d_n10;
        locals.var_t2_dn11 = assign23370_e32304_d_n11;
        locals.var_t2_dn12 = assign23370_e32304_d_n12;
        locals.var_t2_dn13 = assign23370_e32304_d_n13;
        locals.var_t2_dn14 = assign23370_e32304_d_n14;
        locals.var_t2_rv = 0.0;

        let assign23380_e32310: f64 = (locals.var_devsign * (nv0 - nv2));
        let assign23380_e32313: f64 = (p.p512 * locals.var_vdseffii);
        let assign23380_e32314: f64 = (assign23380_e32310 - assign23380_e32313);
        let assign23380_e32316: f64 = (assign23380_e32314 - p.p503);
        let assign23380_e32318: f64 = (-2500.0);
        let assign23380_e32320: f64 = (assign23380_e32318 * 0.05);
        let assign23380_e32322: f64 = if ((0.0 == 0.0) && (assign23380_e32316 < assign23380_e32320)) { 1.0 } else { 0.0 };
        locals.var_guard598 = assign23380_e32322;
        locals.var_guard598_rv = 0.0;

        let (assign23390_e32346, assign23390_e32346_d_n0, assign23390_e32346_d_n2, assign23390_e32346_d_n3, assign23390_e32346_d_n4, assign23390_e32346_d_n5, assign23390_e32346_d_n6, assign23390_e32346_d_n7, assign23390_e32346_d_n8, assign23390_e32346_d_n9, assign23390_e32346_d_n10, assign23390_e32346_d_n11, assign23390_e32346_d_n12, assign23390_e32346_d_n13, assign23390_e32346_d_n14,) = {
    if (((locals.var_guard593 != 0.0) && (locals.var_guard596 == 0.0)) && (locals.var_guard598 != 0.0)) {
        let assign23390_e32330: f64 = (-0.05);
        let assign23390_e32332: f64 = (assign23390_e32330 * 0.05);
        let assign23390_e32336: f64 = (locals.var_devsign * (nv0 - nv2));
        let assign23390_e32339: f64 = (p.p512 * locals.var_vdseffii);
        let assign23390_e32340: f64 = (assign23390_e32336 - assign23390_e32339);
        let assign23390_e32342: f64 = (assign23390_e32340 - p.p503);
        let assign23390_e32343: f64 = (16.0 * assign23390_e32342);
        let assign23390_e32344: f64 = (assign23390_e32332 / assign23390_e32343);
        (assign23390_e32344, (-((assign23390_e32332 * (16.0 * (locals.var_devsign - (p.p512 * locals.var_vdseffii_dn0)))) / (assign23390_e32343 * assign23390_e32343))), (-((assign23390_e32332 * (16.0 * ((-locals.var_devsign) - (p.p512 * locals.var_vdseffii_dn2)))) / (assign23390_e32343 * assign23390_e32343))), (-((assign23390_e32332 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn3)))) / (assign23390_e32343 * assign23390_e32343))), (-((assign23390_e32332 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn4)))) / (assign23390_e32343 * assign23390_e32343))), (-((assign23390_e32332 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn5)))) / (assign23390_e32343 * assign23390_e32343))), (-((assign23390_e32332 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn6)))) / (assign23390_e32343 * assign23390_e32343))), (-((assign23390_e32332 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn7)))) / (assign23390_e32343 * assign23390_e32343))), (-((assign23390_e32332 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn8)))) / (assign23390_e32343 * assign23390_e32343))), (-((assign23390_e32332 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn9)))) / (assign23390_e32343 * assign23390_e32343))), (-((assign23390_e32332 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn10)))) / (assign23390_e32343 * assign23390_e32343))), (-((assign23390_e32332 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn11)))) / (assign23390_e32343 * assign23390_e32343))), (-((assign23390_e32332 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn12)))) / (assign23390_e32343 * assign23390_e32343))), (-((assign23390_e32332 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn13)))) / (assign23390_e32343 * assign23390_e32343))), (-((assign23390_e32332 * (16.0 * (-(p.p512 * locals.var_vdseffii_dn14)))) / (assign23390_e32343 * assign23390_e32343))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign23390_e32346;
        locals.var_t2_dn0 = assign23390_e32346_d_n0;
        locals.var_t2_dn2 = assign23390_e32346_d_n2;
        locals.var_t2_dn3 = assign23390_e32346_d_n3;
        locals.var_t2_dn4 = assign23390_e32346_d_n4;
        locals.var_t2_dn5 = assign23390_e32346_d_n5;
        locals.var_t2_dn6 = assign23390_e32346_d_n6;
        locals.var_t2_dn7 = assign23390_e32346_d_n7;
        locals.var_t2_dn8 = assign23390_e32346_d_n8;
        locals.var_t2_dn9 = assign23390_e32346_d_n9;
        locals.var_t2_dn10 = assign23390_e32346_d_n10;
        locals.var_t2_dn11 = assign23390_e32346_d_n11;
        locals.var_t2_dn12 = assign23390_e32346_d_n12;
        locals.var_t2_dn13 = assign23390_e32346_d_n13;
        locals.var_t2_dn14 = assign23390_e32346_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign23400_e32399, assign23400_e32399_d_n0, assign23400_e32399_d_n2, assign23400_e32399_d_n3, assign23400_e32399_d_n4, assign23400_e32399_d_n5, assign23400_e32399_d_n6, assign23400_e32399_d_n7, assign23400_e32399_d_n8, assign23400_e32399_d_n9, assign23400_e32399_d_n10, assign23400_e32399_d_n11, assign23400_e32399_d_n12, assign23400_e32399_d_n13, assign23400_e32399_d_n14,) = {
    if (((locals.var_guard593 != 0.0) && (locals.var_guard596 == 0.0)) && (locals.var_guard598 == 0.0)) {
        let assign23400_e32357: f64 = (locals.var_devsign * (nv0 - nv2));
        let assign23400_e32360: f64 = (p.p512 * locals.var_vdseffii);
        let assign23400_e32361: f64 = (assign23400_e32357 - assign23400_e32360);
        let assign23400_e32363: f64 = (assign23400_e32361 - p.p503);
        let assign23400_e32365: f64 = assign23400_e32363;
        let assign23400_e32368: f64 = (locals.var_devsign * (nv0 - nv2));
        let assign23400_e32371: f64 = (p.p512 * locals.var_vdseffii);
        let assign23400_e32372: f64 = (assign23400_e32368 - assign23400_e32371);
        let assign23400_e32374: f64 = (assign23400_e32372 - p.p503);
        let assign23400_e32376: f64 = assign23400_e32374;
        let assign23400_e32379: f64 = (locals.var_devsign * (nv0 - nv2));
        let assign23400_e32382: f64 = (p.p512 * locals.var_vdseffii);
        let assign23400_e32383: f64 = (assign23400_e32379 - assign23400_e32382);
        let assign23400_e32385: f64 = (assign23400_e32383 - p.p503);
        let assign23400_e32387: f64 = assign23400_e32385;
        let assign23400_e32388: f64 = (assign23400_e32376 * assign23400_e32387);
        let assign23400_e32391: f64 = (0.25 * 0.05);
        let assign23400_e32393: f64 = (assign23400_e32391 * 0.05);
        let assign23400_e32394: f64 = (assign23400_e32388 + assign23400_e32393);
        let assign23400_e32395: f64 = (assign23400_e32394).sqrt();
        let assign23400_e32396: f64 = (assign23400_e32365 + assign23400_e32395);
        let assign23400_e32397: f64 = (0.5 * assign23400_e32396);
        (assign23400_e32397, (0.5 * ((locals.var_devsign - (p.p512 * locals.var_vdseffii_dn0)) + ((((locals.var_devsign - (p.p512 * locals.var_vdseffii_dn0)) * assign23400_e32387) + (assign23400_e32376 * (locals.var_devsign - (p.p512 * locals.var_vdseffii_dn0)))) / (2.0 * assign23400_e32395)))), (0.5 * (((-locals.var_devsign) - (p.p512 * locals.var_vdseffii_dn2)) + (((((-locals.var_devsign) - (p.p512 * locals.var_vdseffii_dn2)) * assign23400_e32387) + (assign23400_e32376 * ((-locals.var_devsign) - (p.p512 * locals.var_vdseffii_dn2)))) / (2.0 * assign23400_e32395)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn3)) + ((((-(p.p512 * locals.var_vdseffii_dn3)) * assign23400_e32387) + (assign23400_e32376 * (-(p.p512 * locals.var_vdseffii_dn3)))) / (2.0 * assign23400_e32395)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn4)) + ((((-(p.p512 * locals.var_vdseffii_dn4)) * assign23400_e32387) + (assign23400_e32376 * (-(p.p512 * locals.var_vdseffii_dn4)))) / (2.0 * assign23400_e32395)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn5)) + ((((-(p.p512 * locals.var_vdseffii_dn5)) * assign23400_e32387) + (assign23400_e32376 * (-(p.p512 * locals.var_vdseffii_dn5)))) / (2.0 * assign23400_e32395)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn6)) + ((((-(p.p512 * locals.var_vdseffii_dn6)) * assign23400_e32387) + (assign23400_e32376 * (-(p.p512 * locals.var_vdseffii_dn6)))) / (2.0 * assign23400_e32395)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn7)) + ((((-(p.p512 * locals.var_vdseffii_dn7)) * assign23400_e32387) + (assign23400_e32376 * (-(p.p512 * locals.var_vdseffii_dn7)))) / (2.0 * assign23400_e32395)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn8)) + ((((-(p.p512 * locals.var_vdseffii_dn8)) * assign23400_e32387) + (assign23400_e32376 * (-(p.p512 * locals.var_vdseffii_dn8)))) / (2.0 * assign23400_e32395)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn9)) + ((((-(p.p512 * locals.var_vdseffii_dn9)) * assign23400_e32387) + (assign23400_e32376 * (-(p.p512 * locals.var_vdseffii_dn9)))) / (2.0 * assign23400_e32395)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn10)) + ((((-(p.p512 * locals.var_vdseffii_dn10)) * assign23400_e32387) + (assign23400_e32376 * (-(p.p512 * locals.var_vdseffii_dn10)))) / (2.0 * assign23400_e32395)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn11)) + ((((-(p.p512 * locals.var_vdseffii_dn11)) * assign23400_e32387) + (assign23400_e32376 * (-(p.p512 * locals.var_vdseffii_dn11)))) / (2.0 * assign23400_e32395)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn12)) + ((((-(p.p512 * locals.var_vdseffii_dn12)) * assign23400_e32387) + (assign23400_e32376 * (-(p.p512 * locals.var_vdseffii_dn12)))) / (2.0 * assign23400_e32395)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn13)) + ((((-(p.p512 * locals.var_vdseffii_dn13)) * assign23400_e32387) + (assign23400_e32376 * (-(p.p512 * locals.var_vdseffii_dn13)))) / (2.0 * assign23400_e32395)))), (0.5 * ((-(p.p512 * locals.var_vdseffii_dn14)) + ((((-(p.p512 * locals.var_vdseffii_dn14)) * assign23400_e32387) + (assign23400_e32376 * (-(p.p512 * locals.var_vdseffii_dn14)))) / (2.0 * assign23400_e32395)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign23400_e32399;
        locals.var_t2_dn0 = assign23400_e32399_d_n0;
        locals.var_t2_dn2 = assign23400_e32399_d_n2;
        locals.var_t2_dn3 = assign23400_e32399_d_n3;
        locals.var_t2_dn4 = assign23400_e32399_d_n4;
        locals.var_t2_dn5 = assign23400_e32399_d_n5;
        locals.var_t2_dn6 = assign23400_e32399_d_n6;
        locals.var_t2_dn7 = assign23400_e32399_d_n7;
        locals.var_t2_dn8 = assign23400_e32399_d_n8;
        locals.var_t2_dn9 = assign23400_e32399_d_n9;
        locals.var_t2_dn10 = assign23400_e32399_d_n10;
        locals.var_t2_dn11 = assign23400_e32399_d_n11;
        locals.var_t2_dn12 = assign23400_e32399_d_n12;
        locals.var_t2_dn13 = assign23400_e32399_d_n13;
        locals.var_t2_dn14 = assign23400_e32399_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign23410_e32411, assign23410_e32411_d_n0, assign23410_e32411_d_n2, assign23410_e32411_d_n3, assign23410_e32411_d_n4, assign23410_e32411_d_n5, assign23410_e32411_d_n6, assign23410_e32411_d_n7, assign23410_e32411_d_n8, assign23410_e32411_d_n9, assign23410_e32411_d_n10, assign23410_e32411_d_n11, assign23410_e32411_d_n12, assign23410_e32411_d_n13, assign23410_e32411_d_n14,) = {
    if (locals.var_guard593 != 0.0) {
        let assign23410_e32403: f64 = (2.0 * 1.60219e-19);
        let assign23410_e32406: f64 = (p.p110 * 8.85418e-12);
        let assign23410_e32407: f64 = (assign23410_e32403 / assign23410_e32406);
        let assign23410_e32409: f64 = (assign23410_e32407 * locals.var_nextra);
        (assign23410_e32409, (assign23410_e32407 * locals.var_nextra_dn0), (assign23410_e32407 * locals.var_nextra_dn2), (assign23410_e32407 * locals.var_nextra_dn3), (assign23410_e32407 * locals.var_nextra_dn4), (assign23410_e32407 * locals.var_nextra_dn5), (assign23410_e32407 * locals.var_nextra_dn6), (assign23410_e32407 * locals.var_nextra_dn7), (assign23410_e32407 * locals.var_nextra_dn8), (assign23410_e32407 * locals.var_nextra_dn9), (assign23410_e32407 * locals.var_nextra_dn10), (assign23410_e32407 * locals.var_nextra_dn11), (assign23410_e32407 * locals.var_nextra_dn12), (assign23410_e32407 * locals.var_nextra_dn13), (assign23410_e32407 * locals.var_nextra_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign23410_e32411;
        locals.var_t3_dn0 = assign23410_e32411_d_n0;
        locals.var_t3_dn2 = assign23410_e32411_d_n2;
        locals.var_t3_dn3 = assign23410_e32411_d_n3;
        locals.var_t3_dn4 = assign23410_e32411_d_n4;
        locals.var_t3_dn5 = assign23410_e32411_d_n5;
        locals.var_t3_dn6 = assign23410_e32411_d_n6;
        locals.var_t3_dn7 = assign23410_e32411_d_n7;
        locals.var_t3_dn8 = assign23410_e32411_d_n8;
        locals.var_t3_dn9 = assign23410_e32411_d_n9;
        locals.var_t3_dn10 = assign23410_e32411_d_n10;
        locals.var_t3_dn11 = assign23410_e32411_d_n11;
        locals.var_t3_dn12 = assign23410_e32411_d_n12;
        locals.var_t3_dn13 = assign23410_e32411_d_n13;
        locals.var_t3_dn14 = assign23410_e32411_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign23420_e32419, assign23420_e32419_d_n0, assign23420_e32419_d_n2, assign23420_e32419_d_n3, assign23420_e32419_d_n4, assign23420_e32419_d_n5, assign23420_e32419_d_n6, assign23420_e32419_d_n7, assign23420_e32419_d_n8, assign23420_e32419_d_n9, assign23420_e32419_d_n10, assign23420_e32419_d_n11, assign23420_e32419_d_n12, assign23420_e32419_d_n13, assign23420_e32419_d_n14,) = {
    if (locals.var_guard593 != 0.0) {
        let assign23420_e32415: f64 = (locals.var_t3 * locals.var_t2);
        let assign23420_e32417: f64 = (assign23420_e32415).powf(0.5);
        (assign23420_e32417, if 0.0 == 0.0 && ((0.5) as f64).is_finite() && ((0.5) as f64).fract() == 0.0 { if 0.5 == 0.0 { 0.0 } else { (0.5 * ((assign23420_e32415).powf(0.5 - 1.0) * ((locals.var_t3_dn0 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn0)))) } } else { (assign23420_e32417 * (0.5 * (((locals.var_t3_dn0 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn0)) / assign23420_e32415))) }, if 0.0 == 0.0 && ((0.5) as f64).is_finite() && ((0.5) as f64).fract() == 0.0 { if 0.5 == 0.0 { 0.0 } else { (0.5 * ((assign23420_e32415).powf(0.5 - 1.0) * ((locals.var_t3_dn2 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn2)))) } } else { (assign23420_e32417 * (0.5 * (((locals.var_t3_dn2 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn2)) / assign23420_e32415))) }, if 0.0 == 0.0 && ((0.5) as f64).is_finite() && ((0.5) as f64).fract() == 0.0 { if 0.5 == 0.0 { 0.0 } else { (0.5 * ((assign23420_e32415).powf(0.5 - 1.0) * ((locals.var_t3_dn3 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn3)))) } } else { (assign23420_e32417 * (0.5 * (((locals.var_t3_dn3 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn3)) / assign23420_e32415))) }, if 0.0 == 0.0 && ((0.5) as f64).is_finite() && ((0.5) as f64).fract() == 0.0 { if 0.5 == 0.0 { 0.0 } else { (0.5 * ((assign23420_e32415).powf(0.5 - 1.0) * ((locals.var_t3_dn4 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn4)))) } } else { (assign23420_e32417 * (0.5 * (((locals.var_t3_dn4 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn4)) / assign23420_e32415))) }, if 0.0 == 0.0 && ((0.5) as f64).is_finite() && ((0.5) as f64).fract() == 0.0 { if 0.5 == 0.0 { 0.0 } else { (0.5 * ((assign23420_e32415).powf(0.5 - 1.0) * ((locals.var_t3_dn5 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn5)))) } } else { (assign23420_e32417 * (0.5 * (((locals.var_t3_dn5 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn5)) / assign23420_e32415))) }, if 0.0 == 0.0 && ((0.5) as f64).is_finite() && ((0.5) as f64).fract() == 0.0 { if 0.5 == 0.0 { 0.0 } else { (0.5 * ((assign23420_e32415).powf(0.5 - 1.0) * ((locals.var_t3_dn6 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn6)))) } } else { (assign23420_e32417 * (0.5 * (((locals.var_t3_dn6 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn6)) / assign23420_e32415))) }, if 0.0 == 0.0 && ((0.5) as f64).is_finite() && ((0.5) as f64).fract() == 0.0 { if 0.5 == 0.0 { 0.0 } else { (0.5 * ((assign23420_e32415).powf(0.5 - 1.0) * ((locals.var_t3_dn7 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn7)))) } } else { (assign23420_e32417 * (0.5 * (((locals.var_t3_dn7 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn7)) / assign23420_e32415))) }, if 0.0 == 0.0 && ((0.5) as f64).is_finite() && ((0.5) as f64).fract() == 0.0 { if 0.5 == 0.0 { 0.0 } else { (0.5 * ((assign23420_e32415).powf(0.5 - 1.0) * ((locals.var_t3_dn8 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn8)))) } } else { (assign23420_e32417 * (0.5 * (((locals.var_t3_dn8 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn8)) / assign23420_e32415))) }, if 0.0 == 0.0 && ((0.5) as f64).is_finite() && ((0.5) as f64).fract() == 0.0 { if 0.5 == 0.0 { 0.0 } else { (0.5 * ((assign23420_e32415).powf(0.5 - 1.0) * ((locals.var_t3_dn9 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn9)))) } } else { (assign23420_e32417 * (0.5 * (((locals.var_t3_dn9 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn9)) / assign23420_e32415))) }, if 0.0 == 0.0 && ((0.5) as f64).is_finite() && ((0.5) as f64).fract() == 0.0 { if 0.5 == 0.0 { 0.0 } else { (0.5 * ((assign23420_e32415).powf(0.5 - 1.0) * ((locals.var_t3_dn10 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn10)))) } } else { (assign23420_e32417 * (0.5 * (((locals.var_t3_dn10 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn10)) / assign23420_e32415))) }, if 0.0 == 0.0 && ((0.5) as f64).is_finite() && ((0.5) as f64).fract() == 0.0 { if 0.5 == 0.0 { 0.0 } else { (0.5 * ((assign23420_e32415).powf(0.5 - 1.0) * ((locals.var_t3_dn11 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn11)))) } } else { (assign23420_e32417 * (0.5 * (((locals.var_t3_dn11 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn11)) / assign23420_e32415))) }, if 0.0 == 0.0 && ((0.5) as f64).is_finite() && ((0.5) as f64).fract() == 0.0 { if 0.5 == 0.0 { 0.0 } else { (0.5 * ((assign23420_e32415).powf(0.5 - 1.0) * ((locals.var_t3_dn12 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn12)))) } } else { (assign23420_e32417 * (0.5 * (((locals.var_t3_dn12 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn12)) / assign23420_e32415))) }, if 0.0 == 0.0 && ((0.5) as f64).is_finite() && ((0.5) as f64).fract() == 0.0 { if 0.5 == 0.0 { 0.0 } else { (0.5 * ((assign23420_e32415).powf(0.5 - 1.0) * ((locals.var_t3_dn13 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn13)))) } } else { (assign23420_e32417 * (0.5 * (((locals.var_t3_dn13 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn13)) / assign23420_e32415))) }, if 0.0 == 0.0 && ((0.5) as f64).is_finite() && ((0.5) as f64).fract() == 0.0 { if 0.5 == 0.0 { 0.0 } else { (0.5 * ((assign23420_e32415).powf(0.5 - 1.0) * ((locals.var_t3_dn14 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn14)))) } } else { (assign23420_e32417 * (0.5 * (((locals.var_t3_dn14 * locals.var_t2) + (locals.var_t3 * locals.var_t2_dn14)) / assign23420_e32415))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign23420_e32419;
        locals.var_t3_dn0 = assign23420_e32419_d_n0;
        locals.var_t3_dn2 = assign23420_e32419_d_n2;
        locals.var_t3_dn3 = assign23420_e32419_d_n3;
        locals.var_t3_dn4 = assign23420_e32419_d_n4;
        locals.var_t3_dn5 = assign23420_e32419_d_n5;
        locals.var_t3_dn6 = assign23420_e32419_d_n6;
        locals.var_t3_dn7 = assign23420_e32419_d_n7;
        locals.var_t3_dn8 = assign23420_e32419_d_n8;
        locals.var_t3_dn9 = assign23420_e32419_d_n9;
        locals.var_t3_dn10 = assign23420_e32419_d_n10;
        locals.var_t3_dn11 = assign23420_e32419_d_n11;
        locals.var_t3_dn12 = assign23420_e32419_d_n12;
        locals.var_t3_dn13 = assign23420_e32419_d_n13;
        locals.var_t3_dn14 = assign23420_e32419_d_n14;
        locals.var_t3_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_69(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23430_e32431, assign23430_e32431_d_n0, assign23430_e32431_d_n2, assign23430_e32431_d_n3, assign23430_e32431_d_n4, assign23430_e32431_d_n5, assign23430_e32431_d_n6, assign23430_e32431_d_n7, assign23430_e32431_d_n8, assign23430_e32431_d_n9, assign23430_e32431_d_n10, assign23430_e32431_d_n11, assign23430_e32431_d_n12, assign23430_e32431_d_n13, assign23430_e32431_d_n14,) = {
    if (locals.var_guard593 != 0.0) {
        let assign23430_e32423: f64 = (p.p507 * locals.var_vbsx);
        let assign23430_e32426: f64 = (p.p508 * locals.var_vbsx);
        let assign23430_e32428: f64 = (assign23430_e32426 * locals.var_vbsx);
        let assign23430_e32429: f64 = (assign23430_e32423 + assign23430_e32428);
        (assign23430_e32429, ((p.p507 * locals.var_vbsx_dn0) + (((p.p508 * locals.var_vbsx_dn0) * locals.var_vbsx) + (assign23430_e32426 * locals.var_vbsx_dn0))), ((p.p507 * locals.var_vbsx_dn2) + (((p.p508 * locals.var_vbsx_dn2) * locals.var_vbsx) + (assign23430_e32426 * locals.var_vbsx_dn2))), ((p.p507 * locals.var_vbsx_dn3) + (((p.p508 * locals.var_vbsx_dn3) * locals.var_vbsx) + (assign23430_e32426 * locals.var_vbsx_dn3))), ((p.p507 * locals.var_vbsx_dn4) + (((p.p508 * locals.var_vbsx_dn4) * locals.var_vbsx) + (assign23430_e32426 * locals.var_vbsx_dn4))), ((p.p507 * locals.var_vbsx_dn5) + (((p.p508 * locals.var_vbsx_dn5) * locals.var_vbsx) + (assign23430_e32426 * locals.var_vbsx_dn5))), ((p.p507 * locals.var_vbsx_dn6) + (((p.p508 * locals.var_vbsx_dn6) * locals.var_vbsx) + (assign23430_e32426 * locals.var_vbsx_dn6))), ((p.p507 * locals.var_vbsx_dn7) + (((p.p508 * locals.var_vbsx_dn7) * locals.var_vbsx) + (assign23430_e32426 * locals.var_vbsx_dn7))), ((p.p507 * locals.var_vbsx_dn8) + (((p.p508 * locals.var_vbsx_dn8) * locals.var_vbsx) + (assign23430_e32426 * locals.var_vbsx_dn8))), ((p.p507 * locals.var_vbsx_dn9) + (((p.p508 * locals.var_vbsx_dn9) * locals.var_vbsx) + (assign23430_e32426 * locals.var_vbsx_dn9))), ((p.p507 * locals.var_vbsx_dn10) + (((p.p508 * locals.var_vbsx_dn10) * locals.var_vbsx) + (assign23430_e32426 * locals.var_vbsx_dn10))), ((p.p507 * locals.var_vbsx_dn11) + (((p.p508 * locals.var_vbsx_dn11) * locals.var_vbsx) + (assign23430_e32426 * locals.var_vbsx_dn11))), ((p.p507 * locals.var_vbsx_dn12) + (((p.p508 * locals.var_vbsx_dn12) * locals.var_vbsx) + (assign23430_e32426 * locals.var_vbsx_dn12))), ((p.p507 * locals.var_vbsx_dn13) + (((p.p508 * locals.var_vbsx_dn13) * locals.var_vbsx) + (assign23430_e32426 * locals.var_vbsx_dn13))), ((p.p507 * locals.var_vbsx_dn14) + (((p.p508 * locals.var_vbsx_dn14) * locals.var_vbsx) + (assign23430_e32426 * locals.var_vbsx_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign23430_e32431;
        locals.var_t4_dn0 = assign23430_e32431_d_n0;
        locals.var_t4_dn2 = assign23430_e32431_d_n2;
        locals.var_t4_dn3 = assign23430_e32431_d_n3;
        locals.var_t4_dn4 = assign23430_e32431_d_n4;
        locals.var_t4_dn5 = assign23430_e32431_d_n5;
        locals.var_t4_dn6 = assign23430_e32431_d_n6;
        locals.var_t4_dn7 = assign23430_e32431_d_n7;
        locals.var_t4_dn8 = assign23430_e32431_d_n8;
        locals.var_t4_dn9 = assign23430_e32431_d_n9;
        locals.var_t4_dn10 = assign23430_e32431_d_n10;
        locals.var_t4_dn11 = assign23430_e32431_d_n11;
        locals.var_t4_dn12 = assign23430_e32431_d_n12;
        locals.var_t4_dn13 = assign23430_e32431_d_n13;
        locals.var_t4_dn14 = assign23430_e32431_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign23440_e32443, assign23440_e32443_d_n0, assign23440_e32443_d_n2, assign23440_e32443_d_n3, assign23440_e32443_d_n4, assign23440_e32443_d_n5, assign23440_e32443_d_n6, assign23440_e32443_d_n7, assign23440_e32443_d_n8, assign23440_e32443_d_n9, assign23440_e32443_d_n10, assign23440_e32443_d_n11, assign23440_e32443_d_n12, assign23440_e32443_d_n13, assign23440_e32443_d_n14,) = {
    if (locals.var_guard593 != 0.0) {
        let assign23440_e32435: f64 = (p.p509 * locals.var_t2);
        let assign23440_e32439: f64 = (locals.var_t2).powf(p.p511);
        let assign23440_e32440: f64 = (p.p510 * assign23440_e32439);
        let assign23440_e32441: f64 = (assign23440_e32435 + assign23440_e32440);
        (assign23440_e32441, ((p.p509 * locals.var_t2_dn0) + (p.p510 * if 0.0 == 0.0 && ((p.p511) as f64).is_finite() && ((p.p511) as f64).fract() == 0.0 { if p.p511 == 0.0 { 0.0 } else { (p.p511 * ((locals.var_t2).powf(p.p511 - 1.0) * locals.var_t2_dn0)) } } else { (assign23440_e32439 * (p.p511 * (locals.var_t2_dn0 / locals.var_t2))) })), ((p.p509 * locals.var_t2_dn2) + (p.p510 * if 0.0 == 0.0 && ((p.p511) as f64).is_finite() && ((p.p511) as f64).fract() == 0.0 { if p.p511 == 0.0 { 0.0 } else { (p.p511 * ((locals.var_t2).powf(p.p511 - 1.0) * locals.var_t2_dn2)) } } else { (assign23440_e32439 * (p.p511 * (locals.var_t2_dn2 / locals.var_t2))) })), ((p.p509 * locals.var_t2_dn3) + (p.p510 * if 0.0 == 0.0 && ((p.p511) as f64).is_finite() && ((p.p511) as f64).fract() == 0.0 { if p.p511 == 0.0 { 0.0 } else { (p.p511 * ((locals.var_t2).powf(p.p511 - 1.0) * locals.var_t2_dn3)) } } else { (assign23440_e32439 * (p.p511 * (locals.var_t2_dn3 / locals.var_t2))) })), ((p.p509 * locals.var_t2_dn4) + (p.p510 * if 0.0 == 0.0 && ((p.p511) as f64).is_finite() && ((p.p511) as f64).fract() == 0.0 { if p.p511 == 0.0 { 0.0 } else { (p.p511 * ((locals.var_t2).powf(p.p511 - 1.0) * locals.var_t2_dn4)) } } else { (assign23440_e32439 * (p.p511 * (locals.var_t2_dn4 / locals.var_t2))) })), ((p.p509 * locals.var_t2_dn5) + (p.p510 * if 0.0 == 0.0 && ((p.p511) as f64).is_finite() && ((p.p511) as f64).fract() == 0.0 { if p.p511 == 0.0 { 0.0 } else { (p.p511 * ((locals.var_t2).powf(p.p511 - 1.0) * locals.var_t2_dn5)) } } else { (assign23440_e32439 * (p.p511 * (locals.var_t2_dn5 / locals.var_t2))) })), ((p.p509 * locals.var_t2_dn6) + (p.p510 * if 0.0 == 0.0 && ((p.p511) as f64).is_finite() && ((p.p511) as f64).fract() == 0.0 { if p.p511 == 0.0 { 0.0 } else { (p.p511 * ((locals.var_t2).powf(p.p511 - 1.0) * locals.var_t2_dn6)) } } else { (assign23440_e32439 * (p.p511 * (locals.var_t2_dn6 / locals.var_t2))) })), ((p.p509 * locals.var_t2_dn7) + (p.p510 * if 0.0 == 0.0 && ((p.p511) as f64).is_finite() && ((p.p511) as f64).fract() == 0.0 { if p.p511 == 0.0 { 0.0 } else { (p.p511 * ((locals.var_t2).powf(p.p511 - 1.0) * locals.var_t2_dn7)) } } else { (assign23440_e32439 * (p.p511 * (locals.var_t2_dn7 / locals.var_t2))) })), ((p.p509 * locals.var_t2_dn8) + (p.p510 * if 0.0 == 0.0 && ((p.p511) as f64).is_finite() && ((p.p511) as f64).fract() == 0.0 { if p.p511 == 0.0 { 0.0 } else { (p.p511 * ((locals.var_t2).powf(p.p511 - 1.0) * locals.var_t2_dn8)) } } else { (assign23440_e32439 * (p.p511 * (locals.var_t2_dn8 / locals.var_t2))) })), ((p.p509 * locals.var_t2_dn9) + (p.p510 * if 0.0 == 0.0 && ((p.p511) as f64).is_finite() && ((p.p511) as f64).fract() == 0.0 { if p.p511 == 0.0 { 0.0 } else { (p.p511 * ((locals.var_t2).powf(p.p511 - 1.0) * locals.var_t2_dn9)) } } else { (assign23440_e32439 * (p.p511 * (locals.var_t2_dn9 / locals.var_t2))) })), ((p.p509 * locals.var_t2_dn10) + (p.p510 * if 0.0 == 0.0 && ((p.p511) as f64).is_finite() && ((p.p511) as f64).fract() == 0.0 { if p.p511 == 0.0 { 0.0 } else { (p.p511 * ((locals.var_t2).powf(p.p511 - 1.0) * locals.var_t2_dn10)) } } else { (assign23440_e32439 * (p.p511 * (locals.var_t2_dn10 / locals.var_t2))) })), ((p.p509 * locals.var_t2_dn11) + (p.p510 * if 0.0 == 0.0 && ((p.p511) as f64).is_finite() && ((p.p511) as f64).fract() == 0.0 { if p.p511 == 0.0 { 0.0 } else { (p.p511 * ((locals.var_t2).powf(p.p511 - 1.0) * locals.var_t2_dn11)) } } else { (assign23440_e32439 * (p.p511 * (locals.var_t2_dn11 / locals.var_t2))) })), ((p.p509 * locals.var_t2_dn12) + (p.p510 * if 0.0 == 0.0 && ((p.p511) as f64).is_finite() && ((p.p511) as f64).fract() == 0.0 { if p.p511 == 0.0 { 0.0 } else { (p.p511 * ((locals.var_t2).powf(p.p511 - 1.0) * locals.var_t2_dn12)) } } else { (assign23440_e32439 * (p.p511 * (locals.var_t2_dn12 / locals.var_t2))) })), ((p.p509 * locals.var_t2_dn13) + (p.p510 * if 0.0 == 0.0 && ((p.p511) as f64).is_finite() && ((p.p511) as f64).fract() == 0.0 { if p.p511 == 0.0 { 0.0 } else { (p.p511 * ((locals.var_t2).powf(p.p511 - 1.0) * locals.var_t2_dn13)) } } else { (assign23440_e32439 * (p.p511 * (locals.var_t2_dn13 / locals.var_t2))) })), ((p.p509 * locals.var_t2_dn14) + (p.p510 * if 0.0 == 0.0 && ((p.p511) as f64).is_finite() && ((p.p511) as f64).fract() == 0.0 { if p.p511 == 0.0 { 0.0 } else { (p.p511 * ((locals.var_t2).powf(p.p511 - 1.0) * locals.var_t2_dn14)) } } else { (assign23440_e32439 * (p.p511 * (locals.var_t2_dn14 / locals.var_t2))) })),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign23440_e32443;
        locals.var_t5_dn0 = assign23440_e32443_d_n0;
        locals.var_t5_dn2 = assign23440_e32443_d_n2;
        locals.var_t5_dn3 = assign23440_e32443_d_n3;
        locals.var_t5_dn4 = assign23440_e32443_d_n4;
        locals.var_t5_dn5 = assign23440_e32443_d_n5;
        locals.var_t5_dn6 = assign23440_e32443_d_n6;
        locals.var_t5_dn7 = assign23440_e32443_d_n7;
        locals.var_t5_dn8 = assign23440_e32443_d_n8;
        locals.var_t5_dn9 = assign23440_e32443_d_n9;
        locals.var_t5_dn10 = assign23440_e32443_d_n10;
        locals.var_t5_dn11 = assign23440_e32443_d_n11;
        locals.var_t5_dn12 = assign23440_e32443_d_n12;
        locals.var_t5_dn13 = assign23440_e32443_d_n13;
        locals.var_t5_dn14 = assign23440_e32443_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign23450_e32453, assign23450_e32453_d_n0, assign23450_e32453_d_n2, assign23450_e32453_d_n3, assign23450_e32453_d_n4, assign23450_e32453_d_n5, assign23450_e32453_d_n6, assign23450_e32453_d_n7, assign23450_e32453_d_n8, assign23450_e32453_d_n9, assign23450_e32453_d_n10, assign23450_e32453_d_n11, assign23450_e32453_d_n12, assign23450_e32453_d_n13, assign23450_e32453_d_n14,) = {
    if (locals.var_guard593 != 0.0) {
        let assign23450_e32448: f64 = (1.0 + locals.var_t4);
        let assign23450_e32450: f64 = (assign23450_e32448 + locals.var_t5);
        let assign23450_e32451: f64 = (p.p500 * assign23450_e32450);
        (assign23450_e32451, (p.p500 * (locals.var_t4_dn0 + locals.var_t5_dn0)), (p.p500 * (locals.var_t4_dn2 + locals.var_t5_dn2)), (p.p500 * (locals.var_t4_dn3 + locals.var_t5_dn3)), (p.p500 * (locals.var_t4_dn4 + locals.var_t5_dn4)), (p.p500 * (locals.var_t4_dn5 + locals.var_t5_dn5)), (p.p500 * (locals.var_t4_dn6 + locals.var_t5_dn6)), (p.p500 * (locals.var_t4_dn7 + locals.var_t5_dn7)), (p.p500 * (locals.var_t4_dn8 + locals.var_t5_dn8)), (p.p500 * (locals.var_t4_dn9 + locals.var_t5_dn9)), (p.p500 * (locals.var_t4_dn10 + locals.var_t5_dn10)), (p.p500 * (locals.var_t4_dn11 + locals.var_t5_dn11)), (p.p500 * (locals.var_t4_dn12 + locals.var_t5_dn12)), (p.p500 * (locals.var_t4_dn13 + locals.var_t5_dn13)), (p.p500 * (locals.var_t4_dn14 + locals.var_t5_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23450_e32453;
        locals.var_t6_dn0 = assign23450_e32453_d_n0;
        locals.var_t6_dn2 = assign23450_e32453_d_n2;
        locals.var_t6_dn3 = assign23450_e32453_d_n3;
        locals.var_t6_dn4 = assign23450_e32453_d_n4;
        locals.var_t6_dn5 = assign23450_e32453_d_n5;
        locals.var_t6_dn6 = assign23450_e32453_d_n6;
        locals.var_t6_dn7 = assign23450_e32453_d_n7;
        locals.var_t6_dn8 = assign23450_e32453_d_n8;
        locals.var_t6_dn9 = assign23450_e32453_d_n9;
        locals.var_t6_dn10 = assign23450_e32453_d_n10;
        locals.var_t6_dn11 = assign23450_e32453_d_n11;
        locals.var_t6_dn12 = assign23450_e32453_d_n12;
        locals.var_t6_dn13 = assign23450_e32453_d_n13;
        locals.var_t6_dn14 = assign23450_e32453_d_n14;
        locals.var_t6_rv = 0.0;

        let assign23490_e32506: f64 = (p.p501 / 80.0);
        let assign23490_e32507: f64 = if locals.var_t3 > assign23490_e32506 { 1.0 } else { 0.0 };
        locals.var_guard600 = assign23490_e32507;
        locals.var_guard600_rv = 0.0;

        let (assign23500_e32516, assign23500_e32516_d_n0, assign23500_e32516_d_n2, assign23500_e32516_d_n3, assign23500_e32516_d_n4, assign23500_e32516_d_n5, assign23500_e32516_d_n6, assign23500_e32516_d_n7, assign23500_e32516_d_n8, assign23500_e32516_d_n9, assign23500_e32516_d_n10, assign23500_e32516_d_n11, assign23500_e32516_d_n12, assign23500_e32516_d_n13, assign23500_e32516_d_n14,) = {
    if ((locals.var_guard593 != 0.0) && (locals.var_guard600 != 0.0)) {
        let assign23500_e32512: f64 = (-p.p501);
        let assign23500_e32514: f64 = (assign23500_e32512 / locals.var_t3);
        (assign23500_e32514, (-((assign23500_e32512 * locals.var_t3_dn0) / (locals.var_t3 * locals.var_t3))), (-((assign23500_e32512 * locals.var_t3_dn2) / (locals.var_t3 * locals.var_t3))), (-((assign23500_e32512 * locals.var_t3_dn3) / (locals.var_t3 * locals.var_t3))), (-((assign23500_e32512 * locals.var_t3_dn4) / (locals.var_t3 * locals.var_t3))), (-((assign23500_e32512 * locals.var_t3_dn5) / (locals.var_t3 * locals.var_t3))), (-((assign23500_e32512 * locals.var_t3_dn6) / (locals.var_t3 * locals.var_t3))), (-((assign23500_e32512 * locals.var_t3_dn7) / (locals.var_t3 * locals.var_t3))), (-((assign23500_e32512 * locals.var_t3_dn8) / (locals.var_t3 * locals.var_t3))), (-((assign23500_e32512 * locals.var_t3_dn9) / (locals.var_t3 * locals.var_t3))), (-((assign23500_e32512 * locals.var_t3_dn10) / (locals.var_t3 * locals.var_t3))), (-((assign23500_e32512 * locals.var_t3_dn11) / (locals.var_t3 * locals.var_t3))), (-((assign23500_e32512 * locals.var_t3_dn12) / (locals.var_t3 * locals.var_t3))), (-((assign23500_e32512 * locals.var_t3_dn13) / (locals.var_t3 * locals.var_t3))), (-((assign23500_e32512 * locals.var_t3_dn14) / (locals.var_t3 * locals.var_t3))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23500_e32516;
        locals.var_t1_dn0 = assign23500_e32516_d_n0;
        locals.var_t1_dn2 = assign23500_e32516_d_n2;
        locals.var_t1_dn3 = assign23500_e32516_d_n3;
        locals.var_t1_dn4 = assign23500_e32516_d_n4;
        locals.var_t1_dn5 = assign23500_e32516_d_n5;
        locals.var_t1_dn6 = assign23500_e32516_d_n6;
        locals.var_t1_dn7 = assign23500_e32516_d_n7;
        locals.var_t1_dn8 = assign23500_e32516_d_n8;
        locals.var_t1_dn9 = assign23500_e32516_d_n9;
        locals.var_t1_dn10 = assign23500_e32516_d_n10;
        locals.var_t1_dn11 = assign23500_e32516_d_n11;
        locals.var_t1_dn12 = assign23500_e32516_d_n12;
        locals.var_t1_dn13 = assign23500_e32516_d_n13;
        locals.var_t1_dn14 = assign23500_e32516_d_n14;
        locals.var_t1_rv = 0.0;

        let assign23610_e32561: f64 = if ((p.p46 != 0.0) || (p.p47 != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard601 = assign23610_e32561;
        locals.var_guard601_rv = 0.0;

        let (assign23620_e32573, assign23620_e32573_d_n0, assign23620_e32573_d_n2, assign23620_e32573_d_n3, assign23620_e32573_d_n4, assign23620_e32573_d_n5, assign23620_e32573_d_n6, assign23620_e32573_d_n7, assign23620_e32573_d_n8, assign23620_e32573_d_n9, assign23620_e32573_d_n10, assign23620_e32573_d_n11, assign23620_e32573_d_n12, assign23620_e32573_d_n13, assign23620_e32573_d_n14,) = {
    if (locals.var_guard601 != 0.0) {
        let assign23620_e32566: f64 = (locals.var_vgfb - locals.var_psip);
        let assign23620_e32568: f64 = (assign23620_e32566 + locals.var_qs_1);
        let assign23620_e32570: f64 = (assign23620_e32568 + locals.var_qdeff);
        let assign23620_e32571: f64 = (locals.var_nvt * assign23620_e32570);
        (assign23620_e32571, ((locals.var_nvt_dn0 * assign23620_e32570) + (locals.var_nvt * (((locals.var_vgfb_dn0 - locals.var_psip_dn0) + locals.var_qs_1_dn0) + locals.var_qdeff_dn0))), ((locals.var_nvt_dn2 * assign23620_e32570) + (locals.var_nvt * (((locals.var_vgfb_dn2 - locals.var_psip_dn2) + locals.var_qs_1_dn2) + locals.var_qdeff_dn2))), ((locals.var_nvt_dn3 * assign23620_e32570) + (locals.var_nvt * (((locals.var_vgfb_dn3 - locals.var_psip_dn3) + locals.var_qs_1_dn3) + locals.var_qdeff_dn3))), ((locals.var_nvt_dn4 * assign23620_e32570) + (locals.var_nvt * (((locals.var_vgfb_dn4 - locals.var_psip_dn4) + locals.var_qs_1_dn4) + locals.var_qdeff_dn4))), ((locals.var_nvt_dn5 * assign23620_e32570) + (locals.var_nvt * (((locals.var_vgfb_dn5 - locals.var_psip_dn5) + locals.var_qs_1_dn5) + locals.var_qdeff_dn5))), ((locals.var_nvt_dn6 * assign23620_e32570) + (locals.var_nvt * (((locals.var_vgfb_dn6 - locals.var_psip_dn6) + locals.var_qs_1_dn6) + locals.var_qdeff_dn6))), ((locals.var_nvt_dn7 * assign23620_e32570) + (locals.var_nvt * (((locals.var_vgfb_dn7 - locals.var_psip_dn7) + locals.var_qs_1_dn7) + locals.var_qdeff_dn7))), ((locals.var_nvt_dn8 * assign23620_e32570) + (locals.var_nvt * (((locals.var_vgfb_dn8 - locals.var_psip_dn8) + locals.var_qs_1_dn8) + locals.var_qdeff_dn8))), ((locals.var_nvt_dn9 * assign23620_e32570) + (locals.var_nvt * (((locals.var_vgfb_dn9 - locals.var_psip_dn9) + locals.var_qs_1_dn9) + locals.var_qdeff_dn9))), ((locals.var_nvt_dn10 * assign23620_e32570) + (locals.var_nvt * (((locals.var_vgfb_dn10 - locals.var_psip_dn10) + locals.var_qs_1_dn10) + locals.var_qdeff_dn10))), ((locals.var_nvt_dn11 * assign23620_e32570) + (locals.var_nvt * (((locals.var_vgfb_dn11 - locals.var_psip_dn11) + locals.var_qs_1_dn11) + locals.var_qdeff_dn11))), ((locals.var_nvt_dn12 * assign23620_e32570) + (locals.var_nvt * (((locals.var_vgfb_dn12 - locals.var_psip_dn12) + locals.var_qs_1_dn12) + locals.var_qdeff_dn12))), ((locals.var_nvt_dn13 * assign23620_e32570) + (locals.var_nvt * (((locals.var_vgfb_dn13 - locals.var_psip_dn13) + locals.var_qs_1_dn13) + locals.var_qdeff_dn13))), ((locals.var_nvt_dn14 * assign23620_e32570) + (locals.var_nvt * (((locals.var_vgfb_dn14 - locals.var_psip_dn14) + locals.var_qs_1_dn14) + locals.var_qdeff_dn14))),)
    } else {
        (locals.var_voxm, locals.var_voxm_dn0, locals.var_voxm_dn2, locals.var_voxm_dn3, locals.var_voxm_dn4, locals.var_voxm_dn5, locals.var_voxm_dn6, locals.var_voxm_dn7, locals.var_voxm_dn8, locals.var_voxm_dn9, locals.var_voxm_dn10, locals.var_voxm_dn11, locals.var_voxm_dn12, locals.var_voxm_dn13, locals.var_voxm_dn14,)
    }
};
        locals.var_voxm = assign23620_e32573;
        locals.var_voxm_dn0 = assign23620_e32573_d_n0;
        locals.var_voxm_dn2 = assign23620_e32573_d_n2;
        locals.var_voxm_dn3 = assign23620_e32573_d_n3;
        locals.var_voxm_dn4 = assign23620_e32573_d_n4;
        locals.var_voxm_dn5 = assign23620_e32573_d_n5;
        locals.var_voxm_dn6 = assign23620_e32573_d_n6;
        locals.var_voxm_dn7 = assign23620_e32573_d_n7;
        locals.var_voxm_dn8 = assign23620_e32573_d_n8;
        locals.var_voxm_dn9 = assign23620_e32573_d_n9;
        locals.var_voxm_dn10 = assign23620_e32573_d_n10;
        locals.var_voxm_dn11 = assign23620_e32573_d_n11;
        locals.var_voxm_dn12 = assign23620_e32573_d_n12;
        locals.var_voxm_dn13 = assign23620_e32573_d_n13;
        locals.var_voxm_dn14 = assign23620_e32573_d_n14;
        locals.var_voxm_rv = 0.0;

        let (assign23630_e32582, assign23630_e32582_d_n0, assign23630_e32582_d_n2, assign23630_e32582_d_n3, assign23630_e32582_d_n4, assign23630_e32582_d_n5, assign23630_e32582_d_n6, assign23630_e32582_d_n7, assign23630_e32582_d_n8, assign23630_e32582_d_n9, assign23630_e32582_d_n10, assign23630_e32582_d_n11, assign23630_e32582_d_n12, assign23630_e32582_d_n13, assign23630_e32582_d_n14,) = {
    if (locals.var_guard601 != 0.0) {
        let assign23630_e32577: f64 = (locals.var_voxm * locals.var_voxm);
        let assign23630_e32579: f64 = (assign23630_e32577 + 0.0001);
        let assign23630_e32580: f64 = (assign23630_e32579).sqrt();
        (assign23630_e32580, (((locals.var_voxm_dn0 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn0)) / (2.0 * assign23630_e32580)), (((locals.var_voxm_dn2 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn2)) / (2.0 * assign23630_e32580)), (((locals.var_voxm_dn3 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn3)) / (2.0 * assign23630_e32580)), (((locals.var_voxm_dn4 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn4)) / (2.0 * assign23630_e32580)), (((locals.var_voxm_dn5 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn5)) / (2.0 * assign23630_e32580)), (((locals.var_voxm_dn6 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn6)) / (2.0 * assign23630_e32580)), (((locals.var_voxm_dn7 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn7)) / (2.0 * assign23630_e32580)), (((locals.var_voxm_dn8 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn8)) / (2.0 * assign23630_e32580)), (((locals.var_voxm_dn9 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn9)) / (2.0 * assign23630_e32580)), (((locals.var_voxm_dn10 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn10)) / (2.0 * assign23630_e32580)), (((locals.var_voxm_dn11 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn11)) / (2.0 * assign23630_e32580)), (((locals.var_voxm_dn12 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn12)) / (2.0 * assign23630_e32580)), (((locals.var_voxm_dn13 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn13)) / (2.0 * assign23630_e32580)), (((locals.var_voxm_dn14 * locals.var_voxm) + (locals.var_voxm * locals.var_voxm_dn14)) / (2.0 * assign23630_e32580)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23630_e32582;
        locals.var_t1_dn0 = assign23630_e32582_d_n0;
        locals.var_t1_dn2 = assign23630_e32582_d_n2;
        locals.var_t1_dn3 = assign23630_e32582_d_n3;
        locals.var_t1_dn4 = assign23630_e32582_d_n4;
        locals.var_t1_dn5 = assign23630_e32582_d_n5;
        locals.var_t1_dn6 = assign23630_e32582_d_n6;
        locals.var_t1_dn7 = assign23630_e32582_d_n7;
        locals.var_t1_dn8 = assign23630_e32582_d_n8;
        locals.var_t1_dn9 = assign23630_e32582_d_n9;
        locals.var_t1_dn10 = assign23630_e32582_d_n10;
        locals.var_t1_dn11 = assign23630_e32582_d_n11;
        locals.var_t1_dn12 = assign23630_e32582_d_n12;
        locals.var_t1_dn13 = assign23630_e32582_d_n13;
        locals.var_t1_dn14 = assign23630_e32582_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign23640_e32591, assign23640_e32591_d_n0, assign23640_e32591_d_n2, assign23640_e32591_d_n3, assign23640_e32591_d_n4, assign23640_e32591_d_n5, assign23640_e32591_d_n6, assign23640_e32591_d_n7, assign23640_e32591_d_n8, assign23640_e32591_d_n9, assign23640_e32591_d_n10, assign23640_e32591_d_n11, assign23640_e32591_d_n12, assign23640_e32591_d_n13, assign23640_e32591_d_n14,) = {
    if (locals.var_guard601 != 0.0) {
        let assign23640_e32586: f64 = (-locals.var_voxm);
        let assign23640_e32588: f64 = (assign23640_e32586 + locals.var_t1);
        let assign23640_e32589: f64 = (0.5 * assign23640_e32588);
        (assign23640_e32589, (0.5 * ((-locals.var_voxm_dn0) + locals.var_t1_dn0)), (0.5 * ((-locals.var_voxm_dn2) + locals.var_t1_dn2)), (0.5 * ((-locals.var_voxm_dn3) + locals.var_t1_dn3)), (0.5 * ((-locals.var_voxm_dn4) + locals.var_t1_dn4)), (0.5 * ((-locals.var_voxm_dn5) + locals.var_t1_dn5)), (0.5 * ((-locals.var_voxm_dn6) + locals.var_t1_dn6)), (0.5 * ((-locals.var_voxm_dn7) + locals.var_t1_dn7)), (0.5 * ((-locals.var_voxm_dn8) + locals.var_t1_dn8)), (0.5 * ((-locals.var_voxm_dn9) + locals.var_t1_dn9)), (0.5 * ((-locals.var_voxm_dn10) + locals.var_t1_dn10)), (0.5 * ((-locals.var_voxm_dn11) + locals.var_t1_dn11)), (0.5 * ((-locals.var_voxm_dn12) + locals.var_t1_dn12)), (0.5 * ((-locals.var_voxm_dn13) + locals.var_t1_dn13)), (0.5 * ((-locals.var_voxm_dn14) + locals.var_t1_dn14)),)
    } else {
        (locals.var_voxmacc, locals.var_voxmacc_dn0, locals.var_voxmacc_dn2, locals.var_voxmacc_dn3, locals.var_voxmacc_dn4, locals.var_voxmacc_dn5, locals.var_voxmacc_dn6, locals.var_voxmacc_dn7, locals.var_voxmacc_dn8, locals.var_voxmacc_dn9, locals.var_voxmacc_dn10, locals.var_voxmacc_dn11, locals.var_voxmacc_dn12, locals.var_voxmacc_dn13, locals.var_voxmacc_dn14,)
    }
};
        locals.var_voxmacc = assign23640_e32591;
        locals.var_voxmacc_dn0 = assign23640_e32591_d_n0;
        locals.var_voxmacc_dn2 = assign23640_e32591_d_n2;
        locals.var_voxmacc_dn3 = assign23640_e32591_d_n3;
        locals.var_voxmacc_dn4 = assign23640_e32591_d_n4;
        locals.var_voxmacc_dn5 = assign23640_e32591_d_n5;
        locals.var_voxmacc_dn6 = assign23640_e32591_d_n6;
        locals.var_voxmacc_dn7 = assign23640_e32591_d_n7;
        locals.var_voxmacc_dn8 = assign23640_e32591_d_n8;
        locals.var_voxmacc_dn9 = assign23640_e32591_d_n9;
        locals.var_voxmacc_dn10 = assign23640_e32591_d_n10;
        locals.var_voxmacc_dn11 = assign23640_e32591_d_n11;
        locals.var_voxmacc_dn12 = assign23640_e32591_d_n12;
        locals.var_voxmacc_dn13 = assign23640_e32591_d_n13;
        locals.var_voxmacc_dn14 = assign23640_e32591_d_n14;
        locals.var_voxmacc_rv = 0.0;

        let (assign23650_e32599, assign23650_e32599_d_n0, assign23650_e32599_d_n2, assign23650_e32599_d_n3, assign23650_e32599_d_n4, assign23650_e32599_d_n5, assign23650_e32599_d_n6, assign23650_e32599_d_n7, assign23650_e32599_d_n8, assign23650_e32599_d_n9, assign23650_e32599_d_n10, assign23650_e32599_d_n11, assign23650_e32599_d_n12, assign23650_e32599_d_n13, assign23650_e32599_d_n14,) = {
    if (locals.var_guard601 != 0.0) {
        let assign23650_e32596: f64 = (locals.var_voxm + locals.var_t1);
        let assign23650_e32597: f64 = (0.5 * assign23650_e32596);
        (assign23650_e32597, (0.5 * (locals.var_voxm_dn0 + locals.var_t1_dn0)), (0.5 * (locals.var_voxm_dn2 + locals.var_t1_dn2)), (0.5 * (locals.var_voxm_dn3 + locals.var_t1_dn3)), (0.5 * (locals.var_voxm_dn4 + locals.var_t1_dn4)), (0.5 * (locals.var_voxm_dn5 + locals.var_t1_dn5)), (0.5 * (locals.var_voxm_dn6 + locals.var_t1_dn6)), (0.5 * (locals.var_voxm_dn7 + locals.var_t1_dn7)), (0.5 * (locals.var_voxm_dn8 + locals.var_t1_dn8)), (0.5 * (locals.var_voxm_dn9 + locals.var_t1_dn9)), (0.5 * (locals.var_voxm_dn10 + locals.var_t1_dn10)), (0.5 * (locals.var_voxm_dn11 + locals.var_t1_dn11)), (0.5 * (locals.var_voxm_dn12 + locals.var_t1_dn12)), (0.5 * (locals.var_voxm_dn13 + locals.var_t1_dn13)), (0.5 * (locals.var_voxm_dn14 + locals.var_t1_dn14)),)
    } else {
        (locals.var_voxminv, locals.var_voxminv_dn0, locals.var_voxminv_dn2, locals.var_voxminv_dn3, locals.var_voxminv_dn4, locals.var_voxminv_dn5, locals.var_voxminv_dn6, locals.var_voxminv_dn7, locals.var_voxminv_dn8, locals.var_voxminv_dn9, locals.var_voxminv_dn10, locals.var_voxminv_dn11, locals.var_voxminv_dn12, locals.var_voxminv_dn13, locals.var_voxminv_dn14,)
    }
};
        locals.var_voxminv = assign23650_e32599;
        locals.var_voxminv_dn0 = assign23650_e32599_d_n0;
        locals.var_voxminv_dn2 = assign23650_e32599_d_n2;
        locals.var_voxminv_dn3 = assign23650_e32599_d_n3;
        locals.var_voxminv_dn4 = assign23650_e32599_d_n4;
        locals.var_voxminv_dn5 = assign23650_e32599_d_n5;
        locals.var_voxminv_dn6 = assign23650_e32599_d_n6;
        locals.var_voxminv_dn7 = assign23650_e32599_d_n7;
        locals.var_voxminv_dn8 = assign23650_e32599_d_n8;
        locals.var_voxminv_dn9 = assign23650_e32599_d_n9;
        locals.var_voxminv_dn10 = assign23650_e32599_d_n10;
        locals.var_voxminv_dn11 = assign23650_e32599_d_n11;
        locals.var_voxminv_dn12 = assign23650_e32599_d_n12;
        locals.var_voxminv_dn13 = assign23650_e32599_d_n13;
        locals.var_voxminv_dn14 = assign23650_e32599_d_n14;
        locals.var_voxminv_rv = 0.0;

        let assign23660_e32602: f64 = if p.p47 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard602 = assign23660_e32602;
        locals.var_guard602_rv = 0.0;

        let (assign23670_e32612, assign23670_e32612_d_n0, assign23670_e32612_d_n2, assign23670_e32612_d_n3, assign23670_e32612_d_n4, assign23670_e32612_d_n5, assign23670_e32612_d_n6, assign23670_e32612_d_n7, assign23670_e32612_d_n8, assign23670_e32612_d_n9, assign23670_e32612_d_n10, assign23670_e32612_d_n11, assign23670_e32612_d_n12, assign23670_e32612_d_n13, assign23670_e32612_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard602 != 0.0)) {
        let assign23670_e32608: f64 = (locals.var_voxm / locals.var_nigbacc_i);
        let assign23670_e32610: f64 = (assign23670_e32608 / locals.var_vt);
        (assign23670_e32610, ((locals.var_voxm_dn0 / locals.var_nigbacc_i) / locals.var_vt), ((locals.var_voxm_dn2 / locals.var_nigbacc_i) / locals.var_vt), ((locals.var_voxm_dn3 / locals.var_nigbacc_i) / locals.var_vt), ((((locals.var_voxm_dn4 / locals.var_nigbacc_i) * locals.var_vt) - (assign23670_e32608 * locals.var_vt_dn4)) / (locals.var_vt * locals.var_vt)), ((locals.var_voxm_dn5 / locals.var_nigbacc_i) / locals.var_vt), ((locals.var_voxm_dn6 / locals.var_nigbacc_i) / locals.var_vt), ((locals.var_voxm_dn7 / locals.var_nigbacc_i) / locals.var_vt), ((locals.var_voxm_dn8 / locals.var_nigbacc_i) / locals.var_vt), ((locals.var_voxm_dn9 / locals.var_nigbacc_i) / locals.var_vt), ((locals.var_voxm_dn10 / locals.var_nigbacc_i) / locals.var_vt), ((locals.var_voxm_dn11 / locals.var_nigbacc_i) / locals.var_vt), ((locals.var_voxm_dn12 / locals.var_nigbacc_i) / locals.var_vt), ((locals.var_voxm_dn13 / locals.var_nigbacc_i) / locals.var_vt), ((locals.var_voxm_dn14 / locals.var_nigbacc_i) / locals.var_vt),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23670_e32612;
        locals.var_t1_dn0 = assign23670_e32612_d_n0;
        locals.var_t1_dn2 = assign23670_e32612_d_n2;
        locals.var_t1_dn3 = assign23670_e32612_d_n3;
        locals.var_t1_dn4 = assign23670_e32612_d_n4;
        locals.var_t1_dn5 = assign23670_e32612_d_n5;
        locals.var_t1_dn6 = assign23670_e32612_d_n6;
        locals.var_t1_dn7 = assign23670_e32612_d_n7;
        locals.var_t1_dn8 = assign23670_e32612_d_n8;
        locals.var_t1_dn9 = assign23670_e32612_d_n9;
        locals.var_t1_dn10 = assign23670_e32612_d_n10;
        locals.var_t1_dn11 = assign23670_e32612_d_n11;
        locals.var_t1_dn12 = assign23670_e32612_d_n12;
        locals.var_t1_dn13 = assign23670_e32612_d_n13;
        locals.var_t1_dn14 = assign23670_e32612_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign23690_e32673, assign23690_e32673_d_n0, assign23690_e32673_d_n2, assign23690_e32673_d_n3, assign23690_e32673_d_n4, assign23690_e32673_d_n5, assign23690_e32673_d_n6, assign23690_e32673_d_n7, assign23690_e32673_d_n8, assign23690_e32673_d_n9, assign23690_e32673_d_n10, assign23690_e32673_d_n11, assign23690_e32673_d_n12, assign23690_e32673_d_n13, assign23690_e32673_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard602 != 0.0)) {
        let assign23690_e32670: f64 = (locals.var_bigbacc_i * locals.var_voxmacc);
        let assign23690_e32671: f64 = (locals.var_aigbacc_i - assign23690_e32670);
        (assign23690_e32671, (-(locals.var_bigbacc_i * locals.var_voxmacc_dn0)), (-(locals.var_bigbacc_i * locals.var_voxmacc_dn2)), (-(locals.var_bigbacc_i * locals.var_voxmacc_dn3)), (-(locals.var_bigbacc_i * locals.var_voxmacc_dn4)), (-(locals.var_bigbacc_i * locals.var_voxmacc_dn5)), (-(locals.var_bigbacc_i * locals.var_voxmacc_dn6)), (-(locals.var_bigbacc_i * locals.var_voxmacc_dn7)), (-(locals.var_bigbacc_i * locals.var_voxmacc_dn8)), (-(locals.var_bigbacc_i * locals.var_voxmacc_dn9)), (-(locals.var_bigbacc_i * locals.var_voxmacc_dn10)), (-(locals.var_bigbacc_i * locals.var_voxmacc_dn11)), (-(locals.var_bigbacc_i * locals.var_voxmacc_dn12)), (-(locals.var_bigbacc_i * locals.var_voxmacc_dn13)), (-(locals.var_bigbacc_i * locals.var_voxmacc_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign23690_e32673;
        locals.var_t2_dn0 = assign23690_e32673_d_n0;
        locals.var_t2_dn2 = assign23690_e32673_d_n2;
        locals.var_t2_dn3 = assign23690_e32673_d_n3;
        locals.var_t2_dn4 = assign23690_e32673_d_n4;
        locals.var_t2_dn5 = assign23690_e32673_d_n5;
        locals.var_t2_dn6 = assign23690_e32673_d_n6;
        locals.var_t2_dn7 = assign23690_e32673_d_n7;
        locals.var_t2_dn8 = assign23690_e32673_d_n8;
        locals.var_t2_dn9 = assign23690_e32673_d_n9;
        locals.var_t2_dn10 = assign23690_e32673_d_n10;
        locals.var_t2_dn11 = assign23690_e32673_d_n11;
        locals.var_t2_dn12 = assign23690_e32673_d_n12;
        locals.var_t2_dn13 = assign23690_e32673_d_n13;
        locals.var_t2_dn14 = assign23690_e32673_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign23700_e32683, assign23700_e32683_d_n0, assign23700_e32683_d_n2, assign23700_e32683_d_n3, assign23700_e32683_d_n4, assign23700_e32683_d_n5, assign23700_e32683_d_n6, assign23700_e32683_d_n7, assign23700_e32683_d_n8, assign23700_e32683_d_n9, assign23700_e32683_d_n10, assign23700_e32683_d_n11, assign23700_e32683_d_n12, assign23700_e32683_d_n13, assign23700_e32683_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard602 != 0.0)) {
        let assign23700_e32680: f64 = (locals.var_cigbacc_i * locals.var_voxmacc);
        let assign23700_e32681: f64 = (1.0 + assign23700_e32680);
        (assign23700_e32681, (locals.var_cigbacc_i * locals.var_voxmacc_dn0), (locals.var_cigbacc_i * locals.var_voxmacc_dn2), (locals.var_cigbacc_i * locals.var_voxmacc_dn3), (locals.var_cigbacc_i * locals.var_voxmacc_dn4), (locals.var_cigbacc_i * locals.var_voxmacc_dn5), (locals.var_cigbacc_i * locals.var_voxmacc_dn6), (locals.var_cigbacc_i * locals.var_voxmacc_dn7), (locals.var_cigbacc_i * locals.var_voxmacc_dn8), (locals.var_cigbacc_i * locals.var_voxmacc_dn9), (locals.var_cigbacc_i * locals.var_voxmacc_dn10), (locals.var_cigbacc_i * locals.var_voxmacc_dn11), (locals.var_cigbacc_i * locals.var_voxmacc_dn12), (locals.var_cigbacc_i * locals.var_voxmacc_dn13), (locals.var_cigbacc_i * locals.var_voxmacc_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign23700_e32683;
        locals.var_t3_dn0 = assign23700_e32683_d_n0;
        locals.var_t3_dn2 = assign23700_e32683_d_n2;
        locals.var_t3_dn3 = assign23700_e32683_d_n3;
        locals.var_t3_dn4 = assign23700_e32683_d_n4;
        locals.var_t3_dn5 = assign23700_e32683_d_n5;
        locals.var_t3_dn6 = assign23700_e32683_d_n6;
        locals.var_t3_dn7 = assign23700_e32683_d_n7;
        locals.var_t3_dn8 = assign23700_e32683_d_n8;
        locals.var_t3_dn9 = assign23700_e32683_d_n9;
        locals.var_t3_dn10 = assign23700_e32683_d_n10;
        locals.var_t3_dn11 = assign23700_e32683_d_n11;
        locals.var_t3_dn12 = assign23700_e32683_d_n12;
        locals.var_t3_dn13 = assign23700_e32683_d_n13;
        locals.var_t3_dn14 = assign23700_e32683_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign23710_e32696, assign23710_e32696_d_n0, assign23710_e32696_d_n2, assign23710_e32696_d_n3, assign23710_e32696_d_n4, assign23710_e32696_d_n5, assign23710_e32696_d_n6, assign23710_e32696_d_n7, assign23710_e32696_d_n8, assign23710_e32696_d_n9, assign23710_e32696_d_n10, assign23710_e32696_d_n11, assign23710_e32696_d_n12, assign23710_e32696_d_n13, assign23710_e32696_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard602 != 0.0)) {
        let assign23710_e32688: f64 = (-745669000000.0);
        let assign23710_e32690: f64 = (assign23710_e32688 * p.p77);
        let assign23710_e32692: f64 = (assign23710_e32690 * locals.var_t2);
        let assign23710_e32694: f64 = (assign23710_e32692 * locals.var_t3);
        (assign23710_e32694, (((assign23710_e32690 * locals.var_t2_dn0) * locals.var_t3) + (assign23710_e32692 * locals.var_t3_dn0)), (((assign23710_e32690 * locals.var_t2_dn2) * locals.var_t3) + (assign23710_e32692 * locals.var_t3_dn2)), (((assign23710_e32690 * locals.var_t2_dn3) * locals.var_t3) + (assign23710_e32692 * locals.var_t3_dn3)), (((assign23710_e32690 * locals.var_t2_dn4) * locals.var_t3) + (assign23710_e32692 * locals.var_t3_dn4)), (((assign23710_e32690 * locals.var_t2_dn5) * locals.var_t3) + (assign23710_e32692 * locals.var_t3_dn5)), (((assign23710_e32690 * locals.var_t2_dn6) * locals.var_t3) + (assign23710_e32692 * locals.var_t3_dn6)), (((assign23710_e32690 * locals.var_t2_dn7) * locals.var_t3) + (assign23710_e32692 * locals.var_t3_dn7)), (((assign23710_e32690 * locals.var_t2_dn8) * locals.var_t3) + (assign23710_e32692 * locals.var_t3_dn8)), (((assign23710_e32690 * locals.var_t2_dn9) * locals.var_t3) + (assign23710_e32692 * locals.var_t3_dn9)), (((assign23710_e32690 * locals.var_t2_dn10) * locals.var_t3) + (assign23710_e32692 * locals.var_t3_dn10)), (((assign23710_e32690 * locals.var_t2_dn11) * locals.var_t3) + (assign23710_e32692 * locals.var_t3_dn11)), (((assign23710_e32690 * locals.var_t2_dn12) * locals.var_t3) + (assign23710_e32692 * locals.var_t3_dn12)), (((assign23710_e32690 * locals.var_t2_dn13) * locals.var_t3) + (assign23710_e32692 * locals.var_t3_dn13)), (((assign23710_e32690 * locals.var_t2_dn14) * locals.var_t3) + (assign23710_e32692 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign23710_e32696;
        locals.var_t4_dn0 = assign23710_e32696_d_n0;
        locals.var_t4_dn2 = assign23710_e32696_d_n2;
        locals.var_t4_dn3 = assign23710_e32696_d_n3;
        locals.var_t4_dn4 = assign23710_e32696_d_n4;
        locals.var_t4_dn5 = assign23710_e32696_d_n5;
        locals.var_t4_dn6 = assign23710_e32696_d_n6;
        locals.var_t4_dn7 = assign23710_e32696_d_n7;
        locals.var_t4_dn8 = assign23710_e32696_d_n8;
        locals.var_t4_dn9 = assign23710_e32696_d_n9;
        locals.var_t4_dn10 = assign23710_e32696_d_n10;
        locals.var_t4_dn11 = assign23710_e32696_d_n11;
        locals.var_t4_dn12 = assign23710_e32696_d_n12;
        locals.var_t4_dn13 = assign23710_e32696_d_n13;
        locals.var_t4_dn14 = assign23710_e32696_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign23720_e32703, assign23720_e32703_d_n0, assign23720_e32703_d_n2, assign23720_e32703_d_n3, assign23720_e32703_d_n4, assign23720_e32703_d_n5, assign23720_e32703_d_n6, assign23720_e32703_d_n7, assign23720_e32703_d_n8, assign23720_e32703_d_n9, assign23720_e32703_d_n10, assign23720_e32703_d_n11, assign23720_e32703_d_n12, assign23720_e32703_d_n13, assign23720_e32703_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard602 != 0.0)) {
        let assign23720_e32701: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign23720_e32701, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn0), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn2), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn12), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn13), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign23720_e32703;
        locals.var_t5_dn0 = assign23720_e32703_d_n0;
        locals.var_t5_dn2 = assign23720_e32703_d_n2;
        locals.var_t5_dn3 = assign23720_e32703_d_n3;
        locals.var_t5_dn4 = assign23720_e32703_d_n4;
        locals.var_t5_dn5 = assign23720_e32703_d_n5;
        locals.var_t5_dn6 = assign23720_e32703_d_n6;
        locals.var_t5_dn7 = assign23720_e32703_d_n7;
        locals.var_t5_dn8 = assign23720_e32703_d_n8;
        locals.var_t5_dn9 = assign23720_e32703_d_n9;
        locals.var_t5_dn10 = assign23720_e32703_d_n10;
        locals.var_t5_dn11 = assign23720_e32703_d_n11;
        locals.var_t5_dn12 = assign23720_e32703_d_n12;
        locals.var_t5_dn13 = assign23720_e32703_d_n13;
        locals.var_t5_dn14 = assign23720_e32703_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign23730_e32709, assign23730_e32709_d_n0, assign23730_e32709_d_n2, assign23730_e32709_d_n3, assign23730_e32709_d_n4, assign23730_e32709_d_n5, assign23730_e32709_d_n6, assign23730_e32709_d_n7, assign23730_e32709_d_n8, assign23730_e32709_d_n9, assign23730_e32709_d_n10, assign23730_e32709_d_n11, assign23730_e32709_d_n12, assign23730_e32709_d_n13, assign23730_e32709_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard602 != 0.0)) {
        (4.97232e-7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23730_e32709;
        locals.var_t6_dn0 = assign23730_e32709_d_n0;
        locals.var_t6_dn2 = assign23730_e32709_d_n2;
        locals.var_t6_dn3 = assign23730_e32709_d_n3;
        locals.var_t6_dn4 = assign23730_e32709_d_n4;
        locals.var_t6_dn5 = assign23730_e32709_d_n5;
        locals.var_t6_dn6 = assign23730_e32709_d_n6;
        locals.var_t6_dn7 = assign23730_e32709_d_n7;
        locals.var_t6_dn8 = assign23730_e32709_d_n8;
        locals.var_t6_dn9 = assign23730_e32709_d_n9;
        locals.var_t6_dn10 = assign23730_e32709_d_n10;
        locals.var_t6_dn11 = assign23730_e32709_d_n11;
        locals.var_t6_dn12 = assign23730_e32709_d_n12;
        locals.var_t6_dn13 = assign23730_e32709_d_n13;
        locals.var_t6_dn14 = assign23730_e32709_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign23760_e32749, assign23760_e32749_d_n0, assign23760_e32749_d_n2, assign23760_e32749_d_n3, assign23760_e32749_d_n4, assign23760_e32749_d_n5, assign23760_e32749_d_n6, assign23760_e32749_d_n7, assign23760_e32749_d_n8, assign23760_e32749_d_n9, assign23760_e32749_d_n10, assign23760_e32749_d_n11, assign23760_e32749_d_n12, assign23760_e32749_d_n13, assign23760_e32749_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard602 != 0.0)) {
        let assign23760_e32743: f64 = (locals.var_voxm - locals.var_eigbinv_i);
        let assign23760_e32745: f64 = (assign23760_e32743 / locals.var_nigbinv_i);
        let assign23760_e32747: f64 = (assign23760_e32745 / locals.var_vt);
        (assign23760_e32747, ((locals.var_voxm_dn0 / locals.var_nigbinv_i) / locals.var_vt), ((locals.var_voxm_dn2 / locals.var_nigbinv_i) / locals.var_vt), ((locals.var_voxm_dn3 / locals.var_nigbinv_i) / locals.var_vt), ((((locals.var_voxm_dn4 / locals.var_nigbinv_i) * locals.var_vt) - (assign23760_e32745 * locals.var_vt_dn4)) / (locals.var_vt * locals.var_vt)), ((locals.var_voxm_dn5 / locals.var_nigbinv_i) / locals.var_vt), ((locals.var_voxm_dn6 / locals.var_nigbinv_i) / locals.var_vt), ((locals.var_voxm_dn7 / locals.var_nigbinv_i) / locals.var_vt), ((locals.var_voxm_dn8 / locals.var_nigbinv_i) / locals.var_vt), ((locals.var_voxm_dn9 / locals.var_nigbinv_i) / locals.var_vt), ((locals.var_voxm_dn10 / locals.var_nigbinv_i) / locals.var_vt), ((locals.var_voxm_dn11 / locals.var_nigbinv_i) / locals.var_vt), ((locals.var_voxm_dn12 / locals.var_nigbinv_i) / locals.var_vt), ((locals.var_voxm_dn13 / locals.var_nigbinv_i) / locals.var_vt), ((locals.var_voxm_dn14 / locals.var_nigbinv_i) / locals.var_vt),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23760_e32749;
        locals.var_t1_dn0 = assign23760_e32749_d_n0;
        locals.var_t1_dn2 = assign23760_e32749_d_n2;
        locals.var_t1_dn3 = assign23760_e32749_d_n3;
        locals.var_t1_dn4 = assign23760_e32749_d_n4;
        locals.var_t1_dn5 = assign23760_e32749_d_n5;
        locals.var_t1_dn6 = assign23760_e32749_d_n6;
        locals.var_t1_dn7 = assign23760_e32749_d_n7;
        locals.var_t1_dn8 = assign23760_e32749_d_n8;
        locals.var_t1_dn9 = assign23760_e32749_d_n9;
        locals.var_t1_dn10 = assign23760_e32749_d_n10;
        locals.var_t1_dn11 = assign23760_e32749_d_n11;
        locals.var_t1_dn12 = assign23760_e32749_d_n12;
        locals.var_t1_dn13 = assign23760_e32749_d_n13;
        locals.var_t1_dn14 = assign23760_e32749_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign23780_e32802, assign23780_e32802_d_n0, assign23780_e32802_d_n2, assign23780_e32802_d_n3, assign23780_e32802_d_n4, assign23780_e32802_d_n5, assign23780_e32802_d_n6, assign23780_e32802_d_n7, assign23780_e32802_d_n8, assign23780_e32802_d_n9, assign23780_e32802_d_n10, assign23780_e32802_d_n11, assign23780_e32802_d_n12, assign23780_e32802_d_n13, assign23780_e32802_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard602 != 0.0)) {
        let assign23780_e32799: f64 = (locals.var_bigbinv_i * locals.var_voxminv);
        let assign23780_e32800: f64 = (locals.var_aigbinv_i - assign23780_e32799);
        (assign23780_e32800, (-(locals.var_bigbinv_i * locals.var_voxminv_dn0)), (-(locals.var_bigbinv_i * locals.var_voxminv_dn2)), (-(locals.var_bigbinv_i * locals.var_voxminv_dn3)), (-(locals.var_bigbinv_i * locals.var_voxminv_dn4)), (-(locals.var_bigbinv_i * locals.var_voxminv_dn5)), (-(locals.var_bigbinv_i * locals.var_voxminv_dn6)), (-(locals.var_bigbinv_i * locals.var_voxminv_dn7)), (-(locals.var_bigbinv_i * locals.var_voxminv_dn8)), (-(locals.var_bigbinv_i * locals.var_voxminv_dn9)), (-(locals.var_bigbinv_i * locals.var_voxminv_dn10)), (-(locals.var_bigbinv_i * locals.var_voxminv_dn11)), (-(locals.var_bigbinv_i * locals.var_voxminv_dn12)), (-(locals.var_bigbinv_i * locals.var_voxminv_dn13)), (-(locals.var_bigbinv_i * locals.var_voxminv_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign23780_e32802;
        locals.var_t2_dn0 = assign23780_e32802_d_n0;
        locals.var_t2_dn2 = assign23780_e32802_d_n2;
        locals.var_t2_dn3 = assign23780_e32802_d_n3;
        locals.var_t2_dn4 = assign23780_e32802_d_n4;
        locals.var_t2_dn5 = assign23780_e32802_d_n5;
        locals.var_t2_dn6 = assign23780_e32802_d_n6;
        locals.var_t2_dn7 = assign23780_e32802_d_n7;
        locals.var_t2_dn8 = assign23780_e32802_d_n8;
        locals.var_t2_dn9 = assign23780_e32802_d_n9;
        locals.var_t2_dn10 = assign23780_e32802_d_n10;
        locals.var_t2_dn11 = assign23780_e32802_d_n11;
        locals.var_t2_dn12 = assign23780_e32802_d_n12;
        locals.var_t2_dn13 = assign23780_e32802_d_n13;
        locals.var_t2_dn14 = assign23780_e32802_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign23790_e32812, assign23790_e32812_d_n0, assign23790_e32812_d_n2, assign23790_e32812_d_n3, assign23790_e32812_d_n4, assign23790_e32812_d_n5, assign23790_e32812_d_n6, assign23790_e32812_d_n7, assign23790_e32812_d_n8, assign23790_e32812_d_n9, assign23790_e32812_d_n10, assign23790_e32812_d_n11, assign23790_e32812_d_n12, assign23790_e32812_d_n13, assign23790_e32812_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard602 != 0.0)) {
        let assign23790_e32809: f64 = (locals.var_cigbinv_i * locals.var_voxminv);
        let assign23790_e32810: f64 = (1.0 + assign23790_e32809);
        (assign23790_e32810, (locals.var_cigbinv_i * locals.var_voxminv_dn0), (locals.var_cigbinv_i * locals.var_voxminv_dn2), (locals.var_cigbinv_i * locals.var_voxminv_dn3), (locals.var_cigbinv_i * locals.var_voxminv_dn4), (locals.var_cigbinv_i * locals.var_voxminv_dn5), (locals.var_cigbinv_i * locals.var_voxminv_dn6), (locals.var_cigbinv_i * locals.var_voxminv_dn7), (locals.var_cigbinv_i * locals.var_voxminv_dn8), (locals.var_cigbinv_i * locals.var_voxminv_dn9), (locals.var_cigbinv_i * locals.var_voxminv_dn10), (locals.var_cigbinv_i * locals.var_voxminv_dn11), (locals.var_cigbinv_i * locals.var_voxminv_dn12), (locals.var_cigbinv_i * locals.var_voxminv_dn13), (locals.var_cigbinv_i * locals.var_voxminv_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign23790_e32812;
        locals.var_t3_dn0 = assign23790_e32812_d_n0;
        locals.var_t3_dn2 = assign23790_e32812_d_n2;
        locals.var_t3_dn3 = assign23790_e32812_d_n3;
        locals.var_t3_dn4 = assign23790_e32812_d_n4;
        locals.var_t3_dn5 = assign23790_e32812_d_n5;
        locals.var_t3_dn6 = assign23790_e32812_d_n6;
        locals.var_t3_dn7 = assign23790_e32812_d_n7;
        locals.var_t3_dn8 = assign23790_e32812_d_n8;
        locals.var_t3_dn9 = assign23790_e32812_d_n9;
        locals.var_t3_dn10 = assign23790_e32812_d_n10;
        locals.var_t3_dn11 = assign23790_e32812_d_n11;
        locals.var_t3_dn12 = assign23790_e32812_d_n12;
        locals.var_t3_dn13 = assign23790_e32812_d_n13;
        locals.var_t3_dn14 = assign23790_e32812_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign23800_e32825, assign23800_e32825_d_n0, assign23800_e32825_d_n2, assign23800_e32825_d_n3, assign23800_e32825_d_n4, assign23800_e32825_d_n5, assign23800_e32825_d_n6, assign23800_e32825_d_n7, assign23800_e32825_d_n8, assign23800_e32825_d_n9, assign23800_e32825_d_n10, assign23800_e32825_d_n11, assign23800_e32825_d_n12, assign23800_e32825_d_n13, assign23800_e32825_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard602 != 0.0)) {
        let assign23800_e32817: f64 = (-982222000000.0);
        let assign23800_e32819: f64 = (assign23800_e32817 * p.p77);
        let assign23800_e32821: f64 = (assign23800_e32819 * locals.var_t2);
        let assign23800_e32823: f64 = (assign23800_e32821 * locals.var_t3);
        (assign23800_e32823, (((assign23800_e32819 * locals.var_t2_dn0) * locals.var_t3) + (assign23800_e32821 * locals.var_t3_dn0)), (((assign23800_e32819 * locals.var_t2_dn2) * locals.var_t3) + (assign23800_e32821 * locals.var_t3_dn2)), (((assign23800_e32819 * locals.var_t2_dn3) * locals.var_t3) + (assign23800_e32821 * locals.var_t3_dn3)), (((assign23800_e32819 * locals.var_t2_dn4) * locals.var_t3) + (assign23800_e32821 * locals.var_t3_dn4)), (((assign23800_e32819 * locals.var_t2_dn5) * locals.var_t3) + (assign23800_e32821 * locals.var_t3_dn5)), (((assign23800_e32819 * locals.var_t2_dn6) * locals.var_t3) + (assign23800_e32821 * locals.var_t3_dn6)), (((assign23800_e32819 * locals.var_t2_dn7) * locals.var_t3) + (assign23800_e32821 * locals.var_t3_dn7)), (((assign23800_e32819 * locals.var_t2_dn8) * locals.var_t3) + (assign23800_e32821 * locals.var_t3_dn8)), (((assign23800_e32819 * locals.var_t2_dn9) * locals.var_t3) + (assign23800_e32821 * locals.var_t3_dn9)), (((assign23800_e32819 * locals.var_t2_dn10) * locals.var_t3) + (assign23800_e32821 * locals.var_t3_dn10)), (((assign23800_e32819 * locals.var_t2_dn11) * locals.var_t3) + (assign23800_e32821 * locals.var_t3_dn11)), (((assign23800_e32819 * locals.var_t2_dn12) * locals.var_t3) + (assign23800_e32821 * locals.var_t3_dn12)), (((assign23800_e32819 * locals.var_t2_dn13) * locals.var_t3) + (assign23800_e32821 * locals.var_t3_dn13)), (((assign23800_e32819 * locals.var_t2_dn14) * locals.var_t3) + (assign23800_e32821 * locals.var_t3_dn14)),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign23800_e32825;
        locals.var_t4_dn0 = assign23800_e32825_d_n0;
        locals.var_t4_dn2 = assign23800_e32825_d_n2;
        locals.var_t4_dn3 = assign23800_e32825_d_n3;
        locals.var_t4_dn4 = assign23800_e32825_d_n4;
        locals.var_t4_dn5 = assign23800_e32825_d_n5;
        locals.var_t4_dn6 = assign23800_e32825_d_n6;
        locals.var_t4_dn7 = assign23800_e32825_d_n7;
        locals.var_t4_dn8 = assign23800_e32825_d_n8;
        locals.var_t4_dn9 = assign23800_e32825_d_n9;
        locals.var_t4_dn10 = assign23800_e32825_d_n10;
        locals.var_t4_dn11 = assign23800_e32825_d_n11;
        locals.var_t4_dn12 = assign23800_e32825_d_n12;
        locals.var_t4_dn13 = assign23800_e32825_d_n13;
        locals.var_t4_dn14 = assign23800_e32825_d_n14;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_70(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23810_e32832, assign23810_e32832_d_n0, assign23810_e32832_d_n2, assign23810_e32832_d_n3, assign23810_e32832_d_n4, assign23810_e32832_d_n5, assign23810_e32832_d_n6, assign23810_e32832_d_n7, assign23810_e32832_d_n8, assign23810_e32832_d_n9, assign23810_e32832_d_n10, assign23810_e32832_d_n11, assign23810_e32832_d_n12, assign23810_e32832_d_n13, assign23810_e32832_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard602 != 0.0)) {
        let assign23810_e32830: f64 = { let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign23810_e32830, ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn0), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn2), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn3), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn4), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn5), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn6), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn7), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn8), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn9), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn10), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn11), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn12), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn13), ({ let limited_exp_arg = locals.var_t4; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t4_dn14),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign23810_e32832;
        locals.var_t5_dn0 = assign23810_e32832_d_n0;
        locals.var_t5_dn2 = assign23810_e32832_d_n2;
        locals.var_t5_dn3 = assign23810_e32832_d_n3;
        locals.var_t5_dn4 = assign23810_e32832_d_n4;
        locals.var_t5_dn5 = assign23810_e32832_d_n5;
        locals.var_t5_dn6 = assign23810_e32832_d_n6;
        locals.var_t5_dn7 = assign23810_e32832_d_n7;
        locals.var_t5_dn8 = assign23810_e32832_d_n8;
        locals.var_t5_dn9 = assign23810_e32832_d_n9;
        locals.var_t5_dn10 = assign23810_e32832_d_n10;
        locals.var_t5_dn11 = assign23810_e32832_d_n11;
        locals.var_t5_dn12 = assign23810_e32832_d_n12;
        locals.var_t5_dn13 = assign23810_e32832_d_n13;
        locals.var_t5_dn14 = assign23810_e32832_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign23820_e32838, assign23820_e32838_d_n0, assign23820_e32838_d_n2, assign23820_e32838_d_n3, assign23820_e32838_d_n4, assign23820_e32838_d_n5, assign23820_e32838_d_n6, assign23820_e32838_d_n7, assign23820_e32838_d_n8, assign23820_e32838_d_n9, assign23820_e32838_d_n10, assign23820_e32838_d_n11, assign23820_e32838_d_n12, assign23820_e32838_d_n13, assign23820_e32838_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard602 != 0.0)) {
        (3.75956e-7, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign23820_e32838;
        locals.var_t6_dn0 = assign23820_e32838_d_n0;
        locals.var_t6_dn2 = assign23820_e32838_d_n2;
        locals.var_t6_dn3 = assign23820_e32838_d_n3;
        locals.var_t6_dn4 = assign23820_e32838_d_n4;
        locals.var_t6_dn5 = assign23820_e32838_d_n5;
        locals.var_t6_dn6 = assign23820_e32838_d_n6;
        locals.var_t6_dn7 = assign23820_e32838_d_n7;
        locals.var_t6_dn8 = assign23820_e32838_d_n8;
        locals.var_t6_dn9 = assign23820_e32838_d_n9;
        locals.var_t6_dn10 = assign23820_e32838_d_n10;
        locals.var_t6_dn11 = assign23820_e32838_d_n11;
        locals.var_t6_dn12 = assign23820_e32838_d_n12;
        locals.var_t6_dn13 = assign23820_e32838_d_n13;
        locals.var_t6_dn14 = assign23820_e32838_d_n14;
        locals.var_t6_rv = 0.0;

        let assign23860_e32877: f64 = if p.p46 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard603 = assign23860_e32877;
        locals.var_guard603_rv = 0.0;

        let (assign23870_e32887, assign23870_e32887_d_n0, assign23870_e32887_d_n2, assign23870_e32887_d_n3, assign23870_e32887_d_n4, assign23870_e32887_d_n5, assign23870_e32887_d_n6, assign23870_e32887_d_n7, assign23870_e32887_d_n8, assign23870_e32887_d_n9, assign23870_e32887_d_n10, assign23870_e32887_d_n11, assign23870_e32887_d_n12, assign23870_e32887_d_n13, assign23870_e32887_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) {
        let assign23870_e32884: f64 = (locals.var_bigc_i * locals.var_voxminv);
        let assign23870_e32885: f64 = (locals.var_aigc_i - assign23870_e32884);
        (assign23870_e32885, (-(locals.var_bigc_i * locals.var_voxminv_dn0)), (-(locals.var_bigc_i * locals.var_voxminv_dn2)), (-(locals.var_bigc_i * locals.var_voxminv_dn3)), (-(locals.var_bigc_i * locals.var_voxminv_dn4)), (-(locals.var_bigc_i * locals.var_voxminv_dn5)), (-(locals.var_bigc_i * locals.var_voxminv_dn6)), (-(locals.var_bigc_i * locals.var_voxminv_dn7)), (-(locals.var_bigc_i * locals.var_voxminv_dn8)), (-(locals.var_bigc_i * locals.var_voxminv_dn9)), (-(locals.var_bigc_i * locals.var_voxminv_dn10)), (-(locals.var_bigc_i * locals.var_voxminv_dn11)), (-(locals.var_bigc_i * locals.var_voxminv_dn12)), (-(locals.var_bigc_i * locals.var_voxminv_dn13)), (-(locals.var_bigc_i * locals.var_voxminv_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23870_e32887;
        locals.var_t1_dn0 = assign23870_e32887_d_n0;
        locals.var_t1_dn2 = assign23870_e32887_d_n2;
        locals.var_t1_dn3 = assign23870_e32887_d_n3;
        locals.var_t1_dn4 = assign23870_e32887_d_n4;
        locals.var_t1_dn5 = assign23870_e32887_d_n5;
        locals.var_t1_dn6 = assign23870_e32887_d_n6;
        locals.var_t1_dn7 = assign23870_e32887_d_n7;
        locals.var_t1_dn8 = assign23870_e32887_d_n8;
        locals.var_t1_dn9 = assign23870_e32887_d_n9;
        locals.var_t1_dn10 = assign23870_e32887_d_n10;
        locals.var_t1_dn11 = assign23870_e32887_d_n11;
        locals.var_t1_dn12 = assign23870_e32887_d_n12;
        locals.var_t1_dn13 = assign23870_e32887_d_n13;
        locals.var_t1_dn14 = assign23870_e32887_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign23880_e32897, assign23880_e32897_d_n0, assign23880_e32897_d_n2, assign23880_e32897_d_n3, assign23880_e32897_d_n4, assign23880_e32897_d_n5, assign23880_e32897_d_n6, assign23880_e32897_d_n7, assign23880_e32897_d_n8, assign23880_e32897_d_n9, assign23880_e32897_d_n10, assign23880_e32897_d_n11, assign23880_e32897_d_n12, assign23880_e32897_d_n13, assign23880_e32897_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) {
        let assign23880_e32894: f64 = (locals.var_cigc_i * locals.var_voxminv);
        let assign23880_e32895: f64 = (1.0 + assign23880_e32894);
        (assign23880_e32895, (locals.var_cigc_i * locals.var_voxminv_dn0), (locals.var_cigc_i * locals.var_voxminv_dn2), (locals.var_cigc_i * locals.var_voxminv_dn3), (locals.var_cigc_i * locals.var_voxminv_dn4), (locals.var_cigc_i * locals.var_voxminv_dn5), (locals.var_cigc_i * locals.var_voxminv_dn6), (locals.var_cigc_i * locals.var_voxminv_dn7), (locals.var_cigc_i * locals.var_voxminv_dn8), (locals.var_cigc_i * locals.var_voxminv_dn9), (locals.var_cigc_i * locals.var_voxminv_dn10), (locals.var_cigc_i * locals.var_voxminv_dn11), (locals.var_cigc_i * locals.var_voxminv_dn12), (locals.var_cigc_i * locals.var_voxminv_dn13), (locals.var_cigc_i * locals.var_voxminv_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign23880_e32897;
        locals.var_t2_dn0 = assign23880_e32897_d_n0;
        locals.var_t2_dn2 = assign23880_e32897_d_n2;
        locals.var_t2_dn3 = assign23880_e32897_d_n3;
        locals.var_t2_dn4 = assign23880_e32897_d_n4;
        locals.var_t2_dn5 = assign23880_e32897_d_n5;
        locals.var_t2_dn6 = assign23880_e32897_d_n6;
        locals.var_t2_dn7 = assign23880_e32897_d_n7;
        locals.var_t2_dn8 = assign23880_e32897_d_n8;
        locals.var_t2_dn9 = assign23880_e32897_d_n9;
        locals.var_t2_dn10 = assign23880_e32897_d_n10;
        locals.var_t2_dn11 = assign23880_e32897_d_n11;
        locals.var_t2_dn12 = assign23880_e32897_d_n12;
        locals.var_t2_dn13 = assign23880_e32897_d_n13;
        locals.var_t2_dn14 = assign23880_e32897_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign23890_e32907, assign23890_e32907_d_n0, assign23890_e32907_d_n2, assign23890_e32907_d_n3, assign23890_e32907_d_n4, assign23890_e32907_d_n5, assign23890_e32907_d_n6, assign23890_e32907_d_n7, assign23890_e32907_d_n8, assign23890_e32907_d_n9, assign23890_e32907_d_n10, assign23890_e32907_d_n11, assign23890_e32907_d_n12, assign23890_e32907_d_n13, assign23890_e32907_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) {
        let assign23890_e32903: f64 = (locals.var_bechvb * locals.var_t1);
        let assign23890_e32905: f64 = (assign23890_e32903 * locals.var_t2);
        (assign23890_e32905, (((locals.var_bechvb * locals.var_t1_dn0) * locals.var_t2) + (assign23890_e32903 * locals.var_t2_dn0)), (((locals.var_bechvb * locals.var_t1_dn2) * locals.var_t2) + (assign23890_e32903 * locals.var_t2_dn2)), (((locals.var_bechvb * locals.var_t1_dn3) * locals.var_t2) + (assign23890_e32903 * locals.var_t2_dn3)), (((locals.var_bechvb * locals.var_t1_dn4) * locals.var_t2) + (assign23890_e32903 * locals.var_t2_dn4)), (((locals.var_bechvb * locals.var_t1_dn5) * locals.var_t2) + (assign23890_e32903 * locals.var_t2_dn5)), (((locals.var_bechvb * locals.var_t1_dn6) * locals.var_t2) + (assign23890_e32903 * locals.var_t2_dn6)), (((locals.var_bechvb * locals.var_t1_dn7) * locals.var_t2) + (assign23890_e32903 * locals.var_t2_dn7)), (((locals.var_bechvb * locals.var_t1_dn8) * locals.var_t2) + (assign23890_e32903 * locals.var_t2_dn8)), (((locals.var_bechvb * locals.var_t1_dn9) * locals.var_t2) + (assign23890_e32903 * locals.var_t2_dn9)), (((locals.var_bechvb * locals.var_t1_dn10) * locals.var_t2) + (assign23890_e32903 * locals.var_t2_dn10)), (((locals.var_bechvb * locals.var_t1_dn11) * locals.var_t2) + (assign23890_e32903 * locals.var_t2_dn11)), (((locals.var_bechvb * locals.var_t1_dn12) * locals.var_t2) + (assign23890_e32903 * locals.var_t2_dn12)), (((locals.var_bechvb * locals.var_t1_dn13) * locals.var_t2) + (assign23890_e32903 * locals.var_t2_dn13)), (((locals.var_bechvb * locals.var_t1_dn14) * locals.var_t2) + (assign23890_e32903 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign23890_e32907;
        locals.var_t3_dn0 = assign23890_e32907_d_n0;
        locals.var_t3_dn2 = assign23890_e32907_d_n2;
        locals.var_t3_dn3 = assign23890_e32907_d_n3;
        locals.var_t3_dn4 = assign23890_e32907_d_n4;
        locals.var_t3_dn5 = assign23890_e32907_d_n5;
        locals.var_t3_dn6 = assign23890_e32907_d_n6;
        locals.var_t3_dn7 = assign23890_e32907_d_n7;
        locals.var_t3_dn8 = assign23890_e32907_d_n8;
        locals.var_t3_dn9 = assign23890_e32907_d_n9;
        locals.var_t3_dn10 = assign23890_e32907_d_n10;
        locals.var_t3_dn11 = assign23890_e32907_d_n11;
        locals.var_t3_dn12 = assign23890_e32907_d_n12;
        locals.var_t3_dn13 = assign23890_e32907_d_n13;
        locals.var_t3_dn14 = assign23890_e32907_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign23900_e32922, assign23900_e32922_d_n0, assign23900_e32922_d_n2, assign23900_e32922_d_n3, assign23900_e32922_d_n4, assign23900_e32922_d_n5, assign23900_e32922_d_n6, assign23900_e32922_d_n7, assign23900_e32922_d_n8, assign23900_e32922_d_n9, assign23900_e32922_d_n10, assign23900_e32922_d_n11, assign23900_e32922_d_n12, assign23900_e32922_d_n13, assign23900_e32922_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) {
        let assign23900_e32913: f64 = (locals.var_nq * locals.var_nvt);
        let assign23900_e32916: f64 = (locals.var_qs_1 + locals.var_qdeff);
        let assign23900_e32917: f64 = (assign23900_e32913 * assign23900_e32916);
        let assign23900_e32919: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign23900_e32920: f64 = (assign23900_e32917 * assign23900_e32919);
        (assign23900_e32920, ((((((locals.var_nq_dn0 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn0)) * assign23900_e32916) + (assign23900_e32913 * (locals.var_qs_1_dn0 + locals.var_qdeff_dn0))) * assign23900_e32919) + (assign23900_e32917 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn0))), ((((((locals.var_nq_dn2 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn2)) * assign23900_e32916) + (assign23900_e32913 * (locals.var_qs_1_dn2 + locals.var_qdeff_dn2))) * assign23900_e32919) + (assign23900_e32917 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn2))), ((((((locals.var_nq_dn3 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn3)) * assign23900_e32916) + (assign23900_e32913 * (locals.var_qs_1_dn3 + locals.var_qdeff_dn3))) * assign23900_e32919) + (assign23900_e32917 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3))), ((((((locals.var_nq_dn4 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn4)) * assign23900_e32916) + (assign23900_e32913 * (locals.var_qs_1_dn4 + locals.var_qdeff_dn4))) * assign23900_e32919) + (assign23900_e32917 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4))), ((((((locals.var_nq_dn5 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn5)) * assign23900_e32916) + (assign23900_e32913 * (locals.var_qs_1_dn5 + locals.var_qdeff_dn5))) * assign23900_e32919) + (assign23900_e32917 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5))), ((((((locals.var_nq_dn6 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn6)) * assign23900_e32916) + (assign23900_e32913 * (locals.var_qs_1_dn6 + locals.var_qdeff_dn6))) * assign23900_e32919) + (assign23900_e32917 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6))), ((((((locals.var_nq_dn7 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn7)) * assign23900_e32916) + (assign23900_e32913 * (locals.var_qs_1_dn7 + locals.var_qdeff_dn7))) * assign23900_e32919) + (assign23900_e32917 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7))), ((((((locals.var_nq_dn8 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn8)) * assign23900_e32916) + (assign23900_e32913 * (locals.var_qs_1_dn8 + locals.var_qdeff_dn8))) * assign23900_e32919) + (assign23900_e32917 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8))), ((((((locals.var_nq_dn9 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn9)) * assign23900_e32916) + (assign23900_e32913 * (locals.var_qs_1_dn9 + locals.var_qdeff_dn9))) * assign23900_e32919) + (assign23900_e32917 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9))), ((((((locals.var_nq_dn10 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn10)) * assign23900_e32916) + (assign23900_e32913 * (locals.var_qs_1_dn10 + locals.var_qdeff_dn10))) * assign23900_e32919) + (assign23900_e32917 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10))), ((((((locals.var_nq_dn11 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn11)) * assign23900_e32916) + (assign23900_e32913 * (locals.var_qs_1_dn11 + locals.var_qdeff_dn11))) * assign23900_e32919) + (assign23900_e32917 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11))), ((((((locals.var_nq_dn12 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn12)) * assign23900_e32916) + (assign23900_e32913 * (locals.var_qs_1_dn12 + locals.var_qdeff_dn12))) * assign23900_e32919) + (assign23900_e32917 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn12))), ((((((locals.var_nq_dn13 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn13)) * assign23900_e32916) + (assign23900_e32913 * (locals.var_qs_1_dn13 + locals.var_qdeff_dn13))) * assign23900_e32919) + (assign23900_e32917 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn13))), ((((((locals.var_nq_dn14 * locals.var_nvt) + (locals.var_nq * locals.var_nvt_dn14)) * assign23900_e32916) + (assign23900_e32913 * (locals.var_qs_1_dn14 + locals.var_qdeff_dn14))) * assign23900_e32919) + (assign23900_e32917 * ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign23900_e32922;
        locals.var_t4_dn0 = assign23900_e32922_d_n0;
        locals.var_t4_dn2 = assign23900_e32922_d_n2;
        locals.var_t4_dn3 = assign23900_e32922_d_n3;
        locals.var_t4_dn4 = assign23900_e32922_d_n4;
        locals.var_t4_dn5 = assign23900_e32922_d_n5;
        locals.var_t4_dn6 = assign23900_e32922_d_n6;
        locals.var_t4_dn7 = assign23900_e32922_d_n7;
        locals.var_t4_dn8 = assign23900_e32922_d_n8;
        locals.var_t4_dn9 = assign23900_e32922_d_n9;
        locals.var_t4_dn10 = assign23900_e32922_d_n10;
        locals.var_t4_dn11 = assign23900_e32922_d_n11;
        locals.var_t4_dn12 = assign23900_e32922_d_n12;
        locals.var_t4_dn13 = assign23900_e32922_d_n13;
        locals.var_t4_dn14 = assign23900_e32922_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign23920_e32959, assign23920_e32959_d_n0, assign23920_e32959_d_n2, assign23920_e32959_d_n3, assign23920_e32959_d_n4, assign23920_e32959_d_n5, assign23920_e32959_d_n6, assign23920_e32959_d_n7, assign23920_e32959_d_n8, assign23920_e32959_d_n9, assign23920_e32959_d_n10, assign23920_e32959_d_n11, assign23920_e32959_d_n12, assign23920_e32959_d_n13, assign23920_e32959_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) {
        let assign23920_e32952: f64 = (locals.var_vdseff * locals.var_vdseff);
        let assign23920_e32954: f64 = (assign23920_e32952 + 0.01);
        let assign23920_e32955: f64 = (assign23920_e32954).sqrt();
        let assign23920_e32957: f64 = (assign23920_e32955 - 0.1);
        (assign23920_e32957, (((locals.var_vdseff_dn0 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn0)) / (2.0 * assign23920_e32955)), (((locals.var_vdseff_dn2 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn2)) / (2.0 * assign23920_e32955)), (((locals.var_vdseff_dn3 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn3)) / (2.0 * assign23920_e32955)), (((locals.var_vdseff_dn4 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn4)) / (2.0 * assign23920_e32955)), (((locals.var_vdseff_dn5 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn5)) / (2.0 * assign23920_e32955)), (((locals.var_vdseff_dn6 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn6)) / (2.0 * assign23920_e32955)), (((locals.var_vdseff_dn7 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn7)) / (2.0 * assign23920_e32955)), (((locals.var_vdseff_dn8 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn8)) / (2.0 * assign23920_e32955)), (((locals.var_vdseff_dn9 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn9)) / (2.0 * assign23920_e32955)), (((locals.var_vdseff_dn10 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn10)) / (2.0 * assign23920_e32955)), (((locals.var_vdseff_dn11 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn11)) / (2.0 * assign23920_e32955)), (((locals.var_vdseff_dn12 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn12)) / (2.0 * assign23920_e32955)), (((locals.var_vdseff_dn13 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn13)) / (2.0 * assign23920_e32955)), (((locals.var_vdseff_dn14 * locals.var_vdseff) + (locals.var_vdseff * locals.var_vdseff_dn14)) / (2.0 * assign23920_e32955)),)
    } else {
        (locals.var_vdseffx, locals.var_vdseffx_dn0, locals.var_vdseffx_dn2, locals.var_vdseffx_dn3, locals.var_vdseffx_dn4, locals.var_vdseffx_dn5, locals.var_vdseffx_dn6, locals.var_vdseffx_dn7, locals.var_vdseffx_dn8, locals.var_vdseffx_dn9, locals.var_vdseffx_dn10, locals.var_vdseffx_dn11, locals.var_vdseffx_dn12, locals.var_vdseffx_dn13, locals.var_vdseffx_dn14,)
    }
};
        locals.var_vdseffx = assign23920_e32959;
        locals.var_vdseffx_dn0 = assign23920_e32959_d_n0;
        locals.var_vdseffx_dn2 = assign23920_e32959_d_n2;
        locals.var_vdseffx_dn3 = assign23920_e32959_d_n3;
        locals.var_vdseffx_dn4 = assign23920_e32959_d_n4;
        locals.var_vdseffx_dn5 = assign23920_e32959_d_n5;
        locals.var_vdseffx_dn6 = assign23920_e32959_d_n6;
        locals.var_vdseffx_dn7 = assign23920_e32959_d_n7;
        locals.var_vdseffx_dn8 = assign23920_e32959_d_n8;
        locals.var_vdseffx_dn9 = assign23920_e32959_d_n9;
        locals.var_vdseffx_dn10 = assign23920_e32959_d_n10;
        locals.var_vdseffx_dn11 = assign23920_e32959_d_n11;
        locals.var_vdseffx_dn12 = assign23920_e32959_d_n12;
        locals.var_vdseffx_dn13 = assign23920_e32959_d_n13;
        locals.var_vdseffx_dn14 = assign23920_e32959_d_n14;
        locals.var_vdseffx_rv = 0.0;

        let (assign23930_e32967, assign23930_e32967_d_n0, assign23930_e32967_d_n2, assign23930_e32967_d_n3, assign23930_e32967_d_n4, assign23930_e32967_d_n5, assign23930_e32967_d_n6, assign23930_e32967_d_n7, assign23930_e32967_d_n8, assign23930_e32967_d_n9, assign23930_e32967_d_n10, assign23930_e32967_d_n11, assign23930_e32967_d_n12, assign23930_e32967_d_n13, assign23930_e32967_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) {
        let assign23930_e32965: f64 = (locals.var_pigcd_i * locals.var_vdseffx);
        (assign23930_e32965, (locals.var_pigcd_i * locals.var_vdseffx_dn0), (locals.var_pigcd_i * locals.var_vdseffx_dn2), (locals.var_pigcd_i * locals.var_vdseffx_dn3), (locals.var_pigcd_i * locals.var_vdseffx_dn4), (locals.var_pigcd_i * locals.var_vdseffx_dn5), (locals.var_pigcd_i * locals.var_vdseffx_dn6), (locals.var_pigcd_i * locals.var_vdseffx_dn7), (locals.var_pigcd_i * locals.var_vdseffx_dn8), (locals.var_pigcd_i * locals.var_vdseffx_dn9), (locals.var_pigcd_i * locals.var_vdseffx_dn10), (locals.var_pigcd_i * locals.var_vdseffx_dn11), (locals.var_pigcd_i * locals.var_vdseffx_dn12), (locals.var_pigcd_i * locals.var_vdseffx_dn13), (locals.var_pigcd_i * locals.var_vdseffx_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign23930_e32967;
        locals.var_t1_dn0 = assign23930_e32967_d_n0;
        locals.var_t1_dn2 = assign23930_e32967_d_n2;
        locals.var_t1_dn3 = assign23930_e32967_d_n3;
        locals.var_t1_dn4 = assign23930_e32967_d_n4;
        locals.var_t1_dn5 = assign23930_e32967_d_n5;
        locals.var_t1_dn6 = assign23930_e32967_d_n6;
        locals.var_t1_dn7 = assign23930_e32967_d_n7;
        locals.var_t1_dn8 = assign23930_e32967_d_n8;
        locals.var_t1_dn9 = assign23930_e32967_d_n9;
        locals.var_t1_dn10 = assign23930_e32967_d_n10;
        locals.var_t1_dn11 = assign23930_e32967_d_n11;
        locals.var_t1_dn12 = assign23930_e32967_d_n12;
        locals.var_t1_dn13 = assign23930_e32967_d_n13;
        locals.var_t1_dn14 = assign23930_e32967_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign23940_e32975, assign23940_e32975_d_n0, assign23940_e32975_d_n2, assign23940_e32975_d_n3, assign23940_e32975_d_n4, assign23940_e32975_d_n5, assign23940_e32975_d_n6, assign23940_e32975_d_n7, assign23940_e32975_d_n8, assign23940_e32975_d_n9, assign23940_e32975_d_n10, assign23940_e32975_d_n11, assign23940_e32975_d_n12, assign23940_e32975_d_n13, assign23940_e32975_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) {
        let assign23940_e32972: f64 = (-locals.var_t1);
        let assign23940_e32973: f64 = { let limited_exp_arg = assign23940_e32972; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign23940_e32973, ({ let limited_exp_arg = assign23940_e32972; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn0)), ({ let limited_exp_arg = assign23940_e32972; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn2)), ({ let limited_exp_arg = assign23940_e32972; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn3)), ({ let limited_exp_arg = assign23940_e32972; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn4)), ({ let limited_exp_arg = assign23940_e32972; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn5)), ({ let limited_exp_arg = assign23940_e32972; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn6)), ({ let limited_exp_arg = assign23940_e32972; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn7)), ({ let limited_exp_arg = assign23940_e32972; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn8)), ({ let limited_exp_arg = assign23940_e32972; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn9)), ({ let limited_exp_arg = assign23940_e32972; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn10)), ({ let limited_exp_arg = assign23940_e32972; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn11)), ({ let limited_exp_arg = assign23940_e32972; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn12)), ({ let limited_exp_arg = assign23940_e32972; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn13)), ({ let limited_exp_arg = assign23940_e32972; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn14)),)
    } else {
        (locals.var_t1_exp, locals.var_t1_exp_dn0, locals.var_t1_exp_dn2, locals.var_t1_exp_dn3, locals.var_t1_exp_dn4, locals.var_t1_exp_dn5, locals.var_t1_exp_dn6, locals.var_t1_exp_dn7, locals.var_t1_exp_dn8, locals.var_t1_exp_dn9, locals.var_t1_exp_dn10, locals.var_t1_exp_dn11, locals.var_t1_exp_dn12, locals.var_t1_exp_dn13, locals.var_t1_exp_dn14,)
    }
};
        locals.var_t1_exp = assign23940_e32975;
        locals.var_t1_exp_dn0 = assign23940_e32975_d_n0;
        locals.var_t1_exp_dn2 = assign23940_e32975_d_n2;
        locals.var_t1_exp_dn3 = assign23940_e32975_d_n3;
        locals.var_t1_exp_dn4 = assign23940_e32975_d_n4;
        locals.var_t1_exp_dn5 = assign23940_e32975_d_n5;
        locals.var_t1_exp_dn6 = assign23940_e32975_d_n6;
        locals.var_t1_exp_dn7 = assign23940_e32975_d_n7;
        locals.var_t1_exp_dn8 = assign23940_e32975_d_n8;
        locals.var_t1_exp_dn9 = assign23940_e32975_d_n9;
        locals.var_t1_exp_dn10 = assign23940_e32975_d_n10;
        locals.var_t1_exp_dn11 = assign23940_e32975_d_n11;
        locals.var_t1_exp_dn12 = assign23940_e32975_d_n12;
        locals.var_t1_exp_dn13 = assign23940_e32975_d_n13;
        locals.var_t1_exp_dn14 = assign23940_e32975_d_n14;
        locals.var_t1_exp_rv = 0.0;

        let (assign23950_e32987, assign23950_e32987_d_n0, assign23950_e32987_d_n2, assign23950_e32987_d_n3, assign23950_e32987_d_n4, assign23950_e32987_d_n5, assign23950_e32987_d_n6, assign23950_e32987_d_n7, assign23950_e32987_d_n8, assign23950_e32987_d_n9, assign23950_e32987_d_n10, assign23950_e32987_d_n11, assign23950_e32987_d_n12, assign23950_e32987_d_n13, assign23950_e32987_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) {
        let assign23950_e32981: f64 = (locals.var_t1 + locals.var_t1_exp);
        let assign23950_e32983: f64 = (assign23950_e32981 - 1.0);
        let assign23950_e32985: f64 = (assign23950_e32983 + 0.0001);
        (assign23950_e32985, (locals.var_t1_dn0 + locals.var_t1_exp_dn0), (locals.var_t1_dn2 + locals.var_t1_exp_dn2), (locals.var_t1_dn3 + locals.var_t1_exp_dn3), (locals.var_t1_dn4 + locals.var_t1_exp_dn4), (locals.var_t1_dn5 + locals.var_t1_exp_dn5), (locals.var_t1_dn6 + locals.var_t1_exp_dn6), (locals.var_t1_dn7 + locals.var_t1_exp_dn7), (locals.var_t1_dn8 + locals.var_t1_exp_dn8), (locals.var_t1_dn9 + locals.var_t1_exp_dn9), (locals.var_t1_dn10 + locals.var_t1_exp_dn10), (locals.var_t1_dn11 + locals.var_t1_exp_dn11), (locals.var_t1_dn12 + locals.var_t1_exp_dn12), (locals.var_t1_dn13 + locals.var_t1_exp_dn13), (locals.var_t1_dn14 + locals.var_t1_exp_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign23950_e32987;
        locals.var_t3_dn0 = assign23950_e32987_d_n0;
        locals.var_t3_dn2 = assign23950_e32987_d_n2;
        locals.var_t3_dn3 = assign23950_e32987_d_n3;
        locals.var_t3_dn4 = assign23950_e32987_d_n4;
        locals.var_t3_dn5 = assign23950_e32987_d_n5;
        locals.var_t3_dn6 = assign23950_e32987_d_n6;
        locals.var_t3_dn7 = assign23950_e32987_d_n7;
        locals.var_t3_dn8 = assign23950_e32987_d_n8;
        locals.var_t3_dn9 = assign23950_e32987_d_n9;
        locals.var_t3_dn10 = assign23950_e32987_d_n10;
        locals.var_t3_dn11 = assign23950_e32987_d_n11;
        locals.var_t3_dn12 = assign23950_e32987_d_n12;
        locals.var_t3_dn13 = assign23950_e32987_d_n13;
        locals.var_t3_dn14 = assign23950_e32987_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign23960_e33001, assign23960_e33001_d_n0, assign23960_e33001_d_n2, assign23960_e33001_d_n3, assign23960_e33001_d_n4, assign23960_e33001_d_n5, assign23960_e33001_d_n6, assign23960_e33001_d_n7, assign23960_e33001_d_n8, assign23960_e33001_d_n9, assign23960_e33001_d_n10, assign23960_e33001_d_n11, assign23960_e33001_d_n12, assign23960_e33001_d_n13, assign23960_e33001_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) {
        let assign23960_e32994: f64 = (locals.var_t1 + 1.0);
        let assign23960_e32996: f64 = (assign23960_e32994 * locals.var_t1_exp);
        let assign23960_e32997: f64 = (1.0 - assign23960_e32996);
        let assign23960_e32999: f64 = (assign23960_e32997 + 0.0001);
        (assign23960_e32999, (-((locals.var_t1_dn0 * locals.var_t1_exp) + (assign23960_e32994 * locals.var_t1_exp_dn0))), (-((locals.var_t1_dn2 * locals.var_t1_exp) + (assign23960_e32994 * locals.var_t1_exp_dn2))), (-((locals.var_t1_dn3 * locals.var_t1_exp) + (assign23960_e32994 * locals.var_t1_exp_dn3))), (-((locals.var_t1_dn4 * locals.var_t1_exp) + (assign23960_e32994 * locals.var_t1_exp_dn4))), (-((locals.var_t1_dn5 * locals.var_t1_exp) + (assign23960_e32994 * locals.var_t1_exp_dn5))), (-((locals.var_t1_dn6 * locals.var_t1_exp) + (assign23960_e32994 * locals.var_t1_exp_dn6))), (-((locals.var_t1_dn7 * locals.var_t1_exp) + (assign23960_e32994 * locals.var_t1_exp_dn7))), (-((locals.var_t1_dn8 * locals.var_t1_exp) + (assign23960_e32994 * locals.var_t1_exp_dn8))), (-((locals.var_t1_dn9 * locals.var_t1_exp) + (assign23960_e32994 * locals.var_t1_exp_dn9))), (-((locals.var_t1_dn10 * locals.var_t1_exp) + (assign23960_e32994 * locals.var_t1_exp_dn10))), (-((locals.var_t1_dn11 * locals.var_t1_exp) + (assign23960_e32994 * locals.var_t1_exp_dn11))), (-((locals.var_t1_dn12 * locals.var_t1_exp) + (assign23960_e32994 * locals.var_t1_exp_dn12))), (-((locals.var_t1_dn13 * locals.var_t1_exp) + (assign23960_e32994 * locals.var_t1_exp_dn13))), (-((locals.var_t1_dn14 * locals.var_t1_exp) + (assign23960_e32994 * locals.var_t1_exp_dn14))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign23960_e33001;
        locals.var_t4_dn0 = assign23960_e33001_d_n0;
        locals.var_t4_dn2 = assign23960_e33001_d_n2;
        locals.var_t4_dn3 = assign23960_e33001_d_n3;
        locals.var_t4_dn4 = assign23960_e33001_d_n4;
        locals.var_t4_dn5 = assign23960_e33001_d_n5;
        locals.var_t4_dn6 = assign23960_e33001_d_n6;
        locals.var_t4_dn7 = assign23960_e33001_d_n7;
        locals.var_t4_dn8 = assign23960_e33001_d_n8;
        locals.var_t4_dn9 = assign23960_e33001_d_n9;
        locals.var_t4_dn10 = assign23960_e33001_d_n10;
        locals.var_t4_dn11 = assign23960_e33001_d_n11;
        locals.var_t4_dn12 = assign23960_e33001_d_n12;
        locals.var_t4_dn13 = assign23960_e33001_d_n13;
        locals.var_t4_dn14 = assign23960_e33001_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign23970_e33011, assign23970_e33011_d_n0, assign23970_e33011_d_n2, assign23970_e33011_d_n3, assign23970_e33011_d_n4, assign23970_e33011_d_n5, assign23970_e33011_d_n6, assign23970_e33011_d_n7, assign23970_e33011_d_n8, assign23970_e33011_d_n9, assign23970_e33011_d_n10, assign23970_e33011_d_n11, assign23970_e33011_d_n12, assign23970_e33011_d_n13, assign23970_e33011_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) {
        let assign23970_e33007: f64 = (locals.var_t1 * locals.var_t1);
        let assign23970_e33009: f64 = (assign23970_e33007 + 0.0002);
        (assign23970_e33009, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)), ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign23970_e33011;
        locals.var_t5_dn0 = assign23970_e33011_d_n0;
        locals.var_t5_dn2 = assign23970_e33011_d_n2;
        locals.var_t5_dn3 = assign23970_e33011_d_n3;
        locals.var_t5_dn4 = assign23970_e33011_d_n4;
        locals.var_t5_dn5 = assign23970_e33011_d_n5;
        locals.var_t5_dn6 = assign23970_e33011_d_n6;
        locals.var_t5_dn7 = assign23970_e33011_d_n7;
        locals.var_t5_dn8 = assign23970_e33011_d_n8;
        locals.var_t5_dn9 = assign23970_e33011_d_n9;
        locals.var_t5_dn10 = assign23970_e33011_d_n10;
        locals.var_t5_dn11 = assign23970_e33011_d_n11;
        locals.var_t5_dn12 = assign23970_e33011_d_n12;
        locals.var_t5_dn13 = assign23970_e33011_d_n13;
        locals.var_t5_dn14 = assign23970_e33011_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign24030_e33072, assign24030_e33072_d_n0, assign24030_e33072_d_n2, assign24030_e33072_d_n3, assign24030_e33072_d_n4, assign24030_e33072_d_n5, assign24030_e33072_d_n6, assign24030_e33072_d_n7, assign24030_e33072_d_n8, assign24030_e33072_d_n9, assign24030_e33072_d_n10, assign24030_e33072_d_n11, assign24030_e33072_d_n12, assign24030_e33072_d_n13, assign24030_e33072_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) {
        let assign24030_e33070: f64 = (locals.var_vgs_noswap - locals.var_vfbsdr);
        (assign24030_e33070, 0.0, 0.0, 0.0, (-locals.var_vfbsdr_dn4), locals.var_vgs_noswap_dn5, 0.0, locals.var_vgs_noswap_dn7, 0.0, locals.var_vgs_noswap_dn9, 0.0, locals.var_vgs_noswap_dn11, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign24030_e33072;
        locals.var_t2_dn0 = assign24030_e33072_d_n0;
        locals.var_t2_dn2 = assign24030_e33072_d_n2;
        locals.var_t2_dn3 = assign24030_e33072_d_n3;
        locals.var_t2_dn4 = assign24030_e33072_d_n4;
        locals.var_t2_dn5 = assign24030_e33072_d_n5;
        locals.var_t2_dn6 = assign24030_e33072_d_n6;
        locals.var_t2_dn7 = assign24030_e33072_d_n7;
        locals.var_t2_dn8 = assign24030_e33072_d_n8;
        locals.var_t2_dn9 = assign24030_e33072_d_n9;
        locals.var_t2_dn10 = assign24030_e33072_d_n10;
        locals.var_t2_dn11 = assign24030_e33072_d_n11;
        locals.var_t2_dn12 = assign24030_e33072_d_n12;
        locals.var_t2_dn13 = assign24030_e33072_d_n13;
        locals.var_t2_dn14 = assign24030_e33072_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign24040_e33083, assign24040_e33083_d_n0, assign24040_e33083_d_n2, assign24040_e33083_d_n3, assign24040_e33083_d_n4, assign24040_e33083_d_n5, assign24040_e33083_d_n6, assign24040_e33083_d_n7, assign24040_e33083_d_n8, assign24040_e33083_d_n9, assign24040_e33083_d_n10, assign24040_e33083_d_n11, assign24040_e33083_d_n12, assign24040_e33083_d_n13, assign24040_e33083_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) {
        let assign24040_e33078: f64 = (locals.var_t2 * locals.var_t2);
        let assign24040_e33080: f64 = (assign24040_e33078 + 0.0001);
        let assign24040_e33081: f64 = (assign24040_e33080).sqrt();
        (assign24040_e33081, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign24040_e33081)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign24040_e33081)), (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign24040_e33081)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign24040_e33081)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign24040_e33081)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign24040_e33081)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign24040_e33081)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign24040_e33081)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign24040_e33081)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign24040_e33081)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign24040_e33081)), (((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)) / (2.0 * assign24040_e33081)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign24040_e33081)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign24040_e33081)),)
    } else {
        (locals.var_vgs_eff, locals.var_vgs_eff_dn0, locals.var_vgs_eff_dn2, locals.var_vgs_eff_dn3, locals.var_vgs_eff_dn4, locals.var_vgs_eff_dn5, locals.var_vgs_eff_dn6, locals.var_vgs_eff_dn7, locals.var_vgs_eff_dn8, locals.var_vgs_eff_dn9, locals.var_vgs_eff_dn10, locals.var_vgs_eff_dn11, locals.var_vgs_eff_dn12, locals.var_vgs_eff_dn13, locals.var_vgs_eff_dn14,)
    }
};
        locals.var_vgs_eff = assign24040_e33083;
        locals.var_vgs_eff_dn0 = assign24040_e33083_d_n0;
        locals.var_vgs_eff_dn2 = assign24040_e33083_d_n2;
        locals.var_vgs_eff_dn3 = assign24040_e33083_d_n3;
        locals.var_vgs_eff_dn4 = assign24040_e33083_d_n4;
        locals.var_vgs_eff_dn5 = assign24040_e33083_d_n5;
        locals.var_vgs_eff_dn6 = assign24040_e33083_d_n6;
        locals.var_vgs_eff_dn7 = assign24040_e33083_d_n7;
        locals.var_vgs_eff_dn8 = assign24040_e33083_d_n8;
        locals.var_vgs_eff_dn9 = assign24040_e33083_d_n9;
        locals.var_vgs_eff_dn10 = assign24040_e33083_d_n10;
        locals.var_vgs_eff_dn11 = assign24040_e33083_d_n11;
        locals.var_vgs_eff_dn12 = assign24040_e33083_d_n12;
        locals.var_vgs_eff_dn13 = assign24040_e33083_d_n13;
        locals.var_vgs_eff_dn14 = assign24040_e33083_d_n14;
        locals.var_vgs_eff_rv = 0.0;

        let assign24050_e33086: f64 = if p.p1041 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard605 = assign24050_e33086;
        locals.var_guard605_rv = 0.0;

        let (assign24060_e33153, assign24060_e33153_d_n0, assign24060_e33153_d_n2, assign24060_e33153_d_n3, assign24060_e33153_d_n4, assign24060_e33153_d_n5, assign24060_e33153_d_n6, assign24060_e33153_d_n7, assign24060_e33153_d_n8, assign24060_e33153_d_n9, assign24060_e33153_d_n10, assign24060_e33153_d_n11, assign24060_e33153_d_n12, assign24060_e33153_d_n13, assign24060_e33153_d_n14,) = {
    if (((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) && (locals.var_guard605 != 0.0)) {
        let assign24060_e33095: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
        let assign24060_e33096: f64 = (locals.var_aigs_i - assign24060_e33095);
        let assign24060_e33098: f64 = (-10000.0);
        let assign24060_e33100: f64 = (assign24060_e33098 * 1e-6);
        let (assign24060_e33151, assign24060_e33151_d_n0, assign24060_e33151_d_n2, assign24060_e33151_d_n3, assign24060_e33151_d_n4, assign24060_e33151_d_n5, assign24060_e33151_d_n6, assign24060_e33151_d_n7, assign24060_e33151_d_n8, assign24060_e33151_d_n9, assign24060_e33151_d_n10, assign24060_e33151_d_n11, assign24060_e33151_d_n12, assign24060_e33151_d_n13, assign24060_e33151_d_n14,) = {
            if (!(assign24060_e33096 < assign24060_e33100)) {
                let assign24060_e33107: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
                let assign24060_e33108: f64 = (locals.var_aigs_i - assign24060_e33107);
                let assign24060_e33112: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
                let assign24060_e33113: f64 = (locals.var_aigs_i - assign24060_e33112);
                let assign24060_e33117: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
                let assign24060_e33118: f64 = (locals.var_aigs_i - assign24060_e33117);
                let assign24060_e33119: f64 = (assign24060_e33113 * assign24060_e33118);
                let assign24060_e33122: f64 = (4.0 * 1e-6);
                let assign24060_e33124: f64 = (assign24060_e33122 * 1e-6);
                let assign24060_e33125: f64 = (assign24060_e33119 + assign24060_e33124);
                let assign24060_e33126: f64 = (assign24060_e33125).sqrt();
                let assign24060_e33127: f64 = (assign24060_e33108 + assign24060_e33126);
                let assign24060_e33128: f64 = (0.5 * assign24060_e33127);
                (assign24060_e33128, (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn0)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn0)) * assign24060_e33118) + (assign24060_e33113 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn0)))) / (2.0 * assign24060_e33126)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn2)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn2)) * assign24060_e33118) + (assign24060_e33113 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn2)))) / (2.0 * assign24060_e33126)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn3)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn3)) * assign24060_e33118) + (assign24060_e33113 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn3)))) / (2.0 * assign24060_e33126)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn4)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn4)) * assign24060_e33118) + (assign24060_e33113 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn4)))) / (2.0 * assign24060_e33126)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn5)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn5)) * assign24060_e33118) + (assign24060_e33113 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn5)))) / (2.0 * assign24060_e33126)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn6)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn6)) * assign24060_e33118) + (assign24060_e33113 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn6)))) / (2.0 * assign24060_e33126)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn7)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn7)) * assign24060_e33118) + (assign24060_e33113 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn7)))) / (2.0 * assign24060_e33126)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn8)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn8)) * assign24060_e33118) + (assign24060_e33113 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn8)))) / (2.0 * assign24060_e33126)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn9)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn9)) * assign24060_e33118) + (assign24060_e33113 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn9)))) / (2.0 * assign24060_e33126)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn10)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn10)) * assign24060_e33118) + (assign24060_e33113 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn10)))) / (2.0 * assign24060_e33126)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn11)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn11)) * assign24060_e33118) + (assign24060_e33113 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn11)))) / (2.0 * assign24060_e33126)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn12)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn12)) * assign24060_e33118) + (assign24060_e33113 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn12)))) / (2.0 * assign24060_e33126)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn13)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn13)) * assign24060_e33118) + (assign24060_e33113 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn13)))) / (2.0 * assign24060_e33126)))), (0.5 * ((-(locals.var_bigs_i * locals.var_vgs_eff_dn14)) + ((((-(locals.var_bigs_i * locals.var_vgs_eff_dn14)) * assign24060_e33118) + (assign24060_e33113 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn14)))) / (2.0 * assign24060_e33126)))),)
            } else {
                let assign24060_e33132: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
                let assign24060_e33133: f64 = (locals.var_aigs_i - assign24060_e33132);
                let assign24060_e33135: f64 = (-10000.0);
                let assign24060_e33137: f64 = (assign24060_e33135 * 1e-6);
                let (assign24060_e33150, assign24060_e33150_d_n0, assign24060_e33150_d_n2, assign24060_e33150_d_n3, assign24060_e33150_d_n4, assign24060_e33150_d_n5, assign24060_e33150_d_n6, assign24060_e33150_d_n7, assign24060_e33150_d_n8, assign24060_e33150_d_n9, assign24060_e33150_d_n10, assign24060_e33150_d_n11, assign24060_e33150_d_n12, assign24060_e33150_d_n13, assign24060_e33150_d_n14,) = {
                    if (assign24060_e33133 < assign24060_e33137) {
                        let assign24060_e33140: f64 = (-1e-6);
                        let assign24060_e33142: f64 = (assign24060_e33140 * 1e-6);
                        let assign24060_e33146: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
                        let assign24060_e33147: f64 = (locals.var_aigs_i - assign24060_e33146);
                        let assign24060_e33148: f64 = (assign24060_e33142 / assign24060_e33147);
                        (assign24060_e33148, (-((assign24060_e33142 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn0))) / (assign24060_e33147 * assign24060_e33147))), (-((assign24060_e33142 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn2))) / (assign24060_e33147 * assign24060_e33147))), (-((assign24060_e33142 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn3))) / (assign24060_e33147 * assign24060_e33147))), (-((assign24060_e33142 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn4))) / (assign24060_e33147 * assign24060_e33147))), (-((assign24060_e33142 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn5))) / (assign24060_e33147 * assign24060_e33147))), (-((assign24060_e33142 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn6))) / (assign24060_e33147 * assign24060_e33147))), (-((assign24060_e33142 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn7))) / (assign24060_e33147 * assign24060_e33147))), (-((assign24060_e33142 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn8))) / (assign24060_e33147 * assign24060_e33147))), (-((assign24060_e33142 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn9))) / (assign24060_e33147 * assign24060_e33147))), (-((assign24060_e33142 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn10))) / (assign24060_e33147 * assign24060_e33147))), (-((assign24060_e33142 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn11))) / (assign24060_e33147 * assign24060_e33147))), (-((assign24060_e33142 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn12))) / (assign24060_e33147 * assign24060_e33147))), (-((assign24060_e33142 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn13))) / (assign24060_e33147 * assign24060_e33147))), (-((assign24060_e33142 * (-(locals.var_bigs_i * locals.var_vgs_eff_dn14))) / (assign24060_e33147 * assign24060_e33147))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign24060_e33150, assign24060_e33150_d_n0, assign24060_e33150_d_n2, assign24060_e33150_d_n3, assign24060_e33150_d_n4, assign24060_e33150_d_n5, assign24060_e33150_d_n6, assign24060_e33150_d_n7, assign24060_e33150_d_n8, assign24060_e33150_d_n9, assign24060_e33150_d_n10, assign24060_e33150_d_n11, assign24060_e33150_d_n12, assign24060_e33150_d_n13, assign24060_e33150_d_n14,)
            }
        };
        (assign24060_e33151, assign24060_e33151_d_n0, assign24060_e33151_d_n2, assign24060_e33151_d_n3, assign24060_e33151_d_n4, assign24060_e33151_d_n5, assign24060_e33151_d_n6, assign24060_e33151_d_n7, assign24060_e33151_d_n8, assign24060_e33151_d_n9, assign24060_e33151_d_n10, assign24060_e33151_d_n11, assign24060_e33151_d_n12, assign24060_e33151_d_n13, assign24060_e33151_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign24060_e33153;
        locals.var_t1_dn0 = assign24060_e33153_d_n0;
        locals.var_t1_dn2 = assign24060_e33153_d_n2;
        locals.var_t1_dn3 = assign24060_e33153_d_n3;
        locals.var_t1_dn4 = assign24060_e33153_d_n4;
        locals.var_t1_dn5 = assign24060_e33153_d_n5;
        locals.var_t1_dn6 = assign24060_e33153_d_n6;
        locals.var_t1_dn7 = assign24060_e33153_d_n7;
        locals.var_t1_dn8 = assign24060_e33153_d_n8;
        locals.var_t1_dn9 = assign24060_e33153_d_n9;
        locals.var_t1_dn10 = assign24060_e33153_d_n10;
        locals.var_t1_dn11 = assign24060_e33153_d_n11;
        locals.var_t1_dn12 = assign24060_e33153_d_n12;
        locals.var_t1_dn13 = assign24060_e33153_d_n13;
        locals.var_t1_dn14 = assign24060_e33153_d_n14;
        locals.var_t1_rv = 0.0;

        let assign24070_e33156: f64 = if locals.var_cigs_i < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard606 = assign24070_e33156;
        locals.var_guard606_rv = 0.0;

        let (assign24080_e33166,) = {
    if ((((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) && (locals.var_guard605 != 0.0)) && (locals.var_guard606 != 0.0)) {
        (0.01,)
    } else {
        (locals.var_cigs_i,)
    }
};
        locals.var_cigs_i = assign24080_e33166;
        locals.var_cigs_i_rv = 0.0;

        let (assign24090_e33179, assign24090_e33179_d_n0, assign24090_e33179_d_n2, assign24090_e33179_d_n3, assign24090_e33179_d_n4, assign24090_e33179_d_n5, assign24090_e33179_d_n6, assign24090_e33179_d_n7, assign24090_e33179_d_n8, assign24090_e33179_d_n9, assign24090_e33179_d_n10, assign24090_e33179_d_n11, assign24090_e33179_d_n12, assign24090_e33179_d_n13, assign24090_e33179_d_n14,) = {
    if (((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) && (locals.var_guard605 == 0.0)) {
        let assign24090_e33176: f64 = (locals.var_bigs_i * locals.var_vgs_eff);
        let assign24090_e33177: f64 = (locals.var_aigs_i - assign24090_e33176);
        (assign24090_e33177, (-(locals.var_bigs_i * locals.var_vgs_eff_dn0)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn2)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn3)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn4)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn5)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn6)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn7)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn8)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn9)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn10)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn11)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn12)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn13)), (-(locals.var_bigs_i * locals.var_vgs_eff_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign24090_e33179;
        locals.var_t1_dn0 = assign24090_e33179_d_n0;
        locals.var_t1_dn2 = assign24090_e33179_d_n2;
        locals.var_t1_dn3 = assign24090_e33179_d_n3;
        locals.var_t1_dn4 = assign24090_e33179_d_n4;
        locals.var_t1_dn5 = assign24090_e33179_d_n5;
        locals.var_t1_dn6 = assign24090_e33179_d_n6;
        locals.var_t1_dn7 = assign24090_e33179_d_n7;
        locals.var_t1_dn8 = assign24090_e33179_d_n8;
        locals.var_t1_dn9 = assign24090_e33179_d_n9;
        locals.var_t1_dn10 = assign24090_e33179_d_n10;
        locals.var_t1_dn11 = assign24090_e33179_d_n11;
        locals.var_t1_dn12 = assign24090_e33179_d_n12;
        locals.var_t1_dn13 = assign24090_e33179_d_n13;
        locals.var_t1_dn14 = assign24090_e33179_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign24100_e33189, assign24100_e33189_d_n0, assign24100_e33189_d_n2, assign24100_e33189_d_n3, assign24100_e33189_d_n4, assign24100_e33189_d_n5, assign24100_e33189_d_n6, assign24100_e33189_d_n7, assign24100_e33189_d_n8, assign24100_e33189_d_n9, assign24100_e33189_d_n10, assign24100_e33189_d_n11, assign24100_e33189_d_n12, assign24100_e33189_d_n13, assign24100_e33189_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) {
        let assign24100_e33186: f64 = (locals.var_cigs_i * locals.var_vgs_eff);
        let assign24100_e33187: f64 = (1.0 + assign24100_e33186);
        (assign24100_e33187, (locals.var_cigs_i * locals.var_vgs_eff_dn0), (locals.var_cigs_i * locals.var_vgs_eff_dn2), (locals.var_cigs_i * locals.var_vgs_eff_dn3), (locals.var_cigs_i * locals.var_vgs_eff_dn4), (locals.var_cigs_i * locals.var_vgs_eff_dn5), (locals.var_cigs_i * locals.var_vgs_eff_dn6), (locals.var_cigs_i * locals.var_vgs_eff_dn7), (locals.var_cigs_i * locals.var_vgs_eff_dn8), (locals.var_cigs_i * locals.var_vgs_eff_dn9), (locals.var_cigs_i * locals.var_vgs_eff_dn10), (locals.var_cigs_i * locals.var_vgs_eff_dn11), (locals.var_cigs_i * locals.var_vgs_eff_dn12), (locals.var_cigs_i * locals.var_vgs_eff_dn13), (locals.var_cigs_i * locals.var_vgs_eff_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign24100_e33189;
        locals.var_t2_dn0 = assign24100_e33189_d_n0;
        locals.var_t2_dn2 = assign24100_e33189_d_n2;
        locals.var_t2_dn3 = assign24100_e33189_d_n3;
        locals.var_t2_dn4 = assign24100_e33189_d_n4;
        locals.var_t2_dn5 = assign24100_e33189_d_n5;
        locals.var_t2_dn6 = assign24100_e33189_d_n6;
        locals.var_t2_dn7 = assign24100_e33189_d_n7;
        locals.var_t2_dn8 = assign24100_e33189_d_n8;
        locals.var_t2_dn9 = assign24100_e33189_d_n9;
        locals.var_t2_dn10 = assign24100_e33189_d_n10;
        locals.var_t2_dn11 = assign24100_e33189_d_n11;
        locals.var_t2_dn12 = assign24100_e33189_d_n12;
        locals.var_t2_dn13 = assign24100_e33189_d_n13;
        locals.var_t2_dn14 = assign24100_e33189_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_71(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24110_e33199, assign24110_e33199_d_n0, assign24110_e33199_d_n2, assign24110_e33199_d_n3, assign24110_e33199_d_n4, assign24110_e33199_d_n5, assign24110_e33199_d_n6, assign24110_e33199_d_n7, assign24110_e33199_d_n8, assign24110_e33199_d_n9, assign24110_e33199_d_n10, assign24110_e33199_d_n11, assign24110_e33199_d_n12, assign24110_e33199_d_n13, assign24110_e33199_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) {
        let assign24110_e33195: f64 = (locals.var_bechvbedge * locals.var_t1);
        let assign24110_e33197: f64 = (assign24110_e33195 * locals.var_t2);
        (assign24110_e33197, (((locals.var_bechvbedge * locals.var_t1_dn0) * locals.var_t2) + (assign24110_e33195 * locals.var_t2_dn0)), (((locals.var_bechvbedge * locals.var_t1_dn2) * locals.var_t2) + (assign24110_e33195 * locals.var_t2_dn2)), (((locals.var_bechvbedge * locals.var_t1_dn3) * locals.var_t2) + (assign24110_e33195 * locals.var_t2_dn3)), (((locals.var_bechvbedge * locals.var_t1_dn4) * locals.var_t2) + (assign24110_e33195 * locals.var_t2_dn4)), (((locals.var_bechvbedge * locals.var_t1_dn5) * locals.var_t2) + (assign24110_e33195 * locals.var_t2_dn5)), (((locals.var_bechvbedge * locals.var_t1_dn6) * locals.var_t2) + (assign24110_e33195 * locals.var_t2_dn6)), (((locals.var_bechvbedge * locals.var_t1_dn7) * locals.var_t2) + (assign24110_e33195 * locals.var_t2_dn7)), (((locals.var_bechvbedge * locals.var_t1_dn8) * locals.var_t2) + (assign24110_e33195 * locals.var_t2_dn8)), (((locals.var_bechvbedge * locals.var_t1_dn9) * locals.var_t2) + (assign24110_e33195 * locals.var_t2_dn9)), (((locals.var_bechvbedge * locals.var_t1_dn10) * locals.var_t2) + (assign24110_e33195 * locals.var_t2_dn10)), (((locals.var_bechvbedge * locals.var_t1_dn11) * locals.var_t2) + (assign24110_e33195 * locals.var_t2_dn11)), (((locals.var_bechvbedge * locals.var_t1_dn12) * locals.var_t2) + (assign24110_e33195 * locals.var_t2_dn12)), (((locals.var_bechvbedge * locals.var_t1_dn13) * locals.var_t2) + (assign24110_e33195 * locals.var_t2_dn13)), (((locals.var_bechvbedge * locals.var_t1_dn14) * locals.var_t2) + (assign24110_e33195 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign24110_e33199;
        locals.var_t3_dn0 = assign24110_e33199_d_n0;
        locals.var_t3_dn2 = assign24110_e33199_d_n2;
        locals.var_t3_dn3 = assign24110_e33199_d_n3;
        locals.var_t3_dn4 = assign24110_e33199_d_n4;
        locals.var_t3_dn5 = assign24110_e33199_d_n5;
        locals.var_t3_dn6 = assign24110_e33199_d_n6;
        locals.var_t3_dn7 = assign24110_e33199_d_n7;
        locals.var_t3_dn8 = assign24110_e33199_d_n8;
        locals.var_t3_dn9 = assign24110_e33199_d_n9;
        locals.var_t3_dn10 = assign24110_e33199_d_n10;
        locals.var_t3_dn11 = assign24110_e33199_d_n11;
        locals.var_t3_dn12 = assign24110_e33199_d_n12;
        locals.var_t3_dn13 = assign24110_e33199_d_n13;
        locals.var_t3_dn14 = assign24110_e33199_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign24120_e33206, assign24120_e33206_d_n0, assign24120_e33206_d_n2, assign24120_e33206_d_n3, assign24120_e33206_d_n4, assign24120_e33206_d_n5, assign24120_e33206_d_n6, assign24120_e33206_d_n7, assign24120_e33206_d_n8, assign24120_e33206_d_n9, assign24120_e33206_d_n10, assign24120_e33206_d_n11, assign24120_e33206_d_n12, assign24120_e33206_d_n13, assign24120_e33206_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) {
        let assign24120_e33204: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign24120_e33204, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn0), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn2), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn12), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn13), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign24120_e33206;
        locals.var_t4_dn0 = assign24120_e33206_d_n0;
        locals.var_t4_dn2 = assign24120_e33206_d_n2;
        locals.var_t4_dn3 = assign24120_e33206_d_n3;
        locals.var_t4_dn4 = assign24120_e33206_d_n4;
        locals.var_t4_dn5 = assign24120_e33206_d_n5;
        locals.var_t4_dn6 = assign24120_e33206_d_n6;
        locals.var_t4_dn7 = assign24120_e33206_d_n7;
        locals.var_t4_dn8 = assign24120_e33206_d_n8;
        locals.var_t4_dn9 = assign24120_e33206_d_n9;
        locals.var_t4_dn10 = assign24120_e33206_d_n10;
        locals.var_t4_dn11 = assign24120_e33206_d_n11;
        locals.var_t4_dn12 = assign24120_e33206_d_n12;
        locals.var_t4_dn13 = assign24120_e33206_d_n13;
        locals.var_t4_dn14 = assign24120_e33206_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign24150_e33238, assign24150_e33238_d_n0, assign24150_e33238_d_n2, assign24150_e33238_d_n3, assign24150_e33238_d_n4, assign24150_e33238_d_n5, assign24150_e33238_d_n6, assign24150_e33238_d_n7, assign24150_e33238_d_n8, assign24150_e33238_d_n9, assign24150_e33238_d_n10, assign24150_e33238_d_n11, assign24150_e33238_d_n12, assign24150_e33238_d_n13, assign24150_e33238_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) {
        let assign24150_e33236: f64 = (locals.var_vgd_noswap - locals.var_vfbsdr);
        (assign24150_e33236, 0.0, 0.0, 0.0, (-locals.var_vfbsdr_dn4), locals.var_vgd_noswap_dn5, 0.0, locals.var_vgd_noswap_dn7, 0.0, locals.var_vgd_noswap_dn9, 0.0, locals.var_vgd_noswap_dn11, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign24150_e33238;
        locals.var_t2_dn0 = assign24150_e33238_d_n0;
        locals.var_t2_dn2 = assign24150_e33238_d_n2;
        locals.var_t2_dn3 = assign24150_e33238_d_n3;
        locals.var_t2_dn4 = assign24150_e33238_d_n4;
        locals.var_t2_dn5 = assign24150_e33238_d_n5;
        locals.var_t2_dn6 = assign24150_e33238_d_n6;
        locals.var_t2_dn7 = assign24150_e33238_d_n7;
        locals.var_t2_dn8 = assign24150_e33238_d_n8;
        locals.var_t2_dn9 = assign24150_e33238_d_n9;
        locals.var_t2_dn10 = assign24150_e33238_d_n10;
        locals.var_t2_dn11 = assign24150_e33238_d_n11;
        locals.var_t2_dn12 = assign24150_e33238_d_n12;
        locals.var_t2_dn13 = assign24150_e33238_d_n13;
        locals.var_t2_dn14 = assign24150_e33238_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign24160_e33249, assign24160_e33249_d_n0, assign24160_e33249_d_n2, assign24160_e33249_d_n3, assign24160_e33249_d_n4, assign24160_e33249_d_n5, assign24160_e33249_d_n6, assign24160_e33249_d_n7, assign24160_e33249_d_n8, assign24160_e33249_d_n9, assign24160_e33249_d_n10, assign24160_e33249_d_n11, assign24160_e33249_d_n12, assign24160_e33249_d_n13, assign24160_e33249_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) {
        let assign24160_e33244: f64 = (locals.var_t2 * locals.var_t2);
        let assign24160_e33246: f64 = (assign24160_e33244 + 0.0001);
        let assign24160_e33247: f64 = (assign24160_e33246).sqrt();
        (assign24160_e33247, (((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0)) / (2.0 * assign24160_e33247)), (((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2)) / (2.0 * assign24160_e33247)), (((locals.var_t2_dn3 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn3)) / (2.0 * assign24160_e33247)), (((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4)) / (2.0 * assign24160_e33247)), (((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5)) / (2.0 * assign24160_e33247)), (((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6)) / (2.0 * assign24160_e33247)), (((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7)) / (2.0 * assign24160_e33247)), (((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8)) / (2.0 * assign24160_e33247)), (((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9)) / (2.0 * assign24160_e33247)), (((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10)) / (2.0 * assign24160_e33247)), (((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11)) / (2.0 * assign24160_e33247)), (((locals.var_t2_dn12 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn12)) / (2.0 * assign24160_e33247)), (((locals.var_t2_dn13 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn13)) / (2.0 * assign24160_e33247)), (((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14)) / (2.0 * assign24160_e33247)),)
    } else {
        (locals.var_vgd_eff, locals.var_vgd_eff_dn0, locals.var_vgd_eff_dn2, locals.var_vgd_eff_dn3, locals.var_vgd_eff_dn4, locals.var_vgd_eff_dn5, locals.var_vgd_eff_dn6, locals.var_vgd_eff_dn7, locals.var_vgd_eff_dn8, locals.var_vgd_eff_dn9, locals.var_vgd_eff_dn10, locals.var_vgd_eff_dn11, locals.var_vgd_eff_dn12, locals.var_vgd_eff_dn13, locals.var_vgd_eff_dn14,)
    }
};
        locals.var_vgd_eff = assign24160_e33249;
        locals.var_vgd_eff_dn0 = assign24160_e33249_d_n0;
        locals.var_vgd_eff_dn2 = assign24160_e33249_d_n2;
        locals.var_vgd_eff_dn3 = assign24160_e33249_d_n3;
        locals.var_vgd_eff_dn4 = assign24160_e33249_d_n4;
        locals.var_vgd_eff_dn5 = assign24160_e33249_d_n5;
        locals.var_vgd_eff_dn6 = assign24160_e33249_d_n6;
        locals.var_vgd_eff_dn7 = assign24160_e33249_d_n7;
        locals.var_vgd_eff_dn8 = assign24160_e33249_d_n8;
        locals.var_vgd_eff_dn9 = assign24160_e33249_d_n9;
        locals.var_vgd_eff_dn10 = assign24160_e33249_d_n10;
        locals.var_vgd_eff_dn11 = assign24160_e33249_d_n11;
        locals.var_vgd_eff_dn12 = assign24160_e33249_d_n12;
        locals.var_vgd_eff_dn13 = assign24160_e33249_d_n13;
        locals.var_vgd_eff_dn14 = assign24160_e33249_d_n14;
        locals.var_vgd_eff_rv = 0.0;

        let assign24170_e33252: f64 = if p.p1041 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard607 = assign24170_e33252;
        locals.var_guard607_rv = 0.0;

        let (assign24180_e33319, assign24180_e33319_d_n0, assign24180_e33319_d_n2, assign24180_e33319_d_n3, assign24180_e33319_d_n4, assign24180_e33319_d_n5, assign24180_e33319_d_n6, assign24180_e33319_d_n7, assign24180_e33319_d_n8, assign24180_e33319_d_n9, assign24180_e33319_d_n10, assign24180_e33319_d_n11, assign24180_e33319_d_n12, assign24180_e33319_d_n13, assign24180_e33319_d_n14,) = {
    if (((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) && (locals.var_guard607 != 0.0)) {
        let assign24180_e33261: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
        let assign24180_e33262: f64 = (locals.var_aigd_i - assign24180_e33261);
        let assign24180_e33264: f64 = (-10000.0);
        let assign24180_e33266: f64 = (assign24180_e33264 * 1e-6);
        let (assign24180_e33317, assign24180_e33317_d_n0, assign24180_e33317_d_n2, assign24180_e33317_d_n3, assign24180_e33317_d_n4, assign24180_e33317_d_n5, assign24180_e33317_d_n6, assign24180_e33317_d_n7, assign24180_e33317_d_n8, assign24180_e33317_d_n9, assign24180_e33317_d_n10, assign24180_e33317_d_n11, assign24180_e33317_d_n12, assign24180_e33317_d_n13, assign24180_e33317_d_n14,) = {
            if (!(assign24180_e33262 < assign24180_e33266)) {
                let assign24180_e33273: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
                let assign24180_e33274: f64 = (locals.var_aigd_i - assign24180_e33273);
                let assign24180_e33278: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
                let assign24180_e33279: f64 = (locals.var_aigd_i - assign24180_e33278);
                let assign24180_e33283: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
                let assign24180_e33284: f64 = (locals.var_aigd_i - assign24180_e33283);
                let assign24180_e33285: f64 = (assign24180_e33279 * assign24180_e33284);
                let assign24180_e33288: f64 = (4.0 * 1e-6);
                let assign24180_e33290: f64 = (assign24180_e33288 * 1e-6);
                let assign24180_e33291: f64 = (assign24180_e33285 + assign24180_e33290);
                let assign24180_e33292: f64 = (assign24180_e33291).sqrt();
                let assign24180_e33293: f64 = (assign24180_e33274 + assign24180_e33292);
                let assign24180_e33294: f64 = (0.5 * assign24180_e33293);
                (assign24180_e33294, (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn0)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn0)) * assign24180_e33284) + (assign24180_e33279 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn0)))) / (2.0 * assign24180_e33292)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn2)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn2)) * assign24180_e33284) + (assign24180_e33279 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn2)))) / (2.0 * assign24180_e33292)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn3)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn3)) * assign24180_e33284) + (assign24180_e33279 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn3)))) / (2.0 * assign24180_e33292)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn4)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn4)) * assign24180_e33284) + (assign24180_e33279 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn4)))) / (2.0 * assign24180_e33292)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn5)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn5)) * assign24180_e33284) + (assign24180_e33279 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn5)))) / (2.0 * assign24180_e33292)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn6)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn6)) * assign24180_e33284) + (assign24180_e33279 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn6)))) / (2.0 * assign24180_e33292)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn7)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn7)) * assign24180_e33284) + (assign24180_e33279 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn7)))) / (2.0 * assign24180_e33292)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn8)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn8)) * assign24180_e33284) + (assign24180_e33279 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn8)))) / (2.0 * assign24180_e33292)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn9)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn9)) * assign24180_e33284) + (assign24180_e33279 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn9)))) / (2.0 * assign24180_e33292)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn10)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn10)) * assign24180_e33284) + (assign24180_e33279 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn10)))) / (2.0 * assign24180_e33292)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn11)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn11)) * assign24180_e33284) + (assign24180_e33279 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn11)))) / (2.0 * assign24180_e33292)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn12)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn12)) * assign24180_e33284) + (assign24180_e33279 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn12)))) / (2.0 * assign24180_e33292)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn13)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn13)) * assign24180_e33284) + (assign24180_e33279 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn13)))) / (2.0 * assign24180_e33292)))), (0.5 * ((-(locals.var_bigd_i * locals.var_vgd_eff_dn14)) + ((((-(locals.var_bigd_i * locals.var_vgd_eff_dn14)) * assign24180_e33284) + (assign24180_e33279 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn14)))) / (2.0 * assign24180_e33292)))),)
            } else {
                let assign24180_e33298: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
                let assign24180_e33299: f64 = (locals.var_aigd_i - assign24180_e33298);
                let assign24180_e33301: f64 = (-10000.0);
                let assign24180_e33303: f64 = (assign24180_e33301 * 1e-6);
                let (assign24180_e33316, assign24180_e33316_d_n0, assign24180_e33316_d_n2, assign24180_e33316_d_n3, assign24180_e33316_d_n4, assign24180_e33316_d_n5, assign24180_e33316_d_n6, assign24180_e33316_d_n7, assign24180_e33316_d_n8, assign24180_e33316_d_n9, assign24180_e33316_d_n10, assign24180_e33316_d_n11, assign24180_e33316_d_n12, assign24180_e33316_d_n13, assign24180_e33316_d_n14,) = {
                    if (assign24180_e33299 < assign24180_e33303) {
                        let assign24180_e33306: f64 = (-1e-6);
                        let assign24180_e33308: f64 = (assign24180_e33306 * 1e-6);
                        let assign24180_e33312: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
                        let assign24180_e33313: f64 = (locals.var_aigd_i - assign24180_e33312);
                        let assign24180_e33314: f64 = (assign24180_e33308 / assign24180_e33313);
                        (assign24180_e33314, (-((assign24180_e33308 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn0))) / (assign24180_e33313 * assign24180_e33313))), (-((assign24180_e33308 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn2))) / (assign24180_e33313 * assign24180_e33313))), (-((assign24180_e33308 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn3))) / (assign24180_e33313 * assign24180_e33313))), (-((assign24180_e33308 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn4))) / (assign24180_e33313 * assign24180_e33313))), (-((assign24180_e33308 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn5))) / (assign24180_e33313 * assign24180_e33313))), (-((assign24180_e33308 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn6))) / (assign24180_e33313 * assign24180_e33313))), (-((assign24180_e33308 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn7))) / (assign24180_e33313 * assign24180_e33313))), (-((assign24180_e33308 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn8))) / (assign24180_e33313 * assign24180_e33313))), (-((assign24180_e33308 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn9))) / (assign24180_e33313 * assign24180_e33313))), (-((assign24180_e33308 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn10))) / (assign24180_e33313 * assign24180_e33313))), (-((assign24180_e33308 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn11))) / (assign24180_e33313 * assign24180_e33313))), (-((assign24180_e33308 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn12))) / (assign24180_e33313 * assign24180_e33313))), (-((assign24180_e33308 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn13))) / (assign24180_e33313 * assign24180_e33313))), (-((assign24180_e33308 * (-(locals.var_bigd_i * locals.var_vgd_eff_dn14))) / (assign24180_e33313 * assign24180_e33313))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign24180_e33316, assign24180_e33316_d_n0, assign24180_e33316_d_n2, assign24180_e33316_d_n3, assign24180_e33316_d_n4, assign24180_e33316_d_n5, assign24180_e33316_d_n6, assign24180_e33316_d_n7, assign24180_e33316_d_n8, assign24180_e33316_d_n9, assign24180_e33316_d_n10, assign24180_e33316_d_n11, assign24180_e33316_d_n12, assign24180_e33316_d_n13, assign24180_e33316_d_n14,)
            }
        };
        (assign24180_e33317, assign24180_e33317_d_n0, assign24180_e33317_d_n2, assign24180_e33317_d_n3, assign24180_e33317_d_n4, assign24180_e33317_d_n5, assign24180_e33317_d_n6, assign24180_e33317_d_n7, assign24180_e33317_d_n8, assign24180_e33317_d_n9, assign24180_e33317_d_n10, assign24180_e33317_d_n11, assign24180_e33317_d_n12, assign24180_e33317_d_n13, assign24180_e33317_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign24180_e33319;
        locals.var_t1_dn0 = assign24180_e33319_d_n0;
        locals.var_t1_dn2 = assign24180_e33319_d_n2;
        locals.var_t1_dn3 = assign24180_e33319_d_n3;
        locals.var_t1_dn4 = assign24180_e33319_d_n4;
        locals.var_t1_dn5 = assign24180_e33319_d_n5;
        locals.var_t1_dn6 = assign24180_e33319_d_n6;
        locals.var_t1_dn7 = assign24180_e33319_d_n7;
        locals.var_t1_dn8 = assign24180_e33319_d_n8;
        locals.var_t1_dn9 = assign24180_e33319_d_n9;
        locals.var_t1_dn10 = assign24180_e33319_d_n10;
        locals.var_t1_dn11 = assign24180_e33319_d_n11;
        locals.var_t1_dn12 = assign24180_e33319_d_n12;
        locals.var_t1_dn13 = assign24180_e33319_d_n13;
        locals.var_t1_dn14 = assign24180_e33319_d_n14;
        locals.var_t1_rv = 0.0;

        let assign24190_e33322: f64 = if locals.var_cigd_i < 0.01 { 1.0 } else { 0.0 };
        locals.var_guard608 = assign24190_e33322;
        locals.var_guard608_rv = 0.0;

        let (assign24200_e33332,) = {
    if ((((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) && (locals.var_guard607 != 0.0)) && (locals.var_guard608 != 0.0)) {
        (0.01,)
    } else {
        (locals.var_cigd_i,)
    }
};
        locals.var_cigd_i = assign24200_e33332;
        locals.var_cigd_i_rv = 0.0;

        let (assign24210_e33345, assign24210_e33345_d_n0, assign24210_e33345_d_n2, assign24210_e33345_d_n3, assign24210_e33345_d_n4, assign24210_e33345_d_n5, assign24210_e33345_d_n6, assign24210_e33345_d_n7, assign24210_e33345_d_n8, assign24210_e33345_d_n9, assign24210_e33345_d_n10, assign24210_e33345_d_n11, assign24210_e33345_d_n12, assign24210_e33345_d_n13, assign24210_e33345_d_n14,) = {
    if (((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) && (locals.var_guard607 == 0.0)) {
        let assign24210_e33342: f64 = (locals.var_bigd_i * locals.var_vgd_eff);
        let assign24210_e33343: f64 = (locals.var_aigd_i - assign24210_e33342);
        (assign24210_e33343, (-(locals.var_bigd_i * locals.var_vgd_eff_dn0)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn2)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn3)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn4)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn5)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn6)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn7)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn8)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn9)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn10)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn11)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn12)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn13)), (-(locals.var_bigd_i * locals.var_vgd_eff_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign24210_e33345;
        locals.var_t1_dn0 = assign24210_e33345_d_n0;
        locals.var_t1_dn2 = assign24210_e33345_d_n2;
        locals.var_t1_dn3 = assign24210_e33345_d_n3;
        locals.var_t1_dn4 = assign24210_e33345_d_n4;
        locals.var_t1_dn5 = assign24210_e33345_d_n5;
        locals.var_t1_dn6 = assign24210_e33345_d_n6;
        locals.var_t1_dn7 = assign24210_e33345_d_n7;
        locals.var_t1_dn8 = assign24210_e33345_d_n8;
        locals.var_t1_dn9 = assign24210_e33345_d_n9;
        locals.var_t1_dn10 = assign24210_e33345_d_n10;
        locals.var_t1_dn11 = assign24210_e33345_d_n11;
        locals.var_t1_dn12 = assign24210_e33345_d_n12;
        locals.var_t1_dn13 = assign24210_e33345_d_n13;
        locals.var_t1_dn14 = assign24210_e33345_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign24220_e33355, assign24220_e33355_d_n0, assign24220_e33355_d_n2, assign24220_e33355_d_n3, assign24220_e33355_d_n4, assign24220_e33355_d_n5, assign24220_e33355_d_n6, assign24220_e33355_d_n7, assign24220_e33355_d_n8, assign24220_e33355_d_n9, assign24220_e33355_d_n10, assign24220_e33355_d_n11, assign24220_e33355_d_n12, assign24220_e33355_d_n13, assign24220_e33355_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) {
        let assign24220_e33352: f64 = (locals.var_cigd_i * locals.var_vgd_eff);
        let assign24220_e33353: f64 = (1.0 + assign24220_e33352);
        (assign24220_e33353, (locals.var_cigd_i * locals.var_vgd_eff_dn0), (locals.var_cigd_i * locals.var_vgd_eff_dn2), (locals.var_cigd_i * locals.var_vgd_eff_dn3), (locals.var_cigd_i * locals.var_vgd_eff_dn4), (locals.var_cigd_i * locals.var_vgd_eff_dn5), (locals.var_cigd_i * locals.var_vgd_eff_dn6), (locals.var_cigd_i * locals.var_vgd_eff_dn7), (locals.var_cigd_i * locals.var_vgd_eff_dn8), (locals.var_cigd_i * locals.var_vgd_eff_dn9), (locals.var_cigd_i * locals.var_vgd_eff_dn10), (locals.var_cigd_i * locals.var_vgd_eff_dn11), (locals.var_cigd_i * locals.var_vgd_eff_dn12), (locals.var_cigd_i * locals.var_vgd_eff_dn13), (locals.var_cigd_i * locals.var_vgd_eff_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign24220_e33355;
        locals.var_t2_dn0 = assign24220_e33355_d_n0;
        locals.var_t2_dn2 = assign24220_e33355_d_n2;
        locals.var_t2_dn3 = assign24220_e33355_d_n3;
        locals.var_t2_dn4 = assign24220_e33355_d_n4;
        locals.var_t2_dn5 = assign24220_e33355_d_n5;
        locals.var_t2_dn6 = assign24220_e33355_d_n6;
        locals.var_t2_dn7 = assign24220_e33355_d_n7;
        locals.var_t2_dn8 = assign24220_e33355_d_n8;
        locals.var_t2_dn9 = assign24220_e33355_d_n9;
        locals.var_t2_dn10 = assign24220_e33355_d_n10;
        locals.var_t2_dn11 = assign24220_e33355_d_n11;
        locals.var_t2_dn12 = assign24220_e33355_d_n12;
        locals.var_t2_dn13 = assign24220_e33355_d_n13;
        locals.var_t2_dn14 = assign24220_e33355_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign24230_e33365, assign24230_e33365_d_n0, assign24230_e33365_d_n2, assign24230_e33365_d_n3, assign24230_e33365_d_n4, assign24230_e33365_d_n5, assign24230_e33365_d_n6, assign24230_e33365_d_n7, assign24230_e33365_d_n8, assign24230_e33365_d_n9, assign24230_e33365_d_n10, assign24230_e33365_d_n11, assign24230_e33365_d_n12, assign24230_e33365_d_n13, assign24230_e33365_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) {
        let assign24230_e33361: f64 = (locals.var_bechvbedge * locals.var_t1);
        let assign24230_e33363: f64 = (assign24230_e33361 * locals.var_t2);
        (assign24230_e33363, (((locals.var_bechvbedge * locals.var_t1_dn0) * locals.var_t2) + (assign24230_e33361 * locals.var_t2_dn0)), (((locals.var_bechvbedge * locals.var_t1_dn2) * locals.var_t2) + (assign24230_e33361 * locals.var_t2_dn2)), (((locals.var_bechvbedge * locals.var_t1_dn3) * locals.var_t2) + (assign24230_e33361 * locals.var_t2_dn3)), (((locals.var_bechvbedge * locals.var_t1_dn4) * locals.var_t2) + (assign24230_e33361 * locals.var_t2_dn4)), (((locals.var_bechvbedge * locals.var_t1_dn5) * locals.var_t2) + (assign24230_e33361 * locals.var_t2_dn5)), (((locals.var_bechvbedge * locals.var_t1_dn6) * locals.var_t2) + (assign24230_e33361 * locals.var_t2_dn6)), (((locals.var_bechvbedge * locals.var_t1_dn7) * locals.var_t2) + (assign24230_e33361 * locals.var_t2_dn7)), (((locals.var_bechvbedge * locals.var_t1_dn8) * locals.var_t2) + (assign24230_e33361 * locals.var_t2_dn8)), (((locals.var_bechvbedge * locals.var_t1_dn9) * locals.var_t2) + (assign24230_e33361 * locals.var_t2_dn9)), (((locals.var_bechvbedge * locals.var_t1_dn10) * locals.var_t2) + (assign24230_e33361 * locals.var_t2_dn10)), (((locals.var_bechvbedge * locals.var_t1_dn11) * locals.var_t2) + (assign24230_e33361 * locals.var_t2_dn11)), (((locals.var_bechvbedge * locals.var_t1_dn12) * locals.var_t2) + (assign24230_e33361 * locals.var_t2_dn12)), (((locals.var_bechvbedge * locals.var_t1_dn13) * locals.var_t2) + (assign24230_e33361 * locals.var_t2_dn13)), (((locals.var_bechvbedge * locals.var_t1_dn14) * locals.var_t2) + (assign24230_e33361 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign24230_e33365;
        locals.var_t3_dn0 = assign24230_e33365_d_n0;
        locals.var_t3_dn2 = assign24230_e33365_d_n2;
        locals.var_t3_dn3 = assign24230_e33365_d_n3;
        locals.var_t3_dn4 = assign24230_e33365_d_n4;
        locals.var_t3_dn5 = assign24230_e33365_d_n5;
        locals.var_t3_dn6 = assign24230_e33365_d_n6;
        locals.var_t3_dn7 = assign24230_e33365_d_n7;
        locals.var_t3_dn8 = assign24230_e33365_d_n8;
        locals.var_t3_dn9 = assign24230_e33365_d_n9;
        locals.var_t3_dn10 = assign24230_e33365_d_n10;
        locals.var_t3_dn11 = assign24230_e33365_d_n11;
        locals.var_t3_dn12 = assign24230_e33365_d_n12;
        locals.var_t3_dn13 = assign24230_e33365_d_n13;
        locals.var_t3_dn14 = assign24230_e33365_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign24240_e33372, assign24240_e33372_d_n0, assign24240_e33372_d_n2, assign24240_e33372_d_n3, assign24240_e33372_d_n4, assign24240_e33372_d_n5, assign24240_e33372_d_n6, assign24240_e33372_d_n7, assign24240_e33372_d_n8, assign24240_e33372_d_n9, assign24240_e33372_d_n10, assign24240_e33372_d_n11, assign24240_e33372_d_n12, assign24240_e33372_d_n13, assign24240_e33372_d_n14,) = {
    if ((locals.var_guard601 != 0.0) && (locals.var_guard603 != 0.0)) {
        let assign24240_e33370: f64 = { let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign24240_e33370, ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn0), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn2), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn3), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn4), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn5), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn6), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn7), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn8), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn9), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn10), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn11), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn12), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn13), ({ let limited_exp_arg = locals.var_t3; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t3_dn14),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign24240_e33372;
        locals.var_t4_dn0 = assign24240_e33372_d_n0;
        locals.var_t4_dn2 = assign24240_e33372_d_n2;
        locals.var_t4_dn3 = assign24240_e33372_d_n3;
        locals.var_t4_dn4 = assign24240_e33372_d_n4;
        locals.var_t4_dn5 = assign24240_e33372_d_n5;
        locals.var_t4_dn6 = assign24240_e33372_d_n6;
        locals.var_t4_dn7 = assign24240_e33372_d_n7;
        locals.var_t4_dn8 = assign24240_e33372_d_n8;
        locals.var_t4_dn9 = assign24240_e33372_d_n9;
        locals.var_t4_dn10 = assign24240_e33372_d_n10;
        locals.var_t4_dn11 = assign24240_e33372_d_n11;
        locals.var_t4_dn12 = assign24240_e33372_d_n12;
        locals.var_t4_dn13 = assign24240_e33372_d_n13;
        locals.var_t4_dn14 = assign24240_e33372_d_n14;
        locals.var_t4_rv = 0.0;

        let assign24340_e33426: f64 = if p.p45 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard609 = assign24340_e33426;
        locals.var_guard609_rv = 0.0;

        let (assign24350_e33432, assign24350_e33432_d_n0, assign24350_e33432_d_n2, assign24350_e33432_d_n3, assign24350_e33432_d_n4, assign24350_e33432_d_n5, assign24350_e33432_d_n6, assign24350_e33432_d_n7, assign24350_e33432_d_n8, assign24350_e33432_d_n9, assign24350_e33432_d_n10, assign24350_e33432_d_n11, assign24350_e33432_d_n12, assign24350_e33432_d_n13, assign24350_e33432_d_n14,) = {
    if (locals.var_guard609 != 0.0) {
        let assign24350_e33430: f64 = (locals.var_epsratio * p.p77);
        (assign24350_e33430, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign24350_e33432;
        locals.var_t0_dn0 = assign24350_e33432_d_n0;
        locals.var_t0_dn2 = assign24350_e33432_d_n2;
        locals.var_t0_dn3 = assign24350_e33432_d_n3;
        locals.var_t0_dn4 = assign24350_e33432_d_n4;
        locals.var_t0_dn5 = assign24350_e33432_d_n5;
        locals.var_t0_dn6 = assign24350_e33432_d_n6;
        locals.var_t0_dn7 = assign24350_e33432_d_n7;
        locals.var_t0_dn8 = assign24350_e33432_d_n8;
        locals.var_t0_dn9 = assign24350_e33432_d_n9;
        locals.var_t0_dn10 = assign24350_e33432_d_n10;
        locals.var_t0_dn11 = assign24350_e33432_d_n11;
        locals.var_t0_dn12 = assign24350_e33432_d_n12;
        locals.var_t0_dn13 = assign24350_e33432_d_n13;
        locals.var_t0_dn14 = assign24350_e33432_d_n14;
        locals.var_t0_rv = 0.0;

        let assign24360_e33443: f64 = if (((locals.var_agidl_i <= 0.0) || (locals.var_bgidl_t <= 0.0)) || (locals.var_cgidl_i < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard610 = assign24360_e33443;
        locals.var_guard610_rv = 0.0;

        let (assign24370_e33449, assign24370_e33449_d_n0, assign24370_e33449_d_n2, assign24370_e33449_d_n3, assign24370_e33449_d_n4, assign24370_e33449_d_n5, assign24370_e33449_d_n6, assign24370_e33449_d_n7, assign24370_e33449_d_n8, assign24370_e33449_d_n9, assign24370_e33449_d_n10, assign24370_e33449_d_n11, assign24370_e33449_d_n12, assign24370_e33449_d_n13, assign24370_e33449_d_n14,) = {
    if ((locals.var_guard609 != 0.0) && (locals.var_guard610 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign24370_e33449;
        locals.var_t6_dn0 = assign24370_e33449_d_n0;
        locals.var_t6_dn2 = assign24370_e33449_d_n2;
        locals.var_t6_dn3 = assign24370_e33449_d_n3;
        locals.var_t6_dn4 = assign24370_e33449_d_n4;
        locals.var_t6_dn5 = assign24370_e33449_d_n5;
        locals.var_t6_dn6 = assign24370_e33449_d_n6;
        locals.var_t6_dn7 = assign24370_e33449_d_n7;
        locals.var_t6_dn8 = assign24370_e33449_d_n8;
        locals.var_t6_dn9 = assign24370_e33449_d_n9;
        locals.var_t6_dn10 = assign24370_e33449_d_n10;
        locals.var_t6_dn11 = assign24370_e33449_d_n11;
        locals.var_t6_dn12 = assign24370_e33449_d_n12;
        locals.var_t6_dn13 = assign24370_e33449_d_n13;
        locals.var_t6_dn14 = assign24370_e33449_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign24380_e33463, assign24380_e33463_d_n0, assign24380_e33463_d_n2, assign24380_e33463_d_n3, assign24380_e33463_d_n4, assign24380_e33463_d_n5, assign24380_e33463_d_n6, assign24380_e33463_d_n7, assign24380_e33463_d_n8, assign24380_e33463_d_n9, assign24380_e33463_d_n10, assign24380_e33463_d_n11, assign24380_e33463_d_n12, assign24380_e33463_d_n13, assign24380_e33463_d_n14,) = {
    if ((locals.var_guard609 != 0.0) && (locals.var_guard610 == 0.0)) {
        let assign24380_e33455: f64 = (-locals.var_vgd_noswap);
        let assign24380_e33457: f64 = (assign24380_e33455 - locals.var_egidl_i);
        let assign24380_e33459: f64 = (assign24380_e33457 + locals.var_vfbsdr);
        let assign24380_e33461: f64 = (assign24380_e33459 / locals.var_t0);
        (assign24380_e33461, (-((assign24380_e33459 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((assign24380_e33459 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((assign24380_e33459 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign24380_e33459 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgd_noswap_dn5) * locals.var_t0) - (assign24380_e33459 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (-((assign24380_e33459 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgd_noswap_dn7) * locals.var_t0) - (assign24380_e33459 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (-((assign24380_e33459 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgd_noswap_dn9) * locals.var_t0) - (assign24380_e33459 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (-((assign24380_e33459 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgd_noswap_dn11) * locals.var_t0) - (assign24380_e33459 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (-((assign24380_e33459 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))), (-((assign24380_e33459 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))), (-((assign24380_e33459 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign24380_e33463;
        locals.var_t1_dn0 = assign24380_e33463_d_n0;
        locals.var_t1_dn2 = assign24380_e33463_d_n2;
        locals.var_t1_dn3 = assign24380_e33463_d_n3;
        locals.var_t1_dn4 = assign24380_e33463_d_n4;
        locals.var_t1_dn5 = assign24380_e33463_d_n5;
        locals.var_t1_dn6 = assign24380_e33463_d_n6;
        locals.var_t1_dn7 = assign24380_e33463_d_n7;
        locals.var_t1_dn8 = assign24380_e33463_d_n8;
        locals.var_t1_dn9 = assign24380_e33463_d_n9;
        locals.var_t1_dn10 = assign24380_e33463_d_n10;
        locals.var_t1_dn11 = assign24380_e33463_d_n11;
        locals.var_t1_dn12 = assign24380_e33463_d_n12;
        locals.var_t1_dn13 = assign24380_e33463_d_n13;
        locals.var_t1_dn14 = assign24380_e33463_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign24390_e33505, assign24390_e33505_d_n0, assign24390_e33505_d_n2, assign24390_e33505_d_n3, assign24390_e33505_d_n4, assign24390_e33505_d_n5, assign24390_e33505_d_n6, assign24390_e33505_d_n7, assign24390_e33505_d_n8, assign24390_e33505_d_n9, assign24390_e33505_d_n10, assign24390_e33505_d_n11, assign24390_e33505_d_n12, assign24390_e33505_d_n13, assign24390_e33505_d_n14,) = {
    if ((locals.var_guard609 != 0.0) && (locals.var_guard610 == 0.0)) {
        let assign24390_e33470: f64 = (-10000.0);
        let assign24390_e33472: f64 = (assign24390_e33470 * 0.01);
        let (assign24390_e33503, assign24390_e33503_d_n0, assign24390_e33503_d_n2, assign24390_e33503_d_n3, assign24390_e33503_d_n4, assign24390_e33503_d_n5, assign24390_e33503_d_n6, assign24390_e33503_d_n7, assign24390_e33503_d_n8, assign24390_e33503_d_n9, assign24390_e33503_d_n10, assign24390_e33503_d_n11, assign24390_e33503_d_n12, assign24390_e33503_d_n13, assign24390_e33503_d_n14,) = {
            if (!(locals.var_t1 < assign24390_e33472)) {
                let assign24390_e33479: f64 = (locals.var_t1 * locals.var_t1);
                let assign24390_e33482: f64 = (4.0 * 0.01);
                let assign24390_e33484: f64 = (assign24390_e33482 * 0.01);
                let assign24390_e33485: f64 = (assign24390_e33479 + assign24390_e33484);
                let assign24390_e33486: f64 = (assign24390_e33485).sqrt();
                let assign24390_e33487: f64 = (locals.var_t1 + assign24390_e33486);
                let assign24390_e33488: f64 = (0.5 * assign24390_e33487);
                (assign24390_e33488, (0.5 * (locals.var_t1_dn0 + (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign24390_e33486)))), (0.5 * (locals.var_t1_dn2 + (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign24390_e33486)))), (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign24390_e33486)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign24390_e33486)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign24390_e33486)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign24390_e33486)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign24390_e33486)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign24390_e33486)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign24390_e33486)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign24390_e33486)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign24390_e33486)))), (0.5 * (locals.var_t1_dn12 + (((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) / (2.0 * assign24390_e33486)))), (0.5 * (locals.var_t1_dn13 + (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign24390_e33486)))), (0.5 * (locals.var_t1_dn14 + (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign24390_e33486)))),)
            } else {
                let assign24390_e33491: f64 = (-10000.0);
                let assign24390_e33493: f64 = (assign24390_e33491 * 0.01);
                let (assign24390_e33502, assign24390_e33502_d_n0, assign24390_e33502_d_n2, assign24390_e33502_d_n3, assign24390_e33502_d_n4, assign24390_e33502_d_n5, assign24390_e33502_d_n6, assign24390_e33502_d_n7, assign24390_e33502_d_n8, assign24390_e33502_d_n9, assign24390_e33502_d_n10, assign24390_e33502_d_n11, assign24390_e33502_d_n12, assign24390_e33502_d_n13, assign24390_e33502_d_n14,) = {
                    if (locals.var_t1 < assign24390_e33493) {
                        let assign24390_e33496: f64 = (-0.01);
                        let assign24390_e33498: f64 = (assign24390_e33496 * 0.01);
                        let assign24390_e33500: f64 = (assign24390_e33498 / locals.var_t1);
                        (assign24390_e33500, (-((assign24390_e33498 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((assign24390_e33498 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((assign24390_e33498 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1))), (-((assign24390_e33498 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((assign24390_e33498 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((assign24390_e33498 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((assign24390_e33498 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((assign24390_e33498 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((assign24390_e33498 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((assign24390_e33498 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((assign24390_e33498 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((assign24390_e33498 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), (-((assign24390_e33498 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))), (-((assign24390_e33498 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign24390_e33502, assign24390_e33502_d_n0, assign24390_e33502_d_n2, assign24390_e33502_d_n3, assign24390_e33502_d_n4, assign24390_e33502_d_n5, assign24390_e33502_d_n6, assign24390_e33502_d_n7, assign24390_e33502_d_n8, assign24390_e33502_d_n9, assign24390_e33502_d_n10, assign24390_e33502_d_n11, assign24390_e33502_d_n12, assign24390_e33502_d_n13, assign24390_e33502_d_n14,)
            }
        };
        (assign24390_e33503, assign24390_e33503_d_n0, assign24390_e33503_d_n2, assign24390_e33503_d_n3, assign24390_e33503_d_n4, assign24390_e33503_d_n5, assign24390_e33503_d_n6, assign24390_e33503_d_n7, assign24390_e33503_d_n8, assign24390_e33503_d_n9, assign24390_e33503_d_n10, assign24390_e33503_d_n11, assign24390_e33503_d_n12, assign24390_e33503_d_n13, assign24390_e33503_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign24390_e33505;
        locals.var_t1_dn0 = assign24390_e33505_d_n0;
        locals.var_t1_dn2 = assign24390_e33505_d_n2;
        locals.var_t1_dn3 = assign24390_e33505_d_n3;
        locals.var_t1_dn4 = assign24390_e33505_d_n4;
        locals.var_t1_dn5 = assign24390_e33505_d_n5;
        locals.var_t1_dn6 = assign24390_e33505_d_n6;
        locals.var_t1_dn7 = assign24390_e33505_d_n7;
        locals.var_t1_dn8 = assign24390_e33505_d_n8;
        locals.var_t1_dn9 = assign24390_e33505_d_n9;
        locals.var_t1_dn10 = assign24390_e33505_d_n10;
        locals.var_t1_dn11 = assign24390_e33505_d_n11;
        locals.var_t1_dn12 = assign24390_e33505_d_n12;
        locals.var_t1_dn13 = assign24390_e33505_d_n13;
        locals.var_t1_dn14 = assign24390_e33505_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign24400_e33516, assign24400_e33516_d_n0, assign24400_e33516_d_n2, assign24400_e33516_d_n3, assign24400_e33516_d_n4, assign24400_e33516_d_n5, assign24400_e33516_d_n6, assign24400_e33516_d_n7, assign24400_e33516_d_n8, assign24400_e33516_d_n9, assign24400_e33516_d_n10, assign24400_e33516_d_n11, assign24400_e33516_d_n12, assign24400_e33516_d_n13, assign24400_e33516_d_n14,) = {
    if ((locals.var_guard609 != 0.0) && (locals.var_guard610 == 0.0)) {
        let assign24400_e33513: f64 = (locals.var_t1 + 0.001);
        let assign24400_e33514: f64 = (locals.var_bgidl_t / assign24400_e33513);
        (assign24400_e33514, (-((locals.var_bgidl_t * locals.var_t1_dn0) / (assign24400_e33513 * assign24400_e33513))), (-((locals.var_bgidl_t * locals.var_t1_dn2) / (assign24400_e33513 * assign24400_e33513))), (-((locals.var_bgidl_t * locals.var_t1_dn3) / (assign24400_e33513 * assign24400_e33513))), (((locals.var_bgidl_t_dn4 * assign24400_e33513) - (locals.var_bgidl_t * locals.var_t1_dn4)) / (assign24400_e33513 * assign24400_e33513)), (-((locals.var_bgidl_t * locals.var_t1_dn5) / (assign24400_e33513 * assign24400_e33513))), (-((locals.var_bgidl_t * locals.var_t1_dn6) / (assign24400_e33513 * assign24400_e33513))), (-((locals.var_bgidl_t * locals.var_t1_dn7) / (assign24400_e33513 * assign24400_e33513))), (-((locals.var_bgidl_t * locals.var_t1_dn8) / (assign24400_e33513 * assign24400_e33513))), (-((locals.var_bgidl_t * locals.var_t1_dn9) / (assign24400_e33513 * assign24400_e33513))), (-((locals.var_bgidl_t * locals.var_t1_dn10) / (assign24400_e33513 * assign24400_e33513))), (-((locals.var_bgidl_t * locals.var_t1_dn11) / (assign24400_e33513 * assign24400_e33513))), (-((locals.var_bgidl_t * locals.var_t1_dn12) / (assign24400_e33513 * assign24400_e33513))), (-((locals.var_bgidl_t * locals.var_t1_dn13) / (assign24400_e33513 * assign24400_e33513))), (-((locals.var_bgidl_t * locals.var_t1_dn14) / (assign24400_e33513 * assign24400_e33513))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign24400_e33516;
        locals.var_t2_dn0 = assign24400_e33516_d_n0;
        locals.var_t2_dn2 = assign24400_e33516_d_n2;
        locals.var_t2_dn3 = assign24400_e33516_d_n3;
        locals.var_t2_dn4 = assign24400_e33516_d_n4;
        locals.var_t2_dn5 = assign24400_e33516_d_n5;
        locals.var_t2_dn6 = assign24400_e33516_d_n6;
        locals.var_t2_dn7 = assign24400_e33516_d_n7;
        locals.var_t2_dn8 = assign24400_e33516_d_n8;
        locals.var_t2_dn9 = assign24400_e33516_d_n9;
        locals.var_t2_dn10 = assign24400_e33516_d_n10;
        locals.var_t2_dn11 = assign24400_e33516_d_n11;
        locals.var_t2_dn12 = assign24400_e33516_d_n12;
        locals.var_t2_dn13 = assign24400_e33516_d_n13;
        locals.var_t2_dn14 = assign24400_e33516_d_n14;
        locals.var_t2_rv = 0.0;

        let assign24410_e33519: f64 = if locals.var_cgidl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard611 = assign24410_e33519;
        locals.var_guard611_rv = 0.0;

        let (assign24420_e33532, assign24420_e33532_d_n0, assign24420_e33532_d_n2, assign24420_e33532_d_n3, assign24420_e33532_d_n4, assign24420_e33532_d_n5, assign24420_e33532_d_n6, assign24420_e33532_d_n7, assign24420_e33532_d_n8, assign24420_e33532_d_n9, assign24420_e33532_d_n10, assign24420_e33532_d_n11, assign24420_e33532_d_n12, assign24420_e33532_d_n13, assign24420_e33532_d_n14,) = {
    if (((locals.var_guard609 != 0.0) && (locals.var_guard610 == 0.0)) && (locals.var_guard611 != 0.0)) {
        let assign24420_e33528: f64 = (locals.var_vdb_noswap * locals.var_vdb_noswap);
        let assign24420_e33530: f64 = (assign24420_e33528 * locals.var_vdb_noswap);
        (assign24420_e33530, 0.0, 0.0, 0.0, 0.0, ((((locals.var_vdb_noswap_dn5 * locals.var_vdb_noswap) + (locals.var_vdb_noswap * locals.var_vdb_noswap_dn5)) * locals.var_vdb_noswap) + (assign24420_e33528 * locals.var_vdb_noswap_dn5)), 0.0, ((((locals.var_vdb_noswap_dn7 * locals.var_vdb_noswap) + (locals.var_vdb_noswap * locals.var_vdb_noswap_dn7)) * locals.var_vdb_noswap) + (assign24420_e33528 * locals.var_vdb_noswap_dn7)), 0.0, 0.0, 0.0, ((((locals.var_vdb_noswap_dn11 * locals.var_vdb_noswap) + (locals.var_vdb_noswap * locals.var_vdb_noswap_dn11)) * locals.var_vdb_noswap) + (assign24420_e33528 * locals.var_vdb_noswap_dn11)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign24420_e33532;
        locals.var_t3_dn0 = assign24420_e33532_d_n0;
        locals.var_t3_dn2 = assign24420_e33532_d_n2;
        locals.var_t3_dn3 = assign24420_e33532_d_n3;
        locals.var_t3_dn4 = assign24420_e33532_d_n4;
        locals.var_t3_dn5 = assign24420_e33532_d_n5;
        locals.var_t3_dn6 = assign24420_e33532_d_n6;
        locals.var_t3_dn7 = assign24420_e33532_d_n7;
        locals.var_t3_dn8 = assign24420_e33532_d_n8;
        locals.var_t3_dn9 = assign24420_e33532_d_n9;
        locals.var_t3_dn10 = assign24420_e33532_d_n10;
        locals.var_t3_dn11 = assign24420_e33532_d_n11;
        locals.var_t3_dn12 = assign24420_e33532_d_n12;
        locals.var_t3_dn13 = assign24420_e33532_d_n13;
        locals.var_t3_dn14 = assign24420_e33532_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign24430_e33546, assign24430_e33546_d_n0, assign24430_e33546_d_n2, assign24430_e33546_d_n3, assign24430_e33546_d_n4, assign24430_e33546_d_n5, assign24430_e33546_d_n6, assign24430_e33546_d_n7, assign24430_e33546_d_n8, assign24430_e33546_d_n9, assign24430_e33546_d_n10, assign24430_e33546_d_n11, assign24430_e33546_d_n12, assign24430_e33546_d_n13, assign24430_e33546_d_n14,) = {
    if (((locals.var_guard609 != 0.0) && (locals.var_guard610 == 0.0)) && (locals.var_guard611 != 0.0)) {
        let assign24430_e33541: f64 = (locals.var_t3).abs();
        let assign24430_e33542: f64 = (locals.var_cgidl_i + assign24430_e33541);
        let assign24430_e33544: f64 = (assign24430_e33542 + 0.0001);
        (assign24430_e33544, if locals.var_t3 >= 0.0 { locals.var_t3_dn0 } else { (-locals.var_t3_dn0) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn2 } else { (-locals.var_t3_dn2) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn3 } else { (-locals.var_t3_dn3) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn4 } else { (-locals.var_t3_dn4) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn5 } else { (-locals.var_t3_dn5) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn6 } else { (-locals.var_t3_dn6) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn7 } else { (-locals.var_t3_dn7) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn8 } else { (-locals.var_t3_dn8) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn9 } else { (-locals.var_t3_dn9) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn10 } else { (-locals.var_t3_dn10) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn11 } else { (-locals.var_t3_dn11) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn12 } else { (-locals.var_t3_dn12) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn13 } else { (-locals.var_t3_dn13) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn14 } else { (-locals.var_t3_dn14) },)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign24430_e33546;
        locals.var_t4_dn0 = assign24430_e33546_d_n0;
        locals.var_t4_dn2 = assign24430_e33546_d_n2;
        locals.var_t4_dn3 = assign24430_e33546_d_n3;
        locals.var_t4_dn4 = assign24430_e33546_d_n4;
        locals.var_t4_dn5 = assign24430_e33546_d_n5;
        locals.var_t4_dn6 = assign24430_e33546_d_n6;
        locals.var_t4_dn7 = assign24430_e33546_d_n7;
        locals.var_t4_dn8 = assign24430_e33546_d_n8;
        locals.var_t4_dn9 = assign24430_e33546_d_n9;
        locals.var_t4_dn10 = assign24430_e33546_d_n10;
        locals.var_t4_dn11 = assign24430_e33546_d_n11;
        locals.var_t4_dn12 = assign24430_e33546_d_n12;
        locals.var_t4_dn13 = assign24430_e33546_d_n13;
        locals.var_t4_dn14 = assign24430_e33546_d_n14;
        locals.var_t4_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_72(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign24440_e33604, assign24440_e33604_d_n0, assign24440_e33604_d_n2, assign24440_e33604_d_n3, assign24440_e33604_d_n4, assign24440_e33604_d_n5, assign24440_e33604_d_n6, assign24440_e33604_d_n7, assign24440_e33604_d_n8, assign24440_e33604_d_n9, assign24440_e33604_d_n10, assign24440_e33604_d_n11, assign24440_e33604_d_n12, assign24440_e33604_d_n13, assign24440_e33604_d_n14,) = {
    if (((locals.var_guard609 != 0.0) && (locals.var_guard610 == 0.0)) && (locals.var_guard611 != 0.0)) {
        let assign24440_e33555: f64 = (locals.var_t3 / locals.var_t4);
        let assign24440_e33557: f64 = (-10000.0);
        let assign24440_e33559: f64 = (assign24440_e33557 * 1e-6);
        let (assign24440_e33600, assign24440_e33600_d_n0, assign24440_e33600_d_n2, assign24440_e33600_d_n3, assign24440_e33600_d_n4, assign24440_e33600_d_n5, assign24440_e33600_d_n6, assign24440_e33600_d_n7, assign24440_e33600_d_n8, assign24440_e33600_d_n9, assign24440_e33600_d_n10, assign24440_e33600_d_n11, assign24440_e33600_d_n12, assign24440_e33600_d_n13, assign24440_e33600_d_n14,) = {
            if (!(assign24440_e33555 < assign24440_e33559)) {
                let __rspice_inv_cse_0: f64 = 1.0 / locals.var_t4;
                let assign24440_e33565: f64 = (locals.var_t3 * __rspice_inv_cse_0);
                let assign24440_e33568: f64 = (locals.var_t3 * __rspice_inv_cse_0);
                let assign24440_e33571: f64 = (locals.var_t3 * __rspice_inv_cse_0);
                let assign24440_e33572: f64 = (assign24440_e33568 * assign24440_e33571);
                let assign24440_e33575: f64 = (4.0 * 1e-6);
                let assign24440_e33577: f64 = (assign24440_e33575 * 1e-6);
                let assign24440_e33578: f64 = (assign24440_e33572 + assign24440_e33577);
                let assign24440_e33579: f64 = (assign24440_e33578).sqrt();
                let assign24440_e33580: f64 = (assign24440_e33565 + assign24440_e33579);
                let assign24440_e33581: f64 = (0.5 * assign24440_e33580);
                (assign24440_e33581, (0.5 * ((((locals.var_t3_dn0 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn0 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)) * assign24440_e33571) + (assign24440_e33568 * (((locals.var_t3_dn0 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24440_e33579)))), (0.5 * ((((locals.var_t3_dn2 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn2 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)) * assign24440_e33571) + (assign24440_e33568 * (((locals.var_t3_dn2 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24440_e33579)))), (0.5 * ((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) * assign24440_e33571) + (assign24440_e33568 * (((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24440_e33579)))), (0.5 * ((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * assign24440_e33571) + (assign24440_e33568 * (((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24440_e33579)))), (0.5 * ((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * assign24440_e33571) + (assign24440_e33568 * (((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24440_e33579)))), (0.5 * ((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * assign24440_e33571) + (assign24440_e33568 * (((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24440_e33579)))), (0.5 * ((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) * assign24440_e33571) + (assign24440_e33568 * (((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24440_e33579)))), (0.5 * ((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * assign24440_e33571) + (assign24440_e33568 * (((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24440_e33579)))), (0.5 * ((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) * assign24440_e33571) + (assign24440_e33568 * (((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24440_e33579)))), (0.5 * ((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * assign24440_e33571) + (assign24440_e33568 * (((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24440_e33579)))), (0.5 * ((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * assign24440_e33571) + (assign24440_e33568 * (((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24440_e33579)))), (0.5 * ((((locals.var_t3_dn12 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn12)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn12 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn12)) / (locals.var_t4 * locals.var_t4)) * assign24440_e33571) + (assign24440_e33568 * (((locals.var_t3_dn12 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn12)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24440_e33579)))), (0.5 * ((((locals.var_t3_dn13 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn13 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4)) * assign24440_e33571) + (assign24440_e33568 * (((locals.var_t3_dn13 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24440_e33579)))), (0.5 * ((((locals.var_t3_dn14 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn14 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)) * assign24440_e33571) + (assign24440_e33568 * (((locals.var_t3_dn14 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24440_e33579)))),)
            } else {
                let assign24440_e33584: f64 = (locals.var_t3 / locals.var_t4);
                let assign24440_e33586: f64 = (-10000.0);
                let assign24440_e33588: f64 = (assign24440_e33586 * 1e-6);
                let (assign24440_e33599, assign24440_e33599_d_n0, assign24440_e33599_d_n2, assign24440_e33599_d_n3, assign24440_e33599_d_n4, assign24440_e33599_d_n5, assign24440_e33599_d_n6, assign24440_e33599_d_n7, assign24440_e33599_d_n8, assign24440_e33599_d_n9, assign24440_e33599_d_n10, assign24440_e33599_d_n11, assign24440_e33599_d_n12, assign24440_e33599_d_n13, assign24440_e33599_d_n14,) = {
                    if (assign24440_e33584 < assign24440_e33588) {
                        let assign24440_e33591: f64 = (-1e-6);
                        let assign24440_e33593: f64 = (assign24440_e33591 * 1e-6);
                        let assign24440_e33596: f64 = (locals.var_t3 / locals.var_t4);
                        let assign24440_e33597: f64 = (assign24440_e33593 / assign24440_e33596);
                        (assign24440_e33597, (-((assign24440_e33593 * (((locals.var_t3_dn0 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4))) / (assign24440_e33596 * assign24440_e33596))), (-((assign24440_e33593 * (((locals.var_t3_dn2 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4))) / (assign24440_e33596 * assign24440_e33596))), (-((assign24440_e33593 * (((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4))) / (assign24440_e33596 * assign24440_e33596))), (-((assign24440_e33593 * (((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4))) / (assign24440_e33596 * assign24440_e33596))), (-((assign24440_e33593 * (((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4))) / (assign24440_e33596 * assign24440_e33596))), (-((assign24440_e33593 * (((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4))) / (assign24440_e33596 * assign24440_e33596))), (-((assign24440_e33593 * (((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4))) / (assign24440_e33596 * assign24440_e33596))), (-((assign24440_e33593 * (((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4))) / (assign24440_e33596 * assign24440_e33596))), (-((assign24440_e33593 * (((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4))) / (assign24440_e33596 * assign24440_e33596))), (-((assign24440_e33593 * (((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4))) / (assign24440_e33596 * assign24440_e33596))), (-((assign24440_e33593 * (((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4))) / (assign24440_e33596 * assign24440_e33596))), (-((assign24440_e33593 * (((locals.var_t3_dn12 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn12)) / (locals.var_t4 * locals.var_t4))) / (assign24440_e33596 * assign24440_e33596))), (-((assign24440_e33593 * (((locals.var_t3_dn13 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4))) / (assign24440_e33596 * assign24440_e33596))), (-((assign24440_e33593 * (((locals.var_t3_dn14 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4))) / (assign24440_e33596 * assign24440_e33596))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign24440_e33599, assign24440_e33599_d_n0, assign24440_e33599_d_n2, assign24440_e33599_d_n3, assign24440_e33599_d_n4, assign24440_e33599_d_n5, assign24440_e33599_d_n6, assign24440_e33599_d_n7, assign24440_e33599_d_n8, assign24440_e33599_d_n9, assign24440_e33599_d_n10, assign24440_e33599_d_n11, assign24440_e33599_d_n12, assign24440_e33599_d_n13, assign24440_e33599_d_n14,)
            }
        };
        let assign24440_e33602: f64 = (assign24440_e33600 - 1e-6);
        (assign24440_e33602, assign24440_e33600_d_n0, assign24440_e33600_d_n2, assign24440_e33600_d_n3, assign24440_e33600_d_n4, assign24440_e33600_d_n5, assign24440_e33600_d_n6, assign24440_e33600_d_n7, assign24440_e33600_d_n8, assign24440_e33600_d_n9, assign24440_e33600_d_n10, assign24440_e33600_d_n11, assign24440_e33600_d_n12, assign24440_e33600_d_n13, assign24440_e33600_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign24440_e33604;
        locals.var_t5_dn0 = assign24440_e33604_d_n0;
        locals.var_t5_dn2 = assign24440_e33604_d_n2;
        locals.var_t5_dn3 = assign24440_e33604_d_n3;
        locals.var_t5_dn4 = assign24440_e33604_d_n4;
        locals.var_t5_dn5 = assign24440_e33604_d_n5;
        locals.var_t5_dn6 = assign24440_e33604_d_n6;
        locals.var_t5_dn7 = assign24440_e33604_d_n7;
        locals.var_t5_dn8 = assign24440_e33604_d_n8;
        locals.var_t5_dn9 = assign24440_e33604_d_n9;
        locals.var_t5_dn10 = assign24440_e33604_d_n10;
        locals.var_t5_dn11 = assign24440_e33604_d_n11;
        locals.var_t5_dn12 = assign24440_e33604_d_n12;
        locals.var_t5_dn13 = assign24440_e33604_d_n13;
        locals.var_t5_dn14 = assign24440_e33604_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign24450_e33614, assign24450_e33614_d_n0, assign24450_e33614_d_n2, assign24450_e33614_d_n3, assign24450_e33614_d_n4, assign24450_e33614_d_n5, assign24450_e33614_d_n6, assign24450_e33614_d_n7, assign24450_e33614_d_n8, assign24450_e33614_d_n9, assign24450_e33614_d_n10, assign24450_e33614_d_n11, assign24450_e33614_d_n12, assign24450_e33614_d_n13, assign24450_e33614_d_n14,) = {
    if (((locals.var_guard609 != 0.0) && (locals.var_guard610 == 0.0)) && (locals.var_guard611 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign24450_e33614;
        locals.var_t5_dn0 = assign24450_e33614_d_n0;
        locals.var_t5_dn2 = assign24450_e33614_d_n2;
        locals.var_t5_dn3 = assign24450_e33614_d_n3;
        locals.var_t5_dn4 = assign24450_e33614_d_n4;
        locals.var_t5_dn5 = assign24450_e33614_d_n5;
        locals.var_t5_dn6 = assign24450_e33614_d_n6;
        locals.var_t5_dn7 = assign24450_e33614_d_n7;
        locals.var_t5_dn8 = assign24450_e33614_d_n8;
        locals.var_t5_dn9 = assign24450_e33614_d_n9;
        locals.var_t5_dn10 = assign24450_e33614_d_n10;
        locals.var_t5_dn11 = assign24450_e33614_d_n11;
        locals.var_t5_dn12 = assign24450_e33614_d_n12;
        locals.var_t5_dn13 = assign24450_e33614_d_n13;
        locals.var_t5_dn14 = assign24450_e33614_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign24460_e33631, assign24460_e33631_d_n0, assign24460_e33631_d_n2, assign24460_e33631_d_n3, assign24460_e33631_d_n4, assign24460_e33631_d_n5, assign24460_e33631_d_n6, assign24460_e33631_d_n7, assign24460_e33631_d_n8, assign24460_e33631_d_n9, assign24460_e33631_d_n10, assign24460_e33631_d_n11, assign24460_e33631_d_n12, assign24460_e33631_d_n13, assign24460_e33631_d_n14,) = {
    if ((locals.var_guard609 != 0.0) && (locals.var_guard610 == 0.0)) {
        let assign24460_e33621: f64 = (locals.var_agidl_i * locals.var_weff);
        let assign24460_e33623: f64 = (assign24460_e33621 * locals.var_t1);
        let assign24460_e33625: f64 = (-locals.var_t2);
        let assign24460_e33626: f64 = { let limited_exp_arg = assign24460_e33625; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign24460_e33627: f64 = (assign24460_e33623 * assign24460_e33626);
        let assign24460_e33629: f64 = (assign24460_e33627 * locals.var_t5);
        (assign24460_e33629, (((((assign24460_e33621 * locals.var_t1_dn0) * assign24460_e33626) + (assign24460_e33623 * ({ let limited_exp_arg = assign24460_e33625; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn0)))) * locals.var_t5) + (assign24460_e33627 * locals.var_t5_dn0)), (((((assign24460_e33621 * locals.var_t1_dn2) * assign24460_e33626) + (assign24460_e33623 * ({ let limited_exp_arg = assign24460_e33625; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn2)))) * locals.var_t5) + (assign24460_e33627 * locals.var_t5_dn2)), (((((assign24460_e33621 * locals.var_t1_dn3) * assign24460_e33626) + (assign24460_e33623 * ({ let limited_exp_arg = assign24460_e33625; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * locals.var_t5) + (assign24460_e33627 * locals.var_t5_dn3)), (((((assign24460_e33621 * locals.var_t1_dn4) * assign24460_e33626) + (assign24460_e33623 * ({ let limited_exp_arg = assign24460_e33625; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * locals.var_t5) + (assign24460_e33627 * locals.var_t5_dn4)), (((((assign24460_e33621 * locals.var_t1_dn5) * assign24460_e33626) + (assign24460_e33623 * ({ let limited_exp_arg = assign24460_e33625; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * locals.var_t5) + (assign24460_e33627 * locals.var_t5_dn5)), (((((assign24460_e33621 * locals.var_t1_dn6) * assign24460_e33626) + (assign24460_e33623 * ({ let limited_exp_arg = assign24460_e33625; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * locals.var_t5) + (assign24460_e33627 * locals.var_t5_dn6)), (((((assign24460_e33621 * locals.var_t1_dn7) * assign24460_e33626) + (assign24460_e33623 * ({ let limited_exp_arg = assign24460_e33625; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * locals.var_t5) + (assign24460_e33627 * locals.var_t5_dn7)), (((((assign24460_e33621 * locals.var_t1_dn8) * assign24460_e33626) + (assign24460_e33623 * ({ let limited_exp_arg = assign24460_e33625; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * locals.var_t5) + (assign24460_e33627 * locals.var_t5_dn8)), (((((assign24460_e33621 * locals.var_t1_dn9) * assign24460_e33626) + (assign24460_e33623 * ({ let limited_exp_arg = assign24460_e33625; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * locals.var_t5) + (assign24460_e33627 * locals.var_t5_dn9)), (((((assign24460_e33621 * locals.var_t1_dn10) * assign24460_e33626) + (assign24460_e33623 * ({ let limited_exp_arg = assign24460_e33625; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * locals.var_t5) + (assign24460_e33627 * locals.var_t5_dn10)), (((((assign24460_e33621 * locals.var_t1_dn11) * assign24460_e33626) + (assign24460_e33623 * ({ let limited_exp_arg = assign24460_e33625; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * locals.var_t5) + (assign24460_e33627 * locals.var_t5_dn11)), (((((assign24460_e33621 * locals.var_t1_dn12) * assign24460_e33626) + (assign24460_e33623 * ({ let limited_exp_arg = assign24460_e33625; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn12)))) * locals.var_t5) + (assign24460_e33627 * locals.var_t5_dn12)), (((((assign24460_e33621 * locals.var_t1_dn13) * assign24460_e33626) + (assign24460_e33623 * ({ let limited_exp_arg = assign24460_e33625; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn13)))) * locals.var_t5) + (assign24460_e33627 * locals.var_t5_dn13)), (((((assign24460_e33621 * locals.var_t1_dn14) * assign24460_e33626) + (assign24460_e33623 * ({ let limited_exp_arg = assign24460_e33625; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn14)))) * locals.var_t5) + (assign24460_e33627 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign24460_e33631;
        locals.var_t6_dn0 = assign24460_e33631_d_n0;
        locals.var_t6_dn2 = assign24460_e33631_d_n2;
        locals.var_t6_dn3 = assign24460_e33631_d_n3;
        locals.var_t6_dn4 = assign24460_e33631_d_n4;
        locals.var_t6_dn5 = assign24460_e33631_d_n5;
        locals.var_t6_dn6 = assign24460_e33631_d_n6;
        locals.var_t6_dn7 = assign24460_e33631_d_n7;
        locals.var_t6_dn8 = assign24460_e33631_d_n8;
        locals.var_t6_dn9 = assign24460_e33631_d_n9;
        locals.var_t6_dn10 = assign24460_e33631_d_n10;
        locals.var_t6_dn11 = assign24460_e33631_d_n11;
        locals.var_t6_dn12 = assign24460_e33631_d_n12;
        locals.var_t6_dn13 = assign24460_e33631_d_n13;
        locals.var_t6_dn14 = assign24460_e33631_d_n14;
        locals.var_t6_rv = 0.0;

        let assign24480_e33646: f64 = if (((locals.var_agisl_i <= 0.0) || (locals.var_bgisl_t <= 0.0)) || (locals.var_cgisl_i < 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard612 = assign24480_e33646;
        locals.var_guard612_rv = 0.0;

        let (assign24490_e33652, assign24490_e33652_d_n0, assign24490_e33652_d_n2, assign24490_e33652_d_n3, assign24490_e33652_d_n4, assign24490_e33652_d_n5, assign24490_e33652_d_n6, assign24490_e33652_d_n7, assign24490_e33652_d_n8, assign24490_e33652_d_n9, assign24490_e33652_d_n10, assign24490_e33652_d_n11, assign24490_e33652_d_n12, assign24490_e33652_d_n13, assign24490_e33652_d_n14,) = {
    if ((locals.var_guard609 != 0.0) && (locals.var_guard612 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign24490_e33652;
        locals.var_t6_dn0 = assign24490_e33652_d_n0;
        locals.var_t6_dn2 = assign24490_e33652_d_n2;
        locals.var_t6_dn3 = assign24490_e33652_d_n3;
        locals.var_t6_dn4 = assign24490_e33652_d_n4;
        locals.var_t6_dn5 = assign24490_e33652_d_n5;
        locals.var_t6_dn6 = assign24490_e33652_d_n6;
        locals.var_t6_dn7 = assign24490_e33652_d_n7;
        locals.var_t6_dn8 = assign24490_e33652_d_n8;
        locals.var_t6_dn9 = assign24490_e33652_d_n9;
        locals.var_t6_dn10 = assign24490_e33652_d_n10;
        locals.var_t6_dn11 = assign24490_e33652_d_n11;
        locals.var_t6_dn12 = assign24490_e33652_d_n12;
        locals.var_t6_dn13 = assign24490_e33652_d_n13;
        locals.var_t6_dn14 = assign24490_e33652_d_n14;
        locals.var_t6_rv = 0.0;

        let (assign24500_e33666, assign24500_e33666_d_n0, assign24500_e33666_d_n2, assign24500_e33666_d_n3, assign24500_e33666_d_n4, assign24500_e33666_d_n5, assign24500_e33666_d_n6, assign24500_e33666_d_n7, assign24500_e33666_d_n8, assign24500_e33666_d_n9, assign24500_e33666_d_n10, assign24500_e33666_d_n11, assign24500_e33666_d_n12, assign24500_e33666_d_n13, assign24500_e33666_d_n14,) = {
    if ((locals.var_guard609 != 0.0) && (locals.var_guard612 == 0.0)) {
        let assign24500_e33658: f64 = (-locals.var_vgs_noswap);
        let assign24500_e33660: f64 = (assign24500_e33658 - locals.var_egisl_i);
        let assign24500_e33662: f64 = (assign24500_e33660 + locals.var_vfbsdr);
        let assign24500_e33664: f64 = (assign24500_e33662 / locals.var_t0);
        (assign24500_e33664, (-((assign24500_e33662 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((assign24500_e33662 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((assign24500_e33662 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0))), (((locals.var_vfbsdr_dn4 * locals.var_t0) - (assign24500_e33662 * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), ((((-locals.var_vgs_noswap_dn5) * locals.var_t0) - (assign24500_e33662 * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (-((assign24500_e33662 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgs_noswap_dn7) * locals.var_t0) - (assign24500_e33662 * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (-((assign24500_e33662 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgs_noswap_dn9) * locals.var_t0) - (assign24500_e33662 * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (-((assign24500_e33662 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), ((((-locals.var_vgs_noswap_dn11) * locals.var_t0) - (assign24500_e33662 * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (-((assign24500_e33662 * locals.var_t0_dn12) / (locals.var_t0 * locals.var_t0))), (-((assign24500_e33662 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0))), (-((assign24500_e33662 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign24500_e33666;
        locals.var_t1_dn0 = assign24500_e33666_d_n0;
        locals.var_t1_dn2 = assign24500_e33666_d_n2;
        locals.var_t1_dn3 = assign24500_e33666_d_n3;
        locals.var_t1_dn4 = assign24500_e33666_d_n4;
        locals.var_t1_dn5 = assign24500_e33666_d_n5;
        locals.var_t1_dn6 = assign24500_e33666_d_n6;
        locals.var_t1_dn7 = assign24500_e33666_d_n7;
        locals.var_t1_dn8 = assign24500_e33666_d_n8;
        locals.var_t1_dn9 = assign24500_e33666_d_n9;
        locals.var_t1_dn10 = assign24500_e33666_d_n10;
        locals.var_t1_dn11 = assign24500_e33666_d_n11;
        locals.var_t1_dn12 = assign24500_e33666_d_n12;
        locals.var_t1_dn13 = assign24500_e33666_d_n13;
        locals.var_t1_dn14 = assign24500_e33666_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign24510_e33708, assign24510_e33708_d_n0, assign24510_e33708_d_n2, assign24510_e33708_d_n3, assign24510_e33708_d_n4, assign24510_e33708_d_n5, assign24510_e33708_d_n6, assign24510_e33708_d_n7, assign24510_e33708_d_n8, assign24510_e33708_d_n9, assign24510_e33708_d_n10, assign24510_e33708_d_n11, assign24510_e33708_d_n12, assign24510_e33708_d_n13, assign24510_e33708_d_n14,) = {
    if ((locals.var_guard609 != 0.0) && (locals.var_guard612 == 0.0)) {
        let assign24510_e33673: f64 = (-10000.0);
        let assign24510_e33675: f64 = (assign24510_e33673 * 0.01);
        let (assign24510_e33706, assign24510_e33706_d_n0, assign24510_e33706_d_n2, assign24510_e33706_d_n3, assign24510_e33706_d_n4, assign24510_e33706_d_n5, assign24510_e33706_d_n6, assign24510_e33706_d_n7, assign24510_e33706_d_n8, assign24510_e33706_d_n9, assign24510_e33706_d_n10, assign24510_e33706_d_n11, assign24510_e33706_d_n12, assign24510_e33706_d_n13, assign24510_e33706_d_n14,) = {
            if (!(locals.var_t1 < assign24510_e33675)) {
                let assign24510_e33682: f64 = (locals.var_t1 * locals.var_t1);
                let assign24510_e33685: f64 = (4.0 * 0.01);
                let assign24510_e33687: f64 = (assign24510_e33685 * 0.01);
                let assign24510_e33688: f64 = (assign24510_e33682 + assign24510_e33687);
                let assign24510_e33689: f64 = (assign24510_e33688).sqrt();
                let assign24510_e33690: f64 = (locals.var_t1 + assign24510_e33689);
                let assign24510_e33691: f64 = (0.5 * assign24510_e33690);
                (assign24510_e33691, (0.5 * (locals.var_t1_dn0 + (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) / (2.0 * assign24510_e33689)))), (0.5 * (locals.var_t1_dn2 + (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) / (2.0 * assign24510_e33689)))), (0.5 * (locals.var_t1_dn3 + (((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) / (2.0 * assign24510_e33689)))), (0.5 * (locals.var_t1_dn4 + (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) / (2.0 * assign24510_e33689)))), (0.5 * (locals.var_t1_dn5 + (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) / (2.0 * assign24510_e33689)))), (0.5 * (locals.var_t1_dn6 + (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) / (2.0 * assign24510_e33689)))), (0.5 * (locals.var_t1_dn7 + (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) / (2.0 * assign24510_e33689)))), (0.5 * (locals.var_t1_dn8 + (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) / (2.0 * assign24510_e33689)))), (0.5 * (locals.var_t1_dn9 + (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) / (2.0 * assign24510_e33689)))), (0.5 * (locals.var_t1_dn10 + (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) / (2.0 * assign24510_e33689)))), (0.5 * (locals.var_t1_dn11 + (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) / (2.0 * assign24510_e33689)))), (0.5 * (locals.var_t1_dn12 + (((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) / (2.0 * assign24510_e33689)))), (0.5 * (locals.var_t1_dn13 + (((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) / (2.0 * assign24510_e33689)))), (0.5 * (locals.var_t1_dn14 + (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) / (2.0 * assign24510_e33689)))),)
            } else {
                let assign24510_e33694: f64 = (-10000.0);
                let assign24510_e33696: f64 = (assign24510_e33694 * 0.01);
                let (assign24510_e33705, assign24510_e33705_d_n0, assign24510_e33705_d_n2, assign24510_e33705_d_n3, assign24510_e33705_d_n4, assign24510_e33705_d_n5, assign24510_e33705_d_n6, assign24510_e33705_d_n7, assign24510_e33705_d_n8, assign24510_e33705_d_n9, assign24510_e33705_d_n10, assign24510_e33705_d_n11, assign24510_e33705_d_n12, assign24510_e33705_d_n13, assign24510_e33705_d_n14,) = {
                    if (locals.var_t1 < assign24510_e33696) {
                        let assign24510_e33699: f64 = (-0.01);
                        let assign24510_e33701: f64 = (assign24510_e33699 * 0.01);
                        let assign24510_e33703: f64 = (assign24510_e33701 / locals.var_t1);
                        (assign24510_e33703, (-((assign24510_e33701 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((assign24510_e33701 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((assign24510_e33701 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1))), (-((assign24510_e33701 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((assign24510_e33701 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((assign24510_e33701 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((assign24510_e33701 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((assign24510_e33701 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((assign24510_e33701 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((assign24510_e33701 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((assign24510_e33701 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((assign24510_e33701 * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), (-((assign24510_e33701 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))), (-((assign24510_e33701 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign24510_e33705, assign24510_e33705_d_n0, assign24510_e33705_d_n2, assign24510_e33705_d_n3, assign24510_e33705_d_n4, assign24510_e33705_d_n5, assign24510_e33705_d_n6, assign24510_e33705_d_n7, assign24510_e33705_d_n8, assign24510_e33705_d_n9, assign24510_e33705_d_n10, assign24510_e33705_d_n11, assign24510_e33705_d_n12, assign24510_e33705_d_n13, assign24510_e33705_d_n14,)
            }
        };
        (assign24510_e33706, assign24510_e33706_d_n0, assign24510_e33706_d_n2, assign24510_e33706_d_n3, assign24510_e33706_d_n4, assign24510_e33706_d_n5, assign24510_e33706_d_n6, assign24510_e33706_d_n7, assign24510_e33706_d_n8, assign24510_e33706_d_n9, assign24510_e33706_d_n10, assign24510_e33706_d_n11, assign24510_e33706_d_n12, assign24510_e33706_d_n13, assign24510_e33706_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign24510_e33708;
        locals.var_t1_dn0 = assign24510_e33708_d_n0;
        locals.var_t1_dn2 = assign24510_e33708_d_n2;
        locals.var_t1_dn3 = assign24510_e33708_d_n3;
        locals.var_t1_dn4 = assign24510_e33708_d_n4;
        locals.var_t1_dn5 = assign24510_e33708_d_n5;
        locals.var_t1_dn6 = assign24510_e33708_d_n6;
        locals.var_t1_dn7 = assign24510_e33708_d_n7;
        locals.var_t1_dn8 = assign24510_e33708_d_n8;
        locals.var_t1_dn9 = assign24510_e33708_d_n9;
        locals.var_t1_dn10 = assign24510_e33708_d_n10;
        locals.var_t1_dn11 = assign24510_e33708_d_n11;
        locals.var_t1_dn12 = assign24510_e33708_d_n12;
        locals.var_t1_dn13 = assign24510_e33708_d_n13;
        locals.var_t1_dn14 = assign24510_e33708_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign24520_e33719, assign24520_e33719_d_n0, assign24520_e33719_d_n2, assign24520_e33719_d_n3, assign24520_e33719_d_n4, assign24520_e33719_d_n5, assign24520_e33719_d_n6, assign24520_e33719_d_n7, assign24520_e33719_d_n8, assign24520_e33719_d_n9, assign24520_e33719_d_n10, assign24520_e33719_d_n11, assign24520_e33719_d_n12, assign24520_e33719_d_n13, assign24520_e33719_d_n14,) = {
    if ((locals.var_guard609 != 0.0) && (locals.var_guard612 == 0.0)) {
        let assign24520_e33716: f64 = (locals.var_t1 + 0.001);
        let assign24520_e33717: f64 = (locals.var_bgisl_t / assign24520_e33716);
        (assign24520_e33717, (-((locals.var_bgisl_t * locals.var_t1_dn0) / (assign24520_e33716 * assign24520_e33716))), (-((locals.var_bgisl_t * locals.var_t1_dn2) / (assign24520_e33716 * assign24520_e33716))), (-((locals.var_bgisl_t * locals.var_t1_dn3) / (assign24520_e33716 * assign24520_e33716))), (((locals.var_bgisl_t_dn4 * assign24520_e33716) - (locals.var_bgisl_t * locals.var_t1_dn4)) / (assign24520_e33716 * assign24520_e33716)), (-((locals.var_bgisl_t * locals.var_t1_dn5) / (assign24520_e33716 * assign24520_e33716))), (-((locals.var_bgisl_t * locals.var_t1_dn6) / (assign24520_e33716 * assign24520_e33716))), (-((locals.var_bgisl_t * locals.var_t1_dn7) / (assign24520_e33716 * assign24520_e33716))), (-((locals.var_bgisl_t * locals.var_t1_dn8) / (assign24520_e33716 * assign24520_e33716))), (-((locals.var_bgisl_t * locals.var_t1_dn9) / (assign24520_e33716 * assign24520_e33716))), (-((locals.var_bgisl_t * locals.var_t1_dn10) / (assign24520_e33716 * assign24520_e33716))), (-((locals.var_bgisl_t * locals.var_t1_dn11) / (assign24520_e33716 * assign24520_e33716))), (-((locals.var_bgisl_t * locals.var_t1_dn12) / (assign24520_e33716 * assign24520_e33716))), (-((locals.var_bgisl_t * locals.var_t1_dn13) / (assign24520_e33716 * assign24520_e33716))), (-((locals.var_bgisl_t * locals.var_t1_dn14) / (assign24520_e33716 * assign24520_e33716))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign24520_e33719;
        locals.var_t2_dn0 = assign24520_e33719_d_n0;
        locals.var_t2_dn2 = assign24520_e33719_d_n2;
        locals.var_t2_dn3 = assign24520_e33719_d_n3;
        locals.var_t2_dn4 = assign24520_e33719_d_n4;
        locals.var_t2_dn5 = assign24520_e33719_d_n5;
        locals.var_t2_dn6 = assign24520_e33719_d_n6;
        locals.var_t2_dn7 = assign24520_e33719_d_n7;
        locals.var_t2_dn8 = assign24520_e33719_d_n8;
        locals.var_t2_dn9 = assign24520_e33719_d_n9;
        locals.var_t2_dn10 = assign24520_e33719_d_n10;
        locals.var_t2_dn11 = assign24520_e33719_d_n11;
        locals.var_t2_dn12 = assign24520_e33719_d_n12;
        locals.var_t2_dn13 = assign24520_e33719_d_n13;
        locals.var_t2_dn14 = assign24520_e33719_d_n14;
        locals.var_t2_rv = 0.0;

        let assign24530_e33722: f64 = if locals.var_cgisl_i != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard613 = assign24530_e33722;
        locals.var_guard613_rv = 0.0;

        let (assign24540_e33735, assign24540_e33735_d_n0, assign24540_e33735_d_n2, assign24540_e33735_d_n3, assign24540_e33735_d_n4, assign24540_e33735_d_n5, assign24540_e33735_d_n6, assign24540_e33735_d_n7, assign24540_e33735_d_n8, assign24540_e33735_d_n9, assign24540_e33735_d_n10, assign24540_e33735_d_n11, assign24540_e33735_d_n12, assign24540_e33735_d_n13, assign24540_e33735_d_n14,) = {
    if (((locals.var_guard609 != 0.0) && (locals.var_guard612 == 0.0)) && (locals.var_guard613 != 0.0)) {
        let assign24540_e33731: f64 = (locals.var_vsb_noswap * locals.var_vsb_noswap);
        let assign24540_e33733: f64 = (assign24540_e33731 * locals.var_vsb_noswap);
        (assign24540_e33733, 0.0, 0.0, 0.0, 0.0, ((((locals.var_vsb_noswap_dn5 * locals.var_vsb_noswap) + (locals.var_vsb_noswap * locals.var_vsb_noswap_dn5)) * locals.var_vsb_noswap) + (assign24540_e33731 * locals.var_vsb_noswap_dn5)), 0.0, ((((locals.var_vsb_noswap_dn7 * locals.var_vsb_noswap) + (locals.var_vsb_noswap * locals.var_vsb_noswap_dn7)) * locals.var_vsb_noswap) + (assign24540_e33731 * locals.var_vsb_noswap_dn7)), 0.0, 0.0, 0.0, ((((locals.var_vsb_noswap_dn11 * locals.var_vsb_noswap) + (locals.var_vsb_noswap * locals.var_vsb_noswap_dn11)) * locals.var_vsb_noswap) + (assign24540_e33731 * locals.var_vsb_noswap_dn11)), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign24540_e33735;
        locals.var_t3_dn0 = assign24540_e33735_d_n0;
        locals.var_t3_dn2 = assign24540_e33735_d_n2;
        locals.var_t3_dn3 = assign24540_e33735_d_n3;
        locals.var_t3_dn4 = assign24540_e33735_d_n4;
        locals.var_t3_dn5 = assign24540_e33735_d_n5;
        locals.var_t3_dn6 = assign24540_e33735_d_n6;
        locals.var_t3_dn7 = assign24540_e33735_d_n7;
        locals.var_t3_dn8 = assign24540_e33735_d_n8;
        locals.var_t3_dn9 = assign24540_e33735_d_n9;
        locals.var_t3_dn10 = assign24540_e33735_d_n10;
        locals.var_t3_dn11 = assign24540_e33735_d_n11;
        locals.var_t3_dn12 = assign24540_e33735_d_n12;
        locals.var_t3_dn13 = assign24540_e33735_d_n13;
        locals.var_t3_dn14 = assign24540_e33735_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign24550_e33749, assign24550_e33749_d_n0, assign24550_e33749_d_n2, assign24550_e33749_d_n3, assign24550_e33749_d_n4, assign24550_e33749_d_n5, assign24550_e33749_d_n6, assign24550_e33749_d_n7, assign24550_e33749_d_n8, assign24550_e33749_d_n9, assign24550_e33749_d_n10, assign24550_e33749_d_n11, assign24550_e33749_d_n12, assign24550_e33749_d_n13, assign24550_e33749_d_n14,) = {
    if (((locals.var_guard609 != 0.0) && (locals.var_guard612 == 0.0)) && (locals.var_guard613 != 0.0)) {
        let assign24550_e33744: f64 = (locals.var_t3).abs();
        let assign24550_e33745: f64 = (locals.var_cgisl_i + assign24550_e33744);
        let assign24550_e33747: f64 = (assign24550_e33745 + 0.0001);
        (assign24550_e33747, if locals.var_t3 >= 0.0 { locals.var_t3_dn0 } else { (-locals.var_t3_dn0) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn2 } else { (-locals.var_t3_dn2) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn3 } else { (-locals.var_t3_dn3) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn4 } else { (-locals.var_t3_dn4) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn5 } else { (-locals.var_t3_dn5) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn6 } else { (-locals.var_t3_dn6) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn7 } else { (-locals.var_t3_dn7) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn8 } else { (-locals.var_t3_dn8) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn9 } else { (-locals.var_t3_dn9) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn10 } else { (-locals.var_t3_dn10) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn11 } else { (-locals.var_t3_dn11) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn12 } else { (-locals.var_t3_dn12) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn13 } else { (-locals.var_t3_dn13) }, if locals.var_t3 >= 0.0 { locals.var_t3_dn14 } else { (-locals.var_t3_dn14) },)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign24550_e33749;
        locals.var_t4_dn0 = assign24550_e33749_d_n0;
        locals.var_t4_dn2 = assign24550_e33749_d_n2;
        locals.var_t4_dn3 = assign24550_e33749_d_n3;
        locals.var_t4_dn4 = assign24550_e33749_d_n4;
        locals.var_t4_dn5 = assign24550_e33749_d_n5;
        locals.var_t4_dn6 = assign24550_e33749_d_n6;
        locals.var_t4_dn7 = assign24550_e33749_d_n7;
        locals.var_t4_dn8 = assign24550_e33749_d_n8;
        locals.var_t4_dn9 = assign24550_e33749_d_n9;
        locals.var_t4_dn10 = assign24550_e33749_d_n10;
        locals.var_t4_dn11 = assign24550_e33749_d_n11;
        locals.var_t4_dn12 = assign24550_e33749_d_n12;
        locals.var_t4_dn13 = assign24550_e33749_d_n13;
        locals.var_t4_dn14 = assign24550_e33749_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign24560_e33807, assign24560_e33807_d_n0, assign24560_e33807_d_n2, assign24560_e33807_d_n3, assign24560_e33807_d_n4, assign24560_e33807_d_n5, assign24560_e33807_d_n6, assign24560_e33807_d_n7, assign24560_e33807_d_n8, assign24560_e33807_d_n9, assign24560_e33807_d_n10, assign24560_e33807_d_n11, assign24560_e33807_d_n12, assign24560_e33807_d_n13, assign24560_e33807_d_n14,) = {
    if (((locals.var_guard609 != 0.0) && (locals.var_guard612 == 0.0)) && (locals.var_guard613 != 0.0)) {
        let assign24560_e33758: f64 = (locals.var_t3 / locals.var_t4);
        let assign24560_e33760: f64 = (-10000.0);
        let assign24560_e33762: f64 = (assign24560_e33760 * 1e-6);
        let (assign24560_e33803, assign24560_e33803_d_n0, assign24560_e33803_d_n2, assign24560_e33803_d_n3, assign24560_e33803_d_n4, assign24560_e33803_d_n5, assign24560_e33803_d_n6, assign24560_e33803_d_n7, assign24560_e33803_d_n8, assign24560_e33803_d_n9, assign24560_e33803_d_n10, assign24560_e33803_d_n11, assign24560_e33803_d_n12, assign24560_e33803_d_n13, assign24560_e33803_d_n14,) = {
            if (!(assign24560_e33758 < assign24560_e33762)) {
                let __rspice_inv_cse_1: f64 = 1.0 / locals.var_t4;
                let assign24560_e33768: f64 = (locals.var_t3 * __rspice_inv_cse_1);
                let assign24560_e33771: f64 = (locals.var_t3 * __rspice_inv_cse_1);
                let assign24560_e33774: f64 = (locals.var_t3 * __rspice_inv_cse_1);
                let assign24560_e33775: f64 = (assign24560_e33771 * assign24560_e33774);
                let assign24560_e33778: f64 = (4.0 * 1e-6);
                let assign24560_e33780: f64 = (assign24560_e33778 * 1e-6);
                let assign24560_e33781: f64 = (assign24560_e33775 + assign24560_e33780);
                let assign24560_e33782: f64 = (assign24560_e33781).sqrt();
                let assign24560_e33783: f64 = (assign24560_e33768 + assign24560_e33782);
                let assign24560_e33784: f64 = (0.5 * assign24560_e33783);
                (assign24560_e33784, (0.5 * ((((locals.var_t3_dn0 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn0 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)) * assign24560_e33774) + (assign24560_e33771 * (((locals.var_t3_dn0 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24560_e33782)))), (0.5 * ((((locals.var_t3_dn2 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn2 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)) * assign24560_e33774) + (assign24560_e33771 * (((locals.var_t3_dn2 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24560_e33782)))), (0.5 * ((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)) * assign24560_e33774) + (assign24560_e33771 * (((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24560_e33782)))), (0.5 * ((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)) * assign24560_e33774) + (assign24560_e33771 * (((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24560_e33782)))), (0.5 * ((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)) * assign24560_e33774) + (assign24560_e33771 * (((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24560_e33782)))), (0.5 * ((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)) * assign24560_e33774) + (assign24560_e33771 * (((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24560_e33782)))), (0.5 * ((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)) * assign24560_e33774) + (assign24560_e33771 * (((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24560_e33782)))), (0.5 * ((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)) * assign24560_e33774) + (assign24560_e33771 * (((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24560_e33782)))), (0.5 * ((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)) * assign24560_e33774) + (assign24560_e33771 * (((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24560_e33782)))), (0.5 * ((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)) * assign24560_e33774) + (assign24560_e33771 * (((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24560_e33782)))), (0.5 * ((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)) * assign24560_e33774) + (assign24560_e33771 * (((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24560_e33782)))), (0.5 * ((((locals.var_t3_dn12 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn12)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn12 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn12)) / (locals.var_t4 * locals.var_t4)) * assign24560_e33774) + (assign24560_e33771 * (((locals.var_t3_dn12 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn12)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24560_e33782)))), (0.5 * ((((locals.var_t3_dn13 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn13 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4)) * assign24560_e33774) + (assign24560_e33771 * (((locals.var_t3_dn13 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24560_e33782)))), (0.5 * ((((locals.var_t3_dn14 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)) + ((((((locals.var_t3_dn14 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)) * assign24560_e33774) + (assign24560_e33771 * (((locals.var_t3_dn14 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4)))) / (2.0 * assign24560_e33782)))),)
            } else {
                let assign24560_e33787: f64 = (locals.var_t3 / locals.var_t4);
                let assign24560_e33789: f64 = (-10000.0);
                let assign24560_e33791: f64 = (assign24560_e33789 * 1e-6);
                let (assign24560_e33802, assign24560_e33802_d_n0, assign24560_e33802_d_n2, assign24560_e33802_d_n3, assign24560_e33802_d_n4, assign24560_e33802_d_n5, assign24560_e33802_d_n6, assign24560_e33802_d_n7, assign24560_e33802_d_n8, assign24560_e33802_d_n9, assign24560_e33802_d_n10, assign24560_e33802_d_n11, assign24560_e33802_d_n12, assign24560_e33802_d_n13, assign24560_e33802_d_n14,) = {
                    if (assign24560_e33787 < assign24560_e33791) {
                        let assign24560_e33794: f64 = (-1e-6);
                        let assign24560_e33796: f64 = (assign24560_e33794 * 1e-6);
                        let assign24560_e33799: f64 = (locals.var_t3 / locals.var_t4);
                        let assign24560_e33800: f64 = (assign24560_e33796 / assign24560_e33799);
                        (assign24560_e33800, (-((assign24560_e33796 * (((locals.var_t3_dn0 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn0)) / (locals.var_t4 * locals.var_t4))) / (assign24560_e33799 * assign24560_e33799))), (-((assign24560_e33796 * (((locals.var_t3_dn2 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn2)) / (locals.var_t4 * locals.var_t4))) / (assign24560_e33799 * assign24560_e33799))), (-((assign24560_e33796 * (((locals.var_t3_dn3 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn3)) / (locals.var_t4 * locals.var_t4))) / (assign24560_e33799 * assign24560_e33799))), (-((assign24560_e33796 * (((locals.var_t3_dn4 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn4)) / (locals.var_t4 * locals.var_t4))) / (assign24560_e33799 * assign24560_e33799))), (-((assign24560_e33796 * (((locals.var_t3_dn5 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn5)) / (locals.var_t4 * locals.var_t4))) / (assign24560_e33799 * assign24560_e33799))), (-((assign24560_e33796 * (((locals.var_t3_dn6 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn6)) / (locals.var_t4 * locals.var_t4))) / (assign24560_e33799 * assign24560_e33799))), (-((assign24560_e33796 * (((locals.var_t3_dn7 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn7)) / (locals.var_t4 * locals.var_t4))) / (assign24560_e33799 * assign24560_e33799))), (-((assign24560_e33796 * (((locals.var_t3_dn8 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn8)) / (locals.var_t4 * locals.var_t4))) / (assign24560_e33799 * assign24560_e33799))), (-((assign24560_e33796 * (((locals.var_t3_dn9 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn9)) / (locals.var_t4 * locals.var_t4))) / (assign24560_e33799 * assign24560_e33799))), (-((assign24560_e33796 * (((locals.var_t3_dn10 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn10)) / (locals.var_t4 * locals.var_t4))) / (assign24560_e33799 * assign24560_e33799))), (-((assign24560_e33796 * (((locals.var_t3_dn11 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn11)) / (locals.var_t4 * locals.var_t4))) / (assign24560_e33799 * assign24560_e33799))), (-((assign24560_e33796 * (((locals.var_t3_dn12 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn12)) / (locals.var_t4 * locals.var_t4))) / (assign24560_e33799 * assign24560_e33799))), (-((assign24560_e33796 * (((locals.var_t3_dn13 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn13)) / (locals.var_t4 * locals.var_t4))) / (assign24560_e33799 * assign24560_e33799))), (-((assign24560_e33796 * (((locals.var_t3_dn14 * locals.var_t4) - (locals.var_t3 * locals.var_t4_dn14)) / (locals.var_t4 * locals.var_t4))) / (assign24560_e33799 * assign24560_e33799))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign24560_e33802, assign24560_e33802_d_n0, assign24560_e33802_d_n2, assign24560_e33802_d_n3, assign24560_e33802_d_n4, assign24560_e33802_d_n5, assign24560_e33802_d_n6, assign24560_e33802_d_n7, assign24560_e33802_d_n8, assign24560_e33802_d_n9, assign24560_e33802_d_n10, assign24560_e33802_d_n11, assign24560_e33802_d_n12, assign24560_e33802_d_n13, assign24560_e33802_d_n14,)
            }
        };
        let assign24560_e33805: f64 = (assign24560_e33803 - 1e-6);
        (assign24560_e33805, assign24560_e33803_d_n0, assign24560_e33803_d_n2, assign24560_e33803_d_n3, assign24560_e33803_d_n4, assign24560_e33803_d_n5, assign24560_e33803_d_n6, assign24560_e33803_d_n7, assign24560_e33803_d_n8, assign24560_e33803_d_n9, assign24560_e33803_d_n10, assign24560_e33803_d_n11, assign24560_e33803_d_n12, assign24560_e33803_d_n13, assign24560_e33803_d_n14,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign24560_e33807;
        locals.var_t5_dn0 = assign24560_e33807_d_n0;
        locals.var_t5_dn2 = assign24560_e33807_d_n2;
        locals.var_t5_dn3 = assign24560_e33807_d_n3;
        locals.var_t5_dn4 = assign24560_e33807_d_n4;
        locals.var_t5_dn5 = assign24560_e33807_d_n5;
        locals.var_t5_dn6 = assign24560_e33807_d_n6;
        locals.var_t5_dn7 = assign24560_e33807_d_n7;
        locals.var_t5_dn8 = assign24560_e33807_d_n8;
        locals.var_t5_dn9 = assign24560_e33807_d_n9;
        locals.var_t5_dn10 = assign24560_e33807_d_n10;
        locals.var_t5_dn11 = assign24560_e33807_d_n11;
        locals.var_t5_dn12 = assign24560_e33807_d_n12;
        locals.var_t5_dn13 = assign24560_e33807_d_n13;
        locals.var_t5_dn14 = assign24560_e33807_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign24570_e33817, assign24570_e33817_d_n0, assign24570_e33817_d_n2, assign24570_e33817_d_n3, assign24570_e33817_d_n4, assign24570_e33817_d_n5, assign24570_e33817_d_n6, assign24570_e33817_d_n7, assign24570_e33817_d_n8, assign24570_e33817_d_n9, assign24570_e33817_d_n10, assign24570_e33817_d_n11, assign24570_e33817_d_n12, assign24570_e33817_d_n13, assign24570_e33817_d_n14,) = {
    if (((locals.var_guard609 != 0.0) && (locals.var_guard612 == 0.0)) && (locals.var_guard613 == 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign24570_e33817;
        locals.var_t5_dn0 = assign24570_e33817_d_n0;
        locals.var_t5_dn2 = assign24570_e33817_d_n2;
        locals.var_t5_dn3 = assign24570_e33817_d_n3;
        locals.var_t5_dn4 = assign24570_e33817_d_n4;
        locals.var_t5_dn5 = assign24570_e33817_d_n5;
        locals.var_t5_dn6 = assign24570_e33817_d_n6;
        locals.var_t5_dn7 = assign24570_e33817_d_n7;
        locals.var_t5_dn8 = assign24570_e33817_d_n8;
        locals.var_t5_dn9 = assign24570_e33817_d_n9;
        locals.var_t5_dn10 = assign24570_e33817_d_n10;
        locals.var_t5_dn11 = assign24570_e33817_d_n11;
        locals.var_t5_dn12 = assign24570_e33817_d_n12;
        locals.var_t5_dn13 = assign24570_e33817_d_n13;
        locals.var_t5_dn14 = assign24570_e33817_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign24580_e33834, assign24580_e33834_d_n0, assign24580_e33834_d_n2, assign24580_e33834_d_n3, assign24580_e33834_d_n4, assign24580_e33834_d_n5, assign24580_e33834_d_n6, assign24580_e33834_d_n7, assign24580_e33834_d_n8, assign24580_e33834_d_n9, assign24580_e33834_d_n10, assign24580_e33834_d_n11, assign24580_e33834_d_n12, assign24580_e33834_d_n13, assign24580_e33834_d_n14,) = {
    if ((locals.var_guard609 != 0.0) && (locals.var_guard612 == 0.0)) {
        let assign24580_e33824: f64 = (locals.var_agisl_i * locals.var_weff);
        let assign24580_e33826: f64 = (assign24580_e33824 * locals.var_t1);
        let assign24580_e33828: f64 = (-locals.var_t2);
        let assign24580_e33829: f64 = { let limited_exp_arg = assign24580_e33828; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign24580_e33830: f64 = (assign24580_e33826 * assign24580_e33829);
        let assign24580_e33832: f64 = (assign24580_e33830 * locals.var_t5);
        (assign24580_e33832, (((((assign24580_e33824 * locals.var_t1_dn0) * assign24580_e33829) + (assign24580_e33826 * ({ let limited_exp_arg = assign24580_e33828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn0)))) * locals.var_t5) + (assign24580_e33830 * locals.var_t5_dn0)), (((((assign24580_e33824 * locals.var_t1_dn2) * assign24580_e33829) + (assign24580_e33826 * ({ let limited_exp_arg = assign24580_e33828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn2)))) * locals.var_t5) + (assign24580_e33830 * locals.var_t5_dn2)), (((((assign24580_e33824 * locals.var_t1_dn3) * assign24580_e33829) + (assign24580_e33826 * ({ let limited_exp_arg = assign24580_e33828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn3)))) * locals.var_t5) + (assign24580_e33830 * locals.var_t5_dn3)), (((((assign24580_e33824 * locals.var_t1_dn4) * assign24580_e33829) + (assign24580_e33826 * ({ let limited_exp_arg = assign24580_e33828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn4)))) * locals.var_t5) + (assign24580_e33830 * locals.var_t5_dn4)), (((((assign24580_e33824 * locals.var_t1_dn5) * assign24580_e33829) + (assign24580_e33826 * ({ let limited_exp_arg = assign24580_e33828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn5)))) * locals.var_t5) + (assign24580_e33830 * locals.var_t5_dn5)), (((((assign24580_e33824 * locals.var_t1_dn6) * assign24580_e33829) + (assign24580_e33826 * ({ let limited_exp_arg = assign24580_e33828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn6)))) * locals.var_t5) + (assign24580_e33830 * locals.var_t5_dn6)), (((((assign24580_e33824 * locals.var_t1_dn7) * assign24580_e33829) + (assign24580_e33826 * ({ let limited_exp_arg = assign24580_e33828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn7)))) * locals.var_t5) + (assign24580_e33830 * locals.var_t5_dn7)), (((((assign24580_e33824 * locals.var_t1_dn8) * assign24580_e33829) + (assign24580_e33826 * ({ let limited_exp_arg = assign24580_e33828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn8)))) * locals.var_t5) + (assign24580_e33830 * locals.var_t5_dn8)), (((((assign24580_e33824 * locals.var_t1_dn9) * assign24580_e33829) + (assign24580_e33826 * ({ let limited_exp_arg = assign24580_e33828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn9)))) * locals.var_t5) + (assign24580_e33830 * locals.var_t5_dn9)), (((((assign24580_e33824 * locals.var_t1_dn10) * assign24580_e33829) + (assign24580_e33826 * ({ let limited_exp_arg = assign24580_e33828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn10)))) * locals.var_t5) + (assign24580_e33830 * locals.var_t5_dn10)), (((((assign24580_e33824 * locals.var_t1_dn11) * assign24580_e33829) + (assign24580_e33826 * ({ let limited_exp_arg = assign24580_e33828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn11)))) * locals.var_t5) + (assign24580_e33830 * locals.var_t5_dn11)), (((((assign24580_e33824 * locals.var_t1_dn12) * assign24580_e33829) + (assign24580_e33826 * ({ let limited_exp_arg = assign24580_e33828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn12)))) * locals.var_t5) + (assign24580_e33830 * locals.var_t5_dn12)), (((((assign24580_e33824 * locals.var_t1_dn13) * assign24580_e33829) + (assign24580_e33826 * ({ let limited_exp_arg = assign24580_e33828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn13)))) * locals.var_t5) + (assign24580_e33830 * locals.var_t5_dn13)), (((((assign24580_e33824 * locals.var_t1_dn14) * assign24580_e33829) + (assign24580_e33826 * ({ let limited_exp_arg = assign24580_e33828; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t2_dn14)))) * locals.var_t5) + (assign24580_e33830 * locals.var_t5_dn14)),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign24580_e33834;
        locals.var_t6_dn0 = assign24580_e33834_d_n0;
        locals.var_t6_dn2 = assign24580_e33834_d_n2;
        locals.var_t6_dn3 = assign24580_e33834_d_n3;
        locals.var_t6_dn4 = assign24580_e33834_d_n4;
        locals.var_t6_dn5 = assign24580_e33834_d_n5;
        locals.var_t6_dn6 = assign24580_e33834_d_n6;
        locals.var_t6_dn7 = assign24580_e33834_d_n7;
        locals.var_t6_dn8 = assign24580_e33834_d_n8;
        locals.var_t6_dn9 = assign24580_e33834_d_n9;
        locals.var_t6_dn10 = assign24580_e33834_d_n10;
        locals.var_t6_dn11 = assign24580_e33834_d_n11;
        locals.var_t6_dn12 = assign24580_e33834_d_n12;
        locals.var_t6_dn13 = assign24580_e33834_d_n13;
        locals.var_t6_dn14 = assign24580_e33834_d_n14;
        locals.var_t6_rv = 0.0;

        let assign24620_e33855: f64 = (locals.var_vbs_jct / locals.var_nvtms);
        locals.var_t0 = assign24620_e33855;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = (-((locals.var_vbs_jct * locals.var_nvtms_dn4) / (locals.var_nvtms * locals.var_nvtms)));
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = (locals.var_vbs_jct_dn7 / locals.var_nvtms);
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = (locals.var_vbs_jct_dn12 / locals.var_nvtms);
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign24630_e33857: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign24630_e33859: f64 = (assign24630_e33857 - 1.0);
        locals.var_t1 = assign24630_e33859;
        locals.var_t1_dn0 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0);
        locals.var_t1_dn2 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2);
        locals.var_t1_dn3 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3);
        locals.var_t1_dn4 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4);
        locals.var_t1_dn5 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5);
        locals.var_t1_dn6 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6);
        locals.var_t1_dn7 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7);
        locals.var_t1_dn8 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8);
        locals.var_t1_dn9 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9);
        locals.var_t1_dn10 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10);
        locals.var_t1_dn11 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11);
        locals.var_t1_dn12 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn12);
        locals.var_t1_dn13 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13);
        locals.var_t1_dn14 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14);
        locals.var_t1_rv = 0.0;

        let assign24640_e33864: f64 = (locals.var_vbs_jct - locals.var_vjsmrev);
        let assign24640_e33865: f64 = (locals.var_sslprev * assign24640_e33864);
        let assign24640_e33866: f64 = (locals.var_ivjsmrev + assign24640_e33865);
        locals.var_t2 = assign24640_e33866;
        locals.var_t2_dn0 = (locals.var_ivjsmrev_dn0 + ((locals.var_sslprev_dn0 * assign24640_e33864) + (locals.var_sslprev * (-locals.var_vjsmrev_dn0))));
        locals.var_t2_dn2 = (locals.var_ivjsmrev_dn2 + ((locals.var_sslprev_dn2 * assign24640_e33864) + (locals.var_sslprev * (-locals.var_vjsmrev_dn2))));
        locals.var_t2_dn3 = (locals.var_ivjsmrev_dn3 + ((locals.var_sslprev_dn3 * assign24640_e33864) + (locals.var_sslprev * (-locals.var_vjsmrev_dn3))));
        locals.var_t2_dn4 = (locals.var_ivjsmrev_dn4 + ((locals.var_sslprev_dn4 * assign24640_e33864) + (locals.var_sslprev * (-locals.var_vjsmrev_dn4))));
        locals.var_t2_dn5 = (locals.var_ivjsmrev_dn5 + ((locals.var_sslprev_dn5 * assign24640_e33864) + (locals.var_sslprev * (-locals.var_vjsmrev_dn5))));
        locals.var_t2_dn6 = (locals.var_ivjsmrev_dn6 + ((locals.var_sslprev_dn6 * assign24640_e33864) + (locals.var_sslprev * (-locals.var_vjsmrev_dn6))));
        locals.var_t2_dn7 = (locals.var_ivjsmrev_dn7 + ((locals.var_sslprev_dn7 * assign24640_e33864) + (locals.var_sslprev * (locals.var_vbs_jct_dn7 - locals.var_vjsmrev_dn7))));
        locals.var_t2_dn8 = (locals.var_ivjsmrev_dn8 + ((locals.var_sslprev_dn8 * assign24640_e33864) + (locals.var_sslprev * (-locals.var_vjsmrev_dn8))));
        locals.var_t2_dn9 = (locals.var_ivjsmrev_dn9 + ((locals.var_sslprev_dn9 * assign24640_e33864) + (locals.var_sslprev * (-locals.var_vjsmrev_dn9))));
        locals.var_t2_dn10 = (locals.var_ivjsmrev_dn10 + ((locals.var_sslprev_dn10 * assign24640_e33864) + (locals.var_sslprev * (-locals.var_vjsmrev_dn10))));
        locals.var_t2_dn11 = (locals.var_ivjsmrev_dn11 + ((locals.var_sslprev_dn11 * assign24640_e33864) + (locals.var_sslprev * (-locals.var_vjsmrev_dn11))));
        locals.var_t2_dn12 = (locals.var_ivjsmrev_dn12 + ((locals.var_sslprev_dn12 * assign24640_e33864) + (locals.var_sslprev * (locals.var_vbs_jct_dn12 - locals.var_vjsmrev_dn12))));
        locals.var_t2_dn13 = (locals.var_ivjsmrev_dn13 + ((locals.var_sslprev_dn13 * assign24640_e33864) + (locals.var_sslprev * (-locals.var_vjsmrev_dn13))));
        locals.var_t2_dn14 = (locals.var_ivjsmrev_dn14 + ((locals.var_sslprev_dn14 * assign24640_e33864) + (locals.var_sslprev * (-locals.var_vjsmrev_dn14))));
        locals.var_t2_rv = 0.0;

        let assign24650_e33869: f64 = (locals.var_t1 * locals.var_t2);
        locals.var_t3 = assign24650_e33869;
        locals.var_t3_dn0 = ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0));
        locals.var_t3_dn2 = ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2));
        locals.var_t3_dn3 = ((locals.var_t1_dn3 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn3));
        locals.var_t3_dn4 = ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4));
        locals.var_t3_dn5 = ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5));
        locals.var_t3_dn6 = ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6));
        locals.var_t3_dn7 = ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7));
        locals.var_t3_dn8 = ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8));
        locals.var_t3_dn9 = ((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9));
        locals.var_t3_dn10 = ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10));
        locals.var_t3_dn11 = ((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11));
        locals.var_t3_dn12 = ((locals.var_t1_dn12 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn12));
        locals.var_t3_dn13 = ((locals.var_t1_dn13 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn13));
        locals.var_t3_dn14 = ((locals.var_t1_dn14 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn14));
        locals.var_t3_rv = 0.0;

        let assign24660_e33872: f64 = (p.p731 + locals.var_vbs_jct);
        let assign24660_e33874: f64 = (assign24660_e33872 / locals.var_nvtms);
        locals.var_t1 = assign24660_e33874;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = (-((assign24660_e33872 * locals.var_nvtms_dn4) / (locals.var_nvtms * locals.var_nvtms)));
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = (locals.var_vbs_jct_dn7 / locals.var_nvtms);
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = (locals.var_vbs_jct_dn12 / locals.var_nvtms);
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_73(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign24670_e33876: f64 = (-locals.var_t1);
        let assign24670_e33877: f64 = { let limited_exp_arg = assign24670_e33876; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        locals.var_t2 = assign24670_e33877;
        locals.var_t2_dn0 = ({ let limited_exp_arg = assign24670_e33876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn0));
        locals.var_t2_dn2 = ({ let limited_exp_arg = assign24670_e33876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn2));
        locals.var_t2_dn3 = ({ let limited_exp_arg = assign24670_e33876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn3));
        locals.var_t2_dn4 = ({ let limited_exp_arg = assign24670_e33876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn4));
        locals.var_t2_dn5 = ({ let limited_exp_arg = assign24670_e33876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn5));
        locals.var_t2_dn6 = ({ let limited_exp_arg = assign24670_e33876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn6));
        locals.var_t2_dn7 = ({ let limited_exp_arg = assign24670_e33876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn7));
        locals.var_t2_dn8 = ({ let limited_exp_arg = assign24670_e33876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn8));
        locals.var_t2_dn9 = ({ let limited_exp_arg = assign24670_e33876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn9));
        locals.var_t2_dn10 = ({ let limited_exp_arg = assign24670_e33876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn10));
        locals.var_t2_dn11 = ({ let limited_exp_arg = assign24670_e33876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn11));
        locals.var_t2_dn12 = ({ let limited_exp_arg = assign24670_e33876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn12));
        locals.var_t2_dn13 = ({ let limited_exp_arg = assign24670_e33876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn13));
        locals.var_t2_dn14 = ({ let limited_exp_arg = assign24670_e33876; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn14));
        locals.var_t2_rv = 0.0;

        let assign24680_e33880: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign24680_e33882: f64 = (assign24680_e33880 + locals.var_xexpbvs);
        let assign24680_e33884: f64 = (assign24680_e33882 - 1.0);
        let assign24680_e33887: f64 = (p.p733 * locals.var_t2);
        let assign24680_e33888: f64 = (assign24680_e33884 - assign24680_e33887);
        let assign24680_e33889: f64 = (locals.var_isbs * assign24680_e33888);
        locals.var_t4 = assign24680_e33889;
        locals.var_t4_dn0 = ((locals.var_isbs_dn0 * assign24680_e33888) + (locals.var_isbs * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) - (p.p733 * locals.var_t2_dn0))));
        locals.var_t4_dn2 = ((locals.var_isbs_dn2 * assign24680_e33888) + (locals.var_isbs * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) - (p.p733 * locals.var_t2_dn2))));
        locals.var_t4_dn3 = ((locals.var_isbs_dn3 * assign24680_e33888) + (locals.var_isbs * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3) - (p.p733 * locals.var_t2_dn3))));
        locals.var_t4_dn4 = ((locals.var_isbs_dn4 * assign24680_e33888) + (locals.var_isbs * ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + locals.var_xexpbvs_dn4) - (p.p733 * locals.var_t2_dn4))));
        locals.var_t4_dn5 = ((locals.var_isbs_dn5 * assign24680_e33888) + (locals.var_isbs * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) - (p.p733 * locals.var_t2_dn5))));
        locals.var_t4_dn6 = ((locals.var_isbs_dn6 * assign24680_e33888) + (locals.var_isbs * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) - (p.p733 * locals.var_t2_dn6))));
        locals.var_t4_dn7 = ((locals.var_isbs_dn7 * assign24680_e33888) + (locals.var_isbs * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) - (p.p733 * locals.var_t2_dn7))));
        locals.var_t4_dn8 = ((locals.var_isbs_dn8 * assign24680_e33888) + (locals.var_isbs * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) - (p.p733 * locals.var_t2_dn8))));
        locals.var_t4_dn9 = ((locals.var_isbs_dn9 * assign24680_e33888) + (locals.var_isbs * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) - (p.p733 * locals.var_t2_dn9))));
        locals.var_t4_dn10 = ((locals.var_isbs_dn10 * assign24680_e33888) + (locals.var_isbs * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) - (p.p733 * locals.var_t2_dn10))));
        locals.var_t4_dn11 = ((locals.var_isbs_dn11 * assign24680_e33888) + (locals.var_isbs * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) - (p.p733 * locals.var_t2_dn11))));
        locals.var_t4_dn12 = ((locals.var_isbs_dn12 * assign24680_e33888) + (locals.var_isbs * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn12) - (p.p733 * locals.var_t2_dn12))));
        locals.var_t4_dn13 = ((locals.var_isbs_dn13 * assign24680_e33888) + (locals.var_isbs * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) - (p.p733 * locals.var_t2_dn13))));
        locals.var_t4_dn14 = ((locals.var_isbs_dn14 * assign24680_e33888) + (locals.var_isbs * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) - (p.p733 * locals.var_t2_dn14))));
        locals.var_t4_rv = 0.0;

        let assign24690_e33894: f64 = (locals.var_vbs_jct - locals.var_vjsmfwd);
        let assign24690_e33895: f64 = (locals.var_sslpfwd * assign24690_e33894);
        let assign24690_e33896: f64 = (locals.var_ivjsmfwd + assign24690_e33895);
        locals.var_t5 = assign24690_e33896;
        locals.var_t5_dn0 = (locals.var_ivjsmfwd_dn0 + ((locals.var_sslpfwd_dn0 * assign24690_e33894) + (locals.var_sslpfwd * (-locals.var_vjsmfwd_dn0))));
        locals.var_t5_dn2 = (locals.var_ivjsmfwd_dn2 + ((locals.var_sslpfwd_dn2 * assign24690_e33894) + (locals.var_sslpfwd * (-locals.var_vjsmfwd_dn2))));
        locals.var_t5_dn3 = (locals.var_ivjsmfwd_dn3 + ((locals.var_sslpfwd_dn3 * assign24690_e33894) + (locals.var_sslpfwd * (-locals.var_vjsmfwd_dn3))));
        locals.var_t5_dn4 = (locals.var_ivjsmfwd_dn4 + ((locals.var_sslpfwd_dn4 * assign24690_e33894) + (locals.var_sslpfwd * (-locals.var_vjsmfwd_dn4))));
        locals.var_t5_dn5 = (locals.var_ivjsmfwd_dn5 + ((locals.var_sslpfwd_dn5 * assign24690_e33894) + (locals.var_sslpfwd * (-locals.var_vjsmfwd_dn5))));
        locals.var_t5_dn6 = (locals.var_ivjsmfwd_dn6 + ((locals.var_sslpfwd_dn6 * assign24690_e33894) + (locals.var_sslpfwd * (-locals.var_vjsmfwd_dn6))));
        locals.var_t5_dn7 = (locals.var_ivjsmfwd_dn7 + ((locals.var_sslpfwd_dn7 * assign24690_e33894) + (locals.var_sslpfwd * (locals.var_vbs_jct_dn7 - locals.var_vjsmfwd_dn7))));
        locals.var_t5_dn8 = (locals.var_ivjsmfwd_dn8 + ((locals.var_sslpfwd_dn8 * assign24690_e33894) + (locals.var_sslpfwd * (-locals.var_vjsmfwd_dn8))));
        locals.var_t5_dn9 = (locals.var_ivjsmfwd_dn9 + ((locals.var_sslpfwd_dn9 * assign24690_e33894) + (locals.var_sslpfwd * (-locals.var_vjsmfwd_dn9))));
        locals.var_t5_dn10 = (locals.var_ivjsmfwd_dn10 + ((locals.var_sslpfwd_dn10 * assign24690_e33894) + (locals.var_sslpfwd * (-locals.var_vjsmfwd_dn10))));
        locals.var_t5_dn11 = (locals.var_ivjsmfwd_dn11 + ((locals.var_sslpfwd_dn11 * assign24690_e33894) + (locals.var_sslpfwd * (-locals.var_vjsmfwd_dn11))));
        locals.var_t5_dn12 = (locals.var_ivjsmfwd_dn12 + ((locals.var_sslpfwd_dn12 * assign24690_e33894) + (locals.var_sslpfwd * (locals.var_vbs_jct_dn12 - locals.var_vjsmfwd_dn12))));
        locals.var_t5_dn13 = (locals.var_ivjsmfwd_dn13 + ((locals.var_sslpfwd_dn13 * assign24690_e33894) + (locals.var_sslpfwd * (-locals.var_vjsmfwd_dn13))));
        locals.var_t5_dn14 = (locals.var_ivjsmfwd_dn14 + ((locals.var_sslpfwd_dn14 * assign24690_e33894) + (locals.var_sslpfwd * (-locals.var_vjsmfwd_dn14))));
        locals.var_t5_rv = 0.0;

        let assign24700_e33899: f64 = if locals.var_isbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard614 = assign24700_e33899;
        locals.var_guard614_rv = 0.0;

        let (assign24710_e33927, assign24710_e33927_d_n0, assign24710_e33927_d_n2, assign24710_e33927_d_n3, assign24710_e33927_d_n4, assign24710_e33927_d_n5, assign24710_e33927_d_n6, assign24710_e33927_d_n7, assign24710_e33927_d_n8, assign24710_e33927_d_n9, assign24710_e33927_d_n10, assign24710_e33927_d_n11, assign24710_e33927_d_n12, assign24710_e33927_d_n13, assign24710_e33927_d_n14,) = {
    if (locals.var_guard614 != 0.0) {
        let assign24710_e33903: f64 = (locals.var_t3 / 2.0);
        let assign24710_e33907: f64 = (locals.var_vbs_jct - locals.var_vjsmrev);
        let assign24710_e33909: f64 = (assign24710_e33907 / locals.var_nvtms);
        let assign24710_e33910: f64 = (assign24710_e33909).tanh();
        let assign24710_e33911: f64 = (1.0 - assign24710_e33910);
        let assign24710_e33912: f64 = (assign24710_e33903 * assign24710_e33911);
        let assign24710_e33915: f64 = (locals.var_t4 / 2.0);
        let assign24710_e33919: f64 = (locals.var_vbs_jct - locals.var_vjsmrev);
        let assign24710_e33921: f64 = (assign24710_e33919 / locals.var_nvtms);
        let assign24710_e33922: f64 = (assign24710_e33921).tanh();
        let assign24710_e33923: f64 = (1.0 + assign24710_e33922);
        let assign24710_e33924: f64 = (assign24710_e33915 * assign24710_e33923);
        let assign24710_e33925: f64 = (assign24710_e33912 + assign24710_e33924);
        (assign24710_e33925, ((((locals.var_t3_dn0 / 2.0) * assign24710_e33911) + (assign24710_e33903 * (-(((-locals.var_vjsmrev_dn0) / locals.var_nvtms) / ((assign24710_e33909).cosh() * (assign24710_e33909).cosh()))))) + (((locals.var_t4_dn0 / 2.0) * assign24710_e33923) + (assign24710_e33915 * (((-locals.var_vjsmrev_dn0) / locals.var_nvtms) / ((assign24710_e33921).cosh() * (assign24710_e33921).cosh()))))), ((((locals.var_t3_dn2 / 2.0) * assign24710_e33911) + (assign24710_e33903 * (-(((-locals.var_vjsmrev_dn2) / locals.var_nvtms) / ((assign24710_e33909).cosh() * (assign24710_e33909).cosh()))))) + (((locals.var_t4_dn2 / 2.0) * assign24710_e33923) + (assign24710_e33915 * (((-locals.var_vjsmrev_dn2) / locals.var_nvtms) / ((assign24710_e33921).cosh() * (assign24710_e33921).cosh()))))), ((((locals.var_t3_dn3 / 2.0) * assign24710_e33911) + (assign24710_e33903 * (-(((-locals.var_vjsmrev_dn3) / locals.var_nvtms) / ((assign24710_e33909).cosh() * (assign24710_e33909).cosh()))))) + (((locals.var_t4_dn3 / 2.0) * assign24710_e33923) + (assign24710_e33915 * (((-locals.var_vjsmrev_dn3) / locals.var_nvtms) / ((assign24710_e33921).cosh() * (assign24710_e33921).cosh()))))), ((((locals.var_t3_dn4 / 2.0) * assign24710_e33911) + (assign24710_e33903 * (-(((((-locals.var_vjsmrev_dn4) * locals.var_nvtms) - (assign24710_e33907 * locals.var_nvtms_dn4)) / (locals.var_nvtms * locals.var_nvtms)) / ((assign24710_e33909).cosh() * (assign24710_e33909).cosh()))))) + (((locals.var_t4_dn4 / 2.0) * assign24710_e33923) + (assign24710_e33915 * (((((-locals.var_vjsmrev_dn4) * locals.var_nvtms) - (assign24710_e33919 * locals.var_nvtms_dn4)) / (locals.var_nvtms * locals.var_nvtms)) / ((assign24710_e33921).cosh() * (assign24710_e33921).cosh()))))), ((((locals.var_t3_dn5 / 2.0) * assign24710_e33911) + (assign24710_e33903 * (-(((-locals.var_vjsmrev_dn5) / locals.var_nvtms) / ((assign24710_e33909).cosh() * (assign24710_e33909).cosh()))))) + (((locals.var_t4_dn5 / 2.0) * assign24710_e33923) + (assign24710_e33915 * (((-locals.var_vjsmrev_dn5) / locals.var_nvtms) / ((assign24710_e33921).cosh() * (assign24710_e33921).cosh()))))), ((((locals.var_t3_dn6 / 2.0) * assign24710_e33911) + (assign24710_e33903 * (-(((-locals.var_vjsmrev_dn6) / locals.var_nvtms) / ((assign24710_e33909).cosh() * (assign24710_e33909).cosh()))))) + (((locals.var_t4_dn6 / 2.0) * assign24710_e33923) + (assign24710_e33915 * (((-locals.var_vjsmrev_dn6) / locals.var_nvtms) / ((assign24710_e33921).cosh() * (assign24710_e33921).cosh()))))), ((((locals.var_t3_dn7 / 2.0) * assign24710_e33911) + (assign24710_e33903 * (-(((locals.var_vbs_jct_dn7 - locals.var_vjsmrev_dn7) / locals.var_nvtms) / ((assign24710_e33909).cosh() * (assign24710_e33909).cosh()))))) + (((locals.var_t4_dn7 / 2.0) * assign24710_e33923) + (assign24710_e33915 * (((locals.var_vbs_jct_dn7 - locals.var_vjsmrev_dn7) / locals.var_nvtms) / ((assign24710_e33921).cosh() * (assign24710_e33921).cosh()))))), ((((locals.var_t3_dn8 / 2.0) * assign24710_e33911) + (assign24710_e33903 * (-(((-locals.var_vjsmrev_dn8) / locals.var_nvtms) / ((assign24710_e33909).cosh() * (assign24710_e33909).cosh()))))) + (((locals.var_t4_dn8 / 2.0) * assign24710_e33923) + (assign24710_e33915 * (((-locals.var_vjsmrev_dn8) / locals.var_nvtms) / ((assign24710_e33921).cosh() * (assign24710_e33921).cosh()))))), ((((locals.var_t3_dn9 / 2.0) * assign24710_e33911) + (assign24710_e33903 * (-(((-locals.var_vjsmrev_dn9) / locals.var_nvtms) / ((assign24710_e33909).cosh() * (assign24710_e33909).cosh()))))) + (((locals.var_t4_dn9 / 2.0) * assign24710_e33923) + (assign24710_e33915 * (((-locals.var_vjsmrev_dn9) / locals.var_nvtms) / ((assign24710_e33921).cosh() * (assign24710_e33921).cosh()))))), ((((locals.var_t3_dn10 / 2.0) * assign24710_e33911) + (assign24710_e33903 * (-(((-locals.var_vjsmrev_dn10) / locals.var_nvtms) / ((assign24710_e33909).cosh() * (assign24710_e33909).cosh()))))) + (((locals.var_t4_dn10 / 2.0) * assign24710_e33923) + (assign24710_e33915 * (((-locals.var_vjsmrev_dn10) / locals.var_nvtms) / ((assign24710_e33921).cosh() * (assign24710_e33921).cosh()))))), ((((locals.var_t3_dn11 / 2.0) * assign24710_e33911) + (assign24710_e33903 * (-(((-locals.var_vjsmrev_dn11) / locals.var_nvtms) / ((assign24710_e33909).cosh() * (assign24710_e33909).cosh()))))) + (((locals.var_t4_dn11 / 2.0) * assign24710_e33923) + (assign24710_e33915 * (((-locals.var_vjsmrev_dn11) / locals.var_nvtms) / ((assign24710_e33921).cosh() * (assign24710_e33921).cosh()))))), ((((locals.var_t3_dn12 / 2.0) * assign24710_e33911) + (assign24710_e33903 * (-(((locals.var_vbs_jct_dn12 - locals.var_vjsmrev_dn12) / locals.var_nvtms) / ((assign24710_e33909).cosh() * (assign24710_e33909).cosh()))))) + (((locals.var_t4_dn12 / 2.0) * assign24710_e33923) + (assign24710_e33915 * (((locals.var_vbs_jct_dn12 - locals.var_vjsmrev_dn12) / locals.var_nvtms) / ((assign24710_e33921).cosh() * (assign24710_e33921).cosh()))))), ((((locals.var_t3_dn13 / 2.0) * assign24710_e33911) + (assign24710_e33903 * (-(((-locals.var_vjsmrev_dn13) / locals.var_nvtms) / ((assign24710_e33909).cosh() * (assign24710_e33909).cosh()))))) + (((locals.var_t4_dn13 / 2.0) * assign24710_e33923) + (assign24710_e33915 * (((-locals.var_vjsmrev_dn13) / locals.var_nvtms) / ((assign24710_e33921).cosh() * (assign24710_e33921).cosh()))))), ((((locals.var_t3_dn14 / 2.0) * assign24710_e33911) + (assign24710_e33903 * (-(((-locals.var_vjsmrev_dn14) / locals.var_nvtms) / ((assign24710_e33909).cosh() * (assign24710_e33909).cosh()))))) + (((locals.var_t4_dn14 / 2.0) * assign24710_e33923) + (assign24710_e33915 * (((-locals.var_vjsmrev_dn14) / locals.var_nvtms) / ((assign24710_e33921).cosh() * (assign24710_e33921).cosh()))))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign24710_e33927;
        locals.var_t6_dn0 = assign24710_e33927_d_n0;
        locals.var_t6_dn2 = assign24710_e33927_d_n2;
        locals.var_t6_dn3 = assign24710_e33927_d_n3;
        locals.var_t6_dn4 = assign24710_e33927_d_n4;
        locals.var_t6_dn5 = assign24710_e33927_d_n5;
        locals.var_t6_dn6 = assign24710_e33927_d_n6;
        locals.var_t6_dn7 = assign24710_e33927_d_n7;
        locals.var_t6_dn8 = assign24710_e33927_d_n8;
        locals.var_t6_dn9 = assign24710_e33927_d_n9;
        locals.var_t6_dn10 = assign24710_e33927_d_n10;
        locals.var_t6_dn11 = assign24710_e33927_d_n11;
        locals.var_t6_dn12 = assign24710_e33927_d_n12;
        locals.var_t6_dn13 = assign24710_e33927_d_n13;
        locals.var_t6_dn14 = assign24710_e33927_d_n14;
        locals.var_t6_rv = 0.0;

        let assign24740_e33963: f64 = if locals.var_jtss_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard615 = assign24740_e33963;
        locals.var_guard615_rv = 0.0;

        let assign24750_e33966: f64 = (p.p748 - locals.var_vbs_jct);
        let assign24750_e33969: f64 = (p.p748 * 0.001);
        let assign24750_e33970: f64 = if assign24750_e33966 < assign24750_e33969 { 1.0 } else { 0.0 };
        locals.var_guard616 = assign24750_e33970;
        locals.var_guard616_rv = 0.0;

        let (assign24760_e33981, assign24760_e33981_d_n0, assign24760_e33981_d_n2, assign24760_e33981_d_n3, assign24760_e33981_d_n4, assign24760_e33981_d_n5, assign24760_e33981_d_n6, assign24760_e33981_d_n7, assign24760_e33981_d_n8, assign24760_e33981_d_n9, assign24760_e33981_d_n10, assign24760_e33981_d_n11, assign24760_e33981_d_n12, assign24760_e33981_d_n13, assign24760_e33981_d_n14,) = {
    if ((locals.var_guard615 != 0.0) && (locals.var_guard616 != 0.0)) {
        let assign24760_e33975: f64 = (-locals.var_vbs_jct);
        let assign24760_e33977: f64 = (assign24760_e33975 / locals.var_vtm0);
        let assign24760_e33979: f64 = (assign24760_e33977 / locals.var_njts_t);
        (assign24760_e33979, 0.0, 0.0, 0.0, (-((assign24760_e33977 * locals.var_njts_t_dn4) / (locals.var_njts_t * locals.var_njts_t))), 0.0, 0.0, (((-locals.var_vbs_jct_dn7) / locals.var_vtm0) / locals.var_njts_t), 0.0, 0.0, 0.0, 0.0, (((-locals.var_vbs_jct_dn12) / locals.var_vtm0) / locals.var_njts_t), 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign24760_e33981;
        locals.var_t0_dn0 = assign24760_e33981_d_n0;
        locals.var_t0_dn2 = assign24760_e33981_d_n2;
        locals.var_t0_dn3 = assign24760_e33981_d_n3;
        locals.var_t0_dn4 = assign24760_e33981_d_n4;
        locals.var_t0_dn5 = assign24760_e33981_d_n5;
        locals.var_t0_dn6 = assign24760_e33981_d_n6;
        locals.var_t0_dn7 = assign24760_e33981_d_n7;
        locals.var_t0_dn8 = assign24760_e33981_d_n8;
        locals.var_t0_dn9 = assign24760_e33981_d_n9;
        locals.var_t0_dn10 = assign24760_e33981_d_n10;
        locals.var_t0_dn11 = assign24760_e33981_d_n11;
        locals.var_t0_dn12 = assign24760_e33981_d_n12;
        locals.var_t0_dn13 = assign24760_e33981_d_n13;
        locals.var_t0_dn14 = assign24760_e33981_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign24770_e33992, assign24770_e33992_d_n0, assign24770_e33992_d_n2, assign24770_e33992_d_n3, assign24770_e33992_d_n4, assign24770_e33992_d_n5, assign24770_e33992_d_n6, assign24770_e33992_d_n7, assign24770_e33992_d_n8, assign24770_e33992_d_n9, assign24770_e33992_d_n10, assign24770_e33992_d_n11, assign24770_e33992_d_n12, assign24770_e33992_d_n13, assign24770_e33992_d_n14,) = {
    if ((locals.var_guard615 != 0.0) && (locals.var_guard616 != 0.0)) {
        let assign24770_e33987: f64 = (locals.var_t0 * 1000.0);
        let assign24770_e33988: f64 = { let limited_exp_arg = assign24770_e33987; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign24770_e33990: f64 = (assign24770_e33988 - 1.0);
        (assign24770_e33990, ({ let limited_exp_arg = assign24770_e33987; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 * 1000.0)), ({ let limited_exp_arg = assign24770_e33987; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 * 1000.0)), ({ let limited_exp_arg = assign24770_e33987; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 * 1000.0)), ({ let limited_exp_arg = assign24770_e33987; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn4 * 1000.0)), ({ let limited_exp_arg = assign24770_e33987; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 * 1000.0)), ({ let limited_exp_arg = assign24770_e33987; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 * 1000.0)), ({ let limited_exp_arg = assign24770_e33987; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 * 1000.0)), ({ let limited_exp_arg = assign24770_e33987; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 * 1000.0)), ({ let limited_exp_arg = assign24770_e33987; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 * 1000.0)), ({ let limited_exp_arg = assign24770_e33987; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 * 1000.0)), ({ let limited_exp_arg = assign24770_e33987; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 * 1000.0)), ({ let limited_exp_arg = assign24770_e33987; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn12 * 1000.0)), ({ let limited_exp_arg = assign24770_e33987; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 * 1000.0)), ({ let limited_exp_arg = assign24770_e33987; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 * 1000.0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign24770_e33992;
        locals.var_t1_dn0 = assign24770_e33992_d_n0;
        locals.var_t1_dn2 = assign24770_e33992_d_n2;
        locals.var_t1_dn3 = assign24770_e33992_d_n3;
        locals.var_t1_dn4 = assign24770_e33992_d_n4;
        locals.var_t1_dn5 = assign24770_e33992_d_n5;
        locals.var_t1_dn6 = assign24770_e33992_d_n6;
        locals.var_t1_dn7 = assign24770_e33992_d_n7;
        locals.var_t1_dn8 = assign24770_e33992_d_n8;
        locals.var_t1_dn9 = assign24770_e33992_d_n9;
        locals.var_t1_dn10 = assign24770_e33992_d_n10;
        locals.var_t1_dn11 = assign24770_e33992_d_n11;
        locals.var_t1_dn12 = assign24770_e33992_d_n12;
        locals.var_t1_dn13 = assign24770_e33992_d_n13;
        locals.var_t1_dn14 = assign24770_e33992_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign24790_e34016, assign24790_e34016_d_n0, assign24790_e34016_d_n2, assign24790_e34016_d_n3, assign24790_e34016_d_n4, assign24790_e34016_d_n5, assign24790_e34016_d_n6, assign24790_e34016_d_n7, assign24790_e34016_d_n8, assign24790_e34016_d_n9, assign24790_e34016_d_n10, assign24790_e34016_d_n11, assign24790_e34016_d_n12, assign24790_e34016_d_n13, assign24790_e34016_d_n14,) = {
    if ((locals.var_guard615 != 0.0) && (locals.var_guard616 == 0.0)) {
        let assign24790_e34010: f64 = (-locals.var_vbs_jct);
        let assign24790_e34012: f64 = (assign24790_e34010 / locals.var_vtm0);
        let assign24790_e34014: f64 = (assign24790_e34012 / locals.var_njts_t);
        (assign24790_e34014, 0.0, 0.0, 0.0, (-((assign24790_e34012 * locals.var_njts_t_dn4) / (locals.var_njts_t * locals.var_njts_t))), 0.0, 0.0, (((-locals.var_vbs_jct_dn7) / locals.var_vtm0) / locals.var_njts_t), 0.0, 0.0, 0.0, 0.0, (((-locals.var_vbs_jct_dn12) / locals.var_vtm0) / locals.var_njts_t), 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign24790_e34016;
        locals.var_t0_dn0 = assign24790_e34016_d_n0;
        locals.var_t0_dn2 = assign24790_e34016_d_n2;
        locals.var_t0_dn3 = assign24790_e34016_d_n3;
        locals.var_t0_dn4 = assign24790_e34016_d_n4;
        locals.var_t0_dn5 = assign24790_e34016_d_n5;
        locals.var_t0_dn6 = assign24790_e34016_d_n6;
        locals.var_t0_dn7 = assign24790_e34016_d_n7;
        locals.var_t0_dn8 = assign24790_e34016_d_n8;
        locals.var_t0_dn9 = assign24790_e34016_d_n9;
        locals.var_t0_dn10 = assign24790_e34016_d_n10;
        locals.var_t0_dn11 = assign24790_e34016_d_n11;
        locals.var_t0_dn12 = assign24790_e34016_d_n12;
        locals.var_t0_dn13 = assign24790_e34016_d_n13;
        locals.var_t0_dn14 = assign24790_e34016_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign24800_e34032, assign24800_e34032_d_n0, assign24800_e34032_d_n2, assign24800_e34032_d_n3, assign24800_e34032_d_n4, assign24800_e34032_d_n5, assign24800_e34032_d_n6, assign24800_e34032_d_n7, assign24800_e34032_d_n8, assign24800_e34032_d_n9, assign24800_e34032_d_n10, assign24800_e34032_d_n11, assign24800_e34032_d_n12, assign24800_e34032_d_n13, assign24800_e34032_d_n14,) = {
    if ((locals.var_guard615 != 0.0) && (locals.var_guard616 == 0.0)) {
        let assign24800_e34023: f64 = (locals.var_t0 * p.p748);
        let assign24800_e34026: f64 = (p.p748 - locals.var_vbs_jct);
        let assign24800_e34027: f64 = (assign24800_e34023 / assign24800_e34026);
        let assign24800_e34028: f64 = { let limited_exp_arg = assign24800_e34027; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign24800_e34030: f64 = (assign24800_e34028 - 1.0);
        (assign24800_e34030, ({ let limited_exp_arg = assign24800_e34027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 * p.p748) / assign24800_e34026)), ({ let limited_exp_arg = assign24800_e34027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 * p.p748) / assign24800_e34026)), ({ let limited_exp_arg = assign24800_e34027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn3 * p.p748) / assign24800_e34026)), ({ let limited_exp_arg = assign24800_e34027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 * p.p748) / assign24800_e34026)), ({ let limited_exp_arg = assign24800_e34027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn5 * p.p748) / assign24800_e34026)), ({ let limited_exp_arg = assign24800_e34027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 * p.p748) / assign24800_e34026)), ({ let limited_exp_arg = assign24800_e34027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn7 * p.p748) * assign24800_e34026) - (assign24800_e34023 * (-locals.var_vbs_jct_dn7))) / (assign24800_e34026 * assign24800_e34026))), ({ let limited_exp_arg = assign24800_e34027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 * p.p748) / assign24800_e34026)), ({ let limited_exp_arg = assign24800_e34027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 * p.p748) / assign24800_e34026)), ({ let limited_exp_arg = assign24800_e34027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 * p.p748) / assign24800_e34026)), ({ let limited_exp_arg = assign24800_e34027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 * p.p748) / assign24800_e34026)), ({ let limited_exp_arg = assign24800_e34027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn12 * p.p748) * assign24800_e34026) - (assign24800_e34023 * (-locals.var_vbs_jct_dn12))) / (assign24800_e34026 * assign24800_e34026))), ({ let limited_exp_arg = assign24800_e34027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 * p.p748) / assign24800_e34026)), ({ let limited_exp_arg = assign24800_e34027; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 * p.p748) / assign24800_e34026)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign24800_e34032;
        locals.var_t1_dn0 = assign24800_e34032_d_n0;
        locals.var_t1_dn2 = assign24800_e34032_d_n2;
        locals.var_t1_dn3 = assign24800_e34032_d_n3;
        locals.var_t1_dn4 = assign24800_e34032_d_n4;
        locals.var_t1_dn5 = assign24800_e34032_d_n5;
        locals.var_t1_dn6 = assign24800_e34032_d_n6;
        locals.var_t1_dn7 = assign24800_e34032_d_n7;
        locals.var_t1_dn8 = assign24800_e34032_d_n8;
        locals.var_t1_dn9 = assign24800_e34032_d_n9;
        locals.var_t1_dn10 = assign24800_e34032_d_n10;
        locals.var_t1_dn11 = assign24800_e34032_d_n11;
        locals.var_t1_dn12 = assign24800_e34032_d_n12;
        locals.var_t1_dn13 = assign24800_e34032_d_n13;
        locals.var_t1_dn14 = assign24800_e34032_d_n14;
        locals.var_t1_rv = 0.0;

        let assign24820_e34048: f64 = if locals.var_jtssws_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard617 = assign24820_e34048;
        locals.var_guard617_rv = 0.0;

        let assign24830_e34051: f64 = (p.p750 - locals.var_vbs_jct);
        let assign24830_e34054: f64 = (p.p750 * 0.001);
        let assign24830_e34055: f64 = if assign24830_e34051 < assign24830_e34054 { 1.0 } else { 0.0 };
        locals.var_guard618 = assign24830_e34055;
        locals.var_guard618_rv = 0.0;

        let (assign24840_e34066, assign24840_e34066_d_n0, assign24840_e34066_d_n2, assign24840_e34066_d_n3, assign24840_e34066_d_n4, assign24840_e34066_d_n5, assign24840_e34066_d_n6, assign24840_e34066_d_n7, assign24840_e34066_d_n8, assign24840_e34066_d_n9, assign24840_e34066_d_n10, assign24840_e34066_d_n11, assign24840_e34066_d_n12, assign24840_e34066_d_n13, assign24840_e34066_d_n14,) = {
    if ((locals.var_guard617 != 0.0) && (locals.var_guard618 != 0.0)) {
        let assign24840_e34060: f64 = (-locals.var_vbs_jct);
        let assign24840_e34062: f64 = (assign24840_e34060 / locals.var_vtm0);
        let assign24840_e34064: f64 = (assign24840_e34062 / locals.var_njtssw_t);
        (assign24840_e34064, 0.0, 0.0, 0.0, (-((assign24840_e34062 * locals.var_njtssw_t_dn4) / (locals.var_njtssw_t * locals.var_njtssw_t))), 0.0, 0.0, (((-locals.var_vbs_jct_dn7) / locals.var_vtm0) / locals.var_njtssw_t), 0.0, 0.0, 0.0, 0.0, (((-locals.var_vbs_jct_dn12) / locals.var_vtm0) / locals.var_njtssw_t), 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign24840_e34066;
        locals.var_t0_dn0 = assign24840_e34066_d_n0;
        locals.var_t0_dn2 = assign24840_e34066_d_n2;
        locals.var_t0_dn3 = assign24840_e34066_d_n3;
        locals.var_t0_dn4 = assign24840_e34066_d_n4;
        locals.var_t0_dn5 = assign24840_e34066_d_n5;
        locals.var_t0_dn6 = assign24840_e34066_d_n6;
        locals.var_t0_dn7 = assign24840_e34066_d_n7;
        locals.var_t0_dn8 = assign24840_e34066_d_n8;
        locals.var_t0_dn9 = assign24840_e34066_d_n9;
        locals.var_t0_dn10 = assign24840_e34066_d_n10;
        locals.var_t0_dn11 = assign24840_e34066_d_n11;
        locals.var_t0_dn12 = assign24840_e34066_d_n12;
        locals.var_t0_dn13 = assign24840_e34066_d_n13;
        locals.var_t0_dn14 = assign24840_e34066_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign24850_e34077, assign24850_e34077_d_n0, assign24850_e34077_d_n2, assign24850_e34077_d_n3, assign24850_e34077_d_n4, assign24850_e34077_d_n5, assign24850_e34077_d_n6, assign24850_e34077_d_n7, assign24850_e34077_d_n8, assign24850_e34077_d_n9, assign24850_e34077_d_n10, assign24850_e34077_d_n11, assign24850_e34077_d_n12, assign24850_e34077_d_n13, assign24850_e34077_d_n14,) = {
    if ((locals.var_guard617 != 0.0) && (locals.var_guard618 != 0.0)) {
        let assign24850_e34072: f64 = (locals.var_t0 * 1000.0);
        let assign24850_e34073: f64 = { let limited_exp_arg = assign24850_e34072; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign24850_e34075: f64 = (assign24850_e34073 - 1.0);
        (assign24850_e34075, ({ let limited_exp_arg = assign24850_e34072; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 * 1000.0)), ({ let limited_exp_arg = assign24850_e34072; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 * 1000.0)), ({ let limited_exp_arg = assign24850_e34072; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 * 1000.0)), ({ let limited_exp_arg = assign24850_e34072; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn4 * 1000.0)), ({ let limited_exp_arg = assign24850_e34072; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 * 1000.0)), ({ let limited_exp_arg = assign24850_e34072; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 * 1000.0)), ({ let limited_exp_arg = assign24850_e34072; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 * 1000.0)), ({ let limited_exp_arg = assign24850_e34072; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 * 1000.0)), ({ let limited_exp_arg = assign24850_e34072; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 * 1000.0)), ({ let limited_exp_arg = assign24850_e34072; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 * 1000.0)), ({ let limited_exp_arg = assign24850_e34072; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 * 1000.0)), ({ let limited_exp_arg = assign24850_e34072; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn12 * 1000.0)), ({ let limited_exp_arg = assign24850_e34072; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 * 1000.0)), ({ let limited_exp_arg = assign24850_e34072; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 * 1000.0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign24850_e34077;
        locals.var_t1_dn0 = assign24850_e34077_d_n0;
        locals.var_t1_dn2 = assign24850_e34077_d_n2;
        locals.var_t1_dn3 = assign24850_e34077_d_n3;
        locals.var_t1_dn4 = assign24850_e34077_d_n4;
        locals.var_t1_dn5 = assign24850_e34077_d_n5;
        locals.var_t1_dn6 = assign24850_e34077_d_n6;
        locals.var_t1_dn7 = assign24850_e34077_d_n7;
        locals.var_t1_dn8 = assign24850_e34077_d_n8;
        locals.var_t1_dn9 = assign24850_e34077_d_n9;
        locals.var_t1_dn10 = assign24850_e34077_d_n10;
        locals.var_t1_dn11 = assign24850_e34077_d_n11;
        locals.var_t1_dn12 = assign24850_e34077_d_n12;
        locals.var_t1_dn13 = assign24850_e34077_d_n13;
        locals.var_t1_dn14 = assign24850_e34077_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign24870_e34101, assign24870_e34101_d_n0, assign24870_e34101_d_n2, assign24870_e34101_d_n3, assign24870_e34101_d_n4, assign24870_e34101_d_n5, assign24870_e34101_d_n6, assign24870_e34101_d_n7, assign24870_e34101_d_n8, assign24870_e34101_d_n9, assign24870_e34101_d_n10, assign24870_e34101_d_n11, assign24870_e34101_d_n12, assign24870_e34101_d_n13, assign24870_e34101_d_n14,) = {
    if ((locals.var_guard617 != 0.0) && (locals.var_guard618 == 0.0)) {
        let assign24870_e34095: f64 = (-locals.var_vbs_jct);
        let assign24870_e34097: f64 = (assign24870_e34095 / locals.var_vtm0);
        let assign24870_e34099: f64 = (assign24870_e34097 / locals.var_njtssw_t);
        (assign24870_e34099, 0.0, 0.0, 0.0, (-((assign24870_e34097 * locals.var_njtssw_t_dn4) / (locals.var_njtssw_t * locals.var_njtssw_t))), 0.0, 0.0, (((-locals.var_vbs_jct_dn7) / locals.var_vtm0) / locals.var_njtssw_t), 0.0, 0.0, 0.0, 0.0, (((-locals.var_vbs_jct_dn12) / locals.var_vtm0) / locals.var_njtssw_t), 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign24870_e34101;
        locals.var_t0_dn0 = assign24870_e34101_d_n0;
        locals.var_t0_dn2 = assign24870_e34101_d_n2;
        locals.var_t0_dn3 = assign24870_e34101_d_n3;
        locals.var_t0_dn4 = assign24870_e34101_d_n4;
        locals.var_t0_dn5 = assign24870_e34101_d_n5;
        locals.var_t0_dn6 = assign24870_e34101_d_n6;
        locals.var_t0_dn7 = assign24870_e34101_d_n7;
        locals.var_t0_dn8 = assign24870_e34101_d_n8;
        locals.var_t0_dn9 = assign24870_e34101_d_n9;
        locals.var_t0_dn10 = assign24870_e34101_d_n10;
        locals.var_t0_dn11 = assign24870_e34101_d_n11;
        locals.var_t0_dn12 = assign24870_e34101_d_n12;
        locals.var_t0_dn13 = assign24870_e34101_d_n13;
        locals.var_t0_dn14 = assign24870_e34101_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign24880_e34117, assign24880_e34117_d_n0, assign24880_e34117_d_n2, assign24880_e34117_d_n3, assign24880_e34117_d_n4, assign24880_e34117_d_n5, assign24880_e34117_d_n6, assign24880_e34117_d_n7, assign24880_e34117_d_n8, assign24880_e34117_d_n9, assign24880_e34117_d_n10, assign24880_e34117_d_n11, assign24880_e34117_d_n12, assign24880_e34117_d_n13, assign24880_e34117_d_n14,) = {
    if ((locals.var_guard617 != 0.0) && (locals.var_guard618 == 0.0)) {
        let assign24880_e34108: f64 = (locals.var_t0 * p.p750);
        let assign24880_e34111: f64 = (p.p750 - locals.var_vbs_jct);
        let assign24880_e34112: f64 = (assign24880_e34108 / assign24880_e34111);
        let assign24880_e34113: f64 = { let limited_exp_arg = assign24880_e34112; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign24880_e34115: f64 = (assign24880_e34113 - 1.0);
        (assign24880_e34115, ({ let limited_exp_arg = assign24880_e34112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 * p.p750) / assign24880_e34111)), ({ let limited_exp_arg = assign24880_e34112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 * p.p750) / assign24880_e34111)), ({ let limited_exp_arg = assign24880_e34112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn3 * p.p750) / assign24880_e34111)), ({ let limited_exp_arg = assign24880_e34112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 * p.p750) / assign24880_e34111)), ({ let limited_exp_arg = assign24880_e34112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn5 * p.p750) / assign24880_e34111)), ({ let limited_exp_arg = assign24880_e34112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 * p.p750) / assign24880_e34111)), ({ let limited_exp_arg = assign24880_e34112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn7 * p.p750) * assign24880_e34111) - (assign24880_e34108 * (-locals.var_vbs_jct_dn7))) / (assign24880_e34111 * assign24880_e34111))), ({ let limited_exp_arg = assign24880_e34112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 * p.p750) / assign24880_e34111)), ({ let limited_exp_arg = assign24880_e34112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 * p.p750) / assign24880_e34111)), ({ let limited_exp_arg = assign24880_e34112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 * p.p750) / assign24880_e34111)), ({ let limited_exp_arg = assign24880_e34112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 * p.p750) / assign24880_e34111)), ({ let limited_exp_arg = assign24880_e34112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn12 * p.p750) * assign24880_e34111) - (assign24880_e34108 * (-locals.var_vbs_jct_dn12))) / (assign24880_e34111 * assign24880_e34111))), ({ let limited_exp_arg = assign24880_e34112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 * p.p750) / assign24880_e34111)), ({ let limited_exp_arg = assign24880_e34112; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 * p.p750) / assign24880_e34111)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign24880_e34117;
        locals.var_t1_dn0 = assign24880_e34117_d_n0;
        locals.var_t1_dn2 = assign24880_e34117_d_n2;
        locals.var_t1_dn3 = assign24880_e34117_d_n3;
        locals.var_t1_dn4 = assign24880_e34117_d_n4;
        locals.var_t1_dn5 = assign24880_e34117_d_n5;
        locals.var_t1_dn6 = assign24880_e34117_d_n6;
        locals.var_t1_dn7 = assign24880_e34117_d_n7;
        locals.var_t1_dn8 = assign24880_e34117_d_n8;
        locals.var_t1_dn9 = assign24880_e34117_d_n9;
        locals.var_t1_dn10 = assign24880_e34117_d_n10;
        locals.var_t1_dn11 = assign24880_e34117_d_n11;
        locals.var_t1_dn12 = assign24880_e34117_d_n12;
        locals.var_t1_dn13 = assign24880_e34117_d_n13;
        locals.var_t1_dn14 = assign24880_e34117_d_n14;
        locals.var_t1_rv = 0.0;

        let assign24900_e34133: f64 = if locals.var_jtsswgs_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard619 = assign24900_e34133;
        locals.var_guard619_rv = 0.0;

        let assign24910_e34136: f64 = (p.p752 - locals.var_vbs_jct);
        let assign24910_e34139: f64 = (p.p752 * 0.001);
        let assign24910_e34140: f64 = if assign24910_e34136 < assign24910_e34139 { 1.0 } else { 0.0 };
        locals.var_guard620 = assign24910_e34140;
        locals.var_guard620_rv = 0.0;

        let (assign24920_e34151, assign24920_e34151_d_n0, assign24920_e34151_d_n2, assign24920_e34151_d_n3, assign24920_e34151_d_n4, assign24920_e34151_d_n5, assign24920_e34151_d_n6, assign24920_e34151_d_n7, assign24920_e34151_d_n8, assign24920_e34151_d_n9, assign24920_e34151_d_n10, assign24920_e34151_d_n11, assign24920_e34151_d_n12, assign24920_e34151_d_n13, assign24920_e34151_d_n14,) = {
    if ((locals.var_guard619 != 0.0) && (locals.var_guard620 != 0.0)) {
        let assign24920_e34145: f64 = (-locals.var_vbs_jct);
        let assign24920_e34147: f64 = (assign24920_e34145 / locals.var_vtm0);
        let assign24920_e34149: f64 = (assign24920_e34147 / locals.var_njtsswg_t);
        (assign24920_e34149, 0.0, 0.0, 0.0, (-((assign24920_e34147 * locals.var_njtsswg_t_dn4) / (locals.var_njtsswg_t * locals.var_njtsswg_t))), 0.0, 0.0, (((-locals.var_vbs_jct_dn7) / locals.var_vtm0) / locals.var_njtsswg_t), 0.0, 0.0, 0.0, 0.0, (((-locals.var_vbs_jct_dn12) / locals.var_vtm0) / locals.var_njtsswg_t), 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign24920_e34151;
        locals.var_t0_dn0 = assign24920_e34151_d_n0;
        locals.var_t0_dn2 = assign24920_e34151_d_n2;
        locals.var_t0_dn3 = assign24920_e34151_d_n3;
        locals.var_t0_dn4 = assign24920_e34151_d_n4;
        locals.var_t0_dn5 = assign24920_e34151_d_n5;
        locals.var_t0_dn6 = assign24920_e34151_d_n6;
        locals.var_t0_dn7 = assign24920_e34151_d_n7;
        locals.var_t0_dn8 = assign24920_e34151_d_n8;
        locals.var_t0_dn9 = assign24920_e34151_d_n9;
        locals.var_t0_dn10 = assign24920_e34151_d_n10;
        locals.var_t0_dn11 = assign24920_e34151_d_n11;
        locals.var_t0_dn12 = assign24920_e34151_d_n12;
        locals.var_t0_dn13 = assign24920_e34151_d_n13;
        locals.var_t0_dn14 = assign24920_e34151_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign24930_e34162, assign24930_e34162_d_n0, assign24930_e34162_d_n2, assign24930_e34162_d_n3, assign24930_e34162_d_n4, assign24930_e34162_d_n5, assign24930_e34162_d_n6, assign24930_e34162_d_n7, assign24930_e34162_d_n8, assign24930_e34162_d_n9, assign24930_e34162_d_n10, assign24930_e34162_d_n11, assign24930_e34162_d_n12, assign24930_e34162_d_n13, assign24930_e34162_d_n14,) = {
    if ((locals.var_guard619 != 0.0) && (locals.var_guard620 != 0.0)) {
        let assign24930_e34157: f64 = (locals.var_t0 * 1000.0);
        let assign24930_e34158: f64 = { let limited_exp_arg = assign24930_e34157; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign24930_e34160: f64 = (assign24930_e34158 - 1.0);
        (assign24930_e34160, ({ let limited_exp_arg = assign24930_e34157; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 * 1000.0)), ({ let limited_exp_arg = assign24930_e34157; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 * 1000.0)), ({ let limited_exp_arg = assign24930_e34157; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 * 1000.0)), ({ let limited_exp_arg = assign24930_e34157; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn4 * 1000.0)), ({ let limited_exp_arg = assign24930_e34157; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 * 1000.0)), ({ let limited_exp_arg = assign24930_e34157; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 * 1000.0)), ({ let limited_exp_arg = assign24930_e34157; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 * 1000.0)), ({ let limited_exp_arg = assign24930_e34157; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 * 1000.0)), ({ let limited_exp_arg = assign24930_e34157; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 * 1000.0)), ({ let limited_exp_arg = assign24930_e34157; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 * 1000.0)), ({ let limited_exp_arg = assign24930_e34157; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 * 1000.0)), ({ let limited_exp_arg = assign24930_e34157; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn12 * 1000.0)), ({ let limited_exp_arg = assign24930_e34157; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 * 1000.0)), ({ let limited_exp_arg = assign24930_e34157; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 * 1000.0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign24930_e34162;
        locals.var_t1_dn0 = assign24930_e34162_d_n0;
        locals.var_t1_dn2 = assign24930_e34162_d_n2;
        locals.var_t1_dn3 = assign24930_e34162_d_n3;
        locals.var_t1_dn4 = assign24930_e34162_d_n4;
        locals.var_t1_dn5 = assign24930_e34162_d_n5;
        locals.var_t1_dn6 = assign24930_e34162_d_n6;
        locals.var_t1_dn7 = assign24930_e34162_d_n7;
        locals.var_t1_dn8 = assign24930_e34162_d_n8;
        locals.var_t1_dn9 = assign24930_e34162_d_n9;
        locals.var_t1_dn10 = assign24930_e34162_d_n10;
        locals.var_t1_dn11 = assign24930_e34162_d_n11;
        locals.var_t1_dn12 = assign24930_e34162_d_n12;
        locals.var_t1_dn13 = assign24930_e34162_d_n13;
        locals.var_t1_dn14 = assign24930_e34162_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign24950_e34188, assign24950_e34188_d_n0, assign24950_e34188_d_n2, assign24950_e34188_d_n3, assign24950_e34188_d_n4, assign24950_e34188_d_n5, assign24950_e34188_d_n6, assign24950_e34188_d_n7, assign24950_e34188_d_n8, assign24950_e34188_d_n9, assign24950_e34188_d_n10, assign24950_e34188_d_n11, assign24950_e34188_d_n12, assign24950_e34188_d_n13, assign24950_e34188_d_n14,) = {
    if ((locals.var_guard619 != 0.0) && (locals.var_guard620 == 0.0)) {
        let assign24950_e34182: f64 = (-locals.var_vbs_jct);
        let assign24950_e34184: f64 = (assign24950_e34182 / locals.var_vtm0);
        let assign24950_e34186: f64 = (assign24950_e34184 / locals.var_njtsswg_t);
        (assign24950_e34186, 0.0, 0.0, 0.0, (-((assign24950_e34184 * locals.var_njtsswg_t_dn4) / (locals.var_njtsswg_t * locals.var_njtsswg_t))), 0.0, 0.0, (((-locals.var_vbs_jct_dn7) / locals.var_vtm0) / locals.var_njtsswg_t), 0.0, 0.0, 0.0, 0.0, (((-locals.var_vbs_jct_dn12) / locals.var_vtm0) / locals.var_njtsswg_t), 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign24950_e34188;
        locals.var_t0_dn0 = assign24950_e34188_d_n0;
        locals.var_t0_dn2 = assign24950_e34188_d_n2;
        locals.var_t0_dn3 = assign24950_e34188_d_n3;
        locals.var_t0_dn4 = assign24950_e34188_d_n4;
        locals.var_t0_dn5 = assign24950_e34188_d_n5;
        locals.var_t0_dn6 = assign24950_e34188_d_n6;
        locals.var_t0_dn7 = assign24950_e34188_d_n7;
        locals.var_t0_dn8 = assign24950_e34188_d_n8;
        locals.var_t0_dn9 = assign24950_e34188_d_n9;
        locals.var_t0_dn10 = assign24950_e34188_d_n10;
        locals.var_t0_dn11 = assign24950_e34188_d_n11;
        locals.var_t0_dn12 = assign24950_e34188_d_n12;
        locals.var_t0_dn13 = assign24950_e34188_d_n13;
        locals.var_t0_dn14 = assign24950_e34188_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign24960_e34204, assign24960_e34204_d_n0, assign24960_e34204_d_n2, assign24960_e34204_d_n3, assign24960_e34204_d_n4, assign24960_e34204_d_n5, assign24960_e34204_d_n6, assign24960_e34204_d_n7, assign24960_e34204_d_n8, assign24960_e34204_d_n9, assign24960_e34204_d_n10, assign24960_e34204_d_n11, assign24960_e34204_d_n12, assign24960_e34204_d_n13, assign24960_e34204_d_n14,) = {
    if ((locals.var_guard619 != 0.0) && (locals.var_guard620 == 0.0)) {
        let assign24960_e34195: f64 = (locals.var_t0 * p.p752);
        let assign24960_e34198: f64 = (p.p752 - locals.var_vbs_jct);
        let assign24960_e34199: f64 = (assign24960_e34195 / assign24960_e34198);
        let assign24960_e34200: f64 = { let limited_exp_arg = assign24960_e34199; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign24960_e34202: f64 = (assign24960_e34200 - 1.0);
        (assign24960_e34202, ({ let limited_exp_arg = assign24960_e34199; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 * p.p752) / assign24960_e34198)), ({ let limited_exp_arg = assign24960_e34199; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 * p.p752) / assign24960_e34198)), ({ let limited_exp_arg = assign24960_e34199; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn3 * p.p752) / assign24960_e34198)), ({ let limited_exp_arg = assign24960_e34199; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 * p.p752) / assign24960_e34198)), ({ let limited_exp_arg = assign24960_e34199; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn5 * p.p752) / assign24960_e34198)), ({ let limited_exp_arg = assign24960_e34199; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 * p.p752) / assign24960_e34198)), ({ let limited_exp_arg = assign24960_e34199; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn7 * p.p752) * assign24960_e34198) - (assign24960_e34195 * (-locals.var_vbs_jct_dn7))) / (assign24960_e34198 * assign24960_e34198))), ({ let limited_exp_arg = assign24960_e34199; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 * p.p752) / assign24960_e34198)), ({ let limited_exp_arg = assign24960_e34199; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 * p.p752) / assign24960_e34198)), ({ let limited_exp_arg = assign24960_e34199; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 * p.p752) / assign24960_e34198)), ({ let limited_exp_arg = assign24960_e34199; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 * p.p752) / assign24960_e34198)), ({ let limited_exp_arg = assign24960_e34199; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn12 * p.p752) * assign24960_e34198) - (assign24960_e34195 * (-locals.var_vbs_jct_dn12))) / (assign24960_e34198 * assign24960_e34198))), ({ let limited_exp_arg = assign24960_e34199; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn13 * p.p752) / assign24960_e34198)), ({ let limited_exp_arg = assign24960_e34199; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 * p.p752) / assign24960_e34198)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign24960_e34204;
        locals.var_t1_dn0 = assign24960_e34204_d_n0;
        locals.var_t1_dn2 = assign24960_e34204_d_n2;
        locals.var_t1_dn3 = assign24960_e34204_d_n3;
        locals.var_t1_dn4 = assign24960_e34204_d_n4;
        locals.var_t1_dn5 = assign24960_e34204_d_n5;
        locals.var_t1_dn6 = assign24960_e34204_d_n6;
        locals.var_t1_dn7 = assign24960_e34204_d_n7;
        locals.var_t1_dn8 = assign24960_e34204_d_n8;
        locals.var_t1_dn9 = assign24960_e34204_d_n9;
        locals.var_t1_dn10 = assign24960_e34204_d_n10;
        locals.var_t1_dn11 = assign24960_e34204_d_n11;
        locals.var_t1_dn12 = assign24960_e34204_d_n12;
        locals.var_t1_dn13 = assign24960_e34204_d_n13;
        locals.var_t1_dn14 = assign24960_e34204_d_n14;
        locals.var_t1_rv = 0.0;

        let assign24980_e34222: f64 = (locals.var_vbd_jct / locals.var_nvtmd);
        locals.var_t0 = assign24980_e34222;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = (-((locals.var_vbd_jct * locals.var_nvtmd_dn4) / (locals.var_nvtmd * locals.var_nvtmd)));
        locals.var_t0_dn5 = (locals.var_vbd_jct_dn5 / locals.var_nvtmd);
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn12 = 0.0;
        locals.var_t0_dn13 = (locals.var_vbd_jct_dn13 / locals.var_nvtmd);
        locals.var_t0_dn14 = 0.0;
        locals.var_t0_rv = 0.0;

        let assign24990_e34224: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign24990_e34226: f64 = (assign24990_e34224 - 1.0);
        locals.var_t1 = assign24990_e34226;
        locals.var_t1_dn0 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0);
        locals.var_t1_dn2 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2);
        locals.var_t1_dn3 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3);
        locals.var_t1_dn4 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4);
        locals.var_t1_dn5 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5);
        locals.var_t1_dn6 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6);
        locals.var_t1_dn7 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7);
        locals.var_t1_dn8 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8);
        locals.var_t1_dn9 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9);
        locals.var_t1_dn10 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10);
        locals.var_t1_dn11 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11);
        locals.var_t1_dn12 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn12);
        locals.var_t1_dn13 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13);
        locals.var_t1_dn14 = ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14);
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_74(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign25000_e34231: f64 = (locals.var_vbd_jct - locals.var_vjdmrev);
        let assign25000_e34232: f64 = (locals.var_dslprev * assign25000_e34231);
        let assign25000_e34233: f64 = (locals.var_ivjdmrev + assign25000_e34232);
        locals.var_t2 = assign25000_e34233;
        locals.var_t2_dn0 = (locals.var_ivjdmrev_dn0 + ((locals.var_dslprev_dn0 * assign25000_e34231) + (locals.var_dslprev * (-locals.var_vjdmrev_dn0))));
        locals.var_t2_dn2 = (locals.var_ivjdmrev_dn2 + ((locals.var_dslprev_dn2 * assign25000_e34231) + (locals.var_dslprev * (-locals.var_vjdmrev_dn2))));
        locals.var_t2_dn3 = (locals.var_ivjdmrev_dn3 + ((locals.var_dslprev_dn3 * assign25000_e34231) + (locals.var_dslprev * (-locals.var_vjdmrev_dn3))));
        locals.var_t2_dn4 = (locals.var_ivjdmrev_dn4 + ((locals.var_dslprev_dn4 * assign25000_e34231) + (locals.var_dslprev * (-locals.var_vjdmrev_dn4))));
        locals.var_t2_dn5 = (locals.var_ivjdmrev_dn5 + ((locals.var_dslprev_dn5 * assign25000_e34231) + (locals.var_dslprev * (locals.var_vbd_jct_dn5 - locals.var_vjdmrev_dn5))));
        locals.var_t2_dn6 = (locals.var_ivjdmrev_dn6 + ((locals.var_dslprev_dn6 * assign25000_e34231) + (locals.var_dslprev * (-locals.var_vjdmrev_dn6))));
        locals.var_t2_dn7 = (locals.var_ivjdmrev_dn7 + ((locals.var_dslprev_dn7 * assign25000_e34231) + (locals.var_dslprev * (-locals.var_vjdmrev_dn7))));
        locals.var_t2_dn8 = (locals.var_ivjdmrev_dn8 + ((locals.var_dslprev_dn8 * assign25000_e34231) + (locals.var_dslprev * (-locals.var_vjdmrev_dn8))));
        locals.var_t2_dn9 = (locals.var_ivjdmrev_dn9 + ((locals.var_dslprev_dn9 * assign25000_e34231) + (locals.var_dslprev * (-locals.var_vjdmrev_dn9))));
        locals.var_t2_dn10 = (locals.var_ivjdmrev_dn10 + ((locals.var_dslprev_dn10 * assign25000_e34231) + (locals.var_dslprev * (-locals.var_vjdmrev_dn10))));
        locals.var_t2_dn11 = (locals.var_ivjdmrev_dn11 + ((locals.var_dslprev_dn11 * assign25000_e34231) + (locals.var_dslprev * (-locals.var_vjdmrev_dn11))));
        locals.var_t2_dn12 = (locals.var_ivjdmrev_dn12 + ((locals.var_dslprev_dn12 * assign25000_e34231) + (locals.var_dslprev * (-locals.var_vjdmrev_dn12))));
        locals.var_t2_dn13 = (locals.var_ivjdmrev_dn13 + ((locals.var_dslprev_dn13 * assign25000_e34231) + (locals.var_dslprev * (locals.var_vbd_jct_dn13 - locals.var_vjdmrev_dn13))));
        locals.var_t2_dn14 = (locals.var_ivjdmrev_dn14 + ((locals.var_dslprev_dn14 * assign25000_e34231) + (locals.var_dslprev * (-locals.var_vjdmrev_dn14))));
        locals.var_t2_rv = 0.0;

        let assign25010_e34236: f64 = (locals.var_oneminusxpart * locals.var_t1);
        let assign25010_e34238: f64 = (assign25010_e34236 * locals.var_t2);
        locals.var_t3 = assign25010_e34238;
        locals.var_t3_dn0 = (((locals.var_oneminusxpart * locals.var_t1_dn0) * locals.var_t2) + (assign25010_e34236 * locals.var_t2_dn0));
        locals.var_t3_dn2 = (((locals.var_oneminusxpart * locals.var_t1_dn2) * locals.var_t2) + (assign25010_e34236 * locals.var_t2_dn2));
        locals.var_t3_dn3 = (((locals.var_oneminusxpart * locals.var_t1_dn3) * locals.var_t2) + (assign25010_e34236 * locals.var_t2_dn3));
        locals.var_t3_dn4 = (((locals.var_oneminusxpart * locals.var_t1_dn4) * locals.var_t2) + (assign25010_e34236 * locals.var_t2_dn4));
        locals.var_t3_dn5 = (((locals.var_oneminusxpart * locals.var_t1_dn5) * locals.var_t2) + (assign25010_e34236 * locals.var_t2_dn5));
        locals.var_t3_dn6 = (((locals.var_oneminusxpart * locals.var_t1_dn6) * locals.var_t2) + (assign25010_e34236 * locals.var_t2_dn6));
        locals.var_t3_dn7 = (((locals.var_oneminusxpart * locals.var_t1_dn7) * locals.var_t2) + (assign25010_e34236 * locals.var_t2_dn7));
        locals.var_t3_dn8 = (((locals.var_oneminusxpart * locals.var_t1_dn8) * locals.var_t2) + (assign25010_e34236 * locals.var_t2_dn8));
        locals.var_t3_dn9 = (((locals.var_oneminusxpart * locals.var_t1_dn9) * locals.var_t2) + (assign25010_e34236 * locals.var_t2_dn9));
        locals.var_t3_dn10 = (((locals.var_oneminusxpart * locals.var_t1_dn10) * locals.var_t2) + (assign25010_e34236 * locals.var_t2_dn10));
        locals.var_t3_dn11 = (((locals.var_oneminusxpart * locals.var_t1_dn11) * locals.var_t2) + (assign25010_e34236 * locals.var_t2_dn11));
        locals.var_t3_dn12 = (((locals.var_oneminusxpart * locals.var_t1_dn12) * locals.var_t2) + (assign25010_e34236 * locals.var_t2_dn12));
        locals.var_t3_dn13 = (((locals.var_oneminusxpart * locals.var_t1_dn13) * locals.var_t2) + (assign25010_e34236 * locals.var_t2_dn13));
        locals.var_t3_dn14 = (((locals.var_oneminusxpart * locals.var_t1_dn14) * locals.var_t2) + (assign25010_e34236 * locals.var_t2_dn14));
        locals.var_t3_rv = 0.0;

        let assign25020_e34241: f64 = (p.p732 + locals.var_vbd_jct);
        let assign25020_e34243: f64 = (assign25020_e34241 / locals.var_nvtmd);
        locals.var_t1 = assign25020_e34243;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = (-((assign25020_e34241 * locals.var_nvtmd_dn4) / (locals.var_nvtmd * locals.var_nvtmd)));
        locals.var_t1_dn5 = (locals.var_vbd_jct_dn5 / locals.var_nvtmd);
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn12 = 0.0;
        locals.var_t1_dn13 = (locals.var_vbd_jct_dn13 / locals.var_nvtmd);
        locals.var_t1_dn14 = 0.0;
        locals.var_t1_rv = 0.0;

        let assign25030_e34245: f64 = (-locals.var_t1);
        let assign25030_e34246: f64 = { let limited_exp_arg = assign25030_e34245; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        locals.var_t2 = assign25030_e34246;
        locals.var_t2_dn0 = ({ let limited_exp_arg = assign25030_e34245; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn0));
        locals.var_t2_dn2 = ({ let limited_exp_arg = assign25030_e34245; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn2));
        locals.var_t2_dn3 = ({ let limited_exp_arg = assign25030_e34245; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn3));
        locals.var_t2_dn4 = ({ let limited_exp_arg = assign25030_e34245; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn4));
        locals.var_t2_dn5 = ({ let limited_exp_arg = assign25030_e34245; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn5));
        locals.var_t2_dn6 = ({ let limited_exp_arg = assign25030_e34245; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn6));
        locals.var_t2_dn7 = ({ let limited_exp_arg = assign25030_e34245; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn7));
        locals.var_t2_dn8 = ({ let limited_exp_arg = assign25030_e34245; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn8));
        locals.var_t2_dn9 = ({ let limited_exp_arg = assign25030_e34245; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn9));
        locals.var_t2_dn10 = ({ let limited_exp_arg = assign25030_e34245; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn10));
        locals.var_t2_dn11 = ({ let limited_exp_arg = assign25030_e34245; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn11));
        locals.var_t2_dn12 = ({ let limited_exp_arg = assign25030_e34245; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn12));
        locals.var_t2_dn13 = ({ let limited_exp_arg = assign25030_e34245; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn13));
        locals.var_t2_dn14 = ({ let limited_exp_arg = assign25030_e34245; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn14));
        locals.var_t2_rv = 0.0;

        let assign25040_e34249: f64 = (locals.var_oneminusxpart * locals.var_isbd);
        let assign25040_e34251: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign25040_e34253: f64 = (assign25040_e34251 + locals.var_xexpbvd);
        let assign25040_e34255: f64 = (assign25040_e34253 - 1.0);
        let assign25040_e34258: f64 = (p.p734 * locals.var_t2);
        let assign25040_e34259: f64 = (assign25040_e34255 - assign25040_e34258);
        let assign25040_e34260: f64 = (assign25040_e34249 * assign25040_e34259);
        locals.var_t4 = assign25040_e34260;
        locals.var_t4_dn0 = (((locals.var_oneminusxpart * locals.var_isbd_dn0) * assign25040_e34259) + (assign25040_e34249 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) - (p.p734 * locals.var_t2_dn0))));
        locals.var_t4_dn2 = (((locals.var_oneminusxpart * locals.var_isbd_dn2) * assign25040_e34259) + (assign25040_e34249 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) - (p.p734 * locals.var_t2_dn2))));
        locals.var_t4_dn3 = (((locals.var_oneminusxpart * locals.var_isbd_dn3) * assign25040_e34259) + (assign25040_e34249 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3) - (p.p734 * locals.var_t2_dn3))));
        locals.var_t4_dn4 = (((locals.var_oneminusxpart * locals.var_isbd_dn4) * assign25040_e34259) + (assign25040_e34249 * ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + locals.var_xexpbvd_dn4) - (p.p734 * locals.var_t2_dn4))));
        locals.var_t4_dn5 = (((locals.var_oneminusxpart * locals.var_isbd_dn5) * assign25040_e34259) + (assign25040_e34249 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) - (p.p734 * locals.var_t2_dn5))));
        locals.var_t4_dn6 = (((locals.var_oneminusxpart * locals.var_isbd_dn6) * assign25040_e34259) + (assign25040_e34249 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) - (p.p734 * locals.var_t2_dn6))));
        locals.var_t4_dn7 = (((locals.var_oneminusxpart * locals.var_isbd_dn7) * assign25040_e34259) + (assign25040_e34249 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) - (p.p734 * locals.var_t2_dn7))));
        locals.var_t4_dn8 = (((locals.var_oneminusxpart * locals.var_isbd_dn8) * assign25040_e34259) + (assign25040_e34249 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) - (p.p734 * locals.var_t2_dn8))));
        locals.var_t4_dn9 = (((locals.var_oneminusxpart * locals.var_isbd_dn9) * assign25040_e34259) + (assign25040_e34249 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) - (p.p734 * locals.var_t2_dn9))));
        locals.var_t4_dn10 = (((locals.var_oneminusxpart * locals.var_isbd_dn10) * assign25040_e34259) + (assign25040_e34249 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) - (p.p734 * locals.var_t2_dn10))));
        locals.var_t4_dn11 = (((locals.var_oneminusxpart * locals.var_isbd_dn11) * assign25040_e34259) + (assign25040_e34249 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) - (p.p734 * locals.var_t2_dn11))));
        locals.var_t4_dn12 = (((locals.var_oneminusxpart * locals.var_isbd_dn12) * assign25040_e34259) + (assign25040_e34249 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn12) - (p.p734 * locals.var_t2_dn12))));
        locals.var_t4_dn13 = (((locals.var_oneminusxpart * locals.var_isbd_dn13) * assign25040_e34259) + (assign25040_e34249 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) - (p.p734 * locals.var_t2_dn13))));
        locals.var_t4_dn14 = (((locals.var_oneminusxpart * locals.var_isbd_dn14) * assign25040_e34259) + (assign25040_e34249 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) - (p.p734 * locals.var_t2_dn14))));
        locals.var_t4_rv = 0.0;

        let assign25050_e34266: f64 = (locals.var_vbd_jct - locals.var_vjdmfwd);
        let assign25050_e34267: f64 = (locals.var_dslpfwd * assign25050_e34266);
        let assign25050_e34268: f64 = (locals.var_ivjdmfwd + assign25050_e34267);
        let assign25050_e34269: f64 = (locals.var_oneminusxpart * assign25050_e34268);
        locals.var_t5 = assign25050_e34269;
        locals.var_t5_dn0 = (locals.var_oneminusxpart * (locals.var_ivjdmfwd_dn0 + ((locals.var_dslpfwd_dn0 * assign25050_e34266) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn0)))));
        locals.var_t5_dn2 = (locals.var_oneminusxpart * (locals.var_ivjdmfwd_dn2 + ((locals.var_dslpfwd_dn2 * assign25050_e34266) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn2)))));
        locals.var_t5_dn3 = (locals.var_oneminusxpart * (locals.var_ivjdmfwd_dn3 + ((locals.var_dslpfwd_dn3 * assign25050_e34266) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn3)))));
        locals.var_t5_dn4 = (locals.var_oneminusxpart * (locals.var_ivjdmfwd_dn4 + ((locals.var_dslpfwd_dn4 * assign25050_e34266) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn4)))));
        locals.var_t5_dn5 = (locals.var_oneminusxpart * (locals.var_ivjdmfwd_dn5 + ((locals.var_dslpfwd_dn5 * assign25050_e34266) + (locals.var_dslpfwd * (locals.var_vbd_jct_dn5 - locals.var_vjdmfwd_dn5)))));
        locals.var_t5_dn6 = (locals.var_oneminusxpart * (locals.var_ivjdmfwd_dn6 + ((locals.var_dslpfwd_dn6 * assign25050_e34266) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn6)))));
        locals.var_t5_dn7 = (locals.var_oneminusxpart * (locals.var_ivjdmfwd_dn7 + ((locals.var_dslpfwd_dn7 * assign25050_e34266) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn7)))));
        locals.var_t5_dn8 = (locals.var_oneminusxpart * (locals.var_ivjdmfwd_dn8 + ((locals.var_dslpfwd_dn8 * assign25050_e34266) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn8)))));
        locals.var_t5_dn9 = (locals.var_oneminusxpart * (locals.var_ivjdmfwd_dn9 + ((locals.var_dslpfwd_dn9 * assign25050_e34266) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn9)))));
        locals.var_t5_dn10 = (locals.var_oneminusxpart * (locals.var_ivjdmfwd_dn10 + ((locals.var_dslpfwd_dn10 * assign25050_e34266) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn10)))));
        locals.var_t5_dn11 = (locals.var_oneminusxpart * (locals.var_ivjdmfwd_dn11 + ((locals.var_dslpfwd_dn11 * assign25050_e34266) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn11)))));
        locals.var_t5_dn12 = (locals.var_oneminusxpart * (locals.var_ivjdmfwd_dn12 + ((locals.var_dslpfwd_dn12 * assign25050_e34266) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn12)))));
        locals.var_t5_dn13 = (locals.var_oneminusxpart * (locals.var_ivjdmfwd_dn13 + ((locals.var_dslpfwd_dn13 * assign25050_e34266) + (locals.var_dslpfwd * (locals.var_vbd_jct_dn13 - locals.var_vjdmfwd_dn13)))));
        locals.var_t5_dn14 = (locals.var_oneminusxpart * (locals.var_ivjdmfwd_dn14 + ((locals.var_dslpfwd_dn14 * assign25050_e34266) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn14)))));
        locals.var_t5_rv = 0.0;

        let assign25060_e34272: f64 = if locals.var_isbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard621 = assign25060_e34272;
        locals.var_guard621_rv = 0.0;

        let assign25070_e34275: f64 = if locals.var_oneminusxpart > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard622 = assign25070_e34275;
        locals.var_guard622_rv = 0.0;

        let (assign25080_e34305, assign25080_e34305_d_n0, assign25080_e34305_d_n2, assign25080_e34305_d_n3, assign25080_e34305_d_n4, assign25080_e34305_d_n5, assign25080_e34305_d_n6, assign25080_e34305_d_n7, assign25080_e34305_d_n8, assign25080_e34305_d_n9, assign25080_e34305_d_n10, assign25080_e34305_d_n11, assign25080_e34305_d_n12, assign25080_e34305_d_n13, assign25080_e34305_d_n14,) = {
    if ((locals.var_guard621 != 0.0) && (locals.var_guard622 != 0.0)) {
        let assign25080_e34281: f64 = (locals.var_t3 / 2.0);
        let assign25080_e34285: f64 = (locals.var_vbd_jct - locals.var_vjdmrev);
        let assign25080_e34287: f64 = (assign25080_e34285 / locals.var_nvtmd);
        let assign25080_e34288: f64 = (assign25080_e34287).tanh();
        let assign25080_e34289: f64 = (1.0 - assign25080_e34288);
        let assign25080_e34290: f64 = (assign25080_e34281 * assign25080_e34289);
        let assign25080_e34293: f64 = (locals.var_t4 / 2.0);
        let assign25080_e34297: f64 = (locals.var_vbd_jct - locals.var_vjdmrev);
        let assign25080_e34299: f64 = (assign25080_e34297 / locals.var_nvtmd);
        let assign25080_e34300: f64 = (assign25080_e34299).tanh();
        let assign25080_e34301: f64 = (1.0 + assign25080_e34300);
        let assign25080_e34302: f64 = (assign25080_e34293 * assign25080_e34301);
        let assign25080_e34303: f64 = (assign25080_e34290 + assign25080_e34302);
        (assign25080_e34303, ((((locals.var_t3_dn0 / 2.0) * assign25080_e34289) + (assign25080_e34281 * (-(((-locals.var_vjdmrev_dn0) / locals.var_nvtmd) / ((assign25080_e34287).cosh() * (assign25080_e34287).cosh()))))) + (((locals.var_t4_dn0 / 2.0) * assign25080_e34301) + (assign25080_e34293 * (((-locals.var_vjdmrev_dn0) / locals.var_nvtmd) / ((assign25080_e34299).cosh() * (assign25080_e34299).cosh()))))), ((((locals.var_t3_dn2 / 2.0) * assign25080_e34289) + (assign25080_e34281 * (-(((-locals.var_vjdmrev_dn2) / locals.var_nvtmd) / ((assign25080_e34287).cosh() * (assign25080_e34287).cosh()))))) + (((locals.var_t4_dn2 / 2.0) * assign25080_e34301) + (assign25080_e34293 * (((-locals.var_vjdmrev_dn2) / locals.var_nvtmd) / ((assign25080_e34299).cosh() * (assign25080_e34299).cosh()))))), ((((locals.var_t3_dn3 / 2.0) * assign25080_e34289) + (assign25080_e34281 * (-(((-locals.var_vjdmrev_dn3) / locals.var_nvtmd) / ((assign25080_e34287).cosh() * (assign25080_e34287).cosh()))))) + (((locals.var_t4_dn3 / 2.0) * assign25080_e34301) + (assign25080_e34293 * (((-locals.var_vjdmrev_dn3) / locals.var_nvtmd) / ((assign25080_e34299).cosh() * (assign25080_e34299).cosh()))))), ((((locals.var_t3_dn4 / 2.0) * assign25080_e34289) + (assign25080_e34281 * (-(((((-locals.var_vjdmrev_dn4) * locals.var_nvtmd) - (assign25080_e34285 * locals.var_nvtmd_dn4)) / (locals.var_nvtmd * locals.var_nvtmd)) / ((assign25080_e34287).cosh() * (assign25080_e34287).cosh()))))) + (((locals.var_t4_dn4 / 2.0) * assign25080_e34301) + (assign25080_e34293 * (((((-locals.var_vjdmrev_dn4) * locals.var_nvtmd) - (assign25080_e34297 * locals.var_nvtmd_dn4)) / (locals.var_nvtmd * locals.var_nvtmd)) / ((assign25080_e34299).cosh() * (assign25080_e34299).cosh()))))), ((((locals.var_t3_dn5 / 2.0) * assign25080_e34289) + (assign25080_e34281 * (-(((locals.var_vbd_jct_dn5 - locals.var_vjdmrev_dn5) / locals.var_nvtmd) / ((assign25080_e34287).cosh() * (assign25080_e34287).cosh()))))) + (((locals.var_t4_dn5 / 2.0) * assign25080_e34301) + (assign25080_e34293 * (((locals.var_vbd_jct_dn5 - locals.var_vjdmrev_dn5) / locals.var_nvtmd) / ((assign25080_e34299).cosh() * (assign25080_e34299).cosh()))))), ((((locals.var_t3_dn6 / 2.0) * assign25080_e34289) + (assign25080_e34281 * (-(((-locals.var_vjdmrev_dn6) / locals.var_nvtmd) / ((assign25080_e34287).cosh() * (assign25080_e34287).cosh()))))) + (((locals.var_t4_dn6 / 2.0) * assign25080_e34301) + (assign25080_e34293 * (((-locals.var_vjdmrev_dn6) / locals.var_nvtmd) / ((assign25080_e34299).cosh() * (assign25080_e34299).cosh()))))), ((((locals.var_t3_dn7 / 2.0) * assign25080_e34289) + (assign25080_e34281 * (-(((-locals.var_vjdmrev_dn7) / locals.var_nvtmd) / ((assign25080_e34287).cosh() * (assign25080_e34287).cosh()))))) + (((locals.var_t4_dn7 / 2.0) * assign25080_e34301) + (assign25080_e34293 * (((-locals.var_vjdmrev_dn7) / locals.var_nvtmd) / ((assign25080_e34299).cosh() * (assign25080_e34299).cosh()))))), ((((locals.var_t3_dn8 / 2.0) * assign25080_e34289) + (assign25080_e34281 * (-(((-locals.var_vjdmrev_dn8) / locals.var_nvtmd) / ((assign25080_e34287).cosh() * (assign25080_e34287).cosh()))))) + (((locals.var_t4_dn8 / 2.0) * assign25080_e34301) + (assign25080_e34293 * (((-locals.var_vjdmrev_dn8) / locals.var_nvtmd) / ((assign25080_e34299).cosh() * (assign25080_e34299).cosh()))))), ((((locals.var_t3_dn9 / 2.0) * assign25080_e34289) + (assign25080_e34281 * (-(((-locals.var_vjdmrev_dn9) / locals.var_nvtmd) / ((assign25080_e34287).cosh() * (assign25080_e34287).cosh()))))) + (((locals.var_t4_dn9 / 2.0) * assign25080_e34301) + (assign25080_e34293 * (((-locals.var_vjdmrev_dn9) / locals.var_nvtmd) / ((assign25080_e34299).cosh() * (assign25080_e34299).cosh()))))), ((((locals.var_t3_dn10 / 2.0) * assign25080_e34289) + (assign25080_e34281 * (-(((-locals.var_vjdmrev_dn10) / locals.var_nvtmd) / ((assign25080_e34287).cosh() * (assign25080_e34287).cosh()))))) + (((locals.var_t4_dn10 / 2.0) * assign25080_e34301) + (assign25080_e34293 * (((-locals.var_vjdmrev_dn10) / locals.var_nvtmd) / ((assign25080_e34299).cosh() * (assign25080_e34299).cosh()))))), ((((locals.var_t3_dn11 / 2.0) * assign25080_e34289) + (assign25080_e34281 * (-(((-locals.var_vjdmrev_dn11) / locals.var_nvtmd) / ((assign25080_e34287).cosh() * (assign25080_e34287).cosh()))))) + (((locals.var_t4_dn11 / 2.0) * assign25080_e34301) + (assign25080_e34293 * (((-locals.var_vjdmrev_dn11) / locals.var_nvtmd) / ((assign25080_e34299).cosh() * (assign25080_e34299).cosh()))))), ((((locals.var_t3_dn12 / 2.0) * assign25080_e34289) + (assign25080_e34281 * (-(((-locals.var_vjdmrev_dn12) / locals.var_nvtmd) / ((assign25080_e34287).cosh() * (assign25080_e34287).cosh()))))) + (((locals.var_t4_dn12 / 2.0) * assign25080_e34301) + (assign25080_e34293 * (((-locals.var_vjdmrev_dn12) / locals.var_nvtmd) / ((assign25080_e34299).cosh() * (assign25080_e34299).cosh()))))), ((((locals.var_t3_dn13 / 2.0) * assign25080_e34289) + (assign25080_e34281 * (-(((locals.var_vbd_jct_dn13 - locals.var_vjdmrev_dn13) / locals.var_nvtmd) / ((assign25080_e34287).cosh() * (assign25080_e34287).cosh()))))) + (((locals.var_t4_dn13 / 2.0) * assign25080_e34301) + (assign25080_e34293 * (((locals.var_vbd_jct_dn13 - locals.var_vjdmrev_dn13) / locals.var_nvtmd) / ((assign25080_e34299).cosh() * (assign25080_e34299).cosh()))))), ((((locals.var_t3_dn14 / 2.0) * assign25080_e34289) + (assign25080_e34281 * (-(((-locals.var_vjdmrev_dn14) / locals.var_nvtmd) / ((assign25080_e34287).cosh() * (assign25080_e34287).cosh()))))) + (((locals.var_t4_dn14 / 2.0) * assign25080_e34301) + (assign25080_e34293 * (((-locals.var_vjdmrev_dn14) / locals.var_nvtmd) / ((assign25080_e34299).cosh() * (assign25080_e34299).cosh()))))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign25080_e34305;
        locals.var_t6_dn0 = assign25080_e34305_d_n0;
        locals.var_t6_dn2 = assign25080_e34305_d_n2;
        locals.var_t6_dn3 = assign25080_e34305_d_n3;
        locals.var_t6_dn4 = assign25080_e34305_d_n4;
        locals.var_t6_dn5 = assign25080_e34305_d_n5;
        locals.var_t6_dn6 = assign25080_e34305_d_n6;
        locals.var_t6_dn7 = assign25080_e34305_d_n7;
        locals.var_t6_dn8 = assign25080_e34305_d_n8;
        locals.var_t6_dn9 = assign25080_e34305_d_n9;
        locals.var_t6_dn10 = assign25080_e34305_d_n10;
        locals.var_t6_dn11 = assign25080_e34305_d_n11;
        locals.var_t6_dn12 = assign25080_e34305_d_n12;
        locals.var_t6_dn13 = assign25080_e34305_d_n13;
        locals.var_t6_dn14 = assign25080_e34305_d_n14;
        locals.var_t6_rv = 0.0;

        let assign25110_e34349: f64 = if ((p.p1128 > 0.0) && (p.p1097 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard623 = assign25110_e34349;
        locals.var_guard623_rv = 0.0;

        let (assign25120_e34357, assign25120_e34357_d_n0, assign25120_e34357_d_n2, assign25120_e34357_d_n3, assign25120_e34357_d_n4, assign25120_e34357_d_n5, assign25120_e34357_d_n6, assign25120_e34357_d_n7, assign25120_e34357_d_n8, assign25120_e34357_d_n9, assign25120_e34357_d_n10, assign25120_e34357_d_n11, assign25120_e34357_d_n12, assign25120_e34357_d_n13, assign25120_e34357_d_n14,) = {
    if ((locals.var_guard621 != 0.0) && (locals.var_guard623 != 0.0)) {
        let assign25120_e34355: f64 = (locals.var_vbd_ext / locals.var_nvtmd);
        (assign25120_e34355, 0.0, 0.0, 0.0, (-((locals.var_vbd_ext * locals.var_nvtmd_dn4) / (locals.var_nvtmd * locals.var_nvtmd))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_vbd_ext_dn13 / locals.var_nvtmd), (locals.var_vbd_ext_dn14 / locals.var_nvtmd),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign25120_e34357;
        locals.var_t0_dn0 = assign25120_e34357_d_n0;
        locals.var_t0_dn2 = assign25120_e34357_d_n2;
        locals.var_t0_dn3 = assign25120_e34357_d_n3;
        locals.var_t0_dn4 = assign25120_e34357_d_n4;
        locals.var_t0_dn5 = assign25120_e34357_d_n5;
        locals.var_t0_dn6 = assign25120_e34357_d_n6;
        locals.var_t0_dn7 = assign25120_e34357_d_n7;
        locals.var_t0_dn8 = assign25120_e34357_d_n8;
        locals.var_t0_dn9 = assign25120_e34357_d_n9;
        locals.var_t0_dn10 = assign25120_e34357_d_n10;
        locals.var_t0_dn11 = assign25120_e34357_d_n11;
        locals.var_t0_dn12 = assign25120_e34357_d_n12;
        locals.var_t0_dn13 = assign25120_e34357_d_n13;
        locals.var_t0_dn14 = assign25120_e34357_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign25130_e34366, assign25130_e34366_d_n0, assign25130_e34366_d_n2, assign25130_e34366_d_n3, assign25130_e34366_d_n4, assign25130_e34366_d_n5, assign25130_e34366_d_n6, assign25130_e34366_d_n7, assign25130_e34366_d_n8, assign25130_e34366_d_n9, assign25130_e34366_d_n10, assign25130_e34366_d_n11, assign25130_e34366_d_n12, assign25130_e34366_d_n13, assign25130_e34366_d_n14,) = {
    if ((locals.var_guard621 != 0.0) && (locals.var_guard623 != 0.0)) {
        let assign25130_e34362: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign25130_e34364: f64 = (assign25130_e34362 - 1.0);
        (assign25130_e34364, ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn12), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13), ({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign25130_e34366;
        locals.var_t1_dn0 = assign25130_e34366_d_n0;
        locals.var_t1_dn2 = assign25130_e34366_d_n2;
        locals.var_t1_dn3 = assign25130_e34366_d_n3;
        locals.var_t1_dn4 = assign25130_e34366_d_n4;
        locals.var_t1_dn5 = assign25130_e34366_d_n5;
        locals.var_t1_dn6 = assign25130_e34366_d_n6;
        locals.var_t1_dn7 = assign25130_e34366_d_n7;
        locals.var_t1_dn8 = assign25130_e34366_d_n8;
        locals.var_t1_dn9 = assign25130_e34366_d_n9;
        locals.var_t1_dn10 = assign25130_e34366_d_n10;
        locals.var_t1_dn11 = assign25130_e34366_d_n11;
        locals.var_t1_dn12 = assign25130_e34366_d_n12;
        locals.var_t1_dn13 = assign25130_e34366_d_n13;
        locals.var_t1_dn14 = assign25130_e34366_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign25140_e34378, assign25140_e34378_d_n0, assign25140_e34378_d_n2, assign25140_e34378_d_n3, assign25140_e34378_d_n4, assign25140_e34378_d_n5, assign25140_e34378_d_n6, assign25140_e34378_d_n7, assign25140_e34378_d_n8, assign25140_e34378_d_n9, assign25140_e34378_d_n10, assign25140_e34378_d_n11, assign25140_e34378_d_n12, assign25140_e34378_d_n13, assign25140_e34378_d_n14,) = {
    if ((locals.var_guard621 != 0.0) && (locals.var_guard623 != 0.0)) {
        let assign25140_e34374: f64 = (locals.var_vbd_ext - locals.var_vjdmrev);
        let assign25140_e34375: f64 = (locals.var_dslprev * assign25140_e34374);
        let assign25140_e34376: f64 = (locals.var_ivjdmrev + assign25140_e34375);
        (assign25140_e34376, (locals.var_ivjdmrev_dn0 + ((locals.var_dslprev_dn0 * assign25140_e34374) + (locals.var_dslprev * (-locals.var_vjdmrev_dn0)))), (locals.var_ivjdmrev_dn2 + ((locals.var_dslprev_dn2 * assign25140_e34374) + (locals.var_dslprev * (-locals.var_vjdmrev_dn2)))), (locals.var_ivjdmrev_dn3 + ((locals.var_dslprev_dn3 * assign25140_e34374) + (locals.var_dslprev * (-locals.var_vjdmrev_dn3)))), (locals.var_ivjdmrev_dn4 + ((locals.var_dslprev_dn4 * assign25140_e34374) + (locals.var_dslprev * (-locals.var_vjdmrev_dn4)))), (locals.var_ivjdmrev_dn5 + ((locals.var_dslprev_dn5 * assign25140_e34374) + (locals.var_dslprev * (-locals.var_vjdmrev_dn5)))), (locals.var_ivjdmrev_dn6 + ((locals.var_dslprev_dn6 * assign25140_e34374) + (locals.var_dslprev * (-locals.var_vjdmrev_dn6)))), (locals.var_ivjdmrev_dn7 + ((locals.var_dslprev_dn7 * assign25140_e34374) + (locals.var_dslprev * (-locals.var_vjdmrev_dn7)))), (locals.var_ivjdmrev_dn8 + ((locals.var_dslprev_dn8 * assign25140_e34374) + (locals.var_dslprev * (-locals.var_vjdmrev_dn8)))), (locals.var_ivjdmrev_dn9 + ((locals.var_dslprev_dn9 * assign25140_e34374) + (locals.var_dslprev * (-locals.var_vjdmrev_dn9)))), (locals.var_ivjdmrev_dn10 + ((locals.var_dslprev_dn10 * assign25140_e34374) + (locals.var_dslprev * (-locals.var_vjdmrev_dn10)))), (locals.var_ivjdmrev_dn11 + ((locals.var_dslprev_dn11 * assign25140_e34374) + (locals.var_dslprev * (-locals.var_vjdmrev_dn11)))), (locals.var_ivjdmrev_dn12 + ((locals.var_dslprev_dn12 * assign25140_e34374) + (locals.var_dslprev * (-locals.var_vjdmrev_dn12)))), (locals.var_ivjdmrev_dn13 + ((locals.var_dslprev_dn13 * assign25140_e34374) + (locals.var_dslprev * (locals.var_vbd_ext_dn13 - locals.var_vjdmrev_dn13)))), (locals.var_ivjdmrev_dn14 + ((locals.var_dslprev_dn14 * assign25140_e34374) + (locals.var_dslprev * (locals.var_vbd_ext_dn14 - locals.var_vjdmrev_dn14)))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign25140_e34378;
        locals.var_t2_dn0 = assign25140_e34378_d_n0;
        locals.var_t2_dn2 = assign25140_e34378_d_n2;
        locals.var_t2_dn3 = assign25140_e34378_d_n3;
        locals.var_t2_dn4 = assign25140_e34378_d_n4;
        locals.var_t2_dn5 = assign25140_e34378_d_n5;
        locals.var_t2_dn6 = assign25140_e34378_d_n6;
        locals.var_t2_dn7 = assign25140_e34378_d_n7;
        locals.var_t2_dn8 = assign25140_e34378_d_n8;
        locals.var_t2_dn9 = assign25140_e34378_d_n9;
        locals.var_t2_dn10 = assign25140_e34378_d_n10;
        locals.var_t2_dn11 = assign25140_e34378_d_n11;
        locals.var_t2_dn12 = assign25140_e34378_d_n12;
        locals.var_t2_dn13 = assign25140_e34378_d_n13;
        locals.var_t2_dn14 = assign25140_e34378_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign25150_e34388, assign25150_e34388_d_n0, assign25150_e34388_d_n2, assign25150_e34388_d_n3, assign25150_e34388_d_n4, assign25150_e34388_d_n5, assign25150_e34388_d_n6, assign25150_e34388_d_n7, assign25150_e34388_d_n8, assign25150_e34388_d_n9, assign25150_e34388_d_n10, assign25150_e34388_d_n11, assign25150_e34388_d_n12, assign25150_e34388_d_n13, assign25150_e34388_d_n14,) = {
    if ((locals.var_guard621 != 0.0) && (locals.var_guard623 != 0.0)) {
        let assign25150_e34384: f64 = (p.p1128 * locals.var_t1);
        let assign25150_e34386: f64 = (assign25150_e34384 * locals.var_t2);
        (assign25150_e34386, (((p.p1128 * locals.var_t1_dn0) * locals.var_t2) + (assign25150_e34384 * locals.var_t2_dn0)), (((p.p1128 * locals.var_t1_dn2) * locals.var_t2) + (assign25150_e34384 * locals.var_t2_dn2)), (((p.p1128 * locals.var_t1_dn3) * locals.var_t2) + (assign25150_e34384 * locals.var_t2_dn3)), (((p.p1128 * locals.var_t1_dn4) * locals.var_t2) + (assign25150_e34384 * locals.var_t2_dn4)), (((p.p1128 * locals.var_t1_dn5) * locals.var_t2) + (assign25150_e34384 * locals.var_t2_dn5)), (((p.p1128 * locals.var_t1_dn6) * locals.var_t2) + (assign25150_e34384 * locals.var_t2_dn6)), (((p.p1128 * locals.var_t1_dn7) * locals.var_t2) + (assign25150_e34384 * locals.var_t2_dn7)), (((p.p1128 * locals.var_t1_dn8) * locals.var_t2) + (assign25150_e34384 * locals.var_t2_dn8)), (((p.p1128 * locals.var_t1_dn9) * locals.var_t2) + (assign25150_e34384 * locals.var_t2_dn9)), (((p.p1128 * locals.var_t1_dn10) * locals.var_t2) + (assign25150_e34384 * locals.var_t2_dn10)), (((p.p1128 * locals.var_t1_dn11) * locals.var_t2) + (assign25150_e34384 * locals.var_t2_dn11)), (((p.p1128 * locals.var_t1_dn12) * locals.var_t2) + (assign25150_e34384 * locals.var_t2_dn12)), (((p.p1128 * locals.var_t1_dn13) * locals.var_t2) + (assign25150_e34384 * locals.var_t2_dn13)), (((p.p1128 * locals.var_t1_dn14) * locals.var_t2) + (assign25150_e34384 * locals.var_t2_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign25150_e34388;
        locals.var_t3_dn0 = assign25150_e34388_d_n0;
        locals.var_t3_dn2 = assign25150_e34388_d_n2;
        locals.var_t3_dn3 = assign25150_e34388_d_n3;
        locals.var_t3_dn4 = assign25150_e34388_d_n4;
        locals.var_t3_dn5 = assign25150_e34388_d_n5;
        locals.var_t3_dn6 = assign25150_e34388_d_n6;
        locals.var_t3_dn7 = assign25150_e34388_d_n7;
        locals.var_t3_dn8 = assign25150_e34388_d_n8;
        locals.var_t3_dn9 = assign25150_e34388_d_n9;
        locals.var_t3_dn10 = assign25150_e34388_d_n10;
        locals.var_t3_dn11 = assign25150_e34388_d_n11;
        locals.var_t3_dn12 = assign25150_e34388_d_n12;
        locals.var_t3_dn13 = assign25150_e34388_d_n13;
        locals.var_t3_dn14 = assign25150_e34388_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign25160_e34398, assign25160_e34398_d_n0, assign25160_e34398_d_n2, assign25160_e34398_d_n3, assign25160_e34398_d_n4, assign25160_e34398_d_n5, assign25160_e34398_d_n6, assign25160_e34398_d_n7, assign25160_e34398_d_n8, assign25160_e34398_d_n9, assign25160_e34398_d_n10, assign25160_e34398_d_n11, assign25160_e34398_d_n12, assign25160_e34398_d_n13, assign25160_e34398_d_n14,) = {
    if ((locals.var_guard621 != 0.0) && (locals.var_guard623 != 0.0)) {
        let assign25160_e34394: f64 = (p.p732 + locals.var_vbd_ext);
        let assign25160_e34396: f64 = (assign25160_e34394 / locals.var_nvtmd);
        (assign25160_e34396, 0.0, 0.0, 0.0, (-((assign25160_e34394 * locals.var_nvtmd_dn4) / (locals.var_nvtmd * locals.var_nvtmd))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_vbd_ext_dn13 / locals.var_nvtmd), (locals.var_vbd_ext_dn14 / locals.var_nvtmd),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign25160_e34398;
        locals.var_t1_dn0 = assign25160_e34398_d_n0;
        locals.var_t1_dn2 = assign25160_e34398_d_n2;
        locals.var_t1_dn3 = assign25160_e34398_d_n3;
        locals.var_t1_dn4 = assign25160_e34398_d_n4;
        locals.var_t1_dn5 = assign25160_e34398_d_n5;
        locals.var_t1_dn6 = assign25160_e34398_d_n6;
        locals.var_t1_dn7 = assign25160_e34398_d_n7;
        locals.var_t1_dn8 = assign25160_e34398_d_n8;
        locals.var_t1_dn9 = assign25160_e34398_d_n9;
        locals.var_t1_dn10 = assign25160_e34398_d_n10;
        locals.var_t1_dn11 = assign25160_e34398_d_n11;
        locals.var_t1_dn12 = assign25160_e34398_d_n12;
        locals.var_t1_dn13 = assign25160_e34398_d_n13;
        locals.var_t1_dn14 = assign25160_e34398_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign25170_e34406, assign25170_e34406_d_n0, assign25170_e34406_d_n2, assign25170_e34406_d_n3, assign25170_e34406_d_n4, assign25170_e34406_d_n5, assign25170_e34406_d_n6, assign25170_e34406_d_n7, assign25170_e34406_d_n8, assign25170_e34406_d_n9, assign25170_e34406_d_n10, assign25170_e34406_d_n11, assign25170_e34406_d_n12, assign25170_e34406_d_n13, assign25170_e34406_d_n14,) = {
    if ((locals.var_guard621 != 0.0) && (locals.var_guard623 != 0.0)) {
        let assign25170_e34403: f64 = (-locals.var_t1);
        let assign25170_e34404: f64 = { let limited_exp_arg = assign25170_e34403; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25170_e34404, ({ let limited_exp_arg = assign25170_e34403; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn0)), ({ let limited_exp_arg = assign25170_e34403; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn2)), ({ let limited_exp_arg = assign25170_e34403; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn3)), ({ let limited_exp_arg = assign25170_e34403; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn4)), ({ let limited_exp_arg = assign25170_e34403; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn5)), ({ let limited_exp_arg = assign25170_e34403; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn6)), ({ let limited_exp_arg = assign25170_e34403; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn7)), ({ let limited_exp_arg = assign25170_e34403; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn8)), ({ let limited_exp_arg = assign25170_e34403; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn9)), ({ let limited_exp_arg = assign25170_e34403; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn10)), ({ let limited_exp_arg = assign25170_e34403; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn11)), ({ let limited_exp_arg = assign25170_e34403; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn12)), ({ let limited_exp_arg = assign25170_e34403; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn13)), ({ let limited_exp_arg = assign25170_e34403; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_t1_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign25170_e34406;
        locals.var_t2_dn0 = assign25170_e34406_d_n0;
        locals.var_t2_dn2 = assign25170_e34406_d_n2;
        locals.var_t2_dn3 = assign25170_e34406_d_n3;
        locals.var_t2_dn4 = assign25170_e34406_d_n4;
        locals.var_t2_dn5 = assign25170_e34406_d_n5;
        locals.var_t2_dn6 = assign25170_e34406_d_n6;
        locals.var_t2_dn7 = assign25170_e34406_d_n7;
        locals.var_t2_dn8 = assign25170_e34406_d_n8;
        locals.var_t2_dn9 = assign25170_e34406_d_n9;
        locals.var_t2_dn10 = assign25170_e34406_d_n10;
        locals.var_t2_dn11 = assign25170_e34406_d_n11;
        locals.var_t2_dn12 = assign25170_e34406_d_n12;
        locals.var_t2_dn13 = assign25170_e34406_d_n13;
        locals.var_t2_dn14 = assign25170_e34406_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign25180_e34425, assign25180_e34425_d_n0, assign25180_e34425_d_n2, assign25180_e34425_d_n3, assign25180_e34425_d_n4, assign25180_e34425_d_n5, assign25180_e34425_d_n6, assign25180_e34425_d_n7, assign25180_e34425_d_n8, assign25180_e34425_d_n9, assign25180_e34425_d_n10, assign25180_e34425_d_n11, assign25180_e34425_d_n12, assign25180_e34425_d_n13, assign25180_e34425_d_n14,) = {
    if ((locals.var_guard621 != 0.0) && (locals.var_guard623 != 0.0)) {
        let assign25180_e34412: f64 = (p.p1128 * locals.var_isbd);
        let assign25180_e34414: f64 = { let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign25180_e34416: f64 = (assign25180_e34414 + locals.var_xexpbvd);
        let assign25180_e34418: f64 = (assign25180_e34416 - 1.0);
        let assign25180_e34421: f64 = (p.p734 * locals.var_t2);
        let assign25180_e34422: f64 = (assign25180_e34418 - assign25180_e34421);
        let assign25180_e34423: f64 = (assign25180_e34412 * assign25180_e34422);
        (assign25180_e34423, (((p.p1128 * locals.var_isbd_dn0) * assign25180_e34422) + (assign25180_e34412 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn0) - (p.p734 * locals.var_t2_dn0)))), (((p.p1128 * locals.var_isbd_dn2) * assign25180_e34422) + (assign25180_e34412 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn2) - (p.p734 * locals.var_t2_dn2)))), (((p.p1128 * locals.var_isbd_dn3) * assign25180_e34422) + (assign25180_e34412 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn3) - (p.p734 * locals.var_t2_dn3)))), (((p.p1128 * locals.var_isbd_dn4) * assign25180_e34422) + (assign25180_e34412 * ((({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn4) + locals.var_xexpbvd_dn4) - (p.p734 * locals.var_t2_dn4)))), (((p.p1128 * locals.var_isbd_dn5) * assign25180_e34422) + (assign25180_e34412 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn5) - (p.p734 * locals.var_t2_dn5)))), (((p.p1128 * locals.var_isbd_dn6) * assign25180_e34422) + (assign25180_e34412 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn6) - (p.p734 * locals.var_t2_dn6)))), (((p.p1128 * locals.var_isbd_dn7) * assign25180_e34422) + (assign25180_e34412 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn7) - (p.p734 * locals.var_t2_dn7)))), (((p.p1128 * locals.var_isbd_dn8) * assign25180_e34422) + (assign25180_e34412 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn8) - (p.p734 * locals.var_t2_dn8)))), (((p.p1128 * locals.var_isbd_dn9) * assign25180_e34422) + (assign25180_e34412 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn9) - (p.p734 * locals.var_t2_dn9)))), (((p.p1128 * locals.var_isbd_dn10) * assign25180_e34422) + (assign25180_e34412 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn10) - (p.p734 * locals.var_t2_dn10)))), (((p.p1128 * locals.var_isbd_dn11) * assign25180_e34422) + (assign25180_e34412 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn11) - (p.p734 * locals.var_t2_dn11)))), (((p.p1128 * locals.var_isbd_dn12) * assign25180_e34422) + (assign25180_e34412 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn12) - (p.p734 * locals.var_t2_dn12)))), (((p.p1128 * locals.var_isbd_dn13) * assign25180_e34422) + (assign25180_e34412 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn13) - (p.p734 * locals.var_t2_dn13)))), (((p.p1128 * locals.var_isbd_dn14) * assign25180_e34422) + (assign25180_e34412 * (({ let limited_exp_arg = locals.var_t0; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * locals.var_t0_dn14) - (p.p734 * locals.var_t2_dn14)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn12, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign25180_e34425;
        locals.var_t4_dn0 = assign25180_e34425_d_n0;
        locals.var_t4_dn2 = assign25180_e34425_d_n2;
        locals.var_t4_dn3 = assign25180_e34425_d_n3;
        locals.var_t4_dn4 = assign25180_e34425_d_n4;
        locals.var_t4_dn5 = assign25180_e34425_d_n5;
        locals.var_t4_dn6 = assign25180_e34425_d_n6;
        locals.var_t4_dn7 = assign25180_e34425_d_n7;
        locals.var_t4_dn8 = assign25180_e34425_d_n8;
        locals.var_t4_dn9 = assign25180_e34425_d_n9;
        locals.var_t4_dn10 = assign25180_e34425_d_n10;
        locals.var_t4_dn11 = assign25180_e34425_d_n11;
        locals.var_t4_dn12 = assign25180_e34425_d_n12;
        locals.var_t4_dn13 = assign25180_e34425_d_n13;
        locals.var_t4_dn14 = assign25180_e34425_d_n14;
        locals.var_t4_rv = 0.0;

        let (assign25190_e34439, assign25190_e34439_d_n0, assign25190_e34439_d_n2, assign25190_e34439_d_n3, assign25190_e34439_d_n4, assign25190_e34439_d_n5, assign25190_e34439_d_n6, assign25190_e34439_d_n7, assign25190_e34439_d_n8, assign25190_e34439_d_n9, assign25190_e34439_d_n10, assign25190_e34439_d_n11, assign25190_e34439_d_n12, assign25190_e34439_d_n13, assign25190_e34439_d_n14,) = {
    if ((locals.var_guard621 != 0.0) && (locals.var_guard623 != 0.0)) {
        let assign25190_e34434: f64 = (locals.var_vbd_ext - locals.var_vjdmfwd);
        let assign25190_e34435: f64 = (locals.var_dslpfwd * assign25190_e34434);
        let assign25190_e34436: f64 = (locals.var_ivjdmfwd + assign25190_e34435);
        let assign25190_e34437: f64 = (p.p1128 * assign25190_e34436);
        (assign25190_e34437, (p.p1128 * (locals.var_ivjdmfwd_dn0 + ((locals.var_dslpfwd_dn0 * assign25190_e34434) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn0))))), (p.p1128 * (locals.var_ivjdmfwd_dn2 + ((locals.var_dslpfwd_dn2 * assign25190_e34434) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn2))))), (p.p1128 * (locals.var_ivjdmfwd_dn3 + ((locals.var_dslpfwd_dn3 * assign25190_e34434) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn3))))), (p.p1128 * (locals.var_ivjdmfwd_dn4 + ((locals.var_dslpfwd_dn4 * assign25190_e34434) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn4))))), (p.p1128 * (locals.var_ivjdmfwd_dn5 + ((locals.var_dslpfwd_dn5 * assign25190_e34434) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn5))))), (p.p1128 * (locals.var_ivjdmfwd_dn6 + ((locals.var_dslpfwd_dn6 * assign25190_e34434) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn6))))), (p.p1128 * (locals.var_ivjdmfwd_dn7 + ((locals.var_dslpfwd_dn7 * assign25190_e34434) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn7))))), (p.p1128 * (locals.var_ivjdmfwd_dn8 + ((locals.var_dslpfwd_dn8 * assign25190_e34434) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn8))))), (p.p1128 * (locals.var_ivjdmfwd_dn9 + ((locals.var_dslpfwd_dn9 * assign25190_e34434) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn9))))), (p.p1128 * (locals.var_ivjdmfwd_dn10 + ((locals.var_dslpfwd_dn10 * assign25190_e34434) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn10))))), (p.p1128 * (locals.var_ivjdmfwd_dn11 + ((locals.var_dslpfwd_dn11 * assign25190_e34434) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn11))))), (p.p1128 * (locals.var_ivjdmfwd_dn12 + ((locals.var_dslpfwd_dn12 * assign25190_e34434) + (locals.var_dslpfwd * (-locals.var_vjdmfwd_dn12))))), (p.p1128 * (locals.var_ivjdmfwd_dn13 + ((locals.var_dslpfwd_dn13 * assign25190_e34434) + (locals.var_dslpfwd * (locals.var_vbd_ext_dn13 - locals.var_vjdmfwd_dn13))))), (p.p1128 * (locals.var_ivjdmfwd_dn14 + ((locals.var_dslpfwd_dn14 * assign25190_e34434) + (locals.var_dslpfwd * (locals.var_vbd_ext_dn14 - locals.var_vjdmfwd_dn14))))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn3, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn12, locals.var_t5_dn13, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign25190_e34439;
        locals.var_t5_dn0 = assign25190_e34439_d_n0;
        locals.var_t5_dn2 = assign25190_e34439_d_n2;
        locals.var_t5_dn3 = assign25190_e34439_d_n3;
        locals.var_t5_dn4 = assign25190_e34439_d_n4;
        locals.var_t5_dn5 = assign25190_e34439_d_n5;
        locals.var_t5_dn6 = assign25190_e34439_d_n6;
        locals.var_t5_dn7 = assign25190_e34439_d_n7;
        locals.var_t5_dn8 = assign25190_e34439_d_n8;
        locals.var_t5_dn9 = assign25190_e34439_d_n9;
        locals.var_t5_dn10 = assign25190_e34439_d_n10;
        locals.var_t5_dn11 = assign25190_e34439_d_n11;
        locals.var_t5_dn12 = assign25190_e34439_d_n12;
        locals.var_t5_dn13 = assign25190_e34439_d_n13;
        locals.var_t5_dn14 = assign25190_e34439_d_n14;
        locals.var_t5_rv = 0.0;

        let (assign25200_e34469, assign25200_e34469_d_n0, assign25200_e34469_d_n2, assign25200_e34469_d_n3, assign25200_e34469_d_n4, assign25200_e34469_d_n5, assign25200_e34469_d_n6, assign25200_e34469_d_n7, assign25200_e34469_d_n8, assign25200_e34469_d_n9, assign25200_e34469_d_n10, assign25200_e34469_d_n11, assign25200_e34469_d_n12, assign25200_e34469_d_n13, assign25200_e34469_d_n14,) = {
    if ((locals.var_guard621 != 0.0) && (locals.var_guard623 != 0.0)) {
        let assign25200_e34445: f64 = (locals.var_t3 / 2.0);
        let assign25200_e34449: f64 = (locals.var_vbd_ext - locals.var_vjdmrev);
        let assign25200_e34451: f64 = (assign25200_e34449 / locals.var_nvtmd);
        let assign25200_e34452: f64 = (assign25200_e34451).tanh();
        let assign25200_e34453: f64 = (1.0 - assign25200_e34452);
        let assign25200_e34454: f64 = (assign25200_e34445 * assign25200_e34453);
        let assign25200_e34457: f64 = (locals.var_t4 / 2.0);
        let assign25200_e34461: f64 = (locals.var_vbd_ext - locals.var_vjdmrev);
        let assign25200_e34463: f64 = (assign25200_e34461 / locals.var_nvtmd);
        let assign25200_e34464: f64 = (assign25200_e34463).tanh();
        let assign25200_e34465: f64 = (1.0 + assign25200_e34464);
        let assign25200_e34466: f64 = (assign25200_e34457 * assign25200_e34465);
        let assign25200_e34467: f64 = (assign25200_e34454 + assign25200_e34466);
        (assign25200_e34467, ((((locals.var_t3_dn0 / 2.0) * assign25200_e34453) + (assign25200_e34445 * (-(((-locals.var_vjdmrev_dn0) / locals.var_nvtmd) / ((assign25200_e34451).cosh() * (assign25200_e34451).cosh()))))) + (((locals.var_t4_dn0 / 2.0) * assign25200_e34465) + (assign25200_e34457 * (((-locals.var_vjdmrev_dn0) / locals.var_nvtmd) / ((assign25200_e34463).cosh() * (assign25200_e34463).cosh()))))), ((((locals.var_t3_dn2 / 2.0) * assign25200_e34453) + (assign25200_e34445 * (-(((-locals.var_vjdmrev_dn2) / locals.var_nvtmd) / ((assign25200_e34451).cosh() * (assign25200_e34451).cosh()))))) + (((locals.var_t4_dn2 / 2.0) * assign25200_e34465) + (assign25200_e34457 * (((-locals.var_vjdmrev_dn2) / locals.var_nvtmd) / ((assign25200_e34463).cosh() * (assign25200_e34463).cosh()))))), ((((locals.var_t3_dn3 / 2.0) * assign25200_e34453) + (assign25200_e34445 * (-(((-locals.var_vjdmrev_dn3) / locals.var_nvtmd) / ((assign25200_e34451).cosh() * (assign25200_e34451).cosh()))))) + (((locals.var_t4_dn3 / 2.0) * assign25200_e34465) + (assign25200_e34457 * (((-locals.var_vjdmrev_dn3) / locals.var_nvtmd) / ((assign25200_e34463).cosh() * (assign25200_e34463).cosh()))))), ((((locals.var_t3_dn4 / 2.0) * assign25200_e34453) + (assign25200_e34445 * (-(((((-locals.var_vjdmrev_dn4) * locals.var_nvtmd) - (assign25200_e34449 * locals.var_nvtmd_dn4)) / (locals.var_nvtmd * locals.var_nvtmd)) / ((assign25200_e34451).cosh() * (assign25200_e34451).cosh()))))) + (((locals.var_t4_dn4 / 2.0) * assign25200_e34465) + (assign25200_e34457 * (((((-locals.var_vjdmrev_dn4) * locals.var_nvtmd) - (assign25200_e34461 * locals.var_nvtmd_dn4)) / (locals.var_nvtmd * locals.var_nvtmd)) / ((assign25200_e34463).cosh() * (assign25200_e34463).cosh()))))), ((((locals.var_t3_dn5 / 2.0) * assign25200_e34453) + (assign25200_e34445 * (-(((-locals.var_vjdmrev_dn5) / locals.var_nvtmd) / ((assign25200_e34451).cosh() * (assign25200_e34451).cosh()))))) + (((locals.var_t4_dn5 / 2.0) * assign25200_e34465) + (assign25200_e34457 * (((-locals.var_vjdmrev_dn5) / locals.var_nvtmd) / ((assign25200_e34463).cosh() * (assign25200_e34463).cosh()))))), ((((locals.var_t3_dn6 / 2.0) * assign25200_e34453) + (assign25200_e34445 * (-(((-locals.var_vjdmrev_dn6) / locals.var_nvtmd) / ((assign25200_e34451).cosh() * (assign25200_e34451).cosh()))))) + (((locals.var_t4_dn6 / 2.0) * assign25200_e34465) + (assign25200_e34457 * (((-locals.var_vjdmrev_dn6) / locals.var_nvtmd) / ((assign25200_e34463).cosh() * (assign25200_e34463).cosh()))))), ((((locals.var_t3_dn7 / 2.0) * assign25200_e34453) + (assign25200_e34445 * (-(((-locals.var_vjdmrev_dn7) / locals.var_nvtmd) / ((assign25200_e34451).cosh() * (assign25200_e34451).cosh()))))) + (((locals.var_t4_dn7 / 2.0) * assign25200_e34465) + (assign25200_e34457 * (((-locals.var_vjdmrev_dn7) / locals.var_nvtmd) / ((assign25200_e34463).cosh() * (assign25200_e34463).cosh()))))), ((((locals.var_t3_dn8 / 2.0) * assign25200_e34453) + (assign25200_e34445 * (-(((-locals.var_vjdmrev_dn8) / locals.var_nvtmd) / ((assign25200_e34451).cosh() * (assign25200_e34451).cosh()))))) + (((locals.var_t4_dn8 / 2.0) * assign25200_e34465) + (assign25200_e34457 * (((-locals.var_vjdmrev_dn8) / locals.var_nvtmd) / ((assign25200_e34463).cosh() * (assign25200_e34463).cosh()))))), ((((locals.var_t3_dn9 / 2.0) * assign25200_e34453) + (assign25200_e34445 * (-(((-locals.var_vjdmrev_dn9) / locals.var_nvtmd) / ((assign25200_e34451).cosh() * (assign25200_e34451).cosh()))))) + (((locals.var_t4_dn9 / 2.0) * assign25200_e34465) + (assign25200_e34457 * (((-locals.var_vjdmrev_dn9) / locals.var_nvtmd) / ((assign25200_e34463).cosh() * (assign25200_e34463).cosh()))))), ((((locals.var_t3_dn10 / 2.0) * assign25200_e34453) + (assign25200_e34445 * (-(((-locals.var_vjdmrev_dn10) / locals.var_nvtmd) / ((assign25200_e34451).cosh() * (assign25200_e34451).cosh()))))) + (((locals.var_t4_dn10 / 2.0) * assign25200_e34465) + (assign25200_e34457 * (((-locals.var_vjdmrev_dn10) / locals.var_nvtmd) / ((assign25200_e34463).cosh() * (assign25200_e34463).cosh()))))), ((((locals.var_t3_dn11 / 2.0) * assign25200_e34453) + (assign25200_e34445 * (-(((-locals.var_vjdmrev_dn11) / locals.var_nvtmd) / ((assign25200_e34451).cosh() * (assign25200_e34451).cosh()))))) + (((locals.var_t4_dn11 / 2.0) * assign25200_e34465) + (assign25200_e34457 * (((-locals.var_vjdmrev_dn11) / locals.var_nvtmd) / ((assign25200_e34463).cosh() * (assign25200_e34463).cosh()))))), ((((locals.var_t3_dn12 / 2.0) * assign25200_e34453) + (assign25200_e34445 * (-(((-locals.var_vjdmrev_dn12) / locals.var_nvtmd) / ((assign25200_e34451).cosh() * (assign25200_e34451).cosh()))))) + (((locals.var_t4_dn12 / 2.0) * assign25200_e34465) + (assign25200_e34457 * (((-locals.var_vjdmrev_dn12) / locals.var_nvtmd) / ((assign25200_e34463).cosh() * (assign25200_e34463).cosh()))))), ((((locals.var_t3_dn13 / 2.0) * assign25200_e34453) + (assign25200_e34445 * (-(((locals.var_vbd_ext_dn13 - locals.var_vjdmrev_dn13) / locals.var_nvtmd) / ((assign25200_e34451).cosh() * (assign25200_e34451).cosh()))))) + (((locals.var_t4_dn13 / 2.0) * assign25200_e34465) + (assign25200_e34457 * (((locals.var_vbd_ext_dn13 - locals.var_vjdmrev_dn13) / locals.var_nvtmd) / ((assign25200_e34463).cosh() * (assign25200_e34463).cosh()))))), ((((locals.var_t3_dn14 / 2.0) * assign25200_e34453) + (assign25200_e34445 * (-(((locals.var_vbd_ext_dn14 - locals.var_vjdmrev_dn14) / locals.var_nvtmd) / ((assign25200_e34451).cosh() * (assign25200_e34451).cosh()))))) + (((locals.var_t4_dn14 / 2.0) * assign25200_e34465) + (assign25200_e34457 * (((locals.var_vbd_ext_dn14 - locals.var_vjdmrev_dn14) / locals.var_nvtmd) / ((assign25200_e34463).cosh() * (assign25200_e34463).cosh()))))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn3, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn12, locals.var_t6_dn13, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign25200_e34469;
        locals.var_t6_dn0 = assign25200_e34469_d_n0;
        locals.var_t6_dn2 = assign25200_e34469_d_n2;
        locals.var_t6_dn3 = assign25200_e34469_d_n3;
        locals.var_t6_dn4 = assign25200_e34469_d_n4;
        locals.var_t6_dn5 = assign25200_e34469_d_n5;
        locals.var_t6_dn6 = assign25200_e34469_d_n6;
        locals.var_t6_dn7 = assign25200_e34469_d_n7;
        locals.var_t6_dn8 = assign25200_e34469_d_n8;
        locals.var_t6_dn9 = assign25200_e34469_d_n9;
        locals.var_t6_dn10 = assign25200_e34469_d_n10;
        locals.var_t6_dn11 = assign25200_e34469_d_n11;
        locals.var_t6_dn12 = assign25200_e34469_d_n12;
        locals.var_t6_dn13 = assign25200_e34469_d_n13;
        locals.var_t6_dn14 = assign25200_e34469_d_n14;
        locals.var_t6_rv = 0.0;

        let assign25250_e34519: f64 = if locals.var_jtsd_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard624 = assign25250_e34519;
        locals.var_guard624_rv = 0.0;

        let assign25260_e34522: f64 = (p.p749 - locals.var_vbd_jct);
        let assign25260_e34525: f64 = (p.p749 * 0.001);
        let assign25260_e34526: f64 = if assign25260_e34522 < assign25260_e34525 { 1.0 } else { 0.0 };
        locals.var_guard625 = assign25260_e34526;
        locals.var_guard625_rv = 0.0;

        let (assign25270_e34537, assign25270_e34537_d_n0, assign25270_e34537_d_n2, assign25270_e34537_d_n3, assign25270_e34537_d_n4, assign25270_e34537_d_n5, assign25270_e34537_d_n6, assign25270_e34537_d_n7, assign25270_e34537_d_n8, assign25270_e34537_d_n9, assign25270_e34537_d_n10, assign25270_e34537_d_n11, assign25270_e34537_d_n12, assign25270_e34537_d_n13, assign25270_e34537_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard625 != 0.0)) {
        let assign25270_e34531: f64 = (-locals.var_vbd_jct);
        let assign25270_e34533: f64 = (assign25270_e34531 / locals.var_vtm0);
        let assign25270_e34535: f64 = (assign25270_e34533 / locals.var_njtsd_t);
        (assign25270_e34535, 0.0, 0.0, 0.0, (-((assign25270_e34533 * locals.var_njtsd_t_dn4) / (locals.var_njtsd_t * locals.var_njtsd_t))), (((-locals.var_vbd_jct_dn5) / locals.var_vtm0) / locals.var_njtsd_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (((-locals.var_vbd_jct_dn13) / locals.var_vtm0) / locals.var_njtsd_t), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign25270_e34537;
        locals.var_t0_dn0 = assign25270_e34537_d_n0;
        locals.var_t0_dn2 = assign25270_e34537_d_n2;
        locals.var_t0_dn3 = assign25270_e34537_d_n3;
        locals.var_t0_dn4 = assign25270_e34537_d_n4;
        locals.var_t0_dn5 = assign25270_e34537_d_n5;
        locals.var_t0_dn6 = assign25270_e34537_d_n6;
        locals.var_t0_dn7 = assign25270_e34537_d_n7;
        locals.var_t0_dn8 = assign25270_e34537_d_n8;
        locals.var_t0_dn9 = assign25270_e34537_d_n9;
        locals.var_t0_dn10 = assign25270_e34537_d_n10;
        locals.var_t0_dn11 = assign25270_e34537_d_n11;
        locals.var_t0_dn12 = assign25270_e34537_d_n12;
        locals.var_t0_dn13 = assign25270_e34537_d_n13;
        locals.var_t0_dn14 = assign25270_e34537_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign25280_e34548, assign25280_e34548_d_n0, assign25280_e34548_d_n2, assign25280_e34548_d_n3, assign25280_e34548_d_n4, assign25280_e34548_d_n5, assign25280_e34548_d_n6, assign25280_e34548_d_n7, assign25280_e34548_d_n8, assign25280_e34548_d_n9, assign25280_e34548_d_n10, assign25280_e34548_d_n11, assign25280_e34548_d_n12, assign25280_e34548_d_n13, assign25280_e34548_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard625 != 0.0)) {
        let assign25280_e34543: f64 = (locals.var_t0 * 1000.0);
        let assign25280_e34544: f64 = { let limited_exp_arg = assign25280_e34543; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign25280_e34546: f64 = (assign25280_e34544 - 1.0);
        (assign25280_e34546, ({ let limited_exp_arg = assign25280_e34543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 * 1000.0)), ({ let limited_exp_arg = assign25280_e34543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 * 1000.0)), ({ let limited_exp_arg = assign25280_e34543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 * 1000.0)), ({ let limited_exp_arg = assign25280_e34543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn4 * 1000.0)), ({ let limited_exp_arg = assign25280_e34543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 * 1000.0)), ({ let limited_exp_arg = assign25280_e34543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 * 1000.0)), ({ let limited_exp_arg = assign25280_e34543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 * 1000.0)), ({ let limited_exp_arg = assign25280_e34543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 * 1000.0)), ({ let limited_exp_arg = assign25280_e34543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 * 1000.0)), ({ let limited_exp_arg = assign25280_e34543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 * 1000.0)), ({ let limited_exp_arg = assign25280_e34543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 * 1000.0)), ({ let limited_exp_arg = assign25280_e34543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn12 * 1000.0)), ({ let limited_exp_arg = assign25280_e34543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 * 1000.0)), ({ let limited_exp_arg = assign25280_e34543; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 * 1000.0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign25280_e34548;
        locals.var_t1_dn0 = assign25280_e34548_d_n0;
        locals.var_t1_dn2 = assign25280_e34548_d_n2;
        locals.var_t1_dn3 = assign25280_e34548_d_n3;
        locals.var_t1_dn4 = assign25280_e34548_d_n4;
        locals.var_t1_dn5 = assign25280_e34548_d_n5;
        locals.var_t1_dn6 = assign25280_e34548_d_n6;
        locals.var_t1_dn7 = assign25280_e34548_d_n7;
        locals.var_t1_dn8 = assign25280_e34548_d_n8;
        locals.var_t1_dn9 = assign25280_e34548_d_n9;
        locals.var_t1_dn10 = assign25280_e34548_d_n10;
        locals.var_t1_dn11 = assign25280_e34548_d_n11;
        locals.var_t1_dn12 = assign25280_e34548_d_n12;
        locals.var_t1_dn13 = assign25280_e34548_d_n13;
        locals.var_t1_dn14 = assign25280_e34548_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_75(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25300_e34574, assign25300_e34574_d_n0, assign25300_e34574_d_n2, assign25300_e34574_d_n3, assign25300_e34574_d_n4, assign25300_e34574_d_n5, assign25300_e34574_d_n6, assign25300_e34574_d_n7, assign25300_e34574_d_n8, assign25300_e34574_d_n9, assign25300_e34574_d_n10, assign25300_e34574_d_n11, assign25300_e34574_d_n12, assign25300_e34574_d_n13, assign25300_e34574_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard625 == 0.0)) {
        let assign25300_e34568: f64 = (-locals.var_vbd_jct);
        let assign25300_e34570: f64 = (assign25300_e34568 / locals.var_vtm0);
        let assign25300_e34572: f64 = (assign25300_e34570 / locals.var_njtsd_t);
        (assign25300_e34572, 0.0, 0.0, 0.0, (-((assign25300_e34570 * locals.var_njtsd_t_dn4) / (locals.var_njtsd_t * locals.var_njtsd_t))), (((-locals.var_vbd_jct_dn5) / locals.var_vtm0) / locals.var_njtsd_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (((-locals.var_vbd_jct_dn13) / locals.var_vtm0) / locals.var_njtsd_t), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign25300_e34574;
        locals.var_t0_dn0 = assign25300_e34574_d_n0;
        locals.var_t0_dn2 = assign25300_e34574_d_n2;
        locals.var_t0_dn3 = assign25300_e34574_d_n3;
        locals.var_t0_dn4 = assign25300_e34574_d_n4;
        locals.var_t0_dn5 = assign25300_e34574_d_n5;
        locals.var_t0_dn6 = assign25300_e34574_d_n6;
        locals.var_t0_dn7 = assign25300_e34574_d_n7;
        locals.var_t0_dn8 = assign25300_e34574_d_n8;
        locals.var_t0_dn9 = assign25300_e34574_d_n9;
        locals.var_t0_dn10 = assign25300_e34574_d_n10;
        locals.var_t0_dn11 = assign25300_e34574_d_n11;
        locals.var_t0_dn12 = assign25300_e34574_d_n12;
        locals.var_t0_dn13 = assign25300_e34574_d_n13;
        locals.var_t0_dn14 = assign25300_e34574_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign25310_e34590, assign25310_e34590_d_n0, assign25310_e34590_d_n2, assign25310_e34590_d_n3, assign25310_e34590_d_n4, assign25310_e34590_d_n5, assign25310_e34590_d_n6, assign25310_e34590_d_n7, assign25310_e34590_d_n8, assign25310_e34590_d_n9, assign25310_e34590_d_n10, assign25310_e34590_d_n11, assign25310_e34590_d_n12, assign25310_e34590_d_n13, assign25310_e34590_d_n14,) = {
    if ((locals.var_guard624 != 0.0) && (locals.var_guard625 == 0.0)) {
        let assign25310_e34581: f64 = (locals.var_t0 * p.p749);
        let assign25310_e34584: f64 = (p.p749 - locals.var_vbd_jct);
        let assign25310_e34585: f64 = (assign25310_e34581 / assign25310_e34584);
        let assign25310_e34586: f64 = { let limited_exp_arg = assign25310_e34585; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign25310_e34588: f64 = (assign25310_e34586 - 1.0);
        (assign25310_e34588, ({ let limited_exp_arg = assign25310_e34585; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 * p.p749) / assign25310_e34584)), ({ let limited_exp_arg = assign25310_e34585; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 * p.p749) / assign25310_e34584)), ({ let limited_exp_arg = assign25310_e34585; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn3 * p.p749) / assign25310_e34584)), ({ let limited_exp_arg = assign25310_e34585; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 * p.p749) / assign25310_e34584)), ({ let limited_exp_arg = assign25310_e34585; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn5 * p.p749) * assign25310_e34584) - (assign25310_e34581 * (-locals.var_vbd_jct_dn5))) / (assign25310_e34584 * assign25310_e34584))), ({ let limited_exp_arg = assign25310_e34585; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 * p.p749) / assign25310_e34584)), ({ let limited_exp_arg = assign25310_e34585; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 * p.p749) / assign25310_e34584)), ({ let limited_exp_arg = assign25310_e34585; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 * p.p749) / assign25310_e34584)), ({ let limited_exp_arg = assign25310_e34585; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 * p.p749) / assign25310_e34584)), ({ let limited_exp_arg = assign25310_e34585; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 * p.p749) / assign25310_e34584)), ({ let limited_exp_arg = assign25310_e34585; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 * p.p749) / assign25310_e34584)), ({ let limited_exp_arg = assign25310_e34585; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn12 * p.p749) / assign25310_e34584)), ({ let limited_exp_arg = assign25310_e34585; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn13 * p.p749) * assign25310_e34584) - (assign25310_e34581 * (-locals.var_vbd_jct_dn13))) / (assign25310_e34584 * assign25310_e34584))), ({ let limited_exp_arg = assign25310_e34585; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 * p.p749) / assign25310_e34584)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign25310_e34590;
        locals.var_t1_dn0 = assign25310_e34590_d_n0;
        locals.var_t1_dn2 = assign25310_e34590_d_n2;
        locals.var_t1_dn3 = assign25310_e34590_d_n3;
        locals.var_t1_dn4 = assign25310_e34590_d_n4;
        locals.var_t1_dn5 = assign25310_e34590_d_n5;
        locals.var_t1_dn6 = assign25310_e34590_d_n6;
        locals.var_t1_dn7 = assign25310_e34590_d_n7;
        locals.var_t1_dn8 = assign25310_e34590_d_n8;
        locals.var_t1_dn9 = assign25310_e34590_d_n9;
        locals.var_t1_dn10 = assign25310_e34590_d_n10;
        locals.var_t1_dn11 = assign25310_e34590_d_n11;
        locals.var_t1_dn12 = assign25310_e34590_d_n12;
        locals.var_t1_dn13 = assign25310_e34590_d_n13;
        locals.var_t1_dn14 = assign25310_e34590_d_n14;
        locals.var_t1_rv = 0.0;

        let assign25330_e34608: f64 = if locals.var_jtsswd_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard626 = assign25330_e34608;
        locals.var_guard626_rv = 0.0;

        let assign25340_e34615: f64 = if ((p.p1128 > 0.0) && (p.p1097 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard627 = assign25340_e34615;
        locals.var_guard627_rv = 0.0;

        let assign25350_e34619: f64 = (locals.var_weffcj * p.p2);
        let assign25350_e34620: f64 = if locals.var_pdeff > assign25350_e34619 { 1.0 } else { 0.0 };
        locals.var_guard628 = assign25350_e34620;
        locals.var_guard628_rv = 0.0;

        let (assign25360_e34636, assign25360_e34636_d_n0, assign25360_e34636_d_n2, assign25360_e34636_d_n3, assign25360_e34636_d_n4, assign25360_e34636_d_n5, assign25360_e34636_d_n6, assign25360_e34636_d_n7, assign25360_e34636_d_n8, assign25360_e34636_d_n9, assign25360_e34636_d_n10, assign25360_e34636_d_n11, assign25360_e34636_d_n12, assign25360_e34636_d_n13, assign25360_e34636_d_n14,) = {
    if (((locals.var_guard626 != 0.0) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 != 0.0)) {
        let assign25360_e34630: f64 = (locals.var_weffcj * p.p2);
        let assign25360_e34631: f64 = (locals.var_pdeff - assign25360_e34630);
        let assign25360_e34632: f64 = (locals.var_oneminusxpart * assign25360_e34631);
        let assign25360_e34634: f64 = (assign25360_e34632 * locals.var_jtsswd_t);
        (assign25360_e34634, ((locals.var_oneminusxpart * locals.var_pdeff_dn0) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn2) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn3) * locals.var_jtsswd_t), (((locals.var_oneminusxpart * locals.var_pdeff_dn4) * locals.var_jtsswd_t) + (assign25360_e34632 * locals.var_jtsswd_t_dn4)), ((locals.var_oneminusxpart * locals.var_pdeff_dn5) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn6) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn7) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn8) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn9) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn10) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn11) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn12) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn13) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn14) * locals.var_jtsswd_t),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign25360_e34636;
        locals.var_t2_dn0 = assign25360_e34636_d_n0;
        locals.var_t2_dn2 = assign25360_e34636_d_n2;
        locals.var_t2_dn3 = assign25360_e34636_d_n3;
        locals.var_t2_dn4 = assign25360_e34636_d_n4;
        locals.var_t2_dn5 = assign25360_e34636_d_n5;
        locals.var_t2_dn6 = assign25360_e34636_d_n6;
        locals.var_t2_dn7 = assign25360_e34636_d_n7;
        locals.var_t2_dn8 = assign25360_e34636_d_n8;
        locals.var_t2_dn9 = assign25360_e34636_d_n9;
        locals.var_t2_dn10 = assign25360_e34636_d_n10;
        locals.var_t2_dn11 = assign25360_e34636_d_n11;
        locals.var_t2_dn12 = assign25360_e34636_d_n12;
        locals.var_t2_dn13 = assign25360_e34636_d_n13;
        locals.var_t2_dn14 = assign25360_e34636_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign25370_e34649, assign25370_e34649_d_n0, assign25370_e34649_d_n2, assign25370_e34649_d_n3, assign25370_e34649_d_n4, assign25370_e34649_d_n5, assign25370_e34649_d_n6, assign25370_e34649_d_n7, assign25370_e34649_d_n8, assign25370_e34649_d_n9, assign25370_e34649_d_n10, assign25370_e34649_d_n11, assign25370_e34649_d_n12, assign25370_e34649_d_n13, assign25370_e34649_d_n14,) = {
    if (((locals.var_guard626 != 0.0) && (locals.var_guard627 != 0.0)) && (locals.var_guard628 == 0.0)) {
        let assign25370_e34645: f64 = (locals.var_oneminusxpart * locals.var_pdeff);
        let assign25370_e34647: f64 = (assign25370_e34645 * locals.var_jtsswd_t);
        (assign25370_e34647, ((locals.var_oneminusxpart * locals.var_pdeff_dn0) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn2) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn3) * locals.var_jtsswd_t), (((locals.var_oneminusxpart * locals.var_pdeff_dn4) * locals.var_jtsswd_t) + (assign25370_e34645 * locals.var_jtsswd_t_dn4)), ((locals.var_oneminusxpart * locals.var_pdeff_dn5) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn6) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn7) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn8) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn9) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn10) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn11) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn12) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn13) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn14) * locals.var_jtsswd_t),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign25370_e34649;
        locals.var_t2_dn0 = assign25370_e34649_d_n0;
        locals.var_t2_dn2 = assign25370_e34649_d_n2;
        locals.var_t2_dn3 = assign25370_e34649_d_n3;
        locals.var_t2_dn4 = assign25370_e34649_d_n4;
        locals.var_t2_dn5 = assign25370_e34649_d_n5;
        locals.var_t2_dn6 = assign25370_e34649_d_n6;
        locals.var_t2_dn7 = assign25370_e34649_d_n7;
        locals.var_t2_dn8 = assign25370_e34649_d_n8;
        locals.var_t2_dn9 = assign25370_e34649_d_n9;
        locals.var_t2_dn10 = assign25370_e34649_d_n10;
        locals.var_t2_dn11 = assign25370_e34649_d_n11;
        locals.var_t2_dn12 = assign25370_e34649_d_n12;
        locals.var_t2_dn13 = assign25370_e34649_d_n13;
        locals.var_t2_dn14 = assign25370_e34649_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign25380_e34660, assign25380_e34660_d_n0, assign25380_e34660_d_n2, assign25380_e34660_d_n3, assign25380_e34660_d_n4, assign25380_e34660_d_n5, assign25380_e34660_d_n6, assign25380_e34660_d_n7, assign25380_e34660_d_n8, assign25380_e34660_d_n9, assign25380_e34660_d_n10, assign25380_e34660_d_n11, assign25380_e34660_d_n12, assign25380_e34660_d_n13, assign25380_e34660_d_n14,) = {
    if ((locals.var_guard626 != 0.0) && (locals.var_guard627 == 0.0)) {
        let assign25380_e34656: f64 = (locals.var_oneminusxpart * locals.var_pdeff);
        let assign25380_e34658: f64 = (assign25380_e34656 * locals.var_jtsswd_t);
        (assign25380_e34658, ((locals.var_oneminusxpart * locals.var_pdeff_dn0) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn2) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn3) * locals.var_jtsswd_t), (((locals.var_oneminusxpart * locals.var_pdeff_dn4) * locals.var_jtsswd_t) + (assign25380_e34656 * locals.var_jtsswd_t_dn4)), ((locals.var_oneminusxpart * locals.var_pdeff_dn5) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn6) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn7) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn8) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn9) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn10) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn11) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn12) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn13) * locals.var_jtsswd_t), ((locals.var_oneminusxpart * locals.var_pdeff_dn14) * locals.var_jtsswd_t),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign25380_e34660;
        locals.var_t2_dn0 = assign25380_e34660_d_n0;
        locals.var_t2_dn2 = assign25380_e34660_d_n2;
        locals.var_t2_dn3 = assign25380_e34660_d_n3;
        locals.var_t2_dn4 = assign25380_e34660_d_n4;
        locals.var_t2_dn5 = assign25380_e34660_d_n5;
        locals.var_t2_dn6 = assign25380_e34660_d_n6;
        locals.var_t2_dn7 = assign25380_e34660_d_n7;
        locals.var_t2_dn8 = assign25380_e34660_d_n8;
        locals.var_t2_dn9 = assign25380_e34660_d_n9;
        locals.var_t2_dn10 = assign25380_e34660_d_n10;
        locals.var_t2_dn11 = assign25380_e34660_d_n11;
        locals.var_t2_dn12 = assign25380_e34660_d_n12;
        locals.var_t2_dn13 = assign25380_e34660_d_n13;
        locals.var_t2_dn14 = assign25380_e34660_d_n14;
        locals.var_t2_rv = 0.0;

        let assign25390_e34663: f64 = (p.p751 - locals.var_vbd_jct);
        let assign25390_e34666: f64 = (p.p751 * 0.001);
        let assign25390_e34667: f64 = if assign25390_e34663 < assign25390_e34666 { 1.0 } else { 0.0 };
        locals.var_guard629 = assign25390_e34667;
        locals.var_guard629_rv = 0.0;

        let (assign25400_e34678, assign25400_e34678_d_n0, assign25400_e34678_d_n2, assign25400_e34678_d_n3, assign25400_e34678_d_n4, assign25400_e34678_d_n5, assign25400_e34678_d_n6, assign25400_e34678_d_n7, assign25400_e34678_d_n8, assign25400_e34678_d_n9, assign25400_e34678_d_n10, assign25400_e34678_d_n11, assign25400_e34678_d_n12, assign25400_e34678_d_n13, assign25400_e34678_d_n14,) = {
    if ((locals.var_guard626 != 0.0) && (locals.var_guard629 != 0.0)) {
        let assign25400_e34672: f64 = (-locals.var_vbd_jct);
        let assign25400_e34674: f64 = (assign25400_e34672 / locals.var_vtm0);
        let assign25400_e34676: f64 = (assign25400_e34674 / locals.var_njtsswd_t);
        (assign25400_e34676, 0.0, 0.0, 0.0, (-((assign25400_e34674 * locals.var_njtsswd_t_dn4) / (locals.var_njtsswd_t * locals.var_njtsswd_t))), (((-locals.var_vbd_jct_dn5) / locals.var_vtm0) / locals.var_njtsswd_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (((-locals.var_vbd_jct_dn13) / locals.var_vtm0) / locals.var_njtsswd_t), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign25400_e34678;
        locals.var_t0_dn0 = assign25400_e34678_d_n0;
        locals.var_t0_dn2 = assign25400_e34678_d_n2;
        locals.var_t0_dn3 = assign25400_e34678_d_n3;
        locals.var_t0_dn4 = assign25400_e34678_d_n4;
        locals.var_t0_dn5 = assign25400_e34678_d_n5;
        locals.var_t0_dn6 = assign25400_e34678_d_n6;
        locals.var_t0_dn7 = assign25400_e34678_d_n7;
        locals.var_t0_dn8 = assign25400_e34678_d_n8;
        locals.var_t0_dn9 = assign25400_e34678_d_n9;
        locals.var_t0_dn10 = assign25400_e34678_d_n10;
        locals.var_t0_dn11 = assign25400_e34678_d_n11;
        locals.var_t0_dn12 = assign25400_e34678_d_n12;
        locals.var_t0_dn13 = assign25400_e34678_d_n13;
        locals.var_t0_dn14 = assign25400_e34678_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign25410_e34689, assign25410_e34689_d_n0, assign25410_e34689_d_n2, assign25410_e34689_d_n3, assign25410_e34689_d_n4, assign25410_e34689_d_n5, assign25410_e34689_d_n6, assign25410_e34689_d_n7, assign25410_e34689_d_n8, assign25410_e34689_d_n9, assign25410_e34689_d_n10, assign25410_e34689_d_n11, assign25410_e34689_d_n12, assign25410_e34689_d_n13, assign25410_e34689_d_n14,) = {
    if ((locals.var_guard626 != 0.0) && (locals.var_guard629 != 0.0)) {
        let assign25410_e34684: f64 = (locals.var_t0 * 1000.0);
        let assign25410_e34685: f64 = { let limited_exp_arg = assign25410_e34684; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign25410_e34687: f64 = (assign25410_e34685 - 1.0);
        (assign25410_e34687, ({ let limited_exp_arg = assign25410_e34684; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 * 1000.0)), ({ let limited_exp_arg = assign25410_e34684; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 * 1000.0)), ({ let limited_exp_arg = assign25410_e34684; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 * 1000.0)), ({ let limited_exp_arg = assign25410_e34684; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn4 * 1000.0)), ({ let limited_exp_arg = assign25410_e34684; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 * 1000.0)), ({ let limited_exp_arg = assign25410_e34684; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 * 1000.0)), ({ let limited_exp_arg = assign25410_e34684; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 * 1000.0)), ({ let limited_exp_arg = assign25410_e34684; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 * 1000.0)), ({ let limited_exp_arg = assign25410_e34684; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 * 1000.0)), ({ let limited_exp_arg = assign25410_e34684; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 * 1000.0)), ({ let limited_exp_arg = assign25410_e34684; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 * 1000.0)), ({ let limited_exp_arg = assign25410_e34684; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn12 * 1000.0)), ({ let limited_exp_arg = assign25410_e34684; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 * 1000.0)), ({ let limited_exp_arg = assign25410_e34684; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 * 1000.0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign25410_e34689;
        locals.var_t1_dn0 = assign25410_e34689_d_n0;
        locals.var_t1_dn2 = assign25410_e34689_d_n2;
        locals.var_t1_dn3 = assign25410_e34689_d_n3;
        locals.var_t1_dn4 = assign25410_e34689_d_n4;
        locals.var_t1_dn5 = assign25410_e34689_d_n5;
        locals.var_t1_dn6 = assign25410_e34689_d_n6;
        locals.var_t1_dn7 = assign25410_e34689_d_n7;
        locals.var_t1_dn8 = assign25410_e34689_d_n8;
        locals.var_t1_dn9 = assign25410_e34689_d_n9;
        locals.var_t1_dn10 = assign25410_e34689_d_n10;
        locals.var_t1_dn11 = assign25410_e34689_d_n11;
        locals.var_t1_dn12 = assign25410_e34689_d_n12;
        locals.var_t1_dn13 = assign25410_e34689_d_n13;
        locals.var_t1_dn14 = assign25410_e34689_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign25430_e34711, assign25430_e34711_d_n0, assign25430_e34711_d_n2, assign25430_e34711_d_n3, assign25430_e34711_d_n4, assign25430_e34711_d_n5, assign25430_e34711_d_n6, assign25430_e34711_d_n7, assign25430_e34711_d_n8, assign25430_e34711_d_n9, assign25430_e34711_d_n10, assign25430_e34711_d_n11, assign25430_e34711_d_n12, assign25430_e34711_d_n13, assign25430_e34711_d_n14,) = {
    if ((locals.var_guard626 != 0.0) && (locals.var_guard629 == 0.0)) {
        let assign25430_e34705: f64 = (-locals.var_vbd_jct);
        let assign25430_e34707: f64 = (assign25430_e34705 / locals.var_vtm0);
        let assign25430_e34709: f64 = (assign25430_e34707 / locals.var_njtsswd_t);
        (assign25430_e34709, 0.0, 0.0, 0.0, (-((assign25430_e34707 * locals.var_njtsswd_t_dn4) / (locals.var_njtsswd_t * locals.var_njtsswd_t))), (((-locals.var_vbd_jct_dn5) / locals.var_vtm0) / locals.var_njtsswd_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (((-locals.var_vbd_jct_dn13) / locals.var_vtm0) / locals.var_njtsswd_t), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign25430_e34711;
        locals.var_t0_dn0 = assign25430_e34711_d_n0;
        locals.var_t0_dn2 = assign25430_e34711_d_n2;
        locals.var_t0_dn3 = assign25430_e34711_d_n3;
        locals.var_t0_dn4 = assign25430_e34711_d_n4;
        locals.var_t0_dn5 = assign25430_e34711_d_n5;
        locals.var_t0_dn6 = assign25430_e34711_d_n6;
        locals.var_t0_dn7 = assign25430_e34711_d_n7;
        locals.var_t0_dn8 = assign25430_e34711_d_n8;
        locals.var_t0_dn9 = assign25430_e34711_d_n9;
        locals.var_t0_dn10 = assign25430_e34711_d_n10;
        locals.var_t0_dn11 = assign25430_e34711_d_n11;
        locals.var_t0_dn12 = assign25430_e34711_d_n12;
        locals.var_t0_dn13 = assign25430_e34711_d_n13;
        locals.var_t0_dn14 = assign25430_e34711_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign25440_e34727, assign25440_e34727_d_n0, assign25440_e34727_d_n2, assign25440_e34727_d_n3, assign25440_e34727_d_n4, assign25440_e34727_d_n5, assign25440_e34727_d_n6, assign25440_e34727_d_n7, assign25440_e34727_d_n8, assign25440_e34727_d_n9, assign25440_e34727_d_n10, assign25440_e34727_d_n11, assign25440_e34727_d_n12, assign25440_e34727_d_n13, assign25440_e34727_d_n14,) = {
    if ((locals.var_guard626 != 0.0) && (locals.var_guard629 == 0.0)) {
        let assign25440_e34718: f64 = (locals.var_t0 * p.p751);
        let assign25440_e34721: f64 = (p.p751 - locals.var_vbd_jct);
        let assign25440_e34722: f64 = (assign25440_e34718 / assign25440_e34721);
        let assign25440_e34723: f64 = { let limited_exp_arg = assign25440_e34722; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign25440_e34725: f64 = (assign25440_e34723 - 1.0);
        (assign25440_e34725, ({ let limited_exp_arg = assign25440_e34722; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 * p.p751) / assign25440_e34721)), ({ let limited_exp_arg = assign25440_e34722; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 * p.p751) / assign25440_e34721)), ({ let limited_exp_arg = assign25440_e34722; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn3 * p.p751) / assign25440_e34721)), ({ let limited_exp_arg = assign25440_e34722; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 * p.p751) / assign25440_e34721)), ({ let limited_exp_arg = assign25440_e34722; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn5 * p.p751) * assign25440_e34721) - (assign25440_e34718 * (-locals.var_vbd_jct_dn5))) / (assign25440_e34721 * assign25440_e34721))), ({ let limited_exp_arg = assign25440_e34722; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 * p.p751) / assign25440_e34721)), ({ let limited_exp_arg = assign25440_e34722; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 * p.p751) / assign25440_e34721)), ({ let limited_exp_arg = assign25440_e34722; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 * p.p751) / assign25440_e34721)), ({ let limited_exp_arg = assign25440_e34722; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 * p.p751) / assign25440_e34721)), ({ let limited_exp_arg = assign25440_e34722; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 * p.p751) / assign25440_e34721)), ({ let limited_exp_arg = assign25440_e34722; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 * p.p751) / assign25440_e34721)), ({ let limited_exp_arg = assign25440_e34722; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn12 * p.p751) / assign25440_e34721)), ({ let limited_exp_arg = assign25440_e34722; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn13 * p.p751) * assign25440_e34721) - (assign25440_e34718 * (-locals.var_vbd_jct_dn13))) / (assign25440_e34721 * assign25440_e34721))), ({ let limited_exp_arg = assign25440_e34722; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 * p.p751) / assign25440_e34721)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign25440_e34727;
        locals.var_t1_dn0 = assign25440_e34727_d_n0;
        locals.var_t1_dn2 = assign25440_e34727_d_n2;
        locals.var_t1_dn3 = assign25440_e34727_d_n3;
        locals.var_t1_dn4 = assign25440_e34727_d_n4;
        locals.var_t1_dn5 = assign25440_e34727_d_n5;
        locals.var_t1_dn6 = assign25440_e34727_d_n6;
        locals.var_t1_dn7 = assign25440_e34727_d_n7;
        locals.var_t1_dn8 = assign25440_e34727_d_n8;
        locals.var_t1_dn9 = assign25440_e34727_d_n9;
        locals.var_t1_dn10 = assign25440_e34727_d_n10;
        locals.var_t1_dn11 = assign25440_e34727_d_n11;
        locals.var_t1_dn12 = assign25440_e34727_d_n12;
        locals.var_t1_dn13 = assign25440_e34727_d_n13;
        locals.var_t1_dn14 = assign25440_e34727_d_n14;
        locals.var_t1_rv = 0.0;

        let assign25460_e34741: f64 = if locals.var_jtsswgd_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard630 = assign25460_e34741;
        locals.var_guard630_rv = 0.0;

        let assign25470_e34744: f64 = (p.p753 - locals.var_vbd_jct);
        let assign25470_e34747: f64 = (p.p753 * 0.001);
        let assign25470_e34748: f64 = if assign25470_e34744 < assign25470_e34747 { 1.0 } else { 0.0 };
        locals.var_guard631 = assign25470_e34748;
        locals.var_guard631_rv = 0.0;

        let (assign25480_e34759, assign25480_e34759_d_n0, assign25480_e34759_d_n2, assign25480_e34759_d_n3, assign25480_e34759_d_n4, assign25480_e34759_d_n5, assign25480_e34759_d_n6, assign25480_e34759_d_n7, assign25480_e34759_d_n8, assign25480_e34759_d_n9, assign25480_e34759_d_n10, assign25480_e34759_d_n11, assign25480_e34759_d_n12, assign25480_e34759_d_n13, assign25480_e34759_d_n14,) = {
    if ((locals.var_guard630 != 0.0) && (locals.var_guard631 != 0.0)) {
        let assign25480_e34753: f64 = (-locals.var_vbd_jct);
        let assign25480_e34755: f64 = (assign25480_e34753 / locals.var_vtm0);
        let assign25480_e34757: f64 = (assign25480_e34755 / locals.var_njtsswgd_t);
        (assign25480_e34757, 0.0, 0.0, 0.0, (-((assign25480_e34755 * locals.var_njtsswgd_t_dn4) / (locals.var_njtsswgd_t * locals.var_njtsswgd_t))), (((-locals.var_vbd_jct_dn5) / locals.var_vtm0) / locals.var_njtsswgd_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (((-locals.var_vbd_jct_dn13) / locals.var_vtm0) / locals.var_njtsswgd_t), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign25480_e34759;
        locals.var_t0_dn0 = assign25480_e34759_d_n0;
        locals.var_t0_dn2 = assign25480_e34759_d_n2;
        locals.var_t0_dn3 = assign25480_e34759_d_n3;
        locals.var_t0_dn4 = assign25480_e34759_d_n4;
        locals.var_t0_dn5 = assign25480_e34759_d_n5;
        locals.var_t0_dn6 = assign25480_e34759_d_n6;
        locals.var_t0_dn7 = assign25480_e34759_d_n7;
        locals.var_t0_dn8 = assign25480_e34759_d_n8;
        locals.var_t0_dn9 = assign25480_e34759_d_n9;
        locals.var_t0_dn10 = assign25480_e34759_d_n10;
        locals.var_t0_dn11 = assign25480_e34759_d_n11;
        locals.var_t0_dn12 = assign25480_e34759_d_n12;
        locals.var_t0_dn13 = assign25480_e34759_d_n13;
        locals.var_t0_dn14 = assign25480_e34759_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign25490_e34770, assign25490_e34770_d_n0, assign25490_e34770_d_n2, assign25490_e34770_d_n3, assign25490_e34770_d_n4, assign25490_e34770_d_n5, assign25490_e34770_d_n6, assign25490_e34770_d_n7, assign25490_e34770_d_n8, assign25490_e34770_d_n9, assign25490_e34770_d_n10, assign25490_e34770_d_n11, assign25490_e34770_d_n12, assign25490_e34770_d_n13, assign25490_e34770_d_n14,) = {
    if ((locals.var_guard630 != 0.0) && (locals.var_guard631 != 0.0)) {
        let assign25490_e34765: f64 = (locals.var_t0 * 1000.0);
        let assign25490_e34766: f64 = { let limited_exp_arg = assign25490_e34765; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign25490_e34768: f64 = (assign25490_e34766 - 1.0);
        (assign25490_e34768, ({ let limited_exp_arg = assign25490_e34765; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 * 1000.0)), ({ let limited_exp_arg = assign25490_e34765; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 * 1000.0)), ({ let limited_exp_arg = assign25490_e34765; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 * 1000.0)), ({ let limited_exp_arg = assign25490_e34765; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn4 * 1000.0)), ({ let limited_exp_arg = assign25490_e34765; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 * 1000.0)), ({ let limited_exp_arg = assign25490_e34765; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 * 1000.0)), ({ let limited_exp_arg = assign25490_e34765; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 * 1000.0)), ({ let limited_exp_arg = assign25490_e34765; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 * 1000.0)), ({ let limited_exp_arg = assign25490_e34765; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 * 1000.0)), ({ let limited_exp_arg = assign25490_e34765; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 * 1000.0)), ({ let limited_exp_arg = assign25490_e34765; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 * 1000.0)), ({ let limited_exp_arg = assign25490_e34765; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn12 * 1000.0)), ({ let limited_exp_arg = assign25490_e34765; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 * 1000.0)), ({ let limited_exp_arg = assign25490_e34765; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 * 1000.0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign25490_e34770;
        locals.var_t1_dn0 = assign25490_e34770_d_n0;
        locals.var_t1_dn2 = assign25490_e34770_d_n2;
        locals.var_t1_dn3 = assign25490_e34770_d_n3;
        locals.var_t1_dn4 = assign25490_e34770_d_n4;
        locals.var_t1_dn5 = assign25490_e34770_d_n5;
        locals.var_t1_dn6 = assign25490_e34770_d_n6;
        locals.var_t1_dn7 = assign25490_e34770_d_n7;
        locals.var_t1_dn8 = assign25490_e34770_d_n8;
        locals.var_t1_dn9 = assign25490_e34770_d_n9;
        locals.var_t1_dn10 = assign25490_e34770_d_n10;
        locals.var_t1_dn11 = assign25490_e34770_d_n11;
        locals.var_t1_dn12 = assign25490_e34770_d_n12;
        locals.var_t1_dn13 = assign25490_e34770_d_n13;
        locals.var_t1_dn14 = assign25490_e34770_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign25510_e34796, assign25510_e34796_d_n0, assign25510_e34796_d_n2, assign25510_e34796_d_n3, assign25510_e34796_d_n4, assign25510_e34796_d_n5, assign25510_e34796_d_n6, assign25510_e34796_d_n7, assign25510_e34796_d_n8, assign25510_e34796_d_n9, assign25510_e34796_d_n10, assign25510_e34796_d_n11, assign25510_e34796_d_n12, assign25510_e34796_d_n13, assign25510_e34796_d_n14,) = {
    if ((locals.var_guard630 != 0.0) && (locals.var_guard631 == 0.0)) {
        let assign25510_e34790: f64 = (-locals.var_vbd_jct);
        let assign25510_e34792: f64 = (assign25510_e34790 / locals.var_vtm0);
        let assign25510_e34794: f64 = (assign25510_e34792 / locals.var_njtsswgd_t);
        (assign25510_e34794, 0.0, 0.0, 0.0, (-((assign25510_e34792 * locals.var_njtsswgd_t_dn4) / (locals.var_njtsswgd_t * locals.var_njtsswgd_t))), (((-locals.var_vbd_jct_dn5) / locals.var_vtm0) / locals.var_njtsswgd_t), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (((-locals.var_vbd_jct_dn13) / locals.var_vtm0) / locals.var_njtsswgd_t), 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign25510_e34796;
        locals.var_t0_dn0 = assign25510_e34796_d_n0;
        locals.var_t0_dn2 = assign25510_e34796_d_n2;
        locals.var_t0_dn3 = assign25510_e34796_d_n3;
        locals.var_t0_dn4 = assign25510_e34796_d_n4;
        locals.var_t0_dn5 = assign25510_e34796_d_n5;
        locals.var_t0_dn6 = assign25510_e34796_d_n6;
        locals.var_t0_dn7 = assign25510_e34796_d_n7;
        locals.var_t0_dn8 = assign25510_e34796_d_n8;
        locals.var_t0_dn9 = assign25510_e34796_d_n9;
        locals.var_t0_dn10 = assign25510_e34796_d_n10;
        locals.var_t0_dn11 = assign25510_e34796_d_n11;
        locals.var_t0_dn12 = assign25510_e34796_d_n12;
        locals.var_t0_dn13 = assign25510_e34796_d_n13;
        locals.var_t0_dn14 = assign25510_e34796_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign25520_e34812, assign25520_e34812_d_n0, assign25520_e34812_d_n2, assign25520_e34812_d_n3, assign25520_e34812_d_n4, assign25520_e34812_d_n5, assign25520_e34812_d_n6, assign25520_e34812_d_n7, assign25520_e34812_d_n8, assign25520_e34812_d_n9, assign25520_e34812_d_n10, assign25520_e34812_d_n11, assign25520_e34812_d_n12, assign25520_e34812_d_n13, assign25520_e34812_d_n14,) = {
    if ((locals.var_guard630 != 0.0) && (locals.var_guard631 == 0.0)) {
        let assign25520_e34803: f64 = (locals.var_t0 * p.p753);
        let assign25520_e34806: f64 = (p.p753 - locals.var_vbd_jct);
        let assign25520_e34807: f64 = (assign25520_e34803 / assign25520_e34806);
        let assign25520_e34808: f64 = { let limited_exp_arg = assign25520_e34807; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign25520_e34810: f64 = (assign25520_e34808 - 1.0);
        (assign25520_e34810, ({ let limited_exp_arg = assign25520_e34807; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 * p.p753) / assign25520_e34806)), ({ let limited_exp_arg = assign25520_e34807; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 * p.p753) / assign25520_e34806)), ({ let limited_exp_arg = assign25520_e34807; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn3 * p.p753) / assign25520_e34806)), ({ let limited_exp_arg = assign25520_e34807; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 * p.p753) / assign25520_e34806)), ({ let limited_exp_arg = assign25520_e34807; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn5 * p.p753) * assign25520_e34806) - (assign25520_e34803 * (-locals.var_vbd_jct_dn5))) / (assign25520_e34806 * assign25520_e34806))), ({ let limited_exp_arg = assign25520_e34807; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 * p.p753) / assign25520_e34806)), ({ let limited_exp_arg = assign25520_e34807; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 * p.p753) / assign25520_e34806)), ({ let limited_exp_arg = assign25520_e34807; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 * p.p753) / assign25520_e34806)), ({ let limited_exp_arg = assign25520_e34807; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 * p.p753) / assign25520_e34806)), ({ let limited_exp_arg = assign25520_e34807; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 * p.p753) / assign25520_e34806)), ({ let limited_exp_arg = assign25520_e34807; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 * p.p753) / assign25520_e34806)), ({ let limited_exp_arg = assign25520_e34807; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn12 * p.p753) / assign25520_e34806)), ({ let limited_exp_arg = assign25520_e34807; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn13 * p.p753) * assign25520_e34806) - (assign25520_e34803 * (-locals.var_vbd_jct_dn13))) / (assign25520_e34806 * assign25520_e34806))), ({ let limited_exp_arg = assign25520_e34807; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn14 * p.p753) / assign25520_e34806)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign25520_e34812;
        locals.var_t1_dn0 = assign25520_e34812_d_n0;
        locals.var_t1_dn2 = assign25520_e34812_d_n2;
        locals.var_t1_dn3 = assign25520_e34812_d_n3;
        locals.var_t1_dn4 = assign25520_e34812_d_n4;
        locals.var_t1_dn5 = assign25520_e34812_d_n5;
        locals.var_t1_dn6 = assign25520_e34812_d_n6;
        locals.var_t1_dn7 = assign25520_e34812_d_n7;
        locals.var_t1_dn8 = assign25520_e34812_d_n8;
        locals.var_t1_dn9 = assign25520_e34812_d_n9;
        locals.var_t1_dn10 = assign25520_e34812_d_n10;
        locals.var_t1_dn11 = assign25520_e34812_d_n11;
        locals.var_t1_dn12 = assign25520_e34812_d_n12;
        locals.var_t1_dn13 = assign25520_e34812_d_n13;
        locals.var_t1_dn14 = assign25520_e34812_d_n14;
        locals.var_t1_rv = 0.0;

        let assign25540_e34830: f64 = if p.p1128 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard632 = assign25540_e34830;
        locals.var_guard632_rv = 0.0;

        let assign25550_e34833: f64 = if locals.var_jtsd_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard633 = assign25550_e34833;
        locals.var_guard633_rv = 0.0;

        let assign25560_e34836: f64 = (p.p749 - locals.var_vbd_ext);
        let assign25560_e34839: f64 = (p.p749 * 0.001);
        let assign25560_e34840: f64 = if assign25560_e34836 < assign25560_e34839 { 1.0 } else { 0.0 };
        locals.var_guard634 = assign25560_e34840;
        locals.var_guard634_rv = 0.0;

        let (assign25570_e34853, assign25570_e34853_d_n0, assign25570_e34853_d_n2, assign25570_e34853_d_n3, assign25570_e34853_d_n4, assign25570_e34853_d_n5, assign25570_e34853_d_n6, assign25570_e34853_d_n7, assign25570_e34853_d_n8, assign25570_e34853_d_n9, assign25570_e34853_d_n10, assign25570_e34853_d_n11, assign25570_e34853_d_n12, assign25570_e34853_d_n13, assign25570_e34853_d_n14,) = {
    if (((locals.var_guard632 != 0.0) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign25570_e34847: f64 = (-locals.var_vbd_ext);
        let assign25570_e34849: f64 = (assign25570_e34847 / locals.var_vtm0);
        let assign25570_e34851: f64 = (assign25570_e34849 / locals.var_njtsd_t);
        (assign25570_e34851, 0.0, 0.0, 0.0, (-((assign25570_e34849 * locals.var_njtsd_t_dn4) / (locals.var_njtsd_t * locals.var_njtsd_t))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (((-locals.var_vbd_ext_dn13) / locals.var_vtm0) / locals.var_njtsd_t), (((-locals.var_vbd_ext_dn14) / locals.var_vtm0) / locals.var_njtsd_t),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign25570_e34853;
        locals.var_t0_dn0 = assign25570_e34853_d_n0;
        locals.var_t0_dn2 = assign25570_e34853_d_n2;
        locals.var_t0_dn3 = assign25570_e34853_d_n3;
        locals.var_t0_dn4 = assign25570_e34853_d_n4;
        locals.var_t0_dn5 = assign25570_e34853_d_n5;
        locals.var_t0_dn6 = assign25570_e34853_d_n6;
        locals.var_t0_dn7 = assign25570_e34853_d_n7;
        locals.var_t0_dn8 = assign25570_e34853_d_n8;
        locals.var_t0_dn9 = assign25570_e34853_d_n9;
        locals.var_t0_dn10 = assign25570_e34853_d_n10;
        locals.var_t0_dn11 = assign25570_e34853_d_n11;
        locals.var_t0_dn12 = assign25570_e34853_d_n12;
        locals.var_t0_dn13 = assign25570_e34853_d_n13;
        locals.var_t0_dn14 = assign25570_e34853_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign25580_e34866, assign25580_e34866_d_n0, assign25580_e34866_d_n2, assign25580_e34866_d_n3, assign25580_e34866_d_n4, assign25580_e34866_d_n5, assign25580_e34866_d_n6, assign25580_e34866_d_n7, assign25580_e34866_d_n8, assign25580_e34866_d_n9, assign25580_e34866_d_n10, assign25580_e34866_d_n11, assign25580_e34866_d_n12, assign25580_e34866_d_n13, assign25580_e34866_d_n14,) = {
    if (((locals.var_guard632 != 0.0) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 != 0.0)) {
        let assign25580_e34861: f64 = (locals.var_t0 * 1000.0);
        let assign25580_e34862: f64 = { let limited_exp_arg = assign25580_e34861; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign25580_e34864: f64 = (assign25580_e34862 - 1.0);
        (assign25580_e34864, ({ let limited_exp_arg = assign25580_e34861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 * 1000.0)), ({ let limited_exp_arg = assign25580_e34861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 * 1000.0)), ({ let limited_exp_arg = assign25580_e34861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 * 1000.0)), ({ let limited_exp_arg = assign25580_e34861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn4 * 1000.0)), ({ let limited_exp_arg = assign25580_e34861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 * 1000.0)), ({ let limited_exp_arg = assign25580_e34861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 * 1000.0)), ({ let limited_exp_arg = assign25580_e34861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 * 1000.0)), ({ let limited_exp_arg = assign25580_e34861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 * 1000.0)), ({ let limited_exp_arg = assign25580_e34861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 * 1000.0)), ({ let limited_exp_arg = assign25580_e34861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 * 1000.0)), ({ let limited_exp_arg = assign25580_e34861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 * 1000.0)), ({ let limited_exp_arg = assign25580_e34861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn12 * 1000.0)), ({ let limited_exp_arg = assign25580_e34861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 * 1000.0)), ({ let limited_exp_arg = assign25580_e34861; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 * 1000.0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign25580_e34866;
        locals.var_t1_dn0 = assign25580_e34866_d_n0;
        locals.var_t1_dn2 = assign25580_e34866_d_n2;
        locals.var_t1_dn3 = assign25580_e34866_d_n3;
        locals.var_t1_dn4 = assign25580_e34866_d_n4;
        locals.var_t1_dn5 = assign25580_e34866_d_n5;
        locals.var_t1_dn6 = assign25580_e34866_d_n6;
        locals.var_t1_dn7 = assign25580_e34866_d_n7;
        locals.var_t1_dn8 = assign25580_e34866_d_n8;
        locals.var_t1_dn9 = assign25580_e34866_d_n9;
        locals.var_t1_dn10 = assign25580_e34866_d_n10;
        locals.var_t1_dn11 = assign25580_e34866_d_n11;
        locals.var_t1_dn12 = assign25580_e34866_d_n12;
        locals.var_t1_dn13 = assign25580_e34866_d_n13;
        locals.var_t1_dn14 = assign25580_e34866_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign25600_e34896, assign25600_e34896_d_n0, assign25600_e34896_d_n2, assign25600_e34896_d_n3, assign25600_e34896_d_n4, assign25600_e34896_d_n5, assign25600_e34896_d_n6, assign25600_e34896_d_n7, assign25600_e34896_d_n8, assign25600_e34896_d_n9, assign25600_e34896_d_n10, assign25600_e34896_d_n11, assign25600_e34896_d_n12, assign25600_e34896_d_n13, assign25600_e34896_d_n14,) = {
    if (((locals.var_guard632 != 0.0) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 == 0.0)) {
        let assign25600_e34890: f64 = (-locals.var_vbd_ext);
        let assign25600_e34892: f64 = (assign25600_e34890 / locals.var_vtm0);
        let assign25600_e34894: f64 = (assign25600_e34892 / locals.var_njtsd_t);
        (assign25600_e34894, 0.0, 0.0, 0.0, (-((assign25600_e34892 * locals.var_njtsd_t_dn4) / (locals.var_njtsd_t * locals.var_njtsd_t))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (((-locals.var_vbd_ext_dn13) / locals.var_vtm0) / locals.var_njtsd_t), (((-locals.var_vbd_ext_dn14) / locals.var_vtm0) / locals.var_njtsd_t),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign25600_e34896;
        locals.var_t0_dn0 = assign25600_e34896_d_n0;
        locals.var_t0_dn2 = assign25600_e34896_d_n2;
        locals.var_t0_dn3 = assign25600_e34896_d_n3;
        locals.var_t0_dn4 = assign25600_e34896_d_n4;
        locals.var_t0_dn5 = assign25600_e34896_d_n5;
        locals.var_t0_dn6 = assign25600_e34896_d_n6;
        locals.var_t0_dn7 = assign25600_e34896_d_n7;
        locals.var_t0_dn8 = assign25600_e34896_d_n8;
        locals.var_t0_dn9 = assign25600_e34896_d_n9;
        locals.var_t0_dn10 = assign25600_e34896_d_n10;
        locals.var_t0_dn11 = assign25600_e34896_d_n11;
        locals.var_t0_dn12 = assign25600_e34896_d_n12;
        locals.var_t0_dn13 = assign25600_e34896_d_n13;
        locals.var_t0_dn14 = assign25600_e34896_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign25610_e34914, assign25610_e34914_d_n0, assign25610_e34914_d_n2, assign25610_e34914_d_n3, assign25610_e34914_d_n4, assign25610_e34914_d_n5, assign25610_e34914_d_n6, assign25610_e34914_d_n7, assign25610_e34914_d_n8, assign25610_e34914_d_n9, assign25610_e34914_d_n10, assign25610_e34914_d_n11, assign25610_e34914_d_n12, assign25610_e34914_d_n13, assign25610_e34914_d_n14,) = {
    if (((locals.var_guard632 != 0.0) && (locals.var_guard633 != 0.0)) && (locals.var_guard634 == 0.0)) {
        let assign25610_e34905: f64 = (locals.var_t0 * p.p749);
        let assign25610_e34908: f64 = (p.p749 - locals.var_vbd_ext);
        let assign25610_e34909: f64 = (assign25610_e34905 / assign25610_e34908);
        let assign25610_e34910: f64 = { let limited_exp_arg = assign25610_e34909; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign25610_e34912: f64 = (assign25610_e34910 - 1.0);
        (assign25610_e34912, ({ let limited_exp_arg = assign25610_e34909; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 * p.p749) / assign25610_e34908)), ({ let limited_exp_arg = assign25610_e34909; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 * p.p749) / assign25610_e34908)), ({ let limited_exp_arg = assign25610_e34909; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn3 * p.p749) / assign25610_e34908)), ({ let limited_exp_arg = assign25610_e34909; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 * p.p749) / assign25610_e34908)), ({ let limited_exp_arg = assign25610_e34909; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn5 * p.p749) / assign25610_e34908)), ({ let limited_exp_arg = assign25610_e34909; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 * p.p749) / assign25610_e34908)), ({ let limited_exp_arg = assign25610_e34909; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 * p.p749) / assign25610_e34908)), ({ let limited_exp_arg = assign25610_e34909; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 * p.p749) / assign25610_e34908)), ({ let limited_exp_arg = assign25610_e34909; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 * p.p749) / assign25610_e34908)), ({ let limited_exp_arg = assign25610_e34909; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 * p.p749) / assign25610_e34908)), ({ let limited_exp_arg = assign25610_e34909; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 * p.p749) / assign25610_e34908)), ({ let limited_exp_arg = assign25610_e34909; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn12 * p.p749) / assign25610_e34908)), ({ let limited_exp_arg = assign25610_e34909; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn13 * p.p749) * assign25610_e34908) - (assign25610_e34905 * (-locals.var_vbd_ext_dn13))) / (assign25610_e34908 * assign25610_e34908))), ({ let limited_exp_arg = assign25610_e34909; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn14 * p.p749) * assign25610_e34908) - (assign25610_e34905 * (-locals.var_vbd_ext_dn14))) / (assign25610_e34908 * assign25610_e34908))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign25610_e34914;
        locals.var_t1_dn0 = assign25610_e34914_d_n0;
        locals.var_t1_dn2 = assign25610_e34914_d_n2;
        locals.var_t1_dn3 = assign25610_e34914_d_n3;
        locals.var_t1_dn4 = assign25610_e34914_d_n4;
        locals.var_t1_dn5 = assign25610_e34914_d_n5;
        locals.var_t1_dn6 = assign25610_e34914_d_n6;
        locals.var_t1_dn7 = assign25610_e34914_d_n7;
        locals.var_t1_dn8 = assign25610_e34914_d_n8;
        locals.var_t1_dn9 = assign25610_e34914_d_n9;
        locals.var_t1_dn10 = assign25610_e34914_d_n10;
        locals.var_t1_dn11 = assign25610_e34914_d_n11;
        locals.var_t1_dn12 = assign25610_e34914_d_n12;
        locals.var_t1_dn13 = assign25610_e34914_d_n13;
        locals.var_t1_dn14 = assign25610_e34914_d_n14;
        locals.var_t1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_76(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign25630_e34934: f64 = if locals.var_jtsswd_t > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard635 = assign25630_e34934;
        locals.var_guard635_rv = 0.0;

        let assign25640_e34938: f64 = (locals.var_weffcj * p.p2);
        let assign25640_e34939: f64 = if locals.var_pdeff > assign25640_e34938 { 1.0 } else { 0.0 };
        locals.var_guard636 = assign25640_e34939;
        locals.var_guard636_rv = 0.0;

        let (assign25650_e34959, assign25650_e34959_d_n0, assign25650_e34959_d_n2, assign25650_e34959_d_n3, assign25650_e34959_d_n4, assign25650_e34959_d_n5, assign25650_e34959_d_n6, assign25650_e34959_d_n7, assign25650_e34959_d_n8, assign25650_e34959_d_n9, assign25650_e34959_d_n10, assign25650_e34959_d_n11, assign25650_e34959_d_n12, assign25650_e34959_d_n13, assign25650_e34959_d_n14,) = {
    if (((locals.var_guard632 != 0.0) && (locals.var_guard635 != 0.0)) && (locals.var_guard636 != 0.0)) {
        let assign25650_e34949: f64 = (locals.var_weffcj * p.p2);
        let assign25650_e34950: f64 = (locals.var_pdeff - assign25650_e34949);
        let assign25650_e34951: f64 = (p.p1128 * assign25650_e34950);
        let assign25650_e34954: f64 = (locals.var_weffcj * p.p2);
        let assign25650_e34955: f64 = (assign25650_e34951 + assign25650_e34954);
        let assign25650_e34957: f64 = (assign25650_e34955 * locals.var_jtsswd_t);
        (assign25650_e34957, ((p.p1128 * locals.var_pdeff_dn0) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn2) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn3) * locals.var_jtsswd_t), (((p.p1128 * locals.var_pdeff_dn4) * locals.var_jtsswd_t) + (assign25650_e34955 * locals.var_jtsswd_t_dn4)), ((p.p1128 * locals.var_pdeff_dn5) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn6) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn7) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn8) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn9) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn10) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn11) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn12) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn13) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn14) * locals.var_jtsswd_t),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign25650_e34959;
        locals.var_t2_dn0 = assign25650_e34959_d_n0;
        locals.var_t2_dn2 = assign25650_e34959_d_n2;
        locals.var_t2_dn3 = assign25650_e34959_d_n3;
        locals.var_t2_dn4 = assign25650_e34959_d_n4;
        locals.var_t2_dn5 = assign25650_e34959_d_n5;
        locals.var_t2_dn6 = assign25650_e34959_d_n6;
        locals.var_t2_dn7 = assign25650_e34959_d_n7;
        locals.var_t2_dn8 = assign25650_e34959_d_n8;
        locals.var_t2_dn9 = assign25650_e34959_d_n9;
        locals.var_t2_dn10 = assign25650_e34959_d_n10;
        locals.var_t2_dn11 = assign25650_e34959_d_n11;
        locals.var_t2_dn12 = assign25650_e34959_d_n12;
        locals.var_t2_dn13 = assign25650_e34959_d_n13;
        locals.var_t2_dn14 = assign25650_e34959_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign25660_e34972, assign25660_e34972_d_n0, assign25660_e34972_d_n2, assign25660_e34972_d_n3, assign25660_e34972_d_n4, assign25660_e34972_d_n5, assign25660_e34972_d_n6, assign25660_e34972_d_n7, assign25660_e34972_d_n8, assign25660_e34972_d_n9, assign25660_e34972_d_n10, assign25660_e34972_d_n11, assign25660_e34972_d_n12, assign25660_e34972_d_n13, assign25660_e34972_d_n14,) = {
    if (((locals.var_guard632 != 0.0) && (locals.var_guard635 != 0.0)) && (locals.var_guard636 == 0.0)) {
        let assign25660_e34968: f64 = (p.p1128 * locals.var_pdeff);
        let assign25660_e34970: f64 = (assign25660_e34968 * locals.var_jtsswd_t);
        (assign25660_e34970, ((p.p1128 * locals.var_pdeff_dn0) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn2) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn3) * locals.var_jtsswd_t), (((p.p1128 * locals.var_pdeff_dn4) * locals.var_jtsswd_t) + (assign25660_e34968 * locals.var_jtsswd_t_dn4)), ((p.p1128 * locals.var_pdeff_dn5) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn6) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn7) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn8) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn9) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn10) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn11) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn12) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn13) * locals.var_jtsswd_t), ((p.p1128 * locals.var_pdeff_dn14) * locals.var_jtsswd_t),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign25660_e34972;
        locals.var_t2_dn0 = assign25660_e34972_d_n0;
        locals.var_t2_dn2 = assign25660_e34972_d_n2;
        locals.var_t2_dn3 = assign25660_e34972_d_n3;
        locals.var_t2_dn4 = assign25660_e34972_d_n4;
        locals.var_t2_dn5 = assign25660_e34972_d_n5;
        locals.var_t2_dn6 = assign25660_e34972_d_n6;
        locals.var_t2_dn7 = assign25660_e34972_d_n7;
        locals.var_t2_dn8 = assign25660_e34972_d_n8;
        locals.var_t2_dn9 = assign25660_e34972_d_n9;
        locals.var_t2_dn10 = assign25660_e34972_d_n10;
        locals.var_t2_dn11 = assign25660_e34972_d_n11;
        locals.var_t2_dn12 = assign25660_e34972_d_n12;
        locals.var_t2_dn13 = assign25660_e34972_d_n13;
        locals.var_t2_dn14 = assign25660_e34972_d_n14;
        locals.var_t2_rv = 0.0;

        let assign25670_e34975: f64 = (p.p751 - locals.var_vbd_ext);
        let assign25670_e34978: f64 = (p.p751 * 0.001);
        let assign25670_e34979: f64 = if assign25670_e34975 < assign25670_e34978 { 1.0 } else { 0.0 };
        locals.var_guard637 = assign25670_e34979;
        locals.var_guard637_rv = 0.0;

        let (assign25680_e34992, assign25680_e34992_d_n0, assign25680_e34992_d_n2, assign25680_e34992_d_n3, assign25680_e34992_d_n4, assign25680_e34992_d_n5, assign25680_e34992_d_n6, assign25680_e34992_d_n7, assign25680_e34992_d_n8, assign25680_e34992_d_n9, assign25680_e34992_d_n10, assign25680_e34992_d_n11, assign25680_e34992_d_n12, assign25680_e34992_d_n13, assign25680_e34992_d_n14,) = {
    if (((locals.var_guard632 != 0.0) && (locals.var_guard635 != 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign25680_e34986: f64 = (-locals.var_vbd_ext);
        let assign25680_e34988: f64 = (assign25680_e34986 / locals.var_vtm0);
        let assign25680_e34990: f64 = (assign25680_e34988 / locals.var_njtsswd_t);
        (assign25680_e34990, 0.0, 0.0, 0.0, (-((assign25680_e34988 * locals.var_njtsswd_t_dn4) / (locals.var_njtsswd_t * locals.var_njtsswd_t))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (((-locals.var_vbd_ext_dn13) / locals.var_vtm0) / locals.var_njtsswd_t), (((-locals.var_vbd_ext_dn14) / locals.var_vtm0) / locals.var_njtsswd_t),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign25680_e34992;
        locals.var_t0_dn0 = assign25680_e34992_d_n0;
        locals.var_t0_dn2 = assign25680_e34992_d_n2;
        locals.var_t0_dn3 = assign25680_e34992_d_n3;
        locals.var_t0_dn4 = assign25680_e34992_d_n4;
        locals.var_t0_dn5 = assign25680_e34992_d_n5;
        locals.var_t0_dn6 = assign25680_e34992_d_n6;
        locals.var_t0_dn7 = assign25680_e34992_d_n7;
        locals.var_t0_dn8 = assign25680_e34992_d_n8;
        locals.var_t0_dn9 = assign25680_e34992_d_n9;
        locals.var_t0_dn10 = assign25680_e34992_d_n10;
        locals.var_t0_dn11 = assign25680_e34992_d_n11;
        locals.var_t0_dn12 = assign25680_e34992_d_n12;
        locals.var_t0_dn13 = assign25680_e34992_d_n13;
        locals.var_t0_dn14 = assign25680_e34992_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign25690_e35005, assign25690_e35005_d_n0, assign25690_e35005_d_n2, assign25690_e35005_d_n3, assign25690_e35005_d_n4, assign25690_e35005_d_n5, assign25690_e35005_d_n6, assign25690_e35005_d_n7, assign25690_e35005_d_n8, assign25690_e35005_d_n9, assign25690_e35005_d_n10, assign25690_e35005_d_n11, assign25690_e35005_d_n12, assign25690_e35005_d_n13, assign25690_e35005_d_n14,) = {
    if (((locals.var_guard632 != 0.0) && (locals.var_guard635 != 0.0)) && (locals.var_guard637 != 0.0)) {
        let assign25690_e35000: f64 = (locals.var_t0 * 1000.0);
        let assign25690_e35001: f64 = { let limited_exp_arg = assign25690_e35000; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign25690_e35003: f64 = (assign25690_e35001 - 1.0);
        (assign25690_e35003, ({ let limited_exp_arg = assign25690_e35000; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn0 * 1000.0)), ({ let limited_exp_arg = assign25690_e35000; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn2 * 1000.0)), ({ let limited_exp_arg = assign25690_e35000; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn3 * 1000.0)), ({ let limited_exp_arg = assign25690_e35000; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn4 * 1000.0)), ({ let limited_exp_arg = assign25690_e35000; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn5 * 1000.0)), ({ let limited_exp_arg = assign25690_e35000; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn6 * 1000.0)), ({ let limited_exp_arg = assign25690_e35000; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn7 * 1000.0)), ({ let limited_exp_arg = assign25690_e35000; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn8 * 1000.0)), ({ let limited_exp_arg = assign25690_e35000; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn9 * 1000.0)), ({ let limited_exp_arg = assign25690_e35000; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn10 * 1000.0)), ({ let limited_exp_arg = assign25690_e35000; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn11 * 1000.0)), ({ let limited_exp_arg = assign25690_e35000; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn12 * 1000.0)), ({ let limited_exp_arg = assign25690_e35000; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn13 * 1000.0)), ({ let limited_exp_arg = assign25690_e35000; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_t0_dn14 * 1000.0)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign25690_e35005;
        locals.var_t1_dn0 = assign25690_e35005_d_n0;
        locals.var_t1_dn2 = assign25690_e35005_d_n2;
        locals.var_t1_dn3 = assign25690_e35005_d_n3;
        locals.var_t1_dn4 = assign25690_e35005_d_n4;
        locals.var_t1_dn5 = assign25690_e35005_d_n5;
        locals.var_t1_dn6 = assign25690_e35005_d_n6;
        locals.var_t1_dn7 = assign25690_e35005_d_n7;
        locals.var_t1_dn8 = assign25690_e35005_d_n8;
        locals.var_t1_dn9 = assign25690_e35005_d_n9;
        locals.var_t1_dn10 = assign25690_e35005_d_n10;
        locals.var_t1_dn11 = assign25690_e35005_d_n11;
        locals.var_t1_dn12 = assign25690_e35005_d_n12;
        locals.var_t1_dn13 = assign25690_e35005_d_n13;
        locals.var_t1_dn14 = assign25690_e35005_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign25710_e35031, assign25710_e35031_d_n0, assign25710_e35031_d_n2, assign25710_e35031_d_n3, assign25710_e35031_d_n4, assign25710_e35031_d_n5, assign25710_e35031_d_n6, assign25710_e35031_d_n7, assign25710_e35031_d_n8, assign25710_e35031_d_n9, assign25710_e35031_d_n10, assign25710_e35031_d_n11, assign25710_e35031_d_n12, assign25710_e35031_d_n13, assign25710_e35031_d_n14,) = {
    if (((locals.var_guard632 != 0.0) && (locals.var_guard635 != 0.0)) && (locals.var_guard637 == 0.0)) {
        let assign25710_e35025: f64 = (-locals.var_vbd_ext);
        let assign25710_e35027: f64 = (assign25710_e35025 / locals.var_vtm0);
        let assign25710_e35029: f64 = (assign25710_e35027 / locals.var_njtsswd_t);
        (assign25710_e35029, 0.0, 0.0, 0.0, (-((assign25710_e35027 * locals.var_njtsswd_t_dn4) / (locals.var_njtsswd_t * locals.var_njtsswd_t))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (((-locals.var_vbd_ext_dn13) / locals.var_vtm0) / locals.var_njtsswd_t), (((-locals.var_vbd_ext_dn14) / locals.var_vtm0) / locals.var_njtsswd_t),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign25710_e35031;
        locals.var_t0_dn0 = assign25710_e35031_d_n0;
        locals.var_t0_dn2 = assign25710_e35031_d_n2;
        locals.var_t0_dn3 = assign25710_e35031_d_n3;
        locals.var_t0_dn4 = assign25710_e35031_d_n4;
        locals.var_t0_dn5 = assign25710_e35031_d_n5;
        locals.var_t0_dn6 = assign25710_e35031_d_n6;
        locals.var_t0_dn7 = assign25710_e35031_d_n7;
        locals.var_t0_dn8 = assign25710_e35031_d_n8;
        locals.var_t0_dn9 = assign25710_e35031_d_n9;
        locals.var_t0_dn10 = assign25710_e35031_d_n10;
        locals.var_t0_dn11 = assign25710_e35031_d_n11;
        locals.var_t0_dn12 = assign25710_e35031_d_n12;
        locals.var_t0_dn13 = assign25710_e35031_d_n13;
        locals.var_t0_dn14 = assign25710_e35031_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign25720_e35049, assign25720_e35049_d_n0, assign25720_e35049_d_n2, assign25720_e35049_d_n3, assign25720_e35049_d_n4, assign25720_e35049_d_n5, assign25720_e35049_d_n6, assign25720_e35049_d_n7, assign25720_e35049_d_n8, assign25720_e35049_d_n9, assign25720_e35049_d_n10, assign25720_e35049_d_n11, assign25720_e35049_d_n12, assign25720_e35049_d_n13, assign25720_e35049_d_n14,) = {
    if (((locals.var_guard632 != 0.0) && (locals.var_guard635 != 0.0)) && (locals.var_guard637 == 0.0)) {
        let assign25720_e35040: f64 = (locals.var_t0 * p.p751);
        let assign25720_e35043: f64 = (p.p751 - locals.var_vbd_ext);
        let assign25720_e35044: f64 = (assign25720_e35040 / assign25720_e35043);
        let assign25720_e35045: f64 = { let limited_exp_arg = assign25720_e35044; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign25720_e35047: f64 = (assign25720_e35045 - 1.0);
        (assign25720_e35047, ({ let limited_exp_arg = assign25720_e35044; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn0 * p.p751) / assign25720_e35043)), ({ let limited_exp_arg = assign25720_e35044; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn2 * p.p751) / assign25720_e35043)), ({ let limited_exp_arg = assign25720_e35044; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn3 * p.p751) / assign25720_e35043)), ({ let limited_exp_arg = assign25720_e35044; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn4 * p.p751) / assign25720_e35043)), ({ let limited_exp_arg = assign25720_e35044; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn5 * p.p751) / assign25720_e35043)), ({ let limited_exp_arg = assign25720_e35044; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn6 * p.p751) / assign25720_e35043)), ({ let limited_exp_arg = assign25720_e35044; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn7 * p.p751) / assign25720_e35043)), ({ let limited_exp_arg = assign25720_e35044; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn8 * p.p751) / assign25720_e35043)), ({ let limited_exp_arg = assign25720_e35044; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn9 * p.p751) / assign25720_e35043)), ({ let limited_exp_arg = assign25720_e35044; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn10 * p.p751) / assign25720_e35043)), ({ let limited_exp_arg = assign25720_e35044; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn11 * p.p751) / assign25720_e35043)), ({ let limited_exp_arg = assign25720_e35044; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_t0_dn12 * p.p751) / assign25720_e35043)), ({ let limited_exp_arg = assign25720_e35044; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn13 * p.p751) * assign25720_e35043) - (assign25720_e35040 * (-locals.var_vbd_ext_dn13))) / (assign25720_e35043 * assign25720_e35043))), ({ let limited_exp_arg = assign25720_e35044; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_t0_dn14 * p.p751) * assign25720_e35043) - (assign25720_e35040 * (-locals.var_vbd_ext_dn14))) / (assign25720_e35043 * assign25720_e35043))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign25720_e35049;
        locals.var_t1_dn0 = assign25720_e35049_d_n0;
        locals.var_t1_dn2 = assign25720_e35049_d_n2;
        locals.var_t1_dn3 = assign25720_e35049_d_n3;
        locals.var_t1_dn4 = assign25720_e35049_d_n4;
        locals.var_t1_dn5 = assign25720_e35049_d_n5;
        locals.var_t1_dn6 = assign25720_e35049_d_n6;
        locals.var_t1_dn7 = assign25720_e35049_d_n7;
        locals.var_t1_dn8 = assign25720_e35049_d_n8;
        locals.var_t1_dn9 = assign25720_e35049_d_n9;
        locals.var_t1_dn10 = assign25720_e35049_d_n10;
        locals.var_t1_dn11 = assign25720_e35049_d_n11;
        locals.var_t1_dn12 = assign25720_e35049_d_n12;
        locals.var_t1_dn13 = assign25720_e35049_d_n13;
        locals.var_t1_dn14 = assign25720_e35049_d_n14;
        locals.var_t1_rv = 0.0;

        let assign25740_e35065: f64 = (locals.var_cjs_t * locals.var_aseff);
        locals.var_czbs = assign25740_e35065;
        locals.var_czbs_dn0 = (locals.var_cjs_t * locals.var_aseff_dn0);
        locals.var_czbs_dn2 = (locals.var_cjs_t * locals.var_aseff_dn2);
        locals.var_czbs_dn3 = (locals.var_cjs_t * locals.var_aseff_dn3);
        locals.var_czbs_dn4 = ((locals.var_cjs_t_dn4 * locals.var_aseff) + (locals.var_cjs_t * locals.var_aseff_dn4));
        locals.var_czbs_dn5 = (locals.var_cjs_t * locals.var_aseff_dn5);
        locals.var_czbs_dn6 = (locals.var_cjs_t * locals.var_aseff_dn6);
        locals.var_czbs_dn7 = (locals.var_cjs_t * locals.var_aseff_dn7);
        locals.var_czbs_dn8 = (locals.var_cjs_t * locals.var_aseff_dn8);
        locals.var_czbs_dn9 = (locals.var_cjs_t * locals.var_aseff_dn9);
        locals.var_czbs_dn10 = (locals.var_cjs_t * locals.var_aseff_dn10);
        locals.var_czbs_dn11 = (locals.var_cjs_t * locals.var_aseff_dn11);
        locals.var_czbs_dn12 = (locals.var_cjs_t * locals.var_aseff_dn12);
        locals.var_czbs_dn13 = (locals.var_cjs_t * locals.var_aseff_dn13);
        locals.var_czbs_dn14 = (locals.var_cjs_t * locals.var_aseff_dn14);
        locals.var_czbs_rv = 0.0;

        let assign25750_e35068: f64 = (locals.var_cjsws_t * locals.var_pseff);
        locals.var_czbssw = assign25750_e35068;
        locals.var_czbssw_dn0 = (locals.var_cjsws_t * locals.var_pseff_dn0);
        locals.var_czbssw_dn2 = (locals.var_cjsws_t * locals.var_pseff_dn2);
        locals.var_czbssw_dn3 = (locals.var_cjsws_t * locals.var_pseff_dn3);
        locals.var_czbssw_dn4 = ((locals.var_cjsws_t_dn4 * locals.var_pseff) + (locals.var_cjsws_t * locals.var_pseff_dn4));
        locals.var_czbssw_dn5 = (locals.var_cjsws_t * locals.var_pseff_dn5);
        locals.var_czbssw_dn6 = (locals.var_cjsws_t * locals.var_pseff_dn6);
        locals.var_czbssw_dn7 = (locals.var_cjsws_t * locals.var_pseff_dn7);
        locals.var_czbssw_dn8 = (locals.var_cjsws_t * locals.var_pseff_dn8);
        locals.var_czbssw_dn9 = (locals.var_cjsws_t * locals.var_pseff_dn9);
        locals.var_czbssw_dn10 = (locals.var_cjsws_t * locals.var_pseff_dn10);
        locals.var_czbssw_dn11 = (locals.var_cjsws_t * locals.var_pseff_dn11);
        locals.var_czbssw_dn12 = (locals.var_cjsws_t * locals.var_pseff_dn12);
        locals.var_czbssw_dn13 = (locals.var_cjsws_t * locals.var_pseff_dn13);
        locals.var_czbssw_dn14 = (locals.var_cjsws_t * locals.var_pseff_dn14);
        locals.var_czbssw_rv = 0.0;

        let assign25760_e35071: f64 = (locals.var_cjswgs_t * locals.var_weffcj);
        let assign25760_e35073: f64 = (assign25760_e35071 * p.p2);
        locals.var_czbsswg = assign25760_e35073;
        locals.var_czbsswg_dn4 = ((locals.var_cjswgs_t_dn4 * locals.var_weffcj) * p.p2);
        locals.var_czbsswg_rv = 0.0;

        let assign25770_e35076: f64 = (-p.p713);
        let assign25770_e35077: f64 = (0.1_f64).powf(assign25770_e35076);
        locals.var_czbs_p1 = assign25770_e35077;
        locals.var_czbs_p1_rv = 0.0;

        let assign25780_e35080: f64 = if p.p713 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard638 = assign25780_e35080;
        locals.var_guard638_rv = 0.0;

        let (assign25790_e35087,) = {
    if (locals.var_guard638 != 0.0) {
        let assign25790_e35084: f64 = (0.1_f64).ln();
        let assign25790_e35085: f64 = (1.5 - assign25790_e35084);
        (assign25790_e35085,)
    } else {
        (locals.var_czbs_p2,)
    }
};
        locals.var_czbs_p2 = assign25790_e35087;
        locals.var_czbs_p2_rv = 0.0;

        let (assign25800_e35108,) = {
    if (locals.var_guard638 == 0.0) {
        let assign25800_e35093: f64 = (1.0 - p.p713);
        let assign25800_e35094: f64 = (1.0 / assign25800_e35093);
        let assign25800_e35098: f64 = (0.05 * p.p713);
        let assign25800_e35101: f64 = (1.0 + p.p713);
        let assign25800_e35102: f64 = (assign25800_e35098 * assign25800_e35101);
        let assign25800_e35104: f64 = (assign25800_e35102 * locals.var_czbs_p1);
        let assign25800_e35105: f64 = (1.0 - assign25800_e35104);
        let assign25800_e35106: f64 = (assign25800_e35094 * assign25800_e35105);
        (assign25800_e35106,)
    } else {
        (locals.var_czbs_p2,)
    }
};
        locals.var_czbs_p2 = assign25800_e35108;
        locals.var_czbs_p2_rv = 0.0;

        let assign25810_e35111: f64 = (-p.p715);
        let assign25810_e35112: f64 = (0.1_f64).powf(assign25810_e35111);
        locals.var_czbssw_p1 = assign25810_e35112;
        locals.var_czbssw_p1_rv = 0.0;

        let assign25820_e35115: f64 = if p.p715 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard639 = assign25820_e35115;
        locals.var_guard639_rv = 0.0;

        let (assign25830_e35122,) = {
    if (locals.var_guard639 != 0.0) {
        let assign25830_e35119: f64 = (0.1_f64).ln();
        let assign25830_e35120: f64 = (1.5 - assign25830_e35119);
        (assign25830_e35120,)
    } else {
        (locals.var_czbssw_p2,)
    }
};
        locals.var_czbssw_p2 = assign25830_e35122;
        locals.var_czbssw_p2_rv = 0.0;

        let (assign25840_e35143,) = {
    if (locals.var_guard639 == 0.0) {
        let assign25840_e35128: f64 = (1.0 - p.p715);
        let assign25840_e35129: f64 = (1.0 / assign25840_e35128);
        let assign25840_e35133: f64 = (0.05 * p.p715);
        let assign25840_e35136: f64 = (1.0 + p.p715);
        let assign25840_e35137: f64 = (assign25840_e35133 * assign25840_e35136);
        let assign25840_e35139: f64 = (assign25840_e35137 * locals.var_czbssw_p1);
        let assign25840_e35140: f64 = (1.0 - assign25840_e35139);
        let assign25840_e35141: f64 = (assign25840_e35129 * assign25840_e35140);
        (assign25840_e35141,)
    } else {
        (locals.var_czbssw_p2,)
    }
};
        locals.var_czbssw_p2 = assign25840_e35143;
        locals.var_czbssw_p2_rv = 0.0;

        let assign25850_e35146: f64 = (-p.p717);
        let assign25850_e35147: f64 = (0.1_f64).powf(assign25850_e35146);
        locals.var_czbsswg_p1 = assign25850_e35147;
        locals.var_czbsswg_p1_rv = 0.0;

        let assign25860_e35150: f64 = if p.p717 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard640 = assign25860_e35150;
        locals.var_guard640_rv = 0.0;

        let (assign25870_e35157,) = {
    if (locals.var_guard640 != 0.0) {
        let assign25870_e35154: f64 = (0.1_f64).ln();
        let assign25870_e35155: f64 = (1.5 - assign25870_e35154);
        (assign25870_e35155,)
    } else {
        (locals.var_czbsswg_p2,)
    }
};
        locals.var_czbsswg_p2 = assign25870_e35157;
        locals.var_czbsswg_p2_rv = 0.0;

        let (assign25880_e35178,) = {
    if (locals.var_guard640 == 0.0) {
        let assign25880_e35163: f64 = (1.0 - p.p717);
        let assign25880_e35164: f64 = (1.0 / assign25880_e35163);
        let assign25880_e35168: f64 = (0.05 * p.p717);
        let assign25880_e35171: f64 = (1.0 + p.p717);
        let assign25880_e35172: f64 = (assign25880_e35168 * assign25880_e35171);
        let assign25880_e35174: f64 = (assign25880_e35172 * locals.var_czbsswg_p1);
        let assign25880_e35175: f64 = (1.0 - assign25880_e35174);
        let assign25880_e35176: f64 = (assign25880_e35164 * assign25880_e35175);
        (assign25880_e35176,)
    } else {
        (locals.var_czbsswg_p2,)
    }
};
        locals.var_czbsswg_p2 = assign25880_e35178;
        locals.var_czbsswg_p2_rv = 0.0;

        let assign25890_e35181: f64 = if locals.var_czbs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard641 = assign25890_e35181;
        locals.var_guard641_rv = 0.0;

        let (assign25900_e35187, assign25900_e35187_d_n0, assign25900_e35187_d_n2, assign25900_e35187_d_n3, assign25900_e35187_d_n4, assign25900_e35187_d_n5, assign25900_e35187_d_n6, assign25900_e35187_d_n7, assign25900_e35187_d_n8, assign25900_e35187_d_n9, assign25900_e35187_d_n10, assign25900_e35187_d_n11, assign25900_e35187_d_n12, assign25900_e35187_d_n13, assign25900_e35187_d_n14,) = {
    if (locals.var_guard641 != 0.0) {
        let assign25900_e35185: f64 = (locals.var_vbs_jct / locals.var_pbs_t);
        (assign25900_e35185, 0.0, 0.0, 0.0, (-((locals.var_vbs_jct * locals.var_pbs_t_dn4) / (locals.var_pbs_t * locals.var_pbs_t))), 0.0, 0.0, (locals.var_vbs_jct_dn7 / locals.var_pbs_t), 0.0, 0.0, 0.0, 0.0, (locals.var_vbs_jct_dn12 / locals.var_pbs_t), 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign25900_e35187;
        locals.var_t1_dn0 = assign25900_e35187_d_n0;
        locals.var_t1_dn2 = assign25900_e35187_d_n2;
        locals.var_t1_dn3 = assign25900_e35187_d_n3;
        locals.var_t1_dn4 = assign25900_e35187_d_n4;
        locals.var_t1_dn5 = assign25900_e35187_d_n5;
        locals.var_t1_dn6 = assign25900_e35187_d_n6;
        locals.var_t1_dn7 = assign25900_e35187_d_n7;
        locals.var_t1_dn8 = assign25900_e35187_d_n8;
        locals.var_t1_dn9 = assign25900_e35187_d_n9;
        locals.var_t1_dn10 = assign25900_e35187_d_n10;
        locals.var_t1_dn11 = assign25900_e35187_d_n11;
        locals.var_t1_dn12 = assign25900_e35187_d_n12;
        locals.var_t1_dn13 = assign25900_e35187_d_n13;
        locals.var_t1_dn14 = assign25900_e35187_d_n14;
        locals.var_t1_rv = 0.0;

        let assign25910_e35190: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard642 = assign25910_e35190;
        locals.var_guard642_rv = 0.0;

        let (assign25920_e35198, assign25920_e35198_d_n0, assign25920_e35198_d_n2, assign25920_e35198_d_n3, assign25920_e35198_d_n4, assign25920_e35198_d_n5, assign25920_e35198_d_n6, assign25920_e35198_d_n7, assign25920_e35198_d_n8, assign25920_e35198_d_n9, assign25920_e35198_d_n10, assign25920_e35198_d_n11, assign25920_e35198_d_n12, assign25920_e35198_d_n13, assign25920_e35198_d_n14,) = {
    if ((locals.var_guard641 != 0.0) && (locals.var_guard642 != 0.0)) {
        let assign25920_e35196: f64 = (1.0 - locals.var_t1);
        (assign25920_e35196, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn12), (-locals.var_t1_dn13), (-locals.var_t1_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn13, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign25920_e35198;
        locals.var_arg_dn0 = assign25920_e35198_d_n0;
        locals.var_arg_dn2 = assign25920_e35198_d_n2;
        locals.var_arg_dn3 = assign25920_e35198_d_n3;
        locals.var_arg_dn4 = assign25920_e35198_d_n4;
        locals.var_arg_dn5 = assign25920_e35198_d_n5;
        locals.var_arg_dn6 = assign25920_e35198_d_n6;
        locals.var_arg_dn7 = assign25920_e35198_d_n7;
        locals.var_arg_dn8 = assign25920_e35198_d_n8;
        locals.var_arg_dn9 = assign25920_e35198_d_n9;
        locals.var_arg_dn10 = assign25920_e35198_d_n10;
        locals.var_arg_dn11 = assign25920_e35198_d_n11;
        locals.var_arg_dn12 = assign25920_e35198_d_n12;
        locals.var_arg_dn13 = assign25920_e35198_d_n13;
        locals.var_arg_dn14 = assign25920_e35198_d_n14;
        locals.var_arg_rv = 0.0;

        let assign25930_e35201: f64 = if p.p713 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard643 = assign25930_e35201;
        locals.var_guard643_rv = 0.0;

        let assign25940_e35204: f64 = if p.p713 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard644 = assign25940_e35204;
        locals.var_guard644_rv = 0.0;

        let (assign25950_e35217, assign25950_e35217_d_n0, assign25950_e35217_d_n2, assign25950_e35217_d_n3, assign25950_e35217_d_n4, assign25950_e35217_d_n5, assign25950_e35217_d_n6, assign25950_e35217_d_n7, assign25950_e35217_d_n8, assign25950_e35217_d_n9, assign25950_e35217_d_n10, assign25950_e35217_d_n11, assign25950_e35217_d_n12, assign25950_e35217_d_n13, assign25950_e35217_d_n14,) = {
    if ((((locals.var_guard641 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard643 != 0.0)) && (locals.var_guard644 != 0.0)) {
        let assign25950_e35214: f64 = (locals.var_arg).sqrt();
        let assign25950_e35215: f64 = (1.0 / assign25950_e35214);
        (assign25950_e35215, (-((locals.var_arg_dn0 / (2.0 * assign25950_e35214)) / (assign25950_e35214 * assign25950_e35214))), (-((locals.var_arg_dn2 / (2.0 * assign25950_e35214)) / (assign25950_e35214 * assign25950_e35214))), (-((locals.var_arg_dn3 / (2.0 * assign25950_e35214)) / (assign25950_e35214 * assign25950_e35214))), (-((locals.var_arg_dn4 / (2.0 * assign25950_e35214)) / (assign25950_e35214 * assign25950_e35214))), (-((locals.var_arg_dn5 / (2.0 * assign25950_e35214)) / (assign25950_e35214 * assign25950_e35214))), (-((locals.var_arg_dn6 / (2.0 * assign25950_e35214)) / (assign25950_e35214 * assign25950_e35214))), (-((locals.var_arg_dn7 / (2.0 * assign25950_e35214)) / (assign25950_e35214 * assign25950_e35214))), (-((locals.var_arg_dn8 / (2.0 * assign25950_e35214)) / (assign25950_e35214 * assign25950_e35214))), (-((locals.var_arg_dn9 / (2.0 * assign25950_e35214)) / (assign25950_e35214 * assign25950_e35214))), (-((locals.var_arg_dn10 / (2.0 * assign25950_e35214)) / (assign25950_e35214 * assign25950_e35214))), (-((locals.var_arg_dn11 / (2.0 * assign25950_e35214)) / (assign25950_e35214 * assign25950_e35214))), (-((locals.var_arg_dn12 / (2.0 * assign25950_e35214)) / (assign25950_e35214 * assign25950_e35214))), (-((locals.var_arg_dn13 / (2.0 * assign25950_e35214)) / (assign25950_e35214 * assign25950_e35214))), (-((locals.var_arg_dn14 / (2.0 * assign25950_e35214)) / (assign25950_e35214 * assign25950_e35214))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn12, locals.var_sarg_dn13, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign25950_e35217;
        locals.var_sarg_dn0 = assign25950_e35217_d_n0;
        locals.var_sarg_dn2 = assign25950_e35217_d_n2;
        locals.var_sarg_dn3 = assign25950_e35217_d_n3;
        locals.var_sarg_dn4 = assign25950_e35217_d_n4;
        locals.var_sarg_dn5 = assign25950_e35217_d_n5;
        locals.var_sarg_dn6 = assign25950_e35217_d_n6;
        locals.var_sarg_dn7 = assign25950_e35217_d_n7;
        locals.var_sarg_dn8 = assign25950_e35217_d_n8;
        locals.var_sarg_dn9 = assign25950_e35217_d_n9;
        locals.var_sarg_dn10 = assign25950_e35217_d_n10;
        locals.var_sarg_dn11 = assign25950_e35217_d_n11;
        locals.var_sarg_dn12 = assign25950_e35217_d_n12;
        locals.var_sarg_dn13 = assign25950_e35217_d_n13;
        locals.var_sarg_dn14 = assign25950_e35217_d_n14;
        locals.var_sarg_rv = 0.0;

        let (assign25960_e35233, assign25960_e35233_d_n0, assign25960_e35233_d_n2, assign25960_e35233_d_n3, assign25960_e35233_d_n4, assign25960_e35233_d_n5, assign25960_e35233_d_n6, assign25960_e35233_d_n7, assign25960_e35233_d_n8, assign25960_e35233_d_n9, assign25960_e35233_d_n10, assign25960_e35233_d_n11, assign25960_e35233_d_n12, assign25960_e35233_d_n13, assign25960_e35233_d_n14,) = {
    if ((((locals.var_guard641 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard643 != 0.0)) && (locals.var_guard644 == 0.0)) {
        let assign25960_e35227: f64 = (-p.p713);
        let assign25960_e35229: f64 = (locals.var_arg).ln();
        let assign25960_e35230: f64 = (assign25960_e35227 * assign25960_e35229);
        let assign25960_e35231: f64 = { let limited_exp_arg = assign25960_e35230; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign25960_e35231, ({ let limited_exp_arg = assign25960_e35230; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign25960_e35227 * (locals.var_arg_dn0 / locals.var_arg))), ({ let limited_exp_arg = assign25960_e35230; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign25960_e35227 * (locals.var_arg_dn2 / locals.var_arg))), ({ let limited_exp_arg = assign25960_e35230; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign25960_e35227 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign25960_e35230; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign25960_e35227 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign25960_e35230; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign25960_e35227 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign25960_e35230; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign25960_e35227 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign25960_e35230; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign25960_e35227 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign25960_e35230; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign25960_e35227 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign25960_e35230; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign25960_e35227 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign25960_e35230; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign25960_e35227 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign25960_e35230; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign25960_e35227 * (locals.var_arg_dn11 / locals.var_arg))), ({ let limited_exp_arg = assign25960_e35230; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign25960_e35227 * (locals.var_arg_dn12 / locals.var_arg))), ({ let limited_exp_arg = assign25960_e35230; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign25960_e35227 * (locals.var_arg_dn13 / locals.var_arg))), ({ let limited_exp_arg = assign25960_e35230; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign25960_e35227 * (locals.var_arg_dn14 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn12, locals.var_sarg_dn13, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign25960_e35233;
        locals.var_sarg_dn0 = assign25960_e35233_d_n0;
        locals.var_sarg_dn2 = assign25960_e35233_d_n2;
        locals.var_sarg_dn3 = assign25960_e35233_d_n3;
        locals.var_sarg_dn4 = assign25960_e35233_d_n4;
        locals.var_sarg_dn5 = assign25960_e35233_d_n5;
        locals.var_sarg_dn6 = assign25960_e35233_d_n6;
        locals.var_sarg_dn7 = assign25960_e35233_d_n7;
        locals.var_sarg_dn8 = assign25960_e35233_d_n8;
        locals.var_sarg_dn9 = assign25960_e35233_d_n9;
        locals.var_sarg_dn10 = assign25960_e35233_d_n10;
        locals.var_sarg_dn11 = assign25960_e35233_d_n11;
        locals.var_sarg_dn12 = assign25960_e35233_d_n12;
        locals.var_sarg_dn13 = assign25960_e35233_d_n13;
        locals.var_sarg_dn14 = assign25960_e35233_d_n14;
        locals.var_sarg_rv = 0.0;

        let (assign25970_e35253, assign25970_e35253_d_n0, assign25970_e35253_d_n2, assign25970_e35253_d_n3, assign25970_e35253_d_n4, assign25970_e35253_d_n5, assign25970_e35253_d_n6, assign25970_e35253_d_n7, assign25970_e35253_d_n8, assign25970_e35253_d_n9, assign25970_e35253_d_n10, assign25970_e35253_d_n11, assign25970_e35253_d_n12, assign25970_e35253_d_n13, assign25970_e35253_d_n14,) = {
    if (((locals.var_guard641 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard643 != 0.0)) {
        let assign25970_e35241: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign25970_e35245: f64 = (locals.var_arg * locals.var_sarg);
        let assign25970_e35246: f64 = (1.0 - assign25970_e35245);
        let assign25970_e35247: f64 = (assign25970_e35241 * assign25970_e35246);
        let assign25970_e35250: f64 = (1.0 - p.p713);
        let assign25970_e35251: f64 = (assign25970_e35247 / assign25970_e35250);
        (assign25970_e35251, ((((locals.var_pbs_t * locals.var_czbs_dn0) * assign25970_e35246) + (assign25970_e35241 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign25970_e35250), ((((locals.var_pbs_t * locals.var_czbs_dn2) * assign25970_e35246) + (assign25970_e35241 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign25970_e35250), ((((locals.var_pbs_t * locals.var_czbs_dn3) * assign25970_e35246) + (assign25970_e35241 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3))))) / assign25970_e35250), (((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign25970_e35246) + (assign25970_e35241 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign25970_e35250), ((((locals.var_pbs_t * locals.var_czbs_dn5) * assign25970_e35246) + (assign25970_e35241 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign25970_e35250), ((((locals.var_pbs_t * locals.var_czbs_dn6) * assign25970_e35246) + (assign25970_e35241 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign25970_e35250), ((((locals.var_pbs_t * locals.var_czbs_dn7) * assign25970_e35246) + (assign25970_e35241 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign25970_e35250), ((((locals.var_pbs_t * locals.var_czbs_dn8) * assign25970_e35246) + (assign25970_e35241 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign25970_e35250), ((((locals.var_pbs_t * locals.var_czbs_dn9) * assign25970_e35246) + (assign25970_e35241 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign25970_e35250), ((((locals.var_pbs_t * locals.var_czbs_dn10) * assign25970_e35246) + (assign25970_e35241 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign25970_e35250), ((((locals.var_pbs_t * locals.var_czbs_dn11) * assign25970_e35246) + (assign25970_e35241 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign25970_e35250), ((((locals.var_pbs_t * locals.var_czbs_dn12) * assign25970_e35246) + (assign25970_e35241 * (-((locals.var_arg_dn12 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn12))))) / assign25970_e35250), ((((locals.var_pbs_t * locals.var_czbs_dn13) * assign25970_e35246) + (assign25970_e35241 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign25970_e35250), ((((locals.var_pbs_t * locals.var_czbs_dn14) * assign25970_e35246) + (assign25970_e35241 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign25970_e35250),)
    } else {
        (locals.var_qbsj1, locals.var_qbsj1_dn0, locals.var_qbsj1_dn2, locals.var_qbsj1_dn3, locals.var_qbsj1_dn4, locals.var_qbsj1_dn5, locals.var_qbsj1_dn6, locals.var_qbsj1_dn7, locals.var_qbsj1_dn8, locals.var_qbsj1_dn9, locals.var_qbsj1_dn10, locals.var_qbsj1_dn11, locals.var_qbsj1_dn12, locals.var_qbsj1_dn13, locals.var_qbsj1_dn14,)
    }
};
        locals.var_qbsj1 = assign25970_e35253;
        locals.var_qbsj1_dn0 = assign25970_e35253_d_n0;
        locals.var_qbsj1_dn2 = assign25970_e35253_d_n2;
        locals.var_qbsj1_dn3 = assign25970_e35253_d_n3;
        locals.var_qbsj1_dn4 = assign25970_e35253_d_n4;
        locals.var_qbsj1_dn5 = assign25970_e35253_d_n5;
        locals.var_qbsj1_dn6 = assign25970_e35253_d_n6;
        locals.var_qbsj1_dn7 = assign25970_e35253_d_n7;
        locals.var_qbsj1_dn8 = assign25970_e35253_d_n8;
        locals.var_qbsj1_dn9 = assign25970_e35253_d_n9;
        locals.var_qbsj1_dn10 = assign25970_e35253_d_n10;
        locals.var_qbsj1_dn11 = assign25970_e35253_d_n11;
        locals.var_qbsj1_dn12 = assign25970_e35253_d_n12;
        locals.var_qbsj1_dn13 = assign25970_e35253_d_n13;
        locals.var_qbsj1_dn14 = assign25970_e35253_d_n14;
        locals.var_qbsj1_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_77(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign25980_e35268, assign25980_e35268_d_n0, assign25980_e35268_d_n2, assign25980_e35268_d_n3, assign25980_e35268_d_n4, assign25980_e35268_d_n5, assign25980_e35268_d_n6, assign25980_e35268_d_n7, assign25980_e35268_d_n8, assign25980_e35268_d_n9, assign25980_e35268_d_n10, assign25980_e35268_d_n11, assign25980_e35268_d_n12, assign25980_e35268_d_n13, assign25980_e35268_d_n14,) = {
    if (((locals.var_guard641 != 0.0) && (locals.var_guard642 != 0.0)) && (locals.var_guard643 == 0.0)) {
        let assign25980_e35262: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign25980_e35264: f64 = (locals.var_arg).ln();
        let assign25980_e35265: f64 = (-assign25980_e35264);
        let assign25980_e35266: f64 = (assign25980_e35262 * assign25980_e35265);
        (assign25980_e35266, (((locals.var_pbs_t * locals.var_czbs_dn0) * assign25980_e35265) + (assign25980_e35262 * (-(locals.var_arg_dn0 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn2) * assign25980_e35265) + (assign25980_e35262 * (-(locals.var_arg_dn2 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn3) * assign25980_e35265) + (assign25980_e35262 * (-(locals.var_arg_dn3 / locals.var_arg)))), ((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign25980_e35265) + (assign25980_e35262 * (-(locals.var_arg_dn4 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn5) * assign25980_e35265) + (assign25980_e35262 * (-(locals.var_arg_dn5 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn6) * assign25980_e35265) + (assign25980_e35262 * (-(locals.var_arg_dn6 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn7) * assign25980_e35265) + (assign25980_e35262 * (-(locals.var_arg_dn7 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn8) * assign25980_e35265) + (assign25980_e35262 * (-(locals.var_arg_dn8 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn9) * assign25980_e35265) + (assign25980_e35262 * (-(locals.var_arg_dn9 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn10) * assign25980_e35265) + (assign25980_e35262 * (-(locals.var_arg_dn10 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn11) * assign25980_e35265) + (assign25980_e35262 * (-(locals.var_arg_dn11 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn12) * assign25980_e35265) + (assign25980_e35262 * (-(locals.var_arg_dn12 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn13) * assign25980_e35265) + (assign25980_e35262 * (-(locals.var_arg_dn13 / locals.var_arg)))), (((locals.var_pbs_t * locals.var_czbs_dn14) * assign25980_e35265) + (assign25980_e35262 * (-(locals.var_arg_dn14 / locals.var_arg)))),)
    } else {
        (locals.var_qbsj1, locals.var_qbsj1_dn0, locals.var_qbsj1_dn2, locals.var_qbsj1_dn3, locals.var_qbsj1_dn4, locals.var_qbsj1_dn5, locals.var_qbsj1_dn6, locals.var_qbsj1_dn7, locals.var_qbsj1_dn8, locals.var_qbsj1_dn9, locals.var_qbsj1_dn10, locals.var_qbsj1_dn11, locals.var_qbsj1_dn12, locals.var_qbsj1_dn13, locals.var_qbsj1_dn14,)
    }
};
        locals.var_qbsj1 = assign25980_e35268;
        locals.var_qbsj1_dn0 = assign25980_e35268_d_n0;
        locals.var_qbsj1_dn2 = assign25980_e35268_d_n2;
        locals.var_qbsj1_dn3 = assign25980_e35268_d_n3;
        locals.var_qbsj1_dn4 = assign25980_e35268_d_n4;
        locals.var_qbsj1_dn5 = assign25980_e35268_d_n5;
        locals.var_qbsj1_dn6 = assign25980_e35268_d_n6;
        locals.var_qbsj1_dn7 = assign25980_e35268_d_n7;
        locals.var_qbsj1_dn8 = assign25980_e35268_d_n8;
        locals.var_qbsj1_dn9 = assign25980_e35268_d_n9;
        locals.var_qbsj1_dn10 = assign25980_e35268_d_n10;
        locals.var_qbsj1_dn11 = assign25980_e35268_d_n11;
        locals.var_qbsj1_dn12 = assign25980_e35268_d_n12;
        locals.var_qbsj1_dn13 = assign25980_e35268_d_n13;
        locals.var_qbsj1_dn14 = assign25980_e35268_d_n14;
        locals.var_qbsj1_rv = 0.0;

        let (assign25990_e35291, assign25990_e35291_d_n0, assign25990_e35291_d_n2, assign25990_e35291_d_n3, assign25990_e35291_d_n4, assign25990_e35291_d_n5, assign25990_e35291_d_n6, assign25990_e35291_d_n7, assign25990_e35291_d_n8, assign25990_e35291_d_n9, assign25990_e35291_d_n10, assign25990_e35291_d_n11, assign25990_e35291_d_n12, assign25990_e35291_d_n13, assign25990_e35291_d_n14,) = {
    if ((locals.var_guard641 != 0.0) && (locals.var_guard642 == 0.0)) {
        let assign25990_e35276: f64 = (locals.var_t1 - 1.0);
        let assign25990_e35277: f64 = (locals.var_czbs_p1 * assign25990_e35276);
        let assign25990_e35280: f64 = (5.0 * p.p713);
        let assign25990_e35283: f64 = (locals.var_t1 - 1.0);
        let assign25990_e35284: f64 = (assign25990_e35280 * assign25990_e35283);
        let assign25990_e35287: f64 = (1.0 + p.p713);
        let assign25990_e35288: f64 = (assign25990_e35284 + assign25990_e35287);
        let assign25990_e35289: f64 = (assign25990_e35277 * assign25990_e35288);
        (assign25990_e35289, (((locals.var_czbs_p1 * locals.var_t1_dn0) * assign25990_e35288) + (assign25990_e35277 * (assign25990_e35280 * locals.var_t1_dn0))), (((locals.var_czbs_p1 * locals.var_t1_dn2) * assign25990_e35288) + (assign25990_e35277 * (assign25990_e35280 * locals.var_t1_dn2))), (((locals.var_czbs_p1 * locals.var_t1_dn3) * assign25990_e35288) + (assign25990_e35277 * (assign25990_e35280 * locals.var_t1_dn3))), (((locals.var_czbs_p1 * locals.var_t1_dn4) * assign25990_e35288) + (assign25990_e35277 * (assign25990_e35280 * locals.var_t1_dn4))), (((locals.var_czbs_p1 * locals.var_t1_dn5) * assign25990_e35288) + (assign25990_e35277 * (assign25990_e35280 * locals.var_t1_dn5))), (((locals.var_czbs_p1 * locals.var_t1_dn6) * assign25990_e35288) + (assign25990_e35277 * (assign25990_e35280 * locals.var_t1_dn6))), (((locals.var_czbs_p1 * locals.var_t1_dn7) * assign25990_e35288) + (assign25990_e35277 * (assign25990_e35280 * locals.var_t1_dn7))), (((locals.var_czbs_p1 * locals.var_t1_dn8) * assign25990_e35288) + (assign25990_e35277 * (assign25990_e35280 * locals.var_t1_dn8))), (((locals.var_czbs_p1 * locals.var_t1_dn9) * assign25990_e35288) + (assign25990_e35277 * (assign25990_e35280 * locals.var_t1_dn9))), (((locals.var_czbs_p1 * locals.var_t1_dn10) * assign25990_e35288) + (assign25990_e35277 * (assign25990_e35280 * locals.var_t1_dn10))), (((locals.var_czbs_p1 * locals.var_t1_dn11) * assign25990_e35288) + (assign25990_e35277 * (assign25990_e35280 * locals.var_t1_dn11))), (((locals.var_czbs_p1 * locals.var_t1_dn12) * assign25990_e35288) + (assign25990_e35277 * (assign25990_e35280 * locals.var_t1_dn12))), (((locals.var_czbs_p1 * locals.var_t1_dn13) * assign25990_e35288) + (assign25990_e35277 * (assign25990_e35280 * locals.var_t1_dn13))), (((locals.var_czbs_p1 * locals.var_t1_dn14) * assign25990_e35288) + (assign25990_e35277 * (assign25990_e35280 * locals.var_t1_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign25990_e35291;
        locals.var_t2_dn0 = assign25990_e35291_d_n0;
        locals.var_t2_dn2 = assign25990_e35291_d_n2;
        locals.var_t2_dn3 = assign25990_e35291_d_n3;
        locals.var_t2_dn4 = assign25990_e35291_d_n4;
        locals.var_t2_dn5 = assign25990_e35291_d_n5;
        locals.var_t2_dn6 = assign25990_e35291_d_n6;
        locals.var_t2_dn7 = assign25990_e35291_d_n7;
        locals.var_t2_dn8 = assign25990_e35291_d_n8;
        locals.var_t2_dn9 = assign25990_e35291_d_n9;
        locals.var_t2_dn10 = assign25990_e35291_d_n10;
        locals.var_t2_dn11 = assign25990_e35291_d_n11;
        locals.var_t2_dn12 = assign25990_e35291_d_n12;
        locals.var_t2_dn13 = assign25990_e35291_d_n13;
        locals.var_t2_dn14 = assign25990_e35291_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign26000_e35304, assign26000_e35304_d_n0, assign26000_e35304_d_n2, assign26000_e35304_d_n3, assign26000_e35304_d_n4, assign26000_e35304_d_n5, assign26000_e35304_d_n6, assign26000_e35304_d_n7, assign26000_e35304_d_n8, assign26000_e35304_d_n9, assign26000_e35304_d_n10, assign26000_e35304_d_n11, assign26000_e35304_d_n12, assign26000_e35304_d_n13, assign26000_e35304_d_n14,) = {
    if ((locals.var_guard641 != 0.0) && (locals.var_guard642 == 0.0)) {
        let assign26000_e35298: f64 = (locals.var_pbs_t * locals.var_czbs);
        let assign26000_e35301: f64 = (locals.var_t2 + locals.var_czbs_p2);
        let assign26000_e35302: f64 = (assign26000_e35298 * assign26000_e35301);
        (assign26000_e35302, (((locals.var_pbs_t * locals.var_czbs_dn0) * assign26000_e35301) + (assign26000_e35298 * locals.var_t2_dn0)), (((locals.var_pbs_t * locals.var_czbs_dn2) * assign26000_e35301) + (assign26000_e35298 * locals.var_t2_dn2)), (((locals.var_pbs_t * locals.var_czbs_dn3) * assign26000_e35301) + (assign26000_e35298 * locals.var_t2_dn3)), ((((locals.var_pbs_t_dn4 * locals.var_czbs) + (locals.var_pbs_t * locals.var_czbs_dn4)) * assign26000_e35301) + (assign26000_e35298 * locals.var_t2_dn4)), (((locals.var_pbs_t * locals.var_czbs_dn5) * assign26000_e35301) + (assign26000_e35298 * locals.var_t2_dn5)), (((locals.var_pbs_t * locals.var_czbs_dn6) * assign26000_e35301) + (assign26000_e35298 * locals.var_t2_dn6)), (((locals.var_pbs_t * locals.var_czbs_dn7) * assign26000_e35301) + (assign26000_e35298 * locals.var_t2_dn7)), (((locals.var_pbs_t * locals.var_czbs_dn8) * assign26000_e35301) + (assign26000_e35298 * locals.var_t2_dn8)), (((locals.var_pbs_t * locals.var_czbs_dn9) * assign26000_e35301) + (assign26000_e35298 * locals.var_t2_dn9)), (((locals.var_pbs_t * locals.var_czbs_dn10) * assign26000_e35301) + (assign26000_e35298 * locals.var_t2_dn10)), (((locals.var_pbs_t * locals.var_czbs_dn11) * assign26000_e35301) + (assign26000_e35298 * locals.var_t2_dn11)), (((locals.var_pbs_t * locals.var_czbs_dn12) * assign26000_e35301) + (assign26000_e35298 * locals.var_t2_dn12)), (((locals.var_pbs_t * locals.var_czbs_dn13) * assign26000_e35301) + (assign26000_e35298 * locals.var_t2_dn13)), (((locals.var_pbs_t * locals.var_czbs_dn14) * assign26000_e35301) + (assign26000_e35298 * locals.var_t2_dn14)),)
    } else {
        (locals.var_qbsj1, locals.var_qbsj1_dn0, locals.var_qbsj1_dn2, locals.var_qbsj1_dn3, locals.var_qbsj1_dn4, locals.var_qbsj1_dn5, locals.var_qbsj1_dn6, locals.var_qbsj1_dn7, locals.var_qbsj1_dn8, locals.var_qbsj1_dn9, locals.var_qbsj1_dn10, locals.var_qbsj1_dn11, locals.var_qbsj1_dn12, locals.var_qbsj1_dn13, locals.var_qbsj1_dn14,)
    }
};
        locals.var_qbsj1 = assign26000_e35304;
        locals.var_qbsj1_dn0 = assign26000_e35304_d_n0;
        locals.var_qbsj1_dn2 = assign26000_e35304_d_n2;
        locals.var_qbsj1_dn3 = assign26000_e35304_d_n3;
        locals.var_qbsj1_dn4 = assign26000_e35304_d_n4;
        locals.var_qbsj1_dn5 = assign26000_e35304_d_n5;
        locals.var_qbsj1_dn6 = assign26000_e35304_d_n6;
        locals.var_qbsj1_dn7 = assign26000_e35304_d_n7;
        locals.var_qbsj1_dn8 = assign26000_e35304_d_n8;
        locals.var_qbsj1_dn9 = assign26000_e35304_d_n9;
        locals.var_qbsj1_dn10 = assign26000_e35304_d_n10;
        locals.var_qbsj1_dn11 = assign26000_e35304_d_n11;
        locals.var_qbsj1_dn12 = assign26000_e35304_d_n12;
        locals.var_qbsj1_dn13 = assign26000_e35304_d_n13;
        locals.var_qbsj1_dn14 = assign26000_e35304_d_n14;
        locals.var_qbsj1_rv = 0.0;

        let (assign26010_e35309, assign26010_e35309_d_n0, assign26010_e35309_d_n2, assign26010_e35309_d_n3, assign26010_e35309_d_n4, assign26010_e35309_d_n5, assign26010_e35309_d_n6, assign26010_e35309_d_n7, assign26010_e35309_d_n8, assign26010_e35309_d_n9, assign26010_e35309_d_n10, assign26010_e35309_d_n11, assign26010_e35309_d_n12, assign26010_e35309_d_n13, assign26010_e35309_d_n14,) = {
    if (locals.var_guard641 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbsj1, locals.var_qbsj1_dn0, locals.var_qbsj1_dn2, locals.var_qbsj1_dn3, locals.var_qbsj1_dn4, locals.var_qbsj1_dn5, locals.var_qbsj1_dn6, locals.var_qbsj1_dn7, locals.var_qbsj1_dn8, locals.var_qbsj1_dn9, locals.var_qbsj1_dn10, locals.var_qbsj1_dn11, locals.var_qbsj1_dn12, locals.var_qbsj1_dn13, locals.var_qbsj1_dn14,)
    }
};
        locals.var_qbsj1 = assign26010_e35309;
        locals.var_qbsj1_dn0 = assign26010_e35309_d_n0;
        locals.var_qbsj1_dn2 = assign26010_e35309_d_n2;
        locals.var_qbsj1_dn3 = assign26010_e35309_d_n3;
        locals.var_qbsj1_dn4 = assign26010_e35309_d_n4;
        locals.var_qbsj1_dn5 = assign26010_e35309_d_n5;
        locals.var_qbsj1_dn6 = assign26010_e35309_d_n6;
        locals.var_qbsj1_dn7 = assign26010_e35309_d_n7;
        locals.var_qbsj1_dn8 = assign26010_e35309_d_n8;
        locals.var_qbsj1_dn9 = assign26010_e35309_d_n9;
        locals.var_qbsj1_dn10 = assign26010_e35309_d_n10;
        locals.var_qbsj1_dn11 = assign26010_e35309_d_n11;
        locals.var_qbsj1_dn12 = assign26010_e35309_d_n12;
        locals.var_qbsj1_dn13 = assign26010_e35309_d_n13;
        locals.var_qbsj1_dn14 = assign26010_e35309_d_n14;
        locals.var_qbsj1_rv = 0.0;

        let assign26020_e35312: f64 = if locals.var_czbssw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard645 = assign26020_e35312;
        locals.var_guard645_rv = 0.0;

        let (assign26030_e35318, assign26030_e35318_d_n0, assign26030_e35318_d_n2, assign26030_e35318_d_n3, assign26030_e35318_d_n4, assign26030_e35318_d_n5, assign26030_e35318_d_n6, assign26030_e35318_d_n7, assign26030_e35318_d_n8, assign26030_e35318_d_n9, assign26030_e35318_d_n10, assign26030_e35318_d_n11, assign26030_e35318_d_n12, assign26030_e35318_d_n13, assign26030_e35318_d_n14,) = {
    if (locals.var_guard645 != 0.0) {
        let assign26030_e35316: f64 = (locals.var_vbs_jct / locals.var_pbsws_t);
        (assign26030_e35316, 0.0, 0.0, 0.0, (-((locals.var_vbs_jct * locals.var_pbsws_t_dn4) / (locals.var_pbsws_t * locals.var_pbsws_t))), 0.0, 0.0, (locals.var_vbs_jct_dn7 / locals.var_pbsws_t), 0.0, 0.0, 0.0, 0.0, (locals.var_vbs_jct_dn12 / locals.var_pbsws_t), 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign26030_e35318;
        locals.var_t1_dn0 = assign26030_e35318_d_n0;
        locals.var_t1_dn2 = assign26030_e35318_d_n2;
        locals.var_t1_dn3 = assign26030_e35318_d_n3;
        locals.var_t1_dn4 = assign26030_e35318_d_n4;
        locals.var_t1_dn5 = assign26030_e35318_d_n5;
        locals.var_t1_dn6 = assign26030_e35318_d_n6;
        locals.var_t1_dn7 = assign26030_e35318_d_n7;
        locals.var_t1_dn8 = assign26030_e35318_d_n8;
        locals.var_t1_dn9 = assign26030_e35318_d_n9;
        locals.var_t1_dn10 = assign26030_e35318_d_n10;
        locals.var_t1_dn11 = assign26030_e35318_d_n11;
        locals.var_t1_dn12 = assign26030_e35318_d_n12;
        locals.var_t1_dn13 = assign26030_e35318_d_n13;
        locals.var_t1_dn14 = assign26030_e35318_d_n14;
        locals.var_t1_rv = 0.0;

        let assign26040_e35321: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard646 = assign26040_e35321;
        locals.var_guard646_rv = 0.0;

        let (assign26050_e35329, assign26050_e35329_d_n0, assign26050_e35329_d_n2, assign26050_e35329_d_n3, assign26050_e35329_d_n4, assign26050_e35329_d_n5, assign26050_e35329_d_n6, assign26050_e35329_d_n7, assign26050_e35329_d_n8, assign26050_e35329_d_n9, assign26050_e35329_d_n10, assign26050_e35329_d_n11, assign26050_e35329_d_n12, assign26050_e35329_d_n13, assign26050_e35329_d_n14,) = {
    if ((locals.var_guard645 != 0.0) && (locals.var_guard646 != 0.0)) {
        let assign26050_e35327: f64 = (1.0 - locals.var_t1);
        (assign26050_e35327, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn12), (-locals.var_t1_dn13), (-locals.var_t1_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn13, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign26050_e35329;
        locals.var_arg_dn0 = assign26050_e35329_d_n0;
        locals.var_arg_dn2 = assign26050_e35329_d_n2;
        locals.var_arg_dn3 = assign26050_e35329_d_n3;
        locals.var_arg_dn4 = assign26050_e35329_d_n4;
        locals.var_arg_dn5 = assign26050_e35329_d_n5;
        locals.var_arg_dn6 = assign26050_e35329_d_n6;
        locals.var_arg_dn7 = assign26050_e35329_d_n7;
        locals.var_arg_dn8 = assign26050_e35329_d_n8;
        locals.var_arg_dn9 = assign26050_e35329_d_n9;
        locals.var_arg_dn10 = assign26050_e35329_d_n10;
        locals.var_arg_dn11 = assign26050_e35329_d_n11;
        locals.var_arg_dn12 = assign26050_e35329_d_n12;
        locals.var_arg_dn13 = assign26050_e35329_d_n13;
        locals.var_arg_dn14 = assign26050_e35329_d_n14;
        locals.var_arg_rv = 0.0;

        let assign26060_e35332: f64 = if p.p715 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard647 = assign26060_e35332;
        locals.var_guard647_rv = 0.0;

        let assign26070_e35335: f64 = if p.p715 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard648 = assign26070_e35335;
        locals.var_guard648_rv = 0.0;

        let (assign26080_e35348, assign26080_e35348_d_n0, assign26080_e35348_d_n2, assign26080_e35348_d_n3, assign26080_e35348_d_n4, assign26080_e35348_d_n5, assign26080_e35348_d_n6, assign26080_e35348_d_n7, assign26080_e35348_d_n8, assign26080_e35348_d_n9, assign26080_e35348_d_n10, assign26080_e35348_d_n11, assign26080_e35348_d_n12, assign26080_e35348_d_n13, assign26080_e35348_d_n14,) = {
    if ((((locals.var_guard645 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard647 != 0.0)) && (locals.var_guard648 != 0.0)) {
        let assign26080_e35345: f64 = (locals.var_arg).sqrt();
        let assign26080_e35346: f64 = (1.0 / assign26080_e35345);
        (assign26080_e35346, (-((locals.var_arg_dn0 / (2.0 * assign26080_e35345)) / (assign26080_e35345 * assign26080_e35345))), (-((locals.var_arg_dn2 / (2.0 * assign26080_e35345)) / (assign26080_e35345 * assign26080_e35345))), (-((locals.var_arg_dn3 / (2.0 * assign26080_e35345)) / (assign26080_e35345 * assign26080_e35345))), (-((locals.var_arg_dn4 / (2.0 * assign26080_e35345)) / (assign26080_e35345 * assign26080_e35345))), (-((locals.var_arg_dn5 / (2.0 * assign26080_e35345)) / (assign26080_e35345 * assign26080_e35345))), (-((locals.var_arg_dn6 / (2.0 * assign26080_e35345)) / (assign26080_e35345 * assign26080_e35345))), (-((locals.var_arg_dn7 / (2.0 * assign26080_e35345)) / (assign26080_e35345 * assign26080_e35345))), (-((locals.var_arg_dn8 / (2.0 * assign26080_e35345)) / (assign26080_e35345 * assign26080_e35345))), (-((locals.var_arg_dn9 / (2.0 * assign26080_e35345)) / (assign26080_e35345 * assign26080_e35345))), (-((locals.var_arg_dn10 / (2.0 * assign26080_e35345)) / (assign26080_e35345 * assign26080_e35345))), (-((locals.var_arg_dn11 / (2.0 * assign26080_e35345)) / (assign26080_e35345 * assign26080_e35345))), (-((locals.var_arg_dn12 / (2.0 * assign26080_e35345)) / (assign26080_e35345 * assign26080_e35345))), (-((locals.var_arg_dn13 / (2.0 * assign26080_e35345)) / (assign26080_e35345 * assign26080_e35345))), (-((locals.var_arg_dn14 / (2.0 * assign26080_e35345)) / (assign26080_e35345 * assign26080_e35345))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn12, locals.var_sarg_dn13, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign26080_e35348;
        locals.var_sarg_dn0 = assign26080_e35348_d_n0;
        locals.var_sarg_dn2 = assign26080_e35348_d_n2;
        locals.var_sarg_dn3 = assign26080_e35348_d_n3;
        locals.var_sarg_dn4 = assign26080_e35348_d_n4;
        locals.var_sarg_dn5 = assign26080_e35348_d_n5;
        locals.var_sarg_dn6 = assign26080_e35348_d_n6;
        locals.var_sarg_dn7 = assign26080_e35348_d_n7;
        locals.var_sarg_dn8 = assign26080_e35348_d_n8;
        locals.var_sarg_dn9 = assign26080_e35348_d_n9;
        locals.var_sarg_dn10 = assign26080_e35348_d_n10;
        locals.var_sarg_dn11 = assign26080_e35348_d_n11;
        locals.var_sarg_dn12 = assign26080_e35348_d_n12;
        locals.var_sarg_dn13 = assign26080_e35348_d_n13;
        locals.var_sarg_dn14 = assign26080_e35348_d_n14;
        locals.var_sarg_rv = 0.0;

        let (assign26090_e35364, assign26090_e35364_d_n0, assign26090_e35364_d_n2, assign26090_e35364_d_n3, assign26090_e35364_d_n4, assign26090_e35364_d_n5, assign26090_e35364_d_n6, assign26090_e35364_d_n7, assign26090_e35364_d_n8, assign26090_e35364_d_n9, assign26090_e35364_d_n10, assign26090_e35364_d_n11, assign26090_e35364_d_n12, assign26090_e35364_d_n13, assign26090_e35364_d_n14,) = {
    if ((((locals.var_guard645 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard647 != 0.0)) && (locals.var_guard648 == 0.0)) {
        let assign26090_e35358: f64 = (-p.p715);
        let assign26090_e35360: f64 = (locals.var_arg).ln();
        let assign26090_e35361: f64 = (assign26090_e35358 * assign26090_e35360);
        let assign26090_e35362: f64 = { let limited_exp_arg = assign26090_e35361; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign26090_e35362, ({ let limited_exp_arg = assign26090_e35361; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26090_e35358 * (locals.var_arg_dn0 / locals.var_arg))), ({ let limited_exp_arg = assign26090_e35361; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26090_e35358 * (locals.var_arg_dn2 / locals.var_arg))), ({ let limited_exp_arg = assign26090_e35361; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26090_e35358 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign26090_e35361; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26090_e35358 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign26090_e35361; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26090_e35358 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign26090_e35361; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26090_e35358 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign26090_e35361; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26090_e35358 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign26090_e35361; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26090_e35358 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign26090_e35361; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26090_e35358 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign26090_e35361; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26090_e35358 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign26090_e35361; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26090_e35358 * (locals.var_arg_dn11 / locals.var_arg))), ({ let limited_exp_arg = assign26090_e35361; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26090_e35358 * (locals.var_arg_dn12 / locals.var_arg))), ({ let limited_exp_arg = assign26090_e35361; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26090_e35358 * (locals.var_arg_dn13 / locals.var_arg))), ({ let limited_exp_arg = assign26090_e35361; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26090_e35358 * (locals.var_arg_dn14 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn12, locals.var_sarg_dn13, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign26090_e35364;
        locals.var_sarg_dn0 = assign26090_e35364_d_n0;
        locals.var_sarg_dn2 = assign26090_e35364_d_n2;
        locals.var_sarg_dn3 = assign26090_e35364_d_n3;
        locals.var_sarg_dn4 = assign26090_e35364_d_n4;
        locals.var_sarg_dn5 = assign26090_e35364_d_n5;
        locals.var_sarg_dn6 = assign26090_e35364_d_n6;
        locals.var_sarg_dn7 = assign26090_e35364_d_n7;
        locals.var_sarg_dn8 = assign26090_e35364_d_n8;
        locals.var_sarg_dn9 = assign26090_e35364_d_n9;
        locals.var_sarg_dn10 = assign26090_e35364_d_n10;
        locals.var_sarg_dn11 = assign26090_e35364_d_n11;
        locals.var_sarg_dn12 = assign26090_e35364_d_n12;
        locals.var_sarg_dn13 = assign26090_e35364_d_n13;
        locals.var_sarg_dn14 = assign26090_e35364_d_n14;
        locals.var_sarg_rv = 0.0;

        let (assign26100_e35384, assign26100_e35384_d_n0, assign26100_e35384_d_n2, assign26100_e35384_d_n3, assign26100_e35384_d_n4, assign26100_e35384_d_n5, assign26100_e35384_d_n6, assign26100_e35384_d_n7, assign26100_e35384_d_n8, assign26100_e35384_d_n9, assign26100_e35384_d_n10, assign26100_e35384_d_n11, assign26100_e35384_d_n12, assign26100_e35384_d_n13, assign26100_e35384_d_n14,) = {
    if (((locals.var_guard645 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard647 != 0.0)) {
        let assign26100_e35372: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign26100_e35376: f64 = (locals.var_arg * locals.var_sarg);
        let assign26100_e35377: f64 = (1.0 - assign26100_e35376);
        let assign26100_e35378: f64 = (assign26100_e35372 * assign26100_e35377);
        let assign26100_e35381: f64 = (1.0 - p.p715);
        let assign26100_e35382: f64 = (assign26100_e35378 / assign26100_e35381);
        (assign26100_e35382, ((((locals.var_pbsws_t * locals.var_czbssw_dn0) * assign26100_e35377) + (assign26100_e35372 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign26100_e35381), ((((locals.var_pbsws_t * locals.var_czbssw_dn2) * assign26100_e35377) + (assign26100_e35372 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign26100_e35381), ((((locals.var_pbsws_t * locals.var_czbssw_dn3) * assign26100_e35377) + (assign26100_e35372 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3))))) / assign26100_e35381), (((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign26100_e35377) + (assign26100_e35372 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign26100_e35381), ((((locals.var_pbsws_t * locals.var_czbssw_dn5) * assign26100_e35377) + (assign26100_e35372 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign26100_e35381), ((((locals.var_pbsws_t * locals.var_czbssw_dn6) * assign26100_e35377) + (assign26100_e35372 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign26100_e35381), ((((locals.var_pbsws_t * locals.var_czbssw_dn7) * assign26100_e35377) + (assign26100_e35372 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign26100_e35381), ((((locals.var_pbsws_t * locals.var_czbssw_dn8) * assign26100_e35377) + (assign26100_e35372 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign26100_e35381), ((((locals.var_pbsws_t * locals.var_czbssw_dn9) * assign26100_e35377) + (assign26100_e35372 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign26100_e35381), ((((locals.var_pbsws_t * locals.var_czbssw_dn10) * assign26100_e35377) + (assign26100_e35372 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign26100_e35381), ((((locals.var_pbsws_t * locals.var_czbssw_dn11) * assign26100_e35377) + (assign26100_e35372 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign26100_e35381), ((((locals.var_pbsws_t * locals.var_czbssw_dn12) * assign26100_e35377) + (assign26100_e35372 * (-((locals.var_arg_dn12 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn12))))) / assign26100_e35381), ((((locals.var_pbsws_t * locals.var_czbssw_dn13) * assign26100_e35377) + (assign26100_e35372 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign26100_e35381), ((((locals.var_pbsws_t * locals.var_czbssw_dn14) * assign26100_e35377) + (assign26100_e35372 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign26100_e35381),)
    } else {
        (locals.var_qbsj2, locals.var_qbsj2_dn0, locals.var_qbsj2_dn2, locals.var_qbsj2_dn3, locals.var_qbsj2_dn4, locals.var_qbsj2_dn5, locals.var_qbsj2_dn6, locals.var_qbsj2_dn7, locals.var_qbsj2_dn8, locals.var_qbsj2_dn9, locals.var_qbsj2_dn10, locals.var_qbsj2_dn11, locals.var_qbsj2_dn12, locals.var_qbsj2_dn13, locals.var_qbsj2_dn14,)
    }
};
        locals.var_qbsj2 = assign26100_e35384;
        locals.var_qbsj2_dn0 = assign26100_e35384_d_n0;
        locals.var_qbsj2_dn2 = assign26100_e35384_d_n2;
        locals.var_qbsj2_dn3 = assign26100_e35384_d_n3;
        locals.var_qbsj2_dn4 = assign26100_e35384_d_n4;
        locals.var_qbsj2_dn5 = assign26100_e35384_d_n5;
        locals.var_qbsj2_dn6 = assign26100_e35384_d_n6;
        locals.var_qbsj2_dn7 = assign26100_e35384_d_n7;
        locals.var_qbsj2_dn8 = assign26100_e35384_d_n8;
        locals.var_qbsj2_dn9 = assign26100_e35384_d_n9;
        locals.var_qbsj2_dn10 = assign26100_e35384_d_n10;
        locals.var_qbsj2_dn11 = assign26100_e35384_d_n11;
        locals.var_qbsj2_dn12 = assign26100_e35384_d_n12;
        locals.var_qbsj2_dn13 = assign26100_e35384_d_n13;
        locals.var_qbsj2_dn14 = assign26100_e35384_d_n14;
        locals.var_qbsj2_rv = 0.0;

        let (assign26110_e35399, assign26110_e35399_d_n0, assign26110_e35399_d_n2, assign26110_e35399_d_n3, assign26110_e35399_d_n4, assign26110_e35399_d_n5, assign26110_e35399_d_n6, assign26110_e35399_d_n7, assign26110_e35399_d_n8, assign26110_e35399_d_n9, assign26110_e35399_d_n10, assign26110_e35399_d_n11, assign26110_e35399_d_n12, assign26110_e35399_d_n13, assign26110_e35399_d_n14,) = {
    if (((locals.var_guard645 != 0.0) && (locals.var_guard646 != 0.0)) && (locals.var_guard647 == 0.0)) {
        let assign26110_e35393: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign26110_e35395: f64 = (locals.var_arg).ln();
        let assign26110_e35396: f64 = (-assign26110_e35395);
        let assign26110_e35397: f64 = (assign26110_e35393 * assign26110_e35396);
        (assign26110_e35397, (((locals.var_pbsws_t * locals.var_czbssw_dn0) * assign26110_e35396) + (assign26110_e35393 * (-(locals.var_arg_dn0 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn2) * assign26110_e35396) + (assign26110_e35393 * (-(locals.var_arg_dn2 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn3) * assign26110_e35396) + (assign26110_e35393 * (-(locals.var_arg_dn3 / locals.var_arg)))), ((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign26110_e35396) + (assign26110_e35393 * (-(locals.var_arg_dn4 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn5) * assign26110_e35396) + (assign26110_e35393 * (-(locals.var_arg_dn5 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn6) * assign26110_e35396) + (assign26110_e35393 * (-(locals.var_arg_dn6 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn7) * assign26110_e35396) + (assign26110_e35393 * (-(locals.var_arg_dn7 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn8) * assign26110_e35396) + (assign26110_e35393 * (-(locals.var_arg_dn8 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn9) * assign26110_e35396) + (assign26110_e35393 * (-(locals.var_arg_dn9 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn10) * assign26110_e35396) + (assign26110_e35393 * (-(locals.var_arg_dn10 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn11) * assign26110_e35396) + (assign26110_e35393 * (-(locals.var_arg_dn11 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn12) * assign26110_e35396) + (assign26110_e35393 * (-(locals.var_arg_dn12 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn13) * assign26110_e35396) + (assign26110_e35393 * (-(locals.var_arg_dn13 / locals.var_arg)))), (((locals.var_pbsws_t * locals.var_czbssw_dn14) * assign26110_e35396) + (assign26110_e35393 * (-(locals.var_arg_dn14 / locals.var_arg)))),)
    } else {
        (locals.var_qbsj2, locals.var_qbsj2_dn0, locals.var_qbsj2_dn2, locals.var_qbsj2_dn3, locals.var_qbsj2_dn4, locals.var_qbsj2_dn5, locals.var_qbsj2_dn6, locals.var_qbsj2_dn7, locals.var_qbsj2_dn8, locals.var_qbsj2_dn9, locals.var_qbsj2_dn10, locals.var_qbsj2_dn11, locals.var_qbsj2_dn12, locals.var_qbsj2_dn13, locals.var_qbsj2_dn14,)
    }
};
        locals.var_qbsj2 = assign26110_e35399;
        locals.var_qbsj2_dn0 = assign26110_e35399_d_n0;
        locals.var_qbsj2_dn2 = assign26110_e35399_d_n2;
        locals.var_qbsj2_dn3 = assign26110_e35399_d_n3;
        locals.var_qbsj2_dn4 = assign26110_e35399_d_n4;
        locals.var_qbsj2_dn5 = assign26110_e35399_d_n5;
        locals.var_qbsj2_dn6 = assign26110_e35399_d_n6;
        locals.var_qbsj2_dn7 = assign26110_e35399_d_n7;
        locals.var_qbsj2_dn8 = assign26110_e35399_d_n8;
        locals.var_qbsj2_dn9 = assign26110_e35399_d_n9;
        locals.var_qbsj2_dn10 = assign26110_e35399_d_n10;
        locals.var_qbsj2_dn11 = assign26110_e35399_d_n11;
        locals.var_qbsj2_dn12 = assign26110_e35399_d_n12;
        locals.var_qbsj2_dn13 = assign26110_e35399_d_n13;
        locals.var_qbsj2_dn14 = assign26110_e35399_d_n14;
        locals.var_qbsj2_rv = 0.0;

        let (assign26120_e35422, assign26120_e35422_d_n0, assign26120_e35422_d_n2, assign26120_e35422_d_n3, assign26120_e35422_d_n4, assign26120_e35422_d_n5, assign26120_e35422_d_n6, assign26120_e35422_d_n7, assign26120_e35422_d_n8, assign26120_e35422_d_n9, assign26120_e35422_d_n10, assign26120_e35422_d_n11, assign26120_e35422_d_n12, assign26120_e35422_d_n13, assign26120_e35422_d_n14,) = {
    if ((locals.var_guard645 != 0.0) && (locals.var_guard646 == 0.0)) {
        let assign26120_e35407: f64 = (locals.var_t1 - 1.0);
        let assign26120_e35408: f64 = (locals.var_czbssw_p1 * assign26120_e35407);
        let assign26120_e35411: f64 = (5.0 * p.p715);
        let assign26120_e35414: f64 = (locals.var_t1 - 1.0);
        let assign26120_e35415: f64 = (assign26120_e35411 * assign26120_e35414);
        let assign26120_e35418: f64 = (1.0 + p.p715);
        let assign26120_e35419: f64 = (assign26120_e35415 + assign26120_e35418);
        let assign26120_e35420: f64 = (assign26120_e35408 * assign26120_e35419);
        (assign26120_e35420, (((locals.var_czbssw_p1 * locals.var_t1_dn0) * assign26120_e35419) + (assign26120_e35408 * (assign26120_e35411 * locals.var_t1_dn0))), (((locals.var_czbssw_p1 * locals.var_t1_dn2) * assign26120_e35419) + (assign26120_e35408 * (assign26120_e35411 * locals.var_t1_dn2))), (((locals.var_czbssw_p1 * locals.var_t1_dn3) * assign26120_e35419) + (assign26120_e35408 * (assign26120_e35411 * locals.var_t1_dn3))), (((locals.var_czbssw_p1 * locals.var_t1_dn4) * assign26120_e35419) + (assign26120_e35408 * (assign26120_e35411 * locals.var_t1_dn4))), (((locals.var_czbssw_p1 * locals.var_t1_dn5) * assign26120_e35419) + (assign26120_e35408 * (assign26120_e35411 * locals.var_t1_dn5))), (((locals.var_czbssw_p1 * locals.var_t1_dn6) * assign26120_e35419) + (assign26120_e35408 * (assign26120_e35411 * locals.var_t1_dn6))), (((locals.var_czbssw_p1 * locals.var_t1_dn7) * assign26120_e35419) + (assign26120_e35408 * (assign26120_e35411 * locals.var_t1_dn7))), (((locals.var_czbssw_p1 * locals.var_t1_dn8) * assign26120_e35419) + (assign26120_e35408 * (assign26120_e35411 * locals.var_t1_dn8))), (((locals.var_czbssw_p1 * locals.var_t1_dn9) * assign26120_e35419) + (assign26120_e35408 * (assign26120_e35411 * locals.var_t1_dn9))), (((locals.var_czbssw_p1 * locals.var_t1_dn10) * assign26120_e35419) + (assign26120_e35408 * (assign26120_e35411 * locals.var_t1_dn10))), (((locals.var_czbssw_p1 * locals.var_t1_dn11) * assign26120_e35419) + (assign26120_e35408 * (assign26120_e35411 * locals.var_t1_dn11))), (((locals.var_czbssw_p1 * locals.var_t1_dn12) * assign26120_e35419) + (assign26120_e35408 * (assign26120_e35411 * locals.var_t1_dn12))), (((locals.var_czbssw_p1 * locals.var_t1_dn13) * assign26120_e35419) + (assign26120_e35408 * (assign26120_e35411 * locals.var_t1_dn13))), (((locals.var_czbssw_p1 * locals.var_t1_dn14) * assign26120_e35419) + (assign26120_e35408 * (assign26120_e35411 * locals.var_t1_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign26120_e35422;
        locals.var_t2_dn0 = assign26120_e35422_d_n0;
        locals.var_t2_dn2 = assign26120_e35422_d_n2;
        locals.var_t2_dn3 = assign26120_e35422_d_n3;
        locals.var_t2_dn4 = assign26120_e35422_d_n4;
        locals.var_t2_dn5 = assign26120_e35422_d_n5;
        locals.var_t2_dn6 = assign26120_e35422_d_n6;
        locals.var_t2_dn7 = assign26120_e35422_d_n7;
        locals.var_t2_dn8 = assign26120_e35422_d_n8;
        locals.var_t2_dn9 = assign26120_e35422_d_n9;
        locals.var_t2_dn10 = assign26120_e35422_d_n10;
        locals.var_t2_dn11 = assign26120_e35422_d_n11;
        locals.var_t2_dn12 = assign26120_e35422_d_n12;
        locals.var_t2_dn13 = assign26120_e35422_d_n13;
        locals.var_t2_dn14 = assign26120_e35422_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign26130_e35435, assign26130_e35435_d_n0, assign26130_e35435_d_n2, assign26130_e35435_d_n3, assign26130_e35435_d_n4, assign26130_e35435_d_n5, assign26130_e35435_d_n6, assign26130_e35435_d_n7, assign26130_e35435_d_n8, assign26130_e35435_d_n9, assign26130_e35435_d_n10, assign26130_e35435_d_n11, assign26130_e35435_d_n12, assign26130_e35435_d_n13, assign26130_e35435_d_n14,) = {
    if ((locals.var_guard645 != 0.0) && (locals.var_guard646 == 0.0)) {
        let assign26130_e35429: f64 = (locals.var_pbsws_t * locals.var_czbssw);
        let assign26130_e35432: f64 = (locals.var_t2 + locals.var_czbssw_p2);
        let assign26130_e35433: f64 = (assign26130_e35429 * assign26130_e35432);
        (assign26130_e35433, (((locals.var_pbsws_t * locals.var_czbssw_dn0) * assign26130_e35432) + (assign26130_e35429 * locals.var_t2_dn0)), (((locals.var_pbsws_t * locals.var_czbssw_dn2) * assign26130_e35432) + (assign26130_e35429 * locals.var_t2_dn2)), (((locals.var_pbsws_t * locals.var_czbssw_dn3) * assign26130_e35432) + (assign26130_e35429 * locals.var_t2_dn3)), ((((locals.var_pbsws_t_dn4 * locals.var_czbssw) + (locals.var_pbsws_t * locals.var_czbssw_dn4)) * assign26130_e35432) + (assign26130_e35429 * locals.var_t2_dn4)), (((locals.var_pbsws_t * locals.var_czbssw_dn5) * assign26130_e35432) + (assign26130_e35429 * locals.var_t2_dn5)), (((locals.var_pbsws_t * locals.var_czbssw_dn6) * assign26130_e35432) + (assign26130_e35429 * locals.var_t2_dn6)), (((locals.var_pbsws_t * locals.var_czbssw_dn7) * assign26130_e35432) + (assign26130_e35429 * locals.var_t2_dn7)), (((locals.var_pbsws_t * locals.var_czbssw_dn8) * assign26130_e35432) + (assign26130_e35429 * locals.var_t2_dn8)), (((locals.var_pbsws_t * locals.var_czbssw_dn9) * assign26130_e35432) + (assign26130_e35429 * locals.var_t2_dn9)), (((locals.var_pbsws_t * locals.var_czbssw_dn10) * assign26130_e35432) + (assign26130_e35429 * locals.var_t2_dn10)), (((locals.var_pbsws_t * locals.var_czbssw_dn11) * assign26130_e35432) + (assign26130_e35429 * locals.var_t2_dn11)), (((locals.var_pbsws_t * locals.var_czbssw_dn12) * assign26130_e35432) + (assign26130_e35429 * locals.var_t2_dn12)), (((locals.var_pbsws_t * locals.var_czbssw_dn13) * assign26130_e35432) + (assign26130_e35429 * locals.var_t2_dn13)), (((locals.var_pbsws_t * locals.var_czbssw_dn14) * assign26130_e35432) + (assign26130_e35429 * locals.var_t2_dn14)),)
    } else {
        (locals.var_qbsj2, locals.var_qbsj2_dn0, locals.var_qbsj2_dn2, locals.var_qbsj2_dn3, locals.var_qbsj2_dn4, locals.var_qbsj2_dn5, locals.var_qbsj2_dn6, locals.var_qbsj2_dn7, locals.var_qbsj2_dn8, locals.var_qbsj2_dn9, locals.var_qbsj2_dn10, locals.var_qbsj2_dn11, locals.var_qbsj2_dn12, locals.var_qbsj2_dn13, locals.var_qbsj2_dn14,)
    }
};
        locals.var_qbsj2 = assign26130_e35435;
        locals.var_qbsj2_dn0 = assign26130_e35435_d_n0;
        locals.var_qbsj2_dn2 = assign26130_e35435_d_n2;
        locals.var_qbsj2_dn3 = assign26130_e35435_d_n3;
        locals.var_qbsj2_dn4 = assign26130_e35435_d_n4;
        locals.var_qbsj2_dn5 = assign26130_e35435_d_n5;
        locals.var_qbsj2_dn6 = assign26130_e35435_d_n6;
        locals.var_qbsj2_dn7 = assign26130_e35435_d_n7;
        locals.var_qbsj2_dn8 = assign26130_e35435_d_n8;
        locals.var_qbsj2_dn9 = assign26130_e35435_d_n9;
        locals.var_qbsj2_dn10 = assign26130_e35435_d_n10;
        locals.var_qbsj2_dn11 = assign26130_e35435_d_n11;
        locals.var_qbsj2_dn12 = assign26130_e35435_d_n12;
        locals.var_qbsj2_dn13 = assign26130_e35435_d_n13;
        locals.var_qbsj2_dn14 = assign26130_e35435_d_n14;
        locals.var_qbsj2_rv = 0.0;

        let (assign26140_e35440, assign26140_e35440_d_n0, assign26140_e35440_d_n2, assign26140_e35440_d_n3, assign26140_e35440_d_n4, assign26140_e35440_d_n5, assign26140_e35440_d_n6, assign26140_e35440_d_n7, assign26140_e35440_d_n8, assign26140_e35440_d_n9, assign26140_e35440_d_n10, assign26140_e35440_d_n11, assign26140_e35440_d_n12, assign26140_e35440_d_n13, assign26140_e35440_d_n14,) = {
    if (locals.var_guard645 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbsj2, locals.var_qbsj2_dn0, locals.var_qbsj2_dn2, locals.var_qbsj2_dn3, locals.var_qbsj2_dn4, locals.var_qbsj2_dn5, locals.var_qbsj2_dn6, locals.var_qbsj2_dn7, locals.var_qbsj2_dn8, locals.var_qbsj2_dn9, locals.var_qbsj2_dn10, locals.var_qbsj2_dn11, locals.var_qbsj2_dn12, locals.var_qbsj2_dn13, locals.var_qbsj2_dn14,)
    }
};
        locals.var_qbsj2 = assign26140_e35440;
        locals.var_qbsj2_dn0 = assign26140_e35440_d_n0;
        locals.var_qbsj2_dn2 = assign26140_e35440_d_n2;
        locals.var_qbsj2_dn3 = assign26140_e35440_d_n3;
        locals.var_qbsj2_dn4 = assign26140_e35440_d_n4;
        locals.var_qbsj2_dn5 = assign26140_e35440_d_n5;
        locals.var_qbsj2_dn6 = assign26140_e35440_d_n6;
        locals.var_qbsj2_dn7 = assign26140_e35440_d_n7;
        locals.var_qbsj2_dn8 = assign26140_e35440_d_n8;
        locals.var_qbsj2_dn9 = assign26140_e35440_d_n9;
        locals.var_qbsj2_dn10 = assign26140_e35440_d_n10;
        locals.var_qbsj2_dn11 = assign26140_e35440_d_n11;
        locals.var_qbsj2_dn12 = assign26140_e35440_d_n12;
        locals.var_qbsj2_dn13 = assign26140_e35440_d_n13;
        locals.var_qbsj2_dn14 = assign26140_e35440_d_n14;
        locals.var_qbsj2_rv = 0.0;

        let assign26150_e35443: f64 = if locals.var_czbsswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard649 = assign26150_e35443;
        locals.var_guard649_rv = 0.0;

        let (assign26160_e35449, assign26160_e35449_d_n0, assign26160_e35449_d_n2, assign26160_e35449_d_n3, assign26160_e35449_d_n4, assign26160_e35449_d_n5, assign26160_e35449_d_n6, assign26160_e35449_d_n7, assign26160_e35449_d_n8, assign26160_e35449_d_n9, assign26160_e35449_d_n10, assign26160_e35449_d_n11, assign26160_e35449_d_n12, assign26160_e35449_d_n13, assign26160_e35449_d_n14,) = {
    if (locals.var_guard649 != 0.0) {
        let assign26160_e35447: f64 = (locals.var_vbs_jct / locals.var_pbswgs_t);
        (assign26160_e35447, 0.0, 0.0, 0.0, (-((locals.var_vbs_jct * locals.var_pbswgs_t_dn4) / (locals.var_pbswgs_t * locals.var_pbswgs_t))), 0.0, 0.0, (locals.var_vbs_jct_dn7 / locals.var_pbswgs_t), 0.0, 0.0, 0.0, 0.0, (locals.var_vbs_jct_dn12 / locals.var_pbswgs_t), 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign26160_e35449;
        locals.var_t1_dn0 = assign26160_e35449_d_n0;
        locals.var_t1_dn2 = assign26160_e35449_d_n2;
        locals.var_t1_dn3 = assign26160_e35449_d_n3;
        locals.var_t1_dn4 = assign26160_e35449_d_n4;
        locals.var_t1_dn5 = assign26160_e35449_d_n5;
        locals.var_t1_dn6 = assign26160_e35449_d_n6;
        locals.var_t1_dn7 = assign26160_e35449_d_n7;
        locals.var_t1_dn8 = assign26160_e35449_d_n8;
        locals.var_t1_dn9 = assign26160_e35449_d_n9;
        locals.var_t1_dn10 = assign26160_e35449_d_n10;
        locals.var_t1_dn11 = assign26160_e35449_d_n11;
        locals.var_t1_dn12 = assign26160_e35449_d_n12;
        locals.var_t1_dn13 = assign26160_e35449_d_n13;
        locals.var_t1_dn14 = assign26160_e35449_d_n14;
        locals.var_t1_rv = 0.0;

        let assign26170_e35452: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard650 = assign26170_e35452;
        locals.var_guard650_rv = 0.0;

        let (assign26180_e35460, assign26180_e35460_d_n0, assign26180_e35460_d_n2, assign26180_e35460_d_n3, assign26180_e35460_d_n4, assign26180_e35460_d_n5, assign26180_e35460_d_n6, assign26180_e35460_d_n7, assign26180_e35460_d_n8, assign26180_e35460_d_n9, assign26180_e35460_d_n10, assign26180_e35460_d_n11, assign26180_e35460_d_n12, assign26180_e35460_d_n13, assign26180_e35460_d_n14,) = {
    if ((locals.var_guard649 != 0.0) && (locals.var_guard650 != 0.0)) {
        let assign26180_e35458: f64 = (1.0 - locals.var_t1);
        (assign26180_e35458, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn12), (-locals.var_t1_dn13), (-locals.var_t1_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn13, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign26180_e35460;
        locals.var_arg_dn0 = assign26180_e35460_d_n0;
        locals.var_arg_dn2 = assign26180_e35460_d_n2;
        locals.var_arg_dn3 = assign26180_e35460_d_n3;
        locals.var_arg_dn4 = assign26180_e35460_d_n4;
        locals.var_arg_dn5 = assign26180_e35460_d_n5;
        locals.var_arg_dn6 = assign26180_e35460_d_n6;
        locals.var_arg_dn7 = assign26180_e35460_d_n7;
        locals.var_arg_dn8 = assign26180_e35460_d_n8;
        locals.var_arg_dn9 = assign26180_e35460_d_n9;
        locals.var_arg_dn10 = assign26180_e35460_d_n10;
        locals.var_arg_dn11 = assign26180_e35460_d_n11;
        locals.var_arg_dn12 = assign26180_e35460_d_n12;
        locals.var_arg_dn13 = assign26180_e35460_d_n13;
        locals.var_arg_dn14 = assign26180_e35460_d_n14;
        locals.var_arg_rv = 0.0;

        let assign26190_e35463: f64 = if p.p717 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard651 = assign26190_e35463;
        locals.var_guard651_rv = 0.0;

        let assign26200_e35466: f64 = if p.p717 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard652 = assign26200_e35466;
        locals.var_guard652_rv = 0.0;

        let (assign26210_e35479, assign26210_e35479_d_n0, assign26210_e35479_d_n2, assign26210_e35479_d_n3, assign26210_e35479_d_n4, assign26210_e35479_d_n5, assign26210_e35479_d_n6, assign26210_e35479_d_n7, assign26210_e35479_d_n8, assign26210_e35479_d_n9, assign26210_e35479_d_n10, assign26210_e35479_d_n11, assign26210_e35479_d_n12, assign26210_e35479_d_n13, assign26210_e35479_d_n14,) = {
    if ((((locals.var_guard649 != 0.0) && (locals.var_guard650 != 0.0)) && (locals.var_guard651 != 0.0)) && (locals.var_guard652 != 0.0)) {
        let assign26210_e35476: f64 = (locals.var_arg).sqrt();
        let assign26210_e35477: f64 = (1.0 / assign26210_e35476);
        (assign26210_e35477, (-((locals.var_arg_dn0 / (2.0 * assign26210_e35476)) / (assign26210_e35476 * assign26210_e35476))), (-((locals.var_arg_dn2 / (2.0 * assign26210_e35476)) / (assign26210_e35476 * assign26210_e35476))), (-((locals.var_arg_dn3 / (2.0 * assign26210_e35476)) / (assign26210_e35476 * assign26210_e35476))), (-((locals.var_arg_dn4 / (2.0 * assign26210_e35476)) / (assign26210_e35476 * assign26210_e35476))), (-((locals.var_arg_dn5 / (2.0 * assign26210_e35476)) / (assign26210_e35476 * assign26210_e35476))), (-((locals.var_arg_dn6 / (2.0 * assign26210_e35476)) / (assign26210_e35476 * assign26210_e35476))), (-((locals.var_arg_dn7 / (2.0 * assign26210_e35476)) / (assign26210_e35476 * assign26210_e35476))), (-((locals.var_arg_dn8 / (2.0 * assign26210_e35476)) / (assign26210_e35476 * assign26210_e35476))), (-((locals.var_arg_dn9 / (2.0 * assign26210_e35476)) / (assign26210_e35476 * assign26210_e35476))), (-((locals.var_arg_dn10 / (2.0 * assign26210_e35476)) / (assign26210_e35476 * assign26210_e35476))), (-((locals.var_arg_dn11 / (2.0 * assign26210_e35476)) / (assign26210_e35476 * assign26210_e35476))), (-((locals.var_arg_dn12 / (2.0 * assign26210_e35476)) / (assign26210_e35476 * assign26210_e35476))), (-((locals.var_arg_dn13 / (2.0 * assign26210_e35476)) / (assign26210_e35476 * assign26210_e35476))), (-((locals.var_arg_dn14 / (2.0 * assign26210_e35476)) / (assign26210_e35476 * assign26210_e35476))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn12, locals.var_sarg_dn13, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign26210_e35479;
        locals.var_sarg_dn0 = assign26210_e35479_d_n0;
        locals.var_sarg_dn2 = assign26210_e35479_d_n2;
        locals.var_sarg_dn3 = assign26210_e35479_d_n3;
        locals.var_sarg_dn4 = assign26210_e35479_d_n4;
        locals.var_sarg_dn5 = assign26210_e35479_d_n5;
        locals.var_sarg_dn6 = assign26210_e35479_d_n6;
        locals.var_sarg_dn7 = assign26210_e35479_d_n7;
        locals.var_sarg_dn8 = assign26210_e35479_d_n8;
        locals.var_sarg_dn9 = assign26210_e35479_d_n9;
        locals.var_sarg_dn10 = assign26210_e35479_d_n10;
        locals.var_sarg_dn11 = assign26210_e35479_d_n11;
        locals.var_sarg_dn12 = assign26210_e35479_d_n12;
        locals.var_sarg_dn13 = assign26210_e35479_d_n13;
        locals.var_sarg_dn14 = assign26210_e35479_d_n14;
        locals.var_sarg_rv = 0.0;

        let (assign26220_e35495, assign26220_e35495_d_n0, assign26220_e35495_d_n2, assign26220_e35495_d_n3, assign26220_e35495_d_n4, assign26220_e35495_d_n5, assign26220_e35495_d_n6, assign26220_e35495_d_n7, assign26220_e35495_d_n8, assign26220_e35495_d_n9, assign26220_e35495_d_n10, assign26220_e35495_d_n11, assign26220_e35495_d_n12, assign26220_e35495_d_n13, assign26220_e35495_d_n14,) = {
    if ((((locals.var_guard649 != 0.0) && (locals.var_guard650 != 0.0)) && (locals.var_guard651 != 0.0)) && (locals.var_guard652 == 0.0)) {
        let assign26220_e35489: f64 = (-p.p717);
        let assign26220_e35491: f64 = (locals.var_arg).ln();
        let assign26220_e35492: f64 = (assign26220_e35489 * assign26220_e35491);
        let assign26220_e35493: f64 = { let limited_exp_arg = assign26220_e35492; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign26220_e35493, ({ let limited_exp_arg = assign26220_e35492; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26220_e35489 * (locals.var_arg_dn0 / locals.var_arg))), ({ let limited_exp_arg = assign26220_e35492; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26220_e35489 * (locals.var_arg_dn2 / locals.var_arg))), ({ let limited_exp_arg = assign26220_e35492; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26220_e35489 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign26220_e35492; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26220_e35489 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign26220_e35492; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26220_e35489 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign26220_e35492; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26220_e35489 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign26220_e35492; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26220_e35489 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign26220_e35492; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26220_e35489 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign26220_e35492; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26220_e35489 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign26220_e35492; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26220_e35489 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign26220_e35492; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26220_e35489 * (locals.var_arg_dn11 / locals.var_arg))), ({ let limited_exp_arg = assign26220_e35492; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26220_e35489 * (locals.var_arg_dn12 / locals.var_arg))), ({ let limited_exp_arg = assign26220_e35492; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26220_e35489 * (locals.var_arg_dn13 / locals.var_arg))), ({ let limited_exp_arg = assign26220_e35492; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26220_e35489 * (locals.var_arg_dn14 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn12, locals.var_sarg_dn13, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign26220_e35495;
        locals.var_sarg_dn0 = assign26220_e35495_d_n0;
        locals.var_sarg_dn2 = assign26220_e35495_d_n2;
        locals.var_sarg_dn3 = assign26220_e35495_d_n3;
        locals.var_sarg_dn4 = assign26220_e35495_d_n4;
        locals.var_sarg_dn5 = assign26220_e35495_d_n5;
        locals.var_sarg_dn6 = assign26220_e35495_d_n6;
        locals.var_sarg_dn7 = assign26220_e35495_d_n7;
        locals.var_sarg_dn8 = assign26220_e35495_d_n8;
        locals.var_sarg_dn9 = assign26220_e35495_d_n9;
        locals.var_sarg_dn10 = assign26220_e35495_d_n10;
        locals.var_sarg_dn11 = assign26220_e35495_d_n11;
        locals.var_sarg_dn12 = assign26220_e35495_d_n12;
        locals.var_sarg_dn13 = assign26220_e35495_d_n13;
        locals.var_sarg_dn14 = assign26220_e35495_d_n14;
        locals.var_sarg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_78(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26230_e35515, assign26230_e35515_d_n0, assign26230_e35515_d_n2, assign26230_e35515_d_n3, assign26230_e35515_d_n4, assign26230_e35515_d_n5, assign26230_e35515_d_n6, assign26230_e35515_d_n7, assign26230_e35515_d_n8, assign26230_e35515_d_n9, assign26230_e35515_d_n10, assign26230_e35515_d_n11, assign26230_e35515_d_n12, assign26230_e35515_d_n13, assign26230_e35515_d_n14,) = {
    if (((locals.var_guard649 != 0.0) && (locals.var_guard650 != 0.0)) && (locals.var_guard651 != 0.0)) {
        let assign26230_e35503: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign26230_e35507: f64 = (locals.var_arg * locals.var_sarg);
        let assign26230_e35508: f64 = (1.0 - assign26230_e35507);
        let assign26230_e35509: f64 = (assign26230_e35503 * assign26230_e35508);
        let assign26230_e35512: f64 = (1.0 - p.p717);
        let assign26230_e35513: f64 = (assign26230_e35509 / assign26230_e35512);
        (assign26230_e35513, ((assign26230_e35503 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0)))) / assign26230_e35512), ((assign26230_e35503 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2)))) / assign26230_e35512), ((assign26230_e35503 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3)))) / assign26230_e35512), (((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign26230_e35508) + (assign26230_e35503 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign26230_e35512), ((assign26230_e35503 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5)))) / assign26230_e35512), ((assign26230_e35503 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6)))) / assign26230_e35512), ((assign26230_e35503 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7)))) / assign26230_e35512), ((assign26230_e35503 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8)))) / assign26230_e35512), ((assign26230_e35503 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9)))) / assign26230_e35512), ((assign26230_e35503 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10)))) / assign26230_e35512), ((assign26230_e35503 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11)))) / assign26230_e35512), ((assign26230_e35503 * (-((locals.var_arg_dn12 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn12)))) / assign26230_e35512), ((assign26230_e35503 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13)))) / assign26230_e35512), ((assign26230_e35503 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14)))) / assign26230_e35512),)
    } else {
        (locals.var_qbsj3, locals.var_qbsj3_dn0, locals.var_qbsj3_dn2, locals.var_qbsj3_dn3, locals.var_qbsj3_dn4, locals.var_qbsj3_dn5, locals.var_qbsj3_dn6, locals.var_qbsj3_dn7, locals.var_qbsj3_dn8, locals.var_qbsj3_dn9, locals.var_qbsj3_dn10, locals.var_qbsj3_dn11, locals.var_qbsj3_dn12, locals.var_qbsj3_dn13, locals.var_qbsj3_dn14,)
    }
};
        locals.var_qbsj3 = assign26230_e35515;
        locals.var_qbsj3_dn0 = assign26230_e35515_d_n0;
        locals.var_qbsj3_dn2 = assign26230_e35515_d_n2;
        locals.var_qbsj3_dn3 = assign26230_e35515_d_n3;
        locals.var_qbsj3_dn4 = assign26230_e35515_d_n4;
        locals.var_qbsj3_dn5 = assign26230_e35515_d_n5;
        locals.var_qbsj3_dn6 = assign26230_e35515_d_n6;
        locals.var_qbsj3_dn7 = assign26230_e35515_d_n7;
        locals.var_qbsj3_dn8 = assign26230_e35515_d_n8;
        locals.var_qbsj3_dn9 = assign26230_e35515_d_n9;
        locals.var_qbsj3_dn10 = assign26230_e35515_d_n10;
        locals.var_qbsj3_dn11 = assign26230_e35515_d_n11;
        locals.var_qbsj3_dn12 = assign26230_e35515_d_n12;
        locals.var_qbsj3_dn13 = assign26230_e35515_d_n13;
        locals.var_qbsj3_dn14 = assign26230_e35515_d_n14;
        locals.var_qbsj3_rv = 0.0;

        let (assign26240_e35530, assign26240_e35530_d_n0, assign26240_e35530_d_n2, assign26240_e35530_d_n3, assign26240_e35530_d_n4, assign26240_e35530_d_n5, assign26240_e35530_d_n6, assign26240_e35530_d_n7, assign26240_e35530_d_n8, assign26240_e35530_d_n9, assign26240_e35530_d_n10, assign26240_e35530_d_n11, assign26240_e35530_d_n12, assign26240_e35530_d_n13, assign26240_e35530_d_n14,) = {
    if (((locals.var_guard649 != 0.0) && (locals.var_guard650 != 0.0)) && (locals.var_guard651 == 0.0)) {
        let assign26240_e35524: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign26240_e35526: f64 = (locals.var_arg).ln();
        let assign26240_e35527: f64 = (-assign26240_e35526);
        let assign26240_e35528: f64 = (assign26240_e35524 * assign26240_e35527);
        (assign26240_e35528, (assign26240_e35524 * (-(locals.var_arg_dn0 / locals.var_arg))), (assign26240_e35524 * (-(locals.var_arg_dn2 / locals.var_arg))), (assign26240_e35524 * (-(locals.var_arg_dn3 / locals.var_arg))), ((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign26240_e35527) + (assign26240_e35524 * (-(locals.var_arg_dn4 / locals.var_arg)))), (assign26240_e35524 * (-(locals.var_arg_dn5 / locals.var_arg))), (assign26240_e35524 * (-(locals.var_arg_dn6 / locals.var_arg))), (assign26240_e35524 * (-(locals.var_arg_dn7 / locals.var_arg))), (assign26240_e35524 * (-(locals.var_arg_dn8 / locals.var_arg))), (assign26240_e35524 * (-(locals.var_arg_dn9 / locals.var_arg))), (assign26240_e35524 * (-(locals.var_arg_dn10 / locals.var_arg))), (assign26240_e35524 * (-(locals.var_arg_dn11 / locals.var_arg))), (assign26240_e35524 * (-(locals.var_arg_dn12 / locals.var_arg))), (assign26240_e35524 * (-(locals.var_arg_dn13 / locals.var_arg))), (assign26240_e35524 * (-(locals.var_arg_dn14 / locals.var_arg))),)
    } else {
        (locals.var_qbsj3, locals.var_qbsj3_dn0, locals.var_qbsj3_dn2, locals.var_qbsj3_dn3, locals.var_qbsj3_dn4, locals.var_qbsj3_dn5, locals.var_qbsj3_dn6, locals.var_qbsj3_dn7, locals.var_qbsj3_dn8, locals.var_qbsj3_dn9, locals.var_qbsj3_dn10, locals.var_qbsj3_dn11, locals.var_qbsj3_dn12, locals.var_qbsj3_dn13, locals.var_qbsj3_dn14,)
    }
};
        locals.var_qbsj3 = assign26240_e35530;
        locals.var_qbsj3_dn0 = assign26240_e35530_d_n0;
        locals.var_qbsj3_dn2 = assign26240_e35530_d_n2;
        locals.var_qbsj3_dn3 = assign26240_e35530_d_n3;
        locals.var_qbsj3_dn4 = assign26240_e35530_d_n4;
        locals.var_qbsj3_dn5 = assign26240_e35530_d_n5;
        locals.var_qbsj3_dn6 = assign26240_e35530_d_n6;
        locals.var_qbsj3_dn7 = assign26240_e35530_d_n7;
        locals.var_qbsj3_dn8 = assign26240_e35530_d_n8;
        locals.var_qbsj3_dn9 = assign26240_e35530_d_n9;
        locals.var_qbsj3_dn10 = assign26240_e35530_d_n10;
        locals.var_qbsj3_dn11 = assign26240_e35530_d_n11;
        locals.var_qbsj3_dn12 = assign26240_e35530_d_n12;
        locals.var_qbsj3_dn13 = assign26240_e35530_d_n13;
        locals.var_qbsj3_dn14 = assign26240_e35530_d_n14;
        locals.var_qbsj3_rv = 0.0;

        let (assign26250_e35553, assign26250_e35553_d_n0, assign26250_e35553_d_n2, assign26250_e35553_d_n3, assign26250_e35553_d_n4, assign26250_e35553_d_n5, assign26250_e35553_d_n6, assign26250_e35553_d_n7, assign26250_e35553_d_n8, assign26250_e35553_d_n9, assign26250_e35553_d_n10, assign26250_e35553_d_n11, assign26250_e35553_d_n12, assign26250_e35553_d_n13, assign26250_e35553_d_n14,) = {
    if ((locals.var_guard649 != 0.0) && (locals.var_guard650 == 0.0)) {
        let assign26250_e35538: f64 = (locals.var_t1 - 1.0);
        let assign26250_e35539: f64 = (locals.var_czbsswg_p1 * assign26250_e35538);
        let assign26250_e35542: f64 = (5.0 * p.p717);
        let assign26250_e35545: f64 = (locals.var_t1 - 1.0);
        let assign26250_e35546: f64 = (assign26250_e35542 * assign26250_e35545);
        let assign26250_e35549: f64 = (1.0 + p.p717);
        let assign26250_e35550: f64 = (assign26250_e35546 + assign26250_e35549);
        let assign26250_e35551: f64 = (assign26250_e35539 * assign26250_e35550);
        (assign26250_e35551, (((locals.var_czbsswg_p1 * locals.var_t1_dn0) * assign26250_e35550) + (assign26250_e35539 * (assign26250_e35542 * locals.var_t1_dn0))), (((locals.var_czbsswg_p1 * locals.var_t1_dn2) * assign26250_e35550) + (assign26250_e35539 * (assign26250_e35542 * locals.var_t1_dn2))), (((locals.var_czbsswg_p1 * locals.var_t1_dn3) * assign26250_e35550) + (assign26250_e35539 * (assign26250_e35542 * locals.var_t1_dn3))), (((locals.var_czbsswg_p1 * locals.var_t1_dn4) * assign26250_e35550) + (assign26250_e35539 * (assign26250_e35542 * locals.var_t1_dn4))), (((locals.var_czbsswg_p1 * locals.var_t1_dn5) * assign26250_e35550) + (assign26250_e35539 * (assign26250_e35542 * locals.var_t1_dn5))), (((locals.var_czbsswg_p1 * locals.var_t1_dn6) * assign26250_e35550) + (assign26250_e35539 * (assign26250_e35542 * locals.var_t1_dn6))), (((locals.var_czbsswg_p1 * locals.var_t1_dn7) * assign26250_e35550) + (assign26250_e35539 * (assign26250_e35542 * locals.var_t1_dn7))), (((locals.var_czbsswg_p1 * locals.var_t1_dn8) * assign26250_e35550) + (assign26250_e35539 * (assign26250_e35542 * locals.var_t1_dn8))), (((locals.var_czbsswg_p1 * locals.var_t1_dn9) * assign26250_e35550) + (assign26250_e35539 * (assign26250_e35542 * locals.var_t1_dn9))), (((locals.var_czbsswg_p1 * locals.var_t1_dn10) * assign26250_e35550) + (assign26250_e35539 * (assign26250_e35542 * locals.var_t1_dn10))), (((locals.var_czbsswg_p1 * locals.var_t1_dn11) * assign26250_e35550) + (assign26250_e35539 * (assign26250_e35542 * locals.var_t1_dn11))), (((locals.var_czbsswg_p1 * locals.var_t1_dn12) * assign26250_e35550) + (assign26250_e35539 * (assign26250_e35542 * locals.var_t1_dn12))), (((locals.var_czbsswg_p1 * locals.var_t1_dn13) * assign26250_e35550) + (assign26250_e35539 * (assign26250_e35542 * locals.var_t1_dn13))), (((locals.var_czbsswg_p1 * locals.var_t1_dn14) * assign26250_e35550) + (assign26250_e35539 * (assign26250_e35542 * locals.var_t1_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign26250_e35553;
        locals.var_t2_dn0 = assign26250_e35553_d_n0;
        locals.var_t2_dn2 = assign26250_e35553_d_n2;
        locals.var_t2_dn3 = assign26250_e35553_d_n3;
        locals.var_t2_dn4 = assign26250_e35553_d_n4;
        locals.var_t2_dn5 = assign26250_e35553_d_n5;
        locals.var_t2_dn6 = assign26250_e35553_d_n6;
        locals.var_t2_dn7 = assign26250_e35553_d_n7;
        locals.var_t2_dn8 = assign26250_e35553_d_n8;
        locals.var_t2_dn9 = assign26250_e35553_d_n9;
        locals.var_t2_dn10 = assign26250_e35553_d_n10;
        locals.var_t2_dn11 = assign26250_e35553_d_n11;
        locals.var_t2_dn12 = assign26250_e35553_d_n12;
        locals.var_t2_dn13 = assign26250_e35553_d_n13;
        locals.var_t2_dn14 = assign26250_e35553_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign26260_e35566, assign26260_e35566_d_n0, assign26260_e35566_d_n2, assign26260_e35566_d_n3, assign26260_e35566_d_n4, assign26260_e35566_d_n5, assign26260_e35566_d_n6, assign26260_e35566_d_n7, assign26260_e35566_d_n8, assign26260_e35566_d_n9, assign26260_e35566_d_n10, assign26260_e35566_d_n11, assign26260_e35566_d_n12, assign26260_e35566_d_n13, assign26260_e35566_d_n14,) = {
    if ((locals.var_guard649 != 0.0) && (locals.var_guard650 == 0.0)) {
        let assign26260_e35560: f64 = (locals.var_pbswgs_t * locals.var_czbsswg);
        let assign26260_e35563: f64 = (locals.var_t2 + locals.var_czbsswg_p2);
        let assign26260_e35564: f64 = (assign26260_e35560 * assign26260_e35563);
        (assign26260_e35564, (assign26260_e35560 * locals.var_t2_dn0), (assign26260_e35560 * locals.var_t2_dn2), (assign26260_e35560 * locals.var_t2_dn3), ((((locals.var_pbswgs_t_dn4 * locals.var_czbsswg) + (locals.var_pbswgs_t * locals.var_czbsswg_dn4)) * assign26260_e35563) + (assign26260_e35560 * locals.var_t2_dn4)), (assign26260_e35560 * locals.var_t2_dn5), (assign26260_e35560 * locals.var_t2_dn6), (assign26260_e35560 * locals.var_t2_dn7), (assign26260_e35560 * locals.var_t2_dn8), (assign26260_e35560 * locals.var_t2_dn9), (assign26260_e35560 * locals.var_t2_dn10), (assign26260_e35560 * locals.var_t2_dn11), (assign26260_e35560 * locals.var_t2_dn12), (assign26260_e35560 * locals.var_t2_dn13), (assign26260_e35560 * locals.var_t2_dn14),)
    } else {
        (locals.var_qbsj3, locals.var_qbsj3_dn0, locals.var_qbsj3_dn2, locals.var_qbsj3_dn3, locals.var_qbsj3_dn4, locals.var_qbsj3_dn5, locals.var_qbsj3_dn6, locals.var_qbsj3_dn7, locals.var_qbsj3_dn8, locals.var_qbsj3_dn9, locals.var_qbsj3_dn10, locals.var_qbsj3_dn11, locals.var_qbsj3_dn12, locals.var_qbsj3_dn13, locals.var_qbsj3_dn14,)
    }
};
        locals.var_qbsj3 = assign26260_e35566;
        locals.var_qbsj3_dn0 = assign26260_e35566_d_n0;
        locals.var_qbsj3_dn2 = assign26260_e35566_d_n2;
        locals.var_qbsj3_dn3 = assign26260_e35566_d_n3;
        locals.var_qbsj3_dn4 = assign26260_e35566_d_n4;
        locals.var_qbsj3_dn5 = assign26260_e35566_d_n5;
        locals.var_qbsj3_dn6 = assign26260_e35566_d_n6;
        locals.var_qbsj3_dn7 = assign26260_e35566_d_n7;
        locals.var_qbsj3_dn8 = assign26260_e35566_d_n8;
        locals.var_qbsj3_dn9 = assign26260_e35566_d_n9;
        locals.var_qbsj3_dn10 = assign26260_e35566_d_n10;
        locals.var_qbsj3_dn11 = assign26260_e35566_d_n11;
        locals.var_qbsj3_dn12 = assign26260_e35566_d_n12;
        locals.var_qbsj3_dn13 = assign26260_e35566_d_n13;
        locals.var_qbsj3_dn14 = assign26260_e35566_d_n14;
        locals.var_qbsj3_rv = 0.0;

        let (assign26270_e35571, assign26270_e35571_d_n0, assign26270_e35571_d_n2, assign26270_e35571_d_n3, assign26270_e35571_d_n4, assign26270_e35571_d_n5, assign26270_e35571_d_n6, assign26270_e35571_d_n7, assign26270_e35571_d_n8, assign26270_e35571_d_n9, assign26270_e35571_d_n10, assign26270_e35571_d_n11, assign26270_e35571_d_n12, assign26270_e35571_d_n13, assign26270_e35571_d_n14,) = {
    if (locals.var_guard649 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbsj3, locals.var_qbsj3_dn0, locals.var_qbsj3_dn2, locals.var_qbsj3_dn3, locals.var_qbsj3_dn4, locals.var_qbsj3_dn5, locals.var_qbsj3_dn6, locals.var_qbsj3_dn7, locals.var_qbsj3_dn8, locals.var_qbsj3_dn9, locals.var_qbsj3_dn10, locals.var_qbsj3_dn11, locals.var_qbsj3_dn12, locals.var_qbsj3_dn13, locals.var_qbsj3_dn14,)
    }
};
        locals.var_qbsj3 = assign26270_e35571;
        locals.var_qbsj3_dn0 = assign26270_e35571_d_n0;
        locals.var_qbsj3_dn2 = assign26270_e35571_d_n2;
        locals.var_qbsj3_dn3 = assign26270_e35571_d_n3;
        locals.var_qbsj3_dn4 = assign26270_e35571_d_n4;
        locals.var_qbsj3_dn5 = assign26270_e35571_d_n5;
        locals.var_qbsj3_dn6 = assign26270_e35571_d_n6;
        locals.var_qbsj3_dn7 = assign26270_e35571_d_n7;
        locals.var_qbsj3_dn8 = assign26270_e35571_d_n8;
        locals.var_qbsj3_dn9 = assign26270_e35571_d_n9;
        locals.var_qbsj3_dn10 = assign26270_e35571_d_n10;
        locals.var_qbsj3_dn11 = assign26270_e35571_d_n11;
        locals.var_qbsj3_dn12 = assign26270_e35571_d_n12;
        locals.var_qbsj3_dn13 = assign26270_e35571_d_n13;
        locals.var_qbsj3_dn14 = assign26270_e35571_d_n14;
        locals.var_qbsj3_rv = 0.0;

        let assign26280_e35574: f64 = (locals.var_qbsj1 + locals.var_qbsj2);
        let assign26280_e35576: f64 = (assign26280_e35574 + locals.var_qbsj3);
        locals.var_qbsj = assign26280_e35576;
        locals.var_qbsj_dn0 = ((locals.var_qbsj1_dn0 + locals.var_qbsj2_dn0) + locals.var_qbsj3_dn0);
        locals.var_qbsj_dn2 = ((locals.var_qbsj1_dn2 + locals.var_qbsj2_dn2) + locals.var_qbsj3_dn2);
        locals.var_qbsj_dn3 = ((locals.var_qbsj1_dn3 + locals.var_qbsj2_dn3) + locals.var_qbsj3_dn3);
        locals.var_qbsj_dn4 = ((locals.var_qbsj1_dn4 + locals.var_qbsj2_dn4) + locals.var_qbsj3_dn4);
        locals.var_qbsj_dn5 = ((locals.var_qbsj1_dn5 + locals.var_qbsj2_dn5) + locals.var_qbsj3_dn5);
        locals.var_qbsj_dn6 = ((locals.var_qbsj1_dn6 + locals.var_qbsj2_dn6) + locals.var_qbsj3_dn6);
        locals.var_qbsj_dn7 = ((locals.var_qbsj1_dn7 + locals.var_qbsj2_dn7) + locals.var_qbsj3_dn7);
        locals.var_qbsj_dn8 = ((locals.var_qbsj1_dn8 + locals.var_qbsj2_dn8) + locals.var_qbsj3_dn8);
        locals.var_qbsj_dn9 = ((locals.var_qbsj1_dn9 + locals.var_qbsj2_dn9) + locals.var_qbsj3_dn9);
        locals.var_qbsj_dn10 = ((locals.var_qbsj1_dn10 + locals.var_qbsj2_dn10) + locals.var_qbsj3_dn10);
        locals.var_qbsj_dn11 = ((locals.var_qbsj1_dn11 + locals.var_qbsj2_dn11) + locals.var_qbsj3_dn11);
        locals.var_qbsj_dn12 = ((locals.var_qbsj1_dn12 + locals.var_qbsj2_dn12) + locals.var_qbsj3_dn12);
        locals.var_qbsj_dn13 = ((locals.var_qbsj1_dn13 + locals.var_qbsj2_dn13) + locals.var_qbsj3_dn13);
        locals.var_qbsj_dn14 = ((locals.var_qbsj1_dn14 + locals.var_qbsj2_dn14) + locals.var_qbsj3_dn14);
        locals.var_qbsj_rv = 0.0;

        let assign26290_e35579: f64 = (locals.var_oneminusxpart * locals.var_cjd_t);
        let assign26290_e35581: f64 = (assign26290_e35579 * locals.var_adeff);
        locals.var_czbd = assign26290_e35581;
        locals.var_czbd_dn0 = (assign26290_e35579 * locals.var_adeff_dn0);
        locals.var_czbd_dn2 = (assign26290_e35579 * locals.var_adeff_dn2);
        locals.var_czbd_dn3 = (assign26290_e35579 * locals.var_adeff_dn3);
        locals.var_czbd_dn4 = (((locals.var_oneminusxpart * locals.var_cjd_t_dn4) * locals.var_adeff) + (assign26290_e35579 * locals.var_adeff_dn4));
        locals.var_czbd_dn5 = (assign26290_e35579 * locals.var_adeff_dn5);
        locals.var_czbd_dn6 = (assign26290_e35579 * locals.var_adeff_dn6);
        locals.var_czbd_dn7 = (assign26290_e35579 * locals.var_adeff_dn7);
        locals.var_czbd_dn8 = (assign26290_e35579 * locals.var_adeff_dn8);
        locals.var_czbd_dn9 = (assign26290_e35579 * locals.var_adeff_dn9);
        locals.var_czbd_dn10 = (assign26290_e35579 * locals.var_adeff_dn10);
        locals.var_czbd_dn11 = (assign26290_e35579 * locals.var_adeff_dn11);
        locals.var_czbd_dn12 = (assign26290_e35579 * locals.var_adeff_dn12);
        locals.var_czbd_dn13 = (assign26290_e35579 * locals.var_adeff_dn13);
        locals.var_czbd_dn14 = (assign26290_e35579 * locals.var_adeff_dn14);
        locals.var_czbd_rv = 0.0;

        let assign26300_e35585: f64 = (locals.var_weffcj * p.p2);
        let assign26300_e35586: f64 = if locals.var_pdeff > assign26300_e35585 { 1.0 } else { 0.0 };
        locals.var_guard653 = assign26300_e35586;
        locals.var_guard653_rv = 0.0;

        let assign26310_e35593: f64 = if ((p.p1128 > 0.0) && (p.p1097 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard654 = assign26310_e35593;
        locals.var_guard654_rv = 0.0;

        let (assign26320_e35607, assign26320_e35607_d_n0, assign26320_e35607_d_n2, assign26320_e35607_d_n3, assign26320_e35607_d_n4, assign26320_e35607_d_n5, assign26320_e35607_d_n6, assign26320_e35607_d_n7, assign26320_e35607_d_n8, assign26320_e35607_d_n9, assign26320_e35607_d_n10, assign26320_e35607_d_n11, assign26320_e35607_d_n12, assign26320_e35607_d_n13, assign26320_e35607_d_n14,) = {
    if ((locals.var_guard653 != 0.0) && (locals.var_guard654 != 0.0)) {
        let assign26320_e35599: f64 = (locals.var_oneminusxpart * locals.var_cjswd_t);
        let assign26320_e35603: f64 = (locals.var_weffcj * p.p2);
        let assign26320_e35604: f64 = (locals.var_pdeff - assign26320_e35603);
        let assign26320_e35605: f64 = (assign26320_e35599 * assign26320_e35604);
        (assign26320_e35605, (assign26320_e35599 * locals.var_pdeff_dn0), (assign26320_e35599 * locals.var_pdeff_dn2), (assign26320_e35599 * locals.var_pdeff_dn3), (((locals.var_oneminusxpart * locals.var_cjswd_t_dn4) * assign26320_e35604) + (assign26320_e35599 * locals.var_pdeff_dn4)), (assign26320_e35599 * locals.var_pdeff_dn5), (assign26320_e35599 * locals.var_pdeff_dn6), (assign26320_e35599 * locals.var_pdeff_dn7), (assign26320_e35599 * locals.var_pdeff_dn8), (assign26320_e35599 * locals.var_pdeff_dn9), (assign26320_e35599 * locals.var_pdeff_dn10), (assign26320_e35599 * locals.var_pdeff_dn11), (assign26320_e35599 * locals.var_pdeff_dn12), (assign26320_e35599 * locals.var_pdeff_dn13), (assign26320_e35599 * locals.var_pdeff_dn14),)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn3, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn12, locals.var_czbdsw_dn13, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign26320_e35607;
        locals.var_czbdsw_dn0 = assign26320_e35607_d_n0;
        locals.var_czbdsw_dn2 = assign26320_e35607_d_n2;
        locals.var_czbdsw_dn3 = assign26320_e35607_d_n3;
        locals.var_czbdsw_dn4 = assign26320_e35607_d_n4;
        locals.var_czbdsw_dn5 = assign26320_e35607_d_n5;
        locals.var_czbdsw_dn6 = assign26320_e35607_d_n6;
        locals.var_czbdsw_dn7 = assign26320_e35607_d_n7;
        locals.var_czbdsw_dn8 = assign26320_e35607_d_n8;
        locals.var_czbdsw_dn9 = assign26320_e35607_d_n9;
        locals.var_czbdsw_dn10 = assign26320_e35607_d_n10;
        locals.var_czbdsw_dn11 = assign26320_e35607_d_n11;
        locals.var_czbdsw_dn12 = assign26320_e35607_d_n12;
        locals.var_czbdsw_dn13 = assign26320_e35607_d_n13;
        locals.var_czbdsw_dn14 = assign26320_e35607_d_n14;
        locals.var_czbdsw_rv = 0.0;

        let (assign26330_e35618, assign26330_e35618_d_n0, assign26330_e35618_d_n2, assign26330_e35618_d_n3, assign26330_e35618_d_n4, assign26330_e35618_d_n5, assign26330_e35618_d_n6, assign26330_e35618_d_n7, assign26330_e35618_d_n8, assign26330_e35618_d_n9, assign26330_e35618_d_n10, assign26330_e35618_d_n11, assign26330_e35618_d_n12, assign26330_e35618_d_n13, assign26330_e35618_d_n14,) = {
    if ((locals.var_guard653 != 0.0) && (locals.var_guard654 == 0.0)) {
        let assign26330_e35614: f64 = (locals.var_oneminusxpart * locals.var_cjswd_t);
        let assign26330_e35616: f64 = (assign26330_e35614 * locals.var_pdeff);
        (assign26330_e35616, (assign26330_e35614 * locals.var_pdeff_dn0), (assign26330_e35614 * locals.var_pdeff_dn2), (assign26330_e35614 * locals.var_pdeff_dn3), (((locals.var_oneminusxpart * locals.var_cjswd_t_dn4) * locals.var_pdeff) + (assign26330_e35614 * locals.var_pdeff_dn4)), (assign26330_e35614 * locals.var_pdeff_dn5), (assign26330_e35614 * locals.var_pdeff_dn6), (assign26330_e35614 * locals.var_pdeff_dn7), (assign26330_e35614 * locals.var_pdeff_dn8), (assign26330_e35614 * locals.var_pdeff_dn9), (assign26330_e35614 * locals.var_pdeff_dn10), (assign26330_e35614 * locals.var_pdeff_dn11), (assign26330_e35614 * locals.var_pdeff_dn12), (assign26330_e35614 * locals.var_pdeff_dn13), (assign26330_e35614 * locals.var_pdeff_dn14),)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn3, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn12, locals.var_czbdsw_dn13, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign26330_e35618;
        locals.var_czbdsw_dn0 = assign26330_e35618_d_n0;
        locals.var_czbdsw_dn2 = assign26330_e35618_d_n2;
        locals.var_czbdsw_dn3 = assign26330_e35618_d_n3;
        locals.var_czbdsw_dn4 = assign26330_e35618_d_n4;
        locals.var_czbdsw_dn5 = assign26330_e35618_d_n5;
        locals.var_czbdsw_dn6 = assign26330_e35618_d_n6;
        locals.var_czbdsw_dn7 = assign26330_e35618_d_n7;
        locals.var_czbdsw_dn8 = assign26330_e35618_d_n8;
        locals.var_czbdsw_dn9 = assign26330_e35618_d_n9;
        locals.var_czbdsw_dn10 = assign26330_e35618_d_n10;
        locals.var_czbdsw_dn11 = assign26330_e35618_d_n11;
        locals.var_czbdsw_dn12 = assign26330_e35618_d_n12;
        locals.var_czbdsw_dn13 = assign26330_e35618_d_n13;
        locals.var_czbdsw_dn14 = assign26330_e35618_d_n14;
        locals.var_czbdsw_rv = 0.0;

        let (assign26340_e35627, assign26340_e35627_d_n0, assign26340_e35627_d_n2, assign26340_e35627_d_n3, assign26340_e35627_d_n4, assign26340_e35627_d_n5, assign26340_e35627_d_n6, assign26340_e35627_d_n7, assign26340_e35627_d_n8, assign26340_e35627_d_n9, assign26340_e35627_d_n10, assign26340_e35627_d_n11, assign26340_e35627_d_n12, assign26340_e35627_d_n13, assign26340_e35627_d_n14,) = {
    if (locals.var_guard653 == 0.0) {
        let assign26340_e35623: f64 = (locals.var_oneminusxpart * locals.var_cjswd_t);
        let assign26340_e35625: f64 = (assign26340_e35623 * locals.var_pdeff);
        (assign26340_e35625, (assign26340_e35623 * locals.var_pdeff_dn0), (assign26340_e35623 * locals.var_pdeff_dn2), (assign26340_e35623 * locals.var_pdeff_dn3), (((locals.var_oneminusxpart * locals.var_cjswd_t_dn4) * locals.var_pdeff) + (assign26340_e35623 * locals.var_pdeff_dn4)), (assign26340_e35623 * locals.var_pdeff_dn5), (assign26340_e35623 * locals.var_pdeff_dn6), (assign26340_e35623 * locals.var_pdeff_dn7), (assign26340_e35623 * locals.var_pdeff_dn8), (assign26340_e35623 * locals.var_pdeff_dn9), (assign26340_e35623 * locals.var_pdeff_dn10), (assign26340_e35623 * locals.var_pdeff_dn11), (assign26340_e35623 * locals.var_pdeff_dn12), (assign26340_e35623 * locals.var_pdeff_dn13), (assign26340_e35623 * locals.var_pdeff_dn14),)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn3, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn12, locals.var_czbdsw_dn13, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign26340_e35627;
        locals.var_czbdsw_dn0 = assign26340_e35627_d_n0;
        locals.var_czbdsw_dn2 = assign26340_e35627_d_n2;
        locals.var_czbdsw_dn3 = assign26340_e35627_d_n3;
        locals.var_czbdsw_dn4 = assign26340_e35627_d_n4;
        locals.var_czbdsw_dn5 = assign26340_e35627_d_n5;
        locals.var_czbdsw_dn6 = assign26340_e35627_d_n6;
        locals.var_czbdsw_dn7 = assign26340_e35627_d_n7;
        locals.var_czbdsw_dn8 = assign26340_e35627_d_n8;
        locals.var_czbdsw_dn9 = assign26340_e35627_d_n9;
        locals.var_czbdsw_dn10 = assign26340_e35627_d_n10;
        locals.var_czbdsw_dn11 = assign26340_e35627_d_n11;
        locals.var_czbdsw_dn12 = assign26340_e35627_d_n12;
        locals.var_czbdsw_dn13 = assign26340_e35627_d_n13;
        locals.var_czbdsw_dn14 = assign26340_e35627_d_n14;
        locals.var_czbdsw_rv = 0.0;

        let assign26350_e35630: f64 = (locals.var_cjswgd_t * locals.var_weffcj);
        let assign26350_e35632: f64 = (assign26350_e35630 * p.p2);
        locals.var_czbdswg = assign26350_e35632;
        locals.var_czbdswg_dn4 = ((locals.var_cjswgd_t_dn4 * locals.var_weffcj) * p.p2);
        locals.var_czbdswg_rv = 0.0;

        let assign26360_e35635: f64 = (-p.p714);
        let assign26360_e35636: f64 = (0.1_f64).powf(assign26360_e35635);
        locals.var_czbd_p1 = assign26360_e35636;
        locals.var_czbd_p1_rv = 0.0;

        let assign26370_e35639: f64 = if p.p714 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard655 = assign26370_e35639;
        locals.var_guard655_rv = 0.0;

        let (assign26380_e35646,) = {
    if (locals.var_guard655 != 0.0) {
        let assign26380_e35643: f64 = (0.1_f64).ln();
        let assign26380_e35644: f64 = (1.5 - assign26380_e35643);
        (assign26380_e35644,)
    } else {
        (locals.var_czbd_p2,)
    }
};
        locals.var_czbd_p2 = assign26380_e35646;
        locals.var_czbd_p2_rv = 0.0;

        let (assign26390_e35667,) = {
    if (locals.var_guard655 == 0.0) {
        let assign26390_e35652: f64 = (1.0 - p.p714);
        let assign26390_e35653: f64 = (1.0 / assign26390_e35652);
        let assign26390_e35657: f64 = (0.05 * p.p714);
        let assign26390_e35660: f64 = (1.0 + p.p714);
        let assign26390_e35661: f64 = (assign26390_e35657 * assign26390_e35660);
        let assign26390_e35663: f64 = (assign26390_e35661 * locals.var_czbd_p1);
        let assign26390_e35664: f64 = (1.0 - assign26390_e35663);
        let assign26390_e35665: f64 = (assign26390_e35653 * assign26390_e35664);
        (assign26390_e35665,)
    } else {
        (locals.var_czbd_p2,)
    }
};
        locals.var_czbd_p2 = assign26390_e35667;
        locals.var_czbd_p2_rv = 0.0;

        let assign26400_e35670: f64 = (-p.p716);
        let assign26400_e35671: f64 = (0.1_f64).powf(assign26400_e35670);
        locals.var_czbdsw_p1 = assign26400_e35671;
        locals.var_czbdsw_p1_rv = 0.0;

        let assign26410_e35674: f64 = if p.p716 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard656 = assign26410_e35674;
        locals.var_guard656_rv = 0.0;

        let (assign26420_e35681,) = {
    if (locals.var_guard656 != 0.0) {
        let assign26420_e35678: f64 = (0.1_f64).ln();
        let assign26420_e35679: f64 = (1.5 - assign26420_e35678);
        (assign26420_e35679,)
    } else {
        (locals.var_czbdsw_p2,)
    }
};
        locals.var_czbdsw_p2 = assign26420_e35681;
        locals.var_czbdsw_p2_rv = 0.0;

        let (assign26430_e35702,) = {
    if (locals.var_guard656 == 0.0) {
        let assign26430_e35687: f64 = (1.0 - p.p716);
        let assign26430_e35688: f64 = (1.0 / assign26430_e35687);
        let assign26430_e35692: f64 = (0.05 * p.p716);
        let assign26430_e35695: f64 = (1.0 + p.p716);
        let assign26430_e35696: f64 = (assign26430_e35692 * assign26430_e35695);
        let assign26430_e35698: f64 = (assign26430_e35696 * locals.var_czbdsw_p1);
        let assign26430_e35699: f64 = (1.0 - assign26430_e35698);
        let assign26430_e35700: f64 = (assign26430_e35688 * assign26430_e35699);
        (assign26430_e35700,)
    } else {
        (locals.var_czbdsw_p2,)
    }
};
        locals.var_czbdsw_p2 = assign26430_e35702;
        locals.var_czbdsw_p2_rv = 0.0;

        let assign26440_e35705: f64 = (-p.p718);
        let assign26440_e35706: f64 = (0.1_f64).powf(assign26440_e35705);
        locals.var_czbdswg_p1 = assign26440_e35706;
        locals.var_czbdswg_p1_rv = 0.0;

        let assign26450_e35709: f64 = if p.p718 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard657 = assign26450_e35709;
        locals.var_guard657_rv = 0.0;

        let (assign26460_e35716,) = {
    if (locals.var_guard657 != 0.0) {
        let assign26460_e35713: f64 = (0.1_f64).ln();
        let assign26460_e35714: f64 = (1.5 - assign26460_e35713);
        (assign26460_e35714,)
    } else {
        (locals.var_czbdswg_p2,)
    }
};
        locals.var_czbdswg_p2 = assign26460_e35716;
        locals.var_czbdswg_p2_rv = 0.0;

        let (assign26470_e35737,) = {
    if (locals.var_guard657 == 0.0) {
        let assign26470_e35722: f64 = (1.0 - p.p718);
        let assign26470_e35723: f64 = (1.0 / assign26470_e35722);
        let assign26470_e35727: f64 = (0.05 * p.p718);
        let assign26470_e35730: f64 = (1.0 + p.p718);
        let assign26470_e35731: f64 = (assign26470_e35727 * assign26470_e35730);
        let assign26470_e35733: f64 = (assign26470_e35731 * locals.var_czbdswg_p1);
        let assign26470_e35734: f64 = (1.0 - assign26470_e35733);
        let assign26470_e35735: f64 = (assign26470_e35723 * assign26470_e35734);
        (assign26470_e35735,)
    } else {
        (locals.var_czbdswg_p2,)
    }
};
        locals.var_czbdswg_p2 = assign26470_e35737;
        locals.var_czbdswg_p2_rv = 0.0;

        let assign26480_e35740: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard658 = assign26480_e35740;
        locals.var_guard658_rv = 0.0;

        let (assign26490_e35746, assign26490_e35746_d_n0, assign26490_e35746_d_n2, assign26490_e35746_d_n3, assign26490_e35746_d_n4, assign26490_e35746_d_n5, assign26490_e35746_d_n6, assign26490_e35746_d_n7, assign26490_e35746_d_n8, assign26490_e35746_d_n9, assign26490_e35746_d_n10, assign26490_e35746_d_n11, assign26490_e35746_d_n12, assign26490_e35746_d_n13, assign26490_e35746_d_n14,) = {
    if (locals.var_guard658 != 0.0) {
        let assign26490_e35744: f64 = (locals.var_vbd_jctcv / locals.var_pbd_t);
        (assign26490_e35744, 0.0, 0.0, 0.0, (-((locals.var_vbd_jctcv * locals.var_pbd_t_dn4) / (locals.var_pbd_t * locals.var_pbd_t))), (locals.var_vbd_jctcv_dn5 / locals.var_pbd_t), (locals.var_vbd_jctcv_dn6 / locals.var_pbd_t), (locals.var_vbd_jctcv_dn7 / locals.var_pbd_t), 0.0, 0.0, 0.0, (locals.var_vbd_jctcv_dn11 / locals.var_pbd_t), 0.0, (locals.var_vbd_jctcv_dn13 / locals.var_pbd_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign26490_e35746;
        locals.var_t1_dn0 = assign26490_e35746_d_n0;
        locals.var_t1_dn2 = assign26490_e35746_d_n2;
        locals.var_t1_dn3 = assign26490_e35746_d_n3;
        locals.var_t1_dn4 = assign26490_e35746_d_n4;
        locals.var_t1_dn5 = assign26490_e35746_d_n5;
        locals.var_t1_dn6 = assign26490_e35746_d_n6;
        locals.var_t1_dn7 = assign26490_e35746_d_n7;
        locals.var_t1_dn8 = assign26490_e35746_d_n8;
        locals.var_t1_dn9 = assign26490_e35746_d_n9;
        locals.var_t1_dn10 = assign26490_e35746_d_n10;
        locals.var_t1_dn11 = assign26490_e35746_d_n11;
        locals.var_t1_dn12 = assign26490_e35746_d_n12;
        locals.var_t1_dn13 = assign26490_e35746_d_n13;
        locals.var_t1_dn14 = assign26490_e35746_d_n14;
        locals.var_t1_rv = 0.0;

        let assign26500_e35749: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard659 = assign26500_e35749;
        locals.var_guard659_rv = 0.0;

        let (assign26510_e35757, assign26510_e35757_d_n0, assign26510_e35757_d_n2, assign26510_e35757_d_n3, assign26510_e35757_d_n4, assign26510_e35757_d_n5, assign26510_e35757_d_n6, assign26510_e35757_d_n7, assign26510_e35757_d_n8, assign26510_e35757_d_n9, assign26510_e35757_d_n10, assign26510_e35757_d_n11, assign26510_e35757_d_n12, assign26510_e35757_d_n13, assign26510_e35757_d_n14,) = {
    if ((locals.var_guard658 != 0.0) && (locals.var_guard659 != 0.0)) {
        let assign26510_e35755: f64 = (1.0 - locals.var_t1);
        (assign26510_e35755, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn12), (-locals.var_t1_dn13), (-locals.var_t1_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn13, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign26510_e35757;
        locals.var_arg_dn0 = assign26510_e35757_d_n0;
        locals.var_arg_dn2 = assign26510_e35757_d_n2;
        locals.var_arg_dn3 = assign26510_e35757_d_n3;
        locals.var_arg_dn4 = assign26510_e35757_d_n4;
        locals.var_arg_dn5 = assign26510_e35757_d_n5;
        locals.var_arg_dn6 = assign26510_e35757_d_n6;
        locals.var_arg_dn7 = assign26510_e35757_d_n7;
        locals.var_arg_dn8 = assign26510_e35757_d_n8;
        locals.var_arg_dn9 = assign26510_e35757_d_n9;
        locals.var_arg_dn10 = assign26510_e35757_d_n10;
        locals.var_arg_dn11 = assign26510_e35757_d_n11;
        locals.var_arg_dn12 = assign26510_e35757_d_n12;
        locals.var_arg_dn13 = assign26510_e35757_d_n13;
        locals.var_arg_dn14 = assign26510_e35757_d_n14;
        locals.var_arg_rv = 0.0;

        let assign26520_e35760: f64 = if p.p714 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard660 = assign26520_e35760;
        locals.var_guard660_rv = 0.0;

        let assign26530_e35763: f64 = if p.p714 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard661 = assign26530_e35763;
        locals.var_guard661_rv = 0.0;

        let (assign26540_e35776, assign26540_e35776_d_n0, assign26540_e35776_d_n2, assign26540_e35776_d_n3, assign26540_e35776_d_n4, assign26540_e35776_d_n5, assign26540_e35776_d_n6, assign26540_e35776_d_n7, assign26540_e35776_d_n8, assign26540_e35776_d_n9, assign26540_e35776_d_n10, assign26540_e35776_d_n11, assign26540_e35776_d_n12, assign26540_e35776_d_n13, assign26540_e35776_d_n14,) = {
    if ((((locals.var_guard658 != 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 != 0.0)) && (locals.var_guard661 != 0.0)) {
        let assign26540_e35773: f64 = (locals.var_arg).sqrt();
        let assign26540_e35774: f64 = (1.0 / assign26540_e35773);
        (assign26540_e35774, (-((locals.var_arg_dn0 / (2.0 * assign26540_e35773)) / (assign26540_e35773 * assign26540_e35773))), (-((locals.var_arg_dn2 / (2.0 * assign26540_e35773)) / (assign26540_e35773 * assign26540_e35773))), (-((locals.var_arg_dn3 / (2.0 * assign26540_e35773)) / (assign26540_e35773 * assign26540_e35773))), (-((locals.var_arg_dn4 / (2.0 * assign26540_e35773)) / (assign26540_e35773 * assign26540_e35773))), (-((locals.var_arg_dn5 / (2.0 * assign26540_e35773)) / (assign26540_e35773 * assign26540_e35773))), (-((locals.var_arg_dn6 / (2.0 * assign26540_e35773)) / (assign26540_e35773 * assign26540_e35773))), (-((locals.var_arg_dn7 / (2.0 * assign26540_e35773)) / (assign26540_e35773 * assign26540_e35773))), (-((locals.var_arg_dn8 / (2.0 * assign26540_e35773)) / (assign26540_e35773 * assign26540_e35773))), (-((locals.var_arg_dn9 / (2.0 * assign26540_e35773)) / (assign26540_e35773 * assign26540_e35773))), (-((locals.var_arg_dn10 / (2.0 * assign26540_e35773)) / (assign26540_e35773 * assign26540_e35773))), (-((locals.var_arg_dn11 / (2.0 * assign26540_e35773)) / (assign26540_e35773 * assign26540_e35773))), (-((locals.var_arg_dn12 / (2.0 * assign26540_e35773)) / (assign26540_e35773 * assign26540_e35773))), (-((locals.var_arg_dn13 / (2.0 * assign26540_e35773)) / (assign26540_e35773 * assign26540_e35773))), (-((locals.var_arg_dn14 / (2.0 * assign26540_e35773)) / (assign26540_e35773 * assign26540_e35773))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn12, locals.var_sarg_dn13, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign26540_e35776;
        locals.var_sarg_dn0 = assign26540_e35776_d_n0;
        locals.var_sarg_dn2 = assign26540_e35776_d_n2;
        locals.var_sarg_dn3 = assign26540_e35776_d_n3;
        locals.var_sarg_dn4 = assign26540_e35776_d_n4;
        locals.var_sarg_dn5 = assign26540_e35776_d_n5;
        locals.var_sarg_dn6 = assign26540_e35776_d_n6;
        locals.var_sarg_dn7 = assign26540_e35776_d_n7;
        locals.var_sarg_dn8 = assign26540_e35776_d_n8;
        locals.var_sarg_dn9 = assign26540_e35776_d_n9;
        locals.var_sarg_dn10 = assign26540_e35776_d_n10;
        locals.var_sarg_dn11 = assign26540_e35776_d_n11;
        locals.var_sarg_dn12 = assign26540_e35776_d_n12;
        locals.var_sarg_dn13 = assign26540_e35776_d_n13;
        locals.var_sarg_dn14 = assign26540_e35776_d_n14;
        locals.var_sarg_rv = 0.0;

        let (assign26550_e35792, assign26550_e35792_d_n0, assign26550_e35792_d_n2, assign26550_e35792_d_n3, assign26550_e35792_d_n4, assign26550_e35792_d_n5, assign26550_e35792_d_n6, assign26550_e35792_d_n7, assign26550_e35792_d_n8, assign26550_e35792_d_n9, assign26550_e35792_d_n10, assign26550_e35792_d_n11, assign26550_e35792_d_n12, assign26550_e35792_d_n13, assign26550_e35792_d_n14,) = {
    if ((((locals.var_guard658 != 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 != 0.0)) && (locals.var_guard661 == 0.0)) {
        let assign26550_e35786: f64 = (-p.p714);
        let assign26550_e35788: f64 = (locals.var_arg).ln();
        let assign26550_e35789: f64 = (assign26550_e35786 * assign26550_e35788);
        let assign26550_e35790: f64 = { let limited_exp_arg = assign26550_e35789; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign26550_e35790, ({ let limited_exp_arg = assign26550_e35789; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26550_e35786 * (locals.var_arg_dn0 / locals.var_arg))), ({ let limited_exp_arg = assign26550_e35789; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26550_e35786 * (locals.var_arg_dn2 / locals.var_arg))), ({ let limited_exp_arg = assign26550_e35789; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26550_e35786 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign26550_e35789; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26550_e35786 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign26550_e35789; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26550_e35786 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign26550_e35789; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26550_e35786 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign26550_e35789; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26550_e35786 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign26550_e35789; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26550_e35786 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign26550_e35789; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26550_e35786 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign26550_e35789; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26550_e35786 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign26550_e35789; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26550_e35786 * (locals.var_arg_dn11 / locals.var_arg))), ({ let limited_exp_arg = assign26550_e35789; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26550_e35786 * (locals.var_arg_dn12 / locals.var_arg))), ({ let limited_exp_arg = assign26550_e35789; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26550_e35786 * (locals.var_arg_dn13 / locals.var_arg))), ({ let limited_exp_arg = assign26550_e35789; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26550_e35786 * (locals.var_arg_dn14 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn12, locals.var_sarg_dn13, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign26550_e35792;
        locals.var_sarg_dn0 = assign26550_e35792_d_n0;
        locals.var_sarg_dn2 = assign26550_e35792_d_n2;
        locals.var_sarg_dn3 = assign26550_e35792_d_n3;
        locals.var_sarg_dn4 = assign26550_e35792_d_n4;
        locals.var_sarg_dn5 = assign26550_e35792_d_n5;
        locals.var_sarg_dn6 = assign26550_e35792_d_n6;
        locals.var_sarg_dn7 = assign26550_e35792_d_n7;
        locals.var_sarg_dn8 = assign26550_e35792_d_n8;
        locals.var_sarg_dn9 = assign26550_e35792_d_n9;
        locals.var_sarg_dn10 = assign26550_e35792_d_n10;
        locals.var_sarg_dn11 = assign26550_e35792_d_n11;
        locals.var_sarg_dn12 = assign26550_e35792_d_n12;
        locals.var_sarg_dn13 = assign26550_e35792_d_n13;
        locals.var_sarg_dn14 = assign26550_e35792_d_n14;
        locals.var_sarg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_79(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26560_e35812, assign26560_e35812_d_n0, assign26560_e35812_d_n2, assign26560_e35812_d_n3, assign26560_e35812_d_n4, assign26560_e35812_d_n5, assign26560_e35812_d_n6, assign26560_e35812_d_n7, assign26560_e35812_d_n8, assign26560_e35812_d_n9, assign26560_e35812_d_n10, assign26560_e35812_d_n11, assign26560_e35812_d_n12, assign26560_e35812_d_n13, assign26560_e35812_d_n14,) = {
    if (((locals.var_guard658 != 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 != 0.0)) {
        let assign26560_e35800: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign26560_e35804: f64 = (locals.var_arg * locals.var_sarg);
        let assign26560_e35805: f64 = (1.0 - assign26560_e35804);
        let assign26560_e35806: f64 = (assign26560_e35800 * assign26560_e35805);
        let assign26560_e35809: f64 = (1.0 - p.p714);
        let assign26560_e35810: f64 = (assign26560_e35806 / assign26560_e35809);
        (assign26560_e35810, ((((locals.var_pbd_t * locals.var_czbd_dn0) * assign26560_e35805) + (assign26560_e35800 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign26560_e35809), ((((locals.var_pbd_t * locals.var_czbd_dn2) * assign26560_e35805) + (assign26560_e35800 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign26560_e35809), ((((locals.var_pbd_t * locals.var_czbd_dn3) * assign26560_e35805) + (assign26560_e35800 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3))))) / assign26560_e35809), (((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign26560_e35805) + (assign26560_e35800 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign26560_e35809), ((((locals.var_pbd_t * locals.var_czbd_dn5) * assign26560_e35805) + (assign26560_e35800 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign26560_e35809), ((((locals.var_pbd_t * locals.var_czbd_dn6) * assign26560_e35805) + (assign26560_e35800 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign26560_e35809), ((((locals.var_pbd_t * locals.var_czbd_dn7) * assign26560_e35805) + (assign26560_e35800 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign26560_e35809), ((((locals.var_pbd_t * locals.var_czbd_dn8) * assign26560_e35805) + (assign26560_e35800 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign26560_e35809), ((((locals.var_pbd_t * locals.var_czbd_dn9) * assign26560_e35805) + (assign26560_e35800 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign26560_e35809), ((((locals.var_pbd_t * locals.var_czbd_dn10) * assign26560_e35805) + (assign26560_e35800 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign26560_e35809), ((((locals.var_pbd_t * locals.var_czbd_dn11) * assign26560_e35805) + (assign26560_e35800 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign26560_e35809), ((((locals.var_pbd_t * locals.var_czbd_dn12) * assign26560_e35805) + (assign26560_e35800 * (-((locals.var_arg_dn12 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn12))))) / assign26560_e35809), ((((locals.var_pbd_t * locals.var_czbd_dn13) * assign26560_e35805) + (assign26560_e35800 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign26560_e35809), ((((locals.var_pbd_t * locals.var_czbd_dn14) * assign26560_e35805) + (assign26560_e35800 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign26560_e35809),)
    } else {
        (locals.var_qbdj1, locals.var_qbdj1_dn0, locals.var_qbdj1_dn2, locals.var_qbdj1_dn3, locals.var_qbdj1_dn4, locals.var_qbdj1_dn5, locals.var_qbdj1_dn6, locals.var_qbdj1_dn7, locals.var_qbdj1_dn8, locals.var_qbdj1_dn9, locals.var_qbdj1_dn10, locals.var_qbdj1_dn11, locals.var_qbdj1_dn12, locals.var_qbdj1_dn13, locals.var_qbdj1_dn14,)
    }
};
        locals.var_qbdj1 = assign26560_e35812;
        locals.var_qbdj1_dn0 = assign26560_e35812_d_n0;
        locals.var_qbdj1_dn2 = assign26560_e35812_d_n2;
        locals.var_qbdj1_dn3 = assign26560_e35812_d_n3;
        locals.var_qbdj1_dn4 = assign26560_e35812_d_n4;
        locals.var_qbdj1_dn5 = assign26560_e35812_d_n5;
        locals.var_qbdj1_dn6 = assign26560_e35812_d_n6;
        locals.var_qbdj1_dn7 = assign26560_e35812_d_n7;
        locals.var_qbdj1_dn8 = assign26560_e35812_d_n8;
        locals.var_qbdj1_dn9 = assign26560_e35812_d_n9;
        locals.var_qbdj1_dn10 = assign26560_e35812_d_n10;
        locals.var_qbdj1_dn11 = assign26560_e35812_d_n11;
        locals.var_qbdj1_dn12 = assign26560_e35812_d_n12;
        locals.var_qbdj1_dn13 = assign26560_e35812_d_n13;
        locals.var_qbdj1_dn14 = assign26560_e35812_d_n14;
        locals.var_qbdj1_rv = 0.0;

        let (assign26570_e35827, assign26570_e35827_d_n0, assign26570_e35827_d_n2, assign26570_e35827_d_n3, assign26570_e35827_d_n4, assign26570_e35827_d_n5, assign26570_e35827_d_n6, assign26570_e35827_d_n7, assign26570_e35827_d_n8, assign26570_e35827_d_n9, assign26570_e35827_d_n10, assign26570_e35827_d_n11, assign26570_e35827_d_n12, assign26570_e35827_d_n13, assign26570_e35827_d_n14,) = {
    if (((locals.var_guard658 != 0.0) && (locals.var_guard659 != 0.0)) && (locals.var_guard660 == 0.0)) {
        let assign26570_e35821: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign26570_e35823: f64 = (locals.var_arg).ln();
        let assign26570_e35824: f64 = (-assign26570_e35823);
        let assign26570_e35825: f64 = (assign26570_e35821 * assign26570_e35824);
        (assign26570_e35825, (((locals.var_pbd_t * locals.var_czbd_dn0) * assign26570_e35824) + (assign26570_e35821 * (-(locals.var_arg_dn0 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn2) * assign26570_e35824) + (assign26570_e35821 * (-(locals.var_arg_dn2 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn3) * assign26570_e35824) + (assign26570_e35821 * (-(locals.var_arg_dn3 / locals.var_arg)))), ((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign26570_e35824) + (assign26570_e35821 * (-(locals.var_arg_dn4 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn5) * assign26570_e35824) + (assign26570_e35821 * (-(locals.var_arg_dn5 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn6) * assign26570_e35824) + (assign26570_e35821 * (-(locals.var_arg_dn6 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn7) * assign26570_e35824) + (assign26570_e35821 * (-(locals.var_arg_dn7 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn8) * assign26570_e35824) + (assign26570_e35821 * (-(locals.var_arg_dn8 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn9) * assign26570_e35824) + (assign26570_e35821 * (-(locals.var_arg_dn9 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn10) * assign26570_e35824) + (assign26570_e35821 * (-(locals.var_arg_dn10 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn11) * assign26570_e35824) + (assign26570_e35821 * (-(locals.var_arg_dn11 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn12) * assign26570_e35824) + (assign26570_e35821 * (-(locals.var_arg_dn12 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn13) * assign26570_e35824) + (assign26570_e35821 * (-(locals.var_arg_dn13 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn14) * assign26570_e35824) + (assign26570_e35821 * (-(locals.var_arg_dn14 / locals.var_arg)))),)
    } else {
        (locals.var_qbdj1, locals.var_qbdj1_dn0, locals.var_qbdj1_dn2, locals.var_qbdj1_dn3, locals.var_qbdj1_dn4, locals.var_qbdj1_dn5, locals.var_qbdj1_dn6, locals.var_qbdj1_dn7, locals.var_qbdj1_dn8, locals.var_qbdj1_dn9, locals.var_qbdj1_dn10, locals.var_qbdj1_dn11, locals.var_qbdj1_dn12, locals.var_qbdj1_dn13, locals.var_qbdj1_dn14,)
    }
};
        locals.var_qbdj1 = assign26570_e35827;
        locals.var_qbdj1_dn0 = assign26570_e35827_d_n0;
        locals.var_qbdj1_dn2 = assign26570_e35827_d_n2;
        locals.var_qbdj1_dn3 = assign26570_e35827_d_n3;
        locals.var_qbdj1_dn4 = assign26570_e35827_d_n4;
        locals.var_qbdj1_dn5 = assign26570_e35827_d_n5;
        locals.var_qbdj1_dn6 = assign26570_e35827_d_n6;
        locals.var_qbdj1_dn7 = assign26570_e35827_d_n7;
        locals.var_qbdj1_dn8 = assign26570_e35827_d_n8;
        locals.var_qbdj1_dn9 = assign26570_e35827_d_n9;
        locals.var_qbdj1_dn10 = assign26570_e35827_d_n10;
        locals.var_qbdj1_dn11 = assign26570_e35827_d_n11;
        locals.var_qbdj1_dn12 = assign26570_e35827_d_n12;
        locals.var_qbdj1_dn13 = assign26570_e35827_d_n13;
        locals.var_qbdj1_dn14 = assign26570_e35827_d_n14;
        locals.var_qbdj1_rv = 0.0;

        let (assign26580_e35850, assign26580_e35850_d_n0, assign26580_e35850_d_n2, assign26580_e35850_d_n3, assign26580_e35850_d_n4, assign26580_e35850_d_n5, assign26580_e35850_d_n6, assign26580_e35850_d_n7, assign26580_e35850_d_n8, assign26580_e35850_d_n9, assign26580_e35850_d_n10, assign26580_e35850_d_n11, assign26580_e35850_d_n12, assign26580_e35850_d_n13, assign26580_e35850_d_n14,) = {
    if ((locals.var_guard658 != 0.0) && (locals.var_guard659 == 0.0)) {
        let assign26580_e35835: f64 = (locals.var_t1 - 1.0);
        let assign26580_e35836: f64 = (locals.var_czbd_p1 * assign26580_e35835);
        let assign26580_e35839: f64 = (5.0 * p.p714);
        let assign26580_e35842: f64 = (locals.var_t1 - 1.0);
        let assign26580_e35843: f64 = (assign26580_e35839 * assign26580_e35842);
        let assign26580_e35846: f64 = (1.0 + p.p714);
        let assign26580_e35847: f64 = (assign26580_e35843 + assign26580_e35846);
        let assign26580_e35848: f64 = (assign26580_e35836 * assign26580_e35847);
        (assign26580_e35848, (((locals.var_czbd_p1 * locals.var_t1_dn0) * assign26580_e35847) + (assign26580_e35836 * (assign26580_e35839 * locals.var_t1_dn0))), (((locals.var_czbd_p1 * locals.var_t1_dn2) * assign26580_e35847) + (assign26580_e35836 * (assign26580_e35839 * locals.var_t1_dn2))), (((locals.var_czbd_p1 * locals.var_t1_dn3) * assign26580_e35847) + (assign26580_e35836 * (assign26580_e35839 * locals.var_t1_dn3))), (((locals.var_czbd_p1 * locals.var_t1_dn4) * assign26580_e35847) + (assign26580_e35836 * (assign26580_e35839 * locals.var_t1_dn4))), (((locals.var_czbd_p1 * locals.var_t1_dn5) * assign26580_e35847) + (assign26580_e35836 * (assign26580_e35839 * locals.var_t1_dn5))), (((locals.var_czbd_p1 * locals.var_t1_dn6) * assign26580_e35847) + (assign26580_e35836 * (assign26580_e35839 * locals.var_t1_dn6))), (((locals.var_czbd_p1 * locals.var_t1_dn7) * assign26580_e35847) + (assign26580_e35836 * (assign26580_e35839 * locals.var_t1_dn7))), (((locals.var_czbd_p1 * locals.var_t1_dn8) * assign26580_e35847) + (assign26580_e35836 * (assign26580_e35839 * locals.var_t1_dn8))), (((locals.var_czbd_p1 * locals.var_t1_dn9) * assign26580_e35847) + (assign26580_e35836 * (assign26580_e35839 * locals.var_t1_dn9))), (((locals.var_czbd_p1 * locals.var_t1_dn10) * assign26580_e35847) + (assign26580_e35836 * (assign26580_e35839 * locals.var_t1_dn10))), (((locals.var_czbd_p1 * locals.var_t1_dn11) * assign26580_e35847) + (assign26580_e35836 * (assign26580_e35839 * locals.var_t1_dn11))), (((locals.var_czbd_p1 * locals.var_t1_dn12) * assign26580_e35847) + (assign26580_e35836 * (assign26580_e35839 * locals.var_t1_dn12))), (((locals.var_czbd_p1 * locals.var_t1_dn13) * assign26580_e35847) + (assign26580_e35836 * (assign26580_e35839 * locals.var_t1_dn13))), (((locals.var_czbd_p1 * locals.var_t1_dn14) * assign26580_e35847) + (assign26580_e35836 * (assign26580_e35839 * locals.var_t1_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign26580_e35850;
        locals.var_t2_dn0 = assign26580_e35850_d_n0;
        locals.var_t2_dn2 = assign26580_e35850_d_n2;
        locals.var_t2_dn3 = assign26580_e35850_d_n3;
        locals.var_t2_dn4 = assign26580_e35850_d_n4;
        locals.var_t2_dn5 = assign26580_e35850_d_n5;
        locals.var_t2_dn6 = assign26580_e35850_d_n6;
        locals.var_t2_dn7 = assign26580_e35850_d_n7;
        locals.var_t2_dn8 = assign26580_e35850_d_n8;
        locals.var_t2_dn9 = assign26580_e35850_d_n9;
        locals.var_t2_dn10 = assign26580_e35850_d_n10;
        locals.var_t2_dn11 = assign26580_e35850_d_n11;
        locals.var_t2_dn12 = assign26580_e35850_d_n12;
        locals.var_t2_dn13 = assign26580_e35850_d_n13;
        locals.var_t2_dn14 = assign26580_e35850_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign26590_e35863, assign26590_e35863_d_n0, assign26590_e35863_d_n2, assign26590_e35863_d_n3, assign26590_e35863_d_n4, assign26590_e35863_d_n5, assign26590_e35863_d_n6, assign26590_e35863_d_n7, assign26590_e35863_d_n8, assign26590_e35863_d_n9, assign26590_e35863_d_n10, assign26590_e35863_d_n11, assign26590_e35863_d_n12, assign26590_e35863_d_n13, assign26590_e35863_d_n14,) = {
    if ((locals.var_guard658 != 0.0) && (locals.var_guard659 == 0.0)) {
        let assign26590_e35857: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign26590_e35860: f64 = (locals.var_t2 + locals.var_czbd_p2);
        let assign26590_e35861: f64 = (assign26590_e35857 * assign26590_e35860);
        (assign26590_e35861, (((locals.var_pbd_t * locals.var_czbd_dn0) * assign26590_e35860) + (assign26590_e35857 * locals.var_t2_dn0)), (((locals.var_pbd_t * locals.var_czbd_dn2) * assign26590_e35860) + (assign26590_e35857 * locals.var_t2_dn2)), (((locals.var_pbd_t * locals.var_czbd_dn3) * assign26590_e35860) + (assign26590_e35857 * locals.var_t2_dn3)), ((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign26590_e35860) + (assign26590_e35857 * locals.var_t2_dn4)), (((locals.var_pbd_t * locals.var_czbd_dn5) * assign26590_e35860) + (assign26590_e35857 * locals.var_t2_dn5)), (((locals.var_pbd_t * locals.var_czbd_dn6) * assign26590_e35860) + (assign26590_e35857 * locals.var_t2_dn6)), (((locals.var_pbd_t * locals.var_czbd_dn7) * assign26590_e35860) + (assign26590_e35857 * locals.var_t2_dn7)), (((locals.var_pbd_t * locals.var_czbd_dn8) * assign26590_e35860) + (assign26590_e35857 * locals.var_t2_dn8)), (((locals.var_pbd_t * locals.var_czbd_dn9) * assign26590_e35860) + (assign26590_e35857 * locals.var_t2_dn9)), (((locals.var_pbd_t * locals.var_czbd_dn10) * assign26590_e35860) + (assign26590_e35857 * locals.var_t2_dn10)), (((locals.var_pbd_t * locals.var_czbd_dn11) * assign26590_e35860) + (assign26590_e35857 * locals.var_t2_dn11)), (((locals.var_pbd_t * locals.var_czbd_dn12) * assign26590_e35860) + (assign26590_e35857 * locals.var_t2_dn12)), (((locals.var_pbd_t * locals.var_czbd_dn13) * assign26590_e35860) + (assign26590_e35857 * locals.var_t2_dn13)), (((locals.var_pbd_t * locals.var_czbd_dn14) * assign26590_e35860) + (assign26590_e35857 * locals.var_t2_dn14)),)
    } else {
        (locals.var_qbdj1, locals.var_qbdj1_dn0, locals.var_qbdj1_dn2, locals.var_qbdj1_dn3, locals.var_qbdj1_dn4, locals.var_qbdj1_dn5, locals.var_qbdj1_dn6, locals.var_qbdj1_dn7, locals.var_qbdj1_dn8, locals.var_qbdj1_dn9, locals.var_qbdj1_dn10, locals.var_qbdj1_dn11, locals.var_qbdj1_dn12, locals.var_qbdj1_dn13, locals.var_qbdj1_dn14,)
    }
};
        locals.var_qbdj1 = assign26590_e35863;
        locals.var_qbdj1_dn0 = assign26590_e35863_d_n0;
        locals.var_qbdj1_dn2 = assign26590_e35863_d_n2;
        locals.var_qbdj1_dn3 = assign26590_e35863_d_n3;
        locals.var_qbdj1_dn4 = assign26590_e35863_d_n4;
        locals.var_qbdj1_dn5 = assign26590_e35863_d_n5;
        locals.var_qbdj1_dn6 = assign26590_e35863_d_n6;
        locals.var_qbdj1_dn7 = assign26590_e35863_d_n7;
        locals.var_qbdj1_dn8 = assign26590_e35863_d_n8;
        locals.var_qbdj1_dn9 = assign26590_e35863_d_n9;
        locals.var_qbdj1_dn10 = assign26590_e35863_d_n10;
        locals.var_qbdj1_dn11 = assign26590_e35863_d_n11;
        locals.var_qbdj1_dn12 = assign26590_e35863_d_n12;
        locals.var_qbdj1_dn13 = assign26590_e35863_d_n13;
        locals.var_qbdj1_dn14 = assign26590_e35863_d_n14;
        locals.var_qbdj1_rv = 0.0;

        let (assign26600_e35868, assign26600_e35868_d_n0, assign26600_e35868_d_n2, assign26600_e35868_d_n3, assign26600_e35868_d_n4, assign26600_e35868_d_n5, assign26600_e35868_d_n6, assign26600_e35868_d_n7, assign26600_e35868_d_n8, assign26600_e35868_d_n9, assign26600_e35868_d_n10, assign26600_e35868_d_n11, assign26600_e35868_d_n12, assign26600_e35868_d_n13, assign26600_e35868_d_n14,) = {
    if (locals.var_guard658 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdj1, locals.var_qbdj1_dn0, locals.var_qbdj1_dn2, locals.var_qbdj1_dn3, locals.var_qbdj1_dn4, locals.var_qbdj1_dn5, locals.var_qbdj1_dn6, locals.var_qbdj1_dn7, locals.var_qbdj1_dn8, locals.var_qbdj1_dn9, locals.var_qbdj1_dn10, locals.var_qbdj1_dn11, locals.var_qbdj1_dn12, locals.var_qbdj1_dn13, locals.var_qbdj1_dn14,)
    }
};
        locals.var_qbdj1 = assign26600_e35868;
        locals.var_qbdj1_dn0 = assign26600_e35868_d_n0;
        locals.var_qbdj1_dn2 = assign26600_e35868_d_n2;
        locals.var_qbdj1_dn3 = assign26600_e35868_d_n3;
        locals.var_qbdj1_dn4 = assign26600_e35868_d_n4;
        locals.var_qbdj1_dn5 = assign26600_e35868_d_n5;
        locals.var_qbdj1_dn6 = assign26600_e35868_d_n6;
        locals.var_qbdj1_dn7 = assign26600_e35868_d_n7;
        locals.var_qbdj1_dn8 = assign26600_e35868_d_n8;
        locals.var_qbdj1_dn9 = assign26600_e35868_d_n9;
        locals.var_qbdj1_dn10 = assign26600_e35868_d_n10;
        locals.var_qbdj1_dn11 = assign26600_e35868_d_n11;
        locals.var_qbdj1_dn12 = assign26600_e35868_d_n12;
        locals.var_qbdj1_dn13 = assign26600_e35868_d_n13;
        locals.var_qbdj1_dn14 = assign26600_e35868_d_n14;
        locals.var_qbdj1_rv = 0.0;

        let assign26610_e35871: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard662 = assign26610_e35871;
        locals.var_guard662_rv = 0.0;

        let (assign26620_e35877, assign26620_e35877_d_n0, assign26620_e35877_d_n2, assign26620_e35877_d_n3, assign26620_e35877_d_n4, assign26620_e35877_d_n5, assign26620_e35877_d_n6, assign26620_e35877_d_n7, assign26620_e35877_d_n8, assign26620_e35877_d_n9, assign26620_e35877_d_n10, assign26620_e35877_d_n11, assign26620_e35877_d_n12, assign26620_e35877_d_n13, assign26620_e35877_d_n14,) = {
    if (locals.var_guard662 != 0.0) {
        let assign26620_e35875: f64 = (locals.var_vbd_jctcv / locals.var_pbswd_t);
        (assign26620_e35875, 0.0, 0.0, 0.0, (-((locals.var_vbd_jctcv * locals.var_pbswd_t_dn4) / (locals.var_pbswd_t * locals.var_pbswd_t))), (locals.var_vbd_jctcv_dn5 / locals.var_pbswd_t), (locals.var_vbd_jctcv_dn6 / locals.var_pbswd_t), (locals.var_vbd_jctcv_dn7 / locals.var_pbswd_t), 0.0, 0.0, 0.0, (locals.var_vbd_jctcv_dn11 / locals.var_pbswd_t), 0.0, (locals.var_vbd_jctcv_dn13 / locals.var_pbswd_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign26620_e35877;
        locals.var_t1_dn0 = assign26620_e35877_d_n0;
        locals.var_t1_dn2 = assign26620_e35877_d_n2;
        locals.var_t1_dn3 = assign26620_e35877_d_n3;
        locals.var_t1_dn4 = assign26620_e35877_d_n4;
        locals.var_t1_dn5 = assign26620_e35877_d_n5;
        locals.var_t1_dn6 = assign26620_e35877_d_n6;
        locals.var_t1_dn7 = assign26620_e35877_d_n7;
        locals.var_t1_dn8 = assign26620_e35877_d_n8;
        locals.var_t1_dn9 = assign26620_e35877_d_n9;
        locals.var_t1_dn10 = assign26620_e35877_d_n10;
        locals.var_t1_dn11 = assign26620_e35877_d_n11;
        locals.var_t1_dn12 = assign26620_e35877_d_n12;
        locals.var_t1_dn13 = assign26620_e35877_d_n13;
        locals.var_t1_dn14 = assign26620_e35877_d_n14;
        locals.var_t1_rv = 0.0;

        let assign26630_e35880: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard663 = assign26630_e35880;
        locals.var_guard663_rv = 0.0;

        let (assign26640_e35888, assign26640_e35888_d_n0, assign26640_e35888_d_n2, assign26640_e35888_d_n3, assign26640_e35888_d_n4, assign26640_e35888_d_n5, assign26640_e35888_d_n6, assign26640_e35888_d_n7, assign26640_e35888_d_n8, assign26640_e35888_d_n9, assign26640_e35888_d_n10, assign26640_e35888_d_n11, assign26640_e35888_d_n12, assign26640_e35888_d_n13, assign26640_e35888_d_n14,) = {
    if ((locals.var_guard662 != 0.0) && (locals.var_guard663 != 0.0)) {
        let assign26640_e35886: f64 = (1.0 - locals.var_t1);
        (assign26640_e35886, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn12), (-locals.var_t1_dn13), (-locals.var_t1_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn13, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign26640_e35888;
        locals.var_arg_dn0 = assign26640_e35888_d_n0;
        locals.var_arg_dn2 = assign26640_e35888_d_n2;
        locals.var_arg_dn3 = assign26640_e35888_d_n3;
        locals.var_arg_dn4 = assign26640_e35888_d_n4;
        locals.var_arg_dn5 = assign26640_e35888_d_n5;
        locals.var_arg_dn6 = assign26640_e35888_d_n6;
        locals.var_arg_dn7 = assign26640_e35888_d_n7;
        locals.var_arg_dn8 = assign26640_e35888_d_n8;
        locals.var_arg_dn9 = assign26640_e35888_d_n9;
        locals.var_arg_dn10 = assign26640_e35888_d_n10;
        locals.var_arg_dn11 = assign26640_e35888_d_n11;
        locals.var_arg_dn12 = assign26640_e35888_d_n12;
        locals.var_arg_dn13 = assign26640_e35888_d_n13;
        locals.var_arg_dn14 = assign26640_e35888_d_n14;
        locals.var_arg_rv = 0.0;

        let assign26650_e35891: f64 = if p.p716 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard664 = assign26650_e35891;
        locals.var_guard664_rv = 0.0;

        let assign26660_e35894: f64 = if p.p716 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard665 = assign26660_e35894;
        locals.var_guard665_rv = 0.0;

        let (assign26670_e35907, assign26670_e35907_d_n0, assign26670_e35907_d_n2, assign26670_e35907_d_n3, assign26670_e35907_d_n4, assign26670_e35907_d_n5, assign26670_e35907_d_n6, assign26670_e35907_d_n7, assign26670_e35907_d_n8, assign26670_e35907_d_n9, assign26670_e35907_d_n10, assign26670_e35907_d_n11, assign26670_e35907_d_n12, assign26670_e35907_d_n13, assign26670_e35907_d_n14,) = {
    if ((((locals.var_guard662 != 0.0) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) && (locals.var_guard665 != 0.0)) {
        let assign26670_e35904: f64 = (locals.var_arg).sqrt();
        let assign26670_e35905: f64 = (1.0 / assign26670_e35904);
        (assign26670_e35905, (-((locals.var_arg_dn0 / (2.0 * assign26670_e35904)) / (assign26670_e35904 * assign26670_e35904))), (-((locals.var_arg_dn2 / (2.0 * assign26670_e35904)) / (assign26670_e35904 * assign26670_e35904))), (-((locals.var_arg_dn3 / (2.0 * assign26670_e35904)) / (assign26670_e35904 * assign26670_e35904))), (-((locals.var_arg_dn4 / (2.0 * assign26670_e35904)) / (assign26670_e35904 * assign26670_e35904))), (-((locals.var_arg_dn5 / (2.0 * assign26670_e35904)) / (assign26670_e35904 * assign26670_e35904))), (-((locals.var_arg_dn6 / (2.0 * assign26670_e35904)) / (assign26670_e35904 * assign26670_e35904))), (-((locals.var_arg_dn7 / (2.0 * assign26670_e35904)) / (assign26670_e35904 * assign26670_e35904))), (-((locals.var_arg_dn8 / (2.0 * assign26670_e35904)) / (assign26670_e35904 * assign26670_e35904))), (-((locals.var_arg_dn9 / (2.0 * assign26670_e35904)) / (assign26670_e35904 * assign26670_e35904))), (-((locals.var_arg_dn10 / (2.0 * assign26670_e35904)) / (assign26670_e35904 * assign26670_e35904))), (-((locals.var_arg_dn11 / (2.0 * assign26670_e35904)) / (assign26670_e35904 * assign26670_e35904))), (-((locals.var_arg_dn12 / (2.0 * assign26670_e35904)) / (assign26670_e35904 * assign26670_e35904))), (-((locals.var_arg_dn13 / (2.0 * assign26670_e35904)) / (assign26670_e35904 * assign26670_e35904))), (-((locals.var_arg_dn14 / (2.0 * assign26670_e35904)) / (assign26670_e35904 * assign26670_e35904))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn12, locals.var_sarg_dn13, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign26670_e35907;
        locals.var_sarg_dn0 = assign26670_e35907_d_n0;
        locals.var_sarg_dn2 = assign26670_e35907_d_n2;
        locals.var_sarg_dn3 = assign26670_e35907_d_n3;
        locals.var_sarg_dn4 = assign26670_e35907_d_n4;
        locals.var_sarg_dn5 = assign26670_e35907_d_n5;
        locals.var_sarg_dn6 = assign26670_e35907_d_n6;
        locals.var_sarg_dn7 = assign26670_e35907_d_n7;
        locals.var_sarg_dn8 = assign26670_e35907_d_n8;
        locals.var_sarg_dn9 = assign26670_e35907_d_n9;
        locals.var_sarg_dn10 = assign26670_e35907_d_n10;
        locals.var_sarg_dn11 = assign26670_e35907_d_n11;
        locals.var_sarg_dn12 = assign26670_e35907_d_n12;
        locals.var_sarg_dn13 = assign26670_e35907_d_n13;
        locals.var_sarg_dn14 = assign26670_e35907_d_n14;
        locals.var_sarg_rv = 0.0;

        let (assign26680_e35923, assign26680_e35923_d_n0, assign26680_e35923_d_n2, assign26680_e35923_d_n3, assign26680_e35923_d_n4, assign26680_e35923_d_n5, assign26680_e35923_d_n6, assign26680_e35923_d_n7, assign26680_e35923_d_n8, assign26680_e35923_d_n9, assign26680_e35923_d_n10, assign26680_e35923_d_n11, assign26680_e35923_d_n12, assign26680_e35923_d_n13, assign26680_e35923_d_n14,) = {
    if ((((locals.var_guard662 != 0.0) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) && (locals.var_guard665 == 0.0)) {
        let assign26680_e35917: f64 = (-p.p716);
        let assign26680_e35919: f64 = (locals.var_arg).ln();
        let assign26680_e35920: f64 = (assign26680_e35917 * assign26680_e35919);
        let assign26680_e35921: f64 = { let limited_exp_arg = assign26680_e35920; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign26680_e35921, ({ let limited_exp_arg = assign26680_e35920; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26680_e35917 * (locals.var_arg_dn0 / locals.var_arg))), ({ let limited_exp_arg = assign26680_e35920; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26680_e35917 * (locals.var_arg_dn2 / locals.var_arg))), ({ let limited_exp_arg = assign26680_e35920; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26680_e35917 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign26680_e35920; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26680_e35917 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign26680_e35920; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26680_e35917 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign26680_e35920; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26680_e35917 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign26680_e35920; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26680_e35917 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign26680_e35920; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26680_e35917 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign26680_e35920; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26680_e35917 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign26680_e35920; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26680_e35917 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign26680_e35920; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26680_e35917 * (locals.var_arg_dn11 / locals.var_arg))), ({ let limited_exp_arg = assign26680_e35920; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26680_e35917 * (locals.var_arg_dn12 / locals.var_arg))), ({ let limited_exp_arg = assign26680_e35920; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26680_e35917 * (locals.var_arg_dn13 / locals.var_arg))), ({ let limited_exp_arg = assign26680_e35920; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26680_e35917 * (locals.var_arg_dn14 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn12, locals.var_sarg_dn13, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign26680_e35923;
        locals.var_sarg_dn0 = assign26680_e35923_d_n0;
        locals.var_sarg_dn2 = assign26680_e35923_d_n2;
        locals.var_sarg_dn3 = assign26680_e35923_d_n3;
        locals.var_sarg_dn4 = assign26680_e35923_d_n4;
        locals.var_sarg_dn5 = assign26680_e35923_d_n5;
        locals.var_sarg_dn6 = assign26680_e35923_d_n6;
        locals.var_sarg_dn7 = assign26680_e35923_d_n7;
        locals.var_sarg_dn8 = assign26680_e35923_d_n8;
        locals.var_sarg_dn9 = assign26680_e35923_d_n9;
        locals.var_sarg_dn10 = assign26680_e35923_d_n10;
        locals.var_sarg_dn11 = assign26680_e35923_d_n11;
        locals.var_sarg_dn12 = assign26680_e35923_d_n12;
        locals.var_sarg_dn13 = assign26680_e35923_d_n13;
        locals.var_sarg_dn14 = assign26680_e35923_d_n14;
        locals.var_sarg_rv = 0.0;

        let (assign26690_e35943, assign26690_e35943_d_n0, assign26690_e35943_d_n2, assign26690_e35943_d_n3, assign26690_e35943_d_n4, assign26690_e35943_d_n5, assign26690_e35943_d_n6, assign26690_e35943_d_n7, assign26690_e35943_d_n8, assign26690_e35943_d_n9, assign26690_e35943_d_n10, assign26690_e35943_d_n11, assign26690_e35943_d_n12, assign26690_e35943_d_n13, assign26690_e35943_d_n14,) = {
    if (((locals.var_guard662 != 0.0) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 != 0.0)) {
        let assign26690_e35931: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign26690_e35935: f64 = (locals.var_arg * locals.var_sarg);
        let assign26690_e35936: f64 = (1.0 - assign26690_e35935);
        let assign26690_e35937: f64 = (assign26690_e35931 * assign26690_e35936);
        let assign26690_e35940: f64 = (1.0 - p.p716);
        let assign26690_e35941: f64 = (assign26690_e35937 / assign26690_e35940);
        (assign26690_e35941, ((((locals.var_pbswd_t * locals.var_czbdsw_dn0) * assign26690_e35936) + (assign26690_e35931 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign26690_e35940), ((((locals.var_pbswd_t * locals.var_czbdsw_dn2) * assign26690_e35936) + (assign26690_e35931 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign26690_e35940), ((((locals.var_pbswd_t * locals.var_czbdsw_dn3) * assign26690_e35936) + (assign26690_e35931 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3))))) / assign26690_e35940), (((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign26690_e35936) + (assign26690_e35931 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign26690_e35940), ((((locals.var_pbswd_t * locals.var_czbdsw_dn5) * assign26690_e35936) + (assign26690_e35931 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign26690_e35940), ((((locals.var_pbswd_t * locals.var_czbdsw_dn6) * assign26690_e35936) + (assign26690_e35931 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign26690_e35940), ((((locals.var_pbswd_t * locals.var_czbdsw_dn7) * assign26690_e35936) + (assign26690_e35931 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign26690_e35940), ((((locals.var_pbswd_t * locals.var_czbdsw_dn8) * assign26690_e35936) + (assign26690_e35931 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign26690_e35940), ((((locals.var_pbswd_t * locals.var_czbdsw_dn9) * assign26690_e35936) + (assign26690_e35931 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign26690_e35940), ((((locals.var_pbswd_t * locals.var_czbdsw_dn10) * assign26690_e35936) + (assign26690_e35931 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign26690_e35940), ((((locals.var_pbswd_t * locals.var_czbdsw_dn11) * assign26690_e35936) + (assign26690_e35931 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign26690_e35940), ((((locals.var_pbswd_t * locals.var_czbdsw_dn12) * assign26690_e35936) + (assign26690_e35931 * (-((locals.var_arg_dn12 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn12))))) / assign26690_e35940), ((((locals.var_pbswd_t * locals.var_czbdsw_dn13) * assign26690_e35936) + (assign26690_e35931 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign26690_e35940), ((((locals.var_pbswd_t * locals.var_czbdsw_dn14) * assign26690_e35936) + (assign26690_e35931 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign26690_e35940),)
    } else {
        (locals.var_qbdj2, locals.var_qbdj2_dn0, locals.var_qbdj2_dn2, locals.var_qbdj2_dn3, locals.var_qbdj2_dn4, locals.var_qbdj2_dn5, locals.var_qbdj2_dn6, locals.var_qbdj2_dn7, locals.var_qbdj2_dn8, locals.var_qbdj2_dn9, locals.var_qbdj2_dn10, locals.var_qbdj2_dn11, locals.var_qbdj2_dn12, locals.var_qbdj2_dn13, locals.var_qbdj2_dn14,)
    }
};
        locals.var_qbdj2 = assign26690_e35943;
        locals.var_qbdj2_dn0 = assign26690_e35943_d_n0;
        locals.var_qbdj2_dn2 = assign26690_e35943_d_n2;
        locals.var_qbdj2_dn3 = assign26690_e35943_d_n3;
        locals.var_qbdj2_dn4 = assign26690_e35943_d_n4;
        locals.var_qbdj2_dn5 = assign26690_e35943_d_n5;
        locals.var_qbdj2_dn6 = assign26690_e35943_d_n6;
        locals.var_qbdj2_dn7 = assign26690_e35943_d_n7;
        locals.var_qbdj2_dn8 = assign26690_e35943_d_n8;
        locals.var_qbdj2_dn9 = assign26690_e35943_d_n9;
        locals.var_qbdj2_dn10 = assign26690_e35943_d_n10;
        locals.var_qbdj2_dn11 = assign26690_e35943_d_n11;
        locals.var_qbdj2_dn12 = assign26690_e35943_d_n12;
        locals.var_qbdj2_dn13 = assign26690_e35943_d_n13;
        locals.var_qbdj2_dn14 = assign26690_e35943_d_n14;
        locals.var_qbdj2_rv = 0.0;

        let (assign26700_e35958, assign26700_e35958_d_n0, assign26700_e35958_d_n2, assign26700_e35958_d_n3, assign26700_e35958_d_n4, assign26700_e35958_d_n5, assign26700_e35958_d_n6, assign26700_e35958_d_n7, assign26700_e35958_d_n8, assign26700_e35958_d_n9, assign26700_e35958_d_n10, assign26700_e35958_d_n11, assign26700_e35958_d_n12, assign26700_e35958_d_n13, assign26700_e35958_d_n14,) = {
    if (((locals.var_guard662 != 0.0) && (locals.var_guard663 != 0.0)) && (locals.var_guard664 == 0.0)) {
        let assign26700_e35952: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign26700_e35954: f64 = (locals.var_arg).ln();
        let assign26700_e35955: f64 = (-assign26700_e35954);
        let assign26700_e35956: f64 = (assign26700_e35952 * assign26700_e35955);
        (assign26700_e35956, (((locals.var_pbswd_t * locals.var_czbdsw_dn0) * assign26700_e35955) + (assign26700_e35952 * (-(locals.var_arg_dn0 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn2) * assign26700_e35955) + (assign26700_e35952 * (-(locals.var_arg_dn2 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn3) * assign26700_e35955) + (assign26700_e35952 * (-(locals.var_arg_dn3 / locals.var_arg)))), ((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign26700_e35955) + (assign26700_e35952 * (-(locals.var_arg_dn4 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn5) * assign26700_e35955) + (assign26700_e35952 * (-(locals.var_arg_dn5 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn6) * assign26700_e35955) + (assign26700_e35952 * (-(locals.var_arg_dn6 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn7) * assign26700_e35955) + (assign26700_e35952 * (-(locals.var_arg_dn7 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn8) * assign26700_e35955) + (assign26700_e35952 * (-(locals.var_arg_dn8 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn9) * assign26700_e35955) + (assign26700_e35952 * (-(locals.var_arg_dn9 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn10) * assign26700_e35955) + (assign26700_e35952 * (-(locals.var_arg_dn10 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn11) * assign26700_e35955) + (assign26700_e35952 * (-(locals.var_arg_dn11 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn12) * assign26700_e35955) + (assign26700_e35952 * (-(locals.var_arg_dn12 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn13) * assign26700_e35955) + (assign26700_e35952 * (-(locals.var_arg_dn13 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn14) * assign26700_e35955) + (assign26700_e35952 * (-(locals.var_arg_dn14 / locals.var_arg)))),)
    } else {
        (locals.var_qbdj2, locals.var_qbdj2_dn0, locals.var_qbdj2_dn2, locals.var_qbdj2_dn3, locals.var_qbdj2_dn4, locals.var_qbdj2_dn5, locals.var_qbdj2_dn6, locals.var_qbdj2_dn7, locals.var_qbdj2_dn8, locals.var_qbdj2_dn9, locals.var_qbdj2_dn10, locals.var_qbdj2_dn11, locals.var_qbdj2_dn12, locals.var_qbdj2_dn13, locals.var_qbdj2_dn14,)
    }
};
        locals.var_qbdj2 = assign26700_e35958;
        locals.var_qbdj2_dn0 = assign26700_e35958_d_n0;
        locals.var_qbdj2_dn2 = assign26700_e35958_d_n2;
        locals.var_qbdj2_dn3 = assign26700_e35958_d_n3;
        locals.var_qbdj2_dn4 = assign26700_e35958_d_n4;
        locals.var_qbdj2_dn5 = assign26700_e35958_d_n5;
        locals.var_qbdj2_dn6 = assign26700_e35958_d_n6;
        locals.var_qbdj2_dn7 = assign26700_e35958_d_n7;
        locals.var_qbdj2_dn8 = assign26700_e35958_d_n8;
        locals.var_qbdj2_dn9 = assign26700_e35958_d_n9;
        locals.var_qbdj2_dn10 = assign26700_e35958_d_n10;
        locals.var_qbdj2_dn11 = assign26700_e35958_d_n11;
        locals.var_qbdj2_dn12 = assign26700_e35958_d_n12;
        locals.var_qbdj2_dn13 = assign26700_e35958_d_n13;
        locals.var_qbdj2_dn14 = assign26700_e35958_d_n14;
        locals.var_qbdj2_rv = 0.0;

        let (assign26710_e35981, assign26710_e35981_d_n0, assign26710_e35981_d_n2, assign26710_e35981_d_n3, assign26710_e35981_d_n4, assign26710_e35981_d_n5, assign26710_e35981_d_n6, assign26710_e35981_d_n7, assign26710_e35981_d_n8, assign26710_e35981_d_n9, assign26710_e35981_d_n10, assign26710_e35981_d_n11, assign26710_e35981_d_n12, assign26710_e35981_d_n13, assign26710_e35981_d_n14,) = {
    if ((locals.var_guard662 != 0.0) && (locals.var_guard663 == 0.0)) {
        let assign26710_e35966: f64 = (locals.var_t1 - 1.0);
        let assign26710_e35967: f64 = (locals.var_czbdsw_p1 * assign26710_e35966);
        let assign26710_e35970: f64 = (5.0 * p.p716);
        let assign26710_e35973: f64 = (locals.var_t1 - 1.0);
        let assign26710_e35974: f64 = (assign26710_e35970 * assign26710_e35973);
        let assign26710_e35977: f64 = (1.0 + p.p716);
        let assign26710_e35978: f64 = (assign26710_e35974 + assign26710_e35977);
        let assign26710_e35979: f64 = (assign26710_e35967 * assign26710_e35978);
        (assign26710_e35979, (((locals.var_czbdsw_p1 * locals.var_t1_dn0) * assign26710_e35978) + (assign26710_e35967 * (assign26710_e35970 * locals.var_t1_dn0))), (((locals.var_czbdsw_p1 * locals.var_t1_dn2) * assign26710_e35978) + (assign26710_e35967 * (assign26710_e35970 * locals.var_t1_dn2))), (((locals.var_czbdsw_p1 * locals.var_t1_dn3) * assign26710_e35978) + (assign26710_e35967 * (assign26710_e35970 * locals.var_t1_dn3))), (((locals.var_czbdsw_p1 * locals.var_t1_dn4) * assign26710_e35978) + (assign26710_e35967 * (assign26710_e35970 * locals.var_t1_dn4))), (((locals.var_czbdsw_p1 * locals.var_t1_dn5) * assign26710_e35978) + (assign26710_e35967 * (assign26710_e35970 * locals.var_t1_dn5))), (((locals.var_czbdsw_p1 * locals.var_t1_dn6) * assign26710_e35978) + (assign26710_e35967 * (assign26710_e35970 * locals.var_t1_dn6))), (((locals.var_czbdsw_p1 * locals.var_t1_dn7) * assign26710_e35978) + (assign26710_e35967 * (assign26710_e35970 * locals.var_t1_dn7))), (((locals.var_czbdsw_p1 * locals.var_t1_dn8) * assign26710_e35978) + (assign26710_e35967 * (assign26710_e35970 * locals.var_t1_dn8))), (((locals.var_czbdsw_p1 * locals.var_t1_dn9) * assign26710_e35978) + (assign26710_e35967 * (assign26710_e35970 * locals.var_t1_dn9))), (((locals.var_czbdsw_p1 * locals.var_t1_dn10) * assign26710_e35978) + (assign26710_e35967 * (assign26710_e35970 * locals.var_t1_dn10))), (((locals.var_czbdsw_p1 * locals.var_t1_dn11) * assign26710_e35978) + (assign26710_e35967 * (assign26710_e35970 * locals.var_t1_dn11))), (((locals.var_czbdsw_p1 * locals.var_t1_dn12) * assign26710_e35978) + (assign26710_e35967 * (assign26710_e35970 * locals.var_t1_dn12))), (((locals.var_czbdsw_p1 * locals.var_t1_dn13) * assign26710_e35978) + (assign26710_e35967 * (assign26710_e35970 * locals.var_t1_dn13))), (((locals.var_czbdsw_p1 * locals.var_t1_dn14) * assign26710_e35978) + (assign26710_e35967 * (assign26710_e35970 * locals.var_t1_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign26710_e35981;
        locals.var_t2_dn0 = assign26710_e35981_d_n0;
        locals.var_t2_dn2 = assign26710_e35981_d_n2;
        locals.var_t2_dn3 = assign26710_e35981_d_n3;
        locals.var_t2_dn4 = assign26710_e35981_d_n4;
        locals.var_t2_dn5 = assign26710_e35981_d_n5;
        locals.var_t2_dn6 = assign26710_e35981_d_n6;
        locals.var_t2_dn7 = assign26710_e35981_d_n7;
        locals.var_t2_dn8 = assign26710_e35981_d_n8;
        locals.var_t2_dn9 = assign26710_e35981_d_n9;
        locals.var_t2_dn10 = assign26710_e35981_d_n10;
        locals.var_t2_dn11 = assign26710_e35981_d_n11;
        locals.var_t2_dn12 = assign26710_e35981_d_n12;
        locals.var_t2_dn13 = assign26710_e35981_d_n13;
        locals.var_t2_dn14 = assign26710_e35981_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign26720_e35994, assign26720_e35994_d_n0, assign26720_e35994_d_n2, assign26720_e35994_d_n3, assign26720_e35994_d_n4, assign26720_e35994_d_n5, assign26720_e35994_d_n6, assign26720_e35994_d_n7, assign26720_e35994_d_n8, assign26720_e35994_d_n9, assign26720_e35994_d_n10, assign26720_e35994_d_n11, assign26720_e35994_d_n12, assign26720_e35994_d_n13, assign26720_e35994_d_n14,) = {
    if ((locals.var_guard662 != 0.0) && (locals.var_guard663 == 0.0)) {
        let assign26720_e35988: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign26720_e35991: f64 = (locals.var_t2 + locals.var_czbdsw_p2);
        let assign26720_e35992: f64 = (assign26720_e35988 * assign26720_e35991);
        (assign26720_e35992, (((locals.var_pbswd_t * locals.var_czbdsw_dn0) * assign26720_e35991) + (assign26720_e35988 * locals.var_t2_dn0)), (((locals.var_pbswd_t * locals.var_czbdsw_dn2) * assign26720_e35991) + (assign26720_e35988 * locals.var_t2_dn2)), (((locals.var_pbswd_t * locals.var_czbdsw_dn3) * assign26720_e35991) + (assign26720_e35988 * locals.var_t2_dn3)), ((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign26720_e35991) + (assign26720_e35988 * locals.var_t2_dn4)), (((locals.var_pbswd_t * locals.var_czbdsw_dn5) * assign26720_e35991) + (assign26720_e35988 * locals.var_t2_dn5)), (((locals.var_pbswd_t * locals.var_czbdsw_dn6) * assign26720_e35991) + (assign26720_e35988 * locals.var_t2_dn6)), (((locals.var_pbswd_t * locals.var_czbdsw_dn7) * assign26720_e35991) + (assign26720_e35988 * locals.var_t2_dn7)), (((locals.var_pbswd_t * locals.var_czbdsw_dn8) * assign26720_e35991) + (assign26720_e35988 * locals.var_t2_dn8)), (((locals.var_pbswd_t * locals.var_czbdsw_dn9) * assign26720_e35991) + (assign26720_e35988 * locals.var_t2_dn9)), (((locals.var_pbswd_t * locals.var_czbdsw_dn10) * assign26720_e35991) + (assign26720_e35988 * locals.var_t2_dn10)), (((locals.var_pbswd_t * locals.var_czbdsw_dn11) * assign26720_e35991) + (assign26720_e35988 * locals.var_t2_dn11)), (((locals.var_pbswd_t * locals.var_czbdsw_dn12) * assign26720_e35991) + (assign26720_e35988 * locals.var_t2_dn12)), (((locals.var_pbswd_t * locals.var_czbdsw_dn13) * assign26720_e35991) + (assign26720_e35988 * locals.var_t2_dn13)), (((locals.var_pbswd_t * locals.var_czbdsw_dn14) * assign26720_e35991) + (assign26720_e35988 * locals.var_t2_dn14)),)
    } else {
        (locals.var_qbdj2, locals.var_qbdj2_dn0, locals.var_qbdj2_dn2, locals.var_qbdj2_dn3, locals.var_qbdj2_dn4, locals.var_qbdj2_dn5, locals.var_qbdj2_dn6, locals.var_qbdj2_dn7, locals.var_qbdj2_dn8, locals.var_qbdj2_dn9, locals.var_qbdj2_dn10, locals.var_qbdj2_dn11, locals.var_qbdj2_dn12, locals.var_qbdj2_dn13, locals.var_qbdj2_dn14,)
    }
};
        locals.var_qbdj2 = assign26720_e35994;
        locals.var_qbdj2_dn0 = assign26720_e35994_d_n0;
        locals.var_qbdj2_dn2 = assign26720_e35994_d_n2;
        locals.var_qbdj2_dn3 = assign26720_e35994_d_n3;
        locals.var_qbdj2_dn4 = assign26720_e35994_d_n4;
        locals.var_qbdj2_dn5 = assign26720_e35994_d_n5;
        locals.var_qbdj2_dn6 = assign26720_e35994_d_n6;
        locals.var_qbdj2_dn7 = assign26720_e35994_d_n7;
        locals.var_qbdj2_dn8 = assign26720_e35994_d_n8;
        locals.var_qbdj2_dn9 = assign26720_e35994_d_n9;
        locals.var_qbdj2_dn10 = assign26720_e35994_d_n10;
        locals.var_qbdj2_dn11 = assign26720_e35994_d_n11;
        locals.var_qbdj2_dn12 = assign26720_e35994_d_n12;
        locals.var_qbdj2_dn13 = assign26720_e35994_d_n13;
        locals.var_qbdj2_dn14 = assign26720_e35994_d_n14;
        locals.var_qbdj2_rv = 0.0;

        let (assign26730_e35999, assign26730_e35999_d_n0, assign26730_e35999_d_n2, assign26730_e35999_d_n3, assign26730_e35999_d_n4, assign26730_e35999_d_n5, assign26730_e35999_d_n6, assign26730_e35999_d_n7, assign26730_e35999_d_n8, assign26730_e35999_d_n9, assign26730_e35999_d_n10, assign26730_e35999_d_n11, assign26730_e35999_d_n12, assign26730_e35999_d_n13, assign26730_e35999_d_n14,) = {
    if (locals.var_guard662 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdj2, locals.var_qbdj2_dn0, locals.var_qbdj2_dn2, locals.var_qbdj2_dn3, locals.var_qbdj2_dn4, locals.var_qbdj2_dn5, locals.var_qbdj2_dn6, locals.var_qbdj2_dn7, locals.var_qbdj2_dn8, locals.var_qbdj2_dn9, locals.var_qbdj2_dn10, locals.var_qbdj2_dn11, locals.var_qbdj2_dn12, locals.var_qbdj2_dn13, locals.var_qbdj2_dn14,)
    }
};
        locals.var_qbdj2 = assign26730_e35999;
        locals.var_qbdj2_dn0 = assign26730_e35999_d_n0;
        locals.var_qbdj2_dn2 = assign26730_e35999_d_n2;
        locals.var_qbdj2_dn3 = assign26730_e35999_d_n3;
        locals.var_qbdj2_dn4 = assign26730_e35999_d_n4;
        locals.var_qbdj2_dn5 = assign26730_e35999_d_n5;
        locals.var_qbdj2_dn6 = assign26730_e35999_d_n6;
        locals.var_qbdj2_dn7 = assign26730_e35999_d_n7;
        locals.var_qbdj2_dn8 = assign26730_e35999_d_n8;
        locals.var_qbdj2_dn9 = assign26730_e35999_d_n9;
        locals.var_qbdj2_dn10 = assign26730_e35999_d_n10;
        locals.var_qbdj2_dn11 = assign26730_e35999_d_n11;
        locals.var_qbdj2_dn12 = assign26730_e35999_d_n12;
        locals.var_qbdj2_dn13 = assign26730_e35999_d_n13;
        locals.var_qbdj2_dn14 = assign26730_e35999_d_n14;
        locals.var_qbdj2_rv = 0.0;

        let assign26740_e36002: f64 = if locals.var_czbdswg > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard666 = assign26740_e36002;
        locals.var_guard666_rv = 0.0;

        let (assign26750_e36008, assign26750_e36008_d_n0, assign26750_e36008_d_n2, assign26750_e36008_d_n3, assign26750_e36008_d_n4, assign26750_e36008_d_n5, assign26750_e36008_d_n6, assign26750_e36008_d_n7, assign26750_e36008_d_n8, assign26750_e36008_d_n9, assign26750_e36008_d_n10, assign26750_e36008_d_n11, assign26750_e36008_d_n12, assign26750_e36008_d_n13, assign26750_e36008_d_n14,) = {
    if (locals.var_guard666 != 0.0) {
        let assign26750_e36006: f64 = (locals.var_vbd_jctcv / locals.var_pbswgd_t);
        (assign26750_e36006, 0.0, 0.0, 0.0, (-((locals.var_vbd_jctcv * locals.var_pbswgd_t_dn4) / (locals.var_pbswgd_t * locals.var_pbswgd_t))), (locals.var_vbd_jctcv_dn5 / locals.var_pbswgd_t), (locals.var_vbd_jctcv_dn6 / locals.var_pbswgd_t), (locals.var_vbd_jctcv_dn7 / locals.var_pbswgd_t), 0.0, 0.0, 0.0, (locals.var_vbd_jctcv_dn11 / locals.var_pbswgd_t), 0.0, (locals.var_vbd_jctcv_dn13 / locals.var_pbswgd_t), 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign26750_e36008;
        locals.var_t1_dn0 = assign26750_e36008_d_n0;
        locals.var_t1_dn2 = assign26750_e36008_d_n2;
        locals.var_t1_dn3 = assign26750_e36008_d_n3;
        locals.var_t1_dn4 = assign26750_e36008_d_n4;
        locals.var_t1_dn5 = assign26750_e36008_d_n5;
        locals.var_t1_dn6 = assign26750_e36008_d_n6;
        locals.var_t1_dn7 = assign26750_e36008_d_n7;
        locals.var_t1_dn8 = assign26750_e36008_d_n8;
        locals.var_t1_dn9 = assign26750_e36008_d_n9;
        locals.var_t1_dn10 = assign26750_e36008_d_n10;
        locals.var_t1_dn11 = assign26750_e36008_d_n11;
        locals.var_t1_dn12 = assign26750_e36008_d_n12;
        locals.var_t1_dn13 = assign26750_e36008_d_n13;
        locals.var_t1_dn14 = assign26750_e36008_d_n14;
        locals.var_t1_rv = 0.0;

        let assign26760_e36011: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard667 = assign26760_e36011;
        locals.var_guard667_rv = 0.0;

        let (assign26770_e36019, assign26770_e36019_d_n0, assign26770_e36019_d_n2, assign26770_e36019_d_n3, assign26770_e36019_d_n4, assign26770_e36019_d_n5, assign26770_e36019_d_n6, assign26770_e36019_d_n7, assign26770_e36019_d_n8, assign26770_e36019_d_n9, assign26770_e36019_d_n10, assign26770_e36019_d_n11, assign26770_e36019_d_n12, assign26770_e36019_d_n13, assign26770_e36019_d_n14,) = {
    if ((locals.var_guard666 != 0.0) && (locals.var_guard667 != 0.0)) {
        let assign26770_e36017: f64 = (1.0 - locals.var_t1);
        (assign26770_e36017, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn12), (-locals.var_t1_dn13), (-locals.var_t1_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn13, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign26770_e36019;
        locals.var_arg_dn0 = assign26770_e36019_d_n0;
        locals.var_arg_dn2 = assign26770_e36019_d_n2;
        locals.var_arg_dn3 = assign26770_e36019_d_n3;
        locals.var_arg_dn4 = assign26770_e36019_d_n4;
        locals.var_arg_dn5 = assign26770_e36019_d_n5;
        locals.var_arg_dn6 = assign26770_e36019_d_n6;
        locals.var_arg_dn7 = assign26770_e36019_d_n7;
        locals.var_arg_dn8 = assign26770_e36019_d_n8;
        locals.var_arg_dn9 = assign26770_e36019_d_n9;
        locals.var_arg_dn10 = assign26770_e36019_d_n10;
        locals.var_arg_dn11 = assign26770_e36019_d_n11;
        locals.var_arg_dn12 = assign26770_e36019_d_n12;
        locals.var_arg_dn13 = assign26770_e36019_d_n13;
        locals.var_arg_dn14 = assign26770_e36019_d_n14;
        locals.var_arg_rv = 0.0;

        let assign26780_e36022: f64 = if p.p718 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard668 = assign26780_e36022;
        locals.var_guard668_rv = 0.0;

        let assign26790_e36025: f64 = if p.p718 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard669 = assign26790_e36025;
        locals.var_guard669_rv = 0.0;

        let (assign26800_e36038, assign26800_e36038_d_n0, assign26800_e36038_d_n2, assign26800_e36038_d_n3, assign26800_e36038_d_n4, assign26800_e36038_d_n5, assign26800_e36038_d_n6, assign26800_e36038_d_n7, assign26800_e36038_d_n8, assign26800_e36038_d_n9, assign26800_e36038_d_n10, assign26800_e36038_d_n11, assign26800_e36038_d_n12, assign26800_e36038_d_n13, assign26800_e36038_d_n14,) = {
    if ((((locals.var_guard666 != 0.0) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) && (locals.var_guard669 != 0.0)) {
        let assign26800_e36035: f64 = (locals.var_arg).sqrt();
        let assign26800_e36036: f64 = (1.0 / assign26800_e36035);
        (assign26800_e36036, (-((locals.var_arg_dn0 / (2.0 * assign26800_e36035)) / (assign26800_e36035 * assign26800_e36035))), (-((locals.var_arg_dn2 / (2.0 * assign26800_e36035)) / (assign26800_e36035 * assign26800_e36035))), (-((locals.var_arg_dn3 / (2.0 * assign26800_e36035)) / (assign26800_e36035 * assign26800_e36035))), (-((locals.var_arg_dn4 / (2.0 * assign26800_e36035)) / (assign26800_e36035 * assign26800_e36035))), (-((locals.var_arg_dn5 / (2.0 * assign26800_e36035)) / (assign26800_e36035 * assign26800_e36035))), (-((locals.var_arg_dn6 / (2.0 * assign26800_e36035)) / (assign26800_e36035 * assign26800_e36035))), (-((locals.var_arg_dn7 / (2.0 * assign26800_e36035)) / (assign26800_e36035 * assign26800_e36035))), (-((locals.var_arg_dn8 / (2.0 * assign26800_e36035)) / (assign26800_e36035 * assign26800_e36035))), (-((locals.var_arg_dn9 / (2.0 * assign26800_e36035)) / (assign26800_e36035 * assign26800_e36035))), (-((locals.var_arg_dn10 / (2.0 * assign26800_e36035)) / (assign26800_e36035 * assign26800_e36035))), (-((locals.var_arg_dn11 / (2.0 * assign26800_e36035)) / (assign26800_e36035 * assign26800_e36035))), (-((locals.var_arg_dn12 / (2.0 * assign26800_e36035)) / (assign26800_e36035 * assign26800_e36035))), (-((locals.var_arg_dn13 / (2.0 * assign26800_e36035)) / (assign26800_e36035 * assign26800_e36035))), (-((locals.var_arg_dn14 / (2.0 * assign26800_e36035)) / (assign26800_e36035 * assign26800_e36035))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn12, locals.var_sarg_dn13, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign26800_e36038;
        locals.var_sarg_dn0 = assign26800_e36038_d_n0;
        locals.var_sarg_dn2 = assign26800_e36038_d_n2;
        locals.var_sarg_dn3 = assign26800_e36038_d_n3;
        locals.var_sarg_dn4 = assign26800_e36038_d_n4;
        locals.var_sarg_dn5 = assign26800_e36038_d_n5;
        locals.var_sarg_dn6 = assign26800_e36038_d_n6;
        locals.var_sarg_dn7 = assign26800_e36038_d_n7;
        locals.var_sarg_dn8 = assign26800_e36038_d_n8;
        locals.var_sarg_dn9 = assign26800_e36038_d_n9;
        locals.var_sarg_dn10 = assign26800_e36038_d_n10;
        locals.var_sarg_dn11 = assign26800_e36038_d_n11;
        locals.var_sarg_dn12 = assign26800_e36038_d_n12;
        locals.var_sarg_dn13 = assign26800_e36038_d_n13;
        locals.var_sarg_dn14 = assign26800_e36038_d_n14;
        locals.var_sarg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_80(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign26810_e36054, assign26810_e36054_d_n0, assign26810_e36054_d_n2, assign26810_e36054_d_n3, assign26810_e36054_d_n4, assign26810_e36054_d_n5, assign26810_e36054_d_n6, assign26810_e36054_d_n7, assign26810_e36054_d_n8, assign26810_e36054_d_n9, assign26810_e36054_d_n10, assign26810_e36054_d_n11, assign26810_e36054_d_n12, assign26810_e36054_d_n13, assign26810_e36054_d_n14,) = {
    if ((((locals.var_guard666 != 0.0) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) && (locals.var_guard669 == 0.0)) {
        let assign26810_e36048: f64 = (-p.p718);
        let assign26810_e36050: f64 = (locals.var_arg).ln();
        let assign26810_e36051: f64 = (assign26810_e36048 * assign26810_e36050);
        let assign26810_e36052: f64 = { let limited_exp_arg = assign26810_e36051; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign26810_e36052, ({ let limited_exp_arg = assign26810_e36051; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26810_e36048 * (locals.var_arg_dn0 / locals.var_arg))), ({ let limited_exp_arg = assign26810_e36051; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26810_e36048 * (locals.var_arg_dn2 / locals.var_arg))), ({ let limited_exp_arg = assign26810_e36051; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26810_e36048 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign26810_e36051; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26810_e36048 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign26810_e36051; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26810_e36048 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign26810_e36051; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26810_e36048 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign26810_e36051; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26810_e36048 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign26810_e36051; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26810_e36048 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign26810_e36051; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26810_e36048 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign26810_e36051; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26810_e36048 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign26810_e36051; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26810_e36048 * (locals.var_arg_dn11 / locals.var_arg))), ({ let limited_exp_arg = assign26810_e36051; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26810_e36048 * (locals.var_arg_dn12 / locals.var_arg))), ({ let limited_exp_arg = assign26810_e36051; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26810_e36048 * (locals.var_arg_dn13 / locals.var_arg))), ({ let limited_exp_arg = assign26810_e36051; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign26810_e36048 * (locals.var_arg_dn14 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn12, locals.var_sarg_dn13, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign26810_e36054;
        locals.var_sarg_dn0 = assign26810_e36054_d_n0;
        locals.var_sarg_dn2 = assign26810_e36054_d_n2;
        locals.var_sarg_dn3 = assign26810_e36054_d_n3;
        locals.var_sarg_dn4 = assign26810_e36054_d_n4;
        locals.var_sarg_dn5 = assign26810_e36054_d_n5;
        locals.var_sarg_dn6 = assign26810_e36054_d_n6;
        locals.var_sarg_dn7 = assign26810_e36054_d_n7;
        locals.var_sarg_dn8 = assign26810_e36054_d_n8;
        locals.var_sarg_dn9 = assign26810_e36054_d_n9;
        locals.var_sarg_dn10 = assign26810_e36054_d_n10;
        locals.var_sarg_dn11 = assign26810_e36054_d_n11;
        locals.var_sarg_dn12 = assign26810_e36054_d_n12;
        locals.var_sarg_dn13 = assign26810_e36054_d_n13;
        locals.var_sarg_dn14 = assign26810_e36054_d_n14;
        locals.var_sarg_rv = 0.0;

        let (assign26820_e36074, assign26820_e36074_d_n0, assign26820_e36074_d_n2, assign26820_e36074_d_n3, assign26820_e36074_d_n4, assign26820_e36074_d_n5, assign26820_e36074_d_n6, assign26820_e36074_d_n7, assign26820_e36074_d_n8, assign26820_e36074_d_n9, assign26820_e36074_d_n10, assign26820_e36074_d_n11, assign26820_e36074_d_n12, assign26820_e36074_d_n13, assign26820_e36074_d_n14,) = {
    if (((locals.var_guard666 != 0.0) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 != 0.0)) {
        let assign26820_e36062: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign26820_e36066: f64 = (locals.var_arg * locals.var_sarg);
        let assign26820_e36067: f64 = (1.0 - assign26820_e36066);
        let assign26820_e36068: f64 = (assign26820_e36062 * assign26820_e36067);
        let assign26820_e36071: f64 = (1.0 - p.p718);
        let assign26820_e36072: f64 = (assign26820_e36068 / assign26820_e36071);
        (assign26820_e36072, ((assign26820_e36062 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0)))) / assign26820_e36071), ((assign26820_e36062 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2)))) / assign26820_e36071), ((assign26820_e36062 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3)))) / assign26820_e36071), (((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign26820_e36067) + (assign26820_e36062 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign26820_e36071), ((assign26820_e36062 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5)))) / assign26820_e36071), ((assign26820_e36062 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6)))) / assign26820_e36071), ((assign26820_e36062 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7)))) / assign26820_e36071), ((assign26820_e36062 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8)))) / assign26820_e36071), ((assign26820_e36062 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9)))) / assign26820_e36071), ((assign26820_e36062 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10)))) / assign26820_e36071), ((assign26820_e36062 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11)))) / assign26820_e36071), ((assign26820_e36062 * (-((locals.var_arg_dn12 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn12)))) / assign26820_e36071), ((assign26820_e36062 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13)))) / assign26820_e36071), ((assign26820_e36062 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14)))) / assign26820_e36071),)
    } else {
        (locals.var_qbdj3, locals.var_qbdj3_dn0, locals.var_qbdj3_dn2, locals.var_qbdj3_dn3, locals.var_qbdj3_dn4, locals.var_qbdj3_dn5, locals.var_qbdj3_dn6, locals.var_qbdj3_dn7, locals.var_qbdj3_dn8, locals.var_qbdj3_dn9, locals.var_qbdj3_dn10, locals.var_qbdj3_dn11, locals.var_qbdj3_dn12, locals.var_qbdj3_dn13, locals.var_qbdj3_dn14,)
    }
};
        locals.var_qbdj3 = assign26820_e36074;
        locals.var_qbdj3_dn0 = assign26820_e36074_d_n0;
        locals.var_qbdj3_dn2 = assign26820_e36074_d_n2;
        locals.var_qbdj3_dn3 = assign26820_e36074_d_n3;
        locals.var_qbdj3_dn4 = assign26820_e36074_d_n4;
        locals.var_qbdj3_dn5 = assign26820_e36074_d_n5;
        locals.var_qbdj3_dn6 = assign26820_e36074_d_n6;
        locals.var_qbdj3_dn7 = assign26820_e36074_d_n7;
        locals.var_qbdj3_dn8 = assign26820_e36074_d_n8;
        locals.var_qbdj3_dn9 = assign26820_e36074_d_n9;
        locals.var_qbdj3_dn10 = assign26820_e36074_d_n10;
        locals.var_qbdj3_dn11 = assign26820_e36074_d_n11;
        locals.var_qbdj3_dn12 = assign26820_e36074_d_n12;
        locals.var_qbdj3_dn13 = assign26820_e36074_d_n13;
        locals.var_qbdj3_dn14 = assign26820_e36074_d_n14;
        locals.var_qbdj3_rv = 0.0;

        let (assign26830_e36089, assign26830_e36089_d_n0, assign26830_e36089_d_n2, assign26830_e36089_d_n3, assign26830_e36089_d_n4, assign26830_e36089_d_n5, assign26830_e36089_d_n6, assign26830_e36089_d_n7, assign26830_e36089_d_n8, assign26830_e36089_d_n9, assign26830_e36089_d_n10, assign26830_e36089_d_n11, assign26830_e36089_d_n12, assign26830_e36089_d_n13, assign26830_e36089_d_n14,) = {
    if (((locals.var_guard666 != 0.0) && (locals.var_guard667 != 0.0)) && (locals.var_guard668 == 0.0)) {
        let assign26830_e36083: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign26830_e36085: f64 = (locals.var_arg).ln();
        let assign26830_e36086: f64 = (-assign26830_e36085);
        let assign26830_e36087: f64 = (assign26830_e36083 * assign26830_e36086);
        (assign26830_e36087, (assign26830_e36083 * (-(locals.var_arg_dn0 / locals.var_arg))), (assign26830_e36083 * (-(locals.var_arg_dn2 / locals.var_arg))), (assign26830_e36083 * (-(locals.var_arg_dn3 / locals.var_arg))), ((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign26830_e36086) + (assign26830_e36083 * (-(locals.var_arg_dn4 / locals.var_arg)))), (assign26830_e36083 * (-(locals.var_arg_dn5 / locals.var_arg))), (assign26830_e36083 * (-(locals.var_arg_dn6 / locals.var_arg))), (assign26830_e36083 * (-(locals.var_arg_dn7 / locals.var_arg))), (assign26830_e36083 * (-(locals.var_arg_dn8 / locals.var_arg))), (assign26830_e36083 * (-(locals.var_arg_dn9 / locals.var_arg))), (assign26830_e36083 * (-(locals.var_arg_dn10 / locals.var_arg))), (assign26830_e36083 * (-(locals.var_arg_dn11 / locals.var_arg))), (assign26830_e36083 * (-(locals.var_arg_dn12 / locals.var_arg))), (assign26830_e36083 * (-(locals.var_arg_dn13 / locals.var_arg))), (assign26830_e36083 * (-(locals.var_arg_dn14 / locals.var_arg))),)
    } else {
        (locals.var_qbdj3, locals.var_qbdj3_dn0, locals.var_qbdj3_dn2, locals.var_qbdj3_dn3, locals.var_qbdj3_dn4, locals.var_qbdj3_dn5, locals.var_qbdj3_dn6, locals.var_qbdj3_dn7, locals.var_qbdj3_dn8, locals.var_qbdj3_dn9, locals.var_qbdj3_dn10, locals.var_qbdj3_dn11, locals.var_qbdj3_dn12, locals.var_qbdj3_dn13, locals.var_qbdj3_dn14,)
    }
};
        locals.var_qbdj3 = assign26830_e36089;
        locals.var_qbdj3_dn0 = assign26830_e36089_d_n0;
        locals.var_qbdj3_dn2 = assign26830_e36089_d_n2;
        locals.var_qbdj3_dn3 = assign26830_e36089_d_n3;
        locals.var_qbdj3_dn4 = assign26830_e36089_d_n4;
        locals.var_qbdj3_dn5 = assign26830_e36089_d_n5;
        locals.var_qbdj3_dn6 = assign26830_e36089_d_n6;
        locals.var_qbdj3_dn7 = assign26830_e36089_d_n7;
        locals.var_qbdj3_dn8 = assign26830_e36089_d_n8;
        locals.var_qbdj3_dn9 = assign26830_e36089_d_n9;
        locals.var_qbdj3_dn10 = assign26830_e36089_d_n10;
        locals.var_qbdj3_dn11 = assign26830_e36089_d_n11;
        locals.var_qbdj3_dn12 = assign26830_e36089_d_n12;
        locals.var_qbdj3_dn13 = assign26830_e36089_d_n13;
        locals.var_qbdj3_dn14 = assign26830_e36089_d_n14;
        locals.var_qbdj3_rv = 0.0;

        let (assign26840_e36112, assign26840_e36112_d_n0, assign26840_e36112_d_n2, assign26840_e36112_d_n3, assign26840_e36112_d_n4, assign26840_e36112_d_n5, assign26840_e36112_d_n6, assign26840_e36112_d_n7, assign26840_e36112_d_n8, assign26840_e36112_d_n9, assign26840_e36112_d_n10, assign26840_e36112_d_n11, assign26840_e36112_d_n12, assign26840_e36112_d_n13, assign26840_e36112_d_n14,) = {
    if ((locals.var_guard666 != 0.0) && (locals.var_guard667 == 0.0)) {
        let assign26840_e36097: f64 = (locals.var_t1 - 1.0);
        let assign26840_e36098: f64 = (locals.var_czbdswg_p1 * assign26840_e36097);
        let assign26840_e36101: f64 = (5.0 * p.p718);
        let assign26840_e36104: f64 = (locals.var_t1 - 1.0);
        let assign26840_e36105: f64 = (assign26840_e36101 * assign26840_e36104);
        let assign26840_e36108: f64 = (1.0 + p.p718);
        let assign26840_e36109: f64 = (assign26840_e36105 + assign26840_e36108);
        let assign26840_e36110: f64 = (assign26840_e36098 * assign26840_e36109);
        (assign26840_e36110, (((locals.var_czbdswg_p1 * locals.var_t1_dn0) * assign26840_e36109) + (assign26840_e36098 * (assign26840_e36101 * locals.var_t1_dn0))), (((locals.var_czbdswg_p1 * locals.var_t1_dn2) * assign26840_e36109) + (assign26840_e36098 * (assign26840_e36101 * locals.var_t1_dn2))), (((locals.var_czbdswg_p1 * locals.var_t1_dn3) * assign26840_e36109) + (assign26840_e36098 * (assign26840_e36101 * locals.var_t1_dn3))), (((locals.var_czbdswg_p1 * locals.var_t1_dn4) * assign26840_e36109) + (assign26840_e36098 * (assign26840_e36101 * locals.var_t1_dn4))), (((locals.var_czbdswg_p1 * locals.var_t1_dn5) * assign26840_e36109) + (assign26840_e36098 * (assign26840_e36101 * locals.var_t1_dn5))), (((locals.var_czbdswg_p1 * locals.var_t1_dn6) * assign26840_e36109) + (assign26840_e36098 * (assign26840_e36101 * locals.var_t1_dn6))), (((locals.var_czbdswg_p1 * locals.var_t1_dn7) * assign26840_e36109) + (assign26840_e36098 * (assign26840_e36101 * locals.var_t1_dn7))), (((locals.var_czbdswg_p1 * locals.var_t1_dn8) * assign26840_e36109) + (assign26840_e36098 * (assign26840_e36101 * locals.var_t1_dn8))), (((locals.var_czbdswg_p1 * locals.var_t1_dn9) * assign26840_e36109) + (assign26840_e36098 * (assign26840_e36101 * locals.var_t1_dn9))), (((locals.var_czbdswg_p1 * locals.var_t1_dn10) * assign26840_e36109) + (assign26840_e36098 * (assign26840_e36101 * locals.var_t1_dn10))), (((locals.var_czbdswg_p1 * locals.var_t1_dn11) * assign26840_e36109) + (assign26840_e36098 * (assign26840_e36101 * locals.var_t1_dn11))), (((locals.var_czbdswg_p1 * locals.var_t1_dn12) * assign26840_e36109) + (assign26840_e36098 * (assign26840_e36101 * locals.var_t1_dn12))), (((locals.var_czbdswg_p1 * locals.var_t1_dn13) * assign26840_e36109) + (assign26840_e36098 * (assign26840_e36101 * locals.var_t1_dn13))), (((locals.var_czbdswg_p1 * locals.var_t1_dn14) * assign26840_e36109) + (assign26840_e36098 * (assign26840_e36101 * locals.var_t1_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign26840_e36112;
        locals.var_t2_dn0 = assign26840_e36112_d_n0;
        locals.var_t2_dn2 = assign26840_e36112_d_n2;
        locals.var_t2_dn3 = assign26840_e36112_d_n3;
        locals.var_t2_dn4 = assign26840_e36112_d_n4;
        locals.var_t2_dn5 = assign26840_e36112_d_n5;
        locals.var_t2_dn6 = assign26840_e36112_d_n6;
        locals.var_t2_dn7 = assign26840_e36112_d_n7;
        locals.var_t2_dn8 = assign26840_e36112_d_n8;
        locals.var_t2_dn9 = assign26840_e36112_d_n9;
        locals.var_t2_dn10 = assign26840_e36112_d_n10;
        locals.var_t2_dn11 = assign26840_e36112_d_n11;
        locals.var_t2_dn12 = assign26840_e36112_d_n12;
        locals.var_t2_dn13 = assign26840_e36112_d_n13;
        locals.var_t2_dn14 = assign26840_e36112_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign26850_e36125, assign26850_e36125_d_n0, assign26850_e36125_d_n2, assign26850_e36125_d_n3, assign26850_e36125_d_n4, assign26850_e36125_d_n5, assign26850_e36125_d_n6, assign26850_e36125_d_n7, assign26850_e36125_d_n8, assign26850_e36125_d_n9, assign26850_e36125_d_n10, assign26850_e36125_d_n11, assign26850_e36125_d_n12, assign26850_e36125_d_n13, assign26850_e36125_d_n14,) = {
    if ((locals.var_guard666 != 0.0) && (locals.var_guard667 == 0.0)) {
        let assign26850_e36119: f64 = (locals.var_pbswgd_t * locals.var_czbdswg);
        let assign26850_e36122: f64 = (locals.var_t2 + locals.var_czbdswg_p2);
        let assign26850_e36123: f64 = (assign26850_e36119 * assign26850_e36122);
        (assign26850_e36123, (assign26850_e36119 * locals.var_t2_dn0), (assign26850_e36119 * locals.var_t2_dn2), (assign26850_e36119 * locals.var_t2_dn3), ((((locals.var_pbswgd_t_dn4 * locals.var_czbdswg) + (locals.var_pbswgd_t * locals.var_czbdswg_dn4)) * assign26850_e36122) + (assign26850_e36119 * locals.var_t2_dn4)), (assign26850_e36119 * locals.var_t2_dn5), (assign26850_e36119 * locals.var_t2_dn6), (assign26850_e36119 * locals.var_t2_dn7), (assign26850_e36119 * locals.var_t2_dn8), (assign26850_e36119 * locals.var_t2_dn9), (assign26850_e36119 * locals.var_t2_dn10), (assign26850_e36119 * locals.var_t2_dn11), (assign26850_e36119 * locals.var_t2_dn12), (assign26850_e36119 * locals.var_t2_dn13), (assign26850_e36119 * locals.var_t2_dn14),)
    } else {
        (locals.var_qbdj3, locals.var_qbdj3_dn0, locals.var_qbdj3_dn2, locals.var_qbdj3_dn3, locals.var_qbdj3_dn4, locals.var_qbdj3_dn5, locals.var_qbdj3_dn6, locals.var_qbdj3_dn7, locals.var_qbdj3_dn8, locals.var_qbdj3_dn9, locals.var_qbdj3_dn10, locals.var_qbdj3_dn11, locals.var_qbdj3_dn12, locals.var_qbdj3_dn13, locals.var_qbdj3_dn14,)
    }
};
        locals.var_qbdj3 = assign26850_e36125;
        locals.var_qbdj3_dn0 = assign26850_e36125_d_n0;
        locals.var_qbdj3_dn2 = assign26850_e36125_d_n2;
        locals.var_qbdj3_dn3 = assign26850_e36125_d_n3;
        locals.var_qbdj3_dn4 = assign26850_e36125_d_n4;
        locals.var_qbdj3_dn5 = assign26850_e36125_d_n5;
        locals.var_qbdj3_dn6 = assign26850_e36125_d_n6;
        locals.var_qbdj3_dn7 = assign26850_e36125_d_n7;
        locals.var_qbdj3_dn8 = assign26850_e36125_d_n8;
        locals.var_qbdj3_dn9 = assign26850_e36125_d_n9;
        locals.var_qbdj3_dn10 = assign26850_e36125_d_n10;
        locals.var_qbdj3_dn11 = assign26850_e36125_d_n11;
        locals.var_qbdj3_dn12 = assign26850_e36125_d_n12;
        locals.var_qbdj3_dn13 = assign26850_e36125_d_n13;
        locals.var_qbdj3_dn14 = assign26850_e36125_d_n14;
        locals.var_qbdj3_rv = 0.0;

        let (assign26860_e36130, assign26860_e36130_d_n0, assign26860_e36130_d_n2, assign26860_e36130_d_n3, assign26860_e36130_d_n4, assign26860_e36130_d_n5, assign26860_e36130_d_n6, assign26860_e36130_d_n7, assign26860_e36130_d_n8, assign26860_e36130_d_n9, assign26860_e36130_d_n10, assign26860_e36130_d_n11, assign26860_e36130_d_n12, assign26860_e36130_d_n13, assign26860_e36130_d_n14,) = {
    if (locals.var_guard666 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdj3, locals.var_qbdj3_dn0, locals.var_qbdj3_dn2, locals.var_qbdj3_dn3, locals.var_qbdj3_dn4, locals.var_qbdj3_dn5, locals.var_qbdj3_dn6, locals.var_qbdj3_dn7, locals.var_qbdj3_dn8, locals.var_qbdj3_dn9, locals.var_qbdj3_dn10, locals.var_qbdj3_dn11, locals.var_qbdj3_dn12, locals.var_qbdj3_dn13, locals.var_qbdj3_dn14,)
    }
};
        locals.var_qbdj3 = assign26860_e36130;
        locals.var_qbdj3_dn0 = assign26860_e36130_d_n0;
        locals.var_qbdj3_dn2 = assign26860_e36130_d_n2;
        locals.var_qbdj3_dn3 = assign26860_e36130_d_n3;
        locals.var_qbdj3_dn4 = assign26860_e36130_d_n4;
        locals.var_qbdj3_dn5 = assign26860_e36130_d_n5;
        locals.var_qbdj3_dn6 = assign26860_e36130_d_n6;
        locals.var_qbdj3_dn7 = assign26860_e36130_d_n7;
        locals.var_qbdj3_dn8 = assign26860_e36130_d_n8;
        locals.var_qbdj3_dn9 = assign26860_e36130_d_n9;
        locals.var_qbdj3_dn10 = assign26860_e36130_d_n10;
        locals.var_qbdj3_dn11 = assign26860_e36130_d_n11;
        locals.var_qbdj3_dn12 = assign26860_e36130_d_n12;
        locals.var_qbdj3_dn13 = assign26860_e36130_d_n13;
        locals.var_qbdj3_dn14 = assign26860_e36130_d_n14;
        locals.var_qbdj3_rv = 0.0;

        let assign26870_e36133: f64 = (locals.var_qbdj1 + locals.var_qbdj2);
        let assign26870_e36135: f64 = (assign26870_e36133 + locals.var_qbdj3);
        locals.var_qbdj = assign26870_e36135;
        locals.var_qbdj_dn0 = ((locals.var_qbdj1_dn0 + locals.var_qbdj2_dn0) + locals.var_qbdj3_dn0);
        locals.var_qbdj_dn2 = ((locals.var_qbdj1_dn2 + locals.var_qbdj2_dn2) + locals.var_qbdj3_dn2);
        locals.var_qbdj_dn3 = ((locals.var_qbdj1_dn3 + locals.var_qbdj2_dn3) + locals.var_qbdj3_dn3);
        locals.var_qbdj_dn4 = ((locals.var_qbdj1_dn4 + locals.var_qbdj2_dn4) + locals.var_qbdj3_dn4);
        locals.var_qbdj_dn5 = ((locals.var_qbdj1_dn5 + locals.var_qbdj2_dn5) + locals.var_qbdj3_dn5);
        locals.var_qbdj_dn6 = ((locals.var_qbdj1_dn6 + locals.var_qbdj2_dn6) + locals.var_qbdj3_dn6);
        locals.var_qbdj_dn7 = ((locals.var_qbdj1_dn7 + locals.var_qbdj2_dn7) + locals.var_qbdj3_dn7);
        locals.var_qbdj_dn8 = ((locals.var_qbdj1_dn8 + locals.var_qbdj2_dn8) + locals.var_qbdj3_dn8);
        locals.var_qbdj_dn9 = ((locals.var_qbdj1_dn9 + locals.var_qbdj2_dn9) + locals.var_qbdj3_dn9);
        locals.var_qbdj_dn10 = ((locals.var_qbdj1_dn10 + locals.var_qbdj2_dn10) + locals.var_qbdj3_dn10);
        locals.var_qbdj_dn11 = ((locals.var_qbdj1_dn11 + locals.var_qbdj2_dn11) + locals.var_qbdj3_dn11);
        locals.var_qbdj_dn12 = ((locals.var_qbdj1_dn12 + locals.var_qbdj2_dn12) + locals.var_qbdj3_dn12);
        locals.var_qbdj_dn13 = ((locals.var_qbdj1_dn13 + locals.var_qbdj2_dn13) + locals.var_qbdj3_dn13);
        locals.var_qbdj_dn14 = ((locals.var_qbdj1_dn14 + locals.var_qbdj2_dn14) + locals.var_qbdj3_dn14);
        locals.var_qbdj_rv = 0.0;

        let assign26880_e36142: f64 = if ((p.p1128 > 0.0) && (p.p1097 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard670 = assign26880_e36142;
        locals.var_guard670_rv = 0.0;

        let (assign26890_e36150, assign26890_e36150_d_n0, assign26890_e36150_d_n2, assign26890_e36150_d_n3, assign26890_e36150_d_n4, assign26890_e36150_d_n5, assign26890_e36150_d_n6, assign26890_e36150_d_n7, assign26890_e36150_d_n8, assign26890_e36150_d_n9, assign26890_e36150_d_n10, assign26890_e36150_d_n11, assign26890_e36150_d_n12, assign26890_e36150_d_n13, assign26890_e36150_d_n14,) = {
    if (locals.var_guard670 != 0.0) {
        let assign26890_e36146: f64 = (p.p1128 * locals.var_cjd_t);
        let assign26890_e36148: f64 = (assign26890_e36146 * locals.var_adeff);
        (assign26890_e36148, (assign26890_e36146 * locals.var_adeff_dn0), (assign26890_e36146 * locals.var_adeff_dn2), (assign26890_e36146 * locals.var_adeff_dn3), (((p.p1128 * locals.var_cjd_t_dn4) * locals.var_adeff) + (assign26890_e36146 * locals.var_adeff_dn4)), (assign26890_e36146 * locals.var_adeff_dn5), (assign26890_e36146 * locals.var_adeff_dn6), (assign26890_e36146 * locals.var_adeff_dn7), (assign26890_e36146 * locals.var_adeff_dn8), (assign26890_e36146 * locals.var_adeff_dn9), (assign26890_e36146 * locals.var_adeff_dn10), (assign26890_e36146 * locals.var_adeff_dn11), (assign26890_e36146 * locals.var_adeff_dn12), (assign26890_e36146 * locals.var_adeff_dn13), (assign26890_e36146 * locals.var_adeff_dn14),)
    } else {
        (locals.var_czbd, locals.var_czbd_dn0, locals.var_czbd_dn2, locals.var_czbd_dn3, locals.var_czbd_dn4, locals.var_czbd_dn5, locals.var_czbd_dn6, locals.var_czbd_dn7, locals.var_czbd_dn8, locals.var_czbd_dn9, locals.var_czbd_dn10, locals.var_czbd_dn11, locals.var_czbd_dn12, locals.var_czbd_dn13, locals.var_czbd_dn14,)
    }
};
        locals.var_czbd = assign26890_e36150;
        locals.var_czbd_dn0 = assign26890_e36150_d_n0;
        locals.var_czbd_dn2 = assign26890_e36150_d_n2;
        locals.var_czbd_dn3 = assign26890_e36150_d_n3;
        locals.var_czbd_dn4 = assign26890_e36150_d_n4;
        locals.var_czbd_dn5 = assign26890_e36150_d_n5;
        locals.var_czbd_dn6 = assign26890_e36150_d_n6;
        locals.var_czbd_dn7 = assign26890_e36150_d_n7;
        locals.var_czbd_dn8 = assign26890_e36150_d_n8;
        locals.var_czbd_dn9 = assign26890_e36150_d_n9;
        locals.var_czbd_dn10 = assign26890_e36150_d_n10;
        locals.var_czbd_dn11 = assign26890_e36150_d_n11;
        locals.var_czbd_dn12 = assign26890_e36150_d_n12;
        locals.var_czbd_dn13 = assign26890_e36150_d_n13;
        locals.var_czbd_dn14 = assign26890_e36150_d_n14;
        locals.var_czbd_rv = 0.0;

        let assign26900_e36154: f64 = (locals.var_weffcj * p.p2);
        let assign26900_e36155: f64 = if locals.var_pdeff > assign26900_e36154 { 1.0 } else { 0.0 };
        locals.var_guard671 = assign26900_e36155;
        locals.var_guard671_rv = 0.0;

        let (assign26910_e36173, assign26910_e36173_d_n0, assign26910_e36173_d_n2, assign26910_e36173_d_n3, assign26910_e36173_d_n4, assign26910_e36173_d_n5, assign26910_e36173_d_n6, assign26910_e36173_d_n7, assign26910_e36173_d_n8, assign26910_e36173_d_n9, assign26910_e36173_d_n10, assign26910_e36173_d_n11, assign26910_e36173_d_n12, assign26910_e36173_d_n13, assign26910_e36173_d_n14,) = {
    if ((locals.var_guard670 != 0.0) && (locals.var_guard671 != 0.0)) {
        let assign26910_e36164: f64 = (locals.var_weffcj * p.p2);
        let assign26910_e36165: f64 = (locals.var_pdeff - assign26910_e36164);
        let assign26910_e36166: f64 = (p.p1128 * assign26910_e36165);
        let assign26910_e36169: f64 = (locals.var_weffcj * p.p2);
        let assign26910_e36170: f64 = (assign26910_e36166 + assign26910_e36169);
        let assign26910_e36171: f64 = (locals.var_cjswd_t * assign26910_e36170);
        (assign26910_e36171, (locals.var_cjswd_t * (p.p1128 * locals.var_pdeff_dn0)), (locals.var_cjswd_t * (p.p1128 * locals.var_pdeff_dn2)), (locals.var_cjswd_t * (p.p1128 * locals.var_pdeff_dn3)), ((locals.var_cjswd_t_dn4 * assign26910_e36170) + (locals.var_cjswd_t * (p.p1128 * locals.var_pdeff_dn4))), (locals.var_cjswd_t * (p.p1128 * locals.var_pdeff_dn5)), (locals.var_cjswd_t * (p.p1128 * locals.var_pdeff_dn6)), (locals.var_cjswd_t * (p.p1128 * locals.var_pdeff_dn7)), (locals.var_cjswd_t * (p.p1128 * locals.var_pdeff_dn8)), (locals.var_cjswd_t * (p.p1128 * locals.var_pdeff_dn9)), (locals.var_cjswd_t * (p.p1128 * locals.var_pdeff_dn10)), (locals.var_cjswd_t * (p.p1128 * locals.var_pdeff_dn11)), (locals.var_cjswd_t * (p.p1128 * locals.var_pdeff_dn12)), (locals.var_cjswd_t * (p.p1128 * locals.var_pdeff_dn13)), (locals.var_cjswd_t * (p.p1128 * locals.var_pdeff_dn14)),)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn3, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn12, locals.var_czbdsw_dn13, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign26910_e36173;
        locals.var_czbdsw_dn0 = assign26910_e36173_d_n0;
        locals.var_czbdsw_dn2 = assign26910_e36173_d_n2;
        locals.var_czbdsw_dn3 = assign26910_e36173_d_n3;
        locals.var_czbdsw_dn4 = assign26910_e36173_d_n4;
        locals.var_czbdsw_dn5 = assign26910_e36173_d_n5;
        locals.var_czbdsw_dn6 = assign26910_e36173_d_n6;
        locals.var_czbdsw_dn7 = assign26910_e36173_d_n7;
        locals.var_czbdsw_dn8 = assign26910_e36173_d_n8;
        locals.var_czbdsw_dn9 = assign26910_e36173_d_n9;
        locals.var_czbdsw_dn10 = assign26910_e36173_d_n10;
        locals.var_czbdsw_dn11 = assign26910_e36173_d_n11;
        locals.var_czbdsw_dn12 = assign26910_e36173_d_n12;
        locals.var_czbdsw_dn13 = assign26910_e36173_d_n13;
        locals.var_czbdsw_dn14 = assign26910_e36173_d_n14;
        locals.var_czbdsw_rv = 0.0;

        let (assign26920_e36184, assign26920_e36184_d_n0, assign26920_e36184_d_n2, assign26920_e36184_d_n3, assign26920_e36184_d_n4, assign26920_e36184_d_n5, assign26920_e36184_d_n6, assign26920_e36184_d_n7, assign26920_e36184_d_n8, assign26920_e36184_d_n9, assign26920_e36184_d_n10, assign26920_e36184_d_n11, assign26920_e36184_d_n12, assign26920_e36184_d_n13, assign26920_e36184_d_n14,) = {
    if ((locals.var_guard670 != 0.0) && (locals.var_guard671 == 0.0)) {
        let assign26920_e36180: f64 = (p.p1128 * locals.var_cjswd_t);
        let assign26920_e36182: f64 = (assign26920_e36180 * locals.var_pdeff);
        (assign26920_e36182, (assign26920_e36180 * locals.var_pdeff_dn0), (assign26920_e36180 * locals.var_pdeff_dn2), (assign26920_e36180 * locals.var_pdeff_dn3), (((p.p1128 * locals.var_cjswd_t_dn4) * locals.var_pdeff) + (assign26920_e36180 * locals.var_pdeff_dn4)), (assign26920_e36180 * locals.var_pdeff_dn5), (assign26920_e36180 * locals.var_pdeff_dn6), (assign26920_e36180 * locals.var_pdeff_dn7), (assign26920_e36180 * locals.var_pdeff_dn8), (assign26920_e36180 * locals.var_pdeff_dn9), (assign26920_e36180 * locals.var_pdeff_dn10), (assign26920_e36180 * locals.var_pdeff_dn11), (assign26920_e36180 * locals.var_pdeff_dn12), (assign26920_e36180 * locals.var_pdeff_dn13), (assign26920_e36180 * locals.var_pdeff_dn14),)
    } else {
        (locals.var_czbdsw, locals.var_czbdsw_dn0, locals.var_czbdsw_dn2, locals.var_czbdsw_dn3, locals.var_czbdsw_dn4, locals.var_czbdsw_dn5, locals.var_czbdsw_dn6, locals.var_czbdsw_dn7, locals.var_czbdsw_dn8, locals.var_czbdsw_dn9, locals.var_czbdsw_dn10, locals.var_czbdsw_dn11, locals.var_czbdsw_dn12, locals.var_czbdsw_dn13, locals.var_czbdsw_dn14,)
    }
};
        locals.var_czbdsw = assign26920_e36184;
        locals.var_czbdsw_dn0 = assign26920_e36184_d_n0;
        locals.var_czbdsw_dn2 = assign26920_e36184_d_n2;
        locals.var_czbdsw_dn3 = assign26920_e36184_d_n3;
        locals.var_czbdsw_dn4 = assign26920_e36184_d_n4;
        locals.var_czbdsw_dn5 = assign26920_e36184_d_n5;
        locals.var_czbdsw_dn6 = assign26920_e36184_d_n6;
        locals.var_czbdsw_dn7 = assign26920_e36184_d_n7;
        locals.var_czbdsw_dn8 = assign26920_e36184_d_n8;
        locals.var_czbdsw_dn9 = assign26920_e36184_d_n9;
        locals.var_czbdsw_dn10 = assign26920_e36184_d_n10;
        locals.var_czbdsw_dn11 = assign26920_e36184_d_n11;
        locals.var_czbdsw_dn12 = assign26920_e36184_d_n12;
        locals.var_czbdsw_dn13 = assign26920_e36184_d_n13;
        locals.var_czbdsw_dn14 = assign26920_e36184_d_n14;
        locals.var_czbdsw_rv = 0.0;

        let assign26930_e36187: f64 = if locals.var_czbd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard672 = assign26930_e36187;
        locals.var_guard672_rv = 0.0;

        let (assign26940_e36195, assign26940_e36195_d_n0, assign26940_e36195_d_n2, assign26940_e36195_d_n3, assign26940_e36195_d_n4, assign26940_e36195_d_n5, assign26940_e36195_d_n6, assign26940_e36195_d_n7, assign26940_e36195_d_n8, assign26940_e36195_d_n9, assign26940_e36195_d_n10, assign26940_e36195_d_n11, assign26940_e36195_d_n12, assign26940_e36195_d_n13, assign26940_e36195_d_n14,) = {
    if ((locals.var_guard670 != 0.0) && (locals.var_guard672 != 0.0)) {
        let assign26940_e36193: f64 = (locals.var_vbd_ext / locals.var_pbd_t);
        (assign26940_e36193, 0.0, 0.0, 0.0, (-((locals.var_vbd_ext * locals.var_pbd_t_dn4) / (locals.var_pbd_t * locals.var_pbd_t))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_vbd_ext_dn13 / locals.var_pbd_t), (locals.var_vbd_ext_dn14 / locals.var_pbd_t),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign26940_e36195;
        locals.var_t1_dn0 = assign26940_e36195_d_n0;
        locals.var_t1_dn2 = assign26940_e36195_d_n2;
        locals.var_t1_dn3 = assign26940_e36195_d_n3;
        locals.var_t1_dn4 = assign26940_e36195_d_n4;
        locals.var_t1_dn5 = assign26940_e36195_d_n5;
        locals.var_t1_dn6 = assign26940_e36195_d_n6;
        locals.var_t1_dn7 = assign26940_e36195_d_n7;
        locals.var_t1_dn8 = assign26940_e36195_d_n8;
        locals.var_t1_dn9 = assign26940_e36195_d_n9;
        locals.var_t1_dn10 = assign26940_e36195_d_n10;
        locals.var_t1_dn11 = assign26940_e36195_d_n11;
        locals.var_t1_dn12 = assign26940_e36195_d_n12;
        locals.var_t1_dn13 = assign26940_e36195_d_n13;
        locals.var_t1_dn14 = assign26940_e36195_d_n14;
        locals.var_t1_rv = 0.0;

        let assign26950_e36198: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard673 = assign26950_e36198;
        locals.var_guard673_rv = 0.0;

        let (assign26960_e36208, assign26960_e36208_d_n0, assign26960_e36208_d_n2, assign26960_e36208_d_n3, assign26960_e36208_d_n4, assign26960_e36208_d_n5, assign26960_e36208_d_n6, assign26960_e36208_d_n7, assign26960_e36208_d_n8, assign26960_e36208_d_n9, assign26960_e36208_d_n10, assign26960_e36208_d_n11, assign26960_e36208_d_n12, assign26960_e36208_d_n13, assign26960_e36208_d_n14,) = {
    if (((locals.var_guard670 != 0.0) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 != 0.0)) {
        let assign26960_e36206: f64 = (1.0 - locals.var_t1);
        (assign26960_e36206, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn12), (-locals.var_t1_dn13), (-locals.var_t1_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn13, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign26960_e36208;
        locals.var_arg_dn0 = assign26960_e36208_d_n0;
        locals.var_arg_dn2 = assign26960_e36208_d_n2;
        locals.var_arg_dn3 = assign26960_e36208_d_n3;
        locals.var_arg_dn4 = assign26960_e36208_d_n4;
        locals.var_arg_dn5 = assign26960_e36208_d_n5;
        locals.var_arg_dn6 = assign26960_e36208_d_n6;
        locals.var_arg_dn7 = assign26960_e36208_d_n7;
        locals.var_arg_dn8 = assign26960_e36208_d_n8;
        locals.var_arg_dn9 = assign26960_e36208_d_n9;
        locals.var_arg_dn10 = assign26960_e36208_d_n10;
        locals.var_arg_dn11 = assign26960_e36208_d_n11;
        locals.var_arg_dn12 = assign26960_e36208_d_n12;
        locals.var_arg_dn13 = assign26960_e36208_d_n13;
        locals.var_arg_dn14 = assign26960_e36208_d_n14;
        locals.var_arg_rv = 0.0;

        let assign26970_e36211: f64 = if p.p714 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard674 = assign26970_e36211;
        locals.var_guard674_rv = 0.0;

        let assign26980_e36214: f64 = if p.p714 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard675 = assign26980_e36214;
        locals.var_guard675_rv = 0.0;

        let (assign26990_e36229, assign26990_e36229_d_n0, assign26990_e36229_d_n2, assign26990_e36229_d_n3, assign26990_e36229_d_n4, assign26990_e36229_d_n5, assign26990_e36229_d_n6, assign26990_e36229_d_n7, assign26990_e36229_d_n8, assign26990_e36229_d_n9, assign26990_e36229_d_n10, assign26990_e36229_d_n11, assign26990_e36229_d_n12, assign26990_e36229_d_n13, assign26990_e36229_d_n14,) = {
    if (((((locals.var_guard670 != 0.0) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 != 0.0)) && (locals.var_guard674 != 0.0)) && (locals.var_guard675 != 0.0)) {
        let assign26990_e36226: f64 = (locals.var_arg).sqrt();
        let assign26990_e36227: f64 = (1.0 / assign26990_e36226);
        (assign26990_e36227, (-((locals.var_arg_dn0 / (2.0 * assign26990_e36226)) / (assign26990_e36226 * assign26990_e36226))), (-((locals.var_arg_dn2 / (2.0 * assign26990_e36226)) / (assign26990_e36226 * assign26990_e36226))), (-((locals.var_arg_dn3 / (2.0 * assign26990_e36226)) / (assign26990_e36226 * assign26990_e36226))), (-((locals.var_arg_dn4 / (2.0 * assign26990_e36226)) / (assign26990_e36226 * assign26990_e36226))), (-((locals.var_arg_dn5 / (2.0 * assign26990_e36226)) / (assign26990_e36226 * assign26990_e36226))), (-((locals.var_arg_dn6 / (2.0 * assign26990_e36226)) / (assign26990_e36226 * assign26990_e36226))), (-((locals.var_arg_dn7 / (2.0 * assign26990_e36226)) / (assign26990_e36226 * assign26990_e36226))), (-((locals.var_arg_dn8 / (2.0 * assign26990_e36226)) / (assign26990_e36226 * assign26990_e36226))), (-((locals.var_arg_dn9 / (2.0 * assign26990_e36226)) / (assign26990_e36226 * assign26990_e36226))), (-((locals.var_arg_dn10 / (2.0 * assign26990_e36226)) / (assign26990_e36226 * assign26990_e36226))), (-((locals.var_arg_dn11 / (2.0 * assign26990_e36226)) / (assign26990_e36226 * assign26990_e36226))), (-((locals.var_arg_dn12 / (2.0 * assign26990_e36226)) / (assign26990_e36226 * assign26990_e36226))), (-((locals.var_arg_dn13 / (2.0 * assign26990_e36226)) / (assign26990_e36226 * assign26990_e36226))), (-((locals.var_arg_dn14 / (2.0 * assign26990_e36226)) / (assign26990_e36226 * assign26990_e36226))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn12, locals.var_sarg_dn13, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign26990_e36229;
        locals.var_sarg_dn0 = assign26990_e36229_d_n0;
        locals.var_sarg_dn2 = assign26990_e36229_d_n2;
        locals.var_sarg_dn3 = assign26990_e36229_d_n3;
        locals.var_sarg_dn4 = assign26990_e36229_d_n4;
        locals.var_sarg_dn5 = assign26990_e36229_d_n5;
        locals.var_sarg_dn6 = assign26990_e36229_d_n6;
        locals.var_sarg_dn7 = assign26990_e36229_d_n7;
        locals.var_sarg_dn8 = assign26990_e36229_d_n8;
        locals.var_sarg_dn9 = assign26990_e36229_d_n9;
        locals.var_sarg_dn10 = assign26990_e36229_d_n10;
        locals.var_sarg_dn11 = assign26990_e36229_d_n11;
        locals.var_sarg_dn12 = assign26990_e36229_d_n12;
        locals.var_sarg_dn13 = assign26990_e36229_d_n13;
        locals.var_sarg_dn14 = assign26990_e36229_d_n14;
        locals.var_sarg_rv = 0.0;

        let (assign27000_e36247, assign27000_e36247_d_n0, assign27000_e36247_d_n2, assign27000_e36247_d_n3, assign27000_e36247_d_n4, assign27000_e36247_d_n5, assign27000_e36247_d_n6, assign27000_e36247_d_n7, assign27000_e36247_d_n8, assign27000_e36247_d_n9, assign27000_e36247_d_n10, assign27000_e36247_d_n11, assign27000_e36247_d_n12, assign27000_e36247_d_n13, assign27000_e36247_d_n14,) = {
    if (((((locals.var_guard670 != 0.0) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 != 0.0)) && (locals.var_guard674 != 0.0)) && (locals.var_guard675 == 0.0)) {
        let assign27000_e36241: f64 = (-p.p714);
        let assign27000_e36243: f64 = (locals.var_arg).ln();
        let assign27000_e36244: f64 = (assign27000_e36241 * assign27000_e36243);
        let assign27000_e36245: f64 = { let limited_exp_arg = assign27000_e36244; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign27000_e36245, ({ let limited_exp_arg = assign27000_e36244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27000_e36241 * (locals.var_arg_dn0 / locals.var_arg))), ({ let limited_exp_arg = assign27000_e36244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27000_e36241 * (locals.var_arg_dn2 / locals.var_arg))), ({ let limited_exp_arg = assign27000_e36244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27000_e36241 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign27000_e36244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27000_e36241 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign27000_e36244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27000_e36241 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign27000_e36244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27000_e36241 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign27000_e36244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27000_e36241 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign27000_e36244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27000_e36241 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign27000_e36244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27000_e36241 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign27000_e36244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27000_e36241 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign27000_e36244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27000_e36241 * (locals.var_arg_dn11 / locals.var_arg))), ({ let limited_exp_arg = assign27000_e36244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27000_e36241 * (locals.var_arg_dn12 / locals.var_arg))), ({ let limited_exp_arg = assign27000_e36244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27000_e36241 * (locals.var_arg_dn13 / locals.var_arg))), ({ let limited_exp_arg = assign27000_e36244; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27000_e36241 * (locals.var_arg_dn14 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn12, locals.var_sarg_dn13, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign27000_e36247;
        locals.var_sarg_dn0 = assign27000_e36247_d_n0;
        locals.var_sarg_dn2 = assign27000_e36247_d_n2;
        locals.var_sarg_dn3 = assign27000_e36247_d_n3;
        locals.var_sarg_dn4 = assign27000_e36247_d_n4;
        locals.var_sarg_dn5 = assign27000_e36247_d_n5;
        locals.var_sarg_dn6 = assign27000_e36247_d_n6;
        locals.var_sarg_dn7 = assign27000_e36247_d_n7;
        locals.var_sarg_dn8 = assign27000_e36247_d_n8;
        locals.var_sarg_dn9 = assign27000_e36247_d_n9;
        locals.var_sarg_dn10 = assign27000_e36247_d_n10;
        locals.var_sarg_dn11 = assign27000_e36247_d_n11;
        locals.var_sarg_dn12 = assign27000_e36247_d_n12;
        locals.var_sarg_dn13 = assign27000_e36247_d_n13;
        locals.var_sarg_dn14 = assign27000_e36247_d_n14;
        locals.var_sarg_rv = 0.0;

        let (assign27010_e36269, assign27010_e36269_d_n0, assign27010_e36269_d_n2, assign27010_e36269_d_n3, assign27010_e36269_d_n4, assign27010_e36269_d_n5, assign27010_e36269_d_n6, assign27010_e36269_d_n7, assign27010_e36269_d_n8, assign27010_e36269_d_n9, assign27010_e36269_d_n10, assign27010_e36269_d_n11, assign27010_e36269_d_n12, assign27010_e36269_d_n13, assign27010_e36269_d_n14,) = {
    if ((((locals.var_guard670 != 0.0) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 != 0.0)) && (locals.var_guard674 != 0.0)) {
        let assign27010_e36257: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign27010_e36261: f64 = (locals.var_arg * locals.var_sarg);
        let assign27010_e36262: f64 = (1.0 - assign27010_e36261);
        let assign27010_e36263: f64 = (assign27010_e36257 * assign27010_e36262);
        let assign27010_e36266: f64 = (1.0 - p.p714);
        let assign27010_e36267: f64 = (assign27010_e36263 / assign27010_e36266);
        (assign27010_e36267, ((((locals.var_pbd_t * locals.var_czbd_dn0) * assign27010_e36262) + (assign27010_e36257 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign27010_e36266), ((((locals.var_pbd_t * locals.var_czbd_dn2) * assign27010_e36262) + (assign27010_e36257 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign27010_e36266), ((((locals.var_pbd_t * locals.var_czbd_dn3) * assign27010_e36262) + (assign27010_e36257 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3))))) / assign27010_e36266), (((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign27010_e36262) + (assign27010_e36257 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign27010_e36266), ((((locals.var_pbd_t * locals.var_czbd_dn5) * assign27010_e36262) + (assign27010_e36257 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign27010_e36266), ((((locals.var_pbd_t * locals.var_czbd_dn6) * assign27010_e36262) + (assign27010_e36257 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign27010_e36266), ((((locals.var_pbd_t * locals.var_czbd_dn7) * assign27010_e36262) + (assign27010_e36257 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign27010_e36266), ((((locals.var_pbd_t * locals.var_czbd_dn8) * assign27010_e36262) + (assign27010_e36257 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign27010_e36266), ((((locals.var_pbd_t * locals.var_czbd_dn9) * assign27010_e36262) + (assign27010_e36257 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign27010_e36266), ((((locals.var_pbd_t * locals.var_czbd_dn10) * assign27010_e36262) + (assign27010_e36257 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign27010_e36266), ((((locals.var_pbd_t * locals.var_czbd_dn11) * assign27010_e36262) + (assign27010_e36257 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign27010_e36266), ((((locals.var_pbd_t * locals.var_czbd_dn12) * assign27010_e36262) + (assign27010_e36257 * (-((locals.var_arg_dn12 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn12))))) / assign27010_e36266), ((((locals.var_pbd_t * locals.var_czbd_dn13) * assign27010_e36262) + (assign27010_e36257 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign27010_e36266), ((((locals.var_pbd_t * locals.var_czbd_dn14) * assign27010_e36262) + (assign27010_e36257 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign27010_e36266),)
    } else {
        (locals.var_qbdj1_ext, locals.var_qbdj1_ext_dn0, locals.var_qbdj1_ext_dn2, locals.var_qbdj1_ext_dn3, locals.var_qbdj1_ext_dn4, locals.var_qbdj1_ext_dn5, locals.var_qbdj1_ext_dn6, locals.var_qbdj1_ext_dn7, locals.var_qbdj1_ext_dn8, locals.var_qbdj1_ext_dn9, locals.var_qbdj1_ext_dn10, locals.var_qbdj1_ext_dn11, locals.var_qbdj1_ext_dn12, locals.var_qbdj1_ext_dn13, locals.var_qbdj1_ext_dn14,)
    }
};
        locals.var_qbdj1_ext = assign27010_e36269;
        locals.var_qbdj1_ext_dn0 = assign27010_e36269_d_n0;
        locals.var_qbdj1_ext_dn2 = assign27010_e36269_d_n2;
        locals.var_qbdj1_ext_dn3 = assign27010_e36269_d_n3;
        locals.var_qbdj1_ext_dn4 = assign27010_e36269_d_n4;
        locals.var_qbdj1_ext_dn5 = assign27010_e36269_d_n5;
        locals.var_qbdj1_ext_dn6 = assign27010_e36269_d_n6;
        locals.var_qbdj1_ext_dn7 = assign27010_e36269_d_n7;
        locals.var_qbdj1_ext_dn8 = assign27010_e36269_d_n8;
        locals.var_qbdj1_ext_dn9 = assign27010_e36269_d_n9;
        locals.var_qbdj1_ext_dn10 = assign27010_e36269_d_n10;
        locals.var_qbdj1_ext_dn11 = assign27010_e36269_d_n11;
        locals.var_qbdj1_ext_dn12 = assign27010_e36269_d_n12;
        locals.var_qbdj1_ext_dn13 = assign27010_e36269_d_n13;
        locals.var_qbdj1_ext_dn14 = assign27010_e36269_d_n14;
        locals.var_qbdj1_ext_rv = 0.0;

        let (assign27020_e36286, assign27020_e36286_d_n0, assign27020_e36286_d_n2, assign27020_e36286_d_n3, assign27020_e36286_d_n4, assign27020_e36286_d_n5, assign27020_e36286_d_n6, assign27020_e36286_d_n7, assign27020_e36286_d_n8, assign27020_e36286_d_n9, assign27020_e36286_d_n10, assign27020_e36286_d_n11, assign27020_e36286_d_n12, assign27020_e36286_d_n13, assign27020_e36286_d_n14,) = {
    if ((((locals.var_guard670 != 0.0) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 != 0.0)) && (locals.var_guard674 == 0.0)) {
        let assign27020_e36280: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign27020_e36282: f64 = (locals.var_arg).ln();
        let assign27020_e36283: f64 = (-assign27020_e36282);
        let assign27020_e36284: f64 = (assign27020_e36280 * assign27020_e36283);
        (assign27020_e36284, (((locals.var_pbd_t * locals.var_czbd_dn0) * assign27020_e36283) + (assign27020_e36280 * (-(locals.var_arg_dn0 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn2) * assign27020_e36283) + (assign27020_e36280 * (-(locals.var_arg_dn2 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn3) * assign27020_e36283) + (assign27020_e36280 * (-(locals.var_arg_dn3 / locals.var_arg)))), ((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign27020_e36283) + (assign27020_e36280 * (-(locals.var_arg_dn4 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn5) * assign27020_e36283) + (assign27020_e36280 * (-(locals.var_arg_dn5 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn6) * assign27020_e36283) + (assign27020_e36280 * (-(locals.var_arg_dn6 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn7) * assign27020_e36283) + (assign27020_e36280 * (-(locals.var_arg_dn7 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn8) * assign27020_e36283) + (assign27020_e36280 * (-(locals.var_arg_dn8 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn9) * assign27020_e36283) + (assign27020_e36280 * (-(locals.var_arg_dn9 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn10) * assign27020_e36283) + (assign27020_e36280 * (-(locals.var_arg_dn10 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn11) * assign27020_e36283) + (assign27020_e36280 * (-(locals.var_arg_dn11 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn12) * assign27020_e36283) + (assign27020_e36280 * (-(locals.var_arg_dn12 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn13) * assign27020_e36283) + (assign27020_e36280 * (-(locals.var_arg_dn13 / locals.var_arg)))), (((locals.var_pbd_t * locals.var_czbd_dn14) * assign27020_e36283) + (assign27020_e36280 * (-(locals.var_arg_dn14 / locals.var_arg)))),)
    } else {
        (locals.var_qbdj1_ext, locals.var_qbdj1_ext_dn0, locals.var_qbdj1_ext_dn2, locals.var_qbdj1_ext_dn3, locals.var_qbdj1_ext_dn4, locals.var_qbdj1_ext_dn5, locals.var_qbdj1_ext_dn6, locals.var_qbdj1_ext_dn7, locals.var_qbdj1_ext_dn8, locals.var_qbdj1_ext_dn9, locals.var_qbdj1_ext_dn10, locals.var_qbdj1_ext_dn11, locals.var_qbdj1_ext_dn12, locals.var_qbdj1_ext_dn13, locals.var_qbdj1_ext_dn14,)
    }
};
        locals.var_qbdj1_ext = assign27020_e36286;
        locals.var_qbdj1_ext_dn0 = assign27020_e36286_d_n0;
        locals.var_qbdj1_ext_dn2 = assign27020_e36286_d_n2;
        locals.var_qbdj1_ext_dn3 = assign27020_e36286_d_n3;
        locals.var_qbdj1_ext_dn4 = assign27020_e36286_d_n4;
        locals.var_qbdj1_ext_dn5 = assign27020_e36286_d_n5;
        locals.var_qbdj1_ext_dn6 = assign27020_e36286_d_n6;
        locals.var_qbdj1_ext_dn7 = assign27020_e36286_d_n7;
        locals.var_qbdj1_ext_dn8 = assign27020_e36286_d_n8;
        locals.var_qbdj1_ext_dn9 = assign27020_e36286_d_n9;
        locals.var_qbdj1_ext_dn10 = assign27020_e36286_d_n10;
        locals.var_qbdj1_ext_dn11 = assign27020_e36286_d_n11;
        locals.var_qbdj1_ext_dn12 = assign27020_e36286_d_n12;
        locals.var_qbdj1_ext_dn13 = assign27020_e36286_d_n13;
        locals.var_qbdj1_ext_dn14 = assign27020_e36286_d_n14;
        locals.var_qbdj1_ext_rv = 0.0;

        let (assign27030_e36311, assign27030_e36311_d_n0, assign27030_e36311_d_n2, assign27030_e36311_d_n3, assign27030_e36311_d_n4, assign27030_e36311_d_n5, assign27030_e36311_d_n6, assign27030_e36311_d_n7, assign27030_e36311_d_n8, assign27030_e36311_d_n9, assign27030_e36311_d_n10, assign27030_e36311_d_n11, assign27030_e36311_d_n12, assign27030_e36311_d_n13, assign27030_e36311_d_n14,) = {
    if (((locals.var_guard670 != 0.0) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 == 0.0)) {
        let assign27030_e36296: f64 = (locals.var_t1 - 1.0);
        let assign27030_e36297: f64 = (locals.var_czbd_p1 * assign27030_e36296);
        let assign27030_e36300: f64 = (5.0 * p.p714);
        let assign27030_e36303: f64 = (locals.var_t1 - 1.0);
        let assign27030_e36304: f64 = (assign27030_e36300 * assign27030_e36303);
        let assign27030_e36307: f64 = (1.0 + p.p714);
        let assign27030_e36308: f64 = (assign27030_e36304 + assign27030_e36307);
        let assign27030_e36309: f64 = (assign27030_e36297 * assign27030_e36308);
        (assign27030_e36309, (((locals.var_czbd_p1 * locals.var_t1_dn0) * assign27030_e36308) + (assign27030_e36297 * (assign27030_e36300 * locals.var_t1_dn0))), (((locals.var_czbd_p1 * locals.var_t1_dn2) * assign27030_e36308) + (assign27030_e36297 * (assign27030_e36300 * locals.var_t1_dn2))), (((locals.var_czbd_p1 * locals.var_t1_dn3) * assign27030_e36308) + (assign27030_e36297 * (assign27030_e36300 * locals.var_t1_dn3))), (((locals.var_czbd_p1 * locals.var_t1_dn4) * assign27030_e36308) + (assign27030_e36297 * (assign27030_e36300 * locals.var_t1_dn4))), (((locals.var_czbd_p1 * locals.var_t1_dn5) * assign27030_e36308) + (assign27030_e36297 * (assign27030_e36300 * locals.var_t1_dn5))), (((locals.var_czbd_p1 * locals.var_t1_dn6) * assign27030_e36308) + (assign27030_e36297 * (assign27030_e36300 * locals.var_t1_dn6))), (((locals.var_czbd_p1 * locals.var_t1_dn7) * assign27030_e36308) + (assign27030_e36297 * (assign27030_e36300 * locals.var_t1_dn7))), (((locals.var_czbd_p1 * locals.var_t1_dn8) * assign27030_e36308) + (assign27030_e36297 * (assign27030_e36300 * locals.var_t1_dn8))), (((locals.var_czbd_p1 * locals.var_t1_dn9) * assign27030_e36308) + (assign27030_e36297 * (assign27030_e36300 * locals.var_t1_dn9))), (((locals.var_czbd_p1 * locals.var_t1_dn10) * assign27030_e36308) + (assign27030_e36297 * (assign27030_e36300 * locals.var_t1_dn10))), (((locals.var_czbd_p1 * locals.var_t1_dn11) * assign27030_e36308) + (assign27030_e36297 * (assign27030_e36300 * locals.var_t1_dn11))), (((locals.var_czbd_p1 * locals.var_t1_dn12) * assign27030_e36308) + (assign27030_e36297 * (assign27030_e36300 * locals.var_t1_dn12))), (((locals.var_czbd_p1 * locals.var_t1_dn13) * assign27030_e36308) + (assign27030_e36297 * (assign27030_e36300 * locals.var_t1_dn13))), (((locals.var_czbd_p1 * locals.var_t1_dn14) * assign27030_e36308) + (assign27030_e36297 * (assign27030_e36300 * locals.var_t1_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign27030_e36311;
        locals.var_t2_dn0 = assign27030_e36311_d_n0;
        locals.var_t2_dn2 = assign27030_e36311_d_n2;
        locals.var_t2_dn3 = assign27030_e36311_d_n3;
        locals.var_t2_dn4 = assign27030_e36311_d_n4;
        locals.var_t2_dn5 = assign27030_e36311_d_n5;
        locals.var_t2_dn6 = assign27030_e36311_d_n6;
        locals.var_t2_dn7 = assign27030_e36311_d_n7;
        locals.var_t2_dn8 = assign27030_e36311_d_n8;
        locals.var_t2_dn9 = assign27030_e36311_d_n9;
        locals.var_t2_dn10 = assign27030_e36311_d_n10;
        locals.var_t2_dn11 = assign27030_e36311_d_n11;
        locals.var_t2_dn12 = assign27030_e36311_d_n12;
        locals.var_t2_dn13 = assign27030_e36311_d_n13;
        locals.var_t2_dn14 = assign27030_e36311_d_n14;
        locals.var_t2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_81(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (assign27040_e36326, assign27040_e36326_d_n0, assign27040_e36326_d_n2, assign27040_e36326_d_n3, assign27040_e36326_d_n4, assign27040_e36326_d_n5, assign27040_e36326_d_n6, assign27040_e36326_d_n7, assign27040_e36326_d_n8, assign27040_e36326_d_n9, assign27040_e36326_d_n10, assign27040_e36326_d_n11, assign27040_e36326_d_n12, assign27040_e36326_d_n13, assign27040_e36326_d_n14,) = {
    if (((locals.var_guard670 != 0.0) && (locals.var_guard672 != 0.0)) && (locals.var_guard673 == 0.0)) {
        let assign27040_e36320: f64 = (locals.var_pbd_t * locals.var_czbd);
        let assign27040_e36323: f64 = (locals.var_t2 + locals.var_czbd_p2);
        let assign27040_e36324: f64 = (assign27040_e36320 * assign27040_e36323);
        (assign27040_e36324, (((locals.var_pbd_t * locals.var_czbd_dn0) * assign27040_e36323) + (assign27040_e36320 * locals.var_t2_dn0)), (((locals.var_pbd_t * locals.var_czbd_dn2) * assign27040_e36323) + (assign27040_e36320 * locals.var_t2_dn2)), (((locals.var_pbd_t * locals.var_czbd_dn3) * assign27040_e36323) + (assign27040_e36320 * locals.var_t2_dn3)), ((((locals.var_pbd_t_dn4 * locals.var_czbd) + (locals.var_pbd_t * locals.var_czbd_dn4)) * assign27040_e36323) + (assign27040_e36320 * locals.var_t2_dn4)), (((locals.var_pbd_t * locals.var_czbd_dn5) * assign27040_e36323) + (assign27040_e36320 * locals.var_t2_dn5)), (((locals.var_pbd_t * locals.var_czbd_dn6) * assign27040_e36323) + (assign27040_e36320 * locals.var_t2_dn6)), (((locals.var_pbd_t * locals.var_czbd_dn7) * assign27040_e36323) + (assign27040_e36320 * locals.var_t2_dn7)), (((locals.var_pbd_t * locals.var_czbd_dn8) * assign27040_e36323) + (assign27040_e36320 * locals.var_t2_dn8)), (((locals.var_pbd_t * locals.var_czbd_dn9) * assign27040_e36323) + (assign27040_e36320 * locals.var_t2_dn9)), (((locals.var_pbd_t * locals.var_czbd_dn10) * assign27040_e36323) + (assign27040_e36320 * locals.var_t2_dn10)), (((locals.var_pbd_t * locals.var_czbd_dn11) * assign27040_e36323) + (assign27040_e36320 * locals.var_t2_dn11)), (((locals.var_pbd_t * locals.var_czbd_dn12) * assign27040_e36323) + (assign27040_e36320 * locals.var_t2_dn12)), (((locals.var_pbd_t * locals.var_czbd_dn13) * assign27040_e36323) + (assign27040_e36320 * locals.var_t2_dn13)), (((locals.var_pbd_t * locals.var_czbd_dn14) * assign27040_e36323) + (assign27040_e36320 * locals.var_t2_dn14)),)
    } else {
        (locals.var_qbdj1_ext, locals.var_qbdj1_ext_dn0, locals.var_qbdj1_ext_dn2, locals.var_qbdj1_ext_dn3, locals.var_qbdj1_ext_dn4, locals.var_qbdj1_ext_dn5, locals.var_qbdj1_ext_dn6, locals.var_qbdj1_ext_dn7, locals.var_qbdj1_ext_dn8, locals.var_qbdj1_ext_dn9, locals.var_qbdj1_ext_dn10, locals.var_qbdj1_ext_dn11, locals.var_qbdj1_ext_dn12, locals.var_qbdj1_ext_dn13, locals.var_qbdj1_ext_dn14,)
    }
};
        locals.var_qbdj1_ext = assign27040_e36326;
        locals.var_qbdj1_ext_dn0 = assign27040_e36326_d_n0;
        locals.var_qbdj1_ext_dn2 = assign27040_e36326_d_n2;
        locals.var_qbdj1_ext_dn3 = assign27040_e36326_d_n3;
        locals.var_qbdj1_ext_dn4 = assign27040_e36326_d_n4;
        locals.var_qbdj1_ext_dn5 = assign27040_e36326_d_n5;
        locals.var_qbdj1_ext_dn6 = assign27040_e36326_d_n6;
        locals.var_qbdj1_ext_dn7 = assign27040_e36326_d_n7;
        locals.var_qbdj1_ext_dn8 = assign27040_e36326_d_n8;
        locals.var_qbdj1_ext_dn9 = assign27040_e36326_d_n9;
        locals.var_qbdj1_ext_dn10 = assign27040_e36326_d_n10;
        locals.var_qbdj1_ext_dn11 = assign27040_e36326_d_n11;
        locals.var_qbdj1_ext_dn12 = assign27040_e36326_d_n12;
        locals.var_qbdj1_ext_dn13 = assign27040_e36326_d_n13;
        locals.var_qbdj1_ext_dn14 = assign27040_e36326_d_n14;
        locals.var_qbdj1_ext_rv = 0.0;

        let (assign27050_e36333, assign27050_e36333_d_n0, assign27050_e36333_d_n2, assign27050_e36333_d_n3, assign27050_e36333_d_n4, assign27050_e36333_d_n5, assign27050_e36333_d_n6, assign27050_e36333_d_n7, assign27050_e36333_d_n8, assign27050_e36333_d_n9, assign27050_e36333_d_n10, assign27050_e36333_d_n11, assign27050_e36333_d_n12, assign27050_e36333_d_n13, assign27050_e36333_d_n14,) = {
    if ((locals.var_guard670 != 0.0) && (locals.var_guard672 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdj1_ext, locals.var_qbdj1_ext_dn0, locals.var_qbdj1_ext_dn2, locals.var_qbdj1_ext_dn3, locals.var_qbdj1_ext_dn4, locals.var_qbdj1_ext_dn5, locals.var_qbdj1_ext_dn6, locals.var_qbdj1_ext_dn7, locals.var_qbdj1_ext_dn8, locals.var_qbdj1_ext_dn9, locals.var_qbdj1_ext_dn10, locals.var_qbdj1_ext_dn11, locals.var_qbdj1_ext_dn12, locals.var_qbdj1_ext_dn13, locals.var_qbdj1_ext_dn14,)
    }
};
        locals.var_qbdj1_ext = assign27050_e36333;
        locals.var_qbdj1_ext_dn0 = assign27050_e36333_d_n0;
        locals.var_qbdj1_ext_dn2 = assign27050_e36333_d_n2;
        locals.var_qbdj1_ext_dn3 = assign27050_e36333_d_n3;
        locals.var_qbdj1_ext_dn4 = assign27050_e36333_d_n4;
        locals.var_qbdj1_ext_dn5 = assign27050_e36333_d_n5;
        locals.var_qbdj1_ext_dn6 = assign27050_e36333_d_n6;
        locals.var_qbdj1_ext_dn7 = assign27050_e36333_d_n7;
        locals.var_qbdj1_ext_dn8 = assign27050_e36333_d_n8;
        locals.var_qbdj1_ext_dn9 = assign27050_e36333_d_n9;
        locals.var_qbdj1_ext_dn10 = assign27050_e36333_d_n10;
        locals.var_qbdj1_ext_dn11 = assign27050_e36333_d_n11;
        locals.var_qbdj1_ext_dn12 = assign27050_e36333_d_n12;
        locals.var_qbdj1_ext_dn13 = assign27050_e36333_d_n13;
        locals.var_qbdj1_ext_dn14 = assign27050_e36333_d_n14;
        locals.var_qbdj1_ext_rv = 0.0;

        let assign27060_e36336: f64 = if locals.var_czbdsw > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard676 = assign27060_e36336;
        locals.var_guard676_rv = 0.0;

        let (assign27070_e36344, assign27070_e36344_d_n0, assign27070_e36344_d_n2, assign27070_e36344_d_n3, assign27070_e36344_d_n4, assign27070_e36344_d_n5, assign27070_e36344_d_n6, assign27070_e36344_d_n7, assign27070_e36344_d_n8, assign27070_e36344_d_n9, assign27070_e36344_d_n10, assign27070_e36344_d_n11, assign27070_e36344_d_n12, assign27070_e36344_d_n13, assign27070_e36344_d_n14,) = {
    if ((locals.var_guard670 != 0.0) && (locals.var_guard676 != 0.0)) {
        let assign27070_e36342: f64 = (locals.var_vbd_ext / locals.var_pbswd_t);
        (assign27070_e36342, 0.0, 0.0, 0.0, (-((locals.var_vbd_ext * locals.var_pbswd_t_dn4) / (locals.var_pbswd_t * locals.var_pbswd_t))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, (locals.var_vbd_ext_dn13 / locals.var_pbswd_t), (locals.var_vbd_ext_dn14 / locals.var_pbswd_t),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign27070_e36344;
        locals.var_t1_dn0 = assign27070_e36344_d_n0;
        locals.var_t1_dn2 = assign27070_e36344_d_n2;
        locals.var_t1_dn3 = assign27070_e36344_d_n3;
        locals.var_t1_dn4 = assign27070_e36344_d_n4;
        locals.var_t1_dn5 = assign27070_e36344_d_n5;
        locals.var_t1_dn6 = assign27070_e36344_d_n6;
        locals.var_t1_dn7 = assign27070_e36344_d_n7;
        locals.var_t1_dn8 = assign27070_e36344_d_n8;
        locals.var_t1_dn9 = assign27070_e36344_d_n9;
        locals.var_t1_dn10 = assign27070_e36344_d_n10;
        locals.var_t1_dn11 = assign27070_e36344_d_n11;
        locals.var_t1_dn12 = assign27070_e36344_d_n12;
        locals.var_t1_dn13 = assign27070_e36344_d_n13;
        locals.var_t1_dn14 = assign27070_e36344_d_n14;
        locals.var_t1_rv = 0.0;

        let assign27080_e36347: f64 = if locals.var_t1 < 0.9 { 1.0 } else { 0.0 };
        locals.var_guard677 = assign27080_e36347;
        locals.var_guard677_rv = 0.0;

        let (assign27090_e36357, assign27090_e36357_d_n0, assign27090_e36357_d_n2, assign27090_e36357_d_n3, assign27090_e36357_d_n4, assign27090_e36357_d_n5, assign27090_e36357_d_n6, assign27090_e36357_d_n7, assign27090_e36357_d_n8, assign27090_e36357_d_n9, assign27090_e36357_d_n10, assign27090_e36357_d_n11, assign27090_e36357_d_n12, assign27090_e36357_d_n13, assign27090_e36357_d_n14,) = {
    if (((locals.var_guard670 != 0.0) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 != 0.0)) {
        let assign27090_e36355: f64 = (1.0 - locals.var_t1);
        (assign27090_e36355, (-locals.var_t1_dn0), (-locals.var_t1_dn2), (-locals.var_t1_dn3), (-locals.var_t1_dn4), (-locals.var_t1_dn5), (-locals.var_t1_dn6), (-locals.var_t1_dn7), (-locals.var_t1_dn8), (-locals.var_t1_dn9), (-locals.var_t1_dn10), (-locals.var_t1_dn11), (-locals.var_t1_dn12), (-locals.var_t1_dn13), (-locals.var_t1_dn14),)
    } else {
        (locals.var_arg, locals.var_arg_dn0, locals.var_arg_dn2, locals.var_arg_dn3, locals.var_arg_dn4, locals.var_arg_dn5, locals.var_arg_dn6, locals.var_arg_dn7, locals.var_arg_dn8, locals.var_arg_dn9, locals.var_arg_dn10, locals.var_arg_dn11, locals.var_arg_dn12, locals.var_arg_dn13, locals.var_arg_dn14,)
    }
};
        locals.var_arg = assign27090_e36357;
        locals.var_arg_dn0 = assign27090_e36357_d_n0;
        locals.var_arg_dn2 = assign27090_e36357_d_n2;
        locals.var_arg_dn3 = assign27090_e36357_d_n3;
        locals.var_arg_dn4 = assign27090_e36357_d_n4;
        locals.var_arg_dn5 = assign27090_e36357_d_n5;
        locals.var_arg_dn6 = assign27090_e36357_d_n6;
        locals.var_arg_dn7 = assign27090_e36357_d_n7;
        locals.var_arg_dn8 = assign27090_e36357_d_n8;
        locals.var_arg_dn9 = assign27090_e36357_d_n9;
        locals.var_arg_dn10 = assign27090_e36357_d_n10;
        locals.var_arg_dn11 = assign27090_e36357_d_n11;
        locals.var_arg_dn12 = assign27090_e36357_d_n12;
        locals.var_arg_dn13 = assign27090_e36357_d_n13;
        locals.var_arg_dn14 = assign27090_e36357_d_n14;
        locals.var_arg_rv = 0.0;

        let assign27100_e36360: f64 = if p.p716 != 1.0 { 1.0 } else { 0.0 };
        locals.var_guard678 = assign27100_e36360;
        locals.var_guard678_rv = 0.0;

        let assign27110_e36363: f64 = if p.p716 == 0.5 { 1.0 } else { 0.0 };
        locals.var_guard679 = assign27110_e36363;
        locals.var_guard679_rv = 0.0;

        let (assign27120_e36378, assign27120_e36378_d_n0, assign27120_e36378_d_n2, assign27120_e36378_d_n3, assign27120_e36378_d_n4, assign27120_e36378_d_n5, assign27120_e36378_d_n6, assign27120_e36378_d_n7, assign27120_e36378_d_n8, assign27120_e36378_d_n9, assign27120_e36378_d_n10, assign27120_e36378_d_n11, assign27120_e36378_d_n12, assign27120_e36378_d_n13, assign27120_e36378_d_n14,) = {
    if (((((locals.var_guard670 != 0.0) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 != 0.0)) && (locals.var_guard678 != 0.0)) && (locals.var_guard679 != 0.0)) {
        let assign27120_e36375: f64 = (locals.var_arg).sqrt();
        let assign27120_e36376: f64 = (1.0 / assign27120_e36375);
        (assign27120_e36376, (-((locals.var_arg_dn0 / (2.0 * assign27120_e36375)) / (assign27120_e36375 * assign27120_e36375))), (-((locals.var_arg_dn2 / (2.0 * assign27120_e36375)) / (assign27120_e36375 * assign27120_e36375))), (-((locals.var_arg_dn3 / (2.0 * assign27120_e36375)) / (assign27120_e36375 * assign27120_e36375))), (-((locals.var_arg_dn4 / (2.0 * assign27120_e36375)) / (assign27120_e36375 * assign27120_e36375))), (-((locals.var_arg_dn5 / (2.0 * assign27120_e36375)) / (assign27120_e36375 * assign27120_e36375))), (-((locals.var_arg_dn6 / (2.0 * assign27120_e36375)) / (assign27120_e36375 * assign27120_e36375))), (-((locals.var_arg_dn7 / (2.0 * assign27120_e36375)) / (assign27120_e36375 * assign27120_e36375))), (-((locals.var_arg_dn8 / (2.0 * assign27120_e36375)) / (assign27120_e36375 * assign27120_e36375))), (-((locals.var_arg_dn9 / (2.0 * assign27120_e36375)) / (assign27120_e36375 * assign27120_e36375))), (-((locals.var_arg_dn10 / (2.0 * assign27120_e36375)) / (assign27120_e36375 * assign27120_e36375))), (-((locals.var_arg_dn11 / (2.0 * assign27120_e36375)) / (assign27120_e36375 * assign27120_e36375))), (-((locals.var_arg_dn12 / (2.0 * assign27120_e36375)) / (assign27120_e36375 * assign27120_e36375))), (-((locals.var_arg_dn13 / (2.0 * assign27120_e36375)) / (assign27120_e36375 * assign27120_e36375))), (-((locals.var_arg_dn14 / (2.0 * assign27120_e36375)) / (assign27120_e36375 * assign27120_e36375))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn12, locals.var_sarg_dn13, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign27120_e36378;
        locals.var_sarg_dn0 = assign27120_e36378_d_n0;
        locals.var_sarg_dn2 = assign27120_e36378_d_n2;
        locals.var_sarg_dn3 = assign27120_e36378_d_n3;
        locals.var_sarg_dn4 = assign27120_e36378_d_n4;
        locals.var_sarg_dn5 = assign27120_e36378_d_n5;
        locals.var_sarg_dn6 = assign27120_e36378_d_n6;
        locals.var_sarg_dn7 = assign27120_e36378_d_n7;
        locals.var_sarg_dn8 = assign27120_e36378_d_n8;
        locals.var_sarg_dn9 = assign27120_e36378_d_n9;
        locals.var_sarg_dn10 = assign27120_e36378_d_n10;
        locals.var_sarg_dn11 = assign27120_e36378_d_n11;
        locals.var_sarg_dn12 = assign27120_e36378_d_n12;
        locals.var_sarg_dn13 = assign27120_e36378_d_n13;
        locals.var_sarg_dn14 = assign27120_e36378_d_n14;
        locals.var_sarg_rv = 0.0;

        let (assign27130_e36396, assign27130_e36396_d_n0, assign27130_e36396_d_n2, assign27130_e36396_d_n3, assign27130_e36396_d_n4, assign27130_e36396_d_n5, assign27130_e36396_d_n6, assign27130_e36396_d_n7, assign27130_e36396_d_n8, assign27130_e36396_d_n9, assign27130_e36396_d_n10, assign27130_e36396_d_n11, assign27130_e36396_d_n12, assign27130_e36396_d_n13, assign27130_e36396_d_n14,) = {
    if (((((locals.var_guard670 != 0.0) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 != 0.0)) && (locals.var_guard678 != 0.0)) && (locals.var_guard679 == 0.0)) {
        let assign27130_e36390: f64 = (-p.p716);
        let assign27130_e36392: f64 = (locals.var_arg).ln();
        let assign27130_e36393: f64 = (assign27130_e36390 * assign27130_e36392);
        let assign27130_e36394: f64 = { let limited_exp_arg = assign27130_e36393; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign27130_e36394, ({ let limited_exp_arg = assign27130_e36393; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27130_e36390 * (locals.var_arg_dn0 / locals.var_arg))), ({ let limited_exp_arg = assign27130_e36393; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27130_e36390 * (locals.var_arg_dn2 / locals.var_arg))), ({ let limited_exp_arg = assign27130_e36393; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27130_e36390 * (locals.var_arg_dn3 / locals.var_arg))), ({ let limited_exp_arg = assign27130_e36393; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27130_e36390 * (locals.var_arg_dn4 / locals.var_arg))), ({ let limited_exp_arg = assign27130_e36393; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27130_e36390 * (locals.var_arg_dn5 / locals.var_arg))), ({ let limited_exp_arg = assign27130_e36393; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27130_e36390 * (locals.var_arg_dn6 / locals.var_arg))), ({ let limited_exp_arg = assign27130_e36393; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27130_e36390 * (locals.var_arg_dn7 / locals.var_arg))), ({ let limited_exp_arg = assign27130_e36393; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27130_e36390 * (locals.var_arg_dn8 / locals.var_arg))), ({ let limited_exp_arg = assign27130_e36393; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27130_e36390 * (locals.var_arg_dn9 / locals.var_arg))), ({ let limited_exp_arg = assign27130_e36393; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27130_e36390 * (locals.var_arg_dn10 / locals.var_arg))), ({ let limited_exp_arg = assign27130_e36393; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27130_e36390 * (locals.var_arg_dn11 / locals.var_arg))), ({ let limited_exp_arg = assign27130_e36393; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27130_e36390 * (locals.var_arg_dn12 / locals.var_arg))), ({ let limited_exp_arg = assign27130_e36393; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27130_e36390 * (locals.var_arg_dn13 / locals.var_arg))), ({ let limited_exp_arg = assign27130_e36393; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (assign27130_e36390 * (locals.var_arg_dn14 / locals.var_arg))),)
    } else {
        (locals.var_sarg, locals.var_sarg_dn0, locals.var_sarg_dn2, locals.var_sarg_dn3, locals.var_sarg_dn4, locals.var_sarg_dn5, locals.var_sarg_dn6, locals.var_sarg_dn7, locals.var_sarg_dn8, locals.var_sarg_dn9, locals.var_sarg_dn10, locals.var_sarg_dn11, locals.var_sarg_dn12, locals.var_sarg_dn13, locals.var_sarg_dn14,)
    }
};
        locals.var_sarg = assign27130_e36396;
        locals.var_sarg_dn0 = assign27130_e36396_d_n0;
        locals.var_sarg_dn2 = assign27130_e36396_d_n2;
        locals.var_sarg_dn3 = assign27130_e36396_d_n3;
        locals.var_sarg_dn4 = assign27130_e36396_d_n4;
        locals.var_sarg_dn5 = assign27130_e36396_d_n5;
        locals.var_sarg_dn6 = assign27130_e36396_d_n6;
        locals.var_sarg_dn7 = assign27130_e36396_d_n7;
        locals.var_sarg_dn8 = assign27130_e36396_d_n8;
        locals.var_sarg_dn9 = assign27130_e36396_d_n9;
        locals.var_sarg_dn10 = assign27130_e36396_d_n10;
        locals.var_sarg_dn11 = assign27130_e36396_d_n11;
        locals.var_sarg_dn12 = assign27130_e36396_d_n12;
        locals.var_sarg_dn13 = assign27130_e36396_d_n13;
        locals.var_sarg_dn14 = assign27130_e36396_d_n14;
        locals.var_sarg_rv = 0.0;

        let (assign27140_e36418, assign27140_e36418_d_n0, assign27140_e36418_d_n2, assign27140_e36418_d_n3, assign27140_e36418_d_n4, assign27140_e36418_d_n5, assign27140_e36418_d_n6, assign27140_e36418_d_n7, assign27140_e36418_d_n8, assign27140_e36418_d_n9, assign27140_e36418_d_n10, assign27140_e36418_d_n11, assign27140_e36418_d_n12, assign27140_e36418_d_n13, assign27140_e36418_d_n14,) = {
    if ((((locals.var_guard670 != 0.0) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 != 0.0)) && (locals.var_guard678 != 0.0)) {
        let assign27140_e36406: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign27140_e36410: f64 = (locals.var_arg * locals.var_sarg);
        let assign27140_e36411: f64 = (1.0 - assign27140_e36410);
        let assign27140_e36412: f64 = (assign27140_e36406 * assign27140_e36411);
        let assign27140_e36415: f64 = (1.0 - p.p716);
        let assign27140_e36416: f64 = (assign27140_e36412 / assign27140_e36415);
        (assign27140_e36416, ((((locals.var_pbswd_t * locals.var_czbdsw_dn0) * assign27140_e36411) + (assign27140_e36406 * (-((locals.var_arg_dn0 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn0))))) / assign27140_e36415), ((((locals.var_pbswd_t * locals.var_czbdsw_dn2) * assign27140_e36411) + (assign27140_e36406 * (-((locals.var_arg_dn2 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn2))))) / assign27140_e36415), ((((locals.var_pbswd_t * locals.var_czbdsw_dn3) * assign27140_e36411) + (assign27140_e36406 * (-((locals.var_arg_dn3 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn3))))) / assign27140_e36415), (((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign27140_e36411) + (assign27140_e36406 * (-((locals.var_arg_dn4 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn4))))) / assign27140_e36415), ((((locals.var_pbswd_t * locals.var_czbdsw_dn5) * assign27140_e36411) + (assign27140_e36406 * (-((locals.var_arg_dn5 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn5))))) / assign27140_e36415), ((((locals.var_pbswd_t * locals.var_czbdsw_dn6) * assign27140_e36411) + (assign27140_e36406 * (-((locals.var_arg_dn6 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn6))))) / assign27140_e36415), ((((locals.var_pbswd_t * locals.var_czbdsw_dn7) * assign27140_e36411) + (assign27140_e36406 * (-((locals.var_arg_dn7 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn7))))) / assign27140_e36415), ((((locals.var_pbswd_t * locals.var_czbdsw_dn8) * assign27140_e36411) + (assign27140_e36406 * (-((locals.var_arg_dn8 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn8))))) / assign27140_e36415), ((((locals.var_pbswd_t * locals.var_czbdsw_dn9) * assign27140_e36411) + (assign27140_e36406 * (-((locals.var_arg_dn9 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn9))))) / assign27140_e36415), ((((locals.var_pbswd_t * locals.var_czbdsw_dn10) * assign27140_e36411) + (assign27140_e36406 * (-((locals.var_arg_dn10 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn10))))) / assign27140_e36415), ((((locals.var_pbswd_t * locals.var_czbdsw_dn11) * assign27140_e36411) + (assign27140_e36406 * (-((locals.var_arg_dn11 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn11))))) / assign27140_e36415), ((((locals.var_pbswd_t * locals.var_czbdsw_dn12) * assign27140_e36411) + (assign27140_e36406 * (-((locals.var_arg_dn12 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn12))))) / assign27140_e36415), ((((locals.var_pbswd_t * locals.var_czbdsw_dn13) * assign27140_e36411) + (assign27140_e36406 * (-((locals.var_arg_dn13 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn13))))) / assign27140_e36415), ((((locals.var_pbswd_t * locals.var_czbdsw_dn14) * assign27140_e36411) + (assign27140_e36406 * (-((locals.var_arg_dn14 * locals.var_sarg) + (locals.var_arg * locals.var_sarg_dn14))))) / assign27140_e36415),)
    } else {
        (locals.var_qbdj2_ext, locals.var_qbdj2_ext_dn0, locals.var_qbdj2_ext_dn2, locals.var_qbdj2_ext_dn3, locals.var_qbdj2_ext_dn4, locals.var_qbdj2_ext_dn5, locals.var_qbdj2_ext_dn6, locals.var_qbdj2_ext_dn7, locals.var_qbdj2_ext_dn8, locals.var_qbdj2_ext_dn9, locals.var_qbdj2_ext_dn10, locals.var_qbdj2_ext_dn11, locals.var_qbdj2_ext_dn12, locals.var_qbdj2_ext_dn13, locals.var_qbdj2_ext_dn14,)
    }
};
        locals.var_qbdj2_ext = assign27140_e36418;
        locals.var_qbdj2_ext_dn0 = assign27140_e36418_d_n0;
        locals.var_qbdj2_ext_dn2 = assign27140_e36418_d_n2;
        locals.var_qbdj2_ext_dn3 = assign27140_e36418_d_n3;
        locals.var_qbdj2_ext_dn4 = assign27140_e36418_d_n4;
        locals.var_qbdj2_ext_dn5 = assign27140_e36418_d_n5;
        locals.var_qbdj2_ext_dn6 = assign27140_e36418_d_n6;
        locals.var_qbdj2_ext_dn7 = assign27140_e36418_d_n7;
        locals.var_qbdj2_ext_dn8 = assign27140_e36418_d_n8;
        locals.var_qbdj2_ext_dn9 = assign27140_e36418_d_n9;
        locals.var_qbdj2_ext_dn10 = assign27140_e36418_d_n10;
        locals.var_qbdj2_ext_dn11 = assign27140_e36418_d_n11;
        locals.var_qbdj2_ext_dn12 = assign27140_e36418_d_n12;
        locals.var_qbdj2_ext_dn13 = assign27140_e36418_d_n13;
        locals.var_qbdj2_ext_dn14 = assign27140_e36418_d_n14;
        locals.var_qbdj2_ext_rv = 0.0;

        let (assign27150_e36435, assign27150_e36435_d_n0, assign27150_e36435_d_n2, assign27150_e36435_d_n3, assign27150_e36435_d_n4, assign27150_e36435_d_n5, assign27150_e36435_d_n6, assign27150_e36435_d_n7, assign27150_e36435_d_n8, assign27150_e36435_d_n9, assign27150_e36435_d_n10, assign27150_e36435_d_n11, assign27150_e36435_d_n12, assign27150_e36435_d_n13, assign27150_e36435_d_n14,) = {
    if ((((locals.var_guard670 != 0.0) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 != 0.0)) && (locals.var_guard678 == 0.0)) {
        let assign27150_e36429: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign27150_e36431: f64 = (locals.var_arg).ln();
        let assign27150_e36432: f64 = (-assign27150_e36431);
        let assign27150_e36433: f64 = (assign27150_e36429 * assign27150_e36432);
        (assign27150_e36433, (((locals.var_pbswd_t * locals.var_czbdsw_dn0) * assign27150_e36432) + (assign27150_e36429 * (-(locals.var_arg_dn0 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn2) * assign27150_e36432) + (assign27150_e36429 * (-(locals.var_arg_dn2 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn3) * assign27150_e36432) + (assign27150_e36429 * (-(locals.var_arg_dn3 / locals.var_arg)))), ((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign27150_e36432) + (assign27150_e36429 * (-(locals.var_arg_dn4 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn5) * assign27150_e36432) + (assign27150_e36429 * (-(locals.var_arg_dn5 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn6) * assign27150_e36432) + (assign27150_e36429 * (-(locals.var_arg_dn6 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn7) * assign27150_e36432) + (assign27150_e36429 * (-(locals.var_arg_dn7 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn8) * assign27150_e36432) + (assign27150_e36429 * (-(locals.var_arg_dn8 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn9) * assign27150_e36432) + (assign27150_e36429 * (-(locals.var_arg_dn9 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn10) * assign27150_e36432) + (assign27150_e36429 * (-(locals.var_arg_dn10 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn11) * assign27150_e36432) + (assign27150_e36429 * (-(locals.var_arg_dn11 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn12) * assign27150_e36432) + (assign27150_e36429 * (-(locals.var_arg_dn12 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn13) * assign27150_e36432) + (assign27150_e36429 * (-(locals.var_arg_dn13 / locals.var_arg)))), (((locals.var_pbswd_t * locals.var_czbdsw_dn14) * assign27150_e36432) + (assign27150_e36429 * (-(locals.var_arg_dn14 / locals.var_arg)))),)
    } else {
        (locals.var_qbdj2_ext, locals.var_qbdj2_ext_dn0, locals.var_qbdj2_ext_dn2, locals.var_qbdj2_ext_dn3, locals.var_qbdj2_ext_dn4, locals.var_qbdj2_ext_dn5, locals.var_qbdj2_ext_dn6, locals.var_qbdj2_ext_dn7, locals.var_qbdj2_ext_dn8, locals.var_qbdj2_ext_dn9, locals.var_qbdj2_ext_dn10, locals.var_qbdj2_ext_dn11, locals.var_qbdj2_ext_dn12, locals.var_qbdj2_ext_dn13, locals.var_qbdj2_ext_dn14,)
    }
};
        locals.var_qbdj2_ext = assign27150_e36435;
        locals.var_qbdj2_ext_dn0 = assign27150_e36435_d_n0;
        locals.var_qbdj2_ext_dn2 = assign27150_e36435_d_n2;
        locals.var_qbdj2_ext_dn3 = assign27150_e36435_d_n3;
        locals.var_qbdj2_ext_dn4 = assign27150_e36435_d_n4;
        locals.var_qbdj2_ext_dn5 = assign27150_e36435_d_n5;
        locals.var_qbdj2_ext_dn6 = assign27150_e36435_d_n6;
        locals.var_qbdj2_ext_dn7 = assign27150_e36435_d_n7;
        locals.var_qbdj2_ext_dn8 = assign27150_e36435_d_n8;
        locals.var_qbdj2_ext_dn9 = assign27150_e36435_d_n9;
        locals.var_qbdj2_ext_dn10 = assign27150_e36435_d_n10;
        locals.var_qbdj2_ext_dn11 = assign27150_e36435_d_n11;
        locals.var_qbdj2_ext_dn12 = assign27150_e36435_d_n12;
        locals.var_qbdj2_ext_dn13 = assign27150_e36435_d_n13;
        locals.var_qbdj2_ext_dn14 = assign27150_e36435_d_n14;
        locals.var_qbdj2_ext_rv = 0.0;

        let (assign27160_e36460, assign27160_e36460_d_n0, assign27160_e36460_d_n2, assign27160_e36460_d_n3, assign27160_e36460_d_n4, assign27160_e36460_d_n5, assign27160_e36460_d_n6, assign27160_e36460_d_n7, assign27160_e36460_d_n8, assign27160_e36460_d_n9, assign27160_e36460_d_n10, assign27160_e36460_d_n11, assign27160_e36460_d_n12, assign27160_e36460_d_n13, assign27160_e36460_d_n14,) = {
    if (((locals.var_guard670 != 0.0) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 == 0.0)) {
        let assign27160_e36445: f64 = (locals.var_t1 - 1.0);
        let assign27160_e36446: f64 = (locals.var_czbdsw_p1 * assign27160_e36445);
        let assign27160_e36449: f64 = (5.0 * p.p716);
        let assign27160_e36452: f64 = (locals.var_t1 - 1.0);
        let assign27160_e36453: f64 = (assign27160_e36449 * assign27160_e36452);
        let assign27160_e36456: f64 = (1.0 + p.p716);
        let assign27160_e36457: f64 = (assign27160_e36453 + assign27160_e36456);
        let assign27160_e36458: f64 = (assign27160_e36446 * assign27160_e36457);
        (assign27160_e36458, (((locals.var_czbdsw_p1 * locals.var_t1_dn0) * assign27160_e36457) + (assign27160_e36446 * (assign27160_e36449 * locals.var_t1_dn0))), (((locals.var_czbdsw_p1 * locals.var_t1_dn2) * assign27160_e36457) + (assign27160_e36446 * (assign27160_e36449 * locals.var_t1_dn2))), (((locals.var_czbdsw_p1 * locals.var_t1_dn3) * assign27160_e36457) + (assign27160_e36446 * (assign27160_e36449 * locals.var_t1_dn3))), (((locals.var_czbdsw_p1 * locals.var_t1_dn4) * assign27160_e36457) + (assign27160_e36446 * (assign27160_e36449 * locals.var_t1_dn4))), (((locals.var_czbdsw_p1 * locals.var_t1_dn5) * assign27160_e36457) + (assign27160_e36446 * (assign27160_e36449 * locals.var_t1_dn5))), (((locals.var_czbdsw_p1 * locals.var_t1_dn6) * assign27160_e36457) + (assign27160_e36446 * (assign27160_e36449 * locals.var_t1_dn6))), (((locals.var_czbdsw_p1 * locals.var_t1_dn7) * assign27160_e36457) + (assign27160_e36446 * (assign27160_e36449 * locals.var_t1_dn7))), (((locals.var_czbdsw_p1 * locals.var_t1_dn8) * assign27160_e36457) + (assign27160_e36446 * (assign27160_e36449 * locals.var_t1_dn8))), (((locals.var_czbdsw_p1 * locals.var_t1_dn9) * assign27160_e36457) + (assign27160_e36446 * (assign27160_e36449 * locals.var_t1_dn9))), (((locals.var_czbdsw_p1 * locals.var_t1_dn10) * assign27160_e36457) + (assign27160_e36446 * (assign27160_e36449 * locals.var_t1_dn10))), (((locals.var_czbdsw_p1 * locals.var_t1_dn11) * assign27160_e36457) + (assign27160_e36446 * (assign27160_e36449 * locals.var_t1_dn11))), (((locals.var_czbdsw_p1 * locals.var_t1_dn12) * assign27160_e36457) + (assign27160_e36446 * (assign27160_e36449 * locals.var_t1_dn12))), (((locals.var_czbdsw_p1 * locals.var_t1_dn13) * assign27160_e36457) + (assign27160_e36446 * (assign27160_e36449 * locals.var_t1_dn13))), (((locals.var_czbdsw_p1 * locals.var_t1_dn14) * assign27160_e36457) + (assign27160_e36446 * (assign27160_e36449 * locals.var_t1_dn14))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign27160_e36460;
        locals.var_t2_dn0 = assign27160_e36460_d_n0;
        locals.var_t2_dn2 = assign27160_e36460_d_n2;
        locals.var_t2_dn3 = assign27160_e36460_d_n3;
        locals.var_t2_dn4 = assign27160_e36460_d_n4;
        locals.var_t2_dn5 = assign27160_e36460_d_n5;
        locals.var_t2_dn6 = assign27160_e36460_d_n6;
        locals.var_t2_dn7 = assign27160_e36460_d_n7;
        locals.var_t2_dn8 = assign27160_e36460_d_n8;
        locals.var_t2_dn9 = assign27160_e36460_d_n9;
        locals.var_t2_dn10 = assign27160_e36460_d_n10;
        locals.var_t2_dn11 = assign27160_e36460_d_n11;
        locals.var_t2_dn12 = assign27160_e36460_d_n12;
        locals.var_t2_dn13 = assign27160_e36460_d_n13;
        locals.var_t2_dn14 = assign27160_e36460_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign27170_e36475, assign27170_e36475_d_n0, assign27170_e36475_d_n2, assign27170_e36475_d_n3, assign27170_e36475_d_n4, assign27170_e36475_d_n5, assign27170_e36475_d_n6, assign27170_e36475_d_n7, assign27170_e36475_d_n8, assign27170_e36475_d_n9, assign27170_e36475_d_n10, assign27170_e36475_d_n11, assign27170_e36475_d_n12, assign27170_e36475_d_n13, assign27170_e36475_d_n14,) = {
    if (((locals.var_guard670 != 0.0) && (locals.var_guard676 != 0.0)) && (locals.var_guard677 == 0.0)) {
        let assign27170_e36469: f64 = (locals.var_pbswd_t * locals.var_czbdsw);
        let assign27170_e36472: f64 = (locals.var_t2 + locals.var_czbdsw_p2);
        let assign27170_e36473: f64 = (assign27170_e36469 * assign27170_e36472);
        (assign27170_e36473, (((locals.var_pbswd_t * locals.var_czbdsw_dn0) * assign27170_e36472) + (assign27170_e36469 * locals.var_t2_dn0)), (((locals.var_pbswd_t * locals.var_czbdsw_dn2) * assign27170_e36472) + (assign27170_e36469 * locals.var_t2_dn2)), (((locals.var_pbswd_t * locals.var_czbdsw_dn3) * assign27170_e36472) + (assign27170_e36469 * locals.var_t2_dn3)), ((((locals.var_pbswd_t_dn4 * locals.var_czbdsw) + (locals.var_pbswd_t * locals.var_czbdsw_dn4)) * assign27170_e36472) + (assign27170_e36469 * locals.var_t2_dn4)), (((locals.var_pbswd_t * locals.var_czbdsw_dn5) * assign27170_e36472) + (assign27170_e36469 * locals.var_t2_dn5)), (((locals.var_pbswd_t * locals.var_czbdsw_dn6) * assign27170_e36472) + (assign27170_e36469 * locals.var_t2_dn6)), (((locals.var_pbswd_t * locals.var_czbdsw_dn7) * assign27170_e36472) + (assign27170_e36469 * locals.var_t2_dn7)), (((locals.var_pbswd_t * locals.var_czbdsw_dn8) * assign27170_e36472) + (assign27170_e36469 * locals.var_t2_dn8)), (((locals.var_pbswd_t * locals.var_czbdsw_dn9) * assign27170_e36472) + (assign27170_e36469 * locals.var_t2_dn9)), (((locals.var_pbswd_t * locals.var_czbdsw_dn10) * assign27170_e36472) + (assign27170_e36469 * locals.var_t2_dn10)), (((locals.var_pbswd_t * locals.var_czbdsw_dn11) * assign27170_e36472) + (assign27170_e36469 * locals.var_t2_dn11)), (((locals.var_pbswd_t * locals.var_czbdsw_dn12) * assign27170_e36472) + (assign27170_e36469 * locals.var_t2_dn12)), (((locals.var_pbswd_t * locals.var_czbdsw_dn13) * assign27170_e36472) + (assign27170_e36469 * locals.var_t2_dn13)), (((locals.var_pbswd_t * locals.var_czbdsw_dn14) * assign27170_e36472) + (assign27170_e36469 * locals.var_t2_dn14)),)
    } else {
        (locals.var_qbdj2_ext, locals.var_qbdj2_ext_dn0, locals.var_qbdj2_ext_dn2, locals.var_qbdj2_ext_dn3, locals.var_qbdj2_ext_dn4, locals.var_qbdj2_ext_dn5, locals.var_qbdj2_ext_dn6, locals.var_qbdj2_ext_dn7, locals.var_qbdj2_ext_dn8, locals.var_qbdj2_ext_dn9, locals.var_qbdj2_ext_dn10, locals.var_qbdj2_ext_dn11, locals.var_qbdj2_ext_dn12, locals.var_qbdj2_ext_dn13, locals.var_qbdj2_ext_dn14,)
    }
};
        locals.var_qbdj2_ext = assign27170_e36475;
        locals.var_qbdj2_ext_dn0 = assign27170_e36475_d_n0;
        locals.var_qbdj2_ext_dn2 = assign27170_e36475_d_n2;
        locals.var_qbdj2_ext_dn3 = assign27170_e36475_d_n3;
        locals.var_qbdj2_ext_dn4 = assign27170_e36475_d_n4;
        locals.var_qbdj2_ext_dn5 = assign27170_e36475_d_n5;
        locals.var_qbdj2_ext_dn6 = assign27170_e36475_d_n6;
        locals.var_qbdj2_ext_dn7 = assign27170_e36475_d_n7;
        locals.var_qbdj2_ext_dn8 = assign27170_e36475_d_n8;
        locals.var_qbdj2_ext_dn9 = assign27170_e36475_d_n9;
        locals.var_qbdj2_ext_dn10 = assign27170_e36475_d_n10;
        locals.var_qbdj2_ext_dn11 = assign27170_e36475_d_n11;
        locals.var_qbdj2_ext_dn12 = assign27170_e36475_d_n12;
        locals.var_qbdj2_ext_dn13 = assign27170_e36475_d_n13;
        locals.var_qbdj2_ext_dn14 = assign27170_e36475_d_n14;
        locals.var_qbdj2_ext_rv = 0.0;

        let (assign27180_e36482, assign27180_e36482_d_n0, assign27180_e36482_d_n2, assign27180_e36482_d_n3, assign27180_e36482_d_n4, assign27180_e36482_d_n5, assign27180_e36482_d_n6, assign27180_e36482_d_n7, assign27180_e36482_d_n8, assign27180_e36482_d_n9, assign27180_e36482_d_n10, assign27180_e36482_d_n11, assign27180_e36482_d_n12, assign27180_e36482_d_n13, assign27180_e36482_d_n14,) = {
    if ((locals.var_guard670 != 0.0) && (locals.var_guard676 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdj2_ext, locals.var_qbdj2_ext_dn0, locals.var_qbdj2_ext_dn2, locals.var_qbdj2_ext_dn3, locals.var_qbdj2_ext_dn4, locals.var_qbdj2_ext_dn5, locals.var_qbdj2_ext_dn6, locals.var_qbdj2_ext_dn7, locals.var_qbdj2_ext_dn8, locals.var_qbdj2_ext_dn9, locals.var_qbdj2_ext_dn10, locals.var_qbdj2_ext_dn11, locals.var_qbdj2_ext_dn12, locals.var_qbdj2_ext_dn13, locals.var_qbdj2_ext_dn14,)
    }
};
        locals.var_qbdj2_ext = assign27180_e36482;
        locals.var_qbdj2_ext_dn0 = assign27180_e36482_d_n0;
        locals.var_qbdj2_ext_dn2 = assign27180_e36482_d_n2;
        locals.var_qbdj2_ext_dn3 = assign27180_e36482_d_n3;
        locals.var_qbdj2_ext_dn4 = assign27180_e36482_d_n4;
        locals.var_qbdj2_ext_dn5 = assign27180_e36482_d_n5;
        locals.var_qbdj2_ext_dn6 = assign27180_e36482_d_n6;
        locals.var_qbdj2_ext_dn7 = assign27180_e36482_d_n7;
        locals.var_qbdj2_ext_dn8 = assign27180_e36482_d_n8;
        locals.var_qbdj2_ext_dn9 = assign27180_e36482_d_n9;
        locals.var_qbdj2_ext_dn10 = assign27180_e36482_d_n10;
        locals.var_qbdj2_ext_dn11 = assign27180_e36482_d_n11;
        locals.var_qbdj2_ext_dn12 = assign27180_e36482_d_n12;
        locals.var_qbdj2_ext_dn13 = assign27180_e36482_d_n13;
        locals.var_qbdj2_ext_dn14 = assign27180_e36482_d_n14;
        locals.var_qbdj2_ext_rv = 0.0;

        let (assign27190_e36488, assign27190_e36488_d_n0, assign27190_e36488_d_n2, assign27190_e36488_d_n3, assign27190_e36488_d_n4, assign27190_e36488_d_n5, assign27190_e36488_d_n6, assign27190_e36488_d_n7, assign27190_e36488_d_n8, assign27190_e36488_d_n9, assign27190_e36488_d_n10, assign27190_e36488_d_n11, assign27190_e36488_d_n12, assign27190_e36488_d_n13, assign27190_e36488_d_n14,) = {
    if (locals.var_guard670 != 0.0) {
        let assign27190_e36486: f64 = (locals.var_qbdj1_ext + locals.var_qbdj2_ext);
        (assign27190_e36486, (locals.var_qbdj1_ext_dn0 + locals.var_qbdj2_ext_dn0), (locals.var_qbdj1_ext_dn2 + locals.var_qbdj2_ext_dn2), (locals.var_qbdj1_ext_dn3 + locals.var_qbdj2_ext_dn3), (locals.var_qbdj1_ext_dn4 + locals.var_qbdj2_ext_dn4), (locals.var_qbdj1_ext_dn5 + locals.var_qbdj2_ext_dn5), (locals.var_qbdj1_ext_dn6 + locals.var_qbdj2_ext_dn6), (locals.var_qbdj1_ext_dn7 + locals.var_qbdj2_ext_dn7), (locals.var_qbdj1_ext_dn8 + locals.var_qbdj2_ext_dn8), (locals.var_qbdj1_ext_dn9 + locals.var_qbdj2_ext_dn9), (locals.var_qbdj1_ext_dn10 + locals.var_qbdj2_ext_dn10), (locals.var_qbdj1_ext_dn11 + locals.var_qbdj2_ext_dn11), (locals.var_qbdj1_ext_dn12 + locals.var_qbdj2_ext_dn12), (locals.var_qbdj1_ext_dn13 + locals.var_qbdj2_ext_dn13), (locals.var_qbdj1_ext_dn14 + locals.var_qbdj2_ext_dn14),)
    } else {
        (locals.var_qbdj_ext, locals.var_qbdj_ext_dn0, locals.var_qbdj_ext_dn2, locals.var_qbdj_ext_dn3, locals.var_qbdj_ext_dn4, locals.var_qbdj_ext_dn5, locals.var_qbdj_ext_dn6, locals.var_qbdj_ext_dn7, locals.var_qbdj_ext_dn8, locals.var_qbdj_ext_dn9, locals.var_qbdj_ext_dn10, locals.var_qbdj_ext_dn11, locals.var_qbdj_ext_dn12, locals.var_qbdj_ext_dn13, locals.var_qbdj_ext_dn14,)
    }
};
        locals.var_qbdj_ext = assign27190_e36488;
        locals.var_qbdj_ext_dn0 = assign27190_e36488_d_n0;
        locals.var_qbdj_ext_dn2 = assign27190_e36488_d_n2;
        locals.var_qbdj_ext_dn3 = assign27190_e36488_d_n3;
        locals.var_qbdj_ext_dn4 = assign27190_e36488_d_n4;
        locals.var_qbdj_ext_dn5 = assign27190_e36488_d_n5;
        locals.var_qbdj_ext_dn6 = assign27190_e36488_d_n6;
        locals.var_qbdj_ext_dn7 = assign27190_e36488_d_n7;
        locals.var_qbdj_ext_dn8 = assign27190_e36488_d_n8;
        locals.var_qbdj_ext_dn9 = assign27190_e36488_d_n9;
        locals.var_qbdj_ext_dn10 = assign27190_e36488_d_n10;
        locals.var_qbdj_ext_dn11 = assign27190_e36488_d_n11;
        locals.var_qbdj_ext_dn12 = assign27190_e36488_d_n12;
        locals.var_qbdj_ext_dn13 = assign27190_e36488_d_n13;
        locals.var_qbdj_ext_dn14 = assign27190_e36488_d_n14;
        locals.var_qbdj_ext_rv = 0.0;

        let (assign27200_e36493, assign27200_e36493_d_n0, assign27200_e36493_d_n2, assign27200_e36493_d_n3, assign27200_e36493_d_n4, assign27200_e36493_d_n5, assign27200_e36493_d_n6, assign27200_e36493_d_n7, assign27200_e36493_d_n8, assign27200_e36493_d_n9, assign27200_e36493_d_n10, assign27200_e36493_d_n11, assign27200_e36493_d_n12, assign27200_e36493_d_n13, assign27200_e36493_d_n14,) = {
    if (locals.var_guard670 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qbdj_ext, locals.var_qbdj_ext_dn0, locals.var_qbdj_ext_dn2, locals.var_qbdj_ext_dn3, locals.var_qbdj_ext_dn4, locals.var_qbdj_ext_dn5, locals.var_qbdj_ext_dn6, locals.var_qbdj_ext_dn7, locals.var_qbdj_ext_dn8, locals.var_qbdj_ext_dn9, locals.var_qbdj_ext_dn10, locals.var_qbdj_ext_dn11, locals.var_qbdj_ext_dn12, locals.var_qbdj_ext_dn13, locals.var_qbdj_ext_dn14,)
    }
};
        locals.var_qbdj_ext = assign27200_e36493;
        locals.var_qbdj_ext_dn0 = assign27200_e36493_d_n0;
        locals.var_qbdj_ext_dn2 = assign27200_e36493_d_n2;
        locals.var_qbdj_ext_dn3 = assign27200_e36493_d_n3;
        locals.var_qbdj_ext_dn4 = assign27200_e36493_d_n4;
        locals.var_qbdj_ext_dn5 = assign27200_e36493_d_n5;
        locals.var_qbdj_ext_dn6 = assign27200_e36493_d_n6;
        locals.var_qbdj_ext_dn7 = assign27200_e36493_d_n7;
        locals.var_qbdj_ext_dn8 = assign27200_e36493_d_n8;
        locals.var_qbdj_ext_dn9 = assign27200_e36493_d_n9;
        locals.var_qbdj_ext_dn10 = assign27200_e36493_d_n10;
        locals.var_qbdj_ext_dn11 = assign27200_e36493_d_n11;
        locals.var_qbdj_ext_dn12 = assign27200_e36493_d_n12;
        locals.var_qbdj_ext_dn13 = assign27200_e36493_d_n13;
        locals.var_qbdj_ext_dn14 = assign27200_e36493_d_n14;
        locals.var_qbdj_ext_rv = 0.0;

        let assign27210_e36496: f64 = if p.p38 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard680 = assign27210_e36496;
        locals.var_guard680_rv = 0.0;

        let (assign27220_e36504, assign27220_e36504_d_n0, assign27220_e36504_d_n2, assign27220_e36504_d_n3, assign27220_e36504_d_n4, assign27220_e36504_d_n5, assign27220_e36504_d_n6, assign27220_e36504_d_n7, assign27220_e36504_d_n8, assign27220_e36504_d_n9, assign27220_e36504_d_n10, assign27220_e36504_d_n11, assign27220_e36504_d_n12, assign27220_e36504_d_n13, assign27220_e36504_d_n14,) = {
    if (locals.var_guard680 != 0.0) {
        let assign27220_e36500: f64 = (locals.var_ndep_i / 1e23);
        let assign27220_e36502: f64 = (assign27220_e36500).powf(p.p954);
        (assign27220_e36502, if 0.0 == 0.0 && ((p.p954) as f64).is_finite() && ((p.p954) as f64).fract() == 0.0 { if p.p954 == 0.0 { 0.0 } else { (p.p954 * ((assign27220_e36500).powf(p.p954 - 1.0) * (locals.var_ndep_i_dn0 / 1e23))) } } else { (assign27220_e36502 * (p.p954 * ((locals.var_ndep_i_dn0 / 1e23) / assign27220_e36500))) }, if 0.0 == 0.0 && ((p.p954) as f64).is_finite() && ((p.p954) as f64).fract() == 0.0 { if p.p954 == 0.0 { 0.0 } else { (p.p954 * ((assign27220_e36500).powf(p.p954 - 1.0) * (locals.var_ndep_i_dn2 / 1e23))) } } else { (assign27220_e36502 * (p.p954 * ((locals.var_ndep_i_dn2 / 1e23) / assign27220_e36500))) }, if 0.0 == 0.0 && ((p.p954) as f64).is_finite() && ((p.p954) as f64).fract() == 0.0 { if p.p954 == 0.0 { 0.0 } else { (p.p954 * ((assign27220_e36500).powf(p.p954 - 1.0) * (locals.var_ndep_i_dn3 / 1e23))) } } else { (assign27220_e36502 * (p.p954 * ((locals.var_ndep_i_dn3 / 1e23) / assign27220_e36500))) }, if 0.0 == 0.0 && ((p.p954) as f64).is_finite() && ((p.p954) as f64).fract() == 0.0 { if p.p954 == 0.0 { 0.0 } else { (p.p954 * ((assign27220_e36500).powf(p.p954 - 1.0) * (locals.var_ndep_i_dn4 / 1e23))) } } else { (assign27220_e36502 * (p.p954 * ((locals.var_ndep_i_dn4 / 1e23) / assign27220_e36500))) }, if 0.0 == 0.0 && ((p.p954) as f64).is_finite() && ((p.p954) as f64).fract() == 0.0 { if p.p954 == 0.0 { 0.0 } else { (p.p954 * ((assign27220_e36500).powf(p.p954 - 1.0) * (locals.var_ndep_i_dn5 / 1e23))) } } else { (assign27220_e36502 * (p.p954 * ((locals.var_ndep_i_dn5 / 1e23) / assign27220_e36500))) }, if 0.0 == 0.0 && ((p.p954) as f64).is_finite() && ((p.p954) as f64).fract() == 0.0 { if p.p954 == 0.0 { 0.0 } else { (p.p954 * ((assign27220_e36500).powf(p.p954 - 1.0) * (locals.var_ndep_i_dn6 / 1e23))) } } else { (assign27220_e36502 * (p.p954 * ((locals.var_ndep_i_dn6 / 1e23) / assign27220_e36500))) }, if 0.0 == 0.0 && ((p.p954) as f64).is_finite() && ((p.p954) as f64).fract() == 0.0 { if p.p954 == 0.0 { 0.0 } else { (p.p954 * ((assign27220_e36500).powf(p.p954 - 1.0) * (locals.var_ndep_i_dn7 / 1e23))) } } else { (assign27220_e36502 * (p.p954 * ((locals.var_ndep_i_dn7 / 1e23) / assign27220_e36500))) }, if 0.0 == 0.0 && ((p.p954) as f64).is_finite() && ((p.p954) as f64).fract() == 0.0 { if p.p954 == 0.0 { 0.0 } else { (p.p954 * ((assign27220_e36500).powf(p.p954 - 1.0) * (locals.var_ndep_i_dn8 / 1e23))) } } else { (assign27220_e36502 * (p.p954 * ((locals.var_ndep_i_dn8 / 1e23) / assign27220_e36500))) }, if 0.0 == 0.0 && ((p.p954) as f64).is_finite() && ((p.p954) as f64).fract() == 0.0 { if p.p954 == 0.0 { 0.0 } else { (p.p954 * ((assign27220_e36500).powf(p.p954 - 1.0) * (locals.var_ndep_i_dn9 / 1e23))) } } else { (assign27220_e36502 * (p.p954 * ((locals.var_ndep_i_dn9 / 1e23) / assign27220_e36500))) }, if 0.0 == 0.0 && ((p.p954) as f64).is_finite() && ((p.p954) as f64).fract() == 0.0 { if p.p954 == 0.0 { 0.0 } else { (p.p954 * ((assign27220_e36500).powf(p.p954 - 1.0) * (locals.var_ndep_i_dn10 / 1e23))) } } else { (assign27220_e36502 * (p.p954 * ((locals.var_ndep_i_dn10 / 1e23) / assign27220_e36500))) }, if 0.0 == 0.0 && ((p.p954) as f64).is_finite() && ((p.p954) as f64).fract() == 0.0 { if p.p954 == 0.0 { 0.0 } else { (p.p954 * ((assign27220_e36500).powf(p.p954 - 1.0) * (locals.var_ndep_i_dn11 / 1e23))) } } else { (assign27220_e36502 * (p.p954 * ((locals.var_ndep_i_dn11 / 1e23) / assign27220_e36500))) }, if 0.0 == 0.0 && ((p.p954) as f64).is_finite() && ((p.p954) as f64).fract() == 0.0 { if p.p954 == 0.0 { 0.0 } else { (p.p954 * ((assign27220_e36500).powf(p.p954 - 1.0) * (locals.var_ndep_i_dn12 / 1e23))) } } else { (assign27220_e36502 * (p.p954 * ((locals.var_ndep_i_dn12 / 1e23) / assign27220_e36500))) }, if 0.0 == 0.0 && ((p.p954) as f64).is_finite() && ((p.p954) as f64).fract() == 0.0 { if p.p954 == 0.0 { 0.0 } else { (p.p954 * ((assign27220_e36500).powf(p.p954 - 1.0) * (locals.var_ndep_i_dn13 / 1e23))) } } else { (assign27220_e36502 * (p.p954 * ((locals.var_ndep_i_dn13 / 1e23) / assign27220_e36500))) }, if 0.0 == 0.0 && ((p.p954) as f64).is_finite() && ((p.p954) as f64).fract() == 0.0 { if p.p954 == 0.0 { 0.0 } else { (p.p954 * ((assign27220_e36500).powf(p.p954 - 1.0) * (locals.var_ndep_i_dn14 / 1e23))) } } else { (assign27220_e36502 * (p.p954 * ((locals.var_ndep_i_dn14 / 1e23) / assign27220_e36500))) },)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign27220_e36504;
        locals.var_t1_dn0 = assign27220_e36504_d_n0;
        locals.var_t1_dn2 = assign27220_e36504_d_n2;
        locals.var_t1_dn3 = assign27220_e36504_d_n3;
        locals.var_t1_dn4 = assign27220_e36504_d_n4;
        locals.var_t1_dn5 = assign27220_e36504_d_n5;
        locals.var_t1_dn6 = assign27220_e36504_d_n6;
        locals.var_t1_dn7 = assign27220_e36504_d_n7;
        locals.var_t1_dn8 = assign27220_e36504_d_n8;
        locals.var_t1_dn9 = assign27220_e36504_d_n9;
        locals.var_t1_dn10 = assign27220_e36504_d_n10;
        locals.var_t1_dn11 = assign27220_e36504_d_n11;
        locals.var_t1_dn12 = assign27220_e36504_d_n12;
        locals.var_t1_dn13 = assign27220_e36504_d_n13;
        locals.var_t1_dn14 = assign27220_e36504_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign27230_e36512, assign27230_e36512_d_n0, assign27230_e36512_d_n2, assign27230_e36512_d_n3, assign27230_e36512_d_n4, assign27230_e36512_d_n5, assign27230_e36512_d_n6, assign27230_e36512_d_n7, assign27230_e36512_d_n8, assign27230_e36512_d_n9, assign27230_e36512_d_n10, assign27230_e36512_d_n11, assign27230_e36512_d_n12, assign27230_e36512_d_n13, assign27230_e36512_d_n14,) = {
    if (locals.var_guard680 != 0.0) {
        let assign27230_e36508: f64 = (300.0 / locals.var_devtemp);
        let assign27230_e36510: f64 = (assign27230_e36508).powf(p.p955);
        (assign27230_e36510, 0.0, 0.0, 0.0, if 0.0 == 0.0 && ((p.p955) as f64).is_finite() && ((p.p955) as f64).fract() == 0.0 { if p.p955 == 0.0 { 0.0 } else { (p.p955 * ((assign27230_e36508).powf(p.p955 - 1.0) * (-((300.0 * locals.var_devtemp_dn4) / (locals.var_devtemp * locals.var_devtemp))))) } } else { (assign27230_e36510 * (p.p955 * ((-((300.0 * locals.var_devtemp_dn4) / (locals.var_devtemp * locals.var_devtemp))) / assign27230_e36508))) }, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign27230_e36512;
        locals.var_t2_dn0 = assign27230_e36512_d_n0;
        locals.var_t2_dn2 = assign27230_e36512_d_n2;
        locals.var_t2_dn3 = assign27230_e36512_d_n3;
        locals.var_t2_dn4 = assign27230_e36512_d_n4;
        locals.var_t2_dn5 = assign27230_e36512_d_n5;
        locals.var_t2_dn6 = assign27230_e36512_d_n6;
        locals.var_t2_dn7 = assign27230_e36512_d_n7;
        locals.var_t2_dn8 = assign27230_e36512_d_n8;
        locals.var_t2_dn9 = assign27230_e36512_d_n9;
        locals.var_t2_dn10 = assign27230_e36512_d_n10;
        locals.var_t2_dn11 = assign27230_e36512_d_n11;
        locals.var_t2_dn12 = assign27230_e36512_d_n12;
        locals.var_t2_dn13 = assign27230_e36512_d_n13;
        locals.var_t2_dn14 = assign27230_e36512_d_n14;
        locals.var_t2_rv = 0.0;

        let (assign27240_e36522, assign27240_e36522_d_n0, assign27240_e36522_d_n2, assign27240_e36522_d_n3, assign27240_e36522_d_n4, assign27240_e36522_d_n5, assign27240_e36522_d_n6, assign27240_e36522_d_n7, assign27240_e36522_d_n8, assign27240_e36522_d_n9, assign27240_e36522_d_n10, assign27240_e36522_d_n11, assign27240_e36522_d_n12, assign27240_e36522_d_n13, assign27240_e36522_d_n14,) = {
    if (locals.var_guard680 != 0.0) {
        let assign27240_e36516: f64 = (locals.var_devsign * p.p953);
        let assign27240_e36518: f64 = (assign27240_e36516 * (nv11 - nv7));
        let assign27240_e36520: f64 = (assign27240_e36518 / locals.var_vt);
        (assign27240_e36520, 0.0, 0.0, 0.0, (-((assign27240_e36518 * locals.var_vt_dn4) / (locals.var_vt * locals.var_vt))), 0.0, 0.0, ((-assign27240_e36516) / locals.var_vt), 0.0, 0.0, 0.0, (assign27240_e36516 / locals.var_vt), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign27240_e36522;
        locals.var_t3_dn0 = assign27240_e36522_d_n0;
        locals.var_t3_dn2 = assign27240_e36522_d_n2;
        locals.var_t3_dn3 = assign27240_e36522_d_n3;
        locals.var_t3_dn4 = assign27240_e36522_d_n4;
        locals.var_t3_dn5 = assign27240_e36522_d_n5;
        locals.var_t3_dn6 = assign27240_e36522_d_n6;
        locals.var_t3_dn7 = assign27240_e36522_d_n7;
        locals.var_t3_dn8 = assign27240_e36522_d_n8;
        locals.var_t3_dn9 = assign27240_e36522_d_n9;
        locals.var_t3_dn10 = assign27240_e36522_d_n10;
        locals.var_t3_dn11 = assign27240_e36522_d_n11;
        locals.var_t3_dn12 = assign27240_e36522_d_n12;
        locals.var_t3_dn13 = assign27240_e36522_d_n13;
        locals.var_t3_dn14 = assign27240_e36522_d_n14;
        locals.var_t3_rv = 0.0;

        let assign27300_e36597: f64 = (2.0 * locals.var_vsat_a);
        let assign27300_e36599: f64 = (assign27300_e36597 / locals.var_ueff);
        locals.var_esatnoi = assign27300_e36599;
        locals.var_esatnoi_dn0 = ((((2.0 * locals.var_vsat_a_dn0) * locals.var_ueff) - (assign27300_e36597 * locals.var_ueff_dn0)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn2 = ((((2.0 * locals.var_vsat_a_dn2) * locals.var_ueff) - (assign27300_e36597 * locals.var_ueff_dn2)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn3 = ((((2.0 * locals.var_vsat_a_dn3) * locals.var_ueff) - (assign27300_e36597 * locals.var_ueff_dn3)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn4 = ((((2.0 * locals.var_vsat_a_dn4) * locals.var_ueff) - (assign27300_e36597 * locals.var_ueff_dn4)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn5 = ((((2.0 * locals.var_vsat_a_dn5) * locals.var_ueff) - (assign27300_e36597 * locals.var_ueff_dn5)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn6 = ((((2.0 * locals.var_vsat_a_dn6) * locals.var_ueff) - (assign27300_e36597 * locals.var_ueff_dn6)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn7 = ((((2.0 * locals.var_vsat_a_dn7) * locals.var_ueff) - (assign27300_e36597 * locals.var_ueff_dn7)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn8 = ((((2.0 * locals.var_vsat_a_dn8) * locals.var_ueff) - (assign27300_e36597 * locals.var_ueff_dn8)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn9 = ((((2.0 * locals.var_vsat_a_dn9) * locals.var_ueff) - (assign27300_e36597 * locals.var_ueff_dn9)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn10 = ((((2.0 * locals.var_vsat_a_dn10) * locals.var_ueff) - (assign27300_e36597 * locals.var_ueff_dn10)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn11 = ((((2.0 * locals.var_vsat_a_dn11) * locals.var_ueff) - (assign27300_e36597 * locals.var_ueff_dn11)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn12 = ((((2.0 * locals.var_vsat_a_dn12) * locals.var_ueff) - (assign27300_e36597 * locals.var_ueff_dn12)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn13 = ((((2.0 * locals.var_vsat_a_dn13) * locals.var_ueff) - (assign27300_e36597 * locals.var_ueff_dn13)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_dn14 = ((((2.0 * locals.var_vsat_a_dn14) * locals.var_ueff) - (assign27300_e36597 * locals.var_ueff_dn14)) / (locals.var_ueff * locals.var_ueff));
        locals.var_esatnoi_rv = 0.0;

        let assign27310_e36602: f64 = if p.p784 <= 0.0 { 1.0 } else { 0.0 };
        locals.var_guard681 = assign27310_e36602;
        locals.var_guard681_rv = 0.0;

        let (assign27320_e36606, assign27320_e36606_d_n0, assign27320_e36606_d_n2, assign27320_e36606_d_n3, assign27320_e36606_d_n4, assign27320_e36606_d_n5, assign27320_e36606_d_n6, assign27320_e36606_d_n7, assign27320_e36606_d_n8, assign27320_e36606_d_n9, assign27320_e36606_d_n10, assign27320_e36606_d_n11, assign27320_e36606_d_n12, assign27320_e36606_d_n13, assign27320_e36606_d_n14,) = {
    if (locals.var_guard681 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delclm, locals.var_delclm_dn0, locals.var_delclm_dn2, locals.var_delclm_dn3, locals.var_delclm_dn4, locals.var_delclm_dn5, locals.var_delclm_dn6, locals.var_delclm_dn7, locals.var_delclm_dn8, locals.var_delclm_dn9, locals.var_delclm_dn10, locals.var_delclm_dn11, locals.var_delclm_dn12, locals.var_delclm_dn13, locals.var_delclm_dn14,)
    }
};
        locals.var_delclm = assign27320_e36606;
        locals.var_delclm_dn0 = assign27320_e36606_d_n0;
        locals.var_delclm_dn2 = assign27320_e36606_d_n2;
        locals.var_delclm_dn3 = assign27320_e36606_d_n3;
        locals.var_delclm_dn4 = assign27320_e36606_d_n4;
        locals.var_delclm_dn5 = assign27320_e36606_d_n5;
        locals.var_delclm_dn6 = assign27320_e36606_d_n6;
        locals.var_delclm_dn7 = assign27320_e36606_d_n7;
        locals.var_delclm_dn8 = assign27320_e36606_d_n8;
        locals.var_delclm_dn9 = assign27320_e36606_d_n9;
        locals.var_delclm_dn10 = assign27320_e36606_d_n10;
        locals.var_delclm_dn11 = assign27320_e36606_d_n11;
        locals.var_delclm_dn12 = assign27320_e36606_d_n12;
        locals.var_delclm_dn13 = assign27320_e36606_d_n13;
        locals.var_delclm_dn14 = assign27320_e36606_d_n14;
        locals.var_delclm_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_82(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign27330_e36617, assign27330_e36617_d_n0, assign27330_e36617_d_n2, assign27330_e36617_d_n3, assign27330_e36617_d_n4, assign27330_e36617_d_n5, assign27330_e36617_d_n6, assign27330_e36617_d_n7, assign27330_e36617_d_n8, assign27330_e36617_d_n9, assign27330_e36617_d_n10, assign27330_e36617_d_n11, assign27330_e36617_d_n12, assign27330_e36617_d_n13, assign27330_e36617_d_n14,) = {
    if (locals.var_guard681 == 0.0) {
        let assign27330_e36611: f64 = (locals.var_diffvds / locals.var_litl);
        let assign27330_e36613: f64 = (assign27330_e36611 + p.p784);
        let assign27330_e36615: f64 = (assign27330_e36613 / locals.var_esatnoi);
        (assign27330_e36615, ((((locals.var_diffvds_dn0 / locals.var_litl) * locals.var_esatnoi) - (assign27330_e36613 * locals.var_esatnoi_dn0)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn2 / locals.var_litl) * locals.var_esatnoi) - (assign27330_e36613 * locals.var_esatnoi_dn2)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn3 / locals.var_litl) * locals.var_esatnoi) - (assign27330_e36613 * locals.var_esatnoi_dn3)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn4 / locals.var_litl) * locals.var_esatnoi) - (assign27330_e36613 * locals.var_esatnoi_dn4)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn5 / locals.var_litl) * locals.var_esatnoi) - (assign27330_e36613 * locals.var_esatnoi_dn5)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn6 / locals.var_litl) * locals.var_esatnoi) - (assign27330_e36613 * locals.var_esatnoi_dn6)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn7 / locals.var_litl) * locals.var_esatnoi) - (assign27330_e36613 * locals.var_esatnoi_dn7)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn8 / locals.var_litl) * locals.var_esatnoi) - (assign27330_e36613 * locals.var_esatnoi_dn8)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn9 / locals.var_litl) * locals.var_esatnoi) - (assign27330_e36613 * locals.var_esatnoi_dn9)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn10 / locals.var_litl) * locals.var_esatnoi) - (assign27330_e36613 * locals.var_esatnoi_dn10)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn11 / locals.var_litl) * locals.var_esatnoi) - (assign27330_e36613 * locals.var_esatnoi_dn11)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn12 / locals.var_litl) * locals.var_esatnoi) - (assign27330_e36613 * locals.var_esatnoi_dn12)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn13 / locals.var_litl) * locals.var_esatnoi) - (assign27330_e36613 * locals.var_esatnoi_dn13)) / (locals.var_esatnoi * locals.var_esatnoi)), ((((locals.var_diffvds_dn14 / locals.var_litl) * locals.var_esatnoi) - (assign27330_e36613 * locals.var_esatnoi_dn14)) / (locals.var_esatnoi * locals.var_esatnoi)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn3, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn12, locals.var_t0_dn13, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign27330_e36617;
        locals.var_t0_dn0 = assign27330_e36617_d_n0;
        locals.var_t0_dn2 = assign27330_e36617_d_n2;
        locals.var_t0_dn3 = assign27330_e36617_d_n3;
        locals.var_t0_dn4 = assign27330_e36617_d_n4;
        locals.var_t0_dn5 = assign27330_e36617_d_n5;
        locals.var_t0_dn6 = assign27330_e36617_d_n6;
        locals.var_t0_dn7 = assign27330_e36617_d_n7;
        locals.var_t0_dn8 = assign27330_e36617_d_n8;
        locals.var_t0_dn9 = assign27330_e36617_d_n9;
        locals.var_t0_dn10 = assign27330_e36617_d_n10;
        locals.var_t0_dn11 = assign27330_e36617_d_n11;
        locals.var_t0_dn12 = assign27330_e36617_d_n12;
        locals.var_t0_dn13 = assign27330_e36617_d_n13;
        locals.var_t0_dn14 = assign27330_e36617_d_n14;
        locals.var_t0_rv = 0.0;

        let (assign27340_e36627, assign27340_e36627_d_n0, assign27340_e36627_d_n2, assign27340_e36627_d_n3, assign27340_e36627_d_n4, assign27340_e36627_d_n5, assign27340_e36627_d_n6, assign27340_e36627_d_n7, assign27340_e36627_d_n8, assign27340_e36627_d_n9, assign27340_e36627_d_n10, assign27340_e36627_d_n11, assign27340_e36627_d_n12, assign27340_e36627_d_n13, assign27340_e36627_d_n14,) = {
    if (locals.var_guard681 == 0.0) {
        let assign27340_e36623: f64 = (locals.var_t0).max(1e-38);
        let assign27340_e36624: f64 = (assign27340_e36623).ln();
        let assign27340_e36625: f64 = (locals.var_litl * assign27340_e36624);
        (assign27340_e36625, (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn0 } else { 0.0 } / assign27340_e36623)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn2 } else { 0.0 } / assign27340_e36623)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn3 } else { 0.0 } / assign27340_e36623)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn4 } else { 0.0 } / assign27340_e36623)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn5 } else { 0.0 } / assign27340_e36623)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn6 } else { 0.0 } / assign27340_e36623)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn7 } else { 0.0 } / assign27340_e36623)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn8 } else { 0.0 } / assign27340_e36623)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn9 } else { 0.0 } / assign27340_e36623)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn10 } else { 0.0 } / assign27340_e36623)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn11 } else { 0.0 } / assign27340_e36623)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn12 } else { 0.0 } / assign27340_e36623)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn13 } else { 0.0 } / assign27340_e36623)), (locals.var_litl * (if locals.var_t0 >= 1e-38 { locals.var_t0_dn14 } else { 0.0 } / assign27340_e36623)),)
    } else {
        (locals.var_delclm, locals.var_delclm_dn0, locals.var_delclm_dn2, locals.var_delclm_dn3, locals.var_delclm_dn4, locals.var_delclm_dn5, locals.var_delclm_dn6, locals.var_delclm_dn7, locals.var_delclm_dn8, locals.var_delclm_dn9, locals.var_delclm_dn10, locals.var_delclm_dn11, locals.var_delclm_dn12, locals.var_delclm_dn13, locals.var_delclm_dn14,)
    }
};
        locals.var_delclm = assign27340_e36627;
        locals.var_delclm_dn0 = assign27340_e36627_d_n0;
        locals.var_delclm_dn2 = assign27340_e36627_d_n2;
        locals.var_delclm_dn3 = assign27340_e36627_d_n3;
        locals.var_delclm_dn4 = assign27340_e36627_d_n4;
        locals.var_delclm_dn5 = assign27340_e36627_d_n5;
        locals.var_delclm_dn6 = assign27340_e36627_d_n6;
        locals.var_delclm_dn7 = assign27340_e36627_d_n7;
        locals.var_delclm_dn8 = assign27340_e36627_d_n8;
        locals.var_delclm_dn9 = assign27340_e36627_d_n9;
        locals.var_delclm_dn10 = assign27340_e36627_d_n10;
        locals.var_delclm_dn11 = assign27340_e36627_d_n11;
        locals.var_delclm_dn12 = assign27340_e36627_d_n12;
        locals.var_delclm_dn13 = assign27340_e36627_d_n13;
        locals.var_delclm_dn14 = assign27340_e36627_d_n14;
        locals.var_delclm_rv = 0.0;

        let assign27350_e36630: f64 = if locals.var_delclm < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard682 = assign27350_e36630;
        locals.var_guard682_rv = 0.0;

        let (assign27360_e36637, assign27360_e36637_d_n0, assign27360_e36637_d_n2, assign27360_e36637_d_n3, assign27360_e36637_d_n4, assign27360_e36637_d_n5, assign27360_e36637_d_n6, assign27360_e36637_d_n7, assign27360_e36637_d_n8, assign27360_e36637_d_n9, assign27360_e36637_d_n10, assign27360_e36637_d_n11, assign27360_e36637_d_n12, assign27360_e36637_d_n13, assign27360_e36637_d_n14,) = {
    if ((locals.var_guard681 == 0.0) && (locals.var_guard682 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delclm, locals.var_delclm_dn0, locals.var_delclm_dn2, locals.var_delclm_dn3, locals.var_delclm_dn4, locals.var_delclm_dn5, locals.var_delclm_dn6, locals.var_delclm_dn7, locals.var_delclm_dn8, locals.var_delclm_dn9, locals.var_delclm_dn10, locals.var_delclm_dn11, locals.var_delclm_dn12, locals.var_delclm_dn13, locals.var_delclm_dn14,)
    }
};
        locals.var_delclm = assign27360_e36637;
        locals.var_delclm_dn0 = assign27360_e36637_d_n0;
        locals.var_delclm_dn2 = assign27360_e36637_d_n2;
        locals.var_delclm_dn3 = assign27360_e36637_d_n3;
        locals.var_delclm_dn4 = assign27360_e36637_d_n4;
        locals.var_delclm_dn5 = assign27360_e36637_d_n5;
        locals.var_delclm_dn6 = assign27360_e36637_d_n6;
        locals.var_delclm_dn7 = assign27360_e36637_d_n7;
        locals.var_delclm_dn8 = assign27360_e36637_d_n8;
        locals.var_delclm_dn9 = assign27360_e36637_d_n9;
        locals.var_delclm_dn10 = assign27360_e36637_d_n10;
        locals.var_delclm_dn11 = assign27360_e36637_d_n11;
        locals.var_delclm_dn12 = assign27360_e36637_d_n12;
        locals.var_delclm_dn13 = assign27360_e36637_d_n13;
        locals.var_delclm_dn14 = assign27360_e36637_d_n14;
        locals.var_delclm_rv = 0.0;

        let assign27370_e36640: f64 = (locals.var_vt / 1.60219e-19);
        let assign27370_e36643: f64 = (locals.var_cox + locals.var_cdep);
        let assign27370_e36645: f64 = (assign27370_e36643 + locals.var_cit_i);
        let assign27370_e36646: f64 = (assign27370_e36640 * assign27370_e36645);
        locals.var_nstar = assign27370_e36646;
        locals.var_nstar_dn0 = (assign27370_e36640 * locals.var_cdep_dn0);
        locals.var_nstar_dn2 = (assign27370_e36640 * locals.var_cdep_dn2);
        locals.var_nstar_dn3 = (assign27370_e36640 * locals.var_cdep_dn3);
        locals.var_nstar_dn4 = (((locals.var_vt_dn4 / 1.60219e-19) * assign27370_e36645) + (assign27370_e36640 * locals.var_cdep_dn4));
        locals.var_nstar_dn5 = (assign27370_e36640 * locals.var_cdep_dn5);
        locals.var_nstar_dn6 = (assign27370_e36640 * locals.var_cdep_dn6);
        locals.var_nstar_dn7 = (assign27370_e36640 * locals.var_cdep_dn7);
        locals.var_nstar_dn8 = (assign27370_e36640 * locals.var_cdep_dn8);
        locals.var_nstar_dn9 = (assign27370_e36640 * locals.var_cdep_dn9);
        locals.var_nstar_dn10 = (assign27370_e36640 * locals.var_cdep_dn10);
        locals.var_nstar_dn11 = (assign27370_e36640 * locals.var_cdep_dn11);
        locals.var_nstar_dn12 = (assign27370_e36640 * locals.var_cdep_dn12);
        locals.var_nstar_dn13 = (assign27370_e36640 * locals.var_cdep_dn13);
        locals.var_nstar_dn14 = (assign27370_e36640 * locals.var_cdep_dn14);
        locals.var_nstar_rv = 0.0;

        let assign27380_e36649: f64 = (2.0 * locals.var_nq);
        let assign27380_e36651: f64 = (assign27380_e36649 * locals.var_cox);
        let assign27380_e36653: f64 = (assign27380_e36651 * locals.var_vt);
        let assign27380_e36655: f64 = (assign27380_e36653 * locals.var_qdeff);
        let assign27380_e36657: f64 = (assign27380_e36655 * locals.var_mnud1);
        let assign27380_e36659: f64 = (assign27380_e36657 * locals.var_mnud);
        let assign27380_e36661: f64 = (assign27380_e36659 / 1.60219e-19);
        locals.var_nl = assign27380_e36661;
        locals.var_nl_dn0 = ((((((((((2.0 * locals.var_nq_dn0) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign27380_e36653 * locals.var_qdeff_dn0)) * locals.var_mnud1) + (assign27380_e36655 * locals.var_mnud1_dn0)) * locals.var_mnud) + (assign27380_e36657 * locals.var_mnud_dn0)) / 1.60219e-19);
        locals.var_nl_dn2 = ((((((((((2.0 * locals.var_nq_dn2) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign27380_e36653 * locals.var_qdeff_dn2)) * locals.var_mnud1) + (assign27380_e36655 * locals.var_mnud1_dn2)) * locals.var_mnud) + (assign27380_e36657 * locals.var_mnud_dn2)) / 1.60219e-19);
        locals.var_nl_dn3 = ((((((((((2.0 * locals.var_nq_dn3) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign27380_e36653 * locals.var_qdeff_dn3)) * locals.var_mnud1) + (assign27380_e36655 * locals.var_mnud1_dn3)) * locals.var_mnud) + (assign27380_e36657 * locals.var_mnud_dn3)) / 1.60219e-19);
        locals.var_nl_dn4 = (((((((((((2.0 * locals.var_nq_dn4) * locals.var_cox) * locals.var_vt) + (assign27380_e36651 * locals.var_vt_dn4)) * locals.var_qdeff) + (assign27380_e36653 * locals.var_qdeff_dn4)) * locals.var_mnud1) + (assign27380_e36655 * locals.var_mnud1_dn4)) * locals.var_mnud) + (assign27380_e36657 * locals.var_mnud_dn4)) / 1.60219e-19);
        locals.var_nl_dn5 = ((((((((((2.0 * locals.var_nq_dn5) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign27380_e36653 * locals.var_qdeff_dn5)) * locals.var_mnud1) + (assign27380_e36655 * locals.var_mnud1_dn5)) * locals.var_mnud) + (assign27380_e36657 * locals.var_mnud_dn5)) / 1.60219e-19);
        locals.var_nl_dn6 = ((((((((((2.0 * locals.var_nq_dn6) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign27380_e36653 * locals.var_qdeff_dn6)) * locals.var_mnud1) + (assign27380_e36655 * locals.var_mnud1_dn6)) * locals.var_mnud) + (assign27380_e36657 * locals.var_mnud_dn6)) / 1.60219e-19);
        locals.var_nl_dn7 = ((((((((((2.0 * locals.var_nq_dn7) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign27380_e36653 * locals.var_qdeff_dn7)) * locals.var_mnud1) + (assign27380_e36655 * locals.var_mnud1_dn7)) * locals.var_mnud) + (assign27380_e36657 * locals.var_mnud_dn7)) / 1.60219e-19);
        locals.var_nl_dn8 = ((((((((((2.0 * locals.var_nq_dn8) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign27380_e36653 * locals.var_qdeff_dn8)) * locals.var_mnud1) + (assign27380_e36655 * locals.var_mnud1_dn8)) * locals.var_mnud) + (assign27380_e36657 * locals.var_mnud_dn8)) / 1.60219e-19);
        locals.var_nl_dn9 = ((((((((((2.0 * locals.var_nq_dn9) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign27380_e36653 * locals.var_qdeff_dn9)) * locals.var_mnud1) + (assign27380_e36655 * locals.var_mnud1_dn9)) * locals.var_mnud) + (assign27380_e36657 * locals.var_mnud_dn9)) / 1.60219e-19);
        locals.var_nl_dn10 = ((((((((((2.0 * locals.var_nq_dn10) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign27380_e36653 * locals.var_qdeff_dn10)) * locals.var_mnud1) + (assign27380_e36655 * locals.var_mnud1_dn10)) * locals.var_mnud) + (assign27380_e36657 * locals.var_mnud_dn10)) / 1.60219e-19);
        locals.var_nl_dn11 = ((((((((((2.0 * locals.var_nq_dn11) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign27380_e36653 * locals.var_qdeff_dn11)) * locals.var_mnud1) + (assign27380_e36655 * locals.var_mnud1_dn11)) * locals.var_mnud) + (assign27380_e36657 * locals.var_mnud_dn11)) / 1.60219e-19);
        locals.var_nl_dn12 = ((((((((((2.0 * locals.var_nq_dn12) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign27380_e36653 * locals.var_qdeff_dn12)) * locals.var_mnud1) + (assign27380_e36655 * locals.var_mnud1_dn12)) * locals.var_mnud) + (assign27380_e36657 * locals.var_mnud_dn12)) / 1.60219e-19);
        locals.var_nl_dn13 = ((((((((((2.0 * locals.var_nq_dn13) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign27380_e36653 * locals.var_qdeff_dn13)) * locals.var_mnud1) + (assign27380_e36655 * locals.var_mnud1_dn13)) * locals.var_mnud) + (assign27380_e36657 * locals.var_mnud_dn13)) / 1.60219e-19);
        locals.var_nl_dn14 = ((((((((((2.0 * locals.var_nq_dn14) * locals.var_cox) * locals.var_vt) * locals.var_qdeff) + (assign27380_e36653 * locals.var_qdeff_dn14)) * locals.var_mnud1) + (assign27380_e36655 * locals.var_mnud1_dn14)) * locals.var_mnud) + (assign27380_e36657 * locals.var_mnud_dn14)) / 1.60219e-19);
        locals.var_nl_rv = 0.0;

        let assign27390_e36664: f64 = (1.60219e-19 * 1.60219e-19);
        let assign27390_e36666: f64 = (assign27390_e36664 * 1.60219e-19);
        let assign27390_e36668: f64 = (assign27390_e36666 * locals.var_vt);
        let assign27390_e36670: f64 = (locals.var_ids).abs();
        let assign27390_e36671: f64 = (assign27390_e36668 * assign27390_e36670);
        let assign27390_e36673: f64 = (assign27390_e36671 * locals.var_ueff);
        locals.var_t0a = assign27390_e36673;
        locals.var_t0a_dn0 = (((assign27390_e36668 * if locals.var_ids >= 0.0 { locals.var_ids_dn0 } else { (-locals.var_ids_dn0) }) * locals.var_ueff) + (assign27390_e36671 * locals.var_ueff_dn0));
        locals.var_t0a_dn2 = (((assign27390_e36668 * if locals.var_ids >= 0.0 { locals.var_ids_dn2 } else { (-locals.var_ids_dn2) }) * locals.var_ueff) + (assign27390_e36671 * locals.var_ueff_dn2));
        locals.var_t0a_dn3 = (((assign27390_e36668 * if locals.var_ids >= 0.0 { locals.var_ids_dn3 } else { (-locals.var_ids_dn3) }) * locals.var_ueff) + (assign27390_e36671 * locals.var_ueff_dn3));
        locals.var_t0a_dn4 = (((((assign27390_e36666 * locals.var_vt_dn4) * assign27390_e36670) + (assign27390_e36668 * if locals.var_ids >= 0.0 { locals.var_ids_dn4 } else { (-locals.var_ids_dn4) })) * locals.var_ueff) + (assign27390_e36671 * locals.var_ueff_dn4));
        locals.var_t0a_dn5 = (((assign27390_e36668 * if locals.var_ids >= 0.0 { locals.var_ids_dn5 } else { (-locals.var_ids_dn5) }) * locals.var_ueff) + (assign27390_e36671 * locals.var_ueff_dn5));
        locals.var_t0a_dn6 = (((assign27390_e36668 * if locals.var_ids >= 0.0 { locals.var_ids_dn6 } else { (-locals.var_ids_dn6) }) * locals.var_ueff) + (assign27390_e36671 * locals.var_ueff_dn6));
        locals.var_t0a_dn7 = (((assign27390_e36668 * if locals.var_ids >= 0.0 { locals.var_ids_dn7 } else { (-locals.var_ids_dn7) }) * locals.var_ueff) + (assign27390_e36671 * locals.var_ueff_dn7));
        locals.var_t0a_dn8 = (((assign27390_e36668 * if locals.var_ids >= 0.0 { locals.var_ids_dn8 } else { (-locals.var_ids_dn8) }) * locals.var_ueff) + (assign27390_e36671 * locals.var_ueff_dn8));
        locals.var_t0a_dn9 = (((assign27390_e36668 * if locals.var_ids >= 0.0 { locals.var_ids_dn9 } else { (-locals.var_ids_dn9) }) * locals.var_ueff) + (assign27390_e36671 * locals.var_ueff_dn9));
        locals.var_t0a_dn10 = (((assign27390_e36668 * if locals.var_ids >= 0.0 { locals.var_ids_dn10 } else { (-locals.var_ids_dn10) }) * locals.var_ueff) + (assign27390_e36671 * locals.var_ueff_dn10));
        locals.var_t0a_dn11 = (((assign27390_e36668 * if locals.var_ids >= 0.0 { locals.var_ids_dn11 } else { (-locals.var_ids_dn11) }) * locals.var_ueff) + (assign27390_e36671 * locals.var_ueff_dn11));
        locals.var_t0a_dn12 = (((assign27390_e36668 * if locals.var_ids >= 0.0 { locals.var_ids_dn12 } else { (-locals.var_ids_dn12) }) * locals.var_ueff) + (assign27390_e36671 * locals.var_ueff_dn12));
        locals.var_t0a_dn13 = (((assign27390_e36668 * if locals.var_ids >= 0.0 { locals.var_ids_dn13 } else { (-locals.var_ids_dn13) }) * locals.var_ueff) + (assign27390_e36671 * locals.var_ueff_dn13));
        locals.var_t0a_dn14 = (((assign27390_e36668 * if locals.var_ids >= 0.0 { locals.var_ids_dn14 } else { (-locals.var_ids_dn14) }) * locals.var_ueff) + (assign27390_e36671 * locals.var_ueff_dn14));
        locals.var_t0a_rv = 0.0;

        let assign27400_e36676: f64 = (1.60219e-19 * locals.var_vt);
        let assign27400_e36678: f64 = (assign27400_e36676 * locals.var_ids);
        let assign27400_e36680: f64 = (assign27400_e36678 * locals.var_ids);
        locals.var_t0b = assign27400_e36680;
        locals.var_t0b_dn0 = (((assign27400_e36676 * locals.var_ids_dn0) * locals.var_ids) + (assign27400_e36678 * locals.var_ids_dn0));
        locals.var_t0b_dn2 = (((assign27400_e36676 * locals.var_ids_dn2) * locals.var_ids) + (assign27400_e36678 * locals.var_ids_dn2));
        locals.var_t0b_dn3 = (((assign27400_e36676 * locals.var_ids_dn3) * locals.var_ids) + (assign27400_e36678 * locals.var_ids_dn3));
        locals.var_t0b_dn4 = (((((1.60219e-19 * locals.var_vt_dn4) * locals.var_ids) + (assign27400_e36676 * locals.var_ids_dn4)) * locals.var_ids) + (assign27400_e36678 * locals.var_ids_dn4));
        locals.var_t0b_dn5 = (((assign27400_e36676 * locals.var_ids_dn5) * locals.var_ids) + (assign27400_e36678 * locals.var_ids_dn5));
        locals.var_t0b_dn6 = (((assign27400_e36676 * locals.var_ids_dn6) * locals.var_ids) + (assign27400_e36678 * locals.var_ids_dn6));
        locals.var_t0b_dn7 = (((assign27400_e36676 * locals.var_ids_dn7) * locals.var_ids) + (assign27400_e36678 * locals.var_ids_dn7));
        locals.var_t0b_dn8 = (((assign27400_e36676 * locals.var_ids_dn8) * locals.var_ids) + (assign27400_e36678 * locals.var_ids_dn8));
        locals.var_t0b_dn9 = (((assign27400_e36676 * locals.var_ids_dn9) * locals.var_ids) + (assign27400_e36678 * locals.var_ids_dn9));
        locals.var_t0b_dn10 = (((assign27400_e36676 * locals.var_ids_dn10) * locals.var_ids) + (assign27400_e36678 * locals.var_ids_dn10));
        locals.var_t0b_dn11 = (((assign27400_e36676 * locals.var_ids_dn11) * locals.var_ids) + (assign27400_e36678 * locals.var_ids_dn11));
        locals.var_t0b_dn12 = (((assign27400_e36676 * locals.var_ids_dn12) * locals.var_ids) + (assign27400_e36678 * locals.var_ids_dn12));
        locals.var_t0b_dn13 = (((assign27400_e36676 * locals.var_ids_dn13) * locals.var_ids) + (assign27400_e36678 * locals.var_ids_dn13));
        locals.var_t0b_dn14 = (((assign27400_e36676 * locals.var_ids_dn14) * locals.var_ids) + (assign27400_e36678 * locals.var_ids_dn14));
        locals.var_t0b_rv = 0.0;

        let assign27410_e36684: f64 = (p.p799 * locals.var_nl);
        let assign27410_e36685: f64 = (p.p785 + assign27410_e36684);
        let assign27410_e36688: f64 = (p.p800 * locals.var_nl);
        let assign27410_e36690: f64 = (assign27410_e36688 * locals.var_nl);
        let assign27410_e36691: f64 = (assign27410_e36685 + assign27410_e36690);
        locals.var_t0c = assign27410_e36691;
        locals.var_t0c_dn0 = ((p.p799 * locals.var_nl_dn0) + (((p.p800 * locals.var_nl_dn0) * locals.var_nl) + (assign27410_e36688 * locals.var_nl_dn0)));
        locals.var_t0c_dn2 = ((p.p799 * locals.var_nl_dn2) + (((p.p800 * locals.var_nl_dn2) * locals.var_nl) + (assign27410_e36688 * locals.var_nl_dn2)));
        locals.var_t0c_dn3 = ((p.p799 * locals.var_nl_dn3) + (((p.p800 * locals.var_nl_dn3) * locals.var_nl) + (assign27410_e36688 * locals.var_nl_dn3)));
        locals.var_t0c_dn4 = ((p.p799 * locals.var_nl_dn4) + (((p.p800 * locals.var_nl_dn4) * locals.var_nl) + (assign27410_e36688 * locals.var_nl_dn4)));
        locals.var_t0c_dn5 = ((p.p799 * locals.var_nl_dn5) + (((p.p800 * locals.var_nl_dn5) * locals.var_nl) + (assign27410_e36688 * locals.var_nl_dn5)));
        locals.var_t0c_dn6 = ((p.p799 * locals.var_nl_dn6) + (((p.p800 * locals.var_nl_dn6) * locals.var_nl) + (assign27410_e36688 * locals.var_nl_dn6)));
        locals.var_t0c_dn7 = ((p.p799 * locals.var_nl_dn7) + (((p.p800 * locals.var_nl_dn7) * locals.var_nl) + (assign27410_e36688 * locals.var_nl_dn7)));
        locals.var_t0c_dn8 = ((p.p799 * locals.var_nl_dn8) + (((p.p800 * locals.var_nl_dn8) * locals.var_nl) + (assign27410_e36688 * locals.var_nl_dn8)));
        locals.var_t0c_dn9 = ((p.p799 * locals.var_nl_dn9) + (((p.p800 * locals.var_nl_dn9) * locals.var_nl) + (assign27410_e36688 * locals.var_nl_dn9)));
        locals.var_t0c_dn10 = ((p.p799 * locals.var_nl_dn10) + (((p.p800 * locals.var_nl_dn10) * locals.var_nl) + (assign27410_e36688 * locals.var_nl_dn10)));
        locals.var_t0c_dn11 = ((p.p799 * locals.var_nl_dn11) + (((p.p800 * locals.var_nl_dn11) * locals.var_nl) + (assign27410_e36688 * locals.var_nl_dn11)));
        locals.var_t0c_dn12 = ((p.p799 * locals.var_nl_dn12) + (((p.p800 * locals.var_nl_dn12) * locals.var_nl) + (assign27410_e36688 * locals.var_nl_dn12)));
        locals.var_t0c_dn13 = ((p.p799 * locals.var_nl_dn13) + (((p.p800 * locals.var_nl_dn13) * locals.var_nl) + (assign27410_e36688 * locals.var_nl_dn13)));
        locals.var_t0c_dn14 = ((p.p799 * locals.var_nl_dn14) + (((p.p800 * locals.var_nl_dn14) * locals.var_nl) + (assign27410_e36688 * locals.var_nl_dn14)));
        locals.var_t0c_rv = 0.0;

        let assign27420_e36694: f64 = (locals.var_nl + locals.var_nstar);
        let assign27420_e36697: f64 = (locals.var_nl + locals.var_nstar);
        let assign27420_e36698: f64 = (assign27420_e36694 * assign27420_e36697);
        locals.var_t0d = assign27420_e36698;
        locals.var_t0d_dn0 = (((locals.var_nl_dn0 + locals.var_nstar_dn0) * assign27420_e36697) + (assign27420_e36694 * (locals.var_nl_dn0 + locals.var_nstar_dn0)));
        locals.var_t0d_dn2 = (((locals.var_nl_dn2 + locals.var_nstar_dn2) * assign27420_e36697) + (assign27420_e36694 * (locals.var_nl_dn2 + locals.var_nstar_dn2)));
        locals.var_t0d_dn3 = (((locals.var_nl_dn3 + locals.var_nstar_dn3) * assign27420_e36697) + (assign27420_e36694 * (locals.var_nl_dn3 + locals.var_nstar_dn3)));
        locals.var_t0d_dn4 = (((locals.var_nl_dn4 + locals.var_nstar_dn4) * assign27420_e36697) + (assign27420_e36694 * (locals.var_nl_dn4 + locals.var_nstar_dn4)));
        locals.var_t0d_dn5 = (((locals.var_nl_dn5 + locals.var_nstar_dn5) * assign27420_e36697) + (assign27420_e36694 * (locals.var_nl_dn5 + locals.var_nstar_dn5)));
        locals.var_t0d_dn6 = (((locals.var_nl_dn6 + locals.var_nstar_dn6) * assign27420_e36697) + (assign27420_e36694 * (locals.var_nl_dn6 + locals.var_nstar_dn6)));
        locals.var_t0d_dn7 = (((locals.var_nl_dn7 + locals.var_nstar_dn7) * assign27420_e36697) + (assign27420_e36694 * (locals.var_nl_dn7 + locals.var_nstar_dn7)));
        locals.var_t0d_dn8 = (((locals.var_nl_dn8 + locals.var_nstar_dn8) * assign27420_e36697) + (assign27420_e36694 * (locals.var_nl_dn8 + locals.var_nstar_dn8)));
        locals.var_t0d_dn9 = (((locals.var_nl_dn9 + locals.var_nstar_dn9) * assign27420_e36697) + (assign27420_e36694 * (locals.var_nl_dn9 + locals.var_nstar_dn9)));
        locals.var_t0d_dn10 = (((locals.var_nl_dn10 + locals.var_nstar_dn10) * assign27420_e36697) + (assign27420_e36694 * (locals.var_nl_dn10 + locals.var_nstar_dn10)));
        locals.var_t0d_dn11 = (((locals.var_nl_dn11 + locals.var_nstar_dn11) * assign27420_e36697) + (assign27420_e36694 * (locals.var_nl_dn11 + locals.var_nstar_dn11)));
        locals.var_t0d_dn12 = (((locals.var_nl_dn12 + locals.var_nstar_dn12) * assign27420_e36697) + (assign27420_e36694 * (locals.var_nl_dn12 + locals.var_nstar_dn12)));
        locals.var_t0d_dn13 = (((locals.var_nl_dn13 + locals.var_nstar_dn13) * assign27420_e36697) + (assign27420_e36694 * (locals.var_nl_dn13 + locals.var_nstar_dn13)));
        locals.var_t0d_dn14 = (((locals.var_nl_dn14 + locals.var_nstar_dn14) * assign27420_e36697) + (assign27420_e36694 * (locals.var_nl_dn14 + locals.var_nstar_dn14)));
        locals.var_t0d_rv = 0.0;

        let assign27430_e36701: f64 = (p.p785 * 1.60219e-19);
        let assign27430_e36703: f64 = (assign27430_e36701 * locals.var_vt);
        locals.var_t0e = assign27430_e36703;
        locals.var_t0e_dn0 = 0.0;
        locals.var_t0e_dn2 = 0.0;
        locals.var_t0e_dn3 = 0.0;
        locals.var_t0e_dn4 = (assign27430_e36701 * locals.var_vt_dn4);
        locals.var_t0e_dn5 = 0.0;
        locals.var_t0e_dn6 = 0.0;
        locals.var_t0e_dn7 = 0.0;
        locals.var_t0e_dn8 = 0.0;
        locals.var_t0e_dn9 = 0.0;
        locals.var_t0e_dn10 = 0.0;
        locals.var_t0e_dn11 = 0.0;
        locals.var_t0e_dn12 = 0.0;
        locals.var_t0e_dn13 = 0.0;
        locals.var_t0e_dn14 = 0.0;
        locals.var_t0e_rv = 0.0;

        let assign27440_e36706: f64 = if p.p1065 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard683 = assign27440_e36706;
        locals.var_guard683_rv = 0.0;

        let (assign27450_e36710,) = {
    if (locals.var_guard683 != 0.0) {
        (locals.var_leff,)
    } else {
        (locals.var_leffnoih,)
    }
};
        locals.var_leffnoih = assign27450_e36710;
        locals.var_leffnoih_rv = 0.0;

        let (assign27460_e36718, assign27460_e36718_d_n0, assign27460_e36718_d_n2, assign27460_e36718_d_n3, assign27460_e36718_d_n4, assign27460_e36718_d_n5, assign27460_e36718_d_n6, assign27460_e36718_d_n7, assign27460_e36718_d_n8, assign27460_e36718_d_n9, assign27460_e36718_d_n10, assign27460_e36718_d_n11, assign27460_e36718_d_n12, assign27460_e36718_d_n13, assign27460_e36718_d_n14,) = {
    if (locals.var_guard683 != 0.0) {
        let assign27460_e36714: f64 = (locals.var_vg - locals.var_vfb_i);
        let assign27460_e36716: f64 = (assign27460_e36714 / locals.var_vt);
        (assign27460_e36716, ((-locals.var_vfb_i_dn0) / locals.var_vt), ((-locals.var_vfb_i_dn2) / locals.var_vt), ((-locals.var_vfb_i_dn3) / locals.var_vt), ((((-locals.var_vfb_i_dn4) * locals.var_vt) - (assign27460_e36714 * locals.var_vt_dn4)) / (locals.var_vt * locals.var_vt)), ((-locals.var_vfb_i_dn5) / locals.var_vt), ((-locals.var_vfb_i_dn6) / locals.var_vt), ((-locals.var_vfb_i_dn7) / locals.var_vt), ((-locals.var_vfb_i_dn8) / locals.var_vt), ((locals.var_vg_dn9 - locals.var_vfb_i_dn9) / locals.var_vt), ((-locals.var_vfb_i_dn10) / locals.var_vt), ((locals.var_vg_dn11 - locals.var_vfb_i_dn11) / locals.var_vt), ((-locals.var_vfb_i_dn12) / locals.var_vt), ((-locals.var_vfb_i_dn13) / locals.var_vt), ((-locals.var_vfb_i_dn14) / locals.var_vt),)
    } else {
        (locals.var_vgfbh, locals.var_vgfbh_dn0, locals.var_vgfbh_dn2, locals.var_vgfbh_dn3, locals.var_vgfbh_dn4, locals.var_vgfbh_dn5, locals.var_vgfbh_dn6, locals.var_vgfbh_dn7, locals.var_vgfbh_dn8, locals.var_vgfbh_dn9, locals.var_vgfbh_dn10, locals.var_vgfbh_dn11, locals.var_vgfbh_dn12, locals.var_vgfbh_dn13, locals.var_vgfbh_dn14,)
    }
};
        locals.var_vgfbh = assign27460_e36718;
        locals.var_vgfbh_dn0 = assign27460_e36718_d_n0;
        locals.var_vgfbh_dn2 = assign27460_e36718_d_n2;
        locals.var_vgfbh_dn3 = assign27460_e36718_d_n3;
        locals.var_vgfbh_dn4 = assign27460_e36718_d_n4;
        locals.var_vgfbh_dn5 = assign27460_e36718_d_n5;
        locals.var_vgfbh_dn6 = assign27460_e36718_d_n6;
        locals.var_vgfbh_dn7 = assign27460_e36718_d_n7;
        locals.var_vgfbh_dn8 = assign27460_e36718_d_n8;
        locals.var_vgfbh_dn9 = assign27460_e36718_d_n9;
        locals.var_vgfbh_dn10 = assign27460_e36718_d_n10;
        locals.var_vgfbh_dn11 = assign27460_e36718_d_n11;
        locals.var_vgfbh_dn12 = assign27460_e36718_d_n12;
        locals.var_vgfbh_dn13 = assign27460_e36718_d_n13;
        locals.var_vgfbh_dn14 = assign27460_e36718_d_n14;
        locals.var_vgfbh_rv = 0.0;

        let (assign27470_e36733, assign27470_e36733_d_n4,) = {
    if (locals.var_guard683 != 0.0) {
        let assign27470_e36722: f64 = (2.0 * 1.60219e-19);
        let assign27470_e36724: f64 = (assign27470_e36722 * locals.var_epssi);
        let assign27470_e36726: f64 = (assign27470_e36724 * p.p1068);
        let assign27470_e36728: f64 = (assign27470_e36726 / locals.var_vt);
        let assign27470_e36729: f64 = (assign27470_e36728).sqrt();
        let assign27470_e36731: f64 = (assign27470_e36729 / locals.var_cox);
        (assign27470_e36731, (((-((assign27470_e36726 * locals.var_vt_dn4) / (locals.var_vt * locals.var_vt))) / (2.0 * assign27470_e36729)) / locals.var_cox),)
    } else {
        (locals.var_gam_h, locals.var_gam_h_dn4,)
    }
};
        locals.var_gam_h = assign27470_e36733;
        locals.var_gam_h_dn4 = assign27470_e36733_d_n4;
        locals.var_gam_h_rv = 0.0;

        let (assign27480_e36740, assign27480_e36740_d_n0, assign27480_e36740_d_n2, assign27480_e36740_d_n3, assign27480_e36740_d_n4, assign27480_e36740_d_n5, assign27480_e36740_d_n6, assign27480_e36740_d_n7, assign27480_e36740_d_n8, assign27480_e36740_d_n9, assign27480_e36740_d_n10, assign27480_e36740_d_n11, assign27480_e36740_d_n12, assign27480_e36740_d_n13, assign27480_e36740_d_n14,) = {
    if (locals.var_guard683 != 0.0) {
        let assign27480_e36737: f64 = (p.p1068 / locals.var_ni);
        let assign27480_e36738: f64 = (assign27480_e36737).ln();
        (assign27480_e36738, ((-((p.p1068 * locals.var_ni_dn0) / (locals.var_ni * locals.var_ni))) / assign27480_e36737), ((-((p.p1068 * locals.var_ni_dn2) / (locals.var_ni * locals.var_ni))) / assign27480_e36737), ((-((p.p1068 * locals.var_ni_dn3) / (locals.var_ni * locals.var_ni))) / assign27480_e36737), ((-((p.p1068 * locals.var_ni_dn4) / (locals.var_ni * locals.var_ni))) / assign27480_e36737), ((-((p.p1068 * locals.var_ni_dn5) / (locals.var_ni * locals.var_ni))) / assign27480_e36737), ((-((p.p1068 * locals.var_ni_dn6) / (locals.var_ni * locals.var_ni))) / assign27480_e36737), ((-((p.p1068 * locals.var_ni_dn7) / (locals.var_ni * locals.var_ni))) / assign27480_e36737), ((-((p.p1068 * locals.var_ni_dn8) / (locals.var_ni * locals.var_ni))) / assign27480_e36737), ((-((p.p1068 * locals.var_ni_dn9) / (locals.var_ni * locals.var_ni))) / assign27480_e36737), ((-((p.p1068 * locals.var_ni_dn10) / (locals.var_ni * locals.var_ni))) / assign27480_e36737), ((-((p.p1068 * locals.var_ni_dn11) / (locals.var_ni * locals.var_ni))) / assign27480_e36737), ((-((p.p1068 * locals.var_ni_dn12) / (locals.var_ni * locals.var_ni))) / assign27480_e36737), ((-((p.p1068 * locals.var_ni_dn13) / (locals.var_ni * locals.var_ni))) / assign27480_e36737), ((-((p.p1068 * locals.var_ni_dn14) / (locals.var_ni * locals.var_ni))) / assign27480_e36737),)
    } else {
        (locals.var_phib_h, locals.var_phib_h_dn0, locals.var_phib_h_dn2, locals.var_phib_h_dn3, locals.var_phib_h_dn4, locals.var_phib_h_dn5, locals.var_phib_h_dn6, locals.var_phib_h_dn7, locals.var_phib_h_dn8, locals.var_phib_h_dn9, locals.var_phib_h_dn10, locals.var_phib_h_dn11, locals.var_phib_h_dn12, locals.var_phib_h_dn13, locals.var_phib_h_dn14,)
    }
};
        locals.var_phib_h = assign27480_e36740;
        locals.var_phib_h_dn0 = assign27480_e36740_d_n0;
        locals.var_phib_h_dn2 = assign27480_e36740_d_n2;
        locals.var_phib_h_dn3 = assign27480_e36740_d_n3;
        locals.var_phib_h_dn4 = assign27480_e36740_d_n4;
        locals.var_phib_h_dn5 = assign27480_e36740_d_n5;
        locals.var_phib_h_dn6 = assign27480_e36740_d_n6;
        locals.var_phib_h_dn7 = assign27480_e36740_d_n7;
        locals.var_phib_h_dn8 = assign27480_e36740_d_n8;
        locals.var_phib_h_dn9 = assign27480_e36740_d_n9;
        locals.var_phib_h_dn10 = assign27480_e36740_d_n10;
        locals.var_phib_h_dn11 = assign27480_e36740_d_n11;
        locals.var_phib_h_dn12 = assign27480_e36740_d_n12;
        locals.var_phib_h_dn13 = assign27480_e36740_d_n13;
        locals.var_phib_h_dn14 = assign27480_e36740_d_n14;
        locals.var_phib_h_rv = 0.0;

        let (assign27490_e36746, assign27490_e36746_d_n0, assign27490_e36746_d_n2, assign27490_e36746_d_n3, assign27490_e36746_d_n4, assign27490_e36746_d_n5, assign27490_e36746_d_n6, assign27490_e36746_d_n7, assign27490_e36746_d_n8, assign27490_e36746_d_n9, assign27490_e36746_d_n10, assign27490_e36746_d_n11, assign27490_e36746_d_n12, assign27490_e36746_d_n13, assign27490_e36746_d_n14,) = {
    if (locals.var_guard683 != 0.0) {
        let assign27490_e36744: f64 = 1.0;
        (assign27490_e36744, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign27490_e36746;
        locals.var_t1_dn0 = assign27490_e36746_d_n0;
        locals.var_t1_dn2 = assign27490_e36746_d_n2;
        locals.var_t1_dn3 = assign27490_e36746_d_n3;
        locals.var_t1_dn4 = assign27490_e36746_d_n4;
        locals.var_t1_dn5 = assign27490_e36746_d_n5;
        locals.var_t1_dn6 = assign27490_e36746_d_n6;
        locals.var_t1_dn7 = assign27490_e36746_d_n7;
        locals.var_t1_dn8 = assign27490_e36746_d_n8;
        locals.var_t1_dn9 = assign27490_e36746_d_n9;
        locals.var_t1_dn10 = assign27490_e36746_d_n10;
        locals.var_t1_dn11 = assign27490_e36746_d_n11;
        locals.var_t1_dn12 = assign27490_e36746_d_n12;
        locals.var_t1_dn13 = assign27490_e36746_d_n13;
        locals.var_t1_dn14 = assign27490_e36746_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign27500_e36752, assign27500_e36752_d_n0, assign27500_e36752_d_n2, assign27500_e36752_d_n3, assign27500_e36752_d_n4, assign27500_e36752_d_n5, assign27500_e36752_d_n6, assign27500_e36752_d_n7, assign27500_e36752_d_n8, assign27500_e36752_d_n9, assign27500_e36752_d_n10, assign27500_e36752_d_n11, assign27500_e36752_d_n12, assign27500_e36752_d_n13, assign27500_e36752_d_n14,) = {
    if (locals.var_guard683 != 0.0) {
        let assign27500_e36750: f64 = (locals.var_vgfbh / locals.var_t1);
        (assign27500_e36750, (((locals.var_vgfbh_dn0 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn0)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn2 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn2)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn3 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn3)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn4 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn5 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn5)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn6 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn6)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn7 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn7)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn8 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn8)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn9 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn9)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn10 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn10)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn11 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn11)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn12 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn12)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn13 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn13)) / (locals.var_t1 * locals.var_t1)), (((locals.var_vgfbh_dn14 * locals.var_t1) - (locals.var_vgfbh * locals.var_t1_dn14)) / (locals.var_t1 * locals.var_t1)),)
    } else {
        (locals.var_vgfbpd, locals.var_vgfbpd_dn0, locals.var_vgfbpd_dn2, locals.var_vgfbpd_dn3, locals.var_vgfbpd_dn4, locals.var_vgfbpd_dn5, locals.var_vgfbpd_dn6, locals.var_vgfbpd_dn7, locals.var_vgfbpd_dn8, locals.var_vgfbpd_dn9, locals.var_vgfbpd_dn10, locals.var_vgfbpd_dn11, locals.var_vgfbpd_dn12, locals.var_vgfbpd_dn13, locals.var_vgfbpd_dn14,)
    }
};
        locals.var_vgfbpd = assign27500_e36752;
        locals.var_vgfbpd_dn0 = assign27500_e36752_d_n0;
        locals.var_vgfbpd_dn2 = assign27500_e36752_d_n2;
        locals.var_vgfbpd_dn3 = assign27500_e36752_d_n3;
        locals.var_vgfbpd_dn4 = assign27500_e36752_d_n4;
        locals.var_vgfbpd_dn5 = assign27500_e36752_d_n5;
        locals.var_vgfbpd_dn6 = assign27500_e36752_d_n6;
        locals.var_vgfbpd_dn7 = assign27500_e36752_d_n7;
        locals.var_vgfbpd_dn8 = assign27500_e36752_d_n8;
        locals.var_vgfbpd_dn9 = assign27500_e36752_d_n9;
        locals.var_vgfbpd_dn10 = assign27500_e36752_d_n10;
        locals.var_vgfbpd_dn11 = assign27500_e36752_d_n11;
        locals.var_vgfbpd_dn12 = assign27500_e36752_d_n12;
        locals.var_vgfbpd_dn13 = assign27500_e36752_d_n13;
        locals.var_vgfbpd_dn14 = assign27500_e36752_d_n14;
        locals.var_vgfbpd_rv = 0.0;

        let (assign27510_e36758, assign27510_e36758_d_n0, assign27510_e36758_d_n2, assign27510_e36758_d_n3, assign27510_e36758_d_n4, assign27510_e36758_d_n5, assign27510_e36758_d_n6, assign27510_e36758_d_n7, assign27510_e36758_d_n8, assign27510_e36758_d_n9, assign27510_e36758_d_n10, assign27510_e36758_d_n11, assign27510_e36758_d_n12, assign27510_e36758_d_n13, assign27510_e36758_d_n14,) = {
    if (locals.var_guard683 != 0.0) {
        let assign27510_e36756: f64 = (locals.var_gam_h / locals.var_t1);
        (assign27510_e36756, (-((locals.var_gam_h * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_gam_h * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_gam_h * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1))), (((locals.var_gam_h_dn4 * locals.var_t1) - (locals.var_gam_h * locals.var_t1_dn4)) / (locals.var_t1 * locals.var_t1)), (-((locals.var_gam_h * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_gam_h * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_gam_h * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_gam_h * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_gam_h * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((locals.var_gam_h * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_gam_h * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_gam_h * locals.var_t1_dn12) / (locals.var_t1 * locals.var_t1))), (-((locals.var_gam_h * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))), (-((locals.var_gam_h * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_gammapd, locals.var_gammapd_dn0, locals.var_gammapd_dn2, locals.var_gammapd_dn3, locals.var_gammapd_dn4, locals.var_gammapd_dn5, locals.var_gammapd_dn6, locals.var_gammapd_dn7, locals.var_gammapd_dn8, locals.var_gammapd_dn9, locals.var_gammapd_dn10, locals.var_gammapd_dn11, locals.var_gammapd_dn12, locals.var_gammapd_dn13, locals.var_gammapd_dn14,)
    }
};
        locals.var_gammapd = assign27510_e36758;
        locals.var_gammapd_dn0 = assign27510_e36758_d_n0;
        locals.var_gammapd_dn2 = assign27510_e36758_d_n2;
        locals.var_gammapd_dn3 = assign27510_e36758_d_n3;
        locals.var_gammapd_dn4 = assign27510_e36758_d_n4;
        locals.var_gammapd_dn5 = assign27510_e36758_d_n5;
        locals.var_gammapd_dn6 = assign27510_e36758_d_n6;
        locals.var_gammapd_dn7 = assign27510_e36758_d_n7;
        locals.var_gammapd_dn8 = assign27510_e36758_d_n8;
        locals.var_gammapd_dn9 = assign27510_e36758_d_n9;
        locals.var_gammapd_dn10 = assign27510_e36758_d_n10;
        locals.var_gammapd_dn11 = assign27510_e36758_d_n11;
        locals.var_gammapd_dn12 = assign27510_e36758_d_n12;
        locals.var_gammapd_dn13 = assign27510_e36758_d_n13;
        locals.var_gammapd_dn14 = assign27510_e36758_d_n14;
        locals.var_gammapd_rv = 0.0;

        let (assign27520_e36772, assign27520_e36772_d_n0, assign27520_e36772_d_n2, assign27520_e36772_d_n3, assign27520_e36772_d_n4, assign27520_e36772_d_n5, assign27520_e36772_d_n6, assign27520_e36772_d_n7, assign27520_e36772_d_n8, assign27520_e36772_d_n9, assign27520_e36772_d_n10, assign27520_e36772_d_n11, assign27520_e36772_d_n12, assign27520_e36772_d_n13, assign27520_e36772_d_n14,) = {
    if (locals.var_guard683 != 0.0) {
        let assign27520_e36762: f64 = (0.5 * locals.var_vgfbpd);
        let assign27520_e36767: f64 = (locals.var_gammapd / 1.4142135623730951);
        let assign27520_e36768: f64 = (1.0 + assign27520_e36767);
        let assign27520_e36769: f64 = (3.0 * assign27520_e36768);
        let assign27520_e36770: f64 = (assign27520_e36762 - assign27520_e36769);
        (assign27520_e36770, ((0.5 * locals.var_vgfbpd_dn0) - (3.0 * (locals.var_gammapd_dn0 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn2) - (3.0 * (locals.var_gammapd_dn2 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn3) - (3.0 * (locals.var_gammapd_dn3 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn4) - (3.0 * (locals.var_gammapd_dn4 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn5) - (3.0 * (locals.var_gammapd_dn5 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn6) - (3.0 * (locals.var_gammapd_dn6 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn7) - (3.0 * (locals.var_gammapd_dn7 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn8) - (3.0 * (locals.var_gammapd_dn8 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn9) - (3.0 * (locals.var_gammapd_dn9 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn10) - (3.0 * (locals.var_gammapd_dn10 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn11) - (3.0 * (locals.var_gammapd_dn11 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn12) - (3.0 * (locals.var_gammapd_dn12 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn13) - (3.0 * (locals.var_gammapd_dn13 / 1.4142135623730951))), ((0.5 * locals.var_vgfbpd_dn14) - (3.0 * (locals.var_gammapd_dn14 / 1.4142135623730951))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn12, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign27520_e36772;
        locals.var_t1_dn0 = assign27520_e36772_d_n0;
        locals.var_t1_dn2 = assign27520_e36772_d_n2;
        locals.var_t1_dn3 = assign27520_e36772_d_n3;
        locals.var_t1_dn4 = assign27520_e36772_d_n4;
        locals.var_t1_dn5 = assign27520_e36772_d_n5;
        locals.var_t1_dn6 = assign27520_e36772_d_n6;
        locals.var_t1_dn7 = assign27520_e36772_d_n7;
        locals.var_t1_dn8 = assign27520_e36772_d_n8;
        locals.var_t1_dn9 = assign27520_e36772_d_n9;
        locals.var_t1_dn10 = assign27520_e36772_d_n10;
        locals.var_t1_dn11 = assign27520_e36772_d_n11;
        locals.var_t1_dn12 = assign27520_e36772_d_n12;
        locals.var_t1_dn13 = assign27520_e36772_d_n13;
        locals.var_t1_dn14 = assign27520_e36772_d_n14;
        locals.var_t1_rv = 0.0;

        let (assign27530_e36785, assign27530_e36785_d_n0, assign27530_e36785_d_n2, assign27530_e36785_d_n3, assign27530_e36785_d_n4, assign27530_e36785_d_n5, assign27530_e36785_d_n6, assign27530_e36785_d_n7, assign27530_e36785_d_n8, assign27530_e36785_d_n9, assign27530_e36785_d_n10, assign27530_e36785_d_n11, assign27530_e36785_d_n12, assign27530_e36785_d_n13, assign27530_e36785_d_n14,) = {
    if (locals.var_guard683 != 0.0) {
        let assign27530_e36777: f64 = (locals.var_t1 * locals.var_t1);
        let assign27530_e36780: f64 = (6.0 * locals.var_vgfbpd);
        let assign27530_e36781: f64 = (assign27530_e36777 + assign27530_e36780);
        let assign27530_e36782: f64 = (assign27530_e36781).sqrt();
        let assign27530_e36783: f64 = (locals.var_t1 + assign27530_e36782);
        (assign27530_e36783, (locals.var_t1_dn0 + ((((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) + (6.0 * locals.var_vgfbpd_dn0)) / (2.0 * assign27530_e36782))), (locals.var_t1_dn2 + ((((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) + (6.0 * locals.var_vgfbpd_dn2)) / (2.0 * assign27530_e36782))), (locals.var_t1_dn3 + ((((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3)) + (6.0 * locals.var_vgfbpd_dn3)) / (2.0 * assign27530_e36782))), (locals.var_t1_dn4 + ((((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) + (6.0 * locals.var_vgfbpd_dn4)) / (2.0 * assign27530_e36782))), (locals.var_t1_dn5 + ((((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) + (6.0 * locals.var_vgfbpd_dn5)) / (2.0 * assign27530_e36782))), (locals.var_t1_dn6 + ((((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) + (6.0 * locals.var_vgfbpd_dn6)) / (2.0 * assign27530_e36782))), (locals.var_t1_dn7 + ((((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) + (6.0 * locals.var_vgfbpd_dn7)) / (2.0 * assign27530_e36782))), (locals.var_t1_dn8 + ((((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) + (6.0 * locals.var_vgfbpd_dn8)) / (2.0 * assign27530_e36782))), (locals.var_t1_dn9 + ((((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) + (6.0 * locals.var_vgfbpd_dn9)) / (2.0 * assign27530_e36782))), (locals.var_t1_dn10 + ((((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) + (6.0 * locals.var_vgfbpd_dn10)) / (2.0 * assign27530_e36782))), (locals.var_t1_dn11 + ((((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) + (6.0 * locals.var_vgfbpd_dn11)) / (2.0 * assign27530_e36782))), (locals.var_t1_dn12 + ((((locals.var_t1_dn12 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn12)) + (6.0 * locals.var_vgfbpd_dn12)) / (2.0 * assign27530_e36782))), (locals.var_t1_dn13 + ((((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13)) + (6.0 * locals.var_vgfbpd_dn13)) / (2.0 * assign27530_e36782))), (locals.var_t1_dn14 + ((((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) + (6.0 * locals.var_vgfbpd_dn14)) / (2.0 * assign27530_e36782))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn12, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign27530_e36785;
        locals.var_t2_dn0 = assign27530_e36785_d_n0;
        locals.var_t2_dn2 = assign27530_e36785_d_n2;
        locals.var_t2_dn3 = assign27530_e36785_d_n3;
        locals.var_t2_dn4 = assign27530_e36785_d_n4;
        locals.var_t2_dn5 = assign27530_e36785_d_n5;
        locals.var_t2_dn6 = assign27530_e36785_d_n6;
        locals.var_t2_dn7 = assign27530_e36785_d_n7;
        locals.var_t2_dn8 = assign27530_e36785_d_n8;
        locals.var_t2_dn9 = assign27530_e36785_d_n9;
        locals.var_t2_dn10 = assign27530_e36785_d_n10;
        locals.var_t2_dn11 = assign27530_e36785_d_n11;
        locals.var_t2_dn12 = assign27530_e36785_d_n12;
        locals.var_t2_dn13 = assign27530_e36785_d_n13;
        locals.var_t2_dn14 = assign27530_e36785_d_n14;
        locals.var_t2_rv = 0.0;

        let assign27540_e36788: f64 = if locals.var_vgfbpd < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard684 = assign27540_e36788;
        locals.var_guard684_rv = 0.0;

        let (assign27550_e36798, assign27550_e36798_d_n0, assign27550_e36798_d_n2, assign27550_e36798_d_n3, assign27550_e36798_d_n4, assign27550_e36798_d_n5, assign27550_e36798_d_n6, assign27550_e36798_d_n7, assign27550_e36798_d_n8, assign27550_e36798_d_n9, assign27550_e36798_d_n10, assign27550_e36798_d_n11, assign27550_e36798_d_n12, assign27550_e36798_d_n13, assign27550_e36798_d_n14,) = {
    if ((locals.var_guard683 != 0.0) && (locals.var_guard684 != 0.0)) {
        let assign27550_e36794: f64 = (locals.var_vgfbpd - locals.var_t2);
        let assign27550_e36796: f64 = (assign27550_e36794 / locals.var_gammapd);
        (assign27550_e36796, ((((locals.var_vgfbpd_dn0 - locals.var_t2_dn0) * locals.var_gammapd) - (assign27550_e36794 * locals.var_gammapd_dn0)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn2 - locals.var_t2_dn2) * locals.var_gammapd) - (assign27550_e36794 * locals.var_gammapd_dn2)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn3 - locals.var_t2_dn3) * locals.var_gammapd) - (assign27550_e36794 * locals.var_gammapd_dn3)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn4 - locals.var_t2_dn4) * locals.var_gammapd) - (assign27550_e36794 * locals.var_gammapd_dn4)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn5 - locals.var_t2_dn5) * locals.var_gammapd) - (assign27550_e36794 * locals.var_gammapd_dn5)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn6 - locals.var_t2_dn6) * locals.var_gammapd) - (assign27550_e36794 * locals.var_gammapd_dn6)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn7 - locals.var_t2_dn7) * locals.var_gammapd) - (assign27550_e36794 * locals.var_gammapd_dn7)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn8 - locals.var_t2_dn8) * locals.var_gammapd) - (assign27550_e36794 * locals.var_gammapd_dn8)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn9 - locals.var_t2_dn9) * locals.var_gammapd) - (assign27550_e36794 * locals.var_gammapd_dn9)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn10 - locals.var_t2_dn10) * locals.var_gammapd) - (assign27550_e36794 * locals.var_gammapd_dn10)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn11 - locals.var_t2_dn11) * locals.var_gammapd) - (assign27550_e36794 * locals.var_gammapd_dn11)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn12 - locals.var_t2_dn12) * locals.var_gammapd) - (assign27550_e36794 * locals.var_gammapd_dn12)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn13 - locals.var_t2_dn13) * locals.var_gammapd) - (assign27550_e36794 * locals.var_gammapd_dn13)) / (locals.var_gammapd * locals.var_gammapd)), ((((locals.var_vgfbpd_dn14 - locals.var_t2_dn14) * locals.var_gammapd) - (assign27550_e36794 * locals.var_gammapd_dn14)) / (locals.var_gammapd * locals.var_gammapd)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn12, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign27550_e36798;
        locals.var_t3_dn0 = assign27550_e36798_d_n0;
        locals.var_t3_dn2 = assign27550_e36798_d_n2;
        locals.var_t3_dn3 = assign27550_e36798_d_n3;
        locals.var_t3_dn4 = assign27550_e36798_d_n4;
        locals.var_t3_dn5 = assign27550_e36798_d_n5;
        locals.var_t3_dn6 = assign27550_e36798_d_n6;
        locals.var_t3_dn7 = assign27550_e36798_d_n7;
        locals.var_t3_dn8 = assign27550_e36798_d_n8;
        locals.var_t3_dn9 = assign27550_e36798_d_n9;
        locals.var_t3_dn10 = assign27550_e36798_d_n10;
        locals.var_t3_dn11 = assign27550_e36798_d_n11;
        locals.var_t3_dn12 = assign27550_e36798_d_n12;
        locals.var_t3_dn13 = assign27550_e36798_d_n13;
        locals.var_t3_dn14 = assign27550_e36798_d_n14;
        locals.var_t3_rv = 0.0;

        let (assign27560_e36814, assign27560_e36814_d_n0, assign27560_e36814_d_n2, assign27560_e36814_d_n3, assign27560_e36814_d_n4, assign27560_e36814_d_n5, assign27560_e36814_d_n6, assign27560_e36814_d_n7, assign27560_e36814_d_n8, assign27560_e36814_d_n9, assign27560_e36814_d_n10, assign27560_e36814_d_n11, assign27560_e36814_d_n12, assign27560_e36814_d_n13, assign27560_e36814_d_n14,) = {
    if ((locals.var_guard683 != 0.0) && (locals.var_guard684 != 0.0)) {
        let assign27560_e36804: f64 = (1.0 - locals.var_t2);
        let assign27560_e36807: f64 = (locals.var_t3 * locals.var_t3);
        let assign27560_e36808: f64 = (assign27560_e36804 + assign27560_e36807);
        let assign27560_e36810: f64 = (assign27560_e36808).max(1e-38);
        let assign27560_e36811: f64 = (assign27560_e36810).ln();
        let assign27560_e36812: f64 = (-assign27560_e36811);
        (assign27560_e36812, (-(if assign27560_e36808 >= 1e-38 { ((-locals.var_t2_dn0) + ((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0))) } else { 0.0 } / assign27560_e36810)), (-(if assign27560_e36808 >= 1e-38 { ((-locals.var_t2_dn2) + ((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2))) } else { 0.0 } / assign27560_e36810)), (-(if assign27560_e36808 >= 1e-38 { ((-locals.var_t2_dn3) + ((locals.var_t3_dn3 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn3))) } else { 0.0 } / assign27560_e36810)), (-(if assign27560_e36808 >= 1e-38 { ((-locals.var_t2_dn4) + ((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4))) } else { 0.0 } / assign27560_e36810)), (-(if assign27560_e36808 >= 1e-38 { ((-locals.var_t2_dn5) + ((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5))) } else { 0.0 } / assign27560_e36810)), (-(if assign27560_e36808 >= 1e-38 { ((-locals.var_t2_dn6) + ((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6))) } else { 0.0 } / assign27560_e36810)), (-(if assign27560_e36808 >= 1e-38 { ((-locals.var_t2_dn7) + ((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7))) } else { 0.0 } / assign27560_e36810)), (-(if assign27560_e36808 >= 1e-38 { ((-locals.var_t2_dn8) + ((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8))) } else { 0.0 } / assign27560_e36810)), (-(if assign27560_e36808 >= 1e-38 { ((-locals.var_t2_dn9) + ((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9))) } else { 0.0 } / assign27560_e36810)), (-(if assign27560_e36808 >= 1e-38 { ((-locals.var_t2_dn10) + ((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10))) } else { 0.0 } / assign27560_e36810)), (-(if assign27560_e36808 >= 1e-38 { ((-locals.var_t2_dn11) + ((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11))) } else { 0.0 } / assign27560_e36810)), (-(if assign27560_e36808 >= 1e-38 { ((-locals.var_t2_dn12) + ((locals.var_t3_dn12 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn12))) } else { 0.0 } / assign27560_e36810)), (-(if assign27560_e36808 >= 1e-38 { ((-locals.var_t2_dn13) + ((locals.var_t3_dn13 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn13))) } else { 0.0 } / assign27560_e36810)), (-(if assign27560_e36808 >= 1e-38 { ((-locals.var_t2_dn14) + ((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14))) } else { 0.0 } / assign27560_e36810)),)
    } else {
        (locals.var_psiph, locals.var_psiph_dn0, locals.var_psiph_dn2, locals.var_psiph_dn3, locals.var_psiph_dn4, locals.var_psiph_dn5, locals.var_psiph_dn6, locals.var_psiph_dn7, locals.var_psiph_dn8, locals.var_psiph_dn9, locals.var_psiph_dn10, locals.var_psiph_dn11, locals.var_psiph_dn12, locals.var_psiph_dn13, locals.var_psiph_dn14,)
    }
};
        locals.var_psiph = assign27560_e36814;
        locals.var_psiph_dn0 = assign27560_e36814_d_n0;
        locals.var_psiph_dn2 = assign27560_e36814_d_n2;
        locals.var_psiph_dn3 = assign27560_e36814_d_n3;
        locals.var_psiph_dn4 = assign27560_e36814_d_n4;
        locals.var_psiph_dn5 = assign27560_e36814_d_n5;
        locals.var_psiph_dn6 = assign27560_e36814_d_n6;
        locals.var_psiph_dn7 = assign27560_e36814_d_n7;
        locals.var_psiph_dn8 = assign27560_e36814_d_n8;
        locals.var_psiph_dn9 = assign27560_e36814_d_n9;
        locals.var_psiph_dn10 = assign27560_e36814_d_n10;
        locals.var_psiph_dn11 = assign27560_e36814_d_n11;
        locals.var_psiph_dn12 = assign27560_e36814_d_n12;
        locals.var_psiph_dn13 = assign27560_e36814_d_n13;
        locals.var_psiph_dn14 = assign27560_e36814_d_n14;
        locals.var_psiph_rv = 0.0;

    }
}

#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_58(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign21220_e20574: f64 = (-50.0);
        let assign21220_e20575: f64 = if locals.var_fn241_calc_iq__etac < assign21220_e20574 { 1.0 } else { 0.0 };
        locals.var_guard269 = assign21220_e20575;
        locals.var_guard269_rv = 0.0;

        let (assign21230_e20587, assign21230_e20587_d_n2, assign21230_e20587_d_n3, assign21230_e20587_d_n4, assign21230_e20587_d_n7, assign21230_e20587_d_n11, assign21230_e20587_d_n12,) = {
    if ((((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard268 == 0.0)) && (locals.var_guard269 != 0.0)) {
        let assign21230_e20585: f64 = (locals.var_fn241_calc_iq__etac).exp();
        (assign21230_e20585, (assign21230_e20585 * locals.var_fn241_calc_iq__etac_dn2), 0.0, (assign21230_e20585 * locals.var_fn241_calc_iq__etac_dn4), (assign21230_e20585 * locals.var_fn241_calc_iq__etac_dn7), 0.0, (assign21230_e20585 * locals.var_fn241_calc_iq__etac_dn12),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21230_e20587;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21230_e20587_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21230_e20587_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21230_e20587_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21230_e20587_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21230_e20587_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21230_e20587_d_n12;
        locals.var_fn241_calc_iq__exparg_rv = 0.0;

        let (assign21240_e20603, assign21240_e20603_d_n2, assign21240_e20603_d_n3, assign21240_e20603_d_n4, assign21240_e20603_d_n7, assign21240_e20603_d_n11, assign21240_e20603_d_n12,) = {
    if ((((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard268 == 0.0)) && (locals.var_guard269 == 0.0)) {
        let assign21240_e20599: f64 = (locals.var_fn241_calc_iq__etac).exp();
        let assign21240_e20600: f64 = (1.0 + assign21240_e20599);
        let assign21240_e20601: f64 = (assign21240_e20600).ln();
        (assign21240_e20601, ((assign21240_e20599 * locals.var_fn241_calc_iq__etac_dn2) / assign21240_e20600), 0.0, ((assign21240_e20599 * locals.var_fn241_calc_iq__etac_dn4) / assign21240_e20600), ((assign21240_e20599 * locals.var_fn241_calc_iq__etac_dn7) / assign21240_e20600), 0.0, ((assign21240_e20599 * locals.var_fn241_calc_iq__etac_dn12) / assign21240_e20600),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21240_e20603;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21240_e20603_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21240_e20603_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21240_e20603_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21240_e20603_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21240_e20603_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21240_e20603_d_n12;
        locals.var_fn241_calc_iq__exparg_rv = 0.0;

        let (assign21250_e20621, assign21250_e20621_d_n2, assign21250_e20621_d_n3, assign21250_e20621_d_n4, assign21250_e20621_d_n7, assign21250_e20621_d_n11, assign21250_e20621_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) {
        let assign21250_e20609: f64 = (locals.var_fn241_calc_iq__w * locals.var_fn241_calc_iq__ngf);
        let assign21250_e20611: f64 = (assign21250_e20609 * locals.var_fn241_calc_iq__type);
        let assign21250_e20613: f64 = (assign21250_e20611 * locals.var_fn241_calc_iq__cc);
        let assign21250_e20615: f64 = (assign21250_e20613 * locals.var_fn241_calc_iq__two_n_phit0);
        let assign21250_e20617: f64 = (assign21250_e20615 * locals.var_fn241_calc_iq__exparg);
        let assign21250_e20619: f64 = (assign21250_e20617 * locals.var_fn241_calc_iq__trapfracdl);
        (assign21250_e20619, ((assign21250_e20615 * locals.var_fn241_calc_iq__exparg_dn2) * locals.var_fn241_calc_iq__trapfracdl), ((assign21250_e20615 * locals.var_fn241_calc_iq__exparg_dn3) * locals.var_fn241_calc_iq__trapfracdl), ((((((assign21250_e20611 * locals.var_fn241_calc_iq__cc_dn4) * locals.var_fn241_calc_iq__two_n_phit0) + (assign21250_e20613 * locals.var_fn241_calc_iq__two_n_phit0_dn4)) * locals.var_fn241_calc_iq__exparg) + (assign21250_e20615 * locals.var_fn241_calc_iq__exparg_dn4)) * locals.var_fn241_calc_iq__trapfracdl), ((assign21250_e20615 * locals.var_fn241_calc_iq__exparg_dn7) * locals.var_fn241_calc_iq__trapfracdl), ((assign21250_e20615 * locals.var_fn241_calc_iq__exparg_dn11) * locals.var_fn241_calc_iq__trapfracdl), ((assign21250_e20615 * locals.var_fn241_calc_iq__exparg_dn12) * locals.var_fn241_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn241_calc_iq__qcout, locals.var_fn241_calc_iq__qcout_dn2, locals.var_fn241_calc_iq__qcout_dn3, locals.var_fn241_calc_iq__qcout_dn4, locals.var_fn241_calc_iq__qcout_dn7, locals.var_fn241_calc_iq__qcout_dn11, locals.var_fn241_calc_iq__qcout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qcout = assign21250_e20621;
        locals.var_fn241_calc_iq__qcout_dn2 = assign21250_e20621_d_n2;
        locals.var_fn241_calc_iq__qcout_dn3 = assign21250_e20621_d_n3;
        locals.var_fn241_calc_iq__qcout_dn4 = assign21250_e20621_d_n4;
        locals.var_fn241_calc_iq__qcout_dn7 = assign21250_e20621_d_n7;
        locals.var_fn241_calc_iq__qcout_dn11 = assign21250_e20621_d_n11;
        locals.var_fn241_calc_iq__qcout_dn12 = assign21250_e20621_d_n12;
        locals.var_fn241_calc_iq__qcout_rv = 0.0;

        let (assign21260_e20637, assign21260_e20637_d_n3, assign21260_e20637_d_n4, assign21260_e20637_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) {
        let assign21260_e20629: f64 = (p.p51 * 0.5);
        let assign21260_e20631: f64 = (assign21260_e20629 * locals.var_fn241_calc_iq__alpha_phit);
        let assign21260_e20632: f64 = (locals.var_fn241_calc_iq__vtof - assign21260_e20631);
        let assign21260_e20633: f64 = (locals.var_fn241_calc_iq__vbin - assign21260_e20632);
        let assign21260_e20635: f64 = (assign21260_e20633 / locals.var_fn241_calc_iq__two_n_phit0);
        (assign21260_e20635, (locals.var_fn241_calc_iq__vbin_dn3 / locals.var_fn241_calc_iq__two_n_phit0), ((((-(locals.var_fn241_calc_iq__vtof_dn4 - (assign21260_e20629 * locals.var_fn241_calc_iq__alpha_phit_dn4))) * locals.var_fn241_calc_iq__two_n_phit0) - (assign21260_e20633 * locals.var_fn241_calc_iq__two_n_phit0_dn4)) / (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__two_n_phit0)), (locals.var_fn241_calc_iq__vbin_dn12 / locals.var_fn241_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn241_calc_iq__etab, locals.var_fn241_calc_iq__etab_dn3, locals.var_fn241_calc_iq__etab_dn4, locals.var_fn241_calc_iq__etab_dn12,)
    }
};
        locals.var_fn241_calc_iq__etab = assign21260_e20637;
        locals.var_fn241_calc_iq__etab_dn3 = assign21260_e20637_d_n3;
        locals.var_fn241_calc_iq__etab_dn4 = assign21260_e20637_d_n4;
        locals.var_fn241_calc_iq__etab_dn12 = assign21260_e20637_d_n12;
        locals.var_fn241_calc_iq__etab_rv = 0.0;

        let assign21270_e20640: f64 = if locals.var_fn241_calc_iq__etab > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard270 = assign21270_e20640;
        locals.var_guard270_rv = 0.0;

        let (assign21280_e20648, assign21280_e20648_d_n2, assign21280_e20648_d_n3, assign21280_e20648_d_n4, assign21280_e20648_d_n7, assign21280_e20648_d_n11, assign21280_e20648_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard270 != 0.0)) {
        (locals.var_fn241_calc_iq__etab, 0.0, locals.var_fn241_calc_iq__etab_dn3, locals.var_fn241_calc_iq__etab_dn4, 0.0, 0.0, locals.var_fn241_calc_iq__etab_dn12,)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21280_e20648;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21280_e20648_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21280_e20648_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21280_e20648_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21280_e20648_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21280_e20648_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21280_e20648_d_n12;
        locals.var_fn241_calc_iq__exparg_rv = 0.0;

        let assign21290_e20651: f64 = (-50.0);
        let assign21290_e20652: f64 = if locals.var_fn241_calc_iq__etab < assign21290_e20651 { 1.0 } else { 0.0 };
        locals.var_guard271 = assign21290_e20652;
        locals.var_guard271_rv = 0.0;

        let (assign21300_e20664, assign21300_e20664_d_n2, assign21300_e20664_d_n3, assign21300_e20664_d_n4, assign21300_e20664_d_n7, assign21300_e20664_d_n11, assign21300_e20664_d_n12,) = {
    if ((((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard271 != 0.0)) {
        let assign21300_e20662: f64 = (locals.var_fn241_calc_iq__etab).exp();
        (assign21300_e20662, 0.0, (assign21300_e20662 * locals.var_fn241_calc_iq__etab_dn3), (assign21300_e20662 * locals.var_fn241_calc_iq__etab_dn4), 0.0, 0.0, (assign21300_e20662 * locals.var_fn241_calc_iq__etab_dn12),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21300_e20664;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21300_e20664_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21300_e20664_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21300_e20664_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21300_e20664_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21300_e20664_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21300_e20664_d_n12;
        locals.var_fn241_calc_iq__exparg_rv = 0.0;

        let (assign21310_e20680, assign21310_e20680_d_n2, assign21310_e20680_d_n3, assign21310_e20680_d_n4, assign21310_e20680_d_n7, assign21310_e20680_d_n11, assign21310_e20680_d_n12,) = {
    if ((((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) && (locals.var_guard270 == 0.0)) && (locals.var_guard271 == 0.0)) {
        let assign21310_e20676: f64 = (locals.var_fn241_calc_iq__etab).exp();
        let assign21310_e20677: f64 = (1.0 + assign21310_e20676);
        let assign21310_e20678: f64 = (assign21310_e20677).ln();
        (assign21310_e20678, 0.0, ((assign21310_e20676 * locals.var_fn241_calc_iq__etab_dn3) / assign21310_e20677), ((assign21310_e20676 * locals.var_fn241_calc_iq__etab_dn4) / assign21310_e20677), 0.0, 0.0, ((assign21310_e20676 * locals.var_fn241_calc_iq__etab_dn12) / assign21310_e20677),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21310_e20680;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21310_e20680_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21310_e20680_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21310_e20680_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21310_e20680_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21310_e20680_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21310_e20680_d_n12;
        locals.var_fn241_calc_iq__exparg_rv = 0.0;

        let (assign21320_e20698, assign21320_e20698_d_n2, assign21320_e20698_d_n3, assign21320_e20698_d_n4, assign21320_e20698_d_n7, assign21320_e20698_d_n11, assign21320_e20698_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard267 != 0.0)) {
        let assign21320_e20686: f64 = (locals.var_fn241_calc_iq__w * locals.var_fn241_calc_iq__ngf);
        let assign21320_e20688: f64 = (assign21320_e20686 * locals.var_fn241_calc_iq__type);
        let assign21320_e20690: f64 = (assign21320_e20688 * locals.var_fn241_calc_iq__cb);
        let assign21320_e20692: f64 = (assign21320_e20690 * locals.var_fn241_calc_iq__two_n_phit0);
        let assign21320_e20694: f64 = (assign21320_e20692 * locals.var_fn241_calc_iq__exparg);
        let assign21320_e20696: f64 = (assign21320_e20694 * locals.var_fn241_calc_iq__trapfracdl);
        (assign21320_e20696, ((assign21320_e20692 * locals.var_fn241_calc_iq__exparg_dn2) * locals.var_fn241_calc_iq__trapfracdl), ((assign21320_e20692 * locals.var_fn241_calc_iq__exparg_dn3) * locals.var_fn241_calc_iq__trapfracdl), ((((((assign21320_e20688 * locals.var_fn241_calc_iq__cb_dn4) * locals.var_fn241_calc_iq__two_n_phit0) + (assign21320_e20690 * locals.var_fn241_calc_iq__two_n_phit0_dn4)) * locals.var_fn241_calc_iq__exparg) + (assign21320_e20692 * locals.var_fn241_calc_iq__exparg_dn4)) * locals.var_fn241_calc_iq__trapfracdl), ((assign21320_e20692 * locals.var_fn241_calc_iq__exparg_dn7) * locals.var_fn241_calc_iq__trapfracdl), ((assign21320_e20692 * locals.var_fn241_calc_iq__exparg_dn11) * locals.var_fn241_calc_iq__trapfracdl), ((assign21320_e20692 * locals.var_fn241_calc_iq__exparg_dn12) * locals.var_fn241_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn241_calc_iq__qbout, locals.var_fn241_calc_iq__qbout_dn2, locals.var_fn241_calc_iq__qbout_dn3, locals.var_fn241_calc_iq__qbout_dn4, locals.var_fn241_calc_iq__qbout_dn7, locals.var_fn241_calc_iq__qbout_dn11, locals.var_fn241_calc_iq__qbout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qbout = assign21320_e20698;
        locals.var_fn241_calc_iq__qbout_dn2 = assign21320_e20698_d_n2;
        locals.var_fn241_calc_iq__qbout_dn3 = assign21320_e20698_d_n3;
        locals.var_fn241_calc_iq__qbout_dn4 = assign21320_e20698_d_n4;
        locals.var_fn241_calc_iq__qbout_dn7 = assign21320_e20698_d_n7;
        locals.var_fn241_calc_iq__qbout_dn11 = assign21320_e20698_d_n11;
        locals.var_fn241_calc_iq__qbout_dn12 = assign21320_e20698_d_n12;
        locals.var_fn241_calc_iq__qbout_rv = 0.0;

        let (assign21330_e20705, assign21330_e20705_d_n2, assign21330_e20705_d_n3, assign21330_e20705_d_n4, assign21330_e20705_d_n7, assign21330_e20705_d_n11, assign21330_e20705_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard267 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qcout, locals.var_fn241_calc_iq__qcout_dn2, locals.var_fn241_calc_iq__qcout_dn3, locals.var_fn241_calc_iq__qcout_dn4, locals.var_fn241_calc_iq__qcout_dn7, locals.var_fn241_calc_iq__qcout_dn11, locals.var_fn241_calc_iq__qcout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qcout = assign21330_e20705;
        locals.var_fn241_calc_iq__qcout_dn2 = assign21330_e20705_d_n2;
        locals.var_fn241_calc_iq__qcout_dn3 = assign21330_e20705_d_n3;
        locals.var_fn241_calc_iq__qcout_dn4 = assign21330_e20705_d_n4;
        locals.var_fn241_calc_iq__qcout_dn7 = assign21330_e20705_d_n7;
        locals.var_fn241_calc_iq__qcout_dn11 = assign21330_e20705_d_n11;
        locals.var_fn241_calc_iq__qcout_dn12 = assign21330_e20705_d_n12;
        locals.var_fn241_calc_iq__qcout_rv = 0.0;

        let (assign21340_e20712, assign21340_e20712_d_n2, assign21340_e20712_d_n3, assign21340_e20712_d_n4, assign21340_e20712_d_n7, assign21340_e20712_d_n11, assign21340_e20712_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard267 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qbout, locals.var_fn241_calc_iq__qbout_dn2, locals.var_fn241_calc_iq__qbout_dn3, locals.var_fn241_calc_iq__qbout_dn4, locals.var_fn241_calc_iq__qbout_dn7, locals.var_fn241_calc_iq__qbout_dn11, locals.var_fn241_calc_iq__qbout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qbout = assign21340_e20712;
        locals.var_fn241_calc_iq__qbout_dn2 = assign21340_e20712_d_n2;
        locals.var_fn241_calc_iq__qbout_dn3 = assign21340_e20712_d_n3;
        locals.var_fn241_calc_iq__qbout_dn4 = assign21340_e20712_d_n4;
        locals.var_fn241_calc_iq__qbout_dn7 = assign21340_e20712_d_n7;
        locals.var_fn241_calc_iq__qbout_dn11 = assign21340_e20712_d_n11;
        locals.var_fn241_calc_iq__qbout_dn12 = assign21340_e20712_d_n12;
        locals.var_fn241_calc_iq__qbout_rv = 0.0;

        let assign21350_e20715: f64 = if locals.var_fn241_calc_iq__qgsflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard272 = assign21350_e20715;
        locals.var_guard272_rv = 0.0;

        let (assign21360_e20731, assign21360_e20731_d_n2, assign21360_e20731_d_n4, assign21360_e20731_d_n7, assign21360_e20731_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard272 != 0.0)) {
        let assign21360_e20723: f64 = (p.p51 * 0.5);
        let assign21360_e20725: f64 = (assign21360_e20723 * locals.var_fn241_calc_iq__alpha_phit);
        let assign21360_e20726: f64 = (locals.var_fn241_calc_iq__vtof - assign21360_e20725);
        let assign21360_e20727: f64 = (locals.var_fn241_calc_iq__vgsin - assign21360_e20726);
        let assign21360_e20729: f64 = (assign21360_e20727 / locals.var_fn241_calc_iq__two_n_phit0);
        (assign21360_e20729, (locals.var_fn241_calc_iq__vgsin_dn2 / locals.var_fn241_calc_iq__two_n_phit0), ((((-(locals.var_fn241_calc_iq__vtof_dn4 - (assign21360_e20723 * locals.var_fn241_calc_iq__alpha_phit_dn4))) * locals.var_fn241_calc_iq__two_n_phit0) - (assign21360_e20727 * locals.var_fn241_calc_iq__two_n_phit0_dn4)) / (locals.var_fn241_calc_iq__two_n_phit0 * locals.var_fn241_calc_iq__two_n_phit0)), (locals.var_fn241_calc_iq__vgsin_dn7 / locals.var_fn241_calc_iq__two_n_phit0), (locals.var_fn241_calc_iq__vgsin_dn12 / locals.var_fn241_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn241_calc_iq__etags, locals.var_fn241_calc_iq__etags_dn2, locals.var_fn241_calc_iq__etags_dn4, locals.var_fn241_calc_iq__etags_dn7, locals.var_fn241_calc_iq__etags_dn12,)
    }
};
        locals.var_fn241_calc_iq__etags = assign21360_e20731;
        locals.var_fn241_calc_iq__etags_dn2 = assign21360_e20731_d_n2;
        locals.var_fn241_calc_iq__etags_dn4 = assign21360_e20731_d_n4;
        locals.var_fn241_calc_iq__etags_dn7 = assign21360_e20731_d_n7;
        locals.var_fn241_calc_iq__etags_dn12 = assign21360_e20731_d_n12;
        locals.var_fn241_calc_iq__etags_rv = 0.0;

        let assign21370_e20734: f64 = if locals.var_fn241_calc_iq__etags > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard273 = assign21370_e20734;
        locals.var_guard273_rv = 0.0;

        let (assign21380_e20742, assign21380_e20742_d_n2, assign21380_e20742_d_n3, assign21380_e20742_d_n4, assign21380_e20742_d_n7, assign21380_e20742_d_n11, assign21380_e20742_d_n12,) = {
    if (((locals.var_guard240 != 0.0) && (locals.var_guard272 != 0.0)) && (locals.var_guard273 != 0.0)) {
        (locals.var_fn241_calc_iq__etags, locals.var_fn241_calc_iq__etags_dn2, 0.0, locals.var_fn241_calc_iq__etags_dn4, locals.var_fn241_calc_iq__etags_dn7, 0.0, locals.var_fn241_calc_iq__etags_dn12,)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21380_e20742;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21380_e20742_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21380_e20742_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21380_e20742_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21380_e20742_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21380_e20742_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21380_e20742_d_n12;
        locals.var_fn241_calc_iq__exparg_rv = 0.0;

        let assign21390_e20745: f64 = (-50.0);
        let assign21390_e20746: f64 = if locals.var_fn241_calc_iq__etags < assign21390_e20745 { 1.0 } else { 0.0 };
        locals.var_guard274 = assign21390_e20746;
        locals.var_guard274_rv = 0.0;

        let (assign21400_e20758, assign21400_e20758_d_n2, assign21400_e20758_d_n3, assign21400_e20758_d_n4, assign21400_e20758_d_n7, assign21400_e20758_d_n11, assign21400_e20758_d_n12,) = {
    if ((((locals.var_guard240 != 0.0) && (locals.var_guard272 != 0.0)) && (locals.var_guard273 == 0.0)) && (locals.var_guard274 != 0.0)) {
        let assign21400_e20756: f64 = (locals.var_fn241_calc_iq__etags).exp();
        (assign21400_e20756, (assign21400_e20756 * locals.var_fn241_calc_iq__etags_dn2), 0.0, (assign21400_e20756 * locals.var_fn241_calc_iq__etags_dn4), (assign21400_e20756 * locals.var_fn241_calc_iq__etags_dn7), 0.0, (assign21400_e20756 * locals.var_fn241_calc_iq__etags_dn12),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21400_e20758;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21400_e20758_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21400_e20758_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21400_e20758_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21400_e20758_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21400_e20758_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21400_e20758_d_n12;
        locals.var_fn241_calc_iq__exparg_rv = 0.0;

        let (assign21410_e20774, assign21410_e20774_d_n2, assign21410_e20774_d_n3, assign21410_e20774_d_n4, assign21410_e20774_d_n7, assign21410_e20774_d_n11, assign21410_e20774_d_n12,) = {
    if ((((locals.var_guard240 != 0.0) && (locals.var_guard272 != 0.0)) && (locals.var_guard273 == 0.0)) && (locals.var_guard274 == 0.0)) {
        let assign21410_e20770: f64 = (locals.var_fn241_calc_iq__etags).exp();
        let assign21410_e20771: f64 = (1.0 + assign21410_e20770);
        let assign21410_e20772: f64 = (assign21410_e20771).ln();
        (assign21410_e20772, ((assign21410_e20770 * locals.var_fn241_calc_iq__etags_dn2) / assign21410_e20771), 0.0, ((assign21410_e20770 * locals.var_fn241_calc_iq__etags_dn4) / assign21410_e20771), ((assign21410_e20770 * locals.var_fn241_calc_iq__etags_dn7) / assign21410_e20771), 0.0, ((assign21410_e20770 * locals.var_fn241_calc_iq__etags_dn12) / assign21410_e20771),)
    } else {
        (locals.var_fn241_calc_iq__exparg, locals.var_fn241_calc_iq__exparg_dn2, locals.var_fn241_calc_iq__exparg_dn3, locals.var_fn241_calc_iq__exparg_dn4, locals.var_fn241_calc_iq__exparg_dn7, locals.var_fn241_calc_iq__exparg_dn11, locals.var_fn241_calc_iq__exparg_dn12,)
    }
};
        locals.var_fn241_calc_iq__exparg = assign21410_e20774;
        locals.var_fn241_calc_iq__exparg_dn2 = assign21410_e20774_d_n2;
        locals.var_fn241_calc_iq__exparg_dn3 = assign21410_e20774_d_n3;
        locals.var_fn241_calc_iq__exparg_dn4 = assign21410_e20774_d_n4;
        locals.var_fn241_calc_iq__exparg_dn7 = assign21410_e20774_d_n7;
        locals.var_fn241_calc_iq__exparg_dn11 = assign21410_e20774_d_n11;
        locals.var_fn241_calc_iq__exparg_dn12 = assign21410_e20774_d_n12;
        locals.var_fn241_calc_iq__exparg_rv = 0.0;

        let (assign21420_e20792, assign21420_e20792_d_n2, assign21420_e20792_d_n3, assign21420_e20792_d_n4, assign21420_e20792_d_n7, assign21420_e20792_d_n11, assign21420_e20792_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard272 != 0.0)) {
        let assign21420_e20780: f64 = (locals.var_fn241_calc_iq__w * locals.var_fn241_calc_iq__ngf);
        let assign21420_e20782: f64 = (assign21420_e20780 * locals.var_fn241_calc_iq__type);
        let assign21420_e20784: f64 = (assign21420_e20782 * locals.var_fn241_calc_iq__cs);
        let assign21420_e20786: f64 = (assign21420_e20784 * locals.var_fn241_calc_iq__two_n_phit0);
        let assign21420_e20788: f64 = (assign21420_e20786 * locals.var_fn241_calc_iq__exparg);
        let assign21420_e20790: f64 = (assign21420_e20788 * locals.var_fn241_calc_iq__trapfracdl);
        (assign21420_e20790, ((assign21420_e20786 * locals.var_fn241_calc_iq__exparg_dn2) * locals.var_fn241_calc_iq__trapfracdl), ((assign21420_e20786 * locals.var_fn241_calc_iq__exparg_dn3) * locals.var_fn241_calc_iq__trapfracdl), ((((assign21420_e20784 * locals.var_fn241_calc_iq__two_n_phit0_dn4) * locals.var_fn241_calc_iq__exparg) + (assign21420_e20786 * locals.var_fn241_calc_iq__exparg_dn4)) * locals.var_fn241_calc_iq__trapfracdl), ((assign21420_e20786 * locals.var_fn241_calc_iq__exparg_dn7) * locals.var_fn241_calc_iq__trapfracdl), ((assign21420_e20786 * locals.var_fn241_calc_iq__exparg_dn11) * locals.var_fn241_calc_iq__trapfracdl), ((assign21420_e20786 * locals.var_fn241_calc_iq__exparg_dn12) * locals.var_fn241_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn241_calc_iq__qsout, locals.var_fn241_calc_iq__qsout_dn2, locals.var_fn241_calc_iq__qsout_dn3, locals.var_fn241_calc_iq__qsout_dn4, locals.var_fn241_calc_iq__qsout_dn7, locals.var_fn241_calc_iq__qsout_dn11, locals.var_fn241_calc_iq__qsout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qsout = assign21420_e20792;
        locals.var_fn241_calc_iq__qsout_dn2 = assign21420_e20792_d_n2;
        locals.var_fn241_calc_iq__qsout_dn3 = assign21420_e20792_d_n3;
        locals.var_fn241_calc_iq__qsout_dn4 = assign21420_e20792_d_n4;
        locals.var_fn241_calc_iq__qsout_dn7 = assign21420_e20792_d_n7;
        locals.var_fn241_calc_iq__qsout_dn11 = assign21420_e20792_d_n11;
        locals.var_fn241_calc_iq__qsout_dn12 = assign21420_e20792_d_n12;
        locals.var_fn241_calc_iq__qsout_rv = 0.0;

        let (assign21430_e20799, assign21430_e20799_d_n2, assign21430_e20799_d_n3, assign21430_e20799_d_n4, assign21430_e20799_d_n7, assign21430_e20799_d_n11, assign21430_e20799_d_n12,) = {
    if ((locals.var_guard240 != 0.0) && (locals.var_guard272 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn241_calc_iq__qsout, locals.var_fn241_calc_iq__qsout_dn2, locals.var_fn241_calc_iq__qsout_dn3, locals.var_fn241_calc_iq__qsout_dn4, locals.var_fn241_calc_iq__qsout_dn7, locals.var_fn241_calc_iq__qsout_dn11, locals.var_fn241_calc_iq__qsout_dn12,)
    }
};
        locals.var_fn241_calc_iq__qsout = assign21430_e20799;
        locals.var_fn241_calc_iq__qsout_dn2 = assign21430_e20799_d_n2;
        locals.var_fn241_calc_iq__qsout_dn3 = assign21430_e20799_d_n3;
        locals.var_fn241_calc_iq__qsout_dn4 = assign21430_e20799_d_n4;
        locals.var_fn241_calc_iq__qsout_dn7 = assign21430_e20799_d_n7;
        locals.var_fn241_calc_iq__qsout_dn11 = assign21430_e20799_d_n11;
        locals.var_fn241_calc_iq__qsout_dn12 = assign21430_e20799_d_n12;
        locals.var_fn241_calc_iq__qsout_rv = 0.0;

        let (assign21460_e20811, assign21460_e20811_d_n2, assign21460_e20811_d_n4, assign21460_e20811_d_n7, assign21460_e20811_d_n11, assign21460_e20811_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_fn241_calc_iq__qgsout, locals.var_fn241_calc_iq__qgsout_dn2, locals.var_fn241_calc_iq__qgsout_dn4, locals.var_fn241_calc_iq__qgsout_dn7, locals.var_fn241_calc_iq__qgsout_dn11, locals.var_fn241_calc_iq__qgsout_dn12,)
    } else {
        (locals.var_qgsfps3, locals.var_qgsfps3_dn2, locals.var_qgsfps3_dn4, locals.var_qgsfps3_dn7, locals.var_qgsfps3_dn11, locals.var_qgsfps3_dn12,)
    }
};
        locals.var_qgsfps3 = assign21460_e20811;
        locals.var_qgsfps3_dn2 = assign21460_e20811_d_n2;
        locals.var_qgsfps3_dn4 = assign21460_e20811_d_n4;
        locals.var_qgsfps3_dn7 = assign21460_e20811_d_n7;
        locals.var_qgsfps3_dn11 = assign21460_e20811_d_n11;
        locals.var_qgsfps3_dn12 = assign21460_e20811_d_n12;
        locals.var_qgsfps3_rv = 0.0;

        let (assign21470_e20815, assign21470_e20815_d_n2, assign21470_e20815_d_n4, assign21470_e20815_d_n7, assign21470_e20815_d_n11, assign21470_e20815_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_fn241_calc_iq__qgdout, locals.var_fn241_calc_iq__qgdout_dn2, locals.var_fn241_calc_iq__qgdout_dn4, locals.var_fn241_calc_iq__qgdout_dn7, locals.var_fn241_calc_iq__qgdout_dn11, locals.var_fn241_calc_iq__qgdout_dn12,)
    } else {
        (locals.var_qgdfps3, locals.var_qgdfps3_dn2, locals.var_qgdfps3_dn4, locals.var_qgdfps3_dn7, locals.var_qgdfps3_dn11, locals.var_qgdfps3_dn12,)
    }
};
        locals.var_qgdfps3 = assign21470_e20815;
        locals.var_qgdfps3_dn2 = assign21470_e20815_d_n2;
        locals.var_qgdfps3_dn4 = assign21470_e20815_d_n4;
        locals.var_qgdfps3_dn7 = assign21470_e20815_d_n7;
        locals.var_qgdfps3_dn11 = assign21470_e20815_d_n11;
        locals.var_qgdfps3_dn12 = assign21470_e20815_d_n12;
        locals.var_qgdfps3_rv = 0.0;

        let (assign21480_e20819, assign21480_e20819_d_n2, assign21480_e20819_d_n3, assign21480_e20819_d_n4, assign21480_e20819_d_n7, assign21480_e20819_d_n11, assign21480_e20819_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_fn241_calc_iq__qcout, locals.var_fn241_calc_iq__qcout_dn2, locals.var_fn241_calc_iq__qcout_dn3, locals.var_fn241_calc_iq__qcout_dn4, locals.var_fn241_calc_iq__qcout_dn7, locals.var_fn241_calc_iq__qcout_dn11, locals.var_fn241_calc_iq__qcout_dn12,)
    } else {
        (locals.var_qcfps3, locals.var_qcfps3_dn2, locals.var_qcfps3_dn3, locals.var_qcfps3_dn4, locals.var_qcfps3_dn7, locals.var_qcfps3_dn11, locals.var_qcfps3_dn12,)
    }
};
        locals.var_qcfps3 = assign21480_e20819;
        locals.var_qcfps3_dn2 = assign21480_e20819_d_n2;
        locals.var_qcfps3_dn3 = assign21480_e20819_d_n3;
        locals.var_qcfps3_dn4 = assign21480_e20819_d_n4;
        locals.var_qcfps3_dn7 = assign21480_e20819_d_n7;
        locals.var_qcfps3_dn11 = assign21480_e20819_d_n11;
        locals.var_qcfps3_dn12 = assign21480_e20819_d_n12;
        locals.var_qcfps3_rv = 0.0;

        let (assign21490_e20823, assign21490_e20823_d_n2, assign21490_e20823_d_n3, assign21490_e20823_d_n4, assign21490_e20823_d_n7, assign21490_e20823_d_n11, assign21490_e20823_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_fn241_calc_iq__qbout, locals.var_fn241_calc_iq__qbout_dn2, locals.var_fn241_calc_iq__qbout_dn3, locals.var_fn241_calc_iq__qbout_dn4, locals.var_fn241_calc_iq__qbout_dn7, locals.var_fn241_calc_iq__qbout_dn11, locals.var_fn241_calc_iq__qbout_dn12,)
    } else {
        (locals.var_qbfps3, locals.var_qbfps3_dn2, locals.var_qbfps3_dn3, locals.var_qbfps3_dn4, locals.var_qbfps3_dn7, locals.var_qbfps3_dn11, locals.var_qbfps3_dn12,)
    }
};
        locals.var_qbfps3 = assign21490_e20823;
        locals.var_qbfps3_dn2 = assign21490_e20823_d_n2;
        locals.var_qbfps3_dn3 = assign21490_e20823_d_n3;
        locals.var_qbfps3_dn4 = assign21490_e20823_d_n4;
        locals.var_qbfps3_dn7 = assign21490_e20823_d_n7;
        locals.var_qbfps3_dn11 = assign21490_e20823_d_n11;
        locals.var_qbfps3_dn12 = assign21490_e20823_d_n12;
        locals.var_qbfps3_rv = 0.0;

        let (assign21500_e20827, assign21500_e20827_d_n2, assign21500_e20827_d_n3, assign21500_e20827_d_n4, assign21500_e20827_d_n7, assign21500_e20827_d_n11, assign21500_e20827_d_n12,) = {
    if (locals.var_guard240 != 0.0) {
        (locals.var_fn241_calc_iq__qsout, locals.var_fn241_calc_iq__qsout_dn2, locals.var_fn241_calc_iq__qsout_dn3, locals.var_fn241_calc_iq__qsout_dn4, locals.var_fn241_calc_iq__qsout_dn7, locals.var_fn241_calc_iq__qsout_dn11, locals.var_fn241_calc_iq__qsout_dn12,)
    } else {
        (locals.var_qsfps3, locals.var_qsfps3_dn2, locals.var_qsfps3_dn3, locals.var_qsfps3_dn4, locals.var_qsfps3_dn7, locals.var_qsfps3_dn11, locals.var_qsfps3_dn12,)
    }
};
        locals.var_qsfps3 = assign21500_e20827;
        locals.var_qsfps3_dn2 = assign21500_e20827_d_n2;
        locals.var_qsfps3_dn3 = assign21500_e20827_d_n3;
        locals.var_qsfps3_dn4 = assign21500_e20827_d_n4;
        locals.var_qsfps3_dn7 = assign21500_e20827_d_n7;
        locals.var_qsfps3_dn11 = assign21500_e20827_d_n11;
        locals.var_qsfps3_dn12 = assign21500_e20827_d_n12;
        locals.var_qsfps3_rv = 0.0;

        let assign21540_e20842: f64 = if p.p122 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard275 = assign21540_e20842;
        locals.var_guard275_rv = 0.0;

        locals.var_qgsfps4 = 0.0;
        locals.var_qgsfps4_dn2 = 0.0;
        locals.var_qgsfps4_dn4 = 0.0;
        locals.var_qgsfps4_dn7 = 0.0;
        locals.var_qgsfps4_dn12 = 0.0;
        locals.var_qgsfps4_dn13 = 0.0;
        locals.var_qgsfps4_rv = 0.0;

        locals.var_qgdfps4 = 0.0;
        locals.var_qgdfps4_dn2 = 0.0;
        locals.var_qgdfps4_dn4 = 0.0;
        locals.var_qgdfps4_dn7 = 0.0;
        locals.var_qgdfps4_dn12 = 0.0;
        locals.var_qgdfps4_dn13 = 0.0;
        locals.var_qgdfps4_rv = 0.0;

        locals.var_qcfps4 = 0.0;
        locals.var_qcfps4_dn2 = 0.0;
        locals.var_qcfps4_dn3 = 0.0;
        locals.var_qcfps4_dn4 = 0.0;
        locals.var_qcfps4_dn7 = 0.0;
        locals.var_qcfps4_dn12 = 0.0;
        locals.var_qcfps4_dn13 = 0.0;
        locals.var_qcfps4_rv = 0.0;

        locals.var_qbfps4 = 0.0;
        locals.var_qbfps4_dn2 = 0.0;
        locals.var_qbfps4_dn3 = 0.0;
        locals.var_qbfps4_dn4 = 0.0;
        locals.var_qbfps4_dn7 = 0.0;
        locals.var_qbfps4_dn12 = 0.0;
        locals.var_qbfps4_dn13 = 0.0;
        locals.var_qbfps4_rv = 0.0;

        locals.var_qsfps4 = 0.0;
        locals.var_qsfps4_dn2 = 0.0;
        locals.var_qsfps4_dn3 = 0.0;
        locals.var_qsfps4_dn4 = 0.0;
        locals.var_qsfps4_dn7 = 0.0;
        locals.var_qsfps4_dn12 = 0.0;
        locals.var_qsfps4_dn13 = 0.0;
        locals.var_qsfps4_rv = 0.0;

        let assign21630_e20853: f64 = if p.p145 > p.p354 { 1.0 } else { 0.0 };
        locals.var_guard276 = assign21630_e20853;
        locals.var_guard276_rv = 0.0;

        let (assign21660_e20865, assign21660_e20865_d_n2, assign21660_e20865_d_n4, assign21660_e20865_d_n7, assign21660_e20865_d_n12, assign21660_e20865_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qgsout, locals.var_fn277_calc_iq__qgsout_dn2, locals.var_fn277_calc_iq__qgsout_dn4, locals.var_fn277_calc_iq__qgsout_dn7, locals.var_fn277_calc_iq__qgsout_dn12, locals.var_fn277_calc_iq__qgsout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qgsout = assign21660_e20865;
        locals.var_fn277_calc_iq__qgsout_dn2 = assign21660_e20865_d_n2;
        locals.var_fn277_calc_iq__qgsout_dn4 = assign21660_e20865_d_n4;
        locals.var_fn277_calc_iq__qgsout_dn7 = assign21660_e20865_d_n7;
        locals.var_fn277_calc_iq__qgsout_dn12 = assign21660_e20865_d_n12;
        locals.var_fn277_calc_iq__qgsout_dn13 = assign21660_e20865_d_n13;
        locals.var_fn277_calc_iq__qgsout_rv = 0.0;

        let (assign21670_e20869, assign21670_e20869_d_n2, assign21670_e20869_d_n4, assign21670_e20869_d_n7, assign21670_e20869_d_n12, assign21670_e20869_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qgdout, locals.var_fn277_calc_iq__qgdout_dn2, locals.var_fn277_calc_iq__qgdout_dn4, locals.var_fn277_calc_iq__qgdout_dn7, locals.var_fn277_calc_iq__qgdout_dn12, locals.var_fn277_calc_iq__qgdout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qgdout = assign21670_e20869;
        locals.var_fn277_calc_iq__qgdout_dn2 = assign21670_e20869_d_n2;
        locals.var_fn277_calc_iq__qgdout_dn4 = assign21670_e20869_d_n4;
        locals.var_fn277_calc_iq__qgdout_dn7 = assign21670_e20869_d_n7;
        locals.var_fn277_calc_iq__qgdout_dn12 = assign21670_e20869_d_n12;
        locals.var_fn277_calc_iq__qgdout_dn13 = assign21670_e20869_d_n13;
        locals.var_fn277_calc_iq__qgdout_rv = 0.0;

        let (assign21680_e20873, assign21680_e20873_d_n2, assign21680_e20873_d_n3, assign21680_e20873_d_n4, assign21680_e20873_d_n7, assign21680_e20873_d_n12, assign21680_e20873_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qcout, locals.var_fn277_calc_iq__qcout_dn2, locals.var_fn277_calc_iq__qcout_dn3, locals.var_fn277_calc_iq__qcout_dn4, locals.var_fn277_calc_iq__qcout_dn7, locals.var_fn277_calc_iq__qcout_dn12, locals.var_fn277_calc_iq__qcout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qcout = assign21680_e20873;
        locals.var_fn277_calc_iq__qcout_dn2 = assign21680_e20873_d_n2;
        locals.var_fn277_calc_iq__qcout_dn3 = assign21680_e20873_d_n3;
        locals.var_fn277_calc_iq__qcout_dn4 = assign21680_e20873_d_n4;
        locals.var_fn277_calc_iq__qcout_dn7 = assign21680_e20873_d_n7;
        locals.var_fn277_calc_iq__qcout_dn12 = assign21680_e20873_d_n12;
        locals.var_fn277_calc_iq__qcout_dn13 = assign21680_e20873_d_n13;
        locals.var_fn277_calc_iq__qcout_rv = 0.0;

        let (assign21690_e20877, assign21690_e20877_d_n2, assign21690_e20877_d_n3, assign21690_e20877_d_n4, assign21690_e20877_d_n7, assign21690_e20877_d_n12, assign21690_e20877_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qbout, locals.var_fn277_calc_iq__qbout_dn2, locals.var_fn277_calc_iq__qbout_dn3, locals.var_fn277_calc_iq__qbout_dn4, locals.var_fn277_calc_iq__qbout_dn7, locals.var_fn277_calc_iq__qbout_dn12, locals.var_fn277_calc_iq__qbout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qbout = assign21690_e20877;
        locals.var_fn277_calc_iq__qbout_dn2 = assign21690_e20877_d_n2;
        locals.var_fn277_calc_iq__qbout_dn3 = assign21690_e20877_d_n3;
        locals.var_fn277_calc_iq__qbout_dn4 = assign21690_e20877_d_n4;
        locals.var_fn277_calc_iq__qbout_dn7 = assign21690_e20877_d_n7;
        locals.var_fn277_calc_iq__qbout_dn12 = assign21690_e20877_d_n12;
        locals.var_fn277_calc_iq__qbout_dn13 = assign21690_e20877_d_n13;
        locals.var_fn277_calc_iq__qbout_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_59(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign21700_e20881, assign21700_e20881_d_n2, assign21700_e20881_d_n3, assign21700_e20881_d_n4, assign21700_e20881_d_n7, assign21700_e20881_d_n12, assign21700_e20881_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qsout, locals.var_fn277_calc_iq__qsout_dn2, locals.var_fn277_calc_iq__qsout_dn3, locals.var_fn277_calc_iq__qsout_dn4, locals.var_fn277_calc_iq__qsout_dn7, locals.var_fn277_calc_iq__qsout_dn12, locals.var_fn277_calc_iq__qsout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qsout = assign21700_e20881;
        locals.var_fn277_calc_iq__qsout_dn2 = assign21700_e20881_d_n2;
        locals.var_fn277_calc_iq__qsout_dn3 = assign21700_e20881_d_n3;
        locals.var_fn277_calc_iq__qsout_dn4 = assign21700_e20881_d_n4;
        locals.var_fn277_calc_iq__qsout_dn7 = assign21700_e20881_d_n7;
        locals.var_fn277_calc_iq__qsout_dn12 = assign21700_e20881_d_n12;
        locals.var_fn277_calc_iq__qsout_dn13 = assign21700_e20881_d_n13;
        locals.var_fn277_calc_iq__qsout_rv = 0.0;

        let (assign21710_e20885, assign21710_e20885_d_n4, assign21710_e20885_d_n12, assign21710_e20885_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vtdibl, locals.var_fn277_calc_iq__vtdibl_dn4, locals.var_fn277_calc_iq__vtdibl_dn12, locals.var_fn277_calc_iq__vtdibl_dn13,)
    }
};
        locals.var_fn277_calc_iq__vtdibl = assign21710_e20885;
        locals.var_fn277_calc_iq__vtdibl_dn4 = assign21710_e20885_d_n4;
        locals.var_fn277_calc_iq__vtdibl_dn12 = assign21710_e20885_d_n12;
        locals.var_fn277_calc_iq__vtdibl_dn13 = assign21710_e20885_d_n13;
        locals.var_fn277_calc_iq__vtdibl_rv = 0.0;

        let (assign21720_e20889, assign21720_e20889_d_n2, assign21720_e20889_d_n3, assign21720_e20889_d_n4, assign21720_e20889_d_n7, assign21720_e20889_d_n12, assign21720_e20889_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsat1, locals.var_fn277_calc_iq__vdsat1_dn2, locals.var_fn277_calc_iq__vdsat1_dn3, locals.var_fn277_calc_iq__vdsat1_dn4, locals.var_fn277_calc_iq__vdsat1_dn7, locals.var_fn277_calc_iq__vdsat1_dn12, locals.var_fn277_calc_iq__vdsat1_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsat1 = assign21720_e20889;
        locals.var_fn277_calc_iq__vdsat1_dn2 = assign21720_e20889_d_n2;
        locals.var_fn277_calc_iq__vdsat1_dn3 = assign21720_e20889_d_n3;
        locals.var_fn277_calc_iq__vdsat1_dn4 = assign21720_e20889_d_n4;
        locals.var_fn277_calc_iq__vdsat1_dn7 = assign21720_e20889_d_n7;
        locals.var_fn277_calc_iq__vdsat1_dn12 = assign21720_e20889_d_n12;
        locals.var_fn277_calc_iq__vdsat1_dn13 = assign21720_e20889_d_n13;
        locals.var_fn277_calc_iq__vdsat1_rv = 0.0;

        let (assign21730_e20893, assign21730_e20893_d_n2, assign21730_e20893_d_n7, assign21730_e20893_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_vgsfps4, locals.var_vgsfps4_dn2, locals.var_vgsfps4_dn7, locals.var_vgsfps4_dn13,)
    } else {
        (locals.var_fn277_calc_iq__vgsin, locals.var_fn277_calc_iq__vgsin_dn2, locals.var_fn277_calc_iq__vgsin_dn7, locals.var_fn277_calc_iq__vgsin_dn13,)
    }
};
        locals.var_fn277_calc_iq__vgsin = assign21730_e20893;
        locals.var_fn277_calc_iq__vgsin_dn2 = assign21730_e20893_d_n2;
        locals.var_fn277_calc_iq__vgsin_dn7 = assign21730_e20893_d_n7;
        locals.var_fn277_calc_iq__vgsin_dn13 = assign21730_e20893_d_n13;
        locals.var_fn277_calc_iq__vgsin_rv = 0.0;

        let (assign21740_e20897, assign21740_e20897_d_n12, assign21740_e20897_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_vdsfps4, locals.var_vdsfps4_dn12, locals.var_vdsfps4_dn13,)
    } else {
        (locals.var_fn277_calc_iq__vdsin, locals.var_fn277_calc_iq__vdsin_dn12, locals.var_fn277_calc_iq__vdsin_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsin = assign21740_e20897;
        locals.var_fn277_calc_iq__vdsin_dn12 = assign21740_e20897_d_n12;
        locals.var_fn277_calc_iq__vdsin_dn13 = assign21740_e20897_d_n13;
        locals.var_fn277_calc_iq__vdsin_rv = 0.0;

        let (assign21750_e20901,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p151,)
    } else {
        (locals.var_fn277_calc_iq__qcbflag,)
    }
};
        locals.var_fn277_calc_iq__qcbflag = assign21750_e20901;
        locals.var_fn277_calc_iq__qcbflag_rv = 0.0;

        let (assign21760_e20905, assign21760_e20905_d_n2, assign21760_e20905_d_n7, assign21760_e20905_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_vcfps4, locals.var_vcfps4_dn2, locals.var_vcfps4_dn7, locals.var_vcfps4_dn13,)
    } else {
        (locals.var_fn277_calc_iq__vcin, locals.var_fn277_calc_iq__vcin_dn2, locals.var_fn277_calc_iq__vcin_dn7, locals.var_fn277_calc_iq__vcin_dn13,)
    }
};
        locals.var_fn277_calc_iq__vcin = assign21760_e20905;
        locals.var_fn277_calc_iq__vcin_dn2 = assign21760_e20905_d_n2;
        locals.var_fn277_calc_iq__vcin_dn7 = assign21760_e20905_d_n7;
        locals.var_fn277_calc_iq__vcin_dn13 = assign21760_e20905_d_n13;
        locals.var_fn277_calc_iq__vcin_rv = 0.0;

        let (assign21770_e20909, assign21770_e20909_d_n3, assign21770_e20909_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_vbfps4, locals.var_vbfps4_dn3, locals.var_vbfps4_dn13,)
    } else {
        (locals.var_fn277_calc_iq__vbin, locals.var_fn277_calc_iq__vbin_dn3, locals.var_fn277_calc_iq__vbin_dn13,)
    }
};
        locals.var_fn277_calc_iq__vbin = assign21770_e20909;
        locals.var_fn277_calc_iq__vbin_dn3 = assign21770_e20909_d_n3;
        locals.var_fn277_calc_iq__vbin_dn13 = assign21770_e20909_d_n13;
        locals.var_fn277_calc_iq__vbin_rv = 0.0;

        let (assign21780_e20913,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p149,)
    } else {
        (locals.var_fn277_calc_iq__qgsflag,)
    }
};
        locals.var_fn277_calc_iq__qgsflag = assign21780_e20913;
        locals.var_fn277_calc_iq__qgsflag_rv = 0.0;

        let (assign21790_e20917, assign21790_e20917_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_tdut, locals.var_tdut_dn4,)
    } else {
        (locals.var_fn277_calc_iq__tambin, locals.var_fn277_calc_iq__tambin_dn4,)
    }
};
        locals.var_fn277_calc_iq__tambin = assign21790_e20917;
        locals.var_fn277_calc_iq__tambin_dn4 = assign21790_e20917_d_n4;
        locals.var_fn277_calc_iq__tambin_rv = 0.0;

        let (assign21800_e20921,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_tnomk,)
    } else {
        (locals.var_fn277_calc_iq__tnomin,)
    }
};
        locals.var_fn277_calc_iq__tnomin = assign21800_e20921;
        locals.var_fn277_calc_iq__tnomin_rv = 0.0;

        let (assign21810_e20925, assign21810_e20925_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_phit, locals.var_phit_dn4,)
    } else {
        (locals.var_fn277_calc_iq__phitin, locals.var_fn277_calc_iq__phitin_dn4,)
    }
};
        locals.var_fn277_calc_iq__phitin = assign21810_e20925;
        locals.var_fn277_calc_iq__phitin_dn4 = assign21810_e20925_d_n4;
        locals.var_fn277_calc_iq__phitin_rv = 0.0;

        let (assign21820_e20929,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p0,)
    } else {
        (locals.var_fn277_calc_iq__w,)
    }
};
        locals.var_fn277_calc_iq__w = assign21820_e20929;
        locals.var_fn277_calc_iq__w_rv = 0.0;

        let (assign21830_e20933,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p145,)
    } else {
        (locals.var_fn277_calc_iq__lin,)
    }
};
        locals.var_fn277_calc_iq__lin = assign21830_e20933;
        locals.var_fn277_calc_iq__lin_rv = 0.0;

        let (assign21840_e20937, assign21840_e20937_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_cgfps4t, locals.var_cgfps4t_dn4,)
    } else {
        (locals.var_fn277_calc_iq__cgin, locals.var_fn277_calc_iq__cgin_dn4,)
    }
};
        locals.var_fn277_calc_iq__cgin = assign21840_e20937;
        locals.var_fn277_calc_iq__cgin_dn4 = assign21840_e20937_d_n4;
        locals.var_fn277_calc_iq__cgin_rv = 0.0;

        let (assign21850_e20941,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p150,)
    } else {
        (locals.var_fn277_calc_iq__cs,)
    }
};
        locals.var_fn277_calc_iq__cs = assign21850_e20941;
        locals.var_fn277_calc_iq__cs_rv = 0.0;

        let (assign21860_e20945, assign21860_e20945_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_ccfps4t, locals.var_ccfps4t_dn4,)
    } else {
        (locals.var_fn277_calc_iq__cc, locals.var_fn277_calc_iq__cc_dn4,)
    }
};
        locals.var_fn277_calc_iq__cc = assign21860_e20945;
        locals.var_fn277_calc_iq__cc_dn4 = assign21860_e20945_d_n4;
        locals.var_fn277_calc_iq__cc_rv = 0.0;

        let (assign21870_e20949, assign21870_e20949_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_cbfps4t, locals.var_cbfps4t_dn4,)
    } else {
        (locals.var_fn277_calc_iq__cb, locals.var_fn277_calc_iq__cb_dn4,)
    }
};
        locals.var_fn277_calc_iq__cb = assign21870_e20949;
        locals.var_fn277_calc_iq__cb_dn4 = assign21870_e20949_d_n4;
        locals.var_fn277_calc_iq__cb_rv = 0.0;

        let (assign21880_e20953,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p146,)
    } else {
        (locals.var_fn277_calc_iq__vto,)
    }
};
        locals.var_fn277_calc_iq__vto = assign21880_e20953;
        locals.var_fn277_calc_iq__vto_rv = 0.0;

        let (assign21890_e20957,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p160,)
    } else {
        (locals.var_fn277_calc_iq__ss,)
    }
};
        locals.var_fn277_calc_iq__ss = assign21890_e20957;
        locals.var_fn277_calc_iq__ss_rv = 0.0;

        let (assign21900_e20961,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p159,)
    } else {
        (locals.var_fn277_calc_iq__delta1,)
    }
};
        locals.var_fn277_calc_iq__delta1 = assign21900_e20961;
        locals.var_fn277_calc_iq__delta1_rv = 0.0;

        let (assign21910_e20965,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0,)
    } else {
        (locals.var_fn277_calc_iq__delta2,)
    }
};
        locals.var_fn277_calc_iq__delta2 = assign21910_e20965;
        locals.var_fn277_calc_iq__delta2_rv = 0.0;

        let (assign21920_e20969,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p161,)
    } else {
        (locals.var_fn277_calc_iq__nd,)
    }
};
        locals.var_fn277_calc_iq__nd = assign21920_e20969;
        locals.var_fn277_calc_iq__nd_rv = 0.0;

        let (assign21930_e20973,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p165,)
    } else {
        (locals.var_fn277_calc_iq__alpha,)
    }
};
        locals.var_fn277_calc_iq__alpha = assign21930_e20973;
        locals.var_fn277_calc_iq__alpha_rv = 0.0;

        let (assign21940_e20977,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p156,)
    } else {
        (locals.var_fn277_calc_iq__vel0,)
    }
};
        locals.var_fn277_calc_iq__vel0 = assign21940_e20977;
        locals.var_fn277_calc_iq__vel0_rv = 0.0;

        let (assign21950_e20981,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p157,)
    } else {
        (locals.var_fn277_calc_iq__mu0,)
    }
};
        locals.var_fn277_calc_iq__mu0 = assign21950_e20981;
        locals.var_fn277_calc_iq__mu0_rv = 0.0;

        let (assign21960_e20985,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p158,)
    } else {
        (locals.var_fn277_calc_iq__beta,)
    }
};
        locals.var_fn277_calc_iq__beta = assign21960_e20985;
        locals.var_fn277_calc_iq__beta_rv = 0.0;

        let (assign21970_e20989,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p164,)
    } else {
        (locals.var_fn277_calc_iq__mtheta,)
    }
};
        locals.var_fn277_calc_iq__mtheta = assign21970_e20989;
        locals.var_fn277_calc_iq__mtheta_rv = 0.0;

        let (assign21980_e20993,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p163,)
    } else {
        (locals.var_fn277_calc_iq__vtheta,)
    }
};
        locals.var_fn277_calc_iq__vtheta = assign21980_e20993;
        locals.var_fn277_calc_iq__vtheta_rv = 0.0;

        let (assign21990_e20997,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p162,)
    } else {
        (locals.var_fn277_calc_iq__vtzeta,)
    }
};
        locals.var_fn277_calc_iq__vtzeta = assign21990_e20997;
        locals.var_fn277_calc_iq__vtzeta_rv = 0.0;

        let (assign22000_e21001,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p39,)
    } else {
        (locals.var_fn277_calc_iq__dibsat,)
    }
};
        locals.var_fn277_calc_iq__dibsat = assign22000_e21001;
        locals.var_fn277_calc_iq__dibsat_rv = 0.0;

        let (assign22010_e21005,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p47,)
    } else {
        (locals.var_fn277_calc_iq__epsilon,)
    }
};
        locals.var_fn277_calc_iq__epsilon = assign22010_e21005;
        locals.var_fn277_calc_iq__epsilon_rv = 0.0;

        let (assign22020_e21009,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p45,)
    } else {
        (locals.var_fn277_calc_iq__vzeta,)
    }
};
        locals.var_fn277_calc_iq__vzeta = assign22020_e21009;
        locals.var_fn277_calc_iq__vzeta_rv = 0.0;

        let (assign22030_e21013,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p42,)
    } else {
        (locals.var_fn277_calc_iq__lambda,)
    }
};
        locals.var_fn277_calc_iq__lambda = assign22030_e21013;
        locals.var_fn277_calc_iq__lambda_rv = 0.0;

        let (assign22040_e21017,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p2,)
    } else {
        (locals.var_fn277_calc_iq__ngf,)
    }
};
        locals.var_fn277_calc_iq__ngf = assign22040_e21017;
        locals.var_fn277_calc_iq__ngf_rv = 0.0;

        let (assign22050_e21021,) = {
    if (locals.var_guard276 != 0.0) {
        (p.p6,)
    } else {
        (locals.var_fn277_calc_iq__type,)
    }
};
        locals.var_fn277_calc_iq__type = assign22050_e21021;
        locals.var_fn277_calc_iq__type_rv = 0.0;

        let (assign22060_e21025,) = {
    if (locals.var_guard276 != 0.0) {
        (1.0,)
    } else {
        (locals.var_fn277_calc_iq__trapfracdl,)
    }
};
        locals.var_fn277_calc_iq__trapfracdl = assign22060_e21025;
        locals.var_fn277_calc_iq__trapfracdl_rv = 0.0;

        let (assign22070_e21029, assign22070_e21029_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__alpha_phit, locals.var_fn277_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn277_calc_iq__alpha_phit = assign22070_e21029;
        locals.var_fn277_calc_iq__alpha_phit_dn4 = assign22070_e21029_d_n4;
        locals.var_fn277_calc_iq__alpha_phit_rv = 0.0;

        let (assign22080_e21033, assign22080_e21033_d_n12, assign22080_e21033_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__delta, locals.var_fn277_calc_iq__delta_dn12, locals.var_fn277_calc_iq__delta_dn13,)
    }
};
        locals.var_fn277_calc_iq__delta = assign22080_e21033;
        locals.var_fn277_calc_iq__delta_dn12 = assign22080_e21033_d_n12;
        locals.var_fn277_calc_iq__delta_dn13 = assign22080_e21033_d_n13;
        locals.var_fn277_calc_iq__delta_rv = 0.0;

        let (assign22090_e21037, assign22090_e21037_d_n4, assign22090_e21037_d_n12, assign22090_e21037_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__n, locals.var_fn277_calc_iq__n_dn4, locals.var_fn277_calc_iq__n_dn12, locals.var_fn277_calc_iq__n_dn13,)
    }
};
        locals.var_fn277_calc_iq__n = assign22090_e21037;
        locals.var_fn277_calc_iq__n_dn4 = assign22090_e21037_d_n4;
        locals.var_fn277_calc_iq__n_dn12 = assign22090_e21037_d_n12;
        locals.var_fn277_calc_iq__n_dn13 = assign22090_e21037_d_n13;
        locals.var_fn277_calc_iq__n_rv = 0.0;

        let (assign22100_e21041, assign22100_e21041_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vtof, locals.var_fn277_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn277_calc_iq__vtof = assign22100_e21041;
        locals.var_fn277_calc_iq__vtof_dn4 = assign22100_e21041_d_n4;
        locals.var_fn277_calc_iq__vtof_rv = 0.0;

        let (assign22110_e21045, assign22110_e21045_d_n12, assign22110_e21045_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vsatdibl, locals.var_fn277_calc_iq__vsatdibl_dn12, locals.var_fn277_calc_iq__vsatdibl_dn13,)
    }
};
        locals.var_fn277_calc_iq__vsatdibl = assign22110_e21045;
        locals.var_fn277_calc_iq__vsatdibl_dn12 = assign22110_e21045_d_n12;
        locals.var_fn277_calc_iq__vsatdibl_dn13 = assign22110_e21045_d_n13;
        locals.var_fn277_calc_iq__vsatdibl_rv = 0.0;

        let (assign22120_e21049, assign22120_e21049_d_n2, assign22120_e21049_d_n3, assign22120_e21049_d_n4, assign22120_e21049_d_n7, assign22120_e21049_d_n12, assign22120_e21049_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffs, locals.var_fn277_calc_iq__ffs_dn2, locals.var_fn277_calc_iq__ffs_dn3, locals.var_fn277_calc_iq__ffs_dn4, locals.var_fn277_calc_iq__ffs_dn7, locals.var_fn277_calc_iq__ffs_dn12, locals.var_fn277_calc_iq__ffs_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffs = assign22120_e21049;
        locals.var_fn277_calc_iq__ffs_dn2 = assign22120_e21049_d_n2;
        locals.var_fn277_calc_iq__ffs_dn3 = assign22120_e21049_d_n3;
        locals.var_fn277_calc_iq__ffs_dn4 = assign22120_e21049_d_n4;
        locals.var_fn277_calc_iq__ffs_dn7 = assign22120_e21049_d_n7;
        locals.var_fn277_calc_iq__ffs_dn12 = assign22120_e21049_d_n12;
        locals.var_fn277_calc_iq__ffs_dn13 = assign22120_e21049_d_n13;
        locals.var_fn277_calc_iq__ffs_rv = 0.0;

        let (assign22130_e21053, assign22130_e21053_d_n4, assign22130_e21053_d_n12, assign22130_e21053_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__two_n_phit, locals.var_fn277_calc_iq__two_n_phit_dn4, locals.var_fn277_calc_iq__two_n_phit_dn12, locals.var_fn277_calc_iq__two_n_phit_dn13,)
    }
};
        locals.var_fn277_calc_iq__two_n_phit = assign22130_e21053;
        locals.var_fn277_calc_iq__two_n_phit_dn4 = assign22130_e21053_d_n4;
        locals.var_fn277_calc_iq__two_n_phit_dn12 = assign22130_e21053_d_n12;
        locals.var_fn277_calc_iq__two_n_phit_dn13 = assign22130_e21053_d_n13;
        locals.var_fn277_calc_iq__two_n_phit_rv = 0.0;

        let (assign22140_e21057, assign22140_e21057_d_n4, assign22140_e21057_d_n12, assign22140_e21057_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qref, locals.var_fn277_calc_iq__qref_dn4, locals.var_fn277_calc_iq__qref_dn12, locals.var_fn277_calc_iq__qref_dn13,)
    }
};
        locals.var_fn277_calc_iq__qref = assign22140_e21057;
        locals.var_fn277_calc_iq__qref_dn4 = assign22140_e21057_d_n4;
        locals.var_fn277_calc_iq__qref_dn12 = assign22140_e21057_d_n12;
        locals.var_fn277_calc_iq__qref_dn13 = assign22140_e21057_d_n13;
        locals.var_fn277_calc_iq__qref_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_60(
        locals: &mut StampLocals,
    ) {
        let (assign22150_e21061, assign22150_e21061_d_n2, assign22150_e21061_d_n3, assign22150_e21061_d_n4, assign22150_e21061_d_n7, assign22150_e21061_d_n12, assign22150_e21061_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__etas, locals.var_fn277_calc_iq__etas_dn2, locals.var_fn277_calc_iq__etas_dn3, locals.var_fn277_calc_iq__etas_dn4, locals.var_fn277_calc_iq__etas_dn7, locals.var_fn277_calc_iq__etas_dn12, locals.var_fn277_calc_iq__etas_dn13,)
    }
};
        locals.var_fn277_calc_iq__etas = assign22150_e21061;
        locals.var_fn277_calc_iq__etas_dn2 = assign22150_e21061_d_n2;
        locals.var_fn277_calc_iq__etas_dn3 = assign22150_e21061_d_n3;
        locals.var_fn277_calc_iq__etas_dn4 = assign22150_e21061_d_n4;
        locals.var_fn277_calc_iq__etas_dn7 = assign22150_e21061_d_n7;
        locals.var_fn277_calc_iq__etas_dn12 = assign22150_e21061_d_n12;
        locals.var_fn277_calc_iq__etas_dn13 = assign22150_e21061_d_n13;
        locals.var_fn277_calc_iq__etas_rv = 0.0;

        let (assign22160_e21065, assign22160_e21065_d_n2, assign22160_e21065_d_n3, assign22160_e21065_d_n4, assign22160_e21065_d_n7, assign22160_e21065_d_n12, assign22160_e21065_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qinvs, locals.var_fn277_calc_iq__qinvs_dn2, locals.var_fn277_calc_iq__qinvs_dn3, locals.var_fn277_calc_iq__qinvs_dn4, locals.var_fn277_calc_iq__qinvs_dn7, locals.var_fn277_calc_iq__qinvs_dn12, locals.var_fn277_calc_iq__qinvs_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvs = assign22160_e21065;
        locals.var_fn277_calc_iq__qinvs_dn2 = assign22160_e21065_d_n2;
        locals.var_fn277_calc_iq__qinvs_dn3 = assign22160_e21065_d_n3;
        locals.var_fn277_calc_iq__qinvs_dn4 = assign22160_e21065_d_n4;
        locals.var_fn277_calc_iq__qinvs_dn7 = assign22160_e21065_d_n7;
        locals.var_fn277_calc_iq__qinvs_dn12 = assign22160_e21065_d_n12;
        locals.var_fn277_calc_iq__qinvs_dn13 = assign22160_e21065_d_n13;
        locals.var_fn277_calc_iq__qinvs_rv = 0.0;

        let (assign22170_e21069, assign22170_e21069_d_n2, assign22170_e21069_d_n3, assign22170_e21069_d_n4, assign22170_e21069_d_n7, assign22170_e21069_d_n12, assign22170_e21069_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__muf, locals.var_fn277_calc_iq__muf_dn2, locals.var_fn277_calc_iq__muf_dn3, locals.var_fn277_calc_iq__muf_dn4, locals.var_fn277_calc_iq__muf_dn7, locals.var_fn277_calc_iq__muf_dn12, locals.var_fn277_calc_iq__muf_dn13,)
    }
};
        locals.var_fn277_calc_iq__muf = assign22170_e21069;
        locals.var_fn277_calc_iq__muf_dn2 = assign22170_e21069_d_n2;
        locals.var_fn277_calc_iq__muf_dn3 = assign22170_e21069_d_n3;
        locals.var_fn277_calc_iq__muf_dn4 = assign22170_e21069_d_n4;
        locals.var_fn277_calc_iq__muf_dn7 = assign22170_e21069_d_n7;
        locals.var_fn277_calc_iq__muf_dn12 = assign22170_e21069_d_n12;
        locals.var_fn277_calc_iq__muf_dn13 = assign22170_e21069_d_n13;
        locals.var_fn277_calc_iq__muf_rv = 0.0;

        let (assign22180_e21073, assign22180_e21073_d_n2, assign22180_e21073_d_n3, assign22180_e21073_d_n4, assign22180_e21073_d_n7, assign22180_e21073_d_n12, assign22180_e21073_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vx, locals.var_fn277_calc_iq__vx_dn2, locals.var_fn277_calc_iq__vx_dn3, locals.var_fn277_calc_iq__vx_dn4, locals.var_fn277_calc_iq__vx_dn7, locals.var_fn277_calc_iq__vx_dn12, locals.var_fn277_calc_iq__vx_dn13,)
    }
};
        locals.var_fn277_calc_iq__vx = assign22180_e21073;
        locals.var_fn277_calc_iq__vx_dn2 = assign22180_e21073_d_n2;
        locals.var_fn277_calc_iq__vx_dn3 = assign22180_e21073_d_n3;
        locals.var_fn277_calc_iq__vx_dn4 = assign22180_e21073_d_n4;
        locals.var_fn277_calc_iq__vx_dn7 = assign22180_e21073_d_n7;
        locals.var_fn277_calc_iq__vx_dn12 = assign22180_e21073_d_n12;
        locals.var_fn277_calc_iq__vx_dn13 = assign22180_e21073_d_n13;
        locals.var_fn277_calc_iq__vx_rv = 0.0;

        let (assign22200_e21081, assign22200_e21081_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__n0, locals.var_fn277_calc_iq__n0_dn4,)
    }
};
        locals.var_fn277_calc_iq__n0 = assign22200_e21081;
        locals.var_fn277_calc_iq__n0_dn4 = assign22200_e21081_d_n4;
        locals.var_fn277_calc_iq__n0_rv = 0.0;

        let (assign22210_e21085, assign22210_e21085_d_n2, assign22210_e21085_d_n4, assign22210_e21085_d_n7, assign22210_e21085_d_n12, assign22210_e21085_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffs0, locals.var_fn277_calc_iq__ffs0_dn2, locals.var_fn277_calc_iq__ffs0_dn4, locals.var_fn277_calc_iq__ffs0_dn7, locals.var_fn277_calc_iq__ffs0_dn12, locals.var_fn277_calc_iq__ffs0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffs0 = assign22210_e21085;
        locals.var_fn277_calc_iq__ffs0_dn2 = assign22210_e21085_d_n2;
        locals.var_fn277_calc_iq__ffs0_dn4 = assign22210_e21085_d_n4;
        locals.var_fn277_calc_iq__ffs0_dn7 = assign22210_e21085_d_n7;
        locals.var_fn277_calc_iq__ffs0_dn12 = assign22210_e21085_d_n12;
        locals.var_fn277_calc_iq__ffs0_dn13 = assign22210_e21085_d_n13;
        locals.var_fn277_calc_iq__ffs0_rv = 0.0;

        let (assign22220_e21089, assign22220_e21089_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__two_n_phit0, locals.var_fn277_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn277_calc_iq__two_n_phit0 = assign22220_e21089;
        locals.var_fn277_calc_iq__two_n_phit0_dn4 = assign22220_e21089_d_n4;
        locals.var_fn277_calc_iq__two_n_phit0_rv = 0.0;

        let (assign22230_e21093, assign22230_e21093_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qref0, locals.var_fn277_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn277_calc_iq__qref0 = assign22230_e21093;
        locals.var_fn277_calc_iq__qref0_dn4 = assign22230_e21093_d_n4;
        locals.var_fn277_calc_iq__qref0_rv = 0.0;

        let (assign22240_e21097, assign22240_e21097_d_n2, assign22240_e21097_d_n4, assign22240_e21097_d_n7, assign22240_e21097_d_n12, assign22240_e21097_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__etas0, locals.var_fn277_calc_iq__etas0_dn2, locals.var_fn277_calc_iq__etas0_dn4, locals.var_fn277_calc_iq__etas0_dn7, locals.var_fn277_calc_iq__etas0_dn12, locals.var_fn277_calc_iq__etas0_dn13,)
    }
};
        locals.var_fn277_calc_iq__etas0 = assign22240_e21097;
        locals.var_fn277_calc_iq__etas0_dn2 = assign22240_e21097_d_n2;
        locals.var_fn277_calc_iq__etas0_dn4 = assign22240_e21097_d_n4;
        locals.var_fn277_calc_iq__etas0_dn7 = assign22240_e21097_d_n7;
        locals.var_fn277_calc_iq__etas0_dn12 = assign22240_e21097_d_n12;
        locals.var_fn277_calc_iq__etas0_dn13 = assign22240_e21097_d_n13;
        locals.var_fn277_calc_iq__etas0_rv = 0.0;

        let (assign22250_e21101, assign22250_e21101_d_n2, assign22250_e21101_d_n4, assign22250_e21101_d_n7, assign22250_e21101_d_n12, assign22250_e21101_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qinvs0, locals.var_fn277_calc_iq__qinvs0_dn2, locals.var_fn277_calc_iq__qinvs0_dn4, locals.var_fn277_calc_iq__qinvs0_dn7, locals.var_fn277_calc_iq__qinvs0_dn12, locals.var_fn277_calc_iq__qinvs0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvs0 = assign22250_e21101;
        locals.var_fn277_calc_iq__qinvs0_dn2 = assign22250_e21101_d_n2;
        locals.var_fn277_calc_iq__qinvs0_dn4 = assign22250_e21101_d_n4;
        locals.var_fn277_calc_iq__qinvs0_dn7 = assign22250_e21101_d_n7;
        locals.var_fn277_calc_iq__qinvs0_dn12 = assign22250_e21101_d_n12;
        locals.var_fn277_calc_iq__qinvs0_dn13 = assign22250_e21101_d_n13;
        locals.var_fn277_calc_iq__qinvs0_rv = 0.0;

        let (assign22260_e21105, assign22260_e21105_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__muf0, locals.var_fn277_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn277_calc_iq__muf0 = assign22260_e21105;
        locals.var_fn277_calc_iq__muf0_dn4 = assign22260_e21105_d_n4;
        locals.var_fn277_calc_iq__muf0_rv = 0.0;

        let (assign22270_e21109, assign22270_e21109_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vx0, locals.var_fn277_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn277_calc_iq__vx0 = assign22270_e21109;
        locals.var_fn277_calc_iq__vx0_dn4 = assign22270_e21109_d_n4;
        locals.var_fn277_calc_iq__vx0_rv = 0.0;

        let (assign22280_e21113, assign22280_e21113_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__tfacmobin, locals.var_fn277_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn277_calc_iq__tfacmobin = assign22280_e21113;
        locals.var_fn277_calc_iq__tfacmobin_dn4 = assign22280_e21113_d_n4;
        locals.var_fn277_calc_iq__tfacmobin_rv = 0.0;

        let (assign22290_e21117, assign22290_e21117_d_n2, assign22290_e21117_d_n3, assign22290_e21117_d_n4, assign22290_e21117_d_n7, assign22290_e21117_d_n12, assign22290_e21117_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ff, locals.var_fn277_calc_iq__ff_dn2, locals.var_fn277_calc_iq__ff_dn3, locals.var_fn277_calc_iq__ff_dn4, locals.var_fn277_calc_iq__ff_dn7, locals.var_fn277_calc_iq__ff_dn12, locals.var_fn277_calc_iq__ff_dn13,)
    }
};
        locals.var_fn277_calc_iq__ff = assign22290_e21117;
        locals.var_fn277_calc_iq__ff_dn2 = assign22290_e21117_d_n2;
        locals.var_fn277_calc_iq__ff_dn3 = assign22290_e21117_d_n3;
        locals.var_fn277_calc_iq__ff_dn4 = assign22290_e21117_d_n4;
        locals.var_fn277_calc_iq__ff_dn7 = assign22290_e21117_d_n7;
        locals.var_fn277_calc_iq__ff_dn12 = assign22290_e21117_d_n12;
        locals.var_fn277_calc_iq__ff_dn13 = assign22290_e21117_d_n13;
        locals.var_fn277_calc_iq__ff_rv = 0.0;

        let (assign22300_e21121, assign22300_e21121_d_n2, assign22300_e21121_d_n3, assign22300_e21121_d_n4, assign22300_e21121_d_n7, assign22300_e21121_d_n12, assign22300_e21121_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__eta, locals.var_fn277_calc_iq__eta_dn2, locals.var_fn277_calc_iq__eta_dn3, locals.var_fn277_calc_iq__eta_dn4, locals.var_fn277_calc_iq__eta_dn7, locals.var_fn277_calc_iq__eta_dn12, locals.var_fn277_calc_iq__eta_dn13,)
    }
};
        locals.var_fn277_calc_iq__eta = assign22300_e21121;
        locals.var_fn277_calc_iq__eta_dn2 = assign22300_e21121_d_n2;
        locals.var_fn277_calc_iq__eta_dn3 = assign22300_e21121_d_n3;
        locals.var_fn277_calc_iq__eta_dn4 = assign22300_e21121_d_n4;
        locals.var_fn277_calc_iq__eta_dn7 = assign22300_e21121_d_n7;
        locals.var_fn277_calc_iq__eta_dn12 = assign22300_e21121_d_n12;
        locals.var_fn277_calc_iq__eta_dn13 = assign22300_e21121_d_n13;
        locals.var_fn277_calc_iq__eta_rv = 0.0;

        let (assign22310_e21125, assign22310_e21125_d_n2, assign22310_e21125_d_n3, assign22310_e21125_d_n4, assign22310_e21125_d_n7, assign22310_e21125_d_n12, assign22310_e21125_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qinvv, locals.var_fn277_calc_iq__qinvv_dn2, locals.var_fn277_calc_iq__qinvv_dn3, locals.var_fn277_calc_iq__qinvv_dn4, locals.var_fn277_calc_iq__qinvv_dn7, locals.var_fn277_calc_iq__qinvv_dn12, locals.var_fn277_calc_iq__qinvv_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvv = assign22310_e21125;
        locals.var_fn277_calc_iq__qinvv_dn2 = assign22310_e21125_d_n2;
        locals.var_fn277_calc_iq__qinvv_dn3 = assign22310_e21125_d_n3;
        locals.var_fn277_calc_iq__qinvv_dn4 = assign22310_e21125_d_n4;
        locals.var_fn277_calc_iq__qinvv_dn7 = assign22310_e21125_d_n7;
        locals.var_fn277_calc_iq__qinvv_dn12 = assign22310_e21125_d_n12;
        locals.var_fn277_calc_iq__qinvv_dn13 = assign22310_e21125_d_n13;
        locals.var_fn277_calc_iq__qinvv_rv = 0.0;

        let (assign22320_e21129, assign22320_e21129_d_n2, assign22320_e21129_d_n4, assign22320_e21129_d_n7, assign22320_e21129_d_n12, assign22320_e21129_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ff0, locals.var_fn277_calc_iq__ff0_dn2, locals.var_fn277_calc_iq__ff0_dn4, locals.var_fn277_calc_iq__ff0_dn7, locals.var_fn277_calc_iq__ff0_dn12, locals.var_fn277_calc_iq__ff0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ff0 = assign22320_e21129;
        locals.var_fn277_calc_iq__ff0_dn2 = assign22320_e21129_d_n2;
        locals.var_fn277_calc_iq__ff0_dn4 = assign22320_e21129_d_n4;
        locals.var_fn277_calc_iq__ff0_dn7 = assign22320_e21129_d_n7;
        locals.var_fn277_calc_iq__ff0_dn12 = assign22320_e21129_d_n12;
        locals.var_fn277_calc_iq__ff0_dn13 = assign22320_e21129_d_n13;
        locals.var_fn277_calc_iq__ff0_rv = 0.0;

        let (assign22330_e21133, assign22330_e21133_d_n2, assign22330_e21133_d_n4, assign22330_e21133_d_n7, assign22330_e21133_d_n12, assign22330_e21133_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__eta0, locals.var_fn277_calc_iq__eta0_dn2, locals.var_fn277_calc_iq__eta0_dn4, locals.var_fn277_calc_iq__eta0_dn7, locals.var_fn277_calc_iq__eta0_dn12, locals.var_fn277_calc_iq__eta0_dn13,)
    }
};
        locals.var_fn277_calc_iq__eta0 = assign22330_e21133;
        locals.var_fn277_calc_iq__eta0_dn2 = assign22330_e21133_d_n2;
        locals.var_fn277_calc_iq__eta0_dn4 = assign22330_e21133_d_n4;
        locals.var_fn277_calc_iq__eta0_dn7 = assign22330_e21133_d_n7;
        locals.var_fn277_calc_iq__eta0_dn12 = assign22330_e21133_d_n12;
        locals.var_fn277_calc_iq__eta0_dn13 = assign22330_e21133_d_n13;
        locals.var_fn277_calc_iq__eta0_rv = 0.0;

        let (assign22340_e21137, assign22340_e21137_d_n2, assign22340_e21137_d_n4, assign22340_e21137_d_n7, assign22340_e21137_d_n12, assign22340_e21137_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qinvv0, locals.var_fn277_calc_iq__qinvv0_dn2, locals.var_fn277_calc_iq__qinvv0_dn4, locals.var_fn277_calc_iq__qinvv0_dn7, locals.var_fn277_calc_iq__qinvv0_dn12, locals.var_fn277_calc_iq__qinvv0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvv0 = assign22340_e21137;
        locals.var_fn277_calc_iq__qinvv0_dn2 = assign22340_e21137_d_n2;
        locals.var_fn277_calc_iq__qinvv0_dn4 = assign22340_e21137_d_n4;
        locals.var_fn277_calc_iq__qinvv0_dn7 = assign22340_e21137_d_n7;
        locals.var_fn277_calc_iq__qinvv0_dn12 = assign22340_e21137_d_n12;
        locals.var_fn277_calc_iq__qinvv0_dn13 = assign22340_e21137_d_n13;
        locals.var_fn277_calc_iq__qinvv0_rv = 0.0;

        let (assign22350_e21141, assign22350_e21141_d_n2, assign22350_e21141_d_n3, assign22350_e21141_d_n4, assign22350_e21141_d_n7, assign22350_e21141_d_n12, assign22350_e21141_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsats, locals.var_fn277_calc_iq__vdsats_dn2, locals.var_fn277_calc_iq__vdsats_dn3, locals.var_fn277_calc_iq__vdsats_dn4, locals.var_fn277_calc_iq__vdsats_dn7, locals.var_fn277_calc_iq__vdsats_dn12, locals.var_fn277_calc_iq__vdsats_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsats = assign22350_e21141;
        locals.var_fn277_calc_iq__vdsats_dn2 = assign22350_e21141_d_n2;
        locals.var_fn277_calc_iq__vdsats_dn3 = assign22350_e21141_d_n3;
        locals.var_fn277_calc_iq__vdsats_dn4 = assign22350_e21141_d_n4;
        locals.var_fn277_calc_iq__vdsats_dn7 = assign22350_e21141_d_n7;
        locals.var_fn277_calc_iq__vdsats_dn12 = assign22350_e21141_d_n12;
        locals.var_fn277_calc_iq__vdsats_dn13 = assign22350_e21141_d_n13;
        locals.var_fn277_calc_iq__vdsats_rv = 0.0;

        let (assign22360_e21145, assign22360_e21145_d_n2, assign22360_e21145_d_n3, assign22360_e21145_d_n4, assign22360_e21145_d_n7, assign22360_e21145_d_n12, assign22360_e21145_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsats1, locals.var_fn277_calc_iq__vdsats1_dn2, locals.var_fn277_calc_iq__vdsats1_dn3, locals.var_fn277_calc_iq__vdsats1_dn4, locals.var_fn277_calc_iq__vdsats1_dn7, locals.var_fn277_calc_iq__vdsats1_dn12, locals.var_fn277_calc_iq__vdsats1_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsats1 = assign22360_e21145;
        locals.var_fn277_calc_iq__vdsats1_dn2 = assign22360_e21145_d_n2;
        locals.var_fn277_calc_iq__vdsats1_dn3 = assign22360_e21145_d_n3;
        locals.var_fn277_calc_iq__vdsats1_dn4 = assign22360_e21145_d_n4;
        locals.var_fn277_calc_iq__vdsats1_dn7 = assign22360_e21145_d_n7;
        locals.var_fn277_calc_iq__vdsats1_dn12 = assign22360_e21145_d_n12;
        locals.var_fn277_calc_iq__vdsats1_dn13 = assign22360_e21145_d_n13;
        locals.var_fn277_calc_iq__vdsats1_rv = 0.0;

        let (assign22370_e21149, assign22370_e21149_d_n2, assign22370_e21149_d_n3, assign22370_e21149_d_n4, assign22370_e21149_d_n7, assign22370_e21149_d_n12, assign22370_e21149_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsat, locals.var_fn277_calc_iq__vdsat_dn2, locals.var_fn277_calc_iq__vdsat_dn3, locals.var_fn277_calc_iq__vdsat_dn4, locals.var_fn277_calc_iq__vdsat_dn7, locals.var_fn277_calc_iq__vdsat_dn12, locals.var_fn277_calc_iq__vdsat_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsat = assign22370_e21149;
        locals.var_fn277_calc_iq__vdsat_dn2 = assign22370_e21149_d_n2;
        locals.var_fn277_calc_iq__vdsat_dn3 = assign22370_e21149_d_n3;
        locals.var_fn277_calc_iq__vdsat_dn4 = assign22370_e21149_d_n4;
        locals.var_fn277_calc_iq__vdsat_dn7 = assign22370_e21149_d_n7;
        locals.var_fn277_calc_iq__vdsat_dn12 = assign22370_e21149_d_n12;
        locals.var_fn277_calc_iq__vdsat_dn13 = assign22370_e21149_d_n13;
        locals.var_fn277_calc_iq__vdsat_rv = 0.0;

        let (assign22380_e21153, assign22380_e21153_d_n2, assign22380_e21153_d_n3, assign22380_e21153_d_n4, assign22380_e21153_d_n7, assign22380_e21153_d_n12, assign22380_e21153_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__fsd, locals.var_fn277_calc_iq__fsd_dn2, locals.var_fn277_calc_iq__fsd_dn3, locals.var_fn277_calc_iq__fsd_dn4, locals.var_fn277_calc_iq__fsd_dn7, locals.var_fn277_calc_iq__fsd_dn12, locals.var_fn277_calc_iq__fsd_dn13,)
    }
};
        locals.var_fn277_calc_iq__fsd = assign22380_e21153;
        locals.var_fn277_calc_iq__fsd_dn2 = assign22380_e21153_d_n2;
        locals.var_fn277_calc_iq__fsd_dn3 = assign22380_e21153_d_n3;
        locals.var_fn277_calc_iq__fsd_dn4 = assign22380_e21153_d_n4;
        locals.var_fn277_calc_iq__fsd_dn7 = assign22380_e21153_d_n7;
        locals.var_fn277_calc_iq__fsd_dn12 = assign22380_e21153_d_n12;
        locals.var_fn277_calc_iq__fsd_dn13 = assign22380_e21153_d_n13;
        locals.var_fn277_calc_iq__fsd_rv = 0.0;

        let (assign22390_e21157, assign22390_e21157_d_n2, assign22390_e21157_d_n3, assign22390_e21157_d_n4, assign22390_e21157_d_n7, assign22390_e21157_d_n12, assign22390_e21157_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdx, locals.var_fn277_calc_iq__vdx_dn2, locals.var_fn277_calc_iq__vdx_dn3, locals.var_fn277_calc_iq__vdx_dn4, locals.var_fn277_calc_iq__vdx_dn7, locals.var_fn277_calc_iq__vdx_dn12, locals.var_fn277_calc_iq__vdx_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdx = assign22390_e21157;
        locals.var_fn277_calc_iq__vdx_dn2 = assign22390_e21157_d_n2;
        locals.var_fn277_calc_iq__vdx_dn3 = assign22390_e21157_d_n3;
        locals.var_fn277_calc_iq__vdx_dn4 = assign22390_e21157_d_n4;
        locals.var_fn277_calc_iq__vdx_dn7 = assign22390_e21157_d_n7;
        locals.var_fn277_calc_iq__vdx_dn12 = assign22390_e21157_d_n12;
        locals.var_fn277_calc_iq__vdx_dn13 = assign22390_e21157_d_n13;
        locals.var_fn277_calc_iq__vdx_rv = 0.0;

        let (assign22400_e21161, assign22400_e21161_d_n2, assign22400_e21161_d_n3, assign22400_e21161_d_n4, assign22400_e21161_d_n7, assign22400_e21161_d_n12, assign22400_e21161_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__fds, locals.var_fn277_calc_iq__fds_dn2, locals.var_fn277_calc_iq__fds_dn3, locals.var_fn277_calc_iq__fds_dn4, locals.var_fn277_calc_iq__fds_dn7, locals.var_fn277_calc_iq__fds_dn12, locals.var_fn277_calc_iq__fds_dn13,)
    }
};
        locals.var_fn277_calc_iq__fds = assign22400_e21161;
        locals.var_fn277_calc_iq__fds_dn2 = assign22400_e21161_d_n2;
        locals.var_fn277_calc_iq__fds_dn3 = assign22400_e21161_d_n3;
        locals.var_fn277_calc_iq__fds_dn4 = assign22400_e21161_d_n4;
        locals.var_fn277_calc_iq__fds_dn7 = assign22400_e21161_d_n7;
        locals.var_fn277_calc_iq__fds_dn12 = assign22400_e21161_d_n12;
        locals.var_fn277_calc_iq__fds_dn13 = assign22400_e21161_d_n13;
        locals.var_fn277_calc_iq__fds_rv = 0.0;

        let (assign22410_e21165, assign22410_e21165_d_n2, assign22410_e21165_d_n3, assign22410_e21165_d_n4, assign22410_e21165_d_n7, assign22410_e21165_d_n12, assign22410_e21165_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vsx, locals.var_fn277_calc_iq__vsx_dn2, locals.var_fn277_calc_iq__vsx_dn3, locals.var_fn277_calc_iq__vsx_dn4, locals.var_fn277_calc_iq__vsx_dn7, locals.var_fn277_calc_iq__vsx_dn12, locals.var_fn277_calc_iq__vsx_dn13,)
    }
};
        locals.var_fn277_calc_iq__vsx = assign22410_e21165;
        locals.var_fn277_calc_iq__vsx_dn2 = assign22410_e21165_d_n2;
        locals.var_fn277_calc_iq__vsx_dn3 = assign22410_e21165_d_n3;
        locals.var_fn277_calc_iq__vsx_dn4 = assign22410_e21165_d_n4;
        locals.var_fn277_calc_iq__vsx_dn7 = assign22410_e21165_d_n7;
        locals.var_fn277_calc_iq__vsx_dn12 = assign22410_e21165_d_n12;
        locals.var_fn277_calc_iq__vsx_dn13 = assign22410_e21165_d_n13;
        locals.var_fn277_calc_iq__vsx_rv = 0.0;

        let (assign22420_e21169, assign22420_e21169_d_n2, assign22420_e21169_d_n3, assign22420_e21169_d_n4, assign22420_e21169_d_n7, assign22420_e21169_d_n12, assign22420_e21169_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffd, locals.var_fn277_calc_iq__ffd_dn2, locals.var_fn277_calc_iq__ffd_dn3, locals.var_fn277_calc_iq__ffd_dn4, locals.var_fn277_calc_iq__ffd_dn7, locals.var_fn277_calc_iq__ffd_dn12, locals.var_fn277_calc_iq__ffd_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffd = assign22420_e21169;
        locals.var_fn277_calc_iq__ffd_dn2 = assign22420_e21169_d_n2;
        locals.var_fn277_calc_iq__ffd_dn3 = assign22420_e21169_d_n3;
        locals.var_fn277_calc_iq__ffd_dn4 = assign22420_e21169_d_n4;
        locals.var_fn277_calc_iq__ffd_dn7 = assign22420_e21169_d_n7;
        locals.var_fn277_calc_iq__ffd_dn12 = assign22420_e21169_d_n12;
        locals.var_fn277_calc_iq__ffd_dn13 = assign22420_e21169_d_n13;
        locals.var_fn277_calc_iq__ffd_rv = 0.0;

        let (assign22430_e21173, assign22430_e21173_d_n2, assign22430_e21173_d_n3, assign22430_e21173_d_n4, assign22430_e21173_d_n7, assign22430_e21173_d_n12, assign22430_e21173_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__etad, locals.var_fn277_calc_iq__etad_dn2, locals.var_fn277_calc_iq__etad_dn3, locals.var_fn277_calc_iq__etad_dn4, locals.var_fn277_calc_iq__etad_dn7, locals.var_fn277_calc_iq__etad_dn12, locals.var_fn277_calc_iq__etad_dn13,)
    }
};
        locals.var_fn277_calc_iq__etad = assign22430_e21173;
        locals.var_fn277_calc_iq__etad_dn2 = assign22430_e21173_d_n2;
        locals.var_fn277_calc_iq__etad_dn3 = assign22430_e21173_d_n3;
        locals.var_fn277_calc_iq__etad_dn4 = assign22430_e21173_d_n4;
        locals.var_fn277_calc_iq__etad_dn7 = assign22430_e21173_d_n7;
        locals.var_fn277_calc_iq__etad_dn12 = assign22430_e21173_d_n12;
        locals.var_fn277_calc_iq__etad_dn13 = assign22430_e21173_d_n13;
        locals.var_fn277_calc_iq__etad_rv = 0.0;

        let (assign22440_e21177, assign22440_e21177_d_n2, assign22440_e21177_d_n3, assign22440_e21177_d_n4, assign22440_e21177_d_n7, assign22440_e21177_d_n12, assign22440_e21177_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qinvd, locals.var_fn277_calc_iq__qinvd_dn2, locals.var_fn277_calc_iq__qinvd_dn3, locals.var_fn277_calc_iq__qinvd_dn4, locals.var_fn277_calc_iq__qinvd_dn7, locals.var_fn277_calc_iq__qinvd_dn12, locals.var_fn277_calc_iq__qinvd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvd = assign22440_e21177;
        locals.var_fn277_calc_iq__qinvd_dn2 = assign22440_e21177_d_n2;
        locals.var_fn277_calc_iq__qinvd_dn3 = assign22440_e21177_d_n3;
        locals.var_fn277_calc_iq__qinvd_dn4 = assign22440_e21177_d_n4;
        locals.var_fn277_calc_iq__qinvd_dn7 = assign22440_e21177_d_n7;
        locals.var_fn277_calc_iq__qinvd_dn12 = assign22440_e21177_d_n12;
        locals.var_fn277_calc_iq__qinvd_dn13 = assign22440_e21177_d_n13;
        locals.var_fn277_calc_iq__qinvd_rv = 0.0;

        let (assign22450_e21181, assign22450_e21181_d_n2, assign22450_e21181_d_n3, assign22450_e21181_d_n4, assign22450_e21181_d_n7, assign22450_e21181_d_n12, assign22450_e21181_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsc, locals.var_fn277_calc_iq__vdsc_dn2, locals.var_fn277_calc_iq__vdsc_dn3, locals.var_fn277_calc_iq__vdsc_dn4, locals.var_fn277_calc_iq__vdsc_dn7, locals.var_fn277_calc_iq__vdsc_dn12, locals.var_fn277_calc_iq__vdsc_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsc = assign22450_e21181;
        locals.var_fn277_calc_iq__vdsc_dn2 = assign22450_e21181_d_n2;
        locals.var_fn277_calc_iq__vdsc_dn3 = assign22450_e21181_d_n3;
        locals.var_fn277_calc_iq__vdsc_dn4 = assign22450_e21181_d_n4;
        locals.var_fn277_calc_iq__vdsc_dn7 = assign22450_e21181_d_n7;
        locals.var_fn277_calc_iq__vdsc_dn12 = assign22450_e21181_d_n12;
        locals.var_fn277_calc_iq__vdsc_dn13 = assign22450_e21181_d_n13;
        locals.var_fn277_calc_iq__vdsc_rv = 0.0;

        let (assign22480_e21193, assign22480_e21193_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsats0, locals.var_fn277_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn277_calc_iq__vdsats0 = assign22480_e21193;
        locals.var_fn277_calc_iq__vdsats0_dn4 = assign22480_e21193_d_n4;
        locals.var_fn277_calc_iq__vdsats0_rv = 0.0;

        let (assign22490_e21197, assign22490_e21197_d_n2, assign22490_e21197_d_n4, assign22490_e21197_d_n7, assign22490_e21197_d_n12, assign22490_e21197_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsats10, locals.var_fn277_calc_iq__vdsats10_dn2, locals.var_fn277_calc_iq__vdsats10_dn4, locals.var_fn277_calc_iq__vdsats10_dn7, locals.var_fn277_calc_iq__vdsats10_dn12, locals.var_fn277_calc_iq__vdsats10_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsats10 = assign22490_e21197;
        locals.var_fn277_calc_iq__vdsats10_dn2 = assign22490_e21197_d_n2;
        locals.var_fn277_calc_iq__vdsats10_dn4 = assign22490_e21197_d_n4;
        locals.var_fn277_calc_iq__vdsats10_dn7 = assign22490_e21197_d_n7;
        locals.var_fn277_calc_iq__vdsats10_dn12 = assign22490_e21197_d_n12;
        locals.var_fn277_calc_iq__vdsats10_dn13 = assign22490_e21197_d_n13;
        locals.var_fn277_calc_iq__vdsats10_rv = 0.0;

        let (assign22500_e21201, assign22500_e21201_d_n2, assign22500_e21201_d_n4, assign22500_e21201_d_n7, assign22500_e21201_d_n12, assign22500_e21201_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdsat10, locals.var_fn277_calc_iq__vdsat10_dn2, locals.var_fn277_calc_iq__vdsat10_dn4, locals.var_fn277_calc_iq__vdsat10_dn7, locals.var_fn277_calc_iq__vdsat10_dn12, locals.var_fn277_calc_iq__vdsat10_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsat10 = assign22500_e21201;
        locals.var_fn277_calc_iq__vdsat10_dn2 = assign22500_e21201_d_n2;
        locals.var_fn277_calc_iq__vdsat10_dn4 = assign22500_e21201_d_n4;
        locals.var_fn277_calc_iq__vdsat10_dn7 = assign22500_e21201_d_n7;
        locals.var_fn277_calc_iq__vdsat10_dn12 = assign22500_e21201_d_n12;
        locals.var_fn277_calc_iq__vdsat10_dn13 = assign22500_e21201_d_n13;
        locals.var_fn277_calc_iq__vdsat10_rv = 0.0;

        let (assign22510_e21205, assign22510_e21205_d_n2, assign22510_e21205_d_n4, assign22510_e21205_d_n7, assign22510_e21205_d_n12, assign22510_e21205_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__fsd0, locals.var_fn277_calc_iq__fsd0_dn2, locals.var_fn277_calc_iq__fsd0_dn4, locals.var_fn277_calc_iq__fsd0_dn7, locals.var_fn277_calc_iq__fsd0_dn12, locals.var_fn277_calc_iq__fsd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__fsd0 = assign22510_e21205;
        locals.var_fn277_calc_iq__fsd0_dn2 = assign22510_e21205_d_n2;
        locals.var_fn277_calc_iq__fsd0_dn4 = assign22510_e21205_d_n4;
        locals.var_fn277_calc_iq__fsd0_dn7 = assign22510_e21205_d_n7;
        locals.var_fn277_calc_iq__fsd0_dn12 = assign22510_e21205_d_n12;
        locals.var_fn277_calc_iq__fsd0_dn13 = assign22510_e21205_d_n13;
        locals.var_fn277_calc_iq__fsd0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_61(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign22520_e21209, assign22520_e21209_d_n2, assign22520_e21209_d_n4, assign22520_e21209_d_n7, assign22520_e21209_d_n12, assign22520_e21209_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vdx0, locals.var_fn277_calc_iq__vdx0_dn2, locals.var_fn277_calc_iq__vdx0_dn4, locals.var_fn277_calc_iq__vdx0_dn7, locals.var_fn277_calc_iq__vdx0_dn12, locals.var_fn277_calc_iq__vdx0_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdx0 = assign22520_e21209;
        locals.var_fn277_calc_iq__vdx0_dn2 = assign22520_e21209_d_n2;
        locals.var_fn277_calc_iq__vdx0_dn4 = assign22520_e21209_d_n4;
        locals.var_fn277_calc_iq__vdx0_dn7 = assign22520_e21209_d_n7;
        locals.var_fn277_calc_iq__vdx0_dn12 = assign22520_e21209_d_n12;
        locals.var_fn277_calc_iq__vdx0_dn13 = assign22520_e21209_d_n13;
        locals.var_fn277_calc_iq__vdx0_rv = 0.0;

        let (assign22530_e21213, assign22530_e21213_d_n2, assign22530_e21213_d_n4, assign22530_e21213_d_n7, assign22530_e21213_d_n12, assign22530_e21213_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__fds0, locals.var_fn277_calc_iq__fds0_dn2, locals.var_fn277_calc_iq__fds0_dn4, locals.var_fn277_calc_iq__fds0_dn7, locals.var_fn277_calc_iq__fds0_dn12, locals.var_fn277_calc_iq__fds0_dn13,)
    }
};
        locals.var_fn277_calc_iq__fds0 = assign22530_e21213;
        locals.var_fn277_calc_iq__fds0_dn2 = assign22530_e21213_d_n2;
        locals.var_fn277_calc_iq__fds0_dn4 = assign22530_e21213_d_n4;
        locals.var_fn277_calc_iq__fds0_dn7 = assign22530_e21213_d_n7;
        locals.var_fn277_calc_iq__fds0_dn12 = assign22530_e21213_d_n12;
        locals.var_fn277_calc_iq__fds0_dn13 = assign22530_e21213_d_n13;
        locals.var_fn277_calc_iq__fds0_rv = 0.0;

        let (assign22540_e21217, assign22540_e21217_d_n2, assign22540_e21217_d_n4, assign22540_e21217_d_n7, assign22540_e21217_d_n12, assign22540_e21217_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vsx0, locals.var_fn277_calc_iq__vsx0_dn2, locals.var_fn277_calc_iq__vsx0_dn4, locals.var_fn277_calc_iq__vsx0_dn7, locals.var_fn277_calc_iq__vsx0_dn12, locals.var_fn277_calc_iq__vsx0_dn13,)
    }
};
        locals.var_fn277_calc_iq__vsx0 = assign22540_e21217;
        locals.var_fn277_calc_iq__vsx0_dn2 = assign22540_e21217_d_n2;
        locals.var_fn277_calc_iq__vsx0_dn4 = assign22540_e21217_d_n4;
        locals.var_fn277_calc_iq__vsx0_dn7 = assign22540_e21217_d_n7;
        locals.var_fn277_calc_iq__vsx0_dn12 = assign22540_e21217_d_n12;
        locals.var_fn277_calc_iq__vsx0_dn13 = assign22540_e21217_d_n13;
        locals.var_fn277_calc_iq__vsx0_rv = 0.0;

        let (assign22550_e21221, assign22550_e21221_d_n2, assign22550_e21221_d_n4, assign22550_e21221_d_n7, assign22550_e21221_d_n12, assign22550_e21221_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffd0, locals.var_fn277_calc_iq__ffd0_dn2, locals.var_fn277_calc_iq__ffd0_dn4, locals.var_fn277_calc_iq__ffd0_dn7, locals.var_fn277_calc_iq__ffd0_dn12, locals.var_fn277_calc_iq__ffd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffd0 = assign22550_e21221;
        locals.var_fn277_calc_iq__ffd0_dn2 = assign22550_e21221_d_n2;
        locals.var_fn277_calc_iq__ffd0_dn4 = assign22550_e21221_d_n4;
        locals.var_fn277_calc_iq__ffd0_dn7 = assign22550_e21221_d_n7;
        locals.var_fn277_calc_iq__ffd0_dn12 = assign22550_e21221_d_n12;
        locals.var_fn277_calc_iq__ffd0_dn13 = assign22550_e21221_d_n13;
        locals.var_fn277_calc_iq__ffd0_rv = 0.0;

        let (assign22560_e21225, assign22560_e21225_d_n2, assign22560_e21225_d_n4, assign22560_e21225_d_n7, assign22560_e21225_d_n12, assign22560_e21225_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__etad0, locals.var_fn277_calc_iq__etad0_dn2, locals.var_fn277_calc_iq__etad0_dn4, locals.var_fn277_calc_iq__etad0_dn7, locals.var_fn277_calc_iq__etad0_dn12, locals.var_fn277_calc_iq__etad0_dn13,)
    }
};
        locals.var_fn277_calc_iq__etad0 = assign22560_e21225;
        locals.var_fn277_calc_iq__etad0_dn2 = assign22560_e21225_d_n2;
        locals.var_fn277_calc_iq__etad0_dn4 = assign22560_e21225_d_n4;
        locals.var_fn277_calc_iq__etad0_dn7 = assign22560_e21225_d_n7;
        locals.var_fn277_calc_iq__etad0_dn12 = assign22560_e21225_d_n12;
        locals.var_fn277_calc_iq__etad0_dn13 = assign22560_e21225_d_n13;
        locals.var_fn277_calc_iq__etad0_rv = 0.0;

        let (assign22570_e21229, assign22570_e21229_d_n2, assign22570_e21229_d_n4, assign22570_e21229_d_n7, assign22570_e21229_d_n12, assign22570_e21229_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qinvd0, locals.var_fn277_calc_iq__qinvd0_dn2, locals.var_fn277_calc_iq__qinvd0_dn4, locals.var_fn277_calc_iq__qinvd0_dn7, locals.var_fn277_calc_iq__qinvd0_dn12, locals.var_fn277_calc_iq__qinvd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvd0 = assign22570_e21229;
        locals.var_fn277_calc_iq__qinvd0_dn2 = assign22570_e21229_d_n2;
        locals.var_fn277_calc_iq__qinvd0_dn4 = assign22570_e21229_d_n4;
        locals.var_fn277_calc_iq__qinvd0_dn7 = assign22570_e21229_d_n7;
        locals.var_fn277_calc_iq__qinvd0_dn12 = assign22570_e21229_d_n12;
        locals.var_fn277_calc_iq__qinvd0_dn13 = assign22570_e21229_d_n13;
        locals.var_fn277_calc_iq__qinvd0_rv = 0.0;

        let (assign22580_e21233, assign22580_e21233_d_n2, assign22580_e21233_d_n4, assign22580_e21233_d_n7, assign22580_e21233_d_n12, assign22580_e21233_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qs2, locals.var_fn277_calc_iq__qs2_dn2, locals.var_fn277_calc_iq__qs2_dn4, locals.var_fn277_calc_iq__qs2_dn7, locals.var_fn277_calc_iq__qs2_dn12, locals.var_fn277_calc_iq__qs2_dn13,)
    }
};
        locals.var_fn277_calc_iq__qs2 = assign22580_e21233;
        locals.var_fn277_calc_iq__qs2_dn2 = assign22580_e21233_d_n2;
        locals.var_fn277_calc_iq__qs2_dn4 = assign22580_e21233_d_n4;
        locals.var_fn277_calc_iq__qs2_dn7 = assign22580_e21233_d_n7;
        locals.var_fn277_calc_iq__qs2_dn12 = assign22580_e21233_d_n12;
        locals.var_fn277_calc_iq__qs2_dn13 = assign22580_e21233_d_n13;
        locals.var_fn277_calc_iq__qs2_rv = 0.0;

        let (assign22590_e21237, assign22590_e21237_d_n2, assign22590_e21237_d_n4, assign22590_e21237_d_n7, assign22590_e21237_d_n12, assign22590_e21237_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qs3, locals.var_fn277_calc_iq__qs3_dn2, locals.var_fn277_calc_iq__qs3_dn4, locals.var_fn277_calc_iq__qs3_dn7, locals.var_fn277_calc_iq__qs3_dn12, locals.var_fn277_calc_iq__qs3_dn13,)
    }
};
        locals.var_fn277_calc_iq__qs3 = assign22590_e21237;
        locals.var_fn277_calc_iq__qs3_dn2 = assign22590_e21237_d_n2;
        locals.var_fn277_calc_iq__qs3_dn4 = assign22590_e21237_d_n4;
        locals.var_fn277_calc_iq__qs3_dn7 = assign22590_e21237_d_n7;
        locals.var_fn277_calc_iq__qs3_dn12 = assign22590_e21237_d_n12;
        locals.var_fn277_calc_iq__qs3_dn13 = assign22590_e21237_d_n13;
        locals.var_fn277_calc_iq__qs3_rv = 0.0;

        let (assign22600_e21241, assign22600_e21241_d_n2, assign22600_e21241_d_n4, assign22600_e21241_d_n7, assign22600_e21241_d_n12, assign22600_e21241_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qd2, locals.var_fn277_calc_iq__qd2_dn2, locals.var_fn277_calc_iq__qd2_dn4, locals.var_fn277_calc_iq__qd2_dn7, locals.var_fn277_calc_iq__qd2_dn12, locals.var_fn277_calc_iq__qd2_dn13,)
    }
};
        locals.var_fn277_calc_iq__qd2 = assign22600_e21241;
        locals.var_fn277_calc_iq__qd2_dn2 = assign22600_e21241_d_n2;
        locals.var_fn277_calc_iq__qd2_dn4 = assign22600_e21241_d_n4;
        locals.var_fn277_calc_iq__qd2_dn7 = assign22600_e21241_d_n7;
        locals.var_fn277_calc_iq__qd2_dn12 = assign22600_e21241_d_n12;
        locals.var_fn277_calc_iq__qd2_dn13 = assign22600_e21241_d_n13;
        locals.var_fn277_calc_iq__qd2_rv = 0.0;

        let (assign22610_e21245, assign22610_e21245_d_n2, assign22610_e21245_d_n4, assign22610_e21245_d_n7, assign22610_e21245_d_n12, assign22610_e21245_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qd3, locals.var_fn277_calc_iq__qd3_dn2, locals.var_fn277_calc_iq__qd3_dn4, locals.var_fn277_calc_iq__qd3_dn7, locals.var_fn277_calc_iq__qd3_dn12, locals.var_fn277_calc_iq__qd3_dn13,)
    }
};
        locals.var_fn277_calc_iq__qd3 = assign22610_e21245;
        locals.var_fn277_calc_iq__qd3_dn2 = assign22610_e21245_d_n2;
        locals.var_fn277_calc_iq__qd3_dn4 = assign22610_e21245_d_n4;
        locals.var_fn277_calc_iq__qd3_dn7 = assign22610_e21245_d_n7;
        locals.var_fn277_calc_iq__qd3_dn12 = assign22610_e21245_d_n12;
        locals.var_fn277_calc_iq__qd3_dn13 = assign22610_e21245_d_n13;
        locals.var_fn277_calc_iq__qd3_rv = 0.0;

        let (assign22620_e21249, assign22620_e21249_d_n2, assign22620_e21249_d_n4, assign22620_e21249_d_n7, assign22620_e21249_d_n12, assign22620_e21249_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qsqd, locals.var_fn277_calc_iq__qsqd_dn2, locals.var_fn277_calc_iq__qsqd_dn4, locals.var_fn277_calc_iq__qsqd_dn7, locals.var_fn277_calc_iq__qsqd_dn12, locals.var_fn277_calc_iq__qsqd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qsqd = assign22620_e21249;
        locals.var_fn277_calc_iq__qsqd_dn2 = assign22620_e21249_d_n2;
        locals.var_fn277_calc_iq__qsqd_dn4 = assign22620_e21249_d_n4;
        locals.var_fn277_calc_iq__qsqd_dn7 = assign22620_e21249_d_n7;
        locals.var_fn277_calc_iq__qsqd_dn12 = assign22620_e21249_d_n12;
        locals.var_fn277_calc_iq__qsqd_dn13 = assign22620_e21249_d_n13;
        locals.var_fn277_calc_iq__qsqd_rv = 0.0;

        let (assign22630_e21253, assign22630_e21253_d_n2, assign22630_e21253_d_n4, assign22630_e21253_d_n7, assign22630_e21253_d_n12, assign22630_e21253_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qinvdd, locals.var_fn277_calc_iq__qinvdd_dn2, locals.var_fn277_calc_iq__qinvdd_dn4, locals.var_fn277_calc_iq__qinvdd_dn7, locals.var_fn277_calc_iq__qinvdd_dn12, locals.var_fn277_calc_iq__qinvdd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvdd = assign22630_e21253;
        locals.var_fn277_calc_iq__qinvdd_dn2 = assign22630_e21253_d_n2;
        locals.var_fn277_calc_iq__qinvdd_dn4 = assign22630_e21253_d_n4;
        locals.var_fn277_calc_iq__qinvdd_dn7 = assign22630_e21253_d_n7;
        locals.var_fn277_calc_iq__qinvdd_dn12 = assign22630_e21253_d_n12;
        locals.var_fn277_calc_iq__qinvdd_dn13 = assign22630_e21253_d_n13;
        locals.var_fn277_calc_iq__qinvdd_rv = 0.0;

        let (assign22640_e21257, assign22640_e21257_d_n2, assign22640_e21257_d_n4, assign22640_e21257_d_n7, assign22640_e21257_d_n12, assign22640_e21257_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qd1, locals.var_fn277_calc_iq__qd1_dn2, locals.var_fn277_calc_iq__qd1_dn4, locals.var_fn277_calc_iq__qd1_dn7, locals.var_fn277_calc_iq__qd1_dn12, locals.var_fn277_calc_iq__qd1_dn13,)
    }
};
        locals.var_fn277_calc_iq__qd1 = assign22640_e21257;
        locals.var_fn277_calc_iq__qd1_dn2 = assign22640_e21257_d_n2;
        locals.var_fn277_calc_iq__qd1_dn4 = assign22640_e21257_d_n4;
        locals.var_fn277_calc_iq__qd1_dn7 = assign22640_e21257_d_n7;
        locals.var_fn277_calc_iq__qd1_dn12 = assign22640_e21257_d_n12;
        locals.var_fn277_calc_iq__qd1_dn13 = assign22640_e21257_d_n13;
        locals.var_fn277_calc_iq__qd1_rv = 0.0;

        let (assign22650_e21261, assign22650_e21261_d_n2, assign22650_e21261_d_n4, assign22650_e21261_d_n7, assign22650_e21261_d_n12, assign22650_e21261_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qs, locals.var_fn277_calc_iq__qs_dn2, locals.var_fn277_calc_iq__qs_dn4, locals.var_fn277_calc_iq__qs_dn7, locals.var_fn277_calc_iq__qs_dn12, locals.var_fn277_calc_iq__qs_dn13,)
    }
};
        locals.var_fn277_calc_iq__qs = assign22650_e21261;
        locals.var_fn277_calc_iq__qs_dn2 = assign22650_e21261_d_n2;
        locals.var_fn277_calc_iq__qs_dn4 = assign22650_e21261_d_n4;
        locals.var_fn277_calc_iq__qs_dn7 = assign22650_e21261_d_n7;
        locals.var_fn277_calc_iq__qs_dn12 = assign22650_e21261_d_n12;
        locals.var_fn277_calc_iq__qs_dn13 = assign22650_e21261_d_n13;
        locals.var_fn277_calc_iq__qs_rv = 0.0;

        let (assign22660_e21265, assign22660_e21265_d_n2, assign22660_e21265_d_n4, assign22660_e21265_d_n7, assign22660_e21265_d_n12, assign22660_e21265_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qd, locals.var_fn277_calc_iq__qd_dn2, locals.var_fn277_calc_iq__qd_dn4, locals.var_fn277_calc_iq__qd_dn7, locals.var_fn277_calc_iq__qd_dn12, locals.var_fn277_calc_iq__qd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qd = assign22660_e21265;
        locals.var_fn277_calc_iq__qd_dn2 = assign22660_e21265_d_n2;
        locals.var_fn277_calc_iq__qd_dn4 = assign22660_e21265_d_n4;
        locals.var_fn277_calc_iq__qd_dn7 = assign22660_e21265_d_n7;
        locals.var_fn277_calc_iq__qd_dn12 = assign22660_e21265_d_n12;
        locals.var_fn277_calc_iq__qd_dn13 = assign22660_e21265_d_n13;
        locals.var_fn277_calc_iq__qd_rv = 0.0;

        let (assign22670_e21269, assign22670_e21269_d_n2, assign22670_e21269_d_n4, assign22670_e21269_d_n7, assign22670_e21269_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__etac, locals.var_fn277_calc_iq__etac_dn2, locals.var_fn277_calc_iq__etac_dn4, locals.var_fn277_calc_iq__etac_dn7, locals.var_fn277_calc_iq__etac_dn13,)
    }
};
        locals.var_fn277_calc_iq__etac = assign22670_e21269;
        locals.var_fn277_calc_iq__etac_dn2 = assign22670_e21269_d_n2;
        locals.var_fn277_calc_iq__etac_dn4 = assign22670_e21269_d_n4;
        locals.var_fn277_calc_iq__etac_dn7 = assign22670_e21269_d_n7;
        locals.var_fn277_calc_iq__etac_dn13 = assign22670_e21269_d_n13;
        locals.var_fn277_calc_iq__etac_rv = 0.0;

        let (assign22680_e21273, assign22680_e21273_d_n3, assign22680_e21273_d_n4, assign22680_e21273_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__etab, locals.var_fn277_calc_iq__etab_dn3, locals.var_fn277_calc_iq__etab_dn4, locals.var_fn277_calc_iq__etab_dn13,)
    }
};
        locals.var_fn277_calc_iq__etab = assign22680_e21273;
        locals.var_fn277_calc_iq__etab_dn3 = assign22680_e21273_d_n3;
        locals.var_fn277_calc_iq__etab_dn4 = assign22680_e21273_d_n4;
        locals.var_fn277_calc_iq__etab_dn13 = assign22680_e21273_d_n13;
        locals.var_fn277_calc_iq__etab_rv = 0.0;

        let (assign22690_e21277, assign22690_e21277_d_n2, assign22690_e21277_d_n4, assign22690_e21277_d_n7, assign22690_e21277_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__etags, locals.var_fn277_calc_iq__etags_dn2, locals.var_fn277_calc_iq__etags_dn4, locals.var_fn277_calc_iq__etags_dn7, locals.var_fn277_calc_iq__etags_dn13,)
    }
};
        locals.var_fn277_calc_iq__etags = assign22690_e21277;
        locals.var_fn277_calc_iq__etags_dn2 = assign22690_e21277_d_n2;
        locals.var_fn277_calc_iq__etags_dn4 = assign22690_e21277_d_n4;
        locals.var_fn277_calc_iq__etags_dn7 = assign22690_e21277_d_n7;
        locals.var_fn277_calc_iq__etags_dn13 = assign22690_e21277_d_n13;
        locals.var_fn277_calc_iq__etags_rv = 0.0;

        let (assign22700_e21281, assign22700_e21281_d_n2, assign22700_e21281_d_n3, assign22700_e21281_d_n4, assign22700_e21281_d_n7, assign22700_e21281_d_n12, assign22700_e21281_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign22700_e21281;
        locals.var_fn277_calc_iq__exparg_dn2 = assign22700_e21281_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign22700_e21281_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign22700_e21281_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign22700_e21281_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign22700_e21281_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign22700_e21281_d_n13;
        locals.var_fn277_calc_iq__exparg_rv = 0.0;

        let (assign22710_e21285, assign22710_e21285_d_n2, assign22710_e21285_d_n3, assign22710_e21285_d_n4, assign22710_e21285_d_n7, assign22710_e21285_d_n12, assign22710_e21285_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__myarg, locals.var_fn277_calc_iq__myarg_dn2, locals.var_fn277_calc_iq__myarg_dn3, locals.var_fn277_calc_iq__myarg_dn4, locals.var_fn277_calc_iq__myarg_dn7, locals.var_fn277_calc_iq__myarg_dn12, locals.var_fn277_calc_iq__myarg_dn13,)
    }
};
        locals.var_fn277_calc_iq__myarg = assign22710_e21285;
        locals.var_fn277_calc_iq__myarg_dn2 = assign22710_e21285_d_n2;
        locals.var_fn277_calc_iq__myarg_dn3 = assign22710_e21285_d_n3;
        locals.var_fn277_calc_iq__myarg_dn4 = assign22710_e21285_d_n4;
        locals.var_fn277_calc_iq__myarg_dn7 = assign22710_e21285_d_n7;
        locals.var_fn277_calc_iq__myarg_dn12 = assign22710_e21285_d_n12;
        locals.var_fn277_calc_iq__myarg_dn13 = assign22710_e21285_d_n13;
        locals.var_fn277_calc_iq__myarg_rv = 0.0;

        let (assign22720_e21289, assign22720_e21289_d_n12, assign22720_e21289_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__absvdsin, locals.var_fn277_calc_iq__absvdsin_dn12, locals.var_fn277_calc_iq__absvdsin_dn13,)
    }
};
        locals.var_fn277_calc_iq__absvdsin = assign22720_e21289;
        locals.var_fn277_calc_iq__absvdsin_dn12 = assign22720_e21289_d_n12;
        locals.var_fn277_calc_iq__absvdsin_dn13 = assign22720_e21289_d_n13;
        locals.var_fn277_calc_iq__absvdsin_rv = 0.0;

        let (assign22730_e21293, assign22730_e21293_d_n2, assign22730_e21293_d_n7, assign22730_e21293_d_n12, assign22730_e21293_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vgdin, locals.var_fn277_calc_iq__vgdin_dn2, locals.var_fn277_calc_iq__vgdin_dn7, locals.var_fn277_calc_iq__vgdin_dn12, locals.var_fn277_calc_iq__vgdin_dn13,)
    }
};
        locals.var_fn277_calc_iq__vgdin = assign22730_e21293;
        locals.var_fn277_calc_iq__vgdin_dn2 = assign22730_e21293_d_n2;
        locals.var_fn277_calc_iq__vgdin_dn7 = assign22730_e21293_d_n7;
        locals.var_fn277_calc_iq__vgdin_dn12 = assign22730_e21293_d_n12;
        locals.var_fn277_calc_iq__vgdin_dn13 = assign22730_e21293_d_n13;
        locals.var_fn277_calc_iq__vgdin_rv = 0.0;

        let (assign22740_e21297, assign22740_e21297_d_n2, assign22740_e21297_d_n4, assign22740_e21297_d_n7, assign22740_e21297_d_n12, assign22740_e21297_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__exparg0, locals.var_fn277_calc_iq__exparg0_dn2, locals.var_fn277_calc_iq__exparg0_dn4, locals.var_fn277_calc_iq__exparg0_dn7, locals.var_fn277_calc_iq__exparg0_dn12, locals.var_fn277_calc_iq__exparg0_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg0 = assign22740_e21297;
        locals.var_fn277_calc_iq__exparg0_dn2 = assign22740_e21297_d_n2;
        locals.var_fn277_calc_iq__exparg0_dn4 = assign22740_e21297_d_n4;
        locals.var_fn277_calc_iq__exparg0_dn7 = assign22740_e21297_d_n7;
        locals.var_fn277_calc_iq__exparg0_dn12 = assign22740_e21297_d_n12;
        locals.var_fn277_calc_iq__exparg0_dn13 = assign22740_e21297_d_n13;
        locals.var_fn277_calc_iq__exparg0_rv = 0.0;

        let (assign22750_e21301, assign22750_e21301_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        (0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__myarg0, locals.var_fn277_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn277_calc_iq__myarg0 = assign22750_e21301;
        locals.var_fn277_calc_iq__myarg0_dn4 = assign22750_e21301_d_n4;
        locals.var_fn277_calc_iq__myarg0_rv = 0.0;

        let (assign22760_e21328, assign22760_e21328_d_n12, assign22760_e21328_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign22760_e21326, assign22760_e21326_d_n12, assign22760_e21326_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign22760_e21310: f64 = (0.001 / p.p53);
                let assign22760_e21312: f64 = (assign22760_e21310 * locals.var_fn277_calc_iq__vdsin);
                let assign22760_e21313: f64 = (assign22760_e21312).tanh();
                let assign22760_e21314: f64 = (locals.var_fn277_calc_iq__vdsin * assign22760_e21313);
                (assign22760_e21314, ((locals.var_fn277_calc_iq__vdsin_dn12 * assign22760_e21313) + (locals.var_fn277_calc_iq__vdsin * ((assign22760_e21310 * locals.var_fn277_calc_iq__vdsin_dn12) / ((assign22760_e21312).cosh() * (assign22760_e21312).cosh())))), ((locals.var_fn277_calc_iq__vdsin_dn13 * assign22760_e21313) + (locals.var_fn277_calc_iq__vdsin * ((assign22760_e21310 * locals.var_fn277_calc_iq__vdsin_dn13) / ((assign22760_e21312).cosh() * (assign22760_e21312).cosh())))),)
            } else {
                let (assign22760_e21325, assign22760_e21325_d_n12, assign22760_e21325_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign22760_e21320: f64 = (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsin);
                        let assign22760_e21322: f64 = (assign22760_e21320 + p.p53);
                        let assign22760_e21323: f64 = (assign22760_e21322).sqrt();
                        (assign22760_e21323, (((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsin) + (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsin_dn12)) / (2.0 * assign22760_e21323)), (((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsin) + (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsin_dn13)) / (2.0 * assign22760_e21323)),)
                    } else {
                        (0.0, 0.0, 0.0,)
                    }
                };
                (assign22760_e21325, assign22760_e21325_d_n12, assign22760_e21325_d_n13,)
            }
        };
        (assign22760_e21326, assign22760_e21326_d_n12, assign22760_e21326_d_n13,)
    } else {
        (locals.var_fn277_calc_iq__absvdsin, locals.var_fn277_calc_iq__absvdsin_dn12, locals.var_fn277_calc_iq__absvdsin_dn13,)
    }
};
        locals.var_fn277_calc_iq__absvdsin = assign22760_e21328;
        locals.var_fn277_calc_iq__absvdsin_dn12 = assign22760_e21328_d_n12;
        locals.var_fn277_calc_iq__absvdsin_dn13 = assign22760_e21328_d_n13;
        locals.var_fn277_calc_iq__absvdsin_rv = 0.0;

        let (assign22770_e21334, assign22770_e21334_d_n2, assign22770_e21334_d_n7, assign22770_e21334_d_n12, assign22770_e21334_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22770_e21332: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vdsin);
        (assign22770_e21332, locals.var_fn277_calc_iq__vgsin_dn2, locals.var_fn277_calc_iq__vgsin_dn7, (-locals.var_fn277_calc_iq__vdsin_dn12), (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vdsin_dn13),)
    } else {
        (locals.var_fn277_calc_iq__vgdin, locals.var_fn277_calc_iq__vgdin_dn2, locals.var_fn277_calc_iq__vgdin_dn7, locals.var_fn277_calc_iq__vgdin_dn12, locals.var_fn277_calc_iq__vgdin_dn13,)
    }
};
        locals.var_fn277_calc_iq__vgdin = assign22770_e21334;
        locals.var_fn277_calc_iq__vgdin_dn2 = assign22770_e21334_d_n2;
        locals.var_fn277_calc_iq__vgdin_dn7 = assign22770_e21334_d_n7;
        locals.var_fn277_calc_iq__vgdin_dn12 = assign22770_e21334_d_n12;
        locals.var_fn277_calc_iq__vgdin_dn13 = assign22770_e21334_d_n13;
        locals.var_fn277_calc_iq__vgdin_rv = 0.0;

        let (assign22780_e21340, assign22780_e21340_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22780_e21338: f64 = (locals.var_fn277_calc_iq__alpha * locals.var_fn277_calc_iq__phitin);
        (assign22780_e21338, (locals.var_fn277_calc_iq__alpha * locals.var_fn277_calc_iq__phitin_dn4),)
    } else {
        (locals.var_fn277_calc_iq__alpha_phit, locals.var_fn277_calc_iq__alpha_phit_dn4,)
    }
};
        locals.var_fn277_calc_iq__alpha_phit = assign22780_e21340;
        locals.var_fn277_calc_iq__alpha_phit_dn4 = assign22780_e21340_d_n4;
        locals.var_fn277_calc_iq__alpha_phit_rv = 0.0;

        let (assign22790_e21352, assign22790_e21352_d_n4, assign22790_e21352_d_n12, assign22790_e21352_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22790_e21345: f64 = (2.302585092994046 * locals.var_fn277_calc_iq__phitin);
        let assign22790_e21346: f64 = (locals.var_fn277_calc_iq__ss / assign22790_e21345);
        let assign22790_e21349: f64 = (locals.var_fn277_calc_iq__nd * locals.var_fn277_calc_iq__absvdsin);
        let assign22790_e21350: f64 = (assign22790_e21346 + assign22790_e21349);
        (assign22790_e21350, (-((locals.var_fn277_calc_iq__ss * (2.302585092994046 * locals.var_fn277_calc_iq__phitin_dn4)) / (assign22790_e21345 * assign22790_e21345))), (locals.var_fn277_calc_iq__nd * locals.var_fn277_calc_iq__absvdsin_dn12), (locals.var_fn277_calc_iq__nd * locals.var_fn277_calc_iq__absvdsin_dn13),)
    } else {
        (locals.var_fn277_calc_iq__n, locals.var_fn277_calc_iq__n_dn4, locals.var_fn277_calc_iq__n_dn12, locals.var_fn277_calc_iq__n_dn13,)
    }
};
        locals.var_fn277_calc_iq__n = assign22790_e21352;
        locals.var_fn277_calc_iq__n_dn4 = assign22790_e21352_d_n4;
        locals.var_fn277_calc_iq__n_dn12 = assign22790_e21352_d_n12;
        locals.var_fn277_calc_iq__n_dn13 = assign22790_e21352_d_n13;
        locals.var_fn277_calc_iq__n_rv = 0.0;

        let (assign22800_e21362, assign22800_e21362_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22800_e21358: f64 = (locals.var_fn277_calc_iq__tambin - locals.var_fn277_calc_iq__tnomin);
        let assign22800_e21359: f64 = (locals.var_fn277_calc_iq__vtzeta * assign22800_e21358);
        let assign22800_e21360: f64 = (locals.var_fn277_calc_iq__vto + assign22800_e21359);
        (assign22800_e21360, (locals.var_fn277_calc_iq__vtzeta * locals.var_fn277_calc_iq__tambin_dn4),)
    } else {
        (locals.var_fn277_calc_iq__vtof, locals.var_fn277_calc_iq__vtof_dn4,)
    }
};
        locals.var_fn277_calc_iq__vtof = assign22800_e21362;
        locals.var_fn277_calc_iq__vtof_dn4 = assign22800_e21362_d_n4;
        locals.var_fn277_calc_iq__vtof_rv = 0.0;

        let (assign22810_e21370, assign22810_e21370_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22810_e21366: f64 = (locals.var_fn277_calc_iq__tambin / locals.var_fn277_calc_iq__tnomin);
        let assign22810_e21368: f64 = (assign22810_e21366).powf(locals.var_fn277_calc_iq__epsilon);
        (assign22810_e21368, if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__epsilon) as f64).is_finite() && ((locals.var_fn277_calc_iq__epsilon) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__epsilon == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__epsilon * ((assign22810_e21366).powf(locals.var_fn277_calc_iq__epsilon - 1.0) * (locals.var_fn277_calc_iq__tambin_dn4 / locals.var_fn277_calc_iq__tnomin))) } } else { (assign22810_e21368 * (locals.var_fn277_calc_iq__epsilon * ((locals.var_fn277_calc_iq__tambin_dn4 / locals.var_fn277_calc_iq__tnomin) / assign22810_e21366))) },)
    } else {
        (locals.var_fn277_calc_iq__tfacmobin, locals.var_fn277_calc_iq__tfacmobin_dn4,)
    }
};
        locals.var_fn277_calc_iq__tfacmobin = assign22810_e21370;
        locals.var_fn277_calc_iq__tfacmobin_dn4 = assign22810_e21370_d_n4;
        locals.var_fn277_calc_iq__tfacmobin_rv = 0.0;

        let assign22820_e21373: f64 = if locals.var_fn277_calc_iq__dibsat != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard278 = assign22820_e21373;
        locals.var_guard278_rv = 0.0;

        let (assign22830_e21391, assign22830_e21391_d_n12, assign22830_e21391_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard278 != 0.0)) {
        let assign22830_e21381: f64 = (locals.var_fn277_calc_iq__absvdsin / locals.var_fn277_calc_iq__dibsat);
        let assign22830_e21383: f64 = (assign22830_e21381).powf(locals.var_fn277_calc_iq__beta);
        let assign22830_e21384: f64 = (1.0 + assign22830_e21383);
        let assign22830_e21387: f64 = (1.0 / locals.var_fn277_calc_iq__beta);
        let assign22830_e21388: f64 = (assign22830_e21384).powf(assign22830_e21387);
        let assign22830_e21389: f64 = (locals.var_fn277_calc_iq__absvdsin / assign22830_e21388);
        (assign22830_e21389, (((locals.var_fn277_calc_iq__absvdsin_dn12 * assign22830_e21388) - (locals.var_fn277_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign22830_e21387) as f64).is_finite() && ((assign22830_e21387) as f64).fract() == 0.0 { if assign22830_e21387 == 0.0 { 0.0 } else { (assign22830_e21387 * ((assign22830_e21384).powf(assign22830_e21387 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign22830_e21381).powf(locals.var_fn277_calc_iq__beta - 1.0) * (locals.var_fn277_calc_iq__absvdsin_dn12 / locals.var_fn277_calc_iq__dibsat))) } } else { (assign22830_e21383 * (locals.var_fn277_calc_iq__beta * ((locals.var_fn277_calc_iq__absvdsin_dn12 / locals.var_fn277_calc_iq__dibsat) / assign22830_e21381))) })) } } else { (assign22830_e21388 * (assign22830_e21387 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign22830_e21381).powf(locals.var_fn277_calc_iq__beta - 1.0) * (locals.var_fn277_calc_iq__absvdsin_dn12 / locals.var_fn277_calc_iq__dibsat))) } } else { (assign22830_e21383 * (locals.var_fn277_calc_iq__beta * ((locals.var_fn277_calc_iq__absvdsin_dn12 / locals.var_fn277_calc_iq__dibsat) / assign22830_e21381))) } / assign22830_e21384))) })) / (assign22830_e21388 * assign22830_e21388)), (((locals.var_fn277_calc_iq__absvdsin_dn13 * assign22830_e21388) - (locals.var_fn277_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign22830_e21387) as f64).is_finite() && ((assign22830_e21387) as f64).fract() == 0.0 { if assign22830_e21387 == 0.0 { 0.0 } else { (assign22830_e21387 * ((assign22830_e21384).powf(assign22830_e21387 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign22830_e21381).powf(locals.var_fn277_calc_iq__beta - 1.0) * (locals.var_fn277_calc_iq__absvdsin_dn13 / locals.var_fn277_calc_iq__dibsat))) } } else { (assign22830_e21383 * (locals.var_fn277_calc_iq__beta * ((locals.var_fn277_calc_iq__absvdsin_dn13 / locals.var_fn277_calc_iq__dibsat) / assign22830_e21381))) })) } } else { (assign22830_e21388 * (assign22830_e21387 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign22830_e21381).powf(locals.var_fn277_calc_iq__beta - 1.0) * (locals.var_fn277_calc_iq__absvdsin_dn13 / locals.var_fn277_calc_iq__dibsat))) } } else { (assign22830_e21383 * (locals.var_fn277_calc_iq__beta * ((locals.var_fn277_calc_iq__absvdsin_dn13 / locals.var_fn277_calc_iq__dibsat) / assign22830_e21381))) } / assign22830_e21384))) })) / (assign22830_e21388 * assign22830_e21388)),)
    } else {
        (locals.var_fn277_calc_iq__vsatdibl, locals.var_fn277_calc_iq__vsatdibl_dn12, locals.var_fn277_calc_iq__vsatdibl_dn13,)
    }
};
        locals.var_fn277_calc_iq__vsatdibl = assign22830_e21391;
        locals.var_fn277_calc_iq__vsatdibl_dn12 = assign22830_e21391_d_n12;
        locals.var_fn277_calc_iq__vsatdibl_dn13 = assign22830_e21391_d_n13;
        locals.var_fn277_calc_iq__vsatdibl_rv = 0.0;

        let (assign22840_e21398, assign22840_e21398_d_n12, assign22840_e21398_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard278 == 0.0)) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__vsatdibl, locals.var_fn277_calc_iq__vsatdibl_dn12, locals.var_fn277_calc_iq__vsatdibl_dn13,)
    }
};
        locals.var_fn277_calc_iq__vsatdibl = assign22840_e21398;
        locals.var_fn277_calc_iq__vsatdibl_dn12 = assign22840_e21398_d_n12;
        locals.var_fn277_calc_iq__vsatdibl_dn13 = assign22840_e21398_d_n13;
        locals.var_fn277_calc_iq__vsatdibl_rv = 0.0;

        let (assign22850_e21408, assign22850_e21408_d_n12, assign22850_e21408_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22850_e21403: f64 = (locals.var_fn277_calc_iq__vsatdibl * locals.var_fn277_calc_iq__delta2);
        let assign22850_e21404: f64 = (locals.var_fn277_calc_iq__delta1 - assign22850_e21403);
        let assign22850_e21406: f64 = (assign22850_e21404 * locals.var_fn277_calc_iq__absvdsin);
        (assign22850_e21406, (((-(locals.var_fn277_calc_iq__vsatdibl_dn12 * locals.var_fn277_calc_iq__delta2)) * locals.var_fn277_calc_iq__absvdsin) + (assign22850_e21404 * locals.var_fn277_calc_iq__absvdsin_dn12)), (((-(locals.var_fn277_calc_iq__vsatdibl_dn13 * locals.var_fn277_calc_iq__delta2)) * locals.var_fn277_calc_iq__absvdsin) + (assign22850_e21404 * locals.var_fn277_calc_iq__absvdsin_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__delta, locals.var_fn277_calc_iq__delta_dn12, locals.var_fn277_calc_iq__delta_dn13,)
    }
};
        locals.var_fn277_calc_iq__delta = assign22850_e21408;
        locals.var_fn277_calc_iq__delta_dn12 = assign22850_e21408_d_n12;
        locals.var_fn277_calc_iq__delta_dn13 = assign22850_e21408_d_n13;
        locals.var_fn277_calc_iq__delta_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_62(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign22860_e21414, assign22860_e21414_d_n4, assign22860_e21414_d_n12, assign22860_e21414_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22860_e21412: f64 = (locals.var_fn277_calc_iq__vtof - locals.var_fn277_calc_iq__delta);
        (assign22860_e21412, locals.var_fn277_calc_iq__vtof_dn4, (-locals.var_fn277_calc_iq__delta_dn12), (-locals.var_fn277_calc_iq__delta_dn13),)
    } else {
        (locals.var_fn277_calc_iq__vtdibl, locals.var_fn277_calc_iq__vtdibl_dn4, locals.var_fn277_calc_iq__vtdibl_dn12, locals.var_fn277_calc_iq__vtdibl_dn13,)
    }
};
        locals.var_fn277_calc_iq__vtdibl = assign22860_e21414;
        locals.var_fn277_calc_iq__vtdibl_dn4 = assign22860_e21414_d_n4;
        locals.var_fn277_calc_iq__vtdibl_dn12 = assign22860_e21414_d_n12;
        locals.var_fn277_calc_iq__vtdibl_dn13 = assign22860_e21414_d_n13;
        locals.var_fn277_calc_iq__vtdibl_rv = 0.0;

        let (assign22870_e21422, assign22870_e21422_d_n4, assign22870_e21422_d_n12, assign22870_e21422_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22870_e21418: f64 = (2.0 * locals.var_fn277_calc_iq__n);
        let assign22870_e21420: f64 = (assign22870_e21418 * locals.var_fn277_calc_iq__phitin);
        (assign22870_e21420, (((2.0 * locals.var_fn277_calc_iq__n_dn4) * locals.var_fn277_calc_iq__phitin) + (assign22870_e21418 * locals.var_fn277_calc_iq__phitin_dn4)), ((2.0 * locals.var_fn277_calc_iq__n_dn12) * locals.var_fn277_calc_iq__phitin), ((2.0 * locals.var_fn277_calc_iq__n_dn13) * locals.var_fn277_calc_iq__phitin),)
    } else {
        (locals.var_fn277_calc_iq__two_n_phit, locals.var_fn277_calc_iq__two_n_phit_dn4, locals.var_fn277_calc_iq__two_n_phit_dn12, locals.var_fn277_calc_iq__two_n_phit_dn13,)
    }
};
        locals.var_fn277_calc_iq__two_n_phit = assign22870_e21422;
        locals.var_fn277_calc_iq__two_n_phit_dn4 = assign22870_e21422_d_n4;
        locals.var_fn277_calc_iq__two_n_phit_dn12 = assign22870_e21422_d_n12;
        locals.var_fn277_calc_iq__two_n_phit_dn13 = assign22870_e21422_d_n13;
        locals.var_fn277_calc_iq__two_n_phit_rv = 0.0;

        let (assign22880_e21428, assign22880_e21428_d_n4, assign22880_e21428_d_n12, assign22880_e21428_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22880_e21426: f64 = (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__two_n_phit);
        (assign22880_e21426, ((locals.var_fn277_calc_iq__cgin_dn4 * locals.var_fn277_calc_iq__two_n_phit) + (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__two_n_phit_dn4)), (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__two_n_phit_dn12), (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__two_n_phit_dn13),)
    } else {
        (locals.var_fn277_calc_iq__qref, locals.var_fn277_calc_iq__qref_dn4, locals.var_fn277_calc_iq__qref_dn12, locals.var_fn277_calc_iq__qref_dn13,)
    }
};
        locals.var_fn277_calc_iq__qref = assign22880_e21428;
        locals.var_fn277_calc_iq__qref_dn4 = assign22880_e21428_d_n4;
        locals.var_fn277_calc_iq__qref_dn12 = assign22880_e21428_d_n12;
        locals.var_fn277_calc_iq__qref_dn13 = assign22880_e21428_d_n13;
        locals.var_fn277_calc_iq__qref_rv = 0.0;

        let (assign22890_e21438, assign22890_e21438_d_n2, assign22890_e21438_d_n3, assign22890_e21438_d_n4, assign22890_e21438_d_n7, assign22890_e21438_d_n12, assign22890_e21438_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign22890_e21433: f64 = (p.p51 * locals.var_fn277_calc_iq__alpha_phit);
        let assign22890_e21435: f64 = (assign22890_e21433 / 2.0);
        let assign22890_e21436: f64 = (locals.var_fn277_calc_iq__vtdibl - assign22890_e21435);
        (assign22890_e21436, 0.0, 0.0, (locals.var_fn277_calc_iq__vtdibl_dn4 - ((p.p51 * locals.var_fn277_calc_iq__alpha_phit_dn4) / 2.0)), 0.0, locals.var_fn277_calc_iq__vtdibl_dn12, locals.var_fn277_calc_iq__vtdibl_dn13,)
    } else {
        (locals.var_fn277_calc_iq__myarg, locals.var_fn277_calc_iq__myarg_dn2, locals.var_fn277_calc_iq__myarg_dn3, locals.var_fn277_calc_iq__myarg_dn4, locals.var_fn277_calc_iq__myarg_dn7, locals.var_fn277_calc_iq__myarg_dn12, locals.var_fn277_calc_iq__myarg_dn13,)
    }
};
        locals.var_fn277_calc_iq__myarg = assign22890_e21438;
        locals.var_fn277_calc_iq__myarg_dn2 = assign22890_e21438_d_n2;
        locals.var_fn277_calc_iq__myarg_dn3 = assign22890_e21438_d_n3;
        locals.var_fn277_calc_iq__myarg_dn4 = assign22890_e21438_d_n4;
        locals.var_fn277_calc_iq__myarg_dn7 = assign22890_e21438_d_n7;
        locals.var_fn277_calc_iq__myarg_dn12 = assign22890_e21438_d_n12;
        locals.var_fn277_calc_iq__myarg_dn13 = assign22890_e21438_d_n13;
        locals.var_fn277_calc_iq__myarg_rv = 0.0;

        let (assign22900_e21489, assign22900_e21489_d_n2, assign22900_e21489_d_n3, assign22900_e21489_d_n4, assign22900_e21489_d_n7, assign22900_e21489_d_n12, assign22900_e21489_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign22900_e21483, assign22900_e21483_d_n2, assign22900_e21483_d_n7, assign22900_e21483_d_n12, assign22900_e21483_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign22900_e21447: f64 = (locals.var_fn277_calc_iq__vgsin + locals.var_fn277_calc_iq__vgdin);
                let assign22900_e21450: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                let assign22900_e21453: f64 = (0.001 / p.p53);
                let assign22900_e21456: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                let assign22900_e21457: f64 = (assign22900_e21453 * assign22900_e21456);
                let assign22900_e21458: f64 = (assign22900_e21457).tanh();
                let assign22900_e21459: f64 = (assign22900_e21450 * assign22900_e21458);
                let assign22900_e21460: f64 = (assign22900_e21447 + assign22900_e21459);
                let assign22900_e21461: f64 = (0.5 * assign22900_e21460);
                (assign22900_e21461, (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn2 + locals.var_fn277_calc_iq__vgdin_dn2) + (((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2) * assign22900_e21458) + (assign22900_e21450 * ((assign22900_e21453 * (locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2)) / ((assign22900_e21457).cosh() * (assign22900_e21457).cosh())))))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn7 + locals.var_fn277_calc_iq__vgdin_dn7) + (((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7) * assign22900_e21458) + (assign22900_e21450 * ((assign22900_e21453 * (locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7)) / ((assign22900_e21457).cosh() * (assign22900_e21457).cosh())))))), (0.5 * (locals.var_fn277_calc_iq__vgdin_dn12 + (((-locals.var_fn277_calc_iq__vgdin_dn12) * assign22900_e21458) + (assign22900_e21450 * ((assign22900_e21453 * (-locals.var_fn277_calc_iq__vgdin_dn12)) / ((assign22900_e21457).cosh() * (assign22900_e21457).cosh())))))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn13 + locals.var_fn277_calc_iq__vgdin_dn13) + (((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13) * assign22900_e21458) + (assign22900_e21450 * ((assign22900_e21453 * (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13)) / ((assign22900_e21457).cosh() * (assign22900_e21457).cosh())))))),)
            } else {
                let (assign22900_e21482, assign22900_e21482_d_n2, assign22900_e21482_d_n7, assign22900_e21482_d_n12, assign22900_e21482_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign22900_e21468: f64 = (locals.var_fn277_calc_iq__vgsin + locals.var_fn277_calc_iq__vgdin);
                        let assign22900_e21471: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                        let assign22900_e21474: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                        let assign22900_e21475: f64 = (assign22900_e21471 * assign22900_e21474);
                        let assign22900_e21477: f64 = (assign22900_e21475 + p.p53);
                        let assign22900_e21478: f64 = (assign22900_e21477).sqrt();
                        let assign22900_e21479: f64 = (assign22900_e21468 + assign22900_e21478);
                        let assign22900_e21480: f64 = (0.5 * assign22900_e21479);
                        (assign22900_e21480, (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn2 + locals.var_fn277_calc_iq__vgdin_dn2) + ((((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2) * assign22900_e21474) + (assign22900_e21471 * (locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2))) / (2.0 * assign22900_e21478)))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn7 + locals.var_fn277_calc_iq__vgdin_dn7) + ((((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7) * assign22900_e21474) + (assign22900_e21471 * (locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7))) / (2.0 * assign22900_e21478)))), (0.5 * (locals.var_fn277_calc_iq__vgdin_dn12 + ((((-locals.var_fn277_calc_iq__vgdin_dn12) * assign22900_e21474) + (assign22900_e21471 * (-locals.var_fn277_calc_iq__vgdin_dn12))) / (2.0 * assign22900_e21478)))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn13 + locals.var_fn277_calc_iq__vgdin_dn13) + ((((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13) * assign22900_e21474) + (assign22900_e21471 * (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13))) / (2.0 * assign22900_e21478)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign22900_e21482, assign22900_e21482_d_n2, assign22900_e21482_d_n7, assign22900_e21482_d_n12, assign22900_e21482_d_n13,)
            }
        };
        let assign22900_e21485: f64 = (assign22900_e21483 - locals.var_fn277_calc_iq__myarg);
        let assign22900_e21487: f64 = (assign22900_e21485 / locals.var_fn277_calc_iq__alpha_phit);
        (assign22900_e21487, ((assign22900_e21483_d_n2 - locals.var_fn277_calc_iq__myarg_dn2) / locals.var_fn277_calc_iq__alpha_phit), ((-locals.var_fn277_calc_iq__myarg_dn3) / locals.var_fn277_calc_iq__alpha_phit), ((((-locals.var_fn277_calc_iq__myarg_dn4) * locals.var_fn277_calc_iq__alpha_phit) - (assign22900_e21485 * locals.var_fn277_calc_iq__alpha_phit_dn4)) / (locals.var_fn277_calc_iq__alpha_phit * locals.var_fn277_calc_iq__alpha_phit)), ((assign22900_e21483_d_n7 - locals.var_fn277_calc_iq__myarg_dn7) / locals.var_fn277_calc_iq__alpha_phit), ((assign22900_e21483_d_n12 - locals.var_fn277_calc_iq__myarg_dn12) / locals.var_fn277_calc_iq__alpha_phit), ((assign22900_e21483_d_n13 - locals.var_fn277_calc_iq__myarg_dn13) / locals.var_fn277_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign22900_e21489;
        locals.var_fn277_calc_iq__exparg_dn2 = assign22900_e21489_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign22900_e21489_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign22900_e21489_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign22900_e21489_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign22900_e21489_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign22900_e21489_d_n13;
        locals.var_fn277_calc_iq__exparg_rv = 0.0;

        let assign22910_e21492: f64 = if locals.var_fn277_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard279 = assign22910_e21492;
        locals.var_guard279_rv = 0.0;

        let (assign22920_e21498, assign22920_e21498_d_n2, assign22920_e21498_d_n3, assign22920_e21498_d_n4, assign22920_e21498_d_n7, assign22920_e21498_d_n12, assign22920_e21498_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard279 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ff, locals.var_fn277_calc_iq__ff_dn2, locals.var_fn277_calc_iq__ff_dn3, locals.var_fn277_calc_iq__ff_dn4, locals.var_fn277_calc_iq__ff_dn7, locals.var_fn277_calc_iq__ff_dn12, locals.var_fn277_calc_iq__ff_dn13,)
    }
};
        locals.var_fn277_calc_iq__ff = assign22920_e21498;
        locals.var_fn277_calc_iq__ff_dn2 = assign22920_e21498_d_n2;
        locals.var_fn277_calc_iq__ff_dn3 = assign22920_e21498_d_n3;
        locals.var_fn277_calc_iq__ff_dn4 = assign22920_e21498_d_n4;
        locals.var_fn277_calc_iq__ff_dn7 = assign22920_e21498_d_n7;
        locals.var_fn277_calc_iq__ff_dn12 = assign22920_e21498_d_n12;
        locals.var_fn277_calc_iq__ff_dn13 = assign22920_e21498_d_n13;
        locals.var_fn277_calc_iq__ff_rv = 0.0;

        let assign22930_e21501: f64 = (-50.0);
        let assign22930_e21502: f64 = if locals.var_fn277_calc_iq__exparg < assign22930_e21501 { 1.0 } else { 0.0 };
        locals.var_guard280 = assign22930_e21502;
        locals.var_guard280_rv = 0.0;

        let (assign22940_e21511, assign22940_e21511_d_n2, assign22940_e21511_d_n3, assign22940_e21511_d_n4, assign22940_e21511_d_n7, assign22940_e21511_d_n12, assign22940_e21511_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard279 == 0.0)) && (locals.var_guard280 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ff, locals.var_fn277_calc_iq__ff_dn2, locals.var_fn277_calc_iq__ff_dn3, locals.var_fn277_calc_iq__ff_dn4, locals.var_fn277_calc_iq__ff_dn7, locals.var_fn277_calc_iq__ff_dn12, locals.var_fn277_calc_iq__ff_dn13,)
    }
};
        locals.var_fn277_calc_iq__ff = assign22940_e21511;
        locals.var_fn277_calc_iq__ff_dn2 = assign22940_e21511_d_n2;
        locals.var_fn277_calc_iq__ff_dn3 = assign22940_e21511_d_n3;
        locals.var_fn277_calc_iq__ff_dn4 = assign22940_e21511_d_n4;
        locals.var_fn277_calc_iq__ff_dn7 = assign22940_e21511_d_n7;
        locals.var_fn277_calc_iq__ff_dn12 = assign22940_e21511_d_n12;
        locals.var_fn277_calc_iq__ff_dn13 = assign22940_e21511_d_n13;
        locals.var_fn277_calc_iq__ff_rv = 0.0;

        let (assign22950_e21526, assign22950_e21526_d_n2, assign22950_e21526_d_n3, assign22950_e21526_d_n4, assign22950_e21526_d_n7, assign22950_e21526_d_n12, assign22950_e21526_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard279 == 0.0)) && (locals.var_guard280 == 0.0)) {
        let assign22950_e21522: f64 = (locals.var_fn277_calc_iq__exparg).exp();
        let assign22950_e21523: f64 = (1.0 + assign22950_e21522);
        let assign22950_e21524: f64 = (1.0 / assign22950_e21523);
        (assign22950_e21524, (-((assign22950_e21522 * locals.var_fn277_calc_iq__exparg_dn2) / (assign22950_e21523 * assign22950_e21523))), (-((assign22950_e21522 * locals.var_fn277_calc_iq__exparg_dn3) / (assign22950_e21523 * assign22950_e21523))), (-((assign22950_e21522 * locals.var_fn277_calc_iq__exparg_dn4) / (assign22950_e21523 * assign22950_e21523))), (-((assign22950_e21522 * locals.var_fn277_calc_iq__exparg_dn7) / (assign22950_e21523 * assign22950_e21523))), (-((assign22950_e21522 * locals.var_fn277_calc_iq__exparg_dn12) / (assign22950_e21523 * assign22950_e21523))), (-((assign22950_e21522 * locals.var_fn277_calc_iq__exparg_dn13) / (assign22950_e21523 * assign22950_e21523))),)
    } else {
        (locals.var_fn277_calc_iq__ff, locals.var_fn277_calc_iq__ff_dn2, locals.var_fn277_calc_iq__ff_dn3, locals.var_fn277_calc_iq__ff_dn4, locals.var_fn277_calc_iq__ff_dn7, locals.var_fn277_calc_iq__ff_dn12, locals.var_fn277_calc_iq__ff_dn13,)
    }
};
        locals.var_fn277_calc_iq__ff = assign22950_e21526;
        locals.var_fn277_calc_iq__ff_dn2 = assign22950_e21526_d_n2;
        locals.var_fn277_calc_iq__ff_dn3 = assign22950_e21526_d_n3;
        locals.var_fn277_calc_iq__ff_dn4 = assign22950_e21526_d_n4;
        locals.var_fn277_calc_iq__ff_dn7 = assign22950_e21526_d_n7;
        locals.var_fn277_calc_iq__ff_dn12 = assign22950_e21526_d_n12;
        locals.var_fn277_calc_iq__ff_dn13 = assign22950_e21526_d_n13;
        locals.var_fn277_calc_iq__ff_rv = 0.0;

        let (assign22960_e21585, assign22960_e21585_d_n2, assign22960_e21585_d_n3, assign22960_e21585_d_n4, assign22960_e21585_d_n7, assign22960_e21585_d_n12, assign22960_e21585_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign22960_e21571, assign22960_e21571_d_n2, assign22960_e21571_d_n7, assign22960_e21571_d_n12, assign22960_e21571_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign22960_e21535: f64 = (locals.var_fn277_calc_iq__vgsin + locals.var_fn277_calc_iq__vgdin);
                let assign22960_e21538: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                let assign22960_e21541: f64 = (0.001 / p.p53);
                let assign22960_e21544: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                let assign22960_e21545: f64 = (assign22960_e21541 * assign22960_e21544);
                let assign22960_e21546: f64 = (assign22960_e21545).tanh();
                let assign22960_e21547: f64 = (assign22960_e21538 * assign22960_e21546);
                let assign22960_e21548: f64 = (assign22960_e21535 + assign22960_e21547);
                let assign22960_e21549: f64 = (0.5 * assign22960_e21548);
                (assign22960_e21549, (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn2 + locals.var_fn277_calc_iq__vgdin_dn2) + (((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2) * assign22960_e21546) + (assign22960_e21538 * ((assign22960_e21541 * (locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2)) / ((assign22960_e21545).cosh() * (assign22960_e21545).cosh())))))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn7 + locals.var_fn277_calc_iq__vgdin_dn7) + (((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7) * assign22960_e21546) + (assign22960_e21538 * ((assign22960_e21541 * (locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7)) / ((assign22960_e21545).cosh() * (assign22960_e21545).cosh())))))), (0.5 * (locals.var_fn277_calc_iq__vgdin_dn12 + (((-locals.var_fn277_calc_iq__vgdin_dn12) * assign22960_e21546) + (assign22960_e21538 * ((assign22960_e21541 * (-locals.var_fn277_calc_iq__vgdin_dn12)) / ((assign22960_e21545).cosh() * (assign22960_e21545).cosh())))))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn13 + locals.var_fn277_calc_iq__vgdin_dn13) + (((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13) * assign22960_e21546) + (assign22960_e21538 * ((assign22960_e21541 * (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13)) / ((assign22960_e21545).cosh() * (assign22960_e21545).cosh())))))),)
            } else {
                let (assign22960_e21570, assign22960_e21570_d_n2, assign22960_e21570_d_n7, assign22960_e21570_d_n12, assign22960_e21570_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign22960_e21556: f64 = (locals.var_fn277_calc_iq__vgsin + locals.var_fn277_calc_iq__vgdin);
                        let assign22960_e21559: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                        let assign22960_e21562: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                        let assign22960_e21563: f64 = (assign22960_e21559 * assign22960_e21562);
                        let assign22960_e21565: f64 = (assign22960_e21563 + p.p53);
                        let assign22960_e21566: f64 = (assign22960_e21565).sqrt();
                        let assign22960_e21567: f64 = (assign22960_e21556 + assign22960_e21566);
                        let assign22960_e21568: f64 = (0.5 * assign22960_e21567);
                        (assign22960_e21568, (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn2 + locals.var_fn277_calc_iq__vgdin_dn2) + ((((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2) * assign22960_e21562) + (assign22960_e21559 * (locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2))) / (2.0 * assign22960_e21566)))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn7 + locals.var_fn277_calc_iq__vgdin_dn7) + ((((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7) * assign22960_e21562) + (assign22960_e21559 * (locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7))) / (2.0 * assign22960_e21566)))), (0.5 * (locals.var_fn277_calc_iq__vgdin_dn12 + ((((-locals.var_fn277_calc_iq__vgdin_dn12) * assign22960_e21562) + (assign22960_e21559 * (-locals.var_fn277_calc_iq__vgdin_dn12))) / (2.0 * assign22960_e21566)))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn13 + locals.var_fn277_calc_iq__vgdin_dn13) + ((((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13) * assign22960_e21562) + (assign22960_e21559 * (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13))) / (2.0 * assign22960_e21566)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign22960_e21570, assign22960_e21570_d_n2, assign22960_e21570_d_n7, assign22960_e21570_d_n12, assign22960_e21570_d_n13,)
            }
        };
        let assign22960_e21575: f64 = (p.p51 * 0.1);
        let assign22960_e21577: f64 = (assign22960_e21575 * locals.var_fn277_calc_iq__alpha_phit);
        let assign22960_e21579: f64 = (assign22960_e21577 * locals.var_fn277_calc_iq__ff);
        let assign22960_e21580: f64 = (locals.var_fn277_calc_iq__vtdibl - assign22960_e21579);
        let assign22960_e21581: f64 = (assign22960_e21571 - assign22960_e21580);
        let assign22960_e21583: f64 = (assign22960_e21581 / locals.var_fn277_calc_iq__two_n_phit);
        (assign22960_e21583, ((assign22960_e21571_d_n2 - (-(assign22960_e21577 * locals.var_fn277_calc_iq__ff_dn2))) / locals.var_fn277_calc_iq__two_n_phit), ((-(-(assign22960_e21577 * locals.var_fn277_calc_iq__ff_dn3))) / locals.var_fn277_calc_iq__two_n_phit), ((((-(locals.var_fn277_calc_iq__vtdibl_dn4 - (((assign22960_e21575 * locals.var_fn277_calc_iq__alpha_phit_dn4) * locals.var_fn277_calc_iq__ff) + (assign22960_e21577 * locals.var_fn277_calc_iq__ff_dn4)))) * locals.var_fn277_calc_iq__two_n_phit) - (assign22960_e21581 * locals.var_fn277_calc_iq__two_n_phit_dn4)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)), ((assign22960_e21571_d_n7 - (-(assign22960_e21577 * locals.var_fn277_calc_iq__ff_dn7))) / locals.var_fn277_calc_iq__two_n_phit), ((((assign22960_e21571_d_n12 - (locals.var_fn277_calc_iq__vtdibl_dn12 - (assign22960_e21577 * locals.var_fn277_calc_iq__ff_dn12))) * locals.var_fn277_calc_iq__two_n_phit) - (assign22960_e21581 * locals.var_fn277_calc_iq__two_n_phit_dn12)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)), ((((assign22960_e21571_d_n13 - (locals.var_fn277_calc_iq__vtdibl_dn13 - (assign22960_e21577 * locals.var_fn277_calc_iq__ff_dn13))) * locals.var_fn277_calc_iq__two_n_phit) - (assign22960_e21581 * locals.var_fn277_calc_iq__two_n_phit_dn13)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn277_calc_iq__eta, locals.var_fn277_calc_iq__eta_dn2, locals.var_fn277_calc_iq__eta_dn3, locals.var_fn277_calc_iq__eta_dn4, locals.var_fn277_calc_iq__eta_dn7, locals.var_fn277_calc_iq__eta_dn12, locals.var_fn277_calc_iq__eta_dn13,)
    }
};
        locals.var_fn277_calc_iq__eta = assign22960_e21585;
        locals.var_fn277_calc_iq__eta_dn2 = assign22960_e21585_d_n2;
        locals.var_fn277_calc_iq__eta_dn3 = assign22960_e21585_d_n3;
        locals.var_fn277_calc_iq__eta_dn4 = assign22960_e21585_d_n4;
        locals.var_fn277_calc_iq__eta_dn7 = assign22960_e21585_d_n7;
        locals.var_fn277_calc_iq__eta_dn12 = assign22960_e21585_d_n12;
        locals.var_fn277_calc_iq__eta_dn13 = assign22960_e21585_d_n13;
        locals.var_fn277_calc_iq__eta_rv = 0.0;

        let assign22970_e21588: f64 = if locals.var_fn277_calc_iq__eta > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard281 = assign22970_e21588;
        locals.var_guard281_rv = 0.0;

        let (assign22980_e21596, assign22980_e21596_d_n2, assign22980_e21596_d_n3, assign22980_e21596_d_n4, assign22980_e21596_d_n7, assign22980_e21596_d_n12, assign22980_e21596_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard281 != 0.0)) {
        let assign22980_e21594: f64 = (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__eta);
        (assign22980_e21594, (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__eta_dn2), (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__eta_dn3), ((locals.var_fn277_calc_iq__qref_dn4 * locals.var_fn277_calc_iq__eta) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__eta_dn4)), (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__eta_dn7), ((locals.var_fn277_calc_iq__qref_dn12 * locals.var_fn277_calc_iq__eta) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__eta_dn12)), ((locals.var_fn277_calc_iq__qref_dn13 * locals.var_fn277_calc_iq__eta) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__eta_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qinvv, locals.var_fn277_calc_iq__qinvv_dn2, locals.var_fn277_calc_iq__qinvv_dn3, locals.var_fn277_calc_iq__qinvv_dn4, locals.var_fn277_calc_iq__qinvv_dn7, locals.var_fn277_calc_iq__qinvv_dn12, locals.var_fn277_calc_iq__qinvv_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvv = assign22980_e21596;
        locals.var_fn277_calc_iq__qinvv_dn2 = assign22980_e21596_d_n2;
        locals.var_fn277_calc_iq__qinvv_dn3 = assign22980_e21596_d_n3;
        locals.var_fn277_calc_iq__qinvv_dn4 = assign22980_e21596_d_n4;
        locals.var_fn277_calc_iq__qinvv_dn7 = assign22980_e21596_d_n7;
        locals.var_fn277_calc_iq__qinvv_dn12 = assign22980_e21596_d_n12;
        locals.var_fn277_calc_iq__qinvv_dn13 = assign22980_e21596_d_n13;
        locals.var_fn277_calc_iq__qinvv_rv = 0.0;

        let assign22990_e21599: f64 = (-50.0);
        let assign22990_e21600: f64 = if locals.var_fn277_calc_iq__eta < assign22990_e21599 { 1.0 } else { 0.0 };
        locals.var_guard282 = assign22990_e21600;
        locals.var_guard282_rv = 0.0;

        let (assign23000_e21612, assign23000_e21612_d_n2, assign23000_e21612_d_n3, assign23000_e21612_d_n4, assign23000_e21612_d_n7, assign23000_e21612_d_n12, assign23000_e21612_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard281 == 0.0)) && (locals.var_guard282 != 0.0)) {
        let assign23000_e21609: f64 = (locals.var_fn277_calc_iq__eta).exp();
        let assign23000_e21610: f64 = (locals.var_fn277_calc_iq__qref * assign23000_e21609);
        (assign23000_e21610, (locals.var_fn277_calc_iq__qref * (assign23000_e21609 * locals.var_fn277_calc_iq__eta_dn2)), (locals.var_fn277_calc_iq__qref * (assign23000_e21609 * locals.var_fn277_calc_iq__eta_dn3)), ((locals.var_fn277_calc_iq__qref_dn4 * assign23000_e21609) + (locals.var_fn277_calc_iq__qref * (assign23000_e21609 * locals.var_fn277_calc_iq__eta_dn4))), (locals.var_fn277_calc_iq__qref * (assign23000_e21609 * locals.var_fn277_calc_iq__eta_dn7)), ((locals.var_fn277_calc_iq__qref_dn12 * assign23000_e21609) + (locals.var_fn277_calc_iq__qref * (assign23000_e21609 * locals.var_fn277_calc_iq__eta_dn12))), ((locals.var_fn277_calc_iq__qref_dn13 * assign23000_e21609) + (locals.var_fn277_calc_iq__qref * (assign23000_e21609 * locals.var_fn277_calc_iq__eta_dn13))),)
    } else {
        (locals.var_fn277_calc_iq__qinvv, locals.var_fn277_calc_iq__qinvv_dn2, locals.var_fn277_calc_iq__qinvv_dn3, locals.var_fn277_calc_iq__qinvv_dn4, locals.var_fn277_calc_iq__qinvv_dn7, locals.var_fn277_calc_iq__qinvv_dn12, locals.var_fn277_calc_iq__qinvv_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvv = assign23000_e21612;
        locals.var_fn277_calc_iq__qinvv_dn2 = assign23000_e21612_d_n2;
        locals.var_fn277_calc_iq__qinvv_dn3 = assign23000_e21612_d_n3;
        locals.var_fn277_calc_iq__qinvv_dn4 = assign23000_e21612_d_n4;
        locals.var_fn277_calc_iq__qinvv_dn7 = assign23000_e21612_d_n7;
        locals.var_fn277_calc_iq__qinvv_dn12 = assign23000_e21612_d_n12;
        locals.var_fn277_calc_iq__qinvv_dn13 = assign23000_e21612_d_n13;
        locals.var_fn277_calc_iq__qinvv_rv = 0.0;

        let (assign23010_e21628, assign23010_e21628_d_n2, assign23010_e21628_d_n3, assign23010_e21628_d_n4, assign23010_e21628_d_n7, assign23010_e21628_d_n12, assign23010_e21628_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard281 == 0.0)) && (locals.var_guard282 == 0.0)) {
        let assign23010_e21623: f64 = (locals.var_fn277_calc_iq__eta).exp();
        let assign23010_e21624: f64 = (1.0 + assign23010_e21623);
        let assign23010_e21625: f64 = (assign23010_e21624).ln();
        let assign23010_e21626: f64 = (locals.var_fn277_calc_iq__qref * assign23010_e21625);
        (assign23010_e21626, (locals.var_fn277_calc_iq__qref * ((assign23010_e21623 * locals.var_fn277_calc_iq__eta_dn2) / assign23010_e21624)), (locals.var_fn277_calc_iq__qref * ((assign23010_e21623 * locals.var_fn277_calc_iq__eta_dn3) / assign23010_e21624)), ((locals.var_fn277_calc_iq__qref_dn4 * assign23010_e21625) + (locals.var_fn277_calc_iq__qref * ((assign23010_e21623 * locals.var_fn277_calc_iq__eta_dn4) / assign23010_e21624))), (locals.var_fn277_calc_iq__qref * ((assign23010_e21623 * locals.var_fn277_calc_iq__eta_dn7) / assign23010_e21624)), ((locals.var_fn277_calc_iq__qref_dn12 * assign23010_e21625) + (locals.var_fn277_calc_iq__qref * ((assign23010_e21623 * locals.var_fn277_calc_iq__eta_dn12) / assign23010_e21624))), ((locals.var_fn277_calc_iq__qref_dn13 * assign23010_e21625) + (locals.var_fn277_calc_iq__qref * ((assign23010_e21623 * locals.var_fn277_calc_iq__eta_dn13) / assign23010_e21624))),)
    } else {
        (locals.var_fn277_calc_iq__qinvv, locals.var_fn277_calc_iq__qinvv_dn2, locals.var_fn277_calc_iq__qinvv_dn3, locals.var_fn277_calc_iq__qinvv_dn4, locals.var_fn277_calc_iq__qinvv_dn7, locals.var_fn277_calc_iq__qinvv_dn12, locals.var_fn277_calc_iq__qinvv_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvv = assign23010_e21628;
        locals.var_fn277_calc_iq__qinvv_dn2 = assign23010_e21628_d_n2;
        locals.var_fn277_calc_iq__qinvv_dn3 = assign23010_e21628_d_n3;
        locals.var_fn277_calc_iq__qinvv_dn4 = assign23010_e21628_d_n4;
        locals.var_fn277_calc_iq__qinvv_dn7 = assign23010_e21628_d_n7;
        locals.var_fn277_calc_iq__qinvv_dn12 = assign23010_e21628_d_n12;
        locals.var_fn277_calc_iq__qinvv_dn13 = assign23010_e21628_d_n13;
        locals.var_fn277_calc_iq__qinvv_rv = 0.0;

        let (assign23020_e21642, assign23020_e21642_d_n2, assign23020_e21642_d_n3, assign23020_e21642_d_n4, assign23020_e21642_d_n7, assign23020_e21642_d_n12, assign23020_e21642_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23020_e21635: f64 = (locals.var_fn277_calc_iq__mtheta * locals.var_fn277_calc_iq__qinvv);
        let assign23020_e21637: f64 = (assign23020_e21635 / locals.var_fn277_calc_iq__cgin);
        let assign23020_e21638: f64 = (1.0 + assign23020_e21637);
        let assign23020_e21639: f64 = (locals.var_fn277_calc_iq__tfacmobin * assign23020_e21638);
        let assign23020_e21640: f64 = (locals.var_fn277_calc_iq__mu0 / assign23020_e21639);
        (assign23020_e21640, (-((locals.var_fn277_calc_iq__mu0 * (locals.var_fn277_calc_iq__tfacmobin * ((locals.var_fn277_calc_iq__mtheta * locals.var_fn277_calc_iq__qinvv_dn2) / locals.var_fn277_calc_iq__cgin))) / (assign23020_e21639 * assign23020_e21639))), (-((locals.var_fn277_calc_iq__mu0 * (locals.var_fn277_calc_iq__tfacmobin * ((locals.var_fn277_calc_iq__mtheta * locals.var_fn277_calc_iq__qinvv_dn3) / locals.var_fn277_calc_iq__cgin))) / (assign23020_e21639 * assign23020_e21639))), (-((locals.var_fn277_calc_iq__mu0 * ((locals.var_fn277_calc_iq__tfacmobin_dn4 * assign23020_e21638) + (locals.var_fn277_calc_iq__tfacmobin * ((((locals.var_fn277_calc_iq__mtheta * locals.var_fn277_calc_iq__qinvv_dn4) * locals.var_fn277_calc_iq__cgin) - (assign23020_e21635 * locals.var_fn277_calc_iq__cgin_dn4)) / (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__cgin))))) / (assign23020_e21639 * assign23020_e21639))), (-((locals.var_fn277_calc_iq__mu0 * (locals.var_fn277_calc_iq__tfacmobin * ((locals.var_fn277_calc_iq__mtheta * locals.var_fn277_calc_iq__qinvv_dn7) / locals.var_fn277_calc_iq__cgin))) / (assign23020_e21639 * assign23020_e21639))), (-((locals.var_fn277_calc_iq__mu0 * (locals.var_fn277_calc_iq__tfacmobin * ((locals.var_fn277_calc_iq__mtheta * locals.var_fn277_calc_iq__qinvv_dn12) / locals.var_fn277_calc_iq__cgin))) / (assign23020_e21639 * assign23020_e21639))), (-((locals.var_fn277_calc_iq__mu0 * (locals.var_fn277_calc_iq__tfacmobin * ((locals.var_fn277_calc_iq__mtheta * locals.var_fn277_calc_iq__qinvv_dn13) / locals.var_fn277_calc_iq__cgin))) / (assign23020_e21639 * assign23020_e21639))),)
    } else {
        (locals.var_fn277_calc_iq__muf, locals.var_fn277_calc_iq__muf_dn2, locals.var_fn277_calc_iq__muf_dn3, locals.var_fn277_calc_iq__muf_dn4, locals.var_fn277_calc_iq__muf_dn7, locals.var_fn277_calc_iq__muf_dn12, locals.var_fn277_calc_iq__muf_dn13,)
    }
};
        locals.var_fn277_calc_iq__muf = assign23020_e21642;
        locals.var_fn277_calc_iq__muf_dn2 = assign23020_e21642_d_n2;
        locals.var_fn277_calc_iq__muf_dn3 = assign23020_e21642_d_n3;
        locals.var_fn277_calc_iq__muf_dn4 = assign23020_e21642_d_n4;
        locals.var_fn277_calc_iq__muf_dn7 = assign23020_e21642_d_n7;
        locals.var_fn277_calc_iq__muf_dn12 = assign23020_e21642_d_n12;
        locals.var_fn277_calc_iq__muf_dn13 = assign23020_e21642_d_n13;
        locals.var_fn277_calc_iq__muf_rv = 0.0;

        let (assign23030_e21674, assign23030_e21674_d_n2, assign23030_e21674_d_n3, assign23030_e21674_d_n4, assign23030_e21674_d_n7, assign23030_e21674_d_n12, assign23030_e21674_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23030_e21648: f64 = (locals.var_fn277_calc_iq__vzeta * locals.var_fn277_calc_iq__tnomin);
        let assign23030_e21649: f64 = (1.0 + assign23030_e21648);
        let assign23030_e21653: f64 = (locals.var_fn277_calc_iq__vzeta * locals.var_fn277_calc_iq__tambin);
        let assign23030_e21654: f64 = (1.0 + assign23030_e21653);
        let assign23030_e21655: f64 = (assign23030_e21649 / assign23030_e21654);
        let assign23030_e21656: f64 = (locals.var_fn277_calc_iq__vel0 * assign23030_e21655);
        let assign23030_e21660: f64 = (locals.var_fn277_calc_iq__lambda * locals.var_fn277_calc_iq__absvdsin);
        let assign23030_e21662: f64 = (assign23030_e21660 / locals.var_fn277_calc_iq__lin);
        let assign23030_e21663: f64 = (1.0 + assign23030_e21662);
        let assign23030_e21664: f64 = (assign23030_e21656 * assign23030_e21663);
        let assign23030_e21668: f64 = (locals.var_fn277_calc_iq__vtheta * locals.var_fn277_calc_iq__qinvv);
        let assign23030_e21670: f64 = (assign23030_e21668 / locals.var_fn277_calc_iq__cgin);
        let assign23030_e21671: f64 = (1.0 + assign23030_e21670);
        let assign23030_e21672: f64 = (assign23030_e21664 / assign23030_e21671);
        (assign23030_e21672, (-((assign23030_e21664 * ((locals.var_fn277_calc_iq__vtheta * locals.var_fn277_calc_iq__qinvv_dn2) / locals.var_fn277_calc_iq__cgin)) / (assign23030_e21671 * assign23030_e21671))), (-((assign23030_e21664 * ((locals.var_fn277_calc_iq__vtheta * locals.var_fn277_calc_iq__qinvv_dn3) / locals.var_fn277_calc_iq__cgin)) / (assign23030_e21671 * assign23030_e21671))), (((((locals.var_fn277_calc_iq__vel0 * (-((assign23030_e21649 * (locals.var_fn277_calc_iq__vzeta * locals.var_fn277_calc_iq__tambin_dn4)) / (assign23030_e21654 * assign23030_e21654)))) * assign23030_e21663) * assign23030_e21671) - (assign23030_e21664 * ((((locals.var_fn277_calc_iq__vtheta * locals.var_fn277_calc_iq__qinvv_dn4) * locals.var_fn277_calc_iq__cgin) - (assign23030_e21668 * locals.var_fn277_calc_iq__cgin_dn4)) / (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__cgin)))) / (assign23030_e21671 * assign23030_e21671)), (-((assign23030_e21664 * ((locals.var_fn277_calc_iq__vtheta * locals.var_fn277_calc_iq__qinvv_dn7) / locals.var_fn277_calc_iq__cgin)) / (assign23030_e21671 * assign23030_e21671))), ((((assign23030_e21656 * ((locals.var_fn277_calc_iq__lambda * locals.var_fn277_calc_iq__absvdsin_dn12) / locals.var_fn277_calc_iq__lin)) * assign23030_e21671) - (assign23030_e21664 * ((locals.var_fn277_calc_iq__vtheta * locals.var_fn277_calc_iq__qinvv_dn12) / locals.var_fn277_calc_iq__cgin))) / (assign23030_e21671 * assign23030_e21671)), ((((assign23030_e21656 * ((locals.var_fn277_calc_iq__lambda * locals.var_fn277_calc_iq__absvdsin_dn13) / locals.var_fn277_calc_iq__lin)) * assign23030_e21671) - (assign23030_e21664 * ((locals.var_fn277_calc_iq__vtheta * locals.var_fn277_calc_iq__qinvv_dn13) / locals.var_fn277_calc_iq__cgin))) / (assign23030_e21671 * assign23030_e21671)),)
    } else {
        (locals.var_fn277_calc_iq__vx, locals.var_fn277_calc_iq__vx_dn2, locals.var_fn277_calc_iq__vx_dn3, locals.var_fn277_calc_iq__vx_dn4, locals.var_fn277_calc_iq__vx_dn7, locals.var_fn277_calc_iq__vx_dn12, locals.var_fn277_calc_iq__vx_dn13,)
    }
};
        locals.var_fn277_calc_iq__vx = assign23030_e21674;
        locals.var_fn277_calc_iq__vx_dn2 = assign23030_e21674_d_n2;
        locals.var_fn277_calc_iq__vx_dn3 = assign23030_e21674_d_n3;
        locals.var_fn277_calc_iq__vx_dn4 = assign23030_e21674_d_n4;
        locals.var_fn277_calc_iq__vx_dn7 = assign23030_e21674_d_n7;
        locals.var_fn277_calc_iq__vx_dn12 = assign23030_e21674_d_n12;
        locals.var_fn277_calc_iq__vx_dn13 = assign23030_e21674_d_n13;
        locals.var_fn277_calc_iq__vx_rv = 0.0;

        let (assign23050_e21700, assign23050_e21700_d_n2, assign23050_e21700_d_n3, assign23050_e21700_d_n4, assign23050_e21700_d_n7, assign23050_e21700_d_n12, assign23050_e21700_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23050_e21696: f64 = (locals.var_fn277_calc_iq__vx * locals.var_fn277_calc_iq__lin);
        let assign23050_e21698: f64 = (assign23050_e21696 / locals.var_fn277_calc_iq__muf);
        (assign23050_e21698, ((((locals.var_fn277_calc_iq__vx_dn2 * locals.var_fn277_calc_iq__lin) * locals.var_fn277_calc_iq__muf) - (assign23050_e21696 * locals.var_fn277_calc_iq__muf_dn2)) / (locals.var_fn277_calc_iq__muf * locals.var_fn277_calc_iq__muf)), ((((locals.var_fn277_calc_iq__vx_dn3 * locals.var_fn277_calc_iq__lin) * locals.var_fn277_calc_iq__muf) - (assign23050_e21696 * locals.var_fn277_calc_iq__muf_dn3)) / (locals.var_fn277_calc_iq__muf * locals.var_fn277_calc_iq__muf)), ((((locals.var_fn277_calc_iq__vx_dn4 * locals.var_fn277_calc_iq__lin) * locals.var_fn277_calc_iq__muf) - (assign23050_e21696 * locals.var_fn277_calc_iq__muf_dn4)) / (locals.var_fn277_calc_iq__muf * locals.var_fn277_calc_iq__muf)), ((((locals.var_fn277_calc_iq__vx_dn7 * locals.var_fn277_calc_iq__lin) * locals.var_fn277_calc_iq__muf) - (assign23050_e21696 * locals.var_fn277_calc_iq__muf_dn7)) / (locals.var_fn277_calc_iq__muf * locals.var_fn277_calc_iq__muf)), ((((locals.var_fn277_calc_iq__vx_dn12 * locals.var_fn277_calc_iq__lin) * locals.var_fn277_calc_iq__muf) - (assign23050_e21696 * locals.var_fn277_calc_iq__muf_dn12)) / (locals.var_fn277_calc_iq__muf * locals.var_fn277_calc_iq__muf)), ((((locals.var_fn277_calc_iq__vx_dn13 * locals.var_fn277_calc_iq__lin) * locals.var_fn277_calc_iq__muf) - (assign23050_e21696 * locals.var_fn277_calc_iq__muf_dn13)) / (locals.var_fn277_calc_iq__muf * locals.var_fn277_calc_iq__muf)),)
    } else {
        (locals.var_fn277_calc_iq__vdsats, locals.var_fn277_calc_iq__vdsats_dn2, locals.var_fn277_calc_iq__vdsats_dn3, locals.var_fn277_calc_iq__vdsats_dn4, locals.var_fn277_calc_iq__vdsats_dn7, locals.var_fn277_calc_iq__vdsats_dn12, locals.var_fn277_calc_iq__vdsats_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsats = assign23050_e21700;
        locals.var_fn277_calc_iq__vdsats_dn2 = assign23050_e21700_d_n2;
        locals.var_fn277_calc_iq__vdsats_dn3 = assign23050_e21700_d_n3;
        locals.var_fn277_calc_iq__vdsats_dn4 = assign23050_e21700_d_n4;
        locals.var_fn277_calc_iq__vdsats_dn7 = assign23050_e21700_d_n7;
        locals.var_fn277_calc_iq__vdsats_dn12 = assign23050_e21700_d_n12;
        locals.var_fn277_calc_iq__vdsats_dn13 = assign23050_e21700_d_n13;
        locals.var_fn277_calc_iq__vdsats_rv = 0.0;

        let (assign23060_e21717, assign23060_e21717_d_n2, assign23060_e21717_d_n3, assign23060_e21717_d_n4, assign23060_e21717_d_n7, assign23060_e21717_d_n12, assign23060_e21717_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23060_e21706: f64 = (2.0 * locals.var_fn277_calc_iq__qinvv);
        let assign23060_e21708: f64 = (assign23060_e21706 / locals.var_fn277_calc_iq__cgin);
        let assign23060_e21710: f64 = (assign23060_e21708 / locals.var_fn277_calc_iq__vdsats);
        let assign23060_e21711: f64 = (1.0 + assign23060_e21710);
        let assign23060_e21712: f64 = (assign23060_e21711).sqrt();
        let assign23060_e21713: f64 = (locals.var_fn277_calc_iq__vdsats * assign23060_e21712);
        let assign23060_e21715: f64 = (assign23060_e21713 - locals.var_fn277_calc_iq__vdsats);
        (assign23060_e21715, (((locals.var_fn277_calc_iq__vdsats_dn2 * assign23060_e21712) + (locals.var_fn277_calc_iq__vdsats * ((((((2.0 * locals.var_fn277_calc_iq__qinvv_dn2) / locals.var_fn277_calc_iq__cgin) * locals.var_fn277_calc_iq__vdsats) - (assign23060_e21708 * locals.var_fn277_calc_iq__vdsats_dn2)) / (locals.var_fn277_calc_iq__vdsats * locals.var_fn277_calc_iq__vdsats)) / (2.0 * assign23060_e21712)))) - locals.var_fn277_calc_iq__vdsats_dn2), (((locals.var_fn277_calc_iq__vdsats_dn3 * assign23060_e21712) + (locals.var_fn277_calc_iq__vdsats * ((((((2.0 * locals.var_fn277_calc_iq__qinvv_dn3) / locals.var_fn277_calc_iq__cgin) * locals.var_fn277_calc_iq__vdsats) - (assign23060_e21708 * locals.var_fn277_calc_iq__vdsats_dn3)) / (locals.var_fn277_calc_iq__vdsats * locals.var_fn277_calc_iq__vdsats)) / (2.0 * assign23060_e21712)))) - locals.var_fn277_calc_iq__vdsats_dn3), (((locals.var_fn277_calc_iq__vdsats_dn4 * assign23060_e21712) + (locals.var_fn277_calc_iq__vdsats * ((((((((2.0 * locals.var_fn277_calc_iq__qinvv_dn4) * locals.var_fn277_calc_iq__cgin) - (assign23060_e21706 * locals.var_fn277_calc_iq__cgin_dn4)) / (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__cgin)) * locals.var_fn277_calc_iq__vdsats) - (assign23060_e21708 * locals.var_fn277_calc_iq__vdsats_dn4)) / (locals.var_fn277_calc_iq__vdsats * locals.var_fn277_calc_iq__vdsats)) / (2.0 * assign23060_e21712)))) - locals.var_fn277_calc_iq__vdsats_dn4), (((locals.var_fn277_calc_iq__vdsats_dn7 * assign23060_e21712) + (locals.var_fn277_calc_iq__vdsats * ((((((2.0 * locals.var_fn277_calc_iq__qinvv_dn7) / locals.var_fn277_calc_iq__cgin) * locals.var_fn277_calc_iq__vdsats) - (assign23060_e21708 * locals.var_fn277_calc_iq__vdsats_dn7)) / (locals.var_fn277_calc_iq__vdsats * locals.var_fn277_calc_iq__vdsats)) / (2.0 * assign23060_e21712)))) - locals.var_fn277_calc_iq__vdsats_dn7), (((locals.var_fn277_calc_iq__vdsats_dn12 * assign23060_e21712) + (locals.var_fn277_calc_iq__vdsats * ((((((2.0 * locals.var_fn277_calc_iq__qinvv_dn12) / locals.var_fn277_calc_iq__cgin) * locals.var_fn277_calc_iq__vdsats) - (assign23060_e21708 * locals.var_fn277_calc_iq__vdsats_dn12)) / (locals.var_fn277_calc_iq__vdsats * locals.var_fn277_calc_iq__vdsats)) / (2.0 * assign23060_e21712)))) - locals.var_fn277_calc_iq__vdsats_dn12), (((locals.var_fn277_calc_iq__vdsats_dn13 * assign23060_e21712) + (locals.var_fn277_calc_iq__vdsats * ((((((2.0 * locals.var_fn277_calc_iq__qinvv_dn13) / locals.var_fn277_calc_iq__cgin) * locals.var_fn277_calc_iq__vdsats) - (assign23060_e21708 * locals.var_fn277_calc_iq__vdsats_dn13)) / (locals.var_fn277_calc_iq__vdsats * locals.var_fn277_calc_iq__vdsats)) / (2.0 * assign23060_e21712)))) - locals.var_fn277_calc_iq__vdsats_dn13),)
    } else {
        (locals.var_fn277_calc_iq__vdsats1, locals.var_fn277_calc_iq__vdsats1_dn2, locals.var_fn277_calc_iq__vdsats1_dn3, locals.var_fn277_calc_iq__vdsats1_dn4, locals.var_fn277_calc_iq__vdsats1_dn7, locals.var_fn277_calc_iq__vdsats1_dn12, locals.var_fn277_calc_iq__vdsats1_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsats1 = assign23060_e21717;
        locals.var_fn277_calc_iq__vdsats1_dn2 = assign23060_e21717_d_n2;
        locals.var_fn277_calc_iq__vdsats1_dn3 = assign23060_e21717_d_n3;
        locals.var_fn277_calc_iq__vdsats1_dn4 = assign23060_e21717_d_n4;
        locals.var_fn277_calc_iq__vdsats1_dn7 = assign23060_e21717_d_n7;
        locals.var_fn277_calc_iq__vdsats1_dn12 = assign23060_e21717_d_n12;
        locals.var_fn277_calc_iq__vdsats1_dn13 = assign23060_e21717_d_n13;
        locals.var_fn277_calc_iq__vdsats1_rv = 0.0;

        let (assign23070_e21729, assign23070_e21729_d_n2, assign23070_e21729_d_n3, assign23070_e21729_d_n4, assign23070_e21729_d_n7, assign23070_e21729_d_n12, assign23070_e21729_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23070_e21722: f64 = (1.0 - locals.var_fn277_calc_iq__ff);
        let assign23070_e21723: f64 = (locals.var_fn277_calc_iq__vdsats * assign23070_e21722);
        let assign23070_e21726: f64 = (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff);
        let assign23070_e21727: f64 = (assign23070_e21723 + assign23070_e21726);
        (assign23070_e21727, (((locals.var_fn277_calc_iq__vdsats_dn2 * assign23070_e21722) + (locals.var_fn277_calc_iq__vdsats * (-locals.var_fn277_calc_iq__ff_dn2))) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn2)), (((locals.var_fn277_calc_iq__vdsats_dn3 * assign23070_e21722) + (locals.var_fn277_calc_iq__vdsats * (-locals.var_fn277_calc_iq__ff_dn3))) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn3)), (((locals.var_fn277_calc_iq__vdsats_dn4 * assign23070_e21722) + (locals.var_fn277_calc_iq__vdsats * (-locals.var_fn277_calc_iq__ff_dn4))) + ((locals.var_fn277_calc_iq__two_n_phit_dn4 * locals.var_fn277_calc_iq__ff) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn4))), (((locals.var_fn277_calc_iq__vdsats_dn7 * assign23070_e21722) + (locals.var_fn277_calc_iq__vdsats * (-locals.var_fn277_calc_iq__ff_dn7))) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn7)), (((locals.var_fn277_calc_iq__vdsats_dn12 * assign23070_e21722) + (locals.var_fn277_calc_iq__vdsats * (-locals.var_fn277_calc_iq__ff_dn12))) + ((locals.var_fn277_calc_iq__two_n_phit_dn12 * locals.var_fn277_calc_iq__ff) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn12))), (((locals.var_fn277_calc_iq__vdsats_dn13 * assign23070_e21722) + (locals.var_fn277_calc_iq__vdsats * (-locals.var_fn277_calc_iq__ff_dn13))) + ((locals.var_fn277_calc_iq__two_n_phit_dn13 * locals.var_fn277_calc_iq__ff) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn13))),)
    } else {
        (locals.var_fn277_calc_iq__vdsat, locals.var_fn277_calc_iq__vdsat_dn2, locals.var_fn277_calc_iq__vdsat_dn3, locals.var_fn277_calc_iq__vdsat_dn4, locals.var_fn277_calc_iq__vdsat_dn7, locals.var_fn277_calc_iq__vdsat_dn12, locals.var_fn277_calc_iq__vdsat_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsat = assign23070_e21729;
        locals.var_fn277_calc_iq__vdsat_dn2 = assign23070_e21729_d_n2;
        locals.var_fn277_calc_iq__vdsat_dn3 = assign23070_e21729_d_n3;
        locals.var_fn277_calc_iq__vdsat_dn4 = assign23070_e21729_d_n4;
        locals.var_fn277_calc_iq__vdsat_dn7 = assign23070_e21729_d_n7;
        locals.var_fn277_calc_iq__vdsat_dn12 = assign23070_e21729_d_n12;
        locals.var_fn277_calc_iq__vdsat_dn13 = assign23070_e21729_d_n13;
        locals.var_fn277_calc_iq__vdsat_rv = 0.0;

        let (assign23080_e21741, assign23080_e21741_d_n2, assign23080_e21741_d_n3, assign23080_e21741_d_n4, assign23080_e21741_d_n7, assign23080_e21741_d_n12, assign23080_e21741_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23080_e21734: f64 = (1.0 - locals.var_fn277_calc_iq__ff);
        let assign23080_e21735: f64 = (locals.var_fn277_calc_iq__vdsats1 * assign23080_e21734);
        let assign23080_e21738: f64 = (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff);
        let assign23080_e21739: f64 = (assign23080_e21735 + assign23080_e21738);
        (assign23080_e21739, (((locals.var_fn277_calc_iq__vdsats1_dn2 * assign23080_e21734) + (locals.var_fn277_calc_iq__vdsats1 * (-locals.var_fn277_calc_iq__ff_dn2))) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn2)), (((locals.var_fn277_calc_iq__vdsats1_dn3 * assign23080_e21734) + (locals.var_fn277_calc_iq__vdsats1 * (-locals.var_fn277_calc_iq__ff_dn3))) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn3)), (((locals.var_fn277_calc_iq__vdsats1_dn4 * assign23080_e21734) + (locals.var_fn277_calc_iq__vdsats1 * (-locals.var_fn277_calc_iq__ff_dn4))) + ((locals.var_fn277_calc_iq__two_n_phit_dn4 * locals.var_fn277_calc_iq__ff) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn4))), (((locals.var_fn277_calc_iq__vdsats1_dn7 * assign23080_e21734) + (locals.var_fn277_calc_iq__vdsats1 * (-locals.var_fn277_calc_iq__ff_dn7))) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn7)), (((locals.var_fn277_calc_iq__vdsats1_dn12 * assign23080_e21734) + (locals.var_fn277_calc_iq__vdsats1 * (-locals.var_fn277_calc_iq__ff_dn12))) + ((locals.var_fn277_calc_iq__two_n_phit_dn12 * locals.var_fn277_calc_iq__ff) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn12))), (((locals.var_fn277_calc_iq__vdsats1_dn13 * assign23080_e21734) + (locals.var_fn277_calc_iq__vdsats1 * (-locals.var_fn277_calc_iq__ff_dn13))) + ((locals.var_fn277_calc_iq__two_n_phit_dn13 * locals.var_fn277_calc_iq__ff) + (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__ff_dn13))),)
    } else {
        (locals.var_fn277_calc_iq__vdsat1, locals.var_fn277_calc_iq__vdsat1_dn2, locals.var_fn277_calc_iq__vdsat1_dn3, locals.var_fn277_calc_iq__vdsat1_dn4, locals.var_fn277_calc_iq__vdsat1_dn7, locals.var_fn277_calc_iq__vdsat1_dn12, locals.var_fn277_calc_iq__vdsat1_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsat1 = assign23080_e21741;
        locals.var_fn277_calc_iq__vdsat1_dn2 = assign23080_e21741_d_n2;
        locals.var_fn277_calc_iq__vdsat1_dn3 = assign23080_e21741_d_n3;
        locals.var_fn277_calc_iq__vdsat1_dn4 = assign23080_e21741_d_n4;
        locals.var_fn277_calc_iq__vdsat1_dn7 = assign23080_e21741_d_n7;
        locals.var_fn277_calc_iq__vdsat1_dn12 = assign23080_e21741_d_n12;
        locals.var_fn277_calc_iq__vdsat1_dn13 = assign23080_e21741_d_n13;
        locals.var_fn277_calc_iq__vdsat1_rv = 0.0;

        let (assign23090_e21810, assign23090_e21810_d_n2, assign23090_e21810_d_n3, assign23090_e21810_d_n4, assign23090_e21810_d_n7, assign23090_e21810_d_n12, assign23090_e21810_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign23090_e21800, assign23090_e21800_d_n2, assign23090_e21800_d_n3, assign23090_e21800_d_n4, assign23090_e21800_d_n7, assign23090_e21800_d_n12, assign23090_e21800_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign23090_e21753: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat1);
                let assign23090_e21754: f64 = assign23090_e21753;
                let assign23090_e21758: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat1);
                let assign23090_e21759: f64 = (-assign23090_e21758);
                let assign23090_e21762: f64 = (0.001 / p.p53);
                let assign23090_e21766: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat1);
                let assign23090_e21767: f64 = (-assign23090_e21766);
                let assign23090_e21768: f64 = (assign23090_e21762 * assign23090_e21767);
                let assign23090_e21769: f64 = (assign23090_e21768).tanh();
                let assign23090_e21770: f64 = (assign23090_e21759 * assign23090_e21769);
                let assign23090_e21771: f64 = (assign23090_e21754 + assign23090_e21770);
                let assign23090_e21772: f64 = (0.5 * assign23090_e21771);
                (assign23090_e21772, (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21769) + (assign23090_e21759 * ((assign23090_e21762 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23090_e21768).cosh() * (assign23090_e21768).cosh())))))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21769) + (assign23090_e21759 * ((assign23090_e21762 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23090_e21768).cosh() * (assign23090_e21768).cosh())))))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21769) + (assign23090_e21759 * ((assign23090_e21762 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23090_e21768).cosh() * (assign23090_e21768).cosh())))))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21769) + (assign23090_e21759 * ((assign23090_e21762 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23090_e21768).cosh() * (assign23090_e21768).cosh())))))), (0.5 * ((((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + (((-(((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23090_e21769) + (assign23090_e21759 * ((assign23090_e21762 * (-(((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) / ((assign23090_e21768).cosh() * (assign23090_e21768).cosh())))))), (0.5 * ((((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + (((-(((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23090_e21769) + (assign23090_e21759 * ((assign23090_e21762 * (-(((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) / ((assign23090_e21768).cosh() * (assign23090_e21768).cosh())))))),)
            } else {
                let (assign23090_e21799, assign23090_e21799_d_n2, assign23090_e21799_d_n3, assign23090_e21799_d_n4, assign23090_e21799_d_n7, assign23090_e21799_d_n12, assign23090_e21799_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign23090_e21780: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat1);
                        let assign23090_e21781: f64 = assign23090_e21780;
                        let assign23090_e21785: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat1);
                        let assign23090_e21786: f64 = (-assign23090_e21785);
                        let assign23090_e21790: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat1);
                        let assign23090_e21791: f64 = (-assign23090_e21790);
                        let assign23090_e21792: f64 = (assign23090_e21786 * assign23090_e21791);
                        let assign23090_e21794: f64 = (assign23090_e21792 + p.p53);
                        let assign23090_e21795: f64 = (assign23090_e21794).sqrt();
                        let assign23090_e21796: f64 = (assign23090_e21781 + assign23090_e21795);
                        let assign23090_e21797: f64 = (0.5 * assign23090_e21796);
                        (assign23090_e21797, (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21791) + (assign23090_e21786 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23090_e21795)))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21791) + (assign23090_e21786 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23090_e21795)))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21791) + (assign23090_e21786 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23090_e21795)))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23090_e21791) + (assign23090_e21786 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23090_e21795)))), (0.5 * ((((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + ((((-(((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23090_e21791) + (assign23090_e21786 * (-(((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / (2.0 * assign23090_e21795)))), (0.5 * ((((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + ((((-(((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23090_e21791) + (assign23090_e21786 * (-(((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat1) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / (2.0 * assign23090_e21795)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign23090_e21799, assign23090_e21799_d_n2, assign23090_e21799_d_n3, assign23090_e21799_d_n4, assign23090_e21799_d_n7, assign23090_e21799_d_n12, assign23090_e21799_d_n13,)
            }
        };
        let assign23090_e21802: f64 = (assign23090_e21800).powf(locals.var_fn277_calc_iq__beta);
        let assign23090_e21803: f64 = (1.0 + assign23090_e21802);
        let assign23090_e21806: f64 = (1.0 / locals.var_fn277_calc_iq__beta);
        let assign23090_e21807: f64 = (assign23090_e21803).powf(assign23090_e21806);
        let assign23090_e21808: f64 = (1.0 / assign23090_e21807);
        (assign23090_e21808, (-(if 0.0 == 0.0 && ((assign23090_e21806) as f64).is_finite() && ((assign23090_e21806) as f64).fract() == 0.0 { if assign23090_e21806 == 0.0 { 0.0 } else { (assign23090_e21806 * ((assign23090_e21803).powf(assign23090_e21806 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n2)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n2 / assign23090_e21800))) })) } } else { (assign23090_e21807 * (assign23090_e21806 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n2)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n2 / assign23090_e21800))) } / assign23090_e21803))) } / (assign23090_e21807 * assign23090_e21807))), (-(if 0.0 == 0.0 && ((assign23090_e21806) as f64).is_finite() && ((assign23090_e21806) as f64).fract() == 0.0 { if assign23090_e21806 == 0.0 { 0.0 } else { (assign23090_e21806 * ((assign23090_e21803).powf(assign23090_e21806 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n3)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n3 / assign23090_e21800))) })) } } else { (assign23090_e21807 * (assign23090_e21806 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n3)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n3 / assign23090_e21800))) } / assign23090_e21803))) } / (assign23090_e21807 * assign23090_e21807))), (-(if 0.0 == 0.0 && ((assign23090_e21806) as f64).is_finite() && ((assign23090_e21806) as f64).fract() == 0.0 { if assign23090_e21806 == 0.0 { 0.0 } else { (assign23090_e21806 * ((assign23090_e21803).powf(assign23090_e21806 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n4)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n4 / assign23090_e21800))) })) } } else { (assign23090_e21807 * (assign23090_e21806 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n4)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n4 / assign23090_e21800))) } / assign23090_e21803))) } / (assign23090_e21807 * assign23090_e21807))), (-(if 0.0 == 0.0 && ((assign23090_e21806) as f64).is_finite() && ((assign23090_e21806) as f64).fract() == 0.0 { if assign23090_e21806 == 0.0 { 0.0 } else { (assign23090_e21806 * ((assign23090_e21803).powf(assign23090_e21806 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n7)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n7 / assign23090_e21800))) })) } } else { (assign23090_e21807 * (assign23090_e21806 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n7)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n7 / assign23090_e21800))) } / assign23090_e21803))) } / (assign23090_e21807 * assign23090_e21807))), (-(if 0.0 == 0.0 && ((assign23090_e21806) as f64).is_finite() && ((assign23090_e21806) as f64).fract() == 0.0 { if assign23090_e21806 == 0.0 { 0.0 } else { (assign23090_e21806 * ((assign23090_e21803).powf(assign23090_e21806 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n12)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n12 / assign23090_e21800))) })) } } else { (assign23090_e21807 * (assign23090_e21806 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n12)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n12 / assign23090_e21800))) } / assign23090_e21803))) } / (assign23090_e21807 * assign23090_e21807))), (-(if 0.0 == 0.0 && ((assign23090_e21806) as f64).is_finite() && ((assign23090_e21806) as f64).fract() == 0.0 { if assign23090_e21806 == 0.0 { 0.0 } else { (assign23090_e21806 * ((assign23090_e21803).powf(assign23090_e21806 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n13)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n13 / assign23090_e21800))) })) } } else { (assign23090_e21807 * (assign23090_e21806 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23090_e21800).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23090_e21800_d_n13)) } } else { (assign23090_e21802 * (locals.var_fn277_calc_iq__beta * (assign23090_e21800_d_n13 / assign23090_e21800))) } / assign23090_e21803))) } / (assign23090_e21807 * assign23090_e21807))),)
    } else {
        (locals.var_fn277_calc_iq__fsd, locals.var_fn277_calc_iq__fsd_dn2, locals.var_fn277_calc_iq__fsd_dn3, locals.var_fn277_calc_iq__fsd_dn4, locals.var_fn277_calc_iq__fsd_dn7, locals.var_fn277_calc_iq__fsd_dn12, locals.var_fn277_calc_iq__fsd_dn13,)
    }
};
        locals.var_fn277_calc_iq__fsd = assign23090_e21810;
        locals.var_fn277_calc_iq__fsd_dn2 = assign23090_e21810_d_n2;
        locals.var_fn277_calc_iq__fsd_dn3 = assign23090_e21810_d_n3;
        locals.var_fn277_calc_iq__fsd_dn4 = assign23090_e21810_d_n4;
        locals.var_fn277_calc_iq__fsd_dn7 = assign23090_e21810_d_n7;
        locals.var_fn277_calc_iq__fsd_dn12 = assign23090_e21810_d_n12;
        locals.var_fn277_calc_iq__fsd_dn13 = assign23090_e21810_d_n13;
        locals.var_fn277_calc_iq__fsd_rv = 0.0;

        let (assign23100_e21816, assign23100_e21816_d_n2, assign23100_e21816_d_n3, assign23100_e21816_d_n4, assign23100_e21816_d_n7, assign23100_e21816_d_n12, assign23100_e21816_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23100_e21814: f64 = (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd);
        (assign23100_e21814, (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd_dn2), (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd_dn3), (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd_dn4), (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd_dn7), ((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__fsd) + (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd_dn12)), ((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__fsd) + (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__vdx, locals.var_fn277_calc_iq__vdx_dn2, locals.var_fn277_calc_iq__vdx_dn3, locals.var_fn277_calc_iq__vdx_dn4, locals.var_fn277_calc_iq__vdx_dn7, locals.var_fn277_calc_iq__vdx_dn12, locals.var_fn277_calc_iq__vdx_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdx = assign23100_e21816;
        locals.var_fn277_calc_iq__vdx_dn2 = assign23100_e21816_d_n2;
        locals.var_fn277_calc_iq__vdx_dn3 = assign23100_e21816_d_n3;
        locals.var_fn277_calc_iq__vdx_dn4 = assign23100_e21816_d_n4;
        locals.var_fn277_calc_iq__vdx_dn7 = assign23100_e21816_d_n7;
        locals.var_fn277_calc_iq__vdx_dn12 = assign23100_e21816_d_n12;
        locals.var_fn277_calc_iq__vdx_dn13 = assign23100_e21816_d_n13;
        locals.var_fn277_calc_iq__vdx_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_63(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23110_e21891, assign23110_e21891_d_n2, assign23110_e21891_d_n3, assign23110_e21891_d_n4, assign23110_e21891_d_n7, assign23110_e21891_d_n12, assign23110_e21891_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign23110_e21881, assign23110_e21881_d_n2, assign23110_e21881_d_n3, assign23110_e21881_d_n4, assign23110_e21881_d_n7, assign23110_e21881_d_n12, assign23110_e21881_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign23110_e21827: f64 = (-locals.var_fn277_calc_iq__vdsin);
                let assign23110_e21829: f64 = (assign23110_e21827 / locals.var_fn277_calc_iq__vdsat1);
                let assign23110_e21830: f64 = assign23110_e21829;
                let assign23110_e21833: f64 = (-locals.var_fn277_calc_iq__vdsin);
                let assign23110_e21835: f64 = (assign23110_e21833 / locals.var_fn277_calc_iq__vdsat1);
                let assign23110_e21836: f64 = (-assign23110_e21835);
                let assign23110_e21839: f64 = (0.001 / p.p53);
                let assign23110_e21842: f64 = (-locals.var_fn277_calc_iq__vdsin);
                let assign23110_e21844: f64 = (assign23110_e21842 / locals.var_fn277_calc_iq__vdsat1);
                let assign23110_e21845: f64 = (-assign23110_e21844);
                let assign23110_e21846: f64 = (assign23110_e21839 * assign23110_e21845);
                let assign23110_e21847: f64 = (assign23110_e21846).tanh();
                let assign23110_e21848: f64 = (assign23110_e21836 * assign23110_e21847);
                let assign23110_e21849: f64 = (assign23110_e21830 + assign23110_e21848);
                let assign23110_e21850: f64 = (0.5 * assign23110_e21849);
                (assign23110_e21850, (0.5 * ((-((assign23110_e21827 * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((assign23110_e21833 * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21847) + (assign23110_e21836 * ((assign23110_e21839 * (-(-((assign23110_e21842 * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23110_e21846).cosh() * (assign23110_e21846).cosh())))))), (0.5 * ((-((assign23110_e21827 * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((assign23110_e21833 * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21847) + (assign23110_e21836 * ((assign23110_e21839 * (-(-((assign23110_e21842 * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23110_e21846).cosh() * (assign23110_e21846).cosh())))))), (0.5 * ((-((assign23110_e21827 * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((assign23110_e21833 * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21847) + (assign23110_e21836 * ((assign23110_e21839 * (-(-((assign23110_e21842 * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23110_e21846).cosh() * (assign23110_e21846).cosh())))))), (0.5 * ((-((assign23110_e21827 * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + (((-(-((assign23110_e21833 * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21847) + (assign23110_e21836 * ((assign23110_e21839 * (-(-((assign23110_e21842 * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / ((assign23110_e21846).cosh() * (assign23110_e21846).cosh())))))), (0.5 * (((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21827 * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + (((-((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21833 * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23110_e21847) + (assign23110_e21836 * ((assign23110_e21839 * (-((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21842 * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) / ((assign23110_e21846).cosh() * (assign23110_e21846).cosh())))))), (0.5 * (((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21827 * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + (((-((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21833 * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23110_e21847) + (assign23110_e21836 * ((assign23110_e21839 * (-((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21842 * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) / ((assign23110_e21846).cosh() * (assign23110_e21846).cosh())))))),)
            } else {
                let (assign23110_e21880, assign23110_e21880_d_n2, assign23110_e21880_d_n3, assign23110_e21880_d_n4, assign23110_e21880_d_n7, assign23110_e21880_d_n12, assign23110_e21880_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign23110_e21857: f64 = (-locals.var_fn277_calc_iq__vdsin);
                        let assign23110_e21859: f64 = (assign23110_e21857 / locals.var_fn277_calc_iq__vdsat1);
                        let assign23110_e21860: f64 = assign23110_e21859;
                        let assign23110_e21863: f64 = (-locals.var_fn277_calc_iq__vdsin);
                        let assign23110_e21865: f64 = (assign23110_e21863 / locals.var_fn277_calc_iq__vdsat1);
                        let assign23110_e21866: f64 = (-assign23110_e21865);
                        let assign23110_e21869: f64 = (-locals.var_fn277_calc_iq__vdsin);
                        let assign23110_e21871: f64 = (assign23110_e21869 / locals.var_fn277_calc_iq__vdsat1);
                        let assign23110_e21872: f64 = (-assign23110_e21871);
                        let assign23110_e21873: f64 = (assign23110_e21866 * assign23110_e21872);
                        let assign23110_e21875: f64 = (assign23110_e21873 + p.p53);
                        let assign23110_e21876: f64 = (assign23110_e21875).sqrt();
                        let assign23110_e21877: f64 = (assign23110_e21860 + assign23110_e21876);
                        let assign23110_e21878: f64 = (0.5 * assign23110_e21877);
                        (assign23110_e21878, (0.5 * ((-((assign23110_e21857 * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((assign23110_e21863 * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21872) + (assign23110_e21866 * (-(-((assign23110_e21869 * locals.var_fn277_calc_iq__vdsat1_dn2) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23110_e21876)))), (0.5 * ((-((assign23110_e21857 * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((assign23110_e21863 * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21872) + (assign23110_e21866 * (-(-((assign23110_e21869 * locals.var_fn277_calc_iq__vdsat1_dn3) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23110_e21876)))), (0.5 * ((-((assign23110_e21857 * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((assign23110_e21863 * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21872) + (assign23110_e21866 * (-(-((assign23110_e21869 * locals.var_fn277_calc_iq__vdsat1_dn4) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23110_e21876)))), (0.5 * ((-((assign23110_e21857 * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) + ((((-(-((assign23110_e21863 * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))) * assign23110_e21872) + (assign23110_e21866 * (-(-((assign23110_e21869 * locals.var_fn277_calc_iq__vdsat1_dn7) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)))))) / (2.0 * assign23110_e21876)))), (0.5 * (((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21857 * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + ((((-((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21863 * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23110_e21872) + (assign23110_e21866 * (-((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21869 * locals.var_fn277_calc_iq__vdsat1_dn12)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / (2.0 * assign23110_e21876)))), (0.5 * (((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21857 * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1)) + ((((-((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21863 * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))) * assign23110_e21872) + (assign23110_e21866 * (-((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat1) - (assign23110_e21869 * locals.var_fn277_calc_iq__vdsat1_dn13)) / (locals.var_fn277_calc_iq__vdsat1 * locals.var_fn277_calc_iq__vdsat1))))) / (2.0 * assign23110_e21876)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign23110_e21880, assign23110_e21880_d_n2, assign23110_e21880_d_n3, assign23110_e21880_d_n4, assign23110_e21880_d_n7, assign23110_e21880_d_n12, assign23110_e21880_d_n13,)
            }
        };
        let assign23110_e21883: f64 = (assign23110_e21881).powf(locals.var_fn277_calc_iq__beta);
        let assign23110_e21884: f64 = (1.0 + assign23110_e21883);
        let assign23110_e21887: f64 = (1.0 / locals.var_fn277_calc_iq__beta);
        let assign23110_e21888: f64 = (assign23110_e21884).powf(assign23110_e21887);
        let assign23110_e21889: f64 = (1.0 / assign23110_e21888);
        (assign23110_e21889, (-(if 0.0 == 0.0 && ((assign23110_e21887) as f64).is_finite() && ((assign23110_e21887) as f64).fract() == 0.0 { if assign23110_e21887 == 0.0 { 0.0 } else { (assign23110_e21887 * ((assign23110_e21884).powf(assign23110_e21887 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n2)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n2 / assign23110_e21881))) })) } } else { (assign23110_e21888 * (assign23110_e21887 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n2)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n2 / assign23110_e21881))) } / assign23110_e21884))) } / (assign23110_e21888 * assign23110_e21888))), (-(if 0.0 == 0.0 && ((assign23110_e21887) as f64).is_finite() && ((assign23110_e21887) as f64).fract() == 0.0 { if assign23110_e21887 == 0.0 { 0.0 } else { (assign23110_e21887 * ((assign23110_e21884).powf(assign23110_e21887 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n3)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n3 / assign23110_e21881))) })) } } else { (assign23110_e21888 * (assign23110_e21887 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n3)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n3 / assign23110_e21881))) } / assign23110_e21884))) } / (assign23110_e21888 * assign23110_e21888))), (-(if 0.0 == 0.0 && ((assign23110_e21887) as f64).is_finite() && ((assign23110_e21887) as f64).fract() == 0.0 { if assign23110_e21887 == 0.0 { 0.0 } else { (assign23110_e21887 * ((assign23110_e21884).powf(assign23110_e21887 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n4)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n4 / assign23110_e21881))) })) } } else { (assign23110_e21888 * (assign23110_e21887 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n4)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n4 / assign23110_e21881))) } / assign23110_e21884))) } / (assign23110_e21888 * assign23110_e21888))), (-(if 0.0 == 0.0 && ((assign23110_e21887) as f64).is_finite() && ((assign23110_e21887) as f64).fract() == 0.0 { if assign23110_e21887 == 0.0 { 0.0 } else { (assign23110_e21887 * ((assign23110_e21884).powf(assign23110_e21887 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n7)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n7 / assign23110_e21881))) })) } } else { (assign23110_e21888 * (assign23110_e21887 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n7)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n7 / assign23110_e21881))) } / assign23110_e21884))) } / (assign23110_e21888 * assign23110_e21888))), (-(if 0.0 == 0.0 && ((assign23110_e21887) as f64).is_finite() && ((assign23110_e21887) as f64).fract() == 0.0 { if assign23110_e21887 == 0.0 { 0.0 } else { (assign23110_e21887 * ((assign23110_e21884).powf(assign23110_e21887 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n12)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n12 / assign23110_e21881))) })) } } else { (assign23110_e21888 * (assign23110_e21887 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n12)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n12 / assign23110_e21881))) } / assign23110_e21884))) } / (assign23110_e21888 * assign23110_e21888))), (-(if 0.0 == 0.0 && ((assign23110_e21887) as f64).is_finite() && ((assign23110_e21887) as f64).fract() == 0.0 { if assign23110_e21887 == 0.0 { 0.0 } else { (assign23110_e21887 * ((assign23110_e21884).powf(assign23110_e21887 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n13)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n13 / assign23110_e21881))) })) } } else { (assign23110_e21888 * (assign23110_e21887 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23110_e21881).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23110_e21881_d_n13)) } } else { (assign23110_e21883 * (locals.var_fn277_calc_iq__beta * (assign23110_e21881_d_n13 / assign23110_e21881))) } / assign23110_e21884))) } / (assign23110_e21888 * assign23110_e21888))),)
    } else {
        (locals.var_fn277_calc_iq__fds, locals.var_fn277_calc_iq__fds_dn2, locals.var_fn277_calc_iq__fds_dn3, locals.var_fn277_calc_iq__fds_dn4, locals.var_fn277_calc_iq__fds_dn7, locals.var_fn277_calc_iq__fds_dn12, locals.var_fn277_calc_iq__fds_dn13,)
    }
};
        locals.var_fn277_calc_iq__fds = assign23110_e21891;
        locals.var_fn277_calc_iq__fds_dn2 = assign23110_e21891_d_n2;
        locals.var_fn277_calc_iq__fds_dn3 = assign23110_e21891_d_n3;
        locals.var_fn277_calc_iq__fds_dn4 = assign23110_e21891_d_n4;
        locals.var_fn277_calc_iq__fds_dn7 = assign23110_e21891_d_n7;
        locals.var_fn277_calc_iq__fds_dn12 = assign23110_e21891_d_n12;
        locals.var_fn277_calc_iq__fds_dn13 = assign23110_e21891_d_n13;
        locals.var_fn277_calc_iq__fds_rv = 0.0;

        let (assign23120_e21898, assign23120_e21898_d_n2, assign23120_e21898_d_n3, assign23120_e21898_d_n4, assign23120_e21898_d_n7, assign23120_e21898_d_n12, assign23120_e21898_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23120_e21894: f64 = (-locals.var_fn277_calc_iq__vdsin);
        let assign23120_e21896: f64 = (assign23120_e21894 * locals.var_fn277_calc_iq__fds);
        (assign23120_e21896, (assign23120_e21894 * locals.var_fn277_calc_iq__fds_dn2), (assign23120_e21894 * locals.var_fn277_calc_iq__fds_dn3), (assign23120_e21894 * locals.var_fn277_calc_iq__fds_dn4), (assign23120_e21894 * locals.var_fn277_calc_iq__fds_dn7), (((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__fds) + (assign23120_e21894 * locals.var_fn277_calc_iq__fds_dn12)), (((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__fds) + (assign23120_e21894 * locals.var_fn277_calc_iq__fds_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__vsx, locals.var_fn277_calc_iq__vsx_dn2, locals.var_fn277_calc_iq__vsx_dn3, locals.var_fn277_calc_iq__vsx_dn4, locals.var_fn277_calc_iq__vsx_dn7, locals.var_fn277_calc_iq__vsx_dn12, locals.var_fn277_calc_iq__vsx_dn13,)
    }
};
        locals.var_fn277_calc_iq__vsx = assign23120_e21898;
        locals.var_fn277_calc_iq__vsx_dn2 = assign23120_e21898_d_n2;
        locals.var_fn277_calc_iq__vsx_dn3 = assign23120_e21898_d_n3;
        locals.var_fn277_calc_iq__vsx_dn4 = assign23120_e21898_d_n4;
        locals.var_fn277_calc_iq__vsx_dn7 = assign23120_e21898_d_n7;
        locals.var_fn277_calc_iq__vsx_dn12 = assign23120_e21898_d_n12;
        locals.var_fn277_calc_iq__vsx_dn13 = assign23120_e21898_d_n13;
        locals.var_fn277_calc_iq__vsx_rv = 0.0;

        let (assign23130_e21906, assign23130_e21906_d_n2, assign23130_e21906_d_n3, assign23130_e21906_d_n4, assign23130_e21906_d_n7, assign23130_e21906_d_n12, assign23130_e21906_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23130_e21902: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__myarg);
        let assign23130_e21904: f64 = (assign23130_e21902 / locals.var_fn277_calc_iq__alpha_phit);
        (assign23130_e21904, ((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__myarg_dn2) / locals.var_fn277_calc_iq__alpha_phit), ((-locals.var_fn277_calc_iq__myarg_dn3) / locals.var_fn277_calc_iq__alpha_phit), ((((-locals.var_fn277_calc_iq__myarg_dn4) * locals.var_fn277_calc_iq__alpha_phit) - (assign23130_e21902 * locals.var_fn277_calc_iq__alpha_phit_dn4)) / (locals.var_fn277_calc_iq__alpha_phit * locals.var_fn277_calc_iq__alpha_phit)), ((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__myarg_dn7) / locals.var_fn277_calc_iq__alpha_phit), ((-locals.var_fn277_calc_iq__myarg_dn12) / locals.var_fn277_calc_iq__alpha_phit), ((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__myarg_dn13) / locals.var_fn277_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign23130_e21906;
        locals.var_fn277_calc_iq__exparg_dn2 = assign23130_e21906_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign23130_e21906_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign23130_e21906_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign23130_e21906_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign23130_e21906_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign23130_e21906_d_n13;
        locals.var_fn277_calc_iq__exparg_rv = 0.0;

        let assign23140_e21909: f64 = if locals.var_fn277_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard283 = assign23140_e21909;
        locals.var_guard283_rv = 0.0;

        let (assign23150_e21915, assign23150_e21915_d_n2, assign23150_e21915_d_n3, assign23150_e21915_d_n4, assign23150_e21915_d_n7, assign23150_e21915_d_n12, assign23150_e21915_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard283 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffs, locals.var_fn277_calc_iq__ffs_dn2, locals.var_fn277_calc_iq__ffs_dn3, locals.var_fn277_calc_iq__ffs_dn4, locals.var_fn277_calc_iq__ffs_dn7, locals.var_fn277_calc_iq__ffs_dn12, locals.var_fn277_calc_iq__ffs_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffs = assign23150_e21915;
        locals.var_fn277_calc_iq__ffs_dn2 = assign23150_e21915_d_n2;
        locals.var_fn277_calc_iq__ffs_dn3 = assign23150_e21915_d_n3;
        locals.var_fn277_calc_iq__ffs_dn4 = assign23150_e21915_d_n4;
        locals.var_fn277_calc_iq__ffs_dn7 = assign23150_e21915_d_n7;
        locals.var_fn277_calc_iq__ffs_dn12 = assign23150_e21915_d_n12;
        locals.var_fn277_calc_iq__ffs_dn13 = assign23150_e21915_d_n13;
        locals.var_fn277_calc_iq__ffs_rv = 0.0;

        let assign23160_e21918: f64 = (-50.0);
        let assign23160_e21919: f64 = if locals.var_fn277_calc_iq__exparg < assign23160_e21918 { 1.0 } else { 0.0 };
        locals.var_guard284 = assign23160_e21919;
        locals.var_guard284_rv = 0.0;

        let (assign23170_e21928, assign23170_e21928_d_n2, assign23170_e21928_d_n3, assign23170_e21928_d_n4, assign23170_e21928_d_n7, assign23170_e21928_d_n12, assign23170_e21928_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard283 == 0.0)) && (locals.var_guard284 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffs, locals.var_fn277_calc_iq__ffs_dn2, locals.var_fn277_calc_iq__ffs_dn3, locals.var_fn277_calc_iq__ffs_dn4, locals.var_fn277_calc_iq__ffs_dn7, locals.var_fn277_calc_iq__ffs_dn12, locals.var_fn277_calc_iq__ffs_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffs = assign23170_e21928;
        locals.var_fn277_calc_iq__ffs_dn2 = assign23170_e21928_d_n2;
        locals.var_fn277_calc_iq__ffs_dn3 = assign23170_e21928_d_n3;
        locals.var_fn277_calc_iq__ffs_dn4 = assign23170_e21928_d_n4;
        locals.var_fn277_calc_iq__ffs_dn7 = assign23170_e21928_d_n7;
        locals.var_fn277_calc_iq__ffs_dn12 = assign23170_e21928_d_n12;
        locals.var_fn277_calc_iq__ffs_dn13 = assign23170_e21928_d_n13;
        locals.var_fn277_calc_iq__ffs_rv = 0.0;

        let (assign23180_e21943, assign23180_e21943_d_n2, assign23180_e21943_d_n3, assign23180_e21943_d_n4, assign23180_e21943_d_n7, assign23180_e21943_d_n12, assign23180_e21943_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard283 == 0.0)) && (locals.var_guard284 == 0.0)) {
        let assign23180_e21939: f64 = (locals.var_fn277_calc_iq__exparg).exp();
        let assign23180_e21940: f64 = (1.0 + assign23180_e21939);
        let assign23180_e21941: f64 = (1.0 / assign23180_e21940);
        (assign23180_e21941, (-((assign23180_e21939 * locals.var_fn277_calc_iq__exparg_dn2) / (assign23180_e21940 * assign23180_e21940))), (-((assign23180_e21939 * locals.var_fn277_calc_iq__exparg_dn3) / (assign23180_e21940 * assign23180_e21940))), (-((assign23180_e21939 * locals.var_fn277_calc_iq__exparg_dn4) / (assign23180_e21940 * assign23180_e21940))), (-((assign23180_e21939 * locals.var_fn277_calc_iq__exparg_dn7) / (assign23180_e21940 * assign23180_e21940))), (-((assign23180_e21939 * locals.var_fn277_calc_iq__exparg_dn12) / (assign23180_e21940 * assign23180_e21940))), (-((assign23180_e21939 * locals.var_fn277_calc_iq__exparg_dn13) / (assign23180_e21940 * assign23180_e21940))),)
    } else {
        (locals.var_fn277_calc_iq__ffs, locals.var_fn277_calc_iq__ffs_dn2, locals.var_fn277_calc_iq__ffs_dn3, locals.var_fn277_calc_iq__ffs_dn4, locals.var_fn277_calc_iq__ffs_dn7, locals.var_fn277_calc_iq__ffs_dn12, locals.var_fn277_calc_iq__ffs_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffs = assign23180_e21943;
        locals.var_fn277_calc_iq__ffs_dn2 = assign23180_e21943_d_n2;
        locals.var_fn277_calc_iq__ffs_dn3 = assign23180_e21943_d_n3;
        locals.var_fn277_calc_iq__ffs_dn4 = assign23180_e21943_d_n4;
        locals.var_fn277_calc_iq__ffs_dn7 = assign23180_e21943_d_n7;
        locals.var_fn277_calc_iq__ffs_dn12 = assign23180_e21943_d_n12;
        locals.var_fn277_calc_iq__ffs_dn13 = assign23180_e21943_d_n13;
        locals.var_fn277_calc_iq__ffs_rv = 0.0;

        let (assign23190_e21961, assign23190_e21961_d_n2, assign23190_e21961_d_n3, assign23190_e21961_d_n4, assign23190_e21961_d_n7, assign23190_e21961_d_n12, assign23190_e21961_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23190_e21947: f64 = (locals.var_fn277_calc_iq__vgdin - locals.var_fn277_calc_iq__vsx);
        let assign23190_e21951: f64 = (p.p51 * 0.1);
        let assign23190_e21953: f64 = (assign23190_e21951 * locals.var_fn277_calc_iq__alpha_phit);
        let assign23190_e21955: f64 = (assign23190_e21953 * locals.var_fn277_calc_iq__ffs);
        let assign23190_e21956: f64 = (locals.var_fn277_calc_iq__vtdibl - assign23190_e21955);
        let assign23190_e21957: f64 = (assign23190_e21947 - assign23190_e21956);
        let assign23190_e21959: f64 = (assign23190_e21957 / locals.var_fn277_calc_iq__two_n_phit);
        (assign23190_e21959, (((locals.var_fn277_calc_iq__vgdin_dn2 - locals.var_fn277_calc_iq__vsx_dn2) - (-(assign23190_e21953 * locals.var_fn277_calc_iq__ffs_dn2))) / locals.var_fn277_calc_iq__two_n_phit), (((-locals.var_fn277_calc_iq__vsx_dn3) - (-(assign23190_e21953 * locals.var_fn277_calc_iq__ffs_dn3))) / locals.var_fn277_calc_iq__two_n_phit), (((((-locals.var_fn277_calc_iq__vsx_dn4) - (locals.var_fn277_calc_iq__vtdibl_dn4 - (((assign23190_e21951 * locals.var_fn277_calc_iq__alpha_phit_dn4) * locals.var_fn277_calc_iq__ffs) + (assign23190_e21953 * locals.var_fn277_calc_iq__ffs_dn4)))) * locals.var_fn277_calc_iq__two_n_phit) - (assign23190_e21957 * locals.var_fn277_calc_iq__two_n_phit_dn4)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)), (((locals.var_fn277_calc_iq__vgdin_dn7 - locals.var_fn277_calc_iq__vsx_dn7) - (-(assign23190_e21953 * locals.var_fn277_calc_iq__ffs_dn7))) / locals.var_fn277_calc_iq__two_n_phit), (((((locals.var_fn277_calc_iq__vgdin_dn12 - locals.var_fn277_calc_iq__vsx_dn12) - (locals.var_fn277_calc_iq__vtdibl_dn12 - (assign23190_e21953 * locals.var_fn277_calc_iq__ffs_dn12))) * locals.var_fn277_calc_iq__two_n_phit) - (assign23190_e21957 * locals.var_fn277_calc_iq__two_n_phit_dn12)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)), (((((locals.var_fn277_calc_iq__vgdin_dn13 - locals.var_fn277_calc_iq__vsx_dn13) - (locals.var_fn277_calc_iq__vtdibl_dn13 - (assign23190_e21953 * locals.var_fn277_calc_iq__ffs_dn13))) * locals.var_fn277_calc_iq__two_n_phit) - (assign23190_e21957 * locals.var_fn277_calc_iq__two_n_phit_dn13)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn277_calc_iq__etas, locals.var_fn277_calc_iq__etas_dn2, locals.var_fn277_calc_iq__etas_dn3, locals.var_fn277_calc_iq__etas_dn4, locals.var_fn277_calc_iq__etas_dn7, locals.var_fn277_calc_iq__etas_dn12, locals.var_fn277_calc_iq__etas_dn13,)
    }
};
        locals.var_fn277_calc_iq__etas = assign23190_e21961;
        locals.var_fn277_calc_iq__etas_dn2 = assign23190_e21961_d_n2;
        locals.var_fn277_calc_iq__etas_dn3 = assign23190_e21961_d_n3;
        locals.var_fn277_calc_iq__etas_dn4 = assign23190_e21961_d_n4;
        locals.var_fn277_calc_iq__etas_dn7 = assign23190_e21961_d_n7;
        locals.var_fn277_calc_iq__etas_dn12 = assign23190_e21961_d_n12;
        locals.var_fn277_calc_iq__etas_dn13 = assign23190_e21961_d_n13;
        locals.var_fn277_calc_iq__etas_rv = 0.0;

        let assign23200_e21964: f64 = if locals.var_fn277_calc_iq__etas > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard285 = assign23200_e21964;
        locals.var_guard285_rv = 0.0;

        let (assign23210_e21972, assign23210_e21972_d_n2, assign23210_e21972_d_n3, assign23210_e21972_d_n4, assign23210_e21972_d_n7, assign23210_e21972_d_n12, assign23210_e21972_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard285 != 0.0)) {
        let assign23210_e21970: f64 = (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etas);
        (assign23210_e21970, (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etas_dn2), (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etas_dn3), ((locals.var_fn277_calc_iq__qref_dn4 * locals.var_fn277_calc_iq__etas) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etas_dn4)), (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etas_dn7), ((locals.var_fn277_calc_iq__qref_dn12 * locals.var_fn277_calc_iq__etas) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etas_dn12)), ((locals.var_fn277_calc_iq__qref_dn13 * locals.var_fn277_calc_iq__etas) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etas_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qinvs, locals.var_fn277_calc_iq__qinvs_dn2, locals.var_fn277_calc_iq__qinvs_dn3, locals.var_fn277_calc_iq__qinvs_dn4, locals.var_fn277_calc_iq__qinvs_dn7, locals.var_fn277_calc_iq__qinvs_dn12, locals.var_fn277_calc_iq__qinvs_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvs = assign23210_e21972;
        locals.var_fn277_calc_iq__qinvs_dn2 = assign23210_e21972_d_n2;
        locals.var_fn277_calc_iq__qinvs_dn3 = assign23210_e21972_d_n3;
        locals.var_fn277_calc_iq__qinvs_dn4 = assign23210_e21972_d_n4;
        locals.var_fn277_calc_iq__qinvs_dn7 = assign23210_e21972_d_n7;
        locals.var_fn277_calc_iq__qinvs_dn12 = assign23210_e21972_d_n12;
        locals.var_fn277_calc_iq__qinvs_dn13 = assign23210_e21972_d_n13;
        locals.var_fn277_calc_iq__qinvs_rv = 0.0;

        let assign23220_e21975: f64 = (-50.0);
        let assign23220_e21976: f64 = if locals.var_fn277_calc_iq__etas < assign23220_e21975 { 1.0 } else { 0.0 };
        locals.var_guard286 = assign23220_e21976;
        locals.var_guard286_rv = 0.0;

        let (assign23230_e21988, assign23230_e21988_d_n2, assign23230_e21988_d_n3, assign23230_e21988_d_n4, assign23230_e21988_d_n7, assign23230_e21988_d_n12, assign23230_e21988_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard285 == 0.0)) && (locals.var_guard286 != 0.0)) {
        let assign23230_e21985: f64 = (locals.var_fn277_calc_iq__etas).exp();
        let assign23230_e21986: f64 = (locals.var_fn277_calc_iq__qref * assign23230_e21985);
        (assign23230_e21986, (locals.var_fn277_calc_iq__qref * (assign23230_e21985 * locals.var_fn277_calc_iq__etas_dn2)), (locals.var_fn277_calc_iq__qref * (assign23230_e21985 * locals.var_fn277_calc_iq__etas_dn3)), ((locals.var_fn277_calc_iq__qref_dn4 * assign23230_e21985) + (locals.var_fn277_calc_iq__qref * (assign23230_e21985 * locals.var_fn277_calc_iq__etas_dn4))), (locals.var_fn277_calc_iq__qref * (assign23230_e21985 * locals.var_fn277_calc_iq__etas_dn7)), ((locals.var_fn277_calc_iq__qref_dn12 * assign23230_e21985) + (locals.var_fn277_calc_iq__qref * (assign23230_e21985 * locals.var_fn277_calc_iq__etas_dn12))), ((locals.var_fn277_calc_iq__qref_dn13 * assign23230_e21985) + (locals.var_fn277_calc_iq__qref * (assign23230_e21985 * locals.var_fn277_calc_iq__etas_dn13))),)
    } else {
        (locals.var_fn277_calc_iq__qinvs, locals.var_fn277_calc_iq__qinvs_dn2, locals.var_fn277_calc_iq__qinvs_dn3, locals.var_fn277_calc_iq__qinvs_dn4, locals.var_fn277_calc_iq__qinvs_dn7, locals.var_fn277_calc_iq__qinvs_dn12, locals.var_fn277_calc_iq__qinvs_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvs = assign23230_e21988;
        locals.var_fn277_calc_iq__qinvs_dn2 = assign23230_e21988_d_n2;
        locals.var_fn277_calc_iq__qinvs_dn3 = assign23230_e21988_d_n3;
        locals.var_fn277_calc_iq__qinvs_dn4 = assign23230_e21988_d_n4;
        locals.var_fn277_calc_iq__qinvs_dn7 = assign23230_e21988_d_n7;
        locals.var_fn277_calc_iq__qinvs_dn12 = assign23230_e21988_d_n12;
        locals.var_fn277_calc_iq__qinvs_dn13 = assign23230_e21988_d_n13;
        locals.var_fn277_calc_iq__qinvs_rv = 0.0;

        let (assign23240_e22004, assign23240_e22004_d_n2, assign23240_e22004_d_n3, assign23240_e22004_d_n4, assign23240_e22004_d_n7, assign23240_e22004_d_n12, assign23240_e22004_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard285 == 0.0)) && (locals.var_guard286 == 0.0)) {
        let assign23240_e21999: f64 = (locals.var_fn277_calc_iq__etas).exp();
        let assign23240_e22000: f64 = (1.0 + assign23240_e21999);
        let assign23240_e22001: f64 = (assign23240_e22000).ln();
        let assign23240_e22002: f64 = (locals.var_fn277_calc_iq__qref * assign23240_e22001);
        (assign23240_e22002, (locals.var_fn277_calc_iq__qref * ((assign23240_e21999 * locals.var_fn277_calc_iq__etas_dn2) / assign23240_e22000)), (locals.var_fn277_calc_iq__qref * ((assign23240_e21999 * locals.var_fn277_calc_iq__etas_dn3) / assign23240_e22000)), ((locals.var_fn277_calc_iq__qref_dn4 * assign23240_e22001) + (locals.var_fn277_calc_iq__qref * ((assign23240_e21999 * locals.var_fn277_calc_iq__etas_dn4) / assign23240_e22000))), (locals.var_fn277_calc_iq__qref * ((assign23240_e21999 * locals.var_fn277_calc_iq__etas_dn7) / assign23240_e22000)), ((locals.var_fn277_calc_iq__qref_dn12 * assign23240_e22001) + (locals.var_fn277_calc_iq__qref * ((assign23240_e21999 * locals.var_fn277_calc_iq__etas_dn12) / assign23240_e22000))), ((locals.var_fn277_calc_iq__qref_dn13 * assign23240_e22001) + (locals.var_fn277_calc_iq__qref * ((assign23240_e21999 * locals.var_fn277_calc_iq__etas_dn13) / assign23240_e22000))),)
    } else {
        (locals.var_fn277_calc_iq__qinvs, locals.var_fn277_calc_iq__qinvs_dn2, locals.var_fn277_calc_iq__qinvs_dn3, locals.var_fn277_calc_iq__qinvs_dn4, locals.var_fn277_calc_iq__qinvs_dn7, locals.var_fn277_calc_iq__qinvs_dn12, locals.var_fn277_calc_iq__qinvs_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvs = assign23240_e22004;
        locals.var_fn277_calc_iq__qinvs_dn2 = assign23240_e22004_d_n2;
        locals.var_fn277_calc_iq__qinvs_dn3 = assign23240_e22004_d_n3;
        locals.var_fn277_calc_iq__qinvs_dn4 = assign23240_e22004_d_n4;
        locals.var_fn277_calc_iq__qinvs_dn7 = assign23240_e22004_d_n7;
        locals.var_fn277_calc_iq__qinvs_dn12 = assign23240_e22004_d_n12;
        locals.var_fn277_calc_iq__qinvs_dn13 = assign23240_e22004_d_n13;
        locals.var_fn277_calc_iq__qinvs_rv = 0.0;

        let (assign23250_e22012, assign23250_e22012_d_n2, assign23250_e22012_d_n3, assign23250_e22012_d_n4, assign23250_e22012_d_n7, assign23250_e22012_d_n12, assign23250_e22012_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23250_e22008: f64 = (locals.var_fn277_calc_iq__vgdin - locals.var_fn277_calc_iq__myarg);
        let assign23250_e22010: f64 = (assign23250_e22008 / locals.var_fn277_calc_iq__alpha_phit);
        (assign23250_e22010, ((locals.var_fn277_calc_iq__vgdin_dn2 - locals.var_fn277_calc_iq__myarg_dn2) / locals.var_fn277_calc_iq__alpha_phit), ((-locals.var_fn277_calc_iq__myarg_dn3) / locals.var_fn277_calc_iq__alpha_phit), ((((-locals.var_fn277_calc_iq__myarg_dn4) * locals.var_fn277_calc_iq__alpha_phit) - (assign23250_e22008 * locals.var_fn277_calc_iq__alpha_phit_dn4)) / (locals.var_fn277_calc_iq__alpha_phit * locals.var_fn277_calc_iq__alpha_phit)), ((locals.var_fn277_calc_iq__vgdin_dn7 - locals.var_fn277_calc_iq__myarg_dn7) / locals.var_fn277_calc_iq__alpha_phit), ((locals.var_fn277_calc_iq__vgdin_dn12 - locals.var_fn277_calc_iq__myarg_dn12) / locals.var_fn277_calc_iq__alpha_phit), ((locals.var_fn277_calc_iq__vgdin_dn13 - locals.var_fn277_calc_iq__myarg_dn13) / locals.var_fn277_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign23250_e22012;
        locals.var_fn277_calc_iq__exparg_dn2 = assign23250_e22012_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign23250_e22012_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign23250_e22012_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign23250_e22012_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign23250_e22012_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign23250_e22012_d_n13;
        locals.var_fn277_calc_iq__exparg_rv = 0.0;

        let assign23260_e22015: f64 = if locals.var_fn277_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard287 = assign23260_e22015;
        locals.var_guard287_rv = 0.0;

        let (assign23270_e22021, assign23270_e22021_d_n2, assign23270_e22021_d_n3, assign23270_e22021_d_n4, assign23270_e22021_d_n7, assign23270_e22021_d_n12, assign23270_e22021_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard287 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffd, locals.var_fn277_calc_iq__ffd_dn2, locals.var_fn277_calc_iq__ffd_dn3, locals.var_fn277_calc_iq__ffd_dn4, locals.var_fn277_calc_iq__ffd_dn7, locals.var_fn277_calc_iq__ffd_dn12, locals.var_fn277_calc_iq__ffd_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffd = assign23270_e22021;
        locals.var_fn277_calc_iq__ffd_dn2 = assign23270_e22021_d_n2;
        locals.var_fn277_calc_iq__ffd_dn3 = assign23270_e22021_d_n3;
        locals.var_fn277_calc_iq__ffd_dn4 = assign23270_e22021_d_n4;
        locals.var_fn277_calc_iq__ffd_dn7 = assign23270_e22021_d_n7;
        locals.var_fn277_calc_iq__ffd_dn12 = assign23270_e22021_d_n12;
        locals.var_fn277_calc_iq__ffd_dn13 = assign23270_e22021_d_n13;
        locals.var_fn277_calc_iq__ffd_rv = 0.0;

        let assign23280_e22024: f64 = (-50.0);
        let assign23280_e22025: f64 = if locals.var_fn277_calc_iq__exparg < assign23280_e22024 { 1.0 } else { 0.0 };
        locals.var_guard288 = assign23280_e22025;
        locals.var_guard288_rv = 0.0;

        let (assign23290_e22034, assign23290_e22034_d_n2, assign23290_e22034_d_n3, assign23290_e22034_d_n4, assign23290_e22034_d_n7, assign23290_e22034_d_n12, assign23290_e22034_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard287 == 0.0)) && (locals.var_guard288 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffd, locals.var_fn277_calc_iq__ffd_dn2, locals.var_fn277_calc_iq__ffd_dn3, locals.var_fn277_calc_iq__ffd_dn4, locals.var_fn277_calc_iq__ffd_dn7, locals.var_fn277_calc_iq__ffd_dn12, locals.var_fn277_calc_iq__ffd_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffd = assign23290_e22034;
        locals.var_fn277_calc_iq__ffd_dn2 = assign23290_e22034_d_n2;
        locals.var_fn277_calc_iq__ffd_dn3 = assign23290_e22034_d_n3;
        locals.var_fn277_calc_iq__ffd_dn4 = assign23290_e22034_d_n4;
        locals.var_fn277_calc_iq__ffd_dn7 = assign23290_e22034_d_n7;
        locals.var_fn277_calc_iq__ffd_dn12 = assign23290_e22034_d_n12;
        locals.var_fn277_calc_iq__ffd_dn13 = assign23290_e22034_d_n13;
        locals.var_fn277_calc_iq__ffd_rv = 0.0;

        let (assign23300_e22049, assign23300_e22049_d_n2, assign23300_e22049_d_n3, assign23300_e22049_d_n4, assign23300_e22049_d_n7, assign23300_e22049_d_n12, assign23300_e22049_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard287 == 0.0)) && (locals.var_guard288 == 0.0)) {
        let assign23300_e22045: f64 = (locals.var_fn277_calc_iq__exparg).exp();
        let assign23300_e22046: f64 = (1.0 + assign23300_e22045);
        let assign23300_e22047: f64 = (1.0 / assign23300_e22046);
        (assign23300_e22047, (-((assign23300_e22045 * locals.var_fn277_calc_iq__exparg_dn2) / (assign23300_e22046 * assign23300_e22046))), (-((assign23300_e22045 * locals.var_fn277_calc_iq__exparg_dn3) / (assign23300_e22046 * assign23300_e22046))), (-((assign23300_e22045 * locals.var_fn277_calc_iq__exparg_dn4) / (assign23300_e22046 * assign23300_e22046))), (-((assign23300_e22045 * locals.var_fn277_calc_iq__exparg_dn7) / (assign23300_e22046 * assign23300_e22046))), (-((assign23300_e22045 * locals.var_fn277_calc_iq__exparg_dn12) / (assign23300_e22046 * assign23300_e22046))), (-((assign23300_e22045 * locals.var_fn277_calc_iq__exparg_dn13) / (assign23300_e22046 * assign23300_e22046))),)
    } else {
        (locals.var_fn277_calc_iq__ffd, locals.var_fn277_calc_iq__ffd_dn2, locals.var_fn277_calc_iq__ffd_dn3, locals.var_fn277_calc_iq__ffd_dn4, locals.var_fn277_calc_iq__ffd_dn7, locals.var_fn277_calc_iq__ffd_dn12, locals.var_fn277_calc_iq__ffd_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffd = assign23300_e22049;
        locals.var_fn277_calc_iq__ffd_dn2 = assign23300_e22049_d_n2;
        locals.var_fn277_calc_iq__ffd_dn3 = assign23300_e22049_d_n3;
        locals.var_fn277_calc_iq__ffd_dn4 = assign23300_e22049_d_n4;
        locals.var_fn277_calc_iq__ffd_dn7 = assign23300_e22049_d_n7;
        locals.var_fn277_calc_iq__ffd_dn12 = assign23300_e22049_d_n12;
        locals.var_fn277_calc_iq__ffd_dn13 = assign23300_e22049_d_n13;
        locals.var_fn277_calc_iq__ffd_rv = 0.0;

        let (assign23310_e22067, assign23310_e22067_d_n2, assign23310_e22067_d_n3, assign23310_e22067_d_n4, assign23310_e22067_d_n7, assign23310_e22067_d_n12, assign23310_e22067_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23310_e22053: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vdx);
        let assign23310_e22057: f64 = (p.p51 * 0.1);
        let assign23310_e22059: f64 = (assign23310_e22057 * locals.var_fn277_calc_iq__alpha_phit);
        let assign23310_e22061: f64 = (assign23310_e22059 * locals.var_fn277_calc_iq__ffd);
        let assign23310_e22062: f64 = (locals.var_fn277_calc_iq__vtdibl - assign23310_e22061);
        let assign23310_e22063: f64 = (assign23310_e22053 - assign23310_e22062);
        let assign23310_e22065: f64 = (assign23310_e22063 / locals.var_fn277_calc_iq__two_n_phit);
        (assign23310_e22065, (((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vdx_dn2) - (-(assign23310_e22059 * locals.var_fn277_calc_iq__ffd_dn2))) / locals.var_fn277_calc_iq__two_n_phit), (((-locals.var_fn277_calc_iq__vdx_dn3) - (-(assign23310_e22059 * locals.var_fn277_calc_iq__ffd_dn3))) / locals.var_fn277_calc_iq__two_n_phit), (((((-locals.var_fn277_calc_iq__vdx_dn4) - (locals.var_fn277_calc_iq__vtdibl_dn4 - (((assign23310_e22057 * locals.var_fn277_calc_iq__alpha_phit_dn4) * locals.var_fn277_calc_iq__ffd) + (assign23310_e22059 * locals.var_fn277_calc_iq__ffd_dn4)))) * locals.var_fn277_calc_iq__two_n_phit) - (assign23310_e22063 * locals.var_fn277_calc_iq__two_n_phit_dn4)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)), (((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vdx_dn7) - (-(assign23310_e22059 * locals.var_fn277_calc_iq__ffd_dn7))) / locals.var_fn277_calc_iq__two_n_phit), (((((-locals.var_fn277_calc_iq__vdx_dn12) - (locals.var_fn277_calc_iq__vtdibl_dn12 - (assign23310_e22059 * locals.var_fn277_calc_iq__ffd_dn12))) * locals.var_fn277_calc_iq__two_n_phit) - (assign23310_e22063 * locals.var_fn277_calc_iq__two_n_phit_dn12)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)), (((((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vdx_dn13) - (locals.var_fn277_calc_iq__vtdibl_dn13 - (assign23310_e22059 * locals.var_fn277_calc_iq__ffd_dn13))) * locals.var_fn277_calc_iq__two_n_phit) - (assign23310_e22063 * locals.var_fn277_calc_iq__two_n_phit_dn13)) / (locals.var_fn277_calc_iq__two_n_phit * locals.var_fn277_calc_iq__two_n_phit)),)
    } else {
        (locals.var_fn277_calc_iq__etad, locals.var_fn277_calc_iq__etad_dn2, locals.var_fn277_calc_iq__etad_dn3, locals.var_fn277_calc_iq__etad_dn4, locals.var_fn277_calc_iq__etad_dn7, locals.var_fn277_calc_iq__etad_dn12, locals.var_fn277_calc_iq__etad_dn13,)
    }
};
        locals.var_fn277_calc_iq__etad = assign23310_e22067;
        locals.var_fn277_calc_iq__etad_dn2 = assign23310_e22067_d_n2;
        locals.var_fn277_calc_iq__etad_dn3 = assign23310_e22067_d_n3;
        locals.var_fn277_calc_iq__etad_dn4 = assign23310_e22067_d_n4;
        locals.var_fn277_calc_iq__etad_dn7 = assign23310_e22067_d_n7;
        locals.var_fn277_calc_iq__etad_dn12 = assign23310_e22067_d_n12;
        locals.var_fn277_calc_iq__etad_dn13 = assign23310_e22067_d_n13;
        locals.var_fn277_calc_iq__etad_rv = 0.0;

        let assign23320_e22070: f64 = if locals.var_fn277_calc_iq__etad > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard289 = assign23320_e22070;
        locals.var_guard289_rv = 0.0;

        let (assign23330_e22078, assign23330_e22078_d_n2, assign23330_e22078_d_n3, assign23330_e22078_d_n4, assign23330_e22078_d_n7, assign23330_e22078_d_n12, assign23330_e22078_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard289 != 0.0)) {
        let assign23330_e22076: f64 = (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etad);
        (assign23330_e22076, (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etad_dn2), (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etad_dn3), ((locals.var_fn277_calc_iq__qref_dn4 * locals.var_fn277_calc_iq__etad) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etad_dn4)), (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etad_dn7), ((locals.var_fn277_calc_iq__qref_dn12 * locals.var_fn277_calc_iq__etad) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etad_dn12)), ((locals.var_fn277_calc_iq__qref_dn13 * locals.var_fn277_calc_iq__etad) + (locals.var_fn277_calc_iq__qref * locals.var_fn277_calc_iq__etad_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qinvd, locals.var_fn277_calc_iq__qinvd_dn2, locals.var_fn277_calc_iq__qinvd_dn3, locals.var_fn277_calc_iq__qinvd_dn4, locals.var_fn277_calc_iq__qinvd_dn7, locals.var_fn277_calc_iq__qinvd_dn12, locals.var_fn277_calc_iq__qinvd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvd = assign23330_e22078;
        locals.var_fn277_calc_iq__qinvd_dn2 = assign23330_e22078_d_n2;
        locals.var_fn277_calc_iq__qinvd_dn3 = assign23330_e22078_d_n3;
        locals.var_fn277_calc_iq__qinvd_dn4 = assign23330_e22078_d_n4;
        locals.var_fn277_calc_iq__qinvd_dn7 = assign23330_e22078_d_n7;
        locals.var_fn277_calc_iq__qinvd_dn12 = assign23330_e22078_d_n12;
        locals.var_fn277_calc_iq__qinvd_dn13 = assign23330_e22078_d_n13;
        locals.var_fn277_calc_iq__qinvd_rv = 0.0;

        let assign23340_e22081: f64 = (-50.0);
        let assign23340_e22082: f64 = if locals.var_fn277_calc_iq__etad < assign23340_e22081 { 1.0 } else { 0.0 };
        locals.var_guard290 = assign23340_e22082;
        locals.var_guard290_rv = 0.0;

        let (assign23350_e22094, assign23350_e22094_d_n2, assign23350_e22094_d_n3, assign23350_e22094_d_n4, assign23350_e22094_d_n7, assign23350_e22094_d_n12, assign23350_e22094_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard289 == 0.0)) && (locals.var_guard290 != 0.0)) {
        let assign23350_e22091: f64 = (locals.var_fn277_calc_iq__etad).exp();
        let assign23350_e22092: f64 = (locals.var_fn277_calc_iq__qref * assign23350_e22091);
        (assign23350_e22092, (locals.var_fn277_calc_iq__qref * (assign23350_e22091 * locals.var_fn277_calc_iq__etad_dn2)), (locals.var_fn277_calc_iq__qref * (assign23350_e22091 * locals.var_fn277_calc_iq__etad_dn3)), ((locals.var_fn277_calc_iq__qref_dn4 * assign23350_e22091) + (locals.var_fn277_calc_iq__qref * (assign23350_e22091 * locals.var_fn277_calc_iq__etad_dn4))), (locals.var_fn277_calc_iq__qref * (assign23350_e22091 * locals.var_fn277_calc_iq__etad_dn7)), ((locals.var_fn277_calc_iq__qref_dn12 * assign23350_e22091) + (locals.var_fn277_calc_iq__qref * (assign23350_e22091 * locals.var_fn277_calc_iq__etad_dn12))), ((locals.var_fn277_calc_iq__qref_dn13 * assign23350_e22091) + (locals.var_fn277_calc_iq__qref * (assign23350_e22091 * locals.var_fn277_calc_iq__etad_dn13))),)
    } else {
        (locals.var_fn277_calc_iq__qinvd, locals.var_fn277_calc_iq__qinvd_dn2, locals.var_fn277_calc_iq__qinvd_dn3, locals.var_fn277_calc_iq__qinvd_dn4, locals.var_fn277_calc_iq__qinvd_dn7, locals.var_fn277_calc_iq__qinvd_dn12, locals.var_fn277_calc_iq__qinvd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvd = assign23350_e22094;
        locals.var_fn277_calc_iq__qinvd_dn2 = assign23350_e22094_d_n2;
        locals.var_fn277_calc_iq__qinvd_dn3 = assign23350_e22094_d_n3;
        locals.var_fn277_calc_iq__qinvd_dn4 = assign23350_e22094_d_n4;
        locals.var_fn277_calc_iq__qinvd_dn7 = assign23350_e22094_d_n7;
        locals.var_fn277_calc_iq__qinvd_dn12 = assign23350_e22094_d_n12;
        locals.var_fn277_calc_iq__qinvd_dn13 = assign23350_e22094_d_n13;
        locals.var_fn277_calc_iq__qinvd_rv = 0.0;

        let (assign23360_e22110, assign23360_e22110_d_n2, assign23360_e22110_d_n3, assign23360_e22110_d_n4, assign23360_e22110_d_n7, assign23360_e22110_d_n12, assign23360_e22110_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard289 == 0.0)) && (locals.var_guard290 == 0.0)) {
        let assign23360_e22105: f64 = (locals.var_fn277_calc_iq__etad).exp();
        let assign23360_e22106: f64 = (1.0 + assign23360_e22105);
        let assign23360_e22107: f64 = (assign23360_e22106).ln();
        let assign23360_e22108: f64 = (locals.var_fn277_calc_iq__qref * assign23360_e22107);
        (assign23360_e22108, (locals.var_fn277_calc_iq__qref * ((assign23360_e22105 * locals.var_fn277_calc_iq__etad_dn2) / assign23360_e22106)), (locals.var_fn277_calc_iq__qref * ((assign23360_e22105 * locals.var_fn277_calc_iq__etad_dn3) / assign23360_e22106)), ((locals.var_fn277_calc_iq__qref_dn4 * assign23360_e22107) + (locals.var_fn277_calc_iq__qref * ((assign23360_e22105 * locals.var_fn277_calc_iq__etad_dn4) / assign23360_e22106))), (locals.var_fn277_calc_iq__qref * ((assign23360_e22105 * locals.var_fn277_calc_iq__etad_dn7) / assign23360_e22106)), ((locals.var_fn277_calc_iq__qref_dn12 * assign23360_e22107) + (locals.var_fn277_calc_iq__qref * ((assign23360_e22105 * locals.var_fn277_calc_iq__etad_dn12) / assign23360_e22106))), ((locals.var_fn277_calc_iq__qref_dn13 * assign23360_e22107) + (locals.var_fn277_calc_iq__qref * ((assign23360_e22105 * locals.var_fn277_calc_iq__etad_dn13) / assign23360_e22106))),)
    } else {
        (locals.var_fn277_calc_iq__qinvd, locals.var_fn277_calc_iq__qinvd_dn2, locals.var_fn277_calc_iq__qinvd_dn3, locals.var_fn277_calc_iq__qinvd_dn4, locals.var_fn277_calc_iq__qinvd_dn7, locals.var_fn277_calc_iq__qinvd_dn12, locals.var_fn277_calc_iq__qinvd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvd = assign23360_e22110;
        locals.var_fn277_calc_iq__qinvd_dn2 = assign23360_e22110_d_n2;
        locals.var_fn277_calc_iq__qinvd_dn3 = assign23360_e22110_d_n3;
        locals.var_fn277_calc_iq__qinvd_dn4 = assign23360_e22110_d_n4;
        locals.var_fn277_calc_iq__qinvd_dn7 = assign23360_e22110_d_n7;
        locals.var_fn277_calc_iq__qinvd_dn12 = assign23360_e22110_d_n12;
        locals.var_fn277_calc_iq__qinvd_dn13 = assign23360_e22110_d_n13;
        locals.var_fn277_calc_iq__qinvd_rv = 0.0;

        let (assign23370_e22118, assign23370_e22118_d_n2, assign23370_e22118_d_n3, assign23370_e22118_d_n4, assign23370_e22118_d_n7, assign23370_e22118_d_n12, assign23370_e22118_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23370_e22114: f64 = (locals.var_fn277_calc_iq__qinvs - locals.var_fn277_calc_iq__qinvd);
        let assign23370_e22116: f64 = (assign23370_e22114 / locals.var_fn277_calc_iq__cgin);
        (assign23370_e22116, ((locals.var_fn277_calc_iq__qinvs_dn2 - locals.var_fn277_calc_iq__qinvd_dn2) / locals.var_fn277_calc_iq__cgin), ((locals.var_fn277_calc_iq__qinvs_dn3 - locals.var_fn277_calc_iq__qinvd_dn3) / locals.var_fn277_calc_iq__cgin), ((((locals.var_fn277_calc_iq__qinvs_dn4 - locals.var_fn277_calc_iq__qinvd_dn4) * locals.var_fn277_calc_iq__cgin) - (assign23370_e22114 * locals.var_fn277_calc_iq__cgin_dn4)) / (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__cgin)), ((locals.var_fn277_calc_iq__qinvs_dn7 - locals.var_fn277_calc_iq__qinvd_dn7) / locals.var_fn277_calc_iq__cgin), ((locals.var_fn277_calc_iq__qinvs_dn12 - locals.var_fn277_calc_iq__qinvd_dn12) / locals.var_fn277_calc_iq__cgin), ((locals.var_fn277_calc_iq__qinvs_dn13 - locals.var_fn277_calc_iq__qinvd_dn13) / locals.var_fn277_calc_iq__cgin),)
    } else {
        (locals.var_fn277_calc_iq__vdsc, locals.var_fn277_calc_iq__vdsc_dn2, locals.var_fn277_calc_iq__vdsc_dn3, locals.var_fn277_calc_iq__vdsc_dn4, locals.var_fn277_calc_iq__vdsc_dn7, locals.var_fn277_calc_iq__vdsc_dn12, locals.var_fn277_calc_iq__vdsc_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsc = assign23370_e22118;
        locals.var_fn277_calc_iq__vdsc_dn2 = assign23370_e22118_d_n2;
        locals.var_fn277_calc_iq__vdsc_dn3 = assign23370_e22118_d_n3;
        locals.var_fn277_calc_iq__vdsc_dn4 = assign23370_e22118_d_n4;
        locals.var_fn277_calc_iq__vdsc_dn7 = assign23370_e22118_d_n7;
        locals.var_fn277_calc_iq__vdsc_dn12 = assign23370_e22118_d_n12;
        locals.var_fn277_calc_iq__vdsc_dn13 = assign23370_e22118_d_n13;
        locals.var_fn277_calc_iq__vdsc_rv = 0.0;

        let (assign23380_e22124, assign23380_e22124_d_n2, assign23380_e22124_d_n3, assign23380_e22124_d_n4, assign23380_e22124_d_n7, assign23380_e22124_d_n12, assign23380_e22124_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23380_e22122: f64 = (locals.var_fn277_calc_iq__vdsc / locals.var_fn277_calc_iq__vdsat);
        (assign23380_e22122, (((locals.var_fn277_calc_iq__vdsc_dn2 * locals.var_fn277_calc_iq__vdsat) - (locals.var_fn277_calc_iq__vdsc * locals.var_fn277_calc_iq__vdsat_dn2)) / (locals.var_fn277_calc_iq__vdsat * locals.var_fn277_calc_iq__vdsat)), (((locals.var_fn277_calc_iq__vdsc_dn3 * locals.var_fn277_calc_iq__vdsat) - (locals.var_fn277_calc_iq__vdsc * locals.var_fn277_calc_iq__vdsat_dn3)) / (locals.var_fn277_calc_iq__vdsat * locals.var_fn277_calc_iq__vdsat)), (((locals.var_fn277_calc_iq__vdsc_dn4 * locals.var_fn277_calc_iq__vdsat) - (locals.var_fn277_calc_iq__vdsc * locals.var_fn277_calc_iq__vdsat_dn4)) / (locals.var_fn277_calc_iq__vdsat * locals.var_fn277_calc_iq__vdsat)), (((locals.var_fn277_calc_iq__vdsc_dn7 * locals.var_fn277_calc_iq__vdsat) - (locals.var_fn277_calc_iq__vdsc * locals.var_fn277_calc_iq__vdsat_dn7)) / (locals.var_fn277_calc_iq__vdsat * locals.var_fn277_calc_iq__vdsat)), (((locals.var_fn277_calc_iq__vdsc_dn12 * locals.var_fn277_calc_iq__vdsat) - (locals.var_fn277_calc_iq__vdsc * locals.var_fn277_calc_iq__vdsat_dn12)) / (locals.var_fn277_calc_iq__vdsat * locals.var_fn277_calc_iq__vdsat)), (((locals.var_fn277_calc_iq__vdsc_dn13 * locals.var_fn277_calc_iq__vdsat) - (locals.var_fn277_calc_iq__vdsc * locals.var_fn277_calc_iq__vdsat_dn13)) / (locals.var_fn277_calc_iq__vdsat * locals.var_fn277_calc_iq__vdsat)),)
    } else {
        (locals.var_fn277_calc_iq__myarg, locals.var_fn277_calc_iq__myarg_dn2, locals.var_fn277_calc_iq__myarg_dn3, locals.var_fn277_calc_iq__myarg_dn4, locals.var_fn277_calc_iq__myarg_dn7, locals.var_fn277_calc_iq__myarg_dn12, locals.var_fn277_calc_iq__myarg_dn13,)
    }
};
        locals.var_fn277_calc_iq__myarg = assign23380_e22124;
        locals.var_fn277_calc_iq__myarg_dn2 = assign23380_e22124_d_n2;
        locals.var_fn277_calc_iq__myarg_dn3 = assign23380_e22124_d_n3;
        locals.var_fn277_calc_iq__myarg_dn4 = assign23380_e22124_d_n4;
        locals.var_fn277_calc_iq__myarg_dn7 = assign23380_e22124_d_n7;
        locals.var_fn277_calc_iq__myarg_dn12 = assign23380_e22124_d_n12;
        locals.var_fn277_calc_iq__myarg_dn13 = assign23380_e22124_d_n13;
        locals.var_fn277_calc_iq__myarg_rv = 0.0;

        let (assign23420_e22193, assign23420_e22193_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23420_e22190: f64 = (2.302585092994046 * locals.var_fn277_calc_iq__phitin);
        let assign23420_e22191: f64 = (locals.var_fn277_calc_iq__ss / assign23420_e22190);
        (assign23420_e22191, (-((locals.var_fn277_calc_iq__ss * (2.302585092994046 * locals.var_fn277_calc_iq__phitin_dn4)) / (assign23420_e22190 * assign23420_e22190))),)
    } else {
        (locals.var_fn277_calc_iq__n0, locals.var_fn277_calc_iq__n0_dn4,)
    }
};
        locals.var_fn277_calc_iq__n0 = assign23420_e22193;
        locals.var_fn277_calc_iq__n0_dn4 = assign23420_e22193_d_n4;
        locals.var_fn277_calc_iq__n0_rv = 0.0;

        let (assign23430_e22201, assign23430_e22201_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23430_e22197: f64 = (2.0 * locals.var_fn277_calc_iq__n0);
        let assign23430_e22199: f64 = (assign23430_e22197 * locals.var_fn277_calc_iq__phitin);
        (assign23430_e22199, (((2.0 * locals.var_fn277_calc_iq__n0_dn4) * locals.var_fn277_calc_iq__phitin) + (assign23430_e22197 * locals.var_fn277_calc_iq__phitin_dn4)),)
    } else {
        (locals.var_fn277_calc_iq__two_n_phit0, locals.var_fn277_calc_iq__two_n_phit0_dn4,)
    }
};
        locals.var_fn277_calc_iq__two_n_phit0 = assign23430_e22201;
        locals.var_fn277_calc_iq__two_n_phit0_dn4 = assign23430_e22201_d_n4;
        locals.var_fn277_calc_iq__two_n_phit0_rv = 0.0;

        let (assign23440_e22207, assign23440_e22207_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23440_e22205: f64 = (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__two_n_phit0);
        (assign23440_e22205, ((locals.var_fn277_calc_iq__cgin_dn4 * locals.var_fn277_calc_iq__two_n_phit0) + (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__two_n_phit0_dn4)),)
    } else {
        (locals.var_fn277_calc_iq__qref0, locals.var_fn277_calc_iq__qref0_dn4,)
    }
};
        locals.var_fn277_calc_iq__qref0 = assign23440_e22207;
        locals.var_fn277_calc_iq__qref0_dn4 = assign23440_e22207_d_n4;
        locals.var_fn277_calc_iq__qref0_rv = 0.0;

        let (assign23450_e22217, assign23450_e22217_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23450_e22212: f64 = (p.p51 * locals.var_fn277_calc_iq__alpha_phit);
        let assign23450_e22214: f64 = (assign23450_e22212 / 2.0);
        let assign23450_e22215: f64 = (locals.var_fn277_calc_iq__vtof - assign23450_e22214);
        (assign23450_e22215, (locals.var_fn277_calc_iq__vtof_dn4 - ((p.p51 * locals.var_fn277_calc_iq__alpha_phit_dn4) / 2.0)),)
    } else {
        (locals.var_fn277_calc_iq__myarg0, locals.var_fn277_calc_iq__myarg0_dn4,)
    }
};
        locals.var_fn277_calc_iq__myarg0 = assign23450_e22217;
        locals.var_fn277_calc_iq__myarg0_dn4 = assign23450_e22217_d_n4;
        locals.var_fn277_calc_iq__myarg0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_64(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23460_e22268, assign23460_e22268_d_n2, assign23460_e22268_d_n4, assign23460_e22268_d_n7, assign23460_e22268_d_n12, assign23460_e22268_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign23460_e22262, assign23460_e22262_d_n2, assign23460_e22262_d_n7, assign23460_e22262_d_n12, assign23460_e22262_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign23460_e22226: f64 = (locals.var_fn277_calc_iq__vgsin + locals.var_fn277_calc_iq__vgdin);
                let assign23460_e22229: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                let assign23460_e22232: f64 = (0.001 / p.p53);
                let assign23460_e22235: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                let assign23460_e22236: f64 = (assign23460_e22232 * assign23460_e22235);
                let assign23460_e22237: f64 = (assign23460_e22236).tanh();
                let assign23460_e22238: f64 = (assign23460_e22229 * assign23460_e22237);
                let assign23460_e22239: f64 = (assign23460_e22226 + assign23460_e22238);
                let assign23460_e22240: f64 = (0.5 * assign23460_e22239);
                (assign23460_e22240, (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn2 + locals.var_fn277_calc_iq__vgdin_dn2) + (((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2) * assign23460_e22237) + (assign23460_e22229 * ((assign23460_e22232 * (locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2)) / ((assign23460_e22236).cosh() * (assign23460_e22236).cosh())))))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn7 + locals.var_fn277_calc_iq__vgdin_dn7) + (((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7) * assign23460_e22237) + (assign23460_e22229 * ((assign23460_e22232 * (locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7)) / ((assign23460_e22236).cosh() * (assign23460_e22236).cosh())))))), (0.5 * (locals.var_fn277_calc_iq__vgdin_dn12 + (((-locals.var_fn277_calc_iq__vgdin_dn12) * assign23460_e22237) + (assign23460_e22229 * ((assign23460_e22232 * (-locals.var_fn277_calc_iq__vgdin_dn12)) / ((assign23460_e22236).cosh() * (assign23460_e22236).cosh())))))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn13 + locals.var_fn277_calc_iq__vgdin_dn13) + (((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13) * assign23460_e22237) + (assign23460_e22229 * ((assign23460_e22232 * (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13)) / ((assign23460_e22236).cosh() * (assign23460_e22236).cosh())))))),)
            } else {
                let (assign23460_e22261, assign23460_e22261_d_n2, assign23460_e22261_d_n7, assign23460_e22261_d_n12, assign23460_e22261_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign23460_e22247: f64 = (locals.var_fn277_calc_iq__vgsin + locals.var_fn277_calc_iq__vgdin);
                        let assign23460_e22250: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                        let assign23460_e22253: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                        let assign23460_e22254: f64 = (assign23460_e22250 * assign23460_e22253);
                        let assign23460_e22256: f64 = (assign23460_e22254 + p.p53);
                        let assign23460_e22257: f64 = (assign23460_e22256).sqrt();
                        let assign23460_e22258: f64 = (assign23460_e22247 + assign23460_e22257);
                        let assign23460_e22259: f64 = (0.5 * assign23460_e22258);
                        (assign23460_e22259, (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn2 + locals.var_fn277_calc_iq__vgdin_dn2) + ((((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2) * assign23460_e22253) + (assign23460_e22250 * (locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2))) / (2.0 * assign23460_e22257)))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn7 + locals.var_fn277_calc_iq__vgdin_dn7) + ((((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7) * assign23460_e22253) + (assign23460_e22250 * (locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7))) / (2.0 * assign23460_e22257)))), (0.5 * (locals.var_fn277_calc_iq__vgdin_dn12 + ((((-locals.var_fn277_calc_iq__vgdin_dn12) * assign23460_e22253) + (assign23460_e22250 * (-locals.var_fn277_calc_iq__vgdin_dn12))) / (2.0 * assign23460_e22257)))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn13 + locals.var_fn277_calc_iq__vgdin_dn13) + ((((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13) * assign23460_e22253) + (assign23460_e22250 * (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13))) / (2.0 * assign23460_e22257)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign23460_e22261, assign23460_e22261_d_n2, assign23460_e22261_d_n7, assign23460_e22261_d_n12, assign23460_e22261_d_n13,)
            }
        };
        let assign23460_e22264: f64 = (assign23460_e22262 - locals.var_fn277_calc_iq__myarg0);
        let assign23460_e22266: f64 = (assign23460_e22264 / locals.var_fn277_calc_iq__alpha_phit);
        (assign23460_e22266, (assign23460_e22262_d_n2 / locals.var_fn277_calc_iq__alpha_phit), ((((-locals.var_fn277_calc_iq__myarg0_dn4) * locals.var_fn277_calc_iq__alpha_phit) - (assign23460_e22264 * locals.var_fn277_calc_iq__alpha_phit_dn4)) / (locals.var_fn277_calc_iq__alpha_phit * locals.var_fn277_calc_iq__alpha_phit)), (assign23460_e22262_d_n7 / locals.var_fn277_calc_iq__alpha_phit), (assign23460_e22262_d_n12 / locals.var_fn277_calc_iq__alpha_phit), (assign23460_e22262_d_n13 / locals.var_fn277_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn277_calc_iq__exparg0, locals.var_fn277_calc_iq__exparg0_dn2, locals.var_fn277_calc_iq__exparg0_dn4, locals.var_fn277_calc_iq__exparg0_dn7, locals.var_fn277_calc_iq__exparg0_dn12, locals.var_fn277_calc_iq__exparg0_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg0 = assign23460_e22268;
        locals.var_fn277_calc_iq__exparg0_dn2 = assign23460_e22268_d_n2;
        locals.var_fn277_calc_iq__exparg0_dn4 = assign23460_e22268_d_n4;
        locals.var_fn277_calc_iq__exparg0_dn7 = assign23460_e22268_d_n7;
        locals.var_fn277_calc_iq__exparg0_dn12 = assign23460_e22268_d_n12;
        locals.var_fn277_calc_iq__exparg0_dn13 = assign23460_e22268_d_n13;
        locals.var_fn277_calc_iq__exparg0_rv = 0.0;

        let assign23470_e22271: f64 = if locals.var_fn277_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard291 = assign23470_e22271;
        locals.var_guard291_rv = 0.0;

        let (assign23480_e22277, assign23480_e22277_d_n2, assign23480_e22277_d_n4, assign23480_e22277_d_n7, assign23480_e22277_d_n12, assign23480_e22277_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard291 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ff0, locals.var_fn277_calc_iq__ff0_dn2, locals.var_fn277_calc_iq__ff0_dn4, locals.var_fn277_calc_iq__ff0_dn7, locals.var_fn277_calc_iq__ff0_dn12, locals.var_fn277_calc_iq__ff0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ff0 = assign23480_e22277;
        locals.var_fn277_calc_iq__ff0_dn2 = assign23480_e22277_d_n2;
        locals.var_fn277_calc_iq__ff0_dn4 = assign23480_e22277_d_n4;
        locals.var_fn277_calc_iq__ff0_dn7 = assign23480_e22277_d_n7;
        locals.var_fn277_calc_iq__ff0_dn12 = assign23480_e22277_d_n12;
        locals.var_fn277_calc_iq__ff0_dn13 = assign23480_e22277_d_n13;
        locals.var_fn277_calc_iq__ff0_rv = 0.0;

        let assign23490_e22280: f64 = (-50.0);
        let assign23490_e22281: f64 = if locals.var_fn277_calc_iq__exparg0 < assign23490_e22280 { 1.0 } else { 0.0 };
        locals.var_guard292 = assign23490_e22281;
        locals.var_guard292_rv = 0.0;

        let (assign23500_e22290, assign23500_e22290_d_n2, assign23500_e22290_d_n4, assign23500_e22290_d_n7, assign23500_e22290_d_n12, assign23500_e22290_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard291 == 0.0)) && (locals.var_guard292 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ff0, locals.var_fn277_calc_iq__ff0_dn2, locals.var_fn277_calc_iq__ff0_dn4, locals.var_fn277_calc_iq__ff0_dn7, locals.var_fn277_calc_iq__ff0_dn12, locals.var_fn277_calc_iq__ff0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ff0 = assign23500_e22290;
        locals.var_fn277_calc_iq__ff0_dn2 = assign23500_e22290_d_n2;
        locals.var_fn277_calc_iq__ff0_dn4 = assign23500_e22290_d_n4;
        locals.var_fn277_calc_iq__ff0_dn7 = assign23500_e22290_d_n7;
        locals.var_fn277_calc_iq__ff0_dn12 = assign23500_e22290_d_n12;
        locals.var_fn277_calc_iq__ff0_dn13 = assign23500_e22290_d_n13;
        locals.var_fn277_calc_iq__ff0_rv = 0.0;

        let (assign23510_e22305, assign23510_e22305_d_n2, assign23510_e22305_d_n4, assign23510_e22305_d_n7, assign23510_e22305_d_n12, assign23510_e22305_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard291 == 0.0)) && (locals.var_guard292 == 0.0)) {
        let assign23510_e22301: f64 = (locals.var_fn277_calc_iq__exparg0).exp();
        let assign23510_e22302: f64 = (1.0 + assign23510_e22301);
        let assign23510_e22303: f64 = (1.0 / assign23510_e22302);
        (assign23510_e22303, (-((assign23510_e22301 * locals.var_fn277_calc_iq__exparg0_dn2) / (assign23510_e22302 * assign23510_e22302))), (-((assign23510_e22301 * locals.var_fn277_calc_iq__exparg0_dn4) / (assign23510_e22302 * assign23510_e22302))), (-((assign23510_e22301 * locals.var_fn277_calc_iq__exparg0_dn7) / (assign23510_e22302 * assign23510_e22302))), (-((assign23510_e22301 * locals.var_fn277_calc_iq__exparg0_dn12) / (assign23510_e22302 * assign23510_e22302))), (-((assign23510_e22301 * locals.var_fn277_calc_iq__exparg0_dn13) / (assign23510_e22302 * assign23510_e22302))),)
    } else {
        (locals.var_fn277_calc_iq__ff0, locals.var_fn277_calc_iq__ff0_dn2, locals.var_fn277_calc_iq__ff0_dn4, locals.var_fn277_calc_iq__ff0_dn7, locals.var_fn277_calc_iq__ff0_dn12, locals.var_fn277_calc_iq__ff0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ff0 = assign23510_e22305;
        locals.var_fn277_calc_iq__ff0_dn2 = assign23510_e22305_d_n2;
        locals.var_fn277_calc_iq__ff0_dn4 = assign23510_e22305_d_n4;
        locals.var_fn277_calc_iq__ff0_dn7 = assign23510_e22305_d_n7;
        locals.var_fn277_calc_iq__ff0_dn12 = assign23510_e22305_d_n12;
        locals.var_fn277_calc_iq__ff0_dn13 = assign23510_e22305_d_n13;
        locals.var_fn277_calc_iq__ff0_rv = 0.0;

        let (assign23520_e22364, assign23520_e22364_d_n2, assign23520_e22364_d_n4, assign23520_e22364_d_n7, assign23520_e22364_d_n12, assign23520_e22364_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign23520_e22350, assign23520_e22350_d_n2, assign23520_e22350_d_n7, assign23520_e22350_d_n12, assign23520_e22350_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign23520_e22314: f64 = (locals.var_fn277_calc_iq__vgsin + locals.var_fn277_calc_iq__vgdin);
                let assign23520_e22317: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                let assign23520_e22320: f64 = (0.001 / p.p53);
                let assign23520_e22323: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                let assign23520_e22324: f64 = (assign23520_e22320 * assign23520_e22323);
                let assign23520_e22325: f64 = (assign23520_e22324).tanh();
                let assign23520_e22326: f64 = (assign23520_e22317 * assign23520_e22325);
                let assign23520_e22327: f64 = (assign23520_e22314 + assign23520_e22326);
                let assign23520_e22328: f64 = (0.5 * assign23520_e22327);
                (assign23520_e22328, (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn2 + locals.var_fn277_calc_iq__vgdin_dn2) + (((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2) * assign23520_e22325) + (assign23520_e22317 * ((assign23520_e22320 * (locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2)) / ((assign23520_e22324).cosh() * (assign23520_e22324).cosh())))))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn7 + locals.var_fn277_calc_iq__vgdin_dn7) + (((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7) * assign23520_e22325) + (assign23520_e22317 * ((assign23520_e22320 * (locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7)) / ((assign23520_e22324).cosh() * (assign23520_e22324).cosh())))))), (0.5 * (locals.var_fn277_calc_iq__vgdin_dn12 + (((-locals.var_fn277_calc_iq__vgdin_dn12) * assign23520_e22325) + (assign23520_e22317 * ((assign23520_e22320 * (-locals.var_fn277_calc_iq__vgdin_dn12)) / ((assign23520_e22324).cosh() * (assign23520_e22324).cosh())))))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn13 + locals.var_fn277_calc_iq__vgdin_dn13) + (((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13) * assign23520_e22325) + (assign23520_e22317 * ((assign23520_e22320 * (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13)) / ((assign23520_e22324).cosh() * (assign23520_e22324).cosh())))))),)
            } else {
                let (assign23520_e22349, assign23520_e22349_d_n2, assign23520_e22349_d_n7, assign23520_e22349_d_n12, assign23520_e22349_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign23520_e22335: f64 = (locals.var_fn277_calc_iq__vgsin + locals.var_fn277_calc_iq__vgdin);
                        let assign23520_e22338: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                        let assign23520_e22341: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vgdin);
                        let assign23520_e22342: f64 = (assign23520_e22338 * assign23520_e22341);
                        let assign23520_e22344: f64 = (assign23520_e22342 + p.p53);
                        let assign23520_e22345: f64 = (assign23520_e22344).sqrt();
                        let assign23520_e22346: f64 = (assign23520_e22335 + assign23520_e22345);
                        let assign23520_e22347: f64 = (0.5 * assign23520_e22346);
                        (assign23520_e22347, (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn2 + locals.var_fn277_calc_iq__vgdin_dn2) + ((((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2) * assign23520_e22341) + (assign23520_e22338 * (locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vgdin_dn2))) / (2.0 * assign23520_e22345)))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn7 + locals.var_fn277_calc_iq__vgdin_dn7) + ((((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7) * assign23520_e22341) + (assign23520_e22338 * (locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vgdin_dn7))) / (2.0 * assign23520_e22345)))), (0.5 * (locals.var_fn277_calc_iq__vgdin_dn12 + ((((-locals.var_fn277_calc_iq__vgdin_dn12) * assign23520_e22341) + (assign23520_e22338 * (-locals.var_fn277_calc_iq__vgdin_dn12))) / (2.0 * assign23520_e22345)))), (0.5 * ((locals.var_fn277_calc_iq__vgsin_dn13 + locals.var_fn277_calc_iq__vgdin_dn13) + ((((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13) * assign23520_e22341) + (assign23520_e22338 * (locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vgdin_dn13))) / (2.0 * assign23520_e22345)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign23520_e22349, assign23520_e22349_d_n2, assign23520_e22349_d_n7, assign23520_e22349_d_n12, assign23520_e22349_d_n13,)
            }
        };
        let assign23520_e22354: f64 = (p.p51 * 0.1);
        let assign23520_e22356: f64 = (assign23520_e22354 * locals.var_fn277_calc_iq__alpha_phit);
        let assign23520_e22358: f64 = (assign23520_e22356 * locals.var_fn277_calc_iq__ff0);
        let assign23520_e22359: f64 = (locals.var_fn277_calc_iq__vtof - assign23520_e22358);
        let assign23520_e22360: f64 = (assign23520_e22350 - assign23520_e22359);
        let assign23520_e22362: f64 = (assign23520_e22360 / locals.var_fn277_calc_iq__two_n_phit0);
        (assign23520_e22362, ((assign23520_e22350_d_n2 - (-(assign23520_e22356 * locals.var_fn277_calc_iq__ff0_dn2))) / locals.var_fn277_calc_iq__two_n_phit0), ((((-(locals.var_fn277_calc_iq__vtof_dn4 - (((assign23520_e22354 * locals.var_fn277_calc_iq__alpha_phit_dn4) * locals.var_fn277_calc_iq__ff0) + (assign23520_e22356 * locals.var_fn277_calc_iq__ff0_dn4)))) * locals.var_fn277_calc_iq__two_n_phit0) - (assign23520_e22360 * locals.var_fn277_calc_iq__two_n_phit0_dn4)) / (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__two_n_phit0)), ((assign23520_e22350_d_n7 - (-(assign23520_e22356 * locals.var_fn277_calc_iq__ff0_dn7))) / locals.var_fn277_calc_iq__two_n_phit0), ((assign23520_e22350_d_n12 - (-(assign23520_e22356 * locals.var_fn277_calc_iq__ff0_dn12))) / locals.var_fn277_calc_iq__two_n_phit0), ((assign23520_e22350_d_n13 - (-(assign23520_e22356 * locals.var_fn277_calc_iq__ff0_dn13))) / locals.var_fn277_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn277_calc_iq__eta0, locals.var_fn277_calc_iq__eta0_dn2, locals.var_fn277_calc_iq__eta0_dn4, locals.var_fn277_calc_iq__eta0_dn7, locals.var_fn277_calc_iq__eta0_dn12, locals.var_fn277_calc_iq__eta0_dn13,)
    }
};
        locals.var_fn277_calc_iq__eta0 = assign23520_e22364;
        locals.var_fn277_calc_iq__eta0_dn2 = assign23520_e22364_d_n2;
        locals.var_fn277_calc_iq__eta0_dn4 = assign23520_e22364_d_n4;
        locals.var_fn277_calc_iq__eta0_dn7 = assign23520_e22364_d_n7;
        locals.var_fn277_calc_iq__eta0_dn12 = assign23520_e22364_d_n12;
        locals.var_fn277_calc_iq__eta0_dn13 = assign23520_e22364_d_n13;
        locals.var_fn277_calc_iq__eta0_rv = 0.0;

        let assign23530_e22367: f64 = if locals.var_fn277_calc_iq__eta0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard293 = assign23530_e22367;
        locals.var_guard293_rv = 0.0;

        let (assign23540_e22375, assign23540_e22375_d_n2, assign23540_e22375_d_n4, assign23540_e22375_d_n7, assign23540_e22375_d_n12, assign23540_e22375_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard293 != 0.0)) {
        let assign23540_e22373: f64 = (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__eta0);
        (assign23540_e22373, (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__eta0_dn2), ((locals.var_fn277_calc_iq__qref0_dn4 * locals.var_fn277_calc_iq__eta0) + (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__eta0_dn4)), (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__eta0_dn7), (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__eta0_dn12), (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__eta0_dn13),)
    } else {
        (locals.var_fn277_calc_iq__qinvv0, locals.var_fn277_calc_iq__qinvv0_dn2, locals.var_fn277_calc_iq__qinvv0_dn4, locals.var_fn277_calc_iq__qinvv0_dn7, locals.var_fn277_calc_iq__qinvv0_dn12, locals.var_fn277_calc_iq__qinvv0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvv0 = assign23540_e22375;
        locals.var_fn277_calc_iq__qinvv0_dn2 = assign23540_e22375_d_n2;
        locals.var_fn277_calc_iq__qinvv0_dn4 = assign23540_e22375_d_n4;
        locals.var_fn277_calc_iq__qinvv0_dn7 = assign23540_e22375_d_n7;
        locals.var_fn277_calc_iq__qinvv0_dn12 = assign23540_e22375_d_n12;
        locals.var_fn277_calc_iq__qinvv0_dn13 = assign23540_e22375_d_n13;
        locals.var_fn277_calc_iq__qinvv0_rv = 0.0;

        let assign23550_e22378: f64 = (-50.0);
        let assign23550_e22379: f64 = if locals.var_fn277_calc_iq__eta0 < assign23550_e22378 { 1.0 } else { 0.0 };
        locals.var_guard294 = assign23550_e22379;
        locals.var_guard294_rv = 0.0;

        let (assign23560_e22391, assign23560_e22391_d_n2, assign23560_e22391_d_n4, assign23560_e22391_d_n7, assign23560_e22391_d_n12, assign23560_e22391_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard293 == 0.0)) && (locals.var_guard294 != 0.0)) {
        let assign23560_e22388: f64 = (locals.var_fn277_calc_iq__eta0).exp();
        let assign23560_e22389: f64 = (locals.var_fn277_calc_iq__qref0 * assign23560_e22388);
        (assign23560_e22389, (locals.var_fn277_calc_iq__qref0 * (assign23560_e22388 * locals.var_fn277_calc_iq__eta0_dn2)), ((locals.var_fn277_calc_iq__qref0_dn4 * assign23560_e22388) + (locals.var_fn277_calc_iq__qref0 * (assign23560_e22388 * locals.var_fn277_calc_iq__eta0_dn4))), (locals.var_fn277_calc_iq__qref0 * (assign23560_e22388 * locals.var_fn277_calc_iq__eta0_dn7)), (locals.var_fn277_calc_iq__qref0 * (assign23560_e22388 * locals.var_fn277_calc_iq__eta0_dn12)), (locals.var_fn277_calc_iq__qref0 * (assign23560_e22388 * locals.var_fn277_calc_iq__eta0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qinvv0, locals.var_fn277_calc_iq__qinvv0_dn2, locals.var_fn277_calc_iq__qinvv0_dn4, locals.var_fn277_calc_iq__qinvv0_dn7, locals.var_fn277_calc_iq__qinvv0_dn12, locals.var_fn277_calc_iq__qinvv0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvv0 = assign23560_e22391;
        locals.var_fn277_calc_iq__qinvv0_dn2 = assign23560_e22391_d_n2;
        locals.var_fn277_calc_iq__qinvv0_dn4 = assign23560_e22391_d_n4;
        locals.var_fn277_calc_iq__qinvv0_dn7 = assign23560_e22391_d_n7;
        locals.var_fn277_calc_iq__qinvv0_dn12 = assign23560_e22391_d_n12;
        locals.var_fn277_calc_iq__qinvv0_dn13 = assign23560_e22391_d_n13;
        locals.var_fn277_calc_iq__qinvv0_rv = 0.0;

        let (assign23570_e22407, assign23570_e22407_d_n2, assign23570_e22407_d_n4, assign23570_e22407_d_n7, assign23570_e22407_d_n12, assign23570_e22407_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard293 == 0.0)) && (locals.var_guard294 == 0.0)) {
        let assign23570_e22402: f64 = (locals.var_fn277_calc_iq__eta0).exp();
        let assign23570_e22403: f64 = (1.0 + assign23570_e22402);
        let assign23570_e22404: f64 = (assign23570_e22403).ln();
        let assign23570_e22405: f64 = (locals.var_fn277_calc_iq__qref0 * assign23570_e22404);
        (assign23570_e22405, (locals.var_fn277_calc_iq__qref0 * ((assign23570_e22402 * locals.var_fn277_calc_iq__eta0_dn2) / assign23570_e22403)), ((locals.var_fn277_calc_iq__qref0_dn4 * assign23570_e22404) + (locals.var_fn277_calc_iq__qref0 * ((assign23570_e22402 * locals.var_fn277_calc_iq__eta0_dn4) / assign23570_e22403))), (locals.var_fn277_calc_iq__qref0 * ((assign23570_e22402 * locals.var_fn277_calc_iq__eta0_dn7) / assign23570_e22403)), (locals.var_fn277_calc_iq__qref0 * ((assign23570_e22402 * locals.var_fn277_calc_iq__eta0_dn12) / assign23570_e22403)), (locals.var_fn277_calc_iq__qref0 * ((assign23570_e22402 * locals.var_fn277_calc_iq__eta0_dn13) / assign23570_e22403)),)
    } else {
        (locals.var_fn277_calc_iq__qinvv0, locals.var_fn277_calc_iq__qinvv0_dn2, locals.var_fn277_calc_iq__qinvv0_dn4, locals.var_fn277_calc_iq__qinvv0_dn7, locals.var_fn277_calc_iq__qinvv0_dn12, locals.var_fn277_calc_iq__qinvv0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvv0 = assign23570_e22407;
        locals.var_fn277_calc_iq__qinvv0_dn2 = assign23570_e22407_d_n2;
        locals.var_fn277_calc_iq__qinvv0_dn4 = assign23570_e22407_d_n4;
        locals.var_fn277_calc_iq__qinvv0_dn7 = assign23570_e22407_d_n7;
        locals.var_fn277_calc_iq__qinvv0_dn12 = assign23570_e22407_d_n12;
        locals.var_fn277_calc_iq__qinvv0_dn13 = assign23570_e22407_d_n13;
        locals.var_fn277_calc_iq__qinvv0_rv = 0.0;

        let (assign23580_e22413, assign23580_e22413_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23580_e22411: f64 = (locals.var_fn277_calc_iq__mu0 / locals.var_fn277_calc_iq__tfacmobin);
        (assign23580_e22411, (-((locals.var_fn277_calc_iq__mu0 * locals.var_fn277_calc_iq__tfacmobin_dn4) / (locals.var_fn277_calc_iq__tfacmobin * locals.var_fn277_calc_iq__tfacmobin))),)
    } else {
        (locals.var_fn277_calc_iq__muf0, locals.var_fn277_calc_iq__muf0_dn4,)
    }
};
        locals.var_fn277_calc_iq__muf0 = assign23580_e22413;
        locals.var_fn277_calc_iq__muf0_dn4 = assign23580_e22413_d_n4;
        locals.var_fn277_calc_iq__muf0_rv = 0.0;

        let (assign23590_e22429, assign23590_e22429_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23590_e22419: f64 = (locals.var_fn277_calc_iq__vzeta * locals.var_fn277_calc_iq__tnomin);
        let assign23590_e22420: f64 = (1.0 + assign23590_e22419);
        let assign23590_e22424: f64 = (locals.var_fn277_calc_iq__vzeta * locals.var_fn277_calc_iq__tambin);
        let assign23590_e22425: f64 = (1.0 + assign23590_e22424);
        let assign23590_e22426: f64 = (assign23590_e22420 / assign23590_e22425);
        let assign23590_e22427: f64 = (locals.var_fn277_calc_iq__vel0 * assign23590_e22426);
        (assign23590_e22427, (locals.var_fn277_calc_iq__vel0 * (-((assign23590_e22420 * (locals.var_fn277_calc_iq__vzeta * locals.var_fn277_calc_iq__tambin_dn4)) / (assign23590_e22425 * assign23590_e22425)))),)
    } else {
        (locals.var_fn277_calc_iq__vx0, locals.var_fn277_calc_iq__vx0_dn4,)
    }
};
        locals.var_fn277_calc_iq__vx0 = assign23590_e22429;
        locals.var_fn277_calc_iq__vx0_dn4 = assign23590_e22429_d_n4;
        locals.var_fn277_calc_iq__vx0_rv = 0.0;

        let (assign23600_e22437, assign23600_e22437_d_n4,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23600_e22433: f64 = (locals.var_fn277_calc_iq__vx0 * locals.var_fn277_calc_iq__lin);
        let assign23600_e22435: f64 = (assign23600_e22433 / locals.var_fn277_calc_iq__muf0);
        (assign23600_e22435, ((((locals.var_fn277_calc_iq__vx0_dn4 * locals.var_fn277_calc_iq__lin) * locals.var_fn277_calc_iq__muf0) - (assign23600_e22433 * locals.var_fn277_calc_iq__muf0_dn4)) / (locals.var_fn277_calc_iq__muf0 * locals.var_fn277_calc_iq__muf0)),)
    } else {
        (locals.var_fn277_calc_iq__vdsats0, locals.var_fn277_calc_iq__vdsats0_dn4,)
    }
};
        locals.var_fn277_calc_iq__vdsats0 = assign23600_e22437;
        locals.var_fn277_calc_iq__vdsats0_dn4 = assign23600_e22437_d_n4;
        locals.var_fn277_calc_iq__vdsats0_rv = 0.0;

        let (assign23610_e22454, assign23610_e22454_d_n2, assign23610_e22454_d_n4, assign23610_e22454_d_n7, assign23610_e22454_d_n12, assign23610_e22454_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23610_e22443: f64 = (2.0 * locals.var_fn277_calc_iq__qinvv0);
        let assign23610_e22445: f64 = (assign23610_e22443 / locals.var_fn277_calc_iq__cgin);
        let assign23610_e22447: f64 = (assign23610_e22445 / locals.var_fn277_calc_iq__vdsats0);
        let assign23610_e22448: f64 = (1.0 + assign23610_e22447);
        let assign23610_e22449: f64 = (assign23610_e22448).sqrt();
        let assign23610_e22450: f64 = (locals.var_fn277_calc_iq__vdsats0 * assign23610_e22449);
        let assign23610_e22452: f64 = (assign23610_e22450 - locals.var_fn277_calc_iq__vdsats0);
        (assign23610_e22452, (locals.var_fn277_calc_iq__vdsats0 * ((((2.0 * locals.var_fn277_calc_iq__qinvv0_dn2) / locals.var_fn277_calc_iq__cgin) / locals.var_fn277_calc_iq__vdsats0) / (2.0 * assign23610_e22449))), (((locals.var_fn277_calc_iq__vdsats0_dn4 * assign23610_e22449) + (locals.var_fn277_calc_iq__vdsats0 * ((((((((2.0 * locals.var_fn277_calc_iq__qinvv0_dn4) * locals.var_fn277_calc_iq__cgin) - (assign23610_e22443 * locals.var_fn277_calc_iq__cgin_dn4)) / (locals.var_fn277_calc_iq__cgin * locals.var_fn277_calc_iq__cgin)) * locals.var_fn277_calc_iq__vdsats0) - (assign23610_e22445 * locals.var_fn277_calc_iq__vdsats0_dn4)) / (locals.var_fn277_calc_iq__vdsats0 * locals.var_fn277_calc_iq__vdsats0)) / (2.0 * assign23610_e22449)))) - locals.var_fn277_calc_iq__vdsats0_dn4), (locals.var_fn277_calc_iq__vdsats0 * ((((2.0 * locals.var_fn277_calc_iq__qinvv0_dn7) / locals.var_fn277_calc_iq__cgin) / locals.var_fn277_calc_iq__vdsats0) / (2.0 * assign23610_e22449))), (locals.var_fn277_calc_iq__vdsats0 * ((((2.0 * locals.var_fn277_calc_iq__qinvv0_dn12) / locals.var_fn277_calc_iq__cgin) / locals.var_fn277_calc_iq__vdsats0) / (2.0 * assign23610_e22449))), (locals.var_fn277_calc_iq__vdsats0 * ((((2.0 * locals.var_fn277_calc_iq__qinvv0_dn13) / locals.var_fn277_calc_iq__cgin) / locals.var_fn277_calc_iq__vdsats0) / (2.0 * assign23610_e22449))),)
    } else {
        (locals.var_fn277_calc_iq__vdsats10, locals.var_fn277_calc_iq__vdsats10_dn2, locals.var_fn277_calc_iq__vdsats10_dn4, locals.var_fn277_calc_iq__vdsats10_dn7, locals.var_fn277_calc_iq__vdsats10_dn12, locals.var_fn277_calc_iq__vdsats10_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsats10 = assign23610_e22454;
        locals.var_fn277_calc_iq__vdsats10_dn2 = assign23610_e22454_d_n2;
        locals.var_fn277_calc_iq__vdsats10_dn4 = assign23610_e22454_d_n4;
        locals.var_fn277_calc_iq__vdsats10_dn7 = assign23610_e22454_d_n7;
        locals.var_fn277_calc_iq__vdsats10_dn12 = assign23610_e22454_d_n12;
        locals.var_fn277_calc_iq__vdsats10_dn13 = assign23610_e22454_d_n13;
        locals.var_fn277_calc_iq__vdsats10_rv = 0.0;

        let (assign23620_e22466, assign23620_e22466_d_n2, assign23620_e22466_d_n4, assign23620_e22466_d_n7, assign23620_e22466_d_n12, assign23620_e22466_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23620_e22459: f64 = (1.0 - locals.var_fn277_calc_iq__ff0);
        let assign23620_e22460: f64 = (locals.var_fn277_calc_iq__vdsats10 * assign23620_e22459);
        let assign23620_e22463: f64 = (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__ff0);
        let assign23620_e22464: f64 = (assign23620_e22460 + assign23620_e22463);
        (assign23620_e22464, (((locals.var_fn277_calc_iq__vdsats10_dn2 * assign23620_e22459) + (locals.var_fn277_calc_iq__vdsats10 * (-locals.var_fn277_calc_iq__ff0_dn2))) + (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__ff0_dn2)), (((locals.var_fn277_calc_iq__vdsats10_dn4 * assign23620_e22459) + (locals.var_fn277_calc_iq__vdsats10 * (-locals.var_fn277_calc_iq__ff0_dn4))) + ((locals.var_fn277_calc_iq__two_n_phit0_dn4 * locals.var_fn277_calc_iq__ff0) + (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__ff0_dn4))), (((locals.var_fn277_calc_iq__vdsats10_dn7 * assign23620_e22459) + (locals.var_fn277_calc_iq__vdsats10 * (-locals.var_fn277_calc_iq__ff0_dn7))) + (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__ff0_dn7)), (((locals.var_fn277_calc_iq__vdsats10_dn12 * assign23620_e22459) + (locals.var_fn277_calc_iq__vdsats10 * (-locals.var_fn277_calc_iq__ff0_dn12))) + (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__ff0_dn12)), (((locals.var_fn277_calc_iq__vdsats10_dn13 * assign23620_e22459) + (locals.var_fn277_calc_iq__vdsats10 * (-locals.var_fn277_calc_iq__ff0_dn13))) + (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__ff0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__vdsat10, locals.var_fn277_calc_iq__vdsat10_dn2, locals.var_fn277_calc_iq__vdsat10_dn4, locals.var_fn277_calc_iq__vdsat10_dn7, locals.var_fn277_calc_iq__vdsat10_dn12, locals.var_fn277_calc_iq__vdsat10_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdsat10 = assign23620_e22466;
        locals.var_fn277_calc_iq__vdsat10_dn2 = assign23620_e22466_d_n2;
        locals.var_fn277_calc_iq__vdsat10_dn4 = assign23620_e22466_d_n4;
        locals.var_fn277_calc_iq__vdsat10_dn7 = assign23620_e22466_d_n7;
        locals.var_fn277_calc_iq__vdsat10_dn12 = assign23620_e22466_d_n12;
        locals.var_fn277_calc_iq__vdsat10_dn13 = assign23620_e22466_d_n13;
        locals.var_fn277_calc_iq__vdsat10_rv = 0.0;

        let (assign23630_e22535, assign23630_e22535_d_n2, assign23630_e22535_d_n4, assign23630_e22535_d_n7, assign23630_e22535_d_n12, assign23630_e22535_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign23630_e22525, assign23630_e22525_d_n2, assign23630_e22525_d_n4, assign23630_e22525_d_n7, assign23630_e22525_d_n12, assign23630_e22525_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign23630_e22478: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat10);
                let assign23630_e22479: f64 = assign23630_e22478;
                let assign23630_e22483: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat10);
                let assign23630_e22484: f64 = (-assign23630_e22483);
                let assign23630_e22487: f64 = (0.001 / p.p53);
                let assign23630_e22491: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat10);
                let assign23630_e22492: f64 = (-assign23630_e22491);
                let assign23630_e22493: f64 = (assign23630_e22487 * assign23630_e22492);
                let assign23630_e22494: f64 = (assign23630_e22493).tanh();
                let assign23630_e22495: f64 = (assign23630_e22484 * assign23630_e22494);
                let assign23630_e22496: f64 = (assign23630_e22479 + assign23630_e22495);
                let assign23630_e22497: f64 = (0.5 * assign23630_e22496);
                (assign23630_e22497, (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + (((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23630_e22494) + (assign23630_e22484 * ((assign23630_e22487 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))))) / ((assign23630_e22493).cosh() * (assign23630_e22493).cosh())))))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + (((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23630_e22494) + (assign23630_e22484 * ((assign23630_e22487 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))))) / ((assign23630_e22493).cosh() * (assign23630_e22493).cosh())))))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + (((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23630_e22494) + (assign23630_e22484 * ((assign23630_e22487 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))))) / ((assign23630_e22493).cosh() * (assign23630_e22493).cosh())))))), (0.5 * ((((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)) + (((-(((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) * assign23630_e22494) + (assign23630_e22484 * ((assign23630_e22487 * (-(((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) / ((assign23630_e22493).cosh() * (assign23630_e22493).cosh())))))), (0.5 * ((((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)) + (((-(((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) * assign23630_e22494) + (assign23630_e22484 * ((assign23630_e22487 * (-(((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) / ((assign23630_e22493).cosh() * (assign23630_e22493).cosh())))))),)
            } else {
                let (assign23630_e22524, assign23630_e22524_d_n2, assign23630_e22524_d_n4, assign23630_e22524_d_n7, assign23630_e22524_d_n12, assign23630_e22524_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign23630_e22505: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat10);
                        let assign23630_e22506: f64 = assign23630_e22505;
                        let assign23630_e22510: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat10);
                        let assign23630_e22511: f64 = (-assign23630_e22510);
                        let assign23630_e22515: f64 = (locals.var_fn277_calc_iq__vdsin / locals.var_fn277_calc_iq__vdsat10);
                        let assign23630_e22516: f64 = (-assign23630_e22515);
                        let assign23630_e22517: f64 = (assign23630_e22511 * assign23630_e22516);
                        let assign23630_e22519: f64 = (assign23630_e22517 + p.p53);
                        let assign23630_e22520: f64 = (assign23630_e22519).sqrt();
                        let assign23630_e22521: f64 = (assign23630_e22506 + assign23630_e22520);
                        let assign23630_e22522: f64 = (0.5 * assign23630_e22521);
                        (assign23630_e22522, (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + ((((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23630_e22516) + (assign23630_e22511 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))))) / (2.0 * assign23630_e22520)))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + ((((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23630_e22516) + (assign23630_e22511 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))))) / (2.0 * assign23630_e22520)))), (0.5 * ((-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + ((((-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23630_e22516) + (assign23630_e22511 * (-(-((locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))))) / (2.0 * assign23630_e22520)))), (0.5 * ((((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)) + ((((-(((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) * assign23630_e22516) + (assign23630_e22511 * (-(((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))))) / (2.0 * assign23630_e22520)))), (0.5 * ((((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)) + ((((-(((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) * assign23630_e22516) + (assign23630_e22511 * (-(((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__vdsat10) - (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))))) / (2.0 * assign23630_e22520)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign23630_e22524, assign23630_e22524_d_n2, assign23630_e22524_d_n4, assign23630_e22524_d_n7, assign23630_e22524_d_n12, assign23630_e22524_d_n13,)
            }
        };
        let assign23630_e22527: f64 = (assign23630_e22525).powf(locals.var_fn277_calc_iq__beta);
        let assign23630_e22528: f64 = (1.0 + assign23630_e22527);
        let assign23630_e22531: f64 = (1.0 / locals.var_fn277_calc_iq__beta);
        let assign23630_e22532: f64 = (assign23630_e22528).powf(assign23630_e22531);
        let assign23630_e22533: f64 = (1.0 / assign23630_e22532);
        (assign23630_e22533, (-(if 0.0 == 0.0 && ((assign23630_e22531) as f64).is_finite() && ((assign23630_e22531) as f64).fract() == 0.0 { if assign23630_e22531 == 0.0 { 0.0 } else { (assign23630_e22531 * ((assign23630_e22528).powf(assign23630_e22531 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23630_e22525).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23630_e22525_d_n2)) } } else { (assign23630_e22527 * (locals.var_fn277_calc_iq__beta * (assign23630_e22525_d_n2 / assign23630_e22525))) })) } } else { (assign23630_e22532 * (assign23630_e22531 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23630_e22525).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23630_e22525_d_n2)) } } else { (assign23630_e22527 * (locals.var_fn277_calc_iq__beta * (assign23630_e22525_d_n2 / assign23630_e22525))) } / assign23630_e22528))) } / (assign23630_e22532 * assign23630_e22532))), (-(if 0.0 == 0.0 && ((assign23630_e22531) as f64).is_finite() && ((assign23630_e22531) as f64).fract() == 0.0 { if assign23630_e22531 == 0.0 { 0.0 } else { (assign23630_e22531 * ((assign23630_e22528).powf(assign23630_e22531 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23630_e22525).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23630_e22525_d_n4)) } } else { (assign23630_e22527 * (locals.var_fn277_calc_iq__beta * (assign23630_e22525_d_n4 / assign23630_e22525))) })) } } else { (assign23630_e22532 * (assign23630_e22531 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23630_e22525).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23630_e22525_d_n4)) } } else { (assign23630_e22527 * (locals.var_fn277_calc_iq__beta * (assign23630_e22525_d_n4 / assign23630_e22525))) } / assign23630_e22528))) } / (assign23630_e22532 * assign23630_e22532))), (-(if 0.0 == 0.0 && ((assign23630_e22531) as f64).is_finite() && ((assign23630_e22531) as f64).fract() == 0.0 { if assign23630_e22531 == 0.0 { 0.0 } else { (assign23630_e22531 * ((assign23630_e22528).powf(assign23630_e22531 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23630_e22525).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23630_e22525_d_n7)) } } else { (assign23630_e22527 * (locals.var_fn277_calc_iq__beta * (assign23630_e22525_d_n7 / assign23630_e22525))) })) } } else { (assign23630_e22532 * (assign23630_e22531 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23630_e22525).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23630_e22525_d_n7)) } } else { (assign23630_e22527 * (locals.var_fn277_calc_iq__beta * (assign23630_e22525_d_n7 / assign23630_e22525))) } / assign23630_e22528))) } / (assign23630_e22532 * assign23630_e22532))), (-(if 0.0 == 0.0 && ((assign23630_e22531) as f64).is_finite() && ((assign23630_e22531) as f64).fract() == 0.0 { if assign23630_e22531 == 0.0 { 0.0 } else { (assign23630_e22531 * ((assign23630_e22528).powf(assign23630_e22531 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23630_e22525).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23630_e22525_d_n12)) } } else { (assign23630_e22527 * (locals.var_fn277_calc_iq__beta * (assign23630_e22525_d_n12 / assign23630_e22525))) })) } } else { (assign23630_e22532 * (assign23630_e22531 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23630_e22525).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23630_e22525_d_n12)) } } else { (assign23630_e22527 * (locals.var_fn277_calc_iq__beta * (assign23630_e22525_d_n12 / assign23630_e22525))) } / assign23630_e22528))) } / (assign23630_e22532 * assign23630_e22532))), (-(if 0.0 == 0.0 && ((assign23630_e22531) as f64).is_finite() && ((assign23630_e22531) as f64).fract() == 0.0 { if assign23630_e22531 == 0.0 { 0.0 } else { (assign23630_e22531 * ((assign23630_e22528).powf(assign23630_e22531 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23630_e22525).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23630_e22525_d_n13)) } } else { (assign23630_e22527 * (locals.var_fn277_calc_iq__beta * (assign23630_e22525_d_n13 / assign23630_e22525))) })) } } else { (assign23630_e22532 * (assign23630_e22531 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23630_e22525).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23630_e22525_d_n13)) } } else { (assign23630_e22527 * (locals.var_fn277_calc_iq__beta * (assign23630_e22525_d_n13 / assign23630_e22525))) } / assign23630_e22528))) } / (assign23630_e22532 * assign23630_e22532))),)
    } else {
        (locals.var_fn277_calc_iq__fsd0, locals.var_fn277_calc_iq__fsd0_dn2, locals.var_fn277_calc_iq__fsd0_dn4, locals.var_fn277_calc_iq__fsd0_dn7, locals.var_fn277_calc_iq__fsd0_dn12, locals.var_fn277_calc_iq__fsd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__fsd0 = assign23630_e22535;
        locals.var_fn277_calc_iq__fsd0_dn2 = assign23630_e22535_d_n2;
        locals.var_fn277_calc_iq__fsd0_dn4 = assign23630_e22535_d_n4;
        locals.var_fn277_calc_iq__fsd0_dn7 = assign23630_e22535_d_n7;
        locals.var_fn277_calc_iq__fsd0_dn12 = assign23630_e22535_d_n12;
        locals.var_fn277_calc_iq__fsd0_dn13 = assign23630_e22535_d_n13;
        locals.var_fn277_calc_iq__fsd0_rv = 0.0;

        let (assign23640_e22541, assign23640_e22541_d_n2, assign23640_e22541_d_n4, assign23640_e22541_d_n7, assign23640_e22541_d_n12, assign23640_e22541_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23640_e22539: f64 = (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd0);
        (assign23640_e22539, (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd0_dn2), (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd0_dn4), (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd0_dn7), ((locals.var_fn277_calc_iq__vdsin_dn12 * locals.var_fn277_calc_iq__fsd0) + (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd0_dn12)), ((locals.var_fn277_calc_iq__vdsin_dn13 * locals.var_fn277_calc_iq__fsd0) + (locals.var_fn277_calc_iq__vdsin * locals.var_fn277_calc_iq__fsd0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__vdx0, locals.var_fn277_calc_iq__vdx0_dn2, locals.var_fn277_calc_iq__vdx0_dn4, locals.var_fn277_calc_iq__vdx0_dn7, locals.var_fn277_calc_iq__vdx0_dn12, locals.var_fn277_calc_iq__vdx0_dn13,)
    }
};
        locals.var_fn277_calc_iq__vdx0 = assign23640_e22541;
        locals.var_fn277_calc_iq__vdx0_dn2 = assign23640_e22541_d_n2;
        locals.var_fn277_calc_iq__vdx0_dn4 = assign23640_e22541_d_n4;
        locals.var_fn277_calc_iq__vdx0_dn7 = assign23640_e22541_d_n7;
        locals.var_fn277_calc_iq__vdx0_dn12 = assign23640_e22541_d_n12;
        locals.var_fn277_calc_iq__vdx0_dn13 = assign23640_e22541_d_n13;
        locals.var_fn277_calc_iq__vdx0_rv = 0.0;

        let (assign23650_e22616, assign23650_e22616_d_n2, assign23650_e22616_d_n4, assign23650_e22616_d_n7, assign23650_e22616_d_n12, assign23650_e22616_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let (assign23650_e22606, assign23650_e22606_d_n2, assign23650_e22606_d_n4, assign23650_e22606_d_n7, assign23650_e22606_d_n12, assign23650_e22606_d_n13,) = {
            if (p.p52 != 0.0) {
                let assign23650_e22552: f64 = (-locals.var_fn277_calc_iq__vdsin);
                let assign23650_e22554: f64 = (assign23650_e22552 / locals.var_fn277_calc_iq__vdsat10);
                let assign23650_e22555: f64 = assign23650_e22554;
                let assign23650_e22558: f64 = (-locals.var_fn277_calc_iq__vdsin);
                let assign23650_e22560: f64 = (assign23650_e22558 / locals.var_fn277_calc_iq__vdsat10);
                let assign23650_e22561: f64 = (-assign23650_e22560);
                let assign23650_e22564: f64 = (0.001 / p.p53);
                let assign23650_e22567: f64 = (-locals.var_fn277_calc_iq__vdsin);
                let assign23650_e22569: f64 = (assign23650_e22567 / locals.var_fn277_calc_iq__vdsat10);
                let assign23650_e22570: f64 = (-assign23650_e22569);
                let assign23650_e22571: f64 = (assign23650_e22564 * assign23650_e22570);
                let assign23650_e22572: f64 = (assign23650_e22571).tanh();
                let assign23650_e22573: f64 = (assign23650_e22561 * assign23650_e22572);
                let assign23650_e22574: f64 = (assign23650_e22555 + assign23650_e22573);
                let assign23650_e22575: f64 = (0.5 * assign23650_e22574);
                (assign23650_e22575, (0.5 * ((-((assign23650_e22552 * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + (((-(-((assign23650_e22558 * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23650_e22572) + (assign23650_e22561 * ((assign23650_e22564 * (-(-((assign23650_e22567 * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))))) / ((assign23650_e22571).cosh() * (assign23650_e22571).cosh())))))), (0.5 * ((-((assign23650_e22552 * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + (((-(-((assign23650_e22558 * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23650_e22572) + (assign23650_e22561 * ((assign23650_e22564 * (-(-((assign23650_e22567 * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))))) / ((assign23650_e22571).cosh() * (assign23650_e22571).cosh())))))), (0.5 * ((-((assign23650_e22552 * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + (((-(-((assign23650_e22558 * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23650_e22572) + (assign23650_e22561 * ((assign23650_e22564 * (-(-((assign23650_e22567 * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))))) / ((assign23650_e22571).cosh() * (assign23650_e22571).cosh())))))), (0.5 * (((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22552 * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)) + (((-((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22558 * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) * assign23650_e22572) + (assign23650_e22561 * ((assign23650_e22564 * (-((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22567 * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) / ((assign23650_e22571).cosh() * (assign23650_e22571).cosh())))))), (0.5 * (((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22552 * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)) + (((-((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22558 * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) * assign23650_e22572) + (assign23650_e22561 * ((assign23650_e22564 * (-((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22567 * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) / ((assign23650_e22571).cosh() * (assign23650_e22571).cosh())))))),)
            } else {
                let (assign23650_e22605, assign23650_e22605_d_n2, assign23650_e22605_d_n4, assign23650_e22605_d_n7, assign23650_e22605_d_n12, assign23650_e22605_d_n13,) = {
                    if (p.p52 == 0.0) {
                        let assign23650_e22582: f64 = (-locals.var_fn277_calc_iq__vdsin);
                        let assign23650_e22584: f64 = (assign23650_e22582 / locals.var_fn277_calc_iq__vdsat10);
                        let assign23650_e22585: f64 = assign23650_e22584;
                        let assign23650_e22588: f64 = (-locals.var_fn277_calc_iq__vdsin);
                        let assign23650_e22590: f64 = (assign23650_e22588 / locals.var_fn277_calc_iq__vdsat10);
                        let assign23650_e22591: f64 = (-assign23650_e22590);
                        let assign23650_e22594: f64 = (-locals.var_fn277_calc_iq__vdsin);
                        let assign23650_e22596: f64 = (assign23650_e22594 / locals.var_fn277_calc_iq__vdsat10);
                        let assign23650_e22597: f64 = (-assign23650_e22596);
                        let assign23650_e22598: f64 = (assign23650_e22591 * assign23650_e22597);
                        let assign23650_e22600: f64 = (assign23650_e22598 + p.p53);
                        let assign23650_e22601: f64 = (assign23650_e22600).sqrt();
                        let assign23650_e22602: f64 = (assign23650_e22585 + assign23650_e22601);
                        let assign23650_e22603: f64 = (0.5 * assign23650_e22602);
                        (assign23650_e22603, (0.5 * ((-((assign23650_e22582 * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + ((((-(-((assign23650_e22588 * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23650_e22597) + (assign23650_e22591 * (-(-((assign23650_e22594 * locals.var_fn277_calc_iq__vdsat10_dn2) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))))) / (2.0 * assign23650_e22601)))), (0.5 * ((-((assign23650_e22582 * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + ((((-(-((assign23650_e22588 * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23650_e22597) + (assign23650_e22591 * (-(-((assign23650_e22594 * locals.var_fn277_calc_iq__vdsat10_dn4) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))))) / (2.0 * assign23650_e22601)))), (0.5 * ((-((assign23650_e22582 * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) + ((((-(-((assign23650_e22588 * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))) * assign23650_e22597) + (assign23650_e22591 * (-(-((assign23650_e22594 * locals.var_fn277_calc_iq__vdsat10_dn7) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)))))) / (2.0 * assign23650_e22601)))), (0.5 * (((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22582 * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)) + ((((-((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22588 * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) * assign23650_e22597) + (assign23650_e22591 * (-((((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22594 * locals.var_fn277_calc_iq__vdsat10_dn12)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))))) / (2.0 * assign23650_e22601)))), (0.5 * (((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22582 * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10)) + ((((-((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22588 * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))) * assign23650_e22597) + (assign23650_e22591 * (-((((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__vdsat10) - (assign23650_e22594 * locals.var_fn277_calc_iq__vdsat10_dn13)) / (locals.var_fn277_calc_iq__vdsat10 * locals.var_fn277_calc_iq__vdsat10))))) / (2.0 * assign23650_e22601)))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign23650_e22605, assign23650_e22605_d_n2, assign23650_e22605_d_n4, assign23650_e22605_d_n7, assign23650_e22605_d_n12, assign23650_e22605_d_n13,)
            }
        };
        let assign23650_e22608: f64 = (assign23650_e22606).powf(locals.var_fn277_calc_iq__beta);
        let assign23650_e22609: f64 = (1.0 + assign23650_e22608);
        let assign23650_e22612: f64 = (1.0 / locals.var_fn277_calc_iq__beta);
        let assign23650_e22613: f64 = (assign23650_e22609).powf(assign23650_e22612);
        let assign23650_e22614: f64 = (1.0 / assign23650_e22613);
        (assign23650_e22614, (-(if 0.0 == 0.0 && ((assign23650_e22612) as f64).is_finite() && ((assign23650_e22612) as f64).fract() == 0.0 { if assign23650_e22612 == 0.0 { 0.0 } else { (assign23650_e22612 * ((assign23650_e22609).powf(assign23650_e22612 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23650_e22606).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23650_e22606_d_n2)) } } else { (assign23650_e22608 * (locals.var_fn277_calc_iq__beta * (assign23650_e22606_d_n2 / assign23650_e22606))) })) } } else { (assign23650_e22613 * (assign23650_e22612 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23650_e22606).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23650_e22606_d_n2)) } } else { (assign23650_e22608 * (locals.var_fn277_calc_iq__beta * (assign23650_e22606_d_n2 / assign23650_e22606))) } / assign23650_e22609))) } / (assign23650_e22613 * assign23650_e22613))), (-(if 0.0 == 0.0 && ((assign23650_e22612) as f64).is_finite() && ((assign23650_e22612) as f64).fract() == 0.0 { if assign23650_e22612 == 0.0 { 0.0 } else { (assign23650_e22612 * ((assign23650_e22609).powf(assign23650_e22612 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23650_e22606).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23650_e22606_d_n4)) } } else { (assign23650_e22608 * (locals.var_fn277_calc_iq__beta * (assign23650_e22606_d_n4 / assign23650_e22606))) })) } } else { (assign23650_e22613 * (assign23650_e22612 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23650_e22606).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23650_e22606_d_n4)) } } else { (assign23650_e22608 * (locals.var_fn277_calc_iq__beta * (assign23650_e22606_d_n4 / assign23650_e22606))) } / assign23650_e22609))) } / (assign23650_e22613 * assign23650_e22613))), (-(if 0.0 == 0.0 && ((assign23650_e22612) as f64).is_finite() && ((assign23650_e22612) as f64).fract() == 0.0 { if assign23650_e22612 == 0.0 { 0.0 } else { (assign23650_e22612 * ((assign23650_e22609).powf(assign23650_e22612 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23650_e22606).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23650_e22606_d_n7)) } } else { (assign23650_e22608 * (locals.var_fn277_calc_iq__beta * (assign23650_e22606_d_n7 / assign23650_e22606))) })) } } else { (assign23650_e22613 * (assign23650_e22612 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23650_e22606).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23650_e22606_d_n7)) } } else { (assign23650_e22608 * (locals.var_fn277_calc_iq__beta * (assign23650_e22606_d_n7 / assign23650_e22606))) } / assign23650_e22609))) } / (assign23650_e22613 * assign23650_e22613))), (-(if 0.0 == 0.0 && ((assign23650_e22612) as f64).is_finite() && ((assign23650_e22612) as f64).fract() == 0.0 { if assign23650_e22612 == 0.0 { 0.0 } else { (assign23650_e22612 * ((assign23650_e22609).powf(assign23650_e22612 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23650_e22606).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23650_e22606_d_n12)) } } else { (assign23650_e22608 * (locals.var_fn277_calc_iq__beta * (assign23650_e22606_d_n12 / assign23650_e22606))) })) } } else { (assign23650_e22613 * (assign23650_e22612 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23650_e22606).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23650_e22606_d_n12)) } } else { (assign23650_e22608 * (locals.var_fn277_calc_iq__beta * (assign23650_e22606_d_n12 / assign23650_e22606))) } / assign23650_e22609))) } / (assign23650_e22613 * assign23650_e22613))), (-(if 0.0 == 0.0 && ((assign23650_e22612) as f64).is_finite() && ((assign23650_e22612) as f64).fract() == 0.0 { if assign23650_e22612 == 0.0 { 0.0 } else { (assign23650_e22612 * ((assign23650_e22609).powf(assign23650_e22612 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23650_e22606).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23650_e22606_d_n13)) } } else { (assign23650_e22608 * (locals.var_fn277_calc_iq__beta * (assign23650_e22606_d_n13 / assign23650_e22606))) })) } } else { (assign23650_e22613 * (assign23650_e22612 * (if 0.0 == 0.0 && ((locals.var_fn277_calc_iq__beta) as f64).is_finite() && ((locals.var_fn277_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn277_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn277_calc_iq__beta * ((assign23650_e22606).powf(locals.var_fn277_calc_iq__beta - 1.0) * assign23650_e22606_d_n13)) } } else { (assign23650_e22608 * (locals.var_fn277_calc_iq__beta * (assign23650_e22606_d_n13 / assign23650_e22606))) } / assign23650_e22609))) } / (assign23650_e22613 * assign23650_e22613))),)
    } else {
        (locals.var_fn277_calc_iq__fds0, locals.var_fn277_calc_iq__fds0_dn2, locals.var_fn277_calc_iq__fds0_dn4, locals.var_fn277_calc_iq__fds0_dn7, locals.var_fn277_calc_iq__fds0_dn12, locals.var_fn277_calc_iq__fds0_dn13,)
    }
};
        locals.var_fn277_calc_iq__fds0 = assign23650_e22616;
        locals.var_fn277_calc_iq__fds0_dn2 = assign23650_e22616_d_n2;
        locals.var_fn277_calc_iq__fds0_dn4 = assign23650_e22616_d_n4;
        locals.var_fn277_calc_iq__fds0_dn7 = assign23650_e22616_d_n7;
        locals.var_fn277_calc_iq__fds0_dn12 = assign23650_e22616_d_n12;
        locals.var_fn277_calc_iq__fds0_dn13 = assign23650_e22616_d_n13;
        locals.var_fn277_calc_iq__fds0_rv = 0.0;

        let (assign23660_e22623, assign23660_e22623_d_n2, assign23660_e22623_d_n4, assign23660_e22623_d_n7, assign23660_e22623_d_n12, assign23660_e22623_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23660_e22619: f64 = (-locals.var_fn277_calc_iq__vdsin);
        let assign23660_e22621: f64 = (assign23660_e22619 * locals.var_fn277_calc_iq__fds0);
        (assign23660_e22621, (assign23660_e22619 * locals.var_fn277_calc_iq__fds0_dn2), (assign23660_e22619 * locals.var_fn277_calc_iq__fds0_dn4), (assign23660_e22619 * locals.var_fn277_calc_iq__fds0_dn7), (((-locals.var_fn277_calc_iq__vdsin_dn12) * locals.var_fn277_calc_iq__fds0) + (assign23660_e22619 * locals.var_fn277_calc_iq__fds0_dn12)), (((-locals.var_fn277_calc_iq__vdsin_dn13) * locals.var_fn277_calc_iq__fds0) + (assign23660_e22619 * locals.var_fn277_calc_iq__fds0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__vsx0, locals.var_fn277_calc_iq__vsx0_dn2, locals.var_fn277_calc_iq__vsx0_dn4, locals.var_fn277_calc_iq__vsx0_dn7, locals.var_fn277_calc_iq__vsx0_dn12, locals.var_fn277_calc_iq__vsx0_dn13,)
    }
};
        locals.var_fn277_calc_iq__vsx0 = assign23660_e22623;
        locals.var_fn277_calc_iq__vsx0_dn2 = assign23660_e22623_d_n2;
        locals.var_fn277_calc_iq__vsx0_dn4 = assign23660_e22623_d_n4;
        locals.var_fn277_calc_iq__vsx0_dn7 = assign23660_e22623_d_n7;
        locals.var_fn277_calc_iq__vsx0_dn12 = assign23660_e22623_d_n12;
        locals.var_fn277_calc_iq__vsx0_dn13 = assign23660_e22623_d_n13;
        locals.var_fn277_calc_iq__vsx0_rv = 0.0;

        let (assign23670_e22631, assign23670_e22631_d_n2, assign23670_e22631_d_n4, assign23670_e22631_d_n7, assign23670_e22631_d_n12, assign23670_e22631_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23670_e22627: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__myarg0);
        let assign23670_e22629: f64 = (assign23670_e22627 / locals.var_fn277_calc_iq__alpha_phit);
        (assign23670_e22629, (locals.var_fn277_calc_iq__vgsin_dn2 / locals.var_fn277_calc_iq__alpha_phit), ((((-locals.var_fn277_calc_iq__myarg0_dn4) * locals.var_fn277_calc_iq__alpha_phit) - (assign23670_e22627 * locals.var_fn277_calc_iq__alpha_phit_dn4)) / (locals.var_fn277_calc_iq__alpha_phit * locals.var_fn277_calc_iq__alpha_phit)), (locals.var_fn277_calc_iq__vgsin_dn7 / locals.var_fn277_calc_iq__alpha_phit), 0.0, (locals.var_fn277_calc_iq__vgsin_dn13 / locals.var_fn277_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn277_calc_iq__exparg0, locals.var_fn277_calc_iq__exparg0_dn2, locals.var_fn277_calc_iq__exparg0_dn4, locals.var_fn277_calc_iq__exparg0_dn7, locals.var_fn277_calc_iq__exparg0_dn12, locals.var_fn277_calc_iq__exparg0_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg0 = assign23670_e22631;
        locals.var_fn277_calc_iq__exparg0_dn2 = assign23670_e22631_d_n2;
        locals.var_fn277_calc_iq__exparg0_dn4 = assign23670_e22631_d_n4;
        locals.var_fn277_calc_iq__exparg0_dn7 = assign23670_e22631_d_n7;
        locals.var_fn277_calc_iq__exparg0_dn12 = assign23670_e22631_d_n12;
        locals.var_fn277_calc_iq__exparg0_dn13 = assign23670_e22631_d_n13;
        locals.var_fn277_calc_iq__exparg0_rv = 0.0;

        let assign23680_e22634: f64 = if locals.var_fn277_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard295 = assign23680_e22634;
        locals.var_guard295_rv = 0.0;

        let (assign23690_e22640, assign23690_e22640_d_n2, assign23690_e22640_d_n4, assign23690_e22640_d_n7, assign23690_e22640_d_n12, assign23690_e22640_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard295 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffs0, locals.var_fn277_calc_iq__ffs0_dn2, locals.var_fn277_calc_iq__ffs0_dn4, locals.var_fn277_calc_iq__ffs0_dn7, locals.var_fn277_calc_iq__ffs0_dn12, locals.var_fn277_calc_iq__ffs0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffs0 = assign23690_e22640;
        locals.var_fn277_calc_iq__ffs0_dn2 = assign23690_e22640_d_n2;
        locals.var_fn277_calc_iq__ffs0_dn4 = assign23690_e22640_d_n4;
        locals.var_fn277_calc_iq__ffs0_dn7 = assign23690_e22640_d_n7;
        locals.var_fn277_calc_iq__ffs0_dn12 = assign23690_e22640_d_n12;
        locals.var_fn277_calc_iq__ffs0_dn13 = assign23690_e22640_d_n13;
        locals.var_fn277_calc_iq__ffs0_rv = 0.0;

        let assign23700_e22643: f64 = (-50.0);
        let assign23700_e22644: f64 = if locals.var_fn277_calc_iq__exparg0 < assign23700_e22643 { 1.0 } else { 0.0 };
        locals.var_guard296 = assign23700_e22644;
        locals.var_guard296_rv = 0.0;

        let (assign23710_e22653, assign23710_e22653_d_n2, assign23710_e22653_d_n4, assign23710_e22653_d_n7, assign23710_e22653_d_n12, assign23710_e22653_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard295 == 0.0)) && (locals.var_guard296 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffs0, locals.var_fn277_calc_iq__ffs0_dn2, locals.var_fn277_calc_iq__ffs0_dn4, locals.var_fn277_calc_iq__ffs0_dn7, locals.var_fn277_calc_iq__ffs0_dn12, locals.var_fn277_calc_iq__ffs0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffs0 = assign23710_e22653;
        locals.var_fn277_calc_iq__ffs0_dn2 = assign23710_e22653_d_n2;
        locals.var_fn277_calc_iq__ffs0_dn4 = assign23710_e22653_d_n4;
        locals.var_fn277_calc_iq__ffs0_dn7 = assign23710_e22653_d_n7;
        locals.var_fn277_calc_iq__ffs0_dn12 = assign23710_e22653_d_n12;
        locals.var_fn277_calc_iq__ffs0_dn13 = assign23710_e22653_d_n13;
        locals.var_fn277_calc_iq__ffs0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_65(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign23720_e22668, assign23720_e22668_d_n2, assign23720_e22668_d_n4, assign23720_e22668_d_n7, assign23720_e22668_d_n12, assign23720_e22668_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard295 == 0.0)) && (locals.var_guard296 == 0.0)) {
        let assign23720_e22664: f64 = (locals.var_fn277_calc_iq__exparg0).exp();
        let assign23720_e22665: f64 = (1.0 + assign23720_e22664);
        let assign23720_e22666: f64 = (1.0 / assign23720_e22665);
        (assign23720_e22666, (-((assign23720_e22664 * locals.var_fn277_calc_iq__exparg0_dn2) / (assign23720_e22665 * assign23720_e22665))), (-((assign23720_e22664 * locals.var_fn277_calc_iq__exparg0_dn4) / (assign23720_e22665 * assign23720_e22665))), (-((assign23720_e22664 * locals.var_fn277_calc_iq__exparg0_dn7) / (assign23720_e22665 * assign23720_e22665))), (-((assign23720_e22664 * locals.var_fn277_calc_iq__exparg0_dn12) / (assign23720_e22665 * assign23720_e22665))), (-((assign23720_e22664 * locals.var_fn277_calc_iq__exparg0_dn13) / (assign23720_e22665 * assign23720_e22665))),)
    } else {
        (locals.var_fn277_calc_iq__ffs0, locals.var_fn277_calc_iq__ffs0_dn2, locals.var_fn277_calc_iq__ffs0_dn4, locals.var_fn277_calc_iq__ffs0_dn7, locals.var_fn277_calc_iq__ffs0_dn12, locals.var_fn277_calc_iq__ffs0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffs0 = assign23720_e22668;
        locals.var_fn277_calc_iq__ffs0_dn2 = assign23720_e22668_d_n2;
        locals.var_fn277_calc_iq__ffs0_dn4 = assign23720_e22668_d_n4;
        locals.var_fn277_calc_iq__ffs0_dn7 = assign23720_e22668_d_n7;
        locals.var_fn277_calc_iq__ffs0_dn12 = assign23720_e22668_d_n12;
        locals.var_fn277_calc_iq__ffs0_dn13 = assign23720_e22668_d_n13;
        locals.var_fn277_calc_iq__ffs0_rv = 0.0;

        let (assign23730_e22686, assign23730_e22686_d_n2, assign23730_e22686_d_n4, assign23730_e22686_d_n7, assign23730_e22686_d_n12, assign23730_e22686_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23730_e22672: f64 = (locals.var_fn277_calc_iq__vgdin - locals.var_fn277_calc_iq__vsx0);
        let assign23730_e22676: f64 = (p.p51 * 0.1);
        let assign23730_e22678: f64 = (assign23730_e22676 * locals.var_fn277_calc_iq__alpha_phit);
        let assign23730_e22680: f64 = (assign23730_e22678 * locals.var_fn277_calc_iq__ffs0);
        let assign23730_e22681: f64 = (locals.var_fn277_calc_iq__vtof - assign23730_e22680);
        let assign23730_e22682: f64 = (assign23730_e22672 - assign23730_e22681);
        let assign23730_e22684: f64 = (assign23730_e22682 / locals.var_fn277_calc_iq__two_n_phit0);
        (assign23730_e22684, (((locals.var_fn277_calc_iq__vgdin_dn2 - locals.var_fn277_calc_iq__vsx0_dn2) - (-(assign23730_e22678 * locals.var_fn277_calc_iq__ffs0_dn2))) / locals.var_fn277_calc_iq__two_n_phit0), (((((-locals.var_fn277_calc_iq__vsx0_dn4) - (locals.var_fn277_calc_iq__vtof_dn4 - (((assign23730_e22676 * locals.var_fn277_calc_iq__alpha_phit_dn4) * locals.var_fn277_calc_iq__ffs0) + (assign23730_e22678 * locals.var_fn277_calc_iq__ffs0_dn4)))) * locals.var_fn277_calc_iq__two_n_phit0) - (assign23730_e22682 * locals.var_fn277_calc_iq__two_n_phit0_dn4)) / (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__two_n_phit0)), (((locals.var_fn277_calc_iq__vgdin_dn7 - locals.var_fn277_calc_iq__vsx0_dn7) - (-(assign23730_e22678 * locals.var_fn277_calc_iq__ffs0_dn7))) / locals.var_fn277_calc_iq__two_n_phit0), (((locals.var_fn277_calc_iq__vgdin_dn12 - locals.var_fn277_calc_iq__vsx0_dn12) - (-(assign23730_e22678 * locals.var_fn277_calc_iq__ffs0_dn12))) / locals.var_fn277_calc_iq__two_n_phit0), (((locals.var_fn277_calc_iq__vgdin_dn13 - locals.var_fn277_calc_iq__vsx0_dn13) - (-(assign23730_e22678 * locals.var_fn277_calc_iq__ffs0_dn13))) / locals.var_fn277_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn277_calc_iq__etas0, locals.var_fn277_calc_iq__etas0_dn2, locals.var_fn277_calc_iq__etas0_dn4, locals.var_fn277_calc_iq__etas0_dn7, locals.var_fn277_calc_iq__etas0_dn12, locals.var_fn277_calc_iq__etas0_dn13,)
    }
};
        locals.var_fn277_calc_iq__etas0 = assign23730_e22686;
        locals.var_fn277_calc_iq__etas0_dn2 = assign23730_e22686_d_n2;
        locals.var_fn277_calc_iq__etas0_dn4 = assign23730_e22686_d_n4;
        locals.var_fn277_calc_iq__etas0_dn7 = assign23730_e22686_d_n7;
        locals.var_fn277_calc_iq__etas0_dn12 = assign23730_e22686_d_n12;
        locals.var_fn277_calc_iq__etas0_dn13 = assign23730_e22686_d_n13;
        locals.var_fn277_calc_iq__etas0_rv = 0.0;

        let assign23740_e22689: f64 = if locals.var_fn277_calc_iq__etas0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard297 = assign23740_e22689;
        locals.var_guard297_rv = 0.0;

        let (assign23750_e22697, assign23750_e22697_d_n2, assign23750_e22697_d_n4, assign23750_e22697_d_n7, assign23750_e22697_d_n12, assign23750_e22697_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard297 != 0.0)) {
        let assign23750_e22695: f64 = (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etas0);
        (assign23750_e22695, (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etas0_dn2), ((locals.var_fn277_calc_iq__qref0_dn4 * locals.var_fn277_calc_iq__etas0) + (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etas0_dn4)), (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etas0_dn7), (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etas0_dn12), (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etas0_dn13),)
    } else {
        (locals.var_fn277_calc_iq__qinvs0, locals.var_fn277_calc_iq__qinvs0_dn2, locals.var_fn277_calc_iq__qinvs0_dn4, locals.var_fn277_calc_iq__qinvs0_dn7, locals.var_fn277_calc_iq__qinvs0_dn12, locals.var_fn277_calc_iq__qinvs0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvs0 = assign23750_e22697;
        locals.var_fn277_calc_iq__qinvs0_dn2 = assign23750_e22697_d_n2;
        locals.var_fn277_calc_iq__qinvs0_dn4 = assign23750_e22697_d_n4;
        locals.var_fn277_calc_iq__qinvs0_dn7 = assign23750_e22697_d_n7;
        locals.var_fn277_calc_iq__qinvs0_dn12 = assign23750_e22697_d_n12;
        locals.var_fn277_calc_iq__qinvs0_dn13 = assign23750_e22697_d_n13;
        locals.var_fn277_calc_iq__qinvs0_rv = 0.0;

        let assign23760_e22700: f64 = (-50.0);
        let assign23760_e22701: f64 = if locals.var_fn277_calc_iq__etas0 < assign23760_e22700 { 1.0 } else { 0.0 };
        locals.var_guard298 = assign23760_e22701;
        locals.var_guard298_rv = 0.0;

        let (assign23770_e22713, assign23770_e22713_d_n2, assign23770_e22713_d_n4, assign23770_e22713_d_n7, assign23770_e22713_d_n12, assign23770_e22713_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard297 == 0.0)) && (locals.var_guard298 != 0.0)) {
        let assign23770_e22710: f64 = (locals.var_fn277_calc_iq__etas0).exp();
        let assign23770_e22711: f64 = (locals.var_fn277_calc_iq__qref0 * assign23770_e22710);
        (assign23770_e22711, (locals.var_fn277_calc_iq__qref0 * (assign23770_e22710 * locals.var_fn277_calc_iq__etas0_dn2)), ((locals.var_fn277_calc_iq__qref0_dn4 * assign23770_e22710) + (locals.var_fn277_calc_iq__qref0 * (assign23770_e22710 * locals.var_fn277_calc_iq__etas0_dn4))), (locals.var_fn277_calc_iq__qref0 * (assign23770_e22710 * locals.var_fn277_calc_iq__etas0_dn7)), (locals.var_fn277_calc_iq__qref0 * (assign23770_e22710 * locals.var_fn277_calc_iq__etas0_dn12)), (locals.var_fn277_calc_iq__qref0 * (assign23770_e22710 * locals.var_fn277_calc_iq__etas0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qinvs0, locals.var_fn277_calc_iq__qinvs0_dn2, locals.var_fn277_calc_iq__qinvs0_dn4, locals.var_fn277_calc_iq__qinvs0_dn7, locals.var_fn277_calc_iq__qinvs0_dn12, locals.var_fn277_calc_iq__qinvs0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvs0 = assign23770_e22713;
        locals.var_fn277_calc_iq__qinvs0_dn2 = assign23770_e22713_d_n2;
        locals.var_fn277_calc_iq__qinvs0_dn4 = assign23770_e22713_d_n4;
        locals.var_fn277_calc_iq__qinvs0_dn7 = assign23770_e22713_d_n7;
        locals.var_fn277_calc_iq__qinvs0_dn12 = assign23770_e22713_d_n12;
        locals.var_fn277_calc_iq__qinvs0_dn13 = assign23770_e22713_d_n13;
        locals.var_fn277_calc_iq__qinvs0_rv = 0.0;

        let (assign23780_e22729, assign23780_e22729_d_n2, assign23780_e22729_d_n4, assign23780_e22729_d_n7, assign23780_e22729_d_n12, assign23780_e22729_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard297 == 0.0)) && (locals.var_guard298 == 0.0)) {
        let assign23780_e22724: f64 = (locals.var_fn277_calc_iq__etas0).exp();
        let assign23780_e22725: f64 = (1.0 + assign23780_e22724);
        let assign23780_e22726: f64 = (assign23780_e22725).ln();
        let assign23780_e22727: f64 = (locals.var_fn277_calc_iq__qref0 * assign23780_e22726);
        (assign23780_e22727, (locals.var_fn277_calc_iq__qref0 * ((assign23780_e22724 * locals.var_fn277_calc_iq__etas0_dn2) / assign23780_e22725)), ((locals.var_fn277_calc_iq__qref0_dn4 * assign23780_e22726) + (locals.var_fn277_calc_iq__qref0 * ((assign23780_e22724 * locals.var_fn277_calc_iq__etas0_dn4) / assign23780_e22725))), (locals.var_fn277_calc_iq__qref0 * ((assign23780_e22724 * locals.var_fn277_calc_iq__etas0_dn7) / assign23780_e22725)), (locals.var_fn277_calc_iq__qref0 * ((assign23780_e22724 * locals.var_fn277_calc_iq__etas0_dn12) / assign23780_e22725)), (locals.var_fn277_calc_iq__qref0 * ((assign23780_e22724 * locals.var_fn277_calc_iq__etas0_dn13) / assign23780_e22725)),)
    } else {
        (locals.var_fn277_calc_iq__qinvs0, locals.var_fn277_calc_iq__qinvs0_dn2, locals.var_fn277_calc_iq__qinvs0_dn4, locals.var_fn277_calc_iq__qinvs0_dn7, locals.var_fn277_calc_iq__qinvs0_dn12, locals.var_fn277_calc_iq__qinvs0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvs0 = assign23780_e22729;
        locals.var_fn277_calc_iq__qinvs0_dn2 = assign23780_e22729_d_n2;
        locals.var_fn277_calc_iq__qinvs0_dn4 = assign23780_e22729_d_n4;
        locals.var_fn277_calc_iq__qinvs0_dn7 = assign23780_e22729_d_n7;
        locals.var_fn277_calc_iq__qinvs0_dn12 = assign23780_e22729_d_n12;
        locals.var_fn277_calc_iq__qinvs0_dn13 = assign23780_e22729_d_n13;
        locals.var_fn277_calc_iq__qinvs0_rv = 0.0;

        let (assign23790_e22737, assign23790_e22737_d_n2, assign23790_e22737_d_n4, assign23790_e22737_d_n7, assign23790_e22737_d_n12, assign23790_e22737_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23790_e22733: f64 = (locals.var_fn277_calc_iq__vgdin - locals.var_fn277_calc_iq__myarg0);
        let assign23790_e22735: f64 = (assign23790_e22733 / locals.var_fn277_calc_iq__alpha_phit);
        (assign23790_e22735, (locals.var_fn277_calc_iq__vgdin_dn2 / locals.var_fn277_calc_iq__alpha_phit), ((((-locals.var_fn277_calc_iq__myarg0_dn4) * locals.var_fn277_calc_iq__alpha_phit) - (assign23790_e22733 * locals.var_fn277_calc_iq__alpha_phit_dn4)) / (locals.var_fn277_calc_iq__alpha_phit * locals.var_fn277_calc_iq__alpha_phit)), (locals.var_fn277_calc_iq__vgdin_dn7 / locals.var_fn277_calc_iq__alpha_phit), (locals.var_fn277_calc_iq__vgdin_dn12 / locals.var_fn277_calc_iq__alpha_phit), (locals.var_fn277_calc_iq__vgdin_dn13 / locals.var_fn277_calc_iq__alpha_phit),)
    } else {
        (locals.var_fn277_calc_iq__exparg0, locals.var_fn277_calc_iq__exparg0_dn2, locals.var_fn277_calc_iq__exparg0_dn4, locals.var_fn277_calc_iq__exparg0_dn7, locals.var_fn277_calc_iq__exparg0_dn12, locals.var_fn277_calc_iq__exparg0_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg0 = assign23790_e22737;
        locals.var_fn277_calc_iq__exparg0_dn2 = assign23790_e22737_d_n2;
        locals.var_fn277_calc_iq__exparg0_dn4 = assign23790_e22737_d_n4;
        locals.var_fn277_calc_iq__exparg0_dn7 = assign23790_e22737_d_n7;
        locals.var_fn277_calc_iq__exparg0_dn12 = assign23790_e22737_d_n12;
        locals.var_fn277_calc_iq__exparg0_dn13 = assign23790_e22737_d_n13;
        locals.var_fn277_calc_iq__exparg0_rv = 0.0;

        let assign23800_e22740: f64 = if locals.var_fn277_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard299 = assign23800_e22740;
        locals.var_guard299_rv = 0.0;

        let (assign23810_e22746, assign23810_e22746_d_n2, assign23810_e22746_d_n4, assign23810_e22746_d_n7, assign23810_e22746_d_n12, assign23810_e22746_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard299 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffd0, locals.var_fn277_calc_iq__ffd0_dn2, locals.var_fn277_calc_iq__ffd0_dn4, locals.var_fn277_calc_iq__ffd0_dn7, locals.var_fn277_calc_iq__ffd0_dn12, locals.var_fn277_calc_iq__ffd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffd0 = assign23810_e22746;
        locals.var_fn277_calc_iq__ffd0_dn2 = assign23810_e22746_d_n2;
        locals.var_fn277_calc_iq__ffd0_dn4 = assign23810_e22746_d_n4;
        locals.var_fn277_calc_iq__ffd0_dn7 = assign23810_e22746_d_n7;
        locals.var_fn277_calc_iq__ffd0_dn12 = assign23810_e22746_d_n12;
        locals.var_fn277_calc_iq__ffd0_dn13 = assign23810_e22746_d_n13;
        locals.var_fn277_calc_iq__ffd0_rv = 0.0;

        let assign23820_e22749: f64 = (-50.0);
        let assign23820_e22750: f64 = if locals.var_fn277_calc_iq__exparg0 < assign23820_e22749 { 1.0 } else { 0.0 };
        locals.var_guard300 = assign23820_e22750;
        locals.var_guard300_rv = 0.0;

        let (assign23830_e22759, assign23830_e22759_d_n2, assign23830_e22759_d_n4, assign23830_e22759_d_n7, assign23830_e22759_d_n12, assign23830_e22759_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard299 == 0.0)) && (locals.var_guard300 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__ffd0, locals.var_fn277_calc_iq__ffd0_dn2, locals.var_fn277_calc_iq__ffd0_dn4, locals.var_fn277_calc_iq__ffd0_dn7, locals.var_fn277_calc_iq__ffd0_dn12, locals.var_fn277_calc_iq__ffd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffd0 = assign23830_e22759;
        locals.var_fn277_calc_iq__ffd0_dn2 = assign23830_e22759_d_n2;
        locals.var_fn277_calc_iq__ffd0_dn4 = assign23830_e22759_d_n4;
        locals.var_fn277_calc_iq__ffd0_dn7 = assign23830_e22759_d_n7;
        locals.var_fn277_calc_iq__ffd0_dn12 = assign23830_e22759_d_n12;
        locals.var_fn277_calc_iq__ffd0_dn13 = assign23830_e22759_d_n13;
        locals.var_fn277_calc_iq__ffd0_rv = 0.0;

        let (assign23840_e22774, assign23840_e22774_d_n2, assign23840_e22774_d_n4, assign23840_e22774_d_n7, assign23840_e22774_d_n12, assign23840_e22774_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard299 == 0.0)) && (locals.var_guard300 == 0.0)) {
        let assign23840_e22770: f64 = (locals.var_fn277_calc_iq__exparg0).exp();
        let assign23840_e22771: f64 = (1.0 + assign23840_e22770);
        let assign23840_e22772: f64 = (1.0 / assign23840_e22771);
        (assign23840_e22772, (-((assign23840_e22770 * locals.var_fn277_calc_iq__exparg0_dn2) / (assign23840_e22771 * assign23840_e22771))), (-((assign23840_e22770 * locals.var_fn277_calc_iq__exparg0_dn4) / (assign23840_e22771 * assign23840_e22771))), (-((assign23840_e22770 * locals.var_fn277_calc_iq__exparg0_dn7) / (assign23840_e22771 * assign23840_e22771))), (-((assign23840_e22770 * locals.var_fn277_calc_iq__exparg0_dn12) / (assign23840_e22771 * assign23840_e22771))), (-((assign23840_e22770 * locals.var_fn277_calc_iq__exparg0_dn13) / (assign23840_e22771 * assign23840_e22771))),)
    } else {
        (locals.var_fn277_calc_iq__ffd0, locals.var_fn277_calc_iq__ffd0_dn2, locals.var_fn277_calc_iq__ffd0_dn4, locals.var_fn277_calc_iq__ffd0_dn7, locals.var_fn277_calc_iq__ffd0_dn12, locals.var_fn277_calc_iq__ffd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__ffd0 = assign23840_e22774;
        locals.var_fn277_calc_iq__ffd0_dn2 = assign23840_e22774_d_n2;
        locals.var_fn277_calc_iq__ffd0_dn4 = assign23840_e22774_d_n4;
        locals.var_fn277_calc_iq__ffd0_dn7 = assign23840_e22774_d_n7;
        locals.var_fn277_calc_iq__ffd0_dn12 = assign23840_e22774_d_n12;
        locals.var_fn277_calc_iq__ffd0_dn13 = assign23840_e22774_d_n13;
        locals.var_fn277_calc_iq__ffd0_rv = 0.0;

        let (assign23850_e22792, assign23850_e22792_d_n2, assign23850_e22792_d_n4, assign23850_e22792_d_n7, assign23850_e22792_d_n12, assign23850_e22792_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23850_e22778: f64 = (locals.var_fn277_calc_iq__vgsin - locals.var_fn277_calc_iq__vdx0);
        let assign23850_e22782: f64 = (p.p51 * 0.1);
        let assign23850_e22784: f64 = (assign23850_e22782 * locals.var_fn277_calc_iq__alpha_phit);
        let assign23850_e22786: f64 = (assign23850_e22784 * locals.var_fn277_calc_iq__ffd0);
        let assign23850_e22787: f64 = (locals.var_fn277_calc_iq__vtof - assign23850_e22786);
        let assign23850_e22788: f64 = (assign23850_e22778 - assign23850_e22787);
        let assign23850_e22790: f64 = (assign23850_e22788 / locals.var_fn277_calc_iq__two_n_phit0);
        (assign23850_e22790, (((locals.var_fn277_calc_iq__vgsin_dn2 - locals.var_fn277_calc_iq__vdx0_dn2) - (-(assign23850_e22784 * locals.var_fn277_calc_iq__ffd0_dn2))) / locals.var_fn277_calc_iq__two_n_phit0), (((((-locals.var_fn277_calc_iq__vdx0_dn4) - (locals.var_fn277_calc_iq__vtof_dn4 - (((assign23850_e22782 * locals.var_fn277_calc_iq__alpha_phit_dn4) * locals.var_fn277_calc_iq__ffd0) + (assign23850_e22784 * locals.var_fn277_calc_iq__ffd0_dn4)))) * locals.var_fn277_calc_iq__two_n_phit0) - (assign23850_e22788 * locals.var_fn277_calc_iq__two_n_phit0_dn4)) / (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__two_n_phit0)), (((locals.var_fn277_calc_iq__vgsin_dn7 - locals.var_fn277_calc_iq__vdx0_dn7) - (-(assign23850_e22784 * locals.var_fn277_calc_iq__ffd0_dn7))) / locals.var_fn277_calc_iq__two_n_phit0), (((-locals.var_fn277_calc_iq__vdx0_dn12) - (-(assign23850_e22784 * locals.var_fn277_calc_iq__ffd0_dn12))) / locals.var_fn277_calc_iq__two_n_phit0), (((locals.var_fn277_calc_iq__vgsin_dn13 - locals.var_fn277_calc_iq__vdx0_dn13) - (-(assign23850_e22784 * locals.var_fn277_calc_iq__ffd0_dn13))) / locals.var_fn277_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn277_calc_iq__etad0, locals.var_fn277_calc_iq__etad0_dn2, locals.var_fn277_calc_iq__etad0_dn4, locals.var_fn277_calc_iq__etad0_dn7, locals.var_fn277_calc_iq__etad0_dn12, locals.var_fn277_calc_iq__etad0_dn13,)
    }
};
        locals.var_fn277_calc_iq__etad0 = assign23850_e22792;
        locals.var_fn277_calc_iq__etad0_dn2 = assign23850_e22792_d_n2;
        locals.var_fn277_calc_iq__etad0_dn4 = assign23850_e22792_d_n4;
        locals.var_fn277_calc_iq__etad0_dn7 = assign23850_e22792_d_n7;
        locals.var_fn277_calc_iq__etad0_dn12 = assign23850_e22792_d_n12;
        locals.var_fn277_calc_iq__etad0_dn13 = assign23850_e22792_d_n13;
        locals.var_fn277_calc_iq__etad0_rv = 0.0;

        let assign23860_e22795: f64 = if locals.var_fn277_calc_iq__etad0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard301 = assign23860_e22795;
        locals.var_guard301_rv = 0.0;

        let (assign23870_e22803, assign23870_e22803_d_n2, assign23870_e22803_d_n4, assign23870_e22803_d_n7, assign23870_e22803_d_n12, assign23870_e22803_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard301 != 0.0)) {
        let assign23870_e22801: f64 = (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etad0);
        (assign23870_e22801, (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etad0_dn2), ((locals.var_fn277_calc_iq__qref0_dn4 * locals.var_fn277_calc_iq__etad0) + (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etad0_dn4)), (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etad0_dn7), (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etad0_dn12), (locals.var_fn277_calc_iq__qref0 * locals.var_fn277_calc_iq__etad0_dn13),)
    } else {
        (locals.var_fn277_calc_iq__qinvd0, locals.var_fn277_calc_iq__qinvd0_dn2, locals.var_fn277_calc_iq__qinvd0_dn4, locals.var_fn277_calc_iq__qinvd0_dn7, locals.var_fn277_calc_iq__qinvd0_dn12, locals.var_fn277_calc_iq__qinvd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvd0 = assign23870_e22803;
        locals.var_fn277_calc_iq__qinvd0_dn2 = assign23870_e22803_d_n2;
        locals.var_fn277_calc_iq__qinvd0_dn4 = assign23870_e22803_d_n4;
        locals.var_fn277_calc_iq__qinvd0_dn7 = assign23870_e22803_d_n7;
        locals.var_fn277_calc_iq__qinvd0_dn12 = assign23870_e22803_d_n12;
        locals.var_fn277_calc_iq__qinvd0_dn13 = assign23870_e22803_d_n13;
        locals.var_fn277_calc_iq__qinvd0_rv = 0.0;

        let assign23880_e22806: f64 = (-50.0);
        let assign23880_e22807: f64 = if locals.var_fn277_calc_iq__etad0 < assign23880_e22806 { 1.0 } else { 0.0 };
        locals.var_guard302 = assign23880_e22807;
        locals.var_guard302_rv = 0.0;

        let (assign23890_e22819, assign23890_e22819_d_n2, assign23890_e22819_d_n4, assign23890_e22819_d_n7, assign23890_e22819_d_n12, assign23890_e22819_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard302 != 0.0)) {
        let assign23890_e22816: f64 = (locals.var_fn277_calc_iq__etad0).exp();
        let assign23890_e22817: f64 = (locals.var_fn277_calc_iq__qref0 * assign23890_e22816);
        (assign23890_e22817, (locals.var_fn277_calc_iq__qref0 * (assign23890_e22816 * locals.var_fn277_calc_iq__etad0_dn2)), ((locals.var_fn277_calc_iq__qref0_dn4 * assign23890_e22816) + (locals.var_fn277_calc_iq__qref0 * (assign23890_e22816 * locals.var_fn277_calc_iq__etad0_dn4))), (locals.var_fn277_calc_iq__qref0 * (assign23890_e22816 * locals.var_fn277_calc_iq__etad0_dn7)), (locals.var_fn277_calc_iq__qref0 * (assign23890_e22816 * locals.var_fn277_calc_iq__etad0_dn12)), (locals.var_fn277_calc_iq__qref0 * (assign23890_e22816 * locals.var_fn277_calc_iq__etad0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qinvd0, locals.var_fn277_calc_iq__qinvd0_dn2, locals.var_fn277_calc_iq__qinvd0_dn4, locals.var_fn277_calc_iq__qinvd0_dn7, locals.var_fn277_calc_iq__qinvd0_dn12, locals.var_fn277_calc_iq__qinvd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvd0 = assign23890_e22819;
        locals.var_fn277_calc_iq__qinvd0_dn2 = assign23890_e22819_d_n2;
        locals.var_fn277_calc_iq__qinvd0_dn4 = assign23890_e22819_d_n4;
        locals.var_fn277_calc_iq__qinvd0_dn7 = assign23890_e22819_d_n7;
        locals.var_fn277_calc_iq__qinvd0_dn12 = assign23890_e22819_d_n12;
        locals.var_fn277_calc_iq__qinvd0_dn13 = assign23890_e22819_d_n13;
        locals.var_fn277_calc_iq__qinvd0_rv = 0.0;

        let (assign23900_e22835, assign23900_e22835_d_n2, assign23900_e22835_d_n4, assign23900_e22835_d_n7, assign23900_e22835_d_n12, assign23900_e22835_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard301 == 0.0)) && (locals.var_guard302 == 0.0)) {
        let assign23900_e22830: f64 = (locals.var_fn277_calc_iq__etad0).exp();
        let assign23900_e22831: f64 = (1.0 + assign23900_e22830);
        let assign23900_e22832: f64 = (assign23900_e22831).ln();
        let assign23900_e22833: f64 = (locals.var_fn277_calc_iq__qref0 * assign23900_e22832);
        (assign23900_e22833, (locals.var_fn277_calc_iq__qref0 * ((assign23900_e22830 * locals.var_fn277_calc_iq__etad0_dn2) / assign23900_e22831)), ((locals.var_fn277_calc_iq__qref0_dn4 * assign23900_e22832) + (locals.var_fn277_calc_iq__qref0 * ((assign23900_e22830 * locals.var_fn277_calc_iq__etad0_dn4) / assign23900_e22831))), (locals.var_fn277_calc_iq__qref0 * ((assign23900_e22830 * locals.var_fn277_calc_iq__etad0_dn7) / assign23900_e22831)), (locals.var_fn277_calc_iq__qref0 * ((assign23900_e22830 * locals.var_fn277_calc_iq__etad0_dn12) / assign23900_e22831)), (locals.var_fn277_calc_iq__qref0 * ((assign23900_e22830 * locals.var_fn277_calc_iq__etad0_dn13) / assign23900_e22831)),)
    } else {
        (locals.var_fn277_calc_iq__qinvd0, locals.var_fn277_calc_iq__qinvd0_dn2, locals.var_fn277_calc_iq__qinvd0_dn4, locals.var_fn277_calc_iq__qinvd0_dn7, locals.var_fn277_calc_iq__qinvd0_dn12, locals.var_fn277_calc_iq__qinvd0_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvd0 = assign23900_e22835;
        locals.var_fn277_calc_iq__qinvd0_dn2 = assign23900_e22835_d_n2;
        locals.var_fn277_calc_iq__qinvd0_dn4 = assign23900_e22835_d_n4;
        locals.var_fn277_calc_iq__qinvd0_dn7 = assign23900_e22835_d_n7;
        locals.var_fn277_calc_iq__qinvd0_dn12 = assign23900_e22835_d_n12;
        locals.var_fn277_calc_iq__qinvd0_dn13 = assign23900_e22835_d_n13;
        locals.var_fn277_calc_iq__qinvd0_rv = 0.0;

        let (assign23910_e22843, assign23910_e22843_d_n2, assign23910_e22843_d_n4, assign23910_e22843_d_n7, assign23910_e22843_d_n12, assign23910_e22843_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23910_e22839: f64 = (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvs0);
        let assign23910_e22841: f64 = (assign23910_e22839 + 1e-38);
        (assign23910_e22841, ((locals.var_fn277_calc_iq__qinvs0_dn2 * locals.var_fn277_calc_iq__qinvs0) + (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvs0_dn2)), ((locals.var_fn277_calc_iq__qinvs0_dn4 * locals.var_fn277_calc_iq__qinvs0) + (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvs0_dn4)), ((locals.var_fn277_calc_iq__qinvs0_dn7 * locals.var_fn277_calc_iq__qinvs0) + (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvs0_dn7)), ((locals.var_fn277_calc_iq__qinvs0_dn12 * locals.var_fn277_calc_iq__qinvs0) + (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvs0_dn12)), ((locals.var_fn277_calc_iq__qinvs0_dn13 * locals.var_fn277_calc_iq__qinvs0) + (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvs0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qs2, locals.var_fn277_calc_iq__qs2_dn2, locals.var_fn277_calc_iq__qs2_dn4, locals.var_fn277_calc_iq__qs2_dn7, locals.var_fn277_calc_iq__qs2_dn12, locals.var_fn277_calc_iq__qs2_dn13,)
    }
};
        locals.var_fn277_calc_iq__qs2 = assign23910_e22843;
        locals.var_fn277_calc_iq__qs2_dn2 = assign23910_e22843_d_n2;
        locals.var_fn277_calc_iq__qs2_dn4 = assign23910_e22843_d_n4;
        locals.var_fn277_calc_iq__qs2_dn7 = assign23910_e22843_d_n7;
        locals.var_fn277_calc_iq__qs2_dn12 = assign23910_e22843_d_n12;
        locals.var_fn277_calc_iq__qs2_dn13 = assign23910_e22843_d_n13;
        locals.var_fn277_calc_iq__qs2_rv = 0.0;

        let (assign23920_e22851, assign23920_e22851_d_n2, assign23920_e22851_d_n4, assign23920_e22851_d_n7, assign23920_e22851_d_n12, assign23920_e22851_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23920_e22847: f64 = (locals.var_fn277_calc_iq__qs2 * locals.var_fn277_calc_iq__qinvs0);
        let assign23920_e22849: f64 = (assign23920_e22847 + 1e-57);
        (assign23920_e22849, ((locals.var_fn277_calc_iq__qs2_dn2 * locals.var_fn277_calc_iq__qinvs0) + (locals.var_fn277_calc_iq__qs2 * locals.var_fn277_calc_iq__qinvs0_dn2)), ((locals.var_fn277_calc_iq__qs2_dn4 * locals.var_fn277_calc_iq__qinvs0) + (locals.var_fn277_calc_iq__qs2 * locals.var_fn277_calc_iq__qinvs0_dn4)), ((locals.var_fn277_calc_iq__qs2_dn7 * locals.var_fn277_calc_iq__qinvs0) + (locals.var_fn277_calc_iq__qs2 * locals.var_fn277_calc_iq__qinvs0_dn7)), ((locals.var_fn277_calc_iq__qs2_dn12 * locals.var_fn277_calc_iq__qinvs0) + (locals.var_fn277_calc_iq__qs2 * locals.var_fn277_calc_iq__qinvs0_dn12)), ((locals.var_fn277_calc_iq__qs2_dn13 * locals.var_fn277_calc_iq__qinvs0) + (locals.var_fn277_calc_iq__qs2 * locals.var_fn277_calc_iq__qinvs0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qs3, locals.var_fn277_calc_iq__qs3_dn2, locals.var_fn277_calc_iq__qs3_dn4, locals.var_fn277_calc_iq__qs3_dn7, locals.var_fn277_calc_iq__qs3_dn12, locals.var_fn277_calc_iq__qs3_dn13,)
    }
};
        locals.var_fn277_calc_iq__qs3 = assign23920_e22851;
        locals.var_fn277_calc_iq__qs3_dn2 = assign23920_e22851_d_n2;
        locals.var_fn277_calc_iq__qs3_dn4 = assign23920_e22851_d_n4;
        locals.var_fn277_calc_iq__qs3_dn7 = assign23920_e22851_d_n7;
        locals.var_fn277_calc_iq__qs3_dn12 = assign23920_e22851_d_n12;
        locals.var_fn277_calc_iq__qs3_dn13 = assign23920_e22851_d_n13;
        locals.var_fn277_calc_iq__qs3_rv = 0.0;

        let (assign23930_e22859, assign23930_e22859_d_n2, assign23930_e22859_d_n4, assign23930_e22859_d_n7, assign23930_e22859_d_n12, assign23930_e22859_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23930_e22855: f64 = (locals.var_fn277_calc_iq__qinvd0 * locals.var_fn277_calc_iq__qinvd0);
        let assign23930_e22857: f64 = (assign23930_e22855 + 1e-38);
        (assign23930_e22857, ((locals.var_fn277_calc_iq__qinvd0_dn2 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qinvd0 * locals.var_fn277_calc_iq__qinvd0_dn2)), ((locals.var_fn277_calc_iq__qinvd0_dn4 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qinvd0 * locals.var_fn277_calc_iq__qinvd0_dn4)), ((locals.var_fn277_calc_iq__qinvd0_dn7 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qinvd0 * locals.var_fn277_calc_iq__qinvd0_dn7)), ((locals.var_fn277_calc_iq__qinvd0_dn12 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qinvd0 * locals.var_fn277_calc_iq__qinvd0_dn12)), ((locals.var_fn277_calc_iq__qinvd0_dn13 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qinvd0 * locals.var_fn277_calc_iq__qinvd0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qd2, locals.var_fn277_calc_iq__qd2_dn2, locals.var_fn277_calc_iq__qd2_dn4, locals.var_fn277_calc_iq__qd2_dn7, locals.var_fn277_calc_iq__qd2_dn12, locals.var_fn277_calc_iq__qd2_dn13,)
    }
};
        locals.var_fn277_calc_iq__qd2 = assign23930_e22859;
        locals.var_fn277_calc_iq__qd2_dn2 = assign23930_e22859_d_n2;
        locals.var_fn277_calc_iq__qd2_dn4 = assign23930_e22859_d_n4;
        locals.var_fn277_calc_iq__qd2_dn7 = assign23930_e22859_d_n7;
        locals.var_fn277_calc_iq__qd2_dn12 = assign23930_e22859_d_n12;
        locals.var_fn277_calc_iq__qd2_dn13 = assign23930_e22859_d_n13;
        locals.var_fn277_calc_iq__qd2_rv = 0.0;

        let (assign23940_e22867, assign23940_e22867_d_n2, assign23940_e22867_d_n4, assign23940_e22867_d_n7, assign23940_e22867_d_n12, assign23940_e22867_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23940_e22863: f64 = (locals.var_fn277_calc_iq__qd2 * locals.var_fn277_calc_iq__qinvd0);
        let assign23940_e22865: f64 = (assign23940_e22863 + 1e-57);
        (assign23940_e22865, ((locals.var_fn277_calc_iq__qd2_dn2 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qd2 * locals.var_fn277_calc_iq__qinvd0_dn2)), ((locals.var_fn277_calc_iq__qd2_dn4 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qd2 * locals.var_fn277_calc_iq__qinvd0_dn4)), ((locals.var_fn277_calc_iq__qd2_dn7 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qd2 * locals.var_fn277_calc_iq__qinvd0_dn7)), ((locals.var_fn277_calc_iq__qd2_dn12 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qd2 * locals.var_fn277_calc_iq__qinvd0_dn12)), ((locals.var_fn277_calc_iq__qd2_dn13 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qd2 * locals.var_fn277_calc_iq__qinvd0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qd3, locals.var_fn277_calc_iq__qd3_dn2, locals.var_fn277_calc_iq__qd3_dn4, locals.var_fn277_calc_iq__qd3_dn7, locals.var_fn277_calc_iq__qd3_dn12, locals.var_fn277_calc_iq__qd3_dn13,)
    }
};
        locals.var_fn277_calc_iq__qd3 = assign23940_e22867;
        locals.var_fn277_calc_iq__qd3_dn2 = assign23940_e22867_d_n2;
        locals.var_fn277_calc_iq__qd3_dn4 = assign23940_e22867_d_n4;
        locals.var_fn277_calc_iq__qd3_dn7 = assign23940_e22867_d_n7;
        locals.var_fn277_calc_iq__qd3_dn12 = assign23940_e22867_d_n12;
        locals.var_fn277_calc_iq__qd3_dn13 = assign23940_e22867_d_n13;
        locals.var_fn277_calc_iq__qd3_rv = 0.0;

        let (assign23950_e22875, assign23950_e22875_d_n2, assign23950_e22875_d_n4, assign23950_e22875_d_n7, assign23950_e22875_d_n12, assign23950_e22875_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23950_e22871: f64 = (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvd0);
        let assign23950_e22873: f64 = (assign23950_e22871 + 1e-38);
        (assign23950_e22873, ((locals.var_fn277_calc_iq__qinvs0_dn2 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvd0_dn2)), ((locals.var_fn277_calc_iq__qinvs0_dn4 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvd0_dn4)), ((locals.var_fn277_calc_iq__qinvs0_dn7 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvd0_dn7)), ((locals.var_fn277_calc_iq__qinvs0_dn12 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvd0_dn12)), ((locals.var_fn277_calc_iq__qinvs0_dn13 * locals.var_fn277_calc_iq__qinvd0) + (locals.var_fn277_calc_iq__qinvs0 * locals.var_fn277_calc_iq__qinvd0_dn13)),)
    } else {
        (locals.var_fn277_calc_iq__qsqd, locals.var_fn277_calc_iq__qsqd_dn2, locals.var_fn277_calc_iq__qsqd_dn4, locals.var_fn277_calc_iq__qsqd_dn7, locals.var_fn277_calc_iq__qsqd_dn12, locals.var_fn277_calc_iq__qsqd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qsqd = assign23950_e22875;
        locals.var_fn277_calc_iq__qsqd_dn2 = assign23950_e22875_d_n2;
        locals.var_fn277_calc_iq__qsqd_dn4 = assign23950_e22875_d_n4;
        locals.var_fn277_calc_iq__qsqd_dn7 = assign23950_e22875_d_n7;
        locals.var_fn277_calc_iq__qsqd_dn12 = assign23950_e22875_d_n12;
        locals.var_fn277_calc_iq__qsqd_dn13 = assign23950_e22875_d_n13;
        locals.var_fn277_calc_iq__qsqd_rv = 0.0;

        let (assign23960_e22893, assign23960_e22893_d_n2, assign23960_e22893_d_n4, assign23960_e22893_d_n7, assign23960_e22893_d_n12, assign23960_e22893_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23960_e22879: f64 = (2.0 / 3.0);
        let assign23960_e22882: f64 = (locals.var_fn277_calc_iq__qs2 + locals.var_fn277_calc_iq__qd2);
        let assign23960_e22884: f64 = (assign23960_e22882 + locals.var_fn277_calc_iq__qsqd);
        let assign23960_e22885: f64 = (assign23960_e22879 * assign23960_e22884);
        let assign23960_e22888: f64 = (locals.var_fn277_calc_iq__qinvs0 + locals.var_fn277_calc_iq__qinvd0);
        let assign23960_e22890: f64 = (assign23960_e22888 + 2e-19);
        let assign23960_e22891: f64 = (assign23960_e22885 / assign23960_e22890);
        (assign23960_e22891, ((((assign23960_e22879 * ((locals.var_fn277_calc_iq__qs2_dn2 + locals.var_fn277_calc_iq__qd2_dn2) + locals.var_fn277_calc_iq__qsqd_dn2)) * assign23960_e22890) - (assign23960_e22885 * (locals.var_fn277_calc_iq__qinvs0_dn2 + locals.var_fn277_calc_iq__qinvd0_dn2))) / (assign23960_e22890 * assign23960_e22890)), ((((assign23960_e22879 * ((locals.var_fn277_calc_iq__qs2_dn4 + locals.var_fn277_calc_iq__qd2_dn4) + locals.var_fn277_calc_iq__qsqd_dn4)) * assign23960_e22890) - (assign23960_e22885 * (locals.var_fn277_calc_iq__qinvs0_dn4 + locals.var_fn277_calc_iq__qinvd0_dn4))) / (assign23960_e22890 * assign23960_e22890)), ((((assign23960_e22879 * ((locals.var_fn277_calc_iq__qs2_dn7 + locals.var_fn277_calc_iq__qd2_dn7) + locals.var_fn277_calc_iq__qsqd_dn7)) * assign23960_e22890) - (assign23960_e22885 * (locals.var_fn277_calc_iq__qinvs0_dn7 + locals.var_fn277_calc_iq__qinvd0_dn7))) / (assign23960_e22890 * assign23960_e22890)), ((((assign23960_e22879 * ((locals.var_fn277_calc_iq__qs2_dn12 + locals.var_fn277_calc_iq__qd2_dn12) + locals.var_fn277_calc_iq__qsqd_dn12)) * assign23960_e22890) - (assign23960_e22885 * (locals.var_fn277_calc_iq__qinvs0_dn12 + locals.var_fn277_calc_iq__qinvd0_dn12))) / (assign23960_e22890 * assign23960_e22890)), ((((assign23960_e22879 * ((locals.var_fn277_calc_iq__qs2_dn13 + locals.var_fn277_calc_iq__qd2_dn13) + locals.var_fn277_calc_iq__qsqd_dn13)) * assign23960_e22890) - (assign23960_e22885 * (locals.var_fn277_calc_iq__qinvs0_dn13 + locals.var_fn277_calc_iq__qinvd0_dn13))) / (assign23960_e22890 * assign23960_e22890)),)
    } else {
        (locals.var_fn277_calc_iq__qinvdd, locals.var_fn277_calc_iq__qinvdd_dn2, locals.var_fn277_calc_iq__qinvdd_dn4, locals.var_fn277_calc_iq__qinvdd_dn7, locals.var_fn277_calc_iq__qinvdd_dn12, locals.var_fn277_calc_iq__qinvdd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qinvdd = assign23960_e22893;
        locals.var_fn277_calc_iq__qinvdd_dn2 = assign23960_e22893_d_n2;
        locals.var_fn277_calc_iq__qinvdd_dn4 = assign23960_e22893_d_n4;
        locals.var_fn277_calc_iq__qinvdd_dn7 = assign23960_e22893_d_n7;
        locals.var_fn277_calc_iq__qinvdd_dn12 = assign23960_e22893_d_n12;
        locals.var_fn277_calc_iq__qinvdd_dn13 = assign23960_e22893_d_n13;
        locals.var_fn277_calc_iq__qinvdd_rv = 0.0;

        let (assign23970_e22927, assign23970_e22927_d_n2, assign23970_e22927_d_n4, assign23970_e22927_d_n7, assign23970_e22927_d_n12, assign23970_e22927_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23970_e22898: f64 = (2.0 * locals.var_fn277_calc_iq__qs3);
        let assign23970_e22901: f64 = (3.0 * locals.var_fn277_calc_iq__qd3);
        let assign23970_e22902: f64 = (assign23970_e22898 + assign23970_e22901);
        let assign23970_e22905: f64 = (4.0 * locals.var_fn277_calc_iq__qs2);
        let assign23970_e22907: f64 = (assign23970_e22905 * locals.var_fn277_calc_iq__qinvd0);
        let assign23970_e22908: f64 = (assign23970_e22902 + assign23970_e22907);
        let assign23970_e22911: f64 = (6.0 * locals.var_fn277_calc_iq__qd2);
        let assign23970_e22913: f64 = (assign23970_e22911 * locals.var_fn277_calc_iq__qinvs0);
        let assign23970_e22914: f64 = (assign23970_e22908 + assign23970_e22913);
        let assign23970_e22915: f64 = (2.0 * assign23970_e22914);
        let assign23970_e22919: f64 = (locals.var_fn277_calc_iq__qs2 + locals.var_fn277_calc_iq__qd2);
        let assign23970_e22922: f64 = (2.0 * locals.var_fn277_calc_iq__qsqd);
        let assign23970_e22923: f64 = (assign23970_e22919 + assign23970_e22922);
        let assign23970_e22924: f64 = (15.0 * assign23970_e22923);
        let assign23970_e22925: f64 = (assign23970_e22915 / assign23970_e22924);
        (assign23970_e22925, ((((2.0 * ((((2.0 * locals.var_fn277_calc_iq__qs3_dn2) + (3.0 * locals.var_fn277_calc_iq__qd3_dn2)) + (((4.0 * locals.var_fn277_calc_iq__qs2_dn2) * locals.var_fn277_calc_iq__qinvd0) + (assign23970_e22905 * locals.var_fn277_calc_iq__qinvd0_dn2))) + (((6.0 * locals.var_fn277_calc_iq__qd2_dn2) * locals.var_fn277_calc_iq__qinvs0) + (assign23970_e22911 * locals.var_fn277_calc_iq__qinvs0_dn2)))) * assign23970_e22924) - (assign23970_e22915 * (15.0 * ((locals.var_fn277_calc_iq__qs2_dn2 + locals.var_fn277_calc_iq__qd2_dn2) + (2.0 * locals.var_fn277_calc_iq__qsqd_dn2))))) / (assign23970_e22924 * assign23970_e22924)), ((((2.0 * ((((2.0 * locals.var_fn277_calc_iq__qs3_dn4) + (3.0 * locals.var_fn277_calc_iq__qd3_dn4)) + (((4.0 * locals.var_fn277_calc_iq__qs2_dn4) * locals.var_fn277_calc_iq__qinvd0) + (assign23970_e22905 * locals.var_fn277_calc_iq__qinvd0_dn4))) + (((6.0 * locals.var_fn277_calc_iq__qd2_dn4) * locals.var_fn277_calc_iq__qinvs0) + (assign23970_e22911 * locals.var_fn277_calc_iq__qinvs0_dn4)))) * assign23970_e22924) - (assign23970_e22915 * (15.0 * ((locals.var_fn277_calc_iq__qs2_dn4 + locals.var_fn277_calc_iq__qd2_dn4) + (2.0 * locals.var_fn277_calc_iq__qsqd_dn4))))) / (assign23970_e22924 * assign23970_e22924)), ((((2.0 * ((((2.0 * locals.var_fn277_calc_iq__qs3_dn7) + (3.0 * locals.var_fn277_calc_iq__qd3_dn7)) + (((4.0 * locals.var_fn277_calc_iq__qs2_dn7) * locals.var_fn277_calc_iq__qinvd0) + (assign23970_e22905 * locals.var_fn277_calc_iq__qinvd0_dn7))) + (((6.0 * locals.var_fn277_calc_iq__qd2_dn7) * locals.var_fn277_calc_iq__qinvs0) + (assign23970_e22911 * locals.var_fn277_calc_iq__qinvs0_dn7)))) * assign23970_e22924) - (assign23970_e22915 * (15.0 * ((locals.var_fn277_calc_iq__qs2_dn7 + locals.var_fn277_calc_iq__qd2_dn7) + (2.0 * locals.var_fn277_calc_iq__qsqd_dn7))))) / (assign23970_e22924 * assign23970_e22924)), ((((2.0 * ((((2.0 * locals.var_fn277_calc_iq__qs3_dn12) + (3.0 * locals.var_fn277_calc_iq__qd3_dn12)) + (((4.0 * locals.var_fn277_calc_iq__qs2_dn12) * locals.var_fn277_calc_iq__qinvd0) + (assign23970_e22905 * locals.var_fn277_calc_iq__qinvd0_dn12))) + (((6.0 * locals.var_fn277_calc_iq__qd2_dn12) * locals.var_fn277_calc_iq__qinvs0) + (assign23970_e22911 * locals.var_fn277_calc_iq__qinvs0_dn12)))) * assign23970_e22924) - (assign23970_e22915 * (15.0 * ((locals.var_fn277_calc_iq__qs2_dn12 + locals.var_fn277_calc_iq__qd2_dn12) + (2.0 * locals.var_fn277_calc_iq__qsqd_dn12))))) / (assign23970_e22924 * assign23970_e22924)), ((((2.0 * ((((2.0 * locals.var_fn277_calc_iq__qs3_dn13) + (3.0 * locals.var_fn277_calc_iq__qd3_dn13)) + (((4.0 * locals.var_fn277_calc_iq__qs2_dn13) * locals.var_fn277_calc_iq__qinvd0) + (assign23970_e22905 * locals.var_fn277_calc_iq__qinvd0_dn13))) + (((6.0 * locals.var_fn277_calc_iq__qd2_dn13) * locals.var_fn277_calc_iq__qinvs0) + (assign23970_e22911 * locals.var_fn277_calc_iq__qinvs0_dn13)))) * assign23970_e22924) - (assign23970_e22915 * (15.0 * ((locals.var_fn277_calc_iq__qs2_dn13 + locals.var_fn277_calc_iq__qd2_dn13) + (2.0 * locals.var_fn277_calc_iq__qsqd_dn13))))) / (assign23970_e22924 * assign23970_e22924)),)
    } else {
        (locals.var_fn277_calc_iq__qd1, locals.var_fn277_calc_iq__qd1_dn2, locals.var_fn277_calc_iq__qd1_dn4, locals.var_fn277_calc_iq__qd1_dn7, locals.var_fn277_calc_iq__qd1_dn12, locals.var_fn277_calc_iq__qd1_dn13,)
    }
};
        locals.var_fn277_calc_iq__qd1 = assign23970_e22927;
        locals.var_fn277_calc_iq__qd1_dn2 = assign23970_e22927_d_n2;
        locals.var_fn277_calc_iq__qd1_dn4 = assign23970_e22927_d_n4;
        locals.var_fn277_calc_iq__qd1_dn7 = assign23970_e22927_d_n7;
        locals.var_fn277_calc_iq__qd1_dn12 = assign23970_e22927_d_n12;
        locals.var_fn277_calc_iq__qd1_dn13 = assign23970_e22927_d_n13;
        locals.var_fn277_calc_iq__qd1_rv = 0.0;

        let (assign23980_e22933, assign23980_e22933_d_n2, assign23980_e22933_d_n4, assign23980_e22933_d_n7, assign23980_e22933_d_n12, assign23980_e22933_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign23980_e22931: f64 = (locals.var_fn277_calc_iq__qinvdd - locals.var_fn277_calc_iq__qd1);
        (assign23980_e22931, (locals.var_fn277_calc_iq__qinvdd_dn2 - locals.var_fn277_calc_iq__qd1_dn2), (locals.var_fn277_calc_iq__qinvdd_dn4 - locals.var_fn277_calc_iq__qd1_dn4), (locals.var_fn277_calc_iq__qinvdd_dn7 - locals.var_fn277_calc_iq__qd1_dn7), (locals.var_fn277_calc_iq__qinvdd_dn12 - locals.var_fn277_calc_iq__qd1_dn12), (locals.var_fn277_calc_iq__qinvdd_dn13 - locals.var_fn277_calc_iq__qd1_dn13),)
    } else {
        (locals.var_fn277_calc_iq__qs, locals.var_fn277_calc_iq__qs_dn2, locals.var_fn277_calc_iq__qs_dn4, locals.var_fn277_calc_iq__qs_dn7, locals.var_fn277_calc_iq__qs_dn12, locals.var_fn277_calc_iq__qs_dn13,)
    }
};
        locals.var_fn277_calc_iq__qs = assign23980_e22933;
        locals.var_fn277_calc_iq__qs_dn2 = assign23980_e22933_d_n2;
        locals.var_fn277_calc_iq__qs_dn4 = assign23980_e22933_d_n4;
        locals.var_fn277_calc_iq__qs_dn7 = assign23980_e22933_d_n7;
        locals.var_fn277_calc_iq__qs_dn12 = assign23980_e22933_d_n12;
        locals.var_fn277_calc_iq__qs_dn13 = assign23980_e22933_d_n13;
        locals.var_fn277_calc_iq__qs_rv = 0.0;

        let (assign23990_e22937, assign23990_e22937_d_n2, assign23990_e22937_d_n4, assign23990_e22937_d_n7, assign23990_e22937_d_n12, assign23990_e22937_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_fn277_calc_iq__qd1, locals.var_fn277_calc_iq__qd1_dn2, locals.var_fn277_calc_iq__qd1_dn4, locals.var_fn277_calc_iq__qd1_dn7, locals.var_fn277_calc_iq__qd1_dn12, locals.var_fn277_calc_iq__qd1_dn13,)
    } else {
        (locals.var_fn277_calc_iq__qd, locals.var_fn277_calc_iq__qd_dn2, locals.var_fn277_calc_iq__qd_dn4, locals.var_fn277_calc_iq__qd_dn7, locals.var_fn277_calc_iq__qd_dn12, locals.var_fn277_calc_iq__qd_dn13,)
    }
};
        locals.var_fn277_calc_iq__qd = assign23990_e22937;
        locals.var_fn277_calc_iq__qd_dn2 = assign23990_e22937_d_n2;
        locals.var_fn277_calc_iq__qd_dn4 = assign23990_e22937_d_n4;
        locals.var_fn277_calc_iq__qd_dn7 = assign23990_e22937_d_n7;
        locals.var_fn277_calc_iq__qd_dn12 = assign23990_e22937_d_n12;
        locals.var_fn277_calc_iq__qd_dn13 = assign23990_e22937_d_n13;
        locals.var_fn277_calc_iq__qd_rv = 0.0;

        let (assign24000_e22951, assign24000_e22951_d_n2, assign24000_e22951_d_n4, assign24000_e22951_d_n7, assign24000_e22951_d_n12, assign24000_e22951_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign24000_e22941: f64 = (locals.var_fn277_calc_iq__w * locals.var_fn277_calc_iq__ngf);
        let assign24000_e22943: f64 = (assign24000_e22941 * locals.var_fn277_calc_iq__lin);
        let assign24000_e22945: f64 = (assign24000_e22943 * locals.var_fn277_calc_iq__type);
        let assign24000_e22947: f64 = (assign24000_e22945 * locals.var_fn277_calc_iq__qs);
        let assign24000_e22949: f64 = (assign24000_e22947 * locals.var_fn277_calc_iq__trapfracdl);
        (assign24000_e22949, ((assign24000_e22945 * locals.var_fn277_calc_iq__qs_dn2) * locals.var_fn277_calc_iq__trapfracdl), ((assign24000_e22945 * locals.var_fn277_calc_iq__qs_dn4) * locals.var_fn277_calc_iq__trapfracdl), ((assign24000_e22945 * locals.var_fn277_calc_iq__qs_dn7) * locals.var_fn277_calc_iq__trapfracdl), ((assign24000_e22945 * locals.var_fn277_calc_iq__qs_dn12) * locals.var_fn277_calc_iq__trapfracdl), ((assign24000_e22945 * locals.var_fn277_calc_iq__qs_dn13) * locals.var_fn277_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn277_calc_iq__qgsout, locals.var_fn277_calc_iq__qgsout_dn2, locals.var_fn277_calc_iq__qgsout_dn4, locals.var_fn277_calc_iq__qgsout_dn7, locals.var_fn277_calc_iq__qgsout_dn12, locals.var_fn277_calc_iq__qgsout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qgsout = assign24000_e22951;
        locals.var_fn277_calc_iq__qgsout_dn2 = assign24000_e22951_d_n2;
        locals.var_fn277_calc_iq__qgsout_dn4 = assign24000_e22951_d_n4;
        locals.var_fn277_calc_iq__qgsout_dn7 = assign24000_e22951_d_n7;
        locals.var_fn277_calc_iq__qgsout_dn12 = assign24000_e22951_d_n12;
        locals.var_fn277_calc_iq__qgsout_dn13 = assign24000_e22951_d_n13;
        locals.var_fn277_calc_iq__qgsout_rv = 0.0;

        let (assign24010_e22965, assign24010_e22965_d_n2, assign24010_e22965_d_n4, assign24010_e22965_d_n7, assign24010_e22965_d_n12, assign24010_e22965_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        let assign24010_e22955: f64 = (locals.var_fn277_calc_iq__w * locals.var_fn277_calc_iq__ngf);
        let assign24010_e22957: f64 = (assign24010_e22955 * locals.var_fn277_calc_iq__lin);
        let assign24010_e22959: f64 = (assign24010_e22957 * locals.var_fn277_calc_iq__type);
        let assign24010_e22961: f64 = (assign24010_e22959 * locals.var_fn277_calc_iq__qd);
        let assign24010_e22963: f64 = (assign24010_e22961 * locals.var_fn277_calc_iq__trapfracdl);
        (assign24010_e22963, ((assign24010_e22959 * locals.var_fn277_calc_iq__qd_dn2) * locals.var_fn277_calc_iq__trapfracdl), ((assign24010_e22959 * locals.var_fn277_calc_iq__qd_dn4) * locals.var_fn277_calc_iq__trapfracdl), ((assign24010_e22959 * locals.var_fn277_calc_iq__qd_dn7) * locals.var_fn277_calc_iq__trapfracdl), ((assign24010_e22959 * locals.var_fn277_calc_iq__qd_dn12) * locals.var_fn277_calc_iq__trapfracdl), ((assign24010_e22959 * locals.var_fn277_calc_iq__qd_dn13) * locals.var_fn277_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn277_calc_iq__qgdout, locals.var_fn277_calc_iq__qgdout_dn2, locals.var_fn277_calc_iq__qgdout_dn4, locals.var_fn277_calc_iq__qgdout_dn7, locals.var_fn277_calc_iq__qgdout_dn12, locals.var_fn277_calc_iq__qgdout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qgdout = assign24010_e22965;
        locals.var_fn277_calc_iq__qgdout_dn2 = assign24010_e22965_d_n2;
        locals.var_fn277_calc_iq__qgdout_dn4 = assign24010_e22965_d_n4;
        locals.var_fn277_calc_iq__qgdout_dn7 = assign24010_e22965_d_n7;
        locals.var_fn277_calc_iq__qgdout_dn12 = assign24010_e22965_d_n12;
        locals.var_fn277_calc_iq__qgdout_dn13 = assign24010_e22965_d_n13;
        locals.var_fn277_calc_iq__qgdout_rv = 0.0;

        let assign24020_e22968: f64 = if locals.var_fn277_calc_iq__qcbflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard303 = assign24020_e22968;
        locals.var_guard303_rv = 0.0;

        let (assign24030_e22984, assign24030_e22984_d_n2, assign24030_e22984_d_n4, assign24030_e22984_d_n7, assign24030_e22984_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard303 != 0.0)) {
        let assign24030_e22976: f64 = (p.p51 * 0.5);
        let assign24030_e22978: f64 = (assign24030_e22976 * locals.var_fn277_calc_iq__alpha_phit);
        let assign24030_e22979: f64 = (locals.var_fn277_calc_iq__vtof - assign24030_e22978);
        let assign24030_e22980: f64 = (locals.var_fn277_calc_iq__vcin - assign24030_e22979);
        let assign24030_e22982: f64 = (assign24030_e22980 / locals.var_fn277_calc_iq__two_n_phit0);
        (assign24030_e22982, (locals.var_fn277_calc_iq__vcin_dn2 / locals.var_fn277_calc_iq__two_n_phit0), ((((-(locals.var_fn277_calc_iq__vtof_dn4 - (assign24030_e22976 * locals.var_fn277_calc_iq__alpha_phit_dn4))) * locals.var_fn277_calc_iq__two_n_phit0) - (assign24030_e22980 * locals.var_fn277_calc_iq__two_n_phit0_dn4)) / (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__two_n_phit0)), (locals.var_fn277_calc_iq__vcin_dn7 / locals.var_fn277_calc_iq__two_n_phit0), (locals.var_fn277_calc_iq__vcin_dn13 / locals.var_fn277_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn277_calc_iq__etac, locals.var_fn277_calc_iq__etac_dn2, locals.var_fn277_calc_iq__etac_dn4, locals.var_fn277_calc_iq__etac_dn7, locals.var_fn277_calc_iq__etac_dn13,)
    }
};
        locals.var_fn277_calc_iq__etac = assign24030_e22984;
        locals.var_fn277_calc_iq__etac_dn2 = assign24030_e22984_d_n2;
        locals.var_fn277_calc_iq__etac_dn4 = assign24030_e22984_d_n4;
        locals.var_fn277_calc_iq__etac_dn7 = assign24030_e22984_d_n7;
        locals.var_fn277_calc_iq__etac_dn13 = assign24030_e22984_d_n13;
        locals.var_fn277_calc_iq__etac_rv = 0.0;

        let assign24040_e22987: f64 = if locals.var_fn277_calc_iq__etac > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard304 = assign24040_e22987;
        locals.var_guard304_rv = 0.0;

        let (assign24050_e22995, assign24050_e22995_d_n2, assign24050_e22995_d_n3, assign24050_e22995_d_n4, assign24050_e22995_d_n7, assign24050_e22995_d_n12, assign24050_e22995_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard303 != 0.0)) && (locals.var_guard304 != 0.0)) {
        (locals.var_fn277_calc_iq__etac, locals.var_fn277_calc_iq__etac_dn2, 0.0, locals.var_fn277_calc_iq__etac_dn4, locals.var_fn277_calc_iq__etac_dn7, 0.0, locals.var_fn277_calc_iq__etac_dn13,)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign24050_e22995;
        locals.var_fn277_calc_iq__exparg_dn2 = assign24050_e22995_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign24050_e22995_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign24050_e22995_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign24050_e22995_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign24050_e22995_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign24050_e22995_d_n13;
        locals.var_fn277_calc_iq__exparg_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_66(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign24060_e22998: f64 = (-50.0);
        let assign24060_e22999: f64 = if locals.var_fn277_calc_iq__etac < assign24060_e22998 { 1.0 } else { 0.0 };
        locals.var_guard305 = assign24060_e22999;
        locals.var_guard305_rv = 0.0;

        let (assign24070_e23011, assign24070_e23011_d_n2, assign24070_e23011_d_n3, assign24070_e23011_d_n4, assign24070_e23011_d_n7, assign24070_e23011_d_n12, assign24070_e23011_d_n13,) = {
    if ((((locals.var_guard276 != 0.0) && (locals.var_guard303 != 0.0)) && (locals.var_guard304 == 0.0)) && (locals.var_guard305 != 0.0)) {
        let assign24070_e23009: f64 = (locals.var_fn277_calc_iq__etac).exp();
        (assign24070_e23009, (assign24070_e23009 * locals.var_fn277_calc_iq__etac_dn2), 0.0, (assign24070_e23009 * locals.var_fn277_calc_iq__etac_dn4), (assign24070_e23009 * locals.var_fn277_calc_iq__etac_dn7), 0.0, (assign24070_e23009 * locals.var_fn277_calc_iq__etac_dn13),)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign24070_e23011;
        locals.var_fn277_calc_iq__exparg_dn2 = assign24070_e23011_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign24070_e23011_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign24070_e23011_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign24070_e23011_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign24070_e23011_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign24070_e23011_d_n13;
        locals.var_fn277_calc_iq__exparg_rv = 0.0;

        let (assign24080_e23027, assign24080_e23027_d_n2, assign24080_e23027_d_n3, assign24080_e23027_d_n4, assign24080_e23027_d_n7, assign24080_e23027_d_n12, assign24080_e23027_d_n13,) = {
    if ((((locals.var_guard276 != 0.0) && (locals.var_guard303 != 0.0)) && (locals.var_guard304 == 0.0)) && (locals.var_guard305 == 0.0)) {
        let assign24080_e23023: f64 = (locals.var_fn277_calc_iq__etac).exp();
        let assign24080_e23024: f64 = (1.0 + assign24080_e23023);
        let assign24080_e23025: f64 = (assign24080_e23024).ln();
        (assign24080_e23025, ((assign24080_e23023 * locals.var_fn277_calc_iq__etac_dn2) / assign24080_e23024), 0.0, ((assign24080_e23023 * locals.var_fn277_calc_iq__etac_dn4) / assign24080_e23024), ((assign24080_e23023 * locals.var_fn277_calc_iq__etac_dn7) / assign24080_e23024), 0.0, ((assign24080_e23023 * locals.var_fn277_calc_iq__etac_dn13) / assign24080_e23024),)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign24080_e23027;
        locals.var_fn277_calc_iq__exparg_dn2 = assign24080_e23027_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign24080_e23027_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign24080_e23027_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign24080_e23027_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign24080_e23027_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign24080_e23027_d_n13;
        locals.var_fn277_calc_iq__exparg_rv = 0.0;

        let (assign24090_e23045, assign24090_e23045_d_n2, assign24090_e23045_d_n3, assign24090_e23045_d_n4, assign24090_e23045_d_n7, assign24090_e23045_d_n12, assign24090_e23045_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard303 != 0.0)) {
        let assign24090_e23033: f64 = (locals.var_fn277_calc_iq__w * locals.var_fn277_calc_iq__ngf);
        let assign24090_e23035: f64 = (assign24090_e23033 * locals.var_fn277_calc_iq__type);
        let assign24090_e23037: f64 = (assign24090_e23035 * locals.var_fn277_calc_iq__cc);
        let assign24090_e23039: f64 = (assign24090_e23037 * locals.var_fn277_calc_iq__two_n_phit0);
        let assign24090_e23041: f64 = (assign24090_e23039 * locals.var_fn277_calc_iq__exparg);
        let assign24090_e23043: f64 = (assign24090_e23041 * locals.var_fn277_calc_iq__trapfracdl);
        (assign24090_e23043, ((assign24090_e23039 * locals.var_fn277_calc_iq__exparg_dn2) * locals.var_fn277_calc_iq__trapfracdl), ((assign24090_e23039 * locals.var_fn277_calc_iq__exparg_dn3) * locals.var_fn277_calc_iq__trapfracdl), ((((((assign24090_e23035 * locals.var_fn277_calc_iq__cc_dn4) * locals.var_fn277_calc_iq__two_n_phit0) + (assign24090_e23037 * locals.var_fn277_calc_iq__two_n_phit0_dn4)) * locals.var_fn277_calc_iq__exparg) + (assign24090_e23039 * locals.var_fn277_calc_iq__exparg_dn4)) * locals.var_fn277_calc_iq__trapfracdl), ((assign24090_e23039 * locals.var_fn277_calc_iq__exparg_dn7) * locals.var_fn277_calc_iq__trapfracdl), ((assign24090_e23039 * locals.var_fn277_calc_iq__exparg_dn12) * locals.var_fn277_calc_iq__trapfracdl), ((assign24090_e23039 * locals.var_fn277_calc_iq__exparg_dn13) * locals.var_fn277_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn277_calc_iq__qcout, locals.var_fn277_calc_iq__qcout_dn2, locals.var_fn277_calc_iq__qcout_dn3, locals.var_fn277_calc_iq__qcout_dn4, locals.var_fn277_calc_iq__qcout_dn7, locals.var_fn277_calc_iq__qcout_dn12, locals.var_fn277_calc_iq__qcout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qcout = assign24090_e23045;
        locals.var_fn277_calc_iq__qcout_dn2 = assign24090_e23045_d_n2;
        locals.var_fn277_calc_iq__qcout_dn3 = assign24090_e23045_d_n3;
        locals.var_fn277_calc_iq__qcout_dn4 = assign24090_e23045_d_n4;
        locals.var_fn277_calc_iq__qcout_dn7 = assign24090_e23045_d_n7;
        locals.var_fn277_calc_iq__qcout_dn12 = assign24090_e23045_d_n12;
        locals.var_fn277_calc_iq__qcout_dn13 = assign24090_e23045_d_n13;
        locals.var_fn277_calc_iq__qcout_rv = 0.0;

        let (assign24100_e23061, assign24100_e23061_d_n3, assign24100_e23061_d_n4, assign24100_e23061_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard303 != 0.0)) {
        let assign24100_e23053: f64 = (p.p51 * 0.5);
        let assign24100_e23055: f64 = (assign24100_e23053 * locals.var_fn277_calc_iq__alpha_phit);
        let assign24100_e23056: f64 = (locals.var_fn277_calc_iq__vtof - assign24100_e23055);
        let assign24100_e23057: f64 = (locals.var_fn277_calc_iq__vbin - assign24100_e23056);
        let assign24100_e23059: f64 = (assign24100_e23057 / locals.var_fn277_calc_iq__two_n_phit0);
        (assign24100_e23059, (locals.var_fn277_calc_iq__vbin_dn3 / locals.var_fn277_calc_iq__two_n_phit0), ((((-(locals.var_fn277_calc_iq__vtof_dn4 - (assign24100_e23053 * locals.var_fn277_calc_iq__alpha_phit_dn4))) * locals.var_fn277_calc_iq__two_n_phit0) - (assign24100_e23057 * locals.var_fn277_calc_iq__two_n_phit0_dn4)) / (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__two_n_phit0)), (locals.var_fn277_calc_iq__vbin_dn13 / locals.var_fn277_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn277_calc_iq__etab, locals.var_fn277_calc_iq__etab_dn3, locals.var_fn277_calc_iq__etab_dn4, locals.var_fn277_calc_iq__etab_dn13,)
    }
};
        locals.var_fn277_calc_iq__etab = assign24100_e23061;
        locals.var_fn277_calc_iq__etab_dn3 = assign24100_e23061_d_n3;
        locals.var_fn277_calc_iq__etab_dn4 = assign24100_e23061_d_n4;
        locals.var_fn277_calc_iq__etab_dn13 = assign24100_e23061_d_n13;
        locals.var_fn277_calc_iq__etab_rv = 0.0;

        let assign24110_e23064: f64 = if locals.var_fn277_calc_iq__etab > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard306 = assign24110_e23064;
        locals.var_guard306_rv = 0.0;

        let (assign24120_e23072, assign24120_e23072_d_n2, assign24120_e23072_d_n3, assign24120_e23072_d_n4, assign24120_e23072_d_n7, assign24120_e23072_d_n12, assign24120_e23072_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard303 != 0.0)) && (locals.var_guard306 != 0.0)) {
        (locals.var_fn277_calc_iq__etab, 0.0, locals.var_fn277_calc_iq__etab_dn3, locals.var_fn277_calc_iq__etab_dn4, 0.0, 0.0, locals.var_fn277_calc_iq__etab_dn13,)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign24120_e23072;
        locals.var_fn277_calc_iq__exparg_dn2 = assign24120_e23072_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign24120_e23072_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign24120_e23072_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign24120_e23072_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign24120_e23072_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign24120_e23072_d_n13;
        locals.var_fn277_calc_iq__exparg_rv = 0.0;

        let assign24130_e23075: f64 = (-50.0);
        let assign24130_e23076: f64 = if locals.var_fn277_calc_iq__etab < assign24130_e23075 { 1.0 } else { 0.0 };
        locals.var_guard307 = assign24130_e23076;
        locals.var_guard307_rv = 0.0;

        let (assign24140_e23088, assign24140_e23088_d_n2, assign24140_e23088_d_n3, assign24140_e23088_d_n4, assign24140_e23088_d_n7, assign24140_e23088_d_n12, assign24140_e23088_d_n13,) = {
    if ((((locals.var_guard276 != 0.0) && (locals.var_guard303 != 0.0)) && (locals.var_guard306 == 0.0)) && (locals.var_guard307 != 0.0)) {
        let assign24140_e23086: f64 = (locals.var_fn277_calc_iq__etab).exp();
        (assign24140_e23086, 0.0, (assign24140_e23086 * locals.var_fn277_calc_iq__etab_dn3), (assign24140_e23086 * locals.var_fn277_calc_iq__etab_dn4), 0.0, 0.0, (assign24140_e23086 * locals.var_fn277_calc_iq__etab_dn13),)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign24140_e23088;
        locals.var_fn277_calc_iq__exparg_dn2 = assign24140_e23088_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign24140_e23088_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign24140_e23088_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign24140_e23088_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign24140_e23088_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign24140_e23088_d_n13;
        locals.var_fn277_calc_iq__exparg_rv = 0.0;

        let (assign24150_e23104, assign24150_e23104_d_n2, assign24150_e23104_d_n3, assign24150_e23104_d_n4, assign24150_e23104_d_n7, assign24150_e23104_d_n12, assign24150_e23104_d_n13,) = {
    if ((((locals.var_guard276 != 0.0) && (locals.var_guard303 != 0.0)) && (locals.var_guard306 == 0.0)) && (locals.var_guard307 == 0.0)) {
        let assign24150_e23100: f64 = (locals.var_fn277_calc_iq__etab).exp();
        let assign24150_e23101: f64 = (1.0 + assign24150_e23100);
        let assign24150_e23102: f64 = (assign24150_e23101).ln();
        (assign24150_e23102, 0.0, ((assign24150_e23100 * locals.var_fn277_calc_iq__etab_dn3) / assign24150_e23101), ((assign24150_e23100 * locals.var_fn277_calc_iq__etab_dn4) / assign24150_e23101), 0.0, 0.0, ((assign24150_e23100 * locals.var_fn277_calc_iq__etab_dn13) / assign24150_e23101),)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign24150_e23104;
        locals.var_fn277_calc_iq__exparg_dn2 = assign24150_e23104_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign24150_e23104_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign24150_e23104_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign24150_e23104_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign24150_e23104_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign24150_e23104_d_n13;
        locals.var_fn277_calc_iq__exparg_rv = 0.0;

        let (assign24160_e23122, assign24160_e23122_d_n2, assign24160_e23122_d_n3, assign24160_e23122_d_n4, assign24160_e23122_d_n7, assign24160_e23122_d_n12, assign24160_e23122_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard303 != 0.0)) {
        let assign24160_e23110: f64 = (locals.var_fn277_calc_iq__w * locals.var_fn277_calc_iq__ngf);
        let assign24160_e23112: f64 = (assign24160_e23110 * locals.var_fn277_calc_iq__type);
        let assign24160_e23114: f64 = (assign24160_e23112 * locals.var_fn277_calc_iq__cb);
        let assign24160_e23116: f64 = (assign24160_e23114 * locals.var_fn277_calc_iq__two_n_phit0);
        let assign24160_e23118: f64 = (assign24160_e23116 * locals.var_fn277_calc_iq__exparg);
        let assign24160_e23120: f64 = (assign24160_e23118 * locals.var_fn277_calc_iq__trapfracdl);
        (assign24160_e23120, ((assign24160_e23116 * locals.var_fn277_calc_iq__exparg_dn2) * locals.var_fn277_calc_iq__trapfracdl), ((assign24160_e23116 * locals.var_fn277_calc_iq__exparg_dn3) * locals.var_fn277_calc_iq__trapfracdl), ((((((assign24160_e23112 * locals.var_fn277_calc_iq__cb_dn4) * locals.var_fn277_calc_iq__two_n_phit0) + (assign24160_e23114 * locals.var_fn277_calc_iq__two_n_phit0_dn4)) * locals.var_fn277_calc_iq__exparg) + (assign24160_e23116 * locals.var_fn277_calc_iq__exparg_dn4)) * locals.var_fn277_calc_iq__trapfracdl), ((assign24160_e23116 * locals.var_fn277_calc_iq__exparg_dn7) * locals.var_fn277_calc_iq__trapfracdl), ((assign24160_e23116 * locals.var_fn277_calc_iq__exparg_dn12) * locals.var_fn277_calc_iq__trapfracdl), ((assign24160_e23116 * locals.var_fn277_calc_iq__exparg_dn13) * locals.var_fn277_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn277_calc_iq__qbout, locals.var_fn277_calc_iq__qbout_dn2, locals.var_fn277_calc_iq__qbout_dn3, locals.var_fn277_calc_iq__qbout_dn4, locals.var_fn277_calc_iq__qbout_dn7, locals.var_fn277_calc_iq__qbout_dn12, locals.var_fn277_calc_iq__qbout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qbout = assign24160_e23122;
        locals.var_fn277_calc_iq__qbout_dn2 = assign24160_e23122_d_n2;
        locals.var_fn277_calc_iq__qbout_dn3 = assign24160_e23122_d_n3;
        locals.var_fn277_calc_iq__qbout_dn4 = assign24160_e23122_d_n4;
        locals.var_fn277_calc_iq__qbout_dn7 = assign24160_e23122_d_n7;
        locals.var_fn277_calc_iq__qbout_dn12 = assign24160_e23122_d_n12;
        locals.var_fn277_calc_iq__qbout_dn13 = assign24160_e23122_d_n13;
        locals.var_fn277_calc_iq__qbout_rv = 0.0;

        let (assign24170_e23129, assign24170_e23129_d_n2, assign24170_e23129_d_n3, assign24170_e23129_d_n4, assign24170_e23129_d_n7, assign24170_e23129_d_n12, assign24170_e23129_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard303 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qcout, locals.var_fn277_calc_iq__qcout_dn2, locals.var_fn277_calc_iq__qcout_dn3, locals.var_fn277_calc_iq__qcout_dn4, locals.var_fn277_calc_iq__qcout_dn7, locals.var_fn277_calc_iq__qcout_dn12, locals.var_fn277_calc_iq__qcout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qcout = assign24170_e23129;
        locals.var_fn277_calc_iq__qcout_dn2 = assign24170_e23129_d_n2;
        locals.var_fn277_calc_iq__qcout_dn3 = assign24170_e23129_d_n3;
        locals.var_fn277_calc_iq__qcout_dn4 = assign24170_e23129_d_n4;
        locals.var_fn277_calc_iq__qcout_dn7 = assign24170_e23129_d_n7;
        locals.var_fn277_calc_iq__qcout_dn12 = assign24170_e23129_d_n12;
        locals.var_fn277_calc_iq__qcout_dn13 = assign24170_e23129_d_n13;
        locals.var_fn277_calc_iq__qcout_rv = 0.0;

        let (assign24180_e23136, assign24180_e23136_d_n2, assign24180_e23136_d_n3, assign24180_e23136_d_n4, assign24180_e23136_d_n7, assign24180_e23136_d_n12, assign24180_e23136_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard303 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qbout, locals.var_fn277_calc_iq__qbout_dn2, locals.var_fn277_calc_iq__qbout_dn3, locals.var_fn277_calc_iq__qbout_dn4, locals.var_fn277_calc_iq__qbout_dn7, locals.var_fn277_calc_iq__qbout_dn12, locals.var_fn277_calc_iq__qbout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qbout = assign24180_e23136;
        locals.var_fn277_calc_iq__qbout_dn2 = assign24180_e23136_d_n2;
        locals.var_fn277_calc_iq__qbout_dn3 = assign24180_e23136_d_n3;
        locals.var_fn277_calc_iq__qbout_dn4 = assign24180_e23136_d_n4;
        locals.var_fn277_calc_iq__qbout_dn7 = assign24180_e23136_d_n7;
        locals.var_fn277_calc_iq__qbout_dn12 = assign24180_e23136_d_n12;
        locals.var_fn277_calc_iq__qbout_dn13 = assign24180_e23136_d_n13;
        locals.var_fn277_calc_iq__qbout_rv = 0.0;

        let assign24190_e23139: f64 = if locals.var_fn277_calc_iq__qgsflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard308 = assign24190_e23139;
        locals.var_guard308_rv = 0.0;

        let (assign24200_e23155, assign24200_e23155_d_n2, assign24200_e23155_d_n4, assign24200_e23155_d_n7, assign24200_e23155_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard308 != 0.0)) {
        let assign24200_e23147: f64 = (p.p51 * 0.5);
        let assign24200_e23149: f64 = (assign24200_e23147 * locals.var_fn277_calc_iq__alpha_phit);
        let assign24200_e23150: f64 = (locals.var_fn277_calc_iq__vtof - assign24200_e23149);
        let assign24200_e23151: f64 = (locals.var_fn277_calc_iq__vgsin - assign24200_e23150);
        let assign24200_e23153: f64 = (assign24200_e23151 / locals.var_fn277_calc_iq__two_n_phit0);
        (assign24200_e23153, (locals.var_fn277_calc_iq__vgsin_dn2 / locals.var_fn277_calc_iq__two_n_phit0), ((((-(locals.var_fn277_calc_iq__vtof_dn4 - (assign24200_e23147 * locals.var_fn277_calc_iq__alpha_phit_dn4))) * locals.var_fn277_calc_iq__two_n_phit0) - (assign24200_e23151 * locals.var_fn277_calc_iq__two_n_phit0_dn4)) / (locals.var_fn277_calc_iq__two_n_phit0 * locals.var_fn277_calc_iq__two_n_phit0)), (locals.var_fn277_calc_iq__vgsin_dn7 / locals.var_fn277_calc_iq__two_n_phit0), (locals.var_fn277_calc_iq__vgsin_dn13 / locals.var_fn277_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn277_calc_iq__etags, locals.var_fn277_calc_iq__etags_dn2, locals.var_fn277_calc_iq__etags_dn4, locals.var_fn277_calc_iq__etags_dn7, locals.var_fn277_calc_iq__etags_dn13,)
    }
};
        locals.var_fn277_calc_iq__etags = assign24200_e23155;
        locals.var_fn277_calc_iq__etags_dn2 = assign24200_e23155_d_n2;
        locals.var_fn277_calc_iq__etags_dn4 = assign24200_e23155_d_n4;
        locals.var_fn277_calc_iq__etags_dn7 = assign24200_e23155_d_n7;
        locals.var_fn277_calc_iq__etags_dn13 = assign24200_e23155_d_n13;
        locals.var_fn277_calc_iq__etags_rv = 0.0;

        let assign24210_e23158: f64 = if locals.var_fn277_calc_iq__etags > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard309 = assign24210_e23158;
        locals.var_guard309_rv = 0.0;

        let (assign24220_e23166, assign24220_e23166_d_n2, assign24220_e23166_d_n3, assign24220_e23166_d_n4, assign24220_e23166_d_n7, assign24220_e23166_d_n12, assign24220_e23166_d_n13,) = {
    if (((locals.var_guard276 != 0.0) && (locals.var_guard308 != 0.0)) && (locals.var_guard309 != 0.0)) {
        (locals.var_fn277_calc_iq__etags, locals.var_fn277_calc_iq__etags_dn2, 0.0, locals.var_fn277_calc_iq__etags_dn4, locals.var_fn277_calc_iq__etags_dn7, 0.0, locals.var_fn277_calc_iq__etags_dn13,)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign24220_e23166;
        locals.var_fn277_calc_iq__exparg_dn2 = assign24220_e23166_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign24220_e23166_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign24220_e23166_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign24220_e23166_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign24220_e23166_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign24220_e23166_d_n13;
        locals.var_fn277_calc_iq__exparg_rv = 0.0;

        let assign24230_e23169: f64 = (-50.0);
        let assign24230_e23170: f64 = if locals.var_fn277_calc_iq__etags < assign24230_e23169 { 1.0 } else { 0.0 };
        locals.var_guard310 = assign24230_e23170;
        locals.var_guard310_rv = 0.0;

        let (assign24240_e23182, assign24240_e23182_d_n2, assign24240_e23182_d_n3, assign24240_e23182_d_n4, assign24240_e23182_d_n7, assign24240_e23182_d_n12, assign24240_e23182_d_n13,) = {
    if ((((locals.var_guard276 != 0.0) && (locals.var_guard308 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard310 != 0.0)) {
        let assign24240_e23180: f64 = (locals.var_fn277_calc_iq__etags).exp();
        (assign24240_e23180, (assign24240_e23180 * locals.var_fn277_calc_iq__etags_dn2), 0.0, (assign24240_e23180 * locals.var_fn277_calc_iq__etags_dn4), (assign24240_e23180 * locals.var_fn277_calc_iq__etags_dn7), 0.0, (assign24240_e23180 * locals.var_fn277_calc_iq__etags_dn13),)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign24240_e23182;
        locals.var_fn277_calc_iq__exparg_dn2 = assign24240_e23182_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign24240_e23182_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign24240_e23182_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign24240_e23182_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign24240_e23182_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign24240_e23182_d_n13;
        locals.var_fn277_calc_iq__exparg_rv = 0.0;

        let (assign24250_e23198, assign24250_e23198_d_n2, assign24250_e23198_d_n3, assign24250_e23198_d_n4, assign24250_e23198_d_n7, assign24250_e23198_d_n12, assign24250_e23198_d_n13,) = {
    if ((((locals.var_guard276 != 0.0) && (locals.var_guard308 != 0.0)) && (locals.var_guard309 == 0.0)) && (locals.var_guard310 == 0.0)) {
        let assign24250_e23194: f64 = (locals.var_fn277_calc_iq__etags).exp();
        let assign24250_e23195: f64 = (1.0 + assign24250_e23194);
        let assign24250_e23196: f64 = (assign24250_e23195).ln();
        (assign24250_e23196, ((assign24250_e23194 * locals.var_fn277_calc_iq__etags_dn2) / assign24250_e23195), 0.0, ((assign24250_e23194 * locals.var_fn277_calc_iq__etags_dn4) / assign24250_e23195), ((assign24250_e23194 * locals.var_fn277_calc_iq__etags_dn7) / assign24250_e23195), 0.0, ((assign24250_e23194 * locals.var_fn277_calc_iq__etags_dn13) / assign24250_e23195),)
    } else {
        (locals.var_fn277_calc_iq__exparg, locals.var_fn277_calc_iq__exparg_dn2, locals.var_fn277_calc_iq__exparg_dn3, locals.var_fn277_calc_iq__exparg_dn4, locals.var_fn277_calc_iq__exparg_dn7, locals.var_fn277_calc_iq__exparg_dn12, locals.var_fn277_calc_iq__exparg_dn13,)
    }
};
        locals.var_fn277_calc_iq__exparg = assign24250_e23198;
        locals.var_fn277_calc_iq__exparg_dn2 = assign24250_e23198_d_n2;
        locals.var_fn277_calc_iq__exparg_dn3 = assign24250_e23198_d_n3;
        locals.var_fn277_calc_iq__exparg_dn4 = assign24250_e23198_d_n4;
        locals.var_fn277_calc_iq__exparg_dn7 = assign24250_e23198_d_n7;
        locals.var_fn277_calc_iq__exparg_dn12 = assign24250_e23198_d_n12;
        locals.var_fn277_calc_iq__exparg_dn13 = assign24250_e23198_d_n13;
        locals.var_fn277_calc_iq__exparg_rv = 0.0;

        let (assign24260_e23216, assign24260_e23216_d_n2, assign24260_e23216_d_n3, assign24260_e23216_d_n4, assign24260_e23216_d_n7, assign24260_e23216_d_n12, assign24260_e23216_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard308 != 0.0)) {
        let assign24260_e23204: f64 = (locals.var_fn277_calc_iq__w * locals.var_fn277_calc_iq__ngf);
        let assign24260_e23206: f64 = (assign24260_e23204 * locals.var_fn277_calc_iq__type);
        let assign24260_e23208: f64 = (assign24260_e23206 * locals.var_fn277_calc_iq__cs);
        let assign24260_e23210: f64 = (assign24260_e23208 * locals.var_fn277_calc_iq__two_n_phit0);
        let assign24260_e23212: f64 = (assign24260_e23210 * locals.var_fn277_calc_iq__exparg);
        let assign24260_e23214: f64 = (assign24260_e23212 * locals.var_fn277_calc_iq__trapfracdl);
        (assign24260_e23214, ((assign24260_e23210 * locals.var_fn277_calc_iq__exparg_dn2) * locals.var_fn277_calc_iq__trapfracdl), ((assign24260_e23210 * locals.var_fn277_calc_iq__exparg_dn3) * locals.var_fn277_calc_iq__trapfracdl), ((((assign24260_e23208 * locals.var_fn277_calc_iq__two_n_phit0_dn4) * locals.var_fn277_calc_iq__exparg) + (assign24260_e23210 * locals.var_fn277_calc_iq__exparg_dn4)) * locals.var_fn277_calc_iq__trapfracdl), ((assign24260_e23210 * locals.var_fn277_calc_iq__exparg_dn7) * locals.var_fn277_calc_iq__trapfracdl), ((assign24260_e23210 * locals.var_fn277_calc_iq__exparg_dn12) * locals.var_fn277_calc_iq__trapfracdl), ((assign24260_e23210 * locals.var_fn277_calc_iq__exparg_dn13) * locals.var_fn277_calc_iq__trapfracdl),)
    } else {
        (locals.var_fn277_calc_iq__qsout, locals.var_fn277_calc_iq__qsout_dn2, locals.var_fn277_calc_iq__qsout_dn3, locals.var_fn277_calc_iq__qsout_dn4, locals.var_fn277_calc_iq__qsout_dn7, locals.var_fn277_calc_iq__qsout_dn12, locals.var_fn277_calc_iq__qsout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qsout = assign24260_e23216;
        locals.var_fn277_calc_iq__qsout_dn2 = assign24260_e23216_d_n2;
        locals.var_fn277_calc_iq__qsout_dn3 = assign24260_e23216_d_n3;
        locals.var_fn277_calc_iq__qsout_dn4 = assign24260_e23216_d_n4;
        locals.var_fn277_calc_iq__qsout_dn7 = assign24260_e23216_d_n7;
        locals.var_fn277_calc_iq__qsout_dn12 = assign24260_e23216_d_n12;
        locals.var_fn277_calc_iq__qsout_dn13 = assign24260_e23216_d_n13;
        locals.var_fn277_calc_iq__qsout_rv = 0.0;

        let (assign24270_e23223, assign24270_e23223_d_n2, assign24270_e23223_d_n3, assign24270_e23223_d_n4, assign24270_e23223_d_n7, assign24270_e23223_d_n12, assign24270_e23223_d_n13,) = {
    if ((locals.var_guard276 != 0.0) && (locals.var_guard308 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn277_calc_iq__qsout, locals.var_fn277_calc_iq__qsout_dn2, locals.var_fn277_calc_iq__qsout_dn3, locals.var_fn277_calc_iq__qsout_dn4, locals.var_fn277_calc_iq__qsout_dn7, locals.var_fn277_calc_iq__qsout_dn12, locals.var_fn277_calc_iq__qsout_dn13,)
    }
};
        locals.var_fn277_calc_iq__qsout = assign24270_e23223;
        locals.var_fn277_calc_iq__qsout_dn2 = assign24270_e23223_d_n2;
        locals.var_fn277_calc_iq__qsout_dn3 = assign24270_e23223_d_n3;
        locals.var_fn277_calc_iq__qsout_dn4 = assign24270_e23223_d_n4;
        locals.var_fn277_calc_iq__qsout_dn7 = assign24270_e23223_d_n7;
        locals.var_fn277_calc_iq__qsout_dn12 = assign24270_e23223_d_n12;
        locals.var_fn277_calc_iq__qsout_dn13 = assign24270_e23223_d_n13;
        locals.var_fn277_calc_iq__qsout_rv = 0.0;

        let (assign24300_e23235, assign24300_e23235_d_n2, assign24300_e23235_d_n4, assign24300_e23235_d_n7, assign24300_e23235_d_n12, assign24300_e23235_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_fn277_calc_iq__qgsout, locals.var_fn277_calc_iq__qgsout_dn2, locals.var_fn277_calc_iq__qgsout_dn4, locals.var_fn277_calc_iq__qgsout_dn7, locals.var_fn277_calc_iq__qgsout_dn12, locals.var_fn277_calc_iq__qgsout_dn13,)
    } else {
        (locals.var_qgsfps4, locals.var_qgsfps4_dn2, locals.var_qgsfps4_dn4, locals.var_qgsfps4_dn7, locals.var_qgsfps4_dn12, locals.var_qgsfps4_dn13,)
    }
};
        locals.var_qgsfps4 = assign24300_e23235;
        locals.var_qgsfps4_dn2 = assign24300_e23235_d_n2;
        locals.var_qgsfps4_dn4 = assign24300_e23235_d_n4;
        locals.var_qgsfps4_dn7 = assign24300_e23235_d_n7;
        locals.var_qgsfps4_dn12 = assign24300_e23235_d_n12;
        locals.var_qgsfps4_dn13 = assign24300_e23235_d_n13;
        locals.var_qgsfps4_rv = 0.0;

        let (assign24310_e23239, assign24310_e23239_d_n2, assign24310_e23239_d_n4, assign24310_e23239_d_n7, assign24310_e23239_d_n12, assign24310_e23239_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_fn277_calc_iq__qgdout, locals.var_fn277_calc_iq__qgdout_dn2, locals.var_fn277_calc_iq__qgdout_dn4, locals.var_fn277_calc_iq__qgdout_dn7, locals.var_fn277_calc_iq__qgdout_dn12, locals.var_fn277_calc_iq__qgdout_dn13,)
    } else {
        (locals.var_qgdfps4, locals.var_qgdfps4_dn2, locals.var_qgdfps4_dn4, locals.var_qgdfps4_dn7, locals.var_qgdfps4_dn12, locals.var_qgdfps4_dn13,)
    }
};
        locals.var_qgdfps4 = assign24310_e23239;
        locals.var_qgdfps4_dn2 = assign24310_e23239_d_n2;
        locals.var_qgdfps4_dn4 = assign24310_e23239_d_n4;
        locals.var_qgdfps4_dn7 = assign24310_e23239_d_n7;
        locals.var_qgdfps4_dn12 = assign24310_e23239_d_n12;
        locals.var_qgdfps4_dn13 = assign24310_e23239_d_n13;
        locals.var_qgdfps4_rv = 0.0;

        let (assign24320_e23243, assign24320_e23243_d_n2, assign24320_e23243_d_n3, assign24320_e23243_d_n4, assign24320_e23243_d_n7, assign24320_e23243_d_n12, assign24320_e23243_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_fn277_calc_iq__qcout, locals.var_fn277_calc_iq__qcout_dn2, locals.var_fn277_calc_iq__qcout_dn3, locals.var_fn277_calc_iq__qcout_dn4, locals.var_fn277_calc_iq__qcout_dn7, locals.var_fn277_calc_iq__qcout_dn12, locals.var_fn277_calc_iq__qcout_dn13,)
    } else {
        (locals.var_qcfps4, locals.var_qcfps4_dn2, locals.var_qcfps4_dn3, locals.var_qcfps4_dn4, locals.var_qcfps4_dn7, locals.var_qcfps4_dn12, locals.var_qcfps4_dn13,)
    }
};
        locals.var_qcfps4 = assign24320_e23243;
        locals.var_qcfps4_dn2 = assign24320_e23243_d_n2;
        locals.var_qcfps4_dn3 = assign24320_e23243_d_n3;
        locals.var_qcfps4_dn4 = assign24320_e23243_d_n4;
        locals.var_qcfps4_dn7 = assign24320_e23243_d_n7;
        locals.var_qcfps4_dn12 = assign24320_e23243_d_n12;
        locals.var_qcfps4_dn13 = assign24320_e23243_d_n13;
        locals.var_qcfps4_rv = 0.0;

        let (assign24330_e23247, assign24330_e23247_d_n2, assign24330_e23247_d_n3, assign24330_e23247_d_n4, assign24330_e23247_d_n7, assign24330_e23247_d_n12, assign24330_e23247_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_fn277_calc_iq__qbout, locals.var_fn277_calc_iq__qbout_dn2, locals.var_fn277_calc_iq__qbout_dn3, locals.var_fn277_calc_iq__qbout_dn4, locals.var_fn277_calc_iq__qbout_dn7, locals.var_fn277_calc_iq__qbout_dn12, locals.var_fn277_calc_iq__qbout_dn13,)
    } else {
        (locals.var_qbfps4, locals.var_qbfps4_dn2, locals.var_qbfps4_dn3, locals.var_qbfps4_dn4, locals.var_qbfps4_dn7, locals.var_qbfps4_dn12, locals.var_qbfps4_dn13,)
    }
};
        locals.var_qbfps4 = assign24330_e23247;
        locals.var_qbfps4_dn2 = assign24330_e23247_d_n2;
        locals.var_qbfps4_dn3 = assign24330_e23247_d_n3;
        locals.var_qbfps4_dn4 = assign24330_e23247_d_n4;
        locals.var_qbfps4_dn7 = assign24330_e23247_d_n7;
        locals.var_qbfps4_dn12 = assign24330_e23247_d_n12;
        locals.var_qbfps4_dn13 = assign24330_e23247_d_n13;
        locals.var_qbfps4_rv = 0.0;

        let (assign24340_e23251, assign24340_e23251_d_n2, assign24340_e23251_d_n3, assign24340_e23251_d_n4, assign24340_e23251_d_n7, assign24340_e23251_d_n12, assign24340_e23251_d_n13,) = {
    if (locals.var_guard276 != 0.0) {
        (locals.var_fn277_calc_iq__qsout, locals.var_fn277_calc_iq__qsout_dn2, locals.var_fn277_calc_iq__qsout_dn3, locals.var_fn277_calc_iq__qsout_dn4, locals.var_fn277_calc_iq__qsout_dn7, locals.var_fn277_calc_iq__qsout_dn12, locals.var_fn277_calc_iq__qsout_dn13,)
    } else {
        (locals.var_qsfps4, locals.var_qsfps4_dn2, locals.var_qsfps4_dn3, locals.var_qsfps4_dn4, locals.var_qsfps4_dn7, locals.var_qsfps4_dn12, locals.var_qsfps4_dn13,)
    }
};
        locals.var_qsfps4 = assign24340_e23251;
        locals.var_qsfps4_dn2 = assign24340_e23251_d_n2;
        locals.var_qsfps4_dn3 = assign24340_e23251_d_n3;
        locals.var_qsfps4_dn4 = assign24340_e23251_d_n4;
        locals.var_qsfps4_dn7 = assign24340_e23251_d_n7;
        locals.var_qsfps4_dn12 = assign24340_e23251_d_n12;
        locals.var_qsfps4_dn13 = assign24340_e23251_d_n13;
        locals.var_qsfps4_rv = 0.0;

        let assign24380_e23266: f64 = if p.p144 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard311 = assign24380_e23266;
        locals.var_guard311_rv = 0.0;

        locals.var_fn382_calc_iq__return = 0.0;
        locals.var_fn382_calc_iq__return_dn4 = 0.0;
        locals.var_fn382_calc_iq__return_dn5 = 0.0;
        locals.var_fn382_calc_iq__return_dn8 = 0.0;
        locals.var_fn382_calc_iq__return_dn9 = 0.0;
        locals.var_fn382_calc_iq__return_dn22 = 0.0;
        locals.var_fn382_calc_iq__return_dn23 = 0.0;
        locals.var_fn382_calc_iq__return_dn25 = 0.0;
        locals.var_fn382_calc_iq__return_dn26 = 0.0;
        locals.var_fn382_calc_iq__return_rv = 0.0;

        locals.var_fn382_calc_iq__idsout = 0.0;
        locals.var_fn382_calc_iq__idsout_dn4 = 0.0;
        locals.var_fn382_calc_iq__idsout_dn5 = 0.0;
        locals.var_fn382_calc_iq__idsout_dn8 = 0.0;
        locals.var_fn382_calc_iq__idsout_dn9 = 0.0;
        locals.var_fn382_calc_iq__idsout_dn22 = 0.0;
        locals.var_fn382_calc_iq__idsout_dn23 = 0.0;
        locals.var_fn382_calc_iq__idsout_dn25 = 0.0;
        locals.var_fn382_calc_iq__idsout_dn26 = 0.0;
        locals.var_fn382_calc_iq__idsout_rv = 0.0;

        locals.var_fn382_calc_iq__qgsout = 0.0;
        locals.var_fn382_calc_iq__qgsout_dn4 = 0.0;
        locals.var_fn382_calc_iq__qgsout_dn5 = 0.0;
        locals.var_fn382_calc_iq__qgsout_dn8 = 0.0;
        locals.var_fn382_calc_iq__qgsout_dn9 = 0.0;
        locals.var_fn382_calc_iq__qgsout_dn22 = 0.0;
        locals.var_fn382_calc_iq__qgsout_dn23 = 0.0;
        locals.var_fn382_calc_iq__qgsout_dn25 = 0.0;
        locals.var_fn382_calc_iq__qgsout_dn26 = 0.0;
        locals.var_fn382_calc_iq__qgsout_rv = 0.0;

        locals.var_fn382_calc_iq__qgdout = 0.0;
        locals.var_fn382_calc_iq__qgdout_dn4 = 0.0;
        locals.var_fn382_calc_iq__qgdout_dn5 = 0.0;
        locals.var_fn382_calc_iq__qgdout_dn8 = 0.0;
        locals.var_fn382_calc_iq__qgdout_dn9 = 0.0;
        locals.var_fn382_calc_iq__qgdout_dn22 = 0.0;
        locals.var_fn382_calc_iq__qgdout_dn23 = 0.0;
        locals.var_fn382_calc_iq__qgdout_dn25 = 0.0;
        locals.var_fn382_calc_iq__qgdout_dn26 = 0.0;
        locals.var_fn382_calc_iq__qgdout_rv = 0.0;

        locals.var_fn382_calc_iq__vtdibl = 0.0;
        locals.var_fn382_calc_iq__vtdibl_dn4 = 0.0;
        locals.var_fn382_calc_iq__vtdibl_dn5 = 0.0;
        locals.var_fn382_calc_iq__vtdibl_dn9 = 0.0;
        locals.var_fn382_calc_iq__vtdibl_rv = 0.0;

        locals.var_fn382_calc_iq__vdsat1 = 0.0;
        locals.var_fn382_calc_iq__vdsat1_dn4 = 0.0;
        locals.var_fn382_calc_iq__vdsat1_dn5 = 0.0;
        locals.var_fn382_calc_iq__vdsat1_dn8 = 0.0;
        locals.var_fn382_calc_iq__vdsat1_dn9 = 0.0;
        locals.var_fn382_calc_iq__vdsat1_rv = 0.0;

        locals.var_fn382_calc_iq__vgsin = locals.var_vgsi;
        locals.var_fn382_calc_iq__vgsin_dn8 = locals.var_vgsi_dn8;
        locals.var_fn382_calc_iq__vgsin_dn9 = locals.var_vgsi_dn9;
        locals.var_fn382_calc_iq__vgsin_rv = 0.0;

        locals.var_fn382_calc_iq__vdsin = locals.var_vdsi;
        locals.var_fn382_calc_iq__vdsin_dn5 = locals.var_vdsi_dn5;
        locals.var_fn382_calc_iq__vdsin_dn9 = locals.var_vdsi_dn9;
        locals.var_fn382_calc_iq__vdsin_rv = 0.0;

        locals.var_fn382_calc_iq__qcbflag = 0.0;
        locals.var_fn382_calc_iq__qcbflag_rv = 0.0;

        locals.var_fn382_calc_iq__vcin = 0.0;
        locals.var_fn382_calc_iq__vcin_rv = 0.0;

        locals.var_fn382_calc_iq__vbin = 0.0;
        locals.var_fn382_calc_iq__vbin_rv = 0.0;

        locals.var_fn382_calc_iq__qgsflag = 0.0;
        locals.var_fn382_calc_iq__qgsflag_rv = 0.0;

        locals.var_fn382_calc_iq__tambin = locals.var_tdut;
        locals.var_fn382_calc_iq__tambin_dn4 = locals.var_tdut_dn4;
        locals.var_fn382_calc_iq__tambin_rv = 0.0;

        locals.var_fn382_calc_iq__tnomin = locals.var_tnomk;
        locals.var_fn382_calc_iq__tnomin_rv = 0.0;

        locals.var_fn382_calc_iq__phitin = locals.var_phit;
        locals.var_fn382_calc_iq__phitin_dn4 = locals.var_phit_dn4;
        locals.var_fn382_calc_iq__phitin_rv = 0.0;

        locals.var_fn382_calc_iq__w = p.p0;
        locals.var_fn382_calc_iq__w_rv = 0.0;

        locals.var_fn382_calc_iq__lin = p.p1;
        locals.var_fn382_calc_iq__lin_rv = 0.0;

        locals.var_fn382_calc_iq__cgin = locals.var_cgt;
        locals.var_fn382_calc_iq__cgin_dn4 = locals.var_cgt_dn4;
        locals.var_fn382_calc_iq__cgin_rv = 0.0;

        locals.var_fn382_calc_iq__vto = p.p35;
        locals.var_fn382_calc_iq__vto_rv = 0.0;

        locals.var_fn382_calc_iq__ss = p.p36;
        locals.var_fn382_calc_iq__ss_rv = 0.0;

        locals.var_fn382_calc_iq__delta1 = p.p37;
        locals.var_fn382_calc_iq__delta1_rv = 0.0;

        locals.var_fn382_calc_iq__delta2 = p.p38;
        locals.var_fn382_calc_iq__delta2_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_67(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        locals.var_fn382_calc_iq__nd = p.p40;
        locals.var_fn382_calc_iq__nd_rv = 0.0;

        locals.var_fn382_calc_iq__alpha = p.p41;
        locals.var_fn382_calc_iq__alpha_rv = 0.0;

        locals.var_fn382_calc_iq__vel0 = p.p32;
        locals.var_fn382_calc_iq__vel0_rv = 0.0;

        locals.var_fn382_calc_iq__mu0 = p.p33;
        locals.var_fn382_calc_iq__mu0_rv = 0.0;

        locals.var_fn382_calc_iq__beta = p.p34;
        locals.var_fn382_calc_iq__beta_rv = 0.0;

        locals.var_fn382_calc_iq__mtheta = p.p44;
        locals.var_fn382_calc_iq__mtheta_rv = 0.0;

        locals.var_fn382_calc_iq__vtheta = p.p43;
        locals.var_fn382_calc_iq__vtheta_rv = 0.0;

        locals.var_fn382_calc_iq__vtzeta = p.p46;
        locals.var_fn382_calc_iq__vtzeta_rv = 0.0;

        locals.var_fn382_calc_iq__dibsat = p.p39;
        locals.var_fn382_calc_iq__dibsat_rv = 0.0;

        locals.var_fn382_calc_iq__epsilon = p.p47;
        locals.var_fn382_calc_iq__epsilon_rv = 0.0;

        locals.var_fn382_calc_iq__vzeta = p.p45;
        locals.var_fn382_calc_iq__vzeta_rv = 0.0;

        locals.var_fn382_calc_iq__lambda = p.p42;
        locals.var_fn382_calc_iq__lambda_rv = 0.0;

        locals.var_fn382_calc_iq__ngf = p.p2;
        locals.var_fn382_calc_iq__ngf_rv = 0.0;

        locals.var_fn382_calc_iq__type = p.p6;
        locals.var_fn382_calc_iq__type_rv = 0.0;

        locals.var_fn382_calc_iq__trapfracdl = locals.var_chargefrac;
        locals.var_fn382_calc_iq__trapfracdl_dn22 = locals.var_chargefrac_dn22;
        locals.var_fn382_calc_iq__trapfracdl_dn23 = locals.var_chargefrac_dn23;
        locals.var_fn382_calc_iq__trapfracdl_dn25 = locals.var_chargefrac_dn25;
        locals.var_fn382_calc_iq__trapfracdl_dn26 = locals.var_chargefrac_dn26;
        locals.var_fn382_calc_iq__trapfracdl_rv = 0.0;

        locals.var_fn382_calc_iq__alpha_phit = 0.0;
        locals.var_fn382_calc_iq__alpha_phit_dn4 = 0.0;
        locals.var_fn382_calc_iq__alpha_phit_rv = 0.0;

        locals.var_fn382_calc_iq__delta = 0.0;
        locals.var_fn382_calc_iq__delta_dn5 = 0.0;
        locals.var_fn382_calc_iq__delta_dn9 = 0.0;
        locals.var_fn382_calc_iq__delta_rv = 0.0;

        locals.var_fn382_calc_iq__n = 0.0;
        locals.var_fn382_calc_iq__n_dn4 = 0.0;
        locals.var_fn382_calc_iq__n_dn5 = 0.0;
        locals.var_fn382_calc_iq__n_dn9 = 0.0;
        locals.var_fn382_calc_iq__n_rv = 0.0;

        locals.var_fn382_calc_iq__vtof = 0.0;
        locals.var_fn382_calc_iq__vtof_dn4 = 0.0;
        locals.var_fn382_calc_iq__vtof_rv = 0.0;

        locals.var_fn382_calc_iq__vsatdibl = 0.0;
        locals.var_fn382_calc_iq__vsatdibl_dn5 = 0.0;
        locals.var_fn382_calc_iq__vsatdibl_dn9 = 0.0;
        locals.var_fn382_calc_iq__vsatdibl_rv = 0.0;

        locals.var_fn382_calc_iq__ffs = 0.0;
        locals.var_fn382_calc_iq__ffs_dn4 = 0.0;
        locals.var_fn382_calc_iq__ffs_dn5 = 0.0;
        locals.var_fn382_calc_iq__ffs_dn8 = 0.0;
        locals.var_fn382_calc_iq__ffs_dn9 = 0.0;
        locals.var_fn382_calc_iq__ffs_rv = 0.0;

        locals.var_fn382_calc_iq__two_n_phit = 0.0;
        locals.var_fn382_calc_iq__two_n_phit_dn4 = 0.0;
        locals.var_fn382_calc_iq__two_n_phit_dn5 = 0.0;
        locals.var_fn382_calc_iq__two_n_phit_dn9 = 0.0;
        locals.var_fn382_calc_iq__two_n_phit_rv = 0.0;

        locals.var_fn382_calc_iq__qref = 0.0;
        locals.var_fn382_calc_iq__qref_dn4 = 0.0;
        locals.var_fn382_calc_iq__qref_dn5 = 0.0;
        locals.var_fn382_calc_iq__qref_dn9 = 0.0;
        locals.var_fn382_calc_iq__qref_rv = 0.0;

        locals.var_fn382_calc_iq__etas = 0.0;
        locals.var_fn382_calc_iq__etas_dn4 = 0.0;
        locals.var_fn382_calc_iq__etas_dn5 = 0.0;
        locals.var_fn382_calc_iq__etas_dn8 = 0.0;
        locals.var_fn382_calc_iq__etas_dn9 = 0.0;
        locals.var_fn382_calc_iq__etas_rv = 0.0;

        locals.var_fn382_calc_iq__qinvs = 0.0;
        locals.var_fn382_calc_iq__qinvs_dn4 = 0.0;
        locals.var_fn382_calc_iq__qinvs_dn5 = 0.0;
        locals.var_fn382_calc_iq__qinvs_dn8 = 0.0;
        locals.var_fn382_calc_iq__qinvs_dn9 = 0.0;
        locals.var_fn382_calc_iq__qinvs_rv = 0.0;

        locals.var_fn382_calc_iq__muf = 0.0;
        locals.var_fn382_calc_iq__muf_dn4 = 0.0;
        locals.var_fn382_calc_iq__muf_dn5 = 0.0;
        locals.var_fn382_calc_iq__muf_dn8 = 0.0;
        locals.var_fn382_calc_iq__muf_dn9 = 0.0;
        locals.var_fn382_calc_iq__muf_rv = 0.0;

        locals.var_fn382_calc_iq__vx = 0.0;
        locals.var_fn382_calc_iq__vx_dn4 = 0.0;
        locals.var_fn382_calc_iq__vx_dn5 = 0.0;
        locals.var_fn382_calc_iq__vx_dn8 = 0.0;
        locals.var_fn382_calc_iq__vx_dn9 = 0.0;
        locals.var_fn382_calc_iq__vx_rv = 0.0;

        locals.var_fn382_calc_iq__vxf = 0.0;
        locals.var_fn382_calc_iq__vxf_dn4 = 0.0;
        locals.var_fn382_calc_iq__vxf_dn5 = 0.0;
        locals.var_fn382_calc_iq__vxf_dn8 = 0.0;
        locals.var_fn382_calc_iq__vxf_dn9 = 0.0;
        locals.var_fn382_calc_iq__vxf_rv = 0.0;

        locals.var_fn382_calc_iq__n0 = 0.0;
        locals.var_fn382_calc_iq__n0_dn4 = 0.0;
        locals.var_fn382_calc_iq__n0_rv = 0.0;

        locals.var_fn382_calc_iq__ffs0 = 0.0;
        locals.var_fn382_calc_iq__ffs0_dn4 = 0.0;
        locals.var_fn382_calc_iq__ffs0_dn5 = 0.0;
        locals.var_fn382_calc_iq__ffs0_dn8 = 0.0;
        locals.var_fn382_calc_iq__ffs0_dn9 = 0.0;
        locals.var_fn382_calc_iq__ffs0_rv = 0.0;

        locals.var_fn382_calc_iq__two_n_phit0 = 0.0;
        locals.var_fn382_calc_iq__two_n_phit0_dn4 = 0.0;
        locals.var_fn382_calc_iq__two_n_phit0_rv = 0.0;

        locals.var_fn382_calc_iq__qref0 = 0.0;
        locals.var_fn382_calc_iq__qref0_dn4 = 0.0;
        locals.var_fn382_calc_iq__qref0_rv = 0.0;

        locals.var_fn382_calc_iq__etas0 = 0.0;
        locals.var_fn382_calc_iq__etas0_dn4 = 0.0;
        locals.var_fn382_calc_iq__etas0_dn5 = 0.0;
        locals.var_fn382_calc_iq__etas0_dn8 = 0.0;
        locals.var_fn382_calc_iq__etas0_dn9 = 0.0;
        locals.var_fn382_calc_iq__etas0_rv = 0.0;

        locals.var_fn382_calc_iq__qinvs0 = 0.0;
        locals.var_fn382_calc_iq__qinvs0_dn4 = 0.0;
        locals.var_fn382_calc_iq__qinvs0_dn5 = 0.0;
        locals.var_fn382_calc_iq__qinvs0_dn8 = 0.0;
        locals.var_fn382_calc_iq__qinvs0_dn9 = 0.0;
        locals.var_fn382_calc_iq__qinvs0_rv = 0.0;

        locals.var_fn382_calc_iq__muf0 = 0.0;
        locals.var_fn382_calc_iq__muf0_dn4 = 0.0;
        locals.var_fn382_calc_iq__muf0_rv = 0.0;

        locals.var_fn382_calc_iq__vx0 = 0.0;
        locals.var_fn382_calc_iq__vx0_dn4 = 0.0;
        locals.var_fn382_calc_iq__vx0_rv = 0.0;

        locals.var_fn382_calc_iq__tfacmobin = 0.0;
        locals.var_fn382_calc_iq__tfacmobin_dn4 = 0.0;
        locals.var_fn382_calc_iq__tfacmobin_rv = 0.0;

        locals.var_fn382_calc_iq__ff = 0.0;
        locals.var_fn382_calc_iq__ff_dn4 = 0.0;
        locals.var_fn382_calc_iq__ff_dn5 = 0.0;
        locals.var_fn382_calc_iq__ff_dn8 = 0.0;
        locals.var_fn382_calc_iq__ff_dn9 = 0.0;
        locals.var_fn382_calc_iq__ff_rv = 0.0;

        locals.var_fn382_calc_iq__eta = 0.0;
        locals.var_fn382_calc_iq__eta_dn4 = 0.0;
        locals.var_fn382_calc_iq__eta_dn5 = 0.0;
        locals.var_fn382_calc_iq__eta_dn8 = 0.0;
        locals.var_fn382_calc_iq__eta_dn9 = 0.0;
        locals.var_fn382_calc_iq__eta_rv = 0.0;

        locals.var_fn382_calc_iq__qinvv = 0.0;
        locals.var_fn382_calc_iq__qinvv_dn4 = 0.0;
        locals.var_fn382_calc_iq__qinvv_dn5 = 0.0;
        locals.var_fn382_calc_iq__qinvv_dn8 = 0.0;
        locals.var_fn382_calc_iq__qinvv_dn9 = 0.0;
        locals.var_fn382_calc_iq__qinvv_rv = 0.0;

        locals.var_fn382_calc_iq__ff0 = 0.0;
        locals.var_fn382_calc_iq__ff0_dn4 = 0.0;
        locals.var_fn382_calc_iq__ff0_dn5 = 0.0;
        locals.var_fn382_calc_iq__ff0_dn8 = 0.0;
        locals.var_fn382_calc_iq__ff0_dn9 = 0.0;
        locals.var_fn382_calc_iq__ff0_rv = 0.0;

        locals.var_fn382_calc_iq__eta0 = 0.0;
        locals.var_fn382_calc_iq__eta0_dn4 = 0.0;
        locals.var_fn382_calc_iq__eta0_dn5 = 0.0;
        locals.var_fn382_calc_iq__eta0_dn8 = 0.0;
        locals.var_fn382_calc_iq__eta0_dn9 = 0.0;
        locals.var_fn382_calc_iq__eta0_rv = 0.0;

        locals.var_fn382_calc_iq__qinvv0 = 0.0;
        locals.var_fn382_calc_iq__qinvv0_dn4 = 0.0;
        locals.var_fn382_calc_iq__qinvv0_dn5 = 0.0;
        locals.var_fn382_calc_iq__qinvv0_dn8 = 0.0;
        locals.var_fn382_calc_iq__qinvv0_dn9 = 0.0;
        locals.var_fn382_calc_iq__qinvv0_rv = 0.0;

        locals.var_fn382_calc_iq__vdsats = 0.0;
        locals.var_fn382_calc_iq__vdsats_dn4 = 0.0;
        locals.var_fn382_calc_iq__vdsats_dn5 = 0.0;
        locals.var_fn382_calc_iq__vdsats_dn8 = 0.0;
        locals.var_fn382_calc_iq__vdsats_dn9 = 0.0;
        locals.var_fn382_calc_iq__vdsats_rv = 0.0;

        locals.var_fn382_calc_iq__vdsats1 = 0.0;
        locals.var_fn382_calc_iq__vdsats1_dn4 = 0.0;
        locals.var_fn382_calc_iq__vdsats1_dn5 = 0.0;
        locals.var_fn382_calc_iq__vdsats1_dn8 = 0.0;
        locals.var_fn382_calc_iq__vdsats1_dn9 = 0.0;
        locals.var_fn382_calc_iq__vdsats1_rv = 0.0;

        locals.var_fn382_calc_iq__vdsat = 0.0;
        locals.var_fn382_calc_iq__vdsat_dn4 = 0.0;
        locals.var_fn382_calc_iq__vdsat_dn5 = 0.0;
        locals.var_fn382_calc_iq__vdsat_dn8 = 0.0;
        locals.var_fn382_calc_iq__vdsat_dn9 = 0.0;
        locals.var_fn382_calc_iq__vdsat_rv = 0.0;

        locals.var_fn382_calc_iq__fsd = 0.0;
        locals.var_fn382_calc_iq__fsd_dn4 = 0.0;
        locals.var_fn382_calc_iq__fsd_dn5 = 0.0;
        locals.var_fn382_calc_iq__fsd_dn8 = 0.0;
        locals.var_fn382_calc_iq__fsd_dn9 = 0.0;
        locals.var_fn382_calc_iq__fsd_rv = 0.0;

        locals.var_fn382_calc_iq__vdx = 0.0;
        locals.var_fn382_calc_iq__vdx_dn4 = 0.0;
        locals.var_fn382_calc_iq__vdx_dn5 = 0.0;
        locals.var_fn382_calc_iq__vdx_dn8 = 0.0;
        locals.var_fn382_calc_iq__vdx_dn9 = 0.0;
        locals.var_fn382_calc_iq__vdx_rv = 0.0;

        locals.var_fn382_calc_iq__fds = 0.0;
        locals.var_fn382_calc_iq__fds_dn4 = 0.0;
        locals.var_fn382_calc_iq__fds_dn5 = 0.0;
        locals.var_fn382_calc_iq__fds_dn8 = 0.0;
        locals.var_fn382_calc_iq__fds_dn9 = 0.0;
        locals.var_fn382_calc_iq__fds_rv = 0.0;

        locals.var_fn382_calc_iq__vsx = 0.0;
        locals.var_fn382_calc_iq__vsx_dn4 = 0.0;
        locals.var_fn382_calc_iq__vsx_dn5 = 0.0;
        locals.var_fn382_calc_iq__vsx_dn8 = 0.0;
        locals.var_fn382_calc_iq__vsx_dn9 = 0.0;
        locals.var_fn382_calc_iq__vsx_rv = 0.0;

        locals.var_fn382_calc_iq__ffd = 0.0;
        locals.var_fn382_calc_iq__ffd_dn4 = 0.0;
        locals.var_fn382_calc_iq__ffd_dn5 = 0.0;
        locals.var_fn382_calc_iq__ffd_dn8 = 0.0;
        locals.var_fn382_calc_iq__ffd_dn9 = 0.0;
        locals.var_fn382_calc_iq__ffd_rv = 0.0;

        locals.var_fn382_calc_iq__etad = 0.0;
        locals.var_fn382_calc_iq__etad_dn4 = 0.0;
        locals.var_fn382_calc_iq__etad_dn5 = 0.0;
        locals.var_fn382_calc_iq__etad_dn8 = 0.0;
        locals.var_fn382_calc_iq__etad_dn9 = 0.0;
        locals.var_fn382_calc_iq__etad_rv = 0.0;

        locals.var_fn382_calc_iq__qinvd = 0.0;
        locals.var_fn382_calc_iq__qinvd_dn4 = 0.0;
        locals.var_fn382_calc_iq__qinvd_dn5 = 0.0;
        locals.var_fn382_calc_iq__qinvd_dn8 = 0.0;
        locals.var_fn382_calc_iq__qinvd_dn9 = 0.0;
        locals.var_fn382_calc_iq__qinvd_rv = 0.0;

        locals.var_fn382_calc_iq__vdsc = 0.0;
        locals.var_fn382_calc_iq__vdsc_dn4 = 0.0;
        locals.var_fn382_calc_iq__vdsc_dn5 = 0.0;
        locals.var_fn382_calc_iq__vdsc_dn8 = 0.0;
        locals.var_fn382_calc_iq__vdsc_dn9 = 0.0;
        locals.var_fn382_calc_iq__vdsc_rv = 0.0;

        locals.var_fn382_calc_iq__fsat = 0.0;
        locals.var_fn382_calc_iq__fsat_dn4 = 0.0;
        locals.var_fn382_calc_iq__fsat_dn5 = 0.0;
        locals.var_fn382_calc_iq__fsat_dn8 = 0.0;
        locals.var_fn382_calc_iq__fsat_dn9 = 0.0;
        locals.var_fn382_calc_iq__fsat_rv = 0.0;

        locals.var_fn382_calc_iq__vel = 0.0;
        locals.var_fn382_calc_iq__vel_dn4 = 0.0;
        locals.var_fn382_calc_iq__vel_dn5 = 0.0;
        locals.var_fn382_calc_iq__vel_dn8 = 0.0;
        locals.var_fn382_calc_iq__vel_dn9 = 0.0;
        locals.var_fn382_calc_iq__vel_rv = 0.0;

        locals.var_fn382_calc_iq__vdsats0 = 0.0;
        locals.var_fn382_calc_iq__vdsats0_dn4 = 0.0;
        locals.var_fn382_calc_iq__vdsats0_rv = 0.0;

        locals.var_fn382_calc_iq__vdsats10 = 0.0;
        locals.var_fn382_calc_iq__vdsats10_dn4 = 0.0;
        locals.var_fn382_calc_iq__vdsats10_dn5 = 0.0;
        locals.var_fn382_calc_iq__vdsats10_dn8 = 0.0;
        locals.var_fn382_calc_iq__vdsats10_dn9 = 0.0;
        locals.var_fn382_calc_iq__vdsats10_rv = 0.0;

        locals.var_fn382_calc_iq__vdsat10 = 0.0;
        locals.var_fn382_calc_iq__vdsat10_dn4 = 0.0;
        locals.var_fn382_calc_iq__vdsat10_dn5 = 0.0;
        locals.var_fn382_calc_iq__vdsat10_dn8 = 0.0;
        locals.var_fn382_calc_iq__vdsat10_dn9 = 0.0;
        locals.var_fn382_calc_iq__vdsat10_rv = 0.0;

        locals.var_fn382_calc_iq__fsd0 = 0.0;
        locals.var_fn382_calc_iq__fsd0_dn4 = 0.0;
        locals.var_fn382_calc_iq__fsd0_dn5 = 0.0;
        locals.var_fn382_calc_iq__fsd0_dn8 = 0.0;
        locals.var_fn382_calc_iq__fsd0_dn9 = 0.0;
        locals.var_fn382_calc_iq__fsd0_rv = 0.0;

        locals.var_fn382_calc_iq__vdx0 = 0.0;
        locals.var_fn382_calc_iq__vdx0_dn4 = 0.0;
        locals.var_fn382_calc_iq__vdx0_dn5 = 0.0;
        locals.var_fn382_calc_iq__vdx0_dn8 = 0.0;
        locals.var_fn382_calc_iq__vdx0_dn9 = 0.0;
        locals.var_fn382_calc_iq__vdx0_rv = 0.0;

        locals.var_fn382_calc_iq__fds0 = 0.0;
        locals.var_fn382_calc_iq__fds0_dn4 = 0.0;
        locals.var_fn382_calc_iq__fds0_dn5 = 0.0;
        locals.var_fn382_calc_iq__fds0_dn8 = 0.0;
        locals.var_fn382_calc_iq__fds0_dn9 = 0.0;
        locals.var_fn382_calc_iq__fds0_rv = 0.0;

        locals.var_fn382_calc_iq__vsx0 = 0.0;
        locals.var_fn382_calc_iq__vsx0_dn4 = 0.0;
        locals.var_fn382_calc_iq__vsx0_dn5 = 0.0;
        locals.var_fn382_calc_iq__vsx0_dn8 = 0.0;
        locals.var_fn382_calc_iq__vsx0_dn9 = 0.0;
        locals.var_fn382_calc_iq__vsx0_rv = 0.0;

        locals.var_fn382_calc_iq__ffd0 = 0.0;
        locals.var_fn382_calc_iq__ffd0_dn4 = 0.0;
        locals.var_fn382_calc_iq__ffd0_dn5 = 0.0;
        locals.var_fn382_calc_iq__ffd0_dn8 = 0.0;
        locals.var_fn382_calc_iq__ffd0_dn9 = 0.0;
        locals.var_fn382_calc_iq__ffd0_rv = 0.0;

        locals.var_fn382_calc_iq__etad0 = 0.0;
        locals.var_fn382_calc_iq__etad0_dn4 = 0.0;
        locals.var_fn382_calc_iq__etad0_dn5 = 0.0;
        locals.var_fn382_calc_iq__etad0_dn8 = 0.0;
        locals.var_fn382_calc_iq__etad0_dn9 = 0.0;
        locals.var_fn382_calc_iq__etad0_rv = 0.0;

        locals.var_fn382_calc_iq__qinvd0 = 0.0;
        locals.var_fn382_calc_iq__qinvd0_dn4 = 0.0;
        locals.var_fn382_calc_iq__qinvd0_dn5 = 0.0;
        locals.var_fn382_calc_iq__qinvd0_dn8 = 0.0;
        locals.var_fn382_calc_iq__qinvd0_dn9 = 0.0;
        locals.var_fn382_calc_iq__qinvd0_rv = 0.0;

        locals.var_fn382_calc_iq__qs2 = 0.0;
        locals.var_fn382_calc_iq__qs2_dn4 = 0.0;
        locals.var_fn382_calc_iq__qs2_dn5 = 0.0;
        locals.var_fn382_calc_iq__qs2_dn8 = 0.0;
        locals.var_fn382_calc_iq__qs2_dn9 = 0.0;
        locals.var_fn382_calc_iq__qs2_rv = 0.0;

        locals.var_fn382_calc_iq__qs3 = 0.0;
        locals.var_fn382_calc_iq__qs3_dn4 = 0.0;
        locals.var_fn382_calc_iq__qs3_dn5 = 0.0;
        locals.var_fn382_calc_iq__qs3_dn8 = 0.0;
        locals.var_fn382_calc_iq__qs3_dn9 = 0.0;
        locals.var_fn382_calc_iq__qs3_rv = 0.0;

        locals.var_fn382_calc_iq__qd2 = 0.0;
        locals.var_fn382_calc_iq__qd2_dn4 = 0.0;
        locals.var_fn382_calc_iq__qd2_dn5 = 0.0;
        locals.var_fn382_calc_iq__qd2_dn8 = 0.0;
        locals.var_fn382_calc_iq__qd2_dn9 = 0.0;
        locals.var_fn382_calc_iq__qd2_rv = 0.0;

        locals.var_fn382_calc_iq__qd3 = 0.0;
        locals.var_fn382_calc_iq__qd3_dn4 = 0.0;
        locals.var_fn382_calc_iq__qd3_dn5 = 0.0;
        locals.var_fn382_calc_iq__qd3_dn8 = 0.0;
        locals.var_fn382_calc_iq__qd3_dn9 = 0.0;
        locals.var_fn382_calc_iq__qd3_rv = 0.0;

        locals.var_fn382_calc_iq__qsqd = 0.0;
        locals.var_fn382_calc_iq__qsqd_dn4 = 0.0;
        locals.var_fn382_calc_iq__qsqd_dn5 = 0.0;
        locals.var_fn382_calc_iq__qsqd_dn8 = 0.0;
        locals.var_fn382_calc_iq__qsqd_dn9 = 0.0;
        locals.var_fn382_calc_iq__qsqd_rv = 0.0;

        locals.var_fn382_calc_iq__qinvdd = 0.0;
        locals.var_fn382_calc_iq__qinvdd_dn4 = 0.0;
        locals.var_fn382_calc_iq__qinvdd_dn5 = 0.0;
        locals.var_fn382_calc_iq__qinvdd_dn8 = 0.0;
        locals.var_fn382_calc_iq__qinvdd_dn9 = 0.0;
        locals.var_fn382_calc_iq__qinvdd_rv = 0.0;

        locals.var_fn382_calc_iq__qd1 = 0.0;
        locals.var_fn382_calc_iq__qd1_dn4 = 0.0;
        locals.var_fn382_calc_iq__qd1_dn5 = 0.0;
        locals.var_fn382_calc_iq__qd1_dn8 = 0.0;
        locals.var_fn382_calc_iq__qd1_dn9 = 0.0;
        locals.var_fn382_calc_iq__qd1_rv = 0.0;

        locals.var_fn382_calc_iq__qs = 0.0;
        locals.var_fn382_calc_iq__qs_dn4 = 0.0;
        locals.var_fn382_calc_iq__qs_dn5 = 0.0;
        locals.var_fn382_calc_iq__qs_dn8 = 0.0;
        locals.var_fn382_calc_iq__qs_dn9 = 0.0;
        locals.var_fn382_calc_iq__qs_rv = 0.0;

        locals.var_fn382_calc_iq__qd = 0.0;
        locals.var_fn382_calc_iq__qd_dn4 = 0.0;
        locals.var_fn382_calc_iq__qd_dn5 = 0.0;
        locals.var_fn382_calc_iq__qd_dn8 = 0.0;
        locals.var_fn382_calc_iq__qd_dn9 = 0.0;
        locals.var_fn382_calc_iq__qd_rv = 0.0;

        locals.var_fn382_calc_iq__etac = 0.0;
        locals.var_fn382_calc_iq__etac_dn4 = 0.0;
        locals.var_fn382_calc_iq__etac_rv = 0.0;

        locals.var_fn382_calc_iq__etab = 0.0;
        locals.var_fn382_calc_iq__etab_dn4 = 0.0;
        locals.var_fn382_calc_iq__etab_rv = 0.0;

        locals.var_fn382_calc_iq__etags = 0.0;
        locals.var_fn382_calc_iq__etags_dn4 = 0.0;
        locals.var_fn382_calc_iq__etags_dn8 = 0.0;
        locals.var_fn382_calc_iq__etags_dn9 = 0.0;
        locals.var_fn382_calc_iq__etags_rv = 0.0;

        locals.var_fn382_calc_iq__exparg = 0.0;
        locals.var_fn382_calc_iq__exparg_dn4 = 0.0;
        locals.var_fn382_calc_iq__exparg_dn5 = 0.0;
        locals.var_fn382_calc_iq__exparg_dn8 = 0.0;
        locals.var_fn382_calc_iq__exparg_dn9 = 0.0;
        locals.var_fn382_calc_iq__exparg_rv = 0.0;

        locals.var_fn382_calc_iq__myarg = 0.0;
        locals.var_fn382_calc_iq__myarg_dn4 = 0.0;
        locals.var_fn382_calc_iq__myarg_dn5 = 0.0;
        locals.var_fn382_calc_iq__myarg_dn8 = 0.0;
        locals.var_fn382_calc_iq__myarg_dn9 = 0.0;
        locals.var_fn382_calc_iq__myarg_rv = 0.0;

        locals.var_fn382_calc_iq__absvdsin = 0.0;
        locals.var_fn382_calc_iq__absvdsin_dn5 = 0.0;
        locals.var_fn382_calc_iq__absvdsin_dn9 = 0.0;
        locals.var_fn382_calc_iq__absvdsin_rv = 0.0;

        locals.var_fn382_calc_iq__vgdin = 0.0;
        locals.var_fn382_calc_iq__vgdin_dn5 = 0.0;
        locals.var_fn382_calc_iq__vgdin_dn8 = 0.0;
        locals.var_fn382_calc_iq__vgdin_dn9 = 0.0;
        locals.var_fn382_calc_iq__vgdin_rv = 0.0;

        locals.var_fn382_calc_iq__exparg0 = 0.0;
        locals.var_fn382_calc_iq__exparg0_dn4 = 0.0;
        locals.var_fn382_calc_iq__exparg0_dn5 = 0.0;
        locals.var_fn382_calc_iq__exparg0_dn8 = 0.0;
        locals.var_fn382_calc_iq__exparg0_dn9 = 0.0;
        locals.var_fn382_calc_iq__exparg0_rv = 0.0;

        locals.var_fn382_calc_iq__myarg0 = 0.0;
        locals.var_fn382_calc_iq__myarg0_dn4 = 0.0;
        locals.var_fn382_calc_iq__myarg0_rv = 0.0;

        let (assign31170_e28252, assign31170_e28252_d_n5, assign31170_e28252_d_n9,) = {
    if (p.p52 != 0.0) {
        let assign31170_e28236: f64 = (0.001 / p.p53);
        let assign31170_e28238: f64 = (assign31170_e28236 * locals.var_fn382_calc_iq__vdsin);
        let assign31170_e28239: f64 = (assign31170_e28238).tanh();
        let assign31170_e28240: f64 = (locals.var_fn382_calc_iq__vdsin * assign31170_e28239);
        (assign31170_e28240, ((locals.var_fn382_calc_iq__vdsin_dn5 * assign31170_e28239) + (locals.var_fn382_calc_iq__vdsin * ((assign31170_e28236 * locals.var_fn382_calc_iq__vdsin_dn5) / ((assign31170_e28238).cosh() * (assign31170_e28238).cosh())))), ((locals.var_fn382_calc_iq__vdsin_dn9 * assign31170_e28239) + (locals.var_fn382_calc_iq__vdsin * ((assign31170_e28236 * locals.var_fn382_calc_iq__vdsin_dn9) / ((assign31170_e28238).cosh() * (assign31170_e28238).cosh())))),)
    } else {
        let (assign31170_e28251, assign31170_e28251_d_n5, assign31170_e28251_d_n9,) = {
            if (p.p52 == 0.0) {
                let assign31170_e28246: f64 = (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsin);
                let assign31170_e28248: f64 = (assign31170_e28246 + p.p53);
                let assign31170_e28249: f64 = (assign31170_e28248).sqrt();
                (assign31170_e28249, (((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsin) + (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsin_dn5)) / (2.0 * assign31170_e28249)), (((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsin) + (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsin_dn9)) / (2.0 * assign31170_e28249)),)
            } else {
                (0.0, 0.0, 0.0,)
            }
        };
        (assign31170_e28251, assign31170_e28251_d_n5, assign31170_e28251_d_n9,)
    }
};
        locals.var_fn382_calc_iq__absvdsin = assign31170_e28252;
        locals.var_fn382_calc_iq__absvdsin_dn5 = assign31170_e28252_d_n5;
        locals.var_fn382_calc_iq__absvdsin_dn9 = assign31170_e28252_d_n9;
        locals.var_fn382_calc_iq__absvdsin_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_68(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign31180_e28255: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vdsin);
        locals.var_fn382_calc_iq__vgdin = assign31180_e28255;
        locals.var_fn382_calc_iq__vgdin_dn5 = (-locals.var_fn382_calc_iq__vdsin_dn5);
        locals.var_fn382_calc_iq__vgdin_dn8 = locals.var_fn382_calc_iq__vgsin_dn8;
        locals.var_fn382_calc_iq__vgdin_dn9 = (locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vdsin_dn9);
        locals.var_fn382_calc_iq__vgdin_rv = 0.0;

        let assign31190_e28258: f64 = (locals.var_fn382_calc_iq__alpha * locals.var_fn382_calc_iq__phitin);
        locals.var_fn382_calc_iq__alpha_phit = assign31190_e28258;
        locals.var_fn382_calc_iq__alpha_phit_dn4 = (locals.var_fn382_calc_iq__alpha * locals.var_fn382_calc_iq__phitin_dn4);
        locals.var_fn382_calc_iq__alpha_phit_rv = 0.0;

        let assign31200_e28262: f64 = (2.302585092994046 * locals.var_fn382_calc_iq__phitin);
        let assign31200_e28263: f64 = (locals.var_fn382_calc_iq__ss / assign31200_e28262);
        let assign31200_e28266: f64 = (locals.var_fn382_calc_iq__nd * locals.var_fn382_calc_iq__absvdsin);
        let assign31200_e28267: f64 = (assign31200_e28263 + assign31200_e28266);
        locals.var_fn382_calc_iq__n = assign31200_e28267;
        locals.var_fn382_calc_iq__n_dn4 = (-((locals.var_fn382_calc_iq__ss * (2.302585092994046 * locals.var_fn382_calc_iq__phitin_dn4)) / (assign31200_e28262 * assign31200_e28262)));
        locals.var_fn382_calc_iq__n_dn5 = (locals.var_fn382_calc_iq__nd * locals.var_fn382_calc_iq__absvdsin_dn5);
        locals.var_fn382_calc_iq__n_dn9 = (locals.var_fn382_calc_iq__nd * locals.var_fn382_calc_iq__absvdsin_dn9);
        locals.var_fn382_calc_iq__n_rv = 0.0;

        let assign31210_e28272: f64 = (locals.var_fn382_calc_iq__tambin - locals.var_fn382_calc_iq__tnomin);
        let assign31210_e28273: f64 = (locals.var_fn382_calc_iq__vtzeta * assign31210_e28272);
        let assign31210_e28274: f64 = (locals.var_fn382_calc_iq__vto + assign31210_e28273);
        locals.var_fn382_calc_iq__vtof = assign31210_e28274;
        locals.var_fn382_calc_iq__vtof_dn4 = (locals.var_fn382_calc_iq__vtzeta * locals.var_fn382_calc_iq__tambin_dn4);
        locals.var_fn382_calc_iq__vtof_rv = 0.0;

        let assign31220_e28277: f64 = (locals.var_fn382_calc_iq__tambin / locals.var_fn382_calc_iq__tnomin);
        let assign31220_e28279: f64 = (assign31220_e28277).powf(locals.var_fn382_calc_iq__epsilon);
        locals.var_fn382_calc_iq__tfacmobin = assign31220_e28279;
        locals.var_fn382_calc_iq__tfacmobin_dn4 = if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__epsilon) as f64).is_finite() && ((locals.var_fn382_calc_iq__epsilon) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__epsilon == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__epsilon * ((assign31220_e28277).powf(locals.var_fn382_calc_iq__epsilon - 1.0) * (locals.var_fn382_calc_iq__tambin_dn4 / locals.var_fn382_calc_iq__tnomin))) } } else { (assign31220_e28279 * (locals.var_fn382_calc_iq__epsilon * ((locals.var_fn382_calc_iq__tambin_dn4 / locals.var_fn382_calc_iq__tnomin) / assign31220_e28277))) };
        locals.var_fn382_calc_iq__tfacmobin_rv = 0.0;

        let assign31230_e28282: f64 = if locals.var_fn382_calc_iq__dibsat != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard383 = assign31230_e28282;
        locals.var_guard383_rv = 0.0;

        let (assign31240_e28298, assign31240_e28298_d_n5, assign31240_e28298_d_n9,) = {
    if (locals.var_guard383 != 0.0) {
        let assign31240_e28288: f64 = (locals.var_fn382_calc_iq__absvdsin / locals.var_fn382_calc_iq__dibsat);
        let assign31240_e28290: f64 = (assign31240_e28288).powf(locals.var_fn382_calc_iq__beta);
        let assign31240_e28291: f64 = (1.0 + assign31240_e28290);
        let assign31240_e28294: f64 = (1.0 / locals.var_fn382_calc_iq__beta);
        let assign31240_e28295: f64 = (assign31240_e28291).powf(assign31240_e28294);
        let assign31240_e28296: f64 = (locals.var_fn382_calc_iq__absvdsin / assign31240_e28295);
        (assign31240_e28296, (((locals.var_fn382_calc_iq__absvdsin_dn5 * assign31240_e28295) - (locals.var_fn382_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign31240_e28294) as f64).is_finite() && ((assign31240_e28294) as f64).fract() == 0.0 { if assign31240_e28294 == 0.0 { 0.0 } else { (assign31240_e28294 * ((assign31240_e28291).powf(assign31240_e28294 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31240_e28288).powf(locals.var_fn382_calc_iq__beta - 1.0) * (locals.var_fn382_calc_iq__absvdsin_dn5 / locals.var_fn382_calc_iq__dibsat))) } } else { (assign31240_e28290 * (locals.var_fn382_calc_iq__beta * ((locals.var_fn382_calc_iq__absvdsin_dn5 / locals.var_fn382_calc_iq__dibsat) / assign31240_e28288))) })) } } else { (assign31240_e28295 * (assign31240_e28294 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31240_e28288).powf(locals.var_fn382_calc_iq__beta - 1.0) * (locals.var_fn382_calc_iq__absvdsin_dn5 / locals.var_fn382_calc_iq__dibsat))) } } else { (assign31240_e28290 * (locals.var_fn382_calc_iq__beta * ((locals.var_fn382_calc_iq__absvdsin_dn5 / locals.var_fn382_calc_iq__dibsat) / assign31240_e28288))) } / assign31240_e28291))) })) / (assign31240_e28295 * assign31240_e28295)), (((locals.var_fn382_calc_iq__absvdsin_dn9 * assign31240_e28295) - (locals.var_fn382_calc_iq__absvdsin * if 0.0 == 0.0 && ((assign31240_e28294) as f64).is_finite() && ((assign31240_e28294) as f64).fract() == 0.0 { if assign31240_e28294 == 0.0 { 0.0 } else { (assign31240_e28294 * ((assign31240_e28291).powf(assign31240_e28294 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31240_e28288).powf(locals.var_fn382_calc_iq__beta - 1.0) * (locals.var_fn382_calc_iq__absvdsin_dn9 / locals.var_fn382_calc_iq__dibsat))) } } else { (assign31240_e28290 * (locals.var_fn382_calc_iq__beta * ((locals.var_fn382_calc_iq__absvdsin_dn9 / locals.var_fn382_calc_iq__dibsat) / assign31240_e28288))) })) } } else { (assign31240_e28295 * (assign31240_e28294 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31240_e28288).powf(locals.var_fn382_calc_iq__beta - 1.0) * (locals.var_fn382_calc_iq__absvdsin_dn9 / locals.var_fn382_calc_iq__dibsat))) } } else { (assign31240_e28290 * (locals.var_fn382_calc_iq__beta * ((locals.var_fn382_calc_iq__absvdsin_dn9 / locals.var_fn382_calc_iq__dibsat) / assign31240_e28288))) } / assign31240_e28291))) })) / (assign31240_e28295 * assign31240_e28295)),)
    } else {
        (locals.var_fn382_calc_iq__vsatdibl, locals.var_fn382_calc_iq__vsatdibl_dn5, locals.var_fn382_calc_iq__vsatdibl_dn9,)
    }
};
        locals.var_fn382_calc_iq__vsatdibl = assign31240_e28298;
        locals.var_fn382_calc_iq__vsatdibl_dn5 = assign31240_e28298_d_n5;
        locals.var_fn382_calc_iq__vsatdibl_dn9 = assign31240_e28298_d_n9;
        locals.var_fn382_calc_iq__vsatdibl_rv = 0.0;

        let (assign31250_e28303, assign31250_e28303_d_n5, assign31250_e28303_d_n9,) = {
    if (locals.var_guard383 == 0.0) {
        (0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__vsatdibl, locals.var_fn382_calc_iq__vsatdibl_dn5, locals.var_fn382_calc_iq__vsatdibl_dn9,)
    }
};
        locals.var_fn382_calc_iq__vsatdibl = assign31250_e28303;
        locals.var_fn382_calc_iq__vsatdibl_dn5 = assign31250_e28303_d_n5;
        locals.var_fn382_calc_iq__vsatdibl_dn9 = assign31250_e28303_d_n9;
        locals.var_fn382_calc_iq__vsatdibl_rv = 0.0;

        let assign31260_e28307: f64 = (locals.var_fn382_calc_iq__vsatdibl * locals.var_fn382_calc_iq__delta2);
        let assign31260_e28308: f64 = (locals.var_fn382_calc_iq__delta1 - assign31260_e28307);
        let assign31260_e28310: f64 = (assign31260_e28308 * locals.var_fn382_calc_iq__absvdsin);
        locals.var_fn382_calc_iq__delta = assign31260_e28310;
        locals.var_fn382_calc_iq__delta_dn5 = (((-(locals.var_fn382_calc_iq__vsatdibl_dn5 * locals.var_fn382_calc_iq__delta2)) * locals.var_fn382_calc_iq__absvdsin) + (assign31260_e28308 * locals.var_fn382_calc_iq__absvdsin_dn5));
        locals.var_fn382_calc_iq__delta_dn9 = (((-(locals.var_fn382_calc_iq__vsatdibl_dn9 * locals.var_fn382_calc_iq__delta2)) * locals.var_fn382_calc_iq__absvdsin) + (assign31260_e28308 * locals.var_fn382_calc_iq__absvdsin_dn9));
        locals.var_fn382_calc_iq__delta_rv = 0.0;

        let assign31270_e28313: f64 = (locals.var_fn382_calc_iq__vtof - locals.var_fn382_calc_iq__delta);
        locals.var_fn382_calc_iq__vtdibl = assign31270_e28313;
        locals.var_fn382_calc_iq__vtdibl_dn4 = locals.var_fn382_calc_iq__vtof_dn4;
        locals.var_fn382_calc_iq__vtdibl_dn5 = (-locals.var_fn382_calc_iq__delta_dn5);
        locals.var_fn382_calc_iq__vtdibl_dn9 = (-locals.var_fn382_calc_iq__delta_dn9);
        locals.var_fn382_calc_iq__vtdibl_rv = 0.0;

        let assign31280_e28316: f64 = (2.0 * locals.var_fn382_calc_iq__n);
        let assign31280_e28318: f64 = (assign31280_e28316 * locals.var_fn382_calc_iq__phitin);
        locals.var_fn382_calc_iq__two_n_phit = assign31280_e28318;
        locals.var_fn382_calc_iq__two_n_phit_dn4 = (((2.0 * locals.var_fn382_calc_iq__n_dn4) * locals.var_fn382_calc_iq__phitin) + (assign31280_e28316 * locals.var_fn382_calc_iq__phitin_dn4));
        locals.var_fn382_calc_iq__two_n_phit_dn5 = ((2.0 * locals.var_fn382_calc_iq__n_dn5) * locals.var_fn382_calc_iq__phitin);
        locals.var_fn382_calc_iq__two_n_phit_dn9 = ((2.0 * locals.var_fn382_calc_iq__n_dn9) * locals.var_fn382_calc_iq__phitin);
        locals.var_fn382_calc_iq__two_n_phit_rv = 0.0;

        let assign31290_e28321: f64 = (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__two_n_phit);
        locals.var_fn382_calc_iq__qref = assign31290_e28321;
        locals.var_fn382_calc_iq__qref_dn4 = ((locals.var_fn382_calc_iq__cgin_dn4 * locals.var_fn382_calc_iq__two_n_phit) + (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__two_n_phit_dn4));
        locals.var_fn382_calc_iq__qref_dn5 = (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__two_n_phit_dn5);
        locals.var_fn382_calc_iq__qref_dn9 = (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__two_n_phit_dn9);
        locals.var_fn382_calc_iq__qref_rv = 0.0;

        let assign31300_e28325: f64 = (p.p51 * locals.var_fn382_calc_iq__alpha_phit);
        let assign31300_e28327: f64 = (assign31300_e28325 / 2.0);
        let assign31300_e28328: f64 = (locals.var_fn382_calc_iq__vtdibl - assign31300_e28327);
        locals.var_fn382_calc_iq__myarg = assign31300_e28328;
        locals.var_fn382_calc_iq__myarg_dn4 = (locals.var_fn382_calc_iq__vtdibl_dn4 - ((p.p51 * locals.var_fn382_calc_iq__alpha_phit_dn4) / 2.0));
        locals.var_fn382_calc_iq__myarg_dn5 = locals.var_fn382_calc_iq__vtdibl_dn5;
        locals.var_fn382_calc_iq__myarg_dn8 = 0.0;
        locals.var_fn382_calc_iq__myarg_dn9 = locals.var_fn382_calc_iq__vtdibl_dn9;
        locals.var_fn382_calc_iq__myarg_rv = 0.0;

        let (assign31310_e28372, assign31310_e28372_d_n5, assign31310_e28372_d_n8, assign31310_e28372_d_n9,) = {
    if (p.p52 != 0.0) {
        let assign31310_e28336: f64 = (locals.var_fn382_calc_iq__vgsin + locals.var_fn382_calc_iq__vgdin);
        let assign31310_e28339: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
        let assign31310_e28342: f64 = (0.001 / p.p53);
        let assign31310_e28345: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
        let assign31310_e28346: f64 = (assign31310_e28342 * assign31310_e28345);
        let assign31310_e28347: f64 = (assign31310_e28346).tanh();
        let assign31310_e28348: f64 = (assign31310_e28339 * assign31310_e28347);
        let assign31310_e28349: f64 = (assign31310_e28336 + assign31310_e28348);
        let assign31310_e28350: f64 = (0.5 * assign31310_e28349);
        (assign31310_e28350, (0.5 * (locals.var_fn382_calc_iq__vgdin_dn5 + (((-locals.var_fn382_calc_iq__vgdin_dn5) * assign31310_e28347) + (assign31310_e28339 * ((assign31310_e28342 * (-locals.var_fn382_calc_iq__vgdin_dn5)) / ((assign31310_e28346).cosh() * (assign31310_e28346).cosh())))))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn8 + locals.var_fn382_calc_iq__vgdin_dn8) + (((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8) * assign31310_e28347) + (assign31310_e28339 * ((assign31310_e28342 * (locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8)) / ((assign31310_e28346).cosh() * (assign31310_e28346).cosh())))))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn9 + locals.var_fn382_calc_iq__vgdin_dn9) + (((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9) * assign31310_e28347) + (assign31310_e28339 * ((assign31310_e28342 * (locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9)) / ((assign31310_e28346).cosh() * (assign31310_e28346).cosh())))))),)
    } else {
        let (assign31310_e28371, assign31310_e28371_d_n5, assign31310_e28371_d_n8, assign31310_e28371_d_n9,) = {
            if (p.p52 == 0.0) {
                let assign31310_e28357: f64 = (locals.var_fn382_calc_iq__vgsin + locals.var_fn382_calc_iq__vgdin);
                let assign31310_e28360: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
                let assign31310_e28363: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
                let assign31310_e28364: f64 = (assign31310_e28360 * assign31310_e28363);
                let assign31310_e28366: f64 = (assign31310_e28364 + p.p53);
                let assign31310_e28367: f64 = (assign31310_e28366).sqrt();
                let assign31310_e28368: f64 = (assign31310_e28357 + assign31310_e28367);
                let assign31310_e28369: f64 = (0.5 * assign31310_e28368);
                (assign31310_e28369, (0.5 * (locals.var_fn382_calc_iq__vgdin_dn5 + ((((-locals.var_fn382_calc_iq__vgdin_dn5) * assign31310_e28363) + (assign31310_e28360 * (-locals.var_fn382_calc_iq__vgdin_dn5))) / (2.0 * assign31310_e28367)))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn8 + locals.var_fn382_calc_iq__vgdin_dn8) + ((((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8) * assign31310_e28363) + (assign31310_e28360 * (locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8))) / (2.0 * assign31310_e28367)))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn9 + locals.var_fn382_calc_iq__vgdin_dn9) + ((((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9) * assign31310_e28363) + (assign31310_e28360 * (locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9))) / (2.0 * assign31310_e28367)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign31310_e28371, assign31310_e28371_d_n5, assign31310_e28371_d_n8, assign31310_e28371_d_n9,)
    }
};
        let assign31310_e28374: f64 = (assign31310_e28372 - locals.var_fn382_calc_iq__myarg);
        let assign31310_e28376: f64 = (assign31310_e28374 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg = assign31310_e28376;
        locals.var_fn382_calc_iq__exparg_dn4 = ((((-locals.var_fn382_calc_iq__myarg_dn4) * locals.var_fn382_calc_iq__alpha_phit) - (assign31310_e28374 * locals.var_fn382_calc_iq__alpha_phit_dn4)) / (locals.var_fn382_calc_iq__alpha_phit * locals.var_fn382_calc_iq__alpha_phit));
        locals.var_fn382_calc_iq__exparg_dn5 = ((assign31310_e28372_d_n5 - locals.var_fn382_calc_iq__myarg_dn5) / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg_dn8 = ((assign31310_e28372_d_n8 - locals.var_fn382_calc_iq__myarg_dn8) / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg_dn9 = ((assign31310_e28372_d_n9 - locals.var_fn382_calc_iq__myarg_dn9) / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg_rv = 0.0;

        let assign31320_e28379: f64 = if locals.var_fn382_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard384 = assign31320_e28379;
        locals.var_guard384_rv = 0.0;

        let (assign31330_e28383, assign31330_e28383_d_n4, assign31330_e28383_d_n5, assign31330_e28383_d_n8, assign31330_e28383_d_n9,) = {
    if (locals.var_guard384 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ff, locals.var_fn382_calc_iq__ff_dn4, locals.var_fn382_calc_iq__ff_dn5, locals.var_fn382_calc_iq__ff_dn8, locals.var_fn382_calc_iq__ff_dn9,)
    }
};
        locals.var_fn382_calc_iq__ff = assign31330_e28383;
        locals.var_fn382_calc_iq__ff_dn4 = assign31330_e28383_d_n4;
        locals.var_fn382_calc_iq__ff_dn5 = assign31330_e28383_d_n5;
        locals.var_fn382_calc_iq__ff_dn8 = assign31330_e28383_d_n8;
        locals.var_fn382_calc_iq__ff_dn9 = assign31330_e28383_d_n9;
        locals.var_fn382_calc_iq__ff_rv = 0.0;

        let assign31340_e28386: f64 = (-50.0);
        let assign31340_e28387: f64 = if locals.var_fn382_calc_iq__exparg < assign31340_e28386 { 1.0 } else { 0.0 };
        locals.var_guard385 = assign31340_e28387;
        locals.var_guard385_rv = 0.0;

        let (assign31350_e28394, assign31350_e28394_d_n4, assign31350_e28394_d_n5, assign31350_e28394_d_n8, assign31350_e28394_d_n9,) = {
    if ((locals.var_guard384 == 0.0) && (locals.var_guard385 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ff, locals.var_fn382_calc_iq__ff_dn4, locals.var_fn382_calc_iq__ff_dn5, locals.var_fn382_calc_iq__ff_dn8, locals.var_fn382_calc_iq__ff_dn9,)
    }
};
        locals.var_fn382_calc_iq__ff = assign31350_e28394;
        locals.var_fn382_calc_iq__ff_dn4 = assign31350_e28394_d_n4;
        locals.var_fn382_calc_iq__ff_dn5 = assign31350_e28394_d_n5;
        locals.var_fn382_calc_iq__ff_dn8 = assign31350_e28394_d_n8;
        locals.var_fn382_calc_iq__ff_dn9 = assign31350_e28394_d_n9;
        locals.var_fn382_calc_iq__ff_rv = 0.0;

        let (assign31360_e28407, assign31360_e28407_d_n4, assign31360_e28407_d_n5, assign31360_e28407_d_n8, assign31360_e28407_d_n9,) = {
    if ((locals.var_guard384 == 0.0) && (locals.var_guard385 == 0.0)) {
        let assign31360_e28403: f64 = (locals.var_fn382_calc_iq__exparg).exp();
        let assign31360_e28404: f64 = (1.0 + assign31360_e28403);
        let assign31360_e28405: f64 = (1.0 / assign31360_e28404);
        (assign31360_e28405, (-((assign31360_e28403 * locals.var_fn382_calc_iq__exparg_dn4) / (assign31360_e28404 * assign31360_e28404))), (-((assign31360_e28403 * locals.var_fn382_calc_iq__exparg_dn5) / (assign31360_e28404 * assign31360_e28404))), (-((assign31360_e28403 * locals.var_fn382_calc_iq__exparg_dn8) / (assign31360_e28404 * assign31360_e28404))), (-((assign31360_e28403 * locals.var_fn382_calc_iq__exparg_dn9) / (assign31360_e28404 * assign31360_e28404))),)
    } else {
        (locals.var_fn382_calc_iq__ff, locals.var_fn382_calc_iq__ff_dn4, locals.var_fn382_calc_iq__ff_dn5, locals.var_fn382_calc_iq__ff_dn8, locals.var_fn382_calc_iq__ff_dn9,)
    }
};
        locals.var_fn382_calc_iq__ff = assign31360_e28407;
        locals.var_fn382_calc_iq__ff_dn4 = assign31360_e28407_d_n4;
        locals.var_fn382_calc_iq__ff_dn5 = assign31360_e28407_d_n5;
        locals.var_fn382_calc_iq__ff_dn8 = assign31360_e28407_d_n8;
        locals.var_fn382_calc_iq__ff_dn9 = assign31360_e28407_d_n9;
        locals.var_fn382_calc_iq__ff_rv = 0.0;

        let (assign31370_e28451, assign31370_e28451_d_n5, assign31370_e28451_d_n8, assign31370_e28451_d_n9,) = {
    if (p.p52 != 0.0) {
        let assign31370_e28415: f64 = (locals.var_fn382_calc_iq__vgsin + locals.var_fn382_calc_iq__vgdin);
        let assign31370_e28418: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
        let assign31370_e28421: f64 = (0.001 / p.p53);
        let assign31370_e28424: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
        let assign31370_e28425: f64 = (assign31370_e28421 * assign31370_e28424);
        let assign31370_e28426: f64 = (assign31370_e28425).tanh();
        let assign31370_e28427: f64 = (assign31370_e28418 * assign31370_e28426);
        let assign31370_e28428: f64 = (assign31370_e28415 + assign31370_e28427);
        let assign31370_e28429: f64 = (0.5 * assign31370_e28428);
        (assign31370_e28429, (0.5 * (locals.var_fn382_calc_iq__vgdin_dn5 + (((-locals.var_fn382_calc_iq__vgdin_dn5) * assign31370_e28426) + (assign31370_e28418 * ((assign31370_e28421 * (-locals.var_fn382_calc_iq__vgdin_dn5)) / ((assign31370_e28425).cosh() * (assign31370_e28425).cosh())))))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn8 + locals.var_fn382_calc_iq__vgdin_dn8) + (((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8) * assign31370_e28426) + (assign31370_e28418 * ((assign31370_e28421 * (locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8)) / ((assign31370_e28425).cosh() * (assign31370_e28425).cosh())))))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn9 + locals.var_fn382_calc_iq__vgdin_dn9) + (((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9) * assign31370_e28426) + (assign31370_e28418 * ((assign31370_e28421 * (locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9)) / ((assign31370_e28425).cosh() * (assign31370_e28425).cosh())))))),)
    } else {
        let (assign31370_e28450, assign31370_e28450_d_n5, assign31370_e28450_d_n8, assign31370_e28450_d_n9,) = {
            if (p.p52 == 0.0) {
                let assign31370_e28436: f64 = (locals.var_fn382_calc_iq__vgsin + locals.var_fn382_calc_iq__vgdin);
                let assign31370_e28439: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
                let assign31370_e28442: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
                let assign31370_e28443: f64 = (assign31370_e28439 * assign31370_e28442);
                let assign31370_e28445: f64 = (assign31370_e28443 + p.p53);
                let assign31370_e28446: f64 = (assign31370_e28445).sqrt();
                let assign31370_e28447: f64 = (assign31370_e28436 + assign31370_e28446);
                let assign31370_e28448: f64 = (0.5 * assign31370_e28447);
                (assign31370_e28448, (0.5 * (locals.var_fn382_calc_iq__vgdin_dn5 + ((((-locals.var_fn382_calc_iq__vgdin_dn5) * assign31370_e28442) + (assign31370_e28439 * (-locals.var_fn382_calc_iq__vgdin_dn5))) / (2.0 * assign31370_e28446)))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn8 + locals.var_fn382_calc_iq__vgdin_dn8) + ((((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8) * assign31370_e28442) + (assign31370_e28439 * (locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8))) / (2.0 * assign31370_e28446)))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn9 + locals.var_fn382_calc_iq__vgdin_dn9) + ((((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9) * assign31370_e28442) + (assign31370_e28439 * (locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9))) / (2.0 * assign31370_e28446)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign31370_e28450, assign31370_e28450_d_n5, assign31370_e28450_d_n8, assign31370_e28450_d_n9,)
    }
};
        let assign31370_e28455: f64 = (p.p51 * 0.1);
        let assign31370_e28457: f64 = (assign31370_e28455 * locals.var_fn382_calc_iq__alpha_phit);
        let assign31370_e28459: f64 = (assign31370_e28457 * locals.var_fn382_calc_iq__ff);
        let assign31370_e28460: f64 = (locals.var_fn382_calc_iq__vtdibl - assign31370_e28459);
        let assign31370_e28461: f64 = (assign31370_e28451 - assign31370_e28460);
        let assign31370_e28463: f64 = (assign31370_e28461 / locals.var_fn382_calc_iq__two_n_phit);
        locals.var_fn382_calc_iq__eta = assign31370_e28463;
        locals.var_fn382_calc_iq__eta_dn4 = ((((-(locals.var_fn382_calc_iq__vtdibl_dn4 - (((assign31370_e28455 * locals.var_fn382_calc_iq__alpha_phit_dn4) * locals.var_fn382_calc_iq__ff) + (assign31370_e28457 * locals.var_fn382_calc_iq__ff_dn4)))) * locals.var_fn382_calc_iq__two_n_phit) - (assign31370_e28461 * locals.var_fn382_calc_iq__two_n_phit_dn4)) / (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__two_n_phit));
        locals.var_fn382_calc_iq__eta_dn5 = ((((assign31370_e28451_d_n5 - (locals.var_fn382_calc_iq__vtdibl_dn5 - (assign31370_e28457 * locals.var_fn382_calc_iq__ff_dn5))) * locals.var_fn382_calc_iq__two_n_phit) - (assign31370_e28461 * locals.var_fn382_calc_iq__two_n_phit_dn5)) / (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__two_n_phit));
        locals.var_fn382_calc_iq__eta_dn8 = ((assign31370_e28451_d_n8 - (-(assign31370_e28457 * locals.var_fn382_calc_iq__ff_dn8))) / locals.var_fn382_calc_iq__two_n_phit);
        locals.var_fn382_calc_iq__eta_dn9 = ((((assign31370_e28451_d_n9 - (locals.var_fn382_calc_iq__vtdibl_dn9 - (assign31370_e28457 * locals.var_fn382_calc_iq__ff_dn9))) * locals.var_fn382_calc_iq__two_n_phit) - (assign31370_e28461 * locals.var_fn382_calc_iq__two_n_phit_dn9)) / (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__two_n_phit));
        locals.var_fn382_calc_iq__eta_rv = 0.0;

        let assign31380_e28466: f64 = if locals.var_fn382_calc_iq__eta > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard386 = assign31380_e28466;
        locals.var_guard386_rv = 0.0;

        let (assign31390_e28472, assign31390_e28472_d_n4, assign31390_e28472_d_n5, assign31390_e28472_d_n8, assign31390_e28472_d_n9,) = {
    if (locals.var_guard386 != 0.0) {
        let assign31390_e28470: f64 = (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__eta);
        (assign31390_e28470, ((locals.var_fn382_calc_iq__qref_dn4 * locals.var_fn382_calc_iq__eta) + (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__eta_dn4)), ((locals.var_fn382_calc_iq__qref_dn5 * locals.var_fn382_calc_iq__eta) + (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__eta_dn5)), (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__eta_dn8), ((locals.var_fn382_calc_iq__qref_dn9 * locals.var_fn382_calc_iq__eta) + (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__eta_dn9)),)
    } else {
        (locals.var_fn382_calc_iq__qinvv, locals.var_fn382_calc_iq__qinvv_dn4, locals.var_fn382_calc_iq__qinvv_dn5, locals.var_fn382_calc_iq__qinvv_dn8, locals.var_fn382_calc_iq__qinvv_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvv = assign31390_e28472;
        locals.var_fn382_calc_iq__qinvv_dn4 = assign31390_e28472_d_n4;
        locals.var_fn382_calc_iq__qinvv_dn5 = assign31390_e28472_d_n5;
        locals.var_fn382_calc_iq__qinvv_dn8 = assign31390_e28472_d_n8;
        locals.var_fn382_calc_iq__qinvv_dn9 = assign31390_e28472_d_n9;
        locals.var_fn382_calc_iq__qinvv_rv = 0.0;

        let assign31400_e28475: f64 = (-50.0);
        let assign31400_e28476: f64 = if locals.var_fn382_calc_iq__eta < assign31400_e28475 { 1.0 } else { 0.0 };
        locals.var_guard387 = assign31400_e28476;
        locals.var_guard387_rv = 0.0;

        let (assign31410_e28486, assign31410_e28486_d_n4, assign31410_e28486_d_n5, assign31410_e28486_d_n8, assign31410_e28486_d_n9,) = {
    if ((locals.var_guard386 == 0.0) && (locals.var_guard387 != 0.0)) {
        let assign31410_e28483: f64 = (locals.var_fn382_calc_iq__eta).exp();
        let assign31410_e28484: f64 = (locals.var_fn382_calc_iq__qref * assign31410_e28483);
        (assign31410_e28484, ((locals.var_fn382_calc_iq__qref_dn4 * assign31410_e28483) + (locals.var_fn382_calc_iq__qref * (assign31410_e28483 * locals.var_fn382_calc_iq__eta_dn4))), ((locals.var_fn382_calc_iq__qref_dn5 * assign31410_e28483) + (locals.var_fn382_calc_iq__qref * (assign31410_e28483 * locals.var_fn382_calc_iq__eta_dn5))), (locals.var_fn382_calc_iq__qref * (assign31410_e28483 * locals.var_fn382_calc_iq__eta_dn8)), ((locals.var_fn382_calc_iq__qref_dn9 * assign31410_e28483) + (locals.var_fn382_calc_iq__qref * (assign31410_e28483 * locals.var_fn382_calc_iq__eta_dn9))),)
    } else {
        (locals.var_fn382_calc_iq__qinvv, locals.var_fn382_calc_iq__qinvv_dn4, locals.var_fn382_calc_iq__qinvv_dn5, locals.var_fn382_calc_iq__qinvv_dn8, locals.var_fn382_calc_iq__qinvv_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvv = assign31410_e28486;
        locals.var_fn382_calc_iq__qinvv_dn4 = assign31410_e28486_d_n4;
        locals.var_fn382_calc_iq__qinvv_dn5 = assign31410_e28486_d_n5;
        locals.var_fn382_calc_iq__qinvv_dn8 = assign31410_e28486_d_n8;
        locals.var_fn382_calc_iq__qinvv_dn9 = assign31410_e28486_d_n9;
        locals.var_fn382_calc_iq__qinvv_rv = 0.0;

        let (assign31420_e28500, assign31420_e28500_d_n4, assign31420_e28500_d_n5, assign31420_e28500_d_n8, assign31420_e28500_d_n9,) = {
    if ((locals.var_guard386 == 0.0) && (locals.var_guard387 == 0.0)) {
        let assign31420_e28495: f64 = (locals.var_fn382_calc_iq__eta).exp();
        let assign31420_e28496: f64 = (1.0 + assign31420_e28495);
        let assign31420_e28497: f64 = (assign31420_e28496).ln();
        let assign31420_e28498: f64 = (locals.var_fn382_calc_iq__qref * assign31420_e28497);
        (assign31420_e28498, ((locals.var_fn382_calc_iq__qref_dn4 * assign31420_e28497) + (locals.var_fn382_calc_iq__qref * ((assign31420_e28495 * locals.var_fn382_calc_iq__eta_dn4) / assign31420_e28496))), ((locals.var_fn382_calc_iq__qref_dn5 * assign31420_e28497) + (locals.var_fn382_calc_iq__qref * ((assign31420_e28495 * locals.var_fn382_calc_iq__eta_dn5) / assign31420_e28496))), (locals.var_fn382_calc_iq__qref * ((assign31420_e28495 * locals.var_fn382_calc_iq__eta_dn8) / assign31420_e28496)), ((locals.var_fn382_calc_iq__qref_dn9 * assign31420_e28497) + (locals.var_fn382_calc_iq__qref * ((assign31420_e28495 * locals.var_fn382_calc_iq__eta_dn9) / assign31420_e28496))),)
    } else {
        (locals.var_fn382_calc_iq__qinvv, locals.var_fn382_calc_iq__qinvv_dn4, locals.var_fn382_calc_iq__qinvv_dn5, locals.var_fn382_calc_iq__qinvv_dn8, locals.var_fn382_calc_iq__qinvv_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvv = assign31420_e28500;
        locals.var_fn382_calc_iq__qinvv_dn4 = assign31420_e28500_d_n4;
        locals.var_fn382_calc_iq__qinvv_dn5 = assign31420_e28500_d_n5;
        locals.var_fn382_calc_iq__qinvv_dn8 = assign31420_e28500_d_n8;
        locals.var_fn382_calc_iq__qinvv_dn9 = assign31420_e28500_d_n9;
        locals.var_fn382_calc_iq__qinvv_rv = 0.0;

        let assign31430_e28506: f64 = (locals.var_fn382_calc_iq__mtheta * locals.var_fn382_calc_iq__qinvv);
        let assign31430_e28508: f64 = (assign31430_e28506 / locals.var_fn382_calc_iq__cgin);
        let assign31430_e28509: f64 = (1.0 + assign31430_e28508);
        let assign31430_e28510: f64 = (locals.var_fn382_calc_iq__tfacmobin * assign31430_e28509);
        let assign31430_e28511: f64 = (locals.var_fn382_calc_iq__mu0 / assign31430_e28510);
        locals.var_fn382_calc_iq__muf = assign31430_e28511;
        locals.var_fn382_calc_iq__muf_dn4 = (-((locals.var_fn382_calc_iq__mu0 * ((locals.var_fn382_calc_iq__tfacmobin_dn4 * assign31430_e28509) + (locals.var_fn382_calc_iq__tfacmobin * ((((locals.var_fn382_calc_iq__mtheta * locals.var_fn382_calc_iq__qinvv_dn4) * locals.var_fn382_calc_iq__cgin) - (assign31430_e28506 * locals.var_fn382_calc_iq__cgin_dn4)) / (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__cgin))))) / (assign31430_e28510 * assign31430_e28510)));
        locals.var_fn382_calc_iq__muf_dn5 = (-((locals.var_fn382_calc_iq__mu0 * (locals.var_fn382_calc_iq__tfacmobin * ((locals.var_fn382_calc_iq__mtheta * locals.var_fn382_calc_iq__qinvv_dn5) / locals.var_fn382_calc_iq__cgin))) / (assign31430_e28510 * assign31430_e28510)));
        locals.var_fn382_calc_iq__muf_dn8 = (-((locals.var_fn382_calc_iq__mu0 * (locals.var_fn382_calc_iq__tfacmobin * ((locals.var_fn382_calc_iq__mtheta * locals.var_fn382_calc_iq__qinvv_dn8) / locals.var_fn382_calc_iq__cgin))) / (assign31430_e28510 * assign31430_e28510)));
        locals.var_fn382_calc_iq__muf_dn9 = (-((locals.var_fn382_calc_iq__mu0 * (locals.var_fn382_calc_iq__tfacmobin * ((locals.var_fn382_calc_iq__mtheta * locals.var_fn382_calc_iq__qinvv_dn9) / locals.var_fn382_calc_iq__cgin))) / (assign31430_e28510 * assign31430_e28510)));
        locals.var_fn382_calc_iq__muf_rv = 0.0;

        let assign31440_e28516: f64 = (locals.var_fn382_calc_iq__vzeta * locals.var_fn382_calc_iq__tnomin);
        let assign31440_e28517: f64 = (1.0 + assign31440_e28516);
        let assign31440_e28521: f64 = (locals.var_fn382_calc_iq__vzeta * locals.var_fn382_calc_iq__tambin);
        let assign31440_e28522: f64 = (1.0 + assign31440_e28521);
        let assign31440_e28523: f64 = (assign31440_e28517 / assign31440_e28522);
        let assign31440_e28524: f64 = (locals.var_fn382_calc_iq__vel0 * assign31440_e28523);
        let assign31440_e28528: f64 = (locals.var_fn382_calc_iq__lambda * locals.var_fn382_calc_iq__absvdsin);
        let assign31440_e28530: f64 = (assign31440_e28528 / locals.var_fn382_calc_iq__lin);
        let assign31440_e28531: f64 = (1.0 + assign31440_e28530);
        let assign31440_e28532: f64 = (assign31440_e28524 * assign31440_e28531);
        let assign31440_e28536: f64 = (locals.var_fn382_calc_iq__vtheta * locals.var_fn382_calc_iq__qinvv);
        let assign31440_e28538: f64 = (assign31440_e28536 / locals.var_fn382_calc_iq__cgin);
        let assign31440_e28539: f64 = (1.0 + assign31440_e28538);
        let assign31440_e28540: f64 = (assign31440_e28532 / assign31440_e28539);
        locals.var_fn382_calc_iq__vx = assign31440_e28540;
        locals.var_fn382_calc_iq__vx_dn4 = (((((locals.var_fn382_calc_iq__vel0 * (-((assign31440_e28517 * (locals.var_fn382_calc_iq__vzeta * locals.var_fn382_calc_iq__tambin_dn4)) / (assign31440_e28522 * assign31440_e28522)))) * assign31440_e28531) * assign31440_e28539) - (assign31440_e28532 * ((((locals.var_fn382_calc_iq__vtheta * locals.var_fn382_calc_iq__qinvv_dn4) * locals.var_fn382_calc_iq__cgin) - (assign31440_e28536 * locals.var_fn382_calc_iq__cgin_dn4)) / (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__cgin)))) / (assign31440_e28539 * assign31440_e28539));
        locals.var_fn382_calc_iq__vx_dn5 = ((((assign31440_e28524 * ((locals.var_fn382_calc_iq__lambda * locals.var_fn382_calc_iq__absvdsin_dn5) / locals.var_fn382_calc_iq__lin)) * assign31440_e28539) - (assign31440_e28532 * ((locals.var_fn382_calc_iq__vtheta * locals.var_fn382_calc_iq__qinvv_dn5) / locals.var_fn382_calc_iq__cgin))) / (assign31440_e28539 * assign31440_e28539));
        locals.var_fn382_calc_iq__vx_dn8 = (-((assign31440_e28532 * ((locals.var_fn382_calc_iq__vtheta * locals.var_fn382_calc_iq__qinvv_dn8) / locals.var_fn382_calc_iq__cgin)) / (assign31440_e28539 * assign31440_e28539)));
        locals.var_fn382_calc_iq__vx_dn9 = ((((assign31440_e28524 * ((locals.var_fn382_calc_iq__lambda * locals.var_fn382_calc_iq__absvdsin_dn9) / locals.var_fn382_calc_iq__lin)) * assign31440_e28539) - (assign31440_e28532 * ((locals.var_fn382_calc_iq__vtheta * locals.var_fn382_calc_iq__qinvv_dn9) / locals.var_fn382_calc_iq__cgin))) / (assign31440_e28539 * assign31440_e28539));
        locals.var_fn382_calc_iq__vx_rv = 0.0;

        let assign31450_e28543: f64 = (2.0 * locals.var_fn382_calc_iq__ff);
        let assign31450_e28545: f64 = (assign31450_e28543 * locals.var_fn382_calc_iq__phitin);
        let assign31450_e28547: f64 = (assign31450_e28545 * locals.var_fn382_calc_iq__muf);
        let assign31450_e28549: f64 = (assign31450_e28547 / locals.var_fn382_calc_iq__lin);
        let assign31450_e28552: f64 = (1.0 - locals.var_fn382_calc_iq__ff);
        let assign31450_e28554: f64 = (assign31450_e28552 * locals.var_fn382_calc_iq__vx);
        let assign31450_e28555: f64 = (assign31450_e28549 + assign31450_e28554);
        locals.var_fn382_calc_iq__vxf = assign31450_e28555;
        locals.var_fn382_calc_iq__vxf_dn4 = (((((((2.0 * locals.var_fn382_calc_iq__ff_dn4) * locals.var_fn382_calc_iq__phitin) + (assign31450_e28543 * locals.var_fn382_calc_iq__phitin_dn4)) * locals.var_fn382_calc_iq__muf) + (assign31450_e28545 * locals.var_fn382_calc_iq__muf_dn4)) / locals.var_fn382_calc_iq__lin) + (((-locals.var_fn382_calc_iq__ff_dn4) * locals.var_fn382_calc_iq__vx) + (assign31450_e28552 * locals.var_fn382_calc_iq__vx_dn4)));
        locals.var_fn382_calc_iq__vxf_dn5 = ((((((2.0 * locals.var_fn382_calc_iq__ff_dn5) * locals.var_fn382_calc_iq__phitin) * locals.var_fn382_calc_iq__muf) + (assign31450_e28545 * locals.var_fn382_calc_iq__muf_dn5)) / locals.var_fn382_calc_iq__lin) + (((-locals.var_fn382_calc_iq__ff_dn5) * locals.var_fn382_calc_iq__vx) + (assign31450_e28552 * locals.var_fn382_calc_iq__vx_dn5)));
        locals.var_fn382_calc_iq__vxf_dn8 = ((((((2.0 * locals.var_fn382_calc_iq__ff_dn8) * locals.var_fn382_calc_iq__phitin) * locals.var_fn382_calc_iq__muf) + (assign31450_e28545 * locals.var_fn382_calc_iq__muf_dn8)) / locals.var_fn382_calc_iq__lin) + (((-locals.var_fn382_calc_iq__ff_dn8) * locals.var_fn382_calc_iq__vx) + (assign31450_e28552 * locals.var_fn382_calc_iq__vx_dn8)));
        locals.var_fn382_calc_iq__vxf_dn9 = ((((((2.0 * locals.var_fn382_calc_iq__ff_dn9) * locals.var_fn382_calc_iq__phitin) * locals.var_fn382_calc_iq__muf) + (assign31450_e28545 * locals.var_fn382_calc_iq__muf_dn9)) / locals.var_fn382_calc_iq__lin) + (((-locals.var_fn382_calc_iq__ff_dn9) * locals.var_fn382_calc_iq__vx) + (assign31450_e28552 * locals.var_fn382_calc_iq__vx_dn9)));
        locals.var_fn382_calc_iq__vxf_rv = 0.0;

        let assign31460_e28558: f64 = (locals.var_fn382_calc_iq__vx * locals.var_fn382_calc_iq__lin);
        let assign31460_e28560: f64 = (assign31460_e28558 / locals.var_fn382_calc_iq__muf);
        locals.var_fn382_calc_iq__vdsats = assign31460_e28560;
        locals.var_fn382_calc_iq__vdsats_dn4 = ((((locals.var_fn382_calc_iq__vx_dn4 * locals.var_fn382_calc_iq__lin) * locals.var_fn382_calc_iq__muf) - (assign31460_e28558 * locals.var_fn382_calc_iq__muf_dn4)) / (locals.var_fn382_calc_iq__muf * locals.var_fn382_calc_iq__muf));
        locals.var_fn382_calc_iq__vdsats_dn5 = ((((locals.var_fn382_calc_iq__vx_dn5 * locals.var_fn382_calc_iq__lin) * locals.var_fn382_calc_iq__muf) - (assign31460_e28558 * locals.var_fn382_calc_iq__muf_dn5)) / (locals.var_fn382_calc_iq__muf * locals.var_fn382_calc_iq__muf));
        locals.var_fn382_calc_iq__vdsats_dn8 = ((((locals.var_fn382_calc_iq__vx_dn8 * locals.var_fn382_calc_iq__lin) * locals.var_fn382_calc_iq__muf) - (assign31460_e28558 * locals.var_fn382_calc_iq__muf_dn8)) / (locals.var_fn382_calc_iq__muf * locals.var_fn382_calc_iq__muf));
        locals.var_fn382_calc_iq__vdsats_dn9 = ((((locals.var_fn382_calc_iq__vx_dn9 * locals.var_fn382_calc_iq__lin) * locals.var_fn382_calc_iq__muf) - (assign31460_e28558 * locals.var_fn382_calc_iq__muf_dn9)) / (locals.var_fn382_calc_iq__muf * locals.var_fn382_calc_iq__muf));
        locals.var_fn382_calc_iq__vdsats_rv = 0.0;

        let assign31470_e28565: f64 = (2.0 * locals.var_fn382_calc_iq__qinvv);
        let assign31470_e28567: f64 = (assign31470_e28565 / locals.var_fn382_calc_iq__cgin);
        let assign31470_e28569: f64 = (assign31470_e28567 / locals.var_fn382_calc_iq__vdsats);
        let assign31470_e28570: f64 = (1.0 + assign31470_e28569);
        let assign31470_e28571: f64 = (assign31470_e28570).sqrt();
        let assign31470_e28572: f64 = (locals.var_fn382_calc_iq__vdsats * assign31470_e28571);
        let assign31470_e28574: f64 = (assign31470_e28572 - locals.var_fn382_calc_iq__vdsats);
        locals.var_fn382_calc_iq__vdsats1 = assign31470_e28574;
        locals.var_fn382_calc_iq__vdsats1_dn4 = (((locals.var_fn382_calc_iq__vdsats_dn4 * assign31470_e28571) + (locals.var_fn382_calc_iq__vdsats * ((((((((2.0 * locals.var_fn382_calc_iq__qinvv_dn4) * locals.var_fn382_calc_iq__cgin) - (assign31470_e28565 * locals.var_fn382_calc_iq__cgin_dn4)) / (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__cgin)) * locals.var_fn382_calc_iq__vdsats) - (assign31470_e28567 * locals.var_fn382_calc_iq__vdsats_dn4)) / (locals.var_fn382_calc_iq__vdsats * locals.var_fn382_calc_iq__vdsats)) / (2.0 * assign31470_e28571)))) - locals.var_fn382_calc_iq__vdsats_dn4);
        locals.var_fn382_calc_iq__vdsats1_dn5 = (((locals.var_fn382_calc_iq__vdsats_dn5 * assign31470_e28571) + (locals.var_fn382_calc_iq__vdsats * ((((((2.0 * locals.var_fn382_calc_iq__qinvv_dn5) / locals.var_fn382_calc_iq__cgin) * locals.var_fn382_calc_iq__vdsats) - (assign31470_e28567 * locals.var_fn382_calc_iq__vdsats_dn5)) / (locals.var_fn382_calc_iq__vdsats * locals.var_fn382_calc_iq__vdsats)) / (2.0 * assign31470_e28571)))) - locals.var_fn382_calc_iq__vdsats_dn5);
        locals.var_fn382_calc_iq__vdsats1_dn8 = (((locals.var_fn382_calc_iq__vdsats_dn8 * assign31470_e28571) + (locals.var_fn382_calc_iq__vdsats * ((((((2.0 * locals.var_fn382_calc_iq__qinvv_dn8) / locals.var_fn382_calc_iq__cgin) * locals.var_fn382_calc_iq__vdsats) - (assign31470_e28567 * locals.var_fn382_calc_iq__vdsats_dn8)) / (locals.var_fn382_calc_iq__vdsats * locals.var_fn382_calc_iq__vdsats)) / (2.0 * assign31470_e28571)))) - locals.var_fn382_calc_iq__vdsats_dn8);
        locals.var_fn382_calc_iq__vdsats1_dn9 = (((locals.var_fn382_calc_iq__vdsats_dn9 * assign31470_e28571) + (locals.var_fn382_calc_iq__vdsats * ((((((2.0 * locals.var_fn382_calc_iq__qinvv_dn9) / locals.var_fn382_calc_iq__cgin) * locals.var_fn382_calc_iq__vdsats) - (assign31470_e28567 * locals.var_fn382_calc_iq__vdsats_dn9)) / (locals.var_fn382_calc_iq__vdsats * locals.var_fn382_calc_iq__vdsats)) / (2.0 * assign31470_e28571)))) - locals.var_fn382_calc_iq__vdsats_dn9);
        locals.var_fn382_calc_iq__vdsats1_rv = 0.0;

        let assign31480_e28578: f64 = (1.0 - locals.var_fn382_calc_iq__ff);
        let assign31480_e28579: f64 = (locals.var_fn382_calc_iq__vdsats * assign31480_e28578);
        let assign31480_e28582: f64 = (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__ff);
        let assign31480_e28583: f64 = (assign31480_e28579 + assign31480_e28582);
        locals.var_fn382_calc_iq__vdsat = assign31480_e28583;
        locals.var_fn382_calc_iq__vdsat_dn4 = (((locals.var_fn382_calc_iq__vdsats_dn4 * assign31480_e28578) + (locals.var_fn382_calc_iq__vdsats * (-locals.var_fn382_calc_iq__ff_dn4))) + ((locals.var_fn382_calc_iq__two_n_phit_dn4 * locals.var_fn382_calc_iq__ff) + (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__ff_dn4)));
        locals.var_fn382_calc_iq__vdsat_dn5 = (((locals.var_fn382_calc_iq__vdsats_dn5 * assign31480_e28578) + (locals.var_fn382_calc_iq__vdsats * (-locals.var_fn382_calc_iq__ff_dn5))) + ((locals.var_fn382_calc_iq__two_n_phit_dn5 * locals.var_fn382_calc_iq__ff) + (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__ff_dn5)));
        locals.var_fn382_calc_iq__vdsat_dn8 = (((locals.var_fn382_calc_iq__vdsats_dn8 * assign31480_e28578) + (locals.var_fn382_calc_iq__vdsats * (-locals.var_fn382_calc_iq__ff_dn8))) + (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__ff_dn8));
        locals.var_fn382_calc_iq__vdsat_dn9 = (((locals.var_fn382_calc_iq__vdsats_dn9 * assign31480_e28578) + (locals.var_fn382_calc_iq__vdsats * (-locals.var_fn382_calc_iq__ff_dn9))) + ((locals.var_fn382_calc_iq__two_n_phit_dn9 * locals.var_fn382_calc_iq__ff) + (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__ff_dn9)));
        locals.var_fn382_calc_iq__vdsat_rv = 0.0;

        let assign31490_e28587: f64 = (1.0 - locals.var_fn382_calc_iq__ff);
        let assign31490_e28588: f64 = (locals.var_fn382_calc_iq__vdsats1 * assign31490_e28587);
        let assign31490_e28591: f64 = (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__ff);
        let assign31490_e28592: f64 = (assign31490_e28588 + assign31490_e28591);
        locals.var_fn382_calc_iq__vdsat1 = assign31490_e28592;
        locals.var_fn382_calc_iq__vdsat1_dn4 = (((locals.var_fn382_calc_iq__vdsats1_dn4 * assign31490_e28587) + (locals.var_fn382_calc_iq__vdsats1 * (-locals.var_fn382_calc_iq__ff_dn4))) + ((locals.var_fn382_calc_iq__two_n_phit_dn4 * locals.var_fn382_calc_iq__ff) + (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__ff_dn4)));
        locals.var_fn382_calc_iq__vdsat1_dn5 = (((locals.var_fn382_calc_iq__vdsats1_dn5 * assign31490_e28587) + (locals.var_fn382_calc_iq__vdsats1 * (-locals.var_fn382_calc_iq__ff_dn5))) + ((locals.var_fn382_calc_iq__two_n_phit_dn5 * locals.var_fn382_calc_iq__ff) + (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__ff_dn5)));
        locals.var_fn382_calc_iq__vdsat1_dn8 = (((locals.var_fn382_calc_iq__vdsats1_dn8 * assign31490_e28587) + (locals.var_fn382_calc_iq__vdsats1 * (-locals.var_fn382_calc_iq__ff_dn8))) + (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__ff_dn8));
        locals.var_fn382_calc_iq__vdsat1_dn9 = (((locals.var_fn382_calc_iq__vdsats1_dn9 * assign31490_e28587) + (locals.var_fn382_calc_iq__vdsats1 * (-locals.var_fn382_calc_iq__ff_dn9))) + ((locals.var_fn382_calc_iq__two_n_phit_dn9 * locals.var_fn382_calc_iq__ff) + (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__ff_dn9)));
        locals.var_fn382_calc_iq__vdsat1_rv = 0.0;

        let (assign31500_e28650, assign31500_e28650_d_n4, assign31500_e28650_d_n5, assign31500_e28650_d_n8, assign31500_e28650_d_n9,) = {
    if (p.p52 != 0.0) {
        let assign31500_e28603: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat1);
        let assign31500_e28604: f64 = assign31500_e28603;
        let assign31500_e28608: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat1);
        let assign31500_e28609: f64 = (-assign31500_e28608);
        let assign31500_e28612: f64 = (0.001 / p.p53);
        let assign31500_e28616: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat1);
        let assign31500_e28617: f64 = (-assign31500_e28616);
        let assign31500_e28618: f64 = (assign31500_e28612 * assign31500_e28617);
        let assign31500_e28619: f64 = (assign31500_e28618).tanh();
        let assign31500_e28620: f64 = (assign31500_e28609 * assign31500_e28619);
        let assign31500_e28621: f64 = (assign31500_e28604 + assign31500_e28620);
        let assign31500_e28622: f64 = (0.5 * assign31500_e28621);
        (assign31500_e28622, (0.5 * ((-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) + (((-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) * assign31500_e28619) + (assign31500_e28609 * ((assign31500_e28612 * (-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))))) / ((assign31500_e28618).cosh() * (assign31500_e28618).cosh())))))), (0.5 * ((((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)) + (((-(((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) * assign31500_e28619) + (assign31500_e28609 * ((assign31500_e28612 * (-(((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) / ((assign31500_e28618).cosh() * (assign31500_e28618).cosh())))))), (0.5 * ((-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) + (((-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) * assign31500_e28619) + (assign31500_e28609 * ((assign31500_e28612 * (-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))))) / ((assign31500_e28618).cosh() * (assign31500_e28618).cosh())))))), (0.5 * ((((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)) + (((-(((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) * assign31500_e28619) + (assign31500_e28609 * ((assign31500_e28612 * (-(((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) / ((assign31500_e28618).cosh() * (assign31500_e28618).cosh())))))),)
    } else {
        let (assign31500_e28649, assign31500_e28649_d_n4, assign31500_e28649_d_n5, assign31500_e28649_d_n8, assign31500_e28649_d_n9,) = {
            if (p.p52 == 0.0) {
                let assign31500_e28630: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat1);
                let assign31500_e28631: f64 = assign31500_e28630;
                let assign31500_e28635: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat1);
                let assign31500_e28636: f64 = (-assign31500_e28635);
                let assign31500_e28640: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat1);
                let assign31500_e28641: f64 = (-assign31500_e28640);
                let assign31500_e28642: f64 = (assign31500_e28636 * assign31500_e28641);
                let assign31500_e28644: f64 = (assign31500_e28642 + p.p53);
                let assign31500_e28645: f64 = (assign31500_e28644).sqrt();
                let assign31500_e28646: f64 = (assign31500_e28631 + assign31500_e28645);
                let assign31500_e28647: f64 = (0.5 * assign31500_e28646);
                (assign31500_e28647, (0.5 * ((-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) + ((((-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) * assign31500_e28641) + (assign31500_e28636 * (-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))))) / (2.0 * assign31500_e28645)))), (0.5 * ((((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)) + ((((-(((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) * assign31500_e28641) + (assign31500_e28636 * (-(((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))))) / (2.0 * assign31500_e28645)))), (0.5 * ((-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) + ((((-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) * assign31500_e28641) + (assign31500_e28636 * (-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))))) / (2.0 * assign31500_e28645)))), (0.5 * ((((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)) + ((((-(((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) * assign31500_e28641) + (assign31500_e28636 * (-(((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat1) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))))) / (2.0 * assign31500_e28645)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign31500_e28649, assign31500_e28649_d_n4, assign31500_e28649_d_n5, assign31500_e28649_d_n8, assign31500_e28649_d_n9,)
    }
};
        let assign31500_e28652: f64 = (assign31500_e28650).powf(locals.var_fn382_calc_iq__beta);
        let assign31500_e28653: f64 = (1.0 + assign31500_e28652);
        let assign31500_e28656: f64 = (1.0 / locals.var_fn382_calc_iq__beta);
        let assign31500_e28657: f64 = (assign31500_e28653).powf(assign31500_e28656);
        let assign31500_e28658: f64 = (1.0 / assign31500_e28657);
        locals.var_fn382_calc_iq__fsd = assign31500_e28658;
        locals.var_fn382_calc_iq__fsd_dn4 = (-(if 0.0 == 0.0 && ((assign31500_e28656) as f64).is_finite() && ((assign31500_e28656) as f64).fract() == 0.0 { if assign31500_e28656 == 0.0 { 0.0 } else { (assign31500_e28656 * ((assign31500_e28653).powf(assign31500_e28656 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31500_e28650).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31500_e28650_d_n4)) } } else { (assign31500_e28652 * (locals.var_fn382_calc_iq__beta * (assign31500_e28650_d_n4 / assign31500_e28650))) })) } } else { (assign31500_e28657 * (assign31500_e28656 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31500_e28650).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31500_e28650_d_n4)) } } else { (assign31500_e28652 * (locals.var_fn382_calc_iq__beta * (assign31500_e28650_d_n4 / assign31500_e28650))) } / assign31500_e28653))) } / (assign31500_e28657 * assign31500_e28657)));
        locals.var_fn382_calc_iq__fsd_dn5 = (-(if 0.0 == 0.0 && ((assign31500_e28656) as f64).is_finite() && ((assign31500_e28656) as f64).fract() == 0.0 { if assign31500_e28656 == 0.0 { 0.0 } else { (assign31500_e28656 * ((assign31500_e28653).powf(assign31500_e28656 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31500_e28650).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31500_e28650_d_n5)) } } else { (assign31500_e28652 * (locals.var_fn382_calc_iq__beta * (assign31500_e28650_d_n5 / assign31500_e28650))) })) } } else { (assign31500_e28657 * (assign31500_e28656 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31500_e28650).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31500_e28650_d_n5)) } } else { (assign31500_e28652 * (locals.var_fn382_calc_iq__beta * (assign31500_e28650_d_n5 / assign31500_e28650))) } / assign31500_e28653))) } / (assign31500_e28657 * assign31500_e28657)));
        locals.var_fn382_calc_iq__fsd_dn8 = (-(if 0.0 == 0.0 && ((assign31500_e28656) as f64).is_finite() && ((assign31500_e28656) as f64).fract() == 0.0 { if assign31500_e28656 == 0.0 { 0.0 } else { (assign31500_e28656 * ((assign31500_e28653).powf(assign31500_e28656 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31500_e28650).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31500_e28650_d_n8)) } } else { (assign31500_e28652 * (locals.var_fn382_calc_iq__beta * (assign31500_e28650_d_n8 / assign31500_e28650))) })) } } else { (assign31500_e28657 * (assign31500_e28656 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31500_e28650).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31500_e28650_d_n8)) } } else { (assign31500_e28652 * (locals.var_fn382_calc_iq__beta * (assign31500_e28650_d_n8 / assign31500_e28650))) } / assign31500_e28653))) } / (assign31500_e28657 * assign31500_e28657)));
        locals.var_fn382_calc_iq__fsd_dn9 = (-(if 0.0 == 0.0 && ((assign31500_e28656) as f64).is_finite() && ((assign31500_e28656) as f64).fract() == 0.0 { if assign31500_e28656 == 0.0 { 0.0 } else { (assign31500_e28656 * ((assign31500_e28653).powf(assign31500_e28656 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31500_e28650).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31500_e28650_d_n9)) } } else { (assign31500_e28652 * (locals.var_fn382_calc_iq__beta * (assign31500_e28650_d_n9 / assign31500_e28650))) })) } } else { (assign31500_e28657 * (assign31500_e28656 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31500_e28650).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31500_e28650_d_n9)) } } else { (assign31500_e28652 * (locals.var_fn382_calc_iq__beta * (assign31500_e28650_d_n9 / assign31500_e28650))) } / assign31500_e28653))) } / (assign31500_e28657 * assign31500_e28657)));
        locals.var_fn382_calc_iq__fsd_rv = 0.0;

        let assign31510_e28661: f64 = (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__fsd);
        locals.var_fn382_calc_iq__vdx = assign31510_e28661;
        locals.var_fn382_calc_iq__vdx_dn4 = (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__fsd_dn4);
        locals.var_fn382_calc_iq__vdx_dn5 = ((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__fsd) + (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__fsd_dn5));
        locals.var_fn382_calc_iq__vdx_dn8 = (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__fsd_dn8);
        locals.var_fn382_calc_iq__vdx_dn9 = ((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__fsd) + (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__fsd_dn9));
        locals.var_fn382_calc_iq__vdx_rv = 0.0;

        let (assign31520_e28725, assign31520_e28725_d_n4, assign31520_e28725_d_n5, assign31520_e28725_d_n8, assign31520_e28725_d_n9,) = {
    if (p.p52 != 0.0) {
        let assign31520_e28671: f64 = (-locals.var_fn382_calc_iq__vdsin);
        let assign31520_e28673: f64 = (assign31520_e28671 / locals.var_fn382_calc_iq__vdsat1);
        let assign31520_e28674: f64 = assign31520_e28673;
        let assign31520_e28677: f64 = (-locals.var_fn382_calc_iq__vdsin);
        let assign31520_e28679: f64 = (assign31520_e28677 / locals.var_fn382_calc_iq__vdsat1);
        let assign31520_e28680: f64 = (-assign31520_e28679);
        let assign31520_e28683: f64 = (0.001 / p.p53);
        let assign31520_e28686: f64 = (-locals.var_fn382_calc_iq__vdsin);
        let assign31520_e28688: f64 = (assign31520_e28686 / locals.var_fn382_calc_iq__vdsat1);
        let assign31520_e28689: f64 = (-assign31520_e28688);
        let assign31520_e28690: f64 = (assign31520_e28683 * assign31520_e28689);
        let assign31520_e28691: f64 = (assign31520_e28690).tanh();
        let assign31520_e28692: f64 = (assign31520_e28680 * assign31520_e28691);
        let assign31520_e28693: f64 = (assign31520_e28674 + assign31520_e28692);
        let assign31520_e28694: f64 = (0.5 * assign31520_e28693);
        (assign31520_e28694, (0.5 * ((-((assign31520_e28671 * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) + (((-(-((assign31520_e28677 * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) * assign31520_e28691) + (assign31520_e28680 * ((assign31520_e28683 * (-(-((assign31520_e28686 * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))))) / ((assign31520_e28690).cosh() * (assign31520_e28690).cosh())))))), (0.5 * (((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28671 * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)) + (((-((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28677 * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) * assign31520_e28691) + (assign31520_e28680 * ((assign31520_e28683 * (-((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28686 * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) / ((assign31520_e28690).cosh() * (assign31520_e28690).cosh())))))), (0.5 * ((-((assign31520_e28671 * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) + (((-(-((assign31520_e28677 * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) * assign31520_e28691) + (assign31520_e28680 * ((assign31520_e28683 * (-(-((assign31520_e28686 * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))))) / ((assign31520_e28690).cosh() * (assign31520_e28690).cosh())))))), (0.5 * (((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28671 * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)) + (((-((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28677 * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) * assign31520_e28691) + (assign31520_e28680 * ((assign31520_e28683 * (-((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28686 * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) / ((assign31520_e28690).cosh() * (assign31520_e28690).cosh())))))),)
    } else {
        let (assign31520_e28724, assign31520_e28724_d_n4, assign31520_e28724_d_n5, assign31520_e28724_d_n8, assign31520_e28724_d_n9,) = {
            if (p.p52 == 0.0) {
                let assign31520_e28701: f64 = (-locals.var_fn382_calc_iq__vdsin);
                let assign31520_e28703: f64 = (assign31520_e28701 / locals.var_fn382_calc_iq__vdsat1);
                let assign31520_e28704: f64 = assign31520_e28703;
                let assign31520_e28707: f64 = (-locals.var_fn382_calc_iq__vdsin);
                let assign31520_e28709: f64 = (assign31520_e28707 / locals.var_fn382_calc_iq__vdsat1);
                let assign31520_e28710: f64 = (-assign31520_e28709);
                let assign31520_e28713: f64 = (-locals.var_fn382_calc_iq__vdsin);
                let assign31520_e28715: f64 = (assign31520_e28713 / locals.var_fn382_calc_iq__vdsat1);
                let assign31520_e28716: f64 = (-assign31520_e28715);
                let assign31520_e28717: f64 = (assign31520_e28710 * assign31520_e28716);
                let assign31520_e28719: f64 = (assign31520_e28717 + p.p53);
                let assign31520_e28720: f64 = (assign31520_e28719).sqrt();
                let assign31520_e28721: f64 = (assign31520_e28704 + assign31520_e28720);
                let assign31520_e28722: f64 = (0.5 * assign31520_e28721);
                (assign31520_e28722, (0.5 * ((-((assign31520_e28701 * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) + ((((-(-((assign31520_e28707 * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) * assign31520_e28716) + (assign31520_e28710 * (-(-((assign31520_e28713 * locals.var_fn382_calc_iq__vdsat1_dn4) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))))) / (2.0 * assign31520_e28720)))), (0.5 * (((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28701 * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)) + ((((-((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28707 * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) * assign31520_e28716) + (assign31520_e28710 * (-((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28713 * locals.var_fn382_calc_iq__vdsat1_dn5)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))))) / (2.0 * assign31520_e28720)))), (0.5 * ((-((assign31520_e28701 * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) + ((((-(-((assign31520_e28707 * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))) * assign31520_e28716) + (assign31520_e28710 * (-(-((assign31520_e28713 * locals.var_fn382_calc_iq__vdsat1_dn8) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)))))) / (2.0 * assign31520_e28720)))), (0.5 * (((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28701 * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1)) + ((((-((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28707 * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))) * assign31520_e28716) + (assign31520_e28710 * (-((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat1) - (assign31520_e28713 * locals.var_fn382_calc_iq__vdsat1_dn9)) / (locals.var_fn382_calc_iq__vdsat1 * locals.var_fn382_calc_iq__vdsat1))))) / (2.0 * assign31520_e28720)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign31520_e28724, assign31520_e28724_d_n4, assign31520_e28724_d_n5, assign31520_e28724_d_n8, assign31520_e28724_d_n9,)
    }
};
        let assign31520_e28727: f64 = (assign31520_e28725).powf(locals.var_fn382_calc_iq__beta);
        let assign31520_e28728: f64 = (1.0 + assign31520_e28727);
        let assign31520_e28731: f64 = (1.0 / locals.var_fn382_calc_iq__beta);
        let assign31520_e28732: f64 = (assign31520_e28728).powf(assign31520_e28731);
        let assign31520_e28733: f64 = (1.0 / assign31520_e28732);
        locals.var_fn382_calc_iq__fds = assign31520_e28733;
        locals.var_fn382_calc_iq__fds_dn4 = (-(if 0.0 == 0.0 && ((assign31520_e28731) as f64).is_finite() && ((assign31520_e28731) as f64).fract() == 0.0 { if assign31520_e28731 == 0.0 { 0.0 } else { (assign31520_e28731 * ((assign31520_e28728).powf(assign31520_e28731 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31520_e28725).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31520_e28725_d_n4)) } } else { (assign31520_e28727 * (locals.var_fn382_calc_iq__beta * (assign31520_e28725_d_n4 / assign31520_e28725))) })) } } else { (assign31520_e28732 * (assign31520_e28731 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31520_e28725).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31520_e28725_d_n4)) } } else { (assign31520_e28727 * (locals.var_fn382_calc_iq__beta * (assign31520_e28725_d_n4 / assign31520_e28725))) } / assign31520_e28728))) } / (assign31520_e28732 * assign31520_e28732)));
        locals.var_fn382_calc_iq__fds_dn5 = (-(if 0.0 == 0.0 && ((assign31520_e28731) as f64).is_finite() && ((assign31520_e28731) as f64).fract() == 0.0 { if assign31520_e28731 == 0.0 { 0.0 } else { (assign31520_e28731 * ((assign31520_e28728).powf(assign31520_e28731 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31520_e28725).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31520_e28725_d_n5)) } } else { (assign31520_e28727 * (locals.var_fn382_calc_iq__beta * (assign31520_e28725_d_n5 / assign31520_e28725))) })) } } else { (assign31520_e28732 * (assign31520_e28731 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31520_e28725).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31520_e28725_d_n5)) } } else { (assign31520_e28727 * (locals.var_fn382_calc_iq__beta * (assign31520_e28725_d_n5 / assign31520_e28725))) } / assign31520_e28728))) } / (assign31520_e28732 * assign31520_e28732)));
        locals.var_fn382_calc_iq__fds_dn8 = (-(if 0.0 == 0.0 && ((assign31520_e28731) as f64).is_finite() && ((assign31520_e28731) as f64).fract() == 0.0 { if assign31520_e28731 == 0.0 { 0.0 } else { (assign31520_e28731 * ((assign31520_e28728).powf(assign31520_e28731 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31520_e28725).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31520_e28725_d_n8)) } } else { (assign31520_e28727 * (locals.var_fn382_calc_iq__beta * (assign31520_e28725_d_n8 / assign31520_e28725))) })) } } else { (assign31520_e28732 * (assign31520_e28731 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31520_e28725).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31520_e28725_d_n8)) } } else { (assign31520_e28727 * (locals.var_fn382_calc_iq__beta * (assign31520_e28725_d_n8 / assign31520_e28725))) } / assign31520_e28728))) } / (assign31520_e28732 * assign31520_e28732)));
        locals.var_fn382_calc_iq__fds_dn9 = (-(if 0.0 == 0.0 && ((assign31520_e28731) as f64).is_finite() && ((assign31520_e28731) as f64).fract() == 0.0 { if assign31520_e28731 == 0.0 { 0.0 } else { (assign31520_e28731 * ((assign31520_e28728).powf(assign31520_e28731 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31520_e28725).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31520_e28725_d_n9)) } } else { (assign31520_e28727 * (locals.var_fn382_calc_iq__beta * (assign31520_e28725_d_n9 / assign31520_e28725))) })) } } else { (assign31520_e28732 * (assign31520_e28731 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31520_e28725).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31520_e28725_d_n9)) } } else { (assign31520_e28727 * (locals.var_fn382_calc_iq__beta * (assign31520_e28725_d_n9 / assign31520_e28725))) } / assign31520_e28728))) } / (assign31520_e28732 * assign31520_e28732)));
        locals.var_fn382_calc_iq__fds_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_69(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign31530_e28735: f64 = (-locals.var_fn382_calc_iq__vdsin);
        let assign31530_e28737: f64 = (assign31530_e28735 * locals.var_fn382_calc_iq__fds);
        locals.var_fn382_calc_iq__vsx = assign31530_e28737;
        locals.var_fn382_calc_iq__vsx_dn4 = (assign31530_e28735 * locals.var_fn382_calc_iq__fds_dn4);
        locals.var_fn382_calc_iq__vsx_dn5 = (((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__fds) + (assign31530_e28735 * locals.var_fn382_calc_iq__fds_dn5));
        locals.var_fn382_calc_iq__vsx_dn8 = (assign31530_e28735 * locals.var_fn382_calc_iq__fds_dn8);
        locals.var_fn382_calc_iq__vsx_dn9 = (((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__fds) + (assign31530_e28735 * locals.var_fn382_calc_iq__fds_dn9));
        locals.var_fn382_calc_iq__vsx_rv = 0.0;

        let assign31540_e28740: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__myarg);
        let assign31540_e28742: f64 = (assign31540_e28740 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg = assign31540_e28742;
        locals.var_fn382_calc_iq__exparg_dn4 = ((((-locals.var_fn382_calc_iq__myarg_dn4) * locals.var_fn382_calc_iq__alpha_phit) - (assign31540_e28740 * locals.var_fn382_calc_iq__alpha_phit_dn4)) / (locals.var_fn382_calc_iq__alpha_phit * locals.var_fn382_calc_iq__alpha_phit));
        locals.var_fn382_calc_iq__exparg_dn5 = ((-locals.var_fn382_calc_iq__myarg_dn5) / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg_dn8 = ((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__myarg_dn8) / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg_dn9 = ((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__myarg_dn9) / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg_rv = 0.0;

        let assign31550_e28745: f64 = if locals.var_fn382_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard388 = assign31550_e28745;
        locals.var_guard388_rv = 0.0;

        let (assign31560_e28749, assign31560_e28749_d_n4, assign31560_e28749_d_n5, assign31560_e28749_d_n8, assign31560_e28749_d_n9,) = {
    if (locals.var_guard388 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ffs, locals.var_fn382_calc_iq__ffs_dn4, locals.var_fn382_calc_iq__ffs_dn5, locals.var_fn382_calc_iq__ffs_dn8, locals.var_fn382_calc_iq__ffs_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffs = assign31560_e28749;
        locals.var_fn382_calc_iq__ffs_dn4 = assign31560_e28749_d_n4;
        locals.var_fn382_calc_iq__ffs_dn5 = assign31560_e28749_d_n5;
        locals.var_fn382_calc_iq__ffs_dn8 = assign31560_e28749_d_n8;
        locals.var_fn382_calc_iq__ffs_dn9 = assign31560_e28749_d_n9;
        locals.var_fn382_calc_iq__ffs_rv = 0.0;

        let assign31570_e28752: f64 = (-50.0);
        let assign31570_e28753: f64 = if locals.var_fn382_calc_iq__exparg < assign31570_e28752 { 1.0 } else { 0.0 };
        locals.var_guard389 = assign31570_e28753;
        locals.var_guard389_rv = 0.0;

        let (assign31580_e28760, assign31580_e28760_d_n4, assign31580_e28760_d_n5, assign31580_e28760_d_n8, assign31580_e28760_d_n9,) = {
    if ((locals.var_guard388 == 0.0) && (locals.var_guard389 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ffs, locals.var_fn382_calc_iq__ffs_dn4, locals.var_fn382_calc_iq__ffs_dn5, locals.var_fn382_calc_iq__ffs_dn8, locals.var_fn382_calc_iq__ffs_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffs = assign31580_e28760;
        locals.var_fn382_calc_iq__ffs_dn4 = assign31580_e28760_d_n4;
        locals.var_fn382_calc_iq__ffs_dn5 = assign31580_e28760_d_n5;
        locals.var_fn382_calc_iq__ffs_dn8 = assign31580_e28760_d_n8;
        locals.var_fn382_calc_iq__ffs_dn9 = assign31580_e28760_d_n9;
        locals.var_fn382_calc_iq__ffs_rv = 0.0;

        let (assign31590_e28773, assign31590_e28773_d_n4, assign31590_e28773_d_n5, assign31590_e28773_d_n8, assign31590_e28773_d_n9,) = {
    if ((locals.var_guard388 == 0.0) && (locals.var_guard389 == 0.0)) {
        let assign31590_e28769: f64 = (locals.var_fn382_calc_iq__exparg).exp();
        let assign31590_e28770: f64 = (1.0 + assign31590_e28769);
        let assign31590_e28771: f64 = (1.0 / assign31590_e28770);
        (assign31590_e28771, (-((assign31590_e28769 * locals.var_fn382_calc_iq__exparg_dn4) / (assign31590_e28770 * assign31590_e28770))), (-((assign31590_e28769 * locals.var_fn382_calc_iq__exparg_dn5) / (assign31590_e28770 * assign31590_e28770))), (-((assign31590_e28769 * locals.var_fn382_calc_iq__exparg_dn8) / (assign31590_e28770 * assign31590_e28770))), (-((assign31590_e28769 * locals.var_fn382_calc_iq__exparg_dn9) / (assign31590_e28770 * assign31590_e28770))),)
    } else {
        (locals.var_fn382_calc_iq__ffs, locals.var_fn382_calc_iq__ffs_dn4, locals.var_fn382_calc_iq__ffs_dn5, locals.var_fn382_calc_iq__ffs_dn8, locals.var_fn382_calc_iq__ffs_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffs = assign31590_e28773;
        locals.var_fn382_calc_iq__ffs_dn4 = assign31590_e28773_d_n4;
        locals.var_fn382_calc_iq__ffs_dn5 = assign31590_e28773_d_n5;
        locals.var_fn382_calc_iq__ffs_dn8 = assign31590_e28773_d_n8;
        locals.var_fn382_calc_iq__ffs_dn9 = assign31590_e28773_d_n9;
        locals.var_fn382_calc_iq__ffs_rv = 0.0;

        let assign31600_e28776: f64 = (locals.var_fn382_calc_iq__vgdin - locals.var_fn382_calc_iq__vsx);
        let assign31600_e28780: f64 = (p.p51 * 0.1);
        let assign31600_e28782: f64 = (assign31600_e28780 * locals.var_fn382_calc_iq__alpha_phit);
        let assign31600_e28784: f64 = (assign31600_e28782 * locals.var_fn382_calc_iq__ffs);
        let assign31600_e28785: f64 = (locals.var_fn382_calc_iq__vtdibl - assign31600_e28784);
        let assign31600_e28786: f64 = (assign31600_e28776 - assign31600_e28785);
        let assign31600_e28788: f64 = (assign31600_e28786 / locals.var_fn382_calc_iq__two_n_phit);
        locals.var_fn382_calc_iq__etas = assign31600_e28788;
        locals.var_fn382_calc_iq__etas_dn4 = (((((-locals.var_fn382_calc_iq__vsx_dn4) - (locals.var_fn382_calc_iq__vtdibl_dn4 - (((assign31600_e28780 * locals.var_fn382_calc_iq__alpha_phit_dn4) * locals.var_fn382_calc_iq__ffs) + (assign31600_e28782 * locals.var_fn382_calc_iq__ffs_dn4)))) * locals.var_fn382_calc_iq__two_n_phit) - (assign31600_e28786 * locals.var_fn382_calc_iq__two_n_phit_dn4)) / (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__two_n_phit));
        locals.var_fn382_calc_iq__etas_dn5 = (((((locals.var_fn382_calc_iq__vgdin_dn5 - locals.var_fn382_calc_iq__vsx_dn5) - (locals.var_fn382_calc_iq__vtdibl_dn5 - (assign31600_e28782 * locals.var_fn382_calc_iq__ffs_dn5))) * locals.var_fn382_calc_iq__two_n_phit) - (assign31600_e28786 * locals.var_fn382_calc_iq__two_n_phit_dn5)) / (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__two_n_phit));
        locals.var_fn382_calc_iq__etas_dn8 = (((locals.var_fn382_calc_iq__vgdin_dn8 - locals.var_fn382_calc_iq__vsx_dn8) - (-(assign31600_e28782 * locals.var_fn382_calc_iq__ffs_dn8))) / locals.var_fn382_calc_iq__two_n_phit);
        locals.var_fn382_calc_iq__etas_dn9 = (((((locals.var_fn382_calc_iq__vgdin_dn9 - locals.var_fn382_calc_iq__vsx_dn9) - (locals.var_fn382_calc_iq__vtdibl_dn9 - (assign31600_e28782 * locals.var_fn382_calc_iq__ffs_dn9))) * locals.var_fn382_calc_iq__two_n_phit) - (assign31600_e28786 * locals.var_fn382_calc_iq__two_n_phit_dn9)) / (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__two_n_phit));
        locals.var_fn382_calc_iq__etas_rv = 0.0;

        let assign31610_e28791: f64 = if locals.var_fn382_calc_iq__etas > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard390 = assign31610_e28791;
        locals.var_guard390_rv = 0.0;

        let (assign31620_e28797, assign31620_e28797_d_n4, assign31620_e28797_d_n5, assign31620_e28797_d_n8, assign31620_e28797_d_n9,) = {
    if (locals.var_guard390 != 0.0) {
        let assign31620_e28795: f64 = (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__etas);
        (assign31620_e28795, ((locals.var_fn382_calc_iq__qref_dn4 * locals.var_fn382_calc_iq__etas) + (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__etas_dn4)), ((locals.var_fn382_calc_iq__qref_dn5 * locals.var_fn382_calc_iq__etas) + (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__etas_dn5)), (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__etas_dn8), ((locals.var_fn382_calc_iq__qref_dn9 * locals.var_fn382_calc_iq__etas) + (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__etas_dn9)),)
    } else {
        (locals.var_fn382_calc_iq__qinvs, locals.var_fn382_calc_iq__qinvs_dn4, locals.var_fn382_calc_iq__qinvs_dn5, locals.var_fn382_calc_iq__qinvs_dn8, locals.var_fn382_calc_iq__qinvs_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvs = assign31620_e28797;
        locals.var_fn382_calc_iq__qinvs_dn4 = assign31620_e28797_d_n4;
        locals.var_fn382_calc_iq__qinvs_dn5 = assign31620_e28797_d_n5;
        locals.var_fn382_calc_iq__qinvs_dn8 = assign31620_e28797_d_n8;
        locals.var_fn382_calc_iq__qinvs_dn9 = assign31620_e28797_d_n9;
        locals.var_fn382_calc_iq__qinvs_rv = 0.0;

        let assign31630_e28800: f64 = (-50.0);
        let assign31630_e28801: f64 = if locals.var_fn382_calc_iq__etas < assign31630_e28800 { 1.0 } else { 0.0 };
        locals.var_guard391 = assign31630_e28801;
        locals.var_guard391_rv = 0.0;

        let (assign31640_e28811, assign31640_e28811_d_n4, assign31640_e28811_d_n5, assign31640_e28811_d_n8, assign31640_e28811_d_n9,) = {
    if ((locals.var_guard390 == 0.0) && (locals.var_guard391 != 0.0)) {
        let assign31640_e28808: f64 = (locals.var_fn382_calc_iq__etas).exp();
        let assign31640_e28809: f64 = (locals.var_fn382_calc_iq__qref * assign31640_e28808);
        (assign31640_e28809, ((locals.var_fn382_calc_iq__qref_dn4 * assign31640_e28808) + (locals.var_fn382_calc_iq__qref * (assign31640_e28808 * locals.var_fn382_calc_iq__etas_dn4))), ((locals.var_fn382_calc_iq__qref_dn5 * assign31640_e28808) + (locals.var_fn382_calc_iq__qref * (assign31640_e28808 * locals.var_fn382_calc_iq__etas_dn5))), (locals.var_fn382_calc_iq__qref * (assign31640_e28808 * locals.var_fn382_calc_iq__etas_dn8)), ((locals.var_fn382_calc_iq__qref_dn9 * assign31640_e28808) + (locals.var_fn382_calc_iq__qref * (assign31640_e28808 * locals.var_fn382_calc_iq__etas_dn9))),)
    } else {
        (locals.var_fn382_calc_iq__qinvs, locals.var_fn382_calc_iq__qinvs_dn4, locals.var_fn382_calc_iq__qinvs_dn5, locals.var_fn382_calc_iq__qinvs_dn8, locals.var_fn382_calc_iq__qinvs_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvs = assign31640_e28811;
        locals.var_fn382_calc_iq__qinvs_dn4 = assign31640_e28811_d_n4;
        locals.var_fn382_calc_iq__qinvs_dn5 = assign31640_e28811_d_n5;
        locals.var_fn382_calc_iq__qinvs_dn8 = assign31640_e28811_d_n8;
        locals.var_fn382_calc_iq__qinvs_dn9 = assign31640_e28811_d_n9;
        locals.var_fn382_calc_iq__qinvs_rv = 0.0;

        let (assign31650_e28825, assign31650_e28825_d_n4, assign31650_e28825_d_n5, assign31650_e28825_d_n8, assign31650_e28825_d_n9,) = {
    if ((locals.var_guard390 == 0.0) && (locals.var_guard391 == 0.0)) {
        let assign31650_e28820: f64 = (locals.var_fn382_calc_iq__etas).exp();
        let assign31650_e28821: f64 = (1.0 + assign31650_e28820);
        let assign31650_e28822: f64 = (assign31650_e28821).ln();
        let assign31650_e28823: f64 = (locals.var_fn382_calc_iq__qref * assign31650_e28822);
        (assign31650_e28823, ((locals.var_fn382_calc_iq__qref_dn4 * assign31650_e28822) + (locals.var_fn382_calc_iq__qref * ((assign31650_e28820 * locals.var_fn382_calc_iq__etas_dn4) / assign31650_e28821))), ((locals.var_fn382_calc_iq__qref_dn5 * assign31650_e28822) + (locals.var_fn382_calc_iq__qref * ((assign31650_e28820 * locals.var_fn382_calc_iq__etas_dn5) / assign31650_e28821))), (locals.var_fn382_calc_iq__qref * ((assign31650_e28820 * locals.var_fn382_calc_iq__etas_dn8) / assign31650_e28821)), ((locals.var_fn382_calc_iq__qref_dn9 * assign31650_e28822) + (locals.var_fn382_calc_iq__qref * ((assign31650_e28820 * locals.var_fn382_calc_iq__etas_dn9) / assign31650_e28821))),)
    } else {
        (locals.var_fn382_calc_iq__qinvs, locals.var_fn382_calc_iq__qinvs_dn4, locals.var_fn382_calc_iq__qinvs_dn5, locals.var_fn382_calc_iq__qinvs_dn8, locals.var_fn382_calc_iq__qinvs_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvs = assign31650_e28825;
        locals.var_fn382_calc_iq__qinvs_dn4 = assign31650_e28825_d_n4;
        locals.var_fn382_calc_iq__qinvs_dn5 = assign31650_e28825_d_n5;
        locals.var_fn382_calc_iq__qinvs_dn8 = assign31650_e28825_d_n8;
        locals.var_fn382_calc_iq__qinvs_dn9 = assign31650_e28825_d_n9;
        locals.var_fn382_calc_iq__qinvs_rv = 0.0;

        let assign31660_e28828: f64 = (locals.var_fn382_calc_iq__vgdin - locals.var_fn382_calc_iq__myarg);
        let assign31660_e28830: f64 = (assign31660_e28828 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg = assign31660_e28830;
        locals.var_fn382_calc_iq__exparg_dn4 = ((((-locals.var_fn382_calc_iq__myarg_dn4) * locals.var_fn382_calc_iq__alpha_phit) - (assign31660_e28828 * locals.var_fn382_calc_iq__alpha_phit_dn4)) / (locals.var_fn382_calc_iq__alpha_phit * locals.var_fn382_calc_iq__alpha_phit));
        locals.var_fn382_calc_iq__exparg_dn5 = ((locals.var_fn382_calc_iq__vgdin_dn5 - locals.var_fn382_calc_iq__myarg_dn5) / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg_dn8 = ((locals.var_fn382_calc_iq__vgdin_dn8 - locals.var_fn382_calc_iq__myarg_dn8) / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg_dn9 = ((locals.var_fn382_calc_iq__vgdin_dn9 - locals.var_fn382_calc_iq__myarg_dn9) / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg_rv = 0.0;

        let assign31670_e28833: f64 = if locals.var_fn382_calc_iq__exparg > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard392 = assign31670_e28833;
        locals.var_guard392_rv = 0.0;

        let (assign31680_e28837, assign31680_e28837_d_n4, assign31680_e28837_d_n5, assign31680_e28837_d_n8, assign31680_e28837_d_n9,) = {
    if (locals.var_guard392 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ffd, locals.var_fn382_calc_iq__ffd_dn4, locals.var_fn382_calc_iq__ffd_dn5, locals.var_fn382_calc_iq__ffd_dn8, locals.var_fn382_calc_iq__ffd_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffd = assign31680_e28837;
        locals.var_fn382_calc_iq__ffd_dn4 = assign31680_e28837_d_n4;
        locals.var_fn382_calc_iq__ffd_dn5 = assign31680_e28837_d_n5;
        locals.var_fn382_calc_iq__ffd_dn8 = assign31680_e28837_d_n8;
        locals.var_fn382_calc_iq__ffd_dn9 = assign31680_e28837_d_n9;
        locals.var_fn382_calc_iq__ffd_rv = 0.0;

        let assign31690_e28840: f64 = (-50.0);
        let assign31690_e28841: f64 = if locals.var_fn382_calc_iq__exparg < assign31690_e28840 { 1.0 } else { 0.0 };
        locals.var_guard393 = assign31690_e28841;
        locals.var_guard393_rv = 0.0;

        let (assign31700_e28848, assign31700_e28848_d_n4, assign31700_e28848_d_n5, assign31700_e28848_d_n8, assign31700_e28848_d_n9,) = {
    if ((locals.var_guard392 == 0.0) && (locals.var_guard393 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ffd, locals.var_fn382_calc_iq__ffd_dn4, locals.var_fn382_calc_iq__ffd_dn5, locals.var_fn382_calc_iq__ffd_dn8, locals.var_fn382_calc_iq__ffd_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffd = assign31700_e28848;
        locals.var_fn382_calc_iq__ffd_dn4 = assign31700_e28848_d_n4;
        locals.var_fn382_calc_iq__ffd_dn5 = assign31700_e28848_d_n5;
        locals.var_fn382_calc_iq__ffd_dn8 = assign31700_e28848_d_n8;
        locals.var_fn382_calc_iq__ffd_dn9 = assign31700_e28848_d_n9;
        locals.var_fn382_calc_iq__ffd_rv = 0.0;

        let (assign31710_e28861, assign31710_e28861_d_n4, assign31710_e28861_d_n5, assign31710_e28861_d_n8, assign31710_e28861_d_n9,) = {
    if ((locals.var_guard392 == 0.0) && (locals.var_guard393 == 0.0)) {
        let assign31710_e28857: f64 = (locals.var_fn382_calc_iq__exparg).exp();
        let assign31710_e28858: f64 = (1.0 + assign31710_e28857);
        let assign31710_e28859: f64 = (1.0 / assign31710_e28858);
        (assign31710_e28859, (-((assign31710_e28857 * locals.var_fn382_calc_iq__exparg_dn4) / (assign31710_e28858 * assign31710_e28858))), (-((assign31710_e28857 * locals.var_fn382_calc_iq__exparg_dn5) / (assign31710_e28858 * assign31710_e28858))), (-((assign31710_e28857 * locals.var_fn382_calc_iq__exparg_dn8) / (assign31710_e28858 * assign31710_e28858))), (-((assign31710_e28857 * locals.var_fn382_calc_iq__exparg_dn9) / (assign31710_e28858 * assign31710_e28858))),)
    } else {
        (locals.var_fn382_calc_iq__ffd, locals.var_fn382_calc_iq__ffd_dn4, locals.var_fn382_calc_iq__ffd_dn5, locals.var_fn382_calc_iq__ffd_dn8, locals.var_fn382_calc_iq__ffd_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffd = assign31710_e28861;
        locals.var_fn382_calc_iq__ffd_dn4 = assign31710_e28861_d_n4;
        locals.var_fn382_calc_iq__ffd_dn5 = assign31710_e28861_d_n5;
        locals.var_fn382_calc_iq__ffd_dn8 = assign31710_e28861_d_n8;
        locals.var_fn382_calc_iq__ffd_dn9 = assign31710_e28861_d_n9;
        locals.var_fn382_calc_iq__ffd_rv = 0.0;

        let assign31720_e28864: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vdx);
        let assign31720_e28868: f64 = (p.p51 * 0.1);
        let assign31720_e28870: f64 = (assign31720_e28868 * locals.var_fn382_calc_iq__alpha_phit);
        let assign31720_e28872: f64 = (assign31720_e28870 * locals.var_fn382_calc_iq__ffd);
        let assign31720_e28873: f64 = (locals.var_fn382_calc_iq__vtdibl - assign31720_e28872);
        let assign31720_e28874: f64 = (assign31720_e28864 - assign31720_e28873);
        let assign31720_e28876: f64 = (assign31720_e28874 / locals.var_fn382_calc_iq__two_n_phit);
        locals.var_fn382_calc_iq__etad = assign31720_e28876;
        locals.var_fn382_calc_iq__etad_dn4 = (((((-locals.var_fn382_calc_iq__vdx_dn4) - (locals.var_fn382_calc_iq__vtdibl_dn4 - (((assign31720_e28868 * locals.var_fn382_calc_iq__alpha_phit_dn4) * locals.var_fn382_calc_iq__ffd) + (assign31720_e28870 * locals.var_fn382_calc_iq__ffd_dn4)))) * locals.var_fn382_calc_iq__two_n_phit) - (assign31720_e28874 * locals.var_fn382_calc_iq__two_n_phit_dn4)) / (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__two_n_phit));
        locals.var_fn382_calc_iq__etad_dn5 = (((((-locals.var_fn382_calc_iq__vdx_dn5) - (locals.var_fn382_calc_iq__vtdibl_dn5 - (assign31720_e28870 * locals.var_fn382_calc_iq__ffd_dn5))) * locals.var_fn382_calc_iq__two_n_phit) - (assign31720_e28874 * locals.var_fn382_calc_iq__two_n_phit_dn5)) / (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__two_n_phit));
        locals.var_fn382_calc_iq__etad_dn8 = (((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vdx_dn8) - (-(assign31720_e28870 * locals.var_fn382_calc_iq__ffd_dn8))) / locals.var_fn382_calc_iq__two_n_phit);
        locals.var_fn382_calc_iq__etad_dn9 = (((((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vdx_dn9) - (locals.var_fn382_calc_iq__vtdibl_dn9 - (assign31720_e28870 * locals.var_fn382_calc_iq__ffd_dn9))) * locals.var_fn382_calc_iq__two_n_phit) - (assign31720_e28874 * locals.var_fn382_calc_iq__two_n_phit_dn9)) / (locals.var_fn382_calc_iq__two_n_phit * locals.var_fn382_calc_iq__two_n_phit));
        locals.var_fn382_calc_iq__etad_rv = 0.0;

        let assign31730_e28879: f64 = if locals.var_fn382_calc_iq__etad > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard394 = assign31730_e28879;
        locals.var_guard394_rv = 0.0;

        let (assign31740_e28885, assign31740_e28885_d_n4, assign31740_e28885_d_n5, assign31740_e28885_d_n8, assign31740_e28885_d_n9,) = {
    if (locals.var_guard394 != 0.0) {
        let assign31740_e28883: f64 = (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__etad);
        (assign31740_e28883, ((locals.var_fn382_calc_iq__qref_dn4 * locals.var_fn382_calc_iq__etad) + (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__etad_dn4)), ((locals.var_fn382_calc_iq__qref_dn5 * locals.var_fn382_calc_iq__etad) + (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__etad_dn5)), (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__etad_dn8), ((locals.var_fn382_calc_iq__qref_dn9 * locals.var_fn382_calc_iq__etad) + (locals.var_fn382_calc_iq__qref * locals.var_fn382_calc_iq__etad_dn9)),)
    } else {
        (locals.var_fn382_calc_iq__qinvd, locals.var_fn382_calc_iq__qinvd_dn4, locals.var_fn382_calc_iq__qinvd_dn5, locals.var_fn382_calc_iq__qinvd_dn8, locals.var_fn382_calc_iq__qinvd_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvd = assign31740_e28885;
        locals.var_fn382_calc_iq__qinvd_dn4 = assign31740_e28885_d_n4;
        locals.var_fn382_calc_iq__qinvd_dn5 = assign31740_e28885_d_n5;
        locals.var_fn382_calc_iq__qinvd_dn8 = assign31740_e28885_d_n8;
        locals.var_fn382_calc_iq__qinvd_dn9 = assign31740_e28885_d_n9;
        locals.var_fn382_calc_iq__qinvd_rv = 0.0;

        let assign31750_e28888: f64 = (-50.0);
        let assign31750_e28889: f64 = if locals.var_fn382_calc_iq__etad < assign31750_e28888 { 1.0 } else { 0.0 };
        locals.var_guard395 = assign31750_e28889;
        locals.var_guard395_rv = 0.0;

        let (assign31760_e28899, assign31760_e28899_d_n4, assign31760_e28899_d_n5, assign31760_e28899_d_n8, assign31760_e28899_d_n9,) = {
    if ((locals.var_guard394 == 0.0) && (locals.var_guard395 != 0.0)) {
        let assign31760_e28896: f64 = (locals.var_fn382_calc_iq__etad).exp();
        let assign31760_e28897: f64 = (locals.var_fn382_calc_iq__qref * assign31760_e28896);
        (assign31760_e28897, ((locals.var_fn382_calc_iq__qref_dn4 * assign31760_e28896) + (locals.var_fn382_calc_iq__qref * (assign31760_e28896 * locals.var_fn382_calc_iq__etad_dn4))), ((locals.var_fn382_calc_iq__qref_dn5 * assign31760_e28896) + (locals.var_fn382_calc_iq__qref * (assign31760_e28896 * locals.var_fn382_calc_iq__etad_dn5))), (locals.var_fn382_calc_iq__qref * (assign31760_e28896 * locals.var_fn382_calc_iq__etad_dn8)), ((locals.var_fn382_calc_iq__qref_dn9 * assign31760_e28896) + (locals.var_fn382_calc_iq__qref * (assign31760_e28896 * locals.var_fn382_calc_iq__etad_dn9))),)
    } else {
        (locals.var_fn382_calc_iq__qinvd, locals.var_fn382_calc_iq__qinvd_dn4, locals.var_fn382_calc_iq__qinvd_dn5, locals.var_fn382_calc_iq__qinvd_dn8, locals.var_fn382_calc_iq__qinvd_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvd = assign31760_e28899;
        locals.var_fn382_calc_iq__qinvd_dn4 = assign31760_e28899_d_n4;
        locals.var_fn382_calc_iq__qinvd_dn5 = assign31760_e28899_d_n5;
        locals.var_fn382_calc_iq__qinvd_dn8 = assign31760_e28899_d_n8;
        locals.var_fn382_calc_iq__qinvd_dn9 = assign31760_e28899_d_n9;
        locals.var_fn382_calc_iq__qinvd_rv = 0.0;

        let (assign31770_e28913, assign31770_e28913_d_n4, assign31770_e28913_d_n5, assign31770_e28913_d_n8, assign31770_e28913_d_n9,) = {
    if ((locals.var_guard394 == 0.0) && (locals.var_guard395 == 0.0)) {
        let assign31770_e28908: f64 = (locals.var_fn382_calc_iq__etad).exp();
        let assign31770_e28909: f64 = (1.0 + assign31770_e28908);
        let assign31770_e28910: f64 = (assign31770_e28909).ln();
        let assign31770_e28911: f64 = (locals.var_fn382_calc_iq__qref * assign31770_e28910);
        (assign31770_e28911, ((locals.var_fn382_calc_iq__qref_dn4 * assign31770_e28910) + (locals.var_fn382_calc_iq__qref * ((assign31770_e28908 * locals.var_fn382_calc_iq__etad_dn4) / assign31770_e28909))), ((locals.var_fn382_calc_iq__qref_dn5 * assign31770_e28910) + (locals.var_fn382_calc_iq__qref * ((assign31770_e28908 * locals.var_fn382_calc_iq__etad_dn5) / assign31770_e28909))), (locals.var_fn382_calc_iq__qref * ((assign31770_e28908 * locals.var_fn382_calc_iq__etad_dn8) / assign31770_e28909)), ((locals.var_fn382_calc_iq__qref_dn9 * assign31770_e28910) + (locals.var_fn382_calc_iq__qref * ((assign31770_e28908 * locals.var_fn382_calc_iq__etad_dn9) / assign31770_e28909))),)
    } else {
        (locals.var_fn382_calc_iq__qinvd, locals.var_fn382_calc_iq__qinvd_dn4, locals.var_fn382_calc_iq__qinvd_dn5, locals.var_fn382_calc_iq__qinvd_dn8, locals.var_fn382_calc_iq__qinvd_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvd = assign31770_e28913;
        locals.var_fn382_calc_iq__qinvd_dn4 = assign31770_e28913_d_n4;
        locals.var_fn382_calc_iq__qinvd_dn5 = assign31770_e28913_d_n5;
        locals.var_fn382_calc_iq__qinvd_dn8 = assign31770_e28913_d_n8;
        locals.var_fn382_calc_iq__qinvd_dn9 = assign31770_e28913_d_n9;
        locals.var_fn382_calc_iq__qinvd_rv = 0.0;

        let assign31780_e28916: f64 = (locals.var_fn382_calc_iq__qinvs - locals.var_fn382_calc_iq__qinvd);
        let assign31780_e28918: f64 = (assign31780_e28916 / locals.var_fn382_calc_iq__cgin);
        locals.var_fn382_calc_iq__vdsc = assign31780_e28918;
        locals.var_fn382_calc_iq__vdsc_dn4 = ((((locals.var_fn382_calc_iq__qinvs_dn4 - locals.var_fn382_calc_iq__qinvd_dn4) * locals.var_fn382_calc_iq__cgin) - (assign31780_e28916 * locals.var_fn382_calc_iq__cgin_dn4)) / (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__cgin));
        locals.var_fn382_calc_iq__vdsc_dn5 = ((locals.var_fn382_calc_iq__qinvs_dn5 - locals.var_fn382_calc_iq__qinvd_dn5) / locals.var_fn382_calc_iq__cgin);
        locals.var_fn382_calc_iq__vdsc_dn8 = ((locals.var_fn382_calc_iq__qinvs_dn8 - locals.var_fn382_calc_iq__qinvd_dn8) / locals.var_fn382_calc_iq__cgin);
        locals.var_fn382_calc_iq__vdsc_dn9 = ((locals.var_fn382_calc_iq__qinvs_dn9 - locals.var_fn382_calc_iq__qinvd_dn9) / locals.var_fn382_calc_iq__cgin);
        locals.var_fn382_calc_iq__vdsc_rv = 0.0;

        let assign31790_e28921: f64 = (locals.var_fn382_calc_iq__vdsc / locals.var_fn382_calc_iq__vdsat);
        locals.var_fn382_calc_iq__myarg = assign31790_e28921;
        locals.var_fn382_calc_iq__myarg_dn4 = (((locals.var_fn382_calc_iq__vdsc_dn4 * locals.var_fn382_calc_iq__vdsat) - (locals.var_fn382_calc_iq__vdsc * locals.var_fn382_calc_iq__vdsat_dn4)) / (locals.var_fn382_calc_iq__vdsat * locals.var_fn382_calc_iq__vdsat));
        locals.var_fn382_calc_iq__myarg_dn5 = (((locals.var_fn382_calc_iq__vdsc_dn5 * locals.var_fn382_calc_iq__vdsat) - (locals.var_fn382_calc_iq__vdsc * locals.var_fn382_calc_iq__vdsat_dn5)) / (locals.var_fn382_calc_iq__vdsat * locals.var_fn382_calc_iq__vdsat));
        locals.var_fn382_calc_iq__myarg_dn8 = (((locals.var_fn382_calc_iq__vdsc_dn8 * locals.var_fn382_calc_iq__vdsat) - (locals.var_fn382_calc_iq__vdsc * locals.var_fn382_calc_iq__vdsat_dn8)) / (locals.var_fn382_calc_iq__vdsat * locals.var_fn382_calc_iq__vdsat));
        locals.var_fn382_calc_iq__myarg_dn9 = (((locals.var_fn382_calc_iq__vdsc_dn9 * locals.var_fn382_calc_iq__vdsat) - (locals.var_fn382_calc_iq__vdsc * locals.var_fn382_calc_iq__vdsat_dn9)) / (locals.var_fn382_calc_iq__vdsat * locals.var_fn382_calc_iq__vdsat));
        locals.var_fn382_calc_iq__myarg_rv = 0.0;

        let (assign31800_e28947, assign31800_e28947_d_n4, assign31800_e28947_d_n5, assign31800_e28947_d_n8, assign31800_e28947_d_n9,) = {
    if (p.p52 != 0.0) {
        let assign31800_e28931: f64 = (0.001 / p.p53);
        let assign31800_e28933: f64 = (assign31800_e28931 * locals.var_fn382_calc_iq__myarg);
        let assign31800_e28934: f64 = (assign31800_e28933).tanh();
        let assign31800_e28935: f64 = (locals.var_fn382_calc_iq__myarg * assign31800_e28934);
        (assign31800_e28935, ((locals.var_fn382_calc_iq__myarg_dn4 * assign31800_e28934) + (locals.var_fn382_calc_iq__myarg * ((assign31800_e28931 * locals.var_fn382_calc_iq__myarg_dn4) / ((assign31800_e28933).cosh() * (assign31800_e28933).cosh())))), ((locals.var_fn382_calc_iq__myarg_dn5 * assign31800_e28934) + (locals.var_fn382_calc_iq__myarg * ((assign31800_e28931 * locals.var_fn382_calc_iq__myarg_dn5) / ((assign31800_e28933).cosh() * (assign31800_e28933).cosh())))), ((locals.var_fn382_calc_iq__myarg_dn8 * assign31800_e28934) + (locals.var_fn382_calc_iq__myarg * ((assign31800_e28931 * locals.var_fn382_calc_iq__myarg_dn8) / ((assign31800_e28933).cosh() * (assign31800_e28933).cosh())))), ((locals.var_fn382_calc_iq__myarg_dn9 * assign31800_e28934) + (locals.var_fn382_calc_iq__myarg * ((assign31800_e28931 * locals.var_fn382_calc_iq__myarg_dn9) / ((assign31800_e28933).cosh() * (assign31800_e28933).cosh())))),)
    } else {
        let (assign31800_e28946, assign31800_e28946_d_n4, assign31800_e28946_d_n5, assign31800_e28946_d_n8, assign31800_e28946_d_n9,) = {
            if (p.p52 == 0.0) {
                let assign31800_e28941: f64 = (locals.var_fn382_calc_iq__myarg * locals.var_fn382_calc_iq__myarg);
                let assign31800_e28943: f64 = (assign31800_e28941 + p.p53);
                let assign31800_e28944: f64 = (assign31800_e28943).sqrt();
                (assign31800_e28944, (((locals.var_fn382_calc_iq__myarg_dn4 * locals.var_fn382_calc_iq__myarg) + (locals.var_fn382_calc_iq__myarg * locals.var_fn382_calc_iq__myarg_dn4)) / (2.0 * assign31800_e28944)), (((locals.var_fn382_calc_iq__myarg_dn5 * locals.var_fn382_calc_iq__myarg) + (locals.var_fn382_calc_iq__myarg * locals.var_fn382_calc_iq__myarg_dn5)) / (2.0 * assign31800_e28944)), (((locals.var_fn382_calc_iq__myarg_dn8 * locals.var_fn382_calc_iq__myarg) + (locals.var_fn382_calc_iq__myarg * locals.var_fn382_calc_iq__myarg_dn8)) / (2.0 * assign31800_e28944)), (((locals.var_fn382_calc_iq__myarg_dn9 * locals.var_fn382_calc_iq__myarg) + (locals.var_fn382_calc_iq__myarg * locals.var_fn382_calc_iq__myarg_dn9)) / (2.0 * assign31800_e28944)),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign31800_e28946, assign31800_e28946_d_n4, assign31800_e28946_d_n5, assign31800_e28946_d_n8, assign31800_e28946_d_n9,)
    }
};
        let assign31800_e28949: f64 = (assign31800_e28947).powf(locals.var_fn382_calc_iq__beta);
        let assign31800_e28950: f64 = (1.0 + assign31800_e28949);
        let assign31800_e28953: f64 = (1.0 / locals.var_fn382_calc_iq__beta);
        let assign31800_e28954: f64 = (assign31800_e28950).powf(assign31800_e28953);
        let assign31800_e28955: f64 = (locals.var_fn382_calc_iq__myarg / assign31800_e28954);
        locals.var_fn382_calc_iq__fsat = assign31800_e28955;
        locals.var_fn382_calc_iq__fsat_dn4 = (((locals.var_fn382_calc_iq__myarg_dn4 * assign31800_e28954) - (locals.var_fn382_calc_iq__myarg * if 0.0 == 0.0 && ((assign31800_e28953) as f64).is_finite() && ((assign31800_e28953) as f64).fract() == 0.0 { if assign31800_e28953 == 0.0 { 0.0 } else { (assign31800_e28953 * ((assign31800_e28950).powf(assign31800_e28953 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31800_e28947).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31800_e28947_d_n4)) } } else { (assign31800_e28949 * (locals.var_fn382_calc_iq__beta * (assign31800_e28947_d_n4 / assign31800_e28947))) })) } } else { (assign31800_e28954 * (assign31800_e28953 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31800_e28947).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31800_e28947_d_n4)) } } else { (assign31800_e28949 * (locals.var_fn382_calc_iq__beta * (assign31800_e28947_d_n4 / assign31800_e28947))) } / assign31800_e28950))) })) / (assign31800_e28954 * assign31800_e28954));
        locals.var_fn382_calc_iq__fsat_dn5 = (((locals.var_fn382_calc_iq__myarg_dn5 * assign31800_e28954) - (locals.var_fn382_calc_iq__myarg * if 0.0 == 0.0 && ((assign31800_e28953) as f64).is_finite() && ((assign31800_e28953) as f64).fract() == 0.0 { if assign31800_e28953 == 0.0 { 0.0 } else { (assign31800_e28953 * ((assign31800_e28950).powf(assign31800_e28953 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31800_e28947).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31800_e28947_d_n5)) } } else { (assign31800_e28949 * (locals.var_fn382_calc_iq__beta * (assign31800_e28947_d_n5 / assign31800_e28947))) })) } } else { (assign31800_e28954 * (assign31800_e28953 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31800_e28947).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31800_e28947_d_n5)) } } else { (assign31800_e28949 * (locals.var_fn382_calc_iq__beta * (assign31800_e28947_d_n5 / assign31800_e28947))) } / assign31800_e28950))) })) / (assign31800_e28954 * assign31800_e28954));
        locals.var_fn382_calc_iq__fsat_dn8 = (((locals.var_fn382_calc_iq__myarg_dn8 * assign31800_e28954) - (locals.var_fn382_calc_iq__myarg * if 0.0 == 0.0 && ((assign31800_e28953) as f64).is_finite() && ((assign31800_e28953) as f64).fract() == 0.0 { if assign31800_e28953 == 0.0 { 0.0 } else { (assign31800_e28953 * ((assign31800_e28950).powf(assign31800_e28953 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31800_e28947).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31800_e28947_d_n8)) } } else { (assign31800_e28949 * (locals.var_fn382_calc_iq__beta * (assign31800_e28947_d_n8 / assign31800_e28947))) })) } } else { (assign31800_e28954 * (assign31800_e28953 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31800_e28947).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31800_e28947_d_n8)) } } else { (assign31800_e28949 * (locals.var_fn382_calc_iq__beta * (assign31800_e28947_d_n8 / assign31800_e28947))) } / assign31800_e28950))) })) / (assign31800_e28954 * assign31800_e28954));
        locals.var_fn382_calc_iq__fsat_dn9 = (((locals.var_fn382_calc_iq__myarg_dn9 * assign31800_e28954) - (locals.var_fn382_calc_iq__myarg * if 0.0 == 0.0 && ((assign31800_e28953) as f64).is_finite() && ((assign31800_e28953) as f64).fract() == 0.0 { if assign31800_e28953 == 0.0 { 0.0 } else { (assign31800_e28953 * ((assign31800_e28950).powf(assign31800_e28953 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31800_e28947).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31800_e28947_d_n9)) } } else { (assign31800_e28949 * (locals.var_fn382_calc_iq__beta * (assign31800_e28947_d_n9 / assign31800_e28947))) })) } } else { (assign31800_e28954 * (assign31800_e28953 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign31800_e28947).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign31800_e28947_d_n9)) } } else { (assign31800_e28949 * (locals.var_fn382_calc_iq__beta * (assign31800_e28947_d_n9 / assign31800_e28947))) } / assign31800_e28950))) })) / (assign31800_e28954 * assign31800_e28954));
        locals.var_fn382_calc_iq__fsat_rv = 0.0;

        let assign31810_e28958: f64 = (locals.var_fn382_calc_iq__vxf * locals.var_fn382_calc_iq__fsat);
        locals.var_fn382_calc_iq__vel = assign31810_e28958;
        locals.var_fn382_calc_iq__vel_dn4 = ((locals.var_fn382_calc_iq__vxf_dn4 * locals.var_fn382_calc_iq__fsat) + (locals.var_fn382_calc_iq__vxf * locals.var_fn382_calc_iq__fsat_dn4));
        locals.var_fn382_calc_iq__vel_dn5 = ((locals.var_fn382_calc_iq__vxf_dn5 * locals.var_fn382_calc_iq__fsat) + (locals.var_fn382_calc_iq__vxf * locals.var_fn382_calc_iq__fsat_dn5));
        locals.var_fn382_calc_iq__vel_dn8 = ((locals.var_fn382_calc_iq__vxf_dn8 * locals.var_fn382_calc_iq__fsat) + (locals.var_fn382_calc_iq__vxf * locals.var_fn382_calc_iq__fsat_dn8));
        locals.var_fn382_calc_iq__vel_dn9 = ((locals.var_fn382_calc_iq__vxf_dn9 * locals.var_fn382_calc_iq__fsat) + (locals.var_fn382_calc_iq__vxf * locals.var_fn382_calc_iq__fsat_dn9));
        locals.var_fn382_calc_iq__vel_rv = 0.0;

        let assign31820_e28961: f64 = (locals.var_fn382_calc_iq__type * locals.var_fn382_calc_iq__w);
        let assign31820_e28963: f64 = (assign31820_e28961 * locals.var_fn382_calc_iq__ngf);
        let assign31820_e28965: f64 = (assign31820_e28963 * 0.5);
        let assign31820_e28968: f64 = (locals.var_fn382_calc_iq__qinvs + locals.var_fn382_calc_iq__qinvd);
        let assign31820_e28969: f64 = (assign31820_e28965 * assign31820_e28968);
        let assign31820_e28971: f64 = (assign31820_e28969 * locals.var_fn382_calc_iq__vel);
        let assign31820_e28973: f64 = (assign31820_e28971 * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__idsout = assign31820_e28973;
        locals.var_fn382_calc_iq__idsout_dn4 = ((((assign31820_e28965 * (locals.var_fn382_calc_iq__qinvs_dn4 + locals.var_fn382_calc_iq__qinvd_dn4)) * locals.var_fn382_calc_iq__vel) + (assign31820_e28969 * locals.var_fn382_calc_iq__vel_dn4)) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__idsout_dn5 = ((((assign31820_e28965 * (locals.var_fn382_calc_iq__qinvs_dn5 + locals.var_fn382_calc_iq__qinvd_dn5)) * locals.var_fn382_calc_iq__vel) + (assign31820_e28969 * locals.var_fn382_calc_iq__vel_dn5)) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__idsout_dn8 = ((((assign31820_e28965 * (locals.var_fn382_calc_iq__qinvs_dn8 + locals.var_fn382_calc_iq__qinvd_dn8)) * locals.var_fn382_calc_iq__vel) + (assign31820_e28969 * locals.var_fn382_calc_iq__vel_dn8)) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__idsout_dn9 = ((((assign31820_e28965 * (locals.var_fn382_calc_iq__qinvs_dn9 + locals.var_fn382_calc_iq__qinvd_dn9)) * locals.var_fn382_calc_iq__vel) + (assign31820_e28969 * locals.var_fn382_calc_iq__vel_dn9)) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__idsout_dn22 = (assign31820_e28971 * locals.var_fn382_calc_iq__trapfracdl_dn22);
        locals.var_fn382_calc_iq__idsout_dn23 = (assign31820_e28971 * locals.var_fn382_calc_iq__trapfracdl_dn23);
        locals.var_fn382_calc_iq__idsout_dn25 = (assign31820_e28971 * locals.var_fn382_calc_iq__trapfracdl_dn25);
        locals.var_fn382_calc_iq__idsout_dn26 = (assign31820_e28971 * locals.var_fn382_calc_iq__trapfracdl_dn26);
        locals.var_fn382_calc_iq__idsout_rv = 0.0;

        let assign31830_e28977: f64 = (2.302585092994046 * locals.var_fn382_calc_iq__phitin);
        let assign31830_e28978: f64 = (locals.var_fn382_calc_iq__ss / assign31830_e28977);
        locals.var_fn382_calc_iq__n0 = assign31830_e28978;
        locals.var_fn382_calc_iq__n0_dn4 = (-((locals.var_fn382_calc_iq__ss * (2.302585092994046 * locals.var_fn382_calc_iq__phitin_dn4)) / (assign31830_e28977 * assign31830_e28977)));
        locals.var_fn382_calc_iq__n0_rv = 0.0;

        let assign31840_e28981: f64 = (2.0 * locals.var_fn382_calc_iq__n0);
        let assign31840_e28983: f64 = (assign31840_e28981 * locals.var_fn382_calc_iq__phitin);
        locals.var_fn382_calc_iq__two_n_phit0 = assign31840_e28983;
        locals.var_fn382_calc_iq__two_n_phit0_dn4 = (((2.0 * locals.var_fn382_calc_iq__n0_dn4) * locals.var_fn382_calc_iq__phitin) + (assign31840_e28981 * locals.var_fn382_calc_iq__phitin_dn4));
        locals.var_fn382_calc_iq__two_n_phit0_rv = 0.0;

        let assign31850_e28986: f64 = (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__qref0 = assign31850_e28986;
        locals.var_fn382_calc_iq__qref0_dn4 = ((locals.var_fn382_calc_iq__cgin_dn4 * locals.var_fn382_calc_iq__two_n_phit0) + (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__two_n_phit0_dn4));
        locals.var_fn382_calc_iq__qref0_rv = 0.0;

        let assign31860_e28990: f64 = (p.p51 * locals.var_fn382_calc_iq__alpha_phit);
        let assign31860_e28992: f64 = (assign31860_e28990 / 2.0);
        let assign31860_e28993: f64 = (locals.var_fn382_calc_iq__vtof - assign31860_e28992);
        locals.var_fn382_calc_iq__myarg0 = assign31860_e28993;
        locals.var_fn382_calc_iq__myarg0_dn4 = (locals.var_fn382_calc_iq__vtof_dn4 - ((p.p51 * locals.var_fn382_calc_iq__alpha_phit_dn4) / 2.0));
        locals.var_fn382_calc_iq__myarg0_rv = 0.0;

        let (assign31870_e29037, assign31870_e29037_d_n5, assign31870_e29037_d_n8, assign31870_e29037_d_n9,) = {
    if (p.p52 != 0.0) {
        let assign31870_e29001: f64 = (locals.var_fn382_calc_iq__vgsin + locals.var_fn382_calc_iq__vgdin);
        let assign31870_e29004: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
        let assign31870_e29007: f64 = (0.001 / p.p53);
        let assign31870_e29010: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
        let assign31870_e29011: f64 = (assign31870_e29007 * assign31870_e29010);
        let assign31870_e29012: f64 = (assign31870_e29011).tanh();
        let assign31870_e29013: f64 = (assign31870_e29004 * assign31870_e29012);
        let assign31870_e29014: f64 = (assign31870_e29001 + assign31870_e29013);
        let assign31870_e29015: f64 = (0.5 * assign31870_e29014);
        (assign31870_e29015, (0.5 * (locals.var_fn382_calc_iq__vgdin_dn5 + (((-locals.var_fn382_calc_iq__vgdin_dn5) * assign31870_e29012) + (assign31870_e29004 * ((assign31870_e29007 * (-locals.var_fn382_calc_iq__vgdin_dn5)) / ((assign31870_e29011).cosh() * (assign31870_e29011).cosh())))))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn8 + locals.var_fn382_calc_iq__vgdin_dn8) + (((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8) * assign31870_e29012) + (assign31870_e29004 * ((assign31870_e29007 * (locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8)) / ((assign31870_e29011).cosh() * (assign31870_e29011).cosh())))))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn9 + locals.var_fn382_calc_iq__vgdin_dn9) + (((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9) * assign31870_e29012) + (assign31870_e29004 * ((assign31870_e29007 * (locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9)) / ((assign31870_e29011).cosh() * (assign31870_e29011).cosh())))))),)
    } else {
        let (assign31870_e29036, assign31870_e29036_d_n5, assign31870_e29036_d_n8, assign31870_e29036_d_n9,) = {
            if (p.p52 == 0.0) {
                let assign31870_e29022: f64 = (locals.var_fn382_calc_iq__vgsin + locals.var_fn382_calc_iq__vgdin);
                let assign31870_e29025: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
                let assign31870_e29028: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
                let assign31870_e29029: f64 = (assign31870_e29025 * assign31870_e29028);
                let assign31870_e29031: f64 = (assign31870_e29029 + p.p53);
                let assign31870_e29032: f64 = (assign31870_e29031).sqrt();
                let assign31870_e29033: f64 = (assign31870_e29022 + assign31870_e29032);
                let assign31870_e29034: f64 = (0.5 * assign31870_e29033);
                (assign31870_e29034, (0.5 * (locals.var_fn382_calc_iq__vgdin_dn5 + ((((-locals.var_fn382_calc_iq__vgdin_dn5) * assign31870_e29028) + (assign31870_e29025 * (-locals.var_fn382_calc_iq__vgdin_dn5))) / (2.0 * assign31870_e29032)))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn8 + locals.var_fn382_calc_iq__vgdin_dn8) + ((((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8) * assign31870_e29028) + (assign31870_e29025 * (locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8))) / (2.0 * assign31870_e29032)))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn9 + locals.var_fn382_calc_iq__vgdin_dn9) + ((((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9) * assign31870_e29028) + (assign31870_e29025 * (locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9))) / (2.0 * assign31870_e29032)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign31870_e29036, assign31870_e29036_d_n5, assign31870_e29036_d_n8, assign31870_e29036_d_n9,)
    }
};
        let assign31870_e29039: f64 = (assign31870_e29037 - locals.var_fn382_calc_iq__myarg0);
        let assign31870_e29041: f64 = (assign31870_e29039 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg0 = assign31870_e29041;
        locals.var_fn382_calc_iq__exparg0_dn4 = ((((-locals.var_fn382_calc_iq__myarg0_dn4) * locals.var_fn382_calc_iq__alpha_phit) - (assign31870_e29039 * locals.var_fn382_calc_iq__alpha_phit_dn4)) / (locals.var_fn382_calc_iq__alpha_phit * locals.var_fn382_calc_iq__alpha_phit));
        locals.var_fn382_calc_iq__exparg0_dn5 = (assign31870_e29037_d_n5 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg0_dn8 = (assign31870_e29037_d_n8 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg0_dn9 = (assign31870_e29037_d_n9 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg0_rv = 0.0;

        let assign31880_e29044: f64 = if locals.var_fn382_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard396 = assign31880_e29044;
        locals.var_guard396_rv = 0.0;

        let (assign31890_e29048, assign31890_e29048_d_n4, assign31890_e29048_d_n5, assign31890_e29048_d_n8, assign31890_e29048_d_n9,) = {
    if (locals.var_guard396 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ff0, locals.var_fn382_calc_iq__ff0_dn4, locals.var_fn382_calc_iq__ff0_dn5, locals.var_fn382_calc_iq__ff0_dn8, locals.var_fn382_calc_iq__ff0_dn9,)
    }
};
        locals.var_fn382_calc_iq__ff0 = assign31890_e29048;
        locals.var_fn382_calc_iq__ff0_dn4 = assign31890_e29048_d_n4;
        locals.var_fn382_calc_iq__ff0_dn5 = assign31890_e29048_d_n5;
        locals.var_fn382_calc_iq__ff0_dn8 = assign31890_e29048_d_n8;
        locals.var_fn382_calc_iq__ff0_dn9 = assign31890_e29048_d_n9;
        locals.var_fn382_calc_iq__ff0_rv = 0.0;

        let assign31900_e29051: f64 = (-50.0);
        let assign31900_e29052: f64 = if locals.var_fn382_calc_iq__exparg0 < assign31900_e29051 { 1.0 } else { 0.0 };
        locals.var_guard397 = assign31900_e29052;
        locals.var_guard397_rv = 0.0;

        let (assign31910_e29059, assign31910_e29059_d_n4, assign31910_e29059_d_n5, assign31910_e29059_d_n8, assign31910_e29059_d_n9,) = {
    if ((locals.var_guard396 == 0.0) && (locals.var_guard397 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ff0, locals.var_fn382_calc_iq__ff0_dn4, locals.var_fn382_calc_iq__ff0_dn5, locals.var_fn382_calc_iq__ff0_dn8, locals.var_fn382_calc_iq__ff0_dn9,)
    }
};
        locals.var_fn382_calc_iq__ff0 = assign31910_e29059;
        locals.var_fn382_calc_iq__ff0_dn4 = assign31910_e29059_d_n4;
        locals.var_fn382_calc_iq__ff0_dn5 = assign31910_e29059_d_n5;
        locals.var_fn382_calc_iq__ff0_dn8 = assign31910_e29059_d_n8;
        locals.var_fn382_calc_iq__ff0_dn9 = assign31910_e29059_d_n9;
        locals.var_fn382_calc_iq__ff0_rv = 0.0;

        let (assign31920_e29072, assign31920_e29072_d_n4, assign31920_e29072_d_n5, assign31920_e29072_d_n8, assign31920_e29072_d_n9,) = {
    if ((locals.var_guard396 == 0.0) && (locals.var_guard397 == 0.0)) {
        let assign31920_e29068: f64 = (locals.var_fn382_calc_iq__exparg0).exp();
        let assign31920_e29069: f64 = (1.0 + assign31920_e29068);
        let assign31920_e29070: f64 = (1.0 / assign31920_e29069);
        (assign31920_e29070, (-((assign31920_e29068 * locals.var_fn382_calc_iq__exparg0_dn4) / (assign31920_e29069 * assign31920_e29069))), (-((assign31920_e29068 * locals.var_fn382_calc_iq__exparg0_dn5) / (assign31920_e29069 * assign31920_e29069))), (-((assign31920_e29068 * locals.var_fn382_calc_iq__exparg0_dn8) / (assign31920_e29069 * assign31920_e29069))), (-((assign31920_e29068 * locals.var_fn382_calc_iq__exparg0_dn9) / (assign31920_e29069 * assign31920_e29069))),)
    } else {
        (locals.var_fn382_calc_iq__ff0, locals.var_fn382_calc_iq__ff0_dn4, locals.var_fn382_calc_iq__ff0_dn5, locals.var_fn382_calc_iq__ff0_dn8, locals.var_fn382_calc_iq__ff0_dn9,)
    }
};
        locals.var_fn382_calc_iq__ff0 = assign31920_e29072;
        locals.var_fn382_calc_iq__ff0_dn4 = assign31920_e29072_d_n4;
        locals.var_fn382_calc_iq__ff0_dn5 = assign31920_e29072_d_n5;
        locals.var_fn382_calc_iq__ff0_dn8 = assign31920_e29072_d_n8;
        locals.var_fn382_calc_iq__ff0_dn9 = assign31920_e29072_d_n9;
        locals.var_fn382_calc_iq__ff0_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_70(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign31930_e29116, assign31930_e29116_d_n5, assign31930_e29116_d_n8, assign31930_e29116_d_n9,) = {
    if (p.p52 != 0.0) {
        let assign31930_e29080: f64 = (locals.var_fn382_calc_iq__vgsin + locals.var_fn382_calc_iq__vgdin);
        let assign31930_e29083: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
        let assign31930_e29086: f64 = (0.001 / p.p53);
        let assign31930_e29089: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
        let assign31930_e29090: f64 = (assign31930_e29086 * assign31930_e29089);
        let assign31930_e29091: f64 = (assign31930_e29090).tanh();
        let assign31930_e29092: f64 = (assign31930_e29083 * assign31930_e29091);
        let assign31930_e29093: f64 = (assign31930_e29080 + assign31930_e29092);
        let assign31930_e29094: f64 = (0.5 * assign31930_e29093);
        (assign31930_e29094, (0.5 * (locals.var_fn382_calc_iq__vgdin_dn5 + (((-locals.var_fn382_calc_iq__vgdin_dn5) * assign31930_e29091) + (assign31930_e29083 * ((assign31930_e29086 * (-locals.var_fn382_calc_iq__vgdin_dn5)) / ((assign31930_e29090).cosh() * (assign31930_e29090).cosh())))))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn8 + locals.var_fn382_calc_iq__vgdin_dn8) + (((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8) * assign31930_e29091) + (assign31930_e29083 * ((assign31930_e29086 * (locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8)) / ((assign31930_e29090).cosh() * (assign31930_e29090).cosh())))))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn9 + locals.var_fn382_calc_iq__vgdin_dn9) + (((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9) * assign31930_e29091) + (assign31930_e29083 * ((assign31930_e29086 * (locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9)) / ((assign31930_e29090).cosh() * (assign31930_e29090).cosh())))))),)
    } else {
        let (assign31930_e29115, assign31930_e29115_d_n5, assign31930_e29115_d_n8, assign31930_e29115_d_n9,) = {
            if (p.p52 == 0.0) {
                let assign31930_e29101: f64 = (locals.var_fn382_calc_iq__vgsin + locals.var_fn382_calc_iq__vgdin);
                let assign31930_e29104: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
                let assign31930_e29107: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vgdin);
                let assign31930_e29108: f64 = (assign31930_e29104 * assign31930_e29107);
                let assign31930_e29110: f64 = (assign31930_e29108 + p.p53);
                let assign31930_e29111: f64 = (assign31930_e29110).sqrt();
                let assign31930_e29112: f64 = (assign31930_e29101 + assign31930_e29111);
                let assign31930_e29113: f64 = (0.5 * assign31930_e29112);
                (assign31930_e29113, (0.5 * (locals.var_fn382_calc_iq__vgdin_dn5 + ((((-locals.var_fn382_calc_iq__vgdin_dn5) * assign31930_e29107) + (assign31930_e29104 * (-locals.var_fn382_calc_iq__vgdin_dn5))) / (2.0 * assign31930_e29111)))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn8 + locals.var_fn382_calc_iq__vgdin_dn8) + ((((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8) * assign31930_e29107) + (assign31930_e29104 * (locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vgdin_dn8))) / (2.0 * assign31930_e29111)))), (0.5 * ((locals.var_fn382_calc_iq__vgsin_dn9 + locals.var_fn382_calc_iq__vgdin_dn9) + ((((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9) * assign31930_e29107) + (assign31930_e29104 * (locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vgdin_dn9))) / (2.0 * assign31930_e29111)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign31930_e29115, assign31930_e29115_d_n5, assign31930_e29115_d_n8, assign31930_e29115_d_n9,)
    }
};
        let assign31930_e29120: f64 = (p.p51 * 0.1);
        let assign31930_e29122: f64 = (assign31930_e29120 * locals.var_fn382_calc_iq__alpha_phit);
        let assign31930_e29124: f64 = (assign31930_e29122 * locals.var_fn382_calc_iq__ff0);
        let assign31930_e29125: f64 = (locals.var_fn382_calc_iq__vtof - assign31930_e29124);
        let assign31930_e29126: f64 = (assign31930_e29116 - assign31930_e29125);
        let assign31930_e29128: f64 = (assign31930_e29126 / locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__eta0 = assign31930_e29128;
        locals.var_fn382_calc_iq__eta0_dn4 = ((((-(locals.var_fn382_calc_iq__vtof_dn4 - (((assign31930_e29120 * locals.var_fn382_calc_iq__alpha_phit_dn4) * locals.var_fn382_calc_iq__ff0) + (assign31930_e29122 * locals.var_fn382_calc_iq__ff0_dn4)))) * locals.var_fn382_calc_iq__two_n_phit0) - (assign31930_e29126 * locals.var_fn382_calc_iq__two_n_phit0_dn4)) / (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__two_n_phit0));
        locals.var_fn382_calc_iq__eta0_dn5 = ((assign31930_e29116_d_n5 - (-(assign31930_e29122 * locals.var_fn382_calc_iq__ff0_dn5))) / locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__eta0_dn8 = ((assign31930_e29116_d_n8 - (-(assign31930_e29122 * locals.var_fn382_calc_iq__ff0_dn8))) / locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__eta0_dn9 = ((assign31930_e29116_d_n9 - (-(assign31930_e29122 * locals.var_fn382_calc_iq__ff0_dn9))) / locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__eta0_rv = 0.0;

        let assign31940_e29131: f64 = if locals.var_fn382_calc_iq__eta0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard398 = assign31940_e29131;
        locals.var_guard398_rv = 0.0;

        let (assign31950_e29137, assign31950_e29137_d_n4, assign31950_e29137_d_n5, assign31950_e29137_d_n8, assign31950_e29137_d_n9,) = {
    if (locals.var_guard398 != 0.0) {
        let assign31950_e29135: f64 = (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__eta0);
        (assign31950_e29135, ((locals.var_fn382_calc_iq__qref0_dn4 * locals.var_fn382_calc_iq__eta0) + (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__eta0_dn4)), (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__eta0_dn5), (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__eta0_dn8), (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__eta0_dn9),)
    } else {
        (locals.var_fn382_calc_iq__qinvv0, locals.var_fn382_calc_iq__qinvv0_dn4, locals.var_fn382_calc_iq__qinvv0_dn5, locals.var_fn382_calc_iq__qinvv0_dn8, locals.var_fn382_calc_iq__qinvv0_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvv0 = assign31950_e29137;
        locals.var_fn382_calc_iq__qinvv0_dn4 = assign31950_e29137_d_n4;
        locals.var_fn382_calc_iq__qinvv0_dn5 = assign31950_e29137_d_n5;
        locals.var_fn382_calc_iq__qinvv0_dn8 = assign31950_e29137_d_n8;
        locals.var_fn382_calc_iq__qinvv0_dn9 = assign31950_e29137_d_n9;
        locals.var_fn382_calc_iq__qinvv0_rv = 0.0;

        let assign31960_e29140: f64 = (-50.0);
        let assign31960_e29141: f64 = if locals.var_fn382_calc_iq__eta0 < assign31960_e29140 { 1.0 } else { 0.0 };
        locals.var_guard399 = assign31960_e29141;
        locals.var_guard399_rv = 0.0;

        let (assign31970_e29151, assign31970_e29151_d_n4, assign31970_e29151_d_n5, assign31970_e29151_d_n8, assign31970_e29151_d_n9,) = {
    if ((locals.var_guard398 == 0.0) && (locals.var_guard399 != 0.0)) {
        let assign31970_e29148: f64 = (locals.var_fn382_calc_iq__eta0).exp();
        let assign31970_e29149: f64 = (locals.var_fn382_calc_iq__qref0 * assign31970_e29148);
        (assign31970_e29149, ((locals.var_fn382_calc_iq__qref0_dn4 * assign31970_e29148) + (locals.var_fn382_calc_iq__qref0 * (assign31970_e29148 * locals.var_fn382_calc_iq__eta0_dn4))), (locals.var_fn382_calc_iq__qref0 * (assign31970_e29148 * locals.var_fn382_calc_iq__eta0_dn5)), (locals.var_fn382_calc_iq__qref0 * (assign31970_e29148 * locals.var_fn382_calc_iq__eta0_dn8)), (locals.var_fn382_calc_iq__qref0 * (assign31970_e29148 * locals.var_fn382_calc_iq__eta0_dn9)),)
    } else {
        (locals.var_fn382_calc_iq__qinvv0, locals.var_fn382_calc_iq__qinvv0_dn4, locals.var_fn382_calc_iq__qinvv0_dn5, locals.var_fn382_calc_iq__qinvv0_dn8, locals.var_fn382_calc_iq__qinvv0_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvv0 = assign31970_e29151;
        locals.var_fn382_calc_iq__qinvv0_dn4 = assign31970_e29151_d_n4;
        locals.var_fn382_calc_iq__qinvv0_dn5 = assign31970_e29151_d_n5;
        locals.var_fn382_calc_iq__qinvv0_dn8 = assign31970_e29151_d_n8;
        locals.var_fn382_calc_iq__qinvv0_dn9 = assign31970_e29151_d_n9;
        locals.var_fn382_calc_iq__qinvv0_rv = 0.0;

        let (assign31980_e29165, assign31980_e29165_d_n4, assign31980_e29165_d_n5, assign31980_e29165_d_n8, assign31980_e29165_d_n9,) = {
    if ((locals.var_guard398 == 0.0) && (locals.var_guard399 == 0.0)) {
        let assign31980_e29160: f64 = (locals.var_fn382_calc_iq__eta0).exp();
        let assign31980_e29161: f64 = (1.0 + assign31980_e29160);
        let assign31980_e29162: f64 = (assign31980_e29161).ln();
        let assign31980_e29163: f64 = (locals.var_fn382_calc_iq__qref0 * assign31980_e29162);
        (assign31980_e29163, ((locals.var_fn382_calc_iq__qref0_dn4 * assign31980_e29162) + (locals.var_fn382_calc_iq__qref0 * ((assign31980_e29160 * locals.var_fn382_calc_iq__eta0_dn4) / assign31980_e29161))), (locals.var_fn382_calc_iq__qref0 * ((assign31980_e29160 * locals.var_fn382_calc_iq__eta0_dn5) / assign31980_e29161)), (locals.var_fn382_calc_iq__qref0 * ((assign31980_e29160 * locals.var_fn382_calc_iq__eta0_dn8) / assign31980_e29161)), (locals.var_fn382_calc_iq__qref0 * ((assign31980_e29160 * locals.var_fn382_calc_iq__eta0_dn9) / assign31980_e29161)),)
    } else {
        (locals.var_fn382_calc_iq__qinvv0, locals.var_fn382_calc_iq__qinvv0_dn4, locals.var_fn382_calc_iq__qinvv0_dn5, locals.var_fn382_calc_iq__qinvv0_dn8, locals.var_fn382_calc_iq__qinvv0_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvv0 = assign31980_e29165;
        locals.var_fn382_calc_iq__qinvv0_dn4 = assign31980_e29165_d_n4;
        locals.var_fn382_calc_iq__qinvv0_dn5 = assign31980_e29165_d_n5;
        locals.var_fn382_calc_iq__qinvv0_dn8 = assign31980_e29165_d_n8;
        locals.var_fn382_calc_iq__qinvv0_dn9 = assign31980_e29165_d_n9;
        locals.var_fn382_calc_iq__qinvv0_rv = 0.0;

        let assign31990_e29168: f64 = (locals.var_fn382_calc_iq__mu0 / locals.var_fn382_calc_iq__tfacmobin);
        locals.var_fn382_calc_iq__muf0 = assign31990_e29168;
        locals.var_fn382_calc_iq__muf0_dn4 = (-((locals.var_fn382_calc_iq__mu0 * locals.var_fn382_calc_iq__tfacmobin_dn4) / (locals.var_fn382_calc_iq__tfacmobin * locals.var_fn382_calc_iq__tfacmobin)));
        locals.var_fn382_calc_iq__muf0_rv = 0.0;

        let assign32000_e29173: f64 = (locals.var_fn382_calc_iq__vzeta * locals.var_fn382_calc_iq__tnomin);
        let assign32000_e29174: f64 = (1.0 + assign32000_e29173);
        let assign32000_e29178: f64 = (locals.var_fn382_calc_iq__vzeta * locals.var_fn382_calc_iq__tambin);
        let assign32000_e29179: f64 = (1.0 + assign32000_e29178);
        let assign32000_e29180: f64 = (assign32000_e29174 / assign32000_e29179);
        let assign32000_e29181: f64 = (locals.var_fn382_calc_iq__vel0 * assign32000_e29180);
        locals.var_fn382_calc_iq__vx0 = assign32000_e29181;
        locals.var_fn382_calc_iq__vx0_dn4 = (locals.var_fn382_calc_iq__vel0 * (-((assign32000_e29174 * (locals.var_fn382_calc_iq__vzeta * locals.var_fn382_calc_iq__tambin_dn4)) / (assign32000_e29179 * assign32000_e29179))));
        locals.var_fn382_calc_iq__vx0_rv = 0.0;

        let assign32010_e29184: f64 = (locals.var_fn382_calc_iq__vx0 * locals.var_fn382_calc_iq__lin);
        let assign32010_e29186: f64 = (assign32010_e29184 / locals.var_fn382_calc_iq__muf0);
        locals.var_fn382_calc_iq__vdsats0 = assign32010_e29186;
        locals.var_fn382_calc_iq__vdsats0_dn4 = ((((locals.var_fn382_calc_iq__vx0_dn4 * locals.var_fn382_calc_iq__lin) * locals.var_fn382_calc_iq__muf0) - (assign32010_e29184 * locals.var_fn382_calc_iq__muf0_dn4)) / (locals.var_fn382_calc_iq__muf0 * locals.var_fn382_calc_iq__muf0));
        locals.var_fn382_calc_iq__vdsats0_rv = 0.0;

        let assign32020_e29191: f64 = (2.0 * locals.var_fn382_calc_iq__qinvv0);
        let assign32020_e29193: f64 = (assign32020_e29191 / locals.var_fn382_calc_iq__cgin);
        let assign32020_e29195: f64 = (assign32020_e29193 / locals.var_fn382_calc_iq__vdsats0);
        let assign32020_e29196: f64 = (1.0 + assign32020_e29195);
        let assign32020_e29197: f64 = (assign32020_e29196).sqrt();
        let assign32020_e29198: f64 = (locals.var_fn382_calc_iq__vdsats0 * assign32020_e29197);
        let assign32020_e29200: f64 = (assign32020_e29198 - locals.var_fn382_calc_iq__vdsats0);
        locals.var_fn382_calc_iq__vdsats10 = assign32020_e29200;
        locals.var_fn382_calc_iq__vdsats10_dn4 = (((locals.var_fn382_calc_iq__vdsats0_dn4 * assign32020_e29197) + (locals.var_fn382_calc_iq__vdsats0 * ((((((((2.0 * locals.var_fn382_calc_iq__qinvv0_dn4) * locals.var_fn382_calc_iq__cgin) - (assign32020_e29191 * locals.var_fn382_calc_iq__cgin_dn4)) / (locals.var_fn382_calc_iq__cgin * locals.var_fn382_calc_iq__cgin)) * locals.var_fn382_calc_iq__vdsats0) - (assign32020_e29193 * locals.var_fn382_calc_iq__vdsats0_dn4)) / (locals.var_fn382_calc_iq__vdsats0 * locals.var_fn382_calc_iq__vdsats0)) / (2.0 * assign32020_e29197)))) - locals.var_fn382_calc_iq__vdsats0_dn4);
        locals.var_fn382_calc_iq__vdsats10_dn5 = (locals.var_fn382_calc_iq__vdsats0 * ((((2.0 * locals.var_fn382_calc_iq__qinvv0_dn5) / locals.var_fn382_calc_iq__cgin) / locals.var_fn382_calc_iq__vdsats0) / (2.0 * assign32020_e29197)));
        locals.var_fn382_calc_iq__vdsats10_dn8 = (locals.var_fn382_calc_iq__vdsats0 * ((((2.0 * locals.var_fn382_calc_iq__qinvv0_dn8) / locals.var_fn382_calc_iq__cgin) / locals.var_fn382_calc_iq__vdsats0) / (2.0 * assign32020_e29197)));
        locals.var_fn382_calc_iq__vdsats10_dn9 = (locals.var_fn382_calc_iq__vdsats0 * ((((2.0 * locals.var_fn382_calc_iq__qinvv0_dn9) / locals.var_fn382_calc_iq__cgin) / locals.var_fn382_calc_iq__vdsats0) / (2.0 * assign32020_e29197)));
        locals.var_fn382_calc_iq__vdsats10_rv = 0.0;

        let assign32030_e29204: f64 = (1.0 - locals.var_fn382_calc_iq__ff0);
        let assign32030_e29205: f64 = (locals.var_fn382_calc_iq__vdsats10 * assign32030_e29204);
        let assign32030_e29208: f64 = (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__ff0);
        let assign32030_e29209: f64 = (assign32030_e29205 + assign32030_e29208);
        locals.var_fn382_calc_iq__vdsat10 = assign32030_e29209;
        locals.var_fn382_calc_iq__vdsat10_dn4 = (((locals.var_fn382_calc_iq__vdsats10_dn4 * assign32030_e29204) + (locals.var_fn382_calc_iq__vdsats10 * (-locals.var_fn382_calc_iq__ff0_dn4))) + ((locals.var_fn382_calc_iq__two_n_phit0_dn4 * locals.var_fn382_calc_iq__ff0) + (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__ff0_dn4)));
        locals.var_fn382_calc_iq__vdsat10_dn5 = (((locals.var_fn382_calc_iq__vdsats10_dn5 * assign32030_e29204) + (locals.var_fn382_calc_iq__vdsats10 * (-locals.var_fn382_calc_iq__ff0_dn5))) + (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__ff0_dn5));
        locals.var_fn382_calc_iq__vdsat10_dn8 = (((locals.var_fn382_calc_iq__vdsats10_dn8 * assign32030_e29204) + (locals.var_fn382_calc_iq__vdsats10 * (-locals.var_fn382_calc_iq__ff0_dn8))) + (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__ff0_dn8));
        locals.var_fn382_calc_iq__vdsat10_dn9 = (((locals.var_fn382_calc_iq__vdsats10_dn9 * assign32030_e29204) + (locals.var_fn382_calc_iq__vdsats10 * (-locals.var_fn382_calc_iq__ff0_dn9))) + (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__ff0_dn9));
        locals.var_fn382_calc_iq__vdsat10_rv = 0.0;

        let (assign32040_e29267, assign32040_e29267_d_n4, assign32040_e29267_d_n5, assign32040_e29267_d_n8, assign32040_e29267_d_n9,) = {
    if (p.p52 != 0.0) {
        let assign32040_e29220: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat10);
        let assign32040_e29221: f64 = assign32040_e29220;
        let assign32040_e29225: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat10);
        let assign32040_e29226: f64 = (-assign32040_e29225);
        let assign32040_e29229: f64 = (0.001 / p.p53);
        let assign32040_e29233: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat10);
        let assign32040_e29234: f64 = (-assign32040_e29233);
        let assign32040_e29235: f64 = (assign32040_e29229 * assign32040_e29234);
        let assign32040_e29236: f64 = (assign32040_e29235).tanh();
        let assign32040_e29237: f64 = (assign32040_e29226 * assign32040_e29236);
        let assign32040_e29238: f64 = (assign32040_e29221 + assign32040_e29237);
        let assign32040_e29239: f64 = (0.5 * assign32040_e29238);
        (assign32040_e29239, (0.5 * ((-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) + (((-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) * assign32040_e29236) + (assign32040_e29226 * ((assign32040_e29229 * (-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))))) / ((assign32040_e29235).cosh() * (assign32040_e29235).cosh())))))), (0.5 * ((((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)) + (((-(((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) * assign32040_e29236) + (assign32040_e29226 * ((assign32040_e29229 * (-(((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) / ((assign32040_e29235).cosh() * (assign32040_e29235).cosh())))))), (0.5 * ((-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) + (((-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) * assign32040_e29236) + (assign32040_e29226 * ((assign32040_e29229 * (-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))))) / ((assign32040_e29235).cosh() * (assign32040_e29235).cosh())))))), (0.5 * ((((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)) + (((-(((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) * assign32040_e29236) + (assign32040_e29226 * ((assign32040_e29229 * (-(((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) / ((assign32040_e29235).cosh() * (assign32040_e29235).cosh())))))),)
    } else {
        let (assign32040_e29266, assign32040_e29266_d_n4, assign32040_e29266_d_n5, assign32040_e29266_d_n8, assign32040_e29266_d_n9,) = {
            if (p.p52 == 0.0) {
                let assign32040_e29247: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat10);
                let assign32040_e29248: f64 = assign32040_e29247;
                let assign32040_e29252: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat10);
                let assign32040_e29253: f64 = (-assign32040_e29252);
                let assign32040_e29257: f64 = (locals.var_fn382_calc_iq__vdsin / locals.var_fn382_calc_iq__vdsat10);
                let assign32040_e29258: f64 = (-assign32040_e29257);
                let assign32040_e29259: f64 = (assign32040_e29253 * assign32040_e29258);
                let assign32040_e29261: f64 = (assign32040_e29259 + p.p53);
                let assign32040_e29262: f64 = (assign32040_e29261).sqrt();
                let assign32040_e29263: f64 = (assign32040_e29248 + assign32040_e29262);
                let assign32040_e29264: f64 = (0.5 * assign32040_e29263);
                (assign32040_e29264, (0.5 * ((-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) + ((((-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) * assign32040_e29258) + (assign32040_e29253 * (-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))))) / (2.0 * assign32040_e29262)))), (0.5 * ((((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)) + ((((-(((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) * assign32040_e29258) + (assign32040_e29253 * (-(((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))))) / (2.0 * assign32040_e29262)))), (0.5 * ((-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) + ((((-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) * assign32040_e29258) + (assign32040_e29253 * (-(-((locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))))) / (2.0 * assign32040_e29262)))), (0.5 * ((((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)) + ((((-(((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) * assign32040_e29258) + (assign32040_e29253 * (-(((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__vdsat10) - (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))))) / (2.0 * assign32040_e29262)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign32040_e29266, assign32040_e29266_d_n4, assign32040_e29266_d_n5, assign32040_e29266_d_n8, assign32040_e29266_d_n9,)
    }
};
        let assign32040_e29269: f64 = (assign32040_e29267).powf(locals.var_fn382_calc_iq__beta);
        let assign32040_e29270: f64 = (1.0 + assign32040_e29269);
        let assign32040_e29273: f64 = (1.0 / locals.var_fn382_calc_iq__beta);
        let assign32040_e29274: f64 = (assign32040_e29270).powf(assign32040_e29273);
        let assign32040_e29275: f64 = (1.0 / assign32040_e29274);
        locals.var_fn382_calc_iq__fsd0 = assign32040_e29275;
        locals.var_fn382_calc_iq__fsd0_dn4 = (-(if 0.0 == 0.0 && ((assign32040_e29273) as f64).is_finite() && ((assign32040_e29273) as f64).fract() == 0.0 { if assign32040_e29273 == 0.0 { 0.0 } else { (assign32040_e29273 * ((assign32040_e29270).powf(assign32040_e29273 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32040_e29267).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32040_e29267_d_n4)) } } else { (assign32040_e29269 * (locals.var_fn382_calc_iq__beta * (assign32040_e29267_d_n4 / assign32040_e29267))) })) } } else { (assign32040_e29274 * (assign32040_e29273 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32040_e29267).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32040_e29267_d_n4)) } } else { (assign32040_e29269 * (locals.var_fn382_calc_iq__beta * (assign32040_e29267_d_n4 / assign32040_e29267))) } / assign32040_e29270))) } / (assign32040_e29274 * assign32040_e29274)));
        locals.var_fn382_calc_iq__fsd0_dn5 = (-(if 0.0 == 0.0 && ((assign32040_e29273) as f64).is_finite() && ((assign32040_e29273) as f64).fract() == 0.0 { if assign32040_e29273 == 0.0 { 0.0 } else { (assign32040_e29273 * ((assign32040_e29270).powf(assign32040_e29273 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32040_e29267).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32040_e29267_d_n5)) } } else { (assign32040_e29269 * (locals.var_fn382_calc_iq__beta * (assign32040_e29267_d_n5 / assign32040_e29267))) })) } } else { (assign32040_e29274 * (assign32040_e29273 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32040_e29267).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32040_e29267_d_n5)) } } else { (assign32040_e29269 * (locals.var_fn382_calc_iq__beta * (assign32040_e29267_d_n5 / assign32040_e29267))) } / assign32040_e29270))) } / (assign32040_e29274 * assign32040_e29274)));
        locals.var_fn382_calc_iq__fsd0_dn8 = (-(if 0.0 == 0.0 && ((assign32040_e29273) as f64).is_finite() && ((assign32040_e29273) as f64).fract() == 0.0 { if assign32040_e29273 == 0.0 { 0.0 } else { (assign32040_e29273 * ((assign32040_e29270).powf(assign32040_e29273 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32040_e29267).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32040_e29267_d_n8)) } } else { (assign32040_e29269 * (locals.var_fn382_calc_iq__beta * (assign32040_e29267_d_n8 / assign32040_e29267))) })) } } else { (assign32040_e29274 * (assign32040_e29273 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32040_e29267).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32040_e29267_d_n8)) } } else { (assign32040_e29269 * (locals.var_fn382_calc_iq__beta * (assign32040_e29267_d_n8 / assign32040_e29267))) } / assign32040_e29270))) } / (assign32040_e29274 * assign32040_e29274)));
        locals.var_fn382_calc_iq__fsd0_dn9 = (-(if 0.0 == 0.0 && ((assign32040_e29273) as f64).is_finite() && ((assign32040_e29273) as f64).fract() == 0.0 { if assign32040_e29273 == 0.0 { 0.0 } else { (assign32040_e29273 * ((assign32040_e29270).powf(assign32040_e29273 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32040_e29267).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32040_e29267_d_n9)) } } else { (assign32040_e29269 * (locals.var_fn382_calc_iq__beta * (assign32040_e29267_d_n9 / assign32040_e29267))) })) } } else { (assign32040_e29274 * (assign32040_e29273 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32040_e29267).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32040_e29267_d_n9)) } } else { (assign32040_e29269 * (locals.var_fn382_calc_iq__beta * (assign32040_e29267_d_n9 / assign32040_e29267))) } / assign32040_e29270))) } / (assign32040_e29274 * assign32040_e29274)));
        locals.var_fn382_calc_iq__fsd0_rv = 0.0;

        let assign32050_e29278: f64 = (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__fsd0);
        locals.var_fn382_calc_iq__vdx0 = assign32050_e29278;
        locals.var_fn382_calc_iq__vdx0_dn4 = (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__fsd0_dn4);
        locals.var_fn382_calc_iq__vdx0_dn5 = ((locals.var_fn382_calc_iq__vdsin_dn5 * locals.var_fn382_calc_iq__fsd0) + (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__fsd0_dn5));
        locals.var_fn382_calc_iq__vdx0_dn8 = (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__fsd0_dn8);
        locals.var_fn382_calc_iq__vdx0_dn9 = ((locals.var_fn382_calc_iq__vdsin_dn9 * locals.var_fn382_calc_iq__fsd0) + (locals.var_fn382_calc_iq__vdsin * locals.var_fn382_calc_iq__fsd0_dn9));
        locals.var_fn382_calc_iq__vdx0_rv = 0.0;

        let (assign32060_e29342, assign32060_e29342_d_n4, assign32060_e29342_d_n5, assign32060_e29342_d_n8, assign32060_e29342_d_n9,) = {
    if (p.p52 != 0.0) {
        let assign32060_e29288: f64 = (-locals.var_fn382_calc_iq__vdsin);
        let assign32060_e29290: f64 = (assign32060_e29288 / locals.var_fn382_calc_iq__vdsat10);
        let assign32060_e29291: f64 = assign32060_e29290;
        let assign32060_e29294: f64 = (-locals.var_fn382_calc_iq__vdsin);
        let assign32060_e29296: f64 = (assign32060_e29294 / locals.var_fn382_calc_iq__vdsat10);
        let assign32060_e29297: f64 = (-assign32060_e29296);
        let assign32060_e29300: f64 = (0.001 / p.p53);
        let assign32060_e29303: f64 = (-locals.var_fn382_calc_iq__vdsin);
        let assign32060_e29305: f64 = (assign32060_e29303 / locals.var_fn382_calc_iq__vdsat10);
        let assign32060_e29306: f64 = (-assign32060_e29305);
        let assign32060_e29307: f64 = (assign32060_e29300 * assign32060_e29306);
        let assign32060_e29308: f64 = (assign32060_e29307).tanh();
        let assign32060_e29309: f64 = (assign32060_e29297 * assign32060_e29308);
        let assign32060_e29310: f64 = (assign32060_e29291 + assign32060_e29309);
        let assign32060_e29311: f64 = (0.5 * assign32060_e29310);
        (assign32060_e29311, (0.5 * ((-((assign32060_e29288 * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) + (((-(-((assign32060_e29294 * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) * assign32060_e29308) + (assign32060_e29297 * ((assign32060_e29300 * (-(-((assign32060_e29303 * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))))) / ((assign32060_e29307).cosh() * (assign32060_e29307).cosh())))))), (0.5 * (((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29288 * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)) + (((-((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29294 * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) * assign32060_e29308) + (assign32060_e29297 * ((assign32060_e29300 * (-((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29303 * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) / ((assign32060_e29307).cosh() * (assign32060_e29307).cosh())))))), (0.5 * ((-((assign32060_e29288 * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) + (((-(-((assign32060_e29294 * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) * assign32060_e29308) + (assign32060_e29297 * ((assign32060_e29300 * (-(-((assign32060_e29303 * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))))) / ((assign32060_e29307).cosh() * (assign32060_e29307).cosh())))))), (0.5 * (((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29288 * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)) + (((-((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29294 * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) * assign32060_e29308) + (assign32060_e29297 * ((assign32060_e29300 * (-((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29303 * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) / ((assign32060_e29307).cosh() * (assign32060_e29307).cosh())))))),)
    } else {
        let (assign32060_e29341, assign32060_e29341_d_n4, assign32060_e29341_d_n5, assign32060_e29341_d_n8, assign32060_e29341_d_n9,) = {
            if (p.p52 == 0.0) {
                let assign32060_e29318: f64 = (-locals.var_fn382_calc_iq__vdsin);
                let assign32060_e29320: f64 = (assign32060_e29318 / locals.var_fn382_calc_iq__vdsat10);
                let assign32060_e29321: f64 = assign32060_e29320;
                let assign32060_e29324: f64 = (-locals.var_fn382_calc_iq__vdsin);
                let assign32060_e29326: f64 = (assign32060_e29324 / locals.var_fn382_calc_iq__vdsat10);
                let assign32060_e29327: f64 = (-assign32060_e29326);
                let assign32060_e29330: f64 = (-locals.var_fn382_calc_iq__vdsin);
                let assign32060_e29332: f64 = (assign32060_e29330 / locals.var_fn382_calc_iq__vdsat10);
                let assign32060_e29333: f64 = (-assign32060_e29332);
                let assign32060_e29334: f64 = (assign32060_e29327 * assign32060_e29333);
                let assign32060_e29336: f64 = (assign32060_e29334 + p.p53);
                let assign32060_e29337: f64 = (assign32060_e29336).sqrt();
                let assign32060_e29338: f64 = (assign32060_e29321 + assign32060_e29337);
                let assign32060_e29339: f64 = (0.5 * assign32060_e29338);
                (assign32060_e29339, (0.5 * ((-((assign32060_e29318 * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) + ((((-(-((assign32060_e29324 * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) * assign32060_e29333) + (assign32060_e29327 * (-(-((assign32060_e29330 * locals.var_fn382_calc_iq__vdsat10_dn4) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))))) / (2.0 * assign32060_e29337)))), (0.5 * (((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29318 * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)) + ((((-((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29324 * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) * assign32060_e29333) + (assign32060_e29327 * (-((((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29330 * locals.var_fn382_calc_iq__vdsat10_dn5)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))))) / (2.0 * assign32060_e29337)))), (0.5 * ((-((assign32060_e29318 * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) + ((((-(-((assign32060_e29324 * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))) * assign32060_e29333) + (assign32060_e29327 * (-(-((assign32060_e29330 * locals.var_fn382_calc_iq__vdsat10_dn8) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)))))) / (2.0 * assign32060_e29337)))), (0.5 * (((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29318 * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10)) + ((((-((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29324 * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))) * assign32060_e29333) + (assign32060_e29327 * (-((((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__vdsat10) - (assign32060_e29330 * locals.var_fn382_calc_iq__vdsat10_dn9)) / (locals.var_fn382_calc_iq__vdsat10 * locals.var_fn382_calc_iq__vdsat10))))) / (2.0 * assign32060_e29337)))),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign32060_e29341, assign32060_e29341_d_n4, assign32060_e29341_d_n5, assign32060_e29341_d_n8, assign32060_e29341_d_n9,)
    }
};
        let assign32060_e29344: f64 = (assign32060_e29342).powf(locals.var_fn382_calc_iq__beta);
        let assign32060_e29345: f64 = (1.0 + assign32060_e29344);
        let assign32060_e29348: f64 = (1.0 / locals.var_fn382_calc_iq__beta);
        let assign32060_e29349: f64 = (assign32060_e29345).powf(assign32060_e29348);
        let assign32060_e29350: f64 = (1.0 / assign32060_e29349);
        locals.var_fn382_calc_iq__fds0 = assign32060_e29350;
        locals.var_fn382_calc_iq__fds0_dn4 = (-(if 0.0 == 0.0 && ((assign32060_e29348) as f64).is_finite() && ((assign32060_e29348) as f64).fract() == 0.0 { if assign32060_e29348 == 0.0 { 0.0 } else { (assign32060_e29348 * ((assign32060_e29345).powf(assign32060_e29348 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32060_e29342).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32060_e29342_d_n4)) } } else { (assign32060_e29344 * (locals.var_fn382_calc_iq__beta * (assign32060_e29342_d_n4 / assign32060_e29342))) })) } } else { (assign32060_e29349 * (assign32060_e29348 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32060_e29342).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32060_e29342_d_n4)) } } else { (assign32060_e29344 * (locals.var_fn382_calc_iq__beta * (assign32060_e29342_d_n4 / assign32060_e29342))) } / assign32060_e29345))) } / (assign32060_e29349 * assign32060_e29349)));
        locals.var_fn382_calc_iq__fds0_dn5 = (-(if 0.0 == 0.0 && ((assign32060_e29348) as f64).is_finite() && ((assign32060_e29348) as f64).fract() == 0.0 { if assign32060_e29348 == 0.0 { 0.0 } else { (assign32060_e29348 * ((assign32060_e29345).powf(assign32060_e29348 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32060_e29342).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32060_e29342_d_n5)) } } else { (assign32060_e29344 * (locals.var_fn382_calc_iq__beta * (assign32060_e29342_d_n5 / assign32060_e29342))) })) } } else { (assign32060_e29349 * (assign32060_e29348 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32060_e29342).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32060_e29342_d_n5)) } } else { (assign32060_e29344 * (locals.var_fn382_calc_iq__beta * (assign32060_e29342_d_n5 / assign32060_e29342))) } / assign32060_e29345))) } / (assign32060_e29349 * assign32060_e29349)));
        locals.var_fn382_calc_iq__fds0_dn8 = (-(if 0.0 == 0.0 && ((assign32060_e29348) as f64).is_finite() && ((assign32060_e29348) as f64).fract() == 0.0 { if assign32060_e29348 == 0.0 { 0.0 } else { (assign32060_e29348 * ((assign32060_e29345).powf(assign32060_e29348 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32060_e29342).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32060_e29342_d_n8)) } } else { (assign32060_e29344 * (locals.var_fn382_calc_iq__beta * (assign32060_e29342_d_n8 / assign32060_e29342))) })) } } else { (assign32060_e29349 * (assign32060_e29348 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32060_e29342).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32060_e29342_d_n8)) } } else { (assign32060_e29344 * (locals.var_fn382_calc_iq__beta * (assign32060_e29342_d_n8 / assign32060_e29342))) } / assign32060_e29345))) } / (assign32060_e29349 * assign32060_e29349)));
        locals.var_fn382_calc_iq__fds0_dn9 = (-(if 0.0 == 0.0 && ((assign32060_e29348) as f64).is_finite() && ((assign32060_e29348) as f64).fract() == 0.0 { if assign32060_e29348 == 0.0 { 0.0 } else { (assign32060_e29348 * ((assign32060_e29345).powf(assign32060_e29348 - 1.0) * if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32060_e29342).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32060_e29342_d_n9)) } } else { (assign32060_e29344 * (locals.var_fn382_calc_iq__beta * (assign32060_e29342_d_n9 / assign32060_e29342))) })) } } else { (assign32060_e29349 * (assign32060_e29348 * (if 0.0 == 0.0 && ((locals.var_fn382_calc_iq__beta) as f64).is_finite() && ((locals.var_fn382_calc_iq__beta) as f64).fract() == 0.0 { if locals.var_fn382_calc_iq__beta == 0.0 { 0.0 } else { (locals.var_fn382_calc_iq__beta * ((assign32060_e29342).powf(locals.var_fn382_calc_iq__beta - 1.0) * assign32060_e29342_d_n9)) } } else { (assign32060_e29344 * (locals.var_fn382_calc_iq__beta * (assign32060_e29342_d_n9 / assign32060_e29342))) } / assign32060_e29345))) } / (assign32060_e29349 * assign32060_e29349)));
        locals.var_fn382_calc_iq__fds0_rv = 0.0;

        let assign32070_e29352: f64 = (-locals.var_fn382_calc_iq__vdsin);
        let assign32070_e29354: f64 = (assign32070_e29352 * locals.var_fn382_calc_iq__fds0);
        locals.var_fn382_calc_iq__vsx0 = assign32070_e29354;
        locals.var_fn382_calc_iq__vsx0_dn4 = (assign32070_e29352 * locals.var_fn382_calc_iq__fds0_dn4);
        locals.var_fn382_calc_iq__vsx0_dn5 = (((-locals.var_fn382_calc_iq__vdsin_dn5) * locals.var_fn382_calc_iq__fds0) + (assign32070_e29352 * locals.var_fn382_calc_iq__fds0_dn5));
        locals.var_fn382_calc_iq__vsx0_dn8 = (assign32070_e29352 * locals.var_fn382_calc_iq__fds0_dn8);
        locals.var_fn382_calc_iq__vsx0_dn9 = (((-locals.var_fn382_calc_iq__vdsin_dn9) * locals.var_fn382_calc_iq__fds0) + (assign32070_e29352 * locals.var_fn382_calc_iq__fds0_dn9));
        locals.var_fn382_calc_iq__vsx0_rv = 0.0;

        let assign32080_e29357: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__myarg0);
        let assign32080_e29359: f64 = (assign32080_e29357 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg0 = assign32080_e29359;
        locals.var_fn382_calc_iq__exparg0_dn4 = ((((-locals.var_fn382_calc_iq__myarg0_dn4) * locals.var_fn382_calc_iq__alpha_phit) - (assign32080_e29357 * locals.var_fn382_calc_iq__alpha_phit_dn4)) / (locals.var_fn382_calc_iq__alpha_phit * locals.var_fn382_calc_iq__alpha_phit));
        locals.var_fn382_calc_iq__exparg0_dn5 = 0.0;
        locals.var_fn382_calc_iq__exparg0_dn8 = (locals.var_fn382_calc_iq__vgsin_dn8 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg0_dn9 = (locals.var_fn382_calc_iq__vgsin_dn9 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg0_rv = 0.0;

        let assign32090_e29362: f64 = if locals.var_fn382_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard400 = assign32090_e29362;
        locals.var_guard400_rv = 0.0;

        let (assign32100_e29366, assign32100_e29366_d_n4, assign32100_e29366_d_n5, assign32100_e29366_d_n8, assign32100_e29366_d_n9,) = {
    if (locals.var_guard400 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ffs0, locals.var_fn382_calc_iq__ffs0_dn4, locals.var_fn382_calc_iq__ffs0_dn5, locals.var_fn382_calc_iq__ffs0_dn8, locals.var_fn382_calc_iq__ffs0_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffs0 = assign32100_e29366;
        locals.var_fn382_calc_iq__ffs0_dn4 = assign32100_e29366_d_n4;
        locals.var_fn382_calc_iq__ffs0_dn5 = assign32100_e29366_d_n5;
        locals.var_fn382_calc_iq__ffs0_dn8 = assign32100_e29366_d_n8;
        locals.var_fn382_calc_iq__ffs0_dn9 = assign32100_e29366_d_n9;
        locals.var_fn382_calc_iq__ffs0_rv = 0.0;

        let assign32110_e29369: f64 = (-50.0);
        let assign32110_e29370: f64 = if locals.var_fn382_calc_iq__exparg0 < assign32110_e29369 { 1.0 } else { 0.0 };
        locals.var_guard401 = assign32110_e29370;
        locals.var_guard401_rv = 0.0;

        let (assign32120_e29377, assign32120_e29377_d_n4, assign32120_e29377_d_n5, assign32120_e29377_d_n8, assign32120_e29377_d_n9,) = {
    if ((locals.var_guard400 == 0.0) && (locals.var_guard401 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ffs0, locals.var_fn382_calc_iq__ffs0_dn4, locals.var_fn382_calc_iq__ffs0_dn5, locals.var_fn382_calc_iq__ffs0_dn8, locals.var_fn382_calc_iq__ffs0_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffs0 = assign32120_e29377;
        locals.var_fn382_calc_iq__ffs0_dn4 = assign32120_e29377_d_n4;
        locals.var_fn382_calc_iq__ffs0_dn5 = assign32120_e29377_d_n5;
        locals.var_fn382_calc_iq__ffs0_dn8 = assign32120_e29377_d_n8;
        locals.var_fn382_calc_iq__ffs0_dn9 = assign32120_e29377_d_n9;
        locals.var_fn382_calc_iq__ffs0_rv = 0.0;

        let (assign32130_e29390, assign32130_e29390_d_n4, assign32130_e29390_d_n5, assign32130_e29390_d_n8, assign32130_e29390_d_n9,) = {
    if ((locals.var_guard400 == 0.0) && (locals.var_guard401 == 0.0)) {
        let assign32130_e29386: f64 = (locals.var_fn382_calc_iq__exparg0).exp();
        let assign32130_e29387: f64 = (1.0 + assign32130_e29386);
        let assign32130_e29388: f64 = (1.0 / assign32130_e29387);
        (assign32130_e29388, (-((assign32130_e29386 * locals.var_fn382_calc_iq__exparg0_dn4) / (assign32130_e29387 * assign32130_e29387))), (-((assign32130_e29386 * locals.var_fn382_calc_iq__exparg0_dn5) / (assign32130_e29387 * assign32130_e29387))), (-((assign32130_e29386 * locals.var_fn382_calc_iq__exparg0_dn8) / (assign32130_e29387 * assign32130_e29387))), (-((assign32130_e29386 * locals.var_fn382_calc_iq__exparg0_dn9) / (assign32130_e29387 * assign32130_e29387))),)
    } else {
        (locals.var_fn382_calc_iq__ffs0, locals.var_fn382_calc_iq__ffs0_dn4, locals.var_fn382_calc_iq__ffs0_dn5, locals.var_fn382_calc_iq__ffs0_dn8, locals.var_fn382_calc_iq__ffs0_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffs0 = assign32130_e29390;
        locals.var_fn382_calc_iq__ffs0_dn4 = assign32130_e29390_d_n4;
        locals.var_fn382_calc_iq__ffs0_dn5 = assign32130_e29390_d_n5;
        locals.var_fn382_calc_iq__ffs0_dn8 = assign32130_e29390_d_n8;
        locals.var_fn382_calc_iq__ffs0_dn9 = assign32130_e29390_d_n9;
        locals.var_fn382_calc_iq__ffs0_rv = 0.0;

        let assign32140_e29393: f64 = (locals.var_fn382_calc_iq__vgdin - locals.var_fn382_calc_iq__vsx0);
        let assign32140_e29397: f64 = (p.p51 * 0.1);
        let assign32140_e29399: f64 = (assign32140_e29397 * locals.var_fn382_calc_iq__alpha_phit);
        let assign32140_e29401: f64 = (assign32140_e29399 * locals.var_fn382_calc_iq__ffs0);
        let assign32140_e29402: f64 = (locals.var_fn382_calc_iq__vtof - assign32140_e29401);
        let assign32140_e29403: f64 = (assign32140_e29393 - assign32140_e29402);
        let assign32140_e29405: f64 = (assign32140_e29403 / locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__etas0 = assign32140_e29405;
        locals.var_fn382_calc_iq__etas0_dn4 = (((((-locals.var_fn382_calc_iq__vsx0_dn4) - (locals.var_fn382_calc_iq__vtof_dn4 - (((assign32140_e29397 * locals.var_fn382_calc_iq__alpha_phit_dn4) * locals.var_fn382_calc_iq__ffs0) + (assign32140_e29399 * locals.var_fn382_calc_iq__ffs0_dn4)))) * locals.var_fn382_calc_iq__two_n_phit0) - (assign32140_e29403 * locals.var_fn382_calc_iq__two_n_phit0_dn4)) / (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__two_n_phit0));
        locals.var_fn382_calc_iq__etas0_dn5 = (((locals.var_fn382_calc_iq__vgdin_dn5 - locals.var_fn382_calc_iq__vsx0_dn5) - (-(assign32140_e29399 * locals.var_fn382_calc_iq__ffs0_dn5))) / locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__etas0_dn8 = (((locals.var_fn382_calc_iq__vgdin_dn8 - locals.var_fn382_calc_iq__vsx0_dn8) - (-(assign32140_e29399 * locals.var_fn382_calc_iq__ffs0_dn8))) / locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__etas0_dn9 = (((locals.var_fn382_calc_iq__vgdin_dn9 - locals.var_fn382_calc_iq__vsx0_dn9) - (-(assign32140_e29399 * locals.var_fn382_calc_iq__ffs0_dn9))) / locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__etas0_rv = 0.0;

        let assign32150_e29408: f64 = if locals.var_fn382_calc_iq__etas0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard402 = assign32150_e29408;
        locals.var_guard402_rv = 0.0;

        let (assign32160_e29414, assign32160_e29414_d_n4, assign32160_e29414_d_n5, assign32160_e29414_d_n8, assign32160_e29414_d_n9,) = {
    if (locals.var_guard402 != 0.0) {
        let assign32160_e29412: f64 = (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__etas0);
        (assign32160_e29412, ((locals.var_fn382_calc_iq__qref0_dn4 * locals.var_fn382_calc_iq__etas0) + (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__etas0_dn4)), (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__etas0_dn5), (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__etas0_dn8), (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__etas0_dn9),)
    } else {
        (locals.var_fn382_calc_iq__qinvs0, locals.var_fn382_calc_iq__qinvs0_dn4, locals.var_fn382_calc_iq__qinvs0_dn5, locals.var_fn382_calc_iq__qinvs0_dn8, locals.var_fn382_calc_iq__qinvs0_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvs0 = assign32160_e29414;
        locals.var_fn382_calc_iq__qinvs0_dn4 = assign32160_e29414_d_n4;
        locals.var_fn382_calc_iq__qinvs0_dn5 = assign32160_e29414_d_n5;
        locals.var_fn382_calc_iq__qinvs0_dn8 = assign32160_e29414_d_n8;
        locals.var_fn382_calc_iq__qinvs0_dn9 = assign32160_e29414_d_n9;
        locals.var_fn382_calc_iq__qinvs0_rv = 0.0;

        let assign32170_e29417: f64 = (-50.0);
        let assign32170_e29418: f64 = if locals.var_fn382_calc_iq__etas0 < assign32170_e29417 { 1.0 } else { 0.0 };
        locals.var_guard403 = assign32170_e29418;
        locals.var_guard403_rv = 0.0;

        let (assign32180_e29428, assign32180_e29428_d_n4, assign32180_e29428_d_n5, assign32180_e29428_d_n8, assign32180_e29428_d_n9,) = {
    if ((locals.var_guard402 == 0.0) && (locals.var_guard403 != 0.0)) {
        let assign32180_e29425: f64 = (locals.var_fn382_calc_iq__etas0).exp();
        let assign32180_e29426: f64 = (locals.var_fn382_calc_iq__qref0 * assign32180_e29425);
        (assign32180_e29426, ((locals.var_fn382_calc_iq__qref0_dn4 * assign32180_e29425) + (locals.var_fn382_calc_iq__qref0 * (assign32180_e29425 * locals.var_fn382_calc_iq__etas0_dn4))), (locals.var_fn382_calc_iq__qref0 * (assign32180_e29425 * locals.var_fn382_calc_iq__etas0_dn5)), (locals.var_fn382_calc_iq__qref0 * (assign32180_e29425 * locals.var_fn382_calc_iq__etas0_dn8)), (locals.var_fn382_calc_iq__qref0 * (assign32180_e29425 * locals.var_fn382_calc_iq__etas0_dn9)),)
    } else {
        (locals.var_fn382_calc_iq__qinvs0, locals.var_fn382_calc_iq__qinvs0_dn4, locals.var_fn382_calc_iq__qinvs0_dn5, locals.var_fn382_calc_iq__qinvs0_dn8, locals.var_fn382_calc_iq__qinvs0_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvs0 = assign32180_e29428;
        locals.var_fn382_calc_iq__qinvs0_dn4 = assign32180_e29428_d_n4;
        locals.var_fn382_calc_iq__qinvs0_dn5 = assign32180_e29428_d_n5;
        locals.var_fn382_calc_iq__qinvs0_dn8 = assign32180_e29428_d_n8;
        locals.var_fn382_calc_iq__qinvs0_dn9 = assign32180_e29428_d_n9;
        locals.var_fn382_calc_iq__qinvs0_rv = 0.0;

        let (assign32190_e29442, assign32190_e29442_d_n4, assign32190_e29442_d_n5, assign32190_e29442_d_n8, assign32190_e29442_d_n9,) = {
    if ((locals.var_guard402 == 0.0) && (locals.var_guard403 == 0.0)) {
        let assign32190_e29437: f64 = (locals.var_fn382_calc_iq__etas0).exp();
        let assign32190_e29438: f64 = (1.0 + assign32190_e29437);
        let assign32190_e29439: f64 = (assign32190_e29438).ln();
        let assign32190_e29440: f64 = (locals.var_fn382_calc_iq__qref0 * assign32190_e29439);
        (assign32190_e29440, ((locals.var_fn382_calc_iq__qref0_dn4 * assign32190_e29439) + (locals.var_fn382_calc_iq__qref0 * ((assign32190_e29437 * locals.var_fn382_calc_iq__etas0_dn4) / assign32190_e29438))), (locals.var_fn382_calc_iq__qref0 * ((assign32190_e29437 * locals.var_fn382_calc_iq__etas0_dn5) / assign32190_e29438)), (locals.var_fn382_calc_iq__qref0 * ((assign32190_e29437 * locals.var_fn382_calc_iq__etas0_dn8) / assign32190_e29438)), (locals.var_fn382_calc_iq__qref0 * ((assign32190_e29437 * locals.var_fn382_calc_iq__etas0_dn9) / assign32190_e29438)),)
    } else {
        (locals.var_fn382_calc_iq__qinvs0, locals.var_fn382_calc_iq__qinvs0_dn4, locals.var_fn382_calc_iq__qinvs0_dn5, locals.var_fn382_calc_iq__qinvs0_dn8, locals.var_fn382_calc_iq__qinvs0_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvs0 = assign32190_e29442;
        locals.var_fn382_calc_iq__qinvs0_dn4 = assign32190_e29442_d_n4;
        locals.var_fn382_calc_iq__qinvs0_dn5 = assign32190_e29442_d_n5;
        locals.var_fn382_calc_iq__qinvs0_dn8 = assign32190_e29442_d_n8;
        locals.var_fn382_calc_iq__qinvs0_dn9 = assign32190_e29442_d_n9;
        locals.var_fn382_calc_iq__qinvs0_rv = 0.0;

        let assign32200_e29445: f64 = (locals.var_fn382_calc_iq__vgdin - locals.var_fn382_calc_iq__myarg0);
        let assign32200_e29447: f64 = (assign32200_e29445 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg0 = assign32200_e29447;
        locals.var_fn382_calc_iq__exparg0_dn4 = ((((-locals.var_fn382_calc_iq__myarg0_dn4) * locals.var_fn382_calc_iq__alpha_phit) - (assign32200_e29445 * locals.var_fn382_calc_iq__alpha_phit_dn4)) / (locals.var_fn382_calc_iq__alpha_phit * locals.var_fn382_calc_iq__alpha_phit));
        locals.var_fn382_calc_iq__exparg0_dn5 = (locals.var_fn382_calc_iq__vgdin_dn5 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg0_dn8 = (locals.var_fn382_calc_iq__vgdin_dn8 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg0_dn9 = (locals.var_fn382_calc_iq__vgdin_dn9 / locals.var_fn382_calc_iq__alpha_phit);
        locals.var_fn382_calc_iq__exparg0_rv = 0.0;

        let assign32210_e29450: f64 = if locals.var_fn382_calc_iq__exparg0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard404 = assign32210_e29450;
        locals.var_guard404_rv = 0.0;

        let (assign32220_e29454, assign32220_e29454_d_n4, assign32220_e29454_d_n5, assign32220_e29454_d_n8, assign32220_e29454_d_n9,) = {
    if (locals.var_guard404 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ffd0, locals.var_fn382_calc_iq__ffd0_dn4, locals.var_fn382_calc_iq__ffd0_dn5, locals.var_fn382_calc_iq__ffd0_dn8, locals.var_fn382_calc_iq__ffd0_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffd0 = assign32220_e29454;
        locals.var_fn382_calc_iq__ffd0_dn4 = assign32220_e29454_d_n4;
        locals.var_fn382_calc_iq__ffd0_dn5 = assign32220_e29454_d_n5;
        locals.var_fn382_calc_iq__ffd0_dn8 = assign32220_e29454_d_n8;
        locals.var_fn382_calc_iq__ffd0_dn9 = assign32220_e29454_d_n9;
        locals.var_fn382_calc_iq__ffd0_rv = 0.0;

        let assign32230_e29457: f64 = (-50.0);
        let assign32230_e29458: f64 = if locals.var_fn382_calc_iq__exparg0 < assign32230_e29457 { 1.0 } else { 0.0 };
        locals.var_guard405 = assign32230_e29458;
        locals.var_guard405_rv = 0.0;

        let (assign32240_e29465, assign32240_e29465_d_n4, assign32240_e29465_d_n5, assign32240_e29465_d_n8, assign32240_e29465_d_n9,) = {
    if ((locals.var_guard404 == 0.0) && (locals.var_guard405 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__ffd0, locals.var_fn382_calc_iq__ffd0_dn4, locals.var_fn382_calc_iq__ffd0_dn5, locals.var_fn382_calc_iq__ffd0_dn8, locals.var_fn382_calc_iq__ffd0_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffd0 = assign32240_e29465;
        locals.var_fn382_calc_iq__ffd0_dn4 = assign32240_e29465_d_n4;
        locals.var_fn382_calc_iq__ffd0_dn5 = assign32240_e29465_d_n5;
        locals.var_fn382_calc_iq__ffd0_dn8 = assign32240_e29465_d_n8;
        locals.var_fn382_calc_iq__ffd0_dn9 = assign32240_e29465_d_n9;
        locals.var_fn382_calc_iq__ffd0_rv = 0.0;

        let (assign32250_e29478, assign32250_e29478_d_n4, assign32250_e29478_d_n5, assign32250_e29478_d_n8, assign32250_e29478_d_n9,) = {
    if ((locals.var_guard404 == 0.0) && (locals.var_guard405 == 0.0)) {
        let assign32250_e29474: f64 = (locals.var_fn382_calc_iq__exparg0).exp();
        let assign32250_e29475: f64 = (1.0 + assign32250_e29474);
        let assign32250_e29476: f64 = (1.0 / assign32250_e29475);
        (assign32250_e29476, (-((assign32250_e29474 * locals.var_fn382_calc_iq__exparg0_dn4) / (assign32250_e29475 * assign32250_e29475))), (-((assign32250_e29474 * locals.var_fn382_calc_iq__exparg0_dn5) / (assign32250_e29475 * assign32250_e29475))), (-((assign32250_e29474 * locals.var_fn382_calc_iq__exparg0_dn8) / (assign32250_e29475 * assign32250_e29475))), (-((assign32250_e29474 * locals.var_fn382_calc_iq__exparg0_dn9) / (assign32250_e29475 * assign32250_e29475))),)
    } else {
        (locals.var_fn382_calc_iq__ffd0, locals.var_fn382_calc_iq__ffd0_dn4, locals.var_fn382_calc_iq__ffd0_dn5, locals.var_fn382_calc_iq__ffd0_dn8, locals.var_fn382_calc_iq__ffd0_dn9,)
    }
};
        locals.var_fn382_calc_iq__ffd0 = assign32250_e29478;
        locals.var_fn382_calc_iq__ffd0_dn4 = assign32250_e29478_d_n4;
        locals.var_fn382_calc_iq__ffd0_dn5 = assign32250_e29478_d_n5;
        locals.var_fn382_calc_iq__ffd0_dn8 = assign32250_e29478_d_n8;
        locals.var_fn382_calc_iq__ffd0_dn9 = assign32250_e29478_d_n9;
        locals.var_fn382_calc_iq__ffd0_rv = 0.0;

        let assign32260_e29481: f64 = (locals.var_fn382_calc_iq__vgsin - locals.var_fn382_calc_iq__vdx0);
        let assign32260_e29485: f64 = (p.p51 * 0.1);
        let assign32260_e29487: f64 = (assign32260_e29485 * locals.var_fn382_calc_iq__alpha_phit);
        let assign32260_e29489: f64 = (assign32260_e29487 * locals.var_fn382_calc_iq__ffd0);
        let assign32260_e29490: f64 = (locals.var_fn382_calc_iq__vtof - assign32260_e29489);
        let assign32260_e29491: f64 = (assign32260_e29481 - assign32260_e29490);
        let assign32260_e29493: f64 = (assign32260_e29491 / locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__etad0 = assign32260_e29493;
        locals.var_fn382_calc_iq__etad0_dn4 = (((((-locals.var_fn382_calc_iq__vdx0_dn4) - (locals.var_fn382_calc_iq__vtof_dn4 - (((assign32260_e29485 * locals.var_fn382_calc_iq__alpha_phit_dn4) * locals.var_fn382_calc_iq__ffd0) + (assign32260_e29487 * locals.var_fn382_calc_iq__ffd0_dn4)))) * locals.var_fn382_calc_iq__two_n_phit0) - (assign32260_e29491 * locals.var_fn382_calc_iq__two_n_phit0_dn4)) / (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__two_n_phit0));
        locals.var_fn382_calc_iq__etad0_dn5 = (((-locals.var_fn382_calc_iq__vdx0_dn5) - (-(assign32260_e29487 * locals.var_fn382_calc_iq__ffd0_dn5))) / locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__etad0_dn8 = (((locals.var_fn382_calc_iq__vgsin_dn8 - locals.var_fn382_calc_iq__vdx0_dn8) - (-(assign32260_e29487 * locals.var_fn382_calc_iq__ffd0_dn8))) / locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__etad0_dn9 = (((locals.var_fn382_calc_iq__vgsin_dn9 - locals.var_fn382_calc_iq__vdx0_dn9) - (-(assign32260_e29487 * locals.var_fn382_calc_iq__ffd0_dn9))) / locals.var_fn382_calc_iq__two_n_phit0);
        locals.var_fn382_calc_iq__etad0_rv = 0.0;

        let assign32270_e29496: f64 = if locals.var_fn382_calc_iq__etad0 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard406 = assign32270_e29496;
        locals.var_guard406_rv = 0.0;

        let (assign32280_e29502, assign32280_e29502_d_n4, assign32280_e29502_d_n5, assign32280_e29502_d_n8, assign32280_e29502_d_n9,) = {
    if (locals.var_guard406 != 0.0) {
        let assign32280_e29500: f64 = (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__etad0);
        (assign32280_e29500, ((locals.var_fn382_calc_iq__qref0_dn4 * locals.var_fn382_calc_iq__etad0) + (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__etad0_dn4)), (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__etad0_dn5), (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__etad0_dn8), (locals.var_fn382_calc_iq__qref0 * locals.var_fn382_calc_iq__etad0_dn9),)
    } else {
        (locals.var_fn382_calc_iq__qinvd0, locals.var_fn382_calc_iq__qinvd0_dn4, locals.var_fn382_calc_iq__qinvd0_dn5, locals.var_fn382_calc_iq__qinvd0_dn8, locals.var_fn382_calc_iq__qinvd0_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvd0 = assign32280_e29502;
        locals.var_fn382_calc_iq__qinvd0_dn4 = assign32280_e29502_d_n4;
        locals.var_fn382_calc_iq__qinvd0_dn5 = assign32280_e29502_d_n5;
        locals.var_fn382_calc_iq__qinvd0_dn8 = assign32280_e29502_d_n8;
        locals.var_fn382_calc_iq__qinvd0_dn9 = assign32280_e29502_d_n9;
        locals.var_fn382_calc_iq__qinvd0_rv = 0.0;

        let assign32290_e29505: f64 = (-50.0);
        let assign32290_e29506: f64 = if locals.var_fn382_calc_iq__etad0 < assign32290_e29505 { 1.0 } else { 0.0 };
        locals.var_guard407 = assign32290_e29506;
        locals.var_guard407_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_71(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign32300_e29516, assign32300_e29516_d_n4, assign32300_e29516_d_n5, assign32300_e29516_d_n8, assign32300_e29516_d_n9,) = {
    if ((locals.var_guard406 == 0.0) && (locals.var_guard407 != 0.0)) {
        let assign32300_e29513: f64 = (locals.var_fn382_calc_iq__etad0).exp();
        let assign32300_e29514: f64 = (locals.var_fn382_calc_iq__qref0 * assign32300_e29513);
        (assign32300_e29514, ((locals.var_fn382_calc_iq__qref0_dn4 * assign32300_e29513) + (locals.var_fn382_calc_iq__qref0 * (assign32300_e29513 * locals.var_fn382_calc_iq__etad0_dn4))), (locals.var_fn382_calc_iq__qref0 * (assign32300_e29513 * locals.var_fn382_calc_iq__etad0_dn5)), (locals.var_fn382_calc_iq__qref0 * (assign32300_e29513 * locals.var_fn382_calc_iq__etad0_dn8)), (locals.var_fn382_calc_iq__qref0 * (assign32300_e29513 * locals.var_fn382_calc_iq__etad0_dn9)),)
    } else {
        (locals.var_fn382_calc_iq__qinvd0, locals.var_fn382_calc_iq__qinvd0_dn4, locals.var_fn382_calc_iq__qinvd0_dn5, locals.var_fn382_calc_iq__qinvd0_dn8, locals.var_fn382_calc_iq__qinvd0_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvd0 = assign32300_e29516;
        locals.var_fn382_calc_iq__qinvd0_dn4 = assign32300_e29516_d_n4;
        locals.var_fn382_calc_iq__qinvd0_dn5 = assign32300_e29516_d_n5;
        locals.var_fn382_calc_iq__qinvd0_dn8 = assign32300_e29516_d_n8;
        locals.var_fn382_calc_iq__qinvd0_dn9 = assign32300_e29516_d_n9;
        locals.var_fn382_calc_iq__qinvd0_rv = 0.0;

        let (assign32310_e29530, assign32310_e29530_d_n4, assign32310_e29530_d_n5, assign32310_e29530_d_n8, assign32310_e29530_d_n9,) = {
    if ((locals.var_guard406 == 0.0) && (locals.var_guard407 == 0.0)) {
        let assign32310_e29525: f64 = (locals.var_fn382_calc_iq__etad0).exp();
        let assign32310_e29526: f64 = (1.0 + assign32310_e29525);
        let assign32310_e29527: f64 = (assign32310_e29526).ln();
        let assign32310_e29528: f64 = (locals.var_fn382_calc_iq__qref0 * assign32310_e29527);
        (assign32310_e29528, ((locals.var_fn382_calc_iq__qref0_dn4 * assign32310_e29527) + (locals.var_fn382_calc_iq__qref0 * ((assign32310_e29525 * locals.var_fn382_calc_iq__etad0_dn4) / assign32310_e29526))), (locals.var_fn382_calc_iq__qref0 * ((assign32310_e29525 * locals.var_fn382_calc_iq__etad0_dn5) / assign32310_e29526)), (locals.var_fn382_calc_iq__qref0 * ((assign32310_e29525 * locals.var_fn382_calc_iq__etad0_dn8) / assign32310_e29526)), (locals.var_fn382_calc_iq__qref0 * ((assign32310_e29525 * locals.var_fn382_calc_iq__etad0_dn9) / assign32310_e29526)),)
    } else {
        (locals.var_fn382_calc_iq__qinvd0, locals.var_fn382_calc_iq__qinvd0_dn4, locals.var_fn382_calc_iq__qinvd0_dn5, locals.var_fn382_calc_iq__qinvd0_dn8, locals.var_fn382_calc_iq__qinvd0_dn9,)
    }
};
        locals.var_fn382_calc_iq__qinvd0 = assign32310_e29530;
        locals.var_fn382_calc_iq__qinvd0_dn4 = assign32310_e29530_d_n4;
        locals.var_fn382_calc_iq__qinvd0_dn5 = assign32310_e29530_d_n5;
        locals.var_fn382_calc_iq__qinvd0_dn8 = assign32310_e29530_d_n8;
        locals.var_fn382_calc_iq__qinvd0_dn9 = assign32310_e29530_d_n9;
        locals.var_fn382_calc_iq__qinvd0_rv = 0.0;

        let assign32320_e29533: f64 = (locals.var_fn382_calc_iq__qinvs0 * locals.var_fn382_calc_iq__qinvs0);
        let assign32320_e29535: f64 = (assign32320_e29533 + 1e-38);
        locals.var_fn382_calc_iq__qs2 = assign32320_e29535;
        locals.var_fn382_calc_iq__qs2_dn4 = ((locals.var_fn382_calc_iq__qinvs0_dn4 * locals.var_fn382_calc_iq__qinvs0) + (locals.var_fn382_calc_iq__qinvs0 * locals.var_fn382_calc_iq__qinvs0_dn4));
        locals.var_fn382_calc_iq__qs2_dn5 = ((locals.var_fn382_calc_iq__qinvs0_dn5 * locals.var_fn382_calc_iq__qinvs0) + (locals.var_fn382_calc_iq__qinvs0 * locals.var_fn382_calc_iq__qinvs0_dn5));
        locals.var_fn382_calc_iq__qs2_dn8 = ((locals.var_fn382_calc_iq__qinvs0_dn8 * locals.var_fn382_calc_iq__qinvs0) + (locals.var_fn382_calc_iq__qinvs0 * locals.var_fn382_calc_iq__qinvs0_dn8));
        locals.var_fn382_calc_iq__qs2_dn9 = ((locals.var_fn382_calc_iq__qinvs0_dn9 * locals.var_fn382_calc_iq__qinvs0) + (locals.var_fn382_calc_iq__qinvs0 * locals.var_fn382_calc_iq__qinvs0_dn9));
        locals.var_fn382_calc_iq__qs2_rv = 0.0;

        let assign32330_e29538: f64 = (locals.var_fn382_calc_iq__qs2 * locals.var_fn382_calc_iq__qinvs0);
        let assign32330_e29540: f64 = (assign32330_e29538 + 1e-57);
        locals.var_fn382_calc_iq__qs3 = assign32330_e29540;
        locals.var_fn382_calc_iq__qs3_dn4 = ((locals.var_fn382_calc_iq__qs2_dn4 * locals.var_fn382_calc_iq__qinvs0) + (locals.var_fn382_calc_iq__qs2 * locals.var_fn382_calc_iq__qinvs0_dn4));
        locals.var_fn382_calc_iq__qs3_dn5 = ((locals.var_fn382_calc_iq__qs2_dn5 * locals.var_fn382_calc_iq__qinvs0) + (locals.var_fn382_calc_iq__qs2 * locals.var_fn382_calc_iq__qinvs0_dn5));
        locals.var_fn382_calc_iq__qs3_dn8 = ((locals.var_fn382_calc_iq__qs2_dn8 * locals.var_fn382_calc_iq__qinvs0) + (locals.var_fn382_calc_iq__qs2 * locals.var_fn382_calc_iq__qinvs0_dn8));
        locals.var_fn382_calc_iq__qs3_dn9 = ((locals.var_fn382_calc_iq__qs2_dn9 * locals.var_fn382_calc_iq__qinvs0) + (locals.var_fn382_calc_iq__qs2 * locals.var_fn382_calc_iq__qinvs0_dn9));
        locals.var_fn382_calc_iq__qs3_rv = 0.0;

        let assign32340_e29543: f64 = (locals.var_fn382_calc_iq__qinvd0 * locals.var_fn382_calc_iq__qinvd0);
        let assign32340_e29545: f64 = (assign32340_e29543 + 1e-38);
        locals.var_fn382_calc_iq__qd2 = assign32340_e29545;
        locals.var_fn382_calc_iq__qd2_dn4 = ((locals.var_fn382_calc_iq__qinvd0_dn4 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qinvd0 * locals.var_fn382_calc_iq__qinvd0_dn4));
        locals.var_fn382_calc_iq__qd2_dn5 = ((locals.var_fn382_calc_iq__qinvd0_dn5 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qinvd0 * locals.var_fn382_calc_iq__qinvd0_dn5));
        locals.var_fn382_calc_iq__qd2_dn8 = ((locals.var_fn382_calc_iq__qinvd0_dn8 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qinvd0 * locals.var_fn382_calc_iq__qinvd0_dn8));
        locals.var_fn382_calc_iq__qd2_dn9 = ((locals.var_fn382_calc_iq__qinvd0_dn9 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qinvd0 * locals.var_fn382_calc_iq__qinvd0_dn9));
        locals.var_fn382_calc_iq__qd2_rv = 0.0;

        let assign32350_e29548: f64 = (locals.var_fn382_calc_iq__qd2 * locals.var_fn382_calc_iq__qinvd0);
        let assign32350_e29550: f64 = (assign32350_e29548 + 1e-57);
        locals.var_fn382_calc_iq__qd3 = assign32350_e29550;
        locals.var_fn382_calc_iq__qd3_dn4 = ((locals.var_fn382_calc_iq__qd2_dn4 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qd2 * locals.var_fn382_calc_iq__qinvd0_dn4));
        locals.var_fn382_calc_iq__qd3_dn5 = ((locals.var_fn382_calc_iq__qd2_dn5 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qd2 * locals.var_fn382_calc_iq__qinvd0_dn5));
        locals.var_fn382_calc_iq__qd3_dn8 = ((locals.var_fn382_calc_iq__qd2_dn8 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qd2 * locals.var_fn382_calc_iq__qinvd0_dn8));
        locals.var_fn382_calc_iq__qd3_dn9 = ((locals.var_fn382_calc_iq__qd2_dn9 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qd2 * locals.var_fn382_calc_iq__qinvd0_dn9));
        locals.var_fn382_calc_iq__qd3_rv = 0.0;

        let assign32360_e29553: f64 = (locals.var_fn382_calc_iq__qinvs0 * locals.var_fn382_calc_iq__qinvd0);
        let assign32360_e29555: f64 = (assign32360_e29553 + 1e-38);
        locals.var_fn382_calc_iq__qsqd = assign32360_e29555;
        locals.var_fn382_calc_iq__qsqd_dn4 = ((locals.var_fn382_calc_iq__qinvs0_dn4 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qinvs0 * locals.var_fn382_calc_iq__qinvd0_dn4));
        locals.var_fn382_calc_iq__qsqd_dn5 = ((locals.var_fn382_calc_iq__qinvs0_dn5 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qinvs0 * locals.var_fn382_calc_iq__qinvd0_dn5));
        locals.var_fn382_calc_iq__qsqd_dn8 = ((locals.var_fn382_calc_iq__qinvs0_dn8 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qinvs0 * locals.var_fn382_calc_iq__qinvd0_dn8));
        locals.var_fn382_calc_iq__qsqd_dn9 = ((locals.var_fn382_calc_iq__qinvs0_dn9 * locals.var_fn382_calc_iq__qinvd0) + (locals.var_fn382_calc_iq__qinvs0 * locals.var_fn382_calc_iq__qinvd0_dn9));
        locals.var_fn382_calc_iq__qsqd_rv = 0.0;

        let assign32370_e29558: f64 = (2.0 / 3.0);
        let assign32370_e29561: f64 = (locals.var_fn382_calc_iq__qs2 + locals.var_fn382_calc_iq__qd2);
        let assign32370_e29563: f64 = (assign32370_e29561 + locals.var_fn382_calc_iq__qsqd);
        let assign32370_e29564: f64 = (assign32370_e29558 * assign32370_e29563);
        let assign32370_e29567: f64 = (locals.var_fn382_calc_iq__qinvs0 + locals.var_fn382_calc_iq__qinvd0);
        let assign32370_e29569: f64 = (assign32370_e29567 + 2e-19);
        let assign32370_e29570: f64 = (assign32370_e29564 / assign32370_e29569);
        locals.var_fn382_calc_iq__qinvdd = assign32370_e29570;
        locals.var_fn382_calc_iq__qinvdd_dn4 = ((((assign32370_e29558 * ((locals.var_fn382_calc_iq__qs2_dn4 + locals.var_fn382_calc_iq__qd2_dn4) + locals.var_fn382_calc_iq__qsqd_dn4)) * assign32370_e29569) - (assign32370_e29564 * (locals.var_fn382_calc_iq__qinvs0_dn4 + locals.var_fn382_calc_iq__qinvd0_dn4))) / (assign32370_e29569 * assign32370_e29569));
        locals.var_fn382_calc_iq__qinvdd_dn5 = ((((assign32370_e29558 * ((locals.var_fn382_calc_iq__qs2_dn5 + locals.var_fn382_calc_iq__qd2_dn5) + locals.var_fn382_calc_iq__qsqd_dn5)) * assign32370_e29569) - (assign32370_e29564 * (locals.var_fn382_calc_iq__qinvs0_dn5 + locals.var_fn382_calc_iq__qinvd0_dn5))) / (assign32370_e29569 * assign32370_e29569));
        locals.var_fn382_calc_iq__qinvdd_dn8 = ((((assign32370_e29558 * ((locals.var_fn382_calc_iq__qs2_dn8 + locals.var_fn382_calc_iq__qd2_dn8) + locals.var_fn382_calc_iq__qsqd_dn8)) * assign32370_e29569) - (assign32370_e29564 * (locals.var_fn382_calc_iq__qinvs0_dn8 + locals.var_fn382_calc_iq__qinvd0_dn8))) / (assign32370_e29569 * assign32370_e29569));
        locals.var_fn382_calc_iq__qinvdd_dn9 = ((((assign32370_e29558 * ((locals.var_fn382_calc_iq__qs2_dn9 + locals.var_fn382_calc_iq__qd2_dn9) + locals.var_fn382_calc_iq__qsqd_dn9)) * assign32370_e29569) - (assign32370_e29564 * (locals.var_fn382_calc_iq__qinvs0_dn9 + locals.var_fn382_calc_iq__qinvd0_dn9))) / (assign32370_e29569 * assign32370_e29569));
        locals.var_fn382_calc_iq__qinvdd_rv = 0.0;

        let assign32380_e29574: f64 = (2.0 * locals.var_fn382_calc_iq__qs3);
        let assign32380_e29577: f64 = (3.0 * locals.var_fn382_calc_iq__qd3);
        let assign32380_e29578: f64 = (assign32380_e29574 + assign32380_e29577);
        let assign32380_e29581: f64 = (4.0 * locals.var_fn382_calc_iq__qs2);
        let assign32380_e29583: f64 = (assign32380_e29581 * locals.var_fn382_calc_iq__qinvd0);
        let assign32380_e29584: f64 = (assign32380_e29578 + assign32380_e29583);
        let assign32380_e29587: f64 = (6.0 * locals.var_fn382_calc_iq__qd2);
        let assign32380_e29589: f64 = (assign32380_e29587 * locals.var_fn382_calc_iq__qinvs0);
        let assign32380_e29590: f64 = (assign32380_e29584 + assign32380_e29589);
        let assign32380_e29591: f64 = (2.0 * assign32380_e29590);
        let assign32380_e29595: f64 = (locals.var_fn382_calc_iq__qs2 + locals.var_fn382_calc_iq__qd2);
        let assign32380_e29598: f64 = (2.0 * locals.var_fn382_calc_iq__qsqd);
        let assign32380_e29599: f64 = (assign32380_e29595 + assign32380_e29598);
        let assign32380_e29600: f64 = (15.0 * assign32380_e29599);
        let assign32380_e29601: f64 = (assign32380_e29591 / assign32380_e29600);
        locals.var_fn382_calc_iq__qd1 = assign32380_e29601;
        locals.var_fn382_calc_iq__qd1_dn4 = ((((2.0 * ((((2.0 * locals.var_fn382_calc_iq__qs3_dn4) + (3.0 * locals.var_fn382_calc_iq__qd3_dn4)) + (((4.0 * locals.var_fn382_calc_iq__qs2_dn4) * locals.var_fn382_calc_iq__qinvd0) + (assign32380_e29581 * locals.var_fn382_calc_iq__qinvd0_dn4))) + (((6.0 * locals.var_fn382_calc_iq__qd2_dn4) * locals.var_fn382_calc_iq__qinvs0) + (assign32380_e29587 * locals.var_fn382_calc_iq__qinvs0_dn4)))) * assign32380_e29600) - (assign32380_e29591 * (15.0 * ((locals.var_fn382_calc_iq__qs2_dn4 + locals.var_fn382_calc_iq__qd2_dn4) + (2.0 * locals.var_fn382_calc_iq__qsqd_dn4))))) / (assign32380_e29600 * assign32380_e29600));
        locals.var_fn382_calc_iq__qd1_dn5 = ((((2.0 * ((((2.0 * locals.var_fn382_calc_iq__qs3_dn5) + (3.0 * locals.var_fn382_calc_iq__qd3_dn5)) + (((4.0 * locals.var_fn382_calc_iq__qs2_dn5) * locals.var_fn382_calc_iq__qinvd0) + (assign32380_e29581 * locals.var_fn382_calc_iq__qinvd0_dn5))) + (((6.0 * locals.var_fn382_calc_iq__qd2_dn5) * locals.var_fn382_calc_iq__qinvs0) + (assign32380_e29587 * locals.var_fn382_calc_iq__qinvs0_dn5)))) * assign32380_e29600) - (assign32380_e29591 * (15.0 * ((locals.var_fn382_calc_iq__qs2_dn5 + locals.var_fn382_calc_iq__qd2_dn5) + (2.0 * locals.var_fn382_calc_iq__qsqd_dn5))))) / (assign32380_e29600 * assign32380_e29600));
        locals.var_fn382_calc_iq__qd1_dn8 = ((((2.0 * ((((2.0 * locals.var_fn382_calc_iq__qs3_dn8) + (3.0 * locals.var_fn382_calc_iq__qd3_dn8)) + (((4.0 * locals.var_fn382_calc_iq__qs2_dn8) * locals.var_fn382_calc_iq__qinvd0) + (assign32380_e29581 * locals.var_fn382_calc_iq__qinvd0_dn8))) + (((6.0 * locals.var_fn382_calc_iq__qd2_dn8) * locals.var_fn382_calc_iq__qinvs0) + (assign32380_e29587 * locals.var_fn382_calc_iq__qinvs0_dn8)))) * assign32380_e29600) - (assign32380_e29591 * (15.0 * ((locals.var_fn382_calc_iq__qs2_dn8 + locals.var_fn382_calc_iq__qd2_dn8) + (2.0 * locals.var_fn382_calc_iq__qsqd_dn8))))) / (assign32380_e29600 * assign32380_e29600));
        locals.var_fn382_calc_iq__qd1_dn9 = ((((2.0 * ((((2.0 * locals.var_fn382_calc_iq__qs3_dn9) + (3.0 * locals.var_fn382_calc_iq__qd3_dn9)) + (((4.0 * locals.var_fn382_calc_iq__qs2_dn9) * locals.var_fn382_calc_iq__qinvd0) + (assign32380_e29581 * locals.var_fn382_calc_iq__qinvd0_dn9))) + (((6.0 * locals.var_fn382_calc_iq__qd2_dn9) * locals.var_fn382_calc_iq__qinvs0) + (assign32380_e29587 * locals.var_fn382_calc_iq__qinvs0_dn9)))) * assign32380_e29600) - (assign32380_e29591 * (15.0 * ((locals.var_fn382_calc_iq__qs2_dn9 + locals.var_fn382_calc_iq__qd2_dn9) + (2.0 * locals.var_fn382_calc_iq__qsqd_dn9))))) / (assign32380_e29600 * assign32380_e29600));
        locals.var_fn382_calc_iq__qd1_rv = 0.0;

        let assign32390_e29604: f64 = (locals.var_fn382_calc_iq__qinvdd - locals.var_fn382_calc_iq__qd1);
        locals.var_fn382_calc_iq__qs = assign32390_e29604;
        locals.var_fn382_calc_iq__qs_dn4 = (locals.var_fn382_calc_iq__qinvdd_dn4 - locals.var_fn382_calc_iq__qd1_dn4);
        locals.var_fn382_calc_iq__qs_dn5 = (locals.var_fn382_calc_iq__qinvdd_dn5 - locals.var_fn382_calc_iq__qd1_dn5);
        locals.var_fn382_calc_iq__qs_dn8 = (locals.var_fn382_calc_iq__qinvdd_dn8 - locals.var_fn382_calc_iq__qd1_dn8);
        locals.var_fn382_calc_iq__qs_dn9 = (locals.var_fn382_calc_iq__qinvdd_dn9 - locals.var_fn382_calc_iq__qd1_dn9);
        locals.var_fn382_calc_iq__qs_rv = 0.0;

        locals.var_fn382_calc_iq__qd = locals.var_fn382_calc_iq__qd1;
        locals.var_fn382_calc_iq__qd_dn4 = locals.var_fn382_calc_iq__qd1_dn4;
        locals.var_fn382_calc_iq__qd_dn5 = locals.var_fn382_calc_iq__qd1_dn5;
        locals.var_fn382_calc_iq__qd_dn8 = locals.var_fn382_calc_iq__qd1_dn8;
        locals.var_fn382_calc_iq__qd_dn9 = locals.var_fn382_calc_iq__qd1_dn9;
        locals.var_fn382_calc_iq__qd_rv = 0.0;

        let assign32410_e29608: f64 = (locals.var_fn382_calc_iq__w * locals.var_fn382_calc_iq__ngf);
        let assign32410_e29610: f64 = (assign32410_e29608 * locals.var_fn382_calc_iq__lin);
        let assign32410_e29612: f64 = (assign32410_e29610 * locals.var_fn382_calc_iq__type);
        let assign32410_e29614: f64 = (assign32410_e29612 * locals.var_fn382_calc_iq__qs);
        let assign32410_e29616: f64 = (assign32410_e29614 * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__qgsout = assign32410_e29616;
        locals.var_fn382_calc_iq__qgsout_dn4 = ((assign32410_e29612 * locals.var_fn382_calc_iq__qs_dn4) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__qgsout_dn5 = ((assign32410_e29612 * locals.var_fn382_calc_iq__qs_dn5) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__qgsout_dn8 = ((assign32410_e29612 * locals.var_fn382_calc_iq__qs_dn8) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__qgsout_dn9 = ((assign32410_e29612 * locals.var_fn382_calc_iq__qs_dn9) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__qgsout_dn22 = (assign32410_e29614 * locals.var_fn382_calc_iq__trapfracdl_dn22);
        locals.var_fn382_calc_iq__qgsout_dn23 = (assign32410_e29614 * locals.var_fn382_calc_iq__trapfracdl_dn23);
        locals.var_fn382_calc_iq__qgsout_dn25 = (assign32410_e29614 * locals.var_fn382_calc_iq__trapfracdl_dn25);
        locals.var_fn382_calc_iq__qgsout_dn26 = (assign32410_e29614 * locals.var_fn382_calc_iq__trapfracdl_dn26);
        locals.var_fn382_calc_iq__qgsout_rv = 0.0;

        let assign32420_e29619: f64 = (locals.var_fn382_calc_iq__w * locals.var_fn382_calc_iq__ngf);
        let assign32420_e29621: f64 = (assign32420_e29619 * locals.var_fn382_calc_iq__lin);
        let assign32420_e29623: f64 = (assign32420_e29621 * locals.var_fn382_calc_iq__type);
        let assign32420_e29625: f64 = (assign32420_e29623 * locals.var_fn382_calc_iq__qd);
        let assign32420_e29627: f64 = (assign32420_e29625 * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__qgdout = assign32420_e29627;
        locals.var_fn382_calc_iq__qgdout_dn4 = ((assign32420_e29623 * locals.var_fn382_calc_iq__qd_dn4) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__qgdout_dn5 = ((assign32420_e29623 * locals.var_fn382_calc_iq__qd_dn5) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__qgdout_dn8 = ((assign32420_e29623 * locals.var_fn382_calc_iq__qd_dn8) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__qgdout_dn9 = ((assign32420_e29623 * locals.var_fn382_calc_iq__qd_dn9) * locals.var_fn382_calc_iq__trapfracdl);
        locals.var_fn382_calc_iq__qgdout_dn22 = (assign32420_e29625 * locals.var_fn382_calc_iq__trapfracdl_dn22);
        locals.var_fn382_calc_iq__qgdout_dn23 = (assign32420_e29625 * locals.var_fn382_calc_iq__trapfracdl_dn23);
        locals.var_fn382_calc_iq__qgdout_dn25 = (assign32420_e29625 * locals.var_fn382_calc_iq__trapfracdl_dn25);
        locals.var_fn382_calc_iq__qgdout_dn26 = (assign32420_e29625 * locals.var_fn382_calc_iq__trapfracdl_dn26);
        locals.var_fn382_calc_iq__qgdout_rv = 0.0;

        let assign32430_e29630: f64 = if locals.var_fn382_calc_iq__qcbflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard408 = assign32430_e29630;
        locals.var_guard408_rv = 0.0;

        let (assign32440_e29644, assign32440_e29644_d_n4,) = {
    if (locals.var_guard408 != 0.0) {
        let assign32440_e29636: f64 = (p.p51 * 0.5);
        let assign32440_e29638: f64 = (assign32440_e29636 * locals.var_fn382_calc_iq__alpha_phit);
        let assign32440_e29639: f64 = (locals.var_fn382_calc_iq__vtof - assign32440_e29638);
        let assign32440_e29640: f64 = (locals.var_fn382_calc_iq__vcin - assign32440_e29639);
        let assign32440_e29642: f64 = (assign32440_e29640 / locals.var_fn382_calc_iq__two_n_phit0);
        (assign32440_e29642, ((((-(locals.var_fn382_calc_iq__vtof_dn4 - (assign32440_e29636 * locals.var_fn382_calc_iq__alpha_phit_dn4))) * locals.var_fn382_calc_iq__two_n_phit0) - (assign32440_e29640 * locals.var_fn382_calc_iq__two_n_phit0_dn4)) / (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__two_n_phit0)),)
    } else {
        (locals.var_fn382_calc_iq__etac, locals.var_fn382_calc_iq__etac_dn4,)
    }
};
        locals.var_fn382_calc_iq__etac = assign32440_e29644;
        locals.var_fn382_calc_iq__etac_dn4 = assign32440_e29644_d_n4;
        locals.var_fn382_calc_iq__etac_rv = 0.0;

        let assign32450_e29647: f64 = if locals.var_fn382_calc_iq__etac > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard409 = assign32450_e29647;
        locals.var_guard409_rv = 0.0;

        let (assign32460_e29653, assign32460_e29653_d_n4, assign32460_e29653_d_n5, assign32460_e29653_d_n8, assign32460_e29653_d_n9,) = {
    if ((locals.var_guard408 != 0.0) && (locals.var_guard409 != 0.0)) {
        (locals.var_fn382_calc_iq__etac, locals.var_fn382_calc_iq__etac_dn4, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__exparg, locals.var_fn382_calc_iq__exparg_dn4, locals.var_fn382_calc_iq__exparg_dn5, locals.var_fn382_calc_iq__exparg_dn8, locals.var_fn382_calc_iq__exparg_dn9,)
    }
};
        locals.var_fn382_calc_iq__exparg = assign32460_e29653;
        locals.var_fn382_calc_iq__exparg_dn4 = assign32460_e29653_d_n4;
        locals.var_fn382_calc_iq__exparg_dn5 = assign32460_e29653_d_n5;
        locals.var_fn382_calc_iq__exparg_dn8 = assign32460_e29653_d_n8;
        locals.var_fn382_calc_iq__exparg_dn9 = assign32460_e29653_d_n9;
        locals.var_fn382_calc_iq__exparg_rv = 0.0;

        let assign32470_e29656: f64 = (-50.0);
        let assign32470_e29657: f64 = if locals.var_fn382_calc_iq__etac < assign32470_e29656 { 1.0 } else { 0.0 };
        locals.var_guard410 = assign32470_e29657;
        locals.var_guard410_rv = 0.0;

        let (assign32480_e29667, assign32480_e29667_d_n4, assign32480_e29667_d_n5, assign32480_e29667_d_n8, assign32480_e29667_d_n9,) = {
    if (((locals.var_guard408 != 0.0) && (locals.var_guard409 == 0.0)) && (locals.var_guard410 != 0.0)) {
        let assign32480_e29665: f64 = (locals.var_fn382_calc_iq__etac).exp();
        (assign32480_e29665, (assign32480_e29665 * locals.var_fn382_calc_iq__etac_dn4), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__exparg, locals.var_fn382_calc_iq__exparg_dn4, locals.var_fn382_calc_iq__exparg_dn5, locals.var_fn382_calc_iq__exparg_dn8, locals.var_fn382_calc_iq__exparg_dn9,)
    }
};
        locals.var_fn382_calc_iq__exparg = assign32480_e29667;
        locals.var_fn382_calc_iq__exparg_dn4 = assign32480_e29667_d_n4;
        locals.var_fn382_calc_iq__exparg_dn5 = assign32480_e29667_d_n5;
        locals.var_fn382_calc_iq__exparg_dn8 = assign32480_e29667_d_n8;
        locals.var_fn382_calc_iq__exparg_dn9 = assign32480_e29667_d_n9;
        locals.var_fn382_calc_iq__exparg_rv = 0.0;

        let (assign32490_e29681, assign32490_e29681_d_n4, assign32490_e29681_d_n5, assign32490_e29681_d_n8, assign32490_e29681_d_n9,) = {
    if (((locals.var_guard408 != 0.0) && (locals.var_guard409 == 0.0)) && (locals.var_guard410 == 0.0)) {
        let assign32490_e29677: f64 = (locals.var_fn382_calc_iq__etac).exp();
        let assign32490_e29678: f64 = (1.0 + assign32490_e29677);
        let assign32490_e29679: f64 = (assign32490_e29678).ln();
        (assign32490_e29679, ((assign32490_e29677 * locals.var_fn382_calc_iq__etac_dn4) / assign32490_e29678), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__exparg, locals.var_fn382_calc_iq__exparg_dn4, locals.var_fn382_calc_iq__exparg_dn5, locals.var_fn382_calc_iq__exparg_dn8, locals.var_fn382_calc_iq__exparg_dn9,)
    }
};
        locals.var_fn382_calc_iq__exparg = assign32490_e29681;
        locals.var_fn382_calc_iq__exparg_dn4 = assign32490_e29681_d_n4;
        locals.var_fn382_calc_iq__exparg_dn5 = assign32490_e29681_d_n5;
        locals.var_fn382_calc_iq__exparg_dn8 = assign32490_e29681_d_n8;
        locals.var_fn382_calc_iq__exparg_dn9 = assign32490_e29681_d_n9;
        locals.var_fn382_calc_iq__exparg_rv = 0.0;

        let (assign32510_e29711, assign32510_e29711_d_n4,) = {
    if (locals.var_guard408 != 0.0) {
        let assign32510_e29703: f64 = (p.p51 * 0.5);
        let assign32510_e29705: f64 = (assign32510_e29703 * locals.var_fn382_calc_iq__alpha_phit);
        let assign32510_e29706: f64 = (locals.var_fn382_calc_iq__vtof - assign32510_e29705);
        let assign32510_e29707: f64 = (locals.var_fn382_calc_iq__vbin - assign32510_e29706);
        let assign32510_e29709: f64 = (assign32510_e29707 / locals.var_fn382_calc_iq__two_n_phit0);
        (assign32510_e29709, ((((-(locals.var_fn382_calc_iq__vtof_dn4 - (assign32510_e29703 * locals.var_fn382_calc_iq__alpha_phit_dn4))) * locals.var_fn382_calc_iq__two_n_phit0) - (assign32510_e29707 * locals.var_fn382_calc_iq__two_n_phit0_dn4)) / (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__two_n_phit0)),)
    } else {
        (locals.var_fn382_calc_iq__etab, locals.var_fn382_calc_iq__etab_dn4,)
    }
};
        locals.var_fn382_calc_iq__etab = assign32510_e29711;
        locals.var_fn382_calc_iq__etab_dn4 = assign32510_e29711_d_n4;
        locals.var_fn382_calc_iq__etab_rv = 0.0;

        let assign32520_e29714: f64 = if locals.var_fn382_calc_iq__etab > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard411 = assign32520_e29714;
        locals.var_guard411_rv = 0.0;

        let (assign32530_e29720, assign32530_e29720_d_n4, assign32530_e29720_d_n5, assign32530_e29720_d_n8, assign32530_e29720_d_n9,) = {
    if ((locals.var_guard408 != 0.0) && (locals.var_guard411 != 0.0)) {
        (locals.var_fn382_calc_iq__etab, locals.var_fn382_calc_iq__etab_dn4, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__exparg, locals.var_fn382_calc_iq__exparg_dn4, locals.var_fn382_calc_iq__exparg_dn5, locals.var_fn382_calc_iq__exparg_dn8, locals.var_fn382_calc_iq__exparg_dn9,)
    }
};
        locals.var_fn382_calc_iq__exparg = assign32530_e29720;
        locals.var_fn382_calc_iq__exparg_dn4 = assign32530_e29720_d_n4;
        locals.var_fn382_calc_iq__exparg_dn5 = assign32530_e29720_d_n5;
        locals.var_fn382_calc_iq__exparg_dn8 = assign32530_e29720_d_n8;
        locals.var_fn382_calc_iq__exparg_dn9 = assign32530_e29720_d_n9;
        locals.var_fn382_calc_iq__exparg_rv = 0.0;

        let assign32540_e29723: f64 = (-50.0);
        let assign32540_e29724: f64 = if locals.var_fn382_calc_iq__etab < assign32540_e29723 { 1.0 } else { 0.0 };
        locals.var_guard412 = assign32540_e29724;
        locals.var_guard412_rv = 0.0;

        let (assign32550_e29734, assign32550_e29734_d_n4, assign32550_e29734_d_n5, assign32550_e29734_d_n8, assign32550_e29734_d_n9,) = {
    if (((locals.var_guard408 != 0.0) && (locals.var_guard411 == 0.0)) && (locals.var_guard412 != 0.0)) {
        let assign32550_e29732: f64 = (locals.var_fn382_calc_iq__etab).exp();
        (assign32550_e29732, (assign32550_e29732 * locals.var_fn382_calc_iq__etab_dn4), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__exparg, locals.var_fn382_calc_iq__exparg_dn4, locals.var_fn382_calc_iq__exparg_dn5, locals.var_fn382_calc_iq__exparg_dn8, locals.var_fn382_calc_iq__exparg_dn9,)
    }
};
        locals.var_fn382_calc_iq__exparg = assign32550_e29734;
        locals.var_fn382_calc_iq__exparg_dn4 = assign32550_e29734_d_n4;
        locals.var_fn382_calc_iq__exparg_dn5 = assign32550_e29734_d_n5;
        locals.var_fn382_calc_iq__exparg_dn8 = assign32550_e29734_d_n8;
        locals.var_fn382_calc_iq__exparg_dn9 = assign32550_e29734_d_n9;
        locals.var_fn382_calc_iq__exparg_rv = 0.0;

        let (assign32560_e29748, assign32560_e29748_d_n4, assign32560_e29748_d_n5, assign32560_e29748_d_n8, assign32560_e29748_d_n9,) = {
    if (((locals.var_guard408 != 0.0) && (locals.var_guard411 == 0.0)) && (locals.var_guard412 == 0.0)) {
        let assign32560_e29744: f64 = (locals.var_fn382_calc_iq__etab).exp();
        let assign32560_e29745: f64 = (1.0 + assign32560_e29744);
        let assign32560_e29746: f64 = (assign32560_e29745).ln();
        (assign32560_e29746, ((assign32560_e29744 * locals.var_fn382_calc_iq__etab_dn4) / assign32560_e29745), 0.0, 0.0, 0.0,)
    } else {
        (locals.var_fn382_calc_iq__exparg, locals.var_fn382_calc_iq__exparg_dn4, locals.var_fn382_calc_iq__exparg_dn5, locals.var_fn382_calc_iq__exparg_dn8, locals.var_fn382_calc_iq__exparg_dn9,)
    }
};
        locals.var_fn382_calc_iq__exparg = assign32560_e29748;
        locals.var_fn382_calc_iq__exparg_dn4 = assign32560_e29748_d_n4;
        locals.var_fn382_calc_iq__exparg_dn5 = assign32560_e29748_d_n5;
        locals.var_fn382_calc_iq__exparg_dn8 = assign32560_e29748_d_n8;
        locals.var_fn382_calc_iq__exparg_dn9 = assign32560_e29748_d_n9;
        locals.var_fn382_calc_iq__exparg_rv = 0.0;

        let assign32600_e29777: f64 = if locals.var_fn382_calc_iq__qgsflag == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard413 = assign32600_e29777;
        locals.var_guard413_rv = 0.0;

        let (assign32610_e29791, assign32610_e29791_d_n4, assign32610_e29791_d_n8, assign32610_e29791_d_n9,) = {
    if (locals.var_guard413 != 0.0) {
        let assign32610_e29783: f64 = (p.p51 * 0.5);
        let assign32610_e29785: f64 = (assign32610_e29783 * locals.var_fn382_calc_iq__alpha_phit);
        let assign32610_e29786: f64 = (locals.var_fn382_calc_iq__vtof - assign32610_e29785);
        let assign32610_e29787: f64 = (locals.var_fn382_calc_iq__vgsin - assign32610_e29786);
        let assign32610_e29789: f64 = (assign32610_e29787 / locals.var_fn382_calc_iq__two_n_phit0);
        (assign32610_e29789, ((((-(locals.var_fn382_calc_iq__vtof_dn4 - (assign32610_e29783 * locals.var_fn382_calc_iq__alpha_phit_dn4))) * locals.var_fn382_calc_iq__two_n_phit0) - (assign32610_e29787 * locals.var_fn382_calc_iq__two_n_phit0_dn4)) / (locals.var_fn382_calc_iq__two_n_phit0 * locals.var_fn382_calc_iq__two_n_phit0)), (locals.var_fn382_calc_iq__vgsin_dn8 / locals.var_fn382_calc_iq__two_n_phit0), (locals.var_fn382_calc_iq__vgsin_dn9 / locals.var_fn382_calc_iq__two_n_phit0),)
    } else {
        (locals.var_fn382_calc_iq__etags, locals.var_fn382_calc_iq__etags_dn4, locals.var_fn382_calc_iq__etags_dn8, locals.var_fn382_calc_iq__etags_dn9,)
    }
};
        locals.var_fn382_calc_iq__etags = assign32610_e29791;
        locals.var_fn382_calc_iq__etags_dn4 = assign32610_e29791_d_n4;
        locals.var_fn382_calc_iq__etags_dn8 = assign32610_e29791_d_n8;
        locals.var_fn382_calc_iq__etags_dn9 = assign32610_e29791_d_n9;
        locals.var_fn382_calc_iq__etags_rv = 0.0;

        let assign32620_e29794: f64 = if locals.var_fn382_calc_iq__etags > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard414 = assign32620_e29794;
        locals.var_guard414_rv = 0.0;

        let (assign32630_e29800, assign32630_e29800_d_n4, assign32630_e29800_d_n5, assign32630_e29800_d_n8, assign32630_e29800_d_n9,) = {
    if ((locals.var_guard413 != 0.0) && (locals.var_guard414 != 0.0)) {
        (locals.var_fn382_calc_iq__etags, locals.var_fn382_calc_iq__etags_dn4, 0.0, locals.var_fn382_calc_iq__etags_dn8, locals.var_fn382_calc_iq__etags_dn9,)
    } else {
        (locals.var_fn382_calc_iq__exparg, locals.var_fn382_calc_iq__exparg_dn4, locals.var_fn382_calc_iq__exparg_dn5, locals.var_fn382_calc_iq__exparg_dn8, locals.var_fn382_calc_iq__exparg_dn9,)
    }
};
        locals.var_fn382_calc_iq__exparg = assign32630_e29800;
        locals.var_fn382_calc_iq__exparg_dn4 = assign32630_e29800_d_n4;
        locals.var_fn382_calc_iq__exparg_dn5 = assign32630_e29800_d_n5;
        locals.var_fn382_calc_iq__exparg_dn8 = assign32630_e29800_d_n8;
        locals.var_fn382_calc_iq__exparg_dn9 = assign32630_e29800_d_n9;
        locals.var_fn382_calc_iq__exparg_rv = 0.0;

        let assign32640_e29803: f64 = (-50.0);
        let assign32640_e29804: f64 = if locals.var_fn382_calc_iq__etags < assign32640_e29803 { 1.0 } else { 0.0 };
        locals.var_guard415 = assign32640_e29804;
        locals.var_guard415_rv = 0.0;

        let (assign32650_e29814, assign32650_e29814_d_n4, assign32650_e29814_d_n5, assign32650_e29814_d_n8, assign32650_e29814_d_n9,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard414 == 0.0)) && (locals.var_guard415 != 0.0)) {
        let assign32650_e29812: f64 = (locals.var_fn382_calc_iq__etags).exp();
        (assign32650_e29812, (assign32650_e29812 * locals.var_fn382_calc_iq__etags_dn4), 0.0, (assign32650_e29812 * locals.var_fn382_calc_iq__etags_dn8), (assign32650_e29812 * locals.var_fn382_calc_iq__etags_dn9),)
    } else {
        (locals.var_fn382_calc_iq__exparg, locals.var_fn382_calc_iq__exparg_dn4, locals.var_fn382_calc_iq__exparg_dn5, locals.var_fn382_calc_iq__exparg_dn8, locals.var_fn382_calc_iq__exparg_dn9,)
    }
};
        locals.var_fn382_calc_iq__exparg = assign32650_e29814;
        locals.var_fn382_calc_iq__exparg_dn4 = assign32650_e29814_d_n4;
        locals.var_fn382_calc_iq__exparg_dn5 = assign32650_e29814_d_n5;
        locals.var_fn382_calc_iq__exparg_dn8 = assign32650_e29814_d_n8;
        locals.var_fn382_calc_iq__exparg_dn9 = assign32650_e29814_d_n9;
        locals.var_fn382_calc_iq__exparg_rv = 0.0;

        let (assign32660_e29828, assign32660_e29828_d_n4, assign32660_e29828_d_n5, assign32660_e29828_d_n8, assign32660_e29828_d_n9,) = {
    if (((locals.var_guard413 != 0.0) && (locals.var_guard414 == 0.0)) && (locals.var_guard415 == 0.0)) {
        let assign32660_e29824: f64 = (locals.var_fn382_calc_iq__etags).exp();
        let assign32660_e29825: f64 = (1.0 + assign32660_e29824);
        let assign32660_e29826: f64 = (assign32660_e29825).ln();
        (assign32660_e29826, ((assign32660_e29824 * locals.var_fn382_calc_iq__etags_dn4) / assign32660_e29825), 0.0, ((assign32660_e29824 * locals.var_fn382_calc_iq__etags_dn8) / assign32660_e29825), ((assign32660_e29824 * locals.var_fn382_calc_iq__etags_dn9) / assign32660_e29825),)
    } else {
        (locals.var_fn382_calc_iq__exparg, locals.var_fn382_calc_iq__exparg_dn4, locals.var_fn382_calc_iq__exparg_dn5, locals.var_fn382_calc_iq__exparg_dn8, locals.var_fn382_calc_iq__exparg_dn9,)
    }
};
        locals.var_fn382_calc_iq__exparg = assign32660_e29828;
        locals.var_fn382_calc_iq__exparg_dn4 = assign32660_e29828_d_n4;
        locals.var_fn382_calc_iq__exparg_dn5 = assign32660_e29828_d_n5;
        locals.var_fn382_calc_iq__exparg_dn8 = assign32660_e29828_d_n8;
        locals.var_fn382_calc_iq__exparg_dn9 = assign32660_e29828_d_n9;
        locals.var_fn382_calc_iq__exparg_rv = 0.0;

        locals.var_fn382_calc_iq__return = locals.var_fn382_calc_iq__idsout;
        locals.var_fn382_calc_iq__return_dn4 = locals.var_fn382_calc_iq__idsout_dn4;
        locals.var_fn382_calc_iq__return_dn5 = locals.var_fn382_calc_iq__idsout_dn5;
        locals.var_fn382_calc_iq__return_dn8 = locals.var_fn382_calc_iq__idsout_dn8;
        locals.var_fn382_calc_iq__return_dn9 = locals.var_fn382_calc_iq__idsout_dn9;
        locals.var_fn382_calc_iq__return_dn22 = locals.var_fn382_calc_iq__idsout_dn22;
        locals.var_fn382_calc_iq__return_dn23 = locals.var_fn382_calc_iq__idsout_dn23;
        locals.var_fn382_calc_iq__return_dn25 = locals.var_fn382_calc_iq__idsout_dn25;
        locals.var_fn382_calc_iq__return_dn26 = locals.var_fn382_calc_iq__idsout_dn26;
        locals.var_fn382_calc_iq__return_rv = 0.0;

        locals.var_ids = locals.var_fn382_calc_iq__idsout;
        locals.var_ids_dn4 = locals.var_fn382_calc_iq__idsout_dn4;
        locals.var_ids_dn5 = locals.var_fn382_calc_iq__idsout_dn5;
        locals.var_ids_dn8 = locals.var_fn382_calc_iq__idsout_dn8;
        locals.var_ids_dn9 = locals.var_fn382_calc_iq__idsout_dn9;
        locals.var_ids_dn22 = locals.var_fn382_calc_iq__idsout_dn22;
        locals.var_ids_dn23 = locals.var_fn382_calc_iq__idsout_dn23;
        locals.var_ids_dn25 = locals.var_fn382_calc_iq__idsout_dn25;
        locals.var_ids_dn26 = locals.var_fn382_calc_iq__idsout_dn26;
        locals.var_ids_rv = 0.0;

        locals.var_qgs = locals.var_fn382_calc_iq__qgsout;
        locals.var_qgs_dn4 = locals.var_fn382_calc_iq__qgsout_dn4;
        locals.var_qgs_dn5 = locals.var_fn382_calc_iq__qgsout_dn5;
        locals.var_qgs_dn8 = locals.var_fn382_calc_iq__qgsout_dn8;
        locals.var_qgs_dn9 = locals.var_fn382_calc_iq__qgsout_dn9;
        locals.var_qgs_dn22 = locals.var_fn382_calc_iq__qgsout_dn22;
        locals.var_qgs_dn23 = locals.var_fn382_calc_iq__qgsout_dn23;
        locals.var_qgs_dn25 = locals.var_fn382_calc_iq__qgsout_dn25;
        locals.var_qgs_dn26 = locals.var_fn382_calc_iq__qgsout_dn26;
        locals.var_qgs_rv = 0.0;

        locals.var_qgd = locals.var_fn382_calc_iq__qgdout;
        locals.var_qgd_dn4 = locals.var_fn382_calc_iq__qgdout_dn4;
        locals.var_qgd_dn5 = locals.var_fn382_calc_iq__qgdout_dn5;
        locals.var_qgd_dn8 = locals.var_fn382_calc_iq__qgdout_dn8;
        locals.var_qgd_dn9 = locals.var_fn382_calc_iq__qgdout_dn9;
        locals.var_qgd_dn22 = locals.var_fn382_calc_iq__qgdout_dn22;
        locals.var_qgd_dn23 = locals.var_fn382_calc_iq__qgdout_dn23;
        locals.var_qgd_dn25 = locals.var_fn382_calc_iq__qgdout_dn25;
        locals.var_qgd_dn26 = locals.var_fn382_calc_iq__qgdout_dn26;
        locals.var_qgd_rv = 0.0;

        locals.var_ids = locals.var_fn382_calc_iq__return;
        locals.var_ids_dn4 = locals.var_fn382_calc_iq__return_dn4;
        locals.var_ids_dn5 = locals.var_fn382_calc_iq__return_dn5;
        locals.var_ids_dn8 = locals.var_fn382_calc_iq__return_dn8;
        locals.var_ids_dn9 = locals.var_fn382_calc_iq__return_dn9;
        locals.var_ids_dn22 = locals.var_fn382_calc_iq__return_dn22;
        locals.var_ids_dn23 = locals.var_fn382_calc_iq__return_dn23;
        locals.var_ids_dn25 = locals.var_fn382_calc_iq__return_dn25;
        locals.var_ids_dn26 = locals.var_fn382_calc_iq__return_dn26;
        locals.var_ids_rv = 0.0;

        let assign32800_e29863: f64 = if p.p322 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard416 = assign32800_e29863;
        locals.var_guard416_rv = 0.0;

        locals.var_vsch = 0.0;
        locals.var_vsch_dn7 = 0.0;
        locals.var_vsch_dn8 = 0.0;
        locals.var_vsch_rv = 0.0;

        locals.var_qsch = 0.0;
        locals.var_qsch_dn7 = 0.0;
        locals.var_qsch_dn8 = 0.0;
        locals.var_qsch_rv = 0.0;

        locals.var_qsch0 = 0.0;
        locals.var_qsch0_rv = 0.0;

        locals.var_qsch1 = 0.0;
        locals.var_qsch1_dn7 = 0.0;
        locals.var_qsch1_dn8 = 0.0;
        locals.var_qsch1_rv = 0.0;

        locals.var_qsch2 = 0.0;
        locals.var_qsch2_dn7 = 0.0;
        locals.var_qsch2_dn8 = 0.0;
        locals.var_qsch2_rv = 0.0;

        locals.var_qsch3 = 0.0;
        locals.var_qsch3_dn7 = 0.0;
        locals.var_qsch3_dn8 = 0.0;
        locals.var_qsch3_rv = 0.0;

        locals.var_qsch4 = 0.0;
        locals.var_qsch4_dn7 = 0.0;
        locals.var_qsch4_dn8 = 0.0;
        locals.var_qsch4_rv = 0.0;

        locals.var_qsch5 = 0.0;
        locals.var_qsch5_dn7 = 0.0;
        locals.var_qsch5_dn8 = 0.0;
        locals.var_qsch5_rv = 0.0;

        locals.var_vschfc1 = 0.0;
        locals.var_vschfc1_dn7 = 0.0;
        locals.var_vschfc1_dn8 = 0.0;
        locals.var_vschfc1_rv = 0.0;

        locals.var_vschfc2 = 0.0;
        locals.var_vschfc2_dn7 = 0.0;
        locals.var_vschfc2_dn8 = 0.0;
        locals.var_vschfc2_rv = 0.0;

        locals.var_vschfc3 = 0.0;
        locals.var_vschfc3_dn7 = 0.0;
        locals.var_vschfc3_dn8 = 0.0;
        locals.var_vschfc3_rv = 0.0;

        locals.var_vschfc4 = 0.0;
        locals.var_vschfc4_dn7 = 0.0;
        locals.var_vschfc4_dn8 = 0.0;
        locals.var_vschfc4_rv = 0.0;

        locals.var_vschfc5 = 0.0;
        locals.var_vschfc5_dn7 = 0.0;
        locals.var_vschfc5_dn8 = 0.0;
        locals.var_vschfc5_rv = 0.0;

        let assign41530_e39902: f64 = if p.p291 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard461 = assign41530_e39902;
        locals.var_guard461_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_72(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let (assign41540_e39908, assign41540_e39908_d_n7, assign41540_e39908_d_n8,) = {
    if (locals.var_guard461 != 0.0) {
        let assign41540_e39906: f64 = (p.p6 * (nv8 - nv7));
        (assign41540_e39906, (-p.p6), p.p6,)
    } else {
        (locals.var_vsch, locals.var_vsch_dn7, locals.var_vsch_dn8,)
    }
};
        locals.var_vsch = assign41540_e39908;
        locals.var_vsch_dn7 = assign41540_e39908_d_n7;
        locals.var_vsch_dn8 = assign41540_e39908_d_n8;
        locals.var_vsch_rv = 0.0;

        let assign43620_e42207: f64 = (p.p308 * p.p306);
        let assign43620_e42208: f64 = if locals.var_vsch <= assign43620_e42207 { 1.0 } else { 0.0 };
        locals.var_guard473 = assign43620_e42208;
        locals.var_guard473_rv = 0.0;

        let (assign43630_e42237, assign43630_e42237_d_n7, assign43630_e42237_d_n8,) = {
    if ((locals.var_guard461 != 0.0) && (locals.var_guard473 != 0.0)) {
        let assign43630_e42214: f64 = (p.p6 * 2.0);
        let assign43630_e42216: f64 = (assign43630_e42214 * p.p307);
        let assign43630_e42218: f64 = (assign43630_e42216 * p.p0);
        let assign43630_e42221: f64 = (1.0 - p.p311);
        let assign43630_e42222: f64 = (assign43630_e42218 * assign43630_e42221);
        let assign43630_e42224: f64 = (assign43630_e42222 * p.p2);
        let assign43630_e42226: f64 = (assign43630_e42224 * p.p306);
        let assign43630_e42231: f64 = (locals.var_vsch / p.p306);
        let assign43630_e42232: f64 = (1.0 - assign43630_e42231);
        let assign43630_e42233: f64 = (assign43630_e42232).sqrt();
        let assign43630_e42234: f64 = (1.0 - assign43630_e42233);
        let assign43630_e42235: f64 = (assign43630_e42226 * assign43630_e42234);
        (assign43630_e42235, (assign43630_e42226 * (-((-(locals.var_vsch_dn7 / p.p306)) / (2.0 * assign43630_e42233)))), (assign43630_e42226 * (-((-(locals.var_vsch_dn8 / p.p306)) / (2.0 * assign43630_e42233)))),)
    } else {
        (locals.var_qsch, locals.var_qsch_dn7, locals.var_qsch_dn8,)
    }
};
        locals.var_qsch = assign43630_e42237;
        locals.var_qsch_dn7 = assign43630_e42237_d_n7;
        locals.var_qsch_dn8 = assign43630_e42237_d_n8;
        locals.var_qsch_rv = 0.0;

        let (assign43640_e42249,) = {
    if ((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) {
        let assign43640_e42245: f64 = (1.0 - p.p308);
        let assign43640_e42246: f64 = (assign43640_e42245).sqrt();
        let assign43640_e42247: f64 = (1.0 - assign43640_e42246);
        (assign43640_e42247,)
    } else {
        (locals.var_qsch0,)
    }
};
        locals.var_qsch0 = assign43640_e42249;
        locals.var_qsch0_rv = 0.0;

        let assign43650_e42252: f64 = if p.p309 >= 1.0 { 1.0 } else { 0.0 };
        locals.var_guard474 = assign43650_e42252;
        locals.var_guard474_rv = 0.0;

        let (assign43660_e42270,) = {
    if (((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) {
        let assign43660_e42262: f64 = (2.0 * p.p306);
        let assign43660_e42265: f64 = (1.0 - p.p308);
        let assign43660_e42266: f64 = (assign43660_e42265).sqrt();
        let assign43660_e42267: f64 = (assign43660_e42262 * assign43660_e42266);
        let assign43660_e42268: f64 = (1.0 / assign43660_e42267);
        (assign43660_e42268,)
    } else {
        (locals.var_qsch1c,)
    }
};
        locals.var_qsch1c = assign43660_e42270;
        locals.var_qsch1c_rv = 0.0;

        let (assign43670_e42283, assign43670_e42283_d_n7, assign43670_e42283_d_n8,) = {
    if (((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) {
        let assign43670_e42280: f64 = (p.p308 * p.p306);
        let assign43670_e42281: f64 = (locals.var_vsch - assign43670_e42280);
        (assign43670_e42281, locals.var_vsch_dn7, locals.var_vsch_dn8,)
    } else {
        (locals.var_vschfc1, locals.var_vschfc1_dn7, locals.var_vschfc1_dn8,)
    }
};
        locals.var_vschfc1 = assign43670_e42283;
        locals.var_vschfc1_dn7 = assign43670_e42283_d_n7;
        locals.var_vschfc1_dn8 = assign43670_e42283_d_n8;
        locals.var_vschfc1_rv = 0.0;

        let (assign43680_e42294, assign43680_e42294_d_n7, assign43680_e42294_d_n8,) = {
    if (((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) {
        let assign43680_e42292: f64 = (locals.var_qsch1c * locals.var_vschfc1);
        (assign43680_e42292, (locals.var_qsch1c * locals.var_vschfc1_dn7), (locals.var_qsch1c * locals.var_vschfc1_dn8),)
    } else {
        (locals.var_qsch1, locals.var_qsch1_dn7, locals.var_qsch1_dn8,)
    }
};
        locals.var_qsch1 = assign43680_e42294;
        locals.var_qsch1_dn7 = assign43680_e42294_d_n7;
        locals.var_qsch1_dn8 = assign43680_e42294_d_n8;
        locals.var_qsch1_rv = 0.0;

        let assign43690_e42297: f64 = if p.p309 >= 2.0 { 1.0 } else { 0.0 };
        locals.var_guard475 = assign43690_e42297;
        locals.var_guard475_rv = 0.0;

        let (assign43700_e42316,) = {
    if ((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) {
        let assign43700_e42309: f64 = (4.0 * p.p306);
        let assign43700_e42312: f64 = (1.0 - p.p308);
        let assign43700_e42313: f64 = (assign43700_e42309 * assign43700_e42312);
        let assign43700_e42314: f64 = (locals.var_qsch1c / assign43700_e42313);
        (assign43700_e42314,)
    } else {
        (locals.var_qsch2c,)
    }
};
        locals.var_qsch2c = assign43700_e42316;
        locals.var_qsch2c_rv = 0.0;

        let (assign43710_e42329, assign43710_e42329_d_n7, assign43710_e42329_d_n8,) = {
    if ((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) {
        let assign43710_e42327: f64 = (locals.var_vschfc1 * locals.var_vschfc1);
        (assign43710_e42327, ((locals.var_vschfc1_dn7 * locals.var_vschfc1) + (locals.var_vschfc1 * locals.var_vschfc1_dn7)), ((locals.var_vschfc1_dn8 * locals.var_vschfc1) + (locals.var_vschfc1 * locals.var_vschfc1_dn8)),)
    } else {
        (locals.var_vschfc2, locals.var_vschfc2_dn7, locals.var_vschfc2_dn8,)
    }
};
        locals.var_vschfc2 = assign43710_e42329;
        locals.var_vschfc2_dn7 = assign43710_e42329_d_n7;
        locals.var_vschfc2_dn8 = assign43710_e42329_d_n8;
        locals.var_vschfc2_rv = 0.0;

        let (assign43720_e42342, assign43720_e42342_d_n7, assign43720_e42342_d_n8,) = {
    if ((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) {
        let assign43720_e42340: f64 = (locals.var_qsch2c * locals.var_vschfc2);
        (assign43720_e42340, (locals.var_qsch2c * locals.var_vschfc2_dn7), (locals.var_qsch2c * locals.var_vschfc2_dn8),)
    } else {
        (locals.var_qsch2, locals.var_qsch2_dn7, locals.var_qsch2_dn8,)
    }
};
        locals.var_qsch2 = assign43720_e42342;
        locals.var_qsch2_dn7 = assign43720_e42342_d_n7;
        locals.var_qsch2_dn8 = assign43720_e42342_d_n8;
        locals.var_qsch2_rv = 0.0;

        let assign43730_e42345: f64 = if p.p309 >= 3.0 { 1.0 } else { 0.0 };
        locals.var_guard476 = assign43730_e42345;
        locals.var_guard476_rv = 0.0;

        let (assign43740_e42366,) = {
    if (((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) {
        let assign43740_e42359: f64 = (2.0 * p.p306);
        let assign43740_e42362: f64 = (1.0 - p.p308);
        let assign43740_e42363: f64 = (assign43740_e42359 * assign43740_e42362);
        let assign43740_e42364: f64 = (locals.var_qsch2c / assign43740_e42363);
        (assign43740_e42364,)
    } else {
        (locals.var_qsch3c,)
    }
};
        locals.var_qsch3c = assign43740_e42366;
        locals.var_qsch3c_rv = 0.0;

        let (assign43750_e42381, assign43750_e42381_d_n7, assign43750_e42381_d_n8,) = {
    if (((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) {
        let assign43750_e42379: f64 = (locals.var_vschfc2 * locals.var_vschfc1);
        (assign43750_e42379, ((locals.var_vschfc2_dn7 * locals.var_vschfc1) + (locals.var_vschfc2 * locals.var_vschfc1_dn7)), ((locals.var_vschfc2_dn8 * locals.var_vschfc1) + (locals.var_vschfc2 * locals.var_vschfc1_dn8)),)
    } else {
        (locals.var_vschfc3, locals.var_vschfc3_dn7, locals.var_vschfc3_dn8,)
    }
};
        locals.var_vschfc3 = assign43750_e42381;
        locals.var_vschfc3_dn7 = assign43750_e42381_d_n7;
        locals.var_vschfc3_dn8 = assign43750_e42381_d_n8;
        locals.var_vschfc3_rv = 0.0;

        let (assign43760_e42396, assign43760_e42396_d_n7, assign43760_e42396_d_n8,) = {
    if (((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) {
        let assign43760_e42394: f64 = (locals.var_qsch3c * locals.var_vschfc3);
        (assign43760_e42394, (locals.var_qsch3c * locals.var_vschfc3_dn7), (locals.var_qsch3c * locals.var_vschfc3_dn8),)
    } else {
        (locals.var_qsch3, locals.var_qsch3_dn7, locals.var_qsch3_dn8,)
    }
};
        locals.var_qsch3 = assign43760_e42396;
        locals.var_qsch3_dn7 = assign43760_e42396_d_n7;
        locals.var_qsch3_dn8 = assign43760_e42396_d_n8;
        locals.var_qsch3_rv = 0.0;

        let assign43770_e42399: f64 = if p.p309 >= 4.0 { 1.0 } else { 0.0 };
        locals.var_guard477 = assign43770_e42399;
        locals.var_guard477_rv = 0.0;

        let (assign43780_e42424,) = {
    if ((((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) && (locals.var_guard477 != 0.0)) {
        let assign43780_e42414: f64 = (5.0 * locals.var_qsch3c);
        let assign43780_e42417: f64 = (8.0 * p.p306);
        let assign43780_e42420: f64 = (1.0 - p.p308);
        let assign43780_e42421: f64 = (assign43780_e42417 * assign43780_e42420);
        let assign43780_e42422: f64 = (assign43780_e42414 / assign43780_e42421);
        (assign43780_e42422,)
    } else {
        (locals.var_qsch4c,)
    }
};
        locals.var_qsch4c = assign43780_e42424;
        locals.var_qsch4c_rv = 0.0;

        let (assign43790_e42441, assign43790_e42441_d_n7, assign43790_e42441_d_n8,) = {
    if ((((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) && (locals.var_guard477 != 0.0)) {
        let assign43790_e42439: f64 = (locals.var_vschfc3 * locals.var_vschfc1);
        (assign43790_e42439, ((locals.var_vschfc3_dn7 * locals.var_vschfc1) + (locals.var_vschfc3 * locals.var_vschfc1_dn7)), ((locals.var_vschfc3_dn8 * locals.var_vschfc1) + (locals.var_vschfc3 * locals.var_vschfc1_dn8)),)
    } else {
        (locals.var_vschfc4, locals.var_vschfc4_dn7, locals.var_vschfc4_dn8,)
    }
};
        locals.var_vschfc4 = assign43790_e42441;
        locals.var_vschfc4_dn7 = assign43790_e42441_d_n7;
        locals.var_vschfc4_dn8 = assign43790_e42441_d_n8;
        locals.var_vschfc4_rv = 0.0;

        let (assign43800_e42458, assign43800_e42458_d_n7, assign43800_e42458_d_n8,) = {
    if ((((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) && (locals.var_guard477 != 0.0)) {
        let assign43800_e42456: f64 = (locals.var_qsch4c * locals.var_vschfc4);
        (assign43800_e42456, (locals.var_qsch4c * locals.var_vschfc4_dn7), (locals.var_qsch4c * locals.var_vschfc4_dn8),)
    } else {
        (locals.var_qsch4, locals.var_qsch4_dn7, locals.var_qsch4_dn8,)
    }
};
        locals.var_qsch4 = assign43800_e42458;
        locals.var_qsch4_dn7 = assign43800_e42458_d_n7;
        locals.var_qsch4_dn8 = assign43800_e42458_d_n8;
        locals.var_qsch4_rv = 0.0;

        let assign43810_e42461: f64 = if p.p309 >= 5.0 { 1.0 } else { 0.0 };
        locals.var_guard478 = assign43810_e42461;
        locals.var_guard478_rv = 0.0;

        let (assign43820_e42488,) = {
    if (((((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) && (locals.var_guard477 != 0.0)) && (locals.var_guard478 != 0.0)) {
        let assign43820_e42478: f64 = (7.0 * locals.var_qsch4c);
        let assign43820_e42481: f64 = (10.0 * p.p306);
        let assign43820_e42484: f64 = (1.0 - p.p308);
        let assign43820_e42485: f64 = (assign43820_e42481 * assign43820_e42484);
        let assign43820_e42486: f64 = (assign43820_e42478 / assign43820_e42485);
        (assign43820_e42486,)
    } else {
        (locals.var_qsch5c,)
    }
};
        locals.var_qsch5c = assign43820_e42488;
        locals.var_qsch5c_rv = 0.0;

        let (assign43830_e42507, assign43830_e42507_d_n7, assign43830_e42507_d_n8,) = {
    if (((((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) && (locals.var_guard477 != 0.0)) && (locals.var_guard478 != 0.0)) {
        let assign43830_e42505: f64 = (locals.var_vschfc4 * locals.var_vschfc1);
        (assign43830_e42505, ((locals.var_vschfc4_dn7 * locals.var_vschfc1) + (locals.var_vschfc4 * locals.var_vschfc1_dn7)), ((locals.var_vschfc4_dn8 * locals.var_vschfc1) + (locals.var_vschfc4 * locals.var_vschfc1_dn8)),)
    } else {
        (locals.var_vschfc5, locals.var_vschfc5_dn7, locals.var_vschfc5_dn8,)
    }
};
        locals.var_vschfc5 = assign43830_e42507;
        locals.var_vschfc5_dn7 = assign43830_e42507_d_n7;
        locals.var_vschfc5_dn8 = assign43830_e42507_d_n8;
        locals.var_vschfc5_rv = 0.0;

        let (assign43840_e42526, assign43840_e42526_d_n7, assign43840_e42526_d_n8,) = {
    if (((((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) && (locals.var_guard477 != 0.0)) && (locals.var_guard478 != 0.0)) {
        let assign43840_e42524: f64 = (locals.var_qsch5c * locals.var_vschfc5);
        (assign43840_e42524, (locals.var_qsch5c * locals.var_vschfc5_dn7), (locals.var_qsch5c * locals.var_vschfc5_dn8),)
    } else {
        (locals.var_qsch5, locals.var_qsch5_dn7, locals.var_qsch5_dn8,)
    }
};
        locals.var_qsch5 = assign43840_e42526;
        locals.var_qsch5_dn7 = assign43840_e42526_d_n7;
        locals.var_qsch5_dn8 = assign43840_e42526_d_n8;
        locals.var_qsch5_rv = 0.0;

        let (assign43850_e42544,) = {
    if (((((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) && (locals.var_guard477 != 0.0)) && (locals.var_guard478 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_qsch5c,)
    }
};
        locals.var_qsch5c = assign43850_e42544;
        locals.var_qsch5c_rv = 0.0;

        let (assign43860_e42560,) = {
    if ((((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 != 0.0)) && (locals.var_guard477 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_qsch4c,)
    }
};
        locals.var_qsch4c = assign43860_e42560;
        locals.var_qsch4c_rv = 0.0;

        let (assign43870_e42574,) = {
    if (((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 != 0.0)) && (locals.var_guard476 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_qsch3c,)
    }
};
        locals.var_qsch3c = assign43870_e42574;
        locals.var_qsch3c_rv = 0.0;

        let (assign43880_e42586,) = {
    if ((((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 != 0.0)) && (locals.var_guard475 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_qsch2c,)
    }
};
        locals.var_qsch2c = assign43880_e42586;
        locals.var_qsch2c_rv = 0.0;

        let (assign43890_e42596,) = {
    if (((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) && (locals.var_guard474 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_qsch1c,)
    }
};
        locals.var_qsch1c = assign43890_e42596;
        locals.var_qsch1c_rv = 0.0;

        let (assign43900_e42629, assign43900_e42629_d_n7, assign43900_e42629_d_n8,) = {
    if ((locals.var_guard461 != 0.0) && (locals.var_guard473 == 0.0)) {
        let assign43900_e42603: f64 = (p.p6 * 2.0);
        let assign43900_e42605: f64 = (assign43900_e42603 * p.p307);
        let assign43900_e42607: f64 = (assign43900_e42605 * p.p0);
        let assign43900_e42610: f64 = (1.0 - p.p311);
        let assign43900_e42611: f64 = (assign43900_e42607 * assign43900_e42610);
        let assign43900_e42613: f64 = (assign43900_e42611 * p.p2);
        let assign43900_e42615: f64 = (assign43900_e42613 * p.p306);
        let assign43900_e42618: f64 = (locals.var_qsch0 + locals.var_qsch1);
        let assign43900_e42620: f64 = (assign43900_e42618 + locals.var_qsch2);
        let assign43900_e42622: f64 = (assign43900_e42620 + locals.var_qsch3);
        let assign43900_e42624: f64 = (assign43900_e42622 + locals.var_qsch4);
        let assign43900_e42626: f64 = (assign43900_e42624 + locals.var_qsch5);
        let assign43900_e42627: f64 = (assign43900_e42615 * assign43900_e42626);
        (assign43900_e42627, (assign43900_e42615 * ((((locals.var_qsch1_dn7 + locals.var_qsch2_dn7) + locals.var_qsch3_dn7) + locals.var_qsch4_dn7) + locals.var_qsch5_dn7)), (assign43900_e42615 * ((((locals.var_qsch1_dn8 + locals.var_qsch2_dn8) + locals.var_qsch3_dn8) + locals.var_qsch4_dn8) + locals.var_qsch5_dn8)),)
    } else {
        (locals.var_qsch, locals.var_qsch_dn7, locals.var_qsch_dn8,)
    }
};
        locals.var_qsch = assign43900_e42629;
        locals.var_qsch_dn7 = assign43900_e42629_d_n7;
        locals.var_qsch_dn8 = assign43900_e42629_d_n8;
        locals.var_qsch_rv = 0.0;

        let assign46160_e44810: f64 = ((nv6 - nv2) - p.p27);
        let assign46160_e44812: f64 = (assign46160_e44810 / p.p28);
        let assign46160_e44814: f64 = if assign46160_e44812 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard497 = assign46160_e44814;
        locals.var_guard497_rv = 0.0;

        let (assign46170_e44830, assign46170_e44830_d_n2, assign46170_e44830_d_n4, assign46170_e44830_d_n6,) = {
    if (locals.var_guard497 != 0.0) {
        let assign46170_e44818: f64 = (p.p0 * p.p2);
        let assign46170_e44821: f64 = (locals.var_cofsmt0 * (nv6 - nv2));
        let assign46170_e44825: f64 = ((nv6 - nv2) - p.p27);
        let assign46170_e44826: f64 = (locals.var_cofsmt * assign46170_e44825);
        let assign46170_e44827: f64 = (assign46170_e44821 + assign46170_e44826);
        let assign46170_e44828: f64 = (assign46170_e44818 * assign46170_e44827);
        (assign46170_e44828, (assign46170_e44818 * ((-locals.var_cofsmt0) + (-locals.var_cofsmt))), (assign46170_e44818 * ((locals.var_cofsmt0_dn4 * (nv6 - nv2)) + (locals.var_cofsmt_dn4 * assign46170_e44825))), (assign46170_e44818 * (locals.var_cofsmt0 + locals.var_cofsmt)),)
    } else {
        (locals.var_qofs, locals.var_qofs_dn2, locals.var_qofs_dn4, locals.var_qofs_dn6,)
    }
};
        locals.var_qofs = assign46170_e44830;
        locals.var_qofs_dn2 = assign46170_e44830_d_n2;
        locals.var_qofs_dn4 = assign46170_e44830_d_n4;
        locals.var_qofs_dn6 = assign46170_e44830_d_n6;
        locals.var_qofs_rv = 0.0;

        let assign46180_e44833: f64 = ((nv6 - nv2) - p.p27);
        let assign46180_e44835: f64 = (assign46180_e44833 / p.p28);
        let assign46180_e44837: f64 = (-50.0);
        let assign46180_e44838: f64 = if assign46180_e44835 < assign46180_e44837 { 1.0 } else { 0.0 };
        locals.var_guard498 = assign46180_e44838;
        locals.var_guard498_rv = 0.0;

        let (assign46190_e44862, assign46190_e44862_d_n2, assign46190_e44862_d_n4, assign46190_e44862_d_n6,) = {
    if ((locals.var_guard497 == 0.0) && (locals.var_guard498 != 0.0)) {
        let assign46190_e44845: f64 = (p.p0 * p.p2);
        let assign46190_e44848: f64 = (locals.var_cofsmt0 * (nv6 - nv2));
        let assign46190_e44851: f64 = (locals.var_cofsmt * p.p28);
        let assign46190_e44854: f64 = ((nv6 - nv2) - p.p27);
        let assign46190_e44856: f64 = (assign46190_e44854 / p.p28);
        let assign46190_e44857: f64 = (assign46190_e44856).exp();
        let assign46190_e44858: f64 = (assign46190_e44851 * assign46190_e44857);
        let assign46190_e44859: f64 = (assign46190_e44848 + assign46190_e44858);
        let assign46190_e44860: f64 = (assign46190_e44845 * assign46190_e44859);
        (assign46190_e44860, (assign46190_e44845 * ((-locals.var_cofsmt0) + (assign46190_e44851 * (assign46190_e44857 * (-1.0 / p.p28))))), (assign46190_e44845 * ((locals.var_cofsmt0_dn4 * (nv6 - nv2)) + ((locals.var_cofsmt_dn4 * p.p28) * assign46190_e44857))), (assign46190_e44845 * (locals.var_cofsmt0 + (assign46190_e44851 * (assign46190_e44857 * (1.0 / p.p28))))),)
    } else {
        (locals.var_qofs, locals.var_qofs_dn2, locals.var_qofs_dn4, locals.var_qofs_dn6,)
    }
};
        locals.var_qofs = assign46190_e44862;
        locals.var_qofs_dn2 = assign46190_e44862_d_n2;
        locals.var_qofs_dn4 = assign46190_e44862_d_n4;
        locals.var_qofs_dn6 = assign46190_e44862_d_n6;
        locals.var_qofs_rv = 0.0;

        let (assign46200_e44890, assign46200_e44890_d_n2, assign46200_e44890_d_n4, assign46200_e44890_d_n6,) = {
    if ((locals.var_guard497 == 0.0) && (locals.var_guard498 == 0.0)) {
        let assign46200_e44870: f64 = (p.p0 * p.p2);
        let assign46200_e44873: f64 = (locals.var_cofsmt0 * (nv6 - nv2));
        let assign46200_e44876: f64 = (locals.var_cofsmt * p.p28);
        let assign46200_e44880: f64 = ((nv6 - nv2) - p.p27);
        let assign46200_e44882: f64 = (assign46200_e44880 / p.p28);
        let assign46200_e44883: f64 = (assign46200_e44882).exp();
        let assign46200_e44884: f64 = (1.0 + assign46200_e44883);
        let assign46200_e44885: f64 = (assign46200_e44884).ln();
        let assign46200_e44886: f64 = (assign46200_e44876 * assign46200_e44885);
        let assign46200_e44887: f64 = (assign46200_e44873 + assign46200_e44886);
        let assign46200_e44888: f64 = (assign46200_e44870 * assign46200_e44887);
        (assign46200_e44888, (assign46200_e44870 * ((-locals.var_cofsmt0) + (assign46200_e44876 * ((assign46200_e44883 * (-1.0 / p.p28)) / assign46200_e44884)))), (assign46200_e44870 * ((locals.var_cofsmt0_dn4 * (nv6 - nv2)) + ((locals.var_cofsmt_dn4 * p.p28) * assign46200_e44885))), (assign46200_e44870 * (locals.var_cofsmt0 + (assign46200_e44876 * ((assign46200_e44883 * (1.0 / p.p28)) / assign46200_e44884)))),)
    } else {
        (locals.var_qofs, locals.var_qofs_dn2, locals.var_qofs_dn4, locals.var_qofs_dn6,)
    }
};
        locals.var_qofs = assign46200_e44890;
        locals.var_qofs_dn2 = assign46200_e44890_d_n2;
        locals.var_qofs_dn4 = assign46200_e44890_d_n4;
        locals.var_qofs_dn6 = assign46200_e44890_d_n6;
        locals.var_qofs_rv = 0.0;

        let assign46210_e44893: f64 = ((nv6 - nv0) - p.p27);
        let assign46210_e44895: f64 = (assign46210_e44893 / p.p28);
        let assign46210_e44897: f64 = if assign46210_e44895 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard499 = assign46210_e44897;
        locals.var_guard499_rv = 0.0;

        let (assign46220_e44913, assign46220_e44913_d_n0, assign46220_e44913_d_n4, assign46220_e44913_d_n6,) = {
    if (locals.var_guard499 != 0.0) {
        let assign46220_e44901: f64 = (p.p0 * p.p2);
        let assign46220_e44904: f64 = (locals.var_cofdmt0 * (nv6 - nv0));
        let assign46220_e44908: f64 = ((nv6 - nv0) - p.p27);
        let assign46220_e44909: f64 = (locals.var_cofdmt * assign46220_e44908);
        let assign46220_e44910: f64 = (assign46220_e44904 + assign46220_e44909);
        let assign46220_e44911: f64 = (assign46220_e44901 * assign46220_e44910);
        (assign46220_e44911, (assign46220_e44901 * ((-locals.var_cofdmt0) + (-locals.var_cofdmt))), (assign46220_e44901 * ((locals.var_cofdmt0_dn4 * (nv6 - nv0)) + (locals.var_cofdmt_dn4 * assign46220_e44908))), (assign46220_e44901 * (locals.var_cofdmt0 + locals.var_cofdmt)),)
    } else {
        (locals.var_qofd, locals.var_qofd_dn0, locals.var_qofd_dn4, locals.var_qofd_dn6,)
    }
};
        locals.var_qofd = assign46220_e44913;
        locals.var_qofd_dn0 = assign46220_e44913_d_n0;
        locals.var_qofd_dn4 = assign46220_e44913_d_n4;
        locals.var_qofd_dn6 = assign46220_e44913_d_n6;
        locals.var_qofd_rv = 0.0;

        let assign46230_e44916: f64 = ((nv6 - nv0) - p.p27);
        let assign46230_e44918: f64 = (assign46230_e44916 / p.p28);
        let assign46230_e44920: f64 = (-50.0);
        let assign46230_e44921: f64 = if assign46230_e44918 < assign46230_e44920 { 1.0 } else { 0.0 };
        locals.var_guard500 = assign46230_e44921;
        locals.var_guard500_rv = 0.0;

        let (assign46240_e44945, assign46240_e44945_d_n0, assign46240_e44945_d_n4, assign46240_e44945_d_n6,) = {
    if ((locals.var_guard499 == 0.0) && (locals.var_guard500 != 0.0)) {
        let assign46240_e44928: f64 = (p.p0 * p.p2);
        let assign46240_e44931: f64 = (locals.var_cofdmt0 * (nv6 - nv0));
        let assign46240_e44934: f64 = (locals.var_cofdmt * p.p28);
        let assign46240_e44937: f64 = ((nv6 - nv0) - p.p27);
        let assign46240_e44939: f64 = (assign46240_e44937 / p.p28);
        let assign46240_e44940: f64 = (assign46240_e44939).exp();
        let assign46240_e44941: f64 = (assign46240_e44934 * assign46240_e44940);
        let assign46240_e44942: f64 = (assign46240_e44931 + assign46240_e44941);
        let assign46240_e44943: f64 = (assign46240_e44928 * assign46240_e44942);
        (assign46240_e44943, (assign46240_e44928 * ((-locals.var_cofdmt0) + (assign46240_e44934 * (assign46240_e44940 * (-1.0 / p.p28))))), (assign46240_e44928 * ((locals.var_cofdmt0_dn4 * (nv6 - nv0)) + ((locals.var_cofdmt_dn4 * p.p28) * assign46240_e44940))), (assign46240_e44928 * (locals.var_cofdmt0 + (assign46240_e44934 * (assign46240_e44940 * (1.0 / p.p28))))),)
    } else {
        (locals.var_qofd, locals.var_qofd_dn0, locals.var_qofd_dn4, locals.var_qofd_dn6,)
    }
};
        locals.var_qofd = assign46240_e44945;
        locals.var_qofd_dn0 = assign46240_e44945_d_n0;
        locals.var_qofd_dn4 = assign46240_e44945_d_n4;
        locals.var_qofd_dn6 = assign46240_e44945_d_n6;
        locals.var_qofd_rv = 0.0;

        let (assign46250_e44973, assign46250_e44973_d_n0, assign46250_e44973_d_n4, assign46250_e44973_d_n6,) = {
    if ((locals.var_guard499 == 0.0) && (locals.var_guard500 == 0.0)) {
        let assign46250_e44953: f64 = (p.p0 * p.p2);
        let assign46250_e44956: f64 = (locals.var_cofdmt0 * (nv6 - nv0));
        let assign46250_e44959: f64 = (locals.var_cofdmt * p.p28);
        let assign46250_e44963: f64 = ((nv6 - nv0) - p.p27);
        let assign46250_e44965: f64 = (assign46250_e44963 / p.p28);
        let assign46250_e44966: f64 = (assign46250_e44965).exp();
        let assign46250_e44967: f64 = (1.0 + assign46250_e44966);
        let assign46250_e44968: f64 = (assign46250_e44967).ln();
        let assign46250_e44969: f64 = (assign46250_e44959 * assign46250_e44968);
        let assign46250_e44970: f64 = (assign46250_e44956 + assign46250_e44969);
        let assign46250_e44971: f64 = (assign46250_e44953 * assign46250_e44970);
        (assign46250_e44971, (assign46250_e44953 * ((-locals.var_cofdmt0) + (assign46250_e44959 * ((assign46250_e44966 * (-1.0 / p.p28)) / assign46250_e44967)))), (assign46250_e44953 * ((locals.var_cofdmt0_dn4 * (nv6 - nv0)) + ((locals.var_cofdmt_dn4 * p.p28) * assign46250_e44968))), (assign46250_e44953 * (locals.var_cofdmt0 + (assign46250_e44959 * ((assign46250_e44966 * (1.0 / p.p28)) / assign46250_e44967)))),)
    } else {
        (locals.var_qofd, locals.var_qofd_dn0, locals.var_qofd_dn4, locals.var_qofd_dn6,)
    }
};
        locals.var_qofd = assign46250_e44973;
        locals.var_qofd_dn0 = assign46250_e44973_d_n0;
        locals.var_qofd_dn4 = assign46250_e44973_d_n4;
        locals.var_qofd_dn6 = assign46250_e44973_d_n6;
        locals.var_qofd_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_73(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let assign46260_e44976: f64 = ((nv2 - nv0) - p.p27);
        let assign46260_e44978: f64 = (assign46260_e44976 / p.p28);
        let assign46260_e44980: f64 = if assign46260_e44978 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard501 = assign46260_e44980;
        locals.var_guard501_rv = 0.0;

        let (assign46270_e44996, assign46270_e44996_d_n0, assign46270_e44996_d_n2, assign46270_e44996_d_n4,) = {
    if (locals.var_guard501 != 0.0) {
        let assign46270_e44984: f64 = (p.p0 * p.p2);
        let assign46270_e44987: f64 = (locals.var_cofdsmt0 * (nv2 - nv0));
        let assign46270_e44991: f64 = ((nv2 - nv0) - p.p27);
        let assign46270_e44992: f64 = (locals.var_cofdsmt * assign46270_e44991);
        let assign46270_e44993: f64 = (assign46270_e44987 + assign46270_e44992);
        let assign46270_e44994: f64 = (assign46270_e44984 * assign46270_e44993);
        (assign46270_e44994, (assign46270_e44984 * ((-locals.var_cofdsmt0) + (-locals.var_cofdsmt))), (assign46270_e44984 * (locals.var_cofdsmt0 + locals.var_cofdsmt)), (assign46270_e44984 * ((locals.var_cofdsmt0_dn4 * (nv2 - nv0)) + (locals.var_cofdsmt_dn4 * assign46270_e44991))),)
    } else {
        (locals.var_qofds, locals.var_qofds_dn0, locals.var_qofds_dn2, locals.var_qofds_dn4,)
    }
};
        locals.var_qofds = assign46270_e44996;
        locals.var_qofds_dn0 = assign46270_e44996_d_n0;
        locals.var_qofds_dn2 = assign46270_e44996_d_n2;
        locals.var_qofds_dn4 = assign46270_e44996_d_n4;
        locals.var_qofds_rv = 0.0;

        let assign46280_e44999: f64 = ((nv2 - nv0) - p.p27);
        let assign46280_e45001: f64 = (assign46280_e44999 / p.p28);
        let assign46280_e45003: f64 = (-50.0);
        let assign46280_e45004: f64 = if assign46280_e45001 < assign46280_e45003 { 1.0 } else { 0.0 };
        locals.var_guard502 = assign46280_e45004;
        locals.var_guard502_rv = 0.0;

        let (assign46290_e45028, assign46290_e45028_d_n0, assign46290_e45028_d_n2, assign46290_e45028_d_n4,) = {
    if ((locals.var_guard501 == 0.0) && (locals.var_guard502 != 0.0)) {
        let assign46290_e45011: f64 = (p.p0 * p.p2);
        let assign46290_e45014: f64 = (locals.var_cofdsmt0 * (nv2 - nv0));
        let assign46290_e45017: f64 = (locals.var_cofdsmt * p.p28);
        let assign46290_e45020: f64 = ((nv2 - nv0) - p.p27);
        let assign46290_e45022: f64 = (assign46290_e45020 / p.p28);
        let assign46290_e45023: f64 = (assign46290_e45022).exp();
        let assign46290_e45024: f64 = (assign46290_e45017 * assign46290_e45023);
        let assign46290_e45025: f64 = (assign46290_e45014 + assign46290_e45024);
        let assign46290_e45026: f64 = (assign46290_e45011 * assign46290_e45025);
        (assign46290_e45026, (assign46290_e45011 * ((-locals.var_cofdsmt0) + (assign46290_e45017 * (assign46290_e45023 * (-1.0 / p.p28))))), (assign46290_e45011 * (locals.var_cofdsmt0 + (assign46290_e45017 * (assign46290_e45023 * (1.0 / p.p28))))), (assign46290_e45011 * ((locals.var_cofdsmt0_dn4 * (nv2 - nv0)) + ((locals.var_cofdsmt_dn4 * p.p28) * assign46290_e45023))),)
    } else {
        (locals.var_qofds, locals.var_qofds_dn0, locals.var_qofds_dn2, locals.var_qofds_dn4,)
    }
};
        locals.var_qofds = assign46290_e45028;
        locals.var_qofds_dn0 = assign46290_e45028_d_n0;
        locals.var_qofds_dn2 = assign46290_e45028_d_n2;
        locals.var_qofds_dn4 = assign46290_e45028_d_n4;
        locals.var_qofds_rv = 0.0;

        let (assign46300_e45056, assign46300_e45056_d_n0, assign46300_e45056_d_n2, assign46300_e45056_d_n4,) = {
    if ((locals.var_guard501 == 0.0) && (locals.var_guard502 == 0.0)) {
        let assign46300_e45036: f64 = (p.p0 * p.p2);
        let assign46300_e45039: f64 = (locals.var_cofdsmt0 * (nv2 - nv0));
        let assign46300_e45042: f64 = (locals.var_cofdsmt * p.p28);
        let assign46300_e45046: f64 = ((nv2 - nv0) - p.p27);
        let assign46300_e45048: f64 = (assign46300_e45046 / p.p28);
        let assign46300_e45049: f64 = (assign46300_e45048).exp();
        let assign46300_e45050: f64 = (1.0 + assign46300_e45049);
        let assign46300_e45051: f64 = (assign46300_e45050).ln();
        let assign46300_e45052: f64 = (assign46300_e45042 * assign46300_e45051);
        let assign46300_e45053: f64 = (assign46300_e45039 + assign46300_e45052);
        let assign46300_e45054: f64 = (assign46300_e45036 * assign46300_e45053);
        (assign46300_e45054, (assign46300_e45036 * ((-locals.var_cofdsmt0) + (assign46300_e45042 * ((assign46300_e45049 * (-1.0 / p.p28)) / assign46300_e45050)))), (assign46300_e45036 * (locals.var_cofdsmt0 + (assign46300_e45042 * ((assign46300_e45049 * (1.0 / p.p28)) / assign46300_e45050)))), (assign46300_e45036 * ((locals.var_cofdsmt0_dn4 * (nv2 - nv0)) + ((locals.var_cofdsmt_dn4 * p.p28) * assign46300_e45051))),)
    } else {
        (locals.var_qofds, locals.var_qofds_dn0, locals.var_qofds_dn2, locals.var_qofds_dn4,)
    }
};
        locals.var_qofds = assign46300_e45056;
        locals.var_qofds_dn0 = assign46300_e45056_d_n0;
        locals.var_qofds_dn2 = assign46300_e45056_d_n2;
        locals.var_qofds_dn4 = assign46300_e45056_d_n4;
        locals.var_qofds_rv = 0.0;

        let assign46310_e45059: f64 = ((nv3 - nv2) - p.p27);
        let assign46310_e45061: f64 = (assign46310_e45059 / p.p28);
        let assign46310_e45063: f64 = if assign46310_e45061 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard503 = assign46310_e45063;
        locals.var_guard503_rv = 0.0;

        let (assign46320_e45079, assign46320_e45079_d_n2, assign46320_e45079_d_n3, assign46320_e45079_d_n4,) = {
    if (locals.var_guard503 != 0.0) {
        let assign46320_e45067: f64 = (p.p0 * p.p2);
        let assign46320_e45070: f64 = (locals.var_cofssubmt0 * (nv3 - nv2));
        let assign46320_e45074: f64 = ((nv3 - nv2) - p.p27);
        let assign46320_e45075: f64 = (locals.var_cofssubmt * assign46320_e45074);
        let assign46320_e45076: f64 = (assign46320_e45070 + assign46320_e45075);
        let assign46320_e45077: f64 = (assign46320_e45067 * assign46320_e45076);
        (assign46320_e45077, (assign46320_e45067 * ((-locals.var_cofssubmt0) + (-locals.var_cofssubmt))), (assign46320_e45067 * (locals.var_cofssubmt0 + locals.var_cofssubmt)), (assign46320_e45067 * ((locals.var_cofssubmt0_dn4 * (nv3 - nv2)) + (locals.var_cofssubmt_dn4 * assign46320_e45074))),)
    } else {
        (locals.var_qofssub, locals.var_qofssub_dn2, locals.var_qofssub_dn3, locals.var_qofssub_dn4,)
    }
};
        locals.var_qofssub = assign46320_e45079;
        locals.var_qofssub_dn2 = assign46320_e45079_d_n2;
        locals.var_qofssub_dn3 = assign46320_e45079_d_n3;
        locals.var_qofssub_dn4 = assign46320_e45079_d_n4;
        locals.var_qofssub_rv = 0.0;

        let assign46330_e45082: f64 = ((nv3 - nv2) - p.p27);
        let assign46330_e45084: f64 = (assign46330_e45082 / p.p28);
        let assign46330_e45086: f64 = (-50.0);
        let assign46330_e45087: f64 = if assign46330_e45084 < assign46330_e45086 { 1.0 } else { 0.0 };
        locals.var_guard504 = assign46330_e45087;
        locals.var_guard504_rv = 0.0;

        let (assign46340_e45111, assign46340_e45111_d_n2, assign46340_e45111_d_n3, assign46340_e45111_d_n4,) = {
    if ((locals.var_guard503 == 0.0) && (locals.var_guard504 != 0.0)) {
        let assign46340_e45094: f64 = (p.p0 * p.p2);
        let assign46340_e45097: f64 = (locals.var_cofssubmt0 * (nv3 - nv2));
        let assign46340_e45100: f64 = (locals.var_cofssubmt * p.p28);
        let assign46340_e45103: f64 = ((nv3 - nv2) - p.p27);
        let assign46340_e45105: f64 = (assign46340_e45103 / p.p28);
        let assign46340_e45106: f64 = (assign46340_e45105).exp();
        let assign46340_e45107: f64 = (assign46340_e45100 * assign46340_e45106);
        let assign46340_e45108: f64 = (assign46340_e45097 + assign46340_e45107);
        let assign46340_e45109: f64 = (assign46340_e45094 * assign46340_e45108);
        (assign46340_e45109, (assign46340_e45094 * ((-locals.var_cofssubmt0) + (assign46340_e45100 * (assign46340_e45106 * (-1.0 / p.p28))))), (assign46340_e45094 * (locals.var_cofssubmt0 + (assign46340_e45100 * (assign46340_e45106 * (1.0 / p.p28))))), (assign46340_e45094 * ((locals.var_cofssubmt0_dn4 * (nv3 - nv2)) + ((locals.var_cofssubmt_dn4 * p.p28) * assign46340_e45106))),)
    } else {
        (locals.var_qofssub, locals.var_qofssub_dn2, locals.var_qofssub_dn3, locals.var_qofssub_dn4,)
    }
};
        locals.var_qofssub = assign46340_e45111;
        locals.var_qofssub_dn2 = assign46340_e45111_d_n2;
        locals.var_qofssub_dn3 = assign46340_e45111_d_n3;
        locals.var_qofssub_dn4 = assign46340_e45111_d_n4;
        locals.var_qofssub_rv = 0.0;

        let (assign46350_e45139, assign46350_e45139_d_n2, assign46350_e45139_d_n3, assign46350_e45139_d_n4,) = {
    if ((locals.var_guard503 == 0.0) && (locals.var_guard504 == 0.0)) {
        let assign46350_e45119: f64 = (p.p0 * p.p2);
        let assign46350_e45122: f64 = (locals.var_cofssubmt0 * (nv3 - nv2));
        let assign46350_e45125: f64 = (locals.var_cofssubmt * p.p28);
        let assign46350_e45129: f64 = ((nv3 - nv2) - p.p27);
        let assign46350_e45131: f64 = (assign46350_e45129 / p.p28);
        let assign46350_e45132: f64 = (assign46350_e45131).exp();
        let assign46350_e45133: f64 = (1.0 + assign46350_e45132);
        let assign46350_e45134: f64 = (assign46350_e45133).ln();
        let assign46350_e45135: f64 = (assign46350_e45125 * assign46350_e45134);
        let assign46350_e45136: f64 = (assign46350_e45122 + assign46350_e45135);
        let assign46350_e45137: f64 = (assign46350_e45119 * assign46350_e45136);
        (assign46350_e45137, (assign46350_e45119 * ((-locals.var_cofssubmt0) + (assign46350_e45125 * ((assign46350_e45132 * (-1.0 / p.p28)) / assign46350_e45133)))), (assign46350_e45119 * (locals.var_cofssubmt0 + (assign46350_e45125 * ((assign46350_e45132 * (1.0 / p.p28)) / assign46350_e45133)))), (assign46350_e45119 * ((locals.var_cofssubmt0_dn4 * (nv3 - nv2)) + ((locals.var_cofssubmt_dn4 * p.p28) * assign46350_e45134))),)
    } else {
        (locals.var_qofssub, locals.var_qofssub_dn2, locals.var_qofssub_dn3, locals.var_qofssub_dn4,)
    }
};
        locals.var_qofssub = assign46350_e45139;
        locals.var_qofssub_dn2 = assign46350_e45139_d_n2;
        locals.var_qofssub_dn3 = assign46350_e45139_d_n3;
        locals.var_qofssub_dn4 = assign46350_e45139_d_n4;
        locals.var_qofssub_rv = 0.0;

        let assign46360_e45142: f64 = ((nv3 - nv0) - p.p27);
        let assign46360_e45144: f64 = (assign46360_e45142 / p.p28);
        let assign46360_e45146: f64 = if assign46360_e45144 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard505 = assign46360_e45146;
        locals.var_guard505_rv = 0.0;

        let (assign46370_e45162, assign46370_e45162_d_n0, assign46370_e45162_d_n3, assign46370_e45162_d_n4,) = {
    if (locals.var_guard505 != 0.0) {
        let assign46370_e45150: f64 = (p.p0 * p.p2);
        let assign46370_e45153: f64 = (locals.var_cofdsubmt0 * (nv3 - nv0));
        let assign46370_e45157: f64 = ((nv3 - nv0) - p.p27);
        let assign46370_e45158: f64 = (locals.var_cofdsubmt * assign46370_e45157);
        let assign46370_e45159: f64 = (assign46370_e45153 + assign46370_e45158);
        let assign46370_e45160: f64 = (assign46370_e45150 * assign46370_e45159);
        (assign46370_e45160, (assign46370_e45150 * ((-locals.var_cofdsubmt0) + (-locals.var_cofdsubmt))), (assign46370_e45150 * (locals.var_cofdsubmt0 + locals.var_cofdsubmt)), (assign46370_e45150 * ((locals.var_cofdsubmt0_dn4 * (nv3 - nv0)) + (locals.var_cofdsubmt_dn4 * assign46370_e45157))),)
    } else {
        (locals.var_qofdsub, locals.var_qofdsub_dn0, locals.var_qofdsub_dn3, locals.var_qofdsub_dn4,)
    }
};
        locals.var_qofdsub = assign46370_e45162;
        locals.var_qofdsub_dn0 = assign46370_e45162_d_n0;
        locals.var_qofdsub_dn3 = assign46370_e45162_d_n3;
        locals.var_qofdsub_dn4 = assign46370_e45162_d_n4;
        locals.var_qofdsub_rv = 0.0;

        let assign46380_e45165: f64 = ((nv3 - nv0) - p.p27);
        let assign46380_e45167: f64 = (assign46380_e45165 / p.p28);
        let assign46380_e45169: f64 = (-50.0);
        let assign46380_e45170: f64 = if assign46380_e45167 < assign46380_e45169 { 1.0 } else { 0.0 };
        locals.var_guard506 = assign46380_e45170;
        locals.var_guard506_rv = 0.0;

        let (assign46390_e45194, assign46390_e45194_d_n0, assign46390_e45194_d_n3, assign46390_e45194_d_n4,) = {
    if ((locals.var_guard505 == 0.0) && (locals.var_guard506 != 0.0)) {
        let assign46390_e45177: f64 = (p.p0 * p.p2);
        let assign46390_e45180: f64 = (locals.var_cofdsubmt0 * (nv3 - nv0));
        let assign46390_e45183: f64 = (locals.var_cofdsubmt * p.p28);
        let assign46390_e45186: f64 = ((nv3 - nv0) - p.p27);
        let assign46390_e45188: f64 = (assign46390_e45186 / p.p28);
        let assign46390_e45189: f64 = (assign46390_e45188).exp();
        let assign46390_e45190: f64 = (assign46390_e45183 * assign46390_e45189);
        let assign46390_e45191: f64 = (assign46390_e45180 + assign46390_e45190);
        let assign46390_e45192: f64 = (assign46390_e45177 * assign46390_e45191);
        (assign46390_e45192, (assign46390_e45177 * ((-locals.var_cofdsubmt0) + (assign46390_e45183 * (assign46390_e45189 * (-1.0 / p.p28))))), (assign46390_e45177 * (locals.var_cofdsubmt0 + (assign46390_e45183 * (assign46390_e45189 * (1.0 / p.p28))))), (assign46390_e45177 * ((locals.var_cofdsubmt0_dn4 * (nv3 - nv0)) + ((locals.var_cofdsubmt_dn4 * p.p28) * assign46390_e45189))),)
    } else {
        (locals.var_qofdsub, locals.var_qofdsub_dn0, locals.var_qofdsub_dn3, locals.var_qofdsub_dn4,)
    }
};
        locals.var_qofdsub = assign46390_e45194;
        locals.var_qofdsub_dn0 = assign46390_e45194_d_n0;
        locals.var_qofdsub_dn3 = assign46390_e45194_d_n3;
        locals.var_qofdsub_dn4 = assign46390_e45194_d_n4;
        locals.var_qofdsub_rv = 0.0;

        let (assign46400_e45222, assign46400_e45222_d_n0, assign46400_e45222_d_n3, assign46400_e45222_d_n4,) = {
    if ((locals.var_guard505 == 0.0) && (locals.var_guard506 == 0.0)) {
        let assign46400_e45202: f64 = (p.p0 * p.p2);
        let assign46400_e45205: f64 = (locals.var_cofdsubmt0 * (nv3 - nv0));
        let assign46400_e45208: f64 = (locals.var_cofdsubmt * p.p28);
        let assign46400_e45212: f64 = ((nv3 - nv0) - p.p27);
        let assign46400_e45214: f64 = (assign46400_e45212 / p.p28);
        let assign46400_e45215: f64 = (assign46400_e45214).exp();
        let assign46400_e45216: f64 = (1.0 + assign46400_e45215);
        let assign46400_e45217: f64 = (assign46400_e45216).ln();
        let assign46400_e45218: f64 = (assign46400_e45208 * assign46400_e45217);
        let assign46400_e45219: f64 = (assign46400_e45205 + assign46400_e45218);
        let assign46400_e45220: f64 = (assign46400_e45202 * assign46400_e45219);
        (assign46400_e45220, (assign46400_e45202 * ((-locals.var_cofdsubmt0) + (assign46400_e45208 * ((assign46400_e45215 * (-1.0 / p.p28)) / assign46400_e45216)))), (assign46400_e45202 * (locals.var_cofdsubmt0 + (assign46400_e45208 * ((assign46400_e45215 * (1.0 / p.p28)) / assign46400_e45216)))), (assign46400_e45202 * ((locals.var_cofdsubmt0_dn4 * (nv3 - nv0)) + ((locals.var_cofdsubmt_dn4 * p.p28) * assign46400_e45217))),)
    } else {
        (locals.var_qofdsub, locals.var_qofdsub_dn0, locals.var_qofdsub_dn3, locals.var_qofdsub_dn4,)
    }
};
        locals.var_qofdsub = assign46400_e45222;
        locals.var_qofdsub_dn0 = assign46400_e45222_d_n0;
        locals.var_qofdsub_dn3 = assign46400_e45222_d_n3;
        locals.var_qofdsub_dn4 = assign46400_e45222_d_n4;
        locals.var_qofdsub_rv = 0.0;

        let assign46410_e45225: f64 = ((nv6 - nv3) - p.p27);
        let assign46410_e45227: f64 = (assign46410_e45225 / p.p28);
        let assign46410_e45229: f64 = if assign46410_e45227 > 50.0 { 1.0 } else { 0.0 };
        locals.var_guard507 = assign46410_e45229;
        locals.var_guard507_rv = 0.0;

        let (assign46420_e45245, assign46420_e45245_d_n3, assign46420_e45245_d_n4, assign46420_e45245_d_n6,) = {
    if (locals.var_guard507 != 0.0) {
        let assign46420_e45233: f64 = (p.p0 * p.p2);
        let assign46420_e45236: f64 = (locals.var_cofgsubmt0 * (nv6 - nv3));
        let assign46420_e45240: f64 = ((nv6 - nv3) - p.p27);
        let assign46420_e45241: f64 = (locals.var_cofgsubmt * assign46420_e45240);
        let assign46420_e45242: f64 = (assign46420_e45236 + assign46420_e45241);
        let assign46420_e45243: f64 = (assign46420_e45233 * assign46420_e45242);
        (assign46420_e45243, (assign46420_e45233 * ((-locals.var_cofgsubmt0) + (-locals.var_cofgsubmt))), (assign46420_e45233 * ((locals.var_cofgsubmt0_dn4 * (nv6 - nv3)) + (locals.var_cofgsubmt_dn4 * assign46420_e45240))), (assign46420_e45233 * (locals.var_cofgsubmt0 + locals.var_cofgsubmt)),)
    } else {
        (locals.var_qofgsub, locals.var_qofgsub_dn3, locals.var_qofgsub_dn4, locals.var_qofgsub_dn6,)
    }
};
        locals.var_qofgsub = assign46420_e45245;
        locals.var_qofgsub_dn3 = assign46420_e45245_d_n3;
        locals.var_qofgsub_dn4 = assign46420_e45245_d_n4;
        locals.var_qofgsub_dn6 = assign46420_e45245_d_n6;
        locals.var_qofgsub_rv = 0.0;

        let assign46430_e45248: f64 = ((nv6 - nv3) - p.p27);
        let assign46430_e45250: f64 = (assign46430_e45248 / p.p28);
        let assign46430_e45252: f64 = (-50.0);
        let assign46430_e45253: f64 = if assign46430_e45250 < assign46430_e45252 { 1.0 } else { 0.0 };
        locals.var_guard508 = assign46430_e45253;
        locals.var_guard508_rv = 0.0;

        let (assign46440_e45277, assign46440_e45277_d_n3, assign46440_e45277_d_n4, assign46440_e45277_d_n6,) = {
    if ((locals.var_guard507 == 0.0) && (locals.var_guard508 != 0.0)) {
        let assign46440_e45260: f64 = (p.p0 * p.p2);
        let assign46440_e45263: f64 = (locals.var_cofgsubmt0 * (nv6 - nv3));
        let assign46440_e45266: f64 = (locals.var_cofgsubmt * p.p28);
        let assign46440_e45269: f64 = ((nv6 - nv3) - p.p27);
        let assign46440_e45271: f64 = (assign46440_e45269 / p.p28);
        let assign46440_e45272: f64 = (assign46440_e45271).exp();
        let assign46440_e45273: f64 = (assign46440_e45266 * assign46440_e45272);
        let assign46440_e45274: f64 = (assign46440_e45263 + assign46440_e45273);
        let assign46440_e45275: f64 = (assign46440_e45260 * assign46440_e45274);
        (assign46440_e45275, (assign46440_e45260 * ((-locals.var_cofgsubmt0) + (assign46440_e45266 * (assign46440_e45272 * (-1.0 / p.p28))))), (assign46440_e45260 * ((locals.var_cofgsubmt0_dn4 * (nv6 - nv3)) + ((locals.var_cofgsubmt_dn4 * p.p28) * assign46440_e45272))), (assign46440_e45260 * (locals.var_cofgsubmt0 + (assign46440_e45266 * (assign46440_e45272 * (1.0 / p.p28))))),)
    } else {
        (locals.var_qofgsub, locals.var_qofgsub_dn3, locals.var_qofgsub_dn4, locals.var_qofgsub_dn6,)
    }
};
        locals.var_qofgsub = assign46440_e45277;
        locals.var_qofgsub_dn3 = assign46440_e45277_d_n3;
        locals.var_qofgsub_dn4 = assign46440_e45277_d_n4;
        locals.var_qofgsub_dn6 = assign46440_e45277_d_n6;
        locals.var_qofgsub_rv = 0.0;

        let (assign46450_e45305, assign46450_e45305_d_n3, assign46450_e45305_d_n4, assign46450_e45305_d_n6,) = {
    if ((locals.var_guard507 == 0.0) && (locals.var_guard508 == 0.0)) {
        let assign46450_e45285: f64 = (p.p0 * p.p2);
        let assign46450_e45288: f64 = (locals.var_cofgsubmt0 * (nv6 - nv3));
        let assign46450_e45291: f64 = (locals.var_cofgsubmt * p.p28);
        let assign46450_e45295: f64 = ((nv6 - nv3) - p.p27);
        let assign46450_e45297: f64 = (assign46450_e45295 / p.p28);
        let assign46450_e45298: f64 = (assign46450_e45297).exp();
        let assign46450_e45299: f64 = (1.0 + assign46450_e45298);
        let assign46450_e45300: f64 = (assign46450_e45299).ln();
        let assign46450_e45301: f64 = (assign46450_e45291 * assign46450_e45300);
        let assign46450_e45302: f64 = (assign46450_e45288 + assign46450_e45301);
        let assign46450_e45303: f64 = (assign46450_e45285 * assign46450_e45302);
        (assign46450_e45303, (assign46450_e45285 * ((-locals.var_cofgsubmt0) + (assign46450_e45291 * ((assign46450_e45298 * (-1.0 / p.p28)) / assign46450_e45299)))), (assign46450_e45285 * ((locals.var_cofgsubmt0_dn4 * (nv6 - nv3)) + ((locals.var_cofgsubmt_dn4 * p.p28) * assign46450_e45300))), (assign46450_e45285 * (locals.var_cofgsubmt0 + (assign46450_e45291 * ((assign46450_e45298 * (1.0 / p.p28)) / assign46450_e45299)))),)
    } else {
        (locals.var_qofgsub, locals.var_qofgsub_dn3, locals.var_qofgsub_dn4, locals.var_qofgsub_dn6,)
    }
};
        locals.var_qofgsub = assign46450_e45305;
        locals.var_qofgsub_dn3 = assign46450_e45305_d_n3;
        locals.var_qofgsub_dn4 = assign46450_e45305_d_n4;
        locals.var_qofgsub_dn6 = assign46450_e45305_d_n6;
        locals.var_qofgsub_rv = 0.0;

        let assign46690_e45519: f64 = if p.p320 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard523 = assign46690_e45519;
        locals.var_guard523_rv = 0.0;

    }
}

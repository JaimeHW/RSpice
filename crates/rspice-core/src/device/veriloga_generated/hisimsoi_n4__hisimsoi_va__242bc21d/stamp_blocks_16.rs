#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_129(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign36610_e51290, assign36610_e51290_d_n0, assign36610_e51290_d_n2, assign36610_e51290_d_n6, assign36610_e51290_d_n7, assign36610_e51290_d_n10, assign36610_e51290_d_n11, assign36610_e51290_d_n12, assign36610_e51290_d_n16, assign36610_e51290_d_n17, assign36610_e51290_d_n18,) = {
    if ((locals.var_guard1205 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18,)
    }
};
        locals.var_qs_nqs = assign36610_e51290;
        locals.var_qs_nqs_dn0 = assign36610_e51290_d_n0;
        locals.var_qs_nqs_dn2 = assign36610_e51290_d_n2;
        locals.var_qs_nqs_dn6 = assign36610_e51290_d_n6;
        locals.var_qs_nqs_dn7 = assign36610_e51290_d_n7;
        locals.var_qs_nqs_dn10 = assign36610_e51290_d_n10;
        locals.var_qs_nqs_dn11 = assign36610_e51290_d_n11;
        locals.var_qs_nqs_dn12 = assign36610_e51290_d_n12;
        locals.var_qs_nqs_dn16 = assign36610_e51290_d_n16;
        locals.var_qs_nqs_dn17 = assign36610_e51290_d_n17;
        locals.var_qs_nqs_dn18 = assign36610_e51290_d_n18;
        locals.var_qs_nqs_rv = 0.0;

        let (assign36620_e51298, assign36620_e51298_d_n0, assign36620_e51298_d_n2, assign36620_e51298_d_n6, assign36620_e51298_d_n7, assign36620_e51298_d_n10, assign36620_e51298_d_n11, assign36620_e51298_d_n12, assign36620_e51298_d_n13, assign36620_e51298_d_n15, assign36620_e51298_d_n16, assign36620_e51298_d_n17, assign36620_e51298_d_n18,) = {
    if ((locals.var_guard1205 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18,)
    }
};
        locals.var_qg_nqs = assign36620_e51298;
        locals.var_qg_nqs_dn0 = assign36620_e51298_d_n0;
        locals.var_qg_nqs_dn2 = assign36620_e51298_d_n2;
        locals.var_qg_nqs_dn6 = assign36620_e51298_d_n6;
        locals.var_qg_nqs_dn7 = assign36620_e51298_d_n7;
        locals.var_qg_nqs_dn10 = assign36620_e51298_d_n10;
        locals.var_qg_nqs_dn11 = assign36620_e51298_d_n11;
        locals.var_qg_nqs_dn12 = assign36620_e51298_d_n12;
        locals.var_qg_nqs_dn13 = assign36620_e51298_d_n13;
        locals.var_qg_nqs_dn15 = assign36620_e51298_d_n15;
        locals.var_qg_nqs_dn16 = assign36620_e51298_d_n16;
        locals.var_qg_nqs_dn17 = assign36620_e51298_d_n17;
        locals.var_qg_nqs_dn18 = assign36620_e51298_d_n18;
        locals.var_qg_nqs_rv = 0.0;

        let (assign36630_e51306, assign36630_e51306_d_n13,) = {
    if ((locals.var_guard1205 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn13,)
    }
};
        locals.var_qb_nqs = assign36630_e51306;
        locals.var_qb_nqs_dn13 = assign36630_e51306_d_n13;
        locals.var_qb_nqs_rv = 0.0;

        let assign36660_e51311: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1210 = assign36660_e51311;
        locals.var_guard1210_rv = 0.0;

        let (assign36670_e51315, assign36670_e51315_d_n0, assign36670_e51315_d_n2, assign36670_e51315_d_n6, assign36670_e51315_d_n7, assign36670_e51315_d_n10, assign36670_e51315_d_n11, assign36670_e51315_d_n12, assign36670_e51315_d_n17,) = {
    if (locals.var_guard1210 != 0.0) {
        (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn12, locals.var_idse_dn17,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign36670_e51315;
        locals.var_ids_dn0 = assign36670_e51315_d_n0;
        locals.var_ids_dn2 = assign36670_e51315_d_n2;
        locals.var_ids_dn6 = assign36670_e51315_d_n6;
        locals.var_ids_dn7 = assign36670_e51315_d_n7;
        locals.var_ids_dn10 = assign36670_e51315_d_n10;
        locals.var_ids_dn11 = assign36670_e51315_d_n11;
        locals.var_ids_dn12 = assign36670_e51315_d_n12;
        locals.var_ids_dn17 = assign36670_e51315_d_n17;
        locals.var_ids_rv = 0.0;

        let (assign36680_e51319, assign36680_e51319_d_n0, assign36680_e51319_d_n2, assign36680_e51319_d_n6, assign36680_e51319_d_n7, assign36680_e51319_d_n10, assign36680_e51319_d_n11, assign36680_e51319_d_n12, assign36680_e51319_d_n17,) = {
    if (locals.var_guard1210 != 0.0) {
        (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign36680_e51319;
        locals.var_isub_dn0 = assign36680_e51319_d_n0;
        locals.var_isub_dn2 = assign36680_e51319_d_n2;
        locals.var_isub_dn6 = assign36680_e51319_d_n6;
        locals.var_isub_dn7 = assign36680_e51319_d_n7;
        locals.var_isub_dn10 = assign36680_e51319_d_n10;
        locals.var_isub_dn11 = assign36680_e51319_d_n11;
        locals.var_isub_dn12 = assign36680_e51319_d_n12;
        locals.var_isub_dn17 = assign36680_e51319_d_n17;
        locals.var_isub_rv = 0.0;

        let (assign36700_e51329, assign36700_e51329_d_n0, assign36700_e51329_d_n2, assign36700_e51329_d_n6, assign36700_e51329_d_n7, assign36700_e51329_d_n10, assign36700_e51329_d_n11, assign36700_e51329_d_n12, assign36700_e51329_d_n13, assign36700_e51329_d_n15, assign36700_e51329_d_n16, assign36700_e51329_d_n17, assign36700_e51329_d_n18,) = {
    if (locals.var_guard1210 != 0.0) {
        let assign36700_e51327: f64 = (locals.var_qge + locals.var_qg_nqs);
        (assign36700_e51327, (locals.var_qge_dn0 + locals.var_qg_nqs_dn0), (locals.var_qge_dn2 + locals.var_qg_nqs_dn2), (locals.var_qge_dn6 + locals.var_qg_nqs_dn6), (locals.var_qge_dn7 + locals.var_qg_nqs_dn7), (locals.var_qge_dn10 + locals.var_qg_nqs_dn10), (locals.var_qge_dn11 + locals.var_qg_nqs_dn11), (locals.var_qge_dn12 + locals.var_qg_nqs_dn12), (locals.var_qge_dn13 + locals.var_qg_nqs_dn13), (locals.var_qge_dn15 + locals.var_qg_nqs_dn15), (locals.var_qge_dn16 + locals.var_qg_nqs_dn16), (locals.var_qge_dn17 + locals.var_qg_nqs_dn17), (locals.var_qge_dn18 + locals.var_qg_nqs_dn18),)
    } else {
        (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12, locals.var_qg_dn13, locals.var_qg_dn15, locals.var_qg_dn16, locals.var_qg_dn17, locals.var_qg_dn18,)
    }
};
        locals.var_qg = assign36700_e51329;
        locals.var_qg_dn0 = assign36700_e51329_d_n0;
        locals.var_qg_dn2 = assign36700_e51329_d_n2;
        locals.var_qg_dn6 = assign36700_e51329_d_n6;
        locals.var_qg_dn7 = assign36700_e51329_d_n7;
        locals.var_qg_dn10 = assign36700_e51329_d_n10;
        locals.var_qg_dn11 = assign36700_e51329_d_n11;
        locals.var_qg_dn12 = assign36700_e51329_d_n12;
        locals.var_qg_dn13 = assign36700_e51329_d_n13;
        locals.var_qg_dn15 = assign36700_e51329_d_n15;
        locals.var_qg_dn16 = assign36700_e51329_d_n16;
        locals.var_qg_dn17 = assign36700_e51329_d_n17;
        locals.var_qg_dn18 = assign36700_e51329_d_n18;
        locals.var_qg_rv = 0.0;

        let (assign36710_e51335, assign36710_e51335_d_n0, assign36710_e51335_d_n2, assign36710_e51335_d_n6, assign36710_e51335_d_n7, assign36710_e51335_d_n10, assign36710_e51335_d_n11, assign36710_e51335_d_n12, assign36710_e51335_d_n13, assign36710_e51335_d_n15, assign36710_e51335_d_n16, assign36710_e51335_d_n17, assign36710_e51335_d_n18,) = {
    if (locals.var_guard1210 != 0.0) {
        let assign36710_e51333: f64 = (locals.var_qde + locals.var_qd_nqs);
        (assign36710_e51333, (locals.var_qde_dn0 + locals.var_qd_nqs_dn0), (locals.var_qde_dn2 + locals.var_qd_nqs_dn2), (locals.var_qde_dn6 + locals.var_qd_nqs_dn6), (locals.var_qde_dn7 + locals.var_qd_nqs_dn7), (locals.var_qde_dn10 + locals.var_qd_nqs_dn10), (locals.var_qde_dn11 + locals.var_qd_nqs_dn11), (locals.var_qde_dn12 + locals.var_qd_nqs_dn12), locals.var_qde_dn13, (locals.var_qde_dn15 + locals.var_qd_nqs_dn15), locals.var_qde_dn16, (locals.var_qde_dn17 + locals.var_qd_nqs_dn17), (locals.var_qde_dn18 + locals.var_qd_nqs_dn18),)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18,)
    }
};
        locals.var_qd = assign36710_e51335;
        locals.var_qd_dn0 = assign36710_e51335_d_n0;
        locals.var_qd_dn2 = assign36710_e51335_d_n2;
        locals.var_qd_dn6 = assign36710_e51335_d_n6;
        locals.var_qd_dn7 = assign36710_e51335_d_n7;
        locals.var_qd_dn10 = assign36710_e51335_d_n10;
        locals.var_qd_dn11 = assign36710_e51335_d_n11;
        locals.var_qd_dn12 = assign36710_e51335_d_n12;
        locals.var_qd_dn13 = assign36710_e51335_d_n13;
        locals.var_qd_dn15 = assign36710_e51335_d_n15;
        locals.var_qd_dn16 = assign36710_e51335_d_n16;
        locals.var_qd_dn17 = assign36710_e51335_d_n17;
        locals.var_qd_dn18 = assign36710_e51335_d_n18;
        locals.var_qd_rv = 0.0;

        let (assign36730_e51350, assign36730_e51350_d_n0, assign36730_e51350_d_n2, assign36730_e51350_d_n6, assign36730_e51350_d_n7, assign36730_e51350_d_n10, assign36730_e51350_d_n11, assign36730_e51350_d_n12, assign36730_e51350_d_n13, assign36730_e51350_d_n15, assign36730_e51350_d_n16, assign36730_e51350_d_n17, assign36730_e51350_d_n18,) = {
    if (locals.var_guard1210 != 0.0) {
        let assign36730_e51345: f64 = (locals.var_qge + locals.var_qde);
        let assign36730_e51347: f64 = (assign36730_e51345 + locals.var_qse);
        let assign36730_e51348: f64 = (-assign36730_e51347);
        (assign36730_e51348, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)), (-((locals.var_qge_dn15 + locals.var_qde_dn15) + locals.var_qse_dn15)), (-((locals.var_qge_dn16 + locals.var_qde_dn16) + locals.var_qse_dn16)), (-((locals.var_qge_dn17 + locals.var_qde_dn17) + locals.var_qse_dn17)), (-((locals.var_qge_dn18 + locals.var_qde_dn18) + locals.var_qse_dn18)),)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    }
};
        locals.var_qbe = assign36730_e51350;
        locals.var_qbe_dn0 = assign36730_e51350_d_n0;
        locals.var_qbe_dn2 = assign36730_e51350_d_n2;
        locals.var_qbe_dn6 = assign36730_e51350_d_n6;
        locals.var_qbe_dn7 = assign36730_e51350_d_n7;
        locals.var_qbe_dn10 = assign36730_e51350_d_n10;
        locals.var_qbe_dn11 = assign36730_e51350_d_n11;
        locals.var_qbe_dn12 = assign36730_e51350_d_n12;
        locals.var_qbe_dn13 = assign36730_e51350_d_n13;
        locals.var_qbe_dn15 = assign36730_e51350_d_n15;
        locals.var_qbe_dn16 = assign36730_e51350_d_n16;
        locals.var_qbe_dn17 = assign36730_e51350_d_n17;
        locals.var_qbe_dn18 = assign36730_e51350_d_n18;
        locals.var_qbe_rv = 0.0;

        let (assign36740_e51356, assign36740_e51356_d_n0, assign36740_e51356_d_n2, assign36740_e51356_d_n6, assign36740_e51356_d_n7, assign36740_e51356_d_n10, assign36740_e51356_d_n11, assign36740_e51356_d_n12, assign36740_e51356_d_n13, assign36740_e51356_d_n15, assign36740_e51356_d_n16, assign36740_e51356_d_n17, assign36740_e51356_d_n18,) = {
    if (locals.var_guard1210 != 0.0) {
        let assign36740_e51354: f64 = (locals.var_qbe + locals.var_qb_nqs);
        (assign36740_e51354, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, (locals.var_qbe_dn13 + locals.var_qb_nqs_dn13), locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18,)
    }
};
        locals.var_qb = assign36740_e51356;
        locals.var_qb_dn0 = assign36740_e51356_d_n0;
        locals.var_qb_dn2 = assign36740_e51356_d_n2;
        locals.var_qb_dn6 = assign36740_e51356_d_n6;
        locals.var_qb_dn7 = assign36740_e51356_d_n7;
        locals.var_qb_dn10 = assign36740_e51356_d_n10;
        locals.var_qb_dn11 = assign36740_e51356_d_n11;
        locals.var_qb_dn12 = assign36740_e51356_d_n12;
        locals.var_qb_dn13 = assign36740_e51356_d_n13;
        locals.var_qb_dn15 = assign36740_e51356_d_n15;
        locals.var_qb_dn16 = assign36740_e51356_d_n16;
        locals.var_qb_dn17 = assign36740_e51356_d_n17;
        locals.var_qb_dn18 = assign36740_e51356_d_n18;
        locals.var_qb_rv = 0.0;

        let (assign36750_e51362, assign36750_e51362_d_n0, assign36750_e51362_d_n2, assign36750_e51362_d_n6, assign36750_e51362_d_n7, assign36750_e51362_d_n10, assign36750_e51362_d_n11, assign36750_e51362_d_n12, assign36750_e51362_d_n17,) = {
    if (locals.var_guard1210 == 0.0) {
        let assign36750_e51360: f64 = (-locals.var_idse);
        (assign36750_e51360, (-locals.var_idse_dn0), (-locals.var_idse_dn2), (-locals.var_idse_dn6), (-locals.var_idse_dn7), (-locals.var_idse_dn10), (-locals.var_idse_dn11), (-locals.var_idse_dn12), (-locals.var_idse_dn17),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign36750_e51362;
        locals.var_ids_dn0 = assign36750_e51362_d_n0;
        locals.var_ids_dn2 = assign36750_e51362_d_n2;
        locals.var_ids_dn6 = assign36750_e51362_d_n6;
        locals.var_ids_dn7 = assign36750_e51362_d_n7;
        locals.var_ids_dn10 = assign36750_e51362_d_n10;
        locals.var_ids_dn11 = assign36750_e51362_d_n11;
        locals.var_ids_dn12 = assign36750_e51362_d_n12;
        locals.var_ids_dn17 = assign36750_e51362_d_n17;
        locals.var_ids_rv = 0.0;

        let (assign36770_e51372, assign36770_e51372_d_n0, assign36770_e51372_d_n2, assign36770_e51372_d_n6, assign36770_e51372_d_n7, assign36770_e51372_d_n10, assign36770_e51372_d_n11, assign36770_e51372_d_n12, assign36770_e51372_d_n17,) = {
    if (locals.var_guard1210 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign36770_e51372;
        locals.var_isub_dn0 = assign36770_e51372_d_n0;
        locals.var_isub_dn2 = assign36770_e51372_d_n2;
        locals.var_isub_dn6 = assign36770_e51372_d_n6;
        locals.var_isub_dn7 = assign36770_e51372_d_n7;
        locals.var_isub_dn10 = assign36770_e51372_d_n10;
        locals.var_isub_dn11 = assign36770_e51372_d_n11;
        locals.var_isub_dn12 = assign36770_e51372_d_n12;
        locals.var_isub_dn17 = assign36770_e51372_d_n17;
        locals.var_isub_rv = 0.0;

        let (assign36780_e51379, assign36780_e51379_d_n0, assign36780_e51379_d_n2, assign36780_e51379_d_n6, assign36780_e51379_d_n7, assign36780_e51379_d_n10, assign36780_e51379_d_n11, assign36780_e51379_d_n12, assign36780_e51379_d_n13, assign36780_e51379_d_n15, assign36780_e51379_d_n16, assign36780_e51379_d_n17, assign36780_e51379_d_n18,) = {
    if (locals.var_guard1210 == 0.0) {
        let assign36780_e51377: f64 = (locals.var_qge + locals.var_qg_nqs);
        (assign36780_e51377, (locals.var_qge_dn0 + locals.var_qg_nqs_dn0), (locals.var_qge_dn2 + locals.var_qg_nqs_dn2), (locals.var_qge_dn6 + locals.var_qg_nqs_dn6), (locals.var_qge_dn7 + locals.var_qg_nqs_dn7), (locals.var_qge_dn10 + locals.var_qg_nqs_dn10), (locals.var_qge_dn11 + locals.var_qg_nqs_dn11), (locals.var_qge_dn12 + locals.var_qg_nqs_dn12), (locals.var_qge_dn13 + locals.var_qg_nqs_dn13), (locals.var_qge_dn15 + locals.var_qg_nqs_dn15), (locals.var_qge_dn16 + locals.var_qg_nqs_dn16), (locals.var_qge_dn17 + locals.var_qg_nqs_dn17), (locals.var_qge_dn18 + locals.var_qg_nqs_dn18),)
    } else {
        (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12, locals.var_qg_dn13, locals.var_qg_dn15, locals.var_qg_dn16, locals.var_qg_dn17, locals.var_qg_dn18,)
    }
};
        locals.var_qg = assign36780_e51379;
        locals.var_qg_dn0 = assign36780_e51379_d_n0;
        locals.var_qg_dn2 = assign36780_e51379_d_n2;
        locals.var_qg_dn6 = assign36780_e51379_d_n6;
        locals.var_qg_dn7 = assign36780_e51379_d_n7;
        locals.var_qg_dn10 = assign36780_e51379_d_n10;
        locals.var_qg_dn11 = assign36780_e51379_d_n11;
        locals.var_qg_dn12 = assign36780_e51379_d_n12;
        locals.var_qg_dn13 = assign36780_e51379_d_n13;
        locals.var_qg_dn15 = assign36780_e51379_d_n15;
        locals.var_qg_dn16 = assign36780_e51379_d_n16;
        locals.var_qg_dn17 = assign36780_e51379_d_n17;
        locals.var_qg_dn18 = assign36780_e51379_d_n18;
        locals.var_qg_rv = 0.0;

        let (assign36790_e51386, assign36790_e51386_d_n0, assign36790_e51386_d_n2, assign36790_e51386_d_n6, assign36790_e51386_d_n7, assign36790_e51386_d_n10, assign36790_e51386_d_n11, assign36790_e51386_d_n12, assign36790_e51386_d_n13, assign36790_e51386_d_n15, assign36790_e51386_d_n16, assign36790_e51386_d_n17, assign36790_e51386_d_n18,) = {
    if (locals.var_guard1210 == 0.0) {
        let assign36790_e51384: f64 = (locals.var_qse + locals.var_qs_nqs);
        (assign36790_e51384, (locals.var_qse_dn0 + locals.var_qs_nqs_dn0), (locals.var_qse_dn2 + locals.var_qs_nqs_dn2), (locals.var_qse_dn6 + locals.var_qs_nqs_dn6), (locals.var_qse_dn7 + locals.var_qs_nqs_dn7), (locals.var_qse_dn10 + locals.var_qs_nqs_dn10), (locals.var_qse_dn11 + locals.var_qs_nqs_dn11), (locals.var_qse_dn12 + locals.var_qs_nqs_dn12), locals.var_qse_dn13, locals.var_qse_dn15, (locals.var_qse_dn16 + locals.var_qs_nqs_dn16), (locals.var_qse_dn17 + locals.var_qs_nqs_dn17), (locals.var_qse_dn18 + locals.var_qs_nqs_dn18),)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18,)
    }
};
        locals.var_qd = assign36790_e51386;
        locals.var_qd_dn0 = assign36790_e51386_d_n0;
        locals.var_qd_dn2 = assign36790_e51386_d_n2;
        locals.var_qd_dn6 = assign36790_e51386_d_n6;
        locals.var_qd_dn7 = assign36790_e51386_d_n7;
        locals.var_qd_dn10 = assign36790_e51386_d_n10;
        locals.var_qd_dn11 = assign36790_e51386_d_n11;
        locals.var_qd_dn12 = assign36790_e51386_d_n12;
        locals.var_qd_dn13 = assign36790_e51386_d_n13;
        locals.var_qd_dn15 = assign36790_e51386_d_n15;
        locals.var_qd_dn16 = assign36790_e51386_d_n16;
        locals.var_qd_dn17 = assign36790_e51386_d_n17;
        locals.var_qd_dn18 = assign36790_e51386_d_n18;
        locals.var_qd_rv = 0.0;

        let (assign36810_e51403, assign36810_e51403_d_n0, assign36810_e51403_d_n2, assign36810_e51403_d_n6, assign36810_e51403_d_n7, assign36810_e51403_d_n10, assign36810_e51403_d_n11, assign36810_e51403_d_n12, assign36810_e51403_d_n13, assign36810_e51403_d_n15, assign36810_e51403_d_n16, assign36810_e51403_d_n17, assign36810_e51403_d_n18,) = {
    if (locals.var_guard1210 == 0.0) {
        let assign36810_e51398: f64 = (locals.var_qge + locals.var_qde);
        let assign36810_e51400: f64 = (assign36810_e51398 + locals.var_qse);
        let assign36810_e51401: f64 = (-assign36810_e51400);
        (assign36810_e51401, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)), (-((locals.var_qge_dn15 + locals.var_qde_dn15) + locals.var_qse_dn15)), (-((locals.var_qge_dn16 + locals.var_qde_dn16) + locals.var_qse_dn16)), (-((locals.var_qge_dn17 + locals.var_qde_dn17) + locals.var_qse_dn17)), (-((locals.var_qge_dn18 + locals.var_qde_dn18) + locals.var_qse_dn18)),)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    }
};
        locals.var_qbe = assign36810_e51403;
        locals.var_qbe_dn0 = assign36810_e51403_d_n0;
        locals.var_qbe_dn2 = assign36810_e51403_d_n2;
        locals.var_qbe_dn6 = assign36810_e51403_d_n6;
        locals.var_qbe_dn7 = assign36810_e51403_d_n7;
        locals.var_qbe_dn10 = assign36810_e51403_d_n10;
        locals.var_qbe_dn11 = assign36810_e51403_d_n11;
        locals.var_qbe_dn12 = assign36810_e51403_d_n12;
        locals.var_qbe_dn13 = assign36810_e51403_d_n13;
        locals.var_qbe_dn15 = assign36810_e51403_d_n15;
        locals.var_qbe_dn16 = assign36810_e51403_d_n16;
        locals.var_qbe_dn17 = assign36810_e51403_d_n17;
        locals.var_qbe_dn18 = assign36810_e51403_d_n18;
        locals.var_qbe_rv = 0.0;

        let (assign36820_e51410, assign36820_e51410_d_n0, assign36820_e51410_d_n2, assign36820_e51410_d_n6, assign36820_e51410_d_n7, assign36820_e51410_d_n10, assign36820_e51410_d_n11, assign36820_e51410_d_n12, assign36820_e51410_d_n13, assign36820_e51410_d_n15, assign36820_e51410_d_n16, assign36820_e51410_d_n17, assign36820_e51410_d_n18,) = {
    if (locals.var_guard1210 == 0.0) {
        let assign36820_e51408: f64 = (locals.var_qbe + locals.var_qb_nqs);
        (assign36820_e51408, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, (locals.var_qbe_dn13 + locals.var_qb_nqs_dn13), locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18,)
    }
};
        locals.var_qb = assign36820_e51410;
        locals.var_qb_dn0 = assign36820_e51410_d_n0;
        locals.var_qb_dn2 = assign36820_e51410_d_n2;
        locals.var_qb_dn6 = assign36820_e51410_d_n6;
        locals.var_qb_dn7 = assign36820_e51410_d_n7;
        locals.var_qb_dn10 = assign36820_e51410_d_n10;
        locals.var_qb_dn11 = assign36820_e51410_d_n11;
        locals.var_qb_dn12 = assign36820_e51410_d_n12;
        locals.var_qb_dn13 = assign36820_e51410_d_n13;
        locals.var_qb_dn15 = assign36820_e51410_d_n15;
        locals.var_qb_dn16 = assign36820_e51410_d_n16;
        locals.var_qb_dn17 = assign36820_e51410_d_n17;
        locals.var_qb_dn18 = assign36820_e51410_d_n18;
        locals.var_qb_rv = 0.0;

        let assign36880_e51418: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1211 = assign36880_e51418;
        locals.var_guard1211_rv = 0.0;

        let (assign36890_e51422, assign36890_e51422_d_n0, assign36890_e51422_d_n2, assign36890_e51422_d_n6, assign36890_e51422_d_n7, assign36890_e51422_d_n10, assign36890_e51422_d_n11, assign36890_e51422_d_n12, assign36890_e51422_d_n17,) = {
    if (locals.var_guard1211 != 0.0) {
        (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17,)
    } else {
        (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17,)
    }
};
        locals.var_ibd = assign36890_e51422;
        locals.var_ibd_dn0 = assign36890_e51422_d_n0;
        locals.var_ibd_dn2 = assign36890_e51422_d_n2;
        locals.var_ibd_dn6 = assign36890_e51422_d_n6;
        locals.var_ibd_dn7 = assign36890_e51422_d_n7;
        locals.var_ibd_dn10 = assign36890_e51422_d_n10;
        locals.var_ibd_dn11 = assign36890_e51422_d_n11;
        locals.var_ibd_dn12 = assign36890_e51422_d_n12;
        locals.var_ibd_dn17 = assign36890_e51422_d_n17;
        locals.var_ibd_rv = 0.0;

        let (assign36900_e51426, assign36900_e51426_d_n0, assign36900_e51426_d_n2, assign36900_e51426_d_n6, assign36900_e51426_d_n7, assign36900_e51426_d_n10, assign36900_e51426_d_n11, assign36900_e51426_d_n12, assign36900_e51426_d_n17,) = {
    if (locals.var_guard1211 != 0.0) {
        (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign36900_e51426;
        locals.var_qbd_dn0 = assign36900_e51426_d_n0;
        locals.var_qbd_dn2 = assign36900_e51426_d_n2;
        locals.var_qbd_dn6 = assign36900_e51426_d_n6;
        locals.var_qbd_dn7 = assign36900_e51426_d_n7;
        locals.var_qbd_dn10 = assign36900_e51426_d_n10;
        locals.var_qbd_dn11 = assign36900_e51426_d_n11;
        locals.var_qbd_dn12 = assign36900_e51426_d_n12;
        locals.var_qbd_dn17 = assign36900_e51426_d_n17;
        locals.var_qbd_rv = 0.0;

        let (assign36910_e51430, assign36910_e51430_d_n0, assign36910_e51430_d_n2, assign36910_e51430_d_n6, assign36910_e51430_d_n7, assign36910_e51430_d_n10, assign36910_e51430_d_n11, assign36910_e51430_d_n12, assign36910_e51430_d_n17,) = {
    if (locals.var_guard1211 != 0.0) {
        (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17,)
    } else {
        (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17,)
    }
};
        locals.var_ibs = assign36910_e51430;
        locals.var_ibs_dn0 = assign36910_e51430_d_n0;
        locals.var_ibs_dn2 = assign36910_e51430_d_n2;
        locals.var_ibs_dn6 = assign36910_e51430_d_n6;
        locals.var_ibs_dn7 = assign36910_e51430_d_n7;
        locals.var_ibs_dn10 = assign36910_e51430_d_n10;
        locals.var_ibs_dn11 = assign36910_e51430_d_n11;
        locals.var_ibs_dn12 = assign36910_e51430_d_n12;
        locals.var_ibs_dn17 = assign36910_e51430_d_n17;
        locals.var_ibs_rv = 0.0;

        let (assign36920_e51434, assign36920_e51434_d_n0, assign36920_e51434_d_n2, assign36920_e51434_d_n6, assign36920_e51434_d_n7, assign36920_e51434_d_n10, assign36920_e51434_d_n11, assign36920_e51434_d_n12, assign36920_e51434_d_n17,) = {
    if (locals.var_guard1211 != 0.0) {
        (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign36920_e51434;
        locals.var_qbs_dn0 = assign36920_e51434_d_n0;
        locals.var_qbs_dn2 = assign36920_e51434_d_n2;
        locals.var_qbs_dn6 = assign36920_e51434_d_n6;
        locals.var_qbs_dn7 = assign36920_e51434_d_n7;
        locals.var_qbs_dn10 = assign36920_e51434_d_n10;
        locals.var_qbs_dn11 = assign36920_e51434_d_n11;
        locals.var_qbs_dn12 = assign36920_e51434_d_n12;
        locals.var_qbs_dn17 = assign36920_e51434_d_n17;
        locals.var_qbs_rv = 0.0;

        let assign36930_e51441: f64 = if ((p.p38 == 1.0) && (locals.var_mks_rth0 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1212 = assign36930_e51441;
        locals.var_guard1212_rv = 0.0;

        let (assign36950_e51451,) = {
    if (locals.var_guard1212 != 0.0) {
        (locals.var_cth,)
    } else {
        (locals.var_cthe,)
    }
};
        locals.var_cthe = assign36950_e51451;
        locals.var_cthe_rv = 0.0;

        let (assign36980_e51467,) = {
    if (locals.var_guard1212 == 0.0) {
        (0.0,)
    } else {
        (locals.var_cthe,)
    }
};
        locals.var_cthe = assign36980_e51467;
        locals.var_cthe_rv = 0.0;

        locals.var_idse = locals.var_ids;
        locals.var_idse_dn0 = locals.var_ids_dn0;
        locals.var_idse_dn2 = locals.var_ids_dn2;
        locals.var_idse_dn6 = locals.var_ids_dn6;
        locals.var_idse_dn7 = locals.var_ids_dn7;
        locals.var_idse_dn10 = locals.var_ids_dn10;
        locals.var_idse_dn11 = locals.var_ids_dn11;
        locals.var_idse_dn12 = locals.var_ids_dn12;
        locals.var_idse_dn17 = locals.var_ids_dn17;
        locals.var_idse_rv = 0.0;

        let assign37150_e51521: f64 = locals.var_qg_dn6;
        locals.var_cgdbd = assign37150_e51521;
        locals.var_cgdbd_dn0 = 0.0;
        locals.var_cgdbd_dn2 = 0.0;
        locals.var_cgdbd_dn6 = 0.0;
        locals.var_cgdbd_dn7 = 0.0;
        locals.var_cgdbd_dn10 = 0.0;
        locals.var_cgdbd_dn11 = 0.0;
        locals.var_cgdbd_dn12 = 0.0;
        locals.var_cgdbd_dn13 = 0.0;
        locals.var_cgdbd_dn15 = 0.0;
        locals.var_cgdbd_dn16 = 0.0;
        locals.var_cgdbd_dn17 = 0.0;
        locals.var_cgdbd_dn18 = 0.0;
        locals.var_cgdbd_rv = 0.0;

        let assign37160_e51524: f64 = (p.p50 * locals.var_cgdbd);
        locals.var_cgdbd = assign37160_e51524;
        locals.var_cgdbd_dn0 = (p.p50 * locals.var_cgdbd_dn0);
        locals.var_cgdbd_dn2 = (p.p50 * locals.var_cgdbd_dn2);
        locals.var_cgdbd_dn6 = (p.p50 * locals.var_cgdbd_dn6);
        locals.var_cgdbd_dn7 = (p.p50 * locals.var_cgdbd_dn7);
        locals.var_cgdbd_dn10 = (p.p50 * locals.var_cgdbd_dn10);
        locals.var_cgdbd_dn11 = (p.p50 * locals.var_cgdbd_dn11);
        locals.var_cgdbd_dn12 = (p.p50 * locals.var_cgdbd_dn12);
        locals.var_cgdbd_dn13 = (p.p50 * locals.var_cgdbd_dn13);
        locals.var_cgdbd_dn15 = (p.p50 * locals.var_cgdbd_dn15);
        locals.var_cgdbd_dn16 = (p.p50 * locals.var_cgdbd_dn16);
        locals.var_cgdbd_dn17 = (p.p50 * locals.var_cgdbd_dn17);
        locals.var_cgdbd_dn18 = (p.p50 * locals.var_cgdbd_dn18);
        locals.var_cgdbd_rv = 0.0;

        let assign37170_e51527: f64 = locals.var_qg_dn7;
        locals.var_cgsbd = assign37170_e51527;
        locals.var_cgsbd_dn0 = 0.0;
        locals.var_cgsbd_dn2 = 0.0;
        locals.var_cgsbd_dn6 = 0.0;
        locals.var_cgsbd_dn7 = 0.0;
        locals.var_cgsbd_dn10 = 0.0;
        locals.var_cgsbd_dn11 = 0.0;
        locals.var_cgsbd_dn12 = 0.0;
        locals.var_cgsbd_dn13 = 0.0;
        locals.var_cgsbd_dn15 = 0.0;
        locals.var_cgsbd_dn16 = 0.0;
        locals.var_cgsbd_dn17 = 0.0;
        locals.var_cgsbd_dn18 = 0.0;
        locals.var_cgsbd_rv = 0.0;

        let assign37180_e51530: f64 = (p.p50 * locals.var_cgsbd);
        locals.var_cgsbd = assign37180_e51530;
        locals.var_cgsbd_dn0 = (p.p50 * locals.var_cgsbd_dn0);
        locals.var_cgsbd_dn2 = (p.p50 * locals.var_cgsbd_dn2);
        locals.var_cgsbd_dn6 = (p.p50 * locals.var_cgsbd_dn6);
        locals.var_cgsbd_dn7 = (p.p50 * locals.var_cgsbd_dn7);
        locals.var_cgsbd_dn10 = (p.p50 * locals.var_cgsbd_dn10);
        locals.var_cgsbd_dn11 = (p.p50 * locals.var_cgsbd_dn11);
        locals.var_cgsbd_dn12 = (p.p50 * locals.var_cgsbd_dn12);
        locals.var_cgsbd_dn13 = (p.p50 * locals.var_cgsbd_dn13);
        locals.var_cgsbd_dn15 = (p.p50 * locals.var_cgsbd_dn15);
        locals.var_cgsbd_dn16 = (p.p50 * locals.var_cgsbd_dn16);
        locals.var_cgsbd_dn17 = (p.p50 * locals.var_cgsbd_dn17);
        locals.var_cgsbd_dn18 = (p.p50 * locals.var_cgsbd_dn18);
        locals.var_cgsbd_rv = 0.0;

        let assign37450_e51611: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1214 = assign37450_e51611;
        locals.var_guard1214_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_130(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign37460_e51617, assign37460_e51617_d_n0, assign37460_e51617_d_n2, assign37460_e51617_d_n6, assign37460_e51617_d_n7, assign37460_e51617_d_n10, assign37460_e51617_d_n11, assign37460_e51617_d_n12, assign37460_e51617_d_n17,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign37460_e51615: f64 = (p.p50 * locals.var_ibd);
        (assign37460_e51615, (p.p50 * locals.var_ibd_dn0), (p.p50 * locals.var_ibd_dn2), (p.p50 * locals.var_ibd_dn6), (p.p50 * locals.var_ibd_dn7), (p.p50 * locals.var_ibd_dn10), (p.p50 * locals.var_ibd_dn11), (p.p50 * locals.var_ibd_dn12), (p.p50 * locals.var_ibd_dn17),)
    } else {
        (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17,)
    }
};
        locals.var_ibdb = assign37460_e51617;
        locals.var_ibdb_dn0 = assign37460_e51617_d_n0;
        locals.var_ibdb_dn2 = assign37460_e51617_d_n2;
        locals.var_ibdb_dn6 = assign37460_e51617_d_n6;
        locals.var_ibdb_dn7 = assign37460_e51617_d_n7;
        locals.var_ibdb_dn10 = assign37460_e51617_d_n10;
        locals.var_ibdb_dn11 = assign37460_e51617_d_n11;
        locals.var_ibdb_dn12 = assign37460_e51617_d_n12;
        locals.var_ibdb_dn17 = assign37460_e51617_d_n17;
        locals.var_ibdb_rv = 0.0;

        let (assign37470_e51623, assign37470_e51623_d_n0, assign37470_e51623_d_n2, assign37470_e51623_d_n6, assign37470_e51623_d_n7, assign37470_e51623_d_n10, assign37470_e51623_d_n11, assign37470_e51623_d_n12, assign37470_e51623_d_n17,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign37470_e51621: f64 = (p.p50 * locals.var_ibs);
        (assign37470_e51621, (p.p50 * locals.var_ibs_dn0), (p.p50 * locals.var_ibs_dn2), (p.p50 * locals.var_ibs_dn6), (p.p50 * locals.var_ibs_dn7), (p.p50 * locals.var_ibs_dn10), (p.p50 * locals.var_ibs_dn11), (p.p50 * locals.var_ibs_dn12), (p.p50 * locals.var_ibs_dn17),)
    } else {
        (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17,)
    }
};
        locals.var_ibsb = assign37470_e51623;
        locals.var_ibsb_dn0 = assign37470_e51623_d_n0;
        locals.var_ibsb_dn2 = assign37470_e51623_d_n2;
        locals.var_ibsb_dn6 = assign37470_e51623_d_n6;
        locals.var_ibsb_dn7 = assign37470_e51623_d_n7;
        locals.var_ibsb_dn10 = assign37470_e51623_d_n10;
        locals.var_ibsb_dn11 = assign37470_e51623_d_n11;
        locals.var_ibsb_dn12 = assign37470_e51623_d_n12;
        locals.var_ibsb_dn17 = assign37470_e51623_d_n17;
        locals.var_ibsb_rv = 0.0;

        let assign37590_e51675: f64 = (4.0 * 1.3806226e-23);
        let assign37590_e51677: f64 = (assign37590_e51675 * locals.var_ttemp);
        let assign37590_e51679: f64 = assign37590_e51677;
        locals.var_whi_noise = assign37590_e51679;
        locals.var_whi_noise_dn10 = (assign37590_e51675 * locals.var_ttemp_dn10);
        locals.var_whi_noise_rv = 0.0;

        locals.var_qdrat = locals.var_qdrat_noi;
        locals.var_qdrat_dn0 = locals.var_qdrat_noi_dn0;
        locals.var_qdrat_dn2 = locals.var_qdrat_noi_dn2;
        locals.var_qdrat_dn6 = locals.var_qdrat_noi_dn6;
        locals.var_qdrat_dn7 = locals.var_qdrat_noi_dn7;
        locals.var_qdrat_dn10 = locals.var_qdrat_noi_dn10;
        locals.var_qdrat_dn11 = locals.var_qdrat_noi_dn11;
        locals.var_qdrat_dn12 = locals.var_qdrat_noi_dn12;
        locals.var_qdrat_dn17 = locals.var_qdrat_noi_dn17;
        locals.var_qdrat_rv = 0.0;

        let assign37620_e51686: f64 = (locals.var_whi_noise * locals.var_noithrml);
        locals.var_sid = assign37620_e51686;
        locals.var_sid_dn0 = (locals.var_whi_noise * locals.var_noithrml_dn0);
        locals.var_sid_dn2 = (locals.var_whi_noise * locals.var_noithrml_dn2);
        locals.var_sid_dn6 = (locals.var_whi_noise * locals.var_noithrml_dn6);
        locals.var_sid_dn7 = (locals.var_whi_noise * locals.var_noithrml_dn7);
        locals.var_sid_dn10 = ((locals.var_whi_noise_dn10 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn10));
        locals.var_sid_dn11 = (locals.var_whi_noise * locals.var_noithrml_dn11);
        locals.var_sid_dn12 = (locals.var_whi_noise * locals.var_noithrml_dn12);
        locals.var_sid_dn17 = (locals.var_whi_noise * locals.var_noithrml_dn17);
        locals.var_sid_rv = 0.0;

        let (assign37640_e51700, assign37640_e51700_d_n0, assign37640_e51700_d_n2, assign37640_e51700_d_n6, assign37640_e51700_d_n7, assign37640_e51700_d_n10, assign37640_e51700_d_n11, assign37640_e51700_d_n12, assign37640_e51700_d_n13, assign37640_e51700_d_n15, assign37640_e51700_d_n16, assign37640_e51700_d_n17, assign37640_e51700_d_n18,) = {
    if ((locals.var_sid > 0.0) && (locals.var_noiigate > 0.0)) {
        let assign37640_e51697: f64 = (locals.var_noiigate / locals.var_sid);
        let assign37640_e51698: f64 = (assign37640_e51697).sqrt();
        (assign37640_e51698, ((((locals.var_noiigate_dn0 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn0)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((((locals.var_noiigate_dn2 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn2)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((((locals.var_noiigate_dn6 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn6)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((((locals.var_noiigate_dn7 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn7)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((((locals.var_noiigate_dn10 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn10)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((((locals.var_noiigate_dn11 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn11)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((((locals.var_noiigate_dn12 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn12)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((locals.var_noiigate_dn13 / locals.var_sid) / (2.0 * assign37640_e51698)), ((locals.var_noiigate_dn15 / locals.var_sid) / (2.0 * assign37640_e51698)), ((locals.var_noiigate_dn16 / locals.var_sid) / (2.0 * assign37640_e51698)), ((((locals.var_noiigate_dn17 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn17)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37640_e51698)), ((locals.var_noiigate_dn18 / locals.var_sid) / (2.0 * assign37640_e51698)),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        locals.var_sigrat = assign37640_e51700;
        locals.var_sigrat_dn0 = assign37640_e51700_d_n0;
        locals.var_sigrat_dn2 = assign37640_e51700_d_n2;
        locals.var_sigrat_dn6 = assign37640_e51700_d_n6;
        locals.var_sigrat_dn7 = assign37640_e51700_d_n7;
        locals.var_sigrat_dn10 = assign37640_e51700_d_n10;
        locals.var_sigrat_dn11 = assign37640_e51700_d_n11;
        locals.var_sigrat_dn12 = assign37640_e51700_d_n12;
        locals.var_sigrat_dn13 = assign37640_e51700_d_n13;
        locals.var_sigrat_dn15 = assign37640_e51700_d_n15;
        locals.var_sigrat_dn16 = assign37640_e51700_d_n16;
        locals.var_sigrat_dn17 = assign37640_e51700_d_n17;
        locals.var_sigrat_dn18 = assign37640_e51700_d_n18;
        locals.var_sigrat_rv = 0.0;

        let (assign37650_e51712, assign37650_e51712_d_n0, assign37650_e51712_d_n2, assign37650_e51712_d_n6, assign37650_e51712_d_n7, assign37650_e51712_d_n10, assign37650_e51712_d_n11, assign37650_e51712_d_n12, assign37650_e51712_d_n13, assign37650_e51712_d_n15, assign37650_e51712_d_n16, assign37650_e51712_d_n17, assign37650_e51712_d_n18,) = {
    if (locals.var_mode > 0.0) {
        let assign37650_e51707: f64 = (1.0 - locals.var_qdrat);
        let assign37650_e51708: f64 = (locals.var_sigrat * assign37650_e51707);
        (assign37650_e51708, ((locals.var_sigrat_dn0 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn6 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn10 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn11 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn11))), ((locals.var_sigrat_dn12 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn12))), (locals.var_sigrat_dn13 * assign37650_e51707), (locals.var_sigrat_dn15 * assign37650_e51707), (locals.var_sigrat_dn16 * assign37650_e51707), ((locals.var_sigrat_dn17 * assign37650_e51707) + (locals.var_sigrat * (-locals.var_qdrat_dn17))), (locals.var_sigrat_dn18 * assign37650_e51707),)
    } else {
        let assign37650_e51711: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign37650_e51711, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn11 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn11)), ((locals.var_sigrat_dn12 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn12)), (locals.var_sigrat_dn13 * locals.var_qdrat), (locals.var_sigrat_dn15 * locals.var_qdrat), (locals.var_sigrat_dn16 * locals.var_qdrat), ((locals.var_sigrat_dn17 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn17)), (locals.var_sigrat_dn18 * locals.var_qdrat),)
    }
};
        locals.var_sigrat_s = assign37650_e51712;
        locals.var_sigrat_s_dn0 = assign37650_e51712_d_n0;
        locals.var_sigrat_s_dn2 = assign37650_e51712_d_n2;
        locals.var_sigrat_s_dn6 = assign37650_e51712_d_n6;
        locals.var_sigrat_s_dn7 = assign37650_e51712_d_n7;
        locals.var_sigrat_s_dn10 = assign37650_e51712_d_n10;
        locals.var_sigrat_s_dn11 = assign37650_e51712_d_n11;
        locals.var_sigrat_s_dn12 = assign37650_e51712_d_n12;
        locals.var_sigrat_s_dn13 = assign37650_e51712_d_n13;
        locals.var_sigrat_s_dn15 = assign37650_e51712_d_n15;
        locals.var_sigrat_s_dn16 = assign37650_e51712_d_n16;
        locals.var_sigrat_s_dn17 = assign37650_e51712_d_n17;
        locals.var_sigrat_s_dn18 = assign37650_e51712_d_n18;
        locals.var_sigrat_s_rv = 0.0;

        let (assign37660_e51724, assign37660_e51724_d_n0, assign37660_e51724_d_n2, assign37660_e51724_d_n6, assign37660_e51724_d_n7, assign37660_e51724_d_n10, assign37660_e51724_d_n11, assign37660_e51724_d_n12, assign37660_e51724_d_n13, assign37660_e51724_d_n15, assign37660_e51724_d_n16, assign37660_e51724_d_n17, assign37660_e51724_d_n18,) = {
    if (locals.var_mode > 0.0) {
        let assign37660_e51718: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign37660_e51718, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn11 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn11)), ((locals.var_sigrat_dn12 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn12)), (locals.var_sigrat_dn13 * locals.var_qdrat), (locals.var_sigrat_dn15 * locals.var_qdrat), (locals.var_sigrat_dn16 * locals.var_qdrat), ((locals.var_sigrat_dn17 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn17)), (locals.var_sigrat_dn18 * locals.var_qdrat),)
    } else {
        let assign37660_e51722: f64 = (1.0 - locals.var_qdrat);
        let assign37660_e51723: f64 = (locals.var_sigrat * assign37660_e51722);
        (assign37660_e51723, ((locals.var_sigrat_dn0 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn6 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn10 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn11 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn11))), ((locals.var_sigrat_dn12 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn12))), (locals.var_sigrat_dn13 * assign37660_e51722), (locals.var_sigrat_dn15 * assign37660_e51722), (locals.var_sigrat_dn16 * assign37660_e51722), ((locals.var_sigrat_dn17 * assign37660_e51722) + (locals.var_sigrat * (-locals.var_qdrat_dn17))), (locals.var_sigrat_dn18 * assign37660_e51722),)
    }
};
        locals.var_sigrat_d = assign37660_e51724;
        locals.var_sigrat_d_dn0 = assign37660_e51724_d_n0;
        locals.var_sigrat_d_dn2 = assign37660_e51724_d_n2;
        locals.var_sigrat_d_dn6 = assign37660_e51724_d_n6;
        locals.var_sigrat_d_dn7 = assign37660_e51724_d_n7;
        locals.var_sigrat_d_dn10 = assign37660_e51724_d_n10;
        locals.var_sigrat_d_dn11 = assign37660_e51724_d_n11;
        locals.var_sigrat_d_dn12 = assign37660_e51724_d_n12;
        locals.var_sigrat_d_dn13 = assign37660_e51724_d_n13;
        locals.var_sigrat_d_dn15 = assign37660_e51724_d_n15;
        locals.var_sigrat_d_dn16 = assign37660_e51724_d_n16;
        locals.var_sigrat_d_dn17 = assign37660_e51724_d_n17;
        locals.var_sigrat_d_dn18 = assign37660_e51724_d_n18;
        locals.var_sigrat_d_rv = 0.0;

        let assign37680_e51734: f64 = if ((p.p38 > 0.0) && (p.p242 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1222 = assign37680_e51734;
        locals.var_guard1222_rv = 0.0;

        let assign37700_e51741: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1223 = assign37700_e51741;
        locals.var_guard1223_rv = 0.0;

        let assign37710_e51750: f64 = if ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0))) { 1.0 } else { 0.0 };
        locals.var_guard1224 = assign37710_e51750;
        locals.var_guard1224_rv = 0.0;

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
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv4 = ctx.node_voltage(nodes[4]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let (eq3_e319, eq3_e319_d_n0, eq3_e319_d_n2, eq3_e319_d_n6, eq3_e319_d_n7, eq3_e319_d_n10, eq3_e319_d_n11, eq3_e319_d_n12, eq3_e319_d_n17,) = {
    if (locals.var_guard1220 != 0.0) {
        let eq3_e317: f64 = (p.p50 * locals.var_igs);
        let eq3_e317_d_n0: f64 = (p.p50 * locals.var_igs_dn0);
        let eq3_e317_d_n2: f64 = (p.p50 * locals.var_igs_dn2);
        let eq3_e317_d_n6: f64 = (p.p50 * locals.var_igs_dn6);
        let eq3_e317_d_n7: f64 = (p.p50 * locals.var_igs_dn7);
        let eq3_e317_d_n10: f64 = (p.p50 * locals.var_igs_dn10);
        let eq3_e317_d_n11: f64 = (p.p50 * locals.var_igs_dn11);
        let eq3_e317_d_n12: f64 = (p.p50 * locals.var_igs_dn12);
        let eq3_e317_d_n17: f64 = (p.p50 * locals.var_igs_dn17);
        (eq3_e317, eq3_e317_d_n0, eq3_e317_d_n2, eq3_e317_d_n6, eq3_e317_d_n7, eq3_e317_d_n10, eq3_e317_d_n11, eq3_e317_d_n12, eq3_e317_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e319;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(7),
            multiplicity * (eq3_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq3_e319_d_n0), multiplicity * (eq3_e319_d_n2), multiplicity * (eq3_e319_d_n6), multiplicity * (eq3_e319_d_n7), multiplicity * (eq3_e319_d_n10), multiplicity * (eq3_e319_d_n11), multiplicity * (eq3_e319_d_n12), multiplicity * (eq3_e319_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq4_e325, eq4_e325_d_n0, eq4_e325_d_n2, eq4_e325_d_n6, eq4_e325_d_n7, eq4_e325_d_n10, eq4_e325_d_n11, eq4_e325_d_n12, eq4_e325_d_n17,) = {
    if (locals.var_guard1220 != 0.0) {
        let eq4_e323: f64 = (p.p50 * locals.var_igd);
        let eq4_e323_d_n0: f64 = (p.p50 * locals.var_igd_dn0);
        let eq4_e323_d_n2: f64 = (p.p50 * locals.var_igd_dn2);
        let eq4_e323_d_n6: f64 = (p.p50 * locals.var_igd_dn6);
        let eq4_e323_d_n7: f64 = (p.p50 * locals.var_igd_dn7);
        let eq4_e323_d_n10: f64 = (p.p50 * locals.var_igd_dn10);
        let eq4_e323_d_n11: f64 = (p.p50 * locals.var_igd_dn11);
        let eq4_e323_d_n12: f64 = (p.p50 * locals.var_igd_dn12);
        let eq4_e323_d_n17: f64 = (p.p50 * locals.var_igd_dn17);
        (eq4_e323, eq4_e323_d_n0, eq4_e323_d_n2, eq4_e323_d_n6, eq4_e323_d_n7, eq4_e323_d_n10, eq4_e323_d_n11, eq4_e323_d_n12, eq4_e323_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e325;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(6),
            multiplicity * (eq4_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq4_e325_d_n0), multiplicity * (eq4_e325_d_n2), multiplicity * (eq4_e325_d_n6), multiplicity * (eq4_e325_d_n7), multiplicity * (eq4_e325_d_n10), multiplicity * (eq4_e325_d_n11), multiplicity * (eq4_e325_d_n12), multiplicity * (eq4_e325_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq5_e331, eq5_e331_d_n0, eq5_e331_d_n2, eq5_e331_d_n6, eq5_e331_d_n7, eq5_e331_d_n10, eq5_e331_d_n11, eq5_e331_d_n12, eq5_e331_d_n17,) = {
    if (locals.var_guard1220 != 0.0) {
        let eq5_e329: f64 = (p.p50 * locals.var_igb);
        let eq5_e329_d_n0: f64 = (p.p50 * locals.var_igb_dn0);
        let eq5_e329_d_n2: f64 = (p.p50 * locals.var_igb_dn2);
        let eq5_e329_d_n6: f64 = (p.p50 * locals.var_igb_dn6);
        let eq5_e329_d_n7: f64 = (p.p50 * locals.var_igb_dn7);
        let eq5_e329_d_n10: f64 = (p.p50 * locals.var_igb_dn10);
        let eq5_e329_d_n11: f64 = (p.p50 * locals.var_igb_dn11);
        let eq5_e329_d_n12: f64 = (p.p50 * locals.var_igb_dn12);
        let eq5_e329_d_n17: f64 = (p.p50 * locals.var_igb_dn17);
        (eq5_e329, eq5_e329_d_n0, eq5_e329_d_n2, eq5_e329_d_n6, eq5_e329_d_n7, eq5_e329_d_n10, eq5_e329_d_n11, eq5_e329_d_n12, eq5_e329_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e331;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq5_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq5_e331_d_n0), multiplicity * (eq5_e331_d_n2), multiplicity * (eq5_e331_d_n6), multiplicity * (eq5_e331_d_n7), multiplicity * (eq5_e331_d_n10), multiplicity * (eq5_e331_d_n11), multiplicity * (eq5_e331_d_n12), multiplicity * (eq5_e331_d_n17)],
            [],
            [],
            1.0,
        );
        let eq10_e356: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, locals.var_qg);
        let eq10_e357: f64 = (p.p50 * eq10_e356);
        let eq10_e357_d_n0: f64 = (p.p50 * (locals.var_qg_dn0 * ddt_scale));
        let eq10_e357_d_n2: f64 = (p.p50 * (locals.var_qg_dn2 * ddt_scale));
        let eq10_e357_d_n6: f64 = (p.p50 * (locals.var_qg_dn6 * ddt_scale));
        let eq10_e357_d_n7: f64 = (p.p50 * (locals.var_qg_dn7 * ddt_scale));
        let eq10_e357_d_n10: f64 = (p.p50 * (locals.var_qg_dn10 * ddt_scale));
        let eq10_e357_d_n11: f64 = (p.p50 * (locals.var_qg_dn11 * ddt_scale));
        let eq10_e357_d_n12: f64 = (p.p50 * (locals.var_qg_dn12 * ddt_scale));
        let eq10_e357_d_n13: f64 = (p.p50 * (locals.var_qg_dn13 * ddt_scale));
        let eq10_e357_d_n15: f64 = (p.p50 * (locals.var_qg_dn15 * ddt_scale));
        let eq10_e357_d_n16: f64 = (p.p50 * (locals.var_qg_dn16 * ddt_scale));
        let eq10_e357_d_n17: f64 = (p.p50 * (locals.var_qg_dn17 * ddt_scale));
        let eq10_e357_d_n18: f64 = (p.p50 * (locals.var_qg_dn18 * ddt_scale));
        let eq10_value: f64 = eq10_e357;
        let eq10_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq10_node_derivatives: [f64; 12] = [eq10_e357_d_n0, eq10_e357_d_n2, eq10_e357_d_n6, eq10_e357_d_n7, eq10_e357_d_n10, eq10_e357_d_n11, eq10_e357_d_n12, eq10_e357_d_n13, eq10_e357_d_n15, eq10_e357_d_n16, eq10_e357_d_n17, eq10_e357_d_n18];
        let eq10_branch_derivative_indices: [usize; 0] = [];
        let eq10_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivative_indices,
            &eq10_node_derivatives,
            &eq10_branch_derivative_indices,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e360: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, locals.var_qd);
        let eq11_e361: f64 = (p.p50 * eq11_e360);
        let eq11_e361_d_n0: f64 = (p.p50 * (locals.var_qd_dn0 * ddt_scale));
        let eq11_e361_d_n2: f64 = (p.p50 * (locals.var_qd_dn2 * ddt_scale));
        let eq11_e361_d_n6: f64 = (p.p50 * (locals.var_qd_dn6 * ddt_scale));
        let eq11_e361_d_n7: f64 = (p.p50 * (locals.var_qd_dn7 * ddt_scale));
        let eq11_e361_d_n10: f64 = (p.p50 * (locals.var_qd_dn10 * ddt_scale));
        let eq11_e361_d_n11: f64 = (p.p50 * (locals.var_qd_dn11 * ddt_scale));
        let eq11_e361_d_n12: f64 = (p.p50 * (locals.var_qd_dn12 * ddt_scale));
        let eq11_e361_d_n13: f64 = (p.p50 * (locals.var_qd_dn13 * ddt_scale));
        let eq11_e361_d_n15: f64 = (p.p50 * (locals.var_qd_dn15 * ddt_scale));
        let eq11_e361_d_n16: f64 = (p.p50 * (locals.var_qd_dn16 * ddt_scale));
        let eq11_e361_d_n17: f64 = (p.p50 * (locals.var_qd_dn17 * ddt_scale));
        let eq11_e361_d_n18: f64 = (p.p50 * (locals.var_qd_dn18 * ddt_scale));
        let eq11_value: f64 = eq11_e361;
        let eq11_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq11_node_derivatives: [f64; 12] = [eq11_e361_d_n0, eq11_e361_d_n2, eq11_e361_d_n6, eq11_e361_d_n7, eq11_e361_d_n10, eq11_e361_d_n11, eq11_e361_d_n12, eq11_e361_d_n13, eq11_e361_d_n15, eq11_e361_d_n16, eq11_e361_d_n17, eq11_e361_d_n18];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e364: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, locals.var_qb);
        let eq12_e365: f64 = (p.p50 * eq12_e364);
        let eq12_e365_d_n0: f64 = (p.p50 * (locals.var_qb_dn0 * ddt_scale));
        let eq12_e365_d_n2: f64 = (p.p50 * (locals.var_qb_dn2 * ddt_scale));
        let eq12_e365_d_n6: f64 = (p.p50 * (locals.var_qb_dn6 * ddt_scale));
        let eq12_e365_d_n7: f64 = (p.p50 * (locals.var_qb_dn7 * ddt_scale));
        let eq12_e365_d_n10: f64 = (p.p50 * (locals.var_qb_dn10 * ddt_scale));
        let eq12_e365_d_n11: f64 = (p.p50 * (locals.var_qb_dn11 * ddt_scale));
        let eq12_e365_d_n12: f64 = (p.p50 * (locals.var_qb_dn12 * ddt_scale));
        let eq12_e365_d_n13: f64 = (p.p50 * (locals.var_qb_dn13 * ddt_scale));
        let eq12_e365_d_n15: f64 = (p.p50 * (locals.var_qb_dn15 * ddt_scale));
        let eq12_e365_d_n16: f64 = (p.p50 * (locals.var_qb_dn16 * ddt_scale));
        let eq12_e365_d_n17: f64 = (p.p50 * (locals.var_qb_dn17 * ddt_scale));
        let eq12_e365_d_n18: f64 = (p.p50 * (locals.var_qb_dn18 * ddt_scale));
        let eq12_value: f64 = eq12_e365;
        let eq12_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq12_node_derivatives: [f64; 12] = [eq12_e365_d_n0, eq12_e365_d_n2, eq12_e365_d_n6, eq12_e365_d_n7, eq12_e365_d_n10, eq12_e365_d_n11, eq12_e365_d_n12, eq12_e365_d_n13, eq12_e365_d_n15, eq12_e365_d_n16, eq12_e365_d_n17, eq12_e365_d_n18];
        let eq12_branch_derivative_indices: [usize; 0] = [];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq12_value),
            &eq12_node_derivative_indices,
            &eq12_node_derivatives,
            &eq12_branch_derivative_indices,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq17_e391: f64 = (locals.var_ci * (nv14 - 0.0));
        let eq17_e391_d_n0: f64 = (locals.var_ci_dn0 * (nv14 - 0.0));
        let eq17_e391_d_n2: f64 = (locals.var_ci_dn2 * (nv14 - 0.0));
        let eq17_e391_d_n6: f64 = (locals.var_ci_dn6 * (nv14 - 0.0));
        let eq17_e391_d_n7: f64 = (locals.var_ci_dn7 * (nv14 - 0.0));
        let eq17_e391_d_n10: f64 = (locals.var_ci_dn10 * (nv14 - 0.0));
        let eq17_e391_d_n11: f64 = (locals.var_ci_dn11 * (nv14 - 0.0));
        let eq17_e391_d_n12: f64 = (locals.var_ci_dn12 * (nv14 - 0.0));
        let eq17_e391_d_n17: f64 = (locals.var_ci_dn17 * (nv14 - 0.0));
        let eq17_value: f64 = eq17_e391;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq17_value),
            [0, 2, 6, 7, 10, 11, 12, 14, 17],
            [multiplicity * (eq17_e391_d_n0), multiplicity * (eq17_e391_d_n2), multiplicity * (eq17_e391_d_n6), multiplicity * (eq17_e391_d_n7), multiplicity * (eq17_e391_d_n10), multiplicity * (eq17_e391_d_n11), multiplicity * (eq17_e391_d_n12), multiplicity * (locals.var_ci), multiplicity * (eq17_e391_d_n17)],
            [],
            [],
            1.0,
        );
        let eq18_e394: f64 = ((nv14 - 0.0) * locals.var_sigrat_s);
        let eq18_e394_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn0);
        let eq18_e394_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn2);
        let eq18_e394_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn6);
        let eq18_e394_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn7);
        let eq18_e394_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn10);
        let eq18_e394_d_n11: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn11);
        let eq18_e394_d_n12: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn12);
        let eq18_e394_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn13);
        let eq18_e394_d_n15: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn15);
        let eq18_e394_d_n16: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn16);
        let eq18_e394_d_n17: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn17);
        let eq18_e394_d_n18: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn18);
        let eq18_e395: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq18_e394);
        let eq18_value: f64 = eq18_e395;
        let eq18_node_derivative_indices: [usize; 13] = [0, 2, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18];
        let eq18_node_derivatives: [f64; 13] = [(eq18_e394_d_n0 * ddt_scale), (eq18_e394_d_n2 * ddt_scale), (eq18_e394_d_n6 * ddt_scale), (eq18_e394_d_n7 * ddt_scale), (eq18_e394_d_n10 * ddt_scale), (eq18_e394_d_n11 * ddt_scale), (eq18_e394_d_n12 * ddt_scale), (eq18_e394_d_n13 * ddt_scale), (locals.var_sigrat_s * ddt_scale), (eq18_e394_d_n15 * ddt_scale), (eq18_e394_d_n16 * ddt_scale), (eq18_e394_d_n17 * ddt_scale), (eq18_e394_d_n18 * ddt_scale)];
        let eq18_branch_derivative_indices: [usize; 0] = [];
        let eq18_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivative_indices,
            &eq18_node_derivatives,
            &eq18_branch_derivative_indices,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let eq19_e398: f64 = ((nv14 - 0.0) * locals.var_sigrat_d);
        let eq19_e398_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn0);
        let eq19_e398_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn2);
        let eq19_e398_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn6);
        let eq19_e398_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn7);
        let eq19_e398_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn10);
        let eq19_e398_d_n11: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn11);
        let eq19_e398_d_n12: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn12);
        let eq19_e398_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn13);
        let eq19_e398_d_n15: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn15);
        let eq19_e398_d_n16: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn16);
        let eq19_e398_d_n17: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn17);
        let eq19_e398_d_n18: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn18);
        let eq19_e399: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq19_e398);
        let eq19_value: f64 = eq19_e399;
        let eq19_node_derivative_indices: [usize; 13] = [0, 2, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18];
        let eq19_node_derivatives: [f64; 13] = [(eq19_e398_d_n0 * ddt_scale), (eq19_e398_d_n2 * ddt_scale), (eq19_e398_d_n6 * ddt_scale), (eq19_e398_d_n7 * ddt_scale), (eq19_e398_d_n10 * ddt_scale), (eq19_e398_d_n11 * ddt_scale), (eq19_e398_d_n12 * ddt_scale), (eq19_e398_d_n13 * ddt_scale), (locals.var_sigrat_d * ddt_scale), (eq19_e398_d_n15 * ddt_scale), (eq19_e398_d_n16 * ddt_scale), (eq19_e398_d_n17 * ddt_scale), (eq19_e398_d_n18 * ddt_scale)];
        let eq19_branch_derivative_indices: [usize; 0] = [];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq19_value),
            &eq19_node_derivative_indices,
            &eq19_node_derivatives,
            &eq19_branch_derivative_indices,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let (eq25_e451, eq25_e451_d_n1, eq25_e451_d_n11,) = {
    if (p.p35 != 0.0) {
        let eq25_e449: f64 = (locals.var_grg * (nv1 - nv11));
        (eq25_e449, locals.var_grg, (-locals.var_grg),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq25_value: f64 = eq25_e451;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(11),
            multiplicity * (eq25_value),
            1,
            multiplicity * (eq25_e451_d_n1),
            11,
            multiplicity * (eq25_e451_d_n11),
        );
        let (eq27_e462, eq27_e462_d_n10,) = {
    if (locals.var_guard1222 != 0.0) {
        let eq27_e460: f64 = ((nv10 - 0.0) * locals.var_gth);
        (eq27_e460, locals.var_gth,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e462;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq27_value),
            10,
            multiplicity * (eq27_e462_d_n10),
        );
        let (eq30_e480, eq30_e480_d_n10,) = {
    if (locals.var_guard1222 != 0.0) {
        let eq30_e477: f64 = (locals.var_cthe * (nv10 - 0.0));
        let eq30_e478: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq30_e477);
        (eq30_e478, (locals.var_cthe * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e480;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq30_value),
            10,
            multiplicity * (eq30_e480_d_n10),
        );
        let (eq32_e495, eq32_e495_d_n0, eq32_e495_d_n2, eq32_e495_d_n6, eq32_e495_d_n7, eq32_e495_d_n10, eq32_e495_d_n11, eq32_e495_d_n12, eq32_e495_d_n17,) = {
    if (locals.var_guard1223 != 0.0) {
        let eq32_e492: f64 = (locals.var_igidl + locals.var_isub);
        let eq32_e492_d_n0: f64 = (locals.var_igidl_dn0 + locals.var_isub_dn0);
        let eq32_e492_d_n2: f64 = (locals.var_igidl_dn2 + locals.var_isub_dn2);
        let eq32_e492_d_n6: f64 = (locals.var_igidl_dn6 + locals.var_isub_dn6);
        let eq32_e492_d_n7: f64 = (locals.var_igidl_dn7 + locals.var_isub_dn7);
        let eq32_e492_d_n10: f64 = (locals.var_igidl_dn10 + locals.var_isub_dn10);
        let eq32_e492_d_n11: f64 = (locals.var_igidl_dn11 + locals.var_isub_dn11);
        let eq32_e492_d_n12: f64 = (locals.var_igidl_dn12 + locals.var_isub_dn12);
        let eq32_e492_d_n17: f64 = (locals.var_igidl_dn17 + locals.var_isub_dn17);
        let eq32_e493: f64 = (p.p50 * eq32_e492);
        let eq32_e493_d_n0: f64 = (p.p50 * eq32_e492_d_n0);
        let eq32_e493_d_n2: f64 = (p.p50 * eq32_e492_d_n2);
        let eq32_e493_d_n6: f64 = (p.p50 * eq32_e492_d_n6);
        let eq32_e493_d_n7: f64 = (p.p50 * eq32_e492_d_n7);
        let eq32_e493_d_n10: f64 = (p.p50 * eq32_e492_d_n10);
        let eq32_e493_d_n11: f64 = (p.p50 * eq32_e492_d_n11);
        let eq32_e493_d_n12: f64 = (p.p50 * eq32_e492_d_n12);
        let eq32_e493_d_n17: f64 = (p.p50 * eq32_e492_d_n17);
        (eq32_e493, eq32_e493_d_n0, eq32_e493_d_n2, eq32_e493_d_n6, eq32_e493_d_n7, eq32_e493_d_n10, eq32_e493_d_n11, eq32_e493_d_n12, eq32_e493_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq32_value: f64 = eq32_e495;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(12),
            multiplicity * (eq32_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq32_e495_d_n0), multiplicity * (eq32_e495_d_n2), multiplicity * (eq32_e495_d_n6), multiplicity * (eq32_e495_d_n7), multiplicity * (eq32_e495_d_n10), multiplicity * (eq32_e495_d_n11), multiplicity * (eq32_e495_d_n12), multiplicity * (eq32_e495_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq33_e503, eq33_e503_d_n0, eq33_e503_d_n2, eq33_e503_d_n6, eq33_e503_d_n7, eq33_e503_d_n10, eq33_e503_d_n11, eq33_e503_d_n12, eq33_e503_d_n17,) = {
    if (locals.var_guard1223 != 0.0) {
        let eq33_e500: f64 = (locals.var_igisl + locals.var_isubs);
        let eq33_e500_d_n0: f64 = (locals.var_igisl_dn0 + locals.var_isubs_dn0);
        let eq33_e500_d_n2: f64 = (locals.var_igisl_dn2 + locals.var_isubs_dn2);
        let eq33_e500_d_n6: f64 = (locals.var_igisl_dn6 + locals.var_isubs_dn6);
        let eq33_e500_d_n7: f64 = (locals.var_igisl_dn7 + locals.var_isubs_dn7);
        let eq33_e500_d_n10: f64 = (locals.var_igisl_dn10 + locals.var_isubs_dn10);
        let eq33_e500_d_n11: f64 = (locals.var_igisl_dn11 + locals.var_isubs_dn11);
        let eq33_e500_d_n12: f64 = (locals.var_igisl_dn12 + locals.var_isubs_dn12);
        let eq33_e500_d_n17: f64 = (locals.var_igisl_dn17 + locals.var_isubs_dn17);
        let eq33_e501: f64 = (p.p50 * eq33_e500);
        let eq33_e501_d_n0: f64 = (p.p50 * eq33_e500_d_n0);
        let eq33_e501_d_n2: f64 = (p.p50 * eq33_e500_d_n2);
        let eq33_e501_d_n6: f64 = (p.p50 * eq33_e500_d_n6);
        let eq33_e501_d_n7: f64 = (p.p50 * eq33_e500_d_n7);
        let eq33_e501_d_n10: f64 = (p.p50 * eq33_e500_d_n10);
        let eq33_e501_d_n11: f64 = (p.p50 * eq33_e500_d_n11);
        let eq33_e501_d_n12: f64 = (p.p50 * eq33_e500_d_n12);
        let eq33_e501_d_n17: f64 = (p.p50 * eq33_e500_d_n17);
        (eq33_e501, eq33_e501_d_n0, eq33_e501_d_n2, eq33_e501_d_n6, eq33_e501_d_n7, eq33_e501_d_n10, eq33_e501_d_n11, eq33_e501_d_n12, eq33_e501_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e503;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(12),
            multiplicity * (eq33_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq33_e503_d_n0), multiplicity * (eq33_e503_d_n2), multiplicity * (eq33_e503_d_n6), multiplicity * (eq33_e503_d_n7), multiplicity * (eq33_e503_d_n10), multiplicity * (eq33_e503_d_n11), multiplicity * (eq33_e503_d_n12), multiplicity * (eq33_e503_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq34_e512, eq34_e512_d_n0, eq34_e512_d_n2, eq34_e512_d_n6, eq34_e512_d_n7, eq34_e512_d_n10, eq34_e512_d_n11, eq34_e512_d_n12, eq34_e512_d_n17,) = {
    if (locals.var_guard1223 != 0.0) {
        let eq34_e508: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, locals.var_qbs);
        let eq34_e509: f64 = (locals.var_ibs + eq34_e508);
        let eq34_e509_d_n0: f64 = (locals.var_ibs_dn0 + (locals.var_qbs_dn0 * ddt_scale));
        let eq34_e509_d_n2: f64 = (locals.var_ibs_dn2 + (locals.var_qbs_dn2 * ddt_scale));
        let eq34_e509_d_n6: f64 = (locals.var_ibs_dn6 + (locals.var_qbs_dn6 * ddt_scale));
        let eq34_e509_d_n7: f64 = (locals.var_ibs_dn7 + (locals.var_qbs_dn7 * ddt_scale));
        let eq34_e509_d_n10: f64 = (locals.var_ibs_dn10 + (locals.var_qbs_dn10 * ddt_scale));
        let eq34_e509_d_n11: f64 = (locals.var_ibs_dn11 + (locals.var_qbs_dn11 * ddt_scale));
        let eq34_e509_d_n12: f64 = (locals.var_ibs_dn12 + (locals.var_qbs_dn12 * ddt_scale));
        let eq34_e509_d_n17: f64 = (locals.var_ibs_dn17 + (locals.var_qbs_dn17 * ddt_scale));
        let eq34_e510: f64 = (p.p50 * eq34_e509);
        let eq34_e510_d_n0: f64 = (p.p50 * eq34_e509_d_n0);
        let eq34_e510_d_n2: f64 = (p.p50 * eq34_e509_d_n2);
        let eq34_e510_d_n6: f64 = (p.p50 * eq34_e509_d_n6);
        let eq34_e510_d_n7: f64 = (p.p50 * eq34_e509_d_n7);
        let eq34_e510_d_n10: f64 = (p.p50 * eq34_e509_d_n10);
        let eq34_e510_d_n11: f64 = (p.p50 * eq34_e509_d_n11);
        let eq34_e510_d_n12: f64 = (p.p50 * eq34_e509_d_n12);
        let eq34_e510_d_n17: f64 = (p.p50 * eq34_e509_d_n17);
        (eq34_e510, eq34_e510_d_n0, eq34_e510_d_n2, eq34_e510_d_n6, eq34_e510_d_n7, eq34_e510_d_n10, eq34_e510_d_n11, eq34_e510_d_n12, eq34_e510_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e512;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq34_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq34_e512_d_n0), multiplicity * (eq34_e512_d_n2), multiplicity * (eq34_e512_d_n6), multiplicity * (eq34_e512_d_n7), multiplicity * (eq34_e512_d_n10), multiplicity * (eq34_e512_d_n11), multiplicity * (eq34_e512_d_n12), multiplicity * (eq34_e512_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq35_e521, eq35_e521_d_n0, eq35_e521_d_n2, eq35_e521_d_n6, eq35_e521_d_n7, eq35_e521_d_n10, eq35_e521_d_n11, eq35_e521_d_n12, eq35_e521_d_n17,) = {
    if (locals.var_guard1223 != 0.0) {
        let eq35_e517: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, locals.var_qbd);
        let eq35_e518: f64 = (locals.var_ibd + eq35_e517);
        let eq35_e518_d_n0: f64 = (locals.var_ibd_dn0 + (locals.var_qbd_dn0 * ddt_scale));
        let eq35_e518_d_n2: f64 = (locals.var_ibd_dn2 + (locals.var_qbd_dn2 * ddt_scale));
        let eq35_e518_d_n6: f64 = (locals.var_ibd_dn6 + (locals.var_qbd_dn6 * ddt_scale));
        let eq35_e518_d_n7: f64 = (locals.var_ibd_dn7 + (locals.var_qbd_dn7 * ddt_scale));
        let eq35_e518_d_n10: f64 = (locals.var_ibd_dn10 + (locals.var_qbd_dn10 * ddt_scale));
        let eq35_e518_d_n11: f64 = (locals.var_ibd_dn11 + (locals.var_qbd_dn11 * ddt_scale));
        let eq35_e518_d_n12: f64 = (locals.var_ibd_dn12 + (locals.var_qbd_dn12 * ddt_scale));
        let eq35_e518_d_n17: f64 = (locals.var_ibd_dn17 + (locals.var_qbd_dn17 * ddt_scale));
        let eq35_e519: f64 = (p.p50 * eq35_e518);
        let eq35_e519_d_n0: f64 = (p.p50 * eq35_e518_d_n0);
        let eq35_e519_d_n2: f64 = (p.p50 * eq35_e518_d_n2);
        let eq35_e519_d_n6: f64 = (p.p50 * eq35_e518_d_n6);
        let eq35_e519_d_n7: f64 = (p.p50 * eq35_e518_d_n7);
        let eq35_e519_d_n10: f64 = (p.p50 * eq35_e518_d_n10);
        let eq35_e519_d_n11: f64 = (p.p50 * eq35_e518_d_n11);
        let eq35_e519_d_n12: f64 = (p.p50 * eq35_e518_d_n12);
        let eq35_e519_d_n17: f64 = (p.p50 * eq35_e518_d_n17);
        (eq35_e519, eq35_e519_d_n0, eq35_e519_d_n2, eq35_e519_d_n6, eq35_e519_d_n7, eq35_e519_d_n10, eq35_e519_d_n11, eq35_e519_d_n12, eq35_e519_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e521;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq35_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq35_e521_d_n0), multiplicity * (eq35_e521_d_n2), multiplicity * (eq35_e521_d_n6), multiplicity * (eq35_e521_d_n7), multiplicity * (eq35_e521_d_n10), multiplicity * (eq35_e521_d_n11), multiplicity * (eq35_e521_d_n12), multiplicity * (eq35_e521_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq36_e529, eq36_e529_d_n0, eq36_e529_d_n2, eq36_e529_d_n4, eq36_e529_d_n6, eq36_e529_d_n7, eq36_e529_d_n10, eq36_e529_d_n11, eq36_e529_d_n12, eq36_e529_d_n17,) = {
    if ((locals.var_guard1223 != 0.0) && (p.p261 != 0.0)) {
        let eq36_e527: f64 = ((nv4 - nv12) / locals.var_rbulk);
        let eq36_e527_d_n0: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn0) / (locals.var_rbulk * locals.var_rbulk)));
        let eq36_e527_d_n2: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn2) / (locals.var_rbulk * locals.var_rbulk)));
        let eq36_e527_d_n4: f64 = (1.0 / locals.var_rbulk);
        let eq36_e527_d_n6: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn6) / (locals.var_rbulk * locals.var_rbulk)));
        let eq36_e527_d_n7: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn7) / (locals.var_rbulk * locals.var_rbulk)));
        let eq36_e527_d_n10: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn10) / (locals.var_rbulk * locals.var_rbulk)));
        let eq36_e527_d_n11: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn11) / (locals.var_rbulk * locals.var_rbulk)));
        let eq36_e527_d_n12: f64 = (((-locals.var_rbulk) - ((nv4 - nv12) * locals.var_rbulk_dn12)) / (locals.var_rbulk * locals.var_rbulk));
        let eq36_e527_d_n17: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn17) / (locals.var_rbulk * locals.var_rbulk)));
        (eq36_e527, eq36_e527_d_n0, eq36_e527_d_n2, eq36_e527_d_n4, eq36_e527_d_n6, eq36_e527_d_n7, eq36_e527_d_n10, eq36_e527_d_n11, eq36_e527_d_n12, eq36_e527_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e529;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            Some(12),
            multiplicity * (eq36_value),
            [0, 2, 4, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq36_e529_d_n0), multiplicity * (eq36_e529_d_n2), multiplicity * (eq36_e529_d_n4), multiplicity * (eq36_e529_d_n6), multiplicity * (eq36_e529_d_n7), multiplicity * (eq36_e529_d_n10), multiplicity * (eq36_e529_d_n11), multiplicity * (eq36_e529_d_n12), multiplicity * (eq36_e529_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq46_e605, eq46_e605_d_n18,) = {
    if ((locals.var_guard1223 != 0.0) && (p.p34 != 0.0)) {
        let eq46_e600: f64 = (1e-9 / 0.0001);
        let eq46_e602: f64 = (eq46_e600 * (nv18 - 0.0));
        let eq46_e603: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq46_e602);
        (eq46_e603, (eq46_e600 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq46_value: f64 = eq46_e605;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (eq46_value),
            18,
            multiplicity * (eq46_e605_d_n18),
        );
        let (eq47_e616, eq47_e616_d_n13,) = {
    if ((locals.var_guard1223 != 0.0) && (p.p34 != 0.0)) {
        let eq47_e611: f64 = (1e-9 / 0.0001);
        let eq47_e613: f64 = (eq47_e611 * (nv13 - 0.0));
        let eq47_e614: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq47_e613);
        (eq47_e614, (eq47_e611 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e616;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq47_value),
            13,
            multiplicity * (eq47_e616_d_n13),
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
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let (eq52_e655, eq52_e655_d_n17,) = {
    if ((locals.var_guard1223 != 0.0) && (locals.var_guard1224 != 0.0)) {
        let eq52_e650: f64 = (1e-9 / 0.0001);
        let eq52_e652: f64 = (eq52_e650 * (nv17 - 0.0));
        let eq52_e653: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq52_e652);
        (eq52_e653, (eq52_e650 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq52_value: f64 = eq52_e655;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq52_value),
            17,
            multiplicity * (eq52_e655_d_n17),
        );
        let (eq54_e671, eq54_e671_d_n0, eq54_e671_d_n2, eq54_e671_d_n6, eq54_e671_d_n7, eq54_e671_d_n10, eq54_e671_d_n11, eq54_e671_d_n12, eq54_e671_d_n17,) = {
    if (locals.var_guard1223 == 0.0) {
        let eq54_e668: f64 = (locals.var_igidl + locals.var_isub);
        let eq54_e668_d_n0: f64 = (locals.var_igidl_dn0 + locals.var_isub_dn0);
        let eq54_e668_d_n2: f64 = (locals.var_igidl_dn2 + locals.var_isub_dn2);
        let eq54_e668_d_n6: f64 = (locals.var_igidl_dn6 + locals.var_isub_dn6);
        let eq54_e668_d_n7: f64 = (locals.var_igidl_dn7 + locals.var_isub_dn7);
        let eq54_e668_d_n10: f64 = (locals.var_igidl_dn10 + locals.var_isub_dn10);
        let eq54_e668_d_n11: f64 = (locals.var_igidl_dn11 + locals.var_isub_dn11);
        let eq54_e668_d_n12: f64 = (locals.var_igidl_dn12 + locals.var_isub_dn12);
        let eq54_e668_d_n17: f64 = (locals.var_igidl_dn17 + locals.var_isub_dn17);
        let eq54_e669: f64 = (p.p50 * eq54_e668);
        let eq54_e669_d_n0: f64 = (p.p50 * eq54_e668_d_n0);
        let eq54_e669_d_n2: f64 = (p.p50 * eq54_e668_d_n2);
        let eq54_e669_d_n6: f64 = (p.p50 * eq54_e668_d_n6);
        let eq54_e669_d_n7: f64 = (p.p50 * eq54_e668_d_n7);
        let eq54_e669_d_n10: f64 = (p.p50 * eq54_e668_d_n10);
        let eq54_e669_d_n11: f64 = (p.p50 * eq54_e668_d_n11);
        let eq54_e669_d_n12: f64 = (p.p50 * eq54_e668_d_n12);
        let eq54_e669_d_n17: f64 = (p.p50 * eq54_e668_d_n17);
        (eq54_e669, eq54_e669_d_n0, eq54_e669_d_n2, eq54_e669_d_n6, eq54_e669_d_n7, eq54_e669_d_n10, eq54_e669_d_n11, eq54_e669_d_n12, eq54_e669_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq54_value: f64 = eq54_e671;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq54_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq54_e671_d_n0), multiplicity * (eq54_e671_d_n2), multiplicity * (eq54_e671_d_n6), multiplicity * (eq54_e671_d_n7), multiplicity * (eq54_e671_d_n10), multiplicity * (eq54_e671_d_n11), multiplicity * (eq54_e671_d_n12), multiplicity * (eq54_e671_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq55_e680, eq55_e680_d_n0, eq55_e680_d_n2, eq55_e680_d_n6, eq55_e680_d_n7, eq55_e680_d_n10, eq55_e680_d_n11, eq55_e680_d_n12, eq55_e680_d_n17,) = {
    if (locals.var_guard1223 == 0.0) {
        let eq55_e677: f64 = (locals.var_igisl + locals.var_isubs);
        let eq55_e677_d_n0: f64 = (locals.var_igisl_dn0 + locals.var_isubs_dn0);
        let eq55_e677_d_n2: f64 = (locals.var_igisl_dn2 + locals.var_isubs_dn2);
        let eq55_e677_d_n6: f64 = (locals.var_igisl_dn6 + locals.var_isubs_dn6);
        let eq55_e677_d_n7: f64 = (locals.var_igisl_dn7 + locals.var_isubs_dn7);
        let eq55_e677_d_n10: f64 = (locals.var_igisl_dn10 + locals.var_isubs_dn10);
        let eq55_e677_d_n11: f64 = (locals.var_igisl_dn11 + locals.var_isubs_dn11);
        let eq55_e677_d_n12: f64 = (locals.var_igisl_dn12 + locals.var_isubs_dn12);
        let eq55_e677_d_n17: f64 = (locals.var_igisl_dn17 + locals.var_isubs_dn17);
        let eq55_e678: f64 = (p.p50 * eq55_e677);
        let eq55_e678_d_n0: f64 = (p.p50 * eq55_e677_d_n0);
        let eq55_e678_d_n2: f64 = (p.p50 * eq55_e677_d_n2);
        let eq55_e678_d_n6: f64 = (p.p50 * eq55_e677_d_n6);
        let eq55_e678_d_n7: f64 = (p.p50 * eq55_e677_d_n7);
        let eq55_e678_d_n10: f64 = (p.p50 * eq55_e677_d_n10);
        let eq55_e678_d_n11: f64 = (p.p50 * eq55_e677_d_n11);
        let eq55_e678_d_n12: f64 = (p.p50 * eq55_e677_d_n12);
        let eq55_e678_d_n17: f64 = (p.p50 * eq55_e677_d_n17);
        (eq55_e678, eq55_e678_d_n0, eq55_e678_d_n2, eq55_e678_d_n6, eq55_e678_d_n7, eq55_e678_d_n10, eq55_e678_d_n11, eq55_e678_d_n12, eq55_e678_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e680;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq55_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq55_e680_d_n0), multiplicity * (eq55_e680_d_n2), multiplicity * (eq55_e680_d_n6), multiplicity * (eq55_e680_d_n7), multiplicity * (eq55_e680_d_n10), multiplicity * (eq55_e680_d_n11), multiplicity * (eq55_e680_d_n12), multiplicity * (eq55_e680_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq59_e713, eq59_e713_d_n17,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p37 != 0.0)) {
        let eq59_e708: f64 = (1e-9 / 0.0001);
        let eq59_e710: f64 = (eq59_e708 * (nv17 - 0.0));
        let eq59_e711: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq59_e710);
        (eq59_e711, (eq59_e708 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq59_value: f64 = eq59_e713;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq59_value),
            17,
            multiplicity * (eq59_e713_d_n17),
        );
        let (eq67_e781, eq67_e781_d_n15,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p34 != 0.0)) {
        let eq67_e776: f64 = (1e-9 / 0.0001);
        let eq67_e778: f64 = (eq67_e776 * (nv15 - 0.0));
        let eq67_e779: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq67_e778);
        (eq67_e779, (eq67_e776 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq67_value: f64 = eq67_e781;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq67_value),
            15,
            multiplicity * (eq67_e781_d_n15),
        );
        let (eq68_e793, eq68_e793_d_n16,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p34 != 0.0)) {
        let eq68_e788: f64 = (1e-9 / 0.0001);
        let eq68_e790: f64 = (eq68_e788 * (nv16 - 0.0));
        let eq68_e791: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq68_e790);
        (eq68_e791, (eq68_e788 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e793;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (eq68_value),
            16,
            multiplicity * (eq68_e793_d_n16),
        );
        let (eq69_e805, eq69_e805_d_n13,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p34 != 0.0)) {
        let eq69_e800: f64 = (1e-9 / 0.0001);
        let eq69_e802: f64 = (eq69_e800 * (nv13 - 0.0));
        let eq69_e803: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq69_e802);
        (eq69_e803, (eq69_e800 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e805;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq69_value),
            13,
            multiplicity * (eq69_e805_d_n13),
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
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv13 = ctx.node_voltage(nodes[13]);
        let nv14 = ctx.node_voltage(nodes[14]);
        let nv15 = ctx.node_voltage(nodes[15]);
        let nv16 = ctx.node_voltage(nodes[16]);
        let nv17 = ctx.node_voltage(nodes[17]);
        let nv18 = ctx.node_voltage(nodes[18]);
        let eq10_e356_q: f64 = locals.var_qg;
        let eq10_e357: f64 = (p.p50 * locals.var_qg);
        let eq10_e357_d_n0: f64 = (p.p50 * locals.var_qg_dn0);
        let eq10_e357_d_n2: f64 = (p.p50 * locals.var_qg_dn2);
        let eq10_e357_d_n6: f64 = (p.p50 * locals.var_qg_dn6);
        let eq10_e357_d_n7: f64 = (p.p50 * locals.var_qg_dn7);
        let eq10_e357_d_n10: f64 = (p.p50 * locals.var_qg_dn10);
        let eq10_e357_d_n11: f64 = (p.p50 * locals.var_qg_dn11);
        let eq10_e357_d_n12: f64 = (p.p50 * locals.var_qg_dn12);
        let eq10_e357_d_n13: f64 = (p.p50 * locals.var_qg_dn13);
        let eq10_e357_d_n15: f64 = (p.p50 * locals.var_qg_dn15);
        let eq10_e357_d_n16: f64 = (p.p50 * locals.var_qg_dn16);
        let eq10_e357_d_n17: f64 = (p.p50 * locals.var_qg_dn17);
        let eq10_e357_d_n18: f64 = (p.p50 * locals.var_qg_dn18);
        let eq10_e357_q: f64 = (p.p50 * eq10_e356_q);
        let eq10_reactive_node_derivatives: [f64; 19] = [eq10_e357_d_n0, 0.0, eq10_e357_d_n2, 0.0, 0.0, 0.0, eq10_e357_d_n6, eq10_e357_d_n7, 0.0, 0.0, eq10_e357_d_n10, eq10_e357_d_n11, eq10_e357_d_n12, eq10_e357_d_n13, 0.0, eq10_e357_d_n15, eq10_e357_d_n16, eq10_e357_d_n17, eq10_e357_d_n18];
        let eq10_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq10_reactive_node_derivatives,
            branches,
            &eq10_reactive_branch_derivatives,
            multiplicity,
        );
        let eq11_e360_q: f64 = locals.var_qd;
        let eq11_e361: f64 = (p.p50 * locals.var_qd);
        let eq11_e361_d_n0: f64 = (p.p50 * locals.var_qd_dn0);
        let eq11_e361_d_n2: f64 = (p.p50 * locals.var_qd_dn2);
        let eq11_e361_d_n6: f64 = (p.p50 * locals.var_qd_dn6);
        let eq11_e361_d_n7: f64 = (p.p50 * locals.var_qd_dn7);
        let eq11_e361_d_n10: f64 = (p.p50 * locals.var_qd_dn10);
        let eq11_e361_d_n11: f64 = (p.p50 * locals.var_qd_dn11);
        let eq11_e361_d_n12: f64 = (p.p50 * locals.var_qd_dn12);
        let eq11_e361_d_n13: f64 = (p.p50 * locals.var_qd_dn13);
        let eq11_e361_d_n15: f64 = (p.p50 * locals.var_qd_dn15);
        let eq11_e361_d_n16: f64 = (p.p50 * locals.var_qd_dn16);
        let eq11_e361_d_n17: f64 = (p.p50 * locals.var_qd_dn17);
        let eq11_e361_d_n18: f64 = (p.p50 * locals.var_qd_dn18);
        let eq11_e361_q: f64 = (p.p50 * eq11_e360_q);
        let eq11_reactive_node_derivatives: [f64; 19] = [eq11_e361_d_n0, 0.0, eq11_e361_d_n2, 0.0, 0.0, 0.0, eq11_e361_d_n6, eq11_e361_d_n7, 0.0, 0.0, eq11_e361_d_n10, eq11_e361_d_n11, eq11_e361_d_n12, eq11_e361_d_n13, 0.0, eq11_e361_d_n15, eq11_e361_d_n16, eq11_e361_d_n17, eq11_e361_d_n18];
        let eq11_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq12_e364_q: f64 = locals.var_qb;
        let eq12_e365: f64 = (p.p50 * locals.var_qb);
        let eq12_e365_d_n0: f64 = (p.p50 * locals.var_qb_dn0);
        let eq12_e365_d_n2: f64 = (p.p50 * locals.var_qb_dn2);
        let eq12_e365_d_n6: f64 = (p.p50 * locals.var_qb_dn6);
        let eq12_e365_d_n7: f64 = (p.p50 * locals.var_qb_dn7);
        let eq12_e365_d_n10: f64 = (p.p50 * locals.var_qb_dn10);
        let eq12_e365_d_n11: f64 = (p.p50 * locals.var_qb_dn11);
        let eq12_e365_d_n12: f64 = (p.p50 * locals.var_qb_dn12);
        let eq12_e365_d_n13: f64 = (p.p50 * locals.var_qb_dn13);
        let eq12_e365_d_n15: f64 = (p.p50 * locals.var_qb_dn15);
        let eq12_e365_d_n16: f64 = (p.p50 * locals.var_qb_dn16);
        let eq12_e365_d_n17: f64 = (p.p50 * locals.var_qb_dn17);
        let eq12_e365_d_n18: f64 = (p.p50 * locals.var_qb_dn18);
        let eq12_e365_q: f64 = (p.p50 * eq12_e364_q);
        let eq12_reactive_node_derivatives: [f64; 19] = [eq12_e365_d_n0, 0.0, eq12_e365_d_n2, 0.0, 0.0, 0.0, eq12_e365_d_n6, eq12_e365_d_n7, 0.0, 0.0, eq12_e365_d_n10, eq12_e365_d_n11, eq12_e365_d_n12, eq12_e365_d_n13, 0.0, eq12_e365_d_n15, eq12_e365_d_n16, eq12_e365_d_n17, eq12_e365_d_n18];
        let eq12_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq18_e394: f64 = ((nv14 - 0.0) * locals.var_sigrat_s);
        let eq18_e394_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn0);
        let eq18_e394_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn2);
        let eq18_e394_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn6);
        let eq18_e394_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn7);
        let eq18_e394_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn10);
        let eq18_e394_d_n11: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn11);
        let eq18_e394_d_n12: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn12);
        let eq18_e394_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn13);
        let eq18_e394_d_n15: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn15);
        let eq18_e394_d_n16: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn16);
        let eq18_e394_d_n17: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn17);
        let eq18_e394_d_n18: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn18);
        let eq18_e395_q: f64 = eq18_e394;
        let eq18_reactive_node_derivatives: [f64; 19] = [eq18_e394_d_n0, 0.0, eq18_e394_d_n2, 0.0, 0.0, 0.0, eq18_e394_d_n6, eq18_e394_d_n7, 0.0, 0.0, eq18_e394_d_n10, eq18_e394_d_n11, eq18_e394_d_n12, eq18_e394_d_n13, locals.var_sigrat_s, eq18_e394_d_n15, eq18_e394_d_n16, eq18_e394_d_n17, eq18_e394_d_n18];
        let eq18_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq18_reactive_node_derivatives,
            branches,
            &eq18_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e398: f64 = ((nv14 - 0.0) * locals.var_sigrat_d);
        let eq19_e398_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn0);
        let eq19_e398_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn2);
        let eq19_e398_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn6);
        let eq19_e398_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn7);
        let eq19_e398_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn10);
        let eq19_e398_d_n11: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn11);
        let eq19_e398_d_n12: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn12);
        let eq19_e398_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn13);
        let eq19_e398_d_n15: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn15);
        let eq19_e398_d_n16: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn16);
        let eq19_e398_d_n17: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn17);
        let eq19_e398_d_n18: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn18);
        let eq19_e399_q: f64 = eq19_e398;
        let eq19_reactive_node_derivatives: [f64; 19] = [eq19_e398_d_n0, 0.0, eq19_e398_d_n2, 0.0, 0.0, 0.0, eq19_e398_d_n6, eq19_e398_d_n7, 0.0, 0.0, eq19_e398_d_n10, eq19_e398_d_n11, eq19_e398_d_n12, eq19_e398_d_n13, locals.var_sigrat_d, eq19_e398_d_n15, eq19_e398_d_n16, eq19_e398_d_n17, eq19_e398_d_n18];
        let eq19_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq30_e480, eq30_e480_d_n10, eq30_e480_q,) = {
    if (locals.var_guard1222 != 0.0) {
        let eq30_e477: f64 = (locals.var_cthe * (nv10 - 0.0));
        let eq30_e478_q: f64 = eq30_e477;
        (eq30_e477, locals.var_cthe, eq30_e478_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[10]),
            None,
            nodes[10],
            multiplicity * (eq30_e480_d_n10),
        );
        let (eq34_e512, eq34_e512_d_n0, eq34_e512_d_n2, eq34_e512_d_n6, eq34_e512_d_n7, eq34_e512_d_n10, eq34_e512_d_n11, eq34_e512_d_n12, eq34_e512_d_n17, eq34_e512_q, eq34_e512_q_d_n0, eq34_e512_q_d_n2, eq34_e512_q_d_n6, eq34_e512_q_d_n7, eq34_e512_q_d_n10, eq34_e512_q_d_n11, eq34_e512_q_d_n12, eq34_e512_q_d_n17,) = {
    if (locals.var_guard1223 != 0.0) {
        let eq34_e508_q: f64 = locals.var_qbs;
        let eq34_e509: f64 = (locals.var_ibs + locals.var_qbs);
        let eq34_e509_d_n0: f64 = (locals.var_ibs_dn0 + locals.var_qbs_dn0);
        let eq34_e509_d_n2: f64 = (locals.var_ibs_dn2 + locals.var_qbs_dn2);
        let eq34_e509_d_n6: f64 = (locals.var_ibs_dn6 + locals.var_qbs_dn6);
        let eq34_e509_d_n7: f64 = (locals.var_ibs_dn7 + locals.var_qbs_dn7);
        let eq34_e509_d_n10: f64 = (locals.var_ibs_dn10 + locals.var_qbs_dn10);
        let eq34_e509_d_n11: f64 = (locals.var_ibs_dn11 + locals.var_qbs_dn11);
        let eq34_e509_d_n12: f64 = (locals.var_ibs_dn12 + locals.var_qbs_dn12);
        let eq34_e509_d_n17: f64 = (locals.var_ibs_dn17 + locals.var_qbs_dn17);
        let eq34_e509_q: f64 = eq34_e508_q;
        let eq34_e510: f64 = (p.p50 * eq34_e509);
        let eq34_e510_d_n0: f64 = (p.p50 * eq34_e509_d_n0);
        let eq34_e510_d_n2: f64 = (p.p50 * eq34_e509_d_n2);
        let eq34_e510_d_n6: f64 = (p.p50 * eq34_e509_d_n6);
        let eq34_e510_d_n7: f64 = (p.p50 * eq34_e509_d_n7);
        let eq34_e510_d_n10: f64 = (p.p50 * eq34_e509_d_n10);
        let eq34_e510_d_n11: f64 = (p.p50 * eq34_e509_d_n11);
        let eq34_e510_d_n12: f64 = (p.p50 * eq34_e509_d_n12);
        let eq34_e510_d_n17: f64 = (p.p50 * eq34_e509_d_n17);
        let eq34_e510_q: f64 = (p.p50 * eq34_e509_q);
        let eq34_e510_q_d_n0: f64 = (p.p50 * locals.var_qbs_dn0);
        let eq34_e510_q_d_n2: f64 = (p.p50 * locals.var_qbs_dn2);
        let eq34_e510_q_d_n6: f64 = (p.p50 * locals.var_qbs_dn6);
        let eq34_e510_q_d_n7: f64 = (p.p50 * locals.var_qbs_dn7);
        let eq34_e510_q_d_n10: f64 = (p.p50 * locals.var_qbs_dn10);
        let eq34_e510_q_d_n11: f64 = (p.p50 * locals.var_qbs_dn11);
        let eq34_e510_q_d_n12: f64 = (p.p50 * locals.var_qbs_dn12);
        let eq34_e510_q_d_n17: f64 = (p.p50 * locals.var_qbs_dn17);
        (eq34_e510, eq34_e510_d_n0, eq34_e510_d_n2, eq34_e510_d_n6, eq34_e510_d_n7, eq34_e510_d_n10, eq34_e510_d_n11, eq34_e510_d_n12, eq34_e510_d_n17, eq34_e510_q, eq34_e510_q_d_n0, eq34_e510_q_d_n2, eq34_e510_q_d_n6, eq34_e510_q_d_n7, eq34_e510_q_d_n10, eq34_e510_q_d_n11, eq34_e510_q_d_n12, eq34_e510_q_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_reactive_node_derivatives: [f64; 19] = [eq34_e512_q_d_n0, 0.0, eq34_e512_q_d_n2, 0.0, 0.0, 0.0, eq34_e512_q_d_n6, eq34_e512_q_d_n7, 0.0, 0.0, eq34_e512_q_d_n10, eq34_e512_q_d_n11, eq34_e512_q_d_n12, 0.0, 0.0, 0.0, 0.0, eq34_e512_q_d_n17, 0.0];
        let eq34_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq34_reactive_node_derivatives,
            branches,
            &eq34_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq35_e521, eq35_e521_d_n0, eq35_e521_d_n2, eq35_e521_d_n6, eq35_e521_d_n7, eq35_e521_d_n10, eq35_e521_d_n11, eq35_e521_d_n12, eq35_e521_d_n17, eq35_e521_q, eq35_e521_q_d_n0, eq35_e521_q_d_n2, eq35_e521_q_d_n6, eq35_e521_q_d_n7, eq35_e521_q_d_n10, eq35_e521_q_d_n11, eq35_e521_q_d_n12, eq35_e521_q_d_n17,) = {
    if (locals.var_guard1223 != 0.0) {
        let eq35_e517_q: f64 = locals.var_qbd;
        let eq35_e518: f64 = (locals.var_ibd + locals.var_qbd);
        let eq35_e518_d_n0: f64 = (locals.var_ibd_dn0 + locals.var_qbd_dn0);
        let eq35_e518_d_n2: f64 = (locals.var_ibd_dn2 + locals.var_qbd_dn2);
        let eq35_e518_d_n6: f64 = (locals.var_ibd_dn6 + locals.var_qbd_dn6);
        let eq35_e518_d_n7: f64 = (locals.var_ibd_dn7 + locals.var_qbd_dn7);
        let eq35_e518_d_n10: f64 = (locals.var_ibd_dn10 + locals.var_qbd_dn10);
        let eq35_e518_d_n11: f64 = (locals.var_ibd_dn11 + locals.var_qbd_dn11);
        let eq35_e518_d_n12: f64 = (locals.var_ibd_dn12 + locals.var_qbd_dn12);
        let eq35_e518_d_n17: f64 = (locals.var_ibd_dn17 + locals.var_qbd_dn17);
        let eq35_e518_q: f64 = eq35_e517_q;
        let eq35_e519: f64 = (p.p50 * eq35_e518);
        let eq35_e519_d_n0: f64 = (p.p50 * eq35_e518_d_n0);
        let eq35_e519_d_n2: f64 = (p.p50 * eq35_e518_d_n2);
        let eq35_e519_d_n6: f64 = (p.p50 * eq35_e518_d_n6);
        let eq35_e519_d_n7: f64 = (p.p50 * eq35_e518_d_n7);
        let eq35_e519_d_n10: f64 = (p.p50 * eq35_e518_d_n10);
        let eq35_e519_d_n11: f64 = (p.p50 * eq35_e518_d_n11);
        let eq35_e519_d_n12: f64 = (p.p50 * eq35_e518_d_n12);
        let eq35_e519_d_n17: f64 = (p.p50 * eq35_e518_d_n17);
        let eq35_e519_q: f64 = (p.p50 * eq35_e518_q);
        let eq35_e519_q_d_n0: f64 = (p.p50 * locals.var_qbd_dn0);
        let eq35_e519_q_d_n2: f64 = (p.p50 * locals.var_qbd_dn2);
        let eq35_e519_q_d_n6: f64 = (p.p50 * locals.var_qbd_dn6);
        let eq35_e519_q_d_n7: f64 = (p.p50 * locals.var_qbd_dn7);
        let eq35_e519_q_d_n10: f64 = (p.p50 * locals.var_qbd_dn10);
        let eq35_e519_q_d_n11: f64 = (p.p50 * locals.var_qbd_dn11);
        let eq35_e519_q_d_n12: f64 = (p.p50 * locals.var_qbd_dn12);
        let eq35_e519_q_d_n17: f64 = (p.p50 * locals.var_qbd_dn17);
        (eq35_e519, eq35_e519_d_n0, eq35_e519_d_n2, eq35_e519_d_n6, eq35_e519_d_n7, eq35_e519_d_n10, eq35_e519_d_n11, eq35_e519_d_n12, eq35_e519_d_n17, eq35_e519_q, eq35_e519_q_d_n0, eq35_e519_q_d_n2, eq35_e519_q_d_n6, eq35_e519_q_d_n7, eq35_e519_q_d_n10, eq35_e519_q_d_n11, eq35_e519_q_d_n12, eq35_e519_q_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_reactive_node_derivatives: [f64; 19] = [eq35_e521_q_d_n0, 0.0, eq35_e521_q_d_n2, 0.0, 0.0, 0.0, eq35_e521_q_d_n6, eq35_e521_q_d_n7, 0.0, 0.0, eq35_e521_q_d_n10, eq35_e521_q_d_n11, eq35_e521_q_d_n12, 0.0, 0.0, 0.0, 0.0, eq35_e521_q_d_n17, 0.0];
        let eq35_reactive_branch_derivatives: [f64; 15] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq46_e605, eq46_e605_d_n18, eq46_e605_q,) = {
    if ((locals.var_guard1223 != 0.0) && (p.p34 != 0.0)) {
        let eq46_e600: f64 = (1e-9 / 0.0001);
        let eq46_e602: f64 = (eq46_e600 * (nv18 - 0.0));
        let eq46_e603_q: f64 = eq46_e602;
        (eq46_e602, eq46_e600, eq46_e603_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[18]),
            None,
            nodes[18],
            multiplicity * (eq46_e605_d_n18),
        );
        let (eq47_e616, eq47_e616_d_n13, eq47_e616_q,) = {
    if ((locals.var_guard1223 != 0.0) && (p.p34 != 0.0)) {
        let eq47_e611: f64 = (1e-9 / 0.0001);
        let eq47_e613: f64 = (eq47_e611 * (nv13 - 0.0));
        let eq47_e614_q: f64 = eq47_e613;
        (eq47_e613, eq47_e611, eq47_e614_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq47_e616_d_n13),
        );
        let (eq52_e655, eq52_e655_d_n17, eq52_e655_q,) = {
    if ((locals.var_guard1223 != 0.0) && (locals.var_guard1224 != 0.0)) {
        let eq52_e650: f64 = (1e-9 / 0.0001);
        let eq52_e652: f64 = (eq52_e650 * (nv17 - 0.0));
        let eq52_e653_q: f64 = eq52_e652;
        (eq52_e652, eq52_e650, eq52_e653_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq52_e655_d_n17),
        );
        let (eq59_e713, eq59_e713_d_n17, eq59_e713_q,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p37 != 0.0)) {
        let eq59_e708: f64 = (1e-9 / 0.0001);
        let eq59_e710: f64 = (eq59_e708 * (nv17 - 0.0));
        let eq59_e711_q: f64 = eq59_e710;
        (eq59_e710, eq59_e708, eq59_e711_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq59_e713_d_n17),
        );
        let (eq67_e781, eq67_e781_d_n15, eq67_e781_q,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p34 != 0.0)) {
        let eq67_e776: f64 = (1e-9 / 0.0001);
        let eq67_e778: f64 = (eq67_e776 * (nv15 - 0.0));
        let eq67_e779_q: f64 = eq67_e778;
        (eq67_e778, eq67_e776, eq67_e779_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (eq67_e781_d_n15),
        );
        let (eq68_e793, eq68_e793_d_n16, eq68_e793_q,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p34 != 0.0)) {
        let eq68_e788: f64 = (1e-9 / 0.0001);
        let eq68_e790: f64 = (eq68_e788 * (nv16 - 0.0));
        let eq68_e791_q: f64 = eq68_e790;
        (eq68_e790, eq68_e788, eq68_e791_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[16]),
            None,
            nodes[16],
            multiplicity * (eq68_e793_d_n16),
        );
        let (eq69_e805, eq69_e805_d_n13, eq69_e805_q,) = {
    if ((locals.var_guard1223 == 0.0) && (p.p34 != 0.0)) {
        let eq69_e800: f64 = (1e-9 / 0.0001);
        let eq69_e802: f64 = (eq69_e800 * (nv13 - 0.0));
        let eq69_e803_q: f64 = eq69_e802;
        (eq69_e802, eq69_e800, eq69_e803_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq69_e805_d_n13),
        );
    }
}

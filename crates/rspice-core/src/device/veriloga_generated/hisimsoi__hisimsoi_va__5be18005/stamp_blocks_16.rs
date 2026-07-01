#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_129(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign36650_e51315, assign36650_e51315_d_n0, assign36650_e51315_d_n2, assign36650_e51315_d_n6, assign36650_e51315_d_n7, assign36650_e51315_d_n10, assign36650_e51315_d_n11, assign36650_e51315_d_n12, assign36650_e51315_d_n16, assign36650_e51315_d_n17, assign36650_e51315_d_n18,) = {
    if ((locals.var_guard1209 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qs_nqs, locals.var_qs_nqs_dn0, locals.var_qs_nqs_dn2, locals.var_qs_nqs_dn6, locals.var_qs_nqs_dn7, locals.var_qs_nqs_dn10, locals.var_qs_nqs_dn11, locals.var_qs_nqs_dn12, locals.var_qs_nqs_dn16, locals.var_qs_nqs_dn17, locals.var_qs_nqs_dn18,)
    }
};
        locals.var_qs_nqs = assign36650_e51315;
        locals.var_qs_nqs_dn0 = assign36650_e51315_d_n0;
        locals.var_qs_nqs_dn2 = assign36650_e51315_d_n2;
        locals.var_qs_nqs_dn6 = assign36650_e51315_d_n6;
        locals.var_qs_nqs_dn7 = assign36650_e51315_d_n7;
        locals.var_qs_nqs_dn10 = assign36650_e51315_d_n10;
        locals.var_qs_nqs_dn11 = assign36650_e51315_d_n11;
        locals.var_qs_nqs_dn12 = assign36650_e51315_d_n12;
        locals.var_qs_nqs_dn16 = assign36650_e51315_d_n16;
        locals.var_qs_nqs_dn17 = assign36650_e51315_d_n17;
        locals.var_qs_nqs_dn18 = assign36650_e51315_d_n18;
        locals.var_qs_nqs_rv = 0.0;

        let (assign36660_e51323, assign36660_e51323_d_n0, assign36660_e51323_d_n2, assign36660_e51323_d_n6, assign36660_e51323_d_n7, assign36660_e51323_d_n10, assign36660_e51323_d_n11, assign36660_e51323_d_n12, assign36660_e51323_d_n13, assign36660_e51323_d_n15, assign36660_e51323_d_n16, assign36660_e51323_d_n17, assign36660_e51323_d_n18,) = {
    if ((locals.var_guard1209 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_qg_nqs, locals.var_qg_nqs_dn0, locals.var_qg_nqs_dn2, locals.var_qg_nqs_dn6, locals.var_qg_nqs_dn7, locals.var_qg_nqs_dn10, locals.var_qg_nqs_dn11, locals.var_qg_nqs_dn12, locals.var_qg_nqs_dn13, locals.var_qg_nqs_dn15, locals.var_qg_nqs_dn16, locals.var_qg_nqs_dn17, locals.var_qg_nqs_dn18,)
    }
};
        locals.var_qg_nqs = assign36660_e51323;
        locals.var_qg_nqs_dn0 = assign36660_e51323_d_n0;
        locals.var_qg_nqs_dn2 = assign36660_e51323_d_n2;
        locals.var_qg_nqs_dn6 = assign36660_e51323_d_n6;
        locals.var_qg_nqs_dn7 = assign36660_e51323_d_n7;
        locals.var_qg_nqs_dn10 = assign36660_e51323_d_n10;
        locals.var_qg_nqs_dn11 = assign36660_e51323_d_n11;
        locals.var_qg_nqs_dn12 = assign36660_e51323_d_n12;
        locals.var_qg_nqs_dn13 = assign36660_e51323_d_n13;
        locals.var_qg_nqs_dn15 = assign36660_e51323_d_n15;
        locals.var_qg_nqs_dn16 = assign36660_e51323_d_n16;
        locals.var_qg_nqs_dn17 = assign36660_e51323_d_n17;
        locals.var_qg_nqs_dn18 = assign36660_e51323_d_n18;
        locals.var_qg_nqs_rv = 0.0;

        let (assign36670_e51331, assign36670_e51331_d_n13,) = {
    if ((locals.var_guard1209 == 0.0) && (locals.var_flg_nqs == 0.0)) {
        (0.0, 0.0,)
    } else {
        (locals.var_qb_nqs, locals.var_qb_nqs_dn13,)
    }
};
        locals.var_qb_nqs = assign36670_e51331;
        locals.var_qb_nqs_dn13 = assign36670_e51331_d_n13;
        locals.var_qb_nqs_rv = 0.0;

        let assign36700_e51336: f64 = if locals.var_mode == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1214 = assign36700_e51336;
        locals.var_guard1214_rv = 0.0;

        let (assign36710_e51340, assign36710_e51340_d_n0, assign36710_e51340_d_n2, assign36710_e51340_d_n6, assign36710_e51340_d_n7, assign36710_e51340_d_n10, assign36710_e51340_d_n11, assign36710_e51340_d_n12, assign36710_e51340_d_n17,) = {
    if (locals.var_guard1214 != 0.0) {
        (locals.var_idse, locals.var_idse_dn0, locals.var_idse_dn2, locals.var_idse_dn6, locals.var_idse_dn7, locals.var_idse_dn10, locals.var_idse_dn11, locals.var_idse_dn12, locals.var_idse_dn17,)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign36710_e51340;
        locals.var_ids_dn0 = assign36710_e51340_d_n0;
        locals.var_ids_dn2 = assign36710_e51340_d_n2;
        locals.var_ids_dn6 = assign36710_e51340_d_n6;
        locals.var_ids_dn7 = assign36710_e51340_d_n7;
        locals.var_ids_dn10 = assign36710_e51340_d_n10;
        locals.var_ids_dn11 = assign36710_e51340_d_n11;
        locals.var_ids_dn12 = assign36710_e51340_d_n12;
        locals.var_ids_dn17 = assign36710_e51340_d_n17;
        locals.var_ids_rv = 0.0;

        let (assign36720_e51344, assign36720_e51344_d_n0, assign36720_e51344_d_n2, assign36720_e51344_d_n6, assign36720_e51344_d_n7, assign36720_e51344_d_n10, assign36720_e51344_d_n11, assign36720_e51344_d_n12, assign36720_e51344_d_n17,) = {
    if (locals.var_guard1214 != 0.0) {
        (locals.var_isube, locals.var_isube_dn0, locals.var_isube_dn2, locals.var_isube_dn6, locals.var_isube_dn7, locals.var_isube_dn10, locals.var_isube_dn11, locals.var_isube_dn12, locals.var_isube_dn17,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign36720_e51344;
        locals.var_isub_dn0 = assign36720_e51344_d_n0;
        locals.var_isub_dn2 = assign36720_e51344_d_n2;
        locals.var_isub_dn6 = assign36720_e51344_d_n6;
        locals.var_isub_dn7 = assign36720_e51344_d_n7;
        locals.var_isub_dn10 = assign36720_e51344_d_n10;
        locals.var_isub_dn11 = assign36720_e51344_d_n11;
        locals.var_isub_dn12 = assign36720_e51344_d_n12;
        locals.var_isub_dn17 = assign36720_e51344_d_n17;
        locals.var_isub_rv = 0.0;

        let (assign36740_e51354, assign36740_e51354_d_n0, assign36740_e51354_d_n2, assign36740_e51354_d_n6, assign36740_e51354_d_n7, assign36740_e51354_d_n10, assign36740_e51354_d_n11, assign36740_e51354_d_n12, assign36740_e51354_d_n13, assign36740_e51354_d_n15, assign36740_e51354_d_n16, assign36740_e51354_d_n17, assign36740_e51354_d_n18,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign36740_e51352: f64 = (locals.var_qge + locals.var_qg_nqs);
        (assign36740_e51352, (locals.var_qge_dn0 + locals.var_qg_nqs_dn0), (locals.var_qge_dn2 + locals.var_qg_nqs_dn2), (locals.var_qge_dn6 + locals.var_qg_nqs_dn6), (locals.var_qge_dn7 + locals.var_qg_nqs_dn7), (locals.var_qge_dn10 + locals.var_qg_nqs_dn10), (locals.var_qge_dn11 + locals.var_qg_nqs_dn11), (locals.var_qge_dn12 + locals.var_qg_nqs_dn12), (locals.var_qge_dn13 + locals.var_qg_nqs_dn13), (locals.var_qge_dn15 + locals.var_qg_nqs_dn15), (locals.var_qge_dn16 + locals.var_qg_nqs_dn16), (locals.var_qge_dn17 + locals.var_qg_nqs_dn17), (locals.var_qge_dn18 + locals.var_qg_nqs_dn18),)
    } else {
        (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12, locals.var_qg_dn13, locals.var_qg_dn15, locals.var_qg_dn16, locals.var_qg_dn17, locals.var_qg_dn18,)
    }
};
        locals.var_qg = assign36740_e51354;
        locals.var_qg_dn0 = assign36740_e51354_d_n0;
        locals.var_qg_dn2 = assign36740_e51354_d_n2;
        locals.var_qg_dn6 = assign36740_e51354_d_n6;
        locals.var_qg_dn7 = assign36740_e51354_d_n7;
        locals.var_qg_dn10 = assign36740_e51354_d_n10;
        locals.var_qg_dn11 = assign36740_e51354_d_n11;
        locals.var_qg_dn12 = assign36740_e51354_d_n12;
        locals.var_qg_dn13 = assign36740_e51354_d_n13;
        locals.var_qg_dn15 = assign36740_e51354_d_n15;
        locals.var_qg_dn16 = assign36740_e51354_d_n16;
        locals.var_qg_dn17 = assign36740_e51354_d_n17;
        locals.var_qg_dn18 = assign36740_e51354_d_n18;
        locals.var_qg_rv = 0.0;

        let (assign36750_e51360, assign36750_e51360_d_n0, assign36750_e51360_d_n2, assign36750_e51360_d_n6, assign36750_e51360_d_n7, assign36750_e51360_d_n10, assign36750_e51360_d_n11, assign36750_e51360_d_n12, assign36750_e51360_d_n13, assign36750_e51360_d_n15, assign36750_e51360_d_n16, assign36750_e51360_d_n17, assign36750_e51360_d_n18,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign36750_e51358: f64 = (locals.var_qde + locals.var_qd_nqs);
        (assign36750_e51358, (locals.var_qde_dn0 + locals.var_qd_nqs_dn0), (locals.var_qde_dn2 + locals.var_qd_nqs_dn2), (locals.var_qde_dn6 + locals.var_qd_nqs_dn6), (locals.var_qde_dn7 + locals.var_qd_nqs_dn7), (locals.var_qde_dn10 + locals.var_qd_nqs_dn10), (locals.var_qde_dn11 + locals.var_qd_nqs_dn11), (locals.var_qde_dn12 + locals.var_qd_nqs_dn12), locals.var_qde_dn13, (locals.var_qde_dn15 + locals.var_qd_nqs_dn15), locals.var_qde_dn16, (locals.var_qde_dn17 + locals.var_qd_nqs_dn17), (locals.var_qde_dn18 + locals.var_qd_nqs_dn18),)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18,)
    }
};
        locals.var_qd = assign36750_e51360;
        locals.var_qd_dn0 = assign36750_e51360_d_n0;
        locals.var_qd_dn2 = assign36750_e51360_d_n2;
        locals.var_qd_dn6 = assign36750_e51360_d_n6;
        locals.var_qd_dn7 = assign36750_e51360_d_n7;
        locals.var_qd_dn10 = assign36750_e51360_d_n10;
        locals.var_qd_dn11 = assign36750_e51360_d_n11;
        locals.var_qd_dn12 = assign36750_e51360_d_n12;
        locals.var_qd_dn13 = assign36750_e51360_d_n13;
        locals.var_qd_dn15 = assign36750_e51360_d_n15;
        locals.var_qd_dn16 = assign36750_e51360_d_n16;
        locals.var_qd_dn17 = assign36750_e51360_d_n17;
        locals.var_qd_dn18 = assign36750_e51360_d_n18;
        locals.var_qd_rv = 0.0;

        let (assign36770_e51375, assign36770_e51375_d_n0, assign36770_e51375_d_n2, assign36770_e51375_d_n6, assign36770_e51375_d_n7, assign36770_e51375_d_n10, assign36770_e51375_d_n11, assign36770_e51375_d_n12, assign36770_e51375_d_n13, assign36770_e51375_d_n15, assign36770_e51375_d_n16, assign36770_e51375_d_n17, assign36770_e51375_d_n18,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign36770_e51370: f64 = (locals.var_qge + locals.var_qde);
        let assign36770_e51372: f64 = (assign36770_e51370 + locals.var_qse);
        let assign36770_e51373: f64 = (-assign36770_e51372);
        (assign36770_e51373, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)), (-((locals.var_qge_dn15 + locals.var_qde_dn15) + locals.var_qse_dn15)), (-((locals.var_qge_dn16 + locals.var_qde_dn16) + locals.var_qse_dn16)), (-((locals.var_qge_dn17 + locals.var_qde_dn17) + locals.var_qse_dn17)), (-((locals.var_qge_dn18 + locals.var_qde_dn18) + locals.var_qse_dn18)),)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    }
};
        locals.var_qbe = assign36770_e51375;
        locals.var_qbe_dn0 = assign36770_e51375_d_n0;
        locals.var_qbe_dn2 = assign36770_e51375_d_n2;
        locals.var_qbe_dn6 = assign36770_e51375_d_n6;
        locals.var_qbe_dn7 = assign36770_e51375_d_n7;
        locals.var_qbe_dn10 = assign36770_e51375_d_n10;
        locals.var_qbe_dn11 = assign36770_e51375_d_n11;
        locals.var_qbe_dn12 = assign36770_e51375_d_n12;
        locals.var_qbe_dn13 = assign36770_e51375_d_n13;
        locals.var_qbe_dn15 = assign36770_e51375_d_n15;
        locals.var_qbe_dn16 = assign36770_e51375_d_n16;
        locals.var_qbe_dn17 = assign36770_e51375_d_n17;
        locals.var_qbe_dn18 = assign36770_e51375_d_n18;
        locals.var_qbe_rv = 0.0;

        let (assign36780_e51381, assign36780_e51381_d_n0, assign36780_e51381_d_n2, assign36780_e51381_d_n6, assign36780_e51381_d_n7, assign36780_e51381_d_n10, assign36780_e51381_d_n11, assign36780_e51381_d_n12, assign36780_e51381_d_n13, assign36780_e51381_d_n15, assign36780_e51381_d_n16, assign36780_e51381_d_n17, assign36780_e51381_d_n18,) = {
    if (locals.var_guard1214 != 0.0) {
        let assign36780_e51379: f64 = (locals.var_qbe + locals.var_qb_nqs);
        (assign36780_e51379, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, (locals.var_qbe_dn13 + locals.var_qb_nqs_dn13), locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18,)
    }
};
        locals.var_qb = assign36780_e51381;
        locals.var_qb_dn0 = assign36780_e51381_d_n0;
        locals.var_qb_dn2 = assign36780_e51381_d_n2;
        locals.var_qb_dn6 = assign36780_e51381_d_n6;
        locals.var_qb_dn7 = assign36780_e51381_d_n7;
        locals.var_qb_dn10 = assign36780_e51381_d_n10;
        locals.var_qb_dn11 = assign36780_e51381_d_n11;
        locals.var_qb_dn12 = assign36780_e51381_d_n12;
        locals.var_qb_dn13 = assign36780_e51381_d_n13;
        locals.var_qb_dn15 = assign36780_e51381_d_n15;
        locals.var_qb_dn16 = assign36780_e51381_d_n16;
        locals.var_qb_dn17 = assign36780_e51381_d_n17;
        locals.var_qb_dn18 = assign36780_e51381_d_n18;
        locals.var_qb_rv = 0.0;

        let (assign36790_e51387, assign36790_e51387_d_n0, assign36790_e51387_d_n2, assign36790_e51387_d_n6, assign36790_e51387_d_n7, assign36790_e51387_d_n10, assign36790_e51387_d_n11, assign36790_e51387_d_n12, assign36790_e51387_d_n17,) = {
    if (locals.var_guard1214 == 0.0) {
        let assign36790_e51385: f64 = (-locals.var_idse);
        (assign36790_e51385, (-locals.var_idse_dn0), (-locals.var_idse_dn2), (-locals.var_idse_dn6), (-locals.var_idse_dn7), (-locals.var_idse_dn10), (-locals.var_idse_dn11), (-locals.var_idse_dn12), (-locals.var_idse_dn17),)
    } else {
        (locals.var_ids, locals.var_ids_dn0, locals.var_ids_dn2, locals.var_ids_dn6, locals.var_ids_dn7, locals.var_ids_dn10, locals.var_ids_dn11, locals.var_ids_dn12, locals.var_ids_dn17,)
    }
};
        locals.var_ids = assign36790_e51387;
        locals.var_ids_dn0 = assign36790_e51387_d_n0;
        locals.var_ids_dn2 = assign36790_e51387_d_n2;
        locals.var_ids_dn6 = assign36790_e51387_d_n6;
        locals.var_ids_dn7 = assign36790_e51387_d_n7;
        locals.var_ids_dn10 = assign36790_e51387_d_n10;
        locals.var_ids_dn11 = assign36790_e51387_d_n11;
        locals.var_ids_dn12 = assign36790_e51387_d_n12;
        locals.var_ids_dn17 = assign36790_e51387_d_n17;
        locals.var_ids_rv = 0.0;

        let (assign36810_e51397, assign36810_e51397_d_n0, assign36810_e51397_d_n2, assign36810_e51397_d_n6, assign36810_e51397_d_n7, assign36810_e51397_d_n10, assign36810_e51397_d_n11, assign36810_e51397_d_n12, assign36810_e51397_d_n17,) = {
    if (locals.var_guard1214 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_isub, locals.var_isub_dn0, locals.var_isub_dn2, locals.var_isub_dn6, locals.var_isub_dn7, locals.var_isub_dn10, locals.var_isub_dn11, locals.var_isub_dn12, locals.var_isub_dn17,)
    }
};
        locals.var_isub = assign36810_e51397;
        locals.var_isub_dn0 = assign36810_e51397_d_n0;
        locals.var_isub_dn2 = assign36810_e51397_d_n2;
        locals.var_isub_dn6 = assign36810_e51397_d_n6;
        locals.var_isub_dn7 = assign36810_e51397_d_n7;
        locals.var_isub_dn10 = assign36810_e51397_d_n10;
        locals.var_isub_dn11 = assign36810_e51397_d_n11;
        locals.var_isub_dn12 = assign36810_e51397_d_n12;
        locals.var_isub_dn17 = assign36810_e51397_d_n17;
        locals.var_isub_rv = 0.0;

        let (assign36820_e51404, assign36820_e51404_d_n0, assign36820_e51404_d_n2, assign36820_e51404_d_n6, assign36820_e51404_d_n7, assign36820_e51404_d_n10, assign36820_e51404_d_n11, assign36820_e51404_d_n12, assign36820_e51404_d_n13, assign36820_e51404_d_n15, assign36820_e51404_d_n16, assign36820_e51404_d_n17, assign36820_e51404_d_n18,) = {
    if (locals.var_guard1214 == 0.0) {
        let assign36820_e51402: f64 = (locals.var_qge + locals.var_qg_nqs);
        (assign36820_e51402, (locals.var_qge_dn0 + locals.var_qg_nqs_dn0), (locals.var_qge_dn2 + locals.var_qg_nqs_dn2), (locals.var_qge_dn6 + locals.var_qg_nqs_dn6), (locals.var_qge_dn7 + locals.var_qg_nqs_dn7), (locals.var_qge_dn10 + locals.var_qg_nqs_dn10), (locals.var_qge_dn11 + locals.var_qg_nqs_dn11), (locals.var_qge_dn12 + locals.var_qg_nqs_dn12), (locals.var_qge_dn13 + locals.var_qg_nqs_dn13), (locals.var_qge_dn15 + locals.var_qg_nqs_dn15), (locals.var_qge_dn16 + locals.var_qg_nqs_dn16), (locals.var_qge_dn17 + locals.var_qg_nqs_dn17), (locals.var_qge_dn18 + locals.var_qg_nqs_dn18),)
    } else {
        (locals.var_qg, locals.var_qg_dn0, locals.var_qg_dn2, locals.var_qg_dn6, locals.var_qg_dn7, locals.var_qg_dn10, locals.var_qg_dn11, locals.var_qg_dn12, locals.var_qg_dn13, locals.var_qg_dn15, locals.var_qg_dn16, locals.var_qg_dn17, locals.var_qg_dn18,)
    }
};
        locals.var_qg = assign36820_e51404;
        locals.var_qg_dn0 = assign36820_e51404_d_n0;
        locals.var_qg_dn2 = assign36820_e51404_d_n2;
        locals.var_qg_dn6 = assign36820_e51404_d_n6;
        locals.var_qg_dn7 = assign36820_e51404_d_n7;
        locals.var_qg_dn10 = assign36820_e51404_d_n10;
        locals.var_qg_dn11 = assign36820_e51404_d_n11;
        locals.var_qg_dn12 = assign36820_e51404_d_n12;
        locals.var_qg_dn13 = assign36820_e51404_d_n13;
        locals.var_qg_dn15 = assign36820_e51404_d_n15;
        locals.var_qg_dn16 = assign36820_e51404_d_n16;
        locals.var_qg_dn17 = assign36820_e51404_d_n17;
        locals.var_qg_dn18 = assign36820_e51404_d_n18;
        locals.var_qg_rv = 0.0;

        let (assign36830_e51411, assign36830_e51411_d_n0, assign36830_e51411_d_n2, assign36830_e51411_d_n6, assign36830_e51411_d_n7, assign36830_e51411_d_n10, assign36830_e51411_d_n11, assign36830_e51411_d_n12, assign36830_e51411_d_n13, assign36830_e51411_d_n15, assign36830_e51411_d_n16, assign36830_e51411_d_n17, assign36830_e51411_d_n18,) = {
    if (locals.var_guard1214 == 0.0) {
        let assign36830_e51409: f64 = (locals.var_qse + locals.var_qs_nqs);
        (assign36830_e51409, (locals.var_qse_dn0 + locals.var_qs_nqs_dn0), (locals.var_qse_dn2 + locals.var_qs_nqs_dn2), (locals.var_qse_dn6 + locals.var_qs_nqs_dn6), (locals.var_qse_dn7 + locals.var_qs_nqs_dn7), (locals.var_qse_dn10 + locals.var_qs_nqs_dn10), (locals.var_qse_dn11 + locals.var_qs_nqs_dn11), (locals.var_qse_dn12 + locals.var_qs_nqs_dn12), locals.var_qse_dn13, locals.var_qse_dn15, (locals.var_qse_dn16 + locals.var_qs_nqs_dn16), (locals.var_qse_dn17 + locals.var_qs_nqs_dn17), (locals.var_qse_dn18 + locals.var_qs_nqs_dn18),)
    } else {
        (locals.var_qd, locals.var_qd_dn0, locals.var_qd_dn2, locals.var_qd_dn6, locals.var_qd_dn7, locals.var_qd_dn10, locals.var_qd_dn11, locals.var_qd_dn12, locals.var_qd_dn13, locals.var_qd_dn15, locals.var_qd_dn16, locals.var_qd_dn17, locals.var_qd_dn18,)
    }
};
        locals.var_qd = assign36830_e51411;
        locals.var_qd_dn0 = assign36830_e51411_d_n0;
        locals.var_qd_dn2 = assign36830_e51411_d_n2;
        locals.var_qd_dn6 = assign36830_e51411_d_n6;
        locals.var_qd_dn7 = assign36830_e51411_d_n7;
        locals.var_qd_dn10 = assign36830_e51411_d_n10;
        locals.var_qd_dn11 = assign36830_e51411_d_n11;
        locals.var_qd_dn12 = assign36830_e51411_d_n12;
        locals.var_qd_dn13 = assign36830_e51411_d_n13;
        locals.var_qd_dn15 = assign36830_e51411_d_n15;
        locals.var_qd_dn16 = assign36830_e51411_d_n16;
        locals.var_qd_dn17 = assign36830_e51411_d_n17;
        locals.var_qd_dn18 = assign36830_e51411_d_n18;
        locals.var_qd_rv = 0.0;

        let (assign36850_e51428, assign36850_e51428_d_n0, assign36850_e51428_d_n2, assign36850_e51428_d_n6, assign36850_e51428_d_n7, assign36850_e51428_d_n10, assign36850_e51428_d_n11, assign36850_e51428_d_n12, assign36850_e51428_d_n13, assign36850_e51428_d_n15, assign36850_e51428_d_n16, assign36850_e51428_d_n17, assign36850_e51428_d_n18,) = {
    if (locals.var_guard1214 == 0.0) {
        let assign36850_e51423: f64 = (locals.var_qge + locals.var_qde);
        let assign36850_e51425: f64 = (assign36850_e51423 + locals.var_qse);
        let assign36850_e51426: f64 = (-assign36850_e51425);
        (assign36850_e51426, (-((locals.var_qge_dn0 + locals.var_qde_dn0) + locals.var_qse_dn0)), (-((locals.var_qge_dn2 + locals.var_qde_dn2) + locals.var_qse_dn2)), (-((locals.var_qge_dn6 + locals.var_qde_dn6) + locals.var_qse_dn6)), (-((locals.var_qge_dn7 + locals.var_qde_dn7) + locals.var_qse_dn7)), (-((locals.var_qge_dn10 + locals.var_qde_dn10) + locals.var_qse_dn10)), (-((locals.var_qge_dn11 + locals.var_qde_dn11) + locals.var_qse_dn11)), (-((locals.var_qge_dn12 + locals.var_qde_dn12) + locals.var_qse_dn12)), (-((locals.var_qge_dn13 + locals.var_qde_dn13) + locals.var_qse_dn13)), (-((locals.var_qge_dn15 + locals.var_qde_dn15) + locals.var_qse_dn15)), (-((locals.var_qge_dn16 + locals.var_qde_dn16) + locals.var_qse_dn16)), (-((locals.var_qge_dn17 + locals.var_qde_dn17) + locals.var_qse_dn17)), (-((locals.var_qge_dn18 + locals.var_qde_dn18) + locals.var_qse_dn18)),)
    } else {
        (locals.var_qbe, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, locals.var_qbe_dn13, locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    }
};
        locals.var_qbe = assign36850_e51428;
        locals.var_qbe_dn0 = assign36850_e51428_d_n0;
        locals.var_qbe_dn2 = assign36850_e51428_d_n2;
        locals.var_qbe_dn6 = assign36850_e51428_d_n6;
        locals.var_qbe_dn7 = assign36850_e51428_d_n7;
        locals.var_qbe_dn10 = assign36850_e51428_d_n10;
        locals.var_qbe_dn11 = assign36850_e51428_d_n11;
        locals.var_qbe_dn12 = assign36850_e51428_d_n12;
        locals.var_qbe_dn13 = assign36850_e51428_d_n13;
        locals.var_qbe_dn15 = assign36850_e51428_d_n15;
        locals.var_qbe_dn16 = assign36850_e51428_d_n16;
        locals.var_qbe_dn17 = assign36850_e51428_d_n17;
        locals.var_qbe_dn18 = assign36850_e51428_d_n18;
        locals.var_qbe_rv = 0.0;

        let (assign36860_e51435, assign36860_e51435_d_n0, assign36860_e51435_d_n2, assign36860_e51435_d_n6, assign36860_e51435_d_n7, assign36860_e51435_d_n10, assign36860_e51435_d_n11, assign36860_e51435_d_n12, assign36860_e51435_d_n13, assign36860_e51435_d_n15, assign36860_e51435_d_n16, assign36860_e51435_d_n17, assign36860_e51435_d_n18,) = {
    if (locals.var_guard1214 == 0.0) {
        let assign36860_e51433: f64 = (locals.var_qbe + locals.var_qb_nqs);
        (assign36860_e51433, locals.var_qbe_dn0, locals.var_qbe_dn2, locals.var_qbe_dn6, locals.var_qbe_dn7, locals.var_qbe_dn10, locals.var_qbe_dn11, locals.var_qbe_dn12, (locals.var_qbe_dn13 + locals.var_qb_nqs_dn13), locals.var_qbe_dn15, locals.var_qbe_dn16, locals.var_qbe_dn17, locals.var_qbe_dn18,)
    } else {
        (locals.var_qb, locals.var_qb_dn0, locals.var_qb_dn2, locals.var_qb_dn6, locals.var_qb_dn7, locals.var_qb_dn10, locals.var_qb_dn11, locals.var_qb_dn12, locals.var_qb_dn13, locals.var_qb_dn15, locals.var_qb_dn16, locals.var_qb_dn17, locals.var_qb_dn18,)
    }
};
        locals.var_qb = assign36860_e51435;
        locals.var_qb_dn0 = assign36860_e51435_d_n0;
        locals.var_qb_dn2 = assign36860_e51435_d_n2;
        locals.var_qb_dn6 = assign36860_e51435_d_n6;
        locals.var_qb_dn7 = assign36860_e51435_d_n7;
        locals.var_qb_dn10 = assign36860_e51435_d_n10;
        locals.var_qb_dn11 = assign36860_e51435_d_n11;
        locals.var_qb_dn12 = assign36860_e51435_d_n12;
        locals.var_qb_dn13 = assign36860_e51435_d_n13;
        locals.var_qb_dn15 = assign36860_e51435_d_n15;
        locals.var_qb_dn16 = assign36860_e51435_d_n16;
        locals.var_qb_dn17 = assign36860_e51435_d_n17;
        locals.var_qb_dn18 = assign36860_e51435_d_n18;
        locals.var_qb_rv = 0.0;

        let assign36920_e51443: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1215 = assign36920_e51443;
        locals.var_guard1215_rv = 0.0;

        let (assign36930_e51447, assign36930_e51447_d_n0, assign36930_e51447_d_n2, assign36930_e51447_d_n6, assign36930_e51447_d_n7, assign36930_e51447_d_n10, assign36930_e51447_d_n11, assign36930_e51447_d_n12, assign36930_e51447_d_n17,) = {
    if (locals.var_guard1215 != 0.0) {
        (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17,)
    } else {
        (locals.var_ibd, locals.var_ibd_dn0, locals.var_ibd_dn2, locals.var_ibd_dn6, locals.var_ibd_dn7, locals.var_ibd_dn10, locals.var_ibd_dn11, locals.var_ibd_dn12, locals.var_ibd_dn17,)
    }
};
        locals.var_ibd = assign36930_e51447;
        locals.var_ibd_dn0 = assign36930_e51447_d_n0;
        locals.var_ibd_dn2 = assign36930_e51447_d_n2;
        locals.var_ibd_dn6 = assign36930_e51447_d_n6;
        locals.var_ibd_dn7 = assign36930_e51447_d_n7;
        locals.var_ibd_dn10 = assign36930_e51447_d_n10;
        locals.var_ibd_dn11 = assign36930_e51447_d_n11;
        locals.var_ibd_dn12 = assign36930_e51447_d_n12;
        locals.var_ibd_dn17 = assign36930_e51447_d_n17;
        locals.var_ibd_rv = 0.0;

        let (assign36940_e51451, assign36940_e51451_d_n0, assign36940_e51451_d_n2, assign36940_e51451_d_n6, assign36940_e51451_d_n7, assign36940_e51451_d_n10, assign36940_e51451_d_n11, assign36940_e51451_d_n12, assign36940_e51451_d_n17,) = {
    if (locals.var_guard1215 != 0.0) {
        (locals.var_qbd_s0, locals.var_qbd_s0_dn0, locals.var_qbd_s0_dn2, locals.var_qbd_s0_dn6, locals.var_qbd_s0_dn7, locals.var_qbd_s0_dn10, locals.var_qbd_s0_dn11, locals.var_qbd_s0_dn12, locals.var_qbd_s0_dn17,)
    } else {
        (locals.var_qbd, locals.var_qbd_dn0, locals.var_qbd_dn2, locals.var_qbd_dn6, locals.var_qbd_dn7, locals.var_qbd_dn10, locals.var_qbd_dn11, locals.var_qbd_dn12, locals.var_qbd_dn17,)
    }
};
        locals.var_qbd = assign36940_e51451;
        locals.var_qbd_dn0 = assign36940_e51451_d_n0;
        locals.var_qbd_dn2 = assign36940_e51451_d_n2;
        locals.var_qbd_dn6 = assign36940_e51451_d_n6;
        locals.var_qbd_dn7 = assign36940_e51451_d_n7;
        locals.var_qbd_dn10 = assign36940_e51451_d_n10;
        locals.var_qbd_dn11 = assign36940_e51451_d_n11;
        locals.var_qbd_dn12 = assign36940_e51451_d_n12;
        locals.var_qbd_dn17 = assign36940_e51451_d_n17;
        locals.var_qbd_rv = 0.0;

        let (assign36950_e51455, assign36950_e51455_d_n0, assign36950_e51455_d_n2, assign36950_e51455_d_n6, assign36950_e51455_d_n7, assign36950_e51455_d_n10, assign36950_e51455_d_n11, assign36950_e51455_d_n12, assign36950_e51455_d_n17,) = {
    if (locals.var_guard1215 != 0.0) {
        (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17,)
    } else {
        (locals.var_ibs, locals.var_ibs_dn0, locals.var_ibs_dn2, locals.var_ibs_dn6, locals.var_ibs_dn7, locals.var_ibs_dn10, locals.var_ibs_dn11, locals.var_ibs_dn12, locals.var_ibs_dn17,)
    }
};
        locals.var_ibs = assign36950_e51455;
        locals.var_ibs_dn0 = assign36950_e51455_d_n0;
        locals.var_ibs_dn2 = assign36950_e51455_d_n2;
        locals.var_ibs_dn6 = assign36950_e51455_d_n6;
        locals.var_ibs_dn7 = assign36950_e51455_d_n7;
        locals.var_ibs_dn10 = assign36950_e51455_d_n10;
        locals.var_ibs_dn11 = assign36950_e51455_d_n11;
        locals.var_ibs_dn12 = assign36950_e51455_d_n12;
        locals.var_ibs_dn17 = assign36950_e51455_d_n17;
        locals.var_ibs_rv = 0.0;

        let (assign36960_e51459, assign36960_e51459_d_n0, assign36960_e51459_d_n2, assign36960_e51459_d_n6, assign36960_e51459_d_n7, assign36960_e51459_d_n10, assign36960_e51459_d_n11, assign36960_e51459_d_n12, assign36960_e51459_d_n17,) = {
    if (locals.var_guard1215 != 0.0) {
        (locals.var_qbs_s0, locals.var_qbs_s0_dn0, locals.var_qbs_s0_dn2, locals.var_qbs_s0_dn6, locals.var_qbs_s0_dn7, locals.var_qbs_s0_dn10, locals.var_qbs_s0_dn11, locals.var_qbs_s0_dn12, locals.var_qbs_s0_dn17,)
    } else {
        (locals.var_qbs, locals.var_qbs_dn0, locals.var_qbs_dn2, locals.var_qbs_dn6, locals.var_qbs_dn7, locals.var_qbs_dn10, locals.var_qbs_dn11, locals.var_qbs_dn12, locals.var_qbs_dn17,)
    }
};
        locals.var_qbs = assign36960_e51459;
        locals.var_qbs_dn0 = assign36960_e51459_d_n0;
        locals.var_qbs_dn2 = assign36960_e51459_d_n2;
        locals.var_qbs_dn6 = assign36960_e51459_d_n6;
        locals.var_qbs_dn7 = assign36960_e51459_d_n7;
        locals.var_qbs_dn10 = assign36960_e51459_d_n10;
        locals.var_qbs_dn11 = assign36960_e51459_d_n11;
        locals.var_qbs_dn12 = assign36960_e51459_d_n12;
        locals.var_qbs_dn17 = assign36960_e51459_d_n17;
        locals.var_qbs_rv = 0.0;

        let assign36970_e51466: f64 = if ((p.p38 == 1.0) && (locals.var_mks_rth0 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1216 = assign36970_e51466;
        locals.var_guard1216_rv = 0.0;

        let (assign36990_e51476,) = {
    if (locals.var_guard1216 != 0.0) {
        (locals.var_cth,)
    } else {
        (locals.var_cthe,)
    }
};
        locals.var_cthe = assign36990_e51476;
        locals.var_cthe_rv = 0.0;

        let (assign37020_e51492,) = {
    if (locals.var_guard1216 == 0.0) {
        (0.0,)
    } else {
        (locals.var_cthe,)
    }
};
        locals.var_cthe = assign37020_e51492;
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

        let assign37190_e51546: f64 = locals.var_qg_dn6;
        locals.var_cgdbd = assign37190_e51546;
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

        let assign37200_e51549: f64 = (p.p50 * locals.var_cgdbd);
        locals.var_cgdbd = assign37200_e51549;
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

        let assign37210_e51552: f64 = locals.var_qg_dn7;
        locals.var_cgsbd = assign37210_e51552;
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

        let assign37220_e51555: f64 = (p.p50 * locals.var_cgsbd);
        locals.var_cgsbd = assign37220_e51555;
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

        let assign37490_e51636: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1218 = assign37490_e51636;
        locals.var_guard1218_rv = 0.0;

    }

    pub(super) fn stamp_reactive_block_130(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign37500_e51642, assign37500_e51642_d_n0, assign37500_e51642_d_n2, assign37500_e51642_d_n6, assign37500_e51642_d_n7, assign37500_e51642_d_n10, assign37500_e51642_d_n11, assign37500_e51642_d_n12, assign37500_e51642_d_n17,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign37500_e51640: f64 = (p.p50 * locals.var_ibd);
        (assign37500_e51640, (p.p50 * locals.var_ibd_dn0), (p.p50 * locals.var_ibd_dn2), (p.p50 * locals.var_ibd_dn6), (p.p50 * locals.var_ibd_dn7), (p.p50 * locals.var_ibd_dn10), (p.p50 * locals.var_ibd_dn11), (p.p50 * locals.var_ibd_dn12), (p.p50 * locals.var_ibd_dn17),)
    } else {
        (locals.var_ibdb, locals.var_ibdb_dn0, locals.var_ibdb_dn2, locals.var_ibdb_dn6, locals.var_ibdb_dn7, locals.var_ibdb_dn10, locals.var_ibdb_dn11, locals.var_ibdb_dn12, locals.var_ibdb_dn17,)
    }
};
        locals.var_ibdb = assign37500_e51642;
        locals.var_ibdb_dn0 = assign37500_e51642_d_n0;
        locals.var_ibdb_dn2 = assign37500_e51642_d_n2;
        locals.var_ibdb_dn6 = assign37500_e51642_d_n6;
        locals.var_ibdb_dn7 = assign37500_e51642_d_n7;
        locals.var_ibdb_dn10 = assign37500_e51642_d_n10;
        locals.var_ibdb_dn11 = assign37500_e51642_d_n11;
        locals.var_ibdb_dn12 = assign37500_e51642_d_n12;
        locals.var_ibdb_dn17 = assign37500_e51642_d_n17;
        locals.var_ibdb_rv = 0.0;

        let (assign37510_e51648, assign37510_e51648_d_n0, assign37510_e51648_d_n2, assign37510_e51648_d_n6, assign37510_e51648_d_n7, assign37510_e51648_d_n10, assign37510_e51648_d_n11, assign37510_e51648_d_n12, assign37510_e51648_d_n17,) = {
    if (locals.var_guard1218 != 0.0) {
        let assign37510_e51646: f64 = (p.p50 * locals.var_ibs);
        (assign37510_e51646, (p.p50 * locals.var_ibs_dn0), (p.p50 * locals.var_ibs_dn2), (p.p50 * locals.var_ibs_dn6), (p.p50 * locals.var_ibs_dn7), (p.p50 * locals.var_ibs_dn10), (p.p50 * locals.var_ibs_dn11), (p.p50 * locals.var_ibs_dn12), (p.p50 * locals.var_ibs_dn17),)
    } else {
        (locals.var_ibsb, locals.var_ibsb_dn0, locals.var_ibsb_dn2, locals.var_ibsb_dn6, locals.var_ibsb_dn7, locals.var_ibsb_dn10, locals.var_ibsb_dn11, locals.var_ibsb_dn12, locals.var_ibsb_dn17,)
    }
};
        locals.var_ibsb = assign37510_e51648;
        locals.var_ibsb_dn0 = assign37510_e51648_d_n0;
        locals.var_ibsb_dn2 = assign37510_e51648_d_n2;
        locals.var_ibsb_dn6 = assign37510_e51648_d_n6;
        locals.var_ibsb_dn7 = assign37510_e51648_d_n7;
        locals.var_ibsb_dn10 = assign37510_e51648_d_n10;
        locals.var_ibsb_dn11 = assign37510_e51648_d_n11;
        locals.var_ibsb_dn12 = assign37510_e51648_d_n12;
        locals.var_ibsb_dn17 = assign37510_e51648_d_n17;
        locals.var_ibsb_rv = 0.0;

        let assign37630_e51700: f64 = (4.0 * 1.3806226e-23);
        let assign37630_e51702: f64 = (assign37630_e51700 * locals.var_ttemp);
        let assign37630_e51704: f64 = assign37630_e51702;
        locals.var_whi_noise = assign37630_e51704;
        locals.var_whi_noise_dn10 = (assign37630_e51700 * locals.var_ttemp_dn10);
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

        let assign37660_e51711: f64 = (locals.var_whi_noise * locals.var_noithrml);
        locals.var_sid = assign37660_e51711;
        locals.var_sid_dn0 = (locals.var_whi_noise * locals.var_noithrml_dn0);
        locals.var_sid_dn2 = (locals.var_whi_noise * locals.var_noithrml_dn2);
        locals.var_sid_dn6 = (locals.var_whi_noise * locals.var_noithrml_dn6);
        locals.var_sid_dn7 = (locals.var_whi_noise * locals.var_noithrml_dn7);
        locals.var_sid_dn10 = ((locals.var_whi_noise_dn10 * locals.var_noithrml) + (locals.var_whi_noise * locals.var_noithrml_dn10));
        locals.var_sid_dn11 = (locals.var_whi_noise * locals.var_noithrml_dn11);
        locals.var_sid_dn12 = (locals.var_whi_noise * locals.var_noithrml_dn12);
        locals.var_sid_dn17 = (locals.var_whi_noise * locals.var_noithrml_dn17);
        locals.var_sid_rv = 0.0;

        let (assign37680_e51725, assign37680_e51725_d_n0, assign37680_e51725_d_n2, assign37680_e51725_d_n6, assign37680_e51725_d_n7, assign37680_e51725_d_n10, assign37680_e51725_d_n11, assign37680_e51725_d_n12, assign37680_e51725_d_n13, assign37680_e51725_d_n15, assign37680_e51725_d_n16, assign37680_e51725_d_n17, assign37680_e51725_d_n18,) = {
    if ((locals.var_sid > 0.0) && (locals.var_noiigate > 0.0)) {
        let assign37680_e51722: f64 = (locals.var_noiigate / locals.var_sid);
        let assign37680_e51723: f64 = (assign37680_e51722).sqrt();
        (assign37680_e51723, ((((locals.var_noiigate_dn0 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn0)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37680_e51723)), ((((locals.var_noiigate_dn2 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn2)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37680_e51723)), ((((locals.var_noiigate_dn6 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn6)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37680_e51723)), ((((locals.var_noiigate_dn7 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn7)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37680_e51723)), ((((locals.var_noiigate_dn10 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn10)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37680_e51723)), ((((locals.var_noiigate_dn11 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn11)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37680_e51723)), ((((locals.var_noiigate_dn12 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn12)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37680_e51723)), ((locals.var_noiigate_dn13 / locals.var_sid) / (2.0 * assign37680_e51723)), ((locals.var_noiigate_dn15 / locals.var_sid) / (2.0 * assign37680_e51723)), ((locals.var_noiigate_dn16 / locals.var_sid) / (2.0 * assign37680_e51723)), ((((locals.var_noiigate_dn17 * locals.var_sid) - (locals.var_noiigate * locals.var_sid_dn17)) / (locals.var_sid * locals.var_sid)) / (2.0 * assign37680_e51723)), ((locals.var_noiigate_dn18 / locals.var_sid) / (2.0 * assign37680_e51723)),)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        locals.var_sigrat = assign37680_e51725;
        locals.var_sigrat_dn0 = assign37680_e51725_d_n0;
        locals.var_sigrat_dn2 = assign37680_e51725_d_n2;
        locals.var_sigrat_dn6 = assign37680_e51725_d_n6;
        locals.var_sigrat_dn7 = assign37680_e51725_d_n7;
        locals.var_sigrat_dn10 = assign37680_e51725_d_n10;
        locals.var_sigrat_dn11 = assign37680_e51725_d_n11;
        locals.var_sigrat_dn12 = assign37680_e51725_d_n12;
        locals.var_sigrat_dn13 = assign37680_e51725_d_n13;
        locals.var_sigrat_dn15 = assign37680_e51725_d_n15;
        locals.var_sigrat_dn16 = assign37680_e51725_d_n16;
        locals.var_sigrat_dn17 = assign37680_e51725_d_n17;
        locals.var_sigrat_dn18 = assign37680_e51725_d_n18;
        locals.var_sigrat_rv = 0.0;

        let (assign37690_e51737, assign37690_e51737_d_n0, assign37690_e51737_d_n2, assign37690_e51737_d_n6, assign37690_e51737_d_n7, assign37690_e51737_d_n10, assign37690_e51737_d_n11, assign37690_e51737_d_n12, assign37690_e51737_d_n13, assign37690_e51737_d_n15, assign37690_e51737_d_n16, assign37690_e51737_d_n17, assign37690_e51737_d_n18,) = {
    if (locals.var_mode > 0.0) {
        let assign37690_e51732: f64 = (1.0 - locals.var_qdrat);
        let assign37690_e51733: f64 = (locals.var_sigrat * assign37690_e51732);
        (assign37690_e51733, ((locals.var_sigrat_dn0 * assign37690_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign37690_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn6 * assign37690_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign37690_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn10 * assign37690_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn11 * assign37690_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn11))), ((locals.var_sigrat_dn12 * assign37690_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn12))), (locals.var_sigrat_dn13 * assign37690_e51732), (locals.var_sigrat_dn15 * assign37690_e51732), (locals.var_sigrat_dn16 * assign37690_e51732), ((locals.var_sigrat_dn17 * assign37690_e51732) + (locals.var_sigrat * (-locals.var_qdrat_dn17))), (locals.var_sigrat_dn18 * assign37690_e51732),)
    } else {
        let assign37690_e51736: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign37690_e51736, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn11 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn11)), ((locals.var_sigrat_dn12 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn12)), (locals.var_sigrat_dn13 * locals.var_qdrat), (locals.var_sigrat_dn15 * locals.var_qdrat), (locals.var_sigrat_dn16 * locals.var_qdrat), ((locals.var_sigrat_dn17 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn17)), (locals.var_sigrat_dn18 * locals.var_qdrat),)
    }
};
        locals.var_sigrat_s = assign37690_e51737;
        locals.var_sigrat_s_dn0 = assign37690_e51737_d_n0;
        locals.var_sigrat_s_dn2 = assign37690_e51737_d_n2;
        locals.var_sigrat_s_dn6 = assign37690_e51737_d_n6;
        locals.var_sigrat_s_dn7 = assign37690_e51737_d_n7;
        locals.var_sigrat_s_dn10 = assign37690_e51737_d_n10;
        locals.var_sigrat_s_dn11 = assign37690_e51737_d_n11;
        locals.var_sigrat_s_dn12 = assign37690_e51737_d_n12;
        locals.var_sigrat_s_dn13 = assign37690_e51737_d_n13;
        locals.var_sigrat_s_dn15 = assign37690_e51737_d_n15;
        locals.var_sigrat_s_dn16 = assign37690_e51737_d_n16;
        locals.var_sigrat_s_dn17 = assign37690_e51737_d_n17;
        locals.var_sigrat_s_dn18 = assign37690_e51737_d_n18;
        locals.var_sigrat_s_rv = 0.0;

        let (assign37700_e51749, assign37700_e51749_d_n0, assign37700_e51749_d_n2, assign37700_e51749_d_n6, assign37700_e51749_d_n7, assign37700_e51749_d_n10, assign37700_e51749_d_n11, assign37700_e51749_d_n12, assign37700_e51749_d_n13, assign37700_e51749_d_n15, assign37700_e51749_d_n16, assign37700_e51749_d_n17, assign37700_e51749_d_n18,) = {
    if (locals.var_mode > 0.0) {
        let assign37700_e51743: f64 = (locals.var_sigrat * locals.var_qdrat);
        (assign37700_e51743, ((locals.var_sigrat_dn0 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn0)), ((locals.var_sigrat_dn2 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn2)), ((locals.var_sigrat_dn6 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn6)), ((locals.var_sigrat_dn7 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn7)), ((locals.var_sigrat_dn10 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn10)), ((locals.var_sigrat_dn11 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn11)), ((locals.var_sigrat_dn12 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn12)), (locals.var_sigrat_dn13 * locals.var_qdrat), (locals.var_sigrat_dn15 * locals.var_qdrat), (locals.var_sigrat_dn16 * locals.var_qdrat), ((locals.var_sigrat_dn17 * locals.var_qdrat) + (locals.var_sigrat * locals.var_qdrat_dn17)), (locals.var_sigrat_dn18 * locals.var_qdrat),)
    } else {
        let assign37700_e51747: f64 = (1.0 - locals.var_qdrat);
        let assign37700_e51748: f64 = (locals.var_sigrat * assign37700_e51747);
        (assign37700_e51748, ((locals.var_sigrat_dn0 * assign37700_e51747) + (locals.var_sigrat * (-locals.var_qdrat_dn0))), ((locals.var_sigrat_dn2 * assign37700_e51747) + (locals.var_sigrat * (-locals.var_qdrat_dn2))), ((locals.var_sigrat_dn6 * assign37700_e51747) + (locals.var_sigrat * (-locals.var_qdrat_dn6))), ((locals.var_sigrat_dn7 * assign37700_e51747) + (locals.var_sigrat * (-locals.var_qdrat_dn7))), ((locals.var_sigrat_dn10 * assign37700_e51747) + (locals.var_sigrat * (-locals.var_qdrat_dn10))), ((locals.var_sigrat_dn11 * assign37700_e51747) + (locals.var_sigrat * (-locals.var_qdrat_dn11))), ((locals.var_sigrat_dn12 * assign37700_e51747) + (locals.var_sigrat * (-locals.var_qdrat_dn12))), (locals.var_sigrat_dn13 * assign37700_e51747), (locals.var_sigrat_dn15 * assign37700_e51747), (locals.var_sigrat_dn16 * assign37700_e51747), ((locals.var_sigrat_dn17 * assign37700_e51747) + (locals.var_sigrat * (-locals.var_qdrat_dn17))), (locals.var_sigrat_dn18 * assign37700_e51747),)
    }
};
        locals.var_sigrat_d = assign37700_e51749;
        locals.var_sigrat_d_dn0 = assign37700_e51749_d_n0;
        locals.var_sigrat_d_dn2 = assign37700_e51749_d_n2;
        locals.var_sigrat_d_dn6 = assign37700_e51749_d_n6;
        locals.var_sigrat_d_dn7 = assign37700_e51749_d_n7;
        locals.var_sigrat_d_dn10 = assign37700_e51749_d_n10;
        locals.var_sigrat_d_dn11 = assign37700_e51749_d_n11;
        locals.var_sigrat_d_dn12 = assign37700_e51749_d_n12;
        locals.var_sigrat_d_dn13 = assign37700_e51749_d_n13;
        locals.var_sigrat_d_dn15 = assign37700_e51749_d_n15;
        locals.var_sigrat_d_dn16 = assign37700_e51749_d_n16;
        locals.var_sigrat_d_dn17 = assign37700_e51749_d_n17;
        locals.var_sigrat_d_dn18 = assign37700_e51749_d_n18;
        locals.var_sigrat_d_rv = 0.0;

        let assign37720_e51759: f64 = if ((p.p38 > 0.0) && (p.p242 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard1226 = assign37720_e51759;
        locals.var_guard1226_rv = 0.0;

        let assign37740_e51766: f64 = if p.p43 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard1227 = assign37740_e51766;
        locals.var_guard1227_rv = 0.0;

        let assign37750_e51775: f64 = if ((p.p37 != 0.0) || ((p.p25 == 1.0) && (p.p26 == 2.0))) { 1.0 } else { 0.0 };
        locals.var_guard1228 = assign37750_e51775;
        locals.var_guard1228_rv = 0.0;

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
        let (eq4_e330, eq4_e330_d_n0, eq4_e330_d_n2, eq4_e330_d_n6, eq4_e330_d_n7, eq4_e330_d_n10, eq4_e330_d_n11, eq4_e330_d_n12, eq4_e330_d_n17,) = {
    if (locals.var_guard1224 != 0.0) {
        let eq4_e328: f64 = (p.p50 * locals.var_igs);
        let eq4_e328_d_n0: f64 = (p.p50 * locals.var_igs_dn0);
        let eq4_e328_d_n2: f64 = (p.p50 * locals.var_igs_dn2);
        let eq4_e328_d_n6: f64 = (p.p50 * locals.var_igs_dn6);
        let eq4_e328_d_n7: f64 = (p.p50 * locals.var_igs_dn7);
        let eq4_e328_d_n10: f64 = (p.p50 * locals.var_igs_dn10);
        let eq4_e328_d_n11: f64 = (p.p50 * locals.var_igs_dn11);
        let eq4_e328_d_n12: f64 = (p.p50 * locals.var_igs_dn12);
        let eq4_e328_d_n17: f64 = (p.p50 * locals.var_igs_dn17);
        (eq4_e328, eq4_e328_d_n0, eq4_e328_d_n2, eq4_e328_d_n6, eq4_e328_d_n7, eq4_e328_d_n10, eq4_e328_d_n11, eq4_e328_d_n12, eq4_e328_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e330;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(7),
            multiplicity * (eq4_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq4_e330_d_n0), multiplicity * (eq4_e330_d_n2), multiplicity * (eq4_e330_d_n6), multiplicity * (eq4_e330_d_n7), multiplicity * (eq4_e330_d_n10), multiplicity * (eq4_e330_d_n11), multiplicity * (eq4_e330_d_n12), multiplicity * (eq4_e330_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq5_e336, eq5_e336_d_n0, eq5_e336_d_n2, eq5_e336_d_n6, eq5_e336_d_n7, eq5_e336_d_n10, eq5_e336_d_n11, eq5_e336_d_n12, eq5_e336_d_n17,) = {
    if (locals.var_guard1224 != 0.0) {
        let eq5_e334: f64 = (p.p50 * locals.var_igd);
        let eq5_e334_d_n0: f64 = (p.p50 * locals.var_igd_dn0);
        let eq5_e334_d_n2: f64 = (p.p50 * locals.var_igd_dn2);
        let eq5_e334_d_n6: f64 = (p.p50 * locals.var_igd_dn6);
        let eq5_e334_d_n7: f64 = (p.p50 * locals.var_igd_dn7);
        let eq5_e334_d_n10: f64 = (p.p50 * locals.var_igd_dn10);
        let eq5_e334_d_n11: f64 = (p.p50 * locals.var_igd_dn11);
        let eq5_e334_d_n12: f64 = (p.p50 * locals.var_igd_dn12);
        let eq5_e334_d_n17: f64 = (p.p50 * locals.var_igd_dn17);
        (eq5_e334, eq5_e334_d_n0, eq5_e334_d_n2, eq5_e334_d_n6, eq5_e334_d_n7, eq5_e334_d_n10, eq5_e334_d_n11, eq5_e334_d_n12, eq5_e334_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e336;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(6),
            multiplicity * (eq5_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq5_e336_d_n0), multiplicity * (eq5_e336_d_n2), multiplicity * (eq5_e336_d_n6), multiplicity * (eq5_e336_d_n7), multiplicity * (eq5_e336_d_n10), multiplicity * (eq5_e336_d_n11), multiplicity * (eq5_e336_d_n12), multiplicity * (eq5_e336_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq6_e342, eq6_e342_d_n0, eq6_e342_d_n2, eq6_e342_d_n6, eq6_e342_d_n7, eq6_e342_d_n10, eq6_e342_d_n11, eq6_e342_d_n12, eq6_e342_d_n17,) = {
    if (locals.var_guard1224 != 0.0) {
        let eq6_e340: f64 = (p.p50 * locals.var_igb);
        let eq6_e340_d_n0: f64 = (p.p50 * locals.var_igb_dn0);
        let eq6_e340_d_n2: f64 = (p.p50 * locals.var_igb_dn2);
        let eq6_e340_d_n6: f64 = (p.p50 * locals.var_igb_dn6);
        let eq6_e340_d_n7: f64 = (p.p50 * locals.var_igb_dn7);
        let eq6_e340_d_n10: f64 = (p.p50 * locals.var_igb_dn10);
        let eq6_e340_d_n11: f64 = (p.p50 * locals.var_igb_dn11);
        let eq6_e340_d_n12: f64 = (p.p50 * locals.var_igb_dn12);
        let eq6_e340_d_n17: f64 = (p.p50 * locals.var_igb_dn17);
        (eq6_e340, eq6_e340_d_n0, eq6_e340_d_n2, eq6_e340_d_n6, eq6_e340_d_n7, eq6_e340_d_n10, eq6_e340_d_n11, eq6_e340_d_n12, eq6_e340_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e342;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(11),
            Some(12),
            multiplicity * (eq6_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq6_e342_d_n0), multiplicity * (eq6_e342_d_n2), multiplicity * (eq6_e342_d_n6), multiplicity * (eq6_e342_d_n7), multiplicity * (eq6_e342_d_n10), multiplicity * (eq6_e342_d_n11), multiplicity * (eq6_e342_d_n12), multiplicity * (eq6_e342_d_n17)],
            [],
            [],
            1.0,
        );
        let eq11_e367: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 0, locals.var_qg);
        let eq11_e368: f64 = (p.p50 * eq11_e367);
        let eq11_e368_d_n0: f64 = (p.p50 * (locals.var_qg_dn0 * ddt_scale));
        let eq11_e368_d_n2: f64 = (p.p50 * (locals.var_qg_dn2 * ddt_scale));
        let eq11_e368_d_n6: f64 = (p.p50 * (locals.var_qg_dn6 * ddt_scale));
        let eq11_e368_d_n7: f64 = (p.p50 * (locals.var_qg_dn7 * ddt_scale));
        let eq11_e368_d_n10: f64 = (p.p50 * (locals.var_qg_dn10 * ddt_scale));
        let eq11_e368_d_n11: f64 = (p.p50 * (locals.var_qg_dn11 * ddt_scale));
        let eq11_e368_d_n12: f64 = (p.p50 * (locals.var_qg_dn12 * ddt_scale));
        let eq11_e368_d_n13: f64 = (p.p50 * (locals.var_qg_dn13 * ddt_scale));
        let eq11_e368_d_n15: f64 = (p.p50 * (locals.var_qg_dn15 * ddt_scale));
        let eq11_e368_d_n16: f64 = (p.p50 * (locals.var_qg_dn16 * ddt_scale));
        let eq11_e368_d_n17: f64 = (p.p50 * (locals.var_qg_dn17 * ddt_scale));
        let eq11_e368_d_n18: f64 = (p.p50 * (locals.var_qg_dn18 * ddt_scale));
        let eq11_value: f64 = eq11_e368;
        let eq11_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq11_node_derivatives: [f64; 12] = [eq11_e368_d_n0, eq11_e368_d_n2, eq11_e368_d_n6, eq11_e368_d_n7, eq11_e368_d_n10, eq11_e368_d_n11, eq11_e368_d_n12, eq11_e368_d_n13, eq11_e368_d_n15, eq11_e368_d_n16, eq11_e368_d_n17, eq11_e368_d_n18];
        let eq11_branch_derivative_indices: [usize; 0] = [];
        let eq11_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq11_value),
            &eq11_node_derivative_indices,
            &eq11_node_derivatives,
            &eq11_branch_derivative_indices,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e371: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 1, locals.var_qd);
        let eq12_e372: f64 = (p.p50 * eq12_e371);
        let eq12_e372_d_n0: f64 = (p.p50 * (locals.var_qd_dn0 * ddt_scale));
        let eq12_e372_d_n2: f64 = (p.p50 * (locals.var_qd_dn2 * ddt_scale));
        let eq12_e372_d_n6: f64 = (p.p50 * (locals.var_qd_dn6 * ddt_scale));
        let eq12_e372_d_n7: f64 = (p.p50 * (locals.var_qd_dn7 * ddt_scale));
        let eq12_e372_d_n10: f64 = (p.p50 * (locals.var_qd_dn10 * ddt_scale));
        let eq12_e372_d_n11: f64 = (p.p50 * (locals.var_qd_dn11 * ddt_scale));
        let eq12_e372_d_n12: f64 = (p.p50 * (locals.var_qd_dn12 * ddt_scale));
        let eq12_e372_d_n13: f64 = (p.p50 * (locals.var_qd_dn13 * ddt_scale));
        let eq12_e372_d_n15: f64 = (p.p50 * (locals.var_qd_dn15 * ddt_scale));
        let eq12_e372_d_n16: f64 = (p.p50 * (locals.var_qd_dn16 * ddt_scale));
        let eq12_e372_d_n17: f64 = (p.p50 * (locals.var_qd_dn17 * ddt_scale));
        let eq12_e372_d_n18: f64 = (p.p50 * (locals.var_qd_dn18 * ddt_scale));
        let eq12_value: f64 = eq12_e372;
        let eq12_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq12_node_derivatives: [f64; 12] = [eq12_e372_d_n0, eq12_e372_d_n2, eq12_e372_d_n6, eq12_e372_d_n7, eq12_e372_d_n10, eq12_e372_d_n11, eq12_e372_d_n12, eq12_e372_d_n13, eq12_e372_d_n15, eq12_e372_d_n16, eq12_e372_d_n17, eq12_e372_d_n18];
        let eq12_branch_derivative_indices: [usize; 0] = [];
        let eq12_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq12_value),
            &eq12_node_derivative_indices,
            &eq12_node_derivatives,
            &eq12_branch_derivative_indices,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e375: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 2, locals.var_qb);
        let eq13_e376: f64 = (p.p50 * eq13_e375);
        let eq13_e376_d_n0: f64 = (p.p50 * (locals.var_qb_dn0 * ddt_scale));
        let eq13_e376_d_n2: f64 = (p.p50 * (locals.var_qb_dn2 * ddt_scale));
        let eq13_e376_d_n6: f64 = (p.p50 * (locals.var_qb_dn6 * ddt_scale));
        let eq13_e376_d_n7: f64 = (p.p50 * (locals.var_qb_dn7 * ddt_scale));
        let eq13_e376_d_n10: f64 = (p.p50 * (locals.var_qb_dn10 * ddt_scale));
        let eq13_e376_d_n11: f64 = (p.p50 * (locals.var_qb_dn11 * ddt_scale));
        let eq13_e376_d_n12: f64 = (p.p50 * (locals.var_qb_dn12 * ddt_scale));
        let eq13_e376_d_n13: f64 = (p.p50 * (locals.var_qb_dn13 * ddt_scale));
        let eq13_e376_d_n15: f64 = (p.p50 * (locals.var_qb_dn15 * ddt_scale));
        let eq13_e376_d_n16: f64 = (p.p50 * (locals.var_qb_dn16 * ddt_scale));
        let eq13_e376_d_n17: f64 = (p.p50 * (locals.var_qb_dn17 * ddt_scale));
        let eq13_e376_d_n18: f64 = (p.p50 * (locals.var_qb_dn18 * ddt_scale));
        let eq13_value: f64 = eq13_e376;
        let eq13_node_derivative_indices: [usize; 12] = [0, 2, 6, 7, 10, 11, 12, 13, 15, 16, 17, 18];
        let eq13_node_derivatives: [f64; 12] = [eq13_e376_d_n0, eq13_e376_d_n2, eq13_e376_d_n6, eq13_e376_d_n7, eq13_e376_d_n10, eq13_e376_d_n11, eq13_e376_d_n12, eq13_e376_d_n13, eq13_e376_d_n15, eq13_e376_d_n16, eq13_e376_d_n17, eq13_e376_d_n18];
        let eq13_branch_derivative_indices: [usize; 0] = [];
        let eq13_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(12),
            Some(7),
            multiplicity * (eq13_value),
            &eq13_node_derivative_indices,
            &eq13_node_derivatives,
            &eq13_branch_derivative_indices,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq18_e402: f64 = (locals.var_ci * (nv14 - 0.0));
        let eq18_e402_d_n0: f64 = (locals.var_ci_dn0 * (nv14 - 0.0));
        let eq18_e402_d_n2: f64 = (locals.var_ci_dn2 * (nv14 - 0.0));
        let eq18_e402_d_n6: f64 = (locals.var_ci_dn6 * (nv14 - 0.0));
        let eq18_e402_d_n7: f64 = (locals.var_ci_dn7 * (nv14 - 0.0));
        let eq18_e402_d_n10: f64 = (locals.var_ci_dn10 * (nv14 - 0.0));
        let eq18_e402_d_n11: f64 = (locals.var_ci_dn11 * (nv14 - 0.0));
        let eq18_e402_d_n12: f64 = (locals.var_ci_dn12 * (nv14 - 0.0));
        let eq18_e402_d_n17: f64 = (locals.var_ci_dn17 * (nv14 - 0.0));
        let eq18_value: f64 = eq18_e402;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq18_value),
            [0, 2, 6, 7, 10, 11, 12, 14, 17],
            [multiplicity * (eq18_e402_d_n0), multiplicity * (eq18_e402_d_n2), multiplicity * (eq18_e402_d_n6), multiplicity * (eq18_e402_d_n7), multiplicity * (eq18_e402_d_n10), multiplicity * (eq18_e402_d_n11), multiplicity * (eq18_e402_d_n12), multiplicity * (locals.var_ci), multiplicity * (eq18_e402_d_n17)],
            [],
            [],
            1.0,
        );
        let eq19_e405: f64 = ((nv14 - 0.0) * locals.var_sigrat_s);
        let eq19_e405_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn0);
        let eq19_e405_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn2);
        let eq19_e405_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn6);
        let eq19_e405_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn7);
        let eq19_e405_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn10);
        let eq19_e405_d_n11: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn11);
        let eq19_e405_d_n12: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn12);
        let eq19_e405_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn13);
        let eq19_e405_d_n15: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn15);
        let eq19_e405_d_n16: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn16);
        let eq19_e405_d_n17: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn17);
        let eq19_e405_d_n18: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn18);
        let eq19_e406: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 3, eq19_e405);
        let eq19_value: f64 = eq19_e406;
        let eq19_node_derivative_indices: [usize; 13] = [0, 2, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18];
        let eq19_node_derivatives: [f64; 13] = [(eq19_e405_d_n0 * ddt_scale), (eq19_e405_d_n2 * ddt_scale), (eq19_e405_d_n6 * ddt_scale), (eq19_e405_d_n7 * ddt_scale), (eq19_e405_d_n10 * ddt_scale), (eq19_e405_d_n11 * ddt_scale), (eq19_e405_d_n12 * ddt_scale), (eq19_e405_d_n13 * ddt_scale), (locals.var_sigrat_s * ddt_scale), (eq19_e405_d_n15 * ddt_scale), (eq19_e405_d_n16 * ddt_scale), (eq19_e405_d_n17 * ddt_scale), (eq19_e405_d_n18 * ddt_scale)];
        let eq19_branch_derivative_indices: [usize; 0] = [];
        let eq19_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq19_value),
            &eq19_node_derivative_indices,
            &eq19_node_derivatives,
            &eq19_branch_derivative_indices,
            &eq19_branch_derivatives,
            multiplicity,
        );
        let eq20_e409: f64 = ((nv14 - 0.0) * locals.var_sigrat_d);
        let eq20_e409_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn0);
        let eq20_e409_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn2);
        let eq20_e409_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn6);
        let eq20_e409_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn7);
        let eq20_e409_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn10);
        let eq20_e409_d_n11: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn11);
        let eq20_e409_d_n12: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn12);
        let eq20_e409_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn13);
        let eq20_e409_d_n15: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn15);
        let eq20_e409_d_n16: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn16);
        let eq20_e409_d_n17: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn17);
        let eq20_e409_d_n18: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn18);
        let eq20_e410: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 4, eq20_e409);
        let eq20_value: f64 = eq20_e410;
        let eq20_node_derivative_indices: [usize; 13] = [0, 2, 6, 7, 10, 11, 12, 13, 14, 15, 16, 17, 18];
        let eq20_node_derivatives: [f64; 13] = [(eq20_e409_d_n0 * ddt_scale), (eq20_e409_d_n2 * ddt_scale), (eq20_e409_d_n6 * ddt_scale), (eq20_e409_d_n7 * ddt_scale), (eq20_e409_d_n10 * ddt_scale), (eq20_e409_d_n11 * ddt_scale), (eq20_e409_d_n12 * ddt_scale), (eq20_e409_d_n13 * ddt_scale), (locals.var_sigrat_d * ddt_scale), (eq20_e409_d_n15 * ddt_scale), (eq20_e409_d_n16 * ddt_scale), (eq20_e409_d_n17 * ddt_scale), (eq20_e409_d_n18 * ddt_scale)];
        let eq20_branch_derivative_indices: [usize; 0] = [];
        let eq20_branch_derivatives: [f64; 0] = [];
        stamper.stamp_current_indexed_dense_local(
            Some(11),
            Some(6),
            multiplicity * (eq20_value),
            &eq20_node_derivative_indices,
            &eq20_node_derivatives,
            &eq20_branch_derivative_indices,
            &eq20_branch_derivatives,
            multiplicity,
        );
        let (eq26_e462, eq26_e462_d_n1, eq26_e462_d_n11,) = {
    if (p.p35 != 0.0) {
        let eq26_e460: f64 = (locals.var_grg * (nv1 - nv11));
        (eq26_e460, locals.var_grg, (-locals.var_grg),)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        let eq26_value: f64 = eq26_e462;
        stamper.stamp_current_node2_local(
            Some(1),
            Some(11),
            multiplicity * (eq26_value),
            1,
            multiplicity * (eq26_e462_d_n1),
            11,
            multiplicity * (eq26_e462_d_n11),
        );
        let (eq28_e473, eq28_e473_d_n10,) = {
    if (locals.var_guard1226 != 0.0) {
        let eq28_e471: f64 = ((nv10 - 0.0) * locals.var_gth);
        (eq28_e471, locals.var_gth,)
    } else {
        (0.0, 0.0,)
    }
};
        let eq28_value: f64 = eq28_e473;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq28_value),
            10,
            multiplicity * (eq28_e473_d_n10),
        );
        let (eq31_e491, eq31_e491_d_n10,) = {
    if (locals.var_guard1226 != 0.0) {
        let eq31_e488: f64 = (locals.var_cthe * (nv10 - 0.0));
        let eq31_e489: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 5, eq31_e488);
        (eq31_e489, (locals.var_cthe * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq31_value: f64 = eq31_e491;
        stamper.stamp_current_node1_local(
            Some(10),
            None,
            multiplicity * (eq31_value),
            10,
            multiplicity * (eq31_e491_d_n10),
        );
        let (eq33_e506, eq33_e506_d_n0, eq33_e506_d_n2, eq33_e506_d_n6, eq33_e506_d_n7, eq33_e506_d_n10, eq33_e506_d_n11, eq33_e506_d_n12, eq33_e506_d_n17,) = {
    if (locals.var_guard1227 != 0.0) {
        let eq33_e503: f64 = (locals.var_igidl + locals.var_isub);
        let eq33_e503_d_n0: f64 = (locals.var_igidl_dn0 + locals.var_isub_dn0);
        let eq33_e503_d_n2: f64 = (locals.var_igidl_dn2 + locals.var_isub_dn2);
        let eq33_e503_d_n6: f64 = (locals.var_igidl_dn6 + locals.var_isub_dn6);
        let eq33_e503_d_n7: f64 = (locals.var_igidl_dn7 + locals.var_isub_dn7);
        let eq33_e503_d_n10: f64 = (locals.var_igidl_dn10 + locals.var_isub_dn10);
        let eq33_e503_d_n11: f64 = (locals.var_igidl_dn11 + locals.var_isub_dn11);
        let eq33_e503_d_n12: f64 = (locals.var_igidl_dn12 + locals.var_isub_dn12);
        let eq33_e503_d_n17: f64 = (locals.var_igidl_dn17 + locals.var_isub_dn17);
        let eq33_e504: f64 = (p.p50 * eq33_e503);
        let eq33_e504_d_n0: f64 = (p.p50 * eq33_e503_d_n0);
        let eq33_e504_d_n2: f64 = (p.p50 * eq33_e503_d_n2);
        let eq33_e504_d_n6: f64 = (p.p50 * eq33_e503_d_n6);
        let eq33_e504_d_n7: f64 = (p.p50 * eq33_e503_d_n7);
        let eq33_e504_d_n10: f64 = (p.p50 * eq33_e503_d_n10);
        let eq33_e504_d_n11: f64 = (p.p50 * eq33_e503_d_n11);
        let eq33_e504_d_n12: f64 = (p.p50 * eq33_e503_d_n12);
        let eq33_e504_d_n17: f64 = (p.p50 * eq33_e503_d_n17);
        (eq33_e504, eq33_e504_d_n0, eq33_e504_d_n2, eq33_e504_d_n6, eq33_e504_d_n7, eq33_e504_d_n10, eq33_e504_d_n11, eq33_e504_d_n12, eq33_e504_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e506;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(12),
            multiplicity * (eq33_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq33_e506_d_n0), multiplicity * (eq33_e506_d_n2), multiplicity * (eq33_e506_d_n6), multiplicity * (eq33_e506_d_n7), multiplicity * (eq33_e506_d_n10), multiplicity * (eq33_e506_d_n11), multiplicity * (eq33_e506_d_n12), multiplicity * (eq33_e506_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq34_e514, eq34_e514_d_n0, eq34_e514_d_n2, eq34_e514_d_n6, eq34_e514_d_n7, eq34_e514_d_n10, eq34_e514_d_n11, eq34_e514_d_n12, eq34_e514_d_n17,) = {
    if (locals.var_guard1227 != 0.0) {
        let eq34_e511: f64 = (locals.var_igisl + locals.var_isubs);
        let eq34_e511_d_n0: f64 = (locals.var_igisl_dn0 + locals.var_isubs_dn0);
        let eq34_e511_d_n2: f64 = (locals.var_igisl_dn2 + locals.var_isubs_dn2);
        let eq34_e511_d_n6: f64 = (locals.var_igisl_dn6 + locals.var_isubs_dn6);
        let eq34_e511_d_n7: f64 = (locals.var_igisl_dn7 + locals.var_isubs_dn7);
        let eq34_e511_d_n10: f64 = (locals.var_igisl_dn10 + locals.var_isubs_dn10);
        let eq34_e511_d_n11: f64 = (locals.var_igisl_dn11 + locals.var_isubs_dn11);
        let eq34_e511_d_n12: f64 = (locals.var_igisl_dn12 + locals.var_isubs_dn12);
        let eq34_e511_d_n17: f64 = (locals.var_igisl_dn17 + locals.var_isubs_dn17);
        let eq34_e512: f64 = (p.p50 * eq34_e511);
        let eq34_e512_d_n0: f64 = (p.p50 * eq34_e511_d_n0);
        let eq34_e512_d_n2: f64 = (p.p50 * eq34_e511_d_n2);
        let eq34_e512_d_n6: f64 = (p.p50 * eq34_e511_d_n6);
        let eq34_e512_d_n7: f64 = (p.p50 * eq34_e511_d_n7);
        let eq34_e512_d_n10: f64 = (p.p50 * eq34_e511_d_n10);
        let eq34_e512_d_n11: f64 = (p.p50 * eq34_e511_d_n11);
        let eq34_e512_d_n12: f64 = (p.p50 * eq34_e511_d_n12);
        let eq34_e512_d_n17: f64 = (p.p50 * eq34_e511_d_n17);
        (eq34_e512, eq34_e512_d_n0, eq34_e512_d_n2, eq34_e512_d_n6, eq34_e512_d_n7, eq34_e512_d_n10, eq34_e512_d_n11, eq34_e512_d_n12, eq34_e512_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq34_value: f64 = eq34_e514;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(12),
            multiplicity * (eq34_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq34_e514_d_n0), multiplicity * (eq34_e514_d_n2), multiplicity * (eq34_e514_d_n6), multiplicity * (eq34_e514_d_n7), multiplicity * (eq34_e514_d_n10), multiplicity * (eq34_e514_d_n11), multiplicity * (eq34_e514_d_n12), multiplicity * (eq34_e514_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq35_e523, eq35_e523_d_n0, eq35_e523_d_n2, eq35_e523_d_n6, eq35_e523_d_n7, eq35_e523_d_n10, eq35_e523_d_n11, eq35_e523_d_n12, eq35_e523_d_n17,) = {
    if (locals.var_guard1227 != 0.0) {
        let eq35_e519: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 6, locals.var_qbs);
        let eq35_e520: f64 = (locals.var_ibs + eq35_e519);
        let eq35_e520_d_n0: f64 = (locals.var_ibs_dn0 + (locals.var_qbs_dn0 * ddt_scale));
        let eq35_e520_d_n2: f64 = (locals.var_ibs_dn2 + (locals.var_qbs_dn2 * ddt_scale));
        let eq35_e520_d_n6: f64 = (locals.var_ibs_dn6 + (locals.var_qbs_dn6 * ddt_scale));
        let eq35_e520_d_n7: f64 = (locals.var_ibs_dn7 + (locals.var_qbs_dn7 * ddt_scale));
        let eq35_e520_d_n10: f64 = (locals.var_ibs_dn10 + (locals.var_qbs_dn10 * ddt_scale));
        let eq35_e520_d_n11: f64 = (locals.var_ibs_dn11 + (locals.var_qbs_dn11 * ddt_scale));
        let eq35_e520_d_n12: f64 = (locals.var_ibs_dn12 + (locals.var_qbs_dn12 * ddt_scale));
        let eq35_e520_d_n17: f64 = (locals.var_ibs_dn17 + (locals.var_qbs_dn17 * ddt_scale));
        let eq35_e521: f64 = (p.p50 * eq35_e520);
        let eq35_e521_d_n0: f64 = (p.p50 * eq35_e520_d_n0);
        let eq35_e521_d_n2: f64 = (p.p50 * eq35_e520_d_n2);
        let eq35_e521_d_n6: f64 = (p.p50 * eq35_e520_d_n6);
        let eq35_e521_d_n7: f64 = (p.p50 * eq35_e520_d_n7);
        let eq35_e521_d_n10: f64 = (p.p50 * eq35_e520_d_n10);
        let eq35_e521_d_n11: f64 = (p.p50 * eq35_e520_d_n11);
        let eq35_e521_d_n12: f64 = (p.p50 * eq35_e520_d_n12);
        let eq35_e521_d_n17: f64 = (p.p50 * eq35_e520_d_n17);
        (eq35_e521, eq35_e521_d_n0, eq35_e521_d_n2, eq35_e521_d_n6, eq35_e521_d_n7, eq35_e521_d_n10, eq35_e521_d_n11, eq35_e521_d_n12, eq35_e521_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_value: f64 = eq35_e523;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(9),
            Some(7),
            multiplicity * (eq35_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq35_e523_d_n0), multiplicity * (eq35_e523_d_n2), multiplicity * (eq35_e523_d_n6), multiplicity * (eq35_e523_d_n7), multiplicity * (eq35_e523_d_n10), multiplicity * (eq35_e523_d_n11), multiplicity * (eq35_e523_d_n12), multiplicity * (eq35_e523_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq36_e532, eq36_e532_d_n0, eq36_e532_d_n2, eq36_e532_d_n6, eq36_e532_d_n7, eq36_e532_d_n10, eq36_e532_d_n11, eq36_e532_d_n12, eq36_e532_d_n17,) = {
    if (locals.var_guard1227 != 0.0) {
        let eq36_e528: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 7, locals.var_qbd);
        let eq36_e529: f64 = (locals.var_ibd + eq36_e528);
        let eq36_e529_d_n0: f64 = (locals.var_ibd_dn0 + (locals.var_qbd_dn0 * ddt_scale));
        let eq36_e529_d_n2: f64 = (locals.var_ibd_dn2 + (locals.var_qbd_dn2 * ddt_scale));
        let eq36_e529_d_n6: f64 = (locals.var_ibd_dn6 + (locals.var_qbd_dn6 * ddt_scale));
        let eq36_e529_d_n7: f64 = (locals.var_ibd_dn7 + (locals.var_qbd_dn7 * ddt_scale));
        let eq36_e529_d_n10: f64 = (locals.var_ibd_dn10 + (locals.var_qbd_dn10 * ddt_scale));
        let eq36_e529_d_n11: f64 = (locals.var_ibd_dn11 + (locals.var_qbd_dn11 * ddt_scale));
        let eq36_e529_d_n12: f64 = (locals.var_ibd_dn12 + (locals.var_qbd_dn12 * ddt_scale));
        let eq36_e529_d_n17: f64 = (locals.var_ibd_dn17 + (locals.var_qbd_dn17 * ddt_scale));
        let eq36_e530: f64 = (p.p50 * eq36_e529);
        let eq36_e530_d_n0: f64 = (p.p50 * eq36_e529_d_n0);
        let eq36_e530_d_n2: f64 = (p.p50 * eq36_e529_d_n2);
        let eq36_e530_d_n6: f64 = (p.p50 * eq36_e529_d_n6);
        let eq36_e530_d_n7: f64 = (p.p50 * eq36_e529_d_n7);
        let eq36_e530_d_n10: f64 = (p.p50 * eq36_e529_d_n10);
        let eq36_e530_d_n11: f64 = (p.p50 * eq36_e529_d_n11);
        let eq36_e530_d_n12: f64 = (p.p50 * eq36_e529_d_n12);
        let eq36_e530_d_n17: f64 = (p.p50 * eq36_e529_d_n17);
        (eq36_e530, eq36_e530_d_n0, eq36_e530_d_n2, eq36_e530_d_n6, eq36_e530_d_n7, eq36_e530_d_n10, eq36_e530_d_n11, eq36_e530_d_n12, eq36_e530_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_value: f64 = eq36_e532;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(8),
            Some(6),
            multiplicity * (eq36_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq36_e532_d_n0), multiplicity * (eq36_e532_d_n2), multiplicity * (eq36_e532_d_n6), multiplicity * (eq36_e532_d_n7), multiplicity * (eq36_e532_d_n10), multiplicity * (eq36_e532_d_n11), multiplicity * (eq36_e532_d_n12), multiplicity * (eq36_e532_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq37_e540, eq37_e540_d_n0, eq37_e540_d_n2, eq37_e540_d_n4, eq37_e540_d_n6, eq37_e540_d_n7, eq37_e540_d_n10, eq37_e540_d_n11, eq37_e540_d_n12, eq37_e540_d_n17,) = {
    if ((locals.var_guard1227 != 0.0) && (p.p261 != 0.0)) {
        let eq37_e538: f64 = ((nv4 - nv12) / locals.var_rbulk);
        let eq37_e538_d_n0: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn0) / (locals.var_rbulk * locals.var_rbulk)));
        let eq37_e538_d_n2: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn2) / (locals.var_rbulk * locals.var_rbulk)));
        let eq37_e538_d_n4: f64 = (1.0 / locals.var_rbulk);
        let eq37_e538_d_n6: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn6) / (locals.var_rbulk * locals.var_rbulk)));
        let eq37_e538_d_n7: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn7) / (locals.var_rbulk * locals.var_rbulk)));
        let eq37_e538_d_n10: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn10) / (locals.var_rbulk * locals.var_rbulk)));
        let eq37_e538_d_n11: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn11) / (locals.var_rbulk * locals.var_rbulk)));
        let eq37_e538_d_n12: f64 = (((-locals.var_rbulk) - ((nv4 - nv12) * locals.var_rbulk_dn12)) / (locals.var_rbulk * locals.var_rbulk));
        let eq37_e538_d_n17: f64 = (-(((nv4 - nv12) * locals.var_rbulk_dn17) / (locals.var_rbulk * locals.var_rbulk)));
        (eq37_e538, eq37_e538_d_n0, eq37_e538_d_n2, eq37_e538_d_n4, eq37_e538_d_n6, eq37_e538_d_n7, eq37_e538_d_n10, eq37_e538_d_n11, eq37_e538_d_n12, eq37_e538_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq37_value: f64 = eq37_e540;
        stamper.stamp_current_sparse_local::<9, 0>(
            Some(4),
            Some(12),
            multiplicity * (eq37_value),
            [0, 2, 4, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq37_e540_d_n0), multiplicity * (eq37_e540_d_n2), multiplicity * (eq37_e540_d_n4), multiplicity * (eq37_e540_d_n6), multiplicity * (eq37_e540_d_n7), multiplicity * (eq37_e540_d_n10), multiplicity * (eq37_e540_d_n11), multiplicity * (eq37_e540_d_n12), multiplicity * (eq37_e540_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq47_e616, eq47_e616_d_n18,) = {
    if ((locals.var_guard1227 != 0.0) && (p.p34 != 0.0)) {
        let eq47_e611: f64 = (1e-9 / 0.0001);
        let eq47_e613: f64 = (eq47_e611 * (nv18 - 0.0));
        let eq47_e614: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 8, eq47_e613);
        (eq47_e614, (eq47_e611 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq47_value: f64 = eq47_e616;
        stamper.stamp_current_node1_local(
            Some(18),
            None,
            multiplicity * (eq47_value),
            18,
            multiplicity * (eq47_e616_d_n18),
        );
        let (eq48_e627, eq48_e627_d_n13,) = {
    if ((locals.var_guard1227 != 0.0) && (p.p34 != 0.0)) {
        let eq48_e622: f64 = (1e-9 / 0.0001);
        let eq48_e624: f64 = (eq48_e622 * (nv13 - 0.0));
        let eq48_e625: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 9, eq48_e624);
        (eq48_e625, (eq48_e622 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq48_value: f64 = eq48_e627;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq48_value),
            13,
            multiplicity * (eq48_e627_d_n13),
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
        let (eq53_e666, eq53_e666_d_n17,) = {
    if ((locals.var_guard1227 != 0.0) && (locals.var_guard1228 != 0.0)) {
        let eq53_e661: f64 = (1e-9 / 0.0001);
        let eq53_e663: f64 = (eq53_e661 * (nv17 - 0.0));
        let eq53_e664: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 10, eq53_e663);
        (eq53_e664, (eq53_e661 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq53_value: f64 = eq53_e666;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq53_value),
            17,
            multiplicity * (eq53_e666_d_n17),
        );
        let (eq55_e682, eq55_e682_d_n0, eq55_e682_d_n2, eq55_e682_d_n6, eq55_e682_d_n7, eq55_e682_d_n10, eq55_e682_d_n11, eq55_e682_d_n12, eq55_e682_d_n17,) = {
    if (locals.var_guard1227 == 0.0) {
        let eq55_e679: f64 = (locals.var_igidl + locals.var_isub);
        let eq55_e679_d_n0: f64 = (locals.var_igidl_dn0 + locals.var_isub_dn0);
        let eq55_e679_d_n2: f64 = (locals.var_igidl_dn2 + locals.var_isub_dn2);
        let eq55_e679_d_n6: f64 = (locals.var_igidl_dn6 + locals.var_isub_dn6);
        let eq55_e679_d_n7: f64 = (locals.var_igidl_dn7 + locals.var_isub_dn7);
        let eq55_e679_d_n10: f64 = (locals.var_igidl_dn10 + locals.var_isub_dn10);
        let eq55_e679_d_n11: f64 = (locals.var_igidl_dn11 + locals.var_isub_dn11);
        let eq55_e679_d_n12: f64 = (locals.var_igidl_dn12 + locals.var_isub_dn12);
        let eq55_e679_d_n17: f64 = (locals.var_igidl_dn17 + locals.var_isub_dn17);
        let eq55_e680: f64 = (p.p50 * eq55_e679);
        let eq55_e680_d_n0: f64 = (p.p50 * eq55_e679_d_n0);
        let eq55_e680_d_n2: f64 = (p.p50 * eq55_e679_d_n2);
        let eq55_e680_d_n6: f64 = (p.p50 * eq55_e679_d_n6);
        let eq55_e680_d_n7: f64 = (p.p50 * eq55_e679_d_n7);
        let eq55_e680_d_n10: f64 = (p.p50 * eq55_e679_d_n10);
        let eq55_e680_d_n11: f64 = (p.p50 * eq55_e679_d_n11);
        let eq55_e680_d_n12: f64 = (p.p50 * eq55_e679_d_n12);
        let eq55_e680_d_n17: f64 = (p.p50 * eq55_e679_d_n17);
        (eq55_e680, eq55_e680_d_n0, eq55_e680_d_n2, eq55_e680_d_n6, eq55_e680_d_n7, eq55_e680_d_n10, eq55_e680_d_n11, eq55_e680_d_n12, eq55_e680_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq55_value: f64 = eq55_e682;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(6),
            Some(7),
            multiplicity * (eq55_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq55_e682_d_n0), multiplicity * (eq55_e682_d_n2), multiplicity * (eq55_e682_d_n6), multiplicity * (eq55_e682_d_n7), multiplicity * (eq55_e682_d_n10), multiplicity * (eq55_e682_d_n11), multiplicity * (eq55_e682_d_n12), multiplicity * (eq55_e682_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq56_e691, eq56_e691_d_n0, eq56_e691_d_n2, eq56_e691_d_n6, eq56_e691_d_n7, eq56_e691_d_n10, eq56_e691_d_n11, eq56_e691_d_n12, eq56_e691_d_n17,) = {
    if (locals.var_guard1227 == 0.0) {
        let eq56_e688: f64 = (locals.var_igisl + locals.var_isubs);
        let eq56_e688_d_n0: f64 = (locals.var_igisl_dn0 + locals.var_isubs_dn0);
        let eq56_e688_d_n2: f64 = (locals.var_igisl_dn2 + locals.var_isubs_dn2);
        let eq56_e688_d_n6: f64 = (locals.var_igisl_dn6 + locals.var_isubs_dn6);
        let eq56_e688_d_n7: f64 = (locals.var_igisl_dn7 + locals.var_isubs_dn7);
        let eq56_e688_d_n10: f64 = (locals.var_igisl_dn10 + locals.var_isubs_dn10);
        let eq56_e688_d_n11: f64 = (locals.var_igisl_dn11 + locals.var_isubs_dn11);
        let eq56_e688_d_n12: f64 = (locals.var_igisl_dn12 + locals.var_isubs_dn12);
        let eq56_e688_d_n17: f64 = (locals.var_igisl_dn17 + locals.var_isubs_dn17);
        let eq56_e689: f64 = (p.p50 * eq56_e688);
        let eq56_e689_d_n0: f64 = (p.p50 * eq56_e688_d_n0);
        let eq56_e689_d_n2: f64 = (p.p50 * eq56_e688_d_n2);
        let eq56_e689_d_n6: f64 = (p.p50 * eq56_e688_d_n6);
        let eq56_e689_d_n7: f64 = (p.p50 * eq56_e688_d_n7);
        let eq56_e689_d_n10: f64 = (p.p50 * eq56_e688_d_n10);
        let eq56_e689_d_n11: f64 = (p.p50 * eq56_e688_d_n11);
        let eq56_e689_d_n12: f64 = (p.p50 * eq56_e688_d_n12);
        let eq56_e689_d_n17: f64 = (p.p50 * eq56_e688_d_n17);
        (eq56_e689, eq56_e689_d_n0, eq56_e689_d_n2, eq56_e689_d_n6, eq56_e689_d_n7, eq56_e689_d_n10, eq56_e689_d_n11, eq56_e689_d_n12, eq56_e689_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq56_value: f64 = eq56_e691;
        stamper.stamp_current_sparse_local::<8, 0>(
            Some(7),
            Some(6),
            multiplicity * (eq56_value),
            [0, 2, 6, 7, 10, 11, 12, 17],
            [multiplicity * (eq56_e691_d_n0), multiplicity * (eq56_e691_d_n2), multiplicity * (eq56_e691_d_n6), multiplicity * (eq56_e691_d_n7), multiplicity * (eq56_e691_d_n10), multiplicity * (eq56_e691_d_n11), multiplicity * (eq56_e691_d_n12), multiplicity * (eq56_e691_d_n17)],
            [],
            [],
            1.0,
        );
        let (eq60_e724, eq60_e724_d_n17,) = {
    if ((locals.var_guard1227 == 0.0) && (p.p37 != 0.0)) {
        let eq60_e719: f64 = (1e-9 / 0.0001);
        let eq60_e721: f64 = (eq60_e719 * (nv17 - 0.0));
        let eq60_e722: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 11, eq60_e721);
        (eq60_e722, (eq60_e719 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq60_value: f64 = eq60_e724;
        stamper.stamp_current_node1_local(
            Some(17),
            None,
            multiplicity * (eq60_value),
            17,
            multiplicity * (eq60_e724_d_n17),
        );
        let (eq68_e792, eq68_e792_d_n15,) = {
    if ((locals.var_guard1227 == 0.0) && (p.p34 != 0.0)) {
        let eq68_e787: f64 = (1e-9 / 0.0001);
        let eq68_e789: f64 = (eq68_e787 * (nv15 - 0.0));
        let eq68_e790: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 12, eq68_e789);
        (eq68_e790, (eq68_e787 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq68_value: f64 = eq68_e792;
        stamper.stamp_current_node1_local(
            Some(15),
            None,
            multiplicity * (eq68_value),
            15,
            multiplicity * (eq68_e792_d_n15),
        );
        let (eq69_e804, eq69_e804_d_n16,) = {
    if ((locals.var_guard1227 == 0.0) && (p.p34 != 0.0)) {
        let eq69_e799: f64 = (1e-9 / 0.0001);
        let eq69_e801: f64 = (eq69_e799 * (nv16 - 0.0));
        let eq69_e802: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 13, eq69_e801);
        (eq69_e802, (eq69_e799 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq69_value: f64 = eq69_e804;
        stamper.stamp_current_node1_local(
            Some(16),
            None,
            multiplicity * (eq69_value),
            16,
            multiplicity * (eq69_e804_d_n16),
        );
        let (eq70_e816, eq70_e816_d_n13,) = {
    if ((locals.var_guard1227 == 0.0) && (p.p34 != 0.0)) {
        let eq70_e811: f64 = (1e-9 / 0.0001);
        let eq70_e813: f64 = (eq70_e811 * (nv13 - 0.0));
        let eq70_e814: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_older, ddt_state_initialized, ddt_derivative_current, ddt_derivative_previous, ddt_active, ddt_scale, ddt_previous_value_scale, ddt_older_value_scale, ddt_previous_derivative_scale, 14, eq70_e813);
        (eq70_e814, (eq70_e811 * ddt_scale),)
    } else {
        (0.0, 0.0,)
    }
};
        let eq70_value: f64 = eq70_e816;
        stamper.stamp_current_node1_local(
            Some(13),
            None,
            multiplicity * (eq70_value),
            13,
            multiplicity * (eq70_e816_d_n13),
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
        let eq11_e367_q: f64 = locals.var_qg;
        let eq11_e368: f64 = (p.p50 * locals.var_qg);
        let eq11_e368_d_n0: f64 = (p.p50 * locals.var_qg_dn0);
        let eq11_e368_d_n2: f64 = (p.p50 * locals.var_qg_dn2);
        let eq11_e368_d_n6: f64 = (p.p50 * locals.var_qg_dn6);
        let eq11_e368_d_n7: f64 = (p.p50 * locals.var_qg_dn7);
        let eq11_e368_d_n10: f64 = (p.p50 * locals.var_qg_dn10);
        let eq11_e368_d_n11: f64 = (p.p50 * locals.var_qg_dn11);
        let eq11_e368_d_n12: f64 = (p.p50 * locals.var_qg_dn12);
        let eq11_e368_d_n13: f64 = (p.p50 * locals.var_qg_dn13);
        let eq11_e368_d_n15: f64 = (p.p50 * locals.var_qg_dn15);
        let eq11_e368_d_n16: f64 = (p.p50 * locals.var_qg_dn16);
        let eq11_e368_d_n17: f64 = (p.p50 * locals.var_qg_dn17);
        let eq11_e368_d_n18: f64 = (p.p50 * locals.var_qg_dn18);
        let eq11_e368_q: f64 = (p.p50 * eq11_e367_q);
        let eq11_reactive_node_derivatives: [f64; 19] = [eq11_e368_d_n0, 0.0, eq11_e368_d_n2, 0.0, 0.0, 0.0, eq11_e368_d_n6, eq11_e368_d_n7, 0.0, 0.0, eq11_e368_d_n10, eq11_e368_d_n11, eq11_e368_d_n12, eq11_e368_d_n13, 0.0, eq11_e368_d_n15, eq11_e368_d_n16, eq11_e368_d_n17, eq11_e368_d_n18];
        let eq11_reactive_branch_derivatives: [f64; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq11_reactive_node_derivatives,
            branches,
            &eq11_reactive_branch_derivatives,
            multiplicity,
        );
        let eq12_e371_q: f64 = locals.var_qd;
        let eq12_e372: f64 = (p.p50 * locals.var_qd);
        let eq12_e372_d_n0: f64 = (p.p50 * locals.var_qd_dn0);
        let eq12_e372_d_n2: f64 = (p.p50 * locals.var_qd_dn2);
        let eq12_e372_d_n6: f64 = (p.p50 * locals.var_qd_dn6);
        let eq12_e372_d_n7: f64 = (p.p50 * locals.var_qd_dn7);
        let eq12_e372_d_n10: f64 = (p.p50 * locals.var_qd_dn10);
        let eq12_e372_d_n11: f64 = (p.p50 * locals.var_qd_dn11);
        let eq12_e372_d_n12: f64 = (p.p50 * locals.var_qd_dn12);
        let eq12_e372_d_n13: f64 = (p.p50 * locals.var_qd_dn13);
        let eq12_e372_d_n15: f64 = (p.p50 * locals.var_qd_dn15);
        let eq12_e372_d_n16: f64 = (p.p50 * locals.var_qd_dn16);
        let eq12_e372_d_n17: f64 = (p.p50 * locals.var_qd_dn17);
        let eq12_e372_d_n18: f64 = (p.p50 * locals.var_qd_dn18);
        let eq12_e372_q: f64 = (p.p50 * eq12_e371_q);
        let eq12_reactive_node_derivatives: [f64; 19] = [eq12_e372_d_n0, 0.0, eq12_e372_d_n2, 0.0, 0.0, 0.0, eq12_e372_d_n6, eq12_e372_d_n7, 0.0, 0.0, eq12_e372_d_n10, eq12_e372_d_n11, eq12_e372_d_n12, eq12_e372_d_n13, 0.0, eq12_e372_d_n15, eq12_e372_d_n16, eq12_e372_d_n17, eq12_e372_d_n18];
        let eq12_reactive_branch_derivatives: [f64; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq12_reactive_node_derivatives,
            branches,
            &eq12_reactive_branch_derivatives,
            multiplicity,
        );
        let eq13_e375_q: f64 = locals.var_qb;
        let eq13_e376: f64 = (p.p50 * locals.var_qb);
        let eq13_e376_d_n0: f64 = (p.p50 * locals.var_qb_dn0);
        let eq13_e376_d_n2: f64 = (p.p50 * locals.var_qb_dn2);
        let eq13_e376_d_n6: f64 = (p.p50 * locals.var_qb_dn6);
        let eq13_e376_d_n7: f64 = (p.p50 * locals.var_qb_dn7);
        let eq13_e376_d_n10: f64 = (p.p50 * locals.var_qb_dn10);
        let eq13_e376_d_n11: f64 = (p.p50 * locals.var_qb_dn11);
        let eq13_e376_d_n12: f64 = (p.p50 * locals.var_qb_dn12);
        let eq13_e376_d_n13: f64 = (p.p50 * locals.var_qb_dn13);
        let eq13_e376_d_n15: f64 = (p.p50 * locals.var_qb_dn15);
        let eq13_e376_d_n16: f64 = (p.p50 * locals.var_qb_dn16);
        let eq13_e376_d_n17: f64 = (p.p50 * locals.var_qb_dn17);
        let eq13_e376_d_n18: f64 = (p.p50 * locals.var_qb_dn18);
        let eq13_e376_q: f64 = (p.p50 * eq13_e375_q);
        let eq13_reactive_node_derivatives: [f64; 19] = [eq13_e376_d_n0, 0.0, eq13_e376_d_n2, 0.0, 0.0, 0.0, eq13_e376_d_n6, eq13_e376_d_n7, 0.0, 0.0, eq13_e376_d_n10, eq13_e376_d_n11, eq13_e376_d_n12, eq13_e376_d_n13, 0.0, eq13_e376_d_n15, eq13_e376_d_n16, eq13_e376_d_n17, eq13_e376_d_n18];
        let eq13_reactive_branch_derivatives: [f64; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[7]),
            nodes,
            &eq13_reactive_node_derivatives,
            branches,
            &eq13_reactive_branch_derivatives,
            multiplicity,
        );
        let eq19_e405: f64 = ((nv14 - 0.0) * locals.var_sigrat_s);
        let eq19_e405_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn0);
        let eq19_e405_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn2);
        let eq19_e405_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn6);
        let eq19_e405_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn7);
        let eq19_e405_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn10);
        let eq19_e405_d_n11: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn11);
        let eq19_e405_d_n12: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn12);
        let eq19_e405_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn13);
        let eq19_e405_d_n15: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn15);
        let eq19_e405_d_n16: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn16);
        let eq19_e405_d_n17: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn17);
        let eq19_e405_d_n18: f64 = ((nv14 - 0.0) * locals.var_sigrat_s_dn18);
        let eq19_e406_q: f64 = eq19_e405;
        let eq19_reactive_node_derivatives: [f64; 19] = [eq19_e405_d_n0, 0.0, eq19_e405_d_n2, 0.0, 0.0, 0.0, eq19_e405_d_n6, eq19_e405_d_n7, 0.0, 0.0, eq19_e405_d_n10, eq19_e405_d_n11, eq19_e405_d_n12, eq19_e405_d_n13, locals.var_sigrat_s, eq19_e405_d_n15, eq19_e405_d_n16, eq19_e405_d_n17, eq19_e405_d_n18];
        let eq19_reactive_branch_derivatives: [f64; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq19_reactive_node_derivatives,
            branches,
            &eq19_reactive_branch_derivatives,
            multiplicity,
        );
        let eq20_e409: f64 = ((nv14 - 0.0) * locals.var_sigrat_d);
        let eq20_e409_d_n0: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn0);
        let eq20_e409_d_n2: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn2);
        let eq20_e409_d_n6: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn6);
        let eq20_e409_d_n7: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn7);
        let eq20_e409_d_n10: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn10);
        let eq20_e409_d_n11: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn11);
        let eq20_e409_d_n12: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn12);
        let eq20_e409_d_n13: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn13);
        let eq20_e409_d_n15: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn15);
        let eq20_e409_d_n16: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn16);
        let eq20_e409_d_n17: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn17);
        let eq20_e409_d_n18: f64 = ((nv14 - 0.0) * locals.var_sigrat_d_dn18);
        let eq20_e410_q: f64 = eq20_e409;
        let eq20_reactive_node_derivatives: [f64; 19] = [eq20_e409_d_n0, 0.0, eq20_e409_d_n2, 0.0, 0.0, 0.0, eq20_e409_d_n6, eq20_e409_d_n7, 0.0, 0.0, eq20_e409_d_n10, eq20_e409_d_n11, eq20_e409_d_n12, eq20_e409_d_n13, locals.var_sigrat_d, eq20_e409_d_n15, eq20_e409_d_n16, eq20_e409_d_n17, eq20_e409_d_n18];
        let eq20_reactive_branch_derivatives: [f64; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[6]),
            nodes,
            &eq20_reactive_node_derivatives,
            branches,
            &eq20_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq31_e491, eq31_e491_d_n10, eq31_e491_q,) = {
    if (locals.var_guard1226 != 0.0) {
        let eq31_e488: f64 = (locals.var_cthe * (nv10 - 0.0));
        let eq31_e489_q: f64 = eq31_e488;
        (eq31_e488, locals.var_cthe, eq31_e489_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[10]),
            None,
            nodes[10],
            multiplicity * (eq31_e491_d_n10),
        );
        let (eq35_e523, eq35_e523_d_n0, eq35_e523_d_n2, eq35_e523_d_n6, eq35_e523_d_n7, eq35_e523_d_n10, eq35_e523_d_n11, eq35_e523_d_n12, eq35_e523_d_n17, eq35_e523_q, eq35_e523_q_d_n0, eq35_e523_q_d_n2, eq35_e523_q_d_n6, eq35_e523_q_d_n7, eq35_e523_q_d_n10, eq35_e523_q_d_n11, eq35_e523_q_d_n12, eq35_e523_q_d_n17,) = {
    if (locals.var_guard1227 != 0.0) {
        let eq35_e519_q: f64 = locals.var_qbs;
        let eq35_e520: f64 = (locals.var_ibs + locals.var_qbs);
        let eq35_e520_d_n0: f64 = (locals.var_ibs_dn0 + locals.var_qbs_dn0);
        let eq35_e520_d_n2: f64 = (locals.var_ibs_dn2 + locals.var_qbs_dn2);
        let eq35_e520_d_n6: f64 = (locals.var_ibs_dn6 + locals.var_qbs_dn6);
        let eq35_e520_d_n7: f64 = (locals.var_ibs_dn7 + locals.var_qbs_dn7);
        let eq35_e520_d_n10: f64 = (locals.var_ibs_dn10 + locals.var_qbs_dn10);
        let eq35_e520_d_n11: f64 = (locals.var_ibs_dn11 + locals.var_qbs_dn11);
        let eq35_e520_d_n12: f64 = (locals.var_ibs_dn12 + locals.var_qbs_dn12);
        let eq35_e520_d_n17: f64 = (locals.var_ibs_dn17 + locals.var_qbs_dn17);
        let eq35_e520_q: f64 = eq35_e519_q;
        let eq35_e521: f64 = (p.p50 * eq35_e520);
        let eq35_e521_d_n0: f64 = (p.p50 * eq35_e520_d_n0);
        let eq35_e521_d_n2: f64 = (p.p50 * eq35_e520_d_n2);
        let eq35_e521_d_n6: f64 = (p.p50 * eq35_e520_d_n6);
        let eq35_e521_d_n7: f64 = (p.p50 * eq35_e520_d_n7);
        let eq35_e521_d_n10: f64 = (p.p50 * eq35_e520_d_n10);
        let eq35_e521_d_n11: f64 = (p.p50 * eq35_e520_d_n11);
        let eq35_e521_d_n12: f64 = (p.p50 * eq35_e520_d_n12);
        let eq35_e521_d_n17: f64 = (p.p50 * eq35_e520_d_n17);
        let eq35_e521_q: f64 = (p.p50 * eq35_e520_q);
        let eq35_e521_q_d_n0: f64 = (p.p50 * locals.var_qbs_dn0);
        let eq35_e521_q_d_n2: f64 = (p.p50 * locals.var_qbs_dn2);
        let eq35_e521_q_d_n6: f64 = (p.p50 * locals.var_qbs_dn6);
        let eq35_e521_q_d_n7: f64 = (p.p50 * locals.var_qbs_dn7);
        let eq35_e521_q_d_n10: f64 = (p.p50 * locals.var_qbs_dn10);
        let eq35_e521_q_d_n11: f64 = (p.p50 * locals.var_qbs_dn11);
        let eq35_e521_q_d_n12: f64 = (p.p50 * locals.var_qbs_dn12);
        let eq35_e521_q_d_n17: f64 = (p.p50 * locals.var_qbs_dn17);
        (eq35_e521, eq35_e521_d_n0, eq35_e521_d_n2, eq35_e521_d_n6, eq35_e521_d_n7, eq35_e521_d_n10, eq35_e521_d_n11, eq35_e521_d_n12, eq35_e521_d_n17, eq35_e521_q, eq35_e521_q_d_n0, eq35_e521_q_d_n2, eq35_e521_q_d_n6, eq35_e521_q_d_n7, eq35_e521_q_d_n10, eq35_e521_q_d_n11, eq35_e521_q_d_n12, eq35_e521_q_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq35_reactive_node_derivatives: [f64; 19] = [eq35_e523_q_d_n0, 0.0, eq35_e523_q_d_n2, 0.0, 0.0, 0.0, eq35_e523_q_d_n6, eq35_e523_q_d_n7, 0.0, 0.0, eq35_e523_q_d_n10, eq35_e523_q_d_n11, eq35_e523_q_d_n12, 0.0, 0.0, 0.0, 0.0, eq35_e523_q_d_n17, 0.0];
        let eq35_reactive_branch_derivatives: [f64; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq35_reactive_node_derivatives,
            branches,
            &eq35_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq36_e532, eq36_e532_d_n0, eq36_e532_d_n2, eq36_e532_d_n6, eq36_e532_d_n7, eq36_e532_d_n10, eq36_e532_d_n11, eq36_e532_d_n12, eq36_e532_d_n17, eq36_e532_q, eq36_e532_q_d_n0, eq36_e532_q_d_n2, eq36_e532_q_d_n6, eq36_e532_q_d_n7, eq36_e532_q_d_n10, eq36_e532_q_d_n11, eq36_e532_q_d_n12, eq36_e532_q_d_n17,) = {
    if (locals.var_guard1227 != 0.0) {
        let eq36_e528_q: f64 = locals.var_qbd;
        let eq36_e529: f64 = (locals.var_ibd + locals.var_qbd);
        let eq36_e529_d_n0: f64 = (locals.var_ibd_dn0 + locals.var_qbd_dn0);
        let eq36_e529_d_n2: f64 = (locals.var_ibd_dn2 + locals.var_qbd_dn2);
        let eq36_e529_d_n6: f64 = (locals.var_ibd_dn6 + locals.var_qbd_dn6);
        let eq36_e529_d_n7: f64 = (locals.var_ibd_dn7 + locals.var_qbd_dn7);
        let eq36_e529_d_n10: f64 = (locals.var_ibd_dn10 + locals.var_qbd_dn10);
        let eq36_e529_d_n11: f64 = (locals.var_ibd_dn11 + locals.var_qbd_dn11);
        let eq36_e529_d_n12: f64 = (locals.var_ibd_dn12 + locals.var_qbd_dn12);
        let eq36_e529_d_n17: f64 = (locals.var_ibd_dn17 + locals.var_qbd_dn17);
        let eq36_e529_q: f64 = eq36_e528_q;
        let eq36_e530: f64 = (p.p50 * eq36_e529);
        let eq36_e530_d_n0: f64 = (p.p50 * eq36_e529_d_n0);
        let eq36_e530_d_n2: f64 = (p.p50 * eq36_e529_d_n2);
        let eq36_e530_d_n6: f64 = (p.p50 * eq36_e529_d_n6);
        let eq36_e530_d_n7: f64 = (p.p50 * eq36_e529_d_n7);
        let eq36_e530_d_n10: f64 = (p.p50 * eq36_e529_d_n10);
        let eq36_e530_d_n11: f64 = (p.p50 * eq36_e529_d_n11);
        let eq36_e530_d_n12: f64 = (p.p50 * eq36_e529_d_n12);
        let eq36_e530_d_n17: f64 = (p.p50 * eq36_e529_d_n17);
        let eq36_e530_q: f64 = (p.p50 * eq36_e529_q);
        let eq36_e530_q_d_n0: f64 = (p.p50 * locals.var_qbd_dn0);
        let eq36_e530_q_d_n2: f64 = (p.p50 * locals.var_qbd_dn2);
        let eq36_e530_q_d_n6: f64 = (p.p50 * locals.var_qbd_dn6);
        let eq36_e530_q_d_n7: f64 = (p.p50 * locals.var_qbd_dn7);
        let eq36_e530_q_d_n10: f64 = (p.p50 * locals.var_qbd_dn10);
        let eq36_e530_q_d_n11: f64 = (p.p50 * locals.var_qbd_dn11);
        let eq36_e530_q_d_n12: f64 = (p.p50 * locals.var_qbd_dn12);
        let eq36_e530_q_d_n17: f64 = (p.p50 * locals.var_qbd_dn17);
        (eq36_e530, eq36_e530_d_n0, eq36_e530_d_n2, eq36_e530_d_n6, eq36_e530_d_n7, eq36_e530_d_n10, eq36_e530_d_n11, eq36_e530_d_n12, eq36_e530_d_n17, eq36_e530_q, eq36_e530_q_d_n0, eq36_e530_q_d_n2, eq36_e530_q_d_n6, eq36_e530_q_d_n7, eq36_e530_q_d_n10, eq36_e530_q_d_n11, eq36_e530_q_d_n12, eq36_e530_q_d_n17,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq36_reactive_node_derivatives: [f64; 19] = [eq36_e532_q_d_n0, 0.0, eq36_e532_q_d_n2, 0.0, 0.0, 0.0, eq36_e532_q_d_n6, eq36_e532_q_d_n7, 0.0, 0.0, eq36_e532_q_d_n10, eq36_e532_q_d_n11, eq36_e532_q_d_n12, 0.0, 0.0, 0.0, 0.0, eq36_e532_q_d_n17, 0.0];
        let eq36_reactive_branch_derivatives: [f64; 16] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq36_reactive_node_derivatives,
            branches,
            &eq36_reactive_branch_derivatives,
            multiplicity,
        );
        let (eq47_e616, eq47_e616_d_n18, eq47_e616_q,) = {
    if ((locals.var_guard1227 != 0.0) && (p.p34 != 0.0)) {
        let eq47_e611: f64 = (1e-9 / 0.0001);
        let eq47_e613: f64 = (eq47_e611 * (nv18 - 0.0));
        let eq47_e614_q: f64 = eq47_e613;
        (eq47_e613, eq47_e611, eq47_e614_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[18]),
            None,
            nodes[18],
            multiplicity * (eq47_e616_d_n18),
        );
        let (eq48_e627, eq48_e627_d_n13, eq48_e627_q,) = {
    if ((locals.var_guard1227 != 0.0) && (p.p34 != 0.0)) {
        let eq48_e622: f64 = (1e-9 / 0.0001);
        let eq48_e624: f64 = (eq48_e622 * (nv13 - 0.0));
        let eq48_e625_q: f64 = eq48_e624;
        (eq48_e624, eq48_e622, eq48_e625_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq48_e627_d_n13),
        );
        let (eq53_e666, eq53_e666_d_n17, eq53_e666_q,) = {
    if ((locals.var_guard1227 != 0.0) && (locals.var_guard1228 != 0.0)) {
        let eq53_e661: f64 = (1e-9 / 0.0001);
        let eq53_e663: f64 = (eq53_e661 * (nv17 - 0.0));
        let eq53_e664_q: f64 = eq53_e663;
        (eq53_e663, eq53_e661, eq53_e664_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq53_e666_d_n17),
        );
        let (eq60_e724, eq60_e724_d_n17, eq60_e724_q,) = {
    if ((locals.var_guard1227 == 0.0) && (p.p37 != 0.0)) {
        let eq60_e719: f64 = (1e-9 / 0.0001);
        let eq60_e721: f64 = (eq60_e719 * (nv17 - 0.0));
        let eq60_e722_q: f64 = eq60_e721;
        (eq60_e721, eq60_e719, eq60_e722_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[17]),
            None,
            nodes[17],
            multiplicity * (eq60_e724_d_n17),
        );
        let (eq68_e792, eq68_e792_d_n15, eq68_e792_q,) = {
    if ((locals.var_guard1227 == 0.0) && (p.p34 != 0.0)) {
        let eq68_e787: f64 = (1e-9 / 0.0001);
        let eq68_e789: f64 = (eq68_e787 * (nv15 - 0.0));
        let eq68_e790_q: f64 = eq68_e789;
        (eq68_e789, eq68_e787, eq68_e790_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[15]),
            None,
            nodes[15],
            multiplicity * (eq68_e792_d_n15),
        );
        let (eq69_e804, eq69_e804_d_n16, eq69_e804_q,) = {
    if ((locals.var_guard1227 == 0.0) && (p.p34 != 0.0)) {
        let eq69_e799: f64 = (1e-9 / 0.0001);
        let eq69_e801: f64 = (eq69_e799 * (nv16 - 0.0));
        let eq69_e802_q: f64 = eq69_e801;
        (eq69_e801, eq69_e799, eq69_e802_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[16]),
            None,
            nodes[16],
            multiplicity * (eq69_e804_d_n16),
        );
        let (eq70_e816, eq70_e816_d_n13, eq70_e816_q,) = {
    if ((locals.var_guard1227 == 0.0) && (p.p34 != 0.0)) {
        let eq70_e811: f64 = (1e-9 / 0.0001);
        let eq70_e813: f64 = (eq70_e811 * (nv13 - 0.0));
        let eq70_e814_q: f64 = eq70_e813;
        (eq70_e813, eq70_e811, eq70_e814_q,)
    } else {
        (0.0, 0.0, 0.0,)
    }
};
        stamper.stamp_current_reactive_node1(
            Some(nodes[13]),
            None,
            nodes[13],
            multiplicity * (eq70_e816_d_n13),
        );
    }
}

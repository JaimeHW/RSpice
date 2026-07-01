#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_10(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard36: f64,
        var_iae: f64,
        var_ile: f64,
        var_ile2: f64,
        var_iwe: f64,
        var_le: f64,
        var_we_edge: f64,
        var_betnedge_p_slot: &mut f64,
        var_betnedge_p_rv_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cf_p_rv_slot: &mut f64,
        var_cfb_p_slot: &mut f64,
        var_cfb_p_rv_slot: &mut f64,
        var_cfbedge_p_slot: &mut f64,
        var_cfbedge_p_rv_slot: &mut f64,
        var_cfd_p_slot: &mut f64,
        var_cfd_p_rv_slot: &mut f64,
        var_cfdedge_p_slot: &mut f64,
        var_cfdedge_p_rv_slot: &mut f64,
        var_cfedge_p_slot: &mut f64,
        var_cfedge_p_rv_slot: &mut f64,
        var_ct_p_slot: &mut f64,
        var_ct_p_rv_slot: &mut f64,
        var_ctb_p_slot: &mut f64,
        var_ctb_p_rv_slot: &mut f64,
        var_ctg_p_slot: &mut f64,
        var_ctg_p_rv_slot: &mut f64,
        var_cth_p_slot: &mut f64,
        var_cth_p_rv_slot: &mut f64,
        var_dphib_p_slot: &mut f64,
        var_dphib_p_rv_slot: &mut f64,
        var_gfacnud_p_slot: &mut f64,
        var_gfacnud_p_rv_slot: &mut f64,
        var_gpe_edge_slot: &mut f64,
        var_gpe_edge_rv_slot: &mut f64,
        var_guard51_slot: &mut f64,
        var_guard51_rv_slot: &mut f64,
        var_guard52_slot: &mut f64,
        var_guard52_rv_slot: &mut f64,
        var_guard53_slot: &mut f64,
        var_guard53_rv_slot: &mut f64,
        var_guard54_slot: &mut f64,
        var_guard54_rv_slot: &mut f64,
        var_guard55_slot: &mut f64,
        var_guard55_rv_slot: &mut f64,
        var_guard56_slot: &mut f64,
        var_guard56_rv_slot: &mut f64,
        var_guard57_slot: &mut f64,
        var_guard57_rv_slot: &mut f64,
        var_guard58_slot: &mut f64,
        var_guard58_rv_slot: &mut f64,
        var_guard59_slot: &mut f64,
        var_guard59_rv_slot: &mut f64,
        var_guard60_slot: &mut f64,
        var_guard60_rv_slot: &mut f64,
        var_guard61_slot: &mut f64,
        var_guard61_rv_slot: &mut f64,
        var_guard62_slot: &mut f64,
        var_guard62_rv_slot: &mut f64,
        var_guard63_slot: &mut f64,
        var_guard63_rv_slot: &mut f64,
        var_guard64_slot: &mut f64,
        var_guard64_rv_slot: &mut f64,
        var_guard65_slot: &mut f64,
        var_guard65_rv_slot: &mut f64,
        var_guard66_slot: &mut f64,
        var_guard66_rv_slot: &mut f64,
        var_guard67_slot: &mut f64,
        var_guard67_rv_slot: &mut f64,
        var_kuowe_slot: &mut f64,
        var_kuowe_rv_slot: &mut f64,
        var_kvthowe_slot: &mut f64,
        var_kvthowe_rv_slot: &mut f64,
        var_neff_p_slot: &mut f64,
        var_neff_p_rv_slot: &mut f64,
        var_nov_p_slot: &mut f64,
        var_nov_p_rv_slot: &mut f64,
        var_novd_p_slot: &mut f64,
        var_novd_p_rv_slot: &mut f64,
        var_np_p_slot: &mut f64,
        var_np_p_rv_slot: &mut f64,
        var_psce_p_slot: &mut f64,
        var_psce_p_rv_slot: &mut f64,
        var_pscebedge_p_slot: &mut f64,
        var_pscebedge_p_rv_slot: &mut f64,
        var_pscededge_p_slot: &mut f64,
        var_pscededge_p_rv_slot: &mut f64,
        var_psceedge_p_slot: &mut f64,
        var_psceedge_p_rv_slot: &mut f64,
        var_stbetedge_p_slot: &mut f64,
        var_stbetedge_p_rv_slot: &mut f64,
        var_stct_p_slot: &mut f64,
        var_stct_p_rv_slot: &mut f64,
        var_stvfb_p_slot: &mut f64,
        var_stvfb_p_rv_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vfb_p_rv_slot: &mut f64,
        var_vsbnud_p_slot: &mut f64,
        var_vsbnud_p_rv_slot: &mut f64,
    ) {
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_betnedge_p_rv: f64 = *var_betnedge_p_rv_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cf_p_rv: f64 = *var_cf_p_rv_slot;
        let mut var_cfb_p: f64 = *var_cfb_p_slot;
        let mut var_cfb_p_rv: f64 = *var_cfb_p_rv_slot;
        let mut var_cfbedge_p: f64 = *var_cfbedge_p_slot;
        let mut var_cfbedge_p_rv: f64 = *var_cfbedge_p_rv_slot;
        let mut var_cfd_p: f64 = *var_cfd_p_slot;
        let mut var_cfd_p_rv: f64 = *var_cfd_p_rv_slot;
        let mut var_cfdedge_p: f64 = *var_cfdedge_p_slot;
        let mut var_cfdedge_p_rv: f64 = *var_cfdedge_p_rv_slot;
        let mut var_cfedge_p: f64 = *var_cfedge_p_slot;
        let mut var_cfedge_p_rv: f64 = *var_cfedge_p_rv_slot;
        let mut var_ct_p: f64 = *var_ct_p_slot;
        let mut var_ct_p_rv: f64 = *var_ct_p_rv_slot;
        let mut var_ctb_p: f64 = *var_ctb_p_slot;
        let mut var_ctb_p_rv: f64 = *var_ctb_p_rv_slot;
        let mut var_ctg_p: f64 = *var_ctg_p_slot;
        let mut var_ctg_p_rv: f64 = *var_ctg_p_rv_slot;
        let mut var_cth_p: f64 = *var_cth_p_slot;
        let mut var_cth_p_rv: f64 = *var_cth_p_rv_slot;
        let mut var_dphib_p: f64 = *var_dphib_p_slot;
        let mut var_dphib_p_rv: f64 = *var_dphib_p_rv_slot;
        let mut var_gfacnud_p: f64 = *var_gfacnud_p_slot;
        let mut var_gfacnud_p_rv: f64 = *var_gfacnud_p_rv_slot;
        let mut var_gpe_edge: f64 = *var_gpe_edge_slot;
        let mut var_gpe_edge_rv: f64 = *var_gpe_edge_rv_slot;
        let mut var_guard51: f64 = *var_guard51_slot;
        let mut var_guard51_rv: f64 = *var_guard51_rv_slot;
        let mut var_guard52: f64 = *var_guard52_slot;
        let mut var_guard52_rv: f64 = *var_guard52_rv_slot;
        let mut var_guard53: f64 = *var_guard53_slot;
        let mut var_guard53_rv: f64 = *var_guard53_rv_slot;
        let mut var_guard54: f64 = *var_guard54_slot;
        let mut var_guard54_rv: f64 = *var_guard54_rv_slot;
        let mut var_guard55: f64 = *var_guard55_slot;
        let mut var_guard55_rv: f64 = *var_guard55_rv_slot;
        let mut var_guard56: f64 = *var_guard56_slot;
        let mut var_guard56_rv: f64 = *var_guard56_rv_slot;
        let mut var_guard57: f64 = *var_guard57_slot;
        let mut var_guard57_rv: f64 = *var_guard57_rv_slot;
        let mut var_guard58: f64 = *var_guard58_slot;
        let mut var_guard58_rv: f64 = *var_guard58_rv_slot;
        let mut var_guard59: f64 = *var_guard59_slot;
        let mut var_guard59_rv: f64 = *var_guard59_rv_slot;
        let mut var_guard60: f64 = *var_guard60_slot;
        let mut var_guard60_rv: f64 = *var_guard60_rv_slot;
        let mut var_guard61: f64 = *var_guard61_slot;
        let mut var_guard61_rv: f64 = *var_guard61_rv_slot;
        let mut var_guard62: f64 = *var_guard62_slot;
        let mut var_guard62_rv: f64 = *var_guard62_rv_slot;
        let mut var_guard63: f64 = *var_guard63_slot;
        let mut var_guard63_rv: f64 = *var_guard63_rv_slot;
        let mut var_guard64: f64 = *var_guard64_slot;
        let mut var_guard64_rv: f64 = *var_guard64_rv_slot;
        let mut var_guard65: f64 = *var_guard65_slot;
        let mut var_guard65_rv: f64 = *var_guard65_rv_slot;
        let mut var_guard66: f64 = *var_guard66_slot;
        let mut var_guard66_rv: f64 = *var_guard66_rv_slot;
        let mut var_guard67: f64 = *var_guard67_slot;
        let mut var_guard67_rv: f64 = *var_guard67_rv_slot;
        let mut var_kuowe: f64 = *var_kuowe_slot;
        let mut var_kuowe_rv: f64 = *var_kuowe_rv_slot;
        let mut var_kvthowe: f64 = *var_kvthowe_slot;
        let mut var_kvthowe_rv: f64 = *var_kvthowe_rv_slot;
        let mut var_neff_p: f64 = *var_neff_p_slot;
        let mut var_neff_p_rv: f64 = *var_neff_p_rv_slot;
        let mut var_nov_p: f64 = *var_nov_p_slot;
        let mut var_nov_p_rv: f64 = *var_nov_p_rv_slot;
        let mut var_novd_p: f64 = *var_novd_p_slot;
        let mut var_novd_p_rv: f64 = *var_novd_p_rv_slot;
        let mut var_np_p: f64 = *var_np_p_slot;
        let mut var_np_p_rv: f64 = *var_np_p_rv_slot;
        let mut var_psce_p: f64 = *var_psce_p_slot;
        let mut var_psce_p_rv: f64 = *var_psce_p_rv_slot;
        let mut var_pscebedge_p: f64 = *var_pscebedge_p_slot;
        let mut var_pscebedge_p_rv: f64 = *var_pscebedge_p_rv_slot;
        let mut var_pscededge_p: f64 = *var_pscededge_p_slot;
        let mut var_pscededge_p_rv: f64 = *var_pscededge_p_rv_slot;
        let mut var_psceedge_p: f64 = *var_psceedge_p_slot;
        let mut var_psceedge_p_rv: f64 = *var_psceedge_p_rv_slot;
        let mut var_stbetedge_p: f64 = *var_stbetedge_p_slot;
        let mut var_stbetedge_p_rv: f64 = *var_stbetedge_p_rv_slot;
        let mut var_stct_p: f64 = *var_stct_p_slot;
        let mut var_stct_p_rv: f64 = *var_stct_p_rv_slot;
        let mut var_stvfb_p: f64 = *var_stvfb_p_slot;
        let mut var_stvfb_p_rv: f64 = *var_stvfb_p_rv_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vfb_p_rv: f64 = *var_vfb_p_rv_slot;
        let mut var_vsbnud_p: f64 = *var_vsbnud_p_slot;
        let mut var_vsbnud_p_rv: f64 = *var_vsbnud_p_rv_slot;

        let (assign6820_e5305,) = {
    if (var_guard36 != 0.0) {
        let (assign6820_e5303,) = {
            if (var_gpe_edge > 1e-15) {
                (var_gpe_edge,)
            } else {
                (1e-15,)
            }
        };
        (assign6820_e5303,)
    } else {
        (var_gpe_edge,)
    }
};
        var_gpe_edge = assign6820_e5305;
        var_gpe_edge_rv = 0.0;

        let (assign6830_e5321,) = {
    if (var_guard36 != 0.0) {
        let assign6830_e5309: f64 = (p.p259 * var_we_edge);
        let assign6830_e5312: f64 = (var_gpe_edge * var_le);
        let assign6830_e5313: f64 = (assign6830_e5309 / assign6830_e5312);
        let assign6830_e5317: f64 = (p.p420 * var_iwe);
        let assign6830_e5318: f64 = (1.0 + assign6830_e5317);
        let assign6830_e5319: f64 = (assign6830_e5313 * assign6830_e5318);
        (assign6830_e5319,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign6830_e5321;
        var_betnedge_p_rv = 0.0;

        let (assign6840_e5337,) = {
    if (var_guard36 != 0.0) {
        let assign6840_e5326: f64 = (p.p422 * var_ile);
        let assign6840_e5327: f64 = (p.p421 + assign6840_e5326);
        let assign6840_e5330: f64 = (p.p423 * var_iwe);
        let assign6840_e5331: f64 = (assign6840_e5327 + assign6840_e5330);
        let assign6840_e5334: f64 = (p.p424 * var_iae);
        let assign6840_e5335: f64 = (assign6840_e5331 + assign6840_e5334);
        (assign6840_e5335,)
    } else {
        (var_stbetedge_p,)
    }
};
        var_stbetedge_p = assign6840_e5337;
        var_stbetedge_p_rv = 0.0;

        let (assign6850_e5351,) = {
    if (var_guard36 != 0.0) {
        let assign6850_e5342: f64 = (var_ile).powf(p.p426);
        let assign6850_e5343: f64 = (p.p425 * assign6850_e5342);
        let assign6850_e5347: f64 = (p.p427 * var_iwe);
        let assign6850_e5348: f64 = (1.0 + assign6850_e5347);
        let assign6850_e5349: f64 = (assign6850_e5343 * assign6850_e5348);
        (assign6850_e5349,)
    } else {
        (var_psceedge_p,)
    }
};
        var_psceedge_p = assign6850_e5351;
        var_psceedge_p_rv = 0.0;

        let (assign6860_e5355,) = {
    if (var_guard36 != 0.0) {
        (p.p428,)
    } else {
        (var_pscebedge_p,)
    }
};
        var_pscebedge_p = assign6860_e5355;
        var_pscebedge_p_rv = 0.0;

        let (assign6870_e5359,) = {
    if (var_guard36 != 0.0) {
        (p.p429,)
    } else {
        (var_pscededge_p,)
    }
};
        var_pscededge_p = assign6870_e5359;
        var_pscededge_p_rv = 0.0;

        let (assign6880_e5373,) = {
    if (var_guard36 != 0.0) {
        let assign6880_e5364: f64 = (var_ile).powf(p.p431);
        let assign6880_e5365: f64 = (p.p430 * assign6880_e5364);
        let assign6880_e5369: f64 = (p.p432 * var_iwe);
        let assign6880_e5370: f64 = (1.0 + assign6880_e5369);
        let assign6880_e5371: f64 = (assign6880_e5365 * assign6880_e5370);
        (assign6880_e5371,)
    } else {
        (var_cfedge_p,)
    }
};
        var_cfedge_p = assign6880_e5373;
        var_cfedge_p_rv = 0.0;

        let (assign6890_e5377,) = {
    if (var_guard36 != 0.0) {
        (p.p434,)
    } else {
        (var_cfdedge_p,)
    }
};
        var_cfdedge_p = assign6890_e5377;
        var_cfdedge_p_rv = 0.0;

        let (assign6900_e5381,) = {
    if (var_guard36 != 0.0) {
        (p.p433,)
    } else {
        (var_cfbedge_p,)
    }
};
        var_cfbedge_p = assign6900_e5381;
        var_cfbedge_p_rv = 0.0;

        let (assign6960_e5423,) = {
    if (var_guard36 != 0.0) {
        let assign6960_e5412: f64 = (p.p832 * var_ile);
        let assign6960_e5413: f64 = (p.p831 + assign6960_e5412);
        let assign6960_e5416: f64 = (p.p833 * var_iwe);
        let assign6960_e5417: f64 = (assign6960_e5413 + assign6960_e5416);
        let assign6960_e5420: f64 = (p.p834 * var_iae);
        let assign6960_e5421: f64 = (assign6960_e5417 + assign6960_e5420);
        (assign6960_e5421,)
    } else {
        (var_kvthowe,)
    }
};
        var_kvthowe = assign6960_e5423;
        var_kvthowe_rv = 0.0;

        let (assign6970_e5439,) = {
    if (var_guard36 != 0.0) {
        let assign6970_e5428: f64 = (p.p836 * var_ile);
        let assign6970_e5429: f64 = (p.p835 + assign6970_e5428);
        let assign6970_e5432: f64 = (p.p837 * var_iwe);
        let assign6970_e5433: f64 = (assign6970_e5429 + assign6970_e5432);
        let assign6970_e5436: f64 = (p.p838 * var_iae);
        let assign6970_e5437: f64 = (assign6970_e5433 + assign6970_e5436);
        (assign6970_e5437,)
    } else {
        (var_kuowe,)
    }
};
        var_kuowe = assign6970_e5439;
        var_kuowe_rv = 0.0;

        let (assign7120_e5579,) = {
    if (var_guard36 != 0.0) {
        let assign7120_e5571: f64 = (p.p458 / var_ile);
        let assign7120_e5572: f64 = (1.0 + assign7120_e5571);
        let assign7120_e5573: f64 = (p.p457 + assign7120_e5572);
        let assign7120_e5574: f64 = (p.p456 * assign7120_e5573);
        let assign7120_e5576: f64 = (assign7120_e5574 / var_iwe);
        let assign7120_e5577: f64 = (p.p455 + assign7120_e5576);
        (assign7120_e5577,)
    } else {
        (var_cth_p,)
    }
};
        var_cth_p = assign7120_e5579;
        var_cth_p_rv = 0.0;

        let assign7140_e5602: f64 = if (((param_given[460] || param_given[461]) || param_given[462]) || param_given[463]) { 1.0 } else { 0.0 };
        var_guard51 = assign7140_e5602;
        var_guard51_rv = 0.0;

        let (assign7150_e5620,) = {
    if ((var_guard36 != 0.0) && (var_guard51 != 0.0)) {
        let assign7150_e5609: f64 = (p.p461 * var_ile);
        let assign7150_e5610: f64 = (p.p460 + assign7150_e5609);
        let assign7150_e5613: f64 = (p.p462 * var_iwe);
        let assign7150_e5614: f64 = (assign7150_e5610 + assign7150_e5613);
        let assign7150_e5617: f64 = (p.p463 * var_iae);
        let assign7150_e5618: f64 = (assign7150_e5614 + assign7150_e5617);
        (assign7150_e5618,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign7150_e5620;
        var_vfb_p_rv = 0.0;

        let assign7160_e5639: f64 = if (((param_given[464] || param_given[465]) || param_given[466]) || param_given[467]) { 1.0 } else { 0.0 };
        var_guard52 = assign7160_e5639;
        var_guard52_rv = 0.0;

        let (assign7170_e5657,) = {
    if ((var_guard36 != 0.0) && (var_guard52 != 0.0)) {
        let assign7170_e5646: f64 = (p.p465 * var_ile);
        let assign7170_e5647: f64 = (p.p464 + assign7170_e5646);
        let assign7170_e5650: f64 = (p.p466 * var_iwe);
        let assign7170_e5651: f64 = (assign7170_e5647 + assign7170_e5650);
        let assign7170_e5654: f64 = (p.p467 * var_iae);
        let assign7170_e5655: f64 = (assign7170_e5651 + assign7170_e5654);
        (assign7170_e5655,)
    } else {
        (var_stvfb_p,)
    }
};
        var_stvfb_p = assign7170_e5657;
        var_stvfb_p_rv = 0.0;

        let assign7180_e5676: f64 = if (((param_given[468] || param_given[469]) || param_given[470]) || param_given[471]) { 1.0 } else { 0.0 };
        var_guard53 = assign7180_e5676;
        var_guard53_rv = 0.0;

        let (assign7190_e5694,) = {
    if ((var_guard36 != 0.0) && (var_guard53 != 0.0)) {
        let assign7190_e5683: f64 = (p.p469 * var_ile);
        let assign7190_e5684: f64 = (p.p468 + assign7190_e5683);
        let assign7190_e5687: f64 = (p.p470 * var_iwe);
        let assign7190_e5688: f64 = (assign7190_e5684 + assign7190_e5687);
        let assign7190_e5691: f64 = (p.p471 * var_iae);
        let assign7190_e5692: f64 = (assign7190_e5688 + assign7190_e5691);
        (assign7190_e5692,)
    } else {
        (var_neff_p,)
    }
};
        var_neff_p = assign7190_e5694;
        var_neff_p_rv = 0.0;

        let assign7200_e5713: f64 = if (((param_given[472] || param_given[473]) || param_given[474]) || param_given[475]) { 1.0 } else { 0.0 };
        var_guard54 = assign7200_e5713;
        var_guard54_rv = 0.0;

        let (assign7210_e5731,) = {
    if ((var_guard36 != 0.0) && (var_guard54 != 0.0)) {
        let assign7210_e5720: f64 = (p.p473 * var_ile);
        let assign7210_e5721: f64 = (p.p472 + assign7210_e5720);
        let assign7210_e5724: f64 = (p.p474 * var_iwe);
        let assign7210_e5725: f64 = (assign7210_e5721 + assign7210_e5724);
        let assign7210_e5728: f64 = (p.p475 * var_iae);
        let assign7210_e5729: f64 = (assign7210_e5725 + assign7210_e5728);
        (assign7210_e5729,)
    } else {
        (var_gfacnud_p,)
    }
};
        var_gfacnud_p = assign7210_e5731;
        var_gfacnud_p_rv = 0.0;

        let assign7220_e5750: f64 = if (((param_given[476] || param_given[477]) || param_given[478]) || param_given[479]) { 1.0 } else { 0.0 };
        var_guard55 = assign7220_e5750;
        var_guard55_rv = 0.0;

        let (assign7230_e5768,) = {
    if ((var_guard36 != 0.0) && (var_guard55 != 0.0)) {
        let assign7230_e5757: f64 = (p.p477 * var_ile);
        let assign7230_e5758: f64 = (p.p476 + assign7230_e5757);
        let assign7230_e5761: f64 = (p.p478 * var_iwe);
        let assign7230_e5762: f64 = (assign7230_e5758 + assign7230_e5761);
        let assign7230_e5765: f64 = (p.p479 * var_iae);
        let assign7230_e5766: f64 = (assign7230_e5762 + assign7230_e5765);
        (assign7230_e5766,)
    } else {
        (var_vsbnud_p,)
    }
};
        var_vsbnud_p = assign7230_e5768;
        var_vsbnud_p_rv = 0.0;

        let assign7240_e5787: f64 = if (((param_given[480] || param_given[481]) || param_given[482]) || param_given[483]) { 1.0 } else { 0.0 };
        var_guard56 = assign7240_e5787;
        var_guard56_rv = 0.0;

        let (assign7250_e5805,) = {
    if ((var_guard36 != 0.0) && (var_guard56 != 0.0)) {
        let assign7250_e5794: f64 = (p.p481 * var_ile);
        let assign7250_e5795: f64 = (p.p480 + assign7250_e5794);
        let assign7250_e5798: f64 = (p.p482 * var_iwe);
        let assign7250_e5799: f64 = (assign7250_e5795 + assign7250_e5798);
        let assign7250_e5802: f64 = (p.p483 * var_iae);
        let assign7250_e5803: f64 = (assign7250_e5799 + assign7250_e5802);
        (assign7250_e5803,)
    } else {
        (var_dphib_p,)
    }
};
        var_dphib_p = assign7250_e5805;
        var_dphib_p_rv = 0.0;

        let assign7260_e5824: f64 = if (((param_given[484] || param_given[485]) || param_given[486]) || param_given[487]) { 1.0 } else { 0.0 };
        var_guard57 = assign7260_e5824;
        var_guard57_rv = 0.0;

        let (assign7270_e5842,) = {
    if ((var_guard36 != 0.0) && (var_guard57 != 0.0)) {
        let assign7270_e5831: f64 = (p.p485 * var_ile);
        let assign7270_e5832: f64 = (p.p484 + assign7270_e5831);
        let assign7270_e5835: f64 = (p.p486 * var_iwe);
        let assign7270_e5836: f64 = (assign7270_e5832 + assign7270_e5835);
        let assign7270_e5839: f64 = (p.p487 * var_iae);
        let assign7270_e5840: f64 = (assign7270_e5836 + assign7270_e5839);
        (assign7270_e5840,)
    } else {
        (var_np_p,)
    }
};
        var_np_p = assign7270_e5842;
        var_np_p_rv = 0.0;

        let assign7280_e5861: f64 = if (((param_given[488] || param_given[489]) || param_given[490]) || param_given[491]) { 1.0 } else { 0.0 };
        var_guard58 = assign7280_e5861;
        var_guard58_rv = 0.0;

        let (assign7290_e5879,) = {
    if ((var_guard36 != 0.0) && (var_guard58 != 0.0)) {
        let assign7290_e5868: f64 = (p.p489 * var_ile);
        let assign7290_e5869: f64 = (p.p488 + assign7290_e5868);
        let assign7290_e5872: f64 = (p.p490 * var_iwe);
        let assign7290_e5873: f64 = (assign7290_e5869 + assign7290_e5872);
        let assign7290_e5876: f64 = (p.p491 * var_iae);
        let assign7290_e5877: f64 = (assign7290_e5873 + assign7290_e5876);
        (assign7290_e5877,)
    } else {
        (var_nov_p,)
    }
};
        var_nov_p = assign7290_e5879;
        var_nov_p_rv = 0.0;

        let assign7300_e5898: f64 = if (((param_given[492] || param_given[493]) || param_given[494]) || param_given[495]) { 1.0 } else { 0.0 };
        var_guard59 = assign7300_e5898;
        var_guard59_rv = 0.0;

        let (assign7310_e5916,) = {
    if ((var_guard36 != 0.0) && (var_guard59 != 0.0)) {
        let assign7310_e5905: f64 = (p.p493 * var_ile);
        let assign7310_e5906: f64 = (p.p492 + assign7310_e5905);
        let assign7310_e5909: f64 = (p.p494 * var_iwe);
        let assign7310_e5910: f64 = (assign7310_e5906 + assign7310_e5909);
        let assign7310_e5913: f64 = (p.p495 * var_iae);
        let assign7310_e5914: f64 = (assign7310_e5910 + assign7310_e5913);
        (assign7310_e5914,)
    } else {
        (var_novd_p,)
    }
};
        var_novd_p = assign7310_e5916;
        var_novd_p_rv = 0.0;

        let assign7320_e5935: f64 = if (((param_given[496] || param_given[497]) || param_given[498]) || param_given[499]) { 1.0 } else { 0.0 };
        var_guard60 = assign7320_e5935;
        var_guard60_rv = 0.0;

        let (assign7330_e5953,) = {
    if ((var_guard36 != 0.0) && (var_guard60 != 0.0)) {
        let assign7330_e5942: f64 = (p.p497 * var_ile);
        let assign7330_e5943: f64 = (p.p496 + assign7330_e5942);
        let assign7330_e5946: f64 = (p.p498 * var_iwe);
        let assign7330_e5947: f64 = (assign7330_e5943 + assign7330_e5946);
        let assign7330_e5950: f64 = (p.p499 * var_iae);
        let assign7330_e5951: f64 = (assign7330_e5947 + assign7330_e5950);
        (assign7330_e5951,)
    } else {
        (var_ct_p,)
    }
};
        var_ct_p = assign7330_e5953;
        var_ct_p_rv = 0.0;

        let assign7340_e5972: f64 = if (((param_given[504] || param_given[505]) || param_given[506]) || param_given[507]) { 1.0 } else { 0.0 };
        var_guard61 = assign7340_e5972;
        var_guard61_rv = 0.0;

        let (assign7350_e5990,) = {
    if ((var_guard36 != 0.0) && (var_guard61 != 0.0)) {
        let assign7350_e5979: f64 = (p.p505 * var_ile);
        let assign7350_e5980: f64 = (p.p504 + assign7350_e5979);
        let assign7350_e5983: f64 = (p.p506 * var_iwe);
        let assign7350_e5984: f64 = (assign7350_e5980 + assign7350_e5983);
        let assign7350_e5987: f64 = (p.p507 * var_iae);
        let assign7350_e5988: f64 = (assign7350_e5984 + assign7350_e5987);
        (assign7350_e5988,)
    } else {
        (var_ctg_p,)
    }
};
        var_ctg_p = assign7350_e5990;
        var_ctg_p_rv = 0.0;

        let assign7360_e6009: f64 = if (((param_given[500] || param_given[501]) || param_given[502]) || param_given[503]) { 1.0 } else { 0.0 };
        var_guard62 = assign7360_e6009;
        var_guard62_rv = 0.0;

        let (assign7370_e6027,) = {
    if ((var_guard36 != 0.0) && (var_guard62 != 0.0)) {
        let assign7370_e6016: f64 = (p.p501 * var_ile);
        let assign7370_e6017: f64 = (p.p500 + assign7370_e6016);
        let assign7370_e6020: f64 = (p.p502 * var_iwe);
        let assign7370_e6021: f64 = (assign7370_e6017 + assign7370_e6020);
        let assign7370_e6024: f64 = (p.p503 * var_iae);
        let assign7370_e6025: f64 = (assign7370_e6021 + assign7370_e6024);
        (assign7370_e6025,)
    } else {
        (var_ctb_p,)
    }
};
        var_ctb_p = assign7370_e6027;
        var_ctb_p_rv = 0.0;

        let assign7380_e6046: f64 = if (((param_given[508] || param_given[509]) || param_given[510]) || param_given[511]) { 1.0 } else { 0.0 };
        var_guard63 = assign7380_e6046;
        var_guard63_rv = 0.0;

        let (assign7390_e6064,) = {
    if ((var_guard36 != 0.0) && (var_guard63 != 0.0)) {
        let assign7390_e6053: f64 = (p.p509 * var_ile);
        let assign7390_e6054: f64 = (p.p508 + assign7390_e6053);
        let assign7390_e6057: f64 = (p.p510 * var_iwe);
        let assign7390_e6058: f64 = (assign7390_e6054 + assign7390_e6057);
        let assign7390_e6061: f64 = (p.p511 * var_iae);
        let assign7390_e6062: f64 = (assign7390_e6058 + assign7390_e6061);
        (assign7390_e6062,)
    } else {
        (var_stct_p,)
    }
};
        var_stct_p = assign7390_e6064;
        var_stct_p_rv = 0.0;

        let assign7400_e6083: f64 = if (((param_given[512] || param_given[513]) || param_given[514]) || param_given[515]) { 1.0 } else { 0.0 };
        var_guard64 = assign7400_e6083;
        var_guard64_rv = 0.0;

        let (assign7410_e6103,) = {
    if ((var_guard36 != 0.0) && (var_guard64 != 0.0)) {
        let assign7410_e6091: f64 = (p.p513 * var_ile);
        let assign7410_e6092: f64 = (p.p512 + assign7410_e6091);
        let assign7410_e6095: f64 = (p.p514 * var_iwe);
        let assign7410_e6096: f64 = (assign7410_e6092 + assign7410_e6095);
        let assign7410_e6099: f64 = (p.p515 * var_iae);
        let assign7410_e6100: f64 = (assign7410_e6096 + assign7410_e6099);
        let assign7410_e6101: f64 = (var_ile2 * assign7410_e6100);
        (assign7410_e6101,)
    } else {
        (var_cf_p,)
    }
};
        var_cf_p = assign7410_e6103;
        var_cf_p_rv = 0.0;

        let assign7420_e6122: f64 = if (((param_given[520] || param_given[521]) || param_given[522]) || param_given[523]) { 1.0 } else { 0.0 };
        var_guard65 = assign7420_e6122;
        var_guard65_rv = 0.0;

        let (assign7430_e6140,) = {
    if ((var_guard36 != 0.0) && (var_guard65 != 0.0)) {
        let assign7430_e6129: f64 = (p.p521 * var_ile);
        let assign7430_e6130: f64 = (p.p520 + assign7430_e6129);
        let assign7430_e6133: f64 = (p.p522 * var_iwe);
        let assign7430_e6134: f64 = (assign7430_e6130 + assign7430_e6133);
        let assign7430_e6137: f64 = (p.p523 * var_iae);
        let assign7430_e6138: f64 = (assign7430_e6134 + assign7430_e6137);
        (assign7430_e6138,)
    } else {
        (var_cfd_p,)
    }
};
        var_cfd_p = assign7430_e6140;
        var_cfd_p_rv = 0.0;

        let assign7440_e6159: f64 = if (((param_given[516] || param_given[517]) || param_given[518]) || param_given[519]) { 1.0 } else { 0.0 };
        var_guard66 = assign7440_e6159;
        var_guard66_rv = 0.0;

        let (assign7450_e6177,) = {
    if ((var_guard36 != 0.0) && (var_guard66 != 0.0)) {
        let assign7450_e6166: f64 = (p.p517 * var_ile);
        let assign7450_e6167: f64 = (p.p516 + assign7450_e6166);
        let assign7450_e6170: f64 = (p.p518 * var_iwe);
        let assign7450_e6171: f64 = (assign7450_e6167 + assign7450_e6170);
        let assign7450_e6174: f64 = (p.p519 * var_iae);
        let assign7450_e6175: f64 = (assign7450_e6171 + assign7450_e6174);
        (assign7450_e6175,)
    } else {
        (var_cfb_p,)
    }
};
        var_cfb_p = assign7450_e6177;
        var_cfb_p_rv = 0.0;

        let assign7460_e6196: f64 = if (((param_given[524] || param_given[525]) || param_given[526]) || param_given[527]) { 1.0 } else { 0.0 };
        var_guard67 = assign7460_e6196;
        var_guard67_rv = 0.0;

        let (assign7470_e6216,) = {
    if ((var_guard36 != 0.0) && (var_guard67 != 0.0)) {
        let assign7470_e6204: f64 = (p.p525 * var_ile);
        let assign7470_e6205: f64 = (p.p524 + assign7470_e6204);
        let assign7470_e6208: f64 = (p.p526 * var_iwe);
        let assign7470_e6209: f64 = (assign7470_e6205 + assign7470_e6208);
        let assign7470_e6212: f64 = (p.p527 * var_iae);
        let assign7470_e6213: f64 = (assign7470_e6209 + assign7470_e6212);
        let assign7470_e6214: f64 = (var_ile2 * assign7470_e6213);
        (assign7470_e6214,)
    } else {
        (var_psce_p,)
    }
};
        var_psce_p = assign7470_e6216;
        var_psce_p_rv = 0.0;

        *var_betnedge_p_slot = var_betnedge_p;
        *var_betnedge_p_rv_slot = var_betnedge_p_rv;
        *var_cf_p_slot = var_cf_p;
        *var_cf_p_rv_slot = var_cf_p_rv;
        *var_cfb_p_slot = var_cfb_p;
        *var_cfb_p_rv_slot = var_cfb_p_rv;
        *var_cfbedge_p_slot = var_cfbedge_p;
        *var_cfbedge_p_rv_slot = var_cfbedge_p_rv;
        *var_cfd_p_slot = var_cfd_p;
        *var_cfd_p_rv_slot = var_cfd_p_rv;
        *var_cfdedge_p_slot = var_cfdedge_p;
        *var_cfdedge_p_rv_slot = var_cfdedge_p_rv;
        *var_cfedge_p_slot = var_cfedge_p;
        *var_cfedge_p_rv_slot = var_cfedge_p_rv;
        *var_ct_p_slot = var_ct_p;
        *var_ct_p_rv_slot = var_ct_p_rv;
        *var_ctb_p_slot = var_ctb_p;
        *var_ctb_p_rv_slot = var_ctb_p_rv;
        *var_ctg_p_slot = var_ctg_p;
        *var_ctg_p_rv_slot = var_ctg_p_rv;
        *var_cth_p_slot = var_cth_p;
        *var_cth_p_rv_slot = var_cth_p_rv;
        *var_dphib_p_slot = var_dphib_p;
        *var_dphib_p_rv_slot = var_dphib_p_rv;
        *var_gfacnud_p_slot = var_gfacnud_p;
        *var_gfacnud_p_rv_slot = var_gfacnud_p_rv;
        *var_gpe_edge_slot = var_gpe_edge;
        *var_gpe_edge_rv_slot = var_gpe_edge_rv;
        *var_guard51_slot = var_guard51;
        *var_guard51_rv_slot = var_guard51_rv;
        *var_guard52_slot = var_guard52;
        *var_guard52_rv_slot = var_guard52_rv;
        *var_guard53_slot = var_guard53;
        *var_guard53_rv_slot = var_guard53_rv;
        *var_guard54_slot = var_guard54;
        *var_guard54_rv_slot = var_guard54_rv;
        *var_guard55_slot = var_guard55;
        *var_guard55_rv_slot = var_guard55_rv;
        *var_guard56_slot = var_guard56;
        *var_guard56_rv_slot = var_guard56_rv;
        *var_guard57_slot = var_guard57;
        *var_guard57_rv_slot = var_guard57_rv;
        *var_guard58_slot = var_guard58;
        *var_guard58_rv_slot = var_guard58_rv;
        *var_guard59_slot = var_guard59;
        *var_guard59_rv_slot = var_guard59_rv;
        *var_guard60_slot = var_guard60;
        *var_guard60_rv_slot = var_guard60_rv;
        *var_guard61_slot = var_guard61;
        *var_guard61_rv_slot = var_guard61_rv;
        *var_guard62_slot = var_guard62;
        *var_guard62_rv_slot = var_guard62_rv;
        *var_guard63_slot = var_guard63;
        *var_guard63_rv_slot = var_guard63_rv;
        *var_guard64_slot = var_guard64;
        *var_guard64_rv_slot = var_guard64_rv;
        *var_guard65_slot = var_guard65;
        *var_guard65_rv_slot = var_guard65_rv;
        *var_guard66_slot = var_guard66;
        *var_guard66_rv_slot = var_guard66_rv;
        *var_guard67_slot = var_guard67;
        *var_guard67_rv_slot = var_guard67_rv;
        *var_kuowe_slot = var_kuowe;
        *var_kuowe_rv_slot = var_kuowe_rv;
        *var_kvthowe_slot = var_kvthowe;
        *var_kvthowe_rv_slot = var_kvthowe_rv;
        *var_neff_p_slot = var_neff_p;
        *var_neff_p_rv_slot = var_neff_p_rv;
        *var_nov_p_slot = var_nov_p;
        *var_nov_p_rv_slot = var_nov_p_rv;
        *var_novd_p_slot = var_novd_p;
        *var_novd_p_rv_slot = var_novd_p_rv;
        *var_np_p_slot = var_np_p;
        *var_np_p_rv_slot = var_np_p_rv;
        *var_psce_p_slot = var_psce_p;
        *var_psce_p_rv_slot = var_psce_p_rv;
        *var_pscebedge_p_slot = var_pscebedge_p;
        *var_pscebedge_p_rv_slot = var_pscebedge_p_rv;
        *var_pscededge_p_slot = var_pscededge_p;
        *var_pscededge_p_rv_slot = var_pscededge_p_rv;
        *var_psceedge_p_slot = var_psceedge_p;
        *var_psceedge_p_rv_slot = var_psceedge_p_rv;
        *var_stbetedge_p_slot = var_stbetedge_p;
        *var_stbetedge_p_rv_slot = var_stbetedge_p_rv;
        *var_stct_p_slot = var_stct_p;
        *var_stct_p_rv_slot = var_stct_p_rv;
        *var_stvfb_p_slot = var_stvfb_p;
        *var_stvfb_p_rv_slot = var_stvfb_p_rv;
        *var_vfb_p_slot = var_vfb_p;
        *var_vfb_p_rv_slot = var_vfb_p_rv;
        *var_vsbnud_p_slot = var_vsbnud_p;
        *var_vsbnud_p_rv_slot = var_vsbnud_p_rv;
    }

    pub(super) fn stamp_reactive_block_11(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard36: f64,
        var_iae: f64,
        var_ile: f64,
        var_iwe: f64,
        var_le: f64,
        var_we: f64,
        var_a1_p_slot: &mut f64,
        var_a1_p_rv_slot: &mut f64,
        var_a3_p_slot: &mut f64,
        var_a3_p_rv_slot: &mut f64,
        var_a4_p_slot: &mut f64,
        var_a4_p_rv_slot: &mut f64,
        var_alp1_p_slot: &mut f64,
        var_alp1_p_rv_slot: &mut f64,
        var_alp2_p_slot: &mut f64,
        var_alp2_p_rv_slot: &mut f64,
        var_alp_p_slot: &mut f64,
        var_alp_p_rv_slot: &mut f64,
        var_ax_p_slot: &mut f64,
        var_ax_p_rv_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_rv_slot: &mut f64,
        var_cs_p_slot: &mut f64,
        var_cs_p_rv_slot: &mut f64,
        var_guard68_slot: &mut f64,
        var_guard68_rv_slot: &mut f64,
        var_guard69_slot: &mut f64,
        var_guard69_rv_slot: &mut f64,
        var_guard70_slot: &mut f64,
        var_guard70_rv_slot: &mut f64,
        var_guard71_slot: &mut f64,
        var_guard71_rv_slot: &mut f64,
        var_guard72_slot: &mut f64,
        var_guard72_rv_slot: &mut f64,
        var_guard73_slot: &mut f64,
        var_guard73_rv_slot: &mut f64,
        var_guard74_slot: &mut f64,
        var_guard74_rv_slot: &mut f64,
        var_guard75_slot: &mut f64,
        var_guard75_rv_slot: &mut f64,
        var_guard76_slot: &mut f64,
        var_guard76_rv_slot: &mut f64,
        var_guard77_slot: &mut f64,
        var_guard77_rv_slot: &mut f64,
        var_guard78_slot: &mut f64,
        var_guard78_rv_slot: &mut f64,
        var_guard79_slot: &mut f64,
        var_guard79_rv_slot: &mut f64,
        var_guard80_slot: &mut f64,
        var_guard80_rv_slot: &mut f64,
        var_guard81_slot: &mut f64,
        var_guard81_rv_slot: &mut f64,
        var_guard82_slot: &mut f64,
        var_guard82_rv_slot: &mut f64,
        var_guard83_slot: &mut f64,
        var_guard83_rv_slot: &mut f64,
        var_guard84_slot: &mut f64,
        var_guard84_rv_slot: &mut f64,
        var_guard85_slot: &mut f64,
        var_guard85_rv_slot: &mut f64,
        var_guard86_slot: &mut f64,
        var_guard86_rv_slot: &mut f64,
        var_guard87_slot: &mut f64,
        var_guard87_rv_slot: &mut f64,
        var_guard88_slot: &mut f64,
        var_guard88_rv_slot: &mut f64,
        var_guard89_slot: &mut f64,
        var_guard89_rv_slot: &mut f64,
        var_guard90_slot: &mut f64,
        var_guard90_rv_slot: &mut f64,
        var_guard91_slot: &mut f64,
        var_guard91_rv_slot: &mut f64,
        var_guard92_slot: &mut f64,
        var_guard92_rv_slot: &mut f64,
        var_guard93_slot: &mut f64,
        var_guard93_rv_slot: &mut f64,
        var_mue_p_slot: &mut f64,
        var_mue_p_rv_slot: &mut f64,
        var_psceb_p_slot: &mut f64,
        var_psceb_p_rv_slot: &mut f64,
        var_psced_p_slot: &mut f64,
        var_psced_p_rv_slot: &mut f64,
        var_rs_p_slot: &mut f64,
        var_rs_p_rv_slot: &mut f64,
        var_rsb_p_slot: &mut f64,
        var_rsb_p_rv_slot: &mut f64,
        var_rsg_p_slot: &mut f64,
        var_rsg_p_rv_slot: &mut f64,
        var_sta2_p_slot: &mut f64,
        var_sta2_p_rv_slot: &mut f64,
        var_stbet_p_slot: &mut f64,
        var_stbet_p_rv_slot: &mut f64,
        var_strs_p_slot: &mut f64,
        var_strs_p_rv_slot: &mut f64,
        var_stthesat_p_slot: &mut f64,
        var_stthesat_p_rv_slot: &mut f64,
        var_thecs_p_slot: &mut f64,
        var_thecs_p_rv_slot: &mut f64,
        var_themu_p_slot: &mut f64,
        var_themu_p_rv_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesat_p_rv_slot: &mut f64,
        var_thesatb_p_slot: &mut f64,
        var_thesatb_p_rv_slot: &mut f64,
        var_thesatg_p_slot: &mut f64,
        var_thesatg_p_rv_slot: &mut f64,
        var_xcor_p_slot: &mut f64,
        var_xcor_p_rv_slot: &mut f64,
    ) {
        let mut var_a1_p: f64 = *var_a1_p_slot;
        let mut var_a1_p_rv: f64 = *var_a1_p_rv_slot;
        let mut var_a3_p: f64 = *var_a3_p_slot;
        let mut var_a3_p_rv: f64 = *var_a3_p_rv_slot;
        let mut var_a4_p: f64 = *var_a4_p_slot;
        let mut var_a4_p_rv: f64 = *var_a4_p_rv_slot;
        let mut var_alp1_p: f64 = *var_alp1_p_slot;
        let mut var_alp1_p_rv: f64 = *var_alp1_p_rv_slot;
        let mut var_alp2_p: f64 = *var_alp2_p_slot;
        let mut var_alp2_p_rv: f64 = *var_alp2_p_rv_slot;
        let mut var_alp_p: f64 = *var_alp_p_slot;
        let mut var_alp_p_rv: f64 = *var_alp_p_rv_slot;
        let mut var_ax_p: f64 = *var_ax_p_slot;
        let mut var_ax_p_rv: f64 = *var_ax_p_rv_slot;
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_rv: f64 = *var_betn_p_rv_slot;
        let mut var_cs_p: f64 = *var_cs_p_slot;
        let mut var_cs_p_rv: f64 = *var_cs_p_rv_slot;
        let mut var_guard68: f64 = *var_guard68_slot;
        let mut var_guard68_rv: f64 = *var_guard68_rv_slot;
        let mut var_guard69: f64 = *var_guard69_slot;
        let mut var_guard69_rv: f64 = *var_guard69_rv_slot;
        let mut var_guard70: f64 = *var_guard70_slot;
        let mut var_guard70_rv: f64 = *var_guard70_rv_slot;
        let mut var_guard71: f64 = *var_guard71_slot;
        let mut var_guard71_rv: f64 = *var_guard71_rv_slot;
        let mut var_guard72: f64 = *var_guard72_slot;
        let mut var_guard72_rv: f64 = *var_guard72_rv_slot;
        let mut var_guard73: f64 = *var_guard73_slot;
        let mut var_guard73_rv: f64 = *var_guard73_rv_slot;
        let mut var_guard74: f64 = *var_guard74_slot;
        let mut var_guard74_rv: f64 = *var_guard74_rv_slot;
        let mut var_guard75: f64 = *var_guard75_slot;
        let mut var_guard75_rv: f64 = *var_guard75_rv_slot;
        let mut var_guard76: f64 = *var_guard76_slot;
        let mut var_guard76_rv: f64 = *var_guard76_rv_slot;
        let mut var_guard77: f64 = *var_guard77_slot;
        let mut var_guard77_rv: f64 = *var_guard77_rv_slot;
        let mut var_guard78: f64 = *var_guard78_slot;
        let mut var_guard78_rv: f64 = *var_guard78_rv_slot;
        let mut var_guard79: f64 = *var_guard79_slot;
        let mut var_guard79_rv: f64 = *var_guard79_rv_slot;
        let mut var_guard80: f64 = *var_guard80_slot;
        let mut var_guard80_rv: f64 = *var_guard80_rv_slot;
        let mut var_guard81: f64 = *var_guard81_slot;
        let mut var_guard81_rv: f64 = *var_guard81_rv_slot;
        let mut var_guard82: f64 = *var_guard82_slot;
        let mut var_guard82_rv: f64 = *var_guard82_rv_slot;
        let mut var_guard83: f64 = *var_guard83_slot;
        let mut var_guard83_rv: f64 = *var_guard83_rv_slot;
        let mut var_guard84: f64 = *var_guard84_slot;
        let mut var_guard84_rv: f64 = *var_guard84_rv_slot;
        let mut var_guard85: f64 = *var_guard85_slot;
        let mut var_guard85_rv: f64 = *var_guard85_rv_slot;
        let mut var_guard86: f64 = *var_guard86_slot;
        let mut var_guard86_rv: f64 = *var_guard86_rv_slot;
        let mut var_guard87: f64 = *var_guard87_slot;
        let mut var_guard87_rv: f64 = *var_guard87_rv_slot;
        let mut var_guard88: f64 = *var_guard88_slot;
        let mut var_guard88_rv: f64 = *var_guard88_rv_slot;
        let mut var_guard89: f64 = *var_guard89_slot;
        let mut var_guard89_rv: f64 = *var_guard89_rv_slot;
        let mut var_guard90: f64 = *var_guard90_slot;
        let mut var_guard90_rv: f64 = *var_guard90_rv_slot;
        let mut var_guard91: f64 = *var_guard91_slot;
        let mut var_guard91_rv: f64 = *var_guard91_rv_slot;
        let mut var_guard92: f64 = *var_guard92_slot;
        let mut var_guard92_rv: f64 = *var_guard92_rv_slot;
        let mut var_guard93: f64 = *var_guard93_slot;
        let mut var_guard93_rv: f64 = *var_guard93_rv_slot;
        let mut var_mue_p: f64 = *var_mue_p_slot;
        let mut var_mue_p_rv: f64 = *var_mue_p_rv_slot;
        let mut var_psceb_p: f64 = *var_psceb_p_slot;
        let mut var_psceb_p_rv: f64 = *var_psceb_p_rv_slot;
        let mut var_psced_p: f64 = *var_psced_p_slot;
        let mut var_psced_p_rv: f64 = *var_psced_p_rv_slot;
        let mut var_rs_p: f64 = *var_rs_p_slot;
        let mut var_rs_p_rv: f64 = *var_rs_p_rv_slot;
        let mut var_rsb_p: f64 = *var_rsb_p_slot;
        let mut var_rsb_p_rv: f64 = *var_rsb_p_rv_slot;
        let mut var_rsg_p: f64 = *var_rsg_p_slot;
        let mut var_rsg_p_rv: f64 = *var_rsg_p_rv_slot;
        let mut var_sta2_p: f64 = *var_sta2_p_slot;
        let mut var_sta2_p_rv: f64 = *var_sta2_p_rv_slot;
        let mut var_stbet_p: f64 = *var_stbet_p_slot;
        let mut var_stbet_p_rv: f64 = *var_stbet_p_rv_slot;
        let mut var_strs_p: f64 = *var_strs_p_slot;
        let mut var_strs_p_rv: f64 = *var_strs_p_rv_slot;
        let mut var_stthesat_p: f64 = *var_stthesat_p_slot;
        let mut var_stthesat_p_rv: f64 = *var_stthesat_p_rv_slot;
        let mut var_thecs_p: f64 = *var_thecs_p_slot;
        let mut var_thecs_p_rv: f64 = *var_thecs_p_rv_slot;
        let mut var_themu_p: f64 = *var_themu_p_slot;
        let mut var_themu_p_rv: f64 = *var_themu_p_rv_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesat_p_rv: f64 = *var_thesat_p_rv_slot;
        let mut var_thesatb_p: f64 = *var_thesatb_p_slot;
        let mut var_thesatb_p_rv: f64 = *var_thesatb_p_rv_slot;
        let mut var_thesatg_p: f64 = *var_thesatg_p_slot;
        let mut var_thesatg_p_rv: f64 = *var_thesatg_p_rv_slot;
        let mut var_xcor_p: f64 = *var_xcor_p_slot;
        let mut var_xcor_p_rv: f64 = *var_xcor_p_rv_slot;

        let assign7480_e6235: f64 = if (((param_given[532] || param_given[533]) || param_given[534]) || param_given[535]) { 1.0 } else { 0.0 };
        var_guard68 = assign7480_e6235;
        var_guard68_rv = 0.0;

        let (assign7490_e6253,) = {
    if ((var_guard36 != 0.0) && (var_guard68 != 0.0)) {
        let assign7490_e6242: f64 = (p.p533 * var_ile);
        let assign7490_e6243: f64 = (p.p532 + assign7490_e6242);
        let assign7490_e6246: f64 = (p.p534 * var_iwe);
        let assign7490_e6247: f64 = (assign7490_e6243 + assign7490_e6246);
        let assign7490_e6250: f64 = (p.p535 * var_iae);
        let assign7490_e6251: f64 = (assign7490_e6247 + assign7490_e6250);
        (assign7490_e6251,)
    } else {
        (var_psced_p,)
    }
};
        var_psced_p = assign7490_e6253;
        var_psced_p_rv = 0.0;

        let assign7500_e6272: f64 = if (((param_given[528] || param_given[529]) || param_given[530]) || param_given[531]) { 1.0 } else { 0.0 };
        var_guard69 = assign7500_e6272;
        var_guard69_rv = 0.0;

        let (assign7510_e6290,) = {
    if ((var_guard36 != 0.0) && (var_guard69 != 0.0)) {
        let assign7510_e6279: f64 = (p.p529 * var_ile);
        let assign7510_e6280: f64 = (p.p528 + assign7510_e6279);
        let assign7510_e6283: f64 = (p.p530 * var_iwe);
        let assign7510_e6284: f64 = (assign7510_e6280 + assign7510_e6283);
        let assign7510_e6287: f64 = (p.p531 * var_iae);
        let assign7510_e6288: f64 = (assign7510_e6284 + assign7510_e6287);
        (assign7510_e6288,)
    } else {
        (var_psceb_p,)
    }
};
        var_psceb_p = assign7510_e6290;
        var_psceb_p_rv = 0.0;

        let assign7520_e6309: f64 = if (((param_given[536] || param_given[537]) || param_given[538]) || param_given[539]) { 1.0 } else { 0.0 };
        var_guard70 = assign7520_e6309;
        var_guard70_rv = 0.0;

        let (assign7530_e6331,) = {
    if ((var_guard36 != 0.0) && (var_guard70 != 0.0)) {
        let assign7530_e6315: f64 = (var_we / var_le);
        let assign7530_e6319: f64 = (p.p537 * var_ile);
        let assign7530_e6320: f64 = (p.p536 + assign7530_e6319);
        let assign7530_e6323: f64 = (p.p538 * var_iwe);
        let assign7530_e6324: f64 = (assign7530_e6320 + assign7530_e6323);
        let assign7530_e6327: f64 = (p.p539 * var_iae);
        let assign7530_e6328: f64 = (assign7530_e6324 + assign7530_e6327);
        let assign7530_e6329: f64 = (assign7530_e6315 * assign7530_e6328);
        (assign7530_e6329,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign7530_e6331;
        var_betn_p_rv = 0.0;

        let assign7540_e6350: f64 = if (((param_given[540] || param_given[541]) || param_given[542]) || param_given[543]) { 1.0 } else { 0.0 };
        var_guard71 = assign7540_e6350;
        var_guard71_rv = 0.0;

        let (assign7550_e6368,) = {
    if ((var_guard36 != 0.0) && (var_guard71 != 0.0)) {
        let assign7550_e6357: f64 = (p.p541 * var_ile);
        let assign7550_e6358: f64 = (p.p540 + assign7550_e6357);
        let assign7550_e6361: f64 = (p.p542 * var_iwe);
        let assign7550_e6362: f64 = (assign7550_e6358 + assign7550_e6361);
        let assign7550_e6365: f64 = (p.p543 * var_iae);
        let assign7550_e6366: f64 = (assign7550_e6362 + assign7550_e6365);
        (assign7550_e6366,)
    } else {
        (var_stbet_p,)
    }
};
        var_stbet_p = assign7550_e6368;
        var_stbet_p_rv = 0.0;

        let assign7560_e6387: f64 = if (((param_given[544] || param_given[545]) || param_given[546]) || param_given[547]) { 1.0 } else { 0.0 };
        var_guard72 = assign7560_e6387;
        var_guard72_rv = 0.0;

        let (assign7570_e6405,) = {
    if ((var_guard36 != 0.0) && (var_guard72 != 0.0)) {
        let assign7570_e6394: f64 = (p.p545 * var_ile);
        let assign7570_e6395: f64 = (p.p544 + assign7570_e6394);
        let assign7570_e6398: f64 = (p.p546 * var_iwe);
        let assign7570_e6399: f64 = (assign7570_e6395 + assign7570_e6398);
        let assign7570_e6402: f64 = (p.p547 * var_iae);
        let assign7570_e6403: f64 = (assign7570_e6399 + assign7570_e6402);
        (assign7570_e6403,)
    } else {
        (var_mue_p,)
    }
};
        var_mue_p = assign7570_e6405;
        var_mue_p_rv = 0.0;

        let assign7580_e6424: f64 = if (((param_given[548] || param_given[549]) || param_given[550]) || param_given[551]) { 1.0 } else { 0.0 };
        var_guard73 = assign7580_e6424;
        var_guard73_rv = 0.0;

        let (assign7590_e6442,) = {
    if ((var_guard36 != 0.0) && (var_guard73 != 0.0)) {
        let assign7590_e6431: f64 = (p.p549 * var_ile);
        let assign7590_e6432: f64 = (p.p548 + assign7590_e6431);
        let assign7590_e6435: f64 = (p.p550 * var_iwe);
        let assign7590_e6436: f64 = (assign7590_e6432 + assign7590_e6435);
        let assign7590_e6439: f64 = (p.p551 * var_iae);
        let assign7590_e6440: f64 = (assign7590_e6436 + assign7590_e6439);
        (assign7590_e6440,)
    } else {
        (var_themu_p,)
    }
};
        var_themu_p = assign7590_e6442;
        var_themu_p_rv = 0.0;

        let assign7600_e6461: f64 = if (((param_given[552] || param_given[553]) || param_given[554]) || param_given[555]) { 1.0 } else { 0.0 };
        var_guard74 = assign7600_e6461;
        var_guard74_rv = 0.0;

        let (assign7610_e6479,) = {
    if ((var_guard36 != 0.0) && (var_guard74 != 0.0)) {
        let assign7610_e6468: f64 = (p.p553 * var_ile);
        let assign7610_e6469: f64 = (p.p552 + assign7610_e6468);
        let assign7610_e6472: f64 = (p.p554 * var_iwe);
        let assign7610_e6473: f64 = (assign7610_e6469 + assign7610_e6472);
        let assign7610_e6476: f64 = (p.p555 * var_iae);
        let assign7610_e6477: f64 = (assign7610_e6473 + assign7610_e6476);
        (assign7610_e6477,)
    } else {
        (var_cs_p,)
    }
};
        var_cs_p = assign7610_e6479;
        var_cs_p_rv = 0.0;

        let assign7620_e6498: f64 = if (((param_given[556] || param_given[557]) || param_given[558]) || param_given[559]) { 1.0 } else { 0.0 };
        var_guard75 = assign7620_e6498;
        var_guard75_rv = 0.0;

        let (assign7630_e6516,) = {
    if ((var_guard36 != 0.0) && (var_guard75 != 0.0)) {
        let assign7630_e6505: f64 = (p.p557 * var_ile);
        let assign7630_e6506: f64 = (p.p556 + assign7630_e6505);
        let assign7630_e6509: f64 = (p.p558 * var_iwe);
        let assign7630_e6510: f64 = (assign7630_e6506 + assign7630_e6509);
        let assign7630_e6513: f64 = (p.p559 * var_iae);
        let assign7630_e6514: f64 = (assign7630_e6510 + assign7630_e6513);
        (assign7630_e6514,)
    } else {
        (var_thecs_p,)
    }
};
        var_thecs_p = assign7630_e6516;
        var_thecs_p_rv = 0.0;

        let assign7640_e6535: f64 = if (((param_given[560] || param_given[561]) || param_given[562]) || param_given[563]) { 1.0 } else { 0.0 };
        var_guard76 = assign7640_e6535;
        var_guard76_rv = 0.0;

        let (assign7650_e6553,) = {
    if ((var_guard36 != 0.0) && (var_guard76 != 0.0)) {
        let assign7650_e6542: f64 = (p.p561 * var_ile);
        let assign7650_e6543: f64 = (p.p560 + assign7650_e6542);
        let assign7650_e6546: f64 = (p.p562 * var_iwe);
        let assign7650_e6547: f64 = (assign7650_e6543 + assign7650_e6546);
        let assign7650_e6550: f64 = (p.p563 * var_iae);
        let assign7650_e6551: f64 = (assign7650_e6547 + assign7650_e6550);
        (assign7650_e6551,)
    } else {
        (var_xcor_p,)
    }
};
        var_xcor_p = assign7650_e6553;
        var_xcor_p_rv = 0.0;

        let assign7660_e6572: f64 = if (((param_given[564] || param_given[565]) || param_given[566]) || param_given[567]) { 1.0 } else { 0.0 };
        var_guard77 = assign7660_e6572;
        var_guard77_rv = 0.0;

        let (assign7670_e6592,) = {
    if ((var_guard36 != 0.0) && (var_guard77 != 0.0)) {
        let assign7670_e6580: f64 = (p.p565 * var_ile);
        let assign7670_e6581: f64 = (p.p564 + assign7670_e6580);
        let assign7670_e6584: f64 = (p.p566 * var_iwe);
        let assign7670_e6585: f64 = (assign7670_e6581 + assign7670_e6584);
        let assign7670_e6588: f64 = (p.p567 * var_iae);
        let assign7670_e6589: f64 = (assign7670_e6585 + assign7670_e6588);
        let assign7670_e6590: f64 = (var_iwe * assign7670_e6589);
        (assign7670_e6590,)
    } else {
        (var_rs_p,)
    }
};
        var_rs_p = assign7670_e6592;
        var_rs_p_rv = 0.0;

        let assign7680_e6611: f64 = if (((param_given[568] || param_given[569]) || param_given[570]) || param_given[571]) { 1.0 } else { 0.0 };
        var_guard78 = assign7680_e6611;
        var_guard78_rv = 0.0;

        let (assign7690_e6629,) = {
    if ((var_guard36 != 0.0) && (var_guard78 != 0.0)) {
        let assign7690_e6618: f64 = (p.p569 * var_ile);
        let assign7690_e6619: f64 = (p.p568 + assign7690_e6618);
        let assign7690_e6622: f64 = (p.p570 * var_iwe);
        let assign7690_e6623: f64 = (assign7690_e6619 + assign7690_e6622);
        let assign7690_e6626: f64 = (p.p571 * var_iae);
        let assign7690_e6627: f64 = (assign7690_e6623 + assign7690_e6626);
        (assign7690_e6627,)
    } else {
        (var_strs_p,)
    }
};
        var_strs_p = assign7690_e6629;
        var_strs_p_rv = 0.0;

        let assign7700_e6648: f64 = if (((param_given[572] || param_given[573]) || param_given[574]) || param_given[575]) { 1.0 } else { 0.0 };
        var_guard79 = assign7700_e6648;
        var_guard79_rv = 0.0;

        let (assign7710_e6666,) = {
    if ((var_guard36 != 0.0) && (var_guard79 != 0.0)) {
        let assign7710_e6655: f64 = (p.p573 * var_ile);
        let assign7710_e6656: f64 = (p.p572 + assign7710_e6655);
        let assign7710_e6659: f64 = (p.p574 * var_iwe);
        let assign7710_e6660: f64 = (assign7710_e6656 + assign7710_e6659);
        let assign7710_e6663: f64 = (p.p575 * var_iae);
        let assign7710_e6664: f64 = (assign7710_e6660 + assign7710_e6663);
        (assign7710_e6664,)
    } else {
        (var_rsb_p,)
    }
};
        var_rsb_p = assign7710_e6666;
        var_rsb_p_rv = 0.0;

        let assign7720_e6685: f64 = if (((param_given[576] || param_given[577]) || param_given[578]) || param_given[579]) { 1.0 } else { 0.0 };
        var_guard80 = assign7720_e6685;
        var_guard80_rv = 0.0;

        let (assign7730_e6703,) = {
    if ((var_guard36 != 0.0) && (var_guard80 != 0.0)) {
        let assign7730_e6692: f64 = (p.p577 * var_ile);
        let assign7730_e6693: f64 = (p.p576 + assign7730_e6692);
        let assign7730_e6696: f64 = (p.p578 * var_iwe);
        let assign7730_e6697: f64 = (assign7730_e6693 + assign7730_e6696);
        let assign7730_e6700: f64 = (p.p579 * var_iae);
        let assign7730_e6701: f64 = (assign7730_e6697 + assign7730_e6700);
        (assign7730_e6701,)
    } else {
        (var_rsg_p,)
    }
};
        var_rsg_p = assign7730_e6703;
        var_rsg_p_rv = 0.0;

        let assign7740_e6722: f64 = if (((param_given[580] || param_given[581]) || param_given[582]) || param_given[583]) { 1.0 } else { 0.0 };
        var_guard81 = assign7740_e6722;
        var_guard81_rv = 0.0;

        let (assign7750_e6742,) = {
    if ((var_guard36 != 0.0) && (var_guard81 != 0.0)) {
        let assign7750_e6730: f64 = (p.p581 * var_ile);
        let assign7750_e6731: f64 = (p.p580 + assign7750_e6730);
        let assign7750_e6734: f64 = (p.p582 * var_iwe);
        let assign7750_e6735: f64 = (assign7750_e6731 + assign7750_e6734);
        let assign7750_e6738: f64 = (p.p583 * var_iae);
        let assign7750_e6739: f64 = (assign7750_e6735 + assign7750_e6738);
        let assign7750_e6740: f64 = (var_ile * assign7750_e6739);
        (assign7750_e6740,)
    } else {
        (var_thesat_p,)
    }
};
        var_thesat_p = assign7750_e6742;
        var_thesat_p_rv = 0.0;

        let assign7760_e6761: f64 = if (((param_given[584] || param_given[585]) || param_given[586]) || param_given[587]) { 1.0 } else { 0.0 };
        var_guard82 = assign7760_e6761;
        var_guard82_rv = 0.0;

        let (assign7770_e6779,) = {
    if ((var_guard36 != 0.0) && (var_guard82 != 0.0)) {
        let assign7770_e6768: f64 = (p.p585 * var_ile);
        let assign7770_e6769: f64 = (p.p584 + assign7770_e6768);
        let assign7770_e6772: f64 = (p.p586 * var_iwe);
        let assign7770_e6773: f64 = (assign7770_e6769 + assign7770_e6772);
        let assign7770_e6776: f64 = (p.p587 * var_iae);
        let assign7770_e6777: f64 = (assign7770_e6773 + assign7770_e6776);
        (assign7770_e6777,)
    } else {
        (var_stthesat_p,)
    }
};
        var_stthesat_p = assign7770_e6779;
        var_stthesat_p_rv = 0.0;

        let assign7780_e6798: f64 = if (((param_given[588] || param_given[589]) || param_given[590]) || param_given[591]) { 1.0 } else { 0.0 };
        var_guard83 = assign7780_e6798;
        var_guard83_rv = 0.0;

        let (assign7790_e6816,) = {
    if ((var_guard36 != 0.0) && (var_guard83 != 0.0)) {
        let assign7790_e6805: f64 = (p.p589 * var_ile);
        let assign7790_e6806: f64 = (p.p588 + assign7790_e6805);
        let assign7790_e6809: f64 = (p.p590 * var_iwe);
        let assign7790_e6810: f64 = (assign7790_e6806 + assign7790_e6809);
        let assign7790_e6813: f64 = (p.p591 * var_iae);
        let assign7790_e6814: f64 = (assign7790_e6810 + assign7790_e6813);
        (assign7790_e6814,)
    } else {
        (var_thesatb_p,)
    }
};
        var_thesatb_p = assign7790_e6816;
        var_thesatb_p_rv = 0.0;

        let assign7800_e6835: f64 = if (((param_given[592] || param_given[593]) || param_given[594]) || param_given[595]) { 1.0 } else { 0.0 };
        var_guard84 = assign7800_e6835;
        var_guard84_rv = 0.0;

        let (assign7810_e6853,) = {
    if ((var_guard36 != 0.0) && (var_guard84 != 0.0)) {
        let assign7810_e6842: f64 = (p.p593 * var_ile);
        let assign7810_e6843: f64 = (p.p592 + assign7810_e6842);
        let assign7810_e6846: f64 = (p.p594 * var_iwe);
        let assign7810_e6847: f64 = (assign7810_e6843 + assign7810_e6846);
        let assign7810_e6850: f64 = (p.p595 * var_iae);
        let assign7810_e6851: f64 = (assign7810_e6847 + assign7810_e6850);
        (assign7810_e6851,)
    } else {
        (var_thesatg_p,)
    }
};
        var_thesatg_p = assign7810_e6853;
        var_thesatg_p_rv = 0.0;

        let assign7820_e6872: f64 = if (((param_given[596] || param_given[597]) || param_given[598]) || param_given[599]) { 1.0 } else { 0.0 };
        var_guard85 = assign7820_e6872;
        var_guard85_rv = 0.0;

        let (assign7830_e6890,) = {
    if ((var_guard36 != 0.0) && (var_guard85 != 0.0)) {
        let assign7830_e6879: f64 = (p.p597 * var_ile);
        let assign7830_e6880: f64 = (p.p596 + assign7830_e6879);
        let assign7830_e6883: f64 = (p.p598 * var_iwe);
        let assign7830_e6884: f64 = (assign7830_e6880 + assign7830_e6883);
        let assign7830_e6887: f64 = (p.p599 * var_iae);
        let assign7830_e6888: f64 = (assign7830_e6884 + assign7830_e6887);
        (assign7830_e6888,)
    } else {
        (var_ax_p,)
    }
};
        var_ax_p = assign7830_e6890;
        var_ax_p_rv = 0.0;

        let assign7840_e6909: f64 = if (((param_given[600] || param_given[601]) || param_given[602]) || param_given[603]) { 1.0 } else { 0.0 };
        var_guard86 = assign7840_e6909;
        var_guard86_rv = 0.0;

        let (assign7850_e6929,) = {
    if ((var_guard36 != 0.0) && (var_guard86 != 0.0)) {
        let assign7850_e6917: f64 = (p.p601 * var_ile);
        let assign7850_e6918: f64 = (p.p600 + assign7850_e6917);
        let assign7850_e6921: f64 = (p.p602 * var_iwe);
        let assign7850_e6922: f64 = (assign7850_e6918 + assign7850_e6921);
        let assign7850_e6925: f64 = (p.p603 * var_iae);
        let assign7850_e6926: f64 = (assign7850_e6922 + assign7850_e6925);
        let assign7850_e6927: f64 = (var_ile * assign7850_e6926);
        (assign7850_e6927,)
    } else {
        (var_alp_p,)
    }
};
        var_alp_p = assign7850_e6929;
        var_alp_p_rv = 0.0;

        let assign7860_e6948: f64 = if (((param_given[604] || param_given[605]) || param_given[606]) || param_given[607]) { 1.0 } else { 0.0 };
        var_guard87 = assign7860_e6948;
        var_guard87_rv = 0.0;

        let (assign7870_e6966,) = {
    if ((var_guard36 != 0.0) && (var_guard87 != 0.0)) {
        let assign7870_e6955: f64 = (p.p605 * var_ile);
        let assign7870_e6956: f64 = (p.p604 + assign7870_e6955);
        let assign7870_e6959: f64 = (p.p606 * var_iwe);
        let assign7870_e6960: f64 = (assign7870_e6956 + assign7870_e6959);
        let assign7870_e6963: f64 = (p.p607 * var_iae);
        let assign7870_e6964: f64 = (assign7870_e6960 + assign7870_e6963);
        (assign7870_e6964,)
    } else {
        (var_alp1_p,)
    }
};
        var_alp1_p = assign7870_e6966;
        var_alp1_p_rv = 0.0;

        let assign7880_e6985: f64 = if (((param_given[608] || param_given[609]) || param_given[610]) || param_given[611]) { 1.0 } else { 0.0 };
        var_guard88 = assign7880_e6985;
        var_guard88_rv = 0.0;

        let (assign7890_e7003,) = {
    if ((var_guard36 != 0.0) && (var_guard88 != 0.0)) {
        let assign7890_e6992: f64 = (p.p609 * var_ile);
        let assign7890_e6993: f64 = (p.p608 + assign7890_e6992);
        let assign7890_e6996: f64 = (p.p610 * var_iwe);
        let assign7890_e6997: f64 = (assign7890_e6993 + assign7890_e6996);
        let assign7890_e7000: f64 = (p.p611 * var_iae);
        let assign7890_e7001: f64 = (assign7890_e6997 + assign7890_e7000);
        (assign7890_e7001,)
    } else {
        (var_alp2_p,)
    }
};
        var_alp2_p = assign7890_e7003;
        var_alp2_p_rv = 0.0;

        let assign7900_e7022: f64 = if (((param_given[612] || param_given[613]) || param_given[614]) || param_given[615]) { 1.0 } else { 0.0 };
        var_guard89 = assign7900_e7022;
        var_guard89_rv = 0.0;

        let (assign7910_e7040,) = {
    if ((var_guard36 != 0.0) && (var_guard89 != 0.0)) {
        let assign7910_e7029: f64 = (p.p613 * var_ile);
        let assign7910_e7030: f64 = (p.p612 + assign7910_e7029);
        let assign7910_e7033: f64 = (p.p614 * var_iwe);
        let assign7910_e7034: f64 = (assign7910_e7030 + assign7910_e7033);
        let assign7910_e7037: f64 = (p.p615 * var_iae);
        let assign7910_e7038: f64 = (assign7910_e7034 + assign7910_e7037);
        (assign7910_e7038,)
    } else {
        (var_a1_p,)
    }
};
        var_a1_p = assign7910_e7040;
        var_a1_p_rv = 0.0;

        let assign7920_e7059: f64 = if (((param_given[616] || param_given[617]) || param_given[618]) || param_given[619]) { 1.0 } else { 0.0 };
        var_guard90 = assign7920_e7059;
        var_guard90_rv = 0.0;

        let (assign7930_e7077,) = {
    if ((var_guard36 != 0.0) && (var_guard90 != 0.0)) {
        let assign7930_e7066: f64 = (p.p617 * var_ile);
        let assign7930_e7067: f64 = (p.p616 + assign7930_e7066);
        let assign7930_e7070: f64 = (p.p618 * var_iwe);
        let assign7930_e7071: f64 = (assign7930_e7067 + assign7930_e7070);
        let assign7930_e7074: f64 = (p.p619 * var_iae);
        let assign7930_e7075: f64 = (assign7930_e7071 + assign7930_e7074);
        (assign7930_e7075,)
    } else {
        (var_sta2_p,)
    }
};
        var_sta2_p = assign7930_e7077;
        var_sta2_p_rv = 0.0;

        let assign7940_e7096: f64 = if (((param_given[620] || param_given[621]) || param_given[622]) || param_given[623]) { 1.0 } else { 0.0 };
        var_guard91 = assign7940_e7096;
        var_guard91_rv = 0.0;

        let (assign7950_e7114,) = {
    if ((var_guard36 != 0.0) && (var_guard91 != 0.0)) {
        let assign7950_e7103: f64 = (p.p621 * var_ile);
        let assign7950_e7104: f64 = (p.p620 + assign7950_e7103);
        let assign7950_e7107: f64 = (p.p622 * var_iwe);
        let assign7950_e7108: f64 = (assign7950_e7104 + assign7950_e7107);
        let assign7950_e7111: f64 = (p.p623 * var_iae);
        let assign7950_e7112: f64 = (assign7950_e7108 + assign7950_e7111);
        (assign7950_e7112,)
    } else {
        (var_a3_p,)
    }
};
        var_a3_p = assign7950_e7114;
        var_a3_p_rv = 0.0;

        let assign7960_e7133: f64 = if (((param_given[624] || param_given[625]) || param_given[626]) || param_given[627]) { 1.0 } else { 0.0 };
        var_guard92 = assign7960_e7133;
        var_guard92_rv = 0.0;

        let (assign7970_e7151,) = {
    if ((var_guard36 != 0.0) && (var_guard92 != 0.0)) {
        let assign7970_e7140: f64 = (p.p625 * var_ile);
        let assign7970_e7141: f64 = (p.p624 + assign7970_e7140);
        let assign7970_e7144: f64 = (p.p626 * var_iwe);
        let assign7970_e7145: f64 = (assign7970_e7141 + assign7970_e7144);
        let assign7970_e7148: f64 = (p.p627 * var_iae);
        let assign7970_e7149: f64 = (assign7970_e7145 + assign7970_e7148);
        (assign7970_e7149,)
    } else {
        (var_a4_p,)
    }
};
        var_a4_p = assign7970_e7151;
        var_a4_p_rv = 0.0;

        let assign7980_e7170: f64 = if (((param_given[628] || param_given[629]) || param_given[630]) || param_given[631]) { 1.0 } else { 0.0 };
        var_guard93 = assign7980_e7170;
        var_guard93_rv = 0.0;

        *var_a1_p_slot = var_a1_p;
        *var_a1_p_rv_slot = var_a1_p_rv;
        *var_a3_p_slot = var_a3_p;
        *var_a3_p_rv_slot = var_a3_p_rv;
        *var_a4_p_slot = var_a4_p;
        *var_a4_p_rv_slot = var_a4_p_rv;
        *var_alp1_p_slot = var_alp1_p;
        *var_alp1_p_rv_slot = var_alp1_p_rv;
        *var_alp2_p_slot = var_alp2_p;
        *var_alp2_p_rv_slot = var_alp2_p_rv;
        *var_alp_p_slot = var_alp_p;
        *var_alp_p_rv_slot = var_alp_p_rv;
        *var_ax_p_slot = var_ax_p;
        *var_ax_p_rv_slot = var_ax_p_rv;
        *var_betn_p_slot = var_betn_p;
        *var_betn_p_rv_slot = var_betn_p_rv;
        *var_cs_p_slot = var_cs_p;
        *var_cs_p_rv_slot = var_cs_p_rv;
        *var_guard68_slot = var_guard68;
        *var_guard68_rv_slot = var_guard68_rv;
        *var_guard69_slot = var_guard69;
        *var_guard69_rv_slot = var_guard69_rv;
        *var_guard70_slot = var_guard70;
        *var_guard70_rv_slot = var_guard70_rv;
        *var_guard71_slot = var_guard71;
        *var_guard71_rv_slot = var_guard71_rv;
        *var_guard72_slot = var_guard72;
        *var_guard72_rv_slot = var_guard72_rv;
        *var_guard73_slot = var_guard73;
        *var_guard73_rv_slot = var_guard73_rv;
        *var_guard74_slot = var_guard74;
        *var_guard74_rv_slot = var_guard74_rv;
        *var_guard75_slot = var_guard75;
        *var_guard75_rv_slot = var_guard75_rv;
        *var_guard76_slot = var_guard76;
        *var_guard76_rv_slot = var_guard76_rv;
        *var_guard77_slot = var_guard77;
        *var_guard77_rv_slot = var_guard77_rv;
        *var_guard78_slot = var_guard78;
        *var_guard78_rv_slot = var_guard78_rv;
        *var_guard79_slot = var_guard79;
        *var_guard79_rv_slot = var_guard79_rv;
        *var_guard80_slot = var_guard80;
        *var_guard80_rv_slot = var_guard80_rv;
        *var_guard81_slot = var_guard81;
        *var_guard81_rv_slot = var_guard81_rv;
        *var_guard82_slot = var_guard82;
        *var_guard82_rv_slot = var_guard82_rv;
        *var_guard83_slot = var_guard83;
        *var_guard83_rv_slot = var_guard83_rv;
        *var_guard84_slot = var_guard84;
        *var_guard84_rv_slot = var_guard84_rv;
        *var_guard85_slot = var_guard85;
        *var_guard85_rv_slot = var_guard85_rv;
        *var_guard86_slot = var_guard86;
        *var_guard86_rv_slot = var_guard86_rv;
        *var_guard87_slot = var_guard87;
        *var_guard87_rv_slot = var_guard87_rv;
        *var_guard88_slot = var_guard88;
        *var_guard88_rv_slot = var_guard88_rv;
        *var_guard89_slot = var_guard89;
        *var_guard89_rv_slot = var_guard89_rv;
        *var_guard90_slot = var_guard90;
        *var_guard90_rv_slot = var_guard90_rv;
        *var_guard91_slot = var_guard91;
        *var_guard91_rv_slot = var_guard91_rv;
        *var_guard92_slot = var_guard92;
        *var_guard92_rv_slot = var_guard92_rv;
        *var_guard93_slot = var_guard93;
        *var_guard93_rv_slot = var_guard93_rv;
        *var_mue_p_slot = var_mue_p;
        *var_mue_p_rv_slot = var_mue_p_rv;
        *var_psceb_p_slot = var_psceb_p;
        *var_psceb_p_rv_slot = var_psceb_p_rv;
        *var_psced_p_slot = var_psced_p;
        *var_psced_p_rv_slot = var_psced_p_rv;
        *var_rs_p_slot = var_rs_p;
        *var_rs_p_rv_slot = var_rs_p_rv;
        *var_rsb_p_slot = var_rsb_p;
        *var_rsb_p_rv_slot = var_rsb_p_rv;
        *var_rsg_p_slot = var_rsg_p;
        *var_rsg_p_rv_slot = var_rsg_p_rv;
        *var_sta2_p_slot = var_sta2_p;
        *var_sta2_p_rv_slot = var_sta2_p_rv;
        *var_stbet_p_slot = var_stbet_p;
        *var_stbet_p_rv_slot = var_stbet_p_rv;
        *var_strs_p_slot = var_strs_p;
        *var_strs_p_rv_slot = var_strs_p_rv;
        *var_stthesat_p_slot = var_stthesat_p;
        *var_stthesat_p_rv_slot = var_stthesat_p_rv;
        *var_thecs_p_slot = var_thecs_p;
        *var_thecs_p_rv_slot = var_thecs_p_rv;
        *var_themu_p_slot = var_themu_p;
        *var_themu_p_rv_slot = var_themu_p_rv;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesat_p_rv_slot = var_thesat_p_rv;
        *var_thesatb_p_slot = var_thesatb_p;
        *var_thesatb_p_rv_slot = var_thesatb_p_rv;
        *var_thesatg_p_slot = var_thesatg_p;
        *var_thesatg_p_rv_slot = var_thesatg_p_rv;
        *var_xcor_p_slot = var_xcor_p;
        *var_xcor_p_rv_slot = var_xcor_p_rv;
    }

    pub(super) fn stamp_reactive_block_12(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard36: f64,
        var_guard93: f64,
        var_iae: f64,
        var_iiae: f64,
        var_iiwe: f64,
        var_iiwecv: f64,
        var_ile: f64,
        var_iwe: f64,
        var_lecv: f64,
        var_agidl_p_slot: &mut f64,
        var_agidl_p_rv_slot: &mut f64,
        var_agidld_p_slot: &mut f64,
        var_agidld_p_rv_slot: &mut f64,
        var_alp1ac_p_slot: &mut f64,
        var_alp1ac_p_rv_slot: &mut f64,
        var_alpac_p_slot: &mut f64,
        var_alpac_p_rv_slot: &mut f64,
        var_axac_p_slot: &mut f64,
        var_axac_p_rv_slot: &mut f64,
        var_cox_p_slot: &mut f64,
        var_cox_p_rv_slot: &mut f64,
        var_delvtac_p_slot: &mut f64,
        var_delvtac_p_rv_slot: &mut f64,
        var_facneffac_p_slot: &mut f64,
        var_facneffac_p_rv_slot: &mut f64,
        var_guard100_slot: &mut f64,
        var_guard100_rv_slot: &mut f64,
        var_guard101_slot: &mut f64,
        var_guard101_rv_slot: &mut f64,
        var_guard102_slot: &mut f64,
        var_guard102_rv_slot: &mut f64,
        var_guard103_slot: &mut f64,
        var_guard103_rv_slot: &mut f64,
        var_guard104_slot: &mut f64,
        var_guard104_rv_slot: &mut f64,
        var_guard105_slot: &mut f64,
        var_guard105_rv_slot: &mut f64,
        var_guard106_slot: &mut f64,
        var_guard106_rv_slot: &mut f64,
        var_guard107_slot: &mut f64,
        var_guard107_rv_slot: &mut f64,
        var_guard108_slot: &mut f64,
        var_guard108_rv_slot: &mut f64,
        var_guard109_slot: &mut f64,
        var_guard109_rv_slot: &mut f64,
        var_guard110_slot: &mut f64,
        var_guard110_rv_slot: &mut f64,
        var_guard111_slot: &mut f64,
        var_guard111_rv_slot: &mut f64,
        var_guard112_slot: &mut f64,
        var_guard112_rv_slot: &mut f64,
        var_guard113_slot: &mut f64,
        var_guard113_rv_slot: &mut f64,
        var_guard114_slot: &mut f64,
        var_guard114_rv_slot: &mut f64,
        var_guard115_slot: &mut f64,
        var_guard115_rv_slot: &mut f64,
        var_guard116_slot: &mut f64,
        var_guard116_rv_slot: &mut f64,
        var_guard94_slot: &mut f64,
        var_guard94_rv_slot: &mut f64,
        var_guard95_slot: &mut f64,
        var_guard95_rv_slot: &mut f64,
        var_guard96_slot: &mut f64,
        var_guard96_rv_slot: &mut f64,
        var_guard97_slot: &mut f64,
        var_guard97_rv_slot: &mut f64,
        var_guard98_slot: &mut f64,
        var_guard98_rv_slot: &mut f64,
        var_guard99_slot: &mut f64,
        var_guard99_rv_slot: &mut f64,
        var_iginv_p_slot: &mut f64,
        var_iginv_p_rv_slot: &mut f64,
        var_igov_p_slot: &mut f64,
        var_igov_p_rv_slot: &mut f64,
        var_igovd_p_slot: &mut f64,
        var_igovd_p_rv_slot: &mut f64,
        var_plparam_i_slot: &mut f64,
        var_plparam_i_rv_slot: &mut f64,
        var_plwparam_i_slot: &mut f64,
        var_plwparam_i_rv_slot: &mut f64,
        var_poparam_i_slot: &mut f64,
        var_poparam_i_rv_slot: &mut f64,
        var_pwparam_i_slot: &mut f64,
        var_pwparam_i_rv_slot: &mut f64,
        var_stbgidl_p_slot: &mut f64,
        var_stbgidl_p_rv_slot: &mut f64,
        var_stbgidld_p_slot: &mut f64,
        var_stbgidld_p_rv_slot: &mut f64,
        var_stig_p_slot: &mut f64,
        var_stig_p_rv_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_rv_slot: &mut f64,
    ) {
        let mut var_agidl_p: f64 = *var_agidl_p_slot;
        let mut var_agidl_p_rv: f64 = *var_agidl_p_rv_slot;
        let mut var_agidld_p: f64 = *var_agidld_p_slot;
        let mut var_agidld_p_rv: f64 = *var_agidld_p_rv_slot;
        let mut var_alp1ac_p: f64 = *var_alp1ac_p_slot;
        let mut var_alp1ac_p_rv: f64 = *var_alp1ac_p_rv_slot;
        let mut var_alpac_p: f64 = *var_alpac_p_slot;
        let mut var_alpac_p_rv: f64 = *var_alpac_p_rv_slot;
        let mut var_axac_p: f64 = *var_axac_p_slot;
        let mut var_axac_p_rv: f64 = *var_axac_p_rv_slot;
        let mut var_cox_p: f64 = *var_cox_p_slot;
        let mut var_cox_p_rv: f64 = *var_cox_p_rv_slot;
        let mut var_delvtac_p: f64 = *var_delvtac_p_slot;
        let mut var_delvtac_p_rv: f64 = *var_delvtac_p_rv_slot;
        let mut var_facneffac_p: f64 = *var_facneffac_p_slot;
        let mut var_facneffac_p_rv: f64 = *var_facneffac_p_rv_slot;
        let mut var_guard100: f64 = *var_guard100_slot;
        let mut var_guard100_rv: f64 = *var_guard100_rv_slot;
        let mut var_guard101: f64 = *var_guard101_slot;
        let mut var_guard101_rv: f64 = *var_guard101_rv_slot;
        let mut var_guard102: f64 = *var_guard102_slot;
        let mut var_guard102_rv: f64 = *var_guard102_rv_slot;
        let mut var_guard103: f64 = *var_guard103_slot;
        let mut var_guard103_rv: f64 = *var_guard103_rv_slot;
        let mut var_guard104: f64 = *var_guard104_slot;
        let mut var_guard104_rv: f64 = *var_guard104_rv_slot;
        let mut var_guard105: f64 = *var_guard105_slot;
        let mut var_guard105_rv: f64 = *var_guard105_rv_slot;
        let mut var_guard106: f64 = *var_guard106_slot;
        let mut var_guard106_rv: f64 = *var_guard106_rv_slot;
        let mut var_guard107: f64 = *var_guard107_slot;
        let mut var_guard107_rv: f64 = *var_guard107_rv_slot;
        let mut var_guard108: f64 = *var_guard108_slot;
        let mut var_guard108_rv: f64 = *var_guard108_rv_slot;
        let mut var_guard109: f64 = *var_guard109_slot;
        let mut var_guard109_rv: f64 = *var_guard109_rv_slot;
        let mut var_guard110: f64 = *var_guard110_slot;
        let mut var_guard110_rv: f64 = *var_guard110_rv_slot;
        let mut var_guard111: f64 = *var_guard111_slot;
        let mut var_guard111_rv: f64 = *var_guard111_rv_slot;
        let mut var_guard112: f64 = *var_guard112_slot;
        let mut var_guard112_rv: f64 = *var_guard112_rv_slot;
        let mut var_guard113: f64 = *var_guard113_slot;
        let mut var_guard113_rv: f64 = *var_guard113_rv_slot;
        let mut var_guard114: f64 = *var_guard114_slot;
        let mut var_guard114_rv: f64 = *var_guard114_rv_slot;
        let mut var_guard115: f64 = *var_guard115_slot;
        let mut var_guard115_rv: f64 = *var_guard115_rv_slot;
        let mut var_guard116: f64 = *var_guard116_slot;
        let mut var_guard116_rv: f64 = *var_guard116_rv_slot;
        let mut var_guard94: f64 = *var_guard94_slot;
        let mut var_guard94_rv: f64 = *var_guard94_rv_slot;
        let mut var_guard95: f64 = *var_guard95_slot;
        let mut var_guard95_rv: f64 = *var_guard95_rv_slot;
        let mut var_guard96: f64 = *var_guard96_slot;
        let mut var_guard96_rv: f64 = *var_guard96_rv_slot;
        let mut var_guard97: f64 = *var_guard97_slot;
        let mut var_guard97_rv: f64 = *var_guard97_rv_slot;
        let mut var_guard98: f64 = *var_guard98_slot;
        let mut var_guard98_rv: f64 = *var_guard98_rv_slot;
        let mut var_guard99: f64 = *var_guard99_slot;
        let mut var_guard99_rv: f64 = *var_guard99_rv_slot;
        let mut var_iginv_p: f64 = *var_iginv_p_slot;
        let mut var_iginv_p_rv: f64 = *var_iginv_p_rv_slot;
        let mut var_igov_p: f64 = *var_igov_p_slot;
        let mut var_igov_p_rv: f64 = *var_igov_p_rv_slot;
        let mut var_igovd_p: f64 = *var_igovd_p_slot;
        let mut var_igovd_p_rv: f64 = *var_igovd_p_rv_slot;
        let mut var_plparam_i: f64 = *var_plparam_i_slot;
        let mut var_plparam_i_rv: f64 = *var_plparam_i_rv_slot;
        let mut var_plwparam_i: f64 = *var_plwparam_i_slot;
        let mut var_plwparam_i_rv: f64 = *var_plwparam_i_rv_slot;
        let mut var_poparam_i: f64 = *var_poparam_i_slot;
        let mut var_poparam_i_rv: f64 = *var_poparam_i_rv_slot;
        let mut var_pwparam_i: f64 = *var_pwparam_i_slot;
        let mut var_pwparam_i_rv: f64 = *var_pwparam_i_rv_slot;
        let mut var_stbgidl_p: f64 = *var_stbgidl_p_slot;
        let mut var_stbgidl_p_rv: f64 = *var_stbgidl_p_rv_slot;
        let mut var_stbgidld_p: f64 = *var_stbgidld_p_slot;
        let mut var_stbgidld_p_rv: f64 = *var_stbgidld_p_rv_slot;
        let mut var_stig_p: f64 = *var_stig_p_slot;
        let mut var_stig_p_rv: f64 = *var_stig_p_rv_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_rv: f64 = *var_thesatac_p_rv_slot;

        let (assign7990_e7190,) = {
    if ((var_guard36 != 0.0) && (var_guard93 != 0.0)) {
        let assign7990_e7178: f64 = (p.p629 * var_ile);
        let assign7990_e7179: f64 = (p.p628 + assign7990_e7178);
        let assign7990_e7182: f64 = (p.p630 * var_iwe);
        let assign7990_e7183: f64 = (assign7990_e7179 + assign7990_e7182);
        let assign7990_e7186: f64 = (p.p631 * var_iae);
        let assign7990_e7187: f64 = (assign7990_e7183 + assign7990_e7186);
        let assign7990_e7188: f64 = (var_iiae * assign7990_e7187);
        (assign7990_e7188,)
    } else {
        (var_iginv_p,)
    }
};
        var_iginv_p = assign7990_e7190;
        var_iginv_p_rv = 0.0;

        let assign8000_e7209: f64 = if (((param_given[632] || param_given[633]) || param_given[634]) || param_given[635]) { 1.0 } else { 0.0 };
        var_guard94 = assign8000_e7209;
        var_guard94_rv = 0.0;

        let (assign8010_e7229,) = {
    if ((var_guard36 != 0.0) && (var_guard94 != 0.0)) {
        let assign8010_e7217: f64 = (p.p633 * var_ile);
        let assign8010_e7218: f64 = (p.p632 + assign8010_e7217);
        let assign8010_e7221: f64 = (p.p634 * var_iwe);
        let assign8010_e7222: f64 = (assign8010_e7218 + assign8010_e7221);
        let assign8010_e7225: f64 = (p.p635 * var_iae);
        let assign8010_e7226: f64 = (assign8010_e7222 + assign8010_e7225);
        let assign8010_e7227: f64 = (var_iiwe * assign8010_e7226);
        (assign8010_e7227,)
    } else {
        (var_igov_p,)
    }
};
        var_igov_p = assign8010_e7229;
        var_igov_p_rv = 0.0;

        let assign8020_e7248: f64 = if (((param_given[636] || param_given[637]) || param_given[638]) || param_given[639]) { 1.0 } else { 0.0 };
        var_guard95 = assign8020_e7248;
        var_guard95_rv = 0.0;

        let (assign8030_e7268,) = {
    if ((var_guard36 != 0.0) && (var_guard95 != 0.0)) {
        let assign8030_e7256: f64 = (p.p637 * var_ile);
        let assign8030_e7257: f64 = (p.p636 + assign8030_e7256);
        let assign8030_e7260: f64 = (p.p638 * var_iwe);
        let assign8030_e7261: f64 = (assign8030_e7257 + assign8030_e7260);
        let assign8030_e7264: f64 = (p.p639 * var_iae);
        let assign8030_e7265: f64 = (assign8030_e7261 + assign8030_e7264);
        let assign8030_e7266: f64 = (var_iiwe * assign8030_e7265);
        (assign8030_e7266,)
    } else {
        (var_igovd_p,)
    }
};
        var_igovd_p = assign8030_e7268;
        var_igovd_p_rv = 0.0;

        let assign8040_e7287: f64 = if (((param_given[640] || param_given[641]) || param_given[642]) || param_given[643]) { 1.0 } else { 0.0 };
        var_guard96 = assign8040_e7287;
        var_guard96_rv = 0.0;

        let (assign8050_e7305,) = {
    if ((var_guard36 != 0.0) && (var_guard96 != 0.0)) {
        let assign8050_e7294: f64 = (p.p641 * var_ile);
        let assign8050_e7295: f64 = (p.p640 + assign8050_e7294);
        let assign8050_e7298: f64 = (p.p642 * var_iwe);
        let assign8050_e7299: f64 = (assign8050_e7295 + assign8050_e7298);
        let assign8050_e7302: f64 = (p.p643 * var_iae);
        let assign8050_e7303: f64 = (assign8050_e7299 + assign8050_e7302);
        (assign8050_e7303,)
    } else {
        (var_stig_p,)
    }
};
        var_stig_p = assign8050_e7305;
        var_stig_p_rv = 0.0;

        let assign8060_e7324: f64 = if (((param_given[644] || param_given[645]) || param_given[646]) || param_given[647]) { 1.0 } else { 0.0 };
        var_guard97 = assign8060_e7324;
        var_guard97_rv = 0.0;

        let (assign8070_e7344,) = {
    if ((var_guard36 != 0.0) && (var_guard97 != 0.0)) {
        let assign8070_e7332: f64 = (p.p645 * var_ile);
        let assign8070_e7333: f64 = (p.p644 + assign8070_e7332);
        let assign8070_e7336: f64 = (p.p646 * var_iwe);
        let assign8070_e7337: f64 = (assign8070_e7333 + assign8070_e7336);
        let assign8070_e7340: f64 = (p.p647 * var_iae);
        let assign8070_e7341: f64 = (assign8070_e7337 + assign8070_e7340);
        let assign8070_e7342: f64 = (var_iiwe * assign8070_e7341);
        (assign8070_e7342,)
    } else {
        (var_agidl_p,)
    }
};
        var_agidl_p = assign8070_e7344;
        var_agidl_p_rv = 0.0;

        let assign8080_e7363: f64 = if (((param_given[648] || param_given[649]) || param_given[650]) || param_given[651]) { 1.0 } else { 0.0 };
        var_guard98 = assign8080_e7363;
        var_guard98_rv = 0.0;

        let (assign8090_e7383,) = {
    if ((var_guard36 != 0.0) && (var_guard98 != 0.0)) {
        let assign8090_e7371: f64 = (p.p649 * var_ile);
        let assign8090_e7372: f64 = (p.p648 + assign8090_e7371);
        let assign8090_e7375: f64 = (p.p650 * var_iwe);
        let assign8090_e7376: f64 = (assign8090_e7372 + assign8090_e7375);
        let assign8090_e7379: f64 = (p.p651 * var_iae);
        let assign8090_e7380: f64 = (assign8090_e7376 + assign8090_e7379);
        let assign8090_e7381: f64 = (var_iiwe * assign8090_e7380);
        (assign8090_e7381,)
    } else {
        (var_agidld_p,)
    }
};
        var_agidld_p = assign8090_e7383;
        var_agidld_p_rv = 0.0;

        let assign8100_e7402: f64 = if (((param_given[652] || param_given[653]) || param_given[654]) || param_given[655]) { 1.0 } else { 0.0 };
        var_guard99 = assign8100_e7402;
        var_guard99_rv = 0.0;

        let (assign8110_e7420,) = {
    if ((var_guard36 != 0.0) && (var_guard99 != 0.0)) {
        let assign8110_e7409: f64 = (p.p653 * var_ile);
        let assign8110_e7410: f64 = (p.p652 + assign8110_e7409);
        let assign8110_e7413: f64 = (p.p654 * var_iwe);
        let assign8110_e7414: f64 = (assign8110_e7410 + assign8110_e7413);
        let assign8110_e7417: f64 = (p.p655 * var_iae);
        let assign8110_e7418: f64 = (assign8110_e7414 + assign8110_e7417);
        (assign8110_e7418,)
    } else {
        (var_stbgidl_p,)
    }
};
        var_stbgidl_p = assign8110_e7420;
        var_stbgidl_p_rv = 0.0;

        let assign8120_e7439: f64 = if (((param_given[656] || param_given[657]) || param_given[658]) || param_given[659]) { 1.0 } else { 0.0 };
        var_guard100 = assign8120_e7439;
        var_guard100_rv = 0.0;

        let (assign8130_e7457,) = {
    if ((var_guard36 != 0.0) && (var_guard100 != 0.0)) {
        let assign8130_e7446: f64 = (p.p657 * var_ile);
        let assign8130_e7447: f64 = (p.p656 + assign8130_e7446);
        let assign8130_e7450: f64 = (p.p658 * var_iwe);
        let assign8130_e7451: f64 = (assign8130_e7447 + assign8130_e7450);
        let assign8130_e7454: f64 = (p.p659 * var_iae);
        let assign8130_e7455: f64 = (assign8130_e7451 + assign8130_e7454);
        (assign8130_e7455,)
    } else {
        (var_stbgidld_p,)
    }
};
        var_stbgidld_p = assign8130_e7457;
        var_stbgidld_p_rv = 0.0;

        let assign8140_e7476: f64 = if (((param_given[660] || param_given[661]) || param_given[662]) || param_given[663]) { 1.0 } else { 0.0 };
        var_guard101 = assign8140_e7476;
        var_guard101_rv = 0.0;

        let (assign8150_e7500,) = {
    if ((var_guard36 != 0.0) && (var_guard101 != 0.0)) {
        let assign8150_e7482: f64 = (var_iiwecv * var_lecv);
        let assign8150_e7484: f64 = (assign8150_e7482 / 1e-6);
        let assign8150_e7488: f64 = (p.p661 * var_ile);
        let assign8150_e7489: f64 = (p.p660 + assign8150_e7488);
        let assign8150_e7492: f64 = (p.p662 * var_iwe);
        let assign8150_e7493: f64 = (assign8150_e7489 + assign8150_e7492);
        let assign8150_e7496: f64 = (p.p663 * var_iae);
        let assign8150_e7497: f64 = (assign8150_e7493 + assign8150_e7496);
        let assign8150_e7498: f64 = (assign8150_e7484 * assign8150_e7497);
        (assign8150_e7498,)
    } else {
        (var_cox_p,)
    }
};
        var_cox_p = assign8150_e7500;
        var_cox_p_rv = 0.0;

        let assign8160_e7519: f64 = if (((param_given[664] || param_given[665]) || param_given[666]) || param_given[667]) { 1.0 } else { 0.0 };
        var_guard102 = assign8160_e7519;
        var_guard102_rv = 0.0;

        let (assign8170_e7537,) = {
    if ((var_guard36 != 0.0) && (var_guard102 != 0.0)) {
        let assign8170_e7526: f64 = (p.p665 * var_ile);
        let assign8170_e7527: f64 = (p.p664 + assign8170_e7526);
        let assign8170_e7530: f64 = (p.p666 * var_iwe);
        let assign8170_e7531: f64 = (assign8170_e7527 + assign8170_e7530);
        let assign8170_e7534: f64 = (p.p667 * var_iae);
        let assign8170_e7535: f64 = (assign8170_e7531 + assign8170_e7534);
        (assign8170_e7535,)
    } else {
        (var_delvtac_p,)
    }
};
        var_delvtac_p = assign8170_e7537;
        var_delvtac_p_rv = 0.0;

        let assign8180_e7556: f64 = if (((param_given[668] || param_given[669]) || param_given[670]) || param_given[671]) { 1.0 } else { 0.0 };
        var_guard103 = assign8180_e7556;
        var_guard103_rv = 0.0;

        let (assign8190_e7574,) = {
    if ((var_guard36 != 0.0) && (var_guard103 != 0.0)) {
        let assign8190_e7563: f64 = (p.p669 * var_ile);
        let assign8190_e7564: f64 = (p.p668 + assign8190_e7563);
        let assign8190_e7567: f64 = (p.p670 * var_iwe);
        let assign8190_e7568: f64 = (assign8190_e7564 + assign8190_e7567);
        let assign8190_e7571: f64 = (p.p671 * var_iae);
        let assign8190_e7572: f64 = (assign8190_e7568 + assign8190_e7571);
        (assign8190_e7572,)
    } else {
        (var_facneffac_p,)
    }
};
        var_facneffac_p = assign8190_e7574;
        var_facneffac_p_rv = 0.0;

        let assign8200_e7613: f64 = if (((((((param_given[672] || param_given[673]) || param_given[674]) || param_given[675]) || param_given[580]) || param_given[581]) || param_given[582]) || param_given[583]) { 1.0 } else { 0.0 };
        var_guard104 = assign8200_e7613;
        var_guard104_rv = 0.0;

        let (assign8210_e7619,) = {
    if ((var_guard36 != 0.0) && (var_guard104 != 0.0)) {
        (p.p580,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8210_e7619;
        var_poparam_i_rv = 0.0;

        let assign8220_e7621: f64 = if param_given[672] { 1.0 } else { 0.0 };
        let assign8220_e7623: f64 = if assign8220_e7621 == 1.0 { 1.0 } else { 0.0 };
        var_guard105 = assign8220_e7623;
        var_guard105_rv = 0.0;

        let (assign8230_e7631,) = {
    if (((var_guard36 != 0.0) && (var_guard104 != 0.0)) && (var_guard105 != 0.0)) {
        (p.p672,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8230_e7631;
        var_poparam_i_rv = 0.0;

        let (assign8240_e7637,) = {
    if ((var_guard36 != 0.0) && (var_guard104 != 0.0)) {
        (p.p581,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8240_e7637;
        var_plparam_i_rv = 0.0;

        let assign8250_e7639: f64 = if param_given[673] { 1.0 } else { 0.0 };
        let assign8250_e7641: f64 = if assign8250_e7639 == 1.0 { 1.0 } else { 0.0 };
        var_guard106 = assign8250_e7641;
        var_guard106_rv = 0.0;

        let (assign8260_e7649,) = {
    if (((var_guard36 != 0.0) && (var_guard104 != 0.0)) && (var_guard106 != 0.0)) {
        (p.p673,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8260_e7649;
        var_plparam_i_rv = 0.0;

        let (assign8270_e7655,) = {
    if ((var_guard36 != 0.0) && (var_guard104 != 0.0)) {
        (p.p582,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8270_e7655;
        var_pwparam_i_rv = 0.0;

        let assign8280_e7657: f64 = if param_given[674] { 1.0 } else { 0.0 };
        let assign8280_e7659: f64 = if assign8280_e7657 == 1.0 { 1.0 } else { 0.0 };
        var_guard107 = assign8280_e7659;
        var_guard107_rv = 0.0;

        let (assign8290_e7667,) = {
    if (((var_guard36 != 0.0) && (var_guard104 != 0.0)) && (var_guard107 != 0.0)) {
        (p.p674,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8290_e7667;
        var_pwparam_i_rv = 0.0;

        let (assign8300_e7673,) = {
    if ((var_guard36 != 0.0) && (var_guard104 != 0.0)) {
        (p.p583,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8300_e7673;
        var_plwparam_i_rv = 0.0;

        let assign8310_e7675: f64 = if param_given[675] { 1.0 } else { 0.0 };
        let assign8310_e7677: f64 = if assign8310_e7675 == 1.0 { 1.0 } else { 0.0 };
        var_guard108 = assign8310_e7677;
        var_guard108_rv = 0.0;

        let (assign8320_e7685,) = {
    if (((var_guard36 != 0.0) && (var_guard104 != 0.0)) && (var_guard108 != 0.0)) {
        (p.p675,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8320_e7685;
        var_plwparam_i_rv = 0.0;

        let (assign8330_e7705,) = {
    if ((var_guard36 != 0.0) && (var_guard104 != 0.0)) {
        let assign8330_e7693: f64 = (var_plparam_i * var_ile);
        let assign8330_e7694: f64 = (var_poparam_i + assign8330_e7693);
        let assign8330_e7697: f64 = (var_pwparam_i * var_iwe);
        let assign8330_e7698: f64 = (assign8330_e7694 + assign8330_e7697);
        let assign8330_e7701: f64 = (var_plwparam_i * var_iae);
        let assign8330_e7702: f64 = (assign8330_e7698 + assign8330_e7701);
        let assign8330_e7703: f64 = (var_ile * assign8330_e7702);
        (assign8330_e7703,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign8330_e7705;
        var_thesatac_p_rv = 0.0;

        let assign8340_e7744: f64 = if (((((((param_given[676] || param_given[677]) || param_given[678]) || param_given[679]) || param_given[596]) || param_given[597]) || param_given[598]) || param_given[599]) { 1.0 } else { 0.0 };
        var_guard109 = assign8340_e7744;
        var_guard109_rv = 0.0;

        let (assign8350_e7750,) = {
    if ((var_guard36 != 0.0) && (var_guard109 != 0.0)) {
        (p.p596,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8350_e7750;
        var_poparam_i_rv = 0.0;

        let assign8360_e7752: f64 = if param_given[676] { 1.0 } else { 0.0 };
        let assign8360_e7754: f64 = if assign8360_e7752 == 1.0 { 1.0 } else { 0.0 };
        var_guard110 = assign8360_e7754;
        var_guard110_rv = 0.0;

        let (assign8370_e7762,) = {
    if (((var_guard36 != 0.0) && (var_guard109 != 0.0)) && (var_guard110 != 0.0)) {
        (p.p676,)
    } else {
        (var_poparam_i,)
    }
};
        var_poparam_i = assign8370_e7762;
        var_poparam_i_rv = 0.0;

        let (assign8380_e7768,) = {
    if ((var_guard36 != 0.0) && (var_guard109 != 0.0)) {
        (p.p597,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8380_e7768;
        var_plparam_i_rv = 0.0;

        let assign8390_e7770: f64 = if param_given[677] { 1.0 } else { 0.0 };
        let assign8390_e7772: f64 = if assign8390_e7770 == 1.0 { 1.0 } else { 0.0 };
        var_guard111 = assign8390_e7772;
        var_guard111_rv = 0.0;

        let (assign8400_e7780,) = {
    if (((var_guard36 != 0.0) && (var_guard109 != 0.0)) && (var_guard111 != 0.0)) {
        (p.p677,)
    } else {
        (var_plparam_i,)
    }
};
        var_plparam_i = assign8400_e7780;
        var_plparam_i_rv = 0.0;

        let (assign8410_e7786,) = {
    if ((var_guard36 != 0.0) && (var_guard109 != 0.0)) {
        (p.p598,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8410_e7786;
        var_pwparam_i_rv = 0.0;

        let assign8420_e7788: f64 = if param_given[678] { 1.0 } else { 0.0 };
        let assign8420_e7790: f64 = if assign8420_e7788 == 1.0 { 1.0 } else { 0.0 };
        var_guard112 = assign8420_e7790;
        var_guard112_rv = 0.0;

        let (assign8430_e7798,) = {
    if (((var_guard36 != 0.0) && (var_guard109 != 0.0)) && (var_guard112 != 0.0)) {
        (p.p678,)
    } else {
        (var_pwparam_i,)
    }
};
        var_pwparam_i = assign8430_e7798;
        var_pwparam_i_rv = 0.0;

        let (assign8440_e7804,) = {
    if ((var_guard36 != 0.0) && (var_guard109 != 0.0)) {
        (p.p599,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8440_e7804;
        var_plwparam_i_rv = 0.0;

        let assign8450_e7806: f64 = if param_given[679] { 1.0 } else { 0.0 };
        let assign8450_e7808: f64 = if assign8450_e7806 == 1.0 { 1.0 } else { 0.0 };
        var_guard113 = assign8450_e7808;
        var_guard113_rv = 0.0;

        let (assign8460_e7816,) = {
    if (((var_guard36 != 0.0) && (var_guard109 != 0.0)) && (var_guard113 != 0.0)) {
        (p.p679,)
    } else {
        (var_plwparam_i,)
    }
};
        var_plwparam_i = assign8460_e7816;
        var_plwparam_i_rv = 0.0;

        let (assign8470_e7836,) = {
    if ((var_guard36 != 0.0) && (var_guard109 != 0.0)) {
        let assign8470_e7824: f64 = (var_plparam_i * var_ile);
        let assign8470_e7825: f64 = (var_poparam_i + assign8470_e7824);
        let assign8470_e7828: f64 = (var_pwparam_i * var_iwe);
        let assign8470_e7829: f64 = (assign8470_e7825 + assign8470_e7828);
        let assign8470_e7832: f64 = (var_plwparam_i * var_iae);
        let assign8470_e7833: f64 = (assign8470_e7829 + assign8470_e7832);
        let assign8470_e7834: f64 = assign8470_e7833;
        (assign8470_e7834,)
    } else {
        (var_axac_p,)
    }
};
        var_axac_p = assign8470_e7836;
        var_axac_p_rv = 0.0;

        let assign8480_e7855: f64 = if (((param_given[680] || param_given[681]) || param_given[682]) || param_given[683]) { 1.0 } else { 0.0 };
        var_guard114 = assign8480_e7855;
        var_guard114_rv = 0.0;

        let (assign8490_e7875,) = {
    if ((var_guard36 != 0.0) && (var_guard114 != 0.0)) {
        let assign8490_e7863: f64 = (p.p681 * var_ile);
        let assign8490_e7864: f64 = (p.p680 + assign8490_e7863);
        let assign8490_e7867: f64 = (p.p682 * var_iwe);
        let assign8490_e7868: f64 = (assign8490_e7864 + assign8490_e7867);
        let assign8490_e7871: f64 = (p.p683 * var_iae);
        let assign8490_e7872: f64 = (assign8490_e7868 + assign8490_e7871);
        let assign8490_e7873: f64 = (var_ile * assign8490_e7872);
        (assign8490_e7873,)
    } else {
        (var_alpac_p,)
    }
};
        var_alpac_p = assign8490_e7875;
        var_alpac_p_rv = 0.0;

        let assign8500_e7894: f64 = if (((param_given[684] || param_given[685]) || param_given[686]) || param_given[687]) { 1.0 } else { 0.0 };
        var_guard115 = assign8500_e7894;
        var_guard115_rv = 0.0;

        let (assign8510_e7914,) = {
    if ((var_guard36 != 0.0) && (var_guard115 != 0.0)) {
        let assign8510_e7902: f64 = (p.p685 * var_ile);
        let assign8510_e7903: f64 = (p.p684 + assign8510_e7902);
        let assign8510_e7906: f64 = (p.p686 * var_iwe);
        let assign8510_e7907: f64 = (assign8510_e7903 + assign8510_e7906);
        let assign8510_e7910: f64 = (p.p687 * var_iae);
        let assign8510_e7911: f64 = (assign8510_e7907 + assign8510_e7910);
        let assign8510_e7912: f64 = (var_ile * assign8510_e7911);
        (assign8510_e7912,)
    } else {
        (var_alp1ac_p,)
    }
};
        var_alp1ac_p = assign8510_e7914;
        var_alp1ac_p_rv = 0.0;

        let assign8520_e7933: f64 = if (((param_given[688] || param_given[689]) || param_given[690]) || param_given[691]) { 1.0 } else { 0.0 };
        var_guard116 = assign8520_e7933;
        var_guard116_rv = 0.0;

        *var_agidl_p_slot = var_agidl_p;
        *var_agidl_p_rv_slot = var_agidl_p_rv;
        *var_agidld_p_slot = var_agidld_p;
        *var_agidld_p_rv_slot = var_agidld_p_rv;
        *var_alp1ac_p_slot = var_alp1ac_p;
        *var_alp1ac_p_rv_slot = var_alp1ac_p_rv;
        *var_alpac_p_slot = var_alpac_p;
        *var_alpac_p_rv_slot = var_alpac_p_rv;
        *var_axac_p_slot = var_axac_p;
        *var_axac_p_rv_slot = var_axac_p_rv;
        *var_cox_p_slot = var_cox_p;
        *var_cox_p_rv_slot = var_cox_p_rv;
        *var_delvtac_p_slot = var_delvtac_p;
        *var_delvtac_p_rv_slot = var_delvtac_p_rv;
        *var_facneffac_p_slot = var_facneffac_p;
        *var_facneffac_p_rv_slot = var_facneffac_p_rv;
        *var_guard100_slot = var_guard100;
        *var_guard100_rv_slot = var_guard100_rv;
        *var_guard101_slot = var_guard101;
        *var_guard101_rv_slot = var_guard101_rv;
        *var_guard102_slot = var_guard102;
        *var_guard102_rv_slot = var_guard102_rv;
        *var_guard103_slot = var_guard103;
        *var_guard103_rv_slot = var_guard103_rv;
        *var_guard104_slot = var_guard104;
        *var_guard104_rv_slot = var_guard104_rv;
        *var_guard105_slot = var_guard105;
        *var_guard105_rv_slot = var_guard105_rv;
        *var_guard106_slot = var_guard106;
        *var_guard106_rv_slot = var_guard106_rv;
        *var_guard107_slot = var_guard107;
        *var_guard107_rv_slot = var_guard107_rv;
        *var_guard108_slot = var_guard108;
        *var_guard108_rv_slot = var_guard108_rv;
        *var_guard109_slot = var_guard109;
        *var_guard109_rv_slot = var_guard109_rv;
        *var_guard110_slot = var_guard110;
        *var_guard110_rv_slot = var_guard110_rv;
        *var_guard111_slot = var_guard111;
        *var_guard111_rv_slot = var_guard111_rv;
        *var_guard112_slot = var_guard112;
        *var_guard112_rv_slot = var_guard112_rv;
        *var_guard113_slot = var_guard113;
        *var_guard113_rv_slot = var_guard113_rv;
        *var_guard114_slot = var_guard114;
        *var_guard114_rv_slot = var_guard114_rv;
        *var_guard115_slot = var_guard115;
        *var_guard115_rv_slot = var_guard115_rv;
        *var_guard116_slot = var_guard116;
        *var_guard116_rv_slot = var_guard116_rv;
        *var_guard94_slot = var_guard94;
        *var_guard94_rv_slot = var_guard94_rv;
        *var_guard95_slot = var_guard95;
        *var_guard95_rv_slot = var_guard95_rv;
        *var_guard96_slot = var_guard96;
        *var_guard96_rv_slot = var_guard96_rv;
        *var_guard97_slot = var_guard97;
        *var_guard97_rv_slot = var_guard97_rv;
        *var_guard98_slot = var_guard98;
        *var_guard98_rv_slot = var_guard98_rv;
        *var_guard99_slot = var_guard99;
        *var_guard99_rv_slot = var_guard99_rv;
        *var_iginv_p_slot = var_iginv_p;
        *var_iginv_p_rv_slot = var_iginv_p_rv;
        *var_igov_p_slot = var_igov_p;
        *var_igov_p_rv_slot = var_igov_p_rv;
        *var_igovd_p_slot = var_igovd_p;
        *var_igovd_p_rv_slot = var_igovd_p_rv;
        *var_plparam_i_slot = var_plparam_i;
        *var_plparam_i_rv_slot = var_plparam_i_rv;
        *var_plwparam_i_slot = var_plwparam_i;
        *var_plwparam_i_rv_slot = var_plwparam_i_rv;
        *var_poparam_i_slot = var_poparam_i;
        *var_poparam_i_rv_slot = var_poparam_i_rv;
        *var_pwparam_i_slot = var_pwparam_i;
        *var_pwparam_i_rv_slot = var_pwparam_i_rv;
        *var_stbgidl_p_slot = var_stbgidl_p;
        *var_stbgidl_p_rv_slot = var_stbgidl_p_rv;
        *var_stbgidld_p_slot = var_stbgidld_p;
        *var_stbgidld_p_rv_slot = var_stbgidld_p_rv;
        *var_stig_p_slot = var_stig_p;
        *var_stig_p_rv_slot = var_stig_p_rv;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_rv_slot = var_thesatac_p_rv;
    }

    pub(super) fn stamp_reactive_block_13(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        var_guard116: f64,
        var_guard36: f64,
        var_iae: f64,
        var_iiae: f64,
        var_iilcv: f64,
        var_iiwcv: f64,
        var_iiwecv: f64,
        var_ile: f64,
        var_ile2: f64,
        var_iwe: f64,
        var_le: f64,
        var_nf_i: f64,
        var_sa_i: f64,
        var_sb_i: f64,
        var_sd_i: f64,
        var_we_edge: f64,
        var_betnedge_p_slot: &mut f64,
        var_betnedge_p_rv_slot: &mut f64,
        var_cfbedge_p_slot: &mut f64,
        var_cfbedge_p_rv_slot: &mut f64,
        var_cfdedge_p_slot: &mut f64,
        var_cfdedge_p_rv_slot: &mut f64,
        var_cfedge_p_slot: &mut f64,
        var_cfedge_p_rv_slot: &mut f64,
        var_cfr_p_slot: &mut f64,
        var_cfr_p_rv_slot: &mut f64,
        var_cfrd_p_slot: &mut f64,
        var_cfrd_p_rv_slot: &mut f64,
        var_cgbov_p_slot: &mut f64,
        var_cgbov_p_rv_slot: &mut f64,
        var_cgov_p_slot: &mut f64,
        var_cgov_p_rv_slot: &mut f64,
        var_cgovd_p_slot: &mut f64,
        var_cgovd_p_rv_slot: &mut f64,
        var_cinr_p_slot: &mut f64,
        var_cinr_p_rv_slot: &mut f64,
        var_cinrd_p_slot: &mut f64,
        var_cinrd_p_rv_slot: &mut f64,
        var_ctedge_p_slot: &mut f64,
        var_ctedge_p_rv_slot: &mut f64,
        var_cth_p_slot: &mut f64,
        var_cth_p_rv_slot: &mut f64,
        var_dphibedge_p_slot: &mut f64,
        var_dphibedge_p_rv_slot: &mut f64,
        var_guard117_slot: &mut f64,
        var_guard117_rv_slot: &mut f64,
        var_guard118_slot: &mut f64,
        var_guard118_rv_slot: &mut f64,
        var_guard119_slot: &mut f64,
        var_guard119_rv_slot: &mut f64,
        var_guard120_slot: &mut f64,
        var_guard120_rv_slot: &mut f64,
        var_guard121_slot: &mut f64,
        var_guard121_rv_slot: &mut f64,
        var_guard122_slot: &mut f64,
        var_guard122_rv_slot: &mut f64,
        var_guard127_slot: &mut f64,
        var_guard127_rv_slot: &mut f64,
        var_guard128_slot: &mut f64,
        var_guard128_rv_slot: &mut f64,
        var_guard129_slot: &mut f64,
        var_guard129_rv_slot: &mut f64,
        var_guard130_slot: &mut f64,
        var_guard130_rv_slot: &mut f64,
        var_guard131_slot: &mut f64,
        var_guard131_rv_slot: &mut f64,
        var_guard132_slot: &mut f64,
        var_guard132_rv_slot: &mut f64,
        var_guard133_slot: &mut f64,
        var_guard133_rv_slot: &mut f64,
        var_guard134_slot: &mut f64,
        var_guard134_rv_slot: &mut f64,
        var_guard135_slot: &mut f64,
        var_guard135_rv_slot: &mut f64,
        var_guard136_slot: &mut f64,
        var_guard136_rv_slot: &mut f64,
        var_guard137_slot: &mut f64,
        var_guard137_rv_slot: &mut f64,
        var_guard138_slot: &mut f64,
        var_guard138_rv_slot: &mut f64,
        var_guard139_slot: &mut f64,
        var_guard139_rv_slot: &mut f64,
        var_guard144_slot: &mut f64,
        var_guard144_rv_slot: &mut f64,
        var_guard146_slot: &mut f64,
        var_guard146_rv_slot: &mut f64,
        var_guard147_slot: &mut f64,
        var_guard147_rv_slot: &mut f64,
        var_kvsatac_i_slot: &mut f64,
        var_kvsatac_i_rv_slot: &mut f64,
        var_loop__slot: &mut f64,
        var_loop__rv_slot: &mut f64,
        var_neffedge_p_slot: &mut f64,
        var_neffedge_p_rv_slot: &mut f64,
        var_pscebedge_p_slot: &mut f64,
        var_pscebedge_p_rv_slot: &mut f64,
        var_pscededge_p_slot: &mut f64,
        var_pscededge_p_rv_slot: &mut f64,
        var_psceedge_p_slot: &mut f64,
        var_psceedge_p_rv_slot: &mut f64,
        var_stbetedge_p_slot: &mut f64,
        var_stbetedge_p_rv_slot: &mut f64,
        var_stvfbedge_p_slot: &mut f64,
        var_stvfbedge_p_rv_slot: &mut f64,
        var_tmpa_slot: &mut f64,
        var_tmpa_rv_slot: &mut f64,
        var_tmpb_slot: &mut f64,
        var_tmpb_rv_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_vfbedge_p_rv_slot: &mut f64,
    ) {
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_betnedge_p_rv: f64 = *var_betnedge_p_rv_slot;
        let mut var_cfbedge_p: f64 = *var_cfbedge_p_slot;
        let mut var_cfbedge_p_rv: f64 = *var_cfbedge_p_rv_slot;
        let mut var_cfdedge_p: f64 = *var_cfdedge_p_slot;
        let mut var_cfdedge_p_rv: f64 = *var_cfdedge_p_rv_slot;
        let mut var_cfedge_p: f64 = *var_cfedge_p_slot;
        let mut var_cfedge_p_rv: f64 = *var_cfedge_p_rv_slot;
        let mut var_cfr_p: f64 = *var_cfr_p_slot;
        let mut var_cfr_p_rv: f64 = *var_cfr_p_rv_slot;
        let mut var_cfrd_p: f64 = *var_cfrd_p_slot;
        let mut var_cfrd_p_rv: f64 = *var_cfrd_p_rv_slot;
        let mut var_cgbov_p: f64 = *var_cgbov_p_slot;
        let mut var_cgbov_p_rv: f64 = *var_cgbov_p_rv_slot;
        let mut var_cgov_p: f64 = *var_cgov_p_slot;
        let mut var_cgov_p_rv: f64 = *var_cgov_p_rv_slot;
        let mut var_cgovd_p: f64 = *var_cgovd_p_slot;
        let mut var_cgovd_p_rv: f64 = *var_cgovd_p_rv_slot;
        let mut var_cinr_p: f64 = *var_cinr_p_slot;
        let mut var_cinr_p_rv: f64 = *var_cinr_p_rv_slot;
        let mut var_cinrd_p: f64 = *var_cinrd_p_slot;
        let mut var_cinrd_p_rv: f64 = *var_cinrd_p_rv_slot;
        let mut var_ctedge_p: f64 = *var_ctedge_p_slot;
        let mut var_ctedge_p_rv: f64 = *var_ctedge_p_rv_slot;
        let mut var_cth_p: f64 = *var_cth_p_slot;
        let mut var_cth_p_rv: f64 = *var_cth_p_rv_slot;
        let mut var_dphibedge_p: f64 = *var_dphibedge_p_slot;
        let mut var_dphibedge_p_rv: f64 = *var_dphibedge_p_rv_slot;
        let mut var_guard117: f64 = *var_guard117_slot;
        let mut var_guard117_rv: f64 = *var_guard117_rv_slot;
        let mut var_guard118: f64 = *var_guard118_slot;
        let mut var_guard118_rv: f64 = *var_guard118_rv_slot;
        let mut var_guard119: f64 = *var_guard119_slot;
        let mut var_guard119_rv: f64 = *var_guard119_rv_slot;
        let mut var_guard120: f64 = *var_guard120_slot;
        let mut var_guard120_rv: f64 = *var_guard120_rv_slot;
        let mut var_guard121: f64 = *var_guard121_slot;
        let mut var_guard121_rv: f64 = *var_guard121_rv_slot;
        let mut var_guard122: f64 = *var_guard122_slot;
        let mut var_guard122_rv: f64 = *var_guard122_rv_slot;
        let mut var_guard127: f64 = *var_guard127_slot;
        let mut var_guard127_rv: f64 = *var_guard127_rv_slot;
        let mut var_guard128: f64 = *var_guard128_slot;
        let mut var_guard128_rv: f64 = *var_guard128_rv_slot;
        let mut var_guard129: f64 = *var_guard129_slot;
        let mut var_guard129_rv: f64 = *var_guard129_rv_slot;
        let mut var_guard130: f64 = *var_guard130_slot;
        let mut var_guard130_rv: f64 = *var_guard130_rv_slot;
        let mut var_guard131: f64 = *var_guard131_slot;
        let mut var_guard131_rv: f64 = *var_guard131_rv_slot;
        let mut var_guard132: f64 = *var_guard132_slot;
        let mut var_guard132_rv: f64 = *var_guard132_rv_slot;
        let mut var_guard133: f64 = *var_guard133_slot;
        let mut var_guard133_rv: f64 = *var_guard133_rv_slot;
        let mut var_guard134: f64 = *var_guard134_slot;
        let mut var_guard134_rv: f64 = *var_guard134_rv_slot;
        let mut var_guard135: f64 = *var_guard135_slot;
        let mut var_guard135_rv: f64 = *var_guard135_rv_slot;
        let mut var_guard136: f64 = *var_guard136_slot;
        let mut var_guard136_rv: f64 = *var_guard136_rv_slot;
        let mut var_guard137: f64 = *var_guard137_slot;
        let mut var_guard137_rv: f64 = *var_guard137_rv_slot;
        let mut var_guard138: f64 = *var_guard138_slot;
        let mut var_guard138_rv: f64 = *var_guard138_rv_slot;
        let mut var_guard139: f64 = *var_guard139_slot;
        let mut var_guard139_rv: f64 = *var_guard139_rv_slot;
        let mut var_guard144: f64 = *var_guard144_slot;
        let mut var_guard144_rv: f64 = *var_guard144_rv_slot;
        let mut var_guard146: f64 = *var_guard146_slot;
        let mut var_guard146_rv: f64 = *var_guard146_rv_slot;
        let mut var_guard147: f64 = *var_guard147_slot;
        let mut var_guard147_rv: f64 = *var_guard147_rv_slot;
        let mut var_kvsatac_i: f64 = *var_kvsatac_i_slot;
        let mut var_kvsatac_i_rv: f64 = *var_kvsatac_i_rv_slot;
        let mut var_loop_: f64 = *var_loop__slot;
        let mut var_loop__rv: f64 = *var_loop__rv_slot;
        let mut var_neffedge_p: f64 = *var_neffedge_p_slot;
        let mut var_neffedge_p_rv: f64 = *var_neffedge_p_rv_slot;
        let mut var_pscebedge_p: f64 = *var_pscebedge_p_slot;
        let mut var_pscebedge_p_rv: f64 = *var_pscebedge_p_rv_slot;
        let mut var_pscededge_p: f64 = *var_pscededge_p_slot;
        let mut var_pscededge_p_rv: f64 = *var_pscededge_p_rv_slot;
        let mut var_psceedge_p: f64 = *var_psceedge_p_slot;
        let mut var_psceedge_p_rv: f64 = *var_psceedge_p_rv_slot;
        let mut var_stbetedge_p: f64 = *var_stbetedge_p_slot;
        let mut var_stbetedge_p_rv: f64 = *var_stbetedge_p_rv_slot;
        let mut var_stvfbedge_p: f64 = *var_stvfbedge_p_slot;
        let mut var_stvfbedge_p_rv: f64 = *var_stvfbedge_p_rv_slot;
        let mut var_tmpa: f64 = *var_tmpa_slot;
        let mut var_tmpa_rv: f64 = *var_tmpa_rv_slot;
        let mut var_tmpb: f64 = *var_tmpb_slot;
        let mut var_tmpb_rv: f64 = *var_tmpb_rv_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_vfbedge_p_rv: f64 = *var_vfbedge_p_rv_slot;

        let (assign8530_e7953,) = {
    if ((var_guard36 != 0.0) && (var_guard116 != 0.0)) {
        let assign8530_e7941: f64 = (p.p689 * var_ile);
        let assign8530_e7942: f64 = (p.p688 + assign8530_e7941);
        let assign8530_e7945: f64 = (p.p690 * var_iwe);
        let assign8530_e7946: f64 = (assign8530_e7942 + assign8530_e7945);
        let assign8530_e7949: f64 = (p.p691 * var_iae);
        let assign8530_e7950: f64 = (assign8530_e7946 + assign8530_e7949);
        let assign8530_e7951: f64 = (var_iiwecv * assign8530_e7950);
        (assign8530_e7951,)
    } else {
        (var_cgov_p,)
    }
};
        var_cgov_p = assign8530_e7953;
        var_cgov_p_rv = 0.0;

        let assign8540_e7972: f64 = if (((param_given[692] || param_given[693]) || param_given[694]) || param_given[695]) { 1.0 } else { 0.0 };
        var_guard117 = assign8540_e7972;
        var_guard117_rv = 0.0;

        let (assign8550_e7992,) = {
    if ((var_guard36 != 0.0) && (var_guard117 != 0.0)) {
        let assign8550_e7980: f64 = (p.p693 * var_ile);
        let assign8550_e7981: f64 = (p.p692 + assign8550_e7980);
        let assign8550_e7984: f64 = (p.p694 * var_iwe);
        let assign8550_e7985: f64 = (assign8550_e7981 + assign8550_e7984);
        let assign8550_e7988: f64 = (p.p695 * var_iae);
        let assign8550_e7989: f64 = (assign8550_e7985 + assign8550_e7988);
        let assign8550_e7990: f64 = (var_iiwecv * assign8550_e7989);
        (assign8550_e7990,)
    } else {
        (var_cgovd_p,)
    }
};
        var_cgovd_p = assign8550_e7992;
        var_cgovd_p_rv = 0.0;

        let assign8560_e8011: f64 = if (((param_given[696] || param_given[697]) || param_given[698]) || param_given[699]) { 1.0 } else { 0.0 };
        var_guard118 = assign8560_e8011;
        var_guard118_rv = 0.0;

        let (assign8570_e8031,) = {
    if ((var_guard36 != 0.0) && (var_guard118 != 0.0)) {
        let assign8570_e8019: f64 = (p.p697 * var_ile);
        let assign8570_e8020: f64 = (p.p696 + assign8570_e8019);
        let assign8570_e8023: f64 = (p.p698 * var_iwe);
        let assign8570_e8024: f64 = (assign8570_e8020 + assign8570_e8023);
        let assign8570_e8027: f64 = (p.p699 * var_iae);
        let assign8570_e8028: f64 = (assign8570_e8024 + assign8570_e8027);
        let assign8570_e8029: f64 = (var_iilcv * assign8570_e8028);
        (assign8570_e8029,)
    } else {
        (var_cgbov_p,)
    }
};
        var_cgbov_p = assign8570_e8031;
        var_cgbov_p_rv = 0.0;

        let assign8580_e8050: f64 = if (((param_given[700] || param_given[701]) || param_given[702]) || param_given[703]) { 1.0 } else { 0.0 };
        var_guard119 = assign8580_e8050;
        var_guard119_rv = 0.0;

        let (assign8590_e8070,) = {
    if ((var_guard36 != 0.0) && (var_guard119 != 0.0)) {
        let assign8590_e8058: f64 = (p.p701 * var_ile);
        let assign8590_e8059: f64 = (p.p700 + assign8590_e8058);
        let assign8590_e8062: f64 = (p.p702 * var_iwe);
        let assign8590_e8063: f64 = (assign8590_e8059 + assign8590_e8062);
        let assign8590_e8066: f64 = (p.p703 * var_iae);
        let assign8590_e8067: f64 = (assign8590_e8063 + assign8590_e8066);
        let assign8590_e8068: f64 = (var_iiwecv * assign8590_e8067);
        (assign8590_e8068,)
    } else {
        (var_cinr_p,)
    }
};
        var_cinr_p = assign8590_e8070;
        var_cinr_p_rv = 0.0;

        let assign8600_e8089: f64 = if (((param_given[704] || param_given[705]) || param_given[706]) || param_given[707]) { 1.0 } else { 0.0 };
        var_guard120 = assign8600_e8089;
        var_guard120_rv = 0.0;

        let (assign8610_e8109,) = {
    if ((var_guard36 != 0.0) && (var_guard120 != 0.0)) {
        let assign8610_e8097: f64 = (p.p705 * var_ile);
        let assign8610_e8098: f64 = (p.p704 + assign8610_e8097);
        let assign8610_e8101: f64 = (p.p706 * var_iwe);
        let assign8610_e8102: f64 = (assign8610_e8098 + assign8610_e8101);
        let assign8610_e8105: f64 = (p.p707 * var_iae);
        let assign8610_e8106: f64 = (assign8610_e8102 + assign8610_e8105);
        let assign8610_e8107: f64 = (var_iiwecv * assign8610_e8106);
        (assign8610_e8107,)
    } else {
        (var_cinrd_p,)
    }
};
        var_cinrd_p = assign8610_e8109;
        var_cinrd_p_rv = 0.0;

        let assign8620_e8128: f64 = if (((param_given[708] || param_given[709]) || param_given[710]) || param_given[711]) { 1.0 } else { 0.0 };
        var_guard121 = assign8620_e8128;
        var_guard121_rv = 0.0;

        let (assign8630_e8148,) = {
    if ((var_guard36 != 0.0) && (var_guard121 != 0.0)) {
        let assign8630_e8136: f64 = (p.p709 * var_ile);
        let assign8630_e8137: f64 = (p.p708 + assign8630_e8136);
        let assign8630_e8140: f64 = (p.p710 * var_iwe);
        let assign8630_e8141: f64 = (assign8630_e8137 + assign8630_e8140);
        let assign8630_e8144: f64 = (p.p711 * var_iae);
        let assign8630_e8145: f64 = (assign8630_e8141 + assign8630_e8144);
        let assign8630_e8146: f64 = (var_iiwcv * assign8630_e8145);
        (assign8630_e8146,)
    } else {
        (var_cfr_p,)
    }
};
        var_cfr_p = assign8630_e8148;
        var_cfr_p_rv = 0.0;

        let assign8640_e8167: f64 = if (((param_given[712] || param_given[713]) || param_given[714]) || param_given[715]) { 1.0 } else { 0.0 };
        var_guard122 = assign8640_e8167;
        var_guard122_rv = 0.0;

        let (assign8650_e8187,) = {
    if ((var_guard36 != 0.0) && (var_guard122 != 0.0)) {
        let assign8650_e8175: f64 = (p.p713 * var_ile);
        let assign8650_e8176: f64 = (p.p712 + assign8650_e8175);
        let assign8650_e8179: f64 = (p.p714 * var_iwe);
        let assign8650_e8180: f64 = (assign8650_e8176 + assign8650_e8179);
        let assign8650_e8183: f64 = (p.p715 * var_iae);
        let assign8650_e8184: f64 = (assign8650_e8180 + assign8650_e8183);
        let assign8650_e8185: f64 = (var_iiwcv * assign8650_e8184);
        (assign8650_e8185,)
    } else {
        (var_cfrd_p,)
    }
};
        var_cfrd_p = assign8650_e8187;
        var_cfrd_p_rv = 0.0;

        let assign8740_e8362: f64 = if (((param_given[732] || param_given[733]) || param_given[734]) || param_given[735]) { 1.0 } else { 0.0 };
        var_guard127 = assign8740_e8362;
        var_guard127_rv = 0.0;

        let (assign8750_e8380,) = {
    if ((var_guard36 != 0.0) && (var_guard127 != 0.0)) {
        let assign8750_e8369: f64 = (p.p733 * var_ile);
        let assign8750_e8370: f64 = (p.p732 + assign8750_e8369);
        let assign8750_e8373: f64 = (p.p734 * var_iwe);
        let assign8750_e8374: f64 = (assign8750_e8370 + assign8750_e8373);
        let assign8750_e8377: f64 = (p.p735 * var_iae);
        let assign8750_e8378: f64 = (assign8750_e8374 + assign8750_e8377);
        (assign8750_e8378,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign8750_e8380;
        var_vfbedge_p_rv = 0.0;

        let assign8760_e8399: f64 = if (((param_given[736] || param_given[737]) || param_given[738]) || param_given[739]) { 1.0 } else { 0.0 };
        var_guard128 = assign8760_e8399;
        var_guard128_rv = 0.0;

        let (assign8770_e8417,) = {
    if ((var_guard36 != 0.0) && (var_guard128 != 0.0)) {
        let assign8770_e8406: f64 = (p.p737 * var_ile);
        let assign8770_e8407: f64 = (p.p736 + assign8770_e8406);
        let assign8770_e8410: f64 = (p.p738 * var_iwe);
        let assign8770_e8411: f64 = (assign8770_e8407 + assign8770_e8410);
        let assign8770_e8414: f64 = (p.p739 * var_iae);
        let assign8770_e8415: f64 = (assign8770_e8411 + assign8770_e8414);
        (assign8770_e8415,)
    } else {
        (var_stvfbedge_p,)
    }
};
        var_stvfbedge_p = assign8770_e8417;
        var_stvfbedge_p_rv = 0.0;

        let assign8780_e8436: f64 = if (((param_given[740] || param_given[741]) || param_given[742]) || param_given[743]) { 1.0 } else { 0.0 };
        var_guard129 = assign8780_e8436;
        var_guard129_rv = 0.0;

        let (assign8790_e8454,) = {
    if ((var_guard36 != 0.0) && (var_guard129 != 0.0)) {
        let assign8790_e8443: f64 = (p.p741 * var_ile);
        let assign8790_e8444: f64 = (p.p740 + assign8790_e8443);
        let assign8790_e8447: f64 = (p.p742 * var_iwe);
        let assign8790_e8448: f64 = (assign8790_e8444 + assign8790_e8447);
        let assign8790_e8451: f64 = (p.p743 * var_iae);
        let assign8790_e8452: f64 = (assign8790_e8448 + assign8790_e8451);
        (assign8790_e8452,)
    } else {
        (var_dphibedge_p,)
    }
};
        var_dphibedge_p = assign8790_e8454;
        var_dphibedge_p_rv = 0.0;

        let assign8800_e8473: f64 = if (((param_given[744] || param_given[745]) || param_given[746]) || param_given[747]) { 1.0 } else { 0.0 };
        var_guard130 = assign8800_e8473;
        var_guard130_rv = 0.0;

        let (assign8810_e8491,) = {
    if ((var_guard36 != 0.0) && (var_guard130 != 0.0)) {
        let assign8810_e8480: f64 = (p.p745 * var_ile);
        let assign8810_e8481: f64 = (p.p744 + assign8810_e8480);
        let assign8810_e8484: f64 = (p.p746 * var_iwe);
        let assign8810_e8485: f64 = (assign8810_e8481 + assign8810_e8484);
        let assign8810_e8488: f64 = (p.p747 * var_iae);
        let assign8810_e8489: f64 = (assign8810_e8485 + assign8810_e8488);
        (assign8810_e8489,)
    } else {
        (var_neffedge_p,)
    }
};
        var_neffedge_p = assign8810_e8491;
        var_neffedge_p_rv = 0.0;

        let assign8820_e8510: f64 = if (((param_given[748] || param_given[749]) || param_given[750]) || param_given[751]) { 1.0 } else { 0.0 };
        var_guard131 = assign8820_e8510;
        var_guard131_rv = 0.0;

        let (assign8830_e8528,) = {
    if ((var_guard36 != 0.0) && (var_guard131 != 0.0)) {
        let assign8830_e8517: f64 = (p.p749 * var_ile);
        let assign8830_e8518: f64 = (p.p748 + assign8830_e8517);
        let assign8830_e8521: f64 = (p.p750 * var_iwe);
        let assign8830_e8522: f64 = (assign8830_e8518 + assign8830_e8521);
        let assign8830_e8525: f64 = (p.p751 * var_iae);
        let assign8830_e8526: f64 = (assign8830_e8522 + assign8830_e8525);
        (assign8830_e8526,)
    } else {
        (var_ctedge_p,)
    }
};
        var_ctedge_p = assign8830_e8528;
        var_ctedge_p_rv = 0.0;

        let assign8840_e8547: f64 = if (((param_given[752] || param_given[753]) || param_given[754]) || param_given[755]) { 1.0 } else { 0.0 };
        var_guard132 = assign8840_e8547;
        var_guard132_rv = 0.0;

        let (assign8850_e8569,) = {
    if ((var_guard36 != 0.0) && (var_guard132 != 0.0)) {
        let assign8850_e8553: f64 = (var_we_edge / var_le);
        let assign8850_e8557: f64 = (p.p753 * var_ile);
        let assign8850_e8558: f64 = (p.p752 + assign8850_e8557);
        let assign8850_e8561: f64 = (p.p754 * var_iwe);
        let assign8850_e8562: f64 = (assign8850_e8558 + assign8850_e8561);
        let assign8850_e8565: f64 = (p.p755 * var_iae);
        let assign8850_e8566: f64 = (assign8850_e8562 + assign8850_e8565);
        let assign8850_e8567: f64 = (assign8850_e8553 * assign8850_e8566);
        (assign8850_e8567,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign8850_e8569;
        var_betnedge_p_rv = 0.0;

        let assign8860_e8588: f64 = if (((param_given[756] || param_given[757]) || param_given[758]) || param_given[759]) { 1.0 } else { 0.0 };
        var_guard133 = assign8860_e8588;
        var_guard133_rv = 0.0;

        let (assign8870_e8606,) = {
    if ((var_guard36 != 0.0) && (var_guard133 != 0.0)) {
        let assign8870_e8595: f64 = (p.p757 * var_ile);
        let assign8870_e8596: f64 = (p.p756 + assign8870_e8595);
        let assign8870_e8599: f64 = (p.p758 * var_iwe);
        let assign8870_e8600: f64 = (assign8870_e8596 + assign8870_e8599);
        let assign8870_e8603: f64 = (p.p759 * var_iae);
        let assign8870_e8604: f64 = (assign8870_e8600 + assign8870_e8603);
        (assign8870_e8604,)
    } else {
        (var_stbetedge_p,)
    }
};
        var_stbetedge_p = assign8870_e8606;
        var_stbetedge_p_rv = 0.0;

        let assign8880_e8625: f64 = if (((param_given[760] || param_given[761]) || param_given[762]) || param_given[763]) { 1.0 } else { 0.0 };
        var_guard134 = assign8880_e8625;
        var_guard134_rv = 0.0;

        let (assign8890_e8645,) = {
    if ((var_guard36 != 0.0) && (var_guard134 != 0.0)) {
        let assign8890_e8633: f64 = (p.p761 * var_ile);
        let assign8890_e8634: f64 = (p.p760 + assign8890_e8633);
        let assign8890_e8637: f64 = (p.p762 * var_iwe);
        let assign8890_e8638: f64 = (assign8890_e8634 + assign8890_e8637);
        let assign8890_e8641: f64 = (p.p763 * var_iae);
        let assign8890_e8642: f64 = (assign8890_e8638 + assign8890_e8641);
        let assign8890_e8643: f64 = (var_ile2 * assign8890_e8642);
        (assign8890_e8643,)
    } else {
        (var_psceedge_p,)
    }
};
        var_psceedge_p = assign8890_e8645;
        var_psceedge_p_rv = 0.0;

        let assign8900_e8664: f64 = if (((param_given[764] || param_given[765]) || param_given[766]) || param_given[767]) { 1.0 } else { 0.0 };
        var_guard135 = assign8900_e8664;
        var_guard135_rv = 0.0;

        let (assign8910_e8682,) = {
    if ((var_guard36 != 0.0) && (var_guard135 != 0.0)) {
        let assign8910_e8671: f64 = (p.p765 * var_ile);
        let assign8910_e8672: f64 = (p.p764 + assign8910_e8671);
        let assign8910_e8675: f64 = (p.p766 * var_iwe);
        let assign8910_e8676: f64 = (assign8910_e8672 + assign8910_e8675);
        let assign8910_e8679: f64 = (p.p767 * var_iae);
        let assign8910_e8680: f64 = (assign8910_e8676 + assign8910_e8679);
        (assign8910_e8680,)
    } else {
        (var_pscebedge_p,)
    }
};
        var_pscebedge_p = assign8910_e8682;
        var_pscebedge_p_rv = 0.0;

        let assign8920_e8701: f64 = if (((param_given[768] || param_given[769]) || param_given[770]) || param_given[771]) { 1.0 } else { 0.0 };
        var_guard136 = assign8920_e8701;
        var_guard136_rv = 0.0;

        let (assign8930_e8719,) = {
    if ((var_guard36 != 0.0) && (var_guard136 != 0.0)) {
        let assign8930_e8708: f64 = (p.p769 * var_ile);
        let assign8930_e8709: f64 = (p.p768 + assign8930_e8708);
        let assign8930_e8712: f64 = (p.p770 * var_iwe);
        let assign8930_e8713: f64 = (assign8930_e8709 + assign8930_e8712);
        let assign8930_e8716: f64 = (p.p771 * var_iae);
        let assign8930_e8717: f64 = (assign8930_e8713 + assign8930_e8716);
        (assign8930_e8717,)
    } else {
        (var_pscededge_p,)
    }
};
        var_pscededge_p = assign8930_e8719;
        var_pscededge_p_rv = 0.0;

        let assign8940_e8738: f64 = if (((param_given[772] || param_given[773]) || param_given[774]) || param_given[775]) { 1.0 } else { 0.0 };
        var_guard137 = assign8940_e8738;
        var_guard137_rv = 0.0;

        let (assign8950_e8758,) = {
    if ((var_guard36 != 0.0) && (var_guard137 != 0.0)) {
        let assign8950_e8746: f64 = (p.p773 * var_ile);
        let assign8950_e8747: f64 = (p.p772 + assign8950_e8746);
        let assign8950_e8750: f64 = (p.p774 * var_iwe);
        let assign8950_e8751: f64 = (assign8950_e8747 + assign8950_e8750);
        let assign8950_e8754: f64 = (p.p775 * var_iae);
        let assign8950_e8755: f64 = (assign8950_e8751 + assign8950_e8754);
        let assign8950_e8756: f64 = (var_ile2 * assign8950_e8755);
        (assign8950_e8756,)
    } else {
        (var_cfedge_p,)
    }
};
        var_cfedge_p = assign8950_e8758;
        var_cfedge_p_rv = 0.0;

        let assign8960_e8777: f64 = if (((param_given[780] || param_given[781]) || param_given[782]) || param_given[783]) { 1.0 } else { 0.0 };
        var_guard138 = assign8960_e8777;
        var_guard138_rv = 0.0;

        let (assign8970_e8795,) = {
    if ((var_guard36 != 0.0) && (var_guard138 != 0.0)) {
        let assign8970_e8784: f64 = (p.p781 * var_ile);
        let assign8970_e8785: f64 = (p.p780 + assign8970_e8784);
        let assign8970_e8788: f64 = (p.p782 * var_iwe);
        let assign8970_e8789: f64 = (assign8970_e8785 + assign8970_e8788);
        let assign8970_e8792: f64 = (p.p783 * var_iae);
        let assign8970_e8793: f64 = (assign8970_e8789 + assign8970_e8792);
        (assign8970_e8793,)
    } else {
        (var_cfdedge_p,)
    }
};
        var_cfdedge_p = assign8970_e8795;
        var_cfdedge_p_rv = 0.0;

        let assign8980_e8814: f64 = if (((param_given[776] || param_given[777]) || param_given[778]) || param_given[779]) { 1.0 } else { 0.0 };
        var_guard139 = assign8980_e8814;
        var_guard139_rv = 0.0;

        let (assign8990_e8832,) = {
    if ((var_guard36 != 0.0) && (var_guard139 != 0.0)) {
        let assign8990_e8821: f64 = (p.p777 * var_ile);
        let assign8990_e8822: f64 = (p.p776 + assign8990_e8821);
        let assign8990_e8825: f64 = (p.p778 * var_iwe);
        let assign8990_e8826: f64 = (assign8990_e8822 + assign8990_e8825);
        let assign8990_e8829: f64 = (p.p779 * var_iae);
        let assign8990_e8830: f64 = (assign8990_e8826 + assign8990_e8829);
        (assign8990_e8830,)
    } else {
        (var_cfbedge_p,)
    }
};
        var_cfbedge_p = assign8990_e8832;
        var_cfbedge_p_rv = 0.0;

        let assign9080_e9007: f64 = if (((param_given[800] || param_given[801]) || param_given[802]) || param_given[803]) { 1.0 } else { 0.0 };
        var_guard144 = assign9080_e9007;
        var_guard144_rv = 0.0;

        let (assign9090_e9027,) = {
    if ((var_guard36 != 0.0) && (var_guard144 != 0.0)) {
        let assign9090_e9015: f64 = (p.p801 * var_ile);
        let assign9090_e9016: f64 = (p.p800 + assign9090_e9015);
        let assign9090_e9019: f64 = (p.p802 * var_iwe);
        let assign9090_e9020: f64 = (assign9090_e9016 + assign9090_e9019);
        let assign9090_e9023: f64 = (p.p803 * var_iae);
        let assign9090_e9024: f64 = (assign9090_e9020 + assign9090_e9023);
        let assign9090_e9025: f64 = (var_iiae * assign9090_e9024);
        (assign9090_e9025,)
    } else {
        (var_cth_p,)
    }
};
        var_cth_p = assign9090_e9027;
        var_cth_p_rv = 0.0;

        let (assign9120_e9068,) = {
    if (var_guard36 != 0.0) {
        (0.0,)
    } else {
        (var_tmpa,)
    }
};
        var_tmpa = assign9120_e9068;
        var_tmpa_rv = 0.0;

        let (assign9130_e9072,) = {
    if (var_guard36 != 0.0) {
        (0.0,)
    } else {
        (var_tmpb,)
    }
};
        var_tmpb = assign9130_e9072;
        var_tmpb_rv = 0.0;

        let (assign9140_e9076,) = {
    if (var_guard36 != 0.0) {
        (0.0,)
    } else {
        (var_loop_,)
    }
};
        var_loop_ = assign9140_e9076;
        var_loop__rv = 0.0;

        let (assign9150_e9080,) = {
    if (var_guard36 != 0.0) {
        (p.p812,)
    } else {
        (var_kvsatac_i,)
    }
};
        var_kvsatac_i = assign9150_e9080;
        var_kvsatac_i_rv = 0.0;

        let assign9160_e9082: f64 = if param_given[813] { 1.0 } else { 0.0 };
        let assign9160_e9084: f64 = if assign9160_e9082 == 1.0 { 1.0 } else { 0.0 };
        var_guard146 = assign9160_e9084;
        var_guard146_rv = 0.0;

        let (assign9170_e9090,) = {
    if ((var_guard36 != 0.0) && (var_guard146 != 0.0)) {
        (p.p813,)
    } else {
        (var_kvsatac_i,)
    }
};
        var_kvsatac_i = assign9170_e9090;
        var_kvsatac_i_rv = 0.0;

        let assign9180_e9109: f64 = if (((var_sa_i > 0.0) && (var_sb_i > 0.0)) && ((var_nf_i == 1.0) || ((var_nf_i > 1.0) && (var_sd_i > 0.0)))) { 1.0 } else { 0.0 };
        var_guard147 = assign9180_e9109;
        var_guard147_rv = 0.0;

        *var_betnedge_p_slot = var_betnedge_p;
        *var_betnedge_p_rv_slot = var_betnedge_p_rv;
        *var_cfbedge_p_slot = var_cfbedge_p;
        *var_cfbedge_p_rv_slot = var_cfbedge_p_rv;
        *var_cfdedge_p_slot = var_cfdedge_p;
        *var_cfdedge_p_rv_slot = var_cfdedge_p_rv;
        *var_cfedge_p_slot = var_cfedge_p;
        *var_cfedge_p_rv_slot = var_cfedge_p_rv;
        *var_cfr_p_slot = var_cfr_p;
        *var_cfr_p_rv_slot = var_cfr_p_rv;
        *var_cfrd_p_slot = var_cfrd_p;
        *var_cfrd_p_rv_slot = var_cfrd_p_rv;
        *var_cgbov_p_slot = var_cgbov_p;
        *var_cgbov_p_rv_slot = var_cgbov_p_rv;
        *var_cgov_p_slot = var_cgov_p;
        *var_cgov_p_rv_slot = var_cgov_p_rv;
        *var_cgovd_p_slot = var_cgovd_p;
        *var_cgovd_p_rv_slot = var_cgovd_p_rv;
        *var_cinr_p_slot = var_cinr_p;
        *var_cinr_p_rv_slot = var_cinr_p_rv;
        *var_cinrd_p_slot = var_cinrd_p;
        *var_cinrd_p_rv_slot = var_cinrd_p_rv;
        *var_ctedge_p_slot = var_ctedge_p;
        *var_ctedge_p_rv_slot = var_ctedge_p_rv;
        *var_cth_p_slot = var_cth_p;
        *var_cth_p_rv_slot = var_cth_p_rv;
        *var_dphibedge_p_slot = var_dphibedge_p;
        *var_dphibedge_p_rv_slot = var_dphibedge_p_rv;
        *var_guard117_slot = var_guard117;
        *var_guard117_rv_slot = var_guard117_rv;
        *var_guard118_slot = var_guard118;
        *var_guard118_rv_slot = var_guard118_rv;
        *var_guard119_slot = var_guard119;
        *var_guard119_rv_slot = var_guard119_rv;
        *var_guard120_slot = var_guard120;
        *var_guard120_rv_slot = var_guard120_rv;
        *var_guard121_slot = var_guard121;
        *var_guard121_rv_slot = var_guard121_rv;
        *var_guard122_slot = var_guard122;
        *var_guard122_rv_slot = var_guard122_rv;
        *var_guard127_slot = var_guard127;
        *var_guard127_rv_slot = var_guard127_rv;
        *var_guard128_slot = var_guard128;
        *var_guard128_rv_slot = var_guard128_rv;
        *var_guard129_slot = var_guard129;
        *var_guard129_rv_slot = var_guard129_rv;
        *var_guard130_slot = var_guard130;
        *var_guard130_rv_slot = var_guard130_rv;
        *var_guard131_slot = var_guard131;
        *var_guard131_rv_slot = var_guard131_rv;
        *var_guard132_slot = var_guard132;
        *var_guard132_rv_slot = var_guard132_rv;
        *var_guard133_slot = var_guard133;
        *var_guard133_rv_slot = var_guard133_rv;
        *var_guard134_slot = var_guard134;
        *var_guard134_rv_slot = var_guard134_rv;
        *var_guard135_slot = var_guard135;
        *var_guard135_rv_slot = var_guard135_rv;
        *var_guard136_slot = var_guard136;
        *var_guard136_rv_slot = var_guard136_rv;
        *var_guard137_slot = var_guard137;
        *var_guard137_rv_slot = var_guard137_rv;
        *var_guard138_slot = var_guard138;
        *var_guard138_rv_slot = var_guard138_rv;
        *var_guard139_slot = var_guard139;
        *var_guard139_rv_slot = var_guard139_rv;
        *var_guard144_slot = var_guard144;
        *var_guard144_rv_slot = var_guard144_rv;
        *var_guard146_slot = var_guard146;
        *var_guard146_rv_slot = var_guard146_rv;
        *var_guard147_slot = var_guard147;
        *var_guard147_rv_slot = var_guard147_rv;
        *var_kvsatac_i_slot = var_kvsatac_i;
        *var_kvsatac_i_rv_slot = var_kvsatac_i_rv;
        *var_loop__slot = var_loop_;
        *var_loop__rv_slot = var_loop__rv;
        *var_neffedge_p_slot = var_neffedge_p;
        *var_neffedge_p_rv_slot = var_neffedge_p_rv;
        *var_pscebedge_p_slot = var_pscebedge_p;
        *var_pscebedge_p_rv_slot = var_pscebedge_p_rv;
        *var_pscededge_p_slot = var_pscededge_p;
        *var_pscededge_p_rv_slot = var_pscededge_p_rv;
        *var_psceedge_p_slot = var_psceedge_p;
        *var_psceedge_p_rv_slot = var_psceedge_p_rv;
        *var_stbetedge_p_slot = var_stbetedge_p;
        *var_stbetedge_p_rv_slot = var_stbetedge_p_rv;
        *var_stvfbedge_p_slot = var_stvfbedge_p;
        *var_stvfbedge_p_rv_slot = var_stvfbedge_p_rv;
        *var_tmpa_slot = var_tmpa;
        *var_tmpa_rv_slot = var_tmpa_rv;
        *var_tmpb_slot = var_tmpb;
        *var_tmpb_rv_slot = var_tmpb_rv;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_vfbedge_p_rv_slot = var_vfbedge_p_rv;
    }

    pub(super) fn stamp_reactive_block_14(
        p: &Parameters,
        var_dellps: f64,
        var_delwod: f64,
        var_guard147: f64,
        var_guard36: f64,
        var_invnf: f64,
        var_kvsatac_i: f64,
        var_l_i: f64,
        var_nf_i: f64,
        var_rta: f64,
        var_sa_i: f64,
        var_sb_i: f64,
        var_sc_i: f64,
        var_sd_i: f64,
        var_w_i: f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_rv_slot: &mut f64,
        var_betnedge_p_slot: &mut f64,
        var_betnedge_p_rv_slot: &mut f64,
        var_cf_p_slot: &mut f64,
        var_cf_p_rv_slot: &mut f64,
        var_cfedge_p_slot: &mut f64,
        var_cfedge_p_rv_slot: &mut f64,
        var_guard148_slot: &mut f64,
        var_guard148_rv_slot: &mut f64,
        var_guard149_slot: &mut f64,
        var_guard149_rv_slot: &mut f64,
        var_invsa_slot: &mut f64,
        var_invsa_rv_slot: &mut f64,
        var_invsaref_slot: &mut f64,
        var_invsaref_rv_slot: &mut f64,
        var_invsb_slot: &mut f64,
        var_invsb_rv_slot: &mut f64,
        var_invsbref_slot: &mut f64,
        var_invsbref_rv_slot: &mut f64,
        var_kstressu0_slot: &mut f64,
        var_kstressu0_rv_slot: &mut f64,
        var_kstressvth0_slot: &mut f64,
        var_kstressvth0_rv_slot: &mut f64,
        var_loop__slot: &mut f64,
        var_loop__rv_slot: &mut f64,
        var_lx_slot: &mut f64,
        var_lx_rv_slot: &mut f64,
        var_rhobeta_slot: &mut f64,
        var_rhobeta_rv_slot: &mut f64,
        var_rhobetaref_slot: &mut f64,
        var_rhobetaref_rv_slot: &mut f64,
        var_sca_i_slot: &mut f64,
        var_sca_i_rv_slot: &mut f64,
        var_scb_i_slot: &mut f64,
        var_scb_i_rv_slot: &mut f64,
        var_scc_i_slot: &mut f64,
        var_scc_i_rv_slot: &mut f64,
        var_temp0_slot: &mut f64,
        var_temp00_slot: &mut f64,
        var_temp00_rv_slot: &mut f64,
        var_temp0_rv_slot: &mut f64,
        var_templ_slot: &mut f64,
        var_templ_rv_slot: &mut f64,
        var_tempw_slot: &mut f64,
        var_tempw_rv_slot: &mut f64,
        var_thesat_p_slot: &mut f64,
        var_thesat_p_rv_slot: &mut f64,
        var_thesatac_p_slot: &mut f64,
        var_thesatac_p_rv_slot: &mut f64,
        var_tmpa_slot: &mut f64,
        var_tmpa_rv_slot: &mut f64,
        var_tmpb_slot: &mut f64,
        var_tmpb_rv_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vfb_p_rv_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_vfbedge_p_rv_slot: &mut f64,
        var_wx_slot: &mut f64,
        var_wx_rv_slot: &mut f64,
    ) {
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_rv: f64 = *var_betn_p_rv_slot;
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_betnedge_p_rv: f64 = *var_betnedge_p_rv_slot;
        let mut var_cf_p: f64 = *var_cf_p_slot;
        let mut var_cf_p_rv: f64 = *var_cf_p_rv_slot;
        let mut var_cfedge_p: f64 = *var_cfedge_p_slot;
        let mut var_cfedge_p_rv: f64 = *var_cfedge_p_rv_slot;
        let mut var_guard148: f64 = *var_guard148_slot;
        let mut var_guard148_rv: f64 = *var_guard148_rv_slot;
        let mut var_guard149: f64 = *var_guard149_slot;
        let mut var_guard149_rv: f64 = *var_guard149_rv_slot;
        let mut var_invsa: f64 = *var_invsa_slot;
        let mut var_invsa_rv: f64 = *var_invsa_rv_slot;
        let mut var_invsaref: f64 = *var_invsaref_slot;
        let mut var_invsaref_rv: f64 = *var_invsaref_rv_slot;
        let mut var_invsb: f64 = *var_invsb_slot;
        let mut var_invsb_rv: f64 = *var_invsb_rv_slot;
        let mut var_invsbref: f64 = *var_invsbref_slot;
        let mut var_invsbref_rv: f64 = *var_invsbref_rv_slot;
        let mut var_kstressu0: f64 = *var_kstressu0_slot;
        let mut var_kstressu0_rv: f64 = *var_kstressu0_rv_slot;
        let mut var_kstressvth0: f64 = *var_kstressvth0_slot;
        let mut var_kstressvth0_rv: f64 = *var_kstressvth0_rv_slot;
        let mut var_loop_: f64 = *var_loop__slot;
        let mut var_loop__rv: f64 = *var_loop__rv_slot;
        let mut var_lx: f64 = *var_lx_slot;
        let mut var_lx_rv: f64 = *var_lx_rv_slot;
        let mut var_rhobeta: f64 = *var_rhobeta_slot;
        let mut var_rhobeta_rv: f64 = *var_rhobeta_rv_slot;
        let mut var_rhobetaref: f64 = *var_rhobetaref_slot;
        let mut var_rhobetaref_rv: f64 = *var_rhobetaref_rv_slot;
        let mut var_sca_i: f64 = *var_sca_i_slot;
        let mut var_sca_i_rv: f64 = *var_sca_i_rv_slot;
        let mut var_scb_i: f64 = *var_scb_i_slot;
        let mut var_scb_i_rv: f64 = *var_scb_i_rv_slot;
        let mut var_scc_i: f64 = *var_scc_i_slot;
        let mut var_scc_i_rv: f64 = *var_scc_i_rv_slot;
        let mut var_temp0: f64 = *var_temp0_slot;
        let mut var_temp00: f64 = *var_temp00_slot;
        let mut var_temp00_rv: f64 = *var_temp00_rv_slot;
        let mut var_temp0_rv: f64 = *var_temp0_rv_slot;
        let mut var_templ: f64 = *var_templ_slot;
        let mut var_templ_rv: f64 = *var_templ_rv_slot;
        let mut var_tempw: f64 = *var_tempw_slot;
        let mut var_tempw_rv: f64 = *var_tempw_rv_slot;
        let mut var_thesat_p: f64 = *var_thesat_p_slot;
        let mut var_thesat_p_rv: f64 = *var_thesat_p_rv_slot;
        let mut var_thesatac_p: f64 = *var_thesatac_p_slot;
        let mut var_thesatac_p_rv: f64 = *var_thesatac_p_rv_slot;
        let mut var_tmpa: f64 = *var_tmpa_slot;
        let mut var_tmpa_rv: f64 = *var_tmpa_rv_slot;
        let mut var_tmpb: f64 = *var_tmpb_slot;
        let mut var_tmpb_rv: f64 = *var_tmpb_rv_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vfb_p_rv: f64 = *var_vfb_p_rv_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_vfbedge_p_rv: f64 = *var_vfbedge_p_rv_slot;
        let mut var_wx: f64 = *var_wx_slot;
        let mut var_wx_rv: f64 = *var_wx_rv_slot;

        let mut assign9190_loop_guard: usize = 0;
        while {
            let assign9190_cond_e9116: f64 = (var_nf_i - 0.5);
            let assign9190_cond_e9118: f64 = if (((var_guard36 != 0.0) && (var_guard147 != 0.0)) && (var_loop_ < assign9190_cond_e9116)) { 1.0 } else { 0.0 };
            assign9190_cond_e9118 != 0.0
        } {
            assign9190_loop_guard += 1;
            assert!(assign9190_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign9190_body0_e9138,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9190_body0_e9127: f64 = (0.5 * var_l_i);
        let assign9190_body0_e9128: f64 = (var_sa_i + assign9190_body0_e9127);
        let assign9190_body0_e9132: f64 = (var_sd_i + var_l_i);
        let assign9190_body0_e9133: f64 = (var_loop_ * assign9190_body0_e9132);
        let assign9190_body0_e9134: f64 = (assign9190_body0_e9128 + assign9190_body0_e9133);
        let assign9190_body0_e9135: f64 = (1.0 / assign9190_body0_e9134);
        let assign9190_body0_e9136: f64 = (var_tmpa + assign9190_body0_e9135);
        (assign9190_body0_e9136,)
    } else {
        (var_tmpa,)
    }
};
            var_tmpa = assign9190_body0_e9138;
            var_tmpa_rv = 0.0;
            let (assign9190_body1_e9158,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9190_body1_e9147: f64 = (0.5 * var_l_i);
        let assign9190_body1_e9148: f64 = (var_sb_i + assign9190_body1_e9147);
        let assign9190_body1_e9152: f64 = (var_sd_i + var_l_i);
        let assign9190_body1_e9153: f64 = (var_loop_ * assign9190_body1_e9152);
        let assign9190_body1_e9154: f64 = (assign9190_body1_e9148 + assign9190_body1_e9153);
        let assign9190_body1_e9155: f64 = (1.0 / assign9190_body1_e9154);
        let assign9190_body1_e9156: f64 = (var_tmpb + assign9190_body1_e9155);
        (assign9190_body1_e9156,)
    } else {
        (var_tmpb,)
    }
};
            var_tmpb = assign9190_body1_e9158;
            var_tmpb_rv = 0.0;
            let (assign9190_body2_e9166,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9190_body2_e9164: f64 = (var_loop_ + 1.0);
        (assign9190_body2_e9164,)
    } else {
        (var_loop_,)
    }
};
            var_loop_ = assign9190_body2_e9166;
            var_loop__rv = 0.0;
        }

        let (assign9200_e9174,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9200_e9172: f64 = (var_tmpa * var_invnf);
        (assign9200_e9172,)
    } else {
        (var_invsa,)
    }
};
        var_invsa = assign9200_e9174;
        var_invsa_rv = 0.0;

        let (assign9210_e9182,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9210_e9180: f64 = (var_tmpb * var_invnf);
        (assign9210_e9180,)
    } else {
        (var_invsb,)
    }
};
        var_invsb = assign9210_e9182;
        var_invsb_rv = 0.0;

        let (assign9220_e9194,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9220_e9190: f64 = (0.5 * var_l_i);
        let assign9220_e9191: f64 = (p.p808 + assign9220_e9190);
        let assign9220_e9192: f64 = (1.0 / assign9220_e9191);
        (assign9220_e9192,)
    } else {
        (var_invsaref,)
    }
};
        var_invsaref = assign9220_e9194;
        var_invsaref_rv = 0.0;

        let (assign9230_e9206,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9230_e9202: f64 = (0.5 * var_l_i);
        let assign9230_e9203: f64 = (p.p809 + assign9230_e9202);
        let assign9230_e9204: f64 = (1.0 / assign9230_e9203);
        (assign9230_e9204,)
    } else {
        (var_invsbref,)
    }
};
        var_invsbref = assign9230_e9206;
        var_invsbref_rv = 0.0;

        let (assign9240_e9221,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9240_e9212: f64 = (var_l_i + var_dellps);
        let (assign9240_e9219,) = {
            if (assign9240_e9212 > 1e-9) {
                let assign9240_e9217: f64 = (var_l_i + var_dellps);
                (assign9240_e9217,)
            } else {
                (1e-9,)
            }
        };
        (assign9240_e9219,)
    } else {
        (var_lx,)
    }
};
        var_lx = assign9240_e9221;
        var_lx_rv = 0.0;

        let (assign9250_e9240,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9250_e9227: f64 = (var_w_i + var_delwod);
        let assign9250_e9229: f64 = (assign9250_e9227 + p.p810);
        let (assign9250_e9238,) = {
            if (assign9250_e9229 > 1e-9) {
                let assign9250_e9234: f64 = (var_w_i + var_delwod);
                let assign9250_e9236: f64 = (assign9250_e9234 + p.p810);
                (assign9250_e9236,)
            } else {
                (1e-9,)
            }
        };
        (assign9250_e9238,)
    } else {
        (var_wx,)
    }
};
        var_wx = assign9250_e9240;
        var_wx_rv = 0.0;

        let (assign9260_e9250,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9260_e9247: f64 = (var_lx).powf(p.p818);
        let assign9260_e9248: f64 = (1.0 / assign9260_e9247);
        (assign9260_e9248,)
    } else {
        (var_templ,)
    }
};
        var_templ = assign9260_e9250;
        var_templ_rv = 0.0;

        let (assign9270_e9260,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9270_e9257: f64 = (var_wx).powf(p.p819);
        let assign9270_e9258: f64 = (1.0 / assign9270_e9257);
        (assign9270_e9258,)
    } else {
        (var_tempw,)
    }
};
        var_tempw = assign9270_e9260;
        var_tempw_rv = 0.0;

        let (assign9280_e9288,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9280_e9267: f64 = (p.p815 * var_templ);
        let assign9280_e9268: f64 = (1.0 + assign9280_e9267);
        let assign9280_e9271: f64 = (p.p816 * var_tempw);
        let assign9280_e9272: f64 = (assign9280_e9268 + assign9280_e9271);
        let assign9280_e9275: f64 = (p.p817 * var_templ);
        let assign9280_e9277: f64 = (assign9280_e9275 * var_tempw);
        let assign9280_e9278: f64 = (assign9280_e9272 + assign9280_e9277);
        let assign9280_e9283: f64 = (var_rta - 1.0);
        let assign9280_e9284: f64 = (p.p814 * assign9280_e9283);
        let assign9280_e9285: f64 = (1.0 + assign9280_e9284);
        let assign9280_e9286: f64 = (assign9280_e9278 * assign9280_e9285);
        (assign9280_e9286,)
    } else {
        (var_kstressu0,)
    }
};
        var_kstressu0 = assign9280_e9288;
        var_kstressu0_rv = 0.0;

        let (assign9290_e9300,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9290_e9295: f64 = (var_invsa + var_invsb);
        let assign9290_e9296: f64 = (p.p811 * assign9290_e9295);
        let assign9290_e9298: f64 = (assign9290_e9296 / var_kstressu0);
        (assign9290_e9298,)
    } else {
        (var_rhobeta,)
    }
};
        var_rhobeta = assign9290_e9300;
        var_rhobeta_rv = 0.0;

        let (assign9300_e9312,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9300_e9307: f64 = (var_invsaref + var_invsbref);
        let assign9300_e9308: f64 = (p.p811 * assign9300_e9307);
        let assign9300_e9310: f64 = (assign9300_e9308 / var_kstressu0);
        (assign9300_e9310,)
    } else {
        (var_rhobetaref,)
    }
};
        var_rhobetaref = assign9300_e9312;
        var_rhobetaref_rv = 0.0;

        let (assign9310_e9322,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9310_e9319: f64 = (var_lx).powf(p.p824);
        let assign9310_e9320: f64 = (1.0 / assign9310_e9319);
        (assign9310_e9320,)
    } else {
        (var_templ,)
    }
};
        var_templ = assign9310_e9322;
        var_templ_rv = 0.0;

        let (assign9320_e9332,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9320_e9329: f64 = (var_wx).powf(p.p825);
        let assign9320_e9330: f64 = (1.0 / assign9320_e9329);
        (assign9320_e9330,)
    } else {
        (var_tempw,)
    }
};
        var_tempw = assign9320_e9332;
        var_tempw_rv = 0.0;

        let (assign9330_e9352,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9330_e9339: f64 = (p.p821 * var_templ);
        let assign9330_e9340: f64 = (1.0 + assign9330_e9339);
        let assign9330_e9343: f64 = (p.p822 * var_tempw);
        let assign9330_e9344: f64 = (assign9330_e9340 + assign9330_e9343);
        let assign9330_e9347: f64 = (p.p823 * var_templ);
        let assign9330_e9349: f64 = (assign9330_e9347 * var_tempw);
        let assign9330_e9350: f64 = (assign9330_e9344 + assign9330_e9349);
        (assign9330_e9350,)
    } else {
        (var_kstressvth0,)
    }
};
        var_kstressvth0 = assign9330_e9352;
        var_kstressvth0_rv = 0.0;

        let (assign9340_e9364,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9340_e9358: f64 = (var_invsa + var_invsb);
        let assign9340_e9360: f64 = (assign9340_e9358 - var_invsaref);
        let assign9340_e9362: f64 = (assign9340_e9360 - var_invsbref);
        (assign9340_e9362,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign9340_e9364;
        var_temp0_rv = 0.0;

        let (assign9350_e9376,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9350_e9370: f64 = (1.0 + var_rhobeta);
        let assign9350_e9373: f64 = (1.0 + var_rhobetaref);
        let assign9350_e9374: f64 = (assign9350_e9370 / assign9350_e9373);
        (assign9350_e9374,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9350_e9376;
        var_temp00_rv = 0.0;

        let (assign9360_e9384,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9360_e9382: f64 = (var_betn_p * var_temp00);
        (assign9360_e9382,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign9360_e9384;
        var_betn_p_rv = 0.0;

        let (assign9370_e9404,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9370_e9390: f64 = (var_thesat_p * var_temp00);
        let assign9370_e9394: f64 = (p.p812 * var_rhobetaref);
        let assign9370_e9395: f64 = (1.0 + assign9370_e9394);
        let assign9370_e9396: f64 = (assign9370_e9390 * assign9370_e9395);
        let assign9370_e9400: f64 = (p.p812 * var_rhobeta);
        let assign9370_e9401: f64 = (1.0 + assign9370_e9400);
        let assign9370_e9402: f64 = (assign9370_e9396 / assign9370_e9401);
        (assign9370_e9402,)
    } else {
        (var_thesat_p,)
    }
};
        var_thesat_p = assign9370_e9404;
        var_thesat_p_rv = 0.0;

        let (assign9380_e9424,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9380_e9410: f64 = (var_thesatac_p * var_temp00);
        let assign9380_e9414: f64 = (var_kvsatac_i * var_rhobetaref);
        let assign9380_e9415: f64 = (1.0 + assign9380_e9414);
        let assign9380_e9416: f64 = (assign9380_e9410 * assign9380_e9415);
        let assign9380_e9420: f64 = (var_kvsatac_i * var_rhobeta);
        let assign9380_e9421: f64 = (1.0 + assign9380_e9420);
        let assign9380_e9422: f64 = (assign9380_e9416 / assign9380_e9421);
        (assign9380_e9422,)
    } else {
        (var_thesatac_p,)
    }
};
        var_thesatac_p = assign9380_e9424;
        var_thesatac_p_rv = 0.0;

        let (assign9390_e9432,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9390_e9430: f64 = (var_betnedge_p * var_temp00);
        (assign9390_e9430,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign9390_e9432;
        var_betnedge_p_rv = 0.0;

        let (assign9400_e9442,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9400_e9438: f64 = (p.p820 * var_temp0);
        let assign9400_e9440: f64 = (assign9400_e9438 / var_kstressvth0);
        (assign9400_e9440,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9400_e9442;
        var_temp00_rv = 0.0;

        let (assign9410_e9450,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9410_e9448: f64 = (var_vfb_p + var_temp00);
        (assign9410_e9448,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign9410_e9450;
        var_vfb_p_rv = 0.0;

        let (assign9420_e9458,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9420_e9456: f64 = (var_vfbedge_p + var_temp00);
        (assign9420_e9456,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign9420_e9458;
        var_vfbedge_p_rv = 0.0;

        let (assign9430_e9470,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9430_e9464: f64 = (p.p826 * var_temp0);
        let assign9430_e9467: f64 = (var_kstressvth0).powf(p.p827);
        let assign9430_e9468: f64 = (assign9430_e9464 / assign9430_e9467);
        (assign9430_e9468,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9430_e9470;
        var_temp00_rv = 0.0;

        let (assign9440_e9478,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9440_e9476: f64 = (var_cf_p + var_temp00);
        (assign9440_e9476,)
    } else {
        (var_cf_p,)
    }
};
        var_cf_p = assign9440_e9478;
        var_cf_p_rv = 0.0;

        let (assign9450_e9486,) = {
    if ((var_guard36 != 0.0) && (var_guard147 != 0.0)) {
        let assign9450_e9484: f64 = (var_cfedge_p + var_temp00);
        (assign9450_e9484,)
    } else {
        (var_cfedge_p,)
    }
};
        var_cfedge_p = assign9450_e9486;
        var_cfedge_p_rv = 0.0;

        let assign9460_e9501: f64 = if ((((var_sca_i > 0.0) || (var_scb_i > 0.0)) || (var_scc_i > 0.0)) || (var_sc_i > 0.0)) { 1.0 } else { 0.0 };
        var_guard148 = assign9460_e9501;
        var_guard148_rv = 0.0;

        let assign9470_e9512: f64 = if (((var_sca_i == 0.0) && (var_scb_i == 0.0)) && (var_scc_i == 0.0)) { 1.0 } else { 0.0 };
        var_guard149 = assign9470_e9512;
        var_guard149_rv = 0.0;

        let (assign9480_e9522,) = {
    if (((var_guard36 != 0.0) && (var_guard148 != 0.0)) && (var_guard149 != 0.0)) {
        let assign9480_e9520: f64 = (var_sc_i + var_w_i);
        (assign9480_e9520,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign9480_e9522;
        var_temp0_rv = 0.0;

        let (assign9490_e9532,) = {
    if (((var_guard36 != 0.0) && (var_guard148 != 0.0)) && (var_guard149 != 0.0)) {
        let assign9490_e9530: f64 = (1.0 / p.p828);
        (assign9490_e9530,)
    } else {
        (var_temp00,)
    }
};
        var_temp00 = assign9490_e9532;
        var_temp00_rv = 0.0;

        let (assign9500_e9546,) = {
    if (((var_guard36 != 0.0) && (var_guard148 != 0.0)) && (var_guard149 != 0.0)) {
        let assign9500_e9540: f64 = (p.p828 * p.p828);
        let assign9500_e9543: f64 = (var_sc_i * var_temp0);
        let assign9500_e9544: f64 = (assign9500_e9540 / assign9500_e9543);
        (assign9500_e9544,)
    } else {
        (var_sca_i,)
    }
};
        var_sca_i = assign9500_e9546;
        var_sca_i_rv = 0.0;

        let (assign9510_e9586,) = {
    if (((var_guard36 != 0.0) && (var_guard148 != 0.0)) && (var_guard149 != 0.0)) {
        let assign9510_e9554: f64 = (0.1 * var_sc_i);
        let assign9510_e9557: f64 = (0.01 * p.p828);
        let assign9510_e9558: f64 = (assign9510_e9554 + assign9510_e9557);
        let assign9510_e9560: f64 = (-10.0);
        let assign9510_e9562: f64 = (assign9510_e9560 * var_sc_i);
        let assign9510_e9564: f64 = (assign9510_e9562 * var_temp00);
        let assign9510_e9565: f64 = (assign9510_e9564).exp();
        let assign9510_e9566: f64 = (assign9510_e9558 * assign9510_e9565);
        let assign9510_e9569: f64 = (0.1 * var_temp0);
        let assign9510_e9572: f64 = (0.01 * p.p828);
        let assign9510_e9573: f64 = (assign9510_e9569 + assign9510_e9572);
        let assign9510_e9575: f64 = (-10.0);
        let assign9510_e9577: f64 = (assign9510_e9575 * var_temp0);
        let assign9510_e9579: f64 = (assign9510_e9577 * var_temp00);
        let assign9510_e9580: f64 = (assign9510_e9579).exp();
        let assign9510_e9581: f64 = (assign9510_e9573 * assign9510_e9580);
        let assign9510_e9582: f64 = (assign9510_e9566 - assign9510_e9581);
        let assign9510_e9584: f64 = (assign9510_e9582 / var_w_i);
        (assign9510_e9584,)
    } else {
        (var_scb_i,)
    }
};
        var_scb_i = assign9510_e9586;
        var_scb_i_rv = 0.0;

        let (assign9520_e9626,) = {
    if (((var_guard36 != 0.0) && (var_guard148 != 0.0)) && (var_guard149 != 0.0)) {
        let assign9520_e9594: f64 = (0.05 * var_sc_i);
        let assign9520_e9597: f64 = (0.0025 * p.p828);
        let assign9520_e9598: f64 = (assign9520_e9594 + assign9520_e9597);
        let assign9520_e9600: f64 = (-20.0);
        let assign9520_e9602: f64 = (assign9520_e9600 * var_sc_i);
        let assign9520_e9604: f64 = (assign9520_e9602 * var_temp00);
        let assign9520_e9605: f64 = (assign9520_e9604).exp();
        let assign9520_e9606: f64 = (assign9520_e9598 * assign9520_e9605);
        let assign9520_e9609: f64 = (0.05 * var_temp0);
        let assign9520_e9612: f64 = (0.0025 * p.p828);
        let assign9520_e9613: f64 = (assign9520_e9609 + assign9520_e9612);
        let assign9520_e9615: f64 = (-20.0);
        let assign9520_e9617: f64 = (assign9520_e9615 * var_temp0);
        let assign9520_e9619: f64 = (assign9520_e9617 * var_temp00);
        let assign9520_e9620: f64 = (assign9520_e9619).exp();
        let assign9520_e9621: f64 = (assign9520_e9613 * assign9520_e9620);
        let assign9520_e9622: f64 = (assign9520_e9606 - assign9520_e9621);
        let assign9520_e9624: f64 = (assign9520_e9622 / var_w_i);
        (assign9520_e9624,)
    } else {
        (var_scc_i,)
    }
};
        var_scc_i = assign9520_e9626;
        var_scc_i_rv = 0.0;

        *var_betn_p_slot = var_betn_p;
        *var_betn_p_rv_slot = var_betn_p_rv;
        *var_betnedge_p_slot = var_betnedge_p;
        *var_betnedge_p_rv_slot = var_betnedge_p_rv;
        *var_cf_p_slot = var_cf_p;
        *var_cf_p_rv_slot = var_cf_p_rv;
        *var_cfedge_p_slot = var_cfedge_p;
        *var_cfedge_p_rv_slot = var_cfedge_p_rv;
        *var_guard148_slot = var_guard148;
        *var_guard148_rv_slot = var_guard148_rv;
        *var_guard149_slot = var_guard149;
        *var_guard149_rv_slot = var_guard149_rv;
        *var_invsa_slot = var_invsa;
        *var_invsa_rv_slot = var_invsa_rv;
        *var_invsaref_slot = var_invsaref;
        *var_invsaref_rv_slot = var_invsaref_rv;
        *var_invsb_slot = var_invsb;
        *var_invsb_rv_slot = var_invsb_rv;
        *var_invsbref_slot = var_invsbref;
        *var_invsbref_rv_slot = var_invsbref_rv;
        *var_kstressu0_slot = var_kstressu0;
        *var_kstressu0_rv_slot = var_kstressu0_rv;
        *var_kstressvth0_slot = var_kstressvth0;
        *var_kstressvth0_rv_slot = var_kstressvth0_rv;
        *var_loop__slot = var_loop_;
        *var_loop__rv_slot = var_loop__rv;
        *var_lx_slot = var_lx;
        *var_lx_rv_slot = var_lx_rv;
        *var_rhobeta_slot = var_rhobeta;
        *var_rhobeta_rv_slot = var_rhobeta_rv;
        *var_rhobetaref_slot = var_rhobetaref;
        *var_rhobetaref_rv_slot = var_rhobetaref_rv;
        *var_sca_i_slot = var_sca_i;
        *var_sca_i_rv_slot = var_sca_i_rv;
        *var_scb_i_slot = var_scb_i;
        *var_scb_i_rv_slot = var_scb_i_rv;
        *var_scc_i_slot = var_scc_i;
        *var_scc_i_rv_slot = var_scc_i_rv;
        *var_temp0_slot = var_temp0;
        *var_temp00_slot = var_temp00;
        *var_temp00_rv_slot = var_temp00_rv;
        *var_temp0_rv_slot = var_temp0_rv;
        *var_templ_slot = var_templ;
        *var_templ_rv_slot = var_templ_rv;
        *var_tempw_slot = var_tempw;
        *var_tempw_rv_slot = var_tempw_rv;
        *var_thesat_p_slot = var_thesat_p;
        *var_thesat_p_rv_slot = var_thesat_p_rv;
        *var_thesatac_p_slot = var_thesatac_p;
        *var_thesatac_p_rv_slot = var_thesatac_p_rv;
        *var_tmpa_slot = var_tmpa;
        *var_tmpa_rv_slot = var_tmpa_rv;
        *var_tmpb_slot = var_tmpb;
        *var_tmpb_rv_slot = var_tmpb_rv;
        *var_vfb_p_slot = var_vfb_p;
        *var_vfb_p_rv_slot = var_vfb_p_rv;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_vfbedge_p_rv_slot = var_vfbedge_p_rv;
        *var_wx_slot = var_wx;
        *var_wx_rv_slot = var_wx_rv;
    }

    pub(super) fn stamp_reactive_block_15(
        p: &Parameters,
        var_alp1_p: f64,
        var_alp2_p: f64,
        var_alp_p: f64,
        var_ax_p: f64,
        var_cf_p: f64,
        var_cfb_p: f64,
        var_cfd_p: f64,
        var_cs_p: f64,
        var_ct_p: f64,
        var_ctb_p: f64,
        var_ctg_p: f64,
        var_dphib_p: f64,
        var_dvsbnud_p: f64,
        var_epsrox_p: f64,
        var_feta_p: f64,
        var_gfacnud_p: f64,
        var_guard148: f64,
        var_guard36: f64,
        var_kuowe: f64,
        var_kvthowe: f64,
        var_mue_p: f64,
        var_neff_p: f64,
        var_nov_p: f64,
        var_novd_p: f64,
        var_np_p: f64,
        var_psce_p: f64,
        var_psceb_p: f64,
        var_psced_p: f64,
        var_rs_p: f64,
        var_rsb_p: f64,
        var_rsg_p: f64,
        var_sca_i: f64,
        var_scb_i: f64,
        var_scc_i: f64,
        var_st2vfb_p: f64,
        var_stbet_p: f64,
        var_stcs_p: f64,
        var_stct_p: f64,
        var_stmue_p: f64,
        var_strs_p: f64,
        var_stthecs_p: f64,
        var_stthemu_p: f64,
        var_stthesat_p: f64,
        var_stvfb_p: f64,
        var_stxcor_p: f64,
        var_thecs_p: f64,
        var_themu_p: f64,
        var_thesat_p: f64,
        var_thesatb_p: f64,
        var_thesatg_p: f64,
        var_thesatt_p: f64,
        var_tox_p: f64,
        var_toxov_p: f64,
        var_toxovd_p: f64,
        var_vsbnud_p: f64,
        var_xcor_p: f64,
        var_alp1_i_slot: &mut f64,
        var_alp1_i_rv_slot: &mut f64,
        var_alp2_i_slot: &mut f64,
        var_alp2_i_rv_slot: &mut f64,
        var_alp_i_slot: &mut f64,
        var_alp_i_rv_slot: &mut f64,
        var_ax_i_slot: &mut f64,
        var_ax_i_rv_slot: &mut f64,
        var_betn_i_slot: &mut f64,
        var_betn_i_rv_slot: &mut f64,
        var_betn_p_slot: &mut f64,
        var_betn_p_rv_slot: &mut f64,
        var_betnedge_p_slot: &mut f64,
        var_betnedge_p_rv_slot: &mut f64,
        var_cf_i_slot: &mut f64,
        var_cf_i_rv_slot: &mut f64,
        var_cfb_i_slot: &mut f64,
        var_cfb_i_rv_slot: &mut f64,
        var_cfd_i_slot: &mut f64,
        var_cfd_i_rv_slot: &mut f64,
        var_cs_i_slot: &mut f64,
        var_cs_i_rv_slot: &mut f64,
        var_ct_i_slot: &mut f64,
        var_ct_i_rv_slot: &mut f64,
        var_ctb_i_slot: &mut f64,
        var_ctb_i_rv_slot: &mut f64,
        var_ctg_i_slot: &mut f64,
        var_ctg_i_rv_slot: &mut f64,
        var_dphib_i_slot: &mut f64,
        var_dphib_i_rv_slot: &mut f64,
        var_dvsbnud_i_slot: &mut f64,
        var_dvsbnud_i_rv_slot: &mut f64,
        var_epsrox_i_slot: &mut f64,
        var_epsrox_i_rv_slot: &mut f64,
        var_feta_i_slot: &mut f64,
        var_feta_i_rv_slot: &mut f64,
        var_gfacnud_i_slot: &mut f64,
        var_gfacnud_i_rv_slot: &mut f64,
        var_mue_i_slot: &mut f64,
        var_mue_i_rv_slot: &mut f64,
        var_neff_i_slot: &mut f64,
        var_neff_i_rv_slot: &mut f64,
        var_nov_i_slot: &mut f64,
        var_nov_i_rv_slot: &mut f64,
        var_novd_i_slot: &mut f64,
        var_novd_i_rv_slot: &mut f64,
        var_np_i_slot: &mut f64,
        var_np_i_rv_slot: &mut f64,
        var_psce_i_slot: &mut f64,
        var_psce_i_rv_slot: &mut f64,
        var_psceb_i_slot: &mut f64,
        var_psceb_i_rv_slot: &mut f64,
        var_psced_i_slot: &mut f64,
        var_psced_i_rv_slot: &mut f64,
        var_rs_i_slot: &mut f64,
        var_rs_i_rv_slot: &mut f64,
        var_rsb_i_slot: &mut f64,
        var_rsb_i_rv_slot: &mut f64,
        var_rsg_i_slot: &mut f64,
        var_rsg_i_rv_slot: &mut f64,
        var_st2vfb_i_slot: &mut f64,
        var_st2vfb_i_rv_slot: &mut f64,
        var_stbet_i_slot: &mut f64,
        var_stbet_i_rv_slot: &mut f64,
        var_stcs_i_slot: &mut f64,
        var_stcs_i_rv_slot: &mut f64,
        var_stct_i_slot: &mut f64,
        var_stct_i_rv_slot: &mut f64,
        var_stmue_i_slot: &mut f64,
        var_stmue_i_rv_slot: &mut f64,
        var_strs_i_slot: &mut f64,
        var_strs_i_rv_slot: &mut f64,
        var_stthecs_i_slot: &mut f64,
        var_stthecs_i_rv_slot: &mut f64,
        var_stthemu_i_slot: &mut f64,
        var_stthemu_i_rv_slot: &mut f64,
        var_stthesat_i_slot: &mut f64,
        var_stthesat_i_rv_slot: &mut f64,
        var_stvfb_i_slot: &mut f64,
        var_stvfb_i_rv_slot: &mut f64,
        var_stxcor_i_slot: &mut f64,
        var_stxcor_i_rv_slot: &mut f64,
        var_temp0_slot: &mut f64,
        var_temp0_rv_slot: &mut f64,
        var_thecs_i_slot: &mut f64,
        var_thecs_i_rv_slot: &mut f64,
        var_themu_i_slot: &mut f64,
        var_themu_i_rv_slot: &mut f64,
        var_thesat_i_slot: &mut f64,
        var_thesat_i_rv_slot: &mut f64,
        var_thesatb_i_slot: &mut f64,
        var_thesatb_i_rv_slot: &mut f64,
        var_thesatg_i_slot: &mut f64,
        var_thesatg_i_rv_slot: &mut f64,
        var_thesatt_i_slot: &mut f64,
        var_thesatt_i_rv_slot: &mut f64,
        var_tox_i_slot: &mut f64,
        var_tox_i_rv_slot: &mut f64,
        var_toxov_i_slot: &mut f64,
        var_toxov_i_rv_slot: &mut f64,
        var_toxovd_i_slot: &mut f64,
        var_toxovd_i_rv_slot: &mut f64,
        var_vfb_i_slot: &mut f64,
        var_vfb_i_rv_slot: &mut f64,
        var_vfb_p_slot: &mut f64,
        var_vfb_p_rv_slot: &mut f64,
        var_vfbedge_p_slot: &mut f64,
        var_vfbedge_p_rv_slot: &mut f64,
        var_vsbnud_i_slot: &mut f64,
        var_vsbnud_i_rv_slot: &mut f64,
        var_xcor_i_slot: &mut f64,
        var_xcor_i_rv_slot: &mut f64,
    ) {
        let mut var_alp1_i: f64 = *var_alp1_i_slot;
        let mut var_alp1_i_rv: f64 = *var_alp1_i_rv_slot;
        let mut var_alp2_i: f64 = *var_alp2_i_slot;
        let mut var_alp2_i_rv: f64 = *var_alp2_i_rv_slot;
        let mut var_alp_i: f64 = *var_alp_i_slot;
        let mut var_alp_i_rv: f64 = *var_alp_i_rv_slot;
        let mut var_ax_i: f64 = *var_ax_i_slot;
        let mut var_ax_i_rv: f64 = *var_ax_i_rv_slot;
        let mut var_betn_i: f64 = *var_betn_i_slot;
        let mut var_betn_i_rv: f64 = *var_betn_i_rv_slot;
        let mut var_betn_p: f64 = *var_betn_p_slot;
        let mut var_betn_p_rv: f64 = *var_betn_p_rv_slot;
        let mut var_betnedge_p: f64 = *var_betnedge_p_slot;
        let mut var_betnedge_p_rv: f64 = *var_betnedge_p_rv_slot;
        let mut var_cf_i: f64 = *var_cf_i_slot;
        let mut var_cf_i_rv: f64 = *var_cf_i_rv_slot;
        let mut var_cfb_i: f64 = *var_cfb_i_slot;
        let mut var_cfb_i_rv: f64 = *var_cfb_i_rv_slot;
        let mut var_cfd_i: f64 = *var_cfd_i_slot;
        let mut var_cfd_i_rv: f64 = *var_cfd_i_rv_slot;
        let mut var_cs_i: f64 = *var_cs_i_slot;
        let mut var_cs_i_rv: f64 = *var_cs_i_rv_slot;
        let mut var_ct_i: f64 = *var_ct_i_slot;
        let mut var_ct_i_rv: f64 = *var_ct_i_rv_slot;
        let mut var_ctb_i: f64 = *var_ctb_i_slot;
        let mut var_ctb_i_rv: f64 = *var_ctb_i_rv_slot;
        let mut var_ctg_i: f64 = *var_ctg_i_slot;
        let mut var_ctg_i_rv: f64 = *var_ctg_i_rv_slot;
        let mut var_dphib_i: f64 = *var_dphib_i_slot;
        let mut var_dphib_i_rv: f64 = *var_dphib_i_rv_slot;
        let mut var_dvsbnud_i: f64 = *var_dvsbnud_i_slot;
        let mut var_dvsbnud_i_rv: f64 = *var_dvsbnud_i_rv_slot;
        let mut var_epsrox_i: f64 = *var_epsrox_i_slot;
        let mut var_epsrox_i_rv: f64 = *var_epsrox_i_rv_slot;
        let mut var_feta_i: f64 = *var_feta_i_slot;
        let mut var_feta_i_rv: f64 = *var_feta_i_rv_slot;
        let mut var_gfacnud_i: f64 = *var_gfacnud_i_slot;
        let mut var_gfacnud_i_rv: f64 = *var_gfacnud_i_rv_slot;
        let mut var_mue_i: f64 = *var_mue_i_slot;
        let mut var_mue_i_rv: f64 = *var_mue_i_rv_slot;
        let mut var_neff_i: f64 = *var_neff_i_slot;
        let mut var_neff_i_rv: f64 = *var_neff_i_rv_slot;
        let mut var_nov_i: f64 = *var_nov_i_slot;
        let mut var_nov_i_rv: f64 = *var_nov_i_rv_slot;
        let mut var_novd_i: f64 = *var_novd_i_slot;
        let mut var_novd_i_rv: f64 = *var_novd_i_rv_slot;
        let mut var_np_i: f64 = *var_np_i_slot;
        let mut var_np_i_rv: f64 = *var_np_i_rv_slot;
        let mut var_psce_i: f64 = *var_psce_i_slot;
        let mut var_psce_i_rv: f64 = *var_psce_i_rv_slot;
        let mut var_psceb_i: f64 = *var_psceb_i_slot;
        let mut var_psceb_i_rv: f64 = *var_psceb_i_rv_slot;
        let mut var_psced_i: f64 = *var_psced_i_slot;
        let mut var_psced_i_rv: f64 = *var_psced_i_rv_slot;
        let mut var_rs_i: f64 = *var_rs_i_slot;
        let mut var_rs_i_rv: f64 = *var_rs_i_rv_slot;
        let mut var_rsb_i: f64 = *var_rsb_i_slot;
        let mut var_rsb_i_rv: f64 = *var_rsb_i_rv_slot;
        let mut var_rsg_i: f64 = *var_rsg_i_slot;
        let mut var_rsg_i_rv: f64 = *var_rsg_i_rv_slot;
        let mut var_st2vfb_i: f64 = *var_st2vfb_i_slot;
        let mut var_st2vfb_i_rv: f64 = *var_st2vfb_i_rv_slot;
        let mut var_stbet_i: f64 = *var_stbet_i_slot;
        let mut var_stbet_i_rv: f64 = *var_stbet_i_rv_slot;
        let mut var_stcs_i: f64 = *var_stcs_i_slot;
        let mut var_stcs_i_rv: f64 = *var_stcs_i_rv_slot;
        let mut var_stct_i: f64 = *var_stct_i_slot;
        let mut var_stct_i_rv: f64 = *var_stct_i_rv_slot;
        let mut var_stmue_i: f64 = *var_stmue_i_slot;
        let mut var_stmue_i_rv: f64 = *var_stmue_i_rv_slot;
        let mut var_strs_i: f64 = *var_strs_i_slot;
        let mut var_strs_i_rv: f64 = *var_strs_i_rv_slot;
        let mut var_stthecs_i: f64 = *var_stthecs_i_slot;
        let mut var_stthecs_i_rv: f64 = *var_stthecs_i_rv_slot;
        let mut var_stthemu_i: f64 = *var_stthemu_i_slot;
        let mut var_stthemu_i_rv: f64 = *var_stthemu_i_rv_slot;
        let mut var_stthesat_i: f64 = *var_stthesat_i_slot;
        let mut var_stthesat_i_rv: f64 = *var_stthesat_i_rv_slot;
        let mut var_stvfb_i: f64 = *var_stvfb_i_slot;
        let mut var_stvfb_i_rv: f64 = *var_stvfb_i_rv_slot;
        let mut var_stxcor_i: f64 = *var_stxcor_i_slot;
        let mut var_stxcor_i_rv: f64 = *var_stxcor_i_rv_slot;
        let mut var_temp0: f64 = *var_temp0_slot;
        let mut var_temp0_rv: f64 = *var_temp0_rv_slot;
        let mut var_thecs_i: f64 = *var_thecs_i_slot;
        let mut var_thecs_i_rv: f64 = *var_thecs_i_rv_slot;
        let mut var_themu_i: f64 = *var_themu_i_slot;
        let mut var_themu_i_rv: f64 = *var_themu_i_rv_slot;
        let mut var_thesat_i: f64 = *var_thesat_i_slot;
        let mut var_thesat_i_rv: f64 = *var_thesat_i_rv_slot;
        let mut var_thesatb_i: f64 = *var_thesatb_i_slot;
        let mut var_thesatb_i_rv: f64 = *var_thesatb_i_rv_slot;
        let mut var_thesatg_i: f64 = *var_thesatg_i_slot;
        let mut var_thesatg_i_rv: f64 = *var_thesatg_i_rv_slot;
        let mut var_thesatt_i: f64 = *var_thesatt_i_slot;
        let mut var_thesatt_i_rv: f64 = *var_thesatt_i_rv_slot;
        let mut var_tox_i: f64 = *var_tox_i_slot;
        let mut var_tox_i_rv: f64 = *var_tox_i_rv_slot;
        let mut var_toxov_i: f64 = *var_toxov_i_slot;
        let mut var_toxov_i_rv: f64 = *var_toxov_i_rv_slot;
        let mut var_toxovd_i: f64 = *var_toxovd_i_slot;
        let mut var_toxovd_i_rv: f64 = *var_toxovd_i_rv_slot;
        let mut var_vfb_i: f64 = *var_vfb_i_slot;
        let mut var_vfb_i_rv: f64 = *var_vfb_i_rv_slot;
        let mut var_vfb_p: f64 = *var_vfb_p_slot;
        let mut var_vfb_p_rv: f64 = *var_vfb_p_rv_slot;
        let mut var_vfbedge_p: f64 = *var_vfbedge_p_slot;
        let mut var_vfbedge_p_rv: f64 = *var_vfbedge_p_rv_slot;
        let mut var_vsbnud_i: f64 = *var_vsbnud_i_slot;
        let mut var_vsbnud_i_rv: f64 = *var_vsbnud_i_rv_slot;
        let mut var_xcor_i: f64 = *var_xcor_i_slot;
        let mut var_xcor_i_rv: f64 = *var_xcor_i_rv_slot;

        let (assign9530_e9640,) = {
    if ((var_guard36 != 0.0) && (var_guard148 != 0.0)) {
        let assign9530_e9633: f64 = (p.p829 * var_scb_i);
        let assign9530_e9634: f64 = (var_sca_i + assign9530_e9633);
        let assign9530_e9637: f64 = (p.p830 * var_scc_i);
        let assign9530_e9638: f64 = (assign9530_e9634 + assign9530_e9637);
        (assign9530_e9638,)
    } else {
        (var_temp0,)
    }
};
        var_temp0 = assign9530_e9640;
        var_temp0_rv = 0.0;

        let (assign9540_e9650,) = {
    if ((var_guard36 != 0.0) && (var_guard148 != 0.0)) {
        let assign9540_e9647: f64 = (var_kvthowe * var_temp0);
        let assign9540_e9648: f64 = (var_vfb_p + assign9540_e9647);
        (assign9540_e9648,)
    } else {
        (var_vfb_p,)
    }
};
        var_vfb_p = assign9540_e9650;
        var_vfb_p_rv = 0.0;

        let (assign9550_e9662,) = {
    if ((var_guard36 != 0.0) && (var_guard148 != 0.0)) {
        let assign9550_e9658: f64 = (var_kuowe * var_temp0);
        let assign9550_e9659: f64 = (1.0 + assign9550_e9658);
        let assign9550_e9660: f64 = (var_betn_p * assign9550_e9659);
        (assign9550_e9660,)
    } else {
        (var_betn_p,)
    }
};
        var_betn_p = assign9550_e9662;
        var_betn_p_rv = 0.0;

        let (assign9560_e9672,) = {
    if ((var_guard36 != 0.0) && (var_guard148 != 0.0)) {
        let assign9560_e9669: f64 = (var_kvthowe * var_temp0);
        let assign9560_e9670: f64 = (var_vfbedge_p + assign9560_e9669);
        (assign9560_e9670,)
    } else {
        (var_vfbedge_p,)
    }
};
        var_vfbedge_p = assign9560_e9672;
        var_vfbedge_p_rv = 0.0;

        let (assign9570_e9684,) = {
    if ((var_guard36 != 0.0) && (var_guard148 != 0.0)) {
        let assign9570_e9680: f64 = (var_kuowe * var_temp0);
        let assign9570_e9681: f64 = (1.0 + assign9570_e9680);
        let assign9570_e9682: f64 = (var_betnedge_p * assign9570_e9681);
        (assign9570_e9682,)
    } else {
        (var_betnedge_p,)
    }
};
        var_betnedge_p = assign9570_e9684;
        var_betnedge_p_rv = 0.0;

        var_vfb_i = var_vfb_p;
        var_vfb_i_rv = 0.0;

        var_stvfb_i = var_stvfb_p;
        var_stvfb_i_rv = 0.0;

        var_st2vfb_i = var_st2vfb_p;
        var_st2vfb_i_rv = 0.0;

        var_tox_i = var_tox_p;
        var_tox_i_rv = 0.0;

        var_epsrox_i = var_epsrox_p;
        var_epsrox_i_rv = 0.0;

        let (assign9630_e9700,) = {
    if (var_neff_p > 1e20) {
        let (assign9630_e9698,) = {
            if (var_neff_p < 1e26) {
                (var_neff_p,)
            } else {
                (1e26,)
            }
        };
        (assign9630_e9698,)
    } else {
        (1e20,)
    }
};
        var_neff_i = assign9630_e9700;
        var_neff_i_rv = 0.0;

        let (assign9640_e9706,) = {
    if (var_gfacnud_p > 0.01) {
        (var_gfacnud_p,)
    } else {
        (0.01,)
    }
};
        var_gfacnud_i = assign9640_e9706;
        var_gfacnud_i_rv = 0.0;

        let (assign9650_e9712,) = {
    if (var_vsbnud_p > 0.0) {
        (var_vsbnud_p,)
    } else {
        (0.0,)
    }
};
        var_vsbnud_i = assign9650_e9712;
        var_vsbnud_i_rv = 0.0;

        var_dvsbnud_i = var_dvsbnud_p;
        var_dvsbnud_i_rv = 0.0;

        var_dphib_i = var_dphib_p;
        var_dphib_i_rv = 0.0;

        let (assign9680_e9720,) = {
    if (var_np_p > 0.0) {
        (var_np_p,)
    } else {
        (0.0,)
    }
};
        var_np_i = assign9680_e9720;
        var_np_i_rv = 0.0;

        var_toxov_i = var_toxov_p;
        var_toxov_i_rv = 0.0;

        var_toxovd_i = var_toxovd_p;
        var_toxovd_i_rv = 0.0;

        let (assign9710_e9733,) = {
    if (var_nov_p > 1e23) {
        let (assign9710_e9731,) = {
            if (var_nov_p < 1e27) {
                (var_nov_p,)
            } else {
                (1e27,)
            }
        };
        (assign9710_e9731,)
    } else {
        (1e23,)
    }
};
        var_nov_i = assign9710_e9733;
        var_nov_i_rv = 0.0;

        let (assign9720_e9744,) = {
    if (var_novd_p > 1e23) {
        let (assign9720_e9742,) = {
            if (var_novd_p < 1e27) {
                (var_novd_p,)
            } else {
                (1e27,)
            }
        };
        (assign9720_e9742,)
    } else {
        (1e23,)
    }
};
        var_novd_i = assign9720_e9744;
        var_novd_i_rv = 0.0;

        let (assign9730_e9750,) = {
    if (var_ct_p > 0.0) {
        (var_ct_p,)
    } else {
        (0.0,)
    }
};
        var_ct_i = assign9730_e9750;
        var_ct_i_rv = 0.0;

        let (assign9740_e9761,) = {
    if (var_ctb_p > 0.0) {
        let (assign9740_e9759,) = {
            if (var_ctb_p < 0.5) {
                (var_ctb_p,)
            } else {
                (0.5,)
            }
        };
        (assign9740_e9759,)
    } else {
        (0.0,)
    }
};
        var_ctb_i = assign9740_e9761;
        var_ctb_i_rv = 0.0;

        let (assign9750_e9772,) = {
    if (var_ctg_p > 0.0) {
        let (assign9750_e9770,) = {
            if (var_ctg_p < 1.0) {
                (var_ctg_p,)
            } else {
                (1.0,)
            }
        };
        (assign9750_e9770,)
    } else {
        (0.0,)
    }
};
        var_ctg_i = assign9750_e9772;
        var_ctg_i_rv = 0.0;

        var_stct_i = var_stct_p;
        var_stct_i_rv = 0.0;

        let (assign9770_e9779,) = {
    if (var_cf_p > 0.0) {
        (var_cf_p,)
    } else {
        (0.0,)
    }
};
        var_cf_i = assign9770_e9779;
        var_cf_i_rv = 0.0;

        let (assign9780_e9790,) = {
    if (var_cfb_p > 0.0) {
        let (assign9780_e9788,) = {
            if (var_cfb_p < 1.0) {
                (var_cfb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9780_e9788,)
    } else {
        (0.0,)
    }
};
        var_cfb_i = assign9780_e9790;
        var_cfb_i_rv = 0.0;

        let (assign9790_e9796,) = {
    if (var_cfd_p > 0.0) {
        (var_cfd_p,)
    } else {
        (0.0,)
    }
};
        var_cfd_i = assign9790_e9796;
        var_cfd_i_rv = 0.0;

        let (assign9800_e9802,) = {
    if (var_psce_p > 0.0) {
        (var_psce_p,)
    } else {
        (0.0,)
    }
};
        var_psce_i = assign9800_e9802;
        var_psce_i_rv = 0.0;

        let (assign9810_e9813,) = {
    if (var_psceb_p > 0.0) {
        let (assign9810_e9811,) = {
            if (var_psceb_p < 1.0) {
                (var_psceb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9810_e9811,)
    } else {
        (0.0,)
    }
};
        var_psceb_i = assign9810_e9813;
        var_psceb_i_rv = 0.0;

        let (assign9820_e9819,) = {
    if (var_psced_p > 0.0) {
        (var_psced_p,)
    } else {
        (0.0,)
    }
};
        var_psced_i = assign9820_e9819;
        var_psced_i_rv = 0.0;

        let (assign9830_e9825,) = {
    if (var_betn_p > 0.0) {
        (var_betn_p,)
    } else {
        (0.0,)
    }
};
        var_betn_i = assign9830_e9825;
        var_betn_i_rv = 0.0;

        var_stbet_i = var_stbet_p;
        var_stbet_i_rv = 0.0;

        let (assign9850_e9832,) = {
    if (var_mue_p > 0.0) {
        (var_mue_p,)
    } else {
        (0.0,)
    }
};
        var_mue_i = assign9850_e9832;
        var_mue_i_rv = 0.0;

        var_stmue_i = var_stmue_p;
        var_stmue_i_rv = 0.0;

        let (assign9870_e9839,) = {
    if (var_themu_p > 0.0) {
        (var_themu_p,)
    } else {
        (0.0,)
    }
};
        var_themu_i = assign9870_e9839;
        var_themu_i_rv = 0.0;

        var_stthemu_i = var_stthemu_p;
        var_stthemu_i_rv = 0.0;

        let (assign9890_e9846,) = {
    if (var_cs_p > 0.0) {
        (var_cs_p,)
    } else {
        (0.0,)
    }
};
        var_cs_i = assign9890_e9846;
        var_cs_i_rv = 0.0;

        var_stcs_i = var_stcs_p;
        var_stcs_i_rv = 0.0;

        let (assign9910_e9853,) = {
    if (var_thecs_p > 0.0) {
        (var_thecs_p,)
    } else {
        (0.0,)
    }
};
        var_thecs_i = assign9910_e9853;
        var_thecs_i_rv = 0.0;

        var_stthecs_i = var_stthecs_p;
        var_stthecs_i_rv = 0.0;

        let (assign9930_e9860,) = {
    if (var_xcor_p > 0.0) {
        (var_xcor_p,)
    } else {
        (0.0,)
    }
};
        var_xcor_i = assign9930_e9860;
        var_xcor_i_rv = 0.0;

        var_stxcor_i = var_stxcor_p;
        var_stxcor_i_rv = 0.0;

        var_feta_i = var_feta_p;
        var_feta_i_rv = 0.0;

        let (assign9960_e9868,) = {
    if (var_rs_p > 0.0) {
        (var_rs_p,)
    } else {
        (0.0,)
    }
};
        var_rs_i = assign9960_e9868;
        var_rs_i_rv = 0.0;

        var_strs_i = var_strs_p;
        var_strs_i_rv = 0.0;

        let assign9980_e9872: f64 = (-0.5);
        let (assign9980_e9882,) = {
    if (var_rsb_p > assign9980_e9872) {
        let (assign9980_e9879,) = {
            if (var_rsb_p < 1.0) {
                (var_rsb_p,)
            } else {
                (1.0,)
            }
        };
        (assign9980_e9879,)
    } else {
        let assign9980_e9881: f64 = (-0.5);
        (assign9980_e9881,)
    }
};
        var_rsb_i = assign9980_e9882;
        var_rsb_i_rv = 0.0;

        let assign9990_e9885: f64 = (-0.5);
        let (assign9990_e9890,) = {
    if (var_rsg_p > assign9990_e9885) {
        (var_rsg_p,)
    } else {
        let assign9990_e9889: f64 = (-0.5);
        (assign9990_e9889,)
    }
};
        var_rsg_i = assign9990_e9890;
        var_rsg_i_rv = 0.0;

        let (assign10000_e9896,) = {
    if (var_thesat_p > 0.0) {
        (var_thesat_p,)
    } else {
        (0.0,)
    }
};
        var_thesat_i = assign10000_e9896;
        var_thesat_i_rv = 0.0;

        var_stthesat_i = var_stthesat_p;
        var_stthesat_i_rv = 0.0;

        let assign10020_e9900: f64 = (-0.5);
        let (assign10020_e9910,) = {
    if (var_thesatb_p > assign10020_e9900) {
        let (assign10020_e9907,) = {
            if (var_thesatb_p < 1.0) {
                (var_thesatb_p,)
            } else {
                (1.0,)
            }
        };
        (assign10020_e9907,)
    } else {
        let assign10020_e9909: f64 = (-0.5);
        (assign10020_e9909,)
    }
};
        var_thesatb_i = assign10020_e9910;
        var_thesatb_i_rv = 0.0;

        let assign10030_e9913: f64 = (-0.5);
        let (assign10030_e9918,) = {
    if (var_thesatg_p > assign10030_e9913) {
        (var_thesatg_p,)
    } else {
        let assign10030_e9917: f64 = (-0.5);
        (assign10030_e9917,)
    }
};
        var_thesatg_i = assign10030_e9918;
        var_thesatg_i_rv = 0.0;

        let (assign10040_e9924,) = {
    if (var_thesatt_p > 0.01) {
        (var_thesatt_p,)
    } else {
        (0.01,)
    }
};
        var_thesatt_i = assign10040_e9924;
        var_thesatt_i_rv = 0.0;

        let (assign10050_e9930,) = {
    if (var_ax_p > 2.0) {
        (var_ax_p,)
    } else {
        (2.0,)
    }
};
        var_ax_i = assign10050_e9930;
        var_ax_i_rv = 0.0;

        let (assign10060_e9936,) = {
    if (var_alp_p > 0.0) {
        (var_alp_p,)
    } else {
        (0.0,)
    }
};
        var_alp_i = assign10060_e9936;
        var_alp_i_rv = 0.0;

        let (assign10070_e9942,) = {
    if (var_alp1_p > 0.0) {
        (var_alp1_p,)
    } else {
        (0.0,)
    }
};
        var_alp1_i = assign10070_e9942;
        var_alp1_i_rv = 0.0;

        let (assign10080_e9948,) = {
    if (var_alp2_p > 0.0) {
        (var_alp2_p,)
    } else {
        (0.0,)
    }
};
        var_alp2_i = assign10080_e9948;
        var_alp2_i_rv = 0.0;

        *var_alp1_i_slot = var_alp1_i;
        *var_alp1_i_rv_slot = var_alp1_i_rv;
        *var_alp2_i_slot = var_alp2_i;
        *var_alp2_i_rv_slot = var_alp2_i_rv;
        *var_alp_i_slot = var_alp_i;
        *var_alp_i_rv_slot = var_alp_i_rv;
        *var_ax_i_slot = var_ax_i;
        *var_ax_i_rv_slot = var_ax_i_rv;
        *var_betn_i_slot = var_betn_i;
        *var_betn_i_rv_slot = var_betn_i_rv;
        *var_betn_p_slot = var_betn_p;
        *var_betn_p_rv_slot = var_betn_p_rv;
        *var_betnedge_p_slot = var_betnedge_p;
        *var_betnedge_p_rv_slot = var_betnedge_p_rv;
        *var_cf_i_slot = var_cf_i;
        *var_cf_i_rv_slot = var_cf_i_rv;
        *var_cfb_i_slot = var_cfb_i;
        *var_cfb_i_rv_slot = var_cfb_i_rv;
        *var_cfd_i_slot = var_cfd_i;
        *var_cfd_i_rv_slot = var_cfd_i_rv;
        *var_cs_i_slot = var_cs_i;
        *var_cs_i_rv_slot = var_cs_i_rv;
        *var_ct_i_slot = var_ct_i;
        *var_ct_i_rv_slot = var_ct_i_rv;
        *var_ctb_i_slot = var_ctb_i;
        *var_ctb_i_rv_slot = var_ctb_i_rv;
        *var_ctg_i_slot = var_ctg_i;
        *var_ctg_i_rv_slot = var_ctg_i_rv;
        *var_dphib_i_slot = var_dphib_i;
        *var_dphib_i_rv_slot = var_dphib_i_rv;
        *var_dvsbnud_i_slot = var_dvsbnud_i;
        *var_dvsbnud_i_rv_slot = var_dvsbnud_i_rv;
        *var_epsrox_i_slot = var_epsrox_i;
        *var_epsrox_i_rv_slot = var_epsrox_i_rv;
        *var_feta_i_slot = var_feta_i;
        *var_feta_i_rv_slot = var_feta_i_rv;
        *var_gfacnud_i_slot = var_gfacnud_i;
        *var_gfacnud_i_rv_slot = var_gfacnud_i_rv;
        *var_mue_i_slot = var_mue_i;
        *var_mue_i_rv_slot = var_mue_i_rv;
        *var_neff_i_slot = var_neff_i;
        *var_neff_i_rv_slot = var_neff_i_rv;
        *var_nov_i_slot = var_nov_i;
        *var_nov_i_rv_slot = var_nov_i_rv;
        *var_novd_i_slot = var_novd_i;
        *var_novd_i_rv_slot = var_novd_i_rv;
        *var_np_i_slot = var_np_i;
        *var_np_i_rv_slot = var_np_i_rv;
        *var_psce_i_slot = var_psce_i;
        *var_psce_i_rv_slot = var_psce_i_rv;
        *var_psceb_i_slot = var_psceb_i;
        *var_psceb_i_rv_slot = var_psceb_i_rv;
        *var_psced_i_slot = var_psced_i;
        *var_psced_i_rv_slot = var_psced_i_rv;
        *var_rs_i_slot = var_rs_i;
        *var_rs_i_rv_slot = var_rs_i_rv;
        *var_rsb_i_slot = var_rsb_i;
        *var_rsb_i_rv_slot = var_rsb_i_rv;
        *var_rsg_i_slot = var_rsg_i;
        *var_rsg_i_rv_slot = var_rsg_i_rv;
        *var_st2vfb_i_slot = var_st2vfb_i;
        *var_st2vfb_i_rv_slot = var_st2vfb_i_rv;
        *var_stbet_i_slot = var_stbet_i;
        *var_stbet_i_rv_slot = var_stbet_i_rv;
        *var_stcs_i_slot = var_stcs_i;
        *var_stcs_i_rv_slot = var_stcs_i_rv;
        *var_stct_i_slot = var_stct_i;
        *var_stct_i_rv_slot = var_stct_i_rv;
        *var_stmue_i_slot = var_stmue_i;
        *var_stmue_i_rv_slot = var_stmue_i_rv;
        *var_strs_i_slot = var_strs_i;
        *var_strs_i_rv_slot = var_strs_i_rv;
        *var_stthecs_i_slot = var_stthecs_i;
        *var_stthecs_i_rv_slot = var_stthecs_i_rv;
        *var_stthemu_i_slot = var_stthemu_i;
        *var_stthemu_i_rv_slot = var_stthemu_i_rv;
        *var_stthesat_i_slot = var_stthesat_i;
        *var_stthesat_i_rv_slot = var_stthesat_i_rv;
        *var_stvfb_i_slot = var_stvfb_i;
        *var_stvfb_i_rv_slot = var_stvfb_i_rv;
        *var_stxcor_i_slot = var_stxcor_i;
        *var_stxcor_i_rv_slot = var_stxcor_i_rv;
        *var_temp0_slot = var_temp0;
        *var_temp0_rv_slot = var_temp0_rv;
        *var_thecs_i_slot = var_thecs_i;
        *var_thecs_i_rv_slot = var_thecs_i_rv;
        *var_themu_i_slot = var_themu_i;
        *var_themu_i_rv_slot = var_themu_i_rv;
        *var_thesat_i_slot = var_thesat_i;
        *var_thesat_i_rv_slot = var_thesat_i_rv;
        *var_thesatb_i_slot = var_thesatb_i;
        *var_thesatb_i_rv_slot = var_thesatb_i_rv;
        *var_thesatg_i_slot = var_thesatg_i;
        *var_thesatg_i_rv_slot = var_thesatg_i_rv;
        *var_thesatt_i_slot = var_thesatt_i;
        *var_thesatt_i_rv_slot = var_thesatt_i_rv;
        *var_tox_i_slot = var_tox_i;
        *var_tox_i_rv_slot = var_tox_i_rv;
        *var_toxov_i_slot = var_toxov_i;
        *var_toxov_i_rv_slot = var_toxov_i_rv;
        *var_toxovd_i_slot = var_toxovd_i;
        *var_toxovd_i_rv_slot = var_toxovd_i_rv;
        *var_vfb_i_slot = var_vfb_i;
        *var_vfb_i_rv_slot = var_vfb_i_rv;
        *var_vfb_p_slot = var_vfb_p;
        *var_vfb_p_rv_slot = var_vfb_p_rv;
        *var_vfbedge_p_slot = var_vfbedge_p;
        *var_vfbedge_p_rv_slot = var_vfbedge_p_rv;
        *var_vsbnud_i_slot = var_vsbnud_i;
        *var_vsbnud_i_rv_slot = var_vsbnud_i_rv;
        *var_xcor_i_slot = var_xcor_i;
        *var_xcor_i_rv_slot = var_xcor_i_rv;
    }

    pub(super) fn stamp_reactive_block_16(
        p: &Parameters,
        var_a1_p: f64,
        var_a2_p: f64,
        var_a3_p: f64,
        var_a4_p: f64,
        var_agidl_p: f64,
        var_agidld_p: f64,
        var_alp1ac_p: f64,
        var_alpac_p: f64,
        var_axac_p: f64,
        var_axinr_p: f64,
        var_betnedge_p: f64,
        var_bgidl_p: f64,
        var_bgidld_p: f64,
        var_cfbedge_p: f64,
        var_cfdedge_p: f64,
        var_cfedge_p: f64,
        var_cfr_p: f64,
        var_cfrd_p: f64,
        var_cgbov_p: f64,
        var_cgidl_p: f64,
        var_cgidld_p: f64,
        var_cgov_p: f64,
        var_cgovaccg_p: f64,
        var_cgovd_p: f64,
        var_chib_p: f64,
        var_cinr_p: f64,
        var_cinrd_p: f64,
        var_cox_p: f64,
        var_ctedge_p: f64,
        var_cth_p: f64,
        var_delvtac_p: f64,
        var_dphibedge_p: f64,
        var_dvfbinr_p: f64,
        var_facneffac_p: f64,
        var_fcgovacc_p: f64,
        var_fcgovaccd_p: f64,
        var_fcinracc_p: f64,
        var_fcinrdep_p: f64,
        var_fnt_p: f64,
        var_gc2_p: f64,
        var_gc2ov_p: f64,
        var_gc2ovd_p: f64,
        var_gc3_p: f64,
        var_gc3ov_p: f64,
        var_gc3ovd_p: f64,
        var_gco_p: f64,
        var_iginv_p: f64,
        var_igov_p: f64,
        var_igovd_p: f64,
        var_imaxii_p: f64,
        var_neffedge_p: f64,
        var_nf_i: f64,
        var_nov_i: f64,
        var_pscebedge_p: f64,
        var_pscededge_p: f64,
        var_psceedge_p: f64,
        var_sta2_p: f64,
        var_stbetedge_p: f64,
        var_stbgidl_p: f64,
        var_stbgidld_p: f64,
        var_stig_p: f64,
        var_stvfbedge_p: f64,
        var_thesatac_p: f64,
        var_toxov_i: f64,
        var_vfbedge_p: f64,
        var_vp_p: f64,
        var_a1_i_slot: &mut f64,
        var_a1_i_rv_slot: &mut f64,
        var_a2_i_slot: &mut f64,
        var_a2_i_rv_slot: &mut f64,
        var_a3_i_slot: &mut f64,
        var_a3_i_rv_slot: &mut f64,
        var_a4_i_slot: &mut f64,
        var_a4_i_rv_slot: &mut f64,
        var_agidl_i_slot: &mut f64,
        var_agidl_i_rv_slot: &mut f64,
        var_agidld_i_slot: &mut f64,
        var_agidld_i_rv_slot: &mut f64,
        var_alp1ac_i_slot: &mut f64,
        var_alp1ac_i_rv_slot: &mut f64,
        var_alpac_i_slot: &mut f64,
        var_alpac_i_rv_slot: &mut f64,
        var_axac_i_slot: &mut f64,
        var_axac_i_rv_slot: &mut f64,
        var_axinr_i_slot: &mut f64,
        var_axinr_i_rv_slot: &mut f64,
        var_betnedge_i_slot: &mut f64,
        var_betnedge_i_rv_slot: &mut f64,
        var_bgidl_i_slot: &mut f64,
        var_bgidl_i_rv_slot: &mut f64,
        var_bgidld_i_slot: &mut f64,
        var_bgidld_i_rv_slot: &mut f64,
        var_cfbedge_i_slot: &mut f64,
        var_cfbedge_i_rv_slot: &mut f64,
        var_cfdedge_i_slot: &mut f64,
        var_cfdedge_i_rv_slot: &mut f64,
        var_cfedge_i_slot: &mut f64,
        var_cfedge_i_rv_slot: &mut f64,
        var_cfr_i_slot: &mut f64,
        var_cfr_i_rv_slot: &mut f64,
        var_cfrd_i_slot: &mut f64,
        var_cfrd_i_rv_slot: &mut f64,
        var_cgbov_i_slot: &mut f64,
        var_cgbov_i_rv_slot: &mut f64,
        var_cgidl_i_slot: &mut f64,
        var_cgidl_i_rv_slot: &mut f64,
        var_cgidld_i_slot: &mut f64,
        var_cgidld_i_rv_slot: &mut f64,
        var_cgov_i_slot: &mut f64,
        var_cgov_i_rv_slot: &mut f64,
        var_cgovaccg_i_slot: &mut f64,
        var_cgovaccg_i_rv_slot: &mut f64,
        var_cgovd_i_slot: &mut f64,
        var_cgovd_i_rv_slot: &mut f64,
        var_chib_i_slot: &mut f64,
        var_chib_i_rv_slot: &mut f64,
        var_cinr_i_slot: &mut f64,
        var_cinr_i_rv_slot: &mut f64,
        var_cinrd_i_slot: &mut f64,
        var_cinrd_i_rv_slot: &mut f64,
        var_cox_i_slot: &mut f64,
        var_cox_i_rv_slot: &mut f64,
        var_ctedge_i_slot: &mut f64,
        var_ctedge_i_rv_slot: &mut f64,
        var_cth_i_slot: &mut f64,
        var_cth_i_rv_slot: &mut f64,
        var_delvtac_i_slot: &mut f64,
        var_delvtac_i_rv_slot: &mut f64,
        var_delvto_i_slot: &mut f64,
        var_delvto_i_rv_slot: &mut f64,
        var_delvtoedge_i_slot: &mut f64,
        var_delvtoedge_i_rv_slot: &mut f64,
        var_dphibedge_i_slot: &mut f64,
        var_dphibedge_i_rv_slot: &mut f64,
        var_dvfbinr_i_slot: &mut f64,
        var_dvfbinr_i_rv_slot: &mut f64,
        var_facneffac_i_slot: &mut f64,
        var_facneffac_i_rv_slot: &mut f64,
        var_factuo_i_slot: &mut f64,
        var_factuo_i_rv_slot: &mut f64,
        var_factuoedge_i_slot: &mut f64,
        var_factuoedge_i_rv_slot: &mut f64,
        var_fcgovacc_i_slot: &mut f64,
        var_fcgovacc_i_rv_slot: &mut f64,
        var_fcgovaccd_i_slot: &mut f64,
        var_fcgovaccd_i_rv_slot: &mut f64,
        var_fcinracc_i_slot: &mut f64,
        var_fcinracc_i_rv_slot: &mut f64,
        var_fcinrdep_i_slot: &mut f64,
        var_fcinrdep_i_rv_slot: &mut f64,
        var_fnt_i_slot: &mut f64,
        var_fnt_i_rv_slot: &mut f64,
        var_gc2_i_slot: &mut f64,
        var_gc2_i_rv_slot: &mut f64,
        var_gc2ov_i_slot: &mut f64,
        var_gc2ov_i_rv_slot: &mut f64,
        var_gc2ovd_i_slot: &mut f64,
        var_gc2ovd_i_rv_slot: &mut f64,
        var_gc3_i_slot: &mut f64,
        var_gc3_i_rv_slot: &mut f64,
        var_gc3ov_i_slot: &mut f64,
        var_gc3ov_i_rv_slot: &mut f64,
        var_gc3ovd_i_slot: &mut f64,
        var_gc3ovd_i_rv_slot: &mut f64,
        var_gco_i_slot: &mut f64,
        var_gco_i_rv_slot: &mut f64,
        var_guard150_slot: &mut f64,
        var_guard150_rv_slot: &mut f64,
        var_iginv_i_slot: &mut f64,
        var_iginv_i_rv_slot: &mut f64,
        var_igov_i_slot: &mut f64,
        var_igov_i_rv_slot: &mut f64,
        var_igovd_i_slot: &mut f64,
        var_igovd_i_rv_slot: &mut f64,
        var_imaxii_i_slot: &mut f64,
        var_imaxii_i_rv_slot: &mut f64,
        var_mult_inst_slot: &mut f64,
        var_mult_inst_rv_slot: &mut f64,
        var_neffedge_i_slot: &mut f64,
        var_neffedge_i_rv_slot: &mut f64,
        var_novd_i_slot: &mut f64,
        var_novd_i_rv_slot: &mut f64,
        var_pscebedge_i_slot: &mut f64,
        var_pscebedge_i_rv_slot: &mut f64,
        var_pscededge_i_slot: &mut f64,
        var_pscededge_i_rv_slot: &mut f64,
        var_psceedge_i_slot: &mut f64,
        var_psceedge_i_rv_slot: &mut f64,
        var_sta2_i_slot: &mut f64,
        var_sta2_i_rv_slot: &mut f64,
        var_stbetedge_i_slot: &mut f64,
        var_stbetedge_i_rv_slot: &mut f64,
        var_stbgidl_i_slot: &mut f64,
        var_stbgidl_i_rv_slot: &mut f64,
        var_stbgidld_i_slot: &mut f64,
        var_stbgidld_i_rv_slot: &mut f64,
        var_stig_i_slot: &mut f64,
        var_stig_i_rv_slot: &mut f64,
        var_stvfbedge_i_slot: &mut f64,
        var_stvfbedge_i_rv_slot: &mut f64,
        var_thesatac_i_slot: &mut f64,
        var_thesatac_i_rv_slot: &mut f64,
        var_toxovd_i_slot: &mut f64,
        var_toxovd_i_rv_slot: &mut f64,
        var_vfbedge_i_slot: &mut f64,
        var_vfbedge_i_rv_slot: &mut f64,
        var_vp_i_slot: &mut f64,
        var_vp_i_rv_slot: &mut f64,
    ) {
        let mut var_a1_i: f64 = *var_a1_i_slot;
        let mut var_a1_i_rv: f64 = *var_a1_i_rv_slot;
        let mut var_a2_i: f64 = *var_a2_i_slot;
        let mut var_a2_i_rv: f64 = *var_a2_i_rv_slot;
        let mut var_a3_i: f64 = *var_a3_i_slot;
        let mut var_a3_i_rv: f64 = *var_a3_i_rv_slot;
        let mut var_a4_i: f64 = *var_a4_i_slot;
        let mut var_a4_i_rv: f64 = *var_a4_i_rv_slot;
        let mut var_agidl_i: f64 = *var_agidl_i_slot;
        let mut var_agidl_i_rv: f64 = *var_agidl_i_rv_slot;
        let mut var_agidld_i: f64 = *var_agidld_i_slot;
        let mut var_agidld_i_rv: f64 = *var_agidld_i_rv_slot;
        let mut var_alp1ac_i: f64 = *var_alp1ac_i_slot;
        let mut var_alp1ac_i_rv: f64 = *var_alp1ac_i_rv_slot;
        let mut var_alpac_i: f64 = *var_alpac_i_slot;
        let mut var_alpac_i_rv: f64 = *var_alpac_i_rv_slot;
        let mut var_axac_i: f64 = *var_axac_i_slot;
        let mut var_axac_i_rv: f64 = *var_axac_i_rv_slot;
        let mut var_axinr_i: f64 = *var_axinr_i_slot;
        let mut var_axinr_i_rv: f64 = *var_axinr_i_rv_slot;
        let mut var_betnedge_i: f64 = *var_betnedge_i_slot;
        let mut var_betnedge_i_rv: f64 = *var_betnedge_i_rv_slot;
        let mut var_bgidl_i: f64 = *var_bgidl_i_slot;
        let mut var_bgidl_i_rv: f64 = *var_bgidl_i_rv_slot;
        let mut var_bgidld_i: f64 = *var_bgidld_i_slot;
        let mut var_bgidld_i_rv: f64 = *var_bgidld_i_rv_slot;
        let mut var_cfbedge_i: f64 = *var_cfbedge_i_slot;
        let mut var_cfbedge_i_rv: f64 = *var_cfbedge_i_rv_slot;
        let mut var_cfdedge_i: f64 = *var_cfdedge_i_slot;
        let mut var_cfdedge_i_rv: f64 = *var_cfdedge_i_rv_slot;
        let mut var_cfedge_i: f64 = *var_cfedge_i_slot;
        let mut var_cfedge_i_rv: f64 = *var_cfedge_i_rv_slot;
        let mut var_cfr_i: f64 = *var_cfr_i_slot;
        let mut var_cfr_i_rv: f64 = *var_cfr_i_rv_slot;
        let mut var_cfrd_i: f64 = *var_cfrd_i_slot;
        let mut var_cfrd_i_rv: f64 = *var_cfrd_i_rv_slot;
        let mut var_cgbov_i: f64 = *var_cgbov_i_slot;
        let mut var_cgbov_i_rv: f64 = *var_cgbov_i_rv_slot;
        let mut var_cgidl_i: f64 = *var_cgidl_i_slot;
        let mut var_cgidl_i_rv: f64 = *var_cgidl_i_rv_slot;
        let mut var_cgidld_i: f64 = *var_cgidld_i_slot;
        let mut var_cgidld_i_rv: f64 = *var_cgidld_i_rv_slot;
        let mut var_cgov_i: f64 = *var_cgov_i_slot;
        let mut var_cgov_i_rv: f64 = *var_cgov_i_rv_slot;
        let mut var_cgovaccg_i: f64 = *var_cgovaccg_i_slot;
        let mut var_cgovaccg_i_rv: f64 = *var_cgovaccg_i_rv_slot;
        let mut var_cgovd_i: f64 = *var_cgovd_i_slot;
        let mut var_cgovd_i_rv: f64 = *var_cgovd_i_rv_slot;
        let mut var_chib_i: f64 = *var_chib_i_slot;
        let mut var_chib_i_rv: f64 = *var_chib_i_rv_slot;
        let mut var_cinr_i: f64 = *var_cinr_i_slot;
        let mut var_cinr_i_rv: f64 = *var_cinr_i_rv_slot;
        let mut var_cinrd_i: f64 = *var_cinrd_i_slot;
        let mut var_cinrd_i_rv: f64 = *var_cinrd_i_rv_slot;
        let mut var_cox_i: f64 = *var_cox_i_slot;
        let mut var_cox_i_rv: f64 = *var_cox_i_rv_slot;
        let mut var_ctedge_i: f64 = *var_ctedge_i_slot;
        let mut var_ctedge_i_rv: f64 = *var_ctedge_i_rv_slot;
        let mut var_cth_i: f64 = *var_cth_i_slot;
        let mut var_cth_i_rv: f64 = *var_cth_i_rv_slot;
        let mut var_delvtac_i: f64 = *var_delvtac_i_slot;
        let mut var_delvtac_i_rv: f64 = *var_delvtac_i_rv_slot;
        let mut var_delvto_i: f64 = *var_delvto_i_slot;
        let mut var_delvto_i_rv: f64 = *var_delvto_i_rv_slot;
        let mut var_delvtoedge_i: f64 = *var_delvtoedge_i_slot;
        let mut var_delvtoedge_i_rv: f64 = *var_delvtoedge_i_rv_slot;
        let mut var_dphibedge_i: f64 = *var_dphibedge_i_slot;
        let mut var_dphibedge_i_rv: f64 = *var_dphibedge_i_rv_slot;
        let mut var_dvfbinr_i: f64 = *var_dvfbinr_i_slot;
        let mut var_dvfbinr_i_rv: f64 = *var_dvfbinr_i_rv_slot;
        let mut var_facneffac_i: f64 = *var_facneffac_i_slot;
        let mut var_facneffac_i_rv: f64 = *var_facneffac_i_rv_slot;
        let mut var_factuo_i: f64 = *var_factuo_i_slot;
        let mut var_factuo_i_rv: f64 = *var_factuo_i_rv_slot;
        let mut var_factuoedge_i: f64 = *var_factuoedge_i_slot;
        let mut var_factuoedge_i_rv: f64 = *var_factuoedge_i_rv_slot;
        let mut var_fcgovacc_i: f64 = *var_fcgovacc_i_slot;
        let mut var_fcgovacc_i_rv: f64 = *var_fcgovacc_i_rv_slot;
        let mut var_fcgovaccd_i: f64 = *var_fcgovaccd_i_slot;
        let mut var_fcgovaccd_i_rv: f64 = *var_fcgovaccd_i_rv_slot;
        let mut var_fcinracc_i: f64 = *var_fcinracc_i_slot;
        let mut var_fcinracc_i_rv: f64 = *var_fcinracc_i_rv_slot;
        let mut var_fcinrdep_i: f64 = *var_fcinrdep_i_slot;
        let mut var_fcinrdep_i_rv: f64 = *var_fcinrdep_i_rv_slot;
        let mut var_fnt_i: f64 = *var_fnt_i_slot;
        let mut var_fnt_i_rv: f64 = *var_fnt_i_rv_slot;
        let mut var_gc2_i: f64 = *var_gc2_i_slot;
        let mut var_gc2_i_rv: f64 = *var_gc2_i_rv_slot;
        let mut var_gc2ov_i: f64 = *var_gc2ov_i_slot;
        let mut var_gc2ov_i_rv: f64 = *var_gc2ov_i_rv_slot;
        let mut var_gc2ovd_i: f64 = *var_gc2ovd_i_slot;
        let mut var_gc2ovd_i_rv: f64 = *var_gc2ovd_i_rv_slot;
        let mut var_gc3_i: f64 = *var_gc3_i_slot;
        let mut var_gc3_i_rv: f64 = *var_gc3_i_rv_slot;
        let mut var_gc3ov_i: f64 = *var_gc3ov_i_slot;
        let mut var_gc3ov_i_rv: f64 = *var_gc3ov_i_rv_slot;
        let mut var_gc3ovd_i: f64 = *var_gc3ovd_i_slot;
        let mut var_gc3ovd_i_rv: f64 = *var_gc3ovd_i_rv_slot;
        let mut var_gco_i: f64 = *var_gco_i_slot;
        let mut var_gco_i_rv: f64 = *var_gco_i_rv_slot;
        let mut var_guard150: f64 = *var_guard150_slot;
        let mut var_guard150_rv: f64 = *var_guard150_rv_slot;
        let mut var_iginv_i: f64 = *var_iginv_i_slot;
        let mut var_iginv_i_rv: f64 = *var_iginv_i_rv_slot;
        let mut var_igov_i: f64 = *var_igov_i_slot;
        let mut var_igov_i_rv: f64 = *var_igov_i_rv_slot;
        let mut var_igovd_i: f64 = *var_igovd_i_slot;
        let mut var_igovd_i_rv: f64 = *var_igovd_i_rv_slot;
        let mut var_imaxii_i: f64 = *var_imaxii_i_slot;
        let mut var_imaxii_i_rv: f64 = *var_imaxii_i_rv_slot;
        let mut var_mult_inst: f64 = *var_mult_inst_slot;
        let mut var_mult_inst_rv: f64 = *var_mult_inst_rv_slot;
        let mut var_neffedge_i: f64 = *var_neffedge_i_slot;
        let mut var_neffedge_i_rv: f64 = *var_neffedge_i_rv_slot;
        let mut var_novd_i: f64 = *var_novd_i_slot;
        let mut var_novd_i_rv: f64 = *var_novd_i_rv_slot;
        let mut var_pscebedge_i: f64 = *var_pscebedge_i_slot;
        let mut var_pscebedge_i_rv: f64 = *var_pscebedge_i_rv_slot;
        let mut var_pscededge_i: f64 = *var_pscededge_i_slot;
        let mut var_pscededge_i_rv: f64 = *var_pscededge_i_rv_slot;
        let mut var_psceedge_i: f64 = *var_psceedge_i_slot;
        let mut var_psceedge_i_rv: f64 = *var_psceedge_i_rv_slot;
        let mut var_sta2_i: f64 = *var_sta2_i_slot;
        let mut var_sta2_i_rv: f64 = *var_sta2_i_rv_slot;
        let mut var_stbetedge_i: f64 = *var_stbetedge_i_slot;
        let mut var_stbetedge_i_rv: f64 = *var_stbetedge_i_rv_slot;
        let mut var_stbgidl_i: f64 = *var_stbgidl_i_slot;
        let mut var_stbgidl_i_rv: f64 = *var_stbgidl_i_rv_slot;
        let mut var_stbgidld_i: f64 = *var_stbgidld_i_slot;
        let mut var_stbgidld_i_rv: f64 = *var_stbgidld_i_rv_slot;
        let mut var_stig_i: f64 = *var_stig_i_slot;
        let mut var_stig_i_rv: f64 = *var_stig_i_rv_slot;
        let mut var_stvfbedge_i: f64 = *var_stvfbedge_i_slot;
        let mut var_stvfbedge_i_rv: f64 = *var_stvfbedge_i_rv_slot;
        let mut var_thesatac_i: f64 = *var_thesatac_i_slot;
        let mut var_thesatac_i_rv: f64 = *var_thesatac_i_rv_slot;
        let mut var_toxovd_i: f64 = *var_toxovd_i_slot;
        let mut var_toxovd_i_rv: f64 = *var_toxovd_i_rv_slot;
        let mut var_vfbedge_i: f64 = *var_vfbedge_i_slot;
        let mut var_vfbedge_i_rv: f64 = *var_vfbedge_i_rv_slot;
        let mut var_vp_i: f64 = *var_vp_i_slot;
        let mut var_vp_i_rv: f64 = *var_vp_i_rv_slot;

        var_vp_i = var_vp_p;
        var_vp_i_rv = 0.0;

        let (assign10100_e9955,) = {
    if (var_a1_p > 0.0) {
        (var_a1_p,)
    } else {
        (0.0,)
    }
};
        var_a1_i = assign10100_e9955;
        var_a1_i_rv = 0.0;

        var_a2_i = var_a2_p;
        var_a2_i_rv = 0.0;

        var_sta2_i = var_sta2_p;
        var_sta2_i_rv = 0.0;

        let (assign10130_e9963,) = {
    if (var_a3_p > 0.0) {
        (var_a3_p,)
    } else {
        (0.0,)
    }
};
        var_a3_i = assign10130_e9963;
        var_a3_i_rv = 0.0;

        let (assign10140_e9969,) = {
    if (var_a4_p > 0.0) {
        (var_a4_p,)
    } else {
        (0.0,)
    }
};
        var_a4_i = assign10140_e9969;
        var_a4_i_rv = 0.0;

        let (assign10150_e9975,) = {
    if (var_imaxii_p > 1e-12) {
        (var_imaxii_p,)
    } else {
        (1e-12,)
    }
};
        var_imaxii_i = assign10150_e9975;
        var_imaxii_i_rv = 0.0;

        var_gco_i = var_gco_p;
        var_gco_i_rv = 0.0;

        let (assign10170_e9982,) = {
    if (var_iginv_p > 0.0) {
        (var_iginv_p,)
    } else {
        (0.0,)
    }
};
        var_iginv_i = assign10170_e9982;
        var_iginv_i_rv = 0.0;

        let (assign10180_e9988,) = {
    if (var_igov_p > 0.0) {
        (var_igov_p,)
    } else {
        (0.0,)
    }
};
        var_igov_i = assign10180_e9988;
        var_igov_i_rv = 0.0;

        let (assign10190_e9994,) = {
    if (var_igovd_p > 0.0) {
        (var_igovd_p,)
    } else {
        (0.0,)
    }
};
        var_igovd_i = assign10190_e9994;
        var_igovd_i_rv = 0.0;

        var_stig_i = var_stig_p;
        var_stig_i_rv = 0.0;

        var_gc2_i = var_gc2_p;
        var_gc2_i_rv = 0.0;

        var_gc3_i = var_gc3_p;
        var_gc3_i_rv = 0.0;

        var_gc2ov_i = var_gc2ov_p;
        var_gc2ov_i_rv = 0.0;

        var_gc3ov_i = var_gc3ov_p;
        var_gc3ov_i_rv = 0.0;

        var_gc2ovd_i = var_gc2ovd_p;
        var_gc2ovd_i_rv = 0.0;

        var_gc3ovd_i = var_gc3ovd_p;
        var_gc3ovd_i_rv = 0.0;

        var_chib_i = var_chib_p;
        var_chib_i_rv = 0.0;

        let (assign10280_e10008,) = {
    if (var_agidl_p > 0.0) {
        (var_agidl_p,)
    } else {
        (0.0,)
    }
};
        var_agidl_i = assign10280_e10008;
        var_agidl_i_rv = 0.0;

        let (assign10290_e10014,) = {
    if (var_agidld_p > 0.0) {
        (var_agidld_p,)
    } else {
        (0.0,)
    }
};
        var_agidld_i = assign10290_e10014;
        var_agidld_i_rv = 0.0;

        var_bgidl_i = var_bgidl_p;
        var_bgidl_i_rv = 0.0;

        var_bgidld_i = var_bgidld_p;
        var_bgidld_i_rv = 0.0;

        var_stbgidl_i = var_stbgidl_p;
        var_stbgidl_i_rv = 0.0;

        var_stbgidld_i = var_stbgidld_p;
        var_stbgidld_i_rv = 0.0;

        var_cgidl_i = var_cgidl_p;
        var_cgidl_i_rv = 0.0;

        var_cgidld_i = var_cgidld_p;
        var_cgidld_i_rv = 0.0;

        let (assign10360_e10026,) = {
    if (var_cox_p > 0.0) {
        (var_cox_p,)
    } else {
        (0.0,)
    }
};
        var_cox_i = assign10360_e10026;
        var_cox_i_rv = 0.0;

        var_delvtac_i = var_delvtac_p;
        var_delvtac_i_rv = 0.0;

        let (assign10380_e10033,) = {
    if (var_facneffac_p > 0.0) {
        (var_facneffac_p,)
    } else {
        (0.0,)
    }
};
        var_facneffac_i = assign10380_e10033;
        var_facneffac_i_rv = 0.0;

        let (assign10390_e10039,) = {
    if (var_thesatac_p > 0.0) {
        (var_thesatac_p,)
    } else {
        (0.0,)
    }
};
        var_thesatac_i = assign10390_e10039;
        var_thesatac_i_rv = 0.0;

        let (assign10400_e10045,) = {
    if (var_axac_p > 2.0) {
        (var_axac_p,)
    } else {
        (2.0,)
    }
};
        var_axac_i = assign10400_e10045;
        var_axac_i_rv = 0.0;

        var_alpac_i = var_alpac_p;
        var_alpac_i_rv = 0.0;

        let (assign10420_e10052,) = {
    if (var_alp1ac_p > 0.0) {
        (var_alp1ac_p,)
    } else {
        (0.0,)
    }
};
        var_alp1ac_i = assign10420_e10052;
        var_alp1ac_i_rv = 0.0;

        let (assign10430_e10058,) = {
    if (var_cgov_p > 0.0) {
        (var_cgov_p,)
    } else {
        (0.0,)
    }
};
        var_cgov_i = assign10430_e10058;
        var_cgov_i_rv = 0.0;

        let (assign10440_e10064,) = {
    if (var_cgovd_p > 0.0) {
        (var_cgovd_p,)
    } else {
        (0.0,)
    }
};
        var_cgovd_i = assign10440_e10064;
        var_cgovd_i_rv = 0.0;

        var_fcgovacc_i = var_fcgovacc_p;
        var_fcgovacc_i_rv = 0.0;

        var_fcgovaccd_i = var_fcgovaccd_p;
        var_fcgovaccd_i_rv = 0.0;

        var_cgovaccg_i = var_cgovaccg_p;
        var_cgovaccg_i_rv = 0.0;

        let (assign10480_e10073,) = {
    if (var_cgbov_p > 0.0) {
        (var_cgbov_p,)
    } else {
        (0.0,)
    }
};
        var_cgbov_i = assign10480_e10073;
        var_cgbov_i_rv = 0.0;

        let (assign10490_e10079,) = {
    if (var_cinr_p > 0.0) {
        (var_cinr_p,)
    } else {
        (0.0,)
    }
};
        var_cinr_i = assign10490_e10079;
        var_cinr_i_rv = 0.0;

        let (assign10500_e10085,) = {
    if (var_cinrd_p > 0.0) {
        (var_cinrd_p,)
    } else {
        (0.0,)
    }
};
        var_cinrd_i = assign10500_e10085;
        var_cinrd_i_rv = 0.0;

        var_dvfbinr_i = var_dvfbinr_p;
        var_dvfbinr_i_rv = 0.0;

        var_fcinrdep_i = var_fcinrdep_p;
        var_fcinrdep_i_rv = 0.0;

        var_fcinracc_i = var_fcinracc_p;
        var_fcinracc_i_rv = 0.0;

        var_axinr_i = var_axinr_p;
        var_axinr_i_rv = 0.0;

        let (assign10550_e10095,) = {
    if (var_cfr_p > 0.0) {
        (var_cfr_p,)
    } else {
        (0.0,)
    }
};
        var_cfr_i = assign10550_e10095;
        var_cfr_i_rv = 0.0;

        let (assign10560_e10101,) = {
    if (var_cfrd_p > 0.0) {
        (var_cfrd_p,)
    } else {
        (0.0,)
    }
};
        var_cfrd_i = assign10560_e10101;
        var_cfrd_i_rv = 0.0;

        var_fnt_i = var_fnt_p;
        var_fnt_i_rv = 0.0;

        var_vfbedge_i = var_vfbedge_p;
        var_vfbedge_i_rv = 0.0;

        var_stvfbedge_i = var_stvfbedge_p;
        var_stvfbedge_i_rv = 0.0;

        var_dphibedge_i = var_dphibedge_p;
        var_dphibedge_i_rv = 0.0;

        let (assign10660_e10141,) = {
    if (var_neffedge_p > 1e20) {
        let (assign10660_e10139,) = {
            if (var_neffedge_p < 1e26) {
                (var_neffedge_p,)
            } else {
                (1e26,)
            }
        };
        (assign10660_e10139,)
    } else {
        (1e20,)
    }
};
        var_neffedge_i = assign10660_e10141;
        var_neffedge_i_rv = 0.0;

        let (assign10670_e10147,) = {
    if (var_ctedge_p > 0.0) {
        (var_ctedge_p,)
    } else {
        (0.0,)
    }
};
        var_ctedge_i = assign10670_e10147;
        var_ctedge_i_rv = 0.0;

        let (assign10680_e10153,) = {
    if (var_betnedge_p > 0.0) {
        (var_betnedge_p,)
    } else {
        (0.0,)
    }
};
        var_betnedge_i = assign10680_e10153;
        var_betnedge_i_rv = 0.0;

        var_stbetedge_i = var_stbetedge_p;
        var_stbetedge_i_rv = 0.0;

        let (assign10700_e10160,) = {
    if (var_psceedge_p > 0.0) {
        (var_psceedge_p,)
    } else {
        (0.0,)
    }
};
        var_psceedge_i = assign10700_e10160;
        var_psceedge_i_rv = 0.0;

        let (assign10710_e10171,) = {
    if (var_pscebedge_p > 0.0) {
        let (assign10710_e10169,) = {
            if (var_pscebedge_p < 1.0) {
                (var_pscebedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10710_e10169,)
    } else {
        (0.0,)
    }
};
        var_pscebedge_i = assign10710_e10171;
        var_pscebedge_i_rv = 0.0;

        let (assign10720_e10177,) = {
    if (var_pscededge_p > 0.0) {
        (var_pscededge_p,)
    } else {
        (0.0,)
    }
};
        var_pscededge_i = assign10720_e10177;
        var_pscededge_i_rv = 0.0;

        let (assign10730_e10183,) = {
    if (var_cfedge_p > 0.0) {
        (var_cfedge_p,)
    } else {
        (0.0,)
    }
};
        var_cfedge_i = assign10730_e10183;
        var_cfedge_i_rv = 0.0;

        let (assign10740_e10194,) = {
    if (var_cfbedge_p > 0.0) {
        let (assign10740_e10192,) = {
            if (var_cfbedge_p < 1.0) {
                (var_cfbedge_p,)
            } else {
                (1.0,)
            }
        };
        (assign10740_e10192,)
    } else {
        (0.0,)
    }
};
        var_cfbedge_i = assign10740_e10194;
        var_cfbedge_i_rv = 0.0;

        let (assign10750_e10200,) = {
    if (var_cfdedge_p > 0.0) {
        (var_cfdedge_p,)
    } else {
        (0.0,)
    }
};
        var_cfdedge_i = assign10750_e10200;
        var_cfdedge_i_rv = 0.0;

        let (assign10890_e10244,) = {
    if (var_cth_p > 0.0) {
        (var_cth_p,)
    } else {
        (0.0,)
    }
};
        var_cth_i = assign10890_e10244;
        var_cth_i_rv = 0.0;

        let assign10910_e10248: f64 = (p.p31 * var_nf_i);
        let (assign10910_e10255,) = {
    if (assign10910_e10248 > 0.0) {
        let assign10910_e10253: f64 = (p.p31 * var_nf_i);
        (assign10910_e10253,)
    } else {
        (0.0,)
    }
};
        var_mult_inst = assign10910_e10255;
        var_mult_inst_rv = 0.0;

        var_factuo_i = p.p16;
        var_factuo_i_rv = 0.0;

        var_delvto_i = p.p15;
        var_delvto_i_rv = 0.0;

        var_factuoedge_i = p.p18;
        var_factuoedge_i_rv = 0.0;

        var_delvtoedge_i = p.p17;
        var_delvtoedge_i_rv = 0.0;

        let assign10960_e10262: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        var_guard150 = assign10960_e10262;
        var_guard150_rv = 0.0;

        let (assign10970_e10266,) = {
    if (var_guard150 != 0.0) {
        (var_toxov_i,)
    } else {
        (var_toxovd_i,)
    }
};
        var_toxovd_i = assign10970_e10266;
        var_toxovd_i_rv = 0.0;

        let (assign10980_e10270,) = {
    if (var_guard150 != 0.0) {
        (var_nov_i,)
    } else {
        (var_novd_i,)
    }
};
        var_novd_i = assign10980_e10270;
        var_novd_i_rv = 0.0;

        let (assign10990_e10274,) = {
    if (var_guard150 != 0.0) {
        (var_agidl_i,)
    } else {
        (var_agidld_i,)
    }
};
        var_agidld_i = assign10990_e10274;
        var_agidld_i_rv = 0.0;

        let (assign11000_e10278,) = {
    if (var_guard150 != 0.0) {
        (var_bgidl_i,)
    } else {
        (var_bgidld_i,)
    }
};
        var_bgidld_i = assign11000_e10278;
        var_bgidld_i_rv = 0.0;

        let (assign11010_e10282,) = {
    if (var_guard150 != 0.0) {
        (var_stbgidl_i,)
    } else {
        (var_stbgidld_i,)
    }
};
        var_stbgidld_i = assign11010_e10282;
        var_stbgidld_i_rv = 0.0;

        *var_a1_i_slot = var_a1_i;
        *var_a1_i_rv_slot = var_a1_i_rv;
        *var_a2_i_slot = var_a2_i;
        *var_a2_i_rv_slot = var_a2_i_rv;
        *var_a3_i_slot = var_a3_i;
        *var_a3_i_rv_slot = var_a3_i_rv;
        *var_a4_i_slot = var_a4_i;
        *var_a4_i_rv_slot = var_a4_i_rv;
        *var_agidl_i_slot = var_agidl_i;
        *var_agidl_i_rv_slot = var_agidl_i_rv;
        *var_agidld_i_slot = var_agidld_i;
        *var_agidld_i_rv_slot = var_agidld_i_rv;
        *var_alp1ac_i_slot = var_alp1ac_i;
        *var_alp1ac_i_rv_slot = var_alp1ac_i_rv;
        *var_alpac_i_slot = var_alpac_i;
        *var_alpac_i_rv_slot = var_alpac_i_rv;
        *var_axac_i_slot = var_axac_i;
        *var_axac_i_rv_slot = var_axac_i_rv;
        *var_axinr_i_slot = var_axinr_i;
        *var_axinr_i_rv_slot = var_axinr_i_rv;
        *var_betnedge_i_slot = var_betnedge_i;
        *var_betnedge_i_rv_slot = var_betnedge_i_rv;
        *var_bgidl_i_slot = var_bgidl_i;
        *var_bgidl_i_rv_slot = var_bgidl_i_rv;
        *var_bgidld_i_slot = var_bgidld_i;
        *var_bgidld_i_rv_slot = var_bgidld_i_rv;
        *var_cfbedge_i_slot = var_cfbedge_i;
        *var_cfbedge_i_rv_slot = var_cfbedge_i_rv;
        *var_cfdedge_i_slot = var_cfdedge_i;
        *var_cfdedge_i_rv_slot = var_cfdedge_i_rv;
        *var_cfedge_i_slot = var_cfedge_i;
        *var_cfedge_i_rv_slot = var_cfedge_i_rv;
        *var_cfr_i_slot = var_cfr_i;
        *var_cfr_i_rv_slot = var_cfr_i_rv;
        *var_cfrd_i_slot = var_cfrd_i;
        *var_cfrd_i_rv_slot = var_cfrd_i_rv;
        *var_cgbov_i_slot = var_cgbov_i;
        *var_cgbov_i_rv_slot = var_cgbov_i_rv;
        *var_cgidl_i_slot = var_cgidl_i;
        *var_cgidl_i_rv_slot = var_cgidl_i_rv;
        *var_cgidld_i_slot = var_cgidld_i;
        *var_cgidld_i_rv_slot = var_cgidld_i_rv;
        *var_cgov_i_slot = var_cgov_i;
        *var_cgov_i_rv_slot = var_cgov_i_rv;
        *var_cgovaccg_i_slot = var_cgovaccg_i;
        *var_cgovaccg_i_rv_slot = var_cgovaccg_i_rv;
        *var_cgovd_i_slot = var_cgovd_i;
        *var_cgovd_i_rv_slot = var_cgovd_i_rv;
        *var_chib_i_slot = var_chib_i;
        *var_chib_i_rv_slot = var_chib_i_rv;
        *var_cinr_i_slot = var_cinr_i;
        *var_cinr_i_rv_slot = var_cinr_i_rv;
        *var_cinrd_i_slot = var_cinrd_i;
        *var_cinrd_i_rv_slot = var_cinrd_i_rv;
        *var_cox_i_slot = var_cox_i;
        *var_cox_i_rv_slot = var_cox_i_rv;
        *var_ctedge_i_slot = var_ctedge_i;
        *var_ctedge_i_rv_slot = var_ctedge_i_rv;
        *var_cth_i_slot = var_cth_i;
        *var_cth_i_rv_slot = var_cth_i_rv;
        *var_delvtac_i_slot = var_delvtac_i;
        *var_delvtac_i_rv_slot = var_delvtac_i_rv;
        *var_delvto_i_slot = var_delvto_i;
        *var_delvto_i_rv_slot = var_delvto_i_rv;
        *var_delvtoedge_i_slot = var_delvtoedge_i;
        *var_delvtoedge_i_rv_slot = var_delvtoedge_i_rv;
        *var_dphibedge_i_slot = var_dphibedge_i;
        *var_dphibedge_i_rv_slot = var_dphibedge_i_rv;
        *var_dvfbinr_i_slot = var_dvfbinr_i;
        *var_dvfbinr_i_rv_slot = var_dvfbinr_i_rv;
        *var_facneffac_i_slot = var_facneffac_i;
        *var_facneffac_i_rv_slot = var_facneffac_i_rv;
        *var_factuo_i_slot = var_factuo_i;
        *var_factuo_i_rv_slot = var_factuo_i_rv;
        *var_factuoedge_i_slot = var_factuoedge_i;
        *var_factuoedge_i_rv_slot = var_factuoedge_i_rv;
        *var_fcgovacc_i_slot = var_fcgovacc_i;
        *var_fcgovacc_i_rv_slot = var_fcgovacc_i_rv;
        *var_fcgovaccd_i_slot = var_fcgovaccd_i;
        *var_fcgovaccd_i_rv_slot = var_fcgovaccd_i_rv;
        *var_fcinracc_i_slot = var_fcinracc_i;
        *var_fcinracc_i_rv_slot = var_fcinracc_i_rv;
        *var_fcinrdep_i_slot = var_fcinrdep_i;
        *var_fcinrdep_i_rv_slot = var_fcinrdep_i_rv;
        *var_fnt_i_slot = var_fnt_i;
        *var_fnt_i_rv_slot = var_fnt_i_rv;
        *var_gc2_i_slot = var_gc2_i;
        *var_gc2_i_rv_slot = var_gc2_i_rv;
        *var_gc2ov_i_slot = var_gc2ov_i;
        *var_gc2ov_i_rv_slot = var_gc2ov_i_rv;
        *var_gc2ovd_i_slot = var_gc2ovd_i;
        *var_gc2ovd_i_rv_slot = var_gc2ovd_i_rv;
        *var_gc3_i_slot = var_gc3_i;
        *var_gc3_i_rv_slot = var_gc3_i_rv;
        *var_gc3ov_i_slot = var_gc3ov_i;
        *var_gc3ov_i_rv_slot = var_gc3ov_i_rv;
        *var_gc3ovd_i_slot = var_gc3ovd_i;
        *var_gc3ovd_i_rv_slot = var_gc3ovd_i_rv;
        *var_gco_i_slot = var_gco_i;
        *var_gco_i_rv_slot = var_gco_i_rv;
        *var_guard150_slot = var_guard150;
        *var_guard150_rv_slot = var_guard150_rv;
        *var_iginv_i_slot = var_iginv_i;
        *var_iginv_i_rv_slot = var_iginv_i_rv;
        *var_igov_i_slot = var_igov_i;
        *var_igov_i_rv_slot = var_igov_i_rv;
        *var_igovd_i_slot = var_igovd_i;
        *var_igovd_i_rv_slot = var_igovd_i_rv;
        *var_imaxii_i_slot = var_imaxii_i;
        *var_imaxii_i_rv_slot = var_imaxii_i_rv;
        *var_mult_inst_slot = var_mult_inst;
        *var_mult_inst_rv_slot = var_mult_inst_rv;
        *var_neffedge_i_slot = var_neffedge_i;
        *var_neffedge_i_rv_slot = var_neffedge_i_rv;
        *var_novd_i_slot = var_novd_i;
        *var_novd_i_rv_slot = var_novd_i_rv;
        *var_pscebedge_i_slot = var_pscebedge_i;
        *var_pscebedge_i_rv_slot = var_pscebedge_i_rv;
        *var_pscededge_i_slot = var_pscededge_i;
        *var_pscededge_i_rv_slot = var_pscededge_i_rv;
        *var_psceedge_i_slot = var_psceedge_i;
        *var_psceedge_i_rv_slot = var_psceedge_i_rv;
        *var_sta2_i_slot = var_sta2_i;
        *var_sta2_i_rv_slot = var_sta2_i_rv;
        *var_stbetedge_i_slot = var_stbetedge_i;
        *var_stbetedge_i_rv_slot = var_stbetedge_i_rv;
        *var_stbgidl_i_slot = var_stbgidl_i;
        *var_stbgidl_i_rv_slot = var_stbgidl_i_rv;
        *var_stbgidld_i_slot = var_stbgidld_i;
        *var_stbgidld_i_rv_slot = var_stbgidld_i_rv;
        *var_stig_i_slot = var_stig_i;
        *var_stig_i_rv_slot = var_stig_i_rv;
        *var_stvfbedge_i_slot = var_stvfbedge_i;
        *var_stvfbedge_i_rv_slot = var_stvfbedge_i_rv;
        *var_thesatac_i_slot = var_thesatac_i;
        *var_thesatac_i_rv_slot = var_thesatac_i_rv;
        *var_toxovd_i_slot = var_toxovd_i;
        *var_toxovd_i_rv_slot = var_toxovd_i_rv;
        *var_vfbedge_i_slot = var_vfbedge_i;
        *var_vfbedge_i_rv_slot = var_vfbedge_i_rv;
        *var_vp_i_slot = var_vp_i;
        *var_vp_i_rv_slot = var_vp_i_rv;
    }

    pub(super) fn stamp_reactive_block_17(
        p: &Parameters,
        var_ax_i: f64,
        var_axac_i: f64,
        var_cfr_i: f64,
        var_cgidl_i: f64,
        var_cgov_i: f64,
        var_cgovaccg_i: f64,
        var_chib_i: f64,
        var_chnl_type: f64,
        var_cinr_i: f64,
        var_epsrox_i: f64,
        var_epssi: f64,
        var_facneffac_i: f64,
        var_fcgovacc_i: f64,
        var_feta_i: f64,
        var_gc2ov_i: f64,
        var_gc3_i: f64,
        var_gc3ov_i: f64,
        var_guard150: f64,
        var_igov_i: f64,
        var_inv_phita: f64,
        var_neff_i: f64,
        var_nov_i: f64,
        var_novd_i: f64,
        var_tox_i: f64,
        var_toxov_i: f64,
        var_toxovd_i: f64,
        var_vp_i: f64,
        var_ar_slot: &mut f64,
        var_ar_rv_slot: &mut f64,
        var_arac_slot: &mut f64,
        var_arac_rv_slot: &mut f64,
        var_b_fact_slot: &mut f64,
        var_b_fact_rv_slot: &mut f64,
        var_bch_slot: &mut f64,
        var_bch_rv_slot: &mut f64,
        var_bov_slot: &mut f64,
        var_bov_d_slot: &mut f64,
        var_bov_d_rv_slot: &mut f64,
        var_bov_rv_slot: &mut f64,
        var_cfrd_i_slot: &mut f64,
        var_cfrd_i_rv_slot: &mut f64,
        var_cgidld_i_slot: &mut f64,
        var_cgidld_i_rv_slot: &mut f64,
        var_cgovd_i_slot: &mut f64,
        var_cgovd_i_rv_slot: &mut f64,
        var_cinrd_i_slot: &mut f64,
        var_cinrd_i_rv_slot: &mut f64,
        var_cox_over_q_slot: &mut f64,
        var_cox_over_q_rv_slot: &mut f64,
        var_coxovprime_slot: &mut f64,
        var_coxovprime_d_slot: &mut f64,
        var_coxovprime_d_rv_slot: &mut f64,
        var_coxovprime_rv_slot: &mut f64,
        var_coxprime_slot: &mut f64,
        var_coxprime_rv_slot: &mut f64,
        var_dxgb_ov_d_slot: &mut f64,
        var_dxgb_ov_d_rv_slot: &mut f64,
        var_dxgb_ov_s_slot: &mut f64,
        var_dxgb_ov_s_rv_slot: &mut f64,
        var_dxgb_ov_th_slot: &mut f64,
        var_dxgb_ov_th_rv_slot: &mut f64,
        var_e_eff0_slot: &mut f64,
        var_e_eff0_rv_slot: &mut f64,
        var_epsox_slot: &mut f64,
        var_epsox_rv_slot: &mut f64,
        var_eta_mu_slot: &mut f64,
        var_eta_mu1_slot: &mut f64,
        var_eta_mu1_rv_slot: &mut f64,
        var_eta_mu_rv_slot: &mut f64,
        var_fcgovaccd_i_slot: &mut f64,
        var_fcgovaccd_i_rv_slot: &mut f64,
        var_gc2ovd_i_slot: &mut f64,
        var_gc2ovd_i_rv_slot: &mut f64,
        var_gc3ovd_i_slot: &mut f64,
        var_gc3ovd_i_rv_slot: &mut f64,
        var_gcq_slot: &mut f64,
        var_gcq_rv_slot: &mut f64,
        var_gov2_d_slot: &mut f64,
        var_gov2_d_rv_slot: &mut f64,
        var_gov2_s_slot: &mut f64,
        var_gov2_s_rv_slot: &mut f64,
        var_gov_d_slot: &mut f64,
        var_gov_d_rv_slot: &mut f64,
        var_gov_s_slot: &mut f64,
        var_gov_s_rv_slot: &mut f64,
        var_guard151_slot: &mut f64,
        var_guard151_rv_slot: &mut f64,
        var_guard152_slot: &mut f64,
        var_guard152_rv_slot: &mut f64,
        var_guard153_slot: &mut f64,
        var_guard153_rv_slot: &mut f64,
        var_guard154_slot: &mut f64,
        var_guard154_rv_slot: &mut f64,
        var_guard155_slot: &mut f64,
        var_guard155_rv_slot: &mut f64,
        var_guard156_slot: &mut f64,
        var_guard156_rv_slot: &mut f64,
        var_guard157_slot: &mut f64,
        var_guard157_rv_slot: &mut f64,
        var_guard158_slot: &mut f64,
        var_guard158_rv_slot: &mut f64,
        var_guard159_slot: &mut f64,
        var_guard159_rv_slot: &mut f64,
        var_guard160_slot: &mut f64,
        var_guard160_rv_slot: &mut f64,
        var_igovd_i_slot: &mut f64,
        var_igovd_i_rv_slot: &mut f64,
        var_inv_chib_slot: &mut f64,
        var_inv_chib_rv_slot: &mut f64,
        var_inv_gov_slot: &mut f64,
        var_inv_gov_rv_slot: &mut f64,
        var_inv_vp_slot: &mut f64,
        var_inv_vp_rv_slot: &mut f64,
        var_neffac_i_slot: &mut f64,
        var_neffac_i_rv_slot: &mut f64,
        var_qq_slot: &mut f64,
        var_qq_rv_slot: &mut f64,
        var_sp_ov_a_d_slot: &mut f64,
        var_sp_ov_a_d_rv_slot: &mut f64,
        var_sp_ov_a_s_slot: &mut f64,
        var_sp_ov_a_s_rv_slot: &mut f64,
        var_sp_ov_delta_slot: &mut f64,
        var_sp_ov_delta1_d_slot: &mut f64,
        var_sp_ov_delta1_d_rv_slot: &mut f64,
        var_sp_ov_delta1_s_slot: &mut f64,
        var_sp_ov_delta1_s_rv_slot: &mut f64,
        var_sp_ov_delta_rv_slot: &mut f64,
        var_sp_ov_eps_slot: &mut f64,
        var_sp_ov_eps2_d_slot: &mut f64,
        var_sp_ov_eps2_d_rv_slot: &mut f64,
        var_sp_ov_eps2_s_slot: &mut f64,
        var_sp_ov_eps2_s_rv_slot: &mut f64,
        var_sp_ov_eps_rv_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_temp_rv_slot: &mut f64,
        var_tox_sq_slot: &mut f64,
        var_tox_sq_rv_slot: &mut f64,
    ) {
        let mut var_ar: f64 = *var_ar_slot;
        let mut var_ar_rv: f64 = *var_ar_rv_slot;
        let mut var_arac: f64 = *var_arac_slot;
        let mut var_arac_rv: f64 = *var_arac_rv_slot;
        let mut var_b_fact: f64 = *var_b_fact_slot;
        let mut var_b_fact_rv: f64 = *var_b_fact_rv_slot;
        let mut var_bch: f64 = *var_bch_slot;
        let mut var_bch_rv: f64 = *var_bch_rv_slot;
        let mut var_bov: f64 = *var_bov_slot;
        let mut var_bov_d: f64 = *var_bov_d_slot;
        let mut var_bov_d_rv: f64 = *var_bov_d_rv_slot;
        let mut var_bov_rv: f64 = *var_bov_rv_slot;
        let mut var_cfrd_i: f64 = *var_cfrd_i_slot;
        let mut var_cfrd_i_rv: f64 = *var_cfrd_i_rv_slot;
        let mut var_cgidld_i: f64 = *var_cgidld_i_slot;
        let mut var_cgidld_i_rv: f64 = *var_cgidld_i_rv_slot;
        let mut var_cgovd_i: f64 = *var_cgovd_i_slot;
        let mut var_cgovd_i_rv: f64 = *var_cgovd_i_rv_slot;
        let mut var_cinrd_i: f64 = *var_cinrd_i_slot;
        let mut var_cinrd_i_rv: f64 = *var_cinrd_i_rv_slot;
        let mut var_cox_over_q: f64 = *var_cox_over_q_slot;
        let mut var_cox_over_q_rv: f64 = *var_cox_over_q_rv_slot;
        let mut var_coxovprime: f64 = *var_coxovprime_slot;
        let mut var_coxovprime_d: f64 = *var_coxovprime_d_slot;
        let mut var_coxovprime_d_rv: f64 = *var_coxovprime_d_rv_slot;
        let mut var_coxovprime_rv: f64 = *var_coxovprime_rv_slot;
        let mut var_coxprime: f64 = *var_coxprime_slot;
        let mut var_coxprime_rv: f64 = *var_coxprime_rv_slot;
        let mut var_dxgb_ov_d: f64 = *var_dxgb_ov_d_slot;
        let mut var_dxgb_ov_d_rv: f64 = *var_dxgb_ov_d_rv_slot;
        let mut var_dxgb_ov_s: f64 = *var_dxgb_ov_s_slot;
        let mut var_dxgb_ov_s_rv: f64 = *var_dxgb_ov_s_rv_slot;
        let mut var_dxgb_ov_th: f64 = *var_dxgb_ov_th_slot;
        let mut var_dxgb_ov_th_rv: f64 = *var_dxgb_ov_th_rv_slot;
        let mut var_e_eff0: f64 = *var_e_eff0_slot;
        let mut var_e_eff0_rv: f64 = *var_e_eff0_rv_slot;
        let mut var_epsox: f64 = *var_epsox_slot;
        let mut var_epsox_rv: f64 = *var_epsox_rv_slot;
        let mut var_eta_mu: f64 = *var_eta_mu_slot;
        let mut var_eta_mu1: f64 = *var_eta_mu1_slot;
        let mut var_eta_mu1_rv: f64 = *var_eta_mu1_rv_slot;
        let mut var_eta_mu_rv: f64 = *var_eta_mu_rv_slot;
        let mut var_fcgovaccd_i: f64 = *var_fcgovaccd_i_slot;
        let mut var_fcgovaccd_i_rv: f64 = *var_fcgovaccd_i_rv_slot;
        let mut var_gc2ovd_i: f64 = *var_gc2ovd_i_slot;
        let mut var_gc2ovd_i_rv: f64 = *var_gc2ovd_i_rv_slot;
        let mut var_gc3ovd_i: f64 = *var_gc3ovd_i_slot;
        let mut var_gc3ovd_i_rv: f64 = *var_gc3ovd_i_rv_slot;
        let mut var_gcq: f64 = *var_gcq_slot;
        let mut var_gcq_rv: f64 = *var_gcq_rv_slot;
        let mut var_gov2_d: f64 = *var_gov2_d_slot;
        let mut var_gov2_d_rv: f64 = *var_gov2_d_rv_slot;
        let mut var_gov2_s: f64 = *var_gov2_s_slot;
        let mut var_gov2_s_rv: f64 = *var_gov2_s_rv_slot;
        let mut var_gov_d: f64 = *var_gov_d_slot;
        let mut var_gov_d_rv: f64 = *var_gov_d_rv_slot;
        let mut var_gov_s: f64 = *var_gov_s_slot;
        let mut var_gov_s_rv: f64 = *var_gov_s_rv_slot;
        let mut var_guard151: f64 = *var_guard151_slot;
        let mut var_guard151_rv: f64 = *var_guard151_rv_slot;
        let mut var_guard152: f64 = *var_guard152_slot;
        let mut var_guard152_rv: f64 = *var_guard152_rv_slot;
        let mut var_guard153: f64 = *var_guard153_slot;
        let mut var_guard153_rv: f64 = *var_guard153_rv_slot;
        let mut var_guard154: f64 = *var_guard154_slot;
        let mut var_guard154_rv: f64 = *var_guard154_rv_slot;
        let mut var_guard155: f64 = *var_guard155_slot;
        let mut var_guard155_rv: f64 = *var_guard155_rv_slot;
        let mut var_guard156: f64 = *var_guard156_slot;
        let mut var_guard156_rv: f64 = *var_guard156_rv_slot;
        let mut var_guard157: f64 = *var_guard157_slot;
        let mut var_guard157_rv: f64 = *var_guard157_rv_slot;
        let mut var_guard158: f64 = *var_guard158_slot;
        let mut var_guard158_rv: f64 = *var_guard158_rv_slot;
        let mut var_guard159: f64 = *var_guard159_slot;
        let mut var_guard159_rv: f64 = *var_guard159_rv_slot;
        let mut var_guard160: f64 = *var_guard160_slot;
        let mut var_guard160_rv: f64 = *var_guard160_rv_slot;
        let mut var_igovd_i: f64 = *var_igovd_i_slot;
        let mut var_igovd_i_rv: f64 = *var_igovd_i_rv_slot;
        let mut var_inv_chib: f64 = *var_inv_chib_slot;
        let mut var_inv_chib_rv: f64 = *var_inv_chib_rv_slot;
        let mut var_inv_gov: f64 = *var_inv_gov_slot;
        let mut var_inv_gov_rv: f64 = *var_inv_gov_rv_slot;
        let mut var_inv_vp: f64 = *var_inv_vp_slot;
        let mut var_inv_vp_rv: f64 = *var_inv_vp_rv_slot;
        let mut var_neffac_i: f64 = *var_neffac_i_slot;
        let mut var_neffac_i_rv: f64 = *var_neffac_i_rv_slot;
        let mut var_qq: f64 = *var_qq_slot;
        let mut var_qq_rv: f64 = *var_qq_rv_slot;
        let mut var_sp_ov_a_d: f64 = *var_sp_ov_a_d_slot;
        let mut var_sp_ov_a_d_rv: f64 = *var_sp_ov_a_d_rv_slot;
        let mut var_sp_ov_a_s: f64 = *var_sp_ov_a_s_slot;
        let mut var_sp_ov_a_s_rv: f64 = *var_sp_ov_a_s_rv_slot;
        let mut var_sp_ov_delta: f64 = *var_sp_ov_delta_slot;
        let mut var_sp_ov_delta1_d: f64 = *var_sp_ov_delta1_d_slot;
        let mut var_sp_ov_delta1_d_rv: f64 = *var_sp_ov_delta1_d_rv_slot;
        let mut var_sp_ov_delta1_s: f64 = *var_sp_ov_delta1_s_slot;
        let mut var_sp_ov_delta1_s_rv: f64 = *var_sp_ov_delta1_s_rv_slot;
        let mut var_sp_ov_delta_rv: f64 = *var_sp_ov_delta_rv_slot;
        let mut var_sp_ov_eps: f64 = *var_sp_ov_eps_slot;
        let mut var_sp_ov_eps2_d: f64 = *var_sp_ov_eps2_d_slot;
        let mut var_sp_ov_eps2_d_rv: f64 = *var_sp_ov_eps2_d_rv_slot;
        let mut var_sp_ov_eps2_s: f64 = *var_sp_ov_eps2_s_slot;
        let mut var_sp_ov_eps2_s_rv: f64 = *var_sp_ov_eps2_s_rv_slot;
        let mut var_sp_ov_eps_rv: f64 = *var_sp_ov_eps_rv_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_temp_rv: f64 = *var_temp_rv_slot;
        let mut var_tox_sq: f64 = *var_tox_sq_slot;
        let mut var_tox_sq_rv: f64 = *var_tox_sq_rv_slot;

        let (assign11020_e10286,) = {
    if (var_guard150 != 0.0) {
        (var_cgidl_i,)
    } else {
        (var_cgidld_i,)
    }
};
        var_cgidld_i = assign11020_e10286;
        var_cgidld_i_rv = 0.0;

        let (assign11030_e10290,) = {
    if (var_guard150 != 0.0) {
        (var_igov_i,)
    } else {
        (var_igovd_i,)
    }
};
        var_igovd_i = assign11030_e10290;
        var_igovd_i_rv = 0.0;

        let (assign11040_e10294,) = {
    if (var_guard150 != 0.0) {
        (var_gc2ov_i,)
    } else {
        (var_gc2ovd_i,)
    }
};
        var_gc2ovd_i = assign11040_e10294;
        var_gc2ovd_i_rv = 0.0;

        let (assign11050_e10298,) = {
    if (var_guard150 != 0.0) {
        (var_gc3ov_i,)
    } else {
        (var_gc3ovd_i,)
    }
};
        var_gc3ovd_i = assign11050_e10298;
        var_gc3ovd_i_rv = 0.0;

        let (assign11060_e10302,) = {
    if (var_guard150 != 0.0) {
        (var_cgov_i,)
    } else {
        (var_cgovd_i,)
    }
};
        var_cgovd_i = assign11060_e10302;
        var_cgovd_i_rv = 0.0;

        let (assign11070_e10306,) = {
    if (var_guard150 != 0.0) {
        (var_fcgovacc_i,)
    } else {
        (var_fcgovaccd_i,)
    }
};
        var_fcgovaccd_i = assign11070_e10306;
        var_fcgovaccd_i_rv = 0.0;

        let (assign11080_e10310,) = {
    if (var_guard150 != 0.0) {
        (var_cinr_i,)
    } else {
        (var_cinrd_i,)
    }
};
        var_cinrd_i = assign11080_e10310;
        var_cinrd_i_rv = 0.0;

        let (assign11090_e10314,) = {
    if (var_guard150 != 0.0) {
        (var_cfr_i,)
    } else {
        (var_cfrd_i,)
    }
};
        var_cfrd_i = assign11090_e10314;
        var_cfrd_i_rv = 0.0;

        let assign11100_e10317: f64 = (8.8541878176e-12 * var_epsrox_i);
        var_epsox = assign11100_e10317;
        var_epsox_rv = 0.0;

        let assign11110_e10320: f64 = (var_epsox / var_tox_i);
        var_coxprime = assign11110_e10320;
        var_coxprime_rv = 0.0;

        let assign11120_e10323: f64 = (var_tox_i * var_tox_i);
        var_tox_sq = assign11120_e10323;
        var_tox_sq_rv = 0.0;

        let assign11130_e10326: f64 = (var_coxprime / 1.6021918e-19);
        var_cox_over_q = assign11130_e10326;
        var_cox_over_q_rv = 0.0;

        let assign11140_e10329: f64 = (var_facneffac_i * var_neff_i);
        var_neffac_i = assign11140_e10329;
        var_neffac_i_rv = 0.0;

        let (assign11150_e10340,) = {
    if (var_neffac_i > 1e20) {
        let (assign11150_e10338,) = {
            if (var_neffac_i < 1e26) {
                (var_neffac_i,)
            } else {
                (1e26,)
            }
        };
        (assign11150_e10338,)
    } else {
        (1e20,)
    }
};
        var_neffac_i = assign11150_e10340;
        var_neffac_i_rv = 0.0;

        var_qq = 0.0;
        var_qq_rv = 0.0;

        let assign11170_e10344: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };
        var_guard151 = assign11170_e10344;
        var_guard151_rv = 0.0;

        let (assign11180_e10356,) = {
    if (var_guard151 != 0.0) {
        let assign11180_e10348: f64 = (0.4 * 5.951993);
        let assign11180_e10350: f64 = (assign11180_e10348 * p.p51);
        let assign11180_e10353: f64 = (var_coxprime).powf(0.6666666666666666);
        let assign11180_e10354: f64 = (assign11180_e10350 * assign11180_e10353);
        (assign11180_e10354,)
    } else {
        (var_qq,)
    }
};
        var_qq = assign11180_e10356;
        var_qq_rv = 0.0;

        let assign11190_e10359: f64 = (-1.0);
        let assign11190_e10360: f64 = if var_chnl_type == assign11190_e10359 { 1.0 } else { 0.0 };
        var_guard152 = assign11190_e10360;
        var_guard152_rv = 0.0;

        let (assign11200_e10370,) = {
    if ((var_guard151 != 0.0) && (var_guard152 != 0.0)) {
        let assign11200_e10366: f64 = (7.448711 / 5.951993);
        let assign11200_e10368: f64 = (assign11200_e10366 * var_qq);
        (assign11200_e10368,)
    } else {
        (var_qq,)
    }
};
        var_qq = assign11200_e10370;
        var_qq_rv = 0.0;

        let assign11210_e10373: f64 = (1e-8 * var_coxprime);
        let assign11210_e10375: f64 = (assign11210_e10373 / var_epssi);
        var_e_eff0 = assign11210_e10375;
        var_e_eff0_rv = 0.0;

        let assign11220_e10378: f64 = (0.5 * var_feta_i);
        var_eta_mu = assign11220_e10378;
        var_eta_mu_rv = 0.0;

        var_eta_mu1 = 0.5;
        var_eta_mu1_rv = 0.0;

        let assign11240_e10382: f64 = (-1.0);
        let assign11240_e10383: f64 = if var_chnl_type == assign11240_e10382 { 1.0 } else { 0.0 };
        var_guard153 = assign11240_e10383;
        var_guard153_rv = 0.0;

        let (assign11250_e10389,) = {
    if (var_guard153 != 0.0) {
        let assign11250_e10387: f64 = (0.3333333333333333 * var_feta_i);
        (assign11250_e10387,)
    } else {
        (var_eta_mu,)
    }
};
        var_eta_mu = assign11250_e10389;
        var_eta_mu_rv = 0.0;

        let (assign11260_e10393,) = {
    if (var_guard153 != 0.0) {
        (0.3333333333333333,)
    } else {
        (var_eta_mu1,)
    }
};
        var_eta_mu1 = assign11260_e10393;
        var_eta_mu1_rv = 0.0;

        let assign11270_e10396: f64 = (-2.0);
        let assign11270_e10398: f64 = (assign11270_e10396 / var_ax_i);
        let assign11270_e10400: f64 = (assign11270_e10398 + 1.0);
        let assign11270_e10401: f64 = (2.0_f64).powf(assign11270_e10400);
        let assign11270_e10403: f64 = (assign11270_e10401 - 1.0);
        var_temp = assign11270_e10403;
        var_temp_rv = 0.0;

        let assign11280_e10406: f64 = (var_temp - 1.0);
        let assign11280_e10409: f64 = (var_temp - 1.0);
        let assign11280_e10410: f64 = (assign11280_e10406 * assign11280_e10409);
        let assign11280_e10413: f64 = (4.0 * var_temp);
        let (assign11280_e10420,) = {
    if (assign11280_e10413 > 0.0001) {
        let assign11280_e10418: f64 = (4.0 * var_temp);
        (assign11280_e10418,)
    } else {
        (0.0001,)
    }
};
        let assign11280_e10421: f64 = (assign11280_e10410 / assign11280_e10420);
        var_ar = assign11280_e10421;
        var_ar_rv = 0.0;

        let assign11290_e10424: f64 = (-2.0);
        let assign11290_e10426: f64 = (assign11290_e10424 / var_axac_i);
        let assign11290_e10428: f64 = (assign11290_e10426 + 1.0);
        let assign11290_e10429: f64 = (2.0_f64).powf(assign11290_e10428);
        let assign11290_e10431: f64 = (assign11290_e10429 - 1.0);
        var_temp = assign11290_e10431;
        var_temp_rv = 0.0;

        let assign11300_e10434: f64 = (var_temp - 1.0);
        let assign11300_e10437: f64 = (var_temp - 1.0);
        let assign11300_e10438: f64 = (assign11300_e10434 * assign11300_e10437);
        let assign11300_e10441: f64 = (4.0 * var_temp);
        let (assign11300_e10448,) = {
    if (assign11300_e10441 > 0.0001) {
        let assign11300_e10446: f64 = (4.0 * var_temp);
        (assign11300_e10446,)
    } else {
        (0.0001,)
    }
};
        let assign11300_e10449: f64 = (assign11300_e10438 / assign11300_e10448);
        var_arac = assign11300_e10449;
        var_arac_rv = 0.0;

        let assign11310_e10452: f64 = (1.0 / var_vp_i);
        var_inv_vp = assign11310_e10452;
        var_inv_vp_rv = 0.0;

        let assign11320_e10455: f64 = (var_epsox / var_toxov_i);
        var_coxovprime = assign11320_e10455;
        var_coxovprime_rv = 0.0;

        let assign11330_e10458: f64 = (var_epsox / var_toxovd_i);
        var_coxovprime_d = assign11330_e10458;
        var_coxovprime_d_rv = 0.0;

        let assign11340_e10461: f64 = (2.0 * 1.6021918e-19);
        let assign11340_e10463: f64 = (assign11340_e10461 * var_nov_i);
        let assign11340_e10465: f64 = (assign11340_e10463 * var_epssi);
        let assign11340_e10467: f64 = (assign11340_e10465 * var_inv_phita);
        let assign11340_e10468: f64 = (assign11340_e10467).sqrt();
        let assign11340_e10470: f64 = (assign11340_e10468 / var_coxovprime);
        var_gov_s = assign11340_e10470;
        var_gov_s_rv = 0.0;

        let assign11350_e10473: f64 = (2.0 * 1.6021918e-19);
        let assign11350_e10475: f64 = (assign11350_e10473 * var_novd_i);
        let assign11350_e10477: f64 = (assign11350_e10475 * var_epssi);
        let assign11350_e10479: f64 = (assign11350_e10477 * var_inv_phita);
        let assign11350_e10480: f64 = (assign11350_e10479).sqrt();
        let assign11350_e10482: f64 = (assign11350_e10480 / var_coxovprime_d);
        var_gov_d = assign11350_e10482;
        var_gov_d_rv = 0.0;

        let assign11360_e10485: f64 = (var_gov_s * var_gov_s);
        var_gov2_s = assign11360_e10485;
        var_gov2_s_rv = 0.0;

        let assign11370_e10488: f64 = (var_gov_d * var_gov_d);
        var_gov2_d = assign11370_e10488;
        var_gov2_d_rv = 0.0;

        let assign11380_e10491: f64 = (var_cgovaccg_i * 0.005);
        let assign11380_e10493: f64 = (assign11380_e10491 * var_inv_phita);
        let assign11380_e10494: f64 = (assign11380_e10493).exp();
        let assign11380_e10496: f64 = (assign11380_e10494 - 1.0);
        let assign11380_e10497: f64 = (assign11380_e10496).ln();
        let assign11380_e10499: f64 = (assign11380_e10497 / var_cgovaccg_i);
        let assign11380_e10502: f64 = (0.005 * var_inv_phita);
        let assign11380_e10503: f64 = (assign11380_e10502).exp();
        let assign11380_e10505: f64 = (assign11380_e10503 - 1.0);
        let assign11380_e10506: f64 = (assign11380_e10505).ln();
        let assign11380_e10507: f64 = (assign11380_e10499 - assign11380_e10506);
        var_dxgb_ov_th = assign11380_e10507;
        var_dxgb_ov_th_rv = 0.0;

        let assign11390_e10510: f64 = (0.5 * var_gov_s);
        let assign11390_e10511: f64 = (assign11390_e10510).ln();
        let assign11390_e10513: f64 = (assign11390_e10511 + var_dxgb_ov_th);
        var_dxgb_ov_s = assign11390_e10513;
        var_dxgb_ov_s_rv = 0.0;

        let assign11400_e10516: f64 = (0.5 * var_gov_d);
        let assign11400_e10517: f64 = (assign11400_e10516).ln();
        let assign11400_e10519: f64 = (assign11400_e10517 + var_dxgb_ov_th);
        var_dxgb_ov_d = assign11400_e10519;
        var_dxgb_ov_d_rv = 0.0;

        let assign11410_e10522: f64 = (1.0 / var_gov_s);
        var_inv_gov = assign11410_e10522;
        var_inv_gov_rv = 0.0;

        let assign11420_e10525: f64 = (3.1 * var_gov_s);
        let assign11420_e10527: f64 = (assign11420_e10525 + 8.5);
        var_sp_ov_eps = assign11420_e10527;
        var_sp_ov_eps_rv = 0.0;

        let assign11430_e10530: f64 = (var_sp_ov_eps * var_sp_ov_eps);
        var_sp_ov_eps2_s = assign11430_e10530;
        var_sp_ov_eps2_s_rv = 0.0;

        let assign11440_e10533: f64 = (0.5 * var_sp_ov_eps);
        var_sp_ov_delta = assign11440_e10533;
        var_sp_ov_delta_rv = 0.0;

        let assign11450_e10536: f64 = if var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        var_guard154 = assign11450_e10536;
        var_guard154_rv = 0.0;

        let (assign11460_e10542,) = {
    if (var_guard154 != 0.0) {
        let assign11460_e10540: f64 = (64.0 * var_inv_gov);
        (assign11460_e10540,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11460_e10542;
        var_sp_ov_a_s_rv = 0.0;

        let assign11470_e10545: f64 = if var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        var_guard155 = assign11470_e10545;
        var_guard155_rv = 0.0;

        let (assign11480_e10556,) = {
    if ((var_guard154 == 0.0) && (var_guard155 != 0.0)) {
        let assign11480_e10552: f64 = (22.0 * var_inv_gov);
        let assign11480_e10554: f64 = (assign11480_e10552 + 3.0);
        (assign11480_e10554,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11480_e10556;
        var_sp_ov_a_s_rv = 0.0;

        let assign11490_e10559: f64 = if var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        var_guard156 = assign11490_e10559;
        var_guard156_rv = 0.0;

        let (assign11500_e10574,) = {
    if (((var_guard154 == 0.0) && (var_guard155 == 0.0)) && (var_guard156 != 0.0)) {
        let assign11500_e10568: f64 = (-7.2);
        let assign11500_e10570: f64 = (assign11500_e10568 * var_inv_gov);
        let assign11500_e10572: f64 = (assign11500_e10570 + 15.5);
        (assign11500_e10572,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11500_e10574;
        var_sp_ov_a_s_rv = 0.0;

        let (assign11510_e10585,) = {
    if (((var_guard154 == 0.0) && (var_guard155 == 0.0)) && (var_guard156 == 0.0)) {
        (var_gov_s,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11510_e10585;
        var_sp_ov_a_s_rv = 0.0;

        let assign11520_e10589: f64 = (var_gov2_s * 0.5);
        let assign11520_e10590: f64 = (var_sp_ov_delta + assign11520_e10589);
        let assign11520_e10595: f64 = (var_gov2_s * 0.25);
        let assign11520_e10596: f64 = (var_sp_ov_delta + assign11520_e10595);
        let assign11520_e10598: f64 = (assign11520_e10596 + var_sp_ov_a_s);
        let assign11520_e10599: f64 = (assign11520_e10598).sqrt();
        let assign11520_e10600: f64 = (var_gov_s * assign11520_e10599);
        let assign11520_e10601: f64 = (assign11520_e10590 - assign11520_e10600);
        var_sp_ov_delta1_s = assign11520_e10601;
        var_sp_ov_delta1_s_rv = 0.0;

        let assign11530_e10604: f64 = (1.0 / var_gov_d);
        var_inv_gov = assign11530_e10604;
        var_inv_gov_rv = 0.0;

        let assign11540_e10607: f64 = (3.1 * var_gov_d);
        let assign11540_e10609: f64 = (assign11540_e10607 + 8.5);
        var_sp_ov_eps = assign11540_e10609;
        var_sp_ov_eps_rv = 0.0;

        let assign11550_e10612: f64 = (var_sp_ov_eps * var_sp_ov_eps);
        var_sp_ov_eps2_d = assign11550_e10612;
        var_sp_ov_eps2_d_rv = 0.0;

        let assign11560_e10615: f64 = (0.5 * var_sp_ov_eps);
        var_sp_ov_delta = assign11560_e10615;
        var_sp_ov_delta_rv = 0.0;

        let assign11570_e10618: f64 = if var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        var_guard157 = assign11570_e10618;
        var_guard157_rv = 0.0;

        let (assign11580_e10624,) = {
    if (var_guard157 != 0.0) {
        let assign11580_e10622: f64 = (64.0 * var_inv_gov);
        (assign11580_e10622,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11580_e10624;
        var_sp_ov_a_d_rv = 0.0;

        let assign11590_e10627: f64 = if var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        var_guard158 = assign11590_e10627;
        var_guard158_rv = 0.0;

        let (assign11600_e10638,) = {
    if ((var_guard157 == 0.0) && (var_guard158 != 0.0)) {
        let assign11600_e10634: f64 = (22.0 * var_inv_gov);
        let assign11600_e10636: f64 = (assign11600_e10634 + 3.0);
        (assign11600_e10636,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11600_e10638;
        var_sp_ov_a_d_rv = 0.0;

        let assign11610_e10641: f64 = if var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        var_guard159 = assign11610_e10641;
        var_guard159_rv = 0.0;

        let (assign11620_e10656,) = {
    if (((var_guard157 == 0.0) && (var_guard158 == 0.0)) && (var_guard159 != 0.0)) {
        let assign11620_e10650: f64 = (-7.2);
        let assign11620_e10652: f64 = (assign11620_e10650 * var_inv_gov);
        let assign11620_e10654: f64 = (assign11620_e10652 + 15.5);
        (assign11620_e10654,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11620_e10656;
        var_sp_ov_a_d_rv = 0.0;

        let (assign11630_e10667,) = {
    if (((var_guard157 == 0.0) && (var_guard158 == 0.0)) && (var_guard159 == 0.0)) {
        (var_gov_d,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11630_e10667;
        var_sp_ov_a_d_rv = 0.0;

        let assign11640_e10671: f64 = (var_gov2_d * 0.5);
        let assign11640_e10672: f64 = (var_sp_ov_delta + assign11640_e10671);
        let assign11640_e10677: f64 = (var_gov2_d * 0.25);
        let assign11640_e10678: f64 = (var_sp_ov_delta + assign11640_e10677);
        let assign11640_e10680: f64 = (assign11640_e10678 + var_sp_ov_a_d);
        let assign11640_e10681: f64 = (assign11640_e10680).sqrt();
        let assign11640_e10682: f64 = (var_gov_d * assign11640_e10681);
        let assign11640_e10683: f64 = (assign11640_e10672 - assign11640_e10682);
        var_sp_ov_delta1_d = assign11640_e10683;
        var_sp_ov_delta1_d_rv = 0.0;

        let assign11650_e10686: f64 = (1.0 / var_chib_i);
        var_inv_chib = assign11650_e10686;
        var_inv_chib_rv = 0.0;

        let assign11660_e10689: f64 = (4.0 * 0.3333333333333333);
        let assign11660_e10692: f64 = (2.0 * 1.6021918e-19);
        let assign11660_e10694: f64 = (assign11660_e10692 * 9.1093826e-31);
        let assign11660_e10696: f64 = (assign11660_e10694 * var_chib_i);
        let assign11660_e10697: f64 = (assign11660_e10696).sqrt();
        let assign11660_e10698: f64 = (assign11660_e10689 * assign11660_e10697);
        let assign11660_e10700: f64 = (assign11660_e10698 / 1.05457168e-34);
        var_b_fact = assign11660_e10700;
        var_b_fact_rv = 0.0;

        let assign11670_e10703: f64 = (var_b_fact * var_tox_i);
        var_bch = assign11670_e10703;
        var_bch_rv = 0.0;

        let assign11680_e10706: f64 = (var_b_fact * var_toxov_i);
        var_bov = assign11680_e10706;
        var_bov_rv = 0.0;

        let assign11690_e10709: f64 = (var_b_fact * var_toxovd_i);
        var_bov_d = assign11690_e10709;
        var_bov_d_rv = 0.0;

        var_gcq = 0.0;
        var_gcq_rv = 0.0;

        let assign11710_e10713: f64 = if var_gc3_i < 0.0 { 1.0 } else { 0.0 };
        var_guard160 = assign11710_e10713;
        var_guard160_rv = 0.0;

        *var_ar_slot = var_ar;
        *var_ar_rv_slot = var_ar_rv;
        *var_arac_slot = var_arac;
        *var_arac_rv_slot = var_arac_rv;
        *var_b_fact_slot = var_b_fact;
        *var_b_fact_rv_slot = var_b_fact_rv;
        *var_bch_slot = var_bch;
        *var_bch_rv_slot = var_bch_rv;
        *var_bov_slot = var_bov;
        *var_bov_d_slot = var_bov_d;
        *var_bov_d_rv_slot = var_bov_d_rv;
        *var_bov_rv_slot = var_bov_rv;
        *var_cfrd_i_slot = var_cfrd_i;
        *var_cfrd_i_rv_slot = var_cfrd_i_rv;
        *var_cgidld_i_slot = var_cgidld_i;
        *var_cgidld_i_rv_slot = var_cgidld_i_rv;
        *var_cgovd_i_slot = var_cgovd_i;
        *var_cgovd_i_rv_slot = var_cgovd_i_rv;
        *var_cinrd_i_slot = var_cinrd_i;
        *var_cinrd_i_rv_slot = var_cinrd_i_rv;
        *var_cox_over_q_slot = var_cox_over_q;
        *var_cox_over_q_rv_slot = var_cox_over_q_rv;
        *var_coxovprime_slot = var_coxovprime;
        *var_coxovprime_d_slot = var_coxovprime_d;
        *var_coxovprime_d_rv_slot = var_coxovprime_d_rv;
        *var_coxovprime_rv_slot = var_coxovprime_rv;
        *var_coxprime_slot = var_coxprime;
        *var_coxprime_rv_slot = var_coxprime_rv;
        *var_dxgb_ov_d_slot = var_dxgb_ov_d;
        *var_dxgb_ov_d_rv_slot = var_dxgb_ov_d_rv;
        *var_dxgb_ov_s_slot = var_dxgb_ov_s;
        *var_dxgb_ov_s_rv_slot = var_dxgb_ov_s_rv;
        *var_dxgb_ov_th_slot = var_dxgb_ov_th;
        *var_dxgb_ov_th_rv_slot = var_dxgb_ov_th_rv;
        *var_e_eff0_slot = var_e_eff0;
        *var_e_eff0_rv_slot = var_e_eff0_rv;
        *var_epsox_slot = var_epsox;
        *var_epsox_rv_slot = var_epsox_rv;
        *var_eta_mu_slot = var_eta_mu;
        *var_eta_mu1_slot = var_eta_mu1;
        *var_eta_mu1_rv_slot = var_eta_mu1_rv;
        *var_eta_mu_rv_slot = var_eta_mu_rv;
        *var_fcgovaccd_i_slot = var_fcgovaccd_i;
        *var_fcgovaccd_i_rv_slot = var_fcgovaccd_i_rv;
        *var_gc2ovd_i_slot = var_gc2ovd_i;
        *var_gc2ovd_i_rv_slot = var_gc2ovd_i_rv;
        *var_gc3ovd_i_slot = var_gc3ovd_i;
        *var_gc3ovd_i_rv_slot = var_gc3ovd_i_rv;
        *var_gcq_slot = var_gcq;
        *var_gcq_rv_slot = var_gcq_rv;
        *var_gov2_d_slot = var_gov2_d;
        *var_gov2_d_rv_slot = var_gov2_d_rv;
        *var_gov2_s_slot = var_gov2_s;
        *var_gov2_s_rv_slot = var_gov2_s_rv;
        *var_gov_d_slot = var_gov_d;
        *var_gov_d_rv_slot = var_gov_d_rv;
        *var_gov_s_slot = var_gov_s;
        *var_gov_s_rv_slot = var_gov_s_rv;
        *var_guard151_slot = var_guard151;
        *var_guard151_rv_slot = var_guard151_rv;
        *var_guard152_slot = var_guard152;
        *var_guard152_rv_slot = var_guard152_rv;
        *var_guard153_slot = var_guard153;
        *var_guard153_rv_slot = var_guard153_rv;
        *var_guard154_slot = var_guard154;
        *var_guard154_rv_slot = var_guard154_rv;
        *var_guard155_slot = var_guard155;
        *var_guard155_rv_slot = var_guard155_rv;
        *var_guard156_slot = var_guard156;
        *var_guard156_rv_slot = var_guard156_rv;
        *var_guard157_slot = var_guard157;
        *var_guard157_rv_slot = var_guard157_rv;
        *var_guard158_slot = var_guard158;
        *var_guard158_rv_slot = var_guard158_rv;
        *var_guard159_slot = var_guard159;
        *var_guard159_rv_slot = var_guard159_rv;
        *var_guard160_slot = var_guard160;
        *var_guard160_rv_slot = var_guard160_rv;
        *var_igovd_i_slot = var_igovd_i;
        *var_igovd_i_rv_slot = var_igovd_i_rv;
        *var_inv_chib_slot = var_inv_chib;
        *var_inv_chib_rv_slot = var_inv_chib_rv;
        *var_inv_gov_slot = var_inv_gov;
        *var_inv_gov_rv_slot = var_inv_gov_rv;
        *var_inv_vp_slot = var_inv_vp;
        *var_inv_vp_rv_slot = var_inv_vp_rv;
        *var_neffac_i_slot = var_neffac_i;
        *var_neffac_i_rv_slot = var_neffac_i_rv;
        *var_qq_slot = var_qq;
        *var_qq_rv_slot = var_qq_rv;
        *var_sp_ov_a_d_slot = var_sp_ov_a_d;
        *var_sp_ov_a_d_rv_slot = var_sp_ov_a_d_rv;
        *var_sp_ov_a_s_slot = var_sp_ov_a_s;
        *var_sp_ov_a_s_rv_slot = var_sp_ov_a_s_rv;
        *var_sp_ov_delta_slot = var_sp_ov_delta;
        *var_sp_ov_delta1_d_slot = var_sp_ov_delta1_d;
        *var_sp_ov_delta1_d_rv_slot = var_sp_ov_delta1_d_rv;
        *var_sp_ov_delta1_s_slot = var_sp_ov_delta1_s;
        *var_sp_ov_delta1_s_rv_slot = var_sp_ov_delta1_s_rv;
        *var_sp_ov_delta_rv_slot = var_sp_ov_delta_rv;
        *var_sp_ov_eps_slot = var_sp_ov_eps;
        *var_sp_ov_eps2_d_slot = var_sp_ov_eps2_d;
        *var_sp_ov_eps2_d_rv_slot = var_sp_ov_eps2_d_rv;
        *var_sp_ov_eps2_s_slot = var_sp_ov_eps2_s;
        *var_sp_ov_eps2_s_rv_slot = var_sp_ov_eps2_s_rv;
        *var_sp_ov_eps_rv_slot = var_sp_ov_eps_rv;
        *var_temp_slot = var_temp;
        *var_temp_rv_slot = var_temp_rv;
        *var_tox_sq_slot = var_tox_sq;
        *var_tox_sq_rv_slot = var_tox_sq_rv;
    }

    pub(super) fn stamp_reactive_block_18(
        p: &Parameters,
        var_ad_i: f64,
        var_as_i: f64,
        var_axinr_i: f64,
        var_bgidl_i: f64,
        var_bgidld_i: f64,
        var_delta: f64,
        var_fcinracc_i: f64,
        var_gc2_i: f64,
        var_gc2ov_i: f64,
        var_gc2ovd_i: f64,
        var_gc3_i: f64,
        var_gc3ov_i: f64,
        var_gc3ovd_i: f64,
        var_guard160: f64,
        var_idsatbot: f64,
        var_invnf: f64,
        var_jw_i: f64,
        var_pd_i: f64,
        var_ps_i: f64,
        var_rta: f64,
        var_stbgidl_i: f64,
        var_stbgidld_i: f64,
        var_stig_i: f64,
        var_toxov_i: f64,
        var_toxovd_i: f64,
        var_we: f64,
        var_abd_i_slot: &mut f64,
        var_abd_i_rv_slot: &mut f64,
        var_abdrain_i_slot: &mut f64,
        var_abdrain_i_rv_slot: &mut f64,
        var_abs_i_slot: &mut f64,
        var_abs_i_rv_slot: &mut f64,
        var_absource_i_slot: &mut f64,
        var_absource_i_rv_slot: &mut f64,
        var_ainr_slot: &mut f64,
        var_ainr_rv_slot: &mut f64,
        var_b_fact_slot: &mut f64,
        var_b_fact_rv_slot: &mut f64,
        var_bgidl_t_slot: &mut f64,
        var_bgidl_t_rv_slot: &mut f64,
        var_bgidld_t_slot: &mut f64,
        var_bgidld_t_rv_slot: &mut f64,
        var_bgidlds_slot: &mut f64,
        var_bgidlds_rv_slot: &mut f64,
        var_bgidls_slot: &mut f64,
        var_bgidls_rv_slot: &mut f64,
        var_exp_vmax_over_phitd_d_slot: &mut f64,
        var_exp_vmax_over_phitd_d_rv_slot: &mut f64,
        var_exp_vmax_over_phitd_s_slot: &mut f64,
        var_exp_vmax_over_phitd_s_rv_slot: &mut f64,
        var_gcq_slot: &mut f64,
        var_gcq_rv_slot: &mut f64,
        var_gcqov_slot: &mut f64,
        var_gcqov_rv_slot: &mut f64,
        var_gcqovd_slot: &mut f64,
        var_gcqovd_rv_slot: &mut f64,
        var_guard161_slot: &mut f64,
        var_guard161_rv_slot: &mut f64,
        var_guard162_slot: &mut f64,
        var_guard162_rv_slot: &mut f64,
        var_guard163_slot: &mut f64,
        var_guard163_rv_slot: &mut f64,
        var_guard171_slot: &mut f64,
        var_guard171_rv_slot: &mut f64,
        var_guard172_slot: &mut f64,
        var_guard172_rv_slot: &mut f64,
        var_guard173_slot: &mut f64,
        var_guard173_rv_slot: &mut f64,
        var_guard174_slot: &mut f64,
        var_guard174_rv_slot: &mut f64,
        var_guard175_slot: &mut f64,
        var_guard175_rv_slot: &mut f64,
        var_guard176_slot: &mut f64,
        var_guard176_rv_slot: &mut f64,
        var_iginv_i_slot: &mut f64,
        var_iginv_i_rv_slot: &mut f64,
        var_igov_i_slot: &mut f64,
        var_igov_i_rv_slot: &mut f64,
        var_igovd_i_slot: &mut f64,
        var_igovd_i_rv_slot: &mut f64,
        var_jwcorr_slot: &mut f64,
        var_jwcorr_rv_slot: &mut f64,
        var_jww_slot: &mut f64,
        var_jww_rv_slot: &mut f64,
        var_lgd_i_slot: &mut f64,
        var_lgd_i_rv_slot: &mut f64,
        var_lgdrain_i_slot: &mut f64,
        var_lgdrain_i_rv_slot: &mut f64,
        var_lgs_i_slot: &mut f64,
        var_lgs_i_rv_slot: &mut f64,
        var_lgsource_i_slot: &mut f64,
        var_lgsource_i_rv_slot: &mut f64,
        var_lsd_i_slot: &mut f64,
        var_lsd_i_rv_slot: &mut f64,
        var_lsdrain_i_slot: &mut f64,
        var_lsdrain_i_rv_slot: &mut f64,
        var_lss_i_slot: &mut f64,
        var_lss_i_rv_slot: &mut f64,
        var_lssource_i_slot: &mut f64,
        var_lssource_i_rv_slot: &mut f64,
        var_tf_ig_slot: &mut f64,
        var_tf_ig_rv_slot: &mut f64,
        var_vbbtlim_d_slot: &mut f64,
        var_vbbtlim_d_rv_slot: &mut f64,
        var_vbbtlim_s_slot: &mut f64,
        var_vbbtlim_s_rv_slot: &mut f64,
        var_vbimin_d_slot: &mut f64,
        var_vbimin_d_rv_slot: &mut f64,
        var_vbimin_s_slot: &mut f64,
        var_vbimin_s_rv_slot: &mut f64,
        var_vch_d_slot: &mut f64,
        var_vch_d_rv_slot: &mut f64,
        var_vch_s_slot: &mut f64,
        var_vch_s_rv_slot: &mut f64,
        var_vfmin_d_slot: &mut f64,
        var_vfmin_d_rv_slot: &mut f64,
        var_vfmin_s_slot: &mut f64,
        var_vfmin_s_rv_slot: &mut f64,
        var_vinr_max_slot: &mut f64,
        var_vinr_max_rv_slot: &mut f64,
        var_vmax_d_slot: &mut f64,
        var_vmax_d_rv_slot: &mut f64,
        var_vmax_s_slot: &mut f64,
        var_vmax_s_rv_slot: &mut f64,
        var_zflagbot_d_slot: &mut f64,
        var_zflagbot_d_rv_slot: &mut f64,
        var_zflagbot_s_slot: &mut f64,
        var_zflagbot_s_rv_slot: &mut f64,
        var_zflaggat_d_slot: &mut f64,
        var_zflaggat_d_rv_slot: &mut f64,
        var_zflaggat_s_slot: &mut f64,
        var_zflaggat_s_rv_slot: &mut f64,
        var_zflagsti_d_slot: &mut f64,
        var_zflagsti_d_rv_slot: &mut f64,
        var_zflagsti_s_slot: &mut f64,
        var_zflagsti_s_rv_slot: &mut f64,
        var_zfrac_slot: &mut f64,
        var_zfrac_rv_slot: &mut f64,
    ) {
        let mut var_abd_i: f64 = *var_abd_i_slot;
        let mut var_abd_i_rv: f64 = *var_abd_i_rv_slot;
        let mut var_abdrain_i: f64 = *var_abdrain_i_slot;
        let mut var_abdrain_i_rv: f64 = *var_abdrain_i_rv_slot;
        let mut var_abs_i: f64 = *var_abs_i_slot;
        let mut var_abs_i_rv: f64 = *var_abs_i_rv_slot;
        let mut var_absource_i: f64 = *var_absource_i_slot;
        let mut var_absource_i_rv: f64 = *var_absource_i_rv_slot;
        let mut var_ainr: f64 = *var_ainr_slot;
        let mut var_ainr_rv: f64 = *var_ainr_rv_slot;
        let mut var_b_fact: f64 = *var_b_fact_slot;
        let mut var_b_fact_rv: f64 = *var_b_fact_rv_slot;
        let mut var_bgidl_t: f64 = *var_bgidl_t_slot;
        let mut var_bgidl_t_rv: f64 = *var_bgidl_t_rv_slot;
        let mut var_bgidld_t: f64 = *var_bgidld_t_slot;
        let mut var_bgidld_t_rv: f64 = *var_bgidld_t_rv_slot;
        let mut var_bgidlds: f64 = *var_bgidlds_slot;
        let mut var_bgidlds_rv: f64 = *var_bgidlds_rv_slot;
        let mut var_bgidls: f64 = *var_bgidls_slot;
        let mut var_bgidls_rv: f64 = *var_bgidls_rv_slot;
        let mut var_exp_vmax_over_phitd_d: f64 = *var_exp_vmax_over_phitd_d_slot;
        let mut var_exp_vmax_over_phitd_d_rv: f64 = *var_exp_vmax_over_phitd_d_rv_slot;
        let mut var_exp_vmax_over_phitd_s: f64 = *var_exp_vmax_over_phitd_s_slot;
        let mut var_exp_vmax_over_phitd_s_rv: f64 = *var_exp_vmax_over_phitd_s_rv_slot;
        let mut var_gcq: f64 = *var_gcq_slot;
        let mut var_gcq_rv: f64 = *var_gcq_rv_slot;
        let mut var_gcqov: f64 = *var_gcqov_slot;
        let mut var_gcqov_rv: f64 = *var_gcqov_rv_slot;
        let mut var_gcqovd: f64 = *var_gcqovd_slot;
        let mut var_gcqovd_rv: f64 = *var_gcqovd_rv_slot;
        let mut var_guard161: f64 = *var_guard161_slot;
        let mut var_guard161_rv: f64 = *var_guard161_rv_slot;
        let mut var_guard162: f64 = *var_guard162_slot;
        let mut var_guard162_rv: f64 = *var_guard162_rv_slot;
        let mut var_guard163: f64 = *var_guard163_slot;
        let mut var_guard163_rv: f64 = *var_guard163_rv_slot;
        let mut var_guard171: f64 = *var_guard171_slot;
        let mut var_guard171_rv: f64 = *var_guard171_rv_slot;
        let mut var_guard172: f64 = *var_guard172_slot;
        let mut var_guard172_rv: f64 = *var_guard172_rv_slot;
        let mut var_guard173: f64 = *var_guard173_slot;
        let mut var_guard173_rv: f64 = *var_guard173_rv_slot;
        let mut var_guard174: f64 = *var_guard174_slot;
        let mut var_guard174_rv: f64 = *var_guard174_rv_slot;
        let mut var_guard175: f64 = *var_guard175_slot;
        let mut var_guard175_rv: f64 = *var_guard175_rv_slot;
        let mut var_guard176: f64 = *var_guard176_slot;
        let mut var_guard176_rv: f64 = *var_guard176_rv_slot;
        let mut var_iginv_i: f64 = *var_iginv_i_slot;
        let mut var_iginv_i_rv: f64 = *var_iginv_i_rv_slot;
        let mut var_igov_i: f64 = *var_igov_i_slot;
        let mut var_igov_i_rv: f64 = *var_igov_i_rv_slot;
        let mut var_igovd_i: f64 = *var_igovd_i_slot;
        let mut var_igovd_i_rv: f64 = *var_igovd_i_rv_slot;
        let mut var_jwcorr: f64 = *var_jwcorr_slot;
        let mut var_jwcorr_rv: f64 = *var_jwcorr_rv_slot;
        let mut var_jww: f64 = *var_jww_slot;
        let mut var_jww_rv: f64 = *var_jww_rv_slot;
        let mut var_lgd_i: f64 = *var_lgd_i_slot;
        let mut var_lgd_i_rv: f64 = *var_lgd_i_rv_slot;
        let mut var_lgdrain_i: f64 = *var_lgdrain_i_slot;
        let mut var_lgdrain_i_rv: f64 = *var_lgdrain_i_rv_slot;
        let mut var_lgs_i: f64 = *var_lgs_i_slot;
        let mut var_lgs_i_rv: f64 = *var_lgs_i_rv_slot;
        let mut var_lgsource_i: f64 = *var_lgsource_i_slot;
        let mut var_lgsource_i_rv: f64 = *var_lgsource_i_rv_slot;
        let mut var_lsd_i: f64 = *var_lsd_i_slot;
        let mut var_lsd_i_rv: f64 = *var_lsd_i_rv_slot;
        let mut var_lsdrain_i: f64 = *var_lsdrain_i_slot;
        let mut var_lsdrain_i_rv: f64 = *var_lsdrain_i_rv_slot;
        let mut var_lss_i: f64 = *var_lss_i_slot;
        let mut var_lss_i_rv: f64 = *var_lss_i_rv_slot;
        let mut var_lssource_i: f64 = *var_lssource_i_slot;
        let mut var_lssource_i_rv: f64 = *var_lssource_i_rv_slot;
        let mut var_tf_ig: f64 = *var_tf_ig_slot;
        let mut var_tf_ig_rv: f64 = *var_tf_ig_rv_slot;
        let mut var_vbbtlim_d: f64 = *var_vbbtlim_d_slot;
        let mut var_vbbtlim_d_rv: f64 = *var_vbbtlim_d_rv_slot;
        let mut var_vbbtlim_s: f64 = *var_vbbtlim_s_slot;
        let mut var_vbbtlim_s_rv: f64 = *var_vbbtlim_s_rv_slot;
        let mut var_vbimin_d: f64 = *var_vbimin_d_slot;
        let mut var_vbimin_d_rv: f64 = *var_vbimin_d_rv_slot;
        let mut var_vbimin_s: f64 = *var_vbimin_s_slot;
        let mut var_vbimin_s_rv: f64 = *var_vbimin_s_rv_slot;
        let mut var_vch_d: f64 = *var_vch_d_slot;
        let mut var_vch_d_rv: f64 = *var_vch_d_rv_slot;
        let mut var_vch_s: f64 = *var_vch_s_slot;
        let mut var_vch_s_rv: f64 = *var_vch_s_rv_slot;
        let mut var_vfmin_d: f64 = *var_vfmin_d_slot;
        let mut var_vfmin_d_rv: f64 = *var_vfmin_d_rv_slot;
        let mut var_vfmin_s: f64 = *var_vfmin_s_slot;
        let mut var_vfmin_s_rv: f64 = *var_vfmin_s_rv_slot;
        let mut var_vinr_max: f64 = *var_vinr_max_slot;
        let mut var_vinr_max_rv: f64 = *var_vinr_max_rv_slot;
        let mut var_vmax_d: f64 = *var_vmax_d_slot;
        let mut var_vmax_d_rv: f64 = *var_vmax_d_rv_slot;
        let mut var_vmax_s: f64 = *var_vmax_s_slot;
        let mut var_vmax_s_rv: f64 = *var_vmax_s_rv_slot;
        let mut var_zflagbot_d: f64 = *var_zflagbot_d_slot;
        let mut var_zflagbot_d_rv: f64 = *var_zflagbot_d_rv_slot;
        let mut var_zflagbot_s: f64 = *var_zflagbot_s_slot;
        let mut var_zflagbot_s_rv: f64 = *var_zflagbot_s_rv_slot;
        let mut var_zflaggat_d: f64 = *var_zflaggat_d_slot;
        let mut var_zflaggat_d_rv: f64 = *var_zflaggat_d_rv_slot;
        let mut var_zflaggat_s: f64 = *var_zflaggat_s_slot;
        let mut var_zflaggat_s_rv: f64 = *var_zflaggat_s_rv_slot;
        let mut var_zflagsti_d: f64 = *var_zflagsti_d_slot;
        let mut var_zflagsti_d_rv: f64 = *var_zflagsti_d_rv_slot;
        let mut var_zflagsti_s: f64 = *var_zflagsti_s_slot;
        let mut var_zflagsti_s_rv: f64 = *var_zflagsti_s_rv_slot;
        let mut var_zfrac: f64 = *var_zfrac_slot;
        let mut var_zfrac_rv: f64 = *var_zfrac_rv_slot;

        let (assign11720_e10722,) = {
    if (var_guard160 != 0.0) {
        let assign11720_e10716: f64 = (-0.495);
        let assign11720_e10718: f64 = (assign11720_e10716 * var_gc2_i);
        let assign11720_e10720: f64 = (assign11720_e10718 / var_gc3_i);
        (assign11720_e10720,)
    } else {
        (var_gcq,)
    }
};
        var_gcq = assign11720_e10722;
        var_gcq_rv = 0.0;

        var_gcqov = 0.0;
        var_gcqov_rv = 0.0;

        let assign11740_e10726: f64 = if var_gc3ov_i < 0.0 { 1.0 } else { 0.0 };
        var_guard161 = assign11740_e10726;
        var_guard161_rv = 0.0;

        let (assign11750_e10735,) = {
    if (var_guard161 != 0.0) {
        let assign11750_e10729: f64 = (-0.495);
        let assign11750_e10731: f64 = (assign11750_e10729 * var_gc2ov_i);
        let assign11750_e10733: f64 = (assign11750_e10731 / var_gc3ov_i);
        (assign11750_e10733,)
    } else {
        (var_gcqov,)
    }
};
        var_gcqov = assign11750_e10735;
        var_gcqov_rv = 0.0;

        let assign11760_e10738: f64 = if var_gc3ovd_i < 0.0 { 1.0 } else { 0.0 };
        var_guard162 = assign11760_e10738;
        var_guard162_rv = 0.0;

        let (assign11770_e10747,) = {
    if (var_guard162 != 0.0) {
        let assign11770_e10741: f64 = (-0.495);
        let assign11770_e10743: f64 = (assign11770_e10741 * var_gc2ovd_i);
        let assign11770_e10745: f64 = (assign11770_e10743 / var_gc3ovd_i);
        (assign11770_e10745,)
    } else {
        (var_gcqovd,)
    }
};
        var_gcqovd = assign11770_e10747;
        var_gcqovd_rv = 0.0;

        let assign11780_e10750: f64 = (var_rta).powf(var_stig_i);
        var_tf_ig = assign11780_e10750;
        var_tf_ig_rv = 0.0;

        let assign11790_e10753: f64 = (var_iginv_i * var_tf_ig);
        var_iginv_i = assign11790_e10753;
        var_iginv_i_rv = 0.0;

        let assign11800_e10756: f64 = (var_igov_i * var_tf_ig);
        var_igov_i = assign11800_e10756;
        var_igov_i_rv = 0.0;

        let assign11810_e10759: f64 = (var_igovd_i * var_tf_ig);
        var_igovd_i = assign11810_e10759;
        var_igovd_i_rv = 0.0;

        let assign11840_e10777: f64 = (var_stbgidl_i * var_delta);
        let assign11840_e10778: f64 = (1.0 + assign11840_e10777);
        let (assign11840_e10787,) = {
    if (assign11840_e10778 > 0.0) {
        let assign11840_e10784: f64 = (var_stbgidl_i * var_delta);
        let assign11840_e10785: f64 = (1.0 + assign11840_e10784);
        (assign11840_e10785,)
    } else {
        (0.0,)
    }
};
        var_b_fact = assign11840_e10787;
        var_b_fact_rv = 0.0;

        let assign11850_e10790: f64 = (var_bgidl_i * var_b_fact);
        var_bgidl_t = assign11850_e10790;
        var_bgidl_t_rv = 0.0;

        let assign11860_e10793: f64 = (var_bgidl_t * var_toxov_i);
        let assign11860_e10795: f64 = (assign11860_e10793 * 500000000.0);
        var_bgidls = assign11860_e10795;
        var_bgidls_rv = 0.0;

        let assign11870_e10799: f64 = (var_stbgidld_i * var_delta);
        let assign11870_e10800: f64 = (1.0 + assign11870_e10799);
        let (assign11870_e10809,) = {
    if (assign11870_e10800 > 0.0) {
        let assign11870_e10806: f64 = (var_stbgidld_i * var_delta);
        let assign11870_e10807: f64 = (1.0 + assign11870_e10806);
        (assign11870_e10807,)
    } else {
        (0.0,)
    }
};
        var_b_fact = assign11870_e10809;
        var_b_fact_rv = 0.0;

        let assign11880_e10812: f64 = (var_bgidld_i * var_b_fact);
        var_bgidld_t = assign11880_e10812;
        var_bgidld_t_rv = 0.0;

        let assign11890_e10815: f64 = (var_bgidld_t * var_toxovd_i);
        let assign11890_e10817: f64 = (assign11890_e10815 * 500000000.0);
        var_bgidlds = assign11890_e10817;
        var_bgidlds_rv = 0.0;

        var_vinr_max = 0.0;
        var_vinr_max_rv = 0.0;

        let assign11910_e10821: f64 = if var_fcinracc_i > 1e-10 { 1.0 } else { 0.0 };
        var_guard163 = assign11910_e10821;
        var_guard163_rv = 0.0;

        let (assign11920_e10827,) = {
    if (var_guard163 != 0.0) {
        let assign11920_e10825: f64 = (0.75 / var_fcinracc_i);
        (assign11920_e10825,)
    } else {
        (var_vinr_max,)
    }
};
        var_vinr_max = assign11920_e10827;
        var_vinr_max_rv = 0.0;

        let assign11930_e10830: f64 = (var_axinr_i * var_axinr_i);
        var_ainr = assign11930_e10830;
        var_ainr_rv = 0.0;

        let assign12170_e10941: f64 = (var_absource_i * var_invnf);
        var_abs_i = assign12170_e10941;
        var_abs_i_rv = 0.0;

        let assign12180_e10944: f64 = (var_lssource_i * var_invnf);
        var_lss_i = assign12180_e10944;
        var_lss_i_rv = 0.0;

        let assign12190_e10947: f64 = (var_lgsource_i * var_invnf);
        var_lgs_i = assign12190_e10947;
        var_lgs_i_rv = 0.0;

        let assign12200_e10950: f64 = (var_abdrain_i * var_invnf);
        var_abd_i = assign12200_e10950;
        var_abd_i_rv = 0.0;

        let assign12210_e10953: f64 = (var_lsdrain_i * var_invnf);
        var_lsd_i = assign12210_e10953;
        var_lsd_i_rv = 0.0;

        let assign12220_e10956: f64 = (var_lgdrain_i * var_invnf);
        var_lgd_i = assign12220_e10956;
        var_lgd_i_rv = 0.0;

        var_jwcorr = 0.0;
        var_jwcorr_rv = 0.0;

        let assign12240_e10960: f64 = if p.p43 == 3.0 { 1.0 } else { 0.0 };
        var_guard171 = assign12240_e10960;
        var_guard171_rv = 0.0;

        let (assign12250_e10964,) = {
    if (var_guard171 != 0.0) {
        (1.0,)
    } else {
        (var_jwcorr,)
    }
};
        var_jwcorr = assign12250_e10964;
        var_jwcorr_rv = 0.0;

        var_jww = var_we;
        var_jww_rv = 0.0;

        let assign12270_e10968: f64 = if p.p39 == 0.0 { 1.0 } else { 0.0 };
        var_guard172 = assign12270_e10968;
        var_guard172_rv = 0.0;

        let (assign12280_e10977,) = {
    if (var_guard172 != 0.0) {
        let (assign12280_e10975,) = {
            if (var_jw_i > 0.0) {
                (var_jw_i,)
            } else {
                (0.0,)
            }
        };
        (assign12280_e10975,)
    } else {
        (var_jww,)
    }
};
        var_jww = assign12280_e10977;
        var_jww_rv = 0.0;

        let assign12290_e10984: f64 = if ((p.p43 == 2.0) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };
        var_guard173 = assign12290_e10984;
        var_guard173_rv = 0.0;

        let (assign12300_e10990,) = {
    if (var_guard173 != 0.0) {
        let assign12300_e10988: f64 = (var_as_i * var_invnf);
        (assign12300_e10988,)
    } else {
        (var_abs_i,)
    }
};
        var_abs_i = assign12300_e10990;
        var_abs_i_rv = 0.0;

        let (assign12310_e11000,) = {
    if (var_guard173 != 0.0) {
        let assign12310_e10994: f64 = (var_ps_i * var_invnf);
        let assign12310_e10997: f64 = (var_jwcorr * var_jww);
        let assign12310_e10998: f64 = (assign12310_e10994 - assign12310_e10997);
        (assign12310_e10998,)
    } else {
        (var_lss_i,)
    }
};
        var_lss_i = assign12310_e11000;
        var_lss_i_rv = 0.0;

        let (assign12320_e11004,) = {
    if (var_guard173 != 0.0) {
        (var_jww,)
    } else {
        (var_lgs_i,)
    }
};
        var_lgs_i = assign12320_e11004;
        var_lgs_i_rv = 0.0;

        let (assign12330_e11010,) = {
    if (var_guard173 != 0.0) {
        let assign12330_e11008: f64 = (var_ad_i * var_invnf);
        (assign12330_e11008,)
    } else {
        (var_abd_i,)
    }
};
        var_abd_i = assign12330_e11010;
        var_abd_i_rv = 0.0;

        let (assign12340_e11020,) = {
    if (var_guard173 != 0.0) {
        let assign12340_e11014: f64 = (var_pd_i * var_invnf);
        let assign12340_e11017: f64 = (var_jwcorr * var_jww);
        let assign12340_e11018: f64 = (assign12340_e11014 - assign12340_e11017);
        (assign12340_e11018,)
    } else {
        (var_lsd_i,)
    }
};
        var_lsd_i = assign12340_e11020;
        var_lsd_i_rv = 0.0;

        let (assign12350_e11024,) = {
    if (var_guard173 != 0.0) {
        (var_jww,)
    } else {
        (var_lgd_i,)
    }
};
        var_lgd_i = assign12350_e11024;
        var_lgd_i_rv = 0.0;

        let assign12360_e11035: f64 = if (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };
        var_guard174 = assign12360_e11035;
        var_guard174_rv = 0.0;

        let (assign12370_e11044,) = {
    if (var_guard174 != 0.0) {
        let (assign12370_e11042,) = {
            if (var_abs_i > 0.0) {
                (var_abs_i,)
            } else {
                (0.0,)
            }
        };
        (assign12370_e11042,)
    } else {
        (var_absource_i,)
    }
};
        var_absource_i = assign12370_e11044;
        var_absource_i_rv = 0.0;

        let (assign12380_e11053,) = {
    if (var_guard174 != 0.0) {
        let (assign12380_e11051,) = {
            if (var_lss_i > 0.0) {
                (var_lss_i,)
            } else {
                (0.0,)
            }
        };
        (assign12380_e11051,)
    } else {
        (var_lssource_i,)
    }
};
        var_lssource_i = assign12380_e11053;
        var_lssource_i_rv = 0.0;

        let (assign12390_e11062,) = {
    if (var_guard174 != 0.0) {
        let (assign12390_e11060,) = {
            if (var_lgs_i > 0.0) {
                (var_lgs_i,)
            } else {
                (0.0,)
            }
        };
        (assign12390_e11060,)
    } else {
        (var_lgsource_i,)
    }
};
        var_lgsource_i = assign12390_e11062;
        var_lgsource_i_rv = 0.0;

        let (assign12400_e11071,) = {
    if (var_guard174 != 0.0) {
        let (assign12400_e11069,) = {
            if (var_abd_i > 0.0) {
                (var_abd_i,)
            } else {
                (0.0,)
            }
        };
        (assign12400_e11069,)
    } else {
        (var_abdrain_i,)
    }
};
        var_abdrain_i = assign12400_e11071;
        var_abdrain_i_rv = 0.0;

        let (assign12410_e11080,) = {
    if (var_guard174 != 0.0) {
        let (assign12410_e11078,) = {
            if (var_lsd_i > 0.0) {
                (var_lsd_i,)
            } else {
                (0.0,)
            }
        };
        (assign12410_e11078,)
    } else {
        (var_lsdrain_i,)
    }
};
        var_lsdrain_i = assign12410_e11080;
        var_lsdrain_i_rv = 0.0;

        let (assign12420_e11089,) = {
    if (var_guard174 != 0.0) {
        let (assign12420_e11087,) = {
            if (var_lgd_i > 0.0) {
                (var_lgd_i,)
            } else {
                (0.0,)
            }
        };
        (assign12420_e11087,)
    } else {
        (var_lgdrain_i,)
    }
};
        var_lgdrain_i = assign12420_e11089;
        var_lgdrain_i_rv = 0.0;

        let (assign12430_e11094,) = {
    if (var_guard174 == 0.0) {
        (0.0,)
    } else {
        (var_absource_i,)
    }
};
        var_absource_i = assign12430_e11094;
        var_absource_i_rv = 0.0;

        let (assign12440_e11099,) = {
    if (var_guard174 == 0.0) {
        (0.0,)
    } else {
        (var_lssource_i,)
    }
};
        var_lssource_i = assign12440_e11099;
        var_lssource_i_rv = 0.0;

        let (assign12450_e11104,) = {
    if (var_guard174 == 0.0) {
        (0.0,)
    } else {
        (var_lgsource_i,)
    }
};
        var_lgsource_i = assign12450_e11104;
        var_lgsource_i_rv = 0.0;

        let (assign12460_e11109,) = {
    if (var_guard174 == 0.0) {
        (0.0,)
    } else {
        (var_abdrain_i,)
    }
};
        var_abdrain_i = assign12460_e11109;
        var_abdrain_i_rv = 0.0;

        let (assign12470_e11114,) = {
    if (var_guard174 == 0.0) {
        (0.0,)
    } else {
        (var_lsdrain_i,)
    }
};
        var_lsdrain_i = assign12470_e11114;
        var_lsdrain_i_rv = 0.0;

        let (assign12480_e11119,) = {
    if (var_guard174 == 0.0) {
        (0.0,)
    } else {
        (var_lgdrain_i,)
    }
};
        var_lgdrain_i = assign12480_e11119;
        var_lgdrain_i_rv = 0.0;

        var_vbimin_s = 0.0;
        var_vbimin_s_rv = 0.0;

        var_vbimin_d = 0.0;
        var_vbimin_d_rv = 0.0;

        var_vfmin_s = 0.0;
        var_vfmin_s_rv = 0.0;

        var_vfmin_d = 0.0;
        var_vfmin_d_rv = 0.0;

        var_vch_s = 0.0;
        var_vch_s_rv = 0.0;

        var_vch_d = 0.0;
        var_vch_d_rv = 0.0;

        var_vbbtlim_s = 0.0;
        var_vbbtlim_s_rv = 0.0;

        var_vbbtlim_d = 0.0;
        var_vbbtlim_d_rv = 0.0;

        var_vmax_s = 0.0;
        var_vmax_s_rv = 0.0;

        var_vmax_d = 0.0;
        var_vmax_d_rv = 0.0;

        var_exp_vmax_over_phitd_s = 0.0;
        var_exp_vmax_over_phitd_s_rv = 0.0;

        var_exp_vmax_over_phitd_d = 0.0;
        var_exp_vmax_over_phitd_d_rv = 0.0;

        var_zflagbot_s = 1.0;
        var_zflagbot_s_rv = 0.0;

        var_zflagbot_d = 1.0;
        var_zflagbot_d_rv = 0.0;

        var_zflagsti_s = 1.0;
        var_zflagsti_s_rv = 0.0;

        var_zflagsti_d = 1.0;
        var_zflagsti_d_rv = 0.0;

        var_zflaggat_s = 1.0;
        var_zflaggat_s_rv = 0.0;

        var_zflaggat_d = 1.0;
        var_zflaggat_d_rv = 0.0;

        var_zfrac = 0.0;
        var_zfrac_rv = 0.0;

        let assign13050_e11178: f64 = if p.p43 > 0.0 { 1.0 } else { 0.0 };
        var_guard175 = assign13050_e11178;
        var_guard175_rv = 0.0;

        let assign13060_e11181: f64 = (var_idsatbot * var_absource_i);
        let assign13060_e11183: f64 = if assign13060_e11181 > 0.0 { 1.0 } else { 0.0 };
        var_guard176 = assign13060_e11183;
        var_guard176_rv = 0.0;

        *var_abd_i_slot = var_abd_i;
        *var_abd_i_rv_slot = var_abd_i_rv;
        *var_abdrain_i_slot = var_abdrain_i;
        *var_abdrain_i_rv_slot = var_abdrain_i_rv;
        *var_abs_i_slot = var_abs_i;
        *var_abs_i_rv_slot = var_abs_i_rv;
        *var_absource_i_slot = var_absource_i;
        *var_absource_i_rv_slot = var_absource_i_rv;
        *var_ainr_slot = var_ainr;
        *var_ainr_rv_slot = var_ainr_rv;
        *var_b_fact_slot = var_b_fact;
        *var_b_fact_rv_slot = var_b_fact_rv;
        *var_bgidl_t_slot = var_bgidl_t;
        *var_bgidl_t_rv_slot = var_bgidl_t_rv;
        *var_bgidld_t_slot = var_bgidld_t;
        *var_bgidld_t_rv_slot = var_bgidld_t_rv;
        *var_bgidlds_slot = var_bgidlds;
        *var_bgidlds_rv_slot = var_bgidlds_rv;
        *var_bgidls_slot = var_bgidls;
        *var_bgidls_rv_slot = var_bgidls_rv;
        *var_exp_vmax_over_phitd_d_slot = var_exp_vmax_over_phitd_d;
        *var_exp_vmax_over_phitd_d_rv_slot = var_exp_vmax_over_phitd_d_rv;
        *var_exp_vmax_over_phitd_s_slot = var_exp_vmax_over_phitd_s;
        *var_exp_vmax_over_phitd_s_rv_slot = var_exp_vmax_over_phitd_s_rv;
        *var_gcq_slot = var_gcq;
        *var_gcq_rv_slot = var_gcq_rv;
        *var_gcqov_slot = var_gcqov;
        *var_gcqov_rv_slot = var_gcqov_rv;
        *var_gcqovd_slot = var_gcqovd;
        *var_gcqovd_rv_slot = var_gcqovd_rv;
        *var_guard161_slot = var_guard161;
        *var_guard161_rv_slot = var_guard161_rv;
        *var_guard162_slot = var_guard162;
        *var_guard162_rv_slot = var_guard162_rv;
        *var_guard163_slot = var_guard163;
        *var_guard163_rv_slot = var_guard163_rv;
        *var_guard171_slot = var_guard171;
        *var_guard171_rv_slot = var_guard171_rv;
        *var_guard172_slot = var_guard172;
        *var_guard172_rv_slot = var_guard172_rv;
        *var_guard173_slot = var_guard173;
        *var_guard173_rv_slot = var_guard173_rv;
        *var_guard174_slot = var_guard174;
        *var_guard174_rv_slot = var_guard174_rv;
        *var_guard175_slot = var_guard175;
        *var_guard175_rv_slot = var_guard175_rv;
        *var_guard176_slot = var_guard176;
        *var_guard176_rv_slot = var_guard176_rv;
        *var_iginv_i_slot = var_iginv_i;
        *var_iginv_i_rv_slot = var_iginv_i_rv;
        *var_igov_i_slot = var_igov_i;
        *var_igov_i_rv_slot = var_igov_i_rv;
        *var_igovd_i_slot = var_igovd_i;
        *var_igovd_i_rv_slot = var_igovd_i_rv;
        *var_jwcorr_slot = var_jwcorr;
        *var_jwcorr_rv_slot = var_jwcorr_rv;
        *var_jww_slot = var_jww;
        *var_jww_rv_slot = var_jww_rv;
        *var_lgd_i_slot = var_lgd_i;
        *var_lgd_i_rv_slot = var_lgd_i_rv;
        *var_lgdrain_i_slot = var_lgdrain_i;
        *var_lgdrain_i_rv_slot = var_lgdrain_i_rv;
        *var_lgs_i_slot = var_lgs_i;
        *var_lgs_i_rv_slot = var_lgs_i_rv;
        *var_lgsource_i_slot = var_lgsource_i;
        *var_lgsource_i_rv_slot = var_lgsource_i_rv;
        *var_lsd_i_slot = var_lsd_i;
        *var_lsd_i_rv_slot = var_lsd_i_rv;
        *var_lsdrain_i_slot = var_lsdrain_i;
        *var_lsdrain_i_rv_slot = var_lsdrain_i_rv;
        *var_lss_i_slot = var_lss_i;
        *var_lss_i_rv_slot = var_lss_i_rv;
        *var_lssource_i_slot = var_lssource_i;
        *var_lssource_i_rv_slot = var_lssource_i_rv;
        *var_tf_ig_slot = var_tf_ig;
        *var_tf_ig_rv_slot = var_tf_ig_rv;
        *var_vbbtlim_d_slot = var_vbbtlim_d;
        *var_vbbtlim_d_rv_slot = var_vbbtlim_d_rv;
        *var_vbbtlim_s_slot = var_vbbtlim_s;
        *var_vbbtlim_s_rv_slot = var_vbbtlim_s_rv;
        *var_vbimin_d_slot = var_vbimin_d;
        *var_vbimin_d_rv_slot = var_vbimin_d_rv;
        *var_vbimin_s_slot = var_vbimin_s;
        *var_vbimin_s_rv_slot = var_vbimin_s_rv;
        *var_vch_d_slot = var_vch_d;
        *var_vch_d_rv_slot = var_vch_d_rv;
        *var_vch_s_slot = var_vch_s;
        *var_vch_s_rv_slot = var_vch_s_rv;
        *var_vfmin_d_slot = var_vfmin_d;
        *var_vfmin_d_rv_slot = var_vfmin_d_rv;
        *var_vfmin_s_slot = var_vfmin_s;
        *var_vfmin_s_rv_slot = var_vfmin_s_rv;
        *var_vinr_max_slot = var_vinr_max;
        *var_vinr_max_rv_slot = var_vinr_max_rv;
        *var_vmax_d_slot = var_vmax_d;
        *var_vmax_d_rv_slot = var_vmax_d_rv;
        *var_vmax_s_slot = var_vmax_s;
        *var_vmax_s_rv_slot = var_vmax_s_rv;
        *var_zflagbot_d_slot = var_zflagbot_d;
        *var_zflagbot_d_rv_slot = var_zflagbot_d_rv;
        *var_zflagbot_s_slot = var_zflagbot_s;
        *var_zflagbot_s_rv_slot = var_zflagbot_s_rv;
        *var_zflaggat_d_slot = var_zflaggat_d;
        *var_zflaggat_d_rv_slot = var_zflaggat_d_rv;
        *var_zflaggat_s_slot = var_zflaggat_s;
        *var_zflaggat_s_rv_slot = var_zflaggat_s_rv;
        *var_zflagsti_d_slot = var_zflagsti_d;
        *var_zflagsti_d_rv_slot = var_zflagsti_d_rv;
        *var_zflagsti_s_slot = var_zflagsti_s;
        *var_zflagsti_s_rv_slot = var_zflagsti_s_rv;
        *var_zfrac_slot = var_zfrac;
        *var_zfrac_rv_slot = var_zfrac_rv;
    }

    pub(super) fn stamp_reactive_block_19(
        p: &Parameters,
        var_abdrain_i: f64,
        var_absource_i: f64,
        var_guard175: f64,
        var_guard176: f64,
        var_idsatbot: f64,
        var_idsatbot_d: f64,
        var_idsatgat: f64,
        var_idsatgat_d: f64,
        var_idsatsti: f64,
        var_idsatsti_d: f64,
        var_lgdrain_i: f64,
        var_lgsource_i: f64,
        var_lsdrain_i: f64,
        var_lssource_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_vbibot: f64,
        var_vbigat: f64,
        var_vbisti: f64,
        var_exp_vmax_over_phitd_s_slot: &mut f64,
        var_exp_vmax_over_phitd_s_rv_slot: &mut f64,
        var_guard177_slot: &mut f64,
        var_guard177_rv_slot: &mut f64,
        var_guard178_slot: &mut f64,
        var_guard178_rv_slot: &mut f64,
        var_guard179_slot: &mut f64,
        var_guard179_rv_slot: &mut f64,
        var_guard180_slot: &mut f64,
        var_guard180_rv_slot: &mut f64,
        var_guard181_slot: &mut f64,
        var_guard181_rv_slot: &mut f64,
        var_guard182_slot: &mut f64,
        var_guard182_rv_slot: &mut f64,
        var_guard183_slot: &mut f64,
        var_guard183_rv_slot: &mut f64,
        var_guard184_slot: &mut f64,
        var_guard184_rv_slot: &mut f64,
        var_guard185_slot: &mut f64,
        var_guard185_rv_slot: &mut f64,
        var_guard186_slot: &mut f64,
        var_guard186_rv_slot: &mut f64,
        var_pbot2_slot: &mut f64,
        var_pbot2_rv_slot: &mut f64,
        var_pgat2_slot: &mut f64,
        var_pgat2_rv_slot: &mut f64,
        var_pmax_slot: &mut f64,
        var_pmax_rv_slot: &mut f64,
        var_psti2_slot: &mut f64,
        var_psti2_rv_slot: &mut f64,
        var_vbbtlim_s_slot: &mut f64,
        var_vbbtlim_s_rv_slot: &mut f64,
        var_vbibot2_slot: &mut f64,
        var_vbibot2_rv_slot: &mut f64,
        var_vbibot2r_slot: &mut f64,
        var_vbibot2r_rv_slot: &mut f64,
        var_vbigat2_slot: &mut f64,
        var_vbigat2_rv_slot: &mut f64,
        var_vbigat2r_slot: &mut f64,
        var_vbigat2r_rv_slot: &mut f64,
        var_vbimin_s_slot: &mut f64,
        var_vbimin_s_rv_slot: &mut f64,
        var_vbisti2_slot: &mut f64,
        var_vbisti2_rv_slot: &mut f64,
        var_vbisti2r_slot: &mut f64,
        var_vbisti2r_rv_slot: &mut f64,
        var_vch_s_slot: &mut f64,
        var_vch_s_rv_slot: &mut f64,
        var_vfmin_s_slot: &mut f64,
        var_vfmin_s_rv_slot: &mut f64,
        var_vmax_s_slot: &mut f64,
        var_vmax_s_rv_slot: &mut f64,
        var_vmaxbot_slot: &mut f64,
        var_vmaxbot_rv_slot: &mut f64,
        var_vmaxgat_slot: &mut f64,
        var_vmaxgat_rv_slot: &mut f64,
        var_vmaxsti_slot: &mut f64,
        var_vmaxsti_rv_slot: &mut f64,
    ) {
        let mut var_exp_vmax_over_phitd_s: f64 = *var_exp_vmax_over_phitd_s_slot;
        let mut var_exp_vmax_over_phitd_s_rv: f64 = *var_exp_vmax_over_phitd_s_rv_slot;
        let mut var_guard177: f64 = *var_guard177_slot;
        let mut var_guard177_rv: f64 = *var_guard177_rv_slot;
        let mut var_guard178: f64 = *var_guard178_slot;
        let mut var_guard178_rv: f64 = *var_guard178_rv_slot;
        let mut var_guard179: f64 = *var_guard179_slot;
        let mut var_guard179_rv: f64 = *var_guard179_rv_slot;
        let mut var_guard180: f64 = *var_guard180_slot;
        let mut var_guard180_rv: f64 = *var_guard180_rv_slot;
        let mut var_guard181: f64 = *var_guard181_slot;
        let mut var_guard181_rv: f64 = *var_guard181_rv_slot;
        let mut var_guard182: f64 = *var_guard182_slot;
        let mut var_guard182_rv: f64 = *var_guard182_rv_slot;
        let mut var_guard183: f64 = *var_guard183_slot;
        let mut var_guard183_rv: f64 = *var_guard183_rv_slot;
        let mut var_guard184: f64 = *var_guard184_slot;
        let mut var_guard184_rv: f64 = *var_guard184_rv_slot;
        let mut var_guard185: f64 = *var_guard185_slot;
        let mut var_guard185_rv: f64 = *var_guard185_rv_slot;
        let mut var_guard186: f64 = *var_guard186_slot;
        let mut var_guard186_rv: f64 = *var_guard186_rv_slot;
        let mut var_pbot2: f64 = *var_pbot2_slot;
        let mut var_pbot2_rv: f64 = *var_pbot2_rv_slot;
        let mut var_pgat2: f64 = *var_pgat2_slot;
        let mut var_pgat2_rv: f64 = *var_pgat2_rv_slot;
        let mut var_pmax: f64 = *var_pmax_slot;
        let mut var_pmax_rv: f64 = *var_pmax_rv_slot;
        let mut var_psti2: f64 = *var_psti2_slot;
        let mut var_psti2_rv: f64 = *var_psti2_rv_slot;
        let mut var_vbbtlim_s: f64 = *var_vbbtlim_s_slot;
        let mut var_vbbtlim_s_rv: f64 = *var_vbbtlim_s_rv_slot;
        let mut var_vbibot2: f64 = *var_vbibot2_slot;
        let mut var_vbibot2_rv: f64 = *var_vbibot2_rv_slot;
        let mut var_vbibot2r: f64 = *var_vbibot2r_slot;
        let mut var_vbibot2r_rv: f64 = *var_vbibot2r_rv_slot;
        let mut var_vbigat2: f64 = *var_vbigat2_slot;
        let mut var_vbigat2_rv: f64 = *var_vbigat2_rv_slot;
        let mut var_vbigat2r: f64 = *var_vbigat2r_slot;
        let mut var_vbigat2r_rv: f64 = *var_vbigat2r_rv_slot;
        let mut var_vbimin_s: f64 = *var_vbimin_s_slot;
        let mut var_vbimin_s_rv: f64 = *var_vbimin_s_rv_slot;
        let mut var_vbisti2: f64 = *var_vbisti2_slot;
        let mut var_vbisti2_rv: f64 = *var_vbisti2_rv_slot;
        let mut var_vbisti2r: f64 = *var_vbisti2r_slot;
        let mut var_vbisti2r_rv: f64 = *var_vbisti2r_rv_slot;
        let mut var_vch_s: f64 = *var_vch_s_slot;
        let mut var_vch_s_rv: f64 = *var_vch_s_rv_slot;
        let mut var_vfmin_s: f64 = *var_vfmin_s_slot;
        let mut var_vfmin_s_rv: f64 = *var_vfmin_s_rv_slot;
        let mut var_vmax_s: f64 = *var_vmax_s_slot;
        let mut var_vmax_s_rv: f64 = *var_vmax_s_rv_slot;
        let mut var_vmaxbot: f64 = *var_vmaxbot_slot;
        let mut var_vmaxbot_rv: f64 = *var_vmaxbot_rv_slot;
        let mut var_vmaxgat: f64 = *var_vmaxgat_slot;
        let mut var_vmaxgat_rv: f64 = *var_vmaxgat_rv_slot;
        let mut var_vmaxsti: f64 = *var_vmaxsti_slot;
        let mut var_vmaxsti_rv: f64 = *var_vmaxsti_rv_slot;

        let (assign13070_e11198,) = {
    if ((var_guard175 != 0.0) && (var_guard176 != 0.0)) {
        let assign13070_e11191: f64 = (var_idsatbot * var_absource_i);
        let assign13070_e11192: f64 = (p.p839 / assign13070_e11191);
        let assign13070_e11194: f64 = (assign13070_e11192 + 1.0);
        let assign13070_e11195: f64 = (assign13070_e11194).ln();
        let assign13070_e11196: f64 = (var_phitd * assign13070_e11195);
        (assign13070_e11196,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign13070_e11198;
        var_vmaxbot_rv = 0.0;

        let (assign13080_e11205,) = {
    if ((var_guard175 != 0.0) && (var_guard176 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign13080_e11205;
        var_vmaxbot_rv = 0.0;

        let assign13090_e11208: f64 = (var_idsatsti * var_lssource_i);
        let assign13090_e11210: f64 = if assign13090_e11208 > 0.0 { 1.0 } else { 0.0 };
        var_guard177 = assign13090_e11210;
        var_guard177_rv = 0.0;

        let (assign13100_e11225,) = {
    if ((var_guard175 != 0.0) && (var_guard177 != 0.0)) {
        let assign13100_e11218: f64 = (var_idsatsti * var_lssource_i);
        let assign13100_e11219: f64 = (p.p839 / assign13100_e11218);
        let assign13100_e11221: f64 = (assign13100_e11219 + 1.0);
        let assign13100_e11222: f64 = (assign13100_e11221).ln();
        let assign13100_e11223: f64 = (var_phitd * assign13100_e11222);
        (assign13100_e11223,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign13100_e11225;
        var_vmaxsti_rv = 0.0;

        let (assign13110_e11232,) = {
    if ((var_guard175 != 0.0) && (var_guard177 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign13110_e11232;
        var_vmaxsti_rv = 0.0;

        let assign13120_e11235: f64 = (var_idsatgat * var_lgsource_i);
        let assign13120_e11237: f64 = if assign13120_e11235 > 0.0 { 1.0 } else { 0.0 };
        var_guard178 = assign13120_e11237;
        var_guard178_rv = 0.0;

        let (assign13130_e11252,) = {
    if ((var_guard175 != 0.0) && (var_guard178 != 0.0)) {
        let assign13130_e11245: f64 = (var_idsatgat * var_lgsource_i);
        let assign13130_e11246: f64 = (p.p839 / assign13130_e11245);
        let assign13130_e11248: f64 = (assign13130_e11246 + 1.0);
        let assign13130_e11249: f64 = (assign13130_e11248).ln();
        let assign13130_e11250: f64 = (var_phitd * assign13130_e11249);
        (assign13130_e11250,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign13130_e11252;
        var_vmaxgat_rv = 0.0;

        let (assign13140_e11259,) = {
    if ((var_guard175 != 0.0) && (var_guard178 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign13140_e11259;
        var_vmaxgat_rv = 0.0;

        let (assign13150_e11267,) = {
    if (var_guard175 != 0.0) {
        let assign13150_e11263: f64 = (var_vmaxbot).min(var_vmaxsti);
        let assign13150_e11265: f64 = (assign13150_e11263).min(var_vmaxgat);
        (assign13150_e11265,)
    } else {
        (var_vmax_s,)
    }
};
        var_vmax_s = assign13150_e11267;
        var_vmax_s_rv = 0.0;

        let assign13160_e11270: f64 = (var_vmax_s * var_phitdinv);
        let assign13160_e11271: f64 = (assign13160_e11270).abs();
        let assign13160_e11273: f64 = if assign13160_e11271 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard179 = assign13160_e11273;
        var_guard179_rv = 0.0;

        let (assign13170_e11282,) = {
    if ((var_guard175 != 0.0) && (var_guard179 != 0.0)) {
        let assign13170_e11279: f64 = (var_vmax_s * var_phitdinv);
        let assign13170_e11280: f64 = (assign13170_e11279).exp();
        (assign13170_e11280,)
    } else {
        (var_exp_vmax_over_phitd_s,)
    }
};
        var_exp_vmax_over_phitd_s = assign13170_e11282;
        var_exp_vmax_over_phitd_s_rv = 0.0;

        let assign13180_e11285: f64 = (var_vmax_s * var_phitdinv);
        let assign13180_e11287: f64 = if assign13180_e11285 < 0.0 { 1.0 } else { 0.0 };
        var_guard180 = assign13180_e11287;
        var_guard180_rv = 0.0;

        let (assign13190_e11327,) = {
    if (((var_guard175 != 0.0) && (var_guard179 == 0.0)) && (var_guard180 != 0.0)) {
        let assign13190_e11297: f64 = (-230.25850929940458);
        let assign13190_e11300: f64 = (var_vmax_s * var_phitdinv);
        let assign13190_e11301: f64 = (assign13190_e11297 - assign13190_e11300);
        let assign13190_e11305: f64 = (-230.25850929940458);
        let assign13190_e11308: f64 = (var_vmax_s * var_phitdinv);
        let assign13190_e11309: f64 = (assign13190_e11305 - assign13190_e11308);
        let assign13190_e11312: f64 = (-230.25850929940458);
        let assign13190_e11315: f64 = (var_vmax_s * var_phitdinv);
        let assign13190_e11316: f64 = (assign13190_e11312 - assign13190_e11315);
        let assign13190_e11318: f64 = (assign13190_e11316 * 0.3333333333333333);
        let assign13190_e11319: f64 = (1.0 + assign13190_e11318);
        let assign13190_e11320: f64 = (assign13190_e11309 * assign13190_e11319);
        let assign13190_e11321: f64 = (0.5 * assign13190_e11320);
        let assign13190_e11322: f64 = (1.0 + assign13190_e11321);
        let assign13190_e11323: f64 = (assign13190_e11301 * assign13190_e11322);
        let assign13190_e11324: f64 = (1.0 + assign13190_e11323);
        let assign13190_e11325: f64 = (1e-100 / assign13190_e11324);
        (assign13190_e11325,)
    } else {
        (var_exp_vmax_over_phitd_s,)
    }
};
        var_exp_vmax_over_phitd_s = assign13190_e11327;
        var_exp_vmax_over_phitd_s_rv = 0.0;

        let (assign13200_e11365,) = {
    if (((var_guard175 != 0.0) && (var_guard179 == 0.0)) && (var_guard180 == 0.0)) {
        let assign13200_e11339: f64 = (var_vmax_s * var_phitdinv);
        let assign13200_e11341: f64 = (assign13200_e11339 - 230.25850929940458);
        let assign13200_e11346: f64 = (var_vmax_s * var_phitdinv);
        let assign13200_e11348: f64 = (assign13200_e11346 - 230.25850929940458);
        let assign13200_e11352: f64 = (var_vmax_s * var_phitdinv);
        let assign13200_e11354: f64 = (assign13200_e11352 - 230.25850929940458);
        let assign13200_e11356: f64 = (assign13200_e11354 * 0.3333333333333333);
        let assign13200_e11357: f64 = (1.0 + assign13200_e11356);
        let assign13200_e11358: f64 = (assign13200_e11348 * assign13200_e11357);
        let assign13200_e11359: f64 = (0.5 * assign13200_e11358);
        let assign13200_e11360: f64 = (1.0 + assign13200_e11359);
        let assign13200_e11361: f64 = (assign13200_e11341 * assign13200_e11360);
        let assign13200_e11362: f64 = (1.0 + assign13200_e11361);
        let assign13200_e11363: f64 = (1e100 * assign13200_e11362);
        (assign13200_e11363,)
    } else {
        (var_exp_vmax_over_phitd_s,)
    }
};
        var_exp_vmax_over_phitd_s = assign13200_e11365;
        var_exp_vmax_over_phitd_s_rv = 0.0;

        let (assign13210_e11369,) = {
    if (var_guard175 != 0.0) {
        (var_vbibot,)
    } else {
        (var_vbibot2,)
    }
};
        var_vbibot2 = assign13210_e11369;
        var_vbibot2_rv = 0.0;

        let (assign13220_e11373,) = {
    if (var_guard175 != 0.0) {
        (var_vbisti,)
    } else {
        (var_vbisti2,)
    }
};
        var_vbisti2 = assign13220_e11373;
        var_vbisti2_rv = 0.0;

        let (assign13230_e11377,) = {
    if (var_guard175 != 0.0) {
        (var_vbigat,)
    } else {
        (var_vbigat2,)
    }
};
        var_vbigat2 = assign13230_e11377;
        var_vbigat2_rv = 0.0;

        let (assign13240_e11381,) = {
    if (var_guard175 != 0.0) {
        (p.p848,)
    } else {
        (var_pbot2,)
    }
};
        var_pbot2 = assign13240_e11381;
        var_pbot2_rv = 0.0;

        let (assign13250_e11385,) = {
    if (var_guard175 != 0.0) {
        (p.p849,)
    } else {
        (var_psti2,)
    }
};
        var_psti2 = assign13250_e11385;
        var_psti2_rv = 0.0;

        let (assign13260_e11389,) = {
    if (var_guard175 != 0.0) {
        (p.p850,)
    } else {
        (var_pgat2,)
    }
};
        var_pgat2 = assign13260_e11389;
        var_pgat2_rv = 0.0;

        let (assign13270_e11393,) = {
    if (var_guard175 != 0.0) {
        (p.p845,)
    } else {
        (var_vbibot2r,)
    }
};
        var_vbibot2r = assign13270_e11393;
        var_vbibot2r_rv = 0.0;

        let (assign13280_e11397,) = {
    if (var_guard175 != 0.0) {
        (p.p846,)
    } else {
        (var_vbisti2r,)
    }
};
        var_vbisti2r = assign13280_e11397;
        var_vbisti2r_rv = 0.0;

        let (assign13290_e11401,) = {
    if (var_guard175 != 0.0) {
        (p.p847,)
    } else {
        (var_vbigat2r,)
    }
};
        var_vbigat2r = assign13290_e11401;
        var_vbigat2r_rv = 0.0;

        let assign13300_e11404: f64 = if var_absource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard181 = assign13300_e11404;
        var_guard181_rv = 0.0;

        let (assign13310_e11412,) = {
    if ((var_guard175 != 0.0) && (var_guard181 != 0.0)) {
        let assign13310_e11410: f64 = (var_vbisti + var_vbigat);
        (assign13310_e11410,)
    } else {
        (var_vbibot2,)
    }
};
        var_vbibot2 = assign13310_e11412;
        var_vbibot2_rv = 0.0;

        let (assign13320_e11422,) = {
    if ((var_guard175 != 0.0) && (var_guard181 != 0.0)) {
        let assign13320_e11419: f64 = (p.p849).min(p.p850);
        let assign13320_e11420: f64 = (0.9 * assign13320_e11419);
        (assign13320_e11420,)
    } else {
        (var_pbot2,)
    }
};
        var_pbot2 = assign13320_e11422;
        var_pbot2_rv = 0.0;

        let (assign13330_e11430,) = {
    if ((var_guard175 != 0.0) && (var_guard181 != 0.0)) {
        let assign13330_e11428: f64 = (p.p846 + p.p847);
        (assign13330_e11428,)
    } else {
        (var_vbibot2r,)
    }
};
        var_vbibot2r = assign13330_e11430;
        var_vbibot2r_rv = 0.0;

        let assign13340_e11433: f64 = if var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard182 = assign13340_e11433;
        var_guard182_rv = 0.0;

        let (assign13350_e11441,) = {
    if ((var_guard175 != 0.0) && (var_guard182 != 0.0)) {
        let assign13350_e11439: f64 = (var_vbibot + var_vbigat);
        (assign13350_e11439,)
    } else {
        (var_vbisti2,)
    }
};
        var_vbisti2 = assign13350_e11441;
        var_vbisti2_rv = 0.0;

        let (assign13360_e11451,) = {
    if ((var_guard175 != 0.0) && (var_guard182 != 0.0)) {
        let assign13360_e11448: f64 = (p.p848).min(p.p850);
        let assign13360_e11449: f64 = (0.9 * assign13360_e11448);
        (assign13360_e11449,)
    } else {
        (var_psti2,)
    }
};
        var_psti2 = assign13360_e11451;
        var_psti2_rv = 0.0;

        let (assign13370_e11459,) = {
    if ((var_guard175 != 0.0) && (var_guard182 != 0.0)) {
        let assign13370_e11457: f64 = (p.p845 + p.p847);
        (assign13370_e11457,)
    } else {
        (var_vbisti2r,)
    }
};
        var_vbisti2r = assign13370_e11459;
        var_vbisti2r_rv = 0.0;

        let assign13380_e11462: f64 = if var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard183 = assign13380_e11462;
        var_guard183_rv = 0.0;

        let (assign13390_e11470,) = {
    if ((var_guard175 != 0.0) && (var_guard183 != 0.0)) {
        let assign13390_e11468: f64 = (var_vbibot + var_vbisti);
        (assign13390_e11468,)
    } else {
        (var_vbigat2,)
    }
};
        var_vbigat2 = assign13390_e11470;
        var_vbigat2_rv = 0.0;

        let (assign13400_e11480,) = {
    if ((var_guard175 != 0.0) && (var_guard183 != 0.0)) {
        let assign13400_e11477: f64 = (p.p848).min(p.p849);
        let assign13400_e11478: f64 = (0.9 * assign13400_e11477);
        (assign13400_e11478,)
    } else {
        (var_pgat2,)
    }
};
        var_pgat2 = assign13400_e11480;
        var_pgat2_rv = 0.0;

        let (assign13410_e11488,) = {
    if ((var_guard175 != 0.0) && (var_guard183 != 0.0)) {
        let assign13410_e11486: f64 = (p.p845 + p.p846);
        (assign13410_e11486,)
    } else {
        (var_vbigat2r,)
    }
};
        var_vbigat2r = assign13410_e11488;
        var_vbigat2r_rv = 0.0;

        let (assign13420_e11496,) = {
    if (var_guard175 != 0.0) {
        let assign13420_e11492: f64 = (var_vbibot2).min(var_vbisti2);
        let assign13420_e11494: f64 = (assign13420_e11492).min(var_vbigat2);
        (assign13420_e11494,)
    } else {
        (var_vbimin_s,)
    }
};
        var_vbimin_s = assign13420_e11496;
        var_vbimin_s_rv = 0.0;

        let (assign13430_e11502,) = {
    if (var_guard175 != 0.0) {
        let assign13430_e11500: f64 = (var_vbimin_s * 0.1);
        (assign13430_e11500,)
    } else {
        (var_vch_s,)
    }
};
        var_vch_s = assign13430_e11502;
        var_vch_s_rv = 0.0;

        let (assign13440_e11510,) = {
    if (var_guard175 != 0.0) {
        let assign13440_e11506: f64 = (var_pbot2).max(var_psti2);
        let assign13440_e11508: f64 = (assign13440_e11506).max(var_pgat2);
        (assign13440_e11508,)
    } else {
        (var_pmax,)
    }
};
        var_pmax = assign13440_e11510;
        var_pmax_rv = 0.0;

        let (assign13450_e11523,) = {
    if (var_guard175 != 0.0) {
        let assign13450_e11516: f64 = (-1.0);
        let assign13450_e11518: f64 = (assign13450_e11516 / var_pmax);
        let assign13450_e11519: f64 = (2.0_f64).powf(assign13450_e11518);
        let assign13450_e11520: f64 = (1.0 - assign13450_e11519);
        let assign13450_e11521: f64 = (var_vbimin_s * assign13450_e11520);
        (assign13450_e11521,)
    } else {
        (var_vfmin_s,)
    }
};
        var_vfmin_s = assign13450_e11523;
        var_vfmin_s_rv = 0.0;

        let (assign13460_e11533,) = {
    if (var_guard175 != 0.0) {
        let assign13460_e11527: f64 = (var_vbibot2r).min(var_vbisti2r);
        let assign13460_e11529: f64 = (assign13460_e11527).min(var_vbigat2r);
        let assign13460_e11531: f64 = (assign13460_e11529 - 0.05);
        (assign13460_e11531,)
    } else {
        (var_vbbtlim_s,)
    }
};
        var_vbbtlim_s = assign13460_e11533;
        var_vbbtlim_s_rv = 0.0;

        let assign13470_e11536: f64 = (var_idsatbot_d * var_abdrain_i);
        let assign13470_e11538: f64 = if assign13470_e11536 > 0.0 { 1.0 } else { 0.0 };
        var_guard184 = assign13470_e11538;
        var_guard184_rv = 0.0;

        let (assign13480_e11553,) = {
    if ((var_guard175 != 0.0) && (var_guard184 != 0.0)) {
        let assign13480_e11546: f64 = (var_idsatbot_d * var_abdrain_i);
        let assign13480_e11547: f64 = (p.p839 / assign13480_e11546);
        let assign13480_e11549: f64 = (assign13480_e11547 + 1.0);
        let assign13480_e11550: f64 = (assign13480_e11549).ln();
        let assign13480_e11551: f64 = (var_phitd * assign13480_e11550);
        (assign13480_e11551,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign13480_e11553;
        var_vmaxbot_rv = 0.0;

        let (assign13490_e11560,) = {
    if ((var_guard175 != 0.0) && (var_guard184 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign13490_e11560;
        var_vmaxbot_rv = 0.0;

        let assign13500_e11563: f64 = (var_idsatsti_d * var_lsdrain_i);
        let assign13500_e11565: f64 = if assign13500_e11563 > 0.0 { 1.0 } else { 0.0 };
        var_guard185 = assign13500_e11565;
        var_guard185_rv = 0.0;

        let (assign13510_e11580,) = {
    if ((var_guard175 != 0.0) && (var_guard185 != 0.0)) {
        let assign13510_e11573: f64 = (var_idsatsti_d * var_lsdrain_i);
        let assign13510_e11574: f64 = (p.p839 / assign13510_e11573);
        let assign13510_e11576: f64 = (assign13510_e11574 + 1.0);
        let assign13510_e11577: f64 = (assign13510_e11576).ln();
        let assign13510_e11578: f64 = (var_phitd * assign13510_e11577);
        (assign13510_e11578,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign13510_e11580;
        var_vmaxsti_rv = 0.0;

        let (assign13520_e11587,) = {
    if ((var_guard175 != 0.0) && (var_guard185 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign13520_e11587;
        var_vmaxsti_rv = 0.0;

        let assign13530_e11590: f64 = (var_idsatgat_d * var_lgdrain_i);
        let assign13530_e11592: f64 = if assign13530_e11590 > 0.0 { 1.0 } else { 0.0 };
        var_guard186 = assign13530_e11592;
        var_guard186_rv = 0.0;

        *var_exp_vmax_over_phitd_s_slot = var_exp_vmax_over_phitd_s;
        *var_exp_vmax_over_phitd_s_rv_slot = var_exp_vmax_over_phitd_s_rv;
        *var_guard177_slot = var_guard177;
        *var_guard177_rv_slot = var_guard177_rv;
        *var_guard178_slot = var_guard178;
        *var_guard178_rv_slot = var_guard178_rv;
        *var_guard179_slot = var_guard179;
        *var_guard179_rv_slot = var_guard179_rv;
        *var_guard180_slot = var_guard180;
        *var_guard180_rv_slot = var_guard180_rv;
        *var_guard181_slot = var_guard181;
        *var_guard181_rv_slot = var_guard181_rv;
        *var_guard182_slot = var_guard182;
        *var_guard182_rv_slot = var_guard182_rv;
        *var_guard183_slot = var_guard183;
        *var_guard183_rv_slot = var_guard183_rv;
        *var_guard184_slot = var_guard184;
        *var_guard184_rv_slot = var_guard184_rv;
        *var_guard185_slot = var_guard185;
        *var_guard185_rv_slot = var_guard185_rv;
        *var_guard186_slot = var_guard186;
        *var_guard186_rv_slot = var_guard186_rv;
        *var_pbot2_slot = var_pbot2;
        *var_pbot2_rv_slot = var_pbot2_rv;
        *var_pgat2_slot = var_pgat2;
        *var_pgat2_rv_slot = var_pgat2_rv;
        *var_pmax_slot = var_pmax;
        *var_pmax_rv_slot = var_pmax_rv;
        *var_psti2_slot = var_psti2;
        *var_psti2_rv_slot = var_psti2_rv;
        *var_vbbtlim_s_slot = var_vbbtlim_s;
        *var_vbbtlim_s_rv_slot = var_vbbtlim_s_rv;
        *var_vbibot2_slot = var_vbibot2;
        *var_vbibot2_rv_slot = var_vbibot2_rv;
        *var_vbibot2r_slot = var_vbibot2r;
        *var_vbibot2r_rv_slot = var_vbibot2r_rv;
        *var_vbigat2_slot = var_vbigat2;
        *var_vbigat2_rv_slot = var_vbigat2_rv;
        *var_vbigat2r_slot = var_vbigat2r;
        *var_vbigat2r_rv_slot = var_vbigat2r_rv;
        *var_vbimin_s_slot = var_vbimin_s;
        *var_vbimin_s_rv_slot = var_vbimin_s_rv;
        *var_vbisti2_slot = var_vbisti2;
        *var_vbisti2_rv_slot = var_vbisti2_rv;
        *var_vbisti2r_slot = var_vbisti2r;
        *var_vbisti2r_rv_slot = var_vbisti2r_rv;
        *var_vch_s_slot = var_vch_s;
        *var_vch_s_rv_slot = var_vch_s_rv;
        *var_vfmin_s_slot = var_vfmin_s;
        *var_vfmin_s_rv_slot = var_vfmin_s_rv;
        *var_vmax_s_slot = var_vmax_s;
        *var_vmax_s_rv_slot = var_vmax_s_rv;
        *var_vmaxbot_slot = var_vmaxbot;
        *var_vmaxbot_rv_slot = var_vmaxbot_rv;
        *var_vmaxgat_slot = var_vmaxgat;
        *var_vmaxgat_rv_slot = var_vmaxgat_rv;
        *var_vmaxsti_slot = var_vmaxsti;
        *var_vmaxsti_rv_slot = var_vmaxsti_rv;
    }

    pub(super) fn stamp_reactive_block_20(
        p: &Parameters,
        var_abdrain_i: f64,
        var_absource_i: f64,
        var_cjobot: f64,
        var_cjobot_d: f64,
        var_cjogat: f64,
        var_cjogat_d: f64,
        var_cjosti: f64,
        var_cjosti_d: f64,
        var_fjunqd_i: f64,
        var_guard175: f64,
        var_guard186: f64,
        var_idsatgat_d: f64,
        var_lgdrain_i: f64,
        var_lgsource_i: f64,
        var_lsdrain_i: f64,
        var_lssource_i: f64,
        var_pbotd_i: f64,
        var_pgatd_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_pstid_i: f64,
        var_swjunexp_i: f64,
        var_vbibot_d: f64,
        var_vbigat_d: f64,
        var_vbirbotd_i: f64,
        var_vbirgatd_i: f64,
        var_vbirstid_i: f64,
        var_vbisti_d: f64,
        var_vmaxbot: f64,
        var_vmaxsti: f64,
        var_exp_vmax_over_phitd_d_slot: &mut f64,
        var_exp_vmax_over_phitd_d_rv_slot: &mut f64,
        var_guard187_slot: &mut f64,
        var_guard187_rv_slot: &mut f64,
        var_guard188_slot: &mut f64,
        var_guard188_rv_slot: &mut f64,
        var_guard189_slot: &mut f64,
        var_guard189_rv_slot: &mut f64,
        var_guard190_slot: &mut f64,
        var_guard190_rv_slot: &mut f64,
        var_guard191_slot: &mut f64,
        var_guard191_rv_slot: &mut f64,
        var_guard192_slot: &mut f64,
        var_guard192_rv_slot: &mut f64,
        var_guard527_slot: &mut f64,
        var_guard527_rv_slot: &mut f64,
        var_guard528_slot: &mut f64,
        var_guard528_rv_slot: &mut f64,
        var_guard529_slot: &mut f64,
        var_guard529_rv_slot: &mut f64,
        var_guard817_slot: &mut f64,
        var_guard817_rv_slot: &mut f64,
        var_guard818_slot: &mut f64,
        var_guard818_rv_slot: &mut f64,
        var_guard819_slot: &mut f64,
        var_guard819_rv_slot: &mut f64,
        var_pbot2_slot: &mut f64,
        var_pbot2_rv_slot: &mut f64,
        var_pgat2_slot: &mut f64,
        var_pgat2_rv_slot: &mut f64,
        var_pmax_slot: &mut f64,
        var_pmax_rv_slot: &mut f64,
        var_psti2_slot: &mut f64,
        var_psti2_rv_slot: &mut f64,
        var_temp__blk949_slot: &mut f64,
        var_temp__blk949_dn4_slot: &mut f64,
        var_temp__blk949_dn6_slot: &mut f64,
        var_temp__blk949_dn7_slot: &mut f64,
        var_temp__blk949_dn8_slot: &mut f64,
        var_temp__blk949_dn9_slot: &mut f64,
        var_temp__blk949_rv_slot: &mut f64,
        var_vbbtlim_d_slot: &mut f64,
        var_vbbtlim_d_rv_slot: &mut f64,
        var_vbibot2_slot: &mut f64,
        var_vbibot2_rv_slot: &mut f64,
        var_vbibot2r_slot: &mut f64,
        var_vbibot2r_rv_slot: &mut f64,
        var_vbigat2_slot: &mut f64,
        var_vbigat2_rv_slot: &mut f64,
        var_vbigat2r_slot: &mut f64,
        var_vbigat2r_rv_slot: &mut f64,
        var_vbimin_d_slot: &mut f64,
        var_vbimin_d_rv_slot: &mut f64,
        var_vbisti2_slot: &mut f64,
        var_vbisti2_rv_slot: &mut f64,
        var_vbisti2r_slot: &mut f64,
        var_vbisti2r_rv_slot: &mut f64,
        var_vch_d_slot: &mut f64,
        var_vch_d_rv_slot: &mut f64,
        var_vfmin_d_slot: &mut f64,
        var_vfmin_d_rv_slot: &mut f64,
        var_vmax_d_slot: &mut f64,
        var_vmax_d_rv_slot: &mut f64,
        var_vmaxgat_slot: &mut f64,
        var_vmaxgat_rv_slot: &mut f64,
        var_zflagbot_d_slot: &mut f64,
        var_zflagbot_d_rv_slot: &mut f64,
        var_zflagbot_s_slot: &mut f64,
        var_zflagbot_s_rv_slot: &mut f64,
        var_zflaggat_d_slot: &mut f64,
        var_zflaggat_d_rv_slot: &mut f64,
        var_zflaggat_s_slot: &mut f64,
        var_zflaggat_s_rv_slot: &mut f64,
        var_zflagsti_d_slot: &mut f64,
        var_zflagsti_d_rv_slot: &mut f64,
        var_zflagsti_s_slot: &mut f64,
        var_zflagsti_s_rv_slot: &mut f64,
        var_zfrac_slot: &mut f64,
        var_zfrac_rv_slot: &mut f64,
    ) {
        let mut var_exp_vmax_over_phitd_d: f64 = *var_exp_vmax_over_phitd_d_slot;
        let mut var_exp_vmax_over_phitd_d_rv: f64 = *var_exp_vmax_over_phitd_d_rv_slot;
        let mut var_guard187: f64 = *var_guard187_slot;
        let mut var_guard187_rv: f64 = *var_guard187_rv_slot;
        let mut var_guard188: f64 = *var_guard188_slot;
        let mut var_guard188_rv: f64 = *var_guard188_rv_slot;
        let mut var_guard189: f64 = *var_guard189_slot;
        let mut var_guard189_rv: f64 = *var_guard189_rv_slot;
        let mut var_guard190: f64 = *var_guard190_slot;
        let mut var_guard190_rv: f64 = *var_guard190_rv_slot;
        let mut var_guard191: f64 = *var_guard191_slot;
        let mut var_guard191_rv: f64 = *var_guard191_rv_slot;
        let mut var_guard192: f64 = *var_guard192_slot;
        let mut var_guard192_rv: f64 = *var_guard192_rv_slot;
        let mut var_guard527: f64 = *var_guard527_slot;
        let mut var_guard527_rv: f64 = *var_guard527_rv_slot;
        let mut var_guard528: f64 = *var_guard528_slot;
        let mut var_guard528_rv: f64 = *var_guard528_rv_slot;
        let mut var_guard529: f64 = *var_guard529_slot;
        let mut var_guard529_rv: f64 = *var_guard529_rv_slot;
        let mut var_guard817: f64 = *var_guard817_slot;
        let mut var_guard817_rv: f64 = *var_guard817_rv_slot;
        let mut var_guard818: f64 = *var_guard818_slot;
        let mut var_guard818_rv: f64 = *var_guard818_rv_slot;
        let mut var_guard819: f64 = *var_guard819_slot;
        let mut var_guard819_rv: f64 = *var_guard819_rv_slot;
        let mut var_pbot2: f64 = *var_pbot2_slot;
        let mut var_pbot2_rv: f64 = *var_pbot2_rv_slot;
        let mut var_pgat2: f64 = *var_pgat2_slot;
        let mut var_pgat2_rv: f64 = *var_pgat2_rv_slot;
        let mut var_pmax: f64 = *var_pmax_slot;
        let mut var_pmax_rv: f64 = *var_pmax_rv_slot;
        let mut var_psti2: f64 = *var_psti2_slot;
        let mut var_psti2_rv: f64 = *var_psti2_rv_slot;
        let mut var_temp__blk949: f64 = *var_temp__blk949_slot;
        let mut var_temp__blk949_dn4: f64 = *var_temp__blk949_dn4_slot;
        let mut var_temp__blk949_dn6: f64 = *var_temp__blk949_dn6_slot;
        let mut var_temp__blk949_dn7: f64 = *var_temp__blk949_dn7_slot;
        let mut var_temp__blk949_dn8: f64 = *var_temp__blk949_dn8_slot;
        let mut var_temp__blk949_dn9: f64 = *var_temp__blk949_dn9_slot;
        let mut var_temp__blk949_rv: f64 = *var_temp__blk949_rv_slot;
        let mut var_vbbtlim_d: f64 = *var_vbbtlim_d_slot;
        let mut var_vbbtlim_d_rv: f64 = *var_vbbtlim_d_rv_slot;
        let mut var_vbibot2: f64 = *var_vbibot2_slot;
        let mut var_vbibot2_rv: f64 = *var_vbibot2_rv_slot;
        let mut var_vbibot2r: f64 = *var_vbibot2r_slot;
        let mut var_vbibot2r_rv: f64 = *var_vbibot2r_rv_slot;
        let mut var_vbigat2: f64 = *var_vbigat2_slot;
        let mut var_vbigat2_rv: f64 = *var_vbigat2_rv_slot;
        let mut var_vbigat2r: f64 = *var_vbigat2r_slot;
        let mut var_vbigat2r_rv: f64 = *var_vbigat2r_rv_slot;
        let mut var_vbimin_d: f64 = *var_vbimin_d_slot;
        let mut var_vbimin_d_rv: f64 = *var_vbimin_d_rv_slot;
        let mut var_vbisti2: f64 = *var_vbisti2_slot;
        let mut var_vbisti2_rv: f64 = *var_vbisti2_rv_slot;
        let mut var_vbisti2r: f64 = *var_vbisti2r_slot;
        let mut var_vbisti2r_rv: f64 = *var_vbisti2r_rv_slot;
        let mut var_vch_d: f64 = *var_vch_d_slot;
        let mut var_vch_d_rv: f64 = *var_vch_d_rv_slot;
        let mut var_vfmin_d: f64 = *var_vfmin_d_slot;
        let mut var_vfmin_d_rv: f64 = *var_vfmin_d_rv_slot;
        let mut var_vmax_d: f64 = *var_vmax_d_slot;
        let mut var_vmax_d_rv: f64 = *var_vmax_d_rv_slot;
        let mut var_vmaxgat: f64 = *var_vmaxgat_slot;
        let mut var_vmaxgat_rv: f64 = *var_vmaxgat_rv_slot;
        let mut var_zflagbot_d: f64 = *var_zflagbot_d_slot;
        let mut var_zflagbot_d_rv: f64 = *var_zflagbot_d_rv_slot;
        let mut var_zflagbot_s: f64 = *var_zflagbot_s_slot;
        let mut var_zflagbot_s_rv: f64 = *var_zflagbot_s_rv_slot;
        let mut var_zflaggat_d: f64 = *var_zflaggat_d_slot;
        let mut var_zflaggat_d_rv: f64 = *var_zflaggat_d_rv_slot;
        let mut var_zflaggat_s: f64 = *var_zflaggat_s_slot;
        let mut var_zflaggat_s_rv: f64 = *var_zflaggat_s_rv_slot;
        let mut var_zflagsti_d: f64 = *var_zflagsti_d_slot;
        let mut var_zflagsti_d_rv: f64 = *var_zflagsti_d_rv_slot;
        let mut var_zflagsti_s: f64 = *var_zflagsti_s_slot;
        let mut var_zflagsti_s_rv: f64 = *var_zflagsti_s_rv_slot;
        let mut var_zfrac: f64 = *var_zfrac_slot;
        let mut var_zfrac_rv: f64 = *var_zfrac_rv_slot;

        let (assign13540_e11607,) = {
    if ((var_guard175 != 0.0) && (var_guard186 != 0.0)) {
        let assign13540_e11600: f64 = (var_idsatgat_d * var_lgdrain_i);
        let assign13540_e11601: f64 = (p.p839 / assign13540_e11600);
        let assign13540_e11603: f64 = (assign13540_e11601 + 1.0);
        let assign13540_e11604: f64 = (assign13540_e11603).ln();
        let assign13540_e11605: f64 = (var_phitd * assign13540_e11604);
        (assign13540_e11605,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign13540_e11607;
        var_vmaxgat_rv = 0.0;

        let (assign13550_e11614,) = {
    if ((var_guard175 != 0.0) && (var_guard186 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign13550_e11614;
        var_vmaxgat_rv = 0.0;

        let (assign13560_e11622,) = {
    if (var_guard175 != 0.0) {
        let assign13560_e11618: f64 = (var_vmaxbot).min(var_vmaxsti);
        let assign13560_e11620: f64 = (assign13560_e11618).min(var_vmaxgat);
        (assign13560_e11620,)
    } else {
        (var_vmax_d,)
    }
};
        var_vmax_d = assign13560_e11622;
        var_vmax_d_rv = 0.0;

        let assign13570_e11625: f64 = (var_vmax_d * var_phitdinv);
        let assign13570_e11626: f64 = (assign13570_e11625).abs();
        let assign13570_e11628: f64 = if assign13570_e11626 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard187 = assign13570_e11628;
        var_guard187_rv = 0.0;

        let (assign13580_e11637,) = {
    if ((var_guard175 != 0.0) && (var_guard187 != 0.0)) {
        let assign13580_e11634: f64 = (var_vmax_d * var_phitdinv);
        let assign13580_e11635: f64 = (assign13580_e11634).exp();
        (assign13580_e11635,)
    } else {
        (var_exp_vmax_over_phitd_d,)
    }
};
        var_exp_vmax_over_phitd_d = assign13580_e11637;
        var_exp_vmax_over_phitd_d_rv = 0.0;

        let assign13590_e11640: f64 = (var_vmax_d * var_phitdinv);
        let assign13590_e11642: f64 = if assign13590_e11640 < 0.0 { 1.0 } else { 0.0 };
        var_guard188 = assign13590_e11642;
        var_guard188_rv = 0.0;

        let (assign13600_e11682,) = {
    if (((var_guard175 != 0.0) && (var_guard187 == 0.0)) && (var_guard188 != 0.0)) {
        let assign13600_e11652: f64 = (-230.25850929940458);
        let assign13600_e11655: f64 = (var_vmax_d * var_phitdinv);
        let assign13600_e11656: f64 = (assign13600_e11652 - assign13600_e11655);
        let assign13600_e11660: f64 = (-230.25850929940458);
        let assign13600_e11663: f64 = (var_vmax_d * var_phitdinv);
        let assign13600_e11664: f64 = (assign13600_e11660 - assign13600_e11663);
        let assign13600_e11667: f64 = (-230.25850929940458);
        let assign13600_e11670: f64 = (var_vmax_d * var_phitdinv);
        let assign13600_e11671: f64 = (assign13600_e11667 - assign13600_e11670);
        let assign13600_e11673: f64 = (assign13600_e11671 * 0.3333333333333333);
        let assign13600_e11674: f64 = (1.0 + assign13600_e11673);
        let assign13600_e11675: f64 = (assign13600_e11664 * assign13600_e11674);
        let assign13600_e11676: f64 = (0.5 * assign13600_e11675);
        let assign13600_e11677: f64 = (1.0 + assign13600_e11676);
        let assign13600_e11678: f64 = (assign13600_e11656 * assign13600_e11677);
        let assign13600_e11679: f64 = (1.0 + assign13600_e11678);
        let assign13600_e11680: f64 = (1e-100 / assign13600_e11679);
        (assign13600_e11680,)
    } else {
        (var_exp_vmax_over_phitd_d,)
    }
};
        var_exp_vmax_over_phitd_d = assign13600_e11682;
        var_exp_vmax_over_phitd_d_rv = 0.0;

        let (assign13610_e11720,) = {
    if (((var_guard175 != 0.0) && (var_guard187 == 0.0)) && (var_guard188 == 0.0)) {
        let assign13610_e11694: f64 = (var_vmax_d * var_phitdinv);
        let assign13610_e11696: f64 = (assign13610_e11694 - 230.25850929940458);
        let assign13610_e11701: f64 = (var_vmax_d * var_phitdinv);
        let assign13610_e11703: f64 = (assign13610_e11701 - 230.25850929940458);
        let assign13610_e11707: f64 = (var_vmax_d * var_phitdinv);
        let assign13610_e11709: f64 = (assign13610_e11707 - 230.25850929940458);
        let assign13610_e11711: f64 = (assign13610_e11709 * 0.3333333333333333);
        let assign13610_e11712: f64 = (1.0 + assign13610_e11711);
        let assign13610_e11713: f64 = (assign13610_e11703 * assign13610_e11712);
        let assign13610_e11714: f64 = (0.5 * assign13610_e11713);
        let assign13610_e11715: f64 = (1.0 + assign13610_e11714);
        let assign13610_e11716: f64 = (assign13610_e11696 * assign13610_e11715);
        let assign13610_e11717: f64 = (1.0 + assign13610_e11716);
        let assign13610_e11718: f64 = (1e100 * assign13610_e11717);
        (assign13610_e11718,)
    } else {
        (var_exp_vmax_over_phitd_d,)
    }
};
        var_exp_vmax_over_phitd_d = assign13610_e11720;
        var_exp_vmax_over_phitd_d_rv = 0.0;

        let (assign13620_e11724,) = {
    if (var_guard175 != 0.0) {
        (var_vbibot_d,)
    } else {
        (var_vbibot2,)
    }
};
        var_vbibot2 = assign13620_e11724;
        var_vbibot2_rv = 0.0;

        let (assign13630_e11728,) = {
    if (var_guard175 != 0.0) {
        (var_vbisti_d,)
    } else {
        (var_vbisti2,)
    }
};
        var_vbisti2 = assign13630_e11728;
        var_vbisti2_rv = 0.0;

        let (assign13640_e11732,) = {
    if (var_guard175 != 0.0) {
        (var_vbigat_d,)
    } else {
        (var_vbigat2,)
    }
};
        var_vbigat2 = assign13640_e11732;
        var_vbigat2_rv = 0.0;

        let (assign13650_e11736,) = {
    if (var_guard175 != 0.0) {
        (var_pbotd_i,)
    } else {
        (var_pbot2,)
    }
};
        var_pbot2 = assign13650_e11736;
        var_pbot2_rv = 0.0;

        let (assign13660_e11740,) = {
    if (var_guard175 != 0.0) {
        (var_pstid_i,)
    } else {
        (var_psti2,)
    }
};
        var_psti2 = assign13660_e11740;
        var_psti2_rv = 0.0;

        let (assign13670_e11744,) = {
    if (var_guard175 != 0.0) {
        (var_pgatd_i,)
    } else {
        (var_pgat2,)
    }
};
        var_pgat2 = assign13670_e11744;
        var_pgat2_rv = 0.0;

        let (assign13680_e11748,) = {
    if (var_guard175 != 0.0) {
        (var_vbirbotd_i,)
    } else {
        (var_vbibot2r,)
    }
};
        var_vbibot2r = assign13680_e11748;
        var_vbibot2r_rv = 0.0;

        let (assign13690_e11752,) = {
    if (var_guard175 != 0.0) {
        (var_vbirstid_i,)
    } else {
        (var_vbisti2r,)
    }
};
        var_vbisti2r = assign13690_e11752;
        var_vbisti2r_rv = 0.0;

        let (assign13700_e11756,) = {
    if (var_guard175 != 0.0) {
        (var_vbirgatd_i,)
    } else {
        (var_vbigat2r,)
    }
};
        var_vbigat2r = assign13700_e11756;
        var_vbigat2r_rv = 0.0;

        let assign13710_e11759: f64 = if var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard189 = assign13710_e11759;
        var_guard189_rv = 0.0;

        let (assign13720_e11767,) = {
    if ((var_guard175 != 0.0) && (var_guard189 != 0.0)) {
        let assign13720_e11765: f64 = (var_vbisti_d + var_vbigat_d);
        (assign13720_e11765,)
    } else {
        (var_vbibot2,)
    }
};
        var_vbibot2 = assign13720_e11767;
        var_vbibot2_rv = 0.0;

        let (assign13730_e11777,) = {
    if ((var_guard175 != 0.0) && (var_guard189 != 0.0)) {
        let assign13730_e11774: f64 = (var_pstid_i).min(var_pgatd_i);
        let assign13730_e11775: f64 = (0.9 * assign13730_e11774);
        (assign13730_e11775,)
    } else {
        (var_pbot2,)
    }
};
        var_pbot2 = assign13730_e11777;
        var_pbot2_rv = 0.0;

        let (assign13740_e11785,) = {
    if ((var_guard175 != 0.0) && (var_guard189 != 0.0)) {
        let assign13740_e11783: f64 = (var_vbirstid_i + var_vbirgatd_i);
        (assign13740_e11783,)
    } else {
        (var_vbibot2r,)
    }
};
        var_vbibot2r = assign13740_e11785;
        var_vbibot2r_rv = 0.0;

        let assign13750_e11788: f64 = if var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard190 = assign13750_e11788;
        var_guard190_rv = 0.0;

        let (assign13760_e11796,) = {
    if ((var_guard175 != 0.0) && (var_guard190 != 0.0)) {
        let assign13760_e11794: f64 = (var_vbibot_d + var_vbigat_d);
        (assign13760_e11794,)
    } else {
        (var_vbisti2,)
    }
};
        var_vbisti2 = assign13760_e11796;
        var_vbisti2_rv = 0.0;

        let (assign13770_e11806,) = {
    if ((var_guard175 != 0.0) && (var_guard190 != 0.0)) {
        let assign13770_e11803: f64 = (var_pbotd_i).min(var_pgatd_i);
        let assign13770_e11804: f64 = (0.9 * assign13770_e11803);
        (assign13770_e11804,)
    } else {
        (var_psti2,)
    }
};
        var_psti2 = assign13770_e11806;
        var_psti2_rv = 0.0;

        let (assign13780_e11814,) = {
    if ((var_guard175 != 0.0) && (var_guard190 != 0.0)) {
        let assign13780_e11812: f64 = (var_vbirbotd_i + var_vbirgatd_i);
        (assign13780_e11812,)
    } else {
        (var_vbisti2r,)
    }
};
        var_vbisti2r = assign13780_e11814;
        var_vbisti2r_rv = 0.0;

        let assign13790_e11817: f64 = if var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard191 = assign13790_e11817;
        var_guard191_rv = 0.0;

        let (assign13800_e11825,) = {
    if ((var_guard175 != 0.0) && (var_guard191 != 0.0)) {
        let assign13800_e11823: f64 = (var_vbibot_d + var_vbisti_d);
        (assign13800_e11823,)
    } else {
        (var_vbigat2,)
    }
};
        var_vbigat2 = assign13800_e11825;
        var_vbigat2_rv = 0.0;

        let (assign13810_e11835,) = {
    if ((var_guard175 != 0.0) && (var_guard191 != 0.0)) {
        let assign13810_e11832: f64 = (var_pbotd_i).min(var_pstid_i);
        let assign13810_e11833: f64 = (0.9 * assign13810_e11832);
        (assign13810_e11833,)
    } else {
        (var_pgat2,)
    }
};
        var_pgat2 = assign13810_e11835;
        var_pgat2_rv = 0.0;

        let (assign13820_e11843,) = {
    if ((var_guard175 != 0.0) && (var_guard191 != 0.0)) {
        let assign13820_e11841: f64 = (var_vbirbotd_i + var_vbirstid_i);
        (assign13820_e11841,)
    } else {
        (var_vbigat2r,)
    }
};
        var_vbigat2r = assign13820_e11843;
        var_vbigat2r_rv = 0.0;

        let (assign13830_e11851,) = {
    if (var_guard175 != 0.0) {
        let assign13830_e11847: f64 = (var_vbibot2).min(var_vbisti2);
        let assign13830_e11849: f64 = (assign13830_e11847).min(var_vbigat2);
        (assign13830_e11849,)
    } else {
        (var_vbimin_d,)
    }
};
        var_vbimin_d = assign13830_e11851;
        var_vbimin_d_rv = 0.0;

        let (assign13840_e11857,) = {
    if (var_guard175 != 0.0) {
        let assign13840_e11855: f64 = (var_vbimin_d * 0.1);
        (assign13840_e11855,)
    } else {
        (var_vch_d,)
    }
};
        var_vch_d = assign13840_e11857;
        var_vch_d_rv = 0.0;

        let (assign13850_e11865,) = {
    if (var_guard175 != 0.0) {
        let assign13850_e11861: f64 = (var_pbot2).max(var_psti2);
        let assign13850_e11863: f64 = (assign13850_e11861).max(var_pgat2);
        (assign13850_e11863,)
    } else {
        (var_pmax,)
    }
};
        var_pmax = assign13850_e11865;
        var_pmax_rv = 0.0;

        let (assign13860_e11878,) = {
    if (var_guard175 != 0.0) {
        let assign13860_e11871: f64 = (-1.0);
        let assign13860_e11873: f64 = (assign13860_e11871 / var_pmax);
        let assign13860_e11874: f64 = (2.0_f64).powf(assign13860_e11873);
        let assign13860_e11875: f64 = (1.0 - assign13860_e11874);
        let assign13860_e11876: f64 = (var_vbimin_d * assign13860_e11875);
        (assign13860_e11876,)
    } else {
        (var_vfmin_d,)
    }
};
        var_vfmin_d = assign13860_e11878;
        var_vfmin_d_rv = 0.0;

        let (assign13870_e11888,) = {
    if (var_guard175 != 0.0) {
        let assign13870_e11882: f64 = (var_vbibot2r).min(var_vbisti2r);
        let assign13870_e11884: f64 = (assign13870_e11882).min(var_vbigat2r);
        let assign13870_e11886: f64 = (assign13870_e11884 - 0.05);
        (assign13870_e11886,)
    } else {
        (var_vbbtlim_d,)
    }
};
        var_vbbtlim_d = assign13870_e11888;
        var_vbbtlim_d_rv = 0.0;

        let assign13880_e11891: f64 = if var_swjunexp_i == 1.0 { 1.0 } else { 0.0 };
        var_guard192 = assign13880_e11891;
        var_guard192_rv = 0.0;

        let (assign26700_e32412,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign26700_e32401: f64 = (var_absource_i * var_cjobot);
        let assign26700_e32404: f64 = (var_lssource_i * var_cjosti);
        let assign26700_e32405: f64 = (assign26700_e32401 + assign26700_e32404);
        let assign26700_e32408: f64 = (var_lgsource_i * var_cjogat);
        let assign26700_e32409: f64 = (assign26700_e32405 + assign26700_e32408);
        let assign26700_e32410: f64 = (p.p946 * assign26700_e32409);
        (assign26700_e32410,)
    } else {
        (var_zfrac,)
    }
};
        var_zfrac = assign26700_e32412;
        var_zfrac_rv = 0.0;

        let assign26710_e32415: f64 = (var_absource_i * var_cjobot);
        let assign26710_e32417: f64 = if assign26710_e32415 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard527 = assign26710_e32417;
        var_guard527_rv = 0.0;

        let (assign26720_e32425,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard527 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagbot_s,)
    }
};
        var_zflagbot_s = assign26720_e32425;
        var_zflagbot_s_rv = 0.0;

        let assign26730_e32428: f64 = (var_lssource_i * var_cjosti);
        let assign26730_e32430: f64 = if assign26730_e32428 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard528 = assign26730_e32430;
        var_guard528_rv = 0.0;

        let (assign26740_e32438,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard528 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagsti_s,)
    }
};
        var_zflagsti_s = assign26740_e32438;
        var_zflagsti_s_rv = 0.0;

        let assign26750_e32441: f64 = (var_lgsource_i * var_cjogat);
        let assign26750_e32443: f64 = if assign26750_e32441 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard529 = assign26750_e32443;
        var_guard529_rv = 0.0;

        let (assign26760_e32451,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard529 != 0.0)) {
        (0.0,)
    } else {
        (var_zflaggat_s,)
    }
};
        var_zflaggat_s = assign26760_e32451;
        var_zflaggat_s_rv = 0.0;

        let (assign39230_e52805,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign39230_e52794: f64 = (var_abdrain_i * var_cjobot_d);
        let assign39230_e52797: f64 = (var_lsdrain_i * var_cjosti_d);
        let assign39230_e52798: f64 = (assign39230_e52794 + assign39230_e52797);
        let assign39230_e52801: f64 = (var_lgdrain_i * var_cjogat_d);
        let assign39230_e52802: f64 = (assign39230_e52798 + assign39230_e52801);
        let assign39230_e52803: f64 = (var_fjunqd_i * assign39230_e52802);
        (assign39230_e52803,)
    } else {
        (var_zfrac,)
    }
};
        var_zfrac = assign39230_e52805;
        var_zfrac_rv = 0.0;

        let assign39240_e52808: f64 = (var_abdrain_i * var_cjobot_d);
        let assign39240_e52810: f64 = if assign39240_e52808 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard817 = assign39240_e52810;
        var_guard817_rv = 0.0;

        let (assign39250_e52818,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard817 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagbot_d,)
    }
};
        var_zflagbot_d = assign39250_e52818;
        var_zflagbot_d_rv = 0.0;

        let assign39260_e52821: f64 = (var_lsdrain_i * var_cjosti_d);
        let assign39260_e52823: f64 = if assign39260_e52821 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard818 = assign39260_e52823;
        var_guard818_rv = 0.0;

        let (assign39270_e52831,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard818 != 0.0)) {
        (0.0,)
    } else {
        (var_zflagsti_d,)
    }
};
        var_zflagsti_d = assign39270_e52831;
        var_zflagsti_d_rv = 0.0;

        let assign39280_e52834: f64 = (var_lgdrain_i * var_cjogat_d);
        let assign39280_e52836: f64 = if assign39280_e52834 <= var_zfrac { 1.0 } else { 0.0 };
        var_guard819 = assign39280_e52836;
        var_guard819_rv = 0.0;

        let (assign39290_e52844,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard819 != 0.0)) {
        (0.0,)
    } else {
        (var_zflaggat_d,)
    }
};
        var_zflaggat_d = assign39290_e52844;
        var_zflaggat_d_rv = 0.0;

        var_temp__blk949 = 0.0;
        var_temp__blk949_dn4 = 0.0;
        var_temp__blk949_dn6 = 0.0;
        var_temp__blk949_dn7 = 0.0;
        var_temp__blk949_dn8 = 0.0;
        var_temp__blk949_dn9 = 0.0;
        var_temp__blk949_rv = 0.0;

        *var_exp_vmax_over_phitd_d_slot = var_exp_vmax_over_phitd_d;
        *var_exp_vmax_over_phitd_d_rv_slot = var_exp_vmax_over_phitd_d_rv;
        *var_guard187_slot = var_guard187;
        *var_guard187_rv_slot = var_guard187_rv;
        *var_guard188_slot = var_guard188;
        *var_guard188_rv_slot = var_guard188_rv;
        *var_guard189_slot = var_guard189;
        *var_guard189_rv_slot = var_guard189_rv;
        *var_guard190_slot = var_guard190;
        *var_guard190_rv_slot = var_guard190_rv;
        *var_guard191_slot = var_guard191;
        *var_guard191_rv_slot = var_guard191_rv;
        *var_guard192_slot = var_guard192;
        *var_guard192_rv_slot = var_guard192_rv;
        *var_guard527_slot = var_guard527;
        *var_guard527_rv_slot = var_guard527_rv;
        *var_guard528_slot = var_guard528;
        *var_guard528_rv_slot = var_guard528_rv;
        *var_guard529_slot = var_guard529;
        *var_guard529_rv_slot = var_guard529_rv;
        *var_guard817_slot = var_guard817;
        *var_guard817_rv_slot = var_guard817_rv;
        *var_guard818_slot = var_guard818;
        *var_guard818_rv_slot = var_guard818_rv;
        *var_guard819_slot = var_guard819;
        *var_guard819_rv_slot = var_guard819_rv;
        *var_pbot2_slot = var_pbot2;
        *var_pbot2_rv_slot = var_pbot2_rv;
        *var_pgat2_slot = var_pgat2;
        *var_pgat2_rv_slot = var_pgat2_rv;
        *var_pmax_slot = var_pmax;
        *var_pmax_rv_slot = var_pmax_rv;
        *var_psti2_slot = var_psti2;
        *var_psti2_rv_slot = var_psti2_rv;
        *var_temp__blk949_slot = var_temp__blk949;
        *var_temp__blk949_dn4_slot = var_temp__blk949_dn4;
        *var_temp__blk949_dn6_slot = var_temp__blk949_dn6;
        *var_temp__blk949_dn7_slot = var_temp__blk949_dn7;
        *var_temp__blk949_dn8_slot = var_temp__blk949_dn8;
        *var_temp__blk949_dn9_slot = var_temp__blk949_dn9;
        *var_temp__blk949_rv_slot = var_temp__blk949_rv;
        *var_vbbtlim_d_slot = var_vbbtlim_d;
        *var_vbbtlim_d_rv_slot = var_vbbtlim_d_rv;
        *var_vbibot2_slot = var_vbibot2;
        *var_vbibot2_rv_slot = var_vbibot2_rv;
        *var_vbibot2r_slot = var_vbibot2r;
        *var_vbibot2r_rv_slot = var_vbibot2r_rv;
        *var_vbigat2_slot = var_vbigat2;
        *var_vbigat2_rv_slot = var_vbigat2_rv;
        *var_vbigat2r_slot = var_vbigat2r;
        *var_vbigat2r_rv_slot = var_vbigat2r_rv;
        *var_vbimin_d_slot = var_vbimin_d;
        *var_vbimin_d_rv_slot = var_vbimin_d_rv;
        *var_vbisti2_slot = var_vbisti2;
        *var_vbisti2_rv_slot = var_vbisti2_rv;
        *var_vbisti2r_slot = var_vbisti2r;
        *var_vbisti2r_rv_slot = var_vbisti2r_rv;
        *var_vch_d_slot = var_vch_d;
        *var_vch_d_rv_slot = var_vch_d_rv;
        *var_vfmin_d_slot = var_vfmin_d;
        *var_vfmin_d_rv_slot = var_vfmin_d_rv;
        *var_vmax_d_slot = var_vmax_d;
        *var_vmax_d_rv_slot = var_vmax_d_rv;
        *var_vmaxgat_slot = var_vmaxgat;
        *var_vmaxgat_rv_slot = var_vmaxgat_rv;
        *var_zflagbot_d_slot = var_zflagbot_d;
        *var_zflagbot_d_rv_slot = var_zflagbot_d_rv;
        *var_zflagbot_s_slot = var_zflagbot_s;
        *var_zflagbot_s_rv_slot = var_zflagbot_s_rv;
        *var_zflaggat_d_slot = var_zflaggat_d;
        *var_zflaggat_d_rv_slot = var_zflaggat_d_rv;
        *var_zflaggat_s_slot = var_zflaggat_s;
        *var_zflaggat_s_rv_slot = var_zflaggat_s_rv;
        *var_zflagsti_d_slot = var_zflagsti_d;
        *var_zflagsti_d_rv_slot = var_zflagsti_d_rv;
        *var_zflagsti_s_slot = var_zflagsti_s;
        *var_zflagsti_s_rv_slot = var_zflagsti_s_rv;
        *var_zfrac_slot = var_zfrac;
        *var_zfrac_rv_slot = var_zfrac_rv;
    }

    pub(super) fn stamp_reactive_block_21(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_betn_i: f64,
        var_coxprime: f64,
        var_ct_i: f64,
        var_ctg_i: f64,
        var_delvtac_i: f64,
        var_delvto_i: f64,
        var_dphib_i: f64,
        var_dvsbnud_i: f64,
        var_epssi: f64,
        var_factuo_i: f64,
        var_neff_i: f64,
        var_neffac_i: f64,
        var_np_i: f64,
        var_qq: f64,
        var_st2vfb_i: f64,
        var_stbet_i: f64,
        var_stct_i: f64,
        var_stmue_i: f64,
        var_stthemu_i: f64,
        var_stvfb_i: f64,
        var_themu_i: f64,
        var_tka: f64,
        var_tkr: f64,
        var_tox_sq: f64,
        var_vfb_i: f64,
        var_vsbnud_i: f64,
        var_alpha_b_slot: &mut f64,
        var_alpha_b_dn4_slot: &mut f64,
        var_alpha_b_rv_slot: &mut f64,
        var_aphi_ac_slot: &mut f64,
        var_aphi_ac_dn4_slot: &mut f64,
        var_aphi_ac_rv_slot: &mut f64,
        var_aphi_dc_slot: &mut f64,
        var_aphi_dc_dn4_slot: &mut f64,
        var_aphi_dc_rv_slot: &mut f64,
        var_arg2max_slot: &mut f64,
        var_arg2max_rv_slot: &mut f64,
        var_bet_i_slot: &mut f64,
        var_bet_i_dn4_slot: &mut f64,
        var_bet_i_rv_slot: &mut f64,
        var_betn_t_slot: &mut f64,
        var_betn_t_dn4_slot: &mut f64,
        var_betn_t_rv_slot: &mut f64,
        var_bphi_ac_slot: &mut f64,
        var_bphi_ac_dn4_slot: &mut f64,
        var_bphi_ac_rv_slot: &mut f64,
        var_bphi_dc_slot: &mut f64,
        var_bphi_dc_dn4_slot: &mut f64,
        var_bphi_dc_rv_slot: &mut f64,
        var_ct_t_slot: &mut f64,
        var_ct_t_dn4_slot: &mut f64,
        var_ct_t_rv_slot: &mut f64,
        var_ctg_t_slot: &mut f64,
        var_ctg_t_dn4_slot: &mut f64,
        var_ctg_t_rv_slot: &mut f64,
        var_delt_slot: &mut f64,
        var_delt_dn4_slot: &mut f64,
        var_delt_rv_slot: &mut f64,
        var_dphibq_slot: &mut f64,
        var_dphibq_dn4_slot: &mut f64,
        var_dphibq_rv_slot: &mut f64,
        var_eg_slot: &mut f64,
        var_eg_dn4_slot: &mut f64,
        var_eg_rv_slot: &mut f64,
        var_g_0_ac_slot: &mut f64,
        var_g_0_ac_dn4_slot: &mut f64,
        var_g_0_ac_rv_slot: &mut f64,
        var_g_0_dc_slot: &mut f64,
        var_g_0_dc_dn4_slot: &mut f64,
        var_g_0_dc_rv_slot: &mut f64,
        var_guard1024_slot: &mut f64,
        var_guard1024_rv_slot: &mut f64,
        var_guard1025_slot: &mut f64,
        var_guard1025_rv_slot: &mut f64,
        var_guard1026_slot: &mut f64,
        var_guard1026_rv_slot: &mut f64,
        var_inv_phit_slot: &mut f64,
        var_inv_phit_dn4_slot: &mut f64,
        var_inv_phit_rv_slot: &mut f64,
        var_kp_slot: &mut f64,
        var_kp_dn4_slot: &mut f64,
        var_kp_rv_slot: &mut f64,
        var_ln_rtn_slot: &mut f64,
        var_ln_rtn_dn4_slot: &mut f64,
        var_ln_rtn_rv_slot: &mut f64,
        var_np_slot: &mut f64,
        var_np_rv_slot: &mut f64,
        var_phib_ac_slot: &mut f64,
        var_phib_ac_dn4_slot: &mut f64,
        var_phib_ac_rv_slot: &mut f64,
        var_phib_dc_slot: &mut f64,
        var_phib_dc_dn4_slot: &mut f64,
        var_phib_dc_rv_slot: &mut f64,
        var_phibfac_slot: &mut f64,
        var_phibfac_dn4_slot: &mut f64,
        var_phibfac_rv_slot: &mut f64,
        var_phit_slot: &mut f64,
        var_phit_dn4_slot: &mut f64,
        var_phit_rv_slot: &mut f64,
        var_phix1_ac_slot: &mut f64,
        var_phix1_ac_dn4_slot: &mut f64,
        var_phix1_ac_rv_slot: &mut f64,
        var_phix1_dc_slot: &mut f64,
        var_phix1_dc_dn4_slot: &mut f64,
        var_phix1_dc_rv_slot: &mut f64,
        var_phix2_slot: &mut f64,
        var_phix2_dn4_slot: &mut f64,
        var_phix2_rv_slot: &mut f64,
        var_phix_ac_slot: &mut f64,
        var_phix_ac_dn4_slot: &mut f64,
        var_phix_ac_rv_slot: &mut f64,
        var_phix_dc_slot: &mut f64,
        var_phix_dc_dn4_slot: &mut f64,
        var_phix_dc_rv_slot: &mut f64,
        var_qb0_slot: &mut f64,
        var_qb0_dn4_slot: &mut f64,
        var_qb0_rv_slot: &mut f64,
        var_qlim2_slot: &mut f64,
        var_qlim2_dn4_slot: &mut f64,
        var_qlim2_rv_slot: &mut f64,
        var_rtn_slot: &mut f64,
        var_rtn_dn4_slot: &mut f64,
        var_rtn_rv_slot: &mut f64,
        var_sqrt_phib_dc_slot: &mut f64,
        var_sqrt_phib_dc_dn4_slot: &mut f64,
        var_sqrt_phib_dc_rv_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp1_rv_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp2_rv_slot: &mut f64,
        var_tf_bet_slot: &mut f64,
        var_tf_bet_dn4_slot: &mut f64,
        var_tf_bet_rv_slot: &mut f64,
        var_tf_ct_slot: &mut f64,
        var_tf_ct_dn4_slot: &mut f64,
        var_tf_ct_rv_slot: &mut f64,
        var_tf_mue_slot: &mut f64,
        var_tf_mue_dn4_slot: &mut f64,
        var_tf_mue_rv_slot: &mut f64,
        var_themu_t_slot: &mut f64,
        var_themu_t_dn4_slot: &mut f64,
        var_themu_t_rv_slot: &mut f64,
        var_tkd_slot: &mut f64,
        var_tkd_dn4_slot: &mut f64,
        var_tkd_rv_slot: &mut f64,
        var_tkd_sq_slot: &mut f64,
        var_tkd_sq_dn4_slot: &mut f64,
        var_tkd_sq_rv_slot: &mut f64,
        var_us1_slot: &mut f64,
        var_us1_dn4_slot: &mut f64,
        var_us1_rv_slot: &mut f64,
        var_us21_slot: &mut f64,
        var_us21_dn4_slot: &mut f64,
        var_us21_rv_slot: &mut f64,
        var_vfb_t_slot: &mut f64,
        var_vfb_t_dn4_slot: &mut f64,
        var_vfb_t_rv_slot: &mut f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let mut var_alpha_b: f64 = *var_alpha_b_slot;
        let mut var_alpha_b_dn4: f64 = *var_alpha_b_dn4_slot;
        let mut var_alpha_b_rv: f64 = *var_alpha_b_rv_slot;
        let mut var_aphi_ac: f64 = *var_aphi_ac_slot;
        let mut var_aphi_ac_dn4: f64 = *var_aphi_ac_dn4_slot;
        let mut var_aphi_ac_rv: f64 = *var_aphi_ac_rv_slot;
        let mut var_aphi_dc: f64 = *var_aphi_dc_slot;
        let mut var_aphi_dc_dn4: f64 = *var_aphi_dc_dn4_slot;
        let mut var_aphi_dc_rv: f64 = *var_aphi_dc_rv_slot;
        let mut var_arg2max: f64 = *var_arg2max_slot;
        let mut var_arg2max_rv: f64 = *var_arg2max_rv_slot;
        let mut var_bet_i: f64 = *var_bet_i_slot;
        let mut var_bet_i_dn4: f64 = *var_bet_i_dn4_slot;
        let mut var_bet_i_rv: f64 = *var_bet_i_rv_slot;
        let mut var_betn_t: f64 = *var_betn_t_slot;
        let mut var_betn_t_dn4: f64 = *var_betn_t_dn4_slot;
        let mut var_betn_t_rv: f64 = *var_betn_t_rv_slot;
        let mut var_bphi_ac: f64 = *var_bphi_ac_slot;
        let mut var_bphi_ac_dn4: f64 = *var_bphi_ac_dn4_slot;
        let mut var_bphi_ac_rv: f64 = *var_bphi_ac_rv_slot;
        let mut var_bphi_dc: f64 = *var_bphi_dc_slot;
        let mut var_bphi_dc_dn4: f64 = *var_bphi_dc_dn4_slot;
        let mut var_bphi_dc_rv: f64 = *var_bphi_dc_rv_slot;
        let mut var_ct_t: f64 = *var_ct_t_slot;
        let mut var_ct_t_dn4: f64 = *var_ct_t_dn4_slot;
        let mut var_ct_t_rv: f64 = *var_ct_t_rv_slot;
        let mut var_ctg_t: f64 = *var_ctg_t_slot;
        let mut var_ctg_t_dn4: f64 = *var_ctg_t_dn4_slot;
        let mut var_ctg_t_rv: f64 = *var_ctg_t_rv_slot;
        let mut var_delt: f64 = *var_delt_slot;
        let mut var_delt_dn4: f64 = *var_delt_dn4_slot;
        let mut var_delt_rv: f64 = *var_delt_rv_slot;
        let mut var_dphibq: f64 = *var_dphibq_slot;
        let mut var_dphibq_dn4: f64 = *var_dphibq_dn4_slot;
        let mut var_dphibq_rv: f64 = *var_dphibq_rv_slot;
        let mut var_eg: f64 = *var_eg_slot;
        let mut var_eg_dn4: f64 = *var_eg_dn4_slot;
        let mut var_eg_rv: f64 = *var_eg_rv_slot;
        let mut var_g_0_ac: f64 = *var_g_0_ac_slot;
        let mut var_g_0_ac_dn4: f64 = *var_g_0_ac_dn4_slot;
        let mut var_g_0_ac_rv: f64 = *var_g_0_ac_rv_slot;
        let mut var_g_0_dc: f64 = *var_g_0_dc_slot;
        let mut var_g_0_dc_dn4: f64 = *var_g_0_dc_dn4_slot;
        let mut var_g_0_dc_rv: f64 = *var_g_0_dc_rv_slot;
        let mut var_guard1024: f64 = *var_guard1024_slot;
        let mut var_guard1024_rv: f64 = *var_guard1024_rv_slot;
        let mut var_guard1025: f64 = *var_guard1025_slot;
        let mut var_guard1025_rv: f64 = *var_guard1025_rv_slot;
        let mut var_guard1026: f64 = *var_guard1026_slot;
        let mut var_guard1026_rv: f64 = *var_guard1026_rv_slot;
        let mut var_inv_phit: f64 = *var_inv_phit_slot;
        let mut var_inv_phit_dn4: f64 = *var_inv_phit_dn4_slot;
        let mut var_inv_phit_rv: f64 = *var_inv_phit_rv_slot;
        let mut var_kp: f64 = *var_kp_slot;
        let mut var_kp_dn4: f64 = *var_kp_dn4_slot;
        let mut var_kp_rv: f64 = *var_kp_rv_slot;
        let mut var_ln_rtn: f64 = *var_ln_rtn_slot;
        let mut var_ln_rtn_dn4: f64 = *var_ln_rtn_dn4_slot;
        let mut var_ln_rtn_rv: f64 = *var_ln_rtn_rv_slot;
        let mut var_np: f64 = *var_np_slot;
        let mut var_np_rv: f64 = *var_np_rv_slot;
        let mut var_phib_ac: f64 = *var_phib_ac_slot;
        let mut var_phib_ac_dn4: f64 = *var_phib_ac_dn4_slot;
        let mut var_phib_ac_rv: f64 = *var_phib_ac_rv_slot;
        let mut var_phib_dc: f64 = *var_phib_dc_slot;
        let mut var_phib_dc_dn4: f64 = *var_phib_dc_dn4_slot;
        let mut var_phib_dc_rv: f64 = *var_phib_dc_rv_slot;
        let mut var_phibfac: f64 = *var_phibfac_slot;
        let mut var_phibfac_dn4: f64 = *var_phibfac_dn4_slot;
        let mut var_phibfac_rv: f64 = *var_phibfac_rv_slot;
        let mut var_phit: f64 = *var_phit_slot;
        let mut var_phit_dn4: f64 = *var_phit_dn4_slot;
        let mut var_phit_rv: f64 = *var_phit_rv_slot;
        let mut var_phix1_ac: f64 = *var_phix1_ac_slot;
        let mut var_phix1_ac_dn4: f64 = *var_phix1_ac_dn4_slot;
        let mut var_phix1_ac_rv: f64 = *var_phix1_ac_rv_slot;
        let mut var_phix1_dc: f64 = *var_phix1_dc_slot;
        let mut var_phix1_dc_dn4: f64 = *var_phix1_dc_dn4_slot;
        let mut var_phix1_dc_rv: f64 = *var_phix1_dc_rv_slot;
        let mut var_phix2: f64 = *var_phix2_slot;
        let mut var_phix2_dn4: f64 = *var_phix2_dn4_slot;
        let mut var_phix2_rv: f64 = *var_phix2_rv_slot;
        let mut var_phix_ac: f64 = *var_phix_ac_slot;
        let mut var_phix_ac_dn4: f64 = *var_phix_ac_dn4_slot;
        let mut var_phix_ac_rv: f64 = *var_phix_ac_rv_slot;
        let mut var_phix_dc: f64 = *var_phix_dc_slot;
        let mut var_phix_dc_dn4: f64 = *var_phix_dc_dn4_slot;
        let mut var_phix_dc_rv: f64 = *var_phix_dc_rv_slot;
        let mut var_qb0: f64 = *var_qb0_slot;
        let mut var_qb0_dn4: f64 = *var_qb0_dn4_slot;
        let mut var_qb0_rv: f64 = *var_qb0_rv_slot;
        let mut var_qlim2: f64 = *var_qlim2_slot;
        let mut var_qlim2_dn4: f64 = *var_qlim2_dn4_slot;
        let mut var_qlim2_rv: f64 = *var_qlim2_rv_slot;
        let mut var_rtn: f64 = *var_rtn_slot;
        let mut var_rtn_dn4: f64 = *var_rtn_dn4_slot;
        let mut var_rtn_rv: f64 = *var_rtn_rv_slot;
        let mut var_sqrt_phib_dc: f64 = *var_sqrt_phib_dc_slot;
        let mut var_sqrt_phib_dc_dn4: f64 = *var_sqrt_phib_dc_dn4_slot;
        let mut var_sqrt_phib_dc_rv: f64 = *var_sqrt_phib_dc_rv_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp1_rv: f64 = *var_temp1_rv_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp2_rv: f64 = *var_temp2_rv_slot;
        let mut var_tf_bet: f64 = *var_tf_bet_slot;
        let mut var_tf_bet_dn4: f64 = *var_tf_bet_dn4_slot;
        let mut var_tf_bet_rv: f64 = *var_tf_bet_rv_slot;
        let mut var_tf_ct: f64 = *var_tf_ct_slot;
        let mut var_tf_ct_dn4: f64 = *var_tf_ct_dn4_slot;
        let mut var_tf_ct_rv: f64 = *var_tf_ct_rv_slot;
        let mut var_tf_mue: f64 = *var_tf_mue_slot;
        let mut var_tf_mue_dn4: f64 = *var_tf_mue_dn4_slot;
        let mut var_tf_mue_rv: f64 = *var_tf_mue_rv_slot;
        let mut var_themu_t: f64 = *var_themu_t_slot;
        let mut var_themu_t_dn4: f64 = *var_themu_t_dn4_slot;
        let mut var_themu_t_rv: f64 = *var_themu_t_rv_slot;
        let mut var_tkd: f64 = *var_tkd_slot;
        let mut var_tkd_dn4: f64 = *var_tkd_dn4_slot;
        let mut var_tkd_rv: f64 = *var_tkd_rv_slot;
        let mut var_tkd_sq: f64 = *var_tkd_sq_slot;
        let mut var_tkd_sq_dn4: f64 = *var_tkd_sq_dn4_slot;
        let mut var_tkd_sq_rv: f64 = *var_tkd_sq_rv_slot;
        let mut var_us1: f64 = *var_us1_slot;
        let mut var_us1_dn4: f64 = *var_us1_dn4_slot;
        let mut var_us1_rv: f64 = *var_us1_rv_slot;
        let mut var_us21: f64 = *var_us21_slot;
        let mut var_us21_dn4: f64 = *var_us21_dn4_slot;
        let mut var_us21_rv: f64 = *var_us21_rv_slot;
        let mut var_vfb_t: f64 = *var_vfb_t_slot;
        let mut var_vfb_t_dn4: f64 = *var_vfb_t_dn4_slot;
        let mut var_vfb_t_rv: f64 = *var_vfb_t_rv_slot;

        var_temp1 = 0.0;
        var_temp1_dn4 = 0.0;
        var_temp1_dn6 = 0.0;
        var_temp1_dn7 = 0.0;
        var_temp1_dn8 = 0.0;
        var_temp1_dn9 = 0.0;
        var_temp1_rv = 0.0;

        var_temp2 = 0.0;
        var_temp2_dn4 = 0.0;
        var_temp2_dn6 = 0.0;
        var_temp2_dn7 = 0.0;
        var_temp2_dn8 = 0.0;
        var_temp2_dn9 = 0.0;
        var_temp2_rv = 0.0;

        let assign39430_e52953: f64 = (var_tka + (nv4 - 0.0));
        var_tkd = assign39430_e52953;
        var_tkd_dn4 = 1.0;
        var_tkd_rv = 0.0;

        let assign39440_e52956: f64 = (var_tkd * var_tkd);
        var_tkd_sq = assign39440_e52956;
        var_tkd_sq_dn4 = ((var_tkd_dn4 * var_tkd) + (var_tkd * var_tkd_dn4));
        var_tkd_sq_rv = 0.0;

        let assign39450_e52959: f64 = (var_tkd - var_tkr);
        var_delt = assign39450_e52959;
        var_delt_dn4 = var_tkd_dn4;
        var_delt_rv = 0.0;

        let assign39460_e52962: f64 = (var_tkr / var_tkd);
        var_rtn = assign39460_e52962;
        var_rtn_dn4 = (-((var_tkr * var_tkd_dn4) / (var_tkd * var_tkd)));
        var_rtn_rv = 0.0;

        let assign39470_e52964: f64 = (var_rtn).ln();
        var_ln_rtn = assign39470_e52964;
        var_ln_rtn_dn4 = (var_rtn_dn4 / var_rtn);
        var_ln_rtn_rv = 0.0;

        let assign39480_e52967: f64 = (var_tkd * 1.3806505e-23);
        let assign39480_e52969: f64 = (assign39480_e52967 / 1.6021918e-19);
        var_phit = assign39480_e52969;
        var_phit_dn4 = ((var_tkd_dn4 * 1.3806505e-23) / 1.6021918e-19);
        var_phit_rv = 0.0;

        let assign39490_e52972: f64 = (1.0 / var_phit);
        var_inv_phit = assign39490_e52972;
        var_inv_phit_dn4 = (-(var_phit_dn4 / (var_phit * var_phit)));
        var_inv_phit_rv = 0.0;

        let assign39500_e52976: f64 = (9.025e-5 * var_tkd);
        let assign39500_e52977: f64 = (1.179 - assign39500_e52976);
        let assign39500_e52980: f64 = (3.05e-7 * var_tkd_sq);
        let assign39500_e52981: f64 = (assign39500_e52977 - assign39500_e52980);
        var_eg = assign39500_e52981;
        var_eg_dn4 = ((-(9.025e-5 * var_tkd_dn4)) - (3.05e-7 * var_tkd_sq_dn4));
        var_eg_rv = 0.0;

        let assign39510_e52985: f64 = (0.00045 * var_tkd);
        let assign39510_e52986: f64 = (1.045 + assign39510_e52985);
        let assign39510_e52990: f64 = (0.0014 * var_tkd);
        let assign39510_e52991: f64 = (0.523 + assign39510_e52990);
        let assign39510_e52994: f64 = (1.48e-6 * var_tkd_sq);
        let assign39510_e52995: f64 = (assign39510_e52991 - assign39510_e52994);
        let assign39510_e52996: f64 = (assign39510_e52986 * assign39510_e52995);
        let assign39510_e52998: f64 = (assign39510_e52996 * var_tkd_sq);
        let assign39510_e53000: f64 = (assign39510_e52998 / 90000.0);
        var_phibfac = assign39510_e53000;
        var_phibfac_dn4 = ((((((0.00045 * var_tkd_dn4) * assign39510_e52995) + (assign39510_e52986 * ((0.0014 * var_tkd_dn4) - (1.48e-6 * var_tkd_sq_dn4)))) * var_tkd_sq) + (assign39510_e52996 * var_tkd_sq_dn4)) / 90000.0);
        var_phibfac_rv = 0.0;

        let (assign39520_e53006, assign39520_e53006_d_n4,) = {
    if (var_phibfac > 0.001) {
        (var_phibfac, var_phibfac_dn4,)
    } else {
        (0.001, 0.0,)
    }
};
        var_phibfac = assign39520_e53006;
        var_phibfac_dn4 = assign39520_e53006_d_n4;
        var_phibfac_rv = 0.0;

        let assign39540_e53014: f64 = (var_eg + var_dphib_i);
        let assign39540_e53017: f64 = (2.0 * var_phit);
        let assign39540_e53021: f64 = (-0.75);
        let assign39540_e53022: f64 = (var_phibfac).powf(assign39540_e53021);
        let assign39540_e53023: f64 = (var_neff_i * assign39540_e53022);
        let assign39540_e53025: f64 = (assign39540_e53023 * 4e-26);
        let assign39540_e53026: f64 = (assign39540_e53025).ln();
        let assign39540_e53027: f64 = (assign39540_e53017 * assign39540_e53026);
        let assign39540_e53028: f64 = (assign39540_e53014 + assign39540_e53027);
        var_phib_dc = assign39540_e53028;
        var_phib_dc_dn4 = (var_eg_dn4 + (((2.0 * var_phit_dn4) * assign39540_e53026) + (assign39540_e53017 * (((var_neff_i * if 0.0 == 0.0 && ((assign39540_e53021) as f64).is_finite() && ((assign39540_e53021) as f64).fract() == 0.0 { if assign39540_e53021 == 0.0 { 0.0 } else { (assign39540_e53021 * ((var_phibfac).powf(assign39540_e53021 - 1.0) * var_phibfac_dn4)) } } else { (assign39540_e53022 * (assign39540_e53021 * (var_phibfac_dn4 / var_phibfac))) }) * 4e-26) / assign39540_e53025))));
        var_phib_dc_rv = 0.0;

        let (assign39550_e53034, assign39550_e53034_d_n4,) = {
    if (var_phib_dc > 0.05) {
        (var_phib_dc, var_phib_dc_dn4,)
    } else {
        (0.05, 0.0,)
    }
};
        var_phib_dc = assign39550_e53034;
        var_phib_dc_dn4 = assign39550_e53034_d_n4;
        var_phib_dc_rv = 0.0;

        let assign39560_e53037: f64 = (2.0 * 1.6021918e-19);
        let assign39560_e53039: f64 = (assign39560_e53037 * var_neff_i);
        let assign39560_e53041: f64 = (assign39560_e53039 * var_epssi);
        let assign39560_e53043: f64 = (assign39560_e53041 * var_inv_phit);
        let assign39560_e53044: f64 = (assign39560_e53043).sqrt();
        let assign39560_e53046: f64 = (assign39560_e53044 / var_coxprime);
        var_g_0_dc = assign39560_e53046;
        var_g_0_dc_dn4 = (((assign39560_e53041 * var_inv_phit_dn4) / (2.0 * assign39560_e53044)) / var_coxprime);
        var_g_0_dc_rv = 0.0;

        var_kp = 0.0;
        var_kp_dn4 = 0.0;
        var_kp_rv = 0.0;

        var_np = 0.0;
        var_np_rv = 0.0;

        let assign39590_e53051: f64 = if var_np_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1024 = assign39590_e53051;
        var_guard1024_rv = 0.0;

        let (assign39600_e53057,) = {
    if (var_guard1024 != 0.0) {
        let assign39600_e53055: f64 = (80000000.0 / var_tox_sq);
        (assign39600_e53055,)
    } else {
        (var_arg2max,)
    }
};
        var_arg2max = assign39600_e53057;
        var_arg2max_rv = 0.0;

        let (assign39610_e53066,) = {
    if (var_guard1024 != 0.0) {
        let (assign39610_e53064,) = {
            if (var_np_i > var_arg2max) {
                (var_np_i,)
            } else {
                (var_arg2max,)
            }
        };
        (assign39610_e53064,)
    } else {
        (var_np,)
    }
};
        var_np = assign39610_e53066;
        var_np_rv = 0.0;

        let (assign39620_e53075,) = {
    if (var_guard1024 != 0.0) {
        let (assign39620_e53073,) = {
            if (5e24 > var_np) {
                (5e24,)
            } else {
                (var_np,)
            }
        };
        (assign39620_e53073,)
    } else {
        (var_np,)
    }
};
        var_np = assign39620_e53075;
        var_np_rv = 0.0;

        let (assign39630_e53091, assign39630_e53091_d_n4,) = {
    if (var_guard1024 != 0.0) {
        let assign39630_e53079: f64 = (2.0 * var_coxprime);
        let assign39630_e53081: f64 = (assign39630_e53079 * var_coxprime);
        let assign39630_e53083: f64 = (assign39630_e53081 * var_phit);
        let assign39630_e53086: f64 = (1.6021918e-19 * var_np);
        let assign39630_e53088: f64 = (assign39630_e53086 * var_epssi);
        let assign39630_e53089: f64 = (assign39630_e53083 / assign39630_e53088);
        (assign39630_e53089, ((assign39630_e53081 * var_phit_dn4) / assign39630_e53088),)
    } else {
        (var_kp, var_kp_dn4,)
    }
};
        var_kp = assign39630_e53091;
        var_kp_dn4 = assign39630_e53091_d_n4;
        var_kp_rv = 0.0;

        let assign39640_e53094: f64 = (100.0 * var_phit);
        let assign39640_e53096: f64 = (assign39640_e53094 * var_phit);
        var_qlim2 = assign39640_e53096;
        var_qlim2_dn4 = (((100.0 * var_phit_dn4) * var_phit) + (assign39640_e53094 * var_phit_dn4));
        var_qlim2_rv = 0.0;

        let assign39650_e53099: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };
        var_guard1025 = assign39650_e53099;
        var_guard1025_rv = 0.0;

        let (assign39660_e53110, assign39660_e53110_d_n4,) = {
    if (var_guard1025 != 0.0) {
        let assign39660_e53103: f64 = (var_phit * var_g_0_dc);
        let assign39660_e53105: f64 = (assign39660_e53103 * var_g_0_dc);
        let assign39660_e53107: f64 = (assign39660_e53105 * var_phib_dc);
        let assign39660_e53108: f64 = (assign39660_e53107).sqrt();
        (assign39660_e53108, (((((((var_phit_dn4 * var_g_0_dc) + (var_phit * var_g_0_dc_dn4)) * var_g_0_dc) + (assign39660_e53103 * var_g_0_dc_dn4)) * var_phib_dc) + (assign39660_e53105 * var_phib_dc_dn4)) / (2.0 * assign39660_e53108)),)
    } else {
        (var_qb0, var_qb0_dn4,)
    }
};
        var_qb0 = assign39660_e53110;
        var_qb0_dn4 = assign39660_e53110_d_n4;
        var_qb0_rv = 0.0;

        let (assign39670_e53120, assign39670_e53120_d_n4,) = {
    if (var_guard1025 != 0.0) {
        let assign39670_e53114: f64 = (0.75 * var_qq);
        let assign39670_e53117: f64 = (var_qb0).powf(0.6666666666666666);
        let assign39670_e53118: f64 = (assign39670_e53114 * assign39670_e53117);
        (assign39670_e53118, (assign39670_e53114 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_qb0).powf(0.6666666666666666 - 1.0) * var_qb0_dn4)) } } else { (assign39670_e53117 * (0.6666666666666666 * (var_qb0_dn4 / var_qb0))) }),)
    } else {
        (var_dphibq, var_dphibq_dn4,)
    }
};
        var_dphibq = assign39670_e53120;
        var_dphibq_dn4 = assign39670_e53120_d_n4;
        var_dphibq_rv = 0.0;

        let (assign39680_e53126, assign39680_e53126_d_n4,) = {
    if (var_guard1025 != 0.0) {
        let assign39680_e53124: f64 = (var_phib_dc + var_dphibq);
        (assign39680_e53124, (var_phib_dc_dn4 + var_dphibq_dn4),)
    } else {
        (var_phib_dc, var_phib_dc_dn4,)
    }
};
        var_phib_dc = assign39680_e53126;
        var_phib_dc_dn4 = assign39680_e53126_d_n4;
        var_phib_dc_rv = 0.0;

        let (assign39690_e53140, assign39690_e53140_d_n4,) = {
    if (var_guard1025 != 0.0) {
        let assign39690_e53132: f64 = (2.0 * 0.6666666666666666);
        let assign39690_e53134: f64 = (assign39690_e53132 * var_dphibq);
        let assign39690_e53136: f64 = (assign39690_e53134 / var_qb0);
        let assign39690_e53137: f64 = (1.0 + assign39690_e53136);
        let assign39690_e53138: f64 = (var_g_0_dc * assign39690_e53137);
        (assign39690_e53138, ((var_g_0_dc_dn4 * assign39690_e53137) + (var_g_0_dc * ((((assign39690_e53132 * var_dphibq_dn4) * var_qb0) - (assign39690_e53134 * var_qb0_dn4)) / (var_qb0 * var_qb0)))),)
    } else {
        (var_g_0_dc, var_g_0_dc_dn4,)
    }
};
        var_g_0_dc = assign39690_e53140;
        var_g_0_dc_dn4 = assign39690_e53140_d_n4;
        var_g_0_dc_rv = 0.0;

        let assign39700_e53142: f64 = (var_phib_dc).sqrt();
        var_sqrt_phib_dc = assign39700_e53142;
        var_sqrt_phib_dc_dn4 = (var_phib_dc_dn4 / (2.0 * assign39700_e53142));
        var_sqrt_phib_dc_rv = 0.0;

        let assign39710_e53145: f64 = (0.95 * var_phib_dc);
        var_phix_dc = assign39710_e53145;
        var_phix_dc_dn4 = (0.95 * var_phib_dc_dn4);
        var_phix_dc_rv = 0.0;

        let assign39720_e53148: f64 = (0.0025 * var_phib_dc);
        let assign39720_e53150: f64 = (assign39720_e53148 * var_phib_dc);
        var_aphi_dc = assign39720_e53150;
        var_aphi_dc_dn4 = (((0.0025 * var_phib_dc_dn4) * var_phib_dc) + (assign39720_e53148 * var_phib_dc_dn4));
        var_aphi_dc_rv = 0.0;

        var_bphi_dc = var_aphi_dc;
        var_bphi_dc_dn4 = var_aphi_dc_dn4;
        var_bphi_dc_rv = 0.0;

        let assign39740_e53154: f64 = (var_bphi_dc).sqrt();
        let assign39740_e53155: f64 = (0.5 * assign39740_e53154);
        var_phix2 = assign39740_e53155;
        var_phix2_dn4 = (0.5 * (var_bphi_dc_dn4 / (2.0 * assign39740_e53154)));
        var_phix2_rv = 0.0;

        let assign39750_e53159: f64 = (var_phix_dc - var_phix2);
        let assign39750_e53161: f64 = assign39750_e53159;
        let assign39750_e53164: f64 = (var_phix_dc - var_phix2);
        let assign39750_e53166: f64 = assign39750_e53164;
        let assign39750_e53169: f64 = (var_phix_dc - var_phix2);
        let assign39750_e53171: f64 = assign39750_e53169;
        let assign39750_e53172: f64 = (assign39750_e53166 * assign39750_e53171);
        let assign39750_e53174: f64 = (assign39750_e53172 + var_aphi_dc);
        let assign39750_e53175: f64 = (assign39750_e53174).sqrt();
        let assign39750_e53176: f64 = (assign39750_e53161 - assign39750_e53175);
        let assign39750_e53177: f64 = (0.5 * assign39750_e53176);
        var_phix1_dc = assign39750_e53177;
        var_phix1_dc_dn4 = (0.5 * ((var_phix_dc_dn4 - var_phix2_dn4) - (((((var_phix_dc_dn4 - var_phix2_dn4) * assign39750_e53171) + (assign39750_e53166 * (var_phix_dc_dn4 - var_phix2_dn4))) + var_aphi_dc_dn4) / (2.0 * assign39750_e53175))));
        var_phix1_dc_rv = 0.0;

        let assign39760_e53181: f64 = (var_phib_dc + var_eg);
        let assign39760_e53182: f64 = (0.5 * assign39760_e53181);
        var_alpha_b = assign39760_e53182;
        var_alpha_b_dn4 = (0.5 * (var_phib_dc_dn4 + var_eg_dn4));
        var_alpha_b_rv = 0.0;

        let assign39770_e53185: f64 = (var_vsbnud_i + var_phib_dc);
        let assign39770_e53186: f64 = (assign39770_e53185).sqrt();
        let assign39770_e53188: f64 = (assign39770_e53186 - var_sqrt_phib_dc);
        var_us1 = assign39770_e53188;
        var_us1_dn4 = ((var_phib_dc_dn4 / (2.0 * assign39770_e53186)) - var_sqrt_phib_dc_dn4);
        var_us1_rv = 0.0;

        let assign39780_e53191: f64 = (var_vsbnud_i + var_dvsbnud_i);
        let assign39780_e53193: f64 = (assign39780_e53191 + var_phib_dc);
        let assign39780_e53194: f64 = (assign39780_e53193).sqrt();
        let assign39780_e53196: f64 = (assign39780_e53194 - var_sqrt_phib_dc);
        let assign39780_e53198: f64 = (assign39780_e53196 - var_us1);
        var_us21 = assign39780_e53198;
        var_us21_dn4 = (((var_phib_dc_dn4 / (2.0 * assign39780_e53194)) - var_sqrt_phib_dc_dn4) - var_us1_dn4);
        var_us21_rv = 0.0;

        let assign39790_e53201: f64 = (var_eg + var_dphib_i);
        let assign39790_e53203: f64 = (assign39790_e53201 + var_delvtac_i);
        let assign39790_e53206: f64 = (2.0 * var_phit);
        let assign39790_e53210: f64 = (-0.75);
        let assign39790_e53211: f64 = (var_phibfac).powf(assign39790_e53210);
        let assign39790_e53212: f64 = (var_neffac_i * assign39790_e53211);
        let assign39790_e53214: f64 = (assign39790_e53212 * 4e-26);
        let assign39790_e53215: f64 = (assign39790_e53214).ln();
        let assign39790_e53216: f64 = (assign39790_e53206 * assign39790_e53215);
        let assign39790_e53217: f64 = (assign39790_e53203 + assign39790_e53216);
        var_phib_ac = assign39790_e53217;
        var_phib_ac_dn4 = (var_eg_dn4 + (((2.0 * var_phit_dn4) * assign39790_e53215) + (assign39790_e53206 * (((var_neffac_i * if 0.0 == 0.0 && ((assign39790_e53210) as f64).is_finite() && ((assign39790_e53210) as f64).fract() == 0.0 { if assign39790_e53210 == 0.0 { 0.0 } else { (assign39790_e53210 * ((var_phibfac).powf(assign39790_e53210 - 1.0) * var_phibfac_dn4)) } } else { (assign39790_e53211 * (assign39790_e53210 * (var_phibfac_dn4 / var_phibfac))) }) * 4e-26) / assign39790_e53214))));
        var_phib_ac_rv = 0.0;

        let (assign39800_e53223, assign39800_e53223_d_n4,) = {
    if (var_phib_ac > 0.05) {
        (var_phib_ac, var_phib_ac_dn4,)
    } else {
        (0.05, 0.0,)
    }
};
        var_phib_ac = assign39800_e53223;
        var_phib_ac_dn4 = assign39800_e53223_d_n4;
        var_phib_ac_rv = 0.0;

        let assign39810_e53226: f64 = (2.0 * 1.6021918e-19);
        let assign39810_e53228: f64 = (assign39810_e53226 * var_neffac_i);
        let assign39810_e53230: f64 = (assign39810_e53228 * var_epssi);
        let assign39810_e53232: f64 = (assign39810_e53230 * var_inv_phit);
        let assign39810_e53233: f64 = (assign39810_e53232).sqrt();
        let assign39810_e53235: f64 = (assign39810_e53233 / var_coxprime);
        var_g_0_ac = assign39810_e53235;
        var_g_0_ac_dn4 = (((assign39810_e53230 * var_inv_phit_dn4) / (2.0 * assign39810_e53233)) / var_coxprime);
        var_g_0_ac_rv = 0.0;

        let assign39820_e53238: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };
        var_guard1026 = assign39820_e53238;
        var_guard1026_rv = 0.0;

        let (assign39830_e53249, assign39830_e53249_d_n4,) = {
    if (var_guard1026 != 0.0) {
        let assign39830_e53242: f64 = (var_phit * var_g_0_ac);
        let assign39830_e53244: f64 = (assign39830_e53242 * var_g_0_ac);
        let assign39830_e53246: f64 = (assign39830_e53244 * var_phib_ac);
        let assign39830_e53247: f64 = (assign39830_e53246).sqrt();
        (assign39830_e53247, (((((((var_phit_dn4 * var_g_0_ac) + (var_phit * var_g_0_ac_dn4)) * var_g_0_ac) + (assign39830_e53242 * var_g_0_ac_dn4)) * var_phib_ac) + (assign39830_e53244 * var_phib_ac_dn4)) / (2.0 * assign39830_e53247)),)
    } else {
        (var_qb0, var_qb0_dn4,)
    }
};
        var_qb0 = assign39830_e53249;
        var_qb0_dn4 = assign39830_e53249_d_n4;
        var_qb0_rv = 0.0;

        let (assign39840_e53259, assign39840_e53259_d_n4,) = {
    if (var_guard1026 != 0.0) {
        let assign39840_e53253: f64 = (0.75 * var_qq);
        let assign39840_e53256: f64 = (var_qb0).powf(0.6666666666666666);
        let assign39840_e53257: f64 = (assign39840_e53253 * assign39840_e53256);
        (assign39840_e53257, (assign39840_e53253 * if 0.0 == 0.0 && ((0.6666666666666666) as f64).is_finite() && ((0.6666666666666666) as f64).fract() == 0.0 { if 0.6666666666666666 == 0.0 { 0.0 } else { (0.6666666666666666 * ((var_qb0).powf(0.6666666666666666 - 1.0) * var_qb0_dn4)) } } else { (assign39840_e53256 * (0.6666666666666666 * (var_qb0_dn4 / var_qb0))) }),)
    } else {
        (var_dphibq, var_dphibq_dn4,)
    }
};
        var_dphibq = assign39840_e53259;
        var_dphibq_dn4 = assign39840_e53259_d_n4;
        var_dphibq_rv = 0.0;

        let (assign39850_e53265, assign39850_e53265_d_n4,) = {
    if (var_guard1026 != 0.0) {
        let assign39850_e53263: f64 = (var_phib_ac + var_dphibq);
        (assign39850_e53263, (var_phib_ac_dn4 + var_dphibq_dn4),)
    } else {
        (var_phib_ac, var_phib_ac_dn4,)
    }
};
        var_phib_ac = assign39850_e53265;
        var_phib_ac_dn4 = assign39850_e53265_d_n4;
        var_phib_ac_rv = 0.0;

        let (assign39860_e53279, assign39860_e53279_d_n4,) = {
    if (var_guard1026 != 0.0) {
        let assign39860_e53271: f64 = (2.0 * 0.6666666666666666);
        let assign39860_e53273: f64 = (assign39860_e53271 * var_dphibq);
        let assign39860_e53275: f64 = (assign39860_e53273 / var_qb0);
        let assign39860_e53276: f64 = (1.0 + assign39860_e53275);
        let assign39860_e53277: f64 = (var_g_0_ac * assign39860_e53276);
        (assign39860_e53277, ((var_g_0_ac_dn4 * assign39860_e53276) + (var_g_0_ac * ((((assign39860_e53271 * var_dphibq_dn4) * var_qb0) - (assign39860_e53273 * var_qb0_dn4)) / (var_qb0 * var_qb0)))),)
    } else {
        (var_g_0_ac, var_g_0_ac_dn4,)
    }
};
        var_g_0_ac = assign39860_e53279;
        var_g_0_ac_dn4 = assign39860_e53279_d_n4;
        var_g_0_ac_rv = 0.0;

        let assign39870_e53282: f64 = (0.95 * var_phib_ac);
        var_phix_ac = assign39870_e53282;
        var_phix_ac_dn4 = (0.95 * var_phib_ac_dn4);
        var_phix_ac_rv = 0.0;

        let assign39880_e53285: f64 = (0.0025 * var_phib_ac);
        let assign39880_e53287: f64 = (assign39880_e53285 * var_phib_ac);
        var_aphi_ac = assign39880_e53287;
        var_aphi_ac_dn4 = (((0.0025 * var_phib_ac_dn4) * var_phib_ac) + (assign39880_e53285 * var_phib_ac_dn4));
        var_aphi_ac_rv = 0.0;

        var_bphi_ac = var_aphi_ac;
        var_bphi_ac_dn4 = var_aphi_ac_dn4;
        var_bphi_ac_rv = 0.0;

        let assign39900_e53291: f64 = (var_bphi_ac).sqrt();
        let assign39900_e53292: f64 = (0.5 * assign39900_e53291);
        var_phix2 = assign39900_e53292;
        var_phix2_dn4 = (0.5 * (var_bphi_ac_dn4 / (2.0 * assign39900_e53291)));
        var_phix2_rv = 0.0;

        let assign39910_e53296: f64 = (var_phix_ac - var_phix2);
        let assign39910_e53298: f64 = assign39910_e53296;
        let assign39910_e53301: f64 = (var_phix_ac - var_phix2);
        let assign39910_e53303: f64 = assign39910_e53301;
        let assign39910_e53306: f64 = (var_phix_ac - var_phix2);
        let assign39910_e53308: f64 = assign39910_e53306;
        let assign39910_e53309: f64 = (assign39910_e53303 * assign39910_e53308);
        let assign39910_e53311: f64 = (assign39910_e53309 + var_aphi_ac);
        let assign39910_e53312: f64 = (assign39910_e53311).sqrt();
        let assign39910_e53313: f64 = (assign39910_e53298 - assign39910_e53312);
        let assign39910_e53314: f64 = (0.5 * assign39910_e53313);
        var_phix1_ac = assign39910_e53314;
        var_phix1_ac_dn4 = (0.5 * ((var_phix_ac_dn4 - var_phix2_dn4) - (((((var_phix_ac_dn4 - var_phix2_dn4) * assign39910_e53308) + (assign39910_e53303 * (var_phix_ac_dn4 - var_phix2_dn4))) + var_aphi_ac_dn4) / (2.0 * assign39910_e53312))));
        var_phix1_ac_rv = 0.0;

        let assign39920_e53318: f64 = (var_stvfb_i * var_delt);
        let assign39920_e53322: f64 = (var_st2vfb_i * var_delt);
        let assign39920_e53323: f64 = (1.0 + assign39920_e53322);
        let assign39920_e53324: f64 = (assign39920_e53318 * assign39920_e53323);
        let assign39920_e53325: f64 = (var_vfb_i + assign39920_e53324);
        let assign39920_e53327: f64 = (assign39920_e53325 + var_delvto_i);
        var_vfb_t = assign39920_e53327;
        var_vfb_t_dn4 = (((var_stvfb_i * var_delt_dn4) * assign39920_e53323) + (assign39920_e53318 * (var_st2vfb_i * var_delt_dn4)));
        var_vfb_t_rv = 0.0;

        let assign39930_e53330: f64 = (var_stct_i * var_ln_rtn);
        let assign39930_e53331: f64 = (assign39930_e53330).exp();
        var_tf_ct = assign39930_e53331;
        var_tf_ct_dn4 = (assign39930_e53331 * (var_stct_i * var_ln_rtn_dn4));
        var_tf_ct_rv = 0.0;

        let assign39940_e53334: f64 = (var_ct_i * var_tf_ct);
        var_ct_t = assign39940_e53334;
        var_ct_t_dn4 = (var_ct_i * var_tf_ct_dn4);
        var_ct_t_rv = 0.0;

        let assign39950_e53337: f64 = (var_ctg_i / var_rtn);
        var_ctg_t = assign39950_e53337;
        var_ctg_t_dn4 = (-((var_ctg_i * var_rtn_dn4) / (var_rtn * var_rtn)));
        var_ctg_t_rv = 0.0;

        let assign39960_e53340: f64 = (var_stbet_i * var_ln_rtn);
        let assign39960_e53341: f64 = (assign39960_e53340).exp();
        var_tf_bet = assign39960_e53341;
        var_tf_bet_dn4 = (assign39960_e53341 * (var_stbet_i * var_ln_rtn_dn4));
        var_tf_bet_rv = 0.0;

        let assign39970_e53344: f64 = (var_betn_i * var_tf_bet);
        var_betn_t = assign39970_e53344;
        var_betn_t_dn4 = (var_betn_i * var_tf_bet_dn4);
        var_betn_t_rv = 0.0;

        let assign39980_e53347: f64 = (var_factuo_i * var_betn_t);
        let assign39980_e53349: f64 = (assign39980_e53347 * var_coxprime);
        var_bet_i = assign39980_e53349;
        var_bet_i_dn4 = ((var_factuo_i * var_betn_t_dn4) * var_coxprime);
        var_bet_i_rv = 0.0;

        let assign39990_e53353: f64 = (var_stthemu_i * var_ln_rtn);
        let assign39990_e53354: f64 = (assign39990_e53353).exp();
        let assign39990_e53355: f64 = (var_themu_i * assign39990_e53354);
        var_themu_t = assign39990_e53355;
        var_themu_t_dn4 = (var_themu_i * (assign39990_e53354 * (var_stthemu_i * var_ln_rtn_dn4)));
        var_themu_t_rv = 0.0;

        let assign40000_e53358: f64 = (var_stmue_i * var_ln_rtn);
        let assign40000_e53359: f64 = (assign40000_e53358).exp();
        var_tf_mue = assign40000_e53359;
        var_tf_mue_dn4 = (assign40000_e53359 * (var_stmue_i * var_ln_rtn_dn4));
        var_tf_mue_rv = 0.0;

        *var_alpha_b_slot = var_alpha_b;
        *var_alpha_b_dn4_slot = var_alpha_b_dn4;
        *var_alpha_b_rv_slot = var_alpha_b_rv;
        *var_aphi_ac_slot = var_aphi_ac;
        *var_aphi_ac_dn4_slot = var_aphi_ac_dn4;
        *var_aphi_ac_rv_slot = var_aphi_ac_rv;
        *var_aphi_dc_slot = var_aphi_dc;
        *var_aphi_dc_dn4_slot = var_aphi_dc_dn4;
        *var_aphi_dc_rv_slot = var_aphi_dc_rv;
        *var_arg2max_slot = var_arg2max;
        *var_arg2max_rv_slot = var_arg2max_rv;
        *var_bet_i_slot = var_bet_i;
        *var_bet_i_dn4_slot = var_bet_i_dn4;
        *var_bet_i_rv_slot = var_bet_i_rv;
        *var_betn_t_slot = var_betn_t;
        *var_betn_t_dn4_slot = var_betn_t_dn4;
        *var_betn_t_rv_slot = var_betn_t_rv;
        *var_bphi_ac_slot = var_bphi_ac;
        *var_bphi_ac_dn4_slot = var_bphi_ac_dn4;
        *var_bphi_ac_rv_slot = var_bphi_ac_rv;
        *var_bphi_dc_slot = var_bphi_dc;
        *var_bphi_dc_dn4_slot = var_bphi_dc_dn4;
        *var_bphi_dc_rv_slot = var_bphi_dc_rv;
        *var_ct_t_slot = var_ct_t;
        *var_ct_t_dn4_slot = var_ct_t_dn4;
        *var_ct_t_rv_slot = var_ct_t_rv;
        *var_ctg_t_slot = var_ctg_t;
        *var_ctg_t_dn4_slot = var_ctg_t_dn4;
        *var_ctg_t_rv_slot = var_ctg_t_rv;
        *var_delt_slot = var_delt;
        *var_delt_dn4_slot = var_delt_dn4;
        *var_delt_rv_slot = var_delt_rv;
        *var_dphibq_slot = var_dphibq;
        *var_dphibq_dn4_slot = var_dphibq_dn4;
        *var_dphibq_rv_slot = var_dphibq_rv;
        *var_eg_slot = var_eg;
        *var_eg_dn4_slot = var_eg_dn4;
        *var_eg_rv_slot = var_eg_rv;
        *var_g_0_ac_slot = var_g_0_ac;
        *var_g_0_ac_dn4_slot = var_g_0_ac_dn4;
        *var_g_0_ac_rv_slot = var_g_0_ac_rv;
        *var_g_0_dc_slot = var_g_0_dc;
        *var_g_0_dc_dn4_slot = var_g_0_dc_dn4;
        *var_g_0_dc_rv_slot = var_g_0_dc_rv;
        *var_guard1024_slot = var_guard1024;
        *var_guard1024_rv_slot = var_guard1024_rv;
        *var_guard1025_slot = var_guard1025;
        *var_guard1025_rv_slot = var_guard1025_rv;
        *var_guard1026_slot = var_guard1026;
        *var_guard1026_rv_slot = var_guard1026_rv;
        *var_inv_phit_slot = var_inv_phit;
        *var_inv_phit_dn4_slot = var_inv_phit_dn4;
        *var_inv_phit_rv_slot = var_inv_phit_rv;
        *var_kp_slot = var_kp;
        *var_kp_dn4_slot = var_kp_dn4;
        *var_kp_rv_slot = var_kp_rv;
        *var_ln_rtn_slot = var_ln_rtn;
        *var_ln_rtn_dn4_slot = var_ln_rtn_dn4;
        *var_ln_rtn_rv_slot = var_ln_rtn_rv;
        *var_np_slot = var_np;
        *var_np_rv_slot = var_np_rv;
        *var_phib_ac_slot = var_phib_ac;
        *var_phib_ac_dn4_slot = var_phib_ac_dn4;
        *var_phib_ac_rv_slot = var_phib_ac_rv;
        *var_phib_dc_slot = var_phib_dc;
        *var_phib_dc_dn4_slot = var_phib_dc_dn4;
        *var_phib_dc_rv_slot = var_phib_dc_rv;
        *var_phibfac_slot = var_phibfac;
        *var_phibfac_dn4_slot = var_phibfac_dn4;
        *var_phibfac_rv_slot = var_phibfac_rv;
        *var_phit_slot = var_phit;
        *var_phit_dn4_slot = var_phit_dn4;
        *var_phit_rv_slot = var_phit_rv;
        *var_phix1_ac_slot = var_phix1_ac;
        *var_phix1_ac_dn4_slot = var_phix1_ac_dn4;
        *var_phix1_ac_rv_slot = var_phix1_ac_rv;
        *var_phix1_dc_slot = var_phix1_dc;
        *var_phix1_dc_dn4_slot = var_phix1_dc_dn4;
        *var_phix1_dc_rv_slot = var_phix1_dc_rv;
        *var_phix2_slot = var_phix2;
        *var_phix2_dn4_slot = var_phix2_dn4;
        *var_phix2_rv_slot = var_phix2_rv;
        *var_phix_ac_slot = var_phix_ac;
        *var_phix_ac_dn4_slot = var_phix_ac_dn4;
        *var_phix_ac_rv_slot = var_phix_ac_rv;
        *var_phix_dc_slot = var_phix_dc;
        *var_phix_dc_dn4_slot = var_phix_dc_dn4;
        *var_phix_dc_rv_slot = var_phix_dc_rv;
        *var_qb0_slot = var_qb0;
        *var_qb0_dn4_slot = var_qb0_dn4;
        *var_qb0_rv_slot = var_qb0_rv;
        *var_qlim2_slot = var_qlim2;
        *var_qlim2_dn4_slot = var_qlim2_dn4;
        *var_qlim2_rv_slot = var_qlim2_rv;
        *var_rtn_slot = var_rtn;
        *var_rtn_dn4_slot = var_rtn_dn4;
        *var_rtn_rv_slot = var_rtn_rv;
        *var_sqrt_phib_dc_slot = var_sqrt_phib_dc;
        *var_sqrt_phib_dc_dn4_slot = var_sqrt_phib_dc_dn4;
        *var_sqrt_phib_dc_rv_slot = var_sqrt_phib_dc_rv;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp1_rv_slot = var_temp1_rv;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp2_rv_slot = var_temp2_rv;
        *var_tf_bet_slot = var_tf_bet;
        *var_tf_bet_dn4_slot = var_tf_bet_dn4;
        *var_tf_bet_rv_slot = var_tf_bet_rv;
        *var_tf_ct_slot = var_tf_ct;
        *var_tf_ct_dn4_slot = var_tf_ct_dn4;
        *var_tf_ct_rv_slot = var_tf_ct_rv;
        *var_tf_mue_slot = var_tf_mue;
        *var_tf_mue_dn4_slot = var_tf_mue_dn4;
        *var_tf_mue_rv_slot = var_tf_mue_rv;
        *var_themu_t_slot = var_themu_t;
        *var_themu_t_dn4_slot = var_themu_t_dn4;
        *var_themu_t_rv_slot = var_themu_t_rv;
        *var_tkd_slot = var_tkd;
        *var_tkd_dn4_slot = var_tkd_dn4;
        *var_tkd_rv_slot = var_tkd_rv;
        *var_tkd_sq_slot = var_tkd_sq;
        *var_tkd_sq_dn4_slot = var_tkd_sq_dn4;
        *var_tkd_sq_rv_slot = var_tkd_sq_rv;
        *var_us1_slot = var_us1;
        *var_us1_dn4_slot = var_us1_dn4;
        *var_us1_rv_slot = var_us1_rv;
        *var_us21_slot = var_us21;
        *var_us21_dn4_slot = var_us21_dn4;
        *var_us21_rv_slot = var_us21_rv;
        *var_vfb_t_slot = var_vfb_t;
        *var_vfb_t_dn4_slot = var_vfb_t_dn4;
        *var_vfb_t_rv_slot = var_vfb_t_rv;
    }

    pub(super) fn stamp_reactive_block_22(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_a2_i: f64,
        var_bet_i: f64,
        var_bet_i_dn4: f64,
        var_betnedge_i: f64,
        var_chnl_type: f64,
        var_coxprime: f64,
        var_cs_i: f64,
        var_ctedge_i: f64,
        var_delt: f64,
        var_delt_dn4: f64,
        var_delvtoedge_i: f64,
        var_dphibedge_i: f64,
        var_eg: f64,
        var_eg_dn4: f64,
        var_epssi: f64,
        var_factuoedge_i: f64,
        var_fnt_i: f64,
        var_inv_phit: f64,
        var_inv_phit_dn4: f64,
        var_ln_rtn: f64,
        var_ln_rtn_dn4: f64,
        var_mue_i: f64,
        var_neffedge_i: f64,
        var_phibfac: f64,
        var_phibfac_dn4: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_rs_i: f64,
        var_rtn: f64,
        var_rtn_dn4: f64,
        var_sta2_i: f64,
        var_stbetedge_i: f64,
        var_stcs_i: f64,
        var_strs_i: f64,
        var_stthecs_i: f64,
        var_stthesat_i: f64,
        var_stvfbedge_i: f64,
        var_stxcor_i: f64,
        var_tf_mue: f64,
        var_tf_mue_dn4: f64,
        var_thecs_i: f64,
        var_thesat_i: f64,
        var_thesatac_i: f64,
        var_tkd: f64,
        var_tkd_dn4: f64,
        var_vfbedge_i: f64,
        var_xcor_i: f64,
        var_a2_t_slot: &mut f64,
        var_a2_t_dn4_slot: &mut f64,
        var_a2_t_rv_slot: &mut f64,
        var_aphiedge_slot: &mut f64,
        var_aphiedge_dn4_slot: &mut f64,
        var_aphiedge_rv_slot: &mut f64,
        var_betedge_i_slot: &mut f64,
        var_betedge_i_dn4_slot: &mut f64,
        var_betedge_i_rv_slot: &mut f64,
        var_betnedge_t_slot: &mut f64,
        var_betnedge_t_dn4_slot: &mut f64,
        var_betnedge_t_rv_slot: &mut f64,
        var_bphiedge_slot: &mut f64,
        var_bphiedge_dn4_slot: &mut f64,
        var_bphiedge_rv_slot: &mut f64,
        var_cs_t_slot: &mut f64,
        var_cs_t_dn4_slot: &mut f64,
        var_cs_t_rv_slot: &mut f64,
        var_gfedge_slot: &mut f64,
        var_gfedge2_slot: &mut f64,
        var_gfedge2_dn4_slot: &mut f64,
        var_gfedge2_rv_slot: &mut f64,
        var_gfedge_dn4_slot: &mut f64,
        var_gfedge_rv_slot: &mut f64,
        var_guard1027_slot: &mut f64,
        var_guard1027_rv_slot: &mut f64,
        var_guard1028_slot: &mut f64,
        var_guard1028_rv_slot: &mut f64,
        var_lngfedge2_slot: &mut f64,
        var_lngfedge2_dn4_slot: &mut f64,
        var_lngfedge2_rv_slot: &mut f64,
        var_mue_t_slot: &mut f64,
        var_mue_t_dn4_slot: &mut f64,
        var_mue_t_rv_slot: &mut f64,
        var_nt_slot: &mut f64,
        var_nt_dn4_slot: &mut f64,
        var_nt_rv_slot: &mut f64,
        var_phibedge_slot: &mut f64,
        var_phibedge_dn4_slot: &mut f64,
        var_phibedge_rv_slot: &mut f64,
        var_phit0edge_slot: &mut f64,
        var_phit0edge_dn4_slot: &mut f64,
        var_phit0edge_rv_slot: &mut f64,
        var_phix1edge_slot: &mut f64,
        var_phix1edge_dn4_slot: &mut f64,
        var_phix1edge_rv_slot: &mut f64,
        var_phix2edge_slot: &mut f64,
        var_phix2edge_dn4_slot: &mut f64,
        var_phix2edge_rv_slot: &mut f64,
        var_phixedge_slot: &mut f64,
        var_phixedge_dn4_slot: &mut f64,
        var_phixedge_rv_slot: &mut f64,
        var_rs_t_slot: &mut f64,
        var_rs_t_dn4_slot: &mut f64,
        var_rs_t_rv_slot: &mut f64,
        var_tf_betedge_slot: &mut f64,
        var_tf_betedge_dn4_slot: &mut f64,
        var_tf_betedge_rv_slot: &mut f64,
        var_tf_cs_slot: &mut f64,
        var_tf_cs_dn4_slot: &mut f64,
        var_tf_cs_rv_slot: &mut f64,
        var_tf_ther_slot: &mut f64,
        var_tf_ther_dn4_slot: &mut f64,
        var_tf_ther_rv_slot: &mut f64,
        var_tf_thesat_slot: &mut f64,
        var_tf_thesat_dn4_slot: &mut f64,
        var_tf_thesat_rv_slot: &mut f64,
        var_tf_xcor_slot: &mut f64,
        var_tf_xcor_dn4_slot: &mut f64,
        var_tf_xcor_rv_slot: &mut f64,
        var_thecs_t_slot: &mut f64,
        var_thecs_t_dn4_slot: &mut f64,
        var_thecs_t_rv_slot: &mut f64,
        var_ther_i_slot: &mut f64,
        var_ther_i_dn4_slot: &mut f64,
        var_ther_i_rv_slot: &mut f64,
        var_thesat_t_slot: &mut f64,
        var_thesat_t_dn4_slot: &mut f64,
        var_thesat_t_rv_slot: &mut f64,
        var_thesatac_t_slot: &mut f64,
        var_thesatac_t_dn4_slot: &mut f64,
        var_thesatac_t_rv_slot: &mut f64,
        var_v_ds_slot: &mut f64,
        var_v_ds_dn7_slot: &mut f64,
        var_v_ds_dn8_slot: &mut f64,
        var_v_ds_rv_slot: &mut f64,
        var_v_gs_slot: &mut f64,
        var_v_gs_dn6_slot: &mut f64,
        var_v_gs_dn7_slot: &mut f64,
        var_v_gs_dn8_slot: &mut f64,
        var_v_gs_rv_slot: &mut f64,
        var_v_sb_slot: &mut f64,
        var_v_sb_dn7_slot: &mut f64,
        var_v_sb_dn8_slot: &mut f64,
        var_v_sb_dn9_slot: &mut f64,
        var_v_sb_rv_slot: &mut f64,
        var_vfbedge_t_slot: &mut f64,
        var_vfbedge_t_dn4_slot: &mut f64,
        var_vfbedge_t_rv_slot: &mut f64,
        var_vjun_s_slot: &mut f64,
        var_vjun_s_dn11_slot: &mut f64,
        var_vjun_s_dn7_slot: &mut f64,
        var_vjun_s_rv_slot: &mut f64,
        var_xcor_t_slot: &mut f64,
        var_xcor_t_dn4_slot: &mut f64,
        var_xcor_t_rv_slot: &mut f64,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let mut var_a2_t: f64 = *var_a2_t_slot;
        let mut var_a2_t_dn4: f64 = *var_a2_t_dn4_slot;
        let mut var_a2_t_rv: f64 = *var_a2_t_rv_slot;
        let mut var_aphiedge: f64 = *var_aphiedge_slot;
        let mut var_aphiedge_dn4: f64 = *var_aphiedge_dn4_slot;
        let mut var_aphiedge_rv: f64 = *var_aphiedge_rv_slot;
        let mut var_betedge_i: f64 = *var_betedge_i_slot;
        let mut var_betedge_i_dn4: f64 = *var_betedge_i_dn4_slot;
        let mut var_betedge_i_rv: f64 = *var_betedge_i_rv_slot;
        let mut var_betnedge_t: f64 = *var_betnedge_t_slot;
        let mut var_betnedge_t_dn4: f64 = *var_betnedge_t_dn4_slot;
        let mut var_betnedge_t_rv: f64 = *var_betnedge_t_rv_slot;
        let mut var_bphiedge: f64 = *var_bphiedge_slot;
        let mut var_bphiedge_dn4: f64 = *var_bphiedge_dn4_slot;
        let mut var_bphiedge_rv: f64 = *var_bphiedge_rv_slot;
        let mut var_cs_t: f64 = *var_cs_t_slot;
        let mut var_cs_t_dn4: f64 = *var_cs_t_dn4_slot;
        let mut var_cs_t_rv: f64 = *var_cs_t_rv_slot;
        let mut var_gfedge: f64 = *var_gfedge_slot;
        let mut var_gfedge2: f64 = *var_gfedge2_slot;
        let mut var_gfedge2_dn4: f64 = *var_gfedge2_dn4_slot;
        let mut var_gfedge2_rv: f64 = *var_gfedge2_rv_slot;
        let mut var_gfedge_dn4: f64 = *var_gfedge_dn4_slot;
        let mut var_gfedge_rv: f64 = *var_gfedge_rv_slot;
        let mut var_guard1027: f64 = *var_guard1027_slot;
        let mut var_guard1027_rv: f64 = *var_guard1027_rv_slot;
        let mut var_guard1028: f64 = *var_guard1028_slot;
        let mut var_guard1028_rv: f64 = *var_guard1028_rv_slot;
        let mut var_lngfedge2: f64 = *var_lngfedge2_slot;
        let mut var_lngfedge2_dn4: f64 = *var_lngfedge2_dn4_slot;
        let mut var_lngfedge2_rv: f64 = *var_lngfedge2_rv_slot;
        let mut var_mue_t: f64 = *var_mue_t_slot;
        let mut var_mue_t_dn4: f64 = *var_mue_t_dn4_slot;
        let mut var_mue_t_rv: f64 = *var_mue_t_rv_slot;
        let mut var_nt: f64 = *var_nt_slot;
        let mut var_nt_dn4: f64 = *var_nt_dn4_slot;
        let mut var_nt_rv: f64 = *var_nt_rv_slot;
        let mut var_phibedge: f64 = *var_phibedge_slot;
        let mut var_phibedge_dn4: f64 = *var_phibedge_dn4_slot;
        let mut var_phibedge_rv: f64 = *var_phibedge_rv_slot;
        let mut var_phit0edge: f64 = *var_phit0edge_slot;
        let mut var_phit0edge_dn4: f64 = *var_phit0edge_dn4_slot;
        let mut var_phit0edge_rv: f64 = *var_phit0edge_rv_slot;
        let mut var_phix1edge: f64 = *var_phix1edge_slot;
        let mut var_phix1edge_dn4: f64 = *var_phix1edge_dn4_slot;
        let mut var_phix1edge_rv: f64 = *var_phix1edge_rv_slot;
        let mut var_phix2edge: f64 = *var_phix2edge_slot;
        let mut var_phix2edge_dn4: f64 = *var_phix2edge_dn4_slot;
        let mut var_phix2edge_rv: f64 = *var_phix2edge_rv_slot;
        let mut var_phixedge: f64 = *var_phixedge_slot;
        let mut var_phixedge_dn4: f64 = *var_phixedge_dn4_slot;
        let mut var_phixedge_rv: f64 = *var_phixedge_rv_slot;
        let mut var_rs_t: f64 = *var_rs_t_slot;
        let mut var_rs_t_dn4: f64 = *var_rs_t_dn4_slot;
        let mut var_rs_t_rv: f64 = *var_rs_t_rv_slot;
        let mut var_tf_betedge: f64 = *var_tf_betedge_slot;
        let mut var_tf_betedge_dn4: f64 = *var_tf_betedge_dn4_slot;
        let mut var_tf_betedge_rv: f64 = *var_tf_betedge_rv_slot;
        let mut var_tf_cs: f64 = *var_tf_cs_slot;
        let mut var_tf_cs_dn4: f64 = *var_tf_cs_dn4_slot;
        let mut var_tf_cs_rv: f64 = *var_tf_cs_rv_slot;
        let mut var_tf_ther: f64 = *var_tf_ther_slot;
        let mut var_tf_ther_dn4: f64 = *var_tf_ther_dn4_slot;
        let mut var_tf_ther_rv: f64 = *var_tf_ther_rv_slot;
        let mut var_tf_thesat: f64 = *var_tf_thesat_slot;
        let mut var_tf_thesat_dn4: f64 = *var_tf_thesat_dn4_slot;
        let mut var_tf_thesat_rv: f64 = *var_tf_thesat_rv_slot;
        let mut var_tf_xcor: f64 = *var_tf_xcor_slot;
        let mut var_tf_xcor_dn4: f64 = *var_tf_xcor_dn4_slot;
        let mut var_tf_xcor_rv: f64 = *var_tf_xcor_rv_slot;
        let mut var_thecs_t: f64 = *var_thecs_t_slot;
        let mut var_thecs_t_dn4: f64 = *var_thecs_t_dn4_slot;
        let mut var_thecs_t_rv: f64 = *var_thecs_t_rv_slot;
        let mut var_ther_i: f64 = *var_ther_i_slot;
        let mut var_ther_i_dn4: f64 = *var_ther_i_dn4_slot;
        let mut var_ther_i_rv: f64 = *var_ther_i_rv_slot;
        let mut var_thesat_t: f64 = *var_thesat_t_slot;
        let mut var_thesat_t_dn4: f64 = *var_thesat_t_dn4_slot;
        let mut var_thesat_t_rv: f64 = *var_thesat_t_rv_slot;
        let mut var_thesatac_t: f64 = *var_thesatac_t_slot;
        let mut var_thesatac_t_dn4: f64 = *var_thesatac_t_dn4_slot;
        let mut var_thesatac_t_rv: f64 = *var_thesatac_t_rv_slot;
        let mut var_v_ds: f64 = *var_v_ds_slot;
        let mut var_v_ds_dn7: f64 = *var_v_ds_dn7_slot;
        let mut var_v_ds_dn8: f64 = *var_v_ds_dn8_slot;
        let mut var_v_ds_rv: f64 = *var_v_ds_rv_slot;
        let mut var_v_gs: f64 = *var_v_gs_slot;
        let mut var_v_gs_dn6: f64 = *var_v_gs_dn6_slot;
        let mut var_v_gs_dn7: f64 = *var_v_gs_dn7_slot;
        let mut var_v_gs_dn8: f64 = *var_v_gs_dn8_slot;
        let mut var_v_gs_rv: f64 = *var_v_gs_rv_slot;
        let mut var_v_sb: f64 = *var_v_sb_slot;
        let mut var_v_sb_dn7: f64 = *var_v_sb_dn7_slot;
        let mut var_v_sb_dn8: f64 = *var_v_sb_dn8_slot;
        let mut var_v_sb_dn9: f64 = *var_v_sb_dn9_slot;
        let mut var_v_sb_rv: f64 = *var_v_sb_rv_slot;
        let mut var_vfbedge_t: f64 = *var_vfbedge_t_slot;
        let mut var_vfbedge_t_dn4: f64 = *var_vfbedge_t_dn4_slot;
        let mut var_vfbedge_t_rv: f64 = *var_vfbedge_t_rv_slot;
        let mut var_vjun_s: f64 = *var_vjun_s_slot;
        let mut var_vjun_s_dn11: f64 = *var_vjun_s_dn11_slot;
        let mut var_vjun_s_dn7: f64 = *var_vjun_s_dn7_slot;
        let mut var_vjun_s_rv: f64 = *var_vjun_s_rv_slot;
        let mut var_xcor_t: f64 = *var_xcor_t_slot;
        let mut var_xcor_t_dn4: f64 = *var_xcor_t_dn4_slot;
        let mut var_xcor_t_rv: f64 = *var_xcor_t_rv_slot;

        let assign40010_e53362: f64 = (var_mue_i * var_tf_mue);
        var_mue_t = assign40010_e53362;
        var_mue_t_dn4 = (var_mue_i * var_tf_mue_dn4);
        var_mue_t_rv = 0.0;

        let assign40020_e53366: f64 = (var_stthecs_i * var_ln_rtn);
        let assign40020_e53367: f64 = (assign40020_e53366).exp();
        let assign40020_e53368: f64 = (var_thecs_i * assign40020_e53367);
        var_thecs_t = assign40020_e53368;
        var_thecs_t_dn4 = (var_thecs_i * (assign40020_e53367 * (var_stthecs_i * var_ln_rtn_dn4)));
        var_thecs_t_rv = 0.0;

        let assign40030_e53371: f64 = (var_stcs_i * var_ln_rtn);
        let assign40030_e53372: f64 = (assign40030_e53371).exp();
        var_tf_cs = assign40030_e53372;
        var_tf_cs_dn4 = (assign40030_e53372 * (var_stcs_i * var_ln_rtn_dn4));
        var_tf_cs_rv = 0.0;

        let assign40040_e53375: f64 = (var_cs_i * var_tf_cs);
        var_cs_t = assign40040_e53375;
        var_cs_t_dn4 = (var_cs_i * var_tf_cs_dn4);
        var_cs_t_rv = 0.0;

        let assign40050_e53378: f64 = (var_stxcor_i * var_ln_rtn);
        let assign40050_e53379: f64 = (assign40050_e53378).exp();
        var_tf_xcor = assign40050_e53379;
        var_tf_xcor_dn4 = (assign40050_e53379 * (var_stxcor_i * var_ln_rtn_dn4));
        var_tf_xcor_rv = 0.0;

        let assign40060_e53382: f64 = (var_xcor_i * var_tf_xcor);
        var_xcor_t = assign40060_e53382;
        var_xcor_t_dn4 = (var_xcor_i * var_tf_xcor_dn4);
        var_xcor_t_rv = 0.0;

        let assign40070_e53385: f64 = (var_strs_i * var_ln_rtn);
        let assign40070_e53386: f64 = (assign40070_e53385).exp();
        var_tf_ther = assign40070_e53386;
        var_tf_ther_dn4 = (assign40070_e53386 * (var_strs_i * var_ln_rtn_dn4));
        var_tf_ther_rv = 0.0;

        let assign40080_e53389: f64 = (var_rs_i * var_tf_ther);
        var_rs_t = assign40080_e53389;
        var_rs_t_dn4 = (var_rs_i * var_tf_ther_dn4);
        var_rs_t_rv = 0.0;

        let assign40090_e53392: f64 = (2.0 * var_bet_i);
        let assign40090_e53394: f64 = (assign40090_e53392 * var_rs_t);
        var_ther_i = assign40090_e53394;
        var_ther_i_dn4 = (((2.0 * var_bet_i_dn4) * var_rs_t) + (assign40090_e53392 * var_rs_t_dn4));
        var_ther_i_rv = 0.0;

        let assign40100_e53397: f64 = (var_stthesat_i * var_ln_rtn);
        let assign40100_e53398: f64 = (assign40100_e53397).exp();
        var_tf_thesat = assign40100_e53398;
        var_tf_thesat_dn4 = (assign40100_e53398 * (var_stthesat_i * var_ln_rtn_dn4));
        var_tf_thesat_rv = 0.0;

        let assign40110_e53401: f64 = (var_thesat_i * var_tf_thesat);
        var_thesat_t = assign40110_e53401;
        var_thesat_t_dn4 = (var_thesat_i * var_tf_thesat_dn4);
        var_thesat_t_rv = 0.0;

        let assign40120_e53404: f64 = (var_thesatac_i * var_tf_thesat);
        var_thesatac_t = assign40120_e53404;
        var_thesatac_t_dn4 = (var_thesatac_i * var_tf_thesat_dn4);
        var_thesatac_t_rv = 0.0;

        let assign40130_e53407: f64 = (-var_sta2_i);
        let assign40130_e53409: f64 = (assign40130_e53407 * var_ln_rtn);
        let assign40130_e53410: f64 = (assign40130_e53409).exp();
        let assign40130_e53411: f64 = (var_a2_i * assign40130_e53410);
        var_a2_t = assign40130_e53411;
        var_a2_t_dn4 = (var_a2_i * (assign40130_e53410 * (assign40130_e53407 * var_ln_rtn_dn4)));
        var_a2_t_rv = 0.0;

        let assign40140_e53414: f64 = (var_fnt_i * 4.0);
        let assign40140_e53416: f64 = (assign40140_e53414 * 1.3806505e-23);
        let assign40140_e53418: f64 = (assign40140_e53416 * var_tkd);
        var_nt = assign40140_e53418;
        var_nt_dn4 = (assign40140_e53416 * var_tkd_dn4);
        var_nt_rv = 0.0;

        let assign40160_e53432: f64 = if ((p.p46 != 0.0) && (var_betnedge_i > 0.0)) { 1.0 } else { 0.0 };
        var_guard1027 = assign40160_e53432;
        var_guard1027_rv = 0.0;

        let (assign40170_e53442, assign40170_e53442_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40170_e53437: f64 = (var_stvfbedge_i * var_delt);
        let assign40170_e53438: f64 = (var_vfbedge_i + assign40170_e53437);
        let assign40170_e53440: f64 = (assign40170_e53438 + var_delvtoedge_i);
        (assign40170_e53440, (var_stvfbedge_i * var_delt_dn4),)
    } else {
        (var_vfbedge_t, var_vfbedge_t_dn4,)
    }
};
        var_vfbedge_t = assign40170_e53442;
        var_vfbedge_t_dn4 = assign40170_e53442_d_n4;
        var_vfbedge_t_rv = 0.0;

        let (assign40180_e53449, assign40180_e53449_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40180_e53446: f64 = (var_stbetedge_i * var_ln_rtn);
        let assign40180_e53447: f64 = (assign40180_e53446).exp();
        (assign40180_e53447, (assign40180_e53447 * (var_stbetedge_i * var_ln_rtn_dn4)),)
    } else {
        (var_tf_betedge, var_tf_betedge_dn4,)
    }
};
        var_tf_betedge = assign40180_e53449;
        var_tf_betedge_dn4 = assign40180_e53449_d_n4;
        var_tf_betedge_rv = 0.0;

        let (assign40190_e53455, assign40190_e53455_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40190_e53453: f64 = (var_betnedge_i * var_tf_betedge);
        (assign40190_e53453, (var_betnedge_i * var_tf_betedge_dn4),)
    } else {
        (var_betnedge_t, var_betnedge_t_dn4,)
    }
};
        var_betnedge_t = assign40190_e53455;
        var_betnedge_t_dn4 = assign40190_e53455_d_n4;
        var_betnedge_t_rv = 0.0;

        let (assign40200_e53463, assign40200_e53463_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40200_e53459: f64 = (var_factuoedge_i * var_betnedge_t);
        let assign40200_e53461: f64 = (assign40200_e53459 * var_coxprime);
        (assign40200_e53461, ((var_factuoedge_i * var_betnedge_t_dn4) * var_coxprime),)
    } else {
        (var_betedge_i, var_betedge_i_dn4,)
    }
};
        var_betedge_i = assign40200_e53463;
        var_betedge_i_dn4 = assign40200_e53463_d_n4;
        var_betedge_i_rv = 0.0;

        let (assign40210_e53473, assign40210_e53473_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40210_e53469: f64 = (var_ctedge_i * var_rtn);
        let assign40210_e53470: f64 = (1.0 + assign40210_e53469);
        let assign40210_e53471: f64 = (var_phit * assign40210_e53470);
        (assign40210_e53471, ((var_phit_dn4 * assign40210_e53470) + (var_phit * (var_ctedge_i * var_rtn_dn4))),)
    } else {
        (var_phit0edge, var_phit0edge_dn4,)
    }
};
        var_phit0edge = assign40210_e53473;
        var_phit0edge_dn4 = assign40210_e53473_d_n4;
        var_phit0edge_rv = 0.0;

        let (assign40220_e53493, assign40220_e53493_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40220_e53477: f64 = (var_eg + var_dphibedge_i);
        let assign40220_e53480: f64 = (2.0 * var_phit0edge);
        let assign40220_e53484: f64 = (-0.75);
        let assign40220_e53485: f64 = (var_phibfac).powf(assign40220_e53484);
        let assign40220_e53486: f64 = (var_neffedge_i * assign40220_e53485);
        let assign40220_e53488: f64 = (assign40220_e53486 * 4e-26);
        let assign40220_e53489: f64 = (assign40220_e53488).ln();
        let assign40220_e53490: f64 = (assign40220_e53480 * assign40220_e53489);
        let assign40220_e53491: f64 = (assign40220_e53477 + assign40220_e53490);
        (assign40220_e53491, (var_eg_dn4 + (((2.0 * var_phit0edge_dn4) * assign40220_e53489) + (assign40220_e53480 * (((var_neffedge_i * if 0.0 == 0.0 && ((assign40220_e53484) as f64).is_finite() && ((assign40220_e53484) as f64).fract() == 0.0 { if assign40220_e53484 == 0.0 { 0.0 } else { (assign40220_e53484 * ((var_phibfac).powf(assign40220_e53484 - 1.0) * var_phibfac_dn4)) } } else { (assign40220_e53485 * (assign40220_e53484 * (var_phibfac_dn4 / var_phibfac))) }) * 4e-26) / assign40220_e53488)))),)
    } else {
        (var_phibedge, var_phibedge_dn4,)
    }
};
        var_phibedge = assign40220_e53493;
        var_phibedge_dn4 = assign40220_e53493_d_n4;
        var_phibedge_rv = 0.0;

        let (assign40230_e53502, assign40230_e53502_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let (assign40230_e53500, assign40230_e53500_d_n4,) = {
            if (var_phibedge > 0.05) {
                (var_phibedge, var_phibedge_dn4,)
            } else {
                (0.05, 0.0,)
            }
        };
        (assign40230_e53500, assign40230_e53500_d_n4,)
    } else {
        (var_phibedge, var_phibedge_dn4,)
    }
};
        var_phibedge = assign40230_e53502;
        var_phibedge_dn4 = assign40230_e53502_d_n4;
        var_phibedge_rv = 0.0;

        let (assign40240_e53517, assign40240_e53517_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40240_e53506: f64 = (2.0 * 1.6021918e-19);
        let assign40240_e53508: f64 = (assign40240_e53506 * var_neffedge_i);
        let assign40240_e53510: f64 = (assign40240_e53508 * var_epssi);
        let assign40240_e53512: f64 = (assign40240_e53510 * var_inv_phit);
        let assign40240_e53513: f64 = (assign40240_e53512).sqrt();
        let assign40240_e53515: f64 = (assign40240_e53513 / var_coxprime);
        (assign40240_e53515, (((assign40240_e53510 * var_inv_phit_dn4) / (2.0 * assign40240_e53513)) / var_coxprime),)
    } else {
        (var_gfedge, var_gfedge_dn4,)
    }
};
        var_gfedge = assign40240_e53517;
        var_gfedge_dn4 = assign40240_e53517_d_n4;
        var_gfedge_rv = 0.0;

        let (assign40250_e53523, assign40250_e53523_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40250_e53521: f64 = (var_gfedge * var_gfedge);
        (assign40250_e53521, ((var_gfedge_dn4 * var_gfedge) + (var_gfedge * var_gfedge_dn4)),)
    } else {
        (var_gfedge2, var_gfedge2_dn4,)
    }
};
        var_gfedge2 = assign40250_e53523;
        var_gfedge2_dn4 = assign40250_e53523_d_n4;
        var_gfedge2_rv = 0.0;

        let (assign40260_e53528, assign40260_e53528_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40260_e53526: f64 = (var_gfedge2).ln();
        (assign40260_e53526, (var_gfedge2_dn4 / var_gfedge2),)
    } else {
        (var_lngfedge2, var_lngfedge2_dn4,)
    }
};
        var_lngfedge2 = assign40260_e53528;
        var_lngfedge2_dn4 = assign40260_e53528_d_n4;
        var_lngfedge2_rv = 0.0;

        let (assign40270_e53534, assign40270_e53534_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40270_e53532: f64 = (0.95 * var_phibedge);
        (assign40270_e53532, (0.95 * var_phibedge_dn4),)
    } else {
        (var_phixedge, var_phixedge_dn4,)
    }
};
        var_phixedge = assign40270_e53534;
        var_phixedge_dn4 = assign40270_e53534_d_n4;
        var_phixedge_rv = 0.0;

        let (assign40280_e53542, assign40280_e53542_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40280_e53538: f64 = (0.0025 * var_phibedge);
        let assign40280_e53540: f64 = (assign40280_e53538 * var_phibedge);
        (assign40280_e53540, (((0.0025 * var_phibedge_dn4) * var_phibedge) + (assign40280_e53538 * var_phibedge_dn4)),)
    } else {
        (var_aphiedge, var_aphiedge_dn4,)
    }
};
        var_aphiedge = assign40280_e53542;
        var_aphiedge_dn4 = assign40280_e53542_d_n4;
        var_aphiedge_rv = 0.0;

        let (assign40290_e53546, assign40290_e53546_d_n4,) = {
    if (var_guard1027 != 0.0) {
        (var_aphiedge, var_aphiedge_dn4,)
    } else {
        (var_bphiedge, var_bphiedge_dn4,)
    }
};
        var_bphiedge = assign40290_e53546;
        var_bphiedge_dn4 = assign40290_e53546_d_n4;
        var_bphiedge_rv = 0.0;

        let (assign40300_e53553, assign40300_e53553_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40300_e53550: f64 = (var_bphiedge).sqrt();
        let assign40300_e53551: f64 = (0.5 * assign40300_e53550);
        (assign40300_e53551, (0.5 * (var_bphiedge_dn4 / (2.0 * assign40300_e53550))),)
    } else {
        (var_phix2edge, var_phix2edge_dn4,)
    }
};
        var_phix2edge = assign40300_e53553;
        var_phix2edge_dn4 = assign40300_e53553_d_n4;
        var_phix2edge_rv = 0.0;

        let (assign40310_e53578, assign40310_e53578_d_n4,) = {
    if (var_guard1027 != 0.0) {
        let assign40310_e53558: f64 = (var_phixedge - var_phix2edge);
        let assign40310_e53560: f64 = assign40310_e53558;
        let assign40310_e53563: f64 = (var_phixedge - var_phix2edge);
        let assign40310_e53565: f64 = assign40310_e53563;
        let assign40310_e53568: f64 = (var_phixedge - var_phix2edge);
        let assign40310_e53570: f64 = assign40310_e53568;
        let assign40310_e53571: f64 = (assign40310_e53565 * assign40310_e53570);
        let assign40310_e53573: f64 = (assign40310_e53571 + var_aphiedge);
        let assign40310_e53574: f64 = (assign40310_e53573).sqrt();
        let assign40310_e53575: f64 = (assign40310_e53560 - assign40310_e53574);
        let assign40310_e53576: f64 = (0.5 * assign40310_e53575);
        (assign40310_e53576, (0.5 * ((var_phixedge_dn4 - var_phix2edge_dn4) - (((((var_phixedge_dn4 - var_phix2edge_dn4) * assign40310_e53570) + (assign40310_e53565 * (var_phixedge_dn4 - var_phix2edge_dn4))) + var_aphiedge_dn4) / (2.0 * assign40310_e53574)))),)
    } else {
        (var_phix1edge, var_phix1edge_dn4,)
    }
};
        var_phix1edge = assign40310_e53578;
        var_phix1edge_dn4 = assign40310_e53578_d_n4;
        var_phix1edge_rv = 0.0;

        let (assign40340_e53603, assign40340_e53603_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (var_vfbedge_t, var_vfbedge_t_dn4,)
    }
};
        var_vfbedge_t = assign40340_e53603;
        var_vfbedge_t_dn4 = assign40340_e53603_d_n4;
        var_vfbedge_t_rv = 0.0;

        let (assign40350_e53608, assign40350_e53608_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (1.0, 0.0,)
    } else {
        (var_tf_betedge, var_tf_betedge_dn4,)
    }
};
        var_tf_betedge = assign40350_e53608;
        var_tf_betedge_dn4 = assign40350_e53608_d_n4;
        var_tf_betedge_rv = 0.0;

        let (assign40360_e53613, assign40360_e53613_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (var_betnedge_t, var_betnedge_t_dn4,)
    }
};
        var_betnedge_t = assign40360_e53613;
        var_betnedge_t_dn4 = assign40360_e53613_d_n4;
        var_betnedge_t_rv = 0.0;

        let (assign40370_e53618, assign40370_e53618_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (var_betedge_i, var_betedge_i_dn4,)
    }
};
        var_betedge_i = assign40370_e53618;
        var_betedge_i_dn4 = assign40370_e53618_d_n4;
        var_betedge_i_rv = 0.0;

        let (assign40380_e53623, assign40380_e53623_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (var_phit, var_phit_dn4,)
    } else {
        (var_phit0edge, var_phit0edge_dn4,)
    }
};
        var_phit0edge = assign40380_e53623;
        var_phit0edge_dn4 = assign40380_e53623_d_n4;
        var_phit0edge_rv = 0.0;

        let (assign40390_e53628, assign40390_e53628_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (var_phibedge, var_phibedge_dn4,)
    }
};
        var_phibedge = assign40390_e53628;
        var_phibedge_dn4 = assign40390_e53628_d_n4;
        var_phibedge_rv = 0.0;

        let (assign40400_e53633, assign40400_e53633_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (1.0, 0.0,)
    } else {
        (var_gfedge, var_gfedge_dn4,)
    }
};
        var_gfedge = assign40400_e53633;
        var_gfedge_dn4 = assign40400_e53633_d_n4;
        var_gfedge_rv = 0.0;

        let (assign40410_e53638, assign40410_e53638_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (1.0, 0.0,)
    } else {
        (var_gfedge2, var_gfedge2_dn4,)
    }
};
        var_gfedge2 = assign40410_e53638;
        var_gfedge2_dn4 = assign40410_e53638_d_n4;
        var_gfedge2_rv = 0.0;

        let (assign40420_e53643, assign40420_e53643_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (var_lngfedge2, var_lngfedge2_dn4,)
    }
};
        var_lngfedge2 = assign40420_e53643;
        var_lngfedge2_dn4 = assign40420_e53643_d_n4;
        var_lngfedge2_rv = 0.0;

        let (assign40430_e53648, assign40430_e53648_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (var_phixedge, var_phixedge_dn4,)
    }
};
        var_phixedge = assign40430_e53648;
        var_phixedge_dn4 = assign40430_e53648_d_n4;
        var_phixedge_rv = 0.0;

        let (assign40440_e53653, assign40440_e53653_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (var_aphiedge, var_aphiedge_dn4,)
    }
};
        var_aphiedge = assign40440_e53653;
        var_aphiedge_dn4 = assign40440_e53653_d_n4;
        var_aphiedge_rv = 0.0;

        let (assign40450_e53658, assign40450_e53658_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (var_bphiedge, var_bphiedge_dn4,)
    }
};
        var_bphiedge = assign40450_e53658;
        var_bphiedge_dn4 = assign40450_e53658_d_n4;
        var_bphiedge_rv = 0.0;

        let (assign40460_e53663, assign40460_e53663_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (var_phix2edge, var_phix2edge_dn4,)
    }
};
        var_phix2edge = assign40460_e53663;
        var_phix2edge_dn4 = assign40460_e53663_d_n4;
        var_phix2edge_rv = 0.0;

        let (assign40470_e53668, assign40470_e53668_d_n4,) = {
    if (var_guard1027 == 0.0) {
        (0.0, 0.0,)
    } else {
        (var_phix1edge, var_phix1edge_dn4,)
    }
};
        var_phix1edge = assign40470_e53668;
        var_phix1edge_dn4 = assign40470_e53668_d_n4;
        var_phix1edge_rv = 0.0;

        let assign40500_e53681: f64 = 1.0;
        let assign40500_e53682: f64 = if var_chnl_type == assign40500_e53681 { 1.0 } else { 0.0 };
        var_guard1028 = assign40500_e53682;
        var_guard1028_rv = 0.0;

        let (assign40510_e53686, assign40510_e53686_d_n6, assign40510_e53686_d_n7, assign40510_e53686_d_n8,) = {
    if (var_guard1028 != 0.0) {
        ((nv6 - nv7), 1.0, -1.0, 0.0,)
    } else {
        (var_v_gs, var_v_gs_dn6, var_v_gs_dn7, var_v_gs_dn8,)
    }
};
        var_v_gs = assign40510_e53686;
        var_v_gs_dn6 = assign40510_e53686_d_n6;
        var_v_gs_dn7 = assign40510_e53686_d_n7;
        var_v_gs_dn8 = assign40510_e53686_d_n8;
        var_v_gs_rv = 0.0;

        let (assign40520_e53690, assign40520_e53690_d_n7, assign40520_e53690_d_n8,) = {
    if (var_guard1028 != 0.0) {
        ((nv8 - nv7), -1.0, 1.0,)
    } else {
        (var_v_ds, var_v_ds_dn7, var_v_ds_dn8,)
    }
};
        var_v_ds = assign40520_e53690;
        var_v_ds_dn7 = assign40520_e53690_d_n7;
        var_v_ds_dn8 = assign40520_e53690_d_n8;
        var_v_ds_rv = 0.0;

        let (assign40530_e53694, assign40530_e53694_d_n7, assign40530_e53694_d_n8, assign40530_e53694_d_n9,) = {
    if (var_guard1028 != 0.0) {
        ((nv7 - nv9), 1.0, 0.0, -1.0,)
    } else {
        (var_v_sb, var_v_sb_dn7, var_v_sb_dn8, var_v_sb_dn9,)
    }
};
        var_v_sb = assign40530_e53694;
        var_v_sb_dn7 = assign40530_e53694_d_n7;
        var_v_sb_dn8 = assign40530_e53694_d_n8;
        var_v_sb_dn9 = assign40530_e53694_d_n9;
        var_v_sb_rv = 0.0;

        let (assign40540_e53699, assign40540_e53699_d_n7, assign40540_e53699_d_n11,) = {
    if (var_guard1028 != 0.0) {
        let assign40540_e53697: f64 = (-(nv7 - nv11));
        (assign40540_e53697, (-1.0), 1.0,)
    } else {
        (var_vjun_s, var_vjun_s_dn7, var_vjun_s_dn11,)
    }
};
        var_vjun_s = assign40540_e53699;
        var_vjun_s_dn7 = assign40540_e53699_d_n7;
        var_vjun_s_dn11 = assign40540_e53699_d_n11;
        var_vjun_s_rv = 0.0;

        *var_a2_t_slot = var_a2_t;
        *var_a2_t_dn4_slot = var_a2_t_dn4;
        *var_a2_t_rv_slot = var_a2_t_rv;
        *var_aphiedge_slot = var_aphiedge;
        *var_aphiedge_dn4_slot = var_aphiedge_dn4;
        *var_aphiedge_rv_slot = var_aphiedge_rv;
        *var_betedge_i_slot = var_betedge_i;
        *var_betedge_i_dn4_slot = var_betedge_i_dn4;
        *var_betedge_i_rv_slot = var_betedge_i_rv;
        *var_betnedge_t_slot = var_betnedge_t;
        *var_betnedge_t_dn4_slot = var_betnedge_t_dn4;
        *var_betnedge_t_rv_slot = var_betnedge_t_rv;
        *var_bphiedge_slot = var_bphiedge;
        *var_bphiedge_dn4_slot = var_bphiedge_dn4;
        *var_bphiedge_rv_slot = var_bphiedge_rv;
        *var_cs_t_slot = var_cs_t;
        *var_cs_t_dn4_slot = var_cs_t_dn4;
        *var_cs_t_rv_slot = var_cs_t_rv;
        *var_gfedge_slot = var_gfedge;
        *var_gfedge2_slot = var_gfedge2;
        *var_gfedge2_dn4_slot = var_gfedge2_dn4;
        *var_gfedge2_rv_slot = var_gfedge2_rv;
        *var_gfedge_dn4_slot = var_gfedge_dn4;
        *var_gfedge_rv_slot = var_gfedge_rv;
        *var_guard1027_slot = var_guard1027;
        *var_guard1027_rv_slot = var_guard1027_rv;
        *var_guard1028_slot = var_guard1028;
        *var_guard1028_rv_slot = var_guard1028_rv;
        *var_lngfedge2_slot = var_lngfedge2;
        *var_lngfedge2_dn4_slot = var_lngfedge2_dn4;
        *var_lngfedge2_rv_slot = var_lngfedge2_rv;
        *var_mue_t_slot = var_mue_t;
        *var_mue_t_dn4_slot = var_mue_t_dn4;
        *var_mue_t_rv_slot = var_mue_t_rv;
        *var_nt_slot = var_nt;
        *var_nt_dn4_slot = var_nt_dn4;
        *var_nt_rv_slot = var_nt_rv;
        *var_phibedge_slot = var_phibedge;
        *var_phibedge_dn4_slot = var_phibedge_dn4;
        *var_phibedge_rv_slot = var_phibedge_rv;
        *var_phit0edge_slot = var_phit0edge;
        *var_phit0edge_dn4_slot = var_phit0edge_dn4;
        *var_phit0edge_rv_slot = var_phit0edge_rv;
        *var_phix1edge_slot = var_phix1edge;
        *var_phix1edge_dn4_slot = var_phix1edge_dn4;
        *var_phix1edge_rv_slot = var_phix1edge_rv;
        *var_phix2edge_slot = var_phix2edge;
        *var_phix2edge_dn4_slot = var_phix2edge_dn4;
        *var_phix2edge_rv_slot = var_phix2edge_rv;
        *var_phixedge_slot = var_phixedge;
        *var_phixedge_dn4_slot = var_phixedge_dn4;
        *var_phixedge_rv_slot = var_phixedge_rv;
        *var_rs_t_slot = var_rs_t;
        *var_rs_t_dn4_slot = var_rs_t_dn4;
        *var_rs_t_rv_slot = var_rs_t_rv;
        *var_tf_betedge_slot = var_tf_betedge;
        *var_tf_betedge_dn4_slot = var_tf_betedge_dn4;
        *var_tf_betedge_rv_slot = var_tf_betedge_rv;
        *var_tf_cs_slot = var_tf_cs;
        *var_tf_cs_dn4_slot = var_tf_cs_dn4;
        *var_tf_cs_rv_slot = var_tf_cs_rv;
        *var_tf_ther_slot = var_tf_ther;
        *var_tf_ther_dn4_slot = var_tf_ther_dn4;
        *var_tf_ther_rv_slot = var_tf_ther_rv;
        *var_tf_thesat_slot = var_tf_thesat;
        *var_tf_thesat_dn4_slot = var_tf_thesat_dn4;
        *var_tf_thesat_rv_slot = var_tf_thesat_rv;
        *var_tf_xcor_slot = var_tf_xcor;
        *var_tf_xcor_dn4_slot = var_tf_xcor_dn4;
        *var_tf_xcor_rv_slot = var_tf_xcor_rv;
        *var_thecs_t_slot = var_thecs_t;
        *var_thecs_t_dn4_slot = var_thecs_t_dn4;
        *var_thecs_t_rv_slot = var_thecs_t_rv;
        *var_ther_i_slot = var_ther_i;
        *var_ther_i_dn4_slot = var_ther_i_dn4;
        *var_ther_i_rv_slot = var_ther_i_rv;
        *var_thesat_t_slot = var_thesat_t;
        *var_thesat_t_dn4_slot = var_thesat_t_dn4;
        *var_thesat_t_rv_slot = var_thesat_t_rv;
        *var_thesatac_t_slot = var_thesatac_t;
        *var_thesatac_t_dn4_slot = var_thesatac_t_dn4;
        *var_thesatac_t_rv_slot = var_thesatac_t_rv;
        *var_v_ds_slot = var_v_ds;
        *var_v_ds_dn7_slot = var_v_ds_dn7;
        *var_v_ds_dn8_slot = var_v_ds_dn8;
        *var_v_ds_rv_slot = var_v_ds_rv;
        *var_v_gs_slot = var_v_gs;
        *var_v_gs_dn6_slot = var_v_gs_dn6;
        *var_v_gs_dn7_slot = var_v_gs_dn7;
        *var_v_gs_dn8_slot = var_v_gs_dn8;
        *var_v_gs_rv_slot = var_v_gs_rv;
        *var_v_sb_slot = var_v_sb;
        *var_v_sb_dn7_slot = var_v_sb_dn7;
        *var_v_sb_dn8_slot = var_v_sb_dn8;
        *var_v_sb_dn9_slot = var_v_sb_dn9;
        *var_v_sb_rv_slot = var_v_sb_rv;
        *var_vfbedge_t_slot = var_vfbedge_t;
        *var_vfbedge_t_dn4_slot = var_vfbedge_t_dn4;
        *var_vfbedge_t_rv_slot = var_vfbedge_t_rv;
        *var_vjun_s_slot = var_vjun_s;
        *var_vjun_s_dn11_slot = var_vjun_s_dn11;
        *var_vjun_s_dn7_slot = var_vjun_s_dn7;
        *var_vjun_s_rv_slot = var_vjun_s_rv;
        *var_xcor_t_slot = var_xcor_t;
        *var_xcor_t_dn4_slot = var_xcor_t_dn4;
        *var_xcor_t_rv_slot = var_xcor_t_rv;
    }

    pub(super) fn stamp_reactive_block_23(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        var_aphi_dc: f64,
        var_aphi_dc_dn4: f64,
        var_ar: f64,
        var_bphi_dc: f64,
        var_bphi_dc_dn4: f64,
        var_ctg_i: f64,
        var_g_0_dc: f64,
        var_g_0_dc_dn4: f64,
        var_gfacnud_i: f64,
        var_guard1028: f64,
        var_inv_phit: f64,
        var_inv_phit_dn4: f64,
        var_inv_phita: f64,
        var_phib_dc: f64,
        var_phib_dc_dn4: f64,
        var_phix1_dc: f64,
        var_phix1_dc_dn4: f64,
        var_phix_dc: f64,
        var_phix_dc_dn4: f64,
        var_sqrt_phib_dc: f64,
        var_sqrt_phib_dc_dn4: f64,
        var_thesat_t: f64,
        var_thesat_t_dn4: f64,
        var_us1: f64,
        var_us1_dn4: f64,
        var_us21: f64,
        var_us21_dn4: f64,
        var_vfb_t: f64,
        var_vfb_t_dn4: f64,
        var_aphi_slot: &mut f64,
        var_aphi_dn4_slot: &mut f64,
        var_aphi_rv_slot: &mut f64,
        var_arloc_slot: &mut f64,
        var_arloc_rv_slot: &mut f64,
        var_dctg_slot: &mut f64,
        var_dctg_dn4_slot: &mut f64,
        var_dctg_dn6_slot: &mut f64,
        var_dctg_dn7_slot: &mut f64,
        var_dctg_dn8_slot: &mut f64,
        var_dctg_dn9_slot: &mut f64,
        var_dctg_rv_slot: &mut f64,
        var_dvbstar_slot: &mut f64,
        var_dvbstar_dc_slot: &mut f64,
        var_dvbstar_dc_dn4_slot: &mut f64,
        var_dvbstar_dc_dn6_slot: &mut f64,
        var_dvbstar_dc_dn7_slot: &mut f64,
        var_dvbstar_dc_dn8_slot: &mut f64,
        var_dvbstar_dc_dn9_slot: &mut f64,
        var_dvbstar_dc_rv_slot: &mut f64,
        var_dvbstar_dn4_slot: &mut f64,
        var_dvbstar_dn6_slot: &mut f64,
        var_dvbstar_dn7_slot: &mut f64,
        var_dvbstar_dn8_slot: &mut f64,
        var_dvbstar_dn9_slot: &mut f64,
        var_dvbstar_rv_slot: &mut f64,
        var_g_0_slot: &mut f64,
        var_g_0_dn4_slot: &mut f64,
        var_g_0_rv_slot: &mut f64,
        var_guard1029_slot: &mut f64,
        var_guard1029_rv_slot: &mut f64,
        var_guard1189_slot: &mut f64,
        var_guard1189_rv_slot: &mut f64,
        var_guard1190_slot: &mut f64,
        var_guard1190_rv_slot: &mut f64,
        var_phib_slot: &mut f64,
        var_phib_dn4_slot: &mut f64,
        var_phib_rv_slot: &mut f64,
        var_sigvds_slot: &mut f64,
        var_sigvds_rv_slot: &mut f64,
        var_temp__blk949_slot: &mut f64,
        var_temp__blk949_dn4_slot: &mut f64,
        var_temp__blk949_dn6_slot: &mut f64,
        var_temp__blk949_dn7_slot: &mut f64,
        var_temp__blk949_dn8_slot: &mut f64,
        var_temp__blk949_dn9_slot: &mut f64,
        var_temp__blk949_rv_slot: &mut f64,
        var_thesatloc_slot: &mut f64,
        var_thesatloc_dn4_slot: &mut f64,
        var_thesatloc_rv_slot: &mut f64,
        var_us_slot: &mut f64,
        var_us_dn4_slot: &mut f64,
        var_us_dn6_slot: &mut f64,
        var_us_dn7_slot: &mut f64,
        var_us_dn8_slot: &mut f64,
        var_us_dn9_slot: &mut f64,
        var_us_rv_slot: &mut f64,
        var_usnew_slot: &mut f64,
        var_usnew_dn4_slot: &mut f64,
        var_usnew_dn6_slot: &mut f64,
        var_usnew_dn7_slot: &mut f64,
        var_usnew_dn8_slot: &mut f64,
        var_usnew_dn9_slot: &mut f64,
        var_usnew_rv_slot: &mut f64,
        var_v_db_slot: &mut f64,
        var_v_db_dn7_slot: &mut f64,
        var_v_db_dn8_slot: &mut f64,
        var_v_db_dn9_slot: &mut f64,
        var_v_db_rv_slot: &mut f64,
        var_v_ds_slot: &mut f64,
        var_v_ds_dn7_slot: &mut f64,
        var_v_ds_dn8_slot: &mut f64,
        var_v_ds_rv_slot: &mut f64,
        var_v_gs_slot: &mut f64,
        var_v_gs_dn6_slot: &mut f64,
        var_v_gs_dn7_slot: &mut f64,
        var_v_gs_dn8_slot: &mut f64,
        var_v_gs_rv_slot: &mut f64,
        var_v_sb_slot: &mut f64,
        var_v_sb_dn7_slot: &mut f64,
        var_v_sb_dn8_slot: &mut f64,
        var_v_sb_dn9_slot: &mut f64,
        var_v_sb_rv_slot: &mut f64,
        var_v_xb_slot: &mut f64,
        var_v_xb_dc_tmp_slot: &mut f64,
        var_v_xb_dc_tmp_dn4_slot: &mut f64,
        var_v_xb_dc_tmp_dn7_slot: &mut f64,
        var_v_xb_dc_tmp_dn8_slot: &mut f64,
        var_v_xb_dc_tmp_dn9_slot: &mut f64,
        var_v_xb_dc_tmp_rv_slot: &mut f64,
        var_v_xb_dn4_slot: &mut f64,
        var_v_xb_dn7_slot: &mut f64,
        var_v_xb_dn8_slot: &mut f64,
        var_v_xb_dn9_slot: &mut f64,
        var_v_xb_rv_slot: &mut f64,
        var_vdbprime_slot: &mut f64,
        var_vdbprime_dn7_slot: &mut f64,
        var_vdbprime_dn8_slot: &mut f64,
        var_vdbprime_dn9_slot: &mut f64,
        var_vdbprime_rv_slot: &mut f64,
        var_vdsx_slot: &mut f64,
        var_vdsx_dn7_slot: &mut f64,
        var_vdsx_dn8_slot: &mut f64,
        var_vdsx_rv_slot: &mut f64,
        var_vgb_slot: &mut f64,
        var_vgb1_slot: &mut f64,
        var_vgb1_dn4_slot: &mut f64,
        var_vgb1_dn6_slot: &mut f64,
        var_vgb1_dn7_slot: &mut f64,
        var_vgb1_dn8_slot: &mut f64,
        var_vgb1_dn9_slot: &mut f64,
        var_vgb1_rv_slot: &mut f64,
        var_vgb_dn6_slot: &mut f64,
        var_vgb_dn7_slot: &mut f64,
        var_vgb_dn8_slot: &mut f64,
        var_vgb_dn9_slot: &mut f64,
        var_vgb_rv_slot: &mut f64,
        var_vgdprime_slot: &mut f64,
        var_vgdprime_dn6_slot: &mut f64,
        var_vgdprime_dn7_slot: &mut f64,
        var_vgdprime_dn8_slot: &mut f64,
        var_vgdprime_rv_slot: &mut f64,
        var_vgsprime_slot: &mut f64,
        var_vgsprime_dn6_slot: &mut f64,
        var_vgsprime_dn7_slot: &mut f64,
        var_vgsprime_dn8_slot: &mut f64,
        var_vgsprime_rv_slot: &mut f64,
        var_vjun_d_slot: &mut f64,
        var_vjun_d_dn12_slot: &mut f64,
        var_vjun_d_dn8_slot: &mut f64,
        var_vjun_d_rv_slot: &mut f64,
        var_vjun_s_slot: &mut f64,
        var_vjun_s_dn11_slot: &mut f64,
        var_vjun_s_dn7_slot: &mut f64,
        var_vjun_s_rv_slot: &mut f64,
        var_vmb_slot: &mut f64,
        var_vmb_dn4_slot: &mut f64,
        var_vmb_dn6_slot: &mut f64,
        var_vmb_dn7_slot: &mut f64,
        var_vmb_dn8_slot: &mut f64,
        var_vmb_dn9_slot: &mut f64,
        var_vmb_rv_slot: &mut f64,
        var_vmbnew_slot: &mut f64,
        var_vmbnew_dn4_slot: &mut f64,
        var_vmbnew_dn6_slot: &mut f64,
        var_vmbnew_dn7_slot: &mut f64,
        var_vmbnew_dn8_slot: &mut f64,
        var_vmbnew_dn9_slot: &mut f64,
        var_vmbnew_rv_slot: &mut f64,
        var_vsbprime_slot: &mut f64,
        var_vsbprime_dn7_slot: &mut f64,
        var_vsbprime_dn8_slot: &mut f64,
        var_vsbprime_dn9_slot: &mut f64,
        var_vsbprime_rv_slot: &mut f64,
        var_vsbstar_slot: &mut f64,
        var_vsbstar_dc_slot: &mut f64,
        var_vsbstar_dc_dn4_slot: &mut f64,
        var_vsbstar_dc_dn6_slot: &mut f64,
        var_vsbstar_dc_dn7_slot: &mut f64,
        var_vsbstar_dc_dn8_slot: &mut f64,
        var_vsbstar_dc_dn9_slot: &mut f64,
        var_vsbstar_dc_rv_slot: &mut f64,
        var_vsbstar_dc_tmp_slot: &mut f64,
        var_vsbstar_dc_tmp_dn4_slot: &mut f64,
        var_vsbstar_dc_tmp_dn6_slot: &mut f64,
        var_vsbstar_dc_tmp_dn7_slot: &mut f64,
        var_vsbstar_dc_tmp_dn8_slot: &mut f64,
        var_vsbstar_dc_tmp_dn9_slot: &mut f64,
        var_vsbstar_dc_tmp_rv_slot: &mut f64,
        var_vsbstar_dn4_slot: &mut f64,
        var_vsbstar_dn6_slot: &mut f64,
        var_vsbstar_dn7_slot: &mut f64,
        var_vsbstar_dn8_slot: &mut f64,
        var_vsbstar_dn9_slot: &mut f64,
        var_vsbstar_rv_slot: &mut f64,
        var_vsbx_slot: &mut f64,
        var_vsbx_dn4_slot: &mut f64,
        var_vsbx_dn6_slot: &mut f64,
        var_vsbx_dn7_slot: &mut f64,
        var_vsbx_dn8_slot: &mut f64,
        var_vsbx_dn9_slot: &mut f64,
        var_vsbx_rv_slot: &mut f64,
        var_xbct_slot: &mut f64,
        var_xbct_dn4_slot: &mut f64,
        var_xbct_rv_slot: &mut f64,
        var_xgb_ov_slot: &mut f64,
        var_xgb_ov_dn4_slot: &mut f64,
        var_xgb_ov_dn6_slot: &mut f64,
        var_xgb_ov_dn7_slot: &mut f64,
        var_xgb_ov_dn8_slot: &mut f64,
        var_xgb_ov_dn9_slot: &mut f64,
        var_xgb_ov_rv_slot: &mut f64,
        var_xgd_ov_slot: &mut f64,
        var_xgd_ov_dn6_slot: &mut f64,
        var_xgd_ov_dn7_slot: &mut f64,
        var_xgd_ov_dn8_slot: &mut f64,
        var_xgd_ov_rv_slot: &mut f64,
        var_xgs_ov_slot: &mut f64,
        var_xgs_ov_dn6_slot: &mut f64,
        var_xgs_ov_dn7_slot: &mut f64,
        var_xgs_ov_dn8_slot: &mut f64,
        var_xgs_ov_rv_slot: &mut f64,
        var_xsbstar_slot: &mut f64,
        var_xsbstar_dn4_slot: &mut f64,
        var_xsbstar_dn6_slot: &mut f64,
        var_xsbstar_dn7_slot: &mut f64,
        var_xsbstar_dn8_slot: &mut f64,
        var_xsbstar_dn9_slot: &mut f64,
        var_xsbstar_rv_slot: &mut f64,
    ) {
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let mut var_aphi: f64 = *var_aphi_slot;
        let mut var_aphi_dn4: f64 = *var_aphi_dn4_slot;
        let mut var_aphi_rv: f64 = *var_aphi_rv_slot;
        let mut var_arloc: f64 = *var_arloc_slot;
        let mut var_arloc_rv: f64 = *var_arloc_rv_slot;
        let mut var_dctg: f64 = *var_dctg_slot;
        let mut var_dctg_dn4: f64 = *var_dctg_dn4_slot;
        let mut var_dctg_dn6: f64 = *var_dctg_dn6_slot;
        let mut var_dctg_dn7: f64 = *var_dctg_dn7_slot;
        let mut var_dctg_dn8: f64 = *var_dctg_dn8_slot;
        let mut var_dctg_dn9: f64 = *var_dctg_dn9_slot;
        let mut var_dctg_rv: f64 = *var_dctg_rv_slot;
        let mut var_dvbstar: f64 = *var_dvbstar_slot;
        let mut var_dvbstar_dc: f64 = *var_dvbstar_dc_slot;
        let mut var_dvbstar_dc_dn4: f64 = *var_dvbstar_dc_dn4_slot;
        let mut var_dvbstar_dc_dn6: f64 = *var_dvbstar_dc_dn6_slot;
        let mut var_dvbstar_dc_dn7: f64 = *var_dvbstar_dc_dn7_slot;
        let mut var_dvbstar_dc_dn8: f64 = *var_dvbstar_dc_dn8_slot;
        let mut var_dvbstar_dc_dn9: f64 = *var_dvbstar_dc_dn9_slot;
        let mut var_dvbstar_dc_rv: f64 = *var_dvbstar_dc_rv_slot;
        let mut var_dvbstar_dn4: f64 = *var_dvbstar_dn4_slot;
        let mut var_dvbstar_dn6: f64 = *var_dvbstar_dn6_slot;
        let mut var_dvbstar_dn7: f64 = *var_dvbstar_dn7_slot;
        let mut var_dvbstar_dn8: f64 = *var_dvbstar_dn8_slot;
        let mut var_dvbstar_dn9: f64 = *var_dvbstar_dn9_slot;
        let mut var_dvbstar_rv: f64 = *var_dvbstar_rv_slot;
        let mut var_g_0: f64 = *var_g_0_slot;
        let mut var_g_0_dn4: f64 = *var_g_0_dn4_slot;
        let mut var_g_0_rv: f64 = *var_g_0_rv_slot;
        let mut var_guard1029: f64 = *var_guard1029_slot;
        let mut var_guard1029_rv: f64 = *var_guard1029_rv_slot;
        let mut var_guard1189: f64 = *var_guard1189_slot;
        let mut var_guard1189_rv: f64 = *var_guard1189_rv_slot;
        let mut var_guard1190: f64 = *var_guard1190_slot;
        let mut var_guard1190_rv: f64 = *var_guard1190_rv_slot;
        let mut var_phib: f64 = *var_phib_slot;
        let mut var_phib_dn4: f64 = *var_phib_dn4_slot;
        let mut var_phib_rv: f64 = *var_phib_rv_slot;
        let mut var_sigvds: f64 = *var_sigvds_slot;
        let mut var_sigvds_rv: f64 = *var_sigvds_rv_slot;
        let mut var_temp__blk949: f64 = *var_temp__blk949_slot;
        let mut var_temp__blk949_dn4: f64 = *var_temp__blk949_dn4_slot;
        let mut var_temp__blk949_dn6: f64 = *var_temp__blk949_dn6_slot;
        let mut var_temp__blk949_dn7: f64 = *var_temp__blk949_dn7_slot;
        let mut var_temp__blk949_dn8: f64 = *var_temp__blk949_dn8_slot;
        let mut var_temp__blk949_dn9: f64 = *var_temp__blk949_dn9_slot;
        let mut var_temp__blk949_rv: f64 = *var_temp__blk949_rv_slot;
        let mut var_thesatloc: f64 = *var_thesatloc_slot;
        let mut var_thesatloc_dn4: f64 = *var_thesatloc_dn4_slot;
        let mut var_thesatloc_rv: f64 = *var_thesatloc_rv_slot;
        let mut var_us: f64 = *var_us_slot;
        let mut var_us_dn4: f64 = *var_us_dn4_slot;
        let mut var_us_dn6: f64 = *var_us_dn6_slot;
        let mut var_us_dn7: f64 = *var_us_dn7_slot;
        let mut var_us_dn8: f64 = *var_us_dn8_slot;
        let mut var_us_dn9: f64 = *var_us_dn9_slot;
        let mut var_us_rv: f64 = *var_us_rv_slot;
        let mut var_usnew: f64 = *var_usnew_slot;
        let mut var_usnew_dn4: f64 = *var_usnew_dn4_slot;
        let mut var_usnew_dn6: f64 = *var_usnew_dn6_slot;
        let mut var_usnew_dn7: f64 = *var_usnew_dn7_slot;
        let mut var_usnew_dn8: f64 = *var_usnew_dn8_slot;
        let mut var_usnew_dn9: f64 = *var_usnew_dn9_slot;
        let mut var_usnew_rv: f64 = *var_usnew_rv_slot;
        let mut var_v_db: f64 = *var_v_db_slot;
        let mut var_v_db_dn7: f64 = *var_v_db_dn7_slot;
        let mut var_v_db_dn8: f64 = *var_v_db_dn8_slot;
        let mut var_v_db_dn9: f64 = *var_v_db_dn9_slot;
        let mut var_v_db_rv: f64 = *var_v_db_rv_slot;
        let mut var_v_ds: f64 = *var_v_ds_slot;
        let mut var_v_ds_dn7: f64 = *var_v_ds_dn7_slot;
        let mut var_v_ds_dn8: f64 = *var_v_ds_dn8_slot;
        let mut var_v_ds_rv: f64 = *var_v_ds_rv_slot;
        let mut var_v_gs: f64 = *var_v_gs_slot;
        let mut var_v_gs_dn6: f64 = *var_v_gs_dn6_slot;
        let mut var_v_gs_dn7: f64 = *var_v_gs_dn7_slot;
        let mut var_v_gs_dn8: f64 = *var_v_gs_dn8_slot;
        let mut var_v_gs_rv: f64 = *var_v_gs_rv_slot;
        let mut var_v_sb: f64 = *var_v_sb_slot;
        let mut var_v_sb_dn7: f64 = *var_v_sb_dn7_slot;
        let mut var_v_sb_dn8: f64 = *var_v_sb_dn8_slot;
        let mut var_v_sb_dn9: f64 = *var_v_sb_dn9_slot;
        let mut var_v_sb_rv: f64 = *var_v_sb_rv_slot;
        let mut var_v_xb: f64 = *var_v_xb_slot;
        let mut var_v_xb_dc_tmp: f64 = *var_v_xb_dc_tmp_slot;
        let mut var_v_xb_dc_tmp_dn4: f64 = *var_v_xb_dc_tmp_dn4_slot;
        let mut var_v_xb_dc_tmp_dn7: f64 = *var_v_xb_dc_tmp_dn7_slot;
        let mut var_v_xb_dc_tmp_dn8: f64 = *var_v_xb_dc_tmp_dn8_slot;
        let mut var_v_xb_dc_tmp_dn9: f64 = *var_v_xb_dc_tmp_dn9_slot;
        let mut var_v_xb_dc_tmp_rv: f64 = *var_v_xb_dc_tmp_rv_slot;
        let mut var_v_xb_dn4: f64 = *var_v_xb_dn4_slot;
        let mut var_v_xb_dn7: f64 = *var_v_xb_dn7_slot;
        let mut var_v_xb_dn8: f64 = *var_v_xb_dn8_slot;
        let mut var_v_xb_dn9: f64 = *var_v_xb_dn9_slot;
        let mut var_v_xb_rv: f64 = *var_v_xb_rv_slot;
        let mut var_vdbprime: f64 = *var_vdbprime_slot;
        let mut var_vdbprime_dn7: f64 = *var_vdbprime_dn7_slot;
        let mut var_vdbprime_dn8: f64 = *var_vdbprime_dn8_slot;
        let mut var_vdbprime_dn9: f64 = *var_vdbprime_dn9_slot;
        let mut var_vdbprime_rv: f64 = *var_vdbprime_rv_slot;
        let mut var_vdsx: f64 = *var_vdsx_slot;
        let mut var_vdsx_dn7: f64 = *var_vdsx_dn7_slot;
        let mut var_vdsx_dn8: f64 = *var_vdsx_dn8_slot;
        let mut var_vdsx_rv: f64 = *var_vdsx_rv_slot;
        let mut var_vgb: f64 = *var_vgb_slot;
        let mut var_vgb1: f64 = *var_vgb1_slot;
        let mut var_vgb1_dn4: f64 = *var_vgb1_dn4_slot;
        let mut var_vgb1_dn6: f64 = *var_vgb1_dn6_slot;
        let mut var_vgb1_dn7: f64 = *var_vgb1_dn7_slot;
        let mut var_vgb1_dn8: f64 = *var_vgb1_dn8_slot;
        let mut var_vgb1_dn9: f64 = *var_vgb1_dn9_slot;
        let mut var_vgb1_rv: f64 = *var_vgb1_rv_slot;
        let mut var_vgb_dn6: f64 = *var_vgb_dn6_slot;
        let mut var_vgb_dn7: f64 = *var_vgb_dn7_slot;
        let mut var_vgb_dn8: f64 = *var_vgb_dn8_slot;
        let mut var_vgb_dn9: f64 = *var_vgb_dn9_slot;
        let mut var_vgb_rv: f64 = *var_vgb_rv_slot;
        let mut var_vgdprime: f64 = *var_vgdprime_slot;
        let mut var_vgdprime_dn6: f64 = *var_vgdprime_dn6_slot;
        let mut var_vgdprime_dn7: f64 = *var_vgdprime_dn7_slot;
        let mut var_vgdprime_dn8: f64 = *var_vgdprime_dn8_slot;
        let mut var_vgdprime_rv: f64 = *var_vgdprime_rv_slot;
        let mut var_vgsprime: f64 = *var_vgsprime_slot;
        let mut var_vgsprime_dn6: f64 = *var_vgsprime_dn6_slot;
        let mut var_vgsprime_dn7: f64 = *var_vgsprime_dn7_slot;
        let mut var_vgsprime_dn8: f64 = *var_vgsprime_dn8_slot;
        let mut var_vgsprime_rv: f64 = *var_vgsprime_rv_slot;
        let mut var_vjun_d: f64 = *var_vjun_d_slot;
        let mut var_vjun_d_dn12: f64 = *var_vjun_d_dn12_slot;
        let mut var_vjun_d_dn8: f64 = *var_vjun_d_dn8_slot;
        let mut var_vjun_d_rv: f64 = *var_vjun_d_rv_slot;
        let mut var_vjun_s: f64 = *var_vjun_s_slot;
        let mut var_vjun_s_dn11: f64 = *var_vjun_s_dn11_slot;
        let mut var_vjun_s_dn7: f64 = *var_vjun_s_dn7_slot;
        let mut var_vjun_s_rv: f64 = *var_vjun_s_rv_slot;
        let mut var_vmb: f64 = *var_vmb_slot;
        let mut var_vmb_dn4: f64 = *var_vmb_dn4_slot;
        let mut var_vmb_dn6: f64 = *var_vmb_dn6_slot;
        let mut var_vmb_dn7: f64 = *var_vmb_dn7_slot;
        let mut var_vmb_dn8: f64 = *var_vmb_dn8_slot;
        let mut var_vmb_dn9: f64 = *var_vmb_dn9_slot;
        let mut var_vmb_rv: f64 = *var_vmb_rv_slot;
        let mut var_vmbnew: f64 = *var_vmbnew_slot;
        let mut var_vmbnew_dn4: f64 = *var_vmbnew_dn4_slot;
        let mut var_vmbnew_dn6: f64 = *var_vmbnew_dn6_slot;
        let mut var_vmbnew_dn7: f64 = *var_vmbnew_dn7_slot;
        let mut var_vmbnew_dn8: f64 = *var_vmbnew_dn8_slot;
        let mut var_vmbnew_dn9: f64 = *var_vmbnew_dn9_slot;
        let mut var_vmbnew_rv: f64 = *var_vmbnew_rv_slot;
        let mut var_vsbprime: f64 = *var_vsbprime_slot;
        let mut var_vsbprime_dn7: f64 = *var_vsbprime_dn7_slot;
        let mut var_vsbprime_dn8: f64 = *var_vsbprime_dn8_slot;
        let mut var_vsbprime_dn9: f64 = *var_vsbprime_dn9_slot;
        let mut var_vsbprime_rv: f64 = *var_vsbprime_rv_slot;
        let mut var_vsbstar: f64 = *var_vsbstar_slot;
        let mut var_vsbstar_dc: f64 = *var_vsbstar_dc_slot;
        let mut var_vsbstar_dc_dn4: f64 = *var_vsbstar_dc_dn4_slot;
        let mut var_vsbstar_dc_dn6: f64 = *var_vsbstar_dc_dn6_slot;
        let mut var_vsbstar_dc_dn7: f64 = *var_vsbstar_dc_dn7_slot;
        let mut var_vsbstar_dc_dn8: f64 = *var_vsbstar_dc_dn8_slot;
        let mut var_vsbstar_dc_dn9: f64 = *var_vsbstar_dc_dn9_slot;
        let mut var_vsbstar_dc_rv: f64 = *var_vsbstar_dc_rv_slot;
        let mut var_vsbstar_dc_tmp: f64 = *var_vsbstar_dc_tmp_slot;
        let mut var_vsbstar_dc_tmp_dn4: f64 = *var_vsbstar_dc_tmp_dn4_slot;
        let mut var_vsbstar_dc_tmp_dn6: f64 = *var_vsbstar_dc_tmp_dn6_slot;
        let mut var_vsbstar_dc_tmp_dn7: f64 = *var_vsbstar_dc_tmp_dn7_slot;
        let mut var_vsbstar_dc_tmp_dn8: f64 = *var_vsbstar_dc_tmp_dn8_slot;
        let mut var_vsbstar_dc_tmp_dn9: f64 = *var_vsbstar_dc_tmp_dn9_slot;
        let mut var_vsbstar_dc_tmp_rv: f64 = *var_vsbstar_dc_tmp_rv_slot;
        let mut var_vsbstar_dn4: f64 = *var_vsbstar_dn4_slot;
        let mut var_vsbstar_dn6: f64 = *var_vsbstar_dn6_slot;
        let mut var_vsbstar_dn7: f64 = *var_vsbstar_dn7_slot;
        let mut var_vsbstar_dn8: f64 = *var_vsbstar_dn8_slot;
        let mut var_vsbstar_dn9: f64 = *var_vsbstar_dn9_slot;
        let mut var_vsbstar_rv: f64 = *var_vsbstar_rv_slot;
        let mut var_vsbx: f64 = *var_vsbx_slot;
        let mut var_vsbx_dn4: f64 = *var_vsbx_dn4_slot;
        let mut var_vsbx_dn6: f64 = *var_vsbx_dn6_slot;
        let mut var_vsbx_dn7: f64 = *var_vsbx_dn7_slot;
        let mut var_vsbx_dn8: f64 = *var_vsbx_dn8_slot;
        let mut var_vsbx_dn9: f64 = *var_vsbx_dn9_slot;
        let mut var_vsbx_rv: f64 = *var_vsbx_rv_slot;
        let mut var_xbct: f64 = *var_xbct_slot;
        let mut var_xbct_dn4: f64 = *var_xbct_dn4_slot;
        let mut var_xbct_rv: f64 = *var_xbct_rv_slot;
        let mut var_xgb_ov: f64 = *var_xgb_ov_slot;
        let mut var_xgb_ov_dn4: f64 = *var_xgb_ov_dn4_slot;
        let mut var_xgb_ov_dn6: f64 = *var_xgb_ov_dn6_slot;
        let mut var_xgb_ov_dn7: f64 = *var_xgb_ov_dn7_slot;
        let mut var_xgb_ov_dn8: f64 = *var_xgb_ov_dn8_slot;
        let mut var_xgb_ov_dn9: f64 = *var_xgb_ov_dn9_slot;
        let mut var_xgb_ov_rv: f64 = *var_xgb_ov_rv_slot;
        let mut var_xgd_ov: f64 = *var_xgd_ov_slot;
        let mut var_xgd_ov_dn6: f64 = *var_xgd_ov_dn6_slot;
        let mut var_xgd_ov_dn7: f64 = *var_xgd_ov_dn7_slot;
        let mut var_xgd_ov_dn8: f64 = *var_xgd_ov_dn8_slot;
        let mut var_xgd_ov_rv: f64 = *var_xgd_ov_rv_slot;
        let mut var_xgs_ov: f64 = *var_xgs_ov_slot;
        let mut var_xgs_ov_dn6: f64 = *var_xgs_ov_dn6_slot;
        let mut var_xgs_ov_dn7: f64 = *var_xgs_ov_dn7_slot;
        let mut var_xgs_ov_dn8: f64 = *var_xgs_ov_dn8_slot;
        let mut var_xgs_ov_rv: f64 = *var_xgs_ov_rv_slot;
        let mut var_xsbstar: f64 = *var_xsbstar_slot;
        let mut var_xsbstar_dn4: f64 = *var_xsbstar_dn4_slot;
        let mut var_xsbstar_dn6: f64 = *var_xsbstar_dn6_slot;
        let mut var_xsbstar_dn7: f64 = *var_xsbstar_dn7_slot;
        let mut var_xsbstar_dn8: f64 = *var_xsbstar_dn8_slot;
        let mut var_xsbstar_dn9: f64 = *var_xsbstar_dn9_slot;
        let mut var_xsbstar_rv: f64 = *var_xsbstar_rv_slot;

        let (assign40550_e53704, assign40550_e53704_d_n8, assign40550_e53704_d_n12,) = {
    if (var_guard1028 != 0.0) {
        let assign40550_e53702: f64 = (-(nv8 - nv12));
        (assign40550_e53702, (-1.0), 1.0,)
    } else {
        (var_vjun_d, var_vjun_d_dn8, var_vjun_d_dn12,)
    }
};
        var_vjun_d = assign40550_e53704;
        var_vjun_d_dn8 = assign40550_e53704_d_n8;
        var_vjun_d_dn12 = assign40550_e53704_d_n12;
        var_vjun_d_rv = 0.0;

        let (assign40560_e53710, assign40560_e53710_d_n6, assign40560_e53710_d_n7, assign40560_e53710_d_n8,) = {
    if (var_guard1028 == 0.0) {
        let assign40560_e53708: f64 = (-(nv6 - nv7));
        (assign40560_e53708, (-1.0), 1.0, 0.0,)
    } else {
        (var_v_gs, var_v_gs_dn6, var_v_gs_dn7, var_v_gs_dn8,)
    }
};
        var_v_gs = assign40560_e53710;
        var_v_gs_dn6 = assign40560_e53710_d_n6;
        var_v_gs_dn7 = assign40560_e53710_d_n7;
        var_v_gs_dn8 = assign40560_e53710_d_n8;
        var_v_gs_rv = 0.0;

        let (assign40570_e53716, assign40570_e53716_d_n7, assign40570_e53716_d_n8,) = {
    if (var_guard1028 == 0.0) {
        let assign40570_e53714: f64 = (-(nv8 - nv7));
        (assign40570_e53714, 1.0, (-1.0),)
    } else {
        (var_v_ds, var_v_ds_dn7, var_v_ds_dn8,)
    }
};
        var_v_ds = assign40570_e53716;
        var_v_ds_dn7 = assign40570_e53716_d_n7;
        var_v_ds_dn8 = assign40570_e53716_d_n8;
        var_v_ds_rv = 0.0;

        let (assign40580_e53722, assign40580_e53722_d_n7, assign40580_e53722_d_n8, assign40580_e53722_d_n9,) = {
    if (var_guard1028 == 0.0) {
        let assign40580_e53720: f64 = (-(nv7 - nv9));
        (assign40580_e53720, (-1.0), 0.0, 1.0,)
    } else {
        (var_v_sb, var_v_sb_dn7, var_v_sb_dn8, var_v_sb_dn9,)
    }
};
        var_v_sb = assign40580_e53722;
        var_v_sb_dn7 = assign40580_e53722_d_n7;
        var_v_sb_dn8 = assign40580_e53722_d_n8;
        var_v_sb_dn9 = assign40580_e53722_d_n9;
        var_v_sb_rv = 0.0;

        let (assign40590_e53727, assign40590_e53727_d_n7, assign40590_e53727_d_n11,) = {
    if (var_guard1028 == 0.0) {
        ((nv7 - nv11), 1.0, -1.0,)
    } else {
        (var_vjun_s, var_vjun_s_dn7, var_vjun_s_dn11,)
    }
};
        var_vjun_s = assign40590_e53727;
        var_vjun_s_dn7 = assign40590_e53727_d_n7;
        var_vjun_s_dn11 = assign40590_e53727_d_n11;
        var_vjun_s_rv = 0.0;

        let (assign40600_e53732, assign40600_e53732_d_n8, assign40600_e53732_d_n12,) = {
    if (var_guard1028 == 0.0) {
        ((nv8 - nv12), 1.0, -1.0,)
    } else {
        (var_vjun_d, var_vjun_d_dn8, var_vjun_d_dn12,)
    }
};
        var_vjun_d = assign40600_e53732;
        var_vjun_d_dn8 = assign40600_e53732_d_n8;
        var_vjun_d_dn12 = assign40600_e53732_d_n12;
        var_vjun_d_rv = 0.0;

        let assign40610_e53735: f64 = (var_v_gs + var_v_sb);
        var_vgb = assign40610_e53735;
        var_vgb_dn6 = var_v_gs_dn6;
        var_vgb_dn7 = (var_v_gs_dn7 + var_v_sb_dn7);
        var_vgb_dn8 = (var_v_gs_dn8 + var_v_sb_dn8);
        var_vgb_dn9 = var_v_sb_dn9;
        var_vgb_rv = 0.0;

        var_vgsprime = var_v_gs;
        var_vgsprime_dn6 = var_v_gs_dn6;
        var_vgsprime_dn7 = var_v_gs_dn7;
        var_vgsprime_dn8 = var_v_gs_dn8;
        var_vgsprime_rv = 0.0;

        var_vsbprime = var_v_sb;
        var_vsbprime_dn7 = var_v_sb_dn7;
        var_vsbprime_dn8 = var_v_sb_dn8;
        var_vsbprime_dn9 = var_v_sb_dn9;
        var_vsbprime_rv = 0.0;

        let assign40640_e53740: f64 = (var_v_ds + var_v_sb);
        var_vdbprime = assign40640_e53740;
        var_vdbprime_dn7 = (var_v_ds_dn7 + var_v_sb_dn7);
        var_vdbprime_dn8 = (var_v_ds_dn8 + var_v_sb_dn8);
        var_vdbprime_dn9 = var_v_sb_dn9;
        var_vdbprime_rv = 0.0;

        let assign40650_e53743: f64 = (var_v_gs - var_v_ds);
        var_vgdprime = assign40650_e53743;
        var_vgdprime_dn6 = var_v_gs_dn6;
        var_vgdprime_dn7 = (var_v_gs_dn7 - var_v_ds_dn7);
        var_vgdprime_dn8 = (var_v_gs_dn8 - var_v_ds_dn8);
        var_vgdprime_rv = 0.0;

        let assign40660_e53745: f64 = (-var_vgsprime);
        let assign40660_e53747: f64 = (assign40660_e53745 * var_inv_phita);
        var_xgs_ov = assign40660_e53747;
        var_xgs_ov_dn6 = ((-var_vgsprime_dn6) * var_inv_phita);
        var_xgs_ov_dn7 = ((-var_vgsprime_dn7) * var_inv_phita);
        var_xgs_ov_dn8 = ((-var_vgsprime_dn8) * var_inv_phita);
        var_xgs_ov_rv = 0.0;

        let assign40670_e53749: f64 = (-var_vgdprime);
        let assign40670_e53751: f64 = (assign40670_e53749 * var_inv_phita);
        var_xgd_ov = assign40670_e53751;
        var_xgd_ov_dn6 = ((-var_vgdprime_dn6) * var_inv_phita);
        var_xgd_ov_dn7 = ((-var_vgdprime_dn7) * var_inv_phita);
        var_xgd_ov_dn8 = ((-var_vgdprime_dn8) * var_inv_phita);
        var_xgd_ov_rv = 0.0;

        let assign40680_e53754: f64 = (var_vgb - var_vfb_t);
        let assign40680_e53755: f64 = (-assign40680_e53754);
        let assign40680_e53757: f64 = (assign40680_e53755 * var_inv_phita);
        var_xgb_ov = assign40680_e53757;
        var_xgb_ov_dn4 = ((-(-var_vfb_t_dn4)) * var_inv_phita);
        var_xgb_ov_dn6 = ((-var_vgb_dn6) * var_inv_phita);
        var_xgb_ov_dn7 = ((-var_vgb_dn7) * var_inv_phita);
        var_xgb_ov_dn8 = ((-var_vgb_dn8) * var_inv_phita);
        var_xgb_ov_dn9 = ((-var_vgb_dn9) * var_inv_phita);
        var_xgb_ov_rv = 0.0;

        var_sigvds = 1.0;
        var_sigvds_rv = 0.0;

        let assign40700_e53761: f64 = if var_v_ds < 0.0 { 1.0 } else { 0.0 };
        var_guard1029 = assign40700_e53761;
        var_guard1029_rv = 0.0;

        let (assign40710_e53766,) = {
    if (var_guard1029 != 0.0) {
        let assign40710_e53764: f64 = (-1.0);
        (assign40710_e53764,)
    } else {
        (var_sigvds,)
    }
};
        var_sigvds = assign40710_e53766;
        var_sigvds_rv = 0.0;

        let (assign40720_e53772, assign40720_e53772_d_n6, assign40720_e53772_d_n7, assign40720_e53772_d_n8,) = {
    if (var_guard1029 != 0.0) {
        let assign40720_e53770: f64 = (var_v_gs - var_v_ds);
        (assign40720_e53770, var_v_gs_dn6, (var_v_gs_dn7 - var_v_ds_dn7), (var_v_gs_dn8 - var_v_ds_dn8),)
    } else {
        (var_v_gs, var_v_gs_dn6, var_v_gs_dn7, var_v_gs_dn8,)
    }
};
        var_v_gs = assign40720_e53772;
        var_v_gs_dn6 = assign40720_e53772_d_n6;
        var_v_gs_dn7 = assign40720_e53772_d_n7;
        var_v_gs_dn8 = assign40720_e53772_d_n8;
        var_v_gs_rv = 0.0;

        let (assign40730_e53778, assign40730_e53778_d_n7, assign40730_e53778_d_n8, assign40730_e53778_d_n9,) = {
    if (var_guard1029 != 0.0) {
        let assign40730_e53776: f64 = (var_v_sb + var_v_ds);
        (assign40730_e53776, (var_v_sb_dn7 + var_v_ds_dn7), (var_v_sb_dn8 + var_v_ds_dn8), var_v_sb_dn9,)
    } else {
        (var_v_sb, var_v_sb_dn7, var_v_sb_dn8, var_v_sb_dn9,)
    }
};
        var_v_sb = assign40730_e53778;
        var_v_sb_dn7 = assign40730_e53778_d_n7;
        var_v_sb_dn8 = assign40730_e53778_d_n8;
        var_v_sb_dn9 = assign40730_e53778_d_n9;
        var_v_sb_rv = 0.0;

        let (assign40740_e53783, assign40740_e53783_d_n7, assign40740_e53783_d_n8,) = {
    if (var_guard1029 != 0.0) {
        let assign40740_e53781: f64 = (-var_v_ds);
        (assign40740_e53781, (-var_v_ds_dn7), (-var_v_ds_dn8),)
    } else {
        (var_v_ds, var_v_ds_dn7, var_v_ds_dn8,)
    }
};
        var_v_ds = assign40740_e53783;
        var_v_ds_dn7 = assign40740_e53783_d_n7;
        var_v_ds_dn8 = assign40740_e53783_d_n8;
        var_v_ds_rv = 0.0;

        let assign40750_e53786: f64 = (var_v_ds + var_v_sb);
        var_v_db = assign40750_e53786;
        var_v_db_dn7 = (var_v_ds_dn7 + var_v_sb_dn7);
        var_v_db_dn8 = (var_v_ds_dn8 + var_v_sb_dn8);
        var_v_db_dn9 = var_v_sb_dn9;
        var_v_db_rv = 0.0;

        let assign40760_e53789: f64 = (var_v_ds * var_v_ds);
        let assign40760_e53792: f64 = (var_v_ds * var_v_ds);
        let assign40760_e53794: f64 = (assign40760_e53792 + 0.01);
        let assign40760_e53795: f64 = (assign40760_e53794).sqrt();
        let assign40760_e53797: f64 = (assign40760_e53795 + 0.1);
        let assign40760_e53798: f64 = (assign40760_e53789 / assign40760_e53797);
        var_vdsx = assign40760_e53798;
        var_vdsx_dn7 = (((((var_v_ds_dn7 * var_v_ds) + (var_v_ds * var_v_ds_dn7)) * assign40760_e53797) - (assign40760_e53789 * (((var_v_ds_dn7 * var_v_ds) + (var_v_ds * var_v_ds_dn7)) / (2.0 * assign40760_e53795)))) / (assign40760_e53797 * assign40760_e53797));
        var_vdsx_dn8 = (((((var_v_ds_dn8 * var_v_ds) + (var_v_ds * var_v_ds_dn8)) * assign40760_e53797) - (assign40760_e53789 * (((var_v_ds_dn8 * var_v_ds) + (var_v_ds * var_v_ds_dn8)) / (2.0 * assign40760_e53795)))) / (assign40760_e53797 * assign40760_e53797));
        var_vdsx_rv = 0.0;

        let assign40770_e53802: f64 = (var_v_db + var_v_sb);
        let assign40770_e53805: f64 = (var_v_db - var_v_sb);
        let assign40770_e53808: f64 = (var_v_db - var_v_sb);
        let assign40770_e53809: f64 = (assign40770_e53805 * assign40770_e53808);
        let assign40770_e53811: f64 = (assign40770_e53809 + var_bphi_dc);
        let assign40770_e53812: f64 = (assign40770_e53811).sqrt();
        let assign40770_e53813: f64 = (assign40770_e53802 - assign40770_e53812);
        let assign40770_e53814: f64 = (0.5 * assign40770_e53813);
        let assign40770_e53816: f64 = (assign40770_e53814 + var_phix_dc);
        var_v_xb = assign40770_e53816;
        var_v_xb_dn4 = ((0.5 * (-(var_bphi_dc_dn4 / (2.0 * assign40770_e53812)))) + var_phix_dc_dn4);
        var_v_xb_dn7 = (0.5 * ((var_v_db_dn7 + var_v_sb_dn7) - ((((var_v_db_dn7 - var_v_sb_dn7) * assign40770_e53808) + (assign40770_e53805 * (var_v_db_dn7 - var_v_sb_dn7))) / (2.0 * assign40770_e53812))));
        var_v_xb_dn8 = (0.5 * ((var_v_db_dn8 + var_v_sb_dn8) - ((((var_v_db_dn8 - var_v_sb_dn8) * assign40770_e53808) + (assign40770_e53805 * (var_v_db_dn8 - var_v_sb_dn8))) / (2.0 * assign40770_e53812))));
        var_v_xb_dn9 = (0.5 * ((var_v_db_dn9 + var_v_sb_dn9) - ((((var_v_db_dn9 - var_v_sb_dn9) * assign40770_e53808) + (assign40770_e53805 * (var_v_db_dn9 - var_v_sb_dn9))) / (2.0 * assign40770_e53812))));
        var_v_xb_rv = 0.0;

        var_v_xb_dc_tmp = var_v_xb;
        var_v_xb_dc_tmp_dn4 = var_v_xb_dn4;
        var_v_xb_dc_tmp_dn7 = var_v_xb_dn7;
        var_v_xb_dc_tmp_dn8 = var_v_xb_dn8;
        var_v_xb_dc_tmp_dn9 = var_v_xb_dn9;
        var_v_xb_dc_tmp_rv = 0.0;

        let assign40790_e53822: f64 = var_v_xb;
        let assign40790_e53825: f64 = var_v_xb;
        let assign40790_e53828: f64 = var_v_xb;
        let assign40790_e53829: f64 = (assign40790_e53825 * assign40790_e53828);
        let assign40790_e53831: f64 = (assign40790_e53829 + var_aphi_dc);
        let assign40790_e53832: f64 = (assign40790_e53831).sqrt();
        let assign40790_e53833: f64 = (assign40790_e53822 - assign40790_e53832);
        let assign40790_e53834: f64 = (0.5 * assign40790_e53833);
        let assign40790_e53835: f64 = (var_v_sb - assign40790_e53834);
        let assign40790_e53837: f64 = (assign40790_e53835 + var_phix1_dc);
        var_vsbstar_dc = assign40790_e53837;
        var_vsbstar_dc_dn4 = ((-(0.5 * (var_v_xb_dn4 - ((((var_v_xb_dn4 * assign40790_e53828) + (assign40790_e53825 * var_v_xb_dn4)) + var_aphi_dc_dn4) / (2.0 * assign40790_e53832))))) + var_phix1_dc_dn4);
        var_vsbstar_dc_dn6 = 0.0;
        var_vsbstar_dc_dn7 = (var_v_sb_dn7 - (0.5 * (var_v_xb_dn7 - (((var_v_xb_dn7 * assign40790_e53828) + (assign40790_e53825 * var_v_xb_dn7)) / (2.0 * assign40790_e53832)))));
        var_vsbstar_dc_dn8 = (var_v_sb_dn8 - (0.5 * (var_v_xb_dn8 - (((var_v_xb_dn8 * assign40790_e53828) + (assign40790_e53825 * var_v_xb_dn8)) / (2.0 * assign40790_e53832)))));
        var_vsbstar_dc_dn9 = (var_v_sb_dn9 - (0.5 * (var_v_xb_dn9 - (((var_v_xb_dn9 * assign40790_e53828) + (assign40790_e53825 * var_v_xb_dn9)) / (2.0 * assign40790_e53832)))));
        var_vsbstar_dc_rv = 0.0;

        var_vsbstar_dc_tmp = var_vsbstar_dc;
        var_vsbstar_dc_tmp_dn4 = var_vsbstar_dc_dn4;
        var_vsbstar_dc_tmp_dn6 = var_vsbstar_dc_dn6;
        var_vsbstar_dc_tmp_dn7 = var_vsbstar_dc_dn7;
        var_vsbstar_dc_tmp_dn8 = var_vsbstar_dc_dn8;
        var_vsbstar_dc_tmp_dn9 = var_vsbstar_dc_dn9;
        var_vsbstar_dc_tmp_rv = 0.0;

        var_dvbstar_dc = 0.0;
        var_dvbstar_dc_dn4 = 0.0;
        var_dvbstar_dc_dn6 = 0.0;
        var_dvbstar_dc_dn7 = 0.0;
        var_dvbstar_dc_dn8 = 0.0;
        var_dvbstar_dc_dn9 = 0.0;
        var_dvbstar_dc_rv = 0.0;

        let assign40820_e53846: f64 = if ((p.p45 != 0.0) && (var_gfacnud_i != 1.0)) { 1.0 } else { 0.0 };
        var_guard1189 = assign40820_e53846;
        var_guard1189_rv = 0.0;

        let (assign40830_e53856, assign40830_e53856_d_n4, assign40830_e53856_d_n6, assign40830_e53856_d_n7, assign40830_e53856_d_n8, assign40830_e53856_d_n9,) = {
    if (var_guard1189 != 0.0) {
        let assign40830_e53852: f64 = (var_v_ds - var_vdsx);
        let assign40830_e53853: f64 = (0.5 * assign40830_e53852);
        let assign40830_e53854: f64 = (var_vsbstar_dc + assign40830_e53853);
        (assign40830_e53854, var_vsbstar_dc_dn4, var_vsbstar_dc_dn6, (var_vsbstar_dc_dn7 + (0.5 * (var_v_ds_dn7 - var_vdsx_dn7))), (var_vsbstar_dc_dn8 + (0.5 * (var_v_ds_dn8 - var_vdsx_dn8))), var_vsbstar_dc_dn9,)
    } else {
        (var_vmb, var_vmb_dn4, var_vmb_dn6, var_vmb_dn7, var_vmb_dn8, var_vmb_dn9,)
    }
};
        var_vmb = assign40830_e53856;
        var_vmb_dn4 = assign40830_e53856_d_n4;
        var_vmb_dn6 = assign40830_e53856_d_n6;
        var_vmb_dn7 = assign40830_e53856_d_n7;
        var_vmb_dn8 = assign40830_e53856_d_n8;
        var_vmb_dn9 = assign40830_e53856_d_n9;
        var_vmb_rv = 0.0;

        let (assign40840_e53865, assign40840_e53865_d_n4, assign40840_e53865_d_n6, assign40840_e53865_d_n7, assign40840_e53865_d_n8, assign40840_e53865_d_n9,) = {
    if (var_guard1189 != 0.0) {
        let assign40840_e53860: f64 = (var_vmb + var_phib_dc);
        let assign40840_e53861: f64 = (assign40840_e53860).sqrt();
        let assign40840_e53863: f64 = (assign40840_e53861 - var_sqrt_phib_dc);
        (assign40840_e53863, (((var_vmb_dn4 + var_phib_dc_dn4) / (2.0 * assign40840_e53861)) - var_sqrt_phib_dc_dn4), (var_vmb_dn6 / (2.0 * assign40840_e53861)), (var_vmb_dn7 / (2.0 * assign40840_e53861)), (var_vmb_dn8 / (2.0 * assign40840_e53861)), (var_vmb_dn9 / (2.0 * assign40840_e53861)),)
    } else {
        (var_us, var_us_dn4, var_us_dn6, var_us_dn7, var_us_dn8, var_us_dn9,)
    }
};
        var_us = assign40840_e53865;
        var_us_dn4 = assign40840_e53865_d_n4;
        var_us_dn6 = assign40840_e53865_d_n6;
        var_us_dn7 = assign40840_e53865_d_n7;
        var_us_dn8 = assign40840_e53865_d_n8;
        var_us_dn9 = assign40840_e53865_d_n9;
        var_us_rv = 0.0;

        let (assign40850_e53877, assign40850_e53877_d_n4, assign40850_e53877_d_n6, assign40850_e53877_d_n7, assign40850_e53877_d_n8, assign40850_e53877_d_n9,) = {
    if (var_guard1189 != 0.0) {
        let assign40850_e53870: f64 = (var_us - var_us1);
        let assign40850_e53871: f64 = (2.0 * assign40850_e53870);
        let assign40850_e53873: f64 = (assign40850_e53871 / var_us21);
        let assign40850_e53875: f64 = (assign40850_e53873 - 1.0);
        (assign40850_e53875, ((((2.0 * (var_us_dn4 - var_us1_dn4)) * var_us21) - (assign40850_e53871 * var_us21_dn4)) / (var_us21 * var_us21)), ((2.0 * var_us_dn6) / var_us21), ((2.0 * var_us_dn7) / var_us21), ((2.0 * var_us_dn8) / var_us21), ((2.0 * var_us_dn9) / var_us21),)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign40850_e53877;
        var_temp__blk949_dn4 = assign40850_e53877_d_n4;
        var_temp__blk949_dn6 = assign40850_e53877_d_n6;
        var_temp__blk949_dn7 = assign40850_e53877_d_n7;
        var_temp__blk949_dn8 = assign40850_e53877_d_n8;
        var_temp__blk949_dn9 = assign40850_e53877_d_n9;
        var_temp__blk949_rv = 0.0;

        let (assign40860_e53898, assign40860_e53898_d_n4, assign40860_e53898_d_n6, assign40860_e53898_d_n7, assign40860_e53898_d_n8, assign40860_e53898_d_n9,) = {
    if (var_guard1189 != 0.0) {
        let assign40860_e53883: f64 = (1.0 - var_gfacnud_i);
        let assign40860_e53884: f64 = (0.25 * assign40860_e53883);
        let assign40860_e53886: f64 = (assign40860_e53884 * var_us21);
        let assign40860_e53890: f64 = (var_temp__blk949 * var_temp__blk949);
        let assign40860_e53892: f64 = (assign40860_e53890 + 0.4804530139182);
        let assign40860_e53893: f64 = (assign40860_e53892).sqrt();
        let assign40860_e53894: f64 = (var_temp__blk949 + assign40860_e53893);
        let assign40860_e53895: f64 = (assign40860_e53886 * assign40860_e53894);
        let assign40860_e53896: f64 = (var_us - assign40860_e53895);
        (assign40860_e53896, (var_us_dn4 - (((assign40860_e53884 * var_us21_dn4) * assign40860_e53894) + (assign40860_e53886 * (var_temp__blk949_dn4 + (((var_temp__blk949_dn4 * var_temp__blk949) + (var_temp__blk949 * var_temp__blk949_dn4)) / (2.0 * assign40860_e53893)))))), (var_us_dn6 - (assign40860_e53886 * (var_temp__blk949_dn6 + (((var_temp__blk949_dn6 * var_temp__blk949) + (var_temp__blk949 * var_temp__blk949_dn6)) / (2.0 * assign40860_e53893))))), (var_us_dn7 - (assign40860_e53886 * (var_temp__blk949_dn7 + (((var_temp__blk949_dn7 * var_temp__blk949) + (var_temp__blk949 * var_temp__blk949_dn7)) / (2.0 * assign40860_e53893))))), (var_us_dn8 - (assign40860_e53886 * (var_temp__blk949_dn8 + (((var_temp__blk949_dn8 * var_temp__blk949) + (var_temp__blk949 * var_temp__blk949_dn8)) / (2.0 * assign40860_e53893))))), (var_us_dn9 - (assign40860_e53886 * (var_temp__blk949_dn9 + (((var_temp__blk949_dn9 * var_temp__blk949) + (var_temp__blk949 * var_temp__blk949_dn9)) / (2.0 * assign40860_e53893))))),)
    } else {
        (var_usnew, var_usnew_dn4, var_usnew_dn6, var_usnew_dn7, var_usnew_dn8, var_usnew_dn9,)
    }
};
        var_usnew = assign40860_e53898;
        var_usnew_dn4 = assign40860_e53898_d_n4;
        var_usnew_dn6 = assign40860_e53898_d_n6;
        var_usnew_dn7 = assign40860_e53898_d_n7;
        var_usnew_dn8 = assign40860_e53898_d_n8;
        var_usnew_dn9 = assign40860_e53898_d_n9;
        var_usnew_rv = 0.0;

        let (assign40870_e53910, assign40870_e53910_d_n4, assign40870_e53910_d_n6, assign40870_e53910_d_n7, assign40870_e53910_d_n8, assign40870_e53910_d_n9,) = {
    if (var_guard1189 != 0.0) {
        let assign40870_e53902: f64 = (var_usnew * var_usnew);
        let assign40870_e53905: f64 = (2.0 * var_sqrt_phib_dc);
        let assign40870_e53907: f64 = (assign40870_e53905 * var_usnew);
        let assign40870_e53908: f64 = (assign40870_e53902 + assign40870_e53907);
        (assign40870_e53908, (((var_usnew_dn4 * var_usnew) + (var_usnew * var_usnew_dn4)) + (((2.0 * var_sqrt_phib_dc_dn4) * var_usnew) + (assign40870_e53905 * var_usnew_dn4))), (((var_usnew_dn6 * var_usnew) + (var_usnew * var_usnew_dn6)) + (assign40870_e53905 * var_usnew_dn6)), (((var_usnew_dn7 * var_usnew) + (var_usnew * var_usnew_dn7)) + (assign40870_e53905 * var_usnew_dn7)), (((var_usnew_dn8 * var_usnew) + (var_usnew * var_usnew_dn8)) + (assign40870_e53905 * var_usnew_dn8)), (((var_usnew_dn9 * var_usnew) + (var_usnew * var_usnew_dn9)) + (assign40870_e53905 * var_usnew_dn9)),)
    } else {
        (var_vmbnew, var_vmbnew_dn4, var_vmbnew_dn6, var_vmbnew_dn7, var_vmbnew_dn8, var_vmbnew_dn9,)
    }
};
        var_vmbnew = assign40870_e53910;
        var_vmbnew_dn4 = assign40870_e53910_d_n4;
        var_vmbnew_dn6 = assign40870_e53910_d_n6;
        var_vmbnew_dn7 = assign40870_e53910_d_n7;
        var_vmbnew_dn8 = assign40870_e53910_d_n8;
        var_vmbnew_dn9 = assign40870_e53910_d_n9;
        var_vmbnew_rv = 0.0;

        let (assign40880_e53920, assign40880_e53920_d_n4, assign40880_e53920_d_n6, assign40880_e53920_d_n7, assign40880_e53920_d_n8, assign40880_e53920_d_n9,) = {
    if (var_guard1189 != 0.0) {
        let assign40880_e53916: f64 = (var_v_ds - var_vdsx);
        let assign40880_e53917: f64 = (0.5 * assign40880_e53916);
        let assign40880_e53918: f64 = (var_vmbnew - assign40880_e53917);
        (assign40880_e53918, var_vmbnew_dn4, var_vmbnew_dn6, (var_vmbnew_dn7 - (0.5 * (var_v_ds_dn7 - var_vdsx_dn7))), (var_vmbnew_dn8 - (0.5 * (var_v_ds_dn8 - var_vdsx_dn8))), var_vmbnew_dn9,)
    } else {
        (var_vsbstar_dc, var_vsbstar_dc_dn4, var_vsbstar_dc_dn6, var_vsbstar_dc_dn7, var_vsbstar_dc_dn8, var_vsbstar_dc_dn9,)
    }
};
        var_vsbstar_dc = assign40880_e53920;
        var_vsbstar_dc_dn4 = assign40880_e53920_d_n4;
        var_vsbstar_dc_dn6 = assign40880_e53920_d_n6;
        var_vsbstar_dc_dn7 = assign40880_e53920_d_n7;
        var_vsbstar_dc_dn8 = assign40880_e53920_d_n8;
        var_vsbstar_dc_dn9 = assign40880_e53920_d_n9;
        var_vsbstar_dc_rv = 0.0;

        let (assign40890_e53926, assign40890_e53926_d_n4, assign40890_e53926_d_n6, assign40890_e53926_d_n7, assign40890_e53926_d_n8, assign40890_e53926_d_n9,) = {
    if (var_guard1189 != 0.0) {
        let assign40890_e53924: f64 = (var_vsbstar_dc_tmp - var_vsbstar_dc);
        (assign40890_e53924, (var_vsbstar_dc_tmp_dn4 - var_vsbstar_dc_dn4), (var_vsbstar_dc_tmp_dn6 - var_vsbstar_dc_dn6), (var_vsbstar_dc_tmp_dn7 - var_vsbstar_dc_dn7), (var_vsbstar_dc_tmp_dn8 - var_vsbstar_dc_dn8), (var_vsbstar_dc_tmp_dn9 - var_vsbstar_dc_dn9),)
    } else {
        (var_dvbstar_dc, var_dvbstar_dc_dn4, var_dvbstar_dc_dn6, var_dvbstar_dc_dn7, var_dvbstar_dc_dn8, var_dvbstar_dc_dn9,)
    }
};
        var_dvbstar_dc = assign40890_e53926;
        var_dvbstar_dc_dn4 = assign40890_e53926_d_n4;
        var_dvbstar_dc_dn6 = assign40890_e53926_d_n6;
        var_dvbstar_dc_dn7 = assign40890_e53926_d_n7;
        var_dvbstar_dc_dn8 = assign40890_e53926_d_n8;
        var_dvbstar_dc_dn9 = assign40890_e53926_d_n9;
        var_dvbstar_dc_rv = 0.0;

        var_phib = var_phib_dc;
        var_phib_dn4 = var_phib_dc_dn4;
        var_phib_rv = 0.0;

        var_aphi = var_aphi_dc;
        var_aphi_dn4 = var_aphi_dc_dn4;
        var_aphi_rv = 0.0;

        var_g_0 = var_g_0_dc;
        var_g_0_dn4 = var_g_0_dc_dn4;
        var_g_0_rv = 0.0;

        var_vsbstar = var_vsbstar_dc;
        var_vsbstar_dn4 = var_vsbstar_dc_dn4;
        var_vsbstar_dn6 = var_vsbstar_dc_dn6;
        var_vsbstar_dn7 = var_vsbstar_dc_dn7;
        var_vsbstar_dn8 = var_vsbstar_dc_dn8;
        var_vsbstar_dn9 = var_vsbstar_dc_dn9;
        var_vsbstar_rv = 0.0;

        var_dvbstar = var_dvbstar_dc;
        var_dvbstar_dn4 = var_dvbstar_dc_dn4;
        var_dvbstar_dn6 = var_dvbstar_dc_dn6;
        var_dvbstar_dn7 = var_dvbstar_dc_dn7;
        var_dvbstar_dn8 = var_dvbstar_dc_dn8;
        var_dvbstar_dn9 = var_dvbstar_dc_dn9;
        var_dvbstar_rv = 0.0;

        var_thesatloc = var_thesat_t;
        var_thesatloc_dn4 = var_thesat_t_dn4;
        var_thesatloc_rv = 0.0;

        var_arloc = var_ar;
        var_arloc_rv = 0.0;

        let assign40970_e53936: f64 = (var_vgb - var_dvbstar);
        let assign40970_e53938: f64 = (assign40970_e53936 - var_vfb_t);
        var_vgb1 = assign40970_e53938;
        var_vgb1_dn4 = ((-var_dvbstar_dn4) - var_vfb_t_dn4);
        var_vgb1_dn6 = (var_vgb_dn6 - var_dvbstar_dn6);
        var_vgb1_dn7 = (var_vgb_dn7 - var_dvbstar_dn7);
        var_vgb1_dn8 = (var_vgb_dn8 - var_dvbstar_dn8);
        var_vgb1_dn9 = (var_vgb_dn9 - var_dvbstar_dn9);
        var_vgb1_rv = 0.0;

        let assign40980_e53943: f64 = (var_v_ds - var_vdsx);
        let assign40980_e53944: f64 = (0.5 * assign40980_e53943);
        let assign40980_e53945: f64 = (var_vsbstar + assign40980_e53944);
        var_vsbx = assign40980_e53945;
        var_vsbx_dn4 = var_vsbstar_dn4;
        var_vsbx_dn6 = var_vsbstar_dn6;
        var_vsbx_dn7 = (var_vsbstar_dn7 + (0.5 * (var_v_ds_dn7 - var_vdsx_dn7)));
        var_vsbx_dn8 = (var_vsbstar_dn8 + (0.5 * (var_v_ds_dn8 - var_vdsx_dn8)));
        var_vsbx_dn9 = var_vsbstar_dn9;
        var_vsbx_rv = 0.0;

        var_dctg = 1.0;
        var_dctg_dn4 = 0.0;
        var_dctg_dn6 = 0.0;
        var_dctg_dn7 = 0.0;
        var_dctg_dn8 = 0.0;
        var_dctg_dn9 = 0.0;
        var_dctg_rv = 0.0;

        let assign41000_e53949: f64 = if var_ctg_i > 0.0 { 1.0 } else { 0.0 };
        var_guard1190 = assign41000_e53949;
        var_guard1190_rv = 0.0;

        let (assign41010_e53955, assign41010_e53955_d_n4,) = {
    if (var_guard1190 != 0.0) {
        let assign41010_e53953: f64 = (var_phib * var_inv_phit);
        (assign41010_e53953, ((var_phib_dn4 * var_inv_phit) + (var_phib * var_inv_phit_dn4)),)
    } else {
        (var_xbct, var_xbct_dn4,)
    }
};
        var_xbct = assign41010_e53955;
        var_xbct_dn4 = assign41010_e53955_d_n4;
        var_xbct_rv = 0.0;

        let (assign41020_e53961, assign41020_e53961_d_n4, assign41020_e53961_d_n6, assign41020_e53961_d_n7, assign41020_e53961_d_n8, assign41020_e53961_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41020_e53959: f64 = (var_vsbx * var_inv_phit);
        (assign41020_e53959, ((var_vsbx_dn4 * var_inv_phit) + (var_vsbx * var_inv_phit_dn4)), (var_vsbx_dn6 * var_inv_phit), (var_vsbx_dn7 * var_inv_phit), (var_vsbx_dn8 * var_inv_phit), (var_vsbx_dn9 * var_inv_phit),)
    } else {
        (var_xsbstar, var_xsbstar_dn4, var_xsbstar_dn6, var_xsbstar_dn7, var_xsbstar_dn8, var_xsbstar_dn9,)
    }
};
        var_xsbstar = assign41020_e53961;
        var_xsbstar_dn4 = assign41020_e53961_d_n4;
        var_xsbstar_dn6 = assign41020_e53961_d_n6;
        var_xsbstar_dn7 = assign41020_e53961_d_n7;
        var_xsbstar_dn8 = assign41020_e53961_d_n8;
        var_xsbstar_dn9 = assign41020_e53961_d_n9;
        var_xsbstar_rv = 0.0;

        *var_aphi_slot = var_aphi;
        *var_aphi_dn4_slot = var_aphi_dn4;
        *var_aphi_rv_slot = var_aphi_rv;
        *var_arloc_slot = var_arloc;
        *var_arloc_rv_slot = var_arloc_rv;
        *var_dctg_slot = var_dctg;
        *var_dctg_dn4_slot = var_dctg_dn4;
        *var_dctg_dn6_slot = var_dctg_dn6;
        *var_dctg_dn7_slot = var_dctg_dn7;
        *var_dctg_dn8_slot = var_dctg_dn8;
        *var_dctg_dn9_slot = var_dctg_dn9;
        *var_dctg_rv_slot = var_dctg_rv;
        *var_dvbstar_slot = var_dvbstar;
        *var_dvbstar_dc_slot = var_dvbstar_dc;
        *var_dvbstar_dc_dn4_slot = var_dvbstar_dc_dn4;
        *var_dvbstar_dc_dn6_slot = var_dvbstar_dc_dn6;
        *var_dvbstar_dc_dn7_slot = var_dvbstar_dc_dn7;
        *var_dvbstar_dc_dn8_slot = var_dvbstar_dc_dn8;
        *var_dvbstar_dc_dn9_slot = var_dvbstar_dc_dn9;
        *var_dvbstar_dc_rv_slot = var_dvbstar_dc_rv;
        *var_dvbstar_dn4_slot = var_dvbstar_dn4;
        *var_dvbstar_dn6_slot = var_dvbstar_dn6;
        *var_dvbstar_dn7_slot = var_dvbstar_dn7;
        *var_dvbstar_dn8_slot = var_dvbstar_dn8;
        *var_dvbstar_dn9_slot = var_dvbstar_dn9;
        *var_dvbstar_rv_slot = var_dvbstar_rv;
        *var_g_0_slot = var_g_0;
        *var_g_0_dn4_slot = var_g_0_dn4;
        *var_g_0_rv_slot = var_g_0_rv;
        *var_guard1029_slot = var_guard1029;
        *var_guard1029_rv_slot = var_guard1029_rv;
        *var_guard1189_slot = var_guard1189;
        *var_guard1189_rv_slot = var_guard1189_rv;
        *var_guard1190_slot = var_guard1190;
        *var_guard1190_rv_slot = var_guard1190_rv;
        *var_phib_slot = var_phib;
        *var_phib_dn4_slot = var_phib_dn4;
        *var_phib_rv_slot = var_phib_rv;
        *var_sigvds_slot = var_sigvds;
        *var_sigvds_rv_slot = var_sigvds_rv;
        *var_temp__blk949_slot = var_temp__blk949;
        *var_temp__blk949_dn4_slot = var_temp__blk949_dn4;
        *var_temp__blk949_dn6_slot = var_temp__blk949_dn6;
        *var_temp__blk949_dn7_slot = var_temp__blk949_dn7;
        *var_temp__blk949_dn8_slot = var_temp__blk949_dn8;
        *var_temp__blk949_dn9_slot = var_temp__blk949_dn9;
        *var_temp__blk949_rv_slot = var_temp__blk949_rv;
        *var_thesatloc_slot = var_thesatloc;
        *var_thesatloc_dn4_slot = var_thesatloc_dn4;
        *var_thesatloc_rv_slot = var_thesatloc_rv;
        *var_us_slot = var_us;
        *var_us_dn4_slot = var_us_dn4;
        *var_us_dn6_slot = var_us_dn6;
        *var_us_dn7_slot = var_us_dn7;
        *var_us_dn8_slot = var_us_dn8;
        *var_us_dn9_slot = var_us_dn9;
        *var_us_rv_slot = var_us_rv;
        *var_usnew_slot = var_usnew;
        *var_usnew_dn4_slot = var_usnew_dn4;
        *var_usnew_dn6_slot = var_usnew_dn6;
        *var_usnew_dn7_slot = var_usnew_dn7;
        *var_usnew_dn8_slot = var_usnew_dn8;
        *var_usnew_dn9_slot = var_usnew_dn9;
        *var_usnew_rv_slot = var_usnew_rv;
        *var_v_db_slot = var_v_db;
        *var_v_db_dn7_slot = var_v_db_dn7;
        *var_v_db_dn8_slot = var_v_db_dn8;
        *var_v_db_dn9_slot = var_v_db_dn9;
        *var_v_db_rv_slot = var_v_db_rv;
        *var_v_ds_slot = var_v_ds;
        *var_v_ds_dn7_slot = var_v_ds_dn7;
        *var_v_ds_dn8_slot = var_v_ds_dn8;
        *var_v_ds_rv_slot = var_v_ds_rv;
        *var_v_gs_slot = var_v_gs;
        *var_v_gs_dn6_slot = var_v_gs_dn6;
        *var_v_gs_dn7_slot = var_v_gs_dn7;
        *var_v_gs_dn8_slot = var_v_gs_dn8;
        *var_v_gs_rv_slot = var_v_gs_rv;
        *var_v_sb_slot = var_v_sb;
        *var_v_sb_dn7_slot = var_v_sb_dn7;
        *var_v_sb_dn8_slot = var_v_sb_dn8;
        *var_v_sb_dn9_slot = var_v_sb_dn9;
        *var_v_sb_rv_slot = var_v_sb_rv;
        *var_v_xb_slot = var_v_xb;
        *var_v_xb_dc_tmp_slot = var_v_xb_dc_tmp;
        *var_v_xb_dc_tmp_dn4_slot = var_v_xb_dc_tmp_dn4;
        *var_v_xb_dc_tmp_dn7_slot = var_v_xb_dc_tmp_dn7;
        *var_v_xb_dc_tmp_dn8_slot = var_v_xb_dc_tmp_dn8;
        *var_v_xb_dc_tmp_dn9_slot = var_v_xb_dc_tmp_dn9;
        *var_v_xb_dc_tmp_rv_slot = var_v_xb_dc_tmp_rv;
        *var_v_xb_dn4_slot = var_v_xb_dn4;
        *var_v_xb_dn7_slot = var_v_xb_dn7;
        *var_v_xb_dn8_slot = var_v_xb_dn8;
        *var_v_xb_dn9_slot = var_v_xb_dn9;
        *var_v_xb_rv_slot = var_v_xb_rv;
        *var_vdbprime_slot = var_vdbprime;
        *var_vdbprime_dn7_slot = var_vdbprime_dn7;
        *var_vdbprime_dn8_slot = var_vdbprime_dn8;
        *var_vdbprime_dn9_slot = var_vdbprime_dn9;
        *var_vdbprime_rv_slot = var_vdbprime_rv;
        *var_vdsx_slot = var_vdsx;
        *var_vdsx_dn7_slot = var_vdsx_dn7;
        *var_vdsx_dn8_slot = var_vdsx_dn8;
        *var_vdsx_rv_slot = var_vdsx_rv;
        *var_vgb_slot = var_vgb;
        *var_vgb1_slot = var_vgb1;
        *var_vgb1_dn4_slot = var_vgb1_dn4;
        *var_vgb1_dn6_slot = var_vgb1_dn6;
        *var_vgb1_dn7_slot = var_vgb1_dn7;
        *var_vgb1_dn8_slot = var_vgb1_dn8;
        *var_vgb1_dn9_slot = var_vgb1_dn9;
        *var_vgb1_rv_slot = var_vgb1_rv;
        *var_vgb_dn6_slot = var_vgb_dn6;
        *var_vgb_dn7_slot = var_vgb_dn7;
        *var_vgb_dn8_slot = var_vgb_dn8;
        *var_vgb_dn9_slot = var_vgb_dn9;
        *var_vgb_rv_slot = var_vgb_rv;
        *var_vgdprime_slot = var_vgdprime;
        *var_vgdprime_dn6_slot = var_vgdprime_dn6;
        *var_vgdprime_dn7_slot = var_vgdprime_dn7;
        *var_vgdprime_dn8_slot = var_vgdprime_dn8;
        *var_vgdprime_rv_slot = var_vgdprime_rv;
        *var_vgsprime_slot = var_vgsprime;
        *var_vgsprime_dn6_slot = var_vgsprime_dn6;
        *var_vgsprime_dn7_slot = var_vgsprime_dn7;
        *var_vgsprime_dn8_slot = var_vgsprime_dn8;
        *var_vgsprime_rv_slot = var_vgsprime_rv;
        *var_vjun_d_slot = var_vjun_d;
        *var_vjun_d_dn12_slot = var_vjun_d_dn12;
        *var_vjun_d_dn8_slot = var_vjun_d_dn8;
        *var_vjun_d_rv_slot = var_vjun_d_rv;
        *var_vjun_s_slot = var_vjun_s;
        *var_vjun_s_dn11_slot = var_vjun_s_dn11;
        *var_vjun_s_dn7_slot = var_vjun_s_dn7;
        *var_vjun_s_rv_slot = var_vjun_s_rv;
        *var_vmb_slot = var_vmb;
        *var_vmb_dn4_slot = var_vmb_dn4;
        *var_vmb_dn6_slot = var_vmb_dn6;
        *var_vmb_dn7_slot = var_vmb_dn7;
        *var_vmb_dn8_slot = var_vmb_dn8;
        *var_vmb_dn9_slot = var_vmb_dn9;
        *var_vmb_rv_slot = var_vmb_rv;
        *var_vmbnew_slot = var_vmbnew;
        *var_vmbnew_dn4_slot = var_vmbnew_dn4;
        *var_vmbnew_dn6_slot = var_vmbnew_dn6;
        *var_vmbnew_dn7_slot = var_vmbnew_dn7;
        *var_vmbnew_dn8_slot = var_vmbnew_dn8;
        *var_vmbnew_dn9_slot = var_vmbnew_dn9;
        *var_vmbnew_rv_slot = var_vmbnew_rv;
        *var_vsbprime_slot = var_vsbprime;
        *var_vsbprime_dn7_slot = var_vsbprime_dn7;
        *var_vsbprime_dn8_slot = var_vsbprime_dn8;
        *var_vsbprime_dn9_slot = var_vsbprime_dn9;
        *var_vsbprime_rv_slot = var_vsbprime_rv;
        *var_vsbstar_slot = var_vsbstar;
        *var_vsbstar_dc_slot = var_vsbstar_dc;
        *var_vsbstar_dc_dn4_slot = var_vsbstar_dc_dn4;
        *var_vsbstar_dc_dn6_slot = var_vsbstar_dc_dn6;
        *var_vsbstar_dc_dn7_slot = var_vsbstar_dc_dn7;
        *var_vsbstar_dc_dn8_slot = var_vsbstar_dc_dn8;
        *var_vsbstar_dc_dn9_slot = var_vsbstar_dc_dn9;
        *var_vsbstar_dc_rv_slot = var_vsbstar_dc_rv;
        *var_vsbstar_dc_tmp_slot = var_vsbstar_dc_tmp;
        *var_vsbstar_dc_tmp_dn4_slot = var_vsbstar_dc_tmp_dn4;
        *var_vsbstar_dc_tmp_dn6_slot = var_vsbstar_dc_tmp_dn6;
        *var_vsbstar_dc_tmp_dn7_slot = var_vsbstar_dc_tmp_dn7;
        *var_vsbstar_dc_tmp_dn8_slot = var_vsbstar_dc_tmp_dn8;
        *var_vsbstar_dc_tmp_dn9_slot = var_vsbstar_dc_tmp_dn9;
        *var_vsbstar_dc_tmp_rv_slot = var_vsbstar_dc_tmp_rv;
        *var_vsbstar_dn4_slot = var_vsbstar_dn4;
        *var_vsbstar_dn6_slot = var_vsbstar_dn6;
        *var_vsbstar_dn7_slot = var_vsbstar_dn7;
        *var_vsbstar_dn8_slot = var_vsbstar_dn8;
        *var_vsbstar_dn9_slot = var_vsbstar_dn9;
        *var_vsbstar_rv_slot = var_vsbstar_rv;
        *var_vsbx_slot = var_vsbx;
        *var_vsbx_dn4_slot = var_vsbx_dn4;
        *var_vsbx_dn6_slot = var_vsbx_dn6;
        *var_vsbx_dn7_slot = var_vsbx_dn7;
        *var_vsbx_dn8_slot = var_vsbx_dn8;
        *var_vsbx_dn9_slot = var_vsbx_dn9;
        *var_vsbx_rv_slot = var_vsbx_rv;
        *var_xbct_slot = var_xbct;
        *var_xbct_dn4_slot = var_xbct_dn4;
        *var_xbct_rv_slot = var_xbct_rv;
        *var_xgb_ov_slot = var_xgb_ov;
        *var_xgb_ov_dn4_slot = var_xgb_ov_dn4;
        *var_xgb_ov_dn6_slot = var_xgb_ov_dn6;
        *var_xgb_ov_dn7_slot = var_xgb_ov_dn7;
        *var_xgb_ov_dn8_slot = var_xgb_ov_dn8;
        *var_xgb_ov_dn9_slot = var_xgb_ov_dn9;
        *var_xgb_ov_rv_slot = var_xgb_ov_rv;
        *var_xgd_ov_slot = var_xgd_ov;
        *var_xgd_ov_dn6_slot = var_xgd_ov_dn6;
        *var_xgd_ov_dn7_slot = var_xgd_ov_dn7;
        *var_xgd_ov_dn8_slot = var_xgd_ov_dn8;
        *var_xgd_ov_rv_slot = var_xgd_ov_rv;
        *var_xgs_ov_slot = var_xgs_ov;
        *var_xgs_ov_dn6_slot = var_xgs_ov_dn6;
        *var_xgs_ov_dn7_slot = var_xgs_ov_dn7;
        *var_xgs_ov_dn8_slot = var_xgs_ov_dn8;
        *var_xgs_ov_rv_slot = var_xgs_ov_rv;
        *var_xsbstar_slot = var_xsbstar;
        *var_xsbstar_dn4_slot = var_xsbstar_dn4;
        *var_xsbstar_dn6_slot = var_xsbstar_dn6;
        *var_xsbstar_dn7_slot = var_xsbstar_dn7;
        *var_xsbstar_dn8_slot = var_xsbstar_dn8;
        *var_xsbstar_dn9_slot = var_xsbstar_dn9;
        *var_xsbstar_rv_slot = var_xsbstar_rv;
    }

    pub(super) fn stamp_reactive_block_24(
        var_aphi: f64,
        var_aphi_dn4: f64,
        var_cf_i: f64,
        var_cfb_i: f64,
        var_cfd_i: f64,
        var_ct_t: f64,
        var_ct_t_dn4: f64,
        var_ctb_i: f64,
        var_ctg_t: f64,
        var_ctg_t_dn4: f64,
        var_g_0: f64,
        var_g_0_dn4: f64,
        var_guard1190: f64,
        var_inv_phit: f64,
        var_inv_phit_dn4: f64,
        var_phib: f64,
        var_phib_dn4: f64,
        var_phit: f64,
        var_phit_dn4: f64,
        var_psce_i: f64,
        var_psceb_i: f64,
        var_psced_i: f64,
        var_v_xb: f64,
        var_v_xb_dn4: f64,
        var_v_xb_dn7: f64,
        var_v_xb_dn8: f64,
        var_v_xb_dn9: f64,
        var_vdsx: f64,
        var_vdsx_dn7: f64,
        var_vdsx_dn8: f64,
        var_vgb1: f64,
        var_vgb1_dn4: f64,
        var_vgb1_dn6: f64,
        var_vgb1_dn7: f64,
        var_vgb1_dn8: f64,
        var_vgb1_dn9: f64,
        var_vsbstar: f64,
        var_vsbstar_dn4: f64,
        var_vsbstar_dn6: f64,
        var_vsbstar_dn7: f64,
        var_vsbstar_dn8: f64,
        var_vsbstar_dn9: f64,
        var_vsbx: f64,
        var_vsbx_dn4: f64,
        var_vsbx_dn6: f64,
        var_vsbx_dn7: f64,
        var_vsbx_dn8: f64,
        var_vsbx_dn9: f64,
        var_xbct: f64,
        var_xbct_dn4: f64,
        var_xsbstar: f64,
        var_xsbstar_dn4: f64,
        var_xsbstar_dn6: f64,
        var_xsbstar_dn7: f64,
        var_xsbstar_dn8: f64,
        var_xsbstar_dn9: f64,
        var_ct_fact_slot: &mut f64,
        var_ct_fact_dn4_slot: &mut f64,
        var_ct_fact_dn6_slot: &mut f64,
        var_ct_fact_dn7_slot: &mut f64,
        var_ct_fact_dn8_slot: &mut f64,
        var_ct_fact_dn9_slot: &mut f64,
        var_ct_fact_rv_slot: &mut f64,
        var_dctg_slot: &mut f64,
        var_dctg_dn4_slot: &mut f64,
        var_dctg_dn6_slot: &mut f64,
        var_dctg_dn7_slot: &mut f64,
        var_dctg_dn8_slot: &mut f64,
        var_dctg_dn9_slot: &mut f64,
        var_dctg_rv_slot: &mut f64,
        var_delphib_slot: &mut f64,
        var_delphib_dn4_slot: &mut f64,
        var_delphib_dn6_slot: &mut f64,
        var_delphib_dn7_slot: &mut f64,
        var_delphib_dn8_slot: &mut f64,
        var_delphib_dn9_slot: &mut f64,
        var_delphib_rv_slot: &mut f64,
        var_delxb_slot: &mut f64,
        var_delxb_dn4_slot: &mut f64,
        var_delxb_dn6_slot: &mut f64,
        var_delxb_dn7_slot: &mut f64,
        var_delxb_dn8_slot: &mut f64,
        var_delxb_dn9_slot: &mut f64,
        var_delxb_rv_slot: &mut f64,
        var_dphit1_slot: &mut f64,
        var_dphit1_dn4_slot: &mut f64,
        var_dphit1_dn6_slot: &mut f64,
        var_dphit1_dn7_slot: &mut f64,
        var_dphit1_dn8_slot: &mut f64,
        var_dphit1_dn9_slot: &mut f64,
        var_dphit1_rv_slot: &mut f64,
        var_gf_slot: &mut f64,
        var_gf2_slot: &mut f64,
        var_gf2_dn4_slot: &mut f64,
        var_gf2_dn6_slot: &mut f64,
        var_gf2_dn7_slot: &mut f64,
        var_gf2_dn8_slot: &mut f64,
        var_gf2_dn9_slot: &mut f64,
        var_gf2_rv_slot: &mut f64,
        var_gf_dn4_slot: &mut f64,
        var_gf_dn6_slot: &mut f64,
        var_gf_dn7_slot: &mut f64,
        var_gf_dn8_slot: &mut f64,
        var_gf_dn9_slot: &mut f64,
        var_gf_rv_slot: &mut f64,
        var_guard1191_slot: &mut f64,
        var_guard1191_rv_slot: &mut f64,
        var_inv_gf2_slot: &mut f64,
        var_inv_gf2_dn4_slot: &mut f64,
        var_inv_gf2_dn6_slot: &mut f64,
        var_inv_gf2_dn7_slot: &mut f64,
        var_inv_gf2_dn8_slot: &mut f64,
        var_inv_gf2_dn9_slot: &mut f64,
        var_inv_gf2_rv_slot: &mut f64,
        var_inv_phit1_slot: &mut f64,
        var_inv_phit1_dn4_slot: &mut f64,
        var_inv_phit1_dn6_slot: &mut f64,
        var_inv_phit1_dn7_slot: &mut f64,
        var_inv_phit1_dn8_slot: &mut f64,
        var_inv_phit1_dn9_slot: &mut f64,
        var_inv_phit1_rv_slot: &mut f64,
        var_phit1_slot: &mut f64,
        var_phit1_dn4_slot: &mut f64,
        var_phit1_dn6_slot: &mut f64,
        var_phit1_dn7_slot: &mut f64,
        var_phit1_dn8_slot: &mut f64,
        var_phit1_dn9_slot: &mut f64,
        var_phit1_rv_slot: &mut f64,
        var_phitct_slot: &mut f64,
        var_phitct_dn4_slot: &mut f64,
        var_phitct_dn6_slot: &mut f64,
        var_phitct_dn7_slot: &mut f64,
        var_phitct_dn8_slot: &mut f64,
        var_phitct_dn9_slot: &mut f64,
        var_phitct_rv_slot: &mut f64,
        var_temp1_slot: &mut f64,
        var_temp1_dn4_slot: &mut f64,
        var_temp1_dn6_slot: &mut f64,
        var_temp1_dn7_slot: &mut f64,
        var_temp1_dn8_slot: &mut f64,
        var_temp1_dn9_slot: &mut f64,
        var_temp1_rv_slot: &mut f64,
        var_temp2_slot: &mut f64,
        var_temp2_dn4_slot: &mut f64,
        var_temp2_dn6_slot: &mut f64,
        var_temp2_dn7_slot: &mut f64,
        var_temp2_dn8_slot: &mut f64,
        var_temp2_dn9_slot: &mut f64,
        var_temp2_rv_slot: &mut f64,
        var_ux_slot: &mut f64,
        var_ux_dn4_slot: &mut f64,
        var_ux_dn6_slot: &mut f64,
        var_ux_dn7_slot: &mut f64,
        var_ux_dn8_slot: &mut f64,
        var_ux_dn9_slot: &mut f64,
        var_ux_rv_slot: &mut f64,
        var_vdsp_slot: &mut f64,
        var_vdsp_dn7_slot: &mut f64,
        var_vdsp_dn8_slot: &mut f64,
        var_vdsp_rv_slot: &mut f64,
        var_xb_slot: &mut f64,
        var_xb_dn4_slot: &mut f64,
        var_xb_dn6_slot: &mut f64,
        var_xb_dn7_slot: &mut f64,
        var_xb_dn8_slot: &mut f64,
        var_xb_dn9_slot: &mut f64,
        var_xb_rv_slot: &mut f64,
        var_xct_slot: &mut f64,
        var_xct_dn4_slot: &mut f64,
        var_xct_dn6_slot: &mut f64,
        var_xct_dn7_slot: &mut f64,
        var_xct_dn8_slot: &mut f64,
        var_xct_dn9_slot: &mut f64,
        var_xct_rv_slot: &mut f64,
        var_xctmax_slot: &mut f64,
        var_xctmax_dn4_slot: &mut f64,
        var_xctmax_rv_slot: &mut f64,
        var_xg_slot: &mut f64,
        var_xg_dn4_slot: &mut f64,
        var_xg_dn6_slot: &mut f64,
        var_xg_dn7_slot: &mut f64,
        var_xg_dn8_slot: &mut f64,
        var_xg_dn9_slot: &mut f64,
        var_xg_rv_slot: &mut f64,
        var_xgct_slot: &mut f64,
        var_xgct_dn4_slot: &mut f64,
        var_xgct_dn6_slot: &mut f64,
        var_xgct_dn7_slot: &mut f64,
        var_xgct_dn8_slot: &mut f64,
        var_xgct_dn9_slot: &mut f64,
        var_xgct_rv_slot: &mut f64,
        var_xmict_slot: &mut f64,
        var_xmict_dn4_slot: &mut f64,
        var_xmict_dn6_slot: &mut f64,
        var_xmict_dn7_slot: &mut f64,
        var_xmict_dn8_slot: &mut f64,
        var_xmict_dn9_slot: &mut f64,
        var_xmict_rv_slot: &mut f64,
        var_xn_s_slot: &mut f64,
        var_xn_s_dn4_slot: &mut f64,
        var_xn_s_dn6_slot: &mut f64,
        var_xn_s_dn7_slot: &mut f64,
        var_xn_s_dn8_slot: &mut f64,
        var_xn_s_dn9_slot: &mut f64,
        var_xn_s_rv_slot: &mut f64,
        var_xnct_slot: &mut f64,
        var_xnct_dn4_slot: &mut f64,
        var_xnct_dn6_slot: &mut f64,
        var_xnct_dn7_slot: &mut f64,
        var_xnct_dn8_slot: &mut f64,
        var_xnct_dn9_slot: &mut f64,
        var_xnct_rv_slot: &mut f64,
        var_xno_s_slot: &mut f64,
        var_xno_s_dn4_slot: &mut f64,
        var_xno_s_dn6_slot: &mut f64,
        var_xno_s_dn7_slot: &mut f64,
        var_xno_s_dn8_slot: &mut f64,
        var_xno_s_dn9_slot: &mut f64,
        var_xno_s_rv_slot: &mut f64,
        var_xsubct_slot: &mut f64,
        var_xsubct_dn4_slot: &mut f64,
        var_xsubct_dn6_slot: &mut f64,
        var_xsubct_dn7_slot: &mut f64,
        var_xsubct_dn8_slot: &mut f64,
        var_xsubct_dn9_slot: &mut f64,
        var_xsubct_rv_slot: &mut f64,
        var_xwict_slot: &mut f64,
        var_xwict_dn4_slot: &mut f64,
        var_xwict_dn6_slot: &mut f64,
        var_xwict_dn7_slot: &mut f64,
        var_xwict_dn8_slot: &mut f64,
        var_xwict_dn9_slot: &mut f64,
        var_xwict_rv_slot: &mut f64,
    ) {
        let mut var_ct_fact: f64 = *var_ct_fact_slot;
        let mut var_ct_fact_dn4: f64 = *var_ct_fact_dn4_slot;
        let mut var_ct_fact_dn6: f64 = *var_ct_fact_dn6_slot;
        let mut var_ct_fact_dn7: f64 = *var_ct_fact_dn7_slot;
        let mut var_ct_fact_dn8: f64 = *var_ct_fact_dn8_slot;
        let mut var_ct_fact_dn9: f64 = *var_ct_fact_dn9_slot;
        let mut var_ct_fact_rv: f64 = *var_ct_fact_rv_slot;
        let mut var_dctg: f64 = *var_dctg_slot;
        let mut var_dctg_dn4: f64 = *var_dctg_dn4_slot;
        let mut var_dctg_dn6: f64 = *var_dctg_dn6_slot;
        let mut var_dctg_dn7: f64 = *var_dctg_dn7_slot;
        let mut var_dctg_dn8: f64 = *var_dctg_dn8_slot;
        let mut var_dctg_dn9: f64 = *var_dctg_dn9_slot;
        let mut var_dctg_rv: f64 = *var_dctg_rv_slot;
        let mut var_delphib: f64 = *var_delphib_slot;
        let mut var_delphib_dn4: f64 = *var_delphib_dn4_slot;
        let mut var_delphib_dn6: f64 = *var_delphib_dn6_slot;
        let mut var_delphib_dn7: f64 = *var_delphib_dn7_slot;
        let mut var_delphib_dn8: f64 = *var_delphib_dn8_slot;
        let mut var_delphib_dn9: f64 = *var_delphib_dn9_slot;
        let mut var_delphib_rv: f64 = *var_delphib_rv_slot;
        let mut var_delxb: f64 = *var_delxb_slot;
        let mut var_delxb_dn4: f64 = *var_delxb_dn4_slot;
        let mut var_delxb_dn6: f64 = *var_delxb_dn6_slot;
        let mut var_delxb_dn7: f64 = *var_delxb_dn7_slot;
        let mut var_delxb_dn8: f64 = *var_delxb_dn8_slot;
        let mut var_delxb_dn9: f64 = *var_delxb_dn9_slot;
        let mut var_delxb_rv: f64 = *var_delxb_rv_slot;
        let mut var_dphit1: f64 = *var_dphit1_slot;
        let mut var_dphit1_dn4: f64 = *var_dphit1_dn4_slot;
        let mut var_dphit1_dn6: f64 = *var_dphit1_dn6_slot;
        let mut var_dphit1_dn7: f64 = *var_dphit1_dn7_slot;
        let mut var_dphit1_dn8: f64 = *var_dphit1_dn8_slot;
        let mut var_dphit1_dn9: f64 = *var_dphit1_dn9_slot;
        let mut var_dphit1_rv: f64 = *var_dphit1_rv_slot;
        let mut var_gf: f64 = *var_gf_slot;
        let mut var_gf2: f64 = *var_gf2_slot;
        let mut var_gf2_dn4: f64 = *var_gf2_dn4_slot;
        let mut var_gf2_dn6: f64 = *var_gf2_dn6_slot;
        let mut var_gf2_dn7: f64 = *var_gf2_dn7_slot;
        let mut var_gf2_dn8: f64 = *var_gf2_dn8_slot;
        let mut var_gf2_dn9: f64 = *var_gf2_dn9_slot;
        let mut var_gf2_rv: f64 = *var_gf2_rv_slot;
        let mut var_gf_dn4: f64 = *var_gf_dn4_slot;
        let mut var_gf_dn6: f64 = *var_gf_dn6_slot;
        let mut var_gf_dn7: f64 = *var_gf_dn7_slot;
        let mut var_gf_dn8: f64 = *var_gf_dn8_slot;
        let mut var_gf_dn9: f64 = *var_gf_dn9_slot;
        let mut var_gf_rv: f64 = *var_gf_rv_slot;
        let mut var_guard1191: f64 = *var_guard1191_slot;
        let mut var_guard1191_rv: f64 = *var_guard1191_rv_slot;
        let mut var_inv_gf2: f64 = *var_inv_gf2_slot;
        let mut var_inv_gf2_dn4: f64 = *var_inv_gf2_dn4_slot;
        let mut var_inv_gf2_dn6: f64 = *var_inv_gf2_dn6_slot;
        let mut var_inv_gf2_dn7: f64 = *var_inv_gf2_dn7_slot;
        let mut var_inv_gf2_dn8: f64 = *var_inv_gf2_dn8_slot;
        let mut var_inv_gf2_dn9: f64 = *var_inv_gf2_dn9_slot;
        let mut var_inv_gf2_rv: f64 = *var_inv_gf2_rv_slot;
        let mut var_inv_phit1: f64 = *var_inv_phit1_slot;
        let mut var_inv_phit1_dn4: f64 = *var_inv_phit1_dn4_slot;
        let mut var_inv_phit1_dn6: f64 = *var_inv_phit1_dn6_slot;
        let mut var_inv_phit1_dn7: f64 = *var_inv_phit1_dn7_slot;
        let mut var_inv_phit1_dn8: f64 = *var_inv_phit1_dn8_slot;
        let mut var_inv_phit1_dn9: f64 = *var_inv_phit1_dn9_slot;
        let mut var_inv_phit1_rv: f64 = *var_inv_phit1_rv_slot;
        let mut var_phit1: f64 = *var_phit1_slot;
        let mut var_phit1_dn4: f64 = *var_phit1_dn4_slot;
        let mut var_phit1_dn6: f64 = *var_phit1_dn6_slot;
        let mut var_phit1_dn7: f64 = *var_phit1_dn7_slot;
        let mut var_phit1_dn8: f64 = *var_phit1_dn8_slot;
        let mut var_phit1_dn9: f64 = *var_phit1_dn9_slot;
        let mut var_phit1_rv: f64 = *var_phit1_rv_slot;
        let mut var_phitct: f64 = *var_phitct_slot;
        let mut var_phitct_dn4: f64 = *var_phitct_dn4_slot;
        let mut var_phitct_dn6: f64 = *var_phitct_dn6_slot;
        let mut var_phitct_dn7: f64 = *var_phitct_dn7_slot;
        let mut var_phitct_dn8: f64 = *var_phitct_dn8_slot;
        let mut var_phitct_dn9: f64 = *var_phitct_dn9_slot;
        let mut var_phitct_rv: f64 = *var_phitct_rv_slot;
        let mut var_temp1: f64 = *var_temp1_slot;
        let mut var_temp1_dn4: f64 = *var_temp1_dn4_slot;
        let mut var_temp1_dn6: f64 = *var_temp1_dn6_slot;
        let mut var_temp1_dn7: f64 = *var_temp1_dn7_slot;
        let mut var_temp1_dn8: f64 = *var_temp1_dn8_slot;
        let mut var_temp1_dn9: f64 = *var_temp1_dn9_slot;
        let mut var_temp1_rv: f64 = *var_temp1_rv_slot;
        let mut var_temp2: f64 = *var_temp2_slot;
        let mut var_temp2_dn4: f64 = *var_temp2_dn4_slot;
        let mut var_temp2_dn6: f64 = *var_temp2_dn6_slot;
        let mut var_temp2_dn7: f64 = *var_temp2_dn7_slot;
        let mut var_temp2_dn8: f64 = *var_temp2_dn8_slot;
        let mut var_temp2_dn9: f64 = *var_temp2_dn9_slot;
        let mut var_temp2_rv: f64 = *var_temp2_rv_slot;
        let mut var_ux: f64 = *var_ux_slot;
        let mut var_ux_dn4: f64 = *var_ux_dn4_slot;
        let mut var_ux_dn6: f64 = *var_ux_dn6_slot;
        let mut var_ux_dn7: f64 = *var_ux_dn7_slot;
        let mut var_ux_dn8: f64 = *var_ux_dn8_slot;
        let mut var_ux_dn9: f64 = *var_ux_dn9_slot;
        let mut var_ux_rv: f64 = *var_ux_rv_slot;
        let mut var_vdsp: f64 = *var_vdsp_slot;
        let mut var_vdsp_dn7: f64 = *var_vdsp_dn7_slot;
        let mut var_vdsp_dn8: f64 = *var_vdsp_dn8_slot;
        let mut var_vdsp_rv: f64 = *var_vdsp_rv_slot;
        let mut var_xb: f64 = *var_xb_slot;
        let mut var_xb_dn4: f64 = *var_xb_dn4_slot;
        let mut var_xb_dn6: f64 = *var_xb_dn6_slot;
        let mut var_xb_dn7: f64 = *var_xb_dn7_slot;
        let mut var_xb_dn8: f64 = *var_xb_dn8_slot;
        let mut var_xb_dn9: f64 = *var_xb_dn9_slot;
        let mut var_xb_rv: f64 = *var_xb_rv_slot;
        let mut var_xct: f64 = *var_xct_slot;
        let mut var_xct_dn4: f64 = *var_xct_dn4_slot;
        let mut var_xct_dn6: f64 = *var_xct_dn6_slot;
        let mut var_xct_dn7: f64 = *var_xct_dn7_slot;
        let mut var_xct_dn8: f64 = *var_xct_dn8_slot;
        let mut var_xct_dn9: f64 = *var_xct_dn9_slot;
        let mut var_xct_rv: f64 = *var_xct_rv_slot;
        let mut var_xctmax: f64 = *var_xctmax_slot;
        let mut var_xctmax_dn4: f64 = *var_xctmax_dn4_slot;
        let mut var_xctmax_rv: f64 = *var_xctmax_rv_slot;
        let mut var_xg: f64 = *var_xg_slot;
        let mut var_xg_dn4: f64 = *var_xg_dn4_slot;
        let mut var_xg_dn6: f64 = *var_xg_dn6_slot;
        let mut var_xg_dn7: f64 = *var_xg_dn7_slot;
        let mut var_xg_dn8: f64 = *var_xg_dn8_slot;
        let mut var_xg_dn9: f64 = *var_xg_dn9_slot;
        let mut var_xg_rv: f64 = *var_xg_rv_slot;
        let mut var_xgct: f64 = *var_xgct_slot;
        let mut var_xgct_dn4: f64 = *var_xgct_dn4_slot;
        let mut var_xgct_dn6: f64 = *var_xgct_dn6_slot;
        let mut var_xgct_dn7: f64 = *var_xgct_dn7_slot;
        let mut var_xgct_dn8: f64 = *var_xgct_dn8_slot;
        let mut var_xgct_dn9: f64 = *var_xgct_dn9_slot;
        let mut var_xgct_rv: f64 = *var_xgct_rv_slot;
        let mut var_xmict: f64 = *var_xmict_slot;
        let mut var_xmict_dn4: f64 = *var_xmict_dn4_slot;
        let mut var_xmict_dn6: f64 = *var_xmict_dn6_slot;
        let mut var_xmict_dn7: f64 = *var_xmict_dn7_slot;
        let mut var_xmict_dn8: f64 = *var_xmict_dn8_slot;
        let mut var_xmict_dn9: f64 = *var_xmict_dn9_slot;
        let mut var_xmict_rv: f64 = *var_xmict_rv_slot;
        let mut var_xn_s: f64 = *var_xn_s_slot;
        let mut var_xn_s_dn4: f64 = *var_xn_s_dn4_slot;
        let mut var_xn_s_dn6: f64 = *var_xn_s_dn6_slot;
        let mut var_xn_s_dn7: f64 = *var_xn_s_dn7_slot;
        let mut var_xn_s_dn8: f64 = *var_xn_s_dn8_slot;
        let mut var_xn_s_dn9: f64 = *var_xn_s_dn9_slot;
        let mut var_xn_s_rv: f64 = *var_xn_s_rv_slot;
        let mut var_xnct: f64 = *var_xnct_slot;
        let mut var_xnct_dn4: f64 = *var_xnct_dn4_slot;
        let mut var_xnct_dn6: f64 = *var_xnct_dn6_slot;
        let mut var_xnct_dn7: f64 = *var_xnct_dn7_slot;
        let mut var_xnct_dn8: f64 = *var_xnct_dn8_slot;
        let mut var_xnct_dn9: f64 = *var_xnct_dn9_slot;
        let mut var_xnct_rv: f64 = *var_xnct_rv_slot;
        let mut var_xno_s: f64 = *var_xno_s_slot;
        let mut var_xno_s_dn4: f64 = *var_xno_s_dn4_slot;
        let mut var_xno_s_dn6: f64 = *var_xno_s_dn6_slot;
        let mut var_xno_s_dn7: f64 = *var_xno_s_dn7_slot;
        let mut var_xno_s_dn8: f64 = *var_xno_s_dn8_slot;
        let mut var_xno_s_dn9: f64 = *var_xno_s_dn9_slot;
        let mut var_xno_s_rv: f64 = *var_xno_s_rv_slot;
        let mut var_xsubct: f64 = *var_xsubct_slot;
        let mut var_xsubct_dn4: f64 = *var_xsubct_dn4_slot;
        let mut var_xsubct_dn6: f64 = *var_xsubct_dn6_slot;
        let mut var_xsubct_dn7: f64 = *var_xsubct_dn7_slot;
        let mut var_xsubct_dn8: f64 = *var_xsubct_dn8_slot;
        let mut var_xsubct_dn9: f64 = *var_xsubct_dn9_slot;
        let mut var_xsubct_rv: f64 = *var_xsubct_rv_slot;
        let mut var_xwict: f64 = *var_xwict_slot;
        let mut var_xwict_dn4: f64 = *var_xwict_dn4_slot;
        let mut var_xwict_dn6: f64 = *var_xwict_dn6_slot;
        let mut var_xwict_dn7: f64 = *var_xwict_dn7_slot;
        let mut var_xwict_dn8: f64 = *var_xwict_dn8_slot;
        let mut var_xwict_dn9: f64 = *var_xwict_dn9_slot;
        let mut var_xwict_rv: f64 = *var_xwict_rv_slot;

        let (assign41030_e53967, assign41030_e53967_d_n4, assign41030_e53967_d_n6, assign41030_e53967_d_n7, assign41030_e53967_d_n8, assign41030_e53967_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41030_e53965: f64 = (var_vgb1 * var_inv_phit);
        (assign41030_e53965, ((var_vgb1_dn4 * var_inv_phit) + (var_vgb1 * var_inv_phit_dn4)), (var_vgb1_dn6 * var_inv_phit), (var_vgb1_dn7 * var_inv_phit), (var_vgb1_dn8 * var_inv_phit), (var_vgb1_dn9 * var_inv_phit),)
    } else {
        (var_xgct, var_xgct_dn4, var_xgct_dn6, var_xgct_dn7, var_xgct_dn8, var_xgct_dn9,)
    }
};
        var_xgct = assign41030_e53967;
        var_xgct_dn4 = assign41030_e53967_d_n4;
        var_xgct_dn6 = assign41030_e53967_d_n6;
        var_xgct_dn7 = assign41030_e53967_d_n7;
        var_xgct_dn8 = assign41030_e53967_d_n8;
        var_xgct_dn9 = assign41030_e53967_d_n9;
        var_xgct_rv = 0.0;

        let (assign41040_e53978, assign41040_e53978_d_n4, assign41040_e53978_d_n6, assign41040_e53978_d_n7, assign41040_e53978_d_n8, assign41040_e53978_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41040_e53972: f64 = (0.5 * var_g_0);
        let assign41040_e53974: f64 = (var_xbct).sqrt();
        let assign41040_e53975: f64 = (assign41040_e53972 / assign41040_e53974);
        let assign41040_e53976: f64 = (1.0 + assign41040_e53975);
        (assign41040_e53976, ((((0.5 * var_g_0_dn4) * assign41040_e53974) - (assign41040_e53972 * (var_xbct_dn4 / (2.0 * assign41040_e53974)))) / (assign41040_e53974 * assign41040_e53974)), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign41040_e53978;
        var_temp1_dn4 = assign41040_e53978_d_n4;
        var_temp1_dn6 = assign41040_e53978_d_n6;
        var_temp1_dn7 = assign41040_e53978_d_n7;
        var_temp1_dn8 = assign41040_e53978_d_n8;
        var_temp1_dn9 = assign41040_e53978_d_n9;
        var_temp1_rv = 0.0;

        let (assign41050_e53987, assign41050_e53987_d_n4, assign41050_e53987_d_n6, assign41050_e53987_d_n7, assign41050_e53987_d_n8, assign41050_e53987_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41050_e53983: f64 = (var_xbct).sqrt();
        let assign41050_e53984: f64 = (var_g_0 * assign41050_e53983);
        let assign41050_e53985: f64 = (var_xbct + assign41050_e53984);
        (assign41050_e53985, (var_xbct_dn4 + ((var_g_0_dn4 * assign41050_e53983) + (var_g_0 * (var_xbct_dn4 / (2.0 * assign41050_e53983))))), 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign41050_e53987;
        var_temp2_dn4 = assign41050_e53987_d_n4;
        var_temp2_dn6 = assign41050_e53987_d_n6;
        var_temp2_dn7 = assign41050_e53987_d_n7;
        var_temp2_dn8 = assign41050_e53987_d_n8;
        var_temp2_dn9 = assign41050_e53987_d_n9;
        var_temp2_rv = 0.0;

        let (assign41060_e54005, assign41060_e54005_d_n4, assign41060_e54005_d_n6, assign41060_e54005_d_n7, assign41060_e54005_d_n8, assign41060_e54005_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41060_e53991: f64 = (var_xgct - var_temp2);
        let assign41060_e53993: f64 = (assign41060_e53991 / var_temp1);
        let assign41060_e53996: f64 = (0.5 * var_xbct);
        let assign41060_e53997: f64 = (assign41060_e53993 + assign41060_e53996);
        let assign41060_e54000: f64 = (1.0 + var_ctb_i);
        let assign41060_e54002: f64 = (assign41060_e54000 * var_xsbstar);
        let assign41060_e54003: f64 = (assign41060_e53997 - assign41060_e54002);
        (assign41060_e54003, ((((((var_xgct_dn4 - var_temp2_dn4) * var_temp1) - (assign41060_e53991 * var_temp1_dn4)) / (var_temp1 * var_temp1)) + (0.5 * var_xbct_dn4)) - (assign41060_e54000 * var_xsbstar_dn4)), (((((var_xgct_dn6 - var_temp2_dn6) * var_temp1) - (assign41060_e53991 * var_temp1_dn6)) / (var_temp1 * var_temp1)) - (assign41060_e54000 * var_xsbstar_dn6)), (((((var_xgct_dn7 - var_temp2_dn7) * var_temp1) - (assign41060_e53991 * var_temp1_dn7)) / (var_temp1 * var_temp1)) - (assign41060_e54000 * var_xsbstar_dn7)), (((((var_xgct_dn8 - var_temp2_dn8) * var_temp1) - (assign41060_e53991 * var_temp1_dn8)) / (var_temp1 * var_temp1)) - (assign41060_e54000 * var_xsbstar_dn8)), (((((var_xgct_dn9 - var_temp2_dn9) * var_temp1) - (assign41060_e53991 * var_temp1_dn9)) / (var_temp1 * var_temp1)) - (assign41060_e54000 * var_xsbstar_dn9)),)
    } else {
        (var_xwict, var_xwict_dn4, var_xwict_dn6, var_xwict_dn7, var_xwict_dn8, var_xwict_dn9,)
    }
};
        var_xwict = assign41060_e54005;
        var_xwict_dn4 = assign41060_e54005_d_n4;
        var_xwict_dn6 = assign41060_e54005_d_n6;
        var_xwict_dn7 = assign41060_e54005_d_n7;
        var_xwict_dn8 = assign41060_e54005_d_n8;
        var_xwict_dn9 = assign41060_e54005_d_n9;
        var_xwict_rv = 0.0;

        let (assign41070_e54013, assign41070_e54013_d_n4,) = {
    if (var_guard1190 != 0.0) {
        let assign41070_e54009: f64 = (0.5 * var_xbct);
        let assign41070_e54011: f64 = (assign41070_e54009 + 2.0);
        (assign41070_e54011, (0.5 * var_xbct_dn4),)
    } else {
        (var_xctmax, var_xctmax_dn4,)
    }
};
        var_xctmax = assign41070_e54013;
        var_xctmax_dn4 = assign41070_e54013_d_n4;
        var_xctmax_rv = 0.0;

        let (assign41080_e54019, assign41080_e54019_d_n4, assign41080_e54019_d_n6, assign41080_e54019_d_n7, assign41080_e54019_d_n8, assign41080_e54019_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41080_e54017: f64 = (var_xbct + var_xsbstar);
        (assign41080_e54017, (var_xbct_dn4 + var_xsbstar_dn4), var_xsbstar_dn6, var_xsbstar_dn7, var_xsbstar_dn8, var_xsbstar_dn9,)
    } else {
        (var_xnct, var_xnct_dn4, var_xnct_dn6, var_xnct_dn7, var_xnct_dn8, var_xnct_dn9,)
    }
};
        var_xnct = assign41080_e54019;
        var_xnct_dn4 = assign41080_e54019_d_n4;
        var_xnct_dn6 = assign41080_e54019_d_n6;
        var_xnct_dn7 = assign41080_e54019_d_n7;
        var_xnct_dn8 = assign41080_e54019_d_n8;
        var_xnct_dn9 = assign41080_e54019_d_n9;
        var_xnct_rv = 0.0;

        let (assign41090_e54040, assign41090_e54040_d_n4, assign41090_e54040_d_n6, assign41090_e54040_d_n7, assign41090_e54040_d_n8, assign41090_e54040_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41090_e54023: f64 = (var_xgct - var_xnct);
        let assign41090_e54026: f64 = (var_xnct).sqrt();
        let assign41090_e54027: f64 = (var_g_0 * assign41090_e54026);
        let assign41090_e54028: f64 = (assign41090_e54023 - assign41090_e54027);
        let assign41090_e54032: f64 = (var_xbct / var_g_0);
        let assign41090_e54034: f64 = (var_xbct).sqrt();
        let assign41090_e54035: f64 = (assign41090_e54032 + assign41090_e54034);
        let assign41090_e54036: f64 = (assign41090_e54035).ln();
        let assign41090_e54037: f64 = (2.0 * assign41090_e54036);
        let assign41090_e54038: f64 = (assign41090_e54028 - assign41090_e54037);
        (assign41090_e54038, (((var_xgct_dn4 - var_xnct_dn4) - ((var_g_0_dn4 * assign41090_e54026) + (var_g_0 * (var_xnct_dn4 / (2.0 * assign41090_e54026))))) - (2.0 * (((((var_xbct_dn4 * var_g_0) - (var_xbct * var_g_0_dn4)) / (var_g_0 * var_g_0)) + (var_xbct_dn4 / (2.0 * assign41090_e54034))) / assign41090_e54035))), ((var_xgct_dn6 - var_xnct_dn6) - (var_g_0 * (var_xnct_dn6 / (2.0 * assign41090_e54026)))), ((var_xgct_dn7 - var_xnct_dn7) - (var_g_0 * (var_xnct_dn7 / (2.0 * assign41090_e54026)))), ((var_xgct_dn8 - var_xnct_dn8) - (var_g_0 * (var_xnct_dn8 / (2.0 * assign41090_e54026)))), ((var_xgct_dn9 - var_xnct_dn9) - (var_g_0 * (var_xnct_dn9 / (2.0 * assign41090_e54026)))),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign41090_e54040;
        var_temp1_dn4 = assign41090_e54040_d_n4;
        var_temp1_dn6 = assign41090_e54040_d_n6;
        var_temp1_dn7 = assign41090_e54040_d_n7;
        var_temp1_dn8 = assign41090_e54040_d_n8;
        var_temp1_dn9 = assign41090_e54040_d_n9;
        var_temp1_rv = 0.0;

        let (assign41100_e54048, assign41100_e54048_d_n4, assign41100_e54048_d_n6, assign41100_e54048_d_n7, assign41100_e54048_d_n8, assign41100_e54048_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41100_e54044: f64 = (2.0 * var_temp1);
        let assign41100_e54046: f64 = (assign41100_e54044 + var_xctmax);
        (assign41100_e54046, ((2.0 * var_temp1_dn4) + var_xctmax_dn4), (2.0 * var_temp1_dn6), (2.0 * var_temp1_dn7), (2.0 * var_temp1_dn8), (2.0 * var_temp1_dn9),)
    } else {
        (var_xmict, var_xmict_dn4, var_xmict_dn6, var_xmict_dn7, var_xmict_dn8, var_xmict_dn9,)
    }
};
        var_xmict = assign41100_e54048;
        var_xmict_dn4 = assign41100_e54048_d_n4;
        var_xmict_dn6 = assign41100_e54048_d_n6;
        var_xmict_dn7 = assign41100_e54048_d_n7;
        var_xmict_dn8 = assign41100_e54048_d_n8;
        var_xmict_dn9 = assign41100_e54048_d_n9;
        var_xmict_rv = 0.0;

        let (assign41110_e54067, assign41110_e54067_d_n4, assign41110_e54067_d_n6, assign41110_e54067_d_n7, assign41110_e54067_d_n8, assign41110_e54067_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41110_e54053: f64 = (var_xwict + var_xmict);
        let assign41110_e54056: f64 = (var_xwict - var_xmict);
        let assign41110_e54059: f64 = (var_xwict - var_xmict);
        let assign41110_e54060: f64 = (assign41110_e54056 * assign41110_e54059);
        let assign41110_e54062: f64 = (assign41110_e54060 + 20.0);
        let assign41110_e54063: f64 = (assign41110_e54062).sqrt();
        let assign41110_e54064: f64 = (assign41110_e54053 + assign41110_e54063);
        let assign41110_e54065: f64 = (0.5 * assign41110_e54064);
        (assign41110_e54065, (0.5 * ((var_xwict_dn4 + var_xmict_dn4) + ((((var_xwict_dn4 - var_xmict_dn4) * assign41110_e54059) + (assign41110_e54056 * (var_xwict_dn4 - var_xmict_dn4))) / (2.0 * assign41110_e54063)))), (0.5 * ((var_xwict_dn6 + var_xmict_dn6) + ((((var_xwict_dn6 - var_xmict_dn6) * assign41110_e54059) + (assign41110_e54056 * (var_xwict_dn6 - var_xmict_dn6))) / (2.0 * assign41110_e54063)))), (0.5 * ((var_xwict_dn7 + var_xmict_dn7) + ((((var_xwict_dn7 - var_xmict_dn7) * assign41110_e54059) + (assign41110_e54056 * (var_xwict_dn7 - var_xmict_dn7))) / (2.0 * assign41110_e54063)))), (0.5 * ((var_xwict_dn8 + var_xmict_dn8) + ((((var_xwict_dn8 - var_xmict_dn8) * assign41110_e54059) + (assign41110_e54056 * (var_xwict_dn8 - var_xmict_dn8))) / (2.0 * assign41110_e54063)))), (0.5 * ((var_xwict_dn9 + var_xmict_dn9) + ((((var_xwict_dn9 - var_xmict_dn9) * assign41110_e54059) + (assign41110_e54056 * (var_xwict_dn9 - var_xmict_dn9))) / (2.0 * assign41110_e54063)))),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign41110_e54067;
        var_temp1_dn4 = assign41110_e54067_d_n4;
        var_temp1_dn6 = assign41110_e54067_d_n6;
        var_temp1_dn7 = assign41110_e54067_d_n7;
        var_temp1_dn8 = assign41110_e54067_d_n8;
        var_temp1_dn9 = assign41110_e54067_d_n9;
        var_temp1_rv = 0.0;

        let (assign41120_e54077, assign41120_e54077_d_n4, assign41120_e54077_d_n6, assign41120_e54077_d_n7, assign41120_e54077_d_n8, assign41120_e54077_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41120_e54072: f64 = (var_xgct - var_xsbstar);
        let assign41120_e54073: f64 = (2.0 * assign41120_e54072);
        let assign41120_e54075: f64 = (assign41120_e54073 - var_xctmax);
        (assign41120_e54075, ((2.0 * (var_xgct_dn4 - var_xsbstar_dn4)) - var_xctmax_dn4), (2.0 * (var_xgct_dn6 - var_xsbstar_dn6)), (2.0 * (var_xgct_dn7 - var_xsbstar_dn7)), (2.0 * (var_xgct_dn8 - var_xsbstar_dn8)), (2.0 * (var_xgct_dn9 - var_xsbstar_dn9)),)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign41120_e54077;
        var_temp2_dn4 = assign41120_e54077_d_n4;
        var_temp2_dn6 = assign41120_e54077_d_n6;
        var_temp2_dn7 = assign41120_e54077_d_n7;
        var_temp2_dn8 = assign41120_e54077_d_n8;
        var_temp2_dn9 = assign41120_e54077_d_n9;
        var_temp2_rv = 0.0;

        let (assign41130_e54096, assign41130_e54096_d_n4, assign41130_e54096_d_n6, assign41130_e54096_d_n7, assign41130_e54096_d_n8, assign41130_e54096_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41130_e54082: f64 = (var_temp1 + var_temp2);
        let assign41130_e54085: f64 = (var_temp1 - var_temp2);
        let assign41130_e54088: f64 = (var_temp1 - var_temp2);
        let assign41130_e54089: f64 = (assign41130_e54085 * assign41130_e54088);
        let assign41130_e54091: f64 = (assign41130_e54089 + 20.0);
        let assign41130_e54092: f64 = (assign41130_e54091).sqrt();
        let assign41130_e54093: f64 = (assign41130_e54082 - assign41130_e54092);
        let assign41130_e54094: f64 = (0.5 * assign41130_e54093);
        (assign41130_e54094, (0.5 * ((var_temp1_dn4 + var_temp2_dn4) - ((((var_temp1_dn4 - var_temp2_dn4) * assign41130_e54088) + (assign41130_e54085 * (var_temp1_dn4 - var_temp2_dn4))) / (2.0 * assign41130_e54092)))), (0.5 * ((var_temp1_dn6 + var_temp2_dn6) - ((((var_temp1_dn6 - var_temp2_dn6) * assign41130_e54088) + (assign41130_e54085 * (var_temp1_dn6 - var_temp2_dn6))) / (2.0 * assign41130_e54092)))), (0.5 * ((var_temp1_dn7 + var_temp2_dn7) - ((((var_temp1_dn7 - var_temp2_dn7) * assign41130_e54088) + (assign41130_e54085 * (var_temp1_dn7 - var_temp2_dn7))) / (2.0 * assign41130_e54092)))), (0.5 * ((var_temp1_dn8 + var_temp2_dn8) - ((((var_temp1_dn8 - var_temp2_dn8) * assign41130_e54088) + (assign41130_e54085 * (var_temp1_dn8 - var_temp2_dn8))) / (2.0 * assign41130_e54092)))), (0.5 * ((var_temp1_dn9 + var_temp2_dn9) - ((((var_temp1_dn9 - var_temp2_dn9) * assign41130_e54088) + (assign41130_e54085 * (var_temp1_dn9 - var_temp2_dn9))) / (2.0 * assign41130_e54092)))),)
    } else {
        (var_xsubct, var_xsubct_dn4, var_xsubct_dn6, var_xsubct_dn7, var_xsubct_dn8, var_xsubct_dn9,)
    }
};
        var_xsubct = assign41130_e54096;
        var_xsubct_dn4 = assign41130_e54096_d_n4;
        var_xsubct_dn6 = assign41130_e54096_d_n6;
        var_xsubct_dn7 = assign41130_e54096_d_n7;
        var_xsubct_dn8 = assign41130_e54096_d_n8;
        var_xsubct_dn9 = assign41130_e54096_d_n9;
        var_xsubct_rv = 0.0;

        let (assign41140_e54115, assign41140_e54115_d_n4, assign41140_e54115_d_n6, assign41140_e54115_d_n7, assign41140_e54115_d_n8, assign41140_e54115_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41140_e54101: f64 = (var_xsubct + var_xctmax);
        let assign41140_e54104: f64 = (var_xsubct - var_xctmax);
        let assign41140_e54107: f64 = (var_xsubct - var_xctmax);
        let assign41140_e54108: f64 = (assign41140_e54104 * assign41140_e54107);
        let assign41140_e54110: f64 = (assign41140_e54108 + 5.0);
        let assign41140_e54111: f64 = (assign41140_e54110).sqrt();
        let assign41140_e54112: f64 = (assign41140_e54101 - assign41140_e54111);
        let assign41140_e54113: f64 = (0.5 * assign41140_e54112);
        (assign41140_e54113, (0.5 * ((var_xsubct_dn4 + var_xctmax_dn4) - ((((var_xsubct_dn4 - var_xctmax_dn4) * assign41140_e54107) + (assign41140_e54104 * (var_xsubct_dn4 - var_xctmax_dn4))) / (2.0 * assign41140_e54111)))), (0.5 * (var_xsubct_dn6 - (((var_xsubct_dn6 * assign41140_e54107) + (assign41140_e54104 * var_xsubct_dn6)) / (2.0 * assign41140_e54111)))), (0.5 * (var_xsubct_dn7 - (((var_xsubct_dn7 * assign41140_e54107) + (assign41140_e54104 * var_xsubct_dn7)) / (2.0 * assign41140_e54111)))), (0.5 * (var_xsubct_dn8 - (((var_xsubct_dn8 * assign41140_e54107) + (assign41140_e54104 * var_xsubct_dn8)) / (2.0 * assign41140_e54111)))), (0.5 * (var_xsubct_dn9 - (((var_xsubct_dn9 * assign41140_e54107) + (assign41140_e54104 * var_xsubct_dn9)) / (2.0 * assign41140_e54111)))),)
    } else {
        (var_temp1, var_temp1_dn4, var_temp1_dn6, var_temp1_dn7, var_temp1_dn8, var_temp1_dn9,)
    }
};
        var_temp1 = assign41140_e54115;
        var_temp1_dn4 = assign41140_e54115_d_n4;
        var_temp1_dn6 = assign41140_e54115_d_n6;
        var_temp1_dn7 = assign41140_e54115_d_n7;
        var_temp1_dn8 = assign41140_e54115_d_n8;
        var_temp1_dn9 = assign41140_e54115_d_n9;
        var_temp1_rv = 0.0;

        let (assign41150_e54137, assign41150_e54137_d_n4, assign41150_e54137_d_n6, assign41150_e54137_d_n7, assign41150_e54137_d_n8, assign41150_e54137_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41150_e54120: f64 = (-var_xctmax);
        let assign41150_e54121: f64 = (var_temp1 + assign41150_e54120);
        let assign41150_e54124: f64 = (-var_xctmax);
        let assign41150_e54125: f64 = (var_temp1 - assign41150_e54124);
        let assign41150_e54128: f64 = (-var_xctmax);
        let assign41150_e54129: f64 = (var_temp1 - assign41150_e54128);
        let assign41150_e54130: f64 = (assign41150_e54125 * assign41150_e54129);
        let assign41150_e54132: f64 = (assign41150_e54130 + 20.0);
        let assign41150_e54133: f64 = (assign41150_e54132).sqrt();
        let assign41150_e54134: f64 = (assign41150_e54121 + assign41150_e54133);
        let assign41150_e54135: f64 = (0.5 * assign41150_e54134);
        (assign41150_e54135, (0.5 * ((var_temp1_dn4 + (-var_xctmax_dn4)) + ((((var_temp1_dn4 - (-var_xctmax_dn4)) * assign41150_e54129) + (assign41150_e54125 * (var_temp1_dn4 - (-var_xctmax_dn4)))) / (2.0 * assign41150_e54133)))), (0.5 * (var_temp1_dn6 + (((var_temp1_dn6 * assign41150_e54129) + (assign41150_e54125 * var_temp1_dn6)) / (2.0 * assign41150_e54133)))), (0.5 * (var_temp1_dn7 + (((var_temp1_dn7 * assign41150_e54129) + (assign41150_e54125 * var_temp1_dn7)) / (2.0 * assign41150_e54133)))), (0.5 * (var_temp1_dn8 + (((var_temp1_dn8 * assign41150_e54129) + (assign41150_e54125 * var_temp1_dn8)) / (2.0 * assign41150_e54133)))), (0.5 * (var_temp1_dn9 + (((var_temp1_dn9 * assign41150_e54129) + (assign41150_e54125 * var_temp1_dn9)) / (2.0 * assign41150_e54133)))),)
    } else {
        (var_xct, var_xct_dn4, var_xct_dn6, var_xct_dn7, var_xct_dn8, var_xct_dn9,)
    }
};
        var_xct = assign41150_e54137;
        var_xct_dn4 = assign41150_e54137_d_n4;
        var_xct_dn6 = assign41150_e54137_d_n6;
        var_xct_dn7 = assign41150_e54137_d_n7;
        var_xct_dn8 = assign41150_e54137_d_n8;
        var_xct_dn9 = assign41150_e54137_d_n9;
        var_xct_rv = 0.0;

        let (assign41160_e54147, assign41160_e54147_d_n4, assign41160_e54147_d_n6, assign41160_e54147_d_n7, assign41160_e54147_d_n8, assign41160_e54147_d_n9,) = {
    if (var_guard1190 != 0.0) {
        let assign41160_e54142: f64 = (var_xct / var_xctmax);
        let assign41160_e54144: f64 = (assign41160_e54142 + 1.0);
        let assign41160_e54145: f64 = (var_ctg_t * assign41160_e54144);
        (assign41160_e54145, ((var_ctg_t_dn4 * assign41160_e54144) + (var_ctg_t * (((var_xct_dn4 * var_xctmax) - (var_xct * var_xctmax_dn4)) / (var_xctmax * var_xctmax)))), (var_ctg_t * (var_xct_dn6 / var_xctmax)), (var_ctg_t * (var_xct_dn7 / var_xctmax)), (var_ctg_t * (var_xct_dn8 / var_xctmax)), (var_ctg_t * (var_xct_dn9 / var_xctmax)),)
    } else {
        (var_temp2, var_temp2_dn4, var_temp2_dn6, var_temp2_dn7, var_temp2_dn8, var_temp2_dn9,)
    }
};
        var_temp2 = assign41160_e54147;
        var_temp2_dn4 = assign41160_e54147_d_n4;
        var_temp2_dn6 = assign41160_e54147_d_n6;
        var_temp2_dn7 = assign41160_e54147_d_n7;
        var_temp2_dn8 = assign41160_e54147_d_n8;
        var_temp2_dn9 = assign41160_e54147_d_n9;
        var_temp2_rv = 0.0;

        let assign41170_e54150: f64 = (-230.25850929940458);
        let assign41170_e54151: f64 = if var_temp2 > assign41170_e54150 { 1.0 } else { 0.0 };
        var_guard1191 = assign41170_e54151;
        var_guard1191_rv = 0.0;

        let (assign41180_e54158, assign41180_e54158_d_n4, assign41180_e54158_d_n6, assign41180_e54158_d_n7, assign41180_e54158_d_n8, assign41180_e54158_d_n9,) = {
    if ((var_guard1190 != 0.0) && (var_guard1191 != 0.0)) {
        let assign41180_e54156: f64 = (var_temp2).exp();
        (assign41180_e54156, (assign41180_e54156 * var_temp2_dn4), (assign41180_e54156 * var_temp2_dn6), (assign41180_e54156 * var_temp2_dn7), (assign41180_e54156 * var_temp2_dn8), (assign41180_e54156 * var_temp2_dn9),)
    } else {
        (var_dctg, var_dctg_dn4, var_dctg_dn6, var_dctg_dn7, var_dctg_dn8, var_dctg_dn9,)
    }
};
        var_dctg = assign41180_e54158;
        var_dctg_dn4 = assign41180_e54158_d_n4;
        var_dctg_dn6 = assign41180_e54158_d_n6;
        var_dctg_dn7 = assign41180_e54158_d_n7;
        var_dctg_dn8 = assign41180_e54158_d_n8;
        var_dctg_dn9 = assign41180_e54158_d_n9;
        var_dctg_rv = 0.0;

        let (assign41190_e54190, assign41190_e54190_d_n4, assign41190_e54190_d_n6, assign41190_e54190_d_n7, assign41190_e54190_d_n8, assign41190_e54190_d_n9,) = {
    if ((var_guard1190 != 0.0) && (var_guard1191 == 0.0)) {
        let assign41190_e54166: f64 = (-230.25850929940458);
        let assign41190_e54168: f64 = (assign41190_e54166 - var_temp2);
        let assign41190_e54172: f64 = (-230.25850929940458);
        let assign41190_e54174: f64 = (assign41190_e54172 - var_temp2);
        let assign41190_e54177: f64 = (-230.25850929940458);
        let assign41190_e54179: f64 = (assign41190_e54177 - var_temp2);
        let assign41190_e54181: f64 = (assign41190_e54179 * 0.3333333333333333);
        let assign41190_e54182: f64 = (1.0 + assign41190_e54181);
        let assign41190_e54183: f64 = (assign41190_e54174 * assign41190_e54182);
        let assign41190_e54184: f64 = (0.5 * assign41190_e54183);
        let assign41190_e54185: f64 = (1.0 + assign41190_e54184);
        let assign41190_e54186: f64 = (assign41190_e54168 * assign41190_e54185);
        let assign41190_e54187: f64 = (1.0 + assign41190_e54186);
        let assign41190_e54188: f64 = (1e-100 / assign41190_e54187);
        (assign41190_e54188, (-((1e-100 * (((-var_temp2_dn4) * assign41190_e54185) + (assign41190_e54168 * (0.5 * (((-var_temp2_dn4) * assign41190_e54182) + (assign41190_e54174 * ((-var_temp2_dn4) * 0.3333333333333333))))))) / (assign41190_e54187 * assign41190_e54187))), (-((1e-100 * (((-var_temp2_dn6) * assign41190_e54185) + (assign41190_e54168 * (0.5 * (((-var_temp2_dn6) * assign41190_e54182) + (assign41190_e54174 * ((-var_temp2_dn6) * 0.3333333333333333))))))) / (assign41190_e54187 * assign41190_e54187))), (-((1e-100 * (((-var_temp2_dn7) * assign41190_e54185) + (assign41190_e54168 * (0.5 * (((-var_temp2_dn7) * assign41190_e54182) + (assign41190_e54174 * ((-var_temp2_dn7) * 0.3333333333333333))))))) / (assign41190_e54187 * assign41190_e54187))), (-((1e-100 * (((-var_temp2_dn8) * assign41190_e54185) + (assign41190_e54168 * (0.5 * (((-var_temp2_dn8) * assign41190_e54182) + (assign41190_e54174 * ((-var_temp2_dn8) * 0.3333333333333333))))))) / (assign41190_e54187 * assign41190_e54187))), (-((1e-100 * (((-var_temp2_dn9) * assign41190_e54185) + (assign41190_e54168 * (0.5 * (((-var_temp2_dn9) * assign41190_e54182) + (assign41190_e54174 * ((-var_temp2_dn9) * 0.3333333333333333))))))) / (assign41190_e54187 * assign41190_e54187))),)
    } else {
        (var_dctg, var_dctg_dn4, var_dctg_dn6, var_dctg_dn7, var_dctg_dn8, var_dctg_dn9,)
    }
};
        var_dctg = assign41190_e54190;
        var_dctg_dn4 = assign41190_e54190_d_n4;
        var_dctg_dn6 = assign41190_e54190_d_n6;
        var_dctg_dn7 = assign41190_e54190_d_n7;
        var_dctg_dn8 = assign41190_e54190_d_n8;
        var_dctg_dn9 = assign41190_e54190_d_n9;
        var_dctg_rv = 0.0;

        let assign41200_e54194: f64 = (var_ct_t * var_dctg);
        let assign41200_e54195: f64 = (1.0 + assign41200_e54194);
        var_ct_fact = assign41200_e54195;
        var_ct_fact_dn4 = ((var_ct_t_dn4 * var_dctg) + (var_ct_t * var_dctg_dn4));
        var_ct_fact_dn6 = (var_ct_t * var_dctg_dn6);
        var_ct_fact_dn7 = (var_ct_t * var_dctg_dn7);
        var_ct_fact_dn8 = (var_ct_t * var_dctg_dn8);
        var_ct_fact_dn9 = (var_ct_t * var_dctg_dn9);
        var_ct_fact_rv = 0.0;

        let assign41210_e54198: f64 = (var_phit * var_ct_fact);
        var_phitct = assign41210_e54198;
        var_phitct_dn4 = ((var_phit_dn4 * var_ct_fact) + (var_phit * var_ct_fact_dn4));
        var_phitct_dn6 = (var_phit * var_ct_fact_dn6);
        var_phitct_dn7 = (var_phit * var_ct_fact_dn7);
        var_phitct_dn8 = (var_phit * var_ct_fact_dn8);
        var_phitct_dn9 = (var_phit * var_ct_fact_dn9);
        var_phitct_rv = 0.0;

        let assign41220_e54203: f64 = (var_psced_i * var_vdsx);
        let assign41220_e54204: f64 = (1.0 + assign41220_e54203);
        let assign41220_e54205: f64 = (var_psce_i * assign41220_e54204);
        let assign41220_e54209: f64 = (var_psceb_i * var_vsbx);
        let assign41220_e54210: f64 = (1.0 + assign41220_e54209);
        let assign41220_e54211: f64 = (assign41220_e54205 * assign41220_e54210);
        var_dphit1 = assign41220_e54211;
        var_dphit1_dn4 = (assign41220_e54205 * (var_psceb_i * var_vsbx_dn4));
        var_dphit1_dn6 = (assign41220_e54205 * (var_psceb_i * var_vsbx_dn6));
        var_dphit1_dn7 = (((var_psce_i * (var_psced_i * var_vdsx_dn7)) * assign41220_e54210) + (assign41220_e54205 * (var_psceb_i * var_vsbx_dn7)));
        var_dphit1_dn8 = (((var_psce_i * (var_psced_i * var_vdsx_dn8)) * assign41220_e54210) + (assign41220_e54205 * (var_psceb_i * var_vsbx_dn8)));
        var_dphit1_dn9 = (assign41220_e54205 * (var_psceb_i * var_vsbx_dn9));
        var_dphit1_rv = 0.0;

        let assign41230_e54215: f64 = (1.0 + var_dphit1);
        let assign41230_e54216: f64 = (var_phitct * assign41230_e54215);
        var_phit1 = assign41230_e54216;
        var_phit1_dn4 = ((var_phitct_dn4 * assign41230_e54215) + (var_phitct * var_dphit1_dn4));
        var_phit1_dn6 = ((var_phitct_dn6 * assign41230_e54215) + (var_phitct * var_dphit1_dn6));
        var_phit1_dn7 = ((var_phitct_dn7 * assign41230_e54215) + (var_phitct * var_dphit1_dn7));
        var_phit1_dn8 = ((var_phitct_dn8 * assign41230_e54215) + (var_phitct * var_dphit1_dn8));
        var_phit1_dn9 = ((var_phitct_dn9 * assign41230_e54215) + (var_phitct * var_dphit1_dn9));
        var_phit1_rv = 0.0;

        let assign41240_e54219: f64 = (1.0 / var_phit1);
        var_inv_phit1 = assign41240_e54219;
        var_inv_phit1_dn4 = (-(var_phit1_dn4 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn6 = (-(var_phit1_dn6 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn7 = (-(var_phit1_dn7 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn8 = (-(var_phit1_dn8 / (var_phit1 * var_phit1)));
        var_inv_phit1_dn9 = (-(var_phit1_dn9 / (var_phit1 * var_phit1)));
        var_inv_phit1_rv = 0.0;

        let assign41250_e54223: f64 = (var_phit * var_inv_phit1);
        let assign41250_e54224: f64 = (assign41250_e54223).sqrt();
        let assign41250_e54225: f64 = (var_g_0 * assign41250_e54224);
        var_gf = assign41250_e54225;
        var_gf_dn4 = ((var_g_0_dn4 * assign41250_e54224) + (var_g_0 * (((var_phit_dn4 * var_inv_phit1) + (var_phit * var_inv_phit1_dn4)) / (2.0 * assign41250_e54224))));
        var_gf_dn6 = (var_g_0 * ((var_phit * var_inv_phit1_dn6) / (2.0 * assign41250_e54224)));
        var_gf_dn7 = (var_g_0 * ((var_phit * var_inv_phit1_dn7) / (2.0 * assign41250_e54224)));
        var_gf_dn8 = (var_g_0 * ((var_phit * var_inv_phit1_dn8) / (2.0 * assign41250_e54224)));
        var_gf_dn9 = (var_g_0 * ((var_phit * var_inv_phit1_dn9) / (2.0 * assign41250_e54224)));
        var_gf_rv = 0.0;

        let assign41260_e54228: f64 = (var_gf * var_gf);
        var_gf2 = assign41260_e54228;
        var_gf2_dn4 = ((var_gf_dn4 * var_gf) + (var_gf * var_gf_dn4));
        var_gf2_dn6 = ((var_gf_dn6 * var_gf) + (var_gf * var_gf_dn6));
        var_gf2_dn7 = ((var_gf_dn7 * var_gf) + (var_gf * var_gf_dn7));
        var_gf2_dn8 = ((var_gf_dn8 * var_gf) + (var_gf * var_gf_dn8));
        var_gf2_dn9 = ((var_gf_dn9 * var_gf) + (var_gf * var_gf_dn9));
        var_gf2_rv = 0.0;

        let assign41270_e54231: f64 = (1.0 / var_gf2);
        var_inv_gf2 = assign41270_e54231;
        var_inv_gf2_dn4 = (-(var_gf2_dn4 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn6 = (-(var_gf2_dn6 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn7 = (-(var_gf2_dn7 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn8 = (-(var_gf2_dn8 / (var_gf2 * var_gf2)));
        var_inv_gf2_dn9 = (-(var_gf2_dn9 / (var_gf2 * var_gf2)));
        var_inv_gf2_rv = 0.0;

        let assign41280_e54234: f64 = (var_vsbstar * var_inv_phit1);
        var_ux = assign41280_e54234;
        var_ux_dn4 = ((var_vsbstar_dn4 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn4));
        var_ux_dn6 = ((var_vsbstar_dn6 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn6));
        var_ux_dn7 = ((var_vsbstar_dn7 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn7));
        var_ux_dn8 = ((var_vsbstar_dn8 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn8));
        var_ux_dn9 = ((var_vsbstar_dn9 * var_inv_phit1) + (var_vsbstar * var_inv_phit1_dn9));
        var_ux_rv = 0.0;

        let assign41290_e54237: f64 = (var_vgb1 * var_inv_phit1);
        var_xg = assign41290_e54237;
        var_xg_dn4 = ((var_vgb1_dn4 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn4));
        var_xg_dn6 = ((var_vgb1_dn6 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn6));
        var_xg_dn7 = ((var_vgb1_dn7 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn7));
        var_xg_dn8 = ((var_vgb1_dn8 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn8));
        var_xg_dn9 = ((var_vgb1_dn9 * var_inv_phit1) + (var_vgb1 * var_inv_phit1_dn9));
        var_xg_rv = 0.0;

        let assign41300_e54240: f64 = (2.0 * var_vdsx);
        let assign41300_e54245: f64 = (var_cfd_i * var_vdsx);
        let assign41300_e54246: f64 = (1.0 + assign41300_e54245);
        let assign41300_e54247: f64 = (assign41300_e54246).sqrt();
        let assign41300_e54248: f64 = (1.0 + assign41300_e54247);
        let assign41300_e54249: f64 = (assign41300_e54240 / assign41300_e54248);
        var_vdsp = assign41300_e54249;
        var_vdsp_dn7 = ((((2.0 * var_vdsx_dn7) * assign41300_e54248) - (assign41300_e54240 * ((var_cfd_i * var_vdsx_dn7) / (2.0 * assign41300_e54247)))) / (assign41300_e54248 * assign41300_e54248));
        var_vdsp_dn8 = ((((2.0 * var_vdsx_dn8) * assign41300_e54248) - (assign41300_e54240 * ((var_cfd_i * var_vdsx_dn8) / (2.0 * assign41300_e54247)))) / (assign41300_e54248 * assign41300_e54248));
        var_vdsp_rv = 0.0;

        let assign41310_e54252: f64 = (var_cf_i * var_vdsp);
        let assign41310_e54256: f64 = (var_cfb_i * var_vsbx);
        let assign41310_e54257: f64 = (1.0 + assign41310_e54256);
        let assign41310_e54258: f64 = (assign41310_e54252 * assign41310_e54257);
        var_delphib = assign41310_e54258;
        var_delphib_dn4 = (assign41310_e54252 * (var_cfb_i * var_vsbx_dn4));
        var_delphib_dn6 = (assign41310_e54252 * (var_cfb_i * var_vsbx_dn6));
        var_delphib_dn7 = (((var_cf_i * var_vdsp_dn7) * assign41310_e54257) + (assign41310_e54252 * (var_cfb_i * var_vsbx_dn7)));
        var_delphib_dn8 = (((var_cf_i * var_vdsp_dn8) * assign41310_e54257) + (assign41310_e54252 * (var_cfb_i * var_vsbx_dn8)));
        var_delphib_dn9 = (assign41310_e54252 * (var_cfb_i * var_vsbx_dn9));
        var_delphib_rv = 0.0;

        let assign41320_e54261: f64 = (var_phib * var_inv_phit1);
        var_xb = assign41320_e54261;
        var_xb_dn4 = ((var_phib_dn4 * var_inv_phit1) + (var_phib * var_inv_phit1_dn4));
        var_xb_dn6 = (var_phib * var_inv_phit1_dn6);
        var_xb_dn7 = (var_phib * var_inv_phit1_dn7);
        var_xb_dn8 = (var_phib * var_inv_phit1_dn8);
        var_xb_dn9 = (var_phib * var_inv_phit1_dn9);
        var_xb_rv = 0.0;

        let assign41330_e54264: f64 = (var_v_xb * var_v_xb);
        let assign41330_e54266: f64 = (assign41330_e54264 + var_aphi);
        let assign41330_e54267: f64 = (assign41330_e54266).sqrt();
        var_temp1 = assign41330_e54267;
        var_temp1_dn4 = ((((var_v_xb_dn4 * var_v_xb) + (var_v_xb * var_v_xb_dn4)) + var_aphi_dn4) / (2.0 * assign41330_e54267));
        var_temp1_dn6 = 0.0;
        var_temp1_dn7 = (((var_v_xb_dn7 * var_v_xb) + (var_v_xb * var_v_xb_dn7)) / (2.0 * assign41330_e54267));
        var_temp1_dn8 = (((var_v_xb_dn8 * var_v_xb) + (var_v_xb * var_v_xb_dn8)) / (2.0 * assign41330_e54267));
        var_temp1_dn9 = (((var_v_xb_dn9 * var_v_xb) + (var_v_xb * var_v_xb_dn9)) / (2.0 * assign41330_e54267));
        var_temp1_rv = 0.0;

        let assign41340_e54270: f64 = (var_v_xb - var_delphib);
        let assign41340_e54273: f64 = (var_v_xb - var_delphib);
        let assign41340_e54274: f64 = (assign41340_e54270 * assign41340_e54273);
        let assign41340_e54276: f64 = (assign41340_e54274 + var_aphi);
        let assign41340_e54277: f64 = (assign41340_e54276).sqrt();
        var_temp2 = assign41340_e54277;
        var_temp2_dn4 = (((((var_v_xb_dn4 - var_delphib_dn4) * assign41340_e54273) + (assign41340_e54270 * (var_v_xb_dn4 - var_delphib_dn4))) + var_aphi_dn4) / (2.0 * assign41340_e54277));
        var_temp2_dn6 = ((((-var_delphib_dn6) * assign41340_e54273) + (assign41340_e54270 * (-var_delphib_dn6))) / (2.0 * assign41340_e54277));
        var_temp2_dn7 = ((((var_v_xb_dn7 - var_delphib_dn7) * assign41340_e54273) + (assign41340_e54270 * (var_v_xb_dn7 - var_delphib_dn7))) / (2.0 * assign41340_e54277));
        var_temp2_dn8 = ((((var_v_xb_dn8 - var_delphib_dn8) * assign41340_e54273) + (assign41340_e54270 * (var_v_xb_dn8 - var_delphib_dn8))) / (2.0 * assign41340_e54277));
        var_temp2_dn9 = ((((var_v_xb_dn9 - var_delphib_dn9) * assign41340_e54273) + (assign41340_e54270 * (var_v_xb_dn9 - var_delphib_dn9))) / (2.0 * assign41340_e54277));
        var_temp2_rv = 0.0;

        let assign41350_e54280: f64 = (0.5 * var_inv_phit1);
        let assign41350_e54283: f64 = (var_delphib + var_temp1);
        let assign41350_e54285: f64 = (assign41350_e54283 - var_temp2);
        let assign41350_e54286: f64 = (assign41350_e54280 * assign41350_e54285);
        var_delxb = assign41350_e54286;
        var_delxb_dn4 = (((0.5 * var_inv_phit1_dn4) * assign41350_e54285) + (assign41350_e54280 * ((var_delphib_dn4 + var_temp1_dn4) - var_temp2_dn4)));
        var_delxb_dn6 = (((0.5 * var_inv_phit1_dn6) * assign41350_e54285) + (assign41350_e54280 * ((var_delphib_dn6 + var_temp1_dn6) - var_temp2_dn6)));
        var_delxb_dn7 = (((0.5 * var_inv_phit1_dn7) * assign41350_e54285) + (assign41350_e54280 * ((var_delphib_dn7 + var_temp1_dn7) - var_temp2_dn7)));
        var_delxb_dn8 = (((0.5 * var_inv_phit1_dn8) * assign41350_e54285) + (assign41350_e54280 * ((var_delphib_dn8 + var_temp1_dn8) - var_temp2_dn8)));
        var_delxb_dn9 = (((0.5 * var_inv_phit1_dn9) * assign41350_e54285) + (assign41350_e54280 * ((var_delphib_dn9 + var_temp1_dn9) - var_temp2_dn9)));
        var_delxb_rv = 0.0;

        let assign41360_e54289: f64 = (var_xb + var_ux);
        var_xno_s = assign41360_e54289;
        var_xno_s_dn4 = (var_xb_dn4 + var_ux_dn4);
        var_xno_s_dn6 = (var_xb_dn6 + var_ux_dn6);
        var_xno_s_dn7 = (var_xb_dn7 + var_ux_dn7);
        var_xno_s_dn8 = (var_xb_dn8 + var_ux_dn8);
        var_xno_s_dn9 = (var_xb_dn9 + var_ux_dn9);
        var_xno_s_rv = 0.0;

        let assign41370_e54292: f64 = (var_xno_s - var_delxb);
        var_xn_s = assign41370_e54292;
        var_xn_s_dn4 = (var_xno_s_dn4 - var_delxb_dn4);
        var_xn_s_dn6 = (var_xno_s_dn6 - var_delxb_dn6);
        var_xn_s_dn7 = (var_xno_s_dn7 - var_delxb_dn7);
        var_xn_s_dn8 = (var_xno_s_dn8 - var_delxb_dn8);
        var_xn_s_dn9 = (var_xno_s_dn9 - var_delxb_dn9);
        var_xn_s_rv = 0.0;

        *var_ct_fact_slot = var_ct_fact;
        *var_ct_fact_dn4_slot = var_ct_fact_dn4;
        *var_ct_fact_dn6_slot = var_ct_fact_dn6;
        *var_ct_fact_dn7_slot = var_ct_fact_dn7;
        *var_ct_fact_dn8_slot = var_ct_fact_dn8;
        *var_ct_fact_dn9_slot = var_ct_fact_dn9;
        *var_ct_fact_rv_slot = var_ct_fact_rv;
        *var_dctg_slot = var_dctg;
        *var_dctg_dn4_slot = var_dctg_dn4;
        *var_dctg_dn6_slot = var_dctg_dn6;
        *var_dctg_dn7_slot = var_dctg_dn7;
        *var_dctg_dn8_slot = var_dctg_dn8;
        *var_dctg_dn9_slot = var_dctg_dn9;
        *var_dctg_rv_slot = var_dctg_rv;
        *var_delphib_slot = var_delphib;
        *var_delphib_dn4_slot = var_delphib_dn4;
        *var_delphib_dn6_slot = var_delphib_dn6;
        *var_delphib_dn7_slot = var_delphib_dn7;
        *var_delphib_dn8_slot = var_delphib_dn8;
        *var_delphib_dn9_slot = var_delphib_dn9;
        *var_delphib_rv_slot = var_delphib_rv;
        *var_delxb_slot = var_delxb;
        *var_delxb_dn4_slot = var_delxb_dn4;
        *var_delxb_dn6_slot = var_delxb_dn6;
        *var_delxb_dn7_slot = var_delxb_dn7;
        *var_delxb_dn8_slot = var_delxb_dn8;
        *var_delxb_dn9_slot = var_delxb_dn9;
        *var_delxb_rv_slot = var_delxb_rv;
        *var_dphit1_slot = var_dphit1;
        *var_dphit1_dn4_slot = var_dphit1_dn4;
        *var_dphit1_dn6_slot = var_dphit1_dn6;
        *var_dphit1_dn7_slot = var_dphit1_dn7;
        *var_dphit1_dn8_slot = var_dphit1_dn8;
        *var_dphit1_dn9_slot = var_dphit1_dn9;
        *var_dphit1_rv_slot = var_dphit1_rv;
        *var_gf_slot = var_gf;
        *var_gf2_slot = var_gf2;
        *var_gf2_dn4_slot = var_gf2_dn4;
        *var_gf2_dn6_slot = var_gf2_dn6;
        *var_gf2_dn7_slot = var_gf2_dn7;
        *var_gf2_dn8_slot = var_gf2_dn8;
        *var_gf2_dn9_slot = var_gf2_dn9;
        *var_gf2_rv_slot = var_gf2_rv;
        *var_gf_dn4_slot = var_gf_dn4;
        *var_gf_dn6_slot = var_gf_dn6;
        *var_gf_dn7_slot = var_gf_dn7;
        *var_gf_dn8_slot = var_gf_dn8;
        *var_gf_dn9_slot = var_gf_dn9;
        *var_gf_rv_slot = var_gf_rv;
        *var_guard1191_slot = var_guard1191;
        *var_guard1191_rv_slot = var_guard1191_rv;
        *var_inv_gf2_slot = var_inv_gf2;
        *var_inv_gf2_dn4_slot = var_inv_gf2_dn4;
        *var_inv_gf2_dn6_slot = var_inv_gf2_dn6;
        *var_inv_gf2_dn7_slot = var_inv_gf2_dn7;
        *var_inv_gf2_dn8_slot = var_inv_gf2_dn8;
        *var_inv_gf2_dn9_slot = var_inv_gf2_dn9;
        *var_inv_gf2_rv_slot = var_inv_gf2_rv;
        *var_inv_phit1_slot = var_inv_phit1;
        *var_inv_phit1_dn4_slot = var_inv_phit1_dn4;
        *var_inv_phit1_dn6_slot = var_inv_phit1_dn6;
        *var_inv_phit1_dn7_slot = var_inv_phit1_dn7;
        *var_inv_phit1_dn8_slot = var_inv_phit1_dn8;
        *var_inv_phit1_dn9_slot = var_inv_phit1_dn9;
        *var_inv_phit1_rv_slot = var_inv_phit1_rv;
        *var_phit1_slot = var_phit1;
        *var_phit1_dn4_slot = var_phit1_dn4;
        *var_phit1_dn6_slot = var_phit1_dn6;
        *var_phit1_dn7_slot = var_phit1_dn7;
        *var_phit1_dn8_slot = var_phit1_dn8;
        *var_phit1_dn9_slot = var_phit1_dn9;
        *var_phit1_rv_slot = var_phit1_rv;
        *var_phitct_slot = var_phitct;
        *var_phitct_dn4_slot = var_phitct_dn4;
        *var_phitct_dn6_slot = var_phitct_dn6;
        *var_phitct_dn7_slot = var_phitct_dn7;
        *var_phitct_dn8_slot = var_phitct_dn8;
        *var_phitct_dn9_slot = var_phitct_dn9;
        *var_phitct_rv_slot = var_phitct_rv;
        *var_temp1_slot = var_temp1;
        *var_temp1_dn4_slot = var_temp1_dn4;
        *var_temp1_dn6_slot = var_temp1_dn6;
        *var_temp1_dn7_slot = var_temp1_dn7;
        *var_temp1_dn8_slot = var_temp1_dn8;
        *var_temp1_dn9_slot = var_temp1_dn9;
        *var_temp1_rv_slot = var_temp1_rv;
        *var_temp2_slot = var_temp2;
        *var_temp2_dn4_slot = var_temp2_dn4;
        *var_temp2_dn6_slot = var_temp2_dn6;
        *var_temp2_dn7_slot = var_temp2_dn7;
        *var_temp2_dn8_slot = var_temp2_dn8;
        *var_temp2_dn9_slot = var_temp2_dn9;
        *var_temp2_rv_slot = var_temp2_rv;
        *var_ux_slot = var_ux;
        *var_ux_dn4_slot = var_ux_dn4;
        *var_ux_dn6_slot = var_ux_dn6;
        *var_ux_dn7_slot = var_ux_dn7;
        *var_ux_dn8_slot = var_ux_dn8;
        *var_ux_dn9_slot = var_ux_dn9;
        *var_ux_rv_slot = var_ux_rv;
        *var_vdsp_slot = var_vdsp;
        *var_vdsp_dn7_slot = var_vdsp_dn7;
        *var_vdsp_dn8_slot = var_vdsp_dn8;
        *var_vdsp_rv_slot = var_vdsp_rv;
        *var_xb_slot = var_xb;
        *var_xb_dn4_slot = var_xb_dn4;
        *var_xb_dn6_slot = var_xb_dn6;
        *var_xb_dn7_slot = var_xb_dn7;
        *var_xb_dn8_slot = var_xb_dn8;
        *var_xb_dn9_slot = var_xb_dn9;
        *var_xb_rv_slot = var_xb_rv;
        *var_xct_slot = var_xct;
        *var_xct_dn4_slot = var_xct_dn4;
        *var_xct_dn6_slot = var_xct_dn6;
        *var_xct_dn7_slot = var_xct_dn7;
        *var_xct_dn8_slot = var_xct_dn8;
        *var_xct_dn9_slot = var_xct_dn9;
        *var_xct_rv_slot = var_xct_rv;
        *var_xctmax_slot = var_xctmax;
        *var_xctmax_dn4_slot = var_xctmax_dn4;
        *var_xctmax_rv_slot = var_xctmax_rv;
        *var_xg_slot = var_xg;
        *var_xg_dn4_slot = var_xg_dn4;
        *var_xg_dn6_slot = var_xg_dn6;
        *var_xg_dn7_slot = var_xg_dn7;
        *var_xg_dn8_slot = var_xg_dn8;
        *var_xg_dn9_slot = var_xg_dn9;
        *var_xg_rv_slot = var_xg_rv;
        *var_xgct_slot = var_xgct;
        *var_xgct_dn4_slot = var_xgct_dn4;
        *var_xgct_dn6_slot = var_xgct_dn6;
        *var_xgct_dn7_slot = var_xgct_dn7;
        *var_xgct_dn8_slot = var_xgct_dn8;
        *var_xgct_dn9_slot = var_xgct_dn9;
        *var_xgct_rv_slot = var_xgct_rv;
        *var_xmict_slot = var_xmict;
        *var_xmict_dn4_slot = var_xmict_dn4;
        *var_xmict_dn6_slot = var_xmict_dn6;
        *var_xmict_dn7_slot = var_xmict_dn7;
        *var_xmict_dn8_slot = var_xmict_dn8;
        *var_xmict_dn9_slot = var_xmict_dn9;
        *var_xmict_rv_slot = var_xmict_rv;
        *var_xn_s_slot = var_xn_s;
        *var_xn_s_dn4_slot = var_xn_s_dn4;
        *var_xn_s_dn6_slot = var_xn_s_dn6;
        *var_xn_s_dn7_slot = var_xn_s_dn7;
        *var_xn_s_dn8_slot = var_xn_s_dn8;
        *var_xn_s_dn9_slot = var_xn_s_dn9;
        *var_xn_s_rv_slot = var_xn_s_rv;
        *var_xnct_slot = var_xnct;
        *var_xnct_dn4_slot = var_xnct_dn4;
        *var_xnct_dn6_slot = var_xnct_dn6;
        *var_xnct_dn7_slot = var_xnct_dn7;
        *var_xnct_dn8_slot = var_xnct_dn8;
        *var_xnct_dn9_slot = var_xnct_dn9;
        *var_xnct_rv_slot = var_xnct_rv;
        *var_xno_s_slot = var_xno_s;
        *var_xno_s_dn4_slot = var_xno_s_dn4;
        *var_xno_s_dn6_slot = var_xno_s_dn6;
        *var_xno_s_dn7_slot = var_xno_s_dn7;
        *var_xno_s_dn8_slot = var_xno_s_dn8;
        *var_xno_s_dn9_slot = var_xno_s_dn9;
        *var_xno_s_rv_slot = var_xno_s_rv;
        *var_xsubct_slot = var_xsubct;
        *var_xsubct_dn4_slot = var_xsubct_dn4;
        *var_xsubct_dn6_slot = var_xsubct_dn6;
        *var_xsubct_dn7_slot = var_xsubct_dn7;
        *var_xsubct_dn8_slot = var_xsubct_dn8;
        *var_xsubct_dn9_slot = var_xsubct_dn9;
        *var_xsubct_rv_slot = var_xsubct_rv;
        *var_xwict_slot = var_xwict;
        *var_xwict_dn4_slot = var_xwict_dn4;
        *var_xwict_dn6_slot = var_xwict_dn6;
        *var_xwict_dn7_slot = var_xwict_dn7;
        *var_xwict_dn8_slot = var_xwict_dn8;
        *var_xwict_dn9_slot = var_xwict_dn9;
        *var_xwict_rv_slot = var_xwict_rv;
    }

    pub(super) fn stamp_reactive_block_25(
        p: &Parameters,
        var_delxb: f64,
        var_delxb_dn4: f64,
        var_delxb_dn6: f64,
        var_delxb_dn7: f64,
        var_delxb_dn8: f64,
        var_delxb_dn9: f64,
        var_gf: f64,
        var_gf2: f64,
        var_gf2_dn4: f64,
        var_gf2_dn6: f64,
        var_gf2_dn7: f64,
        var_gf2_dn8: f64,
        var_gf2_dn9: f64,
        var_gf_dn4: f64,
        var_gf_dn6: f64,
        var_gf_dn7: f64,
        var_gf_dn8: f64,
        var_gf_dn9: f64,
        var_xg: f64,
        var_xg_dn4: f64,
        var_xg_dn6: f64,
        var_xg_dn7: f64,
        var_xg_dn8: f64,
        var_xg_dn9: f64,
        var_xno_s: f64,
        var_xno_s_dn4: f64,
        var_xno_s_dn6: f64,
        var_xno_s_dn7: f64,
        var_xno_s_dn8: f64,
        var_xno_s_dn9: f64,
        var_delta_ns_slot: &mut f64,
        var_delta_ns_dn4_slot: &mut f64,
        var_delta_ns_dn6_slot: &mut f64,
        var_delta_ns_dn7_slot: &mut f64,
        var_delta_ns_dn8_slot: &mut f64,
        var_delta_ns_dn9_slot: &mut f64,
        var_delta_ns_rv_slot: &mut f64,
        var_dscr0_slot: &mut f64,
        var_dscr0_dn4_slot: &mut f64,
        var_dscr0_dn6_slot: &mut f64,
        var_dscr0_dn7_slot: &mut f64,
        var_dscr0_dn8_slot: &mut f64,
        var_dscr0_dn9_slot: &mut f64,
        var_dscr0_rv_slot: &mut f64,
        var_fscr_slot: &mut f64,
        var_fscr_dn4_slot: &mut f64,
        var_fscr_dn6_slot: &mut f64,
        var_fscr_dn7_slot: &mut f64,
        var_fscr_dn8_slot: &mut f64,
        var_fscr_dn9_slot: &mut f64,
        var_fscr_rv_slot: &mut f64,
        var_guard1192_slot: &mut f64,
        var_guard1192_rv_slot: &mut f64,
        var_guard1193_slot: &mut f64,
        var_guard1193_rv_slot: &mut f64,
        var_guard1194_slot: &mut f64,
        var_guard1194_rv_slot: &mut f64,
        var_guard1195_slot: &mut f64,
        var_guard1195_rv_slot: &mut f64,
        var_guard1196_slot: &mut f64,
        var_guard1196_rv_slot: &mut f64,
        var_guard1197_slot: &mut f64,
        var_guard1197_rv_slot: &mut f64,
        var_nscr_slot: &mut f64,
        var_nscr_dn4_slot: &mut f64,
        var_nscr_dn6_slot: &mut f64,
        var_nscr_dn7_slot: &mut f64,
        var_nscr_dn8_slot: &mut f64,
        var_nscr_dn9_slot: &mut f64,
        var_nscr_rv_slot: &mut f64,
        var_qbscr_slot: &mut f64,
        var_qbscr_dn4_slot: &mut f64,
        var_qbscr_dn6_slot: &mut f64,
        var_qbscr_dn7_slot: &mut f64,
        var_qbscr_dn8_slot: &mut f64,
        var_qbscr_dn9_slot: &mut f64,
        var_qbscr_rv_slot: &mut f64,
        var_qiscr_slot: &mut f64,
        var_qiscr0_slot: &mut f64,
        var_qiscr0_dn4_slot: &mut f64,
        var_qiscr0_dn6_slot: &mut f64,
        var_qiscr0_dn7_slot: &mut f64,
        var_qiscr0_dn8_slot: &mut f64,
        var_qiscr0_dn9_slot: &mut f64,
        var_qiscr0_rv_slot: &mut f64,
        var_qiscr0si_slot: &mut f64,
        var_qiscr0si_dn4_slot: &mut f64,
        var_qiscr0si_dn6_slot: &mut f64,
        var_qiscr0si_dn7_slot: &mut f64,
        var_qiscr0si_dn8_slot: &mut f64,
        var_qiscr0si_dn9_slot: &mut f64,
        var_qiscr0si_rv_slot: &mut f64,
        var_qiscr_dn4_slot: &mut f64,
        var_qiscr_dn6_slot: &mut f64,
        var_qiscr_dn7_slot: &mut f64,
        var_qiscr_dn8_slot: &mut f64,
        var_qiscr_dn9_slot: &mut f64,
        var_qiscr_rv_slot: &mut f64,
        var_temp__blk949_slot: &mut f64,
        var_temp__blk949_dn4_slot: &mut f64,
        var_temp__blk949_dn6_slot: &mut f64,
        var_temp__blk949_dn7_slot: &mut f64,
        var_temp__blk949_dn8_slot: &mut f64,
        var_temp__blk949_dn9_slot: &mut f64,
        var_temp__blk949_rv_slot: &mut f64,
        var_xgtscr_slot: &mut f64,
        var_xgtscr0_slot: &mut f64,
        var_xgtscr0_dn4_slot: &mut f64,
        var_xgtscr0_dn6_slot: &mut f64,
        var_xgtscr0_dn7_slot: &mut f64,
        var_xgtscr0_dn8_slot: &mut f64,
        var_xgtscr0_dn9_slot: &mut f64,
        var_xgtscr0_rv_slot: &mut f64,
        var_xgtscr_dn4_slot: &mut f64,
        var_xgtscr_dn6_slot: &mut f64,
        var_xgtscr_dn7_slot: &mut f64,
        var_xgtscr_dn8_slot: &mut f64,
        var_xgtscr_dn9_slot: &mut f64,
        var_xgtscr_rv_slot: &mut f64,
        var_xi_slot: &mut f64,
        var_xi_dn4_slot: &mut f64,
        var_xi_dn6_slot: &mut f64,
        var_xi_dn7_slot: &mut f64,
        var_xi_dn8_slot: &mut f64,
        var_xi_dn9_slot: &mut f64,
        var_xi_rv_slot: &mut f64,
        var_xn_s_slot: &mut f64,
        var_xn_s_dn4_slot: &mut f64,
        var_xn_s_dn6_slot: &mut f64,
        var_xn_s_dn7_slot: &mut f64,
        var_xn_s_dn8_slot: &mut f64,
        var_xn_s_dn9_slot: &mut f64,
        var_xn_s_rv_slot: &mut f64,
        var_xthscr_slot: &mut f64,
        var_xthscr_dn4_slot: &mut f64,
        var_xthscr_dn6_slot: &mut f64,
        var_xthscr_dn7_slot: &mut f64,
        var_xthscr_dn8_slot: &mut f64,
        var_xthscr_dn9_slot: &mut f64,
        var_xthscr_rv_slot: &mut f64,
    ) {
        let mut var_delta_ns: f64 = *var_delta_ns_slot;
        let mut var_delta_ns_dn4: f64 = *var_delta_ns_dn4_slot;
        let mut var_delta_ns_dn6: f64 = *var_delta_ns_dn6_slot;
        let mut var_delta_ns_dn7: f64 = *var_delta_ns_dn7_slot;
        let mut var_delta_ns_dn8: f64 = *var_delta_ns_dn8_slot;
        let mut var_delta_ns_dn9: f64 = *var_delta_ns_dn9_slot;
        let mut var_delta_ns_rv: f64 = *var_delta_ns_rv_slot;
        let mut var_dscr0: f64 = *var_dscr0_slot;
        let mut var_dscr0_dn4: f64 = *var_dscr0_dn4_slot;
        let mut var_dscr0_dn6: f64 = *var_dscr0_dn6_slot;
        let mut var_dscr0_dn7: f64 = *var_dscr0_dn7_slot;
        let mut var_dscr0_dn8: f64 = *var_dscr0_dn8_slot;
        let mut var_dscr0_dn9: f64 = *var_dscr0_dn9_slot;
        let mut var_dscr0_rv: f64 = *var_dscr0_rv_slot;
        let mut var_fscr: f64 = *var_fscr_slot;
        let mut var_fscr_dn4: f64 = *var_fscr_dn4_slot;
        let mut var_fscr_dn6: f64 = *var_fscr_dn6_slot;
        let mut var_fscr_dn7: f64 = *var_fscr_dn7_slot;
        let mut var_fscr_dn8: f64 = *var_fscr_dn8_slot;
        let mut var_fscr_dn9: f64 = *var_fscr_dn9_slot;
        let mut var_fscr_rv: f64 = *var_fscr_rv_slot;
        let mut var_guard1192: f64 = *var_guard1192_slot;
        let mut var_guard1192_rv: f64 = *var_guard1192_rv_slot;
        let mut var_guard1193: f64 = *var_guard1193_slot;
        let mut var_guard1193_rv: f64 = *var_guard1193_rv_slot;
        let mut var_guard1194: f64 = *var_guard1194_slot;
        let mut var_guard1194_rv: f64 = *var_guard1194_rv_slot;
        let mut var_guard1195: f64 = *var_guard1195_slot;
        let mut var_guard1195_rv: f64 = *var_guard1195_rv_slot;
        let mut var_guard1196: f64 = *var_guard1196_slot;
        let mut var_guard1196_rv: f64 = *var_guard1196_rv_slot;
        let mut var_guard1197: f64 = *var_guard1197_slot;
        let mut var_guard1197_rv: f64 = *var_guard1197_rv_slot;
        let mut var_nscr: f64 = *var_nscr_slot;
        let mut var_nscr_dn4: f64 = *var_nscr_dn4_slot;
        let mut var_nscr_dn6: f64 = *var_nscr_dn6_slot;
        let mut var_nscr_dn7: f64 = *var_nscr_dn7_slot;
        let mut var_nscr_dn8: f64 = *var_nscr_dn8_slot;
        let mut var_nscr_dn9: f64 = *var_nscr_dn9_slot;
        let mut var_nscr_rv: f64 = *var_nscr_rv_slot;
        let mut var_qbscr: f64 = *var_qbscr_slot;
        let mut var_qbscr_dn4: f64 = *var_qbscr_dn4_slot;
        let mut var_qbscr_dn6: f64 = *var_qbscr_dn6_slot;
        let mut var_qbscr_dn7: f64 = *var_qbscr_dn7_slot;
        let mut var_qbscr_dn8: f64 = *var_qbscr_dn8_slot;
        let mut var_qbscr_dn9: f64 = *var_qbscr_dn9_slot;
        let mut var_qbscr_rv: f64 = *var_qbscr_rv_slot;
        let mut var_qiscr: f64 = *var_qiscr_slot;
        let mut var_qiscr0: f64 = *var_qiscr0_slot;
        let mut var_qiscr0_dn4: f64 = *var_qiscr0_dn4_slot;
        let mut var_qiscr0_dn6: f64 = *var_qiscr0_dn6_slot;
        let mut var_qiscr0_dn7: f64 = *var_qiscr0_dn7_slot;
        let mut var_qiscr0_dn8: f64 = *var_qiscr0_dn8_slot;
        let mut var_qiscr0_dn9: f64 = *var_qiscr0_dn9_slot;
        let mut var_qiscr0_rv: f64 = *var_qiscr0_rv_slot;
        let mut var_qiscr0si: f64 = *var_qiscr0si_slot;
        let mut var_qiscr0si_dn4: f64 = *var_qiscr0si_dn4_slot;
        let mut var_qiscr0si_dn6: f64 = *var_qiscr0si_dn6_slot;
        let mut var_qiscr0si_dn7: f64 = *var_qiscr0si_dn7_slot;
        let mut var_qiscr0si_dn8: f64 = *var_qiscr0si_dn8_slot;
        let mut var_qiscr0si_dn9: f64 = *var_qiscr0si_dn9_slot;
        let mut var_qiscr0si_rv: f64 = *var_qiscr0si_rv_slot;
        let mut var_qiscr_dn4: f64 = *var_qiscr_dn4_slot;
        let mut var_qiscr_dn6: f64 = *var_qiscr_dn6_slot;
        let mut var_qiscr_dn7: f64 = *var_qiscr_dn7_slot;
        let mut var_qiscr_dn8: f64 = *var_qiscr_dn8_slot;
        let mut var_qiscr_dn9: f64 = *var_qiscr_dn9_slot;
        let mut var_qiscr_rv: f64 = *var_qiscr_rv_slot;
        let mut var_temp__blk949: f64 = *var_temp__blk949_slot;
        let mut var_temp__blk949_dn4: f64 = *var_temp__blk949_dn4_slot;
        let mut var_temp__blk949_dn6: f64 = *var_temp__blk949_dn6_slot;
        let mut var_temp__blk949_dn7: f64 = *var_temp__blk949_dn7_slot;
        let mut var_temp__blk949_dn8: f64 = *var_temp__blk949_dn8_slot;
        let mut var_temp__blk949_dn9: f64 = *var_temp__blk949_dn9_slot;
        let mut var_temp__blk949_rv: f64 = *var_temp__blk949_rv_slot;
        let mut var_xgtscr: f64 = *var_xgtscr_slot;
        let mut var_xgtscr0: f64 = *var_xgtscr0_slot;
        let mut var_xgtscr0_dn4: f64 = *var_xgtscr0_dn4_slot;
        let mut var_xgtscr0_dn6: f64 = *var_xgtscr0_dn6_slot;
        let mut var_xgtscr0_dn7: f64 = *var_xgtscr0_dn7_slot;
        let mut var_xgtscr0_dn8: f64 = *var_xgtscr0_dn8_slot;
        let mut var_xgtscr0_dn9: f64 = *var_xgtscr0_dn9_slot;
        let mut var_xgtscr0_rv: f64 = *var_xgtscr0_rv_slot;
        let mut var_xgtscr_dn4: f64 = *var_xgtscr_dn4_slot;
        let mut var_xgtscr_dn6: f64 = *var_xgtscr_dn6_slot;
        let mut var_xgtscr_dn7: f64 = *var_xgtscr_dn7_slot;
        let mut var_xgtscr_dn8: f64 = *var_xgtscr_dn8_slot;
        let mut var_xgtscr_dn9: f64 = *var_xgtscr_dn9_slot;
        let mut var_xgtscr_rv: f64 = *var_xgtscr_rv_slot;
        let mut var_xi: f64 = *var_xi_slot;
        let mut var_xi_dn4: f64 = *var_xi_dn4_slot;
        let mut var_xi_dn6: f64 = *var_xi_dn6_slot;
        let mut var_xi_dn7: f64 = *var_xi_dn7_slot;
        let mut var_xi_dn8: f64 = *var_xi_dn8_slot;
        let mut var_xi_dn9: f64 = *var_xi_dn9_slot;
        let mut var_xi_rv: f64 = *var_xi_rv_slot;
        let mut var_xn_s: f64 = *var_xn_s_slot;
        let mut var_xn_s_dn4: f64 = *var_xn_s_dn4_slot;
        let mut var_xn_s_dn6: f64 = *var_xn_s_dn6_slot;
        let mut var_xn_s_dn7: f64 = *var_xn_s_dn7_slot;
        let mut var_xn_s_dn8: f64 = *var_xn_s_dn8_slot;
        let mut var_xn_s_dn9: f64 = *var_xn_s_dn9_slot;
        let mut var_xn_s_rv: f64 = *var_xn_s_rv_slot;
        let mut var_xthscr: f64 = *var_xthscr_slot;
        let mut var_xthscr_dn4: f64 = *var_xthscr_dn4_slot;
        let mut var_xthscr_dn6: f64 = *var_xthscr_dn6_slot;
        let mut var_xthscr_dn7: f64 = *var_xthscr_dn7_slot;
        let mut var_xthscr_dn8: f64 = *var_xthscr_dn8_slot;
        let mut var_xthscr_dn9: f64 = *var_xthscr_dn9_slot;
        let mut var_xthscr_rv: f64 = *var_xthscr_rv_slot;

        let assign41380_e54295: f64 = if p.p45 > 0.0 { 1.0 } else { 0.0 };
        var_guard1192 = assign41380_e54295;
        var_guard1192_rv = 0.0;

        let assign41390_e54297: f64 = (var_xn_s).abs();
        let assign41390_e54299: f64 = if assign41390_e54297 < 1e-5 { 1.0 } else { 0.0 };
        var_guard1193 = assign41390_e54299;
        var_guard1193_rv = 0.0;

        let (assign41400_e54319, assign41400_e54319_d_n4, assign41400_e54319_d_n6, assign41400_e54319_d_n7, assign41400_e54319_d_n8, assign41400_e54319_d_n9,) = {
    if ((var_guard1192 != 0.0) && (var_guard1193 != 0.0)) {
        let assign41400_e54308: f64 = (0.5 * var_xn_s);
        let assign41400_e54312: f64 = (0.3125 * var_xn_s);
        let assign41400_e54313: f64 = (1.0 - assign41400_e54312);
        let assign41400_e54314: f64 = (assign41400_e54308 * assign41400_e54313);
        let assign41400_e54315: f64 = (1.0 - assign41400_e54314);
        let assign41400_e54316: f64 = (var_gf * assign41400_e54315);
        let assign41400_e54317: f64 = (1.0 + assign41400_e54316);
        (assign41400_e54317, ((var_gf_dn4 * assign41400_e54315) + (var_gf * (-(((0.5 * var_xn_s_dn4) * assign41400_e54313) + (assign41400_e54308 * (-(0.3125 * var_xn_s_dn4))))))), ((var_gf_dn6 * assign41400_e54315) + (var_gf * (-(((0.5 * var_xn_s_dn6) * assign41400_e54313) + (assign41400_e54308 * (-(0.3125 * var_xn_s_dn6))))))), ((var_gf_dn7 * assign41400_e54315) + (var_gf * (-(((0.5 * var_xn_s_dn7) * assign41400_e54313) + (assign41400_e54308 * (-(0.3125 * var_xn_s_dn7))))))), ((var_gf_dn8 * assign41400_e54315) + (var_gf * (-(((0.5 * var_xn_s_dn8) * assign41400_e54313) + (assign41400_e54308 * (-(0.3125 * var_xn_s_dn8))))))), ((var_gf_dn9 * assign41400_e54315) + (var_gf * (-(((0.5 * var_xn_s_dn9) * assign41400_e54313) + (assign41400_e54308 * (-(0.3125 * var_xn_s_dn9))))))),)
    } else {
        (var_nscr, var_nscr_dn4, var_nscr_dn6, var_nscr_dn7, var_nscr_dn8, var_nscr_dn9,)
    }
};
        var_nscr = assign41400_e54319;
        var_nscr_dn4 = assign41400_e54319_d_n4;
        var_nscr_dn6 = assign41400_e54319_d_n6;
        var_nscr_dn7 = assign41400_e54319_d_n7;
        var_nscr_dn8 = assign41400_e54319_d_n8;
        var_nscr_dn9 = assign41400_e54319_d_n9;
        var_nscr_rv = 0.0;

        let assign41410_e54322: f64 = if var_xn_s < 460.51701859880916 { 1.0 } else { 0.0 };
        var_guard1194 = assign41410_e54322;
        var_guard1194_rv = 0.0;

        let (assign41420_e54333, assign41420_e54333_d_n4, assign41420_e54333_d_n6, assign41420_e54333_d_n7, assign41420_e54333_d_n8, assign41420_e54333_d_n9,) = {
    if (((var_guard1192 != 0.0) && (var_guard1193 == 0.0)) && (var_guard1194 != 0.0)) {
        let assign41420_e54330: f64 = (-var_xn_s);
        let assign41420_e54331: f64 = (assign41420_e54330).exp();
        (assign41420_e54331, (assign41420_e54331 * (-var_xn_s_dn4)), (assign41420_e54331 * (-var_xn_s_dn6)), (assign41420_e54331 * (-var_xn_s_dn7)), (assign41420_e54331 * (-var_xn_s_dn8)), (assign41420_e54331 * (-var_xn_s_dn9)),)
    } else {
        (var_delta_ns, var_delta_ns_dn4, var_delta_ns_dn6, var_delta_ns_dn7, var_delta_ns_dn8, var_delta_ns_dn9,)
    }
};
        var_delta_ns = assign41420_e54333;
        var_delta_ns_dn4 = assign41420_e54333_d_n4;
        var_delta_ns_dn6 = assign41420_e54333_d_n6;
        var_delta_ns_dn7 = assign41420_e54333_d_n7;
        var_delta_ns_dn8 = assign41420_e54333_d_n8;
        var_delta_ns_dn9 = assign41420_e54333_d_n9;
        var_delta_ns_rv = 0.0;

        let (assign41430_e54365, assign41430_e54365_d_n4, assign41430_e54365_d_n6, assign41430_e54365_d_n7, assign41430_e54365_d_n8, assign41430_e54365_d_n9,) = {
    if (((var_guard1192 != 0.0) && (var_guard1193 == 0.0)) && (var_guard1194 == 0.0)) {
        let assign41430_e54345: f64 = (var_xn_s - 460.51701859880916);
        let assign41430_e54350: f64 = (var_xn_s - 460.51701859880916);
        let assign41430_e54354: f64 = (var_xn_s - 460.51701859880916);
        let assign41430_e54356: f64 = (assign41430_e54354 * 0.3333333333333333);
        let assign41430_e54357: f64 = (1.0 + assign41430_e54356);
        let assign41430_e54358: f64 = (assign41430_e54350 * assign41430_e54357);
        let assign41430_e54359: f64 = (0.5 * assign41430_e54358);
        let assign41430_e54360: f64 = (1.0 + assign41430_e54359);
        let assign41430_e54361: f64 = (assign41430_e54345 * assign41430_e54360);
        let assign41430_e54362: f64 = (1.0 + assign41430_e54361);
        let assign41430_e54363: f64 = (1e-200 / assign41430_e54362);
        (assign41430_e54363, (-((1e-200 * ((var_xn_s_dn4 * assign41430_e54360) + (assign41430_e54345 * (0.5 * ((var_xn_s_dn4 * assign41430_e54357) + (assign41430_e54350 * (var_xn_s_dn4 * 0.3333333333333333))))))) / (assign41430_e54362 * assign41430_e54362))), (-((1e-200 * ((var_xn_s_dn6 * assign41430_e54360) + (assign41430_e54345 * (0.5 * ((var_xn_s_dn6 * assign41430_e54357) + (assign41430_e54350 * (var_xn_s_dn6 * 0.3333333333333333))))))) / (assign41430_e54362 * assign41430_e54362))), (-((1e-200 * ((var_xn_s_dn7 * assign41430_e54360) + (assign41430_e54345 * (0.5 * ((var_xn_s_dn7 * assign41430_e54357) + (assign41430_e54350 * (var_xn_s_dn7 * 0.3333333333333333))))))) / (assign41430_e54362 * assign41430_e54362))), (-((1e-200 * ((var_xn_s_dn8 * assign41430_e54360) + (assign41430_e54345 * (0.5 * ((var_xn_s_dn8 * assign41430_e54357) + (assign41430_e54350 * (var_xn_s_dn8 * 0.3333333333333333))))))) / (assign41430_e54362 * assign41430_e54362))), (-((1e-200 * ((var_xn_s_dn9 * assign41430_e54360) + (assign41430_e54345 * (0.5 * ((var_xn_s_dn9 * assign41430_e54357) + (assign41430_e54350 * (var_xn_s_dn9 * 0.3333333333333333))))))) / (assign41430_e54362 * assign41430_e54362))),)
    } else {
        (var_delta_ns, var_delta_ns_dn4, var_delta_ns_dn6, var_delta_ns_dn7, var_delta_ns_dn8, var_delta_ns_dn9,)
    }
};
        var_delta_ns = assign41430_e54365;
        var_delta_ns_dn4 = assign41430_e54365_d_n4;
        var_delta_ns_dn6 = assign41430_e54365_d_n6;
        var_delta_ns_dn7 = assign41430_e54365_d_n7;
        var_delta_ns_dn8 = assign41430_e54365_d_n8;
        var_delta_ns_dn9 = assign41430_e54365_d_n9;
        var_delta_ns_rv = 0.0;

        let (assign41440_e54378, assign41440_e54378_d_n4, assign41440_e54378_d_n6, assign41440_e54378_d_n7, assign41440_e54378_d_n8, assign41440_e54378_d_n9,) = {
    if ((var_guard1192 != 0.0) && (var_guard1193 == 0.0)) {
        let (assign41440_e54376,) = {
            if (var_xn_s > 0.0) {
                (1.0,)
            } else {
                let assign41440_e54375: f64 = (-1.0);
                (assign41440_e54375,)
            }
        };
        (assign41440_e54376, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign41440_e54378;
        var_temp__blk949_dn4 = assign41440_e54378_d_n4;
        var_temp__blk949_dn6 = assign41440_e54378_d_n6;
        var_temp__blk949_dn7 = assign41440_e54378_d_n7;
        var_temp__blk949_dn8 = assign41440_e54378_d_n8;
        var_temp__blk949_dn9 = assign41440_e54378_d_n9;
        var_temp__blk949_rv = 0.0;

        let (assign41450_e54406, assign41450_e54406_d_n4, assign41450_e54406_d_n6, assign41450_e54406_d_n7, assign41450_e54406_d_n8, assign41450_e54406_d_n9,) = {
    if ((var_guard1192 != 0.0) && (var_guard1193 == 0.0)) {
        let assign41450_e54386: f64 = (var_temp__blk949 * var_gf);
        let assign41450_e54391: f64 = (1.0 - var_xn_s);
        let assign41450_e54392: f64 = (var_delta_ns * assign41450_e54391);
        let assign41450_e54393: f64 = (1.0 - assign41450_e54392);
        let assign41450_e54394: f64 = (assign41450_e54386 * assign41450_e54393);
        let assign41450_e54399: f64 = (1.0 - var_delta_ns);
        let assign41450_e54400: f64 = (var_xn_s * assign41450_e54399);
        let assign41450_e54401: f64 = (assign41450_e54400).sqrt();
        let assign41450_e54402: f64 = (2.0 * assign41450_e54401);
        let assign41450_e54403: f64 = (assign41450_e54394 / assign41450_e54402);
        let assign41450_e54404: f64 = (1.0 + assign41450_e54403);
        (assign41450_e54404, (((((((var_temp__blk949_dn4 * var_gf) + (var_temp__blk949 * var_gf_dn4)) * assign41450_e54393) + (assign41450_e54386 * (-((var_delta_ns_dn4 * assign41450_e54391) + (var_delta_ns * (-var_xn_s_dn4)))))) * assign41450_e54402) - (assign41450_e54394 * (2.0 * (((var_xn_s_dn4 * assign41450_e54399) + (var_xn_s * (-var_delta_ns_dn4))) / (2.0 * assign41450_e54401))))) / (assign41450_e54402 * assign41450_e54402)), (((((((var_temp__blk949_dn6 * var_gf) + (var_temp__blk949 * var_gf_dn6)) * assign41450_e54393) + (assign41450_e54386 * (-((var_delta_ns_dn6 * assign41450_e54391) + (var_delta_ns * (-var_xn_s_dn6)))))) * assign41450_e54402) - (assign41450_e54394 * (2.0 * (((var_xn_s_dn6 * assign41450_e54399) + (var_xn_s * (-var_delta_ns_dn6))) / (2.0 * assign41450_e54401))))) / (assign41450_e54402 * assign41450_e54402)), (((((((var_temp__blk949_dn7 * var_gf) + (var_temp__blk949 * var_gf_dn7)) * assign41450_e54393) + (assign41450_e54386 * (-((var_delta_ns_dn7 * assign41450_e54391) + (var_delta_ns * (-var_xn_s_dn7)))))) * assign41450_e54402) - (assign41450_e54394 * (2.0 * (((var_xn_s_dn7 * assign41450_e54399) + (var_xn_s * (-var_delta_ns_dn7))) / (2.0 * assign41450_e54401))))) / (assign41450_e54402 * assign41450_e54402)), (((((((var_temp__blk949_dn8 * var_gf) + (var_temp__blk949 * var_gf_dn8)) * assign41450_e54393) + (assign41450_e54386 * (-((var_delta_ns_dn8 * assign41450_e54391) + (var_delta_ns * (-var_xn_s_dn8)))))) * assign41450_e54402) - (assign41450_e54394 * (2.0 * (((var_xn_s_dn8 * assign41450_e54399) + (var_xn_s * (-var_delta_ns_dn8))) / (2.0 * assign41450_e54401))))) / (assign41450_e54402 * assign41450_e54402)), (((((((var_temp__blk949_dn9 * var_gf) + (var_temp__blk949 * var_gf_dn9)) * assign41450_e54393) + (assign41450_e54386 * (-((var_delta_ns_dn9 * assign41450_e54391) + (var_delta_ns * (-var_xn_s_dn9)))))) * assign41450_e54402) - (assign41450_e54394 * (2.0 * (((var_xn_s_dn9 * assign41450_e54399) + (var_xn_s * (-var_delta_ns_dn9))) / (2.0 * assign41450_e54401))))) / (assign41450_e54402 * assign41450_e54402)),)
    } else {
        (var_nscr, var_nscr_dn4, var_nscr_dn6, var_nscr_dn7, var_nscr_dn8, var_nscr_dn9,)
    }
};
        var_nscr = assign41450_e54406;
        var_nscr_dn4 = assign41450_e54406_d_n4;
        var_nscr_dn6 = assign41450_e54406_d_n6;
        var_nscr_dn7 = assign41450_e54406_d_n7;
        var_nscr_dn8 = assign41450_e54406_d_n8;
        var_nscr_dn9 = assign41450_e54406_d_n9;
        var_nscr_rv = 0.0;

        let (assign41460_e54418, assign41460_e54418_d_n4, assign41460_e54418_d_n6, assign41460_e54418_d_n7, assign41460_e54418_d_n8, assign41460_e54418_d_n9,) = {
    if (var_guard1192 == 0.0) {
        let assign41460_e54412: f64 = (0.5 * var_gf);
        let assign41460_e54414: f64 = (var_xn_s).sqrt();
        let assign41460_e54415: f64 = (assign41460_e54412 / assign41460_e54414);
        let assign41460_e54416: f64 = (1.0 + assign41460_e54415);
        (assign41460_e54416, ((((0.5 * var_gf_dn4) * assign41460_e54414) - (assign41460_e54412 * (var_xn_s_dn4 / (2.0 * assign41460_e54414)))) / (assign41460_e54414 * assign41460_e54414)), ((((0.5 * var_gf_dn6) * assign41460_e54414) - (assign41460_e54412 * (var_xn_s_dn6 / (2.0 * assign41460_e54414)))) / (assign41460_e54414 * assign41460_e54414)), ((((0.5 * var_gf_dn7) * assign41460_e54414) - (assign41460_e54412 * (var_xn_s_dn7 / (2.0 * assign41460_e54414)))) / (assign41460_e54414 * assign41460_e54414)), ((((0.5 * var_gf_dn8) * assign41460_e54414) - (assign41460_e54412 * (var_xn_s_dn8 / (2.0 * assign41460_e54414)))) / (assign41460_e54414 * assign41460_e54414)), ((((0.5 * var_gf_dn9) * assign41460_e54414) - (assign41460_e54412 * (var_xn_s_dn9 / (2.0 * assign41460_e54414)))) / (assign41460_e54414 * assign41460_e54414)),)
    } else {
        (var_nscr, var_nscr_dn4, var_nscr_dn6, var_nscr_dn7, var_nscr_dn8, var_nscr_dn9,)
    }
};
        var_nscr = assign41460_e54418;
        var_nscr_dn4 = assign41460_e54418_d_n4;
        var_nscr_dn6 = assign41460_e54418_d_n6;
        var_nscr_dn7 = assign41460_e54418_d_n7;
        var_nscr_dn8 = assign41460_e54418_d_n8;
        var_nscr_dn9 = assign41460_e54418_d_n9;
        var_nscr_rv = 0.0;

        let assign41470_e54422: f64 = (var_xn_s).sqrt();
        let assign41470_e54423: f64 = (var_gf * assign41470_e54422);
        let assign41470_e54424: f64 = (var_xn_s + assign41470_e54423);
        let assign41470_e54428: f64 = (var_nscr - 1.0);
        let assign41470_e54429: f64 = (assign41470_e54428).ln();
        let assign41470_e54430: f64 = (var_nscr * assign41470_e54429);
        let assign41470_e54431: f64 = (assign41470_e54424 - assign41470_e54430);
        var_xthscr = assign41470_e54431;
        var_xthscr_dn4 = ((var_xn_s_dn4 + ((var_gf_dn4 * assign41470_e54422) + (var_gf * (var_xn_s_dn4 / (2.0 * assign41470_e54422))))) - ((var_nscr_dn4 * assign41470_e54429) + (var_nscr * (var_nscr_dn4 / assign41470_e54428))));
        var_xthscr_dn6 = ((var_xn_s_dn6 + ((var_gf_dn6 * assign41470_e54422) + (var_gf * (var_xn_s_dn6 / (2.0 * assign41470_e54422))))) - ((var_nscr_dn6 * assign41470_e54429) + (var_nscr * (var_nscr_dn6 / assign41470_e54428))));
        var_xthscr_dn7 = ((var_xn_s_dn7 + ((var_gf_dn7 * assign41470_e54422) + (var_gf * (var_xn_s_dn7 / (2.0 * assign41470_e54422))))) - ((var_nscr_dn7 * assign41470_e54429) + (var_nscr * (var_nscr_dn7 / assign41470_e54428))));
        var_xthscr_dn8 = ((var_xn_s_dn8 + ((var_gf_dn8 * assign41470_e54422) + (var_gf * (var_xn_s_dn8 / (2.0 * assign41470_e54422))))) - ((var_nscr_dn8 * assign41470_e54429) + (var_nscr * (var_nscr_dn8 / assign41470_e54428))));
        var_xthscr_dn9 = ((var_xn_s_dn9 + ((var_gf_dn9 * assign41470_e54422) + (var_gf * (var_xn_s_dn9 / (2.0 * assign41470_e54422))))) - ((var_nscr_dn9 * assign41470_e54429) + (var_nscr * (var_nscr_dn9 / assign41470_e54428))));
        var_xthscr_rv = 0.0;

        let assign41480_e54434: f64 = (var_xg - var_xthscr);
        let assign41480_e54436: f64 = (assign41480_e54434 / var_nscr);
        var_xgtscr = assign41480_e54436;
        var_xgtscr_dn4 = ((((var_xg_dn4 - var_xthscr_dn4) * var_nscr) - (assign41480_e54434 * var_nscr_dn4)) / (var_nscr * var_nscr));
        var_xgtscr_dn6 = ((((var_xg_dn6 - var_xthscr_dn6) * var_nscr) - (assign41480_e54434 * var_nscr_dn6)) / (var_nscr * var_nscr));
        var_xgtscr_dn7 = ((((var_xg_dn7 - var_xthscr_dn7) * var_nscr) - (assign41480_e54434 * var_nscr_dn7)) / (var_nscr * var_nscr));
        var_xgtscr_dn8 = ((((var_xg_dn8 - var_xthscr_dn8) * var_nscr) - (assign41480_e54434 * var_nscr_dn8)) / (var_nscr * var_nscr));
        var_xgtscr_dn9 = ((((var_xg_dn9 - var_xthscr_dn9) * var_nscr) - (assign41480_e54434 * var_nscr_dn9)) / (var_nscr * var_nscr));
        var_xgtscr_rv = 0.0;

        let assign41490_e54439: f64 = (0.5 * var_gf2);
        let assign41490_e54443: f64 = (8.0 / var_gf2);
        let assign41490_e54444: f64 = (1.0 + assign41490_e54443);
        let assign41490_e54445: f64 = (assign41490_e54444).sqrt();
        let assign41490_e54447: f64 = (assign41490_e54445 - 1.0);
        let assign41490_e54448: f64 = (assign41490_e54439 * assign41490_e54447);
        var_qbscr = assign41490_e54448;
        var_qbscr_dn4 = (((0.5 * var_gf2_dn4) * assign41490_e54447) + (assign41490_e54439 * ((-((8.0 * var_gf2_dn4) / (var_gf2 * var_gf2))) / (2.0 * assign41490_e54445))));
        var_qbscr_dn6 = (((0.5 * var_gf2_dn6) * assign41490_e54447) + (assign41490_e54439 * ((-((8.0 * var_gf2_dn6) / (var_gf2 * var_gf2))) / (2.0 * assign41490_e54445))));
        var_qbscr_dn7 = (((0.5 * var_gf2_dn7) * assign41490_e54447) + (assign41490_e54439 * ((-((8.0 * var_gf2_dn7) / (var_gf2 * var_gf2))) / (2.0 * assign41490_e54445))));
        var_qbscr_dn8 = (((0.5 * var_gf2_dn8) * assign41490_e54447) + (assign41490_e54439 * ((-((8.0 * var_gf2_dn8) / (var_gf2 * var_gf2))) / (2.0 * assign41490_e54445))));
        var_qbscr_dn9 = (((0.5 * var_gf2_dn9) * assign41490_e54447) + (assign41490_e54439 * ((-((8.0 * var_gf2_dn9) / (var_gf2 * var_gf2))) / (2.0 * assign41490_e54445))));
        var_qbscr_rv = 0.0;

        var_qiscr = 0.0;
        var_qiscr_dn4 = 0.0;
        var_qiscr_dn6 = 0.0;
        var_qiscr_dn7 = 0.0;
        var_qiscr_dn8 = 0.0;
        var_qiscr_dn9 = 0.0;
        var_qiscr_rv = 0.0;

        var_fscr = 1.0;
        var_fscr_dn4 = 0.0;
        var_fscr_dn6 = 0.0;
        var_fscr_dn7 = 0.0;
        var_fscr_dn8 = 0.0;
        var_fscr_dn9 = 0.0;
        var_fscr_rv = 0.0;

        let assign41520_e54453: f64 = (-30.0);
        let assign41520_e54454: f64 = if var_xgtscr > assign41520_e54453 { 1.0 } else { 0.0 };
        var_guard1195 = assign41520_e54454;
        var_guard1195_rv = 0.0;

        let (assign41530_e54462, assign41530_e54462_d_n4, assign41530_e54462_d_n6, assign41530_e54462_d_n7, assign41530_e54462_d_n8, assign41530_e54462_d_n9,) = {
    if (var_guard1195 != 0.0) {
        let assign41530_e54458: f64 = (var_nscr * var_xgtscr);
        let assign41530_e54460: f64 = (assign41530_e54458 - 1.0);
        (assign41530_e54460, ((var_nscr_dn4 * var_xgtscr) + (var_nscr * var_xgtscr_dn4)), ((var_nscr_dn6 * var_xgtscr) + (var_nscr * var_xgtscr_dn6)), ((var_nscr_dn7 * var_xgtscr) + (var_nscr * var_xgtscr_dn7)), ((var_nscr_dn8 * var_xgtscr) + (var_nscr * var_xgtscr_dn8)), ((var_nscr_dn9 * var_xgtscr) + (var_nscr * var_xgtscr_dn9)),)
    } else {
        (var_xgtscr0, var_xgtscr0_dn4, var_xgtscr0_dn6, var_xgtscr0_dn7, var_xgtscr0_dn8, var_xgtscr0_dn9,)
    }
};
        var_xgtscr0 = assign41530_e54462;
        var_xgtscr0_dn4 = assign41530_e54462_d_n4;
        var_xgtscr0_dn6 = assign41530_e54462_d_n6;
        var_xgtscr0_dn7 = assign41530_e54462_d_n7;
        var_xgtscr0_dn8 = assign41530_e54462_d_n8;
        var_xgtscr0_dn9 = assign41530_e54462_d_n9;
        var_xgtscr0_rv = 0.0;

        let (assign41540_e54475, assign41540_e54475_d_n4, assign41540_e54475_d_n6, assign41540_e54475_d_n7, assign41540_e54475_d_n8, assign41540_e54475_d_n9,) = {
    if (var_guard1195 != 0.0) {
        let assign41540_e54468: f64 = (var_xgtscr0 * var_xgtscr0);
        let assign41540_e54470: f64 = (assign41540_e54468 + 10.0);
        let assign41540_e54471: f64 = (assign41540_e54470).sqrt();
        let assign41540_e54472: f64 = (var_xgtscr0 + assign41540_e54471);
        let assign41540_e54473: f64 = (0.5 * assign41540_e54472);
        (assign41540_e54473, (0.5 * (var_xgtscr0_dn4 + (((var_xgtscr0_dn4 * var_xgtscr0) + (var_xgtscr0 * var_xgtscr0_dn4)) / (2.0 * assign41540_e54471)))), (0.5 * (var_xgtscr0_dn6 + (((var_xgtscr0_dn6 * var_xgtscr0) + (var_xgtscr0 * var_xgtscr0_dn6)) / (2.0 * assign41540_e54471)))), (0.5 * (var_xgtscr0_dn7 + (((var_xgtscr0_dn7 * var_xgtscr0) + (var_xgtscr0 * var_xgtscr0_dn7)) / (2.0 * assign41540_e54471)))), (0.5 * (var_xgtscr0_dn8 + (((var_xgtscr0_dn8 * var_xgtscr0) + (var_xgtscr0 * var_xgtscr0_dn8)) / (2.0 * assign41540_e54471)))), (0.5 * (var_xgtscr0_dn9 + (((var_xgtscr0_dn9 * var_xgtscr0) + (var_xgtscr0 * var_xgtscr0_dn9)) / (2.0 * assign41540_e54471)))),)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign41540_e54475;
        var_temp__blk949_dn4 = assign41540_e54475_d_n4;
        var_temp__blk949_dn6 = assign41540_e54475_d_n6;
        var_temp__blk949_dn7 = assign41540_e54475_d_n7;
        var_temp__blk949_dn8 = assign41540_e54475_d_n8;
        var_temp__blk949_dn9 = assign41540_e54475_d_n9;
        var_temp__blk949_rv = 0.0;

        let (assign41550_e54482, assign41550_e54482_d_n4, assign41550_e54482_d_n6, assign41550_e54482_d_n7, assign41550_e54482_d_n8, assign41550_e54482_d_n9,) = {
    if (var_guard1195 != 0.0) {
        let assign41550_e54479: f64 = (var_temp__blk949).ln();
        let assign41550_e54480: f64 = (var_xgtscr - assign41550_e54479);
        (assign41550_e54480, (var_xgtscr_dn4 - (var_temp__blk949_dn4 / var_temp__blk949)), (var_xgtscr_dn6 - (var_temp__blk949_dn6 / var_temp__blk949)), (var_xgtscr_dn7 - (var_temp__blk949_dn7 / var_temp__blk949)), (var_xgtscr_dn8 - (var_temp__blk949_dn8 / var_temp__blk949)), (var_xgtscr_dn9 - (var_temp__blk949_dn9 / var_temp__blk949)),)
    } else {
        (var_qiscr0si, var_qiscr0si_dn4, var_qiscr0si_dn6, var_qiscr0si_dn7, var_qiscr0si_dn8, var_qiscr0si_dn9,)
    }
};
        var_qiscr0si = assign41550_e54482;
        var_qiscr0si_dn4 = assign41550_e54482_d_n4;
        var_qiscr0si_dn6 = assign41550_e54482_d_n6;
        var_qiscr0si_dn7 = assign41550_e54482_d_n7;
        var_qiscr0si_dn8 = assign41550_e54482_d_n8;
        var_qiscr0si_dn9 = assign41550_e54482_d_n9;
        var_qiscr0si_rv = 0.0;

        let (assign41560_e54495, assign41560_e54495_d_n4, assign41560_e54495_d_n6, assign41560_e54495_d_n7, assign41560_e54495_d_n8, assign41560_e54495_d_n9,) = {
    if (var_guard1195 != 0.0) {
        let assign41560_e54488: f64 = (var_qiscr0si * var_qiscr0si);
        let assign41560_e54490: f64 = (assign41560_e54488 + 2.0);
        let assign41560_e54491: f64 = (assign41560_e54490).sqrt();
        let assign41560_e54492: f64 = (var_qiscr0si + assign41560_e54491);
        let assign41560_e54493: f64 = (0.5 * assign41560_e54492);
        (assign41560_e54493, (0.5 * (var_qiscr0si_dn4 + (((var_qiscr0si_dn4 * var_qiscr0si) + (var_qiscr0si * var_qiscr0si_dn4)) / (2.0 * assign41560_e54491)))), (0.5 * (var_qiscr0si_dn6 + (((var_qiscr0si_dn6 * var_qiscr0si) + (var_qiscr0si * var_qiscr0si_dn6)) / (2.0 * assign41560_e54491)))), (0.5 * (var_qiscr0si_dn7 + (((var_qiscr0si_dn7 * var_qiscr0si) + (var_qiscr0si * var_qiscr0si_dn7)) / (2.0 * assign41560_e54491)))), (0.5 * (var_qiscr0si_dn8 + (((var_qiscr0si_dn8 * var_qiscr0si) + (var_qiscr0si * var_qiscr0si_dn8)) / (2.0 * assign41560_e54491)))), (0.5 * (var_qiscr0si_dn9 + (((var_qiscr0si_dn9 * var_qiscr0si) + (var_qiscr0si * var_qiscr0si_dn9)) / (2.0 * assign41560_e54491)))),)
    } else {
        (var_qiscr0, var_qiscr0_dn4, var_qiscr0_dn6, var_qiscr0_dn7, var_qiscr0_dn8, var_qiscr0_dn9,)
    }
};
        var_qiscr0 = assign41560_e54495;
        var_qiscr0_dn4 = assign41560_e54495_d_n4;
        var_qiscr0_dn6 = assign41560_e54495_d_n6;
        var_qiscr0_dn7 = assign41560_e54495_d_n7;
        var_qiscr0_dn8 = assign41560_e54495_d_n8;
        var_qiscr0_dn9 = assign41560_e54495_d_n9;
        var_qiscr0_rv = 0.0;

        let assign41570_e54498: f64 = (var_xgtscr - var_qiscr0);
        let assign41570_e54500: f64 = if assign41570_e54498 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard1196 = assign41570_e54500;
        var_guard1196_rv = 0.0;

        let (assign41580_e54509, assign41580_e54509_d_n4, assign41580_e54509_d_n6, assign41580_e54509_d_n7, assign41580_e54509_d_n8, assign41580_e54509_d_n9,) = {
    if ((var_guard1195 != 0.0) && (var_guard1196 != 0.0)) {
        let assign41580_e54506: f64 = (var_xgtscr - var_qiscr0);
        let assign41580_e54507: f64 = (assign41580_e54506).exp();
        (assign41580_e54507, (assign41580_e54507 * (var_xgtscr_dn4 - var_qiscr0_dn4)), (assign41580_e54507 * (var_xgtscr_dn6 - var_qiscr0_dn6)), (assign41580_e54507 * (var_xgtscr_dn7 - var_qiscr0_dn7)), (assign41580_e54507 * (var_xgtscr_dn8 - var_qiscr0_dn8)), (assign41580_e54507 * (var_xgtscr_dn9 - var_qiscr0_dn9)),)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign41580_e54509;
        var_temp__blk949_dn4 = assign41580_e54509_d_n4;
        var_temp__blk949_dn6 = assign41580_e54509_d_n6;
        var_temp__blk949_dn7 = assign41580_e54509_d_n7;
        var_temp__blk949_dn8 = assign41580_e54509_d_n8;
        var_temp__blk949_dn9 = assign41580_e54509_d_n9;
        var_temp__blk949_rv = 0.0;

        let (assign41590_e54544, assign41590_e54544_d_n4, assign41590_e54544_d_n6, assign41590_e54544_d_n7, assign41590_e54544_d_n8, assign41590_e54544_d_n9,) = {
    if ((var_guard1195 != 0.0) && (var_guard1196 == 0.0)) {
        let assign41590_e54518: f64 = (var_xgtscr - var_qiscr0);
        let assign41590_e54520: f64 = (assign41590_e54518 - 230.25850929940458);
        let assign41590_e54525: f64 = (var_xgtscr - var_qiscr0);
        let assign41590_e54527: f64 = (assign41590_e54525 - 230.25850929940458);
        let assign41590_e54531: f64 = (var_xgtscr - var_qiscr0);
        let assign41590_e54533: f64 = (assign41590_e54531 - 230.25850929940458);
        let assign41590_e54535: f64 = (assign41590_e54533 * 0.3333333333333333);
        let assign41590_e54536: f64 = (1.0 + assign41590_e54535);
        let assign41590_e54537: f64 = (assign41590_e54527 * assign41590_e54536);
        let assign41590_e54538: f64 = (0.5 * assign41590_e54537);
        let assign41590_e54539: f64 = (1.0 + assign41590_e54538);
        let assign41590_e54540: f64 = (assign41590_e54520 * assign41590_e54539);
        let assign41590_e54541: f64 = (1.0 + assign41590_e54540);
        let assign41590_e54542: f64 = (1e100 * assign41590_e54541);
        (assign41590_e54542, (1e100 * (((var_xgtscr_dn4 - var_qiscr0_dn4) * assign41590_e54539) + (assign41590_e54520 * (0.5 * (((var_xgtscr_dn4 - var_qiscr0_dn4) * assign41590_e54536) + (assign41590_e54527 * ((var_xgtscr_dn4 - var_qiscr0_dn4) * 0.3333333333333333))))))), (1e100 * (((var_xgtscr_dn6 - var_qiscr0_dn6) * assign41590_e54539) + (assign41590_e54520 * (0.5 * (((var_xgtscr_dn6 - var_qiscr0_dn6) * assign41590_e54536) + (assign41590_e54527 * ((var_xgtscr_dn6 - var_qiscr0_dn6) * 0.3333333333333333))))))), (1e100 * (((var_xgtscr_dn7 - var_qiscr0_dn7) * assign41590_e54539) + (assign41590_e54520 * (0.5 * (((var_xgtscr_dn7 - var_qiscr0_dn7) * assign41590_e54536) + (assign41590_e54527 * ((var_xgtscr_dn7 - var_qiscr0_dn7) * 0.3333333333333333))))))), (1e100 * (((var_xgtscr_dn8 - var_qiscr0_dn8) * assign41590_e54539) + (assign41590_e54520 * (0.5 * (((var_xgtscr_dn8 - var_qiscr0_dn8) * assign41590_e54536) + (assign41590_e54527 * ((var_xgtscr_dn8 - var_qiscr0_dn8) * 0.3333333333333333))))))), (1e100 * (((var_xgtscr_dn9 - var_qiscr0_dn9) * assign41590_e54539) + (assign41590_e54520 * (0.5 * (((var_xgtscr_dn9 - var_qiscr0_dn9) * assign41590_e54536) + (assign41590_e54527 * ((var_xgtscr_dn9 - var_qiscr0_dn9) * 0.3333333333333333))))))),)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign41590_e54544;
        var_temp__blk949_dn4 = assign41590_e54544_d_n4;
        var_temp__blk949_dn6 = assign41590_e54544_d_n6;
        var_temp__blk949_dn7 = assign41590_e54544_d_n7;
        var_temp__blk949_dn8 = assign41590_e54544_d_n8;
        var_temp__blk949_dn9 = assign41590_e54544_d_n9;
        var_temp__blk949_rv = 0.0;

        let (assign41600_e54550, assign41600_e54550_d_n4, assign41600_e54550_d_n6, assign41600_e54550_d_n7, assign41600_e54550_d_n8, assign41600_e54550_d_n9,) = {
    if (var_guard1195 != 0.0) {
        let assign41600_e54548: f64 = (var_temp__blk949 / var_nscr);
        (assign41600_e54548, (((var_temp__blk949_dn4 * var_nscr) - (var_temp__blk949 * var_nscr_dn4)) / (var_nscr * var_nscr)), (((var_temp__blk949_dn6 * var_nscr) - (var_temp__blk949 * var_nscr_dn6)) / (var_nscr * var_nscr)), (((var_temp__blk949_dn7 * var_nscr) - (var_temp__blk949 * var_nscr_dn7)) / (var_nscr * var_nscr)), (((var_temp__blk949_dn8 * var_nscr) - (var_temp__blk949 * var_nscr_dn8)) / (var_nscr * var_nscr)), (((var_temp__blk949_dn9 * var_nscr) - (var_temp__blk949 * var_nscr_dn9)) / (var_nscr * var_nscr)),)
    } else {
        (var_dscr0, var_dscr0_dn4, var_dscr0_dn6, var_dscr0_dn7, var_dscr0_dn8, var_dscr0_dn9,)
    }
};
        var_dscr0 = assign41600_e54550;
        var_dscr0_dn4 = assign41600_e54550_d_n4;
        var_dscr0_dn6 = assign41600_e54550_d_n6;
        var_dscr0_dn7 = assign41600_e54550_d_n7;
        var_dscr0_dn8 = assign41600_e54550_d_n8;
        var_dscr0_dn9 = assign41600_e54550_d_n9;
        var_dscr0_rv = 0.0;

        let (assign41610_e54560, assign41610_e54560_d_n4, assign41610_e54560_d_n6, assign41610_e54560_d_n7, assign41610_e54560_d_n8, assign41610_e54560_d_n9,) = {
    if (var_guard1195 != 0.0) {
        let assign41610_e54555: f64 = (var_qiscr0 + 1.0);
        let assign41610_e54556: f64 = (2.0 * assign41610_e54555);
        let assign41610_e54558: f64 = (assign41610_e54556 - var_dscr0);
        (assign41610_e54558, ((2.0 * var_qiscr0_dn4) - var_dscr0_dn4), ((2.0 * var_qiscr0_dn6) - var_dscr0_dn6), ((2.0 * var_qiscr0_dn7) - var_dscr0_dn7), ((2.0 * var_qiscr0_dn8) - var_dscr0_dn8), ((2.0 * var_qiscr0_dn9) - var_dscr0_dn9),)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign41610_e54560;
        var_temp__blk949_dn4 = assign41610_e54560_d_n4;
        var_temp__blk949_dn6 = assign41610_e54560_d_n6;
        var_temp__blk949_dn7 = assign41610_e54560_d_n7;
        var_temp__blk949_dn8 = assign41610_e54560_d_n8;
        var_temp__blk949_dn9 = assign41610_e54560_d_n9;
        var_temp__blk949_rv = 0.0;

        let assign41620_e54563: f64 = if var_dscr0 > 1e-6 { 1.0 } else { 0.0 };
        var_guard1197 = assign41620_e54563;
        var_guard1197_rv = 0.0;

        let (assign41630_e54584, assign41630_e54584_d_n4, assign41630_e54584_d_n6, assign41630_e54584_d_n7, assign41630_e54584_d_n8, assign41630_e54584_d_n9,) = {
    if ((var_guard1195 != 0.0) && (var_guard1197 != 0.0)) {
        let assign41630_e54572: f64 = (var_dscr0 * var_temp__blk949);
        let assign41630_e54573: f64 = (1.0 + assign41630_e54572);
        let assign41630_e54574: f64 = (assign41630_e54573).sqrt();
        let assign41630_e54576: f64 = (assign41630_e54574 - 1.0);
        let assign41630_e54578: f64 = (assign41630_e54576 / var_dscr0);
        let assign41630_e54579: f64 = (var_qiscr0 - assign41630_e54578);
        let assign41630_e54581: f64 = (assign41630_e54579 + 1.0);
        let assign41630_e54582: f64 = (var_nscr * assign41630_e54581);
        (assign41630_e54582, ((var_nscr_dn4 * assign41630_e54581) + (var_nscr * (var_qiscr0_dn4 - ((((((var_dscr0_dn4 * var_temp__blk949) + (var_dscr0 * var_temp__blk949_dn4)) / (2.0 * assign41630_e54574)) * var_dscr0) - (assign41630_e54576 * var_dscr0_dn4)) / (var_dscr0 * var_dscr0))))), ((var_nscr_dn6 * assign41630_e54581) + (var_nscr * (var_qiscr0_dn6 - ((((((var_dscr0_dn6 * var_temp__blk949) + (var_dscr0 * var_temp__blk949_dn6)) / (2.0 * assign41630_e54574)) * var_dscr0) - (assign41630_e54576 * var_dscr0_dn6)) / (var_dscr0 * var_dscr0))))), ((var_nscr_dn7 * assign41630_e54581) + (var_nscr * (var_qiscr0_dn7 - ((((((var_dscr0_dn7 * var_temp__blk949) + (var_dscr0 * var_temp__blk949_dn7)) / (2.0 * assign41630_e54574)) * var_dscr0) - (assign41630_e54576 * var_dscr0_dn7)) / (var_dscr0 * var_dscr0))))), ((var_nscr_dn8 * assign41630_e54581) + (var_nscr * (var_qiscr0_dn8 - ((((((var_dscr0_dn8 * var_temp__blk949) + (var_dscr0 * var_temp__blk949_dn8)) / (2.0 * assign41630_e54574)) * var_dscr0) - (assign41630_e54576 * var_dscr0_dn8)) / (var_dscr0 * var_dscr0))))), ((var_nscr_dn9 * assign41630_e54581) + (var_nscr * (var_qiscr0_dn9 - ((((((var_dscr0_dn9 * var_temp__blk949) + (var_dscr0 * var_temp__blk949_dn9)) / (2.0 * assign41630_e54574)) * var_dscr0) - (assign41630_e54576 * var_dscr0_dn9)) / (var_dscr0 * var_dscr0))))),)
    } else {
        (var_qiscr, var_qiscr_dn4, var_qiscr_dn6, var_qiscr_dn7, var_qiscr_dn8, var_qiscr_dn9,)
    }
};
        var_qiscr = assign41630_e54584;
        var_qiscr_dn4 = assign41630_e54584_d_n4;
        var_qiscr_dn6 = assign41630_e54584_d_n6;
        var_qiscr_dn7 = assign41630_e54584_d_n7;
        var_qiscr_dn8 = assign41630_e54584_d_n8;
        var_qiscr_dn9 = assign41630_e54584_d_n9;
        var_qiscr_rv = 0.0;

        let (assign41640_e54603, assign41640_e54603_d_n4, assign41640_e54603_d_n6, assign41640_e54603_d_n7, assign41640_e54603_d_n8, assign41640_e54603_d_n9,) = {
    if ((var_guard1195 != 0.0) && (var_guard1197 == 0.0)) {
        let assign41640_e54591: f64 = (var_nscr * 0.5);
        let assign41640_e54593: f64 = (assign41640_e54591 * var_dscr0);
        let assign41640_e54597: f64 = (0.25 * var_temp__blk949);
        let assign41640_e54599: f64 = (assign41640_e54597 * var_temp__blk949);
        let assign41640_e54600: f64 = (1.0 + assign41640_e54599);
        let assign41640_e54601: f64 = (assign41640_e54593 * assign41640_e54600);
        (assign41640_e54601, (((((var_nscr_dn4 * 0.5) * var_dscr0) + (assign41640_e54591 * var_dscr0_dn4)) * assign41640_e54600) + (assign41640_e54593 * (((0.25 * var_temp__blk949_dn4) * var_temp__blk949) + (assign41640_e54597 * var_temp__blk949_dn4)))), (((((var_nscr_dn6 * 0.5) * var_dscr0) + (assign41640_e54591 * var_dscr0_dn6)) * assign41640_e54600) + (assign41640_e54593 * (((0.25 * var_temp__blk949_dn6) * var_temp__blk949) + (assign41640_e54597 * var_temp__blk949_dn6)))), (((((var_nscr_dn7 * 0.5) * var_dscr0) + (assign41640_e54591 * var_dscr0_dn7)) * assign41640_e54600) + (assign41640_e54593 * (((0.25 * var_temp__blk949_dn7) * var_temp__blk949) + (assign41640_e54597 * var_temp__blk949_dn7)))), (((((var_nscr_dn8 * 0.5) * var_dscr0) + (assign41640_e54591 * var_dscr0_dn8)) * assign41640_e54600) + (assign41640_e54593 * (((0.25 * var_temp__blk949_dn8) * var_temp__blk949) + (assign41640_e54597 * var_temp__blk949_dn8)))), (((((var_nscr_dn9 * 0.5) * var_dscr0) + (assign41640_e54591 * var_dscr0_dn9)) * assign41640_e54600) + (assign41640_e54593 * (((0.25 * var_temp__blk949_dn9) * var_temp__blk949) + (assign41640_e54597 * var_temp__blk949_dn9)))),)
    } else {
        (var_qiscr, var_qiscr_dn4, var_qiscr_dn6, var_qiscr_dn7, var_qiscr_dn8, var_qiscr_dn9,)
    }
};
        var_qiscr = assign41640_e54603;
        var_qiscr_dn4 = assign41640_e54603_d_n4;
        var_qiscr_dn6 = assign41640_e54603_d_n6;
        var_qiscr_dn7 = assign41640_e54603_d_n7;
        var_qiscr_dn8 = assign41640_e54603_d_n8;
        var_qiscr_dn9 = assign41640_e54603_d_n9;
        var_qiscr_rv = 0.0;

        let (assign41650_e54628, assign41650_e54628_d_n4, assign41650_e54628_d_n6, assign41650_e54628_d_n7, assign41650_e54628_d_n8, assign41650_e54628_d_n9,) = {
    if (var_guard1195 != 0.0) {
        let assign41650_e54608: f64 = (var_xg - var_qiscr);
        let assign41650_e54610: f64 = (assign41650_e54608 + 2.0);
        let assign41650_e54613: f64 = (var_xg - var_qiscr);
        let assign41650_e54615: f64 = (assign41650_e54613 - 2.0);
        let assign41650_e54618: f64 = (var_xg - var_qiscr);
        let assign41650_e54620: f64 = (assign41650_e54618 - 2.0);
        let assign41650_e54621: f64 = (assign41650_e54615 * assign41650_e54620);
        let assign41650_e54623: f64 = (assign41650_e54621 + 1.0);
        let assign41650_e54624: f64 = (assign41650_e54623).sqrt();
        let assign41650_e54625: f64 = (assign41650_e54610 + assign41650_e54624);
        let assign41650_e54626: f64 = (0.5 * assign41650_e54625);
        (assign41650_e54626, (0.5 * ((var_xg_dn4 - var_qiscr_dn4) + ((((var_xg_dn4 - var_qiscr_dn4) * assign41650_e54620) + (assign41650_e54615 * (var_xg_dn4 - var_qiscr_dn4))) / (2.0 * assign41650_e54624)))), (0.5 * ((var_xg_dn6 - var_qiscr_dn6) + ((((var_xg_dn6 - var_qiscr_dn6) * assign41650_e54620) + (assign41650_e54615 * (var_xg_dn6 - var_qiscr_dn6))) / (2.0 * assign41650_e54624)))), (0.5 * ((var_xg_dn7 - var_qiscr_dn7) + ((((var_xg_dn7 - var_qiscr_dn7) * assign41650_e54620) + (assign41650_e54615 * (var_xg_dn7 - var_qiscr_dn7))) / (2.0 * assign41650_e54624)))), (0.5 * ((var_xg_dn8 - var_qiscr_dn8) + ((((var_xg_dn8 - var_qiscr_dn8) * assign41650_e54620) + (assign41650_e54615 * (var_xg_dn8 - var_qiscr_dn8))) / (2.0 * assign41650_e54624)))), (0.5 * ((var_xg_dn9 - var_qiscr_dn9) + ((((var_xg_dn9 - var_qiscr_dn9) * assign41650_e54620) + (assign41650_e54615 * (var_xg_dn9 - var_qiscr_dn9))) / (2.0 * assign41650_e54624)))),)
    } else {
        (var_temp__blk949, var_temp__blk949_dn4, var_temp__blk949_dn6, var_temp__blk949_dn7, var_temp__blk949_dn8, var_temp__blk949_dn9,)
    }
};
        var_temp__blk949 = assign41650_e54628;
        var_temp__blk949_dn4 = assign41650_e54628_d_n4;
        var_temp__blk949_dn6 = assign41650_e54628_d_n6;
        var_temp__blk949_dn7 = assign41650_e54628_d_n7;
        var_temp__blk949_dn8 = assign41650_e54628_d_n8;
        var_temp__blk949_dn9 = assign41650_e54628_d_n9;
        var_temp__blk949_rv = 0.0;

        let (assign41660_e54645, assign41660_e54645_d_n4, assign41660_e54645_d_n6, assign41660_e54645_d_n7, assign41660_e54645_d_n8, assign41660_e54645_d_n9,) = {
    if (var_guard1195 != 0.0) {
        let assign41660_e54632: f64 = (0.5 * var_gf2);
        let assign41660_e54636: f64 = (4.0 / var_gf2);
        let assign41660_e54638: f64 = (assign41660_e54636 * var_temp__blk949);
        let assign41660_e54639: f64 = (1.0 + assign41660_e54638);
        let assign41660_e54640: f64 = (assign41660_e54639).sqrt();
        let assign41660_e54642: f64 = (assign41660_e54640 - 1.0);
        let assign41660_e54643: f64 = (assign41660_e54632 * assign41660_e54642);
        (assign41660_e54643, (((0.5 * var_gf2_dn4) * assign41660_e54642) + (assign41660_e54632 * ((((-((4.0 * var_gf2_dn4) / (var_gf2 * var_gf2))) * var_temp__blk949) + (assign41660_e54636 * var_temp__blk949_dn4)) / (2.0 * assign41660_e54640)))), (((0.5 * var_gf2_dn6) * assign41660_e54642) + (assign41660_e54632 * ((((-((4.0 * var_gf2_dn6) / (var_gf2 * var_gf2))) * var_temp__blk949) + (assign41660_e54636 * var_temp__blk949_dn6)) / (2.0 * assign41660_e54640)))), (((0.5 * var_gf2_dn7) * assign41660_e54642) + (assign41660_e54632 * ((((-((4.0 * var_gf2_dn7) / (var_gf2 * var_gf2))) * var_temp__blk949) + (assign41660_e54636 * var_temp__blk949_dn7)) / (2.0 * assign41660_e54640)))), (((0.5 * var_gf2_dn8) * assign41660_e54642) + (assign41660_e54632 * ((((-((4.0 * var_gf2_dn8) / (var_gf2 * var_gf2))) * var_temp__blk949) + (assign41660_e54636 * var_temp__blk949_dn8)) / (2.0 * assign41660_e54640)))), (((0.5 * var_gf2_dn9) * assign41660_e54642) + (assign41660_e54632 * ((((-((4.0 * var_gf2_dn9) / (var_gf2 * var_gf2))) * var_temp__blk949) + (assign41660_e54636 * var_temp__blk949_dn9)) / (2.0 * assign41660_e54640)))),)
    } else {
        (var_qbscr, var_qbscr_dn4, var_qbscr_dn6, var_qbscr_dn7, var_qbscr_dn8, var_qbscr_dn9,)
    }
};
        var_qbscr = assign41660_e54645;
        var_qbscr_dn4 = assign41660_e54645_d_n4;
        var_qbscr_dn6 = assign41660_e54645_d_n6;
        var_qbscr_dn7 = assign41660_e54645_d_n7;
        var_qbscr_dn8 = assign41660_e54645_d_n8;
        var_qbscr_dn9 = assign41660_e54645_d_n9;
        var_qbscr_rv = 0.0;

        let (assign41670_e54653, assign41670_e54653_d_n4, assign41670_e54653_d_n6, assign41670_e54653_d_n7, assign41670_e54653_d_n8, assign41670_e54653_d_n9,) = {
    if (var_guard1195 != 0.0) {
        let assign41670_e54650: f64 = (var_qbscr + var_qiscr);
        let assign41670_e54651: f64 = (var_qbscr / assign41670_e54650);
        (assign41670_e54651, (((var_qbscr_dn4 * assign41670_e54650) - (var_qbscr * (var_qbscr_dn4 + var_qiscr_dn4))) / (assign41670_e54650 * assign41670_e54650)), (((var_qbscr_dn6 * assign41670_e54650) - (var_qbscr * (var_qbscr_dn6 + var_qiscr_dn6))) / (assign41670_e54650 * assign41670_e54650)), (((var_qbscr_dn7 * assign41670_e54650) - (var_qbscr * (var_qbscr_dn7 + var_qiscr_dn7))) / (assign41670_e54650 * assign41670_e54650)), (((var_qbscr_dn8 * assign41670_e54650) - (var_qbscr * (var_qbscr_dn8 + var_qiscr_dn8))) / (assign41670_e54650 * assign41670_e54650)), (((var_qbscr_dn9 * assign41670_e54650) - (var_qbscr * (var_qbscr_dn9 + var_qiscr_dn9))) / (assign41670_e54650 * assign41670_e54650)),)
    } else {
        (var_fscr, var_fscr_dn4, var_fscr_dn6, var_fscr_dn7, var_fscr_dn8, var_fscr_dn9,)
    }
};
        var_fscr = assign41670_e54653;
        var_fscr_dn4 = assign41670_e54653_d_n4;
        var_fscr_dn6 = assign41670_e54653_d_n6;
        var_fscr_dn7 = assign41670_e54653_d_n7;
        var_fscr_dn8 = assign41670_e54653_d_n8;
        var_fscr_dn9 = assign41670_e54653_d_n9;
        var_fscr_rv = 0.0;

        let (assign41680_e54661, assign41680_e54661_d_n4, assign41680_e54661_d_n6, assign41680_e54661_d_n7, assign41680_e54661_d_n8, assign41680_e54661_d_n9,) = {
    if (var_guard1195 != 0.0) {
        let assign41680_e54658: f64 = (var_fscr * var_delxb);
        let assign41680_e54659: f64 = (var_xno_s - assign41680_e54658);
        (assign41680_e54659, (var_xno_s_dn4 - ((var_fscr_dn4 * var_delxb) + (var_fscr * var_delxb_dn4))), (var_xno_s_dn6 - ((var_fscr_dn6 * var_delxb) + (var_fscr * var_delxb_dn6))), (var_xno_s_dn7 - ((var_fscr_dn7 * var_delxb) + (var_fscr * var_delxb_dn7))), (var_xno_s_dn8 - ((var_fscr_dn8 * var_delxb) + (var_fscr * var_delxb_dn8))), (var_xno_s_dn9 - ((var_fscr_dn9 * var_delxb) + (var_fscr * var_delxb_dn9))),)
    } else {
        (var_xn_s, var_xn_s_dn4, var_xn_s_dn6, var_xn_s_dn7, var_xn_s_dn8, var_xn_s_dn9,)
    }
};
        var_xn_s = assign41680_e54661;
        var_xn_s_dn4 = assign41680_e54661_d_n4;
        var_xn_s_dn6 = assign41680_e54661_d_n6;
        var_xn_s_dn7 = assign41680_e54661_d_n7;
        var_xn_s_dn8 = assign41680_e54661_d_n8;
        var_xn_s_dn9 = assign41680_e54661_d_n9;
        var_xn_s_rv = 0.0;

        let assign41690_e54665: f64 = (var_gf * 0.7071067811865475);
        let assign41690_e54666: f64 = (1.0 + assign41690_e54665);
        var_xi = assign41690_e54666;
        var_xi_dn4 = (var_gf_dn4 * 0.7071067811865475);
        var_xi_dn6 = (var_gf_dn6 * 0.7071067811865475);
        var_xi_dn7 = (var_gf_dn7 * 0.7071067811865475);
        var_xi_dn8 = (var_gf_dn8 * 0.7071067811865475);
        var_xi_dn9 = (var_gf_dn9 * 0.7071067811865475);
        var_xi_rv = 0.0;

        *var_delta_ns_slot = var_delta_ns;
        *var_delta_ns_dn4_slot = var_delta_ns_dn4;
        *var_delta_ns_dn6_slot = var_delta_ns_dn6;
        *var_delta_ns_dn7_slot = var_delta_ns_dn7;
        *var_delta_ns_dn8_slot = var_delta_ns_dn8;
        *var_delta_ns_dn9_slot = var_delta_ns_dn9;
        *var_delta_ns_rv_slot = var_delta_ns_rv;
        *var_dscr0_slot = var_dscr0;
        *var_dscr0_dn4_slot = var_dscr0_dn4;
        *var_dscr0_dn6_slot = var_dscr0_dn6;
        *var_dscr0_dn7_slot = var_dscr0_dn7;
        *var_dscr0_dn8_slot = var_dscr0_dn8;
        *var_dscr0_dn9_slot = var_dscr0_dn9;
        *var_dscr0_rv_slot = var_dscr0_rv;
        *var_fscr_slot = var_fscr;
        *var_fscr_dn4_slot = var_fscr_dn4;
        *var_fscr_dn6_slot = var_fscr_dn6;
        *var_fscr_dn7_slot = var_fscr_dn7;
        *var_fscr_dn8_slot = var_fscr_dn8;
        *var_fscr_dn9_slot = var_fscr_dn9;
        *var_fscr_rv_slot = var_fscr_rv;
        *var_guard1192_slot = var_guard1192;
        *var_guard1192_rv_slot = var_guard1192_rv;
        *var_guard1193_slot = var_guard1193;
        *var_guard1193_rv_slot = var_guard1193_rv;
        *var_guard1194_slot = var_guard1194;
        *var_guard1194_rv_slot = var_guard1194_rv;
        *var_guard1195_slot = var_guard1195;
        *var_guard1195_rv_slot = var_guard1195_rv;
        *var_guard1196_slot = var_guard1196;
        *var_guard1196_rv_slot = var_guard1196_rv;
        *var_guard1197_slot = var_guard1197;
        *var_guard1197_rv_slot = var_guard1197_rv;
        *var_nscr_slot = var_nscr;
        *var_nscr_dn4_slot = var_nscr_dn4;
        *var_nscr_dn6_slot = var_nscr_dn6;
        *var_nscr_dn7_slot = var_nscr_dn7;
        *var_nscr_dn8_slot = var_nscr_dn8;
        *var_nscr_dn9_slot = var_nscr_dn9;
        *var_nscr_rv_slot = var_nscr_rv;
        *var_qbscr_slot = var_qbscr;
        *var_qbscr_dn4_slot = var_qbscr_dn4;
        *var_qbscr_dn6_slot = var_qbscr_dn6;
        *var_qbscr_dn7_slot = var_qbscr_dn7;
        *var_qbscr_dn8_slot = var_qbscr_dn8;
        *var_qbscr_dn9_slot = var_qbscr_dn9;
        *var_qbscr_rv_slot = var_qbscr_rv;
        *var_qiscr_slot = var_qiscr;
        *var_qiscr0_slot = var_qiscr0;
        *var_qiscr0_dn4_slot = var_qiscr0_dn4;
        *var_qiscr0_dn6_slot = var_qiscr0_dn6;
        *var_qiscr0_dn7_slot = var_qiscr0_dn7;
        *var_qiscr0_dn8_slot = var_qiscr0_dn8;
        *var_qiscr0_dn9_slot = var_qiscr0_dn9;
        *var_qiscr0_rv_slot = var_qiscr0_rv;
        *var_qiscr0si_slot = var_qiscr0si;
        *var_qiscr0si_dn4_slot = var_qiscr0si_dn4;
        *var_qiscr0si_dn6_slot = var_qiscr0si_dn6;
        *var_qiscr0si_dn7_slot = var_qiscr0si_dn7;
        *var_qiscr0si_dn8_slot = var_qiscr0si_dn8;
        *var_qiscr0si_dn9_slot = var_qiscr0si_dn9;
        *var_qiscr0si_rv_slot = var_qiscr0si_rv;
        *var_qiscr_dn4_slot = var_qiscr_dn4;
        *var_qiscr_dn6_slot = var_qiscr_dn6;
        *var_qiscr_dn7_slot = var_qiscr_dn7;
        *var_qiscr_dn8_slot = var_qiscr_dn8;
        *var_qiscr_dn9_slot = var_qiscr_dn9;
        *var_qiscr_rv_slot = var_qiscr_rv;
        *var_temp__blk949_slot = var_temp__blk949;
        *var_temp__blk949_dn4_slot = var_temp__blk949_dn4;
        *var_temp__blk949_dn6_slot = var_temp__blk949_dn6;
        *var_temp__blk949_dn7_slot = var_temp__blk949_dn7;
        *var_temp__blk949_dn8_slot = var_temp__blk949_dn8;
        *var_temp__blk949_dn9_slot = var_temp__blk949_dn9;
        *var_temp__blk949_rv_slot = var_temp__blk949_rv;
        *var_xgtscr_slot = var_xgtscr;
        *var_xgtscr0_slot = var_xgtscr0;
        *var_xgtscr0_dn4_slot = var_xgtscr0_dn4;
        *var_xgtscr0_dn6_slot = var_xgtscr0_dn6;
        *var_xgtscr0_dn7_slot = var_xgtscr0_dn7;
        *var_xgtscr0_dn8_slot = var_xgtscr0_dn8;
        *var_xgtscr0_dn9_slot = var_xgtscr0_dn9;
        *var_xgtscr0_rv_slot = var_xgtscr0_rv;
        *var_xgtscr_dn4_slot = var_xgtscr_dn4;
        *var_xgtscr_dn6_slot = var_xgtscr_dn6;
        *var_xgtscr_dn7_slot = var_xgtscr_dn7;
        *var_xgtscr_dn8_slot = var_xgtscr_dn8;
        *var_xgtscr_dn9_slot = var_xgtscr_dn9;
        *var_xgtscr_rv_slot = var_xgtscr_rv;
        *var_xi_slot = var_xi;
        *var_xi_dn4_slot = var_xi_dn4;
        *var_xi_dn6_slot = var_xi_dn6;
        *var_xi_dn7_slot = var_xi_dn7;
        *var_xi_dn8_slot = var_xi_dn8;
        *var_xi_dn9_slot = var_xi_dn9;
        *var_xi_rv_slot = var_xi_rv;
        *var_xn_s_slot = var_xn_s;
        *var_xn_s_dn4_slot = var_xn_s_dn4;
        *var_xn_s_dn6_slot = var_xn_s_dn6;
        *var_xn_s_dn7_slot = var_xn_s_dn7;
        *var_xn_s_dn8_slot = var_xn_s_dn8;
        *var_xn_s_dn9_slot = var_xn_s_dn9;
        *var_xn_s_rv_slot = var_xn_s_rv;
        *var_xthscr_slot = var_xthscr;
        *var_xthscr_dn4_slot = var_xthscr_dn4;
        *var_xthscr_dn6_slot = var_xthscr_dn6;
        *var_xthscr_dn7_slot = var_xthscr_dn7;
        *var_xthscr_dn8_slot = var_xthscr_dn8;
        *var_xthscr_dn9_slot = var_xthscr_dn9;
        *var_xthscr_rv_slot = var_xthscr_rv;
    }
}

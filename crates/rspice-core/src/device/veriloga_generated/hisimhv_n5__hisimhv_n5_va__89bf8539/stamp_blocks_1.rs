#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10590_e5594,) = {
    if ((p.p133 != 0.0) || (p.p134 != 0.0)) {
        (1.0,)
    } else {
        (0.0,)
    }
};
        locals.var_flg_qy = assign10590_e5594;

        let assign10610_e5608: f64 = if (((p.p235 == 0.0) && (p.p237 == 0.0)) || (p.p236 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard252 = assign10610_e5608;

        let (assign10620_e5612,) = {
    if (locals.var_guard252 != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_qmetemp,)
    }
};
        locals.var_flg_qmetemp = assign10620_e5612;

        let (assign10630_e5617,) = {
    if (locals.var_guard252 == 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_qmetemp,)
    }
};
        locals.var_flg_qmetemp = assign10630_e5617;

        let assign10640_e5620: f64 = (locals.var_wg * locals.var_lg);
        locals.var_wlg = assign10640_e5620;

        let assign10650_e5623: f64 = (p.p289 * 1000000.0);
        locals.var_uc_gdld = assign10650_e5623;

        let assign10660_e5629: f64 = (locals.var_ktnom * 1e-7);
        let assign10660_e5630: f64 = (9.025e-5 + assign10660_e5629);
        let assign10660_e5631: f64 = (locals.var_ktnom * assign10660_e5630);
        let assign10660_e5632: f64 = (locals.var_uc_eg0 - assign10660_e5631);
        locals.var_egtnom = assign10660_e5632;

        let assign10670_e5635: f64 = (8.8541878e-12 * p.p267);
        locals.var_cecox = assign10670_e5635;

        locals.var_msc = locals.var_uc_scp22;

        let assign10690_e5639: f64 = if locals.var_uc_pgd1 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard253 = assign10690_e5639;

        let (assign10700_e5643,) = {
    if (locals.var_guard253 != 0.0) {
        (0.0,)
    } else {
        (locals.var_flg_pgd,)
    }
};
        locals.var_flg_pgd = assign10700_e5643;

        let (assign10710_e5647,) = {
    if (locals.var_guard253 != 0.0) {
        (0.0,)
    } else {
        (locals.var_cnstpgd,)
    }
};
        locals.var_cnstpgd = assign10710_e5647;

        let (assign10720_e5652,) = {
    if (locals.var_guard253 == 0.0) {
        (1.0,)
    } else {
        (locals.var_flg_pgd,)
    }
};
        locals.var_flg_pgd = assign10720_e5652;

        let (assign10730_e5665,) = {
    if (locals.var_guard253 == 0.0) {
        let assign10730_e5658: f64 = (1.0 / locals.var_lg);
        let assign10730_e5659: f64 = (1.0 + assign10730_e5658);
        let assign10730_e5661: f64 = (assign10730_e5659).powf(p.p153);
        let assign10730_e5663: f64 = (assign10730_e5661 * locals.var_uc_pgd1);
        (assign10730_e5663,)
    } else {
        (locals.var_cnstpgd,)
    }
};
        locals.var_cnstpgd = assign10730_e5665;

        let assign10740_e5669: f64 = (locals.var_lg).powf(p.p229);
        let assign10740_e5671: f64 = (assign10740_e5669 * p.p230);
        let assign10740_e5672: f64 = (1.0 + assign10740_e5671);
        locals.var_clmmod = assign10740_e5672;

        let assign10750_e5677: f64 = (0.5 * p.p0);
        let assign10750_e5678: f64 = (p.p118 + assign10750_e5677);
        let assign10750_e5679: f64 = (1.0 / assign10750_e5678);
        let assign10750_e5684: f64 = (0.5 * p.p0);
        let assign10750_e5685: f64 = (p.p119 + assign10750_e5684);
        let assign10750_e5686: f64 = (1.0 / assign10750_e5685);
        let assign10750_e5687: f64 = (assign10750_e5679 + assign10750_e5686);
        locals.var_t1 = assign10750_e5687;
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

        let assign10760_e5690: f64 = (2.0 / locals.var_t1);
        locals.var_lod_half_ref = assign10760_e5690;
        locals.var_lod_half_ref_dn0 = (-((2.0 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn2 = (-((2.0 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn4 = (-((2.0 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn5 = (-((2.0 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn6 = (-((2.0 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn7 = (-((2.0 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn8 = (-((2.0 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn9 = (-((2.0 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn10 = (-((2.0 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn11 = (-((2.0 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1)));
        locals.var_lod_half_ref_dn14 = (-((2.0 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1)));

        let assign10770_e5709: f64 = if (((p.p8 > 0.0) && (p.p9 > 0.0)) && ((p.p7 == 1.0) || ((p.p7 > 1.0) && (p.p10 > 0.0)))) { 1.0 } else { 0.0 };
        locals.var_guard254 = assign10770_e5709;

        let (assign10780_e5713, assign10780_e5713_d_n0, assign10780_e5713_d_n2, assign10780_e5713_d_n4, assign10780_e5713_d_n5, assign10780_e5713_d_n6, assign10780_e5713_d_n7, assign10780_e5713_d_n8, assign10780_e5713_d_n9, assign10780_e5713_d_n10, assign10780_e5713_d_n11, assign10780_e5713_d_n14,) = {
    if (locals.var_guard254 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign10780_e5713;
        locals.var_t1_dn0 = assign10780_e5713_d_n0;
        locals.var_t1_dn2 = assign10780_e5713_d_n2;
        locals.var_t1_dn4 = assign10780_e5713_d_n4;
        locals.var_t1_dn5 = assign10780_e5713_d_n5;
        locals.var_t1_dn6 = assign10780_e5713_d_n6;
        locals.var_t1_dn7 = assign10780_e5713_d_n7;
        locals.var_t1_dn8 = assign10780_e5713_d_n8;
        locals.var_t1_dn9 = assign10780_e5713_d_n9;
        locals.var_t1_dn10 = assign10780_e5713_d_n10;
        locals.var_t1_dn11 = assign10780_e5713_d_n11;
        locals.var_t1_dn14 = assign10780_e5713_d_n14;

        let (assign10790_e5717,) = {
    if (locals.var_guard254 != 0.0) {
        (0.0,)
    } else {
        (locals.var_i,)
    }
};
        locals.var_i = assign10790_e5717;

        let mut assign10800_loop_guard: usize = 0;
        while {
            let assign10800_cond_e5722: f64 = if ((locals.var_guard254 != 0.0) && (locals.var_i < p.p7)) { 1.0 } else { 0.0 };
            assign10800_cond_e5722 != 0.0
        } {
            assign10800_loop_guard += 1;
            assert!(assign10800_loop_guard <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            let (assign10800_body0_e5754, assign10800_body0_e5754_d_n0, assign10800_body0_e5754_d_n2, assign10800_body0_e5754_d_n4, assign10800_body0_e5754_d_n5, assign10800_body0_e5754_d_n6, assign10800_body0_e5754_d_n7, assign10800_body0_e5754_d_n8, assign10800_body0_e5754_d_n9, assign10800_body0_e5754_d_n10, assign10800_body0_e5754_d_n11, assign10800_body0_e5754_d_n14,) = {
    if (locals.var_guard254 != 0.0) {
        let assign10800_body0_e5729: f64 = (0.5 * p.p0);
        let assign10800_body0_e5730: f64 = (p.p8 + assign10800_body0_e5729);
        let assign10800_body0_e5734: f64 = (p.p10 + p.p0);
        let assign10800_body0_e5735: f64 = (locals.var_i * assign10800_body0_e5734);
        let assign10800_body0_e5736: f64 = (assign10800_body0_e5730 + assign10800_body0_e5735);
        let assign10800_body0_e5737: f64 = (1.0 / assign10800_body0_e5736);
        let assign10800_body0_e5738: f64 = (locals.var_t1 + assign10800_body0_e5737);
        let assign10800_body0_e5743: f64 = (0.5 * p.p0);
        let assign10800_body0_e5744: f64 = (p.p9 + assign10800_body0_e5743);
        let assign10800_body0_e5748: f64 = (p.p10 + p.p0);
        let assign10800_body0_e5749: f64 = (locals.var_i * assign10800_body0_e5748);
        let assign10800_body0_e5750: f64 = (assign10800_body0_e5744 + assign10800_body0_e5749);
        let assign10800_body0_e5751: f64 = (1.0 / assign10800_body0_e5750);
        let assign10800_body0_e5752: f64 = (assign10800_body0_e5738 + assign10800_body0_e5751);
        (assign10800_body0_e5752, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
            locals.var_t1 = assign10800_body0_e5754;
            locals.var_t1_dn0 = assign10800_body0_e5754_d_n0;
            locals.var_t1_dn2 = assign10800_body0_e5754_d_n2;
            locals.var_t1_dn4 = assign10800_body0_e5754_d_n4;
            locals.var_t1_dn5 = assign10800_body0_e5754_d_n5;
            locals.var_t1_dn6 = assign10800_body0_e5754_d_n6;
            locals.var_t1_dn7 = assign10800_body0_e5754_d_n7;
            locals.var_t1_dn8 = assign10800_body0_e5754_d_n8;
            locals.var_t1_dn9 = assign10800_body0_e5754_d_n9;
            locals.var_t1_dn10 = assign10800_body0_e5754_d_n10;
            locals.var_t1_dn11 = assign10800_body0_e5754_d_n11;
            locals.var_t1_dn14 = assign10800_body0_e5754_d_n14;
            let (assign10800_body1_e5760,) = {
    if (locals.var_guard254 != 0.0) {
        let assign10800_body1_e5758: f64 = (locals.var_i + 1.0);
        (assign10800_body1_e5758,)
    } else {
        (locals.var_i,)
    }
};
            locals.var_i = assign10800_body1_e5760;
        }

        let (assign10810_e5768, assign10810_e5768_d_n0, assign10810_e5768_d_n2, assign10810_e5768_d_n4, assign10810_e5768_d_n5, assign10810_e5768_d_n6, assign10810_e5768_d_n7, assign10810_e5768_d_n8, assign10810_e5768_d_n9, assign10810_e5768_d_n10, assign10810_e5768_d_n11, assign10810_e5768_d_n14,) = {
    if (locals.var_guard254 != 0.0) {
        let assign10810_e5764: f64 = (2.0 * p.p7);
        let assign10810_e5766: f64 = (assign10810_e5764 / locals.var_t1);
        (assign10810_e5766, (-((assign10810_e5764 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((assign10810_e5764 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((assign10810_e5764 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((assign10810_e5764 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((assign10810_e5764 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((assign10810_e5764 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((assign10810_e5764 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((assign10810_e5764 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((assign10810_e5764 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((assign10810_e5764 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((assign10810_e5764 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_lod_half, locals.var_lod_half_dn0, locals.var_lod_half_dn2, locals.var_lod_half_dn4, locals.var_lod_half_dn5, locals.var_lod_half_dn6, locals.var_lod_half_dn7, locals.var_lod_half_dn8, locals.var_lod_half_dn9, locals.var_lod_half_dn10, locals.var_lod_half_dn11, locals.var_lod_half_dn14,)
    }
};
        locals.var_lod_half = assign10810_e5768;
        locals.var_lod_half_dn0 = assign10810_e5768_d_n0;
        locals.var_lod_half_dn2 = assign10810_e5768_d_n2;
        locals.var_lod_half_dn4 = assign10810_e5768_d_n4;
        locals.var_lod_half_dn5 = assign10810_e5768_d_n5;
        locals.var_lod_half_dn6 = assign10810_e5768_d_n6;
        locals.var_lod_half_dn7 = assign10810_e5768_d_n7;
        locals.var_lod_half_dn8 = assign10810_e5768_d_n8;
        locals.var_lod_half_dn9 = assign10810_e5768_d_n9;
        locals.var_lod_half_dn10 = assign10810_e5768_d_n10;
        locals.var_lod_half_dn11 = assign10810_e5768_d_n11;
        locals.var_lod_half_dn14 = assign10810_e5768_d_n14;

        let (assign10820_e5773, assign10820_e5773_d_n0, assign10820_e5773_d_n2, assign10820_e5773_d_n4, assign10820_e5773_d_n5, assign10820_e5773_d_n6, assign10820_e5773_d_n7, assign10820_e5773_d_n8, assign10820_e5773_d_n9, assign10820_e5773_d_n10, assign10820_e5773_d_n11, assign10820_e5773_d_n14,) = {
    if (locals.var_guard254 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_lod_half, locals.var_lod_half_dn0, locals.var_lod_half_dn2, locals.var_lod_half_dn4, locals.var_lod_half_dn5, locals.var_lod_half_dn6, locals.var_lod_half_dn7, locals.var_lod_half_dn8, locals.var_lod_half_dn9, locals.var_lod_half_dn10, locals.var_lod_half_dn11, locals.var_lod_half_dn14,)
    }
};
        locals.var_lod_half = assign10820_e5773;
        locals.var_lod_half_dn0 = assign10820_e5773_d_n0;
        locals.var_lod_half_dn2 = assign10820_e5773_d_n2;
        locals.var_lod_half_dn4 = assign10820_e5773_d_n4;
        locals.var_lod_half_dn5 = assign10820_e5773_d_n5;
        locals.var_lod_half_dn6 = assign10820_e5773_d_n6;
        locals.var_lod_half_dn7 = assign10820_e5773_d_n7;
        locals.var_lod_half_dn8 = assign10820_e5773_d_n8;
        locals.var_lod_half_dn9 = assign10820_e5773_d_n9;
        locals.var_lod_half_dn10 = assign10820_e5773_d_n10;
        locals.var_lod_half_dn11 = assign10820_e5773_d_n11;
        locals.var_lod_half_dn14 = assign10820_e5773_d_n14;

        locals.var_npexte = locals.var_uc_npext;
        locals.var_npexte_dn0 = 0.0;
        locals.var_npexte_dn2 = 0.0;
        locals.var_npexte_dn4 = 0.0;
        locals.var_npexte_dn5 = 0.0;
        locals.var_npexte_dn6 = 0.0;
        locals.var_npexte_dn7 = 0.0;
        locals.var_npexte_dn8 = 0.0;
        locals.var_npexte_dn9 = 0.0;
        locals.var_npexte_dn10 = 0.0;
        locals.var_npexte_dn11 = 0.0;
        locals.var_npexte_dn14 = 0.0;

        locals.var_ef_mueph1 = locals.var_uc_mueph1;
        locals.var_ef_mueph1_dn0 = 0.0;
        locals.var_ef_mueph1_dn2 = 0.0;
        locals.var_ef_mueph1_dn4 = 0.0;
        locals.var_ef_mueph1_dn5 = 0.0;
        locals.var_ef_mueph1_dn6 = 0.0;
        locals.var_ef_mueph1_dn7 = 0.0;
        locals.var_ef_mueph1_dn8 = 0.0;
        locals.var_ef_mueph1_dn9 = 0.0;
        locals.var_ef_mueph1_dn10 = 0.0;
        locals.var_ef_mueph1_dn11 = 0.0;
        locals.var_ef_mueph1_dn14 = 0.0;

        locals.var_ef_nsubp = locals.var_uc_nsubp;
        locals.var_ef_nsubp_dn0 = 0.0;
        locals.var_ef_nsubp_dn2 = 0.0;
        locals.var_ef_nsubp_dn4 = 0.0;
        locals.var_ef_nsubp_dn5 = 0.0;
        locals.var_ef_nsubp_dn6 = 0.0;
        locals.var_ef_nsubp_dn7 = 0.0;
        locals.var_ef_nsubp_dn8 = 0.0;
        locals.var_ef_nsubp_dn9 = 0.0;
        locals.var_ef_nsubp_dn10 = 0.0;
        locals.var_ef_nsubp_dn11 = 0.0;
        locals.var_ef_nsubp_dn14 = 0.0;

        locals.var_ef_nsubc = locals.var_uc_nsubc;
        locals.var_ef_nsubc_dn0 = 0.0;
        locals.var_ef_nsubc_dn2 = 0.0;
        locals.var_ef_nsubc_dn4 = 0.0;
        locals.var_ef_nsubc_dn5 = 0.0;
        locals.var_ef_nsubc_dn6 = 0.0;
        locals.var_ef_nsubc_dn7 = 0.0;
        locals.var_ef_nsubc_dn8 = 0.0;
        locals.var_ef_nsubc_dn9 = 0.0;
        locals.var_ef_nsubc_dn10 = 0.0;
        locals.var_ef_nsubc_dn11 = 0.0;
        locals.var_ef_nsubc_dn14 = 0.0;

        let assign10870_e5782: f64 = if ((p.p32 == 1.0) && (locals.var_nsubcdfm_given != 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard255 = assign10870_e5782;

        let (assign10890_e5803, assign10890_e5803_d_n0, assign10890_e5803_d_n2, assign10890_e5803_d_n4, assign10890_e5803_d_n5, assign10890_e5803_d_n6, assign10890_e5803_d_n7, assign10890_e5803_d_n8, assign10890_e5803_d_n9, assign10890_e5803_d_n10, assign10890_e5803_d_n11, assign10890_e5803_d_n14,) = {
    if (locals.var_guard255 != 0.0) {
        let assign10890_e5794: f64 = (locals.var_mks_nsubcdfm).ln();
        let assign10890_e5796: f64 = (locals.var_ef_nsubc).ln();
        let assign10890_e5797: f64 = (assign10890_e5794 - assign10890_e5796);
        let assign10890_e5798: f64 = (p.p282 * assign10890_e5797);
        let assign10890_e5800: f64 = (assign10890_e5798 + 1.0);
        let assign10890_e5801: f64 = (locals.var_ef_mueph1 * assign10890_e5800);
        (assign10890_e5801, ((locals.var_ef_mueph1_dn0 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn0 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn2 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn2 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn4 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn4 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn5 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn5 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn6 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn6 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn7 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn7 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn8 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn8 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn9 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn9 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn10 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn10 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn11 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn11 / locals.var_ef_nsubc))))), ((locals.var_ef_mueph1_dn14 * assign10890_e5800) + (locals.var_ef_mueph1 * (p.p282 * (-(locals.var_ef_nsubc_dn14 / locals.var_ef_nsubc))))),)
    } else {
        (locals.var_ef_mueph1, locals.var_ef_mueph1_dn0, locals.var_ef_mueph1_dn2, locals.var_ef_mueph1_dn4, locals.var_ef_mueph1_dn5, locals.var_ef_mueph1_dn6, locals.var_ef_mueph1_dn7, locals.var_ef_mueph1_dn8, locals.var_ef_mueph1_dn9, locals.var_ef_mueph1_dn10, locals.var_ef_mueph1_dn11, locals.var_ef_mueph1_dn14,)
    }
};
        locals.var_ef_mueph1 = assign10890_e5803;
        locals.var_ef_mueph1_dn0 = assign10890_e5803_d_n0;
        locals.var_ef_mueph1_dn2 = assign10890_e5803_d_n2;
        locals.var_ef_mueph1_dn4 = assign10890_e5803_d_n4;
        locals.var_ef_mueph1_dn5 = assign10890_e5803_d_n5;
        locals.var_ef_mueph1_dn6 = assign10890_e5803_d_n6;
        locals.var_ef_mueph1_dn7 = assign10890_e5803_d_n7;
        locals.var_ef_mueph1_dn8 = assign10890_e5803_d_n8;
        locals.var_ef_mueph1_dn9 = assign10890_e5803_d_n9;
        locals.var_ef_mueph1_dn10 = assign10890_e5803_d_n10;
        locals.var_ef_mueph1_dn11 = assign10890_e5803_d_n11;
        locals.var_ef_mueph1_dn14 = assign10890_e5803_d_n14;

        let (assign10900_e5811, assign10900_e5811_d_n0, assign10900_e5811_d_n2, assign10900_e5811_d_n4, assign10900_e5811_d_n5, assign10900_e5811_d_n6, assign10900_e5811_d_n7, assign10900_e5811_d_n8, assign10900_e5811_d_n9, assign10900_e5811_d_n10, assign10900_e5811_d_n11, assign10900_e5811_d_n14,) = {
    if (locals.var_guard255 != 0.0) {
        let assign10900_e5807: f64 = (locals.var_ef_nsubp + locals.var_mks_nsubcdfm);
        let assign10900_e5809: f64 = (assign10900_e5807 - locals.var_ef_nsubc);
        (assign10900_e5809, (locals.var_ef_nsubp_dn0 - locals.var_ef_nsubc_dn0), (locals.var_ef_nsubp_dn2 - locals.var_ef_nsubc_dn2), (locals.var_ef_nsubp_dn4 - locals.var_ef_nsubc_dn4), (locals.var_ef_nsubp_dn5 - locals.var_ef_nsubc_dn5), (locals.var_ef_nsubp_dn6 - locals.var_ef_nsubc_dn6), (locals.var_ef_nsubp_dn7 - locals.var_ef_nsubc_dn7), (locals.var_ef_nsubp_dn8 - locals.var_ef_nsubc_dn8), (locals.var_ef_nsubp_dn9 - locals.var_ef_nsubc_dn9), (locals.var_ef_nsubp_dn10 - locals.var_ef_nsubc_dn10), (locals.var_ef_nsubp_dn11 - locals.var_ef_nsubc_dn11), (locals.var_ef_nsubp_dn14 - locals.var_ef_nsubc_dn14),)
    } else {
        (locals.var_ef_nsubp, locals.var_ef_nsubp_dn0, locals.var_ef_nsubp_dn2, locals.var_ef_nsubp_dn4, locals.var_ef_nsubp_dn5, locals.var_ef_nsubp_dn6, locals.var_ef_nsubp_dn7, locals.var_ef_nsubp_dn8, locals.var_ef_nsubp_dn9, locals.var_ef_nsubp_dn10, locals.var_ef_nsubp_dn11, locals.var_ef_nsubp_dn14,)
    }
};
        locals.var_ef_nsubp = assign10900_e5811;
        locals.var_ef_nsubp_dn0 = assign10900_e5811_d_n0;
        locals.var_ef_nsubp_dn2 = assign10900_e5811_d_n2;
        locals.var_ef_nsubp_dn4 = assign10900_e5811_d_n4;
        locals.var_ef_nsubp_dn5 = assign10900_e5811_d_n5;
        locals.var_ef_nsubp_dn6 = assign10900_e5811_d_n6;
        locals.var_ef_nsubp_dn7 = assign10900_e5811_d_n7;
        locals.var_ef_nsubp_dn8 = assign10900_e5811_d_n8;
        locals.var_ef_nsubp_dn9 = assign10900_e5811_d_n9;
        locals.var_ef_nsubp_dn10 = assign10900_e5811_d_n10;
        locals.var_ef_nsubp_dn11 = assign10900_e5811_d_n11;
        locals.var_ef_nsubp_dn14 = assign10900_e5811_d_n14;

        let (assign10910_e5819, assign10910_e5819_d_n0, assign10910_e5819_d_n2, assign10910_e5819_d_n4, assign10910_e5819_d_n5, assign10910_e5819_d_n6, assign10910_e5819_d_n7, assign10910_e5819_d_n8, assign10910_e5819_d_n9, assign10910_e5819_d_n10, assign10910_e5819_d_n11, assign10910_e5819_d_n14,) = {
    if (locals.var_guard255 != 0.0) {
        let assign10910_e5815: f64 = (locals.var_npexte + locals.var_mks_nsubcdfm);
        let assign10910_e5817: f64 = (assign10910_e5815 - locals.var_ef_nsubc);
        (assign10910_e5817, (locals.var_npexte_dn0 - locals.var_ef_nsubc_dn0), (locals.var_npexte_dn2 - locals.var_ef_nsubc_dn2), (locals.var_npexte_dn4 - locals.var_ef_nsubc_dn4), (locals.var_npexte_dn5 - locals.var_ef_nsubc_dn5), (locals.var_npexte_dn6 - locals.var_ef_nsubc_dn6), (locals.var_npexte_dn7 - locals.var_ef_nsubc_dn7), (locals.var_npexte_dn8 - locals.var_ef_nsubc_dn8), (locals.var_npexte_dn9 - locals.var_ef_nsubc_dn9), (locals.var_npexte_dn10 - locals.var_ef_nsubc_dn10), (locals.var_npexte_dn11 - locals.var_ef_nsubc_dn11), (locals.var_npexte_dn14 - locals.var_ef_nsubc_dn14),)
    } else {
        (locals.var_npexte, locals.var_npexte_dn0, locals.var_npexte_dn2, locals.var_npexte_dn4, locals.var_npexte_dn5, locals.var_npexte_dn6, locals.var_npexte_dn7, locals.var_npexte_dn8, locals.var_npexte_dn9, locals.var_npexte_dn10, locals.var_npexte_dn11, locals.var_npexte_dn14,)
    }
};
        locals.var_npexte = assign10910_e5819;
        locals.var_npexte_dn0 = assign10910_e5819_d_n0;
        locals.var_npexte_dn2 = assign10910_e5819_d_n2;
        locals.var_npexte_dn4 = assign10910_e5819_d_n4;
        locals.var_npexte_dn5 = assign10910_e5819_d_n5;
        locals.var_npexte_dn6 = assign10910_e5819_d_n6;
        locals.var_npexte_dn7 = assign10910_e5819_d_n7;
        locals.var_npexte_dn8 = assign10910_e5819_d_n8;
        locals.var_npexte_dn9 = assign10910_e5819_d_n9;
        locals.var_npexte_dn10 = assign10910_e5819_d_n10;
        locals.var_npexte_dn11 = assign10910_e5819_d_n11;
        locals.var_npexte_dn14 = assign10910_e5819_d_n14;

        let (assign10920_e5823, assign10920_e5823_d_n0, assign10920_e5823_d_n2, assign10920_e5823_d_n4, assign10920_e5823_d_n5, assign10920_e5823_d_n6, assign10920_e5823_d_n7, assign10920_e5823_d_n8, assign10920_e5823_d_n9, assign10920_e5823_d_n10, assign10920_e5823_d_n11, assign10920_e5823_d_n14,) = {
    if (locals.var_guard255 != 0.0) {
        (locals.var_mks_nsubcdfm, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ef_nsubc, locals.var_ef_nsubc_dn0, locals.var_ef_nsubc_dn2, locals.var_ef_nsubc_dn4, locals.var_ef_nsubc_dn5, locals.var_ef_nsubc_dn6, locals.var_ef_nsubc_dn7, locals.var_ef_nsubc_dn8, locals.var_ef_nsubc_dn9, locals.var_ef_nsubc_dn10, locals.var_ef_nsubc_dn11, locals.var_ef_nsubc_dn14,)
    }
};
        locals.var_ef_nsubc = assign10920_e5823;
        locals.var_ef_nsubc_dn0 = assign10920_e5823_d_n0;
        locals.var_ef_nsubc_dn2 = assign10920_e5823_d_n2;
        locals.var_ef_nsubc_dn4 = assign10920_e5823_d_n4;
        locals.var_ef_nsubc_dn5 = assign10920_e5823_d_n5;
        locals.var_ef_nsubc_dn6 = assign10920_e5823_d_n6;
        locals.var_ef_nsubc_dn7 = assign10920_e5823_d_n7;
        locals.var_ef_nsubc_dn8 = assign10920_e5823_d_n8;
        locals.var_ef_nsubc_dn9 = assign10920_e5823_d_n9;
        locals.var_ef_nsubc_dn10 = assign10920_e5823_d_n10;
        locals.var_ef_nsubc_dn11 = assign10920_e5823_d_n11;
        locals.var_ef_nsubc_dn14 = assign10920_e5823_d_n14;

        let assign10930_e5829: f64 = (locals.var_wg).powf(p.p163);
        let assign10930_e5830: f64 = (p.p162 / assign10930_e5829);
        let assign10930_e5831: f64 = (1.0 + assign10930_e5830);
        let assign10930_e5832: f64 = (locals.var_ef_mueph1 * assign10930_e5831);
        let assign10930_e5837: f64 = (locals.var_lg).powf(p.p165);
        let assign10930_e5838: f64 = (p.p164 / assign10930_e5837);
        let assign10930_e5839: f64 = (1.0 + assign10930_e5838);
        let assign10930_e5840: f64 = (assign10930_e5832 * assign10930_e5839);
        let assign10930_e5845: f64 = (locals.var_wlg).powf(p.p168);
        let assign10930_e5846: f64 = (p.p167 / assign10930_e5845);
        let assign10930_e5847: f64 = (1.0 + assign10930_e5846);
        let assign10930_e5848: f64 = (assign10930_e5840 * assign10930_e5847);
        locals.var_mueph = assign10930_e5848;
        locals.var_mueph_dn0 = (((locals.var_ef_mueph1_dn0 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        locals.var_mueph_dn2 = (((locals.var_ef_mueph1_dn2 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        locals.var_mueph_dn4 = (((locals.var_ef_mueph1_dn4 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        locals.var_mueph_dn5 = (((locals.var_ef_mueph1_dn5 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        locals.var_mueph_dn6 = (((locals.var_ef_mueph1_dn6 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        locals.var_mueph_dn7 = (((locals.var_ef_mueph1_dn7 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        locals.var_mueph_dn8 = (((locals.var_ef_mueph1_dn8 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        locals.var_mueph_dn9 = (((locals.var_ef_mueph1_dn9 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        locals.var_mueph_dn10 = (((locals.var_ef_mueph1_dn10 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        locals.var_mueph_dn11 = (((locals.var_ef_mueph1_dn11 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);
        locals.var_mueph_dn14 = (((locals.var_ef_mueph1_dn14 * assign10930_e5831) * assign10930_e5839) * assign10930_e5847);

        let assign10940_e5851: f64 = if locals.var_lod_half > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard257 = assign10940_e5851;

        let (assign10950_e5859, assign10950_e5859_d_n0, assign10950_e5859_d_n2, assign10950_e5859_d_n4, assign10950_e5859_d_n5, assign10950_e5859_d_n6, assign10950_e5859_d_n7, assign10950_e5859_d_n8, assign10950_e5859_d_n9, assign10950_e5859_d_n10, assign10950_e5859_d_n11, assign10950_e5859_d_n14,) = {
    if (locals.var_guard257 != 0.0) {
        let assign10950_e5856: f64 = (1.0 + locals.var_uc_muesti2);
        let assign10950_e5857: f64 = (1.0 / assign10950_e5856);
        (assign10950_e5857, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign10950_e5859;
        locals.var_t1_dn0 = assign10950_e5859_d_n0;
        locals.var_t1_dn2 = assign10950_e5859_d_n2;
        locals.var_t1_dn4 = assign10950_e5859_d_n4;
        locals.var_t1_dn5 = assign10950_e5859_d_n5;
        locals.var_t1_dn6 = assign10950_e5859_d_n6;
        locals.var_t1_dn7 = assign10950_e5859_d_n7;
        locals.var_t1_dn8 = assign10950_e5859_d_n8;
        locals.var_t1_dn9 = assign10950_e5859_d_n9;
        locals.var_t1_dn10 = assign10950_e5859_d_n10;
        locals.var_t1_dn11 = assign10950_e5859_d_n11;
        locals.var_t1_dn14 = assign10950_e5859_d_n14;

        let (assign10960_e5867, assign10960_e5867_d_n0, assign10960_e5867_d_n2, assign10960_e5867_d_n4, assign10960_e5867_d_n5, assign10960_e5867_d_n6, assign10960_e5867_d_n7, assign10960_e5867_d_n8, assign10960_e5867_d_n9, assign10960_e5867_d_n10, assign10960_e5867_d_n11, assign10960_e5867_d_n14,) = {
    if (locals.var_guard257 != 0.0) {
        let assign10960_e5863: f64 = (locals.var_uc_muesti1 / locals.var_lod_half);
        let assign10960_e5865: f64 = (assign10960_e5863).powf(locals.var_uc_muesti3);
        (assign10960_e5865, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn9) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn9) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10960_e5863).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_dn14) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign10960_e5865 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_dn14) / (locals.var_lod_half * locals.var_lod_half))) / assign10960_e5863))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign10960_e5867;
        locals.var_t2_dn0 = assign10960_e5867_d_n0;
        locals.var_t2_dn2 = assign10960_e5867_d_n2;
        locals.var_t2_dn4 = assign10960_e5867_d_n4;
        locals.var_t2_dn5 = assign10960_e5867_d_n5;
        locals.var_t2_dn6 = assign10960_e5867_d_n6;
        locals.var_t2_dn7 = assign10960_e5867_d_n7;
        locals.var_t2_dn8 = assign10960_e5867_d_n8;
        locals.var_t2_dn9 = assign10960_e5867_d_n9;
        locals.var_t2_dn10 = assign10960_e5867_d_n10;
        locals.var_t2_dn11 = assign10960_e5867_d_n11;
        locals.var_t2_dn14 = assign10960_e5867_d_n14;

        let (assign10970_e5875, assign10970_e5875_d_n0, assign10970_e5875_d_n2, assign10970_e5875_d_n4, assign10970_e5875_d_n5, assign10970_e5875_d_n6, assign10970_e5875_d_n7, assign10970_e5875_d_n8, assign10970_e5875_d_n9, assign10970_e5875_d_n10, assign10970_e5875_d_n11, assign10970_e5875_d_n14,) = {
    if (locals.var_guard257 != 0.0) {
        let assign10970_e5871: f64 = (locals.var_uc_muesti1 / locals.var_lod_half_ref);
        let assign10970_e5873: f64 = (assign10970_e5871).powf(locals.var_uc_muesti3);
        (assign10970_e5873, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn7) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn7) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn9) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn9) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn11) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn11) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) }, if 0.0 == 0.0 && ((locals.var_uc_muesti3) as f64).is_finite() && ((locals.var_uc_muesti3) as f64).fract() == 0.0 { if locals.var_uc_muesti3 == 0.0 { 0.0 } else { (locals.var_uc_muesti3 * ((assign10970_e5871).powf(locals.var_uc_muesti3 - 1.0) * (-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn14) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign10970_e5873 * (locals.var_uc_muesti3 * ((-((locals.var_uc_muesti1 * locals.var_lod_half_ref_dn14) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign10970_e5871))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign10970_e5875;
        locals.var_t3_dn0 = assign10970_e5875_d_n0;
        locals.var_t3_dn2 = assign10970_e5875_d_n2;
        locals.var_t3_dn4 = assign10970_e5875_d_n4;
        locals.var_t3_dn5 = assign10970_e5875_d_n5;
        locals.var_t3_dn6 = assign10970_e5875_d_n6;
        locals.var_t3_dn7 = assign10970_e5875_d_n7;
        locals.var_t3_dn8 = assign10970_e5875_d_n8;
        locals.var_t3_dn9 = assign10970_e5875_d_n9;
        locals.var_t3_dn10 = assign10970_e5875_d_n10;
        locals.var_t3_dn11 = assign10970_e5875_d_n11;
        locals.var_t3_dn14 = assign10970_e5875_d_n14;

    }

    pub(super) fn stamp_transient_block_17(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign10980_e5891, assign10980_e5891_d_n0, assign10980_e5891_d_n2, assign10980_e5891_d_n4, assign10980_e5891_d_n5, assign10980_e5891_d_n6, assign10980_e5891_d_n7, assign10980_e5891_d_n8, assign10980_e5891_d_n9, assign10980_e5891_d_n10, assign10980_e5891_d_n11, assign10980_e5891_d_n14,) = {
    if (locals.var_guard257 != 0.0) {
        let assign10980_e5881: f64 = (locals.var_t1 * locals.var_t2);
        let assign10980_e5882: f64 = (1.0 + assign10980_e5881);
        let assign10980_e5883: f64 = (locals.var_mueph * assign10980_e5882);
        let assign10980_e5887: f64 = (locals.var_t1 * locals.var_t3);
        let assign10980_e5888: f64 = (1.0 + assign10980_e5887);
        let assign10980_e5889: f64 = (assign10980_e5883 / assign10980_e5888);
        (assign10980_e5889, (((((locals.var_mueph_dn0 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)))) / (assign10980_e5888 * assign10980_e5888)), (((((locals.var_mueph_dn2 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)))) / (assign10980_e5888 * assign10980_e5888)), (((((locals.var_mueph_dn4 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)))) / (assign10980_e5888 * assign10980_e5888)), (((((locals.var_mueph_dn5 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)))) / (assign10980_e5888 * assign10980_e5888)), (((((locals.var_mueph_dn6 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)))) / (assign10980_e5888 * assign10980_e5888)), (((((locals.var_mueph_dn7 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)))) / (assign10980_e5888 * assign10980_e5888)), (((((locals.var_mueph_dn8 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)))) / (assign10980_e5888 * assign10980_e5888)), (((((locals.var_mueph_dn9 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)))) / (assign10980_e5888 * assign10980_e5888)), (((((locals.var_mueph_dn10 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)))) / (assign10980_e5888 * assign10980_e5888)), (((((locals.var_mueph_dn11 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)))) / (assign10980_e5888 * assign10980_e5888)), (((((locals.var_mueph_dn14 * assign10980_e5882) + (locals.var_mueph * ((locals.var_t1_dn14 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn14)))) * assign10980_e5888) - (assign10980_e5883 * ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)))) / (assign10980_e5888 * assign10980_e5888)),)
    } else {
        (locals.var_mueph, locals.var_mueph_dn0, locals.var_mueph_dn2, locals.var_mueph_dn4, locals.var_mueph_dn5, locals.var_mueph_dn6, locals.var_mueph_dn7, locals.var_mueph_dn8, locals.var_mueph_dn9, locals.var_mueph_dn10, locals.var_mueph_dn11, locals.var_mueph_dn14,)
    }
};
        locals.var_mueph = assign10980_e5891;
        locals.var_mueph_dn0 = assign10980_e5891_d_n0;
        locals.var_mueph_dn2 = assign10980_e5891_d_n2;
        locals.var_mueph_dn4 = assign10980_e5891_d_n4;
        locals.var_mueph_dn5 = assign10980_e5891_d_n5;
        locals.var_mueph_dn6 = assign10980_e5891_d_n6;
        locals.var_mueph_dn7 = assign10980_e5891_d_n7;
        locals.var_mueph_dn8 = assign10980_e5891_d_n8;
        locals.var_mueph_dn9 = assign10980_e5891_d_n9;
        locals.var_mueph_dn10 = assign10980_e5891_d_n10;
        locals.var_mueph_dn11 = assign10980_e5891_d_n11;
        locals.var_mueph_dn14 = assign10980_e5891_d_n14;

        let assign10990_e5897: f64 = (locals.var_lg).powf(p.p176);
        let assign10990_e5898: f64 = (p.p173 / assign10990_e5897);
        let assign10990_e5899: f64 = (1.0 + assign10990_e5898);
        let assign10990_e5900: f64 = (p.p171 * assign10990_e5899);
        let assign10990_e5905: f64 = (locals.var_wg).powf(p.p175);
        let assign10990_e5906: f64 = (p.p174 / assign10990_e5905);
        let assign10990_e5907: f64 = (1.0 + assign10990_e5906);
        let assign10990_e5908: f64 = (assign10990_e5900 * assign10990_e5907);
        locals.var_muesr = assign10990_e5908;

        let (assign11020_e5932, assign11020_e5932_d_n0, assign11020_e5932_d_n2, assign11020_e5932_d_n4, assign11020_e5932_d_n5, assign11020_e5932_d_n6, assign11020_e5932_d_n7, assign11020_e5932_d_n8, assign11020_e5932_d_n9, assign11020_e5932_d_n10, assign11020_e5932_d_n11, assign11020_e5932_d_n14,) = {
    if (locals.var_mueph < 1e-25) {
        (1e-25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_mueph, locals.var_mueph_dn0, locals.var_mueph_dn2, locals.var_mueph_dn4, locals.var_mueph_dn5, locals.var_mueph_dn6, locals.var_mueph_dn7, locals.var_mueph_dn8, locals.var_mueph_dn9, locals.var_mueph_dn10, locals.var_mueph_dn11, locals.var_mueph_dn14,)
    }
};
        locals.var_mueph = assign11020_e5932;
        locals.var_mueph_dn0 = assign11020_e5932_d_n0;
        locals.var_mueph_dn2 = assign11020_e5932_d_n2;
        locals.var_mueph_dn4 = assign11020_e5932_d_n4;
        locals.var_mueph_dn5 = assign11020_e5932_d_n5;
        locals.var_mueph_dn6 = assign11020_e5932_d_n6;
        locals.var_mueph_dn7 = assign11020_e5932_d_n7;
        locals.var_mueph_dn8 = assign11020_e5932_d_n8;
        locals.var_mueph_dn9 = assign11020_e5932_d_n9;
        locals.var_mueph_dn10 = assign11020_e5932_d_n10;
        locals.var_mueph_dn11 = assign11020_e5932_d_n11;
        locals.var_mueph_dn14 = assign11020_e5932_d_n14;

        let (assign11030_e5938,) = {
    if (locals.var_muesr < 1e-25) {
        (1e-25,)
    } else {
        (locals.var_muesr,)
    }
};
        locals.var_muesr = assign11030_e5938;

        let assign11040_e5941: f64 = (locals.var_lg).powf(p.p156);
        locals.var_t1 = assign11040_e5941;
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

        let assign11050_e5944: f64 = (locals.var_uc_ndep * locals.var_t1);
        let assign11050_e5947: f64 = (locals.var_t1 + p.p155);
        let assign11050_e5948: f64 = (assign11050_e5944 / assign11050_e5947);
        let assign11050_e5950: f64 = (assign11050_e5948 / 1.034943e-10);
        locals.var_ndep_o_esi = assign11050_e5950;
        locals.var_ndep_o_esi_dn0 = (((((locals.var_uc_ndep * locals.var_t1_dn0) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn0)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn2 = (((((locals.var_uc_ndep * locals.var_t1_dn2) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn2)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn4 = (((((locals.var_uc_ndep * locals.var_t1_dn4) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn4)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn5 = (((((locals.var_uc_ndep * locals.var_t1_dn5) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn5)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn6 = (((((locals.var_uc_ndep * locals.var_t1_dn6) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn6)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn7 = (((((locals.var_uc_ndep * locals.var_t1_dn7) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn7)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn8 = (((((locals.var_uc_ndep * locals.var_t1_dn8) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn8)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn9 = (((((locals.var_uc_ndep * locals.var_t1_dn9) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn9)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn10 = (((((locals.var_uc_ndep * locals.var_t1_dn10) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn10)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn11 = (((((locals.var_uc_ndep * locals.var_t1_dn11) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn11)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);
        locals.var_ndep_o_esi_dn14 = (((((locals.var_uc_ndep * locals.var_t1_dn14) * assign11050_e5947) - (assign11050_e5944 * locals.var_t1_dn14)) / (assign11050_e5947 * assign11050_e5947)) / 1.034943e-10);

        let assign11060_e5953: f64 = (locals.var_uc_ninv / 1.034943e-10);
        locals.var_ninv_o_esi = assign11060_e5953;

        let assign11070_e5959: f64 = (locals.var_lg).powf(p.p321);
        let assign11070_e5960: f64 = (p.p320 / assign11070_e5959);
        let assign11070_e5961: f64 = (1.0 + assign11070_e5960);
        let assign11070_e5962: f64 = (p.p319 * assign11070_e5961);
        let assign11070_e5967: f64 = (locals.var_wg).powf(p.p323);
        let assign11070_e5968: f64 = (p.p322 / assign11070_e5967);
        let assign11070_e5969: f64 = (1.0 + assign11070_e5968);
        let assign11070_e5970: f64 = (assign11070_e5962 * assign11070_e5969);
        locals.var_ninvd0 = assign11070_e5970;

        let assign11080_e5975: f64 = (locals.var_lg).powf(p.p387);
        let assign11080_e5976: f64 = (p.p386 / assign11080_e5975);
        let assign11080_e5977: f64 = (1.0 + assign11080_e5976);
        let assign11080_e5982: f64 = (locals.var_wg).powf(p.p389);
        let assign11080_e5983: f64 = (p.p388 / assign11080_e5982);
        let assign11080_e5984: f64 = (1.0 + assign11080_e5983);
        let assign11080_e5985: f64 = (assign11080_e5977 * assign11080_e5984);
        locals.var_t1 = assign11080_e5985;
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

        let assign11090_e5988: f64 = (p.p384 * locals.var_t1);
        locals.var_ninvd0cres = assign11090_e5988;
        locals.var_ninvd0cres_dn0 = (p.p384 * locals.var_t1_dn0);
        locals.var_ninvd0cres_dn2 = (p.p384 * locals.var_t1_dn2);
        locals.var_ninvd0cres_dn4 = (p.p384 * locals.var_t1_dn4);
        locals.var_ninvd0cres_dn5 = (p.p384 * locals.var_t1_dn5);
        locals.var_ninvd0cres_dn6 = (p.p384 * locals.var_t1_dn6);
        locals.var_ninvd0cres_dn7 = (p.p384 * locals.var_t1_dn7);
        locals.var_ninvd0cres_dn8 = (p.p384 * locals.var_t1_dn8);
        locals.var_ninvd0cres_dn9 = (p.p384 * locals.var_t1_dn9);
        locals.var_ninvd0cres_dn10 = (p.p384 * locals.var_t1_dn10);
        locals.var_ninvd0cres_dn11 = (p.p384 * locals.var_t1_dn11);
        locals.var_ninvd0cres_dn14 = (p.p384 * locals.var_t1_dn14);

        let assign11100_e5991: f64 = (p.p385 * locals.var_t1);
        locals.var_ninvd0hres = assign11100_e5991;
        locals.var_ninvd0hres_dn0 = (p.p385 * locals.var_t1_dn0);
        locals.var_ninvd0hres_dn2 = (p.p385 * locals.var_t1_dn2);
        locals.var_ninvd0hres_dn4 = (p.p385 * locals.var_t1_dn4);
        locals.var_ninvd0hres_dn5 = (p.p385 * locals.var_t1_dn5);
        locals.var_ninvd0hres_dn6 = (p.p385 * locals.var_t1_dn6);
        locals.var_ninvd0hres_dn7 = (p.p385 * locals.var_t1_dn7);
        locals.var_ninvd0hres_dn8 = (p.p385 * locals.var_t1_dn8);
        locals.var_ninvd0hres_dn9 = (p.p385 * locals.var_t1_dn9);
        locals.var_ninvd0hres_dn10 = (p.p385 * locals.var_t1_dn10);
        locals.var_ninvd0hres_dn11 = (p.p385 * locals.var_t1_dn11);
        locals.var_ninvd0hres_dn14 = (p.p385 * locals.var_t1_dn14);

        let assign11110_e5996: f64 = (locals.var_lgate + p.p121);
        let assign11110_e5998: f64 = (assign11110_e5996).powf(p.p122);
        let assign11110_e5999: f64 = (locals.var_mks_ll / assign11110_e5998);
        let assign11110_e6000: f64 = (p.p97 + assign11110_e5999);
        locals.var_dl = assign11110_e6000;

        let assign11120_e6005: f64 = (locals.var_lgate + p.p121);
        let assign11120_e6007: f64 = (assign11120_e6005).powf(p.p122);
        let assign11120_e6008: f64 = (locals.var_mks_ll / assign11120_e6007);
        let assign11120_e6009: f64 = (locals.var_uc_xldld + assign11120_e6008);
        locals.var_dlld = assign11120_e6009;

        let assign11130_e6014: f64 = (locals.var_wgate + p.p128);
        let assign11130_e6016: f64 = (assign11130_e6014).powf(p.p129);
        let assign11130_e6017: f64 = (locals.var_mks_wl / assign11130_e6016);
        let assign11130_e6018: f64 = (p.p114 + assign11130_e6017);
        locals.var_dw = assign11130_e6018;

        let assign11140_e6023: f64 = (locals.var_wgate + p.p128);
        let assign11140_e6025: f64 = (assign11140_e6023).powf(p.p129);
        let assign11140_e6026: f64 = (locals.var_mks_wl / assign11140_e6025);
        let assign11140_e6027: f64 = (p.p295 + assign11140_e6026);
        locals.var_dwld = assign11140_e6027;

        let assign11150_e6032: f64 = (locals.var_wgate + p.p128);
        let assign11150_e6034: f64 = (assign11150_e6032).powf(p.p129);
        let assign11150_e6035: f64 = (locals.var_mks_wl / assign11150_e6034);
        let assign11150_e6036: f64 = (p.p115 + assign11150_e6035);
        locals.var_dwcv = assign11150_e6036;

        let assign11160_e6040: f64 = (locals.var_dl + locals.var_dlld);
        let assign11160_e6041: f64 = (locals.var_lgate - assign11160_e6040);
        locals.var_leff = assign11160_e6041;

        let assign11190_e6053: f64 = (locals.var_wlg).powf(p.p125);
        let assign11190_e6054: f64 = (p.p124 / assign11190_e6053);
        let assign11190_e6055: f64 = (locals.var_lgate + assign11190_e6054);
        locals.var_lgatesm = assign11190_e6055;

        let assign11200_e6059: f64 = (locals.var_wlg).powf(p.p127);
        let assign11200_e6060: f64 = (locals.var_uc_wl2 / assign11200_e6059);
        locals.var_dvthsm = assign11200_e6060;

        let assign11210_e6065: f64 = (locals.var_lgatesm * 1000000.0);
        let assign11210_e6067: f64 = (assign11210_e6065).powf(p.p207);
        let assign11210_e6068: f64 = (p.p206 / assign11210_e6067);
        let assign11210_e6069: f64 = (1.0 + assign11210_e6068);
        locals.var_t1 = assign11210_e6069;
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

        let assign11220_e6074: f64 = (locals.var_wg).powf(p.p209);
        let assign11220_e6075: f64 = (p.p208 / assign11220_e6074);
        let assign11220_e6076: f64 = (1.0 + assign11220_e6075);
        locals.var_t2 = assign11220_e6076;
        locals.var_t2_dn0 = 0.0;
        locals.var_t2_dn2 = 0.0;
        locals.var_t2_dn4 = 0.0;
        locals.var_t2_dn5 = 0.0;
        locals.var_t2_dn6 = 0.0;
        locals.var_t2_dn7 = 0.0;
        locals.var_t2_dn8 = 0.0;
        locals.var_t2_dn9 = 0.0;
        locals.var_t2_dn10 = 0.0;
        locals.var_t2_dn11 = 0.0;
        locals.var_t2_dn14 = 0.0;

        let assign11230_e6079: f64 = (locals.var_uc_wsti * locals.var_t1);
        let assign11230_e6081: f64 = (assign11230_e6079 * locals.var_t2);
        locals.var_uc_wsti = assign11230_e6081;
        locals.var_uc_wsti_dn0 = ((((locals.var_uc_wsti_dn0 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn0)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn0));
        locals.var_uc_wsti_dn2 = ((((locals.var_uc_wsti_dn2 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn2)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn2));
        locals.var_uc_wsti_dn4 = ((((locals.var_uc_wsti_dn4 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn4)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn4));
        locals.var_uc_wsti_dn5 = ((((locals.var_uc_wsti_dn5 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn5)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn5));
        locals.var_uc_wsti_dn6 = ((((locals.var_uc_wsti_dn6 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn6)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn6));
        locals.var_uc_wsti_dn7 = ((((locals.var_uc_wsti_dn7 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn7)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn7));
        locals.var_uc_wsti_dn8 = ((((locals.var_uc_wsti_dn8 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn8)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn8));
        locals.var_uc_wsti_dn9 = ((((locals.var_uc_wsti_dn9 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn9)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn9));
        locals.var_uc_wsti_dn10 = ((((locals.var_uc_wsti_dn10 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn10)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn10));
        locals.var_uc_wsti_dn11 = ((((locals.var_uc_wsti_dn11 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn11)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn11));
        locals.var_uc_wsti_dn14 = ((((locals.var_uc_wsti_dn14 * locals.var_t1) + (locals.var_uc_wsti * locals.var_t1_dn14)) * locals.var_t2) + (assign11230_e6079 * locals.var_t2_dn14));

        let assign11240_e6085: f64 = (2.0 * locals.var_dw);
        let assign11240_e6086: f64 = (locals.var_wgate - assign11240_e6085);
        locals.var_weff = assign11240_e6086;

        let assign11250_e6090: f64 = (2.0 * locals.var_dwld);
        let assign11250_e6091: f64 = (locals.var_wgate - assign11250_e6090);
        locals.var_weff_ld = assign11250_e6091;

        let assign11260_e6095: f64 = (2.0 * locals.var_dwcv);
        let assign11260_e6096: f64 = (locals.var_wgate - assign11260_e6095);
        locals.var_weff_cv = assign11260_e6096;

        let assign11330_e6120: f64 = (locals.var_weff * p.p7);
        locals.var_weff_nf = assign11330_e6120;

        let assign11340_e6123: f64 = (locals.var_weff_cv * p.p7);
        locals.var_weffcv_nf = assign11340_e6123;

        let assign11350_e6129: f64 = (locals.var_wg).powf(p.p143);
        let assign11350_e6130: f64 = (p.p142 / assign11350_e6129);
        let assign11350_e6131: f64 = (1.0 + assign11350_e6130);
        let assign11350_e6132: f64 = (locals.var_ef_nsubp * assign11350_e6131);
        locals.var_nsubpp = assign11350_e6132;
        locals.var_nsubpp_dn0 = (locals.var_ef_nsubp_dn0 * assign11350_e6131);
        locals.var_nsubpp_dn2 = (locals.var_ef_nsubp_dn2 * assign11350_e6131);
        locals.var_nsubpp_dn4 = (locals.var_ef_nsubp_dn4 * assign11350_e6131);
        locals.var_nsubpp_dn5 = (locals.var_ef_nsubp_dn5 * assign11350_e6131);
        locals.var_nsubpp_dn6 = (locals.var_ef_nsubp_dn6 * assign11350_e6131);
        locals.var_nsubpp_dn7 = (locals.var_ef_nsubp_dn7 * assign11350_e6131);
        locals.var_nsubpp_dn8 = (locals.var_ef_nsubp_dn8 * assign11350_e6131);
        locals.var_nsubpp_dn9 = (locals.var_ef_nsubp_dn9 * assign11350_e6131);
        locals.var_nsubpp_dn10 = (locals.var_ef_nsubp_dn10 * assign11350_e6131);
        locals.var_nsubpp_dn11 = (locals.var_ef_nsubp_dn11 * assign11350_e6131);
        locals.var_nsubpp_dn14 = (locals.var_ef_nsubp_dn14 * assign11350_e6131);

        let assign11360_e6138: f64 = (locals.var_wg).powf(p.p234);
        let assign11360_e6139: f64 = (p.p233 / assign11360_e6138);
        let assign11360_e6140: f64 = (1.0 + assign11360_e6139);
        let assign11360_e6141: f64 = (locals.var_ef_nsubc * assign11360_e6140);
        locals.var_ef_nsubc = assign11360_e6141;
        locals.var_ef_nsubc_dn0 = (locals.var_ef_nsubc_dn0 * assign11360_e6140);
        locals.var_ef_nsubc_dn2 = (locals.var_ef_nsubc_dn2 * assign11360_e6140);
        locals.var_ef_nsubc_dn4 = (locals.var_ef_nsubc_dn4 * assign11360_e6140);
        locals.var_ef_nsubc_dn5 = (locals.var_ef_nsubc_dn5 * assign11360_e6140);
        locals.var_ef_nsubc_dn6 = (locals.var_ef_nsubc_dn6 * assign11360_e6140);
        locals.var_ef_nsubc_dn7 = (locals.var_ef_nsubc_dn7 * assign11360_e6140);
        locals.var_ef_nsubc_dn8 = (locals.var_ef_nsubc_dn8 * assign11360_e6140);
        locals.var_ef_nsubc_dn9 = (locals.var_ef_nsubc_dn9 * assign11360_e6140);
        locals.var_ef_nsubc_dn10 = (locals.var_ef_nsubc_dn10 * assign11360_e6140);
        locals.var_ef_nsubc_dn11 = (locals.var_ef_nsubc_dn11 * assign11360_e6140);
        locals.var_ef_nsubc_dn14 = (locals.var_ef_nsubc_dn14 * assign11360_e6140);

        let assign11370_e6144: f64 = (locals.var_ef_nsubc * 1e-6);
        locals.var_t1 = assign11370_e6144;
        locals.var_t1_dn0 = (locals.var_ef_nsubc_dn0 * 1e-6);
        locals.var_t1_dn2 = (locals.var_ef_nsubc_dn2 * 1e-6);
        locals.var_t1_dn4 = (locals.var_ef_nsubc_dn4 * 1e-6);
        locals.var_t1_dn5 = (locals.var_ef_nsubc_dn5 * 1e-6);
        locals.var_t1_dn6 = (locals.var_ef_nsubc_dn6 * 1e-6);
        locals.var_t1_dn7 = (locals.var_ef_nsubc_dn7 * 1e-6);
        locals.var_t1_dn8 = (locals.var_ef_nsubc_dn8 * 1e-6);
        locals.var_t1_dn9 = (locals.var_ef_nsubc_dn9 * 1e-6);
        locals.var_t1_dn10 = (locals.var_ef_nsubc_dn10 * 1e-6);
        locals.var_t1_dn11 = (locals.var_ef_nsubc_dn11 * 1e-6);
        locals.var_t1_dn14 = (locals.var_ef_nsubc_dn14 * 1e-6);

        let assign11380_e6147: f64 = (locals.var_nsubpp * 1e-6);
        locals.var_t2 = assign11380_e6147;
        locals.var_t2_dn0 = (locals.var_nsubpp_dn0 * 1e-6);
        locals.var_t2_dn2 = (locals.var_nsubpp_dn2 * 1e-6);
        locals.var_t2_dn4 = (locals.var_nsubpp_dn4 * 1e-6);
        locals.var_t2_dn5 = (locals.var_nsubpp_dn5 * 1e-6);
        locals.var_t2_dn6 = (locals.var_nsubpp_dn6 * 1e-6);
        locals.var_t2_dn7 = (locals.var_nsubpp_dn7 * 1e-6);
        locals.var_t2_dn8 = (locals.var_nsubpp_dn8 * 1e-6);
        locals.var_t2_dn9 = (locals.var_nsubpp_dn9 * 1e-6);
        locals.var_t2_dn10 = (locals.var_nsubpp_dn10 * 1e-6);
        locals.var_t2_dn11 = (locals.var_nsubpp_dn11 * 1e-6);
        locals.var_t2_dn14 = (locals.var_nsubpp_dn14 * 1e-6);

        let assign11400_e6155: f64 = if locals.var_t1 < 1000000000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard265 = assign11400_e6155;

        let (assign11410_e6159, assign11410_e6159_d_n0, assign11410_e6159_d_n2, assign11410_e6159_d_n4, assign11410_e6159_d_n5, assign11410_e6159_d_n6, assign11410_e6159_d_n7, assign11410_e6159_d_n8, assign11410_e6159_d_n9, assign11410_e6159_d_n10, assign11410_e6159_d_n11, assign11410_e6159_d_n14,) = {
    if (locals.var_guard265 != 0.0) {
        (1000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign11410_e6159;
        locals.var_t1_dn0 = assign11410_e6159_d_n0;
        locals.var_t1_dn2 = assign11410_e6159_d_n2;
        locals.var_t1_dn4 = assign11410_e6159_d_n4;
        locals.var_t1_dn5 = assign11410_e6159_d_n5;
        locals.var_t1_dn6 = assign11410_e6159_d_n6;
        locals.var_t1_dn7 = assign11410_e6159_d_n7;
        locals.var_t1_dn8 = assign11410_e6159_d_n8;
        locals.var_t1_dn9 = assign11410_e6159_d_n9;
        locals.var_t1_dn10 = assign11410_e6159_d_n10;
        locals.var_t1_dn11 = assign11410_e6159_d_n11;
        locals.var_t1_dn14 = assign11410_e6159_d_n14;

        let assign11420_e6162: f64 = (locals.var_t1 / 1e-6);
        locals.var_ef_nsubc = assign11420_e6162;
        locals.var_ef_nsubc_dn0 = (locals.var_t1_dn0 / 1e-6);
        locals.var_ef_nsubc_dn2 = (locals.var_t1_dn2 / 1e-6);
        locals.var_ef_nsubc_dn4 = (locals.var_t1_dn4 / 1e-6);
        locals.var_ef_nsubc_dn5 = (locals.var_t1_dn5 / 1e-6);
        locals.var_ef_nsubc_dn6 = (locals.var_t1_dn6 / 1e-6);
        locals.var_ef_nsubc_dn7 = (locals.var_t1_dn7 / 1e-6);
        locals.var_ef_nsubc_dn8 = (locals.var_t1_dn8 / 1e-6);
        locals.var_ef_nsubc_dn9 = (locals.var_t1_dn9 / 1e-6);
        locals.var_ef_nsubc_dn10 = (locals.var_t1_dn10 / 1e-6);
        locals.var_ef_nsubc_dn11 = (locals.var_t1_dn11 / 1e-6);
        locals.var_ef_nsubc_dn14 = (locals.var_t1_dn14 / 1e-6);

        let assign11440_e6170: f64 = if locals.var_t2 < 1000000000000000.0 { 1.0 } else { 0.0 };
        locals.var_guard267 = assign11440_e6170;

        let (assign11450_e6174, assign11450_e6174_d_n0, assign11450_e6174_d_n2, assign11450_e6174_d_n4, assign11450_e6174_d_n5, assign11450_e6174_d_n6, assign11450_e6174_d_n7, assign11450_e6174_d_n8, assign11450_e6174_d_n9, assign11450_e6174_d_n10, assign11450_e6174_d_n11, assign11450_e6174_d_n14,) = {
    if (locals.var_guard267 != 0.0) {
        (1000000000000000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign11450_e6174;
        locals.var_t2_dn0 = assign11450_e6174_d_n0;
        locals.var_t2_dn2 = assign11450_e6174_d_n2;
        locals.var_t2_dn4 = assign11450_e6174_d_n4;
        locals.var_t2_dn5 = assign11450_e6174_d_n5;
        locals.var_t2_dn6 = assign11450_e6174_d_n6;
        locals.var_t2_dn7 = assign11450_e6174_d_n7;
        locals.var_t2_dn8 = assign11450_e6174_d_n8;
        locals.var_t2_dn9 = assign11450_e6174_d_n9;
        locals.var_t2_dn10 = assign11450_e6174_d_n10;
        locals.var_t2_dn11 = assign11450_e6174_d_n11;
        locals.var_t2_dn14 = assign11450_e6174_d_n14;

        let assign11460_e6177: f64 = (locals.var_t2 / 1e-6);
        locals.var_nsubpp = assign11460_e6177;
        locals.var_nsubpp_dn0 = (locals.var_t2_dn0 / 1e-6);
        locals.var_nsubpp_dn2 = (locals.var_t2_dn2 / 1e-6);
        locals.var_nsubpp_dn4 = (locals.var_t2_dn4 / 1e-6);
        locals.var_nsubpp_dn5 = (locals.var_t2_dn5 / 1e-6);
        locals.var_nsubpp_dn6 = (locals.var_t2_dn6 / 1e-6);
        locals.var_nsubpp_dn7 = (locals.var_t2_dn7 / 1e-6);
        locals.var_nsubpp_dn8 = (locals.var_t2_dn8 / 1e-6);
        locals.var_nsubpp_dn9 = (locals.var_t2_dn9 / 1e-6);
        locals.var_nsubpp_dn10 = (locals.var_t2_dn10 / 1e-6);
        locals.var_nsubpp_dn11 = (locals.var_t2_dn11 / 1e-6);
        locals.var_nsubpp_dn14 = (locals.var_t2_dn14 / 1e-6);

        let assign11470_e6180: f64 = if locals.var_lod_half > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard268 = assign11470_e6180;

        let (assign11480_e6188, assign11480_e6188_d_n0, assign11480_e6188_d_n2, assign11480_e6188_d_n4, assign11480_e6188_d_n5, assign11480_e6188_d_n6, assign11480_e6188_d_n7, assign11480_e6188_d_n8, assign11480_e6188_d_n9, assign11480_e6188_d_n10, assign11480_e6188_d_n11, assign11480_e6188_d_n14,) = {
    if (locals.var_guard268 != 0.0) {
        let assign11480_e6185: f64 = (1.0 + locals.var_uc_nsubpsti2);
        let assign11480_e6186: f64 = (1.0 / assign11480_e6185);
        (assign11480_e6186, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign11480_e6188;
        locals.var_t1_dn0 = assign11480_e6188_d_n0;
        locals.var_t1_dn2 = assign11480_e6188_d_n2;
        locals.var_t1_dn4 = assign11480_e6188_d_n4;
        locals.var_t1_dn5 = assign11480_e6188_d_n5;
        locals.var_t1_dn6 = assign11480_e6188_d_n6;
        locals.var_t1_dn7 = assign11480_e6188_d_n7;
        locals.var_t1_dn8 = assign11480_e6188_d_n8;
        locals.var_t1_dn9 = assign11480_e6188_d_n9;
        locals.var_t1_dn10 = assign11480_e6188_d_n10;
        locals.var_t1_dn11 = assign11480_e6188_d_n11;
        locals.var_t1_dn14 = assign11480_e6188_d_n14;

        let (assign11490_e6196, assign11490_e6196_d_n0, assign11490_e6196_d_n2, assign11490_e6196_d_n4, assign11490_e6196_d_n5, assign11490_e6196_d_n6, assign11490_e6196_d_n7, assign11490_e6196_d_n8, assign11490_e6196_d_n9, assign11490_e6196_d_n10, assign11490_e6196_d_n11, assign11490_e6196_d_n14,) = {
    if (locals.var_guard268 != 0.0) {
        let assign11490_e6192: f64 = (locals.var_uc_nsubpsti1 / locals.var_lod_half);
        let assign11490_e6194: f64 = (assign11490_e6192).powf(locals.var_uc_nsubpsti3);
        (assign11490_e6194, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn0) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn2) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn4) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn5) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn6) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn7) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn8) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn9) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn9) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn10) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn11) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11490_e6192).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn14) / (locals.var_lod_half * locals.var_lod_half))))) } } else { (assign11490_e6194 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_dn14) / (locals.var_lod_half * locals.var_lod_half))) / assign11490_e6192))) },)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign11490_e6196;
        locals.var_t2_dn0 = assign11490_e6196_d_n0;
        locals.var_t2_dn2 = assign11490_e6196_d_n2;
        locals.var_t2_dn4 = assign11490_e6196_d_n4;
        locals.var_t2_dn5 = assign11490_e6196_d_n5;
        locals.var_t2_dn6 = assign11490_e6196_d_n6;
        locals.var_t2_dn7 = assign11490_e6196_d_n7;
        locals.var_t2_dn8 = assign11490_e6196_d_n8;
        locals.var_t2_dn9 = assign11490_e6196_d_n9;
        locals.var_t2_dn10 = assign11490_e6196_d_n10;
        locals.var_t2_dn11 = assign11490_e6196_d_n11;
        locals.var_t2_dn14 = assign11490_e6196_d_n14;

        let (assign11500_e6204, assign11500_e6204_d_n0, assign11500_e6204_d_n2, assign11500_e6204_d_n4, assign11500_e6204_d_n5, assign11500_e6204_d_n6, assign11500_e6204_d_n7, assign11500_e6204_d_n8, assign11500_e6204_d_n9, assign11500_e6204_d_n10, assign11500_e6204_d_n11, assign11500_e6204_d_n14,) = {
    if (locals.var_guard268 != 0.0) {
        let assign11500_e6200: f64 = (locals.var_uc_nsubpsti1 / locals.var_lod_half_ref);
        let assign11500_e6202: f64 = (assign11500_e6200).powf(locals.var_uc_nsubpsti3);
        (assign11500_e6202, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn0) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn2) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn4) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn5) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn6) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn7) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn7) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn8) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn9) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn9) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn10) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn11) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn11) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) }, if 0.0 == 0.0 && ((locals.var_uc_nsubpsti3) as f64).is_finite() && ((locals.var_uc_nsubpsti3) as f64).fract() == 0.0 { if locals.var_uc_nsubpsti3 == 0.0 { 0.0 } else { (locals.var_uc_nsubpsti3 * ((assign11500_e6200).powf(locals.var_uc_nsubpsti3 - 1.0) * (-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn14) / (locals.var_lod_half_ref * locals.var_lod_half_ref))))) } } else { (assign11500_e6202 * (locals.var_uc_nsubpsti3 * ((-((locals.var_uc_nsubpsti1 * locals.var_lod_half_ref_dn14) / (locals.var_lod_half_ref * locals.var_lod_half_ref))) / assign11500_e6200))) },)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign11500_e6204;
        locals.var_t3_dn0 = assign11500_e6204_d_n0;
        locals.var_t3_dn2 = assign11500_e6204_d_n2;
        locals.var_t3_dn4 = assign11500_e6204_d_n4;
        locals.var_t3_dn5 = assign11500_e6204_d_n5;
        locals.var_t3_dn6 = assign11500_e6204_d_n6;
        locals.var_t3_dn7 = assign11500_e6204_d_n7;
        locals.var_t3_dn8 = assign11500_e6204_d_n8;
        locals.var_t3_dn9 = assign11500_e6204_d_n9;
        locals.var_t3_dn10 = assign11500_e6204_d_n10;
        locals.var_t3_dn11 = assign11500_e6204_d_n11;
        locals.var_t3_dn14 = assign11500_e6204_d_n14;

        let (assign11510_e6220, assign11510_e6220_d_n0, assign11510_e6220_d_n2, assign11510_e6220_d_n4, assign11510_e6220_d_n5, assign11510_e6220_d_n6, assign11510_e6220_d_n7, assign11510_e6220_d_n8, assign11510_e6220_d_n9, assign11510_e6220_d_n10, assign11510_e6220_d_n11, assign11510_e6220_d_n14,) = {
    if (locals.var_guard268 != 0.0) {
        let assign11510_e6210: f64 = (locals.var_t1 * locals.var_t2);
        let assign11510_e6211: f64 = (1.0 + assign11510_e6210);
        let assign11510_e6212: f64 = (locals.var_nsubpp * assign11510_e6211);
        let assign11510_e6216: f64 = (locals.var_t1 * locals.var_t3);
        let assign11510_e6217: f64 = (1.0 + assign11510_e6216);
        let assign11510_e6218: f64 = (assign11510_e6212 / assign11510_e6217);
        (assign11510_e6218, (((((locals.var_nsubpp_dn0 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn0 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn0)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn0 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn0)))) / (assign11510_e6217 * assign11510_e6217)), (((((locals.var_nsubpp_dn2 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn2 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn2)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn2 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn2)))) / (assign11510_e6217 * assign11510_e6217)), (((((locals.var_nsubpp_dn4 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn4 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn4)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn4 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn4)))) / (assign11510_e6217 * assign11510_e6217)), (((((locals.var_nsubpp_dn5 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn5 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn5)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn5 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn5)))) / (assign11510_e6217 * assign11510_e6217)), (((((locals.var_nsubpp_dn6 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn6 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn6)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn6 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn6)))) / (assign11510_e6217 * assign11510_e6217)), (((((locals.var_nsubpp_dn7 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn7 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn7)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn7 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn7)))) / (assign11510_e6217 * assign11510_e6217)), (((((locals.var_nsubpp_dn8 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn8 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn8)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn8 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn8)))) / (assign11510_e6217 * assign11510_e6217)), (((((locals.var_nsubpp_dn9 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn9 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn9)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn9 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn9)))) / (assign11510_e6217 * assign11510_e6217)), (((((locals.var_nsubpp_dn10 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn10 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn10)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn10 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn10)))) / (assign11510_e6217 * assign11510_e6217)), (((((locals.var_nsubpp_dn11 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn11 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn11)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn11 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn11)))) / (assign11510_e6217 * assign11510_e6217)), (((((locals.var_nsubpp_dn14 * assign11510_e6211) + (locals.var_nsubpp * ((locals.var_t1_dn14 * locals.var_t2) + (locals.var_t1 * locals.var_t2_dn14)))) * assign11510_e6217) - (assign11510_e6212 * ((locals.var_t1_dn14 * locals.var_t3) + (locals.var_t1 * locals.var_t3_dn14)))) / (assign11510_e6217 * assign11510_e6217)),)
    } else {
        (locals.var_nsubps, locals.var_nsubps_dn0, locals.var_nsubps_dn2, locals.var_nsubps_dn4, locals.var_nsubps_dn5, locals.var_nsubps_dn6, locals.var_nsubps_dn7, locals.var_nsubps_dn8, locals.var_nsubps_dn9, locals.var_nsubps_dn10, locals.var_nsubps_dn11, locals.var_nsubps_dn14,)
    }
};
        locals.var_nsubps = assign11510_e6220;
        locals.var_nsubps_dn0 = assign11510_e6220_d_n0;
        locals.var_nsubps_dn2 = assign11510_e6220_d_n2;
        locals.var_nsubps_dn4 = assign11510_e6220_d_n4;
        locals.var_nsubps_dn5 = assign11510_e6220_d_n5;
        locals.var_nsubps_dn6 = assign11510_e6220_d_n6;
        locals.var_nsubps_dn7 = assign11510_e6220_d_n7;
        locals.var_nsubps_dn8 = assign11510_e6220_d_n8;
        locals.var_nsubps_dn9 = assign11510_e6220_d_n9;
        locals.var_nsubps_dn10 = assign11510_e6220_d_n10;
        locals.var_nsubps_dn11 = assign11510_e6220_d_n11;
        locals.var_nsubps_dn14 = assign11510_e6220_d_n14;

    }

    pub(super) fn stamp_transient_block_18(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11520_e6225, assign11520_e6225_d_n0, assign11520_e6225_d_n2, assign11520_e6225_d_n4, assign11520_e6225_d_n5, assign11520_e6225_d_n6, assign11520_e6225_d_n7, assign11520_e6225_d_n8, assign11520_e6225_d_n9, assign11520_e6225_d_n10, assign11520_e6225_d_n11, assign11520_e6225_d_n14,) = {
    if (locals.var_guard268 == 0.0) {
        (locals.var_nsubpp, locals.var_nsubpp_dn0, locals.var_nsubpp_dn2, locals.var_nsubpp_dn4, locals.var_nsubpp_dn5, locals.var_nsubpp_dn6, locals.var_nsubpp_dn7, locals.var_nsubpp_dn8, locals.var_nsubpp_dn9, locals.var_nsubpp_dn10, locals.var_nsubpp_dn11, locals.var_nsubpp_dn14,)
    } else {
        (locals.var_nsubps, locals.var_nsubps_dn0, locals.var_nsubps_dn2, locals.var_nsubps_dn4, locals.var_nsubps_dn5, locals.var_nsubps_dn6, locals.var_nsubps_dn7, locals.var_nsubps_dn8, locals.var_nsubps_dn9, locals.var_nsubps_dn10, locals.var_nsubps_dn11, locals.var_nsubps_dn14,)
    }
};
        locals.var_nsubps = assign11520_e6225;
        locals.var_nsubps_dn0 = assign11520_e6225_d_n0;
        locals.var_nsubps_dn2 = assign11520_e6225_d_n2;
        locals.var_nsubps_dn4 = assign11520_e6225_d_n4;
        locals.var_nsubps_dn5 = assign11520_e6225_d_n5;
        locals.var_nsubps_dn6 = assign11520_e6225_d_n6;
        locals.var_nsubps_dn7 = assign11520_e6225_d_n7;
        locals.var_nsubps_dn8 = assign11520_e6225_d_n8;
        locals.var_nsubps_dn9 = assign11520_e6225_d_n9;
        locals.var_nsubps_dn10 = assign11520_e6225_d_n10;
        locals.var_nsubps_dn11 = assign11520_e6225_d_n11;
        locals.var_nsubps_dn14 = assign11520_e6225_d_n14;

        let assign11530_e6232: f64 = if ((locals.var_lgate > p.p140) || (p.p140 <= 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard269 = assign11530_e6232;

        let (assign11540_e6246, assign11540_e6246_d_n0, assign11540_e6246_d_n2, assign11540_e6246_d_n4, assign11540_e6246_d_n5, assign11540_e6246_d_n6, assign11540_e6246_d_n7, assign11540_e6246_d_n8, assign11540_e6246_d_n9, assign11540_e6246_d_n10, assign11540_e6246_d_n11, assign11540_e6246_d_n14,) = {
    if (locals.var_guard269 != 0.0) {
        let assign11540_e6237: f64 = (locals.var_lgate - p.p140);
        let assign11540_e6238: f64 = (locals.var_ef_nsubc * assign11540_e6237);
        let assign11540_e6241: f64 = (locals.var_nsubps * p.p140);
        let assign11540_e6242: f64 = (assign11540_e6238 + assign11540_e6241);
        let assign11540_e6244: f64 = (assign11540_e6242 / locals.var_lgate);
        (assign11540_e6244, (((locals.var_ef_nsubc_dn0 * assign11540_e6237) + (locals.var_nsubps_dn0 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn2 * assign11540_e6237) + (locals.var_nsubps_dn2 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn4 * assign11540_e6237) + (locals.var_nsubps_dn4 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn5 * assign11540_e6237) + (locals.var_nsubps_dn5 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn6 * assign11540_e6237) + (locals.var_nsubps_dn6 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn7 * assign11540_e6237) + (locals.var_nsubps_dn7 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn8 * assign11540_e6237) + (locals.var_nsubps_dn8 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn9 * assign11540_e6237) + (locals.var_nsubps_dn9 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn10 * assign11540_e6237) + (locals.var_nsubps_dn10 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn11 * assign11540_e6237) + (locals.var_nsubps_dn11 * p.p140)) / locals.var_lgate), (((locals.var_ef_nsubc_dn14 * assign11540_e6237) + (locals.var_nsubps_dn14 * p.p140)) / locals.var_lgate),)
    } else {
        (locals.var_nsub, locals.var_nsub_dn0, locals.var_nsub_dn2, locals.var_nsub_dn4, locals.var_nsub_dn5, locals.var_nsub_dn6, locals.var_nsub_dn7, locals.var_nsub_dn8, locals.var_nsub_dn9, locals.var_nsub_dn10, locals.var_nsub_dn11, locals.var_nsub_dn14,)
    }
};
        locals.var_nsub = assign11540_e6246;
        locals.var_nsub_dn0 = assign11540_e6246_d_n0;
        locals.var_nsub_dn2 = assign11540_e6246_d_n2;
        locals.var_nsub_dn4 = assign11540_e6246_d_n4;
        locals.var_nsub_dn5 = assign11540_e6246_d_n5;
        locals.var_nsub_dn6 = assign11540_e6246_d_n6;
        locals.var_nsub_dn7 = assign11540_e6246_d_n7;
        locals.var_nsub_dn8 = assign11540_e6246_d_n8;
        locals.var_nsub_dn9 = assign11540_e6246_d_n9;
        locals.var_nsub_dn10 = assign11540_e6246_d_n10;
        locals.var_nsub_dn11 = assign11540_e6246_d_n11;
        locals.var_nsub_dn14 = assign11540_e6246_d_n14;

        let (assign11550_e6261, assign11550_e6261_d_n0, assign11550_e6261_d_n2, assign11550_e6261_d_n4, assign11550_e6261_d_n5, assign11550_e6261_d_n6, assign11550_e6261_d_n7, assign11550_e6261_d_n8, assign11550_e6261_d_n9, assign11550_e6261_d_n10, assign11550_e6261_d_n11, assign11550_e6261_d_n14,) = {
    if (locals.var_guard269 == 0.0) {
        let assign11550_e6252: f64 = (locals.var_nsubps - locals.var_ef_nsubc);
        let assign11550_e6255: f64 = (p.p140 - locals.var_lgate);
        let assign11550_e6256: f64 = (assign11550_e6252 * assign11550_e6255);
        let assign11550_e6258: f64 = (assign11550_e6256 / p.p140);
        let assign11550_e6259: f64 = (locals.var_nsubps + assign11550_e6258);
        (assign11550_e6259, (locals.var_nsubps_dn0 + (((locals.var_nsubps_dn0 - locals.var_ef_nsubc_dn0) * assign11550_e6255) / p.p140)), (locals.var_nsubps_dn2 + (((locals.var_nsubps_dn2 - locals.var_ef_nsubc_dn2) * assign11550_e6255) / p.p140)), (locals.var_nsubps_dn4 + (((locals.var_nsubps_dn4 - locals.var_ef_nsubc_dn4) * assign11550_e6255) / p.p140)), (locals.var_nsubps_dn5 + (((locals.var_nsubps_dn5 - locals.var_ef_nsubc_dn5) * assign11550_e6255) / p.p140)), (locals.var_nsubps_dn6 + (((locals.var_nsubps_dn6 - locals.var_ef_nsubc_dn6) * assign11550_e6255) / p.p140)), (locals.var_nsubps_dn7 + (((locals.var_nsubps_dn7 - locals.var_ef_nsubc_dn7) * assign11550_e6255) / p.p140)), (locals.var_nsubps_dn8 + (((locals.var_nsubps_dn8 - locals.var_ef_nsubc_dn8) * assign11550_e6255) / p.p140)), (locals.var_nsubps_dn9 + (((locals.var_nsubps_dn9 - locals.var_ef_nsubc_dn9) * assign11550_e6255) / p.p140)), (locals.var_nsubps_dn10 + (((locals.var_nsubps_dn10 - locals.var_ef_nsubc_dn10) * assign11550_e6255) / p.p140)), (locals.var_nsubps_dn11 + (((locals.var_nsubps_dn11 - locals.var_ef_nsubc_dn11) * assign11550_e6255) / p.p140)), (locals.var_nsubps_dn14 + (((locals.var_nsubps_dn14 - locals.var_ef_nsubc_dn14) * assign11550_e6255) / p.p140)),)
    } else {
        (locals.var_nsub, locals.var_nsub_dn0, locals.var_nsub_dn2, locals.var_nsub_dn4, locals.var_nsub_dn5, locals.var_nsub_dn6, locals.var_nsub_dn7, locals.var_nsub_dn8, locals.var_nsub_dn9, locals.var_nsub_dn10, locals.var_nsub_dn11, locals.var_nsub_dn14,)
    }
};
        locals.var_nsub = assign11550_e6261;
        locals.var_nsub_dn0 = assign11550_e6261_d_n0;
        locals.var_nsub_dn2 = assign11550_e6261_d_n2;
        locals.var_nsub_dn4 = assign11550_e6261_d_n4;
        locals.var_nsub_dn5 = assign11550_e6261_d_n5;
        locals.var_nsub_dn6 = assign11550_e6261_d_n6;
        locals.var_nsub_dn7 = assign11550_e6261_d_n7;
        locals.var_nsub_dn8 = assign11550_e6261_d_n8;
        locals.var_nsub_dn9 = assign11550_e6261_d_n9;
        locals.var_nsub_dn10 = assign11550_e6261_d_n10;
        locals.var_nsub_dn11 = assign11550_e6261_d_n11;
        locals.var_nsub_dn14 = assign11550_e6261_d_n14;

        let assign11560_e6264: f64 = (0.5 * locals.var_lgate);
        let assign11560_e6266: f64 = (assign11560_e6264 - p.p140);
        locals.var_t3 = assign11560_e6266;
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

        let assign11570_e6269: f64 = (locals.var_t3 - 1e-9);
        let assign11570_e6271: f64 = (assign11570_e6269 - 1e-10);
        locals.var_tmf1 = assign11570_e6271;
        locals.var_tmf1_dn0 = locals.var_t3_dn0;
        locals.var_tmf1_dn2 = locals.var_t3_dn2;
        locals.var_tmf1_dn4 = locals.var_t3_dn4;
        locals.var_tmf1_dn5 = locals.var_t3_dn5;
        locals.var_tmf1_dn6 = locals.var_t3_dn6;
        locals.var_tmf1_dn7 = locals.var_t3_dn7;
        locals.var_tmf1_dn8 = locals.var_t3_dn8;
        locals.var_tmf1_dn9 = locals.var_t3_dn9;
        locals.var_tmf1_dn10 = locals.var_t3_dn10;
        locals.var_tmf1_dn11 = locals.var_t3_dn11;
        locals.var_tmf1_dn14 = locals.var_t3_dn14;

        let assign11580_e6274: f64 = (4.0 * 1e-9);
        let assign11580_e6276: f64 = (assign11580_e6274 * 1e-10);
        locals.var_tmf2 = assign11580_e6276;
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

        let (assign11590_e6283, assign11590_e6283_d_n0, assign11590_e6283_d_n2, assign11590_e6283_d_n4, assign11590_e6283_d_n5, assign11590_e6283_d_n6, assign11590_e6283_d_n7, assign11590_e6283_d_n8, assign11590_e6283_d_n9, assign11590_e6283_d_n10, assign11590_e6283_d_n11, assign11590_e6283_d_n14,) = {
    if (locals.var_tmf2 > 0.0) {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    } else {
        let assign11590_e6282: f64 = (-locals.var_tmf2);
        (assign11590_e6282, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
    }
};
        locals.var_tmf2 = assign11590_e6283;
        locals.var_tmf2_dn0 = assign11590_e6283_d_n0;
        locals.var_tmf2_dn2 = assign11590_e6283_d_n2;
        locals.var_tmf2_dn4 = assign11590_e6283_d_n4;
        locals.var_tmf2_dn5 = assign11590_e6283_d_n5;
        locals.var_tmf2_dn6 = assign11590_e6283_d_n6;
        locals.var_tmf2_dn7 = assign11590_e6283_d_n7;
        locals.var_tmf2_dn8 = assign11590_e6283_d_n8;
        locals.var_tmf2_dn9 = assign11590_e6283_d_n9;
        locals.var_tmf2_dn10 = assign11590_e6283_d_n10;
        locals.var_tmf2_dn11 = assign11590_e6283_d_n11;
        locals.var_tmf2_dn14 = assign11590_e6283_d_n14;

        let assign11600_e6286: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign11600_e6288: f64 = (assign11600_e6286 + locals.var_tmf2);
        let assign11600_e6289: f64 = (assign11600_e6288).sqrt();
        locals.var_tmf2 = assign11600_e6289;
        locals.var_tmf2_dn0 = ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign11600_e6289));
        locals.var_tmf2_dn2 = ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign11600_e6289));
        locals.var_tmf2_dn4 = ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign11600_e6289));
        locals.var_tmf2_dn5 = ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign11600_e6289));
        locals.var_tmf2_dn6 = ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign11600_e6289));
        locals.var_tmf2_dn7 = ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign11600_e6289));
        locals.var_tmf2_dn8 = ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign11600_e6289));
        locals.var_tmf2_dn9 = ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign11600_e6289));
        locals.var_tmf2_dn10 = ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign11600_e6289));
        locals.var_tmf2_dn11 = ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign11600_e6289));
        locals.var_tmf2_dn14 = ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign11600_e6289));

        let assign11610_e6294: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign11610_e6295: f64 = (1.0 + assign11610_e6294);
        let assign11610_e6296: f64 = (0.5 * assign11610_e6295);
        locals.var_t0 = assign11610_e6296;
        locals.var_t0_dn0 = (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn2 = (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn4 = (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn5 = (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn6 = (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn7 = (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn8 = (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn9 = (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn10 = (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn11 = (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2)));
        locals.var_t0_dn14 = (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2)));

        let assign11620_e6301: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign11620_e6302: f64 = (0.5 * assign11620_e6301);
        let assign11620_e6303: f64 = (1e-9 + assign11620_e6302);
        locals.var_t3 = assign11620_e6303;
        locals.var_t3_dn0 = (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0));
        locals.var_t3_dn2 = (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2));
        locals.var_t3_dn4 = (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4));
        locals.var_t3_dn5 = (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5));
        locals.var_t3_dn6 = (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6));
        locals.var_t3_dn7 = (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7));
        locals.var_t3_dn8 = (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8));
        locals.var_t3_dn9 = (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9));
        locals.var_t3_dn10 = (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10));
        locals.var_t3_dn11 = (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11));
        locals.var_t3_dn14 = (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14));

        let assign11630_e6307: f64 = (1.0 / locals.var_t3);
        let assign11630_e6310: f64 = (1.0 / p.p220);
        let assign11630_e6311: f64 = (assign11630_e6307 + assign11630_e6310);
        let assign11630_e6312: f64 = (1.0 / assign11630_e6311);
        locals.var_t1 = assign11630_e6312;
        locals.var_t1_dn0 = (-((-(locals.var_t3_dn0 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        locals.var_t1_dn2 = (-((-(locals.var_t3_dn2 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        locals.var_t1_dn4 = (-((-(locals.var_t3_dn4 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        locals.var_t1_dn5 = (-((-(locals.var_t3_dn5 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        locals.var_t1_dn6 = (-((-(locals.var_t3_dn6 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        locals.var_t1_dn7 = (-((-(locals.var_t3_dn7 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        locals.var_t1_dn8 = (-((-(locals.var_t3_dn8 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        locals.var_t1_dn9 = (-((-(locals.var_t3_dn9 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        locals.var_t1_dn10 = (-((-(locals.var_t3_dn10 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        locals.var_t1_dn11 = (-((-(locals.var_t3_dn11 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));
        locals.var_t1_dn14 = (-((-(locals.var_t3_dn14 / (locals.var_t3 * locals.var_t3))) / (assign11630_e6311 * assign11630_e6311)));

        let (assign11640_e6318, assign11640_e6318_d_n0, assign11640_e6318_d_n2, assign11640_e6318_d_n4, assign11640_e6318_d_n5, assign11640_e6318_d_n6, assign11640_e6318_d_n7, assign11640_e6318_d_n8, assign11640_e6318_d_n9, assign11640_e6318_d_n10, assign11640_e6318_d_n11, assign11640_e6318_d_n14,) = {
    if (0.0 >= locals.var_t1) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t2 = assign11640_e6318;
        locals.var_t2_dn0 = assign11640_e6318_d_n0;
        locals.var_t2_dn2 = assign11640_e6318_d_n2;
        locals.var_t2_dn4 = assign11640_e6318_d_n4;
        locals.var_t2_dn5 = assign11640_e6318_d_n5;
        locals.var_t2_dn6 = assign11640_e6318_d_n6;
        locals.var_t2_dn7 = assign11640_e6318_d_n7;
        locals.var_t2_dn8 = assign11640_e6318_d_n8;
        locals.var_t2_dn9 = assign11640_e6318_d_n9;
        locals.var_t2_dn10 = assign11640_e6318_d_n10;
        locals.var_t2_dn11 = assign11640_e6318_d_n11;
        locals.var_t2_dn14 = assign11640_e6318_d_n14;

        let assign11650_e6323: f64 = (locals.var_npexte - locals.var_ef_nsubc);
        let assign11650_e6324: f64 = (locals.var_t2 * assign11650_e6323);
        let assign11650_e6326: f64 = (assign11650_e6324 / locals.var_lgate);
        let assign11650_e6327: f64 = (locals.var_nsub + assign11650_e6326);
        locals.var_nsub = assign11650_e6327;
        locals.var_nsub_dn0 = (locals.var_nsub_dn0 + (((locals.var_t2_dn0 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn0 - locals.var_ef_nsubc_dn0))) / locals.var_lgate));
        locals.var_nsub_dn2 = (locals.var_nsub_dn2 + (((locals.var_t2_dn2 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn2 - locals.var_ef_nsubc_dn2))) / locals.var_lgate));
        locals.var_nsub_dn4 = (locals.var_nsub_dn4 + (((locals.var_t2_dn4 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn4 - locals.var_ef_nsubc_dn4))) / locals.var_lgate));
        locals.var_nsub_dn5 = (locals.var_nsub_dn5 + (((locals.var_t2_dn5 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn5 - locals.var_ef_nsubc_dn5))) / locals.var_lgate));
        locals.var_nsub_dn6 = (locals.var_nsub_dn6 + (((locals.var_t2_dn6 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn6 - locals.var_ef_nsubc_dn6))) / locals.var_lgate));
        locals.var_nsub_dn7 = (locals.var_nsub_dn7 + (((locals.var_t2_dn7 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn7 - locals.var_ef_nsubc_dn7))) / locals.var_lgate));
        locals.var_nsub_dn8 = (locals.var_nsub_dn8 + (((locals.var_t2_dn8 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn8 - locals.var_ef_nsubc_dn8))) / locals.var_lgate));
        locals.var_nsub_dn9 = (locals.var_nsub_dn9 + (((locals.var_t2_dn9 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn9 - locals.var_ef_nsubc_dn9))) / locals.var_lgate));
        locals.var_nsub_dn10 = (locals.var_nsub_dn10 + (((locals.var_t2_dn10 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn10 - locals.var_ef_nsubc_dn10))) / locals.var_lgate));
        locals.var_nsub_dn11 = (locals.var_nsub_dn11 + (((locals.var_t2_dn11 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn11 - locals.var_ef_nsubc_dn11))) / locals.var_lgate));
        locals.var_nsub_dn14 = (locals.var_nsub_dn14 + (((locals.var_t2_dn14 * assign11650_e6323) + (locals.var_t2 * (locals.var_npexte_dn14 - locals.var_ef_nsubc_dn14))) / locals.var_lgate));

        let assign11660_e6330: f64 = (1.6021918e-19 * locals.var_nsub);
        locals.var_q_nsub = assign11660_e6330;
        locals.var_q_nsub_dn0 = (1.6021918e-19 * locals.var_nsub_dn0);
        locals.var_q_nsub_dn2 = (1.6021918e-19 * locals.var_nsub_dn2);
        locals.var_q_nsub_dn4 = (1.6021918e-19 * locals.var_nsub_dn4);
        locals.var_q_nsub_dn5 = (1.6021918e-19 * locals.var_nsub_dn5);
        locals.var_q_nsub_dn6 = (1.6021918e-19 * locals.var_nsub_dn6);
        locals.var_q_nsub_dn7 = (1.6021918e-19 * locals.var_nsub_dn7);
        locals.var_q_nsub_dn8 = (1.6021918e-19 * locals.var_nsub_dn8);
        locals.var_q_nsub_dn9 = (1.6021918e-19 * locals.var_nsub_dn9);
        locals.var_q_nsub_dn10 = (1.6021918e-19 * locals.var_nsub_dn10);
        locals.var_q_nsub_dn11 = (1.6021918e-19 * locals.var_nsub_dn11);
        locals.var_q_nsub_dn14 = (1.6021918e-19 * locals.var_nsub_dn14);

        let assign11670_e6333: f64 = (locals.var_q_nsub * 1.034943e-10);
        locals.var_qnsub_esi = assign11670_e6333;
        locals.var_qnsub_esi_dn0 = (locals.var_q_nsub_dn0 * 1.034943e-10);
        locals.var_qnsub_esi_dn2 = (locals.var_q_nsub_dn2 * 1.034943e-10);
        locals.var_qnsub_esi_dn4 = (locals.var_q_nsub_dn4 * 1.034943e-10);
        locals.var_qnsub_esi_dn5 = (locals.var_q_nsub_dn5 * 1.034943e-10);
        locals.var_qnsub_esi_dn6 = (locals.var_q_nsub_dn6 * 1.034943e-10);
        locals.var_qnsub_esi_dn7 = (locals.var_q_nsub_dn7 * 1.034943e-10);
        locals.var_qnsub_esi_dn8 = (locals.var_q_nsub_dn8 * 1.034943e-10);
        locals.var_qnsub_esi_dn9 = (locals.var_q_nsub_dn9 * 1.034943e-10);
        locals.var_qnsub_esi_dn10 = (locals.var_q_nsub_dn10 * 1.034943e-10);
        locals.var_qnsub_esi_dn11 = (locals.var_q_nsub_dn11 * 1.034943e-10);
        locals.var_qnsub_esi_dn14 = (locals.var_q_nsub_dn14 * 1.034943e-10);

        let assign11680_e6336: f64 = (2.0 * locals.var_qnsub_esi);
        locals.var_qnsub_esi2 = assign11680_e6336;
        locals.var_qnsub_esi2_dn0 = (2.0 * locals.var_qnsub_esi_dn0);
        locals.var_qnsub_esi2_dn2 = (2.0 * locals.var_qnsub_esi_dn2);
        locals.var_qnsub_esi2_dn4 = (2.0 * locals.var_qnsub_esi_dn4);
        locals.var_qnsub_esi2_dn5 = (2.0 * locals.var_qnsub_esi_dn5);
        locals.var_qnsub_esi2_dn6 = (2.0 * locals.var_qnsub_esi_dn6);
        locals.var_qnsub_esi2_dn7 = (2.0 * locals.var_qnsub_esi_dn7);
        locals.var_qnsub_esi2_dn8 = (2.0 * locals.var_qnsub_esi_dn8);
        locals.var_qnsub_esi2_dn9 = (2.0 * locals.var_qnsub_esi_dn9);
        locals.var_qnsub_esi2_dn10 = (2.0 * locals.var_qnsub_esi_dn10);
        locals.var_qnsub_esi2_dn11 = (2.0 * locals.var_qnsub_esi_dn11);
        locals.var_qnsub_esi2_dn14 = (2.0 * locals.var_qnsub_esi_dn14);

        let assign11690_e6340: f64 = (2.0 * p.p140);
        let assign11690_e6345: f64 = if ((locals.var_lgate <= assign11690_e6340) && (p.p140 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard270 = assign11690_e6345;

        let (assign11700_e6361, assign11700_e6361_d_n0, assign11700_e6361_d_n2, assign11700_e6361_d_n4, assign11700_e6361_d_n5, assign11700_e6361_d_n6, assign11700_e6361_d_n7, assign11700_e6361_d_n8, assign11700_e6361_d_n9, assign11700_e6361_d_n10, assign11700_e6361_d_n11, assign11700_e6361_d_n14,) = {
    if (locals.var_guard270 != 0.0) {
        let assign11700_e6349: f64 = (2.0 * locals.var_nsubps);
        let assign11700_e6352: f64 = (locals.var_nsubps - locals.var_ef_nsubc);
        let assign11700_e6354: f64 = (assign11700_e6352 * locals.var_lgate);
        let assign11700_e6356: f64 = (assign11700_e6354 / p.p140);
        let assign11700_e6357: f64 = (assign11700_e6349 - assign11700_e6356);
        let assign11700_e6359: f64 = (assign11700_e6357 - locals.var_ef_nsubc);
        (assign11700_e6359, (((2.0 * locals.var_nsubps_dn0) - (((locals.var_nsubps_dn0 - locals.var_ef_nsubc_dn0) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn0), (((2.0 * locals.var_nsubps_dn2) - (((locals.var_nsubps_dn2 - locals.var_ef_nsubc_dn2) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn2), (((2.0 * locals.var_nsubps_dn4) - (((locals.var_nsubps_dn4 - locals.var_ef_nsubc_dn4) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn4), (((2.0 * locals.var_nsubps_dn5) - (((locals.var_nsubps_dn5 - locals.var_ef_nsubc_dn5) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn5), (((2.0 * locals.var_nsubps_dn6) - (((locals.var_nsubps_dn6 - locals.var_ef_nsubc_dn6) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn6), (((2.0 * locals.var_nsubps_dn7) - (((locals.var_nsubps_dn7 - locals.var_ef_nsubc_dn7) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn7), (((2.0 * locals.var_nsubps_dn8) - (((locals.var_nsubps_dn8 - locals.var_ef_nsubc_dn8) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn8), (((2.0 * locals.var_nsubps_dn9) - (((locals.var_nsubps_dn9 - locals.var_ef_nsubc_dn9) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn9), (((2.0 * locals.var_nsubps_dn10) - (((locals.var_nsubps_dn10 - locals.var_ef_nsubc_dn10) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn10), (((2.0 * locals.var_nsubps_dn11) - (((locals.var_nsubps_dn11 - locals.var_ef_nsubc_dn11) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn11), (((2.0 * locals.var_nsubps_dn14) - (((locals.var_nsubps_dn14 - locals.var_ef_nsubc_dn14) * locals.var_lgate) / p.p140)) - locals.var_ef_nsubc_dn14),)
    } else {
        (locals.var_nsubb, locals.var_nsubb_dn0, locals.var_nsubb_dn2, locals.var_nsubb_dn4, locals.var_nsubb_dn5, locals.var_nsubb_dn6, locals.var_nsubb_dn7, locals.var_nsubb_dn8, locals.var_nsubb_dn9, locals.var_nsubb_dn10, locals.var_nsubb_dn11, locals.var_nsubb_dn14,)
    }
};
        locals.var_nsubb = assign11700_e6361;
        locals.var_nsubb_dn0 = assign11700_e6361_d_n0;
        locals.var_nsubb_dn2 = assign11700_e6361_d_n2;
        locals.var_nsubb_dn4 = assign11700_e6361_d_n4;
        locals.var_nsubb_dn5 = assign11700_e6361_d_n5;
        locals.var_nsubb_dn6 = assign11700_e6361_d_n6;
        locals.var_nsubb_dn7 = assign11700_e6361_d_n7;
        locals.var_nsubb_dn8 = assign11700_e6361_d_n8;
        locals.var_nsubb_dn9 = assign11700_e6361_d_n9;
        locals.var_nsubb_dn10 = assign11700_e6361_d_n10;
        locals.var_nsubb_dn11 = assign11700_e6361_d_n11;
        locals.var_nsubb_dn14 = assign11700_e6361_d_n14;

        let (assign11710_e6368, assign11710_e6368_d_n0, assign11710_e6368_d_n2, assign11710_e6368_d_n4, assign11710_e6368_d_n5, assign11710_e6368_d_n6, assign11710_e6368_d_n7, assign11710_e6368_d_n8, assign11710_e6368_d_n9, assign11710_e6368_d_n10, assign11710_e6368_d_n11, assign11710_e6368_d_n14,) = {
    if (locals.var_guard270 != 0.0) {
        let assign11710_e6365: f64 = (locals.var_nsubb / locals.var_ef_nsubc);
        let assign11710_e6366: f64 = (assign11710_e6365).ln();
        (assign11710_e6366, ((((locals.var_nsubb_dn0 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn0)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365), ((((locals.var_nsubb_dn2 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn2)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365), ((((locals.var_nsubb_dn4 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn4)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365), ((((locals.var_nsubb_dn5 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn5)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365), ((((locals.var_nsubb_dn6 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn6)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365), ((((locals.var_nsubb_dn7 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn7)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365), ((((locals.var_nsubb_dn8 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn8)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365), ((((locals.var_nsubb_dn9 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn9)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365), ((((locals.var_nsubb_dn10 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn10)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365), ((((locals.var_nsubb_dn11 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn11)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365), ((((locals.var_nsubb_dn14 * locals.var_ef_nsubc) - (locals.var_nsubb * locals.var_ef_nsubc_dn14)) / (locals.var_ef_nsubc * locals.var_ef_nsubc)) / assign11710_e6365),)
    } else {
        (locals.var_ptovr0, locals.var_ptovr0_dn0, locals.var_ptovr0_dn2, locals.var_ptovr0_dn4, locals.var_ptovr0_dn5, locals.var_ptovr0_dn6, locals.var_ptovr0_dn7, locals.var_ptovr0_dn8, locals.var_ptovr0_dn9, locals.var_ptovr0_dn10, locals.var_ptovr0_dn11, locals.var_ptovr0_dn14,)
    }
};
        locals.var_ptovr0 = assign11710_e6368;
        locals.var_ptovr0_dn0 = assign11710_e6368_d_n0;
        locals.var_ptovr0_dn2 = assign11710_e6368_d_n2;
        locals.var_ptovr0_dn4 = assign11710_e6368_d_n4;
        locals.var_ptovr0_dn5 = assign11710_e6368_d_n5;
        locals.var_ptovr0_dn6 = assign11710_e6368_d_n6;
        locals.var_ptovr0_dn7 = assign11710_e6368_d_n7;
        locals.var_ptovr0_dn8 = assign11710_e6368_d_n8;
        locals.var_ptovr0_dn9 = assign11710_e6368_d_n9;
        locals.var_ptovr0_dn10 = assign11710_e6368_d_n10;
        locals.var_ptovr0_dn11 = assign11710_e6368_d_n11;
        locals.var_ptovr0_dn14 = assign11710_e6368_d_n14;

        let (assign11720_e6373, assign11720_e6373_d_n0, assign11720_e6373_d_n2, assign11720_e6373_d_n4, assign11720_e6373_d_n5, assign11720_e6373_d_n6, assign11720_e6373_d_n7, assign11720_e6373_d_n8, assign11720_e6373_d_n9, assign11720_e6373_d_n10, assign11720_e6373_d_n11, assign11720_e6373_d_n14,) = {
    if (locals.var_guard270 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ptovr0, locals.var_ptovr0_dn0, locals.var_ptovr0_dn2, locals.var_ptovr0_dn4, locals.var_ptovr0_dn5, locals.var_ptovr0_dn6, locals.var_ptovr0_dn7, locals.var_ptovr0_dn8, locals.var_ptovr0_dn9, locals.var_ptovr0_dn10, locals.var_ptovr0_dn11, locals.var_ptovr0_dn14,)
    }
};
        locals.var_ptovr0 = assign11720_e6373;
        locals.var_ptovr0_dn0 = assign11720_e6373_d_n0;
        locals.var_ptovr0_dn2 = assign11720_e6373_d_n2;
        locals.var_ptovr0_dn4 = assign11720_e6373_d_n4;
        locals.var_ptovr0_dn5 = assign11720_e6373_d_n5;
        locals.var_ptovr0_dn6 = assign11720_e6373_d_n6;
        locals.var_ptovr0_dn7 = assign11720_e6373_d_n7;
        locals.var_ptovr0_dn8 = assign11720_e6373_d_n8;
        locals.var_ptovr0_dn9 = assign11720_e6373_d_n9;
        locals.var_ptovr0_dn10 = assign11720_e6373_d_n10;
        locals.var_ptovr0_dn11 = assign11720_e6373_d_n11;
        locals.var_ptovr0_dn14 = assign11720_e6373_d_n14;

        let assign11730_e6376: f64 = (2.0 * 1.6021918e-19);
        let assign11730_e6378: f64 = (assign11730_e6376 * locals.var_uc_nsti);
        let assign11730_e6380: f64 = (assign11730_e6378 * 1.034943e-10);
        let assign11730_e6381: f64 = (assign11730_e6380).sqrt();
        locals.var_costi00 = assign11730_e6381;

        let assign11740_e6385: f64 = (locals.var_uc_nsti * locals.var_uc_nsti);
        let assign11740_e6386: f64 = (1.0 / assign11740_e6385);
        locals.var_nsti_p2 = assign11740_e6386;

        let assign11750_e6391: f64 = (locals.var_lg).powf(p.p231);
        let assign11750_e6392: f64 = (locals.var_uc_vover / assign11750_e6391);
        let assign11750_e6393: f64 = (1.0 + assign11750_e6392);
        let assign11750_e6398: f64 = (locals.var_wlg).powf(p.p239);
        let assign11750_e6399: f64 = (p.p238 / assign11750_e6398);
        let assign11750_e6400: f64 = (1.0 + assign11750_e6399);
        let assign11750_e6401: f64 = (assign11750_e6393 * assign11750_e6400);
        locals.var_vmax0 = assign11750_e6401;

        let assign11760_e6404: f64 = (2.0 / 38.68283);
        let assign11760_e6407: f64 = (locals.var_nsub / 1.04e16);
        let assign11760_e6408: f64 = (assign11760_e6407).ln();
        let assign11760_e6409: f64 = (assign11760_e6404 * assign11760_e6408);
        locals.var_pb20 = assign11760_e6409;
        locals.var_pb20_dn0 = (assign11760_e6404 * ((locals.var_nsub_dn0 / 1.04e16) / assign11760_e6407));
        locals.var_pb20_dn2 = (assign11760_e6404 * ((locals.var_nsub_dn2 / 1.04e16) / assign11760_e6407));
        locals.var_pb20_dn4 = (assign11760_e6404 * ((locals.var_nsub_dn4 / 1.04e16) / assign11760_e6407));
        locals.var_pb20_dn5 = (assign11760_e6404 * ((locals.var_nsub_dn5 / 1.04e16) / assign11760_e6407));
        locals.var_pb20_dn6 = (assign11760_e6404 * ((locals.var_nsub_dn6 / 1.04e16) / assign11760_e6407));
        locals.var_pb20_dn7 = (assign11760_e6404 * ((locals.var_nsub_dn7 / 1.04e16) / assign11760_e6407));
        locals.var_pb20_dn8 = (assign11760_e6404 * ((locals.var_nsub_dn8 / 1.04e16) / assign11760_e6407));
        locals.var_pb20_dn9 = (assign11760_e6404 * ((locals.var_nsub_dn9 / 1.04e16) / assign11760_e6407));
        locals.var_pb20_dn10 = (assign11760_e6404 * ((locals.var_nsub_dn10 / 1.04e16) / assign11760_e6407));
        locals.var_pb20_dn11 = (assign11760_e6404 * ((locals.var_nsub_dn11 / 1.04e16) / assign11760_e6407));
        locals.var_pb20_dn14 = (assign11760_e6404 * ((locals.var_nsub_dn14 / 1.04e16) / assign11760_e6407));

        let assign11770_e6412: f64 = (2.0 / 38.68283);
        let assign11770_e6415: f64 = (locals.var_ef_nsubc / 1.04e16);
        let assign11770_e6416: f64 = (assign11770_e6415).ln();
        let assign11770_e6417: f64 = (assign11770_e6412 * assign11770_e6416);
        locals.var_pb2c = assign11770_e6417;
        locals.var_pb2c_dn0 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn0 / 1.04e16) / assign11770_e6415));
        locals.var_pb2c_dn2 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn2 / 1.04e16) / assign11770_e6415));
        locals.var_pb2c_dn4 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn4 / 1.04e16) / assign11770_e6415));
        locals.var_pb2c_dn5 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn5 / 1.04e16) / assign11770_e6415));
        locals.var_pb2c_dn6 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn6 / 1.04e16) / assign11770_e6415));
        locals.var_pb2c_dn7 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn7 / 1.04e16) / assign11770_e6415));
        locals.var_pb2c_dn8 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn8 / 1.04e16) / assign11770_e6415));
        locals.var_pb2c_dn9 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn9 / 1.04e16) / assign11770_e6415));
        locals.var_pb2c_dn10 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn10 / 1.04e16) / assign11770_e6415));
        locals.var_pb2c_dn11 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn11 / 1.04e16) / assign11770_e6415));
        locals.var_pb2c_dn14 = (assign11770_e6412 * ((locals.var_ef_nsubc_dn14 / 1.04e16) / assign11770_e6415));

        let assign11780_e6420: f64 = if p.p51 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard271 = assign11780_e6420;

        let (assign11790_e6430, assign11790_e6430_d_n0, assign11790_e6430_d_n2, assign11790_e6430_d_n4, assign11790_e6430_d_n5, assign11790_e6430_d_n6, assign11790_e6430_d_n7, assign11790_e6430_d_n8, assign11790_e6430_d_n9, assign11790_e6430_d_n10, assign11790_e6430_d_n11, assign11790_e6430_d_n14,) = {
    if (locals.var_guard271 != 0.0) {
        let assign11790_e6426: f64 = (3.0 * p.p4);
        let assign11790_e6427: f64 = (locals.var_weff / assign11790_e6426);
        let assign11790_e6428: f64 = (p.p5 + assign11790_e6427);
        (assign11790_e6428, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign11790_e6430;
        locals.var_t1_dn0 = assign11790_e6430_d_n0;
        locals.var_t1_dn2 = assign11790_e6430_d_n2;
        locals.var_t1_dn4 = assign11790_e6430_d_n4;
        locals.var_t1_dn5 = assign11790_e6430_d_n5;
        locals.var_t1_dn6 = assign11790_e6430_d_n6;
        locals.var_t1_dn7 = assign11790_e6430_d_n7;
        locals.var_t1_dn8 = assign11790_e6430_d_n8;
        locals.var_t1_dn9 = assign11790_e6430_d_n9;
        locals.var_t1_dn10 = assign11790_e6430_d_n10;
        locals.var_t1_dn11 = assign11790_e6430_d_n11;
        locals.var_t1_dn14 = assign11790_e6430_d_n14;

        let (assign11800_e6436, assign11800_e6436_d_n0, assign11800_e6436_d_n2, assign11800_e6436_d_n4, assign11800_e6436_d_n5, assign11800_e6436_d_n6, assign11800_e6436_d_n7, assign11800_e6436_d_n8, assign11800_e6436_d_n9, assign11800_e6436_d_n10, assign11800_e6436_d_n11, assign11800_e6436_d_n14,) = {
    if (locals.var_guard271 != 0.0) {
        let assign11800_e6434: f64 = (locals.var_lgate - p.p6);
        (assign11800_e6434, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign11800_e6436;
        locals.var_t2_dn0 = assign11800_e6436_d_n0;
        locals.var_t2_dn2 = assign11800_e6436_d_n2;
        locals.var_t2_dn4 = assign11800_e6436_d_n4;
        locals.var_t2_dn5 = assign11800_e6436_d_n5;
        locals.var_t2_dn6 = assign11800_e6436_d_n6;
        locals.var_t2_dn7 = assign11800_e6436_d_n7;
        locals.var_t2_dn8 = assign11800_e6436_d_n8;
        locals.var_t2_dn9 = assign11800_e6436_d_n9;
        locals.var_t2_dn10 = assign11800_e6436_d_n10;
        locals.var_t2_dn11 = assign11800_e6436_d_n11;
        locals.var_t2_dn14 = assign11800_e6436_d_n14;

        let assign11860_e6478: f64 = if p.p130 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard273 = assign11860_e6478;

        let (assign11870_e6484,) = {
    if (locals.var_guard273 != 0.0) {
        let assign11870_e6482: f64 = (p.p130 * p.p2);
        (assign11870_e6482,)
    } else {
        (locals.var_rd0,)
    }
};
        locals.var_rd0 = assign11870_e6484;

        let (assign11880_e6490,) = {
    if (locals.var_guard273 != 0.0) {
        let assign11880_e6488: f64 = (p.p130 * p.p3);
        (assign11880_e6488,)
    } else {
        (locals.var_rs0,)
    }
};
        locals.var_rs0 = assign11880_e6490;

        let (assign11890_e6495,) = {
    if (locals.var_guard273 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rd0,)
    }
};
        locals.var_rd0 = assign11890_e6495;

        let (assign11900_e6500,) = {
    if (locals.var_guard273 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rs0,)
    }
};
        locals.var_rs0 = assign11900_e6500;

        let assign11910_e6503: f64 = if p.p131 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard274 = assign11910_e6503;

        let (assign11920_e6509,) = {
    if (locals.var_guard274 != 0.0) {
        let assign11920_e6507: f64 = (p.p131 * p.p3);
        (assign11920_e6507,)
    } else {
        (locals.var_rs0,)
    }
};
        locals.var_rs0 = assign11920_e6509;

    }

    pub(super) fn stamp_transient_block_19(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign11930_e6514,) = {
    if (locals.var_guard274 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rs0,)
    }
};
        locals.var_rs0 = assign11930_e6514;

        let assign11940_e6517: f64 = if locals.var_uc_cordrift == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard275 = assign11940_e6517;

        let assign11950_e6524: f64 = if ((locals.var_uc_rd > 0.0) || (locals.var_uc_rs > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard276 = assign11950_e6524;

        let (assign11960_e6536,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard276 != 0.0)) {
        let assign11960_e6532: f64 = (locals.var_wlg).powf(p.p310);
        let assign11960_e6533: f64 = (p.p309 / assign11960_e6532);
        let assign11960_e6534: f64 = (1.0 + assign11960_e6533);
        (assign11960_e6534,)
    } else {
        (locals.var_rdtemp0,)
    }
};
        locals.var_rdtemp0 = assign11960_e6536;

        let assign11970_e6539: f64 = if locals.var_uc_rdvd != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard277 = assign11970_e6539;

        let (assign11980_e6553, assign11980_e6553_d_n0, assign11980_e6553_d_n2, assign11980_e6553_d_n4, assign11980_e6553_d_n5, assign11980_e6553_d_n6, assign11980_e6553_d_n7, assign11980_e6553_d_n8, assign11980_e6553_d_n9, assign11980_e6553_d_n10, assign11980_e6553_d_n11, assign11980_e6553_d_n14,) = {
    if (((locals.var_guard275 != 0.0) && (locals.var_guard276 != 0.0)) && (locals.var_guard277 != 0.0)) {
        let assign11980_e6549: f64 = (locals.var_wlg).powf(p.p304);
        let assign11980_e6550: f64 = (p.p303 / assign11980_e6549);
        let assign11980_e6551: f64 = (1.0 + assign11980_e6550);
        (assign11980_e6551, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign11980_e6553;
        locals.var_t7_dn0 = assign11980_e6553_d_n0;
        locals.var_t7_dn2 = assign11980_e6553_d_n2;
        locals.var_t7_dn4 = assign11980_e6553_d_n4;
        locals.var_t7_dn5 = assign11980_e6553_d_n5;
        locals.var_t7_dn6 = assign11980_e6553_d_n6;
        locals.var_t7_dn7 = assign11980_e6553_d_n7;
        locals.var_t7_dn8 = assign11980_e6553_d_n8;
        locals.var_t7_dn9 = assign11980_e6553_d_n9;
        locals.var_t7_dn10 = assign11980_e6553_d_n10;
        locals.var_t7_dn11 = assign11980_e6553_d_n11;
        locals.var_t7_dn14 = assign11980_e6553_d_n14;

        let (assign11990_e6566, assign11990_e6566_d_n0, assign11990_e6566_d_n2, assign11990_e6566_d_n4, assign11990_e6566_d_n5, assign11990_e6566_d_n6, assign11990_e6566_d_n7, assign11990_e6566_d_n8, assign11990_e6566_d_n9, assign11990_e6566_d_n10, assign11990_e6566_d_n11, assign11990_e6566_d_n14,) = {
    if (((locals.var_guard275 != 0.0) && (locals.var_guard276 != 0.0)) && (locals.var_guard277 != 0.0)) {
        let assign11990_e6560: f64 = (-p.p301);
        let assign11990_e6563: f64 = (locals.var_lg).powf(p.p302);
        let assign11990_e6564: f64 = (assign11990_e6560 * assign11990_e6563);
        (assign11990_e6564, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign11990_e6566;
        locals.var_t6_dn0 = assign11990_e6566_d_n0;
        locals.var_t6_dn2 = assign11990_e6566_d_n2;
        locals.var_t6_dn4 = assign11990_e6566_d_n4;
        locals.var_t6_dn5 = assign11990_e6566_d_n5;
        locals.var_t6_dn6 = assign11990_e6566_d_n6;
        locals.var_t6_dn7 = assign11990_e6566_d_n7;
        locals.var_t6_dn8 = assign11990_e6566_d_n8;
        locals.var_t6_dn9 = assign11990_e6566_d_n9;
        locals.var_t6_dn10 = assign11990_e6566_d_n10;
        locals.var_t6_dn11 = assign11990_e6566_d_n11;
        locals.var_t6_dn14 = assign11990_e6566_d_n14;

        let assign12000_e6569: f64 = if locals.var_t6 > 60.0 { 1.0 } else { 0.0 };
        locals.var_guard278 = assign12000_e6569;

        let (assign12010_e6579, assign12010_e6579_d_n0, assign12010_e6579_d_n2, assign12010_e6579_d_n4, assign12010_e6579_d_n5, assign12010_e6579_d_n6, assign12010_e6579_d_n7, assign12010_e6579_d_n8, assign12010_e6579_d_n9, assign12010_e6579_d_n10, assign12010_e6579_d_n11, assign12010_e6579_d_n14,) = {
    if ((((locals.var_guard275 != 0.0) && (locals.var_guard276 != 0.0)) && (locals.var_guard277 != 0.0)) && (locals.var_guard278 != 0.0)) {
        (60.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign12010_e6579;
        locals.var_t6_dn0 = assign12010_e6579_d_n0;
        locals.var_t6_dn2 = assign12010_e6579_d_n2;
        locals.var_t6_dn4 = assign12010_e6579_d_n4;
        locals.var_t6_dn5 = assign12010_e6579_d_n5;
        locals.var_t6_dn6 = assign12010_e6579_d_n6;
        locals.var_t6_dn7 = assign12010_e6579_d_n7;
        locals.var_t6_dn8 = assign12010_e6579_d_n8;
        locals.var_t6_dn9 = assign12010_e6579_d_n9;
        locals.var_t6_dn10 = assign12010_e6579_d_n10;
        locals.var_t6_dn11 = assign12010_e6579_d_n11;
        locals.var_t6_dn14 = assign12010_e6579_d_n14;

        let (assign12020_e6588, assign12020_e6588_d_n0, assign12020_e6588_d_n2, assign12020_e6588_d_n4, assign12020_e6588_d_n5, assign12020_e6588_d_n6, assign12020_e6588_d_n7, assign12020_e6588_d_n8, assign12020_e6588_d_n9, assign12020_e6588_d_n10, assign12020_e6588_d_n11, assign12020_e6588_d_n14,) = {
    if (((locals.var_guard275 != 0.0) && (locals.var_guard276 != 0.0)) && (locals.var_guard277 != 0.0)) {
        let assign12020_e6586: f64 = (locals.var_t6).exp();
        (assign12020_e6586, (assign12020_e6586 * locals.var_t6_dn0), (assign12020_e6586 * locals.var_t6_dn2), (assign12020_e6586 * locals.var_t6_dn4), (assign12020_e6586 * locals.var_t6_dn5), (assign12020_e6586 * locals.var_t6_dn6), (assign12020_e6586 * locals.var_t6_dn7), (assign12020_e6586 * locals.var_t6_dn8), (assign12020_e6586 * locals.var_t6_dn9), (assign12020_e6586 * locals.var_t6_dn10), (assign12020_e6586 * locals.var_t6_dn11), (assign12020_e6586 * locals.var_t6_dn14),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign12020_e6588;
        locals.var_t6_dn0 = assign12020_e6588_d_n0;
        locals.var_t6_dn2 = assign12020_e6588_d_n2;
        locals.var_t6_dn4 = assign12020_e6588_d_n4;
        locals.var_t6_dn5 = assign12020_e6588_d_n5;
        locals.var_t6_dn6 = assign12020_e6588_d_n6;
        locals.var_t6_dn7 = assign12020_e6588_d_n7;
        locals.var_t6_dn8 = assign12020_e6588_d_n8;
        locals.var_t6_dn9 = assign12020_e6588_d_n9;
        locals.var_t6_dn10 = assign12020_e6588_d_n10;
        locals.var_t6_dn11 = assign12020_e6588_d_n11;
        locals.var_t6_dn14 = assign12020_e6588_d_n14;

        let (assign12030_e6598, assign12030_e6598_d_n0, assign12030_e6598_d_n2, assign12030_e6598_d_n4, assign12030_e6598_d_n5, assign12030_e6598_d_n6, assign12030_e6598_d_n7, assign12030_e6598_d_n8, assign12030_e6598_d_n9, assign12030_e6598_d_n10, assign12030_e6598_d_n11, assign12030_e6598_d_n14,) = {
    if (((locals.var_guard275 != 0.0) && (locals.var_guard276 != 0.0)) && (locals.var_guard277 != 0.0)) {
        let assign12030_e6596: f64 = (locals.var_t6 * locals.var_t7);
        (assign12030_e6596, ((locals.var_t6_dn0 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn0)), ((locals.var_t6_dn2 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn2)), ((locals.var_t6_dn4 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn4)), ((locals.var_t6_dn5 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn5)), ((locals.var_t6_dn6 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn6)), ((locals.var_t6_dn7 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn7)), ((locals.var_t6_dn8 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn8)), ((locals.var_t6_dn9 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn9)), ((locals.var_t6_dn10 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn10)), ((locals.var_t6_dn11 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn11)), ((locals.var_t6_dn14 * locals.var_t7) + (locals.var_t6 * locals.var_t7_dn14)),)
    } else {
        (locals.var_rdvdtemp0, locals.var_rdvdtemp0_dn0, locals.var_rdvdtemp0_dn2, locals.var_rdvdtemp0_dn4, locals.var_rdvdtemp0_dn5, locals.var_rdvdtemp0_dn6, locals.var_rdvdtemp0_dn7, locals.var_rdvdtemp0_dn8, locals.var_rdvdtemp0_dn9, locals.var_rdvdtemp0_dn10, locals.var_rdvdtemp0_dn11, locals.var_rdvdtemp0_dn14,)
    }
};
        locals.var_rdvdtemp0 = assign12030_e6598;
        locals.var_rdvdtemp0_dn0 = assign12030_e6598_d_n0;
        locals.var_rdvdtemp0_dn2 = assign12030_e6598_d_n2;
        locals.var_rdvdtemp0_dn4 = assign12030_e6598_d_n4;
        locals.var_rdvdtemp0_dn5 = assign12030_e6598_d_n5;
        locals.var_rdvdtemp0_dn6 = assign12030_e6598_d_n6;
        locals.var_rdvdtemp0_dn7 = assign12030_e6598_d_n7;
        locals.var_rdvdtemp0_dn8 = assign12030_e6598_d_n8;
        locals.var_rdvdtemp0_dn9 = assign12030_e6598_d_n9;
        locals.var_rdvdtemp0_dn10 = assign12030_e6598_d_n10;
        locals.var_rdvdtemp0_dn11 = assign12030_e6598_d_n11;
        locals.var_rdvdtemp0_dn14 = assign12030_e6598_d_n14;

        let (assign12040_e6607, assign12040_e6607_d_n0, assign12040_e6607_d_n2, assign12040_e6607_d_n4, assign12040_e6607_d_n5, assign12040_e6607_d_n6, assign12040_e6607_d_n7, assign12040_e6607_d_n8, assign12040_e6607_d_n9, assign12040_e6607_d_n10, assign12040_e6607_d_n11, assign12040_e6607_d_n14,) = {
    if (((locals.var_guard275 != 0.0) && (locals.var_guard276 != 0.0)) && (locals.var_guard277 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvdtemp0, locals.var_rdvdtemp0_dn0, locals.var_rdvdtemp0_dn2, locals.var_rdvdtemp0_dn4, locals.var_rdvdtemp0_dn5, locals.var_rdvdtemp0_dn6, locals.var_rdvdtemp0_dn7, locals.var_rdvdtemp0_dn8, locals.var_rdvdtemp0_dn9, locals.var_rdvdtemp0_dn10, locals.var_rdvdtemp0_dn11, locals.var_rdvdtemp0_dn14,)
    }
};
        locals.var_rdvdtemp0 = assign12040_e6607;
        locals.var_rdvdtemp0_dn0 = assign12040_e6607_d_n0;
        locals.var_rdvdtemp0_dn2 = assign12040_e6607_d_n2;
        locals.var_rdvdtemp0_dn4 = assign12040_e6607_d_n4;
        locals.var_rdvdtemp0_dn5 = assign12040_e6607_d_n5;
        locals.var_rdvdtemp0_dn6 = assign12040_e6607_d_n6;
        locals.var_rdvdtemp0_dn7 = assign12040_e6607_d_n7;
        locals.var_rdvdtemp0_dn8 = assign12040_e6607_d_n8;
        locals.var_rdvdtemp0_dn9 = assign12040_e6607_d_n9;
        locals.var_rdvdtemp0_dn10 = assign12040_e6607_d_n10;
        locals.var_rdvdtemp0_dn11 = assign12040_e6607_d_n11;
        locals.var_rdvdtemp0_dn14 = assign12040_e6607_d_n14;

        let (assign12050_e6614,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard276 == 0.0)) {
        (0.0,)
    } else {
        (locals.var_rdtemp0,)
    }
};
        locals.var_rdtemp0 = assign12050_e6614;

        let (assign12060_e6621, assign12060_e6621_d_n0, assign12060_e6621_d_n2, assign12060_e6621_d_n4, assign12060_e6621_d_n5, assign12060_e6621_d_n6, assign12060_e6621_d_n7, assign12060_e6621_d_n8, assign12060_e6621_d_n9, assign12060_e6621_d_n10, assign12060_e6621_d_n11, assign12060_e6621_d_n14,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard276 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvdtemp0, locals.var_rdvdtemp0_dn0, locals.var_rdvdtemp0_dn2, locals.var_rdvdtemp0_dn4, locals.var_rdvdtemp0_dn5, locals.var_rdvdtemp0_dn6, locals.var_rdvdtemp0_dn7, locals.var_rdvdtemp0_dn8, locals.var_rdvdtemp0_dn9, locals.var_rdvdtemp0_dn10, locals.var_rdvdtemp0_dn11, locals.var_rdvdtemp0_dn14,)
    }
};
        locals.var_rdvdtemp0 = assign12060_e6621;
        locals.var_rdvdtemp0_dn0 = assign12060_e6621_d_n0;
        locals.var_rdvdtemp0_dn2 = assign12060_e6621_d_n2;
        locals.var_rdvdtemp0_dn4 = assign12060_e6621_d_n4;
        locals.var_rdvdtemp0_dn5 = assign12060_e6621_d_n5;
        locals.var_rdvdtemp0_dn6 = assign12060_e6621_d_n6;
        locals.var_rdvdtemp0_dn7 = assign12060_e6621_d_n7;
        locals.var_rdvdtemp0_dn8 = assign12060_e6621_d_n8;
        locals.var_rdvdtemp0_dn9 = assign12060_e6621_d_n9;
        locals.var_rdvdtemp0_dn10 = assign12060_e6621_d_n10;
        locals.var_rdvdtemp0_dn11 = assign12060_e6621_d_n11;
        locals.var_rdvdtemp0_dn14 = assign12060_e6621_d_n14;

        let assign12070_e6624: f64 = if locals.var_uc_rd23 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard279 = assign12070_e6624;

        let (assign12080_e6636, assign12080_e6636_d_n0, assign12080_e6636_d_n2, assign12080_e6636_d_n4, assign12080_e6636_d_n5, assign12080_e6636_d_n6, assign12080_e6636_d_n7, assign12080_e6636_d_n8, assign12080_e6636_d_n9, assign12080_e6636_d_n10, assign12080_e6636_d_n11, assign12080_e6636_d_n14,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard279 != 0.0)) {
        let assign12080_e6632: f64 = (locals.var_wlg).powf(p.p308);
        let assign12080_e6633: f64 = (p.p307 / assign12080_e6632);
        let assign12080_e6634: f64 = (1.0 + assign12080_e6633);
        (assign12080_e6634, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign12080_e6636;
        locals.var_t2_dn0 = assign12080_e6636_d_n0;
        locals.var_t2_dn2 = assign12080_e6636_d_n2;
        locals.var_t2_dn4 = assign12080_e6636_d_n4;
        locals.var_t2_dn5 = assign12080_e6636_d_n5;
        locals.var_t2_dn6 = assign12080_e6636_d_n6;
        locals.var_t2_dn7 = assign12080_e6636_d_n7;
        locals.var_t2_dn8 = assign12080_e6636_d_n8;
        locals.var_t2_dn9 = assign12080_e6636_d_n9;
        locals.var_t2_dn10 = assign12080_e6636_d_n10;
        locals.var_t2_dn11 = assign12080_e6636_d_n11;
        locals.var_t2_dn14 = assign12080_e6636_d_n14;

        let (assign12090_e6647, assign12090_e6647_d_n0, assign12090_e6647_d_n2, assign12090_e6647_d_n4, assign12090_e6647_d_n5, assign12090_e6647_d_n6, assign12090_e6647_d_n7, assign12090_e6647_d_n8, assign12090_e6647_d_n9, assign12090_e6647_d_n10, assign12090_e6647_d_n11, assign12090_e6647_d_n14,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard279 != 0.0)) {
        let assign12090_e6641: f64 = (-p.p305);
        let assign12090_e6644: f64 = (locals.var_lg).powf(p.p306);
        let assign12090_e6645: f64 = (assign12090_e6641 * assign12090_e6644);
        (assign12090_e6645, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign12090_e6647;
        locals.var_t1_dn0 = assign12090_e6647_d_n0;
        locals.var_t1_dn2 = assign12090_e6647_d_n2;
        locals.var_t1_dn4 = assign12090_e6647_d_n4;
        locals.var_t1_dn5 = assign12090_e6647_d_n5;
        locals.var_t1_dn6 = assign12090_e6647_d_n6;
        locals.var_t1_dn7 = assign12090_e6647_d_n7;
        locals.var_t1_dn8 = assign12090_e6647_d_n8;
        locals.var_t1_dn9 = assign12090_e6647_d_n9;
        locals.var_t1_dn10 = assign12090_e6647_d_n10;
        locals.var_t1_dn11 = assign12090_e6647_d_n11;
        locals.var_t1_dn14 = assign12090_e6647_d_n14;

        let assign12100_e6650: f64 = if locals.var_t1 > 60.0 { 1.0 } else { 0.0 };
        locals.var_guard280 = assign12100_e6650;

        let (assign12110_e6658, assign12110_e6658_d_n0, assign12110_e6658_d_n2, assign12110_e6658_d_n4, assign12110_e6658_d_n5, assign12110_e6658_d_n6, assign12110_e6658_d_n7, assign12110_e6658_d_n8, assign12110_e6658_d_n9, assign12110_e6658_d_n10, assign12110_e6658_d_n11, assign12110_e6658_d_n14,) = {
    if (((locals.var_guard275 != 0.0) && (locals.var_guard279 != 0.0)) && (locals.var_guard280 != 0.0)) {
        (60.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign12110_e6658;
        locals.var_t1_dn0 = assign12110_e6658_d_n0;
        locals.var_t1_dn2 = assign12110_e6658_d_n2;
        locals.var_t1_dn4 = assign12110_e6658_d_n4;
        locals.var_t1_dn5 = assign12110_e6658_d_n5;
        locals.var_t1_dn6 = assign12110_e6658_d_n6;
        locals.var_t1_dn7 = assign12110_e6658_d_n7;
        locals.var_t1_dn8 = assign12110_e6658_d_n8;
        locals.var_t1_dn9 = assign12110_e6658_d_n9;
        locals.var_t1_dn10 = assign12110_e6658_d_n10;
        locals.var_t1_dn11 = assign12110_e6658_d_n11;
        locals.var_t1_dn14 = assign12110_e6658_d_n14;

        let (assign12120_e6665, assign12120_e6665_d_n0, assign12120_e6665_d_n2, assign12120_e6665_d_n4, assign12120_e6665_d_n5, assign12120_e6665_d_n6, assign12120_e6665_d_n7, assign12120_e6665_d_n8, assign12120_e6665_d_n9, assign12120_e6665_d_n10, assign12120_e6665_d_n11, assign12120_e6665_d_n14,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard279 != 0.0)) {
        let assign12120_e6663: f64 = (locals.var_t1).exp();
        (assign12120_e6663, (assign12120_e6663 * locals.var_t1_dn0), (assign12120_e6663 * locals.var_t1_dn2), (assign12120_e6663 * locals.var_t1_dn4), (assign12120_e6663 * locals.var_t1_dn5), (assign12120_e6663 * locals.var_t1_dn6), (assign12120_e6663 * locals.var_t1_dn7), (assign12120_e6663 * locals.var_t1_dn8), (assign12120_e6663 * locals.var_t1_dn9), (assign12120_e6663 * locals.var_t1_dn10), (assign12120_e6663 * locals.var_t1_dn11), (assign12120_e6663 * locals.var_t1_dn14),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign12120_e6665;
        locals.var_t1_dn0 = assign12120_e6665_d_n0;
        locals.var_t1_dn2 = assign12120_e6665_d_n2;
        locals.var_t1_dn4 = assign12120_e6665_d_n4;
        locals.var_t1_dn5 = assign12120_e6665_d_n5;
        locals.var_t1_dn6 = assign12120_e6665_d_n6;
        locals.var_t1_dn7 = assign12120_e6665_d_n7;
        locals.var_t1_dn8 = assign12120_e6665_d_n8;
        locals.var_t1_dn9 = assign12120_e6665_d_n9;
        locals.var_t1_dn10 = assign12120_e6665_d_n10;
        locals.var_t1_dn11 = assign12120_e6665_d_n11;
        locals.var_t1_dn14 = assign12120_e6665_d_n14;

        let (assign12130_e6675, assign12130_e6675_d_n0, assign12130_e6675_d_n2, assign12130_e6675_d_n4, assign12130_e6675_d_n5, assign12130_e6675_d_n6, assign12130_e6675_d_n7, assign12130_e6675_d_n8, assign12130_e6675_d_n9, assign12130_e6675_d_n10, assign12130_e6675_d_n11, assign12130_e6675_d_n14,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard279 != 0.0)) {
        let assign12130_e6671: f64 = (locals.var_uc_rd23 * locals.var_t2);
        let assign12130_e6673: f64 = (assign12130_e6671 * locals.var_t1);
        (assign12130_e6673, (((locals.var_uc_rd23 * locals.var_t2_dn0) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn0)), (((locals.var_uc_rd23 * locals.var_t2_dn2) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn2)), (((locals.var_uc_rd23 * locals.var_t2_dn4) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn4)), (((locals.var_uc_rd23 * locals.var_t2_dn5) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn5)), (((locals.var_uc_rd23 * locals.var_t2_dn6) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn6)), (((locals.var_uc_rd23 * locals.var_t2_dn7) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn7)), (((locals.var_uc_rd23 * locals.var_t2_dn8) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn8)), (((locals.var_uc_rd23 * locals.var_t2_dn9) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn9)), (((locals.var_uc_rd23 * locals.var_t2_dn10) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn10)), (((locals.var_uc_rd23 * locals.var_t2_dn11) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn11)), (((locals.var_uc_rd23 * locals.var_t2_dn14) * locals.var_t1) + (assign12130_e6671 * locals.var_t1_dn14)),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign12130_e6675;
        locals.var_t3_dn0 = assign12130_e6675_d_n0;
        locals.var_t3_dn2 = assign12130_e6675_d_n2;
        locals.var_t3_dn4 = assign12130_e6675_d_n4;
        locals.var_t3_dn5 = assign12130_e6675_d_n5;
        locals.var_t3_dn6 = assign12130_e6675_d_n6;
        locals.var_t3_dn7 = assign12130_e6675_d_n7;
        locals.var_t3_dn8 = assign12130_e6675_d_n8;
        locals.var_t3_dn9 = assign12130_e6675_d_n9;
        locals.var_t3_dn10 = assign12130_e6675_d_n10;
        locals.var_t3_dn11 = assign12130_e6675_d_n11;
        locals.var_t3_dn14 = assign12130_e6675_d_n14;

        let (assign12140_e6698, assign12140_e6698_d_n0, assign12140_e6698_d_n2, assign12140_e6698_d_n4, assign12140_e6698_d_n5, assign12140_e6698_d_n6, assign12140_e6698_d_n7, assign12140_e6698_d_n8, assign12140_e6698_d_n9, assign12140_e6698_d_n10, assign12140_e6698_d_n11, assign12140_e6698_d_n14,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard279 != 0.0)) {
        let assign12140_e6683: f64 = (locals.var_t3 * locals.var_t3);
        let assign12140_e6686: f64 = (4.0 * 1e-6);
        let assign12140_e6688: f64 = (assign12140_e6686 / 100.0);
        let assign12140_e6690: f64 = (assign12140_e6688 * 1e-6);
        let assign12140_e6692: f64 = (assign12140_e6690 / 100.0);
        let assign12140_e6693: f64 = (assign12140_e6683 + assign12140_e6692);
        let assign12140_e6694: f64 = (assign12140_e6693).sqrt();
        let assign12140_e6695: f64 = (locals.var_t3 + assign12140_e6694);
        let assign12140_e6696: f64 = (0.5 * assign12140_e6695);
        (assign12140_e6696, (0.5 * (locals.var_t3_dn0 + (((locals.var_t3_dn0 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn0)) / (2.0 * assign12140_e6694)))), (0.5 * (locals.var_t3_dn2 + (((locals.var_t3_dn2 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn2)) / (2.0 * assign12140_e6694)))), (0.5 * (locals.var_t3_dn4 + (((locals.var_t3_dn4 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn4)) / (2.0 * assign12140_e6694)))), (0.5 * (locals.var_t3_dn5 + (((locals.var_t3_dn5 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn5)) / (2.0 * assign12140_e6694)))), (0.5 * (locals.var_t3_dn6 + (((locals.var_t3_dn6 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn6)) / (2.0 * assign12140_e6694)))), (0.5 * (locals.var_t3_dn7 + (((locals.var_t3_dn7 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn7)) / (2.0 * assign12140_e6694)))), (0.5 * (locals.var_t3_dn8 + (((locals.var_t3_dn8 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn8)) / (2.0 * assign12140_e6694)))), (0.5 * (locals.var_t3_dn9 + (((locals.var_t3_dn9 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn9)) / (2.0 * assign12140_e6694)))), (0.5 * (locals.var_t3_dn10 + (((locals.var_t3_dn10 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn10)) / (2.0 * assign12140_e6694)))), (0.5 * (locals.var_t3_dn11 + (((locals.var_t3_dn11 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn11)) / (2.0 * assign12140_e6694)))), (0.5 * (locals.var_t3_dn14 + (((locals.var_t3_dn14 * locals.var_t3) + (locals.var_t3 * locals.var_t3_dn14)) / (2.0 * assign12140_e6694)))),)
    } else {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn11, locals.var_rd23e_dn14,)
    }
};
        locals.var_rd23e = assign12140_e6698;
        locals.var_rd23e_dn0 = assign12140_e6698_d_n0;
        locals.var_rd23e_dn2 = assign12140_e6698_d_n2;
        locals.var_rd23e_dn4 = assign12140_e6698_d_n4;
        locals.var_rd23e_dn5 = assign12140_e6698_d_n5;
        locals.var_rd23e_dn6 = assign12140_e6698_d_n6;
        locals.var_rd23e_dn7 = assign12140_e6698_d_n7;
        locals.var_rd23e_dn8 = assign12140_e6698_d_n8;
        locals.var_rd23e_dn9 = assign12140_e6698_d_n9;
        locals.var_rd23e_dn10 = assign12140_e6698_d_n10;
        locals.var_rd23e_dn11 = assign12140_e6698_d_n11;
        locals.var_rd23e_dn14 = assign12140_e6698_d_n14;

        let (assign12150_e6705, assign12150_e6705_d_n0, assign12150_e6705_d_n2, assign12150_e6705_d_n4, assign12150_e6705_d_n5, assign12150_e6705_d_n6, assign12150_e6705_d_n7, assign12150_e6705_d_n8, assign12150_e6705_d_n9, assign12150_e6705_d_n10, assign12150_e6705_d_n11, assign12150_e6705_d_n14,) = {
    if ((locals.var_guard275 != 0.0) && (locals.var_guard279 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn11, locals.var_rd23e_dn14,)
    }
};
        locals.var_rd23e = assign12150_e6705;
        locals.var_rd23e_dn0 = assign12150_e6705_d_n0;
        locals.var_rd23e_dn2 = assign12150_e6705_d_n2;
        locals.var_rd23e_dn4 = assign12150_e6705_d_n4;
        locals.var_rd23e_dn5 = assign12150_e6705_d_n5;
        locals.var_rd23e_dn6 = assign12150_e6705_d_n6;
        locals.var_rd23e_dn7 = assign12150_e6705_d_n7;
        locals.var_rd23e_dn8 = assign12150_e6705_d_n8;
        locals.var_rd23e_dn9 = assign12150_e6705_d_n9;
        locals.var_rd23e_dn10 = assign12150_e6705_d_n10;
        locals.var_rd23e_dn11 = assign12150_e6705_d_n11;
        locals.var_rd23e_dn14 = assign12150_e6705_d_n14;

        let (assign12160_e6709,) = {
    if (locals.var_guard275 != 0.0) {
        (0.0,)
    } else {
        (locals.var_xmax,)
    }
};
        locals.var_xmax = assign12160_e6709;

        let (assign12170_e6713,) = {
    if (locals.var_guard275 != 0.0) {
        (0.0,)
    } else {
        (locals.var_xmax_s,)
    }
};
        locals.var_xmax_s = assign12170_e6713;

        let (assign12180_e6717,) = {
    if (locals.var_guard275 != 0.0) {
        (0.0,)
    } else {
        (locals.var_rdrvmaxwe,)
    }
};
        locals.var_rdrvmaxwe = assign12180_e6717;

        let (assign12190_e6721,) = {
    if (locals.var_guard275 != 0.0) {
        (0.0,)
    } else {
        (locals.var_rdrvmaxle,)
    }
};
        locals.var_rdrvmaxle = assign12190_e6721;

        let (assign12200_e6725,) = {
    if (locals.var_guard275 != 0.0) {
        (0.0,)
    } else {
        (locals.var_rdrmuele,)
    }
};
        locals.var_rdrmuele = assign12200_e6725;

        let (assign12210_e6729, assign12210_e6729_d_n0, assign12210_e6729_d_n2, assign12210_e6729_d_n4, assign12210_e6729_d_n5, assign12210_e6729_d_n6, assign12210_e6729_d_n7, assign12210_e6729_d_n8, assign12210_e6729_d_n9, assign12210_e6729_d_n10, assign12210_e6729_d_n11, assign12210_e6729_d_n14,) = {
    if (locals.var_guard275 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrmuevbs, locals.var_rdrmuevbs_dn0, locals.var_rdrmuevbs_dn2, locals.var_rdrmuevbs_dn4, locals.var_rdrmuevbs_dn5, locals.var_rdrmuevbs_dn6, locals.var_rdrmuevbs_dn7, locals.var_rdrmuevbs_dn8, locals.var_rdrmuevbs_dn9, locals.var_rdrmuevbs_dn10, locals.var_rdrmuevbs_dn11, locals.var_rdrmuevbs_dn14,)
    }
};
        locals.var_rdrmuevbs = assign12210_e6729;
        locals.var_rdrmuevbs_dn0 = assign12210_e6729_d_n0;
        locals.var_rdrmuevbs_dn2 = assign12210_e6729_d_n2;
        locals.var_rdrmuevbs_dn4 = assign12210_e6729_d_n4;
        locals.var_rdrmuevbs_dn5 = assign12210_e6729_d_n5;
        locals.var_rdrmuevbs_dn6 = assign12210_e6729_d_n6;
        locals.var_rdrmuevbs_dn7 = assign12210_e6729_d_n7;
        locals.var_rdrmuevbs_dn8 = assign12210_e6729_d_n8;
        locals.var_rdrmuevbs_dn9 = assign12210_e6729_d_n9;
        locals.var_rdrmuevbs_dn10 = assign12210_e6729_d_n10;
        locals.var_rdrmuevbs_dn11 = assign12210_e6729_d_n11;
        locals.var_rdrmuevbs_dn14 = assign12210_e6729_d_n14;

        let (assign12220_e6741,) = {
    if (locals.var_guard275 == 0.0) {
        let assign12220_e6734: f64 = (p.p419 * p.p419);
        let assign12220_e6737: f64 = (locals.var_uc_xldld * locals.var_uc_xldld);
        let assign12220_e6738: f64 = (assign12220_e6734 + assign12220_e6737);
        let assign12220_e6739: f64 = (assign12220_e6738).sqrt();
        (assign12220_e6739,)
    } else {
        (locals.var_xmax,)
    }
};
        locals.var_xmax = assign12220_e6741;

        let (assign12230_e6753,) = {
    if (locals.var_guard275 == 0.0) {
        let assign12230_e6746: f64 = (p.p419 * p.p419);
        let assign12230_e6749: f64 = (p.p97 * p.p97);
        let assign12230_e6750: f64 = (assign12230_e6746 + assign12230_e6749);
        let assign12230_e6751: f64 = (assign12230_e6750).sqrt();
        (assign12230_e6751,)
    } else {
        (locals.var_xmax_s,)
    }
};
        locals.var_xmax_s = assign12230_e6753;

        let (assign12240_e6764,) = {
    if (locals.var_guard275 == 0.0) {
        let assign12240_e6760: f64 = (locals.var_wg).powf(p.p425);
        let assign12240_e6761: f64 = (p.p424 / assign12240_e6760);
        let assign12240_e6762: f64 = (1.0 + assign12240_e6761);
        (assign12240_e6762,)
    } else {
        (locals.var_rdrvmaxwe,)
    }
};
        locals.var_rdrvmaxwe = assign12240_e6764;

        let (assign12250_e6775,) = {
    if (locals.var_guard275 == 0.0) {
        let assign12250_e6771: f64 = (locals.var_lg).powf(p.p427);
        let assign12250_e6772: f64 = (p.p426 / assign12250_e6771);
        let assign12250_e6773: f64 = (1.0 + assign12250_e6772);
        (assign12250_e6773,)
    } else {
        (locals.var_rdrvmaxle,)
    }
};
        locals.var_rdrvmaxle = assign12250_e6775;

        let (assign12260_e6786,) = {
    if (locals.var_guard275 == 0.0) {
        let assign12260_e6782: f64 = (locals.var_lg).powf(p.p429);
        let assign12260_e6783: f64 = (p.p428 / assign12260_e6782);
        let assign12260_e6784: f64 = (1.0 + assign12260_e6783);
        (assign12260_e6784,)
    } else {
        (locals.var_rdrmuele,)
    }
};
        locals.var_rdrmuele = assign12260_e6786;

        let (assign12270_e6791, assign12270_e6791_d_n0, assign12270_e6791_d_n2, assign12270_e6791_d_n4, assign12270_e6791_d_n5, assign12270_e6791_d_n6, assign12270_e6791_d_n7, assign12270_e6791_d_n8, assign12270_e6791_d_n9, assign12270_e6791_d_n10, assign12270_e6791_d_n11, assign12270_e6791_d_n14,) = {
    if (locals.var_guard275 == 0.0) {
        (1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdrmuevbs, locals.var_rdrmuevbs_dn0, locals.var_rdrmuevbs_dn2, locals.var_rdrmuevbs_dn4, locals.var_rdrmuevbs_dn5, locals.var_rdrmuevbs_dn6, locals.var_rdrmuevbs_dn7, locals.var_rdrmuevbs_dn8, locals.var_rdrmuevbs_dn9, locals.var_rdrmuevbs_dn10, locals.var_rdrmuevbs_dn11, locals.var_rdrmuevbs_dn14,)
    }
};
        locals.var_rdrmuevbs = assign12270_e6791;
        locals.var_rdrmuevbs_dn0 = assign12270_e6791_d_n0;
        locals.var_rdrmuevbs_dn2 = assign12270_e6791_d_n2;
        locals.var_rdrmuevbs_dn4 = assign12270_e6791_d_n4;
        locals.var_rdrmuevbs_dn5 = assign12270_e6791_d_n5;
        locals.var_rdrmuevbs_dn6 = assign12270_e6791_d_n6;
        locals.var_rdrmuevbs_dn7 = assign12270_e6791_d_n7;
        locals.var_rdrmuevbs_dn8 = assign12270_e6791_d_n8;
        locals.var_rdrmuevbs_dn9 = assign12270_e6791_d_n9;
        locals.var_rdrmuevbs_dn10 = assign12270_e6791_d_n10;
        locals.var_rdrmuevbs_dn11 = assign12270_e6791_d_n11;
        locals.var_rdrmuevbs_dn14 = assign12270_e6791_d_n14;

        let (assign12280_e6796,) = {
    if (locals.var_guard275 == 0.0) {
        (0.0,)
    } else {
        (locals.var_rdtemp0,)
    }
};
        locals.var_rdtemp0 = assign12280_e6796;

    }

    pub(super) fn stamp_transient_block_20(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12290_e6801, assign12290_e6801_d_n0, assign12290_e6801_d_n2, assign12290_e6801_d_n4, assign12290_e6801_d_n5, assign12290_e6801_d_n6, assign12290_e6801_d_n7, assign12290_e6801_d_n8, assign12290_e6801_d_n9, assign12290_e6801_d_n10, assign12290_e6801_d_n11, assign12290_e6801_d_n14,) = {
    if (locals.var_guard275 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvdtemp0, locals.var_rdvdtemp0_dn0, locals.var_rdvdtemp0_dn2, locals.var_rdvdtemp0_dn4, locals.var_rdvdtemp0_dn5, locals.var_rdvdtemp0_dn6, locals.var_rdvdtemp0_dn7, locals.var_rdvdtemp0_dn8, locals.var_rdvdtemp0_dn9, locals.var_rdvdtemp0_dn10, locals.var_rdvdtemp0_dn11, locals.var_rdvdtemp0_dn14,)
    }
};
        locals.var_rdvdtemp0 = assign12290_e6801;
        locals.var_rdvdtemp0_dn0 = assign12290_e6801_d_n0;
        locals.var_rdvdtemp0_dn2 = assign12290_e6801_d_n2;
        locals.var_rdvdtemp0_dn4 = assign12290_e6801_d_n4;
        locals.var_rdvdtemp0_dn5 = assign12290_e6801_d_n5;
        locals.var_rdvdtemp0_dn6 = assign12290_e6801_d_n6;
        locals.var_rdvdtemp0_dn7 = assign12290_e6801_d_n7;
        locals.var_rdvdtemp0_dn8 = assign12290_e6801_d_n8;
        locals.var_rdvdtemp0_dn9 = assign12290_e6801_d_n9;
        locals.var_rdvdtemp0_dn10 = assign12290_e6801_d_n10;
        locals.var_rdvdtemp0_dn11 = assign12290_e6801_d_n11;
        locals.var_rdvdtemp0_dn14 = assign12290_e6801_d_n14;

        let (assign12300_e6806, assign12300_e6806_d_n0, assign12300_e6806_d_n2, assign12300_e6806_d_n4, assign12300_e6806_d_n5, assign12300_e6806_d_n6, assign12300_e6806_d_n7, assign12300_e6806_d_n8, assign12300_e6806_d_n9, assign12300_e6806_d_n10, assign12300_e6806_d_n11, assign12300_e6806_d_n14,) = {
    if (locals.var_guard275 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rd23e, locals.var_rd23e_dn0, locals.var_rd23e_dn2, locals.var_rd23e_dn4, locals.var_rd23e_dn5, locals.var_rd23e_dn6, locals.var_rd23e_dn7, locals.var_rd23e_dn8, locals.var_rd23e_dn9, locals.var_rd23e_dn10, locals.var_rd23e_dn11, locals.var_rd23e_dn14,)
    }
};
        locals.var_rd23e = assign12300_e6806;
        locals.var_rd23e_dn0 = assign12300_e6806_d_n0;
        locals.var_rd23e_dn2 = assign12300_e6806_d_n2;
        locals.var_rd23e_dn4 = assign12300_e6806_d_n4;
        locals.var_rd23e_dn5 = assign12300_e6806_d_n5;
        locals.var_rd23e_dn6 = assign12300_e6806_d_n6;
        locals.var_rd23e_dn7 = assign12300_e6806_d_n7;
        locals.var_rd23e_dn8 = assign12300_e6806_d_n8;
        locals.var_rd23e_dn9 = assign12300_e6806_d_n9;
        locals.var_rd23e_dn10 = assign12300_e6806_d_n10;
        locals.var_rd23e_dn11 = assign12300_e6806_d_n11;
        locals.var_rd23e_dn14 = assign12300_e6806_d_n14;

        let assign12310_e6809: f64 = if locals.var_uc_nover > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard281 = assign12310_e6809;

        let (assign12320_e6819,) = {
    if (locals.var_guard281 != 0.0) {
        let assign12320_e6813: f64 = (2.0 * 1.034943e-10);
        let assign12320_e6816: f64 = (1.6021918e-19 * locals.var_uc_nover);
        let assign12320_e6817: f64 = (assign12320_e6813 / assign12320_e6816);
        (assign12320_e6817,)
    } else {
        (locals.var_kdep,)
    }
};
        locals.var_kdep = assign12320_e6819;

        let (assign12330_e6835, assign12330_e6835_d_n0, assign12330_e6835_d_n2, assign12330_e6835_d_n4, assign12330_e6835_d_n5, assign12330_e6835_d_n6, assign12330_e6835_d_n7, assign12330_e6835_d_n8, assign12330_e6835_d_n9, assign12330_e6835_d_n10, assign12330_e6835_d_n11, assign12330_e6835_d_n14,) = {
    if (locals.var_guard281 != 0.0) {
        let assign12330_e6823: f64 = (2.0 * 1.034943e-10);
        let assign12330_e6825: f64 = (assign12330_e6823 / 1.6021918e-19);
        let assign12330_e6827: f64 = (assign12330_e6825 * locals.var_ef_nsubc);
        let assign12330_e6830: f64 = (locals.var_uc_nover + locals.var_ef_nsubc);
        let assign12330_e6831: f64 = (assign12330_e6827 / assign12330_e6830);
        let assign12330_e6833: f64 = (assign12330_e6831 / locals.var_uc_nover);
        (assign12330_e6833, (((((assign12330_e6825 * locals.var_ef_nsubc_dn0) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn0)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover), (((((assign12330_e6825 * locals.var_ef_nsubc_dn2) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn2)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover), (((((assign12330_e6825 * locals.var_ef_nsubc_dn4) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn4)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover), (((((assign12330_e6825 * locals.var_ef_nsubc_dn5) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn5)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover), (((((assign12330_e6825 * locals.var_ef_nsubc_dn6) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn6)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover), (((((assign12330_e6825 * locals.var_ef_nsubc_dn7) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn7)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover), (((((assign12330_e6825 * locals.var_ef_nsubc_dn8) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn8)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover), (((((assign12330_e6825 * locals.var_ef_nsubc_dn9) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn9)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover), (((((assign12330_e6825 * locals.var_ef_nsubc_dn10) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn10)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover), (((((assign12330_e6825 * locals.var_ef_nsubc_dn11) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn11)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover), (((((assign12330_e6825 * locals.var_ef_nsubc_dn14) * assign12330_e6830) - (assign12330_e6827 * locals.var_ef_nsubc_dn14)) / (assign12330_e6830 * assign12330_e6830)) / locals.var_uc_nover),)
    } else {
        (locals.var_kjunc, locals.var_kjunc_dn0, locals.var_kjunc_dn2, locals.var_kjunc_dn4, locals.var_kjunc_dn5, locals.var_kjunc_dn6, locals.var_kjunc_dn7, locals.var_kjunc_dn8, locals.var_kjunc_dn9, locals.var_kjunc_dn10, locals.var_kjunc_dn11, locals.var_kjunc_dn14,)
    }
};
        locals.var_kjunc = assign12330_e6835;
        locals.var_kjunc_dn0 = assign12330_e6835_d_n0;
        locals.var_kjunc_dn2 = assign12330_e6835_d_n2;
        locals.var_kjunc_dn4 = assign12330_e6835_d_n4;
        locals.var_kjunc_dn5 = assign12330_e6835_d_n5;
        locals.var_kjunc_dn6 = assign12330_e6835_d_n6;
        locals.var_kjunc_dn7 = assign12330_e6835_d_n7;
        locals.var_kjunc_dn8 = assign12330_e6835_d_n8;
        locals.var_kjunc_dn9 = assign12330_e6835_d_n9;
        locals.var_kjunc_dn10 = assign12330_e6835_d_n10;
        locals.var_kjunc_dn11 = assign12330_e6835_d_n11;
        locals.var_kjunc_dn14 = assign12330_e6835_d_n14;

        let (assign12340_e6840,) = {
    if (locals.var_guard281 == 0.0) {
        (0.0,)
    } else {
        (locals.var_kdep,)
    }
};
        locals.var_kdep = assign12340_e6840;

        let (assign12350_e6845, assign12350_e6845_d_n0, assign12350_e6845_d_n2, assign12350_e6845_d_n4, assign12350_e6845_d_n5, assign12350_e6845_d_n6, assign12350_e6845_d_n7, assign12350_e6845_d_n8, assign12350_e6845_d_n9, assign12350_e6845_d_n10, assign12350_e6845_d_n11, assign12350_e6845_d_n14,) = {
    if (locals.var_guard281 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_kjunc, locals.var_kjunc_dn0, locals.var_kjunc_dn2, locals.var_kjunc_dn4, locals.var_kjunc_dn5, locals.var_kjunc_dn6, locals.var_kjunc_dn7, locals.var_kjunc_dn8, locals.var_kjunc_dn9, locals.var_kjunc_dn10, locals.var_kjunc_dn11, locals.var_kjunc_dn14,)
    }
};
        locals.var_kjunc = assign12350_e6845;
        locals.var_kjunc_dn0 = assign12350_e6845_d_n0;
        locals.var_kjunc_dn2 = assign12350_e6845_d_n2;
        locals.var_kjunc_dn4 = assign12350_e6845_d_n4;
        locals.var_kjunc_dn5 = assign12350_e6845_d_n5;
        locals.var_kjunc_dn6 = assign12350_e6845_d_n6;
        locals.var_kjunc_dn7 = assign12350_e6845_d_n7;
        locals.var_kjunc_dn8 = assign12350_e6845_d_n8;
        locals.var_kjunc_dn9 = assign12350_e6845_d_n9;
        locals.var_kjunc_dn10 = assign12350_e6845_d_n10;
        locals.var_kjunc_dn11 = assign12350_e6845_d_n11;
        locals.var_kjunc_dn14 = assign12350_e6845_d_n14;

        let assign12490_e6940: f64 = if p.p44 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard286 = assign12490_e6940;

        let (assign12500_e6948, assign12500_e6948_d_n0, assign12500_e6948_d_n2, assign12500_e6948_d_n4, assign12500_e6948_d_n5, assign12500_e6948_d_n6, assign12500_e6948_d_n7, assign12500_e6948_d_n8, assign12500_e6948_d_n9, assign12500_e6948_d_n10, assign12500_e6948_d_n11, assign12500_e6948_d_n14,) = {
    if (locals.var_guard286 != 0.0) {
        let assign12500_e6944: f64 = (p.p108 * locals.var_lg);
        let assign12500_e6946: f64 = (assign12500_e6944 + p.p109);
        (assign12500_e6946, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign12500_e6948;
        locals.var_t1_dn0 = assign12500_e6948_d_n0;
        locals.var_t1_dn2 = assign12500_e6948_d_n2;
        locals.var_t1_dn4 = assign12500_e6948_d_n4;
        locals.var_t1_dn5 = assign12500_e6948_d_n5;
        locals.var_t1_dn6 = assign12500_e6948_d_n6;
        locals.var_t1_dn7 = assign12500_e6948_d_n7;
        locals.var_t1_dn8 = assign12500_e6948_d_n8;
        locals.var_t1_dn9 = assign12500_e6948_d_n9;
        locals.var_t1_dn10 = assign12500_e6948_d_n10;
        locals.var_t1_dn11 = assign12500_e6948_d_n11;
        locals.var_t1_dn14 = assign12500_e6948_d_n14;

        let assign12510_e6951: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard287 = assign12510_e6951;

        let (assign12520_e6957, assign12520_e6957_d_n0, assign12520_e6957_d_n2, assign12520_e6957_d_n4, assign12520_e6957_d_n5, assign12520_e6957_d_n6, assign12520_e6957_d_n7, assign12520_e6957_d_n8, assign12520_e6957_d_n9, assign12520_e6957_d_n10, assign12520_e6957_d_n11, assign12520_e6957_d_n14,) = {
    if ((locals.var_guard286 != 0.0) && (locals.var_guard287 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign12520_e6957;
        locals.var_t1_dn0 = assign12520_e6957_d_n0;
        locals.var_t1_dn2 = assign12520_e6957_d_n2;
        locals.var_t1_dn4 = assign12520_e6957_d_n4;
        locals.var_t1_dn5 = assign12520_e6957_d_n5;
        locals.var_t1_dn6 = assign12520_e6957_d_n6;
        locals.var_t1_dn7 = assign12520_e6957_d_n7;
        locals.var_t1_dn8 = assign12520_e6957_d_n8;
        locals.var_t1_dn9 = assign12520_e6957_d_n9;
        locals.var_t1_dn10 = assign12520_e6957_d_n10;
        locals.var_t1_dn11 = assign12520_e6957_d_n11;
        locals.var_t1_dn14 = assign12520_e6957_d_n14;

        let (assign12530_e6969, assign12530_e6969_d_n0, assign12530_e6969_d_n2, assign12530_e6969_d_n4, assign12530_e6969_d_n5, assign12530_e6969_d_n6, assign12530_e6969_d_n7, assign12530_e6969_d_n8, assign12530_e6969_d_n9, assign12530_e6969_d_n10, assign12530_e6969_d_n11, assign12530_e6969_d_n14,) = {
    if (locals.var_guard286 != 0.0) {
        let assign12530_e6961: f64 = (locals.var_t1 * p.p107);
        let assign12530_e6964: f64 = (locals.var_t1 + p.p107);
        let assign12530_e6965: f64 = (assign12530_e6961 / assign12530_e6964);
        let assign12530_e6967: f64 = (assign12530_e6965 + 1.0);
        (assign12530_e6967, ((((locals.var_t1_dn0 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn0)) / (assign12530_e6964 * assign12530_e6964)), ((((locals.var_t1_dn2 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn2)) / (assign12530_e6964 * assign12530_e6964)), ((((locals.var_t1_dn4 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn4)) / (assign12530_e6964 * assign12530_e6964)), ((((locals.var_t1_dn5 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn5)) / (assign12530_e6964 * assign12530_e6964)), ((((locals.var_t1_dn6 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn6)) / (assign12530_e6964 * assign12530_e6964)), ((((locals.var_t1_dn7 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn7)) / (assign12530_e6964 * assign12530_e6964)), ((((locals.var_t1_dn8 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn8)) / (assign12530_e6964 * assign12530_e6964)), ((((locals.var_t1_dn9 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn9)) / (assign12530_e6964 * assign12530_e6964)), ((((locals.var_t1_dn10 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn10)) / (assign12530_e6964 * assign12530_e6964)), ((((locals.var_t1_dn11 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn11)) / (assign12530_e6964 * assign12530_e6964)), ((((locals.var_t1_dn14 * p.p107) * assign12530_e6964) - (assign12530_e6961 * locals.var_t1_dn14)) / (assign12530_e6964 * assign12530_e6964)),)
    } else {
        (locals.var_ddlte, locals.var_ddlte_dn0, locals.var_ddlte_dn2, locals.var_ddlte_dn4, locals.var_ddlte_dn5, locals.var_ddlte_dn6, locals.var_ddlte_dn7, locals.var_ddlte_dn8, locals.var_ddlte_dn9, locals.var_ddlte_dn10, locals.var_ddlte_dn11, locals.var_ddlte_dn14,)
    }
};
        locals.var_ddlte = assign12530_e6969;
        locals.var_ddlte_dn0 = assign12530_e6969_d_n0;
        locals.var_ddlte_dn2 = assign12530_e6969_d_n2;
        locals.var_ddlte_dn4 = assign12530_e6969_d_n4;
        locals.var_ddlte_dn5 = assign12530_e6969_d_n5;
        locals.var_ddlte_dn6 = assign12530_e6969_d_n6;
        locals.var_ddlte_dn7 = assign12530_e6969_d_n7;
        locals.var_ddlte_dn8 = assign12530_e6969_d_n8;
        locals.var_ddlte_dn9 = assign12530_e6969_d_n9;
        locals.var_ddlte_dn10 = assign12530_e6969_d_n10;
        locals.var_ddlte_dn11 = assign12530_e6969_d_n11;
        locals.var_ddlte_dn14 = assign12530_e6969_d_n14;

        let (assign12540_e6976, assign12540_e6976_d_n0, assign12540_e6976_d_n2, assign12540_e6976_d_n4, assign12540_e6976_d_n5, assign12540_e6976_d_n6, assign12540_e6976_d_n7, assign12540_e6976_d_n8, assign12540_e6976_d_n9, assign12540_e6976_d_n10, assign12540_e6976_d_n11, assign12540_e6976_d_n14,) = {
    if (locals.var_guard286 == 0.0) {
        let assign12540_e6974: f64 = (p.p108 * locals.var_lg);
        (assign12540_e6974, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign12540_e6976;
        locals.var_t1_dn0 = assign12540_e6976_d_n0;
        locals.var_t1_dn2 = assign12540_e6976_d_n2;
        locals.var_t1_dn4 = assign12540_e6976_d_n4;
        locals.var_t1_dn5 = assign12540_e6976_d_n5;
        locals.var_t1_dn6 = assign12540_e6976_d_n6;
        locals.var_t1_dn7 = assign12540_e6976_d_n7;
        locals.var_t1_dn8 = assign12540_e6976_d_n8;
        locals.var_t1_dn9 = assign12540_e6976_d_n9;
        locals.var_t1_dn10 = assign12540_e6976_d_n10;
        locals.var_t1_dn11 = assign12540_e6976_d_n11;
        locals.var_t1_dn14 = assign12540_e6976_d_n14;

        let assign12550_e6979: f64 = if locals.var_t1 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard288 = assign12550_e6979;

        let (assign12560_e6986, assign12560_e6986_d_n0, assign12560_e6986_d_n2, assign12560_e6986_d_n4, assign12560_e6986_d_n5, assign12560_e6986_d_n6, assign12560_e6986_d_n7, assign12560_e6986_d_n8, assign12560_e6986_d_n9, assign12560_e6986_d_n10, assign12560_e6986_d_n11, assign12560_e6986_d_n14,) = {
    if ((locals.var_guard286 == 0.0) && (locals.var_guard288 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign12560_e6986;
        locals.var_t1_dn0 = assign12560_e6986_d_n0;
        locals.var_t1_dn2 = assign12560_e6986_d_n2;
        locals.var_t1_dn4 = assign12560_e6986_d_n4;
        locals.var_t1_dn5 = assign12560_e6986_d_n5;
        locals.var_t1_dn6 = assign12560_e6986_d_n6;
        locals.var_t1_dn7 = assign12560_e6986_d_n7;
        locals.var_t1_dn8 = assign12560_e6986_d_n8;
        locals.var_t1_dn9 = assign12560_e6986_d_n9;
        locals.var_t1_dn10 = assign12560_e6986_d_n10;
        locals.var_t1_dn11 = assign12560_e6986_d_n11;
        locals.var_t1_dn14 = assign12560_e6986_d_n14;

        let (assign12570_e7001, assign12570_e7001_d_n0, assign12570_e7001_d_n2, assign12570_e7001_d_n4, assign12570_e7001_d_n5, assign12570_e7001_d_n6, assign12570_e7001_d_n7, assign12570_e7001_d_n8, assign12570_e7001_d_n9, assign12570_e7001_d_n10, assign12570_e7001_d_n11, assign12570_e7001_d_n14,) = {
    if (locals.var_guard286 == 0.0) {
        let assign12570_e6991: f64 = (locals.var_t1 * p.p107);
        let assign12570_e6994: f64 = (locals.var_t1 + p.p107);
        let assign12570_e6995: f64 = (assign12570_e6991 / assign12570_e6994);
        let assign12570_e6997: f64 = (assign12570_e6995 + p.p109);
        let assign12570_e6999: f64 = (assign12570_e6997 + 1e-25);
        (assign12570_e6999, ((((locals.var_t1_dn0 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn0)) / (assign12570_e6994 * assign12570_e6994)), ((((locals.var_t1_dn2 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn2)) / (assign12570_e6994 * assign12570_e6994)), ((((locals.var_t1_dn4 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn4)) / (assign12570_e6994 * assign12570_e6994)), ((((locals.var_t1_dn5 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn5)) / (assign12570_e6994 * assign12570_e6994)), ((((locals.var_t1_dn6 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn6)) / (assign12570_e6994 * assign12570_e6994)), ((((locals.var_t1_dn7 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn7)) / (assign12570_e6994 * assign12570_e6994)), ((((locals.var_t1_dn8 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn8)) / (assign12570_e6994 * assign12570_e6994)), ((((locals.var_t1_dn9 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn9)) / (assign12570_e6994 * assign12570_e6994)), ((((locals.var_t1_dn10 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn10)) / (assign12570_e6994 * assign12570_e6994)), ((((locals.var_t1_dn11 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn11)) / (assign12570_e6994 * assign12570_e6994)), ((((locals.var_t1_dn14 * p.p107) * assign12570_e6994) - (assign12570_e6991 * locals.var_t1_dn14)) / (assign12570_e6994 * assign12570_e6994)),)
    } else {
        (locals.var_ddlte, locals.var_ddlte_dn0, locals.var_ddlte_dn2, locals.var_ddlte_dn4, locals.var_ddlte_dn5, locals.var_ddlte_dn6, locals.var_ddlte_dn7, locals.var_ddlte_dn8, locals.var_ddlte_dn9, locals.var_ddlte_dn10, locals.var_ddlte_dn11, locals.var_ddlte_dn14,)
    }
};
        locals.var_ddlte = assign12570_e7001;
        locals.var_ddlte_dn0 = assign12570_e7001_d_n0;
        locals.var_ddlte_dn2 = assign12570_e7001_d_n2;
        locals.var_ddlte_dn4 = assign12570_e7001_d_n4;
        locals.var_ddlte_dn5 = assign12570_e7001_d_n5;
        locals.var_ddlte_dn6 = assign12570_e7001_d_n6;
        locals.var_ddlte_dn7 = assign12570_e7001_d_n7;
        locals.var_ddlte_dn8 = assign12570_e7001_d_n8;
        locals.var_ddlte_dn9 = assign12570_e7001_d_n9;
        locals.var_ddlte_dn10 = assign12570_e7001_d_n10;
        locals.var_ddlte_dn11 = assign12570_e7001_d_n11;
        locals.var_ddlte_dn14 = assign12570_e7001_d_n14;

        let assign12590_e7009: f64 = if locals.var_ddlte < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard290 = assign12590_e7009;

        let (assign12600_e7013, assign12600_e7013_d_n0, assign12600_e7013_d_n2, assign12600_e7013_d_n4, assign12600_e7013_d_n5, assign12600_e7013_d_n6, assign12600_e7013_d_n7, assign12600_e7013_d_n8, assign12600_e7013_d_n9, assign12600_e7013_d_n10, assign12600_e7013_d_n11, assign12600_e7013_d_n14,) = {
    if (locals.var_guard290 != 0.0) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ddlte, locals.var_ddlte_dn0, locals.var_ddlte_dn2, locals.var_ddlte_dn4, locals.var_ddlte_dn5, locals.var_ddlte_dn6, locals.var_ddlte_dn7, locals.var_ddlte_dn8, locals.var_ddlte_dn9, locals.var_ddlte_dn10, locals.var_ddlte_dn11, locals.var_ddlte_dn14,)
    }
};
        locals.var_ddlte = assign12600_e7013;
        locals.var_ddlte_dn0 = assign12600_e7013_d_n0;
        locals.var_ddlte_dn2 = assign12600_e7013_d_n2;
        locals.var_ddlte_dn4 = assign12600_e7013_d_n4;
        locals.var_ddlte_dn5 = assign12600_e7013_d_n5;
        locals.var_ddlte_dn6 = assign12600_e7013_d_n6;
        locals.var_ddlte_dn7 = assign12600_e7013_d_n7;
        locals.var_ddlte_dn8 = assign12600_e7013_d_n8;
        locals.var_ddlte_dn9 = assign12600_e7013_d_n9;
        locals.var_ddlte_dn10 = assign12600_e7013_d_n10;
        locals.var_ddlte_dn11 = assign12600_e7013_d_n11;
        locals.var_ddlte_dn14 = assign12600_e7013_d_n14;

        let (assign12610_e7019, assign12610_e7019_d_n0, assign12610_e7019_d_n2, assign12610_e7019_d_n4, assign12610_e7019_d_n5, assign12610_e7019_d_n6, assign12610_e7019_d_n7, assign12610_e7019_d_n8, assign12610_e7019_d_n9, assign12610_e7019_d_n10, assign12610_e7019_d_n11, assign12610_e7019_d_n14,) = {
    if (p.p23 != 0.0) {
        let assign12610_e7017: f64 = (locals.var_weff).powf(p.p201);
        (assign12610_e7017, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign12610_e7019;
        locals.var_t2_dn0 = assign12610_e7019_d_n0;
        locals.var_t2_dn2 = assign12610_e7019_d_n2;
        locals.var_t2_dn4 = assign12610_e7019_d_n4;
        locals.var_t2_dn5 = assign12610_e7019_d_n5;
        locals.var_t2_dn6 = assign12610_e7019_d_n6;
        locals.var_t2_dn7 = assign12610_e7019_d_n7;
        locals.var_t2_dn8 = assign12610_e7019_d_n8;
        locals.var_t2_dn9 = assign12610_e7019_d_n9;
        locals.var_t2_dn10 = assign12610_e7019_d_n10;
        locals.var_t2_dn11 = assign12610_e7019_d_n11;
        locals.var_t2_dn14 = assign12610_e7019_d_n14;

        let (assign12620_e7037, assign12620_e7037_d_n0, assign12620_e7037_d_n2, assign12620_e7037_d_n4, assign12620_e7037_d_n5, assign12620_e7037_d_n6, assign12620_e7037_d_n7, assign12620_e7037_d_n8, assign12620_e7037_d_n9, assign12620_e7037_d_n10, assign12620_e7037_d_n11, assign12620_e7037_d_n14,) = {
    if (p.p23 != 0.0) {
        let assign12620_e7026: f64 = (locals.var_lgate).powf(p.p199);
        let assign12620_e7027: f64 = (locals.var_mks_svgsl / assign12620_e7026);
        let assign12620_e7028: f64 = (1.0 + assign12620_e7027);
        let assign12620_e7029: f64 = (locals.var_uc_svgs * assign12620_e7028);
        let assign12620_e7033: f64 = (locals.var_t2 + locals.var_mks_svgsw);
        let assign12620_e7034: f64 = (locals.var_t2 / assign12620_e7033);
        let assign12620_e7035: f64 = (assign12620_e7029 * assign12620_e7034);
        (assign12620_e7035, (assign12620_e7029 * (((locals.var_t2_dn0 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn0)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((locals.var_t2_dn2 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn2)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((locals.var_t2_dn4 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn4)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((locals.var_t2_dn5 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn5)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((locals.var_t2_dn6 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn6)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((locals.var_t2_dn7 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn7)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((locals.var_t2_dn8 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn8)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((locals.var_t2_dn9 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn9)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((locals.var_t2_dn10 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn10)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((locals.var_t2_dn11 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn11)) / (assign12620_e7033 * assign12620_e7033))), (assign12620_e7029 * (((locals.var_t2_dn14 * assign12620_e7033) - (locals.var_t2 * locals.var_t2_dn14)) / (assign12620_e7033 * assign12620_e7033))),)
    } else {
        (locals.var_vg2const, locals.var_vg2const_dn0, locals.var_vg2const_dn2, locals.var_vg2const_dn4, locals.var_vg2const_dn5, locals.var_vg2const_dn6, locals.var_vg2const_dn7, locals.var_vg2const_dn8, locals.var_vg2const_dn9, locals.var_vg2const_dn10, locals.var_vg2const_dn11, locals.var_vg2const_dn14,)
    }
};
        locals.var_vg2const = assign12620_e7037;
        locals.var_vg2const_dn0 = assign12620_e7037_d_n0;
        locals.var_vg2const_dn2 = assign12620_e7037_d_n2;
        locals.var_vg2const_dn4 = assign12620_e7037_d_n4;
        locals.var_vg2const_dn5 = assign12620_e7037_d_n5;
        locals.var_vg2const_dn6 = assign12620_e7037_d_n6;
        locals.var_vg2const_dn7 = assign12620_e7037_d_n7;
        locals.var_vg2const_dn8 = assign12620_e7037_d_n8;
        locals.var_vg2const_dn9 = assign12620_e7037_d_n9;
        locals.var_vg2const_dn10 = assign12620_e7037_d_n10;
        locals.var_vg2const_dn11 = assign12620_e7037_d_n11;
        locals.var_vg2const_dn14 = assign12620_e7037_d_n14;

        let (assign12630_e7049,) = {
    if (p.p23 != 0.0) {
        let assign12630_e7044: f64 = (locals.var_lgate).powf(p.p184);
        let assign12630_e7045: f64 = (locals.var_mks_svbsl / assign12630_e7044);
        let assign12630_e7046: f64 = (1.0 + assign12630_e7045);
        let assign12630_e7047: f64 = (locals.var_uc_svbs * assign12630_e7046);
        (assign12630_e7047,)
    } else {
        (locals.var_xvbs,)
    }
};
        locals.var_xvbs = assign12630_e7049;

        let (assign12640_e7061,) = {
    if (p.p23 != 0.0) {
        let assign12640_e7056: f64 = (locals.var_lgate).powf(p.p203);
        let assign12640_e7057: f64 = (locals.var_mks_slgl / assign12640_e7056);
        let assign12640_e7058: f64 = (1.0 + assign12640_e7057);
        let assign12640_e7059: f64 = (locals.var_mks_slg * assign12640_e7058);
        (assign12640_e7059,)
    } else {
        (locals.var_xgate,)
    }
};
        locals.var_xgate = assign12640_e7061;

        let (assign12650_e7073,) = {
    if (p.p23 != 0.0) {
        let assign12650_e7068: f64 = (locals.var_lgate).powf(p.p191);
        let assign12650_e7069: f64 = (locals.var_mks_sub1l / assign12650_e7068);
        let assign12650_e7070: f64 = (1.0 + assign12650_e7069);
        let assign12650_e7071: f64 = (locals.var_uc_sub1 * assign12650_e7070);
        (assign12650_e7071,)
    } else {
        (locals.var_xsub1,)
    }
};
        locals.var_xsub1 = assign12650_e7073;

        let (assign12660_e7083,) = {
    if (p.p23 != 0.0) {
        let assign12660_e7079: f64 = (locals.var_mks_sub2l / locals.var_lgate);
        let assign12660_e7080: f64 = (1.0 + assign12660_e7079);
        let assign12660_e7081: f64 = (locals.var_uc_sub2 * assign12660_e7080);
        (assign12660_e7081,)
    } else {
        (locals.var_xsub2,)
    }
};
        locals.var_xsub2 = assign12660_e7083;

        let (assign12670_e7087,) = {
    if (p.p23 != 0.0) {
        (locals.var_xsub1,)
    } else {
        (locals.var_xsub1_1,)
    }
};
        locals.var_xsub1_1 = assign12670_e7087;

        let (assign12680_e7091,) = {
    if (p.p23 != 0.0) {
        (locals.var_xsub2,)
    } else {
        (locals.var_xsub2_1,)
    }
};
        locals.var_xsub2_1 = assign12680_e7091;

        let (assign12690_e7095, assign12690_e7095_d_n0, assign12690_e7095_d_n2, assign12690_e7095_d_n4, assign12690_e7095_d_n5, assign12690_e7095_d_n6, assign12690_e7095_d_n7, assign12690_e7095_d_n8, assign12690_e7095_d_n9, assign12690_e7095_d_n10, assign12690_e7095_d_n11, assign12690_e7095_d_n14,) = {
    if (p.p23 != 0.0) {
        (locals.var_vg2const, locals.var_vg2const_dn0, locals.var_vg2const_dn2, locals.var_vg2const_dn4, locals.var_vg2const_dn5, locals.var_vg2const_dn6, locals.var_vg2const_dn7, locals.var_vg2const_dn8, locals.var_vg2const_dn9, locals.var_vg2const_dn10, locals.var_vg2const_dn11, locals.var_vg2const_dn14,)
    } else {
        (locals.var_vg2const_1, locals.var_vg2const_1_dn0, locals.var_vg2const_1_dn2, locals.var_vg2const_1_dn4, locals.var_vg2const_1_dn5, locals.var_vg2const_1_dn6, locals.var_vg2const_1_dn7, locals.var_vg2const_1_dn8, locals.var_vg2const_1_dn9, locals.var_vg2const_1_dn10, locals.var_vg2const_1_dn11, locals.var_vg2const_1_dn14,)
    }
};
        locals.var_vg2const_1 = assign12690_e7095;
        locals.var_vg2const_1_dn0 = assign12690_e7095_d_n0;
        locals.var_vg2const_1_dn2 = assign12690_e7095_d_n2;
        locals.var_vg2const_1_dn4 = assign12690_e7095_d_n4;
        locals.var_vg2const_1_dn5 = assign12690_e7095_d_n5;
        locals.var_vg2const_1_dn6 = assign12690_e7095_d_n6;
        locals.var_vg2const_1_dn7 = assign12690_e7095_d_n7;
        locals.var_vg2const_1_dn8 = assign12690_e7095_d_n8;
        locals.var_vg2const_1_dn9 = assign12690_e7095_d_n9;
        locals.var_vg2const_1_dn10 = assign12690_e7095_d_n10;
        locals.var_vg2const_1_dn11 = assign12690_e7095_d_n11;
        locals.var_vg2const_1_dn14 = assign12690_e7095_d_n14;

        let (assign12700_e7099,) = {
    if (p.p23 != 0.0) {
        (locals.var_xvbs,)
    } else {
        (locals.var_xvbs_1,)
    }
};
        locals.var_xvbs_1 = assign12700_e7099;

        let (assign12710_e7103,) = {
    if (p.p23 != 0.0) {
        (locals.var_xgate,)
    } else {
        (locals.var_xgate_1,)
    }
};
        locals.var_xgate_1 = assign12710_e7103;

        let (assign12720_e7117,) = {
    if ((p.p23 != 0.0) && (p.p46 != 0.0)) {
        let assign12720_e7112: f64 = (locals.var_lgate).powf(p.p191);
        let assign12720_e7113: f64 = (locals.var_mks_sub1l / assign12720_e7112);
        let assign12720_e7114: f64 = (1.0 + assign12720_e7113);
        let assign12720_e7115: f64 = (locals.var_uc_sub1snp * assign12720_e7114);
        (assign12720_e7115,)
    } else {
        (locals.var_xsub1_1,)
    }
};
        locals.var_xsub1_1 = assign12720_e7117;

        let (assign12730_e7129,) = {
    if ((p.p23 != 0.0) && (p.p46 != 0.0)) {
        let assign12730_e7125: f64 = (locals.var_mks_sub2l / locals.var_lgate);
        let assign12730_e7126: f64 = (1.0 + assign12730_e7125);
        let assign12730_e7127: f64 = (locals.var_uc_sub2snp * assign12730_e7126);
        (assign12730_e7127,)
    } else {
        (locals.var_xsub2_1,)
    }
};
        locals.var_xsub2_1 = assign12730_e7129;

        let (assign12740_e7141,) = {
    if (p.p23 != 0.0) {
        let assign12740_e7136: f64 = (locals.var_lg).powf(p.p103);
        let assign12740_e7137: f64 = (p.p102 / assign12740_e7136);
        let assign12740_e7138: f64 = (1.0 + assign12740_e7137);
        let assign12740_e7139: f64 = (p.p72 * assign12740_e7138);
        (assign12740_e7139,)
    } else {
        (locals.var_uc_subld1,)
    }
};
        locals.var_uc_subld1 = assign12740_e7141;

        let (assign12750_e7146, assign12750_e7146_d_n0, assign12750_e7146_d_n2, assign12750_e7146_d_n4, assign12750_e7146_d_n5, assign12750_e7146_d_n6, assign12750_e7146_d_n7, assign12750_e7146_d_n8, assign12750_e7146_d_n9, assign12750_e7146_d_n10, assign12750_e7146_d_n11, assign12750_e7146_d_n14,) = {
    if (p.p23 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vg2const, locals.var_vg2const_dn0, locals.var_vg2const_dn2, locals.var_vg2const_dn4, locals.var_vg2const_dn5, locals.var_vg2const_dn6, locals.var_vg2const_dn7, locals.var_vg2const_dn8, locals.var_vg2const_dn9, locals.var_vg2const_dn10, locals.var_vg2const_dn11, locals.var_vg2const_dn14,)
    }
};
        locals.var_vg2const = assign12750_e7146;
        locals.var_vg2const_dn0 = assign12750_e7146_d_n0;
        locals.var_vg2const_dn2 = assign12750_e7146_d_n2;
        locals.var_vg2const_dn4 = assign12750_e7146_d_n4;
        locals.var_vg2const_dn5 = assign12750_e7146_d_n5;
        locals.var_vg2const_dn6 = assign12750_e7146_d_n6;
        locals.var_vg2const_dn7 = assign12750_e7146_d_n7;
        locals.var_vg2const_dn8 = assign12750_e7146_d_n8;
        locals.var_vg2const_dn9 = assign12750_e7146_d_n9;
        locals.var_vg2const_dn10 = assign12750_e7146_d_n10;
        locals.var_vg2const_dn11 = assign12750_e7146_d_n11;
        locals.var_vg2const_dn14 = assign12750_e7146_d_n14;

        let (assign12760_e7151,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xvbs,)
    }
};
        locals.var_xvbs = assign12760_e7151;

        let (assign12770_e7156,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xgate,)
    }
};
        locals.var_xgate = assign12770_e7156;

    }

    pub(super) fn stamp_transient_block_21(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let (assign12780_e7161,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xsub1,)
    }
};
        locals.var_xsub1 = assign12780_e7161;

        let (assign12790_e7166,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xsub2,)
    }
};
        locals.var_xsub2 = assign12790_e7166;

        let (assign12800_e7171,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_uc_subld1,)
    }
};
        locals.var_uc_subld1 = assign12800_e7171;

        let (assign12810_e7176, assign12810_e7176_d_n0, assign12810_e7176_d_n2, assign12810_e7176_d_n4, assign12810_e7176_d_n5, assign12810_e7176_d_n6, assign12810_e7176_d_n7, assign12810_e7176_d_n8, assign12810_e7176_d_n9, assign12810_e7176_d_n10, assign12810_e7176_d_n11, assign12810_e7176_d_n14,) = {
    if (p.p23 == 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vg2const_1, locals.var_vg2const_1_dn0, locals.var_vg2const_1_dn2, locals.var_vg2const_1_dn4, locals.var_vg2const_1_dn5, locals.var_vg2const_1_dn6, locals.var_vg2const_1_dn7, locals.var_vg2const_1_dn8, locals.var_vg2const_1_dn9, locals.var_vg2const_1_dn10, locals.var_vg2const_1_dn11, locals.var_vg2const_1_dn14,)
    }
};
        locals.var_vg2const_1 = assign12810_e7176;
        locals.var_vg2const_1_dn0 = assign12810_e7176_d_n0;
        locals.var_vg2const_1_dn2 = assign12810_e7176_d_n2;
        locals.var_vg2const_1_dn4 = assign12810_e7176_d_n4;
        locals.var_vg2const_1_dn5 = assign12810_e7176_d_n5;
        locals.var_vg2const_1_dn6 = assign12810_e7176_d_n6;
        locals.var_vg2const_1_dn7 = assign12810_e7176_d_n7;
        locals.var_vg2const_1_dn8 = assign12810_e7176_d_n8;
        locals.var_vg2const_1_dn9 = assign12810_e7176_d_n9;
        locals.var_vg2const_1_dn10 = assign12810_e7176_d_n10;
        locals.var_vg2const_1_dn11 = assign12810_e7176_d_n11;
        locals.var_vg2const_1_dn14 = assign12810_e7176_d_n14;

        let (assign12820_e7181,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xvbs_1,)
    }
};
        locals.var_xvbs_1 = assign12820_e7181;

        let (assign12830_e7186,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xgate_1,)
    }
};
        locals.var_xgate_1 = assign12830_e7186;

        let (assign12840_e7191,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xsub1_1,)
    }
};
        locals.var_xsub1_1 = assign12840_e7191;

        let (assign12850_e7196,) = {
    if (p.p23 == 0.0) {
        (0.0,)
    } else {
        (locals.var_xsub2_1,)
    }
};
        locals.var_xsub2_1 = assign12850_e7196;

        let (assign12860_e7210,) = {
    if (locals.var_uc_ibpc1 != 0.0) {
        let assign12860_e7205: f64 = (locals.var_lg).powf(p.p280);
        let assign12860_e7206: f64 = (p.p279 / assign12860_e7205);
        let assign12860_e7207: f64 = (1.0 + assign12860_e7206);
        let assign12860_e7208: f64 = (locals.var_uc_ibpc1 * assign12860_e7207);
        (assign12860_e7208,)
    } else {
        (0.0,)
    }
};
        locals.var_uc_ibpc1 = assign12860_e7210;

        let assign12870_e7214: f64 = (3.141592653589793 / 2.0);
        let assign12870_e7215: f64 = (3.453133e-11 / assign12870_e7214);
        let assign12870_e7217: f64 = (assign12870_e7215 * locals.var_weffcv_nf);
        let assign12870_e7221: f64 = (p.p225 / p.p95);
        let assign12870_e7222: f64 = (1.0 + assign12870_e7221);
        let assign12870_e7223: f64 = (assign12870_e7222).ln();
        let assign12870_e7224: f64 = (assign12870_e7217 * assign12870_e7223);
        locals.var_cfrng = assign12870_e7224;

        let (assign12880_e7238,) = {
    if (p.p134 != 0.0) {
        let assign12880_e7230: f64 = (1000000.0 * locals.var_weffcv_nf);
        let assign12880_e7232: f64 = (assign12880_e7230 * p.p134);
        let assign12880_e7235: f64 = (locals.var_lg).powf(p.p135);
        let assign12880_e7236: f64 = (assign12880_e7232 / assign12880_e7235);
        (assign12880_e7236,)
    } else {
        (0.0,)
    }
};
        locals.var_cqyb0 = assign12880_e7238;

        let assign12890_e7242: f64 = (-p.p286);
        let assign12890_e7243: f64 = (locals.var_lg).powf(assign12890_e7242);
        let assign12890_e7244: f64 = (p.p283 * assign12890_e7243);
        locals.var_ptl0 = assign12890_e7244;

        let assign12900_e7248: f64 = (-p.p291);
        let assign12900_e7249: f64 = (locals.var_lg).powf(assign12900_e7248);
        let assign12900_e7250: f64 = (p.p290 * assign12900_e7249);
        locals.var_pt40 = assign12900_e7250;

        let assign12910_e7254: f64 = (locals.var_lg + locals.var_uc_gdld);
        let assign12910_e7256: f64 = (-p.p288);
        let assign12910_e7257: f64 = (assign12910_e7254).powf(assign12910_e7256);
        let assign12910_e7258: f64 = (p.p287 * assign12910_e7257);
        locals.var_gdl0 = assign12910_e7258;

        let assign12920_e7262: f64 = (locals.var_mfactor * locals.var_weff_nf);
        let assign12920_e7263: f64 = (locals.var_uc_rth0 / assign12920_e7262);
        let assign12920_e7268: f64 = (locals.var_lg).powf(p.p318);
        let assign12920_e7269: f64 = (p.p317 / assign12920_e7268);
        let assign12920_e7270: f64 = (1.0 + assign12920_e7269);
        let assign12920_e7271: f64 = (assign12920_e7263 * assign12920_e7270);
        let assign12920_e7276: f64 = (locals.var_wg).powf(p.p316);
        let assign12920_e7277: f64 = (p.p315 / assign12920_e7276);
        let assign12920_e7278: f64 = (1.0 + assign12920_e7277);
        let assign12920_e7279: f64 = (assign12920_e7271 * assign12920_e7278);
        locals.var_rth = assign12920_e7279;
        locals.var_rth_dn0 = 0.0;
        locals.var_rth_dn2 = 0.0;
        locals.var_rth_dn4 = 0.0;
        locals.var_rth_dn5 = 0.0;
        locals.var_rth_dn6 = 0.0;
        locals.var_rth_dn7 = 0.0;
        locals.var_rth_dn8 = 0.0;
        locals.var_rth_dn9 = 0.0;
        locals.var_rth_dn10 = 0.0;
        locals.var_rth_dn11 = 0.0;
        locals.var_rth_dn14 = 0.0;

        let assign12940_e7289: f64 = (p.p7).powf(p.p327);
        let assign12940_e7290: f64 = (1.0 / assign12940_e7289);
        let assign12940_e7291: f64 = (locals.var_rth * assign12940_e7290);
        locals.var_rth = assign12940_e7291;
        locals.var_rth_dn0 = (locals.var_rth_dn0 * assign12940_e7290);
        locals.var_rth_dn2 = (locals.var_rth_dn2 * assign12940_e7290);
        locals.var_rth_dn4 = (locals.var_rth_dn4 * assign12940_e7290);
        locals.var_rth_dn5 = (locals.var_rth_dn5 * assign12940_e7290);
        locals.var_rth_dn6 = (locals.var_rth_dn6 * assign12940_e7290);
        locals.var_rth_dn7 = (locals.var_rth_dn7 * assign12940_e7290);
        locals.var_rth_dn8 = (locals.var_rth_dn8 * assign12940_e7290);
        locals.var_rth_dn9 = (locals.var_rth_dn9 * assign12940_e7290);
        locals.var_rth_dn10 = (locals.var_rth_dn10 * assign12940_e7290);
        locals.var_rth_dn11 = (locals.var_rth_dn11 * assign12940_e7290);
        locals.var_rth_dn14 = (locals.var_rth_dn14 * assign12940_e7290);

        let assign12950_e7295: f64 = (p.p7).powf(p.p327);
        let assign12950_e7296: f64 = (1.0 / assign12950_e7295);
        let assign12950_e7299: f64 = (locals.var_mfactor * locals.var_weff_nf);
        let assign12950_e7300: f64 = (assign12950_e7296 / assign12950_e7299);
        let assign12950_e7305: f64 = (locals.var_lg).powf(p.p318);
        let assign12950_e7306: f64 = (p.p317 / assign12950_e7305);
        let assign12950_e7307: f64 = (1.0 + assign12950_e7306);
        let assign12950_e7308: f64 = (assign12950_e7300 * assign12950_e7307);
        let assign12950_e7313: f64 = (locals.var_wg).powf(p.p316);
        let assign12950_e7314: f64 = (p.p315 / assign12950_e7313);
        let assign12950_e7315: f64 = (1.0 + assign12950_e7314);
        let assign12950_e7316: f64 = (assign12950_e7308 * assign12950_e7315);
        locals.var_rthtemp0 = assign12950_e7316;

        let assign12960_e7323: f64 = if ((p.p53 == 0.0) || (locals.var_uc_rth0 == 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard291 = assign12960_e7323;

        let (assign12970_e7327, assign12970_e7327_d_n0, assign12970_e7327_d_n2, assign12970_e7327_d_n4, assign12970_e7327_d_n5, assign12970_e7327_d_n6, assign12970_e7327_d_n7, assign12970_e7327_d_n8, assign12970_e7327_d_n9, assign12970_e7327_d_n10, assign12970_e7327_d_n11, assign12970_e7327_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn14,)
    }
};
        locals.var_cnst0over = assign12970_e7327;
        locals.var_cnst0over_dn0 = assign12970_e7327_d_n0;
        locals.var_cnst0over_dn2 = assign12970_e7327_d_n2;
        locals.var_cnst0over_dn4 = assign12970_e7327_d_n4;
        locals.var_cnst0over_dn5 = assign12970_e7327_d_n5;
        locals.var_cnst0over_dn6 = assign12970_e7327_d_n6;
        locals.var_cnst0over_dn7 = assign12970_e7327_d_n7;
        locals.var_cnst0over_dn8 = assign12970_e7327_d_n8;
        locals.var_cnst0over_dn9 = assign12970_e7327_d_n9;
        locals.var_cnst0over_dn10 = assign12970_e7327_d_n10;
        locals.var_cnst0over_dn11 = assign12970_e7327_d_n11;
        locals.var_cnst0over_dn14 = assign12970_e7327_d_n14;

        let (assign12980_e7331, assign12980_e7331_d_n0, assign12980_e7331_d_n2, assign12980_e7331_d_n4, assign12980_e7331_d_n5, assign12980_e7331_d_n6, assign12980_e7331_d_n7, assign12980_e7331_d_n8, assign12980_e7331_d_n9, assign12980_e7331_d_n10, assign12980_e7331_d_n11, assign12980_e7331_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn11, locals.var_cnst0overs_dn14,)
    }
};
        locals.var_cnst0overs = assign12980_e7331;
        locals.var_cnst0overs_dn0 = assign12980_e7331_d_n0;
        locals.var_cnst0overs_dn2 = assign12980_e7331_d_n2;
        locals.var_cnst0overs_dn4 = assign12980_e7331_d_n4;
        locals.var_cnst0overs_dn5 = assign12980_e7331_d_n5;
        locals.var_cnst0overs_dn6 = assign12980_e7331_d_n6;
        locals.var_cnst0overs_dn7 = assign12980_e7331_d_n7;
        locals.var_cnst0overs_dn8 = assign12980_e7331_d_n8;
        locals.var_cnst0overs_dn9 = assign12980_e7331_d_n9;
        locals.var_cnst0overs_dn10 = assign12980_e7331_d_n10;
        locals.var_cnst0overs_dn11 = assign12980_e7331_d_n11;
        locals.var_cnst0overs_dn14 = assign12980_e7331_d_n14;

        let (assign12990_e7337, assign12990_e7337_d_n0, assign12990_e7337_d_n2, assign12990_e7337_d_n4, assign12990_e7337_d_n5, assign12990_e7337_d_n6, assign12990_e7337_d_n7, assign12990_e7337_d_n8, assign12990_e7337_d_n9, assign12990_e7337_d_n10, assign12990_e7337_d_n11, assign12990_e7337_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign12990_e7333: f64 = ctx_temp;
        let assign12990_e7335: f64 = (assign12990_e7333 + p.p11);
        (assign12990_e7335, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign12990_e7337;
        locals.var_ttemp_dn0 = assign12990_e7337_d_n0;
        locals.var_ttemp_dn2 = assign12990_e7337_d_n2;
        locals.var_ttemp_dn4 = assign12990_e7337_d_n4;
        locals.var_ttemp_dn5 = assign12990_e7337_d_n5;
        locals.var_ttemp_dn6 = assign12990_e7337_d_n6;
        locals.var_ttemp_dn7 = assign12990_e7337_d_n7;
        locals.var_ttemp_dn8 = assign12990_e7337_d_n8;
        locals.var_ttemp_dn9 = assign12990_e7337_d_n9;
        locals.var_ttemp_dn10 = assign12990_e7337_d_n10;
        locals.var_ttemp_dn11 = assign12990_e7337_d_n11;
        locals.var_ttemp_dn14 = assign12990_e7337_d_n14;

        let (assign13000_e7341, assign13000_e7341_d_n0, assign13000_e7341_d_n2, assign13000_e7341_d_n4, assign13000_e7341_d_n5, assign13000_e7341_d_n6, assign13000_e7341_d_n7, assign13000_e7341_d_n8, assign13000_e7341_d_n9, assign13000_e7341_d_n10, assign13000_e7341_d_n11, assign13000_e7341_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    } else {
        (locals.var_ttemp0, locals.var_ttemp0_dn0, locals.var_ttemp0_dn2, locals.var_ttemp0_dn4, locals.var_ttemp0_dn5, locals.var_ttemp0_dn6, locals.var_ttemp0_dn7, locals.var_ttemp0_dn8, locals.var_ttemp0_dn9, locals.var_ttemp0_dn10, locals.var_ttemp0_dn11, locals.var_ttemp0_dn14,)
    }
};
        locals.var_ttemp0 = assign13000_e7341;
        locals.var_ttemp0_dn0 = assign13000_e7341_d_n0;
        locals.var_ttemp0_dn2 = assign13000_e7341_d_n2;
        locals.var_ttemp0_dn4 = assign13000_e7341_d_n4;
        locals.var_ttemp0_dn5 = assign13000_e7341_d_n5;
        locals.var_ttemp0_dn6 = assign13000_e7341_d_n6;
        locals.var_ttemp0_dn7 = assign13000_e7341_d_n7;
        locals.var_ttemp0_dn8 = assign13000_e7341_d_n8;
        locals.var_ttemp0_dn9 = assign13000_e7341_d_n9;
        locals.var_ttemp0_dn10 = assign13000_e7341_d_n10;
        locals.var_ttemp0_dn11 = assign13000_e7341_d_n11;
        locals.var_ttemp0_dn14 = assign13000_e7341_d_n14;

        let (assign13010_e7347, assign13010_e7347_d_n0, assign13010_e7347_d_n2, assign13010_e7347_d_n4, assign13010_e7347_d_n5, assign13010_e7347_d_n6, assign13010_e7347_d_n7, assign13010_e7347_d_n8, assign13010_e7347_d_n9, assign13010_e7347_d_n10, assign13010_e7347_d_n11, assign13010_e7347_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13010_e7345: f64 = (locals.var_ttemp + locals.var_deltemp);
        (assign13010_e7345, (locals.var_ttemp_dn0 + locals.var_deltemp_dn0), (locals.var_ttemp_dn2 + locals.var_deltemp_dn2), (locals.var_ttemp_dn4 + locals.var_deltemp_dn4), (locals.var_ttemp_dn5 + locals.var_deltemp_dn5), (locals.var_ttemp_dn6 + locals.var_deltemp_dn6), (locals.var_ttemp_dn7 + locals.var_deltemp_dn7), (locals.var_ttemp_dn8 + locals.var_deltemp_dn8), (locals.var_ttemp_dn9 + locals.var_deltemp_dn9), (locals.var_ttemp_dn10 + locals.var_deltemp_dn10), (locals.var_ttemp_dn11 + locals.var_deltemp_dn11), (locals.var_ttemp_dn14 + locals.var_deltemp_dn14),)
    } else {
        (locals.var_ttemp, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    }
};
        locals.var_ttemp = assign13010_e7347;
        locals.var_ttemp_dn0 = assign13010_e7347_d_n0;
        locals.var_ttemp_dn2 = assign13010_e7347_d_n2;
        locals.var_ttemp_dn4 = assign13010_e7347_d_n4;
        locals.var_ttemp_dn5 = assign13010_e7347_d_n5;
        locals.var_ttemp_dn6 = assign13010_e7347_d_n6;
        locals.var_ttemp_dn7 = assign13010_e7347_d_n7;
        locals.var_ttemp_dn8 = assign13010_e7347_d_n8;
        locals.var_ttemp_dn9 = assign13010_e7347_d_n9;
        locals.var_ttemp_dn10 = assign13010_e7347_d_n10;
        locals.var_ttemp_dn11 = assign13010_e7347_d_n11;
        locals.var_ttemp_dn14 = assign13010_e7347_d_n14;

        let (assign13020_e7353, assign13020_e7353_d_n0, assign13020_e7353_d_n2, assign13020_e7353_d_n4, assign13020_e7353_d_n5, assign13020_e7353_d_n6, assign13020_e7353_d_n7, assign13020_e7353_d_n8, assign13020_e7353_d_n9, assign13020_e7353_d_n10, assign13020_e7353_d_n11, assign13020_e7353_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13020_e7351: f64 = (locals.var_ttemp0 - locals.var_ktnom);
        (assign13020_e7351, locals.var_ttemp0_dn0, locals.var_ttemp0_dn2, locals.var_ttemp0_dn4, locals.var_ttemp0_dn5, locals.var_ttemp0_dn6, locals.var_ttemp0_dn7, locals.var_ttemp0_dn8, locals.var_ttemp0_dn9, locals.var_ttemp0_dn10, locals.var_ttemp0_dn11, locals.var_ttemp0_dn14,)
    } else {
        (locals.var_tdiff0, locals.var_tdiff0_dn0, locals.var_tdiff0_dn2, locals.var_tdiff0_dn4, locals.var_tdiff0_dn5, locals.var_tdiff0_dn6, locals.var_tdiff0_dn7, locals.var_tdiff0_dn8, locals.var_tdiff0_dn9, locals.var_tdiff0_dn10, locals.var_tdiff0_dn11, locals.var_tdiff0_dn14,)
    }
};
        locals.var_tdiff0 = assign13020_e7353;
        locals.var_tdiff0_dn0 = assign13020_e7353_d_n0;
        locals.var_tdiff0_dn2 = assign13020_e7353_d_n2;
        locals.var_tdiff0_dn4 = assign13020_e7353_d_n4;
        locals.var_tdiff0_dn5 = assign13020_e7353_d_n5;
        locals.var_tdiff0_dn6 = assign13020_e7353_d_n6;
        locals.var_tdiff0_dn7 = assign13020_e7353_d_n7;
        locals.var_tdiff0_dn8 = assign13020_e7353_d_n8;
        locals.var_tdiff0_dn9 = assign13020_e7353_d_n9;
        locals.var_tdiff0_dn10 = assign13020_e7353_d_n10;
        locals.var_tdiff0_dn11 = assign13020_e7353_d_n11;
        locals.var_tdiff0_dn14 = assign13020_e7353_d_n14;

        let (assign13030_e7363, assign13030_e7363_d_n0, assign13030_e7363_d_n2, assign13030_e7363_d_n4, assign13030_e7363_d_n5, assign13030_e7363_d_n6, assign13030_e7363_d_n7, assign13030_e7363_d_n8, assign13030_e7363_d_n9, assign13030_e7363_d_n10, assign13030_e7363_d_n11, assign13030_e7363_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13030_e7357: f64 = (locals.var_ttemp0 * locals.var_ttemp0);
        let assign13030_e7360: f64 = (locals.var_ktnom * locals.var_ktnom);
        let assign13030_e7361: f64 = (assign13030_e7357 - assign13030_e7360);
        (assign13030_e7361, ((locals.var_ttemp0_dn0 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn0)), ((locals.var_ttemp0_dn2 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn2)), ((locals.var_ttemp0_dn4 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn4)), ((locals.var_ttemp0_dn5 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn5)), ((locals.var_ttemp0_dn6 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn6)), ((locals.var_ttemp0_dn7 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn7)), ((locals.var_ttemp0_dn8 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn8)), ((locals.var_ttemp0_dn9 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn9)), ((locals.var_ttemp0_dn10 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn10)), ((locals.var_ttemp0_dn11 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn11)), ((locals.var_ttemp0_dn14 * locals.var_ttemp0) + (locals.var_ttemp0 * locals.var_ttemp0_dn14)),)
    } else {
        (locals.var_tdiff0_2, locals.var_tdiff0_2_dn0, locals.var_tdiff0_2_dn2, locals.var_tdiff0_2_dn4, locals.var_tdiff0_2_dn5, locals.var_tdiff0_2_dn6, locals.var_tdiff0_2_dn7, locals.var_tdiff0_2_dn8, locals.var_tdiff0_2_dn9, locals.var_tdiff0_2_dn10, locals.var_tdiff0_2_dn11, locals.var_tdiff0_2_dn14,)
    }
};
        locals.var_tdiff0_2 = assign13030_e7363;
        locals.var_tdiff0_2_dn0 = assign13030_e7363_d_n0;
        locals.var_tdiff0_2_dn2 = assign13030_e7363_d_n2;
        locals.var_tdiff0_2_dn4 = assign13030_e7363_d_n4;
        locals.var_tdiff0_2_dn5 = assign13030_e7363_d_n5;
        locals.var_tdiff0_2_dn6 = assign13030_e7363_d_n6;
        locals.var_tdiff0_2_dn7 = assign13030_e7363_d_n7;
        locals.var_tdiff0_2_dn8 = assign13030_e7363_d_n8;
        locals.var_tdiff0_2_dn9 = assign13030_e7363_d_n9;
        locals.var_tdiff0_2_dn10 = assign13030_e7363_d_n10;
        locals.var_tdiff0_2_dn11 = assign13030_e7363_d_n11;
        locals.var_tdiff0_2_dn14 = assign13030_e7363_d_n14;

        let (assign13040_e7369, assign13040_e7369_d_n0, assign13040_e7369_d_n2, assign13040_e7369_d_n4, assign13040_e7369_d_n5, assign13040_e7369_d_n6, assign13040_e7369_d_n7, assign13040_e7369_d_n8, assign13040_e7369_d_n9, assign13040_e7369_d_n10, assign13040_e7369_d_n11, assign13040_e7369_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13040_e7367: f64 = (locals.var_ttemp - locals.var_ktnom);
        (assign13040_e7367, locals.var_ttemp_dn0, locals.var_ttemp_dn2, locals.var_ttemp_dn4, locals.var_ttemp_dn5, locals.var_ttemp_dn6, locals.var_ttemp_dn7, locals.var_ttemp_dn8, locals.var_ttemp_dn9, locals.var_ttemp_dn10, locals.var_ttemp_dn11, locals.var_ttemp_dn14,)
    } else {
        (locals.var_tdiff, locals.var_tdiff_dn0, locals.var_tdiff_dn2, locals.var_tdiff_dn4, locals.var_tdiff_dn5, locals.var_tdiff_dn6, locals.var_tdiff_dn7, locals.var_tdiff_dn8, locals.var_tdiff_dn9, locals.var_tdiff_dn10, locals.var_tdiff_dn11, locals.var_tdiff_dn14,)
    }
};
        locals.var_tdiff = assign13040_e7369;
        locals.var_tdiff_dn0 = assign13040_e7369_d_n0;
        locals.var_tdiff_dn2 = assign13040_e7369_d_n2;
        locals.var_tdiff_dn4 = assign13040_e7369_d_n4;
        locals.var_tdiff_dn5 = assign13040_e7369_d_n5;
        locals.var_tdiff_dn6 = assign13040_e7369_d_n6;
        locals.var_tdiff_dn7 = assign13040_e7369_d_n7;
        locals.var_tdiff_dn8 = assign13040_e7369_d_n8;
        locals.var_tdiff_dn9 = assign13040_e7369_d_n9;
        locals.var_tdiff_dn10 = assign13040_e7369_d_n10;
        locals.var_tdiff_dn11 = assign13040_e7369_d_n11;
        locals.var_tdiff_dn14 = assign13040_e7369_d_n14;

        let (assign13050_e7379, assign13050_e7379_d_n0, assign13050_e7379_d_n2, assign13050_e7379_d_n4, assign13050_e7379_d_n5, assign13050_e7379_d_n6, assign13050_e7379_d_n7, assign13050_e7379_d_n8, assign13050_e7379_d_n9, assign13050_e7379_d_n10, assign13050_e7379_d_n11, assign13050_e7379_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13050_e7373: f64 = (locals.var_ttemp * locals.var_ttemp);
        let assign13050_e7376: f64 = (locals.var_ktnom * locals.var_ktnom);
        let assign13050_e7377: f64 = (assign13050_e7373 - assign13050_e7376);
        (assign13050_e7377, ((locals.var_ttemp_dn0 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn0)), ((locals.var_ttemp_dn2 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn2)), ((locals.var_ttemp_dn4 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn4)), ((locals.var_ttemp_dn5 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn5)), ((locals.var_ttemp_dn6 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn6)), ((locals.var_ttemp_dn7 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn7)), ((locals.var_ttemp_dn8 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn8)), ((locals.var_ttemp_dn9 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn9)), ((locals.var_ttemp_dn10 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn10)), ((locals.var_ttemp_dn11 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn11)), ((locals.var_ttemp_dn14 * locals.var_ttemp) + (locals.var_ttemp * locals.var_ttemp_dn14)),)
    } else {
        (locals.var_tdiff_2, locals.var_tdiff_2_dn0, locals.var_tdiff_2_dn2, locals.var_tdiff_2_dn4, locals.var_tdiff_2_dn5, locals.var_tdiff_2_dn6, locals.var_tdiff_2_dn7, locals.var_tdiff_2_dn8, locals.var_tdiff_2_dn9, locals.var_tdiff_2_dn10, locals.var_tdiff_2_dn11, locals.var_tdiff_2_dn14,)
    }
};
        locals.var_tdiff_2 = assign13050_e7379;
        locals.var_tdiff_2_dn0 = assign13050_e7379_d_n0;
        locals.var_tdiff_2_dn2 = assign13050_e7379_d_n2;
        locals.var_tdiff_2_dn4 = assign13050_e7379_d_n4;
        locals.var_tdiff_2_dn5 = assign13050_e7379_d_n5;
        locals.var_tdiff_2_dn6 = assign13050_e7379_d_n6;
        locals.var_tdiff_2_dn7 = assign13050_e7379_d_n7;
        locals.var_tdiff_2_dn8 = assign13050_e7379_d_n8;
        locals.var_tdiff_2_dn9 = assign13050_e7379_d_n9;
        locals.var_tdiff_2_dn10 = assign13050_e7379_d_n10;
        locals.var_tdiff_2_dn11 = assign13050_e7379_d_n11;
        locals.var_tdiff_2_dn14 = assign13050_e7379_d_n14;

        let (assign13060_e7385, assign13060_e7385_d_n0, assign13060_e7385_d_n2, assign13060_e7385_d_n4, assign13060_e7385_d_n5, assign13060_e7385_d_n6, assign13060_e7385_d_n7, assign13060_e7385_d_n8, assign13060_e7385_d_n9, assign13060_e7385_d_n10, assign13060_e7385_d_n11, assign13060_e7385_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13060_e7383: f64 = (locals.var_ttemp / locals.var_ktnom);
        (assign13060_e7383, (locals.var_ttemp_dn0 / locals.var_ktnom), (locals.var_ttemp_dn2 / locals.var_ktnom), (locals.var_ttemp_dn4 / locals.var_ktnom), (locals.var_ttemp_dn5 / locals.var_ktnom), (locals.var_ttemp_dn6 / locals.var_ktnom), (locals.var_ttemp_dn7 / locals.var_ktnom), (locals.var_ttemp_dn8 / locals.var_ktnom), (locals.var_ttemp_dn9 / locals.var_ktnom), (locals.var_ttemp_dn10 / locals.var_ktnom), (locals.var_ttemp_dn11 / locals.var_ktnom), (locals.var_ttemp_dn14 / locals.var_ktnom),)
    } else {
        (locals.var_tratio, locals.var_tratio_dn0, locals.var_tratio_dn2, locals.var_tratio_dn4, locals.var_tratio_dn5, locals.var_tratio_dn6, locals.var_tratio_dn7, locals.var_tratio_dn8, locals.var_tratio_dn9, locals.var_tratio_dn10, locals.var_tratio_dn11, locals.var_tratio_dn14,)
    }
};
        locals.var_tratio = assign13060_e7385;
        locals.var_tratio_dn0 = assign13060_e7385_d_n0;
        locals.var_tratio_dn2 = assign13060_e7385_d_n2;
        locals.var_tratio_dn4 = assign13060_e7385_d_n4;
        locals.var_tratio_dn5 = assign13060_e7385_d_n5;
        locals.var_tratio_dn6 = assign13060_e7385_d_n6;
        locals.var_tratio_dn7 = assign13060_e7385_d_n7;
        locals.var_tratio_dn8 = assign13060_e7385_d_n8;
        locals.var_tratio_dn9 = assign13060_e7385_d_n9;
        locals.var_tratio_dn10 = assign13060_e7385_d_n10;
        locals.var_tratio_dn11 = assign13060_e7385_d_n11;
        locals.var_tratio_dn14 = assign13060_e7385_d_n14;

        let (assign13070_e7390, assign13070_e7390_d_n0, assign13070_e7390_d_n2, assign13070_e7390_d_n4, assign13070_e7390_d_n5, assign13070_e7390_d_n6, assign13070_e7390_d_n7, assign13070_e7390_d_n8, assign13070_e7390_d_n9, assign13070_e7390_d_n10, assign13070_e7390_d_n11, assign13070_e7390_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13070_e7388: f64 = (locals.var_tratio).ln();
        (assign13070_e7388, (locals.var_tratio_dn0 / locals.var_tratio), (locals.var_tratio_dn2 / locals.var_tratio), (locals.var_tratio_dn4 / locals.var_tratio), (locals.var_tratio_dn5 / locals.var_tratio), (locals.var_tratio_dn6 / locals.var_tratio), (locals.var_tratio_dn7 / locals.var_tratio), (locals.var_tratio_dn8 / locals.var_tratio), (locals.var_tratio_dn9 / locals.var_tratio), (locals.var_tratio_dn10 / locals.var_tratio), (locals.var_tratio_dn11 / locals.var_tratio), (locals.var_tratio_dn14 / locals.var_tratio),)
    } else {
        (locals.var_log_tratio, locals.var_log_tratio_dn0, locals.var_log_tratio_dn2, locals.var_log_tratio_dn4, locals.var_log_tratio_dn5, locals.var_log_tratio_dn6, locals.var_log_tratio_dn7, locals.var_log_tratio_dn8, locals.var_log_tratio_dn9, locals.var_log_tratio_dn10, locals.var_log_tratio_dn11, locals.var_log_tratio_dn14,)
    }
};
        locals.var_log_tratio = assign13070_e7390;
        locals.var_log_tratio_dn0 = assign13070_e7390_d_n0;
        locals.var_log_tratio_dn2 = assign13070_e7390_d_n2;
        locals.var_log_tratio_dn4 = assign13070_e7390_d_n4;
        locals.var_log_tratio_dn5 = assign13070_e7390_d_n5;
        locals.var_log_tratio_dn6 = assign13070_e7390_d_n6;
        locals.var_log_tratio_dn7 = assign13070_e7390_d_n7;
        locals.var_log_tratio_dn8 = assign13070_e7390_d_n8;
        locals.var_log_tratio_dn9 = assign13070_e7390_d_n9;
        locals.var_log_tratio_dn10 = assign13070_e7390_d_n10;
        locals.var_log_tratio_dn11 = assign13070_e7390_d_n11;
        locals.var_log_tratio_dn14 = assign13070_e7390_d_n14;

        let (assign13080_e7402, assign13080_e7402_d_n0, assign13080_e7402_d_n2, assign13080_e7402_d_n4, assign13080_e7402_d_n5, assign13080_e7402_d_n6, assign13080_e7402_d_n7, assign13080_e7402_d_n8, assign13080_e7402_d_n9, assign13080_e7402_d_n10, assign13080_e7402_d_n11, assign13080_e7402_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13080_e7395: f64 = (locals.var_uc_bgtmp1 * locals.var_tdiff);
        let assign13080_e7396: f64 = (locals.var_egtnom - assign13080_e7395);
        let assign13080_e7399: f64 = (locals.var_uc_bgtmp2 * locals.var_tdiff_2);
        let assign13080_e7400: f64 = (assign13080_e7396 - assign13080_e7399);
        (assign13080_e7400, ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn0)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn0)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn2)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn2)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn4)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn4)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn5)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn5)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn6)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn6)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn7)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn7)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn8)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn8)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn9)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn9)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn10)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn10)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn11)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn11)), ((-(locals.var_uc_bgtmp1 * locals.var_tdiff_dn14)) - (locals.var_uc_bgtmp2 * locals.var_tdiff_2_dn14)),)
    } else {
        (locals.var_eg, locals.var_eg_dn0, locals.var_eg_dn2, locals.var_eg_dn4, locals.var_eg_dn5, locals.var_eg_dn6, locals.var_eg_dn7, locals.var_eg_dn8, locals.var_eg_dn9, locals.var_eg_dn10, locals.var_eg_dn11, locals.var_eg_dn14,)
    }
};
        locals.var_eg = assign13080_e7402;
        locals.var_eg_dn0 = assign13080_e7402_d_n0;
        locals.var_eg_dn2 = assign13080_e7402_d_n2;
        locals.var_eg_dn4 = assign13080_e7402_d_n4;
        locals.var_eg_dn5 = assign13080_e7402_d_n5;
        locals.var_eg_dn6 = assign13080_e7402_d_n6;
        locals.var_eg_dn7 = assign13080_e7402_d_n7;
        locals.var_eg_dn8 = assign13080_e7402_d_n8;
        locals.var_eg_dn9 = assign13080_e7402_d_n9;
        locals.var_eg_dn10 = assign13080_e7402_d_n10;
        locals.var_eg_dn11 = assign13080_e7402_d_n11;
        locals.var_eg_dn14 = assign13080_e7402_d_n14;

        let (assign13090_e7407, assign13090_e7407_d_n0, assign13090_e7407_d_n2, assign13090_e7407_d_n4, assign13090_e7407_d_n5, assign13090_e7407_d_n6, assign13090_e7407_d_n7, assign13090_e7407_d_n8, assign13090_e7407_d_n9, assign13090_e7407_d_n10, assign13090_e7407_d_n11, assign13090_e7407_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13090_e7405: f64 = (locals.var_eg).sqrt();
        (assign13090_e7405, (locals.var_eg_dn0 / (2.0 * assign13090_e7405)), (locals.var_eg_dn2 / (2.0 * assign13090_e7405)), (locals.var_eg_dn4 / (2.0 * assign13090_e7405)), (locals.var_eg_dn5 / (2.0 * assign13090_e7405)), (locals.var_eg_dn6 / (2.0 * assign13090_e7405)), (locals.var_eg_dn7 / (2.0 * assign13090_e7405)), (locals.var_eg_dn8 / (2.0 * assign13090_e7405)), (locals.var_eg_dn9 / (2.0 * assign13090_e7405)), (locals.var_eg_dn10 / (2.0 * assign13090_e7405)), (locals.var_eg_dn11 / (2.0 * assign13090_e7405)), (locals.var_eg_dn14 / (2.0 * assign13090_e7405)),)
    } else {
        (locals.var_sqrt_eg, locals.var_sqrt_eg_dn0, locals.var_sqrt_eg_dn2, locals.var_sqrt_eg_dn4, locals.var_sqrt_eg_dn5, locals.var_sqrt_eg_dn6, locals.var_sqrt_eg_dn7, locals.var_sqrt_eg_dn8, locals.var_sqrt_eg_dn9, locals.var_sqrt_eg_dn10, locals.var_sqrt_eg_dn11, locals.var_sqrt_eg_dn14,)
    }
};
        locals.var_sqrt_eg = assign13090_e7407;
        locals.var_sqrt_eg_dn0 = assign13090_e7407_d_n0;
        locals.var_sqrt_eg_dn2 = assign13090_e7407_d_n2;
        locals.var_sqrt_eg_dn4 = assign13090_e7407_d_n4;
        locals.var_sqrt_eg_dn5 = assign13090_e7407_d_n5;
        locals.var_sqrt_eg_dn6 = assign13090_e7407_d_n6;
        locals.var_sqrt_eg_dn7 = assign13090_e7407_d_n7;
        locals.var_sqrt_eg_dn8 = assign13090_e7407_d_n8;
        locals.var_sqrt_eg_dn9 = assign13090_e7407_d_n9;
        locals.var_sqrt_eg_dn10 = assign13090_e7407_d_n10;
        locals.var_sqrt_eg_dn11 = assign13090_e7407_d_n11;
        locals.var_sqrt_eg_dn14 = assign13090_e7407_d_n14;

        let (assign13100_e7413, assign13100_e7413_d_n0, assign13100_e7413_d_n2, assign13100_e7413_d_n4, assign13100_e7413_d_n5, assign13100_e7413_d_n6, assign13100_e7413_d_n7, assign13100_e7413_d_n8, assign13100_e7413_d_n9, assign13100_e7413_d_n10, assign13100_e7413_d_n11, assign13100_e7413_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13100_e7411: f64 = (1.0 / locals.var_ttemp);
        (assign13100_e7411, (-(locals.var_ttemp_dn0 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn2 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn4 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn5 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn6 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn7 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn8 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn9 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn10 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn11 / (locals.var_ttemp * locals.var_ttemp))), (-(locals.var_ttemp_dn14 / (locals.var_ttemp * locals.var_ttemp))),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign13100_e7413;
        locals.var_t1_dn0 = assign13100_e7413_d_n0;
        locals.var_t1_dn2 = assign13100_e7413_d_n2;
        locals.var_t1_dn4 = assign13100_e7413_d_n4;
        locals.var_t1_dn5 = assign13100_e7413_d_n5;
        locals.var_t1_dn6 = assign13100_e7413_d_n6;
        locals.var_t1_dn7 = assign13100_e7413_d_n7;
        locals.var_t1_dn8 = assign13100_e7413_d_n8;
        locals.var_t1_dn9 = assign13100_e7413_d_n9;
        locals.var_t1_dn10 = assign13100_e7413_d_n10;
        locals.var_t1_dn11 = assign13100_e7413_d_n11;
        locals.var_t1_dn14 = assign13100_e7413_d_n14;

        let (assign13110_e7419, assign13110_e7419_d_n0, assign13110_e7419_d_n2, assign13110_e7419_d_n4, assign13110_e7419_d_n5, assign13110_e7419_d_n6, assign13110_e7419_d_n7, assign13110_e7419_d_n8, assign13110_e7419_d_n9, assign13110_e7419_d_n10, assign13110_e7419_d_n11, assign13110_e7419_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13110_e7417: f64 = (1.0 / locals.var_ktnom);
        (assign13110_e7417, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign13110_e7419;
        locals.var_t2_dn0 = assign13110_e7419_d_n0;
        locals.var_t2_dn2 = assign13110_e7419_d_n2;
        locals.var_t2_dn4 = assign13110_e7419_d_n4;
        locals.var_t2_dn5 = assign13110_e7419_d_n5;
        locals.var_t2_dn6 = assign13110_e7419_d_n6;
        locals.var_t2_dn7 = assign13110_e7419_d_n7;
        locals.var_t2_dn8 = assign13110_e7419_d_n8;
        locals.var_t2_dn9 = assign13110_e7419_d_n9;
        locals.var_t2_dn10 = assign13110_e7419_d_n10;
        locals.var_t2_dn11 = assign13110_e7419_d_n11;
        locals.var_t2_dn14 = assign13110_e7419_d_n14;

    }

    pub(super) fn stamp_transient_block_22(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13120_e7441, assign13120_e7441_d_n0, assign13120_e7441_d_n2, assign13120_e7441_d_n4, assign13120_e7441_d_n5, assign13120_e7441_d_n6, assign13120_e7441_d_n7, assign13120_e7441_d_n8, assign13120_e7441_d_n9, assign13120_e7441_d_n10, assign13120_e7441_d_n11, assign13120_e7441_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13120_e7423: f64 = (locals.var_egtnom + p.p259);
        let assign13120_e7427: f64 = (locals.var_t1 - locals.var_t2);
        let assign13120_e7428: f64 = (p.p260 * assign13120_e7427);
        let assign13120_e7429: f64 = (assign13120_e7423 + assign13120_e7428);
        let assign13120_e7433: f64 = (locals.var_t1 * locals.var_t1);
        let assign13120_e7436: f64 = (locals.var_t2 * locals.var_t2);
        let assign13120_e7437: f64 = (assign13120_e7433 - assign13120_e7436);
        let assign13120_e7438: f64 = (p.p261 * assign13120_e7437);
        let assign13120_e7439: f64 = (assign13120_e7429 + assign13120_e7438);
        (assign13120_e7439, ((p.p260 * (locals.var_t1_dn0 - locals.var_t2_dn0)) + (p.p261 * (((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)) - ((locals.var_t2_dn0 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn0))))), ((p.p260 * (locals.var_t1_dn2 - locals.var_t2_dn2)) + (p.p261 * (((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)) - ((locals.var_t2_dn2 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn2))))), ((p.p260 * (locals.var_t1_dn4 - locals.var_t2_dn4)) + (p.p261 * (((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)) - ((locals.var_t2_dn4 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn4))))), ((p.p260 * (locals.var_t1_dn5 - locals.var_t2_dn5)) + (p.p261 * (((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)) - ((locals.var_t2_dn5 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn5))))), ((p.p260 * (locals.var_t1_dn6 - locals.var_t2_dn6)) + (p.p261 * (((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)) - ((locals.var_t2_dn6 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn6))))), ((p.p260 * (locals.var_t1_dn7 - locals.var_t2_dn7)) + (p.p261 * (((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)) - ((locals.var_t2_dn7 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn7))))), ((p.p260 * (locals.var_t1_dn8 - locals.var_t2_dn8)) + (p.p261 * (((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)) - ((locals.var_t2_dn8 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn8))))), ((p.p260 * (locals.var_t1_dn9 - locals.var_t2_dn9)) + (p.p261 * (((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)) - ((locals.var_t2_dn9 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn9))))), ((p.p260 * (locals.var_t1_dn10 - locals.var_t2_dn10)) + (p.p261 * (((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)) - ((locals.var_t2_dn10 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn10))))), ((p.p260 * (locals.var_t1_dn11 - locals.var_t2_dn11)) + (p.p261 * (((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)) - ((locals.var_t2_dn11 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn11))))), ((p.p260 * (locals.var_t1_dn14 - locals.var_t2_dn14)) + (p.p261 * (((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)) - ((locals.var_t2_dn14 * locals.var_t2) + (locals.var_t2 * locals.var_t2_dn14))))),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign13120_e7441;
        locals.var_t3_dn0 = assign13120_e7441_d_n0;
        locals.var_t3_dn2 = assign13120_e7441_d_n2;
        locals.var_t3_dn4 = assign13120_e7441_d_n4;
        locals.var_t3_dn5 = assign13120_e7441_d_n5;
        locals.var_t3_dn6 = assign13120_e7441_d_n6;
        locals.var_t3_dn7 = assign13120_e7441_d_n7;
        locals.var_t3_dn8 = assign13120_e7441_d_n8;
        locals.var_t3_dn9 = assign13120_e7441_d_n9;
        locals.var_t3_dn10 = assign13120_e7441_d_n10;
        locals.var_t3_dn11 = assign13120_e7441_d_n11;
        locals.var_t3_dn14 = assign13120_e7441_d_n14;

        let (assign13130_e7446, assign13130_e7446_d_n0, assign13130_e7446_d_n2, assign13130_e7446_d_n4, assign13130_e7446_d_n5, assign13130_e7446_d_n6, assign13130_e7446_d_n7, assign13130_e7446_d_n8, assign13130_e7446_d_n9, assign13130_e7446_d_n10, assign13130_e7446_d_n11, assign13130_e7446_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13130_e7444: f64 = (locals.var_t3).sqrt();
        (assign13130_e7444, (locals.var_t3_dn0 / (2.0 * assign13130_e7444)), (locals.var_t3_dn2 / (2.0 * assign13130_e7444)), (locals.var_t3_dn4 / (2.0 * assign13130_e7444)), (locals.var_t3_dn5 / (2.0 * assign13130_e7444)), (locals.var_t3_dn6 / (2.0 * assign13130_e7444)), (locals.var_t3_dn7 / (2.0 * assign13130_e7444)), (locals.var_t3_dn8 / (2.0 * assign13130_e7444)), (locals.var_t3_dn9 / (2.0 * assign13130_e7444)), (locals.var_t3_dn10 / (2.0 * assign13130_e7444)), (locals.var_t3_dn11 / (2.0 * assign13130_e7444)), (locals.var_t3_dn14 / (2.0 * assign13130_e7444)),)
    } else {
        (locals.var_egp12, locals.var_egp12_dn0, locals.var_egp12_dn2, locals.var_egp12_dn4, locals.var_egp12_dn5, locals.var_egp12_dn6, locals.var_egp12_dn7, locals.var_egp12_dn8, locals.var_egp12_dn9, locals.var_egp12_dn10, locals.var_egp12_dn11, locals.var_egp12_dn14,)
    }
};
        locals.var_egp12 = assign13130_e7446;
        locals.var_egp12_dn0 = assign13130_e7446_d_n0;
        locals.var_egp12_dn2 = assign13130_e7446_d_n2;
        locals.var_egp12_dn4 = assign13130_e7446_d_n4;
        locals.var_egp12_dn5 = assign13130_e7446_d_n5;
        locals.var_egp12_dn6 = assign13130_e7446_d_n6;
        locals.var_egp12_dn7 = assign13130_e7446_d_n7;
        locals.var_egp12_dn8 = assign13130_e7446_d_n8;
        locals.var_egp12_dn9 = assign13130_e7446_d_n9;
        locals.var_egp12_dn10 = assign13130_e7446_d_n10;
        locals.var_egp12_dn11 = assign13130_e7446_d_n11;
        locals.var_egp12_dn14 = assign13130_e7446_d_n14;

        let (assign13140_e7452, assign13140_e7452_d_n0, assign13140_e7452_d_n2, assign13140_e7452_d_n4, assign13140_e7452_d_n5, assign13140_e7452_d_n6, assign13140_e7452_d_n7, assign13140_e7452_d_n8, assign13140_e7452_d_n9, assign13140_e7452_d_n10, assign13140_e7452_d_n11, assign13140_e7452_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13140_e7450: f64 = (locals.var_t3 * locals.var_egp12);
        (assign13140_e7450, ((locals.var_t3_dn0 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn0)), ((locals.var_t3_dn2 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn2)), ((locals.var_t3_dn4 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn4)), ((locals.var_t3_dn5 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn5)), ((locals.var_t3_dn6 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn6)), ((locals.var_t3_dn7 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn7)), ((locals.var_t3_dn8 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn8)), ((locals.var_t3_dn9 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn9)), ((locals.var_t3_dn10 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn10)), ((locals.var_t3_dn11 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn11)), ((locals.var_t3_dn14 * locals.var_egp12) + (locals.var_t3 * locals.var_egp12_dn14)),)
    } else {
        (locals.var_egp32, locals.var_egp32_dn0, locals.var_egp32_dn2, locals.var_egp32_dn4, locals.var_egp32_dn5, locals.var_egp32_dn6, locals.var_egp32_dn7, locals.var_egp32_dn8, locals.var_egp32_dn9, locals.var_egp32_dn10, locals.var_egp32_dn11, locals.var_egp32_dn14,)
    }
};
        locals.var_egp32 = assign13140_e7452;
        locals.var_egp32_dn0 = assign13140_e7452_d_n0;
        locals.var_egp32_dn2 = assign13140_e7452_d_n2;
        locals.var_egp32_dn4 = assign13140_e7452_d_n4;
        locals.var_egp32_dn5 = assign13140_e7452_d_n5;
        locals.var_egp32_dn6 = assign13140_e7452_d_n6;
        locals.var_egp32_dn7 = assign13140_e7452_d_n7;
        locals.var_egp32_dn8 = assign13140_e7452_d_n8;
        locals.var_egp32_dn9 = assign13140_e7452_d_n9;
        locals.var_egp32_dn10 = assign13140_e7452_d_n10;
        locals.var_egp32_dn11 = assign13140_e7452_d_n11;
        locals.var_egp32_dn14 = assign13140_e7452_d_n14;

        let (assign13150_e7460, assign13150_e7460_d_n0, assign13150_e7460_d_n2, assign13150_e7460_d_n4, assign13150_e7460_d_n5, assign13150_e7460_d_n6, assign13150_e7460_d_n7, assign13150_e7460_d_n8, assign13150_e7460_d_n9, assign13150_e7460_d_n10, assign13150_e7460_d_n11, assign13150_e7460_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13150_e7457: f64 = (1.3806226e-23 * locals.var_ttemp);
        let assign13150_e7458: f64 = (1.6021918e-19 / assign13150_e7457);
        (assign13150_e7458, (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn0)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn2)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn4)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn5)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn6)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn7)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn8)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn9)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn10)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn11)) / (assign13150_e7457 * assign13150_e7457))), (-((1.6021918e-19 * (1.3806226e-23 * locals.var_ttemp_dn14)) / (assign13150_e7457 * assign13150_e7457))),)
    } else {
        (locals.var_beta, locals.var_beta_dn0, locals.var_beta_dn2, locals.var_beta_dn4, locals.var_beta_dn5, locals.var_beta_dn6, locals.var_beta_dn7, locals.var_beta_dn8, locals.var_beta_dn9, locals.var_beta_dn10, locals.var_beta_dn11, locals.var_beta_dn14,)
    }
};
        locals.var_beta = assign13150_e7460;
        locals.var_beta_dn0 = assign13150_e7460_d_n0;
        locals.var_beta_dn2 = assign13150_e7460_d_n2;
        locals.var_beta_dn4 = assign13150_e7460_d_n4;
        locals.var_beta_dn5 = assign13150_e7460_d_n5;
        locals.var_beta_dn6 = assign13150_e7460_d_n6;
        locals.var_beta_dn7 = assign13150_e7460_d_n7;
        locals.var_beta_dn8 = assign13150_e7460_d_n8;
        locals.var_beta_dn9 = assign13150_e7460_d_n9;
        locals.var_beta_dn10 = assign13150_e7460_d_n10;
        locals.var_beta_dn11 = assign13150_e7460_d_n11;
        locals.var_beta_dn14 = assign13150_e7460_d_n14;

        let (assign13160_e7466, assign13160_e7466_d_n0, assign13160_e7466_d_n2, assign13160_e7466_d_n4, assign13160_e7466_d_n5, assign13160_e7466_d_n6, assign13160_e7466_d_n7, assign13160_e7466_d_n8, assign13160_e7466_d_n9, assign13160_e7466_d_n10, assign13160_e7466_d_n11, assign13160_e7466_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13160_e7464: f64 = (1.0 / locals.var_beta);
        (assign13160_e7464, (-(locals.var_beta_dn0 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn2 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn4 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn5 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn6 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn7 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn8 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn9 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn10 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn11 / (locals.var_beta * locals.var_beta))), (-(locals.var_beta_dn14 / (locals.var_beta * locals.var_beta))),)
    } else {
        (locals.var_beta_inv, locals.var_beta_inv_dn0, locals.var_beta_inv_dn2, locals.var_beta_inv_dn4, locals.var_beta_inv_dn5, locals.var_beta_inv_dn6, locals.var_beta_inv_dn7, locals.var_beta_inv_dn8, locals.var_beta_inv_dn9, locals.var_beta_inv_dn10, locals.var_beta_inv_dn11, locals.var_beta_inv_dn14,)
    }
};
        locals.var_beta_inv = assign13160_e7466;
        locals.var_beta_inv_dn0 = assign13160_e7466_d_n0;
        locals.var_beta_inv_dn2 = assign13160_e7466_d_n2;
        locals.var_beta_inv_dn4 = assign13160_e7466_d_n4;
        locals.var_beta_inv_dn5 = assign13160_e7466_d_n5;
        locals.var_beta_inv_dn6 = assign13160_e7466_d_n6;
        locals.var_beta_inv_dn7 = assign13160_e7466_d_n7;
        locals.var_beta_inv_dn8 = assign13160_e7466_d_n8;
        locals.var_beta_inv_dn9 = assign13160_e7466_d_n9;
        locals.var_beta_inv_dn10 = assign13160_e7466_d_n10;
        locals.var_beta_inv_dn11 = assign13160_e7466_d_n11;
        locals.var_beta_inv_dn14 = assign13160_e7466_d_n14;

        let (assign13170_e7472, assign13170_e7472_d_n0, assign13170_e7472_d_n2, assign13170_e7472_d_n4, assign13170_e7472_d_n5, assign13170_e7472_d_n6, assign13170_e7472_d_n7, assign13170_e7472_d_n8, assign13170_e7472_d_n9, assign13170_e7472_d_n10, assign13170_e7472_d_n11, assign13170_e7472_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13170_e7470: f64 = (locals.var_beta * locals.var_beta);
        (assign13170_e7470, ((locals.var_beta_dn0 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn0)), ((locals.var_beta_dn2 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn2)), ((locals.var_beta_dn4 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn4)), ((locals.var_beta_dn5 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn5)), ((locals.var_beta_dn6 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn6)), ((locals.var_beta_dn7 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn7)), ((locals.var_beta_dn8 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn8)), ((locals.var_beta_dn9 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn9)), ((locals.var_beta_dn10 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn10)), ((locals.var_beta_dn11 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn11)), ((locals.var_beta_dn14 * locals.var_beta) + (locals.var_beta * locals.var_beta_dn14)),)
    } else {
        (locals.var_beta2, locals.var_beta2_dn0, locals.var_beta2_dn2, locals.var_beta2_dn4, locals.var_beta2_dn5, locals.var_beta2_dn6, locals.var_beta2_dn7, locals.var_beta2_dn8, locals.var_beta2_dn9, locals.var_beta2_dn10, locals.var_beta2_dn11, locals.var_beta2_dn14,)
    }
};
        locals.var_beta2 = assign13170_e7472;
        locals.var_beta2_dn0 = assign13170_e7472_d_n0;
        locals.var_beta2_dn2 = assign13170_e7472_d_n2;
        locals.var_beta2_dn4 = assign13170_e7472_d_n4;
        locals.var_beta2_dn5 = assign13170_e7472_d_n5;
        locals.var_beta2_dn6 = assign13170_e7472_d_n6;
        locals.var_beta2_dn7 = assign13170_e7472_d_n7;
        locals.var_beta2_dn8 = assign13170_e7472_d_n8;
        locals.var_beta2_dn9 = assign13170_e7472_d_n9;
        locals.var_beta2_dn10 = assign13170_e7472_d_n10;
        locals.var_beta2_dn11 = assign13170_e7472_d_n11;
        locals.var_beta2_dn14 = assign13170_e7472_d_n14;

        let (assign13180_e7480,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13180_e7477: f64 = (1.3806226e-23 * locals.var_ktnom);
        let assign13180_e7478: f64 = (1.6021918e-19 / assign13180_e7477);
        (assign13180_e7478,)
    } else {
        (locals.var_betatnom,)
    }
};
        locals.var_betatnom = assign13180_e7480;

        let (assign13190_e7503, assign13190_e7503_d_n0, assign13190_e7503_d_n2, assign13190_e7503_d_n4, assign13190_e7503_d_n5, assign13190_e7503_d_n6, assign13190_e7503_d_n7, assign13190_e7503_d_n8, assign13190_e7503_d_n9, assign13190_e7503_d_n10, assign13190_e7503_d_n11, assign13190_e7503_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13190_e7485: f64 = (locals.var_log_tratio * 1.5);
        let assign13190_e7486: f64 = (assign13190_e7485).exp();
        let assign13190_e7487: f64 = (1.04e16 * assign13190_e7486);
        let assign13190_e7489: f64 = (-locals.var_eg);
        let assign13190_e7491: f64 = (assign13190_e7489 / 2.0);
        let assign13190_e7493: f64 = (assign13190_e7491 * locals.var_beta);
        let assign13190_e7496: f64 = (locals.var_egtnom / 2.0);
        let assign13190_e7498: f64 = (assign13190_e7496 * locals.var_betatnom);
        let assign13190_e7499: f64 = (assign13190_e7493 + assign13190_e7498);
        let assign13190_e7500: f64 = (assign13190_e7499).exp();
        let assign13190_e7501: f64 = (assign13190_e7487 * assign13190_e7500);
        (assign13190_e7501, (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn0 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn0) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn0))))), (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn2 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn2) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn2))))), (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn4 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn4) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn4))))), (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn5 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn5) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn5))))), (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn6 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn6) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn6))))), (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn7 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn7) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn7))))), (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn8 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn8) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn8))))), (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn9 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn9) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn9))))), (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn10 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn10) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn10))))), (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn11 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn11) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn11))))), (((1.04e16 * (assign13190_e7486 * (locals.var_log_tratio_dn14 * 1.5))) * assign13190_e7500) + (assign13190_e7487 * (assign13190_e7500 * ((((-locals.var_eg_dn14) / 2.0) * locals.var_beta) + (assign13190_e7491 * locals.var_beta_dn14))))),)
    } else {
        (locals.var_nin, locals.var_nin_dn0, locals.var_nin_dn2, locals.var_nin_dn4, locals.var_nin_dn5, locals.var_nin_dn6, locals.var_nin_dn7, locals.var_nin_dn8, locals.var_nin_dn9, locals.var_nin_dn10, locals.var_nin_dn11, locals.var_nin_dn14,)
    }
};
        locals.var_nin = assign13190_e7503;
        locals.var_nin_dn0 = assign13190_e7503_d_n0;
        locals.var_nin_dn2 = assign13190_e7503_d_n2;
        locals.var_nin_dn4 = assign13190_e7503_d_n4;
        locals.var_nin_dn5 = assign13190_e7503_d_n5;
        locals.var_nin_dn6 = assign13190_e7503_d_n6;
        locals.var_nin_dn7 = assign13190_e7503_d_n7;
        locals.var_nin_dn8 = assign13190_e7503_d_n8;
        locals.var_nin_dn9 = assign13190_e7503_d_n9;
        locals.var_nin_dn10 = assign13190_e7503_d_n10;
        locals.var_nin_dn11 = assign13190_e7503_d_n11;
        locals.var_nin_dn14 = assign13190_e7503_d_n14;

        let (assign13200_e7510, assign13200_e7510_d_n0, assign13200_e7510_d_n2, assign13200_e7510_d_n4, assign13200_e7510_d_n5, assign13200_e7510_d_n6, assign13200_e7510_d_n7, assign13200_e7510_d_n8, assign13200_e7510_d_n9, assign13200_e7510_d_n10, assign13200_e7510_d_n11, assign13200_e7510_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13200_e7507: f64 = (locals.var_log_tratio * locals.var_uc_muetmp);
        let assign13200_e7508: f64 = (assign13200_e7507).exp();
        (assign13200_e7508, (assign13200_e7508 * (locals.var_log_tratio_dn0 * locals.var_uc_muetmp)), (assign13200_e7508 * (locals.var_log_tratio_dn2 * locals.var_uc_muetmp)), (assign13200_e7508 * (locals.var_log_tratio_dn4 * locals.var_uc_muetmp)), (assign13200_e7508 * (locals.var_log_tratio_dn5 * locals.var_uc_muetmp)), (assign13200_e7508 * (locals.var_log_tratio_dn6 * locals.var_uc_muetmp)), (assign13200_e7508 * (locals.var_log_tratio_dn7 * locals.var_uc_muetmp)), (assign13200_e7508 * (locals.var_log_tratio_dn8 * locals.var_uc_muetmp)), (assign13200_e7508 * (locals.var_log_tratio_dn9 * locals.var_uc_muetmp)), (assign13200_e7508 * (locals.var_log_tratio_dn10 * locals.var_uc_muetmp)), (assign13200_e7508 * (locals.var_log_tratio_dn11 * locals.var_uc_muetmp)), (assign13200_e7508 * (locals.var_log_tratio_dn14 * locals.var_uc_muetmp)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign13200_e7510;
        locals.var_t1_dn0 = assign13200_e7510_d_n0;
        locals.var_t1_dn2 = assign13200_e7510_d_n2;
        locals.var_t1_dn4 = assign13200_e7510_d_n4;
        locals.var_t1_dn5 = assign13200_e7510_d_n5;
        locals.var_t1_dn6 = assign13200_e7510_d_n6;
        locals.var_t1_dn7 = assign13200_e7510_d_n7;
        locals.var_t1_dn8 = assign13200_e7510_d_n8;
        locals.var_t1_dn9 = assign13200_e7510_d_n9;
        locals.var_t1_dn10 = assign13200_e7510_d_n10;
        locals.var_t1_dn11 = assign13200_e7510_d_n11;
        locals.var_t1_dn14 = assign13200_e7510_d_n14;

        let (assign13210_e7516, assign13210_e7516_d_n0, assign13210_e7516_d_n2, assign13210_e7516_d_n4, assign13210_e7516_d_n5, assign13210_e7516_d_n6, assign13210_e7516_d_n7, assign13210_e7516_d_n8, assign13210_e7516_d_n9, assign13210_e7516_d_n10, assign13210_e7516_d_n11, assign13210_e7516_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13210_e7514: f64 = (locals.var_t1 / locals.var_mueph);
        (assign13210_e7514, (((locals.var_t1_dn0 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn0)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn2 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn2)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn4 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn4)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn5 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn5)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn6 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn6)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn7 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn7)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn8 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn8)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn9 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn9)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn10 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn10)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn11 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn11)) / (locals.var_mueph * locals.var_mueph)), (((locals.var_t1_dn14 * locals.var_mueph) - (locals.var_t1 * locals.var_mueph_dn14)) / (locals.var_mueph * locals.var_mueph)),)
    } else {
        (locals.var_mphn0, locals.var_mphn0_dn0, locals.var_mphn0_dn2, locals.var_mphn0_dn4, locals.var_mphn0_dn5, locals.var_mphn0_dn6, locals.var_mphn0_dn7, locals.var_mphn0_dn8, locals.var_mphn0_dn9, locals.var_mphn0_dn10, locals.var_mphn0_dn11, locals.var_mphn0_dn14,)
    }
};
        locals.var_mphn0 = assign13210_e7516;
        locals.var_mphn0_dn0 = assign13210_e7516_d_n0;
        locals.var_mphn0_dn2 = assign13210_e7516_d_n2;
        locals.var_mphn0_dn4 = assign13210_e7516_d_n4;
        locals.var_mphn0_dn5 = assign13210_e7516_d_n5;
        locals.var_mphn0_dn6 = assign13210_e7516_d_n6;
        locals.var_mphn0_dn7 = assign13210_e7516_d_n7;
        locals.var_mphn0_dn8 = assign13210_e7516_d_n8;
        locals.var_mphn0_dn9 = assign13210_e7516_d_n9;
        locals.var_mphn0_dn10 = assign13210_e7516_d_n10;
        locals.var_mphn0_dn11 = assign13210_e7516_d_n11;
        locals.var_mphn0_dn14 = assign13210_e7516_d_n14;

        let assign13220_e7523: f64 = if ((locals.var_uc_codep != 0.0) && (locals.var_uc_codep < 3.0)) { 1.0 } else { 0.0 };
        locals.var_guard292 = assign13220_e7523;

        let (assign13230_e7538, assign13230_e7538_d_n0, assign13230_e7538_d_n2, assign13230_e7538_d_n4, assign13230_e7538_d_n5, assign13230_e7538_d_n6, assign13230_e7538_d_n7, assign13230_e7538_d_n8, assign13230_e7538_d_n9, assign13230_e7538_d_n10, assign13230_e7538_d_n11, assign13230_e7538_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) {
        let assign13230_e7529: f64 = (2.0 * 1.034943e-10);
        let assign13230_e7531: f64 = (assign13230_e7529 * 1.6021918e-19);
        let assign13230_e7533: f64 = (assign13230_e7531 * locals.var_uc_ndepm);
        let assign13230_e7535: f64 = (assign13230_e7533 * locals.var_beta_inv);
        let assign13230_e7536: f64 = (assign13230_e7535).sqrt();
        (assign13230_e7536, ((((assign13230_e7531 * locals.var_uc_ndepm_dn0) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn0)) / (2.0 * assign13230_e7536)), ((((assign13230_e7531 * locals.var_uc_ndepm_dn2) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn2)) / (2.0 * assign13230_e7536)), ((((assign13230_e7531 * locals.var_uc_ndepm_dn4) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn4)) / (2.0 * assign13230_e7536)), ((((assign13230_e7531 * locals.var_uc_ndepm_dn5) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn5)) / (2.0 * assign13230_e7536)), ((((assign13230_e7531 * locals.var_uc_ndepm_dn6) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn6)) / (2.0 * assign13230_e7536)), ((((assign13230_e7531 * locals.var_uc_ndepm_dn7) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn7)) / (2.0 * assign13230_e7536)), ((((assign13230_e7531 * locals.var_uc_ndepm_dn8) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn8)) / (2.0 * assign13230_e7536)), ((((assign13230_e7531 * locals.var_uc_ndepm_dn9) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn9)) / (2.0 * assign13230_e7536)), ((((assign13230_e7531 * locals.var_uc_ndepm_dn10) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn10)) / (2.0 * assign13230_e7536)), ((((assign13230_e7531 * locals.var_uc_ndepm_dn11) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn11)) / (2.0 * assign13230_e7536)), ((((assign13230_e7531 * locals.var_uc_ndepm_dn14) * locals.var_beta_inv) + (assign13230_e7533 * locals.var_beta_inv_dn14)) / (2.0 * assign13230_e7536)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn11, locals.var_cnst0_dn14,)
    }
};
        locals.var_cnst0 = assign13230_e7538;
        locals.var_cnst0_dn0 = assign13230_e7538_d_n0;
        locals.var_cnst0_dn2 = assign13230_e7538_d_n2;
        locals.var_cnst0_dn4 = assign13230_e7538_d_n4;
        locals.var_cnst0_dn5 = assign13230_e7538_d_n5;
        locals.var_cnst0_dn6 = assign13230_e7538_d_n6;
        locals.var_cnst0_dn7 = assign13230_e7538_d_n7;
        locals.var_cnst0_dn8 = assign13230_e7538_d_n8;
        locals.var_cnst0_dn9 = assign13230_e7538_d_n9;
        locals.var_cnst0_dn10 = assign13230_e7538_d_n10;
        locals.var_cnst0_dn11 = assign13230_e7538_d_n11;
        locals.var_cnst0_dn14 = assign13230_e7538_d_n14;

        let (assign13240_e7550, assign13240_e7550_d_n0, assign13240_e7550_d_n2, assign13240_e7550_d_n4, assign13240_e7550_d_n5, assign13240_e7550_d_n6, assign13240_e7550_d_n7, assign13240_e7550_d_n8, assign13240_e7550_d_n9, assign13240_e7550_d_n10, assign13240_e7550_d_n11, assign13240_e7550_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) {
        let assign13240_e7544: f64 = (locals.var_nin * locals.var_nin);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_uc_ndepm;
        let assign13240_e7546: f64 = (assign13240_e7544 * __rspice_inv_cse_0);
        let assign13240_e7548: f64 = (assign13240_e7546 * __rspice_inv_cse_0);
        (assign13240_e7548, ((((((((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn11 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn11)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn11)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn11)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn14 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn14)) * locals.var_uc_ndepm) - (assign13240_e7544 * locals.var_uc_ndepm_dn14)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13240_e7546 * locals.var_uc_ndepm_dn14)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn11, locals.var_cnst1_dn14,)
    }
};
        locals.var_cnst1 = assign13240_e7550;
        locals.var_cnst1_dn0 = assign13240_e7550_d_n0;
        locals.var_cnst1_dn2 = assign13240_e7550_d_n2;
        locals.var_cnst1_dn4 = assign13240_e7550_d_n4;
        locals.var_cnst1_dn5 = assign13240_e7550_d_n5;
        locals.var_cnst1_dn6 = assign13240_e7550_d_n6;
        locals.var_cnst1_dn7 = assign13240_e7550_d_n7;
        locals.var_cnst1_dn8 = assign13240_e7550_d_n8;
        locals.var_cnst1_dn9 = assign13240_e7550_d_n9;
        locals.var_cnst1_dn10 = assign13240_e7550_d_n10;
        locals.var_cnst1_dn11 = assign13240_e7550_d_n11;
        locals.var_cnst1_dn14 = assign13240_e7550_d_n14;

        let (assign13250_e7563, assign13250_e7563_d_n0, assign13250_e7563_d_n2, assign13250_e7563_d_n4, assign13250_e7563_d_n5, assign13250_e7563_d_n6, assign13250_e7563_d_n7, assign13250_e7563_d_n8, assign13250_e7563_d_n9, assign13250_e7563_d_n10, assign13250_e7563_d_n11, assign13250_e7563_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) {
        let assign13250_e7556: f64 = (2.0 * locals.var_beta_inv);
        let assign13250_e7559: f64 = (locals.var_uc_ndepm / locals.var_nin);
        let assign13250_e7560: f64 = (assign13250_e7559).ln();
        let assign13250_e7561: f64 = (assign13250_e7556 * assign13250_e7560);
        (assign13250_e7561, (((2.0 * locals.var_beta_inv_dn0) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn0 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))), (((2.0 * locals.var_beta_inv_dn2) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn2 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))), (((2.0 * locals.var_beta_inv_dn4) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn4 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))), (((2.0 * locals.var_beta_inv_dn5) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn5 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))), (((2.0 * locals.var_beta_inv_dn6) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn6 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))), (((2.0 * locals.var_beta_inv_dn7) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn7 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))), (((2.0 * locals.var_beta_inv_dn8) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn8 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))), (((2.0 * locals.var_beta_inv_dn9) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn9 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))), (((2.0 * locals.var_beta_inv_dn10) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn10 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))), (((2.0 * locals.var_beta_inv_dn11) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn11 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))), (((2.0 * locals.var_beta_inv_dn14) * assign13250_e7560) + (assign13250_e7556 * ((((locals.var_uc_ndepm_dn14 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign13250_e7559))),)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn11, locals.var_pb2n_dn14,)
    }
};
        locals.var_pb2n = assign13250_e7563;
        locals.var_pb2n_dn0 = assign13250_e7563_d_n0;
        locals.var_pb2n_dn2 = assign13250_e7563_d_n2;
        locals.var_pb2n_dn4 = assign13250_e7563_d_n4;
        locals.var_pb2n_dn5 = assign13250_e7563_d_n5;
        locals.var_pb2n_dn6 = assign13250_e7563_d_n6;
        locals.var_pb2n_dn7 = assign13250_e7563_d_n7;
        locals.var_pb2n_dn8 = assign13250_e7563_d_n8;
        locals.var_pb2n_dn9 = assign13250_e7563_d_n9;
        locals.var_pb2n_dn10 = assign13250_e7563_d_n10;
        locals.var_pb2n_dn11 = assign13250_e7563_d_n11;
        locals.var_pb2n_dn14 = assign13250_e7563_d_n14;

        let (assign13260_e7578, assign13260_e7578_d_n0, assign13260_e7578_d_n2, assign13260_e7578_d_n4, assign13260_e7578_d_n5, assign13260_e7578_d_n6, assign13260_e7578_d_n7, assign13260_e7578_d_n8, assign13260_e7578_d_n9, assign13260_e7578_d_n10, assign13260_e7578_d_n11, assign13260_e7578_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) {
        let assign13260_e7570: f64 = (locals.var_uc_ndepm * locals.var_ef_nsubc);
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_nin;
        let assign13260_e7572: f64 = (assign13260_e7570 * __rspice_inv_cse_1);
        let assign13260_e7574: f64 = (assign13260_e7572 * __rspice_inv_cse_1);
        let assign13260_e7575: f64 = (assign13260_e7574).ln();
        let assign13260_e7576: f64 = (locals.var_beta_inv * assign13260_e7575);
        (assign13260_e7576, ((locals.var_beta_inv_dn0 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))), ((locals.var_beta_inv_dn2 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))), ((locals.var_beta_inv_dn4 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))), ((locals.var_beta_inv_dn5 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))), ((locals.var_beta_inv_dn6 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))), ((locals.var_beta_inv_dn7 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))), ((locals.var_beta_inv_dn8 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))), ((locals.var_beta_inv_dn9 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))), ((locals.var_beta_inv_dn10 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))), ((locals.var_beta_inv_dn11 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn11 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn11)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))), ((locals.var_beta_inv_dn14 * assign13260_e7575) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn14 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn14)) * locals.var_nin) - (assign13260_e7570 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13260_e7572 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign13260_e7574))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn14,)
    }
};
        locals.var_vbipn = assign13260_e7578;
        locals.var_vbipn_dn0 = assign13260_e7578_d_n0;
        locals.var_vbipn_dn2 = assign13260_e7578_d_n2;
        locals.var_vbipn_dn4 = assign13260_e7578_d_n4;
        locals.var_vbipn_dn5 = assign13260_e7578_d_n5;
        locals.var_vbipn_dn6 = assign13260_e7578_d_n6;
        locals.var_vbipn_dn7 = assign13260_e7578_d_n7;
        locals.var_vbipn_dn8 = assign13260_e7578_d_n8;
        locals.var_vbipn_dn9 = assign13260_e7578_d_n9;
        locals.var_vbipn_dn10 = assign13260_e7578_d_n10;
        locals.var_vbipn_dn11 = assign13260_e7578_d_n11;
        locals.var_vbipn_dn14 = assign13260_e7578_d_n14;

        let (assign13270_e7587, assign13270_e7587_d_n0, assign13270_e7587_d_n2, assign13270_e7587_d_n4, assign13270_e7587_d_n5, assign13270_e7587_d_n6, assign13270_e7587_d_n7, assign13270_e7587_d_n8, assign13270_e7587_d_n9, assign13270_e7587_d_n10, assign13270_e7587_d_n11, assign13270_e7587_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) {
        let assign13270_e7584: f64 = (locals.var_log_tratio * p.p380);
        let assign13270_e7585: f64 = (assign13270_e7584).exp();
        (assign13270_e7585, (assign13270_e7585 * (locals.var_log_tratio_dn0 * p.p380)), (assign13270_e7585 * (locals.var_log_tratio_dn2 * p.p380)), (assign13270_e7585 * (locals.var_log_tratio_dn4 * p.p380)), (assign13270_e7585 * (locals.var_log_tratio_dn5 * p.p380)), (assign13270_e7585 * (locals.var_log_tratio_dn6 * p.p380)), (assign13270_e7585 * (locals.var_log_tratio_dn7 * p.p380)), (assign13270_e7585 * (locals.var_log_tratio_dn8 * p.p380)), (assign13270_e7585 * (locals.var_log_tratio_dn9 * p.p380)), (assign13270_e7585 * (locals.var_log_tratio_dn10 * p.p380)), (assign13270_e7585 * (locals.var_log_tratio_dn11 * p.p380)), (assign13270_e7585 * (locals.var_log_tratio_dn14 * p.p380)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign13270_e7587;
        locals.var_t1_dn0 = assign13270_e7587_d_n0;
        locals.var_t1_dn2 = assign13270_e7587_d_n2;
        locals.var_t1_dn4 = assign13270_e7587_d_n4;
        locals.var_t1_dn5 = assign13270_e7587_d_n5;
        locals.var_t1_dn6 = assign13270_e7587_d_n6;
        locals.var_t1_dn7 = assign13270_e7587_d_n7;
        locals.var_t1_dn8 = assign13270_e7587_d_n8;
        locals.var_t1_dn9 = assign13270_e7587_d_n9;
        locals.var_t1_dn10 = assign13270_e7587_d_n10;
        locals.var_t1_dn11 = assign13270_e7587_d_n11;
        locals.var_t1_dn14 = assign13270_e7587_d_n14;

        let (assign13280_e7595, assign13280_e7595_d_n0, assign13280_e7595_d_n2, assign13280_e7595_d_n4, assign13280_e7595_d_n5, assign13280_e7595_d_n6, assign13280_e7595_d_n7, assign13280_e7595_d_n8, assign13280_e7595_d_n9, assign13280_e7595_d_n10, assign13280_e7595_d_n11, assign13280_e7595_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) {
        let assign13280_e7593: f64 = (locals.var_t1 / locals.var_uc_depmueph1);
        (assign13280_e7593, (locals.var_t1_dn0 / locals.var_uc_depmueph1), (locals.var_t1_dn2 / locals.var_uc_depmueph1), (locals.var_t1_dn4 / locals.var_uc_depmueph1), (locals.var_t1_dn5 / locals.var_uc_depmueph1), (locals.var_t1_dn6 / locals.var_uc_depmueph1), (locals.var_t1_dn7 / locals.var_uc_depmueph1), (locals.var_t1_dn8 / locals.var_uc_depmueph1), (locals.var_t1_dn9 / locals.var_uc_depmueph1), (locals.var_t1_dn10 / locals.var_uc_depmueph1), (locals.var_t1_dn11 / locals.var_uc_depmueph1), (locals.var_t1_dn14 / locals.var_uc_depmueph1),)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn11, locals.var_depmphn0_dn14,)
    }
};
        locals.var_depmphn0 = assign13280_e7595;
        locals.var_depmphn0_dn0 = assign13280_e7595_d_n0;
        locals.var_depmphn0_dn2 = assign13280_e7595_d_n2;
        locals.var_depmphn0_dn4 = assign13280_e7595_d_n4;
        locals.var_depmphn0_dn5 = assign13280_e7595_d_n5;
        locals.var_depmphn0_dn6 = assign13280_e7595_d_n6;
        locals.var_depmphn0_dn7 = assign13280_e7595_d_n7;
        locals.var_depmphn0_dn8 = assign13280_e7595_d_n8;
        locals.var_depmphn0_dn9 = assign13280_e7595_d_n9;
        locals.var_depmphn0_dn10 = assign13280_e7595_d_n10;
        locals.var_depmphn0_dn11 = assign13280_e7595_d_n11;
        locals.var_depmphn0_dn14 = assign13280_e7595_d_n14;

        let (assign13290_e7617, assign13290_e7617_d_n0, assign13290_e7617_d_n2, assign13290_e7617_d_n4, assign13290_e7617_d_n5, assign13290_e7617_d_n6, assign13290_e7617_d_n7, assign13290_e7617_d_n8, assign13290_e7617_d_n9, assign13290_e7617_d_n10, assign13290_e7617_d_n11, assign13290_e7617_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) {
        let assign13290_e7602: f64 = (0.4 * locals.var_tratio);
        let assign13290_e7603: f64 = (1.8 + assign13290_e7602);
        let assign13290_e7606: f64 = (0.1 * locals.var_tratio);
        let assign13290_e7608: f64 = (assign13290_e7606 * locals.var_tratio);
        let assign13290_e7609: f64 = (assign13290_e7603 + assign13290_e7608);
        let assign13290_e7613: f64 = (1.0 - locals.var_tratio);
        let assign13290_e7614: f64 = (p.p379 * assign13290_e7613);
        let assign13290_e7615: f64 = (assign13290_e7609 - assign13290_e7614);
        (assign13290_e7615, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn0))) - (p.p379 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn2))) - (p.p379 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn4))) - (p.p379 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn5))) - (p.p379 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn6))) - (p.p379 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn7))) - (p.p379 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn8))) - (p.p379 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn9))) - (p.p379 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn10))) - (p.p379 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn11) + (((0.1 * locals.var_tratio_dn11) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn11))) - (p.p379 * (-locals.var_tratio_dn11))), (((0.4 * locals.var_tratio_dn14) + (((0.1 * locals.var_tratio_dn14) * locals.var_tratio) + (assign13290_e7606 * locals.var_tratio_dn14))) - (p.p379 * (-locals.var_tratio_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign13290_e7617;
        locals.var_t0_dn0 = assign13290_e7617_d_n0;
        locals.var_t0_dn2 = assign13290_e7617_d_n2;
        locals.var_t0_dn4 = assign13290_e7617_d_n4;
        locals.var_t0_dn5 = assign13290_e7617_d_n5;
        locals.var_t0_dn6 = assign13290_e7617_d_n6;
        locals.var_t0_dn7 = assign13290_e7617_d_n7;
        locals.var_t0_dn8 = assign13290_e7617_d_n8;
        locals.var_t0_dn9 = assign13290_e7617_d_n9;
        locals.var_t0_dn10 = assign13290_e7617_d_n10;
        locals.var_t0_dn11 = assign13290_e7617_d_n11;
        locals.var_t0_dn14 = assign13290_e7617_d_n14;

        let (assign13300_e7625, assign13300_e7625_d_n0, assign13300_e7625_d_n2, assign13300_e7625_d_n4, assign13300_e7625_d_n5, assign13300_e7625_d_n6, assign13300_e7625_d_n7, assign13300_e7625_d_n8, assign13300_e7625_d_n9, assign13300_e7625_d_n10, assign13300_e7625_d_n11, assign13300_e7625_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) {
        let assign13300_e7623: f64 = (locals.var_uc_depvmax / locals.var_t0);
        (assign13300_e7623, (((locals.var_uc_depvmax_dn0 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn2 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn4 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn5 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn6 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn7 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn8 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn9 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn10 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn11 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn14 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign13300_e7625;
        locals.var_uc_depvmax_dn0 = assign13300_e7625_d_n0;
        locals.var_uc_depvmax_dn2 = assign13300_e7625_d_n2;
        locals.var_uc_depvmax_dn4 = assign13300_e7625_d_n4;
        locals.var_uc_depvmax_dn5 = assign13300_e7625_d_n5;
        locals.var_uc_depvmax_dn6 = assign13300_e7625_d_n6;
        locals.var_uc_depvmax_dn7 = assign13300_e7625_d_n7;
        locals.var_uc_depvmax_dn8 = assign13300_e7625_d_n8;
        locals.var_uc_depvmax_dn9 = assign13300_e7625_d_n9;
        locals.var_uc_depvmax_dn10 = assign13300_e7625_d_n10;
        locals.var_uc_depvmax_dn11 = assign13300_e7625_d_n11;
        locals.var_uc_depvmax_dn14 = assign13300_e7625_d_n14;

        let assign13320_e7633: f64 = if locals.var_uc_depvmax < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard294 = assign13320_e7633;

        let (assign13330_e7641, assign13330_e7641_d_n0, assign13330_e7641_d_n2, assign13330_e7641_d_n4, assign13330_e7641_d_n5, assign13330_e7641_d_n6, assign13330_e7641_d_n7, assign13330_e7641_d_n8, assign13330_e7641_d_n9, assign13330_e7641_d_n10, assign13330_e7641_d_n11, assign13330_e7641_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) && (locals.var_guard294 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign13330_e7641;
        locals.var_uc_depvmax_dn0 = assign13330_e7641_d_n0;
        locals.var_uc_depvmax_dn2 = assign13330_e7641_d_n2;
        locals.var_uc_depvmax_dn4 = assign13330_e7641_d_n4;
        locals.var_uc_depvmax_dn5 = assign13330_e7641_d_n5;
        locals.var_uc_depvmax_dn6 = assign13330_e7641_d_n6;
        locals.var_uc_depvmax_dn7 = assign13330_e7641_d_n7;
        locals.var_uc_depvmax_dn8 = assign13330_e7641_d_n8;
        locals.var_uc_depvmax_dn9 = assign13330_e7641_d_n9;
        locals.var_uc_depvmax_dn10 = assign13330_e7641_d_n10;
        locals.var_uc_depvmax_dn11 = assign13330_e7641_d_n11;
        locals.var_uc_depvmax_dn14 = assign13330_e7641_d_n14;

        let (assign13340_e7651, assign13340_e7651_d_n0, assign13340_e7651_d_n2, assign13340_e7651_d_n4, assign13340_e7651_d_n5, assign13340_e7651_d_n6, assign13340_e7651_d_n7, assign13340_e7651_d_n8, assign13340_e7651_d_n9, assign13340_e7651_d_n10, assign13340_e7651_d_n11, assign13340_e7651_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) {
        let assign13340_e7648: f64 = (locals.var_tratio).powf(p.p381);
        let assign13340_e7649: f64 = (locals.var_uc_depmue0 / assign13340_e7648);
        (assign13340_e7649, (((locals.var_uc_depmue0_dn0 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn0)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)), (((locals.var_uc_depmue0_dn2 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn2)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)), (((locals.var_uc_depmue0_dn4 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)), (((locals.var_uc_depmue0_dn5 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn5)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)), (((locals.var_uc_depmue0_dn6 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn6)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)), (((locals.var_uc_depmue0_dn7 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn7)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)), (((locals.var_uc_depmue0_dn8 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn8)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)), (((locals.var_uc_depmue0_dn9 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn9)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)), (((locals.var_uc_depmue0_dn10 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn10)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)), (((locals.var_uc_depmue0_dn11 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn11)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn11 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)), (((locals.var_uc_depmue0_dn14 * assign13340_e7648) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn14)) } } else { (assign13340_e7648 * (p.p381 * (locals.var_tratio_dn14 / locals.var_tratio))) })) / (assign13340_e7648 * assign13340_e7648)),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign13340_e7651;
        locals.var_uc_depmue0_dn0 = assign13340_e7651_d_n0;
        locals.var_uc_depmue0_dn2 = assign13340_e7651_d_n2;
        locals.var_uc_depmue0_dn4 = assign13340_e7651_d_n4;
        locals.var_uc_depmue0_dn5 = assign13340_e7651_d_n5;
        locals.var_uc_depmue0_dn6 = assign13340_e7651_d_n6;
        locals.var_uc_depmue0_dn7 = assign13340_e7651_d_n7;
        locals.var_uc_depmue0_dn8 = assign13340_e7651_d_n8;
        locals.var_uc_depmue0_dn9 = assign13340_e7651_d_n9;
        locals.var_uc_depmue0_dn10 = assign13340_e7651_d_n10;
        locals.var_uc_depmue0_dn11 = assign13340_e7651_d_n11;
        locals.var_uc_depmue0_dn14 = assign13340_e7651_d_n14;

        let (assign13350_e7661, assign13350_e7661_d_n0, assign13350_e7661_d_n2, assign13350_e7661_d_n4, assign13350_e7661_d_n5, assign13350_e7661_d_n6, assign13350_e7661_d_n7, assign13350_e7661_d_n8, assign13350_e7661_d_n9, assign13350_e7661_d_n10, assign13350_e7661_d_n11, assign13350_e7661_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard292 != 0.0)) {
        let assign13350_e7658: f64 = (locals.var_tratio).powf(p.p382);
        let assign13350_e7659: f64 = (locals.var_uc_depmue2 / assign13350_e7658);
        (assign13350_e7659, (((locals.var_uc_depmue2_dn0 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn0)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)), (((locals.var_uc_depmue2_dn2 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn2)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)), (((locals.var_uc_depmue2_dn4 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)), (((locals.var_uc_depmue2_dn5 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn5)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)), (((locals.var_uc_depmue2_dn6 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn6)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)), (((locals.var_uc_depmue2_dn7 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn7)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)), (((locals.var_uc_depmue2_dn8 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn8)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)), (((locals.var_uc_depmue2_dn9 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn9)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)), (((locals.var_uc_depmue2_dn10 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn10)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)), (((locals.var_uc_depmue2_dn11 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn11)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn11 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)), (((locals.var_uc_depmue2_dn14 * assign13350_e7658) - (locals.var_uc_depmue2 * if 0.0 == 0.0 && ((p.p382) as f64).is_finite() && ((p.p382) as f64).fract() == 0.0 { if p.p382 == 0.0 { 0.0 } else { (p.p382 * ((locals.var_tratio).powf(p.p382 - 1.0) * locals.var_tratio_dn14)) } } else { (assign13350_e7658 * (p.p382 * (locals.var_tratio_dn14 / locals.var_tratio))) })) / (assign13350_e7658 * assign13350_e7658)),)
    } else {
        (locals.var_uc_depmue2, locals.var_uc_depmue2_dn0, locals.var_uc_depmue2_dn2, locals.var_uc_depmue2_dn4, locals.var_uc_depmue2_dn5, locals.var_uc_depmue2_dn6, locals.var_uc_depmue2_dn7, locals.var_uc_depmue2_dn8, locals.var_uc_depmue2_dn9, locals.var_uc_depmue2_dn10, locals.var_uc_depmue2_dn11, locals.var_uc_depmue2_dn14,)
    }
};
        locals.var_uc_depmue2 = assign13350_e7661;
        locals.var_uc_depmue2_dn0 = assign13350_e7661_d_n0;
        locals.var_uc_depmue2_dn2 = assign13350_e7661_d_n2;
        locals.var_uc_depmue2_dn4 = assign13350_e7661_d_n4;
        locals.var_uc_depmue2_dn5 = assign13350_e7661_d_n5;
        locals.var_uc_depmue2_dn6 = assign13350_e7661_d_n6;
        locals.var_uc_depmue2_dn7 = assign13350_e7661_d_n7;
        locals.var_uc_depmue2_dn8 = assign13350_e7661_d_n8;
        locals.var_uc_depmue2_dn9 = assign13350_e7661_d_n9;
        locals.var_uc_depmue2_dn10 = assign13350_e7661_d_n10;
        locals.var_uc_depmue2_dn11 = assign13350_e7661_d_n11;
        locals.var_uc_depmue2_dn14 = assign13350_e7661_d_n14;

        let assign13360_e7664: f64 = if locals.var_uc_codep == 3.0 { 1.0 } else { 0.0 };
        locals.var_guard295 = assign13360_e7664;

        let (assign13370_e7682, assign13370_e7682_d_n0, assign13370_e7682_d_n2, assign13370_e7682_d_n4, assign13370_e7682_d_n5, assign13370_e7682_d_n6, assign13370_e7682_d_n7, assign13370_e7682_d_n8, assign13370_e7682_d_n9, assign13370_e7682_d_n10, assign13370_e7682_d_n11, assign13370_e7682_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13370_e7673: f64 = (2.0 * 1.034943e-10);
        let assign13370_e7675: f64 = (assign13370_e7673 * 1.6021918e-19);
        let assign13370_e7677: f64 = (assign13370_e7675 * locals.var_uc_ndepm);
        let assign13370_e7679: f64 = (assign13370_e7677 * locals.var_beta_inv);
        let assign13370_e7680: f64 = (assign13370_e7679).sqrt();
        (assign13370_e7680, ((((assign13370_e7675 * locals.var_uc_ndepm_dn0) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn0)) / (2.0 * assign13370_e7680)), ((((assign13370_e7675 * locals.var_uc_ndepm_dn2) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn2)) / (2.0 * assign13370_e7680)), ((((assign13370_e7675 * locals.var_uc_ndepm_dn4) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn4)) / (2.0 * assign13370_e7680)), ((((assign13370_e7675 * locals.var_uc_ndepm_dn5) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn5)) / (2.0 * assign13370_e7680)), ((((assign13370_e7675 * locals.var_uc_ndepm_dn6) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn6)) / (2.0 * assign13370_e7680)), ((((assign13370_e7675 * locals.var_uc_ndepm_dn7) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn7)) / (2.0 * assign13370_e7680)), ((((assign13370_e7675 * locals.var_uc_ndepm_dn8) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn8)) / (2.0 * assign13370_e7680)), ((((assign13370_e7675 * locals.var_uc_ndepm_dn9) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn9)) / (2.0 * assign13370_e7680)), ((((assign13370_e7675 * locals.var_uc_ndepm_dn10) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn10)) / (2.0 * assign13370_e7680)), ((((assign13370_e7675 * locals.var_uc_ndepm_dn11) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn11)) / (2.0 * assign13370_e7680)), ((((assign13370_e7675 * locals.var_uc_ndepm_dn14) * locals.var_beta_inv) + (assign13370_e7677 * locals.var_beta_inv_dn14)) / (2.0 * assign13370_e7680)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn11, locals.var_cnst0_dn14,)
    }
};
        locals.var_cnst0 = assign13370_e7682;
        locals.var_cnst0_dn0 = assign13370_e7682_d_n0;
        locals.var_cnst0_dn2 = assign13370_e7682_d_n2;
        locals.var_cnst0_dn4 = assign13370_e7682_d_n4;
        locals.var_cnst0_dn5 = assign13370_e7682_d_n5;
        locals.var_cnst0_dn6 = assign13370_e7682_d_n6;
        locals.var_cnst0_dn7 = assign13370_e7682_d_n7;
        locals.var_cnst0_dn8 = assign13370_e7682_d_n8;
        locals.var_cnst0_dn9 = assign13370_e7682_d_n9;
        locals.var_cnst0_dn10 = assign13370_e7682_d_n10;
        locals.var_cnst0_dn11 = assign13370_e7682_d_n11;
        locals.var_cnst0_dn14 = assign13370_e7682_d_n14;

    }

    pub(super) fn stamp_transient_block_23(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13380_e7697, assign13380_e7697_d_n0, assign13380_e7697_d_n2, assign13380_e7697_d_n4, assign13380_e7697_d_n5, assign13380_e7697_d_n6, assign13380_e7697_d_n7, assign13380_e7697_d_n8, assign13380_e7697_d_n9, assign13380_e7697_d_n10, assign13380_e7697_d_n11, assign13380_e7697_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13380_e7691: f64 = (locals.var_nin * locals.var_nin);
        let __rspice_inv_cse_0: f64 = 1.0 / locals.var_uc_ndepm;
        let assign13380_e7693: f64 = (assign13380_e7691 * __rspice_inv_cse_0);
        let assign13380_e7695: f64 = (assign13380_e7693 * __rspice_inv_cse_0);
        (assign13380_e7695, ((((((((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn0)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn2)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn4)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn5)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn6)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn7)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn8)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn9)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn10)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn11 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn11)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn11)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn11)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)), ((((((((locals.var_nin_dn14 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn14)) * locals.var_uc_ndepm) - (assign13380_e7691 * locals.var_uc_ndepm_dn14)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)) * locals.var_uc_ndepm) - (assign13380_e7693 * locals.var_uc_ndepm_dn14)) / (locals.var_uc_ndepm * locals.var_uc_ndepm)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn11, locals.var_cnst1_dn14,)
    }
};
        locals.var_cnst1 = assign13380_e7697;
        locals.var_cnst1_dn0 = assign13380_e7697_d_n0;
        locals.var_cnst1_dn2 = assign13380_e7697_d_n2;
        locals.var_cnst1_dn4 = assign13380_e7697_d_n4;
        locals.var_cnst1_dn5 = assign13380_e7697_d_n5;
        locals.var_cnst1_dn6 = assign13380_e7697_d_n6;
        locals.var_cnst1_dn7 = assign13380_e7697_d_n7;
        locals.var_cnst1_dn8 = assign13380_e7697_d_n8;
        locals.var_cnst1_dn9 = assign13380_e7697_d_n9;
        locals.var_cnst1_dn10 = assign13380_e7697_d_n10;
        locals.var_cnst1_dn11 = assign13380_e7697_d_n11;
        locals.var_cnst1_dn14 = assign13380_e7697_d_n14;

        let (assign13390_e7713, assign13390_e7713_d_n0, assign13390_e7713_d_n2, assign13390_e7713_d_n4, assign13390_e7713_d_n5, assign13390_e7713_d_n6, assign13390_e7713_d_n7, assign13390_e7713_d_n8, assign13390_e7713_d_n9, assign13390_e7713_d_n10, assign13390_e7713_d_n11, assign13390_e7713_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13390_e7706: f64 = (2.0 * locals.var_beta_inv);
        let assign13390_e7709: f64 = (locals.var_uc_ndepm / locals.var_nin);
        let assign13390_e7710: f64 = (assign13390_e7709).ln();
        let assign13390_e7711: f64 = (assign13390_e7706 * assign13390_e7710);
        (assign13390_e7711, (((2.0 * locals.var_beta_inv_dn0) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn0 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))), (((2.0 * locals.var_beta_inv_dn2) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn2 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))), (((2.0 * locals.var_beta_inv_dn4) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn4 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))), (((2.0 * locals.var_beta_inv_dn5) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn5 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))), (((2.0 * locals.var_beta_inv_dn6) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn6 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))), (((2.0 * locals.var_beta_inv_dn7) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn7 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))), (((2.0 * locals.var_beta_inv_dn8) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn8 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))), (((2.0 * locals.var_beta_inv_dn9) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn9 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))), (((2.0 * locals.var_beta_inv_dn10) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn10 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))), (((2.0 * locals.var_beta_inv_dn11) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn11 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))), (((2.0 * locals.var_beta_inv_dn14) * assign13390_e7710) + (assign13390_e7706 * ((((locals.var_uc_ndepm_dn14 * locals.var_nin) - (locals.var_uc_ndepm * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign13390_e7709))),)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn11, locals.var_pb2n_dn14,)
    }
};
        locals.var_pb2n = assign13390_e7713;
        locals.var_pb2n_dn0 = assign13390_e7713_d_n0;
        locals.var_pb2n_dn2 = assign13390_e7713_d_n2;
        locals.var_pb2n_dn4 = assign13390_e7713_d_n4;
        locals.var_pb2n_dn5 = assign13390_e7713_d_n5;
        locals.var_pb2n_dn6 = assign13390_e7713_d_n6;
        locals.var_pb2n_dn7 = assign13390_e7713_d_n7;
        locals.var_pb2n_dn8 = assign13390_e7713_d_n8;
        locals.var_pb2n_dn9 = assign13390_e7713_d_n9;
        locals.var_pb2n_dn10 = assign13390_e7713_d_n10;
        locals.var_pb2n_dn11 = assign13390_e7713_d_n11;
        locals.var_pb2n_dn14 = assign13390_e7713_d_n14;

        let (assign13400_e7731, assign13400_e7731_d_n0, assign13400_e7731_d_n2, assign13400_e7731_d_n4, assign13400_e7731_d_n5, assign13400_e7731_d_n6, assign13400_e7731_d_n7, assign13400_e7731_d_n8, assign13400_e7731_d_n9, assign13400_e7731_d_n10, assign13400_e7731_d_n11, assign13400_e7731_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13400_e7723: f64 = (locals.var_uc_ndepm * locals.var_ef_nsubc);
        let __rspice_inv_cse_1: f64 = 1.0 / locals.var_nin;
        let assign13400_e7725: f64 = (assign13400_e7723 * __rspice_inv_cse_1);
        let assign13400_e7727: f64 = (assign13400_e7725 * __rspice_inv_cse_1);
        let assign13400_e7728: f64 = (assign13400_e7727).ln();
        let assign13400_e7729: f64 = (locals.var_beta_inv * assign13400_e7728);
        (assign13400_e7729, ((locals.var_beta_inv_dn0 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn0 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn0)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))), ((locals.var_beta_inv_dn2 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn2 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn2)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))), ((locals.var_beta_inv_dn4 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn4 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn4)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))), ((locals.var_beta_inv_dn5 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn5 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn5)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))), ((locals.var_beta_inv_dn6 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn6 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn6)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))), ((locals.var_beta_inv_dn7 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn7 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn7)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))), ((locals.var_beta_inv_dn8 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn8 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn8)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))), ((locals.var_beta_inv_dn9 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn9 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn9)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))), ((locals.var_beta_inv_dn10 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn10 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn10)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))), ((locals.var_beta_inv_dn11 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn11 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn11)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))), ((locals.var_beta_inv_dn14 * assign13400_e7728) + (locals.var_beta_inv * (((((((((locals.var_uc_ndepm_dn14 * locals.var_ef_nsubc) + (locals.var_uc_ndepm * locals.var_ef_nsubc_dn14)) * locals.var_nin) - (assign13400_e7723 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) * locals.var_nin) - (assign13400_e7725 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign13400_e7727))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn14,)
    }
};
        locals.var_vbipn = assign13400_e7731;
        locals.var_vbipn_dn0 = assign13400_e7731_d_n0;
        locals.var_vbipn_dn2 = assign13400_e7731_d_n2;
        locals.var_vbipn_dn4 = assign13400_e7731_d_n4;
        locals.var_vbipn_dn5 = assign13400_e7731_d_n5;
        locals.var_vbipn_dn6 = assign13400_e7731_d_n6;
        locals.var_vbipn_dn7 = assign13400_e7731_d_n7;
        locals.var_vbipn_dn8 = assign13400_e7731_d_n8;
        locals.var_vbipn_dn9 = assign13400_e7731_d_n9;
        locals.var_vbipn_dn10 = assign13400_e7731_d_n10;
        locals.var_vbipn_dn11 = assign13400_e7731_d_n11;
        locals.var_vbipn_dn14 = assign13400_e7731_d_n14;

        let (assign13410_e7743, assign13410_e7743_d_n0, assign13410_e7743_d_n2, assign13410_e7743_d_n4, assign13410_e7743_d_n5, assign13410_e7743_d_n6, assign13410_e7743_d_n7, assign13410_e7743_d_n8, assign13410_e7743_d_n9, assign13410_e7743_d_n10, assign13410_e7743_d_n11, assign13410_e7743_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13410_e7740: f64 = (locals.var_log_tratio * p.p380);
        let assign13410_e7741: f64 = (assign13410_e7740).exp();
        (assign13410_e7741, (assign13410_e7741 * (locals.var_log_tratio_dn0 * p.p380)), (assign13410_e7741 * (locals.var_log_tratio_dn2 * p.p380)), (assign13410_e7741 * (locals.var_log_tratio_dn4 * p.p380)), (assign13410_e7741 * (locals.var_log_tratio_dn5 * p.p380)), (assign13410_e7741 * (locals.var_log_tratio_dn6 * p.p380)), (assign13410_e7741 * (locals.var_log_tratio_dn7 * p.p380)), (assign13410_e7741 * (locals.var_log_tratio_dn8 * p.p380)), (assign13410_e7741 * (locals.var_log_tratio_dn9 * p.p380)), (assign13410_e7741 * (locals.var_log_tratio_dn10 * p.p380)), (assign13410_e7741 * (locals.var_log_tratio_dn11 * p.p380)), (assign13410_e7741 * (locals.var_log_tratio_dn14 * p.p380)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign13410_e7743;
        locals.var_t1_dn0 = assign13410_e7743_d_n0;
        locals.var_t1_dn2 = assign13410_e7743_d_n2;
        locals.var_t1_dn4 = assign13410_e7743_d_n4;
        locals.var_t1_dn5 = assign13410_e7743_d_n5;
        locals.var_t1_dn6 = assign13410_e7743_d_n6;
        locals.var_t1_dn7 = assign13410_e7743_d_n7;
        locals.var_t1_dn8 = assign13410_e7743_d_n8;
        locals.var_t1_dn9 = assign13410_e7743_d_n9;
        locals.var_t1_dn10 = assign13410_e7743_d_n10;
        locals.var_t1_dn11 = assign13410_e7743_d_n11;
        locals.var_t1_dn14 = assign13410_e7743_d_n14;

        let (assign13420_e7754, assign13420_e7754_d_n0, assign13420_e7754_d_n2, assign13420_e7754_d_n4, assign13420_e7754_d_n5, assign13420_e7754_d_n6, assign13420_e7754_d_n7, assign13420_e7754_d_n8, assign13420_e7754_d_n9, assign13420_e7754_d_n10, assign13420_e7754_d_n11, assign13420_e7754_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13420_e7752: f64 = (locals.var_t1 / locals.var_uc_depmueph1);
        (assign13420_e7752, (locals.var_t1_dn0 / locals.var_uc_depmueph1), (locals.var_t1_dn2 / locals.var_uc_depmueph1), (locals.var_t1_dn4 / locals.var_uc_depmueph1), (locals.var_t1_dn5 / locals.var_uc_depmueph1), (locals.var_t1_dn6 / locals.var_uc_depmueph1), (locals.var_t1_dn7 / locals.var_uc_depmueph1), (locals.var_t1_dn8 / locals.var_uc_depmueph1), (locals.var_t1_dn9 / locals.var_uc_depmueph1), (locals.var_t1_dn10 / locals.var_uc_depmueph1), (locals.var_t1_dn11 / locals.var_uc_depmueph1), (locals.var_t1_dn14 / locals.var_uc_depmueph1),)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn11, locals.var_depmphn0_dn14,)
    }
};
        locals.var_depmphn0 = assign13420_e7754;
        locals.var_depmphn0_dn0 = assign13420_e7754_d_n0;
        locals.var_depmphn0_dn2 = assign13420_e7754_d_n2;
        locals.var_depmphn0_dn4 = assign13420_e7754_d_n4;
        locals.var_depmphn0_dn5 = assign13420_e7754_d_n5;
        locals.var_depmphn0_dn6 = assign13420_e7754_d_n6;
        locals.var_depmphn0_dn7 = assign13420_e7754_d_n7;
        locals.var_depmphn0_dn8 = assign13420_e7754_d_n8;
        locals.var_depmphn0_dn9 = assign13420_e7754_d_n9;
        locals.var_depmphn0_dn10 = assign13420_e7754_d_n10;
        locals.var_depmphn0_dn11 = assign13420_e7754_d_n11;
        locals.var_depmphn0_dn14 = assign13420_e7754_d_n14;

        let (assign13430_e7779, assign13430_e7779_d_n0, assign13430_e7779_d_n2, assign13430_e7779_d_n4, assign13430_e7779_d_n5, assign13430_e7779_d_n6, assign13430_e7779_d_n7, assign13430_e7779_d_n8, assign13430_e7779_d_n9, assign13430_e7779_d_n10, assign13430_e7779_d_n11, assign13430_e7779_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13430_e7764: f64 = (0.4 * locals.var_tratio);
        let assign13430_e7765: f64 = (1.8 + assign13430_e7764);
        let assign13430_e7768: f64 = (0.1 * locals.var_tratio);
        let assign13430_e7770: f64 = (assign13430_e7768 * locals.var_tratio);
        let assign13430_e7771: f64 = (assign13430_e7765 + assign13430_e7770);
        let assign13430_e7775: f64 = (1.0 - locals.var_tratio);
        let assign13430_e7776: f64 = (p.p379 * assign13430_e7775);
        let assign13430_e7777: f64 = (assign13430_e7771 - assign13430_e7776);
        (assign13430_e7777, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn0))) - (p.p379 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn2))) - (p.p379 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn4))) - (p.p379 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn5))) - (p.p379 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn6))) - (p.p379 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn7))) - (p.p379 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn8))) - (p.p379 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn9))) - (p.p379 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn10))) - (p.p379 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn11) + (((0.1 * locals.var_tratio_dn11) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn11))) - (p.p379 * (-locals.var_tratio_dn11))), (((0.4 * locals.var_tratio_dn14) + (((0.1 * locals.var_tratio_dn14) * locals.var_tratio) + (assign13430_e7768 * locals.var_tratio_dn14))) - (p.p379 * (-locals.var_tratio_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign13430_e7779;
        locals.var_t0_dn0 = assign13430_e7779_d_n0;
        locals.var_t0_dn2 = assign13430_e7779_d_n2;
        locals.var_t0_dn4 = assign13430_e7779_d_n4;
        locals.var_t0_dn5 = assign13430_e7779_d_n5;
        locals.var_t0_dn6 = assign13430_e7779_d_n6;
        locals.var_t0_dn7 = assign13430_e7779_d_n7;
        locals.var_t0_dn8 = assign13430_e7779_d_n8;
        locals.var_t0_dn9 = assign13430_e7779_d_n9;
        locals.var_t0_dn10 = assign13430_e7779_d_n10;
        locals.var_t0_dn11 = assign13430_e7779_d_n11;
        locals.var_t0_dn14 = assign13430_e7779_d_n14;

        let (assign13440_e7790, assign13440_e7790_d_n0, assign13440_e7790_d_n2, assign13440_e7790_d_n4, assign13440_e7790_d_n5, assign13440_e7790_d_n6, assign13440_e7790_d_n7, assign13440_e7790_d_n8, assign13440_e7790_d_n9, assign13440_e7790_d_n10, assign13440_e7790_d_n11, assign13440_e7790_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13440_e7788: f64 = (locals.var_uc_depvmax / locals.var_t0);
        (assign13440_e7788, (((locals.var_uc_depvmax_dn0 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn0)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn2 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn2)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn4 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn4)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn5 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn5)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn6 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn6)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn7 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn7)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn8 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn8)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn9 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn9)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn10 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn10)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn11 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn11)) / (locals.var_t0 * locals.var_t0)), (((locals.var_uc_depvmax_dn14 * locals.var_t0) - (locals.var_uc_depvmax * locals.var_t0_dn14)) / (locals.var_t0 * locals.var_t0)),)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign13440_e7790;
        locals.var_uc_depvmax_dn0 = assign13440_e7790_d_n0;
        locals.var_uc_depvmax_dn2 = assign13440_e7790_d_n2;
        locals.var_uc_depvmax_dn4 = assign13440_e7790_d_n4;
        locals.var_uc_depvmax_dn5 = assign13440_e7790_d_n5;
        locals.var_uc_depvmax_dn6 = assign13440_e7790_d_n6;
        locals.var_uc_depvmax_dn7 = assign13440_e7790_d_n7;
        locals.var_uc_depvmax_dn8 = assign13440_e7790_d_n8;
        locals.var_uc_depvmax_dn9 = assign13440_e7790_d_n9;
        locals.var_uc_depvmax_dn10 = assign13440_e7790_d_n10;
        locals.var_uc_depvmax_dn11 = assign13440_e7790_d_n11;
        locals.var_uc_depvmax_dn14 = assign13440_e7790_d_n14;

        let assign13460_e7798: f64 = if locals.var_uc_depvmax < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard297 = assign13460_e7798;

        let (assign13470_e7809, assign13470_e7809_d_n0, assign13470_e7809_d_n2, assign13470_e7809_d_n4, assign13470_e7809_d_n5, assign13470_e7809_d_n6, assign13470_e7809_d_n7, assign13470_e7809_d_n8, assign13470_e7809_d_n9, assign13470_e7809_d_n10, assign13470_e7809_d_n11, assign13470_e7809_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) && (locals.var_guard297 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_depvmax, locals.var_uc_depvmax_dn0, locals.var_uc_depvmax_dn2, locals.var_uc_depvmax_dn4, locals.var_uc_depvmax_dn5, locals.var_uc_depvmax_dn6, locals.var_uc_depvmax_dn7, locals.var_uc_depvmax_dn8, locals.var_uc_depvmax_dn9, locals.var_uc_depvmax_dn10, locals.var_uc_depvmax_dn11, locals.var_uc_depvmax_dn14,)
    }
};
        locals.var_uc_depvmax = assign13470_e7809;
        locals.var_uc_depvmax_dn0 = assign13470_e7809_d_n0;
        locals.var_uc_depvmax_dn2 = assign13470_e7809_d_n2;
        locals.var_uc_depvmax_dn4 = assign13470_e7809_d_n4;
        locals.var_uc_depvmax_dn5 = assign13470_e7809_d_n5;
        locals.var_uc_depvmax_dn6 = assign13470_e7809_d_n6;
        locals.var_uc_depvmax_dn7 = assign13470_e7809_d_n7;
        locals.var_uc_depvmax_dn8 = assign13470_e7809_d_n8;
        locals.var_uc_depvmax_dn9 = assign13470_e7809_d_n9;
        locals.var_uc_depvmax_dn10 = assign13470_e7809_d_n10;
        locals.var_uc_depvmax_dn11 = assign13470_e7809_d_n11;
        locals.var_uc_depvmax_dn14 = assign13470_e7809_d_n14;

        let (assign13480_e7822, assign13480_e7822_d_n0, assign13480_e7822_d_n2, assign13480_e7822_d_n4, assign13480_e7822_d_n5, assign13480_e7822_d_n6, assign13480_e7822_d_n7, assign13480_e7822_d_n8, assign13480_e7822_d_n9, assign13480_e7822_d_n10, assign13480_e7822_d_n11, assign13480_e7822_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13480_e7819: f64 = (locals.var_tratio).powf(p.p381);
        let assign13480_e7820: f64 = (locals.var_uc_depmue0 / assign13480_e7819);
        (assign13480_e7820, (((locals.var_uc_depmue0_dn0 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn0)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn0 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn2 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn2)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn2 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn4 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn4)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn4 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn5 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn5)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn5 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn6 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn6)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn6 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn7 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn7)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn7 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn8 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn8)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn8 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn9 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn9)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn9 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn10 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn10)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn10 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn11 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn11)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn11 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)), (((locals.var_uc_depmue0_dn14 * assign13480_e7819) - (locals.var_uc_depmue0 * if 0.0 == 0.0 && ((p.p381) as f64).is_finite() && ((p.p381) as f64).fract() == 0.0 { if p.p381 == 0.0 { 0.0 } else { (p.p381 * ((locals.var_tratio).powf(p.p381 - 1.0) * locals.var_tratio_dn14)) } } else { (assign13480_e7819 * (p.p381 * (locals.var_tratio_dn14 / locals.var_tratio))) })) / (assign13480_e7819 * assign13480_e7819)),)
    } else {
        (locals.var_uc_depmue0, locals.var_uc_depmue0_dn0, locals.var_uc_depmue0_dn2, locals.var_uc_depmue0_dn4, locals.var_uc_depmue0_dn5, locals.var_uc_depmue0_dn6, locals.var_uc_depmue0_dn7, locals.var_uc_depmue0_dn8, locals.var_uc_depmue0_dn9, locals.var_uc_depmue0_dn10, locals.var_uc_depmue0_dn11, locals.var_uc_depmue0_dn14,)
    }
};
        locals.var_uc_depmue0 = assign13480_e7822;
        locals.var_uc_depmue0_dn0 = assign13480_e7822_d_n0;
        locals.var_uc_depmue0_dn2 = assign13480_e7822_d_n2;
        locals.var_uc_depmue0_dn4 = assign13480_e7822_d_n4;
        locals.var_uc_depmue0_dn5 = assign13480_e7822_d_n5;
        locals.var_uc_depmue0_dn6 = assign13480_e7822_d_n6;
        locals.var_uc_depmue0_dn7 = assign13480_e7822_d_n7;
        locals.var_uc_depmue0_dn8 = assign13480_e7822_d_n8;
        locals.var_uc_depmue0_dn9 = assign13480_e7822_d_n9;
        locals.var_uc_depmue0_dn10 = assign13480_e7822_d_n10;
        locals.var_uc_depmue0_dn11 = assign13480_e7822_d_n11;
        locals.var_uc_depmue0_dn14 = assign13480_e7822_d_n14;

        let (assign13490_e7837, assign13490_e7837_d_n0, assign13490_e7837_d_n2, assign13490_e7837_d_n4, assign13490_e7837_d_n5, assign13490_e7837_d_n6, assign13490_e7837_d_n7, assign13490_e7837_d_n8, assign13490_e7837_d_n9, assign13490_e7837_d_n10, assign13490_e7837_d_n11, assign13490_e7837_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 != 0.0)) {
        let assign13490_e7833: f64 = (locals.var_tratio - 1.0);
        let assign13490_e7834: f64 = (p.p365 * assign13490_e7833);
        let assign13490_e7835: f64 = (p.p364 + assign13490_e7834);
        (assign13490_e7835, (p.p365 * locals.var_tratio_dn0), (p.p365 * locals.var_tratio_dn2), (p.p365 * locals.var_tratio_dn4), (p.p365 * locals.var_tratio_dn5), (p.p365 * locals.var_tratio_dn6), (p.p365 * locals.var_tratio_dn7), (p.p365 * locals.var_tratio_dn8), (p.p365 * locals.var_tratio_dn9), (p.p365 * locals.var_tratio_dn10), (p.p365 * locals.var_tratio_dn11), (p.p365 * locals.var_tratio_dn14),)
    } else {
        (locals.var_uc_depwlp, locals.var_uc_depwlp_dn0, locals.var_uc_depwlp_dn2, locals.var_uc_depwlp_dn4, locals.var_uc_depwlp_dn5, locals.var_uc_depwlp_dn6, locals.var_uc_depwlp_dn7, locals.var_uc_depwlp_dn8, locals.var_uc_depwlp_dn9, locals.var_uc_depwlp_dn10, locals.var_uc_depwlp_dn11, locals.var_uc_depwlp_dn14,)
    }
};
        locals.var_uc_depwlp = assign13490_e7837;
        locals.var_uc_depwlp_dn0 = assign13490_e7837_d_n0;
        locals.var_uc_depwlp_dn2 = assign13490_e7837_d_n2;
        locals.var_uc_depwlp_dn4 = assign13490_e7837_d_n4;
        locals.var_uc_depwlp_dn5 = assign13490_e7837_d_n5;
        locals.var_uc_depwlp_dn6 = assign13490_e7837_d_n6;
        locals.var_uc_depwlp_dn7 = assign13490_e7837_d_n7;
        locals.var_uc_depwlp_dn8 = assign13490_e7837_d_n8;
        locals.var_uc_depwlp_dn9 = assign13490_e7837_d_n9;
        locals.var_uc_depwlp_dn10 = assign13490_e7837_d_n10;
        locals.var_uc_depwlp_dn11 = assign13490_e7837_d_n11;
        locals.var_uc_depwlp_dn14 = assign13490_e7837_d_n14;

        let (assign13500_e7847, assign13500_e7847_d_n0, assign13500_e7847_d_n2, assign13500_e7847_d_n4, assign13500_e7847_d_n5, assign13500_e7847_d_n6, assign13500_e7847_d_n7, assign13500_e7847_d_n8, assign13500_e7847_d_n9, assign13500_e7847_d_n10, assign13500_e7847_d_n11, assign13500_e7847_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_pb2n, locals.var_pb2n_dn0, locals.var_pb2n_dn2, locals.var_pb2n_dn4, locals.var_pb2n_dn5, locals.var_pb2n_dn6, locals.var_pb2n_dn7, locals.var_pb2n_dn8, locals.var_pb2n_dn9, locals.var_pb2n_dn10, locals.var_pb2n_dn11, locals.var_pb2n_dn14,)
    }
};
        locals.var_pb2n = assign13500_e7847;
        locals.var_pb2n_dn0 = assign13500_e7847_d_n0;
        locals.var_pb2n_dn2 = assign13500_e7847_d_n2;
        locals.var_pb2n_dn4 = assign13500_e7847_d_n4;
        locals.var_pb2n_dn5 = assign13500_e7847_d_n5;
        locals.var_pb2n_dn6 = assign13500_e7847_d_n6;
        locals.var_pb2n_dn7 = assign13500_e7847_d_n7;
        locals.var_pb2n_dn8 = assign13500_e7847_d_n8;
        locals.var_pb2n_dn9 = assign13500_e7847_d_n9;
        locals.var_pb2n_dn10 = assign13500_e7847_d_n10;
        locals.var_pb2n_dn11 = assign13500_e7847_d_n11;
        locals.var_pb2n_dn14 = assign13500_e7847_d_n14;

        let (assign13510_e7866, assign13510_e7866_d_n0, assign13510_e7866_d_n2, assign13510_e7866_d_n4, assign13510_e7866_d_n5, assign13510_e7866_d_n6, assign13510_e7866_d_n7, assign13510_e7866_d_n8, assign13510_e7866_d_n9, assign13510_e7866_d_n10, assign13510_e7866_d_n11, assign13510_e7866_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 == 0.0)) {
        let assign13510_e7858: f64 = (locals.var_uc_njunc / locals.var_nin);
        let assign13510_e7860: f64 = (assign13510_e7858 * locals.var_nsub);
        let assign13510_e7862: f64 = (assign13510_e7860 / locals.var_nin);
        let assign13510_e7863: f64 = (assign13510_e7862).ln();
        let assign13510_e7864: f64 = (locals.var_beta_inv * assign13510_e7863);
        (assign13510_e7864, ((locals.var_beta_inv_dn0 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn0) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn0)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn2 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn2) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn2)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn4 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn4) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn4)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn5 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn5) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn5)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn6 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn6) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn6)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn7 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn7) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn7)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn8 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn8) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn8)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn9 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn9) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn9)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn10 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn10) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn10)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn11 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn11) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn11)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))), ((locals.var_beta_inv_dn14 * assign13510_e7863) + (locals.var_beta_inv * (((((((-((locals.var_uc_njunc * locals.var_nin_dn14) / (locals.var_nin * locals.var_nin))) * locals.var_nsub) + (assign13510_e7858 * locals.var_nsub_dn14)) * locals.var_nin) - (assign13510_e7860 * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign13510_e7862))),)
    } else {
        (locals.var_vbipn, locals.var_vbipn_dn0, locals.var_vbipn_dn2, locals.var_vbipn_dn4, locals.var_vbipn_dn5, locals.var_vbipn_dn6, locals.var_vbipn_dn7, locals.var_vbipn_dn8, locals.var_vbipn_dn9, locals.var_vbipn_dn10, locals.var_vbipn_dn11, locals.var_vbipn_dn14,)
    }
};
        locals.var_vbipn = assign13510_e7866;
        locals.var_vbipn_dn0 = assign13510_e7866_d_n0;
        locals.var_vbipn_dn2 = assign13510_e7866_d_n2;
        locals.var_vbipn_dn4 = assign13510_e7866_d_n4;
        locals.var_vbipn_dn5 = assign13510_e7866_d_n5;
        locals.var_vbipn_dn6 = assign13510_e7866_d_n6;
        locals.var_vbipn_dn7 = assign13510_e7866_d_n7;
        locals.var_vbipn_dn8 = assign13510_e7866_d_n8;
        locals.var_vbipn_dn9 = assign13510_e7866_d_n9;
        locals.var_vbipn_dn10 = assign13510_e7866_d_n10;
        locals.var_vbipn_dn11 = assign13510_e7866_d_n11;
        locals.var_vbipn_dn14 = assign13510_e7866_d_n14;

        let (assign13520_e7876, assign13520_e7876_d_n0, assign13520_e7876_d_n2, assign13520_e7876_d_n4, assign13520_e7876_d_n5, assign13520_e7876_d_n6, assign13520_e7876_d_n7, assign13520_e7876_d_n8, assign13520_e7876_d_n9, assign13520_e7876_d_n10, assign13520_e7876_d_n11, assign13520_e7876_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard292 == 0.0)) && (locals.var_guard295 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_depmphn0, locals.var_depmphn0_dn0, locals.var_depmphn0_dn2, locals.var_depmphn0_dn4, locals.var_depmphn0_dn5, locals.var_depmphn0_dn6, locals.var_depmphn0_dn7, locals.var_depmphn0_dn8, locals.var_depmphn0_dn9, locals.var_depmphn0_dn10, locals.var_depmphn0_dn11, locals.var_depmphn0_dn14,)
    }
};
        locals.var_depmphn0 = assign13520_e7876;
        locals.var_depmphn0_dn0 = assign13520_e7876_d_n0;
        locals.var_depmphn0_dn2 = assign13520_e7876_d_n2;
        locals.var_depmphn0_dn4 = assign13520_e7876_d_n4;
        locals.var_depmphn0_dn5 = assign13520_e7876_d_n5;
        locals.var_depmphn0_dn6 = assign13520_e7876_d_n6;
        locals.var_depmphn0_dn7 = assign13520_e7876_d_n7;
        locals.var_depmphn0_dn8 = assign13520_e7876_d_n8;
        locals.var_depmphn0_dn9 = assign13520_e7876_d_n9;
        locals.var_depmphn0_dn10 = assign13520_e7876_d_n10;
        locals.var_depmphn0_dn11 = assign13520_e7876_d_n11;
        locals.var_depmphn0_dn14 = assign13520_e7876_d_n14;

        let (assign13530_e7882, assign13530_e7882_d_n0, assign13530_e7882_d_n2, assign13530_e7882_d_n4, assign13530_e7882_d_n5, assign13530_e7882_d_n6, assign13530_e7882_d_n7, assign13530_e7882_d_n8, assign13530_e7882_d_n9, assign13530_e7882_d_n10, assign13530_e7882_d_n11, assign13530_e7882_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13530_e7880: f64 = (locals.var_ptovr0 * locals.var_beta_inv);
        (assign13530_e7880, ((locals.var_ptovr0_dn0 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn0)), ((locals.var_ptovr0_dn2 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn2)), ((locals.var_ptovr0_dn4 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn4)), ((locals.var_ptovr0_dn5 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn5)), ((locals.var_ptovr0_dn6 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn6)), ((locals.var_ptovr0_dn7 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn7)), ((locals.var_ptovr0_dn8 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn8)), ((locals.var_ptovr0_dn9 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn9)), ((locals.var_ptovr0_dn10 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn10)), ((locals.var_ptovr0_dn11 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn11)), ((locals.var_ptovr0_dn14 * locals.var_beta_inv) + (locals.var_ptovr0 * locals.var_beta_inv_dn14)),)
    } else {
        (locals.var_ptovr, locals.var_ptovr_dn0, locals.var_ptovr_dn2, locals.var_ptovr_dn4, locals.var_ptovr_dn5, locals.var_ptovr_dn6, locals.var_ptovr_dn7, locals.var_ptovr_dn8, locals.var_ptovr_dn9, locals.var_ptovr_dn10, locals.var_ptovr_dn11, locals.var_ptovr_dn14,)
    }
};
        locals.var_ptovr = assign13530_e7882;
        locals.var_ptovr_dn0 = assign13530_e7882_d_n0;
        locals.var_ptovr_dn2 = assign13530_e7882_d_n2;
        locals.var_ptovr_dn4 = assign13530_e7882_d_n4;
        locals.var_ptovr_dn5 = assign13530_e7882_d_n5;
        locals.var_ptovr_dn6 = assign13530_e7882_d_n6;
        locals.var_ptovr_dn7 = assign13530_e7882_d_n7;
        locals.var_ptovr_dn8 = assign13530_e7882_d_n8;
        locals.var_ptovr_dn9 = assign13530_e7882_d_n9;
        locals.var_ptovr_dn10 = assign13530_e7882_d_n10;
        locals.var_ptovr_dn11 = assign13530_e7882_d_n11;
        locals.var_ptovr_dn14 = assign13530_e7882_d_n14;

        let (assign13540_e7888, assign13540_e7888_d_n0, assign13540_e7888_d_n2, assign13540_e7888_d_n4, assign13540_e7888_d_n5, assign13540_e7888_d_n6, assign13540_e7888_d_n7, assign13540_e7888_d_n8, assign13540_e7888_d_n9, assign13540_e7888_d_n10, assign13540_e7888_d_n11, assign13540_e7888_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13540_e7886: f64 = (locals.var_ttemp / locals.var_ktnom);
        (assign13540_e7886, (locals.var_ttemp_dn0 / locals.var_ktnom), (locals.var_ttemp_dn2 / locals.var_ktnom), (locals.var_ttemp_dn4 / locals.var_ktnom), (locals.var_ttemp_dn5 / locals.var_ktnom), (locals.var_ttemp_dn6 / locals.var_ktnom), (locals.var_ttemp_dn7 / locals.var_ktnom), (locals.var_ttemp_dn8 / locals.var_ktnom), (locals.var_ttemp_dn9 / locals.var_ktnom), (locals.var_ttemp_dn10 / locals.var_ktnom), (locals.var_ttemp_dn11 / locals.var_ktnom), (locals.var_ttemp_dn14 / locals.var_ktnom),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign13540_e7888;
        locals.var_t1_dn0 = assign13540_e7888_d_n0;
        locals.var_t1_dn2 = assign13540_e7888_d_n2;
        locals.var_t1_dn4 = assign13540_e7888_d_n4;
        locals.var_t1_dn5 = assign13540_e7888_d_n5;
        locals.var_t1_dn6 = assign13540_e7888_d_n6;
        locals.var_t1_dn7 = assign13540_e7888_d_n7;
        locals.var_t1_dn8 = assign13540_e7888_d_n8;
        locals.var_t1_dn9 = assign13540_e7888_d_n9;
        locals.var_t1_dn10 = assign13540_e7888_d_n10;
        locals.var_t1_dn11 = assign13540_e7888_d_n11;
        locals.var_t1_dn14 = assign13540_e7888_d_n14;

        let (assign13550_e7908, assign13550_e7908_d_n0, assign13550_e7908_d_n2, assign13550_e7908_d_n4, assign13550_e7908_d_n5, assign13550_e7908_d_n6, assign13550_e7908_d_n7, assign13550_e7908_d_n8, assign13550_e7908_d_n9, assign13550_e7908_d_n10, assign13550_e7908_d_n11, assign13550_e7908_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13550_e7893: f64 = (0.4 * locals.var_t1);
        let assign13550_e7894: f64 = (1.8 + assign13550_e7893);
        let assign13550_e7897: f64 = (0.1 * locals.var_t1);
        let assign13550_e7899: f64 = (assign13550_e7897 * locals.var_t1);
        let assign13550_e7900: f64 = (assign13550_e7894 + assign13550_e7899);
        let assign13550_e7904: f64 = (1.0 - locals.var_t1);
        let assign13550_e7905: f64 = (locals.var_uc_vtmp * assign13550_e7904);
        let assign13550_e7906: f64 = (assign13550_e7900 - assign13550_e7905);
        (assign13550_e7906, (((0.4 * locals.var_t1_dn0) + (((0.1 * locals.var_t1_dn0) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn0))) - (locals.var_uc_vtmp * (-locals.var_t1_dn0))), (((0.4 * locals.var_t1_dn2) + (((0.1 * locals.var_t1_dn2) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn2))) - (locals.var_uc_vtmp * (-locals.var_t1_dn2))), (((0.4 * locals.var_t1_dn4) + (((0.1 * locals.var_t1_dn4) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn4))) - (locals.var_uc_vtmp * (-locals.var_t1_dn4))), (((0.4 * locals.var_t1_dn5) + (((0.1 * locals.var_t1_dn5) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn5))) - (locals.var_uc_vtmp * (-locals.var_t1_dn5))), (((0.4 * locals.var_t1_dn6) + (((0.1 * locals.var_t1_dn6) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn6))) - (locals.var_uc_vtmp * (-locals.var_t1_dn6))), (((0.4 * locals.var_t1_dn7) + (((0.1 * locals.var_t1_dn7) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn7))) - (locals.var_uc_vtmp * (-locals.var_t1_dn7))), (((0.4 * locals.var_t1_dn8) + (((0.1 * locals.var_t1_dn8) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn8))) - (locals.var_uc_vtmp * (-locals.var_t1_dn8))), (((0.4 * locals.var_t1_dn9) + (((0.1 * locals.var_t1_dn9) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn9))) - (locals.var_uc_vtmp * (-locals.var_t1_dn9))), (((0.4 * locals.var_t1_dn10) + (((0.1 * locals.var_t1_dn10) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn10))) - (locals.var_uc_vtmp * (-locals.var_t1_dn10))), (((0.4 * locals.var_t1_dn11) + (((0.1 * locals.var_t1_dn11) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn11))) - (locals.var_uc_vtmp * (-locals.var_t1_dn11))), (((0.4 * locals.var_t1_dn14) + (((0.1 * locals.var_t1_dn14) * locals.var_t1) + (assign13550_e7897 * locals.var_t1_dn14))) - (locals.var_uc_vtmp * (-locals.var_t1_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign13550_e7908;
        locals.var_t0_dn0 = assign13550_e7908_d_n0;
        locals.var_t0_dn2 = assign13550_e7908_d_n2;
        locals.var_t0_dn4 = assign13550_e7908_d_n4;
        locals.var_t0_dn5 = assign13550_e7908_d_n5;
        locals.var_t0_dn6 = assign13550_e7908_d_n6;
        locals.var_t0_dn7 = assign13550_e7908_d_n7;
        locals.var_t0_dn8 = assign13550_e7908_d_n8;
        locals.var_t0_dn9 = assign13550_e7908_d_n9;
        locals.var_t0_dn10 = assign13550_e7908_d_n10;
        locals.var_t0_dn11 = assign13550_e7908_d_n11;
        locals.var_t0_dn14 = assign13550_e7908_d_n14;

        let assign13560_e7911: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard298 = assign13560_e7911;

        let (assign13570_e7931, assign13570_e7931_d_n0, assign13570_e7931_d_n2, assign13570_e7931_d_n4, assign13570_e7931_d_n5, assign13570_e7931_d_n6, assign13570_e7931_d_n7, assign13570_e7931_d_n8, assign13570_e7931_d_n9, assign13570_e7931_d_n10, assign13570_e7931_d_n11, assign13570_e7931_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard298 != 0.0)) {
        let assign13570_e7917: f64 = (locals.var_vmax0 * locals.var_uc_vmax);
        let assign13570_e7919: f64 = (assign13570_e7917 / locals.var_t0);
        let assign13570_e7923: f64 = (p.p90 * locals.var_tdiff0);
        let assign13570_e7924: f64 = (1.0 + assign13570_e7923);
        let assign13570_e7927: f64 = (p.p91 * locals.var_tdiff0_2);
        let assign13570_e7928: f64 = (assign13570_e7924 + assign13570_e7927);
        let assign13570_e7929: f64 = (assign13570_e7919 * assign13570_e7928);
        (assign13570_e7929, (((-((assign13570_e7917 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn0) + (p.p91 * locals.var_tdiff0_2_dn0)))), (((-((assign13570_e7917 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn2) + (p.p91 * locals.var_tdiff0_2_dn2)))), (((-((assign13570_e7917 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn4) + (p.p91 * locals.var_tdiff0_2_dn4)))), (((-((assign13570_e7917 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn5) + (p.p91 * locals.var_tdiff0_2_dn5)))), (((-((assign13570_e7917 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn6) + (p.p91 * locals.var_tdiff0_2_dn6)))), (((-((assign13570_e7917 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn7) + (p.p91 * locals.var_tdiff0_2_dn7)))), (((-((assign13570_e7917 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn8) + (p.p91 * locals.var_tdiff0_2_dn8)))), (((-((assign13570_e7917 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn9) + (p.p91 * locals.var_tdiff0_2_dn9)))), (((-((assign13570_e7917 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn10) + (p.p91 * locals.var_tdiff0_2_dn10)))), (((-((assign13570_e7917 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn11) + (p.p91 * locals.var_tdiff0_2_dn11)))), (((-((assign13570_e7917 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))) * assign13570_e7928) + (assign13570_e7919 * ((p.p90 * locals.var_tdiff0_dn14) + (p.p91 * locals.var_tdiff0_2_dn14)))),)
    } else {
        (locals.var_vmaxeff, locals.var_vmaxeff_dn0, locals.var_vmaxeff_dn2, locals.var_vmaxeff_dn4, locals.var_vmaxeff_dn5, locals.var_vmaxeff_dn6, locals.var_vmaxeff_dn7, locals.var_vmaxeff_dn8, locals.var_vmaxeff_dn9, locals.var_vmaxeff_dn10, locals.var_vmaxeff_dn11, locals.var_vmaxeff_dn14,)
    }
};
        locals.var_vmaxeff = assign13570_e7931;
        locals.var_vmaxeff_dn0 = assign13570_e7931_d_n0;
        locals.var_vmaxeff_dn2 = assign13570_e7931_d_n2;
        locals.var_vmaxeff_dn4 = assign13570_e7931_d_n4;
        locals.var_vmaxeff_dn5 = assign13570_e7931_d_n5;
        locals.var_vmaxeff_dn6 = assign13570_e7931_d_n6;
        locals.var_vmaxeff_dn7 = assign13570_e7931_d_n7;
        locals.var_vmaxeff_dn8 = assign13570_e7931_d_n8;
        locals.var_vmaxeff_dn9 = assign13570_e7931_d_n9;
        locals.var_vmaxeff_dn10 = assign13570_e7931_d_n10;
        locals.var_vmaxeff_dn11 = assign13570_e7931_d_n11;
        locals.var_vmaxeff_dn14 = assign13570_e7931_d_n14;

        let (assign13580_e7952, assign13580_e7952_d_n0, assign13580_e7952_d_n2, assign13580_e7952_d_n4, assign13580_e7952_d_n5, assign13580_e7952_d_n6, assign13580_e7952_d_n7, assign13580_e7952_d_n8, assign13580_e7952_d_n9, assign13580_e7952_d_n10, assign13580_e7952_d_n11, assign13580_e7952_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard298 == 0.0)) {
        let assign13580_e7938: f64 = (locals.var_vmax0 * locals.var_uc_vmax);
        let assign13580_e7940: f64 = (assign13580_e7938 / locals.var_t0);
        let assign13580_e7944: f64 = (p.p90 * locals.var_tdiff);
        let assign13580_e7945: f64 = (1.0 + assign13580_e7944);
        let assign13580_e7948: f64 = (p.p91 * locals.var_tdiff_2);
        let assign13580_e7949: f64 = (assign13580_e7945 + assign13580_e7948);
        let assign13580_e7950: f64 = (assign13580_e7940 * assign13580_e7949);
        (assign13580_e7950, (((-((assign13580_e7938 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn0) + (p.p91 * locals.var_tdiff_2_dn0)))), (((-((assign13580_e7938 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn2) + (p.p91 * locals.var_tdiff_2_dn2)))), (((-((assign13580_e7938 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn4) + (p.p91 * locals.var_tdiff_2_dn4)))), (((-((assign13580_e7938 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn5) + (p.p91 * locals.var_tdiff_2_dn5)))), (((-((assign13580_e7938 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn6) + (p.p91 * locals.var_tdiff_2_dn6)))), (((-((assign13580_e7938 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn7) + (p.p91 * locals.var_tdiff_2_dn7)))), (((-((assign13580_e7938 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn8) + (p.p91 * locals.var_tdiff_2_dn8)))), (((-((assign13580_e7938 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn9) + (p.p91 * locals.var_tdiff_2_dn9)))), (((-((assign13580_e7938 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn10) + (p.p91 * locals.var_tdiff_2_dn10)))), (((-((assign13580_e7938 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn11) + (p.p91 * locals.var_tdiff_2_dn11)))), (((-((assign13580_e7938 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))) * assign13580_e7949) + (assign13580_e7940 * ((p.p90 * locals.var_tdiff_dn14) + (p.p91 * locals.var_tdiff_2_dn14)))),)
    } else {
        (locals.var_vmaxeff, locals.var_vmaxeff_dn0, locals.var_vmaxeff_dn2, locals.var_vmaxeff_dn4, locals.var_vmaxeff_dn5, locals.var_vmaxeff_dn6, locals.var_vmaxeff_dn7, locals.var_vmaxeff_dn8, locals.var_vmaxeff_dn9, locals.var_vmaxeff_dn10, locals.var_vmaxeff_dn11, locals.var_vmaxeff_dn14,)
    }
};
        locals.var_vmaxeff = assign13580_e7952;
        locals.var_vmaxeff_dn0 = assign13580_e7952_d_n0;
        locals.var_vmaxeff_dn2 = assign13580_e7952_d_n2;
        locals.var_vmaxeff_dn4 = assign13580_e7952_d_n4;
        locals.var_vmaxeff_dn5 = assign13580_e7952_d_n5;
        locals.var_vmaxeff_dn6 = assign13580_e7952_d_n6;
        locals.var_vmaxeff_dn7 = assign13580_e7952_d_n7;
        locals.var_vmaxeff_dn8 = assign13580_e7952_d_n8;
        locals.var_vmaxeff_dn9 = assign13580_e7952_d_n9;
        locals.var_vmaxeff_dn10 = assign13580_e7952_d_n10;
        locals.var_vmaxeff_dn11 = assign13580_e7952_d_n11;
        locals.var_vmaxeff_dn14 = assign13580_e7952_d_n14;

        let assign13600_e7960: f64 = if p.p39 != 2.0 { 1.0 } else { 0.0 };
        locals.var_guard300 = assign13600_e7960;

        let (assign13610_e7976, assign13610_e7976_d_n0, assign13610_e7976_d_n2, assign13610_e7976_d_n4, assign13610_e7976_d_n5, assign13610_e7976_d_n6, assign13610_e7976_d_n7, assign13610_e7976_d_n8, assign13610_e7976_d_n9, assign13610_e7976_d_n10, assign13610_e7976_d_n11, assign13610_e7976_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 != 0.0)) {
        let assign13610_e7968: f64 = (p.p324 * locals.var_tdiff0);
        let assign13610_e7969: f64 = (1.0 + assign13610_e7968);
        let assign13610_e7972: f64 = (p.p325 * locals.var_tdiff0_2);
        let assign13610_e7973: f64 = (assign13610_e7969 + assign13610_e7972);
        let assign13610_e7974: f64 = (locals.var_ninvd0 * assign13610_e7973);
        (assign13610_e7974, (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn0) + (p.p325 * locals.var_tdiff0_2_dn0))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn2) + (p.p325 * locals.var_tdiff0_2_dn2))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn4) + (p.p325 * locals.var_tdiff0_2_dn4))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn5) + (p.p325 * locals.var_tdiff0_2_dn5))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn6) + (p.p325 * locals.var_tdiff0_2_dn6))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn7) + (p.p325 * locals.var_tdiff0_2_dn7))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn8) + (p.p325 * locals.var_tdiff0_2_dn8))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn9) + (p.p325 * locals.var_tdiff0_2_dn9))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn10) + (p.p325 * locals.var_tdiff0_2_dn10))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn11) + (p.p325 * locals.var_tdiff0_2_dn11))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff0_dn14) + (p.p325 * locals.var_tdiff0_2_dn14))),)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn11, locals.var_ninvde_dn14,)
    }
};
        locals.var_ninvde = assign13610_e7976;
        locals.var_ninvde_dn0 = assign13610_e7976_d_n0;
        locals.var_ninvde_dn2 = assign13610_e7976_d_n2;
        locals.var_ninvde_dn4 = assign13610_e7976_d_n4;
        locals.var_ninvde_dn5 = assign13610_e7976_d_n5;
        locals.var_ninvde_dn6 = assign13610_e7976_d_n6;
        locals.var_ninvde_dn7 = assign13610_e7976_d_n7;
        locals.var_ninvde_dn8 = assign13610_e7976_d_n8;
        locals.var_ninvde_dn9 = assign13610_e7976_d_n9;
        locals.var_ninvde_dn10 = assign13610_e7976_d_n10;
        locals.var_ninvde_dn11 = assign13610_e7976_d_n11;
        locals.var_ninvde_dn14 = assign13610_e7976_d_n14;

        let (assign13620_e7990, assign13620_e7990_d_n0, assign13620_e7990_d_n2, assign13620_e7990_d_n4, assign13620_e7990_d_n5, assign13620_e7990_d_n6, assign13620_e7990_d_n7, assign13620_e7990_d_n8, assign13620_e7990_d_n9, assign13620_e7990_d_n10, assign13620_e7990_d_n11, assign13620_e7990_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 != 0.0)) {
        let assign13620_e7983: f64 = (p.p390 * locals.var_tdiff0);
        let assign13620_e7984: f64 = (1.0 + assign13620_e7983);
        let assign13620_e7987: f64 = (p.p391 * locals.var_tdiff0_2);
        let assign13620_e7988: f64 = (assign13620_e7984 + assign13620_e7987);
        (assign13620_e7988, ((p.p390 * locals.var_tdiff0_dn0) + (p.p391 * locals.var_tdiff0_2_dn0)), ((p.p390 * locals.var_tdiff0_dn2) + (p.p391 * locals.var_tdiff0_2_dn2)), ((p.p390 * locals.var_tdiff0_dn4) + (p.p391 * locals.var_tdiff0_2_dn4)), ((p.p390 * locals.var_tdiff0_dn5) + (p.p391 * locals.var_tdiff0_2_dn5)), ((p.p390 * locals.var_tdiff0_dn6) + (p.p391 * locals.var_tdiff0_2_dn6)), ((p.p390 * locals.var_tdiff0_dn7) + (p.p391 * locals.var_tdiff0_2_dn7)), ((p.p390 * locals.var_tdiff0_dn8) + (p.p391 * locals.var_tdiff0_2_dn8)), ((p.p390 * locals.var_tdiff0_dn9) + (p.p391 * locals.var_tdiff0_2_dn9)), ((p.p390 * locals.var_tdiff0_dn10) + (p.p391 * locals.var_tdiff0_2_dn10)), ((p.p390 * locals.var_tdiff0_dn11) + (p.p391 * locals.var_tdiff0_2_dn11)), ((p.p390 * locals.var_tdiff0_dn14) + (p.p391 * locals.var_tdiff0_2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign13620_e7990;
        locals.var_t1_dn0 = assign13620_e7990_d_n0;
        locals.var_t1_dn2 = assign13620_e7990_d_n2;
        locals.var_t1_dn4 = assign13620_e7990_d_n4;
        locals.var_t1_dn5 = assign13620_e7990_d_n5;
        locals.var_t1_dn6 = assign13620_e7990_d_n6;
        locals.var_t1_dn7 = assign13620_e7990_d_n7;
        locals.var_t1_dn8 = assign13620_e7990_d_n8;
        locals.var_t1_dn9 = assign13620_e7990_d_n9;
        locals.var_t1_dn10 = assign13620_e7990_d_n10;
        locals.var_t1_dn11 = assign13620_e7990_d_n11;
        locals.var_t1_dn14 = assign13620_e7990_d_n14;

        let (assign13630_e7998, assign13630_e7998_d_n0, assign13630_e7998_d_n2, assign13630_e7998_d_n4, assign13630_e7998_d_n5, assign13630_e7998_d_n6, assign13630_e7998_d_n7, assign13630_e7998_d_n8, assign13630_e7998_d_n9, assign13630_e7998_d_n10, assign13630_e7998_d_n11, assign13630_e7998_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 != 0.0)) {
        let assign13630_e7996: f64 = (locals.var_ninvd0cres * locals.var_t1);
        (assign13630_e7996, ((locals.var_ninvd0cres_dn0 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn0)), ((locals.var_ninvd0cres_dn2 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn2)), ((locals.var_ninvd0cres_dn4 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn4)), ((locals.var_ninvd0cres_dn5 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn5)), ((locals.var_ninvd0cres_dn6 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn6)), ((locals.var_ninvd0cres_dn7 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn7)), ((locals.var_ninvd0cres_dn8 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn8)), ((locals.var_ninvd0cres_dn9 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn9)), ((locals.var_ninvd0cres_dn10 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn10)), ((locals.var_ninvd0cres_dn11 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn11)), ((locals.var_ninvd0cres_dn14 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn14)),)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn11, locals.var_ninvdecres_dn14,)
    }
};
        locals.var_ninvdecres = assign13630_e7998;
        locals.var_ninvdecres_dn0 = assign13630_e7998_d_n0;
        locals.var_ninvdecres_dn2 = assign13630_e7998_d_n2;
        locals.var_ninvdecres_dn4 = assign13630_e7998_d_n4;
        locals.var_ninvdecres_dn5 = assign13630_e7998_d_n5;
        locals.var_ninvdecres_dn6 = assign13630_e7998_d_n6;
        locals.var_ninvdecres_dn7 = assign13630_e7998_d_n7;
        locals.var_ninvdecres_dn8 = assign13630_e7998_d_n8;
        locals.var_ninvdecres_dn9 = assign13630_e7998_d_n9;
        locals.var_ninvdecres_dn10 = assign13630_e7998_d_n10;
        locals.var_ninvdecres_dn11 = assign13630_e7998_d_n11;
        locals.var_ninvdecres_dn14 = assign13630_e7998_d_n14;

    }

    pub(super) fn stamp_transient_block_24(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13640_e8006, assign13640_e8006_d_n0, assign13640_e8006_d_n2, assign13640_e8006_d_n4, assign13640_e8006_d_n5, assign13640_e8006_d_n6, assign13640_e8006_d_n7, assign13640_e8006_d_n8, assign13640_e8006_d_n9, assign13640_e8006_d_n10, assign13640_e8006_d_n11, assign13640_e8006_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 != 0.0)) {
        let assign13640_e8004: f64 = (locals.var_ninvd0hres * locals.var_t1);
        (assign13640_e8004, ((locals.var_ninvd0hres_dn0 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn0)), ((locals.var_ninvd0hres_dn2 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn2)), ((locals.var_ninvd0hres_dn4 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn4)), ((locals.var_ninvd0hres_dn5 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn5)), ((locals.var_ninvd0hres_dn6 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn6)), ((locals.var_ninvd0hres_dn7 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn7)), ((locals.var_ninvd0hres_dn8 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn8)), ((locals.var_ninvd0hres_dn9 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn9)), ((locals.var_ninvd0hres_dn10 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn10)), ((locals.var_ninvd0hres_dn11 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn11)), ((locals.var_ninvd0hres_dn14 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn14)),)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn11, locals.var_ninvdehres_dn14,)
    }
};
        locals.var_ninvdehres = assign13640_e8006;
        locals.var_ninvdehres_dn0 = assign13640_e8006_d_n0;
        locals.var_ninvdehres_dn2 = assign13640_e8006_d_n2;
        locals.var_ninvdehres_dn4 = assign13640_e8006_d_n4;
        locals.var_ninvdehres_dn5 = assign13640_e8006_d_n5;
        locals.var_ninvdehres_dn6 = assign13640_e8006_d_n6;
        locals.var_ninvdehres_dn7 = assign13640_e8006_d_n7;
        locals.var_ninvdehres_dn8 = assign13640_e8006_d_n8;
        locals.var_ninvdehres_dn9 = assign13640_e8006_d_n9;
        locals.var_ninvdehres_dn10 = assign13640_e8006_d_n10;
        locals.var_ninvdehres_dn11 = assign13640_e8006_d_n11;
        locals.var_ninvdehres_dn14 = assign13640_e8006_d_n14;

        let (assign13650_e8023, assign13650_e8023_d_n0, assign13650_e8023_d_n2, assign13650_e8023_d_n4, assign13650_e8023_d_n5, assign13650_e8023_d_n6, assign13650_e8023_d_n7, assign13650_e8023_d_n8, assign13650_e8023_d_n9, assign13650_e8023_d_n10, assign13650_e8023_d_n11, assign13650_e8023_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 == 0.0)) {
        let assign13650_e8015: f64 = (p.p324 * locals.var_tdiff);
        let assign13650_e8016: f64 = (1.0 + assign13650_e8015);
        let assign13650_e8019: f64 = (p.p325 * locals.var_tdiff_2);
        let assign13650_e8020: f64 = (assign13650_e8016 + assign13650_e8019);
        let assign13650_e8021: f64 = (locals.var_ninvd0 * assign13650_e8020);
        (assign13650_e8021, (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn0) + (p.p325 * locals.var_tdiff_2_dn0))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn2) + (p.p325 * locals.var_tdiff_2_dn2))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn4) + (p.p325 * locals.var_tdiff_2_dn4))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn5) + (p.p325 * locals.var_tdiff_2_dn5))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn6) + (p.p325 * locals.var_tdiff_2_dn6))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn7) + (p.p325 * locals.var_tdiff_2_dn7))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn8) + (p.p325 * locals.var_tdiff_2_dn8))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn9) + (p.p325 * locals.var_tdiff_2_dn9))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn10) + (p.p325 * locals.var_tdiff_2_dn10))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn11) + (p.p325 * locals.var_tdiff_2_dn11))), (locals.var_ninvd0 * ((p.p324 * locals.var_tdiff_dn14) + (p.p325 * locals.var_tdiff_2_dn14))),)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn11, locals.var_ninvde_dn14,)
    }
};
        locals.var_ninvde = assign13650_e8023;
        locals.var_ninvde_dn0 = assign13650_e8023_d_n0;
        locals.var_ninvde_dn2 = assign13650_e8023_d_n2;
        locals.var_ninvde_dn4 = assign13650_e8023_d_n4;
        locals.var_ninvde_dn5 = assign13650_e8023_d_n5;
        locals.var_ninvde_dn6 = assign13650_e8023_d_n6;
        locals.var_ninvde_dn7 = assign13650_e8023_d_n7;
        locals.var_ninvde_dn8 = assign13650_e8023_d_n8;
        locals.var_ninvde_dn9 = assign13650_e8023_d_n9;
        locals.var_ninvde_dn10 = assign13650_e8023_d_n10;
        locals.var_ninvde_dn11 = assign13650_e8023_d_n11;
        locals.var_ninvde_dn14 = assign13650_e8023_d_n14;

        let (assign13660_e8038, assign13660_e8038_d_n0, assign13660_e8038_d_n2, assign13660_e8038_d_n4, assign13660_e8038_d_n5, assign13660_e8038_d_n6, assign13660_e8038_d_n7, assign13660_e8038_d_n8, assign13660_e8038_d_n9, assign13660_e8038_d_n10, assign13660_e8038_d_n11, assign13660_e8038_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 == 0.0)) {
        let assign13660_e8031: f64 = (p.p390 * locals.var_tdiff);
        let assign13660_e8032: f64 = (1.0 + assign13660_e8031);
        let assign13660_e8035: f64 = (p.p391 * locals.var_tdiff_2);
        let assign13660_e8036: f64 = (assign13660_e8032 + assign13660_e8035);
        (assign13660_e8036, ((p.p390 * locals.var_tdiff_dn0) + (p.p391 * locals.var_tdiff_2_dn0)), ((p.p390 * locals.var_tdiff_dn2) + (p.p391 * locals.var_tdiff_2_dn2)), ((p.p390 * locals.var_tdiff_dn4) + (p.p391 * locals.var_tdiff_2_dn4)), ((p.p390 * locals.var_tdiff_dn5) + (p.p391 * locals.var_tdiff_2_dn5)), ((p.p390 * locals.var_tdiff_dn6) + (p.p391 * locals.var_tdiff_2_dn6)), ((p.p390 * locals.var_tdiff_dn7) + (p.p391 * locals.var_tdiff_2_dn7)), ((p.p390 * locals.var_tdiff_dn8) + (p.p391 * locals.var_tdiff_2_dn8)), ((p.p390 * locals.var_tdiff_dn9) + (p.p391 * locals.var_tdiff_2_dn9)), ((p.p390 * locals.var_tdiff_dn10) + (p.p391 * locals.var_tdiff_2_dn10)), ((p.p390 * locals.var_tdiff_dn11) + (p.p391 * locals.var_tdiff_2_dn11)), ((p.p390 * locals.var_tdiff_dn14) + (p.p391 * locals.var_tdiff_2_dn14)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign13660_e8038;
        locals.var_t1_dn0 = assign13660_e8038_d_n0;
        locals.var_t1_dn2 = assign13660_e8038_d_n2;
        locals.var_t1_dn4 = assign13660_e8038_d_n4;
        locals.var_t1_dn5 = assign13660_e8038_d_n5;
        locals.var_t1_dn6 = assign13660_e8038_d_n6;
        locals.var_t1_dn7 = assign13660_e8038_d_n7;
        locals.var_t1_dn8 = assign13660_e8038_d_n8;
        locals.var_t1_dn9 = assign13660_e8038_d_n9;
        locals.var_t1_dn10 = assign13660_e8038_d_n10;
        locals.var_t1_dn11 = assign13660_e8038_d_n11;
        locals.var_t1_dn14 = assign13660_e8038_d_n14;

        let (assign13670_e8047, assign13670_e8047_d_n0, assign13670_e8047_d_n2, assign13670_e8047_d_n4, assign13670_e8047_d_n5, assign13670_e8047_d_n6, assign13670_e8047_d_n7, assign13670_e8047_d_n8, assign13670_e8047_d_n9, assign13670_e8047_d_n10, assign13670_e8047_d_n11, assign13670_e8047_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 == 0.0)) {
        let assign13670_e8045: f64 = (locals.var_ninvd0cres * locals.var_t1);
        (assign13670_e8045, ((locals.var_ninvd0cres_dn0 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn0)), ((locals.var_ninvd0cres_dn2 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn2)), ((locals.var_ninvd0cres_dn4 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn4)), ((locals.var_ninvd0cres_dn5 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn5)), ((locals.var_ninvd0cres_dn6 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn6)), ((locals.var_ninvd0cres_dn7 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn7)), ((locals.var_ninvd0cres_dn8 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn8)), ((locals.var_ninvd0cres_dn9 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn9)), ((locals.var_ninvd0cres_dn10 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn10)), ((locals.var_ninvd0cres_dn11 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn11)), ((locals.var_ninvd0cres_dn14 * locals.var_t1) + (locals.var_ninvd0cres * locals.var_t1_dn14)),)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn11, locals.var_ninvdecres_dn14,)
    }
};
        locals.var_ninvdecres = assign13670_e8047;
        locals.var_ninvdecres_dn0 = assign13670_e8047_d_n0;
        locals.var_ninvdecres_dn2 = assign13670_e8047_d_n2;
        locals.var_ninvdecres_dn4 = assign13670_e8047_d_n4;
        locals.var_ninvdecres_dn5 = assign13670_e8047_d_n5;
        locals.var_ninvdecres_dn6 = assign13670_e8047_d_n6;
        locals.var_ninvdecres_dn7 = assign13670_e8047_d_n7;
        locals.var_ninvdecres_dn8 = assign13670_e8047_d_n8;
        locals.var_ninvdecres_dn9 = assign13670_e8047_d_n9;
        locals.var_ninvdecres_dn10 = assign13670_e8047_d_n10;
        locals.var_ninvdecres_dn11 = assign13670_e8047_d_n11;
        locals.var_ninvdecres_dn14 = assign13670_e8047_d_n14;

        let (assign13680_e8056, assign13680_e8056_d_n0, assign13680_e8056_d_n2, assign13680_e8056_d_n4, assign13680_e8056_d_n5, assign13680_e8056_d_n6, assign13680_e8056_d_n7, assign13680_e8056_d_n8, assign13680_e8056_d_n9, assign13680_e8056_d_n10, assign13680_e8056_d_n11, assign13680_e8056_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard300 == 0.0)) {
        let assign13680_e8054: f64 = (locals.var_ninvd0hres * locals.var_t1);
        (assign13680_e8054, ((locals.var_ninvd0hres_dn0 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn0)), ((locals.var_ninvd0hres_dn2 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn2)), ((locals.var_ninvd0hres_dn4 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn4)), ((locals.var_ninvd0hres_dn5 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn5)), ((locals.var_ninvd0hres_dn6 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn6)), ((locals.var_ninvd0hres_dn7 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn7)), ((locals.var_ninvd0hres_dn8 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn8)), ((locals.var_ninvd0hres_dn9 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn9)), ((locals.var_ninvd0hres_dn10 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn10)), ((locals.var_ninvd0hres_dn11 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn11)), ((locals.var_ninvd0hres_dn14 * locals.var_t1) + (locals.var_ninvd0hres * locals.var_t1_dn14)),)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn11, locals.var_ninvdehres_dn14,)
    }
};
        locals.var_ninvdehres = assign13680_e8056;
        locals.var_ninvdehres_dn0 = assign13680_e8056_d_n0;
        locals.var_ninvdehres_dn2 = assign13680_e8056_d_n2;
        locals.var_ninvdehres_dn4 = assign13680_e8056_d_n4;
        locals.var_ninvdehres_dn5 = assign13680_e8056_d_n5;
        locals.var_ninvdehres_dn6 = assign13680_e8056_d_n6;
        locals.var_ninvdehres_dn7 = assign13680_e8056_d_n7;
        locals.var_ninvdehres_dn8 = assign13680_e8056_d_n8;
        locals.var_ninvdehres_dn9 = assign13680_e8056_d_n9;
        locals.var_ninvdehres_dn10 = assign13680_e8056_d_n10;
        locals.var_ninvdehres_dn11 = assign13680_e8056_d_n11;
        locals.var_ninvdehres_dn14 = assign13680_e8056_d_n14;

        let assign13700_e8064: f64 = if locals.var_ninvde < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard302 = assign13700_e8064;

        let (assign13710_e8070, assign13710_e8070_d_n0, assign13710_e8070_d_n2, assign13710_e8070_d_n4, assign13710_e8070_d_n5, assign13710_e8070_d_n6, assign13710_e8070_d_n7, assign13710_e8070_d_n8, assign13710_e8070_d_n9, assign13710_e8070_d_n10, assign13710_e8070_d_n11, assign13710_e8070_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard302 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvde, locals.var_ninvde_dn0, locals.var_ninvde_dn2, locals.var_ninvde_dn4, locals.var_ninvde_dn5, locals.var_ninvde_dn6, locals.var_ninvde_dn7, locals.var_ninvde_dn8, locals.var_ninvde_dn9, locals.var_ninvde_dn10, locals.var_ninvde_dn11, locals.var_ninvde_dn14,)
    }
};
        locals.var_ninvde = assign13710_e8070;
        locals.var_ninvde_dn0 = assign13710_e8070_d_n0;
        locals.var_ninvde_dn2 = assign13710_e8070_d_n2;
        locals.var_ninvde_dn4 = assign13710_e8070_d_n4;
        locals.var_ninvde_dn5 = assign13710_e8070_d_n5;
        locals.var_ninvde_dn6 = assign13710_e8070_d_n6;
        locals.var_ninvde_dn7 = assign13710_e8070_d_n7;
        locals.var_ninvde_dn8 = assign13710_e8070_d_n8;
        locals.var_ninvde_dn9 = assign13710_e8070_d_n9;
        locals.var_ninvde_dn10 = assign13710_e8070_d_n10;
        locals.var_ninvde_dn11 = assign13710_e8070_d_n11;
        locals.var_ninvde_dn14 = assign13710_e8070_d_n14;

        let assign13730_e8078: f64 = if locals.var_ninvdecres < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard304 = assign13730_e8078;

        let (assign13740_e8084, assign13740_e8084_d_n0, assign13740_e8084_d_n2, assign13740_e8084_d_n4, assign13740_e8084_d_n5, assign13740_e8084_d_n6, assign13740_e8084_d_n7, assign13740_e8084_d_n8, assign13740_e8084_d_n9, assign13740_e8084_d_n10, assign13740_e8084_d_n11, assign13740_e8084_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard304 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvdecres, locals.var_ninvdecres_dn0, locals.var_ninvdecres_dn2, locals.var_ninvdecres_dn4, locals.var_ninvdecres_dn5, locals.var_ninvdecres_dn6, locals.var_ninvdecres_dn7, locals.var_ninvdecres_dn8, locals.var_ninvdecres_dn9, locals.var_ninvdecres_dn10, locals.var_ninvdecres_dn11, locals.var_ninvdecres_dn14,)
    }
};
        locals.var_ninvdecres = assign13740_e8084;
        locals.var_ninvdecres_dn0 = assign13740_e8084_d_n0;
        locals.var_ninvdecres_dn2 = assign13740_e8084_d_n2;
        locals.var_ninvdecres_dn4 = assign13740_e8084_d_n4;
        locals.var_ninvdecres_dn5 = assign13740_e8084_d_n5;
        locals.var_ninvdecres_dn6 = assign13740_e8084_d_n6;
        locals.var_ninvdecres_dn7 = assign13740_e8084_d_n7;
        locals.var_ninvdecres_dn8 = assign13740_e8084_d_n8;
        locals.var_ninvdecres_dn9 = assign13740_e8084_d_n9;
        locals.var_ninvdecres_dn10 = assign13740_e8084_d_n10;
        locals.var_ninvdecres_dn11 = assign13740_e8084_d_n11;
        locals.var_ninvdecres_dn14 = assign13740_e8084_d_n14;

        let assign13760_e8092: f64 = if locals.var_ninvdehres < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard306 = assign13760_e8092;

        let (assign13770_e8098, assign13770_e8098_d_n0, assign13770_e8098_d_n2, assign13770_e8098_d_n4, assign13770_e8098_d_n5, assign13770_e8098_d_n6, assign13770_e8098_d_n7, assign13770_e8098_d_n8, assign13770_e8098_d_n9, assign13770_e8098_d_n10, assign13770_e8098_d_n11, assign13770_e8098_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard306 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_ninvdehres, locals.var_ninvdehres_dn0, locals.var_ninvdehres_dn2, locals.var_ninvdehres_dn4, locals.var_ninvdehres_dn5, locals.var_ninvdehres_dn6, locals.var_ninvdehres_dn7, locals.var_ninvdehres_dn8, locals.var_ninvdehres_dn9, locals.var_ninvdehres_dn10, locals.var_ninvdehres_dn11, locals.var_ninvdehres_dn14,)
    }
};
        locals.var_ninvdehres = assign13770_e8098;
        locals.var_ninvdehres_dn0 = assign13770_e8098_d_n0;
        locals.var_ninvdehres_dn2 = assign13770_e8098_d_n2;
        locals.var_ninvdehres_dn4 = assign13770_e8098_d_n4;
        locals.var_ninvdehres_dn5 = assign13770_e8098_d_n5;
        locals.var_ninvdehres_dn6 = assign13770_e8098_d_n6;
        locals.var_ninvdehres_dn7 = assign13770_e8098_d_n7;
        locals.var_ninvdehres_dn8 = assign13770_e8098_d_n8;
        locals.var_ninvdehres_dn9 = assign13770_e8098_d_n9;
        locals.var_ninvdehres_dn10 = assign13770_e8098_d_n10;
        locals.var_ninvdehres_dn11 = assign13770_e8098_d_n11;
        locals.var_ninvdehres_dn14 = assign13770_e8098_d_n14;

        let (assign13780_e8114, assign13780_e8114_d_n0, assign13780_e8114_d_n2, assign13780_e8114_d_n4, assign13780_e8114_d_n5, assign13780_e8114_d_n6, assign13780_e8114_d_n7, assign13780_e8114_d_n8, assign13780_e8114_d_n9, assign13780_e8114_d_n10, assign13780_e8114_d_n11, assign13780_e8114_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (p.p53 != 0.0)) {
        let assign13780_e8105: f64 = (p.p328 * locals.var_tdiff0);
        let assign13780_e8106: f64 = (locals.var_uc_rth0 + assign13780_e8105);
        let assign13780_e8109: f64 = (p.p329 * locals.var_tdiff0_2);
        let assign13780_e8110: f64 = (assign13780_e8106 + assign13780_e8109);
        let assign13780_e8112: f64 = (assign13780_e8110 * locals.var_rthtemp0);
        (assign13780_e8112, (((p.p328 * locals.var_tdiff0_dn0) + (p.p329 * locals.var_tdiff0_2_dn0)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn2) + (p.p329 * locals.var_tdiff0_2_dn2)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn4) + (p.p329 * locals.var_tdiff0_2_dn4)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn5) + (p.p329 * locals.var_tdiff0_2_dn5)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn6) + (p.p329 * locals.var_tdiff0_2_dn6)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn7) + (p.p329 * locals.var_tdiff0_2_dn7)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn8) + (p.p329 * locals.var_tdiff0_2_dn8)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn9) + (p.p329 * locals.var_tdiff0_2_dn9)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn10) + (p.p329 * locals.var_tdiff0_2_dn10)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn11) + (p.p329 * locals.var_tdiff0_2_dn11)) * locals.var_rthtemp0), (((p.p328 * locals.var_tdiff0_dn14) + (p.p329 * locals.var_tdiff0_2_dn14)) * locals.var_rthtemp0),)
    } else {
        (locals.var_rth, locals.var_rth_dn0, locals.var_rth_dn2, locals.var_rth_dn4, locals.var_rth_dn5, locals.var_rth_dn6, locals.var_rth_dn7, locals.var_rth_dn8, locals.var_rth_dn9, locals.var_rth_dn10, locals.var_rth_dn11, locals.var_rth_dn14,)
    }
};
        locals.var_rth = assign13780_e8114;
        locals.var_rth_dn0 = assign13780_e8114_d_n0;
        locals.var_rth_dn2 = assign13780_e8114_d_n2;
        locals.var_rth_dn4 = assign13780_e8114_d_n4;
        locals.var_rth_dn5 = assign13780_e8114_d_n5;
        locals.var_rth_dn6 = assign13780_e8114_d_n6;
        locals.var_rth_dn7 = assign13780_e8114_d_n7;
        locals.var_rth_dn8 = assign13780_e8114_d_n8;
        locals.var_rth_dn9 = assign13780_e8114_d_n9;
        locals.var_rth_dn10 = assign13780_e8114_d_n10;
        locals.var_rth_dn11 = assign13780_e8114_d_n11;
        locals.var_rth_dn14 = assign13780_e8114_d_n14;

        let assign13800_e8122: f64 = if locals.var_rth < 0.0001 { 1.0 } else { 0.0 };
        locals.var_guard308 = assign13800_e8122;

        let (assign13810_e8130, assign13810_e8130_d_n0, assign13810_e8130_d_n2, assign13810_e8130_d_n4, assign13810_e8130_d_n5, assign13810_e8130_d_n6, assign13810_e8130_d_n7, assign13810_e8130_d_n8, assign13810_e8130_d_n9, assign13810_e8130_d_n10, assign13810_e8130_d_n11, assign13810_e8130_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (p.p53 != 0.0)) && (locals.var_guard308 != 0.0)) {
        (0.0001, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rth, locals.var_rth_dn0, locals.var_rth_dn2, locals.var_rth_dn4, locals.var_rth_dn5, locals.var_rth_dn6, locals.var_rth_dn7, locals.var_rth_dn8, locals.var_rth_dn9, locals.var_rth_dn10, locals.var_rth_dn11, locals.var_rth_dn14,)
    }
};
        locals.var_rth = assign13810_e8130;
        locals.var_rth_dn0 = assign13810_e8130_d_n0;
        locals.var_rth_dn2 = assign13810_e8130_d_n2;
        locals.var_rth_dn4 = assign13810_e8130_d_n4;
        locals.var_rth_dn5 = assign13810_e8130_d_n5;
        locals.var_rth_dn6 = assign13810_e8130_d_n6;
        locals.var_rth_dn7 = assign13810_e8130_d_n7;
        locals.var_rth_dn8 = assign13810_e8130_d_n8;
        locals.var_rth_dn9 = assign13810_e8130_d_n9;
        locals.var_rth_dn10 = assign13810_e8130_d_n10;
        locals.var_rth_dn11 = assign13810_e8130_d_n11;
        locals.var_rth_dn14 = assign13810_e8130_d_n14;

        let (assign13820_e8142, assign13820_e8142_d_n0, assign13820_e8142_d_n2, assign13820_e8142_d_n4, assign13820_e8142_d_n5, assign13820_e8142_d_n6, assign13820_e8142_d_n7, assign13820_e8142_d_n8, assign13820_e8142_d_n9, assign13820_e8142_d_n10, assign13820_e8142_d_n11, assign13820_e8142_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13820_e8135: f64 = (p.p330 * locals.var_tdiff0);
        let assign13820_e8136: f64 = (locals.var_uc_powrat + assign13820_e8135);
        let assign13820_e8139: f64 = (p.p331 * locals.var_tdiff0_2);
        let assign13820_e8140: f64 = (assign13820_e8136 + assign13820_e8139);
        (assign13820_e8140, ((p.p330 * locals.var_tdiff0_dn0) + (p.p331 * locals.var_tdiff0_2_dn0)), ((p.p330 * locals.var_tdiff0_dn2) + (p.p331 * locals.var_tdiff0_2_dn2)), ((p.p330 * locals.var_tdiff0_dn4) + (p.p331 * locals.var_tdiff0_2_dn4)), ((p.p330 * locals.var_tdiff0_dn5) + (p.p331 * locals.var_tdiff0_2_dn5)), ((p.p330 * locals.var_tdiff0_dn6) + (p.p331 * locals.var_tdiff0_2_dn6)), ((p.p330 * locals.var_tdiff0_dn7) + (p.p331 * locals.var_tdiff0_2_dn7)), ((p.p330 * locals.var_tdiff0_dn8) + (p.p331 * locals.var_tdiff0_2_dn8)), ((p.p330 * locals.var_tdiff0_dn9) + (p.p331 * locals.var_tdiff0_2_dn9)), ((p.p330 * locals.var_tdiff0_dn10) + (p.p331 * locals.var_tdiff0_2_dn10)), ((p.p330 * locals.var_tdiff0_dn11) + (p.p331 * locals.var_tdiff0_2_dn11)), ((p.p330 * locals.var_tdiff0_dn14) + (p.p331 * locals.var_tdiff0_2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign13820_e8142;
        locals.var_t2_dn0 = assign13820_e8142_d_n0;
        locals.var_t2_dn2 = assign13820_e8142_d_n2;
        locals.var_t2_dn4 = assign13820_e8142_d_n4;
        locals.var_t2_dn5 = assign13820_e8142_d_n5;
        locals.var_t2_dn6 = assign13820_e8142_d_n6;
        locals.var_t2_dn7 = assign13820_e8142_d_n7;
        locals.var_t2_dn8 = assign13820_e8142_d_n8;
        locals.var_t2_dn9 = assign13820_e8142_d_n9;
        locals.var_t2_dn10 = assign13820_e8142_d_n10;
        locals.var_t2_dn11 = assign13820_e8142_d_n11;
        locals.var_t2_dn14 = assign13820_e8142_d_n14;

        let (assign13830_e8150, assign13830_e8150_d_n0, assign13830_e8150_d_n2, assign13830_e8150_d_n4, assign13830_e8150_d_n5, assign13830_e8150_d_n6, assign13830_e8150_d_n7, assign13830_e8150_d_n8, assign13830_e8150_d_n9, assign13830_e8150_d_n10, assign13830_e8150_d_n11, assign13830_e8150_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13830_e8146: f64 = locals.var_t2;
        let assign13830_e8148: f64 = (assign13830_e8146 - 0.05);
        (assign13830_e8148, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign13830_e8150;
        locals.var_tmf1_dn0 = assign13830_e8150_d_n0;
        locals.var_tmf1_dn2 = assign13830_e8150_d_n2;
        locals.var_tmf1_dn4 = assign13830_e8150_d_n4;
        locals.var_tmf1_dn5 = assign13830_e8150_d_n5;
        locals.var_tmf1_dn6 = assign13830_e8150_d_n6;
        locals.var_tmf1_dn7 = assign13830_e8150_d_n7;
        locals.var_tmf1_dn8 = assign13830_e8150_d_n8;
        locals.var_tmf1_dn9 = assign13830_e8150_d_n9;
        locals.var_tmf1_dn10 = assign13830_e8150_d_n10;
        locals.var_tmf1_dn11 = assign13830_e8150_d_n11;
        locals.var_tmf1_dn14 = assign13830_e8150_d_n14;

        let (assign13840_e8158, assign13840_e8158_d_n0, assign13840_e8158_d_n2, assign13840_e8158_d_n4, assign13840_e8158_d_n5, assign13840_e8158_d_n6, assign13840_e8158_d_n7, assign13840_e8158_d_n8, assign13840_e8158_d_n9, assign13840_e8158_d_n10, assign13840_e8158_d_n11, assign13840_e8158_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign13840_e8158;
        locals.var_tmf2_dn0 = assign13840_e8158_d_n0;
        locals.var_tmf2_dn2 = assign13840_e8158_d_n2;
        locals.var_tmf2_dn4 = assign13840_e8158_d_n4;
        locals.var_tmf2_dn5 = assign13840_e8158_d_n5;
        locals.var_tmf2_dn6 = assign13840_e8158_d_n6;
        locals.var_tmf2_dn7 = assign13840_e8158_d_n7;
        locals.var_tmf2_dn8 = assign13840_e8158_d_n8;
        locals.var_tmf2_dn9 = assign13840_e8158_d_n9;
        locals.var_tmf2_dn10 = assign13840_e8158_d_n10;
        locals.var_tmf2_dn11 = assign13840_e8158_d_n11;
        locals.var_tmf2_dn14 = assign13840_e8158_d_n14;

        let (assign13850_e8168, assign13850_e8168_d_n0, assign13850_e8168_d_n2, assign13850_e8168_d_n4, assign13850_e8168_d_n5, assign13850_e8168_d_n6, assign13850_e8168_d_n7, assign13850_e8168_d_n8, assign13850_e8168_d_n9, assign13850_e8168_d_n10, assign13850_e8168_d_n11, assign13850_e8168_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let (assign13850_e8166, assign13850_e8166_d_n0, assign13850_e8166_d_n2, assign13850_e8166_d_n4, assign13850_e8166_d_n5, assign13850_e8166_d_n6, assign13850_e8166_d_n7, assign13850_e8166_d_n8, assign13850_e8166_d_n9, assign13850_e8166_d_n10, assign13850_e8166_d_n11, assign13850_e8166_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign13850_e8165: f64 = (-locals.var_tmf2);
                (assign13850_e8165, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign13850_e8166, assign13850_e8166_d_n0, assign13850_e8166_d_n2, assign13850_e8166_d_n4, assign13850_e8166_d_n5, assign13850_e8166_d_n6, assign13850_e8166_d_n7, assign13850_e8166_d_n8, assign13850_e8166_d_n9, assign13850_e8166_d_n10, assign13850_e8166_d_n11, assign13850_e8166_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign13850_e8168;
        locals.var_tmf2_dn0 = assign13850_e8168_d_n0;
        locals.var_tmf2_dn2 = assign13850_e8168_d_n2;
        locals.var_tmf2_dn4 = assign13850_e8168_d_n4;
        locals.var_tmf2_dn5 = assign13850_e8168_d_n5;
        locals.var_tmf2_dn6 = assign13850_e8168_d_n6;
        locals.var_tmf2_dn7 = assign13850_e8168_d_n7;
        locals.var_tmf2_dn8 = assign13850_e8168_d_n8;
        locals.var_tmf2_dn9 = assign13850_e8168_d_n9;
        locals.var_tmf2_dn10 = assign13850_e8168_d_n10;
        locals.var_tmf2_dn11 = assign13850_e8168_d_n11;
        locals.var_tmf2_dn14 = assign13850_e8168_d_n14;

        let (assign13860_e8177, assign13860_e8177_d_n0, assign13860_e8177_d_n2, assign13860_e8177_d_n4, assign13860_e8177_d_n5, assign13860_e8177_d_n6, assign13860_e8177_d_n7, assign13860_e8177_d_n8, assign13860_e8177_d_n9, assign13860_e8177_d_n10, assign13860_e8177_d_n11, assign13860_e8177_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13860_e8172: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign13860_e8174: f64 = (assign13860_e8172 + locals.var_tmf2);
        let assign13860_e8175: f64 = (assign13860_e8174).sqrt();
        (assign13860_e8175, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign13860_e8175)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign13860_e8175)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign13860_e8177;
        locals.var_tmf2_dn0 = assign13860_e8177_d_n0;
        locals.var_tmf2_dn2 = assign13860_e8177_d_n2;
        locals.var_tmf2_dn4 = assign13860_e8177_d_n4;
        locals.var_tmf2_dn5 = assign13860_e8177_d_n5;
        locals.var_tmf2_dn6 = assign13860_e8177_d_n6;
        locals.var_tmf2_dn7 = assign13860_e8177_d_n7;
        locals.var_tmf2_dn8 = assign13860_e8177_d_n8;
        locals.var_tmf2_dn9 = assign13860_e8177_d_n9;
        locals.var_tmf2_dn10 = assign13860_e8177_d_n10;
        locals.var_tmf2_dn11 = assign13860_e8177_d_n11;
        locals.var_tmf2_dn14 = assign13860_e8177_d_n14;

        let (assign13870_e8187, assign13870_e8187_d_n0, assign13870_e8187_d_n2, assign13870_e8187_d_n4, assign13870_e8187_d_n5, assign13870_e8187_d_n6, assign13870_e8187_d_n7, assign13870_e8187_d_n8, assign13870_e8187_d_n9, assign13870_e8187_d_n10, assign13870_e8187_d_n11, assign13870_e8187_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13870_e8183: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign13870_e8184: f64 = (1.0 + assign13870_e8183);
        let assign13870_e8185: f64 = (0.5 * assign13870_e8184);
        (assign13870_e8185, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign13870_e8187;
        locals.var_t0_dn0 = assign13870_e8187_d_n0;
        locals.var_t0_dn2 = assign13870_e8187_d_n2;
        locals.var_t0_dn4 = assign13870_e8187_d_n4;
        locals.var_t0_dn5 = assign13870_e8187_d_n5;
        locals.var_t0_dn6 = assign13870_e8187_d_n6;
        locals.var_t0_dn7 = assign13870_e8187_d_n7;
        locals.var_t0_dn8 = assign13870_e8187_d_n8;
        locals.var_t0_dn9 = assign13870_e8187_d_n9;
        locals.var_t0_dn10 = assign13870_e8187_d_n10;
        locals.var_t0_dn11 = assign13870_e8187_d_n11;
        locals.var_t0_dn14 = assign13870_e8187_d_n14;

        let (assign13880_e8197, assign13880_e8197_d_n0, assign13880_e8197_d_n2, assign13880_e8197_d_n4, assign13880_e8197_d_n5, assign13880_e8197_d_n6, assign13880_e8197_d_n7, assign13880_e8197_d_n8, assign13880_e8197_d_n9, assign13880_e8197_d_n10, assign13880_e8197_d_n11, assign13880_e8197_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13880_e8193: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign13880_e8194: f64 = (0.5 * assign13880_e8193);
        let assign13880_e8195: f64 = assign13880_e8194;
        (assign13880_e8195, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign13880_e8197;
        locals.var_t2_dn0 = assign13880_e8197_d_n0;
        locals.var_t2_dn2 = assign13880_e8197_d_n2;
        locals.var_t2_dn4 = assign13880_e8197_d_n4;
        locals.var_t2_dn5 = assign13880_e8197_d_n5;
        locals.var_t2_dn6 = assign13880_e8197_d_n6;
        locals.var_t2_dn7 = assign13880_e8197_d_n7;
        locals.var_t2_dn8 = assign13880_e8197_d_n8;
        locals.var_t2_dn9 = assign13880_e8197_d_n9;
        locals.var_t2_dn10 = assign13880_e8197_d_n10;
        locals.var_t2_dn11 = assign13880_e8197_d_n11;
        locals.var_t2_dn14 = assign13880_e8197_d_n14;

        let (assign13890_e8205, assign13890_e8205_d_n0, assign13890_e8205_d_n2, assign13890_e8205_d_n4, assign13890_e8205_d_n5, assign13890_e8205_d_n6, assign13890_e8205_d_n7, assign13890_e8205_d_n8, assign13890_e8205_d_n9, assign13890_e8205_d_n10, assign13890_e8205_d_n11, assign13890_e8205_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13890_e8201: f64 = (1.0 - locals.var_t2);
        let assign13890_e8203: f64 = (assign13890_e8201 - 0.05);
        (assign13890_e8203, (-locals.var_t2_dn0), (-locals.var_t2_dn2), (-locals.var_t2_dn4), (-locals.var_t2_dn5), (-locals.var_t2_dn6), (-locals.var_t2_dn7), (-locals.var_t2_dn8), (-locals.var_t2_dn9), (-locals.var_t2_dn10), (-locals.var_t2_dn11), (-locals.var_t2_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign13890_e8205;
        locals.var_tmf1_dn0 = assign13890_e8205_d_n0;
        locals.var_tmf1_dn2 = assign13890_e8205_d_n2;
        locals.var_tmf1_dn4 = assign13890_e8205_d_n4;
        locals.var_tmf1_dn5 = assign13890_e8205_d_n5;
        locals.var_tmf1_dn6 = assign13890_e8205_d_n6;
        locals.var_tmf1_dn7 = assign13890_e8205_d_n7;
        locals.var_tmf1_dn8 = assign13890_e8205_d_n8;
        locals.var_tmf1_dn9 = assign13890_e8205_d_n9;
        locals.var_tmf1_dn10 = assign13890_e8205_d_n10;
        locals.var_tmf1_dn11 = assign13890_e8205_d_n11;
        locals.var_tmf1_dn14 = assign13890_e8205_d_n14;

        let (assign13900_e8213, assign13900_e8213_d_n0, assign13900_e8213_d_n2, assign13900_e8213_d_n4, assign13900_e8213_d_n5, assign13900_e8213_d_n6, assign13900_e8213_d_n7, assign13900_e8213_d_n8, assign13900_e8213_d_n9, assign13900_e8213_d_n10, assign13900_e8213_d_n11, assign13900_e8213_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13900_e8209: f64 = 4.0;
        let assign13900_e8211: f64 = (assign13900_e8209 * 0.05);
        (assign13900_e8211, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign13900_e8213;
        locals.var_tmf2_dn0 = assign13900_e8213_d_n0;
        locals.var_tmf2_dn2 = assign13900_e8213_d_n2;
        locals.var_tmf2_dn4 = assign13900_e8213_d_n4;
        locals.var_tmf2_dn5 = assign13900_e8213_d_n5;
        locals.var_tmf2_dn6 = assign13900_e8213_d_n6;
        locals.var_tmf2_dn7 = assign13900_e8213_d_n7;
        locals.var_tmf2_dn8 = assign13900_e8213_d_n8;
        locals.var_tmf2_dn9 = assign13900_e8213_d_n9;
        locals.var_tmf2_dn10 = assign13900_e8213_d_n10;
        locals.var_tmf2_dn11 = assign13900_e8213_d_n11;
        locals.var_tmf2_dn14 = assign13900_e8213_d_n14;

        let (assign13910_e8223, assign13910_e8223_d_n0, assign13910_e8223_d_n2, assign13910_e8223_d_n4, assign13910_e8223_d_n5, assign13910_e8223_d_n6, assign13910_e8223_d_n7, assign13910_e8223_d_n8, assign13910_e8223_d_n9, assign13910_e8223_d_n10, assign13910_e8223_d_n11, assign13910_e8223_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let (assign13910_e8221, assign13910_e8221_d_n0, assign13910_e8221_d_n2, assign13910_e8221_d_n4, assign13910_e8221_d_n5, assign13910_e8221_d_n6, assign13910_e8221_d_n7, assign13910_e8221_d_n8, assign13910_e8221_d_n9, assign13910_e8221_d_n10, assign13910_e8221_d_n11, assign13910_e8221_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign13910_e8220: f64 = (-locals.var_tmf2);
                (assign13910_e8220, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign13910_e8221, assign13910_e8221_d_n0, assign13910_e8221_d_n2, assign13910_e8221_d_n4, assign13910_e8221_d_n5, assign13910_e8221_d_n6, assign13910_e8221_d_n7, assign13910_e8221_d_n8, assign13910_e8221_d_n9, assign13910_e8221_d_n10, assign13910_e8221_d_n11, assign13910_e8221_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign13910_e8223;
        locals.var_tmf2_dn0 = assign13910_e8223_d_n0;
        locals.var_tmf2_dn2 = assign13910_e8223_d_n2;
        locals.var_tmf2_dn4 = assign13910_e8223_d_n4;
        locals.var_tmf2_dn5 = assign13910_e8223_d_n5;
        locals.var_tmf2_dn6 = assign13910_e8223_d_n6;
        locals.var_tmf2_dn7 = assign13910_e8223_d_n7;
        locals.var_tmf2_dn8 = assign13910_e8223_d_n8;
        locals.var_tmf2_dn9 = assign13910_e8223_d_n9;
        locals.var_tmf2_dn10 = assign13910_e8223_d_n10;
        locals.var_tmf2_dn11 = assign13910_e8223_d_n11;
        locals.var_tmf2_dn14 = assign13910_e8223_d_n14;

        let (assign13920_e8232, assign13920_e8232_d_n0, assign13920_e8232_d_n2, assign13920_e8232_d_n4, assign13920_e8232_d_n5, assign13920_e8232_d_n6, assign13920_e8232_d_n7, assign13920_e8232_d_n8, assign13920_e8232_d_n9, assign13920_e8232_d_n10, assign13920_e8232_d_n11, assign13920_e8232_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13920_e8227: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign13920_e8229: f64 = (assign13920_e8227 + locals.var_tmf2);
        let assign13920_e8230: f64 = (assign13920_e8229).sqrt();
        (assign13920_e8230, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign13920_e8230)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign13920_e8230)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign13920_e8232;
        locals.var_tmf2_dn0 = assign13920_e8232_d_n0;
        locals.var_tmf2_dn2 = assign13920_e8232_d_n2;
        locals.var_tmf2_dn4 = assign13920_e8232_d_n4;
        locals.var_tmf2_dn5 = assign13920_e8232_d_n5;
        locals.var_tmf2_dn6 = assign13920_e8232_d_n6;
        locals.var_tmf2_dn7 = assign13920_e8232_d_n7;
        locals.var_tmf2_dn8 = assign13920_e8232_d_n8;
        locals.var_tmf2_dn9 = assign13920_e8232_d_n9;
        locals.var_tmf2_dn10 = assign13920_e8232_d_n10;
        locals.var_tmf2_dn11 = assign13920_e8232_d_n11;
        locals.var_tmf2_dn14 = assign13920_e8232_d_n14;

        let (assign13930_e8242, assign13930_e8242_d_n0, assign13930_e8242_d_n2, assign13930_e8242_d_n4, assign13930_e8242_d_n5, assign13930_e8242_d_n6, assign13930_e8242_d_n7, assign13930_e8242_d_n8, assign13930_e8242_d_n9, assign13930_e8242_d_n10, assign13930_e8242_d_n11, assign13930_e8242_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13930_e8238: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign13930_e8239: f64 = (1.0 + assign13930_e8238);
        let assign13930_e8240: f64 = (0.5 * assign13930_e8239);
        (assign13930_e8240, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign13930_e8242;
        locals.var_t0_dn0 = assign13930_e8242_d_n0;
        locals.var_t0_dn2 = assign13930_e8242_d_n2;
        locals.var_t0_dn4 = assign13930_e8242_d_n4;
        locals.var_t0_dn5 = assign13930_e8242_d_n5;
        locals.var_t0_dn6 = assign13930_e8242_d_n6;
        locals.var_t0_dn7 = assign13930_e8242_d_n7;
        locals.var_t0_dn8 = assign13930_e8242_d_n8;
        locals.var_t0_dn9 = assign13930_e8242_d_n9;
        locals.var_t0_dn10 = assign13930_e8242_d_n10;
        locals.var_t0_dn11 = assign13930_e8242_d_n11;
        locals.var_t0_dn14 = assign13930_e8242_d_n14;

    }

    pub(super) fn stamp_transient_block_25(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13940_e8252, assign13940_e8252_d_n0, assign13940_e8252_d_n2, assign13940_e8252_d_n4, assign13940_e8252_d_n5, assign13940_e8252_d_n6, assign13940_e8252_d_n7, assign13940_e8252_d_n8, assign13940_e8252_d_n9, assign13940_e8252_d_n10, assign13940_e8252_d_n11, assign13940_e8252_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13940_e8248: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign13940_e8249: f64 = (0.5 * assign13940_e8248);
        let assign13940_e8250: f64 = (1.0 - assign13940_e8249);
        (assign13940_e8250, (-(0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (-(0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (-(0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (-(0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (-(0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (-(0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (-(0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (-(0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (-(0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (-(0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (-(0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_powratio, locals.var_powratio_dn0, locals.var_powratio_dn2, locals.var_powratio_dn4, locals.var_powratio_dn5, locals.var_powratio_dn6, locals.var_powratio_dn7, locals.var_powratio_dn8, locals.var_powratio_dn9, locals.var_powratio_dn10, locals.var_powratio_dn11, locals.var_powratio_dn14,)
    }
};
        locals.var_powratio = assign13940_e8252;
        locals.var_powratio_dn0 = assign13940_e8252_d_n0;
        locals.var_powratio_dn2 = assign13940_e8252_d_n2;
        locals.var_powratio_dn4 = assign13940_e8252_d_n4;
        locals.var_powratio_dn5 = assign13940_e8252_d_n5;
        locals.var_powratio_dn6 = assign13940_e8252_d_n6;
        locals.var_powratio_dn7 = assign13940_e8252_d_n7;
        locals.var_powratio_dn8 = assign13940_e8252_d_n8;
        locals.var_powratio_dn9 = assign13940_e8252_d_n9;
        locals.var_powratio_dn10 = assign13940_e8252_d_n10;
        locals.var_powratio_dn11 = assign13940_e8252_d_n11;
        locals.var_powratio_dn14 = assign13940_e8252_d_n14;

        let (assign13950_e8263, assign13950_e8263_d_n0, assign13950_e8263_d_n2, assign13950_e8263_d_n4, assign13950_e8263_d_n5, assign13950_e8263_d_n6, assign13950_e8263_d_n7, assign13950_e8263_d_n8, assign13950_e8263_d_n9, assign13950_e8263_d_n10, assign13950_e8263_d_n11, assign13950_e8263_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13950_e8256: f64 = (2.0 * locals.var_beta_inv);
        let assign13950_e8259: f64 = (locals.var_nsub / locals.var_nin);
        let assign13950_e8260: f64 = (assign13950_e8259).ln();
        let assign13950_e8261: f64 = (assign13950_e8256 * assign13950_e8260);
        (assign13950_e8261, (((2.0 * locals.var_beta_inv_dn0) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn0 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn0)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn2) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn2 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn2)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn4) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn4 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn4)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn5) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn5 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn5)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn6) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn6 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn6)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn7) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn7 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn7)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn8) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn8 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn8)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn9) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn9 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn9)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn10) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn10 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn10)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn11) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn11 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn11)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))), (((2.0 * locals.var_beta_inv_dn14) * assign13950_e8260) + (assign13950_e8256 * ((((locals.var_nsub_dn14 * locals.var_nin) - (locals.var_nsub * locals.var_nin_dn14)) / (locals.var_nin * locals.var_nin)) / assign13950_e8259))),)
    } else {
        (locals.var_pb2, locals.var_pb2_dn0, locals.var_pb2_dn2, locals.var_pb2_dn4, locals.var_pb2_dn5, locals.var_pb2_dn6, locals.var_pb2_dn7, locals.var_pb2_dn8, locals.var_pb2_dn9, locals.var_pb2_dn10, locals.var_pb2_dn11, locals.var_pb2_dn14,)
    }
};
        locals.var_pb2 = assign13950_e8263;
        locals.var_pb2_dn0 = assign13950_e8263_d_n0;
        locals.var_pb2_dn2 = assign13950_e8263_d_n2;
        locals.var_pb2_dn4 = assign13950_e8263_d_n4;
        locals.var_pb2_dn5 = assign13950_e8263_d_n5;
        locals.var_pb2_dn6 = assign13950_e8263_d_n6;
        locals.var_pb2_dn7 = assign13950_e8263_d_n7;
        locals.var_pb2_dn8 = assign13950_e8263_d_n8;
        locals.var_pb2_dn9 = assign13950_e8263_d_n9;
        locals.var_pb2_dn10 = assign13950_e8263_d_n10;
        locals.var_pb2_dn11 = assign13950_e8263_d_n11;
        locals.var_pb2_dn14 = assign13950_e8263_d_n14;

        let (assign13960_e8271, assign13960_e8271_d_n0, assign13960_e8271_d_n2, assign13960_e8271_d_n4, assign13960_e8271_d_n5, assign13960_e8271_d_n6, assign13960_e8271_d_n7, assign13960_e8271_d_n8, assign13960_e8271_d_n9, assign13960_e8271_d_n10, assign13960_e8271_d_n11, assign13960_e8271_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13960_e8267: f64 = (2.0 * 1.034943e-10);
        let assign13960_e8269: f64 = (assign13960_e8267 / 1.6021918e-19);
        (assign13960_e8269, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign13960_e8271;
        locals.var_t1_dn0 = assign13960_e8271_d_n0;
        locals.var_t1_dn2 = assign13960_e8271_d_n2;
        locals.var_t1_dn4 = assign13960_e8271_d_n4;
        locals.var_t1_dn5 = assign13960_e8271_d_n5;
        locals.var_t1_dn6 = assign13960_e8271_d_n6;
        locals.var_t1_dn7 = assign13960_e8271_d_n7;
        locals.var_t1_dn8 = assign13960_e8271_d_n8;
        locals.var_t1_dn9 = assign13960_e8271_d_n9;
        locals.var_t1_dn10 = assign13960_e8271_d_n10;
        locals.var_t1_dn11 = assign13960_e8271_d_n11;
        locals.var_t1_dn14 = assign13960_e8271_d_n14;

        let (assign13970_e8278, assign13970_e8278_d_n0, assign13970_e8278_d_n2, assign13970_e8278_d_n4, assign13970_e8278_d_n5, assign13970_e8278_d_n6, assign13970_e8278_d_n7, assign13970_e8278_d_n8, assign13970_e8278_d_n9, assign13970_e8278_d_n10, assign13970_e8278_d_n11, assign13970_e8278_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13970_e8275: f64 = (locals.var_t1 / locals.var_nsub);
        let assign13970_e8276: f64 = (assign13970_e8275).sqrt();
        (assign13970_e8276, ((((locals.var_t1_dn0 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)), ((((locals.var_t1_dn2 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)), ((((locals.var_t1_dn4 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn4)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)), ((((locals.var_t1_dn5 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn5)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)), ((((locals.var_t1_dn6 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)), ((((locals.var_t1_dn7 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)), ((((locals.var_t1_dn8 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn8)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)), ((((locals.var_t1_dn9 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn9)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)), ((((locals.var_t1_dn10 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)), ((((locals.var_t1_dn11 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn11)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)), ((((locals.var_t1_dn14 * locals.var_nsub) - (locals.var_t1 * locals.var_nsub_dn14)) / (locals.var_nsub * locals.var_nsub)) / (2.0 * assign13970_e8276)),)
    } else {
        (locals.var_wdpl, locals.var_wdpl_dn0, locals.var_wdpl_dn2, locals.var_wdpl_dn4, locals.var_wdpl_dn5, locals.var_wdpl_dn6, locals.var_wdpl_dn7, locals.var_wdpl_dn8, locals.var_wdpl_dn9, locals.var_wdpl_dn10, locals.var_wdpl_dn11, locals.var_wdpl_dn14,)
    }
};
        locals.var_wdpl = assign13970_e8278;
        locals.var_wdpl_dn0 = assign13970_e8278_d_n0;
        locals.var_wdpl_dn2 = assign13970_e8278_d_n2;
        locals.var_wdpl_dn4 = assign13970_e8278_d_n4;
        locals.var_wdpl_dn5 = assign13970_e8278_d_n5;
        locals.var_wdpl_dn6 = assign13970_e8278_d_n6;
        locals.var_wdpl_dn7 = assign13970_e8278_d_n7;
        locals.var_wdpl_dn8 = assign13970_e8278_d_n8;
        locals.var_wdpl_dn9 = assign13970_e8278_d_n9;
        locals.var_wdpl_dn10 = assign13970_e8278_d_n10;
        locals.var_wdpl_dn11 = assign13970_e8278_d_n11;
        locals.var_wdpl_dn14 = assign13970_e8278_d_n14;

        let (assign13980_e8285, assign13980_e8285_d_n0, assign13980_e8285_d_n2, assign13980_e8285_d_n4, assign13980_e8285_d_n5, assign13980_e8285_d_n6, assign13980_e8285_d_n7, assign13980_e8285_d_n8, assign13980_e8285_d_n9, assign13980_e8285_d_n10, assign13980_e8285_d_n11, assign13980_e8285_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign13980_e8282: f64 = (locals.var_t1 / locals.var_ef_nsubp);
        let assign13980_e8283: f64 = (assign13980_e8282).sqrt();
        (assign13980_e8283, ((((locals.var_t1_dn0 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn0)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)), ((((locals.var_t1_dn2 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn2)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)), ((((locals.var_t1_dn4 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn4)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)), ((((locals.var_t1_dn5 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn5)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)), ((((locals.var_t1_dn6 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn6)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)), ((((locals.var_t1_dn7 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn7)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)), ((((locals.var_t1_dn8 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn8)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)), ((((locals.var_t1_dn9 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn9)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)), ((((locals.var_t1_dn10 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn10)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)), ((((locals.var_t1_dn11 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn11)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)), ((((locals.var_t1_dn14 * locals.var_ef_nsubp) - (locals.var_t1 * locals.var_ef_nsubp_dn14)) / (locals.var_ef_nsubp * locals.var_ef_nsubp)) / (2.0 * assign13980_e8283)),)
    } else {
        (locals.var_wdplp, locals.var_wdplp_dn0, locals.var_wdplp_dn2, locals.var_wdplp_dn4, locals.var_wdplp_dn5, locals.var_wdplp_dn6, locals.var_wdplp_dn7, locals.var_wdplp_dn8, locals.var_wdplp_dn9, locals.var_wdplp_dn10, locals.var_wdplp_dn11, locals.var_wdplp_dn14,)
    }
};
        locals.var_wdplp = assign13980_e8285;
        locals.var_wdplp_dn0 = assign13980_e8285_d_n0;
        locals.var_wdplp_dn2 = assign13980_e8285_d_n2;
        locals.var_wdplp_dn4 = assign13980_e8285_d_n4;
        locals.var_wdplp_dn5 = assign13980_e8285_d_n5;
        locals.var_wdplp_dn6 = assign13980_e8285_d_n6;
        locals.var_wdplp_dn7 = assign13980_e8285_d_n7;
        locals.var_wdplp_dn8 = assign13980_e8285_d_n8;
        locals.var_wdplp_dn9 = assign13980_e8285_d_n9;
        locals.var_wdplp_dn10 = assign13980_e8285_d_n10;
        locals.var_wdplp_dn11 = assign13980_e8285_d_n11;
        locals.var_wdplp_dn14 = assign13980_e8285_d_n14;

        let assign13990_e8288: f64 = if locals.var_uc_codep == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard309 = assign13990_e8288;

        let (assign14000_e8303, assign14000_e8303_d_n0, assign14000_e8303_d_n2, assign14000_e8303_d_n4, assign14000_e8303_d_n5, assign14000_e8303_d_n6, assign14000_e8303_d_n7, assign14000_e8303_d_n8, assign14000_e8303_d_n9, assign14000_e8303_d_n10, assign14000_e8303_d_n11, assign14000_e8303_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard309 != 0.0)) {
        let assign14000_e8294: f64 = (2.0 * 1.034943e-10);
        let assign14000_e8296: f64 = (assign14000_e8294 * 1.6021918e-19);
        let assign14000_e8298: f64 = (assign14000_e8296 * locals.var_nsub);
        let assign14000_e8300: f64 = (assign14000_e8298 * locals.var_beta_inv);
        let assign14000_e8301: f64 = (assign14000_e8300).sqrt();
        (assign14000_e8301, ((((assign14000_e8296 * locals.var_nsub_dn0) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn0)) / (2.0 * assign14000_e8301)), ((((assign14000_e8296 * locals.var_nsub_dn2) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn2)) / (2.0 * assign14000_e8301)), ((((assign14000_e8296 * locals.var_nsub_dn4) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn4)) / (2.0 * assign14000_e8301)), ((((assign14000_e8296 * locals.var_nsub_dn5) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn5)) / (2.0 * assign14000_e8301)), ((((assign14000_e8296 * locals.var_nsub_dn6) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn6)) / (2.0 * assign14000_e8301)), ((((assign14000_e8296 * locals.var_nsub_dn7) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn7)) / (2.0 * assign14000_e8301)), ((((assign14000_e8296 * locals.var_nsub_dn8) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn8)) / (2.0 * assign14000_e8301)), ((((assign14000_e8296 * locals.var_nsub_dn9) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn9)) / (2.0 * assign14000_e8301)), ((((assign14000_e8296 * locals.var_nsub_dn10) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn10)) / (2.0 * assign14000_e8301)), ((((assign14000_e8296 * locals.var_nsub_dn11) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn11)) / (2.0 * assign14000_e8301)), ((((assign14000_e8296 * locals.var_nsub_dn14) * locals.var_beta_inv) + (assign14000_e8298 * locals.var_beta_inv_dn14)) / (2.0 * assign14000_e8301)),)
    } else {
        (locals.var_cnst0, locals.var_cnst0_dn0, locals.var_cnst0_dn2, locals.var_cnst0_dn4, locals.var_cnst0_dn5, locals.var_cnst0_dn6, locals.var_cnst0_dn7, locals.var_cnst0_dn8, locals.var_cnst0_dn9, locals.var_cnst0_dn10, locals.var_cnst0_dn11, locals.var_cnst0_dn14,)
    }
};
        locals.var_cnst0 = assign14000_e8303;
        locals.var_cnst0_dn0 = assign14000_e8303_d_n0;
        locals.var_cnst0_dn2 = assign14000_e8303_d_n2;
        locals.var_cnst0_dn4 = assign14000_e8303_d_n4;
        locals.var_cnst0_dn5 = assign14000_e8303_d_n5;
        locals.var_cnst0_dn6 = assign14000_e8303_d_n6;
        locals.var_cnst0_dn7 = assign14000_e8303_d_n7;
        locals.var_cnst0_dn8 = assign14000_e8303_d_n8;
        locals.var_cnst0_dn9 = assign14000_e8303_d_n9;
        locals.var_cnst0_dn10 = assign14000_e8303_d_n10;
        locals.var_cnst0_dn11 = assign14000_e8303_d_n11;
        locals.var_cnst0_dn14 = assign14000_e8303_d_n14;

        let (assign14010_e8311, assign14010_e8311_d_n0, assign14010_e8311_d_n2, assign14010_e8311_d_n4, assign14010_e8311_d_n5, assign14010_e8311_d_n6, assign14010_e8311_d_n7, assign14010_e8311_d_n8, assign14010_e8311_d_n9, assign14010_e8311_d_n10, assign14010_e8311_d_n11, assign14010_e8311_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard309 != 0.0)) {
        let assign14010_e8309: f64 = (locals.var_nin / locals.var_nsub);
        (assign14010_e8309, (((locals.var_nin_dn0 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn0)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn2 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn2)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn4 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn4)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn5 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn5)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn6 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn6)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn7 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn7)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn8 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn8)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn9 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn9)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn10 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn10)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn11 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn11)) / (locals.var_nsub * locals.var_nsub)), (((locals.var_nin_dn14 * locals.var_nsub) - (locals.var_nin * locals.var_nsub_dn14)) / (locals.var_nsub * locals.var_nsub)),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign14010_e8311;
        locals.var_t1_dn0 = assign14010_e8311_d_n0;
        locals.var_t1_dn2 = assign14010_e8311_d_n2;
        locals.var_t1_dn4 = assign14010_e8311_d_n4;
        locals.var_t1_dn5 = assign14010_e8311_d_n5;
        locals.var_t1_dn6 = assign14010_e8311_d_n6;
        locals.var_t1_dn7 = assign14010_e8311_d_n7;
        locals.var_t1_dn8 = assign14010_e8311_d_n8;
        locals.var_t1_dn9 = assign14010_e8311_d_n9;
        locals.var_t1_dn10 = assign14010_e8311_d_n10;
        locals.var_t1_dn11 = assign14010_e8311_d_n11;
        locals.var_t1_dn14 = assign14010_e8311_d_n14;

        let (assign14020_e8319, assign14020_e8319_d_n0, assign14020_e8319_d_n2, assign14020_e8319_d_n4, assign14020_e8319_d_n5, assign14020_e8319_d_n6, assign14020_e8319_d_n7, assign14020_e8319_d_n8, assign14020_e8319_d_n9, assign14020_e8319_d_n10, assign14020_e8319_d_n11, assign14020_e8319_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard309 != 0.0)) {
        let assign14020_e8317: f64 = (locals.var_t1 * locals.var_t1);
        (assign14020_e8317, ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0)), ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2)), ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4)), ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5)), ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6)), ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7)), ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8)), ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9)), ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10)), ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11)), ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14)),)
    } else {
        (locals.var_cnst1, locals.var_cnst1_dn0, locals.var_cnst1_dn2, locals.var_cnst1_dn4, locals.var_cnst1_dn5, locals.var_cnst1_dn6, locals.var_cnst1_dn7, locals.var_cnst1_dn8, locals.var_cnst1_dn9, locals.var_cnst1_dn10, locals.var_cnst1_dn11, locals.var_cnst1_dn14,)
    }
};
        locals.var_cnst1 = assign14020_e8319;
        locals.var_cnst1_dn0 = assign14020_e8319_d_n0;
        locals.var_cnst1_dn2 = assign14020_e8319_d_n2;
        locals.var_cnst1_dn4 = assign14020_e8319_d_n4;
        locals.var_cnst1_dn5 = assign14020_e8319_d_n5;
        locals.var_cnst1_dn6 = assign14020_e8319_d_n6;
        locals.var_cnst1_dn7 = assign14020_e8319_d_n7;
        locals.var_cnst1_dn8 = assign14020_e8319_d_n8;
        locals.var_cnst1_dn9 = assign14020_e8319_d_n9;
        locals.var_cnst1_dn10 = assign14020_e8319_d_n10;
        locals.var_cnst1_dn11 = assign14020_e8319_d_n11;
        locals.var_cnst1_dn14 = assign14020_e8319_d_n14;

        let assign14030_e8322: f64 = if locals.var_uc_codep == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard310 = assign14030_e8322;

        let assign14040_e8325: f64 = if locals.var_uc_nover != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard311 = assign14040_e8325;

        let (assign14050_e8338, assign14050_e8338_d_n0, assign14050_e8338_d_n2, assign14050_e8338_d_n4, assign14050_e8338_d_n5, assign14050_e8338_d_n6, assign14050_e8338_d_n7, assign14050_e8338_d_n8, assign14050_e8338_d_n9, assign14050_e8338_d_n10, assign14050_e8338_d_n11, assign14050_e8338_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard310 != 0.0)) && (locals.var_guard311 != 0.0)) {
        let assign14050_e8334: f64 = (locals.var_uc_nover / locals.var_nsub);
        let assign14050_e8335: f64 = (assign14050_e8334).sqrt();
        let assign14050_e8336: f64 = (locals.var_cnst0 * assign14050_e8335);
        (assign14050_e8336, ((locals.var_cnst0_dn0 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))), ((locals.var_cnst0_dn2 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))), ((locals.var_cnst0_dn4 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn4) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))), ((locals.var_cnst0_dn5 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn5) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))), ((locals.var_cnst0_dn6 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))), ((locals.var_cnst0_dn7 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))), ((locals.var_cnst0_dn8 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn8) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))), ((locals.var_cnst0_dn9 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn9) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))), ((locals.var_cnst0_dn10 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))), ((locals.var_cnst0_dn11 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn11) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))), ((locals.var_cnst0_dn14 * assign14050_e8335) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_nsub_dn14) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14050_e8335)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn14,)
    }
};
        locals.var_cnst0over = assign14050_e8338;
        locals.var_cnst0over_dn0 = assign14050_e8338_d_n0;
        locals.var_cnst0over_dn2 = assign14050_e8338_d_n2;
        locals.var_cnst0over_dn4 = assign14050_e8338_d_n4;
        locals.var_cnst0over_dn5 = assign14050_e8338_d_n5;
        locals.var_cnst0over_dn6 = assign14050_e8338_d_n6;
        locals.var_cnst0over_dn7 = assign14050_e8338_d_n7;
        locals.var_cnst0over_dn8 = assign14050_e8338_d_n8;
        locals.var_cnst0over_dn9 = assign14050_e8338_d_n9;
        locals.var_cnst0over_dn10 = assign14050_e8338_d_n10;
        locals.var_cnst0over_dn11 = assign14050_e8338_d_n11;
        locals.var_cnst0over_dn14 = assign14050_e8338_d_n14;

        let assign14060_e8341: f64 = if locals.var_uc_novers != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard312 = assign14060_e8341;

        let (assign14070_e8354, assign14070_e8354_d_n0, assign14070_e8354_d_n2, assign14070_e8354_d_n4, assign14070_e8354_d_n5, assign14070_e8354_d_n6, assign14070_e8354_d_n7, assign14070_e8354_d_n8, assign14070_e8354_d_n9, assign14070_e8354_d_n10, assign14070_e8354_d_n11, assign14070_e8354_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard310 != 0.0)) && (locals.var_guard312 != 0.0)) {
        let assign14070_e8350: f64 = (locals.var_uc_novers / locals.var_nsub);
        let assign14070_e8351: f64 = (assign14070_e8350).sqrt();
        let assign14070_e8352: f64 = (locals.var_cnst0 * assign14070_e8351);
        (assign14070_e8352, ((locals.var_cnst0_dn0 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn0) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))), ((locals.var_cnst0_dn2 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn2) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))), ((locals.var_cnst0_dn4 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn4) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))), ((locals.var_cnst0_dn5 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn5) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))), ((locals.var_cnst0_dn6 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn6) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))), ((locals.var_cnst0_dn7 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn7) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))), ((locals.var_cnst0_dn8 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn8) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))), ((locals.var_cnst0_dn9 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn9) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))), ((locals.var_cnst0_dn10 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn10) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))), ((locals.var_cnst0_dn11 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn11) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))), ((locals.var_cnst0_dn14 * assign14070_e8351) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_nsub_dn14) / (locals.var_nsub * locals.var_nsub))) / (2.0 * assign14070_e8351)))),)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn11, locals.var_cnst0overs_dn14,)
    }
};
        locals.var_cnst0overs = assign14070_e8354;
        locals.var_cnst0overs_dn0 = assign14070_e8354_d_n0;
        locals.var_cnst0overs_dn2 = assign14070_e8354_d_n2;
        locals.var_cnst0overs_dn4 = assign14070_e8354_d_n4;
        locals.var_cnst0overs_dn5 = assign14070_e8354_d_n5;
        locals.var_cnst0overs_dn6 = assign14070_e8354_d_n6;
        locals.var_cnst0overs_dn7 = assign14070_e8354_d_n7;
        locals.var_cnst0overs_dn8 = assign14070_e8354_d_n8;
        locals.var_cnst0overs_dn9 = assign14070_e8354_d_n9;
        locals.var_cnst0overs_dn10 = assign14070_e8354_d_n10;
        locals.var_cnst0overs_dn11 = assign14070_e8354_d_n11;
        locals.var_cnst0overs_dn14 = assign14070_e8354_d_n14;

        let assign14080_e8357: f64 = if locals.var_uc_nover != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard313 = assign14080_e8357;

        let (assign14090_e8371, assign14090_e8371_d_n0, assign14090_e8371_d_n2, assign14090_e8371_d_n4, assign14090_e8371_d_n5, assign14090_e8371_d_n6, assign14090_e8371_d_n7, assign14090_e8371_d_n8, assign14090_e8371_d_n9, assign14090_e8371_d_n10, assign14090_e8371_d_n11, assign14090_e8371_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard310 == 0.0)) && (locals.var_guard313 != 0.0)) {
        let assign14090_e8367: f64 = (locals.var_uc_nover / locals.var_uc_ndepm);
        let assign14090_e8368: f64 = (assign14090_e8367).sqrt();
        let assign14090_e8369: f64 = (locals.var_cnst0 * assign14090_e8368);
        (assign14090_e8369, ((locals.var_cnst0_dn0 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn0) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))), ((locals.var_cnst0_dn2 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn2) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))), ((locals.var_cnst0_dn4 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn4) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))), ((locals.var_cnst0_dn5 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn5) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))), ((locals.var_cnst0_dn6 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn6) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))), ((locals.var_cnst0_dn7 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn7) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))), ((locals.var_cnst0_dn8 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn8) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))), ((locals.var_cnst0_dn9 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn9) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))), ((locals.var_cnst0_dn10 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn10) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))), ((locals.var_cnst0_dn11 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn11) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))), ((locals.var_cnst0_dn14 * assign14090_e8368) + (locals.var_cnst0 * ((-((locals.var_uc_nover * locals.var_uc_ndepm_dn14) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14090_e8368)))),)
    } else {
        (locals.var_cnst0over, locals.var_cnst0over_dn0, locals.var_cnst0over_dn2, locals.var_cnst0over_dn4, locals.var_cnst0over_dn5, locals.var_cnst0over_dn6, locals.var_cnst0over_dn7, locals.var_cnst0over_dn8, locals.var_cnst0over_dn9, locals.var_cnst0over_dn10, locals.var_cnst0over_dn11, locals.var_cnst0over_dn14,)
    }
};
        locals.var_cnst0over = assign14090_e8371;
        locals.var_cnst0over_dn0 = assign14090_e8371_d_n0;
        locals.var_cnst0over_dn2 = assign14090_e8371_d_n2;
        locals.var_cnst0over_dn4 = assign14090_e8371_d_n4;
        locals.var_cnst0over_dn5 = assign14090_e8371_d_n5;
        locals.var_cnst0over_dn6 = assign14090_e8371_d_n6;
        locals.var_cnst0over_dn7 = assign14090_e8371_d_n7;
        locals.var_cnst0over_dn8 = assign14090_e8371_d_n8;
        locals.var_cnst0over_dn9 = assign14090_e8371_d_n9;
        locals.var_cnst0over_dn10 = assign14090_e8371_d_n10;
        locals.var_cnst0over_dn11 = assign14090_e8371_d_n11;
        locals.var_cnst0over_dn14 = assign14090_e8371_d_n14;

        let assign14100_e8374: f64 = if locals.var_uc_novers != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard314 = assign14100_e8374;

        let (assign14110_e8388, assign14110_e8388_d_n0, assign14110_e8388_d_n2, assign14110_e8388_d_n4, assign14110_e8388_d_n5, assign14110_e8388_d_n6, assign14110_e8388_d_n7, assign14110_e8388_d_n8, assign14110_e8388_d_n9, assign14110_e8388_d_n10, assign14110_e8388_d_n11, assign14110_e8388_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard310 == 0.0)) && (locals.var_guard314 != 0.0)) {
        let assign14110_e8384: f64 = (locals.var_uc_novers / locals.var_uc_ndepm);
        let assign14110_e8385: f64 = (assign14110_e8384).sqrt();
        let assign14110_e8386: f64 = (locals.var_cnst0 * assign14110_e8385);
        (assign14110_e8386, ((locals.var_cnst0_dn0 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn0) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))), ((locals.var_cnst0_dn2 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn2) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))), ((locals.var_cnst0_dn4 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn4) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))), ((locals.var_cnst0_dn5 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn5) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))), ((locals.var_cnst0_dn6 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn6) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))), ((locals.var_cnst0_dn7 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn7) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))), ((locals.var_cnst0_dn8 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn8) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))), ((locals.var_cnst0_dn9 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn9) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))), ((locals.var_cnst0_dn10 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn10) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))), ((locals.var_cnst0_dn11 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn11) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))), ((locals.var_cnst0_dn14 * assign14110_e8385) + (locals.var_cnst0 * ((-((locals.var_uc_novers * locals.var_uc_ndepm_dn14) / (locals.var_uc_ndepm * locals.var_uc_ndepm))) / (2.0 * assign14110_e8385)))),)
    } else {
        (locals.var_cnst0overs, locals.var_cnst0overs_dn0, locals.var_cnst0overs_dn2, locals.var_cnst0overs_dn4, locals.var_cnst0overs_dn5, locals.var_cnst0overs_dn6, locals.var_cnst0overs_dn7, locals.var_cnst0overs_dn8, locals.var_cnst0overs_dn9, locals.var_cnst0overs_dn10, locals.var_cnst0overs_dn11, locals.var_cnst0overs_dn14,)
    }
};
        locals.var_cnst0overs = assign14110_e8388;
        locals.var_cnst0overs_dn0 = assign14110_e8388_d_n0;
        locals.var_cnst0overs_dn2 = assign14110_e8388_d_n2;
        locals.var_cnst0overs_dn4 = assign14110_e8388_d_n4;
        locals.var_cnst0overs_dn5 = assign14110_e8388_d_n5;
        locals.var_cnst0overs_dn6 = assign14110_e8388_d_n6;
        locals.var_cnst0overs_dn7 = assign14110_e8388_d_n7;
        locals.var_cnst0overs_dn8 = assign14110_e8388_d_n8;
        locals.var_cnst0overs_dn9 = assign14110_e8388_d_n9;
        locals.var_cnst0overs_dn10 = assign14110_e8388_d_n10;
        locals.var_cnst0overs_dn11 = assign14110_e8388_d_n11;
        locals.var_cnst0overs_dn14 = assign14110_e8388_d_n14;

        let assign14120_e8391: f64 = if locals.var_uc_cordrift == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard315 = assign14120_e8391;

        let assign14130_e8394: f64 = if locals.var_uc_rd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard316 = assign14130_e8394;

        let (assign14140_e8418, assign14140_e8418_d_n0, assign14140_e8418_d_n2, assign14140_e8418_d_n4, assign14140_e8418_d_n5, assign14140_e8418_d_n6, assign14140_e8418_d_n7, assign14140_e8418_d_n8, assign14140_e8418_d_n9, assign14140_e8418_d_n10, assign14140_e8418_d_n11, assign14140_e8418_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) {
        let assign14140_e8403: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign14140_e8405: f64 = (assign14140_e8403 * 1000000.0);
        let assign14140_e8407: f64 = (assign14140_e8405 + locals.var_uc_rdict1);
        let assign14140_e8408: f64 = (locals.var_rdtemp0 * assign14140_e8407);
        let assign14140_e8411: f64 = (p.p68 * p.p100);
        let assign14140_e8413: f64 = (assign14140_e8411 * 1000000.0);
        let assign14140_e8415: f64 = (assign14140_e8413 + p.p101);
        let assign14140_e8416: f64 = (assign14140_e8408 * assign14140_e8415);
        (assign14140_e8416, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign14140_e8418;
        locals.var_t2_dn0 = assign14140_e8418_d_n0;
        locals.var_t2_dn2 = assign14140_e8418_d_n2;
        locals.var_t2_dn4 = assign14140_e8418_d_n4;
        locals.var_t2_dn5 = assign14140_e8418_d_n5;
        locals.var_t2_dn6 = assign14140_e8418_d_n6;
        locals.var_t2_dn7 = assign14140_e8418_d_n7;
        locals.var_t2_dn8 = assign14140_e8418_d_n8;
        locals.var_t2_dn9 = assign14140_e8418_d_n9;
        locals.var_t2_dn10 = assign14140_e8418_d_n10;
        locals.var_t2_dn11 = assign14140_e8418_d_n11;
        locals.var_t2_dn14 = assign14140_e8418_d_n14;

        let assign14150_e8421: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard317 = assign14150_e8421;

        let (assign14160_e8441, assign14160_e8441_d_n0, assign14160_e8441_d_n2, assign14160_e8441_d_n4, assign14160_e8441_d_n5, assign14160_e8441_d_n6, assign14160_e8441_d_n7, assign14160_e8441_d_n8, assign14160_e8441_d_n9, assign14160_e8441_d_n10, assign14160_e8441_d_n11, assign14160_e8441_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14160_e8432: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff0);
        let assign14160_e8433: f64 = (locals.var_uc_rd + assign14160_e8432);
        let assign14160_e8436: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff0_2);
        let assign14160_e8437: f64 = (assign14160_e8433 + assign14160_e8436);
        let assign14160_e8439: f64 = (assign14160_e8437 * locals.var_t2);
        (assign14160_e8439, ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn11) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn11)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn11)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn14) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn14)) * locals.var_t2) + (assign14160_e8437 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign14160_e8441;
        locals.var_rde_dn0 = assign14160_e8441_d_n0;
        locals.var_rde_dn2 = assign14160_e8441_d_n2;
        locals.var_rde_dn4 = assign14160_e8441_d_n4;
        locals.var_rde_dn5 = assign14160_e8441_d_n5;
        locals.var_rde_dn6 = assign14160_e8441_d_n6;
        locals.var_rde_dn7 = assign14160_e8441_d_n7;
        locals.var_rde_dn8 = assign14160_e8441_d_n8;
        locals.var_rde_dn9 = assign14160_e8441_d_n9;
        locals.var_rde_dn10 = assign14160_e8441_d_n10;
        locals.var_rde_dn11 = assign14160_e8441_d_n11;
        locals.var_rde_dn14 = assign14160_e8441_d_n14;

        let (assign14170_e8459, assign14170_e8459_d_n0, assign14170_e8459_d_n2, assign14170_e8459_d_n4, assign14170_e8459_d_n5, assign14170_e8459_d_n6, assign14170_e8459_d_n7, assign14170_e8459_d_n8, assign14170_e8459_d_n9, assign14170_e8459_d_n10, assign14170_e8459_d_n11, assign14170_e8459_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14170_e8452: f64 = (0.005 * locals.var_uc_rd);
        let assign14170_e8453: f64 = (locals.var_rde - assign14170_e8452);
        let assign14170_e8456: f64 = (0.01 * locals.var_uc_rd);
        let assign14170_e8457: f64 = (assign14170_e8453 - assign14170_e8456);
        (assign14170_e8457, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign14170_e8459;
        locals.var_tmf1_dn0 = assign14170_e8459_d_n0;
        locals.var_tmf1_dn2 = assign14170_e8459_d_n2;
        locals.var_tmf1_dn4 = assign14170_e8459_d_n4;
        locals.var_tmf1_dn5 = assign14170_e8459_d_n5;
        locals.var_tmf1_dn6 = assign14170_e8459_d_n6;
        locals.var_tmf1_dn7 = assign14170_e8459_d_n7;
        locals.var_tmf1_dn8 = assign14170_e8459_d_n8;
        locals.var_tmf1_dn9 = assign14170_e8459_d_n9;
        locals.var_tmf1_dn10 = assign14170_e8459_d_n10;
        locals.var_tmf1_dn11 = assign14170_e8459_d_n11;
        locals.var_tmf1_dn14 = assign14170_e8459_d_n14;

        let (assign14180_e8477, assign14180_e8477_d_n0, assign14180_e8477_d_n2, assign14180_e8477_d_n4, assign14180_e8477_d_n5, assign14180_e8477_d_n6, assign14180_e8477_d_n7, assign14180_e8477_d_n8, assign14180_e8477_d_n9, assign14180_e8477_d_n10, assign14180_e8477_d_n11, assign14180_e8477_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14180_e8470: f64 = (0.005 * locals.var_uc_rd);
        let assign14180_e8471: f64 = (4.0 * assign14180_e8470);
        let assign14180_e8474: f64 = (0.01 * locals.var_uc_rd);
        let assign14180_e8475: f64 = (assign14180_e8471 * assign14180_e8474);
        (assign14180_e8475, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14180_e8477;
        locals.var_tmf2_dn0 = assign14180_e8477_d_n0;
        locals.var_tmf2_dn2 = assign14180_e8477_d_n2;
        locals.var_tmf2_dn4 = assign14180_e8477_d_n4;
        locals.var_tmf2_dn5 = assign14180_e8477_d_n5;
        locals.var_tmf2_dn6 = assign14180_e8477_d_n6;
        locals.var_tmf2_dn7 = assign14180_e8477_d_n7;
        locals.var_tmf2_dn8 = assign14180_e8477_d_n8;
        locals.var_tmf2_dn9 = assign14180_e8477_d_n9;
        locals.var_tmf2_dn10 = assign14180_e8477_d_n10;
        locals.var_tmf2_dn11 = assign14180_e8477_d_n11;
        locals.var_tmf2_dn14 = assign14180_e8477_d_n14;

        let (assign14190_e8493, assign14190_e8493_d_n0, assign14190_e8493_d_n2, assign14190_e8493_d_n4, assign14190_e8493_d_n5, assign14190_e8493_d_n6, assign14190_e8493_d_n7, assign14190_e8493_d_n8, assign14190_e8493_d_n9, assign14190_e8493_d_n10, assign14190_e8493_d_n11, assign14190_e8493_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let (assign14190_e8491, assign14190_e8491_d_n0, assign14190_e8491_d_n2, assign14190_e8491_d_n4, assign14190_e8491_d_n5, assign14190_e8491_d_n6, assign14190_e8491_d_n7, assign14190_e8491_d_n8, assign14190_e8491_d_n9, assign14190_e8491_d_n10, assign14190_e8491_d_n11, assign14190_e8491_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign14190_e8490: f64 = (-locals.var_tmf2);
                (assign14190_e8490, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign14190_e8491, assign14190_e8491_d_n0, assign14190_e8491_d_n2, assign14190_e8491_d_n4, assign14190_e8491_d_n5, assign14190_e8491_d_n6, assign14190_e8491_d_n7, assign14190_e8491_d_n8, assign14190_e8491_d_n9, assign14190_e8491_d_n10, assign14190_e8491_d_n11, assign14190_e8491_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14190_e8493;
        locals.var_tmf2_dn0 = assign14190_e8493_d_n0;
        locals.var_tmf2_dn2 = assign14190_e8493_d_n2;
        locals.var_tmf2_dn4 = assign14190_e8493_d_n4;
        locals.var_tmf2_dn5 = assign14190_e8493_d_n5;
        locals.var_tmf2_dn6 = assign14190_e8493_d_n6;
        locals.var_tmf2_dn7 = assign14190_e8493_d_n7;
        locals.var_tmf2_dn8 = assign14190_e8493_d_n8;
        locals.var_tmf2_dn9 = assign14190_e8493_d_n9;
        locals.var_tmf2_dn10 = assign14190_e8493_d_n10;
        locals.var_tmf2_dn11 = assign14190_e8493_d_n11;
        locals.var_tmf2_dn14 = assign14190_e8493_d_n14;

        let (assign14200_e8508, assign14200_e8508_d_n0, assign14200_e8508_d_n2, assign14200_e8508_d_n4, assign14200_e8508_d_n5, assign14200_e8508_d_n6, assign14200_e8508_d_n7, assign14200_e8508_d_n8, assign14200_e8508_d_n9, assign14200_e8508_d_n10, assign14200_e8508_d_n11, assign14200_e8508_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14200_e8503: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14200_e8505: f64 = (assign14200_e8503 + locals.var_tmf2);
        let assign14200_e8506: f64 = (assign14200_e8505).sqrt();
        (assign14200_e8506, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14200_e8506)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14200_e8506)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14200_e8506)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14200_e8506)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14200_e8506)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14200_e8506)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14200_e8506)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14200_e8506)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14200_e8506)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14200_e8506)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign14200_e8506)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14200_e8508;
        locals.var_tmf2_dn0 = assign14200_e8508_d_n0;
        locals.var_tmf2_dn2 = assign14200_e8508_d_n2;
        locals.var_tmf2_dn4 = assign14200_e8508_d_n4;
        locals.var_tmf2_dn5 = assign14200_e8508_d_n5;
        locals.var_tmf2_dn6 = assign14200_e8508_d_n6;
        locals.var_tmf2_dn7 = assign14200_e8508_d_n7;
        locals.var_tmf2_dn8 = assign14200_e8508_d_n8;
        locals.var_tmf2_dn9 = assign14200_e8508_d_n9;
        locals.var_tmf2_dn10 = assign14200_e8508_d_n10;
        locals.var_tmf2_dn11 = assign14200_e8508_d_n11;
        locals.var_tmf2_dn14 = assign14200_e8508_d_n14;

        let (assign14210_e8524, assign14210_e8524_d_n0, assign14210_e8524_d_n2, assign14210_e8524_d_n4, assign14210_e8524_d_n5, assign14210_e8524_d_n6, assign14210_e8524_d_n7, assign14210_e8524_d_n8, assign14210_e8524_d_n9, assign14210_e8524_d_n10, assign14210_e8524_d_n11, assign14210_e8524_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14210_e8520: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14210_e8521: f64 = (1.0 + assign14210_e8520);
        let assign14210_e8522: f64 = (0.5 * assign14210_e8521);
        (assign14210_e8522, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign14210_e8524;
        locals.var_t0_dn0 = assign14210_e8524_d_n0;
        locals.var_t0_dn2 = assign14210_e8524_d_n2;
        locals.var_t0_dn4 = assign14210_e8524_d_n4;
        locals.var_t0_dn5 = assign14210_e8524_d_n5;
        locals.var_t0_dn6 = assign14210_e8524_d_n6;
        locals.var_t0_dn7 = assign14210_e8524_d_n7;
        locals.var_t0_dn8 = assign14210_e8524_d_n8;
        locals.var_t0_dn9 = assign14210_e8524_d_n9;
        locals.var_t0_dn10 = assign14210_e8524_d_n10;
        locals.var_t0_dn11 = assign14210_e8524_d_n11;
        locals.var_t0_dn14 = assign14210_e8524_d_n14;

        let (assign14220_e8542, assign14220_e8542_d_n0, assign14220_e8542_d_n2, assign14220_e8542_d_n4, assign14220_e8542_d_n5, assign14220_e8542_d_n6, assign14220_e8542_d_n7, assign14220_e8542_d_n8, assign14220_e8542_d_n9, assign14220_e8542_d_n10, assign14220_e8542_d_n11, assign14220_e8542_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 != 0.0)) {
        let assign14220_e8534: f64 = (0.005 * locals.var_uc_rd);
        let assign14220_e8538: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14220_e8539: f64 = (0.5 * assign14220_e8538);
        let assign14220_e8540: f64 = (assign14220_e8534 + assign14220_e8539);
        (assign14220_e8540, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign14220_e8542;
        locals.var_rde_dn0 = assign14220_e8542_d_n0;
        locals.var_rde_dn2 = assign14220_e8542_d_n2;
        locals.var_rde_dn4 = assign14220_e8542_d_n4;
        locals.var_rde_dn5 = assign14220_e8542_d_n5;
        locals.var_rde_dn6 = assign14220_e8542_d_n6;
        locals.var_rde_dn7 = assign14220_e8542_d_n7;
        locals.var_rde_dn8 = assign14220_e8542_d_n8;
        locals.var_rde_dn9 = assign14220_e8542_d_n9;
        locals.var_rde_dn10 = assign14220_e8542_d_n10;
        locals.var_rde_dn11 = assign14220_e8542_d_n11;
        locals.var_rde_dn14 = assign14220_e8542_d_n14;

    }

    pub(super) fn stamp_transient_block_26(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14230_e8563, assign14230_e8563_d_n0, assign14230_e8563_d_n2, assign14230_e8563_d_n4, assign14230_e8563_d_n5, assign14230_e8563_d_n6, assign14230_e8563_d_n7, assign14230_e8563_d_n8, assign14230_e8563_d_n9, assign14230_e8563_d_n10, assign14230_e8563_d_n11, assign14230_e8563_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14230_e8554: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff);
        let assign14230_e8555: f64 = (locals.var_uc_rd + assign14230_e8554);
        let assign14230_e8558: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff_2);
        let assign14230_e8559: f64 = (assign14230_e8555 + assign14230_e8558);
        let assign14230_e8561: f64 = (assign14230_e8559 * locals.var_t2);
        (assign14230_e8561, ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn11) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn11)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn11)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn14) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn14)) * locals.var_t2) + (assign14230_e8559 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign14230_e8563;
        locals.var_rde_dn0 = assign14230_e8563_d_n0;
        locals.var_rde_dn2 = assign14230_e8563_d_n2;
        locals.var_rde_dn4 = assign14230_e8563_d_n4;
        locals.var_rde_dn5 = assign14230_e8563_d_n5;
        locals.var_rde_dn6 = assign14230_e8563_d_n6;
        locals.var_rde_dn7 = assign14230_e8563_d_n7;
        locals.var_rde_dn8 = assign14230_e8563_d_n8;
        locals.var_rde_dn9 = assign14230_e8563_d_n9;
        locals.var_rde_dn10 = assign14230_e8563_d_n10;
        locals.var_rde_dn11 = assign14230_e8563_d_n11;
        locals.var_rde_dn14 = assign14230_e8563_d_n14;

        let (assign14240_e8582, assign14240_e8582_d_n0, assign14240_e8582_d_n2, assign14240_e8582_d_n4, assign14240_e8582_d_n5, assign14240_e8582_d_n6, assign14240_e8582_d_n7, assign14240_e8582_d_n8, assign14240_e8582_d_n9, assign14240_e8582_d_n10, assign14240_e8582_d_n11, assign14240_e8582_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14240_e8575: f64 = (0.005 * locals.var_uc_rd);
        let assign14240_e8576: f64 = (locals.var_rde - assign14240_e8575);
        let assign14240_e8579: f64 = (0.01 * locals.var_uc_rd);
        let assign14240_e8580: f64 = (assign14240_e8576 - assign14240_e8579);
        (assign14240_e8580, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign14240_e8582;
        locals.var_tmf1_dn0 = assign14240_e8582_d_n0;
        locals.var_tmf1_dn2 = assign14240_e8582_d_n2;
        locals.var_tmf1_dn4 = assign14240_e8582_d_n4;
        locals.var_tmf1_dn5 = assign14240_e8582_d_n5;
        locals.var_tmf1_dn6 = assign14240_e8582_d_n6;
        locals.var_tmf1_dn7 = assign14240_e8582_d_n7;
        locals.var_tmf1_dn8 = assign14240_e8582_d_n8;
        locals.var_tmf1_dn9 = assign14240_e8582_d_n9;
        locals.var_tmf1_dn10 = assign14240_e8582_d_n10;
        locals.var_tmf1_dn11 = assign14240_e8582_d_n11;
        locals.var_tmf1_dn14 = assign14240_e8582_d_n14;

        let (assign14250_e8601, assign14250_e8601_d_n0, assign14250_e8601_d_n2, assign14250_e8601_d_n4, assign14250_e8601_d_n5, assign14250_e8601_d_n6, assign14250_e8601_d_n7, assign14250_e8601_d_n8, assign14250_e8601_d_n9, assign14250_e8601_d_n10, assign14250_e8601_d_n11, assign14250_e8601_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14250_e8594: f64 = (0.005 * locals.var_uc_rd);
        let assign14250_e8595: f64 = (4.0 * assign14250_e8594);
        let assign14250_e8598: f64 = (0.01 * locals.var_uc_rd);
        let assign14250_e8599: f64 = (assign14250_e8595 * assign14250_e8598);
        (assign14250_e8599, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14250_e8601;
        locals.var_tmf2_dn0 = assign14250_e8601_d_n0;
        locals.var_tmf2_dn2 = assign14250_e8601_d_n2;
        locals.var_tmf2_dn4 = assign14250_e8601_d_n4;
        locals.var_tmf2_dn5 = assign14250_e8601_d_n5;
        locals.var_tmf2_dn6 = assign14250_e8601_d_n6;
        locals.var_tmf2_dn7 = assign14250_e8601_d_n7;
        locals.var_tmf2_dn8 = assign14250_e8601_d_n8;
        locals.var_tmf2_dn9 = assign14250_e8601_d_n9;
        locals.var_tmf2_dn10 = assign14250_e8601_d_n10;
        locals.var_tmf2_dn11 = assign14250_e8601_d_n11;
        locals.var_tmf2_dn14 = assign14250_e8601_d_n14;

        let (assign14260_e8618, assign14260_e8618_d_n0, assign14260_e8618_d_n2, assign14260_e8618_d_n4, assign14260_e8618_d_n5, assign14260_e8618_d_n6, assign14260_e8618_d_n7, assign14260_e8618_d_n8, assign14260_e8618_d_n9, assign14260_e8618_d_n10, assign14260_e8618_d_n11, assign14260_e8618_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let (assign14260_e8616, assign14260_e8616_d_n0, assign14260_e8616_d_n2, assign14260_e8616_d_n4, assign14260_e8616_d_n5, assign14260_e8616_d_n6, assign14260_e8616_d_n7, assign14260_e8616_d_n8, assign14260_e8616_d_n9, assign14260_e8616_d_n10, assign14260_e8616_d_n11, assign14260_e8616_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign14260_e8615: f64 = (-locals.var_tmf2);
                (assign14260_e8615, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign14260_e8616, assign14260_e8616_d_n0, assign14260_e8616_d_n2, assign14260_e8616_d_n4, assign14260_e8616_d_n5, assign14260_e8616_d_n6, assign14260_e8616_d_n7, assign14260_e8616_d_n8, assign14260_e8616_d_n9, assign14260_e8616_d_n10, assign14260_e8616_d_n11, assign14260_e8616_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14260_e8618;
        locals.var_tmf2_dn0 = assign14260_e8618_d_n0;
        locals.var_tmf2_dn2 = assign14260_e8618_d_n2;
        locals.var_tmf2_dn4 = assign14260_e8618_d_n4;
        locals.var_tmf2_dn5 = assign14260_e8618_d_n5;
        locals.var_tmf2_dn6 = assign14260_e8618_d_n6;
        locals.var_tmf2_dn7 = assign14260_e8618_d_n7;
        locals.var_tmf2_dn8 = assign14260_e8618_d_n8;
        locals.var_tmf2_dn9 = assign14260_e8618_d_n9;
        locals.var_tmf2_dn10 = assign14260_e8618_d_n10;
        locals.var_tmf2_dn11 = assign14260_e8618_d_n11;
        locals.var_tmf2_dn14 = assign14260_e8618_d_n14;

        let (assign14270_e8634, assign14270_e8634_d_n0, assign14270_e8634_d_n2, assign14270_e8634_d_n4, assign14270_e8634_d_n5, assign14270_e8634_d_n6, assign14270_e8634_d_n7, assign14270_e8634_d_n8, assign14270_e8634_d_n9, assign14270_e8634_d_n10, assign14270_e8634_d_n11, assign14270_e8634_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14270_e8629: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14270_e8631: f64 = (assign14270_e8629 + locals.var_tmf2);
        let assign14270_e8632: f64 = (assign14270_e8631).sqrt();
        (assign14270_e8632, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14270_e8632)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14270_e8632)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14270_e8632)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14270_e8632)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14270_e8632)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14270_e8632)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14270_e8632)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14270_e8632)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14270_e8632)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14270_e8632)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign14270_e8632)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14270_e8634;
        locals.var_tmf2_dn0 = assign14270_e8634_d_n0;
        locals.var_tmf2_dn2 = assign14270_e8634_d_n2;
        locals.var_tmf2_dn4 = assign14270_e8634_d_n4;
        locals.var_tmf2_dn5 = assign14270_e8634_d_n5;
        locals.var_tmf2_dn6 = assign14270_e8634_d_n6;
        locals.var_tmf2_dn7 = assign14270_e8634_d_n7;
        locals.var_tmf2_dn8 = assign14270_e8634_d_n8;
        locals.var_tmf2_dn9 = assign14270_e8634_d_n9;
        locals.var_tmf2_dn10 = assign14270_e8634_d_n10;
        locals.var_tmf2_dn11 = assign14270_e8634_d_n11;
        locals.var_tmf2_dn14 = assign14270_e8634_d_n14;

        let (assign14280_e8651, assign14280_e8651_d_n0, assign14280_e8651_d_n2, assign14280_e8651_d_n4, assign14280_e8651_d_n5, assign14280_e8651_d_n6, assign14280_e8651_d_n7, assign14280_e8651_d_n8, assign14280_e8651_d_n9, assign14280_e8651_d_n10, assign14280_e8651_d_n11, assign14280_e8651_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14280_e8647: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14280_e8648: f64 = (1.0 + assign14280_e8647);
        let assign14280_e8649: f64 = (0.5 * assign14280_e8648);
        (assign14280_e8649, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign14280_e8651;
        locals.var_t0_dn0 = assign14280_e8651_d_n0;
        locals.var_t0_dn2 = assign14280_e8651_d_n2;
        locals.var_t0_dn4 = assign14280_e8651_d_n4;
        locals.var_t0_dn5 = assign14280_e8651_d_n5;
        locals.var_t0_dn6 = assign14280_e8651_d_n6;
        locals.var_t0_dn7 = assign14280_e8651_d_n7;
        locals.var_t0_dn8 = assign14280_e8651_d_n8;
        locals.var_t0_dn9 = assign14280_e8651_d_n9;
        locals.var_t0_dn10 = assign14280_e8651_d_n10;
        locals.var_t0_dn11 = assign14280_e8651_d_n11;
        locals.var_t0_dn14 = assign14280_e8651_d_n14;

        let (assign14290_e8670, assign14290_e8670_d_n0, assign14290_e8670_d_n2, assign14290_e8670_d_n4, assign14290_e8670_d_n5, assign14290_e8670_d_n6, assign14290_e8670_d_n7, assign14290_e8670_d_n8, assign14290_e8670_d_n9, assign14290_e8670_d_n10, assign14290_e8670_d_n11, assign14290_e8670_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 != 0.0)) && (locals.var_guard317 == 0.0)) {
        let assign14290_e8662: f64 = (0.005 * locals.var_uc_rd);
        let assign14290_e8666: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14290_e8667: f64 = (0.5 * assign14290_e8666);
        let assign14290_e8668: f64 = (assign14290_e8662 + assign14290_e8667);
        (assign14290_e8668, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign14290_e8670;
        locals.var_rde_dn0 = assign14290_e8670_d_n0;
        locals.var_rde_dn2 = assign14290_e8670_d_n2;
        locals.var_rde_dn4 = assign14290_e8670_d_n4;
        locals.var_rde_dn5 = assign14290_e8670_d_n5;
        locals.var_rde_dn6 = assign14290_e8670_d_n6;
        locals.var_rde_dn7 = assign14290_e8670_d_n7;
        locals.var_rde_dn8 = assign14290_e8670_d_n8;
        locals.var_rde_dn9 = assign14290_e8670_d_n9;
        locals.var_rde_dn10 = assign14290_e8670_d_n10;
        locals.var_rde_dn11 = assign14290_e8670_d_n11;
        locals.var_rde_dn14 = assign14290_e8670_d_n14;

        let (assign14300_e8679, assign14300_e8679_d_n0, assign14300_e8679_d_n2, assign14300_e8679_d_n4, assign14300_e8679_d_n5, assign14300_e8679_d_n6, assign14300_e8679_d_n7, assign14300_e8679_d_n8, assign14300_e8679_d_n9, assign14300_e8679_d_n10, assign14300_e8679_d_n11, assign14300_e8679_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard316 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rde, locals.var_rde_dn0, locals.var_rde_dn2, locals.var_rde_dn4, locals.var_rde_dn5, locals.var_rde_dn6, locals.var_rde_dn7, locals.var_rde_dn8, locals.var_rde_dn9, locals.var_rde_dn10, locals.var_rde_dn11, locals.var_rde_dn14,)
    }
};
        locals.var_rde = assign14300_e8679;
        locals.var_rde_dn0 = assign14300_e8679_d_n0;
        locals.var_rde_dn2 = assign14300_e8679_d_n2;
        locals.var_rde_dn4 = assign14300_e8679_d_n4;
        locals.var_rde_dn5 = assign14300_e8679_d_n5;
        locals.var_rde_dn6 = assign14300_e8679_d_n6;
        locals.var_rde_dn7 = assign14300_e8679_d_n7;
        locals.var_rde_dn8 = assign14300_e8679_d_n8;
        locals.var_rde_dn9 = assign14300_e8679_d_n9;
        locals.var_rde_dn10 = assign14300_e8679_d_n10;
        locals.var_rde_dn11 = assign14300_e8679_d_n11;
        locals.var_rde_dn14 = assign14300_e8679_d_n14;

        let assign14310_e8682: f64 = if locals.var_uc_rs > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard318 = assign14310_e8682;

        let (assign14320_e8706, assign14320_e8706_d_n0, assign14320_e8706_d_n2, assign14320_e8706_d_n4, assign14320_e8706_d_n5, assign14320_e8706_d_n6, assign14320_e8706_d_n7, assign14320_e8706_d_n8, assign14320_e8706_d_n9, assign14320_e8706_d_n10, assign14320_e8706_d_n11, assign14320_e8706_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) {
        let assign14320_e8691: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign14320_e8693: f64 = (assign14320_e8691 * 1000000.0);
        let assign14320_e8695: f64 = (assign14320_e8693 + locals.var_uc_rdict1);
        let assign14320_e8696: f64 = (locals.var_rdtemp0 * assign14320_e8695);
        let assign14320_e8699: f64 = (p.p70 * p.p100);
        let assign14320_e8701: f64 = (assign14320_e8699 * 1000000.0);
        let assign14320_e8703: f64 = (assign14320_e8701 + p.p101);
        let assign14320_e8704: f64 = (assign14320_e8696 * assign14320_e8703);
        (assign14320_e8704, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign14320_e8706;
        locals.var_t2_dn0 = assign14320_e8706_d_n0;
        locals.var_t2_dn2 = assign14320_e8706_d_n2;
        locals.var_t2_dn4 = assign14320_e8706_d_n4;
        locals.var_t2_dn5 = assign14320_e8706_d_n5;
        locals.var_t2_dn6 = assign14320_e8706_d_n6;
        locals.var_t2_dn7 = assign14320_e8706_d_n7;
        locals.var_t2_dn8 = assign14320_e8706_d_n8;
        locals.var_t2_dn9 = assign14320_e8706_d_n9;
        locals.var_t2_dn10 = assign14320_e8706_d_n10;
        locals.var_t2_dn11 = assign14320_e8706_d_n11;
        locals.var_t2_dn14 = assign14320_e8706_d_n14;

        let assign14330_e8709: f64 = if p.p39 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard319 = assign14330_e8709;

        let (assign14340_e8729, assign14340_e8729_d_n0, assign14340_e8729_d_n2, assign14340_e8729_d_n4, assign14340_e8729_d_n5, assign14340_e8729_d_n6, assign14340_e8729_d_n7, assign14340_e8729_d_n8, assign14340_e8729_d_n9, assign14340_e8729_d_n10, assign14340_e8729_d_n11, assign14340_e8729_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 != 0.0)) {
        let assign14340_e8720: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff0);
        let assign14340_e8721: f64 = (locals.var_uc_rs + assign14340_e8720);
        let assign14340_e8724: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff0_2);
        let assign14340_e8725: f64 = (assign14340_e8721 + assign14340_e8724);
        let assign14340_e8727: f64 = (assign14340_e8725 * locals.var_t2);
        (assign14340_e8727, ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn11) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn11)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn11)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff0_dn14) + (locals.var_mks_rdtemp2 * locals.var_tdiff0_2_dn14)) * locals.var_t2) + (assign14340_e8725 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign14340_e8729;
        locals.var_rse_dn0 = assign14340_e8729_d_n0;
        locals.var_rse_dn2 = assign14340_e8729_d_n2;
        locals.var_rse_dn4 = assign14340_e8729_d_n4;
        locals.var_rse_dn5 = assign14340_e8729_d_n5;
        locals.var_rse_dn6 = assign14340_e8729_d_n6;
        locals.var_rse_dn7 = assign14340_e8729_d_n7;
        locals.var_rse_dn8 = assign14340_e8729_d_n8;
        locals.var_rse_dn9 = assign14340_e8729_d_n9;
        locals.var_rse_dn10 = assign14340_e8729_d_n10;
        locals.var_rse_dn11 = assign14340_e8729_d_n11;
        locals.var_rse_dn14 = assign14340_e8729_d_n14;

        let (assign14350_e8747, assign14350_e8747_d_n0, assign14350_e8747_d_n2, assign14350_e8747_d_n4, assign14350_e8747_d_n5, assign14350_e8747_d_n6, assign14350_e8747_d_n7, assign14350_e8747_d_n8, assign14350_e8747_d_n9, assign14350_e8747_d_n10, assign14350_e8747_d_n11, assign14350_e8747_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 != 0.0)) {
        let assign14350_e8740: f64 = (0.005 * locals.var_uc_rs);
        let assign14350_e8741: f64 = (locals.var_rse - assign14350_e8740);
        let assign14350_e8744: f64 = (0.01 * locals.var_uc_rs);
        let assign14350_e8745: f64 = (assign14350_e8741 - assign14350_e8744);
        (assign14350_e8745, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign14350_e8747;
        locals.var_tmf1_dn0 = assign14350_e8747_d_n0;
        locals.var_tmf1_dn2 = assign14350_e8747_d_n2;
        locals.var_tmf1_dn4 = assign14350_e8747_d_n4;
        locals.var_tmf1_dn5 = assign14350_e8747_d_n5;
        locals.var_tmf1_dn6 = assign14350_e8747_d_n6;
        locals.var_tmf1_dn7 = assign14350_e8747_d_n7;
        locals.var_tmf1_dn8 = assign14350_e8747_d_n8;
        locals.var_tmf1_dn9 = assign14350_e8747_d_n9;
        locals.var_tmf1_dn10 = assign14350_e8747_d_n10;
        locals.var_tmf1_dn11 = assign14350_e8747_d_n11;
        locals.var_tmf1_dn14 = assign14350_e8747_d_n14;

        let (assign14360_e8765, assign14360_e8765_d_n0, assign14360_e8765_d_n2, assign14360_e8765_d_n4, assign14360_e8765_d_n5, assign14360_e8765_d_n6, assign14360_e8765_d_n7, assign14360_e8765_d_n8, assign14360_e8765_d_n9, assign14360_e8765_d_n10, assign14360_e8765_d_n11, assign14360_e8765_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 != 0.0)) {
        let assign14360_e8758: f64 = (0.005 * locals.var_uc_rs);
        let assign14360_e8759: f64 = (4.0 * assign14360_e8758);
        let assign14360_e8762: f64 = (0.01 * locals.var_uc_rs);
        let assign14360_e8763: f64 = (assign14360_e8759 * assign14360_e8762);
        (assign14360_e8763, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14360_e8765;
        locals.var_tmf2_dn0 = assign14360_e8765_d_n0;
        locals.var_tmf2_dn2 = assign14360_e8765_d_n2;
        locals.var_tmf2_dn4 = assign14360_e8765_d_n4;
        locals.var_tmf2_dn5 = assign14360_e8765_d_n5;
        locals.var_tmf2_dn6 = assign14360_e8765_d_n6;
        locals.var_tmf2_dn7 = assign14360_e8765_d_n7;
        locals.var_tmf2_dn8 = assign14360_e8765_d_n8;
        locals.var_tmf2_dn9 = assign14360_e8765_d_n9;
        locals.var_tmf2_dn10 = assign14360_e8765_d_n10;
        locals.var_tmf2_dn11 = assign14360_e8765_d_n11;
        locals.var_tmf2_dn14 = assign14360_e8765_d_n14;

        let (assign14370_e8781, assign14370_e8781_d_n0, assign14370_e8781_d_n2, assign14370_e8781_d_n4, assign14370_e8781_d_n5, assign14370_e8781_d_n6, assign14370_e8781_d_n7, assign14370_e8781_d_n8, assign14370_e8781_d_n9, assign14370_e8781_d_n10, assign14370_e8781_d_n11, assign14370_e8781_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 != 0.0)) {
        let (assign14370_e8779, assign14370_e8779_d_n0, assign14370_e8779_d_n2, assign14370_e8779_d_n4, assign14370_e8779_d_n5, assign14370_e8779_d_n6, assign14370_e8779_d_n7, assign14370_e8779_d_n8, assign14370_e8779_d_n9, assign14370_e8779_d_n10, assign14370_e8779_d_n11, assign14370_e8779_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign14370_e8778: f64 = (-locals.var_tmf2);
                (assign14370_e8778, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign14370_e8779, assign14370_e8779_d_n0, assign14370_e8779_d_n2, assign14370_e8779_d_n4, assign14370_e8779_d_n5, assign14370_e8779_d_n6, assign14370_e8779_d_n7, assign14370_e8779_d_n8, assign14370_e8779_d_n9, assign14370_e8779_d_n10, assign14370_e8779_d_n11, assign14370_e8779_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14370_e8781;
        locals.var_tmf2_dn0 = assign14370_e8781_d_n0;
        locals.var_tmf2_dn2 = assign14370_e8781_d_n2;
        locals.var_tmf2_dn4 = assign14370_e8781_d_n4;
        locals.var_tmf2_dn5 = assign14370_e8781_d_n5;
        locals.var_tmf2_dn6 = assign14370_e8781_d_n6;
        locals.var_tmf2_dn7 = assign14370_e8781_d_n7;
        locals.var_tmf2_dn8 = assign14370_e8781_d_n8;
        locals.var_tmf2_dn9 = assign14370_e8781_d_n9;
        locals.var_tmf2_dn10 = assign14370_e8781_d_n10;
        locals.var_tmf2_dn11 = assign14370_e8781_d_n11;
        locals.var_tmf2_dn14 = assign14370_e8781_d_n14;

        let (assign14380_e8796, assign14380_e8796_d_n0, assign14380_e8796_d_n2, assign14380_e8796_d_n4, assign14380_e8796_d_n5, assign14380_e8796_d_n6, assign14380_e8796_d_n7, assign14380_e8796_d_n8, assign14380_e8796_d_n9, assign14380_e8796_d_n10, assign14380_e8796_d_n11, assign14380_e8796_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 != 0.0)) {
        let assign14380_e8791: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14380_e8793: f64 = (assign14380_e8791 + locals.var_tmf2);
        let assign14380_e8794: f64 = (assign14380_e8793).sqrt();
        (assign14380_e8794, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14380_e8794)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14380_e8794)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14380_e8794)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14380_e8794)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14380_e8794)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14380_e8794)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14380_e8794)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14380_e8794)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14380_e8794)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14380_e8794)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign14380_e8794)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14380_e8796;
        locals.var_tmf2_dn0 = assign14380_e8796_d_n0;
        locals.var_tmf2_dn2 = assign14380_e8796_d_n2;
        locals.var_tmf2_dn4 = assign14380_e8796_d_n4;
        locals.var_tmf2_dn5 = assign14380_e8796_d_n5;
        locals.var_tmf2_dn6 = assign14380_e8796_d_n6;
        locals.var_tmf2_dn7 = assign14380_e8796_d_n7;
        locals.var_tmf2_dn8 = assign14380_e8796_d_n8;
        locals.var_tmf2_dn9 = assign14380_e8796_d_n9;
        locals.var_tmf2_dn10 = assign14380_e8796_d_n10;
        locals.var_tmf2_dn11 = assign14380_e8796_d_n11;
        locals.var_tmf2_dn14 = assign14380_e8796_d_n14;

        let (assign14390_e8812, assign14390_e8812_d_n0, assign14390_e8812_d_n2, assign14390_e8812_d_n4, assign14390_e8812_d_n5, assign14390_e8812_d_n6, assign14390_e8812_d_n7, assign14390_e8812_d_n8, assign14390_e8812_d_n9, assign14390_e8812_d_n10, assign14390_e8812_d_n11, assign14390_e8812_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 != 0.0)) {
        let assign14390_e8808: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14390_e8809: f64 = (1.0 + assign14390_e8808);
        let assign14390_e8810: f64 = (0.5 * assign14390_e8809);
        (assign14390_e8810, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign14390_e8812;
        locals.var_t0_dn0 = assign14390_e8812_d_n0;
        locals.var_t0_dn2 = assign14390_e8812_d_n2;
        locals.var_t0_dn4 = assign14390_e8812_d_n4;
        locals.var_t0_dn5 = assign14390_e8812_d_n5;
        locals.var_t0_dn6 = assign14390_e8812_d_n6;
        locals.var_t0_dn7 = assign14390_e8812_d_n7;
        locals.var_t0_dn8 = assign14390_e8812_d_n8;
        locals.var_t0_dn9 = assign14390_e8812_d_n9;
        locals.var_t0_dn10 = assign14390_e8812_d_n10;
        locals.var_t0_dn11 = assign14390_e8812_d_n11;
        locals.var_t0_dn14 = assign14390_e8812_d_n14;

        let (assign14400_e8830, assign14400_e8830_d_n0, assign14400_e8830_d_n2, assign14400_e8830_d_n4, assign14400_e8830_d_n5, assign14400_e8830_d_n6, assign14400_e8830_d_n7, assign14400_e8830_d_n8, assign14400_e8830_d_n9, assign14400_e8830_d_n10, assign14400_e8830_d_n11, assign14400_e8830_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 != 0.0)) {
        let assign14400_e8822: f64 = (0.005 * locals.var_uc_rs);
        let assign14400_e8826: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14400_e8827: f64 = (0.5 * assign14400_e8826);
        let assign14400_e8828: f64 = (assign14400_e8822 + assign14400_e8827);
        (assign14400_e8828, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign14400_e8830;
        locals.var_rse_dn0 = assign14400_e8830_d_n0;
        locals.var_rse_dn2 = assign14400_e8830_d_n2;
        locals.var_rse_dn4 = assign14400_e8830_d_n4;
        locals.var_rse_dn5 = assign14400_e8830_d_n5;
        locals.var_rse_dn6 = assign14400_e8830_d_n6;
        locals.var_rse_dn7 = assign14400_e8830_d_n7;
        locals.var_rse_dn8 = assign14400_e8830_d_n8;
        locals.var_rse_dn9 = assign14400_e8830_d_n9;
        locals.var_rse_dn10 = assign14400_e8830_d_n10;
        locals.var_rse_dn11 = assign14400_e8830_d_n11;
        locals.var_rse_dn14 = assign14400_e8830_d_n14;

        let (assign14410_e8851, assign14410_e8851_d_n0, assign14410_e8851_d_n2, assign14410_e8851_d_n4, assign14410_e8851_d_n5, assign14410_e8851_d_n6, assign14410_e8851_d_n7, assign14410_e8851_d_n8, assign14410_e8851_d_n9, assign14410_e8851_d_n10, assign14410_e8851_d_n11, assign14410_e8851_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign14410_e8842: f64 = (locals.var_mks_rdtemp1 * locals.var_tdiff);
        let assign14410_e8843: f64 = (locals.var_uc_rs + assign14410_e8842);
        let assign14410_e8846: f64 = (locals.var_mks_rdtemp2 * locals.var_tdiff_2);
        let assign14410_e8847: f64 = (assign14410_e8843 + assign14410_e8846);
        let assign14410_e8849: f64 = (assign14410_e8847 * locals.var_t2);
        (assign14410_e8849, ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn0)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn2)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn4)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn5)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn6)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn7)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn8)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn9)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn10)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn11) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn11)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn11)), ((((locals.var_mks_rdtemp1 * locals.var_tdiff_dn14) + (locals.var_mks_rdtemp2 * locals.var_tdiff_2_dn14)) * locals.var_t2) + (assign14410_e8847 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign14410_e8851;
        locals.var_rse_dn0 = assign14410_e8851_d_n0;
        locals.var_rse_dn2 = assign14410_e8851_d_n2;
        locals.var_rse_dn4 = assign14410_e8851_d_n4;
        locals.var_rse_dn5 = assign14410_e8851_d_n5;
        locals.var_rse_dn6 = assign14410_e8851_d_n6;
        locals.var_rse_dn7 = assign14410_e8851_d_n7;
        locals.var_rse_dn8 = assign14410_e8851_d_n8;
        locals.var_rse_dn9 = assign14410_e8851_d_n9;
        locals.var_rse_dn10 = assign14410_e8851_d_n10;
        locals.var_rse_dn11 = assign14410_e8851_d_n11;
        locals.var_rse_dn14 = assign14410_e8851_d_n14;

        let (assign14420_e8870, assign14420_e8870_d_n0, assign14420_e8870_d_n2, assign14420_e8870_d_n4, assign14420_e8870_d_n5, assign14420_e8870_d_n6, assign14420_e8870_d_n7, assign14420_e8870_d_n8, assign14420_e8870_d_n9, assign14420_e8870_d_n10, assign14420_e8870_d_n11, assign14420_e8870_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign14420_e8863: f64 = (0.005 * locals.var_uc_rs);
        let assign14420_e8864: f64 = (locals.var_rse - assign14420_e8863);
        let assign14420_e8867: f64 = (0.01 * locals.var_uc_rs);
        let assign14420_e8868: f64 = (assign14420_e8864 - assign14420_e8867);
        (assign14420_e8868, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign14420_e8870;
        locals.var_tmf1_dn0 = assign14420_e8870_d_n0;
        locals.var_tmf1_dn2 = assign14420_e8870_d_n2;
        locals.var_tmf1_dn4 = assign14420_e8870_d_n4;
        locals.var_tmf1_dn5 = assign14420_e8870_d_n5;
        locals.var_tmf1_dn6 = assign14420_e8870_d_n6;
        locals.var_tmf1_dn7 = assign14420_e8870_d_n7;
        locals.var_tmf1_dn8 = assign14420_e8870_d_n8;
        locals.var_tmf1_dn9 = assign14420_e8870_d_n9;
        locals.var_tmf1_dn10 = assign14420_e8870_d_n10;
        locals.var_tmf1_dn11 = assign14420_e8870_d_n11;
        locals.var_tmf1_dn14 = assign14420_e8870_d_n14;

        let (assign14430_e8889, assign14430_e8889_d_n0, assign14430_e8889_d_n2, assign14430_e8889_d_n4, assign14430_e8889_d_n5, assign14430_e8889_d_n6, assign14430_e8889_d_n7, assign14430_e8889_d_n8, assign14430_e8889_d_n9, assign14430_e8889_d_n10, assign14430_e8889_d_n11, assign14430_e8889_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign14430_e8882: f64 = (0.005 * locals.var_uc_rs);
        let assign14430_e8883: f64 = (4.0 * assign14430_e8882);
        let assign14430_e8886: f64 = (0.01 * locals.var_uc_rs);
        let assign14430_e8887: f64 = (assign14430_e8883 * assign14430_e8886);
        (assign14430_e8887, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14430_e8889;
        locals.var_tmf2_dn0 = assign14430_e8889_d_n0;
        locals.var_tmf2_dn2 = assign14430_e8889_d_n2;
        locals.var_tmf2_dn4 = assign14430_e8889_d_n4;
        locals.var_tmf2_dn5 = assign14430_e8889_d_n5;
        locals.var_tmf2_dn6 = assign14430_e8889_d_n6;
        locals.var_tmf2_dn7 = assign14430_e8889_d_n7;
        locals.var_tmf2_dn8 = assign14430_e8889_d_n8;
        locals.var_tmf2_dn9 = assign14430_e8889_d_n9;
        locals.var_tmf2_dn10 = assign14430_e8889_d_n10;
        locals.var_tmf2_dn11 = assign14430_e8889_d_n11;
        locals.var_tmf2_dn14 = assign14430_e8889_d_n14;

        let (assign14440_e8906, assign14440_e8906_d_n0, assign14440_e8906_d_n2, assign14440_e8906_d_n4, assign14440_e8906_d_n5, assign14440_e8906_d_n6, assign14440_e8906_d_n7, assign14440_e8906_d_n8, assign14440_e8906_d_n9, assign14440_e8906_d_n10, assign14440_e8906_d_n11, assign14440_e8906_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 == 0.0)) {
        let (assign14440_e8904, assign14440_e8904_d_n0, assign14440_e8904_d_n2, assign14440_e8904_d_n4, assign14440_e8904_d_n5, assign14440_e8904_d_n6, assign14440_e8904_d_n7, assign14440_e8904_d_n8, assign14440_e8904_d_n9, assign14440_e8904_d_n10, assign14440_e8904_d_n11, assign14440_e8904_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign14440_e8903: f64 = (-locals.var_tmf2);
                (assign14440_e8903, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign14440_e8904, assign14440_e8904_d_n0, assign14440_e8904_d_n2, assign14440_e8904_d_n4, assign14440_e8904_d_n5, assign14440_e8904_d_n6, assign14440_e8904_d_n7, assign14440_e8904_d_n8, assign14440_e8904_d_n9, assign14440_e8904_d_n10, assign14440_e8904_d_n11, assign14440_e8904_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14440_e8906;
        locals.var_tmf2_dn0 = assign14440_e8906_d_n0;
        locals.var_tmf2_dn2 = assign14440_e8906_d_n2;
        locals.var_tmf2_dn4 = assign14440_e8906_d_n4;
        locals.var_tmf2_dn5 = assign14440_e8906_d_n5;
        locals.var_tmf2_dn6 = assign14440_e8906_d_n6;
        locals.var_tmf2_dn7 = assign14440_e8906_d_n7;
        locals.var_tmf2_dn8 = assign14440_e8906_d_n8;
        locals.var_tmf2_dn9 = assign14440_e8906_d_n9;
        locals.var_tmf2_dn10 = assign14440_e8906_d_n10;
        locals.var_tmf2_dn11 = assign14440_e8906_d_n11;
        locals.var_tmf2_dn14 = assign14440_e8906_d_n14;

    }

    pub(super) fn stamp_transient_block_27(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14450_e8922, assign14450_e8922_d_n0, assign14450_e8922_d_n2, assign14450_e8922_d_n4, assign14450_e8922_d_n5, assign14450_e8922_d_n6, assign14450_e8922_d_n7, assign14450_e8922_d_n8, assign14450_e8922_d_n9, assign14450_e8922_d_n10, assign14450_e8922_d_n11, assign14450_e8922_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign14450_e8917: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14450_e8919: f64 = (assign14450_e8917 + locals.var_tmf2);
        let assign14450_e8920: f64 = (assign14450_e8919).sqrt();
        (assign14450_e8920, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14450_e8920)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14450_e8920)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14450_e8920)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14450_e8920)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14450_e8920)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14450_e8920)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14450_e8920)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14450_e8920)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14450_e8920)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14450_e8920)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign14450_e8920)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14450_e8922;
        locals.var_tmf2_dn0 = assign14450_e8922_d_n0;
        locals.var_tmf2_dn2 = assign14450_e8922_d_n2;
        locals.var_tmf2_dn4 = assign14450_e8922_d_n4;
        locals.var_tmf2_dn5 = assign14450_e8922_d_n5;
        locals.var_tmf2_dn6 = assign14450_e8922_d_n6;
        locals.var_tmf2_dn7 = assign14450_e8922_d_n7;
        locals.var_tmf2_dn8 = assign14450_e8922_d_n8;
        locals.var_tmf2_dn9 = assign14450_e8922_d_n9;
        locals.var_tmf2_dn10 = assign14450_e8922_d_n10;
        locals.var_tmf2_dn11 = assign14450_e8922_d_n11;
        locals.var_tmf2_dn14 = assign14450_e8922_d_n14;

        let (assign14460_e8939, assign14460_e8939_d_n0, assign14460_e8939_d_n2, assign14460_e8939_d_n4, assign14460_e8939_d_n5, assign14460_e8939_d_n6, assign14460_e8939_d_n7, assign14460_e8939_d_n8, assign14460_e8939_d_n9, assign14460_e8939_d_n10, assign14460_e8939_d_n11, assign14460_e8939_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign14460_e8935: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14460_e8936: f64 = (1.0 + assign14460_e8935);
        let assign14460_e8937: f64 = (0.5 * assign14460_e8936);
        (assign14460_e8937, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign14460_e8939;
        locals.var_t0_dn0 = assign14460_e8939_d_n0;
        locals.var_t0_dn2 = assign14460_e8939_d_n2;
        locals.var_t0_dn4 = assign14460_e8939_d_n4;
        locals.var_t0_dn5 = assign14460_e8939_d_n5;
        locals.var_t0_dn6 = assign14460_e8939_d_n6;
        locals.var_t0_dn7 = assign14460_e8939_d_n7;
        locals.var_t0_dn8 = assign14460_e8939_d_n8;
        locals.var_t0_dn9 = assign14460_e8939_d_n9;
        locals.var_t0_dn10 = assign14460_e8939_d_n10;
        locals.var_t0_dn11 = assign14460_e8939_d_n11;
        locals.var_t0_dn14 = assign14460_e8939_d_n14;

        let (assign14470_e8958, assign14470_e8958_d_n0, assign14470_e8958_d_n2, assign14470_e8958_d_n4, assign14470_e8958_d_n5, assign14470_e8958_d_n6, assign14470_e8958_d_n7, assign14470_e8958_d_n8, assign14470_e8958_d_n9, assign14470_e8958_d_n10, assign14470_e8958_d_n11, assign14470_e8958_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 != 0.0)) && (locals.var_guard319 == 0.0)) {
        let assign14470_e8950: f64 = (0.005 * locals.var_uc_rs);
        let assign14470_e8954: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14470_e8955: f64 = (0.5 * assign14470_e8954);
        let assign14470_e8956: f64 = (assign14470_e8950 + assign14470_e8955);
        (assign14470_e8956, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign14470_e8958;
        locals.var_rse_dn0 = assign14470_e8958_d_n0;
        locals.var_rse_dn2 = assign14470_e8958_d_n2;
        locals.var_rse_dn4 = assign14470_e8958_d_n4;
        locals.var_rse_dn5 = assign14470_e8958_d_n5;
        locals.var_rse_dn6 = assign14470_e8958_d_n6;
        locals.var_rse_dn7 = assign14470_e8958_d_n7;
        locals.var_rse_dn8 = assign14470_e8958_d_n8;
        locals.var_rse_dn9 = assign14470_e8958_d_n9;
        locals.var_rse_dn10 = assign14470_e8958_d_n10;
        locals.var_rse_dn11 = assign14470_e8958_d_n11;
        locals.var_rse_dn14 = assign14470_e8958_d_n14;

        let (assign14480_e8967, assign14480_e8967_d_n0, assign14480_e8967_d_n2, assign14480_e8967_d_n4, assign14480_e8967_d_n5, assign14480_e8967_d_n6, assign14480_e8967_d_n7, assign14480_e8967_d_n8, assign14480_e8967_d_n9, assign14480_e8967_d_n10, assign14480_e8967_d_n11, assign14480_e8967_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard318 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rse, locals.var_rse_dn0, locals.var_rse_dn2, locals.var_rse_dn4, locals.var_rse_dn5, locals.var_rse_dn6, locals.var_rse_dn7, locals.var_rse_dn8, locals.var_rse_dn9, locals.var_rse_dn10, locals.var_rse_dn11, locals.var_rse_dn14,)
    }
};
        locals.var_rse = assign14480_e8967;
        locals.var_rse_dn0 = assign14480_e8967_d_n0;
        locals.var_rse_dn2 = assign14480_e8967_d_n2;
        locals.var_rse_dn4 = assign14480_e8967_d_n4;
        locals.var_rse_dn5 = assign14480_e8967_d_n5;
        locals.var_rse_dn6 = assign14480_e8967_d_n6;
        locals.var_rse_dn7 = assign14480_e8967_d_n7;
        locals.var_rse_dn8 = assign14480_e8967_d_n8;
        locals.var_rse_dn9 = assign14480_e8967_d_n9;
        locals.var_rse_dn10 = assign14480_e8967_d_n10;
        locals.var_rse_dn11 = assign14480_e8967_d_n11;
        locals.var_rse_dn14 = assign14480_e8967_d_n14;

        let assign14490_e8970: f64 = if locals.var_uc_rdvd > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard320 = assign14490_e8970;

        let (assign14500_e8994, assign14500_e8994_d_n0, assign14500_e8994_d_n2, assign14500_e8994_d_n4, assign14500_e8994_d_n5, assign14500_e8994_d_n6, assign14500_e8994_d_n7, assign14500_e8994_d_n8, assign14500_e8994_d_n9, assign14500_e8994_d_n10, assign14500_e8994_d_n11, assign14500_e8994_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14500_e8979: f64 = (p.p67 * locals.var_uc_rdslp1);
        let assign14500_e8981: f64 = (assign14500_e8979 * 1000000.0);
        let assign14500_e8983: f64 = (assign14500_e8981 + locals.var_uc_rdict1);
        let assign14500_e8984: f64 = (locals.var_rdvdtemp0 * assign14500_e8983);
        let assign14500_e8987: f64 = (p.p68 * p.p100);
        let assign14500_e8989: f64 = (assign14500_e8987 * 1000000.0);
        let assign14500_e8991: f64 = (assign14500_e8989 + p.p101);
        let assign14500_e8992: f64 = (assign14500_e8984 * assign14500_e8991);
        (assign14500_e8992, ((locals.var_rdvdtemp0_dn0 * assign14500_e8983) * assign14500_e8991), ((locals.var_rdvdtemp0_dn2 * assign14500_e8983) * assign14500_e8991), ((locals.var_rdvdtemp0_dn4 * assign14500_e8983) * assign14500_e8991), ((locals.var_rdvdtemp0_dn5 * assign14500_e8983) * assign14500_e8991), ((locals.var_rdvdtemp0_dn6 * assign14500_e8983) * assign14500_e8991), ((locals.var_rdvdtemp0_dn7 * assign14500_e8983) * assign14500_e8991), ((locals.var_rdvdtemp0_dn8 * assign14500_e8983) * assign14500_e8991), ((locals.var_rdvdtemp0_dn9 * assign14500_e8983) * assign14500_e8991), ((locals.var_rdvdtemp0_dn10 * assign14500_e8983) * assign14500_e8991), ((locals.var_rdvdtemp0_dn11 * assign14500_e8983) * assign14500_e8991), ((locals.var_rdvdtemp0_dn14 * assign14500_e8983) * assign14500_e8991),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign14500_e8994;
        locals.var_t4_dn0 = assign14500_e8994_d_n0;
        locals.var_t4_dn2 = assign14500_e8994_d_n2;
        locals.var_t4_dn4 = assign14500_e8994_d_n4;
        locals.var_t4_dn5 = assign14500_e8994_d_n5;
        locals.var_t4_dn6 = assign14500_e8994_d_n6;
        locals.var_t4_dn7 = assign14500_e8994_d_n7;
        locals.var_t4_dn8 = assign14500_e8994_d_n8;
        locals.var_t4_dn9 = assign14500_e8994_d_n9;
        locals.var_t4_dn10 = assign14500_e8994_d_n10;
        locals.var_t4_dn11 = assign14500_e8994_d_n11;
        locals.var_t4_dn14 = assign14500_e8994_d_n14;

        let (assign14510_e9008, assign14510_e9008_d_n0, assign14510_e9008_d_n2, assign14510_e9008_d_n4, assign14510_e9008_d_n5, assign14510_e9008_d_n6, assign14510_e9008_d_n7, assign14510_e9008_d_n8, assign14510_e9008_d_n9, assign14510_e9008_d_n10, assign14510_e9008_d_n11, assign14510_e9008_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14510_e9002: f64 = (1.0 - locals.var_uc_rdov13);
        let assign14510_e9004: f64 = (assign14510_e9002 * p.p63);
        let assign14510_e9006: f64 = (assign14510_e9004 * 1000000.0);
        (assign14510_e9006, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign14510_e9008;
        locals.var_t1_dn0 = assign14510_e9008_d_n0;
        locals.var_t1_dn2 = assign14510_e9008_d_n2;
        locals.var_t1_dn4 = assign14510_e9008_d_n4;
        locals.var_t1_dn5 = assign14510_e9008_d_n5;
        locals.var_t1_dn6 = assign14510_e9008_d_n6;
        locals.var_t1_dn7 = assign14510_e9008_d_n7;
        locals.var_t1_dn8 = assign14510_e9008_d_n8;
        locals.var_t1_dn9 = assign14510_e9008_d_n9;
        locals.var_t1_dn10 = assign14510_e9008_d_n10;
        locals.var_t1_dn11 = assign14510_e9008_d_n11;
        locals.var_t1_dn14 = assign14510_e9008_d_n14;

        let (assign14520_e9029, assign14520_e9029_d_n0, assign14520_e9029_d_n2, assign14520_e9029_d_n4, assign14520_e9029_d_n5, assign14520_e9029_d_n6, assign14520_e9029_d_n7, assign14520_e9029_d_n8, assign14520_e9029_d_n9, assign14520_e9029_d_n10, assign14520_e9029_d_n11, assign14520_e9029_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14520_e9016: f64 = (p.p99 * p.p99);
        let assign14520_e9020: f64 = (0.0001 * 0.01);
        let assign14520_e9021: f64 = (4.0 * assign14520_e9020);
        let assign14520_e9024: f64 = (0.0001 * 0.01);
        let assign14520_e9025: f64 = (assign14520_e9021 * assign14520_e9024);
        let assign14520_e9026: f64 = (assign14520_e9016 + assign14520_e9025);
        let assign14520_e9027: f64 = (assign14520_e9026).sqrt();
        (assign14520_e9027, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14520_e9029;
        locals.var_tmf2_dn0 = assign14520_e9029_d_n0;
        locals.var_tmf2_dn2 = assign14520_e9029_d_n2;
        locals.var_tmf2_dn4 = assign14520_e9029_d_n4;
        locals.var_tmf2_dn5 = assign14520_e9029_d_n5;
        locals.var_tmf2_dn6 = assign14520_e9029_d_n6;
        locals.var_tmf2_dn7 = assign14520_e9029_d_n7;
        locals.var_tmf2_dn8 = assign14520_e9029_d_n8;
        locals.var_tmf2_dn9 = assign14520_e9029_d_n9;
        locals.var_tmf2_dn10 = assign14520_e9029_d_n10;
        locals.var_tmf2_dn11 = assign14520_e9029_d_n11;
        locals.var_tmf2_dn14 = assign14520_e9029_d_n14;

        let (assign14530_e9043, assign14530_e9043_d_n0, assign14530_e9043_d_n2, assign14530_e9043_d_n4, assign14530_e9043_d_n5, assign14530_e9043_d_n6, assign14530_e9043_d_n7, assign14530_e9043_d_n8, assign14530_e9043_d_n9, assign14530_e9043_d_n10, assign14530_e9043_d_n11, assign14530_e9043_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14530_e9039: f64 = (p.p99 / locals.var_tmf2);
        let assign14530_e9040: f64 = (1.0 + assign14530_e9039);
        let assign14530_e9041: f64 = (0.5 * assign14530_e9040);
        (assign14530_e9041, (0.5 * (-((p.p99 * locals.var_tmf2_dn0) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn2) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn4) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn5) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn6) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn7) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn8) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn9) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn10) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn11) / (locals.var_tmf2 * locals.var_tmf2)))), (0.5 * (-((p.p99 * locals.var_tmf2_dn14) / (locals.var_tmf2 * locals.var_tmf2)))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign14530_e9043;
        locals.var_t0_dn0 = assign14530_e9043_d_n0;
        locals.var_t0_dn2 = assign14530_e9043_d_n2;
        locals.var_t0_dn4 = assign14530_e9043_d_n4;
        locals.var_t0_dn5 = assign14530_e9043_d_n5;
        locals.var_t0_dn6 = assign14530_e9043_d_n6;
        locals.var_t0_dn7 = assign14530_e9043_d_n7;
        locals.var_t0_dn8 = assign14530_e9043_d_n8;
        locals.var_t0_dn9 = assign14530_e9043_d_n9;
        locals.var_t0_dn10 = assign14530_e9043_d_n10;
        locals.var_t0_dn11 = assign14530_e9043_d_n11;
        locals.var_t0_dn14 = assign14530_e9043_d_n14;

        let (assign14540_e9055, assign14540_e9055_d_n0, assign14540_e9055_d_n2, assign14540_e9055_d_n4, assign14540_e9055_d_n5, assign14540_e9055_d_n6, assign14540_e9055_d_n7, assign14540_e9055_d_n8, assign14540_e9055_d_n9, assign14540_e9055_d_n10, assign14540_e9055_d_n11, assign14540_e9055_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14540_e9052: f64 = (p.p99 + locals.var_tmf2);
        let assign14540_e9053: f64 = (0.5 * assign14540_e9052);
        (assign14540_e9053, (0.5 * locals.var_tmf2_dn0), (0.5 * locals.var_tmf2_dn2), (0.5 * locals.var_tmf2_dn4), (0.5 * locals.var_tmf2_dn5), (0.5 * locals.var_tmf2_dn6), (0.5 * locals.var_tmf2_dn7), (0.5 * locals.var_tmf2_dn8), (0.5 * locals.var_tmf2_dn9), (0.5 * locals.var_tmf2_dn10), (0.5 * locals.var_tmf2_dn11), (0.5 * locals.var_tmf2_dn14),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign14540_e9055;
        locals.var_t2_dn0 = assign14540_e9055_d_n0;
        locals.var_t2_dn2 = assign14540_e9055_d_n2;
        locals.var_t2_dn4 = assign14540_e9055_d_n4;
        locals.var_t2_dn5 = assign14540_e9055_d_n5;
        locals.var_t2_dn6 = assign14540_e9055_d_n6;
        locals.var_t2_dn7 = assign14540_e9055_d_n7;
        locals.var_t2_dn8 = assign14540_e9055_d_n8;
        locals.var_t2_dn9 = assign14540_e9055_d_n9;
        locals.var_t2_dn10 = assign14540_e9055_d_n10;
        locals.var_t2_dn11 = assign14540_e9055_d_n11;
        locals.var_t2_dn14 = assign14540_e9055_d_n14;

        let assign14550_e9058: f64 = if locals.var_t2 < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard321 = assign14550_e9058;

        let (assign14560_e9068, assign14560_e9068_d_n0, assign14560_e9068_d_n2, assign14560_e9068_d_n4, assign14560_e9068_d_n5, assign14560_e9068_d_n6, assign14560_e9068_d_n7, assign14560_e9068_d_n8, assign14560_e9068_d_n9, assign14560_e9068_d_n10, assign14560_e9068_d_n11, assign14560_e9068_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard321 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign14560_e9068;
        locals.var_t2_dn0 = assign14560_e9068_d_n0;
        locals.var_t2_dn2 = assign14560_e9068_d_n2;
        locals.var_t2_dn4 = assign14560_e9068_d_n4;
        locals.var_t2_dn5 = assign14560_e9068_d_n5;
        locals.var_t2_dn6 = assign14560_e9068_d_n6;
        locals.var_t2_dn7 = assign14560_e9068_d_n7;
        locals.var_t2_dn8 = assign14560_e9068_d_n8;
        locals.var_t2_dn9 = assign14560_e9068_d_n9;
        locals.var_t2_dn10 = assign14560_e9068_d_n10;
        locals.var_t2_dn11 = assign14560_e9068_d_n11;
        locals.var_t2_dn14 = assign14560_e9068_d_n14;

        let (assign14570_e9078, assign14570_e9078_d_n0, assign14570_e9078_d_n2, assign14570_e9078_d_n4, assign14570_e9078_d_n5, assign14570_e9078_d_n6, assign14570_e9078_d_n7, assign14570_e9078_d_n8, assign14570_e9078_d_n9, assign14570_e9078_d_n10, assign14570_e9078_d_n11, assign14570_e9078_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard321 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign14570_e9078;
        locals.var_t0_dn0 = assign14570_e9078_d_n0;
        locals.var_t0_dn2 = assign14570_e9078_d_n2;
        locals.var_t0_dn4 = assign14570_e9078_d_n4;
        locals.var_t0_dn5 = assign14570_e9078_d_n5;
        locals.var_t0_dn6 = assign14570_e9078_d_n6;
        locals.var_t0_dn7 = assign14570_e9078_d_n7;
        locals.var_t0_dn8 = assign14570_e9078_d_n8;
        locals.var_t0_dn9 = assign14570_e9078_d_n9;
        locals.var_t0_dn10 = assign14570_e9078_d_n10;
        locals.var_t0_dn11 = assign14570_e9078_d_n11;
        locals.var_t0_dn14 = assign14570_e9078_d_n14;

        let (assign14580_e9089, assign14580_e9089_d_n0, assign14580_e9089_d_n2, assign14580_e9089_d_n4, assign14580_e9089_d_n5, assign14580_e9089_d_n6, assign14580_e9089_d_n7, assign14580_e9089_d_n8, assign14580_e9089_d_n9, assign14580_e9089_d_n10, assign14580_e9089_d_n11, assign14580_e9089_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14580_e9085: f64 = (-p.p98);
        let assign14580_e9087: f64 = (assign14580_e9085 / locals.var_t2);
        (assign14580_e9087, (-((assign14580_e9085 * locals.var_t2_dn0) / (locals.var_t2 * locals.var_t2))), (-((assign14580_e9085 * locals.var_t2_dn2) / (locals.var_t2 * locals.var_t2))), (-((assign14580_e9085 * locals.var_t2_dn4) / (locals.var_t2 * locals.var_t2))), (-((assign14580_e9085 * locals.var_t2_dn5) / (locals.var_t2 * locals.var_t2))), (-((assign14580_e9085 * locals.var_t2_dn6) / (locals.var_t2 * locals.var_t2))), (-((assign14580_e9085 * locals.var_t2_dn7) / (locals.var_t2 * locals.var_t2))), (-((assign14580_e9085 * locals.var_t2_dn8) / (locals.var_t2 * locals.var_t2))), (-((assign14580_e9085 * locals.var_t2_dn9) / (locals.var_t2 * locals.var_t2))), (-((assign14580_e9085 * locals.var_t2_dn10) / (locals.var_t2 * locals.var_t2))), (-((assign14580_e9085 * locals.var_t2_dn11) / (locals.var_t2 * locals.var_t2))), (-((assign14580_e9085 * locals.var_t2_dn14) / (locals.var_t2 * locals.var_t2))),)
    } else {
        (locals.var_t8, locals.var_t8_dn0, locals.var_t8_dn2, locals.var_t8_dn4, locals.var_t8_dn5, locals.var_t8_dn6, locals.var_t8_dn7, locals.var_t8_dn8, locals.var_t8_dn9, locals.var_t8_dn10, locals.var_t8_dn11, locals.var_t8_dn14,)
    }
};
        locals.var_t8 = assign14580_e9089;
        locals.var_t8_dn0 = assign14580_e9089_d_n0;
        locals.var_t8_dn2 = assign14580_e9089_d_n2;
        locals.var_t8_dn4 = assign14580_e9089_d_n4;
        locals.var_t8_dn5 = assign14580_e9089_d_n5;
        locals.var_t8_dn6 = assign14580_e9089_d_n6;
        locals.var_t8_dn7 = assign14580_e9089_d_n7;
        locals.var_t8_dn8 = assign14580_e9089_d_n8;
        locals.var_t8_dn9 = assign14580_e9089_d_n9;
        locals.var_t8_dn10 = assign14580_e9089_d_n10;
        locals.var_t8_dn11 = assign14580_e9089_d_n11;
        locals.var_t8_dn14 = assign14580_e9089_d_n14;

        let (assign14590_e9105, assign14590_e9105_d_n0, assign14590_e9105_d_n2, assign14590_e9105_d_n4, assign14590_e9105_d_n5, assign14590_e9105_d_n6, assign14590_e9105_d_n7, assign14590_e9105_d_n8, assign14590_e9105_d_n9, assign14590_e9105_d_n10, assign14590_e9105_d_n11, assign14590_e9105_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14590_e9097: f64 = (locals.var_t8 * p.p63);
        let assign14590_e9099: f64 = (assign14590_e9097 * 1000000.0);
        let assign14590_e9101: f64 = (assign14590_e9099 + 1.0);
        let assign14590_e9103: f64 = (assign14590_e9101 + p.p98);
        (assign14590_e9103, ((locals.var_t8_dn0 * p.p63) * 1000000.0), ((locals.var_t8_dn2 * p.p63) * 1000000.0), ((locals.var_t8_dn4 * p.p63) * 1000000.0), ((locals.var_t8_dn5 * p.p63) * 1000000.0), ((locals.var_t8_dn6 * p.p63) * 1000000.0), ((locals.var_t8_dn7 * p.p63) * 1000000.0), ((locals.var_t8_dn8 * p.p63) * 1000000.0), ((locals.var_t8_dn9 * p.p63) * 1000000.0), ((locals.var_t8_dn10 * p.p63) * 1000000.0), ((locals.var_t8_dn11 * p.p63) * 1000000.0), ((locals.var_t8_dn14 * p.p63) * 1000000.0),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign14590_e9105;
        locals.var_t3_dn0 = assign14590_e9105_d_n0;
        locals.var_t3_dn2 = assign14590_e9105_d_n2;
        locals.var_t3_dn4 = assign14590_e9105_d_n4;
        locals.var_t3_dn5 = assign14590_e9105_d_n5;
        locals.var_t3_dn6 = assign14590_e9105_d_n6;
        locals.var_t3_dn7 = assign14590_e9105_d_n7;
        locals.var_t3_dn8 = assign14590_e9105_d_n8;
        locals.var_t3_dn9 = assign14590_e9105_d_n9;
        locals.var_t3_dn10 = assign14590_e9105_d_n10;
        locals.var_t3_dn11 = assign14590_e9105_d_n11;
        locals.var_t3_dn14 = assign14590_e9105_d_n14;

        let (assign14600_e9119, assign14600_e9119_d_n0, assign14600_e9119_d_n2, assign14600_e9119_d_n4, assign14600_e9119_d_n5, assign14600_e9119_d_n6, assign14600_e9119_d_n7, assign14600_e9119_d_n8, assign14600_e9119_d_n9, assign14600_e9119_d_n10, assign14600_e9119_d_n11, assign14600_e9119_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14600_e9113: f64 = (locals.var_t3 * locals.var_t4);
        let assign14600_e9115: f64 = (assign14600_e9113 - locals.var_t4);
        let assign14600_e9117: f64 = (assign14600_e9115 - 0.01);
        (assign14600_e9117, (((locals.var_t3_dn0 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn0)) - locals.var_t4_dn0), (((locals.var_t3_dn2 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn2)) - locals.var_t4_dn2), (((locals.var_t3_dn4 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn4)) - locals.var_t4_dn4), (((locals.var_t3_dn5 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn5)) - locals.var_t4_dn5), (((locals.var_t3_dn6 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn6)) - locals.var_t4_dn6), (((locals.var_t3_dn7 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn7)) - locals.var_t4_dn7), (((locals.var_t3_dn8 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn8)) - locals.var_t4_dn8), (((locals.var_t3_dn9 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn9)) - locals.var_t4_dn9), (((locals.var_t3_dn10 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn10)) - locals.var_t4_dn10), (((locals.var_t3_dn11 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn11)) - locals.var_t4_dn11), (((locals.var_t3_dn14 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn14)) - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign14600_e9119;
        locals.var_tmf1_dn0 = assign14600_e9119_d_n0;
        locals.var_tmf1_dn2 = assign14600_e9119_d_n2;
        locals.var_tmf1_dn4 = assign14600_e9119_d_n4;
        locals.var_tmf1_dn5 = assign14600_e9119_d_n5;
        locals.var_tmf1_dn6 = assign14600_e9119_d_n6;
        locals.var_tmf1_dn7 = assign14600_e9119_d_n7;
        locals.var_tmf1_dn8 = assign14600_e9119_d_n8;
        locals.var_tmf1_dn9 = assign14600_e9119_d_n9;
        locals.var_tmf1_dn10 = assign14600_e9119_d_n10;
        locals.var_tmf1_dn11 = assign14600_e9119_d_n11;
        locals.var_tmf1_dn14 = assign14600_e9119_d_n14;

        let (assign14610_e9131, assign14610_e9131_d_n0, assign14610_e9131_d_n2, assign14610_e9131_d_n4, assign14610_e9131_d_n5, assign14610_e9131_d_n6, assign14610_e9131_d_n7, assign14610_e9131_d_n8, assign14610_e9131_d_n9, assign14610_e9131_d_n10, assign14610_e9131_d_n11, assign14610_e9131_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14610_e9127: f64 = (4.0 * locals.var_t4);
        let assign14610_e9129: f64 = (assign14610_e9127 * 0.01);
        (assign14610_e9129, ((4.0 * locals.var_t4_dn0) * 0.01), ((4.0 * locals.var_t4_dn2) * 0.01), ((4.0 * locals.var_t4_dn4) * 0.01), ((4.0 * locals.var_t4_dn5) * 0.01), ((4.0 * locals.var_t4_dn6) * 0.01), ((4.0 * locals.var_t4_dn7) * 0.01), ((4.0 * locals.var_t4_dn8) * 0.01), ((4.0 * locals.var_t4_dn9) * 0.01), ((4.0 * locals.var_t4_dn10) * 0.01), ((4.0 * locals.var_t4_dn11) * 0.01), ((4.0 * locals.var_t4_dn14) * 0.01),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14610_e9131;
        locals.var_tmf2_dn0 = assign14610_e9131_d_n0;
        locals.var_tmf2_dn2 = assign14610_e9131_d_n2;
        locals.var_tmf2_dn4 = assign14610_e9131_d_n4;
        locals.var_tmf2_dn5 = assign14610_e9131_d_n5;
        locals.var_tmf2_dn6 = assign14610_e9131_d_n6;
        locals.var_tmf2_dn7 = assign14610_e9131_d_n7;
        locals.var_tmf2_dn8 = assign14610_e9131_d_n8;
        locals.var_tmf2_dn9 = assign14610_e9131_d_n9;
        locals.var_tmf2_dn10 = assign14610_e9131_d_n10;
        locals.var_tmf2_dn11 = assign14610_e9131_d_n11;
        locals.var_tmf2_dn14 = assign14610_e9131_d_n14;

        let (assign14620_e9145, assign14620_e9145_d_n0, assign14620_e9145_d_n2, assign14620_e9145_d_n4, assign14620_e9145_d_n5, assign14620_e9145_d_n6, assign14620_e9145_d_n7, assign14620_e9145_d_n8, assign14620_e9145_d_n9, assign14620_e9145_d_n10, assign14620_e9145_d_n11, assign14620_e9145_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let (assign14620_e9143, assign14620_e9143_d_n0, assign14620_e9143_d_n2, assign14620_e9143_d_n4, assign14620_e9143_d_n5, assign14620_e9143_d_n6, assign14620_e9143_d_n7, assign14620_e9143_d_n8, assign14620_e9143_d_n9, assign14620_e9143_d_n10, assign14620_e9143_d_n11, assign14620_e9143_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign14620_e9142: f64 = (-locals.var_tmf2);
                (assign14620_e9142, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign14620_e9143, assign14620_e9143_d_n0, assign14620_e9143_d_n2, assign14620_e9143_d_n4, assign14620_e9143_d_n5, assign14620_e9143_d_n6, assign14620_e9143_d_n7, assign14620_e9143_d_n8, assign14620_e9143_d_n9, assign14620_e9143_d_n10, assign14620_e9143_d_n11, assign14620_e9143_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14620_e9145;
        locals.var_tmf2_dn0 = assign14620_e9145_d_n0;
        locals.var_tmf2_dn2 = assign14620_e9145_d_n2;
        locals.var_tmf2_dn4 = assign14620_e9145_d_n4;
        locals.var_tmf2_dn5 = assign14620_e9145_d_n5;
        locals.var_tmf2_dn6 = assign14620_e9145_d_n6;
        locals.var_tmf2_dn7 = assign14620_e9145_d_n7;
        locals.var_tmf2_dn8 = assign14620_e9145_d_n8;
        locals.var_tmf2_dn9 = assign14620_e9145_d_n9;
        locals.var_tmf2_dn10 = assign14620_e9145_d_n10;
        locals.var_tmf2_dn11 = assign14620_e9145_d_n11;
        locals.var_tmf2_dn14 = assign14620_e9145_d_n14;

        let (assign14630_e9158, assign14630_e9158_d_n0, assign14630_e9158_d_n2, assign14630_e9158_d_n4, assign14630_e9158_d_n5, assign14630_e9158_d_n6, assign14630_e9158_d_n7, assign14630_e9158_d_n8, assign14630_e9158_d_n9, assign14630_e9158_d_n10, assign14630_e9158_d_n11, assign14630_e9158_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14630_e9153: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14630_e9155: f64 = (assign14630_e9153 + locals.var_tmf2);
        let assign14630_e9156: f64 = (assign14630_e9155).sqrt();
        (assign14630_e9156, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14630_e9156)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14630_e9156)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14630_e9156)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14630_e9156)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14630_e9156)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14630_e9156)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14630_e9156)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14630_e9156)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14630_e9156)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14630_e9156)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign14630_e9156)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14630_e9158;
        locals.var_tmf2_dn0 = assign14630_e9158_d_n0;
        locals.var_tmf2_dn2 = assign14630_e9158_d_n2;
        locals.var_tmf2_dn4 = assign14630_e9158_d_n4;
        locals.var_tmf2_dn5 = assign14630_e9158_d_n5;
        locals.var_tmf2_dn6 = assign14630_e9158_d_n6;
        locals.var_tmf2_dn7 = assign14630_e9158_d_n7;
        locals.var_tmf2_dn8 = assign14630_e9158_d_n8;
        locals.var_tmf2_dn9 = assign14630_e9158_d_n9;
        locals.var_tmf2_dn10 = assign14630_e9158_d_n10;
        locals.var_tmf2_dn11 = assign14630_e9158_d_n11;
        locals.var_tmf2_dn14 = assign14630_e9158_d_n14;

        let (assign14640_e9172, assign14640_e9172_d_n0, assign14640_e9172_d_n2, assign14640_e9172_d_n4, assign14640_e9172_d_n5, assign14640_e9172_d_n6, assign14640_e9172_d_n7, assign14640_e9172_d_n8, assign14640_e9172_d_n9, assign14640_e9172_d_n10, assign14640_e9172_d_n11, assign14640_e9172_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14640_e9168: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14640_e9169: f64 = (1.0 + assign14640_e9168);
        let assign14640_e9170: f64 = (0.5 * assign14640_e9169);
        (assign14640_e9170, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign14640_e9172;
        locals.var_t6_dn0 = assign14640_e9172_d_n0;
        locals.var_t6_dn2 = assign14640_e9172_d_n2;
        locals.var_t6_dn4 = assign14640_e9172_d_n4;
        locals.var_t6_dn5 = assign14640_e9172_d_n5;
        locals.var_t6_dn6 = assign14640_e9172_d_n6;
        locals.var_t6_dn7 = assign14640_e9172_d_n7;
        locals.var_t6_dn8 = assign14640_e9172_d_n8;
        locals.var_t6_dn9 = assign14640_e9172_d_n9;
        locals.var_t6_dn10 = assign14640_e9172_d_n10;
        locals.var_t6_dn11 = assign14640_e9172_d_n11;
        locals.var_t6_dn14 = assign14640_e9172_d_n14;

        let (assign14650_e9186, assign14650_e9186_d_n0, assign14650_e9186_d_n2, assign14650_e9186_d_n4, assign14650_e9186_d_n5, assign14650_e9186_d_n6, assign14650_e9186_d_n7, assign14650_e9186_d_n8, assign14650_e9186_d_n9, assign14650_e9186_d_n10, assign14650_e9186_d_n11, assign14650_e9186_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14650_e9182: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14650_e9183: f64 = (0.5 * assign14650_e9182);
        let assign14650_e9184: f64 = (locals.var_t4 + assign14650_e9183);
        (assign14650_e9184, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t4_dn14 + (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign14650_e9186;
        locals.var_t5_dn0 = assign14650_e9186_d_n0;
        locals.var_t5_dn2 = assign14650_e9186_d_n2;
        locals.var_t5_dn4 = assign14650_e9186_d_n4;
        locals.var_t5_dn5 = assign14650_e9186_d_n5;
        locals.var_t5_dn6 = assign14650_e9186_d_n6;
        locals.var_t5_dn7 = assign14650_e9186_d_n7;
        locals.var_t5_dn8 = assign14650_e9186_d_n8;
        locals.var_t5_dn9 = assign14650_e9186_d_n9;
        locals.var_t5_dn10 = assign14650_e9186_d_n10;
        locals.var_t5_dn11 = assign14650_e9186_d_n11;
        locals.var_t5_dn14 = assign14650_e9186_d_n14;

        let (assign14660_e9202, assign14660_e9202_d_n0, assign14660_e9202_d_n2, assign14660_e9202_d_n4, assign14660_e9202_d_n5, assign14660_e9202_d_n6, assign14660_e9202_d_n7, assign14660_e9202_d_n8, assign14660_e9202_d_n9, assign14660_e9202_d_n10, assign14660_e9202_d_n11, assign14660_e9202_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14660_e9195: f64 = (p.p98 + 1.0);
        let assign14660_e9196: f64 = (locals.var_t4 * assign14660_e9195);
        let assign14660_e9198: f64 = (assign14660_e9196 - locals.var_t5);
        let assign14660_e9200: f64 = (assign14660_e9198 - 5e-5);
        (assign14660_e9200, ((locals.var_t4_dn0 * assign14660_e9195) - locals.var_t5_dn0), ((locals.var_t4_dn2 * assign14660_e9195) - locals.var_t5_dn2), ((locals.var_t4_dn4 * assign14660_e9195) - locals.var_t5_dn4), ((locals.var_t4_dn5 * assign14660_e9195) - locals.var_t5_dn5), ((locals.var_t4_dn6 * assign14660_e9195) - locals.var_t5_dn6), ((locals.var_t4_dn7 * assign14660_e9195) - locals.var_t5_dn7), ((locals.var_t4_dn8 * assign14660_e9195) - locals.var_t5_dn8), ((locals.var_t4_dn9 * assign14660_e9195) - locals.var_t5_dn9), ((locals.var_t4_dn10 * assign14660_e9195) - locals.var_t5_dn10), ((locals.var_t4_dn11 * assign14660_e9195) - locals.var_t5_dn11), ((locals.var_t4_dn14 * assign14660_e9195) - locals.var_t5_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign14660_e9202;
        locals.var_tmf1_dn0 = assign14660_e9202_d_n0;
        locals.var_tmf1_dn2 = assign14660_e9202_d_n2;
        locals.var_tmf1_dn4 = assign14660_e9202_d_n4;
        locals.var_tmf1_dn5 = assign14660_e9202_d_n5;
        locals.var_tmf1_dn6 = assign14660_e9202_d_n6;
        locals.var_tmf1_dn7 = assign14660_e9202_d_n7;
        locals.var_tmf1_dn8 = assign14660_e9202_d_n8;
        locals.var_tmf1_dn9 = assign14660_e9202_d_n9;
        locals.var_tmf1_dn10 = assign14660_e9202_d_n10;
        locals.var_tmf1_dn11 = assign14660_e9202_d_n11;
        locals.var_tmf1_dn14 = assign14660_e9202_d_n14;

        let (assign14670_e9218, assign14670_e9218_d_n0, assign14670_e9218_d_n2, assign14670_e9218_d_n4, assign14670_e9218_d_n5, assign14670_e9218_d_n6, assign14670_e9218_d_n7, assign14670_e9218_d_n8, assign14670_e9218_d_n9, assign14670_e9218_d_n10, assign14670_e9218_d_n11, assign14670_e9218_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14670_e9212: f64 = (p.p98 + 1.0);
        let assign14670_e9213: f64 = (locals.var_t4 * assign14670_e9212);
        let assign14670_e9214: f64 = (4.0 * assign14670_e9213);
        let assign14670_e9216: f64 = (assign14670_e9214 * 5e-5);
        (assign14670_e9216, ((4.0 * (locals.var_t4_dn0 * assign14670_e9212)) * 5e-5), ((4.0 * (locals.var_t4_dn2 * assign14670_e9212)) * 5e-5), ((4.0 * (locals.var_t4_dn4 * assign14670_e9212)) * 5e-5), ((4.0 * (locals.var_t4_dn5 * assign14670_e9212)) * 5e-5), ((4.0 * (locals.var_t4_dn6 * assign14670_e9212)) * 5e-5), ((4.0 * (locals.var_t4_dn7 * assign14670_e9212)) * 5e-5), ((4.0 * (locals.var_t4_dn8 * assign14670_e9212)) * 5e-5), ((4.0 * (locals.var_t4_dn9 * assign14670_e9212)) * 5e-5), ((4.0 * (locals.var_t4_dn10 * assign14670_e9212)) * 5e-5), ((4.0 * (locals.var_t4_dn11 * assign14670_e9212)) * 5e-5), ((4.0 * (locals.var_t4_dn14 * assign14670_e9212)) * 5e-5),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14670_e9218;
        locals.var_tmf2_dn0 = assign14670_e9218_d_n0;
        locals.var_tmf2_dn2 = assign14670_e9218_d_n2;
        locals.var_tmf2_dn4 = assign14670_e9218_d_n4;
        locals.var_tmf2_dn5 = assign14670_e9218_d_n5;
        locals.var_tmf2_dn6 = assign14670_e9218_d_n6;
        locals.var_tmf2_dn7 = assign14670_e9218_d_n7;
        locals.var_tmf2_dn8 = assign14670_e9218_d_n8;
        locals.var_tmf2_dn9 = assign14670_e9218_d_n9;
        locals.var_tmf2_dn10 = assign14670_e9218_d_n10;
        locals.var_tmf2_dn11 = assign14670_e9218_d_n11;
        locals.var_tmf2_dn14 = assign14670_e9218_d_n14;

    }

    pub(super) fn stamp_transient_block_28(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14680_e9232, assign14680_e9232_d_n0, assign14680_e9232_d_n2, assign14680_e9232_d_n4, assign14680_e9232_d_n5, assign14680_e9232_d_n6, assign14680_e9232_d_n7, assign14680_e9232_d_n8, assign14680_e9232_d_n9, assign14680_e9232_d_n10, assign14680_e9232_d_n11, assign14680_e9232_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let (assign14680_e9230, assign14680_e9230_d_n0, assign14680_e9230_d_n2, assign14680_e9230_d_n4, assign14680_e9230_d_n5, assign14680_e9230_d_n6, assign14680_e9230_d_n7, assign14680_e9230_d_n8, assign14680_e9230_d_n9, assign14680_e9230_d_n10, assign14680_e9230_d_n11, assign14680_e9230_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign14680_e9229: f64 = (-locals.var_tmf2);
                (assign14680_e9229, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign14680_e9230, assign14680_e9230_d_n0, assign14680_e9230_d_n2, assign14680_e9230_d_n4, assign14680_e9230_d_n5, assign14680_e9230_d_n6, assign14680_e9230_d_n7, assign14680_e9230_d_n8, assign14680_e9230_d_n9, assign14680_e9230_d_n10, assign14680_e9230_d_n11, assign14680_e9230_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14680_e9232;
        locals.var_tmf2_dn0 = assign14680_e9232_d_n0;
        locals.var_tmf2_dn2 = assign14680_e9232_d_n2;
        locals.var_tmf2_dn4 = assign14680_e9232_d_n4;
        locals.var_tmf2_dn5 = assign14680_e9232_d_n5;
        locals.var_tmf2_dn6 = assign14680_e9232_d_n6;
        locals.var_tmf2_dn7 = assign14680_e9232_d_n7;
        locals.var_tmf2_dn8 = assign14680_e9232_d_n8;
        locals.var_tmf2_dn9 = assign14680_e9232_d_n9;
        locals.var_tmf2_dn10 = assign14680_e9232_d_n10;
        locals.var_tmf2_dn11 = assign14680_e9232_d_n11;
        locals.var_tmf2_dn14 = assign14680_e9232_d_n14;

        let (assign14690_e9245, assign14690_e9245_d_n0, assign14690_e9245_d_n2, assign14690_e9245_d_n4, assign14690_e9245_d_n5, assign14690_e9245_d_n6, assign14690_e9245_d_n7, assign14690_e9245_d_n8, assign14690_e9245_d_n9, assign14690_e9245_d_n10, assign14690_e9245_d_n11, assign14690_e9245_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14690_e9240: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14690_e9242: f64 = (assign14690_e9240 + locals.var_tmf2);
        let assign14690_e9243: f64 = (assign14690_e9242).sqrt();
        (assign14690_e9243, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14690_e9243)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14690_e9243)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14690_e9243)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14690_e9243)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14690_e9243)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14690_e9243)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14690_e9243)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14690_e9243)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14690_e9243)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14690_e9243)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign14690_e9243)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14690_e9245;
        locals.var_tmf2_dn0 = assign14690_e9245_d_n0;
        locals.var_tmf2_dn2 = assign14690_e9245_d_n2;
        locals.var_tmf2_dn4 = assign14690_e9245_d_n4;
        locals.var_tmf2_dn5 = assign14690_e9245_d_n5;
        locals.var_tmf2_dn6 = assign14690_e9245_d_n6;
        locals.var_tmf2_dn7 = assign14690_e9245_d_n7;
        locals.var_tmf2_dn8 = assign14690_e9245_d_n8;
        locals.var_tmf2_dn9 = assign14690_e9245_d_n9;
        locals.var_tmf2_dn10 = assign14690_e9245_d_n10;
        locals.var_tmf2_dn11 = assign14690_e9245_d_n11;
        locals.var_tmf2_dn14 = assign14690_e9245_d_n14;

        let (assign14700_e9259, assign14700_e9259_d_n0, assign14700_e9259_d_n2, assign14700_e9259_d_n4, assign14700_e9259_d_n5, assign14700_e9259_d_n6, assign14700_e9259_d_n7, assign14700_e9259_d_n8, assign14700_e9259_d_n9, assign14700_e9259_d_n10, assign14700_e9259_d_n11, assign14700_e9259_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14700_e9255: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14700_e9256: f64 = (1.0 + assign14700_e9255);
        let assign14700_e9257: f64 = (0.5 * assign14700_e9256);
        (assign14700_e9257, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign14700_e9259;
        locals.var_t6_dn0 = assign14700_e9259_d_n0;
        locals.var_t6_dn2 = assign14700_e9259_d_n2;
        locals.var_t6_dn4 = assign14700_e9259_d_n4;
        locals.var_t6_dn5 = assign14700_e9259_d_n5;
        locals.var_t6_dn6 = assign14700_e9259_d_n6;
        locals.var_t6_dn7 = assign14700_e9259_d_n7;
        locals.var_t6_dn8 = assign14700_e9259_d_n8;
        locals.var_t6_dn9 = assign14700_e9259_d_n9;
        locals.var_t6_dn10 = assign14700_e9259_d_n10;
        locals.var_t6_dn11 = assign14700_e9259_d_n11;
        locals.var_t6_dn14 = assign14700_e9259_d_n14;

        let (assign14710_e9277, assign14710_e9277_d_n0, assign14710_e9277_d_n2, assign14710_e9277_d_n4, assign14710_e9277_d_n5, assign14710_e9277_d_n6, assign14710_e9277_d_n7, assign14710_e9277_d_n8, assign14710_e9277_d_n9, assign14710_e9277_d_n10, assign14710_e9277_d_n11, assign14710_e9277_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14710_e9268: f64 = (p.p98 + 1.0);
        let assign14710_e9269: f64 = (locals.var_t4 * assign14710_e9268);
        let assign14710_e9273: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14710_e9274: f64 = (0.5 * assign14710_e9273);
        let assign14710_e9275: f64 = (assign14710_e9269 - assign14710_e9274);
        (assign14710_e9275, ((locals.var_t4_dn0 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((locals.var_t4_dn2 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((locals.var_t4_dn4 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((locals.var_t4_dn5 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((locals.var_t4_dn6 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((locals.var_t4_dn7 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((locals.var_t4_dn8 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((locals.var_t4_dn9 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((locals.var_t4_dn10 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((locals.var_t4_dn11 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((locals.var_t4_dn14 * assign14710_e9268) - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign14710_e9277;
        locals.var_t7_dn0 = assign14710_e9277_d_n0;
        locals.var_t7_dn2 = assign14710_e9277_d_n2;
        locals.var_t7_dn4 = assign14710_e9277_d_n4;
        locals.var_t7_dn5 = assign14710_e9277_d_n5;
        locals.var_t7_dn6 = assign14710_e9277_d_n6;
        locals.var_t7_dn7 = assign14710_e9277_d_n7;
        locals.var_t7_dn8 = assign14710_e9277_d_n8;
        locals.var_t7_dn9 = assign14710_e9277_d_n9;
        locals.var_t7_dn10 = assign14710_e9277_d_n10;
        locals.var_t7_dn11 = assign14710_e9277_d_n11;
        locals.var_t7_dn14 = assign14710_e9277_d_n14;

        let (assign14720_e9293, assign14720_e9293_d_n0, assign14720_e9293_d_n2, assign14720_e9293_d_n4, assign14720_e9293_d_n5, assign14720_e9293_d_n6, assign14720_e9293_d_n7, assign14720_e9293_d_n8, assign14720_e9293_d_n9, assign14720_e9293_d_n10, assign14720_e9293_d_n11, assign14720_e9293_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14720_e9286: f64 = (locals.var_t1 * locals.var_t4);
        let assign14720_e9287: f64 = (locals.var_t7 + assign14720_e9286);
        let assign14720_e9289: f64 = assign14720_e9287;
        let assign14720_e9291: f64 = (assign14720_e9289 - 5e-5);
        (assign14720_e9291, (locals.var_t7_dn0 + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))), (locals.var_t7_dn2 + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))), (locals.var_t7_dn4 + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))), (locals.var_t7_dn5 + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))), (locals.var_t7_dn6 + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))), (locals.var_t7_dn7 + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))), (locals.var_t7_dn8 + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))), (locals.var_t7_dn9 + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))), (locals.var_t7_dn10 + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))), (locals.var_t7_dn11 + ((locals.var_t1_dn11 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn11))), (locals.var_t7_dn14 + ((locals.var_t1_dn14 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn14))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign14720_e9293;
        locals.var_tmf1_dn0 = assign14720_e9293_d_n0;
        locals.var_tmf1_dn2 = assign14720_e9293_d_n2;
        locals.var_tmf1_dn4 = assign14720_e9293_d_n4;
        locals.var_tmf1_dn5 = assign14720_e9293_d_n5;
        locals.var_tmf1_dn6 = assign14720_e9293_d_n6;
        locals.var_tmf1_dn7 = assign14720_e9293_d_n7;
        locals.var_tmf1_dn8 = assign14720_e9293_d_n8;
        locals.var_tmf1_dn9 = assign14720_e9293_d_n9;
        locals.var_tmf1_dn10 = assign14720_e9293_d_n10;
        locals.var_tmf1_dn11 = assign14720_e9293_d_n11;
        locals.var_tmf1_dn14 = assign14720_e9293_d_n14;

        let (assign14730_e9305, assign14730_e9305_d_n0, assign14730_e9305_d_n2, assign14730_e9305_d_n4, assign14730_e9305_d_n5, assign14730_e9305_d_n6, assign14730_e9305_d_n7, assign14730_e9305_d_n8, assign14730_e9305_d_n9, assign14730_e9305_d_n10, assign14730_e9305_d_n11, assign14730_e9305_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14730_e9305;
        locals.var_tmf2_dn0 = assign14730_e9305_d_n0;
        locals.var_tmf2_dn2 = assign14730_e9305_d_n2;
        locals.var_tmf2_dn4 = assign14730_e9305_d_n4;
        locals.var_tmf2_dn5 = assign14730_e9305_d_n5;
        locals.var_tmf2_dn6 = assign14730_e9305_d_n6;
        locals.var_tmf2_dn7 = assign14730_e9305_d_n7;
        locals.var_tmf2_dn8 = assign14730_e9305_d_n8;
        locals.var_tmf2_dn9 = assign14730_e9305_d_n9;
        locals.var_tmf2_dn10 = assign14730_e9305_d_n10;
        locals.var_tmf2_dn11 = assign14730_e9305_d_n11;
        locals.var_tmf2_dn14 = assign14730_e9305_d_n14;

        let (assign14740_e9319, assign14740_e9319_d_n0, assign14740_e9319_d_n2, assign14740_e9319_d_n4, assign14740_e9319_d_n5, assign14740_e9319_d_n6, assign14740_e9319_d_n7, assign14740_e9319_d_n8, assign14740_e9319_d_n9, assign14740_e9319_d_n10, assign14740_e9319_d_n11, assign14740_e9319_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let (assign14740_e9317, assign14740_e9317_d_n0, assign14740_e9317_d_n2, assign14740_e9317_d_n4, assign14740_e9317_d_n5, assign14740_e9317_d_n6, assign14740_e9317_d_n7, assign14740_e9317_d_n8, assign14740_e9317_d_n9, assign14740_e9317_d_n10, assign14740_e9317_d_n11, assign14740_e9317_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign14740_e9316: f64 = (-locals.var_tmf2);
                (assign14740_e9316, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign14740_e9317, assign14740_e9317_d_n0, assign14740_e9317_d_n2, assign14740_e9317_d_n4, assign14740_e9317_d_n5, assign14740_e9317_d_n6, assign14740_e9317_d_n7, assign14740_e9317_d_n8, assign14740_e9317_d_n9, assign14740_e9317_d_n10, assign14740_e9317_d_n11, assign14740_e9317_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14740_e9319;
        locals.var_tmf2_dn0 = assign14740_e9319_d_n0;
        locals.var_tmf2_dn2 = assign14740_e9319_d_n2;
        locals.var_tmf2_dn4 = assign14740_e9319_d_n4;
        locals.var_tmf2_dn5 = assign14740_e9319_d_n5;
        locals.var_tmf2_dn6 = assign14740_e9319_d_n6;
        locals.var_tmf2_dn7 = assign14740_e9319_d_n7;
        locals.var_tmf2_dn8 = assign14740_e9319_d_n8;
        locals.var_tmf2_dn9 = assign14740_e9319_d_n9;
        locals.var_tmf2_dn10 = assign14740_e9319_d_n10;
        locals.var_tmf2_dn11 = assign14740_e9319_d_n11;
        locals.var_tmf2_dn14 = assign14740_e9319_d_n14;

        let (assign14750_e9332, assign14750_e9332_d_n0, assign14750_e9332_d_n2, assign14750_e9332_d_n4, assign14750_e9332_d_n5, assign14750_e9332_d_n6, assign14750_e9332_d_n7, assign14750_e9332_d_n8, assign14750_e9332_d_n9, assign14750_e9332_d_n10, assign14750_e9332_d_n11, assign14750_e9332_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14750_e9327: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14750_e9329: f64 = (assign14750_e9327 + locals.var_tmf2);
        let assign14750_e9330: f64 = (assign14750_e9329).sqrt();
        (assign14750_e9330, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14750_e9330)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14750_e9330)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14750_e9330)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14750_e9330)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14750_e9330)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14750_e9330)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14750_e9330)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14750_e9330)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14750_e9330)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14750_e9330)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign14750_e9330)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14750_e9332;
        locals.var_tmf2_dn0 = assign14750_e9332_d_n0;
        locals.var_tmf2_dn2 = assign14750_e9332_d_n2;
        locals.var_tmf2_dn4 = assign14750_e9332_d_n4;
        locals.var_tmf2_dn5 = assign14750_e9332_d_n5;
        locals.var_tmf2_dn6 = assign14750_e9332_d_n6;
        locals.var_tmf2_dn7 = assign14750_e9332_d_n7;
        locals.var_tmf2_dn8 = assign14750_e9332_d_n8;
        locals.var_tmf2_dn9 = assign14750_e9332_d_n9;
        locals.var_tmf2_dn10 = assign14750_e9332_d_n10;
        locals.var_tmf2_dn11 = assign14750_e9332_d_n11;
        locals.var_tmf2_dn14 = assign14750_e9332_d_n14;

        let (assign14760_e9346, assign14760_e9346_d_n0, assign14760_e9346_d_n2, assign14760_e9346_d_n4, assign14760_e9346_d_n5, assign14760_e9346_d_n6, assign14760_e9346_d_n7, assign14760_e9346_d_n8, assign14760_e9346_d_n9, assign14760_e9346_d_n10, assign14760_e9346_d_n11, assign14760_e9346_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14760_e9342: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14760_e9343: f64 = (1.0 + assign14760_e9342);
        let assign14760_e9344: f64 = (0.5 * assign14760_e9343);
        (assign14760_e9344, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign14760_e9346;
        locals.var_t6_dn0 = assign14760_e9346_d_n0;
        locals.var_t6_dn2 = assign14760_e9346_d_n2;
        locals.var_t6_dn4 = assign14760_e9346_d_n4;
        locals.var_t6_dn5 = assign14760_e9346_d_n5;
        locals.var_t6_dn6 = assign14760_e9346_d_n6;
        locals.var_t6_dn7 = assign14760_e9346_d_n7;
        locals.var_t6_dn8 = assign14760_e9346_d_n8;
        locals.var_t6_dn9 = assign14760_e9346_d_n9;
        locals.var_t6_dn10 = assign14760_e9346_d_n10;
        locals.var_t6_dn11 = assign14760_e9346_d_n11;
        locals.var_t6_dn14 = assign14760_e9346_d_n14;

        let (assign14770_e9360, assign14770_e9360_d_n0, assign14770_e9360_d_n2, assign14770_e9360_d_n4, assign14770_e9360_d_n5, assign14770_e9360_d_n6, assign14770_e9360_d_n7, assign14770_e9360_d_n8, assign14770_e9360_d_n9, assign14770_e9360_d_n10, assign14770_e9360_d_n11, assign14770_e9360_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14770_e9356: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14770_e9357: f64 = (0.5 * assign14770_e9356);
        let assign14770_e9358: f64 = assign14770_e9357;
        (assign14770_e9358, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign14770_e9360;
        locals.var_t2_dn0 = assign14770_e9360_d_n0;
        locals.var_t2_dn2 = assign14770_e9360_d_n2;
        locals.var_t2_dn4 = assign14770_e9360_d_n4;
        locals.var_t2_dn5 = assign14770_e9360_d_n5;
        locals.var_t2_dn6 = assign14770_e9360_d_n6;
        locals.var_t2_dn7 = assign14770_e9360_d_n7;
        locals.var_t2_dn8 = assign14770_e9360_d_n8;
        locals.var_t2_dn9 = assign14770_e9360_d_n9;
        locals.var_t2_dn10 = assign14770_e9360_d_n10;
        locals.var_t2_dn11 = assign14770_e9360_d_n11;
        locals.var_t2_dn14 = assign14770_e9360_d_n14;

        let assign14780_e9367: f64 = if ((p.p39 == 0.0) || (p.p39 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard322 = assign14780_e9367;

        let (assign14790_e9387, assign14790_e9387_d_n0, assign14790_e9387_d_n2, assign14790_e9387_d_n4, assign14790_e9387_d_n5, assign14790_e9387_d_n6, assign14790_e9387_d_n7, assign14790_e9387_d_n8, assign14790_e9387_d_n9, assign14790_e9387_d_n10, assign14790_e9387_d_n11, assign14790_e9387_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 != 0.0)) {
        let assign14790_e9378: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff0);
        let assign14790_e9379: f64 = (locals.var_uc_rdvd + assign14790_e9378);
        let assign14790_e9382: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2);
        let assign14790_e9383: f64 = (assign14790_e9379 + assign14790_e9382);
        let assign14790_e9385: f64 = (assign14790_e9383 * locals.var_t2);
        (assign14790_e9385, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn11) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn11)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn11)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn14) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn14)) * locals.var_t2) + (assign14790_e9383 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign14790_e9387;
        locals.var_rdvde_dn0 = assign14790_e9387_d_n0;
        locals.var_rdvde_dn2 = assign14790_e9387_d_n2;
        locals.var_rdvde_dn4 = assign14790_e9387_d_n4;
        locals.var_rdvde_dn5 = assign14790_e9387_d_n5;
        locals.var_rdvde_dn6 = assign14790_e9387_d_n6;
        locals.var_rdvde_dn7 = assign14790_e9387_d_n7;
        locals.var_rdvde_dn8 = assign14790_e9387_d_n8;
        locals.var_rdvde_dn9 = assign14790_e9387_d_n9;
        locals.var_rdvde_dn10 = assign14790_e9387_d_n10;
        locals.var_rdvde_dn11 = assign14790_e9387_d_n11;
        locals.var_rdvde_dn14 = assign14790_e9387_d_n14;

        let (assign14800_e9405, assign14800_e9405_d_n0, assign14800_e9405_d_n2, assign14800_e9405_d_n4, assign14800_e9405_d_n5, assign14800_e9405_d_n6, assign14800_e9405_d_n7, assign14800_e9405_d_n8, assign14800_e9405_d_n9, assign14800_e9405_d_n10, assign14800_e9405_d_n11, assign14800_e9405_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 != 0.0)) {
        let assign14800_e9398: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14800_e9399: f64 = (locals.var_rdvde - assign14800_e9398);
        let assign14800_e9402: f64 = (0.01 * locals.var_uc_rdvd);
        let assign14800_e9403: f64 = (assign14800_e9399 - assign14800_e9402);
        (assign14800_e9403, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign14800_e9405;
        locals.var_tmf1_dn0 = assign14800_e9405_d_n0;
        locals.var_tmf1_dn2 = assign14800_e9405_d_n2;
        locals.var_tmf1_dn4 = assign14800_e9405_d_n4;
        locals.var_tmf1_dn5 = assign14800_e9405_d_n5;
        locals.var_tmf1_dn6 = assign14800_e9405_d_n6;
        locals.var_tmf1_dn7 = assign14800_e9405_d_n7;
        locals.var_tmf1_dn8 = assign14800_e9405_d_n8;
        locals.var_tmf1_dn9 = assign14800_e9405_d_n9;
        locals.var_tmf1_dn10 = assign14800_e9405_d_n10;
        locals.var_tmf1_dn11 = assign14800_e9405_d_n11;
        locals.var_tmf1_dn14 = assign14800_e9405_d_n14;

        let (assign14810_e9423, assign14810_e9423_d_n0, assign14810_e9423_d_n2, assign14810_e9423_d_n4, assign14810_e9423_d_n5, assign14810_e9423_d_n6, assign14810_e9423_d_n7, assign14810_e9423_d_n8, assign14810_e9423_d_n9, assign14810_e9423_d_n10, assign14810_e9423_d_n11, assign14810_e9423_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 != 0.0)) {
        let assign14810_e9416: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14810_e9417: f64 = (4.0 * assign14810_e9416);
        let assign14810_e9420: f64 = (0.01 * locals.var_uc_rdvd);
        let assign14810_e9421: f64 = (assign14810_e9417 * assign14810_e9420);
        (assign14810_e9421, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14810_e9423;
        locals.var_tmf2_dn0 = assign14810_e9423_d_n0;
        locals.var_tmf2_dn2 = assign14810_e9423_d_n2;
        locals.var_tmf2_dn4 = assign14810_e9423_d_n4;
        locals.var_tmf2_dn5 = assign14810_e9423_d_n5;
        locals.var_tmf2_dn6 = assign14810_e9423_d_n6;
        locals.var_tmf2_dn7 = assign14810_e9423_d_n7;
        locals.var_tmf2_dn8 = assign14810_e9423_d_n8;
        locals.var_tmf2_dn9 = assign14810_e9423_d_n9;
        locals.var_tmf2_dn10 = assign14810_e9423_d_n10;
        locals.var_tmf2_dn11 = assign14810_e9423_d_n11;
        locals.var_tmf2_dn14 = assign14810_e9423_d_n14;

        let (assign14820_e9439, assign14820_e9439_d_n0, assign14820_e9439_d_n2, assign14820_e9439_d_n4, assign14820_e9439_d_n5, assign14820_e9439_d_n6, assign14820_e9439_d_n7, assign14820_e9439_d_n8, assign14820_e9439_d_n9, assign14820_e9439_d_n10, assign14820_e9439_d_n11, assign14820_e9439_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 != 0.0)) {
        let (assign14820_e9437, assign14820_e9437_d_n0, assign14820_e9437_d_n2, assign14820_e9437_d_n4, assign14820_e9437_d_n5, assign14820_e9437_d_n6, assign14820_e9437_d_n7, assign14820_e9437_d_n8, assign14820_e9437_d_n9, assign14820_e9437_d_n10, assign14820_e9437_d_n11, assign14820_e9437_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign14820_e9436: f64 = (-locals.var_tmf2);
                (assign14820_e9436, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign14820_e9437, assign14820_e9437_d_n0, assign14820_e9437_d_n2, assign14820_e9437_d_n4, assign14820_e9437_d_n5, assign14820_e9437_d_n6, assign14820_e9437_d_n7, assign14820_e9437_d_n8, assign14820_e9437_d_n9, assign14820_e9437_d_n10, assign14820_e9437_d_n11, assign14820_e9437_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14820_e9439;
        locals.var_tmf2_dn0 = assign14820_e9439_d_n0;
        locals.var_tmf2_dn2 = assign14820_e9439_d_n2;
        locals.var_tmf2_dn4 = assign14820_e9439_d_n4;
        locals.var_tmf2_dn5 = assign14820_e9439_d_n5;
        locals.var_tmf2_dn6 = assign14820_e9439_d_n6;
        locals.var_tmf2_dn7 = assign14820_e9439_d_n7;
        locals.var_tmf2_dn8 = assign14820_e9439_d_n8;
        locals.var_tmf2_dn9 = assign14820_e9439_d_n9;
        locals.var_tmf2_dn10 = assign14820_e9439_d_n10;
        locals.var_tmf2_dn11 = assign14820_e9439_d_n11;
        locals.var_tmf2_dn14 = assign14820_e9439_d_n14;

        let (assign14830_e9454, assign14830_e9454_d_n0, assign14830_e9454_d_n2, assign14830_e9454_d_n4, assign14830_e9454_d_n5, assign14830_e9454_d_n6, assign14830_e9454_d_n7, assign14830_e9454_d_n8, assign14830_e9454_d_n9, assign14830_e9454_d_n10, assign14830_e9454_d_n11, assign14830_e9454_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 != 0.0)) {
        let assign14830_e9449: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14830_e9451: f64 = (assign14830_e9449 + locals.var_tmf2);
        let assign14830_e9452: f64 = (assign14830_e9451).sqrt();
        (assign14830_e9452, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14830_e9452)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14830_e9452)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14830_e9452)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14830_e9452)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14830_e9452)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14830_e9452)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14830_e9452)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14830_e9452)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14830_e9452)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14830_e9452)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign14830_e9452)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14830_e9454;
        locals.var_tmf2_dn0 = assign14830_e9454_d_n0;
        locals.var_tmf2_dn2 = assign14830_e9454_d_n2;
        locals.var_tmf2_dn4 = assign14830_e9454_d_n4;
        locals.var_tmf2_dn5 = assign14830_e9454_d_n5;
        locals.var_tmf2_dn6 = assign14830_e9454_d_n6;
        locals.var_tmf2_dn7 = assign14830_e9454_d_n7;
        locals.var_tmf2_dn8 = assign14830_e9454_d_n8;
        locals.var_tmf2_dn9 = assign14830_e9454_d_n9;
        locals.var_tmf2_dn10 = assign14830_e9454_d_n10;
        locals.var_tmf2_dn11 = assign14830_e9454_d_n11;
        locals.var_tmf2_dn14 = assign14830_e9454_d_n14;

        let (assign14840_e9470, assign14840_e9470_d_n0, assign14840_e9470_d_n2, assign14840_e9470_d_n4, assign14840_e9470_d_n5, assign14840_e9470_d_n6, assign14840_e9470_d_n7, assign14840_e9470_d_n8, assign14840_e9470_d_n9, assign14840_e9470_d_n10, assign14840_e9470_d_n11, assign14840_e9470_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 != 0.0)) {
        let assign14840_e9466: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14840_e9467: f64 = (1.0 + assign14840_e9466);
        let assign14840_e9468: f64 = (0.5 * assign14840_e9467);
        (assign14840_e9468, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign14840_e9470;
        locals.var_t0_dn0 = assign14840_e9470_d_n0;
        locals.var_t0_dn2 = assign14840_e9470_d_n2;
        locals.var_t0_dn4 = assign14840_e9470_d_n4;
        locals.var_t0_dn5 = assign14840_e9470_d_n5;
        locals.var_t0_dn6 = assign14840_e9470_d_n6;
        locals.var_t0_dn7 = assign14840_e9470_d_n7;
        locals.var_t0_dn8 = assign14840_e9470_d_n8;
        locals.var_t0_dn9 = assign14840_e9470_d_n9;
        locals.var_t0_dn10 = assign14840_e9470_d_n10;
        locals.var_t0_dn11 = assign14840_e9470_d_n11;
        locals.var_t0_dn14 = assign14840_e9470_d_n14;

        let (assign14850_e9488, assign14850_e9488_d_n0, assign14850_e9488_d_n2, assign14850_e9488_d_n4, assign14850_e9488_d_n5, assign14850_e9488_d_n6, assign14850_e9488_d_n7, assign14850_e9488_d_n8, assign14850_e9488_d_n9, assign14850_e9488_d_n10, assign14850_e9488_d_n11, assign14850_e9488_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 != 0.0)) {
        let assign14850_e9480: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14850_e9484: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14850_e9485: f64 = (0.5 * assign14850_e9484);
        let assign14850_e9486: f64 = (assign14850_e9480 + assign14850_e9485);
        (assign14850_e9486, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign14850_e9488;
        locals.var_rdvde_dn0 = assign14850_e9488_d_n0;
        locals.var_rdvde_dn2 = assign14850_e9488_d_n2;
        locals.var_rdvde_dn4 = assign14850_e9488_d_n4;
        locals.var_rdvde_dn5 = assign14850_e9488_d_n5;
        locals.var_rdvde_dn6 = assign14850_e9488_d_n6;
        locals.var_rdvde_dn7 = assign14850_e9488_d_n7;
        locals.var_rdvde_dn8 = assign14850_e9488_d_n8;
        locals.var_rdvde_dn9 = assign14850_e9488_d_n9;
        locals.var_rdvde_dn10 = assign14850_e9488_d_n10;
        locals.var_rdvde_dn11 = assign14850_e9488_d_n11;
        locals.var_rdvde_dn14 = assign14850_e9488_d_n14;

        let (assign14860_e9509, assign14860_e9509_d_n0, assign14860_e9509_d_n2, assign14860_e9509_d_n4, assign14860_e9509_d_n5, assign14860_e9509_d_n6, assign14860_e9509_d_n7, assign14860_e9509_d_n8, assign14860_e9509_d_n9, assign14860_e9509_d_n10, assign14860_e9509_d_n11, assign14860_e9509_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 == 0.0)) {
        let assign14860_e9500: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff);
        let assign14860_e9501: f64 = (locals.var_uc_rdvd + assign14860_e9500);
        let assign14860_e9504: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2);
        let assign14860_e9505: f64 = (assign14860_e9501 + assign14860_e9504);
        let assign14860_e9507: f64 = (assign14860_e9505 * locals.var_t2);
        (assign14860_e9507, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn11) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn11)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn11)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn14) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn14)) * locals.var_t2) + (assign14860_e9505 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign14860_e9509;
        locals.var_rdvde_dn0 = assign14860_e9509_d_n0;
        locals.var_rdvde_dn2 = assign14860_e9509_d_n2;
        locals.var_rdvde_dn4 = assign14860_e9509_d_n4;
        locals.var_rdvde_dn5 = assign14860_e9509_d_n5;
        locals.var_rdvde_dn6 = assign14860_e9509_d_n6;
        locals.var_rdvde_dn7 = assign14860_e9509_d_n7;
        locals.var_rdvde_dn8 = assign14860_e9509_d_n8;
        locals.var_rdvde_dn9 = assign14860_e9509_d_n9;
        locals.var_rdvde_dn10 = assign14860_e9509_d_n10;
        locals.var_rdvde_dn11 = assign14860_e9509_d_n11;
        locals.var_rdvde_dn14 = assign14860_e9509_d_n14;

        let (assign14870_e9528, assign14870_e9528_d_n0, assign14870_e9528_d_n2, assign14870_e9528_d_n4, assign14870_e9528_d_n5, assign14870_e9528_d_n6, assign14870_e9528_d_n7, assign14870_e9528_d_n8, assign14870_e9528_d_n9, assign14870_e9528_d_n10, assign14870_e9528_d_n11, assign14870_e9528_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 == 0.0)) {
        let assign14870_e9521: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14870_e9522: f64 = (locals.var_rdvde - assign14870_e9521);
        let assign14870_e9525: f64 = (0.01 * locals.var_uc_rdvd);
        let assign14870_e9526: f64 = (assign14870_e9522 - assign14870_e9525);
        (assign14870_e9526, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign14870_e9528;
        locals.var_tmf1_dn0 = assign14870_e9528_d_n0;
        locals.var_tmf1_dn2 = assign14870_e9528_d_n2;
        locals.var_tmf1_dn4 = assign14870_e9528_d_n4;
        locals.var_tmf1_dn5 = assign14870_e9528_d_n5;
        locals.var_tmf1_dn6 = assign14870_e9528_d_n6;
        locals.var_tmf1_dn7 = assign14870_e9528_d_n7;
        locals.var_tmf1_dn8 = assign14870_e9528_d_n8;
        locals.var_tmf1_dn9 = assign14870_e9528_d_n9;
        locals.var_tmf1_dn10 = assign14870_e9528_d_n10;
        locals.var_tmf1_dn11 = assign14870_e9528_d_n11;
        locals.var_tmf1_dn14 = assign14870_e9528_d_n14;

        let (assign14880_e9547, assign14880_e9547_d_n0, assign14880_e9547_d_n2, assign14880_e9547_d_n4, assign14880_e9547_d_n5, assign14880_e9547_d_n6, assign14880_e9547_d_n7, assign14880_e9547_d_n8, assign14880_e9547_d_n9, assign14880_e9547_d_n10, assign14880_e9547_d_n11, assign14880_e9547_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 == 0.0)) {
        let assign14880_e9540: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14880_e9541: f64 = (4.0 * assign14880_e9540);
        let assign14880_e9544: f64 = (0.01 * locals.var_uc_rdvd);
        let assign14880_e9545: f64 = (assign14880_e9541 * assign14880_e9544);
        (assign14880_e9545, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14880_e9547;
        locals.var_tmf2_dn0 = assign14880_e9547_d_n0;
        locals.var_tmf2_dn2 = assign14880_e9547_d_n2;
        locals.var_tmf2_dn4 = assign14880_e9547_d_n4;
        locals.var_tmf2_dn5 = assign14880_e9547_d_n5;
        locals.var_tmf2_dn6 = assign14880_e9547_d_n6;
        locals.var_tmf2_dn7 = assign14880_e9547_d_n7;
        locals.var_tmf2_dn8 = assign14880_e9547_d_n8;
        locals.var_tmf2_dn9 = assign14880_e9547_d_n9;
        locals.var_tmf2_dn10 = assign14880_e9547_d_n10;
        locals.var_tmf2_dn11 = assign14880_e9547_d_n11;
        locals.var_tmf2_dn14 = assign14880_e9547_d_n14;

    }

    pub(super) fn stamp_transient_block_29(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14890_e9564, assign14890_e9564_d_n0, assign14890_e9564_d_n2, assign14890_e9564_d_n4, assign14890_e9564_d_n5, assign14890_e9564_d_n6, assign14890_e9564_d_n7, assign14890_e9564_d_n8, assign14890_e9564_d_n9, assign14890_e9564_d_n10, assign14890_e9564_d_n11, assign14890_e9564_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 == 0.0)) {
        let (assign14890_e9562, assign14890_e9562_d_n0, assign14890_e9562_d_n2, assign14890_e9562_d_n4, assign14890_e9562_d_n5, assign14890_e9562_d_n6, assign14890_e9562_d_n7, assign14890_e9562_d_n8, assign14890_e9562_d_n9, assign14890_e9562_d_n10, assign14890_e9562_d_n11, assign14890_e9562_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign14890_e9561: f64 = (-locals.var_tmf2);
                (assign14890_e9561, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign14890_e9562, assign14890_e9562_d_n0, assign14890_e9562_d_n2, assign14890_e9562_d_n4, assign14890_e9562_d_n5, assign14890_e9562_d_n6, assign14890_e9562_d_n7, assign14890_e9562_d_n8, assign14890_e9562_d_n9, assign14890_e9562_d_n10, assign14890_e9562_d_n11, assign14890_e9562_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14890_e9564;
        locals.var_tmf2_dn0 = assign14890_e9564_d_n0;
        locals.var_tmf2_dn2 = assign14890_e9564_d_n2;
        locals.var_tmf2_dn4 = assign14890_e9564_d_n4;
        locals.var_tmf2_dn5 = assign14890_e9564_d_n5;
        locals.var_tmf2_dn6 = assign14890_e9564_d_n6;
        locals.var_tmf2_dn7 = assign14890_e9564_d_n7;
        locals.var_tmf2_dn8 = assign14890_e9564_d_n8;
        locals.var_tmf2_dn9 = assign14890_e9564_d_n9;
        locals.var_tmf2_dn10 = assign14890_e9564_d_n10;
        locals.var_tmf2_dn11 = assign14890_e9564_d_n11;
        locals.var_tmf2_dn14 = assign14890_e9564_d_n14;

        let (assign14900_e9580, assign14900_e9580_d_n0, assign14900_e9580_d_n2, assign14900_e9580_d_n4, assign14900_e9580_d_n5, assign14900_e9580_d_n6, assign14900_e9580_d_n7, assign14900_e9580_d_n8, assign14900_e9580_d_n9, assign14900_e9580_d_n10, assign14900_e9580_d_n11, assign14900_e9580_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 == 0.0)) {
        let assign14900_e9575: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14900_e9577: f64 = (assign14900_e9575 + locals.var_tmf2);
        let assign14900_e9578: f64 = (assign14900_e9577).sqrt();
        (assign14900_e9578, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14900_e9578)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14900_e9578)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14900_e9578)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14900_e9578)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14900_e9578)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14900_e9578)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14900_e9578)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14900_e9578)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14900_e9578)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14900_e9578)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign14900_e9578)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14900_e9580;
        locals.var_tmf2_dn0 = assign14900_e9580_d_n0;
        locals.var_tmf2_dn2 = assign14900_e9580_d_n2;
        locals.var_tmf2_dn4 = assign14900_e9580_d_n4;
        locals.var_tmf2_dn5 = assign14900_e9580_d_n5;
        locals.var_tmf2_dn6 = assign14900_e9580_d_n6;
        locals.var_tmf2_dn7 = assign14900_e9580_d_n7;
        locals.var_tmf2_dn8 = assign14900_e9580_d_n8;
        locals.var_tmf2_dn9 = assign14900_e9580_d_n9;
        locals.var_tmf2_dn10 = assign14900_e9580_d_n10;
        locals.var_tmf2_dn11 = assign14900_e9580_d_n11;
        locals.var_tmf2_dn14 = assign14900_e9580_d_n14;

        let (assign14910_e9597, assign14910_e9597_d_n0, assign14910_e9597_d_n2, assign14910_e9597_d_n4, assign14910_e9597_d_n5, assign14910_e9597_d_n6, assign14910_e9597_d_n7, assign14910_e9597_d_n8, assign14910_e9597_d_n9, assign14910_e9597_d_n10, assign14910_e9597_d_n11, assign14910_e9597_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 == 0.0)) {
        let assign14910_e9593: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign14910_e9594: f64 = (1.0 + assign14910_e9593);
        let assign14910_e9595: f64 = (0.5 * assign14910_e9594);
        (assign14910_e9595, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign14910_e9597;
        locals.var_t0_dn0 = assign14910_e9597_d_n0;
        locals.var_t0_dn2 = assign14910_e9597_d_n2;
        locals.var_t0_dn4 = assign14910_e9597_d_n4;
        locals.var_t0_dn5 = assign14910_e9597_d_n5;
        locals.var_t0_dn6 = assign14910_e9597_d_n6;
        locals.var_t0_dn7 = assign14910_e9597_d_n7;
        locals.var_t0_dn8 = assign14910_e9597_d_n8;
        locals.var_t0_dn9 = assign14910_e9597_d_n9;
        locals.var_t0_dn10 = assign14910_e9597_d_n10;
        locals.var_t0_dn11 = assign14910_e9597_d_n11;
        locals.var_t0_dn14 = assign14910_e9597_d_n14;

        let (assign14920_e9616, assign14920_e9616_d_n0, assign14920_e9616_d_n2, assign14920_e9616_d_n4, assign14920_e9616_d_n5, assign14920_e9616_d_n6, assign14920_e9616_d_n7, assign14920_e9616_d_n8, assign14920_e9616_d_n9, assign14920_e9616_d_n10, assign14920_e9616_d_n11, assign14920_e9616_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard322 == 0.0)) {
        let assign14920_e9608: f64 = (0.005 * locals.var_uc_rdvd);
        let assign14920_e9612: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign14920_e9613: f64 = (0.5 * assign14920_e9612);
        let assign14920_e9614: f64 = (assign14920_e9608 + assign14920_e9613);
        (assign14920_e9614, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign14920_e9616;
        locals.var_rdvde_dn0 = assign14920_e9616_d_n0;
        locals.var_rdvde_dn2 = assign14920_e9616_d_n2;
        locals.var_rdvde_dn4 = assign14920_e9616_d_n4;
        locals.var_rdvde_dn5 = assign14920_e9616_d_n5;
        locals.var_rdvde_dn6 = assign14920_e9616_d_n6;
        locals.var_rdvde_dn7 = assign14920_e9616_d_n7;
        locals.var_rdvde_dn8 = assign14920_e9616_d_n8;
        locals.var_rdvde_dn9 = assign14920_e9616_d_n9;
        locals.var_rdvde_dn10 = assign14920_e9616_d_n10;
        locals.var_rdvde_dn11 = assign14920_e9616_d_n11;
        locals.var_rdvde_dn14 = assign14920_e9616_d_n14;

        let (assign14930_e9640, assign14930_e9640_d_n0, assign14930_e9640_d_n2, assign14930_e9640_d_n4, assign14930_e9640_d_n5, assign14930_e9640_d_n6, assign14930_e9640_d_n7, assign14930_e9640_d_n8, assign14930_e9640_d_n9, assign14930_e9640_d_n10, assign14930_e9640_d_n11, assign14930_e9640_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14930_e9625: f64 = (p.p69 * locals.var_uc_rdslp1);
        let assign14930_e9627: f64 = (assign14930_e9625 * 1000000.0);
        let assign14930_e9629: f64 = (assign14930_e9627 + locals.var_uc_rdict1);
        let assign14930_e9630: f64 = (locals.var_rdvdtemp0 * assign14930_e9629);
        let assign14930_e9633: f64 = (p.p70 * p.p100);
        let assign14930_e9635: f64 = (assign14930_e9633 * 1000000.0);
        let assign14930_e9637: f64 = (assign14930_e9635 + p.p101);
        let assign14930_e9638: f64 = (assign14930_e9630 * assign14930_e9637);
        (assign14930_e9638, ((locals.var_rdvdtemp0_dn0 * assign14930_e9629) * assign14930_e9637), ((locals.var_rdvdtemp0_dn2 * assign14930_e9629) * assign14930_e9637), ((locals.var_rdvdtemp0_dn4 * assign14930_e9629) * assign14930_e9637), ((locals.var_rdvdtemp0_dn5 * assign14930_e9629) * assign14930_e9637), ((locals.var_rdvdtemp0_dn6 * assign14930_e9629) * assign14930_e9637), ((locals.var_rdvdtemp0_dn7 * assign14930_e9629) * assign14930_e9637), ((locals.var_rdvdtemp0_dn8 * assign14930_e9629) * assign14930_e9637), ((locals.var_rdvdtemp0_dn9 * assign14930_e9629) * assign14930_e9637), ((locals.var_rdvdtemp0_dn10 * assign14930_e9629) * assign14930_e9637), ((locals.var_rdvdtemp0_dn11 * assign14930_e9629) * assign14930_e9637), ((locals.var_rdvdtemp0_dn14 * assign14930_e9629) * assign14930_e9637),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign14930_e9640;
        locals.var_t4_dn0 = assign14930_e9640_d_n0;
        locals.var_t4_dn2 = assign14930_e9640_d_n2;
        locals.var_t4_dn4 = assign14930_e9640_d_n4;
        locals.var_t4_dn5 = assign14930_e9640_d_n5;
        locals.var_t4_dn6 = assign14930_e9640_d_n6;
        locals.var_t4_dn7 = assign14930_e9640_d_n7;
        locals.var_t4_dn8 = assign14930_e9640_d_n8;
        locals.var_t4_dn9 = assign14930_e9640_d_n9;
        locals.var_t4_dn10 = assign14930_e9640_d_n10;
        locals.var_t4_dn11 = assign14930_e9640_d_n11;
        locals.var_t4_dn14 = assign14930_e9640_d_n14;

        let (assign14940_e9654, assign14940_e9654_d_n0, assign14940_e9654_d_n2, assign14940_e9654_d_n4, assign14940_e9654_d_n5, assign14940_e9654_d_n6, assign14940_e9654_d_n7, assign14940_e9654_d_n8, assign14940_e9654_d_n9, assign14940_e9654_d_n10, assign14940_e9654_d_n11, assign14940_e9654_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14940_e9648: f64 = (1.0 - locals.var_uc_rdov13);
        let assign14940_e9650: f64 = (assign14940_e9648 * p.p66);
        let assign14940_e9652: f64 = (assign14940_e9650 * 1000000.0);
        (assign14940_e9652, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign14940_e9654;
        locals.var_t1_dn0 = assign14940_e9654_d_n0;
        locals.var_t1_dn2 = assign14940_e9654_d_n2;
        locals.var_t1_dn4 = assign14940_e9654_d_n4;
        locals.var_t1_dn5 = assign14940_e9654_d_n5;
        locals.var_t1_dn6 = assign14940_e9654_d_n6;
        locals.var_t1_dn7 = assign14940_e9654_d_n7;
        locals.var_t1_dn8 = assign14940_e9654_d_n8;
        locals.var_t1_dn9 = assign14940_e9654_d_n9;
        locals.var_t1_dn10 = assign14940_e9654_d_n10;
        locals.var_t1_dn11 = assign14940_e9654_d_n11;
        locals.var_t1_dn14 = assign14940_e9654_d_n14;

        let (assign14950_e9670, assign14950_e9670_d_n0, assign14950_e9670_d_n2, assign14950_e9670_d_n4, assign14950_e9670_d_n5, assign14950_e9670_d_n6, assign14950_e9670_d_n7, assign14950_e9670_d_n8, assign14950_e9670_d_n9, assign14950_e9670_d_n10, assign14950_e9670_d_n11, assign14950_e9670_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14950_e9662: f64 = (locals.var_t8 * p.p66);
        let assign14950_e9664: f64 = (assign14950_e9662 * 1000000.0);
        let assign14950_e9666: f64 = (assign14950_e9664 + 1.0);
        let assign14950_e9668: f64 = (assign14950_e9666 + p.p98);
        (assign14950_e9668, ((locals.var_t8_dn0 * p.p66) * 1000000.0), ((locals.var_t8_dn2 * p.p66) * 1000000.0), ((locals.var_t8_dn4 * p.p66) * 1000000.0), ((locals.var_t8_dn5 * p.p66) * 1000000.0), ((locals.var_t8_dn6 * p.p66) * 1000000.0), ((locals.var_t8_dn7 * p.p66) * 1000000.0), ((locals.var_t8_dn8 * p.p66) * 1000000.0), ((locals.var_t8_dn9 * p.p66) * 1000000.0), ((locals.var_t8_dn10 * p.p66) * 1000000.0), ((locals.var_t8_dn11 * p.p66) * 1000000.0), ((locals.var_t8_dn14 * p.p66) * 1000000.0),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign14950_e9670;
        locals.var_t3_dn0 = assign14950_e9670_d_n0;
        locals.var_t3_dn2 = assign14950_e9670_d_n2;
        locals.var_t3_dn4 = assign14950_e9670_d_n4;
        locals.var_t3_dn5 = assign14950_e9670_d_n5;
        locals.var_t3_dn6 = assign14950_e9670_d_n6;
        locals.var_t3_dn7 = assign14950_e9670_d_n7;
        locals.var_t3_dn8 = assign14950_e9670_d_n8;
        locals.var_t3_dn9 = assign14950_e9670_d_n9;
        locals.var_t3_dn10 = assign14950_e9670_d_n10;
        locals.var_t3_dn11 = assign14950_e9670_d_n11;
        locals.var_t3_dn14 = assign14950_e9670_d_n14;

        let (assign14960_e9684, assign14960_e9684_d_n0, assign14960_e9684_d_n2, assign14960_e9684_d_n4, assign14960_e9684_d_n5, assign14960_e9684_d_n6, assign14960_e9684_d_n7, assign14960_e9684_d_n8, assign14960_e9684_d_n9, assign14960_e9684_d_n10, assign14960_e9684_d_n11, assign14960_e9684_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14960_e9678: f64 = (locals.var_t3 * locals.var_t4);
        let assign14960_e9680: f64 = (assign14960_e9678 - locals.var_t4);
        let assign14960_e9682: f64 = (assign14960_e9680 - 0.01);
        (assign14960_e9682, (((locals.var_t3_dn0 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn0)) - locals.var_t4_dn0), (((locals.var_t3_dn2 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn2)) - locals.var_t4_dn2), (((locals.var_t3_dn4 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn4)) - locals.var_t4_dn4), (((locals.var_t3_dn5 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn5)) - locals.var_t4_dn5), (((locals.var_t3_dn6 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn6)) - locals.var_t4_dn6), (((locals.var_t3_dn7 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn7)) - locals.var_t4_dn7), (((locals.var_t3_dn8 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn8)) - locals.var_t4_dn8), (((locals.var_t3_dn9 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn9)) - locals.var_t4_dn9), (((locals.var_t3_dn10 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn10)) - locals.var_t4_dn10), (((locals.var_t3_dn11 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn11)) - locals.var_t4_dn11), (((locals.var_t3_dn14 * locals.var_t4) + (locals.var_t3 * locals.var_t4_dn14)) - locals.var_t4_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign14960_e9684;
        locals.var_tmf1_dn0 = assign14960_e9684_d_n0;
        locals.var_tmf1_dn2 = assign14960_e9684_d_n2;
        locals.var_tmf1_dn4 = assign14960_e9684_d_n4;
        locals.var_tmf1_dn5 = assign14960_e9684_d_n5;
        locals.var_tmf1_dn6 = assign14960_e9684_d_n6;
        locals.var_tmf1_dn7 = assign14960_e9684_d_n7;
        locals.var_tmf1_dn8 = assign14960_e9684_d_n8;
        locals.var_tmf1_dn9 = assign14960_e9684_d_n9;
        locals.var_tmf1_dn10 = assign14960_e9684_d_n10;
        locals.var_tmf1_dn11 = assign14960_e9684_d_n11;
        locals.var_tmf1_dn14 = assign14960_e9684_d_n14;

        let (assign14970_e9696, assign14970_e9696_d_n0, assign14970_e9696_d_n2, assign14970_e9696_d_n4, assign14970_e9696_d_n5, assign14970_e9696_d_n6, assign14970_e9696_d_n7, assign14970_e9696_d_n8, assign14970_e9696_d_n9, assign14970_e9696_d_n10, assign14970_e9696_d_n11, assign14970_e9696_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14970_e9692: f64 = (4.0 * locals.var_t4);
        let assign14970_e9694: f64 = (assign14970_e9692 * 0.01);
        (assign14970_e9694, ((4.0 * locals.var_t4_dn0) * 0.01), ((4.0 * locals.var_t4_dn2) * 0.01), ((4.0 * locals.var_t4_dn4) * 0.01), ((4.0 * locals.var_t4_dn5) * 0.01), ((4.0 * locals.var_t4_dn6) * 0.01), ((4.0 * locals.var_t4_dn7) * 0.01), ((4.0 * locals.var_t4_dn8) * 0.01), ((4.0 * locals.var_t4_dn9) * 0.01), ((4.0 * locals.var_t4_dn10) * 0.01), ((4.0 * locals.var_t4_dn11) * 0.01), ((4.0 * locals.var_t4_dn14) * 0.01),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14970_e9696;
        locals.var_tmf2_dn0 = assign14970_e9696_d_n0;
        locals.var_tmf2_dn2 = assign14970_e9696_d_n2;
        locals.var_tmf2_dn4 = assign14970_e9696_d_n4;
        locals.var_tmf2_dn5 = assign14970_e9696_d_n5;
        locals.var_tmf2_dn6 = assign14970_e9696_d_n6;
        locals.var_tmf2_dn7 = assign14970_e9696_d_n7;
        locals.var_tmf2_dn8 = assign14970_e9696_d_n8;
        locals.var_tmf2_dn9 = assign14970_e9696_d_n9;
        locals.var_tmf2_dn10 = assign14970_e9696_d_n10;
        locals.var_tmf2_dn11 = assign14970_e9696_d_n11;
        locals.var_tmf2_dn14 = assign14970_e9696_d_n14;

        let (assign14980_e9710, assign14980_e9710_d_n0, assign14980_e9710_d_n2, assign14980_e9710_d_n4, assign14980_e9710_d_n5, assign14980_e9710_d_n6, assign14980_e9710_d_n7, assign14980_e9710_d_n8, assign14980_e9710_d_n9, assign14980_e9710_d_n10, assign14980_e9710_d_n11, assign14980_e9710_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let (assign14980_e9708, assign14980_e9708_d_n0, assign14980_e9708_d_n2, assign14980_e9708_d_n4, assign14980_e9708_d_n5, assign14980_e9708_d_n6, assign14980_e9708_d_n7, assign14980_e9708_d_n8, assign14980_e9708_d_n9, assign14980_e9708_d_n10, assign14980_e9708_d_n11, assign14980_e9708_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign14980_e9707: f64 = (-locals.var_tmf2);
                (assign14980_e9707, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign14980_e9708, assign14980_e9708_d_n0, assign14980_e9708_d_n2, assign14980_e9708_d_n4, assign14980_e9708_d_n5, assign14980_e9708_d_n6, assign14980_e9708_d_n7, assign14980_e9708_d_n8, assign14980_e9708_d_n9, assign14980_e9708_d_n10, assign14980_e9708_d_n11, assign14980_e9708_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14980_e9710;
        locals.var_tmf2_dn0 = assign14980_e9710_d_n0;
        locals.var_tmf2_dn2 = assign14980_e9710_d_n2;
        locals.var_tmf2_dn4 = assign14980_e9710_d_n4;
        locals.var_tmf2_dn5 = assign14980_e9710_d_n5;
        locals.var_tmf2_dn6 = assign14980_e9710_d_n6;
        locals.var_tmf2_dn7 = assign14980_e9710_d_n7;
        locals.var_tmf2_dn8 = assign14980_e9710_d_n8;
        locals.var_tmf2_dn9 = assign14980_e9710_d_n9;
        locals.var_tmf2_dn10 = assign14980_e9710_d_n10;
        locals.var_tmf2_dn11 = assign14980_e9710_d_n11;
        locals.var_tmf2_dn14 = assign14980_e9710_d_n14;

        let (assign14990_e9723, assign14990_e9723_d_n0, assign14990_e9723_d_n2, assign14990_e9723_d_n4, assign14990_e9723_d_n5, assign14990_e9723_d_n6, assign14990_e9723_d_n7, assign14990_e9723_d_n8, assign14990_e9723_d_n9, assign14990_e9723_d_n10, assign14990_e9723_d_n11, assign14990_e9723_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign14990_e9718: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign14990_e9720: f64 = (assign14990_e9718 + locals.var_tmf2);
        let assign14990_e9721: f64 = (assign14990_e9720).sqrt();
        (assign14990_e9721, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign14990_e9721)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign14990_e9721)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign14990_e9721)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign14990_e9721)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign14990_e9721)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign14990_e9721)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign14990_e9721)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign14990_e9721)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign14990_e9721)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign14990_e9721)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign14990_e9721)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign14990_e9723;
        locals.var_tmf2_dn0 = assign14990_e9723_d_n0;
        locals.var_tmf2_dn2 = assign14990_e9723_d_n2;
        locals.var_tmf2_dn4 = assign14990_e9723_d_n4;
        locals.var_tmf2_dn5 = assign14990_e9723_d_n5;
        locals.var_tmf2_dn6 = assign14990_e9723_d_n6;
        locals.var_tmf2_dn7 = assign14990_e9723_d_n7;
        locals.var_tmf2_dn8 = assign14990_e9723_d_n8;
        locals.var_tmf2_dn9 = assign14990_e9723_d_n9;
        locals.var_tmf2_dn10 = assign14990_e9723_d_n10;
        locals.var_tmf2_dn11 = assign14990_e9723_d_n11;
        locals.var_tmf2_dn14 = assign14990_e9723_d_n14;

        let (assign15000_e9737, assign15000_e9737_d_n0, assign15000_e9737_d_n2, assign15000_e9737_d_n4, assign15000_e9737_d_n5, assign15000_e9737_d_n6, assign15000_e9737_d_n7, assign15000_e9737_d_n8, assign15000_e9737_d_n9, assign15000_e9737_d_n10, assign15000_e9737_d_n11, assign15000_e9737_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15000_e9733: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15000_e9734: f64 = (1.0 + assign15000_e9733);
        let assign15000_e9735: f64 = (0.5 * assign15000_e9734);
        (assign15000_e9735, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign15000_e9737;
        locals.var_t6_dn0 = assign15000_e9737_d_n0;
        locals.var_t6_dn2 = assign15000_e9737_d_n2;
        locals.var_t6_dn4 = assign15000_e9737_d_n4;
        locals.var_t6_dn5 = assign15000_e9737_d_n5;
        locals.var_t6_dn6 = assign15000_e9737_d_n6;
        locals.var_t6_dn7 = assign15000_e9737_d_n7;
        locals.var_t6_dn8 = assign15000_e9737_d_n8;
        locals.var_t6_dn9 = assign15000_e9737_d_n9;
        locals.var_t6_dn10 = assign15000_e9737_d_n10;
        locals.var_t6_dn11 = assign15000_e9737_d_n11;
        locals.var_t6_dn14 = assign15000_e9737_d_n14;

        let (assign15010_e9751, assign15010_e9751_d_n0, assign15010_e9751_d_n2, assign15010_e9751_d_n4, assign15010_e9751_d_n5, assign15010_e9751_d_n6, assign15010_e9751_d_n7, assign15010_e9751_d_n8, assign15010_e9751_d_n9, assign15010_e9751_d_n10, assign15010_e9751_d_n11, assign15010_e9751_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15010_e9747: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15010_e9748: f64 = (0.5 * assign15010_e9747);
        let assign15010_e9749: f64 = (locals.var_t4 + assign15010_e9748);
        (assign15010_e9749, (locals.var_t4_dn0 + (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), (locals.var_t4_dn2 + (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), (locals.var_t4_dn4 + (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), (locals.var_t4_dn5 + (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), (locals.var_t4_dn6 + (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), (locals.var_t4_dn7 + (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), (locals.var_t4_dn8 + (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), (locals.var_t4_dn9 + (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), (locals.var_t4_dn10 + (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), (locals.var_t4_dn11 + (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), (locals.var_t4_dn14 + (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t5, locals.var_t5_dn0, locals.var_t5_dn2, locals.var_t5_dn4, locals.var_t5_dn5, locals.var_t5_dn6, locals.var_t5_dn7, locals.var_t5_dn8, locals.var_t5_dn9, locals.var_t5_dn10, locals.var_t5_dn11, locals.var_t5_dn14,)
    }
};
        locals.var_t5 = assign15010_e9751;
        locals.var_t5_dn0 = assign15010_e9751_d_n0;
        locals.var_t5_dn2 = assign15010_e9751_d_n2;
        locals.var_t5_dn4 = assign15010_e9751_d_n4;
        locals.var_t5_dn5 = assign15010_e9751_d_n5;
        locals.var_t5_dn6 = assign15010_e9751_d_n6;
        locals.var_t5_dn7 = assign15010_e9751_d_n7;
        locals.var_t5_dn8 = assign15010_e9751_d_n8;
        locals.var_t5_dn9 = assign15010_e9751_d_n9;
        locals.var_t5_dn10 = assign15010_e9751_d_n10;
        locals.var_t5_dn11 = assign15010_e9751_d_n11;
        locals.var_t5_dn14 = assign15010_e9751_d_n14;

        let (assign15020_e9767, assign15020_e9767_d_n0, assign15020_e9767_d_n2, assign15020_e9767_d_n4, assign15020_e9767_d_n5, assign15020_e9767_d_n6, assign15020_e9767_d_n7, assign15020_e9767_d_n8, assign15020_e9767_d_n9, assign15020_e9767_d_n10, assign15020_e9767_d_n11, assign15020_e9767_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15020_e9760: f64 = (p.p98 + 1.0);
        let assign15020_e9761: f64 = (locals.var_t4 * assign15020_e9760);
        let assign15020_e9763: f64 = (assign15020_e9761 - locals.var_t5);
        let assign15020_e9765: f64 = (assign15020_e9763 - 5e-5);
        (assign15020_e9765, ((locals.var_t4_dn0 * assign15020_e9760) - locals.var_t5_dn0), ((locals.var_t4_dn2 * assign15020_e9760) - locals.var_t5_dn2), ((locals.var_t4_dn4 * assign15020_e9760) - locals.var_t5_dn4), ((locals.var_t4_dn5 * assign15020_e9760) - locals.var_t5_dn5), ((locals.var_t4_dn6 * assign15020_e9760) - locals.var_t5_dn6), ((locals.var_t4_dn7 * assign15020_e9760) - locals.var_t5_dn7), ((locals.var_t4_dn8 * assign15020_e9760) - locals.var_t5_dn8), ((locals.var_t4_dn9 * assign15020_e9760) - locals.var_t5_dn9), ((locals.var_t4_dn10 * assign15020_e9760) - locals.var_t5_dn10), ((locals.var_t4_dn11 * assign15020_e9760) - locals.var_t5_dn11), ((locals.var_t4_dn14 * assign15020_e9760) - locals.var_t5_dn14),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign15020_e9767;
        locals.var_tmf1_dn0 = assign15020_e9767_d_n0;
        locals.var_tmf1_dn2 = assign15020_e9767_d_n2;
        locals.var_tmf1_dn4 = assign15020_e9767_d_n4;
        locals.var_tmf1_dn5 = assign15020_e9767_d_n5;
        locals.var_tmf1_dn6 = assign15020_e9767_d_n6;
        locals.var_tmf1_dn7 = assign15020_e9767_d_n7;
        locals.var_tmf1_dn8 = assign15020_e9767_d_n8;
        locals.var_tmf1_dn9 = assign15020_e9767_d_n9;
        locals.var_tmf1_dn10 = assign15020_e9767_d_n10;
        locals.var_tmf1_dn11 = assign15020_e9767_d_n11;
        locals.var_tmf1_dn14 = assign15020_e9767_d_n14;

        let (assign15030_e9783, assign15030_e9783_d_n0, assign15030_e9783_d_n2, assign15030_e9783_d_n4, assign15030_e9783_d_n5, assign15030_e9783_d_n6, assign15030_e9783_d_n7, assign15030_e9783_d_n8, assign15030_e9783_d_n9, assign15030_e9783_d_n10, assign15030_e9783_d_n11, assign15030_e9783_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15030_e9777: f64 = (p.p98 + 1.0);
        let assign15030_e9778: f64 = (locals.var_t4 * assign15030_e9777);
        let assign15030_e9779: f64 = (4.0 * assign15030_e9778);
        let assign15030_e9781: f64 = (assign15030_e9779 * 5e-5);
        (assign15030_e9781, ((4.0 * (locals.var_t4_dn0 * assign15030_e9777)) * 5e-5), ((4.0 * (locals.var_t4_dn2 * assign15030_e9777)) * 5e-5), ((4.0 * (locals.var_t4_dn4 * assign15030_e9777)) * 5e-5), ((4.0 * (locals.var_t4_dn5 * assign15030_e9777)) * 5e-5), ((4.0 * (locals.var_t4_dn6 * assign15030_e9777)) * 5e-5), ((4.0 * (locals.var_t4_dn7 * assign15030_e9777)) * 5e-5), ((4.0 * (locals.var_t4_dn8 * assign15030_e9777)) * 5e-5), ((4.0 * (locals.var_t4_dn9 * assign15030_e9777)) * 5e-5), ((4.0 * (locals.var_t4_dn10 * assign15030_e9777)) * 5e-5), ((4.0 * (locals.var_t4_dn11 * assign15030_e9777)) * 5e-5), ((4.0 * (locals.var_t4_dn14 * assign15030_e9777)) * 5e-5),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15030_e9783;
        locals.var_tmf2_dn0 = assign15030_e9783_d_n0;
        locals.var_tmf2_dn2 = assign15030_e9783_d_n2;
        locals.var_tmf2_dn4 = assign15030_e9783_d_n4;
        locals.var_tmf2_dn5 = assign15030_e9783_d_n5;
        locals.var_tmf2_dn6 = assign15030_e9783_d_n6;
        locals.var_tmf2_dn7 = assign15030_e9783_d_n7;
        locals.var_tmf2_dn8 = assign15030_e9783_d_n8;
        locals.var_tmf2_dn9 = assign15030_e9783_d_n9;
        locals.var_tmf2_dn10 = assign15030_e9783_d_n10;
        locals.var_tmf2_dn11 = assign15030_e9783_d_n11;
        locals.var_tmf2_dn14 = assign15030_e9783_d_n14;

        let (assign15040_e9797, assign15040_e9797_d_n0, assign15040_e9797_d_n2, assign15040_e9797_d_n4, assign15040_e9797_d_n5, assign15040_e9797_d_n6, assign15040_e9797_d_n7, assign15040_e9797_d_n8, assign15040_e9797_d_n9, assign15040_e9797_d_n10, assign15040_e9797_d_n11, assign15040_e9797_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let (assign15040_e9795, assign15040_e9795_d_n0, assign15040_e9795_d_n2, assign15040_e9795_d_n4, assign15040_e9795_d_n5, assign15040_e9795_d_n6, assign15040_e9795_d_n7, assign15040_e9795_d_n8, assign15040_e9795_d_n9, assign15040_e9795_d_n10, assign15040_e9795_d_n11, assign15040_e9795_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign15040_e9794: f64 = (-locals.var_tmf2);
                (assign15040_e9794, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign15040_e9795, assign15040_e9795_d_n0, assign15040_e9795_d_n2, assign15040_e9795_d_n4, assign15040_e9795_d_n5, assign15040_e9795_d_n6, assign15040_e9795_d_n7, assign15040_e9795_d_n8, assign15040_e9795_d_n9, assign15040_e9795_d_n10, assign15040_e9795_d_n11, assign15040_e9795_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15040_e9797;
        locals.var_tmf2_dn0 = assign15040_e9797_d_n0;
        locals.var_tmf2_dn2 = assign15040_e9797_d_n2;
        locals.var_tmf2_dn4 = assign15040_e9797_d_n4;
        locals.var_tmf2_dn5 = assign15040_e9797_d_n5;
        locals.var_tmf2_dn6 = assign15040_e9797_d_n6;
        locals.var_tmf2_dn7 = assign15040_e9797_d_n7;
        locals.var_tmf2_dn8 = assign15040_e9797_d_n8;
        locals.var_tmf2_dn9 = assign15040_e9797_d_n9;
        locals.var_tmf2_dn10 = assign15040_e9797_d_n10;
        locals.var_tmf2_dn11 = assign15040_e9797_d_n11;
        locals.var_tmf2_dn14 = assign15040_e9797_d_n14;

        let (assign15050_e9810, assign15050_e9810_d_n0, assign15050_e9810_d_n2, assign15050_e9810_d_n4, assign15050_e9810_d_n5, assign15050_e9810_d_n6, assign15050_e9810_d_n7, assign15050_e9810_d_n8, assign15050_e9810_d_n9, assign15050_e9810_d_n10, assign15050_e9810_d_n11, assign15050_e9810_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15050_e9805: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15050_e9807: f64 = (assign15050_e9805 + locals.var_tmf2);
        let assign15050_e9808: f64 = (assign15050_e9807).sqrt();
        (assign15050_e9808, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15050_e9808)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15050_e9808)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign15050_e9808)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign15050_e9808)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign15050_e9808)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign15050_e9808)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign15050_e9808)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign15050_e9808)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign15050_e9808)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign15050_e9808)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign15050_e9808)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15050_e9810;
        locals.var_tmf2_dn0 = assign15050_e9810_d_n0;
        locals.var_tmf2_dn2 = assign15050_e9810_d_n2;
        locals.var_tmf2_dn4 = assign15050_e9810_d_n4;
        locals.var_tmf2_dn5 = assign15050_e9810_d_n5;
        locals.var_tmf2_dn6 = assign15050_e9810_d_n6;
        locals.var_tmf2_dn7 = assign15050_e9810_d_n7;
        locals.var_tmf2_dn8 = assign15050_e9810_d_n8;
        locals.var_tmf2_dn9 = assign15050_e9810_d_n9;
        locals.var_tmf2_dn10 = assign15050_e9810_d_n10;
        locals.var_tmf2_dn11 = assign15050_e9810_d_n11;
        locals.var_tmf2_dn14 = assign15050_e9810_d_n14;

        let (assign15060_e9824, assign15060_e9824_d_n0, assign15060_e9824_d_n2, assign15060_e9824_d_n4, assign15060_e9824_d_n5, assign15060_e9824_d_n6, assign15060_e9824_d_n7, assign15060_e9824_d_n8, assign15060_e9824_d_n9, assign15060_e9824_d_n10, assign15060_e9824_d_n11, assign15060_e9824_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15060_e9820: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15060_e9821: f64 = (1.0 + assign15060_e9820);
        let assign15060_e9822: f64 = (0.5 * assign15060_e9821);
        (assign15060_e9822, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign15060_e9824;
        locals.var_t6_dn0 = assign15060_e9824_d_n0;
        locals.var_t6_dn2 = assign15060_e9824_d_n2;
        locals.var_t6_dn4 = assign15060_e9824_d_n4;
        locals.var_t6_dn5 = assign15060_e9824_d_n5;
        locals.var_t6_dn6 = assign15060_e9824_d_n6;
        locals.var_t6_dn7 = assign15060_e9824_d_n7;
        locals.var_t6_dn8 = assign15060_e9824_d_n8;
        locals.var_t6_dn9 = assign15060_e9824_d_n9;
        locals.var_t6_dn10 = assign15060_e9824_d_n10;
        locals.var_t6_dn11 = assign15060_e9824_d_n11;
        locals.var_t6_dn14 = assign15060_e9824_d_n14;

        let (assign15070_e9842, assign15070_e9842_d_n0, assign15070_e9842_d_n2, assign15070_e9842_d_n4, assign15070_e9842_d_n5, assign15070_e9842_d_n6, assign15070_e9842_d_n7, assign15070_e9842_d_n8, assign15070_e9842_d_n9, assign15070_e9842_d_n10, assign15070_e9842_d_n11, assign15070_e9842_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15070_e9833: f64 = (p.p98 + 1.0);
        let assign15070_e9834: f64 = (locals.var_t4 * assign15070_e9833);
        let assign15070_e9838: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15070_e9839: f64 = (0.5 * assign15070_e9838);
        let assign15070_e9840: f64 = (assign15070_e9834 - assign15070_e9839);
        (assign15070_e9840, ((locals.var_t4_dn0 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0))), ((locals.var_t4_dn2 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2))), ((locals.var_t4_dn4 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4))), ((locals.var_t4_dn5 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5))), ((locals.var_t4_dn6 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6))), ((locals.var_t4_dn7 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7))), ((locals.var_t4_dn8 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8))), ((locals.var_t4_dn9 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9))), ((locals.var_t4_dn10 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10))), ((locals.var_t4_dn11 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11))), ((locals.var_t4_dn14 * assign15070_e9833) - (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14))),)
    } else {
        (locals.var_t7, locals.var_t7_dn0, locals.var_t7_dn2, locals.var_t7_dn4, locals.var_t7_dn5, locals.var_t7_dn6, locals.var_t7_dn7, locals.var_t7_dn8, locals.var_t7_dn9, locals.var_t7_dn10, locals.var_t7_dn11, locals.var_t7_dn14,)
    }
};
        locals.var_t7 = assign15070_e9842;
        locals.var_t7_dn0 = assign15070_e9842_d_n0;
        locals.var_t7_dn2 = assign15070_e9842_d_n2;
        locals.var_t7_dn4 = assign15070_e9842_d_n4;
        locals.var_t7_dn5 = assign15070_e9842_d_n5;
        locals.var_t7_dn6 = assign15070_e9842_d_n6;
        locals.var_t7_dn7 = assign15070_e9842_d_n7;
        locals.var_t7_dn8 = assign15070_e9842_d_n8;
        locals.var_t7_dn9 = assign15070_e9842_d_n9;
        locals.var_t7_dn10 = assign15070_e9842_d_n10;
        locals.var_t7_dn11 = assign15070_e9842_d_n11;
        locals.var_t7_dn14 = assign15070_e9842_d_n14;

        let (assign15080_e9858, assign15080_e9858_d_n0, assign15080_e9858_d_n2, assign15080_e9858_d_n4, assign15080_e9858_d_n5, assign15080_e9858_d_n6, assign15080_e9858_d_n7, assign15080_e9858_d_n8, assign15080_e9858_d_n9, assign15080_e9858_d_n10, assign15080_e9858_d_n11, assign15080_e9858_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15080_e9851: f64 = (locals.var_t1 * locals.var_t4);
        let assign15080_e9852: f64 = (locals.var_t7 + assign15080_e9851);
        let assign15080_e9854: f64 = assign15080_e9852;
        let assign15080_e9856: f64 = (assign15080_e9854 - 5e-5);
        (assign15080_e9856, (locals.var_t7_dn0 + ((locals.var_t1_dn0 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn0))), (locals.var_t7_dn2 + ((locals.var_t1_dn2 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn2))), (locals.var_t7_dn4 + ((locals.var_t1_dn4 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn4))), (locals.var_t7_dn5 + ((locals.var_t1_dn5 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn5))), (locals.var_t7_dn6 + ((locals.var_t1_dn6 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn6))), (locals.var_t7_dn7 + ((locals.var_t1_dn7 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn7))), (locals.var_t7_dn8 + ((locals.var_t1_dn8 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn8))), (locals.var_t7_dn9 + ((locals.var_t1_dn9 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn9))), (locals.var_t7_dn10 + ((locals.var_t1_dn10 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn10))), (locals.var_t7_dn11 + ((locals.var_t1_dn11 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn11))), (locals.var_t7_dn14 + ((locals.var_t1_dn14 * locals.var_t4) + (locals.var_t1 * locals.var_t4_dn14))),)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign15080_e9858;
        locals.var_tmf1_dn0 = assign15080_e9858_d_n0;
        locals.var_tmf1_dn2 = assign15080_e9858_d_n2;
        locals.var_tmf1_dn4 = assign15080_e9858_d_n4;
        locals.var_tmf1_dn5 = assign15080_e9858_d_n5;
        locals.var_tmf1_dn6 = assign15080_e9858_d_n6;
        locals.var_tmf1_dn7 = assign15080_e9858_d_n7;
        locals.var_tmf1_dn8 = assign15080_e9858_d_n8;
        locals.var_tmf1_dn9 = assign15080_e9858_d_n9;
        locals.var_tmf1_dn10 = assign15080_e9858_d_n10;
        locals.var_tmf1_dn11 = assign15080_e9858_d_n11;
        locals.var_tmf1_dn14 = assign15080_e9858_d_n14;

        let (assign15090_e9870, assign15090_e9870_d_n0, assign15090_e9870_d_n2, assign15090_e9870_d_n4, assign15090_e9870_d_n5, assign15090_e9870_d_n6, assign15090_e9870_d_n7, assign15090_e9870_d_n8, assign15090_e9870_d_n9, assign15090_e9870_d_n10, assign15090_e9870_d_n11, assign15090_e9870_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15090_e9870;
        locals.var_tmf2_dn0 = assign15090_e9870_d_n0;
        locals.var_tmf2_dn2 = assign15090_e9870_d_n2;
        locals.var_tmf2_dn4 = assign15090_e9870_d_n4;
        locals.var_tmf2_dn5 = assign15090_e9870_d_n5;
        locals.var_tmf2_dn6 = assign15090_e9870_d_n6;
        locals.var_tmf2_dn7 = assign15090_e9870_d_n7;
        locals.var_tmf2_dn8 = assign15090_e9870_d_n8;
        locals.var_tmf2_dn9 = assign15090_e9870_d_n9;
        locals.var_tmf2_dn10 = assign15090_e9870_d_n10;
        locals.var_tmf2_dn11 = assign15090_e9870_d_n11;
        locals.var_tmf2_dn14 = assign15090_e9870_d_n14;

    }

    pub(super) fn stamp_transient_block_30(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15100_e9884, assign15100_e9884_d_n0, assign15100_e9884_d_n2, assign15100_e9884_d_n4, assign15100_e9884_d_n5, assign15100_e9884_d_n6, assign15100_e9884_d_n7, assign15100_e9884_d_n8, assign15100_e9884_d_n9, assign15100_e9884_d_n10, assign15100_e9884_d_n11, assign15100_e9884_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let (assign15100_e9882, assign15100_e9882_d_n0, assign15100_e9882_d_n2, assign15100_e9882_d_n4, assign15100_e9882_d_n5, assign15100_e9882_d_n6, assign15100_e9882_d_n7, assign15100_e9882_d_n8, assign15100_e9882_d_n9, assign15100_e9882_d_n10, assign15100_e9882_d_n11, assign15100_e9882_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign15100_e9881: f64 = (-locals.var_tmf2);
                (assign15100_e9881, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign15100_e9882, assign15100_e9882_d_n0, assign15100_e9882_d_n2, assign15100_e9882_d_n4, assign15100_e9882_d_n5, assign15100_e9882_d_n6, assign15100_e9882_d_n7, assign15100_e9882_d_n8, assign15100_e9882_d_n9, assign15100_e9882_d_n10, assign15100_e9882_d_n11, assign15100_e9882_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15100_e9884;
        locals.var_tmf2_dn0 = assign15100_e9884_d_n0;
        locals.var_tmf2_dn2 = assign15100_e9884_d_n2;
        locals.var_tmf2_dn4 = assign15100_e9884_d_n4;
        locals.var_tmf2_dn5 = assign15100_e9884_d_n5;
        locals.var_tmf2_dn6 = assign15100_e9884_d_n6;
        locals.var_tmf2_dn7 = assign15100_e9884_d_n7;
        locals.var_tmf2_dn8 = assign15100_e9884_d_n8;
        locals.var_tmf2_dn9 = assign15100_e9884_d_n9;
        locals.var_tmf2_dn10 = assign15100_e9884_d_n10;
        locals.var_tmf2_dn11 = assign15100_e9884_d_n11;
        locals.var_tmf2_dn14 = assign15100_e9884_d_n14;

        let (assign15110_e9897, assign15110_e9897_d_n0, assign15110_e9897_d_n2, assign15110_e9897_d_n4, assign15110_e9897_d_n5, assign15110_e9897_d_n6, assign15110_e9897_d_n7, assign15110_e9897_d_n8, assign15110_e9897_d_n9, assign15110_e9897_d_n10, assign15110_e9897_d_n11, assign15110_e9897_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15110_e9892: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15110_e9894: f64 = (assign15110_e9892 + locals.var_tmf2);
        let assign15110_e9895: f64 = (assign15110_e9894).sqrt();
        (assign15110_e9895, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15110_e9895)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15110_e9895)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign15110_e9895)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign15110_e9895)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign15110_e9895)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign15110_e9895)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign15110_e9895)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign15110_e9895)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign15110_e9895)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign15110_e9895)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign15110_e9895)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15110_e9897;
        locals.var_tmf2_dn0 = assign15110_e9897_d_n0;
        locals.var_tmf2_dn2 = assign15110_e9897_d_n2;
        locals.var_tmf2_dn4 = assign15110_e9897_d_n4;
        locals.var_tmf2_dn5 = assign15110_e9897_d_n5;
        locals.var_tmf2_dn6 = assign15110_e9897_d_n6;
        locals.var_tmf2_dn7 = assign15110_e9897_d_n7;
        locals.var_tmf2_dn8 = assign15110_e9897_d_n8;
        locals.var_tmf2_dn9 = assign15110_e9897_d_n9;
        locals.var_tmf2_dn10 = assign15110_e9897_d_n10;
        locals.var_tmf2_dn11 = assign15110_e9897_d_n11;
        locals.var_tmf2_dn14 = assign15110_e9897_d_n14;

        let (assign15120_e9911, assign15120_e9911_d_n0, assign15120_e9911_d_n2, assign15120_e9911_d_n4, assign15120_e9911_d_n5, assign15120_e9911_d_n6, assign15120_e9911_d_n7, assign15120_e9911_d_n8, assign15120_e9911_d_n9, assign15120_e9911_d_n10, assign15120_e9911_d_n11, assign15120_e9911_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15120_e9907: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15120_e9908: f64 = (1.0 + assign15120_e9907);
        let assign15120_e9909: f64 = (0.5 * assign15120_e9908);
        (assign15120_e9909, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t6, locals.var_t6_dn0, locals.var_t6_dn2, locals.var_t6_dn4, locals.var_t6_dn5, locals.var_t6_dn6, locals.var_t6_dn7, locals.var_t6_dn8, locals.var_t6_dn9, locals.var_t6_dn10, locals.var_t6_dn11, locals.var_t6_dn14,)
    }
};
        locals.var_t6 = assign15120_e9911;
        locals.var_t6_dn0 = assign15120_e9911_d_n0;
        locals.var_t6_dn2 = assign15120_e9911_d_n2;
        locals.var_t6_dn4 = assign15120_e9911_d_n4;
        locals.var_t6_dn5 = assign15120_e9911_d_n5;
        locals.var_t6_dn6 = assign15120_e9911_d_n6;
        locals.var_t6_dn7 = assign15120_e9911_d_n7;
        locals.var_t6_dn8 = assign15120_e9911_d_n8;
        locals.var_t6_dn9 = assign15120_e9911_d_n9;
        locals.var_t6_dn10 = assign15120_e9911_d_n10;
        locals.var_t6_dn11 = assign15120_e9911_d_n11;
        locals.var_t6_dn14 = assign15120_e9911_d_n14;

        let (assign15130_e9925, assign15130_e9925_d_n0, assign15130_e9925_d_n2, assign15130_e9925_d_n4, assign15130_e9925_d_n5, assign15130_e9925_d_n6, assign15130_e9925_d_n7, assign15130_e9925_d_n8, assign15130_e9925_d_n9, assign15130_e9925_d_n10, assign15130_e9925_d_n11, assign15130_e9925_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) {
        let assign15130_e9921: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15130_e9922: f64 = (0.5 * assign15130_e9921);
        let assign15130_e9923: f64 = assign15130_e9922;
        (assign15130_e9923, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15130_e9925;
        locals.var_t2_dn0 = assign15130_e9925_d_n0;
        locals.var_t2_dn2 = assign15130_e9925_d_n2;
        locals.var_t2_dn4 = assign15130_e9925_d_n4;
        locals.var_t2_dn5 = assign15130_e9925_d_n5;
        locals.var_t2_dn6 = assign15130_e9925_d_n6;
        locals.var_t2_dn7 = assign15130_e9925_d_n7;
        locals.var_t2_dn8 = assign15130_e9925_d_n8;
        locals.var_t2_dn9 = assign15130_e9925_d_n9;
        locals.var_t2_dn10 = assign15130_e9925_d_n10;
        locals.var_t2_dn11 = assign15130_e9925_d_n11;
        locals.var_t2_dn14 = assign15130_e9925_d_n14;

        let assign15140_e9932: f64 = if ((p.p39 == 0.0) || (p.p39 == 1.0)) { 1.0 } else { 0.0 };
        locals.var_guard323 = assign15140_e9932;

        let (assign15150_e9952, assign15150_e9952_d_n0, assign15150_e9952_d_n2, assign15150_e9952_d_n4, assign15150_e9952_d_n5, assign15150_e9952_d_n6, assign15150_e9952_d_n7, assign15150_e9952_d_n8, assign15150_e9952_d_n9, assign15150_e9952_d_n10, assign15150_e9952_d_n11, assign15150_e9952_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign15150_e9943: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff0);
        let assign15150_e9944: f64 = (locals.var_uc_rdvd + assign15150_e9943);
        let assign15150_e9947: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2);
        let assign15150_e9948: f64 = (assign15150_e9944 + assign15150_e9947);
        let assign15150_e9950: f64 = (assign15150_e9948 * locals.var_t2);
        (assign15150_e9950, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn0)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn2)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn4)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn5)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn6)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn7)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn8)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn9)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn10)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn11) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn11)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn11)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff0_dn14) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff0_2_dn14)) * locals.var_t2) + (assign15150_e9948 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign15150_e9952;
        locals.var_rsvde_dn0 = assign15150_e9952_d_n0;
        locals.var_rsvde_dn2 = assign15150_e9952_d_n2;
        locals.var_rsvde_dn4 = assign15150_e9952_d_n4;
        locals.var_rsvde_dn5 = assign15150_e9952_d_n5;
        locals.var_rsvde_dn6 = assign15150_e9952_d_n6;
        locals.var_rsvde_dn7 = assign15150_e9952_d_n7;
        locals.var_rsvde_dn8 = assign15150_e9952_d_n8;
        locals.var_rsvde_dn9 = assign15150_e9952_d_n9;
        locals.var_rsvde_dn10 = assign15150_e9952_d_n10;
        locals.var_rsvde_dn11 = assign15150_e9952_d_n11;
        locals.var_rsvde_dn14 = assign15150_e9952_d_n14;

        let (assign15160_e9970, assign15160_e9970_d_n0, assign15160_e9970_d_n2, assign15160_e9970_d_n4, assign15160_e9970_d_n5, assign15160_e9970_d_n6, assign15160_e9970_d_n7, assign15160_e9970_d_n8, assign15160_e9970_d_n9, assign15160_e9970_d_n10, assign15160_e9970_d_n11, assign15160_e9970_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign15160_e9963: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15160_e9964: f64 = (locals.var_rsvde - assign15160_e9963);
        let assign15160_e9967: f64 = (0.01 * locals.var_uc_rdvd);
        let assign15160_e9968: f64 = (assign15160_e9964 - assign15160_e9967);
        (assign15160_e9968, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign15160_e9970;
        locals.var_tmf1_dn0 = assign15160_e9970_d_n0;
        locals.var_tmf1_dn2 = assign15160_e9970_d_n2;
        locals.var_tmf1_dn4 = assign15160_e9970_d_n4;
        locals.var_tmf1_dn5 = assign15160_e9970_d_n5;
        locals.var_tmf1_dn6 = assign15160_e9970_d_n6;
        locals.var_tmf1_dn7 = assign15160_e9970_d_n7;
        locals.var_tmf1_dn8 = assign15160_e9970_d_n8;
        locals.var_tmf1_dn9 = assign15160_e9970_d_n9;
        locals.var_tmf1_dn10 = assign15160_e9970_d_n10;
        locals.var_tmf1_dn11 = assign15160_e9970_d_n11;
        locals.var_tmf1_dn14 = assign15160_e9970_d_n14;

        let (assign15170_e9988, assign15170_e9988_d_n0, assign15170_e9988_d_n2, assign15170_e9988_d_n4, assign15170_e9988_d_n5, assign15170_e9988_d_n6, assign15170_e9988_d_n7, assign15170_e9988_d_n8, assign15170_e9988_d_n9, assign15170_e9988_d_n10, assign15170_e9988_d_n11, assign15170_e9988_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign15170_e9981: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15170_e9982: f64 = (4.0 * assign15170_e9981);
        let assign15170_e9985: f64 = (0.01 * locals.var_uc_rdvd);
        let assign15170_e9986: f64 = (assign15170_e9982 * assign15170_e9985);
        (assign15170_e9986, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15170_e9988;
        locals.var_tmf2_dn0 = assign15170_e9988_d_n0;
        locals.var_tmf2_dn2 = assign15170_e9988_d_n2;
        locals.var_tmf2_dn4 = assign15170_e9988_d_n4;
        locals.var_tmf2_dn5 = assign15170_e9988_d_n5;
        locals.var_tmf2_dn6 = assign15170_e9988_d_n6;
        locals.var_tmf2_dn7 = assign15170_e9988_d_n7;
        locals.var_tmf2_dn8 = assign15170_e9988_d_n8;
        locals.var_tmf2_dn9 = assign15170_e9988_d_n9;
        locals.var_tmf2_dn10 = assign15170_e9988_d_n10;
        locals.var_tmf2_dn11 = assign15170_e9988_d_n11;
        locals.var_tmf2_dn14 = assign15170_e9988_d_n14;

        let (assign15180_e10004, assign15180_e10004_d_n0, assign15180_e10004_d_n2, assign15180_e10004_d_n4, assign15180_e10004_d_n5, assign15180_e10004_d_n6, assign15180_e10004_d_n7, assign15180_e10004_d_n8, assign15180_e10004_d_n9, assign15180_e10004_d_n10, assign15180_e10004_d_n11, assign15180_e10004_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 != 0.0)) {
        let (assign15180_e10002, assign15180_e10002_d_n0, assign15180_e10002_d_n2, assign15180_e10002_d_n4, assign15180_e10002_d_n5, assign15180_e10002_d_n6, assign15180_e10002_d_n7, assign15180_e10002_d_n8, assign15180_e10002_d_n9, assign15180_e10002_d_n10, assign15180_e10002_d_n11, assign15180_e10002_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign15180_e10001: f64 = (-locals.var_tmf2);
                (assign15180_e10001, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign15180_e10002, assign15180_e10002_d_n0, assign15180_e10002_d_n2, assign15180_e10002_d_n4, assign15180_e10002_d_n5, assign15180_e10002_d_n6, assign15180_e10002_d_n7, assign15180_e10002_d_n8, assign15180_e10002_d_n9, assign15180_e10002_d_n10, assign15180_e10002_d_n11, assign15180_e10002_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15180_e10004;
        locals.var_tmf2_dn0 = assign15180_e10004_d_n0;
        locals.var_tmf2_dn2 = assign15180_e10004_d_n2;
        locals.var_tmf2_dn4 = assign15180_e10004_d_n4;
        locals.var_tmf2_dn5 = assign15180_e10004_d_n5;
        locals.var_tmf2_dn6 = assign15180_e10004_d_n6;
        locals.var_tmf2_dn7 = assign15180_e10004_d_n7;
        locals.var_tmf2_dn8 = assign15180_e10004_d_n8;
        locals.var_tmf2_dn9 = assign15180_e10004_d_n9;
        locals.var_tmf2_dn10 = assign15180_e10004_d_n10;
        locals.var_tmf2_dn11 = assign15180_e10004_d_n11;
        locals.var_tmf2_dn14 = assign15180_e10004_d_n14;

        let (assign15190_e10019, assign15190_e10019_d_n0, assign15190_e10019_d_n2, assign15190_e10019_d_n4, assign15190_e10019_d_n5, assign15190_e10019_d_n6, assign15190_e10019_d_n7, assign15190_e10019_d_n8, assign15190_e10019_d_n9, assign15190_e10019_d_n10, assign15190_e10019_d_n11, assign15190_e10019_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign15190_e10014: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15190_e10016: f64 = (assign15190_e10014 + locals.var_tmf2);
        let assign15190_e10017: f64 = (assign15190_e10016).sqrt();
        (assign15190_e10017, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15190_e10017)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15190_e10017)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign15190_e10017)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign15190_e10017)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign15190_e10017)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign15190_e10017)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign15190_e10017)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign15190_e10017)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign15190_e10017)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign15190_e10017)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign15190_e10017)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15190_e10019;
        locals.var_tmf2_dn0 = assign15190_e10019_d_n0;
        locals.var_tmf2_dn2 = assign15190_e10019_d_n2;
        locals.var_tmf2_dn4 = assign15190_e10019_d_n4;
        locals.var_tmf2_dn5 = assign15190_e10019_d_n5;
        locals.var_tmf2_dn6 = assign15190_e10019_d_n6;
        locals.var_tmf2_dn7 = assign15190_e10019_d_n7;
        locals.var_tmf2_dn8 = assign15190_e10019_d_n8;
        locals.var_tmf2_dn9 = assign15190_e10019_d_n9;
        locals.var_tmf2_dn10 = assign15190_e10019_d_n10;
        locals.var_tmf2_dn11 = assign15190_e10019_d_n11;
        locals.var_tmf2_dn14 = assign15190_e10019_d_n14;

        let (assign15200_e10035, assign15200_e10035_d_n0, assign15200_e10035_d_n2, assign15200_e10035_d_n4, assign15200_e10035_d_n5, assign15200_e10035_d_n6, assign15200_e10035_d_n7, assign15200_e10035_d_n8, assign15200_e10035_d_n9, assign15200_e10035_d_n10, assign15200_e10035_d_n11, assign15200_e10035_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign15200_e10031: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15200_e10032: f64 = (1.0 + assign15200_e10031);
        let assign15200_e10033: f64 = (0.5 * assign15200_e10032);
        (assign15200_e10033, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign15200_e10035;
        locals.var_t0_dn0 = assign15200_e10035_d_n0;
        locals.var_t0_dn2 = assign15200_e10035_d_n2;
        locals.var_t0_dn4 = assign15200_e10035_d_n4;
        locals.var_t0_dn5 = assign15200_e10035_d_n5;
        locals.var_t0_dn6 = assign15200_e10035_d_n6;
        locals.var_t0_dn7 = assign15200_e10035_d_n7;
        locals.var_t0_dn8 = assign15200_e10035_d_n8;
        locals.var_t0_dn9 = assign15200_e10035_d_n9;
        locals.var_t0_dn10 = assign15200_e10035_d_n10;
        locals.var_t0_dn11 = assign15200_e10035_d_n11;
        locals.var_t0_dn14 = assign15200_e10035_d_n14;

        let (assign15210_e10053, assign15210_e10053_d_n0, assign15210_e10053_d_n2, assign15210_e10053_d_n4, assign15210_e10053_d_n5, assign15210_e10053_d_n6, assign15210_e10053_d_n7, assign15210_e10053_d_n8, assign15210_e10053_d_n9, assign15210_e10053_d_n10, assign15210_e10053_d_n11, assign15210_e10053_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 != 0.0)) {
        let assign15210_e10045: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15210_e10049: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15210_e10050: f64 = (0.5 * assign15210_e10049);
        let assign15210_e10051: f64 = (assign15210_e10045 + assign15210_e10050);
        (assign15210_e10051, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign15210_e10053;
        locals.var_rsvde_dn0 = assign15210_e10053_d_n0;
        locals.var_rsvde_dn2 = assign15210_e10053_d_n2;
        locals.var_rsvde_dn4 = assign15210_e10053_d_n4;
        locals.var_rsvde_dn5 = assign15210_e10053_d_n5;
        locals.var_rsvde_dn6 = assign15210_e10053_d_n6;
        locals.var_rsvde_dn7 = assign15210_e10053_d_n7;
        locals.var_rsvde_dn8 = assign15210_e10053_d_n8;
        locals.var_rsvde_dn9 = assign15210_e10053_d_n9;
        locals.var_rsvde_dn10 = assign15210_e10053_d_n10;
        locals.var_rsvde_dn11 = assign15210_e10053_d_n11;
        locals.var_rsvde_dn14 = assign15210_e10053_d_n14;

        let (assign15220_e10074, assign15220_e10074_d_n0, assign15220_e10074_d_n2, assign15220_e10074_d_n4, assign15220_e10074_d_n5, assign15220_e10074_d_n6, assign15220_e10074_d_n7, assign15220_e10074_d_n8, assign15220_e10074_d_n9, assign15220_e10074_d_n10, assign15220_e10074_d_n11, assign15220_e10074_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 == 0.0)) {
        let assign15220_e10065: f64 = (locals.var_mks_rdvdtemp1 * locals.var_tdiff);
        let assign15220_e10066: f64 = (locals.var_uc_rdvd + assign15220_e10065);
        let assign15220_e10069: f64 = (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2);
        let assign15220_e10070: f64 = (assign15220_e10066 + assign15220_e10069);
        let assign15220_e10072: f64 = (assign15220_e10070 * locals.var_t2);
        (assign15220_e10072, ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn0) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn0)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn0)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn2) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn2)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn2)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn4) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn4)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn4)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn5) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn5)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn5)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn6) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn6)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn6)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn7) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn7)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn7)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn8) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn8)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn8)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn9) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn9)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn9)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn10) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn10)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn10)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn11) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn11)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn11)), ((((locals.var_mks_rdvdtemp1 * locals.var_tdiff_dn14) + (locals.var_mks_rdvdtemp2 * locals.var_tdiff_2_dn14)) * locals.var_t2) + (assign15220_e10070 * locals.var_t2_dn14)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign15220_e10074;
        locals.var_rsvde_dn0 = assign15220_e10074_d_n0;
        locals.var_rsvde_dn2 = assign15220_e10074_d_n2;
        locals.var_rsvde_dn4 = assign15220_e10074_d_n4;
        locals.var_rsvde_dn5 = assign15220_e10074_d_n5;
        locals.var_rsvde_dn6 = assign15220_e10074_d_n6;
        locals.var_rsvde_dn7 = assign15220_e10074_d_n7;
        locals.var_rsvde_dn8 = assign15220_e10074_d_n8;
        locals.var_rsvde_dn9 = assign15220_e10074_d_n9;
        locals.var_rsvde_dn10 = assign15220_e10074_d_n10;
        locals.var_rsvde_dn11 = assign15220_e10074_d_n11;
        locals.var_rsvde_dn14 = assign15220_e10074_d_n14;

        let (assign15230_e10093, assign15230_e10093_d_n0, assign15230_e10093_d_n2, assign15230_e10093_d_n4, assign15230_e10093_d_n5, assign15230_e10093_d_n6, assign15230_e10093_d_n7, assign15230_e10093_d_n8, assign15230_e10093_d_n9, assign15230_e10093_d_n10, assign15230_e10093_d_n11, assign15230_e10093_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 == 0.0)) {
        let assign15230_e10086: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15230_e10087: f64 = (locals.var_rsvde - assign15230_e10086);
        let assign15230_e10090: f64 = (0.01 * locals.var_uc_rdvd);
        let assign15230_e10091: f64 = (assign15230_e10087 - assign15230_e10090);
        (assign15230_e10091, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    } else {
        (locals.var_tmf1, locals.var_tmf1_dn0, locals.var_tmf1_dn2, locals.var_tmf1_dn4, locals.var_tmf1_dn5, locals.var_tmf1_dn6, locals.var_tmf1_dn7, locals.var_tmf1_dn8, locals.var_tmf1_dn9, locals.var_tmf1_dn10, locals.var_tmf1_dn11, locals.var_tmf1_dn14,)
    }
};
        locals.var_tmf1 = assign15230_e10093;
        locals.var_tmf1_dn0 = assign15230_e10093_d_n0;
        locals.var_tmf1_dn2 = assign15230_e10093_d_n2;
        locals.var_tmf1_dn4 = assign15230_e10093_d_n4;
        locals.var_tmf1_dn5 = assign15230_e10093_d_n5;
        locals.var_tmf1_dn6 = assign15230_e10093_d_n6;
        locals.var_tmf1_dn7 = assign15230_e10093_d_n7;
        locals.var_tmf1_dn8 = assign15230_e10093_d_n8;
        locals.var_tmf1_dn9 = assign15230_e10093_d_n9;
        locals.var_tmf1_dn10 = assign15230_e10093_d_n10;
        locals.var_tmf1_dn11 = assign15230_e10093_d_n11;
        locals.var_tmf1_dn14 = assign15230_e10093_d_n14;

        let (assign15240_e10112, assign15240_e10112_d_n0, assign15240_e10112_d_n2, assign15240_e10112_d_n4, assign15240_e10112_d_n5, assign15240_e10112_d_n6, assign15240_e10112_d_n7, assign15240_e10112_d_n8, assign15240_e10112_d_n9, assign15240_e10112_d_n10, assign15240_e10112_d_n11, assign15240_e10112_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 == 0.0)) {
        let assign15240_e10105: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15240_e10106: f64 = (4.0 * assign15240_e10105);
        let assign15240_e10109: f64 = (0.01 * locals.var_uc_rdvd);
        let assign15240_e10110: f64 = (assign15240_e10106 * assign15240_e10109);
        (assign15240_e10110, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15240_e10112;
        locals.var_tmf2_dn0 = assign15240_e10112_d_n0;
        locals.var_tmf2_dn2 = assign15240_e10112_d_n2;
        locals.var_tmf2_dn4 = assign15240_e10112_d_n4;
        locals.var_tmf2_dn5 = assign15240_e10112_d_n5;
        locals.var_tmf2_dn6 = assign15240_e10112_d_n6;
        locals.var_tmf2_dn7 = assign15240_e10112_d_n7;
        locals.var_tmf2_dn8 = assign15240_e10112_d_n8;
        locals.var_tmf2_dn9 = assign15240_e10112_d_n9;
        locals.var_tmf2_dn10 = assign15240_e10112_d_n10;
        locals.var_tmf2_dn11 = assign15240_e10112_d_n11;
        locals.var_tmf2_dn14 = assign15240_e10112_d_n14;

        let (assign15250_e10129, assign15250_e10129_d_n0, assign15250_e10129_d_n2, assign15250_e10129_d_n4, assign15250_e10129_d_n5, assign15250_e10129_d_n6, assign15250_e10129_d_n7, assign15250_e10129_d_n8, assign15250_e10129_d_n9, assign15250_e10129_d_n10, assign15250_e10129_d_n11, assign15250_e10129_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 == 0.0)) {
        let (assign15250_e10127, assign15250_e10127_d_n0, assign15250_e10127_d_n2, assign15250_e10127_d_n4, assign15250_e10127_d_n5, assign15250_e10127_d_n6, assign15250_e10127_d_n7, assign15250_e10127_d_n8, assign15250_e10127_d_n9, assign15250_e10127_d_n10, assign15250_e10127_d_n11, assign15250_e10127_d_n14,) = {
            if (locals.var_tmf2 > 0.0) {
                (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
            } else {
                let assign15250_e10126: f64 = (-locals.var_tmf2);
                (assign15250_e10126, (-locals.var_tmf2_dn0), (-locals.var_tmf2_dn2), (-locals.var_tmf2_dn4), (-locals.var_tmf2_dn5), (-locals.var_tmf2_dn6), (-locals.var_tmf2_dn7), (-locals.var_tmf2_dn8), (-locals.var_tmf2_dn9), (-locals.var_tmf2_dn10), (-locals.var_tmf2_dn11), (-locals.var_tmf2_dn14),)
            }
        };
        (assign15250_e10127, assign15250_e10127_d_n0, assign15250_e10127_d_n2, assign15250_e10127_d_n4, assign15250_e10127_d_n5, assign15250_e10127_d_n6, assign15250_e10127_d_n7, assign15250_e10127_d_n8, assign15250_e10127_d_n9, assign15250_e10127_d_n10, assign15250_e10127_d_n11, assign15250_e10127_d_n14,)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15250_e10129;
        locals.var_tmf2_dn0 = assign15250_e10129_d_n0;
        locals.var_tmf2_dn2 = assign15250_e10129_d_n2;
        locals.var_tmf2_dn4 = assign15250_e10129_d_n4;
        locals.var_tmf2_dn5 = assign15250_e10129_d_n5;
        locals.var_tmf2_dn6 = assign15250_e10129_d_n6;
        locals.var_tmf2_dn7 = assign15250_e10129_d_n7;
        locals.var_tmf2_dn8 = assign15250_e10129_d_n8;
        locals.var_tmf2_dn9 = assign15250_e10129_d_n9;
        locals.var_tmf2_dn10 = assign15250_e10129_d_n10;
        locals.var_tmf2_dn11 = assign15250_e10129_d_n11;
        locals.var_tmf2_dn14 = assign15250_e10129_d_n14;

        let (assign15260_e10145, assign15260_e10145_d_n0, assign15260_e10145_d_n2, assign15260_e10145_d_n4, assign15260_e10145_d_n5, assign15260_e10145_d_n6, assign15260_e10145_d_n7, assign15260_e10145_d_n8, assign15260_e10145_d_n9, assign15260_e10145_d_n10, assign15260_e10145_d_n11, assign15260_e10145_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 == 0.0)) {
        let assign15260_e10140: f64 = (locals.var_tmf1 * locals.var_tmf1);
        let assign15260_e10142: f64 = (assign15260_e10140 + locals.var_tmf2);
        let assign15260_e10143: f64 = (assign15260_e10142).sqrt();
        (assign15260_e10143, ((((locals.var_tmf1_dn0 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn0)) + locals.var_tmf2_dn0) / (2.0 * assign15260_e10143)), ((((locals.var_tmf1_dn2 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn2)) + locals.var_tmf2_dn2) / (2.0 * assign15260_e10143)), ((((locals.var_tmf1_dn4 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn4)) + locals.var_tmf2_dn4) / (2.0 * assign15260_e10143)), ((((locals.var_tmf1_dn5 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn5)) + locals.var_tmf2_dn5) / (2.0 * assign15260_e10143)), ((((locals.var_tmf1_dn6 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn6)) + locals.var_tmf2_dn6) / (2.0 * assign15260_e10143)), ((((locals.var_tmf1_dn7 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn7)) + locals.var_tmf2_dn7) / (2.0 * assign15260_e10143)), ((((locals.var_tmf1_dn8 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn8)) + locals.var_tmf2_dn8) / (2.0 * assign15260_e10143)), ((((locals.var_tmf1_dn9 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn9)) + locals.var_tmf2_dn9) / (2.0 * assign15260_e10143)), ((((locals.var_tmf1_dn10 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn10)) + locals.var_tmf2_dn10) / (2.0 * assign15260_e10143)), ((((locals.var_tmf1_dn11 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn11)) + locals.var_tmf2_dn11) / (2.0 * assign15260_e10143)), ((((locals.var_tmf1_dn14 * locals.var_tmf1) + (locals.var_tmf1 * locals.var_tmf1_dn14)) + locals.var_tmf2_dn14) / (2.0 * assign15260_e10143)),)
    } else {
        (locals.var_tmf2, locals.var_tmf2_dn0, locals.var_tmf2_dn2, locals.var_tmf2_dn4, locals.var_tmf2_dn5, locals.var_tmf2_dn6, locals.var_tmf2_dn7, locals.var_tmf2_dn8, locals.var_tmf2_dn9, locals.var_tmf2_dn10, locals.var_tmf2_dn11, locals.var_tmf2_dn14,)
    }
};
        locals.var_tmf2 = assign15260_e10145;
        locals.var_tmf2_dn0 = assign15260_e10145_d_n0;
        locals.var_tmf2_dn2 = assign15260_e10145_d_n2;
        locals.var_tmf2_dn4 = assign15260_e10145_d_n4;
        locals.var_tmf2_dn5 = assign15260_e10145_d_n5;
        locals.var_tmf2_dn6 = assign15260_e10145_d_n6;
        locals.var_tmf2_dn7 = assign15260_e10145_d_n7;
        locals.var_tmf2_dn8 = assign15260_e10145_d_n8;
        locals.var_tmf2_dn9 = assign15260_e10145_d_n9;
        locals.var_tmf2_dn10 = assign15260_e10145_d_n10;
        locals.var_tmf2_dn11 = assign15260_e10145_d_n11;
        locals.var_tmf2_dn14 = assign15260_e10145_d_n14;

        let (assign15270_e10162, assign15270_e10162_d_n0, assign15270_e10162_d_n2, assign15270_e10162_d_n4, assign15270_e10162_d_n5, assign15270_e10162_d_n6, assign15270_e10162_d_n7, assign15270_e10162_d_n8, assign15270_e10162_d_n9, assign15270_e10162_d_n10, assign15270_e10162_d_n11, assign15270_e10162_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 == 0.0)) {
        let assign15270_e10158: f64 = (locals.var_tmf1 / locals.var_tmf2);
        let assign15270_e10159: f64 = (1.0 + assign15270_e10158);
        let assign15270_e10160: f64 = (0.5 * assign15270_e10159);
        (assign15270_e10160, (0.5 * (((locals.var_tmf1_dn0 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn0)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn2 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn2)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn4 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn4)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn5 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn5)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn6 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn6)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn7 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn7)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn8 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn8)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn9 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn9)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn10 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn10)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn11 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn11)) / (locals.var_tmf2 * locals.var_tmf2))), (0.5 * (((locals.var_tmf1_dn14 * locals.var_tmf2) - (locals.var_tmf1 * locals.var_tmf2_dn14)) / (locals.var_tmf2 * locals.var_tmf2))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign15270_e10162;
        locals.var_t0_dn0 = assign15270_e10162_d_n0;
        locals.var_t0_dn2 = assign15270_e10162_d_n2;
        locals.var_t0_dn4 = assign15270_e10162_d_n4;
        locals.var_t0_dn5 = assign15270_e10162_d_n5;
        locals.var_t0_dn6 = assign15270_e10162_d_n6;
        locals.var_t0_dn7 = assign15270_e10162_d_n7;
        locals.var_t0_dn8 = assign15270_e10162_d_n8;
        locals.var_t0_dn9 = assign15270_e10162_d_n9;
        locals.var_t0_dn10 = assign15270_e10162_d_n10;
        locals.var_t0_dn11 = assign15270_e10162_d_n11;
        locals.var_t0_dn14 = assign15270_e10162_d_n14;

        let (assign15280_e10181, assign15280_e10181_d_n0, assign15280_e10181_d_n2, assign15280_e10181_d_n4, assign15280_e10181_d_n5, assign15280_e10181_d_n6, assign15280_e10181_d_n7, assign15280_e10181_d_n8, assign15280_e10181_d_n9, assign15280_e10181_d_n10, assign15280_e10181_d_n11, assign15280_e10181_d_n14,) = {
    if ((((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 != 0.0)) && (locals.var_guard323 == 0.0)) {
        let assign15280_e10173: f64 = (0.005 * locals.var_uc_rdvd);
        let assign15280_e10177: f64 = (locals.var_tmf1 + locals.var_tmf2);
        let assign15280_e10178: f64 = (0.5 * assign15280_e10177);
        let assign15280_e10179: f64 = (assign15280_e10173 + assign15280_e10178);
        (assign15280_e10179, (0.5 * (locals.var_tmf1_dn0 + locals.var_tmf2_dn0)), (0.5 * (locals.var_tmf1_dn2 + locals.var_tmf2_dn2)), (0.5 * (locals.var_tmf1_dn4 + locals.var_tmf2_dn4)), (0.5 * (locals.var_tmf1_dn5 + locals.var_tmf2_dn5)), (0.5 * (locals.var_tmf1_dn6 + locals.var_tmf2_dn6)), (0.5 * (locals.var_tmf1_dn7 + locals.var_tmf2_dn7)), (0.5 * (locals.var_tmf1_dn8 + locals.var_tmf2_dn8)), (0.5 * (locals.var_tmf1_dn9 + locals.var_tmf2_dn9)), (0.5 * (locals.var_tmf1_dn10 + locals.var_tmf2_dn10)), (0.5 * (locals.var_tmf1_dn11 + locals.var_tmf2_dn11)), (0.5 * (locals.var_tmf1_dn14 + locals.var_tmf2_dn14)),)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign15280_e10181;
        locals.var_rsvde_dn0 = assign15280_e10181_d_n0;
        locals.var_rsvde_dn2 = assign15280_e10181_d_n2;
        locals.var_rsvde_dn4 = assign15280_e10181_d_n4;
        locals.var_rsvde_dn5 = assign15280_e10181_d_n5;
        locals.var_rsvde_dn6 = assign15280_e10181_d_n6;
        locals.var_rsvde_dn7 = assign15280_e10181_d_n7;
        locals.var_rsvde_dn8 = assign15280_e10181_d_n8;
        locals.var_rsvde_dn9 = assign15280_e10181_d_n9;
        locals.var_rsvde_dn10 = assign15280_e10181_d_n10;
        locals.var_rsvde_dn11 = assign15280_e10181_d_n11;
        locals.var_rsvde_dn14 = assign15280_e10181_d_n14;

        let (assign15290_e10190, assign15290_e10190_d_n0, assign15290_e10190_d_n2, assign15290_e10190_d_n4, assign15290_e10190_d_n5, assign15290_e10190_d_n6, assign15290_e10190_d_n7, assign15290_e10190_d_n8, assign15290_e10190_d_n9, assign15290_e10190_d_n10, assign15290_e10190_d_n11, assign15290_e10190_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdvde, locals.var_rdvde_dn0, locals.var_rdvde_dn2, locals.var_rdvde_dn4, locals.var_rdvde_dn5, locals.var_rdvde_dn6, locals.var_rdvde_dn7, locals.var_rdvde_dn8, locals.var_rdvde_dn9, locals.var_rdvde_dn10, locals.var_rdvde_dn11, locals.var_rdvde_dn14,)
    }
};
        locals.var_rdvde = assign15290_e10190;
        locals.var_rdvde_dn0 = assign15290_e10190_d_n0;
        locals.var_rdvde_dn2 = assign15290_e10190_d_n2;
        locals.var_rdvde_dn4 = assign15290_e10190_d_n4;
        locals.var_rdvde_dn5 = assign15290_e10190_d_n5;
        locals.var_rdvde_dn6 = assign15290_e10190_d_n6;
        locals.var_rdvde_dn7 = assign15290_e10190_d_n7;
        locals.var_rdvde_dn8 = assign15290_e10190_d_n8;
        locals.var_rdvde_dn9 = assign15290_e10190_d_n9;
        locals.var_rdvde_dn10 = assign15290_e10190_d_n10;
        locals.var_rdvde_dn11 = assign15290_e10190_d_n11;
        locals.var_rdvde_dn14 = assign15290_e10190_d_n14;

        let (assign15300_e10199, assign15300_e10199_d_n0, assign15300_e10199_d_n2, assign15300_e10199_d_n4, assign15300_e10199_d_n5, assign15300_e10199_d_n6, assign15300_e10199_d_n7, assign15300_e10199_d_n8, assign15300_e10199_d_n9, assign15300_e10199_d_n10, assign15300_e10199_d_n11, assign15300_e10199_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard315 != 0.0)) && (locals.var_guard320 == 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rsvde, locals.var_rsvde_dn0, locals.var_rsvde_dn2, locals.var_rsvde_dn4, locals.var_rsvde_dn5, locals.var_rsvde_dn6, locals.var_rsvde_dn7, locals.var_rsvde_dn8, locals.var_rsvde_dn9, locals.var_rsvde_dn10, locals.var_rsvde_dn11, locals.var_rsvde_dn14,)
    }
};
        locals.var_rsvde = assign15300_e10199;
        locals.var_rsvde_dn0 = assign15300_e10199_d_n0;
        locals.var_rsvde_dn2 = assign15300_e10199_d_n2;
        locals.var_rsvde_dn4 = assign15300_e10199_d_n4;
        locals.var_rsvde_dn5 = assign15300_e10199_d_n5;
        locals.var_rsvde_dn6 = assign15300_e10199_d_n6;
        locals.var_rsvde_dn7 = assign15300_e10199_d_n7;
        locals.var_rsvde_dn8 = assign15300_e10199_d_n8;
        locals.var_rsvde_dn9 = assign15300_e10199_d_n9;
        locals.var_rsvde_dn10 = assign15300_e10199_d_n10;
        locals.var_rsvde_dn11 = assign15300_e10199_d_n11;
        locals.var_rsvde_dn14 = assign15300_e10199_d_n14;

        let (assign15310_e10206, assign15310_e10206_d_n0, assign15310_e10206_d_n2, assign15310_e10206_d_n4, assign15310_e10206_d_n5, assign15310_e10206_d_n6, assign15310_e10206_d_n7, assign15310_e10206_d_n8, assign15310_e10206_d_n9, assign15310_e10206_d_n10, assign15310_e10206_d_n11, assign15310_e10206_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15310_e10203: f64 = (locals.var_beta_inv).sqrt();
        let assign15310_e10204: f64 = (locals.var_costi00 * assign15310_e10203);
        (assign15310_e10204, (locals.var_costi00 * (locals.var_beta_inv_dn0 / (2.0 * assign15310_e10203))), (locals.var_costi00 * (locals.var_beta_inv_dn2 / (2.0 * assign15310_e10203))), (locals.var_costi00 * (locals.var_beta_inv_dn4 / (2.0 * assign15310_e10203))), (locals.var_costi00 * (locals.var_beta_inv_dn5 / (2.0 * assign15310_e10203))), (locals.var_costi00 * (locals.var_beta_inv_dn6 / (2.0 * assign15310_e10203))), (locals.var_costi00 * (locals.var_beta_inv_dn7 / (2.0 * assign15310_e10203))), (locals.var_costi00 * (locals.var_beta_inv_dn8 / (2.0 * assign15310_e10203))), (locals.var_costi00 * (locals.var_beta_inv_dn9 / (2.0 * assign15310_e10203))), (locals.var_costi00 * (locals.var_beta_inv_dn10 / (2.0 * assign15310_e10203))), (locals.var_costi00 * (locals.var_beta_inv_dn11 / (2.0 * assign15310_e10203))), (locals.var_costi00 * (locals.var_beta_inv_dn14 / (2.0 * assign15310_e10203))),)
    } else {
        (locals.var_costi0, locals.var_costi0_dn0, locals.var_costi0_dn2, locals.var_costi0_dn4, locals.var_costi0_dn5, locals.var_costi0_dn6, locals.var_costi0_dn7, locals.var_costi0_dn8, locals.var_costi0_dn9, locals.var_costi0_dn10, locals.var_costi0_dn11, locals.var_costi0_dn14,)
    }
};
        locals.var_costi0 = assign15310_e10206;
        locals.var_costi0_dn0 = assign15310_e10206_d_n0;
        locals.var_costi0_dn2 = assign15310_e10206_d_n2;
        locals.var_costi0_dn4 = assign15310_e10206_d_n4;
        locals.var_costi0_dn5 = assign15310_e10206_d_n5;
        locals.var_costi0_dn6 = assign15310_e10206_d_n6;
        locals.var_costi0_dn7 = assign15310_e10206_d_n7;
        locals.var_costi0_dn8 = assign15310_e10206_d_n8;
        locals.var_costi0_dn9 = assign15310_e10206_d_n9;
        locals.var_costi0_dn10 = assign15310_e10206_d_n10;
        locals.var_costi0_dn11 = assign15310_e10206_d_n11;
        locals.var_costi0_dn14 = assign15310_e10206_d_n14;

    }

    pub(super) fn stamp_transient_block_31(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15320_e10212, assign15320_e10212_d_n0, assign15320_e10212_d_n2, assign15320_e10212_d_n4, assign15320_e10212_d_n5, assign15320_e10212_d_n6, assign15320_e10212_d_n7, assign15320_e10212_d_n8, assign15320_e10212_d_n9, assign15320_e10212_d_n10, assign15320_e10212_d_n11, assign15320_e10212_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15320_e10210: f64 = (locals.var_costi0 * locals.var_costi0);
        (assign15320_e10210, ((locals.var_costi0_dn0 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn0)), ((locals.var_costi0_dn2 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn2)), ((locals.var_costi0_dn4 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn4)), ((locals.var_costi0_dn5 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn5)), ((locals.var_costi0_dn6 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn6)), ((locals.var_costi0_dn7 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn7)), ((locals.var_costi0_dn8 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn8)), ((locals.var_costi0_dn9 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn9)), ((locals.var_costi0_dn10 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn10)), ((locals.var_costi0_dn11 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn11)), ((locals.var_costi0_dn14 * locals.var_costi0) + (locals.var_costi0 * locals.var_costi0_dn14)),)
    } else {
        (locals.var_costi0_p2, locals.var_costi0_p2_dn0, locals.var_costi0_p2_dn2, locals.var_costi0_p2_dn4, locals.var_costi0_p2_dn5, locals.var_costi0_p2_dn6, locals.var_costi0_p2_dn7, locals.var_costi0_p2_dn8, locals.var_costi0_p2_dn9, locals.var_costi0_p2_dn10, locals.var_costi0_p2_dn11, locals.var_costi0_p2_dn14,)
    }
};
        locals.var_costi0_p2 = assign15320_e10212;
        locals.var_costi0_p2_dn0 = assign15320_e10212_d_n0;
        locals.var_costi0_p2_dn2 = assign15320_e10212_d_n2;
        locals.var_costi0_p2_dn4 = assign15320_e10212_d_n4;
        locals.var_costi0_p2_dn5 = assign15320_e10212_d_n5;
        locals.var_costi0_p2_dn6 = assign15320_e10212_d_n6;
        locals.var_costi0_p2_dn7 = assign15320_e10212_d_n7;
        locals.var_costi0_p2_dn8 = assign15320_e10212_d_n8;
        locals.var_costi0_p2_dn9 = assign15320_e10212_d_n9;
        locals.var_costi0_p2_dn10 = assign15320_e10212_d_n10;
        locals.var_costi0_p2_dn11 = assign15320_e10212_d_n11;
        locals.var_costi0_p2_dn14 = assign15320_e10212_d_n14;

        let (assign15330_e10220, assign15330_e10220_d_n0, assign15330_e10220_d_n2, assign15330_e10220_d_n4, assign15330_e10220_d_n5, assign15330_e10220_d_n6, assign15330_e10220_d_n7, assign15330_e10220_d_n8, assign15330_e10220_d_n9, assign15330_e10220_d_n10, assign15330_e10220_d_n11, assign15330_e10220_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15330_e10216: f64 = (locals.var_nin * locals.var_nin);
        let assign15330_e10218: f64 = (assign15330_e10216 * locals.var_nsti_p2);
        (assign15330_e10218, (((locals.var_nin_dn0 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn0)) * locals.var_nsti_p2), (((locals.var_nin_dn2 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn2)) * locals.var_nsti_p2), (((locals.var_nin_dn4 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn4)) * locals.var_nsti_p2), (((locals.var_nin_dn5 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn5)) * locals.var_nsti_p2), (((locals.var_nin_dn6 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn6)) * locals.var_nsti_p2), (((locals.var_nin_dn7 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn7)) * locals.var_nsti_p2), (((locals.var_nin_dn8 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn8)) * locals.var_nsti_p2), (((locals.var_nin_dn9 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn9)) * locals.var_nsti_p2), (((locals.var_nin_dn10 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn10)) * locals.var_nsti_p2), (((locals.var_nin_dn11 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn11)) * locals.var_nsti_p2), (((locals.var_nin_dn14 * locals.var_nin) + (locals.var_nin * locals.var_nin_dn14)) * locals.var_nsti_p2),)
    } else {
        (locals.var_costi1, locals.var_costi1_dn0, locals.var_costi1_dn2, locals.var_costi1_dn4, locals.var_costi1_dn5, locals.var_costi1_dn6, locals.var_costi1_dn7, locals.var_costi1_dn8, locals.var_costi1_dn9, locals.var_costi1_dn10, locals.var_costi1_dn11, locals.var_costi1_dn14,)
    }
};
        locals.var_costi1 = assign15330_e10220;
        locals.var_costi1_dn0 = assign15330_e10220_d_n0;
        locals.var_costi1_dn2 = assign15330_e10220_d_n2;
        locals.var_costi1_dn4 = assign15330_e10220_d_n4;
        locals.var_costi1_dn5 = assign15330_e10220_d_n5;
        locals.var_costi1_dn6 = assign15330_e10220_d_n6;
        locals.var_costi1_dn7 = assign15330_e10220_d_n7;
        locals.var_costi1_dn8 = assign15330_e10220_d_n8;
        locals.var_costi1_dn9 = assign15330_e10220_d_n9;
        locals.var_costi1_dn10 = assign15330_e10220_d_n10;
        locals.var_costi1_dn11 = assign15330_e10220_d_n11;
        locals.var_costi1_dn14 = assign15330_e10220_d_n14;

        let (assign15340_e10228, assign15340_e10228_d_n0, assign15340_e10228_d_n2, assign15340_e10228_d_n4, assign15340_e10228_d_n5, assign15340_e10228_d_n6, assign15340_e10228_d_n7, assign15340_e10228_d_n8, assign15340_e10228_d_n9, assign15340_e10228_d_n10, assign15340_e10228_d_n11, assign15340_e10228_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15340_e10225: f64 = (p.p448 * locals.var_tdiff);
        let assign15340_e10226: f64 = (p.p447 + assign15340_e10225);
        (assign15340_e10226, (p.p448 * locals.var_tdiff_dn0), (p.p448 * locals.var_tdiff_dn2), (p.p448 * locals.var_tdiff_dn4), (p.p448 * locals.var_tdiff_dn5), (p.p448 * locals.var_tdiff_dn6), (p.p448 * locals.var_tdiff_dn7), (p.p448 * locals.var_tdiff_dn8), (p.p448 * locals.var_tdiff_dn9), (p.p448 * locals.var_tdiff_dn10), (p.p448 * locals.var_tdiff_dn11), (p.p448 * locals.var_tdiff_dn14),)
    } else {
        (locals.var_hbdceff, locals.var_hbdceff_dn0, locals.var_hbdceff_dn2, locals.var_hbdceff_dn4, locals.var_hbdceff_dn5, locals.var_hbdceff_dn6, locals.var_hbdceff_dn7, locals.var_hbdceff_dn8, locals.var_hbdceff_dn9, locals.var_hbdceff_dn10, locals.var_hbdceff_dn11, locals.var_hbdceff_dn14,)
    }
};
        locals.var_hbdceff = assign15340_e10228;
        locals.var_hbdceff_dn0 = assign15340_e10228_d_n0;
        locals.var_hbdceff_dn2 = assign15340_e10228_d_n2;
        locals.var_hbdceff_dn4 = assign15340_e10228_d_n4;
        locals.var_hbdceff_dn5 = assign15340_e10228_d_n5;
        locals.var_hbdceff_dn6 = assign15340_e10228_d_n6;
        locals.var_hbdceff_dn7 = assign15340_e10228_d_n7;
        locals.var_hbdceff_dn8 = assign15340_e10228_d_n8;
        locals.var_hbdceff_dn9 = assign15340_e10228_d_n9;
        locals.var_hbdceff_dn10 = assign15340_e10228_d_n10;
        locals.var_hbdceff_dn11 = assign15340_e10228_d_n11;
        locals.var_hbdceff_dn14 = assign15340_e10228_d_n14;

        let (assign15350_e10232,) = {
    if (locals.var_guard291 != 0.0) {
        (p.p193,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign15350_e10232;

        let assign15380_e10245: f64 = if locals.var_uc_subtmp < 0.0 { 1.0 } else { 0.0 };
        locals.var_guard326 = assign15380_e10245;

        let (assign15390_e10251,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard326 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign15390_e10251;

        let assign15400_e10254: f64 = if locals.var_uc_subtmp > 0.005 { 1.0 } else { 0.0 };
        locals.var_guard327 = assign15400_e10254;

        let (assign15410_e10260,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard327 != 0.0)) {
        (0.005,)
    } else {
        (locals.var_uc_subtmp,)
    }
};
        locals.var_uc_subtmp = assign15410_e10260;

        let assign15420_e10263: f64 = if locals.var_uc_cordrift > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard328 = assign15420_e10263;

        let (assign15430_e10276, assign15430_e10276_d_n0, assign15430_e10276_d_n2, assign15430_e10276_d_n4, assign15430_e10276_d_n5, assign15430_e10276_d_n6, assign15430_e10276_d_n7, assign15430_e10276_d_n8, assign15430_e10276_d_n9, assign15430_e10276_d_n10, assign15430_e10276_d_n11, assign15430_e10276_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) {
        let (assign15430_e10274, assign15430_e10274_d_n0, assign15430_e10274_d_n2, assign15430_e10274_d_n4, assign15430_e10274_d_n5, assign15430_e10274_d_n6, assign15430_e10274_d_n7, assign15430_e10274_d_n8, assign15430_e10274_d_n9, assign15430_e10274_d_n10, assign15430_e10274_d_n11, assign15430_e10274_d_n14,) = {
            if (locals.var_tratio == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign15430_e10273: f64 = (locals.var_tratio).powf(p.p416);
                (assign15430_e10273, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn0)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn0 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn2)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn2 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn4)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn4 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn5)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn5 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn6)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn6 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn7)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn7 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn8)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn8 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn9)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn9 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn10)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn10 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn11)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn11 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p416) as f64).is_finite() && ((p.p416) as f64).fract() == 0.0 { if p.p416 == 0.0 { 0.0 } else { (p.p416 * ((locals.var_tratio).powf(p.p416 - 1.0) * locals.var_tratio_dn14)) } } else { (assign15430_e10273 * (p.p416 * (locals.var_tratio_dn14 / locals.var_tratio))) },)
            }
        };
        (assign15430_e10274, assign15430_e10274_d_n0, assign15430_e10274_d_n2, assign15430_e10274_d_n4, assign15430_e10274_d_n5, assign15430_e10274_d_n6, assign15430_e10274_d_n7, assign15430_e10274_d_n8, assign15430_e10274_d_n9, assign15430_e10274_d_n10, assign15430_e10274_d_n11, assign15430_e10274_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15430_e10276;
        locals.var_t1_dn0 = assign15430_e10276_d_n0;
        locals.var_t1_dn2 = assign15430_e10276_d_n2;
        locals.var_t1_dn4 = assign15430_e10276_d_n4;
        locals.var_t1_dn5 = assign15430_e10276_d_n5;
        locals.var_t1_dn6 = assign15430_e10276_d_n6;
        locals.var_t1_dn7 = assign15430_e10276_d_n7;
        locals.var_t1_dn8 = assign15430_e10276_d_n8;
        locals.var_t1_dn9 = assign15430_e10276_d_n9;
        locals.var_t1_dn10 = assign15430_e10276_d_n10;
        locals.var_t1_dn11 = assign15430_e10276_d_n11;
        locals.var_t1_dn14 = assign15430_e10276_d_n14;

        let (assign15440_e10284, assign15440_e10284_d_n0, assign15440_e10284_d_n2, assign15440_e10284_d_n4, assign15440_e10284_d_n5, assign15440_e10284_d_n6, assign15440_e10284_d_n7, assign15440_e10284_d_n8, assign15440_e10284_d_n9, assign15440_e10284_d_n10, assign15440_e10284_d_n11, assign15440_e10284_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign15440_e10282: f64 = (locals.var_mks_rdrmues / locals.var_t1);
        (assign15440_e10282, (-((locals.var_mks_rdrmues * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmues * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_rrdrmues, locals.var_rrdrmues_dn0, locals.var_rrdrmues_dn2, locals.var_rrdrmues_dn4, locals.var_rrdrmues_dn5, locals.var_rrdrmues_dn6, locals.var_rrdrmues_dn7, locals.var_rrdrmues_dn8, locals.var_rrdrmues_dn9, locals.var_rrdrmues_dn10, locals.var_rrdrmues_dn11, locals.var_rrdrmues_dn14,)
    }
};
        locals.var_rrdrmues = assign15440_e10284;
        locals.var_rrdrmues_dn0 = assign15440_e10284_d_n0;
        locals.var_rrdrmues_dn2 = assign15440_e10284_d_n2;
        locals.var_rrdrmues_dn4 = assign15440_e10284_d_n4;
        locals.var_rrdrmues_dn5 = assign15440_e10284_d_n5;
        locals.var_rrdrmues_dn6 = assign15440_e10284_d_n6;
        locals.var_rrdrmues_dn7 = assign15440_e10284_d_n7;
        locals.var_rrdrmues_dn8 = assign15440_e10284_d_n8;
        locals.var_rrdrmues_dn9 = assign15440_e10284_d_n9;
        locals.var_rrdrmues_dn10 = assign15440_e10284_d_n10;
        locals.var_rrdrmues_dn11 = assign15440_e10284_d_n11;
        locals.var_rrdrmues_dn14 = assign15440_e10284_d_n14;

        let (assign15450_e10306, assign15450_e10306_d_n0, assign15450_e10306_d_n2, assign15450_e10306_d_n4, assign15450_e10306_d_n5, assign15450_e10306_d_n6, assign15450_e10306_d_n7, assign15450_e10306_d_n8, assign15450_e10306_d_n9, assign15450_e10306_d_n10, assign15450_e10306_d_n11, assign15450_e10306_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign15450_e10291: f64 = (0.4 * locals.var_tratio);
        let assign15450_e10292: f64 = (1.8 + assign15450_e10291);
        let assign15450_e10295: f64 = (0.1 * locals.var_tratio);
        let assign15450_e10297: f64 = (assign15450_e10295 * locals.var_tratio);
        let assign15450_e10298: f64 = (assign15450_e10292 + assign15450_e10297);
        let assign15450_e10302: f64 = (1.0 - locals.var_tratio);
        let assign15450_e10303: f64 = (p.p418 * assign15450_e10302);
        let assign15450_e10304: f64 = (assign15450_e10298 - assign15450_e10303);
        (assign15450_e10304, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn0))) - (p.p418 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn2))) - (p.p418 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn4))) - (p.p418 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn5))) - (p.p418 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn6))) - (p.p418 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn7))) - (p.p418 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn8))) - (p.p418 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn9))) - (p.p418 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn10))) - (p.p418 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn11) + (((0.1 * locals.var_tratio_dn11) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn11))) - (p.p418 * (-locals.var_tratio_dn11))), (((0.4 * locals.var_tratio_dn14) + (((0.1 * locals.var_tratio_dn14) * locals.var_tratio) + (assign15450_e10295 * locals.var_tratio_dn14))) - (p.p418 * (-locals.var_tratio_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign15450_e10306;
        locals.var_t0_dn0 = assign15450_e10306_d_n0;
        locals.var_t0_dn2 = assign15450_e10306_d_n2;
        locals.var_t0_dn4 = assign15450_e10306_d_n4;
        locals.var_t0_dn5 = assign15450_e10306_d_n5;
        locals.var_t0_dn6 = assign15450_e10306_d_n6;
        locals.var_t0_dn7 = assign15450_e10306_d_n7;
        locals.var_t0_dn8 = assign15450_e10306_d_n8;
        locals.var_t0_dn9 = assign15450_e10306_d_n9;
        locals.var_t0_dn10 = assign15450_e10306_d_n10;
        locals.var_t0_dn11 = assign15450_e10306_d_n11;
        locals.var_t0_dn14 = assign15450_e10306_d_n14;

        let (assign15460_e10314, assign15460_e10314_d_n0, assign15460_e10314_d_n2, assign15460_e10314_d_n4, assign15460_e10314_d_n5, assign15460_e10314_d_n6, assign15460_e10314_d_n7, assign15460_e10314_d_n8, assign15460_e10314_d_n9, assign15460_e10314_d_n10, assign15460_e10314_d_n11, assign15460_e10314_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign15460_e10312: f64 = (locals.var_mks_rdrvmaxs / locals.var_t0);
        (assign15460_e10312, (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmaxs * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_rrdrvmaxs, locals.var_rrdrvmaxs_dn0, locals.var_rrdrvmaxs_dn2, locals.var_rrdrvmaxs_dn4, locals.var_rrdrvmaxs_dn5, locals.var_rrdrvmaxs_dn6, locals.var_rrdrvmaxs_dn7, locals.var_rrdrvmaxs_dn8, locals.var_rrdrvmaxs_dn9, locals.var_rrdrvmaxs_dn10, locals.var_rrdrvmaxs_dn11, locals.var_rrdrvmaxs_dn14,)
    }
};
        locals.var_rrdrvmaxs = assign15460_e10314;
        locals.var_rrdrvmaxs_dn0 = assign15460_e10314_d_n0;
        locals.var_rrdrvmaxs_dn2 = assign15460_e10314_d_n2;
        locals.var_rrdrvmaxs_dn4 = assign15460_e10314_d_n4;
        locals.var_rrdrvmaxs_dn5 = assign15460_e10314_d_n5;
        locals.var_rrdrvmaxs_dn6 = assign15460_e10314_d_n6;
        locals.var_rrdrvmaxs_dn7 = assign15460_e10314_d_n7;
        locals.var_rrdrvmaxs_dn8 = assign15460_e10314_d_n8;
        locals.var_rrdrvmaxs_dn9 = assign15460_e10314_d_n9;
        locals.var_rrdrvmaxs_dn10 = assign15460_e10314_d_n10;
        locals.var_rrdrvmaxs_dn11 = assign15460_e10314_d_n11;
        locals.var_rrdrvmaxs_dn14 = assign15460_e10314_d_n14;

        let (assign15470_e10326, assign15470_e10326_d_n0, assign15470_e10326_d_n2, assign15470_e10326_d_n4, assign15470_e10326_d_n5, assign15470_e10326_d_n6, assign15470_e10326_d_n7, assign15470_e10326_d_n8, assign15470_e10326_d_n9, assign15470_e10326_d_n10, assign15470_e10326_d_n11, assign15470_e10326_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign15470_e10322: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign15470_e10323: f64 = (p.p439 * assign15470_e10322);
        let assign15470_e10324: f64 = (locals.var_uc_rdrbb_s + assign15470_e10323);
        (assign15470_e10324, (locals.var_uc_rdrbb_s_dn0 + (p.p439 * locals.var_ttemp_dn0)), (locals.var_uc_rdrbb_s_dn2 + (p.p439 * locals.var_ttemp_dn2)), (locals.var_uc_rdrbb_s_dn4 + (p.p439 * locals.var_ttemp_dn4)), (locals.var_uc_rdrbb_s_dn5 + (p.p439 * locals.var_ttemp_dn5)), (locals.var_uc_rdrbb_s_dn6 + (p.p439 * locals.var_ttemp_dn6)), (locals.var_uc_rdrbb_s_dn7 + (p.p439 * locals.var_ttemp_dn7)), (locals.var_uc_rdrbb_s_dn8 + (p.p439 * locals.var_ttemp_dn8)), (locals.var_uc_rdrbb_s_dn9 + (p.p439 * locals.var_ttemp_dn9)), (locals.var_uc_rdrbb_s_dn10 + (p.p439 * locals.var_ttemp_dn10)), (locals.var_uc_rdrbb_s_dn11 + (p.p439 * locals.var_ttemp_dn11)), (locals.var_uc_rdrbb_s_dn14 + (p.p439 * locals.var_ttemp_dn14)),)
    } else {
        (locals.var_uc_rdrbb_s, locals.var_uc_rdrbb_s_dn0, locals.var_uc_rdrbb_s_dn2, locals.var_uc_rdrbb_s_dn4, locals.var_uc_rdrbb_s_dn5, locals.var_uc_rdrbb_s_dn6, locals.var_uc_rdrbb_s_dn7, locals.var_uc_rdrbb_s_dn8, locals.var_uc_rdrbb_s_dn9, locals.var_uc_rdrbb_s_dn10, locals.var_uc_rdrbb_s_dn11, locals.var_uc_rdrbb_s_dn14,)
    }
};
        locals.var_uc_rdrbb_s = assign15470_e10326;
        locals.var_uc_rdrbb_s_dn0 = assign15470_e10326_d_n0;
        locals.var_uc_rdrbb_s_dn2 = assign15470_e10326_d_n2;
        locals.var_uc_rdrbb_s_dn4 = assign15470_e10326_d_n4;
        locals.var_uc_rdrbb_s_dn5 = assign15470_e10326_d_n5;
        locals.var_uc_rdrbb_s_dn6 = assign15470_e10326_d_n6;
        locals.var_uc_rdrbb_s_dn7 = assign15470_e10326_d_n7;
        locals.var_uc_rdrbb_s_dn8 = assign15470_e10326_d_n8;
        locals.var_uc_rdrbb_s_dn9 = assign15470_e10326_d_n9;
        locals.var_uc_rdrbb_s_dn10 = assign15470_e10326_d_n10;
        locals.var_uc_rdrbb_s_dn11 = assign15470_e10326_d_n11;
        locals.var_uc_rdrbb_s_dn14 = assign15470_e10326_d_n14;

        let (assign15480_e10339, assign15480_e10339_d_n0, assign15480_e10339_d_n2, assign15480_e10339_d_n4, assign15480_e10339_d_n5, assign15480_e10339_d_n6, assign15480_e10339_d_n7, assign15480_e10339_d_n8, assign15480_e10339_d_n9, assign15480_e10339_d_n10, assign15480_e10339_d_n11, assign15480_e10339_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) {
        let (assign15480_e10337, assign15480_e10337_d_n0, assign15480_e10337_d_n2, assign15480_e10337_d_n4, assign15480_e10337_d_n5, assign15480_e10337_d_n6, assign15480_e10337_d_n7, assign15480_e10337_d_n8, assign15480_e10337_d_n9, assign15480_e10337_d_n10, assign15480_e10337_d_n11, assign15480_e10337_d_n14,) = {
            if (locals.var_tratio == 0.0) {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign15480_e10336: f64 = (locals.var_tratio).powf(p.p415);
                (assign15480_e10336, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn0)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn0 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn2)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn2 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn4)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn4 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn5)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn5 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn6)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn6 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn7)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn7 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn8)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn8 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn9)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn9 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn10)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn10 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn11)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn11 / locals.var_tratio))) }, if 0.0 == 0.0 && ((p.p415) as f64).is_finite() && ((p.p415) as f64).fract() == 0.0 { if p.p415 == 0.0 { 0.0 } else { (p.p415 * ((locals.var_tratio).powf(p.p415 - 1.0) * locals.var_tratio_dn14)) } } else { (assign15480_e10336 * (p.p415 * (locals.var_tratio_dn14 / locals.var_tratio))) },)
            }
        };
        (assign15480_e10337, assign15480_e10337_d_n0, assign15480_e10337_d_n2, assign15480_e10337_d_n4, assign15480_e10337_d_n5, assign15480_e10337_d_n6, assign15480_e10337_d_n7, assign15480_e10337_d_n8, assign15480_e10337_d_n9, assign15480_e10337_d_n10, assign15480_e10337_d_n11, assign15480_e10337_d_n14,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15480_e10339;
        locals.var_t1_dn0 = assign15480_e10339_d_n0;
        locals.var_t1_dn2 = assign15480_e10339_d_n2;
        locals.var_t1_dn4 = assign15480_e10339_d_n4;
        locals.var_t1_dn5 = assign15480_e10339_d_n5;
        locals.var_t1_dn6 = assign15480_e10339_d_n6;
        locals.var_t1_dn7 = assign15480_e10339_d_n7;
        locals.var_t1_dn8 = assign15480_e10339_d_n8;
        locals.var_t1_dn9 = assign15480_e10339_d_n9;
        locals.var_t1_dn10 = assign15480_e10339_d_n10;
        locals.var_t1_dn11 = assign15480_e10339_d_n11;
        locals.var_t1_dn14 = assign15480_e10339_d_n14;

        let (assign15490_e10347, assign15490_e10347_d_n0, assign15490_e10347_d_n2, assign15490_e10347_d_n4, assign15490_e10347_d_n5, assign15490_e10347_d_n6, assign15490_e10347_d_n7, assign15490_e10347_d_n8, assign15490_e10347_d_n9, assign15490_e10347_d_n10, assign15490_e10347_d_n11, assign15490_e10347_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign15490_e10345: f64 = (locals.var_mks_rdrmue / locals.var_t1);
        (assign15490_e10345, (-((locals.var_mks_rdrmue * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))), (-((locals.var_mks_rdrmue * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))),)
    } else {
        (locals.var_rrdrmue, locals.var_rrdrmue_dn0, locals.var_rrdrmue_dn2, locals.var_rrdrmue_dn4, locals.var_rrdrmue_dn5, locals.var_rrdrmue_dn6, locals.var_rrdrmue_dn7, locals.var_rrdrmue_dn8, locals.var_rrdrmue_dn9, locals.var_rrdrmue_dn10, locals.var_rrdrmue_dn11, locals.var_rrdrmue_dn14,)
    }
};
        locals.var_rrdrmue = assign15490_e10347;
        locals.var_rrdrmue_dn0 = assign15490_e10347_d_n0;
        locals.var_rrdrmue_dn2 = assign15490_e10347_d_n2;
        locals.var_rrdrmue_dn4 = assign15490_e10347_d_n4;
        locals.var_rrdrmue_dn5 = assign15490_e10347_d_n5;
        locals.var_rrdrmue_dn6 = assign15490_e10347_d_n6;
        locals.var_rrdrmue_dn7 = assign15490_e10347_d_n7;
        locals.var_rrdrmue_dn8 = assign15490_e10347_d_n8;
        locals.var_rrdrmue_dn9 = assign15490_e10347_d_n9;
        locals.var_rrdrmue_dn10 = assign15490_e10347_d_n10;
        locals.var_rrdrmue_dn11 = assign15490_e10347_d_n11;
        locals.var_rrdrmue_dn14 = assign15490_e10347_d_n14;

        let (assign15500_e10369, assign15500_e10369_d_n0, assign15500_e10369_d_n2, assign15500_e10369_d_n4, assign15500_e10369_d_n5, assign15500_e10369_d_n6, assign15500_e10369_d_n7, assign15500_e10369_d_n8, assign15500_e10369_d_n9, assign15500_e10369_d_n10, assign15500_e10369_d_n11, assign15500_e10369_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign15500_e10354: f64 = (0.4 * locals.var_tratio);
        let assign15500_e10355: f64 = (1.8 + assign15500_e10354);
        let assign15500_e10358: f64 = (0.1 * locals.var_tratio);
        let assign15500_e10360: f64 = (assign15500_e10358 * locals.var_tratio);
        let assign15500_e10361: f64 = (assign15500_e10355 + assign15500_e10360);
        let assign15500_e10365: f64 = (1.0 - locals.var_tratio);
        let assign15500_e10366: f64 = (p.p417 * assign15500_e10365);
        let assign15500_e10367: f64 = (assign15500_e10361 - assign15500_e10366);
        (assign15500_e10367, (((0.4 * locals.var_tratio_dn0) + (((0.1 * locals.var_tratio_dn0) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn0))) - (p.p417 * (-locals.var_tratio_dn0))), (((0.4 * locals.var_tratio_dn2) + (((0.1 * locals.var_tratio_dn2) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn2))) - (p.p417 * (-locals.var_tratio_dn2))), (((0.4 * locals.var_tratio_dn4) + (((0.1 * locals.var_tratio_dn4) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn4))) - (p.p417 * (-locals.var_tratio_dn4))), (((0.4 * locals.var_tratio_dn5) + (((0.1 * locals.var_tratio_dn5) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn5))) - (p.p417 * (-locals.var_tratio_dn5))), (((0.4 * locals.var_tratio_dn6) + (((0.1 * locals.var_tratio_dn6) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn6))) - (p.p417 * (-locals.var_tratio_dn6))), (((0.4 * locals.var_tratio_dn7) + (((0.1 * locals.var_tratio_dn7) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn7))) - (p.p417 * (-locals.var_tratio_dn7))), (((0.4 * locals.var_tratio_dn8) + (((0.1 * locals.var_tratio_dn8) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn8))) - (p.p417 * (-locals.var_tratio_dn8))), (((0.4 * locals.var_tratio_dn9) + (((0.1 * locals.var_tratio_dn9) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn9))) - (p.p417 * (-locals.var_tratio_dn9))), (((0.4 * locals.var_tratio_dn10) + (((0.1 * locals.var_tratio_dn10) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn10))) - (p.p417 * (-locals.var_tratio_dn10))), (((0.4 * locals.var_tratio_dn11) + (((0.1 * locals.var_tratio_dn11) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn11))) - (p.p417 * (-locals.var_tratio_dn11))), (((0.4 * locals.var_tratio_dn14) + (((0.1 * locals.var_tratio_dn14) * locals.var_tratio) + (assign15500_e10358 * locals.var_tratio_dn14))) - (p.p417 * (-locals.var_tratio_dn14))),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign15500_e10369;
        locals.var_t0_dn0 = assign15500_e10369_d_n0;
        locals.var_t0_dn2 = assign15500_e10369_d_n2;
        locals.var_t0_dn4 = assign15500_e10369_d_n4;
        locals.var_t0_dn5 = assign15500_e10369_d_n5;
        locals.var_t0_dn6 = assign15500_e10369_d_n6;
        locals.var_t0_dn7 = assign15500_e10369_d_n7;
        locals.var_t0_dn8 = assign15500_e10369_d_n8;
        locals.var_t0_dn9 = assign15500_e10369_d_n9;
        locals.var_t0_dn10 = assign15500_e10369_d_n10;
        locals.var_t0_dn11 = assign15500_e10369_d_n11;
        locals.var_t0_dn14 = assign15500_e10369_d_n14;

        let (assign15510_e10377, assign15510_e10377_d_n0, assign15510_e10377_d_n2, assign15510_e10377_d_n4, assign15510_e10377_d_n5, assign15510_e10377_d_n6, assign15510_e10377_d_n7, assign15510_e10377_d_n8, assign15510_e10377_d_n9, assign15510_e10377_d_n10, assign15510_e10377_d_n11, assign15510_e10377_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign15510_e10375: f64 = (locals.var_mks_rdrvmax / locals.var_t0);
        (assign15510_e10375, (-((locals.var_mks_rdrvmax * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0))), (-((locals.var_mks_rdrvmax * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0))),)
    } else {
        (locals.var_rrdrvmax, locals.var_rrdrvmax_dn0, locals.var_rrdrvmax_dn2, locals.var_rrdrvmax_dn4, locals.var_rrdrvmax_dn5, locals.var_rrdrvmax_dn6, locals.var_rrdrvmax_dn7, locals.var_rrdrvmax_dn8, locals.var_rrdrvmax_dn9, locals.var_rrdrvmax_dn10, locals.var_rrdrvmax_dn11, locals.var_rrdrvmax_dn14,)
    }
};
        locals.var_rrdrvmax = assign15510_e10377;
        locals.var_rrdrvmax_dn0 = assign15510_e10377_d_n0;
        locals.var_rrdrvmax_dn2 = assign15510_e10377_d_n2;
        locals.var_rrdrvmax_dn4 = assign15510_e10377_d_n4;
        locals.var_rrdrvmax_dn5 = assign15510_e10377_d_n5;
        locals.var_rrdrvmax_dn6 = assign15510_e10377_d_n6;
        locals.var_rrdrvmax_dn7 = assign15510_e10377_d_n7;
        locals.var_rrdrvmax_dn8 = assign15510_e10377_d_n8;
        locals.var_rrdrvmax_dn9 = assign15510_e10377_d_n9;
        locals.var_rrdrvmax_dn10 = assign15510_e10377_d_n10;
        locals.var_rrdrvmax_dn11 = assign15510_e10377_d_n11;
        locals.var_rrdrvmax_dn14 = assign15510_e10377_d_n14;

        let (assign15520_e10389, assign15520_e10389_d_n0, assign15520_e10389_d_n2, assign15520_e10389_d_n4, assign15520_e10389_d_n5, assign15520_e10389_d_n6, assign15520_e10389_d_n7, assign15520_e10389_d_n8, assign15520_e10389_d_n9, assign15520_e10389_d_n10, assign15520_e10389_d_n11, assign15520_e10389_d_n14,) = {
    if ((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) {
        let assign15520_e10385: f64 = (locals.var_ttemp - locals.var_ktnom);
        let assign15520_e10386: f64 = (p.p438 * assign15520_e10385);
        let assign15520_e10387: f64 = (locals.var_uc_rdrbb + assign15520_e10386);
        (assign15520_e10387, (locals.var_uc_rdrbb_dn0 + (p.p438 * locals.var_ttemp_dn0)), (locals.var_uc_rdrbb_dn2 + (p.p438 * locals.var_ttemp_dn2)), (locals.var_uc_rdrbb_dn4 + (p.p438 * locals.var_ttemp_dn4)), (locals.var_uc_rdrbb_dn5 + (p.p438 * locals.var_ttemp_dn5)), (locals.var_uc_rdrbb_dn6 + (p.p438 * locals.var_ttemp_dn6)), (locals.var_uc_rdrbb_dn7 + (p.p438 * locals.var_ttemp_dn7)), (locals.var_uc_rdrbb_dn8 + (p.p438 * locals.var_ttemp_dn8)), (locals.var_uc_rdrbb_dn9 + (p.p438 * locals.var_ttemp_dn9)), (locals.var_uc_rdrbb_dn10 + (p.p438 * locals.var_ttemp_dn10)), (locals.var_uc_rdrbb_dn11 + (p.p438 * locals.var_ttemp_dn11)), (locals.var_uc_rdrbb_dn14 + (p.p438 * locals.var_ttemp_dn14)),)
    } else {
        (locals.var_uc_rdrbb, locals.var_uc_rdrbb_dn0, locals.var_uc_rdrbb_dn2, locals.var_uc_rdrbb_dn4, locals.var_uc_rdrbb_dn5, locals.var_uc_rdrbb_dn6, locals.var_uc_rdrbb_dn7, locals.var_uc_rdrbb_dn8, locals.var_uc_rdrbb_dn9, locals.var_uc_rdrbb_dn10, locals.var_uc_rdrbb_dn11, locals.var_uc_rdrbb_dn14,)
    }
};
        locals.var_uc_rdrbb = assign15520_e10389;
        locals.var_uc_rdrbb_dn0 = assign15520_e10389_d_n0;
        locals.var_uc_rdrbb_dn2 = assign15520_e10389_d_n2;
        locals.var_uc_rdrbb_dn4 = assign15520_e10389_d_n4;
        locals.var_uc_rdrbb_dn5 = assign15520_e10389_d_n5;
        locals.var_uc_rdrbb_dn6 = assign15520_e10389_d_n6;
        locals.var_uc_rdrbb_dn7 = assign15520_e10389_d_n7;
        locals.var_uc_rdrbb_dn8 = assign15520_e10389_d_n8;
        locals.var_uc_rdrbb_dn9 = assign15520_e10389_d_n9;
        locals.var_uc_rdrbb_dn10 = assign15520_e10389_d_n10;
        locals.var_uc_rdrbb_dn11 = assign15520_e10389_d_n11;
        locals.var_uc_rdrbb_dn14 = assign15520_e10389_d_n14;

        let assign15540_e10397: f64 = if locals.var_uc_rdrbb < 0.1 { 1.0 } else { 0.0 };
        locals.var_guard330 = assign15540_e10397;

        let (assign15550_e10405, assign15550_e10405_d_n0, assign15550_e10405_d_n2, assign15550_e10405_d_n4, assign15550_e10405_d_n5, assign15550_e10405_d_n6, assign15550_e10405_d_n7, assign15550_e10405_d_n8, assign15550_e10405_d_n9, assign15550_e10405_d_n10, assign15550_e10405_d_n11, assign15550_e10405_d_n14,) = {
    if (((locals.var_guard291 != 0.0) && (locals.var_guard328 != 0.0)) && (locals.var_guard330 != 0.0)) {
        (0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uc_rdrbb, locals.var_uc_rdrbb_dn0, locals.var_uc_rdrbb_dn2, locals.var_uc_rdrbb_dn4, locals.var_uc_rdrbb_dn5, locals.var_uc_rdrbb_dn6, locals.var_uc_rdrbb_dn7, locals.var_uc_rdrbb_dn8, locals.var_uc_rdrbb_dn9, locals.var_uc_rdrbb_dn10, locals.var_uc_rdrbb_dn11, locals.var_uc_rdrbb_dn14,)
    }
};
        locals.var_uc_rdrbb = assign15550_e10405;
        locals.var_uc_rdrbb_dn0 = assign15550_e10405_d_n0;
        locals.var_uc_rdrbb_dn2 = assign15550_e10405_d_n2;
        locals.var_uc_rdrbb_dn4 = assign15550_e10405_d_n4;
        locals.var_uc_rdrbb_dn5 = assign15550_e10405_d_n5;
        locals.var_uc_rdrbb_dn6 = assign15550_e10405_d_n6;
        locals.var_uc_rdrbb_dn7 = assign15550_e10405_d_n7;
        locals.var_uc_rdrbb_dn8 = assign15550_e10405_d_n8;
        locals.var_uc_rdrbb_dn9 = assign15550_e10405_d_n9;
        locals.var_uc_rdrbb_dn10 = assign15550_e10405_d_n10;
        locals.var_uc_rdrbb_dn11 = assign15550_e10405_d_n11;
        locals.var_uc_rdrbb_dn14 = assign15550_e10405_d_n14;

        let (assign15560_e10411, assign15560_e10411_d_n0, assign15560_e10411_d_n2, assign15560_e10411_d_n4, assign15560_e10411_d_n5, assign15560_e10411_d_n6, assign15560_e10411_d_n7, assign15560_e10411_d_n8, assign15560_e10411_d_n9, assign15560_e10411_d_n10, assign15560_e10411_d_n11, assign15560_e10411_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15560_e10409: f64 = (locals.var_tratio * locals.var_tratio);
        (assign15560_e10409, ((locals.var_tratio_dn0 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn0)), ((locals.var_tratio_dn2 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn2)), ((locals.var_tratio_dn4 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn4)), ((locals.var_tratio_dn5 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn5)), ((locals.var_tratio_dn6 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn6)), ((locals.var_tratio_dn7 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn7)), ((locals.var_tratio_dn8 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn8)), ((locals.var_tratio_dn9 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn9)), ((locals.var_tratio_dn10 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn10)), ((locals.var_tratio_dn11 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn11)), ((locals.var_tratio_dn14 * locals.var_tratio) + (locals.var_tratio * locals.var_tratio_dn14)),)
    } else {
        (locals.var_t0, locals.var_t0_dn0, locals.var_t0_dn2, locals.var_t0_dn4, locals.var_t0_dn5, locals.var_t0_dn6, locals.var_t0_dn7, locals.var_t0_dn8, locals.var_t0_dn9, locals.var_t0_dn10, locals.var_t0_dn11, locals.var_t0_dn14,)
    }
};
        locals.var_t0 = assign15560_e10411;
        locals.var_t0_dn0 = assign15560_e10411_d_n0;
        locals.var_t0_dn2 = assign15560_e10411_d_n2;
        locals.var_t0_dn4 = assign15560_e10411_d_n4;
        locals.var_t0_dn5 = assign15560_e10411_d_n5;
        locals.var_t0_dn6 = assign15560_e10411_d_n6;
        locals.var_t0_dn7 = assign15560_e10411_d_n7;
        locals.var_t0_dn8 = assign15560_e10411_d_n8;
        locals.var_t0_dn9 = assign15560_e10411_d_n9;
        locals.var_t0_dn10 = assign15560_e10411_d_n10;
        locals.var_t0_dn11 = assign15560_e10411_d_n11;
        locals.var_t0_dn14 = assign15560_e10411_d_n14;

        let (assign15570_e10430, assign15570_e10430_d_n0, assign15570_e10430_d_n2, assign15570_e10430_d_n4, assign15570_e10430_d_n5, assign15570_e10430_d_n6, assign15570_e10430_d_n7, assign15570_e10430_d_n8, assign15570_e10430_d_n9, assign15570_e10430_d_n10, assign15570_e10430_d_n11, assign15570_e10430_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15570_e10416: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15570_e10419: f64 = (locals.var_eg * locals.var_beta);
        let assign15570_e10420: f64 = (assign15570_e10416 - assign15570_e10419);
        let assign15570_e10423: f64 = (p.p499 * locals.var_log_tratio);
        let assign15570_e10424: f64 = (assign15570_e10420 + assign15570_e10423);
        let assign15570_e10426: f64 = (assign15570_e10424 / locals.var_uc_njd);
        let assign15570_e10427: f64 = (assign15570_e10426).exp();
        let assign15570_e10428: f64 = (locals.var_uc_js0d * assign15570_e10427);
        (assign15570_e10428, (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p499 * locals.var_log_tratio_dn11)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15570_e10427 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p499 * locals.var_log_tratio_dn14)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js, locals.var_js_dn0, locals.var_js_dn2, locals.var_js_dn4, locals.var_js_dn5, locals.var_js_dn6, locals.var_js_dn7, locals.var_js_dn8, locals.var_js_dn9, locals.var_js_dn10, locals.var_js_dn11, locals.var_js_dn14,)
    }
};
        locals.var_js = assign15570_e10430;
        locals.var_js_dn0 = assign15570_e10430_d_n0;
        locals.var_js_dn2 = assign15570_e10430_d_n2;
        locals.var_js_dn4 = assign15570_e10430_d_n4;
        locals.var_js_dn5 = assign15570_e10430_d_n5;
        locals.var_js_dn6 = assign15570_e10430_d_n6;
        locals.var_js_dn7 = assign15570_e10430_d_n7;
        locals.var_js_dn8 = assign15570_e10430_d_n8;
        locals.var_js_dn9 = assign15570_e10430_d_n9;
        locals.var_js_dn10 = assign15570_e10430_d_n10;
        locals.var_js_dn11 = assign15570_e10430_d_n11;
        locals.var_js_dn14 = assign15570_e10430_d_n14;

        let (assign15580_e10449, assign15580_e10449_d_n0, assign15580_e10449_d_n2, assign15580_e10449_d_n4, assign15580_e10449_d_n5, assign15580_e10449_d_n6, assign15580_e10449_d_n7, assign15580_e10449_d_n8, assign15580_e10449_d_n9, assign15580_e10449_d_n10, assign15580_e10449_d_n11, assign15580_e10449_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15580_e10435: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15580_e10438: f64 = (locals.var_eg * locals.var_beta);
        let assign15580_e10439: f64 = (assign15580_e10435 - assign15580_e10438);
        let assign15580_e10442: f64 = (p.p499 * locals.var_log_tratio);
        let assign15580_e10443: f64 = (assign15580_e10439 + assign15580_e10442);
        let assign15580_e10445: f64 = (assign15580_e10443 / p.p497);
        let assign15580_e10446: f64 = (assign15580_e10445).exp();
        let assign15580_e10447: f64 = (locals.var_uc_js0swd * assign15580_e10446);
        (assign15580_e10447, (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p497))), (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p497))), (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p497))), (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p497))), (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p497))), (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p497))), (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p497))), (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p497))), (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p497))), (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p499 * locals.var_log_tratio_dn11)) / p.p497))), (locals.var_uc_js0swd * (assign15580_e10446 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p499 * locals.var_log_tratio_dn14)) / p.p497))),)
    } else {
        (locals.var_jssw, locals.var_jssw_dn0, locals.var_jssw_dn2, locals.var_jssw_dn4, locals.var_jssw_dn5, locals.var_jssw_dn6, locals.var_jssw_dn7, locals.var_jssw_dn8, locals.var_jssw_dn9, locals.var_jssw_dn10, locals.var_jssw_dn11, locals.var_jssw_dn14,)
    }
};
        locals.var_jssw = assign15580_e10449;
        locals.var_jssw_dn0 = assign15580_e10449_d_n0;
        locals.var_jssw_dn2 = assign15580_e10449_d_n2;
        locals.var_jssw_dn4 = assign15580_e10449_d_n4;
        locals.var_jssw_dn5 = assign15580_e10449_d_n5;
        locals.var_jssw_dn6 = assign15580_e10449_d_n6;
        locals.var_jssw_dn7 = assign15580_e10449_d_n7;
        locals.var_jssw_dn8 = assign15580_e10449_d_n8;
        locals.var_jssw_dn9 = assign15580_e10449_d_n9;
        locals.var_jssw_dn10 = assign15580_e10449_d_n10;
        locals.var_jssw_dn11 = assign15580_e10449_d_n11;
        locals.var_jssw_dn14 = assign15580_e10449_d_n14;

        let (assign15590_e10468, assign15590_e10468_d_n0, assign15590_e10468_d_n2, assign15590_e10468_d_n4, assign15590_e10468_d_n5, assign15590_e10468_d_n6, assign15590_e10468_d_n7, assign15590_e10468_d_n8, assign15590_e10468_d_n9, assign15590_e10468_d_n10, assign15590_e10468_d_n11, assign15590_e10468_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15590_e10454: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15590_e10457: f64 = (locals.var_eg * locals.var_beta);
        let assign15590_e10458: f64 = (assign15590_e10454 - assign15590_e10457);
        let assign15590_e10461: f64 = (p.p499 * locals.var_log_tratio);
        let assign15590_e10462: f64 = (assign15590_e10458 + assign15590_e10461);
        let assign15590_e10464: f64 = (assign15590_e10462 / p.p498);
        let assign15590_e10465: f64 = (assign15590_e10464).exp();
        let assign15590_e10466: f64 = (p.p495 * assign15590_e10465);
        (assign15590_e10466, (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p499 * locals.var_log_tratio_dn0)) / p.p498))), (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p499 * locals.var_log_tratio_dn2)) / p.p498))), (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p499 * locals.var_log_tratio_dn4)) / p.p498))), (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p499 * locals.var_log_tratio_dn5)) / p.p498))), (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p499 * locals.var_log_tratio_dn6)) / p.p498))), (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p499 * locals.var_log_tratio_dn7)) / p.p498))), (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p499 * locals.var_log_tratio_dn8)) / p.p498))), (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p499 * locals.var_log_tratio_dn9)) / p.p498))), (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p499 * locals.var_log_tratio_dn10)) / p.p498))), (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p499 * locals.var_log_tratio_dn11)) / p.p498))), (p.p495 * (assign15590_e10465 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p499 * locals.var_log_tratio_dn14)) / p.p498))),)
    } else {
        (locals.var_jsswg, locals.var_jsswg_dn0, locals.var_jsswg_dn2, locals.var_jsswg_dn4, locals.var_jsswg_dn5, locals.var_jsswg_dn6, locals.var_jsswg_dn7, locals.var_jsswg_dn8, locals.var_jsswg_dn9, locals.var_jsswg_dn10, locals.var_jsswg_dn11, locals.var_jsswg_dn14,)
    }
};
        locals.var_jsswg = assign15590_e10468;
        locals.var_jsswg_dn0 = assign15590_e10468_d_n0;
        locals.var_jsswg_dn2 = assign15590_e10468_d_n2;
        locals.var_jsswg_dn4 = assign15590_e10468_d_n4;
        locals.var_jsswg_dn5 = assign15590_e10468_d_n5;
        locals.var_jsswg_dn6 = assign15590_e10468_d_n6;
        locals.var_jsswg_dn7 = assign15590_e10468_d_n7;
        locals.var_jsswg_dn8 = assign15590_e10468_d_n8;
        locals.var_jsswg_dn9 = assign15590_e10468_d_n9;
        locals.var_jsswg_dn10 = assign15590_e10468_d_n10;
        locals.var_jsswg_dn11 = assign15590_e10468_d_n11;
        locals.var_jsswg_dn14 = assign15590_e10468_d_n14;

        let (assign15600_e10487, assign15600_e10487_d_n0, assign15600_e10487_d_n2, assign15600_e10487_d_n4, assign15600_e10487_d_n5, assign15600_e10487_d_n6, assign15600_e10487_d_n7, assign15600_e10487_d_n8, assign15600_e10487_d_n9, assign15600_e10487_d_n10, assign15600_e10487_d_n11, assign15600_e10487_d_n14,) = {
    if (locals.var_guard291 != 0.0) {
        let assign15600_e10473: f64 = (locals.var_egtnom * locals.var_betatnom);
        let assign15600_e10476: f64 = (locals.var_eg * locals.var_beta);
        let assign15600_e10477: f64 = (assign15600_e10473 - assign15600_e10476);
        let assign15600_e10480: f64 = (p.p509 * locals.var_log_tratio);
        let assign15600_e10481: f64 = (assign15600_e10477 + assign15600_e10480);
        let assign15600_e10483: f64 = (assign15600_e10481 / locals.var_uc_njd);
        let assign15600_e10484: f64 = (assign15600_e10483).exp();
        let assign15600_e10485: f64 = (locals.var_uc_js0d * assign15600_e10484);
        (assign15600_e10485, (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn0 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn0))) + (p.p509 * locals.var_log_tratio_dn0)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn2 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn2))) + (p.p509 * locals.var_log_tratio_dn2)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn4 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn4))) + (p.p509 * locals.var_log_tratio_dn4)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn5 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn5))) + (p.p509 * locals.var_log_tratio_dn5)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn6 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn6))) + (p.p509 * locals.var_log_tratio_dn6)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn7 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn7))) + (p.p509 * locals.var_log_tratio_dn7)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn8 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn8))) + (p.p509 * locals.var_log_tratio_dn8)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn9 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn9))) + (p.p509 * locals.var_log_tratio_dn9)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn10 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn10))) + (p.p509 * locals.var_log_tratio_dn10)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn11 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn11))) + (p.p509 * locals.var_log_tratio_dn11)) / locals.var_uc_njd))), (locals.var_uc_js0d * (assign15600_e10484 * (((-((locals.var_eg_dn14 * locals.var_beta) + (locals.var_eg * locals.var_beta_dn14))) + (p.p509 * locals.var_log_tratio_dn14)) / locals.var_uc_njd))),)
    } else {
        (locals.var_js2, locals.var_js2_dn0, locals.var_js2_dn2, locals.var_js2_dn4, locals.var_js2_dn5, locals.var_js2_dn6, locals.var_js2_dn7, locals.var_js2_dn8, locals.var_js2_dn9, locals.var_js2_dn10, locals.var_js2_dn11, locals.var_js2_dn14,)
    }
};
        locals.var_js2 = assign15600_e10487;
        locals.var_js2_dn0 = assign15600_e10487_d_n0;
        locals.var_js2_dn2 = assign15600_e10487_d_n2;
        locals.var_js2_dn4 = assign15600_e10487_d_n4;
        locals.var_js2_dn5 = assign15600_e10487_d_n5;
        locals.var_js2_dn6 = assign15600_e10487_d_n6;
        locals.var_js2_dn7 = assign15600_e10487_d_n7;
        locals.var_js2_dn8 = assign15600_e10487_d_n8;
        locals.var_js2_dn9 = assign15600_e10487_d_n9;
        locals.var_js2_dn10 = assign15600_e10487_d_n10;
        locals.var_js2_dn11 = assign15600_e10487_d_n11;
        locals.var_js2_dn14 = assign15600_e10487_d_n14;

    }
}

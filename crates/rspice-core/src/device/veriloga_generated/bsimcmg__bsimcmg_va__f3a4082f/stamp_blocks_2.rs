#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, StampLocals, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign12760_e16930, assign12760_e16930_d_n0, assign12760_e16930_d_n2, assign12760_e16930_d_n3, assign12760_e16930_d_n4, assign12760_e16930_d_n5, assign12760_e16930_d_n6, assign12760_e16930_d_n7, assign12760_e16930_d_n8, assign12760_e16930_d_n9, assign12760_e16930_d_n10, assign12760_e16930_d_n11, assign12760_e16930_d_n13, assign12760_e16930_d_n14,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard221 == 0.0)) {
        let assign12760_e16893: f64 = (1.0 / locals.var_cnon);
        let assign12760_e16900: f64 = (-37.0);
        let (assign12760_e16927, assign12760_e16927_d_n0, assign12760_e16927_d_n2, assign12760_e16927_d_n3, assign12760_e16927_d_n4, assign12760_e16927_d_n5, assign12760_e16927_d_n6, assign12760_e16927_d_n7, assign12760_e16927_d_n8, assign12760_e16927_d_n9, assign12760_e16927_d_n10, assign12760_e16927_d_n11, assign12760_e16927_d_n13, assign12760_e16927_d_n14,) = {
            if ((!(locals.var_tt1 > 37.0)) && (!(locals.var_tt1 < assign12760_e16900))) {
                let assign12760_e16906: f64 = (locals.var_tt1).exp();
                let assign12760_e16907: f64 = (1.0 + assign12760_e16906);
                let assign12760_e16908: f64 = (assign12760_e16907).ln();
                (assign12760_e16908, ((assign12760_e16906 * locals.var_tt1_dn0) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn2) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn3) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn4) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn5) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn6) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn7) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn8) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn9) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn10) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn11) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn13) / assign12760_e16907), ((assign12760_e16906 * locals.var_tt1_dn14) / assign12760_e16907),)
            } else {
                let assign12760_e16915: f64 = (-37.0);
                let (assign12760_e16926, assign12760_e16926_d_n0, assign12760_e16926_d_n2, assign12760_e16926_d_n3, assign12760_e16926_d_n4, assign12760_e16926_d_n5, assign12760_e16926_d_n6, assign12760_e16926_d_n7, assign12760_e16926_d_n8, assign12760_e16926_d_n9, assign12760_e16926_d_n10, assign12760_e16926_d_n11, assign12760_e16926_d_n13, assign12760_e16926_d_n14,) = {
                    if ((!(locals.var_tt1 > 37.0)) && (locals.var_tt1 < assign12760_e16915)) {
                        let assign12760_e16919: f64 = (locals.var_tt1).exp();
                        (assign12760_e16919, (assign12760_e16919 * locals.var_tt1_dn0), (assign12760_e16919 * locals.var_tt1_dn2), (assign12760_e16919 * locals.var_tt1_dn3), (assign12760_e16919 * locals.var_tt1_dn4), (assign12760_e16919 * locals.var_tt1_dn5), (assign12760_e16919 * locals.var_tt1_dn6), (assign12760_e16919 * locals.var_tt1_dn7), (assign12760_e16919 * locals.var_tt1_dn8), (assign12760_e16919 * locals.var_tt1_dn9), (assign12760_e16919 * locals.var_tt1_dn10), (assign12760_e16919 * locals.var_tt1_dn11), (assign12760_e16919 * locals.var_tt1_dn13), (assign12760_e16919 * locals.var_tt1_dn14),)
                    } else {
                        let (assign12760_e16925, assign12760_e16925_d_n0, assign12760_e16925_d_n2, assign12760_e16925_d_n3, assign12760_e16925_d_n4, assign12760_e16925_d_n5, assign12760_e16925_d_n6, assign12760_e16925_d_n7, assign12760_e16925_d_n8, assign12760_e16925_d_n9, assign12760_e16925_d_n10, assign12760_e16925_d_n11, assign12760_e16925_d_n13, assign12760_e16925_d_n14,) = {
                            if (locals.var_tt1 > 37.0) {
                                (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
                            } else {
                                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                            }
                        };
                        (assign12760_e16925, assign12760_e16925_d_n0, assign12760_e16925_d_n2, assign12760_e16925_d_n3, assign12760_e16925_d_n4, assign12760_e16925_d_n5, assign12760_e16925_d_n6, assign12760_e16925_d_n7, assign12760_e16925_d_n8, assign12760_e16925_d_n9, assign12760_e16925_d_n10, assign12760_e16925_d_n11, assign12760_e16925_d_n13, assign12760_e16925_d_n14,)
                    }
                };
                (assign12760_e16926, assign12760_e16926_d_n0, assign12760_e16926_d_n2, assign12760_e16926_d_n3, assign12760_e16926_d_n4, assign12760_e16926_d_n5, assign12760_e16926_d_n6, assign12760_e16926_d_n7, assign12760_e16926_d_n8, assign12760_e16926_d_n9, assign12760_e16926_d_n10, assign12760_e16926_d_n11, assign12760_e16926_d_n13, assign12760_e16926_d_n14,)
            }
        };
        let assign12760_e16928: f64 = (assign12760_e16893 * assign12760_e16927);
        (assign12760_e16928, (assign12760_e16893 * assign12760_e16927_d_n0), (assign12760_e16893 * assign12760_e16927_d_n2), (assign12760_e16893 * assign12760_e16927_d_n3), (assign12760_e16893 * assign12760_e16927_d_n4), (assign12760_e16893 * assign12760_e16927_d_n5), (assign12760_e16893 * assign12760_e16927_d_n6), (assign12760_e16893 * assign12760_e16927_d_n7), (assign12760_e16893 * assign12760_e16927_d_n8), (assign12760_e16893 * assign12760_e16927_d_n9), (assign12760_e16893 * assign12760_e16927_d_n10), (assign12760_e16893 * assign12760_e16927_d_n11), (assign12760_e16893 * assign12760_e16927_d_n13), (assign12760_e16893 * assign12760_e16927_d_n14),)
    } else {
        (locals.var_ccg1, locals.var_ccg1_dn0, locals.var_ccg1_dn2, locals.var_ccg1_dn3, locals.var_ccg1_dn4, locals.var_ccg1_dn5, locals.var_ccg1_dn6, locals.var_ccg1_dn7, locals.var_ccg1_dn8, locals.var_ccg1_dn9, locals.var_ccg1_dn10, locals.var_ccg1_dn11, locals.var_ccg1_dn13, locals.var_ccg1_dn14,)
    }
};
        locals.var_ccg1 = assign12760_e16930;
        locals.var_ccg1_dn0 = assign12760_e16930_d_n0;
        locals.var_ccg1_dn2 = assign12760_e16930_d_n2;
        locals.var_ccg1_dn3 = assign12760_e16930_d_n3;
        locals.var_ccg1_dn4 = assign12760_e16930_d_n4;
        locals.var_ccg1_dn5 = assign12760_e16930_d_n5;
        locals.var_ccg1_dn6 = assign12760_e16930_d_n6;
        locals.var_ccg1_dn7 = assign12760_e16930_d_n7;
        locals.var_ccg1_dn8 = assign12760_e16930_d_n8;
        locals.var_ccg1_dn9 = assign12760_e16930_d_n9;
        locals.var_ccg1_dn10 = assign12760_e16930_d_n10;
        locals.var_ccg1_dn11 = assign12760_e16930_d_n11;
        locals.var_ccg1_dn13 = assign12760_e16930_d_n13;
        locals.var_ccg1_dn14 = assign12760_e16930_d_n14;

        let (assign12770_e16946,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12770_e16936: f64 = (locals.var_wg + p.p90);
        let assign12770_e16937: f64 = (locals.var_trsd / assign12770_e16936);
        let assign12770_e16940: f64 = (locals.var_wg + p.p90);
        let assign12770_e16942: f64 = (assign12770_e16940 / locals.var_trsd);
        let assign12770_e16943: f64 = (assign12770_e16937).min(assign12770_e16942);
        let assign12770_e16944: f64 = (0.5 * assign12770_e16943);
        (assign12770_e16944,)
    } else {
        (locals.var_r1cf,)
    }
};
        locals.var_r1cf = assign12770_e16946;

        let (assign12780_e16952,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12780_e16950: f64 = (locals.var_hgdelta * locals.var_r1cf);
        (assign12780_e16950,)
    } else {
        (locals.var_rcf,)
    }
};
        locals.var_rcf = assign12780_e16952;

        let (assign12790_e16999,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12790_e16956: f64 = (locals.var_epssp * 2.0);
        let assign12790_e16958: f64 = (assign12790_e16956 / 3.141592653589793);
        let assign12790_e16962: f64 = (0.5 * 3.141592653589793);
        let assign12790_e16964: f64 = (assign12790_e16962 * locals.var_rcf);
        let assign12790_e16965: f64 = (p.p1087 + assign12790_e16964);
        let assign12790_e16967: f64 = (assign12790_e16965 / p.p1087);
        let (assign12790_e16996,) = {
            if (!(assign12790_e16967 > 1e-38)) {
                let assign12790_e16972: f64 = (-87.498233534);
                (assign12790_e16972,)
            } else {
                let assign12790_e16976: f64 = (0.5 * 3.141592653589793);
                let assign12790_e16978: f64 = (assign12790_e16976 * locals.var_rcf);
                let assign12790_e16979: f64 = (p.p1087 + assign12790_e16978);
                let assign12790_e16981: f64 = (assign12790_e16979 / p.p1087);
                let (assign12790_e16995,) = {
                    if (assign12790_e16981 > 1e-38) {
                        let assign12790_e16987: f64 = (0.5 * 3.141592653589793);
                        let assign12790_e16989: f64 = (assign12790_e16987 * locals.var_rcf);
                        let assign12790_e16990: f64 = (p.p1087 + assign12790_e16989);
                        let assign12790_e16992: f64 = (assign12790_e16990 / p.p1087);
                        let assign12790_e16993: f64 = (assign12790_e16992).ln();
                        (assign12790_e16993,)
                    } else {
                        (0.0,)
                    }
                };
                (assign12790_e16995,)
            }
        };
        let assign12790_e16997: f64 = (assign12790_e16958 * assign12790_e16996);
        (assign12790_e16997,)
    } else {
        (locals.var_ccg2,)
    }
};
        locals.var_ccg2 = assign12790_e16999;

        let (assign12800_e17007, assign12800_e17007_d_n0, assign12800_e17007_d_n2, assign12800_e17007_d_n3, assign12800_e17007_d_n4, assign12800_e17007_d_n5, assign12800_e17007_d_n6, assign12800_e17007_d_n7, assign12800_e17007_d_n8, assign12800_e17007_d_n9, assign12800_e17007_d_n10, assign12800_e17007_d_n11, assign12800_e17007_d_n13, assign12800_e17007_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12800_e17004: f64 = (locals.var_ccg1 + locals.var_ccg2);
        let assign12800_e17005: f64 = (p.p42 * assign12800_e17004);
        (assign12800_e17005, (p.p42 * locals.var_ccg1_dn0), (p.p42 * locals.var_ccg1_dn2), (p.p42 * locals.var_ccg1_dn3), (p.p42 * locals.var_ccg1_dn4), (p.p42 * locals.var_ccg1_dn5), (p.p42 * locals.var_ccg1_dn6), (p.p42 * locals.var_ccg1_dn7), (p.p42 * locals.var_ccg1_dn8), (p.p42 * locals.var_ccg1_dn9), (p.p42 * locals.var_ccg1_dn10), (p.p42 * locals.var_ccg1_dn11), (p.p42 * locals.var_ccg1_dn13), (p.p42 * locals.var_ccg1_dn14),)
    } else {
        (locals.var_ccg, locals.var_ccg_dn0, locals.var_ccg_dn2, locals.var_ccg_dn3, locals.var_ccg_dn4, locals.var_ccg_dn5, locals.var_ccg_dn6, locals.var_ccg_dn7, locals.var_ccg_dn8, locals.var_ccg_dn9, locals.var_ccg_dn10, locals.var_ccg_dn11, locals.var_ccg_dn13, locals.var_ccg_dn14,)
    }
};
        locals.var_ccg = assign12800_e17007;
        locals.var_ccg_dn0 = assign12800_e17007_d_n0;
        locals.var_ccg_dn2 = assign12800_e17007_d_n2;
        locals.var_ccg_dn3 = assign12800_e17007_d_n3;
        locals.var_ccg_dn4 = assign12800_e17007_d_n4;
        locals.var_ccg_dn5 = assign12800_e17007_d_n5;
        locals.var_ccg_dn6 = assign12800_e17007_d_n6;
        locals.var_ccg_dn7 = assign12800_e17007_d_n7;
        locals.var_ccg_dn8 = assign12800_e17007_d_n8;
        locals.var_ccg_dn9 = assign12800_e17007_d_n9;
        locals.var_ccg_dn10 = assign12800_e17007_d_n10;
        locals.var_ccg_dn11 = assign12800_e17007_d_n11;
        locals.var_ccg_dn13 = assign12800_e17007_d_n13;
        locals.var_ccg_dn14 = assign12800_e17007_d_n14;

        let (assign12810_e17013,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12810_e17011: f64 = (locals.var_lmax / locals.var_wg);
        (assign12810_e17011,)
    } else {
        (locals.var_x,)
    }
};
        locals.var_x = assign12810_e17013;

        let (assign12820_e17026,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12820_e17019: f64 = (locals.var_x + 1.0);
        let assign12820_e17020: f64 = (2.0 * assign12820_e17019);
        let assign12820_e17021: f64 = (assign12820_e17020).sqrt();
        let assign12820_e17023: f64 = (assign12820_e17021 * 3.141592653589793);
        let assign12820_e17024: f64 = (4.0 / assign12820_e17023);
        (assign12820_e17024,)
    } else {
        (locals.var_c1,)
    }
};
        locals.var_c1 = assign12820_e17026;

        let (assign12830_e17060,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12830_e17030: f64 = (p.p90 * p.p90);
        let assign12830_e17033: f64 = (2.0 * locals.var_wg);
        let assign12830_e17035: f64 = (assign12830_e17033 * p.p90);
        let assign12830_e17036: f64 = (assign12830_e17030 + assign12830_e17035);
        let assign12830_e17039: f64 = (locals.var_wg * locals.var_wg);
        let assign12830_e17042: f64 = (locals.var_x + 1.0);
        let assign12830_e17043: f64 = (assign12830_e17039 * assign12830_e17042);
        let assign12830_e17044: f64 = (assign12830_e17036 + assign12830_e17043);
        let assign12830_e17045: f64 = (assign12830_e17044).sqrt();
        let assign12830_e17048: f64 = (locals.var_x + 1.0);
        let assign12830_e17049: f64 = (assign12830_e17048).sqrt();
        let assign12830_e17050: f64 = (assign12830_e17045 * assign12830_e17049);
        let assign12830_e17052: f64 = (assign12830_e17050 + p.p90);
        let assign12830_e17055: f64 = (locals.var_wg * locals.var_x);
        let assign12830_e17056: f64 = (assign12830_e17052 + assign12830_e17055);
        let assign12830_e17058: f64 = (assign12830_e17056 + locals.var_wg);
        (assign12830_e17058,)
    } else {
        (locals.var_c2,)
    }
};
        locals.var_c2 = assign12830_e17060;

        let (assign12840_e17079,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12840_e17065: f64 = (locals.var_x + 1.0);
        let assign12840_e17068: f64 = (locals.var_x + 4.0);
        let assign12840_e17069: f64 = (assign12840_e17065 * assign12840_e17068);
        let assign12840_e17070: f64 = (assign12840_e17069).sqrt();
        let assign12840_e17071: f64 = (p.p90 * assign12840_e17070);
        let assign12840_e17075: f64 = (locals.var_x + 2.0);
        let assign12840_e17076: f64 = (p.p90 * assign12840_e17075);
        let assign12840_e17077: f64 = (assign12840_e17071 + assign12840_e17076);
        (assign12840_e17077,)
    } else {
        (locals.var_c3,)
    }
};
        locals.var_c3 = assign12840_e17079;

        let (assign12850_e17108,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12850_e17085: f64 = (locals.var_c2 / locals.var_c3);
        let (assign12850_e17102,) = {
            if (!(assign12850_e17085 > 1e-38)) {
                let assign12850_e17090: f64 = (-87.498233534);
                (assign12850_e17090,)
            } else {
                let assign12850_e17093: f64 = (locals.var_c2 / locals.var_c3);
                let (assign12850_e17101,) = {
                    if (assign12850_e17093 > 1e-38) {
                        let assign12850_e17098: f64 = (locals.var_c2 / locals.var_c3);
                        let assign12850_e17099: f64 = (assign12850_e17098).ln();
                        (assign12850_e17099,)
                    } else {
                        (0.0,)
                    }
                };
                (assign12850_e17101,)
            }
        };
        let assign12850_e17103: f64 = (locals.var_c1 * assign12850_e17102);
        let assign12850_e17105: f64 = (assign12850_e17103 + 12.27);
        let assign12850_e17106: f64 = (locals.var_epssp * assign12850_e17105);
        (assign12850_e17106,)
    } else {
        (locals.var_cfglog,)
    }
};
        locals.var_cfglog = assign12850_e17108;

        let (assign12860_e17114,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12860_e17112: f64 = (locals.var_hr * locals.var_lr);
        (assign12860_e17112,)
    } else {
        (locals.var_dcf,)
    }
};
        locals.var_dcf = assign12860_e17114;

        let (assign12870_e17123,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12870_e17118: f64 = (locals.var_dcf * locals.var_dcf);
        let assign12870_e17120: f64 = (assign12870_e17118 + 1.0);
        let assign12870_e17121: f64 = (assign12870_e17120).sqrt();
        (assign12870_e17121,)
    } else {
        (locals.var_tt0,)
    }
};
        locals.var_tt0 = assign12870_e17123;

        let (assign12880_e17170, assign12880_e17170_d_n0, assign12880_e17170_d_n2, assign12880_e17170_d_n3, assign12880_e17170_d_n4, assign12880_e17170_d_n5, assign12880_e17170_d_n6, assign12880_e17170_d_n7, assign12880_e17170_d_n8, assign12880_e17170_d_n9, assign12880_e17170_d_n10, assign12880_e17170_d_n11, assign12880_e17170_d_n13, assign12880_e17170_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12880_e17127: f64 = (locals.var_dcf * locals.var_dcf);
        let assign12880_e17129: f64 = (assign12880_e17127 + 1.0);
        let assign12880_e17132: f64 = (locals.var_dcf * p.p90);
        let assign12880_e17135: f64 = (locals.var_dcf * p.p90);
        let assign12880_e17136: f64 = (assign12880_e17132 * assign12880_e17135);
        let assign12880_e17139: f64 = (2.0 * locals.var_dcf);
        let assign12880_e17141: f64 = (assign12880_e17139 * locals.var_lmax);
        let assign12880_e17143: f64 = (assign12880_e17141 * p.p90);
        let assign12880_e17144: f64 = (assign12880_e17136 + assign12880_e17143);
        let assign12880_e17147: f64 = (locals.var_dcf * locals.var_dcf);
        let assign12880_e17149: f64 = (assign12880_e17147 + 1.0);
        let assign12880_e17151: f64 = (assign12880_e17149 * locals.var_lmax);
        let assign12880_e17153: f64 = (assign12880_e17151 * locals.var_lmax);
        let assign12880_e17154: f64 = (assign12880_e17144 + assign12880_e17153);
        let assign12880_e17155: f64 = (assign12880_e17129 * assign12880_e17154);
        let assign12880_e17156: f64 = (assign12880_e17155).sqrt();
        let assign12880_e17159: f64 = (locals.var_dcf * p.p90);
        let assign12880_e17160: f64 = (assign12880_e17156 + assign12880_e17159);
        let assign12880_e17163: f64 = (locals.var_dcf * locals.var_dcf);
        let assign12880_e17165: f64 = (assign12880_e17163 * locals.var_lmax);
        let assign12880_e17166: f64 = (assign12880_e17160 + assign12880_e17165);
        let assign12880_e17168: f64 = (assign12880_e17166 + locals.var_lmax);
        (assign12880_e17168, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign12880_e17170;
        locals.var_tt1_dn0 = assign12880_e17170_d_n0;
        locals.var_tt1_dn2 = assign12880_e17170_d_n2;
        locals.var_tt1_dn3 = assign12880_e17170_d_n3;
        locals.var_tt1_dn4 = assign12880_e17170_d_n4;
        locals.var_tt1_dn5 = assign12880_e17170_d_n5;
        locals.var_tt1_dn6 = assign12880_e17170_d_n6;
        locals.var_tt1_dn7 = assign12880_e17170_d_n7;
        locals.var_tt1_dn8 = assign12880_e17170_d_n8;
        locals.var_tt1_dn9 = assign12880_e17170_d_n9;
        locals.var_tt1_dn10 = assign12880_e17170_d_n10;
        locals.var_tt1_dn11 = assign12880_e17170_d_n11;
        locals.var_tt1_dn13 = assign12880_e17170_d_n13;
        locals.var_tt1_dn14 = assign12880_e17170_d_n14;

        let (assign12890_e17180,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12890_e17174: f64 = (locals.var_tt0 + 1.0);
        let assign12890_e17177: f64 = (locals.var_dcf * p.p90);
        let assign12890_e17178: f64 = (assign12890_e17174 * assign12890_e17177);
        (assign12890_e17178,)
    } else {
        (locals.var_tt2,)
    }
};
        locals.var_tt2 = assign12890_e17180;

        let (assign12900_e17218, assign12900_e17218_d_n0, assign12900_e17218_d_n2, assign12900_e17218_d_n3, assign12900_e17218_d_n4, assign12900_e17218_d_n5, assign12900_e17218_d_n6, assign12900_e17218_d_n7, assign12900_e17218_d_n8, assign12900_e17218_d_n9, assign12900_e17218_d_n10, assign12900_e17218_d_n11, assign12900_e17218_d_n13, assign12900_e17218_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12900_e17184: f64 = (2.0 * locals.var_epssp);
        let assign12900_e17186: f64 = (2.0_f64).sqrt();
        let assign12900_e17187: f64 = (assign12900_e17184 * assign12900_e17186);
        let assign12900_e17189: f64 = (assign12900_e17187 / 3.141592653589793);
        let assign12900_e17191: f64 = (assign12900_e17189 * 0.85);
        let assign12900_e17193: f64 = (assign12900_e17191 * locals.var_dcf);
        let assign12900_e17195: f64 = (assign12900_e17193 / locals.var_tt0);
        let assign12900_e17198: f64 = (locals.var_tt1 / locals.var_tt2);
        let (assign12900_e17215, assign12900_e17215_d_n0, assign12900_e17215_d_n2, assign12900_e17215_d_n3, assign12900_e17215_d_n4, assign12900_e17215_d_n5, assign12900_e17215_d_n6, assign12900_e17215_d_n7, assign12900_e17215_d_n8, assign12900_e17215_d_n9, assign12900_e17215_d_n10, assign12900_e17215_d_n11, assign12900_e17215_d_n13, assign12900_e17215_d_n14,) = {
            if (!(assign12900_e17198 > 1e-38)) {
                let assign12900_e17203: f64 = (-87.498233534);
                (assign12900_e17203, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign12900_e17206: f64 = (locals.var_tt1 / locals.var_tt2);
                let (assign12900_e17214, assign12900_e17214_d_n0, assign12900_e17214_d_n2, assign12900_e17214_d_n3, assign12900_e17214_d_n4, assign12900_e17214_d_n5, assign12900_e17214_d_n6, assign12900_e17214_d_n7, assign12900_e17214_d_n8, assign12900_e17214_d_n9, assign12900_e17214_d_n10, assign12900_e17214_d_n11, assign12900_e17214_d_n13, assign12900_e17214_d_n14,) = {
                    if (assign12900_e17206 > 1e-38) {
                        let assign12900_e17211: f64 = (locals.var_tt1 / locals.var_tt2);
                        let assign12900_e17212: f64 = (assign12900_e17211).ln();
                        (assign12900_e17212, ((locals.var_tt1_dn0 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn2 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn3 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn4 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn5 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn6 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn7 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn8 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn9 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn10 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn11 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn13 / locals.var_tt2) / assign12900_e17211), ((locals.var_tt1_dn14 / locals.var_tt2) / assign12900_e17211),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign12900_e17214, assign12900_e17214_d_n0, assign12900_e17214_d_n2, assign12900_e17214_d_n3, assign12900_e17214_d_n4, assign12900_e17214_d_n5, assign12900_e17214_d_n6, assign12900_e17214_d_n7, assign12900_e17214_d_n8, assign12900_e17214_d_n9, assign12900_e17214_d_n10, assign12900_e17214_d_n11, assign12900_e17214_d_n13, assign12900_e17214_d_n14,)
            }
        };
        let assign12900_e17216: f64 = (assign12900_e17195 * assign12900_e17215);
        (assign12900_e17216, (assign12900_e17195 * assign12900_e17215_d_n0), (assign12900_e17195 * assign12900_e17215_d_n2), (assign12900_e17195 * assign12900_e17215_d_n3), (assign12900_e17195 * assign12900_e17215_d_n4), (assign12900_e17195 * assign12900_e17215_d_n5), (assign12900_e17195 * assign12900_e17215_d_n6), (assign12900_e17195 * assign12900_e17215_d_n7), (assign12900_e17195 * assign12900_e17215_d_n8), (assign12900_e17195 * assign12900_e17215_d_n9), (assign12900_e17195 * assign12900_e17215_d_n10), (assign12900_e17195 * assign12900_e17215_d_n11), (assign12900_e17195 * assign12900_e17215_d_n13), (assign12900_e17195 * assign12900_e17215_d_n14),)
    } else {
        (locals.var_cfgsat, locals.var_cfgsat_dn0, locals.var_cfgsat_dn2, locals.var_cfgsat_dn3, locals.var_cfgsat_dn4, locals.var_cfgsat_dn5, locals.var_cfgsat_dn6, locals.var_cfgsat_dn7, locals.var_cfgsat_dn8, locals.var_cfgsat_dn9, locals.var_cfgsat_dn10, locals.var_cfgsat_dn11, locals.var_cfgsat_dn13, locals.var_cfgsat_dn14,)
    }
};
        locals.var_cfgsat = assign12900_e17218;
        locals.var_cfgsat_dn0 = assign12900_e17218_d_n0;
        locals.var_cfgsat_dn2 = assign12900_e17218_d_n2;
        locals.var_cfgsat_dn3 = assign12900_e17218_d_n3;
        locals.var_cfgsat_dn4 = assign12900_e17218_d_n4;
        locals.var_cfgsat_dn5 = assign12900_e17218_d_n5;
        locals.var_cfgsat_dn6 = assign12900_e17218_d_n6;
        locals.var_cfgsat_dn7 = assign12900_e17218_d_n7;
        locals.var_cfgsat_dn8 = assign12900_e17218_d_n8;
        locals.var_cfgsat_dn9 = assign12900_e17218_d_n9;
        locals.var_cfgsat_dn10 = assign12900_e17218_d_n10;
        locals.var_cfgsat_dn11 = assign12900_e17218_d_n11;
        locals.var_cfgsat_dn13 = assign12900_e17218_d_n13;
        locals.var_cfgsat_dn14 = assign12900_e17218_d_n14;

        let (assign12910_e17222, assign12910_e17222_d_n0, assign12910_e17222_d_n2, assign12910_e17222_d_n3, assign12910_e17222_d_n4, assign12910_e17222_d_n5, assign12910_e17222_d_n6, assign12910_e17222_d_n7, assign12910_e17222_d_n8, assign12910_e17222_d_n9, assign12910_e17222_d_n10, assign12910_e17222_d_n11, assign12910_e17222_d_n13, assign12910_e17222_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        (1.2e-12, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_delta, locals.var_delta_dn0, locals.var_delta_dn2, locals.var_delta_dn3, locals.var_delta_dn4, locals.var_delta_dn5, locals.var_delta_dn6, locals.var_delta_dn7, locals.var_delta_dn8, locals.var_delta_dn9, locals.var_delta_dn10, locals.var_delta_dn11, locals.var_delta_dn13, locals.var_delta_dn14,)
    }
};
        locals.var_delta = assign12910_e17222;
        locals.var_delta_dn0 = assign12910_e17222_d_n0;
        locals.var_delta_dn2 = assign12910_e17222_d_n2;
        locals.var_delta_dn3 = assign12910_e17222_d_n3;
        locals.var_delta_dn4 = assign12910_e17222_d_n4;
        locals.var_delta_dn5 = assign12910_e17222_d_n5;
        locals.var_delta_dn6 = assign12910_e17222_d_n6;
        locals.var_delta_dn7 = assign12910_e17222_d_n7;
        locals.var_delta_dn8 = assign12910_e17222_d_n8;
        locals.var_delta_dn9 = assign12910_e17222_d_n9;
        locals.var_delta_dn10 = assign12910_e17222_d_n10;
        locals.var_delta_dn11 = assign12910_e17222_d_n11;
        locals.var_delta_dn13 = assign12910_e17222_d_n13;
        locals.var_delta_dn14 = assign12910_e17222_d_n14;

        let (assign12920_e17230, assign12920_e17230_d_n0, assign12920_e17230_d_n2, assign12920_e17230_d_n3, assign12920_e17230_d_n4, assign12920_e17230_d_n5, assign12920_e17230_d_n6, assign12920_e17230_d_n7, assign12920_e17230_d_n8, assign12920_e17230_d_n9, assign12920_e17230_d_n10, assign12920_e17230_d_n11, assign12920_e17230_d_n13, assign12920_e17230_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12920_e17226: f64 = (locals.var_cfgsat - locals.var_cfglog);
        let assign12920_e17228: f64 = (assign12920_e17226 - locals.var_delta);
        (assign12920_e17228, (locals.var_cfgsat_dn0 - locals.var_delta_dn0), (locals.var_cfgsat_dn2 - locals.var_delta_dn2), (locals.var_cfgsat_dn3 - locals.var_delta_dn3), (locals.var_cfgsat_dn4 - locals.var_delta_dn4), (locals.var_cfgsat_dn5 - locals.var_delta_dn5), (locals.var_cfgsat_dn6 - locals.var_delta_dn6), (locals.var_cfgsat_dn7 - locals.var_delta_dn7), (locals.var_cfgsat_dn8 - locals.var_delta_dn8), (locals.var_cfgsat_dn9 - locals.var_delta_dn9), (locals.var_cfgsat_dn10 - locals.var_delta_dn10), (locals.var_cfgsat_dn11 - locals.var_delta_dn11), (locals.var_cfgsat_dn13 - locals.var_delta_dn13), (locals.var_cfgsat_dn14 - locals.var_delta_dn14),)
    } else {
        (locals.var_tt1, locals.var_tt1_dn0, locals.var_tt1_dn2, locals.var_tt1_dn3, locals.var_tt1_dn4, locals.var_tt1_dn5, locals.var_tt1_dn6, locals.var_tt1_dn7, locals.var_tt1_dn8, locals.var_tt1_dn9, locals.var_tt1_dn10, locals.var_tt1_dn11, locals.var_tt1_dn13, locals.var_tt1_dn14,)
    }
};
        locals.var_tt1 = assign12920_e17230;
        locals.var_tt1_dn0 = assign12920_e17230_d_n0;
        locals.var_tt1_dn2 = assign12920_e17230_d_n2;
        locals.var_tt1_dn3 = assign12920_e17230_d_n3;
        locals.var_tt1_dn4 = assign12920_e17230_d_n4;
        locals.var_tt1_dn5 = assign12920_e17230_d_n5;
        locals.var_tt1_dn6 = assign12920_e17230_d_n6;
        locals.var_tt1_dn7 = assign12920_e17230_d_n7;
        locals.var_tt1_dn8 = assign12920_e17230_d_n8;
        locals.var_tt1_dn9 = assign12920_e17230_d_n9;
        locals.var_tt1_dn10 = assign12920_e17230_d_n10;
        locals.var_tt1_dn11 = assign12920_e17230_d_n11;
        locals.var_tt1_dn13 = assign12920_e17230_d_n13;
        locals.var_tt1_dn14 = assign12920_e17230_d_n14;

        let (assign12930_e17251, assign12930_e17251_d_n0, assign12930_e17251_d_n2, assign12930_e17251_d_n3, assign12930_e17251_d_n4, assign12930_e17251_d_n5, assign12930_e17251_d_n6, assign12930_e17251_d_n7, assign12930_e17251_d_n8, assign12930_e17251_d_n9, assign12930_e17251_d_n10, assign12930_e17251_d_n11, assign12930_e17251_d_n13, assign12930_e17251_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12930_e17238: f64 = (locals.var_tt1 * locals.var_tt1);
        let assign12930_e17241: f64 = (4.0 * locals.var_delta);
        let assign12930_e17243: f64 = (assign12930_e17241 * locals.var_cfgsat);
        let assign12930_e17244: f64 = (assign12930_e17238 + assign12930_e17243);
        let assign12930_e17245: f64 = (assign12930_e17244).sqrt();
        let assign12930_e17246: f64 = (locals.var_tt1 + assign12930_e17245);
        let assign12930_e17247: f64 = (0.5 * assign12930_e17246);
        let assign12930_e17248: f64 = (locals.var_cfgsat - assign12930_e17247);
        let assign12930_e17249: f64 = (p.p42 * assign12930_e17248);
        (assign12930_e17249, (p.p42 * (locals.var_cfgsat_dn0 - (0.5 * (locals.var_tt1_dn0 + ((((locals.var_tt1_dn0 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn0)) + (((4.0 * locals.var_delta_dn0) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn0))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn2 - (0.5 * (locals.var_tt1_dn2 + ((((locals.var_tt1_dn2 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn2)) + (((4.0 * locals.var_delta_dn2) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn2))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn3 - (0.5 * (locals.var_tt1_dn3 + ((((locals.var_tt1_dn3 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn3)) + (((4.0 * locals.var_delta_dn3) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn3))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn4 - (0.5 * (locals.var_tt1_dn4 + ((((locals.var_tt1_dn4 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn4)) + (((4.0 * locals.var_delta_dn4) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn4))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn5 - (0.5 * (locals.var_tt1_dn5 + ((((locals.var_tt1_dn5 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn5)) + (((4.0 * locals.var_delta_dn5) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn5))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn6 - (0.5 * (locals.var_tt1_dn6 + ((((locals.var_tt1_dn6 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn6)) + (((4.0 * locals.var_delta_dn6) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn6))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn7 - (0.5 * (locals.var_tt1_dn7 + ((((locals.var_tt1_dn7 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn7)) + (((4.0 * locals.var_delta_dn7) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn7))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn8 - (0.5 * (locals.var_tt1_dn8 + ((((locals.var_tt1_dn8 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn8)) + (((4.0 * locals.var_delta_dn8) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn8))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn9 - (0.5 * (locals.var_tt1_dn9 + ((((locals.var_tt1_dn9 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn9)) + (((4.0 * locals.var_delta_dn9) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn9))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn10 - (0.5 * (locals.var_tt1_dn10 + ((((locals.var_tt1_dn10 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn10)) + (((4.0 * locals.var_delta_dn10) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn10))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn11 - (0.5 * (locals.var_tt1_dn11 + ((((locals.var_tt1_dn11 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn11)) + (((4.0 * locals.var_delta_dn11) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn11))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn13 - (0.5 * (locals.var_tt1_dn13 + ((((locals.var_tt1_dn13 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn13)) + (((4.0 * locals.var_delta_dn13) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn13))) / (2.0 * assign12930_e17245)))))), (p.p42 * (locals.var_cfgsat_dn14 - (0.5 * (locals.var_tt1_dn14 + ((((locals.var_tt1_dn14 * locals.var_tt1) + (locals.var_tt1 * locals.var_tt1_dn14)) + (((4.0 * locals.var_delta_dn14) * locals.var_cfgsat) + (assign12930_e17241 * locals.var_cfgsat_dn14))) / (2.0 * assign12930_e17245)))))),)
    } else {
        (locals.var_cfg, locals.var_cfg_dn0, locals.var_cfg_dn2, locals.var_cfg_dn3, locals.var_cfg_dn4, locals.var_cfg_dn5, locals.var_cfg_dn6, locals.var_cfg_dn7, locals.var_cfg_dn8, locals.var_cfg_dn9, locals.var_cfg_dn10, locals.var_cfg_dn11, locals.var_cfg_dn13, locals.var_cfg_dn14,)
    }
};
        locals.var_cfg = assign12930_e17251;
        locals.var_cfg_dn0 = assign12930_e17251_d_n0;
        locals.var_cfg_dn2 = assign12930_e17251_d_n2;
        locals.var_cfg_dn3 = assign12930_e17251_d_n3;
        locals.var_cfg_dn4 = assign12930_e17251_d_n4;
        locals.var_cfg_dn5 = assign12930_e17251_d_n5;
        locals.var_cfg_dn6 = assign12930_e17251_d_n6;
        locals.var_cfg_dn7 = assign12930_e17251_d_n7;
        locals.var_cfg_dn8 = assign12930_e17251_d_n8;
        locals.var_cfg_dn9 = assign12930_e17251_d_n9;
        locals.var_cfg_dn10 = assign12930_e17251_d_n10;
        locals.var_cfg_dn11 = assign12930_e17251_d_n11;
        locals.var_cfg_dn13 = assign12930_e17251_d_n13;
        locals.var_cfg_dn14 = assign12930_e17251_d_n14;

        let (assign12940_e17257, assign12940_e17257_d_n0, assign12940_e17257_d_n2, assign12940_e17257_d_n3, assign12940_e17257_d_n4, assign12940_e17257_d_n5, assign12940_e17257_d_n6, assign12940_e17257_d_n7, assign12940_e17257_d_n8, assign12940_e17257_d_n9, assign12940_e17257_d_n10, assign12940_e17257_d_n11, assign12940_e17257_d_n13, assign12940_e17257_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign12940_e17255: f64 = (locals.var_ccg + locals.var_cfg);
        (assign12940_e17255, (locals.var_ccg_dn0 + locals.var_cfg_dn0), (locals.var_ccg_dn2 + locals.var_cfg_dn2), (locals.var_ccg_dn3 + locals.var_cfg_dn3), (locals.var_ccg_dn4 + locals.var_cfg_dn4), (locals.var_ccg_dn5 + locals.var_cfg_dn5), (locals.var_ccg_dn6 + locals.var_cfg_dn6), (locals.var_ccg_dn7 + locals.var_cfg_dn7), (locals.var_ccg_dn8 + locals.var_cfg_dn8), (locals.var_ccg_dn9 + locals.var_cfg_dn9), (locals.var_ccg_dn10 + locals.var_cfg_dn10), (locals.var_ccg_dn11 + locals.var_cfg_dn11), (locals.var_ccg_dn13 + locals.var_cfg_dn13), (locals.var_ccg_dn14 + locals.var_cfg_dn14),)
    } else {
        (locals.var_cgg_sidepff, locals.var_cgg_sidepff_dn0, locals.var_cgg_sidepff_dn2, locals.var_cgg_sidepff_dn3, locals.var_cgg_sidepff_dn4, locals.var_cgg_sidepff_dn5, locals.var_cgg_sidepff_dn6, locals.var_cgg_sidepff_dn7, locals.var_cgg_sidepff_dn8, locals.var_cgg_sidepff_dn9, locals.var_cgg_sidepff_dn10, locals.var_cgg_sidepff_dn11, locals.var_cgg_sidepff_dn13, locals.var_cgg_sidepff_dn14,)
    }
};
        locals.var_cgg_sidepff = assign12940_e17257;
        locals.var_cgg_sidepff_dn0 = assign12940_e17257_d_n0;
        locals.var_cgg_sidepff_dn2 = assign12940_e17257_d_n2;
        locals.var_cgg_sidepff_dn3 = assign12940_e17257_d_n3;
        locals.var_cgg_sidepff_dn4 = assign12940_e17257_d_n4;
        locals.var_cgg_sidepff_dn5 = assign12940_e17257_d_n5;
        locals.var_cgg_sidepff_dn6 = assign12940_e17257_d_n6;
        locals.var_cgg_sidepff_dn7 = assign12940_e17257_d_n7;
        locals.var_cgg_sidepff_dn8 = assign12940_e17257_d_n8;
        locals.var_cgg_sidepff_dn9 = assign12940_e17257_d_n9;
        locals.var_cgg_sidepff_dn10 = assign12940_e17257_d_n10;
        locals.var_cgg_sidepff_dn11 = assign12940_e17257_d_n11;
        locals.var_cgg_sidepff_dn13 = assign12940_e17257_d_n13;
        locals.var_cgg_sidepff_dn14 = assign12940_e17257_d_n14;

        let assign12950_e17260: f64 = if p.p1090 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard222 = assign12950_e17260;

        let (assign12960_e17266,) = {
    if ((locals.var_guard213 != 0.0) && (locals.var_guard222 != 0.0)) {
        (0.0,)
    } else {
        (locals.var_acorner_topm,)
    }
};
        locals.var_acorner_topm = assign12960_e17266;

        let assign12970_e17269: f64 = if p.p1080 > 0.0 { 1.0 } else { 0.0 };
        locals.var_guard223 = assign12970_e17269;

        let (assign12980_e17286,) = {
    if (((locals.var_guard213 != 0.0) && (locals.var_guard222 == 0.0)) && (locals.var_guard223 != 0.0)) {
        let assign12980_e17278: f64 = (p.p4 - p.p43);
        let assign12980_e17281: f64 = (p.p1080 * p.p1084);
        let assign12980_e17283: f64 = (assign12980_e17281 + p.p1081);
        let assign12980_e17284: f64 = (assign12980_e17278 * assign12980_e17283);
        (assign12980_e17284,)
    } else {
        (locals.var_acorner_topm,)
    }
};
        locals.var_acorner_topm = assign12980_e17286;

        let (assign12990_e17300,) = {
    if (((locals.var_guard213 != 0.0) && (locals.var_guard222 == 0.0)) && (locals.var_guard223 == 0.0)) {
        let assign12990_e17296: f64 = (p.p4 - p.p43);
        let assign12990_e17298: f64 = (assign12990_e17296 * locals.var_hrsd);
        (assign12990_e17298,)
    } else {
        (locals.var_acorner_topm,)
    }
};
        locals.var_acorner_topm = assign12990_e17300;

    }

    pub(super) fn stamp_transient_block_33(
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
        locals: &mut StampLocals,
    ) {
        let (assign13000_e17308,) = {
    if (locals.var_guard213 != 0.0) {
        let assign13000_e17304: f64 = (p.p4 - p.p43);
        let assign13000_e17306: f64 = (assign13000_e17304 * locals.var_hrsd2);
        (assign13000_e17306,)
    } else {
        (locals.var_acorner_tb,)
    }
};
        locals.var_acorner_tb = assign13000_e17308;

        let (assign13010_e17328,) = {
    if (locals.var_guard213 != 0.0) {
        let assign13010_e17314: f64 = (2.0 * p.p56);
        let assign13010_e17316: f64 = (assign13010_e17314 * locals.var_acorner_tb);
        let assign13010_e17317: f64 = (locals.var_acorner_topm + assign13010_e17316);
        let assign13010_e17318: f64 = (p.p5 * assign13010_e17317);
        let assign13010_e17320: f64 = (assign13010_e17318 + p.p1092);
        let assign13010_e17322: f64 = (assign13010_e17320 + p.p1091);
        let assign13010_e17324: f64 = (assign13010_e17322 * locals.var_epssp);
        let assign13010_e17326: f64 = (assign13010_e17324 / p.p1087);
        (assign13010_e17326,)
    } else {
        (locals.var_ccorner,)
    }
};
        locals.var_ccorner = assign13010_e17328;

        let (assign13020_e17360, assign13020_e17360_d_n0, assign13020_e17360_d_n2, assign13020_e17360_d_n3, assign13020_e17360_d_n4, assign13020_e17360_d_n5, assign13020_e17360_d_n6, assign13020_e17360_d_n7, assign13020_e17360_d_n8, assign13020_e17360_d_n9, assign13020_e17360_d_n10, assign13020_e17360_d_n11, assign13020_e17360_d_n13, assign13020_e17360_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign13020_e17334: f64 = (2.0 * p.p56);
        let assign13020_e17336: f64 = (assign13020_e17334 * locals.var_cgg_tb);
        let assign13020_e17337: f64 = (locals.var_cgg_topm + assign13020_e17336);
        let assign13020_e17339: f64 = (assign13020_e17337 * p.p5);
        let assign13020_e17340: f64 = (locals.var_ccorner + assign13020_e17339);
        let assign13020_e17346: f64 = (p.p56 - 1.0);
        let assign13020_e17347: f64 = (locals.var_cgg_sidetb * assign13020_e17346);
        let assign13020_e17348: f64 = (locals.var_cgg_sidetopm + assign13020_e17347);
        let assign13020_e17350: f64 = (assign13020_e17348 + locals.var_cgg_sidepff);
        let assign13020_e17351: f64 = (p.p1103 * assign13020_e17350);
        let assign13020_e17353: f64 = (assign13020_e17351 * p.p5);
        let assign13020_e17355: f64 = (assign13020_e17353 * 2.0);
        let assign13020_e17356: f64 = (assign13020_e17340 + assign13020_e17355);
        let assign13020_e17358: f64 = (assign13020_e17356 * p.p59);
        (assign13020_e17358, ((((locals.var_cgg_topm_dn0 + (assign13020_e17334 * locals.var_cgg_tb_dn0)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn0 + (locals.var_cgg_sidetb_dn0 * assign13020_e17346)) + locals.var_cgg_sidepff_dn0)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn2 + (assign13020_e17334 * locals.var_cgg_tb_dn2)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn2 + (locals.var_cgg_sidetb_dn2 * assign13020_e17346)) + locals.var_cgg_sidepff_dn2)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn3 + (assign13020_e17334 * locals.var_cgg_tb_dn3)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn3 + (locals.var_cgg_sidetb_dn3 * assign13020_e17346)) + locals.var_cgg_sidepff_dn3)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn4 + (assign13020_e17334 * locals.var_cgg_tb_dn4)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn4 + (locals.var_cgg_sidetb_dn4 * assign13020_e17346)) + locals.var_cgg_sidepff_dn4)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn5 + (assign13020_e17334 * locals.var_cgg_tb_dn5)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn5 + (locals.var_cgg_sidetb_dn5 * assign13020_e17346)) + locals.var_cgg_sidepff_dn5)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn6 + (assign13020_e17334 * locals.var_cgg_tb_dn6)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn6 + (locals.var_cgg_sidetb_dn6 * assign13020_e17346)) + locals.var_cgg_sidepff_dn6)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn7 + (assign13020_e17334 * locals.var_cgg_tb_dn7)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn7 + (locals.var_cgg_sidetb_dn7 * assign13020_e17346)) + locals.var_cgg_sidepff_dn7)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn8 + (assign13020_e17334 * locals.var_cgg_tb_dn8)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn8 + (locals.var_cgg_sidetb_dn8 * assign13020_e17346)) + locals.var_cgg_sidepff_dn8)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn9 + (assign13020_e17334 * locals.var_cgg_tb_dn9)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn9 + (locals.var_cgg_sidetb_dn9 * assign13020_e17346)) + locals.var_cgg_sidepff_dn9)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn10 + (assign13020_e17334 * locals.var_cgg_tb_dn10)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn10 + (locals.var_cgg_sidetb_dn10 * assign13020_e17346)) + locals.var_cgg_sidepff_dn10)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn11 + (assign13020_e17334 * locals.var_cgg_tb_dn11)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn11 + (locals.var_cgg_sidetb_dn11 * assign13020_e17346)) + locals.var_cgg_sidepff_dn11)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn13 + (assign13020_e17334 * locals.var_cgg_tb_dn13)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn13 + (locals.var_cgg_sidetb_dn13 * assign13020_e17346)) + locals.var_cgg_sidepff_dn13)) * p.p5) * 2.0)) * p.p59), ((((locals.var_cgg_topm_dn14 + (assign13020_e17334 * locals.var_cgg_tb_dn14)) * p.p5) + (((p.p1103 * ((locals.var_cgg_sidetopm_dn14 + (locals.var_cgg_sidetb_dn14 * assign13020_e17346)) + locals.var_cgg_sidepff_dn14)) * p.p5) * 2.0)) * p.p59),)
    } else {
        (locals.var_cfr_geo, locals.var_cfr_geo_dn0, locals.var_cfr_geo_dn2, locals.var_cfr_geo_dn3, locals.var_cfr_geo_dn4, locals.var_cfr_geo_dn5, locals.var_cfr_geo_dn6, locals.var_cfr_geo_dn7, locals.var_cfr_geo_dn8, locals.var_cfr_geo_dn9, locals.var_cfr_geo_dn10, locals.var_cfr_geo_dn11, locals.var_cfr_geo_dn13, locals.var_cfr_geo_dn14,)
    }
};
        locals.var_cfr_geo = assign13020_e17360;
        locals.var_cfr_geo_dn0 = assign13020_e17360_d_n0;
        locals.var_cfr_geo_dn2 = assign13020_e17360_d_n2;
        locals.var_cfr_geo_dn3 = assign13020_e17360_d_n3;
        locals.var_cfr_geo_dn4 = assign13020_e17360_d_n4;
        locals.var_cfr_geo_dn5 = assign13020_e17360_d_n5;
        locals.var_cfr_geo_dn6 = assign13020_e17360_d_n6;
        locals.var_cfr_geo_dn7 = assign13020_e17360_d_n7;
        locals.var_cfr_geo_dn8 = assign13020_e17360_d_n8;
        locals.var_cfr_geo_dn9 = assign13020_e17360_d_n9;
        locals.var_cfr_geo_dn10 = assign13020_e17360_d_n10;
        locals.var_cfr_geo_dn11 = assign13020_e17360_d_n11;
        locals.var_cfr_geo_dn13 = assign13020_e17360_d_n13;
        locals.var_cfr_geo_dn14 = assign13020_e17360_d_n14;

        let (assign13030_e17380, assign13030_e17380_d_n0, assign13030_e17380_d_n2, assign13030_e17380_d_n3, assign13030_e17380_d_n4, assign13030_e17380_d_n5, assign13030_e17380_d_n6, assign13030_e17380_d_n7, assign13030_e17380_d_n8, assign13030_e17380_d_n9, assign13030_e17380_d_n10, assign13030_e17380_d_n11, assign13030_e17380_d_n13, assign13030_e17380_d_n14,) = {
    if (locals.var_guard213 != 0.0) {
        let assign13030_e17367: f64 = (p.p1100 * p.p43);
        let assign13030_e17368: f64 = (p.p1099 + assign13030_e17367);
        let assign13030_e17371: f64 = (p.p1101 * p.p4);
        let assign13030_e17372: f64 = (assign13030_e17368 + assign13030_e17371);
        let assign13030_e17375: f64 = (p.p1102 * p.p20);
        let assign13030_e17376: f64 = (assign13030_e17372 + assign13030_e17375);
        let assign13030_e17377: f64 = (0.0_f64).max(assign13030_e17376);
        let assign13030_e17378: f64 = (locals.var_cfr_geo * assign13030_e17377);
        (assign13030_e17378, (locals.var_cfr_geo_dn0 * assign13030_e17377), (locals.var_cfr_geo_dn2 * assign13030_e17377), (locals.var_cfr_geo_dn3 * assign13030_e17377), (locals.var_cfr_geo_dn4 * assign13030_e17377), (locals.var_cfr_geo_dn5 * assign13030_e17377), (locals.var_cfr_geo_dn6 * assign13030_e17377), (locals.var_cfr_geo_dn7 * assign13030_e17377), (locals.var_cfr_geo_dn8 * assign13030_e17377), (locals.var_cfr_geo_dn9 * assign13030_e17377), (locals.var_cfr_geo_dn10 * assign13030_e17377), (locals.var_cfr_geo_dn11 * assign13030_e17377), (locals.var_cfr_geo_dn13 * assign13030_e17377), (locals.var_cfr_geo_dn14 * assign13030_e17377),)
    } else {
        (locals.var_cfr_geo, locals.var_cfr_geo_dn0, locals.var_cfr_geo_dn2, locals.var_cfr_geo_dn3, locals.var_cfr_geo_dn4, locals.var_cfr_geo_dn5, locals.var_cfr_geo_dn6, locals.var_cfr_geo_dn7, locals.var_cfr_geo_dn8, locals.var_cfr_geo_dn9, locals.var_cfr_geo_dn10, locals.var_cfr_geo_dn11, locals.var_cfr_geo_dn13, locals.var_cfr_geo_dn14,)
    }
};
        locals.var_cfr_geo = assign13030_e17380;
        locals.var_cfr_geo_dn0 = assign13030_e17380_d_n0;
        locals.var_cfr_geo_dn2 = assign13030_e17380_d_n2;
        locals.var_cfr_geo_dn3 = assign13030_e17380_d_n3;
        locals.var_cfr_geo_dn4 = assign13030_e17380_d_n4;
        locals.var_cfr_geo_dn5 = assign13030_e17380_d_n5;
        locals.var_cfr_geo_dn6 = assign13030_e17380_d_n6;
        locals.var_cfr_geo_dn7 = assign13030_e17380_d_n7;
        locals.var_cfr_geo_dn8 = assign13030_e17380_d_n8;
        locals.var_cfr_geo_dn9 = assign13030_e17380_d_n9;
        locals.var_cfr_geo_dn10 = assign13030_e17380_d_n10;
        locals.var_cfr_geo_dn11 = assign13030_e17380_d_n11;
        locals.var_cfr_geo_dn13 = assign13030_e17380_d_n13;
        locals.var_cfr_geo_dn14 = assign13030_e17380_d_n14;

        let assign13040_e17385: f64 = (p.p92 / p.p91);
        let assign13040_e17386: f64 = (1.0 + assign13040_e17385);
        let (assign13040_e17407,) = {
    if (!(assign13040_e17386 > 1e-38)) {
        let assign13040_e17391: f64 = (-87.498233534);
        (assign13040_e17391,)
    } else {
        let assign13040_e17395: f64 = (p.p92 / p.p91);
        let assign13040_e17396: f64 = (1.0 + assign13040_e17395);
        let (assign13040_e17406,) = {
            if (assign13040_e17396 > 1e-38) {
                let assign13040_e17402: f64 = (p.p92 / p.p91);
                let assign13040_e17403: f64 = (1.0 + assign13040_e17402);
                let assign13040_e17404: f64 = (assign13040_e17403).ln();
                (assign13040_e17404,)
            } else {
                (0.0,)
            }
        };
        (assign13040_e17406,)
    }
};
        let assign13040_e17408: f64 = (p.p1583 * assign13040_e17407);
        locals.var_t0 = assign13040_e17408;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let assign13050_e17411: f64 = (locals.var_cbox * p.p7);
        let assign13050_e17417: f64 = (p.p4 * locals.var_nfintotal);
        let assign13050_e17418: f64 = (p.p9 - assign13050_e17417);
        let assign13050_e17419: f64 = (0.0_f64).max(assign13050_e17418);
        let assign13050_e17420: f64 = (locals.var_t0 * assign13050_e17419);
        let assign13050_e17421: f64 = (assign13050_e17411 + assign13050_e17420);
        locals.var_csbox = assign13050_e17421;
        locals.var_csbox_dn0 = (locals.var_t0_dn0 * assign13050_e17419);
        locals.var_csbox_dn2 = (locals.var_t0_dn2 * assign13050_e17419);
        locals.var_csbox_dn3 = (locals.var_t0_dn3 * assign13050_e17419);
        locals.var_csbox_dn4 = (locals.var_t0_dn4 * assign13050_e17419);
        locals.var_csbox_dn5 = (locals.var_t0_dn5 * assign13050_e17419);
        locals.var_csbox_dn6 = (locals.var_t0_dn6 * assign13050_e17419);
        locals.var_csbox_dn7 = (locals.var_t0_dn7 * assign13050_e17419);
        locals.var_csbox_dn8 = (locals.var_t0_dn8 * assign13050_e17419);
        locals.var_csbox_dn9 = (locals.var_t0_dn9 * assign13050_e17419);
        locals.var_csbox_dn10 = (locals.var_t0_dn10 * assign13050_e17419);
        locals.var_csbox_dn11 = (locals.var_t0_dn11 * assign13050_e17419);
        locals.var_csbox_dn13 = (locals.var_t0_dn13 * assign13050_e17419);
        locals.var_csbox_dn14 = (locals.var_t0_dn14 * assign13050_e17419);

        let assign13060_e17424: f64 = (locals.var_cbox * p.p8);
        let assign13060_e17430: f64 = (p.p4 * locals.var_nfintotal);
        let assign13060_e17431: f64 = (p.p10 - assign13060_e17430);
        let assign13060_e17432: f64 = (0.0_f64).max(assign13060_e17431);
        let assign13060_e17433: f64 = (locals.var_t0 * assign13060_e17432);
        let assign13060_e17434: f64 = (assign13060_e17424 + assign13060_e17433);
        locals.var_cdbox = assign13060_e17434;
        locals.var_cdbox_dn0 = (locals.var_t0_dn0 * assign13060_e17432);
        locals.var_cdbox_dn2 = (locals.var_t0_dn2 * assign13060_e17432);
        locals.var_cdbox_dn3 = (locals.var_t0_dn3 * assign13060_e17432);
        locals.var_cdbox_dn4 = (locals.var_t0_dn4 * assign13060_e17432);
        locals.var_cdbox_dn5 = (locals.var_t0_dn5 * assign13060_e17432);
        locals.var_cdbox_dn6 = (locals.var_t0_dn6 * assign13060_e17432);
        locals.var_cdbox_dn7 = (locals.var_t0_dn7 * assign13060_e17432);
        locals.var_cdbox_dn8 = (locals.var_t0_dn8 * assign13060_e17432);
        locals.var_cdbox_dn9 = (locals.var_t0_dn9 * assign13060_e17432);
        locals.var_cdbox_dn10 = (locals.var_t0_dn10 * assign13060_e17432);
        locals.var_cdbox_dn11 = (locals.var_t0_dn11 * assign13060_e17432);
        locals.var_cdbox_dn13 = (locals.var_t0_dn13 * assign13060_e17432);
        locals.var_cdbox_dn14 = (locals.var_t0_dn14 * assign13060_e17432);

        let assign13070_e17437: f64 = if p.p62 != 5.0 { 1.0 } else { 0.0 };
        locals.var_guard224 = assign13070_e17437;

        let (assign13080_e17451,) = {
    if (locals.var_guard224 != 0.0) {
        let assign13080_e17441: f64 = (p.p1544 * p.p59);
        let assign13080_e17443: f64 = (assign13080_e17441 * p.p6);
        let assign13080_e17446: f64 = (p.p1545 * locals.var_nfintotal);
        let assign13080_e17447: f64 = (assign13080_e17443 + assign13080_e17446);
        let assign13080_e17449: f64 = (assign13080_e17447 * locals.var_lg);
        (assign13080_e17449,)
    } else {
        (locals.var_cgbox,)
    }
};
        locals.var_cgbox = assign13080_e17451;

        let (assign13090_e17470,) = {
    if (locals.var_guard224 == 0.0) {
        let assign13090_e17456: f64 = (p.p1544 * p.p59);
        let assign13090_e17458: f64 = (assign13090_e17456 * p.p6);
        let assign13090_e17462: f64 = (p.p1546 * locals.var_wgaaeff);
        let assign13090_e17463: f64 = (p.p1545 + assign13090_e17462);
        let assign13090_e17465: f64 = (assign13090_e17463 * locals.var_nfintotal);
        let assign13090_e17466: f64 = (assign13090_e17458 + assign13090_e17465);
        let assign13090_e17468: f64 = (assign13090_e17466 * locals.var_lg);
        (assign13090_e17468,)
    } else {
        (locals.var_cgbox,)
    }
};
        locals.var_cgbox = assign13090_e17470;

        let assign13100_e17474: f64 = (locals.var_epsratio * p.p89);
        let assign13100_e17475: f64 = (1e-8 / assign13100_e17474);
        locals.var_eefffactor = assign13100_e17475;

        let assign13110_e17480: f64 = (locals.var_weff0 * 1000000.0);
        let assign13110_e17482: f64 = (assign13110_e17480).powf(locals.var_wr_i);
        let assign13110_e17483: f64 = (locals.var_nfintotal * assign13110_e17482);
        let assign13110_e17484: f64 = (1.0 / assign13110_e17483);
        locals.var_weffwrfactor = assign13110_e17484;

        let assign13120_e17487: f64 = (locals.var_epsratio * p.p89);
        let assign13120_e17489: f64 = (assign13120_e17487 * 0.5);
        let assign13120_e17491: f64 = (assign13120_e17489 * p.p3);
        let assign13120_e17492: f64 = (assign13120_e17491).sqrt();
        locals.var_litl = assign13120_e17492;

        let assign13130_e17495: f64 = (locals.var_epssub * locals.var_ach);
        let assign13130_e17497: f64 = (assign13130_e17495 / locals.var_cins);
        let assign13130_e17501: f64 = (locals.var_ach * locals.var_cins);
        let assign13130_e17504: f64 = (2.0 * locals.var_epssub);
        let assign13130_e17506: f64 = (assign13130_e17504 * locals.var_weff_ufcm);
        let assign13130_e17508: f64 = (assign13130_e17506 * locals.var_weff_ufcm);
        let assign13130_e17509: f64 = (assign13130_e17501 / assign13130_e17508);
        let assign13130_e17510: f64 = (1.0 + assign13130_e17509);
        let assign13130_e17511: f64 = (assign13130_e17497 * assign13130_e17510);
        let assign13130_e17512: f64 = (assign13130_e17511).sqrt();
        locals.var_scl = assign13130_e17512;

        let assign13140_e17515: f64 = if (!param_given[172]) { 1.0 } else { 0.0 };
        locals.var_guard225 = assign13140_e17515;

        let (assign13150_e17525, assign13150_e17525_d_n0, assign13150_e17525_d_n2, assign13150_e17525_d_n3, assign13150_e17525_d_n4, assign13150_e17525_d_n5, assign13150_e17525_d_n6, assign13150_e17525_d_n7, assign13150_e17525_d_n8, assign13150_e17525_d_n9, assign13150_e17525_d_n10, assign13150_e17525_d_n11, assign13150_e17525_d_n13, assign13150_e17525_d_n14,) = {
    if (locals.var_guard225 != 0.0) {
        let assign13150_e17519: f64 = (locals.var_dvt1_i * locals.var_leff_1);
        let assign13150_e17521: f64 = (assign13150_e17519 / locals.var_scl);
        let assign13150_e17523: f64 = (assign13150_e17521 + 1e-6);
        (assign13150_e17523, ((locals.var_dvt1_i * locals.var_leff_1_dn0) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn2) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn3) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn4) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn5) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn6) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn7) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn8) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn9) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn10) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn11) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn13) / locals.var_scl), ((locals.var_dvt1_i * locals.var_leff_1_dn14) / locals.var_scl),)
    } else {
        (locals.var_tmp, locals.var_tmp_dn0, locals.var_tmp_dn2, locals.var_tmp_dn3, locals.var_tmp_dn4, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, locals.var_tmp_dn9, locals.var_tmp_dn10, locals.var_tmp_dn11, locals.var_tmp_dn13, locals.var_tmp_dn14,)
    }
};
        locals.var_tmp = assign13150_e17525;
        locals.var_tmp_dn0 = assign13150_e17525_d_n0;
        locals.var_tmp_dn2 = assign13150_e17525_d_n2;
        locals.var_tmp_dn3 = assign13150_e17525_d_n3;
        locals.var_tmp_dn4 = assign13150_e17525_d_n4;
        locals.var_tmp_dn5 = assign13150_e17525_d_n5;
        locals.var_tmp_dn6 = assign13150_e17525_d_n6;
        locals.var_tmp_dn7 = assign13150_e17525_d_n7;
        locals.var_tmp_dn8 = assign13150_e17525_d_n8;
        locals.var_tmp_dn9 = assign13150_e17525_d_n9;
        locals.var_tmp_dn10 = assign13150_e17525_d_n10;
        locals.var_tmp_dn11 = assign13150_e17525_d_n11;
        locals.var_tmp_dn13 = assign13150_e17525_d_n13;
        locals.var_tmp_dn14 = assign13150_e17525_d_n14;

        let assign13160_e17528: f64 = if locals.var_tmp < 40.0 { 1.0 } else { 0.0 };
        locals.var_guard226 = assign13160_e17528;

        let (assign13170_e17539, assign13170_e17539_d_n0, assign13170_e17539_d_n2, assign13170_e17539_d_n3, assign13170_e17539_d_n4, assign13170_e17539_d_n5, assign13170_e17539_d_n6, assign13170_e17539_d_n7, assign13170_e17539_d_n8, assign13170_e17539_d_n9, assign13170_e17539_d_n10, assign13170_e17539_d_n11, assign13170_e17539_d_n13, assign13170_e17539_d_n14,) = {
    if ((locals.var_guard225 != 0.0) && (locals.var_guard226 != 0.0)) {
        let assign13170_e17534: f64 = (locals.var_tmp).cosh();
        let assign13170_e17536: f64 = (assign13170_e17534 - 1.0);
        let assign13170_e17537: f64 = (0.5 / assign13170_e17536);
        (assign13170_e17537, (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn0)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn2)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn3)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn4)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn5)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn6)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn7)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn8)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn9)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn10)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn11)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn13)) / (assign13170_e17536 * assign13170_e17536))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn14)) / (assign13170_e17536 * assign13170_e17536))),)
    } else {
        (locals.var_theta_sce, locals.var_theta_sce_dn0, locals.var_theta_sce_dn2, locals.var_theta_sce_dn3, locals.var_theta_sce_dn4, locals.var_theta_sce_dn5, locals.var_theta_sce_dn6, locals.var_theta_sce_dn7, locals.var_theta_sce_dn8, locals.var_theta_sce_dn9, locals.var_theta_sce_dn10, locals.var_theta_sce_dn11, locals.var_theta_sce_dn13, locals.var_theta_sce_dn14,)
    }
};
        locals.var_theta_sce = assign13170_e17539;
        locals.var_theta_sce_dn0 = assign13170_e17539_d_n0;
        locals.var_theta_sce_dn2 = assign13170_e17539_d_n2;
        locals.var_theta_sce_dn3 = assign13170_e17539_d_n3;
        locals.var_theta_sce_dn4 = assign13170_e17539_d_n4;
        locals.var_theta_sce_dn5 = assign13170_e17539_d_n5;
        locals.var_theta_sce_dn6 = assign13170_e17539_d_n6;
        locals.var_theta_sce_dn7 = assign13170_e17539_d_n7;
        locals.var_theta_sce_dn8 = assign13170_e17539_d_n8;
        locals.var_theta_sce_dn9 = assign13170_e17539_d_n9;
        locals.var_theta_sce_dn10 = assign13170_e17539_d_n10;
        locals.var_theta_sce_dn11 = assign13170_e17539_d_n11;
        locals.var_theta_sce_dn13 = assign13170_e17539_d_n13;
        locals.var_theta_sce_dn14 = assign13170_e17539_d_n14;

        let (assign13180_e17548, assign13180_e17548_d_n0, assign13180_e17548_d_n2, assign13180_e17548_d_n3, assign13180_e17548_d_n4, assign13180_e17548_d_n5, assign13180_e17548_d_n6, assign13180_e17548_d_n7, assign13180_e17548_d_n8, assign13180_e17548_d_n9, assign13180_e17548_d_n10, assign13180_e17548_d_n11, assign13180_e17548_d_n13, assign13180_e17548_d_n14,) = {
    if ((locals.var_guard225 != 0.0) && (locals.var_guard226 == 0.0)) {
        let assign13180_e17545: f64 = (-locals.var_tmp);
        let assign13180_e17546: f64 = { let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign13180_e17546, ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn0)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn2)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn3)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn4)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn5)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn6)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn7)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn8)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn9)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn10)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn11)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn13)), ({ let limited_exp_arg = assign13180_e17545; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn14)),)
    } else {
        (locals.var_theta_sce, locals.var_theta_sce_dn0, locals.var_theta_sce_dn2, locals.var_theta_sce_dn3, locals.var_theta_sce_dn4, locals.var_theta_sce_dn5, locals.var_theta_sce_dn6, locals.var_theta_sce_dn7, locals.var_theta_sce_dn8, locals.var_theta_sce_dn9, locals.var_theta_sce_dn10, locals.var_theta_sce_dn11, locals.var_theta_sce_dn13, locals.var_theta_sce_dn14,)
    }
};
        locals.var_theta_sce = assign13180_e17548;
        locals.var_theta_sce_dn0 = assign13180_e17548_d_n0;
        locals.var_theta_sce_dn2 = assign13180_e17548_d_n2;
        locals.var_theta_sce_dn3 = assign13180_e17548_d_n3;
        locals.var_theta_sce_dn4 = assign13180_e17548_d_n4;
        locals.var_theta_sce_dn5 = assign13180_e17548_d_n5;
        locals.var_theta_sce_dn6 = assign13180_e17548_d_n6;
        locals.var_theta_sce_dn7 = assign13180_e17548_d_n7;
        locals.var_theta_sce_dn8 = assign13180_e17548_d_n8;
        locals.var_theta_sce_dn9 = assign13180_e17548_d_n9;
        locals.var_theta_sce_dn10 = assign13180_e17548_d_n10;
        locals.var_theta_sce_dn11 = assign13180_e17548_d_n11;
        locals.var_theta_sce_dn13 = assign13180_e17548_d_n13;
        locals.var_theta_sce_dn14 = assign13180_e17548_d_n14;

        let (assign13190_e17553, assign13190_e17553_d_n0, assign13190_e17553_d_n2, assign13190_e17553_d_n3, assign13190_e17553_d_n4, assign13190_e17553_d_n5, assign13190_e17553_d_n6, assign13190_e17553_d_n7, assign13190_e17553_d_n8, assign13190_e17553_d_n9, assign13190_e17553_d_n10, assign13190_e17553_d_n11, assign13190_e17553_d_n13, assign13190_e17553_d_n14,) = {
    if (locals.var_guard225 == 0.0) {
        (p.p172, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_theta_sce, locals.var_theta_sce_dn0, locals.var_theta_sce_dn2, locals.var_theta_sce_dn3, locals.var_theta_sce_dn4, locals.var_theta_sce_dn5, locals.var_theta_sce_dn6, locals.var_theta_sce_dn7, locals.var_theta_sce_dn8, locals.var_theta_sce_dn9, locals.var_theta_sce_dn10, locals.var_theta_sce_dn11, locals.var_theta_sce_dn13, locals.var_theta_sce_dn14,)
    }
};
        locals.var_theta_sce = assign13190_e17553;
        locals.var_theta_sce_dn0 = assign13190_e17553_d_n0;
        locals.var_theta_sce_dn2 = assign13190_e17553_d_n2;
        locals.var_theta_sce_dn3 = assign13190_e17553_d_n3;
        locals.var_theta_sce_dn4 = assign13190_e17553_d_n4;
        locals.var_theta_sce_dn5 = assign13190_e17553_d_n5;
        locals.var_theta_sce_dn6 = assign13190_e17553_d_n6;
        locals.var_theta_sce_dn7 = assign13190_e17553_d_n7;
        locals.var_theta_sce_dn8 = assign13190_e17553_d_n8;
        locals.var_theta_sce_dn9 = assign13190_e17553_d_n9;
        locals.var_theta_sce_dn10 = assign13190_e17553_d_n10;
        locals.var_theta_sce_dn11 = assign13190_e17553_d_n11;
        locals.var_theta_sce_dn13 = assign13190_e17553_d_n13;
        locals.var_theta_sce_dn14 = assign13190_e17553_d_n14;

        let assign13200_e17556: f64 = if (!param_given[174]) { 1.0 } else { 0.0 };
        locals.var_guard227 = assign13200_e17556;

        let (assign13210_e17566, assign13210_e17566_d_n0, assign13210_e17566_d_n2, assign13210_e17566_d_n3, assign13210_e17566_d_n4, assign13210_e17566_d_n5, assign13210_e17566_d_n6, assign13210_e17566_d_n7, assign13210_e17566_d_n8, assign13210_e17566_d_n9, assign13210_e17566_d_n10, assign13210_e17566_d_n11, assign13210_e17566_d_n13, assign13210_e17566_d_n14,) = {
    if (locals.var_guard227 != 0.0) {
        let assign13210_e17560: f64 = (locals.var_dvt1ss_i * locals.var_leff_1);
        let assign13210_e17562: f64 = (assign13210_e17560 / locals.var_scl);
        let assign13210_e17564: f64 = (assign13210_e17562 + 1e-6);
        (assign13210_e17564, ((locals.var_dvt1ss_i * locals.var_leff_1_dn0) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn2) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn3) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn4) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn5) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn6) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn7) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn8) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn9) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn10) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn11) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn13) / locals.var_scl), ((locals.var_dvt1ss_i * locals.var_leff_1_dn14) / locals.var_scl),)
    } else {
        (locals.var_tmp, locals.var_tmp_dn0, locals.var_tmp_dn2, locals.var_tmp_dn3, locals.var_tmp_dn4, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, locals.var_tmp_dn9, locals.var_tmp_dn10, locals.var_tmp_dn11, locals.var_tmp_dn13, locals.var_tmp_dn14,)
    }
};
        locals.var_tmp = assign13210_e17566;
        locals.var_tmp_dn0 = assign13210_e17566_d_n0;
        locals.var_tmp_dn2 = assign13210_e17566_d_n2;
        locals.var_tmp_dn3 = assign13210_e17566_d_n3;
        locals.var_tmp_dn4 = assign13210_e17566_d_n4;
        locals.var_tmp_dn5 = assign13210_e17566_d_n5;
        locals.var_tmp_dn6 = assign13210_e17566_d_n6;
        locals.var_tmp_dn7 = assign13210_e17566_d_n7;
        locals.var_tmp_dn8 = assign13210_e17566_d_n8;
        locals.var_tmp_dn9 = assign13210_e17566_d_n9;
        locals.var_tmp_dn10 = assign13210_e17566_d_n10;
        locals.var_tmp_dn11 = assign13210_e17566_d_n11;
        locals.var_tmp_dn13 = assign13210_e17566_d_n13;
        locals.var_tmp_dn14 = assign13210_e17566_d_n14;

        let assign13220_e17569: f64 = if locals.var_tmp < 40.0 { 1.0 } else { 0.0 };
        locals.var_guard228 = assign13220_e17569;

        let (assign13230_e17580, assign13230_e17580_d_n0, assign13230_e17580_d_n2, assign13230_e17580_d_n3, assign13230_e17580_d_n4, assign13230_e17580_d_n5, assign13230_e17580_d_n6, assign13230_e17580_d_n7, assign13230_e17580_d_n8, assign13230_e17580_d_n9, assign13230_e17580_d_n10, assign13230_e17580_d_n11, assign13230_e17580_d_n13, assign13230_e17580_d_n14,) = {
    if ((locals.var_guard227 != 0.0) && (locals.var_guard228 != 0.0)) {
        let assign13230_e17575: f64 = (locals.var_tmp).cosh();
        let assign13230_e17577: f64 = (assign13230_e17575 - 1.0);
        let assign13230_e17578: f64 = (0.5 / assign13230_e17577);
        (assign13230_e17578, (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn0)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn2)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn3)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn4)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn5)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn6)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn7)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn8)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn9)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn10)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn11)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn13)) / (assign13230_e17577 * assign13230_e17577))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn14)) / (assign13230_e17577 * assign13230_e17577))),)
    } else {
        (locals.var_theta_sw, locals.var_theta_sw_dn0, locals.var_theta_sw_dn2, locals.var_theta_sw_dn3, locals.var_theta_sw_dn4, locals.var_theta_sw_dn5, locals.var_theta_sw_dn6, locals.var_theta_sw_dn7, locals.var_theta_sw_dn8, locals.var_theta_sw_dn9, locals.var_theta_sw_dn10, locals.var_theta_sw_dn11, locals.var_theta_sw_dn13, locals.var_theta_sw_dn14,)
    }
};
        locals.var_theta_sw = assign13230_e17580;
        locals.var_theta_sw_dn0 = assign13230_e17580_d_n0;
        locals.var_theta_sw_dn2 = assign13230_e17580_d_n2;
        locals.var_theta_sw_dn3 = assign13230_e17580_d_n3;
        locals.var_theta_sw_dn4 = assign13230_e17580_d_n4;
        locals.var_theta_sw_dn5 = assign13230_e17580_d_n5;
        locals.var_theta_sw_dn6 = assign13230_e17580_d_n6;
        locals.var_theta_sw_dn7 = assign13230_e17580_d_n7;
        locals.var_theta_sw_dn8 = assign13230_e17580_d_n8;
        locals.var_theta_sw_dn9 = assign13230_e17580_d_n9;
        locals.var_theta_sw_dn10 = assign13230_e17580_d_n10;
        locals.var_theta_sw_dn11 = assign13230_e17580_d_n11;
        locals.var_theta_sw_dn13 = assign13230_e17580_d_n13;
        locals.var_theta_sw_dn14 = assign13230_e17580_d_n14;

        let (assign13240_e17589, assign13240_e17589_d_n0, assign13240_e17589_d_n2, assign13240_e17589_d_n3, assign13240_e17589_d_n4, assign13240_e17589_d_n5, assign13240_e17589_d_n6, assign13240_e17589_d_n7, assign13240_e17589_d_n8, assign13240_e17589_d_n9, assign13240_e17589_d_n10, assign13240_e17589_d_n11, assign13240_e17589_d_n13, assign13240_e17589_d_n14,) = {
    if ((locals.var_guard227 != 0.0) && (locals.var_guard228 == 0.0)) {
        let assign13240_e17586: f64 = (-locals.var_tmp);
        let assign13240_e17587: f64 = { let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign13240_e17587, ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn0)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn2)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn3)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn4)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn5)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn6)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn7)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn8)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn9)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn10)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn11)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn13)), ({ let limited_exp_arg = assign13240_e17586; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn14)),)
    } else {
        (locals.var_theta_sw, locals.var_theta_sw_dn0, locals.var_theta_sw_dn2, locals.var_theta_sw_dn3, locals.var_theta_sw_dn4, locals.var_theta_sw_dn5, locals.var_theta_sw_dn6, locals.var_theta_sw_dn7, locals.var_theta_sw_dn8, locals.var_theta_sw_dn9, locals.var_theta_sw_dn10, locals.var_theta_sw_dn11, locals.var_theta_sw_dn13, locals.var_theta_sw_dn14,)
    }
};
        locals.var_theta_sw = assign13240_e17589;
        locals.var_theta_sw_dn0 = assign13240_e17589_d_n0;
        locals.var_theta_sw_dn2 = assign13240_e17589_d_n2;
        locals.var_theta_sw_dn3 = assign13240_e17589_d_n3;
        locals.var_theta_sw_dn4 = assign13240_e17589_d_n4;
        locals.var_theta_sw_dn5 = assign13240_e17589_d_n5;
        locals.var_theta_sw_dn6 = assign13240_e17589_d_n6;
        locals.var_theta_sw_dn7 = assign13240_e17589_d_n7;
        locals.var_theta_sw_dn8 = assign13240_e17589_d_n8;
        locals.var_theta_sw_dn9 = assign13240_e17589_d_n9;
        locals.var_theta_sw_dn10 = assign13240_e17589_d_n10;
        locals.var_theta_sw_dn11 = assign13240_e17589_d_n11;
        locals.var_theta_sw_dn13 = assign13240_e17589_d_n13;
        locals.var_theta_sw_dn14 = assign13240_e17589_d_n14;

        let (assign13250_e17594, assign13250_e17594_d_n0, assign13250_e17594_d_n2, assign13250_e17594_d_n3, assign13250_e17594_d_n4, assign13250_e17594_d_n5, assign13250_e17594_d_n6, assign13250_e17594_d_n7, assign13250_e17594_d_n8, assign13250_e17594_d_n9, assign13250_e17594_d_n10, assign13250_e17594_d_n11, assign13250_e17594_d_n13, assign13250_e17594_d_n14,) = {
    if (locals.var_guard227 == 0.0) {
        (p.p174, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_theta_sw, locals.var_theta_sw_dn0, locals.var_theta_sw_dn2, locals.var_theta_sw_dn3, locals.var_theta_sw_dn4, locals.var_theta_sw_dn5, locals.var_theta_sw_dn6, locals.var_theta_sw_dn7, locals.var_theta_sw_dn8, locals.var_theta_sw_dn9, locals.var_theta_sw_dn10, locals.var_theta_sw_dn11, locals.var_theta_sw_dn13, locals.var_theta_sw_dn14,)
    }
};
        locals.var_theta_sw = assign13250_e17594;
        locals.var_theta_sw_dn0 = assign13250_e17594_d_n0;
        locals.var_theta_sw_dn2 = assign13250_e17594_d_n2;
        locals.var_theta_sw_dn3 = assign13250_e17594_d_n3;
        locals.var_theta_sw_dn4 = assign13250_e17594_d_n4;
        locals.var_theta_sw_dn5 = assign13250_e17594_d_n5;
        locals.var_theta_sw_dn6 = assign13250_e17594_d_n6;
        locals.var_theta_sw_dn7 = assign13250_e17594_d_n7;
        locals.var_theta_sw_dn8 = assign13250_e17594_d_n8;
        locals.var_theta_sw_dn9 = assign13250_e17594_d_n9;
        locals.var_theta_sw_dn10 = assign13250_e17594_d_n10;
        locals.var_theta_sw_dn11 = assign13250_e17594_d_n11;
        locals.var_theta_sw_dn13 = assign13250_e17594_d_n13;
        locals.var_theta_sw_dn14 = assign13250_e17594_d_n14;

        let assign13260_e17597: f64 = if (!param_given[173]) { 1.0 } else { 0.0 };
        locals.var_guard229 = assign13260_e17597;

        let (assign13270_e17607, assign13270_e17607_d_n0, assign13270_e17607_d_n2, assign13270_e17607_d_n3, assign13270_e17607_d_n4, assign13270_e17607_d_n5, assign13270_e17607_d_n6, assign13270_e17607_d_n7, assign13270_e17607_d_n8, assign13270_e17607_d_n9, assign13270_e17607_d_n10, assign13270_e17607_d_n11, assign13270_e17607_d_n13, assign13270_e17607_d_n14,) = {
    if (locals.var_guard229 != 0.0) {
        let assign13270_e17601: f64 = (locals.var_dsub_i * locals.var_leff_1);
        let assign13270_e17603: f64 = (assign13270_e17601 / locals.var_scl);
        let assign13270_e17605: f64 = (assign13270_e17603 + 1e-6);
        (assign13270_e17605, ((locals.var_dsub_i * locals.var_leff_1_dn0) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn2) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn3) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn4) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn5) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn6) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn7) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn8) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn9) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn10) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn11) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn13) / locals.var_scl), ((locals.var_dsub_i * locals.var_leff_1_dn14) / locals.var_scl),)
    } else {
        (locals.var_tmp, locals.var_tmp_dn0, locals.var_tmp_dn2, locals.var_tmp_dn3, locals.var_tmp_dn4, locals.var_tmp_dn5, locals.var_tmp_dn6, locals.var_tmp_dn7, locals.var_tmp_dn8, locals.var_tmp_dn9, locals.var_tmp_dn10, locals.var_tmp_dn11, locals.var_tmp_dn13, locals.var_tmp_dn14,)
    }
};
        locals.var_tmp = assign13270_e17607;
        locals.var_tmp_dn0 = assign13270_e17607_d_n0;
        locals.var_tmp_dn2 = assign13270_e17607_d_n2;
        locals.var_tmp_dn3 = assign13270_e17607_d_n3;
        locals.var_tmp_dn4 = assign13270_e17607_d_n4;
        locals.var_tmp_dn5 = assign13270_e17607_d_n5;
        locals.var_tmp_dn6 = assign13270_e17607_d_n6;
        locals.var_tmp_dn7 = assign13270_e17607_d_n7;
        locals.var_tmp_dn8 = assign13270_e17607_d_n8;
        locals.var_tmp_dn9 = assign13270_e17607_d_n9;
        locals.var_tmp_dn10 = assign13270_e17607_d_n10;
        locals.var_tmp_dn11 = assign13270_e17607_d_n11;
        locals.var_tmp_dn13 = assign13270_e17607_d_n13;
        locals.var_tmp_dn14 = assign13270_e17607_d_n14;

        let assign13280_e17610: f64 = if locals.var_tmp < 40.0 { 1.0 } else { 0.0 };
        locals.var_guard230 = assign13280_e17610;

        let (assign13290_e17621, assign13290_e17621_d_n0, assign13290_e17621_d_n2, assign13290_e17621_d_n3, assign13290_e17621_d_n4, assign13290_e17621_d_n5, assign13290_e17621_d_n6, assign13290_e17621_d_n7, assign13290_e17621_d_n8, assign13290_e17621_d_n9, assign13290_e17621_d_n10, assign13290_e17621_d_n11, assign13290_e17621_d_n13, assign13290_e17621_d_n14,) = {
    if ((locals.var_guard229 != 0.0) && (locals.var_guard230 != 0.0)) {
        let assign13290_e17616: f64 = (locals.var_tmp).cosh();
        let assign13290_e17618: f64 = (assign13290_e17616 - 1.0);
        let assign13290_e17619: f64 = (0.5 / assign13290_e17618);
        (assign13290_e17619, (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn0)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn2)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn3)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn4)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn5)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn6)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn7)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn8)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn9)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn10)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn11)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn13)) / (assign13290_e17618 * assign13290_e17618))), (-((0.5 * ((locals.var_tmp).sinh() * locals.var_tmp_dn14)) / (assign13290_e17618 * assign13290_e17618))),)
    } else {
        (locals.var_theta_dibl, locals.var_theta_dibl_dn0, locals.var_theta_dibl_dn2, locals.var_theta_dibl_dn3, locals.var_theta_dibl_dn4, locals.var_theta_dibl_dn5, locals.var_theta_dibl_dn6, locals.var_theta_dibl_dn7, locals.var_theta_dibl_dn8, locals.var_theta_dibl_dn9, locals.var_theta_dibl_dn10, locals.var_theta_dibl_dn11, locals.var_theta_dibl_dn13, locals.var_theta_dibl_dn14,)
    }
};
        locals.var_theta_dibl = assign13290_e17621;
        locals.var_theta_dibl_dn0 = assign13290_e17621_d_n0;
        locals.var_theta_dibl_dn2 = assign13290_e17621_d_n2;
        locals.var_theta_dibl_dn3 = assign13290_e17621_d_n3;
        locals.var_theta_dibl_dn4 = assign13290_e17621_d_n4;
        locals.var_theta_dibl_dn5 = assign13290_e17621_d_n5;
        locals.var_theta_dibl_dn6 = assign13290_e17621_d_n6;
        locals.var_theta_dibl_dn7 = assign13290_e17621_d_n7;
        locals.var_theta_dibl_dn8 = assign13290_e17621_d_n8;
        locals.var_theta_dibl_dn9 = assign13290_e17621_d_n9;
        locals.var_theta_dibl_dn10 = assign13290_e17621_d_n10;
        locals.var_theta_dibl_dn11 = assign13290_e17621_d_n11;
        locals.var_theta_dibl_dn13 = assign13290_e17621_d_n13;
        locals.var_theta_dibl_dn14 = assign13290_e17621_d_n14;

    }

    pub(super) fn stamp_transient_block_34(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13300_e17630, assign13300_e17630_d_n0, assign13300_e17630_d_n2, assign13300_e17630_d_n3, assign13300_e17630_d_n4, assign13300_e17630_d_n5, assign13300_e17630_d_n6, assign13300_e17630_d_n7, assign13300_e17630_d_n8, assign13300_e17630_d_n9, assign13300_e17630_d_n10, assign13300_e17630_d_n11, assign13300_e17630_d_n13, assign13300_e17630_d_n14,) = {
    if ((locals.var_guard229 != 0.0) && (locals.var_guard230 == 0.0)) {
        let assign13300_e17627: f64 = (-locals.var_tmp);
        let assign13300_e17628: f64 = { let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        (assign13300_e17628, ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn0)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn2)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn3)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn4)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn5)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn6)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn7)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn8)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn9)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn10)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn11)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn13)), ({ let limited_exp_arg = assign13300_e17627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn14)),)
    } else {
        (locals.var_theta_dibl, locals.var_theta_dibl_dn0, locals.var_theta_dibl_dn2, locals.var_theta_dibl_dn3, locals.var_theta_dibl_dn4, locals.var_theta_dibl_dn5, locals.var_theta_dibl_dn6, locals.var_theta_dibl_dn7, locals.var_theta_dibl_dn8, locals.var_theta_dibl_dn9, locals.var_theta_dibl_dn10, locals.var_theta_dibl_dn11, locals.var_theta_dibl_dn13, locals.var_theta_dibl_dn14,)
    }
};
        locals.var_theta_dibl = assign13300_e17630;
        locals.var_theta_dibl_dn0 = assign13300_e17630_d_n0;
        locals.var_theta_dibl_dn2 = assign13300_e17630_d_n2;
        locals.var_theta_dibl_dn3 = assign13300_e17630_d_n3;
        locals.var_theta_dibl_dn4 = assign13300_e17630_d_n4;
        locals.var_theta_dibl_dn5 = assign13300_e17630_d_n5;
        locals.var_theta_dibl_dn6 = assign13300_e17630_d_n6;
        locals.var_theta_dibl_dn7 = assign13300_e17630_d_n7;
        locals.var_theta_dibl_dn8 = assign13300_e17630_d_n8;
        locals.var_theta_dibl_dn9 = assign13300_e17630_d_n9;
        locals.var_theta_dibl_dn10 = assign13300_e17630_d_n10;
        locals.var_theta_dibl_dn11 = assign13300_e17630_d_n11;
        locals.var_theta_dibl_dn13 = assign13300_e17630_d_n13;
        locals.var_theta_dibl_dn14 = assign13300_e17630_d_n14;

        let (assign13310_e17635, assign13310_e17635_d_n0, assign13310_e17635_d_n2, assign13310_e17635_d_n3, assign13310_e17635_d_n4, assign13310_e17635_d_n5, assign13310_e17635_d_n6, assign13310_e17635_d_n7, assign13310_e17635_d_n8, assign13310_e17635_d_n9, assign13310_e17635_d_n10, assign13310_e17635_d_n11, assign13310_e17635_d_n13, assign13310_e17635_d_n14,) = {
    if (locals.var_guard229 == 0.0) {
        (p.p173, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_theta_dibl, locals.var_theta_dibl_dn0, locals.var_theta_dibl_dn2, locals.var_theta_dibl_dn3, locals.var_theta_dibl_dn4, locals.var_theta_dibl_dn5, locals.var_theta_dibl_dn6, locals.var_theta_dibl_dn7, locals.var_theta_dibl_dn8, locals.var_theta_dibl_dn9, locals.var_theta_dibl_dn10, locals.var_theta_dibl_dn11, locals.var_theta_dibl_dn13, locals.var_theta_dibl_dn14,)
    }
};
        locals.var_theta_dibl = assign13310_e17635;
        locals.var_theta_dibl_dn0 = assign13310_e17635_d_n0;
        locals.var_theta_dibl_dn2 = assign13310_e17635_d_n2;
        locals.var_theta_dibl_dn3 = assign13310_e17635_d_n3;
        locals.var_theta_dibl_dn4 = assign13310_e17635_d_n4;
        locals.var_theta_dibl_dn5 = assign13310_e17635_d_n5;
        locals.var_theta_dibl_dn6 = assign13310_e17635_d_n6;
        locals.var_theta_dibl_dn7 = assign13310_e17635_d_n7;
        locals.var_theta_dibl_dn8 = assign13310_e17635_d_n8;
        locals.var_theta_dibl_dn9 = assign13310_e17635_d_n9;
        locals.var_theta_dibl_dn10 = assign13310_e17635_d_n10;
        locals.var_theta_dibl_dn11 = assign13310_e17635_d_n11;
        locals.var_theta_dibl_dn13 = assign13310_e17635_d_n13;
        locals.var_theta_dibl_dn14 = assign13310_e17635_d_n14;

        let assign13320_e17639: f64 = (locals.var_lpe0_i / locals.var_leff_1);
        let assign13320_e17640: f64 = (1.0 + assign13320_e17639);
        let assign13320_e17641: f64 = (assign13320_e17640).sqrt();
        let assign13320_e17643: f64 = (assign13320_e17641 - 1.0);
        locals.var_theta_rsce = assign13320_e17643;
        locals.var_theta_rsce_dn0 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn0) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn2 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn2) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn3 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn3) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn4 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn4) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn5 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn5) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn6 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn6) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn7 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn7) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn8 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn8) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn9 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn9) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn10 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn10) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn11 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn11) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn13 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn13) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));
        locals.var_theta_rsce_dn14 = ((-((locals.var_lpe0_i * locals.var_leff_1_dn14) / (locals.var_leff_1 * locals.var_leff_1))) / (2.0 * assign13320_e17641));

        let assign13330_e17646: f64 = (locals.var_dsub_i * locals.var_leff_1);
        let assign13330_e17648: f64 = (assign13330_e17646 / locals.var_scl);
        let assign13330_e17650: f64 = (assign13330_e17648 + 1e-6);
        locals.var_tmp = assign13330_e17650;
        locals.var_tmp_dn0 = ((locals.var_dsub_i * locals.var_leff_1_dn0) / locals.var_scl);
        locals.var_tmp_dn2 = ((locals.var_dsub_i * locals.var_leff_1_dn2) / locals.var_scl);
        locals.var_tmp_dn3 = ((locals.var_dsub_i * locals.var_leff_1_dn3) / locals.var_scl);
        locals.var_tmp_dn4 = ((locals.var_dsub_i * locals.var_leff_1_dn4) / locals.var_scl);
        locals.var_tmp_dn5 = ((locals.var_dsub_i * locals.var_leff_1_dn5) / locals.var_scl);
        locals.var_tmp_dn6 = ((locals.var_dsub_i * locals.var_leff_1_dn6) / locals.var_scl);
        locals.var_tmp_dn7 = ((locals.var_dsub_i * locals.var_leff_1_dn7) / locals.var_scl);
        locals.var_tmp_dn8 = ((locals.var_dsub_i * locals.var_leff_1_dn8) / locals.var_scl);
        locals.var_tmp_dn9 = ((locals.var_dsub_i * locals.var_leff_1_dn9) / locals.var_scl);
        locals.var_tmp_dn10 = ((locals.var_dsub_i * locals.var_leff_1_dn10) / locals.var_scl);
        locals.var_tmp_dn11 = ((locals.var_dsub_i * locals.var_leff_1_dn11) / locals.var_scl);
        locals.var_tmp_dn13 = ((locals.var_dsub_i * locals.var_leff_1_dn13) / locals.var_scl);
        locals.var_tmp_dn14 = ((locals.var_dsub_i * locals.var_leff_1_dn14) / locals.var_scl);

        let assign13340_e17653: f64 = if locals.var_tmp < 40.0 { 1.0 } else { 0.0 };
        locals.var_guard231 = assign13340_e17653;

        let (assign13350_e17668, assign13350_e17668_d_n0, assign13350_e17668_d_n2, assign13350_e17668_d_n3, assign13350_e17668_d_n4, assign13350_e17668_d_n5, assign13350_e17668_d_n6, assign13350_e17668_d_n7, assign13350_e17668_d_n8, assign13350_e17668_d_n9, assign13350_e17668_d_n10, assign13350_e17668_d_n11, assign13350_e17668_d_n13, assign13350_e17668_d_n14,) = {
    if (locals.var_guard231 != 0.0) {
        let assign13350_e17659: f64 = (locals.var_tmp).cosh();
        let assign13350_e17661: f64 = (assign13350_e17659 - 2.0);
        let assign13350_e17662: f64 = (p.p171 * assign13350_e17661);
        let assign13350_e17663: f64 = (1.0 + assign13350_e17662);
        let assign13350_e17665: f64 = (assign13350_e17663).max(1e-6);
        let assign13350_e17666: f64 = (1.0 / assign13350_e17665);
        (assign13350_e17666, (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn0)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn2)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn3)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn4)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn5)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn6)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn7)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn8)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn9)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn10)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn11)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn13)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))), (-(if assign13350_e17663 >= 1e-6 { (p.p171 * ((locals.var_tmp).sinh() * locals.var_tmp_dn14)) } else { 0.0 } / (assign13350_e17665 * assign13350_e17665))),)
    } else {
        (locals.var_theta_dits, locals.var_theta_dits_dn0, locals.var_theta_dits_dn2, locals.var_theta_dits_dn3, locals.var_theta_dits_dn4, locals.var_theta_dits_dn5, locals.var_theta_dits_dn6, locals.var_theta_dits_dn7, locals.var_theta_dits_dn8, locals.var_theta_dits_dn9, locals.var_theta_dits_dn10, locals.var_theta_dits_dn11, locals.var_theta_dits_dn13, locals.var_theta_dits_dn14,)
    }
};
        locals.var_theta_dits = assign13350_e17668;
        locals.var_theta_dits_dn0 = assign13350_e17668_d_n0;
        locals.var_theta_dits_dn2 = assign13350_e17668_d_n2;
        locals.var_theta_dits_dn3 = assign13350_e17668_d_n3;
        locals.var_theta_dits_dn4 = assign13350_e17668_d_n4;
        locals.var_theta_dits_dn5 = assign13350_e17668_d_n5;
        locals.var_theta_dits_dn6 = assign13350_e17668_d_n6;
        locals.var_theta_dits_dn7 = assign13350_e17668_d_n7;
        locals.var_theta_dits_dn8 = assign13350_e17668_d_n8;
        locals.var_theta_dits_dn9 = assign13350_e17668_d_n9;
        locals.var_theta_dits_dn10 = assign13350_e17668_d_n10;
        locals.var_theta_dits_dn11 = assign13350_e17668_d_n11;
        locals.var_theta_dits_dn13 = assign13350_e17668_d_n13;
        locals.var_theta_dits_dn14 = assign13350_e17668_d_n14;

        let (assign13360_e17683, assign13360_e17683_d_n0, assign13360_e17683_d_n2, assign13360_e17683_d_n3, assign13360_e17683_d_n4, assign13360_e17683_d_n5, assign13360_e17683_d_n6, assign13360_e17683_d_n7, assign13360_e17683_d_n8, assign13360_e17683_d_n9, assign13360_e17683_d_n10, assign13360_e17683_d_n11, assign13360_e17683_d_n13, assign13360_e17683_d_n14,) = {
    if (locals.var_guard231 == 0.0) {
        let assign13360_e17672: f64 = (-locals.var_tmp);
        let assign13360_e17673: f64 = { let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13360_e17676: f64 = (-locals.var_tmp);
        let assign13360_e17677: f64 = { let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13360_e17678: f64 = (p.p171 + assign13360_e17677);
        let assign13360_e17680: f64 = (assign13360_e17678).max(1e-6);
        let assign13360_e17681: f64 = (assign13360_e17673 / assign13360_e17680);
        (assign13360_e17681, (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn0)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn0)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn2)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn2)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn3)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn3)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn4)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn4)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn5)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn5)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn6)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn6)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn7)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn7)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn8)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn8)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn9)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn9)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn10)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn10)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn11)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn11)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn13)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn13)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)), (((({ let limited_exp_arg = assign13360_e17672; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn14)) * assign13360_e17680) - (assign13360_e17673 * if assign13360_e17678 >= 1e-6 { ({ let limited_exp_arg = assign13360_e17676; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-locals.var_tmp_dn14)) } else { 0.0 })) / (assign13360_e17680 * assign13360_e17680)),)
    } else {
        (locals.var_theta_dits, locals.var_theta_dits_dn0, locals.var_theta_dits_dn2, locals.var_theta_dits_dn3, locals.var_theta_dits_dn4, locals.var_theta_dits_dn5, locals.var_theta_dits_dn6, locals.var_theta_dits_dn7, locals.var_theta_dits_dn8, locals.var_theta_dits_dn9, locals.var_theta_dits_dn10, locals.var_theta_dits_dn11, locals.var_theta_dits_dn13, locals.var_theta_dits_dn14,)
    }
};
        locals.var_theta_dits = assign13360_e17683;
        locals.var_theta_dits_dn0 = assign13360_e17683_d_n0;
        locals.var_theta_dits_dn2 = assign13360_e17683_d_n2;
        locals.var_theta_dits_dn3 = assign13360_e17683_d_n3;
        locals.var_theta_dits_dn4 = assign13360_e17683_d_n4;
        locals.var_theta_dits_dn5 = assign13360_e17683_d_n5;
        locals.var_theta_dits_dn6 = assign13360_e17683_d_n6;
        locals.var_theta_dits_dn7 = assign13360_e17683_d_n7;
        locals.var_theta_dits_dn8 = assign13360_e17683_d_n8;
        locals.var_theta_dits_dn9 = assign13360_e17683_d_n9;
        locals.var_theta_dits_dn10 = assign13360_e17683_d_n10;
        locals.var_theta_dits_dn11 = assign13360_e17683_d_n11;
        locals.var_theta_dits_dn13 = assign13360_e17683_d_n13;
        locals.var_theta_dits_dn14 = assign13360_e17683_d_n14;

        let assign13370_e17686: f64 = (1.60219e-19 * locals.var_nbody_i);
        let assign13370_e17688: f64 = (assign13370_e17686 * locals.var_ach);
        let assign13370_e17690: f64 = (assign13370_e17688 / locals.var_cins);
        locals.var_qbs = assign13370_e17690;

        let assign13380_e17693: f64 = if p.p60 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard232 = assign13380_e17693;

        let (assign13390_e17697,) = {
    if (locals.var_guard232 != 0.0) {
        (4.97232e-7,)
    } else {
        (locals.var_aechvb,)
    }
};
        locals.var_aechvb = assign13390_e17697;

        let (assign13400_e17701,) = {
    if (locals.var_guard232 != 0.0) {
        (745669000000.0,)
    } else {
        (locals.var_bechvb,)
    }
};
        locals.var_bechvb = assign13400_e17701;

        let (assign13410_e17706,) = {
    if (locals.var_guard232 == 0.0) {
        (3.42537e-7,)
    } else {
        (locals.var_aechvb,)
    }
};
        locals.var_aechvb = assign13410_e17706;

        let (assign13420_e17711,) = {
    if (locals.var_guard232 == 0.0) {
        (1166450000000.0,)
    } else {
        (locals.var_bechvb,)
    }
};
        locals.var_bechvb = assign13420_e17711;

        let assign13430_e17714: f64 = (p.p1109 * p.p1109);
        locals.var_t0 = assign13430_e17714;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let assign13440_e17717: f64 = (p.p1109 * locals.var_poxedge_i);
        locals.var_t1 = assign13440_e17717;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;

        let assign13450_e17720: f64 = (locals.var_t1 * locals.var_t1);
        locals.var_t2 = assign13450_e17720;
        locals.var_t2_dn0 = ((locals.var_t1_dn0 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn0));
        locals.var_t2_dn2 = ((locals.var_t1_dn2 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn2));
        locals.var_t2_dn3 = ((locals.var_t1_dn3 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn3));
        locals.var_t2_dn4 = ((locals.var_t1_dn4 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn4));
        locals.var_t2_dn5 = ((locals.var_t1_dn5 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn5));
        locals.var_t2_dn6 = ((locals.var_t1_dn6 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn6));
        locals.var_t2_dn7 = ((locals.var_t1_dn7 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn7));
        locals.var_t2_dn8 = ((locals.var_t1_dn8 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn8));
        locals.var_t2_dn9 = ((locals.var_t1_dn9 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn9));
        locals.var_t2_dn10 = ((locals.var_t1_dn10 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn10));
        locals.var_t2_dn11 = ((locals.var_t1_dn11 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn11));
        locals.var_t2_dn13 = ((locals.var_t1_dn13 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn13));
        locals.var_t2_dn14 = ((locals.var_t1_dn14 * locals.var_t1) + (locals.var_t1 * locals.var_t1_dn14));

        let assign13460_e17723: f64 = (p.p1108 / p.p1109);
        let assign13460_e17725: f64 = (assign13460_e17723).powf(locals.var_ntox_i);
        let assign13460_e17727: f64 = (assign13460_e17725 / locals.var_t0);
        locals.var_toxratio = assign13460_e17727;
        locals.var_toxratio_dn0 = (-((assign13460_e17725 * locals.var_t0_dn0) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn2 = (-((assign13460_e17725 * locals.var_t0_dn2) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn3 = (-((assign13460_e17725 * locals.var_t0_dn3) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn4 = (-((assign13460_e17725 * locals.var_t0_dn4) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn5 = (-((assign13460_e17725 * locals.var_t0_dn5) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn6 = (-((assign13460_e17725 * locals.var_t0_dn6) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn7 = (-((assign13460_e17725 * locals.var_t0_dn7) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn8 = (-((assign13460_e17725 * locals.var_t0_dn8) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn9 = (-((assign13460_e17725 * locals.var_t0_dn9) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn10 = (-((assign13460_e17725 * locals.var_t0_dn10) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn11 = (-((assign13460_e17725 * locals.var_t0_dn11) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn13 = (-((assign13460_e17725 * locals.var_t0_dn13) / (locals.var_t0 * locals.var_t0)));
        locals.var_toxratio_dn14 = (-((assign13460_e17725 * locals.var_t0_dn14) / (locals.var_t0 * locals.var_t0)));

        let assign13470_e17730: f64 = (p.p1108 / locals.var_t1);
        let assign13470_e17732: f64 = (assign13470_e17730).powf(locals.var_ntox_i);
        let assign13470_e17734: f64 = (assign13470_e17732 / locals.var_t2);
        locals.var_toxratioedge = assign13470_e17734;
        locals.var_toxratioedge_dn0 = (((if 0.0 == 0.0 && ((locals.var_ntox_i) as f64).is_finite() && ((locals.var_ntox_i) as f64).fract() == 0.0 { if locals.var_ntox_i == 0.0 { 0.0 } else { (locals.var_ntox_i * ((assign13470_e17730).powf(locals.var_ntox_i - 1.0) * (-((p.p1108 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))))) } } else { (assign13470_e17732 * (locals.var_ntox_i * ((-((p.p1108 * locals.var_t1_dn0) / (locals.var_t1 * locals.var_t1))) / assign13470_e17730))) } * locals.var_t2) - (assign13470_e17732 * locals.var_t2_dn0)) / (locals.var_t2 * locals.var_t2));
        locals.var_toxratioedge_dn2 = (((if 0.0 == 0.0 && ((locals.var_ntox_i) as f64).is_finite() && ((locals.var_ntox_i) as f64).fract() == 0.0 { if locals.var_ntox_i == 0.0 { 0.0 } else { (locals.var_ntox_i * ((assign13470_e17730).powf(locals.var_ntox_i - 1.0) * (-((p.p1108 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))))) } } else { (assign13470_e17732 * (locals.var_ntox_i * ((-((p.p1108 * locals.var_t1_dn2) / (locals.var_t1 * locals.var_t1))) / assign13470_e17730))) } * locals.var_t2) - (assign13470_e17732 * locals.var_t2_dn2)) / (locals.var_t2 * locals.var_t2));
        locals.var_toxratioedge_dn3 = (((if 0.0 == 0.0 && ((locals.var_ntox_i) as f64).is_finite() && ((locals.var_ntox_i) as f64).fract() == 0.0 { if locals.var_ntox_i == 0.0 { 0.0 } else { (locals.var_ntox_i * ((assign13470_e17730).powf(locals.var_ntox_i - 1.0) * (-((p.p1108 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1))))) } } else { (assign13470_e17732 * (locals.var_ntox_i * ((-((p.p1108 * locals.var_t1_dn3) / (locals.var_t1 * locals.var_t1))) / assign13470_e17730))) } * locals.var_t2) - (assign13470_e17732 * locals.var_t2_dn3)) / (locals.var_t2 * locals.var_t2));
        locals.var_toxratioedge_dn4 = (((if 0.0 == 0.0 && ((locals.var_ntox_i) as f64).is_finite() && ((locals.var_ntox_i) as f64).fract() == 0.0 { if locals.var_ntox_i == 0.0 { 0.0 } else { (locals.var_ntox_i * ((assign13470_e17730).powf(locals.var_ntox_i - 1.0) * (-((p.p1108 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))))) } } else { (assign13470_e17732 * (locals.var_ntox_i * ((-((p.p1108 * locals.var_t1_dn4) / (locals.var_t1 * locals.var_t1))) / assign13470_e17730))) } * locals.var_t2) - (assign13470_e17732 * locals.var_t2_dn4)) / (locals.var_t2 * locals.var_t2));
        locals.var_toxratioedge_dn5 = (((if 0.0 == 0.0 && ((locals.var_ntox_i) as f64).is_finite() && ((locals.var_ntox_i) as f64).fract() == 0.0 { if locals.var_ntox_i == 0.0 { 0.0 } else { (locals.var_ntox_i * ((assign13470_e17730).powf(locals.var_ntox_i - 1.0) * (-((p.p1108 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))))) } } else { (assign13470_e17732 * (locals.var_ntox_i * ((-((p.p1108 * locals.var_t1_dn5) / (locals.var_t1 * locals.var_t1))) / assign13470_e17730))) } * locals.var_t2) - (assign13470_e17732 * locals.var_t2_dn5)) / (locals.var_t2 * locals.var_t2));
        locals.var_toxratioedge_dn6 = (((if 0.0 == 0.0 && ((locals.var_ntox_i) as f64).is_finite() && ((locals.var_ntox_i) as f64).fract() == 0.0 { if locals.var_ntox_i == 0.0 { 0.0 } else { (locals.var_ntox_i * ((assign13470_e17730).powf(locals.var_ntox_i - 1.0) * (-((p.p1108 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))))) } } else { (assign13470_e17732 * (locals.var_ntox_i * ((-((p.p1108 * locals.var_t1_dn6) / (locals.var_t1 * locals.var_t1))) / assign13470_e17730))) } * locals.var_t2) - (assign13470_e17732 * locals.var_t2_dn6)) / (locals.var_t2 * locals.var_t2));
        locals.var_toxratioedge_dn7 = (((if 0.0 == 0.0 && ((locals.var_ntox_i) as f64).is_finite() && ((locals.var_ntox_i) as f64).fract() == 0.0 { if locals.var_ntox_i == 0.0 { 0.0 } else { (locals.var_ntox_i * ((assign13470_e17730).powf(locals.var_ntox_i - 1.0) * (-((p.p1108 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))))) } } else { (assign13470_e17732 * (locals.var_ntox_i * ((-((p.p1108 * locals.var_t1_dn7) / (locals.var_t1 * locals.var_t1))) / assign13470_e17730))) } * locals.var_t2) - (assign13470_e17732 * locals.var_t2_dn7)) / (locals.var_t2 * locals.var_t2));
        locals.var_toxratioedge_dn8 = (((if 0.0 == 0.0 && ((locals.var_ntox_i) as f64).is_finite() && ((locals.var_ntox_i) as f64).fract() == 0.0 { if locals.var_ntox_i == 0.0 { 0.0 } else { (locals.var_ntox_i * ((assign13470_e17730).powf(locals.var_ntox_i - 1.0) * (-((p.p1108 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))))) } } else { (assign13470_e17732 * (locals.var_ntox_i * ((-((p.p1108 * locals.var_t1_dn8) / (locals.var_t1 * locals.var_t1))) / assign13470_e17730))) } * locals.var_t2) - (assign13470_e17732 * locals.var_t2_dn8)) / (locals.var_t2 * locals.var_t2));
        locals.var_toxratioedge_dn9 = (((if 0.0 == 0.0 && ((locals.var_ntox_i) as f64).is_finite() && ((locals.var_ntox_i) as f64).fract() == 0.0 { if locals.var_ntox_i == 0.0 { 0.0 } else { (locals.var_ntox_i * ((assign13470_e17730).powf(locals.var_ntox_i - 1.0) * (-((p.p1108 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))))) } } else { (assign13470_e17732 * (locals.var_ntox_i * ((-((p.p1108 * locals.var_t1_dn9) / (locals.var_t1 * locals.var_t1))) / assign13470_e17730))) } * locals.var_t2) - (assign13470_e17732 * locals.var_t2_dn9)) / (locals.var_t2 * locals.var_t2));
        locals.var_toxratioedge_dn10 = (((if 0.0 == 0.0 && ((locals.var_ntox_i) as f64).is_finite() && ((locals.var_ntox_i) as f64).fract() == 0.0 { if locals.var_ntox_i == 0.0 { 0.0 } else { (locals.var_ntox_i * ((assign13470_e17730).powf(locals.var_ntox_i - 1.0) * (-((p.p1108 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))))) } } else { (assign13470_e17732 * (locals.var_ntox_i * ((-((p.p1108 * locals.var_t1_dn10) / (locals.var_t1 * locals.var_t1))) / assign13470_e17730))) } * locals.var_t2) - (assign13470_e17732 * locals.var_t2_dn10)) / (locals.var_t2 * locals.var_t2));
        locals.var_toxratioedge_dn11 = (((if 0.0 == 0.0 && ((locals.var_ntox_i) as f64).is_finite() && ((locals.var_ntox_i) as f64).fract() == 0.0 { if locals.var_ntox_i == 0.0 { 0.0 } else { (locals.var_ntox_i * ((assign13470_e17730).powf(locals.var_ntox_i - 1.0) * (-((p.p1108 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))))) } } else { (assign13470_e17732 * (locals.var_ntox_i * ((-((p.p1108 * locals.var_t1_dn11) / (locals.var_t1 * locals.var_t1))) / assign13470_e17730))) } * locals.var_t2) - (assign13470_e17732 * locals.var_t2_dn11)) / (locals.var_t2 * locals.var_t2));
        locals.var_toxratioedge_dn13 = (((if 0.0 == 0.0 && ((locals.var_ntox_i) as f64).is_finite() && ((locals.var_ntox_i) as f64).fract() == 0.0 { if locals.var_ntox_i == 0.0 { 0.0 } else { (locals.var_ntox_i * ((assign13470_e17730).powf(locals.var_ntox_i - 1.0) * (-((p.p1108 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))))) } } else { (assign13470_e17732 * (locals.var_ntox_i * ((-((p.p1108 * locals.var_t1_dn13) / (locals.var_t1 * locals.var_t1))) / assign13470_e17730))) } * locals.var_t2) - (assign13470_e17732 * locals.var_t2_dn13)) / (locals.var_t2 * locals.var_t2));
        locals.var_toxratioedge_dn14 = (((if 0.0 == 0.0 && ((locals.var_ntox_i) as f64).is_finite() && ((locals.var_ntox_i) as f64).fract() == 0.0 { if locals.var_ntox_i == 0.0 { 0.0 } else { (locals.var_ntox_i * ((assign13470_e17730).powf(locals.var_ntox_i - 1.0) * (-((p.p1108 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))))) } } else { (assign13470_e17732 * (locals.var_ntox_i * ((-((p.p1108 * locals.var_t1_dn14) / (locals.var_t1 * locals.var_t1))) / assign13470_e17730))) } * locals.var_t2) - (assign13470_e17732 * locals.var_t2_dn14)) / (locals.var_t2 * locals.var_t2));

        let assign13480_e17737: f64 = (locals.var_weff0 * locals.var_aechvb);
        let assign13480_e17739: f64 = (assign13480_e17737 * locals.var_toxratioedge);
        locals.var_igsd_mult0 = assign13480_e17739;
        locals.var_igsd_mult0_dn0 = (assign13480_e17737 * locals.var_toxratioedge_dn0);
        locals.var_igsd_mult0_dn2 = (assign13480_e17737 * locals.var_toxratioedge_dn2);
        locals.var_igsd_mult0_dn3 = (assign13480_e17737 * locals.var_toxratioedge_dn3);
        locals.var_igsd_mult0_dn4 = (assign13480_e17737 * locals.var_toxratioedge_dn4);
        locals.var_igsd_mult0_dn5 = (assign13480_e17737 * locals.var_toxratioedge_dn5);
        locals.var_igsd_mult0_dn6 = (assign13480_e17737 * locals.var_toxratioedge_dn6);
        locals.var_igsd_mult0_dn7 = (assign13480_e17737 * locals.var_toxratioedge_dn7);
        locals.var_igsd_mult0_dn8 = (assign13480_e17737 * locals.var_toxratioedge_dn8);
        locals.var_igsd_mult0_dn9 = (assign13480_e17737 * locals.var_toxratioedge_dn9);
        locals.var_igsd_mult0_dn10 = (assign13480_e17737 * locals.var_toxratioedge_dn10);
        locals.var_igsd_mult0_dn11 = (assign13480_e17737 * locals.var_toxratioedge_dn11);
        locals.var_igsd_mult0_dn13 = (assign13480_e17737 * locals.var_toxratioedge_dn13);
        locals.var_igsd_mult0_dn14 = (assign13480_e17737 * locals.var_toxratioedge_dn14);

        let assign13490_e17742: f64 = (-273.15);
        let assign13490_e17743: f64 = if p.p1717 < assign13490_e17742 { 1.0 } else { 0.0 };
        locals.var_guard233 = assign13490_e17743;

        let (assign13500_e17747,) = {
    if (locals.var_guard233 != 0.0) {
        (300.15,)
    } else {
        (locals.var_tnom,)
    }
};
        locals.var_tnom = assign13500_e17747;

        let (assign13510_e17754,) = {
    if (locals.var_guard233 == 0.0) {
        let assign13510_e17752: f64 = (p.p1717 + 273.15);
        (assign13510_e17752,)
    } else {
        (locals.var_tnom,)
    }
};
        locals.var_tnom = assign13510_e17754;

        let assign13520_e17757: f64 = if p.p57 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard234 = assign13520_e17757;

        let (assign13530_e17778,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13530_e17761: f64 = (p.p1806 - locals.var_dimension1_i);
        let assign13530_e17765: f64 = (p.p1827 * 1000000000.0);
        let assign13530_e17768: f64 = (p.p43 * 1000000000.0);
        let assign13530_e17769: f64 = (assign13530_e17765 - assign13530_e17768);
        let assign13530_e17771: f64 = (assign13530_e17769 / p.p1828);
        let assign13530_e17772: f64 = { let limited_exp_arg = assign13530_e17771; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13530_e17773: f64 = (1.0 + assign13530_e17772);
        let assign13530_e17774: f64 = (assign13530_e17761 / assign13530_e17773);
        let assign13530_e17776: f64 = (assign13530_e17774 + locals.var_dimension1_i);
        (assign13530_e17776,)
    } else {
        (locals.var_d1,)
    }
};
        locals.var_d1 = assign13530_e17778;

        let (assign13540_e17799,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13540_e17782: f64 = (p.p1813 - locals.var_dimension2_i);
        let assign13540_e17786: f64 = (p.p1827 * 1000000000.0);
        let assign13540_e17789: f64 = (p.p43 * 1000000000.0);
        let assign13540_e17790: f64 = (assign13540_e17786 - assign13540_e17789);
        let assign13540_e17792: f64 = (assign13540_e17790 / p.p1828);
        let assign13540_e17793: f64 = { let limited_exp_arg = assign13540_e17792; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13540_e17794: f64 = (1.0 + assign13540_e17793);
        let assign13540_e17795: f64 = (assign13540_e17782 / assign13540_e17794);
        let assign13540_e17797: f64 = (assign13540_e17795 + locals.var_dimension2_i);
        (assign13540_e17797,)
    } else {
        (locals.var_d2,)
    }
};
        locals.var_d2 = assign13540_e17799;

        let (assign13550_e17820,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13550_e17803: f64 = (p.p1820 - locals.var_dimension3_i);
        let assign13550_e17807: f64 = (p.p1827 * 1000000000.0);
        let assign13550_e17810: f64 = (p.p43 * 1000000000.0);
        let assign13550_e17811: f64 = (assign13550_e17807 - assign13550_e17810);
        let assign13550_e17813: f64 = (assign13550_e17811 / p.p1828);
        let assign13550_e17814: f64 = { let limited_exp_arg = assign13550_e17813; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13550_e17815: f64 = (1.0 + assign13550_e17814);
        let assign13550_e17816: f64 = (assign13550_e17803 / assign13550_e17815);
        let assign13550_e17818: f64 = (assign13550_e17816 + locals.var_dimension3_i);
        (assign13550_e17818,)
    } else {
        (locals.var_d3,)
    }
};
        locals.var_d3 = assign13550_e17820;

        let (assign13560_e17891,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13560_e17824: f64 = (-p.p1847);
        let assign13560_e17828: f64 = (p.p1850 * 1000000000.0);
        let assign13560_e17831: f64 = (p.p43 * 1000000000.0);
        let assign13560_e17832: f64 = (assign13560_e17828 - assign13560_e17831);
        let assign13560_e17834: f64 = (assign13560_e17832 / p.p1851);
        let assign13560_e17835: f64 = { let limited_exp_arg = assign13560_e17834; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13560_e17836: f64 = (1.0 + assign13560_e17835);
        let assign13560_e17837: f64 = (assign13560_e17824 / assign13560_e17836);
        let assign13560_e17839: f64 = (assign13560_e17837 + locals.var_ssp1_i);
        let assign13560_e17841: f64 = assign13560_e17839;
        let assign13560_e17843: f64 = (-p.p1847);
        let assign13560_e17847: f64 = (p.p1850 * 1000000000.0);
        let assign13560_e17850: f64 = (p.p43 * 1000000000.0);
        let assign13560_e17851: f64 = (assign13560_e17847 - assign13560_e17850);
        let assign13560_e17853: f64 = (assign13560_e17851 / p.p1851);
        let assign13560_e17854: f64 = { let limited_exp_arg = assign13560_e17853; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13560_e17855: f64 = (1.0 + assign13560_e17854);
        let assign13560_e17856: f64 = (assign13560_e17843 / assign13560_e17855);
        let assign13560_e17858: f64 = (assign13560_e17856 + locals.var_ssp1_i);
        let assign13560_e17860: f64 = assign13560_e17858;
        let assign13560_e17862: f64 = (-p.p1847);
        let assign13560_e17866: f64 = (p.p1850 * 1000000000.0);
        let assign13560_e17869: f64 = (p.p43 * 1000000000.0);
        let assign13560_e17870: f64 = (assign13560_e17866 - assign13560_e17869);
        let assign13560_e17872: f64 = (assign13560_e17870 / p.p1851);
        let assign13560_e17873: f64 = { let limited_exp_arg = assign13560_e17872; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13560_e17874: f64 = (1.0 + assign13560_e17873);
        let assign13560_e17875: f64 = (assign13560_e17862 / assign13560_e17874);
        let assign13560_e17877: f64 = (assign13560_e17875 + locals.var_ssp1_i);
        let assign13560_e17879: f64 = assign13560_e17877;
        let assign13560_e17880: f64 = (assign13560_e17860 * assign13560_e17879);
        let assign13560_e17883: f64 = (0.25 * 0.001);
        let assign13560_e17885: f64 = (assign13560_e17883 * 0.001);
        let assign13560_e17886: f64 = (assign13560_e17880 + assign13560_e17885);
        let assign13560_e17887: f64 = (assign13560_e17886).sqrt();
        let assign13560_e17888: f64 = (assign13560_e17841 + assign13560_e17887);
        let assign13560_e17889: f64 = (0.5 * assign13560_e17888);
        (assign13560_e17889,)
    } else {
        (locals.var_p1,)
    }
};
        locals.var_p1 = assign13560_e17891;

        let (assign13570_e17962,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13570_e17895: f64 = (-p.p1848);
        let assign13570_e17899: f64 = (p.p1850 * 1000000000.0);
        let assign13570_e17902: f64 = (p.p43 * 1000000000.0);
        let assign13570_e17903: f64 = (assign13570_e17899 - assign13570_e17902);
        let assign13570_e17905: f64 = (assign13570_e17903 / p.p1851);
        let assign13570_e17906: f64 = { let limited_exp_arg = assign13570_e17905; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13570_e17907: f64 = (1.0 + assign13570_e17906);
        let assign13570_e17908: f64 = (assign13570_e17895 / assign13570_e17907);
        let assign13570_e17910: f64 = (assign13570_e17908 + locals.var_ssp2_i);
        let assign13570_e17912: f64 = assign13570_e17910;
        let assign13570_e17914: f64 = (-p.p1848);
        let assign13570_e17918: f64 = (p.p1850 * 1000000000.0);
        let assign13570_e17921: f64 = (p.p43 * 1000000000.0);
        let assign13570_e17922: f64 = (assign13570_e17918 - assign13570_e17921);
        let assign13570_e17924: f64 = (assign13570_e17922 / p.p1851);
        let assign13570_e17925: f64 = { let limited_exp_arg = assign13570_e17924; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13570_e17926: f64 = (1.0 + assign13570_e17925);
        let assign13570_e17927: f64 = (assign13570_e17914 / assign13570_e17926);
        let assign13570_e17929: f64 = (assign13570_e17927 + locals.var_ssp2_i);
        let assign13570_e17931: f64 = assign13570_e17929;
        let assign13570_e17933: f64 = (-p.p1848);
        let assign13570_e17937: f64 = (p.p1850 * 1000000000.0);
        let assign13570_e17940: f64 = (p.p43 * 1000000000.0);
        let assign13570_e17941: f64 = (assign13570_e17937 - assign13570_e17940);
        let assign13570_e17943: f64 = (assign13570_e17941 / p.p1851);
        let assign13570_e17944: f64 = { let limited_exp_arg = assign13570_e17943; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13570_e17945: f64 = (1.0 + assign13570_e17944);
        let assign13570_e17946: f64 = (assign13570_e17933 / assign13570_e17945);
        let assign13570_e17948: f64 = (assign13570_e17946 + locals.var_ssp2_i);
        let assign13570_e17950: f64 = assign13570_e17948;
        let assign13570_e17951: f64 = (assign13570_e17931 * assign13570_e17950);
        let assign13570_e17954: f64 = (0.25 * 0.001);
        let assign13570_e17956: f64 = (assign13570_e17954 * 0.001);
        let assign13570_e17957: f64 = (assign13570_e17951 + assign13570_e17956);
        let assign13570_e17958: f64 = (assign13570_e17957).sqrt();
        let assign13570_e17959: f64 = (assign13570_e17912 + assign13570_e17958);
        let assign13570_e17960: f64 = (0.5 * assign13570_e17959);
        (assign13570_e17960,)
    } else {
        (locals.var_p2,)
    }
};
        locals.var_p2 = assign13570_e17962;

        let (assign13580_e18033,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13580_e17966: f64 = (-p.p1849);
        let assign13580_e17970: f64 = (p.p1850 * 1000000000.0);
        let assign13580_e17973: f64 = (p.p43 * 1000000000.0);
        let assign13580_e17974: f64 = (assign13580_e17970 - assign13580_e17973);
        let assign13580_e17976: f64 = (assign13580_e17974 / p.p1851);
        let assign13580_e17977: f64 = { let limited_exp_arg = assign13580_e17976; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13580_e17978: f64 = (1.0 + assign13580_e17977);
        let assign13580_e17979: f64 = (assign13580_e17966 / assign13580_e17978);
        let assign13580_e17981: f64 = (assign13580_e17979 + locals.var_ssp3_i);
        let assign13580_e17983: f64 = assign13580_e17981;
        let assign13580_e17985: f64 = (-p.p1849);
        let assign13580_e17989: f64 = (p.p1850 * 1000000000.0);
        let assign13580_e17992: f64 = (p.p43 * 1000000000.0);
        let assign13580_e17993: f64 = (assign13580_e17989 - assign13580_e17992);
        let assign13580_e17995: f64 = (assign13580_e17993 / p.p1851);
        let assign13580_e17996: f64 = { let limited_exp_arg = assign13580_e17995; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13580_e17997: f64 = (1.0 + assign13580_e17996);
        let assign13580_e17998: f64 = (assign13580_e17985 / assign13580_e17997);
        let assign13580_e18000: f64 = (assign13580_e17998 + locals.var_ssp3_i);
        let assign13580_e18002: f64 = assign13580_e18000;
        let assign13580_e18004: f64 = (-p.p1849);
        let assign13580_e18008: f64 = (p.p1850 * 1000000000.0);
        let assign13580_e18011: f64 = (p.p43 * 1000000000.0);
        let assign13580_e18012: f64 = (assign13580_e18008 - assign13580_e18011);
        let assign13580_e18014: f64 = (assign13580_e18012 / p.p1851);
        let assign13580_e18015: f64 = { let limited_exp_arg = assign13580_e18014; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13580_e18016: f64 = (1.0 + assign13580_e18015);
        let assign13580_e18017: f64 = (assign13580_e18004 / assign13580_e18016);
        let assign13580_e18019: f64 = (assign13580_e18017 + locals.var_ssp3_i);
        let assign13580_e18021: f64 = assign13580_e18019;
        let assign13580_e18022: f64 = (assign13580_e18002 * assign13580_e18021);
        let assign13580_e18025: f64 = (0.25 * 0.001);
        let assign13580_e18027: f64 = (assign13580_e18025 * 0.001);
        let assign13580_e18028: f64 = (assign13580_e18022 + assign13580_e18027);
        let assign13580_e18029: f64 = (assign13580_e18028).sqrt();
        let assign13580_e18030: f64 = (assign13580_e17983 + assign13580_e18029);
        let assign13580_e18031: f64 = (0.5 * assign13580_e18030);
        (assign13580_e18031,)
    } else {
        (locals.var_p3,)
    }
};
        locals.var_p3 = assign13580_e18033;

    }

    pub(super) fn stamp_transient_block_35(
        locals: &mut StampLocals,
    ) {
        let (assign13590_e18153,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13590_e18040: f64 = (locals.var_d1 - 1.001);
        let assign13590_e18041: f64 = (1.001 * assign13590_e18040);
        let assign13590_e18043: f64 = assign13590_e18041;
        let assign13590_e18047: f64 = (locals.var_d1 - 1.001);
        let assign13590_e18048: f64 = (1.001 * assign13590_e18047);
        let assign13590_e18050: f64 = assign13590_e18048;
        let assign13590_e18054: f64 = (locals.var_d1 - 1.001);
        let assign13590_e18055: f64 = (1.001 * assign13590_e18054);
        let assign13590_e18057: f64 = assign13590_e18055;
        let assign13590_e18058: f64 = (assign13590_e18050 * assign13590_e18057);
        let assign13590_e18061: f64 = (0.25 * 0.001);
        let assign13590_e18063: f64 = (assign13590_e18061 * 0.001);
        let assign13590_e18064: f64 = (assign13590_e18058 + assign13590_e18063);
        let assign13590_e18065: f64 = (assign13590_e18064).sqrt();
        let assign13590_e18066: f64 = (assign13590_e18043 + assign13590_e18065);
        let assign13590_e18067: f64 = (0.5 * assign13590_e18066);
        let assign13590_e18069: f64 = (assign13590_e18067 + 1.0);
        let assign13590_e18074: f64 = (locals.var_d1 - 1.001);
        let assign13590_e18075: f64 = (1.001 * assign13590_e18074);
        let assign13590_e18077: f64 = assign13590_e18075;
        let assign13590_e18081: f64 = (locals.var_d1 - 1.001);
        let assign13590_e18082: f64 = (1.001 * assign13590_e18081);
        let assign13590_e18084: f64 = assign13590_e18082;
        let assign13590_e18088: f64 = (locals.var_d1 - 1.001);
        let assign13590_e18089: f64 = (1.001 * assign13590_e18088);
        let assign13590_e18091: f64 = assign13590_e18089;
        let assign13590_e18092: f64 = (assign13590_e18084 * assign13590_e18091);
        let assign13590_e18095: f64 = (0.25 * 0.001);
        let assign13590_e18097: f64 = (assign13590_e18095 * 0.001);
        let assign13590_e18098: f64 = (assign13590_e18092 + assign13590_e18097);
        let assign13590_e18099: f64 = (assign13590_e18098).sqrt();
        let assign13590_e18100: f64 = (assign13590_e18077 + assign13590_e18099);
        let assign13590_e18101: f64 = (0.5 * assign13590_e18100);
        let assign13590_e18103: f64 = (assign13590_e18101 - 1.0);
        let assign13590_e18108: f64 = (locals.var_d1 - 1.001);
        let assign13590_e18109: f64 = (1.001 * assign13590_e18108);
        let assign13590_e18111: f64 = assign13590_e18109;
        let assign13590_e18115: f64 = (locals.var_d1 - 1.001);
        let assign13590_e18116: f64 = (1.001 * assign13590_e18115);
        let assign13590_e18118: f64 = assign13590_e18116;
        let assign13590_e18122: f64 = (locals.var_d1 - 1.001);
        let assign13590_e18123: f64 = (1.001 * assign13590_e18122);
        let assign13590_e18125: f64 = assign13590_e18123;
        let assign13590_e18126: f64 = (assign13590_e18118 * assign13590_e18125);
        let assign13590_e18129: f64 = (0.25 * 0.001);
        let assign13590_e18131: f64 = (assign13590_e18129 * 0.001);
        let assign13590_e18132: f64 = (assign13590_e18126 + assign13590_e18131);
        let assign13590_e18133: f64 = (assign13590_e18132).sqrt();
        let assign13590_e18134: f64 = (assign13590_e18111 + assign13590_e18133);
        let assign13590_e18135: f64 = (0.5 * assign13590_e18134);
        let assign13590_e18137: f64 = (assign13590_e18135 - 1.0);
        let assign13590_e18138: f64 = (assign13590_e18103 * assign13590_e18137);
        let assign13590_e18141: f64 = (0.25 * 0.001);
        let assign13590_e18143: f64 = (assign13590_e18141 * 0.001);
        let assign13590_e18144: f64 = (assign13590_e18138 + assign13590_e18143);
        let assign13590_e18145: f64 = (assign13590_e18144).sqrt();
        let assign13590_e18146: f64 = (assign13590_e18069 - assign13590_e18145);
        let assign13590_e18147: f64 = (0.5 * assign13590_e18146);
        let assign13590_e18150: f64 = (0.25 * 0.001);
        let assign13590_e18151: f64 = (assign13590_e18147 + assign13590_e18150);
        (assign13590_e18151,)
    } else {
        (locals.var_wp1,)
    }
};
        locals.var_wp1 = assign13590_e18153;

        let (assign13600_e18273,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13600_e18160: f64 = (locals.var_d1 - 2.001);
        let assign13600_e18161: f64 = (1.001 * assign13600_e18160);
        let assign13600_e18163: f64 = assign13600_e18161;
        let assign13600_e18167: f64 = (locals.var_d1 - 2.001);
        let assign13600_e18168: f64 = (1.001 * assign13600_e18167);
        let assign13600_e18170: f64 = assign13600_e18168;
        let assign13600_e18174: f64 = (locals.var_d1 - 2.001);
        let assign13600_e18175: f64 = (1.001 * assign13600_e18174);
        let assign13600_e18177: f64 = assign13600_e18175;
        let assign13600_e18178: f64 = (assign13600_e18170 * assign13600_e18177);
        let assign13600_e18181: f64 = (0.25 * 0.001);
        let assign13600_e18183: f64 = (assign13600_e18181 * 0.001);
        let assign13600_e18184: f64 = (assign13600_e18178 + assign13600_e18183);
        let assign13600_e18185: f64 = (assign13600_e18184).sqrt();
        let assign13600_e18186: f64 = (assign13600_e18163 + assign13600_e18185);
        let assign13600_e18187: f64 = (0.5 * assign13600_e18186);
        let assign13600_e18189: f64 = (assign13600_e18187 + 1.0);
        let assign13600_e18194: f64 = (locals.var_d1 - 2.001);
        let assign13600_e18195: f64 = (1.001 * assign13600_e18194);
        let assign13600_e18197: f64 = assign13600_e18195;
        let assign13600_e18201: f64 = (locals.var_d1 - 2.001);
        let assign13600_e18202: f64 = (1.001 * assign13600_e18201);
        let assign13600_e18204: f64 = assign13600_e18202;
        let assign13600_e18208: f64 = (locals.var_d1 - 2.001);
        let assign13600_e18209: f64 = (1.001 * assign13600_e18208);
        let assign13600_e18211: f64 = assign13600_e18209;
        let assign13600_e18212: f64 = (assign13600_e18204 * assign13600_e18211);
        let assign13600_e18215: f64 = (0.25 * 0.001);
        let assign13600_e18217: f64 = (assign13600_e18215 * 0.001);
        let assign13600_e18218: f64 = (assign13600_e18212 + assign13600_e18217);
        let assign13600_e18219: f64 = (assign13600_e18218).sqrt();
        let assign13600_e18220: f64 = (assign13600_e18197 + assign13600_e18219);
        let assign13600_e18221: f64 = (0.5 * assign13600_e18220);
        let assign13600_e18223: f64 = (assign13600_e18221 - 1.0);
        let assign13600_e18228: f64 = (locals.var_d1 - 2.001);
        let assign13600_e18229: f64 = (1.001 * assign13600_e18228);
        let assign13600_e18231: f64 = assign13600_e18229;
        let assign13600_e18235: f64 = (locals.var_d1 - 2.001);
        let assign13600_e18236: f64 = (1.001 * assign13600_e18235);
        let assign13600_e18238: f64 = assign13600_e18236;
        let assign13600_e18242: f64 = (locals.var_d1 - 2.001);
        let assign13600_e18243: f64 = (1.001 * assign13600_e18242);
        let assign13600_e18245: f64 = assign13600_e18243;
        let assign13600_e18246: f64 = (assign13600_e18238 * assign13600_e18245);
        let assign13600_e18249: f64 = (0.25 * 0.001);
        let assign13600_e18251: f64 = (assign13600_e18249 * 0.001);
        let assign13600_e18252: f64 = (assign13600_e18246 + assign13600_e18251);
        let assign13600_e18253: f64 = (assign13600_e18252).sqrt();
        let assign13600_e18254: f64 = (assign13600_e18231 + assign13600_e18253);
        let assign13600_e18255: f64 = (0.5 * assign13600_e18254);
        let assign13600_e18257: f64 = (assign13600_e18255 - 1.0);
        let assign13600_e18258: f64 = (assign13600_e18223 * assign13600_e18257);
        let assign13600_e18261: f64 = (0.25 * 0.001);
        let assign13600_e18263: f64 = (assign13600_e18261 * 0.001);
        let assign13600_e18264: f64 = (assign13600_e18258 + assign13600_e18263);
        let assign13600_e18265: f64 = (assign13600_e18264).sqrt();
        let assign13600_e18266: f64 = (assign13600_e18189 - assign13600_e18265);
        let assign13600_e18267: f64 = (0.5 * assign13600_e18266);
        let assign13600_e18270: f64 = (0.25 * 0.001);
        let assign13600_e18271: f64 = (assign13600_e18267 + assign13600_e18270);
        (assign13600_e18271,)
    } else {
        (locals.var_tp1,)
    }
};
        locals.var_tp1 = assign13600_e18273;

        let (assign13610_e18393,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13610_e18280: f64 = (locals.var_d2 - 1.001);
        let assign13610_e18281: f64 = (1.001 * assign13610_e18280);
        let assign13610_e18283: f64 = assign13610_e18281;
        let assign13610_e18287: f64 = (locals.var_d2 - 1.001);
        let assign13610_e18288: f64 = (1.001 * assign13610_e18287);
        let assign13610_e18290: f64 = assign13610_e18288;
        let assign13610_e18294: f64 = (locals.var_d2 - 1.001);
        let assign13610_e18295: f64 = (1.001 * assign13610_e18294);
        let assign13610_e18297: f64 = assign13610_e18295;
        let assign13610_e18298: f64 = (assign13610_e18290 * assign13610_e18297);
        let assign13610_e18301: f64 = (0.25 * 0.001);
        let assign13610_e18303: f64 = (assign13610_e18301 * 0.001);
        let assign13610_e18304: f64 = (assign13610_e18298 + assign13610_e18303);
        let assign13610_e18305: f64 = (assign13610_e18304).sqrt();
        let assign13610_e18306: f64 = (assign13610_e18283 + assign13610_e18305);
        let assign13610_e18307: f64 = (0.5 * assign13610_e18306);
        let assign13610_e18309: f64 = (assign13610_e18307 + 1.0);
        let assign13610_e18314: f64 = (locals.var_d2 - 1.001);
        let assign13610_e18315: f64 = (1.001 * assign13610_e18314);
        let assign13610_e18317: f64 = assign13610_e18315;
        let assign13610_e18321: f64 = (locals.var_d2 - 1.001);
        let assign13610_e18322: f64 = (1.001 * assign13610_e18321);
        let assign13610_e18324: f64 = assign13610_e18322;
        let assign13610_e18328: f64 = (locals.var_d2 - 1.001);
        let assign13610_e18329: f64 = (1.001 * assign13610_e18328);
        let assign13610_e18331: f64 = assign13610_e18329;
        let assign13610_e18332: f64 = (assign13610_e18324 * assign13610_e18331);
        let assign13610_e18335: f64 = (0.25 * 0.001);
        let assign13610_e18337: f64 = (assign13610_e18335 * 0.001);
        let assign13610_e18338: f64 = (assign13610_e18332 + assign13610_e18337);
        let assign13610_e18339: f64 = (assign13610_e18338).sqrt();
        let assign13610_e18340: f64 = (assign13610_e18317 + assign13610_e18339);
        let assign13610_e18341: f64 = (0.5 * assign13610_e18340);
        let assign13610_e18343: f64 = (assign13610_e18341 - 1.0);
        let assign13610_e18348: f64 = (locals.var_d2 - 1.001);
        let assign13610_e18349: f64 = (1.001 * assign13610_e18348);
        let assign13610_e18351: f64 = assign13610_e18349;
        let assign13610_e18355: f64 = (locals.var_d2 - 1.001);
        let assign13610_e18356: f64 = (1.001 * assign13610_e18355);
        let assign13610_e18358: f64 = assign13610_e18356;
        let assign13610_e18362: f64 = (locals.var_d2 - 1.001);
        let assign13610_e18363: f64 = (1.001 * assign13610_e18362);
        let assign13610_e18365: f64 = assign13610_e18363;
        let assign13610_e18366: f64 = (assign13610_e18358 * assign13610_e18365);
        let assign13610_e18369: f64 = (0.25 * 0.001);
        let assign13610_e18371: f64 = (assign13610_e18369 * 0.001);
        let assign13610_e18372: f64 = (assign13610_e18366 + assign13610_e18371);
        let assign13610_e18373: f64 = (assign13610_e18372).sqrt();
        let assign13610_e18374: f64 = (assign13610_e18351 + assign13610_e18373);
        let assign13610_e18375: f64 = (0.5 * assign13610_e18374);
        let assign13610_e18377: f64 = (assign13610_e18375 - 1.0);
        let assign13610_e18378: f64 = (assign13610_e18343 * assign13610_e18377);
        let assign13610_e18381: f64 = (0.25 * 0.001);
        let assign13610_e18383: f64 = (assign13610_e18381 * 0.001);
        let assign13610_e18384: f64 = (assign13610_e18378 + assign13610_e18383);
        let assign13610_e18385: f64 = (assign13610_e18384).sqrt();
        let assign13610_e18386: f64 = (assign13610_e18309 - assign13610_e18385);
        let assign13610_e18387: f64 = (0.5 * assign13610_e18386);
        let assign13610_e18390: f64 = (0.25 * 0.001);
        let assign13610_e18391: f64 = (assign13610_e18387 + assign13610_e18390);
        (assign13610_e18391,)
    } else {
        (locals.var_wp2,)
    }
};
        locals.var_wp2 = assign13610_e18393;

        let (assign13620_e18513,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13620_e18400: f64 = (locals.var_d2 - 2.001);
        let assign13620_e18401: f64 = (1.001 * assign13620_e18400);
        let assign13620_e18403: f64 = assign13620_e18401;
        let assign13620_e18407: f64 = (locals.var_d2 - 2.001);
        let assign13620_e18408: f64 = (1.001 * assign13620_e18407);
        let assign13620_e18410: f64 = assign13620_e18408;
        let assign13620_e18414: f64 = (locals.var_d2 - 2.001);
        let assign13620_e18415: f64 = (1.001 * assign13620_e18414);
        let assign13620_e18417: f64 = assign13620_e18415;
        let assign13620_e18418: f64 = (assign13620_e18410 * assign13620_e18417);
        let assign13620_e18421: f64 = (0.25 * 0.001);
        let assign13620_e18423: f64 = (assign13620_e18421 * 0.001);
        let assign13620_e18424: f64 = (assign13620_e18418 + assign13620_e18423);
        let assign13620_e18425: f64 = (assign13620_e18424).sqrt();
        let assign13620_e18426: f64 = (assign13620_e18403 + assign13620_e18425);
        let assign13620_e18427: f64 = (0.5 * assign13620_e18426);
        let assign13620_e18429: f64 = (assign13620_e18427 + 1.0);
        let assign13620_e18434: f64 = (locals.var_d2 - 2.001);
        let assign13620_e18435: f64 = (1.001 * assign13620_e18434);
        let assign13620_e18437: f64 = assign13620_e18435;
        let assign13620_e18441: f64 = (locals.var_d2 - 2.001);
        let assign13620_e18442: f64 = (1.001 * assign13620_e18441);
        let assign13620_e18444: f64 = assign13620_e18442;
        let assign13620_e18448: f64 = (locals.var_d2 - 2.001);
        let assign13620_e18449: f64 = (1.001 * assign13620_e18448);
        let assign13620_e18451: f64 = assign13620_e18449;
        let assign13620_e18452: f64 = (assign13620_e18444 * assign13620_e18451);
        let assign13620_e18455: f64 = (0.25 * 0.001);
        let assign13620_e18457: f64 = (assign13620_e18455 * 0.001);
        let assign13620_e18458: f64 = (assign13620_e18452 + assign13620_e18457);
        let assign13620_e18459: f64 = (assign13620_e18458).sqrt();
        let assign13620_e18460: f64 = (assign13620_e18437 + assign13620_e18459);
        let assign13620_e18461: f64 = (0.5 * assign13620_e18460);
        let assign13620_e18463: f64 = (assign13620_e18461 - 1.0);
        let assign13620_e18468: f64 = (locals.var_d2 - 2.001);
        let assign13620_e18469: f64 = (1.001 * assign13620_e18468);
        let assign13620_e18471: f64 = assign13620_e18469;
        let assign13620_e18475: f64 = (locals.var_d2 - 2.001);
        let assign13620_e18476: f64 = (1.001 * assign13620_e18475);
        let assign13620_e18478: f64 = assign13620_e18476;
        let assign13620_e18482: f64 = (locals.var_d2 - 2.001);
        let assign13620_e18483: f64 = (1.001 * assign13620_e18482);
        let assign13620_e18485: f64 = assign13620_e18483;
        let assign13620_e18486: f64 = (assign13620_e18478 * assign13620_e18485);
        let assign13620_e18489: f64 = (0.25 * 0.001);
        let assign13620_e18491: f64 = (assign13620_e18489 * 0.001);
        let assign13620_e18492: f64 = (assign13620_e18486 + assign13620_e18491);
        let assign13620_e18493: f64 = (assign13620_e18492).sqrt();
        let assign13620_e18494: f64 = (assign13620_e18471 + assign13620_e18493);
        let assign13620_e18495: f64 = (0.5 * assign13620_e18494);
        let assign13620_e18497: f64 = (assign13620_e18495 - 1.0);
        let assign13620_e18498: f64 = (assign13620_e18463 * assign13620_e18497);
        let assign13620_e18501: f64 = (0.25 * 0.001);
        let assign13620_e18503: f64 = (assign13620_e18501 * 0.001);
        let assign13620_e18504: f64 = (assign13620_e18498 + assign13620_e18503);
        let assign13620_e18505: f64 = (assign13620_e18504).sqrt();
        let assign13620_e18506: f64 = (assign13620_e18429 - assign13620_e18505);
        let assign13620_e18507: f64 = (0.5 * assign13620_e18506);
        let assign13620_e18510: f64 = (0.25 * 0.001);
        let assign13620_e18511: f64 = (assign13620_e18507 + assign13620_e18510);
        (assign13620_e18511,)
    } else {
        (locals.var_tp2,)
    }
};
        locals.var_tp2 = assign13620_e18513;

        let (assign13630_e18633,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13630_e18520: f64 = (locals.var_d3 - 1.001);
        let assign13630_e18521: f64 = (1.001 * assign13630_e18520);
        let assign13630_e18523: f64 = assign13630_e18521;
        let assign13630_e18527: f64 = (locals.var_d3 - 1.001);
        let assign13630_e18528: f64 = (1.001 * assign13630_e18527);
        let assign13630_e18530: f64 = assign13630_e18528;
        let assign13630_e18534: f64 = (locals.var_d3 - 1.001);
        let assign13630_e18535: f64 = (1.001 * assign13630_e18534);
        let assign13630_e18537: f64 = assign13630_e18535;
        let assign13630_e18538: f64 = (assign13630_e18530 * assign13630_e18537);
        let assign13630_e18541: f64 = (0.25 * 0.001);
        let assign13630_e18543: f64 = (assign13630_e18541 * 0.001);
        let assign13630_e18544: f64 = (assign13630_e18538 + assign13630_e18543);
        let assign13630_e18545: f64 = (assign13630_e18544).sqrt();
        let assign13630_e18546: f64 = (assign13630_e18523 + assign13630_e18545);
        let assign13630_e18547: f64 = (0.5 * assign13630_e18546);
        let assign13630_e18549: f64 = (assign13630_e18547 + 1.0);
        let assign13630_e18554: f64 = (locals.var_d3 - 1.001);
        let assign13630_e18555: f64 = (1.001 * assign13630_e18554);
        let assign13630_e18557: f64 = assign13630_e18555;
        let assign13630_e18561: f64 = (locals.var_d3 - 1.001);
        let assign13630_e18562: f64 = (1.001 * assign13630_e18561);
        let assign13630_e18564: f64 = assign13630_e18562;
        let assign13630_e18568: f64 = (locals.var_d3 - 1.001);
        let assign13630_e18569: f64 = (1.001 * assign13630_e18568);
        let assign13630_e18571: f64 = assign13630_e18569;
        let assign13630_e18572: f64 = (assign13630_e18564 * assign13630_e18571);
        let assign13630_e18575: f64 = (0.25 * 0.001);
        let assign13630_e18577: f64 = (assign13630_e18575 * 0.001);
        let assign13630_e18578: f64 = (assign13630_e18572 + assign13630_e18577);
        let assign13630_e18579: f64 = (assign13630_e18578).sqrt();
        let assign13630_e18580: f64 = (assign13630_e18557 + assign13630_e18579);
        let assign13630_e18581: f64 = (0.5 * assign13630_e18580);
        let assign13630_e18583: f64 = (assign13630_e18581 - 1.0);
        let assign13630_e18588: f64 = (locals.var_d3 - 1.001);
        let assign13630_e18589: f64 = (1.001 * assign13630_e18588);
        let assign13630_e18591: f64 = assign13630_e18589;
        let assign13630_e18595: f64 = (locals.var_d3 - 1.001);
        let assign13630_e18596: f64 = (1.001 * assign13630_e18595);
        let assign13630_e18598: f64 = assign13630_e18596;
        let assign13630_e18602: f64 = (locals.var_d3 - 1.001);
        let assign13630_e18603: f64 = (1.001 * assign13630_e18602);
        let assign13630_e18605: f64 = assign13630_e18603;
        let assign13630_e18606: f64 = (assign13630_e18598 * assign13630_e18605);
        let assign13630_e18609: f64 = (0.25 * 0.001);
        let assign13630_e18611: f64 = (assign13630_e18609 * 0.001);
        let assign13630_e18612: f64 = (assign13630_e18606 + assign13630_e18611);
        let assign13630_e18613: f64 = (assign13630_e18612).sqrt();
        let assign13630_e18614: f64 = (assign13630_e18591 + assign13630_e18613);
        let assign13630_e18615: f64 = (0.5 * assign13630_e18614);
        let assign13630_e18617: f64 = (assign13630_e18615 - 1.0);
        let assign13630_e18618: f64 = (assign13630_e18583 * assign13630_e18617);
        let assign13630_e18621: f64 = (0.25 * 0.001);
        let assign13630_e18623: f64 = (assign13630_e18621 * 0.001);
        let assign13630_e18624: f64 = (assign13630_e18618 + assign13630_e18623);
        let assign13630_e18625: f64 = (assign13630_e18624).sqrt();
        let assign13630_e18626: f64 = (assign13630_e18549 - assign13630_e18625);
        let assign13630_e18627: f64 = (0.5 * assign13630_e18626);
        let assign13630_e18630: f64 = (0.25 * 0.001);
        let assign13630_e18631: f64 = (assign13630_e18627 + assign13630_e18630);
        (assign13630_e18631,)
    } else {
        (locals.var_wp3,)
    }
};
        locals.var_wp3 = assign13630_e18633;

        let (assign13640_e18753,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13640_e18640: f64 = (locals.var_d3 - 2.001);
        let assign13640_e18641: f64 = (1.001 * assign13640_e18640);
        let assign13640_e18643: f64 = assign13640_e18641;
        let assign13640_e18647: f64 = (locals.var_d3 - 2.001);
        let assign13640_e18648: f64 = (1.001 * assign13640_e18647);
        let assign13640_e18650: f64 = assign13640_e18648;
        let assign13640_e18654: f64 = (locals.var_d3 - 2.001);
        let assign13640_e18655: f64 = (1.001 * assign13640_e18654);
        let assign13640_e18657: f64 = assign13640_e18655;
        let assign13640_e18658: f64 = (assign13640_e18650 * assign13640_e18657);
        let assign13640_e18661: f64 = (0.25 * 0.001);
        let assign13640_e18663: f64 = (assign13640_e18661 * 0.001);
        let assign13640_e18664: f64 = (assign13640_e18658 + assign13640_e18663);
        let assign13640_e18665: f64 = (assign13640_e18664).sqrt();
        let assign13640_e18666: f64 = (assign13640_e18643 + assign13640_e18665);
        let assign13640_e18667: f64 = (0.5 * assign13640_e18666);
        let assign13640_e18669: f64 = (assign13640_e18667 + 1.0);
        let assign13640_e18674: f64 = (locals.var_d3 - 2.001);
        let assign13640_e18675: f64 = (1.001 * assign13640_e18674);
        let assign13640_e18677: f64 = assign13640_e18675;
        let assign13640_e18681: f64 = (locals.var_d3 - 2.001);
        let assign13640_e18682: f64 = (1.001 * assign13640_e18681);
        let assign13640_e18684: f64 = assign13640_e18682;
        let assign13640_e18688: f64 = (locals.var_d3 - 2.001);
        let assign13640_e18689: f64 = (1.001 * assign13640_e18688);
        let assign13640_e18691: f64 = assign13640_e18689;
        let assign13640_e18692: f64 = (assign13640_e18684 * assign13640_e18691);
        let assign13640_e18695: f64 = (0.25 * 0.001);
        let assign13640_e18697: f64 = (assign13640_e18695 * 0.001);
        let assign13640_e18698: f64 = (assign13640_e18692 + assign13640_e18697);
        let assign13640_e18699: f64 = (assign13640_e18698).sqrt();
        let assign13640_e18700: f64 = (assign13640_e18677 + assign13640_e18699);
        let assign13640_e18701: f64 = (0.5 * assign13640_e18700);
        let assign13640_e18703: f64 = (assign13640_e18701 - 1.0);
        let assign13640_e18708: f64 = (locals.var_d3 - 2.001);
        let assign13640_e18709: f64 = (1.001 * assign13640_e18708);
        let assign13640_e18711: f64 = assign13640_e18709;
        let assign13640_e18715: f64 = (locals.var_d3 - 2.001);
        let assign13640_e18716: f64 = (1.001 * assign13640_e18715);
        let assign13640_e18718: f64 = assign13640_e18716;
        let assign13640_e18722: f64 = (locals.var_d3 - 2.001);
        let assign13640_e18723: f64 = (1.001 * assign13640_e18722);
        let assign13640_e18725: f64 = assign13640_e18723;
        let assign13640_e18726: f64 = (assign13640_e18718 * assign13640_e18725);
        let assign13640_e18729: f64 = (0.25 * 0.001);
        let assign13640_e18731: f64 = (assign13640_e18729 * 0.001);
        let assign13640_e18732: f64 = (assign13640_e18726 + assign13640_e18731);
        let assign13640_e18733: f64 = (assign13640_e18732).sqrt();
        let assign13640_e18734: f64 = (assign13640_e18711 + assign13640_e18733);
        let assign13640_e18735: f64 = (0.5 * assign13640_e18734);
        let assign13640_e18737: f64 = (assign13640_e18735 - 1.0);
        let assign13640_e18738: f64 = (assign13640_e18703 * assign13640_e18737);
        let assign13640_e18741: f64 = (0.25 * 0.001);
        let assign13640_e18743: f64 = (assign13640_e18741 * 0.001);
        let assign13640_e18744: f64 = (assign13640_e18738 + assign13640_e18743);
        let assign13640_e18745: f64 = (assign13640_e18744).sqrt();
        let assign13640_e18746: f64 = (assign13640_e18669 - assign13640_e18745);
        let assign13640_e18747: f64 = (0.5 * assign13640_e18746);
        let assign13640_e18750: f64 = (0.25 * 0.001);
        let assign13640_e18751: f64 = (assign13640_e18747 + assign13640_e18750);
        (assign13640_e18751,)
    } else {
        (locals.var_tp3,)
    }
};
        locals.var_tp3 = assign13640_e18753;

        let (assign13650_e18765,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13650_e18757: f64 = (locals.var_weff0).powf(locals.var_wp1);
        let assign13650_e18760: f64 = (locals.var_ach / locals.var_weff0);
        let assign13650_e18762: f64 = (assign13650_e18760).powf(locals.var_tp1);
        let assign13650_e18763: f64 = (assign13650_e18757 * assign13650_e18762);
        (assign13650_e18763,)
    } else {
        (locals.var_vnd1,)
    }
};
        locals.var_vnd1 = assign13650_e18765;

        let (assign13660_e18771,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13660_e18769: f64 = (locals.var_vnd1 / locals.var_cins);
        (assign13660_e18769,)
    } else {
        (locals.var_qndnf1,)
    }
};
        locals.var_qndnf1 = assign13660_e18771;

        let (assign13670_e18783,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13670_e18775: f64 = (locals.var_weff0).powf(locals.var_wp2);
        let assign13670_e18778: f64 = (locals.var_ach / locals.var_weff0);
        let assign13670_e18780: f64 = (assign13670_e18778).powf(locals.var_tp2);
        let assign13670_e18781: f64 = (assign13670_e18775 * assign13670_e18780);
        (assign13670_e18781,)
    } else {
        (locals.var_vnd2,)
    }
};
        locals.var_vnd2 = assign13670_e18783;

        let (assign13680_e18789,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13680_e18787: f64 = (locals.var_vnd2 / locals.var_cins);
        (assign13680_e18787,)
    } else {
        (locals.var_qndnf2,)
    }
};
        locals.var_qndnf2 = assign13680_e18789;

        let (assign13690_e18801,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13690_e18793: f64 = (locals.var_weff0).powf(locals.var_wp3);
        let assign13690_e18796: f64 = (locals.var_ach / locals.var_weff0);
        let assign13690_e18798: f64 = (assign13690_e18796).powf(locals.var_tp3);
        let assign13690_e18799: f64 = (assign13690_e18793 * assign13690_e18798);
        (assign13690_e18799,)
    } else {
        (locals.var_vnd3,)
    }
};
        locals.var_vnd3 = assign13690_e18801;

        let (assign13700_e18807,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13700_e18805: f64 = (locals.var_vnd3 / locals.var_cins);
        (assign13700_e18805,)
    } else {
        (locals.var_qndnf3,)
    }
};
        locals.var_qndnf3 = assign13700_e18807;

    }

    pub(super) fn stamp_transient_block_36(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13710_e18863,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13710_e18815: f64 = (p.p40 * 1000000000.0);
        let assign13710_e18816: f64 = (2.75 - assign13710_e18815);
        let assign13710_e18818: f64 = (assign13710_e18816 / 0.78);
        let assign13710_e18819: f64 = { let limited_exp_arg = assign13710_e18818; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13710_e18820: f64 = (1.0 + assign13710_e18819);
        let assign13710_e18821: f64 = (1.0 / assign13710_e18820);
        let assign13710_e18823: f64 = (assign13710_e18821 + 0.5);
        let assign13710_e18829: f64 = (p.p40 * 1000000000.0);
        let assign13710_e18830: f64 = (2.75 - assign13710_e18829);
        let assign13710_e18832: f64 = (assign13710_e18830 / 0.78);
        let assign13710_e18833: f64 = { let limited_exp_arg = assign13710_e18832; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13710_e18834: f64 = (1.0 + assign13710_e18833);
        let assign13710_e18835: f64 = (1.0 / assign13710_e18834);
        let assign13710_e18837: f64 = (assign13710_e18835 - 0.5);
        let assign13710_e18843: f64 = (p.p40 * 1000000000.0);
        let assign13710_e18844: f64 = (2.75 - assign13710_e18843);
        let assign13710_e18846: f64 = (assign13710_e18844 / 0.78);
        let assign13710_e18847: f64 = { let limited_exp_arg = assign13710_e18846; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13710_e18848: f64 = (1.0 + assign13710_e18847);
        let assign13710_e18849: f64 = (1.0 / assign13710_e18848);
        let assign13710_e18851: f64 = (assign13710_e18849 - 0.5);
        let assign13710_e18852: f64 = (assign13710_e18837 * assign13710_e18851);
        let assign13710_e18855: f64 = (0.25 * 0.003);
        let assign13710_e18857: f64 = (assign13710_e18855 * 0.003);
        let assign13710_e18858: f64 = (assign13710_e18852 + assign13710_e18857);
        let assign13710_e18859: f64 = (assign13710_e18858).sqrt();
        let assign13710_e18860: f64 = (assign13710_e18823 + assign13710_e18859);
        let assign13710_e18861: f64 = (0.5 * assign13710_e18860);
        (assign13710_e18861,)
    } else {
        (locals.var_nc3d0,)
    }
};
        locals.var_nc3d0 = assign13710_e18863;

        let (assign13720_e18879,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13720_e18867: f64 = (1.0 - locals.var_nc3d0);
        let assign13720_e18870: f64 = (locals.var_d1 - locals.var_dimension1_i);
        let assign13720_e18871: f64 = (assign13720_e18867 * assign13720_e18870);
        let assign13720_e18874: f64 = (p.p1806 - locals.var_dimension1_i);
        let assign13720_e18875: f64 = (assign13720_e18871 / assign13720_e18874);
        let assign13720_e18877: f64 = (assign13720_e18875 + locals.var_nc3d0);
        (assign13720_e18877,)
    } else {
        (locals.var_nc3d,)
    }
};
        locals.var_nc3d = assign13720_e18879;

        let (assign13730_e18892,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13730_e18885: f64 = (locals.var_nc3d - 0.999);
        let assign13730_e18887: f64 = (assign13730_e18885 / 0.0001);
        let assign13730_e18888: f64 = { let limited_exp_arg = assign13730_e18887; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign13730_e18889: f64 = (1.0 + assign13730_e18888);
        let assign13730_e18890: f64 = (1.0 / assign13730_e18889);
        (assign13730_e18890,)
    } else {
        (locals.var_ncq,)
    }
};
        locals.var_ncq = assign13730_e18892;

        let (assign13740_e18910,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13740_e18896: f64 = (0.5 * p.p40);
        let assign13740_e18898: f64 = (assign13740_e18896 * p.p40);
        let assign13740_e18900: f64 = (assign13740_e18898 * 1e18);
        let assign13740_e18903: f64 = (1.5 * p.p40);
        let assign13740_e18905: f64 = (assign13740_e18903 * 1000000000.0);
        let assign13740_e18906: f64 = (assign13740_e18900 - assign13740_e18905);
        let assign13740_e18908: f64 = (assign13740_e18906 + 2.0);
        (assign13740_e18908,)
    } else {
        (locals.var_qt0,)
    }
};
        locals.var_qt0 = assign13740_e18910;

        let (assign13750_e18937,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13750_e18915: f64 = (locals.var_qt0 + 4.0);
        let assign13750_e18918: f64 = (locals.var_qt0 - 4.0);
        let assign13750_e18921: f64 = (locals.var_qt0 - 4.0);
        let assign13750_e18922: f64 = (assign13750_e18918 * assign13750_e18921);
        let assign13750_e18925: f64 = (0.25 * 0.01);
        let assign13750_e18927: f64 = (assign13750_e18925 * 0.01);
        let assign13750_e18928: f64 = (assign13750_e18922 + assign13750_e18927);
        let assign13750_e18929: f64 = (assign13750_e18928).sqrt();
        let assign13750_e18930: f64 = (assign13750_e18915 - assign13750_e18929);
        let assign13750_e18931: f64 = (0.5 * assign13750_e18930);
        let assign13750_e18934: f64 = (0.25 * 0.01);
        let assign13750_e18935: f64 = (assign13750_e18931 + assign13750_e18934);
        (assign13750_e18935,)
    } else {
        (locals.var_qt1,)
    }
};
        locals.var_qt1 = assign13750_e18937;

        let (assign13760_e19165, assign13760_e19165_d_n0, assign13760_e19165_d_n2, assign13760_e19165_d_n3, assign13760_e19165_d_n4, assign13760_e19165_d_n5, assign13760_e19165_d_n6, assign13760_e19165_d_n7, assign13760_e19165_d_n8, assign13760_e19165_d_n9, assign13760_e19165_d_n10, assign13760_e19165_d_n11, assign13760_e19165_d_n13, assign13760_e19165_d_n14,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13760_e18944: f64 = (p.p40 * 1000000000.0);
        let assign13760_e18946: f64 = (assign13760_e18944 - locals.var_qt1);
        let assign13760_e18948: f64 = (assign13760_e18946).powf(p.p1893);
        let assign13760_e18951: f64 = (924000.0 - 18100.0);
        let assign13760_e18952: f64 = (assign13760_e18948 * assign13760_e18951);
        let assign13760_e18955: f64 = (2.0_f64).powf(p.p1893);
        let assign13760_e18956: f64 = (assign13760_e18952 / assign13760_e18955);
        let assign13760_e18957: f64 = (locals.var_t0 + assign13760_e18956);
        let assign13760_e18959: f64 = (assign13760_e18957 + 18100.0);
        let assign13760_e18963: f64 = (p.p40 * 1000000000.0);
        let assign13760_e18965: f64 = (assign13760_e18963 - locals.var_qt1);
        let assign13760_e18967: f64 = (assign13760_e18965).powf(p.p1893);
        let assign13760_e18970: f64 = (924000.0 - 18100.0);
        let assign13760_e18971: f64 = (assign13760_e18967 * assign13760_e18970);
        let assign13760_e18974: f64 = (2.0_f64).powf(p.p1893);
        let assign13760_e18975: f64 = (assign13760_e18971 / assign13760_e18974);
        let assign13760_e18976: f64 = (locals.var_t0 + assign13760_e18975);
        let assign13760_e18978: f64 = (assign13760_e18976 - 18100.0);
        let assign13760_e18982: f64 = (p.p40 * 1000000000.0);
        let assign13760_e18984: f64 = (assign13760_e18982 - locals.var_qt1);
        let assign13760_e18986: f64 = (assign13760_e18984).powf(p.p1893);
        let assign13760_e18989: f64 = (924000.0 - 18100.0);
        let assign13760_e18990: f64 = (assign13760_e18986 * assign13760_e18989);
        let assign13760_e18993: f64 = (2.0_f64).powf(p.p1893);
        let assign13760_e18994: f64 = (assign13760_e18990 / assign13760_e18993);
        let assign13760_e18995: f64 = (locals.var_t0 + assign13760_e18994);
        let assign13760_e18997: f64 = (assign13760_e18995 - 18100.0);
        let assign13760_e18998: f64 = (assign13760_e18978 * assign13760_e18997);
        let assign13760_e19001: f64 = (0.25 * 0.01);
        let assign13760_e19003: f64 = (assign13760_e19001 * 0.01);
        let assign13760_e19004: f64 = (assign13760_e18998 + assign13760_e19003);
        let assign13760_e19005: f64 = (assign13760_e19004).sqrt();
        let assign13760_e19006: f64 = (assign13760_e18959 + assign13760_e19005);
        let assign13760_e19007: f64 = (0.5 * assign13760_e19006);
        let assign13760_e19009: f64 = (assign13760_e19007 + 924000.0);
        let assign13760_e19014: f64 = (p.p40 * 1000000000.0);
        let assign13760_e19016: f64 = (assign13760_e19014 - locals.var_qt1);
        let assign13760_e19018: f64 = (assign13760_e19016).powf(p.p1893);
        let assign13760_e19021: f64 = (924000.0 - 18100.0);
        let assign13760_e19022: f64 = (assign13760_e19018 * assign13760_e19021);
        let assign13760_e19025: f64 = (2.0_f64).powf(p.p1893);
        let assign13760_e19026: f64 = (assign13760_e19022 / assign13760_e19025);
        let assign13760_e19027: f64 = (locals.var_t0 + assign13760_e19026);
        let assign13760_e19029: f64 = (assign13760_e19027 + 18100.0);
        let assign13760_e19033: f64 = (p.p40 * 1000000000.0);
        let assign13760_e19035: f64 = (assign13760_e19033 - locals.var_qt1);
        let assign13760_e19037: f64 = (assign13760_e19035).powf(p.p1893);
        let assign13760_e19040: f64 = (924000.0 - 18100.0);
        let assign13760_e19041: f64 = (assign13760_e19037 * assign13760_e19040);
        let assign13760_e19044: f64 = (2.0_f64).powf(p.p1893);
        let assign13760_e19045: f64 = (assign13760_e19041 / assign13760_e19044);
        let assign13760_e19046: f64 = (locals.var_t0 + assign13760_e19045);
        let assign13760_e19048: f64 = (assign13760_e19046 - 18100.0);
        let assign13760_e19052: f64 = (p.p40 * 1000000000.0);
        let assign13760_e19054: f64 = (assign13760_e19052 - locals.var_qt1);
        let assign13760_e19056: f64 = (assign13760_e19054).powf(p.p1893);
        let assign13760_e19059: f64 = (924000.0 - 18100.0);
        let assign13760_e19060: f64 = (assign13760_e19056 * assign13760_e19059);
        let assign13760_e19063: f64 = (2.0_f64).powf(p.p1893);
        let assign13760_e19064: f64 = (assign13760_e19060 / assign13760_e19063);
        let assign13760_e19065: f64 = (locals.var_t0 + assign13760_e19064);
        let assign13760_e19067: f64 = (assign13760_e19065 - 18100.0);
        let assign13760_e19068: f64 = (assign13760_e19048 * assign13760_e19067);
        let assign13760_e19071: f64 = (0.25 * 0.01);
        let assign13760_e19073: f64 = (assign13760_e19071 * 0.01);
        let assign13760_e19074: f64 = (assign13760_e19068 + assign13760_e19073);
        let assign13760_e19075: f64 = (assign13760_e19074).sqrt();
        let assign13760_e19076: f64 = (assign13760_e19029 + assign13760_e19075);
        let assign13760_e19077: f64 = (0.5 * assign13760_e19076);
        let assign13760_e19079: f64 = (assign13760_e19077 - 924000.0);
        let assign13760_e19084: f64 = (p.p40 * 1000000000.0);
        let assign13760_e19086: f64 = (assign13760_e19084 - locals.var_qt1);
        let assign13760_e19088: f64 = (assign13760_e19086).powf(p.p1893);
        let assign13760_e19091: f64 = (924000.0 - 18100.0);
        let assign13760_e19092: f64 = (assign13760_e19088 * assign13760_e19091);
        let assign13760_e19095: f64 = (2.0_f64).powf(p.p1893);
        let assign13760_e19096: f64 = (assign13760_e19092 / assign13760_e19095);
        let assign13760_e19097: f64 = (locals.var_t0 + assign13760_e19096);
        let assign13760_e19099: f64 = (assign13760_e19097 + 18100.0);
        let assign13760_e19103: f64 = (p.p40 * 1000000000.0);
        let assign13760_e19105: f64 = (assign13760_e19103 - locals.var_qt1);
        let assign13760_e19107: f64 = (assign13760_e19105).powf(p.p1893);
        let assign13760_e19110: f64 = (924000.0 - 18100.0);
        let assign13760_e19111: f64 = (assign13760_e19107 * assign13760_e19110);
        let assign13760_e19114: f64 = (2.0_f64).powf(p.p1893);
        let assign13760_e19115: f64 = (assign13760_e19111 / assign13760_e19114);
        let assign13760_e19116: f64 = (locals.var_t0 + assign13760_e19115);
        let assign13760_e19118: f64 = (assign13760_e19116 - 18100.0);
        let assign13760_e19122: f64 = (p.p40 * 1000000000.0);
        let assign13760_e19124: f64 = (assign13760_e19122 - locals.var_qt1);
        let assign13760_e19126: f64 = (assign13760_e19124).powf(p.p1893);
        let assign13760_e19129: f64 = (924000.0 - 18100.0);
        let assign13760_e19130: f64 = (assign13760_e19126 * assign13760_e19129);
        let assign13760_e19133: f64 = (2.0_f64).powf(p.p1893);
        let assign13760_e19134: f64 = (assign13760_e19130 / assign13760_e19133);
        let assign13760_e19135: f64 = (locals.var_t0 + assign13760_e19134);
        let assign13760_e19137: f64 = (assign13760_e19135 - 18100.0);
        let assign13760_e19138: f64 = (assign13760_e19118 * assign13760_e19137);
        let assign13760_e19141: f64 = (0.25 * 0.01);
        let assign13760_e19143: f64 = (assign13760_e19141 * 0.01);
        let assign13760_e19144: f64 = (assign13760_e19138 + assign13760_e19143);
        let assign13760_e19145: f64 = (assign13760_e19144).sqrt();
        let assign13760_e19146: f64 = (assign13760_e19099 + assign13760_e19145);
        let assign13760_e19147: f64 = (0.5 * assign13760_e19146);
        let assign13760_e19149: f64 = (assign13760_e19147 - 924000.0);
        let assign13760_e19150: f64 = (assign13760_e19079 * assign13760_e19149);
        let assign13760_e19153: f64 = (0.25 * 9240.0);
        let assign13760_e19155: f64 = (assign13760_e19153 * 9240.0);
        let assign13760_e19156: f64 = (assign13760_e19150 + assign13760_e19155);
        let assign13760_e19157: f64 = (assign13760_e19156).sqrt();
        let assign13760_e19158: f64 = (assign13760_e19009 - assign13760_e19157);
        let assign13760_e19159: f64 = (0.5 * assign13760_e19158);
        let assign13760_e19162: f64 = (0.25 * 9240.0);
        let assign13760_e19163: f64 = (assign13760_e19159 + assign13760_e19162);
        (assign13760_e19163, (0.5 * ((0.5 * (locals.var_t0_dn0 + (((locals.var_t0_dn0 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn0)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn0 + (((locals.var_t0_dn0 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn0)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn0 + (((locals.var_t0_dn0 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn0)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn2 + (((locals.var_t0_dn2 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn2)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn2 + (((locals.var_t0_dn2 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn2)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn2 + (((locals.var_t0_dn2 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn2)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn3)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn3)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn3 + (((locals.var_t0_dn3 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn3)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn4)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn4)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn4 + (((locals.var_t0_dn4 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn4)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn5)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn5)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn5 + (((locals.var_t0_dn5 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn5)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn6)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn6)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn6 + (((locals.var_t0_dn6 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn6)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn7)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn7)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn7 + (((locals.var_t0_dn7 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn7)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn8)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn8)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn8 + (((locals.var_t0_dn8 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn8)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn9)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn9)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn9 + (((locals.var_t0_dn9 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn9)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn10)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn10)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn10 + (((locals.var_t0_dn10 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn10)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn11)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn11)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn11 + (((locals.var_t0_dn11 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn11)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn13 + (((locals.var_t0_dn13 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn13)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn13 + (((locals.var_t0_dn13 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn13)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn13 + (((locals.var_t0_dn13 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn13)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))), (0.5 * ((0.5 * (locals.var_t0_dn14 + (((locals.var_t0_dn14 * assign13760_e18997) + (assign13760_e18978 * locals.var_t0_dn14)) / (2.0 * assign13760_e19005)))) - ((((0.5 * (locals.var_t0_dn14 + (((locals.var_t0_dn14 * assign13760_e19067) + (assign13760_e19048 * locals.var_t0_dn14)) / (2.0 * assign13760_e19075)))) * assign13760_e19149) + (assign13760_e19079 * (0.5 * (locals.var_t0_dn14 + (((locals.var_t0_dn14 * assign13760_e19137) + (assign13760_e19118 * locals.var_t0_dn14)) / (2.0 * assign13760_e19145)))))) / (2.0 * assign13760_e19157)))),)
    } else {
        (locals.var_ne2h, locals.var_ne2h_dn0, locals.var_ne2h_dn2, locals.var_ne2h_dn3, locals.var_ne2h_dn4, locals.var_ne2h_dn5, locals.var_ne2h_dn6, locals.var_ne2h_dn7, locals.var_ne2h_dn8, locals.var_ne2h_dn9, locals.var_ne2h_dn10, locals.var_ne2h_dn11, locals.var_ne2h_dn13, locals.var_ne2h_dn14,)
    }
};
        locals.var_ne2h = assign13760_e19165;
        locals.var_ne2h_dn0 = assign13760_e19165_d_n0;
        locals.var_ne2h_dn2 = assign13760_e19165_d_n2;
        locals.var_ne2h_dn3 = assign13760_e19165_d_n3;
        locals.var_ne2h_dn4 = assign13760_e19165_d_n4;
        locals.var_ne2h_dn5 = assign13760_e19165_d_n5;
        locals.var_ne2h_dn6 = assign13760_e19165_d_n6;
        locals.var_ne2h_dn7 = assign13760_e19165_d_n7;
        locals.var_ne2h_dn8 = assign13760_e19165_d_n8;
        locals.var_ne2h_dn9 = assign13760_e19165_d_n9;
        locals.var_ne2h_dn10 = assign13760_e19165_d_n10;
        locals.var_ne2h_dn11 = assign13760_e19165_d_n11;
        locals.var_ne2h_dn13 = assign13760_e19165_d_n13;
        locals.var_ne2h_dn14 = assign13760_e19165_d_n14;

        let (assign13770_e19393,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13770_e19172: f64 = (p.p40 * 1000000000.0);
        let assign13770_e19174: f64 = (assign13770_e19172 - locals.var_qt1);
        let assign13770_e19176: f64 = (assign13770_e19174).powf(p.p1894);
        let assign13770_e19179: f64 = (8.0 - 5.5);
        let assign13770_e19180: f64 = (assign13770_e19176 * assign13770_e19179);
        let assign13770_e19183: f64 = (2.0_f64).powf(p.p1894);
        let assign13770_e19184: f64 = (assign13770_e19180 / assign13770_e19183);
        let assign13770_e19185: f64 = (5.5 + assign13770_e19184);
        let assign13770_e19187: f64 = assign13770_e19185;
        let assign13770_e19191: f64 = (p.p40 * 1000000000.0);
        let assign13770_e19193: f64 = (assign13770_e19191 - locals.var_qt1);
        let assign13770_e19195: f64 = (assign13770_e19193).powf(p.p1894);
        let assign13770_e19198: f64 = (8.0 - 5.5);
        let assign13770_e19199: f64 = (assign13770_e19195 * assign13770_e19198);
        let assign13770_e19202: f64 = (2.0_f64).powf(p.p1894);
        let assign13770_e19203: f64 = (assign13770_e19199 / assign13770_e19202);
        let assign13770_e19204: f64 = (5.5 + assign13770_e19203);
        let assign13770_e19206: f64 = assign13770_e19204;
        let assign13770_e19210: f64 = (p.p40 * 1000000000.0);
        let assign13770_e19212: f64 = (assign13770_e19210 - locals.var_qt1);
        let assign13770_e19214: f64 = (assign13770_e19212).powf(p.p1894);
        let assign13770_e19217: f64 = (8.0 - 5.5);
        let assign13770_e19218: f64 = (assign13770_e19214 * assign13770_e19217);
        let assign13770_e19221: f64 = (2.0_f64).powf(p.p1894);
        let assign13770_e19222: f64 = (assign13770_e19218 / assign13770_e19221);
        let assign13770_e19223: f64 = (5.5 + assign13770_e19222);
        let assign13770_e19225: f64 = assign13770_e19223;
        let assign13770_e19226: f64 = (assign13770_e19206 * assign13770_e19225);
        let assign13770_e19229: f64 = (0.25 * 0.01);
        let assign13770_e19231: f64 = (assign13770_e19229 * 0.01);
        let assign13770_e19232: f64 = (assign13770_e19226 + assign13770_e19231);
        let assign13770_e19233: f64 = (assign13770_e19232).sqrt();
        let assign13770_e19234: f64 = (assign13770_e19187 + assign13770_e19233);
        let assign13770_e19235: f64 = (0.5 * assign13770_e19234);
        let assign13770_e19237: f64 = (assign13770_e19235 + 8.0);
        let assign13770_e19242: f64 = (p.p40 * 1000000000.0);
        let assign13770_e19244: f64 = (assign13770_e19242 - locals.var_qt1);
        let assign13770_e19246: f64 = (assign13770_e19244).powf(p.p1894);
        let assign13770_e19249: f64 = (8.0 - 5.5);
        let assign13770_e19250: f64 = (assign13770_e19246 * assign13770_e19249);
        let assign13770_e19253: f64 = (2.0_f64).powf(p.p1894);
        let assign13770_e19254: f64 = (assign13770_e19250 / assign13770_e19253);
        let assign13770_e19255: f64 = (5.5 + assign13770_e19254);
        let assign13770_e19257: f64 = assign13770_e19255;
        let assign13770_e19261: f64 = (p.p40 * 1000000000.0);
        let assign13770_e19263: f64 = (assign13770_e19261 - locals.var_qt1);
        let assign13770_e19265: f64 = (assign13770_e19263).powf(p.p1894);
        let assign13770_e19268: f64 = (8.0 - 5.5);
        let assign13770_e19269: f64 = (assign13770_e19265 * assign13770_e19268);
        let assign13770_e19272: f64 = (2.0_f64).powf(p.p1894);
        let assign13770_e19273: f64 = (assign13770_e19269 / assign13770_e19272);
        let assign13770_e19274: f64 = (5.5 + assign13770_e19273);
        let assign13770_e19276: f64 = assign13770_e19274;
        let assign13770_e19280: f64 = (p.p40 * 1000000000.0);
        let assign13770_e19282: f64 = (assign13770_e19280 - locals.var_qt1);
        let assign13770_e19284: f64 = (assign13770_e19282).powf(p.p1894);
        let assign13770_e19287: f64 = (8.0 - 5.5);
        let assign13770_e19288: f64 = (assign13770_e19284 * assign13770_e19287);
        let assign13770_e19291: f64 = (2.0_f64).powf(p.p1894);
        let assign13770_e19292: f64 = (assign13770_e19288 / assign13770_e19291);
        let assign13770_e19293: f64 = (5.5 + assign13770_e19292);
        let assign13770_e19295: f64 = assign13770_e19293;
        let assign13770_e19296: f64 = (assign13770_e19276 * assign13770_e19295);
        let assign13770_e19299: f64 = (0.25 * 0.01);
        let assign13770_e19301: f64 = (assign13770_e19299 * 0.01);
        let assign13770_e19302: f64 = (assign13770_e19296 + assign13770_e19301);
        let assign13770_e19303: f64 = (assign13770_e19302).sqrt();
        let assign13770_e19304: f64 = (assign13770_e19257 + assign13770_e19303);
        let assign13770_e19305: f64 = (0.5 * assign13770_e19304);
        let assign13770_e19307: f64 = (assign13770_e19305 - 8.0);
        let assign13770_e19312: f64 = (p.p40 * 1000000000.0);
        let assign13770_e19314: f64 = (assign13770_e19312 - locals.var_qt1);
        let assign13770_e19316: f64 = (assign13770_e19314).powf(p.p1894);
        let assign13770_e19319: f64 = (8.0 - 5.5);
        let assign13770_e19320: f64 = (assign13770_e19316 * assign13770_e19319);
        let assign13770_e19323: f64 = (2.0_f64).powf(p.p1894);
        let assign13770_e19324: f64 = (assign13770_e19320 / assign13770_e19323);
        let assign13770_e19325: f64 = (5.5 + assign13770_e19324);
        let assign13770_e19327: f64 = assign13770_e19325;
        let assign13770_e19331: f64 = (p.p40 * 1000000000.0);
        let assign13770_e19333: f64 = (assign13770_e19331 - locals.var_qt1);
        let assign13770_e19335: f64 = (assign13770_e19333).powf(p.p1894);
        let assign13770_e19338: f64 = (8.0 - 5.5);
        let assign13770_e19339: f64 = (assign13770_e19335 * assign13770_e19338);
        let assign13770_e19342: f64 = (2.0_f64).powf(p.p1894);
        let assign13770_e19343: f64 = (assign13770_e19339 / assign13770_e19342);
        let assign13770_e19344: f64 = (5.5 + assign13770_e19343);
        let assign13770_e19346: f64 = assign13770_e19344;
        let assign13770_e19350: f64 = (p.p40 * 1000000000.0);
        let assign13770_e19352: f64 = (assign13770_e19350 - locals.var_qt1);
        let assign13770_e19354: f64 = (assign13770_e19352).powf(p.p1894);
        let assign13770_e19357: f64 = (8.0 - 5.5);
        let assign13770_e19358: f64 = (assign13770_e19354 * assign13770_e19357);
        let assign13770_e19361: f64 = (2.0_f64).powf(p.p1894);
        let assign13770_e19362: f64 = (assign13770_e19358 / assign13770_e19361);
        let assign13770_e19363: f64 = (5.5 + assign13770_e19362);
        let assign13770_e19365: f64 = assign13770_e19363;
        let assign13770_e19366: f64 = (assign13770_e19346 * assign13770_e19365);
        let assign13770_e19369: f64 = (0.25 * 0.01);
        let assign13770_e19371: f64 = (assign13770_e19369 * 0.01);
        let assign13770_e19372: f64 = (assign13770_e19366 + assign13770_e19371);
        let assign13770_e19373: f64 = (assign13770_e19372).sqrt();
        let assign13770_e19374: f64 = (assign13770_e19327 + assign13770_e19373);
        let assign13770_e19375: f64 = (0.5 * assign13770_e19374);
        let assign13770_e19377: f64 = (assign13770_e19375 - 8.0);
        let assign13770_e19378: f64 = (assign13770_e19307 * assign13770_e19377);
        let assign13770_e19381: f64 = (0.25 * 0.01);
        let assign13770_e19383: f64 = (assign13770_e19381 * 0.01);
        let assign13770_e19384: f64 = (assign13770_e19378 + assign13770_e19383);
        let assign13770_e19385: f64 = (assign13770_e19384).sqrt();
        let assign13770_e19386: f64 = (assign13770_e19237 - assign13770_e19385);
        let assign13770_e19387: f64 = (0.5 * assign13770_e19386);
        let assign13770_e19390: f64 = (0.25 * 0.01);
        let assign13770_e19391: f64 = (assign13770_e19387 + assign13770_e19390);
        (assign13770_e19391,)
    } else {
        (locals.var_pe2h,)
    }
};
        locals.var_pe2h = assign13770_e19393;

        let (assign13780_e19407,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13780_e19398: f64 = (4.0_f64).powf(p.p1895);
        let assign13780_e19399: f64 = (120.66 * assign13780_e19398);
        let assign13780_e19402: f64 = (p.p40 * 1000000000.0);
        let assign13780_e19404: f64 = (assign13780_e19402).powf(p.p1895);
        let assign13780_e19405: f64 = (assign13780_e19399 / assign13780_e19404);
        (assign13780_e19405,)
    } else {
        (locals.var_ne3h,)
    }
};
        locals.var_ne3h = assign13780_e19407;

        let (assign13790_e19421,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13790_e19412: f64 = (4.0_f64).powf(p.p1896);
        let assign13790_e19413: f64 = (2.0 * assign13790_e19412);
        let assign13790_e19416: f64 = (p.p40 * 1000000000.0);
        let assign13790_e19418: f64 = (assign13790_e19416).powf(p.p1896);
        let assign13790_e19419: f64 = (assign13790_e19413 / assign13790_e19418);
        (assign13790_e19419,)
    } else {
        (locals.var_pe3h,)
    }
};
        locals.var_pe3h = assign13790_e19421;

        let (assign13800_e19435,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13800_e19426: f64 = (4.0_f64).powf(p.p1897);
        let assign13800_e19427: f64 = (107.0 * assign13800_e19426);
        let assign13800_e19430: f64 = (p.p40 * 1000000000.0);
        let assign13800_e19432: f64 = (assign13800_e19430).powf(p.p1897);
        let assign13800_e19433: f64 = (assign13800_e19427 / assign13800_e19432);
        (assign13800_e19433,)
    } else {
        (locals.var_nc1l0,)
    }
};
        locals.var_nc1l0 = assign13800_e19435;

        let (assign13810_e19609,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13810_e19442: f64 = (p.p40 * 1000000000.0);
        let assign13810_e19444: f64 = (assign13810_e19442 - locals.var_qt1);
        let assign13810_e19446: f64 = (assign13810_e19444).powf(p.p1898);
        let assign13810_e19448: f64 = (assign13810_e19446 * 0.1);
        let assign13810_e19449: f64 = (0.7 + assign13810_e19448);
        let assign13810_e19451: f64 = (assign13810_e19449 + 0.5);
        let assign13810_e19455: f64 = (p.p40 * 1000000000.0);
        let assign13810_e19457: f64 = (assign13810_e19455 - locals.var_qt1);
        let assign13810_e19459: f64 = (assign13810_e19457).powf(p.p1898);
        let assign13810_e19461: f64 = (assign13810_e19459 * 0.1);
        let assign13810_e19462: f64 = (0.7 + assign13810_e19461);
        let assign13810_e19464: f64 = (assign13810_e19462 - 0.5);
        let assign13810_e19468: f64 = (p.p40 * 1000000000.0);
        let assign13810_e19470: f64 = (assign13810_e19468 - locals.var_qt1);
        let assign13810_e19472: f64 = (assign13810_e19470).powf(p.p1898);
        let assign13810_e19474: f64 = (assign13810_e19472 * 0.1);
        let assign13810_e19475: f64 = (0.7 + assign13810_e19474);
        let assign13810_e19477: f64 = (assign13810_e19475 - 0.5);
        let assign13810_e19478: f64 = (assign13810_e19464 * assign13810_e19477);
        let assign13810_e19481: f64 = (0.25 * 0.01);
        let assign13810_e19483: f64 = (assign13810_e19481 * 0.01);
        let assign13810_e19484: f64 = (assign13810_e19478 + assign13810_e19483);
        let assign13810_e19485: f64 = (assign13810_e19484).sqrt();
        let assign13810_e19486: f64 = (assign13810_e19451 + assign13810_e19485);
        let assign13810_e19487: f64 = (0.5 * assign13810_e19486);
        let assign13810_e19489: f64 = (assign13810_e19487 + 1.0);
        let assign13810_e19494: f64 = (p.p40 * 1000000000.0);
        let assign13810_e19496: f64 = (assign13810_e19494 - locals.var_qt1);
        let assign13810_e19498: f64 = (assign13810_e19496).powf(p.p1898);
        let assign13810_e19500: f64 = (assign13810_e19498 * 0.1);
        let assign13810_e19501: f64 = (0.7 + assign13810_e19500);
        let assign13810_e19503: f64 = (assign13810_e19501 + 0.5);
        let assign13810_e19507: f64 = (p.p40 * 1000000000.0);
        let assign13810_e19509: f64 = (assign13810_e19507 - locals.var_qt1);
        let assign13810_e19511: f64 = (assign13810_e19509).powf(p.p1898);
        let assign13810_e19513: f64 = (assign13810_e19511 * 0.1);
        let assign13810_e19514: f64 = (0.7 + assign13810_e19513);
        let assign13810_e19516: f64 = (assign13810_e19514 - 0.5);
        let assign13810_e19520: f64 = (p.p40 * 1000000000.0);
        let assign13810_e19522: f64 = (assign13810_e19520 - locals.var_qt1);
        let assign13810_e19524: f64 = (assign13810_e19522).powf(p.p1898);
        let assign13810_e19526: f64 = (assign13810_e19524 * 0.1);
        let assign13810_e19527: f64 = (0.7 + assign13810_e19526);
        let assign13810_e19529: f64 = (assign13810_e19527 - 0.5);
        let assign13810_e19530: f64 = (assign13810_e19516 * assign13810_e19529);
        let assign13810_e19533: f64 = (0.25 * 0.01);
        let assign13810_e19535: f64 = (assign13810_e19533 * 0.01);
        let assign13810_e19536: f64 = (assign13810_e19530 + assign13810_e19535);
        let assign13810_e19537: f64 = (assign13810_e19536).sqrt();
        let assign13810_e19538: f64 = (assign13810_e19503 + assign13810_e19537);
        let assign13810_e19539: f64 = (0.5 * assign13810_e19538);
        let assign13810_e19541: f64 = (assign13810_e19539 - 1.0);
        let assign13810_e19546: f64 = (p.p40 * 1000000000.0);
        let assign13810_e19548: f64 = (assign13810_e19546 - locals.var_qt1);
        let assign13810_e19550: f64 = (assign13810_e19548).powf(p.p1898);
        let assign13810_e19552: f64 = (assign13810_e19550 * 0.1);
        let assign13810_e19553: f64 = (0.7 + assign13810_e19552);
        let assign13810_e19555: f64 = (assign13810_e19553 + 0.5);
        let assign13810_e19559: f64 = (p.p40 * 1000000000.0);
        let assign13810_e19561: f64 = (assign13810_e19559 - locals.var_qt1);
        let assign13810_e19563: f64 = (assign13810_e19561).powf(p.p1898);
        let assign13810_e19565: f64 = (assign13810_e19563 * 0.1);
        let assign13810_e19566: f64 = (0.7 + assign13810_e19565);
        let assign13810_e19568: f64 = (assign13810_e19566 - 0.5);
        let assign13810_e19572: f64 = (p.p40 * 1000000000.0);
        let assign13810_e19574: f64 = (assign13810_e19572 - locals.var_qt1);
        let assign13810_e19576: f64 = (assign13810_e19574).powf(p.p1898);
        let assign13810_e19578: f64 = (assign13810_e19576 * 0.1);
        let assign13810_e19579: f64 = (0.7 + assign13810_e19578);
        let assign13810_e19581: f64 = (assign13810_e19579 - 0.5);
        let assign13810_e19582: f64 = (assign13810_e19568 * assign13810_e19581);
        let assign13810_e19585: f64 = (0.25 * 0.01);
        let assign13810_e19587: f64 = (assign13810_e19585 * 0.01);
        let assign13810_e19588: f64 = (assign13810_e19582 + assign13810_e19587);
        let assign13810_e19589: f64 = (assign13810_e19588).sqrt();
        let assign13810_e19590: f64 = (assign13810_e19555 + assign13810_e19589);
        let assign13810_e19591: f64 = (0.5 * assign13810_e19590);
        let assign13810_e19593: f64 = (assign13810_e19591 - 1.0);
        let assign13810_e19594: f64 = (assign13810_e19541 * assign13810_e19593);
        let assign13810_e19597: f64 = (0.25 * 0.01);
        let assign13810_e19599: f64 = (assign13810_e19597 * 0.01);
        let assign13810_e19600: f64 = (assign13810_e19594 + assign13810_e19599);
        let assign13810_e19601: f64 = (assign13810_e19600).sqrt();
        let assign13810_e19602: f64 = (assign13810_e19489 - assign13810_e19601);
        let assign13810_e19603: f64 = (0.5 * assign13810_e19602);
        let assign13810_e19606: f64 = (0.25 * 0.01);
        let assign13810_e19607: f64 = (assign13810_e19603 + assign13810_e19606);
        (assign13810_e19607,)
    } else {
        (locals.var_pnc1l,)
    }
};
        locals.var_pnc1l = assign13810_e19609;

    }

    pub(super) fn stamp_transient_block_37(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13820_e19623,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13820_e19614: f64 = (4.0_f64).powf(p.p1899);
        let assign13820_e19615: f64 = (103.0 * assign13820_e19614);
        let assign13820_e19618: f64 = (p.p40 * 1000000000.0);
        let assign13820_e19620: f64 = (assign13820_e19618).powf(p.p1899);
        let assign13820_e19621: f64 = (assign13820_e19615 / assign13820_e19620);
        (assign13820_e19621,)
    } else {
        (locals.var_nc2l0,)
    }
};
        locals.var_nc2l0 = assign13820_e19623;

        let (assign13830_e19637,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13830_e19628: f64 = (4.0_f64).powf(p.p1900);
        let assign13830_e19629: f64 = (1.5 * assign13830_e19628);
        let assign13830_e19632: f64 = (p.p40 * 1000000000.0);
        let assign13830_e19634: f64 = (assign13830_e19632).powf(p.p1900);
        let assign13830_e19635: f64 = (assign13830_e19629 / assign13830_e19634);
        (assign13830_e19635,)
    } else {
        (locals.var_pnc2l,)
    }
};
        locals.var_pnc2l = assign13830_e19637;

        let (assign13840_e19651,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13840_e19642: f64 = (4.0_f64).powf(p.p1901);
        let assign13840_e19643: f64 = (833.0 * assign13840_e19642);
        let assign13840_e19646: f64 = (p.p40 * 1000000000.0);
        let assign13840_e19648: f64 = (assign13840_e19646).powf(p.p1901);
        let assign13840_e19649: f64 = (assign13840_e19643 / assign13840_e19648);
        (assign13840_e19649,)
    } else {
        (locals.var_nc3l0,)
    }
};
        locals.var_nc3l0 = assign13840_e19651;

        let (assign13850_e19665,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13850_e19656: f64 = (4.0_f64).powf(p.p1902);
        let assign13850_e19657: f64 = (3.4 * assign13850_e19656);
        let assign13850_e19660: f64 = (p.p40 * 1000000000.0);
        let assign13850_e19662: f64 = (assign13850_e19660).powf(p.p1902);
        let assign13850_e19663: f64 = (assign13850_e19657 / assign13850_e19662);
        (assign13850_e19663,)
    } else {
        (locals.var_pnc3l,)
    }
};
        locals.var_pnc3l = assign13850_e19665;

        let (assign13860_e19677, assign13860_e19677_d_n0, assign13860_e19677_d_n2, assign13860_e19677_d_n3, assign13860_e19677_d_n4, assign13860_e19677_d_n5, assign13860_e19677_d_n6, assign13860_e19677_d_n7, assign13860_e19677_d_n8, assign13860_e19677_d_n9, assign13860_e19677_d_n10, assign13860_e19677_d_n11, assign13860_e19677_d_n13, assign13860_e19677_d_n14,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13860_e19670: f64 = (p.p1852 * 1000000000.0);
        let assign13860_e19673: f64 = (p.p1867 * locals.var_pe2h);
        let assign13860_e19674: f64 = (assign13860_e19670).powf(assign13860_e19673);
        let assign13860_e19675: f64 = (locals.var_ne2h / assign13860_e19674);
        (assign13860_e19675, (locals.var_ne2h_dn0 / assign13860_e19674), (locals.var_ne2h_dn2 / assign13860_e19674), (locals.var_ne2h_dn3 / assign13860_e19674), (locals.var_ne2h_dn4 / assign13860_e19674), (locals.var_ne2h_dn5 / assign13860_e19674), (locals.var_ne2h_dn6 / assign13860_e19674), (locals.var_ne2h_dn7 / assign13860_e19674), (locals.var_ne2h_dn8 / assign13860_e19674), (locals.var_ne2h_dn9 / assign13860_e19674), (locals.var_ne2h_dn10 / assign13860_e19674), (locals.var_ne2h_dn11 / assign13860_e19674), (locals.var_ne2h_dn13 / assign13860_e19674), (locals.var_ne2h_dn14 / assign13860_e19674),)
    } else {
        (locals.var_qe2n, locals.var_qe2n_dn0, locals.var_qe2n_dn2, locals.var_qe2n_dn3, locals.var_qe2n_dn4, locals.var_qe2n_dn5, locals.var_qe2n_dn6, locals.var_qe2n_dn7, locals.var_qe2n_dn8, locals.var_qe2n_dn9, locals.var_qe2n_dn10, locals.var_qe2n_dn11, locals.var_qe2n_dn13, locals.var_qe2n_dn14,)
    }
};
        locals.var_qe2n = assign13860_e19677;
        locals.var_qe2n_dn0 = assign13860_e19677_d_n0;
        locals.var_qe2n_dn2 = assign13860_e19677_d_n2;
        locals.var_qe2n_dn3 = assign13860_e19677_d_n3;
        locals.var_qe2n_dn4 = assign13860_e19677_d_n4;
        locals.var_qe2n_dn5 = assign13860_e19677_d_n5;
        locals.var_qe2n_dn6 = assign13860_e19677_d_n6;
        locals.var_qe2n_dn7 = assign13860_e19677_d_n7;
        locals.var_qe2n_dn8 = assign13860_e19677_d_n8;
        locals.var_qe2n_dn9 = assign13860_e19677_d_n9;
        locals.var_qe2n_dn10 = assign13860_e19677_d_n10;
        locals.var_qe2n_dn11 = assign13860_e19677_d_n11;
        locals.var_qe2n_dn13 = assign13860_e19677_d_n13;
        locals.var_qe2n_dn14 = assign13860_e19677_d_n14;

        let (assign13870_e19689,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13870_e19682: f64 = (p.p1852 * 1000000000.0);
        let assign13870_e19685: f64 = (p.p1868 * locals.var_pe3h);
        let assign13870_e19686: f64 = (assign13870_e19682).powf(assign13870_e19685);
        let assign13870_e19687: f64 = (locals.var_ne3h / assign13870_e19686);
        (assign13870_e19687,)
    } else {
        (locals.var_qe3n,)
    }
};
        locals.var_qe3n = assign13870_e19689;

        let (assign13880_e19754, assign13880_e19754_d_n0, assign13880_e19754_d_n2, assign13880_e19754_d_n3, assign13880_e19754_d_n4, assign13880_e19754_d_n5, assign13880_e19754_d_n6, assign13880_e19754_d_n7, assign13880_e19754_d_n8, assign13880_e19754_d_n9, assign13880_e19754_d_n10, assign13880_e19754_d_n11, assign13880_e19754_d_n13, assign13880_e19754_d_n14,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13880_e19697: f64 = (p.p43 * 1000000000.0);
        let assign13880_e19700: f64 = (p.p1867 * locals.var_pe2h);
        let assign13880_e19701: f64 = (assign13880_e19697).powf(assign13880_e19700);
        let assign13880_e19702: f64 = (locals.var_ne2h / assign13880_e19701);
        let assign13880_e19704: f64 = (assign13880_e19702 - locals.var_qe2n);
        let assign13880_e19705: f64 = (p.p1865 * assign13880_e19704);
        let assign13880_e19706: f64 = (locals.var_e2nom_i + assign13880_e19705);
        let assign13880_e19708: f64 = assign13880_e19706;
        let assign13880_e19714: f64 = (p.p43 * 1000000000.0);
        let assign13880_e19717: f64 = (p.p1867 * locals.var_pe2h);
        let assign13880_e19718: f64 = (assign13880_e19714).powf(assign13880_e19717);
        let assign13880_e19719: f64 = (locals.var_ne2h / assign13880_e19718);
        let assign13880_e19721: f64 = (assign13880_e19719 - locals.var_qe2n);
        let assign13880_e19722: f64 = (p.p1865 * assign13880_e19721);
        let assign13880_e19723: f64 = (locals.var_e2nom_i + assign13880_e19722);
        let assign13880_e19725: f64 = assign13880_e19723;
        let assign13880_e19731: f64 = (p.p43 * 1000000000.0);
        let assign13880_e19734: f64 = (p.p1867 * locals.var_pe2h);
        let assign13880_e19735: f64 = (assign13880_e19731).powf(assign13880_e19734);
        let assign13880_e19736: f64 = (locals.var_ne2h / assign13880_e19735);
        let assign13880_e19738: f64 = (assign13880_e19736 - locals.var_qe2n);
        let assign13880_e19739: f64 = (p.p1865 * assign13880_e19738);
        let assign13880_e19740: f64 = (locals.var_e2nom_i + assign13880_e19739);
        let assign13880_e19742: f64 = assign13880_e19740;
        let assign13880_e19743: f64 = (assign13880_e19725 * assign13880_e19742);
        let assign13880_e19746: f64 = (0.25 * 0.01);
        let assign13880_e19748: f64 = (assign13880_e19746 * 0.01);
        let assign13880_e19749: f64 = (assign13880_e19743 + assign13880_e19748);
        let assign13880_e19750: f64 = (assign13880_e19749).sqrt();
        let assign13880_e19751: f64 = (assign13880_e19708 + assign13880_e19750);
        let assign13880_e19752: f64 = (0.5 * assign13880_e19751);
        (assign13880_e19752, (0.5 * ((p.p1865 * ((locals.var_ne2h_dn0 / assign13880_e19701) - locals.var_qe2n_dn0)) + ((((p.p1865 * ((locals.var_ne2h_dn0 / assign13880_e19718) - locals.var_qe2n_dn0)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn0 / assign13880_e19735) - locals.var_qe2n_dn0)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn2 / assign13880_e19701) - locals.var_qe2n_dn2)) + ((((p.p1865 * ((locals.var_ne2h_dn2 / assign13880_e19718) - locals.var_qe2n_dn2)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn2 / assign13880_e19735) - locals.var_qe2n_dn2)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn3 / assign13880_e19701) - locals.var_qe2n_dn3)) + ((((p.p1865 * ((locals.var_ne2h_dn3 / assign13880_e19718) - locals.var_qe2n_dn3)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn3 / assign13880_e19735) - locals.var_qe2n_dn3)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn4 / assign13880_e19701) - locals.var_qe2n_dn4)) + ((((p.p1865 * ((locals.var_ne2h_dn4 / assign13880_e19718) - locals.var_qe2n_dn4)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn4 / assign13880_e19735) - locals.var_qe2n_dn4)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn5 / assign13880_e19701) - locals.var_qe2n_dn5)) + ((((p.p1865 * ((locals.var_ne2h_dn5 / assign13880_e19718) - locals.var_qe2n_dn5)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn5 / assign13880_e19735) - locals.var_qe2n_dn5)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn6 / assign13880_e19701) - locals.var_qe2n_dn6)) + ((((p.p1865 * ((locals.var_ne2h_dn6 / assign13880_e19718) - locals.var_qe2n_dn6)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn6 / assign13880_e19735) - locals.var_qe2n_dn6)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn7 / assign13880_e19701) - locals.var_qe2n_dn7)) + ((((p.p1865 * ((locals.var_ne2h_dn7 / assign13880_e19718) - locals.var_qe2n_dn7)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn7 / assign13880_e19735) - locals.var_qe2n_dn7)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn8 / assign13880_e19701) - locals.var_qe2n_dn8)) + ((((p.p1865 * ((locals.var_ne2h_dn8 / assign13880_e19718) - locals.var_qe2n_dn8)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn8 / assign13880_e19735) - locals.var_qe2n_dn8)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn9 / assign13880_e19701) - locals.var_qe2n_dn9)) + ((((p.p1865 * ((locals.var_ne2h_dn9 / assign13880_e19718) - locals.var_qe2n_dn9)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn9 / assign13880_e19735) - locals.var_qe2n_dn9)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn10 / assign13880_e19701) - locals.var_qe2n_dn10)) + ((((p.p1865 * ((locals.var_ne2h_dn10 / assign13880_e19718) - locals.var_qe2n_dn10)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn10 / assign13880_e19735) - locals.var_qe2n_dn10)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn11 / assign13880_e19701) - locals.var_qe2n_dn11)) + ((((p.p1865 * ((locals.var_ne2h_dn11 / assign13880_e19718) - locals.var_qe2n_dn11)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn11 / assign13880_e19735) - locals.var_qe2n_dn11)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn13 / assign13880_e19701) - locals.var_qe2n_dn13)) + ((((p.p1865 * ((locals.var_ne2h_dn13 / assign13880_e19718) - locals.var_qe2n_dn13)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn13 / assign13880_e19735) - locals.var_qe2n_dn13)))) / (2.0 * assign13880_e19750)))), (0.5 * ((p.p1865 * ((locals.var_ne2h_dn14 / assign13880_e19701) - locals.var_qe2n_dn14)) + ((((p.p1865 * ((locals.var_ne2h_dn14 / assign13880_e19718) - locals.var_qe2n_dn14)) * assign13880_e19742) + (assign13880_e19725 * (p.p1865 * ((locals.var_ne2h_dn14 / assign13880_e19735) - locals.var_qe2n_dn14)))) / (2.0 * assign13880_e19750)))),)
    } else {
        (locals.var_qe2, locals.var_qe2_dn0, locals.var_qe2_dn2, locals.var_qe2_dn3, locals.var_qe2_dn4, locals.var_qe2_dn5, locals.var_qe2_dn6, locals.var_qe2_dn7, locals.var_qe2_dn8, locals.var_qe2_dn9, locals.var_qe2_dn10, locals.var_qe2_dn11, locals.var_qe2_dn13, locals.var_qe2_dn14,)
    }
};
        locals.var_qe2 = assign13880_e19754;
        locals.var_qe2_dn0 = assign13880_e19754_d_n0;
        locals.var_qe2_dn2 = assign13880_e19754_d_n2;
        locals.var_qe2_dn3 = assign13880_e19754_d_n3;
        locals.var_qe2_dn4 = assign13880_e19754_d_n4;
        locals.var_qe2_dn5 = assign13880_e19754_d_n5;
        locals.var_qe2_dn6 = assign13880_e19754_d_n6;
        locals.var_qe2_dn7 = assign13880_e19754_d_n7;
        locals.var_qe2_dn8 = assign13880_e19754_d_n8;
        locals.var_qe2_dn9 = assign13880_e19754_d_n9;
        locals.var_qe2_dn10 = assign13880_e19754_d_n10;
        locals.var_qe2_dn11 = assign13880_e19754_d_n11;
        locals.var_qe2_dn13 = assign13880_e19754_d_n13;
        locals.var_qe2_dn14 = assign13880_e19754_d_n14;

        let (assign13890_e19819,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13890_e19762: f64 = (p.p43 * 1000000000.0);
        let assign13890_e19765: f64 = (p.p1868 * locals.var_pe3h);
        let assign13890_e19766: f64 = (assign13890_e19762).powf(assign13890_e19765);
        let assign13890_e19767: f64 = (locals.var_ne3h / assign13890_e19766);
        let assign13890_e19769: f64 = (assign13890_e19767 - locals.var_qe3n);
        let assign13890_e19770: f64 = (p.p1866 * assign13890_e19769);
        let assign13890_e19771: f64 = (locals.var_e3nom_i + assign13890_e19770);
        let assign13890_e19773: f64 = assign13890_e19771;
        let assign13890_e19779: f64 = (p.p43 * 1000000000.0);
        let assign13890_e19782: f64 = (p.p1868 * locals.var_pe3h);
        let assign13890_e19783: f64 = (assign13890_e19779).powf(assign13890_e19782);
        let assign13890_e19784: f64 = (locals.var_ne3h / assign13890_e19783);
        let assign13890_e19786: f64 = (assign13890_e19784 - locals.var_qe3n);
        let assign13890_e19787: f64 = (p.p1866 * assign13890_e19786);
        let assign13890_e19788: f64 = (locals.var_e3nom_i + assign13890_e19787);
        let assign13890_e19790: f64 = assign13890_e19788;
        let assign13890_e19796: f64 = (p.p43 * 1000000000.0);
        let assign13890_e19799: f64 = (p.p1868 * locals.var_pe3h);
        let assign13890_e19800: f64 = (assign13890_e19796).powf(assign13890_e19799);
        let assign13890_e19801: f64 = (locals.var_ne3h / assign13890_e19800);
        let assign13890_e19803: f64 = (assign13890_e19801 - locals.var_qe3n);
        let assign13890_e19804: f64 = (p.p1866 * assign13890_e19803);
        let assign13890_e19805: f64 = (locals.var_e3nom_i + assign13890_e19804);
        let assign13890_e19807: f64 = assign13890_e19805;
        let assign13890_e19808: f64 = (assign13890_e19790 * assign13890_e19807);
        let assign13890_e19811: f64 = (0.25 * 0.01);
        let assign13890_e19813: f64 = (assign13890_e19811 * 0.01);
        let assign13890_e19814: f64 = (assign13890_e19808 + assign13890_e19813);
        let assign13890_e19815: f64 = (assign13890_e19814).sqrt();
        let assign13890_e19816: f64 = (assign13890_e19773 + assign13890_e19815);
        let assign13890_e19817: f64 = (0.5 * assign13890_e19816);
        (assign13890_e19817,)
    } else {
        (locals.var_qe3,)
    }
};
        locals.var_qe3 = assign13890_e19819;

        let (assign13900_e19884,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13900_e19827: f64 = (p.p43 * 1000000000.0);
        let assign13900_e19830: f64 = (p.p1890 * locals.var_pnc1l);
        let assign13900_e19831: f64 = (assign13900_e19827).powf(assign13900_e19830);
        let assign13900_e19832: f64 = (5.0 * assign13900_e19831);
        let assign13900_e19833: f64 = (1.0 + assign13900_e19832);
        let assign13900_e19835: f64 = (assign13900_e19833).powf(0.5);
        let assign13900_e19836: f64 = (locals.var_nc1l0 / assign13900_e19835);
        let assign13900_e19838: f64 = assign13900_e19836;
        let assign13900_e19844: f64 = (p.p43 * 1000000000.0);
        let assign13900_e19847: f64 = (p.p1890 * locals.var_pnc1l);
        let assign13900_e19848: f64 = (assign13900_e19844).powf(assign13900_e19847);
        let assign13900_e19849: f64 = (5.0 * assign13900_e19848);
        let assign13900_e19850: f64 = (1.0 + assign13900_e19849);
        let assign13900_e19852: f64 = (assign13900_e19850).powf(0.5);
        let assign13900_e19853: f64 = (locals.var_nc1l0 / assign13900_e19852);
        let assign13900_e19855: f64 = assign13900_e19853;
        let assign13900_e19861: f64 = (p.p43 * 1000000000.0);
        let assign13900_e19864: f64 = (p.p1890 * locals.var_pnc1l);
        let assign13900_e19865: f64 = (assign13900_e19861).powf(assign13900_e19864);
        let assign13900_e19866: f64 = (5.0 * assign13900_e19865);
        let assign13900_e19867: f64 = (1.0 + assign13900_e19866);
        let assign13900_e19869: f64 = (assign13900_e19867).powf(0.5);
        let assign13900_e19870: f64 = (locals.var_nc1l0 / assign13900_e19869);
        let assign13900_e19872: f64 = assign13900_e19870;
        let assign13900_e19873: f64 = (assign13900_e19855 * assign13900_e19872);
        let assign13900_e19876: f64 = (0.25 * 0.1);
        let assign13900_e19878: f64 = (assign13900_e19876 * 0.1);
        let assign13900_e19879: f64 = (assign13900_e19873 + assign13900_e19878);
        let assign13900_e19880: f64 = (assign13900_e19879).sqrt();
        let assign13900_e19881: f64 = (assign13900_e19838 + assign13900_e19880);
        let assign13900_e19882: f64 = (0.5 * assign13900_e19881);
        (assign13900_e19882,)
    } else {
        (locals.var_nc1l,)
    }
};
        locals.var_nc1l = assign13900_e19884;

        let (assign13910_e19949,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13910_e19892: f64 = (p.p1852 * 1000000000.0);
        let assign13910_e19895: f64 = (p.p1890 * locals.var_pnc1l);
        let assign13910_e19896: f64 = (assign13910_e19892).powf(assign13910_e19895);
        let assign13910_e19897: f64 = (5.0 * assign13910_e19896);
        let assign13910_e19898: f64 = (1.0 + assign13910_e19897);
        let assign13910_e19900: f64 = (assign13910_e19898).powf(0.5);
        let assign13910_e19901: f64 = (locals.var_nc1l0 / assign13910_e19900);
        let assign13910_e19903: f64 = assign13910_e19901;
        let assign13910_e19909: f64 = (p.p1852 * 1000000000.0);
        let assign13910_e19912: f64 = (p.p1890 * locals.var_pnc1l);
        let assign13910_e19913: f64 = (assign13910_e19909).powf(assign13910_e19912);
        let assign13910_e19914: f64 = (5.0 * assign13910_e19913);
        let assign13910_e19915: f64 = (1.0 + assign13910_e19914);
        let assign13910_e19917: f64 = (assign13910_e19915).powf(0.5);
        let assign13910_e19918: f64 = (locals.var_nc1l0 / assign13910_e19917);
        let assign13910_e19920: f64 = assign13910_e19918;
        let assign13910_e19926: f64 = (p.p1852 * 1000000000.0);
        let assign13910_e19929: f64 = (p.p1890 * locals.var_pnc1l);
        let assign13910_e19930: f64 = (assign13910_e19926).powf(assign13910_e19929);
        let assign13910_e19931: f64 = (5.0 * assign13910_e19930);
        let assign13910_e19932: f64 = (1.0 + assign13910_e19931);
        let assign13910_e19934: f64 = (assign13910_e19932).powf(0.5);
        let assign13910_e19935: f64 = (locals.var_nc1l0 / assign13910_e19934);
        let assign13910_e19937: f64 = assign13910_e19935;
        let assign13910_e19938: f64 = (assign13910_e19920 * assign13910_e19937);
        let assign13910_e19941: f64 = (0.25 * 0.1);
        let assign13910_e19943: f64 = (assign13910_e19941 * 0.1);
        let assign13910_e19944: f64 = (assign13910_e19938 + assign13910_e19943);
        let assign13910_e19945: f64 = (assign13910_e19944).sqrt();
        let assign13910_e19946: f64 = (assign13910_e19903 + assign13910_e19945);
        let assign13910_e19947: f64 = (0.5 * assign13910_e19946);
        (assign13910_e19947,)
    } else {
        (locals.var_nc1ln,)
    }
};
        locals.var_nc1ln = assign13910_e19949;

        let (assign13920_e19959,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13920_e19955: f64 = (locals.var_nc1l - locals.var_nc1ln);
        let assign13920_e19956: f64 = (p.p1887 * assign13920_e19955);
        let assign13920_e19957: f64 = (locals.var_mfq1nom_i + assign13920_e19956);
        (assign13920_e19957,)
    } else {
        (locals.var_nc1,)
    }
};
        locals.var_nc1 = assign13920_e19959;

        let (assign13930_e20024,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13930_e19967: f64 = (p.p43 * 1000000000.0);
        let assign13930_e19970: f64 = (p.p1891 * locals.var_pnc2l);
        let assign13930_e19971: f64 = (assign13930_e19967).powf(assign13930_e19970);
        let assign13930_e19972: f64 = (5.0 * assign13930_e19971);
        let assign13930_e19973: f64 = (1.0 + assign13930_e19972);
        let assign13930_e19975: f64 = (assign13930_e19973).powf(0.5);
        let assign13930_e19976: f64 = (locals.var_nc2l0 / assign13930_e19975);
        let assign13930_e19978: f64 = assign13930_e19976;
        let assign13930_e19984: f64 = (p.p43 * 1000000000.0);
        let assign13930_e19987: f64 = (p.p1891 * locals.var_pnc2l);
        let assign13930_e19988: f64 = (assign13930_e19984).powf(assign13930_e19987);
        let assign13930_e19989: f64 = (5.0 * assign13930_e19988);
        let assign13930_e19990: f64 = (1.0 + assign13930_e19989);
        let assign13930_e19992: f64 = (assign13930_e19990).powf(0.5);
        let assign13930_e19993: f64 = (locals.var_nc2l0 / assign13930_e19992);
        let assign13930_e19995: f64 = assign13930_e19993;
        let assign13930_e20001: f64 = (p.p43 * 1000000000.0);
        let assign13930_e20004: f64 = (p.p1891 * locals.var_pnc2l);
        let assign13930_e20005: f64 = (assign13930_e20001).powf(assign13930_e20004);
        let assign13930_e20006: f64 = (5.0 * assign13930_e20005);
        let assign13930_e20007: f64 = (1.0 + assign13930_e20006);
        let assign13930_e20009: f64 = (assign13930_e20007).powf(0.5);
        let assign13930_e20010: f64 = (locals.var_nc2l0 / assign13930_e20009);
        let assign13930_e20012: f64 = assign13930_e20010;
        let assign13930_e20013: f64 = (assign13930_e19995 * assign13930_e20012);
        let assign13930_e20016: f64 = (0.25 * 0.1);
        let assign13930_e20018: f64 = (assign13930_e20016 * 0.1);
        let assign13930_e20019: f64 = (assign13930_e20013 + assign13930_e20018);
        let assign13930_e20020: f64 = (assign13930_e20019).sqrt();
        let assign13930_e20021: f64 = (assign13930_e19978 + assign13930_e20020);
        let assign13930_e20022: f64 = (0.5 * assign13930_e20021);
        (assign13930_e20022,)
    } else {
        (locals.var_nc2l,)
    }
};
        locals.var_nc2l = assign13930_e20024;

        let (assign13940_e20089,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13940_e20032: f64 = (p.p1852 * 1000000000.0);
        let assign13940_e20035: f64 = (p.p1891 * locals.var_pnc2l);
        let assign13940_e20036: f64 = (assign13940_e20032).powf(assign13940_e20035);
        let assign13940_e20037: f64 = (5.0 * assign13940_e20036);
        let assign13940_e20038: f64 = (1.0 + assign13940_e20037);
        let assign13940_e20040: f64 = (assign13940_e20038).powf(0.5);
        let assign13940_e20041: f64 = (locals.var_nc2l0 / assign13940_e20040);
        let assign13940_e20043: f64 = assign13940_e20041;
        let assign13940_e20049: f64 = (p.p1852 * 1000000000.0);
        let assign13940_e20052: f64 = (p.p1891 * locals.var_pnc2l);
        let assign13940_e20053: f64 = (assign13940_e20049).powf(assign13940_e20052);
        let assign13940_e20054: f64 = (5.0 * assign13940_e20053);
        let assign13940_e20055: f64 = (1.0 + assign13940_e20054);
        let assign13940_e20057: f64 = (assign13940_e20055).powf(0.5);
        let assign13940_e20058: f64 = (locals.var_nc2l0 / assign13940_e20057);
        let assign13940_e20060: f64 = assign13940_e20058;
        let assign13940_e20066: f64 = (p.p1852 * 1000000000.0);
        let assign13940_e20069: f64 = (p.p1891 * locals.var_pnc2l);
        let assign13940_e20070: f64 = (assign13940_e20066).powf(assign13940_e20069);
        let assign13940_e20071: f64 = (5.0 * assign13940_e20070);
        let assign13940_e20072: f64 = (1.0 + assign13940_e20071);
        let assign13940_e20074: f64 = (assign13940_e20072).powf(0.5);
        let assign13940_e20075: f64 = (locals.var_nc2l0 / assign13940_e20074);
        let assign13940_e20077: f64 = assign13940_e20075;
        let assign13940_e20078: f64 = (assign13940_e20060 * assign13940_e20077);
        let assign13940_e20081: f64 = (0.25 * 0.1);
        let assign13940_e20083: f64 = (assign13940_e20081 * 0.1);
        let assign13940_e20084: f64 = (assign13940_e20078 + assign13940_e20083);
        let assign13940_e20085: f64 = (assign13940_e20084).sqrt();
        let assign13940_e20086: f64 = (assign13940_e20043 + assign13940_e20085);
        let assign13940_e20087: f64 = (0.5 * assign13940_e20086);
        (assign13940_e20087,)
    } else {
        (locals.var_nc2ln,)
    }
};
        locals.var_nc2ln = assign13940_e20089;

        let (assign13950_e20099,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13950_e20095: f64 = (locals.var_nc2l - locals.var_nc2ln);
        let assign13950_e20096: f64 = (p.p1888 * assign13950_e20095);
        let assign13950_e20097: f64 = (locals.var_mfq2nom_i + assign13950_e20096);
        (assign13950_e20097,)
    } else {
        (locals.var_nc2,)
    }
};
        locals.var_nc2 = assign13950_e20099;

        let (assign13960_e20164,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13960_e20107: f64 = (p.p43 * 1000000000.0);
        let assign13960_e20110: f64 = (p.p1892 * locals.var_pnc3l);
        let assign13960_e20111: f64 = (assign13960_e20107).powf(assign13960_e20110);
        let assign13960_e20112: f64 = (5.0 * assign13960_e20111);
        let assign13960_e20113: f64 = (1.0 + assign13960_e20112);
        let assign13960_e20115: f64 = (assign13960_e20113).powf(0.5);
        let assign13960_e20116: f64 = (locals.var_nc3l0 / assign13960_e20115);
        let assign13960_e20118: f64 = assign13960_e20116;
        let assign13960_e20124: f64 = (p.p43 * 1000000000.0);
        let assign13960_e20127: f64 = (p.p1892 * locals.var_pnc3l);
        let assign13960_e20128: f64 = (assign13960_e20124).powf(assign13960_e20127);
        let assign13960_e20129: f64 = (5.0 * assign13960_e20128);
        let assign13960_e20130: f64 = (1.0 + assign13960_e20129);
        let assign13960_e20132: f64 = (assign13960_e20130).powf(0.5);
        let assign13960_e20133: f64 = (locals.var_nc3l0 / assign13960_e20132);
        let assign13960_e20135: f64 = assign13960_e20133;
        let assign13960_e20141: f64 = (p.p43 * 1000000000.0);
        let assign13960_e20144: f64 = (p.p1892 * locals.var_pnc3l);
        let assign13960_e20145: f64 = (assign13960_e20141).powf(assign13960_e20144);
        let assign13960_e20146: f64 = (5.0 * assign13960_e20145);
        let assign13960_e20147: f64 = (1.0 + assign13960_e20146);
        let assign13960_e20149: f64 = (assign13960_e20147).powf(0.5);
        let assign13960_e20150: f64 = (locals.var_nc3l0 / assign13960_e20149);
        let assign13960_e20152: f64 = assign13960_e20150;
        let assign13960_e20153: f64 = (assign13960_e20135 * assign13960_e20152);
        let assign13960_e20156: f64 = (0.25 * 0.1);
        let assign13960_e20158: f64 = (assign13960_e20156 * 0.1);
        let assign13960_e20159: f64 = (assign13960_e20153 + assign13960_e20158);
        let assign13960_e20160: f64 = (assign13960_e20159).sqrt();
        let assign13960_e20161: f64 = (assign13960_e20118 + assign13960_e20160);
        let assign13960_e20162: f64 = (0.5 * assign13960_e20161);
        (assign13960_e20162,)
    } else {
        (locals.var_nc3l,)
    }
};
        locals.var_nc3l = assign13960_e20164;

        let (assign13970_e20229,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13970_e20172: f64 = (p.p1852 * 1000000000.0);
        let assign13970_e20175: f64 = (p.p1892 * locals.var_pnc3l);
        let assign13970_e20176: f64 = (assign13970_e20172).powf(assign13970_e20175);
        let assign13970_e20177: f64 = (5.0 * assign13970_e20176);
        let assign13970_e20178: f64 = (1.0 + assign13970_e20177);
        let assign13970_e20180: f64 = (assign13970_e20178).powf(0.5);
        let assign13970_e20181: f64 = (locals.var_nc3l0 / assign13970_e20180);
        let assign13970_e20183: f64 = assign13970_e20181;
        let assign13970_e20189: f64 = (p.p1852 * 1000000000.0);
        let assign13970_e20192: f64 = (p.p1892 * locals.var_pnc3l);
        let assign13970_e20193: f64 = (assign13970_e20189).powf(assign13970_e20192);
        let assign13970_e20194: f64 = (5.0 * assign13970_e20193);
        let assign13970_e20195: f64 = (1.0 + assign13970_e20194);
        let assign13970_e20197: f64 = (assign13970_e20195).powf(0.5);
        let assign13970_e20198: f64 = (locals.var_nc3l0 / assign13970_e20197);
        let assign13970_e20200: f64 = assign13970_e20198;
        let assign13970_e20206: f64 = (p.p1852 * 1000000000.0);
        let assign13970_e20209: f64 = (p.p1892 * locals.var_pnc3l);
        let assign13970_e20210: f64 = (assign13970_e20206).powf(assign13970_e20209);
        let assign13970_e20211: f64 = (5.0 * assign13970_e20210);
        let assign13970_e20212: f64 = (1.0 + assign13970_e20211);
        let assign13970_e20214: f64 = (assign13970_e20212).powf(0.5);
        let assign13970_e20215: f64 = (locals.var_nc3l0 / assign13970_e20214);
        let assign13970_e20217: f64 = assign13970_e20215;
        let assign13970_e20218: f64 = (assign13970_e20200 * assign13970_e20217);
        let assign13970_e20221: f64 = (0.25 * 0.1);
        let assign13970_e20223: f64 = (assign13970_e20221 * 0.1);
        let assign13970_e20224: f64 = (assign13970_e20218 + assign13970_e20223);
        let assign13970_e20225: f64 = (assign13970_e20224).sqrt();
        let assign13970_e20226: f64 = (assign13970_e20183 + assign13970_e20225);
        let assign13970_e20227: f64 = (0.5 * assign13970_e20226);
        (assign13970_e20227,)
    } else {
        (locals.var_nc3ln,)
    }
};
        locals.var_nc3ln = assign13970_e20229;

        let (assign13980_e20239,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13980_e20235: f64 = (locals.var_nc3l - locals.var_nc3ln);
        let assign13980_e20236: f64 = (p.p1889 * assign13980_e20235);
        let assign13980_e20237: f64 = (locals.var_mfq3nom_i + assign13980_e20236);
        (assign13980_e20237,)
    } else {
        (locals.var_nc3,)
    }
};
        locals.var_nc3 = assign13980_e20239;

    }

    pub(super) fn stamp_transient_block_38(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign13990_e20373,) = {
    if (locals.var_guard234 != 0.0) {
        let assign13990_e20244: f64 = (locals.var_d1 / 2.0);
        let assign13990_e20245: f64 = (1.60219e-19 * assign13990_e20244);
        let assign13990_e20249: f64 = (locals.var_d1 / 2.0);
        let assign13990_e20250: f64 = (3.14_f64).powf(assign13990_e20249);
        let assign13990_e20253: f64 = (-4.6);
        let assign13990_e20257: f64 = (locals.var_d1 / 2.0);
        let assign13990_e20258: f64 = (1.0 + assign13990_e20257);
        let assign13990_e20260: f64 = (assign13990_e20258 - 1.0);
        let assign13990_e20261: f64 = (assign13990_e20253 * assign13990_e20260);
        let assign13990_e20262: f64 = (assign13990_e20261).exp();
        let assign13990_e20263: f64 = (0.0385 * assign13990_e20262);
        let assign13990_e20269: f64 = (locals.var_d1 / 2.0);
        let assign13990_e20270: f64 = (1.0 + assign13990_e20269);
        let assign13990_e20271: f64 = (2.0 * assign13990_e20270);
        let assign13990_e20273: f64 = (assign13990_e20271 - 3.0);
        let assign13990_e20275: f64 = (assign13990_e20273).powf(8.0);
        let assign13990_e20276: f64 = (7.5893e-7 * assign13990_e20275);
        let assign13990_e20277: f64 = (assign13990_e20263 + assign13990_e20276);
        let assign13990_e20283: f64 = (locals.var_d1 / 2.0);
        let assign13990_e20284: f64 = (1.0 + assign13990_e20283);
        let assign13990_e20286: f64 = (assign13990_e20284 - 1.0);
        let assign13990_e20287: f64 = (2.0 * assign13990_e20286);
        let assign13990_e20289: f64 = (assign13990_e20287).powf(6.0);
        let assign13990_e20290: f64 = (6.9583e-5 * assign13990_e20289);
        let assign13990_e20291: f64 = (assign13990_e20277 + assign13990_e20290);
        let assign13990_e20297: f64 = (locals.var_d1 / 2.0);
        let assign13990_e20298: f64 = (1.0 + assign13990_e20297);
        let assign13990_e20300: f64 = (assign13990_e20298 - 1.0);
        let assign13990_e20301: f64 = (2.0 * assign13990_e20300);
        let assign13990_e20303: f64 = (assign13990_e20301).powf(5.0);
        let assign13990_e20304: f64 = (0.0006583 * assign13990_e20303);
        let assign13990_e20305: f64 = (assign13990_e20291 - assign13990_e20304);
        let assign13990_e20311: f64 = (locals.var_d1 / 2.0);
        let assign13990_e20312: f64 = (1.0 + assign13990_e20311);
        let assign13990_e20314: f64 = (assign13990_e20312 - 1.0);
        let assign13990_e20315: f64 = (2.0 * assign13990_e20314);
        let assign13990_e20317: f64 = (assign13990_e20315).powf(4.0);
        let assign13990_e20318: f64 = (0.0065 * assign13990_e20317);
        let assign13990_e20319: f64 = (assign13990_e20305 + assign13990_e20318);
        let assign13990_e20325: f64 = (locals.var_d1 / 2.0);
        let assign13990_e20326: f64 = (1.0 + assign13990_e20325);
        let assign13990_e20328: f64 = (assign13990_e20326 - 1.0);
        let assign13990_e20329: f64 = (2.0 * assign13990_e20328);
        let assign13990_e20331: f64 = (assign13990_e20329).powf(3.0);
        let assign13990_e20332: f64 = (0.026 * assign13990_e20331);
        let assign13990_e20333: f64 = (assign13990_e20319 - assign13990_e20332);
        let assign13990_e20339: f64 = (locals.var_d1 / 2.0);
        let assign13990_e20340: f64 = (1.0 + assign13990_e20339);
        let assign13990_e20342: f64 = (assign13990_e20340 - 1.0);
        let assign13990_e20343: f64 = (2.0 * assign13990_e20342);
        let assign13990_e20345: f64 = (assign13990_e20343).powf(2.0);
        let assign13990_e20346: f64 = (0.1371 * assign13990_e20345);
        let assign13990_e20347: f64 = (assign13990_e20333 + assign13990_e20346);
        let assign13990_e20350: f64 = (0.194 * 2.0);
        let assign13990_e20354: f64 = (locals.var_d1 / 2.0);
        let assign13990_e20355: f64 = (1.0 + assign13990_e20354);
        let assign13990_e20357: f64 = (assign13990_e20355 - 1.0);
        let assign13990_e20358: f64 = (assign13990_e20350 * assign13990_e20357);
        let assign13990_e20359: f64 = (assign13990_e20347 - assign13990_e20358);
        let assign13990_e20361: f64 = (assign13990_e20359 + 0.959);
        let assign13990_e20362: f64 = (assign13990_e20250 / assign13990_e20361);
        let assign13990_e20363: f64 = (assign13990_e20245 * assign13990_e20362);
        let assign13990_e20366: f64 = (locals.var_nc1 * 1000000.0);
        let assign13990_e20368: f64 = (assign13990_e20366).powf(locals.var_d1);
        let assign13990_e20369: f64 = (assign13990_e20363 * assign13990_e20368);
        let assign13990_e20371: f64 = (assign13990_e20369 * locals.var_qndnf1);
        (assign13990_e20371,)
    } else {
        (locals.var_qnd10,)
    }
};
        locals.var_qnd10 = assign13990_e20373;

        let (assign14000_e20507,) = {
    if (locals.var_guard234 != 0.0) {
        let assign14000_e20378: f64 = (locals.var_d2 / 2.0);
        let assign14000_e20379: f64 = (1.60219e-19 * assign14000_e20378);
        let assign14000_e20383: f64 = (locals.var_d2 / 2.0);
        let assign14000_e20384: f64 = (3.14_f64).powf(assign14000_e20383);
        let assign14000_e20387: f64 = (-4.6);
        let assign14000_e20391: f64 = (locals.var_d2 / 2.0);
        let assign14000_e20392: f64 = (1.0 + assign14000_e20391);
        let assign14000_e20394: f64 = (assign14000_e20392 - 1.0);
        let assign14000_e20395: f64 = (assign14000_e20387 * assign14000_e20394);
        let assign14000_e20396: f64 = (assign14000_e20395).exp();
        let assign14000_e20397: f64 = (0.0385 * assign14000_e20396);
        let assign14000_e20403: f64 = (locals.var_d2 / 2.0);
        let assign14000_e20404: f64 = (1.0 + assign14000_e20403);
        let assign14000_e20405: f64 = (2.0 * assign14000_e20404);
        let assign14000_e20407: f64 = (assign14000_e20405 - 3.0);
        let assign14000_e20409: f64 = (assign14000_e20407).powf(8.0);
        let assign14000_e20410: f64 = (7.5893e-7 * assign14000_e20409);
        let assign14000_e20411: f64 = (assign14000_e20397 + assign14000_e20410);
        let assign14000_e20417: f64 = (locals.var_d2 / 2.0);
        let assign14000_e20418: f64 = (1.0 + assign14000_e20417);
        let assign14000_e20420: f64 = (assign14000_e20418 - 1.0);
        let assign14000_e20421: f64 = (2.0 * assign14000_e20420);
        let assign14000_e20423: f64 = (assign14000_e20421).powf(6.0);
        let assign14000_e20424: f64 = (6.9583e-5 * assign14000_e20423);
        let assign14000_e20425: f64 = (assign14000_e20411 + assign14000_e20424);
        let assign14000_e20431: f64 = (locals.var_d2 / 2.0);
        let assign14000_e20432: f64 = (1.0 + assign14000_e20431);
        let assign14000_e20434: f64 = (assign14000_e20432 - 1.0);
        let assign14000_e20435: f64 = (2.0 * assign14000_e20434);
        let assign14000_e20437: f64 = (assign14000_e20435).powf(5.0);
        let assign14000_e20438: f64 = (0.0006583 * assign14000_e20437);
        let assign14000_e20439: f64 = (assign14000_e20425 - assign14000_e20438);
        let assign14000_e20445: f64 = (locals.var_d2 / 2.0);
        let assign14000_e20446: f64 = (1.0 + assign14000_e20445);
        let assign14000_e20448: f64 = (assign14000_e20446 - 1.0);
        let assign14000_e20449: f64 = (2.0 * assign14000_e20448);
        let assign14000_e20451: f64 = (assign14000_e20449).powf(4.0);
        let assign14000_e20452: f64 = (0.0065 * assign14000_e20451);
        let assign14000_e20453: f64 = (assign14000_e20439 + assign14000_e20452);
        let assign14000_e20459: f64 = (locals.var_d2 / 2.0);
        let assign14000_e20460: f64 = (1.0 + assign14000_e20459);
        let assign14000_e20462: f64 = (assign14000_e20460 - 1.0);
        let assign14000_e20463: f64 = (2.0 * assign14000_e20462);
        let assign14000_e20465: f64 = (assign14000_e20463).powf(3.0);
        let assign14000_e20466: f64 = (0.026 * assign14000_e20465);
        let assign14000_e20467: f64 = (assign14000_e20453 - assign14000_e20466);
        let assign14000_e20473: f64 = (locals.var_d2 / 2.0);
        let assign14000_e20474: f64 = (1.0 + assign14000_e20473);
        let assign14000_e20476: f64 = (assign14000_e20474 - 1.0);
        let assign14000_e20477: f64 = (2.0 * assign14000_e20476);
        let assign14000_e20479: f64 = (assign14000_e20477).powf(2.0);
        let assign14000_e20480: f64 = (0.1371 * assign14000_e20479);
        let assign14000_e20481: f64 = (assign14000_e20467 + assign14000_e20480);
        let assign14000_e20484: f64 = (0.194 * 2.0);
        let assign14000_e20488: f64 = (locals.var_d2 / 2.0);
        let assign14000_e20489: f64 = (1.0 + assign14000_e20488);
        let assign14000_e20491: f64 = (assign14000_e20489 - 1.0);
        let assign14000_e20492: f64 = (assign14000_e20484 * assign14000_e20491);
        let assign14000_e20493: f64 = (assign14000_e20481 - assign14000_e20492);
        let assign14000_e20495: f64 = (assign14000_e20493 + 0.959);
        let assign14000_e20496: f64 = (assign14000_e20384 / assign14000_e20495);
        let assign14000_e20497: f64 = (assign14000_e20379 * assign14000_e20496);
        let assign14000_e20500: f64 = (locals.var_nc2 * 1000000.0);
        let assign14000_e20502: f64 = (assign14000_e20500).powf(locals.var_d2);
        let assign14000_e20503: f64 = (assign14000_e20497 * assign14000_e20502);
        let assign14000_e20505: f64 = (assign14000_e20503 * locals.var_qndnf2);
        (assign14000_e20505,)
    } else {
        (locals.var_qnd20,)
    }
};
        locals.var_qnd20 = assign14000_e20507;

        let (assign14010_e20641,) = {
    if (locals.var_guard234 != 0.0) {
        let assign14010_e20512: f64 = (locals.var_d3 / 2.0);
        let assign14010_e20513: f64 = (1.60219e-19 * assign14010_e20512);
        let assign14010_e20517: f64 = (locals.var_d3 / 2.0);
        let assign14010_e20518: f64 = (3.14_f64).powf(assign14010_e20517);
        let assign14010_e20521: f64 = (-4.6);
        let assign14010_e20525: f64 = (locals.var_d3 / 2.0);
        let assign14010_e20526: f64 = (1.0 + assign14010_e20525);
        let assign14010_e20528: f64 = (assign14010_e20526 - 1.0);
        let assign14010_e20529: f64 = (assign14010_e20521 * assign14010_e20528);
        let assign14010_e20530: f64 = (assign14010_e20529).exp();
        let assign14010_e20531: f64 = (0.0385 * assign14010_e20530);
        let assign14010_e20537: f64 = (locals.var_d3 / 2.0);
        let assign14010_e20538: f64 = (1.0 + assign14010_e20537);
        let assign14010_e20539: f64 = (2.0 * assign14010_e20538);
        let assign14010_e20541: f64 = (assign14010_e20539 - 3.0);
        let assign14010_e20543: f64 = (assign14010_e20541).powf(8.0);
        let assign14010_e20544: f64 = (7.5893e-7 * assign14010_e20543);
        let assign14010_e20545: f64 = (assign14010_e20531 + assign14010_e20544);
        let assign14010_e20551: f64 = (locals.var_d3 / 2.0);
        let assign14010_e20552: f64 = (1.0 + assign14010_e20551);
        let assign14010_e20554: f64 = (assign14010_e20552 - 1.0);
        let assign14010_e20555: f64 = (2.0 * assign14010_e20554);
        let assign14010_e20557: f64 = (assign14010_e20555).powf(6.0);
        let assign14010_e20558: f64 = (6.9583e-5 * assign14010_e20557);
        let assign14010_e20559: f64 = (assign14010_e20545 + assign14010_e20558);
        let assign14010_e20565: f64 = (locals.var_d3 / 2.0);
        let assign14010_e20566: f64 = (1.0 + assign14010_e20565);
        let assign14010_e20568: f64 = (assign14010_e20566 - 1.0);
        let assign14010_e20569: f64 = (2.0 * assign14010_e20568);
        let assign14010_e20571: f64 = (assign14010_e20569).powf(5.0);
        let assign14010_e20572: f64 = (0.0006583 * assign14010_e20571);
        let assign14010_e20573: f64 = (assign14010_e20559 - assign14010_e20572);
        let assign14010_e20579: f64 = (locals.var_d3 / 2.0);
        let assign14010_e20580: f64 = (1.0 + assign14010_e20579);
        let assign14010_e20582: f64 = (assign14010_e20580 - 1.0);
        let assign14010_e20583: f64 = (2.0 * assign14010_e20582);
        let assign14010_e20585: f64 = (assign14010_e20583).powf(4.0);
        let assign14010_e20586: f64 = (0.0065 * assign14010_e20585);
        let assign14010_e20587: f64 = (assign14010_e20573 + assign14010_e20586);
        let assign14010_e20593: f64 = (locals.var_d3 / 2.0);
        let assign14010_e20594: f64 = (1.0 + assign14010_e20593);
        let assign14010_e20596: f64 = (assign14010_e20594 - 1.0);
        let assign14010_e20597: f64 = (2.0 * assign14010_e20596);
        let assign14010_e20599: f64 = (assign14010_e20597).powf(3.0);
        let assign14010_e20600: f64 = (0.026 * assign14010_e20599);
        let assign14010_e20601: f64 = (assign14010_e20587 - assign14010_e20600);
        let assign14010_e20607: f64 = (locals.var_d3 / 2.0);
        let assign14010_e20608: f64 = (1.0 + assign14010_e20607);
        let assign14010_e20610: f64 = (assign14010_e20608 - 1.0);
        let assign14010_e20611: f64 = (2.0 * assign14010_e20610);
        let assign14010_e20613: f64 = (assign14010_e20611).powf(2.0);
        let assign14010_e20614: f64 = (0.1371 * assign14010_e20613);
        let assign14010_e20615: f64 = (assign14010_e20601 + assign14010_e20614);
        let assign14010_e20618: f64 = (0.194 * 2.0);
        let assign14010_e20622: f64 = (locals.var_d3 / 2.0);
        let assign14010_e20623: f64 = (1.0 + assign14010_e20622);
        let assign14010_e20625: f64 = (assign14010_e20623 - 1.0);
        let assign14010_e20626: f64 = (assign14010_e20618 * assign14010_e20625);
        let assign14010_e20627: f64 = (assign14010_e20615 - assign14010_e20626);
        let assign14010_e20629: f64 = (assign14010_e20627 + 0.959);
        let assign14010_e20630: f64 = (assign14010_e20518 / assign14010_e20629);
        let assign14010_e20631: f64 = (assign14010_e20513 * assign14010_e20630);
        let assign14010_e20634: f64 = (locals.var_nc3 * 1000000.0);
        let assign14010_e20636: f64 = (assign14010_e20634).powf(locals.var_d3);
        let assign14010_e20637: f64 = (assign14010_e20631 * assign14010_e20636);
        let assign14010_e20639: f64 = (assign14010_e20637 * locals.var_qndnf3);
        (assign14010_e20639,)
    } else {
        (locals.var_qnd30,)
    }
};
        locals.var_qnd30 = assign14010_e20641;

        let assign14020_e20644: f64 = if p.p58 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard235 = assign14020_e20644;

        let (assign14030_e20665,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14030_e20649: f64 = (locals.var_etamob_i - p.p889);
        let assign14030_e20652: f64 = (p.p890 * 1000000000.0);
        let assign14030_e20655: f64 = (p.p40 * 1000000000.0);
        let assign14030_e20656: f64 = (assign14030_e20652 - assign14030_e20655);
        let assign14030_e20658: f64 = (assign14030_e20656 / p.p891);
        let assign14030_e20659: f64 = { let limited_exp_arg = assign14030_e20658; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign14030_e20661: f64 = (assign14030_e20659 + 1.0);
        let assign14030_e20662: f64 = (assign14030_e20649 / assign14030_e20661);
        let assign14030_e20663: f64 = (p.p889 + assign14030_e20662);
        (assign14030_e20663,)
    } else {
        (locals.var_etamob_i,)
    }
};
        locals.var_etamob_i = assign14030_e20665;

        let (assign14040_e20677, assign14040_e20677_d_n0, assign14040_e20677_d_n2, assign14040_e20677_d_n3, assign14040_e20677_d_n4, assign14040_e20677_d_n5, assign14040_e20677_d_n6, assign14040_e20677_d_n7, assign14040_e20677_d_n8, assign14040_e20677_d_n9, assign14040_e20677_d_n10, assign14040_e20677_d_n11, assign14040_e20677_d_n13, assign14040_e20677_d_n14,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14040_e20669: f64 = (locals.var_ua_i - p.p892);
        let assign14040_e20672: f64 = (p.p893 * 1000000000.0);
        let assign14040_e20674: f64 = (assign14040_e20672 * p.p894);
        let assign14040_e20675: f64 = (assign14040_e20669 - assign14040_e20674);
        (assign14040_e20675, locals.var_ua_i_dn0, locals.var_ua_i_dn2, locals.var_ua_i_dn3, locals.var_ua_i_dn4, locals.var_ua_i_dn5, locals.var_ua_i_dn6, locals.var_ua_i_dn7, locals.var_ua_i_dn8, locals.var_ua_i_dn9, locals.var_ua_i_dn10, locals.var_ua_i_dn11, locals.var_ua_i_dn13, locals.var_ua_i_dn14,)
    } else {
        (locals.var_mut0, locals.var_mut0_dn0, locals.var_mut0_dn2, locals.var_mut0_dn3, locals.var_mut0_dn4, locals.var_mut0_dn5, locals.var_mut0_dn6, locals.var_mut0_dn7, locals.var_mut0_dn8, locals.var_mut0_dn9, locals.var_mut0_dn10, locals.var_mut0_dn11, locals.var_mut0_dn13, locals.var_mut0_dn14,)
    }
};
        locals.var_mut0 = assign14040_e20677;
        locals.var_mut0_dn0 = assign14040_e20677_d_n0;
        locals.var_mut0_dn2 = assign14040_e20677_d_n2;
        locals.var_mut0_dn3 = assign14040_e20677_d_n3;
        locals.var_mut0_dn4 = assign14040_e20677_d_n4;
        locals.var_mut0_dn5 = assign14040_e20677_d_n5;
        locals.var_mut0_dn6 = assign14040_e20677_d_n6;
        locals.var_mut0_dn7 = assign14040_e20677_d_n7;
        locals.var_mut0_dn8 = assign14040_e20677_d_n8;
        locals.var_mut0_dn9 = assign14040_e20677_d_n9;
        locals.var_mut0_dn10 = assign14040_e20677_d_n10;
        locals.var_mut0_dn11 = assign14040_e20677_d_n11;
        locals.var_mut0_dn13 = assign14040_e20677_d_n13;
        locals.var_mut0_dn14 = assign14040_e20677_d_n14;

        let (assign14050_e20700, assign14050_e20700_d_n0, assign14050_e20700_d_n2, assign14050_e20700_d_n3, assign14050_e20700_d_n4, assign14050_e20700_d_n5, assign14050_e20700_d_n6, assign14050_e20700_d_n7, assign14050_e20700_d_n8, assign14050_e20700_d_n9, assign14050_e20700_d_n10, assign14050_e20700_d_n11, assign14050_e20700_d_n13, assign14050_e20700_d_n14,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14050_e20682: f64 = (p.p40 * 1000000000.0);
        let assign14050_e20684: f64 = (assign14050_e20682 * p.p894);
        let assign14050_e20685: f64 = (locals.var_mut0 + assign14050_e20684);
        let assign14050_e20689: f64 = (p.p895 * 1000000000.0);
        let assign14050_e20692: f64 = (p.p40 * 1000000000.0);
        let assign14050_e20693: f64 = (assign14050_e20689 - assign14050_e20692);
        let assign14050_e20695: f64 = (assign14050_e20693 / p.p896);
        let assign14050_e20696: f64 = { let limited_exp_arg = assign14050_e20695; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign14050_e20697: f64 = (1.0 + assign14050_e20696);
        let assign14050_e20698: f64 = (assign14050_e20685 / assign14050_e20697);
        (assign14050_e20698, (locals.var_mut0_dn0 / assign14050_e20697), (locals.var_mut0_dn2 / assign14050_e20697), (locals.var_mut0_dn3 / assign14050_e20697), (locals.var_mut0_dn4 / assign14050_e20697), (locals.var_mut0_dn5 / assign14050_e20697), (locals.var_mut0_dn6 / assign14050_e20697), (locals.var_mut0_dn7 / assign14050_e20697), (locals.var_mut0_dn8 / assign14050_e20697), (locals.var_mut0_dn9 / assign14050_e20697), (locals.var_mut0_dn10 / assign14050_e20697), (locals.var_mut0_dn11 / assign14050_e20697), (locals.var_mut0_dn13 / assign14050_e20697), (locals.var_mut0_dn14 / assign14050_e20697),)
    } else {
        (locals.var_mut1, locals.var_mut1_dn0, locals.var_mut1_dn2, locals.var_mut1_dn3, locals.var_mut1_dn4, locals.var_mut1_dn5, locals.var_mut1_dn6, locals.var_mut1_dn7, locals.var_mut1_dn8, locals.var_mut1_dn9, locals.var_mut1_dn10, locals.var_mut1_dn11, locals.var_mut1_dn13, locals.var_mut1_dn14,)
    }
};
        locals.var_mut1 = assign14050_e20700;
        locals.var_mut1_dn0 = assign14050_e20700_d_n0;
        locals.var_mut1_dn2 = assign14050_e20700_d_n2;
        locals.var_mut1_dn3 = assign14050_e20700_d_n3;
        locals.var_mut1_dn4 = assign14050_e20700_d_n4;
        locals.var_mut1_dn5 = assign14050_e20700_d_n5;
        locals.var_mut1_dn6 = assign14050_e20700_d_n6;
        locals.var_mut1_dn7 = assign14050_e20700_d_n7;
        locals.var_mut1_dn8 = assign14050_e20700_d_n8;
        locals.var_mut1_dn9 = assign14050_e20700_d_n9;
        locals.var_mut1_dn10 = assign14050_e20700_d_n10;
        locals.var_mut1_dn11 = assign14050_e20700_d_n11;
        locals.var_mut1_dn13 = assign14050_e20700_d_n13;
        locals.var_mut1_dn14 = assign14050_e20700_d_n14;

        let (assign14060_e20735, assign14060_e20735_d_n0, assign14060_e20735_d_n2, assign14060_e20735_d_n3, assign14060_e20735_d_n4, assign14060_e20735_d_n5, assign14060_e20735_d_n6, assign14060_e20735_d_n7, assign14060_e20735_d_n8, assign14060_e20735_d_n9, assign14060_e20735_d_n10, assign14060_e20735_d_n11, assign14060_e20735_d_n13, assign14060_e20735_d_n14,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14060_e20705: f64 = (locals.var_mut1 + p.p892);
        let assign14060_e20708: f64 = (locals.var_ua_i + 0.2);
        let assign14060_e20709: f64 = (assign14060_e20705 + assign14060_e20708);
        let assign14060_e20712: f64 = (locals.var_mut1 + p.p892);
        let assign14060_e20715: f64 = (locals.var_ua_i + 0.2);
        let assign14060_e20716: f64 = (assign14060_e20712 - assign14060_e20715);
        let assign14060_e20719: f64 = (locals.var_mut1 + p.p892);
        let assign14060_e20722: f64 = (locals.var_ua_i + 0.2);
        let assign14060_e20723: f64 = (assign14060_e20719 - assign14060_e20722);
        let assign14060_e20724: f64 = (assign14060_e20716 * assign14060_e20723);
        let assign14060_e20727: f64 = (0.25 * 0.6);
        let assign14060_e20729: f64 = (assign14060_e20727 * 0.6);
        let assign14060_e20730: f64 = (assign14060_e20724 + assign14060_e20729);
        let assign14060_e20731: f64 = (assign14060_e20730).sqrt();
        let assign14060_e20732: f64 = (assign14060_e20709 - assign14060_e20731);
        let assign14060_e20733: f64 = (0.5 * assign14060_e20732);
        (assign14060_e20733, (0.5 * ((locals.var_mut1_dn0 + locals.var_ua_i_dn0) - ((((locals.var_mut1_dn0 - locals.var_ua_i_dn0) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn0 - locals.var_ua_i_dn0))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn2 + locals.var_ua_i_dn2) - ((((locals.var_mut1_dn2 - locals.var_ua_i_dn2) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn2 - locals.var_ua_i_dn2))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn3 + locals.var_ua_i_dn3) - ((((locals.var_mut1_dn3 - locals.var_ua_i_dn3) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn3 - locals.var_ua_i_dn3))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn4 + locals.var_ua_i_dn4) - ((((locals.var_mut1_dn4 - locals.var_ua_i_dn4) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn4 - locals.var_ua_i_dn4))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn5 + locals.var_ua_i_dn5) - ((((locals.var_mut1_dn5 - locals.var_ua_i_dn5) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn5 - locals.var_ua_i_dn5))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn6 + locals.var_ua_i_dn6) - ((((locals.var_mut1_dn6 - locals.var_ua_i_dn6) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn6 - locals.var_ua_i_dn6))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn7 + locals.var_ua_i_dn7) - ((((locals.var_mut1_dn7 - locals.var_ua_i_dn7) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn7 - locals.var_ua_i_dn7))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn8 + locals.var_ua_i_dn8) - ((((locals.var_mut1_dn8 - locals.var_ua_i_dn8) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn8 - locals.var_ua_i_dn8))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn9 + locals.var_ua_i_dn9) - ((((locals.var_mut1_dn9 - locals.var_ua_i_dn9) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn9 - locals.var_ua_i_dn9))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn10 + locals.var_ua_i_dn10) - ((((locals.var_mut1_dn10 - locals.var_ua_i_dn10) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn10 - locals.var_ua_i_dn10))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn11 + locals.var_ua_i_dn11) - ((((locals.var_mut1_dn11 - locals.var_ua_i_dn11) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn11 - locals.var_ua_i_dn11))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn13 + locals.var_ua_i_dn13) - ((((locals.var_mut1_dn13 - locals.var_ua_i_dn13) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn13 - locals.var_ua_i_dn13))) / (2.0 * assign14060_e20731)))), (0.5 * ((locals.var_mut1_dn14 + locals.var_ua_i_dn14) - ((((locals.var_mut1_dn14 - locals.var_ua_i_dn14) * assign14060_e20723) + (assign14060_e20716 * (locals.var_mut1_dn14 - locals.var_ua_i_dn14))) / (2.0 * assign14060_e20731)))),)
    } else {
        (locals.var_ua_i, locals.var_ua_i_dn0, locals.var_ua_i_dn2, locals.var_ua_i_dn3, locals.var_ua_i_dn4, locals.var_ua_i_dn5, locals.var_ua_i_dn6, locals.var_ua_i_dn7, locals.var_ua_i_dn8, locals.var_ua_i_dn9, locals.var_ua_i_dn10, locals.var_ua_i_dn11, locals.var_ua_i_dn13, locals.var_ua_i_dn14,)
    }
};
        locals.var_ua_i = assign14060_e20735;
        locals.var_ua_i_dn0 = assign14060_e20735_d_n0;
        locals.var_ua_i_dn2 = assign14060_e20735_d_n2;
        locals.var_ua_i_dn3 = assign14060_e20735_d_n3;
        locals.var_ua_i_dn4 = assign14060_e20735_d_n4;
        locals.var_ua_i_dn5 = assign14060_e20735_d_n5;
        locals.var_ua_i_dn6 = assign14060_e20735_d_n6;
        locals.var_ua_i_dn7 = assign14060_e20735_d_n7;
        locals.var_ua_i_dn8 = assign14060_e20735_d_n8;
        locals.var_ua_i_dn9 = assign14060_e20735_d_n9;
        locals.var_ua_i_dn10 = assign14060_e20735_d_n10;
        locals.var_ua_i_dn11 = assign14060_e20735_d_n11;
        locals.var_ua_i_dn13 = assign14060_e20735_d_n13;
        locals.var_ua_i_dn14 = assign14060_e20735_d_n14;

        let (assign14070_e20768, assign14070_e20768_d_n0, assign14070_e20768_d_n2, assign14070_e20768_d_n3, assign14070_e20768_d_n4, assign14070_e20768_d_n5, assign14070_e20768_d_n6, assign14070_e20768_d_n7, assign14070_e20768_d_n8, assign14070_e20768_d_n9, assign14070_e20768_d_n10, assign14070_e20768_d_n11, assign14070_e20768_d_n13, assign14070_e20768_d_n14,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14070_e20739: f64 = (p.p897 - locals.var_eu_i);
        let assign14070_e20741: f64 = (assign14070_e20739 * 370.0);
        let assign14070_e20744: f64 = (p.p40 * 1000000000.0);
        let assign14070_e20746: f64 = (assign14070_e20744).powf(p.p898);
        let assign14070_e20747: f64 = (assign14070_e20741 / assign14070_e20746);
        let assign14070_e20750: f64 = (p.p897 - locals.var_eu_i);
        let assign14070_e20754: f64 = (p.p40 * 1000000000.0);
        let assign14070_e20757: f64 = (p.p899 * 1000000000.0);
        let assign14070_e20758: f64 = (assign14070_e20754 - assign14070_e20757);
        let assign14070_e20760: f64 = (assign14070_e20758 / p.p900);
        let assign14070_e20761: f64 = { let limited_exp_arg = assign14070_e20760; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign14070_e20762: f64 = (1.0 + assign14070_e20761);
        let assign14070_e20763: f64 = (assign14070_e20750 / assign14070_e20762);
        let assign14070_e20764: f64 = (assign14070_e20747 + assign14070_e20763);
        let assign14070_e20766: f64 = (assign14070_e20764 + locals.var_eu_i);
        (assign14070_e20766, (((((-locals.var_eu_i_dn0) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn0) / assign14070_e20762)) + locals.var_eu_i_dn0), (((((-locals.var_eu_i_dn2) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn2) / assign14070_e20762)) + locals.var_eu_i_dn2), (((((-locals.var_eu_i_dn3) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn3) / assign14070_e20762)) + locals.var_eu_i_dn3), (((((-locals.var_eu_i_dn4) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn4) / assign14070_e20762)) + locals.var_eu_i_dn4), (((((-locals.var_eu_i_dn5) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn5) / assign14070_e20762)) + locals.var_eu_i_dn5), (((((-locals.var_eu_i_dn6) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn6) / assign14070_e20762)) + locals.var_eu_i_dn6), (((((-locals.var_eu_i_dn7) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn7) / assign14070_e20762)) + locals.var_eu_i_dn7), (((((-locals.var_eu_i_dn8) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn8) / assign14070_e20762)) + locals.var_eu_i_dn8), (((((-locals.var_eu_i_dn9) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn9) / assign14070_e20762)) + locals.var_eu_i_dn9), (((((-locals.var_eu_i_dn10) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn10) / assign14070_e20762)) + locals.var_eu_i_dn10), (((((-locals.var_eu_i_dn11) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn11) / assign14070_e20762)) + locals.var_eu_i_dn11), (((((-locals.var_eu_i_dn13) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn13) / assign14070_e20762)) + locals.var_eu_i_dn13), (((((-locals.var_eu_i_dn14) * 370.0) / assign14070_e20746) + ((-locals.var_eu_i_dn14) / assign14070_e20762)) + locals.var_eu_i_dn14),)
    } else {
        (locals.var_mut2, locals.var_mut2_dn0, locals.var_mut2_dn2, locals.var_mut2_dn3, locals.var_mut2_dn4, locals.var_mut2_dn5, locals.var_mut2_dn6, locals.var_mut2_dn7, locals.var_mut2_dn8, locals.var_mut2_dn9, locals.var_mut2_dn10, locals.var_mut2_dn11, locals.var_mut2_dn13, locals.var_mut2_dn14,)
    }
};
        locals.var_mut2 = assign14070_e20768;
        locals.var_mut2_dn0 = assign14070_e20768_d_n0;
        locals.var_mut2_dn2 = assign14070_e20768_d_n2;
        locals.var_mut2_dn3 = assign14070_e20768_d_n3;
        locals.var_mut2_dn4 = assign14070_e20768_d_n4;
        locals.var_mut2_dn5 = assign14070_e20768_d_n5;
        locals.var_mut2_dn6 = assign14070_e20768_d_n6;
        locals.var_mut2_dn7 = assign14070_e20768_d_n7;
        locals.var_mut2_dn8 = assign14070_e20768_d_n8;
        locals.var_mut2_dn9 = assign14070_e20768_d_n9;
        locals.var_mut2_dn10 = assign14070_e20768_d_n10;
        locals.var_mut2_dn11 = assign14070_e20768_d_n11;
        locals.var_mut2_dn13 = assign14070_e20768_d_n13;
        locals.var_mut2_dn14 = assign14070_e20768_d_n14;

        let (assign14080_e20791, assign14080_e20791_d_n0, assign14080_e20791_d_n2, assign14080_e20791_d_n3, assign14080_e20791_d_n4, assign14080_e20791_d_n5, assign14080_e20791_d_n6, assign14080_e20791_d_n7, assign14080_e20791_d_n8, assign14080_e20791_d_n9, assign14080_e20791_d_n10, assign14080_e20791_d_n11, assign14080_e20791_d_n13, assign14080_e20791_d_n14,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14080_e20773: f64 = (locals.var_mut2 + p.p897);
        let assign14080_e20776: f64 = (locals.var_mut2 - p.p897);
        let assign14080_e20779: f64 = (locals.var_mut2 - p.p897);
        let assign14080_e20780: f64 = (assign14080_e20776 * assign14080_e20779);
        let assign14080_e20783: f64 = (0.25 * 0.2);
        let assign14080_e20785: f64 = (assign14080_e20783 * 0.2);
        let assign14080_e20786: f64 = (assign14080_e20780 + assign14080_e20785);
        let assign14080_e20787: f64 = (assign14080_e20786).sqrt();
        let assign14080_e20788: f64 = (assign14080_e20773 - assign14080_e20787);
        let assign14080_e20789: f64 = (0.5 * assign14080_e20788);
        (assign14080_e20789, (0.5 * (locals.var_mut2_dn0 - (((locals.var_mut2_dn0 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn0)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn2 - (((locals.var_mut2_dn2 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn2)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn3 - (((locals.var_mut2_dn3 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn3)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn4 - (((locals.var_mut2_dn4 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn4)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn5 - (((locals.var_mut2_dn5 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn5)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn6 - (((locals.var_mut2_dn6 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn6)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn7 - (((locals.var_mut2_dn7 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn7)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn8 - (((locals.var_mut2_dn8 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn8)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn9 - (((locals.var_mut2_dn9 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn9)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn10 - (((locals.var_mut2_dn10 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn10)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn11 - (((locals.var_mut2_dn11 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn11)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn13 - (((locals.var_mut2_dn13 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn13)) / (2.0 * assign14080_e20787)))), (0.5 * (locals.var_mut2_dn14 - (((locals.var_mut2_dn14 * assign14080_e20779) + (assign14080_e20776 * locals.var_mut2_dn14)) / (2.0 * assign14080_e20787)))),)
    } else {
        (locals.var_eu_i, locals.var_eu_i_dn0, locals.var_eu_i_dn2, locals.var_eu_i_dn3, locals.var_eu_i_dn4, locals.var_eu_i_dn5, locals.var_eu_i_dn6, locals.var_eu_i_dn7, locals.var_eu_i_dn8, locals.var_eu_i_dn9, locals.var_eu_i_dn10, locals.var_eu_i_dn11, locals.var_eu_i_dn13, locals.var_eu_i_dn14,)
    }
};
        locals.var_eu_i = assign14080_e20791;
        locals.var_eu_i_dn0 = assign14080_e20791_d_n0;
        locals.var_eu_i_dn2 = assign14080_e20791_d_n2;
        locals.var_eu_i_dn3 = assign14080_e20791_d_n3;
        locals.var_eu_i_dn4 = assign14080_e20791_d_n4;
        locals.var_eu_i_dn5 = assign14080_e20791_d_n5;
        locals.var_eu_i_dn6 = assign14080_e20791_d_n6;
        locals.var_eu_i_dn7 = assign14080_e20791_d_n7;
        locals.var_eu_i_dn8 = assign14080_e20791_d_n8;
        locals.var_eu_i_dn9 = assign14080_e20791_d_n9;
        locals.var_eu_i_dn10 = assign14080_e20791_d_n10;
        locals.var_eu_i_dn11 = assign14080_e20791_d_n11;
        locals.var_eu_i_dn13 = assign14080_e20791_d_n13;
        locals.var_eu_i_dn14 = assign14080_e20791_d_n14;

        let (assign14090_e20799,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14090_e20796: f64 = (p.p43 + p.p40);
        let assign14090_e20797: f64 = (p.p43 / assign14090_e20796);
        (assign14090_e20797,)
    } else {
        (locals.var_mut3,)
    }
};
        locals.var_mut3 = assign14090_e20799;

        let (assign14100_e20813,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14100_e20803: f64 = (p.p905 * p.p40);
        let assign14100_e20805: f64 = (assign14100_e20803 * p.p40);
        let assign14100_e20807: f64 = (assign14100_e20805 * 1e18);
        let assign14100_e20810: f64 = (p.p906 * 0.001);
        let assign14100_e20811: f64 = (assign14100_e20807 - assign14100_e20810);
        (assign14100_e20811,)
    } else {
        (locals.var_mut4,)
    }
};
        locals.var_mut4 = assign14100_e20813;

        let (assign14110_e20851,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14110_e20818: f64 = (locals.var_mut4 * locals.var_mut4);
        let assign14110_e20821: f64 = (4.0 * p.p906);
        let assign14110_e20823: f64 = (assign14110_e20821 * 0.001);
        let assign14110_e20826: f64 = (p.p905 + 0.24);
        let assign14110_e20827: f64 = (assign14110_e20823 * assign14110_e20826);
        let assign14110_e20829: f64 = (assign14110_e20827 * p.p40);
        let assign14110_e20831: f64 = (assign14110_e20829 * p.p40);
        let assign14110_e20833: f64 = (assign14110_e20831 * 1e18);
        let assign14110_e20834: f64 = (assign14110_e20818 + assign14110_e20833);
        let assign14110_e20836: f64 = (assign14110_e20834).powf(0.5);
        let assign14110_e20837: f64 = (locals.var_mut4 + assign14110_e20836);
        let assign14110_e20841: f64 = (p.p905 + 0.24);
        let assign14110_e20842: f64 = (2.0 * assign14110_e20841);
        let assign14110_e20844: f64 = (assign14110_e20842 * p.p40);
        let assign14110_e20846: f64 = (assign14110_e20844 * p.p40);
        let assign14110_e20848: f64 = (assign14110_e20846 * 1e18);
        let assign14110_e20849: f64 = (assign14110_e20837 / assign14110_e20848);
        (assign14110_e20849,)
    } else {
        (locals.var_mut5,)
    }
};
        locals.var_mut5 = assign14110_e20851;

        let (assign14120_e20898,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14120_e20857: f64 = (locals.var_mut5 - 0.8208);
        let assign14120_e20860: f64 = (p.p907 * 1e-5);
        let assign14120_e20861: f64 = (assign14120_e20857 - assign14120_e20860);
        let assign14120_e20862: f64 = (0.0001 / assign14120_e20861);
        let assign14120_e20864: f64 = (assign14120_e20862 + 1.0);
        let assign14120_e20868: f64 = (locals.var_mut5 - 0.8208);
        let assign14120_e20871: f64 = (p.p907 * 1e-5);
        let assign14120_e20872: f64 = (assign14120_e20868 - assign14120_e20871);
        let assign14120_e20873: f64 = (0.0001 / assign14120_e20872);
        let assign14120_e20875: f64 = (assign14120_e20873 - 1.0);
        let assign14120_e20879: f64 = (locals.var_mut5 - 0.8208);
        let assign14120_e20882: f64 = (p.p907 * 1e-5);
        let assign14120_e20883: f64 = (assign14120_e20879 - assign14120_e20882);
        let assign14120_e20884: f64 = (0.0001 / assign14120_e20883);
        let assign14120_e20886: f64 = (assign14120_e20884 - 1.0);
        let assign14120_e20887: f64 = (assign14120_e20875 * assign14120_e20886);
        let assign14120_e20890: f64 = (0.25 * 0.06);
        let assign14120_e20892: f64 = (assign14120_e20890 * 0.06);
        let assign14120_e20893: f64 = (assign14120_e20887 + assign14120_e20892);
        let assign14120_e20894: f64 = (assign14120_e20893).sqrt();
        let assign14120_e20895: f64 = (assign14120_e20864 - assign14120_e20894);
        let assign14120_e20896: f64 = (0.5 * assign14120_e20895);
        (assign14120_e20896,)
    } else {
        (locals.var_mut6,)
    }
};
        locals.var_mut6 = assign14120_e20898;

    }

    pub(super) fn stamp_transient_block_39(
        ctx: &GeneratedEvalContext<'_>,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        locals: &mut StampLocals,
    ) {
        let ctx_temp = ctx.temperature();
        let nv4 = ctx.node_voltage(nodes[4]);
        let (assign14130_e20912, assign14130_e20912_d_n0, assign14130_e20912_d_n2, assign14130_e20912_d_n3, assign14130_e20912_d_n4, assign14130_e20912_d_n5, assign14130_e20912_d_n6, assign14130_e20912_d_n7, assign14130_e20912_d_n8, assign14130_e20912_d_n9, assign14130_e20912_d_n10, assign14130_e20912_d_n11, assign14130_e20912_d_n13, assign14130_e20912_d_n14,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14130_e20905: f64 = (1.0 - locals.var_mut3);
        let assign14130_e20906: f64 = (p.p904 * assign14130_e20905);
        let assign14130_e20907: f64 = (locals.var_mut3 + assign14130_e20906);
        let assign14130_e20908: f64 = (locals.var_u0_i * assign14130_e20907);
        let assign14130_e20910: f64 = (assign14130_e20908 * locals.var_mut6);
        (assign14130_e20910, ((locals.var_u0_i_dn0 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn2 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn3 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn4 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn5 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn6 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn7 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn8 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn9 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn10 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn11 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn13 * assign14130_e20907) * locals.var_mut6), ((locals.var_u0_i_dn14 * assign14130_e20907) * locals.var_mut6),)
    } else {
        (locals.var_u0_i, locals.var_u0_i_dn0, locals.var_u0_i_dn2, locals.var_u0_i_dn3, locals.var_u0_i_dn4, locals.var_u0_i_dn5, locals.var_u0_i_dn6, locals.var_u0_i_dn7, locals.var_u0_i_dn8, locals.var_u0_i_dn9, locals.var_u0_i_dn10, locals.var_u0_i_dn11, locals.var_u0_i_dn13, locals.var_u0_i_dn14,)
    }
};
        locals.var_u0_i = assign14130_e20912;
        locals.var_u0_i_dn0 = assign14130_e20912_d_n0;
        locals.var_u0_i_dn2 = assign14130_e20912_d_n2;
        locals.var_u0_i_dn3 = assign14130_e20912_d_n3;
        locals.var_u0_i_dn4 = assign14130_e20912_d_n4;
        locals.var_u0_i_dn5 = assign14130_e20912_d_n5;
        locals.var_u0_i_dn6 = assign14130_e20912_d_n6;
        locals.var_u0_i_dn7 = assign14130_e20912_d_n7;
        locals.var_u0_i_dn8 = assign14130_e20912_d_n8;
        locals.var_u0_i_dn9 = assign14130_e20912_d_n9;
        locals.var_u0_i_dn10 = assign14130_e20912_d_n10;
        locals.var_u0_i_dn11 = assign14130_e20912_d_n11;
        locals.var_u0_i_dn13 = assign14130_e20912_d_n13;
        locals.var_u0_i_dn14 = assign14130_e20912_d_n14;

        let (assign14140_e20961, assign14140_e20961_d_n0, assign14140_e20961_d_n2, assign14140_e20961_d_n3, assign14140_e20961_d_n4, assign14140_e20961_d_n5, assign14140_e20961_d_n6, assign14140_e20961_d_n7, assign14140_e20961_d_n8, assign14140_e20961_d_n9, assign14140_e20961_d_n10, assign14140_e20961_d_n11, assign14140_e20961_d_n13, assign14140_e20961_d_n14,) = {
    if (locals.var_guard235 != 0.0) {
        let assign14140_e20916: f64 = (p.p901 - locals.var_ud_i);
        let assign14140_e20920: f64 = (p.p902 * 1000000000.0);
        let assign14140_e20923: f64 = (p.p40 * 1000000000.0);
        let assign14140_e20924: f64 = (assign14140_e20920 - assign14140_e20923);
        let assign14140_e20926: f64 = assign14140_e20924;
        let assign14140_e20929: f64 = (p.p902 * 1000000000.0);
        let assign14140_e20932: f64 = (p.p40 * 1000000000.0);
        let assign14140_e20933: f64 = (assign14140_e20929 - assign14140_e20932);
        let assign14140_e20935: f64 = assign14140_e20933;
        let assign14140_e20938: f64 = (p.p902 * 1000000000.0);
        let assign14140_e20941: f64 = (p.p40 * 1000000000.0);
        let assign14140_e20942: f64 = (assign14140_e20938 - assign14140_e20941);
        let assign14140_e20944: f64 = assign14140_e20942;
        let assign14140_e20945: f64 = (assign14140_e20935 * assign14140_e20944);
        let assign14140_e20948: f64 = 0.25;
        let assign14140_e20950: f64 = assign14140_e20948;
        let assign14140_e20951: f64 = (assign14140_e20945 + assign14140_e20950);
        let assign14140_e20952: f64 = (assign14140_e20951).sqrt();
        let assign14140_e20953: f64 = (assign14140_e20926 + assign14140_e20952);
        let assign14140_e20954: f64 = (0.5 * assign14140_e20953);
        let assign14140_e20956: f64 = (assign14140_e20954).powf(p.p903);
        let assign14140_e20957: f64 = (assign14140_e20916 * assign14140_e20956);
        let assign14140_e20959: f64 = (assign14140_e20957 + locals.var_ud_i);
        (assign14140_e20959, (((-locals.var_ud_i_dn0) * assign14140_e20956) + locals.var_ud_i_dn0), (((-locals.var_ud_i_dn2) * assign14140_e20956) + locals.var_ud_i_dn2), (((-locals.var_ud_i_dn3) * assign14140_e20956) + locals.var_ud_i_dn3), (((-locals.var_ud_i_dn4) * assign14140_e20956) + locals.var_ud_i_dn4), (((-locals.var_ud_i_dn5) * assign14140_e20956) + locals.var_ud_i_dn5), (((-locals.var_ud_i_dn6) * assign14140_e20956) + locals.var_ud_i_dn6), (((-locals.var_ud_i_dn7) * assign14140_e20956) + locals.var_ud_i_dn7), (((-locals.var_ud_i_dn8) * assign14140_e20956) + locals.var_ud_i_dn8), (((-locals.var_ud_i_dn9) * assign14140_e20956) + locals.var_ud_i_dn9), (((-locals.var_ud_i_dn10) * assign14140_e20956) + locals.var_ud_i_dn10), (((-locals.var_ud_i_dn11) * assign14140_e20956) + locals.var_ud_i_dn11), (((-locals.var_ud_i_dn13) * assign14140_e20956) + locals.var_ud_i_dn13), (((-locals.var_ud_i_dn14) * assign14140_e20956) + locals.var_ud_i_dn14),)
    } else {
        (locals.var_ud_i, locals.var_ud_i_dn0, locals.var_ud_i_dn2, locals.var_ud_i_dn3, locals.var_ud_i_dn4, locals.var_ud_i_dn5, locals.var_ud_i_dn6, locals.var_ud_i_dn7, locals.var_ud_i_dn8, locals.var_ud_i_dn9, locals.var_ud_i_dn10, locals.var_ud_i_dn11, locals.var_ud_i_dn13, locals.var_ud_i_dn14,)
    }
};
        locals.var_ud_i = assign14140_e20961;
        locals.var_ud_i_dn0 = assign14140_e20961_d_n0;
        locals.var_ud_i_dn2 = assign14140_e20961_d_n2;
        locals.var_ud_i_dn3 = assign14140_e20961_d_n3;
        locals.var_ud_i_dn4 = assign14140_e20961_d_n4;
        locals.var_ud_i_dn5 = assign14140_e20961_d_n5;
        locals.var_ud_i_dn6 = assign14140_e20961_d_n6;
        locals.var_ud_i_dn7 = assign14140_e20961_d_n7;
        locals.var_ud_i_dn8 = assign14140_e20961_d_n8;
        locals.var_ud_i_dn9 = assign14140_e20961_d_n9;
        locals.var_ud_i_dn10 = assign14140_e20961_d_n10;
        locals.var_ud_i_dn11 = assign14140_e20961_d_n11;
        locals.var_ud_i_dn13 = assign14140_e20961_d_n13;
        locals.var_ud_i_dn14 = assign14140_e20961_d_n14;

        let assign14150_e20968: f64 = if ((p.p74 != 0.0) && (p.p1791 > 0.0)) { 1.0 } else { 0.0 };
        locals.var_guard236 = assign14150_e20968;

        let (assign14160_e20976, assign14160_e20976_d_n4,) = {
    if (locals.var_guard236 != 0.0) {
        let assign14160_e20970: f64 = ctx_temp;
        let assign14160_e20972: f64 = (assign14160_e20970 + (nv4 - 0.0));
        let assign14160_e20974: f64 = (assign14160_e20972 + p.p22);
        (assign14160_e20974, 1.0,)
    } else {
        (locals.var_devtemp, locals.var_devtemp_dn4,)
    }
};
        locals.var_devtemp = assign14160_e20976;
        locals.var_devtemp_dn4 = assign14160_e20976_d_n4;

        let (assign14170_e20983, assign14170_e20983_d_n4,) = {
    if (locals.var_guard236 == 0.0) {
        let assign14170_e20979: f64 = ctx_temp;
        let assign14170_e20981: f64 = (assign14170_e20979 + p.p22);
        (assign14170_e20981, 0.0,)
    } else {
        (locals.var_devtemp, locals.var_devtemp_dn4,)
    }
};
        locals.var_devtemp = assign14170_e20983;
        locals.var_devtemp_dn4 = assign14170_e20983_d_n4;

        let assign14180_e20986: f64 = (locals.var_devtemp / locals.var_tnom);
        locals.var_tratio = assign14180_e20986;
        locals.var_tratio_dn4 = (locals.var_devtemp_dn4 / locals.var_tnom);

        let assign14190_e20989: f64 = (locals.var_tratio - 1.0);
        locals.var_tratio_m1 = assign14190_e20989;
        locals.var_tratio_m1_dn4 = locals.var_tratio_dn4;

        let assign14200_e20992: f64 = (locals.var_devtemp - locals.var_tnom);
        locals.var_deltemp = assign14200_e20992;
        locals.var_deltemp_dn4 = locals.var_devtemp_dn4;

        let assign14210_e20995: f64 = (8.617087e-5 * locals.var_devtemp);
        locals.var_vtm = assign14210_e20995;
        locals.var_vtm_dn4 = (8.617087e-5 * locals.var_devtemp_dn4);

        let assign14220_e20998: f64 = (8.617087e-5 * locals.var_tnom);
        locals.var_vtm0 = assign14220_e20998;

        locals.var_tlow = p.p1786;

        let assign14240_e21002: f64 = if p.p80 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard237 = assign14240_e21002;

        let (assign14250_e21025, assign14250_e21025_d_n4,) = {
    if (locals.var_guard237 != 0.0) {
        let assign14250_e21007: f64 = (locals.var_devtemp + locals.var_tlow);
        let assign14250_e21010: f64 = (locals.var_devtemp - locals.var_tlow);
        let assign14250_e21013: f64 = (locals.var_devtemp - locals.var_tlow);
        let assign14250_e21014: f64 = (assign14250_e21010 * assign14250_e21013);
        let assign14250_e21017: f64 = (0.25 * p.p1788);
        let assign14250_e21019: f64 = (assign14250_e21017 * p.p1788);
        let assign14250_e21020: f64 = (assign14250_e21014 + assign14250_e21019);
        let assign14250_e21021: f64 = (assign14250_e21020).sqrt();
        let assign14250_e21022: f64 = (assign14250_e21007 + assign14250_e21021);
        let assign14250_e21023: f64 = (0.5 * assign14250_e21022);
        (assign14250_e21023, (0.5 * (locals.var_devtemp_dn4 + (((locals.var_devtemp_dn4 * assign14250_e21013) + (assign14250_e21010 * locals.var_devtemp_dn4)) / (2.0 * assign14250_e21021)))),)
    } else {
        (locals.var_devtemplow0, locals.var_devtemplow0_dn4,)
    }
};
        locals.var_devtemplow0 = assign14250_e21025;
        locals.var_devtemplow0_dn4 = assign14250_e21025_d_n4;

        let (assign14260_e21063, assign14260_e21063_d_n4,) = {
    if (locals.var_guard237 != 0.0) {
        let assign14260_e21029: f64 = (-p.p1790);
        let assign14260_e21032: f64 = (locals.var_devtemp - p.p1787);
        let assign14260_e21033: f64 = (assign14260_e21029 * assign14260_e21032);
        let assign14260_e21035: f64 = assign14260_e21033;
        let assign14260_e21037: f64 = (-p.p1790);
        let assign14260_e21040: f64 = (locals.var_devtemp - p.p1787);
        let assign14260_e21041: f64 = (assign14260_e21037 * assign14260_e21040);
        let assign14260_e21043: f64 = assign14260_e21041;
        let assign14260_e21045: f64 = (-p.p1790);
        let assign14260_e21048: f64 = (locals.var_devtemp - p.p1787);
        let assign14260_e21049: f64 = (assign14260_e21045 * assign14260_e21048);
        let assign14260_e21051: f64 = assign14260_e21049;
        let assign14260_e21052: f64 = (assign14260_e21043 * assign14260_e21051);
        let assign14260_e21055: f64 = (0.25 * p.p1789);
        let assign14260_e21057: f64 = (assign14260_e21055 * p.p1789);
        let assign14260_e21058: f64 = (assign14260_e21052 + assign14260_e21057);
        let assign14260_e21059: f64 = (assign14260_e21058).sqrt();
        let assign14260_e21060: f64 = (assign14260_e21035 + assign14260_e21059);
        let assign14260_e21061: f64 = (0.5 * assign14260_e21060);
        (assign14260_e21061, (0.5 * ((assign14260_e21029 * locals.var_devtemp_dn4) + ((((assign14260_e21037 * locals.var_devtemp_dn4) * assign14260_e21051) + (assign14260_e21043 * (assign14260_e21045 * locals.var_devtemp_dn4))) / (2.0 * assign14260_e21059)))),)
    } else {
        (locals.var_devtemplow1, locals.var_devtemplow1_dn4,)
    }
};
        locals.var_devtemplow1 = assign14260_e21063;
        locals.var_devtemplow1_dn4 = assign14260_e21063_d_n4;

        let assign14270_e21066: f64 = if p.p80 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard238 = assign14270_e21066;

        let (assign14280_e21091, assign14280_e21091_d_n0, assign14280_e21091_d_n2, assign14280_e21091_d_n3, assign14280_e21091_d_n4, assign14280_e21091_d_n5, assign14280_e21091_d_n6, assign14280_e21091_d_n7, assign14280_e21091_d_n8, assign14280_e21091_d_n9, assign14280_e21091_d_n10, assign14280_e21091_d_n11, assign14280_e21091_d_n13, assign14280_e21091_d_n14,) = {
    if ((locals.var_guard237 != 0.0) && (locals.var_guard238 != 0.0)) {
        let assign14280_e21073: f64 = (locals.var_tnom + locals.var_tlow);
        let assign14280_e21076: f64 = (locals.var_tnom - locals.var_tlow);
        let assign14280_e21079: f64 = (locals.var_tnom - locals.var_tlow);
        let assign14280_e21080: f64 = (assign14280_e21076 * assign14280_e21079);
        let assign14280_e21083: f64 = (0.25 * p.p1788);
        let assign14280_e21085: f64 = (assign14280_e21083 * p.p1788);
        let assign14280_e21086: f64 = (assign14280_e21080 + assign14280_e21085);
        let assign14280_e21087: f64 = (assign14280_e21086).sqrt();
        let assign14280_e21088: f64 = (assign14280_e21073 + assign14280_e21087);
        let assign14280_e21089: f64 = (0.5 * assign14280_e21088);
        (assign14280_e21089, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign14280_e21091;
        locals.var_t1_dn0 = assign14280_e21091_d_n0;
        locals.var_t1_dn2 = assign14280_e21091_d_n2;
        locals.var_t1_dn3 = assign14280_e21091_d_n3;
        locals.var_t1_dn4 = assign14280_e21091_d_n4;
        locals.var_t1_dn5 = assign14280_e21091_d_n5;
        locals.var_t1_dn6 = assign14280_e21091_d_n6;
        locals.var_t1_dn7 = assign14280_e21091_d_n7;
        locals.var_t1_dn8 = assign14280_e21091_d_n8;
        locals.var_t1_dn9 = assign14280_e21091_d_n9;
        locals.var_t1_dn10 = assign14280_e21091_d_n10;
        locals.var_t1_dn11 = assign14280_e21091_d_n11;
        locals.var_t1_dn13 = assign14280_e21091_d_n13;
        locals.var_t1_dn14 = assign14280_e21091_d_n14;

        let (assign14290_e21131, assign14290_e21131_d_n0, assign14290_e21131_d_n2, assign14290_e21131_d_n3, assign14290_e21131_d_n4, assign14290_e21131_d_n5, assign14290_e21131_d_n6, assign14290_e21131_d_n7, assign14290_e21131_d_n8, assign14290_e21131_d_n9, assign14290_e21131_d_n10, assign14290_e21131_d_n11, assign14290_e21131_d_n13, assign14290_e21131_d_n14,) = {
    if ((locals.var_guard237 != 0.0) && (locals.var_guard238 != 0.0)) {
        let assign14290_e21097: f64 = (-p.p1790);
        let assign14290_e21100: f64 = (locals.var_tnom - p.p1787);
        let assign14290_e21101: f64 = (assign14290_e21097 * assign14290_e21100);
        let assign14290_e21103: f64 = assign14290_e21101;
        let assign14290_e21105: f64 = (-p.p1790);
        let assign14290_e21108: f64 = (locals.var_tnom - p.p1787);
        let assign14290_e21109: f64 = (assign14290_e21105 * assign14290_e21108);
        let assign14290_e21111: f64 = assign14290_e21109;
        let assign14290_e21113: f64 = (-p.p1790);
        let assign14290_e21116: f64 = (locals.var_tnom - p.p1787);
        let assign14290_e21117: f64 = (assign14290_e21113 * assign14290_e21116);
        let assign14290_e21119: f64 = assign14290_e21117;
        let assign14290_e21120: f64 = (assign14290_e21111 * assign14290_e21119);
        let assign14290_e21123: f64 = (0.25 * p.p1789);
        let assign14290_e21125: f64 = (assign14290_e21123 * p.p1789);
        let assign14290_e21126: f64 = (assign14290_e21120 + assign14290_e21125);
        let assign14290_e21127: f64 = (assign14290_e21126).sqrt();
        let assign14290_e21128: f64 = (assign14290_e21103 + assign14290_e21127);
        let assign14290_e21129: f64 = (0.5 * assign14290_e21128);
        (assign14290_e21129, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign14290_e21131;
        locals.var_t2_dn0 = assign14290_e21131_d_n0;
        locals.var_t2_dn2 = assign14290_e21131_d_n2;
        locals.var_t2_dn3 = assign14290_e21131_d_n3;
        locals.var_t2_dn4 = assign14290_e21131_d_n4;
        locals.var_t2_dn5 = assign14290_e21131_d_n5;
        locals.var_t2_dn6 = assign14290_e21131_d_n6;
        locals.var_t2_dn7 = assign14290_e21131_d_n7;
        locals.var_t2_dn8 = assign14290_e21131_d_n8;
        locals.var_t2_dn9 = assign14290_e21131_d_n9;
        locals.var_t2_dn10 = assign14290_e21131_d_n10;
        locals.var_t2_dn11 = assign14290_e21131_d_n11;
        locals.var_t2_dn13 = assign14290_e21131_d_n13;
        locals.var_t2_dn14 = assign14290_e21131_d_n14;

        let assign14300_e21134: f64 = if locals.var_tnom > locals.var_tlow { 1.0 } else { 0.0 };
        locals.var_guard239 = assign14300_e21134;

        let (assign14310_e21150, assign14310_e21150_d_n0, assign14310_e21150_d_n2, assign14310_e21150_d_n3, assign14310_e21150_d_n4, assign14310_e21150_d_n5, assign14310_e21150_d_n6, assign14310_e21150_d_n7, assign14310_e21150_d_n8, assign14310_e21150_d_n9, assign14310_e21150_d_n10, assign14310_e21150_d_n11, assign14310_e21150_d_n13, assign14310_e21150_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 != 0.0)) && (locals.var_guard239 != 0.0)) {
        let assign14310_e21142: f64 = (locals.var_devtemplow0 + locals.var_devtemplow1);
        let assign14310_e21144: f64 = (assign14310_e21142 - locals.var_t1);
        let assign14310_e21146: f64 = (assign14310_e21144 - locals.var_t2);
        let assign14310_e21148: f64 = (assign14310_e21146 + locals.var_tnom);
        (assign14310_e21148, ((-locals.var_t1_dn0) - locals.var_t2_dn0), ((-locals.var_t1_dn2) - locals.var_t2_dn2), ((-locals.var_t1_dn3) - locals.var_t2_dn3), (((locals.var_devtemplow0_dn4 + locals.var_devtemplow1_dn4) - locals.var_t1_dn4) - locals.var_t2_dn4), ((-locals.var_t1_dn5) - locals.var_t2_dn5), ((-locals.var_t1_dn6) - locals.var_t2_dn6), ((-locals.var_t1_dn7) - locals.var_t2_dn7), ((-locals.var_t1_dn8) - locals.var_t2_dn8), ((-locals.var_t1_dn9) - locals.var_t2_dn9), ((-locals.var_t1_dn10) - locals.var_t2_dn10), ((-locals.var_t1_dn11) - locals.var_t2_dn11), ((-locals.var_t1_dn13) - locals.var_t2_dn13), ((-locals.var_t1_dn14) - locals.var_t2_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign14310_e21150;
        locals.var_t3_dn0 = assign14310_e21150_d_n0;
        locals.var_t3_dn2 = assign14310_e21150_d_n2;
        locals.var_t3_dn3 = assign14310_e21150_d_n3;
        locals.var_t3_dn4 = assign14310_e21150_d_n4;
        locals.var_t3_dn5 = assign14310_e21150_d_n5;
        locals.var_t3_dn6 = assign14310_e21150_d_n6;
        locals.var_t3_dn7 = assign14310_e21150_d_n7;
        locals.var_t3_dn8 = assign14310_e21150_d_n8;
        locals.var_t3_dn9 = assign14310_e21150_d_n9;
        locals.var_t3_dn10 = assign14310_e21150_d_n10;
        locals.var_t3_dn11 = assign14310_e21150_d_n11;
        locals.var_t3_dn13 = assign14310_e21150_d_n13;
        locals.var_t3_dn14 = assign14310_e21150_d_n14;

        let (assign14320_e21167, assign14320_e21167_d_n0, assign14320_e21167_d_n2, assign14320_e21167_d_n3, assign14320_e21167_d_n4, assign14320_e21167_d_n5, assign14320_e21167_d_n6, assign14320_e21167_d_n7, assign14320_e21167_d_n8, assign14320_e21167_d_n9, assign14320_e21167_d_n10, assign14320_e21167_d_n11, assign14320_e21167_d_n13, assign14320_e21167_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 != 0.0)) && (locals.var_guard239 == 0.0)) {
        let assign14320_e21159: f64 = (locals.var_devtemplow0 + locals.var_devtemplow1);
        let assign14320_e21161: f64 = (assign14320_e21159 - locals.var_t1);
        let assign14320_e21163: f64 = (assign14320_e21161 - locals.var_t2);
        let assign14320_e21165: f64 = (assign14320_e21163 + locals.var_tlow);
        (assign14320_e21165, ((-locals.var_t1_dn0) - locals.var_t2_dn0), ((-locals.var_t1_dn2) - locals.var_t2_dn2), ((-locals.var_t1_dn3) - locals.var_t2_dn3), (((locals.var_devtemplow0_dn4 + locals.var_devtemplow1_dn4) - locals.var_t1_dn4) - locals.var_t2_dn4), ((-locals.var_t1_dn5) - locals.var_t2_dn5), ((-locals.var_t1_dn6) - locals.var_t2_dn6), ((-locals.var_t1_dn7) - locals.var_t2_dn7), ((-locals.var_t1_dn8) - locals.var_t2_dn8), ((-locals.var_t1_dn9) - locals.var_t2_dn9), ((-locals.var_t1_dn10) - locals.var_t2_dn10), ((-locals.var_t1_dn11) - locals.var_t2_dn11), ((-locals.var_t1_dn13) - locals.var_t2_dn13), ((-locals.var_t1_dn14) - locals.var_t2_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign14320_e21167;
        locals.var_t3_dn0 = assign14320_e21167_d_n0;
        locals.var_t3_dn2 = assign14320_e21167_d_n2;
        locals.var_t3_dn3 = assign14320_e21167_d_n3;
        locals.var_t3_dn4 = assign14320_e21167_d_n4;
        locals.var_t3_dn5 = assign14320_e21167_d_n5;
        locals.var_t3_dn6 = assign14320_e21167_d_n6;
        locals.var_t3_dn7 = assign14320_e21167_d_n7;
        locals.var_t3_dn8 = assign14320_e21167_d_n8;
        locals.var_t3_dn9 = assign14320_e21167_d_n9;
        locals.var_t3_dn10 = assign14320_e21167_d_n10;
        locals.var_t3_dn11 = assign14320_e21167_d_n11;
        locals.var_t3_dn13 = assign14320_e21167_d_n13;
        locals.var_t3_dn14 = assign14320_e21167_d_n14;

        let (assign14330_e21192, assign14330_e21192_d_n0, assign14330_e21192_d_n2, assign14330_e21192_d_n3, assign14330_e21192_d_n4, assign14330_e21192_d_n5, assign14330_e21192_d_n6, assign14330_e21192_d_n7, assign14330_e21192_d_n8, assign14330_e21192_d_n9, assign14330_e21192_d_n10, assign14330_e21192_d_n11, assign14330_e21192_d_n13, assign14330_e21192_d_n14,) = {
    if ((locals.var_guard237 != 0.0) && (locals.var_guard238 != 0.0)) {
        let assign14330_e21174: f64 = (locals.var_devtemp + locals.var_t3);
        let assign14330_e21177: f64 = (locals.var_devtemp - locals.var_t3);
        let assign14330_e21180: f64 = (locals.var_devtemp - locals.var_t3);
        let assign14330_e21181: f64 = (assign14330_e21177 * assign14330_e21180);
        let assign14330_e21184: f64 = (0.25 * 0.2);
        let assign14330_e21186: f64 = (assign14330_e21184 * 0.2);
        let assign14330_e21187: f64 = (assign14330_e21181 + assign14330_e21186);
        let assign14330_e21188: f64 = (assign14330_e21187).sqrt();
        let assign14330_e21189: f64 = (assign14330_e21174 + assign14330_e21188);
        let assign14330_e21190: f64 = (0.5 * assign14330_e21189);
        (assign14330_e21190, (0.5 * (locals.var_t3_dn0 + ((((-locals.var_t3_dn0) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn0))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn2 + ((((-locals.var_t3_dn2) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn2))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn3 + ((((-locals.var_t3_dn3) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn3))) / (2.0 * assign14330_e21188)))), (0.5 * ((locals.var_devtemp_dn4 + locals.var_t3_dn4) + ((((locals.var_devtemp_dn4 - locals.var_t3_dn4) * assign14330_e21180) + (assign14330_e21177 * (locals.var_devtemp_dn4 - locals.var_t3_dn4))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn5 + ((((-locals.var_t3_dn5) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn5))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn6 + ((((-locals.var_t3_dn6) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn6))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn7 + ((((-locals.var_t3_dn7) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn7))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn8 + ((((-locals.var_t3_dn8) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn8))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn9 + ((((-locals.var_t3_dn9) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn9))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn10 + ((((-locals.var_t3_dn10) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn10))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn11 + ((((-locals.var_t3_dn11) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn11))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn13 + ((((-locals.var_t3_dn13) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn13))) / (2.0 * assign14330_e21188)))), (0.5 * (locals.var_t3_dn14 + ((((-locals.var_t3_dn14) * assign14330_e21180) + (assign14330_e21177 * (-locals.var_t3_dn14))) / (2.0 * assign14330_e21188)))),)
    } else {
        (locals.var_devtempeff, locals.var_devtempeff_dn0, locals.var_devtempeff_dn2, locals.var_devtempeff_dn3, locals.var_devtempeff_dn4, locals.var_devtempeff_dn5, locals.var_devtempeff_dn6, locals.var_devtempeff_dn7, locals.var_devtempeff_dn8, locals.var_devtempeff_dn9, locals.var_devtempeff_dn10, locals.var_devtempeff_dn11, locals.var_devtempeff_dn13, locals.var_devtempeff_dn14,)
    }
};
        locals.var_devtempeff = assign14330_e21192;
        locals.var_devtempeff_dn0 = assign14330_e21192_d_n0;
        locals.var_devtempeff_dn2 = assign14330_e21192_d_n2;
        locals.var_devtempeff_dn3 = assign14330_e21192_d_n3;
        locals.var_devtempeff_dn4 = assign14330_e21192_d_n4;
        locals.var_devtempeff_dn5 = assign14330_e21192_d_n5;
        locals.var_devtempeff_dn6 = assign14330_e21192_d_n6;
        locals.var_devtempeff_dn7 = assign14330_e21192_d_n7;
        locals.var_devtempeff_dn8 = assign14330_e21192_d_n8;
        locals.var_devtempeff_dn9 = assign14330_e21192_d_n9;
        locals.var_devtempeff_dn10 = assign14330_e21192_d_n10;
        locals.var_devtempeff_dn11 = assign14330_e21192_d_n11;
        locals.var_devtempeff_dn13 = assign14330_e21192_d_n13;
        locals.var_devtempeff_dn14 = assign14330_e21192_d_n14;

        let assign14340_e21195: f64 = if locals.var_tlow > 210.0 { 1.0 } else { 0.0 };
        locals.var_guard240 = assign14340_e21195;

        let (assign14350_e21204,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard240 != 0.0)) {
        (210.0,)
    } else {
        (locals.var_tlow,)
    }
};
        locals.var_tlow = assign14350_e21204;

        let (assign14360_e21220, assign14360_e21220_d_n4,) = {
    if ((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) {
        let assign14360_e21214: f64 = (locals.var_devtemp - 210.0);
        let assign14360_e21215: f64 = (0.5 * assign14360_e21214);
        let assign14360_e21216: f64 = (assign14360_e21215).tanh();
        let assign14360_e21217: f64 = (0.5 * assign14360_e21216);
        let assign14360_e21218: f64 = (0.5 + assign14360_e21217);
        (assign14360_e21218, (0.5 * ((0.5 * locals.var_devtemp_dn4) / ((assign14360_e21215).cosh() * (assign14360_e21215).cosh()))),)
    } else {
        (locals.var_wh, locals.var_wh_dn4,)
    }
};
        locals.var_wh = assign14360_e21220;
        locals.var_wh_dn4 = assign14360_e21220_d_n4;

        let (assign14370_e21229, assign14370_e21229_d_n4,) = {
    if ((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) {
        let assign14370_e21227: f64 = (1.0 - locals.var_wh);
        (assign14370_e21227, (-locals.var_wh_dn4),)
    } else {
        (locals.var_wl, locals.var_wl_dn4,)
    }
};
        locals.var_wl = assign14370_e21229;
        locals.var_wl_dn4 = assign14370_e21229_d_n4;

        let assign14380_e21232: f64 = if locals.var_tnom > 210.0 { 1.0 } else { 0.0 };
        locals.var_guard241 = assign14380_e21232;

        let (assign14390_e21260, assign14390_e21260_d_n0, assign14390_e21260_d_n2, assign14390_e21260_d_n3, assign14390_e21260_d_n4, assign14390_e21260_d_n5, assign14390_e21260_d_n6, assign14390_e21260_d_n7, assign14390_e21260_d_n8, assign14390_e21260_d_n9, assign14390_e21260_d_n10, assign14390_e21260_d_n11, assign14390_e21260_d_n13, assign14390_e21260_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 != 0.0)) {
        let assign14390_e21242: f64 = (210.0 + locals.var_tlow);
        let assign14390_e21245: f64 = (210.0 - locals.var_tlow);
        let assign14390_e21248: f64 = (210.0 - locals.var_tlow);
        let assign14390_e21249: f64 = (assign14390_e21245 * assign14390_e21248);
        let assign14390_e21252: f64 = (0.25 * p.p1788);
        let assign14390_e21254: f64 = (assign14390_e21252 * p.p1788);
        let assign14390_e21255: f64 = (assign14390_e21249 + assign14390_e21254);
        let assign14390_e21256: f64 = (assign14390_e21255).sqrt();
        let assign14390_e21257: f64 = (assign14390_e21242 + assign14390_e21256);
        let assign14390_e21258: f64 = (0.5 * assign14390_e21257);
        (assign14390_e21258, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign14390_e21260;
        locals.var_t1_dn0 = assign14390_e21260_d_n0;
        locals.var_t1_dn2 = assign14390_e21260_d_n2;
        locals.var_t1_dn3 = assign14390_e21260_d_n3;
        locals.var_t1_dn4 = assign14390_e21260_d_n4;
        locals.var_t1_dn5 = assign14390_e21260_d_n5;
        locals.var_t1_dn6 = assign14390_e21260_d_n6;
        locals.var_t1_dn7 = assign14390_e21260_d_n7;
        locals.var_t1_dn8 = assign14390_e21260_d_n8;
        locals.var_t1_dn9 = assign14390_e21260_d_n9;
        locals.var_t1_dn10 = assign14390_e21260_d_n10;
        locals.var_t1_dn11 = assign14390_e21260_d_n11;
        locals.var_t1_dn13 = assign14390_e21260_d_n13;
        locals.var_t1_dn14 = assign14390_e21260_d_n14;

        let (assign14400_e21303, assign14400_e21303_d_n0, assign14400_e21303_d_n2, assign14400_e21303_d_n3, assign14400_e21303_d_n4, assign14400_e21303_d_n5, assign14400_e21303_d_n6, assign14400_e21303_d_n7, assign14400_e21303_d_n8, assign14400_e21303_d_n9, assign14400_e21303_d_n10, assign14400_e21303_d_n11, assign14400_e21303_d_n13, assign14400_e21303_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 != 0.0)) {
        let assign14400_e21269: f64 = (-p.p1790);
        let assign14400_e21272: f64 = (210.0 - p.p1787);
        let assign14400_e21273: f64 = (assign14400_e21269 * assign14400_e21272);
        let assign14400_e21275: f64 = assign14400_e21273;
        let assign14400_e21277: f64 = (-p.p1790);
        let assign14400_e21280: f64 = (210.0 - p.p1787);
        let assign14400_e21281: f64 = (assign14400_e21277 * assign14400_e21280);
        let assign14400_e21283: f64 = assign14400_e21281;
        let assign14400_e21285: f64 = (-p.p1790);
        let assign14400_e21288: f64 = (210.0 - p.p1787);
        let assign14400_e21289: f64 = (assign14400_e21285 * assign14400_e21288);
        let assign14400_e21291: f64 = assign14400_e21289;
        let assign14400_e21292: f64 = (assign14400_e21283 * assign14400_e21291);
        let assign14400_e21295: f64 = (0.25 * p.p1789);
        let assign14400_e21297: f64 = (assign14400_e21295 * p.p1789);
        let assign14400_e21298: f64 = (assign14400_e21292 + assign14400_e21297);
        let assign14400_e21299: f64 = (assign14400_e21298).sqrt();
        let assign14400_e21300: f64 = (assign14400_e21275 + assign14400_e21299);
        let assign14400_e21301: f64 = (0.5 * assign14400_e21300);
        (assign14400_e21301, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign14400_e21303;
        locals.var_t2_dn0 = assign14400_e21303_d_n0;
        locals.var_t2_dn2 = assign14400_e21303_d_n2;
        locals.var_t2_dn3 = assign14400_e21303_d_n3;
        locals.var_t2_dn4 = assign14400_e21303_d_n4;
        locals.var_t2_dn5 = assign14400_e21303_d_n5;
        locals.var_t2_dn6 = assign14400_e21303_d_n6;
        locals.var_t2_dn7 = assign14400_e21303_d_n7;
        locals.var_t2_dn8 = assign14400_e21303_d_n8;
        locals.var_t2_dn9 = assign14400_e21303_d_n9;
        locals.var_t2_dn10 = assign14400_e21303_d_n10;
        locals.var_t2_dn11 = assign14400_e21303_d_n11;
        locals.var_t2_dn13 = assign14400_e21303_d_n13;
        locals.var_t2_dn14 = assign14400_e21303_d_n14;

        let (assign14410_e21320, assign14410_e21320_d_n0, assign14410_e21320_d_n2, assign14410_e21320_d_n3, assign14410_e21320_d_n4, assign14410_e21320_d_n5, assign14410_e21320_d_n6, assign14410_e21320_d_n7, assign14410_e21320_d_n8, assign14410_e21320_d_n9, assign14410_e21320_d_n10, assign14410_e21320_d_n11, assign14410_e21320_d_n13, assign14410_e21320_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 != 0.0)) {
        let assign14410_e21312: f64 = (locals.var_devtemplow0 + locals.var_devtemplow1);
        let assign14410_e21314: f64 = (assign14410_e21312 - locals.var_t1);
        let assign14410_e21316: f64 = (assign14410_e21314 - locals.var_t2);
        let assign14410_e21318: f64 = (assign14410_e21316 + 210.0);
        (assign14410_e21318, ((-locals.var_t1_dn0) - locals.var_t2_dn0), ((-locals.var_t1_dn2) - locals.var_t2_dn2), ((-locals.var_t1_dn3) - locals.var_t2_dn3), (((locals.var_devtemplow0_dn4 + locals.var_devtemplow1_dn4) - locals.var_t1_dn4) - locals.var_t2_dn4), ((-locals.var_t1_dn5) - locals.var_t2_dn5), ((-locals.var_t1_dn6) - locals.var_t2_dn6), ((-locals.var_t1_dn7) - locals.var_t2_dn7), ((-locals.var_t1_dn8) - locals.var_t2_dn8), ((-locals.var_t1_dn9) - locals.var_t2_dn9), ((-locals.var_t1_dn10) - locals.var_t2_dn10), ((-locals.var_t1_dn11) - locals.var_t2_dn11), ((-locals.var_t1_dn13) - locals.var_t2_dn13), ((-locals.var_t1_dn14) - locals.var_t2_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign14410_e21320;
        locals.var_t3_dn0 = assign14410_e21320_d_n0;
        locals.var_t3_dn2 = assign14410_e21320_d_n2;
        locals.var_t3_dn3 = assign14410_e21320_d_n3;
        locals.var_t3_dn4 = assign14410_e21320_d_n4;
        locals.var_t3_dn5 = assign14410_e21320_d_n5;
        locals.var_t3_dn6 = assign14410_e21320_d_n6;
        locals.var_t3_dn7 = assign14410_e21320_d_n7;
        locals.var_t3_dn8 = assign14410_e21320_d_n8;
        locals.var_t3_dn9 = assign14410_e21320_d_n9;
        locals.var_t3_dn10 = assign14410_e21320_d_n10;
        locals.var_t3_dn11 = assign14410_e21320_d_n11;
        locals.var_t3_dn13 = assign14410_e21320_d_n13;
        locals.var_t3_dn14 = assign14410_e21320_d_n14;

        let (assign14420_e21348, assign14420_e21348_d_n0, assign14420_e21348_d_n2, assign14420_e21348_d_n3, assign14420_e21348_d_n4, assign14420_e21348_d_n5, assign14420_e21348_d_n6, assign14420_e21348_d_n7, assign14420_e21348_d_n8, assign14420_e21348_d_n9, assign14420_e21348_d_n10, assign14420_e21348_d_n11, assign14420_e21348_d_n13, assign14420_e21348_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 != 0.0)) {
        let assign14420_e21330: f64 = (locals.var_devtemp + locals.var_t3);
        let assign14420_e21333: f64 = (locals.var_devtemp - locals.var_t3);
        let assign14420_e21336: f64 = (locals.var_devtemp - locals.var_t3);
        let assign14420_e21337: f64 = (assign14420_e21333 * assign14420_e21336);
        let assign14420_e21340: f64 = (0.25 * 0.2);
        let assign14420_e21342: f64 = (assign14420_e21340 * 0.2);
        let assign14420_e21343: f64 = (assign14420_e21337 + assign14420_e21342);
        let assign14420_e21344: f64 = (assign14420_e21343).sqrt();
        let assign14420_e21345: f64 = (assign14420_e21330 + assign14420_e21344);
        let assign14420_e21346: f64 = (0.5 * assign14420_e21345);
        (assign14420_e21346, (0.5 * (locals.var_t3_dn0 + ((((-locals.var_t3_dn0) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn0))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn2 + ((((-locals.var_t3_dn2) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn2))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn3 + ((((-locals.var_t3_dn3) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn3))) / (2.0 * assign14420_e21344)))), (0.5 * ((locals.var_devtemp_dn4 + locals.var_t3_dn4) + ((((locals.var_devtemp_dn4 - locals.var_t3_dn4) * assign14420_e21336) + (assign14420_e21333 * (locals.var_devtemp_dn4 - locals.var_t3_dn4))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn5 + ((((-locals.var_t3_dn5) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn5))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn6 + ((((-locals.var_t3_dn6) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn6))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn7 + ((((-locals.var_t3_dn7) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn7))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn8 + ((((-locals.var_t3_dn8) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn8))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn9 + ((((-locals.var_t3_dn9) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn9))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn10 + ((((-locals.var_t3_dn10) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn10))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn11 + ((((-locals.var_t3_dn11) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn11))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn13 + ((((-locals.var_t3_dn13) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn13))) / (2.0 * assign14420_e21344)))), (0.5 * (locals.var_t3_dn14 + ((((-locals.var_t3_dn14) * assign14420_e21336) + (assign14420_e21333 * (-locals.var_t3_dn14))) / (2.0 * assign14420_e21344)))),)
    } else {
        (locals.var_devtempeff, locals.var_devtempeff_dn0, locals.var_devtempeff_dn2, locals.var_devtempeff_dn3, locals.var_devtempeff_dn4, locals.var_devtempeff_dn5, locals.var_devtempeff_dn6, locals.var_devtempeff_dn7, locals.var_devtempeff_dn8, locals.var_devtempeff_dn9, locals.var_devtempeff_dn10, locals.var_devtempeff_dn11, locals.var_devtempeff_dn13, locals.var_devtempeff_dn14,)
    }
};
        locals.var_devtempeff = assign14420_e21348;
        locals.var_devtempeff_dn0 = assign14420_e21348_d_n0;
        locals.var_devtempeff_dn2 = assign14420_e21348_d_n2;
        locals.var_devtempeff_dn3 = assign14420_e21348_d_n3;
        locals.var_devtempeff_dn4 = assign14420_e21348_d_n4;
        locals.var_devtempeff_dn5 = assign14420_e21348_d_n5;
        locals.var_devtempeff_dn6 = assign14420_e21348_d_n6;
        locals.var_devtempeff_dn7 = assign14420_e21348_d_n7;
        locals.var_devtempeff_dn8 = assign14420_e21348_d_n8;
        locals.var_devtempeff_dn9 = assign14420_e21348_d_n9;
        locals.var_devtempeff_dn10 = assign14420_e21348_d_n10;
        locals.var_devtempeff_dn11 = assign14420_e21348_d_n11;
        locals.var_devtempeff_dn13 = assign14420_e21348_d_n13;
        locals.var_devtempeff_dn14 = assign14420_e21348_d_n14;

    }

    pub(super) fn stamp_transient_block_40(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14430_e21377, assign14430_e21377_d_n0, assign14430_e21377_d_n2, assign14430_e21377_d_n3, assign14430_e21377_d_n4, assign14430_e21377_d_n5, assign14430_e21377_d_n6, assign14430_e21377_d_n7, assign14430_e21377_d_n8, assign14430_e21377_d_n9, assign14430_e21377_d_n10, assign14430_e21377_d_n11, assign14430_e21377_d_n13, assign14430_e21377_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 == 0.0)) {
        let assign14430_e21359: f64 = (locals.var_tnom + locals.var_tlow);
        let assign14430_e21362: f64 = (locals.var_tnom - locals.var_tlow);
        let assign14430_e21365: f64 = (locals.var_tnom - locals.var_tlow);
        let assign14430_e21366: f64 = (assign14430_e21362 * assign14430_e21365);
        let assign14430_e21369: f64 = (0.25 * p.p1788);
        let assign14430_e21371: f64 = (assign14430_e21369 * p.p1788);
        let assign14430_e21372: f64 = (assign14430_e21366 + assign14430_e21371);
        let assign14430_e21373: f64 = (assign14430_e21372).sqrt();
        let assign14430_e21374: f64 = (assign14430_e21359 + assign14430_e21373);
        let assign14430_e21375: f64 = (0.5 * assign14430_e21374);
        (assign14430_e21375, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign14430_e21377;
        locals.var_t1_dn0 = assign14430_e21377_d_n0;
        locals.var_t1_dn2 = assign14430_e21377_d_n2;
        locals.var_t1_dn3 = assign14430_e21377_d_n3;
        locals.var_t1_dn4 = assign14430_e21377_d_n4;
        locals.var_t1_dn5 = assign14430_e21377_d_n5;
        locals.var_t1_dn6 = assign14430_e21377_d_n6;
        locals.var_t1_dn7 = assign14430_e21377_d_n7;
        locals.var_t1_dn8 = assign14430_e21377_d_n8;
        locals.var_t1_dn9 = assign14430_e21377_d_n9;
        locals.var_t1_dn10 = assign14430_e21377_d_n10;
        locals.var_t1_dn11 = assign14430_e21377_d_n11;
        locals.var_t1_dn13 = assign14430_e21377_d_n13;
        locals.var_t1_dn14 = assign14430_e21377_d_n14;

        let (assign14440_e21421, assign14440_e21421_d_n0, assign14440_e21421_d_n2, assign14440_e21421_d_n3, assign14440_e21421_d_n4, assign14440_e21421_d_n5, assign14440_e21421_d_n6, assign14440_e21421_d_n7, assign14440_e21421_d_n8, assign14440_e21421_d_n9, assign14440_e21421_d_n10, assign14440_e21421_d_n11, assign14440_e21421_d_n13, assign14440_e21421_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 == 0.0)) {
        let assign14440_e21387: f64 = (-p.p1790);
        let assign14440_e21390: f64 = (locals.var_tnom - p.p1787);
        let assign14440_e21391: f64 = (assign14440_e21387 * assign14440_e21390);
        let assign14440_e21393: f64 = assign14440_e21391;
        let assign14440_e21395: f64 = (-p.p1790);
        let assign14440_e21398: f64 = (locals.var_tnom - p.p1787);
        let assign14440_e21399: f64 = (assign14440_e21395 * assign14440_e21398);
        let assign14440_e21401: f64 = assign14440_e21399;
        let assign14440_e21403: f64 = (-p.p1790);
        let assign14440_e21406: f64 = (locals.var_tnom - p.p1787);
        let assign14440_e21407: f64 = (assign14440_e21403 * assign14440_e21406);
        let assign14440_e21409: f64 = assign14440_e21407;
        let assign14440_e21410: f64 = (assign14440_e21401 * assign14440_e21409);
        let assign14440_e21413: f64 = (0.25 * p.p1789);
        let assign14440_e21415: f64 = (assign14440_e21413 * p.p1789);
        let assign14440_e21416: f64 = (assign14440_e21410 + assign14440_e21415);
        let assign14440_e21417: f64 = (assign14440_e21416).sqrt();
        let assign14440_e21418: f64 = (assign14440_e21393 + assign14440_e21417);
        let assign14440_e21419: f64 = (0.5 * assign14440_e21418);
        (assign14440_e21419, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign14440_e21421;
        locals.var_t2_dn0 = assign14440_e21421_d_n0;
        locals.var_t2_dn2 = assign14440_e21421_d_n2;
        locals.var_t2_dn3 = assign14440_e21421_d_n3;
        locals.var_t2_dn4 = assign14440_e21421_d_n4;
        locals.var_t2_dn5 = assign14440_e21421_d_n5;
        locals.var_t2_dn6 = assign14440_e21421_d_n6;
        locals.var_t2_dn7 = assign14440_e21421_d_n7;
        locals.var_t2_dn8 = assign14440_e21421_d_n8;
        locals.var_t2_dn9 = assign14440_e21421_d_n9;
        locals.var_t2_dn10 = assign14440_e21421_d_n10;
        locals.var_t2_dn11 = assign14440_e21421_d_n11;
        locals.var_t2_dn13 = assign14440_e21421_d_n13;
        locals.var_t2_dn14 = assign14440_e21421_d_n14;

        let assign14450_e21424: f64 = if locals.var_tnom > locals.var_tlow { 1.0 } else { 0.0 };
        locals.var_guard242 = assign14450_e21424;

        let (assign14460_e21444, assign14460_e21444_d_n0, assign14460_e21444_d_n2, assign14460_e21444_d_n3, assign14460_e21444_d_n4, assign14460_e21444_d_n5, assign14460_e21444_d_n6, assign14460_e21444_d_n7, assign14460_e21444_d_n8, assign14460_e21444_d_n9, assign14460_e21444_d_n10, assign14460_e21444_d_n11, assign14460_e21444_d_n13, assign14460_e21444_d_n14,) = {
    if ((((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 == 0.0)) && (locals.var_guard242 != 0.0)) {
        let assign14460_e21436: f64 = (locals.var_devtemplow0 + locals.var_devtemplow1);
        let assign14460_e21438: f64 = (assign14460_e21436 - locals.var_t1);
        let assign14460_e21440: f64 = (assign14460_e21438 - locals.var_t2);
        let assign14460_e21442: f64 = (assign14460_e21440 + locals.var_tnom);
        (assign14460_e21442, ((-locals.var_t1_dn0) - locals.var_t2_dn0), ((-locals.var_t1_dn2) - locals.var_t2_dn2), ((-locals.var_t1_dn3) - locals.var_t2_dn3), (((locals.var_devtemplow0_dn4 + locals.var_devtemplow1_dn4) - locals.var_t1_dn4) - locals.var_t2_dn4), ((-locals.var_t1_dn5) - locals.var_t2_dn5), ((-locals.var_t1_dn6) - locals.var_t2_dn6), ((-locals.var_t1_dn7) - locals.var_t2_dn7), ((-locals.var_t1_dn8) - locals.var_t2_dn8), ((-locals.var_t1_dn9) - locals.var_t2_dn9), ((-locals.var_t1_dn10) - locals.var_t2_dn10), ((-locals.var_t1_dn11) - locals.var_t2_dn11), ((-locals.var_t1_dn13) - locals.var_t2_dn13), ((-locals.var_t1_dn14) - locals.var_t2_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign14460_e21444;
        locals.var_t3_dn0 = assign14460_e21444_d_n0;
        locals.var_t3_dn2 = assign14460_e21444_d_n2;
        locals.var_t3_dn3 = assign14460_e21444_d_n3;
        locals.var_t3_dn4 = assign14460_e21444_d_n4;
        locals.var_t3_dn5 = assign14460_e21444_d_n5;
        locals.var_t3_dn6 = assign14460_e21444_d_n6;
        locals.var_t3_dn7 = assign14460_e21444_d_n7;
        locals.var_t3_dn8 = assign14460_e21444_d_n8;
        locals.var_t3_dn9 = assign14460_e21444_d_n9;
        locals.var_t3_dn10 = assign14460_e21444_d_n10;
        locals.var_t3_dn11 = assign14460_e21444_d_n11;
        locals.var_t3_dn13 = assign14460_e21444_d_n13;
        locals.var_t3_dn14 = assign14460_e21444_d_n14;

        let (assign14470_e21465, assign14470_e21465_d_n0, assign14470_e21465_d_n2, assign14470_e21465_d_n3, assign14470_e21465_d_n4, assign14470_e21465_d_n5, assign14470_e21465_d_n6, assign14470_e21465_d_n7, assign14470_e21465_d_n8, assign14470_e21465_d_n9, assign14470_e21465_d_n10, assign14470_e21465_d_n11, assign14470_e21465_d_n13, assign14470_e21465_d_n14,) = {
    if ((((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 == 0.0)) && (locals.var_guard242 == 0.0)) {
        let assign14470_e21457: f64 = (locals.var_devtemplow0 + locals.var_devtemplow1);
        let assign14470_e21459: f64 = (assign14470_e21457 - locals.var_t1);
        let assign14470_e21461: f64 = (assign14470_e21459 - locals.var_t2);
        let assign14470_e21463: f64 = (assign14470_e21461 + locals.var_tlow);
        (assign14470_e21463, ((-locals.var_t1_dn0) - locals.var_t2_dn0), ((-locals.var_t1_dn2) - locals.var_t2_dn2), ((-locals.var_t1_dn3) - locals.var_t2_dn3), (((locals.var_devtemplow0_dn4 + locals.var_devtemplow1_dn4) - locals.var_t1_dn4) - locals.var_t2_dn4), ((-locals.var_t1_dn5) - locals.var_t2_dn5), ((-locals.var_t1_dn6) - locals.var_t2_dn6), ((-locals.var_t1_dn7) - locals.var_t2_dn7), ((-locals.var_t1_dn8) - locals.var_t2_dn8), ((-locals.var_t1_dn9) - locals.var_t2_dn9), ((-locals.var_t1_dn10) - locals.var_t2_dn10), ((-locals.var_t1_dn11) - locals.var_t2_dn11), ((-locals.var_t1_dn13) - locals.var_t2_dn13), ((-locals.var_t1_dn14) - locals.var_t2_dn14),)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign14470_e21465;
        locals.var_t3_dn0 = assign14470_e21465_d_n0;
        locals.var_t3_dn2 = assign14470_e21465_d_n2;
        locals.var_t3_dn3 = assign14470_e21465_d_n3;
        locals.var_t3_dn4 = assign14470_e21465_d_n4;
        locals.var_t3_dn5 = assign14470_e21465_d_n5;
        locals.var_t3_dn6 = assign14470_e21465_d_n6;
        locals.var_t3_dn7 = assign14470_e21465_d_n7;
        locals.var_t3_dn8 = assign14470_e21465_d_n8;
        locals.var_t3_dn9 = assign14470_e21465_d_n9;
        locals.var_t3_dn10 = assign14470_e21465_d_n10;
        locals.var_t3_dn11 = assign14470_e21465_d_n11;
        locals.var_t3_dn13 = assign14470_e21465_d_n13;
        locals.var_t3_dn14 = assign14470_e21465_d_n14;

        let (assign14480_e21494, assign14480_e21494_d_n0, assign14480_e21494_d_n2, assign14480_e21494_d_n3, assign14480_e21494_d_n4, assign14480_e21494_d_n5, assign14480_e21494_d_n6, assign14480_e21494_d_n7, assign14480_e21494_d_n8, assign14480_e21494_d_n9, assign14480_e21494_d_n10, assign14480_e21494_d_n11, assign14480_e21494_d_n13, assign14480_e21494_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 == 0.0)) {
        let assign14480_e21476: f64 = (locals.var_devtemp + locals.var_t3);
        let assign14480_e21479: f64 = (locals.var_devtemp - locals.var_t3);
        let assign14480_e21482: f64 = (locals.var_devtemp - locals.var_t3);
        let assign14480_e21483: f64 = (assign14480_e21479 * assign14480_e21482);
        let assign14480_e21486: f64 = (0.25 * 0.2);
        let assign14480_e21488: f64 = (assign14480_e21486 * 0.2);
        let assign14480_e21489: f64 = (assign14480_e21483 + assign14480_e21488);
        let assign14480_e21490: f64 = (assign14480_e21489).sqrt();
        let assign14480_e21491: f64 = (assign14480_e21476 + assign14480_e21490);
        let assign14480_e21492: f64 = (0.5 * assign14480_e21491);
        (assign14480_e21492, (0.5 * (locals.var_t3_dn0 + ((((-locals.var_t3_dn0) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn0))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn2 + ((((-locals.var_t3_dn2) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn2))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn3 + ((((-locals.var_t3_dn3) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn3))) / (2.0 * assign14480_e21490)))), (0.5 * ((locals.var_devtemp_dn4 + locals.var_t3_dn4) + ((((locals.var_devtemp_dn4 - locals.var_t3_dn4) * assign14480_e21482) + (assign14480_e21479 * (locals.var_devtemp_dn4 - locals.var_t3_dn4))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn5 + ((((-locals.var_t3_dn5) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn5))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn6 + ((((-locals.var_t3_dn6) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn6))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn7 + ((((-locals.var_t3_dn7) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn7))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn8 + ((((-locals.var_t3_dn8) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn8))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn9 + ((((-locals.var_t3_dn9) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn9))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn10 + ((((-locals.var_t3_dn10) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn10))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn11 + ((((-locals.var_t3_dn11) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn11))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn13 + ((((-locals.var_t3_dn13) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn13))) / (2.0 * assign14480_e21490)))), (0.5 * (locals.var_t3_dn14 + ((((-locals.var_t3_dn14) * assign14480_e21482) + (assign14480_e21479 * (-locals.var_t3_dn14))) / (2.0 * assign14480_e21490)))),)
    } else {
        (locals.var_t4, locals.var_t4_dn0, locals.var_t4_dn2, locals.var_t4_dn3, locals.var_t4_dn4, locals.var_t4_dn5, locals.var_t4_dn6, locals.var_t4_dn7, locals.var_t4_dn8, locals.var_t4_dn9, locals.var_t4_dn10, locals.var_t4_dn11, locals.var_t4_dn13, locals.var_t4_dn14,)
    }
};
        locals.var_t4 = assign14480_e21494;
        locals.var_t4_dn0 = assign14480_e21494_d_n0;
        locals.var_t4_dn2 = assign14480_e21494_d_n2;
        locals.var_t4_dn3 = assign14480_e21494_d_n3;
        locals.var_t4_dn4 = assign14480_e21494_d_n4;
        locals.var_t4_dn5 = assign14480_e21494_d_n5;
        locals.var_t4_dn6 = assign14480_e21494_d_n6;
        locals.var_t4_dn7 = assign14480_e21494_d_n7;
        locals.var_t4_dn8 = assign14480_e21494_d_n8;
        locals.var_t4_dn9 = assign14480_e21494_d_n9;
        locals.var_t4_dn10 = assign14480_e21494_d_n10;
        locals.var_t4_dn11 = assign14480_e21494_d_n11;
        locals.var_t4_dn13 = assign14480_e21494_d_n13;
        locals.var_t4_dn14 = assign14480_e21494_d_n14;

        let (assign14490_e21510, assign14490_e21510_d_n0, assign14490_e21510_d_n2, assign14490_e21510_d_n3, assign14490_e21510_d_n4, assign14490_e21510_d_n5, assign14490_e21510_d_n6, assign14490_e21510_d_n7, assign14490_e21510_d_n8, assign14490_e21510_d_n9, assign14490_e21510_d_n10, assign14490_e21510_d_n11, assign14490_e21510_d_n13, assign14490_e21510_d_n14,) = {
    if (((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) && (locals.var_guard241 == 0.0)) {
        let assign14490_e21504: f64 = (locals.var_wl * locals.var_t4);
        let assign14490_e21507: f64 = (locals.var_wh * locals.var_devtemp);
        let assign14490_e21508: f64 = (assign14490_e21504 + assign14490_e21507);
        (assign14490_e21508, (locals.var_wl * locals.var_t4_dn0), (locals.var_wl * locals.var_t4_dn2), (locals.var_wl * locals.var_t4_dn3), (((locals.var_wl_dn4 * locals.var_t4) + (locals.var_wl * locals.var_t4_dn4)) + ((locals.var_wh_dn4 * locals.var_devtemp) + (locals.var_wh * locals.var_devtemp_dn4))), (locals.var_wl * locals.var_t4_dn5), (locals.var_wl * locals.var_t4_dn6), (locals.var_wl * locals.var_t4_dn7), (locals.var_wl * locals.var_t4_dn8), (locals.var_wl * locals.var_t4_dn9), (locals.var_wl * locals.var_t4_dn10), (locals.var_wl * locals.var_t4_dn11), (locals.var_wl * locals.var_t4_dn13), (locals.var_wl * locals.var_t4_dn14),)
    } else {
        (locals.var_devtempeff, locals.var_devtempeff_dn0, locals.var_devtempeff_dn2, locals.var_devtempeff_dn3, locals.var_devtempeff_dn4, locals.var_devtempeff_dn5, locals.var_devtempeff_dn6, locals.var_devtempeff_dn7, locals.var_devtempeff_dn8, locals.var_devtempeff_dn9, locals.var_devtempeff_dn10, locals.var_devtempeff_dn11, locals.var_devtempeff_dn13, locals.var_devtempeff_dn14,)
    }
};
        locals.var_devtempeff = assign14490_e21510;
        locals.var_devtempeff_dn0 = assign14490_e21510_d_n0;
        locals.var_devtempeff_dn2 = assign14490_e21510_d_n2;
        locals.var_devtempeff_dn3 = assign14490_e21510_d_n3;
        locals.var_devtempeff_dn4 = assign14490_e21510_d_n4;
        locals.var_devtempeff_dn5 = assign14490_e21510_d_n5;
        locals.var_devtempeff_dn6 = assign14490_e21510_d_n6;
        locals.var_devtempeff_dn7 = assign14490_e21510_d_n7;
        locals.var_devtempeff_dn8 = assign14490_e21510_d_n8;
        locals.var_devtempeff_dn9 = assign14490_e21510_d_n9;
        locals.var_devtempeff_dn10 = assign14490_e21510_d_n10;
        locals.var_devtempeff_dn11 = assign14490_e21510_d_n11;
        locals.var_devtempeff_dn13 = assign14490_e21510_d_n13;
        locals.var_devtempeff_dn14 = assign14490_e21510_d_n14;

        let (assign14500_e21536, assign14500_e21536_d_n4,) = {
    if ((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) {
        let assign14500_e21518: f64 = (locals.var_devtemp + 210.0);
        let assign14500_e21521: f64 = (locals.var_devtemp - 210.0);
        let assign14500_e21524: f64 = (locals.var_devtemp - 210.0);
        let assign14500_e21525: f64 = (assign14500_e21521 * assign14500_e21524);
        let assign14500_e21528: f64 = (0.25 * 0.2);
        let assign14500_e21530: f64 = (assign14500_e21528 * 0.2);
        let assign14500_e21531: f64 = (assign14500_e21525 + assign14500_e21530);
        let assign14500_e21532: f64 = (assign14500_e21531).sqrt();
        let assign14500_e21533: f64 = (assign14500_e21518 - assign14500_e21532);
        let assign14500_e21534: f64 = (0.5 * assign14500_e21533);
        (assign14500_e21534, (0.5 * (locals.var_devtemp_dn4 - (((locals.var_devtemp_dn4 * assign14500_e21524) + (assign14500_e21521 * locals.var_devtemp_dn4)) / (2.0 * assign14500_e21532)))),)
    } else {
        (locals.var_devtemp1, locals.var_devtemp1_dn4,)
    }
};
        locals.var_devtemp1 = assign14500_e21536;
        locals.var_devtemp1_dn4 = assign14500_e21536_d_n4;

        let (assign14510_e21564, assign14510_e21564_d_n4,) = {
    if ((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) {
        let assign14510_e21545: f64 = (locals.var_tnom + 210.0);
        let assign14510_e21548: f64 = (locals.var_tnom - 210.0);
        let assign14510_e21551: f64 = (locals.var_tnom - 210.0);
        let assign14510_e21552: f64 = (assign14510_e21548 * assign14510_e21551);
        let assign14510_e21555: f64 = (0.25 * 0.2);
        let assign14510_e21557: f64 = (assign14510_e21555 * 0.2);
        let assign14510_e21558: f64 = (assign14510_e21552 + assign14510_e21557);
        let assign14510_e21559: f64 = (assign14510_e21558).sqrt();
        let assign14510_e21560: f64 = (assign14510_e21545 - assign14510_e21559);
        let assign14510_e21561: f64 = (0.5 * assign14510_e21560);
        let assign14510_e21562: f64 = (locals.var_devtemp1 - assign14510_e21561);
        (assign14510_e21562, locals.var_devtemp1_dn4,)
    } else {
        (locals.var_deltemp1, locals.var_deltemp1_dn4,)
    }
};
        locals.var_deltemp1 = assign14510_e21564;
        locals.var_deltemp1_dn4 = assign14510_e21564_d_n4;

        let (assign14520_e21575, assign14520_e21575_d_n4,) = {
    if ((locals.var_guard237 != 0.0) && (locals.var_guard238 == 0.0)) {
        let assign14520_e21571: f64 = (locals.var_devtemp1 - 210.0);
        let assign14520_e21573: f64 = (assign14520_e21571 / locals.var_tnom);
        (assign14520_e21573, (locals.var_devtemp1_dn4 / locals.var_tnom),)
    } else {
        (locals.var_deltratio1, locals.var_deltratio1_dn4,)
    }
};
        locals.var_deltratio1 = assign14520_e21575;
        locals.var_deltratio1_dn4 = assign14520_e21575_d_n4;

        let (assign14530_e21581, assign14530_e21581_d_n0, assign14530_e21581_d_n2, assign14530_e21581_d_n3, assign14530_e21581_d_n4, assign14530_e21581_d_n5, assign14530_e21581_d_n6, assign14530_e21581_d_n7, assign14530_e21581_d_n8, assign14530_e21581_d_n9, assign14530_e21581_d_n10, assign14530_e21581_d_n11, assign14530_e21581_d_n13, assign14530_e21581_d_n14,) = {
    if (locals.var_guard237 != 0.0) {
        let assign14530_e21579: f64 = (8.617087e-5 * locals.var_devtempeff);
        (assign14530_e21579, (8.617087e-5 * locals.var_devtempeff_dn0), (8.617087e-5 * locals.var_devtempeff_dn2), (8.617087e-5 * locals.var_devtempeff_dn3), (8.617087e-5 * locals.var_devtempeff_dn4), (8.617087e-5 * locals.var_devtempeff_dn5), (8.617087e-5 * locals.var_devtempeff_dn6), (8.617087e-5 * locals.var_devtempeff_dn7), (8.617087e-5 * locals.var_devtempeff_dn8), (8.617087e-5 * locals.var_devtempeff_dn9), (8.617087e-5 * locals.var_devtempeff_dn10), (8.617087e-5 * locals.var_devtempeff_dn11), (8.617087e-5 * locals.var_devtempeff_dn13), (8.617087e-5 * locals.var_devtempeff_dn14),)
    } else {
        (locals.var_vtmeff, locals.var_vtmeff_dn0, locals.var_vtmeff_dn2, locals.var_vtmeff_dn3, locals.var_vtmeff_dn4, locals.var_vtmeff_dn5, locals.var_vtmeff_dn6, locals.var_vtmeff_dn7, locals.var_vtmeff_dn8, locals.var_vtmeff_dn9, locals.var_vtmeff_dn10, locals.var_vtmeff_dn11, locals.var_vtmeff_dn13, locals.var_vtmeff_dn14,)
    }
};
        locals.var_vtmeff = assign14530_e21581;
        locals.var_vtmeff_dn0 = assign14530_e21581_d_n0;
        locals.var_vtmeff_dn2 = assign14530_e21581_d_n2;
        locals.var_vtmeff_dn3 = assign14530_e21581_d_n3;
        locals.var_vtmeff_dn4 = assign14530_e21581_d_n4;
        locals.var_vtmeff_dn5 = assign14530_e21581_d_n5;
        locals.var_vtmeff_dn6 = assign14530_e21581_d_n6;
        locals.var_vtmeff_dn7 = assign14530_e21581_d_n7;
        locals.var_vtmeff_dn8 = assign14530_e21581_d_n8;
        locals.var_vtmeff_dn9 = assign14530_e21581_d_n9;
        locals.var_vtmeff_dn10 = assign14530_e21581_d_n10;
        locals.var_vtmeff_dn11 = assign14530_e21581_d_n11;
        locals.var_vtmeff_dn13 = assign14530_e21581_d_n13;
        locals.var_vtmeff_dn14 = assign14530_e21581_d_n14;

        let assign14540_e21585: f64 = (p.p1718 * locals.var_devtemp);
        let assign14540_e21587: f64 = (assign14540_e21585 * locals.var_devtemp);
        let assign14540_e21590: f64 = (locals.var_devtemp + p.p1719);
        let assign14540_e21591: f64 = (assign14540_e21587 / assign14540_e21590);
        let assign14540_e21592: f64 = (p.p106 - assign14540_e21591);
        locals.var_eg = assign14540_e21592;
        locals.var_eg_dn4 = (-((((((p.p1718 * locals.var_devtemp_dn4) * locals.var_devtemp) + (assign14540_e21585 * locals.var_devtemp_dn4)) * assign14540_e21590) - (assign14540_e21587 * locals.var_devtemp_dn4)) / (assign14540_e21590 * assign14540_e21590)));

        let assign14550_e21596: f64 = (p.p1718 * locals.var_tnom);
        let assign14550_e21598: f64 = (assign14550_e21596 * locals.var_tnom);
        let assign14550_e21601: f64 = (locals.var_tnom + p.p1719);
        let assign14550_e21602: f64 = (assign14550_e21598 / assign14550_e21601);
        let assign14550_e21603: f64 = (p.p106 - assign14550_e21602);
        locals.var_eg0 = assign14550_e21603;

        let __rspice_inv_cse_0: f64 = 1.0 / 300.15;
        let assign14560_e21606: f64 = (locals.var_devtemp * __rspice_inv_cse_0);
        let assign14560_e21609: f64 = (locals.var_devtemp * __rspice_inv_cse_0);
        let assign14560_e21610: f64 = (assign14560_e21609).sqrt();
        let assign14560_e21611: f64 = (assign14560_e21606 * assign14560_e21610);
        locals.var_t1 = assign14560_e21611;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = (((locals.var_devtemp_dn4 / 300.15) * assign14560_e21610) + (assign14560_e21606 * ((locals.var_devtemp_dn4 / 300.15) / (2.0 * assign14560_e21610))));
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;

        let assign14570_e21614: f64 = (p.p105 * locals.var_t1);
        let assign14570_e21618: f64 = (2.0 * 8.617087e-5);
        let assign14570_e21620: f64 = (assign14570_e21618 * 300.15);
        let assign14570_e21621: f64 = (p.p106 / assign14570_e21620);
        let assign14570_e21625: f64 = (2.0 * locals.var_vtm);
        let assign14570_e21626: f64 = (locals.var_eg / assign14570_e21625);
        let assign14570_e21627: f64 = (assign14570_e21621 - assign14570_e21626);
        let assign14570_e21628: f64 = { let limited_exp_arg = assign14570_e21627; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign14570_e21629: f64 = (assign14570_e21614 * assign14570_e21628);
        locals.var_ni = assign14570_e21629;
        locals.var_ni_dn0 = ((p.p105 * locals.var_t1_dn0) * assign14570_e21628);
        locals.var_ni_dn2 = ((p.p105 * locals.var_t1_dn2) * assign14570_e21628);
        locals.var_ni_dn3 = ((p.p105 * locals.var_t1_dn3) * assign14570_e21628);
        locals.var_ni_dn4 = (((p.p105 * locals.var_t1_dn4) * assign14570_e21628) + (assign14570_e21614 * ({ let limited_exp_arg = assign14570_e21627; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (-(((locals.var_eg_dn4 * assign14570_e21625) - (locals.var_eg * (2.0 * locals.var_vtm_dn4))) / (assign14570_e21625 * assign14570_e21625))))));
        locals.var_ni_dn5 = ((p.p105 * locals.var_t1_dn5) * assign14570_e21628);
        locals.var_ni_dn6 = ((p.p105 * locals.var_t1_dn6) * assign14570_e21628);
        locals.var_ni_dn7 = ((p.p105 * locals.var_t1_dn7) * assign14570_e21628);
        locals.var_ni_dn8 = ((p.p105 * locals.var_t1_dn8) * assign14570_e21628);
        locals.var_ni_dn9 = ((p.p105 * locals.var_t1_dn9) * assign14570_e21628);
        locals.var_ni_dn10 = ((p.p105 * locals.var_t1_dn10) * assign14570_e21628);
        locals.var_ni_dn11 = ((p.p105 * locals.var_t1_dn11) * assign14570_e21628);
        locals.var_ni_dn13 = ((p.p105 * locals.var_t1_dn13) * assign14570_e21628);
        locals.var_ni_dn14 = ((p.p105 * locals.var_t1_dn14) * assign14570_e21628);

        let assign14580_e21632: f64 = if p.p80 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard243 = assign14580_e21632;

        let (assign14590_e21638, assign14590_e21638_d_n0, assign14590_e21638_d_n2, assign14590_e21638_d_n3, assign14590_e21638_d_n4, assign14590_e21638_d_n5, assign14590_e21638_d_n6, assign14590_e21638_d_n7, assign14590_e21638_d_n8, assign14590_e21638_d_n9, assign14590_e21638_d_n10, assign14590_e21638_d_n11, assign14590_e21638_d_n13, assign14590_e21638_d_n14,) = {
    if (locals.var_guard243 != 0.0) {
        let assign14590_e21636: f64 = (p.p107 * locals.var_t1);
        (assign14590_e21636, (p.p107 * locals.var_t1_dn0), (p.p107 * locals.var_t1_dn2), (p.p107 * locals.var_t1_dn3), (p.p107 * locals.var_t1_dn4), (p.p107 * locals.var_t1_dn5), (p.p107 * locals.var_t1_dn6), (p.p107 * locals.var_t1_dn7), (p.p107 * locals.var_t1_dn8), (p.p107 * locals.var_t1_dn9), (p.p107 * locals.var_t1_dn10), (p.p107 * locals.var_t1_dn11), (p.p107 * locals.var_t1_dn13), (p.p107 * locals.var_t1_dn14),)
    } else {
        (locals.var_nc, locals.var_nc_dn0, locals.var_nc_dn2, locals.var_nc_dn3, locals.var_nc_dn4, locals.var_nc_dn5, locals.var_nc_dn6, locals.var_nc_dn7, locals.var_nc_dn8, locals.var_nc_dn9, locals.var_nc_dn10, locals.var_nc_dn11, locals.var_nc_dn13, locals.var_nc_dn14,)
    }
};
        locals.var_nc = assign14590_e21638;
        locals.var_nc_dn0 = assign14590_e21638_d_n0;
        locals.var_nc_dn2 = assign14590_e21638_d_n2;
        locals.var_nc_dn3 = assign14590_e21638_d_n3;
        locals.var_nc_dn4 = assign14590_e21638_d_n4;
        locals.var_nc_dn5 = assign14590_e21638_d_n5;
        locals.var_nc_dn6 = assign14590_e21638_d_n6;
        locals.var_nc_dn7 = assign14590_e21638_d_n7;
        locals.var_nc_dn8 = assign14590_e21638_d_n8;
        locals.var_nc_dn9 = assign14590_e21638_d_n9;
        locals.var_nc_dn10 = assign14590_e21638_d_n10;
        locals.var_nc_dn11 = assign14590_e21638_d_n11;
        locals.var_nc_dn13 = assign14590_e21638_d_n13;
        locals.var_nc_dn14 = assign14590_e21638_d_n14;

        let (assign14600_e21652, assign14600_e21652_d_n0, assign14600_e21652_d_n2, assign14600_e21652_d_n3, assign14600_e21652_d_n4, assign14600_e21652_d_n5, assign14600_e21652_d_n6, assign14600_e21652_d_n7, assign14600_e21652_d_n8, assign14600_e21652_d_n9, assign14600_e21652_d_n10, assign14600_e21652_d_n11, assign14600_e21652_d_n13, assign14600_e21652_d_n14,) = {
    if (locals.var_guard243 == 0.0) {
        let assign14600_e21644: f64 = (locals.var_devtempeff / 300.15);
        let assign14600_e21645: f64 = (p.p107 * assign14600_e21644);
        let assign14600_e21648: f64 = (locals.var_devtempeff / 300.15);
        let assign14600_e21649: f64 = (assign14600_e21648).sqrt();
        let assign14600_e21650: f64 = (assign14600_e21645 * assign14600_e21649);
        (assign14600_e21650, (((p.p107 * (locals.var_devtempeff_dn0 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn0 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn2 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn2 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn3 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn3 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn4 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn4 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn5 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn5 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn6 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn6 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn7 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn7 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn8 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn8 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn9 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn9 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn10 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn10 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn11 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn11 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn13 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn13 / 300.15) / (2.0 * assign14600_e21649)))), (((p.p107 * (locals.var_devtempeff_dn14 / 300.15)) * assign14600_e21649) + (assign14600_e21645 * ((locals.var_devtempeff_dn14 / 300.15) / (2.0 * assign14600_e21649)))),)
    } else {
        (locals.var_nc, locals.var_nc_dn0, locals.var_nc_dn2, locals.var_nc_dn3, locals.var_nc_dn4, locals.var_nc_dn5, locals.var_nc_dn6, locals.var_nc_dn7, locals.var_nc_dn8, locals.var_nc_dn9, locals.var_nc_dn10, locals.var_nc_dn11, locals.var_nc_dn13, locals.var_nc_dn14,)
    }
};
        locals.var_nc = assign14600_e21652;
        locals.var_nc_dn0 = assign14600_e21652_d_n0;
        locals.var_nc_dn2 = assign14600_e21652_d_n2;
        locals.var_nc_dn3 = assign14600_e21652_d_n3;
        locals.var_nc_dn4 = assign14600_e21652_d_n4;
        locals.var_nc_dn5 = assign14600_e21652_d_n5;
        locals.var_nc_dn6 = assign14600_e21652_d_n6;
        locals.var_nc_dn7 = assign14600_e21652_d_n7;
        locals.var_nc_dn8 = assign14600_e21652_d_n8;
        locals.var_nc_dn9 = assign14600_e21652_d_n9;
        locals.var_nc_dn10 = assign14600_e21652_d_n10;
        locals.var_nc_dn11 = assign14600_e21652_d_n11;
        locals.var_nc_dn13 = assign14600_e21652_d_n13;
        locals.var_nc_dn14 = assign14600_e21652_d_n14;

        let (assign14610_e21690, assign14610_e21690_d_n0, assign14610_e21690_d_n2, assign14610_e21690_d_n3, assign14610_e21690_d_n4, assign14610_e21690_d_n5, assign14610_e21690_d_n6, assign14610_e21690_d_n7, assign14610_e21690_d_n8, assign14610_e21690_d_n9, assign14610_e21690_d_n10, assign14610_e21690_d_n11, assign14610_e21690_d_n13, assign14610_e21690_d_n14,) = {
    if (locals.var_guard243 == 0.0) {
        let assign14610_e21657: f64 = (p.p105 * locals.var_t1);
        let (assign14610_e21674, assign14610_e21674_d_n0, assign14610_e21674_d_n2, assign14610_e21674_d_n3, assign14610_e21674_d_n4, assign14610_e21674_d_n5, assign14610_e21674_d_n6, assign14610_e21674_d_n7, assign14610_e21674_d_n8, assign14610_e21674_d_n9, assign14610_e21674_d_n10, assign14610_e21674_d_n11, assign14610_e21674_d_n13, assign14610_e21674_d_n14,) = {
            if (!(assign14610_e21657 > 1e-38)) {
                let assign14610_e21662: f64 = (-87.498233534);
                (assign14610_e21662, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            } else {
                let assign14610_e21665: f64 = (p.p105 * locals.var_t1);
                let (assign14610_e21673, assign14610_e21673_d_n0, assign14610_e21673_d_n2, assign14610_e21673_d_n3, assign14610_e21673_d_n4, assign14610_e21673_d_n5, assign14610_e21673_d_n6, assign14610_e21673_d_n7, assign14610_e21673_d_n8, assign14610_e21673_d_n9, assign14610_e21673_d_n10, assign14610_e21673_d_n11, assign14610_e21673_d_n13, assign14610_e21673_d_n14,) = {
                    if (assign14610_e21665 > 1e-38) {
                        let assign14610_e21670: f64 = (p.p105 * locals.var_t1);
                        let assign14610_e21671: f64 = (assign14610_e21670).ln();
                        (assign14610_e21671, ((p.p105 * locals.var_t1_dn0) / assign14610_e21670), ((p.p105 * locals.var_t1_dn2) / assign14610_e21670), ((p.p105 * locals.var_t1_dn3) / assign14610_e21670), ((p.p105 * locals.var_t1_dn4) / assign14610_e21670), ((p.p105 * locals.var_t1_dn5) / assign14610_e21670), ((p.p105 * locals.var_t1_dn6) / assign14610_e21670), ((p.p105 * locals.var_t1_dn7) / assign14610_e21670), ((p.p105 * locals.var_t1_dn8) / assign14610_e21670), ((p.p105 * locals.var_t1_dn9) / assign14610_e21670), ((p.p105 * locals.var_t1_dn10) / assign14610_e21670), ((p.p105 * locals.var_t1_dn11) / assign14610_e21670), ((p.p105 * locals.var_t1_dn13) / assign14610_e21670), ((p.p105 * locals.var_t1_dn14) / assign14610_e21670),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign14610_e21673, assign14610_e21673_d_n0, assign14610_e21673_d_n2, assign14610_e21673_d_n3, assign14610_e21673_d_n4, assign14610_e21673_d_n5, assign14610_e21673_d_n6, assign14610_e21673_d_n7, assign14610_e21673_d_n8, assign14610_e21673_d_n9, assign14610_e21673_d_n10, assign14610_e21673_d_n11, assign14610_e21673_d_n13, assign14610_e21673_d_n14,)
            }
        };
        let assign14610_e21678: f64 = (2.0 * 8.617087e-5);
        let assign14610_e21680: f64 = (assign14610_e21678 * 300.15);
        let assign14610_e21681: f64 = (p.p106 / assign14610_e21680);
        let assign14610_e21682: f64 = (assign14610_e21674 + assign14610_e21681);
        let assign14610_e21686: f64 = (2.0 * locals.var_vtm);
        let assign14610_e21687: f64 = (locals.var_eg / assign14610_e21686);
        let assign14610_e21688: f64 = (assign14610_e21682 - assign14610_e21687);
        (assign14610_e21688, assign14610_e21674_d_n0, assign14610_e21674_d_n2, assign14610_e21674_d_n3, (assign14610_e21674_d_n4 - (((locals.var_eg_dn4 * assign14610_e21686) - (locals.var_eg * (2.0 * locals.var_vtm_dn4))) / (assign14610_e21686 * assign14610_e21686))), assign14610_e21674_d_n5, assign14610_e21674_d_n6, assign14610_e21674_d_n7, assign14610_e21674_d_n8, assign14610_e21674_d_n9, assign14610_e21674_d_n10, assign14610_e21674_d_n11, assign14610_e21674_d_n13, assign14610_e21674_d_n14,)
    } else {
        (locals.var_niln, locals.var_niln_dn0, locals.var_niln_dn2, locals.var_niln_dn3, locals.var_niln_dn4, locals.var_niln_dn5, locals.var_niln_dn6, locals.var_niln_dn7, locals.var_niln_dn8, locals.var_niln_dn9, locals.var_niln_dn10, locals.var_niln_dn11, locals.var_niln_dn13, locals.var_niln_dn14,)
    }
};
        locals.var_niln = assign14610_e21690;
        locals.var_niln_dn0 = assign14610_e21690_d_n0;
        locals.var_niln_dn2 = assign14610_e21690_d_n2;
        locals.var_niln_dn3 = assign14610_e21690_d_n3;
        locals.var_niln_dn4 = assign14610_e21690_d_n4;
        locals.var_niln_dn5 = assign14610_e21690_d_n5;
        locals.var_niln_dn6 = assign14610_e21690_d_n6;
        locals.var_niln_dn7 = assign14610_e21690_d_n7;
        locals.var_niln_dn8 = assign14610_e21690_d_n8;
        locals.var_niln_dn9 = assign14610_e21690_d_n9;
        locals.var_niln_dn10 = assign14610_e21690_d_n10;
        locals.var_niln_dn11 = assign14610_e21690_d_n11;
        locals.var_niln_dn13 = assign14610_e21690_d_n13;
        locals.var_niln_dn14 = assign14610_e21690_d_n14;

        let assign14620_e21694: f64 = (locals.var_tss_i * locals.var_deltemp);
        let assign14620_e21695: f64 = (1.0 + assign14620_e21694);
        let assign14620_e21697: f64 = (assign14620_e21695 - 1e-6);
        let assign14620_e21699: f64 = (-10000.0);
        let assign14620_e21701: f64 = (assign14620_e21699 * 0.001);
        let (assign14620_e21762, assign14620_e21762_d_n4,) = {
    if (!(assign14620_e21697 < assign14620_e21701)) {
        let assign14620_e21708: f64 = (locals.var_tss_i * locals.var_deltemp);
        let assign14620_e21709: f64 = (1.0 + assign14620_e21708);
        let assign14620_e21711: f64 = (assign14620_e21709 - 1e-6);
        let assign14620_e21715: f64 = (locals.var_tss_i * locals.var_deltemp);
        let assign14620_e21716: f64 = (1.0 + assign14620_e21715);
        let assign14620_e21718: f64 = (assign14620_e21716 - 1e-6);
        let assign14620_e21722: f64 = (locals.var_tss_i * locals.var_deltemp);
        let assign14620_e21723: f64 = (1.0 + assign14620_e21722);
        let assign14620_e21725: f64 = (assign14620_e21723 - 1e-6);
        let assign14620_e21726: f64 = (assign14620_e21718 * assign14620_e21725);
        let assign14620_e21729: f64 = (4.0 * 0.001);
        let assign14620_e21731: f64 = (assign14620_e21729 * 0.001);
        let assign14620_e21732: f64 = (assign14620_e21726 + assign14620_e21731);
        let assign14620_e21733: f64 = (assign14620_e21732).sqrt();
        let assign14620_e21734: f64 = (assign14620_e21711 + assign14620_e21733);
        let assign14620_e21735: f64 = (0.5 * assign14620_e21734);
        (assign14620_e21735, (0.5 * ((locals.var_tss_i * locals.var_deltemp_dn4) + ((((locals.var_tss_i * locals.var_deltemp_dn4) * assign14620_e21725) + (assign14620_e21718 * (locals.var_tss_i * locals.var_deltemp_dn4))) / (2.0 * assign14620_e21733)))),)
    } else {
        let assign14620_e21739: f64 = (locals.var_tss_i * locals.var_deltemp);
        let assign14620_e21740: f64 = (1.0 + assign14620_e21739);
        let assign14620_e21742: f64 = (assign14620_e21740 - 1e-6);
        let assign14620_e21744: f64 = (-10000.0);
        let assign14620_e21746: f64 = (assign14620_e21744 * 0.001);
        let (assign14620_e21761, assign14620_e21761_d_n4,) = {
            if (assign14620_e21742 < assign14620_e21746) {
                let assign14620_e21749: f64 = (-0.001);
                let assign14620_e21751: f64 = (assign14620_e21749 * 0.001);
                let assign14620_e21755: f64 = (locals.var_tss_i * locals.var_deltemp);
                let assign14620_e21756: f64 = (1.0 + assign14620_e21755);
                let assign14620_e21758: f64 = (assign14620_e21756 - 1e-6);
                let assign14620_e21759: f64 = (assign14620_e21751 / assign14620_e21758);
                (assign14620_e21759, (-((assign14620_e21751 * (locals.var_tss_i * locals.var_deltemp_dn4)) / (assign14620_e21758 * assign14620_e21758))),)
            } else {
                (0.0, 0.0,)
            }
        };
        (assign14620_e21761, assign14620_e21761_d_n4,)
    }
};
        locals.var_thetass = assign14620_e21762;
        locals.var_thetass_dn4 = assign14620_e21762_d_n4;

        let assign14630_e21765: f64 = (locals.var_vtm * 1.60219e-19);
        locals.var_kt = assign14630_e21765;
        locals.var_kt_dn4 = (locals.var_vtm_dn4 * 1.60219e-19);

        let assign14640_e21768: f64 = (1.05457e-34 * 3.141592653589793);
        let assign14640_e21771: f64 = (2.0 * locals.var_ach);
        let assign14640_e21773: f64 = (assign14640_e21771 / locals.var_weff_ufcm);
        let assign14640_e21774: f64 = (assign14640_e21768 / assign14640_e21773);
        locals.var_t0 = assign14640_e21774;
        locals.var_t0_dn0 = 0.0;
        locals.var_t0_dn2 = 0.0;
        locals.var_t0_dn3 = 0.0;
        locals.var_t0_dn4 = 0.0;
        locals.var_t0_dn5 = 0.0;
        locals.var_t0_dn6 = 0.0;
        locals.var_t0_dn7 = 0.0;
        locals.var_t0_dn8 = 0.0;
        locals.var_t0_dn9 = 0.0;
        locals.var_t0_dn10 = 0.0;
        locals.var_t0_dn11 = 0.0;
        locals.var_t0_dn13 = 0.0;
        locals.var_t0_dn14 = 0.0;

        let assign14650_e21777: f64 = (locals.var_t0 * locals.var_t0);
        let assign14650_e21780: f64 = (2.0 * locals.var_mx);
        let assign14650_e21781: f64 = (assign14650_e21777 / assign14650_e21780);
        locals.var_e0_1 = assign14650_e21781;
        locals.var_e0_1_dn0 = (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / assign14650_e21780);
        locals.var_e0_1_dn2 = (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / assign14650_e21780);
        locals.var_e0_1_dn3 = (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / assign14650_e21780);
        locals.var_e0_1_dn4 = (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / assign14650_e21780);
        locals.var_e0_1_dn5 = (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / assign14650_e21780);
        locals.var_e0_1_dn6 = (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / assign14650_e21780);
        locals.var_e0_1_dn7 = (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / assign14650_e21780);
        locals.var_e0_1_dn8 = (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / assign14650_e21780);
        locals.var_e0_1_dn9 = (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / assign14650_e21780);
        locals.var_e0_1_dn10 = (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / assign14650_e21780);
        locals.var_e0_1_dn11 = (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / assign14650_e21780);
        locals.var_e0_1_dn13 = (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / assign14650_e21780);
        locals.var_e0_1_dn14 = (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / assign14650_e21780);

    }

    pub(super) fn stamp_transient_block_41(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let assign14660_e21784: f64 = (locals.var_t0 * locals.var_t0);
        let assign14660_e21787: f64 = (2.0 * locals.var_mxprime);
        let assign14660_e21788: f64 = (assign14660_e21784 / assign14660_e21787);
        locals.var_e0prime = assign14660_e21788;
        locals.var_e0prime_dn0 = (((locals.var_t0_dn0 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn0)) / assign14660_e21787);
        locals.var_e0prime_dn2 = (((locals.var_t0_dn2 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn2)) / assign14660_e21787);
        locals.var_e0prime_dn3 = (((locals.var_t0_dn3 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn3)) / assign14660_e21787);
        locals.var_e0prime_dn4 = (((locals.var_t0_dn4 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn4)) / assign14660_e21787);
        locals.var_e0prime_dn5 = (((locals.var_t0_dn5 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn5)) / assign14660_e21787);
        locals.var_e0prime_dn6 = (((locals.var_t0_dn6 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn6)) / assign14660_e21787);
        locals.var_e0prime_dn7 = (((locals.var_t0_dn7 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn7)) / assign14660_e21787);
        locals.var_e0prime_dn8 = (((locals.var_t0_dn8 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn8)) / assign14660_e21787);
        locals.var_e0prime_dn9 = (((locals.var_t0_dn9 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn9)) / assign14660_e21787);
        locals.var_e0prime_dn10 = (((locals.var_t0_dn10 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn10)) / assign14660_e21787);
        locals.var_e0prime_dn11 = (((locals.var_t0_dn11 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn11)) / assign14660_e21787);
        locals.var_e0prime_dn13 = (((locals.var_t0_dn13 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn13)) / assign14660_e21787);
        locals.var_e0prime_dn14 = (((locals.var_t0_dn14 * locals.var_t0) + (locals.var_t0 * locals.var_t0_dn14)) / assign14660_e21787);

        let assign14670_e21791: f64 = (4.0 * locals.var_e0_1);
        locals.var_e1_1 = assign14670_e21791;
        locals.var_e1_1_dn0 = (4.0 * locals.var_e0_1_dn0);
        locals.var_e1_1_dn2 = (4.0 * locals.var_e0_1_dn2);
        locals.var_e1_1_dn3 = (4.0 * locals.var_e0_1_dn3);
        locals.var_e1_1_dn4 = (4.0 * locals.var_e0_1_dn4);
        locals.var_e1_1_dn5 = (4.0 * locals.var_e0_1_dn5);
        locals.var_e1_1_dn6 = (4.0 * locals.var_e0_1_dn6);
        locals.var_e1_1_dn7 = (4.0 * locals.var_e0_1_dn7);
        locals.var_e1_1_dn8 = (4.0 * locals.var_e0_1_dn8);
        locals.var_e1_1_dn9 = (4.0 * locals.var_e0_1_dn9);
        locals.var_e1_1_dn10 = (4.0 * locals.var_e0_1_dn10);
        locals.var_e1_1_dn11 = (4.0 * locals.var_e0_1_dn11);
        locals.var_e1_1_dn13 = (4.0 * locals.var_e0_1_dn13);
        locals.var_e1_1_dn14 = (4.0 * locals.var_e0_1_dn14);

        let assign14680_e21794: f64 = (4.0 * locals.var_e0prime);
        locals.var_e1prime = assign14680_e21794;
        locals.var_e1prime_dn0 = (4.0 * locals.var_e0prime_dn0);
        locals.var_e1prime_dn2 = (4.0 * locals.var_e0prime_dn2);
        locals.var_e1prime_dn3 = (4.0 * locals.var_e0prime_dn3);
        locals.var_e1prime_dn4 = (4.0 * locals.var_e0prime_dn4);
        locals.var_e1prime_dn5 = (4.0 * locals.var_e0prime_dn5);
        locals.var_e1prime_dn6 = (4.0 * locals.var_e0prime_dn6);
        locals.var_e1prime_dn7 = (4.0 * locals.var_e0prime_dn7);
        locals.var_e1prime_dn8 = (4.0 * locals.var_e0prime_dn8);
        locals.var_e1prime_dn9 = (4.0 * locals.var_e0prime_dn9);
        locals.var_e1prime_dn10 = (4.0 * locals.var_e0prime_dn10);
        locals.var_e1prime_dn11 = (4.0 * locals.var_e0prime_dn11);
        locals.var_e1prime_dn13 = (4.0 * locals.var_e0prime_dn13);
        locals.var_e1prime_dn14 = (4.0 * locals.var_e0prime_dn14);

        let assign14690_e21797: f64 = (locals.var_gprime * locals.var_mdprime);
        let assign14690_e21800: f64 = (locals.var_gfactor * locals.var_md);
        let assign14690_e21801: f64 = (assign14690_e21797 / assign14690_e21800);
        locals.var_t1 = assign14690_e21801;
        locals.var_t1_dn0 = 0.0;
        locals.var_t1_dn2 = 0.0;
        locals.var_t1_dn3 = 0.0;
        locals.var_t1_dn4 = 0.0;
        locals.var_t1_dn5 = 0.0;
        locals.var_t1_dn6 = 0.0;
        locals.var_t1_dn7 = 0.0;
        locals.var_t1_dn8 = 0.0;
        locals.var_t1_dn9 = 0.0;
        locals.var_t1_dn10 = 0.0;
        locals.var_t1_dn11 = 0.0;
        locals.var_t1_dn13 = 0.0;
        locals.var_t1_dn14 = 0.0;

        let assign14700_e21806: f64 = (locals.var_e0_1 - locals.var_e0prime);
        let assign14700_e21808: f64 = (assign14700_e21806 / locals.var_kt);
        let assign14700_e21809: f64 = { let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign14700_e21810: f64 = (locals.var_t1 * assign14700_e21809);
        let assign14700_e21811: f64 = (1.0 + assign14700_e21810);
        locals.var_gam0 = assign14700_e21811;
        locals.var_gam0_dn0 = ((locals.var_t1_dn0 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn0 - locals.var_e0prime_dn0) / locals.var_kt))));
        locals.var_gam0_dn2 = ((locals.var_t1_dn2 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn2 - locals.var_e0prime_dn2) / locals.var_kt))));
        locals.var_gam0_dn3 = ((locals.var_t1_dn3 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn3 - locals.var_e0prime_dn3) / locals.var_kt))));
        locals.var_gam0_dn4 = ((locals.var_t1_dn4 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_e0_1_dn4 - locals.var_e0prime_dn4) * locals.var_kt) - (assign14700_e21806 * locals.var_kt_dn4)) / (locals.var_kt * locals.var_kt)))));
        locals.var_gam0_dn5 = ((locals.var_t1_dn5 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn5 - locals.var_e0prime_dn5) / locals.var_kt))));
        locals.var_gam0_dn6 = ((locals.var_t1_dn6 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn6 - locals.var_e0prime_dn6) / locals.var_kt))));
        locals.var_gam0_dn7 = ((locals.var_t1_dn7 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn7 - locals.var_e0prime_dn7) / locals.var_kt))));
        locals.var_gam0_dn8 = ((locals.var_t1_dn8 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn8 - locals.var_e0prime_dn8) / locals.var_kt))));
        locals.var_gam0_dn9 = ((locals.var_t1_dn9 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn9 - locals.var_e0prime_dn9) / locals.var_kt))));
        locals.var_gam0_dn10 = ((locals.var_t1_dn10 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn10 - locals.var_e0prime_dn10) / locals.var_kt))));
        locals.var_gam0_dn11 = ((locals.var_t1_dn11 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn11 - locals.var_e0prime_dn11) / locals.var_kt))));
        locals.var_gam0_dn13 = ((locals.var_t1_dn13 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn13 - locals.var_e0prime_dn13) / locals.var_kt))));
        locals.var_gam0_dn14 = ((locals.var_t1_dn14 * assign14700_e21809) + (locals.var_t1 * ({ let limited_exp_arg = assign14700_e21808; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn14 - locals.var_e0prime_dn14) / locals.var_kt))));

        let assign14710_e21815: f64 = (locals.var_e0_1 - locals.var_e1_1);
        let assign14710_e21817: f64 = (assign14710_e21815 / locals.var_kt);
        let assign14710_e21818: f64 = { let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign14710_e21819: f64 = (locals.var_gam0 + assign14710_e21818);
        let assign14710_e21823: f64 = (locals.var_e0_1 - locals.var_e1prime);
        let assign14710_e21825: f64 = (assign14710_e21823 / locals.var_kt);
        let assign14710_e21826: f64 = { let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign14710_e21827: f64 = (locals.var_t1 * assign14710_e21826);
        let assign14710_e21828: f64 = (assign14710_e21819 + assign14710_e21827);
        locals.var_gam1 = assign14710_e21828;
        locals.var_gam1_dn0 = ((locals.var_gam0_dn0 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn0 - locals.var_e1_1_dn0) / locals.var_kt))) + ((locals.var_t1_dn0 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn0 - locals.var_e1prime_dn0) / locals.var_kt)))));
        locals.var_gam1_dn2 = ((locals.var_gam0_dn2 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn2 - locals.var_e1_1_dn2) / locals.var_kt))) + ((locals.var_t1_dn2 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn2 - locals.var_e1prime_dn2) / locals.var_kt)))));
        locals.var_gam1_dn3 = ((locals.var_gam0_dn3 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn3 - locals.var_e1_1_dn3) / locals.var_kt))) + ((locals.var_t1_dn3 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn3 - locals.var_e1prime_dn3) / locals.var_kt)))));
        locals.var_gam1_dn4 = ((locals.var_gam0_dn4 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_e0_1_dn4 - locals.var_e1_1_dn4) * locals.var_kt) - (assign14710_e21815 * locals.var_kt_dn4)) / (locals.var_kt * locals.var_kt)))) + ((locals.var_t1_dn4 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((((locals.var_e0_1_dn4 - locals.var_e1prime_dn4) * locals.var_kt) - (assign14710_e21823 * locals.var_kt_dn4)) / (locals.var_kt * locals.var_kt))))));
        locals.var_gam1_dn5 = ((locals.var_gam0_dn5 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn5 - locals.var_e1_1_dn5) / locals.var_kt))) + ((locals.var_t1_dn5 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn5 - locals.var_e1prime_dn5) / locals.var_kt)))));
        locals.var_gam1_dn6 = ((locals.var_gam0_dn6 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn6 - locals.var_e1_1_dn6) / locals.var_kt))) + ((locals.var_t1_dn6 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn6 - locals.var_e1prime_dn6) / locals.var_kt)))));
        locals.var_gam1_dn7 = ((locals.var_gam0_dn7 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn7 - locals.var_e1_1_dn7) / locals.var_kt))) + ((locals.var_t1_dn7 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn7 - locals.var_e1prime_dn7) / locals.var_kt)))));
        locals.var_gam1_dn8 = ((locals.var_gam0_dn8 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn8 - locals.var_e1_1_dn8) / locals.var_kt))) + ((locals.var_t1_dn8 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn8 - locals.var_e1prime_dn8) / locals.var_kt)))));
        locals.var_gam1_dn9 = ((locals.var_gam0_dn9 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn9 - locals.var_e1_1_dn9) / locals.var_kt))) + ((locals.var_t1_dn9 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn9 - locals.var_e1prime_dn9) / locals.var_kt)))));
        locals.var_gam1_dn10 = ((locals.var_gam0_dn10 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn10 - locals.var_e1_1_dn10) / locals.var_kt))) + ((locals.var_t1_dn10 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn10 - locals.var_e1prime_dn10) / locals.var_kt)))));
        locals.var_gam1_dn11 = ((locals.var_gam0_dn11 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn11 - locals.var_e1_1_dn11) / locals.var_kt))) + ((locals.var_t1_dn11 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn11 - locals.var_e1prime_dn11) / locals.var_kt)))));
        locals.var_gam1_dn13 = ((locals.var_gam0_dn13 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn13 - locals.var_e1_1_dn13) / locals.var_kt))) + ((locals.var_t1_dn13 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn13 - locals.var_e1prime_dn13) / locals.var_kt)))));
        locals.var_gam1_dn14 = ((locals.var_gam0_dn14 + ({ let limited_exp_arg = assign14710_e21817; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn14 - locals.var_e1_1_dn14) / locals.var_kt))) + ((locals.var_t1_dn14 * assign14710_e21826) + (locals.var_t1 * ({ let limited_exp_arg = assign14710_e21825; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * ((locals.var_e0_1_dn14 - locals.var_e1prime_dn14) / locals.var_kt)))));

        let assign14720_e21830: f64 = (-locals.var_vtm);
        let assign14720_e21833: f64 = (locals.var_gfactor * locals.var_md);
        let assign14720_e21836: f64 = (3.141592653589793 * 1.05457e-34);
        let assign14720_e21838: f64 = (assign14720_e21836 * 1.05457e-34);
        let assign14720_e21840: f64 = (assign14720_e21838 * locals.var_nc);
        let assign14720_e21841: f64 = (assign14720_e21833 / assign14720_e21840);
        let assign14720_e21843: f64 = (assign14720_e21841 * locals.var_kt);
        let assign14720_e21846: f64 = (2.0 * locals.var_ach);
        let assign14720_e21848: f64 = (assign14720_e21846 / locals.var_weff_ufcm);
        let assign14720_e21849: f64 = (assign14720_e21843 / assign14720_e21848);
        let assign14720_e21851: f64 = (assign14720_e21849 * locals.var_gam1);
        let (assign14720_e21904, assign14720_e21904_d_n0, assign14720_e21904_d_n2, assign14720_e21904_d_n3, assign14720_e21904_d_n4, assign14720_e21904_d_n5, assign14720_e21904_d_n6, assign14720_e21904_d_n7, assign14720_e21904_d_n8, assign14720_e21904_d_n9, assign14720_e21904_d_n10, assign14720_e21904_d_n11, assign14720_e21904_d_n13, assign14720_e21904_d_n14,) = {
    if (!(assign14720_e21851 > 1e-38)) {
        let assign14720_e21856: f64 = (-87.498233534);
        (assign14720_e21856, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        let assign14720_e21859: f64 = (locals.var_gfactor * locals.var_md);
        let assign14720_e21862: f64 = (3.141592653589793 * 1.05457e-34);
        let assign14720_e21864: f64 = (assign14720_e21862 * 1.05457e-34);
        let assign14720_e21866: f64 = (assign14720_e21864 * locals.var_nc);
        let assign14720_e21867: f64 = (assign14720_e21859 / assign14720_e21866);
        let assign14720_e21869: f64 = (assign14720_e21867 * locals.var_kt);
        let assign14720_e21872: f64 = (2.0 * locals.var_ach);
        let assign14720_e21874: f64 = (assign14720_e21872 / locals.var_weff_ufcm);
        let assign14720_e21875: f64 = (assign14720_e21869 / assign14720_e21874);
        let assign14720_e21877: f64 = (assign14720_e21875 * locals.var_gam1);
        let (assign14720_e21903, assign14720_e21903_d_n0, assign14720_e21903_d_n2, assign14720_e21903_d_n3, assign14720_e21903_d_n4, assign14720_e21903_d_n5, assign14720_e21903_d_n6, assign14720_e21903_d_n7, assign14720_e21903_d_n8, assign14720_e21903_d_n9, assign14720_e21903_d_n10, assign14720_e21903_d_n11, assign14720_e21903_d_n13, assign14720_e21903_d_n14,) = {
            if (assign14720_e21877 > 1e-38) {
                let assign14720_e21882: f64 = (locals.var_gfactor * locals.var_md);
                let assign14720_e21885: f64 = (3.141592653589793 * 1.05457e-34);
                let assign14720_e21887: f64 = (assign14720_e21885 * 1.05457e-34);
                let assign14720_e21889: f64 = (assign14720_e21887 * locals.var_nc);
                let assign14720_e21890: f64 = (assign14720_e21882 / assign14720_e21889);
                let assign14720_e21892: f64 = (assign14720_e21890 * locals.var_kt);
                let assign14720_e21895: f64 = (2.0 * locals.var_ach);
                let assign14720_e21897: f64 = (assign14720_e21895 / locals.var_weff_ufcm);
                let assign14720_e21898: f64 = (assign14720_e21892 / assign14720_e21897);
                let assign14720_e21900: f64 = (assign14720_e21898 * locals.var_gam1);
                let assign14720_e21901: f64 = (assign14720_e21900).ln();
                (assign14720_e21901, ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn0)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn0)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn2)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn2)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn3)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn3)) / assign14720_e21900), (((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn4)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) + (assign14720_e21890 * locals.var_kt_dn4)) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn4)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn5)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn5)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn6)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn6)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn7)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn7)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn8)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn8)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn9)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn9)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn10)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn10)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn11)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn11)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn13)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn13)) / assign14720_e21900), ((((((-((assign14720_e21882 * (assign14720_e21887 * locals.var_nc_dn14)) / (assign14720_e21889 * assign14720_e21889))) * locals.var_kt) / assign14720_e21897) * locals.var_gam1) + (assign14720_e21898 * locals.var_gam1_dn14)) / assign14720_e21900),)
            } else {
                (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
            }
        };
        (assign14720_e21903, assign14720_e21903_d_n0, assign14720_e21903_d_n2, assign14720_e21903_d_n3, assign14720_e21903_d_n4, assign14720_e21903_d_n5, assign14720_e21903_d_n6, assign14720_e21903_d_n7, assign14720_e21903_d_n8, assign14720_e21903_d_n9, assign14720_e21903_d_n10, assign14720_e21903_d_n11, assign14720_e21903_d_n13, assign14720_e21903_d_n14,)
    }
};
        let assign14720_e21905: f64 = (assign14720_e21830 * assign14720_e21904);
        locals.var_t2 = assign14720_e21905;
        locals.var_t2_dn0 = (assign14720_e21830 * assign14720_e21904_d_n0);
        locals.var_t2_dn2 = (assign14720_e21830 * assign14720_e21904_d_n2);
        locals.var_t2_dn3 = (assign14720_e21830 * assign14720_e21904_d_n3);
        locals.var_t2_dn4 = (((-locals.var_vtm_dn4) * assign14720_e21904) + (assign14720_e21830 * assign14720_e21904_d_n4));
        locals.var_t2_dn5 = (assign14720_e21830 * assign14720_e21904_d_n5);
        locals.var_t2_dn6 = (assign14720_e21830 * assign14720_e21904_d_n6);
        locals.var_t2_dn7 = (assign14720_e21830 * assign14720_e21904_d_n7);
        locals.var_t2_dn8 = (assign14720_e21830 * assign14720_e21904_d_n8);
        locals.var_t2_dn9 = (assign14720_e21830 * assign14720_e21904_d_n9);
        locals.var_t2_dn10 = (assign14720_e21830 * assign14720_e21904_d_n10);
        locals.var_t2_dn11 = (assign14720_e21830 * assign14720_e21904_d_n11);
        locals.var_t2_dn13 = (assign14720_e21830 * assign14720_e21904_d_n13);
        locals.var_t2_dn14 = (assign14720_e21830 * assign14720_e21904_d_n14);

        let assign14730_e21909: f64 = (locals.var_e0_1 / 1.60219e-19);
        let assign14730_e21911: f64 = (assign14730_e21909 + locals.var_t2);
        let assign14730_e21912: f64 = (locals.var_qmfactor_i * assign14730_e21911);
        locals.var_dvch_qm = assign14730_e21912;
        locals.var_dvch_qm_dn0 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn0 / 1.60219e-19) + locals.var_t2_dn0));
        locals.var_dvch_qm_dn2 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn2 / 1.60219e-19) + locals.var_t2_dn2));
        locals.var_dvch_qm_dn3 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn3 / 1.60219e-19) + locals.var_t2_dn3));
        locals.var_dvch_qm_dn4 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn4 / 1.60219e-19) + locals.var_t2_dn4));
        locals.var_dvch_qm_dn5 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn5 / 1.60219e-19) + locals.var_t2_dn5));
        locals.var_dvch_qm_dn6 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn6 / 1.60219e-19) + locals.var_t2_dn6));
        locals.var_dvch_qm_dn7 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn7 / 1.60219e-19) + locals.var_t2_dn7));
        locals.var_dvch_qm_dn8 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn8 / 1.60219e-19) + locals.var_t2_dn8));
        locals.var_dvch_qm_dn9 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn9 / 1.60219e-19) + locals.var_t2_dn9));
        locals.var_dvch_qm_dn10 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn10 / 1.60219e-19) + locals.var_t2_dn10));
        locals.var_dvch_qm_dn11 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn11 / 1.60219e-19) + locals.var_t2_dn11));
        locals.var_dvch_qm_dn13 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn13 / 1.60219e-19) + locals.var_t2_dn13));
        locals.var_dvch_qm_dn14 = (locals.var_qmfactor_i * ((locals.var_e0_1_dn14 / 1.60219e-19) + locals.var_t2_dn14));

        let assign14740_e21914: f64 = (locals.var_tratio).ln();
        locals.var_trat_ln = assign14740_e21914;
        locals.var_trat_ln_dn4 = (locals.var_tratio_dn4 / locals.var_tratio);

        let assign14750_e21917: f64 = if p.p80 == 0.0 { 1.0 } else { 0.0 };
        locals.var_guard244 = assign14750_e21917;

        let (assign14760_e21926, assign14760_e21926_d_n0, assign14760_e21926_d_n2, assign14760_e21926_d_n3, assign14760_e21926_d_n4, assign14760_e21926_d_n5, assign14760_e21926_d_n6, assign14760_e21926_d_n7, assign14760_e21926_d_n8, assign14760_e21926_d_n9, assign14760_e21926_d_n10, assign14760_e21926_d_n11, assign14760_e21926_d_n13, assign14760_e21926_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        let assign14760_e21922: f64 = (locals.var_ute_i * locals.var_trat_ln);
        let assign14760_e21923: f64 = (assign14760_e21922).exp();
        let assign14760_e21924: f64 = (locals.var_u0_i * assign14760_e21923);
        (assign14760_e21924, (locals.var_u0_i_dn0 * assign14760_e21923), (locals.var_u0_i_dn2 * assign14760_e21923), (locals.var_u0_i_dn3 * assign14760_e21923), ((locals.var_u0_i_dn4 * assign14760_e21923) + (locals.var_u0_i * (assign14760_e21923 * (locals.var_ute_i * locals.var_trat_ln_dn4)))), (locals.var_u0_i_dn5 * assign14760_e21923), (locals.var_u0_i_dn6 * assign14760_e21923), (locals.var_u0_i_dn7 * assign14760_e21923), (locals.var_u0_i_dn8 * assign14760_e21923), (locals.var_u0_i_dn9 * assign14760_e21923), (locals.var_u0_i_dn10 * assign14760_e21923), (locals.var_u0_i_dn11 * assign14760_e21923), (locals.var_u0_i_dn13 * assign14760_e21923), (locals.var_u0_i_dn14 * assign14760_e21923),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign14760_e21926;
        locals.var_t1_dn0 = assign14760_e21926_d_n0;
        locals.var_t1_dn2 = assign14760_e21926_d_n2;
        locals.var_t1_dn3 = assign14760_e21926_d_n3;
        locals.var_t1_dn4 = assign14760_e21926_d_n4;
        locals.var_t1_dn5 = assign14760_e21926_d_n5;
        locals.var_t1_dn6 = assign14760_e21926_d_n6;
        locals.var_t1_dn7 = assign14760_e21926_d_n7;
        locals.var_t1_dn8 = assign14760_e21926_d_n8;
        locals.var_t1_dn9 = assign14760_e21926_d_n9;
        locals.var_t1_dn10 = assign14760_e21926_d_n10;
        locals.var_t1_dn11 = assign14760_e21926_d_n11;
        locals.var_t1_dn13 = assign14760_e21926_d_n13;
        locals.var_t1_dn14 = assign14760_e21926_d_n14;

        let (assign14770_e21980, assign14770_e21980_d_n0, assign14770_e21980_d_n2, assign14770_e21980_d_n3, assign14770_e21980_d_n4, assign14770_e21980_d_n5, assign14770_e21980_d_n6, assign14770_e21980_d_n7, assign14770_e21980_d_n8, assign14770_e21980_d_n9, assign14770_e21980_d_n10, assign14770_e21980_d_n11, assign14770_e21980_d_n13, assign14770_e21980_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        let assign14770_e21930: f64 = (-0.9);
        let assign14770_e21932: f64 = (assign14770_e21930 * locals.var_t1);
        let assign14770_e21936: f64 = (locals.var_utl_i * locals.var_deltemp);
        let assign14770_e21938: f64 = (-0.9);
        let assign14770_e21940: f64 = (assign14770_e21938 * locals.var_t1);
        let assign14770_e21941: f64 = (assign14770_e21936 - assign14770_e21940);
        let assign14770_e21943: f64 = (assign14770_e21941 - 0.0001);
        let assign14770_e21946: f64 = (locals.var_utl_i * locals.var_deltemp);
        let assign14770_e21948: f64 = (-0.9);
        let assign14770_e21950: f64 = (assign14770_e21948 * locals.var_t1);
        let assign14770_e21951: f64 = (assign14770_e21946 - assign14770_e21950);
        let assign14770_e21953: f64 = (assign14770_e21951 - 0.0001);
        let assign14770_e21956: f64 = (locals.var_utl_i * locals.var_deltemp);
        let assign14770_e21958: f64 = (-0.9);
        let assign14770_e21960: f64 = (assign14770_e21958 * locals.var_t1);
        let assign14770_e21961: f64 = (assign14770_e21956 - assign14770_e21960);
        let assign14770_e21963: f64 = (assign14770_e21961 - 0.0001);
        let assign14770_e21964: f64 = (assign14770_e21953 * assign14770_e21963);
        let assign14770_e21967: f64 = (-0.9);
        let assign14770_e21969: f64 = (assign14770_e21967 * locals.var_t1);
        let assign14770_e21970: f64 = (4.0 * assign14770_e21969);
        let assign14770_e21972: f64 = (assign14770_e21970 * 0.0001);
        let assign14770_e21973: f64 = (assign14770_e21964 - assign14770_e21972);
        let assign14770_e21974: f64 = (assign14770_e21973).sqrt();
        let assign14770_e21975: f64 = (assign14770_e21943 + assign14770_e21974);
        let assign14770_e21976: f64 = (0.5 * assign14770_e21975);
        let assign14770_e21977: f64 = (assign14770_e21932 + assign14770_e21976);
        let assign14770_e21978: f64 = (locals.var_t1 + assign14770_e21977);
        (assign14770_e21978, (locals.var_t1_dn0 + ((assign14770_e21930 * locals.var_t1_dn0) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn0)) + (((((-(assign14770_e21948 * locals.var_t1_dn0)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn0)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn0)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn2 + ((assign14770_e21930 * locals.var_t1_dn2) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn2)) + (((((-(assign14770_e21948 * locals.var_t1_dn2)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn2)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn2)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn3 + ((assign14770_e21930 * locals.var_t1_dn3) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn3)) + (((((-(assign14770_e21948 * locals.var_t1_dn3)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn3)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn3)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn4 + ((assign14770_e21930 * locals.var_t1_dn4) + (0.5 * (((locals.var_utl_i * locals.var_deltemp_dn4) - (assign14770_e21938 * locals.var_t1_dn4)) + ((((((locals.var_utl_i * locals.var_deltemp_dn4) - (assign14770_e21948 * locals.var_t1_dn4)) * assign14770_e21963) + (assign14770_e21953 * ((locals.var_utl_i * locals.var_deltemp_dn4) - (assign14770_e21958 * locals.var_t1_dn4)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn4)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn5 + ((assign14770_e21930 * locals.var_t1_dn5) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn5)) + (((((-(assign14770_e21948 * locals.var_t1_dn5)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn5)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn5)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn6 + ((assign14770_e21930 * locals.var_t1_dn6) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn6)) + (((((-(assign14770_e21948 * locals.var_t1_dn6)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn6)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn6)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn7 + ((assign14770_e21930 * locals.var_t1_dn7) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn7)) + (((((-(assign14770_e21948 * locals.var_t1_dn7)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn7)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn7)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn8 + ((assign14770_e21930 * locals.var_t1_dn8) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn8)) + (((((-(assign14770_e21948 * locals.var_t1_dn8)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn8)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn8)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn9 + ((assign14770_e21930 * locals.var_t1_dn9) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn9)) + (((((-(assign14770_e21948 * locals.var_t1_dn9)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn9)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn9)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn10 + ((assign14770_e21930 * locals.var_t1_dn10) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn10)) + (((((-(assign14770_e21948 * locals.var_t1_dn10)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn10)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn10)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn11 + ((assign14770_e21930 * locals.var_t1_dn11) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn11)) + (((((-(assign14770_e21948 * locals.var_t1_dn11)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn11)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn11)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn13 + ((assign14770_e21930 * locals.var_t1_dn13) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn13)) + (((((-(assign14770_e21948 * locals.var_t1_dn13)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn13)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn13)) * 0.0001)) / (2.0 * assign14770_e21974)))))), (locals.var_t1_dn14 + ((assign14770_e21930 * locals.var_t1_dn14) + (0.5 * ((-(assign14770_e21938 * locals.var_t1_dn14)) + (((((-(assign14770_e21948 * locals.var_t1_dn14)) * assign14770_e21963) + (assign14770_e21953 * (-(assign14770_e21958 * locals.var_t1_dn14)))) - ((4.0 * (assign14770_e21967 * locals.var_t1_dn14)) * 0.0001)) / (2.0 * assign14770_e21974)))))),)
    } else {
        (locals.var_u0_v, locals.var_u0_v_dn0, locals.var_u0_v_dn2, locals.var_u0_v_dn3, locals.var_u0_v_dn4, locals.var_u0_v_dn5, locals.var_u0_v_dn6, locals.var_u0_v_dn7, locals.var_u0_v_dn8, locals.var_u0_v_dn9, locals.var_u0_v_dn10, locals.var_u0_v_dn11, locals.var_u0_v_dn13, locals.var_u0_v_dn14,)
    }
};
        locals.var_u0_v = assign14770_e21980;
        locals.var_u0_v_dn0 = assign14770_e21980_d_n0;
        locals.var_u0_v_dn2 = assign14770_e21980_d_n2;
        locals.var_u0_v_dn3 = assign14770_e21980_d_n3;
        locals.var_u0_v_dn4 = assign14770_e21980_d_n4;
        locals.var_u0_v_dn5 = assign14770_e21980_d_n5;
        locals.var_u0_v_dn6 = assign14770_e21980_d_n6;
        locals.var_u0_v_dn7 = assign14770_e21980_d_n7;
        locals.var_u0_v_dn8 = assign14770_e21980_d_n8;
        locals.var_u0_v_dn9 = assign14770_e21980_d_n9;
        locals.var_u0_v_dn10 = assign14770_e21980_d_n10;
        locals.var_u0_v_dn11 = assign14770_e21980_d_n11;
        locals.var_u0_v_dn13 = assign14770_e21980_d_n13;
        locals.var_u0_v_dn14 = assign14770_e21980_d_n14;

        let assign14780_e21983: f64 = if p.p66 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard245 = assign14780_e21983;

        let (assign14790_e21994, assign14790_e21994_d_n0, assign14790_e21994_d_n2, assign14790_e21994_d_n3, assign14790_e21994_d_n4, assign14790_e21994_d_n5, assign14790_e21994_d_n6, assign14790_e21994_d_n7, assign14790_e21994_d_n8, assign14790_e21994_d_n9, assign14790_e21994_d_n10, assign14790_e21994_d_n11, assign14790_e21994_d_n13, assign14790_e21994_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard245 != 0.0)) {
        let assign14790_e21990: f64 = (locals.var_uter_i * locals.var_trat_ln);
        let assign14790_e21991: f64 = (assign14790_e21990).exp();
        let assign14790_e21992: f64 = (locals.var_u0r_i * assign14790_e21991);
        (assign14790_e21992, (locals.var_u0r_i_dn0 * assign14790_e21991), (locals.var_u0r_i_dn2 * assign14790_e21991), (locals.var_u0r_i_dn3 * assign14790_e21991), ((locals.var_u0r_i_dn4 * assign14790_e21991) + (locals.var_u0r_i * (assign14790_e21991 * (locals.var_uter_i * locals.var_trat_ln_dn4)))), (locals.var_u0r_i_dn5 * assign14790_e21991), (locals.var_u0r_i_dn6 * assign14790_e21991), (locals.var_u0r_i_dn7 * assign14790_e21991), (locals.var_u0r_i_dn8 * assign14790_e21991), (locals.var_u0r_i_dn9 * assign14790_e21991), (locals.var_u0r_i_dn10 * assign14790_e21991), (locals.var_u0r_i_dn11 * assign14790_e21991), (locals.var_u0r_i_dn13 * assign14790_e21991), (locals.var_u0r_i_dn14 * assign14790_e21991),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign14790_e21994;
        locals.var_t1_dn0 = assign14790_e21994_d_n0;
        locals.var_t1_dn2 = assign14790_e21994_d_n2;
        locals.var_t1_dn3 = assign14790_e21994_d_n3;
        locals.var_t1_dn4 = assign14790_e21994_d_n4;
        locals.var_t1_dn5 = assign14790_e21994_d_n5;
        locals.var_t1_dn6 = assign14790_e21994_d_n6;
        locals.var_t1_dn7 = assign14790_e21994_d_n7;
        locals.var_t1_dn8 = assign14790_e21994_d_n8;
        locals.var_t1_dn9 = assign14790_e21994_d_n9;
        locals.var_t1_dn10 = assign14790_e21994_d_n10;
        locals.var_t1_dn11 = assign14790_e21994_d_n11;
        locals.var_t1_dn13 = assign14790_e21994_d_n13;
        locals.var_t1_dn14 = assign14790_e21994_d_n14;

        let (assign14800_e22050, assign14800_e22050_d_n0, assign14800_e22050_d_n2, assign14800_e22050_d_n3, assign14800_e22050_d_n4, assign14800_e22050_d_n5, assign14800_e22050_d_n6, assign14800_e22050_d_n7, assign14800_e22050_d_n8, assign14800_e22050_d_n9, assign14800_e22050_d_n10, assign14800_e22050_d_n11, assign14800_e22050_d_n13, assign14800_e22050_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard245 != 0.0)) {
        let assign14800_e22000: f64 = (-0.9);
        let assign14800_e22002: f64 = (assign14800_e22000 * locals.var_t1);
        let assign14800_e22006: f64 = (locals.var_utlr_i * locals.var_deltemp);
        let assign14800_e22008: f64 = (-0.9);
        let assign14800_e22010: f64 = (assign14800_e22008 * locals.var_t1);
        let assign14800_e22011: f64 = (assign14800_e22006 - assign14800_e22010);
        let assign14800_e22013: f64 = (assign14800_e22011 - 0.0001);
        let assign14800_e22016: f64 = (locals.var_utlr_i * locals.var_deltemp);
        let assign14800_e22018: f64 = (-0.9);
        let assign14800_e22020: f64 = (assign14800_e22018 * locals.var_t1);
        let assign14800_e22021: f64 = (assign14800_e22016 - assign14800_e22020);
        let assign14800_e22023: f64 = (assign14800_e22021 - 0.0001);
        let assign14800_e22026: f64 = (locals.var_utlr_i * locals.var_deltemp);
        let assign14800_e22028: f64 = (-0.9);
        let assign14800_e22030: f64 = (assign14800_e22028 * locals.var_t1);
        let assign14800_e22031: f64 = (assign14800_e22026 - assign14800_e22030);
        let assign14800_e22033: f64 = (assign14800_e22031 - 0.0001);
        let assign14800_e22034: f64 = (assign14800_e22023 * assign14800_e22033);
        let assign14800_e22037: f64 = (-0.9);
        let assign14800_e22039: f64 = (assign14800_e22037 * locals.var_t1);
        let assign14800_e22040: f64 = (4.0 * assign14800_e22039);
        let assign14800_e22042: f64 = (assign14800_e22040 * 0.0001);
        let assign14800_e22043: f64 = (assign14800_e22034 - assign14800_e22042);
        let assign14800_e22044: f64 = (assign14800_e22043).sqrt();
        let assign14800_e22045: f64 = (assign14800_e22013 + assign14800_e22044);
        let assign14800_e22046: f64 = (0.5 * assign14800_e22045);
        let assign14800_e22047: f64 = (assign14800_e22002 + assign14800_e22046);
        let assign14800_e22048: f64 = (locals.var_t1 + assign14800_e22047);
        (assign14800_e22048, (locals.var_t1_dn0 + ((assign14800_e22000 * locals.var_t1_dn0) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn0)) + (((((-(assign14800_e22018 * locals.var_t1_dn0)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn0)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn0)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn2 + ((assign14800_e22000 * locals.var_t1_dn2) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn2)) + (((((-(assign14800_e22018 * locals.var_t1_dn2)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn2)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn2)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn3 + ((assign14800_e22000 * locals.var_t1_dn3) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn3)) + (((((-(assign14800_e22018 * locals.var_t1_dn3)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn3)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn3)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn4 + ((assign14800_e22000 * locals.var_t1_dn4) + (0.5 * (((locals.var_utlr_i * locals.var_deltemp_dn4) - (assign14800_e22008 * locals.var_t1_dn4)) + ((((((locals.var_utlr_i * locals.var_deltemp_dn4) - (assign14800_e22018 * locals.var_t1_dn4)) * assign14800_e22033) + (assign14800_e22023 * ((locals.var_utlr_i * locals.var_deltemp_dn4) - (assign14800_e22028 * locals.var_t1_dn4)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn4)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn5 + ((assign14800_e22000 * locals.var_t1_dn5) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn5)) + (((((-(assign14800_e22018 * locals.var_t1_dn5)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn5)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn5)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn6 + ((assign14800_e22000 * locals.var_t1_dn6) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn6)) + (((((-(assign14800_e22018 * locals.var_t1_dn6)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn6)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn6)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn7 + ((assign14800_e22000 * locals.var_t1_dn7) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn7)) + (((((-(assign14800_e22018 * locals.var_t1_dn7)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn7)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn7)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn8 + ((assign14800_e22000 * locals.var_t1_dn8) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn8)) + (((((-(assign14800_e22018 * locals.var_t1_dn8)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn8)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn8)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn9 + ((assign14800_e22000 * locals.var_t1_dn9) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn9)) + (((((-(assign14800_e22018 * locals.var_t1_dn9)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn9)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn9)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn10 + ((assign14800_e22000 * locals.var_t1_dn10) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn10)) + (((((-(assign14800_e22018 * locals.var_t1_dn10)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn10)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn10)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn11 + ((assign14800_e22000 * locals.var_t1_dn11) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn11)) + (((((-(assign14800_e22018 * locals.var_t1_dn11)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn11)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn11)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn13 + ((assign14800_e22000 * locals.var_t1_dn13) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn13)) + (((((-(assign14800_e22018 * locals.var_t1_dn13)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn13)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn13)) * 0.0001)) / (2.0 * assign14800_e22044)))))), (locals.var_t1_dn14 + ((assign14800_e22000 * locals.var_t1_dn14) + (0.5 * ((-(assign14800_e22008 * locals.var_t1_dn14)) + (((((-(assign14800_e22018 * locals.var_t1_dn14)) * assign14800_e22033) + (assign14800_e22023 * (-(assign14800_e22028 * locals.var_t1_dn14)))) - ((4.0 * (assign14800_e22037 * locals.var_t1_dn14)) * 0.0001)) / (2.0 * assign14800_e22044)))))),)
    } else {
        (locals.var_u0r_t, locals.var_u0r_t_dn0, locals.var_u0r_t_dn2, locals.var_u0r_t_dn3, locals.var_u0r_t_dn4, locals.var_u0r_t_dn5, locals.var_u0r_t_dn6, locals.var_u0r_t_dn7, locals.var_u0r_t_dn8, locals.var_u0r_t_dn9, locals.var_u0r_t_dn10, locals.var_u0r_t_dn11, locals.var_u0r_t_dn13, locals.var_u0r_t_dn14,)
    }
};
        locals.var_u0r_t = assign14800_e22050;
        locals.var_u0r_t_dn0 = assign14800_e22050_d_n0;
        locals.var_u0r_t_dn2 = assign14800_e22050_d_n2;
        locals.var_u0r_t_dn3 = assign14800_e22050_d_n3;
        locals.var_u0r_t_dn4 = assign14800_e22050_d_n4;
        locals.var_u0r_t_dn5 = assign14800_e22050_d_n5;
        locals.var_u0r_t_dn6 = assign14800_e22050_d_n6;
        locals.var_u0r_t_dn7 = assign14800_e22050_d_n7;
        locals.var_u0r_t_dn8 = assign14800_e22050_d_n8;
        locals.var_u0r_t_dn9 = assign14800_e22050_d_n9;
        locals.var_u0r_t_dn10 = assign14800_e22050_d_n10;
        locals.var_u0r_t_dn11 = assign14800_e22050_d_n11;
        locals.var_u0r_t_dn13 = assign14800_e22050_d_n13;
        locals.var_u0r_t_dn14 = assign14800_e22050_d_n14;

        let (assign14810_e22056, assign14810_e22056_d_n0, assign14810_e22056_d_n2, assign14810_e22056_d_n3, assign14810_e22056_d_n4, assign14810_e22056_d_n5, assign14810_e22056_d_n6, assign14810_e22056_d_n7, assign14810_e22056_d_n8, assign14810_e22056_d_n9, assign14810_e22056_d_n10, assign14810_e22056_d_n11, assign14810_e22056_d_n13, assign14810_e22056_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard245 != 0.0)) {
        (locals.var_u0r_t, locals.var_u0r_t_dn0, locals.var_u0r_t_dn2, locals.var_u0r_t_dn3, locals.var_u0r_t_dn4, locals.var_u0r_t_dn5, locals.var_u0r_t_dn6, locals.var_u0r_t_dn7, locals.var_u0r_t_dn8, locals.var_u0r_t_dn9, locals.var_u0r_t_dn10, locals.var_u0r_t_dn11, locals.var_u0r_t_dn13, locals.var_u0r_t_dn14,)
    } else {
        (locals.var_u0r_v, locals.var_u0r_v_dn0, locals.var_u0r_v_dn2, locals.var_u0r_v_dn3, locals.var_u0r_v_dn4, locals.var_u0r_v_dn5, locals.var_u0r_v_dn6, locals.var_u0r_v_dn7, locals.var_u0r_v_dn8, locals.var_u0r_v_dn9, locals.var_u0r_v_dn10, locals.var_u0r_v_dn11, locals.var_u0r_v_dn13, locals.var_u0r_v_dn14,)
    }
};
        locals.var_u0r_v = assign14810_e22056;
        locals.var_u0r_v_dn0 = assign14810_e22056_d_n0;
        locals.var_u0r_v_dn2 = assign14810_e22056_d_n2;
        locals.var_u0r_v_dn3 = assign14810_e22056_d_n3;
        locals.var_u0r_v_dn4 = assign14810_e22056_d_n4;
        locals.var_u0r_v_dn5 = assign14810_e22056_d_n5;
        locals.var_u0r_v_dn6 = assign14810_e22056_d_n6;
        locals.var_u0r_v_dn7 = assign14810_e22056_d_n7;
        locals.var_u0r_v_dn8 = assign14810_e22056_d_n8;
        locals.var_u0r_v_dn9 = assign14810_e22056_d_n9;
        locals.var_u0r_v_dn10 = assign14810_e22056_d_n10;
        locals.var_u0r_v_dn11 = assign14810_e22056_d_n11;
        locals.var_u0r_v_dn13 = assign14810_e22056_d_n13;
        locals.var_u0r_v_dn14 = assign14810_e22056_d_n14;

        let (assign14820_e22100, assign14820_e22100_d_n0, assign14820_e22100_d_n2, assign14820_e22100_d_n3, assign14820_e22100_d_n4, assign14820_e22100_d_n5, assign14820_e22100_d_n6, assign14820_e22100_d_n7, assign14820_e22100_d_n8, assign14820_e22100_d_n9, assign14820_e22100_d_n10, assign14820_e22100_d_n11, assign14820_e22100_d_n13, assign14820_e22100_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        let assign14820_e22060: f64 = (-locals.var_ua_i);
        let assign14820_e22064: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign14820_e22066: f64 = (-locals.var_ua_i);
        let assign14820_e22067: f64 = (assign14820_e22064 - assign14820_e22066);
        let assign14820_e22069: f64 = (assign14820_e22067 - 1e-6);
        let assign14820_e22072: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign14820_e22074: f64 = (-locals.var_ua_i);
        let assign14820_e22075: f64 = (assign14820_e22072 - assign14820_e22074);
        let assign14820_e22077: f64 = (assign14820_e22075 - 1e-6);
        let assign14820_e22080: f64 = (locals.var_ua1_i * locals.var_deltemp);
        let assign14820_e22082: f64 = (-locals.var_ua_i);
        let assign14820_e22083: f64 = (assign14820_e22080 - assign14820_e22082);
        let assign14820_e22085: f64 = (assign14820_e22083 - 1e-6);
        let assign14820_e22086: f64 = (assign14820_e22077 * assign14820_e22085);
        let assign14820_e22089: f64 = (-locals.var_ua_i);
        let assign14820_e22090: f64 = (4.0 * assign14820_e22089);
        let assign14820_e22092: f64 = (assign14820_e22090 * 1e-6);
        let assign14820_e22093: f64 = (assign14820_e22086 - assign14820_e22092);
        let assign14820_e22094: f64 = (assign14820_e22093).sqrt();
        let assign14820_e22095: f64 = (assign14820_e22069 + assign14820_e22094);
        let assign14820_e22096: f64 = (0.5 * assign14820_e22095);
        let assign14820_e22097: f64 = (assign14820_e22060 + assign14820_e22096);
        let assign14820_e22098: f64 = (locals.var_ua_i + assign14820_e22097);
        (assign14820_e22098, (locals.var_ua_i_dn0 + ((-locals.var_ua_i_dn0) + (0.5 * ((-(-locals.var_ua_i_dn0)) + (((((-(-locals.var_ua_i_dn0)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn0)))) - ((4.0 * (-locals.var_ua_i_dn0)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn2 + ((-locals.var_ua_i_dn2) + (0.5 * ((-(-locals.var_ua_i_dn2)) + (((((-(-locals.var_ua_i_dn2)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn2)))) - ((4.0 * (-locals.var_ua_i_dn2)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn3 + ((-locals.var_ua_i_dn3) + (0.5 * ((-(-locals.var_ua_i_dn3)) + (((((-(-locals.var_ua_i_dn3)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn3)))) - ((4.0 * (-locals.var_ua_i_dn3)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn4 + ((-locals.var_ua_i_dn4) + (0.5 * (((locals.var_ua1_i * locals.var_deltemp_dn4) - (-locals.var_ua_i_dn4)) + ((((((locals.var_ua1_i * locals.var_deltemp_dn4) - (-locals.var_ua_i_dn4)) * assign14820_e22085) + (assign14820_e22077 * ((locals.var_ua1_i * locals.var_deltemp_dn4) - (-locals.var_ua_i_dn4)))) - ((4.0 * (-locals.var_ua_i_dn4)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn5 + ((-locals.var_ua_i_dn5) + (0.5 * ((-(-locals.var_ua_i_dn5)) + (((((-(-locals.var_ua_i_dn5)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn5)))) - ((4.0 * (-locals.var_ua_i_dn5)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn6 + ((-locals.var_ua_i_dn6) + (0.5 * ((-(-locals.var_ua_i_dn6)) + (((((-(-locals.var_ua_i_dn6)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn6)))) - ((4.0 * (-locals.var_ua_i_dn6)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn7 + ((-locals.var_ua_i_dn7) + (0.5 * ((-(-locals.var_ua_i_dn7)) + (((((-(-locals.var_ua_i_dn7)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn7)))) - ((4.0 * (-locals.var_ua_i_dn7)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn8 + ((-locals.var_ua_i_dn8) + (0.5 * ((-(-locals.var_ua_i_dn8)) + (((((-(-locals.var_ua_i_dn8)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn8)))) - ((4.0 * (-locals.var_ua_i_dn8)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn9 + ((-locals.var_ua_i_dn9) + (0.5 * ((-(-locals.var_ua_i_dn9)) + (((((-(-locals.var_ua_i_dn9)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn9)))) - ((4.0 * (-locals.var_ua_i_dn9)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn10 + ((-locals.var_ua_i_dn10) + (0.5 * ((-(-locals.var_ua_i_dn10)) + (((((-(-locals.var_ua_i_dn10)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn10)))) - ((4.0 * (-locals.var_ua_i_dn10)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn11 + ((-locals.var_ua_i_dn11) + (0.5 * ((-(-locals.var_ua_i_dn11)) + (((((-(-locals.var_ua_i_dn11)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn11)))) - ((4.0 * (-locals.var_ua_i_dn11)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn13 + ((-locals.var_ua_i_dn13) + (0.5 * ((-(-locals.var_ua_i_dn13)) + (((((-(-locals.var_ua_i_dn13)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn13)))) - ((4.0 * (-locals.var_ua_i_dn13)) * 1e-6)) / (2.0 * assign14820_e22094)))))), (locals.var_ua_i_dn14 + ((-locals.var_ua_i_dn14) + (0.5 * ((-(-locals.var_ua_i_dn14)) + (((((-(-locals.var_ua_i_dn14)) * assign14820_e22085) + (assign14820_e22077 * (-(-locals.var_ua_i_dn14)))) - ((4.0 * (-locals.var_ua_i_dn14)) * 1e-6)) / (2.0 * assign14820_e22094)))))),)
    } else {
        (locals.var_ua_t, locals.var_ua_t_dn0, locals.var_ua_t_dn2, locals.var_ua_t_dn3, locals.var_ua_t_dn4, locals.var_ua_t_dn5, locals.var_ua_t_dn6, locals.var_ua_t_dn7, locals.var_ua_t_dn8, locals.var_ua_t_dn9, locals.var_ua_t_dn10, locals.var_ua_t_dn11, locals.var_ua_t_dn13, locals.var_ua_t_dn14,)
    }
};
        locals.var_ua_t = assign14820_e22100;
        locals.var_ua_t_dn0 = assign14820_e22100_d_n0;
        locals.var_ua_t_dn2 = assign14820_e22100_d_n2;
        locals.var_ua_t_dn3 = assign14820_e22100_d_n3;
        locals.var_ua_t_dn4 = assign14820_e22100_d_n4;
        locals.var_ua_t_dn5 = assign14820_e22100_d_n5;
        locals.var_ua_t_dn6 = assign14820_e22100_d_n6;
        locals.var_ua_t_dn7 = assign14820_e22100_d_n7;
        locals.var_ua_t_dn8 = assign14820_e22100_d_n8;
        locals.var_ua_t_dn9 = assign14820_e22100_d_n9;
        locals.var_ua_t_dn10 = assign14820_e22100_d_n10;
        locals.var_ua_t_dn11 = assign14820_e22100_d_n11;
        locals.var_ua_t_dn13 = assign14820_e22100_d_n13;
        locals.var_ua_t_dn14 = assign14820_e22100_d_n14;

        let (assign14830_e22104, assign14830_e22104_d_n0, assign14830_e22104_d_n2, assign14830_e22104_d_n3, assign14830_e22104_d_n4, assign14830_e22104_d_n5, assign14830_e22104_d_n6, assign14830_e22104_d_n7, assign14830_e22104_d_n8, assign14830_e22104_d_n9, assign14830_e22104_d_n10, assign14830_e22104_d_n11, assign14830_e22104_d_n13, assign14830_e22104_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        (locals.var_eu_i, locals.var_eu_i_dn0, locals.var_eu_i_dn2, locals.var_eu_i_dn3, locals.var_eu_i_dn4, locals.var_eu_i_dn5, locals.var_eu_i_dn6, locals.var_eu_i_dn7, locals.var_eu_i_dn8, locals.var_eu_i_dn9, locals.var_eu_i_dn10, locals.var_eu_i_dn11, locals.var_eu_i_dn13, locals.var_eu_i_dn14,)
    } else {
        (locals.var_eu_t, locals.var_eu_t_dn0, locals.var_eu_t_dn2, locals.var_eu_t_dn3, locals.var_eu_t_dn4, locals.var_eu_t_dn5, locals.var_eu_t_dn6, locals.var_eu_t_dn7, locals.var_eu_t_dn8, locals.var_eu_t_dn9, locals.var_eu_t_dn10, locals.var_eu_t_dn11, locals.var_eu_t_dn13, locals.var_eu_t_dn14,)
    }
};
        locals.var_eu_t = assign14830_e22104;
        locals.var_eu_t_dn0 = assign14830_e22104_d_n0;
        locals.var_eu_t_dn2 = assign14830_e22104_d_n2;
        locals.var_eu_t_dn3 = assign14830_e22104_d_n3;
        locals.var_eu_t_dn4 = assign14830_e22104_d_n4;
        locals.var_eu_t_dn5 = assign14830_e22104_d_n5;
        locals.var_eu_t_dn6 = assign14830_e22104_d_n6;
        locals.var_eu_t_dn7 = assign14830_e22104_d_n7;
        locals.var_eu_t_dn8 = assign14830_e22104_d_n8;
        locals.var_eu_t_dn9 = assign14830_e22104_d_n9;
        locals.var_eu_t_dn10 = assign14830_e22104_d_n10;
        locals.var_eu_t_dn11 = assign14830_e22104_d_n11;
        locals.var_eu_t_dn13 = assign14830_e22104_d_n13;
        locals.var_eu_t_dn14 = assign14830_e22104_d_n14;

        let assign14840_e22107: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard246 = assign14840_e22107;

        let (assign14850_e22153, assign14850_e22153_d_n0, assign14850_e22153_d_n2, assign14850_e22153_d_n3, assign14850_e22153_d_n4, assign14850_e22153_d_n5, assign14850_e22153_d_n6, assign14850_e22153_d_n7, assign14850_e22153_d_n8, assign14850_e22153_d_n9, assign14850_e22153_d_n10, assign14850_e22153_d_n11, assign14850_e22153_d_n13, assign14850_e22153_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard246 != 0.0)) {
        let assign14850_e22113: f64 = (-locals.var_uar_i);
        let assign14850_e22117: f64 = (locals.var_ua1r_i * locals.var_deltemp);
        let assign14850_e22119: f64 = (-locals.var_uar_i);
        let assign14850_e22120: f64 = (assign14850_e22117 - assign14850_e22119);
        let assign14850_e22122: f64 = (assign14850_e22120 - 1e-6);
        let assign14850_e22125: f64 = (locals.var_ua1r_i * locals.var_deltemp);
        let assign14850_e22127: f64 = (-locals.var_uar_i);
        let assign14850_e22128: f64 = (assign14850_e22125 - assign14850_e22127);
        let assign14850_e22130: f64 = (assign14850_e22128 - 1e-6);
        let assign14850_e22133: f64 = (locals.var_ua1r_i * locals.var_deltemp);
        let assign14850_e22135: f64 = (-locals.var_uar_i);
        let assign14850_e22136: f64 = (assign14850_e22133 - assign14850_e22135);
        let assign14850_e22138: f64 = (assign14850_e22136 - 1e-6);
        let assign14850_e22139: f64 = (assign14850_e22130 * assign14850_e22138);
        let assign14850_e22142: f64 = (-locals.var_uar_i);
        let assign14850_e22143: f64 = (4.0 * assign14850_e22142);
        let assign14850_e22145: f64 = (assign14850_e22143 * 1e-6);
        let assign14850_e22146: f64 = (assign14850_e22139 - assign14850_e22145);
        let assign14850_e22147: f64 = (assign14850_e22146).sqrt();
        let assign14850_e22148: f64 = (assign14850_e22122 + assign14850_e22147);
        let assign14850_e22149: f64 = (0.5 * assign14850_e22148);
        let assign14850_e22150: f64 = (assign14850_e22113 + assign14850_e22149);
        let assign14850_e22151: f64 = (locals.var_uar_i + assign14850_e22150);
        (assign14850_e22151, (locals.var_uar_i_dn0 + ((-locals.var_uar_i_dn0) + (0.5 * ((-(-locals.var_uar_i_dn0)) + (((((-(-locals.var_uar_i_dn0)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn0)))) - ((4.0 * (-locals.var_uar_i_dn0)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn2 + ((-locals.var_uar_i_dn2) + (0.5 * ((-(-locals.var_uar_i_dn2)) + (((((-(-locals.var_uar_i_dn2)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn2)))) - ((4.0 * (-locals.var_uar_i_dn2)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn3 + ((-locals.var_uar_i_dn3) + (0.5 * ((-(-locals.var_uar_i_dn3)) + (((((-(-locals.var_uar_i_dn3)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn3)))) - ((4.0 * (-locals.var_uar_i_dn3)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn4 + ((-locals.var_uar_i_dn4) + (0.5 * (((locals.var_ua1r_i * locals.var_deltemp_dn4) - (-locals.var_uar_i_dn4)) + ((((((locals.var_ua1r_i * locals.var_deltemp_dn4) - (-locals.var_uar_i_dn4)) * assign14850_e22138) + (assign14850_e22130 * ((locals.var_ua1r_i * locals.var_deltemp_dn4) - (-locals.var_uar_i_dn4)))) - ((4.0 * (-locals.var_uar_i_dn4)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn5 + ((-locals.var_uar_i_dn5) + (0.5 * ((-(-locals.var_uar_i_dn5)) + (((((-(-locals.var_uar_i_dn5)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn5)))) - ((4.0 * (-locals.var_uar_i_dn5)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn6 + ((-locals.var_uar_i_dn6) + (0.5 * ((-(-locals.var_uar_i_dn6)) + (((((-(-locals.var_uar_i_dn6)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn6)))) - ((4.0 * (-locals.var_uar_i_dn6)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn7 + ((-locals.var_uar_i_dn7) + (0.5 * ((-(-locals.var_uar_i_dn7)) + (((((-(-locals.var_uar_i_dn7)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn7)))) - ((4.0 * (-locals.var_uar_i_dn7)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn8 + ((-locals.var_uar_i_dn8) + (0.5 * ((-(-locals.var_uar_i_dn8)) + (((((-(-locals.var_uar_i_dn8)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn8)))) - ((4.0 * (-locals.var_uar_i_dn8)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn9 + ((-locals.var_uar_i_dn9) + (0.5 * ((-(-locals.var_uar_i_dn9)) + (((((-(-locals.var_uar_i_dn9)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn9)))) - ((4.0 * (-locals.var_uar_i_dn9)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn10 + ((-locals.var_uar_i_dn10) + (0.5 * ((-(-locals.var_uar_i_dn10)) + (((((-(-locals.var_uar_i_dn10)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn10)))) - ((4.0 * (-locals.var_uar_i_dn10)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn11 + ((-locals.var_uar_i_dn11) + (0.5 * ((-(-locals.var_uar_i_dn11)) + (((((-(-locals.var_uar_i_dn11)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn11)))) - ((4.0 * (-locals.var_uar_i_dn11)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn13 + ((-locals.var_uar_i_dn13) + (0.5 * ((-(-locals.var_uar_i_dn13)) + (((((-(-locals.var_uar_i_dn13)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn13)))) - ((4.0 * (-locals.var_uar_i_dn13)) * 1e-6)) / (2.0 * assign14850_e22147)))))), (locals.var_uar_i_dn14 + ((-locals.var_uar_i_dn14) + (0.5 * ((-(-locals.var_uar_i_dn14)) + (((((-(-locals.var_uar_i_dn14)) * assign14850_e22138) + (assign14850_e22130 * (-(-locals.var_uar_i_dn14)))) - ((4.0 * (-locals.var_uar_i_dn14)) * 1e-6)) / (2.0 * assign14850_e22147)))))),)
    } else {
        (locals.var_uar_t, locals.var_uar_t_dn0, locals.var_uar_t_dn2, locals.var_uar_t_dn3, locals.var_uar_t_dn4, locals.var_uar_t_dn5, locals.var_uar_t_dn6, locals.var_uar_t_dn7, locals.var_uar_t_dn8, locals.var_uar_t_dn9, locals.var_uar_t_dn10, locals.var_uar_t_dn11, locals.var_uar_t_dn13, locals.var_uar_t_dn14,)
    }
};
        locals.var_uar_t = assign14850_e22153;
        locals.var_uar_t_dn0 = assign14850_e22153_d_n0;
        locals.var_uar_t_dn2 = assign14850_e22153_d_n2;
        locals.var_uar_t_dn3 = assign14850_e22153_d_n3;
        locals.var_uar_t_dn4 = assign14850_e22153_d_n4;
        locals.var_uar_t_dn5 = assign14850_e22153_d_n5;
        locals.var_uar_t_dn6 = assign14850_e22153_d_n6;
        locals.var_uar_t_dn7 = assign14850_e22153_d_n7;
        locals.var_uar_t_dn8 = assign14850_e22153_d_n8;
        locals.var_uar_t_dn9 = assign14850_e22153_d_n9;
        locals.var_uar_t_dn10 = assign14850_e22153_d_n10;
        locals.var_uar_t_dn11 = assign14850_e22153_d_n11;
        locals.var_uar_t_dn13 = assign14850_e22153_d_n13;
        locals.var_uar_t_dn14 = assign14850_e22153_d_n14;

    }

    pub(super) fn stamp_transient_block_42(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign14860_e22162, assign14860_e22162_d_n0, assign14860_e22162_d_n2, assign14860_e22162_d_n3, assign14860_e22162_d_n4, assign14860_e22162_d_n5, assign14860_e22162_d_n6, assign14860_e22162_d_n7, assign14860_e22162_d_n8, assign14860_e22162_d_n9, assign14860_e22162_d_n10, assign14860_e22162_d_n11, assign14860_e22162_d_n13, assign14860_e22162_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        let assign14860_e22158: f64 = (locals.var_ud1_i * locals.var_trat_ln);
        let assign14860_e22159: f64 = (assign14860_e22158).exp();
        let assign14860_e22160: f64 = (locals.var_ud_i * assign14860_e22159);
        (assign14860_e22160, (locals.var_ud_i_dn0 * assign14860_e22159), (locals.var_ud_i_dn2 * assign14860_e22159), (locals.var_ud_i_dn3 * assign14860_e22159), ((locals.var_ud_i_dn4 * assign14860_e22159) + (locals.var_ud_i * (assign14860_e22159 * (locals.var_ud1_i * locals.var_trat_ln_dn4)))), (locals.var_ud_i_dn5 * assign14860_e22159), (locals.var_ud_i_dn6 * assign14860_e22159), (locals.var_ud_i_dn7 * assign14860_e22159), (locals.var_ud_i_dn8 * assign14860_e22159), (locals.var_ud_i_dn9 * assign14860_e22159), (locals.var_ud_i_dn10 * assign14860_e22159), (locals.var_ud_i_dn11 * assign14860_e22159), (locals.var_ud_i_dn13 * assign14860_e22159), (locals.var_ud_i_dn14 * assign14860_e22159),)
    } else {
        (locals.var_ud_t, locals.var_ud_t_dn0, locals.var_ud_t_dn2, locals.var_ud_t_dn3, locals.var_ud_t_dn4, locals.var_ud_t_dn5, locals.var_ud_t_dn6, locals.var_ud_t_dn7, locals.var_ud_t_dn8, locals.var_ud_t_dn9, locals.var_ud_t_dn10, locals.var_ud_t_dn11, locals.var_ud_t_dn13, locals.var_ud_t_dn14,)
    }
};
        locals.var_ud_t = assign14860_e22162;
        locals.var_ud_t_dn0 = assign14860_e22162_d_n0;
        locals.var_ud_t_dn2 = assign14860_e22162_d_n2;
        locals.var_ud_t_dn3 = assign14860_e22162_d_n3;
        locals.var_ud_t_dn4 = assign14860_e22162_d_n4;
        locals.var_ud_t_dn5 = assign14860_e22162_d_n5;
        locals.var_ud_t_dn6 = assign14860_e22162_d_n6;
        locals.var_ud_t_dn7 = assign14860_e22162_d_n7;
        locals.var_ud_t_dn8 = assign14860_e22162_d_n8;
        locals.var_ud_t_dn9 = assign14860_e22162_d_n9;
        locals.var_ud_t_dn10 = assign14860_e22162_d_n10;
        locals.var_ud_t_dn11 = assign14860_e22162_d_n11;
        locals.var_ud_t_dn13 = assign14860_e22162_d_n13;
        locals.var_ud_t_dn14 = assign14860_e22162_d_n14;

        let assign14870_e22165: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard247 = assign14870_e22165;

        let (assign14880_e22176, assign14880_e22176_d_n0, assign14880_e22176_d_n2, assign14880_e22176_d_n3, assign14880_e22176_d_n4, assign14880_e22176_d_n5, assign14880_e22176_d_n6, assign14880_e22176_d_n7, assign14880_e22176_d_n8, assign14880_e22176_d_n9, assign14880_e22176_d_n10, assign14880_e22176_d_n11, assign14880_e22176_d_n13, assign14880_e22176_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard247 != 0.0)) {
        let assign14880_e22172: f64 = (locals.var_ud1r_i * locals.var_trat_ln);
        let assign14880_e22173: f64 = (assign14880_e22172).exp();
        let assign14880_e22174: f64 = (locals.var_udr_i * assign14880_e22173);
        (assign14880_e22174, (locals.var_udr_i_dn0 * assign14880_e22173), (locals.var_udr_i_dn2 * assign14880_e22173), (locals.var_udr_i_dn3 * assign14880_e22173), ((locals.var_udr_i_dn4 * assign14880_e22173) + (locals.var_udr_i * (assign14880_e22173 * (locals.var_ud1r_i * locals.var_trat_ln_dn4)))), (locals.var_udr_i_dn5 * assign14880_e22173), (locals.var_udr_i_dn6 * assign14880_e22173), (locals.var_udr_i_dn7 * assign14880_e22173), (locals.var_udr_i_dn8 * assign14880_e22173), (locals.var_udr_i_dn9 * assign14880_e22173), (locals.var_udr_i_dn10 * assign14880_e22173), (locals.var_udr_i_dn11 * assign14880_e22173), (locals.var_udr_i_dn13 * assign14880_e22173), (locals.var_udr_i_dn14 * assign14880_e22173),)
    } else {
        (locals.var_udr_t, locals.var_udr_t_dn0, locals.var_udr_t_dn2, locals.var_udr_t_dn3, locals.var_udr_t_dn4, locals.var_udr_t_dn5, locals.var_udr_t_dn6, locals.var_udr_t_dn7, locals.var_udr_t_dn8, locals.var_udr_t_dn9, locals.var_udr_t_dn10, locals.var_udr_t_dn11, locals.var_udr_t_dn13, locals.var_udr_t_dn14,)
    }
};
        locals.var_udr_t = assign14880_e22176;
        locals.var_udr_t_dn0 = assign14880_e22176_d_n0;
        locals.var_udr_t_dn2 = assign14880_e22176_d_n2;
        locals.var_udr_t_dn3 = assign14880_e22176_d_n3;
        locals.var_udr_t_dn4 = assign14880_e22176_d_n4;
        locals.var_udr_t_dn5 = assign14880_e22176_d_n5;
        locals.var_udr_t_dn6 = assign14880_e22176_d_n6;
        locals.var_udr_t_dn7 = assign14880_e22176_d_n7;
        locals.var_udr_t_dn8 = assign14880_e22176_d_n8;
        locals.var_udr_t_dn9 = assign14880_e22176_d_n9;
        locals.var_udr_t_dn10 = assign14880_e22176_d_n10;
        locals.var_udr_t_dn11 = assign14880_e22176_d_n11;
        locals.var_udr_t_dn13 = assign14880_e22176_d_n13;
        locals.var_udr_t_dn14 = assign14880_e22176_d_n14;

        let (assign14890_e22185, assign14890_e22185_d_n4,) = {
    if (locals.var_guard244 != 0.0) {
        let assign14890_e22181: f64 = (locals.var_ucste_i * locals.var_trat_ln);
        let assign14890_e22182: f64 = (assign14890_e22181).exp();
        let assign14890_e22183: f64 = (locals.var_ucs_i * assign14890_e22182);
        (assign14890_e22183, (locals.var_ucs_i * (assign14890_e22182 * (locals.var_ucste_i * locals.var_trat_ln_dn4))),)
    } else {
        (locals.var_ucs_t, locals.var_ucs_t_dn4,)
    }
};
        locals.var_ucs_t = assign14890_e22185;
        locals.var_ucs_t_dn4 = assign14890_e22185_d_n4;

        let (assign14900_e22260, assign14900_e22260_d_n0, assign14900_e22260_d_n2, assign14900_e22260_d_n3, assign14900_e22260_d_n4, assign14900_e22260_d_n5, assign14900_e22260_d_n6, assign14900_e22260_d_n7, assign14900_e22260_d_n8, assign14900_e22260_d_n9, assign14900_e22260_d_n10, assign14900_e22260_d_n11, assign14900_e22260_d_n13, assign14900_e22260_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        let assign14900_e22190: f64 = (locals.var_prt_i * locals.var_deltemp);
        let assign14900_e22191: f64 = (1.0 + assign14900_e22190);
        let assign14900_e22193: f64 = (assign14900_e22191 - 1e-6);
        let assign14900_e22195: f64 = (-10000.0);
        let assign14900_e22197: f64 = (assign14900_e22195 * 0.001);
        let (assign14900_e22258, assign14900_e22258_d_n4,) = {
            if (!(assign14900_e22193 < assign14900_e22197)) {
                let assign14900_e22204: f64 = (locals.var_prt_i * locals.var_deltemp);
                let assign14900_e22205: f64 = (1.0 + assign14900_e22204);
                let assign14900_e22207: f64 = (assign14900_e22205 - 1e-6);
                let assign14900_e22211: f64 = (locals.var_prt_i * locals.var_deltemp);
                let assign14900_e22212: f64 = (1.0 + assign14900_e22211);
                let assign14900_e22214: f64 = (assign14900_e22212 - 1e-6);
                let assign14900_e22218: f64 = (locals.var_prt_i * locals.var_deltemp);
                let assign14900_e22219: f64 = (1.0 + assign14900_e22218);
                let assign14900_e22221: f64 = (assign14900_e22219 - 1e-6);
                let assign14900_e22222: f64 = (assign14900_e22214 * assign14900_e22221);
                let assign14900_e22225: f64 = (4.0 * 0.001);
                let assign14900_e22227: f64 = (assign14900_e22225 * 0.001);
                let assign14900_e22228: f64 = (assign14900_e22222 + assign14900_e22227);
                let assign14900_e22229: f64 = (assign14900_e22228).sqrt();
                let assign14900_e22230: f64 = (assign14900_e22207 + assign14900_e22229);
                let assign14900_e22231: f64 = (0.5 * assign14900_e22230);
                (assign14900_e22231, (0.5 * ((locals.var_prt_i * locals.var_deltemp_dn4) + ((((locals.var_prt_i * locals.var_deltemp_dn4) * assign14900_e22221) + (assign14900_e22214 * (locals.var_prt_i * locals.var_deltemp_dn4))) / (2.0 * assign14900_e22229)))),)
            } else {
                let assign14900_e22235: f64 = (locals.var_prt_i * locals.var_deltemp);
                let assign14900_e22236: f64 = (1.0 + assign14900_e22235);
                let assign14900_e22238: f64 = (assign14900_e22236 - 1e-6);
                let assign14900_e22240: f64 = (-10000.0);
                let assign14900_e22242: f64 = (assign14900_e22240 * 0.001);
                let (assign14900_e22257, assign14900_e22257_d_n4,) = {
                    if (assign14900_e22238 < assign14900_e22242) {
                        let assign14900_e22245: f64 = (-0.001);
                        let assign14900_e22247: f64 = (assign14900_e22245 * 0.001);
                        let assign14900_e22251: f64 = (locals.var_prt_i * locals.var_deltemp);
                        let assign14900_e22252: f64 = (1.0 + assign14900_e22251);
                        let assign14900_e22254: f64 = (assign14900_e22252 - 1e-6);
                        let assign14900_e22255: f64 = (assign14900_e22247 / assign14900_e22254);
                        (assign14900_e22255, (-((assign14900_e22247 * (locals.var_prt_i * locals.var_deltemp_dn4)) / (assign14900_e22254 * assign14900_e22254))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign14900_e22257, assign14900_e22257_d_n4,)
            }
        };
        (assign14900_e22258, 0.0, 0.0, 0.0, assign14900_e22258_d_n4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_rdstemp, locals.var_rdstemp_dn0, locals.var_rdstemp_dn2, locals.var_rdstemp_dn3, locals.var_rdstemp_dn4, locals.var_rdstemp_dn5, locals.var_rdstemp_dn6, locals.var_rdstemp_dn7, locals.var_rdstemp_dn8, locals.var_rdstemp_dn9, locals.var_rdstemp_dn10, locals.var_rdstemp_dn11, locals.var_rdstemp_dn13, locals.var_rdstemp_dn14,)
    }
};
        locals.var_rdstemp = assign14900_e22260;
        locals.var_rdstemp_dn0 = assign14900_e22260_d_n0;
        locals.var_rdstemp_dn2 = assign14900_e22260_d_n2;
        locals.var_rdstemp_dn3 = assign14900_e22260_d_n3;
        locals.var_rdstemp_dn4 = assign14900_e22260_d_n4;
        locals.var_rdstemp_dn5 = assign14900_e22260_d_n5;
        locals.var_rdstemp_dn6 = assign14900_e22260_d_n6;
        locals.var_rdstemp_dn7 = assign14900_e22260_d_n7;
        locals.var_rdstemp_dn8 = assign14900_e22260_d_n8;
        locals.var_rdstemp_dn9 = assign14900_e22260_d_n9;
        locals.var_rdstemp_dn10 = assign14900_e22260_d_n10;
        locals.var_rdstemp_dn11 = assign14900_e22260_d_n11;
        locals.var_rdstemp_dn13 = assign14900_e22260_d_n13;
        locals.var_rdstemp_dn14 = assign14900_e22260_d_n14;

        let assign14910_e22263: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard248 = assign14910_e22263;

        let (assign14920_e22312, assign14920_e22312_d_n0, assign14920_e22312_d_n2, assign14920_e22312_d_n3, assign14920_e22312_d_n4, assign14920_e22312_d_n5, assign14920_e22312_d_n6, assign14920_e22312_d_n7, assign14920_e22312_d_n8, assign14920_e22312_d_n9, assign14920_e22312_d_n10, assign14920_e22312_d_n11, assign14920_e22312_d_n13, assign14920_e22312_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard248 != 0.0)) {
        let assign14920_e22269: f64 = (-locals.var_vsat_i);
        let assign14920_e22272: f64 = (-locals.var_at_i);
        let assign14920_e22274: f64 = (assign14920_e22272 * locals.var_deltemp);
        let assign14920_e22276: f64 = (-locals.var_vsat_i);
        let assign14920_e22277: f64 = (assign14920_e22274 - assign14920_e22276);
        let assign14920_e22279: f64 = (assign14920_e22277 - 1e-6);
        let assign14920_e22281: f64 = (-locals.var_at_i);
        let assign14920_e22283: f64 = (assign14920_e22281 * locals.var_deltemp);
        let assign14920_e22285: f64 = (-locals.var_vsat_i);
        let assign14920_e22286: f64 = (assign14920_e22283 - assign14920_e22285);
        let assign14920_e22288: f64 = (assign14920_e22286 - 1e-6);
        let assign14920_e22290: f64 = (-locals.var_at_i);
        let assign14920_e22292: f64 = (assign14920_e22290 * locals.var_deltemp);
        let assign14920_e22294: f64 = (-locals.var_vsat_i);
        let assign14920_e22295: f64 = (assign14920_e22292 - assign14920_e22294);
        let assign14920_e22297: f64 = (assign14920_e22295 - 1e-6);
        let assign14920_e22298: f64 = (assign14920_e22288 * assign14920_e22297);
        let assign14920_e22301: f64 = (-locals.var_vsat_i);
        let assign14920_e22302: f64 = (4.0 * assign14920_e22301);
        let assign14920_e22304: f64 = (assign14920_e22302 * 1e-6);
        let assign14920_e22305: f64 = (assign14920_e22298 - assign14920_e22304);
        let assign14920_e22306: f64 = (assign14920_e22305).sqrt();
        let assign14920_e22307: f64 = (assign14920_e22279 + assign14920_e22306);
        let assign14920_e22308: f64 = (0.5 * assign14920_e22307);
        let assign14920_e22309: f64 = (assign14920_e22269 + assign14920_e22308);
        let assign14920_e22310: f64 = (locals.var_vsat_i + assign14920_e22309);
        (assign14920_e22310, (locals.var_vsat_i_dn0 + ((-locals.var_vsat_i_dn0) + (0.5 * ((-(-locals.var_vsat_i_dn0)) + (((((-(-locals.var_vsat_i_dn0)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn0)))) - ((4.0 * (-locals.var_vsat_i_dn0)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn2 + ((-locals.var_vsat_i_dn2) + (0.5 * ((-(-locals.var_vsat_i_dn2)) + (((((-(-locals.var_vsat_i_dn2)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn2)))) - ((4.0 * (-locals.var_vsat_i_dn2)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn3 + ((-locals.var_vsat_i_dn3) + (0.5 * ((-(-locals.var_vsat_i_dn3)) + (((((-(-locals.var_vsat_i_dn3)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn3)))) - ((4.0 * (-locals.var_vsat_i_dn3)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn4 + ((-locals.var_vsat_i_dn4) + (0.5 * (((assign14920_e22272 * locals.var_deltemp_dn4) - (-locals.var_vsat_i_dn4)) + ((((((assign14920_e22281 * locals.var_deltemp_dn4) - (-locals.var_vsat_i_dn4)) * assign14920_e22297) + (assign14920_e22288 * ((assign14920_e22290 * locals.var_deltemp_dn4) - (-locals.var_vsat_i_dn4)))) - ((4.0 * (-locals.var_vsat_i_dn4)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn5 + ((-locals.var_vsat_i_dn5) + (0.5 * ((-(-locals.var_vsat_i_dn5)) + (((((-(-locals.var_vsat_i_dn5)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn5)))) - ((4.0 * (-locals.var_vsat_i_dn5)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn6 + ((-locals.var_vsat_i_dn6) + (0.5 * ((-(-locals.var_vsat_i_dn6)) + (((((-(-locals.var_vsat_i_dn6)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn6)))) - ((4.0 * (-locals.var_vsat_i_dn6)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn7 + ((-locals.var_vsat_i_dn7) + (0.5 * ((-(-locals.var_vsat_i_dn7)) + (((((-(-locals.var_vsat_i_dn7)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn7)))) - ((4.0 * (-locals.var_vsat_i_dn7)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn8 + ((-locals.var_vsat_i_dn8) + (0.5 * ((-(-locals.var_vsat_i_dn8)) + (((((-(-locals.var_vsat_i_dn8)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn8)))) - ((4.0 * (-locals.var_vsat_i_dn8)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn9 + ((-locals.var_vsat_i_dn9) + (0.5 * ((-(-locals.var_vsat_i_dn9)) + (((((-(-locals.var_vsat_i_dn9)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn9)))) - ((4.0 * (-locals.var_vsat_i_dn9)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn10 + ((-locals.var_vsat_i_dn10) + (0.5 * ((-(-locals.var_vsat_i_dn10)) + (((((-(-locals.var_vsat_i_dn10)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn10)))) - ((4.0 * (-locals.var_vsat_i_dn10)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn11 + ((-locals.var_vsat_i_dn11) + (0.5 * ((-(-locals.var_vsat_i_dn11)) + (((((-(-locals.var_vsat_i_dn11)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn11)))) - ((4.0 * (-locals.var_vsat_i_dn11)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn13 + ((-locals.var_vsat_i_dn13) + (0.5 * ((-(-locals.var_vsat_i_dn13)) + (((((-(-locals.var_vsat_i_dn13)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn13)))) - ((4.0 * (-locals.var_vsat_i_dn13)) * 1e-6)) / (2.0 * assign14920_e22306)))))), (locals.var_vsat_i_dn14 + ((-locals.var_vsat_i_dn14) + (0.5 * ((-(-locals.var_vsat_i_dn14)) + (((((-(-locals.var_vsat_i_dn14)) * assign14920_e22297) + (assign14920_e22288 * (-(-locals.var_vsat_i_dn14)))) - ((4.0 * (-locals.var_vsat_i_dn14)) * 1e-6)) / (2.0 * assign14920_e22306)))))),)
    } else {
        (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14,)
    }
};
        locals.var_vsat_t = assign14920_e22312;
        locals.var_vsat_t_dn0 = assign14920_e22312_d_n0;
        locals.var_vsat_t_dn2 = assign14920_e22312_d_n2;
        locals.var_vsat_t_dn3 = assign14920_e22312_d_n3;
        locals.var_vsat_t_dn4 = assign14920_e22312_d_n4;
        locals.var_vsat_t_dn5 = assign14920_e22312_d_n5;
        locals.var_vsat_t_dn6 = assign14920_e22312_d_n6;
        locals.var_vsat_t_dn7 = assign14920_e22312_d_n7;
        locals.var_vsat_t_dn8 = assign14920_e22312_d_n8;
        locals.var_vsat_t_dn9 = assign14920_e22312_d_n9;
        locals.var_vsat_t_dn10 = assign14920_e22312_d_n10;
        locals.var_vsat_t_dn11 = assign14920_e22312_d_n11;
        locals.var_vsat_t_dn13 = assign14920_e22312_d_n13;
        locals.var_vsat_t_dn14 = assign14920_e22312_d_n14;

        let (assign14930_e22398, assign14930_e22398_d_n0, assign14930_e22398_d_n2, assign14930_e22398_d_n3, assign14930_e22398_d_n4, assign14930_e22398_d_n5, assign14930_e22398_d_n6, assign14930_e22398_d_n7, assign14930_e22398_d_n8, assign14930_e22398_d_n9, assign14930_e22398_d_n10, assign14930_e22398_d_n11, assign14930_e22398_d_n13, assign14930_e22398_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard248 == 0.0)) {
        let assign14930_e22320: f64 = (-locals.var_at_i);
        let assign14930_e22322: f64 = (assign14930_e22320 * locals.var_deltemp);
        let assign14930_e22323: f64 = (1.0 + assign14930_e22322);
        let assign14930_e22325: f64 = (assign14930_e22323 - 1e-6);
        let assign14930_e22327: f64 = (-10000.0);
        let assign14930_e22329: f64 = (assign14930_e22327 * 0.001);
        let (assign14930_e22395, assign14930_e22395_d_n4,) = {
            if (!(assign14930_e22325 < assign14930_e22329)) {
                let assign14930_e22335: f64 = (-locals.var_at_i);
                let assign14930_e22337: f64 = (assign14930_e22335 * locals.var_deltemp);
                let assign14930_e22338: f64 = (1.0 + assign14930_e22337);
                let assign14930_e22340: f64 = (assign14930_e22338 - 1e-6);
                let assign14930_e22343: f64 = (-locals.var_at_i);
                let assign14930_e22345: f64 = (assign14930_e22343 * locals.var_deltemp);
                let assign14930_e22346: f64 = (1.0 + assign14930_e22345);
                let assign14930_e22348: f64 = (assign14930_e22346 - 1e-6);
                let assign14930_e22351: f64 = (-locals.var_at_i);
                let assign14930_e22353: f64 = (assign14930_e22351 * locals.var_deltemp);
                let assign14930_e22354: f64 = (1.0 + assign14930_e22353);
                let assign14930_e22356: f64 = (assign14930_e22354 - 1e-6);
                let assign14930_e22357: f64 = (assign14930_e22348 * assign14930_e22356);
                let assign14930_e22360: f64 = (4.0 * 0.001);
                let assign14930_e22362: f64 = (assign14930_e22360 * 0.001);
                let assign14930_e22363: f64 = (assign14930_e22357 + assign14930_e22362);
                let assign14930_e22364: f64 = (assign14930_e22363).sqrt();
                let assign14930_e22365: f64 = (assign14930_e22340 + assign14930_e22364);
                let assign14930_e22366: f64 = (0.5 * assign14930_e22365);
                (assign14930_e22366, (0.5 * ((assign14930_e22335 * locals.var_deltemp_dn4) + ((((assign14930_e22343 * locals.var_deltemp_dn4) * assign14930_e22356) + (assign14930_e22348 * (assign14930_e22351 * locals.var_deltemp_dn4))) / (2.0 * assign14930_e22364)))),)
            } else {
                let assign14930_e22369: f64 = (-locals.var_at_i);
                let assign14930_e22371: f64 = (assign14930_e22369 * locals.var_deltemp);
                let assign14930_e22372: f64 = (1.0 + assign14930_e22371);
                let assign14930_e22374: f64 = (assign14930_e22372 - 1e-6);
                let assign14930_e22376: f64 = (-10000.0);
                let assign14930_e22378: f64 = (assign14930_e22376 * 0.001);
                let (assign14930_e22394, assign14930_e22394_d_n4,) = {
                    if (assign14930_e22374 < assign14930_e22378) {
                        let assign14930_e22381: f64 = (-0.001);
                        let assign14930_e22383: f64 = (assign14930_e22381 * 0.001);
                        let assign14930_e22386: f64 = (-locals.var_at_i);
                        let assign14930_e22388: f64 = (assign14930_e22386 * locals.var_deltemp);
                        let assign14930_e22389: f64 = (1.0 + assign14930_e22388);
                        let assign14930_e22391: f64 = (assign14930_e22389 - 1e-6);
                        let assign14930_e22392: f64 = (assign14930_e22383 / assign14930_e22391);
                        (assign14930_e22392, (-((assign14930_e22383 * (assign14930_e22386 * locals.var_deltemp_dn4)) / (assign14930_e22391 * assign14930_e22391))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign14930_e22394, assign14930_e22394_d_n4,)
            }
        };
        let assign14930_e22396: f64 = (locals.var_vsat_i * assign14930_e22395);
        (assign14930_e22396, (locals.var_vsat_i_dn0 * assign14930_e22395), (locals.var_vsat_i_dn2 * assign14930_e22395), (locals.var_vsat_i_dn3 * assign14930_e22395), ((locals.var_vsat_i_dn4 * assign14930_e22395) + (locals.var_vsat_i * assign14930_e22395_d_n4)), (locals.var_vsat_i_dn5 * assign14930_e22395), (locals.var_vsat_i_dn6 * assign14930_e22395), (locals.var_vsat_i_dn7 * assign14930_e22395), (locals.var_vsat_i_dn8 * assign14930_e22395), (locals.var_vsat_i_dn9 * assign14930_e22395), (locals.var_vsat_i_dn10 * assign14930_e22395), (locals.var_vsat_i_dn11 * assign14930_e22395), (locals.var_vsat_i_dn13 * assign14930_e22395), (locals.var_vsat_i_dn14 * assign14930_e22395),)
    } else {
        (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14,)
    }
};
        locals.var_vsat_t = assign14930_e22398;
        locals.var_vsat_t_dn0 = assign14930_e22398_d_n0;
        locals.var_vsat_t_dn2 = assign14930_e22398_d_n2;
        locals.var_vsat_t_dn3 = assign14930_e22398_d_n3;
        locals.var_vsat_t_dn4 = assign14930_e22398_d_n4;
        locals.var_vsat_t_dn5 = assign14930_e22398_d_n5;
        locals.var_vsat_t_dn6 = assign14930_e22398_d_n6;
        locals.var_vsat_t_dn7 = assign14930_e22398_d_n7;
        locals.var_vsat_t_dn8 = assign14930_e22398_d_n8;
        locals.var_vsat_t_dn9 = assign14930_e22398_d_n9;
        locals.var_vsat_t_dn10 = assign14930_e22398_d_n10;
        locals.var_vsat_t_dn11 = assign14930_e22398_d_n11;
        locals.var_vsat_t_dn13 = assign14930_e22398_d_n13;
        locals.var_vsat_t_dn14 = assign14930_e22398_d_n14;

        let assign14940_e22401: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard249 = assign14940_e22401;

        let assign14950_e22404: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard250 = assign14950_e22404;

        let (assign14960_e22455, assign14960_e22455_d_n4,) = {
    if (((locals.var_guard244 != 0.0) && (locals.var_guard249 != 0.0)) && (locals.var_guard250 != 0.0)) {
        let assign14960_e22412: f64 = (-locals.var_vsatr_i);
        let assign14960_e22415: f64 = (-locals.var_atr_i);
        let assign14960_e22417: f64 = (assign14960_e22415 * locals.var_deltemp);
        let assign14960_e22419: f64 = (-locals.var_vsatr_i);
        let assign14960_e22420: f64 = (assign14960_e22417 - assign14960_e22419);
        let assign14960_e22422: f64 = (assign14960_e22420 - 1e-6);
        let assign14960_e22424: f64 = (-locals.var_atr_i);
        let assign14960_e22426: f64 = (assign14960_e22424 * locals.var_deltemp);
        let assign14960_e22428: f64 = (-locals.var_vsatr_i);
        let assign14960_e22429: f64 = (assign14960_e22426 - assign14960_e22428);
        let assign14960_e22431: f64 = (assign14960_e22429 - 1e-6);
        let assign14960_e22433: f64 = (-locals.var_atr_i);
        let assign14960_e22435: f64 = (assign14960_e22433 * locals.var_deltemp);
        let assign14960_e22437: f64 = (-locals.var_vsatr_i);
        let assign14960_e22438: f64 = (assign14960_e22435 - assign14960_e22437);
        let assign14960_e22440: f64 = (assign14960_e22438 - 1e-6);
        let assign14960_e22441: f64 = (assign14960_e22431 * assign14960_e22440);
        let assign14960_e22444: f64 = (-locals.var_vsatr_i);
        let assign14960_e22445: f64 = (4.0 * assign14960_e22444);
        let assign14960_e22447: f64 = (assign14960_e22445 * 1e-6);
        let assign14960_e22448: f64 = (assign14960_e22441 - assign14960_e22447);
        let assign14960_e22449: f64 = (assign14960_e22448).sqrt();
        let assign14960_e22450: f64 = (assign14960_e22422 + assign14960_e22449);
        let assign14960_e22451: f64 = (0.5 * assign14960_e22450);
        let assign14960_e22452: f64 = (assign14960_e22412 + assign14960_e22451);
        let assign14960_e22453: f64 = (locals.var_vsatr_i + assign14960_e22452);
        (assign14960_e22453, (0.5 * ((assign14960_e22415 * locals.var_deltemp_dn4) + ((((assign14960_e22424 * locals.var_deltemp_dn4) * assign14960_e22440) + (assign14960_e22431 * (assign14960_e22433 * locals.var_deltemp_dn4))) / (2.0 * assign14960_e22449)))),)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn4,)
    }
};
        locals.var_vsatr_t = assign14960_e22455;
        locals.var_vsatr_t_dn4 = assign14960_e22455_d_n4;

        let (assign14970_e22543, assign14970_e22543_d_n4,) = {
    if (((locals.var_guard244 != 0.0) && (locals.var_guard249 != 0.0)) && (locals.var_guard250 == 0.0)) {
        let assign14970_e22465: f64 = (-locals.var_atr_i);
        let assign14970_e22467: f64 = (assign14970_e22465 * locals.var_deltemp);
        let assign14970_e22468: f64 = (1.0 + assign14970_e22467);
        let assign14970_e22470: f64 = (assign14970_e22468 - 1e-6);
        let assign14970_e22472: f64 = (-10000.0);
        let assign14970_e22474: f64 = (assign14970_e22472 * 0.001);
        let (assign14970_e22540, assign14970_e22540_d_n4,) = {
            if (!(assign14970_e22470 < assign14970_e22474)) {
                let assign14970_e22480: f64 = (-locals.var_atr_i);
                let assign14970_e22482: f64 = (assign14970_e22480 * locals.var_deltemp);
                let assign14970_e22483: f64 = (1.0 + assign14970_e22482);
                let assign14970_e22485: f64 = (assign14970_e22483 - 1e-6);
                let assign14970_e22488: f64 = (-locals.var_atr_i);
                let assign14970_e22490: f64 = (assign14970_e22488 * locals.var_deltemp);
                let assign14970_e22491: f64 = (1.0 + assign14970_e22490);
                let assign14970_e22493: f64 = (assign14970_e22491 - 1e-6);
                let assign14970_e22496: f64 = (-locals.var_atr_i);
                let assign14970_e22498: f64 = (assign14970_e22496 * locals.var_deltemp);
                let assign14970_e22499: f64 = (1.0 + assign14970_e22498);
                let assign14970_e22501: f64 = (assign14970_e22499 - 1e-6);
                let assign14970_e22502: f64 = (assign14970_e22493 * assign14970_e22501);
                let assign14970_e22505: f64 = (4.0 * 0.001);
                let assign14970_e22507: f64 = (assign14970_e22505 * 0.001);
                let assign14970_e22508: f64 = (assign14970_e22502 + assign14970_e22507);
                let assign14970_e22509: f64 = (assign14970_e22508).sqrt();
                let assign14970_e22510: f64 = (assign14970_e22485 + assign14970_e22509);
                let assign14970_e22511: f64 = (0.5 * assign14970_e22510);
                (assign14970_e22511, (0.5 * ((assign14970_e22480 * locals.var_deltemp_dn4) + ((((assign14970_e22488 * locals.var_deltemp_dn4) * assign14970_e22501) + (assign14970_e22493 * (assign14970_e22496 * locals.var_deltemp_dn4))) / (2.0 * assign14970_e22509)))),)
            } else {
                let assign14970_e22514: f64 = (-locals.var_atr_i);
                let assign14970_e22516: f64 = (assign14970_e22514 * locals.var_deltemp);
                let assign14970_e22517: f64 = (1.0 + assign14970_e22516);
                let assign14970_e22519: f64 = (assign14970_e22517 - 1e-6);
                let assign14970_e22521: f64 = (-10000.0);
                let assign14970_e22523: f64 = (assign14970_e22521 * 0.001);
                let (assign14970_e22539, assign14970_e22539_d_n4,) = {
                    if (assign14970_e22519 < assign14970_e22523) {
                        let assign14970_e22526: f64 = (-0.001);
                        let assign14970_e22528: f64 = (assign14970_e22526 * 0.001);
                        let assign14970_e22531: f64 = (-locals.var_atr_i);
                        let assign14970_e22533: f64 = (assign14970_e22531 * locals.var_deltemp);
                        let assign14970_e22534: f64 = (1.0 + assign14970_e22533);
                        let assign14970_e22536: f64 = (assign14970_e22534 - 1e-6);
                        let assign14970_e22537: f64 = (assign14970_e22528 / assign14970_e22536);
                        (assign14970_e22537, (-((assign14970_e22528 * (assign14970_e22531 * locals.var_deltemp_dn4)) / (assign14970_e22536 * assign14970_e22536))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign14970_e22539, assign14970_e22539_d_n4,)
            }
        };
        let assign14970_e22541: f64 = (locals.var_vsatr_i * assign14970_e22540);
        (assign14970_e22541, (locals.var_vsatr_i * assign14970_e22540_d_n4),)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn4,)
    }
};
        locals.var_vsatr_t = assign14970_e22543;
        locals.var_vsatr_t_dn4 = assign14970_e22543_d_n4;

        let assign14980_e22546: f64 = if locals.var_vsatr_t < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard251 = assign14980_e22546;

        let (assign14990_e22554, assign14990_e22554_d_n4,) = {
    if (((locals.var_guard244 != 0.0) && (locals.var_guard249 != 0.0)) && (locals.var_guard251 != 0.0)) {
        (1000.0, 0.0,)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn4,)
    }
};
        locals.var_vsatr_t = assign14990_e22554;
        locals.var_vsatr_t_dn4 = assign14990_e22554_d_n4;

        let assign15000_e22557: f64 = if p.p67 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard252 = assign15000_e22557;

        let (assign15010_e22568, assign15010_e22568_d_n0, assign15010_e22568_d_n2, assign15010_e22568_d_n3, assign15010_e22568_d_n4, assign15010_e22568_d_n5, assign15010_e22568_d_n6, assign15010_e22568_d_n7, assign15010_e22568_d_n8, assign15010_e22568_d_n9, assign15010_e22568_d_n10, assign15010_e22568_d_n11, assign15010_e22568_d_n13, assign15010_e22568_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard252 != 0.0)) {
        let assign15010_e22564: f64 = (locals.var_utecv_i * locals.var_trat_ln);
        let assign15010_e22565: f64 = (assign15010_e22564).exp();
        let assign15010_e22566: f64 = (locals.var_u0cv_i * assign15010_e22565);
        (assign15010_e22566, (locals.var_u0cv_i_dn0 * assign15010_e22565), (locals.var_u0cv_i_dn2 * assign15010_e22565), (locals.var_u0cv_i_dn3 * assign15010_e22565), ((locals.var_u0cv_i_dn4 * assign15010_e22565) + (locals.var_u0cv_i * (assign15010_e22565 * (locals.var_utecv_i * locals.var_trat_ln_dn4)))), (locals.var_u0cv_i_dn5 * assign15010_e22565), (locals.var_u0cv_i_dn6 * assign15010_e22565), (locals.var_u0cv_i_dn7 * assign15010_e22565), (locals.var_u0cv_i_dn8 * assign15010_e22565), (locals.var_u0cv_i_dn9 * assign15010_e22565), (locals.var_u0cv_i_dn10 * assign15010_e22565), (locals.var_u0cv_i_dn11 * assign15010_e22565), (locals.var_u0cv_i_dn13 * assign15010_e22565), (locals.var_u0cv_i_dn14 * assign15010_e22565),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15010_e22568;
        locals.var_t1_dn0 = assign15010_e22568_d_n0;
        locals.var_t1_dn2 = assign15010_e22568_d_n2;
        locals.var_t1_dn3 = assign15010_e22568_d_n3;
        locals.var_t1_dn4 = assign15010_e22568_d_n4;
        locals.var_t1_dn5 = assign15010_e22568_d_n5;
        locals.var_t1_dn6 = assign15010_e22568_d_n6;
        locals.var_t1_dn7 = assign15010_e22568_d_n7;
        locals.var_t1_dn8 = assign15010_e22568_d_n8;
        locals.var_t1_dn9 = assign15010_e22568_d_n9;
        locals.var_t1_dn10 = assign15010_e22568_d_n10;
        locals.var_t1_dn11 = assign15010_e22568_d_n11;
        locals.var_t1_dn13 = assign15010_e22568_d_n13;
        locals.var_t1_dn14 = assign15010_e22568_d_n14;

        let (assign15020_e22624, assign15020_e22624_d_n0, assign15020_e22624_d_n2, assign15020_e22624_d_n3, assign15020_e22624_d_n4, assign15020_e22624_d_n5, assign15020_e22624_d_n6, assign15020_e22624_d_n7, assign15020_e22624_d_n8, assign15020_e22624_d_n9, assign15020_e22624_d_n10, assign15020_e22624_d_n11, assign15020_e22624_d_n13, assign15020_e22624_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard252 != 0.0)) {
        let assign15020_e22574: f64 = (-0.9);
        let assign15020_e22576: f64 = (assign15020_e22574 * locals.var_t1);
        let assign15020_e22580: f64 = (locals.var_utlcv_i * locals.var_deltemp);
        let assign15020_e22582: f64 = (-0.9);
        let assign15020_e22584: f64 = (assign15020_e22582 * locals.var_t1);
        let assign15020_e22585: f64 = (assign15020_e22580 - assign15020_e22584);
        let assign15020_e22587: f64 = (assign15020_e22585 - 0.0001);
        let assign15020_e22590: f64 = (locals.var_utlcv_i * locals.var_deltemp);
        let assign15020_e22592: f64 = (-0.9);
        let assign15020_e22594: f64 = (assign15020_e22592 * locals.var_t1);
        let assign15020_e22595: f64 = (assign15020_e22590 - assign15020_e22594);
        let assign15020_e22597: f64 = (assign15020_e22595 - 0.0001);
        let assign15020_e22600: f64 = (locals.var_utlcv_i * locals.var_deltemp);
        let assign15020_e22602: f64 = (-0.9);
        let assign15020_e22604: f64 = (assign15020_e22602 * locals.var_t1);
        let assign15020_e22605: f64 = (assign15020_e22600 - assign15020_e22604);
        let assign15020_e22607: f64 = (assign15020_e22605 - 0.0001);
        let assign15020_e22608: f64 = (assign15020_e22597 * assign15020_e22607);
        let assign15020_e22611: f64 = (-0.9);
        let assign15020_e22613: f64 = (assign15020_e22611 * locals.var_t1);
        let assign15020_e22614: f64 = (4.0 * assign15020_e22613);
        let assign15020_e22616: f64 = (assign15020_e22614 * 0.0001);
        let assign15020_e22617: f64 = (assign15020_e22608 - assign15020_e22616);
        let assign15020_e22618: f64 = (assign15020_e22617).sqrt();
        let assign15020_e22619: f64 = (assign15020_e22587 + assign15020_e22618);
        let assign15020_e22620: f64 = (0.5 * assign15020_e22619);
        let assign15020_e22621: f64 = (assign15020_e22576 + assign15020_e22620);
        let assign15020_e22622: f64 = (locals.var_t1 + assign15020_e22621);
        (assign15020_e22622, (locals.var_t1_dn0 + ((assign15020_e22574 * locals.var_t1_dn0) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn0)) + (((((-(assign15020_e22592 * locals.var_t1_dn0)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn0)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn0)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn2 + ((assign15020_e22574 * locals.var_t1_dn2) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn2)) + (((((-(assign15020_e22592 * locals.var_t1_dn2)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn2)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn2)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn3 + ((assign15020_e22574 * locals.var_t1_dn3) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn3)) + (((((-(assign15020_e22592 * locals.var_t1_dn3)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn3)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn3)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn4 + ((assign15020_e22574 * locals.var_t1_dn4) + (0.5 * (((locals.var_utlcv_i * locals.var_deltemp_dn4) - (assign15020_e22582 * locals.var_t1_dn4)) + ((((((locals.var_utlcv_i * locals.var_deltemp_dn4) - (assign15020_e22592 * locals.var_t1_dn4)) * assign15020_e22607) + (assign15020_e22597 * ((locals.var_utlcv_i * locals.var_deltemp_dn4) - (assign15020_e22602 * locals.var_t1_dn4)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn4)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn5 + ((assign15020_e22574 * locals.var_t1_dn5) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn5)) + (((((-(assign15020_e22592 * locals.var_t1_dn5)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn5)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn5)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn6 + ((assign15020_e22574 * locals.var_t1_dn6) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn6)) + (((((-(assign15020_e22592 * locals.var_t1_dn6)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn6)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn6)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn7 + ((assign15020_e22574 * locals.var_t1_dn7) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn7)) + (((((-(assign15020_e22592 * locals.var_t1_dn7)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn7)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn7)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn8 + ((assign15020_e22574 * locals.var_t1_dn8) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn8)) + (((((-(assign15020_e22592 * locals.var_t1_dn8)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn8)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn8)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn9 + ((assign15020_e22574 * locals.var_t1_dn9) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn9)) + (((((-(assign15020_e22592 * locals.var_t1_dn9)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn9)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn9)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn10 + ((assign15020_e22574 * locals.var_t1_dn10) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn10)) + (((((-(assign15020_e22592 * locals.var_t1_dn10)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn10)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn10)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn11 + ((assign15020_e22574 * locals.var_t1_dn11) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn11)) + (((((-(assign15020_e22592 * locals.var_t1_dn11)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn11)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn11)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn13 + ((assign15020_e22574 * locals.var_t1_dn13) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn13)) + (((((-(assign15020_e22592 * locals.var_t1_dn13)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn13)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn13)) * 0.0001)) / (2.0 * assign15020_e22618)))))), (locals.var_t1_dn14 + ((assign15020_e22574 * locals.var_t1_dn14) + (0.5 * ((-(assign15020_e22582 * locals.var_t1_dn14)) + (((((-(assign15020_e22592 * locals.var_t1_dn14)) * assign15020_e22607) + (assign15020_e22597 * (-(assign15020_e22602 * locals.var_t1_dn14)))) - ((4.0 * (assign15020_e22611 * locals.var_t1_dn14)) * 0.0001)) / (2.0 * assign15020_e22618)))))),)
    } else {
        (locals.var_u0_cv, locals.var_u0_cv_dn0, locals.var_u0_cv_dn2, locals.var_u0_cv_dn3, locals.var_u0_cv_dn4, locals.var_u0_cv_dn5, locals.var_u0_cv_dn6, locals.var_u0_cv_dn7, locals.var_u0_cv_dn8, locals.var_u0_cv_dn9, locals.var_u0_cv_dn10, locals.var_u0_cv_dn11, locals.var_u0_cv_dn13, locals.var_u0_cv_dn14,)
    }
};
        locals.var_u0_cv = assign15020_e22624;
        locals.var_u0_cv_dn0 = assign15020_e22624_d_n0;
        locals.var_u0_cv_dn2 = assign15020_e22624_d_n2;
        locals.var_u0_cv_dn3 = assign15020_e22624_d_n3;
        locals.var_u0_cv_dn4 = assign15020_e22624_d_n4;
        locals.var_u0_cv_dn5 = assign15020_e22624_d_n5;
        locals.var_u0_cv_dn6 = assign15020_e22624_d_n6;
        locals.var_u0_cv_dn7 = assign15020_e22624_d_n7;
        locals.var_u0_cv_dn8 = assign15020_e22624_d_n8;
        locals.var_u0_cv_dn9 = assign15020_e22624_d_n9;
        locals.var_u0_cv_dn10 = assign15020_e22624_d_n10;
        locals.var_u0_cv_dn11 = assign15020_e22624_d_n11;
        locals.var_u0_cv_dn13 = assign15020_e22624_d_n13;
        locals.var_u0_cv_dn14 = assign15020_e22624_d_n14;

        let (assign15030_e22670, assign15030_e22670_d_n0, assign15030_e22670_d_n2, assign15030_e22670_d_n3, assign15030_e22670_d_n4, assign15030_e22670_d_n5, assign15030_e22670_d_n6, assign15030_e22670_d_n7, assign15030_e22670_d_n8, assign15030_e22670_d_n9, assign15030_e22670_d_n10, assign15030_e22670_d_n11, assign15030_e22670_d_n13, assign15030_e22670_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard252 != 0.0)) {
        let assign15030_e22630: f64 = (-locals.var_uacv_i);
        let assign15030_e22634: f64 = (locals.var_ua1cv_i * locals.var_deltemp);
        let assign15030_e22636: f64 = (-locals.var_uacv_i);
        let assign15030_e22637: f64 = (assign15030_e22634 - assign15030_e22636);
        let assign15030_e22639: f64 = (assign15030_e22637 - 1e-6);
        let assign15030_e22642: f64 = (locals.var_ua1cv_i * locals.var_deltemp);
        let assign15030_e22644: f64 = (-locals.var_uacv_i);
        let assign15030_e22645: f64 = (assign15030_e22642 - assign15030_e22644);
        let assign15030_e22647: f64 = (assign15030_e22645 - 1e-6);
        let assign15030_e22650: f64 = (locals.var_ua1cv_i * locals.var_deltemp);
        let assign15030_e22652: f64 = (-locals.var_uacv_i);
        let assign15030_e22653: f64 = (assign15030_e22650 - assign15030_e22652);
        let assign15030_e22655: f64 = (assign15030_e22653 - 1e-6);
        let assign15030_e22656: f64 = (assign15030_e22647 * assign15030_e22655);
        let assign15030_e22659: f64 = (-locals.var_uacv_i);
        let assign15030_e22660: f64 = (4.0 * assign15030_e22659);
        let assign15030_e22662: f64 = (assign15030_e22660 * 1e-6);
        let assign15030_e22663: f64 = (assign15030_e22656 - assign15030_e22662);
        let assign15030_e22664: f64 = (assign15030_e22663).sqrt();
        let assign15030_e22665: f64 = (assign15030_e22639 + assign15030_e22664);
        let assign15030_e22666: f64 = (0.5 * assign15030_e22665);
        let assign15030_e22667: f64 = (assign15030_e22630 + assign15030_e22666);
        let assign15030_e22668: f64 = (locals.var_uacv_i + assign15030_e22667);
        (assign15030_e22668, 0.0, 0.0, 0.0, (0.5 * ((locals.var_ua1cv_i * locals.var_deltemp_dn4) + ((((locals.var_ua1cv_i * locals.var_deltemp_dn4) * assign15030_e22655) + (assign15030_e22647 * (locals.var_ua1cv_i * locals.var_deltemp_dn4))) / (2.0 * assign15030_e22664)))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uacv_t, locals.var_uacv_t_dn0, locals.var_uacv_t_dn2, locals.var_uacv_t_dn3, locals.var_uacv_t_dn4, locals.var_uacv_t_dn5, locals.var_uacv_t_dn6, locals.var_uacv_t_dn7, locals.var_uacv_t_dn8, locals.var_uacv_t_dn9, locals.var_uacv_t_dn10, locals.var_uacv_t_dn11, locals.var_uacv_t_dn13, locals.var_uacv_t_dn14,)
    }
};
        locals.var_uacv_t = assign15030_e22670;
        locals.var_uacv_t_dn0 = assign15030_e22670_d_n0;
        locals.var_uacv_t_dn2 = assign15030_e22670_d_n2;
        locals.var_uacv_t_dn3 = assign15030_e22670_d_n3;
        locals.var_uacv_t_dn4 = assign15030_e22670_d_n4;
        locals.var_uacv_t_dn5 = assign15030_e22670_d_n5;
        locals.var_uacv_t_dn6 = assign15030_e22670_d_n6;
        locals.var_uacv_t_dn7 = assign15030_e22670_d_n7;
        locals.var_uacv_t_dn8 = assign15030_e22670_d_n8;
        locals.var_uacv_t_dn9 = assign15030_e22670_d_n9;
        locals.var_uacv_t_dn10 = assign15030_e22670_d_n10;
        locals.var_uacv_t_dn11 = assign15030_e22670_d_n11;
        locals.var_uacv_t_dn13 = assign15030_e22670_d_n13;
        locals.var_uacv_t_dn14 = assign15030_e22670_d_n14;

    }

    pub(super) fn stamp_transient_block_43(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15040_e22681, assign15040_e22681_d_n4,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard252 != 0.0)) {
        let assign15040_e22677: f64 = (locals.var_ud1cv_i * locals.var_trat_ln);
        let assign15040_e22678: f64 = (assign15040_e22677).exp();
        let assign15040_e22679: f64 = (locals.var_udcv_i * assign15040_e22678);
        (assign15040_e22679, (locals.var_udcv_i * (assign15040_e22678 * (locals.var_ud1cv_i * locals.var_trat_ln_dn4))),)
    } else {
        (locals.var_udcv_t, locals.var_udcv_t_dn4,)
    }
};
        locals.var_udcv_t = assign15040_e22681;
        locals.var_udcv_t_dn4 = assign15040_e22681_d_n4;

        let assign15050_e22684: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard253 = assign15050_e22684;

        let (assign15060_e22733, assign15060_e22733_d_n0, assign15060_e22733_d_n2, assign15060_e22733_d_n3, assign15060_e22733_d_n4, assign15060_e22733_d_n5, assign15060_e22733_d_n6, assign15060_e22733_d_n7, assign15060_e22733_d_n8, assign15060_e22733_d_n9, assign15060_e22733_d_n10, assign15060_e22733_d_n11, assign15060_e22733_d_n13, assign15060_e22733_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard253 != 0.0)) {
        let assign15060_e22690: f64 = (-locals.var_vsat1_i);
        let assign15060_e22693: f64 = (-locals.var_at_i);
        let assign15060_e22695: f64 = (assign15060_e22693 * locals.var_deltemp);
        let assign15060_e22697: f64 = (-locals.var_vsat1_i);
        let assign15060_e22698: f64 = (assign15060_e22695 - assign15060_e22697);
        let assign15060_e22700: f64 = (assign15060_e22698 - 1e-6);
        let assign15060_e22702: f64 = (-locals.var_at_i);
        let assign15060_e22704: f64 = (assign15060_e22702 * locals.var_deltemp);
        let assign15060_e22706: f64 = (-locals.var_vsat1_i);
        let assign15060_e22707: f64 = (assign15060_e22704 - assign15060_e22706);
        let assign15060_e22709: f64 = (assign15060_e22707 - 1e-6);
        let assign15060_e22711: f64 = (-locals.var_at_i);
        let assign15060_e22713: f64 = (assign15060_e22711 * locals.var_deltemp);
        let assign15060_e22715: f64 = (-locals.var_vsat1_i);
        let assign15060_e22716: f64 = (assign15060_e22713 - assign15060_e22715);
        let assign15060_e22718: f64 = (assign15060_e22716 - 1e-6);
        let assign15060_e22719: f64 = (assign15060_e22709 * assign15060_e22718);
        let assign15060_e22722: f64 = (-locals.var_vsat1_i);
        let assign15060_e22723: f64 = (4.0 * assign15060_e22722);
        let assign15060_e22725: f64 = (assign15060_e22723 * 1e-6);
        let assign15060_e22726: f64 = (assign15060_e22719 - assign15060_e22725);
        let assign15060_e22727: f64 = (assign15060_e22726).sqrt();
        let assign15060_e22728: f64 = (assign15060_e22700 + assign15060_e22727);
        let assign15060_e22729: f64 = (0.5 * assign15060_e22728);
        let assign15060_e22730: f64 = (assign15060_e22690 + assign15060_e22729);
        let assign15060_e22731: f64 = (locals.var_vsat1_i + assign15060_e22730);
        (assign15060_e22731, (locals.var_vsat1_i_dn0 + ((-locals.var_vsat1_i_dn0) + (0.5 * ((-(-locals.var_vsat1_i_dn0)) + (((((-(-locals.var_vsat1_i_dn0)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn0)))) - ((4.0 * (-locals.var_vsat1_i_dn0)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn2 + ((-locals.var_vsat1_i_dn2) + (0.5 * ((-(-locals.var_vsat1_i_dn2)) + (((((-(-locals.var_vsat1_i_dn2)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn2)))) - ((4.0 * (-locals.var_vsat1_i_dn2)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn3 + ((-locals.var_vsat1_i_dn3) + (0.5 * ((-(-locals.var_vsat1_i_dn3)) + (((((-(-locals.var_vsat1_i_dn3)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn3)))) - ((4.0 * (-locals.var_vsat1_i_dn3)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn4 + ((-locals.var_vsat1_i_dn4) + (0.5 * (((assign15060_e22693 * locals.var_deltemp_dn4) - (-locals.var_vsat1_i_dn4)) + ((((((assign15060_e22702 * locals.var_deltemp_dn4) - (-locals.var_vsat1_i_dn4)) * assign15060_e22718) + (assign15060_e22709 * ((assign15060_e22711 * locals.var_deltemp_dn4) - (-locals.var_vsat1_i_dn4)))) - ((4.0 * (-locals.var_vsat1_i_dn4)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn5 + ((-locals.var_vsat1_i_dn5) + (0.5 * ((-(-locals.var_vsat1_i_dn5)) + (((((-(-locals.var_vsat1_i_dn5)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn5)))) - ((4.0 * (-locals.var_vsat1_i_dn5)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn6 + ((-locals.var_vsat1_i_dn6) + (0.5 * ((-(-locals.var_vsat1_i_dn6)) + (((((-(-locals.var_vsat1_i_dn6)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn6)))) - ((4.0 * (-locals.var_vsat1_i_dn6)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn7 + ((-locals.var_vsat1_i_dn7) + (0.5 * ((-(-locals.var_vsat1_i_dn7)) + (((((-(-locals.var_vsat1_i_dn7)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn7)))) - ((4.0 * (-locals.var_vsat1_i_dn7)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn8 + ((-locals.var_vsat1_i_dn8) + (0.5 * ((-(-locals.var_vsat1_i_dn8)) + (((((-(-locals.var_vsat1_i_dn8)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn8)))) - ((4.0 * (-locals.var_vsat1_i_dn8)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn9 + ((-locals.var_vsat1_i_dn9) + (0.5 * ((-(-locals.var_vsat1_i_dn9)) + (((((-(-locals.var_vsat1_i_dn9)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn9)))) - ((4.0 * (-locals.var_vsat1_i_dn9)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn10 + ((-locals.var_vsat1_i_dn10) + (0.5 * ((-(-locals.var_vsat1_i_dn10)) + (((((-(-locals.var_vsat1_i_dn10)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn10)))) - ((4.0 * (-locals.var_vsat1_i_dn10)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn11 + ((-locals.var_vsat1_i_dn11) + (0.5 * ((-(-locals.var_vsat1_i_dn11)) + (((((-(-locals.var_vsat1_i_dn11)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn11)))) - ((4.0 * (-locals.var_vsat1_i_dn11)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn13 + ((-locals.var_vsat1_i_dn13) + (0.5 * ((-(-locals.var_vsat1_i_dn13)) + (((((-(-locals.var_vsat1_i_dn13)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn13)))) - ((4.0 * (-locals.var_vsat1_i_dn13)) * 1e-6)) / (2.0 * assign15060_e22727)))))), (locals.var_vsat1_i_dn14 + ((-locals.var_vsat1_i_dn14) + (0.5 * ((-(-locals.var_vsat1_i_dn14)) + (((((-(-locals.var_vsat1_i_dn14)) * assign15060_e22718) + (assign15060_e22709 * (-(-locals.var_vsat1_i_dn14)))) - ((4.0 * (-locals.var_vsat1_i_dn14)) * 1e-6)) / (2.0 * assign15060_e22727)))))),)
    } else {
        (locals.var_vsat1_t, locals.var_vsat1_t_dn0, locals.var_vsat1_t_dn2, locals.var_vsat1_t_dn3, locals.var_vsat1_t_dn4, locals.var_vsat1_t_dn5, locals.var_vsat1_t_dn6, locals.var_vsat1_t_dn7, locals.var_vsat1_t_dn8, locals.var_vsat1_t_dn9, locals.var_vsat1_t_dn10, locals.var_vsat1_t_dn11, locals.var_vsat1_t_dn13, locals.var_vsat1_t_dn14,)
    }
};
        locals.var_vsat1_t = assign15060_e22733;
        locals.var_vsat1_t_dn0 = assign15060_e22733_d_n0;
        locals.var_vsat1_t_dn2 = assign15060_e22733_d_n2;
        locals.var_vsat1_t_dn3 = assign15060_e22733_d_n3;
        locals.var_vsat1_t_dn4 = assign15060_e22733_d_n4;
        locals.var_vsat1_t_dn5 = assign15060_e22733_d_n5;
        locals.var_vsat1_t_dn6 = assign15060_e22733_d_n6;
        locals.var_vsat1_t_dn7 = assign15060_e22733_d_n7;
        locals.var_vsat1_t_dn8 = assign15060_e22733_d_n8;
        locals.var_vsat1_t_dn9 = assign15060_e22733_d_n9;
        locals.var_vsat1_t_dn10 = assign15060_e22733_d_n10;
        locals.var_vsat1_t_dn11 = assign15060_e22733_d_n11;
        locals.var_vsat1_t_dn13 = assign15060_e22733_d_n13;
        locals.var_vsat1_t_dn14 = assign15060_e22733_d_n14;

        let (assign15070_e22819, assign15070_e22819_d_n0, assign15070_e22819_d_n2, assign15070_e22819_d_n3, assign15070_e22819_d_n4, assign15070_e22819_d_n5, assign15070_e22819_d_n6, assign15070_e22819_d_n7, assign15070_e22819_d_n8, assign15070_e22819_d_n9, assign15070_e22819_d_n10, assign15070_e22819_d_n11, assign15070_e22819_d_n13, assign15070_e22819_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard253 == 0.0)) {
        let assign15070_e22741: f64 = (-locals.var_at_i);
        let assign15070_e22743: f64 = (assign15070_e22741 * locals.var_deltemp);
        let assign15070_e22744: f64 = (1.0 + assign15070_e22743);
        let assign15070_e22746: f64 = (assign15070_e22744 - 1e-6);
        let assign15070_e22748: f64 = (-10000.0);
        let assign15070_e22750: f64 = (assign15070_e22748 * 0.001);
        let (assign15070_e22816, assign15070_e22816_d_n4,) = {
            if (!(assign15070_e22746 < assign15070_e22750)) {
                let assign15070_e22756: f64 = (-locals.var_at_i);
                let assign15070_e22758: f64 = (assign15070_e22756 * locals.var_deltemp);
                let assign15070_e22759: f64 = (1.0 + assign15070_e22758);
                let assign15070_e22761: f64 = (assign15070_e22759 - 1e-6);
                let assign15070_e22764: f64 = (-locals.var_at_i);
                let assign15070_e22766: f64 = (assign15070_e22764 * locals.var_deltemp);
                let assign15070_e22767: f64 = (1.0 + assign15070_e22766);
                let assign15070_e22769: f64 = (assign15070_e22767 - 1e-6);
                let assign15070_e22772: f64 = (-locals.var_at_i);
                let assign15070_e22774: f64 = (assign15070_e22772 * locals.var_deltemp);
                let assign15070_e22775: f64 = (1.0 + assign15070_e22774);
                let assign15070_e22777: f64 = (assign15070_e22775 - 1e-6);
                let assign15070_e22778: f64 = (assign15070_e22769 * assign15070_e22777);
                let assign15070_e22781: f64 = (4.0 * 0.001);
                let assign15070_e22783: f64 = (assign15070_e22781 * 0.001);
                let assign15070_e22784: f64 = (assign15070_e22778 + assign15070_e22783);
                let assign15070_e22785: f64 = (assign15070_e22784).sqrt();
                let assign15070_e22786: f64 = (assign15070_e22761 + assign15070_e22785);
                let assign15070_e22787: f64 = (0.5 * assign15070_e22786);
                (assign15070_e22787, (0.5 * ((assign15070_e22756 * locals.var_deltemp_dn4) + ((((assign15070_e22764 * locals.var_deltemp_dn4) * assign15070_e22777) + (assign15070_e22769 * (assign15070_e22772 * locals.var_deltemp_dn4))) / (2.0 * assign15070_e22785)))),)
            } else {
                let assign15070_e22790: f64 = (-locals.var_at_i);
                let assign15070_e22792: f64 = (assign15070_e22790 * locals.var_deltemp);
                let assign15070_e22793: f64 = (1.0 + assign15070_e22792);
                let assign15070_e22795: f64 = (assign15070_e22793 - 1e-6);
                let assign15070_e22797: f64 = (-10000.0);
                let assign15070_e22799: f64 = (assign15070_e22797 * 0.001);
                let (assign15070_e22815, assign15070_e22815_d_n4,) = {
                    if (assign15070_e22795 < assign15070_e22799) {
                        let assign15070_e22802: f64 = (-0.001);
                        let assign15070_e22804: f64 = (assign15070_e22802 * 0.001);
                        let assign15070_e22807: f64 = (-locals.var_at_i);
                        let assign15070_e22809: f64 = (assign15070_e22807 * locals.var_deltemp);
                        let assign15070_e22810: f64 = (1.0 + assign15070_e22809);
                        let assign15070_e22812: f64 = (assign15070_e22810 - 1e-6);
                        let assign15070_e22813: f64 = (assign15070_e22804 / assign15070_e22812);
                        (assign15070_e22813, (-((assign15070_e22804 * (assign15070_e22807 * locals.var_deltemp_dn4)) / (assign15070_e22812 * assign15070_e22812))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15070_e22815, assign15070_e22815_d_n4,)
            }
        };
        let assign15070_e22817: f64 = (locals.var_vsat1_i * assign15070_e22816);
        (assign15070_e22817, (locals.var_vsat1_i_dn0 * assign15070_e22816), (locals.var_vsat1_i_dn2 * assign15070_e22816), (locals.var_vsat1_i_dn3 * assign15070_e22816), ((locals.var_vsat1_i_dn4 * assign15070_e22816) + (locals.var_vsat1_i * assign15070_e22816_d_n4)), (locals.var_vsat1_i_dn5 * assign15070_e22816), (locals.var_vsat1_i_dn6 * assign15070_e22816), (locals.var_vsat1_i_dn7 * assign15070_e22816), (locals.var_vsat1_i_dn8 * assign15070_e22816), (locals.var_vsat1_i_dn9 * assign15070_e22816), (locals.var_vsat1_i_dn10 * assign15070_e22816), (locals.var_vsat1_i_dn11 * assign15070_e22816), (locals.var_vsat1_i_dn13 * assign15070_e22816), (locals.var_vsat1_i_dn14 * assign15070_e22816),)
    } else {
        (locals.var_vsat1_t, locals.var_vsat1_t_dn0, locals.var_vsat1_t_dn2, locals.var_vsat1_t_dn3, locals.var_vsat1_t_dn4, locals.var_vsat1_t_dn5, locals.var_vsat1_t_dn6, locals.var_vsat1_t_dn7, locals.var_vsat1_t_dn8, locals.var_vsat1_t_dn9, locals.var_vsat1_t_dn10, locals.var_vsat1_t_dn11, locals.var_vsat1_t_dn13, locals.var_vsat1_t_dn14,)
    }
};
        locals.var_vsat1_t = assign15070_e22819;
        locals.var_vsat1_t_dn0 = assign15070_e22819_d_n0;
        locals.var_vsat1_t_dn2 = assign15070_e22819_d_n2;
        locals.var_vsat1_t_dn3 = assign15070_e22819_d_n3;
        locals.var_vsat1_t_dn4 = assign15070_e22819_d_n4;
        locals.var_vsat1_t_dn5 = assign15070_e22819_d_n5;
        locals.var_vsat1_t_dn6 = assign15070_e22819_d_n6;
        locals.var_vsat1_t_dn7 = assign15070_e22819_d_n7;
        locals.var_vsat1_t_dn8 = assign15070_e22819_d_n8;
        locals.var_vsat1_t_dn9 = assign15070_e22819_d_n9;
        locals.var_vsat1_t_dn10 = assign15070_e22819_d_n10;
        locals.var_vsat1_t_dn11 = assign15070_e22819_d_n11;
        locals.var_vsat1_t_dn13 = assign15070_e22819_d_n13;
        locals.var_vsat1_t_dn14 = assign15070_e22819_d_n14;

        let assign15080_e22822: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard254 = assign15080_e22822;

        let assign15090_e22825: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard255 = assign15090_e22825;

        let (assign15100_e22876, assign15100_e22876_d_n0, assign15100_e22876_d_n2, assign15100_e22876_d_n3, assign15100_e22876_d_n4, assign15100_e22876_d_n5, assign15100_e22876_d_n6, assign15100_e22876_d_n7, assign15100_e22876_d_n8, assign15100_e22876_d_n9, assign15100_e22876_d_n10, assign15100_e22876_d_n11, assign15100_e22876_d_n13, assign15100_e22876_d_n14,) = {
    if (((locals.var_guard244 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard255 != 0.0)) {
        let assign15100_e22833: f64 = (-locals.var_vsat1r_i);
        let assign15100_e22836: f64 = (-locals.var_at_i);
        let assign15100_e22838: f64 = (assign15100_e22836 * locals.var_deltemp);
        let assign15100_e22840: f64 = (-locals.var_vsat1r_i);
        let assign15100_e22841: f64 = (assign15100_e22838 - assign15100_e22840);
        let assign15100_e22843: f64 = (assign15100_e22841 - 1e-6);
        let assign15100_e22845: f64 = (-locals.var_at_i);
        let assign15100_e22847: f64 = (assign15100_e22845 * locals.var_deltemp);
        let assign15100_e22849: f64 = (-locals.var_vsat1r_i);
        let assign15100_e22850: f64 = (assign15100_e22847 - assign15100_e22849);
        let assign15100_e22852: f64 = (assign15100_e22850 - 1e-6);
        let assign15100_e22854: f64 = (-locals.var_at_i);
        let assign15100_e22856: f64 = (assign15100_e22854 * locals.var_deltemp);
        let assign15100_e22858: f64 = (-locals.var_vsat1r_i);
        let assign15100_e22859: f64 = (assign15100_e22856 - assign15100_e22858);
        let assign15100_e22861: f64 = (assign15100_e22859 - 1e-6);
        let assign15100_e22862: f64 = (assign15100_e22852 * assign15100_e22861);
        let assign15100_e22865: f64 = (-locals.var_vsat1r_i);
        let assign15100_e22866: f64 = (4.0 * assign15100_e22865);
        let assign15100_e22868: f64 = (assign15100_e22866 * 1e-6);
        let assign15100_e22869: f64 = (assign15100_e22862 - assign15100_e22868);
        let assign15100_e22870: f64 = (assign15100_e22869).sqrt();
        let assign15100_e22871: f64 = (assign15100_e22843 + assign15100_e22870);
        let assign15100_e22872: f64 = (0.5 * assign15100_e22871);
        let assign15100_e22873: f64 = (assign15100_e22833 + assign15100_e22872);
        let assign15100_e22874: f64 = (locals.var_vsat1r_i + assign15100_e22873);
        (assign15100_e22874, (locals.var_vsat1r_i_dn0 + ((-locals.var_vsat1r_i_dn0) + (0.5 * ((-(-locals.var_vsat1r_i_dn0)) + (((((-(-locals.var_vsat1r_i_dn0)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn0)))) - ((4.0 * (-locals.var_vsat1r_i_dn0)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn2 + ((-locals.var_vsat1r_i_dn2) + (0.5 * ((-(-locals.var_vsat1r_i_dn2)) + (((((-(-locals.var_vsat1r_i_dn2)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn2)))) - ((4.0 * (-locals.var_vsat1r_i_dn2)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn3 + ((-locals.var_vsat1r_i_dn3) + (0.5 * ((-(-locals.var_vsat1r_i_dn3)) + (((((-(-locals.var_vsat1r_i_dn3)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn3)))) - ((4.0 * (-locals.var_vsat1r_i_dn3)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn4 + ((-locals.var_vsat1r_i_dn4) + (0.5 * (((assign15100_e22836 * locals.var_deltemp_dn4) - (-locals.var_vsat1r_i_dn4)) + ((((((assign15100_e22845 * locals.var_deltemp_dn4) - (-locals.var_vsat1r_i_dn4)) * assign15100_e22861) + (assign15100_e22852 * ((assign15100_e22854 * locals.var_deltemp_dn4) - (-locals.var_vsat1r_i_dn4)))) - ((4.0 * (-locals.var_vsat1r_i_dn4)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn5 + ((-locals.var_vsat1r_i_dn5) + (0.5 * ((-(-locals.var_vsat1r_i_dn5)) + (((((-(-locals.var_vsat1r_i_dn5)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn5)))) - ((4.0 * (-locals.var_vsat1r_i_dn5)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn6 + ((-locals.var_vsat1r_i_dn6) + (0.5 * ((-(-locals.var_vsat1r_i_dn6)) + (((((-(-locals.var_vsat1r_i_dn6)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn6)))) - ((4.0 * (-locals.var_vsat1r_i_dn6)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn7 + ((-locals.var_vsat1r_i_dn7) + (0.5 * ((-(-locals.var_vsat1r_i_dn7)) + (((((-(-locals.var_vsat1r_i_dn7)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn7)))) - ((4.0 * (-locals.var_vsat1r_i_dn7)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn8 + ((-locals.var_vsat1r_i_dn8) + (0.5 * ((-(-locals.var_vsat1r_i_dn8)) + (((((-(-locals.var_vsat1r_i_dn8)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn8)))) - ((4.0 * (-locals.var_vsat1r_i_dn8)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn9 + ((-locals.var_vsat1r_i_dn9) + (0.5 * ((-(-locals.var_vsat1r_i_dn9)) + (((((-(-locals.var_vsat1r_i_dn9)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn9)))) - ((4.0 * (-locals.var_vsat1r_i_dn9)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn10 + ((-locals.var_vsat1r_i_dn10) + (0.5 * ((-(-locals.var_vsat1r_i_dn10)) + (((((-(-locals.var_vsat1r_i_dn10)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn10)))) - ((4.0 * (-locals.var_vsat1r_i_dn10)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn11 + ((-locals.var_vsat1r_i_dn11) + (0.5 * ((-(-locals.var_vsat1r_i_dn11)) + (((((-(-locals.var_vsat1r_i_dn11)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn11)))) - ((4.0 * (-locals.var_vsat1r_i_dn11)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn13 + ((-locals.var_vsat1r_i_dn13) + (0.5 * ((-(-locals.var_vsat1r_i_dn13)) + (((((-(-locals.var_vsat1r_i_dn13)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn13)))) - ((4.0 * (-locals.var_vsat1r_i_dn13)) * 1e-6)) / (2.0 * assign15100_e22870)))))), (locals.var_vsat1r_i_dn14 + ((-locals.var_vsat1r_i_dn14) + (0.5 * ((-(-locals.var_vsat1r_i_dn14)) + (((((-(-locals.var_vsat1r_i_dn14)) * assign15100_e22861) + (assign15100_e22852 * (-(-locals.var_vsat1r_i_dn14)))) - ((4.0 * (-locals.var_vsat1r_i_dn14)) * 1e-6)) / (2.0 * assign15100_e22870)))))),)
    } else {
        (locals.var_vsat1r_t, locals.var_vsat1r_t_dn0, locals.var_vsat1r_t_dn2, locals.var_vsat1r_t_dn3, locals.var_vsat1r_t_dn4, locals.var_vsat1r_t_dn5, locals.var_vsat1r_t_dn6, locals.var_vsat1r_t_dn7, locals.var_vsat1r_t_dn8, locals.var_vsat1r_t_dn9, locals.var_vsat1r_t_dn10, locals.var_vsat1r_t_dn11, locals.var_vsat1r_t_dn13, locals.var_vsat1r_t_dn14,)
    }
};
        locals.var_vsat1r_t = assign15100_e22876;
        locals.var_vsat1r_t_dn0 = assign15100_e22876_d_n0;
        locals.var_vsat1r_t_dn2 = assign15100_e22876_d_n2;
        locals.var_vsat1r_t_dn3 = assign15100_e22876_d_n3;
        locals.var_vsat1r_t_dn4 = assign15100_e22876_d_n4;
        locals.var_vsat1r_t_dn5 = assign15100_e22876_d_n5;
        locals.var_vsat1r_t_dn6 = assign15100_e22876_d_n6;
        locals.var_vsat1r_t_dn7 = assign15100_e22876_d_n7;
        locals.var_vsat1r_t_dn8 = assign15100_e22876_d_n8;
        locals.var_vsat1r_t_dn9 = assign15100_e22876_d_n9;
        locals.var_vsat1r_t_dn10 = assign15100_e22876_d_n10;
        locals.var_vsat1r_t_dn11 = assign15100_e22876_d_n11;
        locals.var_vsat1r_t_dn13 = assign15100_e22876_d_n13;
        locals.var_vsat1r_t_dn14 = assign15100_e22876_d_n14;

        let (assign15110_e22964, assign15110_e22964_d_n0, assign15110_e22964_d_n2, assign15110_e22964_d_n3, assign15110_e22964_d_n4, assign15110_e22964_d_n5, assign15110_e22964_d_n6, assign15110_e22964_d_n7, assign15110_e22964_d_n8, assign15110_e22964_d_n9, assign15110_e22964_d_n10, assign15110_e22964_d_n11, assign15110_e22964_d_n13, assign15110_e22964_d_n14,) = {
    if (((locals.var_guard244 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard255 == 0.0)) {
        let assign15110_e22886: f64 = (-locals.var_at_i);
        let assign15110_e22888: f64 = (assign15110_e22886 * locals.var_deltemp);
        let assign15110_e22889: f64 = (1.0 + assign15110_e22888);
        let assign15110_e22891: f64 = (assign15110_e22889 - 1e-6);
        let assign15110_e22893: f64 = (-10000.0);
        let assign15110_e22895: f64 = (assign15110_e22893 * 0.001);
        let (assign15110_e22961, assign15110_e22961_d_n4,) = {
            if (!(assign15110_e22891 < assign15110_e22895)) {
                let assign15110_e22901: f64 = (-locals.var_at_i);
                let assign15110_e22903: f64 = (assign15110_e22901 * locals.var_deltemp);
                let assign15110_e22904: f64 = (1.0 + assign15110_e22903);
                let assign15110_e22906: f64 = (assign15110_e22904 - 1e-6);
                let assign15110_e22909: f64 = (-locals.var_at_i);
                let assign15110_e22911: f64 = (assign15110_e22909 * locals.var_deltemp);
                let assign15110_e22912: f64 = (1.0 + assign15110_e22911);
                let assign15110_e22914: f64 = (assign15110_e22912 - 1e-6);
                let assign15110_e22917: f64 = (-locals.var_at_i);
                let assign15110_e22919: f64 = (assign15110_e22917 * locals.var_deltemp);
                let assign15110_e22920: f64 = (1.0 + assign15110_e22919);
                let assign15110_e22922: f64 = (assign15110_e22920 - 1e-6);
                let assign15110_e22923: f64 = (assign15110_e22914 * assign15110_e22922);
                let assign15110_e22926: f64 = (4.0 * 0.001);
                let assign15110_e22928: f64 = (assign15110_e22926 * 0.001);
                let assign15110_e22929: f64 = (assign15110_e22923 + assign15110_e22928);
                let assign15110_e22930: f64 = (assign15110_e22929).sqrt();
                let assign15110_e22931: f64 = (assign15110_e22906 + assign15110_e22930);
                let assign15110_e22932: f64 = (0.5 * assign15110_e22931);
                (assign15110_e22932, (0.5 * ((assign15110_e22901 * locals.var_deltemp_dn4) + ((((assign15110_e22909 * locals.var_deltemp_dn4) * assign15110_e22922) + (assign15110_e22914 * (assign15110_e22917 * locals.var_deltemp_dn4))) / (2.0 * assign15110_e22930)))),)
            } else {
                let assign15110_e22935: f64 = (-locals.var_at_i);
                let assign15110_e22937: f64 = (assign15110_e22935 * locals.var_deltemp);
                let assign15110_e22938: f64 = (1.0 + assign15110_e22937);
                let assign15110_e22940: f64 = (assign15110_e22938 - 1e-6);
                let assign15110_e22942: f64 = (-10000.0);
                let assign15110_e22944: f64 = (assign15110_e22942 * 0.001);
                let (assign15110_e22960, assign15110_e22960_d_n4,) = {
                    if (assign15110_e22940 < assign15110_e22944) {
                        let assign15110_e22947: f64 = (-0.001);
                        let assign15110_e22949: f64 = (assign15110_e22947 * 0.001);
                        let assign15110_e22952: f64 = (-locals.var_at_i);
                        let assign15110_e22954: f64 = (assign15110_e22952 * locals.var_deltemp);
                        let assign15110_e22955: f64 = (1.0 + assign15110_e22954);
                        let assign15110_e22957: f64 = (assign15110_e22955 - 1e-6);
                        let assign15110_e22958: f64 = (assign15110_e22949 / assign15110_e22957);
                        (assign15110_e22958, (-((assign15110_e22949 * (assign15110_e22952 * locals.var_deltemp_dn4)) / (assign15110_e22957 * assign15110_e22957))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15110_e22960, assign15110_e22960_d_n4,)
            }
        };
        let assign15110_e22962: f64 = (locals.var_vsat1r_i * assign15110_e22961);
        (assign15110_e22962, (locals.var_vsat1r_i_dn0 * assign15110_e22961), (locals.var_vsat1r_i_dn2 * assign15110_e22961), (locals.var_vsat1r_i_dn3 * assign15110_e22961), ((locals.var_vsat1r_i_dn4 * assign15110_e22961) + (locals.var_vsat1r_i * assign15110_e22961_d_n4)), (locals.var_vsat1r_i_dn5 * assign15110_e22961), (locals.var_vsat1r_i_dn6 * assign15110_e22961), (locals.var_vsat1r_i_dn7 * assign15110_e22961), (locals.var_vsat1r_i_dn8 * assign15110_e22961), (locals.var_vsat1r_i_dn9 * assign15110_e22961), (locals.var_vsat1r_i_dn10 * assign15110_e22961), (locals.var_vsat1r_i_dn11 * assign15110_e22961), (locals.var_vsat1r_i_dn13 * assign15110_e22961), (locals.var_vsat1r_i_dn14 * assign15110_e22961),)
    } else {
        (locals.var_vsat1r_t, locals.var_vsat1r_t_dn0, locals.var_vsat1r_t_dn2, locals.var_vsat1r_t_dn3, locals.var_vsat1r_t_dn4, locals.var_vsat1r_t_dn5, locals.var_vsat1r_t_dn6, locals.var_vsat1r_t_dn7, locals.var_vsat1r_t_dn8, locals.var_vsat1r_t_dn9, locals.var_vsat1r_t_dn10, locals.var_vsat1r_t_dn11, locals.var_vsat1r_t_dn13, locals.var_vsat1r_t_dn14,)
    }
};
        locals.var_vsat1r_t = assign15110_e22964;
        locals.var_vsat1r_t_dn0 = assign15110_e22964_d_n0;
        locals.var_vsat1r_t_dn2 = assign15110_e22964_d_n2;
        locals.var_vsat1r_t_dn3 = assign15110_e22964_d_n3;
        locals.var_vsat1r_t_dn4 = assign15110_e22964_d_n4;
        locals.var_vsat1r_t_dn5 = assign15110_e22964_d_n5;
        locals.var_vsat1r_t_dn6 = assign15110_e22964_d_n6;
        locals.var_vsat1r_t_dn7 = assign15110_e22964_d_n7;
        locals.var_vsat1r_t_dn8 = assign15110_e22964_d_n8;
        locals.var_vsat1r_t_dn9 = assign15110_e22964_d_n9;
        locals.var_vsat1r_t_dn10 = assign15110_e22964_d_n10;
        locals.var_vsat1r_t_dn11 = assign15110_e22964_d_n11;
        locals.var_vsat1r_t_dn13 = assign15110_e22964_d_n13;
        locals.var_vsat1r_t_dn14 = assign15110_e22964_d_n14;

        let assign15120_e22967: f64 = if locals.var_vsat1r_t < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard256 = assign15120_e22967;

        let (assign15130_e22975, assign15130_e22975_d_n0, assign15130_e22975_d_n2, assign15130_e22975_d_n3, assign15130_e22975_d_n4, assign15130_e22975_d_n5, assign15130_e22975_d_n6, assign15130_e22975_d_n7, assign15130_e22975_d_n8, assign15130_e22975_d_n9, assign15130_e22975_d_n10, assign15130_e22975_d_n11, assign15130_e22975_d_n13, assign15130_e22975_d_n14,) = {
    if (((locals.var_guard244 != 0.0) && (locals.var_guard254 != 0.0)) && (locals.var_guard256 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsat1r_t, locals.var_vsat1r_t_dn0, locals.var_vsat1r_t_dn2, locals.var_vsat1r_t_dn3, locals.var_vsat1r_t_dn4, locals.var_vsat1r_t_dn5, locals.var_vsat1r_t_dn6, locals.var_vsat1r_t_dn7, locals.var_vsat1r_t_dn8, locals.var_vsat1r_t_dn9, locals.var_vsat1r_t_dn10, locals.var_vsat1r_t_dn11, locals.var_vsat1r_t_dn13, locals.var_vsat1r_t_dn14,)
    }
};
        locals.var_vsat1r_t = assign15130_e22975;
        locals.var_vsat1r_t_dn0 = assign15130_e22975_d_n0;
        locals.var_vsat1r_t_dn2 = assign15130_e22975_d_n2;
        locals.var_vsat1r_t_dn3 = assign15130_e22975_d_n3;
        locals.var_vsat1r_t_dn4 = assign15130_e22975_d_n4;
        locals.var_vsat1r_t_dn5 = assign15130_e22975_d_n5;
        locals.var_vsat1r_t_dn6 = assign15130_e22975_d_n6;
        locals.var_vsat1r_t_dn7 = assign15130_e22975_d_n7;
        locals.var_vsat1r_t_dn8 = assign15130_e22975_d_n8;
        locals.var_vsat1r_t_dn9 = assign15130_e22975_d_n9;
        locals.var_vsat1r_t_dn10 = assign15130_e22975_d_n10;
        locals.var_vsat1r_t_dn11 = assign15130_e22975_d_n11;
        locals.var_vsat1r_t_dn13 = assign15130_e22975_d_n13;
        locals.var_vsat1r_t_dn14 = assign15130_e22975_d_n14;

        let assign15140_e22978: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard257 = assign15140_e22978;

        let (assign15150_e23027, assign15150_e23027_d_n0, assign15150_e23027_d_n2, assign15150_e23027_d_n3, assign15150_e23027_d_n4, assign15150_e23027_d_n5, assign15150_e23027_d_n6, assign15150_e23027_d_n7, assign15150_e23027_d_n8, assign15150_e23027_d_n9, assign15150_e23027_d_n10, assign15150_e23027_d_n11, assign15150_e23027_d_n13, assign15150_e23027_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard257 != 0.0)) {
        let assign15150_e22984: f64 = (-locals.var_vsatcv_i);
        let assign15150_e22987: f64 = (-locals.var_atcv_i);
        let assign15150_e22989: f64 = (assign15150_e22987 * locals.var_deltemp);
        let assign15150_e22991: f64 = (-locals.var_vsatcv_i);
        let assign15150_e22992: f64 = (assign15150_e22989 - assign15150_e22991);
        let assign15150_e22994: f64 = (assign15150_e22992 - 1e-6);
        let assign15150_e22996: f64 = (-locals.var_atcv_i);
        let assign15150_e22998: f64 = (assign15150_e22996 * locals.var_deltemp);
        let assign15150_e23000: f64 = (-locals.var_vsatcv_i);
        let assign15150_e23001: f64 = (assign15150_e22998 - assign15150_e23000);
        let assign15150_e23003: f64 = (assign15150_e23001 - 1e-6);
        let assign15150_e23005: f64 = (-locals.var_atcv_i);
        let assign15150_e23007: f64 = (assign15150_e23005 * locals.var_deltemp);
        let assign15150_e23009: f64 = (-locals.var_vsatcv_i);
        let assign15150_e23010: f64 = (assign15150_e23007 - assign15150_e23009);
        let assign15150_e23012: f64 = (assign15150_e23010 - 1e-6);
        let assign15150_e23013: f64 = (assign15150_e23003 * assign15150_e23012);
        let assign15150_e23016: f64 = (-locals.var_vsatcv_i);
        let assign15150_e23017: f64 = (4.0 * assign15150_e23016);
        let assign15150_e23019: f64 = (assign15150_e23017 * 1e-6);
        let assign15150_e23020: f64 = (assign15150_e23013 - assign15150_e23019);
        let assign15150_e23021: f64 = (assign15150_e23020).sqrt();
        let assign15150_e23022: f64 = (assign15150_e22994 + assign15150_e23021);
        let assign15150_e23023: f64 = (0.5 * assign15150_e23022);
        let assign15150_e23024: f64 = (assign15150_e22984 + assign15150_e23023);
        let assign15150_e23025: f64 = (locals.var_vsatcv_i + assign15150_e23024);
        (assign15150_e23025, (locals.var_vsatcv_i_dn0 + ((-locals.var_vsatcv_i_dn0) + (0.5 * ((-(-locals.var_vsatcv_i_dn0)) + (((((-(-locals.var_vsatcv_i_dn0)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn0)))) - ((4.0 * (-locals.var_vsatcv_i_dn0)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn2 + ((-locals.var_vsatcv_i_dn2) + (0.5 * ((-(-locals.var_vsatcv_i_dn2)) + (((((-(-locals.var_vsatcv_i_dn2)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn2)))) - ((4.0 * (-locals.var_vsatcv_i_dn2)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn3 + ((-locals.var_vsatcv_i_dn3) + (0.5 * ((-(-locals.var_vsatcv_i_dn3)) + (((((-(-locals.var_vsatcv_i_dn3)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn3)))) - ((4.0 * (-locals.var_vsatcv_i_dn3)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn4 + ((-locals.var_vsatcv_i_dn4) + (0.5 * (((assign15150_e22987 * locals.var_deltemp_dn4) - (-locals.var_vsatcv_i_dn4)) + ((((((assign15150_e22996 * locals.var_deltemp_dn4) - (-locals.var_vsatcv_i_dn4)) * assign15150_e23012) + (assign15150_e23003 * ((assign15150_e23005 * locals.var_deltemp_dn4) - (-locals.var_vsatcv_i_dn4)))) - ((4.0 * (-locals.var_vsatcv_i_dn4)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn5 + ((-locals.var_vsatcv_i_dn5) + (0.5 * ((-(-locals.var_vsatcv_i_dn5)) + (((((-(-locals.var_vsatcv_i_dn5)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn5)))) - ((4.0 * (-locals.var_vsatcv_i_dn5)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn6 + ((-locals.var_vsatcv_i_dn6) + (0.5 * ((-(-locals.var_vsatcv_i_dn6)) + (((((-(-locals.var_vsatcv_i_dn6)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn6)))) - ((4.0 * (-locals.var_vsatcv_i_dn6)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn7 + ((-locals.var_vsatcv_i_dn7) + (0.5 * ((-(-locals.var_vsatcv_i_dn7)) + (((((-(-locals.var_vsatcv_i_dn7)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn7)))) - ((4.0 * (-locals.var_vsatcv_i_dn7)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn8 + ((-locals.var_vsatcv_i_dn8) + (0.5 * ((-(-locals.var_vsatcv_i_dn8)) + (((((-(-locals.var_vsatcv_i_dn8)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn8)))) - ((4.0 * (-locals.var_vsatcv_i_dn8)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn9 + ((-locals.var_vsatcv_i_dn9) + (0.5 * ((-(-locals.var_vsatcv_i_dn9)) + (((((-(-locals.var_vsatcv_i_dn9)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn9)))) - ((4.0 * (-locals.var_vsatcv_i_dn9)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn10 + ((-locals.var_vsatcv_i_dn10) + (0.5 * ((-(-locals.var_vsatcv_i_dn10)) + (((((-(-locals.var_vsatcv_i_dn10)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn10)))) - ((4.0 * (-locals.var_vsatcv_i_dn10)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn11 + ((-locals.var_vsatcv_i_dn11) + (0.5 * ((-(-locals.var_vsatcv_i_dn11)) + (((((-(-locals.var_vsatcv_i_dn11)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn11)))) - ((4.0 * (-locals.var_vsatcv_i_dn11)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn13 + ((-locals.var_vsatcv_i_dn13) + (0.5 * ((-(-locals.var_vsatcv_i_dn13)) + (((((-(-locals.var_vsatcv_i_dn13)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn13)))) - ((4.0 * (-locals.var_vsatcv_i_dn13)) * 1e-6)) / (2.0 * assign15150_e23021)))))), (locals.var_vsatcv_i_dn14 + ((-locals.var_vsatcv_i_dn14) + (0.5 * ((-(-locals.var_vsatcv_i_dn14)) + (((((-(-locals.var_vsatcv_i_dn14)) * assign15150_e23012) + (assign15150_e23003 * (-(-locals.var_vsatcv_i_dn14)))) - ((4.0 * (-locals.var_vsatcv_i_dn14)) * 1e-6)) / (2.0 * assign15150_e23021)))))),)
    } else {
        (locals.var_vsatcv_t, locals.var_vsatcv_t_dn0, locals.var_vsatcv_t_dn2, locals.var_vsatcv_t_dn3, locals.var_vsatcv_t_dn4, locals.var_vsatcv_t_dn5, locals.var_vsatcv_t_dn6, locals.var_vsatcv_t_dn7, locals.var_vsatcv_t_dn8, locals.var_vsatcv_t_dn9, locals.var_vsatcv_t_dn10, locals.var_vsatcv_t_dn11, locals.var_vsatcv_t_dn13, locals.var_vsatcv_t_dn14,)
    }
};
        locals.var_vsatcv_t = assign15150_e23027;
        locals.var_vsatcv_t_dn0 = assign15150_e23027_d_n0;
        locals.var_vsatcv_t_dn2 = assign15150_e23027_d_n2;
        locals.var_vsatcv_t_dn3 = assign15150_e23027_d_n3;
        locals.var_vsatcv_t_dn4 = assign15150_e23027_d_n4;
        locals.var_vsatcv_t_dn5 = assign15150_e23027_d_n5;
        locals.var_vsatcv_t_dn6 = assign15150_e23027_d_n6;
        locals.var_vsatcv_t_dn7 = assign15150_e23027_d_n7;
        locals.var_vsatcv_t_dn8 = assign15150_e23027_d_n8;
        locals.var_vsatcv_t_dn9 = assign15150_e23027_d_n9;
        locals.var_vsatcv_t_dn10 = assign15150_e23027_d_n10;
        locals.var_vsatcv_t_dn11 = assign15150_e23027_d_n11;
        locals.var_vsatcv_t_dn13 = assign15150_e23027_d_n13;
        locals.var_vsatcv_t_dn14 = assign15150_e23027_d_n14;

        let (assign15160_e23113, assign15160_e23113_d_n0, assign15160_e23113_d_n2, assign15160_e23113_d_n3, assign15160_e23113_d_n4, assign15160_e23113_d_n5, assign15160_e23113_d_n6, assign15160_e23113_d_n7, assign15160_e23113_d_n8, assign15160_e23113_d_n9, assign15160_e23113_d_n10, assign15160_e23113_d_n11, assign15160_e23113_d_n13, assign15160_e23113_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard257 == 0.0)) {
        let assign15160_e23035: f64 = (-locals.var_atcv_i);
        let assign15160_e23037: f64 = (assign15160_e23035 * locals.var_deltemp);
        let assign15160_e23038: f64 = (1.0 + assign15160_e23037);
        let assign15160_e23040: f64 = (assign15160_e23038 - 1e-6);
        let assign15160_e23042: f64 = (-10000.0);
        let assign15160_e23044: f64 = (assign15160_e23042 * 0.001);
        let (assign15160_e23110, assign15160_e23110_d_n4,) = {
            if (!(assign15160_e23040 < assign15160_e23044)) {
                let assign15160_e23050: f64 = (-locals.var_atcv_i);
                let assign15160_e23052: f64 = (assign15160_e23050 * locals.var_deltemp);
                let assign15160_e23053: f64 = (1.0 + assign15160_e23052);
                let assign15160_e23055: f64 = (assign15160_e23053 - 1e-6);
                let assign15160_e23058: f64 = (-locals.var_atcv_i);
                let assign15160_e23060: f64 = (assign15160_e23058 * locals.var_deltemp);
                let assign15160_e23061: f64 = (1.0 + assign15160_e23060);
                let assign15160_e23063: f64 = (assign15160_e23061 - 1e-6);
                let assign15160_e23066: f64 = (-locals.var_atcv_i);
                let assign15160_e23068: f64 = (assign15160_e23066 * locals.var_deltemp);
                let assign15160_e23069: f64 = (1.0 + assign15160_e23068);
                let assign15160_e23071: f64 = (assign15160_e23069 - 1e-6);
                let assign15160_e23072: f64 = (assign15160_e23063 * assign15160_e23071);
                let assign15160_e23075: f64 = (4.0 * 0.001);
                let assign15160_e23077: f64 = (assign15160_e23075 * 0.001);
                let assign15160_e23078: f64 = (assign15160_e23072 + assign15160_e23077);
                let assign15160_e23079: f64 = (assign15160_e23078).sqrt();
                let assign15160_e23080: f64 = (assign15160_e23055 + assign15160_e23079);
                let assign15160_e23081: f64 = (0.5 * assign15160_e23080);
                (assign15160_e23081, (0.5 * ((assign15160_e23050 * locals.var_deltemp_dn4) + ((((assign15160_e23058 * locals.var_deltemp_dn4) * assign15160_e23071) + (assign15160_e23063 * (assign15160_e23066 * locals.var_deltemp_dn4))) / (2.0 * assign15160_e23079)))),)
            } else {
                let assign15160_e23084: f64 = (-locals.var_atcv_i);
                let assign15160_e23086: f64 = (assign15160_e23084 * locals.var_deltemp);
                let assign15160_e23087: f64 = (1.0 + assign15160_e23086);
                let assign15160_e23089: f64 = (assign15160_e23087 - 1e-6);
                let assign15160_e23091: f64 = (-10000.0);
                let assign15160_e23093: f64 = (assign15160_e23091 * 0.001);
                let (assign15160_e23109, assign15160_e23109_d_n4,) = {
                    if (assign15160_e23089 < assign15160_e23093) {
                        let assign15160_e23096: f64 = (-0.001);
                        let assign15160_e23098: f64 = (assign15160_e23096 * 0.001);
                        let assign15160_e23101: f64 = (-locals.var_atcv_i);
                        let assign15160_e23103: f64 = (assign15160_e23101 * locals.var_deltemp);
                        let assign15160_e23104: f64 = (1.0 + assign15160_e23103);
                        let assign15160_e23106: f64 = (assign15160_e23104 - 1e-6);
                        let assign15160_e23107: f64 = (assign15160_e23098 / assign15160_e23106);
                        (assign15160_e23107, (-((assign15160_e23098 * (assign15160_e23101 * locals.var_deltemp_dn4)) / (assign15160_e23106 * assign15160_e23106))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15160_e23109, assign15160_e23109_d_n4,)
            }
        };
        let assign15160_e23111: f64 = (locals.var_vsatcv_i * assign15160_e23110);
        (assign15160_e23111, (locals.var_vsatcv_i_dn0 * assign15160_e23110), (locals.var_vsatcv_i_dn2 * assign15160_e23110), (locals.var_vsatcv_i_dn3 * assign15160_e23110), ((locals.var_vsatcv_i_dn4 * assign15160_e23110) + (locals.var_vsatcv_i * assign15160_e23110_d_n4)), (locals.var_vsatcv_i_dn5 * assign15160_e23110), (locals.var_vsatcv_i_dn6 * assign15160_e23110), (locals.var_vsatcv_i_dn7 * assign15160_e23110), (locals.var_vsatcv_i_dn8 * assign15160_e23110), (locals.var_vsatcv_i_dn9 * assign15160_e23110), (locals.var_vsatcv_i_dn10 * assign15160_e23110), (locals.var_vsatcv_i_dn11 * assign15160_e23110), (locals.var_vsatcv_i_dn13 * assign15160_e23110), (locals.var_vsatcv_i_dn14 * assign15160_e23110),)
    } else {
        (locals.var_vsatcv_t, locals.var_vsatcv_t_dn0, locals.var_vsatcv_t_dn2, locals.var_vsatcv_t_dn3, locals.var_vsatcv_t_dn4, locals.var_vsatcv_t_dn5, locals.var_vsatcv_t_dn6, locals.var_vsatcv_t_dn7, locals.var_vsatcv_t_dn8, locals.var_vsatcv_t_dn9, locals.var_vsatcv_t_dn10, locals.var_vsatcv_t_dn11, locals.var_vsatcv_t_dn13, locals.var_vsatcv_t_dn14,)
    }
};
        locals.var_vsatcv_t = assign15160_e23113;
        locals.var_vsatcv_t_dn0 = assign15160_e23113_d_n0;
        locals.var_vsatcv_t_dn2 = assign15160_e23113_d_n2;
        locals.var_vsatcv_t_dn3 = assign15160_e23113_d_n3;
        locals.var_vsatcv_t_dn4 = assign15160_e23113_d_n4;
        locals.var_vsatcv_t_dn5 = assign15160_e23113_d_n5;
        locals.var_vsatcv_t_dn6 = assign15160_e23113_d_n6;
        locals.var_vsatcv_t_dn7 = assign15160_e23113_d_n7;
        locals.var_vsatcv_t_dn8 = assign15160_e23113_d_n8;
        locals.var_vsatcv_t_dn9 = assign15160_e23113_d_n9;
        locals.var_vsatcv_t_dn10 = assign15160_e23113_d_n10;
        locals.var_vsatcv_t_dn11 = assign15160_e23113_d_n11;
        locals.var_vsatcv_t_dn13 = assign15160_e23113_d_n13;
        locals.var_vsatcv_t_dn14 = assign15160_e23113_d_n14;

        let (assign15170_e23202, assign15170_e23202_d_n0, assign15170_e23202_d_n2, assign15170_e23202_d_n3, assign15170_e23202_d_n4, assign15170_e23202_d_n5, assign15170_e23202_d_n6, assign15170_e23202_d_n7, assign15170_e23202_d_n8, assign15170_e23202_d_n9, assign15170_e23202_d_n10, assign15170_e23202_d_n11, assign15170_e23202_d_n13, assign15170_e23202_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        let assign15170_e23119: f64 = (p.p450 * locals.var_deltemp);
        let assign15170_e23120: f64 = (1.0 + assign15170_e23119);
        let assign15170_e23121: f64 = (locals.var_mexp_i * assign15170_e23120);
        let assign15170_e23123: f64 = (assign15170_e23121 - 2.0);
        let assign15170_e23125: f64 = (-10000.0);
        let assign15170_e23127: f64 = (assign15170_e23125 * 0.001);
        let (assign15170_e23198, assign15170_e23198_d_n0, assign15170_e23198_d_n2, assign15170_e23198_d_n3, assign15170_e23198_d_n4, assign15170_e23198_d_n5, assign15170_e23198_d_n6, assign15170_e23198_d_n7, assign15170_e23198_d_n8, assign15170_e23198_d_n9, assign15170_e23198_d_n10, assign15170_e23198_d_n11, assign15170_e23198_d_n13, assign15170_e23198_d_n14,) = {
            if (!(assign15170_e23123 < assign15170_e23127)) {
                let assign15170_e23135: f64 = (p.p450 * locals.var_deltemp);
                let assign15170_e23136: f64 = (1.0 + assign15170_e23135);
                let assign15170_e23137: f64 = (locals.var_mexp_i * assign15170_e23136);
                let assign15170_e23139: f64 = (assign15170_e23137 - 2.0);
                let assign15170_e23144: f64 = (p.p450 * locals.var_deltemp);
                let assign15170_e23145: f64 = (1.0 + assign15170_e23144);
                let assign15170_e23146: f64 = (locals.var_mexp_i * assign15170_e23145);
                let assign15170_e23148: f64 = (assign15170_e23146 - 2.0);
                let assign15170_e23153: f64 = (p.p450 * locals.var_deltemp);
                let assign15170_e23154: f64 = (1.0 + assign15170_e23153);
                let assign15170_e23155: f64 = (locals.var_mexp_i * assign15170_e23154);
                let assign15170_e23157: f64 = (assign15170_e23155 - 2.0);
                let assign15170_e23158: f64 = (assign15170_e23148 * assign15170_e23157);
                let assign15170_e23161: f64 = (4.0 * 0.001);
                let assign15170_e23163: f64 = (assign15170_e23161 * 0.001);
                let assign15170_e23164: f64 = (assign15170_e23158 + assign15170_e23163);
                let assign15170_e23165: f64 = (assign15170_e23164).sqrt();
                let assign15170_e23166: f64 = (assign15170_e23139 + assign15170_e23165);
                let assign15170_e23167: f64 = (0.5 * assign15170_e23166);
                (assign15170_e23167, (0.5 * ((locals.var_mexp_i_dn0 * assign15170_e23136) + ((((locals.var_mexp_i_dn0 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn0 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn2 * assign15170_e23136) + ((((locals.var_mexp_i_dn2 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn2 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn3 * assign15170_e23136) + ((((locals.var_mexp_i_dn3 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn3 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * (((locals.var_mexp_i_dn4 * assign15170_e23136) + (locals.var_mexp_i * (p.p450 * locals.var_deltemp_dn4))) + (((((locals.var_mexp_i_dn4 * assign15170_e23145) + (locals.var_mexp_i * (p.p450 * locals.var_deltemp_dn4))) * assign15170_e23157) + (assign15170_e23148 * ((locals.var_mexp_i_dn4 * assign15170_e23154) + (locals.var_mexp_i * (p.p450 * locals.var_deltemp_dn4))))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn5 * assign15170_e23136) + ((((locals.var_mexp_i_dn5 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn5 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn6 * assign15170_e23136) + ((((locals.var_mexp_i_dn6 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn6 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn7 * assign15170_e23136) + ((((locals.var_mexp_i_dn7 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn7 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn8 * assign15170_e23136) + ((((locals.var_mexp_i_dn8 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn8 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn9 * assign15170_e23136) + ((((locals.var_mexp_i_dn9 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn9 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn10 * assign15170_e23136) + ((((locals.var_mexp_i_dn10 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn10 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn11 * assign15170_e23136) + ((((locals.var_mexp_i_dn11 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn11 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn13 * assign15170_e23136) + ((((locals.var_mexp_i_dn13 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn13 * assign15170_e23154))) / (2.0 * assign15170_e23165)))), (0.5 * ((locals.var_mexp_i_dn14 * assign15170_e23136) + ((((locals.var_mexp_i_dn14 * assign15170_e23145) * assign15170_e23157) + (assign15170_e23148 * (locals.var_mexp_i_dn14 * assign15170_e23154))) / (2.0 * assign15170_e23165)))),)
            } else {
                let assign15170_e23172: f64 = (p.p450 * locals.var_deltemp);
                let assign15170_e23173: f64 = (1.0 + assign15170_e23172);
                let assign15170_e23174: f64 = (locals.var_mexp_i * assign15170_e23173);
                let assign15170_e23176: f64 = (assign15170_e23174 - 2.0);
                let assign15170_e23178: f64 = (-10000.0);
                let assign15170_e23180: f64 = (assign15170_e23178 * 0.001);
                let (assign15170_e23197, assign15170_e23197_d_n0, assign15170_e23197_d_n2, assign15170_e23197_d_n3, assign15170_e23197_d_n4, assign15170_e23197_d_n5, assign15170_e23197_d_n6, assign15170_e23197_d_n7, assign15170_e23197_d_n8, assign15170_e23197_d_n9, assign15170_e23197_d_n10, assign15170_e23197_d_n11, assign15170_e23197_d_n13, assign15170_e23197_d_n14,) = {
                    if (assign15170_e23176 < assign15170_e23180) {
                        let assign15170_e23183: f64 = (-0.001);
                        let assign15170_e23185: f64 = (assign15170_e23183 * 0.001);
                        let assign15170_e23190: f64 = (p.p450 * locals.var_deltemp);
                        let assign15170_e23191: f64 = (1.0 + assign15170_e23190);
                        let assign15170_e23192: f64 = (locals.var_mexp_i * assign15170_e23191);
                        let assign15170_e23194: f64 = (assign15170_e23192 - 2.0);
                        let assign15170_e23195: f64 = (assign15170_e23185 / assign15170_e23194);
                        (assign15170_e23195, (-((assign15170_e23185 * (locals.var_mexp_i_dn0 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn2 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn3 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * ((locals.var_mexp_i_dn4 * assign15170_e23191) + (locals.var_mexp_i * (p.p450 * locals.var_deltemp_dn4)))) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn5 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn6 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn7 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn8 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn9 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn10 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn11 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn13 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))), (-((assign15170_e23185 * (locals.var_mexp_i_dn14 * assign15170_e23191)) / (assign15170_e23194 * assign15170_e23194))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign15170_e23197, assign15170_e23197_d_n0, assign15170_e23197_d_n2, assign15170_e23197_d_n3, assign15170_e23197_d_n4, assign15170_e23197_d_n5, assign15170_e23197_d_n6, assign15170_e23197_d_n7, assign15170_e23197_d_n8, assign15170_e23197_d_n9, assign15170_e23197_d_n10, assign15170_e23197_d_n11, assign15170_e23197_d_n13, assign15170_e23197_d_n14,)
            }
        };
        let assign15170_e23200: f64 = (assign15170_e23198 + 2.0);
        (assign15170_e23200, assign15170_e23198_d_n0, assign15170_e23198_d_n2, assign15170_e23198_d_n3, assign15170_e23198_d_n4, assign15170_e23198_d_n5, assign15170_e23198_d_n6, assign15170_e23198_d_n7, assign15170_e23198_d_n8, assign15170_e23198_d_n9, assign15170_e23198_d_n10, assign15170_e23198_d_n11, assign15170_e23198_d_n13, assign15170_e23198_d_n14,)
    } else {
        (locals.var_mexp_t, locals.var_mexp_t_dn0, locals.var_mexp_t_dn2, locals.var_mexp_t_dn3, locals.var_mexp_t_dn4, locals.var_mexp_t_dn5, locals.var_mexp_t_dn6, locals.var_mexp_t_dn7, locals.var_mexp_t_dn8, locals.var_mexp_t_dn9, locals.var_mexp_t_dn10, locals.var_mexp_t_dn11, locals.var_mexp_t_dn13, locals.var_mexp_t_dn14,)
    }
};
        locals.var_mexp_t = assign15170_e23202;
        locals.var_mexp_t_dn0 = assign15170_e23202_d_n0;
        locals.var_mexp_t_dn2 = assign15170_e23202_d_n2;
        locals.var_mexp_t_dn3 = assign15170_e23202_d_n3;
        locals.var_mexp_t_dn4 = assign15170_e23202_d_n4;
        locals.var_mexp_t_dn5 = assign15170_e23202_d_n5;
        locals.var_mexp_t_dn6 = assign15170_e23202_d_n6;
        locals.var_mexp_t_dn7 = assign15170_e23202_d_n7;
        locals.var_mexp_t_dn8 = assign15170_e23202_d_n8;
        locals.var_mexp_t_dn9 = assign15170_e23202_d_n9;
        locals.var_mexp_t_dn10 = assign15170_e23202_d_n10;
        locals.var_mexp_t_dn11 = assign15170_e23202_d_n11;
        locals.var_mexp_t_dn13 = assign15170_e23202_d_n13;
        locals.var_mexp_t_dn14 = assign15170_e23202_d_n14;

        let assign15180_e23205: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard258 = assign15180_e23205;

    }

    pub(super) fn stamp_transient_block_44(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15190_e23296, assign15190_e23296_d_n0, assign15190_e23296_d_n2, assign15190_e23296_d_n3, assign15190_e23296_d_n4, assign15190_e23296_d_n5, assign15190_e23296_d_n6, assign15190_e23296_d_n7, assign15190_e23296_d_n8, assign15190_e23296_d_n9, assign15190_e23296_d_n10, assign15190_e23296_d_n11, assign15190_e23296_d_n13, assign15190_e23296_d_n14,) = {
    if ((locals.var_guard244 != 0.0) && (locals.var_guard258 != 0.0)) {
        let assign15190_e23213: f64 = (p.p452 * locals.var_deltemp);
        let assign15190_e23214: f64 = (1.0 + assign15190_e23213);
        let assign15190_e23215: f64 = (locals.var_mexpr_i * assign15190_e23214);
        let assign15190_e23217: f64 = (assign15190_e23215 - 2.0);
        let assign15190_e23219: f64 = (-10000.0);
        let assign15190_e23221: f64 = (assign15190_e23219 * 0.001);
        let (assign15190_e23292, assign15190_e23292_d_n0, assign15190_e23292_d_n2, assign15190_e23292_d_n3, assign15190_e23292_d_n4, assign15190_e23292_d_n5, assign15190_e23292_d_n6, assign15190_e23292_d_n7, assign15190_e23292_d_n8, assign15190_e23292_d_n9, assign15190_e23292_d_n10, assign15190_e23292_d_n11, assign15190_e23292_d_n13, assign15190_e23292_d_n14,) = {
            if (!(assign15190_e23217 < assign15190_e23221)) {
                let assign15190_e23229: f64 = (p.p452 * locals.var_deltemp);
                let assign15190_e23230: f64 = (1.0 + assign15190_e23229);
                let assign15190_e23231: f64 = (locals.var_mexpr_i * assign15190_e23230);
                let assign15190_e23233: f64 = (assign15190_e23231 - 2.0);
                let assign15190_e23238: f64 = (p.p452 * locals.var_deltemp);
                let assign15190_e23239: f64 = (1.0 + assign15190_e23238);
                let assign15190_e23240: f64 = (locals.var_mexpr_i * assign15190_e23239);
                let assign15190_e23242: f64 = (assign15190_e23240 - 2.0);
                let assign15190_e23247: f64 = (p.p452 * locals.var_deltemp);
                let assign15190_e23248: f64 = (1.0 + assign15190_e23247);
                let assign15190_e23249: f64 = (locals.var_mexpr_i * assign15190_e23248);
                let assign15190_e23251: f64 = (assign15190_e23249 - 2.0);
                let assign15190_e23252: f64 = (assign15190_e23242 * assign15190_e23251);
                let assign15190_e23255: f64 = (4.0 * 0.001);
                let assign15190_e23257: f64 = (assign15190_e23255 * 0.001);
                let assign15190_e23258: f64 = (assign15190_e23252 + assign15190_e23257);
                let assign15190_e23259: f64 = (assign15190_e23258).sqrt();
                let assign15190_e23260: f64 = (assign15190_e23233 + assign15190_e23259);
                let assign15190_e23261: f64 = (0.5 * assign15190_e23260);
                (assign15190_e23261, (0.5 * ((locals.var_mexpr_i_dn0 * assign15190_e23230) + ((((locals.var_mexpr_i_dn0 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn0 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn2 * assign15190_e23230) + ((((locals.var_mexpr_i_dn2 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn2 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn3 * assign15190_e23230) + ((((locals.var_mexpr_i_dn3 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn3 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * (((locals.var_mexpr_i_dn4 * assign15190_e23230) + (locals.var_mexpr_i * (p.p452 * locals.var_deltemp_dn4))) + (((((locals.var_mexpr_i_dn4 * assign15190_e23239) + (locals.var_mexpr_i * (p.p452 * locals.var_deltemp_dn4))) * assign15190_e23251) + (assign15190_e23242 * ((locals.var_mexpr_i_dn4 * assign15190_e23248) + (locals.var_mexpr_i * (p.p452 * locals.var_deltemp_dn4))))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn5 * assign15190_e23230) + ((((locals.var_mexpr_i_dn5 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn5 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn6 * assign15190_e23230) + ((((locals.var_mexpr_i_dn6 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn6 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn7 * assign15190_e23230) + ((((locals.var_mexpr_i_dn7 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn7 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn8 * assign15190_e23230) + ((((locals.var_mexpr_i_dn8 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn8 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn9 * assign15190_e23230) + ((((locals.var_mexpr_i_dn9 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn9 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn10 * assign15190_e23230) + ((((locals.var_mexpr_i_dn10 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn10 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn11 * assign15190_e23230) + ((((locals.var_mexpr_i_dn11 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn11 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn13 * assign15190_e23230) + ((((locals.var_mexpr_i_dn13 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn13 * assign15190_e23248))) / (2.0 * assign15190_e23259)))), (0.5 * ((locals.var_mexpr_i_dn14 * assign15190_e23230) + ((((locals.var_mexpr_i_dn14 * assign15190_e23239) * assign15190_e23251) + (assign15190_e23242 * (locals.var_mexpr_i_dn14 * assign15190_e23248))) / (2.0 * assign15190_e23259)))),)
            } else {
                let assign15190_e23266: f64 = (p.p452 * locals.var_deltemp);
                let assign15190_e23267: f64 = (1.0 + assign15190_e23266);
                let assign15190_e23268: f64 = (locals.var_mexpr_i * assign15190_e23267);
                let assign15190_e23270: f64 = (assign15190_e23268 - 2.0);
                let assign15190_e23272: f64 = (-10000.0);
                let assign15190_e23274: f64 = (assign15190_e23272 * 0.001);
                let (assign15190_e23291, assign15190_e23291_d_n0, assign15190_e23291_d_n2, assign15190_e23291_d_n3, assign15190_e23291_d_n4, assign15190_e23291_d_n5, assign15190_e23291_d_n6, assign15190_e23291_d_n7, assign15190_e23291_d_n8, assign15190_e23291_d_n9, assign15190_e23291_d_n10, assign15190_e23291_d_n11, assign15190_e23291_d_n13, assign15190_e23291_d_n14,) = {
                    if (assign15190_e23270 < assign15190_e23274) {
                        let assign15190_e23277: f64 = (-0.001);
                        let assign15190_e23279: f64 = (assign15190_e23277 * 0.001);
                        let assign15190_e23284: f64 = (p.p452 * locals.var_deltemp);
                        let assign15190_e23285: f64 = (1.0 + assign15190_e23284);
                        let assign15190_e23286: f64 = (locals.var_mexpr_i * assign15190_e23285);
                        let assign15190_e23288: f64 = (assign15190_e23286 - 2.0);
                        let assign15190_e23289: f64 = (assign15190_e23279 / assign15190_e23288);
                        (assign15190_e23289, (-((assign15190_e23279 * (locals.var_mexpr_i_dn0 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn2 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn3 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * ((locals.var_mexpr_i_dn4 * assign15190_e23285) + (locals.var_mexpr_i * (p.p452 * locals.var_deltemp_dn4)))) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn5 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn6 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn7 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn8 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn9 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn10 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn11 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn13 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))), (-((assign15190_e23279 * (locals.var_mexpr_i_dn14 * assign15190_e23285)) / (assign15190_e23288 * assign15190_e23288))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign15190_e23291, assign15190_e23291_d_n0, assign15190_e23291_d_n2, assign15190_e23291_d_n3, assign15190_e23291_d_n4, assign15190_e23291_d_n5, assign15190_e23291_d_n6, assign15190_e23291_d_n7, assign15190_e23291_d_n8, assign15190_e23291_d_n9, assign15190_e23291_d_n10, assign15190_e23291_d_n11, assign15190_e23291_d_n13, assign15190_e23291_d_n14,)
            }
        };
        let assign15190_e23294: f64 = (assign15190_e23292 + 2.0);
        (assign15190_e23294, assign15190_e23292_d_n0, assign15190_e23292_d_n2, assign15190_e23292_d_n3, assign15190_e23292_d_n4, assign15190_e23292_d_n5, assign15190_e23292_d_n6, assign15190_e23292_d_n7, assign15190_e23292_d_n8, assign15190_e23292_d_n9, assign15190_e23292_d_n10, assign15190_e23292_d_n11, assign15190_e23292_d_n13, assign15190_e23292_d_n14,)
    } else {
        (locals.var_mexpr_t, locals.var_mexpr_t_dn0, locals.var_mexpr_t_dn2, locals.var_mexpr_t_dn3, locals.var_mexpr_t_dn4, locals.var_mexpr_t_dn5, locals.var_mexpr_t_dn6, locals.var_mexpr_t_dn7, locals.var_mexpr_t_dn8, locals.var_mexpr_t_dn9, locals.var_mexpr_t_dn10, locals.var_mexpr_t_dn11, locals.var_mexpr_t_dn13, locals.var_mexpr_t_dn14,)
    }
};
        locals.var_mexpr_t = assign15190_e23296;
        locals.var_mexpr_t_dn0 = assign15190_e23296_d_n0;
        locals.var_mexpr_t_dn2 = assign15190_e23296_d_n2;
        locals.var_mexpr_t_dn3 = assign15190_e23296_d_n3;
        locals.var_mexpr_t_dn4 = assign15190_e23296_d_n4;
        locals.var_mexpr_t_dn5 = assign15190_e23296_d_n5;
        locals.var_mexpr_t_dn6 = assign15190_e23296_d_n6;
        locals.var_mexpr_t_dn7 = assign15190_e23296_d_n7;
        locals.var_mexpr_t_dn8 = assign15190_e23296_d_n8;
        locals.var_mexpr_t_dn9 = assign15190_e23296_d_n9;
        locals.var_mexpr_t_dn10 = assign15190_e23296_d_n10;
        locals.var_mexpr_t_dn11 = assign15190_e23296_d_n11;
        locals.var_mexpr_t_dn13 = assign15190_e23296_d_n13;
        locals.var_mexpr_t_dn14 = assign15190_e23296_d_n14;

        let (assign15200_e23300, assign15200_e23300_d_n4,) = {
    if (locals.var_guard244 != 0.0) {
        (locals.var_ksativ_i, 0.0,)
    } else {
        (locals.var_ksativ_t, locals.var_ksativ_t_dn4,)
    }
};
        locals.var_ksativ_t = assign15200_e23300;
        locals.var_ksativ_t_dn4 = assign15200_e23300_d_n4;

        let (assign15210_e23304, assign15210_e23304_d_n0, assign15210_e23304_d_n2, assign15210_e23304_d_n3, assign15210_e23304_d_n4, assign15210_e23304_d_n5, assign15210_e23304_d_n6, assign15210_e23304_d_n7, assign15210_e23304_d_n8, assign15210_e23304_d_n9, assign15210_e23304_d_n10, assign15210_e23304_d_n11, assign15210_e23304_d_n13, assign15210_e23304_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        (locals.var_pclm_i, locals.var_pclm_i_dn0, locals.var_pclm_i_dn2, locals.var_pclm_i_dn3, locals.var_pclm_i_dn4, locals.var_pclm_i_dn5, locals.var_pclm_i_dn6, locals.var_pclm_i_dn7, locals.var_pclm_i_dn8, locals.var_pclm_i_dn9, locals.var_pclm_i_dn10, locals.var_pclm_i_dn11, locals.var_pclm_i_dn13, locals.var_pclm_i_dn14,)
    } else {
        (locals.var_pclm_t, locals.var_pclm_t_dn0, locals.var_pclm_t_dn2, locals.var_pclm_t_dn3, locals.var_pclm_t_dn4, locals.var_pclm_t_dn5, locals.var_pclm_t_dn6, locals.var_pclm_t_dn7, locals.var_pclm_t_dn8, locals.var_pclm_t_dn9, locals.var_pclm_t_dn10, locals.var_pclm_t_dn11, locals.var_pclm_t_dn13, locals.var_pclm_t_dn14,)
    }
};
        locals.var_pclm_t = assign15210_e23304;
        locals.var_pclm_t_dn0 = assign15210_e23304_d_n0;
        locals.var_pclm_t_dn2 = assign15210_e23304_d_n2;
        locals.var_pclm_t_dn3 = assign15210_e23304_d_n3;
        locals.var_pclm_t_dn4 = assign15210_e23304_d_n4;
        locals.var_pclm_t_dn5 = assign15210_e23304_d_n5;
        locals.var_pclm_t_dn6 = assign15210_e23304_d_n6;
        locals.var_pclm_t_dn7 = assign15210_e23304_d_n7;
        locals.var_pclm_t_dn8 = assign15210_e23304_d_n8;
        locals.var_pclm_t_dn9 = assign15210_e23304_d_n9;
        locals.var_pclm_t_dn10 = assign15210_e23304_d_n10;
        locals.var_pclm_t_dn11 = assign15210_e23304_d_n11;
        locals.var_pclm_t_dn13 = assign15210_e23304_d_n13;
        locals.var_pclm_t_dn14 = assign15210_e23304_d_n14;

        let (assign15220_e23314, assign15220_e23314_d_n0, assign15220_e23314_d_n2, assign15220_e23314_d_n3, assign15220_e23314_d_n4, assign15220_e23314_d_n5, assign15220_e23314_d_n6, assign15220_e23314_d_n7, assign15220_e23314_d_n8, assign15220_e23314_d_n9, assign15220_e23314_d_n10, assign15220_e23314_d_n11, assign15220_e23314_d_n13, assign15220_e23314_d_n14,) = {
    if (locals.var_guard244 != 0.0) {
        let assign15220_e23309: f64 = (p.p1720 / locals.var_leff_1);
        let assign15220_e23310: f64 = (locals.var_kt1_i + assign15220_e23309);
        let assign15220_e23312: f64 = (assign15220_e23310 * locals.var_tratio_m1);
        (assign15220_e23312, ((-((p.p1720 * locals.var_leff_1_dn0) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn2) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn3) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), (((-((p.p1720 * locals.var_leff_1_dn4) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1) + (assign15220_e23310 * locals.var_tratio_m1_dn4)), ((-((p.p1720 * locals.var_leff_1_dn5) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn6) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn7) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn8) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn9) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn10) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn11) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn13) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1), ((-((p.p1720 * locals.var_leff_1_dn14) / (locals.var_leff_1 * locals.var_leff_1))) * locals.var_tratio_m1),)
    } else {
        (locals.var_dvth_temp, locals.var_dvth_temp_dn0, locals.var_dvth_temp_dn2, locals.var_dvth_temp_dn3, locals.var_dvth_temp_dn4, locals.var_dvth_temp_dn5, locals.var_dvth_temp_dn6, locals.var_dvth_temp_dn7, locals.var_dvth_temp_dn8, locals.var_dvth_temp_dn9, locals.var_dvth_temp_dn10, locals.var_dvth_temp_dn11, locals.var_dvth_temp_dn13, locals.var_dvth_temp_dn14,)
    }
};
        locals.var_dvth_temp = assign15220_e23314;
        locals.var_dvth_temp_dn0 = assign15220_e23314_d_n0;
        locals.var_dvth_temp_dn2 = assign15220_e23314_d_n2;
        locals.var_dvth_temp_dn3 = assign15220_e23314_d_n3;
        locals.var_dvth_temp_dn4 = assign15220_e23314_d_n4;
        locals.var_dvth_temp_dn5 = assign15220_e23314_d_n5;
        locals.var_dvth_temp_dn6 = assign15220_e23314_d_n6;
        locals.var_dvth_temp_dn7 = assign15220_e23314_d_n7;
        locals.var_dvth_temp_dn8 = assign15220_e23314_d_n8;
        locals.var_dvth_temp_dn9 = assign15220_e23314_d_n9;
        locals.var_dvth_temp_dn10 = assign15220_e23314_d_n10;
        locals.var_dvth_temp_dn11 = assign15220_e23314_d_n11;
        locals.var_dvth_temp_dn13 = assign15220_e23314_d_n13;
        locals.var_dvth_temp_dn14 = assign15220_e23314_d_n14;

        let assign15230_e23317: f64 = if p.p80 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard259 = assign15230_e23317;

        let (assign15240_e23333, assign15240_e23333_d_n0, assign15240_e23333_d_n2, assign15240_e23333_d_n3, assign15240_e23333_d_n4, assign15240_e23333_d_n5, assign15240_e23333_d_n6, assign15240_e23333_d_n7, assign15240_e23333_d_n8, assign15240_e23333_d_n9, assign15240_e23333_d_n10, assign15240_e23333_d_n11, assign15240_e23333_d_n13, assign15240_e23333_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15240_e23326: f64 = (locals.var_ute1_i * locals.var_tratio);
        let assign15240_e23327: f64 = (locals.var_ute_i + assign15240_e23326);
        let assign15240_e23329: f64 = (assign15240_e23327 * locals.var_trat_ln);
        let assign15240_e23330: f64 = (assign15240_e23329).exp();
        let assign15240_e23331: f64 = (locals.var_u0_i * assign15240_e23330);
        (assign15240_e23331, (locals.var_u0_i_dn0 * assign15240_e23330), (locals.var_u0_i_dn2 * assign15240_e23330), (locals.var_u0_i_dn3 * assign15240_e23330), ((locals.var_u0_i_dn4 * assign15240_e23330) + (locals.var_u0_i * (assign15240_e23330 * (((locals.var_ute1_i * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15240_e23327 * locals.var_trat_ln_dn4))))), (locals.var_u0_i_dn5 * assign15240_e23330), (locals.var_u0_i_dn6 * assign15240_e23330), (locals.var_u0_i_dn7 * assign15240_e23330), (locals.var_u0_i_dn8 * assign15240_e23330), (locals.var_u0_i_dn9 * assign15240_e23330), (locals.var_u0_i_dn10 * assign15240_e23330), (locals.var_u0_i_dn11 * assign15240_e23330), (locals.var_u0_i_dn13 * assign15240_e23330), (locals.var_u0_i_dn14 * assign15240_e23330),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15240_e23333;
        locals.var_t1_dn0 = assign15240_e23333_d_n0;
        locals.var_t1_dn2 = assign15240_e23333_d_n2;
        locals.var_t1_dn3 = assign15240_e23333_d_n3;
        locals.var_t1_dn4 = assign15240_e23333_d_n4;
        locals.var_t1_dn5 = assign15240_e23333_d_n5;
        locals.var_t1_dn6 = assign15240_e23333_d_n6;
        locals.var_t1_dn7 = assign15240_e23333_d_n7;
        locals.var_t1_dn8 = assign15240_e23333_d_n8;
        locals.var_t1_dn9 = assign15240_e23333_d_n9;
        locals.var_t1_dn10 = assign15240_e23333_d_n10;
        locals.var_t1_dn11 = assign15240_e23333_d_n11;
        locals.var_t1_dn13 = assign15240_e23333_d_n13;
        locals.var_t1_dn14 = assign15240_e23333_d_n14;

        let (assign15250_e23390, assign15250_e23390_d_n0, assign15250_e23390_d_n2, assign15250_e23390_d_n3, assign15250_e23390_d_n4, assign15250_e23390_d_n5, assign15250_e23390_d_n6, assign15250_e23390_d_n7, assign15250_e23390_d_n8, assign15250_e23390_d_n9, assign15250_e23390_d_n10, assign15250_e23390_d_n11, assign15250_e23390_d_n13, assign15250_e23390_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15250_e23340: f64 = (-0.9);
        let assign15250_e23342: f64 = (assign15250_e23340 * locals.var_t1);
        let assign15250_e23346: f64 = (locals.var_utl_i * locals.var_deltemp);
        let assign15250_e23348: f64 = (-0.9);
        let assign15250_e23350: f64 = (assign15250_e23348 * locals.var_t1);
        let assign15250_e23351: f64 = (assign15250_e23346 - assign15250_e23350);
        let assign15250_e23353: f64 = (assign15250_e23351 - 0.0001);
        let assign15250_e23356: f64 = (locals.var_utl_i * locals.var_deltemp);
        let assign15250_e23358: f64 = (-0.9);
        let assign15250_e23360: f64 = (assign15250_e23358 * locals.var_t1);
        let assign15250_e23361: f64 = (assign15250_e23356 - assign15250_e23360);
        let assign15250_e23363: f64 = (assign15250_e23361 - 0.0001);
        let assign15250_e23366: f64 = (locals.var_utl_i * locals.var_deltemp);
        let assign15250_e23368: f64 = (-0.9);
        let assign15250_e23370: f64 = (assign15250_e23368 * locals.var_t1);
        let assign15250_e23371: f64 = (assign15250_e23366 - assign15250_e23370);
        let assign15250_e23373: f64 = (assign15250_e23371 - 0.0001);
        let assign15250_e23374: f64 = (assign15250_e23363 * assign15250_e23373);
        let assign15250_e23377: f64 = (-0.9);
        let assign15250_e23379: f64 = (assign15250_e23377 * locals.var_t1);
        let assign15250_e23380: f64 = (4.0 * assign15250_e23379);
        let assign15250_e23382: f64 = (assign15250_e23380 * 0.0001);
        let assign15250_e23383: f64 = (assign15250_e23374 - assign15250_e23382);
        let assign15250_e23384: f64 = (assign15250_e23383).sqrt();
        let assign15250_e23385: f64 = (assign15250_e23353 + assign15250_e23384);
        let assign15250_e23386: f64 = (0.5 * assign15250_e23385);
        let assign15250_e23387: f64 = (assign15250_e23342 + assign15250_e23386);
        let assign15250_e23388: f64 = (locals.var_t1 + assign15250_e23387);
        (assign15250_e23388, (locals.var_t1_dn0 + ((assign15250_e23340 * locals.var_t1_dn0) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn0)) + (((((-(assign15250_e23358 * locals.var_t1_dn0)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn0)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn0)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn2 + ((assign15250_e23340 * locals.var_t1_dn2) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn2)) + (((((-(assign15250_e23358 * locals.var_t1_dn2)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn2)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn2)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn3 + ((assign15250_e23340 * locals.var_t1_dn3) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn3)) + (((((-(assign15250_e23358 * locals.var_t1_dn3)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn3)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn3)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn4 + ((assign15250_e23340 * locals.var_t1_dn4) + (0.5 * (((locals.var_utl_i * locals.var_deltemp_dn4) - (assign15250_e23348 * locals.var_t1_dn4)) + ((((((locals.var_utl_i * locals.var_deltemp_dn4) - (assign15250_e23358 * locals.var_t1_dn4)) * assign15250_e23373) + (assign15250_e23363 * ((locals.var_utl_i * locals.var_deltemp_dn4) - (assign15250_e23368 * locals.var_t1_dn4)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn4)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn5 + ((assign15250_e23340 * locals.var_t1_dn5) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn5)) + (((((-(assign15250_e23358 * locals.var_t1_dn5)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn5)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn5)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn6 + ((assign15250_e23340 * locals.var_t1_dn6) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn6)) + (((((-(assign15250_e23358 * locals.var_t1_dn6)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn6)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn6)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn7 + ((assign15250_e23340 * locals.var_t1_dn7) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn7)) + (((((-(assign15250_e23358 * locals.var_t1_dn7)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn7)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn7)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn8 + ((assign15250_e23340 * locals.var_t1_dn8) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn8)) + (((((-(assign15250_e23358 * locals.var_t1_dn8)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn8)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn8)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn9 + ((assign15250_e23340 * locals.var_t1_dn9) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn9)) + (((((-(assign15250_e23358 * locals.var_t1_dn9)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn9)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn9)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn10 + ((assign15250_e23340 * locals.var_t1_dn10) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn10)) + (((((-(assign15250_e23358 * locals.var_t1_dn10)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn10)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn10)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn11 + ((assign15250_e23340 * locals.var_t1_dn11) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn11)) + (((((-(assign15250_e23358 * locals.var_t1_dn11)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn11)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn11)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn13 + ((assign15250_e23340 * locals.var_t1_dn13) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn13)) + (((((-(assign15250_e23358 * locals.var_t1_dn13)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn13)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn13)) * 0.0001)) / (2.0 * assign15250_e23384)))))), (locals.var_t1_dn14 + ((assign15250_e23340 * locals.var_t1_dn14) + (0.5 * ((-(assign15250_e23348 * locals.var_t1_dn14)) + (((((-(assign15250_e23358 * locals.var_t1_dn14)) * assign15250_e23373) + (assign15250_e23363 * (-(assign15250_e23368 * locals.var_t1_dn14)))) - ((4.0 * (assign15250_e23377 * locals.var_t1_dn14)) * 0.0001)) / (2.0 * assign15250_e23384)))))),)
    } else {
        (locals.var_u0_v, locals.var_u0_v_dn0, locals.var_u0_v_dn2, locals.var_u0_v_dn3, locals.var_u0_v_dn4, locals.var_u0_v_dn5, locals.var_u0_v_dn6, locals.var_u0_v_dn7, locals.var_u0_v_dn8, locals.var_u0_v_dn9, locals.var_u0_v_dn10, locals.var_u0_v_dn11, locals.var_u0_v_dn13, locals.var_u0_v_dn14,)
    }
};
        locals.var_u0_v = assign15250_e23390;
        locals.var_u0_v_dn0 = assign15250_e23390_d_n0;
        locals.var_u0_v_dn2 = assign15250_e23390_d_n2;
        locals.var_u0_v_dn3 = assign15250_e23390_d_n3;
        locals.var_u0_v_dn4 = assign15250_e23390_d_n4;
        locals.var_u0_v_dn5 = assign15250_e23390_d_n5;
        locals.var_u0_v_dn6 = assign15250_e23390_d_n6;
        locals.var_u0_v_dn7 = assign15250_e23390_d_n7;
        locals.var_u0_v_dn8 = assign15250_e23390_d_n8;
        locals.var_u0_v_dn9 = assign15250_e23390_d_n9;
        locals.var_u0_v_dn10 = assign15250_e23390_d_n10;
        locals.var_u0_v_dn11 = assign15250_e23390_d_n11;
        locals.var_u0_v_dn13 = assign15250_e23390_d_n13;
        locals.var_u0_v_dn14 = assign15250_e23390_d_n14;

        let assign15260_e23393: f64 = if p.p66 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard260 = assign15260_e23393;

        let (assign15270_e23411, assign15270_e23411_d_n0, assign15270_e23411_d_n2, assign15270_e23411_d_n3, assign15270_e23411_d_n4, assign15270_e23411_d_n5, assign15270_e23411_d_n6, assign15270_e23411_d_n7, assign15270_e23411_d_n8, assign15270_e23411_d_n9, assign15270_e23411_d_n10, assign15270_e23411_d_n11, assign15270_e23411_d_n13, assign15270_e23411_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard260 != 0.0)) {
        let assign15270_e23404: f64 = (locals.var_ute1_i * locals.var_tratio);
        let assign15270_e23405: f64 = (locals.var_uter_i + assign15270_e23404);
        let assign15270_e23407: f64 = (assign15270_e23405 * locals.var_trat_ln);
        let assign15270_e23408: f64 = (assign15270_e23407).exp();
        let assign15270_e23409: f64 = (locals.var_u0r_i * assign15270_e23408);
        (assign15270_e23409, (locals.var_u0r_i_dn0 * assign15270_e23408), (locals.var_u0r_i_dn2 * assign15270_e23408), (locals.var_u0r_i_dn3 * assign15270_e23408), ((locals.var_u0r_i_dn4 * assign15270_e23408) + (locals.var_u0r_i * (assign15270_e23408 * (((locals.var_ute1_i * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15270_e23405 * locals.var_trat_ln_dn4))))), (locals.var_u0r_i_dn5 * assign15270_e23408), (locals.var_u0r_i_dn6 * assign15270_e23408), (locals.var_u0r_i_dn7 * assign15270_e23408), (locals.var_u0r_i_dn8 * assign15270_e23408), (locals.var_u0r_i_dn9 * assign15270_e23408), (locals.var_u0r_i_dn10 * assign15270_e23408), (locals.var_u0r_i_dn11 * assign15270_e23408), (locals.var_u0r_i_dn13 * assign15270_e23408), (locals.var_u0r_i_dn14 * assign15270_e23408),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15270_e23411;
        locals.var_t1_dn0 = assign15270_e23411_d_n0;
        locals.var_t1_dn2 = assign15270_e23411_d_n2;
        locals.var_t1_dn3 = assign15270_e23411_d_n3;
        locals.var_t1_dn4 = assign15270_e23411_d_n4;
        locals.var_t1_dn5 = assign15270_e23411_d_n5;
        locals.var_t1_dn6 = assign15270_e23411_d_n6;
        locals.var_t1_dn7 = assign15270_e23411_d_n7;
        locals.var_t1_dn8 = assign15270_e23411_d_n8;
        locals.var_t1_dn9 = assign15270_e23411_d_n9;
        locals.var_t1_dn10 = assign15270_e23411_d_n10;
        locals.var_t1_dn11 = assign15270_e23411_d_n11;
        locals.var_t1_dn13 = assign15270_e23411_d_n13;
        locals.var_t1_dn14 = assign15270_e23411_d_n14;

        let (assign15280_e23470, assign15280_e23470_d_n0, assign15280_e23470_d_n2, assign15280_e23470_d_n3, assign15280_e23470_d_n4, assign15280_e23470_d_n5, assign15280_e23470_d_n6, assign15280_e23470_d_n7, assign15280_e23470_d_n8, assign15280_e23470_d_n9, assign15280_e23470_d_n10, assign15280_e23470_d_n11, assign15280_e23470_d_n13, assign15280_e23470_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard260 != 0.0)) {
        let assign15280_e23420: f64 = (-0.9);
        let assign15280_e23422: f64 = (assign15280_e23420 * locals.var_t1);
        let assign15280_e23426: f64 = (locals.var_utlr_i * locals.var_deltemp);
        let assign15280_e23428: f64 = (-0.9);
        let assign15280_e23430: f64 = (assign15280_e23428 * locals.var_t1);
        let assign15280_e23431: f64 = (assign15280_e23426 - assign15280_e23430);
        let assign15280_e23433: f64 = (assign15280_e23431 - 0.0001);
        let assign15280_e23436: f64 = (locals.var_utlr_i * locals.var_deltemp);
        let assign15280_e23438: f64 = (-0.9);
        let assign15280_e23440: f64 = (assign15280_e23438 * locals.var_t1);
        let assign15280_e23441: f64 = (assign15280_e23436 - assign15280_e23440);
        let assign15280_e23443: f64 = (assign15280_e23441 - 0.0001);
        let assign15280_e23446: f64 = (locals.var_utlr_i * locals.var_deltemp);
        let assign15280_e23448: f64 = (-0.9);
        let assign15280_e23450: f64 = (assign15280_e23448 * locals.var_t1);
        let assign15280_e23451: f64 = (assign15280_e23446 - assign15280_e23450);
        let assign15280_e23453: f64 = (assign15280_e23451 - 0.0001);
        let assign15280_e23454: f64 = (assign15280_e23443 * assign15280_e23453);
        let assign15280_e23457: f64 = (-0.9);
        let assign15280_e23459: f64 = (assign15280_e23457 * locals.var_t1);
        let assign15280_e23460: f64 = (4.0 * assign15280_e23459);
        let assign15280_e23462: f64 = (assign15280_e23460 * 0.0001);
        let assign15280_e23463: f64 = (assign15280_e23454 - assign15280_e23462);
        let assign15280_e23464: f64 = (assign15280_e23463).sqrt();
        let assign15280_e23465: f64 = (assign15280_e23433 + assign15280_e23464);
        let assign15280_e23466: f64 = (0.5 * assign15280_e23465);
        let assign15280_e23467: f64 = (assign15280_e23422 + assign15280_e23466);
        let assign15280_e23468: f64 = (locals.var_t1 + assign15280_e23467);
        (assign15280_e23468, (locals.var_t1_dn0 + ((assign15280_e23420 * locals.var_t1_dn0) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn0)) + (((((-(assign15280_e23438 * locals.var_t1_dn0)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn0)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn0)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn2 + ((assign15280_e23420 * locals.var_t1_dn2) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn2)) + (((((-(assign15280_e23438 * locals.var_t1_dn2)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn2)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn2)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn3 + ((assign15280_e23420 * locals.var_t1_dn3) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn3)) + (((((-(assign15280_e23438 * locals.var_t1_dn3)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn3)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn3)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn4 + ((assign15280_e23420 * locals.var_t1_dn4) + (0.5 * (((locals.var_utlr_i * locals.var_deltemp_dn4) - (assign15280_e23428 * locals.var_t1_dn4)) + ((((((locals.var_utlr_i * locals.var_deltemp_dn4) - (assign15280_e23438 * locals.var_t1_dn4)) * assign15280_e23453) + (assign15280_e23443 * ((locals.var_utlr_i * locals.var_deltemp_dn4) - (assign15280_e23448 * locals.var_t1_dn4)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn4)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn5 + ((assign15280_e23420 * locals.var_t1_dn5) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn5)) + (((((-(assign15280_e23438 * locals.var_t1_dn5)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn5)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn5)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn6 + ((assign15280_e23420 * locals.var_t1_dn6) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn6)) + (((((-(assign15280_e23438 * locals.var_t1_dn6)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn6)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn6)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn7 + ((assign15280_e23420 * locals.var_t1_dn7) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn7)) + (((((-(assign15280_e23438 * locals.var_t1_dn7)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn7)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn7)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn8 + ((assign15280_e23420 * locals.var_t1_dn8) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn8)) + (((((-(assign15280_e23438 * locals.var_t1_dn8)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn8)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn8)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn9 + ((assign15280_e23420 * locals.var_t1_dn9) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn9)) + (((((-(assign15280_e23438 * locals.var_t1_dn9)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn9)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn9)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn10 + ((assign15280_e23420 * locals.var_t1_dn10) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn10)) + (((((-(assign15280_e23438 * locals.var_t1_dn10)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn10)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn10)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn11 + ((assign15280_e23420 * locals.var_t1_dn11) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn11)) + (((((-(assign15280_e23438 * locals.var_t1_dn11)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn11)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn11)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn13 + ((assign15280_e23420 * locals.var_t1_dn13) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn13)) + (((((-(assign15280_e23438 * locals.var_t1_dn13)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn13)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn13)) * 0.0001)) / (2.0 * assign15280_e23464)))))), (locals.var_t1_dn14 + ((assign15280_e23420 * locals.var_t1_dn14) + (0.5 * ((-(assign15280_e23428 * locals.var_t1_dn14)) + (((((-(assign15280_e23438 * locals.var_t1_dn14)) * assign15280_e23453) + (assign15280_e23443 * (-(assign15280_e23448 * locals.var_t1_dn14)))) - ((4.0 * (assign15280_e23457 * locals.var_t1_dn14)) * 0.0001)) / (2.0 * assign15280_e23464)))))),)
    } else {
        (locals.var_u0r_t, locals.var_u0r_t_dn0, locals.var_u0r_t_dn2, locals.var_u0r_t_dn3, locals.var_u0r_t_dn4, locals.var_u0r_t_dn5, locals.var_u0r_t_dn6, locals.var_u0r_t_dn7, locals.var_u0r_t_dn8, locals.var_u0r_t_dn9, locals.var_u0r_t_dn10, locals.var_u0r_t_dn11, locals.var_u0r_t_dn13, locals.var_u0r_t_dn14,)
    }
};
        locals.var_u0r_t = assign15280_e23470;
        locals.var_u0r_t_dn0 = assign15280_e23470_d_n0;
        locals.var_u0r_t_dn2 = assign15280_e23470_d_n2;
        locals.var_u0r_t_dn3 = assign15280_e23470_d_n3;
        locals.var_u0r_t_dn4 = assign15280_e23470_d_n4;
        locals.var_u0r_t_dn5 = assign15280_e23470_d_n5;
        locals.var_u0r_t_dn6 = assign15280_e23470_d_n6;
        locals.var_u0r_t_dn7 = assign15280_e23470_d_n7;
        locals.var_u0r_t_dn8 = assign15280_e23470_d_n8;
        locals.var_u0r_t_dn9 = assign15280_e23470_d_n9;
        locals.var_u0r_t_dn10 = assign15280_e23470_d_n10;
        locals.var_u0r_t_dn11 = assign15280_e23470_d_n11;
        locals.var_u0r_t_dn13 = assign15280_e23470_d_n13;
        locals.var_u0r_t_dn14 = assign15280_e23470_d_n14;

        let (assign15290_e23479, assign15290_e23479_d_n0, assign15290_e23479_d_n2, assign15290_e23479_d_n3, assign15290_e23479_d_n4, assign15290_e23479_d_n5, assign15290_e23479_d_n6, assign15290_e23479_d_n7, assign15290_e23479_d_n8, assign15290_e23479_d_n9, assign15290_e23479_d_n10, assign15290_e23479_d_n11, assign15290_e23479_d_n13, assign15290_e23479_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard260 != 0.0)) {
        (locals.var_u0r_t, locals.var_u0r_t_dn0, locals.var_u0r_t_dn2, locals.var_u0r_t_dn3, locals.var_u0r_t_dn4, locals.var_u0r_t_dn5, locals.var_u0r_t_dn6, locals.var_u0r_t_dn7, locals.var_u0r_t_dn8, locals.var_u0r_t_dn9, locals.var_u0r_t_dn10, locals.var_u0r_t_dn11, locals.var_u0r_t_dn13, locals.var_u0r_t_dn14,)
    } else {
        (locals.var_u0r_v, locals.var_u0r_v_dn0, locals.var_u0r_v_dn2, locals.var_u0r_v_dn3, locals.var_u0r_v_dn4, locals.var_u0r_v_dn5, locals.var_u0r_v_dn6, locals.var_u0r_v_dn7, locals.var_u0r_v_dn8, locals.var_u0r_v_dn9, locals.var_u0r_v_dn10, locals.var_u0r_v_dn11, locals.var_u0r_v_dn13, locals.var_u0r_v_dn14,)
    }
};
        locals.var_u0r_v = assign15290_e23479;
        locals.var_u0r_v_dn0 = assign15290_e23479_d_n0;
        locals.var_u0r_v_dn2 = assign15290_e23479_d_n2;
        locals.var_u0r_v_dn3 = assign15290_e23479_d_n3;
        locals.var_u0r_v_dn4 = assign15290_e23479_d_n4;
        locals.var_u0r_v_dn5 = assign15290_e23479_d_n5;
        locals.var_u0r_v_dn6 = assign15290_e23479_d_n6;
        locals.var_u0r_v_dn7 = assign15290_e23479_d_n7;
        locals.var_u0r_v_dn8 = assign15290_e23479_d_n8;
        locals.var_u0r_v_dn9 = assign15290_e23479_d_n9;
        locals.var_u0r_v_dn10 = assign15290_e23479_d_n10;
        locals.var_u0r_v_dn11 = assign15290_e23479_d_n11;
        locals.var_u0r_v_dn13 = assign15290_e23479_d_n13;
        locals.var_u0r_v_dn14 = assign15290_e23479_d_n14;

        let (assign15300_e23495, assign15300_e23495_d_n0, assign15300_e23495_d_n2, assign15300_e23495_d_n3, assign15300_e23495_d_n4, assign15300_e23495_d_n5, assign15300_e23495_d_n6, assign15300_e23495_d_n7, assign15300_e23495_d_n8, assign15300_e23495_d_n9, assign15300_e23495_d_n10, assign15300_e23495_d_n11, assign15300_e23495_d_n13, assign15300_e23495_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15300_e23488: f64 = (locals.var_ua2_i * locals.var_tratio);
        let assign15300_e23489: f64 = (locals.var_ua1_i + assign15300_e23488);
        let assign15300_e23491: f64 = (assign15300_e23489 * locals.var_trat_ln);
        let assign15300_e23492: f64 = (assign15300_e23491).exp();
        let assign15300_e23493: f64 = (locals.var_ua_i * assign15300_e23492);
        (assign15300_e23493, (locals.var_ua_i_dn0 * assign15300_e23492), (locals.var_ua_i_dn2 * assign15300_e23492), (locals.var_ua_i_dn3 * assign15300_e23492), ((locals.var_ua_i_dn4 * assign15300_e23492) + (locals.var_ua_i * (assign15300_e23492 * (((locals.var_ua2_i * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15300_e23489 * locals.var_trat_ln_dn4))))), (locals.var_ua_i_dn5 * assign15300_e23492), (locals.var_ua_i_dn6 * assign15300_e23492), (locals.var_ua_i_dn7 * assign15300_e23492), (locals.var_ua_i_dn8 * assign15300_e23492), (locals.var_ua_i_dn9 * assign15300_e23492), (locals.var_ua_i_dn10 * assign15300_e23492), (locals.var_ua_i_dn11 * assign15300_e23492), (locals.var_ua_i_dn13 * assign15300_e23492), (locals.var_ua_i_dn14 * assign15300_e23492),)
    } else {
        (locals.var_ua_t, locals.var_ua_t_dn0, locals.var_ua_t_dn2, locals.var_ua_t_dn3, locals.var_ua_t_dn4, locals.var_ua_t_dn5, locals.var_ua_t_dn6, locals.var_ua_t_dn7, locals.var_ua_t_dn8, locals.var_ua_t_dn9, locals.var_ua_t_dn10, locals.var_ua_t_dn11, locals.var_ua_t_dn13, locals.var_ua_t_dn14,)
    }
};
        locals.var_ua_t = assign15300_e23495;
        locals.var_ua_t_dn0 = assign15300_e23495_d_n0;
        locals.var_ua_t_dn2 = assign15300_e23495_d_n2;
        locals.var_ua_t_dn3 = assign15300_e23495_d_n3;
        locals.var_ua_t_dn4 = assign15300_e23495_d_n4;
        locals.var_ua_t_dn5 = assign15300_e23495_d_n5;
        locals.var_ua_t_dn6 = assign15300_e23495_d_n6;
        locals.var_ua_t_dn7 = assign15300_e23495_d_n7;
        locals.var_ua_t_dn8 = assign15300_e23495_d_n8;
        locals.var_ua_t_dn9 = assign15300_e23495_d_n9;
        locals.var_ua_t_dn10 = assign15300_e23495_d_n10;
        locals.var_ua_t_dn11 = assign15300_e23495_d_n11;
        locals.var_ua_t_dn13 = assign15300_e23495_d_n13;
        locals.var_ua_t_dn14 = assign15300_e23495_d_n14;

        let assign15310_e23498: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard261 = assign15310_e23498;

        let (assign15320_e23516, assign15320_e23516_d_n0, assign15320_e23516_d_n2, assign15320_e23516_d_n3, assign15320_e23516_d_n4, assign15320_e23516_d_n5, assign15320_e23516_d_n6, assign15320_e23516_d_n7, assign15320_e23516_d_n8, assign15320_e23516_d_n9, assign15320_e23516_d_n10, assign15320_e23516_d_n11, assign15320_e23516_d_n13, assign15320_e23516_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard261 != 0.0)) {
        let assign15320_e23509: f64 = (locals.var_ua2_i * locals.var_tratio);
        let assign15320_e23510: f64 = (locals.var_ua1r_i + assign15320_e23509);
        let assign15320_e23512: f64 = (assign15320_e23510 * locals.var_trat_ln);
        let assign15320_e23513: f64 = (assign15320_e23512).exp();
        let assign15320_e23514: f64 = (locals.var_uar_i * assign15320_e23513);
        (assign15320_e23514, (locals.var_uar_i_dn0 * assign15320_e23513), (locals.var_uar_i_dn2 * assign15320_e23513), (locals.var_uar_i_dn3 * assign15320_e23513), ((locals.var_uar_i_dn4 * assign15320_e23513) + (locals.var_uar_i * (assign15320_e23513 * (((locals.var_ua2_i * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15320_e23510 * locals.var_trat_ln_dn4))))), (locals.var_uar_i_dn5 * assign15320_e23513), (locals.var_uar_i_dn6 * assign15320_e23513), (locals.var_uar_i_dn7 * assign15320_e23513), (locals.var_uar_i_dn8 * assign15320_e23513), (locals.var_uar_i_dn9 * assign15320_e23513), (locals.var_uar_i_dn10 * assign15320_e23513), (locals.var_uar_i_dn11 * assign15320_e23513), (locals.var_uar_i_dn13 * assign15320_e23513), (locals.var_uar_i_dn14 * assign15320_e23513),)
    } else {
        (locals.var_uar_t, locals.var_uar_t_dn0, locals.var_uar_t_dn2, locals.var_uar_t_dn3, locals.var_uar_t_dn4, locals.var_uar_t_dn5, locals.var_uar_t_dn6, locals.var_uar_t_dn7, locals.var_uar_t_dn8, locals.var_uar_t_dn9, locals.var_uar_t_dn10, locals.var_uar_t_dn11, locals.var_uar_t_dn13, locals.var_uar_t_dn14,)
    }
};
        locals.var_uar_t = assign15320_e23516;
        locals.var_uar_t_dn0 = assign15320_e23516_d_n0;
        locals.var_uar_t_dn2 = assign15320_e23516_d_n2;
        locals.var_uar_t_dn3 = assign15320_e23516_d_n3;
        locals.var_uar_t_dn4 = assign15320_e23516_d_n4;
        locals.var_uar_t_dn5 = assign15320_e23516_d_n5;
        locals.var_uar_t_dn6 = assign15320_e23516_d_n6;
        locals.var_uar_t_dn7 = assign15320_e23516_d_n7;
        locals.var_uar_t_dn8 = assign15320_e23516_d_n8;
        locals.var_uar_t_dn9 = assign15320_e23516_d_n9;
        locals.var_uar_t_dn10 = assign15320_e23516_d_n10;
        locals.var_uar_t_dn11 = assign15320_e23516_d_n11;
        locals.var_uar_t_dn13 = assign15320_e23516_d_n13;
        locals.var_uar_t_dn14 = assign15320_e23516_d_n14;

        let (assign15330_e23532, assign15330_e23532_d_n0, assign15330_e23532_d_n2, assign15330_e23532_d_n3, assign15330_e23532_d_n4, assign15330_e23532_d_n5, assign15330_e23532_d_n6, assign15330_e23532_d_n7, assign15330_e23532_d_n8, assign15330_e23532_d_n9, assign15330_e23532_d_n10, assign15330_e23532_d_n11, assign15330_e23532_d_n13, assign15330_e23532_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15330_e23525: f64 = (locals.var_ud2_i * locals.var_tratio);
        let assign15330_e23526: f64 = (locals.var_ud1_i + assign15330_e23525);
        let assign15330_e23528: f64 = (assign15330_e23526 * locals.var_trat_ln);
        let assign15330_e23529: f64 = (assign15330_e23528).exp();
        let assign15330_e23530: f64 = (locals.var_ud_i * assign15330_e23529);
        (assign15330_e23530, (locals.var_ud_i_dn0 * assign15330_e23529), (locals.var_ud_i_dn2 * assign15330_e23529), (locals.var_ud_i_dn3 * assign15330_e23529), ((locals.var_ud_i_dn4 * assign15330_e23529) + (locals.var_ud_i * (assign15330_e23529 * (((locals.var_ud2_i * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15330_e23526 * locals.var_trat_ln_dn4))))), (locals.var_ud_i_dn5 * assign15330_e23529), (locals.var_ud_i_dn6 * assign15330_e23529), (locals.var_ud_i_dn7 * assign15330_e23529), (locals.var_ud_i_dn8 * assign15330_e23529), (locals.var_ud_i_dn9 * assign15330_e23529), (locals.var_ud_i_dn10 * assign15330_e23529), (locals.var_ud_i_dn11 * assign15330_e23529), (locals.var_ud_i_dn13 * assign15330_e23529), (locals.var_ud_i_dn14 * assign15330_e23529),)
    } else {
        (locals.var_ud_t, locals.var_ud_t_dn0, locals.var_ud_t_dn2, locals.var_ud_t_dn3, locals.var_ud_t_dn4, locals.var_ud_t_dn5, locals.var_ud_t_dn6, locals.var_ud_t_dn7, locals.var_ud_t_dn8, locals.var_ud_t_dn9, locals.var_ud_t_dn10, locals.var_ud_t_dn11, locals.var_ud_t_dn13, locals.var_ud_t_dn14,)
    }
};
        locals.var_ud_t = assign15330_e23532;
        locals.var_ud_t_dn0 = assign15330_e23532_d_n0;
        locals.var_ud_t_dn2 = assign15330_e23532_d_n2;
        locals.var_ud_t_dn3 = assign15330_e23532_d_n3;
        locals.var_ud_t_dn4 = assign15330_e23532_d_n4;
        locals.var_ud_t_dn5 = assign15330_e23532_d_n5;
        locals.var_ud_t_dn6 = assign15330_e23532_d_n6;
        locals.var_ud_t_dn7 = assign15330_e23532_d_n7;
        locals.var_ud_t_dn8 = assign15330_e23532_d_n8;
        locals.var_ud_t_dn9 = assign15330_e23532_d_n9;
        locals.var_ud_t_dn10 = assign15330_e23532_d_n10;
        locals.var_ud_t_dn11 = assign15330_e23532_d_n11;
        locals.var_ud_t_dn13 = assign15330_e23532_d_n13;
        locals.var_ud_t_dn14 = assign15330_e23532_d_n14;

        let assign15340_e23535: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard262 = assign15340_e23535;

        let (assign15350_e23553, assign15350_e23553_d_n0, assign15350_e23553_d_n2, assign15350_e23553_d_n3, assign15350_e23553_d_n4, assign15350_e23553_d_n5, assign15350_e23553_d_n6, assign15350_e23553_d_n7, assign15350_e23553_d_n8, assign15350_e23553_d_n9, assign15350_e23553_d_n10, assign15350_e23553_d_n11, assign15350_e23553_d_n13, assign15350_e23553_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard262 != 0.0)) {
        let assign15350_e23546: f64 = (locals.var_ud2_i * locals.var_tratio);
        let assign15350_e23547: f64 = (locals.var_ud1r_i + assign15350_e23546);
        let assign15350_e23549: f64 = (assign15350_e23547 * locals.var_trat_ln);
        let assign15350_e23550: f64 = (assign15350_e23549).exp();
        let assign15350_e23551: f64 = (locals.var_udr_i * assign15350_e23550);
        (assign15350_e23551, (locals.var_udr_i_dn0 * assign15350_e23550), (locals.var_udr_i_dn2 * assign15350_e23550), (locals.var_udr_i_dn3 * assign15350_e23550), ((locals.var_udr_i_dn4 * assign15350_e23550) + (locals.var_udr_i * (assign15350_e23550 * (((locals.var_ud2_i * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15350_e23547 * locals.var_trat_ln_dn4))))), (locals.var_udr_i_dn5 * assign15350_e23550), (locals.var_udr_i_dn6 * assign15350_e23550), (locals.var_udr_i_dn7 * assign15350_e23550), (locals.var_udr_i_dn8 * assign15350_e23550), (locals.var_udr_i_dn9 * assign15350_e23550), (locals.var_udr_i_dn10 * assign15350_e23550), (locals.var_udr_i_dn11 * assign15350_e23550), (locals.var_udr_i_dn13 * assign15350_e23550), (locals.var_udr_i_dn14 * assign15350_e23550),)
    } else {
        (locals.var_udr_t, locals.var_udr_t_dn0, locals.var_udr_t_dn2, locals.var_udr_t_dn3, locals.var_udr_t_dn4, locals.var_udr_t_dn5, locals.var_udr_t_dn6, locals.var_udr_t_dn7, locals.var_udr_t_dn8, locals.var_udr_t_dn9, locals.var_udr_t_dn10, locals.var_udr_t_dn11, locals.var_udr_t_dn13, locals.var_udr_t_dn14,)
    }
};
        locals.var_udr_t = assign15350_e23553;
        locals.var_udr_t_dn0 = assign15350_e23553_d_n0;
        locals.var_udr_t_dn2 = assign15350_e23553_d_n2;
        locals.var_udr_t_dn3 = assign15350_e23553_d_n3;
        locals.var_udr_t_dn4 = assign15350_e23553_d_n4;
        locals.var_udr_t_dn5 = assign15350_e23553_d_n5;
        locals.var_udr_t_dn6 = assign15350_e23553_d_n6;
        locals.var_udr_t_dn7 = assign15350_e23553_d_n7;
        locals.var_udr_t_dn8 = assign15350_e23553_d_n8;
        locals.var_udr_t_dn9 = assign15350_e23553_d_n9;
        locals.var_udr_t_dn10 = assign15350_e23553_d_n10;
        locals.var_udr_t_dn11 = assign15350_e23553_d_n11;
        locals.var_udr_t_dn13 = assign15350_e23553_d_n13;
        locals.var_udr_t_dn14 = assign15350_e23553_d_n14;

        let (assign15360_e23569, assign15360_e23569_d_n4,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15360_e23562: f64 = (p.p881 * locals.var_tratio);
        let assign15360_e23563: f64 = (locals.var_ucste_i + assign15360_e23562);
        let assign15360_e23565: f64 = (assign15360_e23563 * locals.var_trat_ln);
        let assign15360_e23566: f64 = (assign15360_e23565).exp();
        let assign15360_e23567: f64 = (locals.var_ucs_i * assign15360_e23566);
        (assign15360_e23567, (locals.var_ucs_i * (assign15360_e23566 * (((p.p881 * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15360_e23563 * locals.var_trat_ln_dn4)))),)
    } else {
        (locals.var_ucs_t, locals.var_ucs_t_dn4,)
    }
};
        locals.var_ucs_t = assign15360_e23569;
        locals.var_ucs_t_dn4 = assign15360_e23569_d_n4;

        let (assign15370_e23583, assign15370_e23583_d_n4,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15370_e23577: f64 = (locals.var_uds1_i * locals.var_tratio_m1);
        let assign15370_e23578: f64 = { let limited_exp_arg = assign15370_e23577; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign15370_e23580: f64 = (assign15370_e23578 - 1.0);
        let assign15370_e23581: f64 = (locals.var_uds_i * assign15370_e23580);
        (assign15370_e23581, (locals.var_uds_i * ({ let limited_exp_arg = assign15370_e23577; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_uds1_i * locals.var_tratio_m1_dn4))),)
    } else {
        (locals.var_uds_t, locals.var_uds_t_dn4,)
    }
};
        locals.var_uds_t = assign15370_e23583;
        locals.var_uds_t_dn4 = assign15370_e23583_d_n4;

        let (assign15380_e23597, assign15380_e23597_d_n4,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15380_e23591: f64 = (locals.var_udd1_i * locals.var_tratio_m1);
        let assign15380_e23592: f64 = { let limited_exp_arg = assign15380_e23591; if limited_exp_arg > 80.0 { LIMEXP_MAX * (1.0 + limited_exp_arg - 80.0) } else if limited_exp_arg < -80.0 { 1.804851387e-35 } else { limited_exp_arg.exp() } };
        let assign15380_e23594: f64 = (assign15380_e23592 - 1.0);
        let assign15380_e23595: f64 = (locals.var_udd_i * assign15380_e23594);
        (assign15380_e23595, (locals.var_udd_i * ({ let limited_exp_arg = assign15380_e23591; if limited_exp_arg > 80.0 { LIMEXP_MAX } else if limited_exp_arg < -80.0 { 0.0 } else { limited_exp_arg.exp() } } * (locals.var_udd1_i * locals.var_tratio_m1_dn4))),)
    } else {
        (locals.var_udd_t, locals.var_udd_t_dn4,)
    }
};
        locals.var_udd_t = assign15380_e23597;
        locals.var_udd_t_dn4 = assign15380_e23597_d_n4;

        let (assign15390_e23606, assign15390_e23606_d_n4,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15390_e23604: f64 = (0.5 + locals.var_uds_t);
        (assign15390_e23604, locals.var_uds_t_dn4,)
    } else {
        (locals.var_udseff_t, locals.var_udseff_t_dn4,)
    }
};
        locals.var_udseff_t = assign15390_e23606;
        locals.var_udseff_t_dn4 = assign15390_e23606_d_n4;

        let (assign15400_e23615, assign15400_e23615_d_n4,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15400_e23613: f64 = (0.5 + locals.var_udd_t);
        (assign15400_e23613, locals.var_udd_t_dn4,)
    } else {
        (locals.var_uddeff_t, locals.var_uddeff_t_dn4,)
    }
};
        locals.var_uddeff_t = assign15400_e23615;
        locals.var_uddeff_t_dn4 = assign15400_e23615_d_n4;

        let assign15410_e23618: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard263 = assign15410_e23618;

    }

    pub(super) fn stamp_transient_block_45(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15420_e23667, assign15420_e23667_d_n0, assign15420_e23667_d_n2, assign15420_e23667_d_n3, assign15420_e23667_d_n4, assign15420_e23667_d_n5, assign15420_e23667_d_n6, assign15420_e23667_d_n7, assign15420_e23667_d_n8, assign15420_e23667_d_n9, assign15420_e23667_d_n10, assign15420_e23667_d_n11, assign15420_e23667_d_n13, assign15420_e23667_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard263 != 0.0)) {
        let assign15420_e23627: f64 = (-locals.var_eu_i);
        let assign15420_e23631: f64 = (locals.var_eu1_i * locals.var_deltemp);
        let assign15420_e23633: f64 = (-locals.var_eu_i);
        let assign15420_e23634: f64 = (assign15420_e23631 - assign15420_e23633);
        let assign15420_e23636: f64 = (assign15420_e23634 - 1e-6);
        let assign15420_e23639: f64 = (locals.var_eu1_i * locals.var_deltemp);
        let assign15420_e23641: f64 = (-locals.var_eu_i);
        let assign15420_e23642: f64 = (assign15420_e23639 - assign15420_e23641);
        let assign15420_e23644: f64 = (assign15420_e23642 - 1e-6);
        let assign15420_e23647: f64 = (locals.var_eu1_i * locals.var_deltemp);
        let assign15420_e23649: f64 = (-locals.var_eu_i);
        let assign15420_e23650: f64 = (assign15420_e23647 - assign15420_e23649);
        let assign15420_e23652: f64 = (assign15420_e23650 - 1e-6);
        let assign15420_e23653: f64 = (assign15420_e23644 * assign15420_e23652);
        let assign15420_e23656: f64 = (-locals.var_eu_i);
        let assign15420_e23657: f64 = (4.0 * assign15420_e23656);
        let assign15420_e23659: f64 = (assign15420_e23657 * 1e-6);
        let assign15420_e23660: f64 = (assign15420_e23653 - assign15420_e23659);
        let assign15420_e23661: f64 = (assign15420_e23660).sqrt();
        let assign15420_e23662: f64 = (assign15420_e23636 + assign15420_e23661);
        let assign15420_e23663: f64 = (0.5 * assign15420_e23662);
        let assign15420_e23664: f64 = (assign15420_e23627 + assign15420_e23663);
        let assign15420_e23665: f64 = (locals.var_eu_i + assign15420_e23664);
        (assign15420_e23665, (locals.var_eu_i_dn0 + ((-locals.var_eu_i_dn0) + (0.5 * ((-(-locals.var_eu_i_dn0)) + (((((-(-locals.var_eu_i_dn0)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn0)))) - ((4.0 * (-locals.var_eu_i_dn0)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn2 + ((-locals.var_eu_i_dn2) + (0.5 * ((-(-locals.var_eu_i_dn2)) + (((((-(-locals.var_eu_i_dn2)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn2)))) - ((4.0 * (-locals.var_eu_i_dn2)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn3 + ((-locals.var_eu_i_dn3) + (0.5 * ((-(-locals.var_eu_i_dn3)) + (((((-(-locals.var_eu_i_dn3)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn3)))) - ((4.0 * (-locals.var_eu_i_dn3)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn4 + ((-locals.var_eu_i_dn4) + (0.5 * (((locals.var_eu1_i * locals.var_deltemp_dn4) - (-locals.var_eu_i_dn4)) + ((((((locals.var_eu1_i * locals.var_deltemp_dn4) - (-locals.var_eu_i_dn4)) * assign15420_e23652) + (assign15420_e23644 * ((locals.var_eu1_i * locals.var_deltemp_dn4) - (-locals.var_eu_i_dn4)))) - ((4.0 * (-locals.var_eu_i_dn4)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn5 + ((-locals.var_eu_i_dn5) + (0.5 * ((-(-locals.var_eu_i_dn5)) + (((((-(-locals.var_eu_i_dn5)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn5)))) - ((4.0 * (-locals.var_eu_i_dn5)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn6 + ((-locals.var_eu_i_dn6) + (0.5 * ((-(-locals.var_eu_i_dn6)) + (((((-(-locals.var_eu_i_dn6)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn6)))) - ((4.0 * (-locals.var_eu_i_dn6)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn7 + ((-locals.var_eu_i_dn7) + (0.5 * ((-(-locals.var_eu_i_dn7)) + (((((-(-locals.var_eu_i_dn7)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn7)))) - ((4.0 * (-locals.var_eu_i_dn7)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn8 + ((-locals.var_eu_i_dn8) + (0.5 * ((-(-locals.var_eu_i_dn8)) + (((((-(-locals.var_eu_i_dn8)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn8)))) - ((4.0 * (-locals.var_eu_i_dn8)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn9 + ((-locals.var_eu_i_dn9) + (0.5 * ((-(-locals.var_eu_i_dn9)) + (((((-(-locals.var_eu_i_dn9)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn9)))) - ((4.0 * (-locals.var_eu_i_dn9)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn10 + ((-locals.var_eu_i_dn10) + (0.5 * ((-(-locals.var_eu_i_dn10)) + (((((-(-locals.var_eu_i_dn10)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn10)))) - ((4.0 * (-locals.var_eu_i_dn10)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn11 + ((-locals.var_eu_i_dn11) + (0.5 * ((-(-locals.var_eu_i_dn11)) + (((((-(-locals.var_eu_i_dn11)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn11)))) - ((4.0 * (-locals.var_eu_i_dn11)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn13 + ((-locals.var_eu_i_dn13) + (0.5 * ((-(-locals.var_eu_i_dn13)) + (((((-(-locals.var_eu_i_dn13)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn13)))) - ((4.0 * (-locals.var_eu_i_dn13)) * 1e-6)) / (2.0 * assign15420_e23661)))))), (locals.var_eu_i_dn14 + ((-locals.var_eu_i_dn14) + (0.5 * ((-(-locals.var_eu_i_dn14)) + (((((-(-locals.var_eu_i_dn14)) * assign15420_e23652) + (assign15420_e23644 * (-(-locals.var_eu_i_dn14)))) - ((4.0 * (-locals.var_eu_i_dn14)) * 1e-6)) / (2.0 * assign15420_e23661)))))),)
    } else {
        (locals.var_eu_t, locals.var_eu_t_dn0, locals.var_eu_t_dn2, locals.var_eu_t_dn3, locals.var_eu_t_dn4, locals.var_eu_t_dn5, locals.var_eu_t_dn6, locals.var_eu_t_dn7, locals.var_eu_t_dn8, locals.var_eu_t_dn9, locals.var_eu_t_dn10, locals.var_eu_t_dn11, locals.var_eu_t_dn13, locals.var_eu_t_dn14,)
    }
};
        locals.var_eu_t = assign15420_e23667;
        locals.var_eu_t_dn0 = assign15420_e23667_d_n0;
        locals.var_eu_t_dn2 = assign15420_e23667_d_n2;
        locals.var_eu_t_dn3 = assign15420_e23667_d_n3;
        locals.var_eu_t_dn4 = assign15420_e23667_d_n4;
        locals.var_eu_t_dn5 = assign15420_e23667_d_n5;
        locals.var_eu_t_dn6 = assign15420_e23667_d_n6;
        locals.var_eu_t_dn7 = assign15420_e23667_d_n7;
        locals.var_eu_t_dn8 = assign15420_e23667_d_n8;
        locals.var_eu_t_dn9 = assign15420_e23667_d_n9;
        locals.var_eu_t_dn10 = assign15420_e23667_d_n10;
        locals.var_eu_t_dn11 = assign15420_e23667_d_n11;
        locals.var_eu_t_dn13 = assign15420_e23667_d_n13;
        locals.var_eu_t_dn14 = assign15420_e23667_d_n14;

        let (assign15430_e23750, assign15430_e23750_d_n0, assign15430_e23750_d_n2, assign15430_e23750_d_n3, assign15430_e23750_d_n4, assign15430_e23750_d_n5, assign15430_e23750_d_n6, assign15430_e23750_d_n7, assign15430_e23750_d_n8, assign15430_e23750_d_n9, assign15430_e23750_d_n10, assign15430_e23750_d_n11, assign15430_e23750_d_n13, assign15430_e23750_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard263 == 0.0)) {
        let assign15430_e23679: f64 = (locals.var_eu1_i * locals.var_deltemp);
        let assign15430_e23680: f64 = (1.0 + assign15430_e23679);
        let assign15430_e23682: f64 = (assign15430_e23680 - 1e-6);
        let assign15430_e23684: f64 = (-10000.0);
        let assign15430_e23686: f64 = (assign15430_e23684 * 0.001);
        let (assign15430_e23747, assign15430_e23747_d_n4,) = {
            if (!(assign15430_e23682 < assign15430_e23686)) {
                let assign15430_e23693: f64 = (locals.var_eu1_i * locals.var_deltemp);
                let assign15430_e23694: f64 = (1.0 + assign15430_e23693);
                let assign15430_e23696: f64 = (assign15430_e23694 - 1e-6);
                let assign15430_e23700: f64 = (locals.var_eu1_i * locals.var_deltemp);
                let assign15430_e23701: f64 = (1.0 + assign15430_e23700);
                let assign15430_e23703: f64 = (assign15430_e23701 - 1e-6);
                let assign15430_e23707: f64 = (locals.var_eu1_i * locals.var_deltemp);
                let assign15430_e23708: f64 = (1.0 + assign15430_e23707);
                let assign15430_e23710: f64 = (assign15430_e23708 - 1e-6);
                let assign15430_e23711: f64 = (assign15430_e23703 * assign15430_e23710);
                let assign15430_e23714: f64 = (4.0 * 0.001);
                let assign15430_e23716: f64 = (assign15430_e23714 * 0.001);
                let assign15430_e23717: f64 = (assign15430_e23711 + assign15430_e23716);
                let assign15430_e23718: f64 = (assign15430_e23717).sqrt();
                let assign15430_e23719: f64 = (assign15430_e23696 + assign15430_e23718);
                let assign15430_e23720: f64 = (0.5 * assign15430_e23719);
                (assign15430_e23720, (0.5 * ((locals.var_eu1_i * locals.var_deltemp_dn4) + ((((locals.var_eu1_i * locals.var_deltemp_dn4) * assign15430_e23710) + (assign15430_e23703 * (locals.var_eu1_i * locals.var_deltemp_dn4))) / (2.0 * assign15430_e23718)))),)
            } else {
                let assign15430_e23724: f64 = (locals.var_eu1_i * locals.var_deltemp);
                let assign15430_e23725: f64 = (1.0 + assign15430_e23724);
                let assign15430_e23727: f64 = (assign15430_e23725 - 1e-6);
                let assign15430_e23729: f64 = (-10000.0);
                let assign15430_e23731: f64 = (assign15430_e23729 * 0.001);
                let (assign15430_e23746, assign15430_e23746_d_n4,) = {
                    if (assign15430_e23727 < assign15430_e23731) {
                        let assign15430_e23734: f64 = (-0.001);
                        let assign15430_e23736: f64 = (assign15430_e23734 * 0.001);
                        let assign15430_e23740: f64 = (locals.var_eu1_i * locals.var_deltemp);
                        let assign15430_e23741: f64 = (1.0 + assign15430_e23740);
                        let assign15430_e23743: f64 = (assign15430_e23741 - 1e-6);
                        let assign15430_e23744: f64 = (assign15430_e23736 / assign15430_e23743);
                        (assign15430_e23744, (-((assign15430_e23736 * (locals.var_eu1_i * locals.var_deltemp_dn4)) / (assign15430_e23743 * assign15430_e23743))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15430_e23746, assign15430_e23746_d_n4,)
            }
        };
        let assign15430_e23748: f64 = (locals.var_eu_i * assign15430_e23747);
        (assign15430_e23748, (locals.var_eu_i_dn0 * assign15430_e23747), (locals.var_eu_i_dn2 * assign15430_e23747), (locals.var_eu_i_dn3 * assign15430_e23747), ((locals.var_eu_i_dn4 * assign15430_e23747) + (locals.var_eu_i * assign15430_e23747_d_n4)), (locals.var_eu_i_dn5 * assign15430_e23747), (locals.var_eu_i_dn6 * assign15430_e23747), (locals.var_eu_i_dn7 * assign15430_e23747), (locals.var_eu_i_dn8 * assign15430_e23747), (locals.var_eu_i_dn9 * assign15430_e23747), (locals.var_eu_i_dn10 * assign15430_e23747), (locals.var_eu_i_dn11 * assign15430_e23747), (locals.var_eu_i_dn13 * assign15430_e23747), (locals.var_eu_i_dn14 * assign15430_e23747),)
    } else {
        (locals.var_eu_t, locals.var_eu_t_dn0, locals.var_eu_t_dn2, locals.var_eu_t_dn3, locals.var_eu_t_dn4, locals.var_eu_t_dn5, locals.var_eu_t_dn6, locals.var_eu_t_dn7, locals.var_eu_t_dn8, locals.var_eu_t_dn9, locals.var_eu_t_dn10, locals.var_eu_t_dn11, locals.var_eu_t_dn13, locals.var_eu_t_dn14,)
    }
};
        locals.var_eu_t = assign15430_e23750;
        locals.var_eu_t_dn0 = assign15430_e23750_d_n0;
        locals.var_eu_t_dn2 = assign15430_e23750_d_n2;
        locals.var_eu_t_dn3 = assign15430_e23750_d_n3;
        locals.var_eu_t_dn4 = assign15430_e23750_d_n4;
        locals.var_eu_t_dn5 = assign15430_e23750_d_n5;
        locals.var_eu_t_dn6 = assign15430_e23750_d_n6;
        locals.var_eu_t_dn7 = assign15430_e23750_d_n7;
        locals.var_eu_t_dn8 = assign15430_e23750_d_n8;
        locals.var_eu_t_dn9 = assign15430_e23750_d_n9;
        locals.var_eu_t_dn10 = assign15430_e23750_d_n10;
        locals.var_eu_t_dn11 = assign15430_e23750_d_n11;
        locals.var_eu_t_dn13 = assign15430_e23750_d_n13;
        locals.var_eu_t_dn14 = assign15430_e23750_d_n14;

        let assign15440_e23753: f64 = if p.p67 == 1.0 { 1.0 } else { 0.0 };
        locals.var_guard264 = assign15440_e23753;

        let (assign15450_e23771, assign15450_e23771_d_n0, assign15450_e23771_d_n2, assign15450_e23771_d_n3, assign15450_e23771_d_n4, assign15450_e23771_d_n5, assign15450_e23771_d_n6, assign15450_e23771_d_n7, assign15450_e23771_d_n8, assign15450_e23771_d_n9, assign15450_e23771_d_n10, assign15450_e23771_d_n11, assign15450_e23771_d_n13, assign15450_e23771_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard264 != 0.0)) {
        let assign15450_e23764: f64 = (locals.var_ute1cv_i * locals.var_tratio);
        let assign15450_e23765: f64 = (locals.var_utecv_i + assign15450_e23764);
        let assign15450_e23767: f64 = (assign15450_e23765 * locals.var_trat_ln);
        let assign15450_e23768: f64 = (assign15450_e23767).exp();
        let assign15450_e23769: f64 = (locals.var_u0cv_i * assign15450_e23768);
        (assign15450_e23769, (locals.var_u0cv_i_dn0 * assign15450_e23768), (locals.var_u0cv_i_dn2 * assign15450_e23768), (locals.var_u0cv_i_dn3 * assign15450_e23768), ((locals.var_u0cv_i_dn4 * assign15450_e23768) + (locals.var_u0cv_i * (assign15450_e23768 * (((locals.var_ute1cv_i * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15450_e23765 * locals.var_trat_ln_dn4))))), (locals.var_u0cv_i_dn5 * assign15450_e23768), (locals.var_u0cv_i_dn6 * assign15450_e23768), (locals.var_u0cv_i_dn7 * assign15450_e23768), (locals.var_u0cv_i_dn8 * assign15450_e23768), (locals.var_u0cv_i_dn9 * assign15450_e23768), (locals.var_u0cv_i_dn10 * assign15450_e23768), (locals.var_u0cv_i_dn11 * assign15450_e23768), (locals.var_u0cv_i_dn13 * assign15450_e23768), (locals.var_u0cv_i_dn14 * assign15450_e23768),)
    } else {
        (locals.var_t1, locals.var_t1_dn0, locals.var_t1_dn2, locals.var_t1_dn3, locals.var_t1_dn4, locals.var_t1_dn5, locals.var_t1_dn6, locals.var_t1_dn7, locals.var_t1_dn8, locals.var_t1_dn9, locals.var_t1_dn10, locals.var_t1_dn11, locals.var_t1_dn13, locals.var_t1_dn14,)
    }
};
        locals.var_t1 = assign15450_e23771;
        locals.var_t1_dn0 = assign15450_e23771_d_n0;
        locals.var_t1_dn2 = assign15450_e23771_d_n2;
        locals.var_t1_dn3 = assign15450_e23771_d_n3;
        locals.var_t1_dn4 = assign15450_e23771_d_n4;
        locals.var_t1_dn5 = assign15450_e23771_d_n5;
        locals.var_t1_dn6 = assign15450_e23771_d_n6;
        locals.var_t1_dn7 = assign15450_e23771_d_n7;
        locals.var_t1_dn8 = assign15450_e23771_d_n8;
        locals.var_t1_dn9 = assign15450_e23771_d_n9;
        locals.var_t1_dn10 = assign15450_e23771_d_n10;
        locals.var_t1_dn11 = assign15450_e23771_d_n11;
        locals.var_t1_dn13 = assign15450_e23771_d_n13;
        locals.var_t1_dn14 = assign15450_e23771_d_n14;

        let (assign15460_e23830, assign15460_e23830_d_n0, assign15460_e23830_d_n2, assign15460_e23830_d_n3, assign15460_e23830_d_n4, assign15460_e23830_d_n5, assign15460_e23830_d_n6, assign15460_e23830_d_n7, assign15460_e23830_d_n8, assign15460_e23830_d_n9, assign15460_e23830_d_n10, assign15460_e23830_d_n11, assign15460_e23830_d_n13, assign15460_e23830_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard264 != 0.0)) {
        let assign15460_e23780: f64 = (-0.9);
        let assign15460_e23782: f64 = (assign15460_e23780 * locals.var_t1);
        let assign15460_e23786: f64 = (locals.var_utlcv_i * locals.var_deltemp);
        let assign15460_e23788: f64 = (-0.9);
        let assign15460_e23790: f64 = (assign15460_e23788 * locals.var_t1);
        let assign15460_e23791: f64 = (assign15460_e23786 - assign15460_e23790);
        let assign15460_e23793: f64 = (assign15460_e23791 - 0.0001);
        let assign15460_e23796: f64 = (locals.var_utlcv_i * locals.var_deltemp);
        let assign15460_e23798: f64 = (-0.9);
        let assign15460_e23800: f64 = (assign15460_e23798 * locals.var_t1);
        let assign15460_e23801: f64 = (assign15460_e23796 - assign15460_e23800);
        let assign15460_e23803: f64 = (assign15460_e23801 - 0.0001);
        let assign15460_e23806: f64 = (locals.var_utlcv_i * locals.var_deltemp);
        let assign15460_e23808: f64 = (-0.9);
        let assign15460_e23810: f64 = (assign15460_e23808 * locals.var_t1);
        let assign15460_e23811: f64 = (assign15460_e23806 - assign15460_e23810);
        let assign15460_e23813: f64 = (assign15460_e23811 - 0.0001);
        let assign15460_e23814: f64 = (assign15460_e23803 * assign15460_e23813);
        let assign15460_e23817: f64 = (-0.9);
        let assign15460_e23819: f64 = (assign15460_e23817 * locals.var_t1);
        let assign15460_e23820: f64 = (4.0 * assign15460_e23819);
        let assign15460_e23822: f64 = (assign15460_e23820 * 0.0001);
        let assign15460_e23823: f64 = (assign15460_e23814 - assign15460_e23822);
        let assign15460_e23824: f64 = (assign15460_e23823).sqrt();
        let assign15460_e23825: f64 = (assign15460_e23793 + assign15460_e23824);
        let assign15460_e23826: f64 = (0.5 * assign15460_e23825);
        let assign15460_e23827: f64 = (assign15460_e23782 + assign15460_e23826);
        let assign15460_e23828: f64 = (locals.var_t1 + assign15460_e23827);
        (assign15460_e23828, (locals.var_t1_dn0 + ((assign15460_e23780 * locals.var_t1_dn0) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn0)) + (((((-(assign15460_e23798 * locals.var_t1_dn0)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn0)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn0)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn2 + ((assign15460_e23780 * locals.var_t1_dn2) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn2)) + (((((-(assign15460_e23798 * locals.var_t1_dn2)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn2)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn2)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn3 + ((assign15460_e23780 * locals.var_t1_dn3) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn3)) + (((((-(assign15460_e23798 * locals.var_t1_dn3)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn3)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn3)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn4 + ((assign15460_e23780 * locals.var_t1_dn4) + (0.5 * (((locals.var_utlcv_i * locals.var_deltemp_dn4) - (assign15460_e23788 * locals.var_t1_dn4)) + ((((((locals.var_utlcv_i * locals.var_deltemp_dn4) - (assign15460_e23798 * locals.var_t1_dn4)) * assign15460_e23813) + (assign15460_e23803 * ((locals.var_utlcv_i * locals.var_deltemp_dn4) - (assign15460_e23808 * locals.var_t1_dn4)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn4)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn5 + ((assign15460_e23780 * locals.var_t1_dn5) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn5)) + (((((-(assign15460_e23798 * locals.var_t1_dn5)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn5)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn5)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn6 + ((assign15460_e23780 * locals.var_t1_dn6) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn6)) + (((((-(assign15460_e23798 * locals.var_t1_dn6)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn6)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn6)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn7 + ((assign15460_e23780 * locals.var_t1_dn7) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn7)) + (((((-(assign15460_e23798 * locals.var_t1_dn7)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn7)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn7)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn8 + ((assign15460_e23780 * locals.var_t1_dn8) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn8)) + (((((-(assign15460_e23798 * locals.var_t1_dn8)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn8)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn8)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn9 + ((assign15460_e23780 * locals.var_t1_dn9) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn9)) + (((((-(assign15460_e23798 * locals.var_t1_dn9)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn9)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn9)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn10 + ((assign15460_e23780 * locals.var_t1_dn10) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn10)) + (((((-(assign15460_e23798 * locals.var_t1_dn10)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn10)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn10)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn11 + ((assign15460_e23780 * locals.var_t1_dn11) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn11)) + (((((-(assign15460_e23798 * locals.var_t1_dn11)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn11)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn11)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn13 + ((assign15460_e23780 * locals.var_t1_dn13) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn13)) + (((((-(assign15460_e23798 * locals.var_t1_dn13)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn13)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn13)) * 0.0001)) / (2.0 * assign15460_e23824)))))), (locals.var_t1_dn14 + ((assign15460_e23780 * locals.var_t1_dn14) + (0.5 * ((-(assign15460_e23788 * locals.var_t1_dn14)) + (((((-(assign15460_e23798 * locals.var_t1_dn14)) * assign15460_e23813) + (assign15460_e23803 * (-(assign15460_e23808 * locals.var_t1_dn14)))) - ((4.0 * (assign15460_e23817 * locals.var_t1_dn14)) * 0.0001)) / (2.0 * assign15460_e23824)))))),)
    } else {
        (locals.var_u0_cv, locals.var_u0_cv_dn0, locals.var_u0_cv_dn2, locals.var_u0_cv_dn3, locals.var_u0_cv_dn4, locals.var_u0_cv_dn5, locals.var_u0_cv_dn6, locals.var_u0_cv_dn7, locals.var_u0_cv_dn8, locals.var_u0_cv_dn9, locals.var_u0_cv_dn10, locals.var_u0_cv_dn11, locals.var_u0_cv_dn13, locals.var_u0_cv_dn14,)
    }
};
        locals.var_u0_cv = assign15460_e23830;
        locals.var_u0_cv_dn0 = assign15460_e23830_d_n0;
        locals.var_u0_cv_dn2 = assign15460_e23830_d_n2;
        locals.var_u0_cv_dn3 = assign15460_e23830_d_n3;
        locals.var_u0_cv_dn4 = assign15460_e23830_d_n4;
        locals.var_u0_cv_dn5 = assign15460_e23830_d_n5;
        locals.var_u0_cv_dn6 = assign15460_e23830_d_n6;
        locals.var_u0_cv_dn7 = assign15460_e23830_d_n7;
        locals.var_u0_cv_dn8 = assign15460_e23830_d_n8;
        locals.var_u0_cv_dn9 = assign15460_e23830_d_n9;
        locals.var_u0_cv_dn10 = assign15460_e23830_d_n10;
        locals.var_u0_cv_dn11 = assign15460_e23830_d_n11;
        locals.var_u0_cv_dn13 = assign15460_e23830_d_n13;
        locals.var_u0_cv_dn14 = assign15460_e23830_d_n14;

        let (assign15470_e23848, assign15470_e23848_d_n0, assign15470_e23848_d_n2, assign15470_e23848_d_n3, assign15470_e23848_d_n4, assign15470_e23848_d_n5, assign15470_e23848_d_n6, assign15470_e23848_d_n7, assign15470_e23848_d_n8, assign15470_e23848_d_n9, assign15470_e23848_d_n10, assign15470_e23848_d_n11, assign15470_e23848_d_n13, assign15470_e23848_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard264 != 0.0)) {
        let assign15470_e23841: f64 = (locals.var_ua2cv_i * locals.var_tratio);
        let assign15470_e23842: f64 = (locals.var_ua1cv_i + assign15470_e23841);
        let assign15470_e23844: f64 = (assign15470_e23842 * locals.var_trat_ln);
        let assign15470_e23845: f64 = (assign15470_e23844).exp();
        let assign15470_e23846: f64 = (locals.var_uacv_i * assign15470_e23845);
        (assign15470_e23846, 0.0, 0.0, 0.0, (locals.var_uacv_i * (assign15470_e23845 * (((locals.var_ua2cv_i * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15470_e23842 * locals.var_trat_ln_dn4)))), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_uacv_t, locals.var_uacv_t_dn0, locals.var_uacv_t_dn2, locals.var_uacv_t_dn3, locals.var_uacv_t_dn4, locals.var_uacv_t_dn5, locals.var_uacv_t_dn6, locals.var_uacv_t_dn7, locals.var_uacv_t_dn8, locals.var_uacv_t_dn9, locals.var_uacv_t_dn10, locals.var_uacv_t_dn11, locals.var_uacv_t_dn13, locals.var_uacv_t_dn14,)
    }
};
        locals.var_uacv_t = assign15470_e23848;
        locals.var_uacv_t_dn0 = assign15470_e23848_d_n0;
        locals.var_uacv_t_dn2 = assign15470_e23848_d_n2;
        locals.var_uacv_t_dn3 = assign15470_e23848_d_n3;
        locals.var_uacv_t_dn4 = assign15470_e23848_d_n4;
        locals.var_uacv_t_dn5 = assign15470_e23848_d_n5;
        locals.var_uacv_t_dn6 = assign15470_e23848_d_n6;
        locals.var_uacv_t_dn7 = assign15470_e23848_d_n7;
        locals.var_uacv_t_dn8 = assign15470_e23848_d_n8;
        locals.var_uacv_t_dn9 = assign15470_e23848_d_n9;
        locals.var_uacv_t_dn10 = assign15470_e23848_d_n10;
        locals.var_uacv_t_dn11 = assign15470_e23848_d_n11;
        locals.var_uacv_t_dn13 = assign15470_e23848_d_n13;
        locals.var_uacv_t_dn14 = assign15470_e23848_d_n14;

        let (assign15480_e23866, assign15480_e23866_d_n4,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard264 != 0.0)) {
        let assign15480_e23859: f64 = (locals.var_ud2cv_i * locals.var_tratio);
        let assign15480_e23860: f64 = (locals.var_ud1cv_i + assign15480_e23859);
        let assign15480_e23862: f64 = (assign15480_e23860 * locals.var_trat_ln);
        let assign15480_e23863: f64 = (assign15480_e23862).exp();
        let assign15480_e23864: f64 = (locals.var_udcv_i * assign15480_e23863);
        (assign15480_e23864, (locals.var_udcv_i * (assign15480_e23863 * (((locals.var_ud2cv_i * locals.var_tratio_dn4) * locals.var_trat_ln) + (assign15480_e23860 * locals.var_trat_ln_dn4)))),)
    } else {
        (locals.var_udcv_t, locals.var_udcv_t_dn4,)
    }
};
        locals.var_udcv_t = assign15480_e23866;
        locals.var_udcv_t_dn4 = assign15480_e23866_d_n4;

        let assign15490_e23869: f64 = if locals.var_prt_i == locals.var_prt1_i { 1.0 } else { 0.0 };
        locals.var_guard265 = assign15490_e23869;

        let (assign15500_e23882, assign15500_e23882_d_n0, assign15500_e23882_d_n2, assign15500_e23882_d_n3, assign15500_e23882_d_n4, assign15500_e23882_d_n5, assign15500_e23882_d_n6, assign15500_e23882_d_n7, assign15500_e23882_d_n8, assign15500_e23882_d_n9, assign15500_e23882_d_n10, assign15500_e23882_d_n11, assign15500_e23882_d_n13, assign15500_e23882_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 != 0.0)) {
        let assign15500_e23879: f64 = (locals.var_prt_i * locals.var_deltemp);
        let assign15500_e23880: f64 = (1.0 + assign15500_e23879);
        (assign15500_e23880, 0.0, 0.0, 0.0, (locals.var_prt_i * locals.var_deltemp_dn4), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15500_e23882;
        locals.var_t2_dn0 = assign15500_e23882_d_n0;
        locals.var_t2_dn2 = assign15500_e23882_d_n2;
        locals.var_t2_dn3 = assign15500_e23882_d_n3;
        locals.var_t2_dn4 = assign15500_e23882_d_n4;
        locals.var_t2_dn5 = assign15500_e23882_d_n5;
        locals.var_t2_dn6 = assign15500_e23882_d_n6;
        locals.var_t2_dn7 = assign15500_e23882_d_n7;
        locals.var_t2_dn8 = assign15500_e23882_d_n8;
        locals.var_t2_dn9 = assign15500_e23882_d_n9;
        locals.var_t2_dn10 = assign15500_e23882_d_n10;
        locals.var_t2_dn11 = assign15500_e23882_d_n11;
        locals.var_t2_dn13 = assign15500_e23882_d_n13;
        locals.var_t2_dn14 = assign15500_e23882_d_n14;

        let assign15510_e23885: f64 = if locals.var_tr0_i < locals.var_tnom { 1.0 } else { 0.0 };
        locals.var_guard266 = assign15510_e23885;

        let (assign15520_e23901, assign15520_e23901_d_n4,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 != 0.0)) {
        let assign15520_e23898: f64 = (locals.var_prt_i * locals.var_deltemp);
        let assign15520_e23899: f64 = (1.0 + assign15520_e23898);
        (assign15520_e23899, (locals.var_prt_i * locals.var_deltemp_dn4),)
    } else {
        (locals.var_rdstemp0, locals.var_rdstemp0_dn4,)
    }
};
        locals.var_rdstemp0 = assign15520_e23901;
        locals.var_rdstemp0_dn4 = assign15520_e23901_d_n4;

        let (assign15530_e23925, assign15530_e23925_d_n4,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 != 0.0)) {
        let assign15530_e23915: f64 = (locals.var_devtemp - locals.var_tr0_i);
        let assign15530_e23916: f64 = (locals.var_prt1_i * assign15530_e23915);
        let assign15530_e23917: f64 = (1.0 + assign15530_e23916);
        let assign15530_e23921: f64 = (locals.var_tr0_i - locals.var_tnom);
        let assign15530_e23922: f64 = (locals.var_prt_i * assign15530_e23921);
        let assign15530_e23923: f64 = (assign15530_e23917 + assign15530_e23922);
        (assign15530_e23923, (locals.var_prt1_i * locals.var_devtemp_dn4),)
    } else {
        (locals.var_rdstemp1, locals.var_rdstemp1_dn4,)
    }
};
        locals.var_rdstemp1 = assign15530_e23925;
        locals.var_rdstemp1_dn4 = assign15530_e23925_d_n4;

        let (assign15540_e23943, assign15540_e23943_d_n0, assign15540_e23943_d_n2, assign15540_e23943_d_n3, assign15540_e23943_d_n4, assign15540_e23943_d_n5, assign15540_e23943_d_n6, assign15540_e23943_d_n7, assign15540_e23943_d_n8, assign15540_e23943_d_n9, assign15540_e23943_d_n10, assign15540_e23943_d_n11, assign15540_e23943_d_n13, assign15540_e23943_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 != 0.0)) {
        let assign15540_e23937: f64 = (locals.var_prt_i - locals.var_prt1_i);
        let assign15540_e23940: f64 = (locals.var_tr0_i - locals.var_tnom);
        let assign15540_e23941: f64 = (assign15540_e23937 * assign15540_e23940);
        (assign15540_e23941, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign15540_e23943;
        locals.var_t3_dn0 = assign15540_e23943_d_n0;
        locals.var_t3_dn2 = assign15540_e23943_d_n2;
        locals.var_t3_dn3 = assign15540_e23943_d_n3;
        locals.var_t3_dn4 = assign15540_e23943_d_n4;
        locals.var_t3_dn5 = assign15540_e23943_d_n5;
        locals.var_t3_dn6 = assign15540_e23943_d_n6;
        locals.var_t3_dn7 = assign15540_e23943_d_n7;
        locals.var_t3_dn8 = assign15540_e23943_d_n8;
        locals.var_t3_dn9 = assign15540_e23943_d_n9;
        locals.var_t3_dn10 = assign15540_e23943_d_n10;
        locals.var_t3_dn11 = assign15540_e23943_d_n11;
        locals.var_t3_dn13 = assign15540_e23943_d_n13;
        locals.var_t3_dn14 = assign15540_e23943_d_n14;

        let assign15550_e23946: f64 = if locals.var_prt1_i < locals.var_prt_i { 1.0 } else { 0.0 };
        locals.var_guard267 = assign15550_e23946;

        let (assign15560_e24000, assign15560_e24000_d_n0, assign15560_e24000_d_n2, assign15560_e24000_d_n3, assign15560_e24000_d_n4, assign15560_e24000_d_n5, assign15560_e24000_d_n6, assign15560_e24000_d_n7, assign15560_e24000_d_n8, assign15560_e24000_d_n9, assign15560_e24000_d_n10, assign15560_e24000_d_n11, assign15560_e24000_d_n13, assign15560_e24000_d_n14,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 != 0.0)) && (locals.var_guard267 != 0.0)) {
        let assign15560_e23961: f64 = (locals.var_rdstemp0 + locals.var_rdstemp1);
        let assign15560_e23964: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign15560_e23967: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign15560_e23968: f64 = (assign15560_e23964 * assign15560_e23967);
        let assign15560_e23971: f64 = (0.25 * locals.var_sprt_i);
        let assign15560_e23973: f64 = (assign15560_e23971 * locals.var_sprt_i);
        let assign15560_e23974: f64 = (assign15560_e23968 + assign15560_e23973);
        let assign15560_e23975: f64 = (assign15560_e23974).sqrt();
        let assign15560_e23976: f64 = (assign15560_e23961 + assign15560_e23975);
        let assign15560_e23977: f64 = (0.5 * assign15560_e23976);
        let assign15560_e23981: f64 = locals.var_t3;
        let assign15560_e23984: f64 = locals.var_t3;
        let assign15560_e23987: f64 = locals.var_t3;
        let assign15560_e23988: f64 = (assign15560_e23984 * assign15560_e23987);
        let assign15560_e23991: f64 = (0.25 * locals.var_sprt_i);
        let assign15560_e23993: f64 = (assign15560_e23991 * locals.var_sprt_i);
        let assign15560_e23994: f64 = (assign15560_e23988 + assign15560_e23993);
        let assign15560_e23995: f64 = (assign15560_e23994).sqrt();
        let assign15560_e23996: f64 = (assign15560_e23981 + assign15560_e23995);
        let assign15560_e23997: f64 = (0.5 * assign15560_e23996);
        let assign15560_e23998: f64 = (assign15560_e23977 - assign15560_e23997);
        (assign15560_e23998, (-(0.5 * (locals.var_t3_dn0 + (((locals.var_t3_dn0 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn0)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn2 + (((locals.var_t3_dn2 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn2)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn3 + (((locals.var_t3_dn3 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn3)) / (2.0 * assign15560_e23995))))), ((0.5 * ((locals.var_rdstemp0_dn4 + locals.var_rdstemp1_dn4) + ((((locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4) * assign15560_e23967) + (assign15560_e23964 * (locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4))) / (2.0 * assign15560_e23975)))) - (0.5 * (locals.var_t3_dn4 + (((locals.var_t3_dn4 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn4)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn5 + (((locals.var_t3_dn5 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn5)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn6 + (((locals.var_t3_dn6 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn6)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn7 + (((locals.var_t3_dn7 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn7)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn8 + (((locals.var_t3_dn8 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn8)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn9 + (((locals.var_t3_dn9 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn9)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn10 + (((locals.var_t3_dn10 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn10)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn11 + (((locals.var_t3_dn11 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn11)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn13 + (((locals.var_t3_dn13 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn13)) / (2.0 * assign15560_e23995))))), (-(0.5 * (locals.var_t3_dn14 + (((locals.var_t3_dn14 * assign15560_e23987) + (assign15560_e23984 * locals.var_t3_dn14)) / (2.0 * assign15560_e23995))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15560_e24000;
        locals.var_t2_dn0 = assign15560_e24000_d_n0;
        locals.var_t2_dn2 = assign15560_e24000_d_n2;
        locals.var_t2_dn3 = assign15560_e24000_d_n3;
        locals.var_t2_dn4 = assign15560_e24000_d_n4;
        locals.var_t2_dn5 = assign15560_e24000_d_n5;
        locals.var_t2_dn6 = assign15560_e24000_d_n6;
        locals.var_t2_dn7 = assign15560_e24000_d_n7;
        locals.var_t2_dn8 = assign15560_e24000_d_n8;
        locals.var_t2_dn9 = assign15560_e24000_d_n9;
        locals.var_t2_dn10 = assign15560_e24000_d_n10;
        locals.var_t2_dn11 = assign15560_e24000_d_n11;
        locals.var_t2_dn13 = assign15560_e24000_d_n13;
        locals.var_t2_dn14 = assign15560_e24000_d_n14;

        let (assign15570_e24055, assign15570_e24055_d_n0, assign15570_e24055_d_n2, assign15570_e24055_d_n3, assign15570_e24055_d_n4, assign15570_e24055_d_n5, assign15570_e24055_d_n6, assign15570_e24055_d_n7, assign15570_e24055_d_n8, assign15570_e24055_d_n9, assign15570_e24055_d_n10, assign15570_e24055_d_n11, assign15570_e24055_d_n13, assign15570_e24055_d_n14,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 != 0.0)) && (locals.var_guard267 == 0.0)) {
        let assign15570_e24016: f64 = (locals.var_rdstemp0 + locals.var_rdstemp1);
        let assign15570_e24019: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign15570_e24022: f64 = (locals.var_rdstemp0 - locals.var_rdstemp1);
        let assign15570_e24023: f64 = (assign15570_e24019 * assign15570_e24022);
        let assign15570_e24026: f64 = (0.25 * locals.var_sprt_i);
        let assign15570_e24028: f64 = (assign15570_e24026 * locals.var_sprt_i);
        let assign15570_e24029: f64 = (assign15570_e24023 + assign15570_e24028);
        let assign15570_e24030: f64 = (assign15570_e24029).sqrt();
        let assign15570_e24031: f64 = (assign15570_e24016 - assign15570_e24030);
        let assign15570_e24032: f64 = (0.5 * assign15570_e24031);
        let assign15570_e24036: f64 = locals.var_t3;
        let assign15570_e24039: f64 = locals.var_t3;
        let assign15570_e24042: f64 = locals.var_t3;
        let assign15570_e24043: f64 = (assign15570_e24039 * assign15570_e24042);
        let assign15570_e24046: f64 = (0.25 * locals.var_sprt_i);
        let assign15570_e24048: f64 = (assign15570_e24046 * locals.var_sprt_i);
        let assign15570_e24049: f64 = (assign15570_e24043 + assign15570_e24048);
        let assign15570_e24050: f64 = (assign15570_e24049).sqrt();
        let assign15570_e24051: f64 = (assign15570_e24036 - assign15570_e24050);
        let assign15570_e24052: f64 = (0.5 * assign15570_e24051);
        let assign15570_e24053: f64 = (assign15570_e24032 - assign15570_e24052);
        (assign15570_e24053, (-(0.5 * (locals.var_t3_dn0 - (((locals.var_t3_dn0 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn0)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn2 - (((locals.var_t3_dn2 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn2)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn3 - (((locals.var_t3_dn3 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn3)) / (2.0 * assign15570_e24050))))), ((0.5 * ((locals.var_rdstemp0_dn4 + locals.var_rdstemp1_dn4) - ((((locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4) * assign15570_e24022) + (assign15570_e24019 * (locals.var_rdstemp0_dn4 - locals.var_rdstemp1_dn4))) / (2.0 * assign15570_e24030)))) - (0.5 * (locals.var_t3_dn4 - (((locals.var_t3_dn4 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn4)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn5 - (((locals.var_t3_dn5 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn5)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn6 - (((locals.var_t3_dn6 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn6)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn7 - (((locals.var_t3_dn7 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn7)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn8 - (((locals.var_t3_dn8 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn8)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn9 - (((locals.var_t3_dn9 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn9)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn10 - (((locals.var_t3_dn10 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn10)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn11 - (((locals.var_t3_dn11 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn11)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn13 - (((locals.var_t3_dn13 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn13)) / (2.0 * assign15570_e24050))))), (-(0.5 * (locals.var_t3_dn14 - (((locals.var_t3_dn14 * assign15570_e24042) + (assign15570_e24039 * locals.var_t3_dn14)) / (2.0 * assign15570_e24050))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15570_e24055;
        locals.var_t2_dn0 = assign15570_e24055_d_n0;
        locals.var_t2_dn2 = assign15570_e24055_d_n2;
        locals.var_t2_dn3 = assign15570_e24055_d_n3;
        locals.var_t2_dn4 = assign15570_e24055_d_n4;
        locals.var_t2_dn5 = assign15570_e24055_d_n5;
        locals.var_t2_dn6 = assign15570_e24055_d_n6;
        locals.var_t2_dn7 = assign15570_e24055_d_n7;
        locals.var_t2_dn8 = assign15570_e24055_d_n8;
        locals.var_t2_dn9 = assign15570_e24055_d_n9;
        locals.var_t2_dn10 = assign15570_e24055_d_n10;
        locals.var_t2_dn11 = assign15570_e24055_d_n11;
        locals.var_t2_dn13 = assign15570_e24055_d_n13;
        locals.var_t2_dn14 = assign15570_e24055_d_n14;

        let (assign15580_e24074, assign15580_e24074_d_n4,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 == 0.0)) {
        let assign15580_e24070: f64 = (locals.var_devtemp - locals.var_tnom);
        let assign15580_e24071: f64 = (locals.var_prt1_i * assign15580_e24070);
        let assign15580_e24072: f64 = (1.0 + assign15580_e24071);
        (assign15580_e24072, (locals.var_prt1_i * locals.var_devtemp_dn4),)
    } else {
        (locals.var_rdstemp1, locals.var_rdstemp1_dn4,)
    }
};
        locals.var_rdstemp1 = assign15580_e24074;
        locals.var_rdstemp1_dn4 = assign15580_e24074_d_n4;

        let (assign15590_e24099, assign15590_e24099_d_n4,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 == 0.0)) {
        let assign15590_e24089: f64 = (locals.var_devtemp - locals.var_tr0_i);
        let assign15590_e24090: f64 = (locals.var_prt_i * assign15590_e24089);
        let assign15590_e24091: f64 = (1.0 + assign15590_e24090);
        let assign15590_e24095: f64 = (locals.var_tr0_i - locals.var_tnom);
        let assign15590_e24096: f64 = (locals.var_prt1_i * assign15590_e24095);
        let assign15590_e24097: f64 = (assign15590_e24091 + assign15590_e24096);
        (assign15590_e24097, (locals.var_prt_i * locals.var_devtemp_dn4),)
    } else {
        (locals.var_rdstemp0, locals.var_rdstemp0_dn4,)
    }
};
        locals.var_rdstemp0 = assign15590_e24099;
        locals.var_rdstemp0_dn4 = assign15590_e24099_d_n4;

        let (assign15600_e24118, assign15600_e24118_d_n0, assign15600_e24118_d_n2, assign15600_e24118_d_n3, assign15600_e24118_d_n4, assign15600_e24118_d_n5, assign15600_e24118_d_n6, assign15600_e24118_d_n7, assign15600_e24118_d_n8, assign15600_e24118_d_n9, assign15600_e24118_d_n10, assign15600_e24118_d_n11, assign15600_e24118_d_n13, assign15600_e24118_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 == 0.0)) {
        let assign15600_e24112: f64 = (locals.var_prt1_i - locals.var_prt_i);
        let assign15600_e24115: f64 = (locals.var_tr0_i - locals.var_tnom);
        let assign15600_e24116: f64 = (assign15600_e24112 * assign15600_e24115);
        (assign15600_e24116, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_t3, locals.var_t3_dn0, locals.var_t3_dn2, locals.var_t3_dn3, locals.var_t3_dn4, locals.var_t3_dn5, locals.var_t3_dn6, locals.var_t3_dn7, locals.var_t3_dn8, locals.var_t3_dn9, locals.var_t3_dn10, locals.var_t3_dn11, locals.var_t3_dn13, locals.var_t3_dn14,)
    }
};
        locals.var_t3 = assign15600_e24118;
        locals.var_t3_dn0 = assign15600_e24118_d_n0;
        locals.var_t3_dn2 = assign15600_e24118_d_n2;
        locals.var_t3_dn3 = assign15600_e24118_d_n3;
        locals.var_t3_dn4 = assign15600_e24118_d_n4;
        locals.var_t3_dn5 = assign15600_e24118_d_n5;
        locals.var_t3_dn6 = assign15600_e24118_d_n6;
        locals.var_t3_dn7 = assign15600_e24118_d_n7;
        locals.var_t3_dn8 = assign15600_e24118_d_n8;
        locals.var_t3_dn9 = assign15600_e24118_d_n9;
        locals.var_t3_dn10 = assign15600_e24118_d_n10;
        locals.var_t3_dn11 = assign15600_e24118_d_n11;
        locals.var_t3_dn13 = assign15600_e24118_d_n13;
        locals.var_t3_dn14 = assign15600_e24118_d_n14;

        let assign15610_e24121: f64 = if locals.var_prt1_i < locals.var_prt_i { 1.0 } else { 0.0 };
        locals.var_guard268 = assign15610_e24121;

        let (assign15620_e24176, assign15620_e24176_d_n0, assign15620_e24176_d_n2, assign15620_e24176_d_n3, assign15620_e24176_d_n4, assign15620_e24176_d_n5, assign15620_e24176_d_n6, assign15620_e24176_d_n7, assign15620_e24176_d_n8, assign15620_e24176_d_n9, assign15620_e24176_d_n10, assign15620_e24176_d_n11, assign15620_e24176_d_n13, assign15620_e24176_d_n14,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 == 0.0)) && (locals.var_guard268 != 0.0)) {
        let assign15620_e24137: f64 = (locals.var_rdstemp1 + locals.var_rdstemp0);
        let assign15620_e24140: f64 = (locals.var_rdstemp1 - locals.var_rdstemp0);
        let assign15620_e24143: f64 = (locals.var_rdstemp1 - locals.var_rdstemp0);
        let assign15620_e24144: f64 = (assign15620_e24140 * assign15620_e24143);
        let assign15620_e24147: f64 = (0.25 * locals.var_sprt_i);
        let assign15620_e24149: f64 = (assign15620_e24147 * locals.var_sprt_i);
        let assign15620_e24150: f64 = (assign15620_e24144 + assign15620_e24149);
        let assign15620_e24151: f64 = (assign15620_e24150).sqrt();
        let assign15620_e24152: f64 = (assign15620_e24137 + assign15620_e24151);
        let assign15620_e24153: f64 = (0.5 * assign15620_e24152);
        let assign15620_e24157: f64 = locals.var_t3;
        let assign15620_e24160: f64 = locals.var_t3;
        let assign15620_e24163: f64 = locals.var_t3;
        let assign15620_e24164: f64 = (assign15620_e24160 * assign15620_e24163);
        let assign15620_e24167: f64 = (0.25 * locals.var_sprt_i);
        let assign15620_e24169: f64 = (assign15620_e24167 * locals.var_sprt_i);
        let assign15620_e24170: f64 = (assign15620_e24164 + assign15620_e24169);
        let assign15620_e24171: f64 = (assign15620_e24170).sqrt();
        let assign15620_e24172: f64 = (assign15620_e24157 + assign15620_e24171);
        let assign15620_e24173: f64 = (0.5 * assign15620_e24172);
        let assign15620_e24174: f64 = (assign15620_e24153 - assign15620_e24173);
        (assign15620_e24174, (-(0.5 * (locals.var_t3_dn0 + (((locals.var_t3_dn0 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn0)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn2 + (((locals.var_t3_dn2 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn2)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn3 + (((locals.var_t3_dn3 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn3)) / (2.0 * assign15620_e24171))))), ((0.5 * ((locals.var_rdstemp1_dn4 + locals.var_rdstemp0_dn4) + ((((locals.var_rdstemp1_dn4 - locals.var_rdstemp0_dn4) * assign15620_e24143) + (assign15620_e24140 * (locals.var_rdstemp1_dn4 - locals.var_rdstemp0_dn4))) / (2.0 * assign15620_e24151)))) - (0.5 * (locals.var_t3_dn4 + (((locals.var_t3_dn4 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn4)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn5 + (((locals.var_t3_dn5 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn5)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn6 + (((locals.var_t3_dn6 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn6)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn7 + (((locals.var_t3_dn7 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn7)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn8 + (((locals.var_t3_dn8 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn8)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn9 + (((locals.var_t3_dn9 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn9)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn10 + (((locals.var_t3_dn10 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn10)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn11 + (((locals.var_t3_dn11 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn11)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn13 + (((locals.var_t3_dn13 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn13)) / (2.0 * assign15620_e24171))))), (-(0.5 * (locals.var_t3_dn14 + (((locals.var_t3_dn14 * assign15620_e24163) + (assign15620_e24160 * locals.var_t3_dn14)) / (2.0 * assign15620_e24171))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15620_e24176;
        locals.var_t2_dn0 = assign15620_e24176_d_n0;
        locals.var_t2_dn2 = assign15620_e24176_d_n2;
        locals.var_t2_dn3 = assign15620_e24176_d_n3;
        locals.var_t2_dn4 = assign15620_e24176_d_n4;
        locals.var_t2_dn5 = assign15620_e24176_d_n5;
        locals.var_t2_dn6 = assign15620_e24176_d_n6;
        locals.var_t2_dn7 = assign15620_e24176_d_n7;
        locals.var_t2_dn8 = assign15620_e24176_d_n8;
        locals.var_t2_dn9 = assign15620_e24176_d_n9;
        locals.var_t2_dn10 = assign15620_e24176_d_n10;
        locals.var_t2_dn11 = assign15620_e24176_d_n11;
        locals.var_t2_dn13 = assign15620_e24176_d_n13;
        locals.var_t2_dn14 = assign15620_e24176_d_n14;

    }

    pub(super) fn stamp_transient_block_46(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15630_e24232, assign15630_e24232_d_n0, assign15630_e24232_d_n2, assign15630_e24232_d_n3, assign15630_e24232_d_n4, assign15630_e24232_d_n5, assign15630_e24232_d_n6, assign15630_e24232_d_n7, assign15630_e24232_d_n8, assign15630_e24232_d_n9, assign15630_e24232_d_n10, assign15630_e24232_d_n11, assign15630_e24232_d_n13, assign15630_e24232_d_n14,) = {
    if (((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard265 == 0.0)) && (locals.var_guard266 == 0.0)) && (locals.var_guard268 == 0.0)) {
        let assign15630_e24193: f64 = (locals.var_rdstemp1 + locals.var_rdstemp0);
        let assign15630_e24196: f64 = (locals.var_rdstemp1 - locals.var_rdstemp0);
        let assign15630_e24199: f64 = (locals.var_rdstemp1 - locals.var_rdstemp0);
        let assign15630_e24200: f64 = (assign15630_e24196 * assign15630_e24199);
        let assign15630_e24203: f64 = (0.25 * locals.var_sprt_i);
        let assign15630_e24205: f64 = (assign15630_e24203 * locals.var_sprt_i);
        let assign15630_e24206: f64 = (assign15630_e24200 + assign15630_e24205);
        let assign15630_e24207: f64 = (assign15630_e24206).sqrt();
        let assign15630_e24208: f64 = (assign15630_e24193 - assign15630_e24207);
        let assign15630_e24209: f64 = (0.5 * assign15630_e24208);
        let assign15630_e24213: f64 = locals.var_t3;
        let assign15630_e24216: f64 = locals.var_t3;
        let assign15630_e24219: f64 = locals.var_t3;
        let assign15630_e24220: f64 = (assign15630_e24216 * assign15630_e24219);
        let assign15630_e24223: f64 = (0.25 * locals.var_sprt_i);
        let assign15630_e24225: f64 = (assign15630_e24223 * locals.var_sprt_i);
        let assign15630_e24226: f64 = (assign15630_e24220 + assign15630_e24225);
        let assign15630_e24227: f64 = (assign15630_e24226).sqrt();
        let assign15630_e24228: f64 = (assign15630_e24213 - assign15630_e24227);
        let assign15630_e24229: f64 = (0.5 * assign15630_e24228);
        let assign15630_e24230: f64 = (assign15630_e24209 - assign15630_e24229);
        (assign15630_e24230, (-(0.5 * (locals.var_t3_dn0 - (((locals.var_t3_dn0 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn0)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn2 - (((locals.var_t3_dn2 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn2)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn3 - (((locals.var_t3_dn3 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn3)) / (2.0 * assign15630_e24227))))), ((0.5 * ((locals.var_rdstemp1_dn4 + locals.var_rdstemp0_dn4) - ((((locals.var_rdstemp1_dn4 - locals.var_rdstemp0_dn4) * assign15630_e24199) + (assign15630_e24196 * (locals.var_rdstemp1_dn4 - locals.var_rdstemp0_dn4))) / (2.0 * assign15630_e24207)))) - (0.5 * (locals.var_t3_dn4 - (((locals.var_t3_dn4 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn4)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn5 - (((locals.var_t3_dn5 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn5)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn6 - (((locals.var_t3_dn6 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn6)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn7 - (((locals.var_t3_dn7 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn7)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn8 - (((locals.var_t3_dn8 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn8)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn9 - (((locals.var_t3_dn9 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn9)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn10 - (((locals.var_t3_dn10 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn10)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn11 - (((locals.var_t3_dn11 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn11)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn13 - (((locals.var_t3_dn13 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn13)) / (2.0 * assign15630_e24227))))), (-(0.5 * (locals.var_t3_dn14 - (((locals.var_t3_dn14 * assign15630_e24219) + (assign15630_e24216 * locals.var_t3_dn14)) / (2.0 * assign15630_e24227))))),)
    } else {
        (locals.var_t2, locals.var_t2_dn0, locals.var_t2_dn2, locals.var_t2_dn3, locals.var_t2_dn4, locals.var_t2_dn5, locals.var_t2_dn6, locals.var_t2_dn7, locals.var_t2_dn8, locals.var_t2_dn9, locals.var_t2_dn10, locals.var_t2_dn11, locals.var_t2_dn13, locals.var_t2_dn14,)
    }
};
        locals.var_t2 = assign15630_e24232;
        locals.var_t2_dn0 = assign15630_e24232_d_n0;
        locals.var_t2_dn2 = assign15630_e24232_d_n2;
        locals.var_t2_dn3 = assign15630_e24232_d_n3;
        locals.var_t2_dn4 = assign15630_e24232_d_n4;
        locals.var_t2_dn5 = assign15630_e24232_d_n5;
        locals.var_t2_dn6 = assign15630_e24232_d_n6;
        locals.var_t2_dn7 = assign15630_e24232_d_n7;
        locals.var_t2_dn8 = assign15630_e24232_d_n8;
        locals.var_t2_dn9 = assign15630_e24232_d_n9;
        locals.var_t2_dn10 = assign15630_e24232_d_n10;
        locals.var_t2_dn11 = assign15630_e24232_d_n11;
        locals.var_t2_dn13 = assign15630_e24232_d_n13;
        locals.var_t2_dn14 = assign15630_e24232_d_n14;

        let (assign15640_e24286, assign15640_e24286_d_n0, assign15640_e24286_d_n2, assign15640_e24286_d_n3, assign15640_e24286_d_n4, assign15640_e24286_d_n5, assign15640_e24286_d_n6, assign15640_e24286_d_n7, assign15640_e24286_d_n8, assign15640_e24286_d_n9, assign15640_e24286_d_n10, assign15640_e24286_d_n11, assign15640_e24286_d_n13, assign15640_e24286_d_n14,) = {
    if ((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) {
        let assign15640_e24239: f64 = (locals.var_t2 - 1e-6);
        let assign15640_e24241: f64 = (-10000.0);
        let assign15640_e24243: f64 = (assign15640_e24241 * 0.001);
        let (assign15640_e24284, assign15640_e24284_d_n0, assign15640_e24284_d_n2, assign15640_e24284_d_n3, assign15640_e24284_d_n4, assign15640_e24284_d_n5, assign15640_e24284_d_n6, assign15640_e24284_d_n7, assign15640_e24284_d_n8, assign15640_e24284_d_n9, assign15640_e24284_d_n10, assign15640_e24284_d_n11, assign15640_e24284_d_n13, assign15640_e24284_d_n14,) = {
            if (!(assign15640_e24239 < assign15640_e24243)) {
                let assign15640_e24249: f64 = (locals.var_t2 - 1e-6);
                let assign15640_e24252: f64 = (locals.var_t2 - 1e-6);
                let assign15640_e24255: f64 = (locals.var_t2 - 1e-6);
                let assign15640_e24256: f64 = (assign15640_e24252 * assign15640_e24255);
                let assign15640_e24259: f64 = (4.0 * 0.001);
                let assign15640_e24261: f64 = (assign15640_e24259 * 0.001);
                let assign15640_e24262: f64 = (assign15640_e24256 + assign15640_e24261);
                let assign15640_e24263: f64 = (assign15640_e24262).sqrt();
                let assign15640_e24264: f64 = (assign15640_e24249 + assign15640_e24263);
                let assign15640_e24265: f64 = (0.5 * assign15640_e24264);
                (assign15640_e24265, (0.5 * (locals.var_t2_dn0 + (((locals.var_t2_dn0 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn0)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn2 + (((locals.var_t2_dn2 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn2)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn3 + (((locals.var_t2_dn3 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn3)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn4 + (((locals.var_t2_dn4 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn4)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn5 + (((locals.var_t2_dn5 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn5)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn6 + (((locals.var_t2_dn6 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn6)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn7 + (((locals.var_t2_dn7 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn7)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn8 + (((locals.var_t2_dn8 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn8)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn9 + (((locals.var_t2_dn9 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn9)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn10 + (((locals.var_t2_dn10 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn10)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn11 + (((locals.var_t2_dn11 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn11)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn13 + (((locals.var_t2_dn13 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn13)) / (2.0 * assign15640_e24263)))), (0.5 * (locals.var_t2_dn14 + (((locals.var_t2_dn14 * assign15640_e24255) + (assign15640_e24252 * locals.var_t2_dn14)) / (2.0 * assign15640_e24263)))),)
            } else {
                let assign15640_e24268: f64 = (locals.var_t2 - 1e-6);
                let assign15640_e24270: f64 = (-10000.0);
                let assign15640_e24272: f64 = (assign15640_e24270 * 0.001);
                let (assign15640_e24283, assign15640_e24283_d_n0, assign15640_e24283_d_n2, assign15640_e24283_d_n3, assign15640_e24283_d_n4, assign15640_e24283_d_n5, assign15640_e24283_d_n6, assign15640_e24283_d_n7, assign15640_e24283_d_n8, assign15640_e24283_d_n9, assign15640_e24283_d_n10, assign15640_e24283_d_n11, assign15640_e24283_d_n13, assign15640_e24283_d_n14,) = {
                    if (assign15640_e24268 < assign15640_e24272) {
                        let assign15640_e24275: f64 = (-0.001);
                        let assign15640_e24277: f64 = (assign15640_e24275 * 0.001);
                        let assign15640_e24280: f64 = (locals.var_t2 - 1e-6);
                        let assign15640_e24281: f64 = (assign15640_e24277 / assign15640_e24280);
                        (assign15640_e24281, (-((assign15640_e24277 * locals.var_t2_dn0) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn2) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn3) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn4) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn5) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn6) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn7) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn8) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn9) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn10) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn11) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn13) / (assign15640_e24280 * assign15640_e24280))), (-((assign15640_e24277 * locals.var_t2_dn14) / (assign15640_e24280 * assign15640_e24280))),)
                    } else {
                        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
                    }
                };
                (assign15640_e24283, assign15640_e24283_d_n0, assign15640_e24283_d_n2, assign15640_e24283_d_n3, assign15640_e24283_d_n4, assign15640_e24283_d_n5, assign15640_e24283_d_n6, assign15640_e24283_d_n7, assign15640_e24283_d_n8, assign15640_e24283_d_n9, assign15640_e24283_d_n10, assign15640_e24283_d_n11, assign15640_e24283_d_n13, assign15640_e24283_d_n14,)
            }
        };
        (assign15640_e24284, assign15640_e24284_d_n0, assign15640_e24284_d_n2, assign15640_e24284_d_n3, assign15640_e24284_d_n4, assign15640_e24284_d_n5, assign15640_e24284_d_n6, assign15640_e24284_d_n7, assign15640_e24284_d_n8, assign15640_e24284_d_n9, assign15640_e24284_d_n10, assign15640_e24284_d_n11, assign15640_e24284_d_n13, assign15640_e24284_d_n14,)
    } else {
        (locals.var_rdstemp, locals.var_rdstemp_dn0, locals.var_rdstemp_dn2, locals.var_rdstemp_dn3, locals.var_rdstemp_dn4, locals.var_rdstemp_dn5, locals.var_rdstemp_dn6, locals.var_rdstemp_dn7, locals.var_rdstemp_dn8, locals.var_rdstemp_dn9, locals.var_rdstemp_dn10, locals.var_rdstemp_dn11, locals.var_rdstemp_dn13, locals.var_rdstemp_dn14,)
    }
};
        locals.var_rdstemp = assign15640_e24286;
        locals.var_rdstemp_dn0 = assign15640_e24286_d_n0;
        locals.var_rdstemp_dn2 = assign15640_e24286_d_n2;
        locals.var_rdstemp_dn3 = assign15640_e24286_d_n3;
        locals.var_rdstemp_dn4 = assign15640_e24286_d_n4;
        locals.var_rdstemp_dn5 = assign15640_e24286_d_n5;
        locals.var_rdstemp_dn6 = assign15640_e24286_d_n6;
        locals.var_rdstemp_dn7 = assign15640_e24286_d_n7;
        locals.var_rdstemp_dn8 = assign15640_e24286_d_n8;
        locals.var_rdstemp_dn9 = assign15640_e24286_d_n9;
        locals.var_rdstemp_dn10 = assign15640_e24286_d_n10;
        locals.var_rdstemp_dn11 = assign15640_e24286_d_n11;
        locals.var_rdstemp_dn13 = assign15640_e24286_d_n13;
        locals.var_rdstemp_dn14 = assign15640_e24286_d_n14;

        let assign15650_e24289: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard269 = assign15650_e24289;

        let (assign15660_e24359, assign15660_e24359_d_n0, assign15660_e24359_d_n2, assign15660_e24359_d_n3, assign15660_e24359_d_n4, assign15660_e24359_d_n5, assign15660_e24359_d_n6, assign15660_e24359_d_n7, assign15660_e24359_d_n8, assign15660_e24359_d_n9, assign15660_e24359_d_n10, assign15660_e24359_d_n11, assign15660_e24359_d_n13, assign15660_e24359_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard269 != 0.0)) {
        let assign15660_e24298: f64 = (-locals.var_vsat_i);
        let assign15660_e24301: f64 = (-locals.var_at_i);
        let assign15660_e24303: f64 = (assign15660_e24301 * locals.var_deltemp);
        let assign15660_e24306: f64 = (p.p561 * locals.var_deltemp);
        let assign15660_e24308: f64 = (assign15660_e24306 * locals.var_deltemp);
        let assign15660_e24309: f64 = (assign15660_e24303 + assign15660_e24308);
        let assign15660_e24311: f64 = (-locals.var_vsat_i);
        let assign15660_e24312: f64 = (assign15660_e24309 - assign15660_e24311);
        let assign15660_e24314: f64 = (assign15660_e24312 - 1e-6);
        let assign15660_e24316: f64 = (-locals.var_at_i);
        let assign15660_e24318: f64 = (assign15660_e24316 * locals.var_deltemp);
        let assign15660_e24321: f64 = (p.p561 * locals.var_deltemp);
        let assign15660_e24323: f64 = (assign15660_e24321 * locals.var_deltemp);
        let assign15660_e24324: f64 = (assign15660_e24318 + assign15660_e24323);
        let assign15660_e24326: f64 = (-locals.var_vsat_i);
        let assign15660_e24327: f64 = (assign15660_e24324 - assign15660_e24326);
        let assign15660_e24329: f64 = (assign15660_e24327 - 1e-6);
        let assign15660_e24331: f64 = (-locals.var_at_i);
        let assign15660_e24333: f64 = (assign15660_e24331 * locals.var_deltemp);
        let assign15660_e24336: f64 = (p.p561 * locals.var_deltemp);
        let assign15660_e24338: f64 = (assign15660_e24336 * locals.var_deltemp);
        let assign15660_e24339: f64 = (assign15660_e24333 + assign15660_e24338);
        let assign15660_e24341: f64 = (-locals.var_vsat_i);
        let assign15660_e24342: f64 = (assign15660_e24339 - assign15660_e24341);
        let assign15660_e24344: f64 = (assign15660_e24342 - 1e-6);
        let assign15660_e24345: f64 = (assign15660_e24329 * assign15660_e24344);
        let assign15660_e24348: f64 = (-locals.var_vsat_i);
        let assign15660_e24349: f64 = (4.0 * assign15660_e24348);
        let assign15660_e24351: f64 = (assign15660_e24349 * 1e-6);
        let assign15660_e24352: f64 = (assign15660_e24345 - assign15660_e24351);
        let assign15660_e24353: f64 = (assign15660_e24352).sqrt();
        let assign15660_e24354: f64 = (assign15660_e24314 + assign15660_e24353);
        let assign15660_e24355: f64 = (0.5 * assign15660_e24354);
        let assign15660_e24356: f64 = (assign15660_e24298 + assign15660_e24355);
        let assign15660_e24357: f64 = (locals.var_vsat_i + assign15660_e24356);
        (assign15660_e24357, (locals.var_vsat_i_dn0 + ((-locals.var_vsat_i_dn0) + (0.5 * ((-(-locals.var_vsat_i_dn0)) + (((((-(-locals.var_vsat_i_dn0)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn0)))) - ((4.0 * (-locals.var_vsat_i_dn0)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn2 + ((-locals.var_vsat_i_dn2) + (0.5 * ((-(-locals.var_vsat_i_dn2)) + (((((-(-locals.var_vsat_i_dn2)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn2)))) - ((4.0 * (-locals.var_vsat_i_dn2)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn3 + ((-locals.var_vsat_i_dn3) + (0.5 * ((-(-locals.var_vsat_i_dn3)) + (((((-(-locals.var_vsat_i_dn3)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn3)))) - ((4.0 * (-locals.var_vsat_i_dn3)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn4 + ((-locals.var_vsat_i_dn4) + (0.5 * ((((assign15660_e24301 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15660_e24306 * locals.var_deltemp_dn4))) - (-locals.var_vsat_i_dn4)) + (((((((assign15660_e24316 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15660_e24321 * locals.var_deltemp_dn4))) - (-locals.var_vsat_i_dn4)) * assign15660_e24344) + (assign15660_e24329 * (((assign15660_e24331 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15660_e24336 * locals.var_deltemp_dn4))) - (-locals.var_vsat_i_dn4)))) - ((4.0 * (-locals.var_vsat_i_dn4)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn5 + ((-locals.var_vsat_i_dn5) + (0.5 * ((-(-locals.var_vsat_i_dn5)) + (((((-(-locals.var_vsat_i_dn5)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn5)))) - ((4.0 * (-locals.var_vsat_i_dn5)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn6 + ((-locals.var_vsat_i_dn6) + (0.5 * ((-(-locals.var_vsat_i_dn6)) + (((((-(-locals.var_vsat_i_dn6)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn6)))) - ((4.0 * (-locals.var_vsat_i_dn6)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn7 + ((-locals.var_vsat_i_dn7) + (0.5 * ((-(-locals.var_vsat_i_dn7)) + (((((-(-locals.var_vsat_i_dn7)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn7)))) - ((4.0 * (-locals.var_vsat_i_dn7)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn8 + ((-locals.var_vsat_i_dn8) + (0.5 * ((-(-locals.var_vsat_i_dn8)) + (((((-(-locals.var_vsat_i_dn8)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn8)))) - ((4.0 * (-locals.var_vsat_i_dn8)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn9 + ((-locals.var_vsat_i_dn9) + (0.5 * ((-(-locals.var_vsat_i_dn9)) + (((((-(-locals.var_vsat_i_dn9)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn9)))) - ((4.0 * (-locals.var_vsat_i_dn9)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn10 + ((-locals.var_vsat_i_dn10) + (0.5 * ((-(-locals.var_vsat_i_dn10)) + (((((-(-locals.var_vsat_i_dn10)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn10)))) - ((4.0 * (-locals.var_vsat_i_dn10)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn11 + ((-locals.var_vsat_i_dn11) + (0.5 * ((-(-locals.var_vsat_i_dn11)) + (((((-(-locals.var_vsat_i_dn11)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn11)))) - ((4.0 * (-locals.var_vsat_i_dn11)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn13 + ((-locals.var_vsat_i_dn13) + (0.5 * ((-(-locals.var_vsat_i_dn13)) + (((((-(-locals.var_vsat_i_dn13)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn13)))) - ((4.0 * (-locals.var_vsat_i_dn13)) * 1e-6)) / (2.0 * assign15660_e24353)))))), (locals.var_vsat_i_dn14 + ((-locals.var_vsat_i_dn14) + (0.5 * ((-(-locals.var_vsat_i_dn14)) + (((((-(-locals.var_vsat_i_dn14)) * assign15660_e24344) + (assign15660_e24329 * (-(-locals.var_vsat_i_dn14)))) - ((4.0 * (-locals.var_vsat_i_dn14)) * 1e-6)) / (2.0 * assign15660_e24353)))))),)
    } else {
        (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14,)
    }
};
        locals.var_vsat_t = assign15660_e24359;
        locals.var_vsat_t_dn0 = assign15660_e24359_d_n0;
        locals.var_vsat_t_dn2 = assign15660_e24359_d_n2;
        locals.var_vsat_t_dn3 = assign15660_e24359_d_n3;
        locals.var_vsat_t_dn4 = assign15660_e24359_d_n4;
        locals.var_vsat_t_dn5 = assign15660_e24359_d_n5;
        locals.var_vsat_t_dn6 = assign15660_e24359_d_n6;
        locals.var_vsat_t_dn7 = assign15660_e24359_d_n7;
        locals.var_vsat_t_dn8 = assign15660_e24359_d_n8;
        locals.var_vsat_t_dn9 = assign15660_e24359_d_n9;
        locals.var_vsat_t_dn10 = assign15660_e24359_d_n10;
        locals.var_vsat_t_dn11 = assign15660_e24359_d_n11;
        locals.var_vsat_t_dn13 = assign15660_e24359_d_n13;
        locals.var_vsat_t_dn14 = assign15660_e24359_d_n14;

        let (assign15670_e24484, assign15670_e24484_d_n0, assign15670_e24484_d_n2, assign15670_e24484_d_n3, assign15670_e24484_d_n4, assign15670_e24484_d_n5, assign15670_e24484_d_n6, assign15670_e24484_d_n7, assign15670_e24484_d_n8, assign15670_e24484_d_n9, assign15670_e24484_d_n10, assign15670_e24484_d_n11, assign15670_e24484_d_n13, assign15670_e24484_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard269 == 0.0)) {
        let assign15670_e24370: f64 = (-locals.var_at_i);
        let assign15670_e24372: f64 = (assign15670_e24370 * locals.var_deltemp);
        let assign15670_e24373: f64 = (1.0 + assign15670_e24372);
        let assign15670_e24376: f64 = (p.p561 * locals.var_deltemp);
        let assign15670_e24378: f64 = (assign15670_e24376 * locals.var_deltemp);
        let assign15670_e24379: f64 = (assign15670_e24373 + assign15670_e24378);
        let assign15670_e24381: f64 = (assign15670_e24379 - 1e-6);
        let assign15670_e24383: f64 = (-10000.0);
        let assign15670_e24385: f64 = (assign15670_e24383 * 0.001);
        let (assign15670_e24481, assign15670_e24481_d_n4,) = {
            if (!(assign15670_e24381 < assign15670_e24385)) {
                let assign15670_e24391: f64 = (-locals.var_at_i);
                let assign15670_e24393: f64 = (assign15670_e24391 * locals.var_deltemp);
                let assign15670_e24394: f64 = (1.0 + assign15670_e24393);
                let assign15670_e24397: f64 = (p.p561 * locals.var_deltemp);
                let assign15670_e24399: f64 = (assign15670_e24397 * locals.var_deltemp);
                let assign15670_e24400: f64 = (assign15670_e24394 + assign15670_e24399);
                let assign15670_e24402: f64 = (assign15670_e24400 - 1e-6);
                let assign15670_e24405: f64 = (-locals.var_at_i);
                let assign15670_e24407: f64 = (assign15670_e24405 * locals.var_deltemp);
                let assign15670_e24408: f64 = (1.0 + assign15670_e24407);
                let assign15670_e24411: f64 = (p.p561 * locals.var_deltemp);
                let assign15670_e24413: f64 = (assign15670_e24411 * locals.var_deltemp);
                let assign15670_e24414: f64 = (assign15670_e24408 + assign15670_e24413);
                let assign15670_e24416: f64 = (assign15670_e24414 - 1e-6);
                let assign15670_e24419: f64 = (-locals.var_at_i);
                let assign15670_e24421: f64 = (assign15670_e24419 * locals.var_deltemp);
                let assign15670_e24422: f64 = (1.0 + assign15670_e24421);
                let assign15670_e24425: f64 = (p.p561 * locals.var_deltemp);
                let assign15670_e24427: f64 = (assign15670_e24425 * locals.var_deltemp);
                let assign15670_e24428: f64 = (assign15670_e24422 + assign15670_e24427);
                let assign15670_e24430: f64 = (assign15670_e24428 - 1e-6);
                let assign15670_e24431: f64 = (assign15670_e24416 * assign15670_e24430);
                let assign15670_e24434: f64 = (4.0 * 0.001);
                let assign15670_e24436: f64 = (assign15670_e24434 * 0.001);
                let assign15670_e24437: f64 = (assign15670_e24431 + assign15670_e24436);
                let assign15670_e24438: f64 = (assign15670_e24437).sqrt();
                let assign15670_e24439: f64 = (assign15670_e24402 + assign15670_e24438);
                let assign15670_e24440: f64 = (0.5 * assign15670_e24439);
                (assign15670_e24440, (0.5 * (((assign15670_e24391 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15670_e24397 * locals.var_deltemp_dn4))) + (((((assign15670_e24405 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15670_e24411 * locals.var_deltemp_dn4))) * assign15670_e24430) + (assign15670_e24416 * ((assign15670_e24419 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15670_e24425 * locals.var_deltemp_dn4))))) / (2.0 * assign15670_e24438)))),)
            } else {
                let assign15670_e24443: f64 = (-locals.var_at_i);
                let assign15670_e24445: f64 = (assign15670_e24443 * locals.var_deltemp);
                let assign15670_e24446: f64 = (1.0 + assign15670_e24445);
                let assign15670_e24449: f64 = (p.p561 * locals.var_deltemp);
                let assign15670_e24451: f64 = (assign15670_e24449 * locals.var_deltemp);
                let assign15670_e24452: f64 = (assign15670_e24446 + assign15670_e24451);
                let assign15670_e24454: f64 = (assign15670_e24452 - 1e-6);
                let assign15670_e24456: f64 = (-10000.0);
                let assign15670_e24458: f64 = (assign15670_e24456 * 0.001);
                let (assign15670_e24480, assign15670_e24480_d_n4,) = {
                    if (assign15670_e24454 < assign15670_e24458) {
                        let assign15670_e24461: f64 = (-0.001);
                        let assign15670_e24463: f64 = (assign15670_e24461 * 0.001);
                        let assign15670_e24466: f64 = (-locals.var_at_i);
                        let assign15670_e24468: f64 = (assign15670_e24466 * locals.var_deltemp);
                        let assign15670_e24469: f64 = (1.0 + assign15670_e24468);
                        let assign15670_e24472: f64 = (p.p561 * locals.var_deltemp);
                        let assign15670_e24474: f64 = (assign15670_e24472 * locals.var_deltemp);
                        let assign15670_e24475: f64 = (assign15670_e24469 + assign15670_e24474);
                        let assign15670_e24477: f64 = (assign15670_e24475 - 1e-6);
                        let assign15670_e24478: f64 = (assign15670_e24463 / assign15670_e24477);
                        (assign15670_e24478, (-((assign15670_e24463 * ((assign15670_e24466 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15670_e24472 * locals.var_deltemp_dn4)))) / (assign15670_e24477 * assign15670_e24477))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15670_e24480, assign15670_e24480_d_n4,)
            }
        };
        let assign15670_e24482: f64 = (locals.var_vsat_i * assign15670_e24481);
        (assign15670_e24482, (locals.var_vsat_i_dn0 * assign15670_e24481), (locals.var_vsat_i_dn2 * assign15670_e24481), (locals.var_vsat_i_dn3 * assign15670_e24481), ((locals.var_vsat_i_dn4 * assign15670_e24481) + (locals.var_vsat_i * assign15670_e24481_d_n4)), (locals.var_vsat_i_dn5 * assign15670_e24481), (locals.var_vsat_i_dn6 * assign15670_e24481), (locals.var_vsat_i_dn7 * assign15670_e24481), (locals.var_vsat_i_dn8 * assign15670_e24481), (locals.var_vsat_i_dn9 * assign15670_e24481), (locals.var_vsat_i_dn10 * assign15670_e24481), (locals.var_vsat_i_dn11 * assign15670_e24481), (locals.var_vsat_i_dn13 * assign15670_e24481), (locals.var_vsat_i_dn14 * assign15670_e24481),)
    } else {
        (locals.var_vsat_t, locals.var_vsat_t_dn0, locals.var_vsat_t_dn2, locals.var_vsat_t_dn3, locals.var_vsat_t_dn4, locals.var_vsat_t_dn5, locals.var_vsat_t_dn6, locals.var_vsat_t_dn7, locals.var_vsat_t_dn8, locals.var_vsat_t_dn9, locals.var_vsat_t_dn10, locals.var_vsat_t_dn11, locals.var_vsat_t_dn13, locals.var_vsat_t_dn14,)
    }
};
        locals.var_vsat_t = assign15670_e24484;
        locals.var_vsat_t_dn0 = assign15670_e24484_d_n0;
        locals.var_vsat_t_dn2 = assign15670_e24484_d_n2;
        locals.var_vsat_t_dn3 = assign15670_e24484_d_n3;
        locals.var_vsat_t_dn4 = assign15670_e24484_d_n4;
        locals.var_vsat_t_dn5 = assign15670_e24484_d_n5;
        locals.var_vsat_t_dn6 = assign15670_e24484_d_n6;
        locals.var_vsat_t_dn7 = assign15670_e24484_d_n7;
        locals.var_vsat_t_dn8 = assign15670_e24484_d_n8;
        locals.var_vsat_t_dn9 = assign15670_e24484_d_n9;
        locals.var_vsat_t_dn10 = assign15670_e24484_d_n10;
        locals.var_vsat_t_dn11 = assign15670_e24484_d_n11;
        locals.var_vsat_t_dn13 = assign15670_e24484_d_n13;
        locals.var_vsat_t_dn14 = assign15670_e24484_d_n14;

        let assign15680_e24487: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard270 = assign15680_e24487;

        let assign15690_e24490: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard271 = assign15690_e24490;

        let (assign15700_e24562, assign15700_e24562_d_n4,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard270 != 0.0)) && (locals.var_guard271 != 0.0)) {
        let assign15700_e24501: f64 = (-locals.var_vsatr_i);
        let assign15700_e24504: f64 = (-locals.var_atr_i);
        let assign15700_e24506: f64 = (assign15700_e24504 * locals.var_deltemp);
        let assign15700_e24509: f64 = (p.p561 * locals.var_deltemp);
        let assign15700_e24511: f64 = (assign15700_e24509 * locals.var_deltemp);
        let assign15700_e24512: f64 = (assign15700_e24506 + assign15700_e24511);
        let assign15700_e24514: f64 = (-locals.var_vsatr_i);
        let assign15700_e24515: f64 = (assign15700_e24512 - assign15700_e24514);
        let assign15700_e24517: f64 = (assign15700_e24515 - 1e-6);
        let assign15700_e24519: f64 = (-locals.var_atr_i);
        let assign15700_e24521: f64 = (assign15700_e24519 * locals.var_deltemp);
        let assign15700_e24524: f64 = (p.p561 * locals.var_deltemp);
        let assign15700_e24526: f64 = (assign15700_e24524 * locals.var_deltemp);
        let assign15700_e24527: f64 = (assign15700_e24521 + assign15700_e24526);
        let assign15700_e24529: f64 = (-locals.var_vsatr_i);
        let assign15700_e24530: f64 = (assign15700_e24527 - assign15700_e24529);
        let assign15700_e24532: f64 = (assign15700_e24530 - 1e-6);
        let assign15700_e24534: f64 = (-locals.var_atr_i);
        let assign15700_e24536: f64 = (assign15700_e24534 * locals.var_deltemp);
        let assign15700_e24539: f64 = (p.p561 * locals.var_deltemp);
        let assign15700_e24541: f64 = (assign15700_e24539 * locals.var_deltemp);
        let assign15700_e24542: f64 = (assign15700_e24536 + assign15700_e24541);
        let assign15700_e24544: f64 = (-locals.var_vsatr_i);
        let assign15700_e24545: f64 = (assign15700_e24542 - assign15700_e24544);
        let assign15700_e24547: f64 = (assign15700_e24545 - 1e-6);
        let assign15700_e24548: f64 = (assign15700_e24532 * assign15700_e24547);
        let assign15700_e24551: f64 = (-locals.var_vsatr_i);
        let assign15700_e24552: f64 = (4.0 * assign15700_e24551);
        let assign15700_e24554: f64 = (assign15700_e24552 * 1e-6);
        let assign15700_e24555: f64 = (assign15700_e24548 - assign15700_e24554);
        let assign15700_e24556: f64 = (assign15700_e24555).sqrt();
        let assign15700_e24557: f64 = (assign15700_e24517 + assign15700_e24556);
        let assign15700_e24558: f64 = (0.5 * assign15700_e24557);
        let assign15700_e24559: f64 = (assign15700_e24501 + assign15700_e24558);
        let assign15700_e24560: f64 = (locals.var_vsatr_i + assign15700_e24559);
        (assign15700_e24560, (0.5 * (((assign15700_e24504 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15700_e24509 * locals.var_deltemp_dn4))) + (((((assign15700_e24519 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15700_e24524 * locals.var_deltemp_dn4))) * assign15700_e24547) + (assign15700_e24532 * ((assign15700_e24534 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15700_e24539 * locals.var_deltemp_dn4))))) / (2.0 * assign15700_e24556)))),)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn4,)
    }
};
        locals.var_vsatr_t = assign15700_e24562;
        locals.var_vsatr_t_dn4 = assign15700_e24562_d_n4;

        let (assign15710_e24689, assign15710_e24689_d_n4,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard270 != 0.0)) && (locals.var_guard271 == 0.0)) {
        let assign15710_e24575: f64 = (-locals.var_atr_i);
        let assign15710_e24577: f64 = (assign15710_e24575 * locals.var_deltemp);
        let assign15710_e24578: f64 = (1.0 + assign15710_e24577);
        let assign15710_e24581: f64 = (p.p561 * locals.var_deltemp);
        let assign15710_e24583: f64 = (assign15710_e24581 * locals.var_deltemp);
        let assign15710_e24584: f64 = (assign15710_e24578 + assign15710_e24583);
        let assign15710_e24586: f64 = (assign15710_e24584 - 1e-6);
        let assign15710_e24588: f64 = (-10000.0);
        let assign15710_e24590: f64 = (assign15710_e24588 * 0.001);
        let (assign15710_e24686, assign15710_e24686_d_n4,) = {
            if (!(assign15710_e24586 < assign15710_e24590)) {
                let assign15710_e24596: f64 = (-locals.var_atr_i);
                let assign15710_e24598: f64 = (assign15710_e24596 * locals.var_deltemp);
                let assign15710_e24599: f64 = (1.0 + assign15710_e24598);
                let assign15710_e24602: f64 = (p.p561 * locals.var_deltemp);
                let assign15710_e24604: f64 = (assign15710_e24602 * locals.var_deltemp);
                let assign15710_e24605: f64 = (assign15710_e24599 + assign15710_e24604);
                let assign15710_e24607: f64 = (assign15710_e24605 - 1e-6);
                let assign15710_e24610: f64 = (-locals.var_atr_i);
                let assign15710_e24612: f64 = (assign15710_e24610 * locals.var_deltemp);
                let assign15710_e24613: f64 = (1.0 + assign15710_e24612);
                let assign15710_e24616: f64 = (p.p561 * locals.var_deltemp);
                let assign15710_e24618: f64 = (assign15710_e24616 * locals.var_deltemp);
                let assign15710_e24619: f64 = (assign15710_e24613 + assign15710_e24618);
                let assign15710_e24621: f64 = (assign15710_e24619 - 1e-6);
                let assign15710_e24624: f64 = (-locals.var_atr_i);
                let assign15710_e24626: f64 = (assign15710_e24624 * locals.var_deltemp);
                let assign15710_e24627: f64 = (1.0 + assign15710_e24626);
                let assign15710_e24630: f64 = (p.p561 * locals.var_deltemp);
                let assign15710_e24632: f64 = (assign15710_e24630 * locals.var_deltemp);
                let assign15710_e24633: f64 = (assign15710_e24627 + assign15710_e24632);
                let assign15710_e24635: f64 = (assign15710_e24633 - 1e-6);
                let assign15710_e24636: f64 = (assign15710_e24621 * assign15710_e24635);
                let assign15710_e24639: f64 = (4.0 * 0.001);
                let assign15710_e24641: f64 = (assign15710_e24639 * 0.001);
                let assign15710_e24642: f64 = (assign15710_e24636 + assign15710_e24641);
                let assign15710_e24643: f64 = (assign15710_e24642).sqrt();
                let assign15710_e24644: f64 = (assign15710_e24607 + assign15710_e24643);
                let assign15710_e24645: f64 = (0.5 * assign15710_e24644);
                (assign15710_e24645, (0.5 * (((assign15710_e24596 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15710_e24602 * locals.var_deltemp_dn4))) + (((((assign15710_e24610 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15710_e24616 * locals.var_deltemp_dn4))) * assign15710_e24635) + (assign15710_e24621 * ((assign15710_e24624 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15710_e24630 * locals.var_deltemp_dn4))))) / (2.0 * assign15710_e24643)))),)
            } else {
                let assign15710_e24648: f64 = (-locals.var_atr_i);
                let assign15710_e24650: f64 = (assign15710_e24648 * locals.var_deltemp);
                let assign15710_e24651: f64 = (1.0 + assign15710_e24650);
                let assign15710_e24654: f64 = (p.p561 * locals.var_deltemp);
                let assign15710_e24656: f64 = (assign15710_e24654 * locals.var_deltemp);
                let assign15710_e24657: f64 = (assign15710_e24651 + assign15710_e24656);
                let assign15710_e24659: f64 = (assign15710_e24657 - 1e-6);
                let assign15710_e24661: f64 = (-10000.0);
                let assign15710_e24663: f64 = (assign15710_e24661 * 0.001);
                let (assign15710_e24685, assign15710_e24685_d_n4,) = {
                    if (assign15710_e24659 < assign15710_e24663) {
                        let assign15710_e24666: f64 = (-0.001);
                        let assign15710_e24668: f64 = (assign15710_e24666 * 0.001);
                        let assign15710_e24671: f64 = (-locals.var_atr_i);
                        let assign15710_e24673: f64 = (assign15710_e24671 * locals.var_deltemp);
                        let assign15710_e24674: f64 = (1.0 + assign15710_e24673);
                        let assign15710_e24677: f64 = (p.p561 * locals.var_deltemp);
                        let assign15710_e24679: f64 = (assign15710_e24677 * locals.var_deltemp);
                        let assign15710_e24680: f64 = (assign15710_e24674 + assign15710_e24679);
                        let assign15710_e24682: f64 = (assign15710_e24680 - 1e-6);
                        let assign15710_e24683: f64 = (assign15710_e24668 / assign15710_e24682);
                        (assign15710_e24683, (-((assign15710_e24668 * ((assign15710_e24671 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15710_e24677 * locals.var_deltemp_dn4)))) / (assign15710_e24682 * assign15710_e24682))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15710_e24685, assign15710_e24685_d_n4,)
            }
        };
        let assign15710_e24687: f64 = (locals.var_vsatr_i * assign15710_e24686);
        (assign15710_e24687, (locals.var_vsatr_i * assign15710_e24686_d_n4),)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn4,)
    }
};
        locals.var_vsatr_t = assign15710_e24689;
        locals.var_vsatr_t_dn4 = assign15710_e24689_d_n4;

        let assign15720_e24692: f64 = if locals.var_vsatr_t < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard272 = assign15720_e24692;

        let (assign15730_e24703, assign15730_e24703_d_n4,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard270 != 0.0)) && (locals.var_guard272 != 0.0)) {
        (1000.0, 0.0,)
    } else {
        (locals.var_vsatr_t, locals.var_vsatr_t_dn4,)
    }
};
        locals.var_vsatr_t = assign15730_e24703;
        locals.var_vsatr_t_dn4 = assign15730_e24703_d_n4;

        let assign15740_e24706: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard273 = assign15740_e24706;

        let (assign15750_e24776, assign15750_e24776_d_n0, assign15750_e24776_d_n2, assign15750_e24776_d_n3, assign15750_e24776_d_n4, assign15750_e24776_d_n5, assign15750_e24776_d_n6, assign15750_e24776_d_n7, assign15750_e24776_d_n8, assign15750_e24776_d_n9, assign15750_e24776_d_n10, assign15750_e24776_d_n11, assign15750_e24776_d_n13, assign15750_e24776_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard273 != 0.0)) {
        let assign15750_e24715: f64 = (-locals.var_vsat1_i);
        let assign15750_e24718: f64 = (-locals.var_at_i);
        let assign15750_e24720: f64 = (assign15750_e24718 * locals.var_deltemp);
        let assign15750_e24723: f64 = (p.p561 * locals.var_deltemp);
        let assign15750_e24725: f64 = (assign15750_e24723 * locals.var_deltemp);
        let assign15750_e24726: f64 = (assign15750_e24720 + assign15750_e24725);
        let assign15750_e24728: f64 = (-locals.var_vsat1_i);
        let assign15750_e24729: f64 = (assign15750_e24726 - assign15750_e24728);
        let assign15750_e24731: f64 = (assign15750_e24729 - 1e-6);
        let assign15750_e24733: f64 = (-locals.var_at_i);
        let assign15750_e24735: f64 = (assign15750_e24733 * locals.var_deltemp);
        let assign15750_e24738: f64 = (p.p561 * locals.var_deltemp);
        let assign15750_e24740: f64 = (assign15750_e24738 * locals.var_deltemp);
        let assign15750_e24741: f64 = (assign15750_e24735 + assign15750_e24740);
        let assign15750_e24743: f64 = (-locals.var_vsat1_i);
        let assign15750_e24744: f64 = (assign15750_e24741 - assign15750_e24743);
        let assign15750_e24746: f64 = (assign15750_e24744 - 1e-6);
        let assign15750_e24748: f64 = (-locals.var_at_i);
        let assign15750_e24750: f64 = (assign15750_e24748 * locals.var_deltemp);
        let assign15750_e24753: f64 = (p.p561 * locals.var_deltemp);
        let assign15750_e24755: f64 = (assign15750_e24753 * locals.var_deltemp);
        let assign15750_e24756: f64 = (assign15750_e24750 + assign15750_e24755);
        let assign15750_e24758: f64 = (-locals.var_vsat1_i);
        let assign15750_e24759: f64 = (assign15750_e24756 - assign15750_e24758);
        let assign15750_e24761: f64 = (assign15750_e24759 - 1e-6);
        let assign15750_e24762: f64 = (assign15750_e24746 * assign15750_e24761);
        let assign15750_e24765: f64 = (-locals.var_vsat1_i);
        let assign15750_e24766: f64 = (4.0 * assign15750_e24765);
        let assign15750_e24768: f64 = (assign15750_e24766 * 1e-6);
        let assign15750_e24769: f64 = (assign15750_e24762 - assign15750_e24768);
        let assign15750_e24770: f64 = (assign15750_e24769).sqrt();
        let assign15750_e24771: f64 = (assign15750_e24731 + assign15750_e24770);
        let assign15750_e24772: f64 = (0.5 * assign15750_e24771);
        let assign15750_e24773: f64 = (assign15750_e24715 + assign15750_e24772);
        let assign15750_e24774: f64 = (locals.var_vsat1_i + assign15750_e24773);
        (assign15750_e24774, (locals.var_vsat1_i_dn0 + ((-locals.var_vsat1_i_dn0) + (0.5 * ((-(-locals.var_vsat1_i_dn0)) + (((((-(-locals.var_vsat1_i_dn0)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn0)))) - ((4.0 * (-locals.var_vsat1_i_dn0)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn2 + ((-locals.var_vsat1_i_dn2) + (0.5 * ((-(-locals.var_vsat1_i_dn2)) + (((((-(-locals.var_vsat1_i_dn2)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn2)))) - ((4.0 * (-locals.var_vsat1_i_dn2)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn3 + ((-locals.var_vsat1_i_dn3) + (0.5 * ((-(-locals.var_vsat1_i_dn3)) + (((((-(-locals.var_vsat1_i_dn3)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn3)))) - ((4.0 * (-locals.var_vsat1_i_dn3)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn4 + ((-locals.var_vsat1_i_dn4) + (0.5 * ((((assign15750_e24718 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15750_e24723 * locals.var_deltemp_dn4))) - (-locals.var_vsat1_i_dn4)) + (((((((assign15750_e24733 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15750_e24738 * locals.var_deltemp_dn4))) - (-locals.var_vsat1_i_dn4)) * assign15750_e24761) + (assign15750_e24746 * (((assign15750_e24748 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15750_e24753 * locals.var_deltemp_dn4))) - (-locals.var_vsat1_i_dn4)))) - ((4.0 * (-locals.var_vsat1_i_dn4)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn5 + ((-locals.var_vsat1_i_dn5) + (0.5 * ((-(-locals.var_vsat1_i_dn5)) + (((((-(-locals.var_vsat1_i_dn5)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn5)))) - ((4.0 * (-locals.var_vsat1_i_dn5)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn6 + ((-locals.var_vsat1_i_dn6) + (0.5 * ((-(-locals.var_vsat1_i_dn6)) + (((((-(-locals.var_vsat1_i_dn6)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn6)))) - ((4.0 * (-locals.var_vsat1_i_dn6)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn7 + ((-locals.var_vsat1_i_dn7) + (0.5 * ((-(-locals.var_vsat1_i_dn7)) + (((((-(-locals.var_vsat1_i_dn7)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn7)))) - ((4.0 * (-locals.var_vsat1_i_dn7)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn8 + ((-locals.var_vsat1_i_dn8) + (0.5 * ((-(-locals.var_vsat1_i_dn8)) + (((((-(-locals.var_vsat1_i_dn8)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn8)))) - ((4.0 * (-locals.var_vsat1_i_dn8)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn9 + ((-locals.var_vsat1_i_dn9) + (0.5 * ((-(-locals.var_vsat1_i_dn9)) + (((((-(-locals.var_vsat1_i_dn9)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn9)))) - ((4.0 * (-locals.var_vsat1_i_dn9)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn10 + ((-locals.var_vsat1_i_dn10) + (0.5 * ((-(-locals.var_vsat1_i_dn10)) + (((((-(-locals.var_vsat1_i_dn10)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn10)))) - ((4.0 * (-locals.var_vsat1_i_dn10)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn11 + ((-locals.var_vsat1_i_dn11) + (0.5 * ((-(-locals.var_vsat1_i_dn11)) + (((((-(-locals.var_vsat1_i_dn11)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn11)))) - ((4.0 * (-locals.var_vsat1_i_dn11)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn13 + ((-locals.var_vsat1_i_dn13) + (0.5 * ((-(-locals.var_vsat1_i_dn13)) + (((((-(-locals.var_vsat1_i_dn13)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn13)))) - ((4.0 * (-locals.var_vsat1_i_dn13)) * 1e-6)) / (2.0 * assign15750_e24770)))))), (locals.var_vsat1_i_dn14 + ((-locals.var_vsat1_i_dn14) + (0.5 * ((-(-locals.var_vsat1_i_dn14)) + (((((-(-locals.var_vsat1_i_dn14)) * assign15750_e24761) + (assign15750_e24746 * (-(-locals.var_vsat1_i_dn14)))) - ((4.0 * (-locals.var_vsat1_i_dn14)) * 1e-6)) / (2.0 * assign15750_e24770)))))),)
    } else {
        (locals.var_vsat1_t, locals.var_vsat1_t_dn0, locals.var_vsat1_t_dn2, locals.var_vsat1_t_dn3, locals.var_vsat1_t_dn4, locals.var_vsat1_t_dn5, locals.var_vsat1_t_dn6, locals.var_vsat1_t_dn7, locals.var_vsat1_t_dn8, locals.var_vsat1_t_dn9, locals.var_vsat1_t_dn10, locals.var_vsat1_t_dn11, locals.var_vsat1_t_dn13, locals.var_vsat1_t_dn14,)
    }
};
        locals.var_vsat1_t = assign15750_e24776;
        locals.var_vsat1_t_dn0 = assign15750_e24776_d_n0;
        locals.var_vsat1_t_dn2 = assign15750_e24776_d_n2;
        locals.var_vsat1_t_dn3 = assign15750_e24776_d_n3;
        locals.var_vsat1_t_dn4 = assign15750_e24776_d_n4;
        locals.var_vsat1_t_dn5 = assign15750_e24776_d_n5;
        locals.var_vsat1_t_dn6 = assign15750_e24776_d_n6;
        locals.var_vsat1_t_dn7 = assign15750_e24776_d_n7;
        locals.var_vsat1_t_dn8 = assign15750_e24776_d_n8;
        locals.var_vsat1_t_dn9 = assign15750_e24776_d_n9;
        locals.var_vsat1_t_dn10 = assign15750_e24776_d_n10;
        locals.var_vsat1_t_dn11 = assign15750_e24776_d_n11;
        locals.var_vsat1_t_dn13 = assign15750_e24776_d_n13;
        locals.var_vsat1_t_dn14 = assign15750_e24776_d_n14;

    }

    pub(super) fn stamp_transient_block_47(
        p: &Parameters,
        locals: &mut StampLocals,
    ) {
        let (assign15760_e24901, assign15760_e24901_d_n0, assign15760_e24901_d_n2, assign15760_e24901_d_n3, assign15760_e24901_d_n4, assign15760_e24901_d_n5, assign15760_e24901_d_n6, assign15760_e24901_d_n7, assign15760_e24901_d_n8, assign15760_e24901_d_n9, assign15760_e24901_d_n10, assign15760_e24901_d_n11, assign15760_e24901_d_n13, assign15760_e24901_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard273 == 0.0)) {
        let assign15760_e24787: f64 = (-locals.var_at_i);
        let assign15760_e24789: f64 = (assign15760_e24787 * locals.var_deltemp);
        let assign15760_e24790: f64 = (1.0 + assign15760_e24789);
        let assign15760_e24793: f64 = (p.p561 * locals.var_deltemp);
        let assign15760_e24795: f64 = (assign15760_e24793 * locals.var_deltemp);
        let assign15760_e24796: f64 = (assign15760_e24790 + assign15760_e24795);
        let assign15760_e24798: f64 = (assign15760_e24796 - 1e-6);
        let assign15760_e24800: f64 = (-10000.0);
        let assign15760_e24802: f64 = (assign15760_e24800 * 0.001);
        let (assign15760_e24898, assign15760_e24898_d_n4,) = {
            if (!(assign15760_e24798 < assign15760_e24802)) {
                let assign15760_e24808: f64 = (-locals.var_at_i);
                let assign15760_e24810: f64 = (assign15760_e24808 * locals.var_deltemp);
                let assign15760_e24811: f64 = (1.0 + assign15760_e24810);
                let assign15760_e24814: f64 = (p.p561 * locals.var_deltemp);
                let assign15760_e24816: f64 = (assign15760_e24814 * locals.var_deltemp);
                let assign15760_e24817: f64 = (assign15760_e24811 + assign15760_e24816);
                let assign15760_e24819: f64 = (assign15760_e24817 - 1e-6);
                let assign15760_e24822: f64 = (-locals.var_at_i);
                let assign15760_e24824: f64 = (assign15760_e24822 * locals.var_deltemp);
                let assign15760_e24825: f64 = (1.0 + assign15760_e24824);
                let assign15760_e24828: f64 = (p.p561 * locals.var_deltemp);
                let assign15760_e24830: f64 = (assign15760_e24828 * locals.var_deltemp);
                let assign15760_e24831: f64 = (assign15760_e24825 + assign15760_e24830);
                let assign15760_e24833: f64 = (assign15760_e24831 - 1e-6);
                let assign15760_e24836: f64 = (-locals.var_at_i);
                let assign15760_e24838: f64 = (assign15760_e24836 * locals.var_deltemp);
                let assign15760_e24839: f64 = (1.0 + assign15760_e24838);
                let assign15760_e24842: f64 = (p.p561 * locals.var_deltemp);
                let assign15760_e24844: f64 = (assign15760_e24842 * locals.var_deltemp);
                let assign15760_e24845: f64 = (assign15760_e24839 + assign15760_e24844);
                let assign15760_e24847: f64 = (assign15760_e24845 - 1e-6);
                let assign15760_e24848: f64 = (assign15760_e24833 * assign15760_e24847);
                let assign15760_e24851: f64 = (4.0 * 0.001);
                let assign15760_e24853: f64 = (assign15760_e24851 * 0.001);
                let assign15760_e24854: f64 = (assign15760_e24848 + assign15760_e24853);
                let assign15760_e24855: f64 = (assign15760_e24854).sqrt();
                let assign15760_e24856: f64 = (assign15760_e24819 + assign15760_e24855);
                let assign15760_e24857: f64 = (0.5 * assign15760_e24856);
                (assign15760_e24857, (0.5 * (((assign15760_e24808 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15760_e24814 * locals.var_deltemp_dn4))) + (((((assign15760_e24822 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15760_e24828 * locals.var_deltemp_dn4))) * assign15760_e24847) + (assign15760_e24833 * ((assign15760_e24836 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15760_e24842 * locals.var_deltemp_dn4))))) / (2.0 * assign15760_e24855)))),)
            } else {
                let assign15760_e24860: f64 = (-locals.var_at_i);
                let assign15760_e24862: f64 = (assign15760_e24860 * locals.var_deltemp);
                let assign15760_e24863: f64 = (1.0 + assign15760_e24862);
                let assign15760_e24866: f64 = (p.p561 * locals.var_deltemp);
                let assign15760_e24868: f64 = (assign15760_e24866 * locals.var_deltemp);
                let assign15760_e24869: f64 = (assign15760_e24863 + assign15760_e24868);
                let assign15760_e24871: f64 = (assign15760_e24869 - 1e-6);
                let assign15760_e24873: f64 = (-10000.0);
                let assign15760_e24875: f64 = (assign15760_e24873 * 0.001);
                let (assign15760_e24897, assign15760_e24897_d_n4,) = {
                    if (assign15760_e24871 < assign15760_e24875) {
                        let assign15760_e24878: f64 = (-0.001);
                        let assign15760_e24880: f64 = (assign15760_e24878 * 0.001);
                        let assign15760_e24883: f64 = (-locals.var_at_i);
                        let assign15760_e24885: f64 = (assign15760_e24883 * locals.var_deltemp);
                        let assign15760_e24886: f64 = (1.0 + assign15760_e24885);
                        let assign15760_e24889: f64 = (p.p561 * locals.var_deltemp);
                        let assign15760_e24891: f64 = (assign15760_e24889 * locals.var_deltemp);
                        let assign15760_e24892: f64 = (assign15760_e24886 + assign15760_e24891);
                        let assign15760_e24894: f64 = (assign15760_e24892 - 1e-6);
                        let assign15760_e24895: f64 = (assign15760_e24880 / assign15760_e24894);
                        (assign15760_e24895, (-((assign15760_e24880 * ((assign15760_e24883 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15760_e24889 * locals.var_deltemp_dn4)))) / (assign15760_e24894 * assign15760_e24894))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15760_e24897, assign15760_e24897_d_n4,)
            }
        };
        let assign15760_e24899: f64 = (locals.var_vsat1_i * assign15760_e24898);
        (assign15760_e24899, (locals.var_vsat1_i_dn0 * assign15760_e24898), (locals.var_vsat1_i_dn2 * assign15760_e24898), (locals.var_vsat1_i_dn3 * assign15760_e24898), ((locals.var_vsat1_i_dn4 * assign15760_e24898) + (locals.var_vsat1_i * assign15760_e24898_d_n4)), (locals.var_vsat1_i_dn5 * assign15760_e24898), (locals.var_vsat1_i_dn6 * assign15760_e24898), (locals.var_vsat1_i_dn7 * assign15760_e24898), (locals.var_vsat1_i_dn8 * assign15760_e24898), (locals.var_vsat1_i_dn9 * assign15760_e24898), (locals.var_vsat1_i_dn10 * assign15760_e24898), (locals.var_vsat1_i_dn11 * assign15760_e24898), (locals.var_vsat1_i_dn13 * assign15760_e24898), (locals.var_vsat1_i_dn14 * assign15760_e24898),)
    } else {
        (locals.var_vsat1_t, locals.var_vsat1_t_dn0, locals.var_vsat1_t_dn2, locals.var_vsat1_t_dn3, locals.var_vsat1_t_dn4, locals.var_vsat1_t_dn5, locals.var_vsat1_t_dn6, locals.var_vsat1_t_dn7, locals.var_vsat1_t_dn8, locals.var_vsat1_t_dn9, locals.var_vsat1_t_dn10, locals.var_vsat1_t_dn11, locals.var_vsat1_t_dn13, locals.var_vsat1_t_dn14,)
    }
};
        locals.var_vsat1_t = assign15760_e24901;
        locals.var_vsat1_t_dn0 = assign15760_e24901_d_n0;
        locals.var_vsat1_t_dn2 = assign15760_e24901_d_n2;
        locals.var_vsat1_t_dn3 = assign15760_e24901_d_n3;
        locals.var_vsat1_t_dn4 = assign15760_e24901_d_n4;
        locals.var_vsat1_t_dn5 = assign15760_e24901_d_n5;
        locals.var_vsat1_t_dn6 = assign15760_e24901_d_n6;
        locals.var_vsat1_t_dn7 = assign15760_e24901_d_n7;
        locals.var_vsat1_t_dn8 = assign15760_e24901_d_n8;
        locals.var_vsat1_t_dn9 = assign15760_e24901_d_n9;
        locals.var_vsat1_t_dn10 = assign15760_e24901_d_n10;
        locals.var_vsat1_t_dn11 = assign15760_e24901_d_n11;
        locals.var_vsat1_t_dn13 = assign15760_e24901_d_n13;
        locals.var_vsat1_t_dn14 = assign15760_e24901_d_n14;

        let assign15770_e24904: f64 = if p.p66 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard274 = assign15770_e24904;

        let assign15780_e24907: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard275 = assign15780_e24907;

        let (assign15790_e24979, assign15790_e24979_d_n0, assign15790_e24979_d_n2, assign15790_e24979_d_n3, assign15790_e24979_d_n4, assign15790_e24979_d_n5, assign15790_e24979_d_n6, assign15790_e24979_d_n7, assign15790_e24979_d_n8, assign15790_e24979_d_n9, assign15790_e24979_d_n10, assign15790_e24979_d_n11, assign15790_e24979_d_n13, assign15790_e24979_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard274 != 0.0)) && (locals.var_guard275 != 0.0)) {
        let assign15790_e24918: f64 = (-locals.var_vsat1r_i);
        let assign15790_e24921: f64 = (-locals.var_at_i);
        let assign15790_e24923: f64 = (assign15790_e24921 * locals.var_deltemp);
        let assign15790_e24926: f64 = (p.p561 * locals.var_deltemp);
        let assign15790_e24928: f64 = (assign15790_e24926 * locals.var_deltemp);
        let assign15790_e24929: f64 = (assign15790_e24923 + assign15790_e24928);
        let assign15790_e24931: f64 = (-locals.var_vsat1r_i);
        let assign15790_e24932: f64 = (assign15790_e24929 - assign15790_e24931);
        let assign15790_e24934: f64 = (assign15790_e24932 - 1e-6);
        let assign15790_e24936: f64 = (-locals.var_at_i);
        let assign15790_e24938: f64 = (assign15790_e24936 * locals.var_deltemp);
        let assign15790_e24941: f64 = (p.p561 * locals.var_deltemp);
        let assign15790_e24943: f64 = (assign15790_e24941 * locals.var_deltemp);
        let assign15790_e24944: f64 = (assign15790_e24938 + assign15790_e24943);
        let assign15790_e24946: f64 = (-locals.var_vsat1r_i);
        let assign15790_e24947: f64 = (assign15790_e24944 - assign15790_e24946);
        let assign15790_e24949: f64 = (assign15790_e24947 - 1e-6);
        let assign15790_e24951: f64 = (-locals.var_at_i);
        let assign15790_e24953: f64 = (assign15790_e24951 * locals.var_deltemp);
        let assign15790_e24956: f64 = (p.p561 * locals.var_deltemp);
        let assign15790_e24958: f64 = (assign15790_e24956 * locals.var_deltemp);
        let assign15790_e24959: f64 = (assign15790_e24953 + assign15790_e24958);
        let assign15790_e24961: f64 = (-locals.var_vsat1r_i);
        let assign15790_e24962: f64 = (assign15790_e24959 - assign15790_e24961);
        let assign15790_e24964: f64 = (assign15790_e24962 - 1e-6);
        let assign15790_e24965: f64 = (assign15790_e24949 * assign15790_e24964);
        let assign15790_e24968: f64 = (-locals.var_vsat1r_i);
        let assign15790_e24969: f64 = (4.0 * assign15790_e24968);
        let assign15790_e24971: f64 = (assign15790_e24969 * 1e-6);
        let assign15790_e24972: f64 = (assign15790_e24965 - assign15790_e24971);
        let assign15790_e24973: f64 = (assign15790_e24972).sqrt();
        let assign15790_e24974: f64 = (assign15790_e24934 + assign15790_e24973);
        let assign15790_e24975: f64 = (0.5 * assign15790_e24974);
        let assign15790_e24976: f64 = (assign15790_e24918 + assign15790_e24975);
        let assign15790_e24977: f64 = (locals.var_vsat1r_i + assign15790_e24976);
        (assign15790_e24977, (locals.var_vsat1r_i_dn0 + ((-locals.var_vsat1r_i_dn0) + (0.5 * ((-(-locals.var_vsat1r_i_dn0)) + (((((-(-locals.var_vsat1r_i_dn0)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn0)))) - ((4.0 * (-locals.var_vsat1r_i_dn0)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn2 + ((-locals.var_vsat1r_i_dn2) + (0.5 * ((-(-locals.var_vsat1r_i_dn2)) + (((((-(-locals.var_vsat1r_i_dn2)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn2)))) - ((4.0 * (-locals.var_vsat1r_i_dn2)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn3 + ((-locals.var_vsat1r_i_dn3) + (0.5 * ((-(-locals.var_vsat1r_i_dn3)) + (((((-(-locals.var_vsat1r_i_dn3)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn3)))) - ((4.0 * (-locals.var_vsat1r_i_dn3)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn4 + ((-locals.var_vsat1r_i_dn4) + (0.5 * ((((assign15790_e24921 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15790_e24926 * locals.var_deltemp_dn4))) - (-locals.var_vsat1r_i_dn4)) + (((((((assign15790_e24936 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15790_e24941 * locals.var_deltemp_dn4))) - (-locals.var_vsat1r_i_dn4)) * assign15790_e24964) + (assign15790_e24949 * (((assign15790_e24951 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15790_e24956 * locals.var_deltemp_dn4))) - (-locals.var_vsat1r_i_dn4)))) - ((4.0 * (-locals.var_vsat1r_i_dn4)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn5 + ((-locals.var_vsat1r_i_dn5) + (0.5 * ((-(-locals.var_vsat1r_i_dn5)) + (((((-(-locals.var_vsat1r_i_dn5)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn5)))) - ((4.0 * (-locals.var_vsat1r_i_dn5)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn6 + ((-locals.var_vsat1r_i_dn6) + (0.5 * ((-(-locals.var_vsat1r_i_dn6)) + (((((-(-locals.var_vsat1r_i_dn6)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn6)))) - ((4.0 * (-locals.var_vsat1r_i_dn6)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn7 + ((-locals.var_vsat1r_i_dn7) + (0.5 * ((-(-locals.var_vsat1r_i_dn7)) + (((((-(-locals.var_vsat1r_i_dn7)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn7)))) - ((4.0 * (-locals.var_vsat1r_i_dn7)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn8 + ((-locals.var_vsat1r_i_dn8) + (0.5 * ((-(-locals.var_vsat1r_i_dn8)) + (((((-(-locals.var_vsat1r_i_dn8)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn8)))) - ((4.0 * (-locals.var_vsat1r_i_dn8)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn9 + ((-locals.var_vsat1r_i_dn9) + (0.5 * ((-(-locals.var_vsat1r_i_dn9)) + (((((-(-locals.var_vsat1r_i_dn9)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn9)))) - ((4.0 * (-locals.var_vsat1r_i_dn9)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn10 + ((-locals.var_vsat1r_i_dn10) + (0.5 * ((-(-locals.var_vsat1r_i_dn10)) + (((((-(-locals.var_vsat1r_i_dn10)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn10)))) - ((4.0 * (-locals.var_vsat1r_i_dn10)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn11 + ((-locals.var_vsat1r_i_dn11) + (0.5 * ((-(-locals.var_vsat1r_i_dn11)) + (((((-(-locals.var_vsat1r_i_dn11)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn11)))) - ((4.0 * (-locals.var_vsat1r_i_dn11)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn13 + ((-locals.var_vsat1r_i_dn13) + (0.5 * ((-(-locals.var_vsat1r_i_dn13)) + (((((-(-locals.var_vsat1r_i_dn13)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn13)))) - ((4.0 * (-locals.var_vsat1r_i_dn13)) * 1e-6)) / (2.0 * assign15790_e24973)))))), (locals.var_vsat1r_i_dn14 + ((-locals.var_vsat1r_i_dn14) + (0.5 * ((-(-locals.var_vsat1r_i_dn14)) + (((((-(-locals.var_vsat1r_i_dn14)) * assign15790_e24964) + (assign15790_e24949 * (-(-locals.var_vsat1r_i_dn14)))) - ((4.0 * (-locals.var_vsat1r_i_dn14)) * 1e-6)) / (2.0 * assign15790_e24973)))))),)
    } else {
        (locals.var_vsat1r_t, locals.var_vsat1r_t_dn0, locals.var_vsat1r_t_dn2, locals.var_vsat1r_t_dn3, locals.var_vsat1r_t_dn4, locals.var_vsat1r_t_dn5, locals.var_vsat1r_t_dn6, locals.var_vsat1r_t_dn7, locals.var_vsat1r_t_dn8, locals.var_vsat1r_t_dn9, locals.var_vsat1r_t_dn10, locals.var_vsat1r_t_dn11, locals.var_vsat1r_t_dn13, locals.var_vsat1r_t_dn14,)
    }
};
        locals.var_vsat1r_t = assign15790_e24979;
        locals.var_vsat1r_t_dn0 = assign15790_e24979_d_n0;
        locals.var_vsat1r_t_dn2 = assign15790_e24979_d_n2;
        locals.var_vsat1r_t_dn3 = assign15790_e24979_d_n3;
        locals.var_vsat1r_t_dn4 = assign15790_e24979_d_n4;
        locals.var_vsat1r_t_dn5 = assign15790_e24979_d_n5;
        locals.var_vsat1r_t_dn6 = assign15790_e24979_d_n6;
        locals.var_vsat1r_t_dn7 = assign15790_e24979_d_n7;
        locals.var_vsat1r_t_dn8 = assign15790_e24979_d_n8;
        locals.var_vsat1r_t_dn9 = assign15790_e24979_d_n9;
        locals.var_vsat1r_t_dn10 = assign15790_e24979_d_n10;
        locals.var_vsat1r_t_dn11 = assign15790_e24979_d_n11;
        locals.var_vsat1r_t_dn13 = assign15790_e24979_d_n13;
        locals.var_vsat1r_t_dn14 = assign15790_e24979_d_n14;

        let (assign15800_e25106, assign15800_e25106_d_n0, assign15800_e25106_d_n2, assign15800_e25106_d_n3, assign15800_e25106_d_n4, assign15800_e25106_d_n5, assign15800_e25106_d_n6, assign15800_e25106_d_n7, assign15800_e25106_d_n8, assign15800_e25106_d_n9, assign15800_e25106_d_n10, assign15800_e25106_d_n11, assign15800_e25106_d_n13, assign15800_e25106_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard274 != 0.0)) && (locals.var_guard275 == 0.0)) {
        let assign15800_e24992: f64 = (-locals.var_at_i);
        let assign15800_e24994: f64 = (assign15800_e24992 * locals.var_deltemp);
        let assign15800_e24995: f64 = (1.0 + assign15800_e24994);
        let assign15800_e24998: f64 = (p.p561 * locals.var_deltemp);
        let assign15800_e25000: f64 = (assign15800_e24998 * locals.var_deltemp);
        let assign15800_e25001: f64 = (assign15800_e24995 + assign15800_e25000);
        let assign15800_e25003: f64 = (assign15800_e25001 - 1e-6);
        let assign15800_e25005: f64 = (-10000.0);
        let assign15800_e25007: f64 = (assign15800_e25005 * 0.001);
        let (assign15800_e25103, assign15800_e25103_d_n4,) = {
            if (!(assign15800_e25003 < assign15800_e25007)) {
                let assign15800_e25013: f64 = (-locals.var_at_i);
                let assign15800_e25015: f64 = (assign15800_e25013 * locals.var_deltemp);
                let assign15800_e25016: f64 = (1.0 + assign15800_e25015);
                let assign15800_e25019: f64 = (p.p561 * locals.var_deltemp);
                let assign15800_e25021: f64 = (assign15800_e25019 * locals.var_deltemp);
                let assign15800_e25022: f64 = (assign15800_e25016 + assign15800_e25021);
                let assign15800_e25024: f64 = (assign15800_e25022 - 1e-6);
                let assign15800_e25027: f64 = (-locals.var_at_i);
                let assign15800_e25029: f64 = (assign15800_e25027 * locals.var_deltemp);
                let assign15800_e25030: f64 = (1.0 + assign15800_e25029);
                let assign15800_e25033: f64 = (p.p561 * locals.var_deltemp);
                let assign15800_e25035: f64 = (assign15800_e25033 * locals.var_deltemp);
                let assign15800_e25036: f64 = (assign15800_e25030 + assign15800_e25035);
                let assign15800_e25038: f64 = (assign15800_e25036 - 1e-6);
                let assign15800_e25041: f64 = (-locals.var_at_i);
                let assign15800_e25043: f64 = (assign15800_e25041 * locals.var_deltemp);
                let assign15800_e25044: f64 = (1.0 + assign15800_e25043);
                let assign15800_e25047: f64 = (p.p561 * locals.var_deltemp);
                let assign15800_e25049: f64 = (assign15800_e25047 * locals.var_deltemp);
                let assign15800_e25050: f64 = (assign15800_e25044 + assign15800_e25049);
                let assign15800_e25052: f64 = (assign15800_e25050 - 1e-6);
                let assign15800_e25053: f64 = (assign15800_e25038 * assign15800_e25052);
                let assign15800_e25056: f64 = (4.0 * 0.001);
                let assign15800_e25058: f64 = (assign15800_e25056 * 0.001);
                let assign15800_e25059: f64 = (assign15800_e25053 + assign15800_e25058);
                let assign15800_e25060: f64 = (assign15800_e25059).sqrt();
                let assign15800_e25061: f64 = (assign15800_e25024 + assign15800_e25060);
                let assign15800_e25062: f64 = (0.5 * assign15800_e25061);
                (assign15800_e25062, (0.5 * (((assign15800_e25013 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15800_e25019 * locals.var_deltemp_dn4))) + (((((assign15800_e25027 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15800_e25033 * locals.var_deltemp_dn4))) * assign15800_e25052) + (assign15800_e25038 * ((assign15800_e25041 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15800_e25047 * locals.var_deltemp_dn4))))) / (2.0 * assign15800_e25060)))),)
            } else {
                let assign15800_e25065: f64 = (-locals.var_at_i);
                let assign15800_e25067: f64 = (assign15800_e25065 * locals.var_deltemp);
                let assign15800_e25068: f64 = (1.0 + assign15800_e25067);
                let assign15800_e25071: f64 = (p.p561 * locals.var_deltemp);
                let assign15800_e25073: f64 = (assign15800_e25071 * locals.var_deltemp);
                let assign15800_e25074: f64 = (assign15800_e25068 + assign15800_e25073);
                let assign15800_e25076: f64 = (assign15800_e25074 - 1e-6);
                let assign15800_e25078: f64 = (-10000.0);
                let assign15800_e25080: f64 = (assign15800_e25078 * 0.001);
                let (assign15800_e25102, assign15800_e25102_d_n4,) = {
                    if (assign15800_e25076 < assign15800_e25080) {
                        let assign15800_e25083: f64 = (-0.001);
                        let assign15800_e25085: f64 = (assign15800_e25083 * 0.001);
                        let assign15800_e25088: f64 = (-locals.var_at_i);
                        let assign15800_e25090: f64 = (assign15800_e25088 * locals.var_deltemp);
                        let assign15800_e25091: f64 = (1.0 + assign15800_e25090);
                        let assign15800_e25094: f64 = (p.p561 * locals.var_deltemp);
                        let assign15800_e25096: f64 = (assign15800_e25094 * locals.var_deltemp);
                        let assign15800_e25097: f64 = (assign15800_e25091 + assign15800_e25096);
                        let assign15800_e25099: f64 = (assign15800_e25097 - 1e-6);
                        let assign15800_e25100: f64 = (assign15800_e25085 / assign15800_e25099);
                        (assign15800_e25100, (-((assign15800_e25085 * ((assign15800_e25088 * locals.var_deltemp_dn4) + (((p.p561 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15800_e25094 * locals.var_deltemp_dn4)))) / (assign15800_e25099 * assign15800_e25099))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15800_e25102, assign15800_e25102_d_n4,)
            }
        };
        let assign15800_e25104: f64 = (locals.var_vsat1r_i * assign15800_e25103);
        (assign15800_e25104, (locals.var_vsat1r_i_dn0 * assign15800_e25103), (locals.var_vsat1r_i_dn2 * assign15800_e25103), (locals.var_vsat1r_i_dn3 * assign15800_e25103), ((locals.var_vsat1r_i_dn4 * assign15800_e25103) + (locals.var_vsat1r_i * assign15800_e25103_d_n4)), (locals.var_vsat1r_i_dn5 * assign15800_e25103), (locals.var_vsat1r_i_dn6 * assign15800_e25103), (locals.var_vsat1r_i_dn7 * assign15800_e25103), (locals.var_vsat1r_i_dn8 * assign15800_e25103), (locals.var_vsat1r_i_dn9 * assign15800_e25103), (locals.var_vsat1r_i_dn10 * assign15800_e25103), (locals.var_vsat1r_i_dn11 * assign15800_e25103), (locals.var_vsat1r_i_dn13 * assign15800_e25103), (locals.var_vsat1r_i_dn14 * assign15800_e25103),)
    } else {
        (locals.var_vsat1r_t, locals.var_vsat1r_t_dn0, locals.var_vsat1r_t_dn2, locals.var_vsat1r_t_dn3, locals.var_vsat1r_t_dn4, locals.var_vsat1r_t_dn5, locals.var_vsat1r_t_dn6, locals.var_vsat1r_t_dn7, locals.var_vsat1r_t_dn8, locals.var_vsat1r_t_dn9, locals.var_vsat1r_t_dn10, locals.var_vsat1r_t_dn11, locals.var_vsat1r_t_dn13, locals.var_vsat1r_t_dn14,)
    }
};
        locals.var_vsat1r_t = assign15800_e25106;
        locals.var_vsat1r_t_dn0 = assign15800_e25106_d_n0;
        locals.var_vsat1r_t_dn2 = assign15800_e25106_d_n2;
        locals.var_vsat1r_t_dn3 = assign15800_e25106_d_n3;
        locals.var_vsat1r_t_dn4 = assign15800_e25106_d_n4;
        locals.var_vsat1r_t_dn5 = assign15800_e25106_d_n5;
        locals.var_vsat1r_t_dn6 = assign15800_e25106_d_n6;
        locals.var_vsat1r_t_dn7 = assign15800_e25106_d_n7;
        locals.var_vsat1r_t_dn8 = assign15800_e25106_d_n8;
        locals.var_vsat1r_t_dn9 = assign15800_e25106_d_n9;
        locals.var_vsat1r_t_dn10 = assign15800_e25106_d_n10;
        locals.var_vsat1r_t_dn11 = assign15800_e25106_d_n11;
        locals.var_vsat1r_t_dn13 = assign15800_e25106_d_n13;
        locals.var_vsat1r_t_dn14 = assign15800_e25106_d_n14;

        let assign15810_e25109: f64 = if locals.var_vsat1r_t < 1000.0 { 1.0 } else { 0.0 };
        locals.var_guard276 = assign15810_e25109;

        let (assign15820_e25120, assign15820_e25120_d_n0, assign15820_e25120_d_n2, assign15820_e25120_d_n3, assign15820_e25120_d_n4, assign15820_e25120_d_n5, assign15820_e25120_d_n6, assign15820_e25120_d_n7, assign15820_e25120_d_n8, assign15820_e25120_d_n9, assign15820_e25120_d_n10, assign15820_e25120_d_n11, assign15820_e25120_d_n13, assign15820_e25120_d_n14,) = {
    if ((((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard274 != 0.0)) && (locals.var_guard276 != 0.0)) {
        (1000.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (locals.var_vsat1r_t, locals.var_vsat1r_t_dn0, locals.var_vsat1r_t_dn2, locals.var_vsat1r_t_dn3, locals.var_vsat1r_t_dn4, locals.var_vsat1r_t_dn5, locals.var_vsat1r_t_dn6, locals.var_vsat1r_t_dn7, locals.var_vsat1r_t_dn8, locals.var_vsat1r_t_dn9, locals.var_vsat1r_t_dn10, locals.var_vsat1r_t_dn11, locals.var_vsat1r_t_dn13, locals.var_vsat1r_t_dn14,)
    }
};
        locals.var_vsat1r_t = assign15820_e25120;
        locals.var_vsat1r_t_dn0 = assign15820_e25120_d_n0;
        locals.var_vsat1r_t_dn2 = assign15820_e25120_d_n2;
        locals.var_vsat1r_t_dn3 = assign15820_e25120_d_n3;
        locals.var_vsat1r_t_dn4 = assign15820_e25120_d_n4;
        locals.var_vsat1r_t_dn5 = assign15820_e25120_d_n5;
        locals.var_vsat1r_t_dn6 = assign15820_e25120_d_n6;
        locals.var_vsat1r_t_dn7 = assign15820_e25120_d_n7;
        locals.var_vsat1r_t_dn8 = assign15820_e25120_d_n8;
        locals.var_vsat1r_t_dn9 = assign15820_e25120_d_n9;
        locals.var_vsat1r_t_dn10 = assign15820_e25120_d_n10;
        locals.var_vsat1r_t_dn11 = assign15820_e25120_d_n11;
        locals.var_vsat1r_t_dn13 = assign15820_e25120_d_n13;
        locals.var_vsat1r_t_dn14 = assign15820_e25120_d_n14;

        let assign15830_e25123: f64 = if p.p75 != 0.0 { 1.0 } else { 0.0 };
        locals.var_guard277 = assign15830_e25123;

        let (assign15840_e25193, assign15840_e25193_d_n0, assign15840_e25193_d_n2, assign15840_e25193_d_n3, assign15840_e25193_d_n4, assign15840_e25193_d_n5, assign15840_e25193_d_n6, assign15840_e25193_d_n7, assign15840_e25193_d_n8, assign15840_e25193_d_n9, assign15840_e25193_d_n10, assign15840_e25193_d_n11, assign15840_e25193_d_n13, assign15840_e25193_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard277 != 0.0)) {
        let assign15840_e25132: f64 = (-locals.var_vsatcv_i);
        let assign15840_e25135: f64 = (-locals.var_atcv_i);
        let assign15840_e25137: f64 = (assign15840_e25135 * locals.var_deltemp);
        let assign15840_e25140: f64 = (p.p574 * locals.var_deltemp);
        let assign15840_e25142: f64 = (assign15840_e25140 * locals.var_deltemp);
        let assign15840_e25143: f64 = (assign15840_e25137 + assign15840_e25142);
        let assign15840_e25145: f64 = (-locals.var_vsatcv_i);
        let assign15840_e25146: f64 = (assign15840_e25143 - assign15840_e25145);
        let assign15840_e25148: f64 = (assign15840_e25146 - 1e-6);
        let assign15840_e25150: f64 = (-locals.var_atcv_i);
        let assign15840_e25152: f64 = (assign15840_e25150 * locals.var_deltemp);
        let assign15840_e25155: f64 = (p.p574 * locals.var_deltemp);
        let assign15840_e25157: f64 = (assign15840_e25155 * locals.var_deltemp);
        let assign15840_e25158: f64 = (assign15840_e25152 + assign15840_e25157);
        let assign15840_e25160: f64 = (-locals.var_vsatcv_i);
        let assign15840_e25161: f64 = (assign15840_e25158 - assign15840_e25160);
        let assign15840_e25163: f64 = (assign15840_e25161 - 1e-6);
        let assign15840_e25165: f64 = (-locals.var_atcv_i);
        let assign15840_e25167: f64 = (assign15840_e25165 * locals.var_deltemp);
        let assign15840_e25170: f64 = (p.p574 * locals.var_deltemp);
        let assign15840_e25172: f64 = (assign15840_e25170 * locals.var_deltemp);
        let assign15840_e25173: f64 = (assign15840_e25167 + assign15840_e25172);
        let assign15840_e25175: f64 = (-locals.var_vsatcv_i);
        let assign15840_e25176: f64 = (assign15840_e25173 - assign15840_e25175);
        let assign15840_e25178: f64 = (assign15840_e25176 - 1e-6);
        let assign15840_e25179: f64 = (assign15840_e25163 * assign15840_e25178);
        let assign15840_e25182: f64 = (-locals.var_vsatcv_i);
        let assign15840_e25183: f64 = (4.0 * assign15840_e25182);
        let assign15840_e25185: f64 = (assign15840_e25183 * 1e-6);
        let assign15840_e25186: f64 = (assign15840_e25179 - assign15840_e25185);
        let assign15840_e25187: f64 = (assign15840_e25186).sqrt();
        let assign15840_e25188: f64 = (assign15840_e25148 + assign15840_e25187);
        let assign15840_e25189: f64 = (0.5 * assign15840_e25188);
        let assign15840_e25190: f64 = (assign15840_e25132 + assign15840_e25189);
        let assign15840_e25191: f64 = (locals.var_vsatcv_i + assign15840_e25190);
        (assign15840_e25191, (locals.var_vsatcv_i_dn0 + ((-locals.var_vsatcv_i_dn0) + (0.5 * ((-(-locals.var_vsatcv_i_dn0)) + (((((-(-locals.var_vsatcv_i_dn0)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn0)))) - ((4.0 * (-locals.var_vsatcv_i_dn0)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn2 + ((-locals.var_vsatcv_i_dn2) + (0.5 * ((-(-locals.var_vsatcv_i_dn2)) + (((((-(-locals.var_vsatcv_i_dn2)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn2)))) - ((4.0 * (-locals.var_vsatcv_i_dn2)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn3 + ((-locals.var_vsatcv_i_dn3) + (0.5 * ((-(-locals.var_vsatcv_i_dn3)) + (((((-(-locals.var_vsatcv_i_dn3)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn3)))) - ((4.0 * (-locals.var_vsatcv_i_dn3)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn4 + ((-locals.var_vsatcv_i_dn4) + (0.5 * ((((assign15840_e25135 * locals.var_deltemp_dn4) + (((p.p574 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15840_e25140 * locals.var_deltemp_dn4))) - (-locals.var_vsatcv_i_dn4)) + (((((((assign15840_e25150 * locals.var_deltemp_dn4) + (((p.p574 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15840_e25155 * locals.var_deltemp_dn4))) - (-locals.var_vsatcv_i_dn4)) * assign15840_e25178) + (assign15840_e25163 * (((assign15840_e25165 * locals.var_deltemp_dn4) + (((p.p574 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15840_e25170 * locals.var_deltemp_dn4))) - (-locals.var_vsatcv_i_dn4)))) - ((4.0 * (-locals.var_vsatcv_i_dn4)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn5 + ((-locals.var_vsatcv_i_dn5) + (0.5 * ((-(-locals.var_vsatcv_i_dn5)) + (((((-(-locals.var_vsatcv_i_dn5)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn5)))) - ((4.0 * (-locals.var_vsatcv_i_dn5)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn6 + ((-locals.var_vsatcv_i_dn6) + (0.5 * ((-(-locals.var_vsatcv_i_dn6)) + (((((-(-locals.var_vsatcv_i_dn6)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn6)))) - ((4.0 * (-locals.var_vsatcv_i_dn6)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn7 + ((-locals.var_vsatcv_i_dn7) + (0.5 * ((-(-locals.var_vsatcv_i_dn7)) + (((((-(-locals.var_vsatcv_i_dn7)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn7)))) - ((4.0 * (-locals.var_vsatcv_i_dn7)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn8 + ((-locals.var_vsatcv_i_dn8) + (0.5 * ((-(-locals.var_vsatcv_i_dn8)) + (((((-(-locals.var_vsatcv_i_dn8)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn8)))) - ((4.0 * (-locals.var_vsatcv_i_dn8)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn9 + ((-locals.var_vsatcv_i_dn9) + (0.5 * ((-(-locals.var_vsatcv_i_dn9)) + (((((-(-locals.var_vsatcv_i_dn9)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn9)))) - ((4.0 * (-locals.var_vsatcv_i_dn9)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn10 + ((-locals.var_vsatcv_i_dn10) + (0.5 * ((-(-locals.var_vsatcv_i_dn10)) + (((((-(-locals.var_vsatcv_i_dn10)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn10)))) - ((4.0 * (-locals.var_vsatcv_i_dn10)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn11 + ((-locals.var_vsatcv_i_dn11) + (0.5 * ((-(-locals.var_vsatcv_i_dn11)) + (((((-(-locals.var_vsatcv_i_dn11)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn11)))) - ((4.0 * (-locals.var_vsatcv_i_dn11)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn13 + ((-locals.var_vsatcv_i_dn13) + (0.5 * ((-(-locals.var_vsatcv_i_dn13)) + (((((-(-locals.var_vsatcv_i_dn13)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn13)))) - ((4.0 * (-locals.var_vsatcv_i_dn13)) * 1e-6)) / (2.0 * assign15840_e25187)))))), (locals.var_vsatcv_i_dn14 + ((-locals.var_vsatcv_i_dn14) + (0.5 * ((-(-locals.var_vsatcv_i_dn14)) + (((((-(-locals.var_vsatcv_i_dn14)) * assign15840_e25178) + (assign15840_e25163 * (-(-locals.var_vsatcv_i_dn14)))) - ((4.0 * (-locals.var_vsatcv_i_dn14)) * 1e-6)) / (2.0 * assign15840_e25187)))))),)
    } else {
        (locals.var_vsatcv_t, locals.var_vsatcv_t_dn0, locals.var_vsatcv_t_dn2, locals.var_vsatcv_t_dn3, locals.var_vsatcv_t_dn4, locals.var_vsatcv_t_dn5, locals.var_vsatcv_t_dn6, locals.var_vsatcv_t_dn7, locals.var_vsatcv_t_dn8, locals.var_vsatcv_t_dn9, locals.var_vsatcv_t_dn10, locals.var_vsatcv_t_dn11, locals.var_vsatcv_t_dn13, locals.var_vsatcv_t_dn14,)
    }
};
        locals.var_vsatcv_t = assign15840_e25193;
        locals.var_vsatcv_t_dn0 = assign15840_e25193_d_n0;
        locals.var_vsatcv_t_dn2 = assign15840_e25193_d_n2;
        locals.var_vsatcv_t_dn3 = assign15840_e25193_d_n3;
        locals.var_vsatcv_t_dn4 = assign15840_e25193_d_n4;
        locals.var_vsatcv_t_dn5 = assign15840_e25193_d_n5;
        locals.var_vsatcv_t_dn6 = assign15840_e25193_d_n6;
        locals.var_vsatcv_t_dn7 = assign15840_e25193_d_n7;
        locals.var_vsatcv_t_dn8 = assign15840_e25193_d_n8;
        locals.var_vsatcv_t_dn9 = assign15840_e25193_d_n9;
        locals.var_vsatcv_t_dn10 = assign15840_e25193_d_n10;
        locals.var_vsatcv_t_dn11 = assign15840_e25193_d_n11;
        locals.var_vsatcv_t_dn13 = assign15840_e25193_d_n13;
        locals.var_vsatcv_t_dn14 = assign15840_e25193_d_n14;

        let (assign15850_e25318, assign15850_e25318_d_n0, assign15850_e25318_d_n2, assign15850_e25318_d_n3, assign15850_e25318_d_n4, assign15850_e25318_d_n5, assign15850_e25318_d_n6, assign15850_e25318_d_n7, assign15850_e25318_d_n8, assign15850_e25318_d_n9, assign15850_e25318_d_n10, assign15850_e25318_d_n11, assign15850_e25318_d_n13, assign15850_e25318_d_n14,) = {
    if (((locals.var_guard244 == 0.0) && (locals.var_guard259 != 0.0)) && (locals.var_guard277 == 0.0)) {
        let assign15850_e25204: f64 = (-locals.var_atcv_i);
        let assign15850_e25206: f64 = (assign15850_e25204 * locals.var_deltemp);
        let assign15850_e25207: f64 = (1.0 + assign15850_e25206);
        let assign15850_e25210: f64 = (p.p574 * locals.var_deltemp);
        let assign15850_e25212: f64 = (assign15850_e25210 * locals.var_deltemp);
        let assign15850_e25213: f64 = (assign15850_e25207 + assign15850_e25212);
        let assign15850_e25215: f64 = (assign15850_e25213 - 1e-6);
        let assign15850_e25217: f64 = (-10000.0);
        let assign15850_e25219: f64 = (assign15850_e25217 * 0.001);
        let (assign15850_e25315, assign15850_e25315_d_n4,) = {
            if (!(assign15850_e25215 < assign15850_e25219)) {
                let assign15850_e25225: f64 = (-locals.var_atcv_i);
                let assign15850_e25227: f64 = (assign15850_e25225 * locals.var_deltemp);
                let assign15850_e25228: f64 = (1.0 + assign15850_e25227);
                let assign15850_e25231: f64 = (p.p574 * locals.var_deltemp);
                let assign15850_e25233: f64 = (assign15850_e25231 * locals.var_deltemp);
                let assign15850_e25234: f64 = (assign15850_e25228 + assign15850_e25233);
                let assign15850_e25236: f64 = (assign15850_e25234 - 1e-6);
                let assign15850_e25239: f64 = (-locals.var_atcv_i);
                let assign15850_e25241: f64 = (assign15850_e25239 * locals.var_deltemp);
                let assign15850_e25242: f64 = (1.0 + assign15850_e25241);
                let assign15850_e25245: f64 = (p.p574 * locals.var_deltemp);
                let assign15850_e25247: f64 = (assign15850_e25245 * locals.var_deltemp);
                let assign15850_e25248: f64 = (assign15850_e25242 + assign15850_e25247);
                let assign15850_e25250: f64 = (assign15850_e25248 - 1e-6);
                let assign15850_e25253: f64 = (-locals.var_atcv_i);
                let assign15850_e25255: f64 = (assign15850_e25253 * locals.var_deltemp);
                let assign15850_e25256: f64 = (1.0 + assign15850_e25255);
                let assign15850_e25259: f64 = (p.p574 * locals.var_deltemp);
                let assign15850_e25261: f64 = (assign15850_e25259 * locals.var_deltemp);
                let assign15850_e25262: f64 = (assign15850_e25256 + assign15850_e25261);
                let assign15850_e25264: f64 = (assign15850_e25262 - 1e-6);
                let assign15850_e25265: f64 = (assign15850_e25250 * assign15850_e25264);
                let assign15850_e25268: f64 = (4.0 * 0.001);
                let assign15850_e25270: f64 = (assign15850_e25268 * 0.001);
                let assign15850_e25271: f64 = (assign15850_e25265 + assign15850_e25270);
                let assign15850_e25272: f64 = (assign15850_e25271).sqrt();
                let assign15850_e25273: f64 = (assign15850_e25236 + assign15850_e25272);
                let assign15850_e25274: f64 = (0.5 * assign15850_e25273);
                (assign15850_e25274, (0.5 * (((assign15850_e25225 * locals.var_deltemp_dn4) + (((p.p574 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15850_e25231 * locals.var_deltemp_dn4))) + (((((assign15850_e25239 * locals.var_deltemp_dn4) + (((p.p574 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15850_e25245 * locals.var_deltemp_dn4))) * assign15850_e25264) + (assign15850_e25250 * ((assign15850_e25253 * locals.var_deltemp_dn4) + (((p.p574 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15850_e25259 * locals.var_deltemp_dn4))))) / (2.0 * assign15850_e25272)))),)
            } else {
                let assign15850_e25277: f64 = (-locals.var_atcv_i);
                let assign15850_e25279: f64 = (assign15850_e25277 * locals.var_deltemp);
                let assign15850_e25280: f64 = (1.0 + assign15850_e25279);
                let assign15850_e25283: f64 = (p.p574 * locals.var_deltemp);
                let assign15850_e25285: f64 = (assign15850_e25283 * locals.var_deltemp);
                let assign15850_e25286: f64 = (assign15850_e25280 + assign15850_e25285);
                let assign15850_e25288: f64 = (assign15850_e25286 - 1e-6);
                let assign15850_e25290: f64 = (-10000.0);
                let assign15850_e25292: f64 = (assign15850_e25290 * 0.001);
                let (assign15850_e25314, assign15850_e25314_d_n4,) = {
                    if (assign15850_e25288 < assign15850_e25292) {
                        let assign15850_e25295: f64 = (-0.001);
                        let assign15850_e25297: f64 = (assign15850_e25295 * 0.001);
                        let assign15850_e25300: f64 = (-locals.var_atcv_i);
                        let assign15850_e25302: f64 = (assign15850_e25300 * locals.var_deltemp);
                        let assign15850_e25303: f64 = (1.0 + assign15850_e25302);
                        let assign15850_e25306: f64 = (p.p574 * locals.var_deltemp);
                        let assign15850_e25308: f64 = (assign15850_e25306 * locals.var_deltemp);
                        let assign15850_e25309: f64 = (assign15850_e25303 + assign15850_e25308);
                        let assign15850_e25311: f64 = (assign15850_e25309 - 1e-6);
                        let assign15850_e25312: f64 = (assign15850_e25297 / assign15850_e25311);
                        (assign15850_e25312, (-((assign15850_e25297 * ((assign15850_e25300 * locals.var_deltemp_dn4) + (((p.p574 * locals.var_deltemp_dn4) * locals.var_deltemp) + (assign15850_e25306 * locals.var_deltemp_dn4)))) / (assign15850_e25311 * assign15850_e25311))),)
                    } else {
                        (0.0, 0.0,)
                    }
                };
                (assign15850_e25314, assign15850_e25314_d_n4,)
            }
        };
        let assign15850_e25316: f64 = (locals.var_vsatcv_i * assign15850_e25315);
        (assign15850_e25316, (locals.var_vsatcv_i_dn0 * assign15850_e25315), (locals.var_vsatcv_i_dn2 * assign15850_e25315), (locals.var_vsatcv_i_dn3 * assign15850_e25315), ((locals.var_vsatcv_i_dn4 * assign15850_e25315) + (locals.var_vsatcv_i * assign15850_e25315_d_n4)), (locals.var_vsatcv_i_dn5 * assign15850_e25315), (locals.var_vsatcv_i_dn6 * assign15850_e25315), (locals.var_vsatcv_i_dn7 * assign15850_e25315), (locals.var_vsatcv_i_dn8 * assign15850_e25315), (locals.var_vsatcv_i_dn9 * assign15850_e25315), (locals.var_vsatcv_i_dn10 * assign15850_e25315), (locals.var_vsatcv_i_dn11 * assign15850_e25315), (locals.var_vsatcv_i_dn13 * assign15850_e25315), (locals.var_vsatcv_i_dn14 * assign15850_e25315),)
    } else {
        (locals.var_vsatcv_t, locals.var_vsatcv_t_dn0, locals.var_vsatcv_t_dn2, locals.var_vsatcv_t_dn3, locals.var_vsatcv_t_dn4, locals.var_vsatcv_t_dn5, locals.var_vsatcv_t_dn6, locals.var_vsatcv_t_dn7, locals.var_vsatcv_t_dn8, locals.var_vsatcv_t_dn9, locals.var_vsatcv_t_dn10, locals.var_vsatcv_t_dn11, locals.var_vsatcv_t_dn13, locals.var_vsatcv_t_dn14,)
    }
};
        locals.var_vsatcv_t = assign15850_e25318;
        locals.var_vsatcv_t_dn0 = assign15850_e25318_d_n0;
        locals.var_vsatcv_t_dn2 = assign15850_e25318_d_n2;
        locals.var_vsatcv_t_dn3 = assign15850_e25318_d_n3;
        locals.var_vsatcv_t_dn4 = assign15850_e25318_d_n4;
        locals.var_vsatcv_t_dn5 = assign15850_e25318_d_n5;
        locals.var_vsatcv_t_dn6 = assign15850_e25318_d_n6;
        locals.var_vsatcv_t_dn7 = assign15850_e25318_d_n7;
        locals.var_vsatcv_t_dn8 = assign15850_e25318_d_n8;
        locals.var_vsatcv_t_dn9 = assign15850_e25318_d_n9;
        locals.var_vsatcv_t_dn10 = assign15850_e25318_d_n10;
        locals.var_vsatcv_t_dn11 = assign15850_e25318_d_n11;
        locals.var_vsatcv_t_dn13 = assign15850_e25318_d_n13;
        locals.var_vsatcv_t_dn14 = assign15850_e25318_d_n14;

    }
}
